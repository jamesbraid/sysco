# ARM L1 link, power and UART variants

Each entry applies to its carrier, package and extracted ELF. ARM AAPCS is assumed. Argument counts and custom ABI details are unknown. The four printed register slots are candidate argument locations. Values printed as `unknown` remain unresolved. Symbol names do not establish a hardware role. Value roles, pointee meanings, C types and runtime path feasibility remain unknown.

In unreached-range statements, "any path from the entry" means any path in the recovered known-edge control-flow graph. Range ends are exclusive. Computed transfers with unknown targets remain unresolved. Repeated accesses count static records with the same width, access kind and resolved expression. Their order need not describe one execution path. Grouping unresolved accesses does not establish that their effective addresses are equal. The [complete facts](disasm/arm-variants/link-power-uart-variants-facts.txt) retain every routine’s analysis.

Image `cd-ist-3.24:l1#4`, version 1.62.1: `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, inner `/stand/sysco/bin/l1`.

The package begins at carrier offset 0x7ecf04 and is 4948212 bytes. CramFS begins at package offset 0x1012fc. The ELF is 1999676 bytes, SHA-256 `22cb62871e447a394acbd662f4f49f3146580afec93839953b37a5354545bfc5`.

The executable PT_LOAD begins at ELF file offset 0x0, virtual address 0x8000, and is 1693992 bytes, SHA-256 `5ad8f4be9cbda073b307873683a90846744df308e9629d628f27b0b87af1e73c`. Carrier SHA-256 is `b4b5634949a93f05ad6eb24462e6cd7f10104dd42ff5c2ae8616dd5d87697790`. Package SHA-256 is `8c964136f81ec663d511b553041f07e798d9a30adebdd7b85fe420332e480f89`.

Image `patches/SG0005762:l1#3`, version 1.32.15: `patches/SG0005762/usr/cpu/firmware/sysco/l1.bin`, inner `/stand/sysco/bin/l1`.

The package begins at carrier offset 0x3ac27c and is 3555572 bytes. CramFS begins at package offset 0xb8c04. The ELF is 1182828 bytes, SHA-256 `7def5f2157e5bd37bd0c76685bd101eac73c15dcae304a8f9a900c2cf3594237`.

The executable PT_LOAD begins at ELF file offset 0x0, virtual address 0x8000, and is 1009192 bytes, SHA-256 `e116bdec7fd26f21465201d0c37a512858ce7aacea4c2174dee3dbc6fa71549a`. Carrier SHA-256 is `2399ea17dfeb2af20fae4286f0cfeeffae6d3d9f637fcc02e880e5689f662656`. Package SHA-256 is `67b4258a889facc68d9406dde68f145531d558d90da4eab5867eafa70b25212c`.

Image `patches/SG0007149:l1#4`, version 1.48.1: `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`, inner `/stand/sysco/bin/l1`.

The package begins at carrier offset 0x53d154 and is 3817724 bytes. CramFS begins at package offset 0xbb73c. The ELF is 1363924 bytes, SHA-256 `3eac4cd27d87d21313e0a8e06b6d719cb2ebc4cda81bd74536a04b8b4ad8cd12`.

The executable PT_LOAD begins at ELF file offset 0x0, virtual address 0x8000, and is 1166540 bytes, SHA-256 `9d5ddc793a620fe8f424df57ce1a727054526c98cfaf38ea1e80d69493b379fe`. Carrier SHA-256 is `536bfa4c6d0240b7afbc25bef94e160bdf1a675016c35e68ec52e1bfb1b451f9`. Package SHA-256 is `6bcee9bbd522c9bbd33538db5ea8c2c36666f7bb22c049a7912a72bf2c6c06e2`.

