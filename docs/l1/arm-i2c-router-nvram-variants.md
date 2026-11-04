# ARM L1 I²C, router and NVRAM variants

Each entry applies to its carrier, package and extracted ELF. ARM AAPCS is assumed. Argument counts and custom ABI details are unknown. The four printed register slots are candidate argument locations. Values printed as `unknown` remain unresolved. Symbol names do not establish a hardware role. Value roles, pointee meanings, C types and runtime path feasibility remain unknown.

In unreached-range statements, "any path from the entry" means any path in the recovered known-edge control-flow graph. Range ends are exclusive. Computed transfers with unknown targets remain unresolved. Repeated accesses count static records with the same width, access kind and resolved expression. Their order need not describe one execution path. The [complete facts](disasm/arm-variants/i2c-router-nvram-variants-facts.txt) retain every routine’s analysis.

Image `cd-ist-3.24:l1#4`, version 1.62.1: `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, inner `/stand/sysco/bin/l1`.

The package begins at carrier offset 0x7ecf04 and is 4948212 bytes. CramFS begins at package offset 0x1012fc. The ELF is 1999676 bytes, SHA-256 `22cb62871e447a394acbd662f4f49f3146580afec93839953b37a5354545bfc5`.

The executable PT_LOAD begins at ELF file offset 0x0, virtual address 0x8000, and is 1693992 bytes, SHA-256 `5ad8f4be9cbda073b307873683a90846744df308e9629d628f27b0b87af1e73c`. Carrier SHA-256 is `b4b5634949a93f05ad6eb24462e6cd7f10104dd42ff5c2ae8616dd5d87697790`. Package SHA-256 is `8c964136f81ec663d511b553041f07e798d9a30adebdd7b85fe420332e480f89`.

Image `cd-ist-3.24:l1#2`, version 1.62.1: `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, inner `/stand/sysco/bin/l1`.

The package begins at carrier offset 0x298558 and is 3817724 bytes. CramFS begins at package offset 0xbb728. The ELF is 1368336 bytes, SHA-256 `45ff1a458f755d99954be0d8f953933675796e4580a565a5c3360bb9ae4d1eed`.

The executable PT_LOAD begins at ELF file offset 0x0, virtual address 0x8000, and is 1169736 bytes, SHA-256 `51811253107f27d6a290f8c4579834de686b6af9c8d0849a9af9949e77d3d347`. Carrier SHA-256 is `b4b5634949a93f05ad6eb24462e6cd7f10104dd42ff5c2ae8616dd5d87697790`. Package SHA-256 is `c6f5f194b4ed6da5938c75ee40fab49e4ef6998428e6a687c50d12452d4d06ce`.

Image `patches/SG0007149:l1#4`, version 1.48.1: `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`, inner `/stand/sysco/bin/l1`.

The package begins at carrier offset 0x53d154 and is 3817724 bytes. CramFS begins at package offset 0xbb73c. The ELF is 1363924 bytes, SHA-256 `3eac4cd27d87d21313e0a8e06b6d719cb2ebc4cda81bd74536a04b8b4ad8cd12`.

The executable PT_LOAD begins at ELF file offset 0x0, virtual address 0x8000, and is 1166540 bytes, SHA-256 `9d5ddc793a620fe8f424df57ce1a727054526c98cfaf38ea1e80d69493b379fe`. Carrier SHA-256 is `536bfa4c6d0240b7afbc25bef94e160bdf1a675016c35e68ec52e1bfb1b451f9`. Package SHA-256 is `6bcee9bbd522c9bbd33538db5ea8c2c36666f7bb22c049a7912a72bf2c6c06e2`.

Image `patches/SG0005762:l1#3`, version 1.32.15: `patches/SG0005762/usr/cpu/firmware/sysco/l1.bin`, inner `/stand/sysco/bin/l1`.

The package begins at carrier offset 0x3ac27c and is 3555572 bytes. CramFS begins at package offset 0xb8c04. The ELF is 1182828 bytes, SHA-256 `7def5f2157e5bd37bd0c76685bd101eac73c15dcae304a8f9a900c2cf3594237`.

The executable PT_LOAD begins at ELF file offset 0x0, virtual address 0x8000, and is 1009192 bytes, SHA-256 `e116bdec7fd26f21465201d0c37a512858ce7aacea4c2174dee3dbc6fa71549a`. Carrier SHA-256 is `2399ea17dfeb2af20fae4286f0cfeeffae6d3d9f637fcc02e880e5689f662656`. Package SHA-256 is `67b4258a889facc68d9406dde68f145531d558d90da4eab5867eafa70b25212c`.

