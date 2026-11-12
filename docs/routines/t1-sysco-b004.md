# Tier-1 routine readings, sysco batch 4

Role statements for targets 100 to 124 of the tier-1 list, written from the rendered
instruction facts only (registers, calls, constants and branch targets). All 25 rendered
and passed the grader, so none were rejected. Every grade is inferred. The nine 6.5.17
entries at the end repeat the role text of their 6.5.16 entries: at those addresses the
rendered facts match once the build name is set aside. None of these routines has a
symbol in the image; the `sub_` names are the inventory's own.

- `sub_fe048ba0` in `6.5.16/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** A leaf block of 180 bytes that reads and writes fixed RAM fields at 0xfa200abc, 0xfa200ab8 and 0xfa200ac2 (0xfe048ba8 to 0xfe048bd4) and fills 0xfa203c80 to 0xfa203cac with halfword and byte stores. After it stores 0x11 to the halfword at 0xfa2009c0 (0xfe048c08), the loop at 0xfe048c10 reloads that halfword and repeats while its low bit is set (the branch at 0xfe048c18 follows the andi. at 0xfe048c14); that loop stores nothing. The routine stores to 0x705cd8 at 0xfe048c44 and to 0xfa200030 at 0xfe048c4c. It makes no calls and returns with r3 unknown at 0xfe048c50, so its purpose is a guess from the access pattern.
  Reviewed: 2026-11-12

- `sub_fe048c54` in `6.5.16/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** A 476-byte routine with a 0x20-byte stack frame (stwu at 0xfe048c54) that reads the flag word at 0x705cd8 (0xfe048c74). The branch at 0xfe048c8c skips the call to sub_fe048ba0 at 0xfe048c90 when that flag is nonzero, so the call runs only when the flag is clear. It loads the halfwords at 0xfa203c82 and 0xfa203c80 (0xfe048c9c, 0xfe048cb8) and uses them as offsets from 0xfa2009c0 plus 0x1640. Two loops closed by bdnz at 0xfe048d44 and 0xfe048de4 reload a halfword at such an address and leave when its 0x8000 bit is clear, each limited to 0x989680 iterations. It calls 0xfe044770 at 0xfe048e08 on the paths where a loop ran out (r3=0x705720 or 0x705740), and returns either the halfword loaded at 0xfe048df8 or -1 (0xfe048e0c). The contents of the polled fields are not in the facts, so the role comes from the call and branch shape.
  Reviewed: 2026-11-12

- `sub_c000a5e0` in `6.5.16/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** A 68-byte routine with a 0x20-byte frame that calls 0xc0004710 at 0xc000a5ec first. It loads the word at 0xfa200284 (0xc000a5f4), stores it back with 0x80 or-ed in (0xc000a600), and rewrites the MSR (mfmsr at 0xc000a5f8, rlwinm at 0xc000a604, mtmsr at 0xc000a608), clearing one MSR bit by a mask the facts do not evaluate. After the mtmsr it loads a byte from 0xfa20028c (0xc000a60c) and stores it to the stack frame (0xc000a614), and no second MSR write in this routine restores the old value. It ends with a call to 0xc0010640 with r3=0xc00f02d8 at 0xc000a61c, then branches to itself at 0xc000a620 and does not return. The byte, the effect of the MSR change and the callees are not identified in the facts.
  Reviewed: 2026-11-12

- `sub_c00a2374` in `6.5.16/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** A 120-byte routine with a 0x10-byte frame that stores words to 0xc01624e8 through 0xc01624f8 (0xc00a23a8 to 0xc00a23d8) and to 0xfa200030 at 0xc00a2388. It calls 0xc000b454 at 0xc00a23b8 with r4=r3+4096, r5=0xfa200000, r7=1 and r6, r8, r9 and r10 set to 0xc0160000, which suggests a region setup of one 4 KB page. It executes tlbia at 0xc00a23d0, a TLB invalidate the renderer does not model, and returns r3 unknown.
  Reviewed: 2026-11-12

