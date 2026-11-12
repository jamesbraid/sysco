# Tier-1 routine readings, sysco batch 4

Role statements for targets 100 to 124 of the tier-1 list, written from the rendered
instruction facts only (registers, calls, constants and branch targets). All 25 rendered
and passed the grader, so none were rejected. Every grade is inferred. The nine 6.5.17
entries at the end repeat the role text of their 6.5.16 entries: at those addresses the
rendered facts match once the build name is set aside. None of these routines has a
symbol in the image; the `sub_` names are the inventory's own.

- `sub_fe048ba0` in `6.5.16/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** A leaf block of 180 bytes that reads and writes fixed RAM fields at 0xfa200abc, 0xfa200ab8 and 0xfa200ac2 (0xfe048ba8 to 0xfe048bd4) and fills 0xfa203c80 to 0xfa203cac with halfword and byte stores. A byte-store loop at 0xfe048c10 is closed by the branch at 0xfe048c18 on the Z flag, and the routine stores to 0x705cd8 at 0xfe048c44 and to 0xfa200030 at 0xfe048c4c. It makes no calls and returns with r3 unknown at 0xfe048c50, so its purpose is a guess from the access pattern.

- `sub_fe048c54` in `6.5.16/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** A 476-byte routine with a 0x20-byte stack frame (stwu at 0xfe048c54) that reads the flag word at 0x705cd8 (0xfe048c74). The branch at 0xfe048c8c skips the call to sub_fe048ba0 at 0xfe048c90 when that flag is nonzero, so the call runs only when the flag is clear. It then walks halfword fields at 0xfa203c82 and 0xfa203c80 with bdnz loops at 0xfe048d44 and 0xfe048de4, calls 0xfe044770 at 0xfe048e08, and returns r3 unknown. The contents of the walked fields are not in the facts, so the role comes from the call and branch shape.

- `sub_c000a5e0` in `6.5.16/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** A 68-byte routine with a 0x20-byte frame that calls 0xc0004710 at 0xc000a5ec first. It reads a word at 0xfa200284 (0xc000a5f4) and then rewrites the MSR (mfmsr at 0xc000a5f8, rlwinm at 0xc000a604, mtmsr at 0xc000a608), changing one MSR bit by a mask the facts do not evaluate. After the mtmsr it loads and stores a byte through pointers at 0xc000a60c and 0xc000a614, and no second MSR write in this routine restores the old value. It ends with a call to 0xc0010640 with r3=0xc00f02d8 at 0xc000a61c. The byte, the effect of the MSR change and the callees are not identified in the facts.

- `sub_c00a2374` in `6.5.16/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** A 120-byte routine with a 0x10-byte frame that stores words to 0xc01624e8 through 0xc01624f8 (0xc00a23a8 to 0xc00a23d8) and to 0xfa200030 at 0xc00a2388. It calls 0xc000b454 at 0xc00a23b8 with r4=r3+4096, r5=0xfa200000, r7=1 and r6, r8, r9 and r10 set to 0xc0160000, which suggests a region setup of one 4 KB page. It executes tlbia at 0xc00a23d0, a TLB invalidate the renderer does not model, and returns r3 unknown.

- `sub_c00a23ec` in `6.5.16/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** A 144-byte routine with a 0x10-byte frame that stores words to 0xfa200940 and 0xfa200948 (0xc00a2414, 0xc00a2418, 0xc00a2464) and reloads 0xfa200940 at 0xc00a245c. It calls 0xc0003300 at 0xc00a2430 with r3=5 and r4=0xc00a247c, the address of sub_c00a247c, and branches at 0xc00a2438 on the returned value. It then calls 0xc0010094 with r3=0xc010b324 at 0xc00a2444 and sub_c00a250c at 0xc00a2458, returning that result. What 0xc0003300 does with code 5 and a function address is not in the facts.

- `sub_c00a247c` in `6.5.16/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** A 144-byte routine with a 0x10-byte frame that stores a halfword to 0xfa200930 at 0xc00a2494 and reloads it at 0xc00a2498. It walks tables with indexed loads (lwzx at 0xc00a24ac and 0xc00a24c0) and stores to 0xfa20094c at 0xc00a24f0. It makes an indirect call through a register with blrl at 0xc00a24c8, whose target the facts do not resolve. sub_c00a23ec passes this address to 0xc0003300, so it is probably a callback, but the table and callee contents are not in the facts.

