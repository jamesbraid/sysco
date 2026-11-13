# Linux L1 packages (ARM S3C2410) in the 1.44.0 and 1.48.1 containers

Each seven-package L1 container of IRIX 6.5.30 (version 1.44.0) and patch SG0007149 (version 1.48.1) holds two ARM S3C2410 Linux packages. Their outer layout is in [package-container.md](package-container.md). This note lays out the kernel and filesystem of each, names the brick types and chassis each one can configure, and lists the user-space programs by name. It does not read the programs.

The file-by-file listing is [../inventory/arm-linux-files.tsv](../../../evidence/inventory/arm-linux-files.tsv). `retools/tools/sysco/sysco_tables.py` writes it, and the `detail` column of [../inventory/packages.tsv](../../../evidence/inventory/packages.tsv) carries the layout figures below.

Grades: **established** means measured from the package bytes or the decompressed kernel or filesystem. **inferred** means read from names or partial evidence, with the reasoning given. **unknown** means nothing read so far settles it. Hashes are SHA-256 of the exact bytes named.

## Packages

| Key | Label | Version | Package sha256 |
| --- | --- | --- | --- |
| `6.5.30:l1#4` | Legacy Linux L1 image | 1.44.0 | `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6` |
| `6.5.30:l1#6` | Linux L1 image | 1.44.0 | `e57ab8ecd48858e39ecdd0050a23e444a491c35e87f9d8ebb48881fc25229a06` |
| `patches/SG0007149:l1#4` | Legacy Linux L1 image | 1.48.1 | `6bcee9bbd522c9bbd33538db5ea8c2c36666f7bb22c049a7912a72bf2c6c06e2` |
| `patches/SG0007149:l1#6` | Linux L1 image | 1.48.1 | `333891e54763faf27b63031875afc189dcf6adda6e6999c6ad87f074a297e6a5` |

The legacy packages carry a Linux 2.4 kernel and the other two a Linux 2.6 kernel. Both packages of a container have the same package header layout: the `ESTFBINR` marker, a zero word at `0x08` (no load address) and a nonzero word at `0x0c`. The role of the word at `0x0c` and of the word at `0x24` (value 3) is unknown.

Every package is a gzip member holding the kernel, followed by a little-endian CramFS image. The gzip header names the member `kernel` in all four. The 2.6 kernels name the ARM920T core and the 2.4 kernels do not; all four name the Samsung S3C2410 family.

### 1. Legacy Linux L1, 1.44.0 (`6.5.30:l1#4`)

- Container offset `0x53d154`, length 3,817,724 bytes. Established.
- Gzip kernel at package offset `0x2802c` (container `0x565180`), 603,899 bytes, one byte before the filesystem. Header time 1149867015, June 9 2006. Established.
- Kernel: 1,353,548 bytes, sha256 `505b88668c62a285593b038f111a9af8ea2fd42b92d6336ad14e68c679ac82d5`. Release `2.4.25-vrs2-swl8c-sgil1`, built `Fri Jun 9 10:23:42 CDT 2006`, banner at decompressed offset `0x12513c`. The S3C2410 string occurs 23 times. Established.
- CramFS at package offset `0xbb728` (container `0x5f887c`), 3,047,424 bytes, sha256 `ab1721002c4ac750582e91b7b516b2e4a903f2ca373000b062bd2710c8faed35`. Superblock: signature `Compressed ROMFS`, stored CRC `0x45062a7e` (not recomputed here), 1,825 data blocks, 453 entries counting the root. Established.
- Entries: 105 regular files, 134 symlinks, 168 character devices, 13 block devices, 1 FIFO, 32 directories. Every block chain sums to its file size and decompresses. Established.
- Bytes after the CramFS: 2,516. Established.

### 2. Linux L1, 1.44.0 (`6.5.30:l1#6`)

