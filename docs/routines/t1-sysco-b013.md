# Tier-1 routine readings, sysco batch 13

Role statements for targets 325 to 349 of the tier-1 list, written from the rendered instruction facts
only (registers, stack use, constants, callees and branch targets). Each role is a reading of those
facts, not a confirmed contract. All 25 grades are inferred. The grader accepted every entry, so none
were rejected.

- `sub_c00a5eb0` in `6.5.28/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** Builds a 0x20-byte frame, saves a register block with `stmw` at 0xc00a5eb8, and writes four words into the 0xfa200e40 to 0xfa200e4c window. It reads halfword tables at 0xc012eac8 and 0xc012eaca in a `bdnz` loop and calls 0xc00046a0, 0xc00aa0a0, 0xc0004c2c and 0xc00a56e0. The likely role is initialising a control block at 0xfa200e40 from a table, but the facts do not show what the callees do.

- `sub_c01344f0` in `6.5.28/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01344f0
  **Inferred.** Stores two words into 0xfa200380, reads 0xfa200280 and 0xc0160c7c, then passes a constant string address (0xc00f011c) to the call at 0xc0010640. It ends with a check of the result of 0xc0003300 at 0xc01345e0 and a branch at 0xc01345e8 that skips a call when that result is zero. The shape suggests a status or log routine, since 0xc0010640 gets a constant address as its first argument in many places, but the facts do not name it.

- `sub_c013d7e4` in `6.5.28/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc013d7e4
  **Inferred.** A large routine of 1892 bytes that writes a long run of fields in the 0xc0198ab0 to 0xc0198c58 block, with 51 word stores and about 45 halfword stores. It passes block sizes 0xc4 and 0x2c to the calls at 0xc000c8d4 and 0xc000cbe4, calls 0xc006b73c twice, and calls 0xc00a25cc and 0xc00a2600 with sizes 0x20 and 0x80. The overall role looks like setup of a sizable state table, but the facts do not settle which table or which callee does the allocation.

- `sub_ffe5eddc` in `6.5.29/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5eddc
  **Inferred.** A ColdFire routine that range-checks a parameter block at `[sp+4]`, comparing fields at offsets 0x0 to 0x14 with the constants 0x3d, 0x3c, 0x1f, 0x18, 0xc, 0x1e and 0x258 (branches at 0xffe5edf6 to 0xffe5ee24). It calls 0xffe60928 with d3 to d7 at 0xffe5ee38, and calls 0xffe60b18 twice at 0xffe5ee48 and 0xffe5ee66 to read a status word at offset +0x20. It also reads and rewrites the bytes at 0x200007f8 through 0x200007fe and clears a bit with `bclr.b` at 0xffe5efec, so it probably validates a request and updates a status byte, but the facts do not say what the fields mean.

- `sub_ffe5e898` in `6.5.29/usr/cpu/firmware/sysco/l1.bin(l1#1)` at 0xffe5e898
  **Inferred.** A ColdFire routine that sets a bit in the byte at 0x200007f8 with `bset.b` at 0xffe5e8ae, then reads the flag bytes at 0x200007f9 through 0x200007ff and clears a bit in an unnamed byte with `bclr.b` at 0xffe5e99a. It makes one call to 0xffe5a248 at 0xffe5ea7e and tests a value against 0x23a with an unsigned compare at 0xffe5e9c4. The flag-byte use matches the other l1 routines in this batch, so the role is probably a status-flag update, but the facts do not name it.

- `sub_ffe73ff0` in `6.5.29/usr/cpu/firmware/sysco/l1.bin(l1#3)` at 0xffe73ff0
  **Inferred.** Same shape as the l1#0 routine at 0xffe5eddc: it checks a parameter block at `[sp+4]` against the constants 0x3d, 0x3c, 0x1f, 0x18, 0xc, 0x1e and 0x258, and it touches the flag bytes at 0x200007f8 through 0x200007fe. It calls 0xffe77a20 at 0xffe7404c and 0xffe77c10 twice at 0xffe7405c and 0xffe7407a, and it tests the result word at +0x20 of 0xffe77c10 against zero at 0xffe7406a. The facts do not show what the calls do, so the role is inferred from the parallel structure only.

- `sub_fe004efc` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** Builds a 0x20-byte frame, calls 0xfe004610 six times with constant string addresses (0xe01418, 0xe01428, 0xe01454, 0xe01474 and others), and calls 0xfe0051f0, 0xfe004ff4 and 0xfe004f98 with r3 set to 0xfe040000 on two of those calls. A word at 0xfa200004 is stored at 0xfe004f84, and the branch at 0xfe004f40 depends on the result of 0xfe004ff4. The facts point to a sequence of status messages and sub-checks at image start, but they do not name the checks.

