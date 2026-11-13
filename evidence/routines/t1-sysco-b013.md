# Tier-1 routine readings, sysco batch 13

Role statements for targets 325 to 349 of the tier-1 list, written from the rendered instruction facts
only (registers, stack use, constants, callees and branch targets). Each role is a reading of those
facts, not a confirmed contract. All 25 grades are inferred. The grader accepted every entry, so none
were rejected.

- `sub_c00a5eb0` in `6.5.28/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** Builds a 0x20-byte frame, saves a register block with `stmw` at 0xc00a5eb8, and writes four words into the 0xfa200e40 to 0xfa200e4c window. It loads halfwords from 0xc012eac8 and 0xc012eaca, runs `bdnz` loops at 0xc00a5f58, 0xc00a6018 and 0xc00a6048, and calls 0xc00046a0, 0xc00aa0a0, 0xc0004c2c and 0xc00a56e0. The likely role is initialising a control block at 0xfa200e40, but the facts do not show what the callees do.
  Reviewed: 2026-11-12

- `sub_c01344f0` in `6.5.28/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01344f0
  **Inferred.** Stores two words into 0xfa200380, reads 0xfa200280 and 0xc0160c7c, then passes a constant string address (0xc00f011c) to the call at 0xc0010640. It ends with a check of the result of 0xc0003300 at 0xc01345e0 and a branch at 0xc01345e8 that skips a call when that result is zero. The shape suggests a status or log routine, since 0xc0010640 gets a constant address as its first argument in many places, but the facts do not name it.
  Reviewed: 2026-11-12

- `sub_c013d7e4` in `6.5.28/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc013d7e4
  **Inferred.** A large routine of 1892 bytes that stores to the 0xc0198ab0 to 0xc0198c28 block. It has 51 word stores and about 45 halfword stores through registers the facts do not resolve, besides the stores to absolute addresses. It calls 0xc000cbe4 with r4 = 0xc4 and 0xc000c8d4 with r5 = 0x2c and 0xc4, calls 0xc006b73c twice, and calls 0xc00a25cc and 0xc00a2600 with r3 = 0x20 and 0x80. The overall role looks like setup of a sizable state table, but the facts do not settle which table or which callee does the allocation.
  Reviewed: 2026-11-12

- `sub_ffe5eddc` in `6.5.29/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5eddc
  **Inferred.** A ColdFire routine that compares fields of a parameter block at `[sp+4]`, at offsets 0x0 to 0x14, with the constants 0x3d, 0x3c, 0x1f, 0x18, 0xc, 0x1e, 0x0 and 0x258 (branches at 0xffe5edf6 to 0xffe5ee2a). It calls 0xffe60928 with `[sp+4]` and d3 to d7 on the stack at 0xffe5ee38, and calls 0xffe60b18 twice at 0xffe5ee48 and 0xffe5ee66. After the first of those calls it tests the word at offset +0x20 of the returned pointer at 0xffe5ee52, with a `beq.b` at 0xffe5ee56. It also reads and rewrites the bytes at 0x200007f8 through 0x200007ff and clears a bit with `bclr.b` at 0xffe5efec. The shape is a range check on a request followed by flag-byte updates, but the facts do not say what the fields mean.
  Reviewed: 2026-11-12

- `sub_ffe5e898` in `6.5.29/usr/cpu/firmware/sysco/l1.bin(l1#1)` at 0xffe5e898
  **Inferred.** A ColdFire routine that sets a bit in the byte at 0x200007f8 with `bset.b` at 0xffe5e8ae, then reads the bytes at 0x200007f9, 0x200007fa, 0x200007fb, 0x200007fd, 0x200007fe and 0x200007ff, and clears a bit in a byte whose address the facts do not resolve with `bclr.b` at 0xffe5e99a. It makes one call to 0xffe5a248 at 0xffe5ea7e and tests a value against 0x23a with an unsigned compare (`bls.b`) at 0xffe5e9c4. The flag-byte use resembles the other l1 routines in this batch, so the role is probably a status-flag update, but the facts do not name it.
  Reviewed: 2026-11-12

