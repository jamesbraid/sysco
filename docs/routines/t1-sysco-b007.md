# Tier-1 routine readings, sysco batch 7

Role statements for targets 176 to 200 of the tier-1 list (`--from-grade located --hardware-only`,
ranked order), written from the rendered instruction facts only: registers, calls, constants and
branch targets. Every grade is inferred, because none of these routines has a name or a documented
contract that settles its role. The 6.5.21 kernel entries have the same instruction bodies as the
6.5.20 build at the same address, so their statements repeat.

- `sub_c00a23ec` in `6.5.20/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** Sets up a 16-byte stack frame, stores to abs 0xfa200940 (0xc00a2414, 0xc00a2464) and abs 0xfa200948 (0xc00a2418), then calls 0xc0003300 with r3=5 and r4=0xc00a247c, the next routine in this batch, passed as a function address. When that call returns zero (beq at 0xc00a2438) it skips the call to 0xc0010094, then calls 0xc00a250c and returns that call's result. The callee at 0xc0003300 is unnamed, so the registration reading is a guess from the argument shape.

- `sub_c00a247c` in `6.5.20/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** Sets up a 16-byte frame, stores a halfword to abs 0xfa200930 at 0xc00a2494 and reads it back at 0xc00a2498, then loads several words through computed addresses (lwzx at 0xc00a24ac and 0xc00a24c0). It makes an indirect call at 0xc00a24c8 (blrl, target not resolved) and branches on a loaded value at 0xc00a24b8, then stores to abs 0xfa20094c at 0xc00a24f0. This looks like a dispatch on a 16-bit field, but the table and the target are not in the facts.

- `sub_c00a250c` in `6.5.20/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** Saves registers with stmw and sets up a 32-byte frame, calls 0xc00a2594 at 0xc00a252c, and branches on a compare at 0xc00a2528 and 0xc00a2548. It calls 0xc0010640 with the constant pointer 0xc010a1d4 at 0xc00a2558, the same call shape other routines in this batch use with constant pointers, so 0xc0010640 may be a print or log function. It loads a word from abs 0xfa200948 at 0xc00a256c and restores the saved registers with lmw at 0xc00a2588.

- `sub_c00a2594` in `6.5.20/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** A 56-byte leaf routine with no calls. It stores two words through computed addresses (stwx at 0xc00a25a8 and 0xc00a25ac), reads a word from abs 0xfa200948 at 0xc00a25b4, stores a result at 0xc00a25c4, and returns an unresolved value at 0xc00a25c8. The stores go to addresses the renderer could not resolve, so its effect is limited to the word it reads from 0xfa200948.

- `sub_c00a4e98` in `6.5.20/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** Sets up a 48-byte frame with stmw, then runs a sequence of halfword loads and stores (lhz at 0xc00a4eb8 and 0xc00a4ee0, sth at 0xc00a4ee8 and 0xc00a4efc) through computed addresses. Near the end it has a load-reserve and store-conditional pair (lwarx at 0xc00a4fac, stwcx. at 0xc00a4fb4) with a retry branch at 0xc00a4fb8, which suggests an atomic update of a shared word. It calls 0xc0004868, the pair 0xc00046d4 and 0xc0004710, and 0xc00046e0, and writes abs 0xfa200e54 at 0xc00a4f6c.

- `sub_c00a515c` in `6.5.20/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** A 688-byte routine with a 64-byte frame. On entry it reads abs 0xfa200e44 (0xc00a5174) and branches around its main body at 0xc00a5188 when that word is zero. The body is a long loop over many lhz and lwz accesses through computed addresses, and it calls 0xc0010640 with the constant pointers 0xc010a894 and 0xc010a8a4. It calls its neighbours 0xc00a540c and 0xc00a563c, plus 0xc0013dec and 0xc0013764, and the pair 0xc00046d4 and 0xc0004710 with 0xc00046e0. The facts do not show what the loop processes.

- `sub_c00a563c` in `6.5.20/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** Sets up a 32-byte frame and walks a pointer chain from the global at 0xc0160508, testing its +4 and +8 words at 0xc00a568c and 0xc00a56b8. It stores to the MMIO-range words abs 0xfa200e80 and 0xfa200e84 (0xc00a5664, 0xc00a56c4, 0xc00a56c8) and writes the list words 0xc0160504 and 0xc0160508 at 0xc00a56a8 and 0xc00a56b4. It makes an indirect call at 0xc00a5698 through the value loaded from abs 0xfa200e80, and calls 0xc0010640 with the constant 0xc010a910.

- `sub_c00a56e0` in `6.5.20/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** Sets up a 48-byte frame and reads the global words at 0xc0160504, 0xc0160508 and 0xc016050c. It rewrites the fields of the entry at 0xc0160504 (0xc00a5728 to 0xc00a5734) and the 0xc016050c words (0xc00a5744 to 0xc00a575c), then stores abs 0xfa200e84 and 0xfa200e80 (0xc00a5768, 0xc00a576c). It calls 0xc00046d4 with 0xc0004710 and 0xc00046e0, the last with r10 loaded from 0xc0160504. Read with sub_c00a563c, it looks like the other half of the same list update, but the facts do not say whether each side links or unlinks an entry.

