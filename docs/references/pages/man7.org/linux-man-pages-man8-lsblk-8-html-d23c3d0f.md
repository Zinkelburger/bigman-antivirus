---
url: "https://man7.org/linux/man-pages/man8/lsblk.8.html"
title: "lsblk(8) - Linux manual page"
captured: "2026-11-03T02:57:29Z"
via: "live"
sha256: "2026def667b01c35650c37f00f72bc62de53fa9560817d72ab9455ef3fca1c0f"
wayback: "https://web.archive.org/web/20261008094044/https://www.man7.org/linux/man-pages/man8/lsblk.8.html"
cited_in:
  - "docs/research/hardening.md"
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# lsblk(8) - Linux manual page

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://man7.org/linux/man-pages/man8/lsblk.8.html>
> Snapshot: <https://web.archive.org/web/20261008094044/https://www.man7.org/linux/man-pages/man8/lsblk.8.html>

|

[man7.org](https://man7.org/index.html) > Linux > [man-pages](https://man7.org/linux/man-pages/index.html)

|

[Linux/UNIX system programming training](http://man7.org/training/)

# lsblk(8) — Linux manual page

|

[NAME](https://man7.org/linux/man-pages/man8/lsblk.8.html#NAME) | [SYNOPSIS](https://man7.org/linux/man-pages/man8/lsblk.8.html#SYNOPSIS) | [DESCRIPTION](https://man7.org/linux/man-pages/man8/lsblk.8.html#DESCRIPTION) | [OPTIONS](https://man7.org/linux/man-pages/man8/lsblk.8.html#OPTIONS) | [EXIT STATUS](https://man7.org/linux/man-pages/man8/lsblk.8.html#EXIT_STATUS) | [COLORS](https://man7.org/linux/man-pages/man8/lsblk.8.html#COLORS) | [ENVIRONMENT](https://man7.org/linux/man-pages/man8/lsblk.8.html#ENVIRONMENT) | [NOTES](https://man7.org/linux/man-pages/man8/lsblk.8.html#NOTES) | [AUTHORS](https://man7.org/linux/man-pages/man8/lsblk.8.html#AUTHORS) | [SEE ALSO](https://man7.org/linux/man-pages/man8/lsblk.8.html#SEE_ALSO) | [REPORTING BUGS](https://man7.org/linux/man-pages/man8/lsblk.8.html#REPORTING_BUGS) | [AVAILABILITY](https://man7.org/linux/man-pages/man8/lsblk.8.html#AVAILABILITY)

|

|

```

LSBLK(8)                  System Administration                  LSBLK(8)

```

## https://man7.org/linux/man-pages/man8/lsblk.8.html#NAMENAME [top](https://man7.org/linux/man-pages/man8/lsblk.8.html#top_of_page)

```

       lsblk - list block devices

```

## https://man7.org/linux/man-pages/man8/lsblk.8.html#SYNOPSISSYNOPSIS [top](https://man7.org/linux/man-pages/man8/lsblk.8.html#top_of_page)

```

       lsblk [options] [device...]

```

## https://man7.org/linux/man-pages/man8/lsblk.8.html#DESCRIPTIONDESCRIPTION [top](https://man7.org/linux/man-pages/man8/lsblk.8.html#top_of_page)

```

       lsblk lists information about all available or the specified block
       devices. The lsblk command reads the sysfs filesystem and udev db
       to gather information. If the udev db is not available or lsblk is
       compiled without udev support, then it tries to read LABELs, UUIDs
       and filesystem types from the block device. In this case root
       permissions are necessary.

       By default, the command prints all block devices (except RAM
       disks) in a tree-like format. The same device can be repeated in
       the tree if it relates to other devices. The --merge option is
       recommended for more complicated setups to gather groups of
       devices and describe complex N:M relationships.

       The tree-like output (or children[] array in the JSON output) is
       enabled only if NAME column it present in the output or when
       --tree command line option is used. See also --nodeps and --list
       to control the tree formatting.

       The default output, as well as the default output from options
       like --fs and --topology, is subject to change. So whenever
       possible, you should avoid using default outputs in your scripts.
       Always explicitly define expected columns by using --output
       columns-list and --list in environments where a stable output is
       required.

       Use lsblk --list-columns to get a list of all available columns.

       Note that lsblk might be executed in time when udev does not have
       all information about recently added or modified devices yet. In
       this case it is recommended to use udevadm settle before lsblk to
       synchronize with udev.

       The relationship between block devices and filesystems is not
       always one-to-one. The filesystem may use more block devices, or
       the same filesystem may be accessible by more paths. This is the
       reason why lsblk provides MOUNTPOINT and MOUNTPOINTS (pl.)
       columns. The column MOUNTPOINT displays only one mount point
       (usually the last mounted instance of the filesystem), and the
       column MOUNTPOINTS displays by multi-line cell all mount points
       associated with the device.

```

## https://man7.org/linux/man-pages/man8/lsblk.8.html#OPTIONSOPTIONS [top](https://man7.org/linux/man-pages/man8/lsblk.8.html#top_of_page)

```

       -A, --noempty
           Don’t print empty devices.

       -a, --all
           Disable all built-in filters and list all empty devices and
           RAM disk devices too.

       -b, --bytes
           Print sizes in bytes rather than in human-readable form.

           By default, sizes are shown in units that are powers of 1024
           bytes. The formal abbreviations for these units (KiB, MiB,
           GiB, ...) are further shortened to just their first letter: K,
           M, G, ....

       -H, --list-columns
           List the columns that can be specified with the --output
           option. Can be used with --json or --raw to get the list in a
           machine-readable format.

       -D, --discard
           Print information about the discarding capabilities (TRIM,
           UNMAP) for each device.

       -d, --nodeps
           Do not print holder devices or slaves. For example, lsblk
           --nodeps /dev/sda prints information about the sda device
           only.

       -E, --dedup column
           Use column as a de-duplication key to de-duplicate output
           tree. If the key is not available for the device, or the
           device is a partition and parental whole-disk device provides
           the same key than the device is always printed.

           The usual use case is to de-duplicate output on system
           multi-path devices, for example by -E WWN.

       -e, --exclude list
           Exclude the devices specified by the comma-separated list of
           major device numbers. Note that RAM disks (major=1) are
           excluded by default if --all is not specified. The filter is
           applied to the top-level devices only. This may be confusing
           for --list output format where hierarchy of the devices is not
           obvious.

       -f, --fs
           Output info about filesystems. This option is equivalent to -o
           NAME,FSTYPE,FSVER,LABEL,UUID,FSAVAIL,FSUSE%,MOUNTPOINTS. The
           authoritative information about filesystems and raids is
           provided by the [blkid(8)](https://man7.org/linux/man-pages/man8/blkid.8.html) command.

       --hyperlink[=when]
           Print paths as terminal hyperlinks. The optional when argument
           can be always, never, or auto. If the argument is omitted, it
           defaults to auto, which means that hyperlinks will only be
           used when the output goes to a terminal.

       -I, --include list
           Include devices specified by the comma-separated list of major
           device numbers. The filter is applied to the top-level devices
           only. This may be confusing for --list output format where
           hierarchy of the devices is not obvious.

       -i, --ascii
           Use ASCII characters for tree formatting.

       -J, --json
           Use JSON output format. It’s strongly recommended to use
           --output and also --tree if necessary. Note that children[] is
           used only if NAME column or --tree is used.

       -l, --list
           Produce output in the form of a list. The output does not
           provide information about relationships between devices and
           since version 2.34 every device is printed only once if
           --pairs or --raw not specified (the parsable outputs are
           maintained in backwardly compatible way).

       -M, --merge
           Group parents of sub-trees to provide more readable output for
           RAIDs and Multi-path devices. The tree-like output is
           required.

       -m, --perms
           Output info about device owner, group and mode. This option is
           equivalent to -o NAME,SIZE,OWNER,GROUP,MODE.

       -N, --nvme
           Output info about NVMe devices only.

       -v, --virtio
           Output info about virtio devices only.

       -n, --noheadings
           Do not print a header line.

       -o, --output list
           Specify which output columns to print. Use --list-columns to
           get a list of all supported columns. The columns may affect
           tree-like output. The default is to use tree for the column
           'NAME' (see also --tree).

           The default list of columns may be extended if list is
           specified in the format +list (e.g., lsblk -o +UUID).

       -O, --output-all
           Output all available columns.

       -P, --pairs
           Produce output in the form of key="value" pairs. The output
           lines are still ordered by dependencies. All potentially
           unsafe value characters are hex-escaped (\x<code>). See also
           option --shell.

       -p, --paths
           Print full device paths.

       -Q, --filter expression
           Print only the devices that meet the conditions specified by
           expression. The filter is assessed prior to lsblk collecting
           data for all output columns. Only the necessary data for the
           lazy evaluation of the expression is retrieved from the
           system. This approach can enhance performance when compared to
           post-filtering, as commonly done by tools such as [grep(1)](https://man7.org/linux/man-pages/man1/grep.1.html).

           This feature is EXPERIMENTAL. See also [scols-filter(5)](https://man7.org/linux/man-pages/man5/scols-filter.5.html).

           For example, to exclude sda and sdb, but print everything else
           ('!~' is a negative regular-expression matching operator):

              lsblk --filter 'NAME !~ "sd[ab]"'

       --highlight expression
           Colorize lines matching expression.

           This feature is EXPERIMENTAL. See also [scols-filter(5)](https://man7.org/linux/man-pages/man5/scols-filter.5.html).

       --ct name[:parameter[:function]]
           Define a custom counter. Custom counters are printed after the
           standard output. The name is the custom name of the counter,
           the optional parameter is the name of the column to be used
           for the counter, and the optional function specifies the
           aggregation function. Supported functions are: count, min,
           max, sum. The default function is count. If parameter is not
           specified, then the counter counts the number of lines.

           This feature is EXPERIMENTAL. See also --ct-filter.

           For example, --ct MyCounter:SIZE:sum reports the total of the
           SIZE values. To report the number of SATA disks, one can use:

              lsblk --ct-filter 'TYPE=="disk" && TRAN=="sata"' \
                    --ct "Number of SATA devices"

       --ct-filter expression
           Define a restriction for the next counter.

           This feature is EXPERIMENTAL. See also --ct and
           [scols-filter(5)](https://man7.org/linux/man-pages/man5/scols-filter.5.html).

           For example, to aggregate sizes by device type:

              lsblk --ct-filter 'TYPE=="part"' --ct Partitions:SIZE:sum \
                    --ct-filter 'TYPE=="disk"' --ct WholeDisks:SIZE:sum

       -r, --raw
           Produce output in raw format. The output lines are still
           ordered by dependencies. All potentially unsafe characters are
           hex-escaped (\x<code>) in the NAME, KNAME, LABEL, PARTLABEL
           and MOUNTPOINT columns.

       -S, --scsi
           Output info about SCSI devices only. All partitions, slaves
           and holder devices are ignored.

       -s, --inverse
           Print dependencies in inverse order. If the --list output is
           requested then the lines are still ordered by dependencies.

       -T, --tree[=column]
           Force tree-like output format. If column is specified, then a
           tree is printed in the column. The default is NAME column.

       -t, --topology
           Output info about block-device topology. This option is
           equivalent to

           -o
           NAME,ALIGNMENT,MIN-IO,OPT-IO,PHY-SEC,LOG-SEC,ROTA,SCHED,RQ-SIZE,RA,WSAME.

       -w, --width number
           Specifies output width as a number of characters. The default
           is the number of the terminal columns, and if not executed on
           a terminal, then output width is not restricted at all by
           default. This option also forces lsblk to assume that terminal
           control characters and unsafe characters are not allowed. The
           expected use-case is for example when lsblk is used by the
           [watch(1)](https://man7.org/linux/man-pages/man1/watch.1.html) command.

       -x, --sort column
           Sort output lines by column. This option enables --list output
           format by default. It is possible to use the option --tree to
           force tree-like output and than the tree branches are sorted
           by the column.

       -y, --shell
           The column name will be modified to contain only characters
           allowed for shell variable identifiers, for example, MIN_IO
           and FSUSE_PCT instead of MIN-IO and FSUSE%. This is usable,
           for example, with --pairs. Note that this feature has been
           automatically enabled for --pairs in version 2.37, but due to
           compatibility issues, now it’s necessary to request this
           behavior by --shell.

       -z, --zoned
           Print the zone related information for each device.

       --sysroot directory
           Gather data for a Linux instance other than the instance from
           which the lsblk command is issued. The specified directory is
           the system root of the Linux instance to be inspected. The
           real device nodes in the target directory can be replaced by
           text files with udev attributes.

       --properties-by list
           This option specifies the methods used by lsblk to gather
           information about filesystems and partition tables. The list
           is a comma-separated list of method names. The default setting
           is "file,udev,blkid". The supported methods are:

           udev
               Reads data from udev DB. If unsuccessful, it continues to
               the next probing method.

           blkid
               Reads data directly from the device using libblkid. If
               unsuccessful, it continues to the next probing method.

           file
               Reads data from a file. This method is only used if the
               --sysroot option is specified. This method always stops
               probing if used.

           none
               Does not probe. This method always stops probing.

       --annotate[=when]
           Adds an annotation to each column header name. Such an
           annotation can be shown as a tooltip by terminals that support
           this feature. The optional when argument can be always, never,
           or auto. If the argument is omitted, it defaults to auto,
           which means that annotations will only be used when the output
           goes to a terminal.

       -h, --help
           Display help text and exit.

       -V, --version
           Display version and exit.

```

## https://man7.org/linux/man-pages/man8/lsblk.8.html#EXIT_STATUSEXIT STATUS [top](https://man7.org/linux/man-pages/man8/lsblk.8.html#top_of_page)

```

       0
           success

       1
           failure

       32
           none of specified devices found

       64
           some specified devices found, some not found

```

## https://man7.org/linux/man-pages/man8/lsblk.8.html#COLORSCOLORS [top](https://man7.org/linux/man-pages/man8/lsblk.8.html#top_of_page)

```

       The output colorization is implemented by [terminal-colors.d(5)](https://man7.org/linux/man-pages/man5/terminal-colors.d.5.html)
       functionality. Implicit coloring can be disabled by an empty file

          /etc/terminal-colors.d/lsblk.disable

       for the lsblk command or for all tools by

          /etc/terminal-colors.d/disable

       Since version 2.41, the $NO_COLOR environment variable is also
       supported to disable output colorization unless explicitly enabled
       by a command-line option.

       The user-specific $XDG_CONFIG_HOME/terminal-colors.d or
       $HOME/.config/terminal-colors.d overrides the global setting.

       Note that the output colorization may be enabled by default, and
       in this case terminal-colors.d directories do not have to exist
       yet.

```

## https://man7.org/linux/man-pages/man8/lsblk.8.html#ENVIRONMENTENVIRONMENT [top](https://man7.org/linux/man-pages/man8/lsblk.8.html#top_of_page)

```

       LIBBLKID_DEBUG=all
           Enable libblkid debug output.

       LIBMOUNT_DEBUG=all
           Enable libmount debug output.

       LIBSMARTCOLS_DEBUG=all
           Enable libsmartcols debug output.

       LIBSMARTCOLS_DEBUG_PADDING=on
           Use visible padding characters.

       LIBSMARTCOLS_JSON=compact|lines
           Control JSON output format when using --json. Supported values
           are compact for JSON output with minimal whitespace, and lines
           for JSON Lines format (one JSON object per line). If unset or
           set to any other value, pretty-printed JSON is used.

       LSBLK_COLUMNS=
           Specify a comma-separated list of output columns to print. All
           columns listed by --list-columns can be used.

       LSBLK_DEBUG=all
           Enable lsblk debug output.

       ULPATH_DEBUG=all
           Enable path debug output.

```

## https://man7.org/linux/man-pages/man8/lsblk.8.html#NOTESNOTES [top](https://man7.org/linux/man-pages/man8/lsblk.8.html#top_of_page)

```

       For partitions, some information (e.g., queue attributes) is
       inherited from the parent device.

       The lsblk command needs to be able to look up each block device by
       major:minor numbers, which is done by using /sys/dev/block. This
       sysfs block directory appeared in kernel 2.6.27 (October 2008). In
       case of problems with a new enough kernel, check that CONFIG_SYSFS
       was enabled at the time of the kernel build.

```

## https://man7.org/linux/man-pages/man8/lsblk.8.html#AUTHORSAUTHORS [top](https://man7.org/linux/man-pages/man8/lsblk.8.html#top_of_page)

```

       Milan Broz <gmazyland@gmail.com>, Karel Zak <kzak@redhat.com>

```

## https://man7.org/linux/man-pages/man8/lsblk.8.html#SEE_ALSOSEE ALSO [top](https://man7.org/linux/man-pages/man8/lsblk.8.html#top_of_page)

```

       [blkid(8)](https://man7.org/linux/man-pages/man8/blkid.8.html), [findmnt(8)](https://man7.org/linux/man-pages/man8/findmnt.8.html), [ls(1)](https://man7.org/linux/man-pages/man1/ls.1.html), [scols-filter(5)](https://man7.org/linux/man-pages/man5/scols-filter.5.html)

```

## https://man7.org/linux/man-pages/man8/lsblk.8.html#REPORTING_BUGSREPORTING BUGS [top](https://man7.org/linux/man-pages/man8/lsblk.8.html#top_of_page)

```

       For bug reports, use the issue tracker
       <https://github.com/util-linux/util-linux/issues>.

```

## https://man7.org/linux/man-pages/man8/lsblk.8.html#AVAILABILITYAVAILABILITY [top](https://man7.org/linux/man-pages/man8/lsblk.8.html#top_of_page)

```

       The lsblk command is part of the util-linux package which can be
       downloaded from Linux Kernel Archive
       <https://www.kernel.org/pub/linux/utils/util-linux/>. This page is
       part of the util-linux (a random collection of Linux utilities)
       project. Information about the project can be found at
       ⟨https://www.kernel.org/pub/linux/utils/util-linux/⟩. If you have a
       bug report for this manual page, send it to
       util-linux@vger.kernel.org. This page was obtained from the
       project's upstream Git repository
       ⟨git://git.kernel.org/pub/scm/utils/util-linux/util-linux.git⟩ on
       2026-08-04. (At that time, the date of the most recent commit that
       was found in the repository was 2026-08-03.) If you discover any
       rendering problems in this HTML version of the page, or you
       believe there is a better or more up-to-date source for the page,
       or you have corrections or improvements to the information in this
       COLOPHON (which is not part of the original manual page), send a
       mail to man-pages@man7.org

util-linux 2.43.devel-739-eee2e 2026-05-24                       LSBLK(8)

```

Pages that refer to this page: [eject(1)](https://man7.org/linux/man-pages/man1/eject.1.html), [mount(2)](https://man7.org/linux/man-pages/man2/mount.2.html), [fstab(5)](https://man7.org/linux/man-pages/man5/fstab.5.html), [blkdeactivate(8)](https://man7.org/linux/man-pages/man8/blkdeactivate.8.html), [blkid(8)](https://man7.org/linux/man-pages/man8/blkid.8.html), [cfdisk(8)](https://man7.org/linux/man-pages/man8/cfdisk.8.html), [findfs(8)](https://man7.org/linux/man-pages/man8/findfs.8.html), [mount(8)](https://man7.org/linux/man-pages/man8/mount.8.html)

|

HTML rendering created 2026-09-09 by [Michael Kerrisk](https://man7.org/mtk/index.html), author of [The Linux Programming Interface](https://man7.org/tlpi/).

For details of in-depth Linux/UNIX system programming training courses that I teach, look [here](https://man7.org/training/).

Hosting by [jambit GmbH](https://www.jambit.com/index_en.html).

|    |  https://man7.org/tlpi/
