# ARM L1 I²C, fan and power routines

The findings are specific to each carrier, package and extracted ELF below.
ARM AAPCS is an assumed calling convention.
Argument count and custom ABI details are unknown.
The four register slots at each call are candidate locations.
`unknown` preserves an unresolved value.
Value roles, pointee meanings, C types, runtime path feasibility and equality of repeated loads remain unknown.
Symbol names do not establish a hardware role.
Effects are not transferred between builds.

In unreached-range statements, "any path from the entry" means any path in the recovered known-edge control-flow graph from that entry.
These statements do not prove runtime path feasibility.
Range ends are exclusive.
Raw undecodable ranges are separate from decoded instructions.
The [complete facts](disasm/arm-variants/i2c-fan-power-1.62.1-1.48.1-facts.txt) retain the full analysis for every routine.

Image `cd-ist-3.24:l1#4`, version 1.62.1: `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, inner `/stand/sysco/bin/l1`.
Package begins at carrier offset 0x7ecf04 and is 4948212 bytes.
CramFS begins at package offset 0x1012fc.
ELF size is 1999676 bytes, SHA-256 `22cb62871e447a394acbd662f4f49f3146580afec93839953b37a5354545bfc5`.
Its executable PT_LOAD begins at ELF file offset 0x0, virtual address 0x8000, size 1693992 bytes, SHA-256 `5ad8f4be9cbda073b307873683a90846744df308e9629d628f27b0b87af1e73c`.
Routine source offsets are entry address minus 0x8000 in this segment.
Carrier SHA-256 is `b4b5634949a93f05ad6eb24462e6cd7f10104dd42ff5c2ae8616dd5d87697790`.
Package SHA-256 is `8c964136f81ec663d511b553041f07e798d9a30adebdd7b85fe420332e480f89`.

Image `cd-ist-3.24:l1#2`, version 1.62.1: `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, inner `/stand/sysco/bin/l1`.
Package begins at carrier offset 0x298558 and is 3817724 bytes.
CramFS begins at package offset 0xbb728.
ELF size is 1368336 bytes, SHA-256 `45ff1a458f755d99954be0d8f953933675796e4580a565a5c3360bb9ae4d1eed`.
Its executable PT_LOAD begins at ELF file offset 0x0, virtual address 0x8000, size 1169736 bytes, SHA-256 `51811253107f27d6a290f8c4579834de686b6af9c8d0849a9af9949e77d3d347`.
Routine source offsets are entry address minus 0x8000 in this segment.
Carrier SHA-256 is `b4b5634949a93f05ad6eb24462e6cd7f10104dd42ff5c2ae8616dd5d87697790`.
Package SHA-256 is `c6f5f194b4ed6da5938c75ee40fab49e4ef6998428e6a687c50d12452d4d06ce`.

Image `patches/SG0007149:l1#6`, version 1.48.1: `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`, inner `/stand/sysco/bin/l1`.
Package begins at carrier offset 0xa90dfc and is 4948212 bytes.
CramFS begins at package offset 0x101304.
ELF size is 1976140 bytes, SHA-256 `61274890f8ba1708b1380a8716e90060d48c22b2505eebf27255744123cdf96e`.
Its executable PT_LOAD begins at ELF file offset 0x0, virtual address 0x8000, size 1674816 bytes, SHA-256 `081529c2c1a72bbb7e930f5bd6c11cf0bc6b23feb32dff48fcf2335845126629`.
Routine source offsets are entry address minus 0x8000 in this segment.
Carrier SHA-256 is `536bfa4c6d0240b7afbc25bef94e160bdf1a675016c35e68ec52e1bfb1b451f9`.
Package SHA-256 is `333891e54763faf27b63031875afc189dcf6adda6e6999c6ad87f074a297e6a5`.

- `i2cInit`, `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, 0x11617c.
  Version 1.62.1, package `cd-ist-3.24:l1#4` (Linux L1 image), inner `/stand/sysco/bin/l1`; occupies 472 bytes.
  There are 16 word loads from PC-relative literal locations, including `ldr` at 0x116190 reads absolute 0x11632c; `ldr` at 0x1161a0 reads absolute 0x116330; `ldr` at 0x1161ac reads absolute 0x116334.
  2 such word loads access absolute 0x1b0240, including `ldr` at 0x116194, `ldr` at 0x1162b0.
  3 such word loads access ret(0x20904) +0x0 (from call at 0x1161dc, call at 0x11624c, call at 0x1162bc, respectively), including `ldr` at 0x1161f0, `ldr` at 0x116260, `ldr` at 0x1162c4.
  2 such word stores access absolute 0x3330f4, including `str` at 0x11623c, `str` at 0x116294.
  2 such word loads access absolute 0x3330f4, including `ldr` at 0x116240, `ldr` at 0x11628c.
  A word store accesses absolute 0x1b0240 (`str` at 0x1162ac).
  `bl` at 0x1161b8 calls (unknown callee) [0x22998] with r0=0x9, r1=0x19c118, r2=0x59, r3=0x0.
  `bl` at 0x1161c4 calls (unknown callee) [0x20c10] with r0=0x19c120, r1=0x1002, r2=unknown, r3=unknown.
  `bl` at 0x1161dc calls (unknown callee) [0x20904] with r0=ret(0x20c10), r1=unknown, r2=unknown, r3=ret(0x20c10).
  `bl` at 0x116208 calls (unknown callee) [0x22998] with r0=0x9, r1=0x19c118, r2=0x5f, r3=0x0.
  `bl` at 0x116210 calls (unknown callee) [0x20c58] with r0=0x1, r1=unknown, r2=unknown, r3=unknown.
  `bl` at 0x116238 calls (unknown callee) [0x208ec] with r0=0x0, r1=0x100, r2=0x3, r3=0x1.
  `bl` at 0x11624c calls (unknown callee) [0x20904] with r0=ret(0x208ec), r1=unknown, r2=unknown, r3=[0x3330f4].
  `bl` at 0x116278 calls (unknown callee) [0x22998] with r0=0x9, r1=0x19c118, r2=0x67, r3=0x0.
  `bl` at 0x116280 calls (unknown callee) [0x20c58] with r0=0x1, r1=unknown, r2=unknown, r3=unknown.
  `bl` at 0x1162a0 calls (unknown callee) [0x20c10] with r0=0x19c178, r1=0x2, r2=0x3330f4, r3=[0x3330f4]+36.
  `bl` at 0x1162bc calls (unknown callee) [0x20904] with r0=ret(0x20c10), r1=unknown, r2=ret(0x20c10), r3=[0x1b0240].
  `bl` at 0x1162c8 calls (unknown callee) [0x222cc] with r0=[ret(0x20904)], r1=unknown, r2=unknown, r3=ret(0x20904).
  `bl` at 0x1162ec calls (unknown callee) [0x22998] with r0=0x9, r1=0x19c118, r2=0x71, r3=0x0.
  `bl` at 0x11630c calls (unknown callee) [0x130988] with r0=unknown, r1=unknown, r2=unknown, r3=unknown.
  `bl` at 0x116310 calls (unknown callee) [0x1169b4] with r0=ret(0x130988), r1=unknown, r2=unknown, r3=unknown.
  `blt` at 0x11619c is taken when [0x1b0240] < 0x0 (signed, 32-bit) to 0x1161bc; it falls through when [0x1b0240] >= 0x0 (signed, 32-bit) to 0x1161a0.
  For the call at 0x1161c4, `bge` at 0x1161d8 is taken when ret(0x20c10) >= 0x0 (signed, 32-bit) to 0x116214; it falls through when ret(0x20c10) < 0x0 (signed, 32-bit) to 0x1161dc.
  `bne` at 0x116248 is taken when Z == 0 to 0x116284; it falls through when not (Z == 0) to 0x11624c.
  `bge` at 0x1162b8 is taken when [0x1b0240] >= 0x0 (signed, 32-bit) to 0x1162f0; it falls through when [0x1b0240] < 0x0 (signed, 32-bit) to 0x1162bc.
  `bhi` at 0x116300 is taken when hi flags condition (operands unknown) to 0x116324; it falls through when not (hi flags condition (operands unknown)) to 0x116304.
  The return (`ldm` at 0x116328) has r0 = unknown.
  The instructions from 0x11632c to 0x116354 are not reached by any path from the entry, and their branches are not listed.
  Unsupported-effect warnings: 0x116244 cmn: unsupported effects invalidated.

