# Open questions

Each entry lists what has been tried and the next route. Move an answered question into the note that owns it and delete the entry.

## Other versions

- The I/O register table and protocol rows are read in L1 1.44.0, package 1 (Fuel/PE/O300 1 MB) only. The general request catalogue is compared across the sets in [version-diffs.md](version-diffs.md); the I/O and protocol rows are not. Route: run the same register scan over the 1.48.1 package, whose load layout is identical (same length 0xefd8c).
- Which request codes does each non-Origin-300 ColdFire package add or drop? Route: the same table-shape search in packages 0, 2, 3 and 5.

## L1 hardware seen by the code

- Which MCF5206e on-chip peripherals does the code program beyond those in [mcf5206e-peripherals.md](l1/mcf5206e-peripherals.md)? Timer registers and the remaining routines of the first-bank listing have not been scanned for base-relative accesses. Route: scan the listing for accesses relative to the base stored at `0x300002e8` and tabulate by offset. The chip selects, interrupt controls, watchdog, restart chain and quad UART are read in [l1/board-boot-contract.md](l1/board-boot-contract.md) and [l1/quad-uart-and-serial-links.md](l1/quad-uart-and-serial-links.md); left: the timer routine's remaining writes and the on-chip UART 1 driver for the letters that open it.
- The revision-0x81 USB callback resolves to an empty routine in package 1 ([l1/flash-nvram-and-image-records.md](l1/flash-nvram-and-image-records.md)); the other packages were not read. Route: read the pointer table at `0xffe7f0e4+0x6c0` in each package.
- The fitted USB device-controller part is not named by any image. The firmware probe is USS-820-shaped (revision at offset 0x18, scratch at 0x1b, accepted revisions 0x10, 0x11, 0x13); a full register table for that family would let the offset use be named. Unknown: no shipped image names the part.
- The M-Bus clock: the divider is 0x13 but the processor clock is not stated anywhere in the image. Unknown: no shipped image states it; routes tried: the timer setup and the version banner path.
- Which monitor part answers at M-Bus addresses `0x2c` to `0x2e`? The initialization routine (`0xffe6836c`) writes offsets `0x40`, `0x43`, `0x44` and `0x47`, which agree with the DS1780 register map. Routes tried: that routine. Next: tabulate every offset the monitor readers and the temperature path touch and compare with the DS1780 and LM78 maps; look for reads of `0x3e` and `0x3f` in this image and in the IRIX-side L1 host objects.
- Do callers of the chunked EEPROM reader (`0xffe6b03c`) retry a failed read? Routes tried: the reader and the M-Bus layer (neither retries). Next: the error branches of the reader's callers at `0xffe1686a`, `0xffe39614` and the record readers.
- What does the routine at `0xffe48370` (stored in the pointer at `0x300002f4`, which the MAC EEPROM command calls) do, and does it re-read the IO8 record? Next: read `0xffe48370` and the callers of the per-entry loader at `0xffe489be` to `0xffe48b5a`.
- Which routine stores the guard word at `0x30004d88` besides the peer initializer at `0xffe2dc3a`? Next: the other users of the record pointer `0x30004d70`, and the network mode setting at `0xffe53400`.
- What does the MCF5206E reset-status test at `0xffe00ac8` (offset `0x40`, bit 5 of the first argument `0x10000000`) select, and how is MBAR reachable there? Next: the public MCF5206E reset-state description and MAME's MCF5206E core.

## Images not yet read

- L1 packages 0, 2, 3 and 5: [l1/personalities.md](l1/personalities.md) compares the 1.44.0 ones with the Origin 300 package (layout, personality selection, request codes, module list). Not read: the board devices each module drives (I/O addresses, expected replies), the other sets' versions of these packages, and the added request handlers other than 0x0012.
- What is word 7 of the bank record (`0xffe7fd24` and its siblings in each package)? Tried: xor, byte sum, 16-bit and little-endian word sums, CRC-32, Adler-32 and two rotating sums over the pre-record bank, with and without the record header; none matches. Next: find the code that reads the record (the loader reads words 0 to 3 and the metadata fields) and see whether anything consumes word 7.
- Who writes persistent setting keys 9 (brick type) and 0x10 (sub-type)? Route: follow the setter reached from the debug console and from request codes in task 6.
- The L2 image: container, cramfs, file inventory and boot sequence are read ([l2/](l2/image-layout.md)). Unread: the boot loader (bytes 0x34 to 0x4a218 of `l2.bin`), the kernel's entry path and board setup, the L2 daemon's request handlers outside the partition task, `libscan`, the display and touch protocol in `l2gui`. The cramfs CRC field is not reproduced (routes tried: four spans, see the layout note).
- `pbay.bin` (5,888 bytes, identical in all four sets): format not examined.
- The host tools' command tables (`l1cmd`, `l2cmd`, `flashsc` options and the update protocol they speak to the controllers).

## Media

- Other IRIX releases (6.5.20 to 6.5.29 overlay discs and customer diagnostics discs) may carry L1 and L2 versions between 1.24 and 1.44. Route: list `field_diags_sysco` on each disc and add the files to `root/` per IRIX_FILES.md.
