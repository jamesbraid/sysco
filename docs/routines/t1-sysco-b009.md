# Tier-1 routine readings, sysco batch 9

Role statements for the 25 targets at positions 225 to 249 of the tier-1 list, written from the
rendered instruction facts only (registers, calls, constants and branch targets). Every grade is
inferred: the facts show what the code does, not what the routine is for. The grader accepted all
25 entries, so none were rejected. The eight kernel routines in the 6.5.24 build have the same
rendered facts as their 6.5.23 twins at the same addresses, so their roles repeat the 6.5.23 wording.

- `sub_fe0051a0` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** A leaf with no calls and no branches. It stores halfwords to 0xfa200950, 0xfa200952, 0xfa200954, 0xfa200960, 0xfa200962, 0xfa200964 and 0xfa200966 between 0xfe0051a8 and 0xfe0051e8, stores words to 0xfa200abc, 0xfa200ab8 and 0xfa200ac4, and returns with r3 unchanged at 0xfe0051ec. The facts show a fixed block of stores into the 0xfa2009xx to 0xfa200acx window, but not what those fields mean.

- `sub_fe048ba0` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** A helper called from sub_fe048c54 at 0xfe048c90 with r9=0x700000. It reads and rewrites the words at 0xfa200abc and 0xfa200ab8 and the halfword at 0xfa200ac2, stores into the 0xfa203c80 to 0xfa203cac block, and writes the word at 0x705cd8 (0xfe048c44) and 0xfa200030 (0xfe048c4c). A byte-store loop at 0xfe048c10 to 0xfe048c18 has an andi. whose effect the renderer did not model, so the loop's exit condition is unresolved.

- `sub_fe048c54` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** A routine with a 0x20-byte frame (stwu at 0xfe048c54) that tests the word at 0x705cd8 (bne at 0xfe048c8c). It calls sub_fe048ba0 at 0xfe048c90 and a routine at 0xfe044770 at 0xfe048e08, and between those calls runs two counted loops (bdnz at 0xfe048d44 and 0xfe048de4) that store bytes, halfwords and words to addresses the renderer could not resolve. The purpose of the whole pass is not established by these facts.

- `sub_c000a5e0` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** A routine with a 0x20-byte frame that calls 0xc0004710 at 0xc000a5ec and reads the word at 0xfa200284 (lwz at 0xc000a5f4). It reads and writes the machine state register (mfmsr at 0xc000a5f8, mtmsr at 0xc000a608) around a byte update at an unresolved address, which may be a short critical section. It also calls 0xc0010640 with r3=0xc00f0164 at 0xc000a61c. The value written to the MSR is not in the facts, so this reading stops there.

- `sub_c00a2374` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** A routine with a 0x10-byte frame (stwu at 0xc00a2374) that stores to 0xfa200030 and to the words at 0xc01604ec, 0xc01604f0, 0xc01604f4 and 0xc01604f8, and writes 0xc01604e8. It calls 0xc000b454 at 0xc00a23b8 with r4=r3+4096, r5=0xfa200000, r6=0xc0160000 and r7=1, and it executes tlbia at 0xc00a23d0. The shape suggests a region mapping followed by a TLB flush, but the callee at 0xc000b454 is not named, so that purpose is unconfirmed.

- `sub_c00a23ec` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** A wrapper with a 0x10-byte frame that stores words to 0xfa200940 and 0xfa200948, then calls 0xc0003300 at 0xc00a2430 with r3=5 and r4=0xc00a247c, the address of sub_c00a247c. If that call fails (beq at 0xc00a2438), it reports through 0xc0010094 at 0xc00a2444 with r3=0xc010a1b8. It then calls sub_c00a250c at 0xc00a2458 and returns its result. A constant paired with a code address suggests a handler registration, but 0xc0003300 is not named in the facts.

