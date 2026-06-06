variable "project" {
  description = "Short project slug used in resource names."
  type        = string
  default     = "rust-portfolio"
}

variable "environment" {
  description = "Deployment environment slug (prod, staging, ...)."
  type        = string
  default     = "prod"
}

variable "resource_group_name" {
  description = "Resource group that holds ALL portfolio resources."
  type        = string
  default     = "MyPortfolioSite"
}

variable "location" {
  description = "Azure region."
  type        = string
  default     = "eastus"
}

variable "container_image" {
  description = <<-EOT
    Image the Container App runs. Defaults to a public placeholder so the very
    first `terraform apply` succeeds before any image exists in ACR. The real
    image is pushed + set by the GitHub Actions deploy (az containerapp update);
    Terraform ignores image drift thereafter (see lifecycle in main.tf).
  EOT
  type        = string
  default     = "mcr.microsoft.com/azuredocs/containerapps-helloworld:latest"
}

variable "mailgun_api_key" {
  description = "Mailgun API key. Provide via `export TF_VAR_mailgun_api_key=...`; never commit it. Stored in Key Vault."
  type        = string
  sensitive   = true
}

variable "mailgun_domain" {
  description = "Mailgun sending domain (e.g. mg.zachary-ernst.dev). Not secret; passed as a plain container env var."
  type        = string
}

variable "cpu" {
  description = "vCPU per replica (must pair with a valid memory value)."
  type        = number
  default     = 0.5
}

variable "memory" {
  description = "Memory per replica (valid pair for the cpu value)."
  type        = string
  default     = "1Gi"
}
