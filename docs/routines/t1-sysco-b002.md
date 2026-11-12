# Routine readings: sysco, tier-1 batch 2

These entries cover 25 routines from the 6.5.13 and 6.5.14 sysco firmware images: the L2 kernel and flash
segments and the L1 image. They are entries 50 to 74 (zero-based) of the ranked tier-1 reading list. Each
was rendered from its code body and passed the grader, so none were rejected. Every grade is inferred.
The rendered facts give control flow, calls, constants and memory accesses. The roles below are read from
those facts and say where the facts stop short.

- `sub_c00a193c` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a193c
  **Inferred.** Sets up a small frame, writes a halfword to the fixed address 0xfa200930 at 0xc00a1954 and reads the
  same halfword back at 0xc00a1958. It makes one indirect call at 0xc00a1984 through a loaded pointer, which it
  skips when that pointer is zero at 0xc00a1974. The call passes a second loaded word in r3. The routine sets r3 for
  that call only, so the value it returns is the callee's result on the call path and r3 as it was at entry on the
  skip path. The meaning of the halfword is not established here.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a19c8` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a19c8
  **Inferred.** Saves the registers with stmw. It calls sub_c00a1a50 at 0xc00a19e8 only when r4 is zero (the bne at
  0xc00a19e4 skips it otherwise). When r4 is nonzero it calls 0xc00102c8 at 0xc00a1a14, but only when a word loaded
  from a table indexed by r3 is nonzero (the beq at 0xc00a1a04 skips it), with r3 set to the constant 0xc0105e6c,
  which looks like a message pointer, and r4 passed on. It then stores r5 and r4 into that table entry and reads and
  writes back the word at 0xfa200948 at 0xc00a1a28 and 0xc00a1a38, the same cell sub_c00a1a50 and sub_c00a250c use.
  The caller is not named here, so the purpose of the call to 0xc00102c8 is a guess.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a1a50` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a1a50
  **Inferred.** A leaf of about 56 bytes that reads the word at 0xfa200948 at 0xc00a1a70 and stores through two
  computed indexes at 0xc00a1a64 and 0xc00a1a68. Its shift and rotate instructions are not modelled by the
  renderer, so the stored values are only partly known.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a4b1c` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4b1c
  **Inferred.** Loads the pointer held at 0xc015b4ec and branches on it being null at 0xc00a4b48. When it is null it
  calls 0xc00102c8 with r3 set to the constant 0xc0106564, which looks like a message pointer, and returns.
  Otherwise it loads the pointers at offsets +0x4 and +0x8 from it. When the pointer at +0x4 is nonzero (the beq at
  0xc00a4b64) it calls through that pointer at 0xc00a4b70, with r3 set to the word loaded from 0xfa200e80. It then
  writes the cells 0xc015b4ec and 0xc015b4e8, stores to offset +0x8 of the first node, and, when the pointer from
  +0x8 is nonzero (the beq at 0xc00a4b90), writes the word at its offset 0 to 0xfa200e80. It looks like removing the
  head node of a small linked list and moving it to a second list, but the field names and the object's type are not
  established.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a4bb0` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4bb0
  **Inferred.** Updates the same list cells as sub_c00a4b1c, reading 0xc015b4e8, 0xc015b4ec and 0xc015b4f0 at
  0xc00a4be4, 0xc00a4bf8 and 0xc00a4c1c. It calls 0xc0004694 and 0xc0004748 at 0xc00a4bd8 and 0xc00a4bdc, and
  0xc00046e4 at 0xc00a4c50 with r3 loaded from the stack slot at sp+8 that the first call filled. When the node
  pointer read from 0xc015b4e8 is null (the beq at 0xc00a4bf0) it skips the updates and returns 1; otherwise it
  writes the word at 0xc015b4e8 from that node's offset +8 and stores into the node at offsets 0, +4 and +8
  (0xc00a4c04 to 0xc00a4c10), then returns 0. A second null test at 0xc00a4c14, on the word at 0xc015b4ec, chooses
  which of the cells 0xc015b4ec, 0xc015b4f0 and 0xfa200e80 are also written. The callee names are not known.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a530c` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a530c
  **Inferred.** Saves registers with stmw and reads a word from 0xfa200e40 at 0xc00a532c. When bit 2 of that word is
  clear (the bne at 0xc00a5340 skips it otherwise) it writes the word back with bit 2 set. It calls sub_c00a4c6c
  twice, at 0xc00a535c with r3 set to its own r4 argument and r4 loaded from offset 0x10 of a structure it follows
  from r4, and at 0xc00a536c with r3 as before and r4 set to 0xc012ac64. The second branch, the bne at 0xc00a5370,
  tests the same bit-2 result, and when the bit was clear on entry it writes the saved word back to 0xfa200e40 at
  0xc00a5374.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a5394` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5394
  **Inferred.** Saves registers, reads 0xfa200e40 at 0xc00a53ac, and calls sub_c00a5538 at 0xc00a53c4 only when bit
  2 of that word is clear (the bne at 0xc00a53bc skips it otherwise). It then checks a word at offset 0xb0 of a
  structure it follows from r3 and returns -0x13 when it is zero (the beq at 0xc00a53dc). Otherwise it calls
  sub_c00a4c6c several times, one with r4 set to 0xc012ac74. It then calls 0xc000cfc0 at 0xc00a5414 in a polling
  loop that re-reads a word at offset 0xc8 and goes back at 0xc00a5420 while it is zero. After the loop it calls
  sub_c00a4c6c again, sets the word at offset 0xd0 to 1, calls 0xc00a4ea0 at 0xc00a5440, clears a bit in a word with
  an lwarx and stwcx. retry loop that goes back at 0xc00a5450, and returns 0. The meaning of the words is not
  established.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a5538` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5538
  **Inferred.** A 548-byte routine with many loops. It stores a word to 0xfa200e40 at 0xc00a5560 and reads it back
  at 0xc00a5564, reads halfwords from 0xc015b4dc and 0xc015b4de at 0xc00a55c0 and 0xc00a55c8, and compares a counter
  against 0x31 with a ble at 0xc00a55a4. It also uses a bdnz count loop at 0xc00a56a0 and 0xc00a56d0. It calls
  0xc0004660, calls 0xc00102c8 with r3 set to 0xc0106824, and calls 0xc00a9698 at 0xc00a5674. The loop bodies and
  the purpose of the halfword values are not derived.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a575c` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a575c
  **Inferred.** Reads 0xfa200e40 at 0xc00a5774 and runs a counter loop bounded by 0x31 at 0xc00a57c8, with a
  separate equality test against 0x32 at 0xc00a57d0. It calls 0xc0004660 at 0xc00a57b4 and 0xc00102c8 at 0xc00a57dc
  with r3 set to 0xc01068cc. It is structurally similar to sub_c00a5538, and the shared bounds suggest the two
  belong to one subsystem, which is not confirmed.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c013042c` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc013042c
  **Inferred.** Reads the word at 0xc0180f08 at 0xc0130448 and stores twice to the one cell 0xfa200380, at
  0xc0130454 and 0xc013045c. It stores into 0xc015b070 at 0xc01304e0 and calls 0xc0003304 at 0xc0130510 with r3 set
  to 0xf, branching on its result being zero at 0xc0130518. It calls 0xc000fd1c at 0xc0130524 only when that result
  is nonzero. It also calls 0xc00102c8 with r5 set to 0x3c and 0xc0004608. The overall shape resembles an
  initialisation with a status check, but nothing names it.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c01393ac` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01393ac
  **Inferred.** A 1892-byte routine. Its main body is a run of word and halfword stores at 0xc0139408 to 0xc01395a8
  into two blocks that start at 0xc0180584 and 0xc0180648, followed by a loop that runs four times (the ble at
  0xc0139af4 goes back). It calls 0xc000cb70 with r3 set to 0xc0180584 and r4 set to 0xc4 at 0xc01393f4, and calls
  0xc000c860 with lengths 0x2c and 0xc4 at 0xc0139450 and 0xc013957c; what those callees do is not derived here. It
  also writes 0xfa200950, 0xfa200952 and 0xfa200954, reads 0xc012d7e0 and 0xc012d7dc for a call to 0xc00102c8, and
  calls sub_c00a19c8 at 0xc0139acc, inside the loop and only when the call at 0xc0139780 returned nonzero (the beq
  at 0xc0139790 skips it). The blocks' meaning and the routine's name are not derived from these facts.
  Reviewed: 2026-11-12 (sonnet)

