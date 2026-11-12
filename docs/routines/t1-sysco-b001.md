# Tier-1 routine role statements, sysco batch 1

These statements describe what each routine does in software, read from its rendered instruction facts. They say nothing about hardware behavior. Where the facts do not settle a routine's purpose, the statement says so.

- `sub_fe048c20` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c20
  **Inferred.** Sets up a 0x20-byte frame and keeps r3 to r6 in r30, r28, r31 and r29. It loads the word at 0xc05bf8
  (0xfe048c40); when it is zero the routine calls 0xfe048b74 at 0xfe048c5c, and a nonzero word makes the `bne` at
  0xfe048c58 jump past that call to 0xfe048c60 (r9 holds 0xc00000 from 0xfe048c3c, not from the call site). It reads
  halfwords at 0xfa203c80 and 0xfa203c82, writes bytes to 0xfa200860, 0xfa20086c and 0xfa200870, and stores halfwords
  and words through two pointers near 0xfa2009c0. The two `bdnz` loops at 0xfe048d10 and 0xfe048db0 are polling loops:
  each re-reads the halfword at 0(r5) and leaves when bit 0x8000 is clear or the count in r9 (from 0x989680) runs out;
  they do not walk tables. When a count runs out, it calls 0xfe044770 with r3 0xc05640 or 0xc05660 (0xfe048dd4) and
  returns -1; otherwise it returns the halfword at 2(r7) (0xfe048dc4). The purpose is not settled.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe048dfc` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048dfc
  **Inferred.** A leaf routine of eleven stores and no calls. It writes halfwords to 0xfa200950, 0xfa200952,
  0xfa200954, 0xfa200960, 0xfa200962, 0xfa200964, 0xfa200966 and 0xfa200ac2, and words to 0xfa200ab8, 0xfa200abc and
  0xfa200ac4. The immediates are 0xff for 0xfa200952, 0x704 for 0xfa200960, 0x700 for 0xfa200966, 0x30 for 0xfa200ac2
  and 0xfa200abc, 0x3030 for 0xfa200ab8, 0x3000 for 0xfa200ac4, and 0 for the other four halfwords. It returns without
  setting r3. The purpose is not settled.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe049098` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe049098
  **Inferred.** Sets up a 0x20-byte frame, keeps r3 in r30 and writes fields through it at offsets +0x0 to +0xc and
  +0x18. It also writes a word to 0xfa200004 (0xfe0490bc) and a byte to 0xfa100003 (0xfe0490d0), then calls 0xfe048dfc
  with r9 0xfa100003 at 0xfe0490d4. The loop at 0xfe049110 is a poll that re-reads the halfword at 0xfa2009c0 until
  bit 0 clears. It then calls 0xfe04874c and 0xfe0495c4 (r3 8, r4 0). When 0xfe0495c4 returns nonzero, a byte-copy
  loop at 0xfe049174 to 0xfe04919c copies six bytes from that result to r3+0x10 and calls 0xfe044fd4 on each; calls to
  0xfe044770 pass 0xc057c4 (when the result is zero), 0xc05808, 0xc05838 (in the loop, for the first five bytes) and
  0xc056a4. Two calls to 0xfe048e4c pass r4 0xc0000000 and 0xc0001000 (0xfe0491bc, 0xfe0491d0). The callees are not
  named, so the purpose is partly unknown.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c0008f80` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc0008f80
  **Inferred.** A small wrapper. It sets up a 0x10-byte frame, loads the word at 0xfa20001c (0xc0008f90), shifts it
  right by 26 and passes the result as r4 to 0xc00041fc at 0xc0008f98. The load at 0xc0008f9c is the reload of the
  saved return address from the stack, not a second data read. r3 is not set before the call, and the routine returns
  with whatever r3 the call leaves. The meaning of the 0xfa20001c read is not settled.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00c1c34` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00c1c34
  **Inferred.** Saves registers with `stmw` and writes a halfword to 0xfa2009c0 (0xc00c1c54) and a word to 0xfa200030 (0xc00c1c6c). It stores four words to 0xc00f9254, 0xc00f9258, 0xc00f925c and 0xc00f9260 (0xc00c1c90 to 0xc00c1ca0), then calls 0xc0009acc at 0xc00c1ca8 with 0xc00dfc90, 0xc0100000, 0xfa200000 and 0x800 among its register arguments. The second call, to 0xc000a5c0 at 0xc00c1cc4, takes the word at 0xc00f925c as r4. The callees are unnamed, so the role is inferred from the stored addresses and arguments.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00c1ce4` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00c1ce4
  **Inferred.** Sets up a 0x10-byte frame and stores words to 0xfa200940 (0xc00c1d0c) and 0xfa200948 (0xc00c1d10). It
  calls 0xc0003db8 at 0xc00c1d28 with r3 5, the code address 0xc00c1d74 as r4 and 0xc00dbea0 as r6. A nonzero result
  leads to a call to 0xc0010870 with r3 0xc00dbea4 at 0xc00c1d3c, and a zero result skips it. It then calls 0xc00c1e08
  at 0xc00c1d50 with r3 0, r4 0xc00c1e04 and r5 0, reads 0xfa200940 back at 0xc00c1d54 and writes it again with bit
  0x80 set (0xc00c1d5c). The call is not the last step. r3 is not set after the 0xc00c1e08 call, so it returns that
  call's result. The facts name no callee, so the purpose is inferred from the call pattern.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00c1d74` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00c1d74
  **Inferred.** Writes 1 as a halfword to 0xfa200930 (0xc00c1d8c) and reads it back at 0xc00c1d90, then uses the read
  value shifted right by 11 as an index (r31) into a table of 8-byte entries at 0xc010b06c: `lwzx` at 0xc00c1da4 loads
  the first word of the entry and `lwzx` at 0xc00c1db4 loads the second. This is an indexed lookup, not a walk. When
  the first word is nonzero it is the target of the `blrl` at 0xc00c1dbc, with the second word as r3. When it is zero
  the routine instead clears bit r31 of the word at 0xfa200948 (0xc00c1dc4 to 0xc00c1dd4). On both paths it then sets
  bit r31 in the word at 0xfa20094c (0xc00c1ddc to 0xc00c1dec). The table contents are not in the facts, so the
  indirect target and the purpose are unknown.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00c4918` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00c4918
  **Inferred.** Saves r27 to r31 with `stmw` and keeps r4 in r28. It branches on the word at +0x24 of r4. When that
  word is nonzero, it subtracts the word at +0x48 from the global at 0xc00ede88 and compares the difference with 0x13:
  `bgt` at 0xc00c4954 continues when it is greater, and otherwise the routine returns 1 (0xc00c4958). This is a
  threshold on a difference, not a selector, and no jump table is involved. The continuing path calls 0xc0010f08 at
  0xc00c496c and 0xc00c49ac, runs two loops of eight calls to 0xc0010f08 (0xc00c49cc, 0xc00c49f0), stores to +0x48 and
  zero to +0x24, and returns 0. When the word at +0x24 is zero, it calls 0xc000712c at 0xc00c4a20 with r3 0 and tests
  the result, then calls it again at 0xc00c4a44 with r3 0 only if the first result was zero; a nonzero result reports
  through 0xc0010f08 (0xc00c4a5c) and returns 1. Otherwise it updates fields through pointers, calls 0xc00027b0 and
  0xc0006930, stores a word to 0xfa200e54 (0xc00c4b1c), reads 0xc00f9034 (0xc00c4b68) and calls 0xc000697c only when a
  bit in the saved r30 is set and that word is nonzero. The callees are not named, so the purpose is partly unknown.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00c4ba8` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00c4ba8
  **Inferred.** Saves r25 to r31 with `stmw` and keeps r4 in r27. When the word at +0x20 of r4 is nonzero it calls
  0xc0010f08 at 0xc00c4be0 with r3 0xc00dc240, before any test of the register word. It then stores 1 to +0x20 and
  reads the word at 0xfa200e44 (0xc00c4bec). When that word is zero the routine branches to 0xc00c4dc4 (0xc00c4bf4),
  which stores 0 to +0x20 and returns without setting r3, skipping the rest. Otherwise it writes the word back to
  0xfa200e44 (0xc00c4c00) and tests bits of it: 0xc0010f08 is called at 0xc00c4c14 (r3 0xc00dc268) when bits in mask
  0xe0400000 are set, 0xc00c4de0 when bits in 0x03000000 are set, and 0xc00c4fec when bit 0x00800000 is set. A loop
  starting at 0xc00c4c38 calls 0xc0006acc at 0xc00c4d08 and tests its result, calls 0xc0010f08 at 0xc00c4cdc (r3
  0xc00dc278) when bit 0x8000 is set in a halfword, and calls 0xc0006fe8 with r3 9 and r4 0xc00edf34 at 0xc00c4d7c
  under a further condition. The `bne` at 0xc00c4dc0 loops back to 0xc00c4bfc while the word at 0xfa200e44 is nonzero.
  The callees are not named, so the purpose is not settled.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00c58ac` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00c58ac
  **Inferred.** Saves r25 to r31 with `stmw` and keeps the pointer at +0x5c of r3 in r29. It stores four words to
  0xfa200e40, 0xfa200e48, 0xfa200e44 and 0xfa200e4c (0xc00c58e4 to 0xc00c5910), in that order. The `bdnz` at
  0xc00c58f4 branches to itself, a delay with no stores. It reads the halfwords at 0xc00edc84 and 0xc00edc86 and
  combines them into a word stored to 0xfa200e00 (0xc00c592c), and the `bdnz` loop at 0xc00c5958 copies six bytes from
  0xc00edc84 into r3+0x68. Two later `bdnz` loops (0xc00c59fc and 0xc00c5a34) fill ring entries with halfword stores.
  The word at 0xc00f9274 chooses which of two value pairs is stored to 0xfa200f44 and 0xfa200f84 (4 and 4, or 6 and
  that word); both paths run on, so it is a choice and not an optional path. It then calls 0xc0006cd4 at 0xc00c5a88,
  stores one of two halfwords to 0xfa200970 depending on its masked result, and calls 0xc00c5080 with r3 0x600a0000
  and r4 0xc00c541c at 0xc00c5ab4. The callees are unnamed, so the purpose is only partly settled.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00efd04` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00efd04
  **Inferred.** Saves registers with `stmw` at 0xc00efd0c and writes a word to 0xfa200380 twice (0xc00efd24, 0xc00efd2c). It reads 0xfa200280 at 0xc00efd30 and writes that location at 0xc00efd40. It reads a pointer from 0xc00eddc4 (0xc00efd38) and a field at +0x8 of it (0xc00efd44), then calls 0xc0010f08 at 0xc00efd64 with r3 0xc00c8710 and r5 0x3c. It restores registers with `lmw` at 0xc00efdbc. The read-then-write of 0xfa200280 is clear from the facts; the overall purpose is not settled.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00efdc8` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00efdc8
  **Inferred.** Sets up a 0x10-byte frame and stores words to 0xfa200300, 0xfa200320 and 0xfa20030c (0xc00efde4 to 0xc00efe00), then reads a halfword at 0xfa200220 (0xc00efe04). It calls 0xc0003db8 at 0xc00efe34 with r3 0xf, r4 0xc0008de4 and r6 0xc00c8754, and branches on the result at 0xc00efe3c. A nonzero result leads to a call to 0xc0010870 with r3 0xc00c875c at 0xc00efe48. The purpose is not settled beyond this setup pattern.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00f7210` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00f7210
  **Inferred.** The largest routine in the batch at 1896 bytes. It saves r14 to r31 with `stmw` and stores words and
  halfwords into a block running from 0xc010b16c to 0xc010b2e4, with a long run of word stores between 0xc00f7394 and
  0xc00f73f4. It calls 0xc000ac80 at 0xc00f7274 with r3 0xc010b16c and r4 0xc4, and calls 0xc000a9d0 at 0xc00f72d0 and
  0xc00f73fc with addresses inside that block as r3. It also calls 0xc0020408 with r3 0x78 and r4 0x15 at 0xc00f7604,
  and 0xc00c1ebc (r3 0x20) and 0xc00c1ef0 (r3 0x80) twice each from 0xc00f7664 to 0xc00f7724. The halfword updates at
  0xc00f7480 to 0xc00f74e8 are straight-line stores and read-modify-writes of 0xfa200950 to 0xfa200966, not loops. The
  `ble` at 0xc00f795c follows an unsigned `cmplwi` against 3 and is the back edge of an outer loop that runs the work
  from 0xc00f7524 for four values of r30. The callees are not named, so the purpose is partly unknown.
  Reviewed: 2026-11-12 (sonnet)

