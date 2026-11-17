# Readings: `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)`

One entry per routine, filed here from the reading batches by binary.

- `sub_c000a5e0` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** A small stub that never returns. It calls 0xc0004710 first, sets bit 0x80 in the device word at
  0xfa200284 (lwz at 0xc000a5f4, stw at 0xc000a600), clears one bit of the machine state register with mfmsr and
  mtmsr, reads the device byte at 0xfa20028c (lbz at 0xc000a60c) into a stack slot (stb at 0xc000a614), then calls
  0xc0010640 with a constant pointer 0xc00f0164 and branches to itself. The facts do not show what the byte or the
  device word represents.
  Reviewed: 2026-11-12

- `sub_c00a2374` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** A setup stub that stores 0xfa200030 and the globals at 0xc01604ec, 0xc01604f0, 0xc01604f4 and
  0xc01604f8, then stores 0xc01604e8 after its call. It calls 0xc000b454 with r4 set to its argument plus
  4096, r5 set to the register base 0xfa200000, and r6, r8, r9 and r10 set to the global block at 0xc0160000.
  The 4 KB window and the base address suggest a mapping step, but the callee is not in the facts.
  Reviewed: 2026-11-12

- `sub_c00a23ec` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** Stores 0xe45f00 to the device word 0xfa200940 and 0 to 0xfa200948. It calls 0xc0003300 with r3=5 and
  r4=0xc00a247c, which is sub_c00a247c, so it appears to register that routine as a handler for id 5; a nonzero return
  (beq at 0xc00a2438) leads to a message call through 0xc0010094. It then calls sub_c00a250c with r3=0, r4=0xc00a2508
  and r5=0, where 0xc00a2508 is a bare blr at the end of sub_c00a247c's range, and finally sets bit 0x80 in
  0xfa200940.
  Reviewed: 2026-11-12

- `sub_c00a247c` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** The handler that sub_c00a23ec registers. It stores 1 to the halfword 0xfa200930, reads it back, uses
  the value shifted right by 11 as an index into an 8-byte-entry table at 0xc01989b0 (lwzx at 0xc00a24ac and
  0xc00a24c0). When the first loaded word is nonzero (beq at 0xc00a24b8) it makes an indirect call (blrl at
  0xc00a24c8) with the second word as r3 and its own r5 as r4; when it is zero it clears that bit in the word at
  0xfa200948. On both paths it then stores a one-bit mask (1 shifted left by the index) to 0xfa20094c, not the call
  result. The routine at 0xc00a2508 is a bare blr that sub_c00a23ec passes as an argument.
  Reviewed: 2026-11-12

- `sub_c00a250c` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** Saves r27 to r31 with stmw. When its second argument is zero it calls sub_c00a2594 at 0xc00a252c with
  the first argument and returns without touching the table (bne at 0xc00a2528 skips the call otherwise). Otherwise it
  indexes the 8-byte table with lwzx at 0xc00a2540; when the existing entry is nonzero (beq at 0xc00a2548) it first
  calls 0xc0010640 with a constant pointer at 0xc010a1d4, then stores the two arguments into the entry with stwx
  (0xc00a2560 and 0xc00a2564) and sets the matching bit in the device word at 0xfa200948, which sub_c00a23ec wrote. It
  ends with an lmw restore.
  Reviewed: 2026-11-12

- `sub_c00a2594` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** A 56-byte leaf called from sub_c00a250c. It stores two words into a table through stwx at
  0xc00a25a8 and 0xc00a25ac, reads 0xfa200948, stores one word at 0xc00a25c4, and returns. The facts give no
  more than those stores and that read.
  Reviewed: 2026-11-12

- `sub_c00a4e98` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** A 328-byte routine. It takes a pointer from the +0x30 word of the object at +0x60 of its second
  argument and loads a halfword through it (lhz at 0xc00a4eb8). When bit 0x8000 of that halfword is set it calls
  0xc0010640 and returns 1. Otherwise it rewrites that halfword and the next word from fields of the first argument
  (+0x80 and +0x5e), stores the first argument into an indexed slot of that object (index in the halfword at +0x20,
  wrapped to 0 to 7), calls 0xc0004868, then 0xc00046d4 and 0xc0004710, sets bits 0x9c00 in the halfword, stores the
  global at 0xc0160178 into +0x4c of its second argument, stores 0x1000000 to the device word 0xfa200e54, and advances
  its pointer by 8 or to the word at +0x28 depending on bit 0x2000. When the next entry's bit 0x8000 is set it stores
  the byte 1 at +0x98 (0xc00a4fa0) and sets bit 0 of the word at +0x2c of the second argument with lwarx and stwcx. It
  calls 0xc00046e0 last and returns 0. Its purpose is not settled by the facts.
  Reviewed: 2026-11-12

- `sub_c00a515c` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** A 688-byte routine that reads the device word at 0xfa200e44 and branches on it, with many lbz,
  lhz and lwz accesses through unresolved pointers. It calls 0xc0010640 with constant pointers at 0xc010a894
  and 0xc010a8a4, calls 0xc00a540c and 0xc0013dec, and runs lwarx and stwcx. update sequences at 0xc00a52a8,
  0xc00a534c and 0xc00a5368 whose targets the renderer does not resolve. It calls sub_c00a563c at 0xc00a53e8,
  and the meaning of the device word that selects its paths is not in the facts.
  Reviewed: 2026-11-12