- `i2cRead`, `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, 0x116aec.
  Version 1.62.1, package `cd-ist-3.24:l1#4` (Linux L1 image), inner `/stand/sysco/bin/l1`; occupies 80 bytes.
  `bl` at 0x116b28 calls (unknown callee) [0x116df0] with r0=0x1, r1=(r0 & 0xff), r2=unknown, r3=[sp-24].
  The return (`ldm` at 0x116b38) has r0 = ret(0x116df0) from the call at 0x116b28; `mov` at 0x116b2c copies r0 to r3 and `mov` at 0x116b30 copies r3 to r0 without an intervening call.
  Unsupported-effect warnings: 0x116b20 mvn: unsupported effects invalidated.

- `i2cWrite`, `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, 0x116a3c.
  Version 1.62.1, package `cd-ist-3.24:l1#4` (Linux L1 image), inner `/stand/sysco/bin/l1`; occupies 80 bytes.
  `bl` at 0x116a78 calls (unknown callee) [0x116df0] with r0=0x0, r1=(r0 & 0xff), r2=unknown, r3=[sp-24].
  The return (`ldm` at 0x116a88) has r0 = ret(0x116df0) from the call at 0x116a78; `mov` at 0x116a7c copies r0 to r3 and `mov` at 0x116a80 copies r3 to r0 without an intervening call.
  Unsupported-effect warnings: 0x116a70 mvn: unsupported effects invalidated.

- `fanSetSpeed`, `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, 0x6b390.
  Version 1.62.1, package `cd-ist-3.24:l1#4` (Linux L1 image), inner `/stand/sysco/bin/l1`; occupies 796 bytes.
  There are 9 word loads from PC-relative literal locations, including `ldr` at 0x6b3b4 reads absolute 0x6b690; `ldr` at 0x6b3dc reads absolute 0x6b694; `ldr` at 0x6b3f4 reads absolute 0x6b698.
  A word load accesses absolute 0x1b5d10 (`ldr` at 0x6b3bc).
  2 such word loads access absolute 0x1b5d0c, including `ldr` at 0x6b3e0, `ldr` at 0x6b418.
  A byte load accesses an unknown effective address (`ldrb` at 0x6b3e8).
  2 such byte loads access [sp-44] +0x4, including `ldrb` at 0x6b428, `ldrb` at 0x6b4b8.
  A word load accesses [sp-44] +0x8 (`ldr` at 0x6b548).
  A byte load accesses an unknown effective address (`ldrb` at 0x6b550).
  A byte load accesses ret(0x6ba84) +0x0 (from call at 0x6b56c) (`ldrb` at 0x6b574).
  A byte load accesses ret(0x6bbb4) +0x3 (from call at 0x6b580) (`ldrb` at 0x6b590).
  A byte load accesses ret(0x6bbb4) +0x7 (from call at 0x6b580) (`ldrb` at 0x6b5fc).
  A byte load accesses ret(0x6bbb4) +0x8 (from call at 0x6b580) (`ldrb` at 0x6b60c).
  A byte load accesses ret(0x6bbb4) +0x9 (from call at 0x6b580) (`ldrb` at 0x6b61c).
  `bl` at 0x6b444 calls (unknown callee) [0x32d64] with r0=r0, r1=0x1, r2=0x0, r3=0x0.
  `bl` at 0x6b450 calls (unknown callee) [0x20ac0] with r0=ret(0x32d64), r1=unknown, r2=unknown, r3=unknown.
  `bl` at 0x6b498 calls (unknown callee) [0x20b98] with r0=[sp-56], r1=[sp-48], r2=[sp-52], r3=unknown.
  `bl` at 0x6b4d0 calls (unknown callee) [0x22998] with r0=0x9, r1=0x170af4, r2=0x2bb, r3=0x0.
  `bl` at 0x6b504 calls (unknown callee) [0x6bcd8] with r0=unknown, r1=unknown, r2=sp-30, r3=sp-31.
  `bl` at 0x6b56c calls (unknown callee) [0x6ba84] with r0=[sp-20], r1=unknown, r2=unknown, r3=unknown.
  `bl` at 0x6b580 calls (unknown callee) [0x6bbb4] with r0=[sp-20], r1=[ret(0x6ba84)]:1, r2=unknown, r3=[ret(0x6ba84)]:1.
  `bl` at 0x6b640 calls (unknown callee) [0x6aff0] with r0=[sp-20], r1=[sp-32]:1, r2=unknown, r3=[sp-32]:1.
  `bge` at 0x6b3c4 is taken when r0 >= [0x1b5d10] (signed, 32-bit) to 0x6b3f4; it falls through when r0 < [0x1b5d10] (signed, 32-bit) to 0x6b3c8.
  `bne` at 0x6b3f0 is taken when unknown != 0x0 to 0x6b400; it falls through when unknown == 0x0 to 0x6b3f4.
  `bne` at 0x6b430 is taken when [[sp-44]+4]:1 != 0x0 to 0x6b4d4; it falls through when [[sp-44]+4]:1 == 0x0 to 0x6b434.
  `bne` at 0x6b4dc is taken when [sp-44] != 0x0 to 0x6b4ec; it falls through when [sp-44] == 0x0 to 0x6b4e0.
  For the call at 0x6b504, `beq` at 0x6b518 is taken when ret(0x6bcd8) == 0x0 to 0x6b528; it falls through when ret(0x6bcd8) != 0x0 to 0x6b51c.
  `bhs` at 0x6b53c is taken when C == 1 to 0x6b67c; it falls through when not (C == 1) to 0x6b540.
  `bne` at 0x6b558 is taken when Z == 0 to 0x6b560; it falls through when not (Z == 0) to 0x6b55c.
  For the call at 0x6b580, `bne` at 0x6b598 is taken when [ret(0x6bbb4)+3]:1 != 0xff to 0x6b5b8; it falls through when [ret(0x6bbb4)+3]:1 == 0xff to 0x6b59c.
  `bne` at 0x6b5a4 is taken when [sp-21]:1 != 0xff to 0x6b5ac; it falls through when [sp-21]:1 == 0xff to 0x6b5a8.
  `beq` at 0x6b5c8 is taken when [sp-22]:1 == 0x1 to 0x6b608; it falls through when [sp-22]:1 != 0x1 to 0x6b5cc.
  `bgt` at 0x6b5d4 is taken when [sp-22]:1 > 0x1 (signed, 32-bit) to 0x6b5e8; it falls through when [sp-22]:1 <= 0x1 (signed, 32-bit) to 0x6b5d8.
  `beq` at 0x6b5e0 is taken when [sp-22]:1 == 0x0 to 0x6b5f8; it falls through when [sp-22]:1 != 0x0 to 0x6b5e4.
  `beq` at 0x6b5f0 is taken when [sp-22]:1 == 0x2 to 0x6b618; it falls through when [sp-22]:1 != 0x2 to 0x6b5f4.
  For the call at 0x6b640, `beq` at 0x6b654 is taken when ret(0x6aff0) == 0x0 to 0x6b66c; it falls through when ret(0x6aff0) != 0x0 to 0x6b658.
  `bne` at 0x6b660 is taken when [sp-36] != 0x0 to 0x6b66c; it falls through when [sp-36] == 0x0 to 0x6b664.
  The return (`ldm` at 0x6b68c) has r0 = unknown.
  The instructions from 0x6b690 to 0x6b698 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x6b69c to 0x6b6a8 are not reached by any path from the entry, and their branches are not listed.
  Undecodable bytes at 0x6b698-0x6b69c; fall-through stops at this range.
  Undecodable bytes at 0x6b6a8-0x6b6ac; fall-through stops at this range.
  Unsupported-effect warnings: 0x6b474 rsb: unsupported effects invalidated.

