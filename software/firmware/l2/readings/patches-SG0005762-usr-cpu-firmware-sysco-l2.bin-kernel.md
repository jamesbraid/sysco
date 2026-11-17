# Readings: `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)`

One entry per routine, filed here from the reading batches by binary.

- `sub_c000a5e0` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** Sets up a 0x20-byte frame, calls 0xc0004710 at 0xc000a5ec, reads the word at 0xfa200284 (lwz at
  0xc000a5f4), sets bit 0x80 and writes it back (store at 0xc000a600), then loads the byte at 0xfa20028c
  (0xc000a60c) and stores it to the stack (0xc000a614). It touches the machine state register with mfmsr at
  0xc000a5f8 and mtmsr at 0xc000a608, with an rlwinm between them, whose bits the renderer does not model. It
  calls 0xc0010640 with the constant 0xc00f0164 at 0xc000a61c and ends in a branch to itself at 0xc000a620. The
  MSR bits it changes are unknown.
  Reviewed: 2026-11-12

- `sub_c00a2374` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** Same code as the cd-ist-3.24 copy at 0xc00a2374: a 16-byte frame, five word stores into
  0xc01604e8 to 0xc01604f8 (four at 0xc00a23a8 to 0xc00a23b4, one at 0xc00a23d8), a call to 0xc000b454 with
  r5=0xfa200000 and r6=0xc0160000 from 0xc00a23b8, and a store of 1 to 0xfa200030 at 0xc00a2388. The tlbia at
  0xc00a23d0 is not modelled, so the TLB effect and the purpose of the store block are unclear.
  Reviewed: 2026-11-12

- `sub_c00a23ec` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** Same code as the cd-ist-3.24 copy at 0xc00a23ec: a frame, stores to 0xfa200940 and 0xfa200948,
  a call to 0xc0003300 with r3=5 and r4=0xc00a247c, a call to 0xc0010094 when that result is nonzero, and a call
  to 0xc00a250c at 0xc00a2458 (r3=0, r4=0xc00a2508, r5=0). After that call it sets bit 0x80 in the word at
  0xfa200940 and returns the call's r3. The callee at 0xc0003300 is not identified.
  Reviewed: 2026-11-12

- `sub_c00a247c` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** Same code as the cd-ist-3.24 copy at 0xc00a247c: a halfword store and reload at 0xfa200930, an
  index into the table at 0xc01989b0, a register-indirect call at 0xc00a24c8 made only when the table word is
  non-zero (target unresolved), an AND into the word at 0xfa200948 when it is zero, and a store to 0xfa20094c at
  0xc00a24f0 on both paths. The return value is unknown.
  Reviewed: 2026-11-12

- `sub_c00a250c` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** Same code as the cd-ist-3.24 copy at 0xc00a250c: stmw under a 0x20-byte frame, a call to 0xc00a2594
  at 0xc00a252c, a call to 0xc0010640 with the constant 0xc010a1d4 at 0xc00a2558, and a read of 0xfa200948 at
  0xc00a256c. The purpose beyond this call pattern is unclear.
  Reviewed: 2026-11-12

- `sub_c00a2594` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** Same code as the cd-ist-3.24 copy at 0xc00a2594: two indexed word stores at 0xc00a25a8 and
  0xc00a25ac, a word load from 0xfa200948 at 0xc00a25b4, and a store at 0xc00a25c4, returning an unresolved r3
  at 0xc00a25c8. The exact computation is unknown.
  Reviewed: 2026-11-12

- `sub_c00a4e98` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** Same code as the cd-ist-3.24 copy at 0xc00a4e98: a 0x30-byte frame, halfword loads and stores
  from 0xc00a4eb8 to 0xc00a4f28 (no backward branch there), an lwarx/stwcx. loop at 0xc00a4fac to 0xc00a4fb4, and
  calls to 0xc0010640, 0xc0004868, 0xc00046d4, 0xc0004710 and 0xc00046e0. The meaning of the reservation loop is
  unclear.
  Reviewed: 2026-11-12

- `sub_c00a515c` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** Same code as the cd-ist-3.24 copy at 0xc00a515c: a 0x40-byte frame, a test of the word at 0xfa200e44
  at 0xc00a5188, reservation loops at 0xc00a52a8, 0xc00a534c and 0xc00a5368, and a call to the routine at
  0xc00a563c at 0xc00a53e8. The overall purpose is not settled.
  Reviewed: 2026-11-12

- `sub_c00a563c` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** Same code as the cd-ist-3.24 copy at 0xc00a563c: walks the structure rooted at 0xc0160508,
  rewrites 0xc0160504 and 0xc0160508, and stores to 0xfa200e84 and, when the new root is non-null, to 0xfa200e80.
  When the record's word at +4 is non-zero it makes an indirect call through that word at 0xc00a5698, with the
  value loaded from 0xfa200e80 as r3; the target is not resolved. When the root is null it calls 0xc0010640 with
  the constant 0xc010a910 instead.
  Reviewed: 2026-11-12

