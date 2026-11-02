---
url: "https://www.kernel.org/doc/html/latest/usb/authorization.html"
title: "Authorizing (or not) your USB devices to connect to the system — The Linux Kernel documentation"
captured: "2026-11-02T18:57:13Z"
via: "live"
sha256: "97de096106cab342806995cc69b01abbc471a0174a1e198afc8daa53fe47a133"
wayback: "https://web.archive.org/web/20260224190428/https://www.kernel.org/doc/html/latest/usb/authorization.html"
cited_in:
  - "docs/threats/08-desktop-and-sandboxing.md"
---

# Authorizing (or not) your USB devices to connect to the system — The Linux Kernel documentation

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www.kernel.org/doc/html/latest/usb/authorization.html>
> Snapshot: <https://web.archive.org/web/20260224190428/https://www.kernel.org/doc/html/latest/usb/authorization.html>

https://www.kernel.org/doc/html/latest/index.html

# [The Linux Kernel](https://www.kernel.org/doc/html/latest/index.html)

7.4.0-rc1

### Quick search

### Contents

Working with the development community

- [Development process](https://www.kernel.org/doc/html/latest/process/development-process.html)
- [Submitting patches](https://www.kernel.org/doc/html/latest/process/submitting-patches.html)
- [Code of conduct](https://www.kernel.org/doc/html/latest/process/code-of-conduct.html)
- [Maintainer handbook](https://www.kernel.org/doc/html/latest/maintainer/index.html)
- [All development-process docs](https://www.kernel.org/doc/html/latest/process/index.html)

Internal API manuals

- [Core API](https://www.kernel.org/doc/html/latest/core-api/index.html)
- [Driver APIs](https://www.kernel.org/doc/html/latest/driver-api/index.html)
- [Subsystems](https://www.kernel.org/doc/html/latest/subsystem-apis.html)

- [Core subsystems](https://www.kernel.org/doc/html/latest/subsystem-apis.html#core-subsystems)
- [Human interfaces](https://www.kernel.org/doc/html/latest/subsystem-apis.html#human-interfaces)
- [Networking interfaces](https://www.kernel.org/doc/html/latest/subsystem-apis.html#networking-interfaces)
- [Storage interfaces](https://www.kernel.org/doc/html/latest/subsystem-apis.html#storage-interfaces)
- [Other subsystems](https://www.kernel.org/doc/html/latest/subsystem-apis.html#other-subsystems)

- [Accounting](https://www.kernel.org/doc/html/latest/accounting/index.html)
- [CPUFreq - CPU frequency and voltage scaling code in the Linux(TM) kernel](https://www.kernel.org/doc/html/latest/cpu-freq/index.html)
- [EDAC Subsystem](https://www.kernel.org/doc/html/latest/edac/index.html)
- [FPGA](https://www.kernel.org/doc/html/latest/fpga/index.html)
- [I2C/SMBus Subsystem](https://www.kernel.org/doc/html/latest/i2c/index.html)
- [Industrial I/O](https://www.kernel.org/doc/html/latest/iio/index.html)
- [PCMCIA](https://www.kernel.org/doc/html/latest/pcmcia/index.html)
- [Serial Peripheral Interface (SPI)](https://www.kernel.org/doc/html/latest/spi/index.html)
- [1-Wire Subsystem](https://www.kernel.org/doc/html/latest/w1/index.html)
- [Watchdog Support](https://www.kernel.org/doc/html/latest/watchdog/index.html)
- [Virtualization Support](https://www.kernel.org/doc/html/latest/virt/index.html)
- [Hardware Monitoring](https://www.kernel.org/doc/html/latest/hwmon/index.html)
- [Compute Accelerators](https://www.kernel.org/doc/html/latest/accel/index.html)
- [Security Documentation](https://www.kernel.org/doc/html/latest/security/index.html)
- [Crypto API](https://www.kernel.org/doc/html/latest/crypto/index.html)
- [BPF Documentation](https://www.kernel.org/doc/html/latest/bpf/index.html)
- [USB support](https://www.kernel.org/doc/html/latest/usb/index.html)
- [PCI Bus Subsystem](https://www.kernel.org/doc/html/latest/PCI/index.html)
- [Assorted Miscellaneous Devices Documentation](https://www.kernel.org/doc/html/latest/misc-devices/index.html)
- [PECI Subsystem](https://www.kernel.org/doc/html/latest/peci/index.html)
- [WMI Subsystem](https://www.kernel.org/doc/html/latest/wmi/index.html)
- [TEE Subsystem](https://www.kernel.org/doc/html/latest/tee/index.html)

- [Locking](https://www.kernel.org/doc/html/latest/locking/index.html)

Development tools and processes

- [Licensing rules](https://www.kernel.org/doc/html/latest/process/license-rules.html)
- [Writing documentation](https://www.kernel.org/doc/html/latest/doc-guide/index.html)
- [Development tools](https://www.kernel.org/doc/html/latest/dev-tools/index.html)
- [Testing guide](https://www.kernel.org/doc/html/latest/dev-tools/testing-overview.html)
- [Hacking guide](https://www.kernel.org/doc/html/latest/kernel-hacking/index.html)
- [Tracing](https://www.kernel.org/doc/html/latest/trace/index.html)
- [Fault injection](https://www.kernel.org/doc/html/latest/fault-injection/index.html)
- [Livepatching](https://www.kernel.org/doc/html/latest/livepatch/index.html)
- [Rust](https://www.kernel.org/doc/html/latest/rust/index.html)

User-oriented documentation

- [Administration](https://www.kernel.org/doc/html/latest/admin-guide/index.html)
- [Build system](https://www.kernel.org/doc/html/latest/kbuild/index.html)
- [Reporting issues](https://www.kernel.org/doc/html/latest/admin-guide/reporting-issues.html)
- [Userspace tools](https://www.kernel.org/doc/html/latest/tools/index.html)
- [Userspace API](https://www.kernel.org/doc/html/latest/userspace-api/index.html)

Firmware-related documentation

- [Firmware](https://www.kernel.org/doc/html/latest/firmware-guide/index.html)
- [Firmware and Devicetree](https://www.kernel.org/doc/html/latest/devicetree/index.html)

Architecture-specific documentation

- [CPU architectures](https://www.kernel.org/doc/html/latest/arch/index.html)

Other documentation

- [Unsorted documentation](https://www.kernel.org/doc/html/latest/staging/index.html)

Translations

- [Translations](https://www.kernel.org/doc/html/latest/translations/index.html)

### This Page

- [Show Source](https://www.kernel.org/doc/html/latest/_sources/usb/authorization.rst.txt)

English

- [Chinese (Simplified)](https://www.kernel.org/doc/html/latest/translations/zh_CN/usb/authorization.html)

# Authorizing (or not) your USB devices to connect to the system[¶](https://www.kernel.org/doc/html/latest/usb/authorization.html#authorizing-or-not-your-usb-devices-to-connect-to-the-system)

Copyright (C) 2007 Inaky Perez-Gonzalez <inaky@linux.intel.com> Intel Corporation

This feature allows you to control if a USB device can be used (or not) in a system. This feature will allow you to implement a lock-down of USB devices, fully controlled by user space.

As of now, when a USB device is connected it is configured and its interfaces are immediately made available to the users. With this modification, only if root authorizes the device to be configured will then it be possible to use it.

## Usage[¶](https://www.kernel.org/doc/html/latest/usb/authorization.html#usage)

Authorize a device to connect:

```
$ echo 1 > /sys/bus/usb/devices/DEVICE/authorized

```

De-authorize a device:

```
$ echo 0 > /sys/bus/usb/devices/DEVICE/authorized

```

Set new devices connected to hostX to be deauthorized by default (ie: lock down):

```
$ echo 0 > /sys/bus/usb/devices/usbX/authorized_default

```

Remove the lock down:

```
$ echo 1 > /sys/bus/usb/devices/usbX/authorized_default

```

By default, all USB devices are authorized. Writing “2” to the authorized_default attribute causes the kernel to authorize by default only devices connected to internal USB ports.

### Example system lockdown (lame)[¶](https://www.kernel.org/doc/html/latest/usb/authorization.html#example-system-lockdown-lame)

Imagine you want to implement a lockdown so only devices of type XYZ can be connected (for example, it is a kiosk machine with a visible USB port):

```
boot up
rc.local ->

 for host in /sys/bus/usb/devices/usb*
 do
    echo 0 > $host/authorized_default
 done

```

Hookup an script to udev, for new USB devices:

```
if device_is_my_type $DEV
then
  echo 1 > $device_path/authorized
done

```

Now, device_is_my_type() is where the juice for a lockdown is. Just checking if the class, type and protocol match something is the worse security verification you can make (or the best, for someone willing to break it). If you need something secure, use crypto and Certificate Authentication or stuff like that. Something simple for an storage key could be:

```
function device_is_my_type()
{
  echo 1 > authorized          # temporarily authorize it
                               # FIXME: make sure none can mount it
  mount DEVICENODE /mntpoint
  sum=$(md5sum /mntpoint/.signature)
  if [ $sum = $(cat /etc/lockdown/keysum) ]
  then
       echo "We are good, connected"
       umount /mntpoint
       # Other stuff so others can use it
  else
       echo 0 > authorized
  fi
}

```

Of course, this is lame, you’d want to do a real certificate verification stuff with PKI, so you don’t depend on a shared secret, etc, but you get the idea. Anybody with access to a device gadget kit can fake descriptors and device info. Don’t trust that. You are welcome.

### Interface authorization[¶](https://www.kernel.org/doc/html/latest/usb/authorization.html#interface-authorization)

There is a similar approach to allow or deny specific USB interfaces. That allows to block only a subset of an USB device.

Authorize an interface:

```
$ echo 1 > /sys/bus/usb/devices/INTERFACE/authorized

```

Deauthorize an interface:

```
$ echo 0 > /sys/bus/usb/devices/INTERFACE/authorized

```

The default value for new interfaces on a particular USB bus can be changed, too.

Allow interfaces per default:

```
$ echo 1 > /sys/bus/usb/devices/usbX/interface_authorized_default

```

Deny interfaces per default:

```
$ echo 0 > /sys/bus/usb/devices/usbX/interface_authorized_default

```

Per default the interface_authorized_default bit is 1. So all interfaces would authorized per default.

Note:

If a deauthorized interface will be authorized so the driver probing must be triggered manually by writing INTERFACE to /sys/bus/usb/drivers_probe

For drivers that need multiple interfaces all needed interfaces should be authorized first. After that the drivers should be probed. This avoids side effects.

©The kernel development community. | Powered by [Sphinx 5.3.0](https://www.sphinx-doc.org/) & [Alabaster 0.7.16](https://alabaster.readthedocs.io) | [Page source](https://www.kernel.org/doc/html/latest/_sources/usb/authorization.rst.txt)
