# Tier-1 routine readings, sysco batch 11

Role statements for targets 275 to 299 of the tier-1 list, written from the rendered instruction
facts only: registers, calls, constants, branch targets and stored addresses. Every entry is
inferred. None of the 25 was rejected by the grader. The L2 firmware images have no symbols, so the
`sub_` names are the inventory's own. Where the renderer marked an instruction as not modelled, the
entry says so.

- `sub_fe004efc` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** A flash routine with a 0x20-byte frame that calls the helper at 0xfe004610 six times
  (0xfe004f30, 0xfe004f4c, 0xfe004f60, 0xfe004f6c, 0xfe004f74, 0xfe004f8c), each time with a constant pointer
  in the 0xe013ec to 0xe01474 range as its first argument. That call pattern looks like printing constant
  messages. Before the first of them it calls 0xfe0051f0 with r3=0xe01868. It calls 0xfe004ff4 with
  r3=0xfe040000 (copied from r9 through r29), and branches at 0xfe004f40 on its return: a non-zero return
  skips the 0xfe004f44 to 0xfe004f74 block (a call to 0xfe004f98 and four of the helper calls), so those
  calls depend on the status check. Every path then stores 0xffffff80 as a word at 0xfa200004 (0xfe004f84),
  calls the helper once more at 0xfe004f8c and 0xfe0046c8, and ends in a branch to itself at 0xfe004f94.
  Reviewed: 2026-11-12

- `sub_fe0051a0` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** A leaf routine with no calls. It stores halfwords to 0xfa200950, 0xfa200952, 0xfa200954,
  0xfa200960, 0xfa200962, 0xfa200964 and 0xfa200966, and words to 0xfa200ab8, 0xfa200abc and 0xfa200ac4,
  with a halfword at 0xfa200ac2. It looks like a block of fixed values written into the 0xfa2009xx to
  0xfa200axx region, and sub_fe048ba0 reads back the 0xfa200abc, 0xfa200ab8 and 0xfa200ac2 fields. What
  those fields configure is a question for the device's datasheet, not something the code shows.
  Reviewed: 2026-11-12

- `sub_fe048ba0` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** A routine that reads and then rewrites the fields at 0xfa200abc, 0xfa200ab8 and
  0xfa200ac2 (loads at 0xfe048ba8, 0xfe048bb8 and 0xfe048bc8, stores at 0xfe048bb4, 0xfe048bc4 and
  0xfe048bd4), the fields sub_fe0051a0 writes. It also stores halfwords to 0xfa203c80, 0xfa203c82,
  0xfa203cac, 0xfa203c86 and 0xfa2009c0, and bytes to 0xfa203c84 and 0xfa203c85. The loop at 0xfe048c10
  only loads: it reads the halfword at 0xfa2009c0 and repeats while bit 0 of it is set (the `andi.` at
  0xfe048c14 was not modelled, so that bit test is read from the opcode, with the exit test at
  0xfe048c18). After the loop it stores bytes to 0xfa200860, 0xfa200864, 0xfa200868, 0xfa200870 and 0xfa200874, a word
  at 0x705cd8 and a word at 0xfa200030.
  Reviewed: 2026-11-12

- `sub_fe048c54` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** A 0x20-byte-frame flash routine that loads the word at 0x705cd8 (0xfe048c74) and
  branches on it at 0xfe048c8c: it calls sub_fe048ba0 at 0xfe048c90 only when that word is zero (r9=0x700000
  is the address base, not an argument). It then reads the halfwords at 0xfa203c80 and 0xfa203c82, builds
  indexed addresses from them, and runs two bounded wait loops (bdnz at 0xfe048d44 and 0xfe048de4) that
  re-read a halfword at that computed address until bit 0x8000 clears or a count runs out. It stores bytes to
  0xfa200860, 0xfa20086c and 0xfa200870. A timeout in either loop reaches the call to 0xfe044770 at
  0xfe048e08 with r3 set to a constant pointer (0x705720 or 0x705740), followed by a return of -1; the other
  exit returns the halfword at [r7+2]. Most of the other stores resolve only to a register value.
  Reviewed: 2026-11-12

