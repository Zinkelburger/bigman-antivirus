---
url: "https://github.com/int128/kubelogin"
title: "GitHub - int128/kubelogin: kubectl plugin for Kubernetes OpenID Connect authentication (kubectl oidc-login)"
site: "GitHub"
captured: "2026-11-02T19:03:36Z"
via: "live"
sha256: "65728917e7479665ba86e680e82cd6be6b03d5c472c188e6d6545f7e3be85186"
wayback: "https://web.archive.org/web/20260823235020/https://github.com/int128/kubelogin"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# GitHub - int128/kubelogin: kubectl plugin for Kubernetes OpenID Connect authentication (kubectl oidc-login)

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/int128/kubelogin>
> Snapshot: <https://web.archive.org/web/20260823235020/https://github.com/int128/kubelogin>

# kubelogin https://github.com/int128/kubelogin/actions/workflows/go.yaml https://goreportcard.com/report/github.com/int128/kubelogin

https://github.com/int128/kubelogin#kubelogin--

This is a kubectl plugin for [Kubernetes OpenID Connect (OIDC) authentication](https://kubernetes.io/docs/reference/access-authn-authz/authentication/#openid-connect-tokens), also known as kubectl oidc-login.

Here is an example of Kubernetes authentication with the Google Identity Platform:

https://user-images.githubusercontent.com/321266/85427290-86e43700-b5b6-11ea-9e97-ffefd736c9b7.gif

Kubelogin is designed to run as a [client-go credential plugin](https://kubernetes.io/docs/reference/access-authn-authz/authentication/#client-go-credential-plugins). When you run kubectl, kubelogin opens the browser and you can log in to the provider. Then kubelogin gets a token from the provider and kubectl access Kubernetes APIs with the token. Take a look at the diagram:

https://github.com/int128/kubelogin/blob/master/docs/credential-plugin-diagram.svg

## Getting Started

https://github.com/int128/kubelogin#getting-started

### Setup

https://github.com/int128/kubelogin#setup

Install the latest release from [Homebrew](https://brew.sh/), [Krew](https://github.com/kubernetes-sigs/krew), [Chocolatey](https://chocolatey.org/packages/kubelogin) or [GitHub Releases](https://github.com/int128/kubelogin/releases).

```
# Homebrew (macOS and Linux)
brew install kubelogin

# Krew (macOS, Linux, Windows and ARM)
kubectl krew install oidc-login

# Chocolatey (Windows)
choco install kubelogin
```

If you install via GitHub releases, save the binary as the name kubectl-oidc_login on your path. When you invoke kubectl oidc-login, kubectl finds it by the [naming convention of kubectl plugins](https://kubernetes.io/docs/tasks/extend-kubectl/kubectl-plugins/). The other install methods do this for you.

You need to set up the OIDC provider, cluster role binding, Kubernetes API server and kubeconfig. Your kubeconfig looks like this:

```
users:
  - name: oidc
    user:
      exec:
        apiVersion: client.authentication.k8s.io/v1
        command: kubectl
        args:
          - oidc-login
          - get-token
          - --oidc-issuer-url=ISSUER_URL
          - --oidc-client-id=YOUR_CLIENT_ID
```

See the [setup guide](https://github.com/int128/kubelogin/blob/master/docs/setup.md) for more.

### Run

https://github.com/int128/kubelogin#run

Run kubectl.

```
kubectl get pods
```

Kubectl executes kubelogin before calling the Kubernetes APIs. Kubelogin automatically opens the browser, and you can log in to the provider.

After the authentication, kubelogin returns the credentials to kubectl. Kubectl then calls the Kubernetes APIs with the credentials.

```
% kubectl get pods
Open http://localhost:8000 for authentication
NAME                          READY   STATUS    RESTARTS   AGE
echoserver-86c78fdccd-nzmd5   1/1     Running   0          26d
```

Kubelogin stores the ID token and refresh token to the cache. If the ID token is valid, it just returns it. If the ID token has expired, it will refresh the token using the refresh token. If the refresh token has expired, it will perform re-authentication.

## Troubleshooting

https://github.com/int128/kubelogin#troubleshooting

### Token cache

https://github.com/int128/kubelogin#token-cache

Kubelogin stores the token cache to the file system by default. For enhanced security, it is recommended to store it to the keyring. See the [token cache](https://github.com/int128/kubelogin/blob/master/docs/usage.md#token-cache) for details.

You can log out by deleting the token cache.

```
% kubectl oidc-login clean
Deleted the token cache at /home/user/.kube/cache/oidc-login
Deleted the token cache from the keyring
```

Kubelogin will ask you to log in via the browser again. If the browser has a cookie for the provider, you need to log out from the provider or clear the cookie.

### ID token claims

https://github.com/int128/kubelogin#id-token-claims

You can run setup command to dump the claims of an ID token from the provider.

```
% kubectl oidc-login setup --oidc-issuer-url=ISSUER_URL --oidc-client-id=REDACTED
...
You got a token with the following claims:

{
  "sub": "********",
  "iss": "https://accounts.google.com",
  "aud": "********",
  ...
}
```

You can set -v1 option to increase the log level.

```
users:
  - name: oidc
    user:
      exec:
        apiVersion: client.authentication.k8s.io/v1
        command: kubectl
        args:
          - oidc-login
          - get-token
          - -v1
```

You can run the [acceptance test](https://github.com/int128/kubelogin/blob/master/acceptance_test) to verify if kubelogin works with your provider.

## Docs

https://github.com/int128/kubelogin#docs

- [Setup guide](https://github.com/int128/kubelogin/blob/master/docs/setup.md)
- [Usage and options](https://github.com/int128/kubelogin/blob/master/docs/usage.md)
- [Standalone mode](https://github.com/int128/kubelogin/blob/master/docs/standalone-mode.md)
- [System test](https://github.com/int128/kubelogin/blob/master/system_test)
- [Acceptance_test for identity providers](https://github.com/int128/kubelogin/blob/master/acceptance_test)

## Contributions

https://github.com/int128/kubelogin#contributions

This is an open source software licensed under Apache License 2.0. Feel free to open issues and pull requests for improving code and documents.
