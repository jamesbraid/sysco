# Tier-1 routine readings, sysco batch 11

Role statements for targets 275 to 299 of the tier-1 list, written from the rendered instruction
facts only: registers, calls, constants, branch targets and stored addresses. Every entry is
inferred. None of the 25 was rejected by the grader. The L2 firmware images have no symbols, so the
`sub_` names are the inventory's own. Where the renderer marked an instruction as not modelled, the
entry says so.

- `sub_fe004efc` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** A flash routine with a 0x20-byte frame that calls the helper at 0xfe004610 five times,
  each time with a constant pointer in the 0xe013ec to 0xe01474 range as its first argument. That call
  pattern looks like printing constant messages. It also calls 0xfe0051f0 with r9=0xfe040000 and
  0xfe004ff4 with r3=0xfe040000, and branches at 0xfe004f40 on the return of 0xfe004ff4, so the printing
  depends on a status check. It ends by storing 0xfa200004 at 0xfe004f84.

- `sub_fe0051a0` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** A leaf routine with no calls. It stores halfwords to 0xfa200950, 0xfa200952, 0xfa200954,
  0xfa200960, 0xfa200962, 0xfa200964 and 0xfa200966, and words to 0xfa200ab8, 0xfa200abc and 0xfa200ac4,
  with a halfword at 0xfa200ac2. It looks like a block of fixed values written into the 0xfa2009xx to
  0xfa200axx region, and sub_fe048ba0 reads back the 0xfa200abc, 0xfa200ab8 and 0xfa200ac2 fields. What
  those fields configure is a question for the device's datasheet, not something the code shows.

- `sub_fe048ba0` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** A routine that reads and then rewrites the fields at 0xfa200abc, 0xfa200ab8 and
  0xfa200ac2 (loads at 0xfe048ba8, 0xfe048bb8 and 0xfe048bc8, stores at 0xfe048bb4, 0xfe048bc4 and
  0xfe048bd4), the fields sub_fe0051a0 writes. It also stores halfwords to 0xfa203c80, 0xfa203c82,
  0xfa203cac, 0xfa203c86 and 0xfa2009c0, and bytes to 0xfa203c84 and 0xfa203c85. The loop at 0xfe048c10
  stores to addresses the renderer shows only as a register value, and its `andi.` at 0xfe048c14 was
  not modelled, so this reading covers the straight-line part and the loop's exit test at 0xfe048c18.

- `sub_fe048c54` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** A 0x20-byte-frame flash routine that loads the flag word at 0x705cd8 (0xfe048c74) and
  branches on it at 0xfe048c8c. It calls sub_fe048ba0 at 0xfe048c90 with r9=0x700000 and calls 0xfe044770
  at 0xfe048e08. Between those points it runs several loops with bdnz branches at 0xfe048d44 and
  0xfe048de4 that load halfwords and bytes from 0xfa203c80 and 0xfa203c82 and store bytes to 0xfa200860,
  0xfa20086c and 0xfa200870. Most of the other stores resolve only to a register value, so the loops are
  described by their branches and counts.

- `sub_c000a5e0` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** A kernel routine with a 0x20-byte frame that reads the word at 0xfa200284 (0xc000a5f4)
  and reads and writes one byte through a register address at 0xc000a60c and 0xc000a614. It calls
  0xc0004710 at 0xc000a5ec and calls 0xc0010640 with r3=0xc00f0164 at 0xc000a61c, the print-style helper
  that several routines below also call. The opcodes at 0xc000a5f8 (mfmsr) and 0xc000a608 (mtmsr) show it
  reads and then writes the machine state register; the renderer did not model those two instructions.

- `sub_c00a2374` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** A kernel setup routine that stores to 0xfa200030 (0xc00a2388) and to the words at
  0xc01604e8, 0xc01604f0, 0xc01604f4 and 0xc01604f8 (0xc00a23a8 to 0xc00a23b4), then stores to 0xc01604e8
  again at 0xc00a23d8. It calls 0xc000b454 at 0xc00a23b8 with r4=r3+4096, r5=0xfa200000 and r6 to r10
  set to 0xc0160000, which suggests a helper that maps a 4 KiB region at the 0xfa200000 base. It ends with
  tlbia at 0xc00a23d0, which invalidates the TLB; that instruction was not modelled.

