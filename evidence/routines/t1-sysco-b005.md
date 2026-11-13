## Tier-1 batch 5, sysco firmware and kernel routines

- `sub_c00a515c` in `6.5.17/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** Opens a 0x40-byte frame and saves a register block with stmw at 0xc00a5164, then reads the word at abs 0xfa200e44 and branches on it at 0xc00a5188. It calls 0xc0010640 twice with constant pointers (r3=0xc010ba3c at 0xc00a51b4 and r3=0xc010ba4c at 0xc00a527c), calls 0xc00046d4 and 0xc0004710 as a pair, and calls 0xc0013dec with r3=0x0 and r4=0x1. Near its end it calls sub_c00a563c at 0xc00a53e8.
  Reviewed: 2026-11-12

- `sub_c00a563c` in `6.5.17/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** Reads the global at abs 0xc0162508 and the word at 0xfa200e80, and stores 0 to 0xfa200e84 at 0xc00a5664. When the global is zero it calls 0xc0010640 with r3=0xc010bab8 at 0xc00a567c and returns. Otherwise the global is a node pointer: if the word at offset 4 of the node is nonzero, it makes an indirect call through that field at 0xc00a5698, with the word read from 0xfa200e80 as r3 and the original r3 moved to r4. It then stores the node's offset-8 word to 0xc0162508 (0xc00a56a8), loads 0xc0162504 into the node's offset 8 and stores the node to 0xc0162504 (0xc00a56b0 and 0xc00a56b4), and, when a next node exists, stores that node's first word to 0xfa200e80 and 0x14 to 0xfa200e84 (0xc00a56c4 and 0xc00a56c8). This is probably a step that runs the callback of the head node of a pending list and moves the node onto a second list.
  Reviewed: 2026-11-12

- `sub_c00a56e0` in `6.5.17/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** Calls 0xc00046d4 with r3 at sp+8, then 0xc0004710 (0xc00a56fc and 0xc00a5700), and reads the global at abs 0xc0162504. When it is zero it sets r30 to 1; otherwise that global is a node pointer: it stores the node's offset-8 word to 0xc0162504 (0xc00a5728), fills the node's offsets 0, 4 and 8 with r3, r4 and 0, and appends the node to the list headed by 0xc0162508 with its tail at 0xc016250c. When 0xc0162508 is nonzero it stores the node at offset 8 of the node that 0xc016250c points to (0xc00a5744) and into 0xc016250c; when it is zero it stores the node to 0xc0162508 and 0xc016250c (0xc00a5758) and stores 0x14 and r3 to 0xfa200e84 and 0xfa200e80 (0xc00a5768 and 0xc00a576c). It then calls 0xc00046e0 at 0xc00a577c with the word saved at sp+8, and returns r30 (0 when a node was taken). It works on the same words as sub_c00a563c in this build.
  Reviewed: 2026-11-12

- `sub_c00a5eb0` in `6.5.17/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** Saves a register block, reads abs 0xc0133278 and stores 1 to the device window at 0xfa200e40 (0xc00a5eec), then calls 0xc00046a0 at 0xc00a5ef0 and stores words to 0xfa200e48, 0xfa200e44 and 0xfa200e4c (0xc00a5efc to 0xc00a5f0c). It reads the halfwords at 0xc0130ac8, 0xc0130aca and 0xc0130acc and stores them to 0xfa200e00 and 0xfa200e04, and runs a six-iteration loop that loads a byte and stores it with stbx at 0xc00a5f50. It calls 0xc00aa0a0 at 0xc00a5fec only for a nonzero pointer loaded at 0xc00a5fb8 whose word at offset 0x70 is 1 or reaches zero after an atomic decrement, and calls 0xc0004c2c at 0xc00a6094. It reads the global at abs 0xc0162518 and tests it at 0xc00a6060 to choose between two pairs of stores to 0xfa200f44 and 0xfa200f84. Its last call is sub_c00a56e0 at 0xc00a60c0 with r3=0x600a0000 and r4=0xc00a5a60.
  Reviewed: 2026-11-12

