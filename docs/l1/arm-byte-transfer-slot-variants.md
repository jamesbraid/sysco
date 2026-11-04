# ARM L1 byte-transfer and slot variants

Each entry applies to its carrier, package and extracted ELF. ARM AAPCS is assumed. Argument counts and custom ABI details are unknown. The four printed register slots are candidate argument locations. Values printed as `unknown` remain unresolved. Symbol names do not establish a hardware role. Value roles, pointee meanings, C types and runtime path feasibility remain unknown.

In unreached-range statements, "any path from the entry" means any path in the recovered known-edge control-flow graph. Range ends are exclusive. Computed transfers with unknown targets remain unresolved. Repeated accesses count static records with the same width, access kind and resolved expression. Their order need not describe one execution path. The [complete facts](disasm/arm-variants/byte-transfer-slot-variants-facts.txt) retain every routine’s analysis.

Image `patches/SG0007149:l1#6`, version 1.48.1: `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`, inner `/stand/sysco/bin/l1`.

The package begins at carrier offset 0xa90dfc and is 4948212 bytes. CramFS begins at package offset 0x101304. The ELF is 1976140 bytes, SHA-256 `61274890f8ba1708b1380a8716e90060d48c22b2505eebf27255744123cdf96e`.

The executable PT_LOAD begins at ELF file offset 0x0, virtual address 0x8000, and is 1674816 bytes, SHA-256 `081529c2c1a72bbb7e930f5bd6c11cf0bc6b23feb32dff48fcf2335845126629`. Carrier SHA-256 is `536bfa4c6d0240b7afbc25bef94e160bdf1a675016c35e68ec52e1bfb1b451f9`. Package SHA-256 is `333891e54763faf27b63031875afc189dcf6adda6e6999c6ad87f074a297e6a5`.

Image `cd-ist-3.24:l1#2`, version 1.62.1: `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, inner `/stand/sysco/bin/l1`.

The package begins at carrier offset 0x298558 and is 3817724 bytes. CramFS begins at package offset 0xbb728. The ELF is 1368336 bytes, SHA-256 `45ff1a458f755d99954be0d8f953933675796e4580a565a5c3360bb9ae4d1eed`.

The executable PT_LOAD begins at ELF file offset 0x0, virtual address 0x8000, and is 1169736 bytes, SHA-256 `51811253107f27d6a290f8c4579834de686b6af9c8d0849a9af9949e77d3d347`. Carrier SHA-256 is `b4b5634949a93f05ad6eb24462e6cd7f10104dd42ff5c2ae8616dd5d87697790`. Package SHA-256 is `c6f5f194b4ed6da5938c75ee40fab49e4ef6998428e6a687c50d12452d4d06ce`.

Image `cd-ist-3.24:l1#4`, version 1.62.1: `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, inner `/stand/sysco/bin/l1`.

The package begins at carrier offset 0x7ecf04 and is 4948212 bytes. CramFS begins at package offset 0x1012fc. The ELF is 1999676 bytes, SHA-256 `22cb62871e447a394acbd662f4f49f3146580afec93839953b37a5354545bfc5`.

The executable PT_LOAD begins at ELF file offset 0x0, virtual address 0x8000, and is 1693992 bytes, SHA-256 `5ad8f4be9cbda073b307873683a90846744df308e9629d628f27b0b87af1e73c`. Carrier SHA-256 is `b4b5634949a93f05ad6eb24462e6cd7f10104dd42ff5c2ae8616dd5d87697790`. Package SHA-256 is `8c964136f81ec663d511b553041f07e798d9a30adebdd7b85fe420332e480f89`.

