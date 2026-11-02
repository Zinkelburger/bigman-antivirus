---
url: "https://www.wiz.io/blog/shai-hulud-npm-supply-chain-attack"
title: "Shai-Hulud npm Supply Chain Attack | Wiz Blog"
site: "wiz.io"
published: "2025-09-16T10:04:50-04:00"
captured: "2026-11-02T19:04:38Z"
via: "live"
sha256: "d0c07fd4b3fb90d32b38c0bfa8d36e54c8f51c79676d8e77c8ccf9607ce5d868"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# Shai-Hulud npm Supply Chain Attack | Wiz Blog

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.wiz.io/blog/shai-hulud-npm-supply-chain-attack>

November 24, 2025 update:

This blog covers the first Shai-Hulud supply chain attack, for the second coming of Shai-Hulud visit our new blog: [Shai-Hulud 2.0: Ongoing Supply Chain Attack.](https://www.wiz.io/blog/shai-hulud-2-0-ongoing-supply-chain-attack)

On September 15, 2025, malicious versions of multiple popular packages were published to npm. They contained a post-install script that harvested sensitive data and exfiltrated it to attacker-created public GitHub repos named Shai-Hulud. Beyond data theft, the malware exhibits worm-like behaviour: when a compromised package encounters additional npm tokens in its environment, it will automatically publish malicious versions of any packages it can access - spreading across the npm ecosystem. Wiz Research assesses this campaign is directly downstream of the late-August 2025 [s1ngularity/Nx compromise](https://www.wiz.io/blog/s1ngularity-supply-chain-attack) (initial GitHub token theft to npm token theft to mass package poisoning). As the first [successful](https://jamie.build/how-to-build-an-npm-worm) self-propagating attack in the npm ecosystem, this appears to be one of the most severe JavaScript supply-chain attacks observed to date.

###

Change log

September 17, 2025 10AM UTC: payload analysis section was added.

September 17, 2025 1PM UTC: additional packages and versions have been added to the appendix.

September 17, 2025 2PM UTC: added IOCs section

September 19, 2025 4:30PM UTC: created infographic

Infographic summarizing key analysis of Shai-Hulud worm

#

What happened?

Once a version of one of the malicious packages is installed, the included payload uses the TruffleHog secret scanning tool to identify secrets, in addition to harvesting environment variables and IMDS-exposed cloud keys when available.

Additionally, the script validates collected credentials and, if it finds GitHub tokens, it abuses them in multiple ways:

-

It creates a public repository named Shai-Hulud containing a dump of harvested secrets

-

It pushes a new GitHub Actions workflow to all accessible repositories. This action exfiltrates each repo’s secrets to https://webhook[.]site/bb8ca5f6-4175-45d2-b042-fc9ebb8170b7. It migrates private organizational repositories to public personal repositories under the attacker-controlled user (private org/repo → public user/repo) with the description “Shai-Hulud Migration”, and a -migration suffix added to the name.

This attack is a self-propagating worm. When a compromised package encounters additional npm tokens in a victim environment, it will automatically publish malicious versions of any packages it can access. See further discussion of the malicious code in [Socket's initial analysis](https://socket.dev/blog/tinycolor-supply-chain-attack-affects-40-packages).

Based on victimology, Wiz Research assesses this activity is tied to the recent [s1ngularity / Nx supply chain attack](https://test.wiz.io/boards/threat-center/wiz-adv-2025-084), where initial GitHub token theft enabled the broader chain of compromise and leaking of formerly private repositories. The initial npm packages that started this chain reaction included multiple known-compromised victims of the s1ngularity attack.

###

Payload analysis

The worm deploys a "workflow script" to /tmp/processor.sh, executing it to trigger automated branch creation, triggering GitHub workflows to exfiltrate its secrets. Similarly, it drops a "migration script" at /tmp/migrate-repos.sh, which, once executed, forces private repositories to be cloned and republished as public repositories with the -migration suffix.

During this process, the malware appears to use /tmp/github-migration as a temporary working directory for cloning and related operations - though this is not fully confirmed due to some obfuscation and irregularities in the code. Importantly, the migration routine first creates the repository as private and then immediately flips its visibility to public. This results in two observable GitHub events in quick succession: a CreateEvent followed by a PublicEvent.

Another observation, one of the payloads appears to be AI-generated, another variant lacks that same stylistic pattern, suggesting it may have been copied from an external source that we have not yet identified.

#

What was the impact?

The attacker exfiltrated data using webhook[.]site within a malicious GitHub Action. However, under the free plan used, the webhook can only receive a total of 100 callbacks. We observed that the webhook had been deactivated by the platform for excessive activity. This renders the webhook based exfiltration non-functional. However, secrets will still be exposed in the workflow log on GitHub.

Wiz Research initially observed 36 GitHub users with secrets exposed in the “Shai-Hulud” repo (data.json, double–base64 encoded) and 8 users whose private repositories were force-migrated to public with the label “Shai-Hulud Migration.”

Tracking down repositories with the malicious workflow proved challenging, since GitHub doesn't index deleted branches, commits, or file contents. Still, we were able to uncover 64 additional repositories where a shai-hulud branch had been created, and in most of them, we found a commit containing the malicious workflow. In many cases, the workflow logs exposed sensitive secrets like GitHub tokens, NPM credentials, Atlassian keys, and Datadog API keys. These are still accessible as of writing this blog.

Despite the clear propagation of the worm to additional npm packages, we have not currently observed the creation of further repositories.

Example of successful compromise during the first wave of the attack

Example of a successful compromise during the second wave of the attack

#

Which actions should security teams take?

###

Immediate Remediation

-

Remove malicious versions (rm -rf node_modules && npm cache clean --force). See Appendix: Impacted Packages for a point-in-time list of known malicious packages from this attack

-

Upgrade to a clean release (many of the malicious versions have been removed from npm, but as this is still unfolding it is advised to stay vigilant).

###

Audit & Detection

-

Check for any evidence of GitHub repos created within your organization’s developers private accounts and user accounts, named Shai-Hulud, or private repositories with the suffix -migration and the description Shai-Hulud Migration.

-

Use the following [Github Query](https://github.com/search?q=Shai-Hulud+org%3APLACEHOLDER&type=repositories&s=updated&o=desc)to search for public repositories named Shai-Hulud that were impacted from the first wave of the attack, specifying your org name. Successful compromise would exfiltrate double encoded base64 payload under data.json file path with secrets from the affected machine.

-

Use the following [Github Query](https://github.com/search?q=%22Shai-Hulud+Migration%22+org%3APLACEHOLDER&type=repositories&s=updated&o=desc)to search for public repositories with the description Shai-Hulud Migration, specifying your org name, that were impacted from the second wave of the attack, successful compromise will result in publication of private repositories publicly, named with a -mitigation suffix.

-

Review GitHub audit logs for anomalous API usage.

-

Monitor developer endpoints and CI/CD pipelines for suspicious API calls and unexpected child processes.

###

Credential Rotation

Revoke and regenerate all GitHub tokens, npm tokens, SSH keys, API keys, and environment variable secrets that may have been leaked in these repositories.

#

How can Wiz help?

-

While Wiz Threat Research are still reviewing impact, Wiz customers can use the pre-built query and advisory in the [Wiz Threat Center](https://app.wiz.io/boards/threat-center/wiz-adv-2025-090) to assess the risk in their environment.

-

Wiz customers can use the [SBOM page](https://app.wiz.io/inventory/sbom#~%28filters~%28name~%28equalsAny~%28~%27*40ahmedhfarag*2fngx-perfect-scrollbar~%27*40ahmedhfarag*2fngx-virtual-scroller~%27*40art-ws*2fcommon~%27*40art-ws*2fconfig-eslint~%27*40art-ws*2fconfig-ts~%27*40art-ws*2fdb-context~%27*40art-ws*2fdi~%27*40art-ws*2fdi-node~%27*40art-ws*2feslint~%27*40art-ws*2ffastify-http-server~%27*40art-ws*2fhttp-server~%27*40art-ws*2fopenapi~%27*40art-ws*2fpackage-base~%27*40art-ws*2fprettier~%27*40art-ws*2fslf~%27*40art-ws*2fssl-info~%27*40art-ws*2fweb-app~%27*40crowdstrike*2fcommitlint~%27*40crowdstrike*2ffalcon-shoelace~%27*40crowdstrike*2ffoundry-js~%27*40crowdstrike*2fglide-core~%27*40crowdstrike*2flogscale-dashboard~%27*40crowdstrike*2flogscale-file-editor~%27*40crowdstrike*2flogscale-parser-edit~%27*40crowdstrike*2flogscale-search~%27*40crowdstrike*2ftailwind-toucan-base~%27*40ctrl*2fdeluge~%27*40ctrl*2fgolang-template~%27*40ctrl*2fmagnet-link~%27*40ctrl*2fngx-codemirror~%27*40ctrl*2fngx-csv~%27*40ctrl*2fngx-emoji-mart~%27*40ctrl*2fngx-rightclick~%27*40ctrl*2fqbittorrent~%27*40ctrl*2freact-adsense~%27*40ctrl*2fshared-torrent~%27*40ctrl*2ftinycolor~%27*40ctrl*2ftorrent-file~%27*40ctrl*2ftransmission~%27*40ctrl*2fts-base32~%27*40hestjs*2fcore~%27*40hestjs*2fcqrs~%27*40hestjs*2fdemo~%27*40hestjs*2feslint-config~%27*40hestjs*2flogger~%27*40hestjs*2fscalar~%27*40hestjs*2fvalidation~%27*40nstudio*2fweb~%27*40nstudio*2fweb-angular~%27*40nstudio*2fxplat~%27*40nstudio*2fxplat-utils~%27*40operato*2fboard~%27*40operato*2fdata-grist~%27*40operato*2fgraphql~%27*40operato*2fheadroom~%27*40operato*2fhelp~%27*40operato*2fi18n~%27*40operato*2finput~%27*40operato*2flayout~%27*40operato*2fpopup~%27*40operato*2fpull-to-refresh~%27*40operato*2fshell~%27*40operato*2fstyles~%27*40operato*2futils~%27*40teselagen*2fbounce-loader~%27*40teselagen*2fliquibase-tools~%27*40teselagen*2frange-utils~%27*40teselagen*2freact-list~%27*40teselagen*2freact-table~%27*40thangved*2fcallback-window~%27*40things-factory*2fattachment-base~%27*40things-factory*2fauth-base~%27*40things-factory*2femail-base~%27*40things-factory*2fenv~%27*40things-factory*2fintegration-base~%27*40things-factory*2fintegration-marketplace~%27*40things-factory*2fshell~%27*40tnf-dev*2fapi~%27*40tnf-dev*2fcore~%27*40tnf-dev*2fjs~%27*40tnf-dev*2fmui~%27*40tnf-dev*2freact~%27*40ui-ux-gang*2fdevextreme-angular-rpk~%27*40yoobic*2fdesign-system~%27*40yoobic*2fjpeg-camera-es6~%27*40yoobic*2fyobi~%27airchief~%27airpilot~%27angulartics2~%27browser-webdriver-downloader~%27capacitor-notificationhandler~%27capacitor-plugin-healthapp~%27capacitor-plugin-ihealth~%27capacitor-plugin-vonage~%27capacitorandroidpermissions~%27config-cordova~%27cordova-plugin-voxeet2~%27cordova-voxeet~%27create-hest-app~%27db-evo~%27devextreme-angular-rpk~%27ember-browser-services~%27ember-headless-form~%27ember-headless-form-yup~%27ember-headless-table~%27ember-url-hash-polyfill~%27ember-velcro~%27encounter-playground~%27eslint-config-crowdstrike~%27eslint-config-crowdstrike-node~%27eslint-config-teselagen~%27globalize-rpk~%27graphql-sequelize-teselagen~%27html-to-base64-image~%27json-rules-engine-simplified~%27jumpgate~%27koa2-swagger-ui~%27mcfly-semantic-release~%27mcp-knowledge-base~%27mcp-knowledge-graph~%27mobioffice-cli~%27monorepo-next~%27mstate-angular~%27mstate-cli~%27mstate-dev-react~%27mstate-react~%27ng2-file-upload~%27ngx-bootstrap~%27ngx-color~%27ngx-toastr~%27ngx-trend~%27ngx-ws~%27oradm-to-gql~%27oradm-to-sqlz~%27ove-auto-annotate~%27pm2-gelf-json~%27printjs-rpk~%27react-complaint-image~%27react-jsonschema-form-conditionals~%27remark-preset-lint-crowdstrike~%27rxnt-authentication~%27rxnt-healthchecks-nestjs~%27rxnt-kue~%27swc-plugin-component-annotate~%27tbssnch~%27teselagen-interval-tree~%27tg-client-query-builder~%27tg-redbird~%27tg-seq-gen~%27thangved-react-grid~%27ts-gaussian~%27ts-imports~%27tvi-cli~%27ve-bamreader~%27ve-editor~%27verror-extra~%27voip-callkit~%27wdio-web-reporter~%27yargs-help-output~%27yoo-styles~%27*40nativescript-community*2farraybuffers~%27*40nativescript-community*2fgesturehandler~%27*40nativescript-community*2fperms~%27*40nativescript-community*2fsqlite~%27*40nativescript-community*2ftext~%27*40nativescript-community*2ftypeorm~%27*40nativescript-community*2fui-collectionview~%27*40nativescript-community*2fui-document-picker~%27*40nativescript-community*2fui-drawer~%27*40nativescript-community*2fui-image~%27*40nativescript-community*2fui-label~%27*40nativescript-community*2fui-material-bottom-navigation~%27*40nativescript-community*2fui-material-bottomsheet~%27*40nativescript-community*2fui-material-core~%27*40nativescript-community*2fui-material-core-tabs~%27*40nativescript-community*2fui-material-ripple~%27*40nativescript-community*2fui-material-tabs~%27*40nativescript-community*2fui-pager~%27*40nativescript-community*2fui-pulltorefresh~%27*40nexe*2fconfig-manager~%27*40nexe*2feslint-config~%27*40nexe*2flogger~%27*40nstudio*2fangular~%27*40nstudio*2ffocus~%27*40nstudio*2fnativescript-checkbox~%27*40nstudio*2fnativescript-loading-indicator~%27*40nstudio*2fui-collectionview%29%29%29%29) to identify malicious versions and packages in their environment.

-

Wiz Threat Research added [YARA-based detection](https://app.wiz.io/explorer/graph#~%28query~%28relationships~%28~%28type~%28~%28type~%27INFECTS%29%29~with~%28select~true~type~%28~%27VIRTUAL_MACHINE~%27CONTAINER_IMAGE~%27BUCKET~%27SERVERLESS~%27CONTAINER~%27REPOSITORY_BRANCH%29~where~%28userMetadata_isIgnored~%28EQUALS~false%29%29%29%29~%28type~%28~%28type~%27INSTANCE_OF%29%29~with~%28select~true~type~%28~%27MALWARE%29%29%29%29~select~true~type~%28~%27MALWARE_INSTANCE%29~where~%28_or~%28~%28sha256~%28EQUALS~%28~%2746faab8ab153fae6e80e7cca38eab363075bb524edd79e42269217a083628f09%29%29%29~%28familyName~%28EQUALS~%28~%27JS-Script.InfoStealer.ShaiHulud%29%29%29%29%29%29~view~%27table%29) to identify instances of the malicious script in customer environments, which should be treated as evidence of potential compromise of the host

-

Wiz Defend customers can use [Threat Detection rules](https://app.wiz.io/threats#~%28filters~%28status~%28equals~%28~%27OPEN~%27IN_PROGRESS%29%29~sourceRule~%28equals~%28~%27cer-correlation-id-201~%27cer-correlation-id-203~%27cer-azure-control-apiCallsUsingOffensiveTool~%27cer-azure-data-apiCallByKnownOffensiveTool~%27cer-gcp-control-apiCallsUsingOffensiveTool~%27cer-sen-id-1092%29%29~createdAt~%28dateRange~%28past~%28amount~14~unit~%27day%29%29%29%29%29) to identify the malicious behaviors from this attack in their environments.

###

Investigate emerging threats with Mika AI / Wiz MCP Threat Intel investigation tool

Mika AI, Wiz's AI assistant, can investigate new threat center items, analyze your environment for impact, and create recommendation plan to minimize impact, turning hours of manual analysis into minutes of AI-powered insights.

Simply ask Mika AI, "How do I know if I'm affected by the latest Shai-Hulud NPM Supply Chain Attack?", and get comprehensive analysis, based on all Wiz capabilities of affected resources, exposure level, real-time threats and risk, and receive step-by-step remediation recommendations.

Investigate emerging threats with Mika AI:

Investigate emerging threats with Wiz MCP:

#

Indicators of compromise

-

/tmp/processor.sh - Creates shai-hulud branch and uploads the workflow payload

-

/tmp/migrate-repos.sh- Migrates private repos to public

-

46faab8ab153fae6e80e7cca38eab363075bb524edd79e42269217a083628f09 - Malicious JS file hash (SHA256)

-

webhook[.]site/bb8ca5f6-4175-45d2-b042-fc9ebb8170b7 - Webhook

#

Appendix: Impacted Packages

-

@ctrl/deluge (7.2.2, 7.2.1)

-

@ctrl/golang-template (1.4.3, 1.4.2)

-

@ctrl/magnet-link (4.0.4, 4.0.3)

-

@ctrl/ngx-codemirror (7.0.2, 7.0.1)

-

@ctrl/ngx-csv (6.0.2, 6.0.1)

-

@ctrl/ngx-emoji-mart (9.2.2, 9.2.1)

-

@ctrl/ngx-rightclick (4.0.2, 4.0.1)

-

@ctrl/qbittorrent (9.7.2, 9.7.1)

-

@ctrl/react-adsense (2.0.2, 2.0.1)

-

@ctrl/shared-torrent (6.3.2, 6.3.1)

-

@ctrl/tinycolor@4.1.1, (4.1.2)

-

@ctrl/torrent-file (4.1.2, 4.1.1)

-

@ctrl/transmission (7.3.1)

-

@ctrl/ts-base32 (4.0.2, 4.0.1)

-

@nativescript-community/gesturehandler (2.0.35)

-

@nativescript-community/sentry (4.6.43)

-

@nativescript-community/text (1.6.13, 1.6.10, 1.6.11, 1.6.12, 1.6.9)

-

@nativescript-community/ui-collectionview (6.0.6)

-

@nativescript-community/ui-drawer (0.1.30)

-

@nativescript-community/ui-image (4.5.6)

-

@nativescript-community/ui-material-bottomsheet (7.2.72)

-

@nativescript-community/ui-material-core (7.2.76, 7.2.72, 7.2.73, 7.2.74, 7.2.75)

-

@nativescript-community/ui-material-core-tabs (7.2.76, 7.2.72, 7.2.73, 7.2.74, 7.2.75)

-

@teselagen/bio-parsers (0.4.29, 0.4.30)

-

@teselagen/bounce-loader (0.3.16, 0.3.17)

-

@teselagen/file-utils (0.3.21, 0.3.22)

-

@teselagen/liquibase-tools (0.4.1)

-

@teselagen/ove (0.7.39, 0.7.40)

-

@teselagen/range-utils (0.3.14, 0.3.15)

-

@teselagen/react-list (0.8.19, 0.8.20)

-

@teselagen/react-table (6.10.21, 6.10.19, 6.10.20, 6.10.22)

-

@teselagen/sequence-utils (0.3.33, 0.3.34)

-

@teselagen/ui (0.9.9, 0.9.10)

-

angulartics2 (14.1.2, 14.1.1)

-

encounter-playground (0.0.4, 0.0.5, 0.0.2, 0.0.3)

-

eslint-config-teselagen (6.1.7, 6.1.8)

-

graphql-sequelize-teselagen (5.3.8, 5.3.9)

-

json-rules-engine-simplified (0.2.3, 0.2.4, 0.2.1)

-

koa2-swagger-ui (5.11.2, 5.11.1)

-

ng2-file-upload (8.0.3, 7.0.2, 7.0.3, 8.0.1, 8.0.2, 9.0.1)

-

ngx-bootstrap (18.1.4, 19.0.3, 20.0.4, 20.0.5, 20.0.6, 19.0.4, 20.0.3)

-

ngx-color (10.0.2, 10.0.1)

-

ngx-toastr (19.0.2, 19.0.1)

-

ngx-trend (8.0.1)

-

oradm-to-gql (35.0.14, 35.0.15)

-

oradm-to-sqlz (1.1.4, 1.1.2)

-

ove-auto-annotate (0.0.9, 0.0.10)

-

react-complaint-image (0.0.34, 0.0.35, 0.0.32)

-

react-jsonschema-form-conditionals (0.3.20, 0.3.21, 0.3.18)

-

react-jsonschema-form-extras (1.0.3, 1.0.4)

-

react-jsonschema-rxnt-extras (0.4.8, 0.4.9)

-

rxnt-authentication (0.0.5, 0.0.6, 0.0.3, 0.0.4)

-

rxnt-healthchecks-nestjs (1.0.4, 1.0.5, 1.0.2, 1.0.3)

-

rxnt-kue (1.0.6, 1.0.7, 1.0.4, 1.0.5)

-

swc-plugin-component-annotate (1.9.2, 1.9.1)

-

tg-client-query-builder (2.14.4, 2.14.5)

-

tg-redbird (1.3.1, 1.3.2)

-

tg-seq-gen (1.0.9, 1.0.10)

-

ts-gaussian (3.0.6, 3.0.5)

-

ve-bamreader (0.2.6, 0.2.7)

-

ve-editor (1.0.1, 1.0.2)

-

@ahmedhfarag/ngx-perfect-scrollbar (20.0.20)

-

@ahmedhfarag/ngx-virtual-scroller (4.0.4)

-

@art-ws/common (2.0.28)

-

@art-ws/config-eslint (2.0.4, 2.0.5)

-

@art-ws/config-ts (2.0.7, 2.0.8)

-

@art-ws/db-context (2.0.24)

-

@art-ws/di-node (2.0.13)

-

@art-ws/di (2.0.28, 2.0.32)

-

@art-ws/eslint (1.0.5, 1.0.6)

-

@art-ws/fastify-http-server (2.0.24, 2.0.27)

-

@art-ws/http-server (2.0.21, 2.0.25)

-

@art-ws/openapi (0.1.12, 0.1.9)

-

@art-ws/package-base (1.0.5, 1.0.6)

-

@art-ws/prettier (1.0.5, 1.0.6)

-

@art-ws/slf (2.0.15, 2.0.22)

-

@art-ws/ssl-info (1.0.10, 1.0.9)

-

@art-ws/web-app (1.0.3, 1.0.4)

-

@crowdstrike/commitlint (8.1.1, 8.1.2)

-

@crowdstrike/falcon-shoelace (0.4.1, 0.4.2)

-

@crowdstrike/foundry-js (0.19.1, 0.19.2)

-

@crowdstrike/glide-core (0.34.2, 0.34.3)

-

@crowdstrike/logscale-dashboard (1.205.1, 1.205.2)

-

@crowdstrike/logscale-file-editor (1.205.1, 1.205.2)

-

@crowdstrike/logscale-parser-edit (1.205.1, 1.205.2)

-

@crowdstrike/logscale-search (1.205.1, 1.205.2)

-

@crowdstrike/tailwind-toucan-base (5.0.1, 5.0.2)

-

@ctrl/tinycolor (4.1.1, 4.1.2)

-

@hestjs/core (0.2.1)

-

@hestjs/cqrs (0.1.6)

-

@hestjs/demo (0.1.2)

-

@hestjs/eslint-config (0.1.2)

-

@hestjs/logger (0.1.6)

-

@hestjs/scalar (0.1.7)

-

@hestjs/validation (0.1.6)

-

@nativescript-community/arraybuffers (1.1.6, 1.1.7, 1.1.8)

-

@nativescript-community/perms (3.0.5, 3.0.6, 3.0.7, 3.0.8, 3.0.9)

-

@nativescript-community/sqlite (3.5.2, 3.5.3, 3.5.4, 3.5.5)

-

@nativescript-community/typeorm (0.2.30, 0.2.31, 0.2.32, 0.2.33)

-

@nativescript-community/ui-document-picker (1.1.27, 1.1.28, 13.0.32)

-

@nativescript-community/ui-label (1.3.35, 1.3.36, 1.3.37)

-

@nativescript-community/ui-material-bottom-navigation (7.2.72, 7.2.73, 7.2.74, 7.2.75)

-

@nativescript-community/ui-material-ripple (7.2.72, 7.2.73, 7.2.74, 7.2.75)

-

@nativescript-community/ui-material-tabs (7.2.72, 7.2.73, 7.2.74, 7.2.75)

-

@nativescript-community/ui-pager (14.1.36, 14.1.37, 14.1.38)

-

@nativescript-community/ui-pulltorefresh (2.5.4, 2.5.5, 2.5.6, 2.5.7)

-

@nexe/config-manager (0.1.1)

-

@nexe/eslint-config (0.1.1)

-

@nexe/logger (0.1.3)

-

@nstudio/angular (20.0.4, 20.0.5, 20.0.6)

-

@nstudio/focus (20.0.4, 20.0.5, 20.0.6)

-

@nstudio/nativescript-checkbox (2.0.6, 2.0.7, 2.0.8, 2.0.9)

-

@nstudio/nativescript-loading-indicator (5.0.1, 5.0.2, 5.0.3, 5.0.4)

-

@nstudio/ui-collectionview (5.1.11, 5.1.12, 5.1.13, 5.1.14)

-

@nstudio/web-angular (20.0.4)

-

@nstudio/web (20.0.4)

-

@nstudio/xplat-utils (20.0.5, 20.0.6, 20.0.7)

-

@nstudio/xplat (20.0.5, 20.0.6, 20.0.7)

-

@operato/board (9.0.36, 9.0.37, 9.0.38, 9.0.39, 9.0.40, 9.0.41, 9.0.42, 9.0.43, 9.0.44, 9.0.45, 9.0.46)

-

@operato/data-grist (9.0.29, 9.0.35, 9.0.36, 9.0.37)

-

@operato/graphql (9.0.22, 9.0.35, 9.0.36, 9.0.37, 9.0.38, 9.0.39, 9.0.40, 9.0.41, 9.0.42, 9.0.43, 9.0.44, 9.0.45, 9.0.46)

-

@operato/headroom (9.0.2, 9.0.35, 9.0.36, 9.0.37)

-

@operato/help (9.0.35, 9.0.36, 9.0.37, 9.0.38, 9.0.39, 9.0.40, 9.0.41, 9.0.42, 9.0.43, 9.0.44, 9.0.45, 9.0.46)

-

@operato/i18n (9.0.35, 9.0.36, 9.0.37)

-

@operato/input (9.0.27, 9.0.35, 9.0.36, 9.0.37, 9.0.38, 9.0.39, 9.0.40, 9.0.41, 9.0.42, 9.0.43, 9.0.44, 9.0.45, 9.0.46, 9.0.47, 9.0.48)

-

@operato/layout (9.0.35, 9.0.36, 9.0.37)

-

@operato/popup (9.0.22, 9.0.35, 9.0.36, 9.0.37, 9.0.38, 9.0.39, 9.0.40, 9.0.41, 9.0.42, 9.0.43, 9.0.44, 9.0.45, 9.0.46, 9.0.49)

-

@operato/pull-to-refresh (9.0.36, 9.0.37, 9.0.38, 9.0.39, 9.0.40, 9.0.41, 9.0.42)

-

@operato/shell (9.0.22, 9.0.35, 9.0.36, 9.0.37, 9.0.38, 9.0.39)

-

@operato/styles (9.0.2, 9.0.35, 9.0.36, 9.0.37)

-

@operato/utils (9.0.22, 9.0.35, 9.0.36, 9.0.37, 9.0.38, 9.0.39, 9.0.40, 9.0.41, 9.0.42, 9.0.43, 9.0.44, 9.0.45, 9.0.46, 9.0.49)

-

@thangved/callback-window (1.1.4)

-

@things-factory/attachment-base (9.0.42, 9.0.43, 9.0.44, 9.0.45, 9.0.46, 9.0.47, 9.0.48, 9.0.49, 9.0.50, 9.0.51, 9.0.52, 9.0.53, 9.0.54, 9.0.55)

-

@things-factory/auth-base (9.0.42, 9.0.43, 9.0.44, 9.0.45)

-

@things-factory/email-base (9.0.42, 9.0.43, 9.0.44, 9.0.45, 9.0.46, 9.0.47, 9.0.48, 9.0.49, 9.0.50, 9.0.51, 9.0.52, 9.0.53, 9.0.54, 9.0.55, 9.0.56, 9.0.57, 9.0.58, 9.0.59)

-

@things-factory/env (9.0.42, 9.0.43, 9.0.44, 9.0.45)

-

@things-factory/integration-base (9.0.42, 9.0.43, 9.0.44, 9.0.45)

-

@things-factory/integration-marketplace (9.0.42, 9.0.43, 9.0.44, 9.0.45)

-

@things-factory/shell (9.0.42, 9.0.43, 9.0.44, 9.0.45)

-

@tnf-dev/api (1.0.8)

-

@tnf-dev/core (1.0.8)

-

@tnf-dev/js (1.0.8)

-

@tnf-dev/mui (1.0.8)

-

@tnf-dev/react (1.0.8)

-

@ui-ux-gang/devextreme-angular-rpk (24.1.7)

-

@yoobic/design-system (6.5.17)

-

@yoobic/jpeg-camera-es6 (1.0.13)

-

@yoobic/yobi (8.7.53)

-

airchief (0.3.1)

-

airpilot (0.8.8)

-

browser-webdriver-downloader (3.0.8)

-

capacitor-notificationhandler (0.0.2, 0.0.3)

-

capacitor-plugin-healthapp (0.0.2, 0.0.3)

-

capacitor-plugin-ihealth (1.1.8, 1.1.9)

-

capacitor-plugin-vonage (1.0.2, 1.0.3)

-

capacitorandroidpermissions (0.0.4, 0.0.5)

-

config-cordova (0.8.5)

-

cordova-plugin-voxeet2 (1.0.24)

-

cordova-voxeet (1.0.32)

-

create-hest-app (0.1.9)

-

db-evo (1.1.4, 1.1.5)

-

devextreme-angular-rpk (21.2.8)

-

ember-browser-services (5.0.2, 5.0.3)

-

ember-headless-form-yup (1.0.1)

-

ember-headless-form (1.1.2, 1.1.3)

-

ember-headless-table (2.1.5, 2.1.6)

-

ember-url-hash-polyfill (1.0.12, 1.0.13)

-

ember-velcro (2.2.1, 2.2.2)

-

eslint-config-crowdstrike-node (4.0.3, 4.0.4)

-

eslint-config-crowdstrike (11.0.2, 11.0.3)

-

globalize-rpk (1.7.4)

-

html-to-base64-image (1.0.2)

-

jumpgate (0.0.2)

-

mcfly-semantic-release (1.3.1)

-

mcp-knowledge-base (0.0.2)

-

mcp-knowledge-graph (1.2.1)

-

mobioffice-cli (1.0.3)

-

monorepo-next (13.0.1, 13.0.2)

-

mstate-angular (0.4.4)

-

mstate-cli (0.4.7)

-

mstate-dev-react (1.1.1)

-

mstate-react (1.6.5)

-

ngx-ws (1.1.5, 1.1.6)

-

pm2-gelf-json (1.0.4, 1.0.5)

-

printjs-rpk (1.6.1)

-

remark-preset-lint-crowdstrike (4.0.1, 4.0.2)

-

tbssnch (1.0.2)

-

teselagen-interval-tree (1.1.2)

-

thangved-react-grid (1.0.3)

-

ts-imports (1.0.1, 1.0.2)

-

tvi-cli (0.1.5)

-

verror-extra (6.0.1)

-

voip-callkit (1.0.2, 1.0.3)

-

wdio-web-reporter (0.1.3)

-

yargs-help-output (5.0.3)

-

yoo-styles (6.0.326)

-

devextreme-rpk (21.2.8)

-

@basic-ui-components-stc/basic-ui-components (1.0.5)

#

References

-

[Socket blogpost](https://socket.dev/blog/tinycolor-supply-chain-attack-affects-40-packages)

-

[ReversingLabs tweet](https://x.com/ReversingLabs/status/1967576549873381431)

-

[NGX Bootstrap issue](https://github.com/valor-software/ngx-bootstrap/issues/6776)

-

[NGX Bootstrap advisory](https://github.com/advisories/GHSA-6m4g-vm7c-f8w6)

-

[Daniel Pereira LinkedIn post](https://www.linkedin.com/feed/update/urn:li:activity:7373489836437114880/)

-

[The Hacker News article](https://thehackernews.com/2025/09/40-npm-packages-compromised-in-supply.html)

-

[Aikido developer advocate Mackenzie Jackson post](https://www.linkedin.com/posts/advocatemack_crowdstrike-has-had-its-npm-packages-compromised-activity-7373625746822696960-w8xz?utm_source=share&utm_medium=member_desktop&rcm=ACoAACzR6wYBhL7V5NomFDXjxooQFyCMC5oQHzU)

Tags

[#Research](https://www.wiz.io/blog/tag/research)[#Security](https://www.wiz.io/blog/tag/security)[#Threat Intel](https://www.wiz.io/blog/tag/threat-intel)

https://www.wiz.io/blog/dora-compliance-in-the-cloud-era-insights-from-deloitte-and-wiz

[### DORA Compliance in the Cloud Era: Insights from Deloitte and Wiz](https://www.wiz.io/blog/dora-compliance-in-the-cloud-era-insights-from-deloitte-and-wiz)

https://www.wiz.io/authors/zlatko-ungerhttps://www.wiz.io/authors/ida-stanarhttps://www.wiz.io/authors/robbert-van-der-polhttps://www.wiz.io/authors/nathalie-steenvoorden

[Zlatko Unger](https://www.wiz.io/authors/zlatko-unger), [Ida Stanar](https://www.wiz.io/authors/ida-stanar), [Robbert Van Der Pol](https://www.wiz.io/authors/robbert-van-der-pol), [Nathalie Steenvoorden](https://www.wiz.io/authors/nathalie-steenvoorden)

September 12, 2025

How to address DORA compliance challenges with Wiz and Deloitte.

https://www.wiz.io/blog/wiz-customers-on-how-ai-is-changing-security

[### How Wiz Customers like Brex and FICO See AI Changing Security](https://www.wiz.io/blog/wiz-customers-on-how-ai-is-changing-security)

https://www.wiz.io/authors/snegha

[Snegha Ramnarayanan](https://www.wiz.io/authors/snegha)

September 11, 2025

Executives from Brex and FICO share how AI is reshaping security strategies.

https://www.wiz.io/blog/wiz-named-aspm-leader-by-idc

[### Wiz Recognized as a Leader in the 2025 IDC MarketScape for ASPM](https://www.wiz.io/blog/wiz-named-aspm-leader-by-idc)

https://www.wiz.io/authors/jiong-liu

[Jiong Liu](https://www.wiz.io/authors/jiong-liu)

September 9, 2025

We believe recognition in the IDC MarketScape for ASPM reflects our commitment to redefining how modern/cloud and AI-native applications are built and secured.
