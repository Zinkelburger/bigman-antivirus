---
url: "https://bitwarden.com/help/cli/"
title: "Password Manager CLI | Bitwarden"
site: "Bitwarden"
captured: "2026-11-02T18:21:50Z"
via: "live"
sha256: "ec39b99b20e0a8a1aa7dddb151adfaa6204572fc32336e277479a1902c6ebf03"
wayback: "https://web.archive.org/web/20261024060318/https://bitwarden.com/help/cli/"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# Password Manager CLI | Bitwarden

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://bitwarden.com/help/cli/>
> Snapshot: <https://web.archive.org/web/20261024060318/https://bitwarden.com/help/cli/>

# Password Manager CLI

The Bitwarden command-line interface (CLI) is a powerful, fully-featured tool for accessing and managing your vault. Most features that you find in other Bitwarden client applications (desktop, browser extension, etc.) are available from the CLI.

Bitwarden CLI

The Bitwarden CLI is self-documented. From the command line, learn about the available commands using:

```

Bash

bw --help

```

Or, pass --help as an option on any bw command to see available options and examples:

```

Bash

bw list --help

bw move --help

```

Most information you'll need can be accessed using --help, however this article replicates all that information and goes into greater depth on some topics.

https://bitwarden.com/help/cli/#download-and-install

## Download and install

The CLI can be used cross-platform on Windows, macOS, and Linux distributions. To download and install the Bitwarden CLI:

note

For arm64 devices, install the CLI using npm.

Natively packaged versions of the CLI are available for each platform and have no dependencies. Download using one of these links:

-