- `i2cReset`, `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`, 0x11330c.
  Version 1.48.1, package `patches/SG0007149:l1#6`, inner `/stand/sysco/bin/l1`; occupies 136 bytes.
  4 word loads read PC-relative literal locations, including absolute 0x113384 (`ldr` at 0x113330); absolute 0x113388 (`ldr` at 0x113338); absolute 0x11338c (`ldr` at 0x11335c).
  A word load accesses absolute 0x1ab708 (`ldr` at 0x113334).
  A word load accesses absolute 0x1adad8 (`ldr` at 0x113360).
  A word load accesses ret(0x206d0) +0x0 (`ldr` at 0x113368). The ret expression comes from the call at 0x113350.
  `bl` at 0x113340 calls (unknown callee) [0x20a90] with r0=[0x1ab708], r1=0x782, r2=0x0, r3=0x1ab708.
  `bl` at 0x113350 calls (unknown callee) [0x206d0] with r0=ret(0x20a90), r1=unknown, r2=unknown, r3=ret(0x20a90). The ret expression comes from the call at 0x113340.
  `bl` at 0x11336c calls (unknown callee) [0x24bc8] with r0=[0x1adad8], r1=0x19784c, r2=[ret(0x206d0)], r3=0x1adad8. The ret expression comes from the call at 0x113350.
  `bgt` at 0x11332c is taken when gt flags condition (operands unknown) to 0x113380; it falls through when not (gt flags condition (operands unknown)) to 0x113330.
  `bge` at 0x11334c is taken when ret(0x20a90) >= 0x0 (signed, 32-bit) to 0x113370; it falls through when ret(0x20a90) < 0x0 (signed, 32-bit) to 0x113350. The ret expression comes from the call at 0x113340.
  The return (`ldm` at 0x113380) has r0 = unknown.
  The instructions from 0x113384 to 0x113394 are not reached by any path from the entry, and their branches are not listed.

- `i2cReadByte`, `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`, 0x113628.
  Version 1.48.1, package `patches/SG0007149:l1#6`, inner `/stand/sysco/bin/l1`; occupies 136 bytes.
  `bl` at 0x113654 calls (unknown callee) [0x11303c] with r0=(r0 & 0xff), r1=r1, r2=r2, r3=(r0 & 0xff).
  `bl` at 0x113688 calls (unknown callee) [0x113444] with r0=[sp-18]:1, r1=[sp-24], r2=0x1, r3=[sp-18]:1.
  `bl` at 0x113698 calls (unknown callee) [0x113180] with r0=0x0, r1=unknown, r2=unknown, r3=ret(0x113444). The ret expression comes from the call at 0x113688.
  `beq` at 0x113668 is taken when ret(0x11303c) == 0x0 to 0x113678; it falls through when ret(0x11303c) != 0x0 to 0x11366c. The ret expression comes from the call at 0x113654.
  At the return (`ldm` at 0x1136ac), r0 is unknown.

- `i2cWriteByte`, `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`, 0x1134f4.
  Version 1.48.1, package `patches/SG0007149:l1#6`, inner `/stand/sysco/bin/l1`; occupies 144 bytes.
  `bl` at 0x113524 calls (unknown callee) [0x11303c] with r0=(r0 & 0xff), r1=r1, r2=r2, r3=(r0 & 0xff).
  `bl` at 0x11355c calls (unknown callee) [0x113394] with r0=[sp-18]:1, r1=sp-19, r2=0x1, r3=[sp-18]:1.
  `bl` at 0x11356c calls (unknown callee) [0x113180] with r0=0x0, r1=unknown, r2=unknown, r3=ret(0x113394). The ret expression comes from the call at 0x11355c.
  `beq` at 0x113538 is taken when ret(0x11303c) == 0x0 to 0x113548; it falls through when ret(0x11303c) != 0x0 to 0x11353c. The ret expression comes from the call at 0x113524.
  The r0 value at the return (`ldm` at 0x113580) is unknown.