- `sub_ffe5d5c4` in `6.5.13/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5d5c4
  **Inferred.** Saves d2 to d7 with `movem.l` and sets bit 6 of the byte at 0x200007f8 with `bset.b` at 0xffe5d5d2. It
  reads the bytes at 0x200007f8, 0x200007ff, 0x200007fe, 0x200007fd, 0x200007fb, 0x200007fa and 0x200007f9 with
  `move.b`, splits their nibbles and combines them with shifts and adds, and clears bit 6 of 0x200007f8 with `bclr.b`
  at 0xffe5d702. It reads a longword from the table at 0x30000570 indexed by a computed value (0xffe5d71c, 0xffe5d794;
  an indexed read, not a walk) and stores a longword to 0x3000e5a8. The call sites at 0xffe5d79e, 0xffe5d7ae,
  0xffe5d7be and 0xffe5d7d2 call 0xffe58ee8 after `pea.l` pushes of 0x4a, 0x4c, 0x49 and 0x48. At most three execute
  in one run: the unsigned branches at 0xffe5d7a8 and 0xffe5d7b8 compare a value with the result of the 0x4a and 0x4c
  calls and either reach 0x49 or jump to 0x48. The routine returns the sum in d3 as d0 and stores it through the
  pointer argument when that pointer is nonzero. The purpose is not settled.
  Reviewed: 2026-11-12 (sonnet)

- `sub_ffe5dc30` in `6.5.13/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5dc30
  **Inferred.** Sets bit 6 of the byte at 0x200007f8 with `bset.b` at 0xffe5dc36 and clears it with `bclr.b` at
  0xffe5dda4. Between those it reads the bytes at 0x200007f8 through 0x200007ff, converts them into local words, and
  reads one longword from the table at 0x30000570 (0xffe5ddb4) into a local; the table is read, not updated. The
  signed branches at 0xffe5ddc2 and 0xffe5dde0 are sign and range tests, not a loop. At the end it copies nine
  longwords from the locals to the buffer addressed by its first argument. It makes no calls and returns constant 0 in
  d0 at 0xffe5de08. The facts do not settle what the values are for.
  Reviewed: 2026-11-12 (sonnet)

