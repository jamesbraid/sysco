# Readings: `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin(l1#3)`

One entry per routine, filed here from the reading batches by binary.

- `sub_ffe73e48` in `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin(l1#3)` at 0xffe73e48
  **Inferred.** The same coldfire record validation and update as sub_ffe5e878, with the same field bounds at
  0xffe73e62 to 0xffe73e96, but it rejects with 0xfffff894 rather than 0xfffff768. It calls 0xffe75938, then
  0xffe75b28, and a second 0xffe75b28 only when +0x20 of the returned object is nonzero (beq at 0xffe73ec2); the jsr
  targets are direct addresses. It reads the global at 0x30000cb8. The same instruction shape appears again at
  0xfff63e48 in this image.
  Reviewed: 2026-11-12

- `sub_fff63e48` in `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin(l1#3)` at 0xfff63e48
  **Inferred.** The same coldfire record validation as sub_ffe73e48, with the same field bounds at 0xfff63e62 to
  0xfff63e96 and the same 0xfffff894 rejection value. It calls 0xfff65938, then 0xfff65b28, and a second 0xfff65b28
  only when +0x20 of the returned object is nonzero (beq at 0xfff63ec2); the jsr targets are direct addresses. It
  reads the global at 0x30000cb8. The facts give no reason to read it as a different routine from the 0xffe73e48 copy.
  Reviewed: 2026-11-12