- `i2cBus`, `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`, 0x112cac.
  Version 1.48.1, package `patches/SG0007149:l1#6`, inner `/stand/sysco/bin/l1`; occupies 188 bytes.
  4 word loads read PC-relative literal locations, including absolute 0x112d5c (`ldr` at 0x112ce8); absolute 0x112d60 (`ldr` at 0x112cf8); absolute 0x112d64 (`ldr` at 0x112d04).
  2 such word loads access absolute 0x325848, including `ldr` at 0x112cec, `ldr` at 0x112d30.
  A word load has base r3 and an unknown effective address (`ldr` at 0x112d38).
  `bl` at 0x112cc8 calls (unknown callee) [0x32824] with r0=r0, r1=r1, r2=r2, r3=r1.
  `bl` at 0x112d10 calls (unknown callee) [0x22764] with r0=0x9, r1=0x197688, r2=0x94, r3=0x0.
  `bne` at 0x112cd8 is taken when ret(0x32824) != [sp-20] to 0x112ce8; it falls through when ret(0x32824) == [sp-20] to 0x112cdc. The ret expression comes from the call at 0x112cc8.
  `bne` at 0x112cf4 is taken when [0x325848] != 0x0 to 0x112d14; it falls through when [0x325848] == 0x0 to 0x112cf8.
  Returning with `ldm` at 0x112d58 leaves r0 unknown.
  The instructions from 0x112d5c to 0x112d68 are not reached by any path from the entry, and their branches are not listed.
  The analysis invalidates unsupported effects for `mvn` at 0x112d14.

- `powerUpSlot`, `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`, 0x6211c.
  Version 1.48.1, package `patches/SG0007149:l1#6`, inner `/stand/sysco/bin/l1`; occupies 224 bytes.
  3 word loads read PC-relative literal locations, including absolute 0x621f0 (`ldr` at 0x62174); absolute 0x621f4 (`ldr` at 0x62194); absolute 0x621f8 (`ldr` at 0x621b4).
  `bl` at 0x6213c calls (unknown callee) [0x4934c] with r0=r0, r1=r1, r2=r2, r3=r3.
  `bl` at 0x62150 calls (unknown callee) [0x4934c] with r0=[sp-20], r1=unknown, r2=unknown, r3=ret(0x4934c). The ret expression comes from the call at 0x6213c.
  `bl` at 0x62164 calls (unknown callee) [0x4934c] with r0=[sp-20], r1=unknown, r2=unknown, r3=ret(0x4934c). The ret expression comes from the call at 0x6213c.
  `bl` at 0x62184 calls (unknown callee) [0x4a260] with r0=[sp-20], r1=unknown, r2=unknown, r3=ret(0x4934c). The ret expression comes from the call at 0x62164.
  `bl` at 0x621a4 calls (unknown callee) [0x49a58] with r0=[sp-20], r1=unknown, r2=unknown, r3=ret(0x4a260). The ret expression comes from the call at 0x62184.
  `bl` at 0x621d8 calls (unknown callee) [0x61b60] with r0=unknown, r1=[sp-24], r2=[sp-28], r3=unknown.
  `bge` at 0x62148 is taken when ret(0x4934c) >= 0x0 (signed, 32-bit) to 0x62160; it falls through when ret(0x4934c) < 0x0 (signed, 32-bit) to 0x6214c. The ret expression comes from the call at 0x6213c.
  `bne` at 0x62170 is taken when ret(0x4934c) != 0x0 to 0x62180; it falls through when ret(0x4934c) == 0x0 to 0x62174. The ret expression comes from the call at 0x62164.
  `bne` at 0x62190 is taken when ret(0x4a260) != 0x0 to 0x621a0; it falls through when ret(0x4a260) == 0x0 to 0x62194. The ret expression comes from the call at 0x62184.
  `bne` at 0x621b0 is taken when ret(0x49a58) != 0x0 to 0x621c0; it falls through when ret(0x49a58) == 0x0 to 0x621b4. The ret expression comes from the call at 0x621a4.
  The return instruction (`ldm` at 0x621ec) has an unknown r0 value.
  `ldr` at 0x62174 sets r3 to 0xfffff05e.
  `ldr` at 0x62194 sets r3 to 0xfffff05c.
  `ldr` at 0x621b4 sets r3 to 0xfffff05b.
  Undecodable bytes at 0x621f0-0x621fc; fall-through stops at this range.