- `sub_ffecd5c4` in `6.5.13/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffecd5c4
  **Inferred.** The rendered facts match sub_ffe5d5c4 at 0xffe5d5c4 line for line, with the same byte at 0x200007f8 (`bset.b` at 0xffecd5d2, `bclr.b` at 0xffecd702), the same table longword at 0x30000570 and the same stores to 0x3000e5a8. The one difference is the callee: the four `jsr` at 0xffecd79e to 0xffecd7d2 call 0xffec8ee8 where the other copy calls 0xffe58ee8. The purpose is not settled, as for the other copy.
  Reviewed: 2026-11-12 (sonnet)

- `sub_ffecdc30` in `6.5.13/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffecdc30
  **Inferred.** The rendered facts match sub_ffe5dc30 at 0xffe5dc30 apart from branch targets. It sets bit 6 of the
  byte at 0x200007f8 with `bset.b` at 0xffecdc36 and clears it with `bclr.b` at 0xffecdda4, reads the bytes at
  0x200007f8 through 0x200007ff, and reads one longword from the table at 0x30000570 (0xffecddb4) into a local; the
  table is read, not updated. The signed branches at 0xffecddc2 and 0xffecdde0 are sign and range tests, not a loop.
  It copies nine longwords from locals to the buffer addressed by its first argument, makes no calls and returns 0 in
  d0 (`moveq` at 0xffecde08). The facts do not settle what the values are for.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe004ee8` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004ee8
  **Inferred.** Sets up a 0x20-byte frame and keeps 0xfe040000 in r29. It calls 0xfe0051dc at 0xfe004f10 with r3
  0xe01868 and r9 0xfe040000, then calls 0xfe0045fc at 0xfe004f1c with r3 0xe013ec. It calls 0xfe004fe0 at 0xfe004f24
  with r3 0xfe040000 and tests the result at 0xfe004f2c. Only when the result is zero (the `bne` skips them otherwise)
  it calls 0xfe0045fc with 0xe01418 (0xfe004f38), calls 0xfe004f84 with r3 0xfe040000 (0xfe004f40) and calls
  0xfe0045fc with 0xe01428, 0xe01454 and 0xe01428 again (0xfe004f4c to 0xfe004f60). On every path it stores a word to
  0xfa200004 at 0xfe004f70, calls 0xfe0045fc with r3 0xe01474 at 0xfe004f78 with r9 0xfa200000, and calls 0xfe0046b4
  at 0xfe004f7c with r3 as left by that call. It then loops on a branch to itself at 0xfe004f80 and does not return.
  The repeated constant addresses suggest text or message output, but the callee is not named, so this is inferred
  only.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe00518c` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe00518c
  **Inferred.** A leaf routine of eleven stores that does not set r3. It stores to the same addresses as sub_fe048dfc
  in 6.5.12, in the same order, with the same immediates: halfwords to 0xfa200952, 0xfa200950, 0xfa200954, 0xfa200962,
  0xfa200960, 0xfa200964, 0xfa200966 and 0xfa200ac2, and words to 0xfa200abc, 0xfa200ab8 and 0xfa200ac4, interleaved.
  The purpose is not settled.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe048b1c` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048b1c
  **Inferred.** A block of reads and writes with no calls. It reads and rewrites the words at 0xfa200abc and
  0xfa200ab8, each with 0x30 ORed in (`lwz` then `stw` at 0xfe048b24 to 0xfe048b40), and ORs 0x30 into the halfword at
  0xfa200ac2. It writes halfwords to 0xfa203c80, 0xfa203c82, 0xfa203c86 and 0xfa203cac, and bytes to 0xfa203c84 and
  0xfa203c85. It then writes 0x11 to the halfword at 0xfa2009c0 and the loop at 0xfe048b8c polls it until bit 0
  clears; this is a read loop, not a byte-store loop. After that it writes bytes to 0xfa200860, 0xfa200864,
  0xfa200868, 0xfa200874 and 0xfa200870, stores words to 0x705ce0 (0xfe048bc0) and 0xfa200030 (0xfe048bc8), and
  returns without setting r3. The purpose is not settled.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe048bd0` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048bd0
  **Inferred.** Sets up a 0x20-byte frame, reads the word at 0x705ce0 (0xfe048bf0) and branches on it at 0xfe048c08. A nonzero value skips the call at 0xfe048c0c, which is `sub_fe048b1c` with r9 0x700000, and a zero value makes that call. The rest reads halfwords at 0xfa203c80 and 0xfa203c82 and writes bytes to 0xfa200860, 0xfa200870 and 0xfa20086c, with `bdnz` loops at 0xfe048cc0 and 0xfe048d60. It calls 0xfe04475c at 0xfe048d84. The stores to 0xfa200860, 0xfa200870 and 0xfa20086c and the reads from 0xfa203c80 and 0xfa203c82 match sub_fe048c20 in 6.5.12, so this is probably the same routine in another build. The purpose is not settled.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe048dac` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048dac
  **Inferred.** A leaf routine of eleven stores that does not set r3, with the same addresses and immediates as
  sub_fe048dfc in 6.5.12: halfwords to 0xfa200952, 0xfa200950, 0xfa200954, 0xfa200962, 0xfa200960, 0xfa200964,
  0xfa200966 and 0xfa200ac2, and words to 0xfa200abc, 0xfa200ab8 and 0xfa200ac4, interleaved in the code. The purpose
  is not settled.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe049048` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe049048
  **Inferred.** Sets up a 0x20-byte frame, writes a word to 0xfa200004 (0xfe04906c) and a byte to 0xfa100003 (0xfe049080), and calls 0xfe048dac with r9 0xfa100003 at 0xfe049084. It writes fields through r3 at offsets +0x0 to +0xc and +0x18, then calls 0xfe0486fc at 0xfe0490e8 and 0xfe049578 with r3 8 and r4 0 at 0xfe0490f4. Four calls to 0xfe04475c pass constant addresses 0x705870, 0x7058b4, 0x7058e4 and 0x705750 (0xfe049108 to 0xfe04915c), and 0xfe044f8c is called at 0xfe049134. Two calls to 0xfe048dfc pass r4 0xc0000000 and 0xc0001000 (0xfe049170, 0xfe049184). The stores and call pattern resemble sub_fe049098 in 6.5.12, but the callees are different addresses and none is named, so the purpose is partly unknown.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a1834` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a1834
  **Inferred.** Sets up a 0x10-byte frame and stores a word to 0xfa200030 (0xc00a1848), then four words to 0xc015b4c4, 0xc015b4c8, 0xc015b4cc and 0xc015b4d0 (0xc00a1868 to 0xc00a1874) and one to 0xc015b4e4 (0xc00a1898). It calls 0xc000b678 at 0xc00a1878 with r4 set to r3 plus 0x1000, r5 0xfa200000, and r6, r8, r9 and r10 all 0xc0160000. After the call it reads through the returned pointer at 0xc00a187c and writes that pointer's word back at 0xc00a188c. The routine looks like a memory range setup, but the callee is not named, so this is inferred only.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a18ac` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a18ac
  **Inferred.** Has the same shape as sub_c00c1ce4 in 6.5.12: it stores words to 0xfa200940 (0xc00a18d4, 0xc00a1924)
  and 0xfa200948 (0xc00a18d8), calls 0xc0003304 at 0xc00a18f0 with r3 5, r4 0xc00a193c and r6 0xc0105e4c, and branches
  on the result at 0xc00a18f8. The nonzero path calls 0xc000fd1c with r3 0xc0105e50 at 0xc00a1904. It then calls
  0xc00a19c8 at 0xc00a1918 with r3 0, r4 0xc00a19c4 and r5 0, reads 0xfa200940 back and writes it with bit 0x80 set
  (0xc00a191c to 0xc00a1924), and returns the 0xc00a19c8 result in r3. The callees are not named, so the purpose is
  partly unknown.
  Reviewed: 2026-11-12 (sonnet)
