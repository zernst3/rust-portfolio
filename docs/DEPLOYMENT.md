# Deployment

This repo deploys to **Azure Container Apps** via **GitHub Actions** (passwordless
OIDC). Everything is self-contained in its own resource group and is independent
of any other Azure infrastructure.

```
GitHub push to main
      │  (OIDC login, no stored password)
      ▼
az acr build  ──►  Azure Container Registry  ──►  az containerapp update
                                                        │
                                                        ▼
                                            Azure Container App (ingress :8080)
                                              ├─ image pulled via managed identity
                                              └─ MAILGUN_API_KEY from Key Vault
```

What gets created (Terraform, `infrastructure/`): resource group, Log Analytics
workspace, Container Registry (Basic), a user-assigned managed identity, Key
Vault (RBAC) + the Mailgun secret, Container Apps environment, and the Container
App (external ingress on 8080, scale 0→1).

The runtime image is multi-stage: a Rust builder runs `dx bundle`, and a
`debian:bookworm-slim` runtime carries the `server` binary, `public/`
(hydration bootstrap + wasm, kept a sibling of the binary), and `assets/`
(served at `/static`). The server binds `0.0.0.0:$PORT` (set to 8080).

---

## Prerequisites (local, one time)

- Azure CLI (`az`), logged in: `az login`
- Terraform ≥ 1.6
- A Mailgun account + sending domain + API key

---

## Step 1 — One-time: passwordless GitHub → Azure (OIDC)

Create an Entra ID app GitHub Actions logs in as, with a federated credential
scoped to this repo's `main` branch. Replace the repo slug if different.

```bash
SUBSCRIPTION_ID=$(az account show --query id -o tsv)
TENANT_ID=$(az account show --query tenantId -o tsv)
REPO="zernst3/rust-portfolio"

# 1. App registration + service principal
APP_ID=$(az ad app create --display-name "rust-portfolio-deploy" --query appId -o tsv)
az ad sp create --id "$APP_ID"

# 2. Federated credential for pushes to main
az ad app federated-credential create --id "$APP_ID" --parameters '{
  "name": "github-main",
  "issuer": "https://token.actions.githubusercontent.com",
  "subject": "repo:'"$REPO"':ref:refs/heads/main",
  "audiences": ["api://AzureADTokenExchange"]
}'

# (Optional) also allow manual runs via the Actions UI:
az ad app federated-credential create --id "$APP_ID" --parameters '{
  "name": "github-dispatch",
  "issuer": "https://token.actions.githubusercontent.com",
  "subject": "repo:'"$REPO"':ref:refs/heads/main",
  "audiences": ["api://AzureADTokenExchange"]
}'

echo "AZURE_CLIENT_ID=$APP_ID"
echo "AZURE_TENANT_ID=$TENANT_ID"
echo "AZURE_SUBSCRIPTION_ID=$SUBSCRIPTION_ID"
```

You'll grant this principal a role on the resource group in Step 2 (after the RG
exists). `Contributor` on the RG is the simple choice and covers `az acr build`
+ `az containerapp update`.

**Add to GitHub** (repo → Settings → Secrets and variables → Actions):

| Type     | Name                    | Value                                  |
|----------|-------------------------|----------------------------------------|
| Secret   | `AZURE_CLIENT_ID`       | the `APP_ID` above                     |
| Secret   | `AZURE_TENANT_ID`       | tenant id                              |
| Secret   | `AZURE_SUBSCRIPTION_ID` | subscription id                        |
| Variable | `ACR_NAME`              | from `terraform output acr_name`       |
| Variable | `RESOURCE_GROUP`        | from `terraform output resource_group_name` |
| Variable | `CONTAINER_APP_NAME`    | from `terraform output container_app_name`  |

