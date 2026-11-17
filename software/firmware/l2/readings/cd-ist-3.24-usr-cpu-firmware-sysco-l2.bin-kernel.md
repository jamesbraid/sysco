# Readings: `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(kernel)`

One entry per routine, filed here from the reading batches by binary.

- `sub_c000a5e0` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** Calls 0xc0004710 at 0xc000a5ec, then reads the word at 0xfa200284, ORs in 0x80 and writes it
  back (the store at 0xc000a600). It reads the machine state with `mfmsr` at 0xc000a5f8, clears one bit of it
  and writes it with `mtmsr` at 0xc000a608. It reads the byte at 0xfa20028c (`lbz` at 0xc000a60c), stores it to
  the stack frame at 0xc000a614, and calls 0xc0010640 with r3=0xc00f0164 at 0xc000a61c. It never returns: the
  `b` at 0xc000a620 branches to itself. This may be a fatal-error report, but the callees are not identified.
  Reviewed: 2026-11-12

- `sub_c00a2374` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** Sets up a 16-byte frame and stores five words into the block at 0xc01604e8 to 0xc01604f8
  (four stores at 0xc00a23a8 to 0xc00a23b4 and a fifth, to 0xc01604e8, at 0xc00a23d8). It calls the routine at
  0xc000b454 from 0xc00a23b8 with r5=0xfa200000 and r6=0xc0160000, and stores 1 to 0xfa200030 at 0xc00a2388.
  After the call it sets bit 1 in the word at the address the call returned in r3 (0xc00a23bc to 0xc00a23cc). The
  tlbia at 0xc00a23d0 is not modelled, so the TLB effect and the purpose of the store block are unclear.
  Reviewed: 2026-11-12

- `sub_c00a23ec` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** Sets up a frame and writes 0xfa200940 and 0xfa200948 (stores at 0xc00a2414, 0xc00a2418 and
  0xc00a2464). It calls 0xc0003300 with r3=5 and r4=0xc00a247c at 0xc00a2430, branches on that result at
  0xc00a2438, and calls the routine at 0xc00a250c at 0xc00a2458 before returning its r3. The shape looks like a
  small wrapper around a registration or lookup call; the callee at 0xc0003300 is not identified.
  Reviewed: 2026-11-12

- `sub_c00a247c` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** Sets up a frame, stores a halfword (1) to 0xfa200930 and reloads it at 0xc00a2494 and
  0xc00a2498. It shifts the reloaded value right by 11 and uses it times eight as an index into the table at
  0xc01989b0 (lwzx at 0xc00a24ac). When that table word is non-zero it loads the next word of the entry into r3
  (lwzx at 0xc00a24c0) and makes a register-indirect call through the first word at 0xc00a24c8; the target comes
  from a runtime table and is unresolved. When it is zero it instead ANDs a rotated mask into the word at
  0xfa200948 (lwz at 0xc00a24d0, store at 0xc00a24e0). Both paths end by storing a one-bit mask to 0xfa20094c at
  0xc00a24f0. The return value is unknown, and the instruction at 0xc00a2508 is not reached from the entry.
  Reviewed: 2026-11-12

- `sub_c00a250c` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** Saves registers with stmw under a 0x20-byte frame. When r4 is zero it calls 0xc00a2594 at
  0xc00a252c with r3 unchanged and returns. Otherwise it indexes the table at 0xc01989b0 by r3 times eight. When
  the table word is non-zero it calls 0xc0010640 with the constant address 0xc010a1d4 and r4 at 0xc00a2558. On
  this whole path it then stores the original third argument and r4 into the entry (0xc00a2560 and 0xc00a2564)
  and ORs a mask of 1 shifted left by r3 into the word at 0xfa200948 (lwz at 0xc00a256c, store at 0xc00a257c).
  The purpose beyond this call pattern is unclear.
  Reviewed: 2026-11-12

- `sub_c00a2594` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** A small routine with two indexed word stores at 0xc00a25a8 and 0xc00a25ac to unresolved
  addresses, a word load from 0xfa200948 at 0xc00a25b4, and a store at 0xc00a25c4. It returns an unresolved
  r3 at 0xc00a25c8. The rotate and and instructions at 0xc00a25a4 to 0xc00a25c0 are not modelled, so the
  exact computation is unknown.
  Reviewed: 2026-11-12