- `sub_ffe5c5fc` in `6.5.18/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5c5fc
  **Inferred.** Takes one pointer argument at [a6+8], a block of six longwords. It compares the longwords at offsets 0x14, 0x10, 0xc, 0x8, 0x4 and 0x0 with fixed limits (cmp.l at 0xffe5c610 to 0xffe5c648) and returns 0xfffff7cc in d0 when one is out of range. Otherwise it calls 0xffe5e0ec at 0xffe5c658 with that pointer as its only argument and keeps the result in a local, then calls 0xffe5e2dc at 0xffe5c668 with the address of that local. The returned object's longword at offset 0x20 is tested: when it is nonzero, the global at abs 0x300004f8 is subtracted from the local and 0xffe5e2dc is called a second time at 0xffe5c686. It then adds 0x76c to offset 0x14 and increments offsets 0x10 and 0x18 of the returned object, and does read-modify-write byte accesses to abs 0x200007f8 through 0x200007ff, setting bit 7 of 0x200007f8 first and clearing it with bclr.b at 0xffe5c80c. It returns 0 in d0 on that path.
  Reviewed: 2026-11-12

- `sub_ffe5d140` in `6.5.18/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5d140
  **Inferred.** Saves registers with movem.l and sets bit 6 of the byte at abs 0x200007f8 (bset.b at 0xffe5d152). It reads that byte (move.b at 0xffe5d156 and 0xffe5d16a) and the byte at abs 0x200007ff (move.b at 0xffe5d180 and 0xffe5d192), then clears bit 6 of 0x200007f8 again (bclr.b at 0xffe5d19c). It compares a computed value against 0x2, 0x3 and 0x4 (beq.b at 0xffe5d234 and 0xffe5d23a, bne.b at 0xffe5d240), stores two longwords through the pointers at [a6+8] and [a6+0xc] (0xffe5d25c and 0xffe5d266), and returns the constant 0x0 in d0 (moveq at 0xffe5d268, rts at 0xffe5d272).
  Reviewed: 2026-11-12

- `sub_ffe6f350` in `6.5.18/usr/cpu/firmware/sysco/l1.bin(l1#1)` at 0xffe6f350
  **Inferred.** Takes one pointer argument at [a6+8], a block of six longwords. It compares the longwords at offsets 0x14, 0x10, 0xc, 0x8, 0x4 and 0x0 with fixed limits (cmp.l at 0xffe6f364 to 0xffe6f39c) and returns 0xfffff7cc in d0 when one is out of range. Otherwise it calls 0xffe70e40 at 0xffe6f3ac with that pointer as its only argument and keeps the result in a local, then calls 0xffe71030 at 0xffe6f3bc with the address of that local. The returned object's longword at offset 0x20 is tested: when it is nonzero, the global at abs 0x300005d8 is read (0xffe6f3cc), subtracted from the local, and 0xffe71030 is called a second time at 0xffe6f3da. It then adds 0x76c to offset 0x14 and increments offsets 0x10 and 0x18 of the returned object, and does read-modify-write byte accesses to abs 0x200007f8 through 0x200007ff. It returns 0 in d0 on that path.
  Reviewed: 2026-11-12

- `sub_fe004efc` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** Opens a 0x20-byte frame, calls 0xfe0051f0 with r3=0xe01868, then calls 0xfe004610 with r3=0xe013ec (0xfe004f30) and 0xfe004ff4 with r3=0xfe040000, and branches on the value returned by 0xfe004ff4 (bne at 0xfe004f40). When that value is zero it also calls 0xfe004610 with r3=0xe01418, calls 0xfe004f98 with r3=0xfe040000, and calls 0xfe004610 three more times with r3=0xe01428, 0xe01454 and 0xe01428 (0xfe004f4c to 0xfe004f74). On both paths it stores a word to abs 0xfa200004 at 0xfe004f84, calls 0xfe004610 with r3=0xe01474 at 0xfe004f8c, calls 0xfe0046c8 and then branches to itself at 0xfe004f94, so it does not return. The 0xe01xxx values passed to 0xfe004610 could be message strings, but the facts here do not show their contents.
  Reviewed: 2026-11-12

