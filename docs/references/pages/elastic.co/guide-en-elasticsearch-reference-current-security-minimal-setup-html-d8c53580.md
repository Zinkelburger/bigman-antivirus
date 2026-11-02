---
url: "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-minimal-setup.html"
captured: "2026-11-02T18:53:31Z"
via: "live"
source_url: "https://www.elastic.co/docs/deploy-manage/security/set-up-minimal-security"
sha256: "35dafd342838cce9f0c155ed49bf438b579ddbf79e5b6ab8e7975e898bd7c76b"
wayback: "https://web.archive.org/web/20250315210229/https://www.elastic.co/guide/en/elasticsearch/reference/current/security-minimal-setup.html"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# https://www.elastic.co/guide/en/elasticsearch/reference/current/security-minimal-setup.html

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.elastic.co/guide/en/elasticsearch/reference/current/security-minimal-setup.html>
> Snapshot: <https://web.archive.org/web/20250315210229/https://www.elastic.co/guide/en/elasticsearch/reference/current/security-minimal-setup.html>

﻿---
title: Minimal security setup
description: In Elasticsearch 8.0 and later, security is enabled automatically when you start Elasticsearch for the first time. If you’re running an existing Elasticsearch...
url: https://www.elastic.co/docs/deploy-manage/security/set-up-minimal-security
products:
  - Elastic Documentation
  - Elasticsearch
applies_to:
  - Self-managed Elastic deployments: Generally available
---

# Minimal security setup
<important>
  You only need to complete the following steps if you’re running an existing, unsecured cluster and want to enable the Elasticsearch security features.
</important>