(The three Variables come from Step 2's outputs — add them after running Terraform.)

---

## Step 2 — One-time: provision infrastructure (Terraform)

First, register the resource providers this stack uses (one time per
subscription — the provider is configured NOT to bulk-register, which otherwise
hangs on fresh subscriptions). These are idempotent:

```bash
for ns in Microsoft.App Microsoft.ContainerRegistry Microsoft.KeyVault \
          Microsoft.OperationalInsights Microsoft.ManagedIdentity; do
  az provider register --namespace "$ns"
done
# Confirm they're "Registered" (takes a minute or two) before applying:
for ns in Microsoft.App Microsoft.ContainerRegistry Microsoft.KeyVault \
          Microsoft.OperationalInsights Microsoft.ManagedIdentity; do
  echo "$ns: $(az provider show -n $ns --query registrationState -o tsv)"
done
```

Then provision:

```bash
cd infrastructure
cp terraform.tfvars.example terraform.tfvars   # set mailgun_domain, project, etc.

export TF_VAR_mailgun_api_key="key-xxxxxxxxxxxxxxxxxxxx"   # never commit this
terraform init
terraform apply
```

Notes:
- The first apply creates the app with a **placeholder image** (a public
  hello-world). That's expected — Step 3 replaces it with the real build.
- Key Vault uses RBAC. If the Mailgun secret create fails with a 403 on a brand
  new vault, it's just role propagation lag — **re-run `terraform apply`**.
- Then grant the deploy principal access to the new RG:
  ```bash
  RG=$(terraform output -raw resource_group_name)
  az role assignment create --assignee "<AZURE_CLIENT_ID>" \
    --role Contributor \
    --scope "/subscriptions/$(az account show --query id -o tsv)/resourceGroups/$RG"
  ```
- Copy outputs into the GitHub **Variables** from Step 1:
  ```bash
  terraform output
  ```

---

## Step 3 — First deploy

Push to `main` (or run the **Deploy** workflow manually). It builds the image in
ACR and points the Container App at it. Then:

```bash
terraform output -raw container_app_url   # open this
```

Subsequent pushes to `main` redeploy automatically.

> Scale-to-zero (min 0): after a period of no traffic the app spins down, so the
> first request cold-starts (a few seconds). Set `min_replicas = 1` in
> `infrastructure/main.tf` if you'd rather pay to keep it warm.

---

## Step 4 — Custom domain + free managed TLS (optional)

Container Apps issues a free managed certificate once the domain is validated.

1. Get the values you'll need:
   ```bash
   cd infrastructure
   terraform output -raw container_app_fqdn                 # CNAME target
   terraform output -raw custom_domain_verification_id       # TXT value
   ```
2. Create DNS records at your registrar (use a subdomain like `www`; apex
   domains need an ALIAS/ANAME record instead of CNAME):
   - `CNAME`  `www.zachary-ernst.dev`  →  `<container_app_fqdn>`
   - `TXT`    `asuid.www.zachary-ernst.dev`  →  `<custom_domain_verification_id>`
3. Add + bind the hostname (Azure then issues the managed cert automatically):
   ```bash
   RG=$(terraform output -raw resource_group_name)
   APP=$(terraform output -raw container_app_name)
   ENV="cae-rust-portfolio-prod"   # or: az containerapp show -n $APP -g $RG --query properties.managedEnvironmentId

   az containerapp hostname add  --name "$APP" --resource-group "$RG" --hostname www.zachary-ernst.dev
   az containerapp hostname bind --name "$APP" --resource-group "$RG" \
     --hostname www.zachary-ernst.dev --environment "$ENV" --validation-method CNAME
   ```
   The managed certificate provisions in a few minutes; HTTPS then serves on the
   custom domain at no cost.

---

## Local development

```bash
make serve     # dx serve (release) at http://127.0.0.1:8080
make check     # fmt + clippy + check + test (same gates as CI)
make bundle    # production bundle locally (what the Docker builder runs)
```

The headless browser smoke test (`scripts/headless-verify.mjs`) needs a Node
with global `fetch`/`WebSocket`; the repo was verified with `node@24`.