- `i2cBus`, `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, 0x116354.
  Version 1.62.1, package `cd-ist-3.24:l1#4`, inner `/stand/sysco/bin/l1`; occupies 188 bytes.
  4 word loads read PC-relative literal locations, including absolute 0x116404 (`ldr` at 0x116390), absolute 0x116408 (`ldr` at 0x1163a0), absolute 0x11640c (`ldr` at 0x1163ac).
  2 such word loads access absolute 0x32a4ac, including `ldr` at 0x116394, `ldr` at 0x1163d8.
  A word load accesses an unknown effective address (`ldr` at 0x1163e0).
  `bl` at 0x116370 calls (unknown callee) [0x32d44] with r0=r0, r1=r1, r2=r2, r3=r1.
  `bl` at 0x1163b8 calls (unknown callee) [0x22998] with r0=0x9, r1=0x19c118, r2=0x94, r3=0x0.
  `bne` at 0x116380 is taken when ret(0x32d44) != [sp-20] to 0x116390; it falls through when ret(0x32d44) == [sp-20] to 0x116384. The ret expression comes from the call at 0x116370.
  `bne` at 0x11639c is taken when [0x32a4ac] != 0x0 to 0x1163bc; it falls through when [0x32a4ac] == 0x0 to 0x1163a0.
  At the return (`ldm` at 0x116400), r0 is unknown.
  The instructions from 0x116404 to 0x116410 are not reached by any path from the entry, and their branches are not listed.
  The analysis invalidates unsupported effects for `mvn` at 0x1163bc.

- `powerDownSlot`, `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, 0x63a04.
  Version 1.62.1, package `cd-ist-3.24:l1#4`, inner `/stand/sysco/bin/l1`; occupies 208 bytes.
  3 word loads read PC-relative literal locations, including absolute 0x63ac8 (`ldr` at 0x63a58), absolute 0x63acc (`ldr` at 0x63a78), absolute 0x63ad0 (`ldr` at 0x63a98).
  `bl` at 0x63a20 calls (unknown callee) [0x49894] with r0=r0, r1=r1, r2=r2, r3=r3.
  `bl` at 0x63a34 calls (unknown callee) [0x49894] with r0=[sp-20], r1=unknown, r2=unknown, r3=ret(0x49894). The ret expression comes from the call at 0x63a20.
  `bl` at 0x63a48 calls (unknown callee) [0x49894] with r0=[sp-20], r1=unknown, r2=unknown, r3=ret(0x49894). The ret expression comes from the call at 0x63a20.
  `bl` at 0x63a68 calls (unknown callee) [0x4a7a8] with r0=[sp-20], r1=unknown, r2=unknown, r3=ret(0x49894). The ret expression comes from the call at 0x63a48.
  `bl` at 0x63a88 calls (unknown callee) [0x49fa0] with r0=[sp-20], r1=unknown, r2=unknown, r3=ret(0x4a7a8). The ret expression comes from the call at 0x63a68.
  `bl` at 0x63ab0 calls (unknown callee) [0x63528] with r0=[sp-20], r1=[sp-24], r2=0x1, r3=ret(0x49fa0). The ret expression comes from the call at 0x63a88.
  `bge` at 0x63a2c is taken when ret(0x49894) >= 0x0 (signed, 32-bit) to 0x63a44; it falls through when ret(0x49894) < 0x0 (signed, 32-bit) to 0x63a30. The ret expression comes from the call at 0x63a20.
  `bne` at 0x63a54 is taken when ret(0x49894) != 0x0 to 0x63a64; it falls through when ret(0x49894) == 0x0 to 0x63a58. The ret expression comes from the call at 0x63a48.
  `bne` at 0x63a74 is taken when ret(0x4a7a8) != 0x0 to 0x63a84; it falls through when ret(0x4a7a8) == 0x0 to 0x63a78. The ret expression comes from the call at 0x63a68.
  `bne` at 0x63a94 is taken when ret(0x49fa0) != 0x0 to 0x63aa4; it falls through when ret(0x49fa0) == 0x0 to 0x63a98. The ret expression comes from the call at 0x63a88.
  At the return (`ldm` at 0x63ac4), r0 is unknown.
  The bytes from 0x63ac8 to 0x63ad4 are undecodable.