- `sub_ffe587bc` in `6.5.14/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe587bc
  **Inferred.** An m68k routine of 464 bytes with a link and one movem that saves eight registers. It sets bit 6 of
  the byte at 0x200007f8 with bset.b at 0xffe587d2 and clears it with bclr.b at 0xffe588be. Between them it only
  reads the byte cells 0x200007f8 to 0x200007ff with move.b, splitting each into nibbles and combining them with
  multiplications by constants such as 10, 100, 0x3e8, 0x16d, 0x18 and 0x3c, which looks like a date and time
  conversion but is not established. It stores a long to 0x3000f7c0 at 0xffe58976 and adds the stored long to its
  running sum at 0xffe58978. It calls 0xffe59ea4 at 0xffe58944 and at 0xffe5896e, and at 0xffe58954 only when the
  compare against the first result at 0xffe5894e does not take the bls. It stores the sum through its pointer
  argument when that is nonzero and returns it. The meaning of the bit cells is not established.
  Reviewed: 2026-11-12 (sonnet)

- `sub_ffe58ab8` in `6.5.14/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe58ab8
  **Inferred.** A 676-byte m68k routine that reads six stack arguments at sp+4 to sp+0x18 and range-checks them.
  The branches compare them with 0x9c4, 0x78a, 0xc, 0x1f, 0x18, 0x3c and 0x3d at 0xffe58ae2 to 0xffe58b0c. On
  the in-range path it modifies the same byte cells 0x200007f8 to 0x200007ff as sub_ffe587bc, and it reads a long
  at 0x30000650 through an index at 0xffe58cba. The bounds suggest a field-value check, but which field is not
  derived.
  Reviewed: 2026-11-12 (sonnet)