- `powerReset`, `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, 0x615e8.
  Version 1.62.1, package `cd-ist-3.24:l1#4` (Linux L1 image), inner `/stand/sysco/bin/l1`; occupies 544 bytes.
  There are 4 word loads from PC-relative literal locations, including `ldr` at 0x61604 reads absolute 0x617fc; `ldr` at 0x61628 reads absolute 0x61800; `ldr` at 0x61640 reads absolute 0x61804.
  A word load accesses absolute 0x1b5d04 (`ldr` at 0x61608).
  2 such word loads access absolute 0x1b5d00, including `ldr` at 0x6162c, `ldr` at 0x61664.
  A word load accesses an unknown effective address (`ldr` at 0x61634).
  3 such byte stores access [sp-48] +0xf, including `strb` at 0x61678, `strb` at 0x61774, `strb` at 0x61798.
  A word load accesses [sp-48] +0x8 (`ldr` at 0x61688).
  A word load accesses [sp-48] +0x0 (`ldr` at 0x616b0).
  A word load accesses [sp-48] +0x4 (`ldr` at 0x616c0).
  A byte load accesses ([[sp-48]+4] + [sp-44]) +0x0 (`ldrb` at 0x616cc).
  A byte load accesses [sp-24] +0x6 (`ldrb` at 0x616dc).
  2 such byte loads access [sp-48] +0xf, including `ldrsb` at 0x61760, `ldrsb` at 0x61780.
  `bl` at 0x616f0 calls (unknown callee) [0x67d90] with r0=[sp-20], r1=[sp-24], r2=[[sp-48]+4], r3=[[sp-24]+6]:1.
  `bl` at 0x61730 calls (unknown callee) [0x67ca0] with r0=[sp-20], r1=[sp-24], r2=unknown, r3=ret(0x67d90).
  `bl` at 0x61750 calls (unknown callee) [0x67144] with r0=[sp-20], r1=[sp-24], r2=unknown, r3=unknown.
  `bl` at 0x617ac calls (unknown callee) [0x677fc] with r0=[sp-20], r1=[sp-24], r2=[sp-25]:1, r3=[sp-25]:1.
  `beq` at 0x61610 is taken when [0x1b5d04] == 0x0 to 0x61640; it falls through when [0x1b5d04] != 0x0 to 0x61614.
  `bne` at 0x6163c is taken when unknown != 0x0 to 0x6164c; it falls through when unknown == 0x0 to 0x61640.
  `ble` at 0x61694 is taken when le flags condition (operands unknown) to 0x617e8; it falls through when not (le flags condition (operands unknown)) to 0x61698.
  `beq` at 0x616d4 is taken when [([[sp-48]+4] + [sp-44])]:1 == 0x0 to 0x617d8; it falls through when [([[sp-48]+4] + [sp-44])]:1 != 0x0 to 0x616d8.
  `beq` at 0x616e4 is taken when [[sp-24]+6]:1 == 0xff to 0x617d8; it falls through when [[sp-24]+6]:1 != 0xff to 0x616e8.
  For the call at 0x616f0, `beq` at 0x6170c is taken when ret(0x67d90) == 0x0 to 0x61748; it falls through when ret(0x67d90) != 0x0 to 0x61710.
  For the call at 0x616f0, `blt` at 0x61718 is taken when ret(0x67d90) < 0x0 (signed, 32-bit) to 0x61740; it falls through when ret(0x67d90) >= 0x0 (signed, 32-bit) to 0x6171c.
  For the call at 0x616f0, `bgt` at 0x61724 is taken when ret(0x67d90) > 0x4 (signed, 32-bit) to 0x61740; it falls through when ret(0x67d90) <= 0x4 (signed, 32-bit) to 0x61728.
  `bne` at 0x61768 is taken when Z == 0 to 0x6177c; it falls through when not (Z == 0) to 0x6176c.
  `beq` at 0x6178c is taken when unknown == [sp-32] to 0x6179c; it falls through when unknown != [sp-32] to 0x61790.
  For the call at 0x617ac, `beq` at 0x617c0 is taken when ret(0x677fc) == 0x0 to 0x617d8; it falls through when ret(0x677fc) != 0x0 to 0x617c4.
  `bne` at 0x617cc is taken when [sp-36] != 0x0 to 0x617d8; it falls through when [sp-36] == 0x0 to 0x617d0.
  The return (`ldm` at 0x617f8) has r0 = unknown.
  The instructions from 0x617fc to 0x61804 are not reached by any path from the entry, and their branches are not listed.
  Undecodable bytes at 0x61804-0x61808; fall-through stops at this range.
  Unsupported-effect warnings: 0x61674 mvn: unsupported effects invalidated; 0x616a8 rsb: unsupported effects invalidated; 0x61764 cmn: unsupported effects invalidated.