In Elasticsearch 8.0 and later, security is [enabled automatically](https://www.elastic.co/docs/deploy-manage/security/self-auto-setup) when you start Elasticsearch for the first time.
If you’re running an existing Elasticsearch cluster where security is disabled, you can manually enable the Elasticsearch security features and then create passwords for [built-in users](https://www.elastic.co/docs/deploy-manage/users-roles/cluster-or-deployment-auth/built-in-users). You can add more users later, but using the built-in users simplifies the process of enabling security for your cluster.
<important>
  The minimal security scenario described in this document is not sufficient for [production mode](/docs/deploy-manage/deploy/self-managed/bootstrap-checks#dev-vs-prod-mode) clusters. If your cluster has multiple nodes, you must follow this guide, together with [configure Transport Layer Security (TLS)](https://www.elastic.co/docs/deploy-manage/security/set-up-basic-security) between nodes.
</important>


## Enable Elasticsearch security features

Enabling the Elasticsearch security features provides basic authentication so that you can run a local cluster with username and password authentication.
1. On **every** node in your cluster, stop both Kibana and Elasticsearch if they are running.
2. On **every** node in your cluster, add the `xpack.security.enabled` setting to the `$ES_PATH_CONF/elasticsearch.yml` file and set the value to `true`:
   ```yaml
   xpack.security.enabled: true
   ```
   <note>
   The `$ES_PATH_CONF` variable is the path for the Elasticsearch configuration files. If you installed Elasticsearch using archive distributions (`zip` or `tar.gz`), the variable defaults to `$ES_HOME/config`. If you used package distributions (Debian or RPM), the variable defaults to `/etc/elasticsearch`.
   </note>
3. If your cluster has a single node, add the `discovery.type` setting in the `$ES_PATH_CONF/elasticsearch.yml` file and set the value to `single-node`. This setting ensures that your node does not inadvertently connect to other clusters that might be running on your network.
   ```yaml
   discovery.type: single-node
   ```


## Set passwords for built-in users

To communicate with your cluster, you must configure a password for the `elastic` and `kibana_system` [built-in users](https://www.elastic.co/docs/deploy-manage/users-roles/cluster-or-deployment-auth/built-in-users). Unless you enable anonymous access (not recommended), all requests that don’t include credentials are rejected.
1. On **every** node in your cluster, start Elasticsearch. For example, if you installed Elasticsearch with a `.tar.gz` package, run the following command from the `ES_HOME` directory:
   ```shell
   ./bin/elasticsearch
   ```
   <note>
   If you are following this procedure for a multi-node cluster, you will have to set up [transport TLS certificates](https://www.elastic.co/docs/deploy-manage/security/set-up-basic-security) in your nodes before being able to start the nodes.
   </note>
2. On any node in your cluster, open another terminal window and set the password for the `elastic` built-in user by running the [`elasticsearch-reset-password`](https://www.elastic.co/docs/reference/elasticsearch/command-line-tools/reset-password) utility. This command resets the password to an auto-generated value.
   ```shell
   ./bin/elasticsearch-reset-password -u elastic
   ```
   If you want to set the password to a specific value, run the command with the interactive (`-i`) parameter.
   ```shell
   ./bin/elasticsearch-reset-password -i -u elastic
   ```
3. Set the password for the `kibana_system` built-in user.
   ```shell
   ./bin/elasticsearch-reset-password -u kibana_system
   ```
4. Save the new passwords. In the next step, you’ll add the password for the `kibana_system` user to Kibana.

**Next**: [Configure Kibana to connect to Elasticsearch with a password](#add-built-in-users)

## Configure Kibana to connect to Elasticsearch with a password

When the Elasticsearch security features are enabled, users must log in to Kibana with a valid username and password.
You’ll configure Kibana to use the built-in `kibana_system` user and the password that you created earlier. Kibana performs some background tasks that require use of the `kibana_system` user.
This account is not meant for individual users and does not have permission to log in to Kibana from a browser. Instead, you’ll log in to Kibana as the `elastic` superuser.
1. Add the `elasticsearch.username` setting to the `KBN_PATH_CONF/kibana.yml` file and set the value to the `kibana_system` user:
   ```yaml
   elasticsearch.username: "kibana_system"
   ```
   <note>
   The `KBN_PATH_CONF` variable is the path for the Kibana configuration files. If you installed Kibana using archive distributions (`zip` or `tar.gz`), the variable defaults to `KIB_HOME/config`. If you used package distributions (Debian or RPM), the variable defaults to `/etc/kibana`.
   </note>
2. From the directory where you installed Kibana, run the following commands to create the Kibana keystore and add the [secure settings](https://www.elastic.co/docs/deploy-manage/security/secure-settings):
   1. Create the Kibana keystore:
   ```shell
   ./bin/kibana-keystore create
   ```
2. Add the password for the `kibana_system` user to the Kibana keystore:
   ```shell
   ./bin/kibana-keystore add elasticsearch.password
   ```
   When prompted, enter the password for the `kibana_system` user.
3. Restart Kibana. For example, if you installed Kibana with a `.tar.gz` package, run the following command from the Kibana directory:
   ```shell
   ./bin/kibana
   ```
4. Log in to Kibana as the `elastic` user. Use this superuser account to [manage spaces, create new users, and assign roles](https://www.elastic.co/docs/deploy-manage/users-roles/cluster-or-deployment-auth/quickstart). If you’re running Kibana locally, go to `http://localhost:5601` to view the login page.


## What’s next?

Congratulations! You enabled password protection for your local cluster to prevent unauthorized access. You can log in to Kibana as the `elastic` user and create additional users and roles, but take in mind that your browser connections and the traffic between Kibana and Elasticsearch will still be unencrypted with plain HTTP.
If your cluster has multiple nodes, then you must [configure Transport Layer Security (TLS) between nodes](https://www.elastic.co/docs/deploy-manage/security/set-up-basic-security). [Production mode](/docs/deploy-manage/deploy/self-managed/bootstrap-checks#dev-vs-prod-mode) clusters will not start if you do not enable TLS.
Regardless of your cluster being a [single-node](/docs/deploy-manage/deploy/self-managed/bootstrap-checks#single-node-discovery) or a multi-node cluster, it's highly recommended to [secure the HTTP layer](https://www.elastic.co/docs/deploy-manage/security/set-up-basic-security-plus-https) with TLS certificates.