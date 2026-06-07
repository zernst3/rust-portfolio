output "resource_group_name" {
  description = "Resource group — set as the RESOURCE_GROUP GitHub variable."
  value       = azurerm_resource_group.main.name
}

output "container_app_name" {
  description = "Container App name — set as the CONTAINER_APP_NAME GitHub variable."
  value       = azurerm_container_app.main.name
}

output "container_app_url" {
  description = "Default HTTPS URL of the app."
  value       = "https://${azurerm_container_app.main.ingress[0].fqdn}"
}

output "container_app_fqdn" {
  description = "Default ingress FQDN (CNAME target for a custom domain)."
  value       = azurerm_container_app.main.ingress[0].fqdn
}

output "custom_domain_verification_id" {
  description = "Value for the asuid.<domain> TXT record when binding a custom domain. Read with: terraform output -raw custom_domain_verification_id"
  value       = azurerm_container_app.main.custom_domain_verification_id
  sensitive   = true
}

output "key_vault_name" {
  description = "Key Vault holding the Mailgun secret."
  value       = azurerm_key_vault.main.name
}