- `IRouterWritePacketReliable`, `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, 0x10b594.
  Version 1.62.1, package `cd-ist-3.24:l1#4`, inner `/stand/sysco/bin/l1`; occupies 484 bytes.
  A halfword load accesses an unknown effective address (`ldrh` at 0x10b5cc).
  A byte load accesses an unknown effective address (`ldrb` at 0x10b5d8).
  2 such byte loads access [sp-32] +0x16, including `ldrb` at 0x10b614, `ldrb` at 0x10b6c8.
  5 word loads read PC-relative literal locations, including absolute 0x10b76c (`ldr` at 0x10b628); absolute 0x10b770 (`ldr` at 0x10b62c); absolute 0x10b770 (`ldr` at 0x10b650).
  A word load accesses [sp-36] +0x0 (`ldr` at 0x10b744).
  `bl` at 0x10b5f8 calls (unknown callee) [0x20994] with r0=unknown, r1=unknown, r2=unknown, r3=unknown.
  `bl` at 0x10b638 calls (unknown callee) [0x10ac40] with r0=0x332c90, r1=0xfffffda4, r2=[sp-32], r3=0x0.
  `bl` at 0x10b64c calls (unknown callee) [0x1092f4] with r0=[sp-32], r1=unknown, r2=unknown, r3=[sp].
  `bl` at 0x10b670 calls (unknown callee) [0x20b98] with r0=ret(0x20994), r1=[sp-36], r2=[sp-40]+16, r3=[sp-40]+16; the printed ret expression comes from the call at 0x10b5f8.
  `bl` at 0x10b67c calls (unknown callee) [0x10b778] with r0=[sp-52], r1=0x1, r2=unknown, r3=unknown.
  `mov` at 0x10b69c calls (unknown callee) [unknown target] with r0=[sp-28], r1=[sp-52]+16, r2=[sp-40], r3=[sp-52]+16.
  `bl` at 0x10b6ec calls (unknown callee) [0x10ac40] with r0=0x332c90, r1=unknown, r2=[sp-32], r3=0x0.
  `bl` at 0x10b6f4 calls (unknown callee) [0x20cb8] with r0=[sp-52], r1=unknown, r2=unknown, r3=unknown.
  `bl` at 0x10b708 calls (unknown callee) [0x1092f4] with r0=[sp-32], r1=unknown, r2=unknown, r3=[sp].
  `bl` at 0x10b73c calls (unknown callee) [0x20cb8] with r0=[sp-52], r1=unknown, r2=ret(unknown), r3=[sp-40]; the printed ret expression comes from the call at 0x10b69c.
  `bl` at 0x10b754 calls (unknown callee) [0x1092f4] with r0=unknown, r1=unknown, r2=unknown, r3=unknown.
  `beq` at 0x10b5c4 is taken when Z == 1 to 0x10b750; it falls through when not (Z == 1) to 0x10b5c8.
  `bne` at 0x10b60c is taken when ret(0x20994) != 0x0 to 0x10b65c; it falls through when ret(0x20994) == 0x0 to 0x10b610; the printed ret expression comes from the call at 0x10b5f8.
  `bne` at 0x10b61c is taken when [[sp-32]+22]:1 != 0x0 to 0x10b63c; it falls through when [[sp-32]+22]:1 == 0x0 to 0x10b620.
  `beq` at 0x10b644 is taken when [sp] == 0x0 to 0x10b650; it falls through when [sp] != 0x0 to 0x10b648.
  `beq` at 0x10b6b4 is taken when ret(unknown) == [sp-40] to 0x10b738; it falls through when ret(unknown) != [sp-40] to 0x10b6b8; the printed ret expression comes from the call at 0x10b69c.
  `bge` at 0x10b6c0 is taken when ret(unknown) >= 0x0 (signed, 32-bit) to 0x10b6f0; it falls through when ret(unknown) < 0x0 (signed, 32-bit) to 0x10b6c4; the printed ret expression comes from the call at 0x10b69c.
  `bne` at 0x10b6d0 is taken when [[sp-32]+22]:1 != 0x0 to 0x10b6f0; it falls through when [[sp-32]+22]:1 == 0x0 to 0x10b6d4.
  `beq` at 0x10b700 is taken when [sp] == 0x0 to 0x10b70c; it falls through when [sp] != 0x0 to 0x10b704.
  `bne` at 0x10b714 is taken when [sp-48] != 0x0 to 0x10b724; it falls through when [sp-48] == 0x0 to 0x10b718.
  The return (`ldm` at 0x10b768) has r0 = unknown.
  The instructions from 0x10b76c to 0x10b770 are not reached by any path from the entry, and their branches are not listed.
  0x10b6e0 mvn: unsupported effects invalidated.
  0x10b724 mvn: unsupported effects invalidated.
  Undecodable bytes at 0x10b770-0x10b778; fall-through stops at this range.

- `nvramWrite`, `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, 0x1ef4c.
  Version 1.62.1, package `cd-ist-3.24:l1#2`, inner `/stand/sysco/bin/l1`; occupies 116 bytes.
  2 word loads read PC-relative literal locations, including absolute 0x1efb8 (`ldr` at 0x1ef74); absolute 0x1efbc (`ldr` at 0x1ef90).
  `bl` at 0x1efa4 calls (unknown callee) [0x1976c] with r0=unknown, r1=unknown, r2=unknown, r3=unknown.
  `ble` at 0x1ef7c is taken when (r0 + r2) <= 0x1fff (signed, 32-bit) to 0x1ef8c; it falls through when (r0 + r2) > 0x1fff (signed, 32-bit) to 0x1ef80.
  The return (`ldm` at 0x1efb4) has r0 = [sp-28].
  The instructions from 0x1efb8 to 0x1efc0 are not reached by any path from the entry, and their branches are not listed.
  0x1ef84 rsb: unsupported effects invalidated.

