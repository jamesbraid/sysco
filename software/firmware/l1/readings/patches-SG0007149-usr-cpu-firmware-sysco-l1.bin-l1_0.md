# Readings: `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin(l1#0)`

One entry per routine, filed here from the reading batches by binary.

- `sub_ffe5e878` in `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5e878
  **Inferred.** A coldfire routine that validates the six 32-bit fields of the record its first argument points to. It
  rejects with the value 0xfffff768 when +0x14 is above 0x258 or (when positive) below 0x1e, +0x10 above 0xc, +0xc
  above 0x1f, +0x8 above 0x18, +0x4 above 0x3c or +0x0 above 0x3d (compares at 0xffe5e88c to 0xffe5e8c4), which has
  the shape of seconds, minutes, hours, day, month and year. Otherwise it calls 0xffe60368 with the record, then calls
  0xffe60558 with the address of the result (a direct jsr at 0xffe5e8e4); when the field +0x20 of the returned object
  is nonzero it subtracts the global at 0x30000534 and calls 0xffe60558 again (jsr at 0xffe5e902). It then adds 0x76c
  to +0x14 and increments +0x10 and +0x18 of the returned object, and rewrites the bytes at 0x200007f8 to 0x200007ff
  and returns 0. The meaning of the record is not in the facts.
  Reviewed: 2026-11-12
