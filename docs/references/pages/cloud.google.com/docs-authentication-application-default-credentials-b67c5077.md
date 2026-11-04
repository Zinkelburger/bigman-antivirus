---
url: "https://cloud.google.com/docs/authentication/application-default-credentials"
title: "How Application Default Credentials works | Authentication | Google Cloud Documentation"
site: "Google Cloud Documentation"
captured: "2026-11-02T18:25:38Z"
via: "live"
source_url: "https://docs.cloud.google.com/docs/authentication/application-default-credentials"
sha256: "06f3ae56d99e087c0aa26905f4f9c3831786242215f285801ef0966d21a09252"
wayback: "https://web.archive.org/web/20251203082245/https://cloud.google.com/docs/authentication/application-default-credentials"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# How Application Default Credentials works | Authentication | Google Cloud Documentation

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://cloud.google.com/docs/authentication/application-default-credentials>
> Snapshot: <https://web.archive.org/web/20251203082245/https://cloud.google.com/docs/authentication/application-default-credentials>

-  [Home](https://docs.cloud.google.com/)
-   [Documentation](https://docs.cloud.google.com/docs)
-   [Developer tools](https://docs.cloud.google.com/docs/costs-usage)
-   [Google Cloud SDK](https://docs.cloud.google.com/sdk/docs)
-   [Authentication](https://docs.cloud.google.com/docs/authentication)
-   [Guides](https://docs.cloud.google.com/sdk/docs/overview)

#  How Application Default Credentials works

This page describes the locations where Application Default Credentials (ADC) looks for credentials. Understanding how ADC works can help you understand which credentials ADC is using, and how it's finding them.

Application Default Credentials (ADC) is a strategy used by the authentication libraries to automatically find credentials based on the application environment. The authentication libraries make those credentials available to [Cloud Client Libraries and Google API Client Libraries](https://docs.cloud.google.com/apis/docs/client-libraries-explained). When you use ADC, your code can run in either a development or production environment without changing how your application authenticates to Google Cloud services and APIs.

For information about how to provide credentials to ADC, including how to generate a local ADC file, see [Set up Application Default Credentials](https://docs.cloud.google.com/docs/authentication/provide-credentials-adc).

## Search order

ADC searches for credentials in the following locations:

- [GOOGLE_APPLICATION_CREDENTIALS environment variable](https://docs.cloud.google.com/docs/authentication/application-default-credentials#GAC)
- [A credential file created by using the gcloud auth application-default login command](https://docs.cloud.google.com/docs/authentication/application-default-credentials#personal)
- [The attached service account, returned by the metadata server](https://docs.cloud.google.com/docs/authentication/application-default-credentials#attached-sa)

The order of the locations ADC checks for credentials is not related to the relative merit of each location. For help with understanding the best ways to provide credentials to ADC, see [Set up Application Default Credentials](https://docs.cloud.google.com/docs/authentication/provide-credentials-adc).

### GOOGLE_APPLICATION_CREDENTIALS environment variable

You can use the GOOGLE_APPLICATION_CREDENTIALS environment variable to provide the location of a credential JSON file. This JSON file can be one of the following types of files:

-

A credential configuration file for Workforce Identity Federation

Workforce Identity Federation lets you use an external identity provider (IdP) to authenticate and authorize users to access Google Cloud resources. For more information, see [Workforce Identity Federation](https://docs.cloud.google.com/iam/docs/workforce-identity-federation) in the Identity and Access Management (IAM) documentation.

-

A credential configuration file for Workload Identity Federation

Workload Identity Federation lets you use an external IdP to authenticate and authorize workloads to access Google Cloud resources. For more information, see [Authenticating by using client libraries, the gcloud CLI, or Terraform](https://docs.cloud.google.com/iam/docs/using-workload-identity-federation#generate-automatic) in the Identity and Access Management (IAM) documentation.

-

A service account key

Service account keys create a security risk and are not recommended. Unlike the other credential file types, compromised service account keys can be used by a bad actor without any additional information. For more information, see [Best practices for using and managing service account keys](https://docs.cloud.google.com/iam/docs/best-practices-for-managing-service-account-keys).

### A credential file created by using the gcloud auth application-default login command

You can [provide credentials to ADC](https://docs.cloud.google.com/docs/authentication/set-up-adc-local-dev-environment) by running the [gcloud auth application-default login](https://docs.cloud.google.com/sdk/gcloud/reference/auth/application-default/login) command. This command creates a JSON file containing the credentials you provide (either from your user account or from impersonating a service account) and places it in a well-known location on your file system. The location depends on your operating system:

- Linux, macOS: $HOME/.config/gcloud/application_default_credentials.json
- Windows: %APPDATA%\gcloud\application_default_credentials.json

The credentials you provide to ADC by using the gcloud CLI are distinct from your gcloud credentials—the credentials the gcloud CLI uses to authenticate to Google Cloud. For more information about these two sets of credentials, see [gcloud CLI authentication configuration and ADC configuration](https://docs.cloud.google.com/docs/authentication/provide-credentials-adc#gcloud-credentials).

By default, the access tokens generated from a local ADC file created with user credentials include the [cloud-wide scope https://www.googleapis.com/auth/cloud-platform](https://docs.cloud.google.com/docs/authentication#authorization-gcp). To specify scopes explicitly, you use the [--scopes flag](https://docs.cloud.google.com/sdk/gcloud/reference/auth/application-default/login#--scopes) with the gcloud auth application-default login command.

To add scopes for services outside of Google Cloud, such as Google Drive, you can do one of the following:

-  OAuth authentication: [Create an OAuth client ID](https://support.google.com/cloud/answer/6158849). Provide the client ID to the gcloud auth application-default login command by using the [--client-id-file flag](https://docs.cloud.google.com/sdk/gcloud/reference/auth/application-default/login#--client-id-file), and specify your scopes with the [--scopes flag](https://docs.cloud.google.com/sdk/gcloud/reference/auth/application-default/login#--scopes).
-  Service account authentication: Create a service account. [Impersonate the service account](https://docs.cloud.google.com/docs/authentication/use-service-account-impersonation) by providing its email address to the gcloud auth application-default login command with the --impersonate-service-account flag, and specify your scopes with the [--scopes flag](https://docs.cloud.google.com/sdk/gcloud/reference/auth/application-default/login#--scopes).

### The attached service account

Many Google Cloud services let you attach a service account that can be used to provide credentials for accessing Google Cloud APIs. If ADC does not find credentials it can use in either the GOOGLE_APPLICATION_CREDENTIALS environment variable or the well-known location for local ADC credentials, it uses the [metadata server](https://docs.cloud.google.com/compute/docs/metadata/overview) to get credentials for the service where the code is running.

Using the credentials from the attached service account is the preferred method for finding credentials in a production environment on Google Cloud. To use the attached service account, follow these steps:

- Create a user-managed service account.
- Grant that service account the [least privileged](https://docs.cloud.google.com/iam/docs/using-iam-securely#least_privilege) IAM roles possible.
- Attach the service account to the resource where your code is running.

For help with creating a service account, see [Creating and managing service accounts](https://docs.cloud.google.com/iam/docs/service-accounts-create). For help with attaching a service account, see [Attaching a service account to a resource](https://docs.cloud.google.com/iam/docs/attach-service-accounts#attaching-to-resources). For help with determining the required IAM roles for your service account, see [Choose predefined roles](https://docs.cloud.google.com/iam/docs/choose-predefined-roles).

## What's next

- Learn the best ways to [provide credentials to ADC](https://docs.cloud.google.com/docs/authentication/provide-credentials-adc).
- [Authenticate using the Cloud Client Libraries](https://docs.cloud.google.com/docs/authentication/client-libraries).
- Explore [authentication methods](https://docs.cloud.google.com/docs/authentication).
- Learn about [client libraries](https://docs.cloud.google.com/apis/docs/client-libraries-explained).