- `i2cReset`, `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, 0xbbd14.
  Version 1.62.1, package `cd-ist-3.24:l1#2`, inner `/stand/sysco/bin/l1`; occupies 136 bytes.
  4 word loads read PC-relative literal locations, including absolute 0xbbd8c (`ldr` at 0xbbd38); absolute 0xbbd90 (`ldr` at 0xbbd40); absolute 0xbbd94 (`ldr` at 0xbbd64).
  A word load accesses absolute 0x12e8e4 (`ldr` at 0xbbd3c).
  A word load accesses absolute 0x12fec0 (`ldr` at 0xbbd68).
  A word load accesses ret(0x1952c) +0x0 (`ldr` at 0xbbd70). Producer calls for these accesses, respectively: 0xbbd58.
  `bl` at 0xbbd48 calls (unknown callee) [0x19874] with r0=[0x12e8e4], r1=0x782, r2=0x0, r3=0x12e8e4.
  `bl` at 0xbbd58 calls (unknown callee) [0x1952c] with r0=ret(0x19874), r1=unknown, r2=unknown, r3=ret(0x19874); the printed ret expression comes from the call at 0xbbd48.
  `bl` at 0xbbd74 calls (unknown callee) [0x2244c] with r0=[0x12fec0], r1=0x11fa80, r2=[ret(0x1952c)], r3=0x12fec0; the printed ret expression comes from the call at 0xbbd58.
  `bgt` at 0xbbd34 is taken when gt flags condition (operands unknown) to 0xbbd88; it falls through when not (gt flags condition (operands unknown)) to 0xbbd38.
  `bge` at 0xbbd54 is taken when ret(0x19874) >= 0x0 (signed, 32-bit) to 0xbbd78; it falls through when ret(0x19874) < 0x0 (signed, 32-bit) to 0xbbd58; the printed ret expression comes from the call at 0xbbd48.
  The return (`ldm` at 0xbbd88) has r0 = unknown.
  The instructions from 0xbbd8c to 0xbbd9c are not reached by any path from the entry, and their branches are not listed.

- `i2cInit`, `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`, 0xbaaa0.
  Version 1.48.1, package `patches/SG0007149:l1#4`, inner `/stand/sysco/bin/l1`; occupies 472 bytes.
  16 word loads read PC-relative literal locations, including absolute 0xbac50 (`ldr` at 0xbaab4); absolute 0xbac54 (`ldr` at 0xbaac4); absolute 0xbac58 (`ldr` at 0xbaad0).
  2 such word loads access absolute 0x12d8dc, including `ldr` at 0xbaab8, `ldr` at 0xbabd4.
  3 such word loads access ret(0x193d0) +0x0, including `ldr` at 0xbab14, `ldr` at 0xbab84, `ldr` at 0xbabe8. Producer calls for these accesses, respectively: 0xbab00, 0xbab70, 0xbabe0.
  2 such word stores access absolute 0x29aeec, including `str` at 0xbab60, `str` at 0xbabb8.
  2 such word loads access absolute 0x29aeec, including `ldr` at 0xbab64, `ldr` at 0xbabb0.
  A word store accesses absolute 0x12d8dc (`str` at 0xbabd0).
  `bl` at 0xbaadc calls (unknown callee) [0x200a0] with r0=0x9, r1=0x11ec8c, r2=0x4d, r3=0x0.
  `bl` at 0xbaae8 calls (unknown callee) [0x1967c] with r0=0x11ec94, r1=0x1002, r2=unknown, r3=unknown.
  `bl` at 0xbab00 calls (unknown callee) [0x193d0] with r0=ret(0x1967c), r1=unknown, r2=unknown, r3=ret(0x1967c); symbolic call result comes from the call at 0xbaae8.
  `bl` at 0xbab2c calls (unknown callee) [0x200a0] with r0=0x9, r1=0x11ec8c, r2=0x53, r3=0x0.
  `bl` at 0xbab34 calls (unknown callee) [0x196b8] with r0=0x1, r1=unknown, r2=unknown, r3=unknown.
  `bl` at 0xbab5c calls (unknown callee) [0x193b8] with r0=0x0, r1=0x100, r2=0x3, r3=0x1.
  `bl` at 0xbab70 calls (unknown callee) [0x193d0] with r0=ret(0x193b8), r1=unknown, r2=unknown, r3=[0x29aeec]; symbolic call result comes from the call at 0xbab5c.
  `bl` at 0xbab9c calls (unknown callee) [0x200a0] with r0=0x9, r1=0x11ec8c, r2=0x5b, r3=0x0.
  `bl` at 0xbaba4 calls (unknown callee) [0x196b8] with r0=0x1, r1=unknown, r2=unknown, r3=unknown.
  `bl` at 0xbabc4 calls (unknown callee) [0x1967c] with r0=0x11ecec, r1=0x2, r2=0x29aeec, r3=[0x29aeec]+36.
  `bl` at 0xbabe0 calls (unknown callee) [0x193d0] with r0=ret(0x1967c), r1=unknown, r2=ret(0x1967c), r3=[0x12d8dc]; symbolic call result comes from the call at 0xbabc4.
  `bl` at 0xbabec calls (unknown callee) [0x1f8e0] with r0=[ret(0x193d0)], r1=unknown, r2=unknown, r3=ret(0x193d0); symbolic call result comes from the call at 0xbabe0.
  `bl` at 0xbac10 calls (unknown callee) [0x200a0] with r0=0x9, r1=0x11ec8c, r2=0x65, r3=0x0.
  `bl` at 0xbac30 calls (unknown callee) [0xc9238] with r0=unknown, r1=unknown, r2=unknown, r3=unknown.
  `bl` at 0xbac34 calls (unknown callee) [0xbb220] with r0=ret(0xc9238), r1=unknown, r2=unknown, r3=unknown; symbolic call result comes from the call at 0xbac30.
  `blt` at 0xbaac0 is taken when [0x12d8dc] < 0x0 (signed, 32-bit) to 0xbaae0; it falls through when [0x12d8dc] >= 0x0 (signed, 32-bit) to 0xbaac4.
  `bge` at 0xbaafc is taken when ret(0x1967c) >= 0x0 (signed, 32-bit) to 0xbab38; it falls through when ret(0x1967c) < 0x0 (signed, 32-bit) to 0xbab00. For the result of the call at 0xbaae8.
  `bne` at 0xbab6c is taken when Z == 0 to 0xbaba8; it falls through when not (Z == 0) to 0xbab70.
  `bge` at 0xbabdc is taken when [0x12d8dc] >= 0x0 (signed, 32-bit) to 0xbac14; it falls through when [0x12d8dc] < 0x0 (signed, 32-bit) to 0xbabe0.
  `bhi` at 0xbac24 is taken when hi flags condition (operands unknown) to 0xbac48; it falls through when not (hi flags condition (operands unknown)) to 0xbac28.
  The return (`ldm` at 0xbac4c) has r0 = unknown.
  The instructions from 0xbac50 to 0xbac78 are not reached by any path from the entry, and their branches are not listed.
  0xbab68 cmn: unsupported effects invalidated.