- `i2cInit`, `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, 0xbb594.
  Established for version 1.62.1, package `cd-ist-3.24:l1#2` (Legacy Linux L1 image), inner `/stand/sysco/bin/l1`; occupies 472 bytes.
  Calls (unknown callee), target 0x201fc, with `r0=0x9, r1=0x11f908, r2=0x4d, r3=0x0` (`bl` at 0xbb5d0).
  Calls (unknown callee), target 0x197d8, with `r0=0x11f910, r1=0x1002, r2=unknown, r3=unknown` (`bl` at 0xbb5dc).
  Calls (unknown callee), target 0x1952c, with `r0=ret(0x197d8), r1=unknown, r2=unknown, r3=ret(0x197d8)` (`bl` at 0xbb5f4).
  Calls (unknown callee), target 0x201fc, with `r0=0x9, r1=0x11f908, r2=0x53, r3=0x0` (`bl` at 0xbb620).
  Calls (unknown callee), target 0x19814, with `r0=0x1, r1=unknown, r2=unknown, r3=unknown` (`bl` at 0xbb628).
  Calls (unknown callee), target 0x19514, with `r0=0x0, r1=0x100, r2=0x3, r3=0x1` (`bl` at 0xbb650).
  Calls (unknown callee), target 0x1952c, with `r0=ret(0x19514), r1=unknown, r2=unknown, r3=[0x29c114]` (`bl` at 0xbb664).
  Calls (unknown callee), target 0x201fc, with `r0=0x9, r1=0x11f908, r2=0x5b, r3=0x0` (`bl` at 0xbb690).
  Calls (unknown callee), target 0x19814, with `r0=0x1, r1=unknown, r2=unknown, r3=unknown` (`bl` at 0xbb698).
  Calls (unknown callee), target 0x197d8, with `r0=0x11f968, r1=0x2, r2=0x29c114, r3=[0x29c114]+36` (`bl` at 0xbb6b8).
  Calls (unknown callee), target 0x1952c, with `r0=ret(0x197d8), r1=unknown, r2=ret(0x197d8), r3=[0x12e8e4]` (`bl` at 0xbb6d4).
  Calls (unknown callee), target 0x1fa3c, with `r0=[ret(0x1952c)], r1=unknown, r2=unknown, r3=ret(0x1952c)` (`bl` at 0xbb6e0).
  Calls (unknown callee), target 0x201fc, with `r0=0x9, r1=0x11f908, r2=0x65, r3=0x0` (`bl` at 0xbb704).
  Calls (unknown callee), target 0xc9d2c, with `r0=unknown, r1=unknown, r2=unknown, r3=unknown` (`bl` at 0xbb724).
  Calls (unknown callee), target 0xbbd14, with `r0=ret(0xc9d2c), r1=unknown, r2=unknown, r3=unknown` (`bl` at 0xbb728).
  16 word loads through `pc` read literal locations, including 0xbb744 (`ldr` at 0xbb5a8), 0xbb748 (`ldr` at 0xbb5b8), 0xbb74c (`ldr` at 0xbb5c4).
  2 word loads address absolute 0x12e8e4 (`ldr` at 0xbb5ac, `ldr` at 0xbb6c8).
  3 such word loads access `ret(0x1952c) +0x0`, from calls at 0xbb5f4, 0xbb664 and 0xbb6d4 respectively, including `ldr` at 0xbb608, `ldr` at 0xbb678, `ldr` at 0xbb6dc.
  2 word stores address absolute 0x29c114 (`str` at 0xbb654, `str` at 0xbb6ac).
  2 word loads address absolute 0x29c114 (`ldr` at 0xbb658, `ldr` at 0xbb6a4).
  A word store addresses absolute 0x12e8e4 (`str` at 0xbb6c4).
  For the result of the `bl` at 0xbb5dc, `mov` at 0xbb5e0 copies r0 to r3 and `str` at 0xbb5e4 writes the word to `sp -0x1c`; `ldr` at 0xbb5e8 reloads that word before the following comparison.
  `blt` at 0xbb5b4 takes target 0xbb5d4 when `[0x12e8e4] < 0x0 (signed, 32-bit)`; it falls through to 0xbb5b8 when `[0x12e8e4] >= 0x0 (signed, 32-bit)`.
  For the call-site 0xbb5dc, `bge` at 0xbb5f0 takes target 0xbb62c when `ret(0x197d8) >= 0x0 (signed, 32-bit)`; it falls through to 0xbb5f4 when `ret(0x197d8) < 0x0 (signed, 32-bit)`.
  `bne` at 0xbb660 takes target 0xbb69c when `Z == 0`; it falls through to 0xbb664 when `not (Z == 0)`.
  `bge` at 0xbb6d0 takes target 0xbb708 when `[0x12e8e4] >= 0x0 (signed, 32-bit)`; it falls through to 0xbb6d4 when `[0x12e8e4] < 0x0 (signed, 32-bit)`.
  `bhi` at 0xbb718 takes target 0xbb73c when `hi flags condition (operands unknown)`; it falls through to 0xbb71c when `not (hi flags condition (operands unknown))`.
  At the return (`ldm` at 0xbb740), `r0` is unknown.
  The instructions from 0xbb744 to 0xbb76c are not reached by any path from the entry, and their branches are not listed.
  Analysis warning: 0xbb65c cmn: unsupported effects invalidated.

- `i2cRead`, `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, 0xbbe4c.
  Established for version 1.62.1, package `cd-ist-3.24:l1#2` (Legacy Linux L1 image), inner `/stand/sysco/bin/l1`; occupies 80 bytes.
  Calls (unknown callee), target 0xbc150, with `r0=0x1, r1=(r0 & 0xff), r2=unknown, r3=[sp-24]` (`bl` at 0xbbe88).
  At the return (`ldm` at 0xbbe98), r0 holds the value copied from call-site 0xbbe88 r0 by `mov` at 0xbbe8c and `mov` at 0xbbe90; the intervening `sub` at 0xbbe94 changes sp only.
  Analysis warning: 0xbbe80 mvn: unsupported effects invalidated.

- `i2cWrite`, `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, 0xbbd9c.
  Established for version 1.62.1, package `cd-ist-3.24:l1#2` (Legacy Linux L1 image), inner `/stand/sysco/bin/l1`; occupies 80 bytes.
  Calls (unknown callee), target 0xbc150, with `r0=0x0, r1=(r0 & 0xff), r2=unknown, r3=[sp-24]` (`bl` at 0xbbdd8).
  At the return (`ldm` at 0xbbde8), r0 holds the value copied from call-site 0xbbdd8 r0 by `mov` at 0xbbddc and `mov` at 0xbbde0; the intervening `sub` at 0xbbde4 changes sp only.
  Analysis warning: 0xbbdd0 mvn: unsupported effects invalidated.

