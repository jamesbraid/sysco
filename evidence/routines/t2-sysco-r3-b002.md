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
  Reviewed: 2026-11-12

- `sub_ffec7534` in `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffec7534
  **Inferred.** A ColdFire routine with a link.w frame and movem saves at 0xffec753c. It sets bit 6 of the byte
  at abs 0x200007f8 (bset.b at 0xffec7546) and clears the same bit of the same byte (bclr.b at 0xffec7590); the
  byte at abs 0x200007ff is only read in between, not set or cleared. Branches at 0xffec7628 to 0xffec7634
  compare a selector with 0x2, 0x3 and 0x4, and it returns d0 = 0 at the single return at 0xffec7666. The
  meaning of the byte values is not established.
  Reviewed: 2026-11-12

- `sub_ffe75984` in `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin(l1#1)` at 0xffe75984
  **Inferred.** A ColdFire routine that reads fields at offsets 0x0 to 0x14 of the structure whose pointer is
  its first stack argument ([sp+4] at entry) and compares each against a limit: 0x258 (bgt at 0xffe7599e), 0x1e
  (bgt at 0xffe759ac, reached only when the +0x14 field is positive), 0xc (+0x10), 0x1f (+0xc), 0x18 (+0x8),
  0x3c (+0x4, blt at 0xffe759cc) and 0x3d (the word at +0x0, bge at 0xffe759d2). Any failed check goes to
  0xffe759d4, which loads d0 = -0x76c and returns through 0xffe75b9a. Otherwise it calls the routine at
  0xffe774d0 (jsr at 0xffe759e0, absolute address) with the structure pointer, calls 0xffe776c0 (jsr at
  0xffe759f0) with a pointer to the result, and calls 0xffe776c0 a second time (jsr at 0xffe75a0e) only when the
  word at +0x20 of the record returned by the first 0xffe776c0 call is nonzero. It then adds 0x76c to the +0x14
  field and increments the +0x10 and +0x18 fields of the record returned by 0xffe776c0, and writes bytes within
  abs 0x200007f8 to 0x200007ff. Role (from facts only): a range check on a record of seven compared words
  followed by callouts; the record's meaning is inferred from the limits only.
  Reviewed: 2026-11-12

- `sub_ffe7bca0` in `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin(l1#3)` at 0xffe7bca0
  **Inferred.** The same frame and field-limit checks as the routine at 0xffe75984 in the l1#1 copy, down to the
  same limit constants at 0xffe7bcb4 to 0xffe7bcec (including the 0x3d check on the word at +0x0). It calls the
  routine at 0xffe7fb4c (jsr at 0xffe7bcfc, absolute address) and then 0xffe7fd3c (jsr at 0xffe7bd0c), with a
  second call to 0xffe7fd3c (jsr at 0xffe7bd2a) only when the word at +0x20 of the record returned by the first
  0xffe7fd3c call is nonzero, so it is a sibling of that routine with different callees. Its purpose beyond the
  range checks is inferred only from that shape.
  Reviewed: 2026-11-12

- `sub_ffebf300` in `6.5.10/usr/cpu/firmware/l1.bin(l1#0)` at 0xffebf300
  **Inferred.** A ColdFire arithmetic routine with a link.w frame and movem saves at 0xffebf308. Its body is
  dominated by signed divide and multiply instructions (divs.l from 0xffebf378 to 0xffebf5aa, muls.l at
  0xffebf51e) and negations, and it returns d0 = ? at 0xffebf5dc. Because those instructions are unmodeled in
  these facts, the operands and the result are not settled, and the purpose is not established by the instruction mix alone.
  Reviewed: 2026-11-12

- `sub_ffebf5e0` in `6.5.10/usr/cpu/firmware/l1.bin(l1#0)` at 0xffebf5e0
  **Inferred.** A ColdFire routine with a link.w frame of 0x28 bytes (link.w a6,#-0x28). It sets bit 6 of the
  byte at abs 0x200007f8 (bset.b at 0xffebf5e6) and clears it again later (bclr.b at 0xffebf754). Between them
  it reads the bytes at abs 0x200007f9 to 0x200007ff, splitting each into nibbles with extb.l, asr.l and and.l,
  and combining them with add.l into nine words in its frame (nine move.l stores and nine add.l
  read-modify-write updates on frame slots). It then copies those nine words with nine move.l (a1)+,(a0)+ stores
  to the pointer passed as its first stack argument and returns d0 = 0 at 0xffebf7bc. There is no loop; the
  sequence is straight-line. The record's meaning is not named by these facts.
  Reviewed: 2026-11-12

- `sub_ffebfd8c` in `6.5.10/usr/cpu/firmware/l1.bin(l1#0)` at 0xffebfd8c
  **Inferred.** A ColdFire wrapper with a link.w frame and a movem save of d2-d7 at 0xffebfd94. It calls the
  routine at 0xffebfedc twice (bsr.w at 0xffebfe3c and 0xffebfe74), each time after pushing one longword (move.l
  d5,-(a7)), and calls the routine at 0xffebb084 twice (jsr at 0xffebfeb8 and 0xffebfec6), each time after
  pushing a register longword and a constant (0x4a, then 0x4c, via pea). It also clears a bit of the byte at abs
  0x200007f8 (bclr.b at 0xffebfe04) and returns d0 = 0 at 0xffebfed8. The callees are not read in this entry, so
  the work they do is unknown.
  Reviewed: 2026-11-12

- `sub_fe048e3c` in `6.5.10/usr/cpu/firmware/l2.bin(flash)` at 0xfe048e3c
  **Inferred.** A PowerPC leaf routine that stores registers to fixed addresses in the 0xfa200950 to 0xfa200ac4
  range: sth at 0xfe048e44, 0xfe048e4c and 0xfe048e50 (halfwords) and stw at 0xfe048e58, 0xfe048e60 and 0xfe048e6c.
  It returns r3 unchanged at 0xfe048e88. The device behind the window is not named by these facts.
  Reviewed: 2026-11-12

- `sub_fe0490d8` in `6.5.10/usr/cpu/firmware/l2.bin(flash)` at 0xfe0490d8
  **Inferred.** A PowerPC routine with a 0x20-byte frame that stores to its frame at sp-0x10 to sp+4 relative to
  the entry sp, writes abs 0xfa200004 (stw at 0xfe0490fc) and a byte at abs 0xfa100003 (stb at 0xfe049110). It
  calls the routine at 0xfe048e3c (bl at 0xfe049114), then 0xfe04874c with the entry r3, then 0xfe0495d4 with
  (r3, r4) = (8, 0). When that result is zero it calls 0xfe044770 with 0xe05748 and goes to 0xfe0491bc;
  otherwise it calls 0xfe044770 with 0xe0578c, loops six times calling 0xfe044fd4 and (while the counter is at
  most 4) 0xfe044770 with 0xe057bc, and finishes with 0xfe044770 with 0xe05628. It then calls 0xfe049204 and
  ends with two calls to 0xfe048e8c, with (r3, r4, r5) = (4, 0xc0000000, 8) and (5, 0xc0001000, 0xa), so it
  appears to register two regions by base and size during setup. That reading rests on the argument shape alone.
  Reviewed: 2026-11-12

- `sub_c00c16e4` in `6.5.10/usr/cpu/firmware/l2.bin(kernel)` at 0xc00c16e4
  **Inferred.** A PowerPC routine with a 0x20-byte frame and an stmw save at 0xc00c16ec. It walks an indexed
  table (slwi at 0xc00c1700, lwzx at 0xc00c1704, stwx at 0xc00c172c and 0xc00c1730). Only when the loaded table
  word is nonzero (beq at 0xc00c1714 skips it) it makes one call to 0xc0010f08 at 0xc00c1724 with r3=0xc00db4c0.
  It then reads abs 0xfa200948 (0xc00c1738), ORs in a bit chosen by the first argument and stores the word back
  (stw at 0xc00c1748), and returns r3 at 0xc00c175c. The table's element type is not established.
  Reviewed: 2026-11-12

- `sub_c00c4cfc` in `6.5.10/usr/cpu/firmware/l2.bin(kernel)` at 0xc00c4cfc
  **Inferred.** A PowerPC routine that calls 0xc0006930 at 0xc00c4d1c (after mfmsr at 0xc00c4d18). It then reads
  the word at abs 0xc00f8278 and its neighbour at 0xc00f827c. When the first word is nonzero it stores the word
  at offset 8 of the record that word points to back to 0xc00f8278 (stw at 0xc00c4d40, a different value from
  the one read) and writes the entry arguments and a zero into that record. When the first word is zero it skips
  these updates. If the saved machine state has bit 0x8000 set and a word at 0xc00f8034 is nonzero it calls
  0xc000697c at 0xc00c4da4 with r3 = the saved state and r9=0xc0100000; otherwise it writes the state back with
  mtmsr at 0xc00c4db0. The mfmsr and mtmsr instructions have unmodeled effects here, so the save and restore is
  read from the opcodes, not from the values.
  Reviewed: 2026-11-12

- `sub_c00f661c` in `6.5.10/usr/cpu/firmware/l2.bin(kernel)` at 0xc00f661c
  **Inferred.** A large PowerPC routine with a 0x30-byte frame and an stmw save. It makes many stw stores to
  addresses the facts do not resolve. It calls 0xc0020400 and 0xc000ac80, then 0xc00b2084, then 0xc0022228 with
  0x15 (once before a loop and once in a loop that runs four times), each followed by 0xc0009acc and 0xc000a5c0.
  It calls 0xc0003db8 with r6=0xc00dbb00, then 0xc0010870 with r3=0xc00dbb04 only when 0xc0003db8 returns
  nonzero, calls 0xc00c53fc, and ends with three calls to 0xc0010f08 with pointers 0xc00dbb20, 0xc00dbb3c (in a
  loop of five passes) and 0xc00dbb44. Its shape suggests an initialization that prints or logs messages, but
  the callees are not read in this entry and the purpose is not settled.
  Reviewed: 2026-11-12

- `sub_ffe5a210` in `6.5.12/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5a210
  **Inferred.** A ColdFire routine with a link.w frame and movem saves at 0xffe5a218, the same frame and divide
  and multiply mix as sub_ffebf300 in the 6.5.10 image. The divs.l and muls.l instructions run from 0xffe5a294 to
  0xffe5a4c6, and the return at 0xffe5a4f8 gives d0 = ? because those effects are unmodeled. The purpose is not
  established: the instruction mix and the shared frame are all the facts show.
  Reviewed: 2026-11-12

- `sub_ffe5a4fc` in `6.5.12/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5a4fc
  **Inferred.** A ColdFire routine with the same shape as sub_ffebf5e0 in the 6.5.10 image: bset.b on abs
  0x200007f8 at 0xffe5a502, 18 move.l stores, 9 add.l read-modify-write pairs and a return of d0 = 0 at 0xffe5a6d8.
  It fills or accumulates a block of words, and the block is not named by these facts.
  Reviewed: 2026-11-12

- `sub_ffe5aca8` in `6.5.12/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5aca8
  **Inferred.** A ColdFire wrapper with a link.w frame and a movem save of d2-d7 (0x18 bytes). It calls
  0xffe5adf8 twice (bsr.w at 0xffe5ad58 and 0xffe5ad90), each after pushing one longword, and 0xffe57200 twice
  (jsr at 0xffe5add4 and 0xffe5ade2), each after pushing a register longword and a constant (0x4a, then 0x4c,
  via pea). It also clears a bit of the byte at abs 0x200007f8 (bclr.b at 0xffe5ad20) and returns d0 = 0 at
  0xffe5adf4. It has the same shape as sub_ffebfd8c; the callees are not read here.
  Reviewed: 2026-11-12

- `sub_ffeca210` in `6.5.12/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffeca210
  **Inferred.** A ColdFire routine with a link.w frame and movem saves at 0xffeca218, matching the frame and
  instruction counts of sub_ffe5a210 at 0xffe5a210 in the same image. It has the same divide and multiply mix, and
  it returns d0 = ? at 0xffeca4f8. The copy at 0xffeca210 and the copy at 0xffe5a210 are read as the same routine
  from those facts; byte-level equality was not checked in this entry.
  Reviewed: 2026-11-12

- `sub_ffeca4fc` in `6.5.12/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffeca4fc
  **Inferred.** A ColdFire routine that matches sub_ffe5a4fc at 0xffe5a4fc in the same image: a bset.b on abs
  0x200007f8 at 0xffeca502, 18 move.l stores, 9 add.l pairs and a return of d0 = 0 at 0xffeca6d8. It fills or
  accumulates a block of words, and the block is not named by these facts.
  Reviewed: 2026-11-12

- `sub_ffecaca8` in `6.5.12/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffecaca8
  **Inferred.** A ColdFire wrapper with the same shape as sub_ffe5aca8 at 0xffe5aca8. It calls 0xffecadf8 twice
  (bsr.w at 0xffecad58 and 0xffecad90), each after pushing one longword, and 0xffec7200 twice (jsr at 0xffecadd4
  and 0xffecade2), each after pushing a register longword and a constant, and returns d0 = 0 at 0xffecadf4. The
  callees are not read in this entry.
  Reviewed: 2026-11-12

- `sub_c00c1e08` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00c1e08
  **Inferred.** A PowerPC routine with the same shape as sub_c00c16e4 at 0xc00c16e4: a 0x20-byte frame, stmw at
  0xc00c1e10, an indexed lwzx and stwx pair, and a read of abs 0xfa200948 at 0xc00c1e5c followed by a store of
  the word with one bit ORed in (stw at 0xc00c1e6c). It makes one call to 0xc0010f08 at 0xc00c1e48 with
  r3=0xc00dbec0, only when the loaded table word is nonzero (beq at 0xc00c1e38 skips it). It returns r3 at
  0xc00c1e80. The table it walks is not named by these facts.
  Reviewed: 2026-11-12

- `sub_c00c4fec` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00c4fec
  **Inferred.** A PowerPC routine that reads a pointer from abs 0xc00f9288 and the word at abs 0xfa200e80. When
  the pointer is zero it calls 0xc0010f08 at 0xc00c5024 with r3=0xc00dc2e4 and the entry r3 in r4, and finishes.
  When the pointer is nonzero and the word at its +4 is nonzero, it makes an indirect call through the link
  register at 0xc00c5040 (blrl) with the word from 0xfa200e80 in r3 and r9. It then stores to the pointer
  variable, offset 8 of the record and a second variable (stw at 0xc00c5050, 0xc00c5058 and 0xc00c505c) and,
  when a next record exists, writes that record's first word to 0xfa200e80 (stw at 0xc00c5068). The indirect
  target is not resolved by these facts, so the call's effect is unknown.
  Reviewed: 2026-11-12

- `sub_c00c5080` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00c5080
  **Inferred.** A PowerPC routine with a 0x20-byte frame that reads the machine state register (mfmsr at
  0xc00c509c) and calls 0xc0006930 at 0xc00c50a0. It then reads the pointer at abs 0xc00f9284 and, when it is
  nonzero, the word at 0xc00f9288 and stores the word at offset 8 of the record the pointer addresses to
  0xc00f9284 (0xc00c50c4). When that pointer is zero it skips the update. It calls 0xc000697c at 0xc00c5128 with
  r3 = the saved state and r9=0xc0100000 only when bit 0x8000 of the saved state and a word at abs 0xc00f9034
  are set (the callee pair 0xc0006930 and 0xc000697c matches sub_c00c4cfc at 0xc00c4cfc). The role is inferred
  from that pair and the pointer update.
  Reviewed: 2026-11-12

- `sub_c00f7c04` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00f7c04
  **Inferred.** A large PowerPC routine that matches sub_c00f661c at 0xc00f661c in the 6.5.10 image: a 0x30-byte
  frame, an stmw save, many stw stores, then calls to 0xc0020408, 0xc000ac80, 0xc00b1ee0, 0xc0022230 (twice, one
  in a loop), 0xc0009acc (twice), 0xc000a5c0 (twice), 0xc0003db8, 0xc0010870 (only when 0xc0003db8 returns
  nonzero), 0xc00c58ac and 0xc0010f08 (three times, the second in a loop), with pointers 0xc00dc490, 0xc00dc4ac
  and 0xc00dc4b4. The purpose is inferred from that shape only.
  Reviewed: 2026-11-12

- `__start` in `6.5.12/usr/sbin/flashsc` at 0x10006110
  **Inferred.** An n32 start stub. The bal at 0x10006128 targets 0x10006130, the instruction after its delay
  slot, which is the usual idiom for reading the program counter to build the global pointer, so it is not a
  call to a separate routine; and the facts show it with a1 = sp+4. It then loads three slots at 0x40e40,
  0x4137c and 0x41380 and stores to them (sw at 0x1000616c, 0x10006184 and 0x10006198); the store at 0x1000616c
  is skipped when the word loaded from the 0x40e40 slot is nonzero (bne at 0x1000615c). It makes four indirect
  calls (jalr at 0x100061a4, 0x100061b8, 0x100061f8 and 0x10006210). The first (0x100061a4) has a3 read from the
  0x40e40 slot and the third (0x100061f8) has a0, a1 and a2 read from the 0x4137c, 0x41380 and 0x40e40 slots;
  the facts leave the other arguments unknown, and the callees are not named, so the stub's full sequence is
  inferred from the call pattern.
  Reviewed: 2026-11-12

- `FlashFileOpen` in `6.5.12/usr/sbin/flashsc` at 0x10008000
  **Inferred.** An n32 routine that opens a file with open (jalr at 0x1000805c) using flag 2 when its sixth
  argument (the word at sp+0x10c) is nonzero and flag 0 otherwise (beq at 0x10008034). If open returns a
  negative value (bgez at 0x10008074) it prints through fprintf (format at 0x100428f0, destination
  sym(__iob)+32) and returns -1 (v0 = -0x1 at 0x100080b4) without closing. Otherwise it calls fstat into a
  264-byte stack buffer (0x100080c8), then malloc (0x10008150), then read (0x100081d8), then
  FlashValidateFileHeader at 0x10008440 (jal at 0x10008260). Three later failures (fstat negative, malloc zero,
  and a read result that differs from a value the facts do not resolve) print through fprintf (format strings at
  0x10042910, 0x10042930 and 0x10042960), close the descriptor and return -1 (v0 = -0x1 at 0x10008130,
  0x100081c0 and 0x10008248). A negative result from FlashValidateFileHeader closes the descriptor and returns
  -1 (v0 = -0x1 at 0x100082a0) with no fprintf in the facts. On success it returns the descriptor at 0x100082dc.
  The destination +32 of __iob is presumably stderr, which the facts do not confirm.
  Reviewed: 2026-11-12

- `L2CfgMDeleteNode` in `6.5.12/usr/sbin/flashsc` at 0x1000ec10
  **Inferred.** An n32 routine that prints a debug line through fprintf (format at 0x100438b0, jalr at
  0x1000ec5c) when the flag at sym(L2CfgDebug) is set (beq at 0x1000ec38 skips it when clear). The three delete
  stages are each guarded. When the word at [a0+20] is nonzero (beq at 0x1000ec74 skips the stage) it appears to
  loop (bne at 0x1000ece4 goes back to 0x1000ec7c): it calls 0x1000ef70 (call at 0x1000ec94) and, when that
  returns nonzero, calls L2CfgMDeleteL2Peer at 0x1000fcb0 (jal at 0x1000ecb4); the facts list a return of v0 = 0
  at 0x1000eccc right after that call. Separately, when the word at [a0+0x1eec] is nonzero it runs the same
  pattern with L2CfgDeleteL2Client at 0x1000fd70 and a0+0x1eec (jal at 0x1000ed40, return at 0x1000ed58), and
  likewise for [a0+0x1ee8] and a0+0x1ee8 (jal at 0x1000edcc, return at 0x1000ede4). 0x1000ef70 is called three
  times in all (once per stage). The routine also returns v0 = 0 at 0x1000ee14. The record layout is inferred
  from the offsets alone.
  Reviewed: 2026-11-12
