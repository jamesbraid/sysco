# L2 root file system, boot sequence and user space

Read from the cramfs of L2 1.44.0 (see [image-layout.md](image-layout.md); the complete listing is [rootfs-1.44.0.tsv](rootfs-1.44.0.tsv)). Established means read from the extracted files.

## What is in it

A small Linux 2.4.7 userland for a 32-bit PowerPC MPC8xx, built with GCC 2.95.2 and glibc 2.1.3 (the `.comment` sections and `libc-2.1.3.so` say so). Everything runs from the read-only cramfs; writable space is tmpfs created at boot.

| Group | Files | Notes |
|---|---|---|
| Libc | `/lib/ld-2.1.3.so` (`ld.so.1`), `libc-2.1.3.so`, `libcrypt`, `libdl`, `libnsl`, `libnss_db`, `libnss_files`, `libutil` 2.1.3, `libproc.so.2.0.0`, `libtermcap.so.2.0.8` | `libm` is absent |
| System tools | `/bin/init` (30,344 bytes), `bash` (`/bin/sh` is a link to it), `busybox` (230,920 bytes; 90 applet links such as `ls`, `cp`, `mount`, `ifconfig`, `route`), `halt` (`reboot` links to it), `shutdown`, `hwclock`, `date`, `ps`, `od`, `login`, `inetd`, `ftpd`, `telnetd`, `rshd`, `rlogind`, `rexecd`, `rcp`, `dhcpcd`, `pppd`, `ping`, `strace`, `gdbserver`, `ldd` | `/sbin`, `/usr/bin`, `/usr/sbin` all link to `/bin` |
| L2 programs | `/stand/sysco/bin/`: `l2` (the daemon, 248,344 bytes), `l2term`, `l2gui`, `l2part`, `partmgr`, `l2flash`, `flashraw`, `l2dhcps`, `l2passwd`, `l2staticip`, `fixtelnet` (a shell script) | `/bin/l2term` is a 678-byte shell script, not the program |
| L2 libraries | `/stand/sysco/lib/`: `libmain`, `libapi`, `libsys`, `libRouter`, `libIRouter`, `libpart`, `libdrivers`, each `lib<name>.ppclinux.so` | the daemon's `DT_RPATH` is `/stand/sysco/lib`; no `libscan` (see [image-layout.md](image-layout.md)) |
| Config | `/etc/inittab`, `rc.sysinit`, `fstab`, `inetd.conf`, `services`, `passwd` (root and ftp with empty passwords), `hosts.equiv` and `/.rhosts` (a lone `+`, i.e. any host), `motd` (`SGI L2 Controller`), `ppp/options.ttyS0`, `dhcpc/dhcpcd-eth0.exe`, `l2firmware_version` | |
| Other | `/opt/hardhat/devkit/ppc/8xx/powerpc-hardhat-linux` is a link to `/` (a vendor cross-toolchain path baked into the tree); `/rw`, `/mnt`, `/proc`, `/tmp`, `/var` are empty mount points | |

### Main L2 binaries (1.44.0)

All are `ET_EXEC` PowerPC executables, dynamically linked, interpreter `/lib/ld.so.1`; the libraries are `ET_DYN` with text relocations. Exports and imports are dynamic-symbol counts. The binaries are stripped; the dynamic symbol table is all the naming there is.

| File | Bytes | Exports | Imports | Needs (besides libc) |
|---|---|---|---|---|
| `bin/l2` | 248,344 | 13 | 180 | libmain, libapi, libRouter, libIRouter, libsys, libdrivers |
| `bin/l2part` | 16,176 | 4 | 31 | libpart plus the same set |
| `bin/partmgr` | 45,872 | 4 | 48 | the same set plus libpart |
| `bin/l2gui` | 525,588 | 62 | 84 | the same set (a bundled GUI toolkit statically linked in) |
| `bin/l2term`, `l2flash`, `l2dhcps`, `l2passwd` (also libcrypt), `l2staticip`, `flashraw` | 12,980 / 6,404 / 20,976 / 4,812 / 7,004 / 6,884 | 4 each | 12 to 38 | libscan (missing, see above) plus the same set |
| `lib/libmain` | 146,708 | 90 | 86 | NVRAM, system configuration (`L2Cfg*`, `SysCfg*`), the Ethernet server, flash-file handling |
| `lib/libapi` | 89,844 | 84 | 100 | the client API (`sn1SC*`) and the direct L1 link (`L1Direct*`, `l1d_*`) |
| `lib/libsys` | 108,268 | 78 | 46 | number and text conversion, logging, command parser, PPP framing, FRU/VPD records, brick-type names, packet statistics |
| `lib/libRouter` | 98,636 | 126 | 54 | the event loop (select based) and the socket router |
| `lib/libIRouter` | 40,084 | 38 | 48 | the packet layer on top of it (`IRouter*`) |
| `lib/libdrivers` | 14,036 | 16 | 12 | flash access |
| `lib/libpart` | 27,608 | 20 | 20 | the partition mode database |

## Boot sequence

Established from `/etc/inittab` and `/etc/rc.sysinit`.