- `i2cRead`, `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`, 0xbb358.
  Version 1.48.1, package `patches/SG0007149:l1#4`, inner `/stand/sysco/bin/l1`; occupies 80 bytes.
  `bl` at 0xbb394 calls (unknown callee) [0xbb65c] with r0=0x1, r1=(r0 & 0xff), r2=unknown, r3=[sp-24].
  The return (`ldm` at 0xbb3a4) has r0 = ret(0xbb65c) from the call at 0xbb394.
  0xbb38c mvn: unsupported effects invalidated.

- `i2cWrite`, `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`, 0xbb2a8.
  Version 1.48.1, package `patches/SG0007149:l1#4`, inner `/stand/sysco/bin/l1`; occupies 80 bytes.
  `bl` at 0xbb2e4 calls (unknown callee) [0xbb65c] with r0=0x0, r1=(r0 & 0xff), r2=unknown, r3=[sp-24].
  The return (`ldm` at 0xbb2f4) has r0 = ret(0xbb65c) from the call at 0xbb2e4.
  0xbb2dc mvn: unsupported effects invalidated.

- `fanSetSpeed`, `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`, 0x40764.
  Version 1.48.1, package `patches/SG0007149:l1#4`, inner `/stand/sysco/bin/l1`; occupies 520 bytes.
  2 word loads read PC-relative literal locations, including absolute 0x40964 (`ldr` at 0x4079c); absolute 0x40968 (`ldr` at 0x40880).
  A word load accesses absolute 0x13220c (`ldr` at 0x407a0).
  A byte load accesses [sp-44] +0x4 (`ldrb` at 0x40804).
  A word load accesses [sp-44] +0x8 (`ldr` at 0x4081c).
  A byte load accesses an unknown effective address (`ldrb` at 0x40824).
  A byte load accesses ret(0x40d44) +0x0 (`ldrb` at 0x40848). Producer calls for these accesses, respectively: 0x40840.
  A byte load accesses ret(0x40e84) +0x3 (`ldrb` at 0x40864). Producer calls for these accesses, respectively: 0x40854.
  A byte load accesses ret(0x40e84) +0x7 (`ldrb` at 0x408d0). Producer calls for these accesses, respectively: 0x40854.
  A byte load accesses ret(0x40e84) +0x8 (`ldrb` at 0x408e0). Producer calls for these accesses, respectively: 0x40854.
  A byte load accesses ret(0x40e84) +0x9 (`ldrb` at 0x408f0). Producer calls for these accesses, respectively: 0x40854.
  `bl` at 0x407c4 calls (unknown callee) [0x41224] with r0=r0, r1=(r1 & 0xff), r2=sp-30, r3=sp-31.
  `bl` at 0x40840 calls (unknown callee) [0x40d44] with r0=[sp-20], r1=unknown, r2=unknown, r3=unknown.
  `bl` at 0x40854 calls (unknown callee) [0x40e84] with r0=[sp-20], r1=[ret(0x40d44)]:1, r2=unknown, r3=[ret(0x40d44)]:1; symbolic call result comes from the call at 0x40840.
  `bl` at 0x40914 calls (unknown callee) [0x404c4] with r0=[sp-20], r1=[sp-32]:1, r2=unknown, r3=[sp-32]:1.
  `beq` at 0x407d8 is taken when ret(0x41224) == 0x0 to 0x407e8; it falls through when ret(0x41224) != 0x0 to 0x407dc. For the result of the call at 0x407c4.
  `bhi` at 0x407fc is taken when hi flags condition (operands unknown) to 0x40950; it falls through when not (hi flags condition (operands unknown)) to 0x40800.
  `bls` at 0x40810 is taken when ls flags condition (operands unknown) to 0x40940; it falls through when not (ls flags condition (operands unknown)) to 0x40814.
  `bne` at 0x4082c is taken when Z == 0 to 0x40834; it falls through when not (Z == 0) to 0x40830.
  `bne` at 0x4086c is taken when [ret(0x40e84)+3]:1 != 0xff to 0x4088c; it falls through when [ret(0x40e84)+3]:1 == 0xff to 0x40870. For the result of the call at 0x40854.
  `bne` at 0x40878 is taken when [sp-21]:1 != 0xff to 0x40880; it falls through when [sp-21]:1 == 0xff to 0x4087c.
  `beq` at 0x4089c is taken when [sp-22]:1 == 0x1 to 0x408dc; it falls through when [sp-22]:1 != 0x1 to 0x408a0.
  `bgt` at 0x408a8 is taken when [sp-22]:1 > 0x1 (signed, 32-bit) to 0x408bc; it falls through when [sp-22]:1 <= 0x1 (signed, 32-bit) to 0x408ac.
  `beq` at 0x408b4 is taken when [sp-22]:1 == 0x0 to 0x408cc; it falls through when [sp-22]:1 != 0x0 to 0x408b8.
  `beq` at 0x408c4 is taken when [sp-22]:1 == 0x2 to 0x408ec; it falls through when [sp-22]:1 != 0x2 to 0x408c8.
  `beq` at 0x40928 is taken when ret(0x404c4) == 0x0 to 0x40940; it falls through when ret(0x404c4) != 0x0 to 0x4092c. For the result of the call at 0x40914.
  `bne` at 0x40934 is taken when [sp-36] != 0x0 to 0x40940; it falls through when [sp-36] == 0x0 to 0x40938.
  The return (`ldm` at 0x40960) has r0 = unknown.
  The instructions from 0x40964 to 0x40968 are not reached by any path from the entry, and their branches are not listed.
  Undecodable bytes at 0x40968-0x4096c; fall-through stops at this range.