- `sub_fe0051a0` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** A leaf routine with no calls and no branches. It stores halfwords to 0xfa200950, 0xfa200952, 0xfa200954, 0xfa200960, 0xfa200962, 0xfa200964, 0xfa200966 and 0xfa200ac2, and words to 0xfa200ab8, 0xfa200abc and 0xfa200ac4, then returns r3. This is a block of register-window writes, so it looks like a reset or default-value load for that window, but the facts do not give the values.

- `sub_fe048ba0` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** Reads and rewrites the words at 0xfa200abc and 0xfa200ab8 and the halfword at 0xfa200ac2, then stores halfwords and bytes to 0xfa203c80 through 0xfa203cac. A byte loop at 0xfe048c10 to 0xfe048c2c runs until its `bne` at 0xfe048c18 falls through, and the routine stores to absolute addresses 0x705cd8 and 0xfa200030 on the way out. It is a short copy-or-clear pass over the 0xfa20 window, called from 0xfe048c54 with r9 set to 0x700000.

- `sub_fe048c54` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** A 476-byte routine that calls 0xfe048ba0 at 0xfe048c90 with r9 set to 0x700000, and branches on the word at 0x705cd8 at 0xfe048c8c. It then walks a table with `lhzx` and a `bdnz` loop from 0xfe048d34 to 0xfe048d44, stores bytes to 0xfa200860, 0xfa200870 and 0xfa20086c, and calls 0xfe044770 at 0xfe048e08. The overall shape is a driver for the pass in 0xfe048ba0, but the table contents and the call at 0xfe044770 are not in the facts.

- `sub_c000a5e0` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** Reads the machine state register with `mfmsr` at 0xc000a5f8, changes it with `rlwinm` at 0xc000a604 and writes it back with `mtmsr` at 0xc000a608, around a read of 0xfa200284 at 0xc000a5f4 and a byte store at 0xc000a614. It calls 0xc0004710 at 0xc000a5ec and calls 0xc0010640 with the constant address 0xc00f0164 at 0xc000a61c. The likely role is a short machine-state change with a message, but the facts do not say which MSR bits change.

- `sub_c00a2374` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** Stores words to 0xfa200030 and to the 0xc01604e8 to 0xc01604f8 cells, then calls 0xc000b454 at 0xc00a23b8 with r4 set to r3+0x1000, r5 set to 0xfa200000, and r6, r8, r9 and r10 set to 0xc0160000. It ends with `tlbia` at 0xc00a23d0, which the facts list with an unsupported effect. The shape is a TLB flush and mapping setup, but the facts do not show what 0xc000b454 maps.

- `sub_c00a23ec` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** Stores two words to the 0xfa200940 and 0xfa200948 window, calls 0xc0003300 with r3=5 at 0xc00a2430 and branches on its result at 0xc00a2438, and prints a constant string through 0xc0010094 at 0xc00a2444 on the path taken when that result is non-zero. It reads back 0xfa200940 and returns the result of 0xc00a250c at 0xc00a2458. The routine looks like a wrapper that writes a pair of control words, calls one helper, and returns the next helper's result, but the helper names are not in the facts.

- `sub_c00a247c` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** Stores a halfword to 0xfa200930, reads it back at 0xc00a2498, and scans a table with `lwzx` at 0xc00a24ac and 0xc00a24c0 and `lwz` loads at 0xc00a24d0, 0xc00a24f4 and 0xc00a24fc. It calls through a function pointer at 0xc00a24c8 (`blrl`, target taken from a register) when the entry is not zero at 0xc00a24b8, then stores the result to 0xfa20094c. The facts suggest an indexed dispatch table, but they do not pin down the callee or the table layout.

- `sub_c00a250c` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** A 136-byte routine with `stmw` at 0xc00a2514 and an indexed table update through `lwzx` at 0xc00a2540 and `stwx` at 0xc00a2560 and 0xc00a2564. It reads the word at 0xfa200948 at 0xc00a256c and calls 0xc00a2594 at 0xc00a252c. It also calls 0xc0010640 with the constant 0xc010a254 at 0xc00a2558 on a later path. The role is probably an indexed update of the table that 0xc00a2594 also writes, but the facts do not show the index source.

- `sub_c00a2594` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** A 56-byte leaf routine. It stores two words with `stwx` at 0xc00a25a8 and 0xc00a25ac, reads 0xfa200948 at 0xc00a25b4, stores one more word at 0xc00a25c4, and returns with a rotate and mask (`rotlw` and `and`) at 0xc00a25bc to 0xc00a25c0. It is a small indexed store helper for the 0xfa200948 control word, called from 0xc00a250c, but the facts do not name the fields it fills.