- `sub_c00a56e0` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** Updates a queue-like state record. It calls 0xc00046d4 with a stack slot first, then 0xc0004710, and
  calls 0xc00046e0 last with the stack slot it passed to the first. It loads the pointer in the global at 0xc0160504
  and returns 1 when it is zero. Otherwise it unlinks that record by storing its +0x8 word back to the global, stores
  its arguments into the record at +0x0 and +0x4, clears +0x8, and tests the global at 0xc0160508. When that word is
  zero the record becomes the value of 0xc0160508 and 0xc016050c and the routine stores its first argument and 0x14 to
  the device words at 0xfa200e80 and 0xfa200e84; otherwise it links the record after the one at 0xc016050c and writes
  no device word. It returns 1 when no record was available and 0 otherwise. It looks like an entry and exit wrapper
  around a critical section; the callees are not in the facts.
  Reviewed: 2026-11-12

- `sub_c00a5e34` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5e34
  **Inferred.** Loads the halfword at r3 +0x54 [@0xc00a5e48] and branches on it at 0xc00a5e58. When that branch falls through [@0xc00a5e58 fall] it calls 0xc0010640 with r3=0xc010aa3c and r4=r3 [call 1, @0xc00a5e58 fall] and reads [@0xc00a5e68] and writes back [@0xc00a5e70] the word at abs 0xfa200f44. When it is taken [@0xc00a5e58 taken] it reads [@0xc00a5e78] and writes back [@0xc00a5e80] the same word, loads the halfword at r3 +0x54 again [@0xc00a5e84] and, when the branch at 0xc00a5e8c falls through [@0xc00a5e8c fall], stores to 0xfa200e08 and 0xfa200e0c [@0xc00a5e94] [@0xc00a5e98]. Role (from facts): updates the word at 0xfa200f44 according to a flag in the halfword at +0x54 of its argument record [@0xc00a5e48] [@0xc00a5e68] and, on one path, writes fixed words at 0xfa200e08 and 0xfa200e0c [@0xc00a5e94], with one call to 0xc0010640 [call 1].
  Reviewed: 2026-11-12

- `sub_c00a5eb0` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** A table-driven setup routine. It reads 16-bit values from 0xc012eac8, 0xc012eaca and 0xc012eacc,
  writes device words at 0xfa200e40, 0xfa200e44, 0xfa200e48 and 0xfa200e4c among others, and has three bdnz loops
  (0xc00a5f58 is a six-byte copy; 0xc00a6018 and 0xc00a6048 step through eight-byte table entries). A separate compare
  loop (ble at 0xc00a5ffc) over a pointer array calls 0xc00aa0a0 for a non-null pointer whose word at +0x70 is 1 or
  drops to zero after an atomic decrement. Its last call is sub_c00a56e0 with r3=0x600a0000 and r4=0xc00a5a60. The
  table's purpose is not resolved by the facts.
  Reviewed: 2026-11-12

- `sub_c01344f0` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01344f0
  **Inferred.** Reads the word at 0xc0160c7c and the device word at 0xfa200280, writes 0xfa200380 twice, sets a bit in
  0xfa200280, and stores halfwords to the device words 0xfa200220 (0xc01345a8 and 0xc01345b8) and 0xfa200200
  (0xc01345c8). It calls 0xc0010640 with a constant pointer as r3, a value computed from the 0xc0160c7c word as r4 and
  0x3c as r5, then registers through 0xc0003300 with r3=0xf and r4=0xc000a584. A nonzero return (beq at 0xc01345e8
  skips the call on zero) leads to a second message call through 0xc0010094, so it reads as a setup step that reports
  failure.
  Reviewed: 2026-11-12

- `sub_c013d7e4` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc013d7e4
  **Inferred.** A large bring-up initializer of about 1900 bytes. It calls 0xc000cbe4 with the record at 0xc0198ab0
  and length 0xc4, which is likely a clear, then fills fields of the records from 0xc0198ab0 to 0xc0198c28 and calls
  0xc000c8d4 with length 0x2c three times and 0xc4 once. It reads 0xc01604e8, which sub_c00a2374 writes, calls
  0xc006b73c twice with a branch on each result (0xc013d9f0 and 0xc013da0c), and then runs a loop of four passes (ble
  at 0xc013df2c). A pass is skipped when 0xc002778c returns zero (beq at 0xc013dbc8); otherwise it calls 0xc00a25cc
  and 0xc00a2600 (two call sites each), 0xc00a250c and 0xc00a2630. On two paths inside the loop it reads an indexed
  halfword from 0xc012eac0 or 0xc012eabc, writes it shifted left by 4 with bit 0 set to a device halfword and polls
  that halfword until bit 0 clears; the tables are read, not copied. The subsystem is not named in the facts.
  Reviewed: 2026-11-12
