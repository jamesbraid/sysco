# L2 image: container, kernel and root file system

Read from the L2 image of IRIX 6.5.30 (`/usr/cpu/firmware/sysco/l2.bin`, firmware 1.44.0, 2,924,652 bytes, sha256 `6bcfda07a24cb4d56850922dab17a98223abcd14221d307e3a8c3778434e8761`). The other three sets (1.24.1, 1.32.4, 1.48.0) were carved with the same script and have the same layout; where a value differs between sets it says so. Grades are those of [../../AGENTS.md](../../AGENTS.md).

## Carving it

`retools/tools/sysco/l2_extract.py l2.bin OUTDIR [--list FILE]` needs only the Python standard library. It writes `OUTDIR/kernel.bin` (the inflated kernel) and `OUTDIR/rootfs/` (the file tree). `--list` writes one row per entry: path, kind, mode, size, sha256, link target or device numbers. The 1.44.0 listing is [rootfs-1.44.0.tsv](rootfs-1.44.0.tsv). `retools/tools/sysco/l2_list.py ELF` makes an annotated PowerPC listing of one user-space executable or library: it names PLT and text-relocation targets, resolves constants built with `lis` plus `addi`, and shows strings. It uses capstone and pyelftools from the toolchain image.

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

So the image is boot loader, kernel and cramfs in one blob that the header places at flash address `0xfe000000`. The 44-byte header is not part of what the flash holds: the body starts at file offset `0x2c`, so an address `A` in the flash window is at file offset `A - 0xfe000000 + 0x2c`. The file offsets in the table above are file offsets; the addresses below are flash addresses. The boot loader is read in the next section.

### The boot loader

Read from 1.44.0 (`l2.bin`, sha256 above) with capstone in PowerPC big-endian mode; the instruction streams below were read in full unless marked. The 860 internal-register offsets follow the public MPC860 user's manual names and are a reading of the offsets, not something the image states.

There are two loaders, one behind the other, each a 256 KiB slot:

- **Stage 1, the ROM (flash `0xfe000000`; file `0x2c` to `0x4002c`).** Its text at offset 8 is `SGI L2 Controller ROM 0.01`. It has a vector table of 31 stubs (one per `0x100`, offsets `0x100` to `0x1f00`) and its code runs from the reset vector, `0xfe000100`. The reset code writes the internal-memory-map base register (SPR `0x27e`) with `0xfa200000`, writes `0x707a0900` to offset 0 of that block (the SIU module configuration register), `0x62000000` to offset `0x280` and `0x5040` to offset `0x284` (clock control and PLL registers), loads three cache-control registers with the invalidate and disable commands, and branches to `0xfe0041cc`. That routine (a) programs the memory controller: boot chip select 0 gets base `0xfe000001` and option `0xff800940` (4 MiB window at `0xfe000000`, `0xfe004398`-`0xfe004398`), a 256-byte program is loaded into the memory-controller RAM and the machine-mode registers are set (`0xfe0042f0`-`0xfe0043a0`), (b) copies the `0x1f94` bytes of vectors and ROM head to RAM address `0x100` while XOR-summing them (`0xfe00441c`), (c) resets the SIU, enables the console UART block (`0xfe004460`-`0xfe0044fc`; a busy-wait helper at `0xfe004500` turns a millisecond count into time-base ticks assuming a 60 MHz time base, established as a constant, the clock itself inferred), then sets `r1` to `0x3f0000`, `r2` to `0xdeadbeef`, and jumps to `0xfe001fa0`. That stub copies its C-compiled code (`0xfe004550` to `0xfe005db0`) to RAM `0x00e00000`, clears `0x00e01860` to `0x00e078cc`, sets the stack at `0x00e078cc`+`0x6000`-`0x100` rounded to 16 and jumps to `main` at `0x00e009ac`.
- **Stage 2, the boot loader (flash `0xfe040000`; file `0x4002c` to `0x4a218`).** The word at `0xfe040000` is not code: it is the flash address of the image record (below). Offset 8 repeats the ROM string, and `SGI L2 Controller Boot Loader` is at `0xfe041fa0`. Its reset-style entry is `0xfe040100`, with the same register set-up as stage 1; its code differs from stage 1.