- `sub_ffe73ff0` in `6.5.29/usr/cpu/firmware/sysco/l1.bin(l1#3)` at 0xffe73ff0
  **Inferred.** Same shape as the l1#0 routine at 0xffe5eddc: it checks a parameter block at `[sp+4]` against the constants 0x3d, 0x3c, 0x1f, 0x18, 0xc, 0x1e and 0x258, and it touches the flag bytes at 0x200007f8 through 0x200007fe. It calls 0xffe77a20 at 0xffe7404c and 0xffe77c10 twice at 0xffe7405c and 0xffe7407a, and it tests the result word at +0x20 of 0xffe77c10 against zero at 0xffe7406a. The facts do not show what the calls do, so the role is inferred from the parallel structure only.
  Reviewed: 2026-11-12

- `sub_fe004efc` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** Builds a 0x20-byte frame, calls 0xfe004610 six times with constant addresses in r3 (0xe013ec, 0xe01418, 0xe01428, 0xe01454, 0xe01428 and 0xe01474), and calls 0xfe0051f0, 0xfe004ff4 and 0xfe004f98, the last two with r3 = 0xfe040000. The `bne` at 0xfe004f40 jumps to 0xfe004f78, skipping the calls at 0xfe004f4c to 0xfe004f74 (including the call to 0xfe004f98), when 0xfe004ff4 returns non-zero. A word is stored to 0xfa200004 at 0xfe004f84, and the routine ends with a call to 0xfe0046c8 at 0xfe004f90. The facts point to a sequence of calls with sub-checks at image start, but they do not say what the constants are or what the checks do.
  Reviewed: 2026-11-12

- `sub_fe0051a0` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** A leaf routine with no calls and no branches. It stores halfwords to 0xfa200950, 0xfa200952, 0xfa200954, 0xfa200960, 0xfa200962, 0xfa200964, 0xfa200966 and 0xfa200ac2, and words to 0xfa200ab8, 0xfa200abc and 0xfa200ac4, then returns r3. This is a block of register-window writes, so it looks like a reset or default-value load for that window, but the facts do not give the values.
  Reviewed: 2026-11-12

- `sub_fe048ba0` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** Reads and rewrites the words at 0xfa200abc and 0xfa200ab8 and the halfword at 0xfa200ac2, then stores halfwords and bytes in the 0xfa203c80 to 0xfa203cac block and a halfword to 0xfa2009c0. A loop at 0xfe048c10 to 0xfe048c18 loads a halfword from an address the facts do not resolve and repeats while its `bne` is taken. After the loop it stores bytes to the block at 0xfa200860 and then to the absolute addresses 0x705cd8 and 0xfa200030. It looks like a register-window initialisation, but the facts do not say what the values mean. The routine at 0xfe048c54 calls it at 0xfe048c90 with r9 = 0x700000.
  Reviewed: 2026-11-12

- `sub_fe048c54` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** A 476-byte routine that tests the word at 0x705cd8 and, only when it is zero (the `bne` at 0xfe048c8c skips it otherwise), calls 0xfe048ba0 at 0xfe048c90 with r9 = 0x700000. It loads halfwords at 0xfa203c80 and 0xfa203c82, stores bytes to 0xfa200860, 0xfa200870 and 0xfa20086c, and runs two `bdnz` loops, from 0xfe048d34 to 0xfe048d44 and 0xfe048dd4 to 0xfe048de4, each also left through a `beq` on a `lhz` result. It calls 0xfe044770 at 0xfe048e08. It looks like a routine that initialises the 0xfa20 window if needed and then starts a transfer, but the facts do not give the loop conditions' meaning or what the call at 0xfe044770 does.
  Reviewed: 2026-11-12

- `sub_c000a5e0` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** Calls 0xc0004710 at 0xc000a5ec, reads 0xfa200284 at 0xc000a5f4 and stores a word to an address the facts do not resolve at 0xc000a600. It reads the machine state register with `mfmsr` at 0xc000a5f8, changes it with `rlwinm` at 0xc000a604 and writes it back with `mtmsr` at 0xc000a608, then loads a byte at 0xc000a60c, stores a byte at 0xc000a614 and calls 0xc0010640 with the constant address 0xc00f0164 at 0xc000a61c. The likely role is a short machine-state change with a message, but the facts do not say which MSR bits change.
  Reviewed: 2026-11-12

