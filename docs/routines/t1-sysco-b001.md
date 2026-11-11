# Tier-1 routine role statements, sysco batch 1

These statements describe what each routine does in software, read from its rendered instruction facts. They say nothing about hardware behavior. Where the facts do not settle a routine's purpose, the statement says so.

- `sub_fe048c20` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c20
  **Inferred.** Sets up a 0x20-byte frame, then tests the word at 0xc05bf8 (`bne` at 0xfe048c58). A zero word leads to a call to 0xfe048b74 with r9 set to 0xc00000 at 0xfe048c5c, and a nonzero word jumps past it to 0xfe048c60. The rest is halfword and byte work on 0xfa200860, 0xfa200870 and 0xfa20086c and reads of 0xfa203c80 and 0xfa203c82, with `bdnz` loops over halfword tables and one call to 0xfe044770 at 0xfe048dd4. The facts do not show what the stored values mean, so the purpose is not settled.

- `sub_fe048dfc` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048dfc
  **Inferred.** A leaf routine of eleven stores and no calls. It writes halfwords to 0xfa200950, 0xfa200952, 0xfa200954, 0xfa200960, 0xfa200962, 0xfa200964, 0xfa200966 and 0xfa200ac2, and words to 0xfa200ab8, 0xfa200abc and 0xfa200ac4, then returns r3. The values stored are not in the facts, so the purpose is a state setup whose content is not settled.

- `sub_fe049098` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe049098
  **Inferred.** Sets up a 0x20-byte frame and writes fields through r3 at offsets +0x0 to +0xc and +0x18. It also writes a word to 0xfa200004 (0xfe0490bc) and a byte to 0xfa100003 (0xfe0490d0), then calls 0xfe048dfc with r9 0xfa100003 at 0xfe0490d4. A series of calls to 0xfe044770 passes constant addresses 0xc057c4, 0xc05808, 0xc05838 and 0xc056a4 (0xfe049154 to 0xfe0491a8), and two calls to 0xfe048e4c pass r4 0xc0000000 and 0xc0001000 (0xfe0491bc, 0xfe0491d0). A halfword loop runs between 0xfe049110 and 0xfe04919c. The shape looks like descriptor setup plus a table walk, but the callees are not named, so the purpose is partly unknown.

- `sub_c0008f80` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc0008f80
  **Inferred.** A small wrapper. It sets up a 0x10-byte frame, reads the word at 0xfa20001c (0xc0008f90), reads a second word through a register (0xc0008f9c), calls 0xc00041fc at 0xc0008f98 and returns that call's result in r3. The argument setup is not in the facts, so the meaning of the 0xfa20001c read is not settled.

- `sub_c00c1c34` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00c1c34
  **Inferred.** Saves registers with `stmw` and writes a halfword to 0xfa2009c0 (0xc00c1c54) and a word to 0xfa200030 (0xc00c1c6c). It stores four words to 0xc00f9254, 0xc00f9258, 0xc00f925c and 0xc00f9260 (0xc00c1c90 to 0xc00c1ca0), then calls 0xc0009acc at 0xc00c1ca8 with 0xc00dfc90, 0xc0100000, 0xfa200000 and 0x800 among its register arguments. The second call, to 0xc000a5c0 at 0xc00c1cc4, takes the word at 0xc00f925c as r4. The callees are unnamed, so the role is inferred from the stored addresses and arguments.

- `sub_c00c1ce4` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00c1ce4
  **Inferred.** Sets up a 0x10-byte frame and stores words to 0xfa200940 (including 0xc00c1d0c and 0xc00c1d5c) and 0xfa200948 (0xc00c1d10), then reads 0xfa200940 back at 0xc00c1d54. It calls 0xc0003db8 at 0xc00c1d28 with r3 5, the code address 0xc00c1d74 as r4 and 0xc00dbea0 as r6. A nonzero result leads to a call to 0xc0010870 with r3 0xc00dbea4 at 0xc00c1d3c, and a zero result skips it. It ends by calling 0xc00c1e08 with r4 0xc00c1e04 and returns that call's result. The facts name no callee, so the purpose is inferred from the call pattern.

