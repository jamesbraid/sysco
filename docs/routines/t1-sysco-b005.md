## Tier-1 batch 5, sysco firmware and kernel routines

- `sub_c00a515c` in `6.5.17/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** Opens a 0x40-byte frame and saves a register block with stmw at 0xc00a5164, then reads the word at abs 0xfa200e44 and branches on it at 0xc00a5188. It calls 0xc0010640 twice with constant pointers (r3=0xc010ba3c at 0xc00a51b4 and r3=0xc010ba4c at 0xc00a527c), calls 0xc00046d4 and 0xc0004710 as a pair, and calls 0xc0013dec with r3=0x0 and r4=0x1. Near its end it calls sub_c00a563c at 0xc00a53e8.

- `sub_c00a563c` in `6.5.17/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** Reads the globals at abs 0xc0162508 and 0xc0162504 and the word at 0xfa200e80, then calls 0xc0010640 with r3=0xc010bab8 at 0xc00a567c. It makes an indirect call at 0xc00a5698 through a register loaded from 0xfa200e80, and before returning it stores new words to 0xc0162508, 0xc0162504 and 0xfa200e80 (0xc00a56a8, 0xc00a56b4 and 0xc00a56c4). This is probably a step that updates a two-word list and then dispatches through a stored function pointer.

- `sub_c00a56e0` in `6.5.17/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** Reads the globals at abs 0xc0162504, 0xc0162508 and 0xc016250c, stores into 0xc0162504 and 0xc0162508 (0xc00a5728 and 0xc00a5758) and into the block at 0xc016250c (0xc00a5744), then stores words to 0xfa200e84 and 0xfa200e80 at 0xc00a5768 and 0xc00a576c. It calls 0xc00046d4 and 0xc0004710 as a pair at 0xc00a56fc and 0xc00a5700, then calls 0xc00046e0 at 0xc00a577c with r10 loaded from 0xc0162504. It works on the same words as sub_c00a563c in this build.

- `sub_c00a5eb0` in `6.5.17/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** Saves a register block and stores four words to the device window at 0xfa200e40, 0xfa200e44, 0xfa200e48 and 0xfa200e4c (0xc00a5eec to 0xc00a5f0c), after reading abs 0xc0133278 and the halfwords at 0xc0130ac8 and 0xc0130aca. It loads a byte and stores it with stbx at 0xc00a5f50, and calls 0xc00046a0, 0xc00aa0a0 and 0xc0004c2c (r9 from abs 0xc0162518, r10=0x8). It ends by calling sub_c00a56e0 at 0xc00a60c0 with r3=0x600a0000 and r4=0xc00a5a60.

- `sub_ffe5c5fc` in `6.5.18/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5c5fc
  **Inferred.** Calls the routine at 0xffe5e0ec at 0xffe5c658 with six arguments, the first read from [sp+4] and the rest from d3 to d7. It then calls 0xffe5e2dc twice (at 0xffe5c668 and 0xffe5c686), each time with one argument stored at sp-8. It looks like a wrapper that makes one setup call and then runs a helper twice.

- `sub_ffe5d140` in `6.5.18/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5d140
  **Inferred.** Saves registers with movem.l and sets a bit in the byte at abs 0x200007f8 (bset.b at 0xffe5d152), then clears a bit in a byte through a pointer (bclr.b at 0xffe5d19c) after reading abs 0x200007ff (move.b at 0xffe5d180). It reads three bytes through pointers (move.b at 0xffe5d156, 0xffe5d16a and 0xffe5d192) and compares a value against 0x2, 0x3 and 0x4 (beq.b at 0xffe5d234 and 0xffe5d23a, bne.b at 0xffe5d240). It returns the constant 0x0 in d0 at 0xffe5d272.

- `sub_ffe6f350` in `6.5.18/usr/cpu/firmware/sysco/l1.bin(l1#1)` at 0xffe6f350
  **Inferred.** Saves six registers and compares six longwords of the argument block at [sp+4] (offsets 0x0 to 0x14, cmp.l at 0xffe6f374 to 0xffe6f39c). It stores two longwords with pea.l, reads the global at abs 0x300005d8 at 0xffe6f3cc, then calls 0xffe70e40 at 0xffe6f3ac with the argument block and d3 to d7. It calls 0xffe71030 twice (at 0xffe6f3bc and 0xffe6f3da) and reads fields at offsets 0x20 and 0x14 of the object that call returns.