- `portLinkQuerySlot`, `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`, 0x28fb4.
  Version 1.48.1, package `patches/SG0007149:l1#6`, inner `/stand/sysco/bin/l1`; occupies 168 bytes.
  A word store accesses [sp-28] +0x0 (`str` at 0x29028) with value ret(0x20760). The ret expression comes from the call at 0x29020.
  A word load accesses [sp-28] +0x0 (`ldr` at 0x2902c).
  A byte store accesses [[sp-28]] +0x0 (`strb` at 0x2903c) with value 0x0.
  `bl` at 0x28fe8 calls (unknown callee) [0x268c0] with r0=r0, r1=sp-44, r2=r2, r3=sp-44.
  `bl` at 0x29020 calls (unknown callee) [0x20760] with r0=unknown, r1=unknown, r2=ret(0x268c0), r3=unknown. The ret expression comes from the call at 0x28fe8.
  `bl` at 0x29050 calls (unknown callee) [0x28294] with r0=[sp-24], r1=[sp-60], r2=[sp-32], r3=[sp-36].
  `bne` at 0x28ffc is taken when ret(0x268c0) != 0x0 to 0x29004; it falls through when ret(0x268c0) == 0x0 to 0x29000. The ret expression comes from the call at 0x28fe8.
  For the return (`ldm` at 0x29058), the r0 value remains unknown.

- `i2cReadByte`, `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, 0xbc030.
  Version 1.62.1, package `cd-ist-3.24:l1#2`, inner `/stand/sysco/bin/l1`. It occupies 136 bytes.
  `bl` at 0xbc05c calls (unknown callee) [0xbba44] with r0=(r0 & 0xff), r1=r1, r2=r2, r3=(r0 & 0xff).
  `bl` at 0xbc090 calls (unknown callee) [0xbbe4c] with r0=[sp-18]:1, r1=[sp-24], r2=0x1, r3=[sp-18]:1.
  `bl` at 0xbc0a0 calls (unknown callee) [0xbbb88] with r0=0x0, r1=unknown, r2=unknown, r3=ret(0xbbe4c). The printed result comes from the call at 0xbc090.
  `beq` at 0xbc070 is taken when ret(0xbba44) == 0x0 to 0xbc080. It falls through when ret(0xbba44) != 0x0 to 0xbc074. The compared result comes from the call at 0xbc05c.
  At the return (`ldm` at 0xbc0b4), r0 is unknown.

- `i2cWriteByte`, `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, 0xbbefc.
  Version 1.62.1, package `cd-ist-3.24:l1#2`, inner `/stand/sysco/bin/l1`. It occupies 144 bytes.
  `bl` at 0xbbf2c calls (unknown callee) [0xbba44] with r0=(r0 & 0xff), r1=r1, r2=r2, r3=(r0 & 0xff).
  `bl` at 0xbbf64 calls (unknown callee) [0xbbd9c] with r0=[sp-18]:1, r1=sp-19, r2=0x1, r3=[sp-18]:1.
  `bl` at 0xbbf74 calls (unknown callee) [0xbbb88] with r0=0x0, r1=unknown, r2=unknown, r3=ret(0xbbd9c). The printed result comes from the call at 0xbbf64.
  `beq` at 0xbbf40 is taken when ret(0xbba44) == 0x0 to 0xbbf50. It falls through when ret(0xbba44) != 0x0 to 0xbbf44. The compared result comes from the call at 0xbbf2c.
  The value of r0 is unknown at the return (`ldm` at 0xbbf88).