- `sub_c00a23ec` in `6.5.16/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** A 144-byte routine with a 0x10-byte frame that stores words to 0xfa200940 and 0xfa200948 (0xc00a2414, 0xc00a2418, 0xc00a2464) and reloads 0xfa200940 at 0xc00a245c. It calls 0xc0003300 at 0xc00a2430 with r3=5 and r4=0xc00a247c, the address of sub_c00a247c, and branches at 0xc00a2438 on the returned value: it calls 0xc0010094 with r3=0xc010b324 at 0xc00a2444 only when that value is nonzero. It then calls sub_c00a250c at 0xc00a2458 with r3=0, r4=0xc00a2508 and r5=0, or-s 0x80 into 0xfa200940 and returns with r3 as sub_c00a250c left it. What 0xc0003300 does with code 5 and a function address is not in the facts.
  Reviewed: 2026-11-12

- `sub_c00a247c` in `6.5.16/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** A 144-byte routine with a 0x10-byte frame that stores a halfword to 0xfa200930 at 0xc00a2494 and reloads it at 0xc00a2498. It shifts the reloaded value right by 11 and uses it to index a table of 8-byte entries (lwzx at 0xc00a24ac and 0xc00a24c0). When the first word of the entry is nonzero it loads the second word into r3 and makes an indirect call through the first word with blrl at 0xc00a24c8, whose target the facts do not resolve. When it is zero the routine clears one bit of the word at 0xfa200948 instead (0xc00a24d0 to 0xc00a24e0). It stores to 0xfa20094c at 0xc00a24f0 on both paths. sub_c00a23ec passes this address to 0xc0003300, so it is probably a callback, but the table and callee contents are not in the facts.
  Reviewed: 2026-11-12

- `sub_c00a250c` in `6.5.16/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** A 136-byte routine with a 0x20-byte frame that saves registers with stmw at 0xc00a2514 and restores them with lmw at 0xc00a2588. The branch at 0xc00a2528 tests r4: when it is zero the routine calls the leaf sub_c00a2594 at 0xc00a252c and returns. Otherwise it uses r3 to index a table of 8-byte entries; when the entry's first word is nonzero (branch at 0xc00a2548) it calls 0xc0010640 with r3=0xc010b340 at 0xc00a2558. It then does indexed word stores of r5 and r4 into the entry at 0xc00a2560 and 0xc00a2564, and reads and rewrites the word at 0xfa200948 (load at 0xc00a256c). The facts do not show what the conditions mean, so the role is a shape reading.
  Reviewed: 2026-11-12

- `sub_c00a2594` in `6.5.16/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** A 56-byte leaf routine with no calls. It loads the word at 0xfa200948 (0xc00a25b4), applies slwi at 0xc00a25a4 and rotlw and and at 0xc00a25bc and 0xc00a25c0, and stores through indexed words at 0xc00a25a8 and 0xc00a25ac and a word at 0xc00a25c4. It returns r3 unknown at 0xc00a25c8. The renderer does not model the rotate and mask, so the bit operation is inferred from the mnemonics.
  Reviewed: 2026-11-12