- `sub_fe004efc` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** Opens a 0x20-byte frame, stores a word to abs 0xfa200004 at 0xfe004f84, and calls 0xfe004610 six times with constant pointers in the 0xe01xxx range (0xe013ec, 0xe01418, 0xe01428, 0xe01454 and 0xe01474, from 0xfe004f30 to 0xfe004f8c). It calls 0xfe0051f0 with r3=0xe01868 and 0xfe004ff4 and 0xfe004f98 with r3=0xfe040000, branches on the value returned by 0xfe004ff4 (bne at 0xfe004f40), and ends with a call to 0xfe0046c8. The pointers look like message strings, so this is probably a sequence of console messages.

- `sub_fe0051a0` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** Stores halfwords to abs 0xfa200950, 0xfa200952, 0xfa200954, 0xfa200960, 0xfa200962, 0xfa200964, 0xfa200966 and 0xfa200ac2, and words to 0xfa200ab8, 0xfa200abc and 0xfa200ac4. It is straight-line code with no calls or branches, and it returns with r3 unchanged. The words it writes are the ones that sub_fe048ba0 reads back.

- `sub_fe048ba0` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** Reads the words at abs 0xfa200abc and 0xfa200ab8 and the halfword at 0xfa200ac2, then writes them back (0xfe048ba8 to 0xfe048bd4). It also stores halfwords and bytes to 0xfa203c80 through 0xfa203cac and to 0xfa2009c0. It ends with a byte-store loop that branches back to 0xfe048c10 (bne at 0xfe048c18), then stores words to abs 0x705cd8 and 0xfa200030 at 0xfe048c44 and 0xfe048c4c.

- `sub_fe048c54` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** Opens a 0x20-byte frame and reads the word at abs 0x705cd8 (0xfe048c74). It calls sub_fe048ba0 at 0xfe048c90 with r9=0x700000 only when that word is zero (the bne at 0xfe048c8c skips the call when it is not), then reads the halfwords at 0xfa203c82 and 0xfa203c80 that the callee probably wrote. It stores bytes to 0xfa200860, 0xfa200870 and 0xfa20086c, and calls 0xfe044770 at 0xfe048e08.

- `sub_c000a5e0` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** Opens a 0x20-byte frame, calls 0xc0004710 with no arguments at 0xc000a5ec, and reads the word at abs 0xfa200284 at 0xc000a5f4. It stores a word and a byte through pointers (0xc000a600 and 0xc000a614), then calls 0xc0010640 with r3=0xc00f0164 at 0xc000a61c. It is a short wrapper around one device-window read and one call.

- `sub_c00a2374` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** Stores words to abs 0xfa200030, 0xc01604ec, 0xc01604f0, 0xc01604f4 and 0xc01604f8 (0xc00a2388 to 0xc00a23b4). It calls 0xc000b454 at 0xc00a23b8 with r4=r3+0x1000, r5=0xfa200000 (a device window base), r6=r8=r9=r10=0xc0160000 and r7=0x1. It then reads a word through the pointer that call returns (lwz at 0xc00a23bc) and stores to 0xc01604e8 at 0xc00a23d8.

- `sub_c00a23ec` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** Stores words to abs 0xfa200940 and 0xfa200948 and reads 0xfa200940 back at 0xc00a245c. It calls 0xc0003300 with r3=0x5 and r4=0xc00a247c, which is sub_c00a247c, and calls 0xc0010094 at 0xc00a2444 only when that result is nonzero (beq at 0xc00a2438). It then calls sub_c00a250c at 0xc00a2458 and returns its result, so 0xc0003300 is probably the routine that registers sub_c00a247c as a callback.

- `sub_c00a247c` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** Stores a halfword to abs 0xfa200930 and reads it back at 0xc00a2498, then stores a word to 0xfa20094c at 0xc00a24f0. It makes one indirect call through a register at 0xc00a24c8, so the target is not named in the rendered facts. It is probably the callback that sub_c00a23ec registers.

- `sub_c00a250c` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** Calls sub_c00a2594 at 0xc00a252c and passes the pointer 0xc010a1d4 to 0xc0010640 at 0xc00a2558. It reads the word at abs 0xfa200948 at 0xc00a256c, the same word that sub_c00a23ec stores at 0xc00a2418. It is a small follow-up routine that reports on that word.

- `sub_c00a2594` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** Reads the word at abs 0xfa200948 at 0xc00a25b4 and returns, with no calls and no stores. The word is the one sub_c00a23ec writes, so this routine probably returns that value to its caller.

