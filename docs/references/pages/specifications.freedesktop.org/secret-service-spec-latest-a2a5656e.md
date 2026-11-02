---
url: "https://specifications.freedesktop.org/secret-service-spec/latest/"
title: "Secret Service API Draft"
captured: "2026-11-02T18:45:21Z"
via: "live"
source_url: "https://specifications.freedesktop.org/secret-service/latest/"
sha256: "2097a51b29b55f066bfa379f1182e84cf4da26f7f1fc66e3ef75cb313ca9eaa8"
wayback: "https://web.archive.org/web/20250624041447/https://specifications.freedesktop.org/secret-service-spec/latest/"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# Secret Service API Draft

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://specifications.freedesktop.org/secret-service-spec/latest/>
> Snapshot: <https://web.archive.org/web/20250624041447/https://specifications.freedesktop.org/secret-service-spec/latest/>

[Jump to content](https://specifications.freedesktop.org/secret-service/latest/#_content)[Jump to page navigation: previous page [access key p]/next page [access key n]](https://specifications.freedesktop.org/secret-service/latest/#_bottom-navigation)

https://specifications.freedesktop.org/

[Secret Service API Draft](https://specifications.freedesktop.org/secret-service/latest/index.html)

Navigation←[→](https://specifications.freedesktop.org/secret-service/latest/description.html)

[Secret Service API Draft](https://specifications.freedesktop.org/secret-service/latest/index.html)

[Top](https://specifications.freedesktop.org/secret-service/latest/#)

←[→](https://specifications.freedesktop.org/secret-service/latest/description.html)

# Secret Service API Draft

Authors:

Stef Walter <stefw@collabora.co.uk>(GNOME Keyring Developer)

Michael Leupold <lemma@confuego.org>(KWallet Developer)

Publication Date: 2026-10-19, Version:  Secret Service 0.2 DRAFT

Copyright © 2008-2011 The Secret Service API Authors

[I API Documentation](https://specifications.freedesktop.org/secret-service/latest/description.html)

[1 Introduction](https://specifications.freedesktop.org/secret-service/latest/ch01.html)

[2 Secrets](https://specifications.freedesktop.org/secret-service/latest/ch02.html)

[3 Collection and Items](https://specifications.freedesktop.org/secret-service/latest/ch03.html)

[4 Aliases](https://specifications.freedesktop.org/secret-service/latest/aliases.html)

[5 Lookup Attributes](https://specifications.freedesktop.org/secret-service/latest/lookup-attributes.html)

[6 Sessions](https://specifications.freedesktop.org/secret-service/latest/sessions.html)

[7 Transfer of Secrets](https://specifications.freedesktop.org/secret-service/latest/transfer-secrets.html)

[7.1 Negotiation of Algorithms](https://specifications.freedesktop.org/secret-service/latest/transfer-secrets.html#id-1.2.8.6)

[7.2 Algorithm: plain](https://specifications.freedesktop.org/secret-service/latest/ch07s02.html)

[7.3 Algorithm: dh-ietf1024-sha256-aes128-cbc-pkcs7](https://specifications.freedesktop.org/secret-service/latest/ch07s03.html)

[8 Locking and Unlocking](https://specifications.freedesktop.org/secret-service/latest/unlocking.html)

[9 Prompts and Prompting](https://specifications.freedesktop.org/secret-service/latest/prompts.html)

[10 What's not included in the API](https://specifications.freedesktop.org/secret-service/latest/ch10.html)

[11 Notes for Service Implementors](https://specifications.freedesktop.org/secret-service/latest/ch11.html)

[II D-Bus API Reference](https://specifications.freedesktop.org/secret-service/latest/ref-dbus-api.html)

[12 Object Paths](https://specifications.freedesktop.org/secret-service/latest/object-paths.html)

[13 Interfaces](https://specifications.freedesktop.org/secret-service/latest/interfaces.html)

[org.freedesktop.Secret.Service](https://specifications.freedesktop.org/secret-service/latest/org.freedesktop.Secret.Service.html) — The Secret Service manages all the sessions and collections.

[org.freedesktop.Secret.Collection](https://specifications.freedesktop.org/secret-service/latest/org.freedesktop.Secret.Collection.html) — A collection of items containing secrets.

[org.freedesktop.Secret.Item](https://specifications.freedesktop.org/secret-service/latest/org.freedesktop.Secret.Item.html) — An item contains a secret, lookup attributes and has a label.

[org.freedesktop.Secret.Session](https://specifications.freedesktop.org/secret-service/latest/org.freedesktop.Secret.Session.html) — A session tracks state between the service and a client application.

[org.freedesktop.Secret.Prompt](https://specifications.freedesktop.org/secret-service/latest/org.freedesktop.Secret.Prompt.html) — A prompt necessary to complete an operation.

[14 Types](https://specifications.freedesktop.org/secret-service/latest/types.html)

[14.1 Struct types](https://specifications.freedesktop.org/secret-service/latest/types.html#id-1.3.4.2)

[14.2 Map types](https://specifications.freedesktop.org/secret-service/latest/ch14s02.html)

[15 Errors](https://specifications.freedesktop.org/secret-service/latest/errors.html)

[References](https://specifications.freedesktop.org/secret-service/latest/bi01.html)

List of Examples

[13.1 Example for properties](https://specifications.freedesktop.org/secret-service/latest/org.freedesktop.Secret.Service.html#id-1.3.3.2.4.3.4.1.2.1.1.1)

[13.2 Example for properties](https://specifications.freedesktop.org/secret-service/latest/org.freedesktop.Secret.Collection.html#id-1.3.3.3.4.4.4.1.2.1.1.1)

[→Part I API Documentation](https://specifications.freedesktop.org/secret-service/latest/description.html)

© 2026 Freedesktop.org
