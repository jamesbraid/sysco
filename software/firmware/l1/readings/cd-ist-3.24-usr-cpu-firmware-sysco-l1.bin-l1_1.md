# Readings: `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin(l1#1)`

One entry per routine, filed here from the reading batches by binary.

- `sub_ffe75984` in `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin(l1#1)` at 0xffe75984
  **Inferred.** Has the same instruction sequence as sub_ffe569f0 at 0xffe569f0, differing only in constants and
  branch targets. It calls 0xffe774d0 and 0xffe776c0 where sub_ffe569f0 calls 0xffe584e0 and 0xffe586d0, reads abs
  0x30000d00 where sub_ffe569f0 reads 0x3000052c, and returns 0xfffff894 where it returns 0xfffff768 on a range
  failure. The byte addresses at 0x200007f8 to 0x200007ff and the field offsets are the same. This is probably a
  per-instance copy on another l1 segment. The copy is inferred from shape, not verified.
  Reviewed: 2026-11-12

- `sub_ffe75984` in `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin(l1#1)` at 0xffe75984
  **Inferred.** A ColdFire routine that reads fields at offsets 0x0 to 0x14 of the structure whose pointer is
  its first stack argument ([sp+4] at entry) and compares each against a limit: 0x258 (bgt at 0xffe7599e), 0x1e
  (bgt at 0xffe759ac, reached only when the +0x14 field is positive), 0xc (+0x10), 0x1f (+0xc), 0x18 (+0x8),
  0x3c (+0x4, blt at 0xffe759cc) and 0x3d (the word at +0x0, bge at 0xffe759d2). Any failed check goes to
  0xffe759d4, which loads d0 = -0x76c and returns through 0xffe75b9a. Otherwise it calls the routine at
  0xffe774d0 (jsr at 0xffe759e0, absolute address) with the structure pointer, calls 0xffe776c0 (jsr at
  0xffe759f0) with a pointer to the result, and calls 0xffe776c0 a second time (jsr at 0xffe75a0e) only when the
  word at +0x20 of the record returned by the first 0xffe776c0 call is nonzero. It then adds 0x76c to the +0x14
  field and increments the +0x10 and +0x18 fields of the record returned by 0xffe776c0, and writes bytes within
  abs 0x200007f8 to 0x200007ff. Role (from facts only): a range check on a record of seven compared words
  followed by callouts; the record's meaning is inferred from the limits only.
  Reviewed: 2026-11-12
