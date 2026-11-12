# Tier-1 routine readings, sysco batch 17

Role statements for targets 425 to 449 of the tier-1 list, written from the rendered instruction
facts only (registers, calls, constants and branch targets). Each role is a reading of those facts,
not a confirmed contract. Callees are unnamed in the facts, so the roles name them by address. Calls
to 0xc0010640, 0xc0010094 and 0xfe004610 take a constant pointer as r3 and look like message emitters,
but the facts do not show the text. All 25 entries are inferred; none reach established.

- `sub_c00a56e0` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** Updates a two-word state record. It loads the pointer in the global at 0xc0160504 and the
  words at 0xc0160508 and 0xc016050c, rewrites the record fields at +0x0, +0x4 and +0x8, then stores to
  the device words at 0xfa200e80 and 0xfa200e84. It calls 0xc00046d4 and 0xc0004710 first and 0xc00046e0
  last, with r10 set to the record pointer, which looks like an entry and exit wrapper; the callees are
  not in the facts.

- `sub_c00a5eb0` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** A table-driven setup routine. It reads 16-bit values from 0xc012eac8 and 0xc012eaca,
  writes device words at 0xfa200e40, 0xfa200e44, 0xfa200e48 and 0xfa200e4c, and walks its tables with
  counted loops (bdnz at 0xc00a5f58, 0xc00a6018 and 0xc00a6048). It calls 0xc00aa0a0 inside the loop and
  ends by calling sub_c00a56e0 with r3=0x600a0000 and r4=0xc00a5a60. The loop bounds and the table's
  purpose are not resolved by the facts.

- `sub_c01344f0` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01344f0
  **Inferred.** Reads the word at 0xc0160c7c and the device word at 0xfa200280, writes 0xfa200380 twice,
  and writes 16-bit values at 0xc01345a8, 0xc01345b8 and 0xc01345c8. It calls 0xc0010640 with a constant
  pointer and length 0x3c, then registers a handler through 0xc0003300 with r3=0xf and r4=0xc000a584. A
  nonzero return (beq at 0xc01345e8 skips the call on zero) leads to a second message call through
  0xc0010094, so it reads as a setup step that reports failure.

- `sub_c013d7e4` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc013d7e4
  **Inferred.** A large bring-up initializer of about 1900 bytes. It calls 0xc000cbe4 with the record at
  0xc0198ab0 and length 0xc4, which is likely a clear, then fills fields of the records from 0xc0198ab0 to
  0xc0198c28 and copies from the tables at 0xc012eac0 and 0xc012eabc. It reads 0xc01604e8, which sub_c00a2374
  writes, calls 0xc006b73c twice with a branch on its result at 0xc013d9f0, and calls 0xc00a25cc, 0xc00a2600,
  0xc00a250c and 0xc00a2630 later on. The subsystem is not named in the facts.

