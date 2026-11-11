# Routine role entries, sysco tier-1 batch 10

Role statements for targets 250 to 274 of the tier-1 list, written from the rendered instruction facts only (calls, constants, branch targets and stores). Every entry is inferred. None of the 25 targets was rejected by the grader.

- `sub_c00a563c` in `6.5.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** Sets up a frame and tests the pointer at 0xc0160508 and the words at its +0x4 and +0x8 (branches at 0xc00a5670, 0xc00a568c and 0xc00a56b8). It calls 0xc0010640 at 0xc00a567c with r3=0xc010a910, and makes an indirect call through the pointer in 0xfa200e80 at 0xc00a5698 (blrl). It stores to 0xfa200e80 and 0xfa200e84. The facts do not settle what the record at 0xc0160508 holds, and the return value is not derived.

- `sub_c00a56e0` in `6.5.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** Reads the pointers at 0xc0160504 and 0xc0160508 and updates the fields at offsets +0x0, +0x4 and +0x8 of the first (stores at 0xc00a5728 to 0xc00a5734). It calls 0xc00046d4 and 0xc0004710 at 0xc00a56fc and 0xc00a5700, then 0xc00046e0 at 0xc00a577c with r10 loaded from 0xc0160504. It writes 0xfa200e80 and 0xfa200e84 at 0xc00a5768 and 0xc00a576c. Its role is inferred from those stores and calls; the return value is not derived.

- `sub_c00a5eb0` in `6.5.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** A 568-byte routine that stores to the words at 0xfa200e40 to 0xfa200e4c and scans a halfword table at 0xc012eac8 and 0xc012eaca with a count-controlled loop (bdnz at 0xc00a5f58, 0xc00a6018, 0xc00a6048). It calls 0xc00046a0 at 0xc00a5ef0, 0xc00aa0a0 at 0xc00a5fec and 0xc0004c2c at 0xc00a6094, and it calls sub_c00a56e0 at 0xc00a60c0 with r3=0x600a0000 and r4=0xc00a5a60. It tests 0xc0160518 at 0xc00a6064. The table contents and the meaning of the scan are not established.

- `sub_c01344f0` in `6.5.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01344f0
  **Inferred.** Calls 0xc0010640 at 0xc0134548 with r3=0xc00f011c and r5=0x3c, which reads like a message or buffer copy. It reads 0xc0160c7c and 0xfa200280 and stores to 0xfa200380 at 0xc0134518 and 0xc0134520. It calls 0xc0003300 at 0xc01345e0 with r3=0xf and a string at 0xc000a584, and branches on the result (beq at 0xc01345e8) before calling 0xc0010094 at 0xc01345f4. The purpose of the 0xfa2003xx words and of the call at 0xc0003300 is not established.

- `sub_c013d7e4` in `6.5.24/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc013d7e4
  **Inferred.** A 1892-byte routine with a stmw frame that writes a block of about 50 words at 0xc0198ab0 to 0xc0198c58, and copies between those blocks with 0xc000c8d4 (sizes 0x2c and 0xc4). It calls 0xc006b73c at 0xc013d9e4 and 0xc013da04, and branches on the first result (beq at 0xc013d9f0). It makes two calls with sizes 0x20 and 0x80 (0xc00a25cc and 0xc00a2600 at 0xc013dc40 and 0xc013dc4c) and one with 0x90 and 0xf0 (0xc002778c at 0xc013dbb8). It writes halfword and byte tables at 0xc0198ac0 to 0xc0198ac8 and loops over them. The overall purpose is not established by the facts.

- `sub_ffe5fe58` in `6.5.25/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5fe58
  **Inferred.** A ColdFire routine that compares the words at offsets +0x0, +0x4, +0x8, +0xc, +0x10 and +0x14 of the structure at [sp+4] against constants (0x258, 0x0, 0x1e, 0xc, 0x1f, 0x18, 0x3c and 0x3d; branches at 0xffe5fe72 to 0xffe5fea6). If the checks pass it calls 0xffe5f8ac at 0xffe5feb4 and calls 0xffe5fa9c twice at 0xffe5fec4 and 0xffe5fee2. It updates the fields at +0x10, +0x14 and +0x18 of the 0xffe5fa9c result, and it reads and clears bits in the bytes at 0x200007f8 to 0x200007fc. The limits, the record layout and the return in d0 are not established.

- `sub_ffe5fc6c` in `6.5.25/usr/cpu/firmware/sysco/l1.bin(l1#1)` at 0xffe5fc6c
  **Inferred.** The same shape as the l1#0 routine at 0xffe5fe58 in this build. It applies the same limit checks to the structure at [sp+4] (branches at 0xffe5fc86 to 0xffe5fcba), then calls 0xffe5f6c0 at 0xffe5fcc8 and 0xffe5f8b0 at 0xffe5fcd8 and 0xffe5fcf6. It reads the constant word at 0x30000510 at 0xffe5fce8, where the l1#0 copy reads 0x300004dc. The callees and the record layout are not named by the facts.

