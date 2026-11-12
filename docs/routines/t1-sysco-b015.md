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

- `sub_fe005194` in `6.5.9/usr/cpu/firmware/l2.bin(flash)` at 0xfe005194
  **Inferred.** The only rendered effect is the return at 0xfe0051e0, which leaves r3 as it was on entry.
  No loads, stores or calls are rendered in its 80 bytes, so it may be a pass-through or a function whose
  work is register-only and not shown by the facts. The role is not settled.

- `sub_fe048b88` in `6.5.9/usr/cpu/firmware/l2.bin(flash)` at 0xfe048b88
  **Inferred.** A byte-wise loop: a `bne` at 0xfe048bf8 back to 0xfe048bf0 and four `stb` stores to
  unresolved addresses, including 0xfe048c00, 0xfe048c08 and 0xfe048c0c. It returns an unresolved value.
  The shape suggests a byte fill or clear over a buffer. Its only known caller is sub_fe048c30 at
  0xfe048c6c, with r9=0xe00000, so the buffer is not identified.

- `sub_fe048c30` in `6.5.9/usr/cpu/firmware/l2.bin(flash)` at 0xfe048c30
  **Inferred.** A larger setup routine. It tests the global at 0xe05b18 at 0xfe048c68, calls sub_fe048b88
  at 0xfe048c6c, then runs `bdnz` loops at 0xfe048d1c and 0xfe048d94 that mix 16-bit and 32-bit loads and
  stores with unresolved addresses. Its data targets are not resolved, so the records it handles are not
  identified.

- `sub_c0008f80` in `6.5.9/usr/cpu/firmware/l2.bin(kernel)` at 0xc0008f80
  **Inferred.** A thin wrapper. It shifts a value right at 0xc0008f94 with `srwi`, calls 0xc00041fc at
  0xc0008f98, and returns that call's result at 0xc0008fa8. The shift amount is not in the facts, and
  the callee is not identified.

- `sub_c00bf1a8` in `6.5.9/usr/cpu/firmware/l2.bin(kernel)` at 0xc00bf1a8
  **Inferred.** Saves registers with `stmw` at 0xc00bf1b0, calls 0xc0009acc with r9=0xfa200000, r10=0x800,
  r6 and r8=0xc00f0000 and r3=0xc00dcc90, then calls 0xc000a5c0 with r4 loaded from 0xc00f6268 at
  0xc00bf238, and restores with `lmw` at 0xc00bf24c. The 0xfa200000 base and the 0x800 length suggest a
  device or buffer setup, but the callee's role is not known from these facts.

- `sub_c00bf258` in `6.5.9/usr/cpu/firmware/l2.bin(kernel)` at 0xc00bf258
  **Inferred.** Stores twice to abs 0xfa200940 and calls 0xc0003db8 at 0xc00bf29c with r3=5 and
  r4=0xc00bf2e8, which is the address of sub_c00bf2e8, followed by a branch on the return value at
  0xc00bf2a4. This reads as registering sub_c00bf2e8 as a handler under id 5 through 0xc0003db8. That
  identity of 0xc0003db8 is inferred from the call shape, not confirmed.

- `sub_c00bf2e8` in `6.5.9/usr/cpu/firmware/l2.bin(kernel)` at 0xc00bf2e8
  **Inferred.** A dispatcher. It loads two table entries with `lwzx` at 0xc00bf318 and 0xc00bf328, checks
  one against zero at 0xc00bf320, and calls through a register at 0xc00bf330 whose target is unknown. It
  is the address that sub_c00bf258 passes to 0xc0003db8, so it is probably the handler that code path
  dispatches to. The table contents are not resolved.

- `sub_c00c1d48` in `6.5.9/usr/cpu/firmware/l2.bin(kernel)` at 0xc00c1d48
  **Inferred.** Saves with `stmw` at 0xc00c1d50 and copies or converts 16-bit fields with `lhz` and `sth`
  (for example at 0xc00c1da4 and 0xc00c1dac), then calls 0xc0010f08 with r3=0xc00d9264 and r9=0xc00f0000
  at 0xc00c1d68. The callee 0xc0010f08 is also called from several neighbours with pointers in the
  0xc00d93xx and 0xc00d94xx data range, which suggests a message-printing helper. That helper identity is
  not confirmed.