- `powerReset`, `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`, 0x362f8.
  Version 1.48.1, package `patches/SG0007149:l1#4`, inner `/stand/sysco/bin/l1`; occupies 460 bytes.
  A word load reads a PC-relative literal location, at absolute 0x364c0 (`ldr` at 0x36320).
  A word load accesses absolute 0x13219c (`ldr` at 0x36324).
  3 such byte stores access [sp-24] +0xf, including `strb` at 0x36340, `strb` at 0x3643c, `strb` at 0x36460.
  A word load accesses [sp-24] +0x8 (`ldr` at 0x36350).
  A word load accesses [sp-24] +0x0 (`ldr` at 0x36378).
  A word load accesses [sp-24] +0x4 (`ldr` at 0x36388).
  A byte load accesses ([[sp-24]+4] + [sp-48]) +0x0 (`ldrb` at 0x36394).
  A byte load accesses [sp-28] +0x6 (`ldrb` at 0x363a4).
  2 such byte loads access [sp-24] +0xf, including `ldrsb` at 0x36428, `ldrsb` at 0x36448.
  `bl` at 0x363b8 calls (unknown callee) [0x3a950] with r0=[sp-20], r1=[sp-28], r2=[[sp-24]+4], r3=[[sp-28]+6]:1.
  `bl` at 0x363f8 calls (unknown callee) [0x3a8d8] with r0=[sp-20], r1=[sp-28], r2=unknown, r3=ret(0x3a950); symbolic call result comes from the call at 0x363b8.
  `bl` at 0x36418 calls (unknown callee) [0x39f34] with r0=[sp-20], r1=[sp-28], r2=unknown, r3=unknown.
  `bl` at 0x36474 calls (unknown callee) [0x3a430] with r0=[sp-20], r1=[sp-28], r2=[sp-29]:1, r3=[sp-29]:1.
  `ble` at 0x3635c is taken when le flags condition (operands unknown) to 0x364b0; it falls through when not (le flags condition (operands unknown)) to 0x36360.
  `beq` at 0x3639c is taken when [([[sp-24]+4] + [sp-48])]:1 == 0x0 to 0x364a0; it falls through when [([[sp-24]+4] + [sp-48])]:1 != 0x0 to 0x363a0.
  `beq` at 0x363ac is taken when [[sp-28]+6]:1 == 0xff to 0x364a0; it falls through when [[sp-28]+6]:1 != 0xff to 0x363b0.
  `beq` at 0x363d4 is taken when ret(0x3a950) == 0x0 to 0x36410; it falls through when ret(0x3a950) != 0x0 to 0x363d8. For the result of the call at 0x363b8.
  `blt` at 0x363e0 is taken when ret(0x3a950) < 0x0 (signed, 32-bit) to 0x36408; it falls through when ret(0x3a950) >= 0x0 (signed, 32-bit) to 0x363e4. For the result of the call at 0x363b8.
  `bgt` at 0x363ec is taken when ret(0x3a950) > 0x4 (signed, 32-bit) to 0x36408; it falls through when ret(0x3a950) <= 0x4 (signed, 32-bit) to 0x363f0. For the result of the call at 0x363b8.
  `bne` at 0x36430 is taken when Z == 0 to 0x36444; it falls through when not (Z == 0) to 0x36434.
  `beq` at 0x36454 is taken when unknown == [sp-36] to 0x36464; it falls through when unknown != [sp-36] to 0x36458.
  `beq` at 0x36488 is taken when ret(0x3a430) == 0x0 to 0x364a0; it falls through when ret(0x3a430) != 0x0 to 0x3648c. For the result of the call at 0x36474.
  `bne` at 0x36494 is taken when [sp-40] != 0x0 to 0x364a0; it falls through when [sp-40] == 0x0 to 0x36498.
  The return (`ldm` at 0x364bc) has r0 = [sp-40].
  The instructions from 0x364c0 to 0x364c4 are not reached by any path from the entry, and their branches are not listed.
  0x3633c mvn: unsupported effects invalidated.
  0x36370 rsb: unsupported effects invalidated.
  0x3642c cmn: unsupported effects invalidated.

- `nvramWrite`, `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`, 0x1edf0.
  Version 1.48.1, package `patches/SG0007149:l1#4`, inner `/stand/sysco/bin/l1`; occupies 116 bytes.
  2 word loads read PC-relative literal locations, including absolute 0x1ee5c (`ldr` at 0x1ee18); absolute 0x1ee60 (`ldr` at 0x1ee34).
  `bl` at 0x1ee48 calls (unknown callee) [0x19610] with r0=unknown, r1=unknown, r2=unknown, r3=unknown.
  `ble` at 0x1ee20 is taken when (r0 + r2) <= 0x1fff (signed, 32-bit) to 0x1ee30; it falls through when (r0 + r2) > 0x1fff (signed, 32-bit) to 0x1ee24.
  The return (`ldm` at 0x1ee58) has r0 = [sp-28].
  The instructions from 0x1ee5c to 0x1ee64 are not reached by any path from the entry, and their branches are not listed.
  0x1ee28 rsb: unsupported effects invalidated.