- `sub_c00a4e98` in `6.5.16/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** A 328-byte routine with a 0x30-byte frame that saves registers with stmw at 0xc00a4ea0 and restores them with lmw at 0xc00a4fd4. It loads a halfword through a pointer chain from r4 (offsets 0x60, then 0x30) at 0xc00a4eb8 and branches at 0xc00a4ec0 on its 0x8000 bit. When that bit is set it calls 0xc0010640 at 0xc00a4ecc with r3=0xc010b984, stores zero at +0x9c of the record at r4+0x60, and returns 1. When it is clear it goes on to call 0xc0004868 at 0xc00a4f38, 0xc00046d4 and 0xc0004710 at 0xc00a4f40 and 0xc00a4f44, and 0xc00046e0 at 0xc00a4fc0, and returns 0. Its lwarx and stwcx. retry loop at 0xc00a4fac to 0xc00a4fb8 runs only when a second halfword has the 0x8000 bit set (0xc00a4f94) and suggests an atomic or of 1 into the word at r4+0x2c. The call pattern may be a lock taken and released, but the callees are unnamed, so this is a guess.
  Reviewed: 2026-11-12

- `sub_c00a515c` in `6.5.16/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** A 688-byte routine with a 0x40-byte frame, saving registers with stmw at 0xc00a5164 and restoring them with lmw at 0xc00a5400. It loads the word at 0xfa200e44 (0xc00a5174) and returns at once when it is zero (beq at 0xc00a5188 to 0xc00a53f8); otherwise it loops, reloading that word at 0xc00a53ec. Bit tests on the word guard three calls: 0xc0010640 at 0xc00a51b4 (mask 0xe0400000), the local routine 0xc00a540c at 0xc00a51c4 (mask 0x03000000) and sub_c00a563c at 0xc00a53e8 (mask 0x00800000). With mask 0x0c000000 set it runs chains of beq branches on bits of halfword fields (0xc00a51dc to 0xc00a5288) that increment counters, and makes further helper calls (0xc0010640 at 0xc00a527c, 0xc00046d4, 0xc0004710, 0xc0013dec, 0xc0013764). Several lwarx and stwcx. sequences (0xc00a52a8, 0xc00a534c, 0xc00a5368) suggest atomic updates of shared words, but the meaning of the word at 0xfa200e44 is not in the facts.
  Reviewed: 2026-11-12

- `sub_c00a563c` in `6.5.16/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** A 164-byte routine with a 0x20-byte frame that follows a pointer chain from the global at 0xc0162508: it loads that pointer at 0xc00a5650, stores zero to 0xfa200e84 (0xc00a5664) and loads 0xfa200e80 (0xc00a5668). When the pointer is zero it calls 0xc0010640 with r3=0xc010bab8 at 0xc00a567c and returns. Otherwise it tests the word at +4 of the record (beq at 0xc00a568c) and, when it is nonzero, makes an indirect call through that word with blrl at 0xc00a5698, passing the value loaded from 0xfa200e80 as r3. It then writes 0xc0162504 and 0xc0162508 (0xc00a56a8 to 0xc00a56b4) and, when the record's +8 word is nonzero (beq at 0xc00a56b8), stores 0xfa200e80 and 0xfa200e84 (0xc00a56c4, 0xc00a56c8). The record layout and the target of that call are not in the facts.
  Reviewed: 2026-11-12

- `sub_c00a56e0` in `6.5.16/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** A 184-byte routine with a 0x30-byte frame that saves registers with stmw at 0xc00a56e8 and restores them with lmw at 0xc00a578c. It reads 0xc0162504, 0xc0162508 and 0xc016250c (0xc00a5708, 0xc00a571c, 0xc00a5740) and rewrites those globals and the fields of one record (0xc00a5728 to 0xc00a5758). It returns 1 when the word at 0xc0162504 is zero (0xc00a5774) and 0 otherwise. It stores 0xfa200e84 and 0xfa200e80 at 0xc00a5768 and 0xc00a576c only when the word at 0xc0162508 is zero, and calls 0xc00046d4, 0xc0004710 and 0xc00046e0 at 0xc00a56fc, 0xc00a5700 and 0xc00a577c. It uses the same globals as sub_c00a563c, which moves a record from the list at 0xc0162508 to the one at 0xc0162504, and this routine moves one the other way, so the two look like a pair, but the facts do not settle what the records are.
  Reviewed: 2026-11-12