- `sub_c00a4e98` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** Saves registers under a 0x30-byte frame and reads and writes halfword fields through pointers
  loaded from r4 (lhz and sth between 0xc00a4eb8 and 0xc00a4f28; that stretch has no backward branch). When bit
  0x8000 of the halfword at the first pointer is set, it calls 0xc0010640 with the constant 0xc010a7dc at
  0xc00a4ecc, stores 0 to a record field and returns 1. Otherwise it calls 0xc0004868 at 0xc00a4f38, 0xc00046d4
  and then 0xc0004710 at 0xc00a4f40 and 0xc00a4f44, and 0xc00046e0 at 0xc00a4fc0, and returns 0. On that path an
  lwarx/stwcx. loop at 0xc00a4fac to 0xc00a4fb4 updates a word at an unresolved address (r4 plus 0x2c), and it is
  reached only when bit 0x8000 of the halfword then at r31 is set. The meaning of the reservation loop is
  unclear.
  Reviewed: 2026-11-12

- `sub_c00a515c` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** A large routine with a 0x40-byte frame. It loads the word at 0xfa200e44 (0xc00a5174) and returns
  when it is zero (branch at 0xc00a5188); it rereads the word at 0xc00a53ec and repeats while it is non-zero.
  Bits of that value choose the work. It calls 0xc0010640 with the constants 0xc010a894 (0xc00a51b4) and
  0xc010a8a4 (0xc00a527c), 0xc00a540c at 0xc00a51c4, and the routine at 0xc00a563c at 0xc00a53e8. Reservation
  loops (lwarx/stwcx. at 0xc00a52a8, 0xc00a534c and 0xc00a5368) update words: a decrement of the word at +0x70 of
  an entry, and a clear and a set of bits in the word at +0x2c of r4. It calls 0xc0013dec with r4=1 at 0xc00a52e0
  (when that decrement reaches zero) and at 0xc00a53a0, 0xc0013764 at 0xc00a53b0, and 0xc00046d4, 0xc0004710 and
  0xc00046e0 around the two 0xc0013dec calls. A halfword at +0x22 and a byte at +0x98 of the record are updated
  by ordinary stores. The control flow is dense and many tests have unresolved operands, so only the calls are
  listed here; the overall purpose is not settled.
  Reviewed: 2026-11-12

- `sub_c00a563c` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** Walks a small pointer structure rooted at the word 0xc0160508. It stores 0 to 0xfa200e84 at
  0xc00a5664 and loads the word at 0xfa200e80. When the root word is null it calls 0xc0010640 with the constant
  0xc010a910 at 0xc00a567c and returns. Otherwise it loads the word at +4 of the root record (0xc00a5684) and,
  when that is non-zero, calls through it with the value loaded from 0xfa200e80 as r3 (blrl at 0xc00a5698; the
  target is not resolved). It then moves the record's +8 link into 0xc0160508 and pushes the record onto the word
  at 0xc0160504 (stores at 0xc00a56a8 to 0xc00a56b4). When the new root is non-null it stores that record's
  first word to 0xfa200e80 and 0x14 to 0xfa200e84 (0xc00a56c4 and 0xc00a56c8). The routine at 0xc00a515c calls
  this one at 0xc00a53e8.
  Reviewed: 2026-11-12

- `sub_c00a56e0` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** Saves registers, calls 0xc00046d4 and then 0xc0004710 at 0xc00a56fc and 0xc00a5700, and loads
  the word at 0xc0160504. When that word is null it skips the stores and returns 1. Otherwise it takes the record
  it points to, stores the record's +8 link back to 0xc0160504 (0xc00a5728), and writes r4, r3 and 0 to the
  record's +4, +0 and +8 (0xc00a572c to 0xc00a5734). It then appends the record: when the word at 0xc0160508 is
  non-zero it links the record after the one at 0xc016050c (stores at 0xc00a5744 and 0xc00a5748); otherwise it
  stores the record to 0xc0160508 and 0xc016050c and writes 0x14 and r3 to 0xfa200e84 and 0xfa200e80 (0xc00a5758
  to 0xc00a576c). Every path calls 0xc00046e0 at 0xc00a577c, and the routine returns 0 after an insert. It uses
  the same 0xc0160504 and 0xc0160508 words as the routine at 0xc00a563c, and the routine at 0xc00a5eb0 calls it
  at 0xc00a60c0. It looks like a list insert or update helper, but the list layout is not settled.
  Reviewed: 2026-11-12

