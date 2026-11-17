# Readings: `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin(l1#3)`

One entry per routine, filed here from the reading batches by binary.

- `sub_ffe7bca0` in `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin(l1#3)` at 0xffe7bca0
  **Inferred.** Has the same instruction sequence as sub_ffe569f0 at 0xffe569f0 and sub_ffe75984 at 0xffe75984,
  differing only in constants and branch targets. It calls 0xffe7fb4c and 0xffe7fd3c, reads abs 0x30000d30,
  and returns 0xfffff894 on a range failure, as sub_ffe75984 does. This is probably another per-instance copy on
  the l1#3 segment. The copy is inferred from shape, not verified.
  Reviewed: 2026-11-12

- `sub_ffe7bca0` in `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin(l1#3)` at 0xffe7bca0
  **Inferred.** The same frame and field-limit checks as the routine at 0xffe75984 in the l1#1 copy, down to the
  same limit constants at 0xffe7bcb4 to 0xffe7bcec (including the 0x3d check on the word at +0x0). It calls the
  routine at 0xffe7fb4c (jsr at 0xffe7bcfc, absolute address) and then 0xffe7fd3c (jsr at 0xffe7bd0c), with a
  second call to 0xffe7fd3c (jsr at 0xffe7bd2a) only when the word at +0x20 of the record returned by the first
  0xffe7fd3c call is nonzero, so it is a sibling of that routine with different callees. Its purpose beyond the
  range checks is inferred only from that shape.
  Reviewed: 2026-11-12
