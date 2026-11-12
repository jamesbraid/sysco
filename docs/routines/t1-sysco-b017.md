# Tier-1 routine readings, sysco batch 17

Role statements for targets 425 to 449 of the tier-1 list, written from the rendered instruction
facts only (registers, calls, constants and branch targets). Each role is a reading of those facts,
not a confirmed contract. Callees are unnamed in the facts, so the roles name them by address. Calls
to 0xc0010640, 0xc0010094 and 0xfe004610 take a constant pointer as r3 and look like message emitters,
but the facts do not show the text. All 25 entries are inferred; none reach established.

- `sub_c00a56e0` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** Updates a queue-like state record. It calls 0xc00046d4 with a stack slot first, then 0xc0004710, and
  calls 0xc00046e0 last with the stack slot it passed to the first. It loads the pointer in the global at 0xc0160504
  and returns 1 when it is zero. Otherwise it unlinks that record by storing its +0x8 word back to the global, stores
  its arguments into the record at +0x0 and +0x4, clears +0x8, and tests the global at 0xc0160508. When that word is
  zero the record becomes the value of 0xc0160508 and 0xc016050c and the routine stores its first argument and 0x14 to
  the device words at 0xfa200e80 and 0xfa200e84; otherwise it links the record after the one at 0xc016050c and writes
  no device word. It returns 1 when no record was available and 0 otherwise. It looks like an entry and exit wrapper
  around a critical section; the callees are not in the facts.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a5eb0` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** A table-driven setup routine. It reads 16-bit values from 0xc012eac8, 0xc012eaca and 0xc012eacc,
  writes device words at 0xfa200e40, 0xfa200e44, 0xfa200e48 and 0xfa200e4c among others, and has three bdnz loops
  (0xc00a5f58 is a six-byte copy; 0xc00a6018 and 0xc00a6048 step through eight-byte table entries). A separate compare
  loop (ble at 0xc00a5ffc) over a pointer array calls 0xc00aa0a0 for a non-null pointer whose word at +0x70 is 1 or
  drops to zero after an atomic decrement. Its last call is sub_c00a56e0 with r3=0x600a0000 and r4=0xc00a5a60. The
  table's purpose is not resolved by the facts.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c01344f0` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01344f0
  **Inferred.** Reads the word at 0xc0160c7c and the device word at 0xfa200280, writes 0xfa200380 twice, sets a bit in
  0xfa200280, and stores halfwords to the device words 0xfa200220 (0xc01345a8 and 0xc01345b8) and 0xfa200200
  (0xc01345c8). It calls 0xc0010640 with a constant pointer as r3, a value computed from the 0xc0160c7c word as r4 and
  0x3c as r5, then registers through 0xc0003300 with r3=0xf and r4=0xc000a584. A nonzero return (beq at 0xc01345e8
  skips the call on zero) leads to a second message call through 0xc0010094, so it reads as a setup step that reports
  failure.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c013d7e4` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc013d7e4
  **Inferred.** A large bring-up initializer of about 1900 bytes. It calls 0xc000cbe4 with the record at 0xc0198ab0
  and length 0xc4, which is likely a clear, then fills fields of the records from 0xc0198ab0 to 0xc0198c28 and calls
  0xc000c8d4 with length 0x2c three times and 0xc4 once. It reads 0xc01604e8, which sub_c00a2374 writes, calls
  0xc006b73c twice with a branch on each result (0xc013d9f0 and 0xc013da0c), and then runs a loop of four passes (ble
  at 0xc013df2c). A pass is skipped when 0xc002778c returns zero (beq at 0xc013dbc8); otherwise it calls 0xc00a25cc
  and 0xc00a2600 (two call sites each), 0xc00a250c and 0xc00a2630. On two paths inside the loop it reads an indexed
  halfword from 0xc012eac0 or 0xc012eabc, writes it shifted left by 4 with bit 0 set to a device halfword and polls
  that halfword until bit 0 clears; the tables are read, not copied. The subsystem is not named in the facts.
  Reviewed: 2026-11-12 (sonnet)

- `sub_ffe5e878` in `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5e878
  **Inferred.** A coldfire routine that validates the six 32-bit fields of the record its first argument points to. It
  rejects with the value 0xfffff768 when +0x14 is above 0x258 or (when positive) below 0x1e, +0x10 above 0xc, +0xc
  above 0x1f, +0x8 above 0x18, +0x4 above 0x3c or +0x0 above 0x3d (compares at 0xffe5e88c to 0xffe5e8c4), which has
  the shape of seconds, minutes, hours, day, month and year. Otherwise it calls 0xffe60368 with the record, then calls
  0xffe60558 with the address of the result (a direct jsr at 0xffe5e8e4); when the field +0x20 of the returned object
  is nonzero it subtracts the global at 0x30000534 and calls 0xffe60558 again (jsr at 0xffe5e902). It then adds 0x76c
  to +0x14 and increments +0x10 and +0x18 of the returned object, and rewrites the bytes at 0x200007f8 to 0x200007ff
  and returns 0. The meaning of the record is not in the facts.
  Reviewed: 2026-11-12 (sonnet)