- `sub_c00a5eb0` in `6.5.16/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** A 568-byte routine with a 0x20-byte frame that saves registers with stmw at 0xc00a5eb8 and restores them with lmw at 0xc00a60dc. It stores four words to 0xfa200e40, 0xfa200e48, 0xfa200e44 and 0xfa200e4c (0xc00a5eec to 0xc00a5f0c) and reads halfwords at 0xc0130ac8 and 0xc0130aca (0xc00a5f14, 0xc00a5f1c). It runs loops closed by bdnz at 0xc00a5f58, 0xc00a6018 and 0xc00a6048, calls sub_c00a56e0 at 0xc00a60c0 with r3=0x600a0000 and r4=0xc00a5a60, and calls 0xc00046a0, 0xc00aa0a0 and 0xc0004c2c on the way. It looks like table or device setup, but the facts do not settle it.
  Reviewed: 2026-11-12

- `sub_ffe5d9b8` in `6.5.17/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5d9b8
  **Inferred.** A 544-byte ColdFire routine with a link.w frame and movem.l saves at 0xffe5d9c0, restored at 0xffe5dbce. It range-checks six fields of the record at [sp+4]: the field at +0x14 must be at most 0x258 and, when positive, at least 0x1e (zero or below passes), and the fields at +0x10, +0xc, +0x8, +0x4 and +0 are bounded above by 0xc, 0x1f, 0x18, 0x3c and 0x3d, with out-of-range branches to 0xffe5da08, which returns 0xfffff7cc. It calls 0xffe5f4a8 at 0xffe5da14 with the record pointer, calls 0xffe5f698 at 0xffe5da24 with the address of that result, and calls 0xffe5f698 again at 0xffe5da42 only when the word at +0x20 of the record it returned is nonzero (beq at 0xffe5da32). It then adds 0x76c to the word at +0x14 of the returned record, increments its words at +0x10 and +0x18, and writes bytes at 0x200007f8 to 0x200007ff from the fields divided by 10. The record layout and the callees are not identified in the facts, so the role is a guess from the checks.
  Reviewed: 2026-11-12

- `sub_fe004efc` in `6.5.17/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** A 156-byte routine with a 0x20-byte frame that stores a word to 0xfa200004 at 0xfe004f84 on every path. It calls 0xfe0051f0 at 0xfe004f24 with r3=0xe01868 and r9=0xfe040000, calls 0xfe004610 at 0xfe004f30 with r3=0xe013ec, and calls 0xfe004ff4 at 0xfe004f38 with r3=0xfe040000. The branch at 0xfe004f40 skips to 0xfe004f78 when that result is nonzero; otherwise it calls 0xfe004610 with 0xe01418, then 0xfe004f98 at 0xfe004f54 with r3=0xfe040000, then 0xfe004610 with 0xe01428, 0xe01454 and 0xe01428 again (0xfe004f4c to 0xfe004f74). On both paths it calls 0xfe004610 with 0xe01474 at 0xfe004f8c, then 0xfe0046c8 at 0xfe004f90 with the result of that call, and branches to itself at 0xfe004f94, so it does not return. The constants look like data addresses, and the role reads as a start-up sequence, which is a guess.
  Reviewed: 2026-11-12

- `sub_fe0051a0` in `6.5.17/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** An 80-byte leaf routine with no calls that stores halfwords to 0xfa200950, 0xfa200952, 0xfa200954 and 0xfa200960 through 0xfa200966 (0xfe0051a8 to 0xfe0051e8), and words to 0xfa200abc, 0xfa200ab8 and 0xfa200ac4 (0xfe0051bc to 0xfe0051d0), with a halfword to 0xfa200ac2 at 0xfe0051c8. The stored values are small immediates. Three of those fields, 0xfa200abc, 0xfa200ab8 and 0xfa200ac2, are also read and written by sub_fe048ba0 at 0xfe048ba8 to 0xfe048bd4, so this may be setup for it, but the other fields are not touched there. The routine returns r3 unchanged at 0xfe0051ec.
  Reviewed: 2026-11-12

