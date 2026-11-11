# Routine readings: sysco, tier-1 batch 2

These entries cover 25 routines from the 6.5.13 and 6.5.14 sysco firmware images: the L2 kernel and flash
segments and the L1 image. They are entries 50 to 74 (zero-based) of the ranked tier-1 reading list. Each
was rendered from its code body and passed the grader, so none were rejected. Every grade is inferred.
The rendered facts give control flow, calls, constants and memory accesses. The roles below are read from
those facts and say where the facts stop short.

- `sub_c00a193c` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a193c
  **Inferred.** Sets up a small frame, writes a halfword to the fixed address 0xfa200930 at 0xc00a1954 and reads
  the same halfword back at 0xc00a1958. It makes one indirect call at 0xc00a1984 through a loaded pointer, which
  it skips when that pointer is zero at 0xc00a1974. The return value is whatever the loads leave in r3, and the
  meaning of the halfword is not established here.

- `sub_c00a19c8` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a19c8
  **Inferred.** Saves the registers with stmw, calls sub_c00a1a50 at 0xc00a19e8, and calls 0xc00102c8 at 0xc00a1a14
  with r3 set to the constant 0xc0105e6c, which looks like a message pointer. It reads the word at 0xfa200948 at
  0xc00a1a28, the same cell sub_c00a1a50 and sub_c00a250c use. The caller is not named here, so the purpose of the
  call to 0xc00102c8 is a guess.

- `sub_c00a1a50` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a1a50
  **Inferred.** A leaf of about 56 bytes that reads the word at 0xfa200948 at 0xc00a1a70 and stores through two
  computed indexes at 0xc00a1a64 and 0xc00a1a68. Its shift and rotate instructions are not modelled by the
  renderer, so the stored values are only partly known.

- `sub_c00a4b1c` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4b1c
  **Inferred.** Follows a pointer held at 0xc015b4ec and then the pointers at offsets +0x4 and +0x8 from it, and
  branches on each being null at 0xc00a4b48, 0xc00a4b64 and 0xc00a4b90. It writes the cells 0xc015b4ec, 0xc015b4e8
  and 0xfa200e80 as it goes, and calls through the pointer loaded from 0xfa200e80 at 0xc00a4b70. It looks like a
  walk over a small linked structure, but the field names and the object's type are not established.

- `sub_c00a4bb0` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4bb0
  **Inferred.** Updates the same pointer cells as sub_c00a4b1c, reading 0xc015b4e8, 0xc015b4ec and 0xc015b4f0 and
  writing them back with new offsets from 0xc015b4e8 at 0xc00a4c04 to 0xc00a4c10. It calls 0xc0004694 and
  0xc0004748 at 0xc00a4bd8 and 0xc00a4bdc, and 0xc00046e4 at 0xc00a4c50 with r10 set from 0xc015b4e8. Two null tests
  at 0xc00a4bf0 and 0xc00a4c14 choose which cells are written, and the callee names are not known.

- `sub_c00a530c` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a530c
  **Inferred.** Saves registers with stmw and reads a word from 0xfa200e40 at 0xc00a532c. It calls sub_c00a4c6c
  twice, at 0xc00a535c with no argument set and at 0xc00a536c with r4 set to 0xc012ac64, and branches on two
  condition-register tests at 0xc00a5340 and 0xc00a5370. The tests' operands are unknown to the renderer, so the
  branches cannot be tied to a value.

- `sub_c00a5394` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5394
  **Inferred.** Saves registers, reads 0xfa200e40 at 0xc00a53ac, and calls sub_c00a5538 at 0xc00a53c4. It then
  makes several calls to sub_c00a4c6c, one with r4 set to 0xc012ac74, plus 0xc000cfc0 and 0xc00a4ea0 inside a
  loop that branches back at 0xc00a5450. It reads like a driver that runs sub_c00a5538 and then a sequence of
  helpers, but the loop's exit condition is not derived.

- `sub_c00a5538` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5538
  **Inferred.** A 548-byte routine with many loops. It stores a word to 0xfa200e40 at 0xc00a5560 and reads it back
  at 0xc00a5564, reads halfwords from 0xc015b4dc and 0xc015b4de at 0xc00a55c0 and 0xc00a55c8, and compares a counter
  against 0x31 with a ble at 0xc00a55a4. It also uses a bdnz count loop at 0xc00a56a0 and 0xc00a56d0. It calls
  0xc0004660, calls 0xc00102c8 with r3 set to 0xc0106824, and calls 0xc00a9698 at 0xc00a5674. The loop bodies and
  the purpose of the halfword values are not derived.