- `fanSetSpeed`, `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, 0x412ac.
  Established for version 1.62.1, package `cd-ist-3.24:l1#2` (Legacy Linux L1 image), inner `/stand/sysco/bin/l1`; occupies 520 bytes.
  Calls (unknown callee), target 0x41d6c, with `r0=r0, r1=(r1 & 0xff), r2=sp-30, r3=sp-31` (`bl` at 0x4130c).
  Calls (unknown callee), target 0x4188c, with `r0=[sp-20], r1=unknown, r2=unknown, r3=unknown` (`bl` at 0x41388).
  Calls (unknown callee), target 0x419cc, with `r0=[sp-20], r1=[ret(0x4188c)]:1, r2=unknown, r3=[ret(0x4188c)]:1` (`bl` at 0x4139c).
  Calls (unknown callee), target 0x4100c, with `r0=[sp-20], r1=[sp-32]:1, r2=unknown, r3=[sp-32]:1` (`bl` at 0x4145c).
  2 word loads through `pc` read literal locations, including 0x414ac (`ldr` at 0x412e4), 0x414b0 (`ldr` at 0x413c8).
  A word load addresses absolute 0x13321c (`ldr` at 0x412e8).
  A byte load addresses `[sp-44] +0x4` (`ldrb` at 0x4134c).
  A word load addresses `[sp-44] +0x8` (`ldr` at 0x41364).
  A byte load addresses an unknown effective address (`ldrb` at 0x4136c).
  A byte load accesses `ret(0x4188c) +0x0`, from the call at 0x41388 (`ldrb` at 0x41390).
  A byte load accesses `ret(0x419cc) +0x3`, from the call at 0x4139c (`ldrb` at 0x413ac).
  A byte load accesses `ret(0x419cc) +0x7`, from the call at 0x4139c (`ldrb` at 0x41418).
  A byte load accesses `ret(0x419cc) +0x8`, from the call at 0x4139c (`ldrb` at 0x41428).
  A byte load accesses `ret(0x419cc) +0x9`, from the call at 0x4139c (`ldrb` at 0x41438).
  For the result of the `bl` at 0x4130c, `mov` at 0x41310 copies r0 to r3 and `str` at 0x41314 writes the word to `sp -0x24`; `ldr` at 0x41318 reloads that word before the following comparison.
  For the result of the `bl` at 0x4145c, `mov` at 0x41460 copies r0 to r3 and `str` at 0x41464 writes the word to `sp -0x28`; `ldr` at 0x41468 reloads that word before the following comparison.
  For the call-site 0x4130c, `beq` at 0x41320 takes target 0x41330 when `ret(0x41d6c) == 0x0`; it falls through to 0x41324 when `ret(0x41d6c) != 0x0`.
  `bhi` at 0x41344 takes target 0x41498 when `hi flags condition (operands unknown)`; it falls through to 0x41348 when `not (hi flags condition (operands unknown))`.
  `bls` at 0x41358 takes target 0x41488 when `ls flags condition (operands unknown)`; it falls through to 0x4135c when `not (ls flags condition (operands unknown))`.
  `bne` at 0x41374 takes target 0x4137c when `Z == 0`; it falls through to 0x41378 when `not (Z == 0)`.
  For call-site 0x4139c, `bne` at 0x413b4 takes target 0x413d4 when `[ret(0x419cc)+3]:1 != 0xff`; it falls through to 0x413b8 when `[ret(0x419cc)+3]:1 == 0xff`.
  `bne` at 0x413c0 takes target 0x413c8 when `[sp-21]:1 != 0xff`; it falls through to 0x413c4 when `[sp-21]:1 == 0xff`.
  `beq` at 0x413e4 takes target 0x41424 when `[sp-22]:1 == 0x1`; it falls through to 0x413e8 when `[sp-22]:1 != 0x1`.
  `bgt` at 0x413f0 takes target 0x41404 when `[sp-22]:1 > 0x1 (signed, 32-bit)`; it falls through to 0x413f4 when `[sp-22]:1 <= 0x1 (signed, 32-bit)`.
  `beq` at 0x413fc takes target 0x41414 when `[sp-22]:1 == 0x0`; it falls through to 0x41400 when `[sp-22]:1 != 0x0`.
  `beq` at 0x4140c takes target 0x41434 when `[sp-22]:1 == 0x2`; it falls through to 0x41410 when `[sp-22]:1 != 0x2`.
  For the call-site 0x4145c, `beq` at 0x41470 takes target 0x41488 when `ret(0x4100c) == 0x0`; it falls through to 0x41474 when `ret(0x4100c) != 0x0`.
  `bne` at 0x4147c takes target 0x41488 when `[sp-36] != 0x0`; it falls through to 0x41480 when `[sp-36] == 0x0`.
  At the return (`ldm` at 0x414a8), `r0` is unknown.
  The instructions from 0x414ac to 0x414b0 are not reached by any path from the entry, and their branches are not listed.
  The bytes from 0x414b0 to 0x414b4 are undecodable; fall-through stops at that range, whose end is exclusive.

- `powerReset`, `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, 0x36e28.
  Established for version 1.62.1, package `cd-ist-3.24:l1#2` (Legacy Linux L1 image), inner `/stand/sysco/bin/l1`; occupies 460 bytes.
  Calls (unknown callee), target 0x3b480, with `r0=[sp-20], r1=[sp-28], r2=[[sp-24]+4], r3=[[sp-28]+6]:1` (`bl` at 0x36ee8).
  Calls (unknown callee), target 0x3b408, with `r0=[sp-20], r1=[sp-28], r2=unknown, r3=ret(0x3b480)` (`bl` at 0x36f28).
  Calls (unknown callee), target 0x3aa64, with `r0=[sp-20], r1=[sp-28], r2=unknown, r3=unknown` (`bl` at 0x36f48).
  Calls (unknown callee), target 0x3af60, with `r0=[sp-20], r1=[sp-28], r2=[sp-29]:1, r3=[sp-29]:1` (`bl` at 0x36fa4).
  A word load through `pc` reads the literal location 0x36ff0 (`ldr` at 0x36e50).
  A word load addresses absolute 0x1331ac (`ldr` at 0x36e54).
  3 byte stores address `[sp-24] +0xf` (`strb` at 0x36e70, `strb` at 0x36f6c, `strb` at 0x36f90).
  A word load addresses `[sp-24] +0x8` (`ldr` at 0x36e80).
  A word load addresses `[sp-24] +0x0` (`ldr` at 0x36ea8).
  A word load addresses `[sp-24] +0x4` (`ldr` at 0x36eb8).
  A byte load addresses `([[sp-24]+4] + [sp-48]) +0x0` (`ldrb` at 0x36ec4).
  A byte load addresses `[sp-28] +0x6` (`ldrb` at 0x36ed4).
  2 byte loads address `[sp-24] +0xf` (`ldrsb` at 0x36f58, `ldrsb` at 0x36f78).
  For the result of the `bl` at 0x36ee8, `mov` at 0x36eec copies r0 to r3 and `str` at 0x36ef0 writes the word to `sp -0x24`; `ldr` at 0x36ef4 reloads that word before the following comparison.
  For the result of the `bl` at 0x36fa4, `mov` at 0x36fa8 copies r0 to r3 and `str` at 0x36fac writes the word to `sp -0x2c`; `ldr` at 0x36fb0 reloads that word before the following comparison.
  The result word from the `bl` at 0x36ee8 is copied from `sp -0x24` to `sp -0x34` by `ldr` at 0x36ef4 and `str` at 0x36ef8; `ldr` at 0x36efc, 0x36f08 and 0x36f14 reload that slot before the comparisons at 0x36f00, 0x36f0c and 0x36f18.
  `ble` at 0x36e8c takes target 0x36fe0 when `le flags condition (operands unknown)`; it falls through to 0x36e90 when `not (le flags condition (operands unknown))`.
  `beq` at 0x36ecc takes target 0x36fd0 when `[([[sp-24]+4] + [sp-48])]:1 == 0x0`; it falls through to 0x36ed0 when `[([[sp-24]+4] + [sp-48])]:1 != 0x0`.
  `beq` at 0x36edc takes target 0x36fd0 when `[[sp-28]+6]:1 == 0xff`; it falls through to 0x36ee0 when `[[sp-28]+6]:1 != 0xff`.
  For the call-site 0x36ee8, `beq` at 0x36f04 takes target 0x36f40 when `ret(0x3b480) == 0x0`; it falls through to 0x36f08 when `ret(0x3b480) != 0x0`.
  For the call-site 0x36ee8, `blt` at 0x36f10 takes target 0x36f38 when `ret(0x3b480) < 0x0 (signed, 32-bit)`; it falls through to 0x36f14 when `ret(0x3b480) >= 0x0 (signed, 32-bit)`.
  For the call-site 0x36ee8, `bgt` at 0x36f1c takes target 0x36f38 when `ret(0x3b480) > 0x4 (signed, 32-bit)`; it falls through to 0x36f20 when `ret(0x3b480) <= 0x4 (signed, 32-bit)`.
  `bne` at 0x36f60 takes target 0x36f74 when `Z == 0`; it falls through to 0x36f64 when `not (Z == 0)`.
  `beq` at 0x36f84 takes target 0x36f94 when `unknown == [sp-36]`; it falls through to 0x36f88 when `unknown != [sp-36]`.
  For the call-site 0x36fa4, `beq` at 0x36fb8 takes target 0x36fd0 when `ret(0x3af60) == 0x0`; it falls through to 0x36fbc when `ret(0x3af60) != 0x0`.
  `bne` at 0x36fc4 takes target 0x36fd0 when `[sp-40] != 0x0`; it falls through to 0x36fc8 when `[sp-40] == 0x0`.
  At the return (`ldm` at 0x36fec), `r0` is `[sp-40]`.
  The instructions from 0x36ff0 to 0x36ff4 are not reached by any path from the entry, and their branches are not listed.
  Analysis warning: 0x36e6c mvn: unsupported effects invalidated.
  Analysis warning: 0x36ea0 rsb: unsupported effects invalidated.
  Analysis warning: 0x36f5c cmn: unsupported effects invalidated.