- `SHUB_ScanSetRackSlot`, `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, 0x6cda4.
  Version 1.62.1, package `cd-ist-3.24:l1#4`, inner `/stand/sysco/bin/l1`; occupies 412 bytes.
  10 word loads read PC-relative literal locations, including absolute 0x6cf20 (`ldr` at 0x6cdd0), absolute 0x6cf24 (`ldr` at 0x6cde0), absolute 0x6cf28 (`ldr` at 0x6cdec).
  2 such word loads access absolute 0x32ae54, including `ldr` at 0x6cdd4, `ldr` at 0x6ce68.
  2 such word loads access unknown effective addresses, including `ldr` at 0x6ce00, `ldr` at 0x6ce14.
  A word load accesses an unknown effective address (`ldr` at 0x6ce24).
  A word load accesses an unknown effective address (`ldr` at 0x6ce48).
  `bl` at 0x6cdc4 calls (unknown callee) [0xac40c] with r0=0x0, r1=r1, r2=r2, r3=0x6c.
  `bl` at 0x6cdf8 calls (unknown callee) [0x22998] with r0=0x6, r1=0x170d78, r2=0x1cd, r3=0x0.
  `bl` at 0x6ce9c calls (unknown callee) [0x22998] with r0=0x6, r1=0x170d78, r2=0x1d8, r3=0x0.
  `bl` at 0x6ced0 calls (unknown callee) [0x6c2e4] with r0=[sp-20], r1=0x101d0100, r2=[sp-28], r3=unknown.
  `bl` at 0x6cf08 calls (unknown callee) [0x31734] with r0=0x170e24, r1=[sp-20], r2=unknown, r3=unknown.
  `beq` at 0x6cddc is taken when [0x32ae54] == 0x0 to 0x6cdfc; it falls through when [0x32ae54] != 0x0 to 0x6cde0.
  `beq` at 0x6ce70 is taken when [0x32ae54] == 0x0 to 0x6cea0; it falls through when [0x32ae54] != 0x0 to 0x6ce74.
  `bgt` at 0x6ceb0 is taken when gt flags condition (operands unknown) to 0x6cefc; it falls through when not (gt flags condition (operands unknown)) to 0x6ceb4.
  `bne` at 0x6cedc is taken when ret(0x6c2e4) != 0x1 to 0x6ceec; it falls through when ret(0x6c2e4) == 0x1 to 0x6cee0. The ret expression comes from the call at 0x6ced0.
  At the return (`ldm` at 0x6cf1c), r0 is unknown.
  The instructions from 0x6cf20 to 0x6cf3c are not reached by any path from the entry, and their branches are not listed.
  The bytes from 0x6cf3c to 0x6cf40 are undecodable.

- `i2cReadByte`, `patches/SG0005762/usr/cpu/firmware/sysco/l1.bin`, 0xa3650.
  Version 1.32.15, package `patches/SG0005762:l1#3`, inner `/stand/sysco/bin/l1`. It occupies 132 bytes.
  `bl` at 0xa367c calls (unknown callee) [0xa30d0] with r0=(r0 & 0xff), r1=r1, r2=r2, r3=(r0 & 0xff).
  `bl` at 0xa36b0 calls (unknown callee) [0xa347c] with r0=[sp-18]:1, r1=[sp-24], r2=0x1, r3=[sp-18]:1.
  `bl` at 0xa36c0 calls (unknown callee) [0xa3210] with r0=0x0, r1=unknown, r2=unknown, r3=ret(0xa347c). The ret expression comes from the call at 0xa36b0.
  `beq` at 0xa3690 is taken when ret(0xa30d0) == 0x0 to 0xa36a0. It falls through when ret(0xa30d0) != 0x0 to 0xa3694. The ret expression comes from the call at 0xa367c.
  At the return (`ldmdb` at 0xa36d0), r0 is unknown.

- `i2cWriteByte`, `patches/SG0005762/usr/cpu/firmware/sysco/l1.bin`, 0xa3524.
  Version 1.32.15, package `patches/SG0005762:l1#3`, inner `/stand/sysco/bin/l1`. It occupies 140 bytes.
  `bl` at 0xa3554 calls (unknown callee) [0xa30d0] with r0=(r0 & 0xff), r1=r1, r2=r2, r3=(r0 & 0xff).
  `bl` at 0xa358c calls (unknown callee) [0xa33d4] with r0=[sp-18]:1, r1=sp-19, r2=0x1, r3=[sp-18]:1.
  `bl` at 0xa359c calls (unknown callee) [0xa3210] with r0=0x0, r1=unknown, r2=unknown, r3=ret(0xa33d4). The ret expression comes from the call at 0xa358c.
  `beq` at 0xa3568 is taken when ret(0xa30d0) == 0x0 to 0xa3578. It falls through when ret(0xa30d0) != 0x0 to 0xa356c. The ret expression comes from the call at 0xa3554.
  The r0 value at the return (`ldmdb` at 0xa35ac) is unknown.