- `sub_c00a575c` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a575c
  **Inferred.** Reads 0xfa200e40 at 0xc00a5774 and runs a counter loop bounded by 0x31 at 0xc00a57c8, with a
  separate equality test against 0x32 at 0xc00a57d0. It calls 0xc0004660 at 0xc00a57b4 and 0xc00102c8 at 0xc00a57dc
  with r3 set to 0xc01068cc. It is structurally similar to sub_c00a5538, and the shared bounds suggest the two
  belong to one subsystem, which is not confirmed.

- `sub_c013042c` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc013042c
  **Inferred.** Reads the word at 0xc0180f08 at 0xc0130448 and stores to the two cells 0xfa200380 at 0xc0130454
  and 0xc013045c. It stores into 0xc015b070 at 0xc01304e0 and calls 0xc0003304 at 0xc0130510 with r3 set to 0xf,
  branching on its result being zero at 0xc0130518. It also calls 0xc00102c8 with r5 set to 0x3c, and 0xc0004608 and
  0xc000fd1c. The overall shape resembles an initialisation with a status check, but nothing names it.

- `sub_c01393ac` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01393ac
  **Inferred.** A 1892-byte routine. Its main body is a long run of stores into the table at 0xc0180584 through
  0xc0180700, with about fifty word and halfword writes. It clears a block of 0xc4 bytes at 0xc0180584 through
  0xc000cb70 and copies blocks with lengths 0x2c and 0xc4 through 0xc000c860. It also writes 0xfa200950, 0xfa200952 and 0xfa200954, reads 0xc012d7e0 and 0xc012d7dc
  for a call to 0xc00102c8, and calls sub_c00a19c8 at 0xc0139acc. The table's meaning and the routine's name are not
  derived from these facts.

- `sub_ffe587bc` in `6.5.14/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe587bc
  **Inferred.** An m68k routine of 464 bytes with a link and eight movem saves. It sets and clears bits in the byte
  cells 0x200007f8 to 0x200007ff with bset.b, bclr.b and move.b at 0xffe587d2 to 0xffe588be. It stores a long to
  0x3000f7c0 at 0xffe58976 and adds to it at 0xffe58978. It calls 0xffe59ea4 three times at 0xffe58944, 0xffe58954
  and 0xffe5896e, and compares the unsigned result of the first call at 0xffe5894e. The meaning of the bit cells is
  not established.

- `sub_ffe58ab8` in `6.5.14/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe58ab8
  **Inferred.** A 676-byte m68k routine that reads six stack arguments at sp+4 to sp+0x18 and range-checks them.
  The branches compare them with 0x9c4, 0x78a, 0xc, 0x1f, 0x18, 0x3c and 0x3d at 0xffe58ae2 to 0xffe58b0c. On
  the in-range path it modifies the same byte cells 0x200007f8 to 0x200007ff as sub_ffe587bc, and it reads a long
  at 0x30000650 through an index at 0xffe58cba. The bounds suggest a field-value check, but which field is not
  derived.

- `sub_ffec87bc` in `6.5.14/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffec87bc
  **Inferred.** The rendered facts match sub_ffe587bc at 0xffe587bc: 464 bytes, the same movem frame, the same byte
  cells 0x200007f8 to 0x200007ff, the same long at 0x3000f7c0, and three calls to 0xffec9ea4 at 0xffec8944,
  0xffec8954 and 0xffec896e. The image appears to hold a second copy of the same routine, but the reason for the
  copy is not established.

- `sub_ffec8ab8` in `6.5.14/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffec8ab8
  **Inferred.** The same 676-byte m68k body as sub_ffe58ab8, with matching byte-cell accesses to 0x200007f8 to
  0x200007ff and the long at 0x30000650. It is a second copy at the 0xffec window, so its role is the same as the
  routine at 0xffe58ab8 and its grade rests on that match.

- `sub_fe004efc` in `6.5.14/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** A 156-byte routine that makes a run of calls to 0xfe004610 with different constant arguments
  (0xe013ec, 0xe01418, 0xe01428, 0xe01454 and 0xe01474) at 0xfe004f30 to 0xfe004f8c. It calls 0xfe004ff4 at
  0xfe004f38 and branches on its result, and writes 0xfa200004 at 0xfe004f84. The calls with constant pointers look
  like messages, but the callee's behaviour is not derived.

- `sub_fe0051a0` in `6.5.14/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** A leaf of 80 bytes that stores halfwords to 0xfa200950, 0xfa200952, 0xfa200954, 0xfa200960,
  0xfa200962, 0xfa200964, 0xfa200966 and 0xfa200ac2, and words to 0xfa200ab8, 0xfa200abc and 0xfa200ac4. It returns
  its r3 unchanged. The cells are the same ones sub_c01393ac writes at 0xfa200950 to 0xfa200954, and the
  routine looks like a register setup, but no device is named.