- `sub_c00a247c` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** A routine with a 0x10-byte frame (stwu at 0xc00a247c) that writes the halfword at 0xfa200930 (sth at 0xc00a2494), reads it back (lhz at 0xc00a2498), walks two tables with indexed loads (lwzx at 0xc00a24ac and 0xc00a24c0), and calls through a register at 0xc00a24c8 (blrl, target unresolved). It stores the result at 0xfa20094c (0xc00a24f0). Its address is passed as r4 by sub_c00a23ec at 0xc00a2430, so it is probably a callback, but the facts do not settle what it computes.

- `sub_c00a250c` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** A routine with a 0x20-byte frame that saves a register range with stmw at 0xc00a2514 and restores it with lmw at 0xc00a2588. It calls sub_c00a2594 at 0xc00a252c, loads the word at 0xfa200948 (lwz at 0xc00a256c), and on one branch calls 0xc0010640 with r3=0xc010a1d4 at 0xc00a2558. The shift, rotate and or sequence between 0xc00a253c and 0xc00a2578 builds a bit-field value whose use the facts do not show.

- `sub_c00a2594` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** A leaf with no calls. It stores two words through indexed addresses (stwx at 0xc00a25a8 and 0xc00a25ac), reads the word at 0xfa200948 (lwz at 0xc00a25b4), stores one word at 0xc00a25c4, and returns an unresolved value at 0xc00a25c8. The shifts and rotates before the stores (slwi at 0xc00a25a4, rotlw at 0xc00a25bc) suggest a bit index into a table, but the table base is not resolved.

- `sub_c00a4e98` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** A routine with a 0x30-byte frame that saves registers with stmw at 0xc00a4ea0 and restores them with lmw at 0xc00a4fd4. It calls 0xc0010640 with r3=0xc010a7dc at 0xc00a4ecc, and on later paths calls 0xc0004868 (0xc00a4f38), 0xc00046d4 with 0xc0004710 (0xc00a4f40 and 0xc00a4f44) and 0xc00046e0 (0xc00a4fc0). Its tail is a load-reserve and store-conditional retry loop (lwarx at 0xc00a4fac, stwcx. at 0xc00a4fb4, retry branch at 0xc00a4fb8) on a word the facts do not locate. It reads 0xc0160178 (0xc00a4f58) and writes 0xfa200e54 (0xc00a4f6c).

- `sub_c00a515c` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** A large routine with a 0x40-byte frame that saves registers with stmw at 0xc00a5164. It first tests the word at 0xfa200e44 (lwz at 0xc00a5174, beq at 0xc00a5188), then walks several structures with word, halfword and byte loads and compare branches. Load-reserve and store-conditional sequences near 0xc00a52a8 and 0xc00a5368 update fields the facts do not resolve, and it calls 0xc0013dec twice (0xc00a52e0 and 0xc00a53a0), 0xc0013764 at 0xc00a53b0, and 0xc00a563c at 0xc00a53e8. Its overall job is not established by the facts.

- `sub_c00a563c` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** A routine with a 0x20-byte frame and a stmw at 0xc00a5644 that works on a record whose pointer is the word at 0xc0160508. It loads the fields at +0x4 and +0x8 of that record (lwz at 0xc00a5684 and 0xc00a569c), writes the pointers at 0xc0160508 and 0xc0160504 and the words 0xfa200e80 and 0xfa200e84, and calls through the pointer at 0xfa200e80 (blrl at 0xc00a5698). It also calls 0xc0010640 at 0xc00a567c with r3=0xc010a910. The pattern looks like a linked-list update with a callback, but the facts do not say what the list holds.

- `sub_c00a56e0` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** A routine with a 0x30-byte frame (stwu at 0xc00a56e0) that reads the words at 0xc0160504, 0xc0160508 and 0xc016050c and rewrites them, storing into the record that 0xc0160504 points to at offsets +0x0, +0x4 and +0x8, and into the record at 0xc016050c at +0x8. It has the same list shape as sub_c00a563c. It calls 0xc00046d4 and 0xc0004710 at 0xc00a56fc and 0xc00a5700, and 0xc00046e0 at 0xc00a577c with r10 loaded from 0xc0160504. The facts show the list shape but not what the list holds.