- `sub_c00c222c` in `6.5.9/usr/cpu/firmware/l2.bin(kernel)` at 0xc00c222c
  **Inferred.** A large routine of about 650 bytes. It reads a table bound at 0xc00eaddc, compares an index
  against 0x13 at 0xc00c2268, and calls 0xc0010f08 several times with data-range pointers. It brackets
  work with `mfmsr` at 0xc00c2404 and `mtmsr` at 0xc00c2498, which suggests a machine-state change around
  a region. The purpose of the region is not identified.

- `sub_c00c24bc` in `6.5.9/usr/cpu/firmware/l2.bin(kernel)` at 0xc00c24bc
  **Inferred.** Another large routine of about 570 bytes. It tests the flag word at 0xfa200e44 at
  0xc00c2508, calls 0xc0010f08 with pointers 0xc00d93fc, 0xc00d9424 and 0xc00d9434, calls sub_c00c2900 at
  0xc00c26c8, and calls 0xc0006fe8 with r3=9 and r4=0xc00eae88. The sequence looks like a status or
  diagnostic pass. Its exact purpose is not settled.

- `sub_c00c2900` in `6.5.9/usr/cpu/firmware/l2.bin(kernel)` at 0xc00c2900
  **Inferred.** Follows the global pointer at 0xc00f6288. If the pointer is set and its slots at +4 and +8
  are non-null (the branches at 0xc00c2948 and 0xc00c2974), it calls through the pointer stored at
  0xfa200e80 at 0xc00c2954. It also calls 0xc0010f08 with r3=0xc00d94a0. This looks like an optional
  handler table. The meanings of the slots are not known.

- `sub_c00c2994` in `6.5.9/usr/cpu/firmware/l2.bin(kernel)` at 0xc00c2994
  **Inferred.** Saves the machine state with `mfmsr` at 0xc00c29b0 and restores it with `mtmsr` at
  0xc00c2a48, which suggests a critical section. It stores twice to abs 0xc00f628c and tests the globals
  at 0xc00f6284, 0xc00f6288 and 0xc00f6034 along the way. It is called by sub_c00c3094 at 0xc00c32b8 with
  r3=0x607e0000 and r4=0xc00c2a68. The exact protected operation is not identified.

- `sub_c00c3094` in `6.5.9/usr/cpu/firmware/l2.bin(kernel)` at 0xc00c3094
  **Inferred.** A large routine of about 800 bytes. Its `bdnz` loops at 0xc00c30dc and 0xc00c3148 fill
  records, `mulhwu` at 0xc00c30b4 scales a value, and `lhzu` loads at 0xc00c31f8 and 0xc00c3228 step through
  a table. It stores twice to 0xfa200e40 and calls sub_c00c2994 at 0xc00c32b8, guarded by the global at
  0xc00f6278. This reads as a table initialiser that runs inside a critical section. The table is not
  identified.

- `sub_c00eccf0` in `6.5.9/usr/cpu/firmware/l2.bin(kernel)` at 0xc00eccf0
  **Inferred.** Saves with `stmw` at 0xc00eccf8, performs `mullw`, `mulhw`, `srawi` and `divw` at
  0xc00ecd3c to 0xc00ecd70, and stores results to 0xfa200380 and to unresolved addresses. It calls
  0xc0010f08 with r3=0xc00c5ed0 and r5=0x3c. The divisor appears to be 60 from the call argument, so
  this may be a unit conversion, but the divisor operand is not shown and the result is not identified.

- `sub_c00ecdb4` in `6.5.9/usr/cpu/firmware/l2.bin(kernel)` at 0xc00ecdb4
  **Inferred.** Stores to the 0xfa200300, 0xfa200320 and 0xfa20030c registers, then calls 0xc0003db8 at
  0xc00ece20 with r3=0xf and r4=0xc0008de4, which is a handler address, and branches on the return at
  0xc00ece28. This looks like the same handler registration as sub_c00bf258, here under id 0xf. The
  identity of 0xc0003db8 is inferred from the call shape.