- `uartClose`, `patches/SG0005762/usr/cpu/firmware/sysco/l1.bin`, 0xa5388.
  Version 1.32.15, package `patches/SG0005762:l1#3`, inner `/stand/sysco/bin/l1`. It occupies 504 bytes.
  2 such word loads access r0 +0x34, including `ldr` at 0xa5400, `ldr` at 0xa5478.
  A word load accesses r0 +0x38 (`ldr` at 0xa543c).
  A word load accesses [sp-24] +0x38 (`ldr` at 0xa5490).
  2 such word stores write 0x0 to [sp-24] +0x34, including `str` at 0xa5420, `str` at 0xa54b4.
  2 such word stores write 0x0 to [sp-24] +0x38, including `str` at 0xa545c, `str` at 0xa54c0.
  A word load accesses an unknown effective address (`ldr` at 0xa54d0).
  A word load accesses an unknown effective address (`ldr` at 0xa54e0).
  A word load accesses an unknown effective address (`ldr` at 0xa54f0).
  2 such word loads access unknown effective addresses, including `ldr` at 0xa5500, `ldr` at 0xa5538.
  A word load accesses an unknown effective address (`ldr` at 0xa5560).
  2 such word stores write unknown values to [sp-24] +0x4, including `str` at 0xa5514, `str` at 0xa5570.
  2 such word stores write unknown values to [sp-24] +0x0, including `str` at 0xa551c, `str` at 0xa5548.
  `bl` at 0xa53f8 calls (unknown callee) [0x1e908] with r0=r0, r1=r1, r2=r2, r3=[sp-48].
  `bl` at 0xa5434 calls (unknown callee) [0x1e908] with r0=r0, r1=r1, r2=r2, r3=[sp-48].
  `bl` at 0xa5470 calls (unknown callee) [0x1e908] with r0=r0, r1=r1, r2=r2, r3=[sp-48].
  `bl` at 0xa5488 calls (unknown callee) [0x1e908] with r0=ret(0x1e908), r1=unknown, r2=ret(0x1e908), r3=[r0+52]. The ret expression comes from the call at 0xa5470.
  `bl` at 0xa5504 calls (unknown callee) [0xa5cf8] with r0=unknown, r1=unknown, r2=unknown, r3=unknown.
  `bl` at 0xa553c calls (unknown callee) [0x179d8] with r0=unknown, r1=unknown, r2=unknown, r3=unknown.
  `bl` at 0xa5564 calls (unknown callee) [0x179d8] with r0=unknown, r1=unknown, r2=unknown, r3=unknown.
  `beq` at 0xa53c4 is taken when [sp-48] == 0x1 to 0xa5430. It falls through when [sp-48] != 0x1 to 0xa53c8.
  `bgt` at 0xa53d0 is taken when [sp-48] > 0x1 (signed, 32-bit) to 0xa53e4. It falls through when [sp-48] <= 0x1 (signed, 32-bit) to 0xa53d4.
  `beq` at 0xa53dc is taken when [sp-48] == 0x0 to 0xa53f4. It falls through when [sp-48] != 0x0 to 0xa53e0.
  `beq` at 0xa53ec is taken when [sp-48] == 0x2 to 0xa546c. It falls through when [sp-48] != 0x2 to 0xa53f0.
  `beq` at 0xa5408 is taken when [r0+52] == ret(0x1e908) to 0xa5418. It falls through when [r0+52] != ret(0x1e908) to 0xa540c. The ret expression comes from the call at 0xa53f8.
  `beq` at 0xa5444 is taken when [r0+56] == ret(0x1e908) to 0xa5454. It falls through when [r0+56] != ret(0x1e908) to 0xa5448. The ret expression comes from the call at 0xa5434.
  `bne` at 0xa5480 is taken when [r0+52] != ret(0x1e908) to 0xa54a0. It falls through when [r0+52] == ret(0x1e908) to 0xa5484. The ret expression comes from the call at 0xa5470.
  `bne` at 0xa5498 is taken when [[sp-24]+56] != ret(0x1e908) to 0xa54a0. It falls through when [[sp-24]+56] == ret(0x1e908) to 0xa549c. The ret expression comes from the call at 0xa5488.
  `bne` at 0xa54d8 is taken when Z == 0 to 0xa5524. It falls through when not (Z == 0) to 0xa54dc.
  `bne` at 0xa54e8 is taken when Z == 0 to 0xa5574. It falls through when not (Z == 0) to 0xa54ec.
  `bne` at 0xa54f8 is taken when Z == 0 to 0xa5574. It falls through when not (Z == 0) to 0xa54fc.
  `beq` at 0xa5530 is taken when Z == 1 to 0xa554c. It falls through when not (Z == 1) to 0xa5534.
  `beq` at 0xa5558 is taken when Z == 1 to 0xa5574. It falls through when not (Z == 1) to 0xa555c.
  Returning with `ldmdb` at 0xa557c leaves r0 unknown.