- `i2cInit`, `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`, 0x112ad4
  Established for ARM L1 1.48.1, package `patches/SG0007149:l1#6` (Linux L1 image), inner `/stand/sysco/bin/l1`. The routine occupies 472 bytes.
  There are 16 word reads relative to pc, including `ldr` at 0x112ae8 (absolute 0x112c84), `ldr` at 0x112af8 (absolute 0x112c88), `ldr` at 0x112b04 (absolute 0x112c8c).
  2 such word reads at absolute 0x1ab708, including `ldr` at 0x112aec, `ldr` at 0x112c08.
  2 such word writes at absolute 0x32e490, including `str` at 0x112b94, `str` at 0x112bec.
  2 such word reads at absolute 0x32e490, including `ldr` at 0x112b98, `ldr` at 0x112be4.
  1 such word write at absolute 0x1ab708, including `str` at 0x112c04.
  3 such word reads access `ret(0x206d0) +0x0`, from calls at 0x112b34, 0x112ba4 and 0x112c14 respectively (`ldr` at 0x112b48, `ldr` at 0x112bb8, `ldr` at 0x112c1c).
  `bl` at 0x112b10 calls (unknown callee), target 0x22764, with r0=0x9, r1=0x197688, r2=0x59, r3=0x0.
  `bl` at 0x112b1c calls (unknown callee), target 0x209dc, with r0=0x197690, r1=0x1002, r2=unknown, r3=unknown.
  `bl` at 0x112b34 calls (unknown callee), target 0x206d0, with r0=ret(0x209dc), r1=unknown, r2=unknown, r3=ret(0x209dc).
  `bl` at 0x112b60 calls (unknown callee), target 0x22764, with r0=0x9, r1=0x197688, r2=0x5f, r3=0x0.
  `bl` at 0x112b68 calls (unknown callee), target 0x20a24, with r0=0x1, r1=unknown, r2=unknown, r3=unknown.
  `bl` at 0x112b90 calls (unknown callee), target 0x206b8, with r0=0x0, r1=0x100, r2=0x3, r3=0x1.
  `bl` at 0x112ba4 calls (unknown callee), target 0x206d0, with r0=ret(0x206b8), r1=unknown, r2=unknown, r3=[0x32e490].
  `bl` at 0x112bd0 calls (unknown callee), target 0x22764, with r0=0x9, r1=0x197688, r2=0x67, r3=0x0.
  `bl` at 0x112bd8 calls (unknown callee), target 0x20a24, with r0=0x1, r1=unknown, r2=unknown, r3=unknown.
  `bl` at 0x112bf8 calls (unknown callee), target 0x209dc, with r0=0x1976e8, r1=0x2, r2=0x32e490, r3=[0x32e490]+36.
  `bl` at 0x112c14 calls (unknown callee), target 0x206d0, with r0=ret(0x209dc), r1=unknown, r2=ret(0x209dc), r3=[0x1ab708].
  `bl` at 0x112c20 calls (unknown callee), target 0x22098, with r0=[ret(0x206d0)], r1=unknown, r2=unknown, r3=ret(0x206d0).
  `bl` at 0x112c44 calls (unknown callee), target 0x22764, with r0=0x9, r1=0x197688, r2=0x71, r3=0x0.
  `bl` at 0x112c64 calls (unknown callee), target 0x12cdd4, with r0=unknown, r1=unknown, r2=unknown, r3=unknown.
  `bl` at 0x112c68 calls (unknown callee), target 0x11330c, with r0=ret(0x12cdd4), r1=unknown, r2=unknown, r3=unknown.
  `blt` at 0x112af4 is taken to 0x112b14 when `[0x1ab708] < 0x0 (signed, 32-bit)`; it falls through to 0x112af8 when `[0x1ab708] >= 0x0 (signed, 32-bit)`.
  For the result of the call at 0x112b1c, `bge` at 0x112b30 is taken to 0x112b6c when `ret(0x209dc) >= 0x0 (signed, 32-bit)`; it falls through to 0x112b34 when `ret(0x209dc) < 0x0 (signed, 32-bit)`.
  The result of the call at 0x112b1c reaches this comparison through `mov` at 0x112b20, the word store to sp -0x1c (`str` at 0x112b24), and its reload (`ldr` at 0x112b28).
  `bne` at 0x112ba0 is taken to 0x112bdc when `Z == 0`; it falls through to 0x112ba4 when `not (Z == 0)`.
  `bge` at 0x112c10 is taken to 0x112c48 when `[0x1ab708] >= 0x0 (signed, 32-bit)`; it falls through to 0x112c14 when `[0x1ab708] < 0x0 (signed, 32-bit)`.
  `bhi` at 0x112c58 is taken to 0x112c7c when `hi flags condition (operands unknown)`; it falls through to 0x112c5c when `not (hi flags condition (operands unknown))`.
  At `ldm` at 0x112c80, the returned r0 value is unknown.
  The instructions from 0x112c84 to 0x112cac are not reached by any path from the entry, and their branches are not listed.
  Analysis warning: 0x112b9c cmn: unsupported effects invalidated.