- `sub_c00a2374` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** Stores a word to 0xfa200030 and words to the 0xc01604ec, 0xc01604f0, 0xc01604f4 and 0xc01604f8 cells, then calls 0xc000b454 at 0xc00a23b8 with r4 = r3+0x1000, r5 = 0xfa200000, r7 = 1 and r6, r8, r9 and r10 = 0xc0160000. It then loads and stores a word through the returned pointer, executes `tlbia` at 0xc00a23d0, which the facts list with an unsupported effect, and stores to 0xc01604e8 at 0xc00a23d8. The shape is a TLB flush and mapping setup, but the facts do not show what 0xc000b454 maps.
  Reviewed: 2026-11-12

- `sub_c00a23ec` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** Stores two words to the 0xfa200940 and 0xfa200948 window, calls 0xc0003300 with r3=5 at 0xc00a2430 and branches on its result at 0xc00a2438, and prints a constant string through 0xc0010094 at 0xc00a2444 on the path taken when that result is non-zero. It reads back 0xfa200940 and returns the result of 0xc00a250c at 0xc00a2458. The routine looks like a wrapper that writes a pair of control words, calls one helper, and returns the next helper's result, but the helper names are not in the facts.
  Reviewed: 2026-11-12

- `sub_c00a247c` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** Stores a halfword to 0xfa200930, reads it back at 0xc00a2498, and loads words with `lwzx` at 0xc00a24ac and 0xc00a24c0 from a table the facts do not resolve. It calls through a function pointer at 0xc00a24c8 (`blrl`, target taken from a register) when the entry loaded at 0xc00a24ac is not zero (the `beq` at 0xc00a24b8 skips it otherwise). Both paths end with a word stored to 0xfa20094c at 0xc00a24f0. The facts suggest an indexed dispatch table, but they do not pin down the callee, the table layout or the stored value.
  Reviewed: 2026-11-12

- `sub_c00a250c` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** A 136-byte routine with `stmw` at 0xc00a2514 and an indexed table update through `lwzx` at 0xc00a2540 and `stwx` at 0xc00a2560 and 0xc00a2564. It reads the word at 0xfa200948 at 0xc00a256c and calls 0xc00a2594 at 0xc00a252c. It also calls 0xc0010640 with the constant 0xc010a254 at 0xc00a2558, on the path where the `beq` at 0xc00a2548 is not taken. The role is possibly an indexed update of a table, but the facts do not show the table base or the index source, so they do not tie it to the table 0xc00a2594 writes.
  Reviewed: 2026-11-12

- `sub_c00a2594` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** A 56-byte leaf routine. It stores through `stwx` at 0xc00a25a8 and 0xc00a25ac to addresses the facts do not resolve, reads 0xfa200948 at 0xc00a25b4 and stores a word at 0xc00a25c4, then returns at 0xc00a25c8 with r3 not resolved. It is called from 0xc00a250c. The facts do not name the fields it fills or the value stored at 0xc00a25c4.
  Reviewed: 2026-11-12

- `sub_c00a4e98` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** A 328-byte routine that uses a load-reserve and store-conditional pair (`lwarx` at 0xc00a4fac and `stwcx.` at 0xc00a4fb4) on a word, and stores to 0xfa200e54 at 0xc00a4f6c after reading 0xc0160178 at 0xc00a4f58. It calls 0xc0004868 at 0xc00a4f38, 0xc00046d4 and 0xc0004710 at 0xc00a4f40 and 0xc00a4f44, and 0xc00046e0 at 0xc00a4fc0. It calls 0xc0010640 with the constant 0xc010a85c at 0xc00a4ecc, on the path where the `beq` at 0xc00a4ec0 is not taken. The atomic update suggests a shared-counter or lock word, but the facts do not show the protocol.
  Reviewed: 2026-11-12

- `sub_c00a515c` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** A 688-byte routine with about 15 word stores, 19 word loads and 12 halfword loads through registers not resolved by the facts. It reads 0xfa200e44 at 0xc00a5174 and branches on it at 0xc00a5188 to a path at 0xc00a53f8 when it is zero, and it calls 0xc0010640 with constant addresses 0xc010a914 and 0xc010a924. The routine calls 0xc00a563c at 0xc00a53e8 and 0xc00a540c at 0xc00a51c4, and it has sixteen `beq` branches, most on condition flags the facts do not resolve. Its scope is too broad for the facts to name the service it performs.
  Reviewed: 2026-11-12