- `sub_ffecfc6c` in `6.5.25/usr/cpu/firmware/sysco/l1.bin(l1#1)` at 0xffecfc6c
  **Inferred.** Another copy of the same 544-byte routine at the 0xffec bank address, with the same frame and limit checks. Its calls go to 0xffecf6c0 at 0xffecfcc8 and to 0xffecf8b0 at 0xffecfcd8 and 0xffecfcf6, and it returns at 0xffecfe8a. The facts do not say what the structure at [sp+4] is.

- `sub_fe004efc` in `6.5.25/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** Sets up a frame and calls 0xfe0051f0 at 0xfe004f24 with r3=0xe01868 and r9=0xfe040000. It calls 0xfe004610 five times with constant pointers 0xe013ec, 0xe01418, 0xe01428, 0xe01454 and 0xe01474, and calls 0xfe0046c8 with its result at 0xfe004f90. It calls 0xfe004ff4 and 0xfe004f98 with 0xfe040000 and branches on the result of 0xfe004ff4 (bne at 0xfe004f40). It stores to 0xfa200004 at 0xfe004f84. This reads as a start-up sequence of checks and messages, but the callee names are not known.

- `sub_fe0051a0` in `6.5.25/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** A leaf routine with no calls that makes eleven halfword and word stores into the window at 0xfa200950 to 0xfa200ac4 (sth at 0xfe0051a8, 0xfe0051b0 and 0xfe0051b4; stw at 0xfe0051bc, 0xfe0051c4 and 0xfe0051d0, and the rest of the sth stores). It returns r3 unchanged at 0xfe0051ec. It looks like a register setup step, but the facts do not say which device these addresses belong to.

- `sub_fe048ba0` in `6.5.25/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** Reads and rewrites the words at 0xfa200abc and 0xfa200ab8 and the halfword at 0xfa200ac2 (lwz at 0xfe048ba8 and 0xfe048bb8, lhz at 0xfe048bc8). It stores halfwords and bytes to 0xfa203c80 to 0xfa203cac and to 0xfa2009c0, then runs a loop of byte stores to computed addresses (bne at 0xfe048c18 back to 0xfe048c10). It writes 0x705cd8 at 0xfe048c44 and 0xfa200030 at 0xfe048c4c. The return value is not derived, and the purpose of the device window stores is not established.

- `sub_fe048c54` in `6.5.25/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** Calls sub_fe048ba0 at 0xfe048c90 with r9=0x700000. The bne at 0xfe048c8c skips that call when the word at 0x705cd8 is nonzero, so the call happens when that word is zero. It also calls 0xfe044770 at 0xfe048e08. In between it reads halfwords from 0xfa203c82 and 0xfa203c80, writes bytes to 0xfa200860, 0xfa200870 and 0xfa20086c, and runs loops (bdnz at 0xfe048d44 and 0xfe048de4) over halfword and byte accesses. It looks like a per-device setup pass, but the facts do not identify the device or the table.

- `sub_c000a5e0` in `6.5.25/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** A 68-byte routine that reads the word at 0xfa200284 (lwz at 0xc000a5f4), writes a word to an unknown address (0xc000a600) and a byte (0xc000a614), and tests a byte (lbz at 0xc000a60c). It calls 0xc0004710 at 0xc000a5ec and 0xc0010640 at 0xc000a61c with r3=0xc00f0164, which suggests a short status check that logs a message. The facts do not say what is checked.

- `sub_c00a2374` in `6.5.25/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** Stores 0xfa200030 at 0xc00a2388 and the words at 0xc01604ec, 0xc01604f0, 0xc01604f4 and 0xc01604f8 (0xc00a23a8 to 0xc00a23b4). It calls 0xc000b454 at 0xc00a23b8 with r4=r3+4096, r5=0xfa200000 and r6 to r10 set to 0xc0160000, then stores the result at 0xc01604e8 (0xc00a23d8). The shape looks like one-time setup of a mapped region at 0xfa200000, but the callee's name is unknown and the return value is not derived.

- `sub_c00a23ec` in `6.5.25/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** Stores words to 0xfa200940 and 0xfa200948 and reads them back (lwz at 0xc00a245c and 0xc00a2468). It calls 0xc0003300 at 0xc00a2430 with r3=5 and r4=0xc00a247c, which is the next routine, and branches on the result (beq at 0xc00a2438). It then calls 0xc0010094 at 0xc00a2444 and sub_c00a250c at 0xc00a2458, and returns the result of that call. Passing the address of 0xc00a247c as a callback to a numbered registration call would fit, but the facts do not confirm it.

- `sub_c00a247c` in `6.5.25/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** Reads the halfword at 0xfa200930 (lhz at 0xc00a2498) and loads through computed indices (lwzx at 0xc00a24ac and 0xc00a24c0, lwz at 0xc00a24d0 to 0xc00a24fc). It makes an indirect call at 0xc00a24c8 (blrl) whose target the facts do not resolve, and stores to 0xfa20094c at 0xc00a24f0. It reads like a poll or dispatch step, but its purpose is not established.

