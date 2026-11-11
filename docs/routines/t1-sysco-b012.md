# Tier-1 routine readings, sysco batch 12

Role statements for targets 300 to 324 of the tier-1 list (hardware-only, located or better), written from the
rendered instruction facts only: registers, stack frames, memory accesses, callees, constants and branch targets.
Each role is a reading of those facts, not a confirmed contract. All 25 grades are inferred, and the grader
accepted every entry, so none were rejected. Several routines appear in both the 6.5.27 and 6.5.28 builds with
the same address; their entries name each build, and where the constants differ the entry says so.

- `sub_c00a250c` in `6.5.27/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** Builds a 0x20-byte frame (`stwu` at 0xc00a250c) and saves registers with `stmw` at 0xc00a2514. It calls the routine at 0xc00a2594 (`bl` at 0xc00a252c) and 0xc0010640 with r3=0xc010a1d4 (`bl` at 0xc00a2558), and it reads the word at abs 0xfa200948 (`lwz` at 0xc00a256c). Two conditional branches at 0xc00a2528 and 0xc00a2548 gate the calls. The rendered facts do not settle what the branches test, so the purpose is unknown.

- `sub_c00a2594` in `6.5.27/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** A 56-byte routine that stores two words with indexed stores (`stwx` at 0xc00a25a8 and 0xc00a25ac), reads the word at abs 0xfa200948 (`lwz` at 0xc00a25b4), and stores once at 0xc00a25c4 before returning at 0xc00a25c8. Its caller in this batch is the routine at 0xc00a250c (`bl` at 0xc00a252c). What the stores write is not derived from the facts.

- `sub_c00a4e98` in `6.5.27/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** A 328-byte routine with a 0x30-byte frame (`stwu` at 0xc00a4e98) and mixed 2- and 4-byte accesses: eight `lhz`, four `sth`, ten `lwz` and seven `stw` through computed addresses. It reads abs 0xc0160178 (`lwz` at 0xc00a4f58) and writes abs 0xfa200e54 (`stw` at 0xc00a4f6c). It calls 0xc0010640 with r3=0xc010a7dc (`bl` at 0xc00a4ecc), 0xc0004868 (`bl` at 0xc00a4f38), 0xc00046d4 followed by 0xc0004710 (`bl` at 0xc00a4f40 and 0xc00a4f44), and 0xc00046e0 (`bl` at 0xc00a4fc0). The data structure it fills is not identified.

- `sub_c00a515c` in `6.5.27/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** A 688-byte routine with a 0x40-byte frame (`stwu` at 0xc00a515c) that reads the status word at abs 0xfa200e44 (`lwz` at 0xc00a5174) and branches on it at 0xc00a5188. Its body is a long run of `beq` and `bne` tests on the Z flag (from 0xc00a51a4 to 0xc00a53f4), which suggests a dispatch over several fields. It calls 0xc0010640 with r3=0xc010a894 and 0xc010a8a4 (`bl` at 0xc00a51b4 and 0xc00a527c), the routine at 0xc00a540c (`bl` at 0xc00a51c4), 0xc0013dec (`bl` at 0xc00a52e0 and 0xc00a53a0), and the routine at 0xc00a563c (`bl` at 0xc00a53e8). The dispatch meaning is a guess.

- `sub_c00a563c` in `6.5.27/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** A 164-byte routine that reads the pointers at abs 0xc0160508 and 0xc0160504 (`lwz` at 0xc00a5650 and 0xc00a56a4) and follows the record at [0xc0160508] through offsets +0x4 and +0x8. It writes both pointer cells back (`stw` at 0xc00a56a8 and 0xc00a56b4) and stores abs 0xfa200e80 (`stw` at 0xc00a56c4). It calls 0xc0010640 with r3=0xc010a910 (`bl` at 0xc00a567c) and makes an indirect call at 0xc00a5698. It looks like a walk or update of a small linked record at 0xc0160504 and 0xc0160508, but the record layout is not settled.

- `sub_c00a56e0` in `6.5.27/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** A 184-byte routine with a 0x30-byte frame (`stwu` at 0xc00a56e0) that reads the same cells as the previous routine, abs 0xc0160504 and 0xc0160508 (`lwz` at 0xc00a5708 and 0xc00a571c), and writes the record at [0xc0160504] at offsets +0x0, +0x4 and +0x8. It writes abs 0xc016050c and 0xc0160508 (`stw` at 0xc00a5758 and 0xc00a575c), and stores abs 0xfa200e84 and 0xfa200e80 (`stw` at 0xc00a5768 and 0xc00a576c). It calls 0xc00046d4 and 0xc0004710 (`bl` at 0xc00a56fc and 0xc00a5700) and 0xc00046e0 (`bl` at 0xc00a577c). It seems to be a companion update of the record at 0xc0160504, not settled by the facts.

