# Tier-1 routine readings, sysco batch 16

Role statements for targets 400 to 424 of the ranked tier-1 list, written from the rendered
instruction facts only (registers, calls, constants and branch targets). Every role is an
inference from those facts, not a confirmed contract. The lwarx/stwcx., mfmsr/mtmsr, tlbia,
rotate and shift instructions are marked unsupported by the renderer, so their effects are
not part of any role below. The last eight kernel entries are the patch copies of the first
eight cd-ist-3.24 kernel routines; their rendered facts match those copies line for line apart
from the binary path.

- `sub_c00a2374` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** Sets up a 16-byte frame and stores four words into the block at 0xc01604e8 to 0xc01604f8
  (stores at 0xc00a23a8 to 0xc00a23d8). It calls the routine at 0xc000b454 from 0xc00a23b8 with r5=0xfa200000
  and r6=0xc0160000, and stores to 0xfa200030 at 0xc00a2388. The tlbia at 0xc00a23d0 is not modelled, so the
  TLB effect and the purpose of the store block are unclear.

- `sub_c00a23ec` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** Sets up a frame and writes 0xfa200940 and 0xfa200948 (stores at 0xc00a2414, 0xc00a2418 and
  0xc00a2464). It calls 0xc0003300 with r3=5 and r4=0xc00a247c at 0xc00a2430, branches on that result at
  0xc00a2438, and calls the routine at 0xc00a250c at 0xc00a2458 before returning its r3. The shape looks like a
  small wrapper around a registration or lookup call; the callee at 0xc0003300 is not identified.

- `sub_c00a247c` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** Sets up a frame, stores a halfword to 0xfa200930 and reloads it at 0xc00a2494 and 0xc00a2498,
  then walks several loads through unresolved pointers (lwzx at 0xc00a24ac and 0xc00a24c0, lwz from 0xc00a24d0
  to 0xc00a24fc). It makes a register-indirect call at 0xc00a24c8 whose target is unresolved, and stores a
  word to 0xfa20094c at 0xc00a24f0. The return value is unknown, and the instruction at 0xc00a2508 is not
  reached from the entry.

- `sub_c00a250c` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** Saves registers with stmw under a 0x20-byte frame, then calls 0xc00a2594 at 0xc00a252c and
  the function at 0xc0010640 with the constant address 0xc010a1d4 at 0xc00a2558. It reads the word at
  0xfa200948 (lwz at 0xc00a256c) and branches on an unresolved operand at 0xc00a2548. The indexed stores and
  the stmw/lmw pair are not fully modelled, so the purpose beyond this call pattern is unclear.

- `sub_c00a2594` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** A small routine with two indexed word stores at 0xc00a25a8 and 0xc00a25ac to unresolved
  addresses, a word load from 0xfa200948 at 0xc00a25b4, and a store at 0xc00a25c4. It returns an unresolved
  r3 at 0xc00a25c8. The rotate and and instructions at 0xc00a25a4 to 0xc00a25c0 are not modelled, so the
  exact computation is unknown.

- `sub_c00a4e98` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** Saves registers under a 0x30-byte frame and loops over halfword fields (lhz and sth between
  0xc00a4eb8 and 0xc00a4f28). It contains an lwarx/stwcx. loop at 0xc00a4fac to 0xc00a4fb4 that updates a word
  at an unresolved address. It calls 0xc0010640 with the constant 0xc010a7dc at 0xc00a4ecc, 0xc0004868 at
  0xc00a4f38, 0xc00046d4 and then 0xc0004710 at 0xc00a4f40 and 0xc00a4f44, and 0xc00046e0 at 0xc00a4fc0.
  The meaning of the reservation loop is unclear.

- `sub_c00a515c` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** A large routine with a 0x40-byte frame that tests the word at 0xfa200e44 at 0xc00a5188 and
  takes a different path when it is zero. Several reservation loops (lwarx/stwcx. at 0xc00a52a8, 0xc00a534c
  and 0xc00a5368) then update halfword and byte fields. It calls 0xc0010640 with the constants 0xc010a894 and
  0xc010a8a4, 0xc0013dec with r4=1 at 0xc00a52e0 and 0xc00a53a0, 0xc0013764 at 0xc00a53b0, and the routine at
  0xc00a563c at 0xc00a53e8. The control flow is dense and many tests have unresolved operands, so only the
  call structure is settled here; the overall purpose is not.

- `sub_c00a563c` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** Walks a small pointer structure rooted at the word 0xc0160508. When that word is non-null it
  reads the fields at +4 and +8 (0xc00a5684 and 0xc00a569c), then rewrites the words at 0xc0160504 and
  0xc0160508 (stores at 0xc00a56a8 to 0xc00a56b4). It stores working values to 0xfa200e80 and 0xfa200e84 and
  calls 0xc0010640 with the constant 0xc010a910 at 0xc00a567c. It then makes an indirect call through the
  pointer held in 0xfa200e80 (blrl at 0xc00a5698), whose target is not resolved. The routine at 0xc00a515c
  calls this one at 0xc00a53e8.

