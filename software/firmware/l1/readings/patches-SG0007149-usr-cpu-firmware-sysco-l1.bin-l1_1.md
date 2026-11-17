# Readings: `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin(l1#1)`

One entry per routine, filed here from the reading batches by binary.

- `sub_ffe5e530` in `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin(l1#1)` at 0xffe5e530
  **Inferred.** The same coldfire validation and record update as sub_ffe5e878 in the l1#0 image, with the same field
  bounds at 0xffe5e54a to 0xffe5e57e. The jsr targets are direct addresses: it calls 0xffe60020, then 0xffe60210 (jsr
  at 0xffe5e59c), and a second 0xffe60210 only when +0x20 of the returned object is nonzero (beq at 0xffe5e5aa). It
  reads the global at 0x30000560 where the l1#0 copy reads 0x30000534, so its callee addresses and globals differ.
  Reviewed: 2026-11-12