- `sub_c00a5eb0` in `6.5.27/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** A 568-byte routine with a 0x20-byte frame (`stwu` at 0xc00a5eb0) that reads abs 0xc0131278 (`lwz` at 0xc00a5ec8) and stores four words to abs 0xfa200e40, 0xfa200e48, 0xfa200e44 and 0xfa200e4c (`stw` at 0xc00a5eec to 0xc00a5f0c). It reads 16-bit values from abs 0xc012eac8 and 0xc012eaca (`lhz` at 0xc00a5f14 and 0xc00a5f1c), stores ten halfwords, and walks with `bdnz` loops and `lhzu` (0xc00a601c, 0xc00a604c). It calls 0xc00046a0, 0xc00aa0a0, 0xc0004c2c (with r10=8) and the routine at 0xc00a56e0 (`bl` at 0xc00a60c0). It looks like a table or buffer fill that moves 16-bit fields, but the table contents are not identified.

- `sub_c01344f0` in `6.5.27/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01344f0
  **Inferred.** A 284-byte routine with a 0x20-byte frame (`stwu` at 0xc01344f0) that reads abs 0xc0160c7c (`lwz` at 0xc013450c), writes abs 0xfa200380 at 0xc0134518 and 0xc0134520, and reads abs 0xfa200280 (`lwz` at 0xc0134524). It calls 0xc0010640 with r3=0xc00f011c and r5=0x3c (`bl` at 0xc0134548), 0xc0004630 (`bl` at 0xc0134580), and 0xc0003300 with r3=0xf (`bl` at 0xc01345e0). The `beq` at 0xc01345e8 jumps to 0xc01345f8 when the result of 0xc0003300 is zero, and 0xc0010094 is called with r3=0xc00f0144 (`bl` at 0xc01345f4). It sets up a device window in the 0xfa2002xx and 0xfa2003xx range, but the device is not named in the facts.

- `sub_c013d7e4` in `6.5.27/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc013d7e4
  **Inferred.** The largest routine in this batch, 1892 bytes with a 0x60-byte frame (`stwu` at 0xc013d7e4). It fills a block at 0xc0198ab0 with `stw` at 0xc013d840, 0xc013d868 and 0xc013d86c and halfword stores at 0xc013d870 to 0xc013d880, and a second block at 0xc0198b18 to 0xc0198b20 (`stw` at 0xc013d94c to 0xc013d954). It reads abs 0xc0131688, 0xc0131684 and 0xc0160500 (`lwz` at 0xc013d80c, 0xc013d810 and 0xc013d890). It calls 0xc000cbe4 with r3=0xc0198ab0 and r4=0xc4 (`bl` at 0xc013d82c), 0xc000c8d4 four times with pointers into that block (`bl` at 0xc013d888 to 0xc013db28), and the helpers at 0xc00a25cc, 0xc00a2600, 0xc00a2630 and 0xc00a250c (`bl` at 0xc013dc40 to 0xc013df1c). It is probably a kernel-data initializer, but the block's purpose is not identified.

- `sub_ffe5e850` in `6.5.28/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5e850
  **Inferred.** A 544-byte routine built on an m68k-style frame: `link.w` at 0xffe5e850 and six `movem.l` saves at 0xffe5e858. It loads a value from [sp+4]+0x14 (`move.l` at 0xffe5e860) and compares six fields of the block at [sp+4], offsets 0x0 to 0x14 (`cmp.l` at 0xffe5e874 to 0xffe5e89c). It calls 0xffe60340 with arg0=[sp+4] and arg1 to arg5 from d3 to d7 (`jsr` at 0xffe5e8ac), and 0xffe60530 twice (`jsr` at 0xffe5e8bc and 0xffe5e8da). It returns d0 at 0xffe5ea6e. The shape looks like a validation of a six-field request before dispatch, but the record and the callees are not identified.

- `sub_ffe73c74` in `6.5.28/usr/cpu/firmware/sysco/l1.bin(l1#3)` at 0xffe73c74
  **Inferred.** Same shape as the routine at 0xffe5e850 in the l1#0 image: a 544-byte m68k-style routine with `link.w` at 0xffe73c74, six `movem.l` saves at 0xffe73c7c, and `cmp.l` tests of [sp+4] offsets 0x0 to 0x14 (0xffe73c98 to 0xffe73cc0). It calls 0xffe776a4 with arg0=[sp+4] and arg1 to arg5 from d3 to d7 (`jsr` at 0xffe73cd0), and 0xffe77894 twice (`jsr` at 0xffe73ce0 and 0xffe73cfe). It returns d0 at 0xffe73e92. It is a second copy of the same check and dispatch in a different segment of the firmware, and the callee addresses differ.