- `uartWrite`, `patches/SG0005762/usr/cpu/firmware/sysco/l1.bin`, 0xa5740.
  Version 1.32.15, package `patches/SG0005762:l1#3`, inner `/stand/sysco/bin/l1`. It occupies 144 bytes.
  A word load accesses r0 +0x4 (`ldr` at 0xa5778).
  A word load accesses ret(0x17a8c) +0x0 (`ldr` at 0xa57b0). The ret expression comes from the call at 0xa57a8.
  `bl` at 0xa5784 calls (unknown callee) [0x17990] with r0=[r0+4], r1=r1, r2=r2, r3=r0.
  `bl` at 0xa57a4 calls (unknown callee) [0xa589c] with r0=[sp-20], r1=0x2, r2=unknown, r3=ret(0x17990). The ret expression comes from the call at 0xa5784.
  `bl` at 0xa57a8 calls (unknown callee) [0x17a8c] with r0=ret(0xa589c), r1=unknown, r2=unknown, r3=unknown. The ret expression comes from the call at 0xa57a4.
  `bgt` at 0xa5764 is taken when r2 > 0x0 (signed, 32-bit) to 0xa5774. It falls through when r2 <= 0x0 (signed, 32-bit) to 0xa5768.
  `bge` at 0xa5798 is taken when ret(0x17990) >= 0x0 (signed, 32-bit) to 0xa57c0. It falls through when ret(0x17990) < 0x0 (signed, 32-bit) to 0xa579c. The ret expression comes from the call at 0xa5784.
  The return instruction (`ldmdb` at 0xa57cc) has an unknown r0 value.

- `powerEnable`, `patches/SG0005762/usr/cpu/firmware/sysco/l1.bin`, 0x352ac.
  Version 1.32.15, package `patches/SG0005762:l1#3`, inner `/stand/sysco/bin/l1`. It occupies 140 bytes.
  4 word loads access PC-relative literal locations, including absolute 0x35328 (`ldr` at 0x352cc), absolute 0x3532c (`ldr` at 0x352e4), absolute 0x35330 (`ldr` at 0x352f8).
  A word load accesses absolute 0x10f980 (`ldr` at 0x352d4).
  A word load accesses absolute 0x10f960 (`ldr` at 0x35310).
  A byte store writes 0x1 to ([0x10f960] + r0) +0x0 (`strb` at 0x35320).
  `bl` at 0x35304 calls (unknown callee) [0x1e098] with r0=0x6, r1=0xd5b18, r2=0x14c, r3=0x0.
  `blt` at 0x352c8 is taken when r0 < 0x0 (signed, 32-bit) to 0x352e4. It falls through when r0 >= 0x0 (signed, 32-bit) to 0x352cc.
  `bge` at 0x352dc is taken when r0 >= [0x10f980] (signed, 32-bit) to 0x352e4. It falls through when r0 < [0x10f980] (signed, 32-bit) to 0x352e0.
  At the return (`ldmdb` at 0x35324), r0 is unknown.
  The instructions from 0x35328 to 0x35338 are not reached by any path from the entry, and their branches are not listed.

- `TIOSetLinkEnable`, `patches/SG0005762/usr/cpu/firmware/sysco/l1.bin`, 0x43ddc.
  Version 1.32.15, package `patches/SG0005762:l1#3`, inner `/stand/sysco/bin/l1`. It occupies 136 bytes.
  A word load accesses absolute 0x43e60 (`ldr` at 0x43e14).
  A byte load accesses absolute 0x10ba14 (`ldrb` at 0x43e18).
  `bl` at 0x43e30 calls (unknown callee) [0xba778] with r0=r0, r1=r1, r2=r2, r3=[0x10ba14]:1.
  `bl` at 0x43e40 calls (unknown callee) [0x43e64] with r0=[sp-20], r1=[sp-24], r2=[sp-28], r3=unknown.
  `bl` at 0x43e4c calls (unknown callee) [0xba7e4] with r0=ret(0x43e64), r1=unknown, r2=unknown, r3=ret(0x43e64). The ret expression comes from the call at 0x43e40.
  `beq` at 0x43e04 is taken when unknown == 0x4 to 0x43e14. It falls through when unknown != 0x4 to 0x43e08.
  `bne` at 0x43e20 is taken when [0x10ba14]:1 != 0x0 to 0x43e30. It falls through when [0x10ba14]:1 == 0x0 to 0x43e24.
  The r0 value at the return (`ldmdb` at 0x43e5c) is unknown.
  The instructions from 0x43e60 to 0x43e64 are not reached by any path from the entry, and their branches are not listed.