- `sub_c00a250c` in `6.5.16/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** A 136-byte routine with a 0x20-byte frame that saves registers with stmw at 0xc00a2514 and restores them with lmw at 0xc00a2588. It calls the leaf sub_c00a2594 at 0xc00a252c and calls 0xc0010640 with r3=0xc010b340 at 0xc00a2558. It reads 0xfa200948 at 0xc00a256c, does indexed word stores at 0xc00a2560 and 0xc00a2564, and branches at 0xc00a2528 and 0xc00a2548 on flag and register tests. The facts do not show what those conditions mean, so the role is a shape reading.

- `sub_c00a2594` in `6.5.16/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** A 56-byte leaf routine with no calls. It loads the word at 0xfa200948 (0xc00a25b4), applies slwi at 0xc00a25a4 and rotlw and and at 0xc00a25bc and 0xc00a25c0, and stores through indexed words at 0xc00a25a8 and 0xc00a25ac and a word at 0xc00a25c4. It returns r3 unknown at 0xc00a25c8. The renderer does not model the rotate and mask, so the bit operation is inferred from the mnemonics.

- `sub_c00a4e98` in `6.5.16/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** A 328-byte routine with a 0x30-byte frame that saves registers with stmw at 0xc00a4ea0 and restores them with lmw at 0xc00a4fd4. Its lwarx and stwcx. retry loop at 0xc00a4fac to 0xc00a4fb8 suggests an atomic update of a shared word. It calls 0xc0010640 at 0xc00a4ecc with r3=0xc010b984, calls 0xc0004868 at 0xc00a4f38, calls 0xc00046d4 and 0xc0004710 at 0xc00a4f40 and 0xc00a4f44, and calls 0xc00046e0 at 0xc00a4fc0. The pattern may be a lock taken and released, but the callees are unnamed, so this is a guess.

- `sub_c00a515c` in `6.5.16/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** A 688-byte routine with a 0x40-byte frame, saving registers with stmw at 0xc00a5164 and restoring them with lmw at 0xc00a5400. It branches on the word at 0xfa200e44 (loaded at 0xc00a5174, beq at 0xc00a5188 to 0xc00a53f8), and runs chains of beq branches on halfword fields from 0xc00a51a4 to 0xc00a5288. It calls the local routine 0xc00a540c at 0xc00a51c4 and sub_c00a563c at 0xc00a53e8, and makes several helper calls (0xc0010640, 0xc00046d4, 0xc0004710, 0xc0013dec, 0xc0013764). Several lwarx and stwcx. sequences (0xc00a52a8, 0xc00a534c, 0xc00a5368) suggest atomic updates of shared words, but the meaning of the word at 0xfa200e44 is not in the facts.

- `sub_c00a563c` in `6.5.16/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** A 164-byte routine with a 0x20-byte frame that follows a pointer chain from the global at 0xc0162508: it loads that pointer at 0xc00a5650, then tests the words at +4 and +8 of it (beq at 0xc00a568c and 0xc00a56b8). It writes 0xc0162504 and 0xc0162508 (0xc00a56a8 to 0xc00a56b4) and stores 0xfa200e80 and 0xfa200e84 (0xc00a5664, 0xc00a56c4). It calls 0xc0010640 with r3=0xc010bab8 at 0xc00a567c and makes an indirect call with blrl at 0xc00a5698 through the pointer kept at 0xfa200e80. The record layout and the target of that call are not in the facts.

- `sub_c00a56e0` in `6.5.16/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** A 184-byte routine with a 0x30-byte frame that saves registers with stmw at 0xc00a56e8 and restores them with lmw at 0xc00a578c. It reads 0xc0162504, 0xc0162508 and 0xc016250c (0xc00a5708, 0xc00a571c, 0xc00a5740) and rewrites those globals and their fields (0xc00a5728 to 0xc00a5758). It stores 0xfa200e84 and 0xfa200e80 at 0xc00a5768 and 0xc00a576c, and calls 0xc00046d4, 0xc0004710 and 0xc00046e0 at 0xc00a56fc, 0xc00a5700 and 0xc00a577c. It uses the same globals as sub_c00a563c, which suggests a reset or teardown counterpart, but the facts do not settle that.