- `sub_c00c1d74` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00c1d74
  **Inferred.** Writes a halfword to 0xfa200930 (0xc00c1d8c) and reads it back at 0xc00c1d90, then walks a table with `lwzx` loads at 0xc00c1da4 and 0xc00c1db4. It makes an indirect call with `blrl` at 0xc00c1dbc whose target is not known from the facts. It also reads the word at 0xfa20094c (0xc00c1ddc), and its return value is unknown. Most of the behavior depends on that indirect call, so the purpose is unclear.

- `sub_c00c4918` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00c4918
  **Inferred.** Saves registers with `stmw` and runs a selector check: `bgt` at 0xc00c4954 branches when the value exceeds 0x13, which suggests a selector with up to twenty cases, but the facts show no jump table. The main path makes six calls to 0xc0010f08, two with address arguments 0xc00dc188 and 0xc00dc1a4 and one with 0xc00dc228, and calls 0xc000712c twice with r3 0 and tests its result. It stores a word to 0xfa200e54 (0xc00c4b1c) and reads 0xc00f9034 near the end. The callees are not named, so the purpose is partly unknown.

- `sub_c00c4ba8` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00c4ba8
  **Inferred.** Saves registers and reads the word at 0xfa200e44 (0xc00c4bec). When that word is zero the routine branches to 0xc00c4dc4 (0xc00c4bf4), skipping the main path. The main path calls 0xc0010f08 at 0xc00c4be0, 0xc00c4c14 and 0xc00c4cdc with address arguments 0xc00dc240, 0xc00dc268 and 0xc00dc278, calls 0xc0006acc at 0xc00c4d08 and tests its result, and calls 0xc0006fe8 with r3 9 and r4 0xc00edf34 at 0xc00c4d7c. A loop at 0xc00c4dc0 branches back to 0xc00c4bfc. The facts do not name the callees, so the purpose is not settled.

- `sub_c00c58ac` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00c58ac
  **Inferred.** Saves registers and stores four words to 0xfa200e40, 0xfa200e44, 0xfa200e48 and 0xfa200e4c (0xc00c58e4 to 0xc00c5910). It fills tables with halfword stores in two `bdnz` loops at 0xc00c58f4 and 0xc00c5958, reading the halfwords at 0xc00edc84 and 0xc00edc86 first. It calls 0xc0006cd4 with r9 0x14 and r10 0x8 at 0xc00c5a88, and calls 0xc00c5080 with r3 0x600a0000 and r4 0xc00c541c at 0xc00c5ab4. The word at 0xc00f9274 selects an optional path at 0xc00c5a50. The callees are unnamed, so the purpose is only partly settled.

- `sub_c00efd04` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00efd04
  **Inferred.** Saves registers with `stmw` at 0xc00efd0c and writes a word to 0xfa200380 twice (0xc00efd24, 0xc00efd2c). It reads 0xfa200280 at 0xc00efd30 and writes that location at 0xc00efd40. It reads a pointer from 0xc00eddc4 (0xc00efd38) and a field at +0x8 of it (0xc00efd44), then calls 0xc0010f08 at 0xc00efd64 with r3 0xc00c8710 and r5 0x3c. It restores registers with `lmw` at 0xc00efdbc. The read-then-write of 0xfa200280 is clear from the facts; the overall purpose is not settled.

- `sub_c00efdc8` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00efdc8
  **Inferred.** Sets up a 0x10-byte frame and stores words to 0xfa200300, 0xfa200320 and 0xfa20030c (0xc00efde4 to 0xc00efe00), then reads a halfword at 0xfa200220 (0xc00efe04). It calls 0xc0003db8 at 0xc00efe34 with r3 0xf, r4 0xc0008de4 and r6 0xc00c8754, and branches on the result at 0xc00efe3c. A nonzero result leads to a call to 0xc0010870 with r3 0xc00c875c at 0xc00efe48. The purpose is not settled beyond this setup pattern.

- `sub_c00f7210` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00f7210
  **Inferred.** The largest routine in the batch at 1896 bytes. It saves registers with `stmw` and then stores words and halfwords into a block running from 0xc010b16c to 0xc010b2e4, with a long run of word stores between 0xc00f7394 and 0xc00f73f4. It calls 0xc000ac80 at 0xc00f7274 with r3 0xc010b16c and r4 0xc4, and calls 0xc000a9d0 at 0xc00f72d0 and 0xc00f73fc with addresses inside that block as arguments. It also calls 0xc0020408 with 0x78 and 0x15 at 0xc00f7604, and 0xc00c1ebc and 0xc00c1ef0 with 0x20 and 0x80 at 0xc00f7664 to 0xc00f7724. Its later loops with halfword stores at 0xc00f7480 to 0xc00f74e8 and a signed `ble` at 0xc00f795c look like table processing, but the callees are not named, so the purpose is partly unknown.