- `sub_ffe5e530` in `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin(l1#1)` at 0xffe5e530
  **Inferred.** The same coldfire validation and record update as sub_ffe5e878 in the l1#0 image, with the same field
  bounds at 0xffe5e54a to 0xffe5e57e. The jsr targets are direct addresses: it calls 0xffe60020, then 0xffe60210 (jsr
  at 0xffe5e59c), and a second 0xffe60210 only when +0x20 of the returned object is nonzero (beq at 0xffe5e5aa). It
  reads the global at 0x30000560 where the l1#0 copy reads 0x30000534, so its callee addresses and globals differ.
  Reviewed: 2026-11-12 (sonnet)

- `sub_ffe73e48` in `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin(l1#3)` at 0xffe73e48
  **Inferred.** The same coldfire record validation and update as sub_ffe5e878, with the same field bounds at
  0xffe73e62 to 0xffe73e96, but it rejects with 0xfffff894 rather than 0xfffff768. It calls 0xffe75938, then
  0xffe75b28, and a second 0xffe75b28 only when +0x20 of the returned object is nonzero (beq at 0xffe73ec2); the jsr
  targets are direct addresses. It reads the global at 0x30000cb8. The same instruction shape appears again at
  0xfff63e48 in this image.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fff63e48` in `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin(l1#3)` at 0xfff63e48
  **Inferred.** The same coldfire record validation as sub_ffe73e48, with the same field bounds at 0xfff63e62 to
  0xfff63e96 and the same 0xfffff894 rejection value. It calls 0xfff65938, then 0xfff65b28, and a second 0xfff65b28
  only when +0x20 of the returned object is nonzero (beq at 0xfff63ec2); the jsr targets are direct addresses. It
  reads the global at 0x30000cb8. The facts give no reason to read it as a different routine from the 0xffe73e48 copy.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe004efc` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** A flash-resident routine that never returns. It calls 0xfe0051f0 with 0xe01868 first, then the message
  emitter at 0xfe004610 with 0xe013ec, and tests the result of 0xfe004ff4 with r3=0xfe040000 (bne at 0xfe004f40). Only
  when that result is zero it emits 0xe01418, calls 0xfe004f98 and emits 0xe01428, 0xe01454 and 0xe01428 again. On
  either path it emits 0xe01474 (the emitter runs six times with five distinct constant pointers when the result is
  zero), stores 0xffffff80 to the device word 0xfa200004, calls 0xfe0046c8 and ends in a branch to itself
  (0xfe004f94). It looks like a boot-time check that reports its progress.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe0051a0` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** A short leaf with no calls that stores immediate constants to fixed device words: halfwords at
  0xfa200950 (0), 0xfa200952 (0xff), 0xfa200954 (0), 0xfa200960 (0x704), 0xfa200962 (0), 0xfa200964 (0), 0xfa200966
  (0x700) and 0xfa200ac2 (0x30), and words at 0xfa200ab8 (0x3030), 0xfa200abc (0x30) and 0xfa200ac4 (0x3000). It does
  not touch r3, so r3 returns unchanged. The stores are constants, not copies of anything the routine reads;
  sub_fe048ba0 read-modify-writes three of these words, so the pair looks like an initialise and then adjust of one
  device block.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe048ba0` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** Sets bits in the device registers at 0xfa200abc and 0xfa200ab8 (word read-modify-write, OR 0x30) and
  0xfa200ac2 (halfword, OR 0x30), the same addresses sub_fe0051a0 initialised. It stores constants to a block at
  0xfa203c80 (halfwords at +0x0 and +0x2, bytes at +0x4 and +0x5, halfwords at +0x6 and +0x2c), writes 0x11 to the
  halfword at 0xfa2009c0 and polls it until bit 0 clears (halfword loop at 0xfe048c10 to 0xfe048c18), then stores
  bytes at 0xfa200860, 0xfa200864, 0xfa200868, 0xfa200870 and 0xfa200874 and finishes by storing 1 to 0x705cd8 and to
  0xfa200030. The use of the 0xfa203c80 block is not in the facts.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe048c54` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** Calls sub_fe048ba0 only when the word at 0x705cd8 is zero (bne at 0xfe048c8c skips the call on
  nonzero). It then reads the halfwords at 0xfa203c82 and 0xfa203c80, aligns a buffer address near 0x706d6c to 16
  bytes, writes status bytes at 0xfa200860, 0xfa200870 and 0xfa20086c, and runs two timed polling loops (bdnz at
  0xfe048d44 and 0xfe048de4) that wait for bit 0x8000 of a halfword to clear, with a counter of 0x989680. On a timeout
  it calls 0xfe044770 with a constant pointer (0x705720 or 0x705740) and returns -1; otherwise it returns the halfword
  at +0x2 of a descriptor. The meaning of the status bytes is not resolved.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c000a5e0` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** A small stub that never returns. It calls 0xc0004710 first, sets bit 0x80 in the device word at
  0xfa200284 (lwz at 0xc000a5f4, stw at 0xc000a600), clears one bit of the machine state register with mfmsr and
  mtmsr, reads the device byte at 0xfa20028c (lbz at 0xc000a60c) into a stack slot (stb at 0xc000a614), then calls
  0xc0010640 with a constant pointer 0xc00f0164 and branches to itself. The facts do not show what the byte or the
  device word represents.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a2374` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** A setup stub that stores 0xfa200030 and the globals at 0xc01604ec, 0xc01604f0, 0xc01604f4 and
  0xc01604f8, then stores 0xc01604e8 after its call. It calls 0xc000b454 with r4 set to its argument plus
  4096, r5 set to the register base 0xfa200000, and r6, r8, r9 and r10 set to the global block at 0xc0160000.
  The 4 KB window and the base address suggest a mapping step, but the callee is not in the facts.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a23ec` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** Stores 0xe45f00 to the device word 0xfa200940 and 0 to 0xfa200948. It calls 0xc0003300 with r3=5 and
  r4=0xc00a247c, which is sub_c00a247c, so it appears to register that routine as a handler for id 5; a nonzero return
  (beq at 0xc00a2438) leads to a message call through 0xc0010094. It then calls sub_c00a250c with r3=0, r4=0xc00a2508
  and r5=0, where 0xc00a2508 is a bare blr at the end of sub_c00a247c's range, and finally sets bit 0x80 in
  0xfa200940.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a247c` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** The handler that sub_c00a23ec registers. It stores 1 to the halfword 0xfa200930, reads it back, uses
  the value shifted right by 11 as an index into an 8-byte-entry table at 0xc01989b0 (lwzx at 0xc00a24ac and
  0xc00a24c0). When the first loaded word is nonzero (beq at 0xc00a24b8) it makes an indirect call (blrl at
  0xc00a24c8) with the second word as r3 and its own r5 as r4; when it is zero it clears that bit in the word at
  0xfa200948. On both paths it then stores a one-bit mask (1 shifted left by the index) to 0xfa20094c, not the call
  result. The routine at 0xc00a2508 is a bare blr that sub_c00a23ec passes as an argument.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a250c` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** Saves r27 to r31 with stmw. When its second argument is zero it calls sub_c00a2594 at 0xc00a252c with
  the first argument and returns without touching the table (bne at 0xc00a2528 skips the call otherwise). Otherwise it
  indexes the 8-byte table with lwzx at 0xc00a2540; when the existing entry is nonzero (beq at 0xc00a2548) it first
  calls 0xc0010640 with a constant pointer at 0xc010a1d4, then stores the two arguments into the entry with stwx
  (0xc00a2560 and 0xc00a2564) and sets the matching bit in the device word at 0xfa200948, which sub_c00a23ec wrote. It
  ends with an lmw restore.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a2594` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** A 56-byte leaf called from sub_c00a250c. It stores two words into a table through stwx at
  0xc00a25a8 and 0xc00a25ac, reads 0xfa200948, stores one word at 0xc00a25c4, and returns. The facts give no
  more than those stores and that read.
  Reviewed: 2026-11-12 (sonnet)

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
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a515c` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** A 688-byte routine that reads the device word at 0xfa200e44 and branches on it, with many lbz,
  lhz and lwz accesses through unresolved pointers. It calls 0xc0010640 with constant pointers at 0xc010a894
  and 0xc010a8a4, calls 0xc00a540c and 0xc0013dec, and runs lwarx and stwcx. update sequences at 0xc00a52a8,
  0xc00a534c and 0xc00a5368 whose targets the renderer does not resolve. It calls sub_c00a563c at 0xc00a53e8,
  and the meaning of the device word that selects its paths is not in the facts.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a563c` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** A 164-byte routine that reads the global at 0xc0160508 and the device word at 0xfa200e80, and always
  stores 0 to the device word at 0xfa200e84. When the global is zero it calls 0xc0010640 with a constant pointer
  (0xc010a910) and the routine's first argument as r4, and returns. Otherwise it makes an indirect call at 0xc00a5698
  only when the word at +0x4 of that record is nonzero, passing the device word as r3 and its argument as r4, then
  unlinks the record, storing the next record into 0xc0160508 and the old head into 0xc0160504, and, when a next
  record exists, writes its first word to 0xfa200e80 and 0x14 to 0xfa200e84. The facts do not show what the device
  word encodes.
  Reviewed: 2026-11-12 (sonnet)

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
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a5eb0` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** The same table-driven setup as sub_c00a5eb0 in the SG0005762 kernel image (identical instructions). It
  reads 16-bit values from 0xc012eac8, 0xc012eaca and 0xc012eacc, writes device words at 0xfa200e40, 0xfa200e44,
  0xfa200e48 and 0xfa200e4c among others, and has three bdnz loops (0xc00a5f58 is a six-byte copy; 0xc00a6018 and
  0xc00a6048 step through eight-byte table entries). A separate compare loop (ble at 0xc00a5ffc) over a pointer array
  calls 0xc00aa0a0 for a non-null pointer whose word at +0x70 is 1 or drops to zero after an atomic decrement. Its
  last call is sub_c00a56e0 with r3=0x600a0000 and r4=0xc00a5a60. The table's purpose is not resolved by the facts.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c01344f0` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01344f0
  **Inferred.** The same setup step as sub_c01344f0 in the SG0005762 kernel image (identical instructions). Reads the
  word at 0xc0160c7c and the device word at 0xfa200280, writes 0xfa200380 twice, sets a bit in 0xfa200280, and stores
  halfwords to the device words 0xfa200220 (0xc01345a8 and 0xc01345b8) and 0xfa200200 (0xc01345c8). It calls
  0xc0010640 with a constant pointer as r3, a value computed from the 0xc0160c7c word as r4 and 0x3c as r5, then
  registers through 0xc0003300 with r3=0xf and r4=0xc000a584. A nonzero return (beq at 0xc01345e8 skips the call on
  zero) leads to a second message call through 0xc0010094, so it reads as a setup step that reports failure.
  Reviewed: 2026-11-12 (sonnet)

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
  Reviewed: 2026-11-12 (sonnet)