- `sub_c00a5eb0` in `6.5.20/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** Sets up a 32-byte frame and reads the global word at 0xc0131278 (0xc00a5ec8). It writes four words to abs 0xfa200e40, 0xfa200e44, 0xfa200e48 and 0xfa200e4c (0xc00a5eec to 0xc00a5f0c), then runs a loop of halfword loads from 0xc012eac8 and 0xc012eaca with a bdnz loop at 0xc00a5f58 and a long run of halfword stores. It uses a load-reserve and store-conditional pair at 0xc00a5fd4 and 0xc00a5fdc, and calls 0xc00046a0, 0xc00aa0a0, 0xc0004c2c (with r10=8) and 0xc00a56e0 with r3=0x600a0000 and r4=0xc00a5a60. The loop contents are not resolved in the facts.

- `sub_c01344f0` in `6.5.20/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01344f0
  **Inferred.** Sets up a 32-byte frame, reads the word at 0xc0160c7c and abs 0xfa200280, and stores to abs 0xfa200380 at 0xc0134518 and 0xc0134520. It does a divide and multiply sequence on loaded values (divw at 0xc0134568 and 0xc0134578, mulhw at 0xc0134554), calls 0xc0010640 with the constant 0xc00f011c and a count of 0x3c, and calls 0xc0004630. It ends with a call to 0xc0003300 with r3=0xf and the callback address 0xc000a584, a call shape like the one at 0xc00a23ec, and returns an unresolved value.

- `sub_c013d7e4` in `6.5.20/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc013d7e4
  **Inferred.** The largest routine in the batch, 1892 bytes with a 96-byte frame. It makes about fifty word stores into the block at 0xc0198ab0 to 0xc0198c28 (0xc013d840 to 0xc013d9e0), then copies blocks with calls to 0xc000cbe4 (size 0xc4 at 0xc013d82c) and 0xc000c8d4 (at 0xc013d888 and 0xc013d9b4). It stores 16-bit values to abs 0xfa200950, 0xfa200952 and 0xfa200954 (0xc013da38 to 0xc013da40) and reads the globals 0xc0160500 and 0xc01604e8. It calls 0xc0013c38 with r4=0xc00a3000 at entry, and its neighbours 0xc00a25cc, 0xc00a2600, 0xc00a250c and 0xc00a2630 near the end. The facts show what it writes, not what the fields mean, so its purpose is unclear.

- `sub_fe004efc` in `6.5.21/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** Sets up a 32-byte frame and makes a fixed series of calls to 0xfe004610, each with a constant argument (0xe013ec, 0xe01418, 0xe01428, 0xe01454, 0xe01474). These are interleaved with calls to 0xfe0051f0 (r3=0xe01868, r9=0xfe040000), 0xfe004ff4 (r3=0xfe040000) and 0xfe0046c8. It skips the block at 0xfe004f78 unless the return of 0xfe004ff4 is nonzero (bne at 0xfe004f40), and stores to abs 0xfa200004 at 0xfe004f84. The sequence looks like boot-time output gated by a check on the block at 0xfe040000, but the facts do not name the callees.

- `sub_fe0051a0` in `6.5.21/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** An 80-byte leaf routine with no calls. It stores halfwords to abs 0xfa200950, 0xfa200952, 0xfa200954 and 0xfa200960 to 0xfa200966 (0xfe0051a8 to 0xfe0051e8), and words to abs 0xfa200ab8, 0xfa200abc and 0xfa200ac4 (0xfe0051bc, 0xfe0051c4, 0xfe0051d0), with a halfword to 0xfa200ac2 at 0xfe0051c8. It returns r3 unchanged at 0xfe0051ec. It looks like an initialization of a block of register fields, but the facts do not name them.

- `sub_fe048ba0` in `6.5.21/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** A 180-byte routine that reads and writes back the words at abs 0xfa200abc and 0xfa200ab8 and the halfword at 0xfa200ac2 (0xfe048ba8 to 0xfe048bd4). It then stores halfwords and bytes to abs 0xfa203c80 to 0xfa203cac, runs a counting loop at 0xfe048c10 to 0xfe048c18 that stores bytes, and returns an unresolved value at 0xfe048c50. It writes the global 0x705cd8 at 0xfe048c44 and abs 0xfa200030 at 0xfe048c4c, so it looks like a status update that sub_fe048c54 reads back.