- `i2cInit`, `patches/SG0005762/usr/cpu/firmware/sysco/l1.bin`, 0xa2cbc.
  Version 1.32.15, package `patches/SG0005762:l1#3`, inner `/stand/sysco/bin/l1`; occupies 420 bytes.
  16 word loads read PC-relative literal locations, including absolute 0xa2e38 (`ldr` at 0xa2cd0); absolute 0xa2e3c (`ldr` at 0xa2ce0); absolute 0xa2e40 (`ldr` at 0xa2cec).
  2 such word loads access absolute 0x107318, including `ldr` at 0xa2cd4, `ldr` at 0xa2df0.
  3 such word loads access ret(0x17a8c) +0x0, including `ldr` at 0xa2d30, `ldr` at 0xa2da0, `ldr` at 0xa2e04. Producer calls for these accesses, respectively: 0xa2d1c, 0xa2d8c, 0xa2dfc.
  2 such word stores access absolute 0x111240, including `str` at 0xa2d7c, `str` at 0xa2dd4.
  2 such word loads access absolute 0x111240, including `ldr` at 0xa2d80, `ldr` at 0xa2dcc.
  A word store accesses absolute 0x107318 (`str` at 0xa2dec).
  `bl` at 0xa2cf8 calls (unknown callee) [0x1e098] with r0=0x9, r1=0xf8cdc, r2=0x4a, r3=0x0.
  `bl` at 0xa2d04 calls (unknown callee) [0x17d08] with r0=0xf8ce4, r1=0x1002, r2=unknown, r3=unknown.
  `bl` at 0xa2d1c calls (unknown callee) [0x17a8c] with r0=ret(0x17d08), r1=unknown, r2=unknown, r3=ret(0x17d08). Result provenance: 0xa2d1c uses the result from call 0xa2d04.
  `bl` at 0xa2d48 calls (unknown callee) [0x1e098] with r0=0x9, r1=0xf8cdc, r2=0x50, r3=0x0.
  `bl` at 0xa2d50 calls (unknown callee) [0x17d2c] with r0=0x1, r1=unknown, r2=unknown, r3=unknown.
  `bl` at 0xa2d78 calls (unknown callee) [0x17a74] with r0=0x0, r1=0x100, r2=0x3, r3=0x1.
  `bl` at 0xa2d8c calls (unknown callee) [0x17a8c] with r0=ret(0x17a74), r1=unknown, r2=unknown, r3=[0x111240]. Result provenance: 0xa2d8c uses the result from call 0xa2d78.
  `bl` at 0xa2db8 calls (unknown callee) [0x1e098] with r0=0x9, r1=0xf8cdc, r2=0x58, r3=0x0.
  `bl` at 0xa2dc0 calls (unknown callee) [0x17d2c] with r0=0x1, r1=unknown, r2=unknown, r3=unknown.
  `bl` at 0xa2de0 calls (unknown callee) [0x17d08] with r0=0xf8d3c, r1=0x2, r2=0x111240, r3=[0x111240]+36.
  `bl` at 0xa2dfc calls (unknown callee) [0x17a8c] with r0=ret(0x17d08), r1=unknown, r2=ret(0x17d08), r3=[0x107318]. Result provenance: 0xa2dfc uses the result from call 0xa2de0.
  `bl` at 0xa2e08 calls (unknown callee) [0x1d8e4] with r0=[ret(0x17a8c)], r1=unknown, r2=unknown, r3=ret(0x17a8c). Result provenance: 0xa2e08 uses the result from call 0xa2dfc.
  `bl` at 0xa2e2c calls (unknown callee) [0x1e098] with r0=0x9, r1=0xf8cdc, r2=0x62, r3=0x0.
  `bl` at 0xa2e30 calls (unknown callee) [0xa3390] with r0=unknown, r1=unknown, r2=unknown, r3=unknown.
  `blt` at 0xa2cdc is taken when [0x107318] < 0x0 (signed, 32-bit) to 0xa2cfc; it falls through when [0x107318] >= 0x0 (signed, 32-bit) to 0xa2ce0.
  `bge` at 0xa2d18 is taken when ret(0x17d08) >= 0x0 (signed, 32-bit) to 0xa2d54; it falls through when ret(0x17d08) < 0x0 (signed, 32-bit) to 0xa2d1c. Result provenance: 0xa2d18 uses the result from call 0xa2d04.
  `bne` at 0xa2d88 is taken when Z == 0 to 0xa2dc4; it falls through when not (Z == 0) to 0xa2d8c.
  `bge` at 0xa2df8 is taken when [0x107318] >= 0x0 (signed, 32-bit) to 0xa2e30; it falls through when [0x107318] < 0x0 (signed, 32-bit) to 0xa2dfc.
  The return (`ldmdb` at 0xa2e34) has r0 = ret(0xa3390). Result provenance: 0xa2e34 uses the result from call 0xa2e30.
  The instructions from 0xa2e38 to 0xa2e60 are not reached by any path from the entry, and their branches are not listed.
  0xa2d84 cmn: unsupported effects invalidated.

- `i2cRead`, `patches/SG0005762/usr/cpu/firmware/sysco/l1.bin`, 0xa347c.
  Version 1.32.15, package `patches/SG0005762:l1#3`, inner `/stand/sysco/bin/l1`; occupies 76 bytes.
  `bl` at 0xa34b8 calls (unknown callee) [0xa3768] with r0=0x1, r1=(r0 & 0xff), r2=unknown, r3=[sp-24].
  The return (`ldmdb` at 0xa34c4) has r0 = ret(0xa3768). Result provenance: 0xa34c4 uses the result from call 0xa34b8.
  0xa34b0 mvn: unsupported effects invalidated.