- `sub_fe048ba0` in `6.5.17/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** A leaf block of 180 bytes that reads and writes fixed RAM fields at 0xfa200abc, 0xfa200ab8 and 0xfa200ac2 (0xfe048ba8 to 0xfe048bd4) and fills 0xfa203c80 to 0xfa203cac with halfword and byte stores. After it stores 0x11 to the halfword at 0xfa2009c0 (0xfe048c08), the loop at 0xfe048c10 reloads that halfword and repeats while its low bit is set (the branch at 0xfe048c18 follows the andi. at 0xfe048c14); that loop stores nothing. The routine stores to 0x705cd8 at 0xfe048c44 and to 0xfa200030 at 0xfe048c4c. It makes no calls and returns with r3 unknown at 0xfe048c50, so its purpose is a guess from the access pattern.
  Reviewed: 2026-11-12

- `sub_fe048c54` in `6.5.17/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** A 476-byte routine with a 0x20-byte stack frame (stwu at 0xfe048c54) that reads the flag word at 0x705cd8 (0xfe048c74). The branch at 0xfe048c8c skips the call to sub_fe048ba0 at 0xfe048c90 when that flag is nonzero, so the call runs only when the flag is clear. It loads the halfwords at 0xfa203c82 and 0xfa203c80 (0xfe048c9c, 0xfe048cb8) and uses them as offsets from 0xfa2009c0 plus 0x1640. Two loops closed by bdnz at 0xfe048d44 and 0xfe048de4 reload a halfword at such an address and leave when its 0x8000 bit is clear, each limited to 0x989680 iterations. It calls 0xfe044770 at 0xfe048e08 on the paths where a loop ran out (r3=0x705720 or 0x705740), and returns either the halfword loaded at 0xfe048df8 or -1 (0xfe048e0c). The contents of the polled fields are not in the facts, so the role comes from the call and branch shape.
  Reviewed: 2026-11-12

- `sub_c000a5e0` in `6.5.17/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** A 68-byte routine with a 0x20-byte frame that calls 0xc0004710 at 0xc000a5ec first. It loads the word at 0xfa200284 (0xc000a5f4), stores it back with 0x80 or-ed in (0xc000a600), and rewrites the MSR (mfmsr at 0xc000a5f8, rlwinm at 0xc000a604, mtmsr at 0xc000a608), clearing one MSR bit by a mask the facts do not evaluate. After the mtmsr it loads a byte from 0xfa20028c (0xc000a60c) and stores it to the stack frame (0xc000a614), and no second MSR write in this routine restores the old value. It ends with a call to 0xc0010640 with r3=0xc00f02d8 at 0xc000a61c, then branches to itself at 0xc000a620 and does not return. The byte, the effect of the MSR change and the callees are not identified in the facts.
  Reviewed: 2026-11-12

- `sub_c00a2374` in `6.5.17/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** A 120-byte routine with a 0x10-byte frame that stores words to 0xc01624e8 through 0xc01624f8 (0xc00a23a8 to 0xc00a23d8) and to 0xfa200030 at 0xc00a2388. It calls 0xc000b454 at 0xc00a23b8 with r4=r3+4096, r5=0xfa200000, r7=1 and r6, r8, r9 and r10 set to 0xc0160000, which suggests a region setup of one 4 KB page. It executes tlbia at 0xc00a23d0, a TLB invalidate the renderer does not model, and returns r3 unknown.
  Reviewed: 2026-11-12