- `sub_fe048c54` in `6.5.21/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** Sets up a 32-byte frame and reads the flag word at 0x705cd8 (0xfe048c74). It calls sub_fe048ba0 with r9=0x700000 at 0xfe048c90, but only when that flag is zero (bne at 0xfe048c8c skips the call otherwise). It reads halfwords at abs 0xfa203c82 and 0xfa203c80, stores bytes to abs 0xfa200860, 0xfa200870 and 0xfa20086c, and runs halfword and byte loops with a bdnz at 0xfe048d44. It ends with a call to 0xfe044770 at 0xfe048e08. The loop data and the meaning of the stores are not shown in the facts.

- `sub_c000a5e0` in `6.5.21/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** A 68-byte routine with a 32-byte frame. It reads abs 0xfa200284 at 0xc000a5f4 and does a byte read-modify-write (lbz at 0xc000a60c, stb at 0xc000a614) through a pointer the renderer could not resolve. It reads the machine state register with mfmsr at 0xc000a5f8 and writes it back with mtmsr at 0xc000a608 after an rlwinm at 0xc000a604, which suggests it changes the interrupt-enable bit around the byte update. The facts do not show the restore, so that reading is a guess, and it calls 0xc0004710 at 0xc000a5ec and 0xc0010640 with the constant 0xc00f0164 at 0xc000a61c.

- `sub_c00a2374` in `6.5.21/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** A 16-byte-frame routine that stores to abs 0xfa200030 at 0xc00a2388 and to the globals 0xc01604e8 to 0xc01604f8 (0xc00a23a8 to 0xc00a23d8). It calls 0xc000b454 at 0xc00a23b8 with r4 set to its r3 plus 4096, r5=0xfa200000, r6=0xc0160000 and r7=1. That pattern suggests it maps a 0x1000-byte window at 0xfa200000 and keeps the pointer in a global, but the facts do not confirm what 0xc000b454 does. It then reads and writes through the pointer that call returned (lwz at 0xc00a23bc, stw at 0xc00a23cc).

- `sub_c00a23ec` in `6.5.21/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** The same instruction body as the 6.5.20 build at this address. It sets up a 16-byte stack frame, stores to abs 0xfa200940 and 0xfa200948, then calls 0xc0003300 with r3=5 and r4=0xc00a247c as a function address. When that call returns zero it skips the call to 0xc0010094, then calls 0xc00a250c and returns that result. The callee at 0xc0003300 is unnamed, so the registration reading is a guess from the argument shape.

- `sub_c00a247c` in `6.5.21/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** The same instruction body as the 6.5.20 build at this address. It stores a halfword to abs 0xfa200930 and reads it back, loads several words through computed addresses, and makes an indirect call at 0xc00a24c8 with no resolved target. It branches on a loaded value at 0xc00a24b8 and stores to abs 0xfa20094c at 0xc00a24f0. This looks like a dispatch on a 16-bit field, but the table and the target are not in the facts.

- `sub_c00a250c` in `6.5.21/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** The same instruction body as the 6.5.20 build at this address. It saves registers with stmw, calls 0xc00a2594, and calls 0xc0010640 with the constant pointer 0xc010a1d4 at 0xc00a2558, so 0xc0010640 may be a print or log function. It loads a word from abs 0xfa200948 at 0xc00a256c and restores the saved registers with lmw at 0xc00a2588.

- `sub_c00a2594` in `6.5.21/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** The same 56-byte leaf body as the 6.5.20 build at this address, with no calls. It stores two words through computed addresses (stwx at 0xc00a25a8 and 0xc00a25ac), reads a word from abs 0xfa200948 at 0xc00a25b4, and returns an unresolved value at 0xc00a25c8. Its effect is limited to the word it reads from 0xfa200948.

- `sub_c00a4e98` in `6.5.21/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** The same instruction body as the 6.5.20 build at this address. It sets up a 48-byte frame with stmw and runs halfword loads and stores through computed addresses. It has a load-reserve and store-conditional pair with a retry branch at 0xc00a4fb8 (lwarx at 0xc00a4fac, stwcx. at 0xc00a4fb4), which suggests an atomic update of a shared word, and it writes abs 0xfa200e54 at 0xc00a4f6c.

- `sub_c00a515c` in `6.5.21/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** The same 688-byte body as the 6.5.20 build at this address. It reads abs 0xfa200e44 on entry and branches around the main loop at 0xc00a5188 when that word is zero. The loop makes many halfword and word loads through computed addresses, calls 0xc0010640 with the constant pointers 0xc010a894 and 0xc010a8a4, and calls its neighbours 0xc00a540c and 0xc00a563c. The facts do not show what the loop processes.

- `sub_c00a563c` in `6.5.21/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** The same 164-byte body as the 6.5.20 build at this address. It walks the pointer chain from the global at 0xc0160508, stores to the MMIO-range words abs 0xfa200e80 and 0xfa200e84, writes the list words 0xc0160504 and 0xc0160508, and makes an indirect call at 0xc00a5698 through the value loaded from abs 0xfa200e80. It calls 0xc0010640 with the constant 0xc010a910.

- `sub_c00a56e0` in `6.5.21/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** The same 184-byte body as the 6.5.20 build at this address. It reads the globals 0xc0160504, 0xc0160508 and 0xc016050c, rewrites the entry fields at 0xc0160504 and 0xc016050c, and stores abs 0xfa200e84 and 0xfa200e80 at 0xc00a5768 and 0xc00a576c. It calls 0xc00046d4 with 0xc0004710 and 0xc00046e0. The facts do not say whether each side of the list update links or unlinks an entry.
