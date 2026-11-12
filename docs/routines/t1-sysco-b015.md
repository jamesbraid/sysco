# Tier-1 routine readings, sysco batch 15

Role statements for targets 375 to 399 of the tier-1 list (located rows, hardware-only), written from
the rendered instruction facts only: registers, calls, constants, branch targets and memory
accesses. Each role is a reading of those facts, not a confirmed contract. All 25 are inferred. The
grader accepted every rendered entry, so none were rejected.

- `sub_fe004ef0` in `6.5.9/usr/cpu/firmware/l2.bin(flash)` at 0xfe004ef0
  **Inferred.** Standard 0x20-byte frame prologue, then a run of calls to 0xfe004604 with constant
  pointers 0xe013fc, 0xe01428, 0xe01438, 0xe01464 and 0xe01484, interleaved with calls to 0xfe004fe8 and
  0xfe004f8c. The result of 0xfe004fe8 selects a branch at 0xfe004f34, and a word is stored to 0xfa200004
  at 0xfe004f78. This looks like a boot-time sequence that reports messages and then sets a status
  register in the 0xfa2 page. The pointer arguments are not resolved, so the messages are not identified.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe005194` in `6.5.9/usr/cpu/firmware/l2.bin(flash)` at 0xfe005194
  **Inferred.** A leaf routine with no calls and no loads. It stores constants to the 0xfa200000 page:
  halfwords 0xff to 0xfa200952, 0 to 0xfa200950, 0xfa200954 and 0xfa200962, 0x704 to 0xfa200960, 0 to
  0xfa200964 and 0x700 to 0xfa200966, then words 0x30 to 0xfa200abc, 0x3030 to 0xfa200ab8 and 0x3000 to
  0xfa200ac4 and the halfword 0x30 to 0xfa200ac2, and returns at 0xfe0051e0. Every stored value is an
  immediate, so this may be a fixed reset or initial-state write for those registers. The role is not settled.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe048b88` in `6.5.9/usr/cpu/firmware/l2.bin(flash)` at 0xfe048b88
  **Inferred.** Sets bits 0x30 in the words at 0xfa200abc and 0xfa200ab8 and the halfword at 0xfa200ac2
  (read, OR, write back), then stores constants to 0xfa203c80 through 0xfa203c87 (halfwords 0x850 and 0x840,
  bytes 0x10 at 0xfa203c84 and 0xfa203c85, halfword 0x80). It writes the halfword 0x11 to 0xfa2009c0 and
  polls the same halfword in the loop at 0xfe048bf0 to 0xfe048bf8 until bit 0 reads clear. It then stores
  bytes to 0xfa200864, 0xfa200868, 0xfa200870 and 0xfa200874, writes 1 to the global at 0xe05b18 and 1 to
  0xfa200030, and returns. Its only known caller is sub_fe048c30, which calls it at 0xfe048c6c only when
  that global is zero, so it looks like a one-time setup. The role is not settled.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe048c30` in `6.5.9/usr/cpu/firmware/l2.bin(flash)` at 0xfe048c30
  **Inferred.** Saves its four arguments (r3 to r6) in r28 to r31. It calls sub_fe048b88 at 0xfe048c6c only
  when the global at 0xe05b18 is zero (the branch at 0xfe048c68). It then reads the halfwords at 0xfa203c80
  and 0xfa203c82, derives a 16-byte-aligned buffer address from 0xe06bac, and stores a descriptor made from its
  four arguments into that buffer and into two slots near 0xfa2009c0 chosen by those halfwords. Between the
  stores it writes bytes 1, 0xff and 0x81 to 0xfa200860, 0xfa200870 and 0xfa20086c. It runs two polling loops
  on a slot's halfword, each leaving when bit 0x8000 reads clear and otherwise bounded by a `bdnz` count of
  0x989680 at 0xfe048d1c and 0xfe048d94. Each loop is entered only if bit 0x8000 is already set. No
  error is reported when a loop runs out, and r3 is returned holding the first argument with bit 0 set (the
  `ori` at 0xfe048cf0). The role is not settled.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c0008f80` in `6.5.9/usr/cpu/firmware/l2.bin(kernel)` at 0xc0008f80
  **Inferred.** A wrapper that calls one routine. It loads the word at 0xfa20001c at 0xc0008f90, shifts it
  right by 26 (`srwi` at 0xc0008f94) into r4, calls 0xc00041fc at 0xc0008f98 with r3 unchanged, and returns
  that call's result at 0xc0008fa8. The callee is not identified.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00bf1a8` in `6.5.9/usr/cpu/firmware/l2.bin(kernel)` at 0xc00bf1a8
  **Inferred.** Saves registers with `stmw` at 0xc00bf1b0, calls 0xc0009acc with r9=0xfa200000, r10=0x800,
  r6 and r8=0xc00f0000 and r3=0xc00dcc90, then calls 0xc000a5c0 with r4 loaded from 0xc00f6268 at
  0xc00bf238, and restores with `lmw` at 0xc00bf24c. The 0xfa200000 base and the 0x800 length suggest a
  device or buffer setup, but the callee's role is not known from these facts.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00bf258` in `6.5.9/usr/cpu/firmware/l2.bin(kernel)` at 0xc00bf258
  **Inferred.** Stores twice to abs 0xfa200940 and calls 0xc0003db8 at 0xc00bf29c with r3=5 and
  r4=0xc00bf2e8, which is the address of sub_c00bf2e8, followed by a branch on the return value at
  0xc00bf2a4. This reads as registering sub_c00bf2e8 as a handler under id 5 through 0xc0003db8. That
  identity of 0xc0003db8 is inferred from the call shape, not confirmed.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00bf2e8` in `6.5.9/usr/cpu/firmware/l2.bin(kernel)` at 0xc00bf2e8
  **Inferred.** Writes 1 to the halfword at 0xfa200930, reads it back and shifts it right by 11 to get an
  index. It uses the index to load two words from the table at 0xc010798c (lwzx at 0xc00bf318 and 0xc00bf328:
  entry 8*index and the word at +4). If the first word is nonzero it calls through it with r3 set to the second
  word (the `blrl` at 0xc00bf330). If it is zero it instead clears bit `index` in the word at 0xfa200948.
  Either way it then sets bit `index` in the word at 0xfa20094c. It is the address that sub_c00bf258 passes to
  0xc0003db8, so it is probably the handler registered there. The table contents are not resolved.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00c1d48` in `6.5.9/usr/cpu/firmware/l2.bin(kernel)` at 0xc00c1d48
  **Inferred.** Saves r29 to r31 with `stmw` at 0xc00c1d50. It loads the pointer at 0xc00f6280 into r29, then
  calls 0xc0010f08 with r3=0xc00d9264 at 0xc00c1d68. It writes halfwords 0xff to 0xfa200952 and 0 to
  0xfa200950 and 0xfa200954. Using read, mask or OR and write back it clears bit fields in the words at
  r29+0xfc and r29+0x104, sets 0x3000 in the word at r29+0xf8, clears 0x3000 in the halfword at r29+0x102
  (`lhz` at 0xc00c1da4, `sth` at 0xc00c1dac) and adjusts the halfwords at 0xfa200960, 0xfa200962, 0xfa200964
  and 0xfa200966. It stores 0x1b120900 to r29+0x12c and returns. The callee 0xc0010f08 is also called from
  several neighbours with pointers in the 0xc00d93xx and 0xc00d94xx range, which suggests a message-printing
  helper. That identity is not confirmed.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00c222c` in `6.5.9/usr/cpu/firmware/l2.bin(kernel)` at 0xc00c222c
  **Inferred.** A large routine of about 650 bytes with its arguments in r3 and r4. If the word at r4+0x24 is
  nonzero it subtracts r4+0x48 from the global word at 0xc00eaddc and, when the difference is 0x13 or less
  (`bgt` at 0xc00c2268), returns 1. Otherwise it calls 0xc0010f08 several times with pointers in the
  0xc00d93xx data range, including two loops of 8 iterations that call it with the halfwords at 0 and 2 and the word at 4 of the record
  at [r29+0x28] and then [r29+0x24] (the pointer in r31 is not advanced inside either loop), and clears r4+0x24. If the word at r4+0x24 is zero it calls 0xc000712c on two buffers, printing and returning 1 when
  either call returns nonzero. Otherwise it fills a descriptor, calls 0xc00027b0 and 0xc0006930, ORs 0x9c00 into
  a halfword and stores 0x1000000 to 0xfa200e54. It saves the machine state with `mfmsr` at 0xc00c2404 and
  restores it with `mtmsr` at 0xc00c2498, or by calling 0xc000697c when the saved value has bit 0x8000 and
  the global at 0xc00f6034 is nonzero. The purpose is not identified.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00c24bc` in `6.5.9/usr/cpu/firmware/l2.bin(kernel)` at 0xc00c24bc
  **Inferred.** A large routine of about 570 bytes. It loads the flag word at 0xfa200e44 at 0xc00c2500 and
  loops while it is nonzero (the branch at 0xc00c2508 leaves when it is zero; the loop closes at 0xc00c26d4),
  writing the value back to 0xfa200e44 each time. Depending on bits of the flag it calls 0xc0010f08 with
  r3=0xc00d9424 (bits in 0xe0400000, the call at 0xc00c2528) and with r3=0xc00d93fc (when the word at r4+0x20 was already
  set at entry), calls 0xc00c26f4 (bits 0x03000000), walks 8-byte records counting status bits and calling
  0xc0010f08 with 0xc00d9434 when a descriptor has bit 0x8000 set (bits 0x0c000000), and calls sub_c00c2900 at
  0xc00c26c8 (bit 0x00800000). A call to 0xc0006fe8 with r3=9 and r4=0xc00eae88 at 0xc00c2690 is made only when
  the byte at offset 0x98 of the block at r4+0x5c and the word at r4+0x24 are both nonzero. Its exact purpose is not settled.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00c2900` in `6.5.9/usr/cpu/firmware/l2.bin(kernel)` at 0xc00c2900
  **Inferred.** Reads the global pointer at 0xc00f6288 and the word at 0xfa200e80, and keeps its first argument
  in r4. If the pointer is null it calls 0xc0010f08 with r3=0xc00d94a0 and returns. Otherwise, if the pointer's
  slot +4 is nonzero (the branch at 0xc00c2948) it calls through that slot at 0xc00c2954 with r3 set to the word
  read from 0xfa200e80. It then unlinks the node: the global at 0xc00f6288 takes the node's +8 link, the node
  goes onto the list at 0xc00f6284, and if the new head is nonnull (the branch at 0xc00c2974) the first word of
  that head is written to 0xfa200e80. This looks like a queue of callbacks. The meanings of the fields are not known.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00c2994` in `6.5.9/usr/cpu/firmware/l2.bin(kernel)` at 0xc00c2994
  **Inferred.** Saves the machine state with `mfmsr` at 0xc00c29b0 and calls 0xc0006930, which suggests a critical
  section. It restores the state at the end with `mtmsr` at 0xc00c2a48, or through 0xc000697c when the saved
  value has bit 0x8000 and the global at 0xc00f6034 is nonzero. If the list head at 0xc00f6284 is nonnull it
  takes a node from it, fills the node with r3, r4 and 0, and appends it to the list whose head is at
  0xc00f6288 and tail at 0xc00f628c (one store to 0xc00f628c on each of two exclusive paths; the empty-list
  path also writes r3 to 0xfa200e80) and returns 0. If the head at 0xc00f6284 is null it returns 1. It is called by
  sub_c00c3094 at 0xc00c32b8 with r3=0x607e0000 and r4=0xc00c2a68. The exact protected operation is not
  identified.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00c3094` in `6.5.9/usr/cpu/firmware/l2.bin(kernel)` at 0xc00c3094
  **Inferred.** A large routine of about 800 bytes. `mulhwu` at 0xc00c30b4 multiplies 0xa7bc by the global at
  0xc00eacf4 and takes the high word, which sets the count of the empty `bdnz` loop at 0xc00c30dc. That loop runs only when the product is nonzero and sits
  between stores of 5 and 4 to 0xfa200e40, and a store of 6 follows at 0xc00c32a0. It writes 0xffc0, 0xf800000 and 0x20000000 to
  the words at 0xfa200e44, 0xfa200e48 and 0xfa200e4c, writes 0x1fff to the halfword at 0xfa200972 and 0x1c58
  or 0x1fff to the one at 0xfa200970 depending on the result of 0xc0006cd4 (0x500 or less selects 0x1c58),
  builds the word at 0xfa200e00 and the halfword at 0xfa200e04 from halfwords at 0xc00eabd8, and the `bdnz` loop at 0xc00c3148 copies 6 bytes from 0xc00eabd8 into the control
  block at r3+0x68. It walks 8 words at r29, and for each nonzero one calls 0xc0006acc (then 0xc0066ce0 when that returns nonzero) and zeroes it, then writes 0x8000 into each of 8 descriptors and 0 into 8 others, re-reading the last of each
  (`lhzu` at 0xc00c31f8 and 0xc00c3228) to set bit 0x2000. The global at 0xc00f6278 only selects what is stored at
  0xfa200f44 and 0xfa200f84 (the branch at 0xc00c3240): 4 and 4 when it is nonzero, 6 and 0 when it is zero. It also calls 0xc0006cd4 and 0xc00c2c4c, then
  calls sub_c00c2994 at 0xc00c32b8 unconditionally. The role is not settled.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00eccf0` in `6.5.9/usr/cpu/firmware/l2.bin(kernel)` at 0xc00eccf0
  **Inferred.** Saves r29 to r31 with `stmw` at 0xc00eccf8. It stores the words 0xaa3355cc and 0x55ccaa33 to
  0xfa200380 and sets bit 0x02000000 in the word at 0xfa200280. It loads the word at offset 8 of the structure at
  the pointer in the global at 0xc00ead18 and multiplies it by 0x393870 (`mullw` at 0xc00ecd3c), calls 0xc0010f08 with
  r3=0xc00c5ed0, r4 the product and r5=0x3c, then scales the product with `mulhw` by 0x51eb851f, `srawi` by 5 and `divw`
  by 0x3c (the divisor is the `li r7, 0x3c` at 0xc00ecd68) for one result, and with `mulhw` by 0x431bde83 and
  `srawi` by 18 for the other. It stores the two results to the globals at 0xc00f6050 and 0xc00f6054 and the constant 0x3c to 0xc00f600c. This may be a
  unit conversion of a time value, but the units are not identified.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00ecdb4` in `6.5.9/usr/cpu/firmware/l2.bin(kernel)` at 0xc00ecdb4
  **Inferred.** Stores to the 0xfa200300, 0xfa200320 and 0xfa20030c registers, then calls 0xc0003db8 at
  0xc00ece20 with r3=0xf and r4=0xc0008de4, which is a handler address, and branches on the return at
  0xc00ece28. This looks like the same handler registration as sub_c00bf258, here under id 0xf. The
  identity of 0xc0003db8 is inferred from the call shape.
  Reviewed: 2026-11-12 (sonnet)

- `sub_ffe569f0` in `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe569f0
  **Inferred.** An m68k routine with `link.w` and one `movem.l` save of d3 to d7 and a5. It range-checks the six
  longwords of its argument block at offsets +0x0 to +0x14 (for example `cmp.l` at 0xffe56a14) and returns
  0xfffff768 when any is out of range. Otherwise it calls 0xffe584e0 and 0xffe586d0, subtracting the word at
  0x3000052c and calling 0xffe586d0 again when the longword at +0x20 of the returned structure is nonzero. It
  then adds 0x76c to +0x14, increments +0x10 and +0x18 of that structure, and read-modify-writes eight bytes at abs 0x200007f8 to
  0x200007ff, packing values derived from the argument fields (divided by 10, 100 and 1000, shifted left by 4)
  into them. Of these, 0x200007f8 and 0x200007ff also appear in sub_ffec7534. It returns
  0 on success. The role is not settled.
  Reviewed: 2026-11-12 (sonnet)

