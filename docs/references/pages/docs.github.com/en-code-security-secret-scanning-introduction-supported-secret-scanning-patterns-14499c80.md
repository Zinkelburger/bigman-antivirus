---
url: "https://docs.github.com/en/code-security/secret-scanning/introduction/supported-secret-scanning-patterns"
title: "Supported secret scanning patterns - GitHub Docs"
site: "GitHub Docs"
captured: "2026-11-02T18:30:31Z"
via: "live"
source_url: "https://docs.github.com/en/code-security/reference/secret-security/supported-secret-scanning-patterns"
sha256: "3d8f808743be396993739aee4be87588798364854a1d81627b7bba4da451b877"
wayback: "https://web.archive.org/web/20260113042410/https://docs.github.com/en/code-security/secret-scanning/introduction/supported-secret-scanning-patterns"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# Supported secret scanning patterns - GitHub Docs

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://docs.github.com/en/code-security/secret-scanning/introduction/supported-secret-scanning-patterns>
> Snapshot: <https://web.archive.org/web/20260113042410/https://docs.github.com/en/code-security/secret-scanning/introduction/supported-secret-scanning-patterns>

# Supported secret scanning patterns

Lists of supported secrets and the partners that GitHub works with to prevent fraudulent use of secrets that were committed accidentally.

## Who can use this feature?

Secret scanning is available for the following repository types:

- Public repositories: Secret scanning runs automatically for free.
- Organization-owned private and internal repositories: Available with [GitHub Secret Protection](https://docs.github.com/en/get-started/learning-about-github/about-github-advanced-security) enabled on GitHub Team or GitHub Enterprise Cloud.
- User-owned repositories: Available on GitHub Enterprise Cloud with Enterprise Managed Users. Available on GitHub Enterprise Server when the enterprise has [GitHub Secret Protection](https://docs.github.com/en/get-started/learning-about-github/about-github-advanced-security) enabled.

## In this article

## Up next

[Estimating the price of Secret Protection](https://docs.github.com/en/code-security/how-tos/secure-at-scale/configure-organization-security/configure-specific-tools/estimate-price)

## [About secret scanning patterns](https://docs.github.com/en/code-security/reference/secret-security/supported-secret-scanning-patterns#about-secret-scanning-patterns)

There are three types of secret scanning alerts:

- User alerts: Reported to users in the  Security and quality tab of the repository, when a supported secret is detected in the repository.
- Push protection alerts: Reported to users in the  Security and quality tab of the repository, when a contributor bypasses push protection.
- Partner alerts: Reported directly to secret providers that are part of secret scanning's partner program. These alerts are not reported in the  Security and quality tab of the repository.

For in-depth information about each alert type, see [About secret scanning alerts](https://docs.github.com/en/code-security/concepts/secret-security/about-alerts).

If you use the REST API for secret scanning, you can use the Secret type to report on secrets from specific issuers. For more information, see [REST API endpoints for secret scanning](https://docs.github.com/en/enterprise-cloud@latest/rest/secret-scanning).

### [Pattern categories](https://docs.github.com/en/code-security/reference/secret-security/supported-secret-scanning-patterns#pattern-categories)

| Category | Description | Detection approach | Example

| Generic | Secrets not tied to a specific provider, such as private keys and database connection strings | Regex-based | rsa_private_key

| AI-detected | Passwords and other unstructured secrets detected using AI models | AI-based | password

| Provider | Secrets tied to a specific service provider (such as AWS, Azure, Stripe) | Regex-based | aws_access_key_id

### [Capabilities by category](https://docs.github.com/en/code-security/reference/secret-security/supported-secret-scanning-patterns#capabilities-by-category)

| Capability | Generic patterns | AI-detected | Provider patterns

| User alerts |  |  |

| Partner notifications |  |  |  (if partner)

| Push protection |  |  |  (most)

| Validity checks |  |  | Some

| Extended metadata |  |  | Some

| Base64 format support |  |  | Some

The table shows the capabilities that each category supports. Whether a capability is available to you also depends on your plan and your repository's settings.

## [Supported generic patterns](https://docs.github.com/en/code-security/reference/secret-security/supported-secret-scanning-patterns#supported-generic-patterns)

Precision levels are estimated based on the pattern type's typical false positive rates.

| Provider | Token | Description | Precision

| Generic | ec_private_key | Elliptic Curve (EC) private keys used for cryptographic operations | High

| Generic | generic_private_key | Cryptographic private keys with -----BEGIN PRIVATE KEY----- header | High

| Generic | http_basic_authentication_header | HTTP Basic Authentication credentials in request headers | Medium

| Generic | http_bearer_authentication_header | HTTP Bearer tokens used for API authentication | Medium

| Generic | mongodb_connection_string | Connection strings for MongoDB databases containing credentials | High

| Generic | mysql_connection_url | Connection strings for MySQL databases containing credentials | High

| Generic | openssh_private_key | OpenSSH format private keys used for SSH authentication | High

| Generic | pgp_private_key | PGP (Pretty Good Privacy) private keys used for encryption and signing | High

| Generic | postgres_connection_string | Connection strings for PostgreSQL databases containing credentials | High

| Generic | rsa_private_key | RSA private keys used for cryptographic operations | High

Note

Validity checks are not supported for generic patterns.

## [Supported AI-detected patterns](https://docs.github.com/en/code-security/reference/secret-security/supported-secret-scanning-patterns#supported-ai-detected-patterns)

Secret scanning uses Copilot to detect generic secrets using AI. See [Application card: GitHub security and quality AI features](https://docs.github.com/en/code-security/responsible-use/security-and-quality-ai-features).

| Provider | Token

| Generic | password

Note

Validity checks are not supported for passwords.

## [Supported provider patterns](https://docs.github.com/en/code-security/reference/secret-security/supported-secret-scanning-patterns#supported-provider-patterns)

Use the table below to search, filter, and browse all supported patterns. You can filter by provider name, push protection support, validity checks, and more.

Note

Service providers update the patterns used to generate tokens periodically and may support more than one version of a token. Push protection only supports the most recent token versions that secret scanning can identify with confidence. This avoids push protection blocking commits unnecessarily when a result may be a false positive, which is more likely to happen with legacy tokens.

Showing 531 of 531 patterns

## Supported patterns

|  | Secret | Partner | User alert | Push protection | Validity check | Metadata check | Base64

| 1Password |

1Password Service Account Token

1password_service_account_token

| ✗ | ✓ | ✓ | ✗ | ✗ | ✗

| Adafruit |

Adafruit IO Key

adafruit_io_key

| ✓ | ✓ | ✓ | ✓ | ✓ | ✗

| Adobe |

Adobe Client Secret

adobe_client_secret

| ✓ | ✓ | ✓ | ✗ | ✗ | ✗

| Adobe |

Adobe Device Token

adobe_device_token

| ✓ | ✓ | ✓ | ✗ | ✗ | ✗

| Adobe |

Adobe PAC Token

adobe_pac_token

| ✓ | ✓ | ✓ | ✗ | ✗ | ✗

| Adobe |

Adobe Refresh Token

adobe_refresh_token

| ✓ | ✓ | ✓ | ✗ | ✗ | ✗

| Adobe |

Adobe Service Token

adobe_service_token

| ✓ | ✓ | ✓ | ✗ | ✗ | ✗

| Adobe |

Adobe Short-Lived Access Token

adobe_short_lived_access_token

| ✓ | ✓ | ✓ | ✗ | ✗ | ✗

| Aikido |

Aikido API Client Secret

aikido_api_client_secret

| ✗ | ✓ | ✓ | ✗ | ✗ | ✗

| Aikido |

Aikido CI Scanning Token

aikido_ci_scanning_token

| ✗ | ✓ | ✓ | ✗ | ✗ | ✗

| Airtable |

Airtable API Key

airtable_api_key

| ✗ | ✓ | ✓ | ✗ | ✓ | ✗

| Airtable |

Airtable Personal Access Token

airtable_personal_access_token

| ✗ | ✓ | ✓ | ✓ | ✓ | ✗

| Aiven |

Aiven Auth Token

aiven_auth_token

| ✓ | ✓ | ✓ | ✗ | ✗ | ✗

| Aiven |

Aiven Service Password

aiven_service_password

| ✓ | ✓ | ✓ | ✗ | ✗ | ✗

| Alibaba |

Alibaba Cloud AccessKey ID

alibaba_cloud_access_key_id, alibaba_cloud_access_key_secret

| ✓ | ✓ | ✓ | ✓ | ✗ | ✗

| Amazon AWS |

Amazon AWS Access Key ID

aws_access_key_id, aws_secret_access_key, [Token versions](https://docs.github.com/en/code-security/reference/secret-security/supported-secret-scanning-patterns#token-versions)

| ✓ | ✓ | ✓ | ✓ | ✗ | ✓

| Amazon AWS |

Amazon AWS API Key ID

aws_api_key

| ✓ | ✓ | ✓ | ✗ | ✗ | ✗

| Amazon AWS |

Amazon AWS Session Token

aws_secret_access_key, aws_session_token, aws_temporary_access_key_id

| ✗ | ✓ | ✓ | ✓ | ✗ | ✗

| Anthropic |

Anthropic Admin API Key

anthropic_admin_api_key

| ✓ | ✓ | ✓ | ✓ | ✗ | ✗

| Anthropic |

Anthropic API Key

anthropic_api_key, [Token versions](https://docs.github.com/en/code-security/reference/secret-security/supported-secret-scanning-patterns#token-versions)

| ✓ | ✓ | ✓ | ✓ | ✓ | ✓

| Anthropic |

Anthropic Service Account API Key

anthropic_service_api_key

| ✓ | ✓ | ✓ | ✗ | ✗ | ✗

| Anthropic |

Anthropic Session ID

anthropic_session_id

| ✓ | ✓ | ✓ | ✗ | ✗ | ✗

| Anthropic |

Anthropic User API Key

anthropic_user_api_key

| ✓ | ✓ | ✓ | ✗ | ✗ | ✗

| APIclub |

APIclub API Key

apiclub_api_key

| ✗ | ✓ | ✓ | ✗ | ✗ | ✗

| Apify |

Apify Actor Run API Token

apify_actor_run_api_token

| ✓ | ✓ | ✓ | ✗ | ✗ | ✗
