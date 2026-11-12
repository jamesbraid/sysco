# Routine readings: sysco, tier-2 batch 2

These entries cover 25 routines from the 6.5.9, 6.5.10, 6.5.12 and cd-ist-3.24 sysco firmware trees: L1 and
L2 firmware segments, and the flashsc host tool. They are entries 50 to 74 (zero-based) of the ranked tier-2
reading list. Each was rendered from its code body and passed the grader, so none were rejected. Every grade
is inferred. The rendered facts give control flow, calls, constants and memory accesses. The roles below are
read from those facts and say where the facts stop short. Several analysis notes report instructions whose
effects were not modeled, so data flow through them is unknown.

- `sub_c00eccf0` in `6.5.9/usr/cpu/firmware/l2.bin(kernel)` at 0xc00eccf0
  **Inferred.** A PowerPC routine with a 0x20-byte frame that saves registers with stmw at 0xc00eccf8 and
  writes a word to the fixed address 0xfa200380 twice (0xc00ecd10, 0xc00ecd18). It reads a pointer from the
  table at 0xc00ead18 and makes one call to 0xc0010f08 at 0xc00ecd50 with r3=0xc00c5ed0 and r5=0x3c. Its purpose
  beyond that is not settled, because the multiply and divide instructions in the body have unmodeled effects.