- `sub_ffec7534` in `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffec7534
  **Inferred.** A small m68k routine. It sets bit 6 of the byte at 0x200007f8 with `bset.b` at 0xffec7546 and
  clears the same bit with `bclr.b` at 0xffec7590, with reads of 0x200007f8 and 0x200007ff between them. It
  decodes the digit fields of those two bytes into a number in d7 and does arithmetic with the constants
  0x16d, 0x16e, 0x190, 0x64 and 4 (`muls.l`, `divs.l`). It compares d7 against 2, 3 and 4 between 0xffec7628 and 0xffec7634 to choose
  between adding 0xcb or 0xd2. It stores two results through the pointers in its two arguments and returns 0 in d0.
  The meaning of the fields is not known.
  Reviewed: 2026-11-12 (sonnet)

- `sub_ffe75984` in `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin(l1#1)` at 0xffe75984
  **Inferred.** Has the same instruction sequence as sub_ffe569f0 at 0xffe569f0, differing only in constants and
  branch targets. It calls 0xffe774d0 and 0xffe776c0 where sub_ffe569f0 calls 0xffe584e0 and 0xffe586d0, reads abs
  0x30000d00 where sub_ffe569f0 reads 0x3000052c, and returns 0xfffff894 where it returns 0xfffff768 on a range
  failure. The byte addresses at 0x200007f8 to 0x200007ff and the field offsets are the same. This is probably a
  per-instance copy on another l1 segment. The copy is inferred from shape, not verified.
  Reviewed: 2026-11-12 (sonnet)

- `sub_ffe7bca0` in `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin(l1#3)` at 0xffe7bca0
  **Inferred.** Has the same instruction sequence as sub_ffe569f0 at 0xffe569f0 and sub_ffe75984 at 0xffe75984,
  differing only in constants and branch targets. It calls 0xffe7fb4c and 0xffe7fd3c, reads abs 0x30000d30,
  and returns 0xfffff894 on a range failure, as sub_ffe75984 does. This is probably another per-instance copy on
  the l1#3 segment. The copy is inferred from shape, not verified.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe004efc` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** A near-sibling of sub_fe004ef0 in the 6.5.9 image. It makes the same run of calls to
  0xfe004610, with constants 0xe013ec, 0xe01418, 0xe01428, 0xe01454 and 0xe01474, and calls 0xfe004ff4 at
  0xfe004f38, branching on its result at 0xfe004f40. It also stores a word to 0xfa200004 at 0xfe004f84. The build difference is
  in the callee addresses and constants, so the message role is inferred from the shared shape.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe0051a0` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** A leaf routine with no calls and no loads. It stores constants to the 0xfa200000 page: halfwords
  0xff to 0xfa200952, 0 to 0xfa200950, 0xfa200954 and 0xfa200962, 0x704 to 0xfa200960, 0 to 0xfa200964 and
  0x700 to 0xfa200966, words 0x30 to 0xfa200abc, 0x3030 to 0xfa200ab8 and 0x3000 to 0xfa200ac4 and the
  halfword 0x30 to 0xfa200ac2 (for example `sth` at 0xfe0051a8 and `stw` at 0xfe0051bc), then returns. Every
  stored value is an immediate and the stores match those of sub_fe005194 in 6.5.9, so this may be a fixed
  initial-state write for those registers. The role is not settled.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe048ba0` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** Sets bits 0x30 in the words at 0xfa200abc and 0xfa200ab8 and the halfword at 0xfa200ac2 (read, OR,
  write back, 0xfe048ba8 to 0xfe048bd4). It stores constants to 0xfa203c80, 0xfa203c82, 0xfa203c84, 0xfa203c85,
  0xfa203c86 and 0xfa203cac, writes the halfword 0x11 to 0xfa2009c0 and polls that halfword in the loop at
  0xfe048c10 to 0xfe048c18 until bit 0 reads clear. It then stores bytes to 0xfa200860, 0xfa200864, 0xfa200868, 0xfa200870 and 0xfa200874, writes 1 to the flag word at abs 0x705cd8 at 0xfe048c44 and 1 to 0xfa200030, and returns.
  It is called by sub_fe048c54 at 0xfe048c90. It matches sub_fe048b88 in 6.5.9 except for the stored
  constants (0x100 not 0x80 at 0xfa203c86, 0x80 not 0x34 at 0xfa200864, 4 not 7 at 0xfa200868), the extra
  stores to 0xfa203cac and 0xfa200860, and the flag address. The role is not settled.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe048c54` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** Saves its four arguments (r3 to r6) in r28 to r31. It tests the flag word at 0x705cd8 at
  0xfe048c8c and calls sub_fe048ba0 at 0xfe048c90 only when the flag is zero. It then reads the halfwords at
  0xfa203c80 and 0xfa203c82, builds a buffer pointer from 0x706d6c rounded to 16 bytes and stores a
  descriptor made from its four arguments into it and into two slots near 0xfa2009c0 chosen by those halfwords. Around those stores it
  writes bytes 1, 0xff and 0x81 to 0xfa200860, 0xfa200870 and 0xfa20086c. It runs two polling loops on the
  halfword at slot r5 until bit 0x8000 clears, each bounded by a `bdnz` count of 0x989680 (the loop with the `bdnz`
  at 0xfe048d44 and the second at 0xfe048de4). On a timeout it calls 0xfe044770 at 0xfe048e08 with r3=0x705720 or
  0x705740 and returns -1. Otherwise it returns the halfword at r7+2. It resembles sub_fe048c30 in 6.5.9 but adds the timeout checks.
  The role is not settled.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c000a5e0` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** Calls 0xc0004710 at 0xc000a5ec, then reads the word at 0xfa200284, ORs in 0x80 and writes it
  back (the store at 0xc000a600). It reads the machine state with `mfmsr` at 0xc000a5f8, clears one bit of it
  and writes it with `mtmsr` at 0xc000a608. It reads the byte at 0xfa20028c (`lbz` at 0xc000a60c), stores it to
  the stack frame at 0xc000a614, and calls 0xc0010640 with r3=0xc00f0164 at 0xc000a61c. It never returns: the
  `b` at 0xc000a620 branches to itself. This may be a fatal-error report, but the callees are not identified.
  Reviewed: 2026-11-12 (sonnet)