- `sub_c00a23ec` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** A kernel routine that calls 0xc0003300 at 0xc00a2430 with r3=5 and r4=0xc00a247c, which
  is the entry of sub_c00a247c, so it looks like it registers sub_c00a247c as a handler for code 5. It
  prints a constant string through 0xc0010094 at 0xc00a2444 and calls sub_c00a250c at 0xc00a2458 with
  r4=0xc00a2508. It stores to 0xfa200940 and 0xfa200948 (0xc00a2414, 0xc00a2418), reads 0xfa200940 back at
  0xc00a245c, and branches at 0xc00a2438 on the return of 0xc0003300.

- `sub_c00a247c` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** A handler-shaped routine whose address sub_c00a23ec passes as r4 to 0xc0003300
  (0xc00a2430), so it is probably called back from that registration. It stores a halfword to 0xfa200930
  and reads it back (0xc00a2494, 0xc00a2498), walks an indexed table with lwzx and lwz at 0xc00a24ac to
  0xc00a24fc, and stores to 0xfa20094c at 0xc00a24f0. Its indirect call at 0xc00a24c8 goes through a
  register whose target the renderer could not resolve, so the callee there is an open point.

- `sub_c00a250c` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** A kernel routine with a 0x20-byte frame that saves registers with stmw at 0xc00a2514 and
  restores them with lmw at 0xc00a2588. It calls sub_c00a2594 at 0xc00a252c and prints a constant string
  through 0xc0010640 at 0xc00a2558 with r3=0xc010a1d4. It reads the word at 0xfa200948 that sub_c00a23ec
  writes (0xc00a256c), and its indexed stores at 0xc00a2560 and 0xc00a2564 appear to update a table under
  the branch at 0xc00a2528.

- `sub_c00a2594` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** A leaf routine with no calls. It loads the word at 0xfa200948 (0xc00a25b4), stores two
  words through indexed addresses (0xc00a25a8, 0xc00a25ac) and one word at 0xc00a25c4, and runs bit
  operations (slwi at 0xc00a25a4, rotlw at 0xc00a25bc, and at 0xc00a25c0) that look like a bit-mask step.
  The renderer did not model those operations, so the mask is read from the opcodes only. It is the
  callee of sub_c00a250c, which reads the same 0xfa200948 word.

- `sub_c00a4e98` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** A kernel routine with a 0x30-byte frame and a stmw save at 0xc00a4ea0. It prints a constant
  string through 0xc0010640 at 0xc00a4ecc with r3=0xc010a7dc, then calls 0xc00046d4 at 0xc00a4f40, 0xc0004710
  on its result at 0xc00a4f44, 0xc0004868 at 0xc00a4f38 and 0xc00046e0 at 0xc00a4fc0. It reads the word at
  0xc0160178 (0xc00a4f58), stores to 0xfa200e54 (0xc00a4f6c), and does an atomic word update with lwarx at
  0xc00a4fac and stwcx. at 0xc00a4fb4. That pattern suggests a reservation-based lock or counter.

- `sub_c00a515c` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** A large kernel routine that reads the flag word at 0xfa200e44 (0xc00a5174) and branches on
  it at 0xc00a5188. It runs loops of lbz, lhz and lwz with table-like indexing and several beq and bne
  tests, and it has atomic word updates with lwarx and stwcx. at 0xc00a52a8 and 0xc00a52b0, 0xc00a534c and
  0xc00a5354, and 0xc00a5368 and 0xc00a5370. It calls 0xc0013dec with r3=0 and r4=1 (0xc00a52e0, 0xc00a53a0),
  0xc0013764 (0xc00a53b0), 0xc00a540c (0xc00a51c4), and sub_c00a563c at 0xc00a53e8. The loop bodies store to
  addresses the renderer shows only as register values, so the table semantics are given by the branches.