- `sub_ffe5d5c4` in `6.5.13/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5d5c4
  **Inferred.** Saves registers with `movem.l` and sets a bit in the byte at 0x200007f8 with `bset.b` at 0xffe5d5d2. It reads the bytes at 0x200007f8 through 0x200007ff with `move.b`, and clears the bit at 0x200007f8 with `bclr.b` at 0xffe5d702. It walks a table through the longword at 0x30000570 (0xffe5d71c, 0xffe5d794) and stores longwords to 0x3000e5a8. It calls 0xffe58ee8 four times, from 0xffe5d79e to 0xffe5d7d2, each after pushing arguments with `pea.l`, and compares a value against that call's result with unsigned branches at 0xffe5d7a8 and 0xffe5d7b8. The return value is not settled.

- `sub_ffe5dc30` in `6.5.13/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5dc30
  **Inferred.** Sets the bit at 0x200007f8 with `bset.b` at 0xffe5dc36 and clears it with `bclr.b` at 0xffe5dda4. Between those it reads the bytes at 0x200007f8 through 0x200007ff and updates a set of longwords through pointers, including the table longword at 0x30000570 (0xffe5ddb4). Signed branches at 0xffe5ddc2 and 0xffe5dde0 bound a loop. It makes no calls and returns constant 0 in d0 at 0xffe5de0c. The facts do not settle what the updates are for.

- `sub_ffecd5c4` in `6.5.13/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffecd5c4
  **Inferred.** The rendered facts match sub_ffe5d5c4 at 0xffe5d5c4 line for line, with the same byte at 0x200007f8 (`bset.b` at 0xffecd5d2, `bclr.b` at 0xffecd702), the same table longword at 0x30000570 and the same stores to 0x3000e5a8. The one difference is the callee: the four `jsr` at 0xffecd79e to 0xffecd7d2 call 0xffec8ee8 where the other copy calls 0xffe58ee8. The purpose is not settled, as for the other copy.

- `sub_ffecdc30` in `6.5.13/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffecdc30
  **Inferred.** The rendered facts match sub_ffe5dc30 at 0xffe5dc30. It sets the bit at 0x200007f8 with `bset.b` at 0xffecdc36 and clears it with `bclr.b` at 0xffecdda4, reads the bytes at 0x200007f8 through 0x200007ff, and updates longwords through pointers, including the table longword at 0x30000570 (0xffecddb4). Signed branches at 0xffecddc2 and 0xffecdde0 bound a loop. It makes no calls and returns 0 in d0 at 0xffecde0c. The facts do not settle what the updates are for.

- `sub_fe004ee8` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004ee8
  **Inferred.** Sets up a 0x20-byte frame and makes six calls to 0xfe0045fc at 0xfe004f1c to 0xfe004f78, each with a different constant address in r3 from 0xe013ec to 0xe01474, the last one also setting r9 to 0xfa200000. It calls 0xfe0051dc at 0xfe004f10 with r3 0xe01868 and r9 0xfe040000, and calls 0xfe004fe0 at 0xfe004f24 with r3 0xfe040000 and tests its result at 0xfe004f2c. It calls 0xfe004f84 at 0xfe004f40 with r3 0xfe040000 and passes the result of 0xfe0045fc into 0xfe0046b4 at 0xfe004f7c. It stores a word to 0xfa200004 at 0xfe004f70. The repeated constant addresses suggest text or message output, but the callee is not named, so this is inferred only.

- `sub_fe00518c` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe00518c
  **Inferred.** A leaf routine of eleven stores and a return of r3. It stores to the same addresses as sub_fe048dfc in 6.5.12, in the same order: halfwords to 0xfa200952, 0xfa200950, 0xfa200954, 0xfa200962, 0xfa200960, 0xfa200964, 0xfa200966 and 0xfa200ac2, and words to 0xfa200abc, 0xfa200ab8 and 0xfa200ac4. The values stored are not in the facts, so the purpose is not settled.

