# Tier-1 routine readings, sysco batch 16

Role statements for targets 400 to 424 of the ranked tier-1 list, written from the rendered
instruction facts only (registers, calls, constants and branch targets). Every role is an
inference from those facts, not a confirmed contract. The lwarx/stwcx., mfmsr/mtmsr, tlbia,
rotate and shift instructions are marked unsupported by the renderer, so their effects are
not part of any role below. The last eight kernel entries are the patch copies of the first
eight cd-ist-3.24 kernel routines; their rendered facts match those copies line for line apart
from the binary path.

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

- `sub_fe004efc` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** Sets up a 0x20-byte frame and stores a word to 0xfa200004 at 0xfe004f84. It makes a sequence of
  calls to 0xfe004610 with the constant addresses 0xe013ec, 0xe01418, 0xe01428, 0xe01454 and 0xe01474
  (0xfe004f30 to 0xfe004f8c), and calls 0xfe0051f0 with r3=0xe01868 at 0xfe004f24. It calls 0xfe004ff4 and
  0xfe004f98 with r3=0xfe040000, and branches at 0xfe004f40 on the result of 0xfe004ff4, so the later calls
  depend on that result. The constant addresses are not read here, so what the calls report is unknown.
  Reviewed: 2026-11-12

- `sub_fe0051a0` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** A routine with no frame and no calls. It stores halfwords to 0xfa200950, 0xfa200952, 0xfa200954
  and 0xfa200962 to 0xfa200966, and stores words to 0xfa200ab8, 0xfa200abc and 0xfa200ac4, with a halfword to
  0xfa200ac2 (stores at 0xfe0051a8 to 0xfe0051e8). Its return at 0xfe0051ec gives back its r3 unchanged. It
  looks like a field initialiser for the 0xfa2009xx and 0xfa200axx block, but the field names are not known.
  Reviewed: 2026-11-12

- `sub_fe048ba0` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** Reads and rewrites the words at 0xfa200abc and 0xfa200ab8 (loads at 0xfe048ba8 and 0xfe048bb8,
  stores at 0xfe048bb4 and 0xfe048bc4), ORs 0x30 into the halfword at 0xfa200ac2 (0xfe048bc8 to 0xfe048bd4), and
  writes fields at 0xfa203c80 to 0xfa203cac (stores at 0xfe048bd8 to 0xfe048bf8). It writes 0x11 to the halfword
  at 0xfa2009c0 (0xfe048c08), then polls that halfword until bit 0 clears (loop at 0xfe048c10 to 0xfe048c18).
  After the poll it makes five byte stores to 0xfa200860 to 0xfa200874 (0xfe048c1c to 0xfe048c38), and writes 1
  to 0x705cd8 and 0xfa200030 at 0xfe048c44 and 0xfe048c4c. The routine at 0xfe048c54 calls it at 0xfe048c90 with
  r9=0x700000 when the word at 0x705cd8 is zero. The purpose of the writes to 0xfa203c8x is not settled.
  Reviewed: 2026-11-12

- `sub_fe048c54` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** A routine with a 0x20-byte frame that loads the word at 0x705cd8 (0xfe048c74) and calls
  0xfe048ba0 at 0xfe048c90 with r9=0x700000 only when that word is zero (the bne at 0xfe048c8c skips the call when
  it is nonzero). It reads the halfwords at 0xfa203c80 and 0xfa203c82 and adds 0x1640 and the base 0xfa2009c0 to
  form two addresses. It stores bytes to 0xfa200860, 0xfa20086c and 0xfa200870, and runs two counted loops (bdnz
  at 0xfe048d44 and 0xfe048de4, count 0x989680) that poll a halfword until its bit 0x8000 clears. When either
  poll runs out it calls 0xfe044770 at 0xfe048e08 with r3=0x705720 or 0x705740 and returns -1. The meaning of the
  descriptors and the purpose are unclear.
  Reviewed: 2026-11-12

- `sub_c000a5e0` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** Sets up a 0x20-byte frame, calls 0xc0004710 at 0xc000a5ec, reads the word at 0xfa200284 (lwz at
  0xc000a5f4), sets bit 0x80 and writes it back (store at 0xc000a600), then loads the byte at 0xfa20028c
  (0xc000a60c) and stores it to the stack (0xc000a614). It touches the machine state register with mfmsr at
  0xc000a5f8 and mtmsr at 0xc000a608, with an rlwinm between them, whose bits the renderer does not model. It
  calls 0xc0010640 with the constant 0xc00f0164 at 0xc000a61c and ends in a branch to itself at 0xc000a620. The
  MSR bits it changes are unknown.
  Reviewed: 2026-11-12