- `sub_c000a5e0` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** A kernel routine with a 0x20-byte frame that calls 0xc0004710 at 0xc000a5ec, then
  reads the word at 0xfa200284 (0xc000a5f4) and stores it back with bit 0x80 set (0xc000a600), and loads one
  byte from 0xfa20028c (0xc000a60c) into a stack slot (0xc000a614). It calls 0xc0010640 with r3=0xc00f0164 at
  0xc000a61c, which looks like the print helper that several routines below also call, and then branches to
  itself at 0xc000a620. The opcodes at 0xc000a5f8 (mfmsr) and 0xc000a608 (mtmsr) show it reads and then
  writes the machine state register; the renderer did not model those two instructions.
  Reviewed: 2026-11-12

- `sub_c00a2374` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** A kernel setup routine that stores to 0xfa200030 (0xc00a2388) and to the words at
  0xc01604ec, 0xc01604f0, 0xc01604f8 and 0xc01604f4 (0xc00a23a8 to 0xc00a23b4). It calls 0xc000b454 at
  0xc00a23b8 with r4=r3+4096 (r3 unchanged from entry), r5=0xfa200000 and r6, r8, r9 and r10 set to
  0xc0160000 (r7 is 1), which suggests a helper that maps a 4 KiB region at the 0xfa200000 base. It then
  sets bit 2 in the word the call returned a pointer to (0xc00a23bc to 0xc00a23cc), executes tlbia at
  0xc00a23d0 (not modelled; it invalidates the TLB) and stores 0xfa2009c0 to 0xc01604e8 at 0xc00a23d8
  before returning.
  Reviewed: 2026-11-12

- `sub_c00a23ec` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** A kernel routine that calls 0xc0003300 at 0xc00a2430 with r3=5 and r4=0xc00a247c, which
  is the entry of sub_c00a247c, so it looks like it registers sub_c00a247c as a handler for code 5. It
  branches at 0xc00a2438 on the return of 0xc0003300: when the return is non-zero it calls 0xc0010094 at
  0xc00a2444 with a constant pointer in r3 (a print-style call, inferred from the call pattern). It then
  calls sub_c00a250c at 0xc00a2458 with r3=0, r4=0xc00a2508 and r5=0. It stores to 0xfa200940 and
  0xfa200948 (0xc00a2414, 0xc00a2418), and reads 0xfa200940 back at 0xc00a245c and stores it again with bit
  0x80 set.
  Reviewed: 2026-11-12

- `sub_c00a247c` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** A handler-shaped routine whose address sub_c00a23ec passes as r4 to 0xc0003300
  (0xc00a2430), so it is probably called back from that registration. It stores a halfword to 0xfa200930 and
  reads it back (0xc00a2494, 0xc00a2498), takes the index r9>>11 from that value and looks up an 8-byte
  entry in a table with lwzx (0xc00a24ac, 0xc00a24c0). When the first word of the entry is non-zero it
  makes an indirect call at 0xc00a24c8 through that word, with the second word as r3 and its own r5 as r4;
  the target is table data, so the callee is an open point. When it is zero it clears the matching bit in
  0xfa200948 instead (0xc00a24d0 to 0xc00a24e0). It then stores a one-bit mask to 0xfa20094c at 0xc00a24f0
  on both paths.
  Reviewed: 2026-11-12

- `sub_c00a250c` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** A kernel routine with a 0x20-byte frame that saves registers with stmw at 0xc00a2514
  and restores them with lmw at 0xc00a2588. When its r4 argument is zero it calls sub_c00a2594 at
  0xc00a252c and returns. Otherwise it indexes a table by r3 (8-byte entries). If the entry's first word is
  non-zero it prints a constant string through 0xc0010640 at 0xc00a2558 with r3=0xc010a1d4 and r4 set to its
  own r4. It then stores its r5 and r4 into the entry (0xc00a2560, 0xc00a2564), and reads the word at
  0xfa200948 that sub_c00a23ec writes (0xc00a256c), sets bit r3 in it and stores it back (0xc00a257c).
  Reviewed: 2026-11-12

- `sub_c00a2594` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** A leaf routine with no calls. It loads the word at 0xfa200948 (0xc00a25b4), stores two
  words through indexed addresses (0xc00a25a8, 0xc00a25ac) and one word at 0xc00a25c4, and runs bit
  operations (slwi at 0xc00a25a4, rotlw at 0xc00a25bc, and at 0xc00a25c0) that look like a bit-mask step.
  The renderer did not model those operations, so the mask is read from the opcodes only. It is the
  callee of sub_c00a250c, which reads the same 0xfa200948 word.
  Reviewed: 2026-11-12