- `sub_c00a5eb0` in `6.5.16/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** A 568-byte routine with a 0x20-byte frame that saves registers with stmw at 0xc00a5eb8 and restores them with lmw at 0xc00a60dc. It stores four words to 0xfa200e40, 0xfa200e48, 0xfa200e44 and 0xfa200e4c (0xc00a5eec to 0xc00a5f0c) and reads halfwords at 0xc0130ac8 and 0xc0130aca (0xc00a5f14, 0xc00a5f1c). It runs loops closed by bdnz at 0xc00a5f58, 0xc00a6018 and 0xc00a6048, calls sub_c00a56e0 at 0xc00a60c0 with r3=0x600a0000 and r4=0xc00a5a60, and calls 0xc00046a0, 0xc00aa0a0 and 0xc0004c2c on the way. It looks like table or device setup, but the facts do not settle it.

- `sub_ffe5d9b8` in `6.5.17/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5d9b8
  **Inferred.** A 544-byte ColdFire routine with a link.w frame and movem.l saves at 0xffe5d9c0, restored at 0xffe5dbce. It range-checks six fields of the record at [sp+4]: the field at +0x14 must lie from 0x1e to 0x258, and the fields at +0x10, +0xc, +0x8 and +0x4 are bounded above by 0xc, 0x1f, 0x18 and 0x3c, with out-of-range branches to 0xffe5da08. It calls 0xffe5f4a8 at 0xffe5da14 with the record fields, and calls 0xffe5f698 at 0xffe5da24 and 0xffe5da42, testing its result at 0xffe5da32 and updating counters at its +0x10, +0x14 and +0x18. The record layout and the callees are not identified in the facts, so the role is a guess from the checks.

- `sub_fe004efc` in `6.5.17/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** A 156-byte routine with a 0x20-byte frame that stores a word to 0xfa200004 at 0xfe004f84. It calls 0xfe0051f0 at 0xfe004f24 with r3=0xe01868 and r9=0xfe040000, calls 0xfe004610 six times with constant addresses 0xe013ec, 0xe01418, 0xe01428, 0xe01454, 0xe01428 again and 0xe01474 (0xfe004f30 to 0xfe004f8c), and calls 0xfe004ff4 at 0xfe004f38 with r3=0xfe040000. The branch at 0xfe004f40 skips to 0xfe004f78 when that result is nonzero, and the routine ends by calling 0xfe0046c8 with the result of 0xfe004610. The constants look like data addresses, and the role reads as a start-up sequence, which is a guess.

- `sub_fe0051a0` in `6.5.17/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** An 80-byte leaf routine with no calls that stores halfwords to 0xfa200950, 0xfa200952, 0xfa200954 and 0xfa200960 through 0xfa200966 (0xfe0051a8 to 0xfe0051e8), and words to 0xfa200abc, 0xfa200ab8 and 0xfa200ac4 (0xfe0051bc to 0xfe0051d0), with a halfword to 0xfa200ac2 at 0xfe0051c8. The stored values are not in the facts. Those RAM fields are the ones sub_fe048ba0 reads and writes at 0xfe048ba8 to 0xfe048bd4, so this is likely setup for it, and the routine returns r3 unchanged at 0xfe0051ec.

- `sub_fe048ba0` in `6.5.17/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** A leaf block of 180 bytes that reads and writes fixed RAM fields at 0xfa200abc, 0xfa200ab8 and 0xfa200ac2 (0xfe048ba8 to 0xfe048bd4) and fills 0xfa203c80 to 0xfa203cac with halfword and byte stores. A byte-store loop at 0xfe048c10 is closed by the branch at 0xfe048c18 on the Z flag, and the routine stores to 0x705cd8 at 0xfe048c44 and to 0xfa200030 at 0xfe048c4c. It makes no calls and returns with r3 unknown at 0xfe048c50, so its purpose is a guess from the access pattern.

- `sub_fe048c54` in `6.5.17/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** A 476-byte routine with a 0x20-byte stack frame (stwu at 0xfe048c54) that reads the flag word at 0x705cd8 (0xfe048c74). The branch at 0xfe048c8c skips the call to sub_fe048ba0 at 0xfe048c90 when that flag is nonzero, so the call runs only when the flag is clear. It then walks halfword fields at 0xfa203c82 and 0xfa203c80 with bdnz loops at 0xfe048d44 and 0xfe048de4, calls 0xfe044770 at 0xfe048e08, and returns r3 unknown. The contents of the walked fields are not in the facts, so the role comes from the call and branch shape.