- `sub_c00a563c` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** A kernel routine that walks a two-level pointer structure rooted at 0xc0160508: it loads
  the head pointer at 0xc00a5650, tests the next pointers at offsets +4 and +8 (beq at 0xc00a568c and
  0xc00a56b8), and reads the word at [0xc0160508]+8 (0xc00a56bc). It writes pointers back to 0xc0160508
  and 0xc0160504 (0xc00a56a8, 0xc00a56b4), stores to 0xfa200e80 and 0xfa200e84, and makes an indirect call
  at 0xc00a5698 with r3 set from 0xfa200e80. That shape looks like a linked list of handlers, but the call
  target is not resolved in this image.

- `sub_c00a56e0` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** A kernel routine that reads the pointers at 0xc0160504, 0xc0160508 and 0xc016050c and
  rewrites their fields (stores at 0xc00a5728 to 0xc00a5758), so it looks like the reset or relink step for
  the structure sub_c00a563c walks. It tests [0xc0160504] at 0xc00a5714 and [0xc0160508] at 0xc00a5738
  for zero before it uses them. It calls 0xc00046d4 and 0xc0004710 (0xc00a56fc, 0xc00a5700) and 0xc00046e0
  with r10=[0xc0160504] at 0xc00a577c, and it writes the 0xfa200e84 and 0xfa200e80 pair at 0xc00a5768 and
  0xc00a576c, the pair sub_c00a563c also uses.

- `sub_c00a5eb0` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** A large kernel routine with a 0x20-byte frame. It stores four words at 0xfa200e40, 0xfa200e44,
  0xfa200e48 and 0xfa200e4c (0xc00a5eec to 0xc00a5f0c), reads the word at 0xc0131278 (0xc00a5ec8) and the
  halfwords at 0xc012eac8 and 0xc012eaca (0xc00a5f14, 0xc00a5f1c), then scans with bdnz and lhzu loops
  (branches at 0xc00a5f58, 0xc00a6018 and 0xc00a6048). It does an atomic update with lwarx and stwcx. at
  0xc00a5fd4 to 0xc00a5fdc, calls 0xc00046a0 at 0xc00a5ef0 and 0xc00aa0a0 at 0xc00a5fec, and calls
  sub_c00a56e0 at 0xc00a60c0 with r3=0x600a0000 and r4=0xc00a5a60. The multiply instructions in the scan
  were not modelled.

- `sub_c01344f0` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01344f0
  **Inferred.** A kernel routine with a 0x20-byte frame and a stmw save at 0xc01344f8. It prints through
  0xc0010640 at 0xc0134548 with r3=0xc00f011c and r5=0x3c, and calls 0xc0004630 at 0xc0134580. It reads the
  word at 0xc0160c7c (0xc013450c) and the word at 0xfa200280 (0xc0134524), stores to 0xfa200380 (0xc0134518,
  0xc0134520), and calls 0xc0003300 at 0xc01345e0 with r3=0xf and r4=0xc000a584 before branching on the
  result at 0xc01345e8. It prints again through 0xc0010094 at 0xc01345f4. The call to 0xc0003300 with a
  code address in r4 matches the registration pattern in sub_c00a23ec.

- `sub_c013d7e4` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc013d7e4
  **Inferred.** A large kernel setup routine with a 0x60-byte frame and a stmw save at 0xc013d7ec. It fills a
  structure at 0xc0198ab0 through 0xc0198c58 with dozens of word, halfword and byte stores (for example at
  0xc013d840, 0xc013d9d8 and 0xc013d9e0), reads 0xc0131684, 0xc0131688 and 0xc01604e8, and prints through
  0xc0010640 at 0xc013d81c with r3=0xc010a504. It calls the copy or clear helpers 0xc000cbe4 at 0xc013d82c
  with r4=0xc4, and 0xc000c8d4 at 0xc013d888 and 0xc013d9b4 with r5=0x2c and r4=0xc4. It calls 0xc00a25cc with
  r3=0x20 (0xc013dc40), 0xc00a2600 with r3=0x80 (0xc013dc4c), sub_c00a250c at 0xc013df04 and 0xc00a2630 at
  0xc013df1c.