- `sub_c00a5e34` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5e34
  **Inferred.** Loads the halfword at r3 +0x54 [@0xc00a5e48] and branches on it at 0xc00a5e58. When that branch falls through [@0xc00a5e58 fall] it calls 0xc0010640 with r3=0xc010aa3c and r4=r3 [call 1, @0xc00a5e58 fall] and reads [@0xc00a5e68] and writes back [@0xc00a5e70] the word at abs 0xfa200f44. When it is taken [@0xc00a5e58 taken] it reads [@0xc00a5e78] and writes back [@0xc00a5e80] the same word, loads the halfword at r3 +0x54 again [@0xc00a5e84] and, when the branch at 0xc00a5e8c falls through [@0xc00a5e8c fall], stores to 0xfa200e08 and 0xfa200e0c [@0xc00a5e94] [@0xc00a5e98]. Role (from facts): updates the word at 0xfa200f44 according to a flag in the halfword at +0x54 of its argument record [@0xc00a5e48] [@0xc00a5e68] and, on one path, writes fixed words at 0xfa200e08 and 0xfa200e0c [@0xc00a5e94], with one call to 0xc0010640 [call 1].
  Reviewed: 2026-11-12

- `sub_c00a5eb0` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** Saves registers under a 0x20-byte frame, reads the word at 0xc0131278 (lwz at 0xc00a5ec8) and
  calls 0xc00046a0 at 0xc00a5ef0 with a value computed from it. It stores words at 0xfa200e40, 0xfa200e44,
  0xfa200e48 and 0xfa200e4c among others, and combines the halfwords at 0xc012eac8 and 0xc012eaca into the word
  at 0xfa200e00. A counted loop (bdnz at 0xc00a5f58) copies six bytes from 0xc012eac8 to offset 0x71 of its first
  argument. Two more counted loops (bdnz at 0xc00a6018 and 0xc00a6048) write eight halfword entries and eight
  halfword and word entries through the pointers at +0x24 and +0x28 of the record at +0x60 of the argument. A
  loop with no count walks eight slots of that record and calls 0xc00aa0a0 at 0xc00a5fec for a slot whose word at
  +0x70 is 1 or reaches zero. It also calls 0xc0004c2c at 0xc00a6094, and the routine at 0xc00a56e0 at
  0xc00a60c0 with r3=0x600a0000. The loop bounds and destinations are only partly resolved, so what the tables
  are for is unclear.
  Reviewed: 2026-11-12

- `sub_c01344f0` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01344f0
  **Inferred.** Sets up a 0x20-byte frame and reads the word at 0xc0160c7c (lwz at 0xc013450c), then writes
  0xfa200380 (stores at 0xc0134518 and 0xc0134520) and reads 0xfa200280 at 0xc0134524, writing it back with a
  bit set at 0xc013453c. It divides with divw at 0xc0134568 and 0xc0134578 (after a mulhw at 0xc0134554) on
  operands derived from the word at 0xc0160c7c; the routine has no loops. It calls 0xc0010640 with the constant
  0xc00f011c and r5=0x3c at 0xc0134548, 0xc0004630 at 0xc0134580, and 0xc0003300 with r3=0xf and r4=0xc000a584 at
  0xc01345e0. It calls 0xc0010094 at 0xc01345f4 only when that result is nonzero. The value the division
  computes is unknown.
  Reviewed: 2026-11-12

- `sub_c013d7e4` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc013d7e4
  **Inferred.** A large routine with a 0x60-byte frame and a 0x48-byte register save. It calls 0xc0013c38 at
  0xc013d800, loads the words at 0xc0131684 and 0xc0131688 and calls 0xc0010640 with the constant 0xc010a504 at
  0xc013d81c. It calls 0xc000cbe4 at 0xc013d82c with r3=0xc0198ab0 and r4=0xc4, and 0xc000c8d4 at 0xc013d888
  with r3=0xc0198acc, then stores 31 words and halfwords into the block at 0xc0198ab0 (offsets up to +0xb4, at
  0xc013d840 to 0xc013d9ac) and five into the block at 0xc0198b74. It calls 0xc000c8d4 again at 0xc013d9b4,
  0xc006b73c at 0xc013d9e4 with r3 pointing at the 0xc0198ab0 block and again at 0xc013da04 with r3 pointing at
  the 0xc0198b74 block, and 0xc0010094 after each when the result is nonzero. It loads the word at 0xc0160500.
  Near the middle and end it calls 0xc00a25cc and 0xc00a2600 (0xc013dc40 to 0xc013dd00), 0xc00a250c at
  0xc013df04 and 0xc00a2630 at 0xc013df1c. The purpose of the block fill is not settled.
  Reviewed: 2026-11-12