- `sub_ffec87bc` in `6.5.14/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffec87bc
  **Inferred.** The rendered facts match sub_ffe587bc at 0xffe587bc: 464 bytes, the same movem frame, the same byte
  cells 0x200007f8 to 0x200007ff, the same long at 0x3000f7c0, and three calls to 0xffec9ea4 at 0xffec8944,
  0xffec8954 and 0xffec896e. The image appears to hold a second copy of the same routine, but the reason for the
  copy is not established.
  Reviewed: 2026-11-12 (sonnet)

- `sub_ffec8ab8` in `6.5.14/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffec8ab8
  **Inferred.** The same 676-byte m68k body as sub_ffe58ab8, with matching byte-cell accesses to 0x200007f8 to
  0x200007ff and the long at 0x30000650. It is a second copy at the 0xffec window, so its role is the same as the
  routine at 0xffe58ab8 and its grade rests on that match.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe004efc` in `6.5.14/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** A 156-byte routine that makes a run of calls to 0xfe004610 with different constant arguments
  (0xe013ec, 0xe01418, 0xe01428, 0xe01454 and 0xe01474) at 0xfe004f30 to 0xfe004f8c. It calls 0xfe004ff4 at
  0xfe004f38 and branches on its result, and writes 0xfa200004 at 0xfe004f84. The calls with constant pointers look
  like messages, but the callee's behaviour is not derived.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe0051a0` in `6.5.14/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** A leaf of 80 bytes that stores halfwords to 0xfa200950, 0xfa200952, 0xfa200954, 0xfa200960,
  0xfa200962, 0xfa200964, 0xfa200966 and 0xfa200ac2, and words to 0xfa200ab8, 0xfa200abc and 0xfa200ac4. It returns
  its r3 unchanged. The cells are the same ones sub_c01393ac writes at 0xfa200950 to 0xfa200954, and the
  routine looks like a register setup, but no device is named.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe048ba0` in `6.5.14/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** Reads and writes back the word pair at 0xfa200ab8 and 0xfa200abc and the halfword at 0xfa200ac2,
  which sub_fe0051a0 initialises. It then stores halfwords and bytes into 0xfa203c80 to 0xfa203cac and 0xfa2009c0,
  runs a polling loop that re-reads the halfword at 0xfa2009c0 and goes back at 0xfe048c18 while bit 0 of it is set,
  stores bytes to 0xfa200860 to 0xfa200874, and ends with stores to 0x705d50 and 0xfa200030. The purpose of the
  cells at 0xfa203c8x is not derived.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe048c54` in `6.5.14/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** A 476-byte routine that tests the word at 0x705d50 at 0xfe048c7c and calls sub_fe048ba0 at
  0xfe048c90 only when it is zero (the bne at 0xfe048c8c skips it otherwise). It reads 0xfa203c82 at 0xfe048c9c and
  0xfa203c80 at 0xfe048cb8, writes bytes to 0xfa200860, 0xfa200870 and 0xfa20086c at 0xfe048cf0 to 0xfe048d00, and
  runs two bdnz loops that poll halfword reads. It calls 0xfe044770 at 0xfe048e08, on the paths where a poll count
  reaches zero. The purpose of the routine is not derived.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c000a5e0` in `6.5.14/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** A 68-byte routine that calls 0xc0004710 at 0xc000a5ec, then reads the word at 0xfa200284 at
  0xc000a5f4 and writes it back with bit 0x80 set at 0xc000a600. It reads MSR and writes it back with some bits
  cleared (mfmsr at 0xc000a5f8, mtmsr at 0xc000a608). It reads one byte from 0xfa20028c at 0xc000a60c and stores it
  to its own stack frame at 0xc000a614. It calls 0xc0010640 at 0xc000a61c with r3 set to 0xc00f02d8, which looks
  like a message pointer, and then branches to itself at 0xc000a620, so it does not return. The read-modify-write of
  0xfa200284 might be a control or status access, but that is not confirmed.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a2374` in `6.5.14/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** Stores the value 1 to the cell 0xfa200030 at 0xc00a2388 and stores five words to 0xc01624e8,
  0xc01624ec, 0xc01624f0, 0xc01624f4 and 0xc01624f8 at 0xc00a23a8 to 0xc00a23d8. It calls 0xc000b454 at 0xc00a23b8
  with r4 set to r3+4096, r5 set to 0xfa200000 and r7 set to 1, and then sets a bit in the word that call returned a
  pointer to, and executes tlbia. The 0xfa200000 base suggests a mapping call, but the callee is not derived.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a23ec` in `6.5.14/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** Stores words to 0xfa200940 and 0xfa200948 at 0xc00a2414 to 0xc00a2464 and reads 0xfa200940 back at
  0xc00a245c. It calls 0xc0003300 at 0xc00a2430 with r4 set to the address of sub_c00a247c, and calls 0xc0010094
  at 0xc00a2444 only when that result is nonzero, branching on it at 0xc00a2438. It then calls sub_c00a250c at
  0xc00a2458 on either path. The callee of 0xc0003300 is not derived.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a247c` in `6.5.14/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** Writes the halfword at 0xfa200930 at 0xc00a2494 and reads it back at 0xc00a2498, the same
  pattern as sub_c00a193c. It makes an indirect call at 0xc00a24c8 through a loaded pointer, which it skips when
  that pointer is zero at 0xc00a24b8, and stores a word to 0xfa20094c at 0xc00a24f0. It is the address that
  sub_c00a23ec passes to 0xc0003300.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a250c` in `6.5.14/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** Saves registers with stmw. It calls sub_c00a2594 at 0xc00a252c only when r4 is zero (the bne at
  0xc00a2528 skips it otherwise). Otherwise it calls 0xc0010640 with r3 set to 0xc010b340 at 0xc00a2558 when a table
  word loaded at 0xc00a2540 is nonzero (the beq at 0xc00a2548 skips it), and stores r5 and r4 into that table entry.
  It reads 0xfa200948 at 0xc00a256c and writes it back at 0xc00a257c, the same cell sub_c00a1a50 reads. The
  routine's caller is sub_c00a23ec, and its purpose beyond that is not derived.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a2594` in `6.5.14/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** A leaf of 56 bytes that makes two indexed stores at 0xc00a25a8 and 0xc00a25ac, reads 0xfa200948 at
  0xc00a25b4, stores a word at 0xc00a25c4, and returns with r3 unknown. It is called by sub_c00a250c and reads the
  same cell 0xfa200948 as sub_c00a1a50.
  Reviewed: 2026-11-12 (sonnet)
