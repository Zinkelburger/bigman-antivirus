---
url: "https://learn.microsoft.com/en-us/cli/azure/msal-based-azure-cli"
title: "MSAL-based Azure CLI"
captured: "2026-11-02T18:34:35Z"
via: "live"
source_url: "https://learn.microsoft.com/en-us/cli/azure/msal-based-azure-cli?view=azure-cli-latest"
sha256: "48d3e406e65656d440d01bdd2b68d2d34f217baebaec38734889da484feb4162"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# MSAL-based Azure CLI

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://learn.microsoft.com/en-us/cli/azure/msal-based-azure-cli>

[Edit](https://github.com/MicrosoftDocs/azure-docs-cli/blob/main/docs-ref-conceptual/Latest-version/msal-based-azure-cli.md)

# MSAL-based Azure CLI

Starting with version 2.30.0, Azure CLI uses Microsoft Authentication Library [(MSAL)](https://github.com/AzureAD/microsoft-authentication-library-for-python) as its underlying authentication library. MSAL uses the Azure Active Directory v2.0 authentication flow to provide enhanced functionality and increase security for token cache.

Warning

Breaking Changes are introduced in Azure CLI 2.30.0. Carefully read this document before installation.

## accessTokens.json deprecation

Previous versions of Azure CLI saved Azure AD Authentication Library (ADAL) tokens and service principal entries to ~/.azure/accessToken.json. The latest versions of the Azure CLI use MSAL and no longer generate accessTokens.json. Existing workflows depending on accessTokens.json no longer work.

The MSAL token cache and service principal entries are saved as encrypted files on Windows, and plaintext files on Linux and macOS.

Important

When using Azure CLI in a pipeline, such as Azure DevOps, ensure all tasks and stages are using versions of Azure CLI v2.30.0 or higher for MSAL-based Azure CLI. Azure CLI 2.30.0 isn't backward compatible with prior versions and throws an error when working with versions earlier than 2.30.0.

## Alternatives to consider

Alternatives to consider for stability:

### Calling az account get-access-token

You can manually call [az account get-access-token](https://learn.microsoft.com/en-us/cli/azure/account#az_account_get_access_token) in a terminal or use a subprocess to call it from another programming language. By default, the returned access token is for Azure Resource Manager (ARM) and the default subscription/tenant shown in [az account show](https://learn.microsoft.com/en-us/cli/azure/account#az_account_show).

```
# get the active subscription
az account show --output table

# get access token for the active subscription
az account get-access-token

# get access token for a specific subscription
az account get-access-token --subscription "<subscription ID or name>"

```

To learn more about access tokens, see [Sign in with Azure CLI](https://learn.microsoft.com/en-us/cli/azure/authenticate-azure-cli-interactively?view=azure-cli-latest#refresh-tokens).

### Using AzureCliCredential

AzureCliCredential is a credential type in all existing language SDKs. It uses a subprocess to call az account get-access-token to retrieve an access token for the current logged-in account.

## See also

- MSAL

- [Overview of the Microsoft Authentication Library (MSAL)](https://learn.microsoft.com/en-us/azure/active-directory/develop/msal-overview)
- [Migrate applications to the Microsoft Authentication Library (MSAL)](https://learn.microsoft.com/en-us/azure/active-directory/develop/msal-migration)

- Python

- [AzureCliCredential Class](https://learn.microsoft.com/en-us/python/api/azure-identity/azure.identity.azureclicredential) in Python

- .NET

- [AzureCliCredential Class](https://learn.microsoft.com/en-us/dotnet/api/azure.identity.azureclicredential) in .NET

- Java

- [AzureCliCredential Class](https://learn.microsoft.com/en-us/java/api/com.azure.identity.azureclicredential) in Java

## Feedback

Was this page helpful?

No

Need help with this topic?

Want to try using Ask Learn to clarify or guide you through this topic?

-   Last updated on  2026-09-01
