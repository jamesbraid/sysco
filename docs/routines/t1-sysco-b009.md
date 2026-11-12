# Tier-1 routine readings, sysco batch 9

Role statements for the 25 targets at positions 225 to 249 of the tier-1 list, written from the
rendered instruction facts only (registers, calls, constants and branch targets). Every grade is
inferred: the facts show what the code does, not what the routine is for. The grader accepted all
25 entries, so none were rejected. The eight kernel routines in the 6.5.24 build have the same
rendered facts as their 6.5.23 twins at the same addresses, so their roles repeat the 6.5.23 wording.

- `sub_fe0051a0` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** A leaf with no calls and no branches. It stores halfwords to 0xfa200950, 0xfa200952, 0xfa200954, 0xfa200960, 0xfa200962, 0xfa200964, 0xfa200966 and 0xfa200ac2, and words to 0xfa200abc, 0xfa200ab8 and 0xfa200ac4, between 0xfe0051a8 and 0xfe0051e8, then returns through blr at 0xfe0051ec with r3 untouched. The facts show a fixed block of stores into the 0xfa2009xx to 0xfa200acx window, but not what those fields mean.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe048ba0` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** A helper that sub_fe048c54 calls at 0xfe048c90, only when the word at 0x705cd8 is zero. It reads and rewrites the words at 0xfa200abc and 0xfa200ab8 and the halfword at 0xfa200ac2 (each ORed with 0x30), stores into the 0xfa203c80 to 0xfa203cac block, and writes the word at 0x705cd8 (0xfe048c44) and 0xfa200030 (0xfe048c4c). The loop at 0xfe048c10 to 0xfe048c18 stores 0x11 to the halfword at 0xfa2009c0 beforehand, then polls that halfword with lhz and andi. 1 and repeats while bit 0 is set. It does not store inside the loop. What the registers mean is not in the facts.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe048c54` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** A routine with a 0x20-byte frame (stwu at 0xfe048c54) that tests the word at 0x705cd8 (bne at 0xfe048c8c) and calls sub_fe048ba0 at 0xfe048c90 only when that word is zero. It stores bytes to 0xfa200860 and its neighbours and halfwords and words to addresses built from the halfwords at 0xfa203c80 and 0xfa203c82, and runs two polling loops (bdnz at 0xfe048d44 and 0xfe048de4). Each loop re-reads a halfword with lhz and exits when bit 0x8000 is clear or a counter that starts at 0x989680 runs out. The loops do not store. If a counter runs out, it loads r3 with 0x705720 or 0x705740 and calls 0xfe044770 at 0xfe048e08, then loads r3=-1. Otherwise it returns the halfword at 2(r7) after the second loop. The facts do not say what is being transferred.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c000a5e0` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** A routine with a 0x20-byte frame that calls 0xc0004710 at 0xc000a5ec, then reads the word at 0xfa200284 (lwz at 0xc000a5f4) and writes it back with 0x80 ORed in (stw at 0xc000a600). It reads the machine state register (mfmsr at 0xc000a5f8), clears one bit of the value (rlwinm at 0xc000a604) and writes it back (mtmsr at 0xc000a608). It loads the byte at 0xfa20028c onto the stack, calls 0xc0010640 with r3=0xc00f0164 at 0xc000a61c, and then branches to itself at 0xc000a620, so it does not return. What the bit and the registers mean is not in the facts.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a2374` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** A routine with a 0x10-byte frame (stwu at 0xc00a2374) that stores to 0xfa200030 and to the words at 0xc01604ec, 0xc01604f0, 0xc01604f4 and 0xc01604f8, and writes 0xc01604e8. It calls 0xc000b454 at 0xc00a23b8 with r4=r3+4096, r5=0xfa200000, r6=0xc0160000 and r7=1, and it executes tlbia at 0xc00a23d0. The shape suggests a region mapping followed by a TLB flush, but the callee at 0xc000b454 is not named, so that purpose is unconfirmed.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a23ec` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** A wrapper with a 0x10-byte frame that stores words to 0xfa200940 and 0xfa200948, then calls 0xc0003300 at 0xc00a2430 with r3=5, r4=0xc00a247c (the address of sub_c00a247c) and r5=r7=0. If that call returns nonzero (the beq at 0xc00a2438 skips otherwise), it calls 0xc0010094 at 0xc00a2444 with r3=0xc010a1b8. It then calls sub_c00a250c at 0xc00a2458 with r3=0, r4=0xc00a2508 and r5=0, ORs 0x80 into the word at 0xfa200940 (0xc00a245c to 0xc00a2464) and returns with r3 as sub_c00a250c left it. A constant paired with a code address suggests a handler registration, but 0xc0003300 is not named in the facts.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a247c` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** A routine with a 0x10-byte frame (stwu at 0xc00a247c) that writes 1 to the halfword at 0xfa200930 (sth at 0xc00a2494), reads it back (lhz at 0xc00a2498) and shifts the value right by 11 to get an index. It loads a word from a table of 8-byte entries at 0xc01989b0 (lwzx at 0xc00a24ac). If that word is nonzero, it loads the second word of the entry (lwzx at 0xc00a24c0) into r3 and calls through the first word with blrl at 0xc00a24c8, with r4 taken from r5. If it is zero, it clears bit index of the word at 0xfa200948 instead. Either way it stores 1 shifted left by the index to 0xfa20094c (stw at 0xc00a24f0). That value is a bit mask, not a result of the call. Its address is passed as r4 by sub_c00a23ec at 0xc00a2430, so it is probably a callback, but the facts do not settle what it computes.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a250c` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** A routine with a 0x20-byte frame that saves a register range with stmw at 0xc00a2514 and restores it with lmw at 0xc00a2588. If r4 is zero it calls sub_c00a2594 at 0xc00a252c with r3 unchanged and returns. Otherwise it indexes the 8-byte-entry table at 0xc01989b0 by r3. If the first word of the entry is nonzero it calls 0xc0010640 with r3=0xc010a1d4 and r4 set to the new r4. It then stores r5 and r4 into the entry (stwx at 0xc00a2560 and 0xc00a2564), loads the word at 0xfa200948 (lwz at 0xc00a256c) and ORs in 1 shifted left by r3 (slw at 0xc00a2574, or at 0xc00a2578). The facts do not say what the table or the mask mean.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a2594` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** A leaf with no calls. It indexes the 8-byte-entry table at 0xc01989b0 by r3 and stores zero into both words of the entry (stwx at 0xc00a25a8 and 0xc00a25ac). It loads the word at 0xfa200948 (lwz at 0xc00a25b4), clears the bit selected by r3 (rotlw at 0xc00a25bc of -2, and at 0xc00a25c0) and stores it back (stw at 0xc00a25c4). It returns with r3 unchanged at 0xc00a25c8. sub_c00a250c calls it when its r4 is zero, so it undoes that routine's table entry and mask bit, but the facts do not say what they are for.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a4e98` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** A routine with a 0x30-byte frame that saves registers with stmw at 0xc00a4ea0 and restores them with lmw at 0xc00a4fd4. It tests bit 0x8000 of the halfword at the pointer loaded from +0x30 of the structure that r4 points to at +0x60 (lhz and andi. at 0xc00a4eb8 to 0xc00a4ebc). If it is set, it calls 0xc0010640 with r3=0xc010a7dc at 0xc00a4ecc, clears the word at 0x9c of that second structure and returns 1. Otherwise it clears bits of that halfword and writes fields of that structure (stw at 0xc00a4ef4, sth at 0xc00a4efc), calls 0xc0004868 (0xc00a4f38), calls 0xc00046d4 and 0xc0004710 back to back (0xc00a4f40 and 0xc00a4f44) and later 0xc00046e0 (0xc00a4fc0). It also reads 0xc0160178 (0xc00a4f58), writes 0xfa200e54 (0xc00a4f6c) and returns 0. A load-reserve and store-conditional retry loop (lwarx at 0xc00a4fac, stwcx. at 0xc00a4fb4, retry branch at 0xc00a4fb8) ORs 1 into the word at r4+0x2c, only if bit 0x8000 of the next descriptor's halfword is set. The facts do not say what the descriptors are.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a515c` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** A large routine with a 0x40-byte frame that saves registers with stmw at 0xc00a5164. It first tests the word at 0xfa200e44 (lwz at 0xc00a5174, beq at 0xc00a5188), then walks several structures with word, halfword and byte loads and compare branches. Load-reserve and store-conditional sequences near 0xc00a52a8 and 0xc00a5368 update fields the facts do not resolve, and it calls 0xc0013dec twice (0xc00a52e0 and 0xc00a53a0), 0xc0013764 at 0xc00a53b0, and 0xc00a563c at 0xc00a53e8. Its overall job is not established by the facts.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a563c` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** A routine with a 0x20-byte frame and a stmw at 0xc00a5644 that works on a record whose pointer is the word at 0xc0160508. It clears the word at 0xfa200e84 first. If the pointer is zero it calls 0xc0010640 at 0xc00a567c with r3=0xc010a910 and returns. Otherwise it loads the word at +0x4 of the record (lwz at 0xc00a5684) and, if nonzero, calls through it (blrl at 0xc00a5698) with r3 taken from the word at 0xfa200e80. It then loads the field at +0x8 (lwz at 0xc00a569c), stores it as the new pointer at 0xc0160508 and stores the old record into the list head at 0xc0160504 (offset +0x8 of the record gets the old head). If the new pointer is nonzero it copies its first word to 0xfa200e80 and stores 0x14 to 0xfa200e84. The pattern looks like a linked-list update with a callback, but the facts do not say what the list holds.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a56e0` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** A routine with a 0x30-byte frame (stwu at 0xc00a56e0) that calls 0xc00046d4 and 0xc0004710 at 0xc00a56fc and 0xc00a5700. It then reads the word at 0xc0160504. If it is zero it returns 1. Otherwise it unlinks that record (stores its +0x8 field to 0xc0160504) and stores r4, r3 and zero at its +0x4, +0x0 and +0x8. It reads 0xc0160508. If that is nonzero it reads 0xc016050c and links the record after it by storing at +0x8 and at 0xc016050c. If it is zero it stores the record to 0xc0160508 and 0xc016050c, writes r3 to 0xfa200e80 and 0x14 to 0xfa200e84. It calls 0xc00046e0 at 0xc00a577c with r3 reloaded from the stack, not from 0xc0160504. It returns 1 only when the word at 0xc0160504 is zero and 0 otherwise. It has the same list shape as sub_c00a563c. The facts show the list shape but not what the list holds.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a5eb0` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** A routine with a 0x20-byte frame that saves registers with stmw at 0xc00a5eb8 and restores them with lmw at 0xc00a60dc. It reads the word at 0xc0131278 (lwz at 0xc00a5ec8), calls 0xc00046a0 at 0xc00a5ef0, and stores words to 0xfa200e40, 0xfa200e44, 0xfa200e48 and 0xfa200e4c, among others. It copies 6 bytes with an lbz and stbx loop (bdnz at 0xc00a5f58) from 0xc012eac8 to r3+0x71. Two more counted loops run 8 times at 8-byte strides: bdnz at 0xc00a6018 stores 0x8000 halfwords through the pointer at +0x24 of the structure at r3+0x60, and bdnz at 0xc00a6048 clears a halfword and a word per entry through the pointer at +0x28. Near the end it calls sub_c00a56e0 at 0xc00a60c0 with r3=0x600a0000 and r4=0xc00a5a60. The facts do not say what the tables are for.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c01344f0` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01344f0
  **Inferred.** A routine with a 0x20-byte frame and stmw at 0xc01344f8. It reads the word at 0xc0160c7c, writes two words to 0xfa200380 (stw at 0xc0134518 and 0xc0134520), reads 0xfa200280 and writes it back with 0x02000000 ORed in (oris and stw at 0xc0134530 and 0xc013453c), and does a multiply and divide (mulli at 0xc013452c, divw at 0xc0134568 and 0xc0134578). It calls 0xc0010640 with r3=0xc00f011c and r5=0x3c at 0xc0134548 and 0xc0004630 at 0xc0134580. It calls 0xc0003300 at 0xc01345e0 with r3=0xf and r4=0xc000a584 and, if that returns nonzero, calls 0xc0010094 with r3=0xc00f0144. The call shape matches the one in sub_c00a23ec, which suggests a registration, but the callee is unnamed.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c013d7e4` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc013d7e4
  **Inferred.** A routine of 1892 bytes with a 0x60-byte frame and stmw at 0xc013d7ec. It stores words and halfwords into a 0xc4-byte block at 0xc0198ab0 (for example at 0xc013d840, 0xc013d874 and 0xc013d94c) and a second block at 0xc0198b74 (r30, stores near 0xc013d9cc), and calls 0xc002778c with r3=0x90 and r4=0xf0 at 0xc013dbb8. The whole routine has 80 stw and 55 sth instructions, some of them outside that block. It calls 0xc000cbe4 at 0xc013d82c with the 0xc0198ab0 block and length 0xc4. It calls 0xc000c8d4 four times (0xc013d888, 0xc013d9b4, 0xc013db10 and 0xc013db28), the one at 0xc013d9b4 with r4=0xc0198ab0 and r5=0xc4. It calls 0xc00a25cc (0xc013dc40, 0xc013dcf4 with r3=0x20) and 0xc00a2600 (0xc013dc4c, 0xc013dd00 with r3=0x80). It does single indexed halfword loads from 0xc012eac0 and 0xc012eabc (lhzx at 0xc013ddc4 and 0xc013deb0), calls sub_c00a250c at 0xc013df04, and ends with a conditional backward branch at 0xc013df2c to 0xc013dad8 (cmplwi r30 against 3). The record's meaning and the callee names are not in the facts, so the reading stops at its shape.
  Reviewed: 2026-11-12 (sonnet)