- `sub_fe0051a0` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** Stores halfwords to abs 0xfa200950, 0xfa200952, 0xfa200954, 0xfa200960, 0xfa200962, 0xfa200964, 0xfa200966 and 0xfa200ac2, and words to 0xfa200ab8, 0xfa200abc and 0xfa200ac4. It is straight-line code with no calls or branches, and it returns with r3 unchanged. The words at 0xfa200ab8 and 0xfa200abc and the halfword at 0xfa200ac2 are the ones sub_fe048ba0 reads back.
  Reviewed: 2026-11-12

- `sub_fe048ba0` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** Reads the words at abs 0xfa200abc and 0xfa200ab8 and the halfword at 0xfa200ac2, ORs 0x30 into each and writes them back (0xfe048ba8 to 0xfe048bd4). It also stores halfwords and bytes to 0xfa203c80 through 0xfa203cac, stores 0x11 to the halfword at 0xfa2009c0 (0xfe048c08), and then polls that halfword (lhz at 0xfe048c10) in a loop that branches back to 0xfe048c10 while bit 0 is set (bne at 0xfe048c18). It then stores bytes to 0xfa200860 and nearby, and words to abs 0x705cd8 and 0xfa200030 at 0xfe048c44 and 0xfe048c4c.
  Reviewed: 2026-11-12

- `sub_fe048c54` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** Opens a 0x20-byte frame and reads the word at abs 0x705cd8 (0xfe048c74). It calls sub_fe048ba0 at 0xfe048c90 only when that word is zero (the bne at 0xfe048c8c skips the call when it is not), then reads the halfwords at 0xfa203c82 and 0xfa203c80, which sub_fe048ba0 stores. It stores bytes to 0xfa200860, 0xfa200870 and 0xfa20086c, builds descriptors in the 0xfa2009c0 area, and polls a halfword for bit 0x8000 in two bounded loops (bdnz at 0xfe048d44 and 0xfe048de4). It calls 0xfe044770 at 0xfe048e08 only on a path where a poll ran out, with r3=0x705720 or 0x705740, and then returns -1. Otherwise it returns the halfword at offset 2 of the second descriptor (lhz at 0xfe048df8).
  Reviewed: 2026-11-12

- `sub_c000a5e0` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** Opens a 0x20-byte frame, calls 0xc0004710 at 0xc000a5ec without setting r3 first, and reads the word at abs 0xfa200284 (lwz at 0xc000a5f4). It reads the MSR, writes the word back to 0xfa200284 with 0x80 ORed in (0xc000a600), clears a bit of the saved MSR and writes it with mtmsr (0xc000a608). It then loads a byte from 0xfa20028c and stores it to the stack at sp+8 (0xc000a614), calls 0xc0010640 with r3=0xc00f0164 at 0xc000a61c, and branches to itself at 0xc000a620, so it does not return.
  Reviewed: 2026-11-12

- `sub_c00a2374` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** Stores words to abs 0xfa200030, 0xc01604ec, 0xc01604f0, 0xc01604f4 and 0xc01604f8 (0xc00a2388 to 0xc00a23b4). It calls 0xc000b454 at 0xc00a23b8 with r4=r3+0x1000, r5=0xfa200000 (a device window base), r6=r8=r9=r10=0xc0160000 and r7=0x1. It then reads a word through the pointer that call returns (lwz at 0xc00a23bc) and stores to 0xc01604e8 at 0xc00a23d8.
  Reviewed: 2026-11-12