- `sub_c00a56e0` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** Rewrites the record that the word at 0xc0160504 points to (stores at 0xc00a5728 to 0xc00a5734),
  and relinks the words at 0xc016050c and 0xc0160508 (stores at 0xc00a5744 to 0xc00a5758). It stores to
  0xfa200e84 and 0xfa200e80 at 0xc00a5768 and 0xc00a576c, calls 0xc00046d4 and then 0xc0004710 at 0xc00a56fc
  and 0xc00a5700, and calls 0xc00046e0 at 0xc00a577c. It uses the same 0xc0160504 and 0xc0160508 words as the
  routine at 0xc00a563c, and the routine at 0xc00a5eb0 calls it at 0xc00a60c0. It looks like a list insert or
  update helper, but the list layout is not settled.

- `sub_c00a5eb0` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** Saves registers under a 0x20-byte frame, reads the word at 0xc0131278 (lwz at 0xc00a5ec8), and
  stores four words at 0xfa200e40, 0xfa200e44, 0xfa200e48 and 0xfa200e4c. It then runs a counted loop (bdnz at
  0xc00a5f58) that reads halfwords from 0xc012eac8 and 0xc012eaca and stores halfwords through unresolved
  pointers. It calls 0xc00046a0, 0xc00aa0a0, and 0xc0004c2c with r10=8, and the routine at 0xc00a56e0 at
  0xc00a60c0 with r3=0x600a0000. The loop bounds and destinations are unresolved, so what the table is for is
  unclear.

- `sub_c01344f0` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01344f0
  **Inferred.** Sets up a 0x20-byte frame and reads the word at 0xc0160c7c (lwz at 0xc013450c), then writes
  0xfa200380 (stores at 0xc0134518 and 0xc0134520) and reads 0xfa200280 at 0xc0134524. Halfword loops use divw
  and mulhw at 0xc0134568 and 0xc0134578 on operands that are not resolved. It calls 0xc0010640 with the constant
  0xc00f011c and r5=0x3c at 0xc0134548, 0xc0004630 at 0xc0134580, and 0xc0003300 with r3=0xf and r4=0xc000a584 at
  0xc01345e0. It calls 0xc0010094 at 0xc01345f4 only when that result is nonzero. The value the division computes
  is unknown.

- `sub_c013d7e4` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc013d7e4
  **Inferred.** A large routine with a 0x60-byte frame and a 0x48-byte register save. It loads the words at
  0xc0131684, 0xc0131688 and 0xc0160500 and fills a block from 0xc0198ab0 to 0xc0198b64 with more than fifty
  word and halfword stores. It calls 0xc0013c38 at 0xc013d800, 0xc0010640 with the constant 0xc010a504 at
  0xc013d81c, 0xc000cbe4 and 0xc000c8d4 on blocks at 0xc0198ab0 and 0xc0198acc, and 0xc006b73c with the
  constant 0xc010a528 at 0xc013d9e4. Near its end it calls 0xc00a25cc, 0xc00a2600 and 0xc00a250c at
  0xc013dc40 to 0xc013df04. The purpose of the block fill is not settled.

- `sub_fe004efc` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** Sets up a 0x20-byte frame and stores a word to 0xfa200004 at 0xfe004f84. It makes a sequence of
  calls to 0xfe004610 with the constant addresses 0xe013ec, 0xe01418, 0xe01428, 0xe01454 and 0xe01474
  (0xfe004f30 to 0xfe004f8c), and calls 0xfe0051f0 with r3=0xe01868 at 0xfe004f24. It calls 0xfe004ff4 and
  0xfe004f98 with r3=0xfe040000, and branches at 0xfe004f40 on the result of 0xfe004ff4, so the later calls
  depend on that result. The constant addresses are not read here, so what the calls report is unknown.

- `sub_fe0051a0` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** A routine with no frame and no calls. It stores halfwords to 0xfa200950, 0xfa200952, 0xfa200954
  and 0xfa200962 to 0xfa200966, and stores words to 0xfa200ab8, 0xfa200abc and 0xfa200ac4, with a halfword to
  0xfa200ac2 (stores at 0xfe0051a8 to 0xfe0051e8). Its return at 0xfe0051ec gives back its r3 unchanged. It
  looks like a field initialiser for the 0xfa2009xx and 0xfa200axx block, but the field names are not known.