- `sub_c00a23ec` in `6.5.17/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** A 144-byte routine with a 0x10-byte frame that stores words to 0xfa200940 and 0xfa200948 (0xc00a2414, 0xc00a2418, 0xc00a2464) and reloads 0xfa200940 at 0xc00a245c. It calls 0xc0003300 at 0xc00a2430 with r3=5 and r4=0xc00a247c, the address of sub_c00a247c, and branches at 0xc00a2438 on the returned value: it calls 0xc0010094 with r3=0xc010b324 at 0xc00a2444 only when that value is nonzero. It then calls sub_c00a250c at 0xc00a2458 with r3=0, r4=0xc00a2508 and r5=0, or-s 0x80 into 0xfa200940 and returns with r3 as sub_c00a250c left it. What 0xc0003300 does with code 5 and a function address is not in the facts.
  Reviewed: 2026-11-12

- `sub_c00a247c` in `6.5.17/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** A 144-byte routine with a 0x10-byte frame that stores a halfword to 0xfa200930 at 0xc00a2494 and reloads it at 0xc00a2498. It shifts the reloaded value right by 11 and uses it to index a table of 8-byte entries (lwzx at 0xc00a24ac and 0xc00a24c0). When the first word of the entry is nonzero it loads the second word into r3 and makes an indirect call through the first word with blrl at 0xc00a24c8, whose target the facts do not resolve. When it is zero the routine clears one bit of the word at 0xfa200948 instead (0xc00a24d0 to 0xc00a24e0). It stores to 0xfa20094c at 0xc00a24f0 on both paths. sub_c00a23ec passes this address to 0xc0003300, so it is probably a callback, but the table and callee contents are not in the facts.
  Reviewed: 2026-11-12

- `sub_c00a250c` in `6.5.17/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** A 136-byte routine with a 0x20-byte frame that saves registers with stmw at 0xc00a2514 and restores them with lmw at 0xc00a2588. The branch at 0xc00a2528 tests r4: when it is zero the routine calls the leaf sub_c00a2594 at 0xc00a252c and returns. Otherwise it uses r3 to index a table of 8-byte entries; when the entry's first word is nonzero (branch at 0xc00a2548) it calls 0xc0010640 with r3=0xc010b340 at 0xc00a2558. It then does indexed word stores of r5 and r4 into the entry at 0xc00a2560 and 0xc00a2564, and reads and rewrites the word at 0xfa200948 (load at 0xc00a256c). The facts do not show what the conditions mean, so the role is a shape reading.
  Reviewed: 2026-11-12

- `sub_c00a2594` in `6.5.17/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** A 56-byte leaf routine with no calls. It loads the word at 0xfa200948 (0xc00a25b4), applies slwi at 0xc00a25a4 and rotlw and and at 0xc00a25bc and 0xc00a25c0, and stores through indexed words at 0xc00a25a8 and 0xc00a25ac and a word at 0xc00a25c4. It returns r3 unknown at 0xc00a25c8. The renderer does not model the rotate and mask, so the bit operation is inferred from the mnemonics.
  Reviewed: 2026-11-12

- `sub_c00a4e98` in `6.5.17/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** A 328-byte routine with a 0x30-byte frame that saves registers with stmw at 0xc00a4ea0 and restores them with lmw at 0xc00a4fd4. It loads a halfword through a pointer chain from r4 (offsets 0x60, then 0x30) at 0xc00a4eb8 and branches at 0xc00a4ec0 on its 0x8000 bit. When that bit is set it calls 0xc0010640 at 0xc00a4ecc with r3=0xc010b984, stores zero at +0x9c of the record at r4+0x60, and returns 1. When it is clear it goes on to call 0xc0004868 at 0xc00a4f38, 0xc00046d4 and 0xc0004710 at 0xc00a4f40 and 0xc00a4f44, and 0xc00046e0 at 0xc00a4fc0, and returns 0. Its lwarx and stwcx. retry loop at 0xc00a4fac to 0xc00a4fb8 runs only when a second halfword has the 0x8000 bit set (0xc00a4f94) and suggests an atomic or of 1 into the word at r4+0x2c. The call pattern may be a lock taken and released, but the callees are unnamed, so this is a guess.
  Reviewed: 2026-11-12