- `i2cRead`, `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`, 0x113444
  Established for ARM L1 1.48.1, package `patches/SG0007149:l1#6` (Linux L1 image), inner `/stand/sysco/bin/l1`. The routine occupies 80 bytes.
  `bl` at 0x113480 calls (unknown callee), target 0x113748, with r0=0x1, r1=(r0 & 0xff), r2=unknown, r3=[sp-24].
  At `ldm` at 0x113490, r0 carries the result of the call at 0x113480, copied through `mov` at 0x113484 and `mov` at 0x113488, with no later r0 writer before the return.
  Analysis warning: 0x113478 mvn: unsupported effects invalidated.

- `i2cWrite`, `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`, 0x113394
  Established for ARM L1 1.48.1, package `patches/SG0007149:l1#6` (Linux L1 image), inner `/stand/sysco/bin/l1`. The routine occupies 80 bytes.
  `bl` at 0x1133d0 calls (unknown callee), target 0x113748, with r0=0x0, r1=(r0 & 0xff), r2=unknown, r3=[sp-24].
  At `ldm` at 0x1133e0, r0 carries the result of the call at 0x1133d0, copied through `mov` at 0x1133d4 and `mov` at 0x1133d8, with no later r0 writer before the return.
  Analysis warning: 0x1133c8 mvn: unsupported effects invalidated.

- `fanSetSpeed`, `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`, 0x6a064
  Established for ARM L1 1.48.1, package `patches/SG0007149:l1#6` (Linux L1 image), inner `/stand/sysco/bin/l1`. The routine occupies 796 bytes.
  There are 9 word reads relative to pc, including `ldr` at 0x6a088 (absolute 0x6a364), `ldr` at 0x6a0b0 (absolute 0x6a368), `ldr` at 0x6a0c8 (absolute 0x6a36c).
  1 such word read at absolute 0x1b11cc, including `ldr` at 0x6a090.
  2 such word reads at absolute 0x1b11c8, including `ldr` at 0x6a0b4, `ldr` at 0x6a0ec.
  1 such byte read at unknown effective address, including `ldrb` at 0x6a0bc.
  2 such byte reads at [sp-44] +0x4, including `ldrb` at 0x6a0fc, `ldrb` at 0x6a18c.
  1 such word read at [sp-44] +0x8, including `ldr` at 0x6a21c.
  1 such byte read at unknown effective address, including `ldrb` at 0x6a224.
  A byte read accesses `ret(0x6a758) +0x0`, from the call at 0x6a240 (`ldrb` at 0x6a248).
  A byte read accesses `ret(0x6a888) +0x3`, from the call at 0x6a254 (`ldrb` at 0x6a264).
  A byte read accesses `ret(0x6a888) +0x7`, from the call at 0x6a254 (`ldrb` at 0x6a2d0).
  A byte read accesses `ret(0x6a888) +0x8`, from the call at 0x6a254 (`ldrb` at 0x6a2e0).
  A byte read accesses `ret(0x6a888) +0x9`, from the call at 0x6a254 (`ldrb` at 0x6a2f0).
  `bl` at 0x6a118 calls (unknown callee), target 0x32844, with r0=r0, r1=0x1, r2=0x0, r3=0x0.
  `bl` at 0x6a124 calls (unknown callee), target 0x2088c, with r0=ret(0x32844), r1=unknown, r2=unknown, r3=unknown.
  `bl` at 0x6a16c calls (unknown callee), target 0x20964, with r0=[sp-56], r1=[sp-48], r2=[sp-52], r3=unknown.
  `bl` at 0x6a1a4 calls (unknown callee), target 0x22764, with r0=0x9, r1=0x16cc84, r2=0x2bb, r3=0x0.
  `bl` at 0x6a1d8 calls (unknown callee), target 0x6a9ac, with r0=unknown, r1=unknown, r2=sp-30, r3=sp-31.
  `bl` at 0x6a240 calls (unknown callee), target 0x6a758, with r0=[sp-20], r1=unknown, r2=unknown, r3=unknown.
  `bl` at 0x6a254 calls (unknown callee), target 0x6a888, with r0=[sp-20], r1=[ret(0x6a758)]:1, r2=unknown, r3=[ret(0x6a758)]:1.
  `bl` at 0x6a314 calls (unknown callee), target 0x69cc4, with r0=[sp-20], r1=[sp-32]:1, r2=unknown, r3=[sp-32]:1.
  `bge` at 0x6a098 is taken to 0x6a0c8 when `r0 >= [0x1b11cc] (signed, 32-bit)`; it falls through to 0x6a09c when `r0 < [0x1b11cc] (signed, 32-bit)`.
  `bne` at 0x6a0c4 is taken to 0x6a0d4 when `unknown != 0x0`; it falls through to 0x6a0c8 when `unknown == 0x0`.
  `bne` at 0x6a104 is taken to 0x6a1a8 when `[[sp-44]+4]:1 != 0x0`; it falls through to 0x6a108 when `[[sp-44]+4]:1 == 0x0`.
  `bne` at 0x6a1b0 is taken to 0x6a1c0 when `[sp-44] != 0x0`; it falls through to 0x6a1b4 when `[sp-44] == 0x0`.
  For the result of the call at 0x6a1d8, `beq` at 0x6a1ec is taken to 0x6a1fc when `ret(0x6a9ac) == 0x0`; it falls through to 0x6a1f0 when `ret(0x6a9ac) != 0x0`.
  The result of the call at 0x6a1d8 reaches this comparison through `mov` at 0x6a1dc, the word store to sp -0x24 (`str` at 0x6a1e0), and its reload (`ldr` at 0x6a1e4).
  `bhs` at 0x6a210 is taken to 0x6a350 when `C == 1`; it falls through to 0x6a214 when `not (C == 1)`.
  `bne` at 0x6a22c is taken to 0x6a234 when `Z == 0`; it falls through to 0x6a230 when `not (Z == 0)`.
  For the result of the call at 0x6a254, `bne` at 0x6a26c is taken to 0x6a28c when `[ret(0x6a888)+3]:1 != 0xff`; it falls through to 0x6a270 when `[ret(0x6a888)+3]:1 == 0xff`.
  The result of the call at 0x6a254 reaches the byte load through `mov` at 0x6a258, the word store to sp -0x1c (`str` at 0x6a25c), and its reload (`ldr` at 0x6a260).
  `bne` at 0x6a278 is taken to 0x6a280 when `[sp-21]:1 != 0xff`; it falls through to 0x6a27c when `[sp-21]:1 == 0xff`.
  `beq` at 0x6a29c is taken to 0x6a2dc when `[sp-22]:1 == 0x1`; it falls through to 0x6a2a0 when `[sp-22]:1 != 0x1`.
  `bgt` at 0x6a2a8 is taken to 0x6a2bc when `[sp-22]:1 > 0x1 (signed, 32-bit)`; it falls through to 0x6a2ac when `[sp-22]:1 <= 0x1 (signed, 32-bit)`.
  `beq` at 0x6a2b4 is taken to 0x6a2cc when `[sp-22]:1 == 0x0`; it falls through to 0x6a2b8 when `[sp-22]:1 != 0x0`.
  `beq` at 0x6a2c4 is taken to 0x6a2ec when `[sp-22]:1 == 0x2`; it falls through to 0x6a2c8 when `[sp-22]:1 != 0x2`.
  For the result of the call at 0x6a314, `beq` at 0x6a328 is taken to 0x6a340 when `ret(0x69cc4) == 0x0`; it falls through to 0x6a32c when `ret(0x69cc4) != 0x0`.
  The result of the call at 0x6a314 reaches this comparison through `mov` at 0x6a318, the word store to sp -0x28 (`str` at 0x6a31c), and its reload (`ldr` at 0x6a320).
  `bne` at 0x6a334 is taken to 0x6a340 when `[sp-36] != 0x0`; it falls through to 0x6a338 when `[sp-36] == 0x0`.
  At `ldm` at 0x6a360, the returned r0 value is unknown.
  The instructions from 0x6a364 to 0x6a36c are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x6a370 to 0x6a37c are not reached by any path from the entry, and their branches are not listed.
  Analysis warning: 0x6a148 rsb: unsupported effects invalidated.
  Analysis warning: undecodable bytes at 0x6a36c-0x6a370; fall-through stops at this range.
  Analysis warning: undecodable bytes at 0x6a37c-0x6a380; fall-through stops at this range.