- `uartClose`, `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, 0xbe100.
  Version 1.62.1, package `cd-ist-3.24:l1#2`, inner `/stand/sysco/bin/l1`. It occupies 508 bytes.
  2 such word loads access r0 +0x34, including `ldr` at 0xbe178, `ldr` at 0xbe1f0.
  2 such word stores access [sp-24] +0x34, including `str` at 0xbe198, `str` at 0xbe22c; stored value 0x0.
  A word load accesses r0 +0x38 (`ldr` at 0xbe1b4).
  2 such word stores access [sp-24] +0x38, including `str` at 0xbe1d4, `str` at 0xbe238; stored value 0x0.
  A word load accesses [sp-24] +0x38 (`ldr` at 0xbe208).
  The analysis includes 6 static word loads with base r3 and unknown effective addresses, including `ldr` at 0xbe248, `ldr` at 0xbe258, `ldr` at 0xbe268.
  2 such word stores access [sp-24] +0x4, including `str` at 0xbe28c, `str` at 0xbe2e8; stored value unknown.
  2 such word stores access [sp-24] +0x0, including `str` at 0xbe294, `str` at 0xbe2c0; stored value unknown.
  `bl` at 0xbe170 calls (unknown callee) [0x20a78] with r0=r0, r1=r1, r2=r2, r3=[sp-48].
  `bl` at 0xbe1ac calls (unknown callee) [0x20a78] with r0=r0, r1=r1, r2=r2, r3=[sp-48].
  `bl` at 0xbe1e8 calls (unknown callee) [0x20a78] with r0=r0, r1=r1, r2=r2, r3=[sp-48].
  `bl` at 0xbe200 calls (unknown callee) [0x20a78] with r0=ret(0x20a78), r1=unknown, r2=ret(0x20a78), r3=[r0+52]. The printed result comes from the call at 0xbe1e8.
  `bl` at 0xbe27c calls (unknown callee) [0xbea88] with r0=unknown, r1=unknown, r2=unknown, r3=unknown.
  `bl` at 0xbe2b4 calls (unknown callee) [0x19478] with r0=unknown, r1=unknown, r2=unknown, r3=unknown.
  `bl` at 0xbe2dc calls (unknown callee) [0x19478] with r0=unknown, r1=unknown, r2=unknown, r3=unknown.
  `beq` at 0xbe13c is taken when [sp-48] == 0x1 to 0xbe1a8. It falls through when [sp-48] != 0x1 to 0xbe140.
  `bgt` at 0xbe148 is taken when [sp-48] > 0x1 (signed, 32-bit) to 0xbe15c. It falls through when [sp-48] <= 0x1 (signed, 32-bit) to 0xbe14c.
  `beq` at 0xbe154 is taken when [sp-48] == 0x0 to 0xbe16c. It falls through when [sp-48] != 0x0 to 0xbe158.
  `beq` at 0xbe164 is taken when [sp-48] == 0x2 to 0xbe1e4. It falls through when [sp-48] != 0x2 to 0xbe168.
  `beq` at 0xbe180 is taken when [r0+52] == ret(0x20a78) to 0xbe190. It falls through when [r0+52] != ret(0x20a78) to 0xbe184. The compared result comes from the call at 0xbe170.
  `beq` at 0xbe1bc is taken when [r0+56] == ret(0x20a78) to 0xbe1cc. It falls through when [r0+56] != ret(0x20a78) to 0xbe1c0. The compared result comes from the call at 0xbe1ac.
  `bne` at 0xbe1f8 is taken when [r0+52] != ret(0x20a78) to 0xbe218. It falls through when [r0+52] == ret(0x20a78) to 0xbe1fc. The compared result comes from the call at 0xbe1e8.
  `bne` at 0xbe210 is taken when [[sp-24]+56] != ret(0x20a78) to 0xbe218. It falls through when [[sp-24]+56] == ret(0x20a78) to 0xbe214. The compared result comes from the call at 0xbe200.
  `bne` at 0xbe250 is taken when Z == 0 to 0xbe29c. It falls through when not (Z == 0) to 0xbe254.
  `bne` at 0xbe260 is taken when Z == 0 to 0xbe2ec. It falls through when not (Z == 0) to 0xbe264.
  `bne` at 0xbe270 is taken when Z == 0 to 0xbe2ec. It falls through when not (Z == 0) to 0xbe274.
  `beq` at 0xbe2a8 is taken when Z == 1 to 0xbe2c4. It falls through when not (Z == 1) to 0xbe2ac.
  `beq` at 0xbe2d0 is taken when Z == 1 to 0xbe2ec. It falls through when not (Z == 1) to 0xbe2d4.
  r0 remains unknown at the return (`ldm` at 0xbe2f8).
  The analysis invalidates unsupported effects for 7 such instructions, including `mvn` at 0xbe184, `mvn` at 0xbe1c0, `mvn` at 0xbe218.