- Container offset `0xa936a8`, length 4,899,060 bytes. Established.
- Gzip kernel at container `0xabb6d4`, 888,987 bytes, one byte before the filesystem. Established.
- Kernel: 1,821,072 bytes, sha256 `0d8ed66ca3df4f6115a3fd2f28bfa3d3b5b08adb099fb130dd96d16fd63f6bed`. Release `2.6.15.6-sgil1`, built `Fri Jun 9 10:36:26 CDT 2006` as a PREEMPT kernel, banner at decompressed offset `0x172dd0`. The ARM920T string occurs once and S3C2410 13 times. Established.
- CramFS at container `0xb94770`, 3,837,952 bytes, sha256 `1b19eda1b712926e1257dd0e2380d28027b7ae6ad8b4f402c7496e59525e459e`. Stored CRC `0x978e34be`, 2,231 data blocks, 529 entries counting the root. Established.
- Entries: 132 regular files, 139 symlinks, 209 character devices, 13 block devices, 2 FIFOs, 34 directories. Established.
- Bytes after the CramFS: 8,236. Established.

### 3. Legacy Linux L1, 1.48.1 (`patches/SG0007149:l1#4`)

- Same container offset and length as entry 1 (`0x53d154`, 3,817,724 bytes). Established.
- Gzip kernel 603,918 bytes with a two-byte gap before the filesystem. Header time 1168609936, January 12 2007. Established.
- Kernel: 1,353,548 bytes, sha256 `0f53c289549133ab5af457c6424ece4af6141556e981b705132baa6e09857fdb`. Same release string as entry 1, built `Fri Jan 12 07:44:03 CST 2007`, banner at the same decompressed offset `0x12513c`. The kernel bytes differ from entry 1 although the release matches. Established.
- CramFS at package offset `0xbb73c` (container `0x5f8890`), 3,047,424 bytes, sha256 `9cf43fad62a8be4d4b214a56a5b29de93487e39231a81e5cd1f95b684a3bd4e9`. Stored CRC `0x12cc94f8`, 1,825 data blocks, 453 entries counting the root. Established.
- Entries: the same type counts as entry 1. Established.
- Bytes after the CramFS: 2,496. Established.

### 4. Linux L1, 1.48.1 (`patches/SG0007149:l1#6`)

- Container offset `0xa90dfc`, length 4,948,212 bytes. Established.
- Gzip kernel at container `0xab8e28`, 889,559 bytes, one byte before the filesystem. Header time 1168610561, January 12 2007. Established.
- Kernel: 1,821,120 bytes, sha256 `a02e64d3827c716bc38cc7c26c8b32a3fff3150324d67b7dd1b60377bcd2139c`. Release `2.6.15.6-sgil1`, built `Fri Jan 12 07:55:44 CST 2007` as a PREEMPT kernel, banner at decompressed offset `0x173330`. ARM920T occurs once and S3C2410 13 times. Established.
- CramFS at container `0xb92100`, 3,883,008 bytes, sha256 `85e63d5f6e985fdd9df966e0e4fc420ee1f961f82bf1105f27eb350a39140d68`. Stored CRC `0xd048b4c4`, 2,256 data blocks, 537 entries counting the root. Established.
- Entries: 140 regular files, 139 symlinks, 209 character devices, 13 block devices, 2 FIFOs, 34 directories. Established.
- Bytes after the CramFS: 11,760. Established.

## Kernel and filesystem layout

Each CramFS root has the same top-level entries in all four packages: directories `bin`, `boot`, `dev`, `etc`, `lib`, `mnt`, `proc`, `sbin`, `stand`, `tmp`, `usr`, `var`, `web` and `work`, and two files, `.dummy` (empty) and `.rhosts` (45 bytes, mode 0600). Established from the listings. `/boot` holds `bootldr.bin`, 72,876 bytes in the legacy 1.44.0 and 1.48.1 packages and 76,080 bytes in the 2.6 pair. Established. `/work` is empty in the image; it is the flash partition the start script reads from.