- `i2cWrite`, `patches/SG0005762/usr/cpu/firmware/sysco/l1.bin`, 0xa33d4.
  Version 1.32.15, package `patches/SG0005762:l1#3`, inner `/stand/sysco/bin/l1`; occupies 76 bytes.
  `bl` at 0xa3410 calls (unknown callee) [0xa3768] with r0=0x0, r1=(r0 & 0xff), r2=unknown, r3=[sp-24].
  The return (`ldmdb` at 0xa341c) has r0 = ret(0xa3768). Result provenance: 0xa341c uses the result from call 0xa3410.
  0xa3408 mvn: unsupported effects invalidated.

- `fanSetSpeed`, `patches/SG0005762/usr/cpu/firmware/sysco/l1.bin`, 0x3e1c4.
  Version 1.32.15, package `patches/SG0005762:l1#3`, inner `/stand/sysco/bin/l1`; occupies 476 bytes.
  3 word loads read PC-relative literal locations, including absolute 0x3e394 (`ldr` at 0x3e244); absolute 0x3e398 (`ldr` at 0x3e260); absolute 0x3e39c (`ldr` at 0x3e2b4).
  A word load accesses absolute 0x10ba04 (`ldr` at 0x3e248).
  A byte load accesses an unknown effective address (`ldrb` at 0x3e250).
  A word load accesses absolute 0x10b9f8 (`ldr` at 0x3e264).
  A byte load accesses ret(0x3e720) +0x0 (`ldrb` at 0x3e280). Producer calls for these accesses, respectively: 0x3e278.
  A byte load accesses ret(0x3e7b4) +0x3 (`ldrb` at 0x3e298). Producer calls for these accesses, respectively: 0x3e288.
  A byte load accesses ret(0x3e7b4) +0x7 (`ldrb` at 0x3e304). Producer calls for these accesses, respectively: 0x3e288.
  A byte load accesses ret(0x3e7b4) +0x8 (`ldrb` at 0x3e314). Producer calls for these accesses, respectively: 0x3e288.
  A byte load accesses ret(0x3e7b4) +0x9 (`ldrb` at 0x3e324). Producer calls for these accesses, respectively: 0x3e288.
  `bl` at 0x3e200 calls (unknown callee) [0x3e84c] with r0=(r0 & 0xff), r1=sp-26, r2=sp-27, r3=(r0 & 0xff).
  `bl` at 0x3e278 calls (unknown callee) [0x3e720] with r0=unknown, r1=unknown, r2=[0x10b9f8], r3=unknown.
  `bl` at 0x3e288 calls (unknown callee) [0x3e7b4] with r0=[ret(0x3e720)]:1, r1=unknown, r2=unknown, r3=[ret(0x3e720)]:1. Result provenance: 0x3e288 uses the result from call 0x3e278.
  `bl` at 0x3e348 calls (unknown callee) [0x3df5c] with r0=[sp-28]:1, r1=unknown, r2=unknown, r3=[sp-28]:1.
  `beq` at 0x3e214 is taken when ret(0x3e84c) == 0x0 to 0x3e224; it falls through when ret(0x3e84c) != 0x0 to 0x3e218. Result provenance: 0x3e214 uses the result from call 0x3e200.
  `bls` at 0x3e238 is taken when ls flags condition (operands unknown) to 0x3e240; it falls through when not (ls flags condition (operands unknown)) to 0x3e23c.
  `beq` at 0x3e258 is taken when Z == 1 to 0x3e374; it falls through when not (Z == 1) to 0x3e25c.
  `bge` at 0x3e26c is taken when ge flags condition (operands unknown) to 0x3e374; it falls through when not (ge flags condition (operands unknown)) to 0x3e270.
  `bne` at 0x3e2a0 is taken when [ret(0x3e7b4)+3]:1 != 0xff to 0x3e2c0; it falls through when [ret(0x3e7b4)+3]:1 == 0xff to 0x3e2a4. Result provenance: 0x3e2a0 uses the result from call 0x3e288.
  `bne` at 0x3e2ac is taken when [sp-17]:1 != 0xff to 0x3e2b4; it falls through when [sp-17]:1 == 0xff to 0x3e2b0.
  `beq` at 0x3e2d0 is taken when [sp-18]:1 == 0x1 to 0x3e310; it falls through when [sp-18]:1 != 0x1 to 0x3e2d4.
  `bgt` at 0x3e2dc is taken when [sp-18]:1 > 0x1 (signed, 32-bit) to 0x3e2f0; it falls through when [sp-18]:1 <= 0x1 (signed, 32-bit) to 0x3e2e0.
  `beq` at 0x3e2e8 is taken when [sp-18]:1 == 0x0 to 0x3e300; it falls through when [sp-18]:1 != 0x0 to 0x3e2ec.
  `beq` at 0x3e2f8 is taken when [sp-18]:1 == 0x2 to 0x3e320; it falls through when [sp-18]:1 != 0x2 to 0x3e2fc.
  `beq` at 0x3e35c is taken when ret(0x3df5c) == 0x0 to 0x3e374; it falls through when ret(0x3df5c) != 0x0 to 0x3e360. Result provenance: 0x3e35c uses the result from call 0x3e348.
  `bne` at 0x3e368 is taken when [sp-32] != 0x0 to 0x3e374; it falls through when [sp-32] == 0x0 to 0x3e36c.
  The return (`ldmdb` at 0x3e390) has r0 = unknown.
  The instructions from 0x3e394 to 0x3e39c are not reached by any path from the entry, and their branches are not listed.
  Undecodable bytes at 0x3e39c-0x3e3a0; fall-through stops at this range.

