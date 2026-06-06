# ─────────────────────────────────────────────────────────────────────────────
# rust-portfolio infrastructure (Azure Container Apps).
#
# Entirely self-contained: its own resource group, registry, vault, and
# environment. It does NOT touch the Agora infrastructure.
#
# Identity model: ONE user-assigned managed identity, created first and granted
# AcrPull + Key Vault Secrets User BEFORE the Container App is created. That
# avoids the chicken-and-egg where a system-assigned identity doesn't exist yet
# when its first revision needs to pull the image / read the secret.
# ─────────────────────────────────────────────────────────────────────────────

# Suffix for globally-unique names (ACR + Key Vault).
resource "random_string" "suffix" {
  length  = 6
  lower   = true
  upper   = false
  numeric = true
  special = false
}

locals {
  base        = "${var.project}-${var.environment}"
  base_nodash = replace(var.project, "-", "")
  tags = {
    project     = var.project
    environment = var.environment
    managed_by  = "terraform"
  }
}

resource "azurerm_resource_group" "main" {
  name     = var.resource_group_name
  location = var.location
  tags     = local.tags
}

resource "azurerm_log_analytics_workspace" "main" {
  name                = "log-${local.base}"
  location            = azurerm_resource_group.main.location
  resource_group_name = azurerm_resource_group.main.name
  sku                 = "PerGB2018"
  retention_in_days   = 30
  tags                = local.tags
}

resource "azurerm_container_registry" "main" {
  name                = "acr${local.base_nodash}${random_string.suffix.result}"
  resource_group_name = azurerm_resource_group.main.name
  location            = azurerm_resource_group.main.location
  sku                 = "Basic"
  admin_enabled       = false # pull via managed identity, not admin creds
  tags                = local.tags
}

# ── Identity + role grants (created before the Container App) ─────────────────
resource "azurerm_user_assigned_identity" "app" {
  name                = "id-${local.base}"
  resource_group_name = azurerm_resource_group.main.name
  location            = azurerm_resource_group.main.location
  tags                = local.tags
}

resource "azurerm_role_assignment" "acr_pull" {
  scope                = azurerm_container_registry.main.id
  role_definition_name = "AcrPull"
  principal_id         = azurerm_user_assigned_identity.app.principal_id
}

resource "azurerm_role_assignment" "kv_secrets_user" {
  scope                = azurerm_key_vault.main.id
  role_definition_name = "Key Vault Secrets User"
  principal_id         = azurerm_user_assigned_identity.app.principal_id
}

# Lets whoever runs `terraform apply` write the Mailgun secret into the
# RBAC-authorized vault. RBAC propagation can take a minute or two — if the
# secret create 403s on a fresh vault, just re-run apply.
resource "azurerm_role_assignment" "kv_deployer" {
  scope                = azurerm_key_vault.main.id
  role_definition_name = "Key Vault Secrets Officer"
  principal_id         = data.azurerm_client_config.current.object_id
}

# ── Key Vault + Mailgun secret ───────────────────────────────────────────────
resource "azurerm_key_vault" "main" {
  name                       = "kv${local.base_nodash}${random_string.suffix.result}"
  location                   = azurerm_resource_group.main.location
  resource_group_name        = azurerm_resource_group.main.name
  tenant_id                  = data.azurerm_client_config.current.tenant_id
  sku_name                   = "standard"
  rbac_authorization_enabled = true
  tags                       = local.tags
}

resource "azurerm_key_vault_secret" "mailgun" {
  name         = "mailgun-api-key"
  value        = var.mailgun_api_key
  key_vault_id = azurerm_key_vault.main.id
  depends_on   = [azurerm_role_assignment.kv_deployer]
}

# ── Container Apps environment + app ─────────────────────────────────────────
resource "azurerm_container_app_environment" "main" {
  name                       = "cae-${local.base}"
  location                   = azurerm_resource_group.main.location
  resource_group_name        = azurerm_resource_group.main.name
  log_analytics_workspace_id = azurerm_log_analytics_workspace.main.id
  tags                       = local.tags
}

resource "azurerm_container_app" "main" {
  name                         = "ca-${local.base}"
  container_app_environment_id = azurerm_container_app_environment.main.id
  resource_group_name          = azurerm_resource_group.main.name
  revision_mode                = "Single"
  tags                         = local.tags

  identity {
    type         = "UserAssigned"
    identity_ids = [azurerm_user_assigned_identity.app.id]
  }

  # Pull from ACR using the managed identity (no admin password).
  registry {
    server   = azurerm_container_registry.main.login_server
    identity = azurerm_user_assigned_identity.app.id
  }

  # Mailgun API key sourced from Key Vault via the managed identity (never raw).
  secret {
    name                = "mailgun-api-key"
    identity            = azurerm_user_assigned_identity.app.id
    key_vault_secret_id = azurerm_key_vault_secret.mailgun.versionless_id
  }

  ingress {
    external_enabled = true
    target_port      = 8080
    transport        = "auto"
    traffic_weight {
      latest_revision = true
      percentage      = 100
    }
  }

  template {
    min_replicas = 0 # scale to zero when idle (cold start on first request)
    max_replicas = 1

    container {
      name   = "portfolio"
      image  = var.container_image
      cpu    = var.cpu
      memory = var.memory

      env {
        name  = "PORT"
        value = "8080"
      }
      env {
        name  = "MAILGUN_DOMAIN"
        value = var.mailgun_domain
      }
      env {
        name        = "MAILGUN_API_KEY"
        secret_name = "mailgun-api-key"
      }
    }
  }

  lifecycle {
    # The image is rolled out-of-band by the GitHub Actions deploy
    # (az containerapp update --image). Don't let Terraform revert it back to
    # the placeholder default on subsequent applies.
    ignore_changes = [template[0].container[0].image]
  }

  depends_on = [
    azurerm_role_assignment.acr_pull,
    azurerm_role_assignment.kv_secrets_user,
  ]
}