- `sub_ffe83af8` in `6.5.24/usr/cpu/firmware/sysco/l1.bin(l1#2)` at 0xffe83af8
  **Inferred.** A ColdFire routine with a link.w frame and movem.l saves at 0xffe83b00. It sets bit 6 of the byte at 0x200007f8 (bset.b at 0xffe83b0e), then reads that byte and the bytes at 0x200007f9 through 0x200007ff with move.b, splitting each into high and low nibbles (asr.l by 4 and and.l 0xf) and combining them with muls.l by 10, 100 and 1000, which looks like decoding packed decimal digits. It clears bit 6 of 0x200007f8 with bclr.b at 0xffe83bfa. It returns -1 if every decoded field is zero or if the first decoded value minus 1930 is above 570 (unsigned compare at 0xffe83c1e to 0xffe83c24). Otherwise it calls 0xffe7c7fc at 0xffe83cde with 0x48 pushed, adds the result, stores the total through the pointer argument if it is nonzero (0xffe83cec) and returns it in d0. The facts do not say which clock or register block it reads.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c000a5e0` in `6.5.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** Same instructions as the 6.5.23 build. A routine with a 0x20-byte frame that calls 0xc0004710 at 0xc000a5ec, then reads the word at 0xfa200284 (lwz at 0xc000a5f4) and writes it back with 0x80 ORed in (stw at 0xc000a600). It reads the machine state register (mfmsr at 0xc000a5f8), clears one bit of the value (rlwinm at 0xc000a604) and writes it back (mtmsr at 0xc000a608). It loads the byte at 0xfa20028c onto the stack, calls 0xc0010640 with r3=0xc00f0164 at 0xc000a61c, and then branches to itself at 0xc000a620, so it does not return. What the bit and the registers mean is not in the facts.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a2374` in `6.5.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** Same rendered facts as the 6.5.23 build. A routine with a 0x10-byte frame that stores to 0xfa200030 and to the words at 0xc01604ec, 0xc01604f0, 0xc01604f4 and 0xc01604f8, and writes 0xc01604e8. It calls 0xc000b454 at 0xc00a23b8 with r4=r3+4096, r5=0xfa200000, r6=0xc0160000 and r7=1, and it executes tlbia at 0xc00a23d0. The shape suggests a region mapping followed by a TLB flush, but the callee at 0xc000b454 is not named, so that purpose is unconfirmed.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a23ec` in `6.5.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** Same instructions as the 6.5.23 build. A wrapper with a 0x10-byte frame that stores words to 0xfa200940 and 0xfa200948, then calls 0xc0003300 at 0xc00a2430 with r3=5, r4=0xc00a247c (the address of sub_c00a247c) and r5=r7=0. If that call returns nonzero (the beq at 0xc00a2438 skips otherwise), it calls 0xc0010094 at 0xc00a2444 with r3=0xc010a1b8. It then calls sub_c00a250c at 0xc00a2458 with r3=0, r4=0xc00a2508 and r5=0, ORs 0x80 into the word at 0xfa200940 (0xc00a245c to 0xc00a2464) and returns with r3 as sub_c00a250c left it. A constant paired with a code address suggests a handler registration, but 0xc0003300 is not named in the facts.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a247c` in `6.5.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** Same instructions as the 6.5.23 build. A routine with a 0x10-byte frame (stwu at 0xc00a247c) that writes 1 to the halfword at 0xfa200930 (sth at 0xc00a2494), reads it back (lhz at 0xc00a2498) and shifts the value right by 11 to get an index. It loads a word from a table of 8-byte entries at 0xc01989b0 (lwzx at 0xc00a24ac). If that word is nonzero, it loads the second word of the entry (lwzx at 0xc00a24c0) into r3 and calls through the first word with blrl at 0xc00a24c8, with r4 taken from r5. If it is zero, it clears bit index of the word at 0xfa200948 instead. Either way it stores 1 shifted left by the index to 0xfa20094c (stw at 0xc00a24f0). That value is a bit mask, not a result of the call. Its address is passed as r4 by sub_c00a23ec at 0xc00a2430, so it is probably a callback, but the facts do not settle what it computes.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a250c` in `6.5.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** Same instructions as the 6.5.23 build. A routine with a 0x20-byte frame that saves a register range with stmw at 0xc00a2514 and restores it with lmw at 0xc00a2588. If r4 is zero it calls sub_c00a2594 at 0xc00a252c with r3 unchanged and returns. Otherwise it indexes the 8-byte-entry table at 0xc01989b0 by r3. If the first word of the entry is nonzero it calls 0xc0010640 with r3=0xc010a1d4 and r4 set to the new r4. It then stores r5 and r4 into the entry (stwx at 0xc00a2560 and 0xc00a2564), loads the word at 0xfa200948 (lwz at 0xc00a256c) and ORs in 1 shifted left by r3 (slw at 0xc00a2574, or at 0xc00a2578). The facts do not say what the table or the mask mean.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a2594` in `6.5.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** Same instructions as the 6.5.23 build. A leaf with no calls. It indexes the 8-byte-entry table at 0xc01989b0 by r3 and stores zero into both words of the entry (stwx at 0xc00a25a8 and 0xc00a25ac). It loads the word at 0xfa200948 (lwz at 0xc00a25b4), clears the bit selected by r3 (rotlw at 0xc00a25bc of -2, and at 0xc00a25c0) and stores it back (stw at 0xc00a25c4). It returns with r3 unchanged at 0xc00a25c8. sub_c00a250c calls it when its r4 is zero, so it undoes that routine's table entry and mask bit, but the facts do not say what they are for.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a4e98` in `6.5.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** Same instructions as the 6.5.23 build. A routine with a 0x30-byte frame that saves registers with stmw at 0xc00a4ea0 and restores them with lmw at 0xc00a4fd4. It tests bit 0x8000 of the halfword at the pointer loaded from +0x30 of the structure that r4 points to at +0x60 (lhz and andi. at 0xc00a4eb8 to 0xc00a4ebc). If it is set, it calls 0xc0010640 with r3=0xc010a7dc at 0xc00a4ecc, clears the word at 0x9c of that second structure and returns 1. Otherwise it clears bits of that halfword and writes fields of that structure (stw at 0xc00a4ef4, sth at 0xc00a4efc), calls 0xc0004868 (0xc00a4f38), calls 0xc00046d4 and 0xc0004710 back to back (0xc00a4f40 and 0xc00a4f44) and later 0xc00046e0 (0xc00a4fc0). It also reads 0xc0160178 (0xc00a4f58), writes 0xfa200e54 (0xc00a4f6c) and returns 0. A load-reserve and store-conditional retry loop (lwarx at 0xc00a4fac, stwcx. at 0xc00a4fb4, retry branch at 0xc00a4fb8) ORs 1 into the word at r4+0x2c, only if bit 0x8000 of the next descriptor's halfword is set. The facts do not say what the descriptors are.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a515c` in `6.5.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** Same rendered facts as the 6.5.23 build. A large routine with a 0x40-byte frame that saves registers with stmw at 0xc00a5164. It first tests the word at 0xfa200e44 (lwz at 0xc00a5174, beq at 0xc00a5188), then walks several structures with word, halfword and byte loads and compare branches. Load-reserve and store-conditional sequences near 0xc00a52a8 and 0xc00a5368 update fields the facts do not resolve, and it calls 0xc0013dec twice (0xc00a52e0 and 0xc00a53a0), 0xc0013764 at 0xc00a53b0, and 0xc00a563c at 0xc00a53e8. Its overall job is not established by the facts.
  Reviewed: 2026-11-12 (sonnet)