The start script `/etc/rc.sysinit` mounts tmpfs on `/tmp`, `/var` and `/etc` and copies the read-only `/etc` into the writable tmpfs copy. `/etc/fstab` mounts three flash partitions as YAFFS: `/dev/mtdblock1` on `/boot/image_A`, `/dev/mtdblock2` on `/boot/image_B`, and `/dev/mtdblock3` on `/work`. `/work` holds the configuration overrides the start script reads. Established from the text of the files.

Both kernels carry YAFFS and MTD strings. The lists of kernel modules are not present: there is no module directory in either filesystem. Established from the listing; whether the kernels are monolithic is inferred from that.

## Bricks and chassis

The brick evidence is in three places: the L1 program `l1`, the shared library `libsys` (brick types) and `libmain` (brick connection labels), and the file names of the L1 I/O bitstreams in `/stand/sysco/bin`. A bitstream file name is a guess at the brick it serves, from the name alone. The bitstream contents carry no name; only the `lbi.bit` file has a Xilinx header, and it names its own design.

Brick labels found in the packages. The labels are the strings the brick-type table in `libsys` and the L1 program use; a bitstream column says whether a file of that name exists in the same package.

| Label or name in the image | Legacy 2.4 (both versions) | Linux 2.6 (both versions) | Grade |
| --- | --- | --- | --- |
| `C (IP35)`, `C (IP37)`, `C (IP45)`, `C (Twister)` | label; no bitstream | label; no bitstream | established as labels; whether these types need a bitstream is unknown |
| `C (IP41)` | label, bitstream `l1io_ip41` | label, no bitstream | established |
| `C (IP67)` | label, bitstream `l1io_ip67` | label, bitstream `l1io_ip67` | inferred from name match |
| `C (Vortix)` | label, bitstream `l1io_vortix` | label, bitstream `l1io_vortix` | inferred from name match |
| `Dorado` | label, bitstream `l1io_p0_dorado` | label, bitstreams `l1io_dorado` and `l1io_p0_dorado` | inferred from name match; the meaning of `p0` is unknown |
| `Gemini` | label, bitstream `l1io_gemini` | label, bitstream `l1io_gemini` | inferred from name match |
| `NL4R`, `NL4 Snooper` | labels, bitstream `l1io_nl4r` | labels, bitstream `l1io_nl4r` | inferred from name match for the router; the snooper label is established |
| `PA` | label, bitstream `l1io_pa` | label, bitstream `l1io_pa` | inferred from name match |
| `SA` | no label, bitstream `l1io_sa` | no label, bitstream `l1io_sa` | unknown: no brick label or family name found |
| `C (IP63, 1932 Chassis)`, `Daytona (1932 Chassis)` | labels, bitstream `l1io_1932` | no label | inferred from the chassis name |
| `C (IP69 HACK, 1932 Chassis)` | no label, bitstream `l1io_1932` | label, bitstream `l1io_1932` | inferred from the chassis name |
| `IP71` | no label | label, bitstream `l1io_ip71` | inferred from name match |
| `Quad Dense Router` | no label | label, bitstream `l1io_dense_rtr` | inferred from name match |
| `191010 Chassis` | no label | label, bitstream `l1io_191010` | inferred from name match |
| `1955 Chassis` | no label | label, bitstream `l1io_1955` | inferred from name match |

Both builds also carry the labels `PE`, `PX`, `IX`, `IA` and `P0 N` beside `PA`. Other labels in both builds name systems that have no bitstream. Their roles are unknown and they are not listed here.

Every package also carries the brick-count labels for classes C, G, I, N, P, PE, R and X, and the connection labels `C or I/O brick (422 connection)` and `C/R/G brick (USB connection)`. The meaning of the letters is inferred from the connection labels only.

Which bricks each image serves, inferred from the table:

- **Legacy 2.4 images (both versions):** the C-brick types IP35, IP37, IP41, IP45, IP67, Twister and Vortix; the chassis labelled with the 1932 name (IP63 or Daytona); Dorado, Gemini, the NL4R router and `PA`. Nothing for IP69, IP71, the dense router, or the 191010 and 1955 chassis.
- **Linux 2.6 images (both versions):** the C-brick types IP35, IP37, IP41, IP45, IP67, Twister and Vortix; the IP69 chassis labelled with the 1932 name; IP71; the Quad Dense Router; Dorado, Gemini, the NL4R router and `PA`; and the 191010 and 1955 chassis. The 2.6 images have no `C (IP63, ...)` label and no Daytona label.