- `powerReset`, `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`, 0x602bc
  Established for ARM L1 1.48.1, package `patches/SG0007149:l1#6` (Linux L1 image), inner `/stand/sysco/bin/l1`. The routine occupies 544 bytes.
  There are 4 word reads relative to pc, including `ldr` at 0x602d8 (absolute 0x604d0), `ldr` at 0x602fc (absolute 0x604d4), `ldr` at 0x60314 (absolute 0x604d8).
  1 such word read at absolute 0x1b11c0, including `ldr` at 0x602dc.
  2 such word reads at absolute 0x1b11bc, including `ldr` at 0x60300, `ldr` at 0x60338.
  1 such word read at unknown effective address, including `ldr` at 0x60308.
  3 such byte writes at [sp-48] +0xf, including `strb` at 0x6034c, `strb` at 0x60448, `strb` at 0x6046c.
  1 such word read at [sp-48] +0x8, including `ldr` at 0x6035c.
  1 such word read at [sp-48] +0x0, including `ldr` at 0x60384.
  1 such word read at [sp-48] +0x4, including `ldr` at 0x60394.
  1 such byte read at ([[sp-48]+4] + [sp-44]) +0x0, including `ldrb` at 0x603a0.
  1 such byte read at [sp-24] +0x6, including `ldrb` at 0x603b0.
  2 such byte reads at [sp-48] +0xf, including `ldrsb` at 0x60434, `ldrsb` at 0x60454.
  `bl` at 0x603c4 calls (unknown callee), target 0x66a64, with r0=[sp-20], r1=[sp-24], r2=[[sp-48]+4], r3=[[sp-24]+6]:1.
  `bl` at 0x60404 calls (unknown callee), target 0x66974, with r0=[sp-20], r1=[sp-24], r2=unknown, r3=ret(0x66a64).
  `bl` at 0x60424 calls (unknown callee), target 0x65e18, with r0=[sp-20], r1=[sp-24], r2=unknown, r3=unknown.
  `bl` at 0x60480 calls (unknown callee), target 0x664d0, with r0=[sp-20], r1=[sp-24], r2=[sp-25]:1, r3=[sp-25]:1.
  `beq` at 0x602e4 is taken to 0x60314 when `[0x1b11c0] == 0x0`; it falls through to 0x602e8 when `[0x1b11c0] != 0x0`.
  `bne` at 0x60310 is taken to 0x60320 when `unknown != 0x0`; it falls through to 0x60314 when `unknown == 0x0`.
  `ble` at 0x60368 is taken to 0x604bc when `le flags condition (operands unknown)`; it falls through to 0x6036c when `not (le flags condition (operands unknown))`.
  `beq` at 0x603a8 is taken to 0x604ac when `[([[sp-48]+4] + [sp-44])]:1 == 0x0`; it falls through to 0x603ac when `[([[sp-48]+4] + [sp-44])]:1 != 0x0`.
  `beq` at 0x603b8 is taken to 0x604ac when `[[sp-24]+6]:1 == 0xff`; it falls through to 0x603bc when `[[sp-24]+6]:1 != 0xff`.
  For the result of the call at 0x603c4, `beq` at 0x603e0 is taken to 0x6041c when `ret(0x66a64) == 0x0`; it falls through to 0x603e4 when `ret(0x66a64) != 0x0`.
  The result of the call at 0x603c4 passes through `mov` at 0x603c8, the word store to sp -0x20 (`str` at 0x603cc), its reload (`ldr` at 0x603d0), and the word store to sp -0x38 (`str` at 0x603d4). The three comparisons reload that last slot (`ldr` at 0x603d8, 0x603e4, 0x603f0), with no intervening write to it.
  For the result of the call at 0x603c4, `blt` at 0x603ec is taken to 0x60414 when `ret(0x66a64) < 0x0 (signed, 32-bit)`; it falls through to 0x603f0 when `ret(0x66a64) >= 0x0 (signed, 32-bit)`.
  For the result of the call at 0x603c4, `bgt` at 0x603f8 is taken to 0x60414 when `ret(0x66a64) > 0x4 (signed, 32-bit)`; it falls through to 0x603fc when `ret(0x66a64) <= 0x4 (signed, 32-bit)`.
  `bne` at 0x6043c is taken to 0x60450 when `Z == 0`; it falls through to 0x60440 when `not (Z == 0)`.
  `beq` at 0x60460 is taken to 0x60470 when `unknown == [sp-32]`; it falls through to 0x60464 when `unknown != [sp-32]`.
  For the result of the call at 0x60480, `beq` at 0x60494 is taken to 0x604ac when `ret(0x664d0) == 0x0`; it falls through to 0x60498 when `ret(0x664d0) != 0x0`.
  The result of the call at 0x60480 reaches this comparison through `mov` at 0x60484, the word store to sp -0x28 (`str` at 0x60488), and its reload (`ldr` at 0x6048c).
  `bne` at 0x604a0 is taken to 0x604ac when `[sp-36] != 0x0`; it falls through to 0x604a4 when `[sp-36] == 0x0`.
  At `ldm` at 0x604cc, the returned r0 value is unknown.
  The instructions from 0x604d0 to 0x604d8 are not reached by any path from the entry, and their branches are not listed.
  Analysis warning: 0x60348 mvn: unsupported effects invalidated.
  Analysis warning: 0x6037c rsb: unsupported effects invalidated.
  Analysis warning: 0x60438 cmn: unsupported effects invalidated.
  Analysis warning: undecodable bytes at 0x604d8-0x604dc; fall-through stops at this range.