- `sub_ffec7534` in `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffec7534
  **Inferred.** A ColdFire routine with a link.w frame and movem saves at 0xffec753c. It sets and clears single
  bits in the byte flags at abs 0x200007f8 and 0x200007ff (bset.b at 0xffec7546, bclr.b at 0xffec7590). Branches
  at 0xffec7628 to 0xffec7634 compare a selector with 0x2, 0x3 and 0x4, and it returns d0 = 0 on the path that
  reaches 0xffec7666. The meaning of the flag bits is not established by these facts.

- `sub_ffe75984` in `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin(l1#1)` at 0xffe75984
  **Inferred.** A ColdFire routine that reads six fields at offsets 0x0 to 0x14 of the structure at [sp+4] and
  checks each against a limit: 0x258 (bgt at 0xffe7599e), 0x1e, 0xc, 0x1f, 0x18 and 0x3c (blt and bgt branches
  at 0xffe759ac to 0xffe759cc). It then calls through the pointer at 0xffe774d0 (jsr at 0xffe759e0) and twice
  through 0xffe776c0 (jsr at 0xffe759f0 and 0xffe75a0e), updating fields of the return record at offsets 0x10 to
  0x18. The role is a range check on a configuration record followed by callouts; the record's meaning is inferred
  from the limits only.

- `sub_ffe7bca0` in `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin(l1#3)` at 0xffe7bca0
  **Inferred.** The same frame and field-limit checks as the routine at 0xffe75984 in the l1#1 copy, down to the
  same limit constants at 0xffe7bcc4 to 0xffe7bcec. It calls through 0xffe7fb4c (jsr at 0xffe7bcfc) and twice
  through 0xffe7fd3c (jsr at 0xffe7bd0c and 0xffe7bd2a), so it is a sibling of that routine with different
  callees. Its purpose beyond the range checks is inferred only from that shape.

- `sub_ffebf300` in `6.5.10/usr/cpu/firmware/l1.bin(l1#0)` at 0xffebf300
  **Inferred.** A ColdFire arithmetic routine with a link.w frame and movem saves at 0xffebf308. Its body is
  dominated by signed divide and multiply instructions (divs.l from 0xffebf378 to 0xffebf5aa, muls.l at
  0xffebf51e) and negations, and it returns d0 = ? at 0xffebf5dc. Because those instructions are unmodeled in
  these facts, the operands and the result are not settled, and the purpose is not established by the instruction mix alone.

- `sub_ffebf5e0` in `6.5.10/usr/cpu/firmware/l1.bin(l1#0)` at 0xffebf5e0
  **Inferred.** A ColdFire routine with a 4-byte link.w frame that sets one bit of the byte at abs 0x200007f8
  (bset.b at 0xffebf5e6). It then runs a loop body of 18 move.l stores and 9 add.l read-modify-write pairs,
  using extb.l and asr.l shifts, and returns d0 = 0 at 0xffebf7bc. It fills or accumulates a block of words; the
  block and its target are not named by these facts.

- `sub_ffebfd8c` in `6.5.10/usr/cpu/firmware/l1.bin(l1#0)` at 0xffebfd8c
  **Inferred.** A ColdFire wrapper that saves six longs with movem at 0xffebfd94. It calls the routine at 0xffebfedc twice
  (bsr.w at 0xffebfe3c and 0xffebfe74) and the routine at 0xffebb084 twice (jsr at 0xffebfeb8 and 0xffebfec6),
  passing no arguments, and returns d0 = 0 at 0xffebfed8. The callees are not read in this entry, so the work
  they do is unknown.

- `sub_fe048e3c` in `6.5.10/usr/cpu/firmware/l2.bin(flash)` at 0xfe048e3c
  **Inferred.** A PowerPC leaf routine that stores registers to fixed addresses in the 0xfa200950 to 0xfa200ac4
  range: sth at 0xfe048e44, 0xfe048e4c and 0xfe048e50 (halfwords) and stw at 0xfe048e58, 0xfe048e60 and 0xfe048e6c.
  It returns r3 unchanged at 0xfe048e88. The device behind the window is not named by these facts.

- `sub_fe0490d8` in `6.5.10/usr/cpu/firmware/l2.bin(flash)` at 0xfe0490d8
  **Inferred.** A PowerPC routine with a 0x20-byte frame that stores to its frame at sp-0x10 to sp+4 and writes
  abs 0xfa200004 (0xfe0490fc). It calls the routine at 0xfe048e3c (bl at 0xfe049114), then calls 0xfe04874c,
  0xfe0495d4, 0xfe044770 (four times, with the constant pointers 0xe05748, 0xe0578c, 0xe057bc and 0xe05628) and
  0xfe044fd4. It ends with two calls to 0xfe048e8c, with (r3, r4, r5) = (4, 0xc0000000, 8) and (5, 0xc0001000, 0xa),
  so it appears to register two regions by base and size during setup. That reading rests on the argument shape alone.

- `sub_c00c16e4` in `6.5.10/usr/cpu/firmware/l2.bin(kernel)` at 0xc00c16e4
  **Inferred.** A PowerPC routine with a 0x20-byte frame and an stmw save at 0xc00c16ec. It walks an indexed table
  (slwi at 0xc00c1700, lwzx at 0xc00c1704, stwx at 0xc00c172c and 0xc00c1730) and reads abs 0xfa200948 at
  0xc00c1738. It makes one call to 0xc0010f08 at 0xc00c1724 with r3=0xc00db4c0 and returns r3 at 0xc00c175c. The
  table's element type is not established.

- `sub_c00c4cfc` in `6.5.10/usr/cpu/firmware/l2.bin(kernel)` at 0xc00c4cfc
  **Inferred.** A PowerPC routine that reads the word at abs 0xc00f8278 and its neighbour at 0xc00f827c, stores
  the first back at 0xc00f8278 (0xc00c4d40), and calls 0xc0006930 at 0xc00c4d1c and 0xc000697c at 0xc00c4da4 with
  r9=0xc0100000. The body reads the machine state register (mfmsr at 0xc00c4d18) and writes it back (mtmsr at
  0xc00c4db0) around that update. Those two instructions have unmodeled effects here, so the save and
  restore is read from the opcodes, not from the values.

- `sub_c00f661c` in `6.5.10/usr/cpu/firmware/l2.bin(kernel)` at 0xc00f661c
  **Inferred.** A large PowerPC routine with a 0x30-byte frame, 15 stw stores to addresses the facts do not resolve and a stmw save. It
  calls 0xc0020400 and 0xc000ac80, then 0xc00b2084, then 0xc0022228 (twice) with 0x15, and 0xc0009acc and 0xc000a5c0
  in pairs after each. It makes a series of calls to 0xc0010f08 with pointers 0xc00dbb20 and 0xc00dbb44 (and one call with no constant argument), and calls 0xc0003db8 with r6=0xc00dbb00, 0xc0010870 with r3=0xc00dbb04 and 0xc00c53fc. Its shape suggests an initialization that prints or logs messages, but the
  callees are not read in this entry and the purpose is not settled.

- `sub_ffe5a210` in `6.5.12/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5a210
  **Inferred.** A ColdFire routine with a link.w frame and movem saves at 0xffe5a218, the same frame and divide
  and multiply mix as sub_ffebf300 in the 6.5.10 image. The divs.l and muls.l instructions run from 0xffe5a294 to
  0xffe5a4c6, and the return at 0xffe5a4f8 gives d0 = ? because those effects are unmodeled. The purpose is not
  established: the instruction mix and the shared frame are all the facts show.

- `sub_ffe5a4fc` in `6.5.12/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5a4fc
  **Inferred.** A ColdFire routine with the same shape as sub_ffebf5e0 in the 6.5.10 image: bset.b on abs
  0x200007f8 at 0xffe5a502, 18 move.l stores, 9 add.l read-modify-write pairs and a return of d0 = 0 at 0xffe5a6d8.
  It fills or accumulates a block of words, and the block is not named by these facts.

- `sub_ffe5aca8` in `6.5.12/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5aca8
  **Inferred.** A ColdFire wrapper with a 0x24-byte movem save area. It calls 0xffe5adf8 twice (bsr.w at 0xffe5ad58
  and 0xffe5ad90) and 0xffe57200 twice (jsr at 0xffe5add4 and 0xffe5ade2), with no arguments, and returns d0 = 0 at
  0xffe5adf4. It has the same shape as sub_ffebfd8c; the callees are not read here.

- `sub_ffeca210` in `6.5.12/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffeca210
  **Inferred.** A ColdFire routine with a link.w frame and movem saves at 0xffeca218, matching the frame and
  instruction counts of sub_ffe5a210 at 0xffe5a210 in the same image. It has the same divide and multiply mix, and
  it returns d0 = ? at 0xffeca4f8. The copy at 0xffeca210 and the copy at 0xffe5a210 are read as the same routine
  from those facts; byte-level equality was not checked in this entry.

- `sub_ffeca4fc` in `6.5.12/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffeca4fc
  **Inferred.** A ColdFire routine that matches sub_ffe5a4fc at 0xffe5a4fc in the same image: a bset.b on abs
  0x200007f8 at 0xffeca502, 18 move.l stores, 9 add.l pairs and a return of d0 = 0 at 0xffeca6d8. It fills or
  accumulates a block of words, and the block is not named by these facts.

- `sub_ffecaca8` in `6.5.12/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffecaca8
  **Inferred.** A ColdFire wrapper with the same shape as sub_ffe5aca8 at 0xffe5aca8. It calls 0xffecadf8 twice (bsr.w at
  0xffecad58 and 0xffecad90) and 0xffec7200 twice (jsr at 0xffecadd4 and 0xffecade2), with no arguments, and
  returns d0 = 0 at 0xffecadf4. The callees are not read in this entry.

- `sub_c00c1e08` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00c1e08
  **Inferred.** A PowerPC routine with the same shape as sub_c00c16e4 at 0xc00c16e4: a 0x20-byte frame, stmw at 0xc00c1e10, an
  indexed lwzx and stwx pair, a read of abs 0xfa200948 at 0xc00c1e5c and one call to 0xc0010f08 at 0xc00c1e48 with
  r3=0xc00dbec0. It returns r3 at 0xc00c1e80. The table it walks is not named by these facts.

- `sub_c00c4fec` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00c4fec
  **Inferred.** A PowerPC routine that reads a pointer from abs 0xc00f9288 and the word at abs 0xfa200e80. It calls
  0xc0010f08 at 0xc00c5024 with r3=0xc00dc2e4 and that word in r9, then makes an indirect call through the
  link register at 0xc00c5040 (blrl) with the word from 0xfa200e80 as r3 and r9. The indirect target is not resolved
  by these facts, so the call's effect is unknown.

- `sub_c00c5080` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00c5080
  **Inferred.** A PowerPC routine with a 0x20-byte frame that reads the pointer at abs 0xc00f9284 and the word at
  0xc00f9288, then stores a new value to 0xc00f9284 (0xc00c50c4). It calls 0xc0006930 at 0xc00c50a0 and 0xc000697c at
  0xc00c5128 with r9=0xc0100000, the same callee pair as sub_c00c4cfc at 0xc00c4cfc. The role is inferred from that
  pair and the pointer update.

- `sub_c00f7c04` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00f7c04
  **Inferred.** A large PowerPC routine that matches sub_c00f661c at 0xc00f661c in the 6.5.10 image: a 0x30-byte
  frame, 15 stw stores and a stmw save, then calls to 0xc0020408, 0xc000ac80, 0xc00b1ee0, 0xc0022230 (twice),
  0xc0009acc (twice), 0xc000a5c0 (twice), 0xc0003db8, 0xc0010870, 0xc00c58ac and 0xc0010f08 (three times), with
  pointers 0xc00dc490, 0xc00dc4ac and 0xc00dc4b4. The purpose is inferred from that shape only.

- `__start` in `6.5.12/usr/sbin/flashsc` at 0x10006110
  **Inferred.** An n32 start stub that calls 0x10006130 through bal at 0x10006128, passing a1 = sp+4 and the entry
  arguments a2 and a3. It loads and stores three table slots at 0x40e40, 0x4137c and 0x41380 (lw and sw at 0x1000616c,
  0x10006184 and 0x10006198), then makes four indirect calls (jalr at 0x100061a4, 0x100061b8, 0x100061f8 and
  0x10006210) with arguments read from those slots. The callees are not named, so the stub's full sequence is
  inferred from the call pattern.

- `FlashFileOpen` in `6.5.12/usr/sbin/flashsc` at 0x10008000
  **Inferred.** An n32 routine that opens a file with open (jalr at 0x1000805c) using flag 0 or 2, then calls fstat
  into a 264-byte stack buffer (0x100080c8) and malloc (0x10008150). It reads the file with read (0x100081d8) and
  validates its header with a call to FlashValidateFileHeader at 0x10008440 (jal at 0x10008260). Failures print to
  stderr through fprintf (format strings at 0x100428f0, 0x10042910, 0x10042930 and 0x10042960), close the descriptor
  and return -1 (v0 = -0x1 at 0x100080b4, 0x10008130, 0x100081c0, 0x10008248 and 0x100082a0). On success it returns
  the descriptor at 0x100082dc.

- `L2CfgMDeleteNode` in `6.5.12/usr/sbin/flashsc` at 0x1000ec10
  **Inferred.** An n32 routine that prints a debug line through fprintf (format at 0x100438b0, jalr at 0x1000ec5c)
  when the flag at sym(L2CfgDebug) is set (beq at 0x1000ec38). It deletes the peer through
  L2CfgMDeleteL2Peer at 0x1000fcb0 (jal at 0x1000ecb4) only when the word at [a0+20] is nonzero (the beq at
  0x1000ec74 skips that call), then deletes two L2 clients through L2CfgDeleteL2Client at
  0x1000fd70 using the record offsets 0x1eec and 0x1ee8 (jal at 0x1000ed40 and 0x1000edcc). It also calls 0x1000ef70
  three times. It returns v0 = 0 at 0x1000eccc, 0x1000ed58, 0x1000ede4 and 0x1000ee14. The record layout is inferred
  from the offsets alone.