- `sub_ffe5e878` in `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5e878
  **Inferred.** A coldfire routine that validates the six 32-bit fields of the record its first argument
  points to. The compares at 0xffe5e89c to 0xffe5e8c4 test field +0x14 against 0x1e and 0x258, then +0x10
  against 0xc, +0xc against 0x1f, +0x8 against 0x18, +0x4 against 0x3c and +0x0 against 0x3d, which has the
  shape of seconds, minutes, hours, day and month. Past those checks it calls 0xffe60368 with the record,
  calls through the pointer slot 0xffe60558 twice (jsr at 0xffe5e8e4 and 0xffe5e902), updates fields +0x10,
  +0x14 and +0x18 of that object, and rewrites the bytes at 0x200007f8 to 0x200007ff. The meaning of the
  record is not in the facts.

- `sub_ffe5e530` in `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin(l1#1)` at 0xffe5e530
  **Inferred.** The same coldfire validation and record update as sub_ffe5e878 in the l1#0 image, with the
  same field bounds at 0xffe5e54a to 0xffe5e57e. This copy calls through the slots at 0xffe60020 and
  0xffe60210 and reads the global at 0x30000560 where the l1#0 copy reads 0x30000534, so its callee tables
  differ.

- `sub_ffe73e48` in `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin(l1#3)` at 0xffe73e48
  **Inferred.** The same coldfire record validation and update as sub_ffe5e878, with the same field bounds
  at 0xffe73e62 to 0xffe73e96. It calls through the slots at 0xffe75938 and 0xffe75b28 and reads the global
  at 0x30000cb8. The same instruction shape appears again at 0xfff63e48 in this image.

- `sub_fff63e48` in `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin(l1#3)` at 0xfff63e48
  **Inferred.** The same coldfire record validation as sub_ffe73e48, with the same field bounds at
  0xfff63e62 to 0xfff63e96. It calls through the slots at 0xfff65938 and 0xfff65b28 and reads the global at
  0x30000cb8. The facts give no reason to read it as a different routine from the 0xffe73e48 copy.

- `sub_fe004efc` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** A flash-resident status routine that calls the message emitter at 0xfe004610 six times
  with five distinct constant pointers, the first two of them 0xe013ec and 0xe01418. It calls 0xfe0051f0
  with 0xe01868 first, and tests the result of 0xfe004ff4 with r3=0xfe040000 (bne at 0xfe004f40), calling
  0xfe004f98 only when that result is zero. It stores 0xfa200004 at the end and calls 0xfe0046c8 with the
  emitter's result, so it looks like a boot-time check that reports its progress.

- `sub_fe0051a0` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** A short leaf that stores a group of values to fixed device words: halfwords at 0xfa200950,
  0xfa200952, 0xfa200954, 0xfa200960, 0xfa200962, 0xfa200964 and 0xfa200966, and words at 0xfa200ab8,
  0xfa200abc and 0xfa200ac4, with a halfword at 0xfa200ac2. It returns its first argument in r3. The facts do
  not say where each stored value comes from, and sub_fe048ba0 reads back three of these words, so the pair
  looks like a save and restore of one block.

- `sub_fe048ba0` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** Reads back the words and halfword that sub_fe0051a0 wrote at 0xfa200abc, 0xfa200ab8 and
  0xfa200ac2, and writes them through to a shadow block at 0xfa203c80 to 0xfa203cac. It also writes
  0xfa2009c0, runs a byte loop at 0xfe048c10 to 0xfe048c18, and finishes with stores to 0x705cd8 and
  0xfa200030. The use of the shadow block is not in the facts.

- `sub_fe048c54` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** Calls sub_fe048ba0 with r9=0x700000 only when the word at 0x705cd8 is zero (bne at
  0xfe048c8c skips the call on nonzero). It then reads the shadow halfwords at 0xfa203c82 and 0xfa203c80,
  writes status bytes at 0xfa200860, 0xfa200870 and 0xfa20086c, and walks halfword tables under bdnz loops
  at 0xfe048d44 and 0xfe048de4. It ends with a call to 0xfe044770, and the table contents and the status
  meanings are not resolved.

- `sub_c000a5e0` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** A small stub that reads the word at 0xfa200284 into its save area, stores a word through a
  pointer at 0xc000a600, and reads and writes one byte through another pointer (lbz at 0xc000a60c, stb at
  0xc000a614). It calls 0xc0004710 first and calls 0xc0010640 last with a constant pointer at 0xc00f0164.
  The facts do not show what the byte or the device word represents.

- `sub_c00a2374` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** A setup stub that stores 0xfa200030 and the globals at 0xc01604ec, 0xc01604f0, 0xc01604f4 and
  0xc01604f8, then stores 0xc01604e8 after its call. It calls 0xc000b454 with r4 set to its argument plus
  4096, r5 set to the register base 0xfa200000, and r6, r8, r9 and r10 set to the global block at 0xc0160000.
  The 4 KB window and the base address suggest a mapping step, but the callee is not in the facts.

- `sub_c00a23ec` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** Stores two words to the device block at 0xfa200940 and 0xfa200948 and reads 0xfa200940 back.
  It calls 0xc0003300 with r3=5 and r4=0xc00a247c, which is sub_c00a247c, so it appears to register that
  routine as a handler for id 5; a nonzero return (beq at 0xc00a2438) leads to a message call through
  0xc0010094. It then calls sub_c00a250c with r4=0xc00a2508, a word the rendered facts never reach.

- `sub_c00a247c` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** The handler that sub_c00a23ec registers. It stores a halfword to 0xfa200930, reads it back,
  and walks two lwzx-indexed tables. When a loaded word is nonzero (beq at 0xc00a24b8) it makes an indirect
  call at 0xc00a24c8 to a callee the facts do not resolve, then stores the result to 0xfa20094c. The word at
  0xc00a2508 is not reached on any path in the facts, though sub_c00a23ec passes it as an argument.

- `sub_c00a250c` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** Saves r27 to r31 with stmw and calls sub_c00a2594 at 0xc00a252c. It then reads the device
  word at 0xfa200948 that sub_c00a23ec wrote and walks a table with lwzx and stwx at 0xc00a2540 to
  0xc00a2564. On one path (beq at 0xc00a2548) it calls 0xc0010640 with a constant pointer at 0xc010a1d4, and
  it ends with an lmw restore.

- `sub_c00a2594` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** A 56-byte leaf called from sub_c00a250c. It stores two words into a table through stwx at
  0xc00a25a8 and 0xc00a25ac, reads 0xfa200948, stores one word at 0xc00a25c4, and returns. The facts give no
  more than those stores and that read.

- `sub_c00a4e98` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** A 328-byte routine that reads a table of 16-bit values with lhz from 0xc00a4eb8 to 0xc00a4ef8
  and writes local state. It reads the global at 0xc0160178, writes 0xfa200e54, and stores a byte at 0xc00a4fa0.
  It calls 0xc0004868 and 0xc00046d4 with 0xc0004710 near the end, then 0xc00046e0 last. Its purpose is not
  settled by the facts.

- `sub_c00a515c` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** A 688-byte routine that reads the device word at 0xfa200e44 and branches on it, with many lbz,
  lhz and lwz accesses through unresolved pointers. It calls 0xc0010640 with constant pointers at 0xc010a894
  and 0xc010a8a4, calls 0xc00a540c and 0xc0013dec, and runs lwarx and stwcx. update sequences at 0xc00a52a8,
  0xc00a534c and 0xc00a5368 whose targets the renderer does not resolve. It calls sub_c00a563c at 0xc00a53e8,
  and the meaning of the device word that selects its paths is not in the facts.

- `sub_c00a563c` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** A 164-byte routine that reads the globals at 0xc0160504 and 0xc0160508 and the device word at
  0xfa200e80, writes new values back to all three, and calls 0xc0010640 with the device word as an argument.
  It then makes an indirect call at 0xc00a5698 to an unresolved callee, passing the device word as r3. The
  facts do not show what that value encodes.

- `sub_c00a56e0` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** The same code as sub_c00a56e0 in the SG0005762 kernel image at the same address. It updates
  the state record through the globals at 0xc0160504 and 0xc0160508, stores device words at 0xfa200e80 and
  0xfa200e84, and calls 0xc00046d4, 0xc0004710 and 0xc00046e0 around the update. The callee roles are not in
  the facts.

- `sub_c00a5eb0` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** The same table-driven setup as sub_c00a5eb0 in the SG0005762 kernel image. It reads 16-bit
  values from 0xc012eac8, writes device words at 0xfa200e40 to 0xfa200e4c, runs counted loops, and calls
  sub_c00a56e0 with r3=0x600a0000 and r4=0xc00a5a60 at the end. The loop bounds and the table's purpose are
  unresolved.

- `sub_c01344f0` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01344f0
  **Inferred.** The same setup step as sub_c01344f0 in the SG0005762 kernel image. It reads 0xc0160c7c and
  0xfa200280, writes 0xfa200380 twice, writes 16-bit values in a table, and calls 0xc0010640 with a constant
  pointer and length 0x3c. It registers a handler through 0xc0003300 with r3=0xf, and a nonzero return leads
  to a message call through 0xc0010094.

- `sub_c013d7e4` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc013d7e4
  **Inferred.** The same large bring-up initializer as sub_c013d7e4 in the SG0005762 kernel image. It calls
  0xc000cbe4 on the 0xc4-byte record at 0xc0198ab0, fills the records from 0xc0198ab0 to 0xc0198c28, and
  reads 0xc01604e8 that sub_c00a2374 writes. It calls 0xc006b73c twice with a branch on its result, and the
  subsystem it brings up is not named in the facts.