- `sub_c00a4e98` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** A kernel routine with a 0x30-byte frame and a stmw save at 0xc00a4ea0. It tests bit
  0x8000 of the halfword at [[r4+0x60]+0x30]. When it is set, it prints a constant string through 0xc0010640
  at 0xc00a4ecc with r3=0xc010a7dc, clears a field and returns 1. Otherwise it calls 0xc0004868 at
  0xc00a4f38, 0xc00046d4 at 0xc00a4f40 with r3 set to a stack address and 0xc0004710 at 0xc00a4f44, reads
  the word at 0xc0160178 (0xc00a4f58), stores to 0xfa200e54 (0xc00a4f6c), and calls 0xc00046e0 at 0xc00a4fc0
  with r3 loaded from the stack slot that the call at 0xc00a4f40 filled. When bit 0x8000 of a later halfword
  is set, it does an atomic word update with lwarx at 0xc00a4fac and stwcx. at 0xc00a4fb4 that or-s 1 into
  the word at r4+0x2c, a reservation-based flag update.
  Reviewed: 2026-11-12

- `sub_c00a515c` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** A large kernel routine that reads the flag word at 0xfa200e44 (0xc00a5174) and branches on
  it at 0xc00a5188. It runs loops of lbz, lhz and lwz with table-like indexing and several beq and bne
  tests, and it has atomic word updates with lwarx and stwcx. at 0xc00a52a8 and 0xc00a52b0, 0xc00a534c and
  0xc00a5354, and 0xc00a5368 and 0xc00a5370. It calls 0xc0013dec with r3=0 and r4=1 (0xc00a52e0, 0xc00a53a0),
  0xc0013764 (0xc00a53b0), 0xc00a540c (0xc00a51c4), and sub_c00a563c at 0xc00a53e8. The loop bodies store to
  addresses the renderer shows only as register values, so the table semantics are given by the branches.
  Reviewed: 2026-11-12

- `sub_c00a563c` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** A kernel routine that works on a list headed at 0xc0160508: it loads the head pointer
  at 0xc00a5650 and, when it is null, prints a constant string through 0xc0010640 and returns. Otherwise it
  tests the word at head+4 (beq at 0xc00a568c) and, when that is non-zero, makes an indirect call at
  0xc00a5698 through it with r3 set from 0xfa200e80 (read at 0xc00a5668). It then moves the head to the node's
  +8 pointer (store at 0xc00a56a8), pushes the old head node onto the list at 0xc0160504 (0xc00a56b4) and,
  when the new head is non-null, loads the word at its offset 0 and stores it to 0xfa200e80 with 0x14 to
  0xfa200e84 (0xc00a56c4, 0xc00a56c8). It also stores 0 to 0xfa200e84 at 0xc00a5664. That shape looks like a
  queue of handlers with a free list, but the call target is data in the node and is not resolved here.
  Reviewed: 2026-11-12

- `sub_c00a56e0` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** A kernel routine that takes a node from the list at 0xc0160504 and appends it to the
  list headed at 0xc0160508, so it looks like the enqueue step for the structure sub_c00a563c consumes. It
  calls 0xc00046d4 and 0xc0004710 (0xc00a56fc, 0xc00a5700) first, then tests [0xc0160504] at 0xc00a5714: when
  it is zero it skips to the end and returns 1. Otherwise it stores its r3 and r4 into the node (offsets 0
  and 4), clears offset 8 and relinks (stores at 0xc00a5728 to 0xc00a5758). It tests [0xc0160508] at
  0xc00a5738: when it is non-null it links the node after the tail at 0xc016050c; when it is null it makes
  the node the head and tail and writes 0x14 to 0xfa200e84 and its r3 to 0xfa200e80 (0xc00a5768,
  0xc00a576c), the pair sub_c00a563c also uses. It ends with a call to 0xc00046e0 at 0xc00a577c with r3
  loaded from the stack slot filled by the call at 0xc00a56fc.
  Reviewed: 2026-11-12

