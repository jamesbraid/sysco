# Tier-1 routine readings, sysco batch 3

Role statements for targets 75 to 99 of the tier-1 list, written from the rendered
instruction facts only (registers, calls, constants, branch targets and address ranges).
Each role is a reading of those facts, not a confirmed contract. All 25 renders passed
grade_readings.py, so none were rejected. Routines that render identically in two builds
get the same role text under each build's full path.

- `sub_c00a4e98` in `6.5.14/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** A 0x30-byte frame with a `stmw` save at 0xc00a4ea0, then reads through unknown pointers and a byte store at 0xc00a4fa0. It stores to the fixed address 0xfa200e54 at 0xc00a4f6c, reads the word at 0xc0162178 at 0xc00a4f58, and calls 0xc0010640, 0xc0004868, 0xc00046d4 with 0xc0004710, and 0xc00046e0. The purpose is unclear: the facts show a status-style store and helper calls but no decoded field meanings.

- `sub_c00a515c` in `6.5.14/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** A 0x40-byte frame that tests the word at 0xfa200e44 (loaded at 0xc00a5174, branch at 0xc00a5188 taken when it is zero). Otherwise it runs a long run of helper calls, including 0xc0013dec with r3=0 and r4=1, 0xc0013764, and a call to the routine at 0xc00a563c at 0xc00a53e8, then returns an unknown r3 at 0xc00a5408. The flag's meaning and the callee roles are not derivable from these facts.

- `sub_c00a563c` in `6.5.14/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** Follows a pointer chain rooted at the cell 0xc0162508: it loads the pointer, then the words at +4 and +8, and tests each. It stores into the fixed addresses 0xfa200e80 and 0xfa200e84, and calls 0xc0010640 with r3=0xc010bab8 at 0xc00a567c. A `blrl` at 0xc00a5698 calls through a pointer read from 0xfa200e80, whose target the facts do not resolve.

- `sub_c00a56e0` in `6.5.14/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** Relinks the same cell chain as the routine at 0xc00a563c: it reads 0xc0162504 and 0xc0162508, writes the link words at 0xc0162504 +0 to +8, writes 0xc016250c+8 and 0xc0162508, and stores to 0xfa200e80 and 0xfa200e84. It calls 0xc00046d4 with 0xc0004710, and 0xc00046e0 with r10 loaded from 0xc0162504. The role reads as a list reset or relink, but the list's meaning is not established.

- `sub_c00a5eb0` in `6.5.14/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** A 0x20-byte frame with a loop body that uses three `bdnz` branches (0xc00a5f58, 0xc00a6018, 0xc00a6048). It loads halfwords from the constant addresses 0xc0130ac8 and 0xc0130aca, stores into 0xfa200e40, 0xfa200e44, 0xfa200e48 and 0xfa200e4c, and calls 0xc00046a0, 0xc00aa0a0, and 0xc0004c2c with r10=8. It ends with a call to the routine at 0xc00a56e0 with r3=0x600a0000 and r4=0xc00a5a60 at 0xc00a60c0. It looks like a table copy into fixed registers, but the table's contents are not in the facts.

- `sub_ffe5b6c8` in `6.5.15/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5b6c8
  **Inferred.** A coldfire routine with a `link.w` frame and `movem.l` saves. It sets a bit in the byte at 0x200007f8 with `bset.b` at 0xffe5b6da and clears one with `bclr.b` at 0xffe5b724, reads the byte at 0x200007ff at 0xffe5b708, and branches on the data at several points. It returns the constant 0 in d0 at 0xffe5b7fa. The bit meanings are not decoded by the facts, so the role is limited to bit manipulation of two byte cells.