- `sub_ffe569f0` in `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe569f0
  **Inferred.** An m68k routine with `link.w` and six `movem.l` saves. It compares six longwords of its
  argument block at offsets +0x0 to +0x14 (for example `cmp.l` at 0xffe56a14). It updates counters at
  offsets +0x10, +0x14 and +0x18 of the value returned by the call at 0xffe586d0, and it reads and writes
  bytes at abs 0x200007f8, 0x200007fb, 0x200007fd, 0x200007fe and 0x200007ff. Of these, 0x200007f8 and 0x200007ff
  also appear in sub_ffec7534. The role is not settled.

- `sub_ffec7534` in `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffec7534
  **Inferred.** A small m68k routine. It sets bit 0x200007f8 with `bset.b` at 0xffec7546 and clears a bit
  in another byte with `bclr.b` at 0xffec7590. It branches on a value compared against 2, 3 and 4 between
  0xffec7628 and 0xffec7634, and returns 0 in d0. This looks like a flag set or clear with a three-way dispatch. The
  meaning of the flag bits is not known.

- `sub_ffe75984` in `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin(l1#1)` at 0xffe75984
  **Inferred.** Has the same instruction shape as sub_ffe569f0 at 0xffe569f0, with the same offsets
  and byte addresses. Its constants differ: it calls the value returned at 0xffe776c0 and reads abs
  0x30000d00 where sub_ffe569f0 reads 0x3000052c. This is probably a per-instance copy on another l1
  segment. The copy is inferred from shape, not verified.

- `sub_ffe7bca0` in `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin(l1#3)` at 0xffe7bca0
  **Inferred.** Same instruction shape as sub_ffe569f0 at 0xffe569f0 and sub_ffe75984 at 0xffe75984.
  It uses the value returned at 0xffe7fd3c and reads abs 0x30000d30. This is probably another
  per-instance copy on the l1#3 segment. The copy is inferred from shape, not verified.

- `sub_fe004efc` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** A near-sibling of sub_fe004ef0 in the 6.5.9 image. It makes the same run of calls to
  0xfe004610, with constants 0xe013ec, 0xe01418, 0xe01428, 0xe01454 and 0xe01474, and calls 0xfe004ff4 at
  0xfe004f38, branching on its result at 0xfe004f40. It also stores a word to 0xfa200004 at 0xfe004f84. The build difference is
  in the callee addresses and constants, so the message role is inferred from the shared shape.

- `sub_fe0051a0` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** A leaf routine with no calls. It stores halfwords and words to 0xfa200950 through
  0xfa200966 and to 0xfa200ab8, 0xfa200abc and 0xfa200ac2 (for example `sth` at 0xfe0051a8 and `stw` at
  0xfe0051bc), then returns. The stored values come from registers the facts do not show, so this may
  be a state-block initialiser for the 0xfa2009xx and 0xfa200axx registers.

- `sub_fe048ba0` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** Reads, then writes back, the words at 0xfa200abc and 0xfa200ab8 and the halfword at
  0xfa200ac2 (0xfe048ba8 to 0xfe048bd4). It stores to 0xfa203c80, 0xfa203c82, 0xfa203c84, 0xfa203c85,
  0xfa203c86 and 0xfa203cac, runs a byte-store loop with a `bne` at 0xfe048c18, and stores a word to the
  flag at abs 0x705cd8 at 0xfe048c44. It is called by sub_fe048c54 at 0xfe048c90. Its role is probably a
  copy of the 0xfa200abx state into the 0xfa203c8x block, but the mapping is not confirmed.

- `sub_fe048c54` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** A larger routine that tests the flag word at 0x705cd8 at 0xfe048c8c and calls
  sub_fe048ba0 at 0xfe048c90 with r9=0x700000. It then reads 0xfa203c80 and 0xfa203c82, runs an `lhzx`
  loop with a `bdnz` at 0xfe048d44, writes byte registers at 0xfa200860, 0xfa200870 and 0xfa20086c, and
  calls 0xfe044770 at 0xfe048e08. The routine looks like a guarded init that processes records, and the
  records are not identified.

- `sub_c000a5e0` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** Reads the word at abs 0xfa200284 at 0xc000a5f4, updates a byte through a pointer with
  `lbz` at 0xc000a60c and `stb` at 0xc000a614, and calls 0xc0004710 at 0xc000a5ec. It then calls
  0xc0010640 with r3=0xc00f0164 at 0xc000a61c, which suggests a short status report. The callees are not
  identified.