- `SHUB_ScanSetRackSlot`, `patches/SG0005762/usr/cpu/firmware/sysco/l1.bin`, 0x243b8.
  Version 1.32.15, package `patches/SG0005762:l1#3`, inner `/stand/sysco/bin/l1`. It occupies 416 bytes.
  11 word loads access PC-relative literal locations, including absolute 0x24534 (`ldr` at 0x243e4), absolute 0x24538 (`ldr` at 0x243f4), absolute 0x2453c (`ldr` at 0x24400).
  2 such word loads access absolute 0x10d3c0, including `ldr` at 0x243e8, `ldr` at 0x2447c.
  2 such word loads access unknown effective addresses, including `ldr` at 0x24414, `ldr` at 0x24428.
  A word load accesses an unknown effective address (`ldr` at 0x24438).
  A word load accesses an unknown effective address (`ldr` at 0x2445c).
  `bl` at 0x243d8 calls (unknown callee) [0x5aa74] with r0=0x0, r1=r1, r2=r2, r3=0x6c.
  `bl` at 0x2440c calls (unknown callee) [0x1e098] with r0=0x6, r1=0xd03d0, r2=0x1be, r3=0x0.
  `bl` at 0x244b0 calls (unknown callee) [0x1e098] with r0=0x6, r1=0xd03d0, r2=0x1c9, r3=0x0.
  `bl` at 0x244e8 calls (unknown callee) [0x23960] with r0=[sp-20], r1=0x101d0100, r2=[sp-28], r3=unknown.
  `bl` at 0x24520 calls (unknown callee) [0x1cd6c] with r0=0xd047c, r1=[sp-20], r2=unknown, r3=unknown.
  `beq` at 0x243f0 is taken when [0x10d3c0] == 0x0 to 0x24410. It falls through when [0x10d3c0] != 0x0 to 0x243f4.
  `beq` at 0x24484 is taken when [0x10d3c0] == 0x0 to 0x244b4. It falls through when [0x10d3c0] != 0x0 to 0x24488.
  `ble` at 0x244c4 is taken when le flags condition (operands unknown) to 0x244cc. It falls through when not (le flags condition (operands unknown)) to 0x244c8.
  `bne` at 0x244f4 is taken when ret(0x23960) != 0x1 to 0x24504. It falls through when ret(0x23960) == 0x1 to 0x244f8. The ret expression comes from the call at 0x244e8.
  Returning with `ldmdb` at 0x24530 leaves r0 unknown.
  The instructions from 0x24534 to 0x24554 are not reached by any path from the entry, and their branches are not listed.
  The bytes from 0x24554 to 0x24558 are undecodable.

- `i2cReadByte`, `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`, 0xbb53c.
  Version 1.48.1, package `patches/SG0007149:l1#4`, inner `/stand/sysco/bin/l1`. It occupies 136 bytes.
  `bl` at 0xbb568 calls (unknown callee) [0xbaf50] with r0=(r0 & 0xff), r1=r1, r2=r2, r3=(r0 & 0xff).
  `bl` at 0xbb59c calls (unknown callee) [0xbb358] with r0=[sp-18]:1, r1=[sp-24], r2=0x1, r3=[sp-18]:1.
  `bl` at 0xbb5ac calls (unknown callee) [0xbb094] with r0=0x0, r1=unknown, r2=unknown, r3=ret(0xbb358). The ret expression comes from the call at 0xbb59c.
  `beq` at 0xbb57c is taken when ret(0xbaf50) == 0x0 to 0xbb58c. It falls through when ret(0xbaf50) != 0x0 to 0xbb580. The ret expression comes from the call at 0xbb568.
  The r0 value at the return (`ldm` at 0xbb5c0) is unknown.