- `sub_fe004efc` in `6.5.15/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** A 0x20-byte frame that stores a set of stack slots and then the word 0xfa200004 at 0xfe004f84. It calls 0xfe004610 six times, each with a constant argument in the 0xe01xxx range (0xe013ec, 0xe01418, 0xe01428, 0xe01454, 0xe01474), plus 0xfe0051f0, 0xfe004ff4, 0xfe004f98 and 0xfe0046c8. A branch at 0xfe004f40 depends on the return of 0xfe004ff4. Those constants are probably data pointers, but the facts do not show their contents.

- `sub_fe0051a0` in `6.5.15/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** A leaf routine with no calls and no loads from unknown pointers. It stores halfwords to 0xfa200950, 0xfa200952, 0xfa200954, 0xfa200960, 0xfa200962, 0xfa200964, 0xfa200966 and 0xfa200ac2, and words to 0xfa200ab8, 0xfa200abc and 0xfa200ac4. It returns its r3 unchanged at 0xfe0051ec. It looks like a fixed register-initialisation block, with no further role shown by the facts.

- `sub_fe048ba0` in `6.5.15/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** Read-modify-writes the words at 0xfa200ab8 and 0xfa200abc, the same cells that `sub_fe0051a0` writes. It also reads and writes halfword 0xfa200ac2, stores to 0xfa203c80, 0xfa203c82, 0xfa203c84, 0xfa203c85, 0xfa203c86, 0xfa203cac and 0xfa2009c0, and writes 0xfa200030 and the constant address 0x705cd8 at 0xfe048c44. A loop at 0xfe048c10 with a `bne` back at 0xfe048c18 walks an unknown buffer. The purpose is unclear.

- `sub_fe048c54` in `6.5.15/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** A 0x20-byte frame. It tests the word at 0x705cd8 at 0xfe048c8c and calls the routine at 0xfe048ba0 with r9=0x700000 at 0xfe048c90. It then loops over buffers with `bdnz` branches at 0xfe048d44 and 0xfe048de4, writes byte cells at 0xfa200860, 0xfa20086c and 0xfa200870, and calls 0xfe044770 at 0xfe048e08 before returning an unknown r3 at 0xfe048e2c. The loop data and the callee are not identified by the facts.

- `sub_c000a5e0` in `6.5.15/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** A small 0x20-byte frame routine. It loads a word from 0xfa200284 at 0xc000a5f4, calls 0xc0004710 with no argument set at 0xc000a5ec, moves one byte with `lbz` and `stb` through unknown pointers, and calls 0xc0010640 with r3=0xc00f02d8 at 0xc000a61c. The role beyond that is unclear.

- `sub_c00a2374` in `6.5.15/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** A 0x10-byte frame that writes the constant-address word 0xfa200030 at 0xc00a2388 and the cells 0xc01624ec, 0xc01624f0, 0xc01624f4 and 0xc01624f8, then writes 0xc01624e8. It calls 0xc000b454 at 0xc00a23b8 with r4=r3+4096, r5=0xfa200000, r6=r8=r9=r10=0xc0160000 and r7=1. The shape suggests a mapping or region setup, but the callee's behaviour is not in the facts.

- `sub_c00a23ec` in `6.5.15/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** Stores words at 0xfa200940 and 0xfa200948, reads them back at 0xc00a245c and from the frame, and calls 0xc0003300 with r3=5, r4=0xc00a247c and r6=0xc010b320 at 0xc00a2430. If that call returns zero it branches to 0xc00a2448 and calls 0xc0010094 with r3=0xc010b324. It then calls the routine at 0xc00a250c at 0xc00a2458 and returns its result. Passing the address of the routine at 0xc00a247c suggests a registration or callback hook, but that reading is inferred from the call's shape.

- `sub_c00a247c` in `6.5.15/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** Stores and reads back the halfword at 0xfa200930, then walks an indexed table with two `lwzx` loads and a `lwz` sequence. It stores a word to 0xfa20094c at 0xc00a24f0 and makes an indirect `blrl` call at 0xc00a24c8 whose target the facts do not resolve. A branch at 0xc00a24b8 depends on the loaded value. Its role is unclear beyond being a table walker that makes an indirect call.

- `sub_c00a250c` in `6.5.15/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** A 0x20-byte frame with an `stmw` save at 0xc00a2514. It calls the routine at 0xc00a2594 at 0xc00a252c and calls 0xc0010640 with r3=0xc010b340 at 0xc00a2558 on one path. It reads the word at 0xfa200948 at 0xc00a256c and does indexed loads and stores through unknown pointers. The role is unclear, though it looks like a helper for the routine at 0xc00a23ec.

- `sub_c00a2594` in `6.5.15/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** A leaf routine with no calls. It reads the word at 0xfa200948 at 0xc00a25b4 and does two indexed stores (`stwx` at 0xc00a25a8 and 0xc00a25ac) plus one store through an unknown pointer at 0xc00a25c4, then returns an unknown r3 at 0xc00a25c8. The facts do not show what the stores update.