- `sub_fe004efc` in `6.5.27/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** A flash routine with the same instruction facts as the 6.5.26 build at this address: five calls
  to 0xfe004610 with constant pointers 0xe013ec to 0xe01474, calls to 0xfe0051f0 and 0xfe004ff4, a branch at
  0xfe004f40 on the return of 0xfe004ff4, and a store of 0xfa200004 at 0xfe004f84. The call pattern looks
  like printing constant messages after a status check.

- `sub_fe0051a0` in `6.5.27/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** A leaf routine with the same stores as the 6.5.26 build at this address: halfwords to 0xfa200950,
  0xfa200952, 0xfa200954, 0xfa200960, 0xfa200962, 0xfa200964 and 0xfa200966, and words to 0xfa200ab8, 0xfa200abc
  and 0xfa200ac4. It looks like a block of fixed values written into the 0xfa2009xx to 0xfa200axx region that
  sub_fe048ba0 reads back.

- `sub_fe048ba0` in `6.5.27/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** A leaf routine with the same instruction facts as the 6.5.26 build at this address. It reads and
  rewrites the fields at 0xfa200abc, 0xfa200ab8 and 0xfa200ac2, and stores to 0xfa203c80, 0xfa203c82, 0xfa203c84,
  0xfa203c85, 0xfa203cac, 0xfa203c86 and 0xfa2009c0. Its loop at 0xfe048c10 stores to register-valued addresses,
  and its `andi.` at 0xfe048c14 was not modelled, so the reading covers the straight-line part.

- `sub_fe048c54` in `6.5.27/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** A flash routine with the same instruction facts as the 6.5.26 build at this address. It loads the
  flag word at 0x705cd8 (0xfe048c74), calls sub_fe048ba0 at 0xfe048c90 with r9=0x700000 and calls 0xfe044770 at
  0xfe048e08, and runs bdnz-driven loops that read 0xfa203c80 and 0xfa203c82 and store bytes to 0xfa200860,
  0xfa20086c and 0xfa200870.

- `sub_c000a5e0` in `6.5.27/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** A kernel routine with the same instruction facts as the 6.5.26 build at this address. It reads the
  word at 0xfa200284, reads and writes one byte through a register address, calls 0xc0004710 and then the print-style
  helper 0xc0010640 with r3=0xc00f0164, and contains mfmsr and mtmsr at 0xc000a5f8 and 0xc000a608, which the
  renderer did not model.

- `sub_c00a2374` in `6.5.27/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** A kernel setup routine with the same instruction facts as the 6.5.26 build at this address. It
  stores to 0xfa200030 and to the words at 0xc01604e8, 0xc01604f0, 0xc01604f4 and 0xc01604f8, and calls 0xc000b454
  at 0xc00a23b8 with r4=r3+4096 and r5=0xfa200000. It suggests a helper that maps a 4 KiB region at 0xfa200000, and
  it ends with tlbia at 0xc00a23d0, which the renderer did not model.

- `sub_c00a23ec` in `6.5.27/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** A kernel routine with the same instruction facts as the 6.5.26 build at this address. It calls
  0xc0003300 at 0xc00a2430 with r3=5 and r4=0xc00a247c, which looks like a handler registration for code 5, and it
  prints through 0xc0010094 at 0xc00a2444 before calling sub_c00a250c at 0xc00a2458. It stores to 0xfa200940 and
  0xfa200948 and reads 0xfa200940 back at 0xc00a245c.

- `sub_c00a247c` in `6.5.27/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** A handler-shaped routine with the same instruction facts as the 6.5.26 build at this address. It is
  the address that sub_c00a23ec passes to 0xc0003300, so it is probably called back from there. It stores a halfword
  to 0xfa200930, walks an indexed table with lwzx and lwz, stores to 0xfa20094c, and makes an indirect call at
  0xc00a24c8 whose target the renderer could not resolve.