- `sub_c00a5eb0` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** A large kernel routine with a 0x20-byte frame. It reads the word at 0xc0131278
  (0xc00a5ec8) and scales it with mulhwu and mulli (0xc00a5ed0, 0xc00a5ed8, not modelled) into the argument
  of 0xc00046a0 (called at 0xc00a5ef0). It stores words at 0xfa200e40, 0xfa200e44, 0xfa200e48 and 0xfa200e4c
  (0xc00a5eec to 0xc00a5f0c) and reads the halfwords at 0xc012eac8 and 0xc012eaca (0xc00a5f14, 0xc00a5f1c).
  It then runs a byte copy with a bdnz loop (0xc00a5f58) and two bdnz fill loops (0xc00a6018, 0xc00a6048)
  that write halfwords and words at register-valued addresses. A loop over the words at r30+0 does an atomic
  decrement with lwarx and stwcx. at 0xc00a5fd4 to 0xc00a5fdc and calls 0xc00aa0a0 at 0xc00a5fec when the
  result is zero or the count is 1. It also calls 0xc0004c2c at 0xc00a6094 and sub_c00a56e0 at 0xc00a60c0
  with r3=0x600a0000 and r4=0xc00a5a60.
  Reviewed: 2026-11-12

- `sub_c01344f0` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01344f0
  **Inferred.** A kernel routine with a 0x20-byte frame and a stmw save at 0xc01344f8. It prints through
  0xc0010640 at 0xc0134548 with r3=0xc00f011c and r5=0x3c, and calls 0xc0004630 at 0xc0134580. It reads the
  word at 0xc0160c7c (0xc013450c), reads the word at 0xfa200280 (0xc0134524) and stores it back with bit
  0x2000000 set (0xc013453c), and stores to 0xfa200380 (0xc0134518, 0xc0134520). It calls 0xc0003300 at
  0xc01345e0 with r3=0xf and r4=0xc000a584 and branches on the result at 0xc01345e8: only when the return
  is non-zero it prints through 0xc0010094 at 0xc01345f4. The call to 0xc0003300 with a code address in r4
  matches the registration pattern in sub_c00a23ec.
  Reviewed: 2026-11-12

- `sub_c013d7e4` in `6.5.26/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc013d7e4
  **Inferred.** A large kernel setup routine with a 0x60-byte frame and a stmw save at 0xc013d7ec. It
  fills a structure that starts at 0xc0198ab0 with word, halfword and byte stores (for example at 0xc013d840,
  0xc013d9d8 and 0xc013d9e0), and stores elsewhere, including 0xfa200950 and 0xfa200954. It reads 0xc0131684,
  0xc0131688 and 0xc01604e8, and prints through 0xc0010640 at 0xc013d81c with r3=0xc010a504, r4 and r5 the
  two words at 0xc0131684 and 0xc0131688. It calls 0xc000cbe4 at 0xc013d82c with r3=0xc0198ab0 and r4=0xc4,
  and 0xc000c8d4 at 0xc013d888 (r5=0x2c, r4=0xc017e40c) and 0xc013d9b4, which look like a clear and a copy
  helper (inferred from the arguments). It calls 0xc00a25cc with r3=0x20 (0xc013dc40) and 0xc00a2600 with
  r3=0x80 (0xc013dc4c), calls sub_c00a250c at 0xc013df04 and 0xc00a2630 at 0xc013df1c.
  Reviewed: 2026-11-12

- `sub_fe004efc` in `6.5.27/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** A flash routine with the same instruction facts as the 6.5.26 build at this address:
  six calls to 0xfe004610 with constant pointers 0xe013ec to 0xe01474, a call to 0xfe0051f0 before them, a call
  to 0xfe004ff4 with r3=0xfe040000, and a branch at 0xfe004f40 on its return that skips the middle block of
  the helper calls when the return is non-zero. Every path stores 0xffffff80 at 0xfa200004 (0xfe004f84) and
  ends in a branch to itself at 0xfe004f94. The call pattern looks like printing constant messages after a
  status check.
  Reviewed: 2026-11-12