- `sub_c000a5e0` in `6.5.17/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** A 68-byte routine with a 0x20-byte frame that calls 0xc0004710 at 0xc000a5ec first. It reads a word at 0xfa200284 (0xc000a5f4) and then rewrites the MSR (mfmsr at 0xc000a5f8, rlwinm at 0xc000a604, mtmsr at 0xc000a608), changing one MSR bit by a mask the facts do not evaluate. After the mtmsr it loads and stores a byte through pointers at 0xc000a60c and 0xc000a614, and no second MSR write in this routine restores the old value. It ends with a call to 0xc0010640 with r3=0xc00f02d8 at 0xc000a61c. The byte, the effect of the MSR change and the callees are not identified in the facts.

- `sub_c00a2374` in `6.5.17/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** A 120-byte routine with a 0x10-byte frame that stores words to 0xc01624e8 through 0xc01624f8 (0xc00a23a8 to 0xc00a23d8) and to 0xfa200030 at 0xc00a2388. It calls 0xc000b454 at 0xc00a23b8 with r4=r3+4096, r5=0xfa200000, r7=1 and r6, r8, r9 and r10 set to 0xc0160000, which suggests a region setup of one 4 KB page. It executes tlbia at 0xc00a23d0, a TLB invalidate the renderer does not model, and returns r3 unknown.

- `sub_c00a23ec` in `6.5.17/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** A 144-byte routine with a 0x10-byte frame that stores words to 0xfa200940 and 0xfa200948 (0xc00a2414, 0xc00a2418, 0xc00a2464) and reloads 0xfa200940 at 0xc00a245c. It calls 0xc0003300 at 0xc00a2430 with r3=5 and r4=0xc00a247c, the address of sub_c00a247c, and branches at 0xc00a2438 on the returned value. It then calls 0xc0010094 with r3=0xc010b324 at 0xc00a2444 and sub_c00a250c at 0xc00a2458, returning that result. What 0xc0003300 does with code 5 and a function address is not in the facts.

- `sub_c00a247c` in `6.5.17/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** A 144-byte routine with a 0x10-byte frame that stores a halfword to 0xfa200930 at 0xc00a2494 and reloads it at 0xc00a2498. It walks tables with indexed loads (lwzx at 0xc00a24ac and 0xc00a24c0) and stores to 0xfa20094c at 0xc00a24f0. It makes an indirect call through a register with blrl at 0xc00a24c8, whose target the facts do not resolve. sub_c00a23ec passes this address to 0xc0003300, so it is probably a callback, but the table and callee contents are not in the facts.

- `sub_c00a250c` in `6.5.17/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** A 136-byte routine with a 0x20-byte frame that saves registers with stmw at 0xc00a2514 and restores them with lmw at 0xc00a2588. It calls the leaf sub_c00a2594 at 0xc00a252c and calls 0xc0010640 with r3=0xc010b340 at 0xc00a2558. It reads 0xfa200948 at 0xc00a256c, does indexed word stores at 0xc00a2560 and 0xc00a2564, and branches at 0xc00a2528 and 0xc00a2548 on flag and register tests. The facts do not show what those conditions mean, so the role is a shape reading.

- `sub_c00a2594` in `6.5.17/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** A 56-byte leaf routine with no calls. It loads the word at 0xfa200948 (0xc00a25b4), applies slwi at 0xc00a25a4 and rotlw and and at 0xc00a25bc and 0xc00a25c0, and stores through indexed words at 0xc00a25a8 and 0xc00a25ac and a word at 0xc00a25c4. It returns r3 unknown at 0xc00a25c8. The renderer does not model the rotate and mask, so the bit operation is inferred from the mnemonics.

- `sub_c00a4e98` in `6.5.17/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** A 328-byte routine with a 0x30-byte frame that saves registers with stmw at 0xc00a4ea0 and restores them with lmw at 0xc00a4fd4. Its lwarx and stwcx. retry loop at 0xc00a4fac to 0xc00a4fb8 suggests an atomic update of a shared word. It calls 0xc0010640 at 0xc00a4ecc with r3=0xc010b984, calls 0xc0004868 at 0xc00a4f38, calls 0xc00046d4 and 0xc0004710 at 0xc00a4f40 and 0xc00a4f44, and calls 0xc00046e0 at 0xc00a4fc0. The pattern may be a lock taken and released, but the callees are unnamed, so this is a guess.
