# Readings: `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin(l1#0)`

One entry per routine, filed here from the reading batches by binary.

- `sub_ffe569f0` in `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe569f0
  **Inferred.** An m68k routine with `link.w` and one `movem.l` save of d3 to d7 and a5. It range-checks the six
  longwords of its argument block at offsets +0x0 to +0x14 (for example `cmp.l` at 0xffe56a14) and returns
  0xfffff768 when any is out of range. Otherwise it calls 0xffe584e0 and 0xffe586d0, subtracting the word at
  0x3000052c and calling 0xffe586d0 again when the longword at +0x20 of the returned structure is nonzero. It
  then adds 0x76c to +0x14, increments +0x10 and +0x18 of that structure, and read-modify-writes eight bytes at abs 0x200007f8 to
  0x200007ff, packing values derived from the argument fields (divided by 10, 100 and 1000, shifted left by 4)
  into them. Of these, 0x200007f8 and 0x200007ff also appear in sub_ffec7534. It returns
  0 on success. The role is not settled.
  Reviewed: 2026-11-12

- `sub_ffe569f0` in `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe569f0
  **Inferred.** Checks six fields of the record that its first stack argument points to against limits [@0xffe56a00]. The field at +0x14 fails when it is above 0x258 [@0xffe56a0a taken] or when it is above 0x0 and below 0x1e [@0xffe56a18 taken]. The fields at +0x10, +0xc, +0x8 and +0x4 fail when they are above 0xc, 0x1f, 0x18 and 0x3c [@0xffe56a20 taken] [@0xffe56a28 taken] [@0xffe56a30 taken] [@0xffe56a38 taken], and the field at +0x0 fails when it is above 0x3d [@0xffe56a3e fall]. Otherwise it calls 0xffe584e0 [call 1] and calls 0xffe586d0 [call 2], and calls 0xffe586d0 again when the word at +0x20 of the returned record is nonzero [call 3, @0xffe56a6a fall]. It adds to the fields at +0x14, +0x10 and +0x18 [@0xffe56a8a] [@0xffe56a8e] [@0xffe56a92] and writes the fields as decimal digit pairs into the bytes at 0x200007f8 to 0x200007ff [@0xffe56aa2] [@0xffe56bfe]. Role (from facts): validates a record of six checked fields and writes it, as digit pairs, to the byte registers at 0x200007f8 to 0x200007ff [@0xffe56a0a] [@0xffe56aa2] [@0xffe56bfe].
  Reviewed: 2026-11-12

- `sub_ffec7534` in `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffec7534
  **Inferred.** A small m68k routine. It sets bit 6 of the byte at 0x200007f8 with `bset.b` at 0xffec7546 and
  clears the same bit with `bclr.b` at 0xffec7590, with reads of 0x200007f8 and 0x200007ff between them. It
  decodes the digit fields of those two bytes into a number in d7 and does arithmetic with the constants
  0x16d, 0x16e, 0x190, 0x64 and 4 (`muls.l`, `divs.l`). It compares d7 against 2, 3 and 4 between 0xffec7628 and 0xffec7634 to choose
  between adding 0xcb or 0xd2. It stores two results through the pointers in its two arguments and returns 0 in d0.
  The meaning of the fields is not known.
  Reviewed: 2026-11-12

- `sub_ffec7534` in `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffec7534
  **Inferred.** A ColdFire routine with a link.w frame and movem saves at 0xffec753c. It sets bit 6 of the byte
  at abs 0x200007f8 (bset.b at 0xffec7546) and clears the same bit of the same byte (bclr.b at 0xffec7590); the
  byte at abs 0x200007ff is only read in between, not set or cleared. Branches at 0xffec7628 to 0xffec7634
  compare a selector with 0x2, 0x3 and 0x4, and it returns d0 = 0 at the single return at 0xffec7666. The
  meaning of the byte values is not established.
  Reviewed: 2026-11-12

- `sub_ffec7534` in `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffec7534
  **Inferred.** ColdFire L1 routine with a link frame. It sets bit 6 of the byte at 0x200007f8 (`bset.b` at 0xffec7546), reads that byte and the byte at 0x200007ff, and clears bit 6 again with `bclr.b` at 0xffec7590. Between the two it shifts and masks the bytes and scales them with `muls.l` by 0x3e8, 0x64 and 0xa. After the clear it does arithmetic with the constants 0x16d, 0x16e, 0x190, 0x64, 0x5b, 0x5c and 0x5e, with `divs.l` at 0xffec75e2, 0xffec75f2 and 0xffec761a. It compares a value against 2, 3 and 4 (`beq.b` at 0xffec7628 and 0xffec762e, `bne.b` at 0xffec7634) to add 0xcb or 0xd2 to a result, stores two results through the pointers at stack offsets +8 and +0xc (0xffec7650, 0xffec765a) and returns d0 = 0 at 0xffec7666. The facts do not name what the bytes or the computed values mean.
  Reviewed: 2026-11-12