- `sub_fe048b1c` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048b1c
  **Inferred.** A block of reads and writes with no calls. It reads and rewrites the words at 0xfa200abc and 0xfa200ab8 (`lwz` then `stw` at 0xfe048b24 to 0xfe048b40) and updates the halfword at 0xfa200ac2. It writes halfwords to 0xfa203c80, 0xfa203c82, 0xfa203c86 and 0xfa203cac, and bytes to 0xfa203c84 and 0xfa203c85. A byte-store loop at 0xfe048b8c closes the middle section, then it stores words to 0x705ce0 (0xfe048bc0) and 0xfa200030 (0xfe048bc8) and returns r3 unknown. The read-then-write pairs do not show what changes, so the purpose is not settled.

- `sub_fe048bd0` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048bd0
  **Inferred.** Sets up a 0x20-byte frame, reads the word at 0x705ce0 (0xfe048bf0) and branches on it at 0xfe048c08. A nonzero value skips the call at 0xfe048c0c, which is `sub_fe048b1c` with r9 0x700000, and a zero value makes that call. The rest reads halfwords at 0xfa203c80 and 0xfa203c82 and writes bytes to 0xfa200860, 0xfa200870 and 0xfa20086c, with `bdnz` loops at 0xfe048cc0 and 0xfe048d60. It calls 0xfe04475c at 0xfe048d84. The stores to 0xfa200860, 0xfa200870 and 0xfa20086c and the reads from 0xfa203c80 and 0xfa203c82 match sub_fe048c20 in 6.5.12, so this is probably the same routine in another build. The purpose is not settled.

- `sub_fe048dac` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048dac
  **Inferred.** A leaf routine of eleven stores with the same addresses as sub_fe048dfc in 6.5.12: halfwords to 0xfa200952, 0xfa200950, 0xfa200954, 0xfa200962, 0xfa200960, 0xfa200964, 0xfa200966 and 0xfa200ac2, and words to 0xfa200abc, 0xfa200ab8 and 0xfa200ac4, then a return of r3. The values stored are not in the facts, so the purpose is not settled.

- `sub_fe049048` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe049048
  **Inferred.** Sets up a 0x20-byte frame, writes a word to 0xfa200004 (0xfe04906c) and a byte to 0xfa100003 (0xfe049080), and calls 0xfe048dac with r9 0xfa100003 at 0xfe049084. It writes fields through r3 at offsets +0x0 to +0xc and +0x18, then calls 0xfe0486fc at 0xfe0490e8 and 0xfe049578 with r3 8 and r4 0 at 0xfe0490f4. Four calls to 0xfe04475c pass constant addresses 0x705870, 0x7058b4, 0x7058e4 and 0x705750 (0xfe049108 to 0xfe04915c), and 0xfe044f8c is called at 0xfe049134. Two calls to 0xfe048dfc pass r4 0xc0000000 and 0xc0001000 (0xfe049170, 0xfe049184). The stores and call pattern resemble sub_fe049098 in 6.5.12, but the callees are different addresses and none is named, so the purpose is partly unknown.

- `sub_c00a1834` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a1834
  **Inferred.** Sets up a 0x10-byte frame and stores a word to 0xfa200030 (0xc00a1848), then four words to 0xc015b4c4, 0xc015b4c8, 0xc015b4cc and 0xc015b4d0 (0xc00a1868 to 0xc00a1874) and one to 0xc015b4e4 (0xc00a1898). It calls 0xc000b678 at 0xc00a1878 with r4 set to r3 plus 0x1000, r5 0xfa200000, and r6, r8, r9 and r10 all 0xc0160000. After the call it reads through the returned pointer at 0xc00a187c and writes that pointer's word back at 0xc00a188c. The routine looks like a memory range setup, but the callee is not named, so this is inferred only.

- `sub_c00a18ac` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a18ac
  **Inferred.** Has the same shape as sub_c00c1ce4 in 6.5.12: it stores words to 0xfa200940 (0xc00a18d4, 0xc00a1924) and 0xfa200948 (0xc00a18d8), calls 0xc0003304 at 0xc00a18f0 with r3 5, r4 0xc00a193c and r6 0xc0105e4c, and branches on the result at 0xc00a18f8. The nonzero path calls 0xc000fd1c with r3 0xc0105e50 at 0xc00a1904. The routine ends with a call to 0xc00a19c8 at 0xc00a1918 with r4 0xc00a19c4 and returns its result. The callees are not named, so the purpose is partly unknown.
