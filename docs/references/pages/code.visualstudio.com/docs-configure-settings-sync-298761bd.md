---
url: "https://code.visualstudio.com/docs/configure/settings-sync"
title: "Settings Sync"
captured: "2026-11-02T18:25:22Z"
via: "live"
sha256: "1bdedc33ae9cfe7b63f8c3f8e2531f66612d9ab088394d06afdd67fa5caf6d81"
wayback: "https://web.archive.org/web/20261020175718/https://code.visualstudio.com/docs/configure/settings-sync"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# Settings Sync

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://code.visualstudio.com/docs/configure/settings-sync>
> Snapshot: <https://web.archive.org/web/20261020175718/https://code.visualstudio.com/docs/configure/settings-sync>

# Settings Sync

-
-  [View as Markdown](https://code.visualstudio.com/docs/configure/settings-sync.md)
-
-  Ask in VS Code
-  Ask in VS Code Insiders

Settings Sync keeps your Visual Studio Code preferences consistent across devices. Use this article to turn on synchronization, choose which data to sync, restore or delete synced data, and troubleshoot synchronization and credential-storage issues.

## Turn on Settings Sync

To turn on Settings Sync, follow these steps:

-

Select Backup and Sync Settings... from the Manage gear menu or the Accounts menu at the bottom of the Activity Bar.

-

Select the data that you want to synchronize.

-

Select Sign in, and then select a Microsoft or GitHub account.

-

If you are not already signed in with the selected account, complete the authentication flow in the browser.

After you sign in, Settings Sync automatically merges your local and cloud data and continues to synchronize changes in the background. If it cannot merge the data, you are prompted to [resolve the conflicts](https://code.visualstudio.com/docs/configure/settings-sync#_resolve-conflicts).

## Configure synced data

Run the Settings Sync: Configure... command or select Settings Sync is On > Settings Sync: Configure... from the Manage gear menu. Choose from the following data categories:

| Data category  | Synchronized data and behavior

| Settings  | [User settings](https://code.visualstudio.com/docs/configure/settings), except settings with the machine or machine-overridable scope and settings that you exclude.

| Keyboard Shortcuts  | User-defined keyboard shortcuts. Shortcuts are synchronized separately for each operating system by default.

| Snippets  | [User snippets](https://code.visualstudio.com/docs/editing/userdefinedsnippets).

| Tasks  | [User-level tasks](https://code.visualstudio.com/docs/debugtest/tasks#_user-level-tasks). Workspace tasks are not synchronized.

| MCP Servers  | [User-profile MCP server configurations](https://code.visualstudio.com/docs/agents/run/manage-mcp-servers#_synchronize-mcp-configuration-across-devices).

| UI State  | Display language, Activity Bar and Panel entries, view layout and visibility, recently used commands, and 'Do not show again' notification choices.

| Extensions  | Installed extensions and the global enablement state of built-in and installed extensions.

| Profiles  | [Profiles](https://code.visualstudio.com/docs/configure/profiles) and their data for the selected categories. You can synchronize up to 20 profiles.

| Prompts and Instructions  | [User prompt files](https://code.visualstudio.com/docs/agent-customization/prompt-files#_sync-user-prompt-files-across-devices) and [Local agent instructions](https://code.visualstudio.com/docs/agent-customization/custom-instructions#_sync-local-agent-instructions-across-devices) that are stored in your user profile.

Note

VS Code does not synchronize your extensions to or from a [remote](https://code.visualstudio.com/docs/remote/remote-overview) window, such as when you're connected to SSH, a development container (devcontainer), or WSL.

### Exclude settings and extensions

To exclude a user setting from synchronization, use the setting's actions in the Settings editor or add the setting to the    settingsSync.ignoredSettings    Open in VS Code Open in VS Code Insiders   setting.

To exclude an extension, use the extension's actions in the Extensions view (⇧⌘X (Windows, Linux Ctrl+Shift+X)) or add the extension to the    settingsSync.ignoredExtensions    Open in VS Code Open in VS Code Insiders   setting.

### Use the same keyboard shortcuts across operating systems

To use the same keyboard shortcuts on every operating system, clear the    settingsSync.keybindingsPerPlatform    Open in VS Code Open in VS Code Insiders   setting.

## Resolve conflicts

Conflicts can occur when you first turn on Settings Sync on a device or when you change data while a device is offline. Synchronization pauses until you resolve the conflicts.

The available actions depend on whether you are turning on sync or resolving a later conflict:

- Accept Local or Replace Remote uses your local data and overwrites the data in the cloud.
- Accept Remote or Replace Local uses the data in the cloud and overwrites your local data.
- Show Conflicts opens a diff editor where you can compare the local and remote data. Edit the merge result, and then select Complete Merge.

## Switch accounts

To synchronize your data with a different account, run the Settings Sync: Turn Off command, and then turn on Settings Sync with the other account.

## Synchronize Stable and Insiders

By default, the VS Code Stable and [Insiders](https://code.visualstudio.com/insiders) builds use separate Settings Sync services and do not share data. To share data between the builds, select the Stable sync service when you turn on Settings Sync in VS Code Insiders.

Note

Synchronizing Stable and Insiders can cause data incompatibility because Insiders is newer than Stable. If this occurs, Settings Sync turns off automatically in Stable. Update Stable to a compatible version before you turn on sync again.

## Restore synced data

VS Code stores local and remote backups of your preferences. You can use these backups to restore an earlier version of your data.

Run the Settings Sync: Show Synced Data command to view remote backups. To view local backups in the same view, open the Views submenu from the Settings Sync view overflow menu, and then select Sync Activity (Local).

To access local backups on disk, run the Settings Sync: Open Local Backups Folder command. The folder is organized by data category and contains timestamped versions of your JSON files.

Note

Local backups are deleted after 30 days. For remote backups, the latest 20 versions of each data category are retained.

## Manage synced machines

VS Code tracks the devices that synchronize your data. Run the Settings Sync: Show Synced Data command, and then expand Synced Machines to view them.

Each device has a default name based on its operating system and whether it runs Stable or Insiders. Use the actions for a device to rename it or turn off Settings Sync remotely.

## Turn off and delete synced data

To stop synchronizing on the current device, select Settings Sync is On > Settings Sync: Turn Off from the Manage gear menu, and then select Turn off.

To turn off synchronization on all your devices and remove your data from the service, select the checkbox labeled Turn off sync on all your devices and clear the data from the cloud. Then select Turn off. If you turn on Settings Sync again, it starts as a first-time setup.

## Troubleshoot Settings Sync

### Resolve data limits

Settings, keyboard shortcuts, and user tasks must each be 100 KB or smaller. You can synchronize up to 20 profiles. If your data exceeds a limit, Settings Sync reports an error and disables an affected category that was already synchronizing. Reduce the file size or number of profiles. Then turn on Settings Sync again, or run the Settings Sync: Configure... command to enable the affected category.

### Troubleshoot credential-store issues

On desktop, Settings Sync stores authentication information by using the operating system credential store. This section uses keychain as a general term for a keychain, keyring, wallet, or credential store.

If the keychain is unavailable or misconfigured, restart VS Code with the following options to generate a verbose log:

```
code --verbose --vmodule="*/components/os_crypt/*=1"

```

#### Windows and macOS

Windows and macOS usually do not require additional keychain configuration. If the problem continues, [report the issue](https://code.visualstudio.com/docs/configure/settings-sync#_collect-logs-and-report-an-issue) and include the verbose log.

#### Linux

VS Code uses Chromium to detect the desktop environment and select a keyring. Search the verbose log for OSCrypt, password storage, or selected backend messages to identify the selected keyring.

##### GNOME or Unity

If the log contains Cannot create an item in a locked collection, unlock the default keyring, which is usually named Login. You can use a keyring manager such as [Seahorse](https://wiki.gnome.org/Apps/Seahorse). The keyring must be unlocked when you sign in to the operating system.

##### KDE

Open [KWalletManager](https://apps.kde.org/kwalletmanager5/) and make sure that the default kdewallet wallet is open. If VS Code cannot connect to KWallet, try a keyring that implements the Secret Service API, as described in the next section.

##### Configure a keyring backend

To select a keyring backend manually, start VS Code with the password-store option. For example, install a keyring that implements the [Secret Service API](https://specifications.freedesktop.org/secret-service/latest/), and then run:

```
code --password-store="gnome-libsecret"

```

If the selected backend works, run Preferences: Configure Runtime Arguments from the Command Palette (⇧⌘P (Windows, Linux Ctrl+Shift+P)) and add "password-store": "gnome-libsecret" to the argv.json file.

The password-store option supports these values:

- kwallet5 for KWallet 5.
- gnome-libsecret for keyrings that implement the Secret Service API, such as GNOME Keyring, KWallet, and KeePassXC.
- kwallet for older KWallet versions.
- basic for basic text encryption. This option is not recommended.

If the desktop environment or keyring is not detected, [report the issue](https://code.visualstudio.com/docs/configure/settings-sync#_collect-logs-and-report-an-issue) and include the verbose log.

##### Configure basic text encryption

Warning

Basic text encryption uses a key derived from a value hardcoded in Chromium. It provides obfuscation rather than secure encryption, and processes on your system might be able to decrypt the stored data.

If you accept this risk, run Preferences: Configure Runtime Arguments from the Command Palette (⇧⌘P (Windows, Linux Ctrl+Shift+P)) and add "password-store": "basic" to the argv.json file.

### Collect logs and report an issue

Settings Sync activity is recorded in the Log (Settings Sync) output channel. Authentication activity is recorded in the Account output channel.

When you [report an issue](https://github.com/microsoft/vscode/issues/new/choose), include the relevant output channels. For credential-store problems, also include the verbose log from the previous section.

## Common questions

### Is Settings Sync the same as the Settings Sync extension?

No. The [Settings Sync extension](https://marketplace.visualstudio.com/items?itemName=Shan.code-settings-sync) by [Shan Khan](https://marketplace.visualstudio.com/publishers/Shan) uses a private GitHub Gist to share settings. It is unrelated to the built-in Settings Sync feature.

### What accounts can I use?

Settings Sync supports Microsoft and GitHub accounts. GitHub Enterprise Server accounts are not supported.

Note

Settings Sync does not support [Microsoft Sovereign Cloud](https://www.microsoft.com/en-us/industry/sovereignty/cloud) accounts.

### Can I use a different backend or service?

No. Settings Sync uses a dedicated service to store data and coordinate updates. Custom backends are not supported.

## Related resources

- [Configure user and workspace settings](https://code.visualstudio.com/docs/configure/settings)
- [Create and manage profiles](https://code.visualstudio.com/docs/configure/profiles)
- [Synchronize extension global state](https://code.visualstudio.com/api/extension-capabilities/common-capabilities#data-storage)

10/28/2026