- `sub_c00a563c` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** Works on a list anchored at 0xc0160504 and 0xc0160508: it loads the head pointer from 0xc0160508 at 0xc00a5650, follows the node at offset +0x8 at 0xc00a56bc, and writes new links back at 0xc00a56a8 to 0xc00a56b4. It stores to 0xfa200e80 and 0xfa200e84. When the head is zero it calls 0xc0010640 with the constant at 0xc010a990. When the head is non-zero and the word at node offset +0x4 is non-zero, it makes an indirect call through `blrl` at 0xc00a5698 with r3 set from the word at 0xfa200e80, so the callback is a runtime value taken from a register the facts do not resolve.
  Reviewed: 2026-11-12

- `sub_c00a56e0` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** A 184-byte list routine on the same anchors as 0xc00a563c, reading 0xc0160504, 0xc0160508 and 0xc016050c, and rewriting the links at 0xc00a5728 to 0xc00a5768. It stores 0xfa200e84 and 0xfa200e80 at 0xc00a5768 and 0xc00a576c and calls 0xc00046d4 and 0xc0004710 at 0xc00a56fc and 0xc00a5700. Together with 0xc00a563c it looks like a queue insert and remove pair, but the facts only show the stores, not the intended order.
  Reviewed: 2026-11-12

- `sub_c00a5eb0` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** The same code as the 6.5.28 build at this address: a 0x20-byte frame, a `stmw` at 0xc00a5eb8, four word stores at 0xfa200e40 to 0xfa200e4c, halfword table reads at 0xc012eac8 and 0xc012eaca, and calls to 0xc00046a0, 0xc00aa0a0, 0xc0004c2c and 0xc00a56e0. The rendered facts match the 6.5.28 entry field for field. It carries the same inferred role as that entry, with the same limits.
  Reviewed: 2026-11-12

- `sub_c01344f0` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01344f0
  **Inferred.** The same code as the 6.5.28 build at this address: two word stores to 0xfa200380, a load from 0xfa200280, a call to 0xc0010640 with the constant 0xc00f011c, and a result check of 0xc0003300 with a `beq` at 0xc01345e8. The role is the same status or log routine suggested by the 6.5.28 entry, and the facts do not confirm it.
  Reviewed: 2026-11-12

- `sub_c013d7e4` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc013d7e4
  **Inferred.** The same code as the 6.5.28 build at this address, a 1892-byte routine writing the 0xc0198ab0 to 0xc0198c28 block and calling 0xc000c8d4, 0xc000cbe4, 0xc006b73c, 0xc00a25cc and 0xc00a2600. Its branches and constants match the 6.5.28 entry in every field the facts report. It carries the same state-table setup role suggested there, with the same limits.
  Reviewed: 2026-11-12

- `sub_ffe5e878` in `6.5.30/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5e878
  **Inferred.** Same shape as the l1#0 routine at 0xffe5eddc in 6.5.29: a parameter block at `[sp+4]` is checked against 0x3d, 0x3c, 0x1f, 0x18, 0xc, 0x1e and 0x258, with the flag bytes at 0x200007f8 to 0x200007fe updated. It calls 0xffe60368 at 0xffe5e8d4 with d3 to d7 and calls 0xffe60558 twice at 0xffe5e8e4 and 0xffe5e902, then returns d0. The helper addresses differ from the 6.5.29 build, so the code is not the same there, but the role is inferred from the parallel structure only.
  Reviewed: 2026-11-12

- `sub_ffe5e530` in `6.5.30/usr/cpu/firmware/sysco/l1.bin(l1#1)` at 0xffe5e530
  **Inferred.** Same shape as the other l1 routines in this batch: it checks a parameter block at `[sp+4]` against 0x3d, 0x3c, 0x1f, 0x18, 0xc, 0x1e and 0x258, and it updates the flag bytes at 0x200007f8 to 0x200007fe. It calls 0xffe60020 at 0xffe5e58c with d3 to d7, calls 0xffe60210 at 0xffe5e59c and 0xffe5e5ba, and clears a flag with `bclr.b` at 0xffe5e740. The facts do not show what the callees do, so the role is inferred from the parallel structure only.
  Reviewed: 2026-11-12