- `uartWrite`, `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, 0xbe4b4.
  Version 1.62.1, package `cd-ist-3.24:l1#2`, inner `/stand/sysco/bin/l1`. It occupies 148 bytes.
  A word load accesses r0 +0x4 (`ldr` at 0xbe4ec).
  A word load accesses ret(0x1952c) +0x0 (`ldr` at 0xbe524). The result comes from the call at 0xbe51c.
  `bl` at 0xbe4f8 calls (unknown callee) [0x1943c] with r0=[r0+4], r1=r1, r2=r2, r3=r0.
  `bl` at 0xbe518 calls (unknown callee) [0xbe61c] with r0=[sp-20], r1=0x2, r2=unknown, r3=ret(0x1943c). The printed result comes from the call at 0xbe4f8.
  `bl` at 0xbe51c calls (unknown callee) [0x1952c] with r0=ret(0xbe61c), r1=unknown, r2=unknown, r3=unknown. The printed result comes from the call at 0xbe518.
  `bgt` at 0xbe4d8 is taken when r2 > 0x0 (signed, 32-bit) to 0xbe4e8. It falls through when r2 <= 0x0 (signed, 32-bit) to 0xbe4dc.
  `bge` at 0xbe50c is taken when ret(0x1943c) >= 0x0 (signed, 32-bit) to 0xbe534. It falls through when ret(0x1943c) < 0x0 (signed, 32-bit) to 0xbe510. The compared result comes from the call at 0xbe4f8.
  The return value in r0 is unknown (`ldm` at 0xbe544).
  The analysis invalidates unsupported effects for `rsb` at 0xbe528.

- `SHUB_ScanSetRackSlot`, `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, 0x42e34.
  Version 1.62.1, package `cd-ist-3.24:l1#2`, inner `/stand/sysco/bin/l1`. It occupies 412 bytes.
  10 word loads read PC-relative literal locations, including absolute 0x42fb0 (`ldr` at 0x42e60), absolute 0x42fb4 (`ldr` at 0x42e70), absolute 0x42fb8 (`ldr` at 0x42e7c).
  2 such word loads access absolute 0x29838c, including `ldr` at 0x42e64, `ldr` at 0x42ef8.
  The analysis includes 4 static word loads with base r3 and unknown effective addresses, including `ldr` at 0x42e90, `ldr` at 0x42ea4, `ldr` at 0x42eb4.
  `bl` at 0x42e54 calls (unknown callee) [0x6abd0] with r0=0x0, r1=r1, r2=r2, r3=0x6c.
  `bl` at 0x42e88 calls (unknown callee) [0x201fc] with r0=0x6, r1=0xf74b0, r2=0x1cd, r3=0x0.
  `bl` at 0x42f2c calls (unknown callee) [0x201fc] with r0=0x6, r1=0xf74b0, r2=0x1d8, r3=0x0.
  `bl` at 0x42f60 calls (unknown callee) [0x42374] with r0=[sp-20], r1=0x101d0100, r2=[sp-28], r3=unknown.
  `bl` at 0x42f98 calls (unknown callee) [0x1eab4] with r0=0xf755c, r1=[sp-20], r2=unknown, r3=unknown.
  `beq` at 0x42e6c is taken when [0x29838c] == 0x0 to 0x42e8c. It falls through when [0x29838c] != 0x0 to 0x42e70.
  `beq` at 0x42f00 is taken when [0x29838c] == 0x0 to 0x42f30. It falls through when [0x29838c] != 0x0 to 0x42f04.
  `bgt` at 0x42f40 is taken when gt flags condition (operands unknown) to 0x42f8c. It falls through when not (gt flags condition (operands unknown)) to 0x42f44.
  `bne` at 0x42f6c is taken when ret(0x42374) != 0x1 to 0x42f7c. It falls through when ret(0x42374) == 0x1 to 0x42f70. The compared result comes from the call at 0x42f60.
  The return (`ldm` at 0x42fac) leaves r0 unknown.
  The instructions from 0x42fb0 to 0x42fcc are not reached by any path from the entry, and their branches are not listed.
  Undecodable bytes at 0x42fcc-0x42fd0. Fall-through stops at this range.