- `sub_fe004efc` in `6.5.28/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** A 156-byte routine with a 0x20-byte frame (`stwu` at 0xfe004efc) that saves four words to the stack (`stw` at 0xfe004f04 to 0xfe004f10) and stores abs 0xfa200004 (`stw` at 0xfe004f84). It calls 0xfe0051f0 with r3=0xe01868 and r9=0xfe040000 (`bl` at 0xfe004f24), 0xfe004610 five times with r3 values 0xe013ec, 0xe01418, 0xe01428, 0xe01454 and 0xe01474 (`bl` at 0xfe004f30 to 0xfe004f8c), 0xfe004ff4 and 0xfe004f98, and 0xfe0046c8 (`bl` at 0xfe004f90). The `bne` at 0xfe004f40 tests the result of 0xfe004ff4. The pattern of several calls to one routine with different pointers suggests a startup message sequence, which the facts do not confirm.

- `sub_fe0051a0` in `6.5.28/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** An 80-byte straight-line routine with no branches and no callees. It writes halfwords to abs 0xfa200950, 0xfa200952, 0xfa200954, 0xfa200960, 0xfa200962, 0xfa200964 and 0xfa200966 (`sth` at 0xfe0051a8 to 0xfe0051e8), words to 0xfa200ab8, 0xfa200abc and 0xfa200ac4 (`stw` at 0xfe0051bc, 0xfe0051c4 and 0xfe0051d0), and one halfword to 0xfa200ac2 (`sth` at 0xfe0051c8). It returns r3 unchanged at 0xfe0051ec. It sets a fixed set of fields in the 0xfa2009xx to 0xfa200axx window, but the values written are not in the facts.

- `sub_fe048ba0` in `6.5.28/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** A 180-byte leaf routine with no calls. It reads and writes back the same words at abs 0xfa200abc and 0xfa200ab8 (`lwz` at 0xfe048ba8 and 0xfe048bb8, `stw` at 0xfe048bb4 and 0xfe048bc4), and the halfword at 0xfa200ac2 (`lhz` at 0xfe048bc8, `sth` at 0xfe048bd4). It writes halfwords and bytes to abs 0xfa203c80 to 0xfa203cac and 0xfa2009c0 (`sth` at 0xfe048bd8 to 0xfe048c08), and stores a word to abs 0x705cd8 and 0xfa200030 (`stw` at 0xfe048c44 and 0xfe048c4c). It is a read-modify-write helper for the 0xfa200ab8 and 0xfa203c8x fields, and its caller is the routine at 0xfe048c54 (`bl` at 0xfe048c90).

- `sub_fe048c54` in `6.5.28/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** A 476-byte routine with a 0x20-byte frame (`stwu` at 0xfe048c54) that reads abs 0x705cd8 (`lwz` at 0xfe048c74) and halfwords at abs 0xfa203c82 and 0xfa203c80 (`lhz` at 0xfe048c9c and 0xfe048cb8). It stores bytes to abs 0xfa200860, 0xfa200870 and 0xfa20086c (`stb` at 0xfe048cf0 to 0xfe048d00) and makes many stores through computed addresses. It calls the helper at 0xfe048ba0 with r9=0x700000 (`bl` at 0xfe048c90) and 0xfe044770 (`bl` at 0xfe048e08). It appears to fill status fields in the 0xfa2008xx and 0xfa203cxx windows after the helper runs, which the facts do not confirm.

- `sub_c000a5e0` in `6.5.28/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** A 68-byte wrapper with a 0x20-byte frame (`stwu` at 0xc000a5e0). It reads the word at abs 0xfa200284 (`lwz` at 0xc000a5f4), and reads and writes one byte at computed addresses (`lbz` at 0xc000a60c, `stb` at 0xc000a614). It calls 0xc0004710 (`bl` at 0xc000a5ec) and 0xc0010640 with r3=0xc00f0164 (`bl` at 0xc000a61c). It is a short status read followed by a report through 0xc0010640, but the byte and the message are not identified.

- `sub_c00a2374` in `6.5.28/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** A 120-byte routine with a 0x10-byte frame (`stwu` at 0xc00a2374). It stores abs 0xfa200030 (`stw` at 0xc00a2388) and four pointer cells at abs 0xc01604ec, 0xc01604f0, 0xc01604f4 and 0xc01604f8 (`stw` at 0xc00a23a8 to 0xc00a23b4), and it stores abs 0xc01604e8 (`stw` at 0xc00a23d8). It calls 0xc000b454 with r4=r3+4096, r5=0xfa200000, r6=0xc0160000 and r7=1 (`bl` at 0xc00a23b8). The pattern looks like setup of a region or buffer descriptor passed to 0xc000b454, but the descriptor layout is not settled.