1. The kernel runs `/bin/init`. `initdefault` is **runlevel 5**.
2. `rc.sysinit`: mount 300 KiB, 50 KiB and 150 KiB tmpfs on `/tmp`, `/var` and `/etc` (the read-only `/etc` is copied into the last one); `mount -a` (only `proc` and `usbdevfs` in `fstab`); `hwclock --hctosys`; `l2passwd` (applies a stored L2 password to the account files); `l2staticip` (exit 0: address taken from NVRAM or the kernel, 1: configuration error, 2: use DHCP); on 2, start `dhcpcd -t 15 -i SGI_L2_CONTROLLER` when the host id is still zero, then `l2dhcps`; bring up loopback; add `default` to `eth0`.
3. `inittab` entries: `inetd` (runlevels 2 to 5); `fixtelnet` (one throw-away telnet to local port 5000, a workaround for a kernel crash with several simultaneous sessions, by the script's own comment); `l2` respawned on levels 3 to 5 with the console as its standard streams; `l2flash --l2 localhost` on 4 and 5; `l2gui --l2 localhost` and `l2part --l2 localhost` on level 5 only.
4. `inetd.conf`: `ftp` on the usual port; `telnet` runs `telnetd -L /bin/l2term` (the L2 command prompt); service `sgil2` (port 5000 in `/etc/services`, comment "window into L2 controller") runs a plain `telnetd` (a shell). `/bin/l2term` refuses more than 5 concurrent sessions and then points at the `l2term` program and other L2s.

Consequence: at the default level 5 all of `l2`, `l2flash`, `l2gui` and `l2part` run. The partition database service (task 10) is `l2part`, so it exists only at level 5; see [partition-task.md](partition-task.md).

## Device nodes

Established from the listing (major,minor). 88 character and 12 block nodes plus one FIFO.

| Node | Number | Used by |
|---|---|---|
| `/dev/sgil1_0` to `sgil1_39` | 180,64 to 180,103 | the daemon, USB mode: one per L1 USB device, opened read-write |
| `/dev/sgil1_cs` | 180,104 (mode 444) | the daemon's USB control channel (open read-only) |
| `/dev/ttyS0` to `ttyS3` | 4,64 to 4,67 | `ttyS1` is `/dev/console` (a link); `ttyS0` the modem port (daemon strings); `ttyS3` is opened non-blocking by `l2gui` and given terminal attributes, inferred to be the touch panel since no string names it |
| `/dev/fb0` | 29,0 | `l2gui` (mmap) |
| `/dev/mem`, `/dev/kmem`, `/dev/port` | 1,1 / 1,2 / 1,4 | `libdrivers` flash access |
| `/dev/i2c-0` | 89,0 | kernel I2C adapter |
| `/dev/rtc`, `/dev/watchdog` | 10,135 / 10,130 | clock and watchdog |
| `/dev/mtd0` to `mtd3` | 31,0 to 31,3 (block) | present; the daemon does not open them |
| `/dev/ppp` | 108,0 | `pppd` on `ttyS0` (`options.ttyS0`: `silent noauth crtscts lock 115200`) |
| `/dev/ptyp0-f`, `ttyp0-f`, `loop0-7`, `null`, `zero`, `full`, `initctl` | standard | |

## Network and serial access summary

- Ethernet `eth0` (the on-chip fast Ethernet controller): DHCP or the NVRAM static address; telnet (L2 prompt), the `sgil2` shell window on 5000, ftp; the daemon's own TCP ports (see [daemon.md](daemon.md)).
- Serial: `ttyS1` console at 38400, `ttyS0` modem port at 38400, 19200 or 9600 selectable (strings in the daemon's `baud` command and session table), `ttyS3` (opened by `l2gui`; purpose inferred).
- Authentication: `passwd` has root and ftp with empty passwords, `hosts.equiv` and `.rhosts` trust everything. The L2's own access password is a separate NVRAM setting enforced by the daemon in the command sessions (see [daemon.md](daemon.md)).

## Differences between versions

From a byte comparison of the four carved trees: the libc, shell, inittab, inetd configuration, services and all system tools are identical in all four, except `busybox` (same size, different bytes) and `dhcpcd` (1.24.1 differs). `rc.sysinit` differs only in 1.48.0 (2,522 bytes against 2,327); that difference is not read. The sizes of the libraries and the daemon:

| File | 1.24.1 | 1.32.4 | 1.44.0 | 1.48.0 |
|---|---|---|---|---|
| `bin/l2` | 208,096 | 226,836 | 248,344 | 255,476 |
| `lib/libmain` | 129,464 | 135,300 | 146,708 | 147,424 |
| `lib/libapi` | 82,124 | 84,416 | 89,844 | 90,232 |
| `lib/libsys` | 58,872 | 132,912 | 108,268 | 100,820 |
| `lib/libRouter` | 106,872 | 94,640 | 98,636 | 98,912 |
| `lib/libIRouter` | 37,940 | 38,572 | 40,084 | 40,524 |
| `lib/libdrivers` | 13,920 | 14,036 | 14,036 | 14,036 |
| `lib/libpart` | 27,688 | 27,688 | 27,608 | 27,608 |

Between 1.44.0 and 1.48.0 these are byte-identical (same hash): `libdrivers`, `libpart`, `flashraw`, `l2dhcps`, `l2passwd`, `l2staticip`. Every other L2 program and library differs in every release. Nothing in this directory is claimed for versions other than 1.44.0 unless a line says so.