- `sub_c00a5eb0` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** A routine with a 0x20-byte frame that saves registers with stmw at 0xc00a5eb8 and restores them with lmw at 0xc00a60dc. It reads the word at 0xc0131278 (lwz at 0xc00a5ec8), stores four words to 0xfa200e40, 0xfa200e44, 0xfa200e48 and 0xfa200e4c, and loops over halfword tables at 0xc012eac8 and 0xc012eaca (bdnz at 0xc00a5f58 and 0xc00a6048). Near the end it calls sub_c00a56e0 at 0xc00a60c0 with r3=0x600a0000 and r4=0xc00a5a60. The table contents and the loop purposes are not resolved.

- `sub_c01344f0` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01344f0
  **Inferred.** A routine with a 0x20-byte frame and stmw at 0xc01344f8. It reads the word at 0xc0160c7c and writes two words to 0xfa200380 (stw at 0xc0134518 and 0xc0134520), reads 0xfa200280, and does a multiply and divide (mulli at 0xc013452c, divw at 0xc0134568 and 0xc0134578). It calls 0xc0010640 with r3=0xc00f011c and r5=0x3c at 0xc0134548. It calls 0xc0003300 at 0xc01345e0 with r3=0xf and r4=0xc000a584 and, on failure, reports through 0xc0010094 with r3=0xc00f0144. The call shape matches the one in sub_c00a23ec, which suggests a registration, but the callee is unnamed.

- `sub_c013d7e4` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc013d7e4
  **Inferred.** The largest routine in the batch, 1892 bytes, with a 0x60-byte frame and stmw at 0xc013d7ec. It fills a block from 0xc0198ab0 to 0xc0198c28 with more than fifty word stores and about forty halfword stores (for example at 0xc013d840, 0xc013d94c and 0xc013d9e0). It passes blocks to 0xc000c8d4 at 0xc013d888 and 0xc013d9b4 (the second with length 0xc4 from 0xc0198ab0), calls 0xc000cbe4 with the same 0xc0198ab0 block and length 0xc4 at 0xc013d82c, and calls 0xc00a25cc (0xc013dc40, 0xc013dcf4 with r3=0x20) and 0xc00a2600 (0xc013dc4c, 0xc013dd00 with r3=0x80). It walks halfword tables at 0xc012eac0 and 0xc012eabc (lhzx at 0xc013ddc4 and 0xc013deb0) and calls sub_c00a250c at 0xc013df04. The record's meaning and the callee names are not in the facts, so the reading stops at its shape.

- `sub_ffe83af8` in `6.5.24/usr/cpu/firmware/sysco/l1.bin(l1#2)` at 0xffe83af8
  **Inferred.** A ColdFire routine with a link.w frame and movem.l saves at 0xffe83b00. It sets a bit in the byte at 0x200007f8 (bset.b at 0xffe83b0e, bit number not in the facts) and reads the bytes at 0x200007f9 through 0x200007ff with move.b between 0xffe83b3c and 0xffe83be0, then clears a bit with bclr.b at 0xffe83bfa. It calls 0xffe7c7fc at 0xffe83cde after a run of compare and branch tests. The byte-flag pattern suggests a status poll, but the meaning of each bit is not in the facts.

- `sub_c000a5e0` in `6.5.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** Same rendered facts as the 6.5.23 build. A routine with a 0x20-byte frame that calls 0xc0004710 at 0xc000a5ec and reads the word at 0xfa200284 (lwz at 0xc000a5f4). It reads and writes the machine state register (mfmsr at 0xc000a5f8, mtmsr at 0xc000a608) around a byte update at an unresolved address, which may be a short critical section. It also calls 0xc0010640 with r3=0xc00f0164 at 0xc000a61c. The value written to the MSR is not in the facts, so this reading stops there.

- `sub_c00a2374` in `6.5.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** Same rendered facts as the 6.5.23 build. A routine with a 0x10-byte frame that stores to 0xfa200030 and to the words at 0xc01604ec, 0xc01604f0, 0xc01604f4 and 0xc01604f8, and writes 0xc01604e8. It calls 0xc000b454 at 0xc00a23b8 with r4=r3+4096, r5=0xfa200000, r6=0xc0160000 and r7=1, and it executes tlbia at 0xc00a23d0. The shape suggests a region mapping followed by a TLB flush, but the callee at 0xc000b454 is not named, so that purpose is unconfirmed.