- `sub_fe0051a0` in `6.5.27/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** A leaf routine with the same stores as the 6.5.26 build at this address: halfwords to 0xfa200950,
  0xfa200952, 0xfa200954, 0xfa200960, 0xfa200962, 0xfa200964 and 0xfa200966, and words to 0xfa200ab8, 0xfa200abc
  and 0xfa200ac4. It looks like a block of fixed values written into the 0xfa2009xx to 0xfa200axx region that
  sub_fe048ba0 reads back.
  Reviewed: 2026-11-12

- `sub_fe048ba0` in `6.5.27/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** A leaf routine with the same instruction facts as the 6.5.26 build at this address. It
  reads and rewrites the fields at 0xfa200abc, 0xfa200ab8 and 0xfa200ac2, and stores to 0xfa203c80, 0xfa203c82,
  0xfa203c84, 0xfa203c85, 0xfa203cac, 0xfa203c86 and 0xfa2009c0. Its loop at 0xfe048c10 only loads the
  halfword at 0xfa2009c0 and repeats while bit 0 of it is set (the `andi.` at 0xfe048c14 was not modelled, so
  the bit test is read from the opcode). After it the routine stores bytes to 0xfa200860, 0xfa200864,
  0xfa200868, 0xfa200870 and 0xfa200874, a word at 0x705cd8 and a word at 0xfa200030.
  Reviewed: 2026-11-12

- `sub_fe048c54` in `6.5.27/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** A flash routine with the same instruction facts as the 6.5.26 build at this address. It
  loads the word at 0x705cd8 (0xfe048c74) and calls sub_fe048ba0 at 0xfe048c90 only when it is zero (r9=0x700000
  is an address base, not an argument). It reads 0xfa203c80 and 0xfa203c82, runs two bdnz wait loops that
  re-read a halfword at a computed address, stores bytes to 0xfa200860, 0xfa20086c and 0xfa200870, and
  reaches the call to 0xfe044770 at 0xfe048e08 on a timeout in either loop.
  Reviewed: 2026-11-12

- `sub_c000a5e0` in `6.5.27/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** A kernel routine with the same instruction facts as the 6.5.26 build at this address. It
  calls 0xc0004710, reads the word at 0xfa200284 and stores it back with bit 0x80 set, loads one byte from
  0xfa20028c into a stack slot, calls 0xc0010640 with r3=0xc00f0164 (the print-style helper), branches to
  itself afterwards, and contains mfmsr and mtmsr at 0xc000a5f8 and 0xc000a608, which the renderer did not
  model.
  Reviewed: 2026-11-12

- `sub_c00a2374` in `6.5.27/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** A kernel setup routine with the same instruction facts as the 6.5.26 build at this address.
  It stores to 0xfa200030 and to the words at 0xc01604ec, 0xc01604f0, 0xc01604f4 and 0xc01604f8, and calls
  0xc000b454 at 0xc00a23b8 with r4=r3+4096 and r5=0xfa200000. It suggests a helper that maps a 4 KiB region
  at 0xfa200000. It executes tlbia at 0xc00a23d0 (not modelled) and then stores 0xfa2009c0 to 0xc01604e8
  before returning.
  Reviewed: 2026-11-12

- `sub_c00a23ec` in `6.5.27/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** A kernel routine with the same instruction facts as the 6.5.26 build at this address. It
  calls 0xc0003300 at 0xc00a2430 with r3=5 and r4=0xc00a247c, which looks like a handler registration for
  code 5. When the return is non-zero it prints through 0xc0010094 at 0xc00a2444 (a print-style call,
  inferred from the call pattern). It then calls sub_c00a250c at 0xc00a2458, stores to 0xfa200940 and
  0xfa200948 and reads 0xfa200940 back at 0xc00a245c.
  Reviewed: 2026-11-12

- `sub_c00a247c` in `6.5.27/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** A handler-shaped routine with the same instruction facts as the 6.5.26 build at this
  address. It is the address that sub_c00a23ec passes to 0xc0003300, so it is probably called back from
  there. It stores a halfword to 0xfa200930 and reads it back, uses the value shifted right by 11 as an
  index into an 8-byte-entry table (lwzx), makes an indirect call at 0xc00a24c8 through the entry's first
  word when it is non-zero (the target is table data and is not resolved), and otherwise clears a bit in
  0xfa200948. It stores a one-bit mask to 0xfa20094c on both paths.
  Reviewed: 2026-11-12
