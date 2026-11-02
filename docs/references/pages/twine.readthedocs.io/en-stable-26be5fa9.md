---
url: "https://twine.readthedocs.io/en/stable/"
title: "twine 7.0.0 documentationContentsMenuExpandLight modeDark modeAuto light/dark, in light modeAuto light/dark, in dark mode"
captured: "2026-11-02T18:47:14Z"
via: "live"
sha256: "ed4d8facb7c53abe30156aafc4132b484c21ac81aa2a2fe323a8385098990058"
wayback: "https://web.archive.org/web/20260909173409/https://twine.readthedocs.io/en/stable/"
cited_in:
  - "docs/threats/06-credential-and-secret-protection.md"
---

# twine 7.0.0 documentationContentsMenuExpandLight modeDark modeAuto light/dark, in light modeAuto light/dark, in dark mode

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://twine.readthedocs.io/en/stable/>
> Snapshot: <https://web.archive.org/web/20260909173409/https://twine.readthedocs.io/en/stable/>

# Twine[¶](https://twine.readthedocs.io/en/stable/#twine)

Twine is a utility for [publishing](https://packaging.python.org/tutorials/packaging-projects/) Python packages to [PyPI](https://pypi.org) and other [repositories](https://packaging.python.org/glossary/#term-Package-Index). It provides build system independent uploads of source and binary [distribution artifacts](https://packaging.python.org/glossary/#term-Distribution-Package) for both new and existing [projects](https://packaging.python.org/glossary/#term-Project).

## Why Should I Use This?[¶](https://twine.readthedocs.io/en/stable/#why-should-i-use-this)

The goal of Twine is to improve PyPI interaction by improving security and testability.

The biggest reason to use Twine is that it securely authenticates you to PyPI over HTTPS using a verified connection, regardless of the underlying Python version. Meanwhile, python setup.py upload will only work correctly and securely if your build system, Python version, and underlying operating system are configured properly.

Secondly, Twine encourages you to build your distribution files. python setup.py upload only allows you to upload a package as a final step after building with distutils or setuptools, within the same command invocation. This means that you cannot test the exact file you’re going to upload to PyPI to ensure that it works before uploading it.

Finally, Twine allows you to pre-sign your files and pass the .asc files into the command line invocation (twine upload myproject-1.0.1.tar.gz myproject-1.0.1.tar.gz.asc). This enables you to be assured that you’re typing your gpg passphrase into gpg itself and not anything else, since you will be the one directly executing gpg --detach-sign -a <filename>.

## Features[¶](https://twine.readthedocs.io/en/stable/#features)

-

Verified HTTPS connections

-

Uploading doesn’t require executing setup.py

-

Uploading files that have already been created, allowing testing of distributions before release

-

Supports uploading any packaging format (including [wheels](https://packaging.python.org/glossary/#term-Wheel))

## Installation[¶](https://twine.readthedocs.io/en/stable/#installation)

```
pip install twine

```

## Using Twine[¶](https://twine.readthedocs.io/en/stable/#using-twine)

-

Create some distributions in the normal way:

```
python -m build

```

-

Upload to [Test PyPI](https://packaging.python.org/guides/using-testpypi/) and verify things look right:

```
twine upload -r testpypi dist/*

```

Twine will prompt for your username and password.

-

Upload to [PyPI](https://pypi.org):

```
twine upload dist/*

```

-

Done!

Note

Like many other command line tools, Twine does not show any characters when you enter your password.

If you’re using Windows and trying to paste your username, password, or token in the Command Prompt or PowerShell, Ctrl-V and Shift+Insert won’t work. Instead, you can use “Edit > Paste” from the window menu, or enable “Use Ctrl+Shift+C/V as Copy/Paste” in “Properties”. This is a [known issue](https://bugs.python.org/issue37426) with Python’s getpass module.

More documentation on using Twine to upload packages to PyPI is in the [Python Packaging User Guide](https://packaging.python.org/tutorials/packaging-projects/).

## Commands[¶](https://twine.readthedocs.io/en/stable/#commands)

### twine upload[¶](https://twine.readthedocs.io/en/stable/#twine-upload)

Uploads one or more distributions to a repository.

```
usage: twine upload [-h] [-r REPOSITORY] [--repository-url REPOSITORY_URL]
                    [--attestations] [-s] [--sign-with SIGN_WITH]
                    [-i IDENTITY] [-u USERNAME] [-p PASSWORD]
                    [--non-interactive] [-c COMMENT]
                    [--config-file CONFIG_FILE] [--skip-existing]
                    [--cert path] [--client-cert path] [--verbose]
                    [--disable-progress-bar]
                    dist [dist ...]

positional arguments:
  dist                  The distribution files to upload to the repository
                        (package index). Usually dist/* . May additionally
                        contain a .asc file to include an existing signature
                        with the file upload.

options:
  -h, --help            show this help message and exit
  -r REPOSITORY, --repository REPOSITORY
                        The repository (package index) to upload the package
                        to. Should be a section in the config file [default:
                        pypi]. (Can also be set via TWINE_REPOSITORY
                        environment variable.)
  --repository-url REPOSITORY_URL
                        The repository (package index) URL to upload the
                        package to. This overrides --repository. (Can also be
                        set via TWINE_REPOSITORY_URL environment variable.)
  --attestations        Upload each file's associated attestations.
  -s, --sign            Sign files to upload using GPG.
  --sign-with SIGN_WITH
                        GPG program used to sign uploads [default: gpg].
  -i IDENTITY, --identity IDENTITY
                        GPG identity used to sign files.
  -u USERNAME, --username USERNAME
                        The username to authenticate to the repository
                        (package index) as. Has no effect on PyPI or TestPyPI.
                        (Can also be set via TWINE_USERNAME environment
                        variable.)
  -p PASSWORD, --password PASSWORD
                        The password to authenticate to the repository
                        (package index) with. (Can also be set via
                        TWINE_PASSWORD environment variable.)
  --non-interactive     Do not interactively prompt for username/password if
                        the required credentials are missing. (Can also be set
                        via TWINE_NON_INTERACTIVE environment variable.)
  -c COMMENT, --comment COMMENT
                        The comment to include with the distribution file.
  --config-file CONFIG_FILE
                        The .pypirc config file to use.
  --skip-existing       Continue uploading files if one already exists. (Only
                        valid when uploading to PyPI. Other implementations
                        may not support this.)
  --cert path           Path to alternate CA bundle (can also be set via
                        TWINE_CERT environment variable).
  --client-cert path    Path to SSL client certificate, a single file
                        containing the private key and the certificate in PEM
                        format.
  --verbose             Show verbose output.
  --disable-progress-bar
                        Disable the progress bar.

```

### twine check[¶](https://twine.readthedocs.io/en/stable/#twine-check)

Checks whether your distribution’s long description will render correctly on PyPI.

```
usage: twine check [-h] [--strict] dist [dist ...]

positional arguments:
  dist        The distribution files to check, usually dist/*

options:
  -h, --help  show this help message and exit
  --strict    Fail on warnings

```

### twine register[¶](https://twine.readthedocs.io/en/stable/#twine-register)

Pre-register a name with a repository before uploading a distribution.

Warning

Pre-registration is [not supported on PyPI](https://packaging.python.org/guides/migrating-to-pypi-org/#registering-package-names-metadata), so the register command is only necessary if you are using a different repository that requires it. See [issue #1627 on Warehouse](https://github.com/pypi/warehouse/issues/1627) (the software running on PyPI) for more details.

```
usage: twine register [-h] [-r REPOSITORY] [--repository-url REPOSITORY_URL]
                      [--attestations] [-s] [--sign-with SIGN_WITH]
                      [-i IDENTITY] [-u USERNAME] [-p PASSWORD]
                      [--non-interactive] [-c COMMENT]
                      [--config-file CONFIG_FILE] [--skip-existing]
                      [--cert path] [--client-cert path] [--verbose]
                      [--disable-progress-bar]
                      package

register operation is not required with PyPI.org

positional arguments:
  package               File from which we read the package metadata.

options:
  -h, --help            show this help message and exit
  -r REPOSITORY, --repository REPOSITORY
                        The repository (package index) to upload the package
                        to. Should be a section in the config file [default:
                        pypi]. (Can also be set via TWINE_REPOSITORY
                        environment variable.)
  --repository-url REPOSITORY_URL
                        The repository (package index) URL to upload the
                        package to. This overrides --repository. (Can also be
                        set via TWINE_REPOSITORY_URL environment variable.)
  --attestations        Upload each file's associated attestations.
  -s, --sign            Sign files to upload using GPG.
  --sign-with SIGN_WITH
                        GPG program used to sign uploads [default: gpg].
  -i IDENTITY, --identity IDENTITY
                        GPG identity used to sign files.
  -u USERNAME, --username USERNAME
                        The username to authenticate to the repository
                        (package index) as. Has no effect on PyPI or TestPyPI.
                        (Can also be set via TWINE_USERNAME environment
                        variable.)
  -p PASSWORD, --password PASSWORD
                        The password to authenticate to the repository
                        (package index) with. (Can also be set via
                        TWINE_PASSWORD environment variable.)
  --non-interactive     Do not interactively prompt for username/password if
                        the required credentials are missing. (Can also be set
                        via TWINE_NON_INTERACTIVE environment variable.)
  -c COMMENT, --comment COMMENT
                        The comment to include with the distribution file.
  --config-file CONFIG_FILE
                        The .pypirc config file to use.
  --skip-existing       Continue uploading files if one already exists. (Only
                        valid when uploading to PyPI. Other implementations
                        may not support this.)
  --cert path           Path to alternate CA bundle (can also be set via
                        TWINE_CERT environment variable).
  --client-cert path    Path to SSL client certificate, a single file
                        containing the private key and the certificate in PEM
                        format.
  --verbose             Show verbose output.
  --disable-progress-bar
                        Disable the progress bar.

```

## Configuration[¶](https://twine.readthedocs.io/en/stable/#configuration)

Twine can read repository configuration from a .pypirc file, either in your home directory, or provided with the --config-file option. For details on writing and using .pypirc, see the [specification](https://packaging.python.org/specifications/pypirc/) in the Python Packaging User Guide.

### Environment Variables[¶](https://twine.readthedocs.io/en/stable/#environment-variables)

Twine also supports configuration via environment variables. Options passed on the command line will take precedence over options set via environment variables. Definition via environment variable is helpful in environments where it is not convenient to create a .pypirc file (for example, on a CI/build server).

-

TWINE_USERNAME - the username to use for authentication to the repository.

-

TWINE_PASSWORD - the password to use for authentication to the repository.

-

TWINE_REPOSITORY - the repository configuration, either defined as a section in .pypirc or provided as a full URL.

-

TWINE_REPOSITORY_URL - the repository URL to use.

-

TWINE_CERT - custom CA certificate to use for repositories with self-signed or untrusted certificates.

-

TWINE_NON_INTERACTIVE - Do not interactively prompt for username/password if the required credentials are missing.

### Proxy Support[¶](https://twine.readthedocs.io/en/stable/#proxy-support)

Twine can be configured to use a proxy by setting environment variables. For example, to use a proxy for just the twine command, without export-ing it for other tools:

```
HTTPS_PROXY=socks5://user:pass@host:port twine upload dist/*

```

For more information, see the Requests documentation on [Proxies](https://requests.readthedocs.io/en/latest/user/advanced/#proxies) and [SOCKS](https://requests.readthedocs.io/en/latest/user/advanced/#socks), and [an in-depth article about proxy environment variables](https://about.gitlab.com/blog/2021/01/27/we-need-to-talk-no-proxy/).

## Keyring Support[¶](https://twine.readthedocs.io/en/stable/#keyring-support)

Instead of typing in your password every time you upload a distribution, Twine allows storing a username and password securely using [keyring](https://pypi.org/project/keyring/). Keyring is installed with Twine but for some systems (Linux mainly) may require [additional installation steps](https://pypi.org/project/keyring/#installation-linux).

Once Twine is installed, use the keyring program to set a username and password to use for each repository to which you may upload.

For example, to set an API token for PyPI:

```
keyring set https://upload.pypi.org/legacy/ __token__

```

and paste your API key when prompted.

For a different repository, replace the URL with the relevant repository URL. For example, for Test PyPI, use https://test.pypi.org/legacy/.

Note

If you are using Linux in a headless environment (such as on a server) you’ll need to do some additional steps to ensure that Keyring can store secrets securely. See [Using Keyring on headless systems](https://keyring.readthedocs.io/en/latest/#using-keyring-on-headless-linux-systems).

### Disabling Keyring[¶](https://twine.readthedocs.io/en/stable/#disabling-keyring)

In most cases, simply not setting a password with keyring will allow Twine to fall back to prompting for a password. In some cases, the presence of Keyring will cause unexpected or undesirable prompts from the backing system. In these cases, it may be desirable to disable Keyring altogether. To disable Keyring, run:

```
keyring --disable

```

See [Twine issue #338](https://github.com/pypa/twine/issues/338) for discussion and background.
