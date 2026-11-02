# L2 image: container, kernel and root file system

Read from the L2 image of IRIX 6.5.30 (`/usr/cpu/firmware/sysco/l2.bin`, firmware 1.44.0, 2,924,652 bytes, sha256 `6bcfda07a24cb4d56850922dab17a98223abcd14221d307e3a8c3778434e8761`). The other three sets (1.24.1, 1.32.4, 1.48.0) were carved with the same script and have the same layout; where a value differs between sets it says so. Grades are those of [../../AGENTS.md](../../AGENTS.md).

## Carving it

`scripts/l2_extract.py l2.bin OUTDIR [--list FILE]` needs only the Python standard library. It writes `OUTDIR/kernel.bin` (the inflated kernel) and `OUTDIR/rootfs/` (the file tree). `--list` writes one row per entry: path, kind, mode, size, sha256, link target or device numbers. The 1.44.0 listing is [rootfs-1.44.0.tsv](rootfs-1.44.0.tsv). `scripts/l2_list.py ELF` makes an annotated PowerPC listing of one user-space executable or library: it names PLT and text-relocation targets, resolves constants built with `lis` plus `addi`, and shows strings. It uses capstone and pyelftools from the toolchain image.

## Layout of l2.bin (1.44.0)

| Offset | Contents | Grade |
|---|---|---|
| 0x00 | ASCII `ESTFBINR`, then big-endian words: 0x08 `0xfe000000` (load), 0x0c `0xfe2ca03f` (last), 0x10 `0xff000000`, 0x14 `0`, 0x18 and 0x1c repeat load and last, 0x20 `0xffffffff`, 0x24 `3` | established |
| 0x34 | string `SGI L2 Controller ROM 0.01` | established |
| 0x207c | string `SGI L2 Controller Boot Loader` | established |
| 0x4a218 | gzip stream, ends at 0xd501d, inflates to 1,430,148 bytes | established (inflated and measured) |
| 0xd501d | 512 zero-filled bytes, then the cramfs superblock at 0xd521d | established |
| 0x2ca01d | end of the cramfs (0xd501d plus the superblock's size field 2,052,096) | established |
| 0x2ca01d to end | 79-byte trailer: the build stamp `20060717113711`, the version string `1.44.0`, and a short table of big-endian words (`0xfe2c9ff4`, `0xfe2ca004`, `0xfe2ca030`, ...) | established as bytes; meaning of the words unknown |

So the image is boot loader, kernel and cramfs in one blob that the header places at flash address `0xfe000000`. The boot loader (bytes 0x34 to 0x4a218) is not read yet; it is listed in [../open-questions.md](../open-questions.md).

### The kernel

The inflated image is a flat PowerPC binary, not an ELF. Its first word is `7c 7f 1b 78` (`mr r31,r3`); the entry path is not read, so how the loader hands it boot information is unknown. The version banner, at inflated offset 0xedcd8, reads `Linux version 2.4.7-sgil2 ... gcc version 2.95.2 ... #1 Tue May 23 12:45:01 CDT 2006` in 1.44.0 (established). Strings in the kernel show which hardware it has drivers for (established as strings; that the hardware is present is inferred):

- MPC8xx on-chip support: `init_8xx_ports() for SGI L2`, `wdt_mpc8xx_init` (watchdog), `cpm_iic_*` with `i2c-algo-8xx.o` (an I2C adapter on the communications processor), `fec` with a PHY probe (the fast Ethernet controller), and the on-chip serial ports;
- `rtc.c:set_rtc_time` (a real-time clock driver);
- `lcd137x: Epson SED%s LCD controller detected` and `SGI L2 (SED137x LCD controller) frame buffer device` (the display);
- a USB host stack (`usb-ohci.c`, `hub.c`) and an SGI driver `sgil1.c` for L1 controllers, whose ioctl interface is read in [daemon.md](daemon.md);
- RAM-disk support for a cramfs root (`RAMDISK: Compressed image found`).

No flash-chip or MTD-map string was found, although the root has `/dev/mtd0` to `/dev/mtd3` nodes. The daemon libraries reach the flash through `/dev/mem`, not through those nodes (see [daemon.md](daemon.md)).

### The cramfs

Standard cramfs fields, big-endian (established from the bytes): magic `0x28cd3d45`, size field 2,052,096, flags 3 (version-2 fsid and sorted directories), signature `Compressed ROMFS`, 1,177 blocks, 308 files, name `Compressed`. Two properties matter to anyone writing a reader:

1. The superblock sits 512 bytes into the image, and every offset inside it, including the size field's span, counts from the start of that 512-byte pad rather than from the superblock. Reading offsets from the superblock finds the root directory entries 512 bytes too early.
2. The CRC field (`0x873f5a16`) is not reproduced by a CRC-32 over the image with the field zeroed, over four spans tried (pad included or not; the stated size or the remainder of the file). Extraction does not need it. Unresolved.

File data is one zlib stream per 4 KiB block, with a table of end offsets in front, as in the public cramfs layout. A device node keeps its device numbers in the size field.

Totals for 1.44.0, from the listing: 69 regular files, 116 symbolic links, 88 character device nodes, 12 block device nodes, 1 FIFO and 21 directories (the root counts as the 308th entry). The tree is described in [rootfs.md](rootfs.md).

## Versions

| Set | `/etc/l2firmware_version` | `libscan.ppclinux.so` in the tree |
|---|---|---|
| IRIX 6.5.22 | 1.24.1 | absent |
| patch SG0005762 | 1.32.4 | absent |
| IRIX 6.5.30 | 1.44.0 | absent |
| patch SG0007149 | 1.48.0 | present, 203,564 bytes |

In 1.44.0 the small tools (`l2term`, `l2flash`, `l2dhcps`, `l2passwd`, `l2staticip`, `flashraw`) list `libscan.ppclinux.so` among their needed libraries, yet no such file is in the 1.44.0 cramfs, so as shipped they cannot start on that image unless the loader finds the library elsewhere. The 1.24.1 and 1.32.4 builds do not need it. The 1.48.0 file exports 253 dynamic symbols, 210 functions (one weak) and 43 data objects; the functions are named like a JTAG scan master (`jmstr_scan`, `jmstr_sel_sic`, `jmstr_sel_sic_multi`, `scancmd_*`, a vector-file section reader `readsection`) with an `enode_*` error-record set. None of the 1.44.0 tools imports a symbol that only a scan library could supply, so the dependency looks vestigial there. Established: the needed-library lists, the absence, the export names. Inferred: the role (names only; no routine read) and the vestigial reading.

The same holds in 1.48.0, where the library is present. Its own needed list is `libc.so.6` alone. The six tools above list it, and none of them has an undefined dynamic symbol that matches any of its 253 exports; no other ELF file in the 1.48.0 tree (the daemon, the other libraries, the other tools) imports one either. So no program in the image calls into the library, and the needed entry is a link-line leftover there too (established, from every ELF file's dynamic symbol table and needed list in the carved tree). What the library is for stays inferred from its export names.