[Windows x64](https://bitwarden.com/download/?app=cli&platform=windows)

-

[macOS x64](https://bitwarden.com/download/?app=cli&platform=macos)

-

[Linux x64](https://bitwarden.com/download/?app=cli&platform=linux)

Note that, when using the downloaded native executable, you'll need to add the executable to your PATH or else run commands from the directory the file is downloaded to.

tip

In Linux and UNIX systems, you might get a Permission denied message. If you do, grant permission by running:

```

Bash

chmod +x </path/to/executable>

```

For each bundle of the Password Manager CLI available on GitHub, there is an OSS (e.g. bw-oss-windows-2024.12.0.zip) and non-OSS build (e.g. bw-windows-2024.12.0.zip). The non-OSS version is the default package distributed on distribution platforms and includes features under a non-OSS license, such as [device approval](https://bitwarden.com/help/cli/#device-approval) commands, that the OSS version lacks.

tip

The Bitwarden Password Manager CLI build pipeline creates SHA-256 checksum files that are available on GitHub. [Learn how to validate checksums for the CLI](https://bitwarden.com/help/security-faqs/#tab-cli-4iwx6mhPS3Bgu3eLpNthUw/).

https://bitwarden.com/help/cli/#log-in

## Log in

Before logging in, make sure your CLI is connected to the correct server (for example, [EU cloud](https://bitwarden.com/help/server-geographies/) or self-hosted) using the config command ([learn more](https://bitwarden.com/help/cli/#config)). There are three methods for logging in to the Bitwarden CLI using the login command, each of which is suited to different situations. Please review the following options to determine which method to use:

-

[Using email and master password](https://bitwarden.com/help/cli/#using-email-and-password)

-

[Using an API key](https://bitwarden.com/help/cli/#using-an-api-key)

-

[Using SSO](https://bitwarden.com/help/cli/#using-sso)

tip

No matter which option you use, a master password will be required to unlock the client in order to access data with a [session key](https://bitwarden.com/help/cli/#unlock). The [email and master password](https://bitwarden.com/help/cli/#using-email-and-password) option will authenticate your identity and generate a session key simultaneously, however the [API key](https://bitwarden.com/help/cli/#using-an-api-key) or [SSO](https://bitwarden.com/help/cli/#using-sso) will require you subsequent use of the unlock command to generate a session key if you will be working with data directly.

Users who [join an organization using trusted devices](https://bitwarden.com/help/about-trusted-devices/#impact-on-master-passwords) will not be able to access data using the CLI. There are, however, a few commands that do not require decrypted data and therefore can be used without a master password, including config, encode, generate, update, and status.

https://bitwarden.com/help/cli/#using-email-and-password

### Using email and password

Logging in with email and password is recommended for interactive sessions. To log in with email and password:

```

Bash

bw login

```

This will initiate a prompt for your Email Address, Master Password, and ([if enabled](https://bitwarden.com/help/setup-two-step-login/)) at Two-step Login code. The CLI currently supports two-step login via [authenticator](https://bitwarden.com/help/setup-two-step-login-authenticator/), [email](https://bitwarden.com/help/setup-two-step-login-email/), or [Yubikey](https://bitwarden.com/help/setup-two-step-login-yubikey/).

You can string these factors together into a single command as in the following example, however this isn't recommended for security reasons:

```

Bash

bw login [email] [password] --method <method> --code <code>

```

See [Enums](https://bitwarden.com/help/cli/#enums) for two-step login <method> values.

tip

Getting prompted for additional authentication or getting a Your authentication request appears to be coming from a bot. error? Use your API Key client_secret to answer the authentication challenge. [Learn more](https://bitwarden.com/help/cli-auth-challenges/).

https://bitwarden.com/help/cli/#using-an-api-key

### Using an API key

Using the [personal API key](https://bitwarden.com/help/personal-api-key/) for CLI authentication is suitable for automated workflows, for providing access to an external application, or if your account uses a 2FA method not supported by the CLI (FIDO2 or Duo). The following command will prompt you for your personal client_id and client_secret:

```

Bash

bw login --apikey

```

While there are some commands that do not require your data be decrypted, to use many of the CLI commands you will need to subsequently decrypt your data using the unlock command ([learn more](https://bitwarden.com/help/cli/#unlock)), unless you're a member of an organization using [Key Connector](https://bitwarden.com/help/about-key-connector/). Your API key is not a substitute for your master password.

tip

If your organization [requires SSO](https://bitwarden.com/help/policies/#single-sign-on-authentication), you can still use --apikey to log in to the CLI.

#### Using API key environment variables

In scenarios where automated work is being done with the Bitwarden CLI, you can save environment variables to prevent the need for manual intervention at authentication.

|

Environment variable name

|

Required value

|

BW_CLIENTID

|

client_id

|

BW_CLIENTSECRET

|

client_secret

https://bitwarden.com/help/cli/#using-sso

### Using SSO

Logging in with [SSO](https://bitwarden.com/help/about-sso/) is recommended if an organization requires SSO authentication. To log in with SSO:

```

Bash

bw login --sso

```

This will initiate the [SSO authentication flow](https://bitwarden.com/help/using-sso/) in your web browser. Once your session is authenticated, you can use the unlock command. [Learn more](https://bitwarden.com/help/cli/#unlock).

tip

If your organization [requires SSO](https://bitwarden.com/help/policies/#single-sign-on-authentication), you may alternatively use --apikey to log in to the CLI.

https://bitwarden.com/help/cli/#log-in-to-multiple-accounts

### Log in to multiple accounts

Like using [account switching](https://bitwarden.com/help/account-switching/) on other Bitwarden apps, the CLI has the ability to log in to multiple accounts simultaneously using the BITWARDENCLI_APPDATA_DIR environment variable pointing to the location of a bw configuration file, usually named data.json. You can, for example, set aliases in a .bashrc profile for two separate configurations:

```

Bash

alias bw-personal="BITWARDENCLI_APPDATA_DIR=~/.config/Bitwarden\ CLI\ Personal /path/to/bw $@"
alias bw-work="BITWARDENCLI_APPDATA_DIR=~/.config/Bitwarden\ CLI\ Work /path/to/bw $@"

```

Using this example, you could then use login to two accounts by running first source /path/to/.bashrc, followed by bw-personal login and bw-work login.

https://bitwarden.com/help/cli/#unlock

## Unlock

Using an [API key](https://bitwarden.com/help/cli/#using-an-api-key) or [SSO](https://bitwarden.com/help/cli/#using-sso) to log in will require you to follow-up the login command with an explicit bw unlock if you'll be working with vault data directly.

Unlocking your vault generates a session key which acts as a decryption key used to interact with data in your vault. The [session key must be used](https://bitwarden.com/help/cli/#using-a-session-key) to perform any command that touches vault data (for example, list, get, edit). Session keys are valid until invalidated using bw lock or bw logout, however they will not persist if you open a new terminal window. Generate a new session key at any time using:

```

Bash

bw unlock

```

When you're finished, always end your session using the bw lock command.

https://bitwarden.com/help/cli/#unlock-options

### Unlock options

You can use the --passwordenv <passwordenv> or --passwordfile <passwordfile> options with bw unlock to retrieve your master password rather than enter it manually, for example:

-

The following will look for an environment variable BW_PASSWORD. If BW_PASSWORD is non-empty and has correct values, the CLI will successfully unlock and return a session key:

```

Bash

bw unlock --passwordenv BW_PASSWORD

```

-

The following will look for the file ~Users/Me/Documents/mp.txt (which must have your master password as the first line). If the file is non-empty and has a correct value, the CLI will successfully unlock and return a session key:

```

Bash

bw unlock --passwordfile ~/Users/Me/Documents/mp.txt

```

warning

If you use the --passwordfile option, protect your password file by locking access down to only the user who needs to run bw unlock and only providing read access to that user.

https://bitwarden.com/help/cli/#using-a-session-key

### Using a session key

When you unlock your vault using bw login with [email and password](https://bitwarden.com/help/cli/#using-email-and-password) or bw unlock, the CLI will return both an export BW_SESSION (Bash) and env:BW_SESSION (PowerShell) command, including your session key. Copy and paste the relevant entry to save the required environment variable.

With the BW_SESSION environment variable set, bw commands will reference that variable and can be run cleanly, for example:

```

Bash

export BW_SESSION="5PBYGU+5yt3RHcCjoeJKx/wByU34vokGRZjXpSH7Ylo8w=="

bw list items

```

Alternatively, if you don't set the environment variable, you can pass the session key as an option with each bw command:

```

Bash

bw list items --session "5PBYGU+5yt3RHcCjoeJKx/wByU34vokGRZjXpSH7Ylo8w=="

```

tip

When you're finished, always end your session using the bw lock or bw logout commands. This will invalidate the active session key.

https://bitwarden.com/help/cli/#core-commands

## Core Commands

https://bitwarden.com/help/cli/#create

### create

The create command creates a new object ( item, attachment, and more) in your vault:

```

Bash

bw create (item|attachment|folder|org-collection) <encodedJson> [options]

```

The create command takes encoded JSON. A typical workflow for creating an object might look something like:

-

Use the get template command (see [get core commands](https://bitwarden.com/help/cli/#get) for details) to output the appropriate JSON template for the object type.

-

Use a [command-line JSON processor like jq](https://stedolan.github.io/jq/) to manipulate the outputted template as required.

-

Use the encode command (see [details](https://bitwarden.com/help/cli/#encode)) to encode the manipulated JSON.

-

Use the create command to create an object from the encoded JSON.

For example:

```

Bash

bw get template folder | jq '.name="My First Folder"' | bw encode | bw create folder

```

or

```

Bash

bw get template item | jq ".name=\"My Login Item\" | .login=$(bw get template item.login | jq '.username="jdoe" | .password="myp@ssword123"')" | bw encode | bw create item

```

Upon successful creation, the newly created object will be returned as JSON.

#### create other item types

The create command defaults to creating a login item, but you can use a[command-line JSON processor like jq](https://stedolan.github.io/jq/) to change a .type= attribute to create other [item types](https://bitwarden.com/help/managing-items/):

|

Name

|

Value

|

Login

|

.type=1

|

Secure note

|

.type=2

|

Card

|

.type=3

|

Identity

|

.type=4

|

SSH Key

|

. type=5

For example, the following command will create a secure note:

```

Bash

bw get template item | jq '.type = 2 | .secureNote.type = 0 | .notes = "Contents of my Secure Note." | .name = "My Secure Note"' | bw encode | bw create item

```

note

Notice in the above example that Secure Notes require a sub-template ( .secureNote.type). You can view item type sub-templates using bw get template (see [here](https://bitwarden.com/help/cli/#get-template) for details).

#### create attachment

The create attachment command attaches a file to an existing item.

Unlike other create operations, you don’t need to use a JSON processor or encode to create an attachment. Instead, use the --file option to specify the file to attach and the --itemid option to specify the item to attach it to. For example:

```

Bash

bw create attachment --file ./path/to/file --itemid 16b15b89-65b3-4639-ad2a-95052a6d8f66

```

tip

If you don’t know the exact itemid you want to use, use bw get item <search-term> to return the item (see [details](https://bitwarden.com/help/cli/#get)), including its id.

https://bitwarden.com/help/cli/#get

### get

The get command retrieves a single object ( item, username, password, and more) from your vault:

```

Bash

bw get (item|username|password|uri|totp|exposed|attachment|folder|collection|organization|org-collection|template|fingerprint) <id> [options]

```

The get command takes an item id or string for its argument. If you use a string (for example, anything other than an exact id), get will search your vault objects for one with a value that matches. For example, the following command would return a Github password:

```

Bash

bw get password Github

```

note

The get command can only return one result, so you should use specific search terms. If multiple results are found, the CLI will return an error.

#### get attachment

The get attachment command downloads a file attachment:

```

Bash

bw get attachment <filename> --itemid <id>

```

The get attachment command takes a filename and exactid. By default, get attachment will download the attachment to the current working directory. You can use the --output option to specify a different output directory, for example:

```

Bash

bw get attachment photo.png --itemid 99ee88d2-6046-4ea7-92c2-acac464b1412 --output /Users/myaccount/Pictures/

```

note

When using --output, the path must end a forward slash ( /) to specify a directory or a filename ( /Users/myaccount/Pictures/photo.png).

#### get notes

The get notes command retrieves the note for any vault item:

```

Bash

bw get notes <id>

```

The get notes command takes an exact item id or string. If you use a string (for example, anything other than an exact id), get notes will search your vault objects for one with a value that matches. For example, the following command would return a Github note:

```

Bash

bw get notes Github

```

#### get template

The get template command returns the expected JSON formatting for an object ( item, item.field, item.login, and more):

```

Bash

bw get template (item|item.field|item.login|item.login.uri|item.card|item.identity|item.securenote|folder|collection|item-collections|org-collection)

```

While you can use get template to output the format to your screen, the most common use-case is to pipe the output into a bw create operation, using a [command-line JSON processor like jq](https://stedolan.github.io/jq/) and bw encode to manipulate the values retrieved from the template, for example:

```

Bash

bw get template folder | jq '.name="My First Folder"' | bw encode | bw create folder

```

note

Any item.xxx template should be used as a sub-object to an item template, for example:

```

Bash

bw get template item | jq ".name=\"My Login Item\" | .login=$(bw get template item.login | jq '.username="jdoe" | .password="myp@ssword123"')" | bw encode | bw create item

```

#### get fingerprint

Retrieve the fingerprint phrase of a user. You may specify userId directly, or use the shortcut me to get your own fingerprint phrase:

```

Plain Text

bw get fingerprint <userId>

```

```

Plain Text

bw get fingerprint me

```

https://bitwarden.com/help/cli/#edit

### edit

The edit command edits an object ( item, item-collections, etc.) in your vault:

```

Bash

bw edit (item|item-collections|folder|org-collection) <id> [encodedJson] [options]

```

The edit command takes an exactid (the object to edit) and encoded JSON (edits to be made). A typical workflow might look something like:

-

Use the get command (see [details](https://bitwarden.com/help/cli/#get)) to output the object to edit.

-

Use a [command-line JSON processor like jq](https://stedolan.github.io/jq/) to manipulate the outputted object as required.

-

Use the encode command (see [details](https://bitwarden.com/help/cli/#encode)) to encode the manipulated JSON.

-

Use the edit command (including the object id) to edit the object.

For example, to edit the password of a login item:

```

Bash

bw get item 7ac9cae8-5067-4faf-b6ab-acfd00e2c328 | jq '.login.password="newp@ssw0rd"' | bw encode | bw edit item 7ac9cae8-5067-4faf-b6ab-acfd00e2c328

```

Or, to edit the collection(s) an item is in:

```

Bash

echo '["5c926f4f-de9c-449b-8d5f-aec1011c48f6"]' | bw encode | bw edit item-collections 28399a57-73a0-45a3-80f8-aec1011c48f6 --organizationid 4016326f-98b6-42ff-b9fc-ac63014988f5

```

Or, to edit a collection:

```

Bash

bw get org-collection ee9f9dc2-ec29-4b7f-9afb-aac8010631a1 --organizationid 4016326f-98b6-42ff-b9fc-ac63014988f5 | jq '.name="My Collection"' | bw encode | bw edit org-collection ee9f9dc2-ec29-4b7f-9afb-aac8010631a1 --organizationid 4016326f-98b6-42ff-b9fc-ac63014988f5

```

The edit command will perform a replace operation on the object. Once completed, the updated object will be returned as JSON.

https://bitwarden.com/help/cli/#list

### list

The list command retrieves an array of objects ( items, folders, collections, and more) from your vault:

```

Bash

bw list (items|folders|collections|organizations|org-collections|org-members) [options]

```

You can add filters to the list command to narrow down what's returned:

-

--url <url>

-

--folderid <folderid>

-

--collectionid <collectionid>

-

--organizationid <organizationid>

-

--trash

Any filter will accept null or notnull. Combining multiple filters in one command will perform an OR operation, for example:

```

Bash

bw list items --folderid null --collectionid null

```

This command will return items that aren't in a folder or collection.

Additionally, you can search for specific objects using --search <search-term>. Combining filter and search in one command will perform an AND operation, like:

```

Bash

bw list items --search github --folderid 9742101e-68b8-4a07-b5b1-9578b5f88e6f

```

This command will search for items with the string github in the specified folder.

https://bitwarden.com/help/cli/#report

### report

The report command generates a report based on the contents of your vault:

```

Bash

bw report password-health [options]

```

The report command takes an object signifying the type of report to run as its argument and returns an array of objects as JSON. Currently, this command supports the password-health report. Reports will read decrypted vault data, so you'll need an active session key to use them.

#### password-health

The password-health argument evaluates the password of any login item that has one for weakness, reuse, and presence in a known data breach. [Non-login items](https://bitwarden.com/help/managing-items/#item-types), logins with no associated password, and [items in the trash](https://bitwarden.com/help/managing-items/#items-in-the-trash) are skipped. Each returned object in the JSON array will look like the following:

```

Bash

{
    "object": "password-health", #Always password-health
    "id": "0eb24d58-6qbe-438f-85c4-b38c10f4f256", #ID of the login item
    "name": "Netflix", #Name of the login item
    "passwordStrength": 0, #Strength score for this password, from 0 (weakest) to 4 (strongest)
    "reuseCount": 3, #Number of items in your vault that share this password, null if the password is unique
    "exposed": true, #Whether this password was found in a known data breach, true or false
    "exposedCount": 52372427, #Number of breaches this password was found in, null if the password was not found in a data breach
    "exposedError": null #null unless the breach check failed for this item
  },

```

note

The breach check makes one request to Have I Been Pwned per login item and can take some time on large vaults, so the CLI writes a progress indicator to stderr while it runs. This will leave stdout as clean JSON that you can redirect to a file:

```

Bash

bw report password-health > password-health.json

```

You can also use the --no-check-exposed option to skip breach checks entirely, in which case the password-health process will make no network requests.

https://bitwarden.com/help/cli/#delete

### delete

The delete command deletes an object from your vault. delete takes only an exactid for its argument.

```

Bash

bw delete (item|attachment|folder|org-collection) <id> [options]

```

By default, delete will send an item to the [Trash](https://bitwarden.com/help/managing-items/#items-in-the-trash), where it will remain for 30 days. You can permanently delete an item using the -p, --permanent option.

```

Bash

bw delete item 7063feab-4b10-472e-b64c-785e2b870b92 --permanent

```

To delete an org-collection, you'll also need to specify --organizationid <organizationid>. See [Organization IDs](https://bitwarden.com/help/cli/#organization-ids).

warning

While items that are deleted using delete can be recovered using the restore command for up to 30 days (see [details](https://bitwarden.com/help/cli/#restore)), items that are deleted using delete --permanentare completely removed and irrecoverable.

https://bitwarden.com/help/cli/#restore

### restore

The restore command restores a deleted object from your trash. restore takes only an exactid for its argument.

```

Bash

bw restore (item) <id> [options]

```

For example:

```

Bash

bw restore item 7063feab-4b10-472e-b64c-785e2b870b92

```

https://bitwarden.com/help/cli/#send

### send

The send command creates a [Bitwarden Send](https://bitwarden.com/help/about-send/) object for ephemeral sharing. This section will detail simple send operations, however send is a highly flexible tool and we recommend referring to the dedicated article on [Send from CLI](https://bitwarden.com/help/send-cli/).

To create a simple text Send:

```

Bash

bw send -n "My First Send" -d 7 --hidden "The contents of my first text Send."

```

To create a simple file Send:

```

Bash

bw send -n "A Sensitive File" -d 14 -f /Users/my_account/Documents/sensitive_file.pdf

```

https://bitwarden.com/help/cli/#receive

### receive

The receive command accesses a [Bitwarden Send](https://bitwarden.com/help/about-send/) object. To receive a Send object:

```

Bash

bw receive --password passwordforaccess https://vault.bitwarden.com/#/send/yawoill8rk6VM6zCATXv2A/9WN8wD-hzsDJjfnXLeNc2Q

```

https://bitwarden.com/help/cli/#organizations-commands

## Organizations commands

https://bitwarden.com/help/cli/#organization-ids

### Organization IDs

Accessing an organization from the CLI requires knowledge of an ID for your organization, as well as IDs for individual [members](https://bitwarden.com/help/managing-users/) and [collections](https://bitwarden.com/help/about-collections/).

Retrieve this information directly from the CLI using bw list, for example:

```

Bash

bw list organizations
bw list org-members --organizationid 4016326f-98b6-42ff-b9fc-ac63014988f5
bw list org-collections --organizationid 4016326f-98b6-42ff-b9fc-ac63014988f5

```

tip

You can bw list both collections and org-collections. The bw list collections command will list all collections, agnostic of which organization they belong to. bw list org-collections will list only collections that belong to the organization specified using --organizationid.

https://bitwarden.com/help/cli/#move

### move

note

August 2021: The share command has been changed to move. [Find out more](https://bitwarden.com/help/releasenotes/).

The move command transfers a vault item [to an organization](https://bitwarden.com/help/sharing/):

```

Bash

bw move <itemid> <organizationid> [encodedJson]

```

The move command requires you to encode a collection ID, and takes an exactid (the object to share) and an exactorganizationid (the organization to share the object to). For example:

```

Bash

echo '["bq209461-4129-4b8d-b760-acd401474va2"]' | bw encode | bw move ed42f44c-f81f-48de-a123-ad01013132ca dfghbc921-04eb-43a7-84b1-ac74013bqb2e

```

Once completed, the updated item will be returned.

https://bitwarden.com/help/cli/#confirm

### confirm

The confirm command confirms [invited members](https://bitwarden.com/help/managing-users/#confirm-invited-users) to your organization who have accepted their invitation:

warning

Before administering the confirm command, it is strongly advised that administrators validate the legitimacy of a request by ensuring that the fingerprint phrase self-reported by the user matches the fingerprint phrase associated with the user you expect to be confirmed:

-

From the Admin Console, you can view a user's associated fingerprint phrase [during the confirm step](https://bitwarden.com/help/managing-users/#confirm).

-

From the CLI, you can view a user's associated fingerprint phrase with the command bw get fingerprint <id>, where <id> is that member's user identifier. User identifiers can be retrieved [with the Public API](https://bitwarden.com/help/api/).

Once a user is confirmed, they have the ability to decrypt organization data, so ensuring users' self-reported fingerprint phrases match expected values is an important step prior to confirming.

```

Bash

bw confirm org-member <id> --organizationid <orgid>

```

The confirm command takes an exact member id and an exactorganizationID, for example:

```

Bash

bw confirm org-member 7063feab-4b10-472e-b64c-785e2b870b92 --organizationid 310d5ffd-e9a2-4451-af87-ea054dce0f78

```

https://bitwarden.com/help/cli/#device-approval

### Device approval

Allows admins and owners to manage device approval requests where a user has requested admin approval.

note

At this time, bulk device approval is only available for the Bitwarden CLI client downloaded from [Bitwarden.com](https://bitwarden.com/download/#command-line-interface).

warning

In most scenarios, users are able to approve their own login requests, and admin device approval is not necessary. See [Add a trusted device](https://bitwarden.com/help/add-a-trusted-device/). Automatic or bulk approval of admin device approval requests neglect verification steps that administrators can perform in order to ensure a request is legitimate, such as checking the user's reported Fingerprint Phrase.

Bitwarden recommends that significant security controls such as IdP credential standards, IdP MFA, and IdP device registration and trust be reviewed before enabling and using bulk device approval.

The list command will show all pending device approval requests for an organization:

```

Plain Text

bw device-approval list --organizationid <organization_Id>

```

The approve command is used to approve pending device authorization requests for an organization:

```

Plain Text

bw device-approval approve --organizationid <organizationId> <requestId>

```

Similarly, approve-all command can be used to approve all current pending requests:

```

Plain Text

bw device-approval approve-all --organization <organizationId>

```

To deny a pending authorization request:

```

Plain Text

bw device-approval deny --organizationid <organizationId> <requestId>

```

To deny-all pending authorization requests:

```

Plain Text

bw device-approval deny-all --organizationid <organizationId>

```

https://bitwarden.com/help/cli/#other-commands

## Other commands

https://bitwarden.com/help/cli/#config

### config

The config command specifies settings for the Bitwarden CLI to use:

```

Bash

bw config server <setting> [value]

```

A primary use of bw config is to [connect your CLI to a self-hosted](https://bitwarden.com/help/change-client-environment/#tab-cli-4dQ4hW1QAwVBuReXk2Txx0) Bitwarden server:

```

Bash

bw config server https://your.bw.domain.com

```

tip

Connect to the Bitwarden [EU server](https://bitwarden.com/help/server-geographies/) by running the following command:

```

Bash

bw config server https://vault.bitwarden.eu

```

Pass bw config server without a value to read the server you're connected to.

Users with unique setups may elect to specify the URL of each service independently. Note that any subsequent use of the config command will overwrite all previous specifications, so this must be run as a single command each time you make a change:

```

Bash

bw config server --web-vault <url> \
  --api <url> \
  --identity <url> \
  --icons <url> \
  --notifications <url> \
  --events <url> \
  --key-connector <url>

```

note

The bw config server --key-connector <url> command is required if your organization uses [Key Connector](https://bitwarden.com/help/about-key-connector/) and you’re using the --apikey option to login after having [removed your master password](https://bitwarden.com/help/using-sso/#login-using-sso).

Contact an organization owner to get the required URL.

https://bitwarden.com/help/cli/#sync

### sync

The sync command downloads your encrypted vault from the Bitwarden server. This command is most useful when you have changed something in your Bitwarden vault on another client application (for example web vault, browser extension, mobile app) since [logging in](https://bitwarden.com/help/cli/#log-in) on the CLI.

```

Bash

bw sync

```

You can pass the --last option to return only the timestamp ([ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)) of the last time a sync was performed.

tip

It’s important to know that synconly performs a pull from the server. Data is automatically pushed to the server any time you make a change to your vault (for example, create, edit, delete).

https://bitwarden.com/help/cli/#encode

### encode

The encode command Base 64 encodes stdin. This command is typically used in combination with a [command-line JSON processor like jq](https://stedolan.github.io/jq/) when performing create and edit operations, for example:

```

Bash

bw get template folder | jq '.name="My First Folder"' | bw encode | bw create folder

bw get item 7ac9cae8-5067-4faf-b6ab-acfd00e2c328 | jq '.login.password="newp@ssw0rd"' | bw encode | bw edit item 7ac9cae8-5067-4faf-b6ab-acfd00e2c328

```

https://bitwarden.com/help/cli/#import

### import

The import command imports data from a Bitwarden export or other [supported password management application](https://bitwarden.com/help/import-data/). The command must be pointed to a file and include the following arguments:

```

Bash

bw import <format> <path>

```

For example:

```

Bash

bw import lastpasscsv /Users/myaccount/Documents/mydata.csv

```

tip

Bitwarden supports lots of formats for import, too many to list here! Use bw import --formats to return the list in your CLI, or [see here](https://bitwarden.com/help/import-faqs/#q-what-file-formats-does-bitwarden-support-for-import).

If you are importing an [encrypted .json file that you've created with a password](https://bitwarden.com/help/encrypted-export/), you will be prompted to enter the password before import completes.

#### import to an organization vault

Using the import command with the --organizationid option, you can import data to an organization vault:

```

Plain Text

bw import --organizationid cf14adc3-aca5-4573-890a-f6fa231436d9 bitwardencsv ./from/source.csv

```

https://bitwarden.com/help/cli/#export

### export

The export command exports vault data as a .json or .csv, [encrypted .json](https://bitwarden.com/help/encrypted-export/), or as a .zip with [attachments](https://bitwarden.com/help/attachments/):

```

Bash

bw export [--output <filePath>] [--format <format>] [--password <password>] [--organizationid <orgid>]

```

By default, the export command will generate a .csv (equivalent to specifying --format csv) to the current working directory, however you can specify:

-

--format json to export a .json file

-

--format encrypted_json to export an [encrypted .json](https://bitwarden.com/help/encrypted-export/) file

-

--password <password> to specify a password to use to encrypt encrypted_json exports instead of your [account encryption key](https://bitwarden.com/help/account-encryption-key/)

-

--format zip to export a .zip that includes your [attachments](https://bitwarden.com/help/attachments/)

-

--output <path> to export to a specific location

-

--raw to return the export to stdout instead of to a file

#### export from an organization vault

Using the export command with the --organizationid option, you can export an organization vault:

```

Bash

bw export --organizationid 7063feab-4b10-472e-b64c-785e2b870b92 --format json --output /Users/myaccount/Downloads/

```

https://bitwarden.com/help/cli/#generate

### generate

The generate command generates a strong password or [passphrase](https://bitwarden.com/help/cli/#generate-a-passphrase):

```

Bash

bw generate [--lowercase --uppercase --number --special --length <length> --passphrase --separator <separator> --words <words>]

```

By default, the generate command will generate a 14-character password with uppercase characters, lowercase characters, and numbers. This is the equivalent of passing:

```

Bash

bw generate -uln --length 14

```

You can generate more complex passwords using the options available to the command, including:

-

--uppercase, -u (include uppercase)

-

--lowercase, -l (include lowercase)

-

--number, -n (include numbers)

-

--special, -s (include special characters)

-

--length <length> (length of the password, min of 5)

#### generate a passphrase

Using the generate command with the --passphrase option, you can generate a passphrase instead of a password:

```

Bash

bw generate --passphrase --words <words> --separator <separator>

```

By default, bw generate --passphrase will generate a three-word passphrase separated by a dash ( -). This is the equivalent of passing:

```

Bash

bw generate --passphrase --words 3 --separator -

```

You can generate a complex passphrase using the options available to the command, including:

-

--words <words> (number of words)

-

--separator <separator> (separator character)

-

--capitalize, -c (include to title-case the passphrase)

-

--includeNumber (Include a single numerical character in your passphrase)

https://bitwarden.com/help/cli/#update

### update

The update command checks whether your Bitwarden CLI is running the most recent version. updatedoesn't automatically update the CLI for you.

```

Bash

bw update

```

If a new version is detected, you'll need to download the new version of the CLI using the printed URL for the executable, or using the tools available for the package manager you used to [download the CLI](https://bitwarden.com/help/cli/#download-and-install) (for example, npm install -g @bitwarden/cli).

https://bitwarden.com/help/cli/#status

### status

The status command returns status information about the Bitwarden CLI, including [configured](https://bitwarden.com/help/cli/#config) server URL, timestamp for the last sync ([ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)), user email and ID, and the vault status.

```

Bash

bw status

```

Status will return information as a JSON object, for example:

```

Bash

{
  "serverUrl": "https://bitwarden.example.com",
  "lastSync": "2020-06-16T06:33:51.419Z",
  "userEmail": "user@example.com",
  "userId": "00000000-0000-0000-0000-000000000000",
  "status": "unlocked"
}

```

status may be one of the following:

-

"unlocked", indicating you are logged in and your vault is unlocked (a BW_SESSION key environment variable is saved with an [active session key](https://bitwarden.com/help/cli/#using-a-session-key))

-

"locked", indicating you are logged in but your vault is locked (noBW_SESSION key environment variable is saved with an [active session key](https://bitwarden.com/help/cli/#using-a-session-key))

-

"unauthenticated", indicating you aren't logged in

tip

When "status": "unauthenticated", lastSync, userEmail, and userID will always return null.

https://bitwarden.com/help/cli/#serve

### serve

The serve command starts a local express web server that can be used to take all actions accessible from the CLI in the form of RESTful API calls from an HTTP interface.

```

Bash

bw serve --port <port> --hostname <hostname>

```

By default, serve will start the web server at port 8087 however you can specify an alternate port with the --port option.

By default, serve will bind your API web server to localhost however you can specify an alternate hostname with the --hostname option. API requests can only be made from the bound hostname.

By default, serve will block any request with an Origin header. You can circumvent this protection using the --disable-origin-protection option, however this is not recommended.

warning

You can specify --hostname all for no hostname binding, however this will allow any machine on the network to make API requests.

[View the API spec](https://bitwarden.com/help/vault-management-api/)for help making calls with serve.

https://bitwarden.com/help/cli/#debug

### Debug

The debug environment variable can be added for additional troubleshooting information.

```

Plain Text

export BITWARDENCLI_DEBUG=true

```

https://bitwarden.com/help/cli/#appendices

## Appendices

https://bitwarden.com/help/cli/#global-options

### Global options

The following options are available globally:

|

Option

|

Description

|

--pretty

|

Format output. JSON is tabbed with two spaces.

|

--raw

|

Return raw output instead of a descriptive message.

|

--response

|

Return a JSON formatted version of response output.

|

--quiet

|

Don't return anything to stdout. You might use this option, for example, when piping a credential value to a file or application.

|

--nointeraction

|

Do not prompt for interactive user input.

|

--session <session>

|

Pass session key instead of reading from an environment variable.

|

-v, --version

|

Output the Bitwarden CLI version number.

|

-h, --help

|

Display help text for the command.

https://bitwarden.com/help/cli/#zsh-shell-completion

### ZSH shell completion

The Bitwarden CLI includes support for ZSH shell completion. To setup shell completion, use one of the following methods:

-

Vanilla ZSH: Add the following line to your .zshrc file:

```

Bash

eval "$(bw completion --shell zsh); compdef _bw bw;"

```

-

Vanilla (vendor-completions): Run the following command:

```

Bash

bw completion --shell zsh | sudo tee /usr/share/zsh/vendor-completions/_bw

```

-

[zinit:](https://github.com/zdharma/zinit) Run the following commands:

```

Bash

bw completion --shell zsh > ~/.local/share/zsh/completions/_bw
zinit creinstall ~/.local/share/zsh/completions

```

https://bitwarden.com/help/cli/#using-self-signed-certificates

### Using self-signed certificates

If your self-hosted Bitwarden server exposes a self-signed TLS certificate, specify the Node.js environment variable [NODE_EXTRA_CA_CERTS](https://nodejs.org/api/cli.html#cli_node_extra_ca_certs_file):

 Bash

```

Bash

export NODE_EXTRA_CA_CERTS="absolute/path/to/your/certificates.pem"

```

 PowerShell

```

Bash

$env:NODE_EXTRA_CA_CERTS="absolute/path/to/your/certificates.pem"

```

https://bitwarden.com/help/cli/#enums

### Enums

The following tables enumerate values required in documented scenarios:

#### Two-step login methods

Used to specify which [two-step login method](https://bitwarden.com/help/setup-two-step-login/) to use when [logging in](https://bitwarden.com/help/cli/#log-in):

|

Name

|

Value

|

Authenticator

|

0

|

Email

|

1

|

YubiKey

|

3

note

FIDO2 and Duo are not supported by the CLI.

#### Item types

Used with the create command to specify a [vault item type](https://bitwarden.com/help/managing-items/):

|

Name

|

Value

|

Login

|

1

|

Secure Note

|

2

|

Card

|

3

|

Identity

|

4

|

SSH key

|

5

#### Login URI match types

Used with the create and edit command to specify [URI match detection](https://bitwarden.com/help/uri-match-detection/) behavior for a login item:

|

Name

|

Value

|

Domain

|

0

|

Host

|

1

|

Starts With

|

2

|

Exact

|

3

|

Regular Expression

|

4

|

Never

|

5

#### Field types

Used with the create and edit commands to configure [custom fields](https://bitwarden.com/help/custom-fields/):

|

Name

|

Value

|

Text

|

0

|

Hidden

|

1

|

Boolean

|

2

#### Organization user types

Indicates a [user's type](https://bitwarden.com/help/user-types-access-control/):

|

Name

|

Value

|

Owner

|

0

|

Admin

|

1

|

User

|

2

|

Manager

|

3

|

Custom

|

4

#### Organization user statuses

Indicates a user's [status within the organization](https://bitwarden.com/help/managing-users/):

|

Name

|

Value

|

Invited

|

0

|

Accepted

|

1

|

Confirmed

|

2

|

Revoked

|

-1