- `sub_c00a563c` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** A 164-byte routine that reads the global at 0xc0160508 and the device word at 0xfa200e80, and always
  stores 0 to the device word at 0xfa200e84. When the global is zero it calls 0xc0010640 with a constant pointer
  (0xc010a910) and the routine's first argument as r4, and returns. Otherwise it makes an indirect call at 0xc00a5698
  only when the word at +0x4 of that record is nonzero, passing the device word as r3 and its argument as r4, then
  unlinks the record, storing the next record into 0xc0160508 and the old head into 0xc0160504, and, when a next
  record exists, writes its first word to 0xfa200e80 and 0x14 to 0xfa200e84. The facts do not show what the device
  word encodes.
  Reviewed: 2026-11-12

- `sub_c00a56e0` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** The same code as sub_c00a56e0 in the SG0005762 kernel image at the same address (the instruction
  streams are identical). It updates a queue-like state record. It calls 0xc00046d4 with a stack slot first, then
  0xc0004710, and calls 0xc00046e0 last with the stack slot it passed to the first. It loads the pointer in the global
  at 0xc0160504 and returns 1 when it is zero. Otherwise it unlinks that record by storing its +0x8 word back to the
  global, stores its arguments into the record at +0x0 and +0x4, clears +0x8, and tests the global at 0xc0160508. When
  that word is zero the record becomes the value of 0xc0160508 and 0xc016050c and the routine stores its first
  argument and 0x14 to the device words at 0xfa200e80 and 0xfa200e84; otherwise it links the record after the one at
  0xc016050c and writes no device word. It returns 1 when no record was available and 0 otherwise. It looks like an
  entry and exit wrapper around a critical section; the callees are not in the facts.
  Reviewed: 2026-11-12

- `sub_c00a5e34` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5e34
  **Inferred.** Loads the halfword at r3 +0x54 [@0xc00a5e48] and branches on it at 0xc00a5e58. When that branch falls through [@0xc00a5e58 fall] it calls 0xc0010640 with r3=0xc010aa3c and r4=r3 [call 1, @0xc00a5e58 fall] and reads [@0xc00a5e68] and writes back [@0xc00a5e70] the word at abs 0xfa200f44. When it is taken [@0xc00a5e58 taken] it reads [@0xc00a5e78] and writes back [@0xc00a5e80] the same word, loads the halfword at r3 +0x54 again [@0xc00a5e84] and, when the branch at 0xc00a5e8c falls through [@0xc00a5e8c fall], stores to 0xfa200e08 and 0xfa200e0c [@0xc00a5e94] [@0xc00a5e98]. Role (from facts): updates the word at 0xfa200f44 according to a flag in the halfword at +0x54 of its argument record [@0xc00a5e48] [@0xc00a5e68] and, on one path, writes fixed words at 0xfa200e08 and 0xfa200e0c [@0xc00a5e94], with one call to 0xc0010640 [call 1].
  Reviewed: 2026-11-12

- `sub_c00a5eb0` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** The same table-driven setup as sub_c00a5eb0 in the SG0005762 kernel image (identical instructions). It
  reads 16-bit values from 0xc012eac8, 0xc012eaca and 0xc012eacc, writes device words at 0xfa200e40, 0xfa200e44,
  0xfa200e48 and 0xfa200e4c among others, and has three bdnz loops (0xc00a5f58 is a six-byte copy; 0xc00a6018 and
  0xc00a6048 step through eight-byte table entries). A separate compare loop (ble at 0xc00a5ffc) over a pointer array
  calls 0xc00aa0a0 for a non-null pointer whose word at +0x70 is 1 or drops to zero after an atomic decrement. Its
  last call is sub_c00a56e0 with r3=0x600a0000 and r4=0xc00a5a60. The table's purpose is not resolved by the facts.
  Reviewed: 2026-11-12

- `sub_c01344f0` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01344f0
  **Inferred.** The same setup step as sub_c01344f0 in the SG0005762 kernel image (identical instructions). Reads the
  word at 0xc0160c7c and the device word at 0xfa200280, writes 0xfa200380 twice, sets a bit in 0xfa200280, and stores
  halfwords to the device words 0xfa200220 (0xc01345a8 and 0xc01345b8) and 0xfa200200 (0xc01345c8). It calls
  0xc0010640 with a constant pointer as r3, a value computed from the 0xc0160c7c word as r4 and 0x3c as r5, then
  registers through 0xc0003300 with r3=0xf and r4=0xc000a584. A nonzero return (beq at 0xc01345e8 skips the call on
  zero) leads to a second message call through 0xc0010094, so it reads as a setup step that reports failure.
  Reviewed: 2026-11-12

- `sub_c013d7e4` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc013d7e4
  **Inferred.** The same large bring-up initializer as sub_c013d7e4 in the SG0005762 kernel image (identical
  instructions). It calls 0xc000cbe4 with the record at 0xc0198ab0 and length 0xc4, which is likely a clear, then
  fills fields of the records from 0xc0198ab0 to 0xc0198c28 and calls 0xc000c8d4 with length 0x2c three times and 0xc4
  once. It reads 0xc01604e8, which sub_c00a2374 writes, calls 0xc006b73c twice with a branch on each result
  (0xc013d9f0 and 0xc013da0c), and then runs a loop of four passes (ble at 0xc013df2c). A pass is skipped when
  0xc002778c returns zero (beq at 0xc013dbc8); otherwise it calls 0xc00a25cc and 0xc00a2600 (two call sites each),
  0xc00a250c and 0xc00a2630. On two paths inside the loop it reads an indexed halfword from 0xc012eac0 or 0xc012eabc,
  writes it shifted left by 4 with bit 0 set to a device halfword and polls that halfword until bit 0 clears; the
  tables are read, not copied. The subsystem is not named in the facts.
  Reviewed: 2026-11-12
