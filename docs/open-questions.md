# Open questions

Each entry lists what has been tried and the next route. Move an answered question into the note that owns it and delete the entry.

## Other versions

- The request catalogue, I/O register table and protocol rows are read in L1 1.44.0, package 1 (Fuel/PE/O300 1 MB) only. Routes: locate the same dispatch tables in 1.24.8, 1.32.15 and 1.48.1 by their jump-table shape (the general-request table has entries for codes 0x0000 to 0x0011) and diff the handler set. Next: 1.48.1 package 1, whose load layout is identical (same length 0xefd8c).
- Which request codes does each non-Origin-300 ColdFire package add or drop? Route: the same table-shape search in packages 0, 2, 3 and 5.

## L1 hardware seen by the code

- Which MCF5206e on-chip peripherals besides the M-Bus does the code program: chip selects, DRAM controller, timers, the two on-chip UARTs? The notes cover the M-Bus (`MBAR+0x1e0` to `+0x1f0`) and the RAMBAR and MBAR writes only. Route: scan the first-bank listing for accesses relative to the MBAR base `0x10000000` and tabulate by offset.
- Which software UART devices (3 and 4 for the Origin 300 peer links, others for the node link) are on-chip UART channels, and at what offsets? Route: follow the UART open routine that the peer links call.
- What does the revision-0x81 USB callback (`0x300006f4`) point at? Route: find who writes that RAM word.
- The fitted USB device-controller part is not named by any image. The firmware probe is USS-820-shaped (revision at offset 0x18, scratch at 0x1b, accepted revisions 0x10, 0x11, 0x13); a full register table for that family would let the offset use be named. Unknown: no shipped image names the part.
- The M-Bus clock: the divider is 0x13 but the processor clock is not stated anywhere in the image. Unknown: no shipped image states it; routes tried: the timer setup and the version banner path.

## Images not yet read

- L1 packages 0, 2, 3 and 5 of every set (other brick families) and the two Linux packages: only their layout is tabulated.
- The L2 image: layout only (boot loader, kernel, cramfs); the daemon, libraries and the USB host side are unread. The 1.44.0 cramfs is at offset 0xd521d, size 2,052,096 bytes.
- `pbay.bin` (5,888 bytes, identical in all four sets): format not examined.
- The host tools' command tables (`l1cmd`, `l2cmd`, `flashsc` options and the update protocol they speak to the controllers).

## Media

- Other IRIX releases (6.5.20 to 6.5.29 overlay discs and customer diagnostics discs) may carry L1 and L2 versions between 1.24 and 1.44. Route: list `field_diags_sysco` on each disc and add the files to `root/` per IRIX_FILES.md.