- `sub_c00a4e98` in `6.5.15/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** Same code as the 6.5.14 build at this address, which renders identically. A 0x30-byte frame with a `stmw` save, stores to the fixed address 0xfa200e54 at 0xc00a4f6c, a read of 0xc0162178 at 0xc00a4f58, and calls to 0xc0010640, 0xc0004868, 0xc00046d4 with 0xc0004710, and 0xc00046e0. The purpose is unclear.

- `sub_c00a515c` in `6.5.15/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** Same code as the 6.5.14 build at this address, which renders identically. It tests the word at 0xfa200e44 at 0xc00a5188 and makes a long run of helper calls, including 0xc0013dec with r3=0, r4=1, 0xc0013764, and the routine at 0xc00a563c at 0xc00a53e8, then returns an unknown r3. The flag's meaning is not derivable from these facts.

- `sub_c00a563c` in `6.5.15/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** Same code as the 6.5.14 build at this address, which renders identically. It follows the pointer chain rooted at 0xc0162508, stores to 0xfa200e80 and 0xfa200e84, calls 0xc0010640 with r3=0xc010bab8, and makes a `blrl` through the pointer read from 0xfa200e80. The callee is not resolved.

- `sub_c00a56e0` in `6.5.15/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** Same code as the 6.5.14 build at this address, which renders identically. It relinks the cell chain at 0xc0162504, 0xc0162508 and 0xc016250c, stores to 0xfa200e80 and 0xfa200e84, and calls 0xc00046d4 with 0xc0004710 and 0xc00046e0. The list it manipulates is not identified.

- `sub_c00a5eb0` in `6.5.15/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** Same code as the 6.5.14 build at this address, which renders identically. It loads halfwords from 0xc0130ac8 and 0xc0130aca, stores into 0xfa200e40 through 0xfa200e4c, runs three `bdnz` loops, and calls 0xc00a56e0 with r3=0x600a0000 and r4=0xc00a5a60. The table contents and loop data are not in the facts.

- `sub_ffe605b4` in `6.5.16/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe605b4
  **Inferred.** The same coldfire shape as the routine at 0xffe5b6c8 in the 6.5.15 build: a `link.w` frame, `movem.l` saves, `bset.b` and `bclr.b` on the byte at 0x200007f8, a `move.b` read of 0x200007ff, several data-dependent branches, and a return of the constant 0 in d0 at 0xffe606e6. The bit meanings are not decoded by the facts.

- `sub_ffed05b4` in `6.5.16/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffed05b4
  **Inferred.** Renders the same code shape as the routine at 0xffe605b4 in the same build, 0x70000 higher: `link.w`, `movem.l` saves, `bset.b` and `bclr.b` on the byte at 0x200007f8, a read of 0x200007ff, and a return of the constant 0 in d0 at 0xffed06e6. The facts do not show what the bits control.

- `sub_fe004efc` in `6.5.16/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** Renders identically to the 6.5.15 flash build at this address. It stores stack slots and the word at 0xfa200004 at 0xfe004f84, calls 0xfe004610 with the constant arguments 0xe013ec, 0xe01418, 0xe01428, 0xe01454 and 0xe01474, and calls 0xfe0051f0, 0xfe004ff4, 0xfe004f98 and 0xfe0046c8. A branch at 0xfe004f40 depends on the return of 0xfe004ff4.

- `sub_fe0051a0` in `6.5.16/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** Renders identically to the 6.5.15 flash build at this address. It is a leaf that stores halfwords to 0xfa200950 through 0xfa200966 (including 0xfa200ac2) and words to 0xfa200ab8, 0xfa200abc and 0xfa200ac4, then returns its r3 unchanged at 0xfe0051ec. The facts show no further role.