- `sub_c00a23ec` in `6.5.28/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** A 144-byte routine with a 0x10-byte frame (`stwu` at 0xc00a23ec) that stores two words to abs 0xfa200940 and one to abs 0xfa200948 (`stw` at 0xc00a2414, 0xc00a2464 and 0xc00a2418), and reads abs 0xfa200940 at 0xc00a245c. It calls 0xc0003300 with r3=5 and r4=0xc00a247c (`bl` at 0xc00a2430), tests the result at 0xc00a2438, calls 0xc0010094 with r3=0xc010a238 (`bl` at 0xc00a2444), and calls 0xc00a250c (`bl` at 0xc00a2458). It records state in the 0xfa200940 window and then runs the routine at 0xc00a250c, but the intent of the call through 0xc0003300 is not settled.

- `sub_c00a247c` in `6.5.28/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** A 144-byte routine with a 0x10-byte frame (`stwu` at 0xc00a247c). It stores a halfword to abs 0xfa200930 and reads it back (`sth` at 0xc00a2494, `lhz` at 0xc00a2498), uses two indexed loads (`lwzx` at 0xc00a24ac and 0xc00a24c0), and stores abs 0xfa20094c (`stw` at 0xc00a24f0). It makes an indirect call at 0xc00a24c8 (`blrl`) whose target the facts do not resolve, guarded by the `beq` at 0xc00a24b8. The purpose of the status write and the indirect call is unknown.

- `sub_c00a250c` in `6.5.28/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** The same routine as the 6.5.27 build at this address, with the same frame, `stmw`, branches, call to 0xc00a2594 (`bl` at 0xc00a252c) and read of abs 0xfa200948 (`lwz` at 0xc00a256c). The only difference the facts show is the r3 constant of the 0xc0010640 call at 0xc00a2558, which is 0xc010a254 here and 0xc010a1d4 in the 6.5.27 build. Its role is unsettled, as in the 6.5.27 entry.

- `sub_c00a2594` in `6.5.28/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** The same 56-byte routine as the 6.5.27 build: two indexed stores (`stwx` at 0xc00a25a8 and 0xc00a25ac), a read of abs 0xfa200948 (`lwz` at 0xc00a25b4), one store at 0xc00a25c4, and a return at 0xc00a25c8. What it writes is not derived from the facts.

- `sub_c00a4e98` in `6.5.28/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** The same 328-byte routine as the 6.5.27 build, with the same frame, mixed 2- and 4-byte accesses, reads of abs 0xc0160178 and writes of abs 0xfa200e54, and the same callees 0xc0004868, 0xc00046d4, 0xc0004710 and 0xc00046e0. The one constant that differs is the r3 of the 0xc0010640 call at 0xc00a4ecc, which is 0xc010a85c here and 0xc010a7dc in the 6.5.27 build. Its data structure is not identified.

- `sub_c00a515c` in `6.5.28/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** The same 688-byte dispatch-shaped routine as the 6.5.27 build, with the same status read from abs 0xfa200e44 at 0xc00a5174, the same chain of Z-flag tests, and calls to 0xc00a540c, 0xc0013dec, 0xc0013764 and 0xc00a563c. The 0xc0010640 calls use r3=0xc010a914 and 0xc010a924 here, against 0xc010a894 and 0xc010a8a4 in the 6.5.27 build. The dispatch meaning is a guess.

- `sub_c00a563c` in `6.5.28/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** The same 164-byte record walk as the 6.5.27 build, reading and writing the cells at 0xc0160504 and 0xc0160508 and storing abs 0xfa200e80, with an indirect call at 0xc00a5698. The 0xc0010640 call at 0xc00a567c uses r3=0xc010a990 here, against 0xc010a910 in the 6.5.27 build. The record layout is not settled by the facts.

- `sub_c00a56e0` in `6.5.28/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** The same 184-byte companion update of the record at 0xc0160504 as the 6.5.27 build, with the same reads and writes of abs 0xc0160504, 0xc0160508 and 0xc016050c, the same stores to abs 0xfa200e84 and 0xfa200e80, and the same callees 0xc00046d4, 0xc0004710 and 0xc00046e0. Its role is not settled by the facts.