- `sub_c00a2374` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** Same code as the cd-ist-3.24 copy at 0xc00a2374: a 16-byte frame, five word stores into
  0xc01604e8 to 0xc01604f8 (four at 0xc00a23a8 to 0xc00a23b4, one at 0xc00a23d8), a call to 0xc000b454 with
  r5=0xfa200000 and r6=0xc0160000 from 0xc00a23b8, and a store of 1 to 0xfa200030 at 0xc00a2388. The tlbia at
  0xc00a23d0 is not modelled, so the TLB effect and the purpose of the store block are unclear.
  Reviewed: 2026-11-12

- `sub_c00a23ec` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** Same code as the cd-ist-3.24 copy at 0xc00a23ec: a frame, stores to 0xfa200940 and 0xfa200948,
  a call to 0xc0003300 with r3=5 and r4=0xc00a247c, a call to 0xc0010094 when that result is nonzero, and a call
  to 0xc00a250c at 0xc00a2458 (r3=0, r4=0xc00a2508, r5=0). After that call it sets bit 0x80 in the word at
  0xfa200940 and returns the call's r3. The callee at 0xc0003300 is not identified.
  Reviewed: 2026-11-12

- `sub_c00a247c` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** Same code as the cd-ist-3.24 copy at 0xc00a247c: a halfword store and reload at 0xfa200930, an
  index into the table at 0xc01989b0, a register-indirect call at 0xc00a24c8 made only when the table word is
  non-zero (target unresolved), an AND into the word at 0xfa200948 when it is zero, and a store to 0xfa20094c at
  0xc00a24f0 on both paths. The return value is unknown.
  Reviewed: 2026-11-12

- `sub_c00a250c` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** Same code as the cd-ist-3.24 copy at 0xc00a250c: stmw under a 0x20-byte frame, a call to 0xc00a2594
  at 0xc00a252c, a call to 0xc0010640 with the constant 0xc010a1d4 at 0xc00a2558, and a read of 0xfa200948 at
  0xc00a256c. The purpose beyond this call pattern is unclear.
  Reviewed: 2026-11-12

- `sub_c00a2594` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** Same code as the cd-ist-3.24 copy at 0xc00a2594: two indexed word stores at 0xc00a25a8 and
  0xc00a25ac, a word load from 0xfa200948 at 0xc00a25b4, and a store at 0xc00a25c4, returning an unresolved r3
  at 0xc00a25c8. The exact computation is unknown.
  Reviewed: 2026-11-12

- `sub_c00a4e98` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** Same code as the cd-ist-3.24 copy at 0xc00a4e98: a 0x30-byte frame, halfword loads and stores
  from 0xc00a4eb8 to 0xc00a4f28 (no backward branch there), an lwarx/stwcx. loop at 0xc00a4fac to 0xc00a4fb4, and
  calls to 0xc0010640, 0xc0004868, 0xc00046d4, 0xc0004710 and 0xc00046e0. The meaning of the reservation loop is
  unclear.
  Reviewed: 2026-11-12

- `sub_c00a515c` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** Same code as the cd-ist-3.24 copy at 0xc00a515c: a 0x40-byte frame, a test of the word at 0xfa200e44
  at 0xc00a5188, reservation loops at 0xc00a52a8, 0xc00a534c and 0xc00a5368, and a call to the routine at
  0xc00a563c at 0xc00a53e8. The overall purpose is not settled.
  Reviewed: 2026-11-12

- `sub_c00a563c` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** Same code as the cd-ist-3.24 copy at 0xc00a563c: walks the structure rooted at 0xc0160508,
  rewrites 0xc0160504 and 0xc0160508, and stores to 0xfa200e84 and, when the new root is non-null, to 0xfa200e80.
  When the record's word at +4 is non-zero it makes an indirect call through that word at 0xc00a5698, with the
  value loaded from 0xfa200e80 as r3; the target is not resolved. When the root is null it calls 0xc0010640 with
  the constant 0xc010a910 instead.
  Reviewed: 2026-11-12