- `i2cReadByte`, `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, 0x116cd0.
  Version 1.62.1, package `cd-ist-3.24:l1#4`, inner `/stand/sysco/bin/l1`; occupies 136 bytes.
  `bl` at 0x116cfc calls (unknown callee) [0x1166e4] with r0=(r0 & 0xff), r1=r1, r2=r2, r3=(r0 & 0xff).
  `bl` at 0x116d30 calls (unknown callee) [0x116aec] with r0=[sp-18]:1, r1=[sp-24], r2=0x1, r3=[sp-18]:1.
  `bl` at 0x116d40 calls (unknown callee) [0x116828] with r0=0x0, r1=unknown, r2=unknown, r3=ret(0x116aec); the result expression comes from the call at 0x116d30.
  `beq` at 0x116d10 is taken when ret(0x1166e4) == 0x0 to 0x116d20; it falls through when ret(0x1166e4) != 0x0 to 0x116d14; the result expression comes from the call at 0x116cfc.
  The return (`ldm` at 0x116d54) has r0 = unknown.

- `i2cWriteByte`, `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, 0x116b9c.
  Version 1.62.1, package `cd-ist-3.24:l1#4`, inner `/stand/sysco/bin/l1`; occupies 144 bytes.
  `bl` at 0x116bcc calls (unknown callee) [0x1166e4] with r0=(r0 & 0xff), r1=r1, r2=r2, r3=(r0 & 0xff).
  `bl` at 0x116c04 calls (unknown callee) [0x116a3c] with r0=[sp-18]:1, r1=sp-19, r2=0x1, r3=[sp-18]:1.
  `bl` at 0x116c14 calls (unknown callee) [0x116828] with r0=0x0, r1=unknown, r2=unknown, r3=ret(0x116a3c); the result expression comes from the call at 0x116c04.
  `beq` at 0x116be0 is taken when ret(0x1166e4) == 0x0 to 0x116bf0; it falls through when ret(0x1166e4) != 0x0 to 0x116be4; the result expression comes from the call at 0x116bcc.
  The return (`ldm` at 0x116c28) has r0 = unknown.