- `sub_fe048ba0` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** Reads and rewrites the words at 0xfa200abc and 0xfa200ab8 (loads at 0xfe048ba8 and 0xfe048bb8,
  stores at 0xfe048bb4 and 0xfe048bc4), copies the halfword at 0xfa200ac2, and writes fields at 0xfa203c80 to
  0xfa203cac. A byte loop at 0xfe048c10 to 0xfe048c18 makes five byte stores, and the routine writes 0x705cd8
  and 0xfa200030 at 0xfe048c44 and 0xfe048c4c. The routine at 0xfe048c54 calls it at 0xfe048c90 with r9=0x700000.
  The purpose of the writes to 0xfa203c8x is not settled.

- `sub_fe048c54` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** A routine with a 0x20-byte frame that tests the word at 0x705cd8 (bne at 0xfe048c8c) and calls
  0xfe048ba0 with r9=0x700000 when it is nonzero. It then runs two counted loops (bdnz at 0xfe048d44 and
  0xfe048de4) over halfword data loaded from 0xfa203c80 and 0xfa203c82, with byte and halfword stores to
  0xfa200860, 0xfa200870 and 0xfa20086c, and calls 0xfe044770 at 0xfe048e08. The trip counts and the data
  source are unresolved, so the purpose is unclear.

- `sub_c000a5e0` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** Sets up a 0x20-byte frame, reads the word at 0xfa200284 (lwz at 0xc000a5f4), and does a byte
  load and store at 0xc000a60c and 0xc000a614 through unresolved pointers. It touches the machine state
  register with mfmsr at 0xc000a5f8 and mtmsr at 0xc000a608, whose bits the renderer does not model. It calls
  0xc0004710 at 0xc000a5ec and 0xc0010640 with the constant 0xc00f0164 at 0xc000a61c. The MSR bits it changes
  are unknown.

- `sub_c00a2374` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** Same code as the cd-ist-3.24 copy at 0xc00a2374: a 16-byte frame, four stores into 0xc01604e8 to
  0xc01604f8, a call to 0xc000b454 with r5=0xfa200000 and r6=0xc0160000 from 0xc00a23b8, and a store to
  0xfa200030 at 0xc00a2388. The tlbia at 0xc00a23d0 is not modelled, so the TLB effect and the purpose of the
  store block are unclear.

- `sub_c00a23ec` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** Same code as the cd-ist-3.24 copy at 0xc00a23ec: a frame, stores to 0xfa200940 and 0xfa200948, a
  call to 0xc0003300 with r3=5 and r4=0xc00a247c, and a tail call to 0xc00a250c at 0xc00a2458 whose r3 becomes
  the return value. The callee at 0xc0003300 is not identified.

- `sub_c00a247c` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** Same code as the cd-ist-3.24 copy at 0xc00a247c: a halfword store and reload at 0xfa200930, several
  loads through unresolved pointers, a register-indirect call at 0xc00a24c8 with an unresolved target, and a
  store to 0xfa20094c at 0xc00a24f0. The return value is unknown.

- `sub_c00a250c` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** Same code as the cd-ist-3.24 copy at 0xc00a250c: stmw under a 0x20-byte frame, a call to 0xc00a2594
  at 0xc00a252c, a call to 0xc0010640 with the constant 0xc010a1d4 at 0xc00a2558, and a read of 0xfa200948 at
  0xc00a256c. The purpose beyond this call pattern is unclear.

- `sub_c00a2594` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** Same code as the cd-ist-3.24 copy at 0xc00a2594: two indexed word stores at 0xc00a25a8 and
  0xc00a25ac, a word load from 0xfa200948 at 0xc00a25b4, and a store at 0xc00a25c4, returning an unresolved r3
  at 0xc00a25c8. The exact computation is unknown.

- `sub_c00a4e98` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** Same code as the cd-ist-3.24 copy at 0xc00a4e98: a 0x30-byte frame, halfword loops from 0xc00a4eb8
  to 0xc00a4f28, an lwarx/stwcx. loop at 0xc00a4fac to 0xc00a4fb4, and calls to 0xc0010640, 0xc0004868, 0xc00046d4,
  0xc0004710 and 0xc00046e0. The meaning of the reservation loop is unclear.

- `sub_c00a515c` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** Same code as the cd-ist-3.24 copy at 0xc00a515c: a 0x40-byte frame, a test of the word at 0xfa200e44
  at 0xc00a5188, reservation loops at 0xc00a52a8, 0xc00a534c and 0xc00a5368, and a call to the routine at
  0xc00a563c at 0xc00a53e8. The overall purpose is not settled.

- `sub_c00a563c` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** Same code as the cd-ist-3.24 copy at 0xc00a563c: walks the structure rooted at 0xc0160508, rewrites
  0xc0160504 and 0xc0160508, stores to 0xfa200e80 and 0xfa200e84, and makes an indirect call through the pointer in
  0xfa200e80 at 0xc00a5698. The indirect target is not resolved.