- `sub_c00a23ec` in `6.5.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** Same rendered facts as the 6.5.23 build. A wrapper with a 0x10-byte frame that stores words to 0xfa200940 and 0xfa200948, then calls 0xc0003300 at 0xc00a2430 with r3=5 and r4=0xc00a247c, the address of sub_c00a247c. If that call fails (beq at 0xc00a2438), it reports through 0xc0010094 at 0xc00a2444 with r3=0xc010a1b8. It then calls sub_c00a250c at 0xc00a2458 and returns its result. A constant paired with a code address suggests a handler registration, but 0xc0003300 is not named in the facts.

- `sub_c00a247c` in `6.5.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** Same rendered facts as the 6.5.23 build. A routine with a 0x10-byte frame that writes the halfword at 0xfa200930 (sth at 0xc00a2494), reads it back (lhz at 0xc00a2498), walks two tables with indexed loads (lwzx at 0xc00a24ac and 0xc00a24c0), and calls through a register at 0xc00a24c8 (blrl, target unresolved). It stores the result at 0xfa20094c (0xc00a24f0). It is probably the callback passed as r4 by sub_c00a23ec at 0xc00a2430, but the facts do not settle what it computes.

- `sub_c00a250c` in `6.5.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** Same rendered facts as the 6.5.23 build. A routine with a 0x20-byte frame that saves a register range with stmw at 0xc00a2514 and restores it with lmw at 0xc00a2588. It calls sub_c00a2594 at 0xc00a252c, loads the word at 0xfa200948 (lwz at 0xc00a256c), and on one branch calls 0xc0010640 with r3=0xc010a1d4 at 0xc00a2558. The shift, rotate and or sequence between 0xc00a253c and 0xc00a2578 builds a bit-field value whose use the facts do not show.

- `sub_c00a2594` in `6.5.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** Same rendered facts as the 6.5.23 build. A leaf with no calls. It stores two words through indexed addresses (stwx at 0xc00a25a8 and 0xc00a25ac), reads the word at 0xfa200948 (lwz at 0xc00a25b4), stores one word at 0xc00a25c4, and returns an unresolved value at 0xc00a25c8. The shifts and rotates before the stores (slwi at 0xc00a25a4, rotlw at 0xc00a25bc) suggest a bit index into a table, but the table base is not resolved.

- `sub_c00a4e98` in `6.5.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** Same rendered facts as the 6.5.23 build. A routine with a 0x30-byte frame that saves registers with stmw at 0xc00a4ea0 and restores them with lmw at 0xc00a4fd4. It calls 0xc0010640 with r3=0xc010a7dc at 0xc00a4ecc, and on later paths calls 0xc0004868 (0xc00a4f38), 0xc00046d4 with 0xc0004710 (0xc00a4f40 and 0xc00a4f44) and 0xc00046e0 (0xc00a4fc0). Its tail is a load-reserve and store-conditional retry loop (lwarx at 0xc00a4fac, stwcx. at 0xc00a4fb4, retry branch at 0xc00a4fb8) on a word the facts do not locate. It reads 0xc0160178 (0xc00a4f58) and writes 0xfa200e54 (0xc00a4f6c).

- `sub_c00a515c` in `6.5.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** Same rendered facts as the 6.5.23 build. A large routine with a 0x40-byte frame that saves registers with stmw at 0xc00a5164. It first tests the word at 0xfa200e44 (lwz at 0xc00a5174, beq at 0xc00a5188), then walks several structures with word, halfword and byte loads and compare branches. Load-reserve and store-conditional sequences near 0xc00a52a8 and 0xc00a5368 update fields the facts do not resolve, and it calls 0xc0013dec twice (0xc00a52e0 and 0xc00a53a0), 0xc0013764 at 0xc00a53b0, and 0xc00a563c at 0xc00a53e8. Its overall job is not established by the facts.
