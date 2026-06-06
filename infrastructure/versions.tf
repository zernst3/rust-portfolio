terraform {
  required_version = ">= 1.6"

  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 4.0"
    }
    random = {
      source  = "hashicorp/random"
      version = "~> 3.6"
    }
  }

  # Remote state is recommended so CI and your laptop share one state file.
  # Bootstrap a storage account once (see docs/DEPLOYMENT.md), then uncomment:
  #
  # backend "azurerm" {
  #   resource_group_name  = "rg-tfstate"
  #   storage_account_name = "sttfstate<unique>"
  #   container_name       = "tfstate"
  #   key                  = "rust-portfolio.tfstate"
  # }
}

provider "azurerm" {
  features {}
}

provider "random" {}

data "azurerm_client_config" "current" {}