**What stage 1's `main` does (`0x00e009ac`).** It prints a banner and `Validating L2 Controller Flash image....` on the console UART, then validates the image at `0xfe040000`. The word at `0xfe040000` must be a word-aligned address inside the 4 MiB flash window (`0xfe000000` to `0xfe3fffff`), and so must the word it points to. For this image the first word is `0xfe2ca00c`, an eight-word record near the end of the file: type/length halfwords `3` and `0x20`, the image start `0xfe040000`, two copies of the size `0x28a02c`, pointers to the build stamp `20060717113711`, the version `1.44.0` and a four-word table, and a final word `0x0721df60`. The check adds the 32-bit words of the record (the type-and-length word included when the type is above 2, left out for types 1 and 2) and requires zero; this record's words sum to zero (computed). The same word-sum rule appears in the L1 image record ([../l1/flash-nvram-and-image-records.md](../l1/flash-nvram-and-image-records.md)), where it protects the loader's 48-byte record; here it protects only the record, not the image bytes it describes (the size words are not used by stage 1's checks that were read). On a bad pointer or a nonzero sum stage 1 prints the offending address or sum and returns -1. On success it prints `Booting...`, copies the first `0x1f94` bytes of the image to RAM address 0 (vector area), and calls `0xfe040100` through the link register (established: `0x00e00a48`-`0x00e00a90`). On failure it prints the recovery text (`BOOT FAILED`, `FATAL ERROR!!! Your L2 controller binary image is corrupted!!`, followed by the serial-port recovery procedure and the `l2recover` command naming `/usr/cpu/firmware/sysco/l2.bin`), writes `0x80` to word 1 of the internal-map base block and parks (`0x00e00a28`-`0x00e00a44`). The recovery text is the only place in the images that names `l2recover`.

**What stage 2 does.** Its reset code mirrors stage 1 (same SIU, clock and cache set-up), initialises the memory controller and vector area, then copies itself from `0xfe0444a0` (including the compressed kernel, up to flash `0xfe04a1ec`) to RAM `0x00700000`, clears `0x0070 5cb0` to `0x0070 efa8`, sets a stack, and calls a start-up routine and then a decompress routine (`0xfe0445f8`) with the loader's start (`0xfe041fa0`), its length in words and the board-information pointer it was given in `r11`. The strings are those of the public Linux/PowerPC "simple" boot wrapper: `Linux/PPC load:` followed by a command line (default `=/dev/nfs rw` appended to a stored line; the operator can edit the line at the console for a bounded time), `Uncompressing Linux...`, and `Now booting the kernel`, plus its zlib error texts, and it carries Motorola PCI/VPD helper strings that this board does not use. The decompress routine copies `0x1c` bytes of the board-information structure to a RAM buffer, reads the command line, calls an inflate routine as `gunzip(destination 0, limit 0x400000, source = the gzip stream, &length)` (`0xfe044f00`, a 4 MiB limit) and returns the buffer address. The start-up code then sets `r3` to that buffer address, `r4` and `r5` to two words of its state (the initial RAM-disk start and end, taken from the global words at `0x00705c88` and `0x00705c8c`), `r6` to the command-line start, `r7` to the address of its terminating zero, clears the link register and returns, which transfers control to address 0, where the inflated kernel sits (established for the register set-up and the jump: `0xfe0445fc`-`0xfe044640`). The gzip stream itself is at flash `0xfe04a1ec` (file `0x4a218`).

The kernel's first five instructions save `r3` to `r7` into `r31` to `r27` ([the kernel entry](#the-kernel)); with the register set-up above, `r3` is the board-information pointer, `r4` and `r5` the RAM-disk bounds and `r6`, `r7` the command-line bounds, which is the public Linux 2.4 PowerPC boot-parameter convention (inferred from that match; the kernel's use of the registers was not read).

**Unknown after this read.** The meaning of the stage-1 vector stubs beyond the first reset (inferred: exception stubs that jump through RAM), the origin of the `r11` board-information structure that stage 2 receives (it is set by stage 1 or by the reset code; a first read of `0xfe040100` onward did not show where `r11` is loaded), and what the stage-2 memory-controller program contains. Test: step the reset path of both stages in an emulator with the flash image mapped and watch the stores to the SIU registers and `r11`; or read stage 2's reset path one further level, which needs only the listings above.

### The kernel

The inflated image is a flat PowerPC binary, not an ELF. Its first word is `7c 7f 1b 78` (`mr r31,r3`), followed by moves of `r4` to `r7` into `r30` to `r27`; the loader's side of that hand-off is in [the boot loader](#the-boot-loader), and the kernel's own entry path is not read. The version banner, at inflated offset 0xedcd8, reads `Linux version 2.4.7-sgil2 ... gcc version 2.95.2 ... #1 Tue May 23 12:45:01 CDT 2006` in 1.44.0 (established). Strings in the kernel show which hardware it has drivers for (established as strings; that the hardware is present is inferred):

- MPC8xx on-chip support: `init_8xx_ports() for SGI L2`, `wdt_mpc8xx_init` (watchdog), `cpm_iic_*` with `i2c-algo-8xx.o` (an I2C adapter on the communications processor), `fec` with a PHY probe (the fast Ethernet controller), and the on-chip serial ports;
- `rtc.c:set_rtc_time` (a real-time clock driver);
- `lcd137x: Epson SED%s LCD controller detected` and `SGI L2 (SED137x LCD controller) frame buffer device` (the display);
- a USB host stack (`usb-ohci.c`, `hub.c`) and an SGI driver the controller firmware source unit 68 for L1 controllers, whose ioctl interface is read in [daemon.md](daemon.md);
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