- `sub_c00a250c` in `6.5.25/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** Saves registers, then calls sub_c00a2594 at 0xc00a252c with no arguments set. When the branch at 0xc00a2528 is not taken it prints the string at 0xc010a1d4 through 0xc0010640 at 0xc00a2558. It reads the word at 0xfa200948 (0xc00a256c), which sub_c00a2594 also reads, and stores through computed indices (stwx at 0xc00a2560 and 0xc00a2564). The return value is not derived, and the meaning of the branch conditions is not established.

- `sub_c00a2594` in `6.5.25/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** A leaf routine with no calls. It reads the word at 0xfa200948 (lwz at 0xc00a25b4), stores through two computed indices (stwx at 0xc00a25a8 and 0xc00a25ac), and stores to one more address at 0xc00a25c4. It is the callee of the bl at 0xc00a252c in sub_c00a250c, and it looks like a helper that updates the same record that sub_c00a250c reads. The facts do not name that record.

- `sub_c00a4e98` in `6.5.25/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** A 328-byte routine with an stmw frame that loads ten words and eight halfwords from computed addresses and stores halfwords and words. It calls 0xc0010640 at 0xc00a4ecc with r3=0xc010a7dc, calls 0xc0004868 at 0xc00a4f38, and calls 0xc00046d4 and 0xc0004710 at 0xc00a4f40 and 0xc00a4f44 with the result passed as r3. It reads 0xc0160178 at 0xc00a4f58 and stores to 0xfa200e54 at 0xc00a4f6c. Several flag branches (beq at 0xc00a4ec0, 0xc00a4f78 and 0xc00a4f98) gate its calls, but the facts do not give its purpose.

- `sub_c00a515c` in `6.5.25/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** A 688-byte routine that reads the word at 0xfa200e44 (lwz at 0xc00a5174) and branches to 0xc00a53f8 when it is zero (beq at 0xc00a5188). Its body is a long chain of flag-tested branches with calls to 0xc0010640 (strings at 0xc010a894 and 0xc010a8a4), 0xc0013dec with arguments 0 and 1 (0xc00a52e0 and 0xc00a53a0), 0xc0013764 at 0xc00a53b0, and 0xc00a540c at 0xc00a51c4. It ends by calling 0xc00a563c at 0xc00a53e8. The role of the routine is not established by the facts.

- `sub_c00a563c` in `6.5.25/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** Same 164-byte range and call sites as the 6.5.24 copy of this routine at the same address: a call to 0xc0010640 at 0xc00a567c with r3=0xc010a910, and an indirect call through the pointer in 0xfa200e80 at 0xc00a5698. It stores to 0xfa200e80 and 0xfa200e84 and reads the record at 0xc0160508. The facts do not settle the record's meaning or the return value.

- `sub_c00a56e0` in `6.5.25/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** Same 184-byte range and call sites as the 6.5.24 copy at this address. It calls 0xc00046d4 and 0xc0004710 at 0xc00a56fc and 0xc00a5700, then 0xc00046e0 at 0xc00a577c with r10 loaded from 0xc0160504. It updates the pointer and fields at 0xc0160504 and 0xc0160508 and writes 0xfa200e80 and 0xfa200e84. The purpose is inferred from those stores and calls; the return value is not derived.

- `sub_c00a5eb0` in `6.5.25/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** Same 568-byte range and call sites as the 6.5.24 copy at this address. It stores to 0xfa200e40 to 0xfa200e4c, scans the halfword table at 0xc012eac8 and 0xc012eaca with a count-controlled loop, and calls sub_c00a56e0 at 0xc00a60c0 with r3=0x600a0000. It also calls 0xc00046a0, 0xc00aa0a0 and 0xc0004c2c. The table contents are not established.

- `sub_c01344f0` in `6.5.25/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01344f0
  **Inferred.** Same 284-byte range and call sites as the 6.5.24 copy at this address. It calls 0xc0010640 at 0xc0134548 with r3=0xc00f011c and r5=0x3c, calls 0xc0003300 at 0xc01345e0 with r3=0xf, and calls 0xc0010094 at 0xc01345f4. It stores to 0xfa200380 and reads 0xc0160c7c. The purpose of the 0xfa2003xx words is not established.

- `sub_c013d7e4` in `6.5.25/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc013d7e4
  **Inferred.** Same 1892-byte range and call sites as the 6.5.24 copy at this address. It writes the block at 0xc0198ab0 to 0xc0198c58 through calls to 0xc000c8d4, calls 0xc006b73c twice (beq at 0xc013d9f0 tests the first result), and calls 0xc00a25cc and 0xc00a2600 with sizes 0x20 and 0x80. It writes halfword and byte tables at 0xc0198ac0 to 0xc0198ac8. The overall purpose is not established by the facts.