The 2.6 L1 binaries also carry configuration-family prefixes with no bitstream and no brick label, among them `Abacus`, `IP73` and `rtr55`, which do not appear in `libsys`. Their roles are unknown and they are not listed here.

The 1.48.1 images carry the same brick labels and bitstream names as the 1.44.0 images of the same kernel.

## User-space programs

These are the names of the programs and libraries in each filesystem, with the layout they sit in. What they do is for the reading that comes later.

- **C library and loader:** glibc 2.3.5 (`ld-2.3.5.so`, `libc-2.3.5.so`, the threads library, `libdl`, `libnsl`, `libnss_files`, `libresolv`, `libcrypt`, `libutil`, `libthread_db`, `libSegFault`), and `libproc-3.2.6.so`, which the name ties to the procps package (inferred). Established by name and version string. The 2.6 images also carry ncurses 5.3 and `libcurses`; the legacy images do not. Established.
- **BusyBox:** one binary, `/bin/busybox`, banner `BusyBox v1.1.0` built `2006.05.23` in 1.44.0 and `2007.01.12` in 1.48.1. Established. Forty-five applets are links to it under `/bin`, and further links sit under `/usr/bin` and `/usr/sbin`. Established. Shells and login: `/bin/ash`, `/bin/sh`, `/bin/login`.
- **Other `/bin` and `/sbin` binaries** (same in all four except where marked): `catchsegv`, `dhcpcd`, `ldd`, `login`; `flasherase`, `flashload`, `halt`, `hotplug`, `init`, `ip_calc_bcast`, `mtd_debug`, `nandbadblk`, `nandyaffsfix`, `runlevel`, `shutdown`; `nandfix` and `nandfixall` (2.6 only); `gdbserver` (2.6 only, in `/bin`; the legacy images have it in `/usr/bin`). Established.
- **`/usr/bin` and `/usr/sbin` binaries** (19 files in each package): `ethtool`, `l2term`, `mpstat`, `ps`, `rcp`, `sadc`, `sar`, `strace`, `sysctl`, `tcpdump`, `vmstat`; `ftpd`, `inetd`, `pppd`, `rexecd`, and the remote-login, remote-shell and telnet servers (names in the listing). The 2.6 images add `ifplugd` in `/usr/sbin`; `gdbserver` is in `/usr/bin` in the legacy images. Established.
- **L1 controller programs** in `/stand/sysco/bin`, all four: `l1` (main program), `lbi.bit`, the I/O bitstreams listed above, and the helpers `clkset`, `i2cdetect`, `i2cdump`, `i2cset`, `i2cspeed`, `i2cswitch`, `jtagport`, `spiload`, `ulisten`, `uloop`, `watchdogd`. Established from the listing. The 2.6 images add `l1log`, `resetmon`, the `cfg_diff`, `cfg_dump` and `cfg_snap` helpers, and the scan configuration files under `/stand/sysco/cfg/scan`. Established.
- **L2 programs** in `/stand/sysco/bin`: `l2`, `l2dhcps`, `l2passwd`, `l2sim`, `l2staticip`, `l2term` and `webs` in all four, and `l2setmac`, `l2slit`, `l2tune` in the 2.6 images only. `/web` holds `home.asp` and a logo, next to `webs`. The name `webs` as a web server is inferred.
- **Diagnostic programs** added by the 1.48.1 2.6 images: `l1cmd`, `netwatch`, `rdmem`, `rdmem8`, `rdmem16`, `wrmem`, `wrmem8`, `wrmem16`. Established.
- **Shared libraries** for the L1 programs: `libsys.armlinux.so`, `libmain.armlinux.so`, `libapi.armlinux.so`, `libscan.armlinux.so`, `libRouter.armlinux.so`, `libIRouter.armlinux.so`. In the 2.6 pair the six libraries and the L1 program differ in size between 1.44.0 and 1.48.1 (`libsys` 94,468 and 69,844 bytes; `l1` 1,925,560 and 1,976,140 bytes). In the legacy pair only `l1` differs (1,363,912 and 1,363,924 bytes). Established.

