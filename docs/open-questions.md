# Open questions

Each entry lists what has been tried and the next route. Move an answered question into the note that owns it and delete the entry.

## Other versions

- The I/O register table and protocol rows are read in L1 1.44.0, package 1 (Fuel/PE/O300 1 MB) only. The general request catalogue is compared across the sets in [version-diffs.md](version-diffs.md); the I/O and protocol rows are not. Route: run the same register scan over the 1.48.1 package, whose load layout is identical (same length 0xefd8c).
- Which request codes does each non-Origin-300 ColdFire package add or drop? Route: the same table-shape search in packages 0, 2, 3 and 5.

## L1 hardware seen by the code

- Which MCF5206e on-chip peripherals does the code program beyond those in [mcf5206e-peripherals.md](l1/mcf5206e-peripherals.md)? Timer registers and the remaining routines of the first-bank listing have not been scanned for base-relative accesses. Route: scan the listing for accesses relative to the base stored at `0x300002e8` and tabulate by offset.
- Which software UART devices (3 and 4 for the Origin 300 peer links, others for the node link) are on-chip UART channels, and at what offsets? Route: follow the UART open routine that the peer links call.
- What does the revision-0x81 USB callback (`0x300006f4`) point at? The only absolute-address use in the body is the load at `0xffe6dbce`, so any writer uses a base register and offset. Route: find stores into the record that holds it.
- The fitted USB device-controller part is not named by any image. The firmware probe is USS-820-shaped (revision at offset 0x18, scratch at 0x1b, accepted revisions 0x10, 0x11, 0x13); a full register table for that family would let the offset use be named. Unknown: no shipped image names the part.
- The M-Bus clock: the divider is 0x13 but the processor clock is not stated anywhere in the image. Unknown: no shipped image states it; routes tried: the timer setup and the version banner path.
- Which monitor part answers at M-Bus addresses `0x2c` to `0x2e`? The initialization routine (`0xffe6836c`) writes offsets `0x40`, `0x43`, `0x44` and `0x47`, which agree with the DS1780 register map. Routes tried: that routine. Next: tabulate every offset the monitor readers and the temperature path touch and compare with the DS1780 and LM78 maps; look for reads of `0x3e` and `0x3f` in this image and in the IRIX-side L1 host objects.
- Do callers of the chunked EEPROM reader (`0xffe6b03c`) retry a failed read? Routes tried: the reader and the M-Bus layer (neither retries). Next: the error branches of the reader's callers at `0xffe1686a`, `0xffe39614` and the record readers.
- What does the routine at `0xffe48370` (stored in the pointer at `0x300002f4`, which the MAC EEPROM command calls) do, and does it re-read the IO8 record? Next: read `0xffe48370` and the callers of the per-entry loader at `0xffe489be` to `0xffe48b5a`.
- Which routine stores the guard word at `0x30004d88` besides the peer initializer at `0xffe2dc3a`? Next: the other users of the record pointer `0x30004d70`, and the network mode setting at `0xffe53400`.
- What does the MCF5206E reset-status test at `0xffe00ac8` (offset `0x40`, bit 5 of the first argument `0x10000000`) select, and how is MBAR reachable there? Next: the public MCF5206E reset-state description and MAME's MCF5206E core.

## Images not yet read

- L1 packages 0, 2, 3 and 5 of every set (other brick families) and the two Linux packages: only their layout is tabulated.
- The L2 image: container, cramfs, file inventory and boot sequence are read ([l2/](l2/image-layout.md)). Unread: the boot loader (bytes 0x34 to 0x4a218 of `l2.bin`), the kernel's entry path and board setup, the L2 daemon's request handlers outside the partition task, `libscan`, the display and touch protocol in `l2gui`. The cramfs CRC field is not reproduced (routes tried: four spans, see the layout note).
- `pbay.bin` (5,888 bytes, identical in all four sets): format not examined.
- The host tools' command tables (`l1cmd`, `l2cmd`, `flashsc` options and the update protocol they speak to the controllers).

## Media

- Other IRIX releases (6.5.20 to 6.5.29 overlay discs and customer diagnostics discs) may carry L1 and L2 versions between 1.24 and 1.44. Route: list `field_diags_sysco` on each disc and add the files to `root/` per IRIX_FILES.md.