- `i2cWriteByte`, `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`, 0xbb408.
  Version 1.48.1, package `patches/SG0007149:l1#4`, inner `/stand/sysco/bin/l1`. It occupies 144 bytes.
  `bl` at 0xbb438 calls (unknown callee) [0xbaf50] with r0=(r0 & 0xff), r1=r1, r2=r2, r3=(r0 & 0xff).
  `bl` at 0xbb470 calls (unknown callee) [0xbb2a8] with r0=[sp-18]:1, r1=sp-19, r2=0x1, r3=[sp-18]:1.
  `bl` at 0xbb480 calls (unknown callee) [0xbb094] with r0=0x0, r1=unknown, r2=unknown, r3=ret(0xbb2a8). The ret expression comes from the call at 0xbb470.
  `beq` at 0xbb44c is taken when ret(0xbaf50) == 0x0 to 0xbb45c. It falls through when ret(0xbaf50) != 0x0 to 0xbb450. The ret expression comes from the call at 0xbb438.
  Returning with `ldm` at 0xbb494 leaves r0 unknown.

- `uartClose`, `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`, 0xbd60c.
  Version 1.48.1, package `patches/SG0007149:l1#4`, inner `/stand/sysco/bin/l1`. It occupies 508 bytes.
  2 such word loads access r0 +0x34, including `ldr` at 0xbd684, `ldr` at 0xbd6fc.
  2 such word stores access [sp-24] +0x34 with value 0x0, including `str` at 0xbd6a4, `str` at 0xbd738.
  A word load accesses r0 +0x38 (`ldr` at 0xbd6c0).
  2 such word stores access [sp-24] +0x38 with value 0x0, including `str` at 0xbd6e0, `str` at 0xbd744.
  A word load accesses [sp-24] +0x38 (`ldr` at 0xbd714).
  A word load accesses an unknown effective address (`ldr` at 0xbd754).
  A word load accesses an unknown effective address (`ldr` at 0xbd764).
  A word load accesses an unknown effective address (`ldr` at 0xbd774).
  2 such word loads access unknown effective addresses, including `ldr` at 0xbd784, `ldr` at 0xbd7bc.
  2 such word stores access [sp-24] +0x4 with value unknown, including `str` at 0xbd798, `str` at 0xbd7f4.
  2 such word stores access [sp-24] +0x0 with value unknown, including `str` at 0xbd7a0, `str` at 0xbd7cc.
  A word load accesses an unknown effective address (`ldr` at 0xbd7e4).
  `bl` at 0xbd67c calls (unknown callee) [0x2091c] with r0=r0, r1=r1, r2=r2, r3=[sp-48].
  `bl` at 0xbd6b8 calls (unknown callee) [0x2091c] with r0=r0, r1=r1, r2=r2, r3=[sp-48].
  `bl` at 0xbd6f4 calls (unknown callee) [0x2091c] with r0=r0, r1=r1, r2=r2, r3=[sp-48].
  `bl` at 0xbd70c calls (unknown callee) [0x2091c] with r0=ret(0x2091c), r1=unknown, r2=ret(0x2091c), r3=[r0+52]. The ret expression comes from the call at 0xbd6f4.
  `bl` at 0xbd788 calls (unknown callee) [0xbdf94] with r0=unknown, r1=unknown, r2=unknown, r3=unknown.
  `bl` at 0xbd7c0 calls (unknown callee) [0x1931c] with r0=unknown, r1=unknown, r2=unknown, r3=unknown.
  `bl` at 0xbd7e8 calls (unknown callee) [0x1931c] with r0=unknown, r1=unknown, r2=unknown, r3=unknown.
  `beq` at 0xbd648 is taken when [sp-48] == 0x1 to 0xbd6b4. It falls through when [sp-48] != 0x1 to 0xbd64c.
  `bgt` at 0xbd654 is taken when [sp-48] > 0x1 (signed, 32-bit) to 0xbd668. It falls through when [sp-48] <= 0x1 (signed, 32-bit) to 0xbd658.
  `beq` at 0xbd660 is taken when [sp-48] == 0x0 to 0xbd678. It falls through when [sp-48] != 0x0 to 0xbd664.
  `beq` at 0xbd670 is taken when [sp-48] == 0x2 to 0xbd6f0. It falls through when [sp-48] != 0x2 to 0xbd674.
  `beq` at 0xbd68c is taken when [r0+52] == ret(0x2091c) to 0xbd69c. It falls through when [r0+52] != ret(0x2091c) to 0xbd690. The ret expression comes from the call at 0xbd67c.
  `beq` at 0xbd6c8 is taken when [r0+56] == ret(0x2091c) to 0xbd6d8. It falls through when [r0+56] != ret(0x2091c) to 0xbd6cc. The ret expression comes from the call at 0xbd6b8.
  `bne` at 0xbd704 is taken when [r0+52] != ret(0x2091c) to 0xbd724. It falls through when [r0+52] == ret(0x2091c) to 0xbd708. The ret expression comes from the call at 0xbd6f4.
  `bne` at 0xbd71c is taken when [[sp-24]+56] != ret(0x2091c) to 0xbd724. It falls through when [[sp-24]+56] == ret(0x2091c) to 0xbd720. The ret expression comes from the call at 0xbd70c.
  `bne` at 0xbd75c is taken when Z == 0 to 0xbd7a8. It falls through when not (Z == 0) to 0xbd760.
  `bne` at 0xbd76c is taken when Z == 0 to 0xbd7f8. It falls through when not (Z == 0) to 0xbd770.
  `bne` at 0xbd77c is taken when Z == 0 to 0xbd7f8. It falls through when not (Z == 0) to 0xbd780.
  `beq` at 0xbd7b4 is taken when Z == 1 to 0xbd7d0. It falls through when not (Z == 1) to 0xbd7b8.
  `beq` at 0xbd7dc is taken when Z == 1 to 0xbd7f8. It falls through when not (Z == 1) to 0xbd7e0.
  The return leaves r0 unknown (`ldm` at 0xbd804).

