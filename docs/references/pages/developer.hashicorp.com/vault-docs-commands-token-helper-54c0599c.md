---
url: "https://developer.hashicorp.com/vault/docs/commands/token-helper"
title: "Use a custom token helper | Vault | HashiCorp Developer"
site: "Use a custom token helper | Vault | HashiCorp Developer"
captured: "2026-11-02T18:26:28Z"
via: "live"
sha256: "09c63aa937e0f52dc524996a272028fa1835e51fc23fa5af71c1d566f6dc213e"
wayback: "https://web.archive.org/web/20261012085332/https://developer.hashicorp.com/vault/docs/commands/token-helper"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# Use a custom token helper | Vault | HashiCorp Developer

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://developer.hashicorp.com/vault/docs/commands/token-helper>
> Snapshot: <https://web.archive.org/web/20261012085332/https://developer.hashicorp.com/vault/docs/commands/token-helper>

# Use a custom token helper

A token helper is a program or script that saves, retrieves, or erases a saved authentication token.

By default, the Vault CLI includes a token helper that caches tokens from any enabled authentication backend in a ~/.vault-token file. You can customize the caching behavior with a custom token helper.

## https://developer.hashicorp.com/vault/docs/commands/token-helper#step-1-script-your-helperStep 1: Script your helper

Your token helper must accept a single command-line argument:

| Argument | Action

| get | Fetch and print a cached authentication token to stdout

| store | Read an authentication token from stdin and save it in a secure location

| erase | Delete a cached authentication token

You can manage the authentication tokens in whatever way you prefer, but your helper must adhere to following output requirements:

- Limit stdout writes to token strings.
- Write all error messages to stderr.
- Write all non-error and non-token output to syslog or a log file.
- Return the status code 0 on success.
- Return non-zero status codes for errors.

## https://developer.hashicorp.com/vault/docs/commands/token-helper#step-2-configure-vaultStep 2: Configure Vault

To configure a custom token helper, edit (or create) a CLI configuration file called .vault under your home directory and set the token_helper parameter with the fully qualified path to your new helper:

```
echo 'token_helper = "/path/to/token/helper.sh"' >> ${HOME}/.vault

```

Tip

Make sure the script is executable by the Vault binary.

## https://developer.hashicorp.com/vault/docs/commands/token-helper#example-token-helperExample token helper

The following token helper manages tokens in a JSON file in the home directory called .vault_tokens.

The helper relies on the $VAULT_ADDR environment variable to store and retrieve tokens from different Vault servers.

```
#!/bin/bash

function write_error(){ >&2 echo $@; }

# Customize the hash key for tokens. Currently, we remove the strings
# 'https://', '.', and ':' from the passed address (Vault address environment
# by default) because jq has trouble with special characeters in JSON field
# names
function createHashKey {

  local key=""

  if [[ -z "${1}" ]] ; then key="${VAULT_ADDR}"
  else                      key="${1}"
  fi

  # We index the token according to the Vault server address by default so
  # return an error if the address is empty
  if [[ -z "${key}" ]] ; then
    write_error "Error: VAULT_ADDR environment variable unset."
    exit 100
  fi

  key=${key//"http://"/""}
  key=${key//"."/"_"}
  key=${key//":"/"_"}

  echo "addr-${key}"
}

TOKEN_FILE="${HOME}/.vault_token"
KEY=$(createHashKey)
TOKEN="null"

# If the token file does not exist, create it
if [ ! -f ${TOKEN_FILE} ] ; then
   echo "{}" > ${TOKEN_FILE}
fi

case "${1}" in
    "get")

      # Read the current JSON data and pull the token associated with ${KEY}
      TOKEN=$(cat ${TOKEN_FILE} | jq --arg key "${KEY}" -r '.[$key]')

      # If the token != to the string "null", print the token to stdout
      # jq returns "null" if the key was not found in the JSON data
      if [ ! "${TOKEN}" == "null" ] ; then
        echo "${TOKEN}"
      fi
      exit 0
    ;;

    "store")

      # Get the token from stdin
      read TOKEN

      # Read the current JSON data and add a new entry
      JSON=$(
        jq                      \
        --arg key "${KEY}"      \
        --arg token "${TOKEN}"  \
        '.[$key] = $token' ${TOKEN_FILE}
      )

    ;;

    "erase")
      # Read the current JSON data and remove the entry if it exists
      JSON=$(
        jq                      \
        --arg key "${KEY}"      \
        --arg token "${TOKEN}"  \
        'del(.[$key])' ${TOKEN_FILE}
      )

    ;;

    *)
      # change to stderr for real code
      write_error "Error: Provide a valid command: get, store, or erase."
      exit 101
esac

# Update the JSON file and return success
echo $JSON | jq "." > ${TOKEN_FILE}
exit 0

```

[Edit this page on GitHub](https://github.com/hashicorp/web-unified-docs/blob/main/content/vault/v2.x/content/docs/commands/token-helper.mdx)