- `uartClose`, `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, 0x11b20c.
  Version 1.62.1, package `cd-ist-3.24:l1#4`, inner `/stand/sysco/bin/l1`; occupies 508 bytes.
  2 such word loads access r0 +0x34, including `ldr` at 0x11b284, `ldr` at 0x11b2fc.
  2 such word stores access [sp-24] +0x34, including `str` at 0x11b2a4, `str` at 0x11b338; stored value 0x0.
  A word load accesses r0 +0x38 (`ldr` at 0x11b2c0).
  2 such word stores access [sp-24] +0x38, including `str` at 0x11b2e0, `str` at 0x11b344; stored value 0x0.
  A word load accesses [sp-24] +0x38 (`ldr` at 0x11b314).
  The analysis includes 6 static word loads with base r3 and unknown effective addresses, including `ldr` at 0x11b354, `ldr` at 0x11b364, `ldr` at 0x11b374.
  2 such word stores access [sp-24] +0x4, including `str` at 0x11b398, `str` at 0x11b3f4; stored value unknown.
  2 such word stores access [sp-24] +0x0, including `str` at 0x11b3a0, `str` at 0x11b3cc; stored value unknown.
  `bl` at 0x11b27c calls (unknown callee) [0x23214] with r0=r0, r1=r1, r2=r2, r3=[sp-48].
  `bl` at 0x11b2b8 calls (unknown callee) [0x23214] with r0=r0, r1=r1, r2=r2, r3=[sp-48].
  `bl` at 0x11b2f4 calls (unknown callee) [0x23214] with r0=r0, r1=r1, r2=r2, r3=[sp-48].
  `bl` at 0x11b30c calls (unknown callee) [0x23214] with r0=ret(0x23214), r1=unknown, r2=ret(0x23214), r3=[r0+52]; the result expression comes from the call at 0x11b2f4.
  `bl` at 0x11b388 calls (unknown callee) [0x11bccc] with r0=unknown, r1=unknown, r2=unknown, r3=unknown.
  `bl` at 0x11b3c0 calls (unknown callee) [0x20838] with r0=unknown, r1=unknown, r2=unknown, r3=unknown.
  `bl` at 0x11b3e8 calls (unknown callee) [0x20838] with r0=unknown, r1=unknown, r2=unknown, r3=unknown.
  `beq` at 0x11b248 is taken when [sp-48] == 0x1 to 0x11b2b4; it falls through when [sp-48] != 0x1 to 0x11b24c.
  `bgt` at 0x11b254 is taken when [sp-48] > 0x1 (signed, 32-bit) to 0x11b268; it falls through when [sp-48] <= 0x1 (signed, 32-bit) to 0x11b258.
  `beq` at 0x11b260 is taken when [sp-48] == 0x0 to 0x11b278; it falls through when [sp-48] != 0x0 to 0x11b264.
  `beq` at 0x11b270 is taken when [sp-48] == 0x2 to 0x11b2f0; it falls through when [sp-48] != 0x2 to 0x11b274.
  `beq` at 0x11b28c is taken when [r0+52] == ret(0x23214) to 0x11b29c; it falls through when [r0+52] != ret(0x23214) to 0x11b290; the result expression comes from the call at 0x11b27c.
  `beq` at 0x11b2c8 is taken when [r0+56] == ret(0x23214) to 0x11b2d8; it falls through when [r0+56] != ret(0x23214) to 0x11b2cc; the result expression comes from the call at 0x11b2b8.
  `bne` at 0x11b304 is taken when [r0+52] != ret(0x23214) to 0x11b324; it falls through when [r0+52] == ret(0x23214) to 0x11b308; the result expression comes from the call at 0x11b2f4.
  `bne` at 0x11b31c is taken when [[sp-24]+56] != ret(0x23214) to 0x11b324; it falls through when [[sp-24]+56] == ret(0x23214) to 0x11b320; the result expression comes from the call at 0x11b30c.
  `bne` at 0x11b35c is taken when Z == 0 to 0x11b3a8; it falls through when not (Z == 0) to 0x11b360.
  `bne` at 0x11b36c is taken when Z == 0 to 0x11b3f8; it falls through when not (Z == 0) to 0x11b370.
  `bne` at 0x11b37c is taken when Z == 0 to 0x11b3f8; it falls through when not (Z == 0) to 0x11b380.
  `beq` at 0x11b3b4 is taken when Z == 1 to 0x11b3d0; it falls through when not (Z == 1) to 0x11b3b8.
  `beq` at 0x11b3dc is taken when Z == 1 to 0x11b3f8; it falls through when not (Z == 1) to 0x11b3e0.
  The return (`ldm` at 0x11b404) has r0 = unknown.
  The analysis invalidates unsupported effects for 7 such instructions, including `mvn` at 0x11b290, `mvn` at 0x11b2cc, `mvn` at 0x11b324.

- `uartWrite`, `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`, 0x11b6dc.
  Version 1.62.1, package `cd-ist-3.24:l1#4`, inner `/stand/sysco/bin/l1`; occupies 148 bytes.
  A word load accesses r0 +0x4 (`ldr` at 0x11b714).
  A word load accesses ret(0x20904) +0x0 (`ldr` at 0x11b74c); the base expression comes from the call at 0x11b744.
  `bl` at 0x11b720 calls (unknown callee) [0x207fc] with r0=[r0+4], r1=r1, r2=r2, r3=r0.
  `bl` at 0x11b740 calls (unknown callee) [0x11b844] with r0=[sp-20], r1=0x2, r2=unknown, r3=ret(0x207fc); the result expression comes from the call at 0x11b720.
  `bl` at 0x11b744 calls (unknown callee) [0x20904] with r0=ret(0x11b844), r1=unknown, r2=unknown, r3=unknown; the result expression comes from the call at 0x11b740.
  `bgt` at 0x11b700 is taken when r2 > 0x0 (signed, 32-bit) to 0x11b710; it falls through when r2 <= 0x0 (signed, 32-bit) to 0x11b704.
  `bge` at 0x11b734 is taken when ret(0x207fc) >= 0x0 (signed, 32-bit) to 0x11b75c; it falls through when ret(0x207fc) < 0x0 (signed, 32-bit) to 0x11b738; the result expression comes from the call at 0x11b720.
  The return (`ldm` at 0x11b76c) has r0 = unknown.
  The analysis invalidates unsupported effects for `rsb` at 0x11b750.
