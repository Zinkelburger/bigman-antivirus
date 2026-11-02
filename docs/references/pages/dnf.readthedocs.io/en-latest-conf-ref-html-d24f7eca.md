---
url: "https://dnf.readthedocs.io/en/latest/conf_ref.html"
title: "DNF Configuration Reference — DNF @DNF_VERSION@-1 documentation"
captured: "2026-11-02T18:27:35Z"
via: "live"
sha256: "922eb5164fcf9df5a646e2ad1aec00decb430500826dea63e767bd277e65e860"
wayback: "https://web.archive.org/web/20261025224714/https://dnf.readthedocs.io/en/latest/conf_ref.html"
cited_in:
  - "docs/threats/03-distro-packaging-and-install-channels.md"
---

# DNF Configuration Reference — DNF @DNF_VERSION@-1 documentation

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://dnf.readthedocs.io/en/latest/conf_ref.html>
> Snapshot: <https://web.archive.org/web/20261025224714/https://dnf.readthedocs.io/en/latest/conf_ref.html>

- https://dnf.readthedocs.io/en/latest/index.html
- DNF Configuration Reference
-  [Edit on GitHub](https://github.com/rpm-software-management/dnf/blob/master/doc/conf_ref.rst)

# DNF Configuration Reference[](https://dnf.readthedocs.io/en/latest/conf_ref.html#dnf-configuration-reference)

## Description[](https://dnf.readthedocs.io/en/latest/conf_ref.html#description)

[DNF](https://github.com/rpm-software-management/dnf/) by default uses the global configuration file at /etc/dnf/dnf.conf and all *.repo files found under /etc/yum.repos.d. The latter is typically used for repository configuration and takes precedence over global configuration.

The configuration file has INI format consisting of section declaration and name=value options below each on separate line. There are two types of sections in the configuration files: main and repository. Main section defines all global configuration options and should be only one.

The repository sections define the configuration for each (remote or local) repository. The section name of the repository in brackets serve as repo ID reference and should be unique across configuration files. The allowed characters of repo ID string are lower and upper case alphabetic letters, digits, -, _, . and :. The minimal repository configuration file should aside from repo ID consists of [baseurl](https://dnf.readthedocs.io/en/latest/conf_ref.html#baseurl-label), [metalink](https://dnf.readthedocs.io/en/latest/conf_ref.html#metalink-label) or [mirrorlist](https://dnf.readthedocs.io/en/latest/conf_ref.html#mirrorlist-label) option definition.

## Distribution-Specific Configuration[](https://dnf.readthedocs.io/en/latest/conf_ref.html#distribution-specific-configuration)

Configuration options, namely [best](https://dnf.readthedocs.io/en/latest/conf_ref.html#best-label) and [skip_if_unavailable](https://dnf.readthedocs.io/en/latest/conf_ref.html#skip-if-unavailable-label), can be set in the DNF configuration file by your distribution to override the DNF defaults.

## [main] Options[](https://dnf.readthedocs.io/en/latest/conf_ref.html#main-options)

allow_vendor_change

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

If disabled dnf will stick to vendor when upgrading or downgrading rpms. Default is True

Warning

This option is currently not supported for downgrade and distro-sync commands

arch

[string](https://dnf.readthedocs.io/en/latest/conf_ref.html#string-label)

The architecture used for installing packages. By default this is auto-detected. Often used together with [ignorearch](https://dnf.readthedocs.io/en/latest/conf_ref.html#ignorearch-label) option.

assumeno

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

If enabled dnf will assume No where it would normally prompt for confirmation from user input. Default is False.

assumeyes

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

If enabled dnf will assume Yes where it would normally prompt for confirmation from user input (see also [defaultyes](https://dnf.readthedocs.io/en/latest/conf_ref.html#defaultyes-label)). Default is False.

autocheck_running_kernel

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

Automatic check whether there is installed newer kernel module with security update than currently running kernel. Default is True.

basearch

[string](https://dnf.readthedocs.io/en/latest/conf_ref.html#string-label)

The base architecture used for installing packages. By default this is auto-detected.

best

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

True instructs the solver to either use a package with the highest available version or fail. On False, do not fail if the latest version cannot be installed and go with the lower version. The default is False. Note this option in particular [can be set in your configuration file by your distribution](https://dnf.readthedocs.io/en/latest/conf_ref.html#conf-distribution-specific-label). Also note that the use of the highest available version is only guaranteed for the packages directly requested and not for their dependencies.

cachedir

[string](https://dnf.readthedocs.io/en/latest/conf_ref.html#string-label)

Path to a directory used by various DNF subsystems for storing cache data. Has a reasonable root-writable default depending on the distribution. DNF needs to be able to create files and directories at this location.

cacheonly

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

If set to True DNF will run entirely from system cache, will not update the cache and will use it even in case it is expired. Default is False.

API Notes: Must be set before repository objects are created. Plugins must set this in the pre_config hook. Later changes are ignored.

check_config_file_age

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

Specifies whether dnf should automatically expire metadata of repos, which are older than their corresponding configuration file (usually the dnf.conf file and the foo.repo file). Default is True (perform the check). Expire of metadata is also affected by metadata age. See also [metadata_expire](https://dnf.readthedocs.io/en/latest/conf_ref.html#metadata-expire-label).

clean_requirements_on_remove

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

Remove dependencies that are no longer used during dnf remove. A package only qualifies for removal via clean_requirements_on_remove if it was installed through DNF but not on explicit user request, i.e. it was pulled in as a dependency. The default is True. ([installonlypkgs](https://dnf.readthedocs.io/en/latest/conf_ref.html#installonlypkgs-label) are never automatically removed.)

config_file_path

[string](https://dnf.readthedocs.io/en/latest/conf_ref.html#string-label)

Path to the default main configuration file. Default is /etc/dnf/dnf.conf.

debuglevel

[integer](https://dnf.readthedocs.io/en/latest/conf_ref.html#integer-label)

Debug messages output level, in the range 0 to 10. The higher the number the more debug output is put to stdout. Default is 2.

debug_solver

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

Controls whether the libsolv debug files should be created when solving the transaction. The debug files are created in the ./debugdata directory. Default is False.

defaultyes

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

If enabled the default answer to user confirmation prompts will be Yes. Not to be confused with [assumeyes](https://dnf.readthedocs.io/en/latest/conf_ref.html#assumeyes-label) which will not prompt at all. Default is False.

diskspacecheck

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

Controls whether rpm should check available disk space during the transaction. Default is True.

errorlevel

[integer](https://dnf.readthedocs.io/en/latest/conf_ref.html#integer-label)

Error messages output level, in the range 0 to 10. The higher the number the more error output is put to stderr. Default is 3. This is deprecated in DNF and overwritten by -[-verbose](https://dnf.readthedocs.io/en/latest/command_ref.html#verbose-options-label) commandline option.

exclude_from_weak

[list](https://dnf.readthedocs.io/en/latest/conf_ref.html#list-label)

Prevent installing packages as weak dependencies (recommends or supplements). The packages can be specified by a name or a glob and separated by a comma. Defaults to [].

exclude_from_weak_autodetect

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

If enabled, dnf will autodetect unmet weak dependencies (recommends or supplements) of packages installed on the system. Providers of these weak dependencies will not be installed by dnf as weak dependencies any more (they will still be installed if pulled in as a regular dependency). Defaults to true.

exit_on_lock

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

Should the dnf client exit immediately when something else has the lock. Default is False.

gpgkey_dns_verification

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

Should the dnf attempt to automatically verify GPG verification keys using the DNS system. This option requires the unbound python module (python3-unbound) to be installed on the client system. This system has two main features. The first one is to check if any of the already installed keys have been revoked. Automatic removal of the key is not yet available, so it is up to the user, to remove revoked keys from the system. The second feature is automatic verification of new keys when a repository is added to the system. In interactive mode, the result is written to the output as a suggestion to the user. In non-interactive mode (i.e. when -y is used), this system will automatically accept keys that are available in the DNS and are correctly signed using DNSSEC. It will also accept keys that do not exist in the DNS system and their NON-existence is cryptographically proven using DNSSEC. This is mainly to preserve backward compatibility. Default is False.

group_package_types

[list](https://dnf.readthedocs.io/en/latest/conf_ref.html#list-label)

List of the following: optional, default, mandatory. Tells dnf which type of packages in groups will be installed when ‘groupinstall’ is called. Default is: default, mandatory.

ignorearch

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

If set to True, RPM will allow attempts to install packages incompatible with the CPU’s architecture. Defaults to False. Often used together with [arch](https://dnf.readthedocs.io/en/latest/conf_ref.html#arch-label) option.

installonlypkgs

[list](https://dnf.readthedocs.io/en/latest/conf_ref.html#list-label)

List of provide names of packages that should only ever be installed, never upgraded. Kernels in particular fall into this category. These packages are never removed by dnf autoremove even if they were installed as dependencies (see [clean_requirements_on_remove](https://dnf.readthedocs.io/en/latest/conf_ref.html#clean-requirements-on-remove-label) for auto removal details). This option append the list values to the default installonlypkgs list used by DNF. The number of kept package versions is regulated by [installonly_limit](https://dnf.readthedocs.io/en/latest/conf_ref.html#installonly-limit-label).

installonly_limit

[integer](https://dnf.readthedocs.io/en/latest/conf_ref.html#integer-label)

Number of [installonly packages](https://dnf.readthedocs.io/en/latest/conf_ref.html#installonlypkgs-label) allowed to be installed concurrently. Defaults to 3. The minimal number of installonly packages is 2. Value 0 means unlimited number of installonly packages. Value 1 is explicitly not allowed since it complicates kernel upgrades due to protection of the running kernel from removal.

installroot

[string](https://dnf.readthedocs.io/en/latest/conf_ref.html#string-label)

The root of the filesystem for all packaging operations. It requires an absolute path. See also [–installroot commandline option](https://dnf.readthedocs.io/en/latest/conf_ref.html#installroot-label).

install_weak_deps

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

When this option is set to True and a new package is about to be installed, all packages linked by weak dependency relation (Recommends or Supplements flags) with this package will be pulled into the transaction. Default is True.

keepcache

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

Keeps downloaded packages in the cache when set to True. Even if it is set to False and packages have not been installed they will still persist until next successful transaction. The default is False.

logdir

[string](https://dnf.readthedocs.io/en/latest/conf_ref.html#string-label)

Directory where the log files will be stored. Default is /var/log.

logfilelevel

[integer](https://dnf.readthedocs.io/en/latest/conf_ref.html#integer-label)

Log file messages output level, in the range 0 to 10. The higher the number the more debug output is put to logs. Default is 9.

This option controls dnf.log, dnf.librepo.log and hawkey.log. Although dnf.librepo.log and hawkey.log are affected only by setting the logfilelevel to 10.

log_compress

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

When set to True, log files are compressed when they are rotated. Default is False.

log_rotate

[integer](https://dnf.readthedocs.io/en/latest/conf_ref.html#integer-label)

Log files are rotated log_rotate times before being removed. If log_rotate is 0, the rotation is not performed. Default is 4.

log_size

storage size

Log files are rotated when they grow bigger than log_size bytes. If log_size is 0, the rotation is not performed. The default is 1 MB. Valid units are ‘k’, ‘M’, ‘G’.

The size applies for individual log files, not the sum of all log files. See also [log_rotate](https://dnf.readthedocs.io/en/latest/conf_ref.html#log-rotate-label).

metadata_timer_sync

time in seconds

The minimal period between two consecutive makecache timer runs. The command will stop immediately if it’s less than this time period since its last run. Does not affect simple makecache run. Use 0 to completely disable automatic metadata synchronizing. The default corresponds to three hours. The value is rounded to the next commenced hour.

module_obsoletes

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

This option controls whether dnf should apply modular obsoletes when possible. Default is False.

module_platform_id

[string](https://dnf.readthedocs.io/en/latest/conf_ref.html#string-label)

Set this to $name:$stream to override PLATFORM_ID detected from /etc/os-release. It is necessary to perform a system upgrade and switch to a new platform.

module_stream_switch

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

This option controls whether it’s possible to switch enabled streams of a module. Default is False.

multilib_policy

[string](https://dnf.readthedocs.io/en/latest/conf_ref.html#string-label)

Controls how multilib packages are treated during install operations. Can either be "best" (the default) for the depsolver to prefer packages which best match the system’s architecture, or "all" to install packages for all available architectures.

obsoletes

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

This option only has affect during an install/update. It enables dnf’s obsoletes processing logic, which means it makes dnf check whether any dependencies of given package are no longer required and removes them. Useful when doing distribution level upgrades. Default is ‘true’.

Command-line option: [–obsoletes](https://dnf.readthedocs.io/en/latest/command_ref.html#obsoletes-option-label)

optional_metadata_types

[list](https://dnf.readthedocs.io/en/latest/conf_ref.html#list-label)

List of metadata types to be loaded in addition to primary, modules, comps, updateinfo and presto, which are loaded always.

Note that the list can be extended by individual commands to explicitly request loading specific metadata type.

Currently only filelists value is supported. Default is an empty list.

persistdir

[string](https://dnf.readthedocs.io/en/latest/conf_ref.html#string-label)

Directory where DNF stores its persistent data between runs. Default is "/var/lib/dnf".

persistence

[string](https://dnf.readthedocs.io/en/latest/conf_ref.html#string-label)

Whether changes should persist across system reboots. Default is auto. Passing [–transient](https://dnf.readthedocs.io/en/latest/command_ref.html#transient-option-label) will override this setting to transient. Valid values are:

-

auto: Changes will persist across reboots, unless the target is a running bootc system and the system is already in an unlocked state (i.e. /usr is writable).

-

transient: Changes will be lost on the next reboot. Only applicable on bootc systems. Beware that changes to /etc and /var will persist, depending on the configuration of your bootc system. See also https://bootc-dev.github.io/bootc/man/bootc-usr-overlay.html.

-

persist: Changes will persist across reboots.

pluginconfpath

[list](https://dnf.readthedocs.io/en/latest/conf_ref.html#list-label)

List of directories that are searched for plugin configurations to load. All configuration files found in these directories, that are named same as a plugin, are parsed. The default path is /etc/dnf/plugins.

pluginpath

[list](https://dnf.readthedocs.io/en/latest/conf_ref.html#list-label)

List of directories that are searched for plugins to load. Plugins found in any of the directories in this configuration option are used. The default contains a Python version-specific path.

plugins

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

Controls whether the plugins are enabled. Default is True.

protected_packages

[list](https://dnf.readthedocs.io/en/latest/conf_ref.html#list-label)

List of packages that DNF should never completely remove. They are protected via Obsoletes as well as user/plugin removals.

The default is: dnf, glob:/etc/yum/protected.d/*.conf and glob:/etc/dnf/protected.d/*.conf. So any packages which should be protected can do so by including a file in /etc/dnf/protected.d with their package name in it.

DNF will protect also the package corresponding to the running version of the kernel. See also [protect_running_kernel](https://dnf.readthedocs.io/en/latest/conf_ref.html#protect-running-kernel-label) option.

protect_running_kernel

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

Controls whether the package corresponding to the running version of kernel is protected from removal. Default is True.

releasever

[string](https://dnf.readthedocs.io/en/latest/conf_ref.html#string-label)

Used for substitution of $releasever in the repository configuration.

The $releasever_major and $releasever_minor variables will be automatically derived from $releasever by splitting it on the first .. For example, if $releasever is set to 1.23, then $releasever_major will be 1 and $releasever_minor will be 23.

$releasever_major and $releasever_minor can also be set by the distribution.

See also [repo variables](https://dnf.readthedocs.io/en/latest/conf_ref.html#repo-variables-label).

reposdir

[list](https://dnf.readthedocs.io/en/latest/conf_ref.html#list-label)

DNF searches for repository configuration files in the paths specified by reposdir. The behavior of reposdir could differ when it is used along with -[-installroot](https://dnf.readthedocs.io/en/latest/conf_ref.html#installroot-label) option.

rpmverbosity

[string](https://dnf.readthedocs.io/en/latest/conf_ref.html#string-label)

RPM debug scriptlet output level. One of: critical, emergency, error, warn, info or debug. Default is info.

strict

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

If disabled, all unavailable packages or packages with broken dependencies given to DNF command will be skipped without raising the error causing the whole operation to fail. Currently works for install command only. The default is True.

tsflags

[list](https://dnf.readthedocs.io/en/latest/conf_ref.html#list-label)

List of strings adding extra flags for the RPM transaction.

|

tsflag value

|

RPM Transaction Flag

|

noscripts

|

RPMTRANS_FLAG_NOSCRIPTS

|

test

|

RPMTRANS_FLAG_TEST

|

notriggers

|

RPMTRANS_FLAG_NOTRIGGERS

|

nodocs

|

RPMTRANS_FLAG_NODOCS

|

justdb

|

RPMTRANS_FLAG_JUSTDB

|

nocontexts

|

RPMTRANS_FLAG_NOCONTEXTS

|

nocaps

|

RPMTRANS_FLAG_NOCAPS

|

nocrypto

|

RPMTRANS_FLAG_NOFILEDIGEST

|

deploops

|

RPMTRANS_FLAG_DEPLOOPS

|

noplugins

|

RPMTRANS_FLAG_NOPLUGINS

The nocrypto option will also set the _RPMVSF_NOSIGNATURES and _RPMVSF_NODIGESTS VS flags. The test option provides a transaction check without performing the transaction. It includes downloading of packages, gpg keys check (including permanent import of additional keys if necessary), and rpm check to prevent file conflicts. The nocaps is supported with rpm-4.14 or later. When nocaps is used but rpm doesn’t support it, DNF only reports it as an invalid tsflag. The noplugins option is supported with rpm-4.12 or later to disable use of rpm plugins. Similarly, this will be reported as an invalid tsflag if rpm doesn’t support it.

upgrade_group_objects_upgrade

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

Set this to False to disable the automatic running of group upgrade when running the upgrade command. Default is True (perform the operation).

usr_drift_protected_paths

[list](https://dnf.readthedocs.io/en/latest/conf_ref.html#list-label)

List of paths that are likely to cause problems when their contents drift with respect to /usr, e.g. /etc/pam.d/*. If a transient transaction would modify these paths, DNF aborts the operation and prints an error. Supports globs. Defaults to glob:/etc/dnf/usr-drift-protected-paths.d/*.conf. So a list of paths can be protected by creating a .conf file in /etc/dnf/usr-drift-protected-paths.d/ containing one path (or glob pattern) per line.

When using persistence=transient on bootc systems, a transient overlay is created on /usr, and any changes DNF makes to /usr will be discarded on reboot. However, other paths such as /etc and /var are (often) not backed by a transient overlay, so changes to them will persist across reboots. Usually, this “filesystem drift” is fine, but it can cause problems in certain situations. For example, a configuration file in /etc that’s shared by multiple packages might reference a .so file under /usr/lib64 that no longer exists.

varsdir

[list](https://dnf.readthedocs.io/en/latest/conf_ref.html#list-label)

List of directories where variables definition files are looked for. Defaults to "/etc/dnf/vars", "/etc/yum/vars". See [variable files](https://dnf.readthedocs.io/en/latest/conf_ref.html#varfiles-label) in Configuration reference.

zchunk

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

Enables or disables the use of repository metadata compressed using the zchunk format (if available). Default is True.

## [main] Options - Colors[](https://dnf.readthedocs.io/en/latest/conf_ref.html#main-options-colors)

color

[string](https://dnf.readthedocs.io/en/latest/conf_ref.html#string-label)

Controls if DNF uses colored output on the command line. Possible values: “auto”, “never”, “always”. Default is “auto”.

color_list_available_downgrade

[color](https://dnf.readthedocs.io/en/latest/conf_ref.html#color-label)

Color of available packages that are older than installed packages. The option is used during list operations. Default is magenta.

color_list_available_install

[color](https://dnf.readthedocs.io/en/latest/conf_ref.html#color-label)

Color of packages that are available for installation and none of their versions in installed. The option is used during list operations. Default is bold,cyan.

color_list_available_reinstall

[color](https://dnf.readthedocs.io/en/latest/conf_ref.html#color-label)

Color of available packages that are identical to installed versions and are available for reinstalls. Default is bold,underline,green. The option is used during list operations.

color_list_available_upgrade

[color](https://dnf.readthedocs.io/en/latest/conf_ref.html#color-label)

Color of available packages that are newer than installed packages. Default is bold,blue. The option is used during list operations.

color_list_installed_extra

[color](https://dnf.readthedocs.io/en/latest/conf_ref.html#color-label)

Color of installed packages that do not have any version among available packages. The option is used during list operations. Default is bold,red.

color_list_installed_newer

[color](https://dnf.readthedocs.io/en/latest/conf_ref.html#color-label)

Color of installed packages that are newer than any version among available packages. The option is used during list operations. Default is bold,yellow.

color_list_installed_older

[color](https://dnf.readthedocs.io/en/latest/conf_ref.html#color-label)

Color of installed packages that are older than any version among available packages. The option is used during list operations. Default is yellow.

color_list_installed_reinstall

[color](https://dnf.readthedocs.io/en/latest/conf_ref.html#color-label)

Color of installed packages that are among available packages and can be reinstalled. The option is used during list operations. Default is cyan.

color_search_match

[color](https://dnf.readthedocs.io/en/latest/conf_ref.html#color-label)

Color of patterns matched in search output. Default is bold,magenta.

color_update_installed

[color](https://dnf.readthedocs.io/en/latest/conf_ref.html#color-label)

Color of removed packages. Default is red. This option is used during displaying transactions.

color_update_local

[color](https://dnf.readthedocs.io/en/latest/conf_ref.html#color-label)

Color of local packages that are installed from the @commandline repository. This option is used during displaying transactions. Default is green.

color_update_remote

[color](https://dnf.readthedocs.io/en/latest/conf_ref.html#color-label)

Color of packages that are installed/upgraded/downgraded from remote repositories. This option is used during displaying transactions. Default is bold,green.

## Repo Options[](https://dnf.readthedocs.io/en/latest/conf_ref.html#repo-options)

baseurl

[list](https://dnf.readthedocs.io/en/latest/conf_ref.html#list-label)

List of URLs for the repository. Defaults to [].

URLs are tried in the listed order (equivalent to yum’s “failovermethod=priority” behaviour).

cost

[integer](https://dnf.readthedocs.io/en/latest/conf_ref.html#integer-label)

The relative cost of accessing this repository, defaulting to 1000. This value is compared when the priorities of two repositories are the same. The repository with the lowest cost is picked. It is useful to make the library prefer on-disk repositories to remote ones.

enabled

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

Include this repository as a package source. The default is True.

gpgkey

[list](https://dnf.readthedocs.io/en/latest/conf_ref.html#list-label) of strings

URLs of a GPG key files that can be used for signing metadata and packages of this repository, empty by default. If a file can not be verified using the already imported keys, import of keys from this option is attempted and the keys are then used for verification.

metalink

[string](https://dnf.readthedocs.io/en/latest/conf_ref.html#string-label)

URL of a metalink for the repository. Defaults to None.

mirrorlist

[string](https://dnf.readthedocs.io/en/latest/conf_ref.html#string-label)

URL of a mirrorlist for the repository. Defaults to None.

module_hotfixes

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

Set this to True to disable module RPM filtering and make all RPMs from the repository available. The default is False. This allows user to create a repository with cherry-picked hotfixes that are included in a package set on a modular system.

name

[string](https://dnf.readthedocs.io/en/latest/conf_ref.html#string-label)

A human-readable name of the repository. Defaults to the ID of the repository.

priority

[integer](https://dnf.readthedocs.io/en/latest/conf_ref.html#integer-label)

The priority value of this repository, default is 99. If there is more than one candidate package for a particular operation, the one from a repo with the lowest priority value is picked, possibly despite being less convenient otherwise (e.g. by being a lower version).

type

[string](https://dnf.readthedocs.io/en/latest/conf_ref.html#string-label)

Type of repository metadata. Supported values are: rpm-md. Aliases for rpm-md: rpm, repomd, rpmmd, yum, YUM.

## Source and debuginfo repository names[](https://dnf.readthedocs.io/en/latest/conf_ref.html#source-and-debuginfo-repository-names)

For a given repository with an identifier in the form “<ID>-rpms”, its corresponding source repository is expected to have an identifier in the form “<ID>-source-rpms” and debuginfo repository an identifier in the form “<ID>-debug-rpms”. Otherwise (if the repository identifier doesn’t have the “-rpms” suffix), the source repository is expected to have an identifier in the form “<ID>-source” and debuginfo repository an identifier in the form “<ID>-debuginfo”.

For example, for repository “fedora”, the source repository is “fedora-source” and debuginfo repository is “fedora-debuginfo”. For repository “fedora-rpms”, the source repository is “fedora-source-rpms” and debuginfo repository is “fedora-debug-rpms”.

## Repo Variables[](https://dnf.readthedocs.io/en/latest/conf_ref.html#repo-variables)

Right side of every repo option can be enriched by the following variables:

$arch

Refers to the system’s CPU architecture e.g, aarch64, i586, i686 and x86_64.

$basearch

Refers to the base architecture of the system. For example, i686 and i586 machines both have a base architecture of i386, and AMD64 and Intel64 machines have a base architecture of x86_64.

$releasever

Refers to the release version of operating system which DNF derives from information available in RPMDB.

$releasever_major

Major version of $releasever, i.e. the component of $releasever occurring before the first ..

$releasever_minor

Minor version of $releasever, i.e. the component of $releasever occurring after the first ..

In addition to these hard coded variables, user-defined ones can also be used. They can be defined either via [variable files](https://dnf.readthedocs.io/en/latest/conf_ref.html#varfiles-label), or by using special environmental variables. The names of these variables must be prefixed with DNF_VAR_ and they can only consist of alphanumeric characters and underscores:

```
$ DNF_VAR_MY_VARIABLE=value

```

To use such variable in your repository configuration remove the prefix. E.g.:

```
[myrepo]
baseurl=https://example.site/pub/fedora/$MY_VARIABLE/releases/$releasever

```

Note that it is not possible to override the arch and basearch variables using either variable files or environmental variables.

Although users are encouraged to use named variables, the numbered environmental variables DNF0 - DNF9 are still supported:

```
$ DNF1=value

[myrepo]
baseurl=https://example.site/pub/fedora/$DNF1/releases/$releasever

```

A limited form of shell-like parameter expansion is supported for variables.

${my_variable:-word} If my_variable is unset or empty, then word will be substituted. Otherwise, the value of my_variable will be substituted.

${my_variable:+word} If my_variable is set and not empty, then word will be substituted. Otherwise, the empty string will be substituted.

Parameter expansions can be nested up to a maximum depth of 32. For example:

```
${my_defined_variable:+${my_undefined_variable:-foobar}}

```

will evaluate to foobar.

## Options for both [main] and Repo[](https://dnf.readthedocs.io/en/latest/conf_ref.html#options-for-both-main-and-repo)

Some options can be applied in either the main section, per repository, or in a combination. The value provided in the main section is used for all repositories as the default value, which repositories can then override in their configuration.

auto_import_local_keys

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

Whether to import the GPG keys from the [gpgkey](https://dnf.readthedocs.io/en/latest/conf_ref.html#repo-gpgkey-label) URLs of this repository without asking for confirmation when the key file is inside one of the trusted keys directories. The default is False.

The trusted keys directories are /usr/share/pki/rpm-gpg for keys installed by packages and /etc/pki/rpm-gpg for keys added by the administrator, unless changed when libdnf is built (CMake option AUTO_IMPORT_KEYS_DIRS). They cannot be changed in configuration files.

The key URL must be spelled file:///path. The path is resolved before it is compared with the directories, so .., a symlink pointing outside them, and a directory whose name only starts with the same prefix (for example /etc/pki/rpm-gpg-other) are not trusted. Any other key, including a key from a remote URL or a key spelled file:/path, asks for confirmation as usual, so the fingerprint can be verified before the key is trusted. [assumeno](https://dnf.readthedocs.io/en/latest/conf_ref.html#assumeno-label) takes precedence: the import is declined and nothing is imported.

The option applies to both places where DNF imports keys: the per-repository keyring used for the metadata signature check ([repo_gpgcheck](https://dnf.readthedocs.io/en/latest/conf_ref.html#repo-gpgcheck-label)) and the RPM database used for the package signature check ([gpgcheck](https://dnf.readthedocs.io/en/latest/conf_ref.html#gpgcheck-label)). Keys for the package check are imported when a transaction runs, so commands that only read repositories do not change the RPM database. When every key of a repository can be imported this way, the key import does not need -y in a run without a terminal. The signature checks themselves are not affected.

bandwidth

storage size

Total bandwidth available for downloading. Meaningful when used with the [throttle option](https://dnf.readthedocs.io/en/latest/conf_ref.html#throttle-label). Storage size is in bytes by default but can be specified with a unit of storage. Valid units are ‘k’, ‘M’, ‘G’.

countme

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

When enabled, one (and only one) HTTP GET request for the metalink file will be selected at random every week to carry a special URL flag.

This flag allows the repository provider to estimate the number of systems consuming the repository, by counting such requests over a week’s time. This method is more accurate than just counting unique IP addresses (which is subject to both overcounting and undercounting due to short DHCP leases and NAT, respectively).

This is not an out-of-band HTTP request made for this purpose alone. Only requests initiated by DNF during normal operation, such as to check for metadata updates, can get this flag.

The flag is a simple “countme=N” parameter appended to the metalink URL where N is an integer representing the age “bucket” this system belongs to. Four buckets are defined, based on how many full weeks have passed since the installation of a system:

|

bucket

|

system age

|

1

|

first week

|

2

|

first month (2 - 4 weeks)

|

3

|

first 6 months (5 - 24 weeks)

|

4

|

more than 6 months (> 24 weeks)

This number is meant to help distinguish short-lived (throwaway) machines from long-term installs and get a better picture of how systems are used over time.

To determine a system’s installation time (“epoch”), the machine-id(5) file’s modification time is used as the single source of truth. This file is semantically tied to the system’s lifetime as it’s typically populated at installation time or during the first boot by an installer tool or init system (such as systemd(1)), respectively, and remains unchanged.

If the file is empty or missing (such as in containers), the time of the very first request made using the expanded metalink URL (i.e. with any repository variables such as $releasever substituted) that carried the flag is declared as the epoch.

If no metalink URL is defined for this repository but a mirrorlist URL is, the latter is used for this purpose instead.

Default is False.

deltarpm

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

When enabled, DNF will save bandwidth by downloading much smaller delta RPM files, rebuilding them to RPM locally. However, this is quite CPU and I/O intensive. Default is False. It requires /usr/bin/applydeltarpm on the system.

deltarpm_percentage

[integer](https://dnf.readthedocs.io/en/latest/conf_ref.html#integer-label)

When the relative size of delta vs pkg is larger than this, delta is not used. Default value is 75 (Deltas must be at least 25% smaller than the pkg). Use 0 to turn off delta rpm processing. Local repositories (with file:// baseurl) have delta rpms turned off by default.

enablegroups

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

Determines whether DNF will allow the use of package groups for this repository. Default is True (package groups are allowed).

excludepkgs

[list](https://dnf.readthedocs.io/en/latest/conf_ref.html#list-label)

Exclude packages of this repository, specified by a name or a glob and separated by a comma, from all operations. Can be disabled using --disableexcludes command line switch. Defaults to [].

fastestmirror

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

If enabled, TCP socket latency is used to find the closest available mirror. A mirror is then selected at random with less than twice the lowest latency for load balancing purposes. This overrides the order provided by the mirrorlist/metalink file itself, and does not take into consideration mirrorlist parameters such as mirror bandwidth nor preferred mirrors for client IP addresses.

gpgcheck

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

Whether to perform GPG signature check on packages found in this repository. The default is False.

This option can only be used to strengthen the active RPM security policy set with the %_pkgverify_level macro (see the /usr/lib/rpm/macros file for details). That means, if the macro is set to ‘signature’ or ‘all’ and this option is False, it will be overridden to True during DNF runtime, and a warning will be printed. To squelch the warning, make sure this option is True for every enabled repository, and also enable [localpkg_gpgcheck](https://dnf.readthedocs.io/en/latest/conf_ref.html#localpkg-gpgcheck-label).

To import the repository’s signing keys without asking for confirmation, see [auto_import_local_keys](https://dnf.readthedocs.io/en/latest/conf_ref.html#auto-import-local-keys-label).

includepkgs

[list](https://dnf.readthedocs.io/en/latest/conf_ref.html#list-label)

Include packages of this repository, specified by a name or a glob and separated by a comma, in all operations. Inverse of [excludepkgs](https://dnf.readthedocs.io/en/latest/conf_ref.html#exclude-label), DNF will exclude any package in the repository that doesn’t match this list. This works in conjunction with excludepkgs and doesn’t override it, so if you ‘excludepkgs=*.i386’ and ‘includepkgs=python*’ then only packages starting with python that do not have an i386 arch will be seen by DNF in this repo. Can be disabled using --disableexcludes command line switch. Defaults to [].

ip_resolve

IP address type

Determines how DNF resolves host names. Set this to ‘4’/’IPv4’ or ‘6’/’IPv6’ to resolve to IPv4 or IPv6 addresses only. By default, DNF resolves to either addresses.

localpkg_gpgcheck

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

Whether to perform a GPG signature check on local packages (packages in a file, not in a repository). The default is False. This option is subject to the active RPM security policy (see [gpgcheck](https://dnf.readthedocs.io/en/latest/conf_ref.html#gpgcheck-label) for more details).

max_parallel_downloads

[integer](https://dnf.readthedocs.io/en/latest/conf_ref.html#integer-label)

Maximum number of simultaneous package downloads. Defaults to 3. Maximum of 20.

metadata_expire

time in seconds

The period after which the remote repository is checked for metadata update and in the positive case the local metadata cache is updated. The default corresponds to 48 hours. Set this to -1 or never to make the repo never considered expired. Expire of metadata can be also triggered by change of timestamp of configuration files (dnf.conf, <repo>.repo). See also [check_config_file_age](https://dnf.readthedocs.io/en/latest/conf_ref.html#check-config-file-age-label).

minrate

storage size

This sets the low speed threshold in bytes per second. If the server is sending data at the same or slower speed than this value for at least [timeout option](https://dnf.readthedocs.io/en/latest/conf_ref.html#timeout-label) seconds, DNF aborts the connection. The default is 1000. Valid units are ‘k’, ‘M’, ‘G’.

password

[string](https://dnf.readthedocs.io/en/latest/conf_ref.html#string-label)

The password to use for connecting to a repository with basic HTTP authentication. Empty by default.

proxy

[string](https://dnf.readthedocs.io/en/latest/conf_ref.html#string-label)

URL of a proxy server to connect through. Set to an empty string in the repository configuration to disable proxy setting inherited from the main section. The expected format of this option is <scheme>://<ip-or-hostname>[:port]. (For backward compatibility, ‘_none_’ can be used instead of the empty string.)

Note: The curl environment variables (such as http_proxy) are effective if this option is unset (or ‘_none_’ is set in the repository configuration). See the curl man page for details.

proxy_username

[string](https://dnf.readthedocs.io/en/latest/conf_ref.html#string-label)

The username to use for connecting to the proxy server. Empty by default.

proxy_password

[string](https://dnf.readthedocs.io/en/latest/conf_ref.html#string-label)

The password to use for connecting to the proxy server. Empty by default.

proxy_auth_method

[string](https://dnf.readthedocs.io/en/latest/conf_ref.html#string-label)

The authentication method used by the proxy server. Valid values are

|

method

|

meaning

|

basic

|

HTTP Basic authentication

|

digest

|

HTTP Digest authentication

|

negotiate

|

HTTP Negotiate (SPNEGO) authentication

|

ntlm

|

HTTP NTLM authentication

|

digest_ie

|

HTTP Digest authentication with an IE flavor

|

ntlm_wb

|

NTLM delegating to winbind helper

|

none

|

None auth method

|

any

|

All suitable methods

Defaults to any

proxy_sslcacert

[string](https://dnf.readthedocs.io/en/latest/conf_ref.html#string-label)

Path to the file containing the certificate authorities to verify proxy SSL certificates. Empty by default - uses system default.

proxy_sslverify

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

When enabled, proxy SSL certificates are verified. If the client can not be authenticated, connecting fails and the repository is not used any further. If False, SSL connections can be used, but certificates are not verified. Default is True.

proxy_sslclientcert

[string](https://dnf.readthedocs.io/en/latest/conf_ref.html#string-label)

Path to the SSL client certificate used to connect to proxy server. Empty by default.

proxy_sslclientkey

[string](https://dnf.readthedocs.io/en/latest/conf_ref.html#string-label)

Path to the SSL client key used to connect to proxy server. Empty by default.

repo_gpgcheck

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

Whether to perform GPG signature check on this repository’s metadata. The default is False. Note that GPG keys for this check are stored separately from GPG keys used in package signature verification. Furthermore, they are also stored separately for each repository.

This means that dnf may ask to import the same key multiple times. For example, when a key was already imported for package signature verification and this option is turned on, it may be needed to import it again for the repository.

To import these keys without asking for confirmation, see [auto_import_local_keys](https://dnf.readthedocs.io/en/latest/conf_ref.html#auto-import-local-keys-label).

retries

[integer](https://dnf.readthedocs.io/en/latest/conf_ref.html#integer-label)

Set the number of total retries for downloading packages. The number is accumulative, so e.g. for retries=10, dnf will fail after any package download fails for eleventh time. Setting this to 0 makes dnf try forever. Default is 10.

skip_if_unavailable

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

If enabled, DNF will continue running and disable the repository that couldn’t be synchronized for any reason. This option doesn’t affect skipping of unavailable packages after dependency resolution. To check inaccessibility of repository use it in combination with [refresh command line option](https://dnf.readthedocs.io/en/latest/command_ref.html#refresh-command-label). The default is False. Note this option in particular [can be set in your configuration file by your distribution](https://dnf.readthedocs.io/en/latest/conf_ref.html#conf-distribution-specific-label).

sslcacert

[string](https://dnf.readthedocs.io/en/latest/conf_ref.html#string-label)

Path to the file containing the certificate authorities to verify SSL certificates. Empty by default - uses system default.

sslverify

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

When enabled, remote SSL certificates are verified. If the client can not be authenticated, connecting fails and the repository is not used any further. If False, SSL connections can be used, but certificates are not verified. Default is True.

sslverifystatus

[boolean](https://dnf.readthedocs.io/en/latest/conf_ref.html#boolean-label)

When enabled, revocation status of the server certificate is verified using the “Certificate Status Request” TLS extension (aka. OCSP stapling). Default is False.

sslclientcert

[string](https://dnf.readthedocs.io/en/latest/conf_ref.html#string-label)

Path to the SSL client certificate used to connect to remote sites. Empty by default.

sslclientkey

[string](https://dnf.readthedocs.io/en/latest/conf_ref.html#string-label)

Path to the SSL client key used to connect to remote sites. Empty by default.

throttle

storage size

Limits the downloading speed. It might be an absolute value or a percentage, relative to the value of the [bandwidth option](https://dnf.readthedocs.io/en/latest/conf_ref.html#bandwidth-label) option. 0 means no throttling (the default). The absolute value is in bytes by default but can be specified with a unit of storage. Valid units are ‘k’, ‘M’, ‘G’.

timeout

time in seconds

Number of seconds to wait for a connection before timing out. Used in combination with [minrate option](https://dnf.readthedocs.io/en/latest/conf_ref.html#minrate-label) option. Defaults to 30 seconds.

username

[string](https://dnf.readthedocs.io/en/latest/conf_ref.html#string-label)

The username to use for connecting to repo with basic HTTP authentication. Empty by default.

user_agent

[string](https://dnf.readthedocs.io/en/latest/conf_ref.html#string-label)

The User-Agent string to include in HTTP requests sent by DNF. Defaults to

```
libdnf (NAME VERSION_ID; VARIANT_ID; OS.BASEARCH)

```

where NAME, VERSION_ID and VARIANT_ID are OS identifiers read from the os-release(5) file, and OS and BASEARCH are the canonical OS name and base architecture, respectively. Example:

```
libdnf (Fedora 31; server; Linux.x86_64)

```

## Types of Options[](https://dnf.readthedocs.io/en/latest/conf_ref.html#types-of-options)

boolean

This is a data type with only two possible values.

One of following options can be used: 1, 0, True, False, yes, no

integer

It is a whole number that can be written without a fractional component.

list

It is an option that could represent one or more strings separated by space or comma characters.

string

It is a sequence of symbols or digits without any whitespace character.

color

A string describing color and modifiers separated with a comma, for example “red,bold”.

-

Colors: black, blue, cyan, green, magenta, red, white, yellow

-

Modifiers: bold, blink, dim, normal, reverse, underline

## Files[](https://dnf.readthedocs.io/en/latest/conf_ref.html#files)

Cache Files

/var/cache/dnf

Main Configuration File

/etc/dnf/dnf.conf

Repository

/etc/yum.repos.d/

Variables

Any properly named file in /etc/dnf/vars is turned into a variable named after the filename (or overrides any of the above variables but those set from commandline). Filenames may contain only alphanumeric characters and underscores and be in lowercase. Variables are also read from /etc/yum/vars for YUM compatibility reasons.

## See Also[](https://dnf.readthedocs.io/en/latest/conf_ref.html#see-also)

-

dnf(8), [DNF Command Reference](https://dnf.readthedocs.io/en/latest/command_ref.html#command-ref-label)