- `sub_c00a4e98` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** A 328-byte routine that uses a load-reserve and store-conditional pair (`lwarx` at 0xc00a4fac and `stwcx.` at 0xc00a4fb4) on a word, and stores to 0xfa200e54 at 0xc00a4f6c after reading 0xc0160178 at 0xc00a4f58. It calls 0xc0004868 at 0xc00a4f38, 0xc00046d4 and 0xc0004710 at 0xc00a4f40 and 0xc00a4f44, and 0xc00046e0 at 0xc00a4fc0. It prints the constant at 0xc010a85c on the way in. The atomic update suggests a shared-counter or lock word, but the facts do not show the protocol.

- `sub_c00a515c` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** A 688-byte routine with about 15 word stores, 19 word loads and 12 halfword loads from registers not resolved by the facts. It reads 0xfa200e44 at 0xc00a5174 and branches on it at 0xc00a5188 to a path at 0xc00a53f8, and it calls 0xc0010640 with constant strings at 0xc010a914 and 0xc010a924. The routine calls 0xc00a563c at 0xc00a53e8 and 0xc00a540c at 0xc00a51c4, and it has a chain of about ten `beq` tests on loaded values. Its scope is too broad for the facts to name the service it performs.

- `sub_c00a563c` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** Works on a list anchored at 0xc0160504 and 0xc0160508: it loads the head pointer from 0xc0160508 at 0xc00a5650, follows the node at offset +0x8 at 0xc00a56bc, and writes new links back at 0xc00a56a8 to 0xc00a56b4. It stores to 0xfa200e80 and 0xfa200e84 and calls 0xc0010640 with the constant at 0xc010a990. It also makes an indirect call through `blrl` at 0xc00a5698 whose target comes from the word at 0xfa200e80, so the callback is a runtime value.

- `sub_c00a56e0` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** A 184-byte list routine on the same anchors as 0xc00a563c, reading 0xc0160504, 0xc0160508 and 0xc016050c, and rewriting the links at 0xc00a5728 to 0xc00a5768. It stores 0xfa200e84 and 0xfa200e80 at 0xc00a5768 and 0xc00a576c and calls 0xc00046d4 and 0xc0004710 at 0xc00a56fc and 0xc00a5700. Together with 0xc00a563c it looks like a queue insert and remove pair, but the facts only show the stores, not the intended order.

- `sub_c00a5eb0` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** The same code as the 6.5.28 build at this address: a 0x20-byte frame, a `stmw` at 0xc00a5eb8, four word stores at 0xfa200e40 to 0xfa200e4c, halfword table reads at 0xc012eac8 and 0xc012eaca, and calls to 0xc00046a0, 0xc00aa0a0, 0xc0004c2c and 0xc00a56e0. The rendered facts match the 6.5.28 entry field for field. It carries the same inferred role as that entry, with the same limits.

- `sub_c01344f0` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01344f0
  **Inferred.** The same code as the 6.5.28 build at this address: two word stores to 0xfa200380, a load from 0xfa200280, a call to 0xc0010640 with the constant 0xc00f011c, and a result check of 0xc0003300 with a `beq` at 0xc01345e8. The role is the same status or log routine suggested by the 6.5.28 entry, and the facts do not confirm it.

- `sub_c013d7e4` in `6.5.29/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc013d7e4
  **Inferred.** The same code as the 6.5.28 build at this address, a 1892-byte routine writing the 0xc0198ab0 to 0xc0198c58 block and calling 0xc000c8d4, 0xc006b73c, 0xc00a25cc and 0xc00a2600. Its branches and constants match the 6.5.28 entry in every field the facts report. It carries the same state-table setup role suggested there, with the same limits.

- `sub_ffe5e878` in `6.5.30/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5e878
  **Inferred.** Same shape as the l1#0 routine at 0xffe5eddc in 6.5.29: a parameter block at `[sp+4]` is checked against 0x3d, 0x3c, 0x1f, 0x18, 0xc, 0x1e and 0x258, with the flag bytes at 0x200007f8 to 0x200007fe updated. It calls 0xffe60368 at 0xffe5e8d4 with d3 to d7 and calls 0xffe60558 twice at 0xffe5e8e4 and 0xffe5e902, then returns d0. The helper addresses differ from the 6.5.29 build, so the code is not the same there, but the role is inferred from the parallel structure only.

- `sub_ffe5e530` in `6.5.30/usr/cpu/firmware/sysco/l1.bin(l1#1)` at 0xffe5e530
  **Inferred.** Same shape as the other l1 routines in this batch: it checks a parameter block at `[sp+4]` against 0x3d, 0x3c, 0x1f, 0x18, 0xc, 0x1e and 0x258, and it updates the flag bytes at 0x200007f8 to 0x200007fe. It calls 0xffe60020 at 0xffe5e58c with d3 to d7, calls 0xffe60210 at 0xffe5e59c and 0xffe5e5ba, and clears a flag with `bclr.b` at 0xffe5e740. The facts do not show what the callees do, so the role is inferred from the parallel structure only.