- `sub_c00a23ec` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** Stores words to abs 0xfa200940 and 0xfa200948 and later reads 0xfa200940 at 0xc00a245c, ORs 0x80 into it and stores it back. It calls 0xc0003300 with r3=0x5 and r4=0xc00a247c, which is sub_c00a247c, and calls 0xc0010094 at 0xc00a2444 only when that result is nonzero (beq at 0xc00a2438). It then calls sub_c00a250c at 0xc00a2458 with r3=0, r4=0xc00a2508 and r5=0, and does not set r3 after that call. 0xc0003300 is probably the routine that registers sub_c00a247c as a callback.
  Reviewed: 2026-11-12

- `sub_c00a247c` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** Stores a halfword to abs 0xfa200930 and reads it back at 0xc00a2498, then stores a word to 0xfa20094c at 0xc00a24f0. It makes one indirect call through a register at 0xc00a24c8, so the target is not named in the rendered facts. It is probably the callback that sub_c00a23ec registers.
  Reviewed: 2026-11-12

- `sub_c00a250c` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** Takes an index in r3, a handler pointer in r4 and a value in r5. When r4 is zero it calls sub_c00a2594 at 0xc00a252c and returns. Otherwise, when the table entry for that index (0xc01989b0 plus r3*8) already holds a nonzero word, it passes the pointer 0xc010a1d4 and r4 to 0xc0010640 at 0xc00a2558. It then stores r4 and r5 into the table entry, and reads the word at abs 0xfa200948 at 0xc00a256c, ORs in bit r3 and stores it back (0xc00a257c). That is the same word sub_c00a23ec stores at 0xc00a2418.
  Reviewed: 2026-11-12

- `sub_c00a2594` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** Takes an index in r3. It stores zero into both words of the table entry at 0xc01989b0 plus r3*8 (0xc00a25a8 and 0xc00a25ac), then reads the word at abs 0xfa200948 at 0xc00a25b4, clears bit r3 and stores it back at 0xc00a25c4. It makes no calls. The word is the one sub_c00a23ec initialises and sub_c00a250c sets bits in, so this routine clears the entry that sub_c00a250c fills.
  Reviewed: 2026-11-12

- `sub_c00a4e98` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** Reads the global at abs 0xc0160178 (0xc00a4f58) and stores a word to 0xfa200e54 at 0xc00a4f6c. When the halfword at offset 0 of the descriptor has bit 0x8000 set, it calls 0xc0010640 with r3=0xc010a7dc at 0xc00a4ecc and returns 1. Otherwise it calls 0xc0004868 at 0xc00a4f38, then the 0xc00046d4 and 0xc0004710 pair at 0xc00a4f40 and 0xc00a4f44 and 0xc00046e0 at 0xc00a4fc0, and returns 0. These are the same helper calls that sub_c00a515c makes.
  Reviewed: 2026-11-12

- `sub_c00a515c` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** Same frame, register save and branch on the word at abs 0xfa200e44 as the 6.5.17 build of this routine. The call targets are the same, but the constant pointers differ (r3=0xc010a894 at 0xc00a51b4 and r3=0xc010a8a4 at 0xc00a527c), so the code is not identical across the two builds. It ends by calling sub_c00a563c at 0xc00a53e8.
  Reviewed: 2026-11-12

- `sub_c00a563c` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** Reads the global at abs 0xc0160508 and the word at 0xfa200e80, and stores 0 to 0xfa200e84 at 0xc00a5664. When the global is zero it calls 0xc0010640 with r3=0xc010a910 at 0xc00a567c and returns. Otherwise it treats the global as a node pointer: if the word at offset 4 of the node is nonzero, it makes an indirect call through that field at 0xc00a5698, passing the word read from 0xfa200e80 as r3. It then stores the node's offset-8 word to 0xc0160508, moves the node onto the list at 0xc0160504 (0xc00a56b0 and 0xc00a56b4) and, when a next node exists, stores its first word to 0xfa200e80 and 0x14 to 0xfa200e84. It is the 6.5.18 counterpart of the 6.5.17 routine at the same address.
  Reviewed: 2026-11-12