Startup from `/etc/inittab` (text): `l1` runs on the console in runlevels 3 to 5, `l2` and `webs` in runlevels 4 and 5, and `watchdogd` in runlevels 1 to 5. The network daemons start in runlevels 2 to 5. These lines hold in all four packages. Established from the text.

The other inittab lines differ by package. The legacy images start neither `ifplugd` nor `resetmon`. The 2.6 images of both versions start `ifplugd` in runlevels 2 to 5, to watch the network link. The 1.44.0 2.6 image has the `resetmon` line commented out; the 1.48.1 2.6 image enables it in runlevels 1 to 5, runs `netwatch` through `/bin/sh` in runlevels 4 and 5, starts `l1` with `-bg`, and gives the login shell runlevels 1 to 5 (`sh:12345`; the other three use `sh:12`). Established from the text.

`/etc/rc.sysinit` is the same in the 1.44.0 and 1.48.1 2.6 packages. After mounting, it takes `/work/sysconfig` if present, sets the clock from the real-time clock, imports the boot environment with `l1 -env`, and runs `l2setmac` when `eth0` exists. Established from the text.

## Differences between the four packages

- Legacy to Linux 2.6, in both versions: `gdbserver` moves from `/usr/bin` to `/bin`, `/lib/libncurses` and `/lib/libcurses` appear, the `/stand/sysco/bin` set gains `l1log`, `l2slit`, `l2tune`, `l2setmac`, `resetmon` and the `cfg_*` helpers, the scan configuration directory appears, and the bitstream list gains `1955`, `191010`, `dense_rtr`, `dorado` and `ip71` while losing `ip41`.
- 1.44.0 to 1.48.1, Linux 2.6: adds `l1cmd`, `netwatch`, and the `rdmem` and `wrmem` families. The bitstream names do not change; the `dense_rtr` and `191010` bitstreams change size (221,853 to 224,751 bytes, and 107,584 to 107,653 bytes).
- 1.44.0 to 1.48.1, legacy: the file set is unchanged, and only `busybox`, `l1` and `l2` differ in bytes. The libraries, bitstreams and scripts are identical. Established from the hash of every file in the two packages.
- 1.44.0 to 1.48.1, Linux 2.6, bytes: 25 files differ, the `/bin/busybox`, `/boot/bootldr.bin`, `/etc/ifcfg-eth0`, `/etc/inittab`, `l1`, `l2`, `webs`, `l2slit`, `l2term`, `l2tune`, `uloop`, `ulisten`, `resetmon`, the three `cfg_*` helpers, `/usr/bin/strace`, the `dense_rtr` and `191010` bitstreams, and the six shared libraries. Established.

## Open items

- The meaning of the header words at `0x0c` and `0x24`, and of the stored CRC beyond its value: unknown. Routes tried: the header bytes compared across the four packages, and the filesystem layout. Not tried: recomputing the CRC.
- `SA` and `lbi.bit`: unknown. Routes tried: brick labels and family prefixes in `l1` and `libsys`; the `lbi.bit` header. Not tried: reading the program that loads bitstreams.
- Which brick types need no bitstream (`IP35`, `IP37`, `IP45`, `Twister`): unknown. The program has an error string for a brick type with no bitstream, so the question is open rather than settled.
- The scan configuration name tokens (`32` to `512` with a suffix, and `32bb`): unknown.
- Every bitstream-to-brick mapping above is by name. The program's table that binds each brick type to a bitstream path would confirm or correct it. That is program reading, deferred.
- Which chassis the 1932 bitstream serves in the legacy images: `C (IP63, 1932 Chassis)` or `Daytona (1932 Chassis)`. Both labels appear. Unknown.