- `sub_fe048ba0` in `6.5.14/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** Reads and writes back the word pair at 0xfa200ab8 and 0xfa200abc and the halfword at 0xfa200ac2,
  which sub_fe0051a0 initialises. It then stores halfwords and bytes into 0xfa203c80 to 0xfa203cac and
  0xfa2009c0, runs a byte-store loop that branches on Z at 0xfe048c18, and ends with stores to 0x705d50 and
  0xfa200030. The purpose of the cells at 0xfa203c8x is not derived.

- `sub_fe048c54` in `6.5.14/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** A 476-byte routine that tests the word at 0x705d50 at 0xfe048c8c and calls sub_fe048ba0 at 0xfe048c90
  with r9 set to 0x700000 when it is nonzero. It reads 0xfa203c80 and 0xfa203c82 at 0xfe048c9c and 0xfe048cb8, writes
  bytes to 0xfa200860, 0xfa200870 and 0xfa20086c at 0xfe048cf0 to 0xfe048d00, and runs a bdnz loop over halfword
  reads. It calls 0xfe044770 at 0xfe048e08 near the end. The purpose of the routine is not derived.

- `sub_c000a5e0` in `6.5.14/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** A 68-byte routine that reads the word at 0xfa200284 at 0xc000a5f4, calls 0xc0004710 at 0xc000a5ec,
  and reads and writes one byte through a computed pointer at 0xc000a60c and 0xc000a614. It calls 0xc0010640 at 0xc000a61c
  with r3 set to 0xc00f02d8, which looks like a message pointer. The read of 0xfa200284 suggests a status check,
  but that is not confirmed.

- `sub_c00a2374` in `6.5.14/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** Stores 0xfa200030 at 0xc00a2388 and four words into 0xc01624e8 to 0xc01624f8 at 0xc00a23a8 to
  0xc00a23d8. It calls 0xc000b454 at 0xc00a23b8 with r4 set to r3+4096, r5 set to 0xfa200000 and r7 set to 1. The
  0xfa200000 base suggests a mapping call, but the callee is not derived.

- `sub_c00a23ec` in `6.5.14/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** Stores words to 0xfa200940 and 0xfa200948 at 0xc00a2414 to 0xc00a2464 and reads 0xfa200940 back at
  0xc00a245c. It calls 0xc0003300 at 0xc00a2430 with r4 set to the address of sub_c00a247c, and calls 0xc0010094
  at 0xc00a2444 only when that result is nonzero, branching on it at 0xc00a2438. It then calls sub_c00a250c at
  0xc00a2458 on either path. The callee of 0xc0003300 is not derived.

- `sub_c00a247c` in `6.5.14/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** Writes the halfword at 0xfa200930 at 0xc00a2494 and reads it back at 0xc00a2498, the same
  pattern as sub_c00a193c. It makes an indirect call at 0xc00a24c8 through a loaded pointer, which it skips when
  that pointer is zero at 0xc00a24b8, and stores a word to 0xfa20094c at 0xc00a24f0. It is the address that
  sub_c00a23ec passes to 0xc0003300.

- `sub_c00a250c` in `6.5.14/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** Saves registers with stmw and calls sub_c00a2594 unconditionally at 0xc00a252c. It calls 0xc0010640
  with r3 set to 0xc010b340 at 0xc00a2558 when a test at 0xc00a2548 is nonzero. It reads 0xfa200948 at 0xc00a256c
  and uses indexed loads and stores on its frame, and that cell is the same one sub_c00a1a50 reads. The routine's caller is sub_c00a23ec, and its
  purpose beyond that is not derived.

- `sub_c00a2594` in `6.5.14/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** A leaf of 56 bytes that makes two indexed stores at 0xc00a25a8 and 0xc00a25ac, reads 0xfa200948 at
  0xc00a25b4, stores a word at 0xc00a25c4, and returns with r3 unknown. It is called by sub_c00a250c and reads the
  same cell 0xfa200948 as sub_c00a1a50.