- `sub_c00a56e0` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** Calls 0xc00046d4 with r3 at sp+8, then 0xc0004710 (0xc00a56fc and 0xc00a5700), and reads the global at abs 0xc0160504. When it is zero it returns 1; otherwise it takes that node, stores its offset-8 word back to 0xc0160504, fills the node from r3 and r4, and appends it to the list at 0xc0160508 with its tail at 0xc016050c. It stores words to 0xfa200e84 and 0xfa200e80 (0xc00a5768 and 0xc00a576c) only when 0xc0160508 was zero. It calls 0xc00046e0 at 0xc00a577c with the word saved at sp+8 and returns 0 when a node was taken. It works on the same two lists as sub_c00a563c.
  Reviewed: 2026-11-12

- `sub_c00a5eb0` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** Reads abs 0xc0131278 at 0xc00a5ec8 and stores 1 to the device window at 0xfa200e40 (0xc00a5eec), calls 0xc00046a0 at 0xc00a5ef0, then stores words to 0xfa200e48, 0xfa200e44 and 0xfa200e4c. It reads the halfwords at 0xc012eac8, 0xc012eaca and 0xc012eacc and stores them to 0xfa200e00 and 0xfa200e04. It calls 0xc00aa0a0 at 0xc00a5fec only for a nonzero pointer loaded at 0xc00a5fb8 whose word at offset 0x70 is 1 or reaches zero after an atomic decrement, and calls 0xc0004c2c at 0xc00a6094. It reads the global at abs 0xc0160518 and tests it at 0xc00a6060 to choose between two store sequences. Its last call is sub_c00a56e0 at 0xc00a60c0 with r3=0x600a0000 and r4=0xc00a5a60.
  Reviewed: 2026-11-12

- `sub_c01344f0` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01344f0
  **Inferred.** Stores two words to abs 0xfa200380 (0xc0134518 and 0xc0134520) and reads the words at abs 0xc0160c7c and 0xfa200280. It calls 0xc0010640 with r3=0xc00f011c and r5=0x3c at 0xc0134548, and calls 0xc0004630 at 0xc0134580. It calls 0xc0003300 at 0xc01345e0 with r3=0xf and r4=0xc000a584, skips the call to 0xc0010094 at 0xc01345f4 when that result is zero, and returns.
  Reviewed: 2026-11-12

- `sub_c013d7e4` in `6.5.18/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc013d7e4
  **Inferred.** Stores many words and halfwords into the block at 0xc0198ab0 through 0xc0198b74 (for example at 0xc013d840, 0xc013d86c and 0xc013d94c). It calls 0xc0013c38 with r3=0x3 and r4=0xc00a3000 at 0xc013d800, calls 0xc0010640 with r3=0xc010a504 at 0xc013d81c, and calls the helpers 0xc000cbe4 and 0xc000c8d4 on that block. When 0xc002778c returns nonzero at 0xc013dbbc it goes on to call 0xc00a25cc and 0xc00a2600 in pairs, then sub_c00a250c at 0xc013df04 and sub_c00a2630 at 0xc013df1c. A zero result skips those calls (beq at 0xc013dbc8), and the code after them branches back to 0xc013dad8 while r30 is at most 3 (ble at 0xc013df2c). It returns r3=0. The many stores suggest a setup routine, but that is not shown by the facts.
  Reviewed: 2026-11-12

- `sub_ffe59638` in `6.5.19/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe59638
  **Inferred.** Takes one pointer argument at [a6+8], a block of six longwords, compares its longwords with fixed limits and returns an error value in d0 when one is out of range. Otherwise it calls 0xffe5908c at 0xffe59694 with that pointer as its only argument. It calls 0xffe5927c at 0xffe596a4 and tests the field at offset 0x20 of the object returned. When that field is nonzero it calls 0xffe5927c a second time at 0xffe596c2. It then adds to or increments the fields at offsets 0x10, 0x14 and 0x18 of the returned object. It also reads and writes the bytes at abs 0x200007f8 through 0x200007ff, the same byte window that sub_ffe5d140 uses.
  Reviewed: 2026-11-12