- `powerReset`, `patches/SG0005762/usr/cpu/firmware/sysco/l1.bin`, 0x35338.
  Version 1.32.15, package `patches/SG0005762:l1#3`, inner `/stand/sysco/bin/l1`; occupies 436 bytes.
  8 word loads read PC-relative literal locations, including absolute 0x354dc (`ldr` at 0x35350); absolute 0x354e0 (`ldr` at 0x35364); absolute 0x354e4 (`ldr` at 0x3537c).
  3 such word stores access absolute 0x107210, including `str` at 0x35358, `str` at 0x35460, `str` at 0x35484.
  A word load accesses absolute 0x10f980 (`ldr` at 0x3536c).
  A word load accesses absolute 0x10f9a0 (`ldr` at 0x35394).
  A word load accesses absolute 0x10f960 (`ldr` at 0x353a4).
  A byte load accesses an unknown effective address (`ldrb` at 0x353b0).
  A byte load accesses an unknown effective address (`ldrb` at 0x353c0).
  A byte load accesses an unknown effective address (`ldrb` at 0x353d0).
  A byte load accesses [sp-20] +0x10 (`ldrb` at 0x35414).
  2 such word loads access absolute 0x107210, including `ldr` at 0x3544c, `ldr` at 0x3546c.
  `bl` at 0x353dc calls (unknown callee) [0x1b944] with r0=unknown, r1=0x10f9a0, r2=[0x10f960], r3=unknown.
  `bl` at 0x35420 calls (unknown callee) [0x1b944] with r0=[[sp-20]+16]:1+130, r1=unknown, r2=unknown, r3=[[sp-20]+16]:1+130.
  `bl` at 0x3543c calls (unknown callee) [0x39404] with r0=[sp-20], r1=unknown, r2=unknown, r3=unknown.
  `bl` at 0x35494 calls (unknown callee) [0x399fc] with r0=[sp-20], r1=[sp-21]:1, r2=unknown, r3=[sp-21]:1.
  `blt` at 0x35374 is taken when lt flags condition (operands unknown) to 0x3537c; it falls through when not (lt flags condition (operands unknown)) to 0x35378.
  `beq` at 0x353b8 is taken when Z == 1 to 0x354c0; it falls through when not (Z == 1) to 0x353bc.
  `beq` at 0x353c8 is taken when Z == 1 to 0x354c0; it falls through when not (Z == 1) to 0x353cc.
  `beq` at 0x353f4 is taken when ret(0x1b944) == 0x0 to 0x35438; it falls through when ret(0x1b944) != 0x0 to 0x353f8. Result provenance: 0x353f4 uses the result from call 0x353dc.
  `blt` at 0x35400 is taken when ret(0x1b944) < 0x0 (signed, 32-bit) to 0x35430; it falls through when ret(0x1b944) >= 0x0 (signed, 32-bit) to 0x35404. Result provenance: 0x35400 uses the result from call 0x353dc.
  `bgt` at 0x3540c is taken when ret(0x1b944) > 0x4 (signed, 32-bit) to 0x35430; it falls through when ret(0x1b944) <= 0x4 (signed, 32-bit) to 0x35410. Result provenance: 0x3540c uses the result from call 0x353dc.
  `bne` at 0x35454 is taken when Z == 0 to 0x35468; it falls through when not (Z == 0) to 0x35458.
  `beq` at 0x35478 is taken when [0x107210] == [sp-28] to 0x35488; it falls through when [0x107210] != [sp-28] to 0x3547c.
  `beq` at 0x354a8 is taken when ret(0x399fc) == 0x0 to 0x354c0; it falls through when ret(0x399fc) != 0x0 to 0x354ac. Result provenance: 0x354a8 uses the result from call 0x35494.
  `bne` at 0x354b4 is taken when [sp-32] != 0x0 to 0x354c0; it falls through when [sp-32] == 0x0 to 0x354b8.
  The return (`ldmdb` at 0x354d8) has r0 = [sp-32].
  The instructions from 0x354dc to 0x354ec are not reached by any path from the entry, and their branches are not listed.
  0x35354 mvn: unsupported effects invalidated.
  0x35450 cmn: unsupported effects invalidated.

- `i2c_int_init`, `patches/SG0005762/usr/cpu/firmware/sysco/l1.bin`, 0xa3f94.
  Version 1.32.15, package `patches/SG0005762:l1#3`, inner `/stand/sysco/bin/l1`; occupies 224 bytes.
  A word store accesses r0 +0x4 (`str` at 0xa3fbc).
  A word store accesses [sp-20] +0x8 (`str` at 0xa3fc8).
  A word store accesses [sp-20] +0xc (`str` at 0xa3fd4).
  5 word loads read PC-relative literal locations, including absolute 0xa4064 (`ldr` at 0xa3fe0); absolute 0xa4068 (`ldr` at 0xa3ff8); absolute 0xa406c (`ldr` at 0xa4040).
  A byte load accesses absolute 0x111398 (`ldrb` at 0xa4054).
  A byte store accesses absolute 0x111398 (`strb` at 0xa405c).
  `bl` at 0xa3fd8 calls (unknown callee) [0xa45fc] with r0=r0, r1=r1, r2=[sp-20], r3=[sp-28].
  `bl` at 0xa4048 calls (unknown callee) [0x1e660] with r0=[sp-20]+16, r1=[sp], r2=0xa4074, r3=0x1.
  `bls` at 0xa3fe8 is taken when ret(0xa45fc) <= 0x7ffff (unsigned, 32-bit) to 0xa3ff8; it falls through when ret(0xa45fc) > 0x7ffff (unsigned, 32-bit) to 0xa3fec. Result provenance: 0xa3fe8 uses the result from call 0xa3fd8.
  The return (`ldmdb` at 0xa4060) has r0 = ret(0x1e660). Result provenance: 0xa4060 uses the result from call 0xa4048.
  The instructions from 0xa4068 to 0xa4074 are not reached by any path from the entry, and their branches are not listed.
  Undecodable bytes at 0xa4064-0xa4068; fall-through stops at this range.