- `uartWrite`, `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`, 0xbd9c0.
  Version 1.48.1, package `patches/SG0007149:l1#4`, inner `/stand/sysco/bin/l1`. It occupies 148 bytes.
  A word load accesses r0 +0x4 (`ldr` at 0xbd9f8).
  A word load accesses ret(0x193d0) +0x0 (`ldr` at 0xbda30). The ret expression comes from the call at 0xbda28.
  `bl` at 0xbda04 calls (unknown callee) [0x192e0] with r0=[r0+4], r1=r1, r2=r2, r3=r0.
  `bl` at 0xbda24 calls (unknown callee) [0xbdb28] with r0=[sp-20], r1=0x2, r2=unknown, r3=ret(0x192e0). The ret expression comes from the call at 0xbda04.
  `bl` at 0xbda28 calls (unknown callee) [0x193d0] with r0=ret(0xbdb28), r1=unknown, r2=unknown, r3=unknown. The ret expression comes from the call at 0xbda24.
  `bgt` at 0xbd9e4 is taken when r2 > 0x0 (signed, 32-bit) to 0xbd9f4. It falls through when r2 <= 0x0 (signed, 32-bit) to 0xbd9e8.
  `bge` at 0xbda18 is taken when ret(0x192e0) >= 0x0 (signed, 32-bit) to 0xbda40. It falls through when ret(0x192e0) < 0x0 (signed, 32-bit) to 0xbda1c. The ret expression comes from the call at 0xbda04.
  The return value in r0 is unknown (`ldm` at 0xbda50).

- `TIOSetLinkEnable`, `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`, 0x4ba7c.
  Version 1.48.1, package `patches/SG0007149:l1#4`, inner `/stand/sysco/bin/l1`. It occupies 136 bytes.
  A word load accesses absolute 0x4bb00 (`ldr` at 0x4bab8).
  A byte load accesses absolute 0x132494 (`ldrb` at 0x4babc).
  `bl` at 0x4bae0 calls (unknown callee) [0x4bb04] with r0=r0, r1=r1, r2=r2, r3=[0x132494]:1.
  `beq` at 0x4baa8 is taken when unknown == 0x4 to 0x4bab8. It falls through when unknown != 0x4 to 0x4baac.
  `bne` at 0x4bac4 is taken when [0x132494]:1 != 0x0 to 0x4bad4. It falls through when [0x132494]:1 == 0x0 to 0x4bac8.
  At the return (`ldm` at 0x4bafc), r0 is unknown.
  The instructions from 0x4bb00 to 0x4bb04 are not reached by any path from the entry, and their branches are not listed.
