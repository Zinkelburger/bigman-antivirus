---
url: "https://docs.python.org/3/library/site.html"
title: "site — Site-specific configuration hook"
site: "Python documentation"
captured: "2026-11-02T18:28:56Z"
via: "live"
sha256: "b0704e1edc665afaa2513bf399030e7980f5dfda1650fbd54b80c62d55248dd3"
wayback: "https://web.archive.org/web/20261028121322/https://docs.python.org/3/library/site.html"
cited_in:
  - "docs/threats/02-supply-chain-packages.md"
---

# site — Site-specific configuration hook

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://docs.python.org/3/library/site.html>
> Snapshot: <https://web.archive.org/web/20261028121322/https://docs.python.org/3/library/site.html>

### Navigation

-  [index](https://docs.python.org/3/genindex.html)
-  [modules](https://docs.python.org/3/py-modindex.html) |
-  [next](https://docs.python.org/3/library/custominterp.html) |
-  [previous](https://docs.python.org/3/library/annotationlib.html) |
-
- [Python](https://www.python.org/) »
-

-
-  [3.15.2 documentation](https://docs.python.org/3/index.html) »
- [The Python standard library](https://docs.python.org/3/library/index.html) »
- [Python Runtime Services](https://docs.python.org/3/library/python.html) »
- site — Site-specific configuration hook
-

|
-   Theme   |

# site — Site-specific configuration hook[¶](https://docs.python.org/3/library/site.html#module-site)

Source code: [Lib/site.py](https://github.com/python/cpython/tree/3.15/Lib/site.py)

This module is automatically imported during initialization. The automatic import can be suppressed using the interpreter’s [-S](https://docs.python.org/3/using/cmdline.html#cmdoption-S) option.

Importing this module normally appends site-specific paths to the module search path and adds [callables](https://docs.python.org/3/builtins/constants.html#site-consts), including [help()](https://docs.python.org/3/builtins/functions.html#help) to the built-in namespace. However, Python startup option [-S](https://docs.python.org/3/using/cmdline.html#cmdoption-S) blocks this, and this module can be safely imported with no automatic modifications to the module search path or additions to the builtins. To explicitly trigger the usual site-specific additions, call the [main()](https://docs.python.org/3/library/site.html#site.main) function.

Changed in version 3.3: Importing the module used to trigger paths manipulation even when using [-S](https://docs.python.org/3/using/cmdline.html#cmdoption-S).

It starts by constructing up to four directories from a head and a tail part. For the head part, it uses sys.prefix and sys.exec_prefix; empty heads are skipped. For the tail part, it uses the empty string and then lib/site-packages (on Windows) or lib/pythonX.Y[t]/site-packages (on Unix and macOS). (The optional suffix “t” indicates the [free-threaded build](https://docs.python.org/3/glossary.html#term-free-threaded-build), and is appended if "t" is present in the [sys.abiflags](https://docs.python.org/3/library/sys.html#sys.abiflags) constant.) For each of the distinct head-tail combinations, it sees if it refers to an existing directory, and if so, adds it to sys.path and also inspects the newly added path for configuration files.

Changed in version 3.5: Support for the “site-python” directory has been removed.

Changed in version 3.13: On Unix, [Free threading](https://docs.python.org/3/glossary.html#term-free-threading) Python installations are identified by the “t” suffix in the version-specific directory name, such as lib/python3.13t/.

Changed in version 3.14: site is no longer responsible for updating [sys.prefix](https://docs.python.org/3/library/sys.html#sys.prefix) and [sys.exec_prefix](https://docs.python.org/3/library/sys.html#sys.exec_prefix) on [Virtual Environments](https://docs.python.org/3/library/sys_path_init.html#sys-path-init-virtual-environments). This is now done during the [path initialization](https://docs.python.org/3/library/sys_path_init.html#sys-path-init). As a result, under [Virtual Environments](https://docs.python.org/3/library/sys_path_init.html#sys-path-init-virtual-environments), sys.prefix and sys.exec_prefix no longer depend on the site initialization, and are therefore unaffected by [-S](https://docs.python.org/3/using/cmdline.html#cmdoption-S).

When running under a [virtual environment](https://docs.python.org/3/library/sys_path_init.html#sys-path-init-virtual-environments), the pyvenv.cfg file in [sys.prefix](https://docs.python.org/3/library/sys.html#sys.prefix) is checked for site-specific configurations. If the include-system-site-packages key exists and is set to true (case-insensitive), the system-level prefixes will be searched for site-packages, otherwise they won’t. If the system-level prefixes are not searched then the user site prefixes are also implicitly not searched for site-packages.

The site module recognizes two startup configuration files of the form name.pth for path configurations, and name.start for pre-first-line code execution. Both files can exist in one of the four directories mentioned above. Within each directory, these files are sorted alphabetically by filename, then parsed in sorted order.

## Path extensions (.pth files)[¶](https://docs.python.org/3/library/site.html#path-extensions-pth-files)

name.pth contains additional items (one per line) to be appended to sys.path. Items that name non-existing directories are never added to sys.path, and no check is made that the item refers to a directory rather than a file. No item is added to sys.path more than once. Blank lines and lines beginning with # are skipped.

For backward compatibility, lines starting with import (followed by space or tab) are executed with [exec()](https://docs.python.org/3/builtins/functions.html#exec).

Changed in version 3.13: The .pth files are now decoded by UTF-8 at first and then by the [locale encoding](https://docs.python.org/3/glossary.html#term-locale-encoding) if it fails.

Changed in version 3.15: .pth file lines starting with import are deprecated. During the deprecation period, such lines are still executed (except in the case below), but a diagnostic message is emitted only when the [-v](https://docs.python.org/3/using/cmdline.html#cmdoption-v) flag is given.

import lines in name.pth are silently ignored when a [matching](https://docs.python.org/3/library/site.html#site-start-files) name.start file exists.

Errors on individual lines no longer abort processing of the rest of the file. Each error is reported and the remaining lines continue to be processed.

Deprecated since version 3.15, will be removed in version 3.20: Decoding name.pth files in any encoding other than utf-8-sig is deprecated in Python 3.15, and support for decoding from the locale encoding will be removed in Python 3.20.

import lines in name.pth files are deprecated and will be silently ignored in Python 3.18 and 3.19. In Python 3.20 a warning will be produced for import lines in name.pth files.

## Startup entry points (.start files)[¶](https://docs.python.org/3/library/site.html#startup-entry-points-start-files)

Added in version 3.15.

A startup entry point file is a file whose name has the form name.start and exists in one of the site-packages directories described above. Each file specifies entry points to be called during interpreter startup, using the pkg.mod:callable syntax understood by [pkgutil.resolve_name()](https://docs.python.org/3/library/pkgutil.html#pkgutil.resolve_name).

Each non-blank line that does not begin with # must contain an entry point reference in the form pkg.mod:callable. The colon and callable portion are mandatory. Each callable is invoked with no arguments, and any return value is discarded.

.start files are processed after all .pth path extensions have been applied to [sys.path](https://docs.python.org/3/library/sys.html#sys.path), ensuring that paths are available before any startup code runs.

Unlike [sys.path](https://docs.python.org/3/library/sys.html#sys.path) extensions from .pth files, duplicate entry points are not de-duplicated — if an entry point appears more than once, it will be called more than once.

If an exception occurs during resolution or invocation of an entry point, a traceback is printed to [sys.stderr](https://docs.python.org/3/library/sys.html#sys.stderr) and processing continues with the remaining entry points.

.start files must be encoded in UTF-8.

[PEP 829](https://peps.python.org/pep-0829/) defined the original specification for these features.

Note

If a name.start file exists alongside a name.pth file with the same base name, any import lines in the .pth file are ignored in favor of the entry points in the .start file.

Note

Executable lines (import lines in name.pth files and name.start file entry points) are always run at Python startup (unless [-S](https://docs.python.org/3/using/cmdline.html#cmdoption-S) is given to disable the site.py module entirely), regardless of whether a particular module is actually going to be used.

Note

name.start files invoke [pkgutil.resolve_name()](https://docs.python.org/3/library/pkgutil.html#pkgutil.resolve_name) with strict=True, which requires the full pkg.mod:callable form.

## Startup file examples[¶](https://docs.python.org/3/library/site.html#startup-file-examples)

For example, suppose sys.prefix and sys.exec_prefix are set to /usr/local. The Python X.Y library is then installed in /usr/local/lib/pythonX.Y. Suppose this has a subdirectory /usr/local/lib/pythonX.Y/site-packages with three sub-subdirectories, foo, bar and spam, and two path configuration files, foo.pth and bar.pth. Assume foo.pth contains the following:

```
# foo package configuration

foo
bar
bletch

```

and bar.pth contains:

```
# bar package configuration

bar

```

Then the following version-specific directories are added to sys.path, in this order:

```
/usr/local/lib/pythonX.Y/site-packages/bar
/usr/local/lib/pythonX.Y/site-packages/foo

```

Note that bletch is omitted because it doesn’t exist; the bar directory precedes the foo directory because bar.pth comes alphabetically before foo.pth; and spam is omitted because it is not mentioned in either path configuration file.

Let’s say that there is also a foo.start file containing the following:

```
# foo package startup code

foo.submod:initialize

```

Now, after sys.path has been extended as above, and before Python turns control over to user code, the foo.submod module is imported and the initialize() function from that module is called.

## Migrating from import lines in .pth files to .start files[¶](https://docs.python.org/3/library/site.html#migrating-from-import-lines-in-pth-files-to-start-files)

If your package currently ships a name.pth file, you can keep all sys.path extension lines unchanged. Only import lines need to be migrated.

To migrate, create a callable (taking zero arguments) within an importable module in your package. Reference it as a pkg.mod:callable entry point in a matching name.start file. Move everything on your import line after the first semi-colon into the callable() function.

If your package must straddle older Pythons that do not support [PEP 829](https://peps.python.org/pep-0829/) and newer Pythons that do, change the import lines in your name.pth to use the following form:

```
import pkg.mod; pkg.mod.callable()

```

Older Pythons will execute these import lines, while newer Pythons will ignore them in favor of the name.start file. After the straddling period, remove all import lines from your .pth files.

## sitecustomize[¶](https://docs.python.org/3/library/site.html#module-sitecustomize)

After these path manipulations, an attempt is made to import a module named sitecustomize, which can perform arbitrary site-specific customizations. It is typically created by a system administrator in the site-packages directory. If this import fails with an [ImportError](https://docs.python.org/3/builtins/exceptions.html#ImportError) or its subclass exception, and the exception’s [name](https://docs.python.org/3/builtins/exceptions.html#ImportError.name) attribute equals 'sitecustomize', it is silently ignored. If Python is started without output streams available, as with pythonw.exe on Windows (which is used by default to start IDLE), attempted output from sitecustomize is ignored. Any other exception causes a silent and perhaps mysterious failure of the process.

## usercustomize[¶](https://docs.python.org/3/library/site.html#module-usercustomize)

After this, an attempt is made to import a module named usercustomize, which can perform arbitrary user-specific customizations, if [ENABLE_USER_SITE](https://docs.python.org/3/library/site.html#site.ENABLE_USER_SITE) is true. This file is intended to be created in the user site-packages directory (see below), which is part of sys.path unless disabled by [-s](https://docs.python.org/3/using/cmdline.html#cmdoption-s). If this import fails with an [ImportError](https://docs.python.org/3/builtins/exceptions.html#ImportError) or its subclass exception, and the exception’s [name](https://docs.python.org/3/builtins/exceptions.html#ImportError.name) attribute equals 'usercustomize', it is silently ignored.

Note that for some non-Unix systems, sys.prefix and sys.exec_prefix are empty, and the path manipulations are skipped; however the import of [sitecustomize](https://docs.python.org/3/library/site.html#module-sitecustomize) and usercustomize is still attempted.

## Readline configuration[¶](https://docs.python.org/3/library/site.html#readline-configuration)

On systems that support [readline](https://docs.python.org/3/library/readline.html#module-readline), this module will also import and configure the [rlcompleter](https://docs.python.org/3/library/rlcompleter.html#module-rlcompleter) module, if Python is started in [interactive mode](https://docs.python.org/3/tutorial/interpreter.html#tut-interactive) and without the [-S](https://docs.python.org/3/using/cmdline.html#cmdoption-S) option. The default behavior is to enable tab completion and to use ~/.python_history as the history save file. To disable it, delete (or override) the [sys.__interactivehook__](https://docs.python.org/3/library/sys.html#sys.__interactivehook__) attribute in your [sitecustomize](https://docs.python.org/3/library/site.html#module-sitecustomize) or [usercustomize](https://docs.python.org/3/library/site.html#module-usercustomize) module or your [PYTHONSTARTUP](https://docs.python.org/3/using/cmdline.html#envvar-PYTHONSTARTUP) file.

Changed in version 3.4: Activation of rlcompleter and history was made automatic.

## Module contents[¶](https://docs.python.org/3/library/site.html#module-contents)

site.PREFIXES[¶](https://docs.python.org/3/library/site.html#site.PREFIXES)

A list of prefixes for site-packages directories.

site.ENABLE_USER_SITE[¶](https://docs.python.org/3/library/site.html#site.ENABLE_USER_SITE)

Flag showing the status of the user site-packages directory. True means that it is enabled and was added to sys.path. False means that it was disabled by user request (with [-s](https://docs.python.org/3/using/cmdline.html#cmdoption-s) or [PYTHONNOUSERSITE](https://docs.python.org/3/using/cmdline.html#envvar-PYTHONNOUSERSITE)). None means it was disabled for security reasons (mismatch between user or group id and effective id) or by an administrator.

site.USER_SITE[¶](https://docs.python.org/3/library/site.html#site.USER_SITE)

Path to the user site-packages for the running Python. Can be None if [getusersitepackages()](https://docs.python.org/3/library/site.html#site.getusersitepackages) hasn’t been called yet. Default value is ~/.local/lib/pythonX.Y[t]/site-packages for UNIX and non-framework macOS builds, ~/Library/Python/X.Y/lib/python/site-packages for macOS framework builds, and %APPDATA%\Python\PythonXY\site-packages on Windows. The optional “t” indicates the free-threaded build. This directory is a site directory, which means that .pth files in it will be processed.

site.USER_BASE[¶](https://docs.python.org/3/library/site.html#site.USER_BASE)

Path to the base directory for the user site-packages. Can be None if [getuserbase()](https://docs.python.org/3/library/site.html#site.getuserbase) hasn’t been called yet. Default value is ~/.local for UNIX and macOS non-framework builds, ~/Library/Python/X.Y for macOS framework builds, and %APPDATA%\Python for Windows. This value is used to compute the installation directories for scripts, data files, Python modules, etc. for the [user installation scheme](https://docs.python.org/3/library/sysconfig.html#sysconfig-user-scheme). See also [PYTHONUSERBASE](https://docs.python.org/3/using/cmdline.html#envvar-PYTHONUSERBASE).

site.main()[¶](https://docs.python.org/3/library/site.html#site.main)

Adds all the standard site-specific directories to the module search path. This function is called automatically when this module is imported, unless the Python interpreter was started with the [-S](https://docs.python.org/3/using/cmdline.html#cmdoption-S) flag.

Changed in version 3.3: This function used to be called unconditionally.

site.makepath(*paths)[¶](https://docs.python.org/3/library/site.html#site.makepath)

Join paths with [os.path.join()](https://docs.python.org/3/library/os.path.html#os.path.join), attempt to make the result absolute with [os.path.abspath()](https://docs.python.org/3/library/os.path.html#os.path.abspath), and return a 2-tuple containing the absolute path and its case-normalized form as produced by [os.path.normcase()](https://docs.python.org/3/library/os.path.html#os.path.normcase). If os.path.abspath() raises [OSError](https://docs.python.org/3/builtins/exceptions.html#OSError), the joined path is used unchanged for the case-normalization step.

The second element of the returned tuple is the form used throughout the site module to compare paths on case-insensitive file systems, and is what populates the known_paths sets that prevent duplicate [sys.path](https://docs.python.org/3/library/sys.html#sys.path) entries in various APIs within this module.

class site.StartupState(known_paths=None)[¶](https://docs.python.org/3/library/site.html#site.StartupState)

Instances of this class accumulate interpreter startup configuration data from one or more site directories. They are the preferred interface for batching the processing of .pth and .start files across multiple site directories, so that every [sys.path](https://docs.python.org/3/library/sys.html#sys.path) extension is visible before any startup code runs.

The optional known_paths argument is a set of case-normalized paths (which can be produced by [makepath()](https://docs.python.org/3/library/site.html#site.makepath)) used to prevent duplicate [sys.path](https://docs.python.org/3/library/sys.html#sys.path) entries. When None (the default), the set is built from the current sys.path. [main()](https://docs.python.org/3/library/site.html#site.main) implicitly uses an instance of this class.

Typical use:

```
state = site.StartupState()
for sitedir in site_dirs:
    state.addsitedir(sitedir)
state.process()

```

Added in version 3.15.

addsitedir(sitedir)[¶](https://docs.python.org/3/library/site.html#site.StartupState.addsitedir)

Read the .pth and .start files in sitedir and record their [sys.path](https://docs.python.org/3/library/sys.html#sys.path) extensions, deprecated .pth import lines, and .start entry points on this state. The recorded data is not applied until [process()](https://docs.python.org/3/library/site.html#site.StartupState.process) is called.

addusersitepackages()[¶](https://docs.python.org/3/library/site.html#site.StartupState.addusersitepackages)

Add the per-user site-packages directory, if enabled and if it exists. The directory’s startup data is accumulated for later processing by [process()](https://docs.python.org/3/library/site.html#site.StartupState.process).

addsitepackages(prefixes=None)[¶](https://docs.python.org/3/library/site.html#site.StartupState.addsitepackages)

Add global site-packages directories, computed from prefixes or from the global [PREFIXES](https://docs.python.org/3/library/site.html#site.PREFIXES) when prefixes is None. Each directory’s startup data is accumulated for later processing by [process()](https://docs.python.org/3/library/site.html#site.StartupState.process).

process()[¶](https://docs.python.org/3/library/site.html#site.StartupState.process)

Apply the accumulated state by first adding the path extensions to [sys.path](https://docs.python.org/3/library/sys.html#sys.path), then executing the .start file entry points and .pth file import lines ([deprecated](https://docs.python.org/3/library/site.html#site-pth-files)).

This method is not idempotent and must not be called more than once on the same instance. Doing so will apply the accumulated state more than once, re-running entry points and import lines.

site.addsitedir(sitedir, known_paths=None)[¶](https://docs.python.org/3/library/site.html#site.addsitedir)

Add a directory to sys.path and parse the .pth and .start files found in that directory. Typically used in [sitecustomize](https://docs.python.org/3/library/site.html#module-sitecustomize) or [usercustomize](https://docs.python.org/3/library/site.html#module-usercustomize) (see above).

The known_paths argument is an optional set of case-normalized paths used to prevent duplicate [sys.path](https://docs.python.org/3/library/sys.html#sys.path) entries. When None (the default), the set is built from the current sys.path.

For batched processing across multiple site directories, build a [StartupState](https://docs.python.org/3/library/site.html#site.StartupState) explicitly and call [StartupState.addsitedir()](https://docs.python.org/3/library/site.html#site.StartupState.addsitedir) on it; that defers .pth and .start processing until a single [StartupState.process()](https://docs.python.org/3/library/site.html#site.StartupState.process) call, ensuring every [sys.path](https://docs.python.org/3/library/sys.html#sys.path) extension is visible before any startup code runs.

Changed in version 3.15: Also processes .start files. See [Startup entry points (.start files)](https://docs.python.org/3/library/site.html#site-start-files).

site.getsitepackages(prefixes=None)[¶](https://docs.python.org/3/library/site.html#site.getsitepackages)

Return a list containing all global site-packages directories.

For each directory given in prefixes (or [PREFIXES](https://docs.python.org/3/library/site.html#site.PREFIXES) if prefixes is None), this function will compute its site-packages subdirectory depending on the system environment, and will return a list of full paths, which are not checked for existence.

Added in version 3.2.

Changed in version 3.3: Added the optional prefixes parameter.

site.getuserbase()[¶](https://docs.python.org/3/library/site.html#site.getuserbase)

Return the path of the user base directory, [USER_BASE](https://docs.python.org/3/library/site.html#site.USER_BASE). If it is not initialized yet, this function will also set it, respecting [PYTHONUSERBASE](https://docs.python.org/3/using/cmdline.html#envvar-PYTHONUSERBASE).

Added in version 3.2.

site.getusersitepackages()[¶](https://docs.python.org/3/library/site.html#site.getusersitepackages)

Return the path of the user-specific site-packages directory, [USER_SITE](https://docs.python.org/3/library/site.html#site.USER_SITE). If it is not initialized yet, this function will also set it, respecting [USER_BASE](https://docs.python.org/3/library/site.html#site.USER_BASE). To determine if the user-specific site-packages was added to sys.path [ENABLE_USER_SITE](https://docs.python.org/3/library/site.html#site.ENABLE_USER_SITE) should be used.

Added in version 3.2.

## Command-line interface[¶](https://docs.python.org/3/library/site.html#command-line-interface)

The site module also provides a way to get the user directories from the command line:

```
$ python -m site --user-site
/home/user/.local/lib/python3.11/site-packages

```

If it is called without arguments, it will print the contents of [sys.path](https://docs.python.org/3/library/sys.html#sys.path) on the standard output, followed by the value of [USER_BASE](https://docs.python.org/3/library/site.html#site.USER_BASE) and whether the directory exists, then the same thing for [USER_SITE](https://docs.python.org/3/library/site.html#site.USER_SITE), and finally the value of [ENABLE_USER_SITE](https://docs.python.org/3/library/site.html#site.ENABLE_USER_SITE).

--user-base[¶](https://docs.python.org/3/library/site.html#cmdoption-site-user-base)

Print the path to the user base directory.

--user-site[¶](https://docs.python.org/3/library/site.html#cmdoption-site-user-site)

Print the path to the user site-packages directory.

If both options are given, user base and user site will be printed (always in this order), separated by [os.pathsep](https://docs.python.org/3/library/os.html#os.pathsep).

If any option is given, the script will exit with one of these values: 0 if the user site-packages directory is enabled, 1 if it was disabled by the user, 2 if it is disabled for security reasons or by an administrator, and a value greater than 2 if there is an error.

See also

-

[PEP 370](https://peps.python.org/pep-0370/) – Per user site-packages directory

-

[PEP 829](https://peps.python.org/pep-0829/) – Startup entry points and the deprecation of import lines in .pth files

-

[The initialization of the sys.path module search path](https://docs.python.org/3/library/sys_path_init.html#sys-path-init) – The initialization of [sys.path](https://docs.python.org/3/library/sys.html#sys.path).

### [Table of Contents](https://docs.python.org/3/contents.html)

- [site — Site-specific configuration hook](https://docs.python.org/3/library/site.html#)

- [Path extensions (.pth files)](https://docs.python.org/3/library/site.html#path-extensions-pth-files)
- [Startup entry points (.start files)](https://docs.python.org/3/library/site.html#startup-entry-points-start-files)
- [Startup file examples](https://docs.python.org/3/library/site.html#startup-file-examples)
- [Migrating from import lines in .pth files to .start files](https://docs.python.org/3/library/site.html#migrating-from-import-lines-in-pth-files-to-start-files)
- [sitecustomize](https://docs.python.org/3/library/site.html#module-sitecustomize)
- [usercustomize](https://docs.python.org/3/library/site.html#module-usercustomize)
- [Readline configuration](https://docs.python.org/3/library/site.html#readline-configuration)
- [Module contents](https://docs.python.org/3/library/site.html#module-contents)
- [Command-line interface](https://docs.python.org/3/library/site.html#command-line-interface)

#### Previous topic

[annotationlib — Functionality for introspecting annotations](https://docs.python.org/3/library/annotationlib.html)

#### Next topic

[Custom Python Interpreters](https://docs.python.org/3/library/custominterp.html)

### This page

- [Report a bug](https://docs.python.org/3/bugs.html)
- [Improve this page](https://docs.python.org/3/improve-page-nojs.html)
-  [Show source](https://github.com/python/cpython/blob/3.15/Doc/library/site.rst?plain=1)

«

### Navigation

-  [index](https://docs.python.org/3/genindex.html)
-  [modules](https://docs.python.org/3/py-modindex.html) |
-  [next](https://docs.python.org/3/library/custominterp.html) |
-  [previous](https://docs.python.org/3/library/annotationlib.html) |
-
- [Python](https://www.python.org/) »
-

-
-  [3.15.2 documentation](https://docs.python.org/3/index.html) »
- [The Python standard library](https://docs.python.org/3/library/index.html) »
- [Python Runtime Services](https://docs.python.org/3/library/python.html) »
- site — Site-specific configuration hook
-

|
-   Theme   |

© [Copyright](https://docs.python.org/3/copyright.html) 2001 Python Software Foundation.
This page is licensed under the Python Software Foundation License Version 2.
Examples, recipes, and other code in the documentation are additionally licensed under the Zero Clause BSD License.
See [History and License](https://docs.python.org/license.html) for more information.

The Python Software Foundation is a non-profit corporation. [Please donate.](https://www.python.org/psf/donations/)

Last updated on Nov 02, 2026 (17:22 UTC). [Found a bug](https://docs.python.org/bugs.html)?
Created using [Sphinx](https://www.sphinx-doc.org/) 8.2.3.