- `sub_c00a4e98` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** Reads the global at abs 0xc0160178 (0xc00a4f58) and stores a word to 0xfa200e54 at 0xc00a4f6c. It calls 0xc0010640 with r3=0xc010a7dc at 0xc00a4ecc and 0xc0004868 at 0xc00a4f38, then calls the 0xc00046d4 and 0xc0004710 pair at 0xc00a4f40 and 0xc00a4f44 and 0xc00046e0 at 0xc00a4fc0. These are the same helper calls that sub_c00a515c makes, so this is probably a related setup step.

- `sub_c00a515c` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** Same frame, register save and branch on the word at abs 0xfa200e44 as the 6.5.17 build of this routine. The call targets are the same, but the constant pointers differ (r3=0xc010a894 at 0xc00a51b4 and r3=0xc010a8a4 at 0xc00a527c), so the code is not identical across the two builds. It ends by calling sub_c00a563c at 0xc00a53e8.

- `sub_c00a563c` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** Moves values between the globals at 0xc0160504 and 0xc0160508 and the device-window words at 0xfa200e80 and 0xfa200e84. It calls 0xc0010640 with r3=0xc010a910 and r9 loaded from 0xfa200e80 at 0xc00a567c, then makes an indirect call through the pointer read from 0xfa200e80 at 0xc00a5698. It is the 6.5.18 counterpart of the 6.5.17 routine at the same address.

- `sub_c00a56e0` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** Updates the globals at 0xc0160504, 0xc0160508 and 0xc016050c and stores words to 0xfa200e84 and 0xfa200e80 (0xc00a5768 and 0xc00a576c). It calls the 0xc00046d4 and 0xc0004710 pair at 0xc00a56fc and 0xc00a5700, then calls 0xc00046e0 at 0xc00a577c with r10 loaded from 0xc0160504. It probably works on the same two-word list as sub_c00a563c.

- `sub_c00a5eb0` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** Stores four words to the device window at 0xfa200e40, 0xfa200e44, 0xfa200e48 and 0xfa200e4c, after reading abs 0xc0131278 at 0xc00a5ec8 and the halfwords at 0xc012eac8 and 0xc012eaca. It calls 0xc00046a0 at 0xc00a5ef0, 0xc00aa0a0 at 0xc00a5fec, and 0xc0004c2c at 0xc00a6094 (r9 from abs 0xc0160518, r10=0x8). It ends by calling sub_c00a56e0 at 0xc00a60c0 with r3=0x600a0000 and r4=0xc00a5a60.

- `sub_c01344f0` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01344f0
  **Inferred.** Stores two words to abs 0xfa200380 (0xc0134518 and 0xc0134520) and reads the words at abs 0xc0160c7c and 0xfa200280. It calls 0xc0010640 with r3=0xc00f011c and r5=0x3c at 0xc0134548, and calls 0xc0004630 at 0xc0134580. It calls 0xc0003300 at 0xc01345e0 with r3=0xf and r4=0xc000a584, skips the call to 0xc0010094 at 0xc01345f4 when that result is zero, and returns.

- `sub_c013d7e4` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc013d7e4
  **Inferred.** A large setup routine that stores many words and halfwords into the block at 0xc0198ab0 through 0xc0198b74 (for example at 0xc013d840, 0xc013d86c and 0xc013d94c). It calls 0xc0013c38 with r3=0x3 and r4=0xc00a3000 at 0xc013d800, calls 0xc0010640 with r3=0xc010a504 at 0xc013d81c, and calls the helpers 0xc000cbe4 and 0xc000c8d4 on that block. It calls 0xc00a25cc and 0xc00a2600 in pairs, then calls sub_c00a250c at 0xc013df04 and sub_c00a2630 at 0xc013df1c before returning.

- `sub_ffe59638` in `6.5.19/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe59638
  **Inferred.** Calls 0xffe5908c at 0xffe59694 with six arguments, the first read from [sp+4] and the rest from d3 to d7. It calls 0xffe5927c twice (at 0xffe596a4 and 0xffe596c2) and adds to or increments the fields at offsets 0x10, 0x14 and 0x18 of the object that call returns, and tests the field at offset 0x20. It also reads and writes the bytes at abs 0x200007f8, 0x200007fb, 0x200007fd, 0x200007fe and 0x200007ff, the same byte window that sub_ffe5d140 uses.
