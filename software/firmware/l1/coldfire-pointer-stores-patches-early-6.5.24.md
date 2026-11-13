# ColdFire L1 pointer stores in patches and selected early through 6.5.24 images

Each entry describes its exact carrier, package, payload and selected byte position. Addresses are analysis coordinates formed from the stated payload load plus byte offset. Later positions in one payload do not establish additional unique functions, physical flash banks, runtime mapping or image selection. No byte or semantic equivalence between positions is assumed.

The spans are inferred from entry, return and bounded context evidence. They are analyzed windows, not proved function sizes. Hardware roles, pointee meanings, types, fitted silicon and runtime path feasibility remain unknown. ColdFire classification comes from package metadata and decoding uses the 68040 common subset.

`sp` is the stack pointer at the analyzed entry. The six stack slots at each call are candidate argument locations under an assumed m68k-sysv convention. Actual arity and custom conventions remain unknown. A `ret(F)` expression does not identify its producing instruction. Repeated access groups count static shapes and do not establish equal values from separate loads or one unconditional path through different branch arms. Unknown values preserve tool question marks.

The [complete facts](disasm/coldfire-variants/pointer-stores-patches-early-6.5.24-facts.txt) retain the complete analyzed windows. Range ends are exclusive. Bounded contexts with clipped instructions or uncertain initial alignment are not proof of predecessor flow or full function extent.

- `0xffecb714`, `root/patches/SG0005762/usr/cpu/firmware/sysco/l1.bin`.
  Package `patches/SG0005762:l1#0`, version `1.32.15`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffecb714 to 0xffecb7d0 (exclusive), 188 bytes.
  Carrier: 7406448 bytes, SHA256 `2399ea17dfeb2af20fae4286f0cfeeffae6d3d9f637fcc02e880e5689f662656`.
  Package: carrier offset 0x0, 981788 bytes, SHA256 `cfe50f185db4cbb8da444dd49efedbd662c463ae161559ec6d24a8acbff2c7cb`.
  Payload: carrier offset 0x50, 981708 bytes, SHA256 `330ef3074bdfdbbabcffd95fb2c26ea942118cd33e4029e0277d136d63e11e0f`.
  Read window: SHA256 `b1aa55ebcf793703f1e3eba618cb2cdc5c09e21ec66386b626ce5f5771ca0c29`.
  Call (`jsr` at 0xffecb71a): (unknown callee), target 0xffece2d4, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffecb722): (unknown callee), target 0xffecb02c, stack arg0=`ret(0xffece2d4)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffecb728).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffecb72e).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffecb732).
  Grouped accesses: 2 4-byte load records with target `abs 0x300025bc`, including `tst.l` at 0xffecb73c, `move.l` at 0xffecb77c.
  Branch (`ble.b` at 0xffecb740): taken to 0xffecb77c when `[0x300025bc] <= 0x0 (signed, 32-bit)`, falls through to 0xffecb742 when `[0x300025bc] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x300003bc`, including `movea.l` at 0xffecb748, `movea.l` at 0xffecb752, `movea.l` at 0xffecb758.
  Two-byte store: target `[0x300003bc] +0x88`, value `0x4000` (`move.w` at 0xffecb74e).
  Four-byte store: target `[0x300003bc] +0x8c`, value `0x0` (`clr.l` at 0xffecb754).
  Two-byte store: target `[0x300003bc] +0x92`, value `0x953` (`move.w` at 0xffecb75e).
  Call (`jsr` at 0xffecb772): (unknown callee), target 0xffedd590, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffecb782): taken to 0xffecb7c8 when `Z == 0`, falls through to 0xffecb784 when `not (Z == 0)`.
  Byte store: target `[0x300003bc] +0x16`, value `0x8c` (`move.b` at 0xffecb78e).
  Two-byte store: target `[0x300003bc] +0xa0`, value `0x6000` (`move.w` at 0xffecb798).
  Four-byte store: target `[0x300003bc] +0xa4`, value `0x0` (`clr.l` at 0xffecb79e).
  Two-byte store: target `[0x300003bc] +0xaa`, value `0x953` (`move.w` at 0xffecb7a8).
  Call (`jsr` at 0xffecb7be): (unknown callee), target 0xffedd590, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the return (`rts` at 0xffecb7ce), d0 is `unknown`.

- `0xffecb3b8`, `root/patches/SG0005762/usr/cpu/firmware/sysco/l1.bin`.
  Package `patches/SG0005762:l1#1`, version `1.32.15`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffecb3b8 to 0xffecb474 (exclusive), 188 bytes.
  Carrier: 7406448 bytes, SHA256 `2399ea17dfeb2af20fae4286f0cfeeffae6d3d9f637fcc02e880e5689f662656`.
  Package: carrier offset 0xefb1c, 981728 bytes, SHA256 `fdc8eb9f405f0cc5a8eb72dd9e3e579e28e1f72b574c0508639269d722830922`.
  Payload: carrier offset 0xefb48, 981684 bytes, SHA256 `1aa8d9f5f1ebd4a38b084a4f783f6b3c402fa4ec14b3e08051b8bf913c36dcbc`.
  Read window: SHA256 `3208caa0de66d532f8b65af49490abf440f02fabb985e4e4eae58bbb7f7b8d62`.
  Call (`jsr` at 0xffecb3be): (unknown callee), target 0xffecdf78, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffecb3c6): (unknown callee), target 0xffecacd0, stack arg0=`ret(0xffecdf78)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffecb3cc).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffecb3d2).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffecb3d6).
  Grouped accesses: 2 4-byte load records with target `abs 0x300026b8`, including `tst.l` at 0xffecb3e0, `move.l` at 0xffecb420.
  Branch (`ble.b` at 0xffecb3e4): taken to 0xffecb420 when `[0x300026b8] <= 0x0 (signed, 32-bit)`, falls through to 0xffecb3e6 when `[0x300026b8] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x300003e8`, including `movea.l` at 0xffecb3ec, `movea.l` at 0xffecb3f6, `movea.l` at 0xffecb3fc.
  Two-byte store: target `[0x300003e8] +0x88`, value `0x4000` (`move.w` at 0xffecb3f2).
  Four-byte store: target `[0x300003e8] +0x8c`, value `0x0` (`clr.l` at 0xffecb3f8).
  Two-byte store: target `[0x300003e8] +0x92`, value `0x953` (`move.w` at 0xffecb402).
  Call (`jsr` at 0xffecb416): (unknown callee), target 0xffedd42c, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffecb426): taken to 0xffecb46c when `Z == 0`, falls through to 0xffecb428 when `not (Z == 0)`.
  Byte store: target `[0x300003e8] +0x16`, value `0x8c` (`move.b` at 0xffecb432).
  Two-byte store: target `[0x300003e8] +0xa0`, value `0x6000` (`move.w` at 0xffecb43c).
  Four-byte store: target `[0x300003e8] +0xa4`, value `0x0` (`clr.l` at 0xffecb442).
  Two-byte store: target `[0x300003e8] +0xaa`, value `0x953` (`move.w` at 0xffecb44c).
  Call (`jsr` at 0xffecb462): (unknown callee), target 0xffedd42c, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  The exit (`rts` at 0xffecb472) reports d0 = `unknown`.

- `0xfff7e138`, `root/patches/SG0005762/usr/cpu/firmware/sysco/l1.bin`.
  Package `patches/SG0005762:l1#2`, version `1.32.15`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xfff7e138 to 0xfff7e1f4 (exclusive), 188 bytes.
  Carrier: 7406448 bytes, SHA256 `2399ea17dfeb2af20fae4286f0cfeeffae6d3d9f637fcc02e880e5689f662656`.
  Package: carrier offset 0x1df5fc, 1887360 bytes, SHA256 `7872ba43629634598f82b69c94bc0954b8cd3680c2e5578e2f3c8471c1cd7aa4`.
  Payload: carrier offset 0x1df628, 1887316 bytes, SHA256 `ad6c39f0232f959b55c18e57ec29425f071354c7a7d8bc21abc0ceb136c8c823`.
  Read window: SHA256 `c9af4af5ce131d30bb5c4eced387467a0e00a2172bfd7e8c572a2e91b80c8d4e`.
  Call (`jsr` at 0xfff7e13e): (unknown callee), target 0xfff80cf8, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xfff7e146): (unknown callee), target 0xfff7da50, stack arg0=`ret(0xfff80cf8)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xfff7e14c).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xfff7e152).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xfff7e156).
  Grouped accesses: 2 4-byte load records with target `abs 0x30003298`, including `tst.l` at 0xfff7e160, `move.l` at 0xfff7e1a0.
  Branch (`ble.b` at 0xfff7e164): taken to 0xfff7e1a0 when `[0x30003298] <= 0x0 (signed, 32-bit)`, falls through to 0xfff7e166 when `[0x30003298] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000a40`, including `movea.l` at 0xfff7e16c, `movea.l` at 0xfff7e176, `movea.l` at 0xfff7e17c.
  Two-byte store: target `[0x30000a40] +0x88`, value `0x4000` (`move.w` at 0xfff7e172).
  Four-byte store: target `[0x30000a40] +0x8c`, value `0x0` (`clr.l` at 0xfff7e178).
  Two-byte store: target `[0x30000a40] +0x92`, value `0x953` (`move.w` at 0xfff7e182).
  Call (`jsr` at 0xfff7e196): (unknown callee), target 0xfff9dfa4, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xfff7e1a6): taken to 0xfff7e1ec when `Z == 0`, falls through to 0xfff7e1a8 when `not (Z == 0)`.
  Byte store: target `[0x30000a40] +0x16`, value `0x8c` (`move.b` at 0xfff7e1b2).
  Two-byte store: target `[0x30000a40] +0xa0`, value `0x6000` (`move.w` at 0xfff7e1bc).
  Four-byte store: target `[0x30000a40] +0xa4`, value `0x0` (`clr.l` at 0xfff7e1c2).
  Two-byte store: target `[0x30000a40] +0xaa`, value `0x953` (`move.w` at 0xfff7e1cc).
  Call (`jsr` at 0xfff7e1e2): (unknown callee), target 0xfff9dfa4, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  `rts` at 0xfff7e1f2 has recorded d0 = `unknown`.

- `0xfff63508`, `root/patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`.
  Package `patches/SG0007149:l1#5`, version `1.48.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xfff63508 to 0xfff635c4 (exclusive), 188 bytes.
  Carrier: 16027376 bytes, SHA256 `536bfa4c6d0240b7afbc25bef94e160bdf1a675016c35e68ec52e1bfb1b451f9`.
  Package: carrier offset 0x8e1250, 1768364 bytes, SHA256 `c83d94ff29874a2a3276cdc052556e40e0c42e1c332e722711cdb4cfadfbb579`.
  Payload: carrier offset 0x8e127c, 1768320 bytes, SHA256 `57d9c981a155eeb1c3ccaaae9d2dee701f553710f5052765f01ca645d279fe9f`.
  Read window: SHA256 `74681106e909d074027377fa58b5f0f5d03de8fa964cc2dead67949708d1c5e0`.
  Call (`jsr` at 0xfff6350e): (unknown callee), target 0xfff635c4, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xfff63516): (unknown callee), target 0xfff6310c, stack arg0=`ret(0xfff635c4)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xfff6351c).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xfff63522).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xfff63526).
  Grouped accesses: 2 4-byte load records with target `abs 0x30002ca0`, including `tst.l` at 0xfff63530, `move.l` at 0xfff63570.
  Branch (`ble.b` at 0xfff63534): taken to 0xfff63570 when `[0x30002ca0] <= 0x0 (signed, 32-bit)`, falls through to 0xfff63536 when `[0x30002ca0] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000a84`, including `movea.l` at 0xfff6353c, `movea.l` at 0xfff63546, `movea.l` at 0xfff6354c.
  Two-byte store: target `[0x30000a84] +0x88`, value `0x4000` (`move.w` at 0xfff63542).
  Four-byte store: target `[0x30000a84] +0x8c`, value `0x0` (`clr.l` at 0xfff63548).
  Two-byte store: target `[0x30000a84] +0x92`, value `0x953` (`move.w` at 0xfff63552).
  Call (`jsr` at 0xfff63566): (unknown callee), target 0xfff829d4, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xfff63576): taken to 0xfff635bc when `Z == 0`, falls through to 0xfff63578 when `not (Z == 0)`.
  Byte store: target `[0x30000a84] +0x16`, value `0x8c` (`move.b` at 0xfff63582).
  Two-byte store: target `[0x30000a84] +0xa0`, value `0x6000` (`move.w` at 0xfff6358c).
  Four-byte store: target `[0x30000a84] +0xa4`, value `0x0` (`clr.l` at 0xfff63592).
  Two-byte store: target `[0x30000a84] +0xaa`, value `0x953` (`move.w` at 0xfff6359c).
  Call (`jsr` at 0xfff635b2): (unknown callee), target 0xfff829d4, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Recorded d0 at the return is `unknown` (`rts` at 0xfff635c2).

- `0xffeb2060`, `root/6.5.9/usr/cpu/firmware/l1.bin`.
  Package `6.5.9:l1#0`, version `1.0.2`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffeb2060 to 0xffeb20f6 (exclusive), 150 bytes.
  Carrier: 927304 bytes, SHA256 `346ebde8121eda2b390df35b37bca75a28c5bbfc44dd5f329b91d56455b575e8`.
  Package: carrier offset 0x0, 927304 bytes, SHA256 `346ebde8121eda2b390df35b37bca75a28c5bbfc44dd5f329b91d56455b575e8`.
  Payload: carrier offset 0x20, 927272 bytes, SHA256 `8a177e55cfdd58f8d233ea3c6e5f8b1a5a9bcaf10a20e95cbde444bd34da6545`.
  Read window: SHA256 `b7b111ae58461ae8ba6ce7608fcaea3816ae6dc7afb4df9ae150de6f6fc8746e`.
  Call (`jsr` at 0xffeb2066): (unknown callee), target 0xffeb7428, stack arg0=`d7`, stack arg1=`a6`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffeb2076): (unknown callee), target 0xffeb20f8, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bgt.b` at 0xffeb208a): taken to 0xffeb20a0 when `gt flags condition (operands unknown)`, falls through to 0xffeb208c when `not (gt flags condition (operands unknown))`.
  Branch (`beq.b` at 0xffeb208c): taken to 0xffeb20b2 when `Z == 1`, falls through to 0xffeb208e when `not (Z == 1)`.
  Branch (`beq.b` at 0xffeb2094): taken to 0xffeb20b2 when `Z == 1`, falls through to 0xffeb2096 when `not (Z == 1)`.
  Branch (`beq.b` at 0xffeb2098): taken to 0xffeb20b2 when `Z == 1`, falls through to 0xffeb209a when `not (Z == 1)`.
  Branch (`beq.b` at 0xffeb209c): taken to 0xffeb20b2 when `Z == 1`, falls through to 0xffeb209e when `not (Z == 1)`.
  Jump (`bra.b` at 0xffeb209e): target 0xffeb20ee.
  Branch (`beq.b` at 0xffeb20a2): taken to 0xffeb20b2 when `Z == 1`, falls through to 0xffeb20a4 when `not (Z == 1)`.
  Branch (`beq.b` at 0xffeb20aa): taken to 0xffeb20b2 when `Z == 1`, falls through to 0xffeb20ac when `not (Z == 1)`.
  Branch (`beq.b` at 0xffeb20ae): taken to 0xffeb20b2 when `Z == 1`, falls through to 0xffeb20b0 when `not (Z == 1)`.
  Jump (`bra.b` at 0xffeb20b0): target 0xffeb20ee.
  Grouped accesses: 3 4-byte load records with target `abs 0x30000480`, including `movea.l` at 0xffeb20b2, `movea.l` at 0xffeb20c0, `movea.l` at 0xffeb20ca.
  Two-byte store: target `[0x30000480] +0x88`, value `unknown` (`move.w` at 0xffeb20bc).
  Four-byte store: target `[0x30000480] +0x8c`, value `0x0` (`clr.l` at 0xffeb20c6).
  Two-byte store: target `[0x30000480] +0x92`, value `unknown` (`move.w` at 0xffeb20d4).
  Call (`jsr` at 0xffeb20e4): (unknown callee), target 0xffec43d4, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the return (`rts` at 0xffeb20f4), d0 is `unknown`.

- `0xffe5a3f8`, `root/6.5.10/usr/cpu/firmware/l1.bin`.
  Package `6.5.10:l1#0`, version `1.1.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe5a3f8 to 0xffe5a40c (exclusive), 20 bytes.
  Carrier: 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Package: carrier offset 0x0, 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Payload: carrier offset 0x28, 961564 bytes, SHA256 `5b9d6771ee2405c5d556304b713f1a6b1deaa9bfd49009cd21856cd266eb3ef9`.
  Read window: SHA256 `f84c60761956fa33653152c06154bf58790f8a3e307953352e2deaf0aa486c6a`.
  Four-byte load: target `abs 0x30000490` (`movea.l` at 0xffe5a3fc).
  Byte store: target `[0x30000490] +0x1e8`, value `0x80` (`move.b` at 0xffe5a404).
  The exit (`rts` at 0xffe5a40a) reports d0 = `0xffffff80`.

- `0xffeca3f8`, `root/6.5.10/usr/cpu/firmware/l1.bin`.
  Package `6.5.10:l1#0`, version `1.1.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffeca3f8 to 0xffeca40c (exclusive), 20 bytes.
  Carrier: 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Package: carrier offset 0x0, 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Payload: carrier offset 0x28, 961564 bytes, SHA256 `5b9d6771ee2405c5d556304b713f1a6b1deaa9bfd49009cd21856cd266eb3ef9`.
  Read window: SHA256 `f84c60761956fa33653152c06154bf58790f8a3e307953352e2deaf0aa486c6a`.
  Four-byte load: target `abs 0x30000490` (`movea.l` at 0xffeca3fc).
  Byte store: target `[0x30000490] +0x1e8`, value `0x80` (`move.b` at 0xffeca404).
  `rts` at 0xffeca40a has recorded d0 = `0xffffff80`.

- `0xffe654ac`, `root/6.5.12/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.12:l1#0`, version `1.4.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe654ac to 0xffe654c0 (exclusive), 20 bytes.
  Carrier: 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Package: carrier offset 0x0, 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Payload: carrier offset 0x28, 947796 bytes, SHA256 `7084835eb61932052dc28531fe420d397a42df805f59b49d47bec8a95f6dcc93`.
  Read window: SHA256 `54c68f0828deeb406f57e63e23ecb7c3cc083d3f6fe7b1222cb34bebef85914b`.
  Four-byte load: target `abs 0x30000198` (`movea.l` at 0xffe654b0).
  Byte store: target `[0x30000198] +0x1e8`, value `0x80` (`move.b` at 0xffe654b8).
  Recorded d0 at the return is `0xffffff80` (`rts` at 0xffe654be).

- `0xffed54ac`, `root/6.5.12/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.12:l1#0`, version `1.4.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffed54ac to 0xffed54c0 (exclusive), 20 bytes.
  Carrier: 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Package: carrier offset 0x0, 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Payload: carrier offset 0x28, 947796 bytes, SHA256 `7084835eb61932052dc28531fe420d397a42df805f59b49d47bec8a95f6dcc93`.
  Read window: SHA256 `54c68f0828deeb406f57e63e23ecb7c3cc083d3f6fe7b1222cb34bebef85914b`.
  Four-byte load: target `abs 0x30000198` (`movea.l` at 0xffed54b0).
  Byte store: target `[0x30000198] +0x1e8`, value `0x80` (`move.b` at 0xffed54b8).
  At the return (`rts` at 0xffed54be), d0 is `0xffffff80`.

- `0xffe67730`, `root/6.5.13/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.13:l1#0`, version `1.6.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe67730 to 0xffe67744 (exclusive), 20 bytes.
  Carrier: 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Package: carrier offset 0x0, 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Payload: carrier offset 0x28, 957064 bytes, SHA256 `6e560e68a673766d56d716333f39ae79dca4b8779b59c7c97ef02741c822611d`.
  Read window: SHA256 `0a26c2e1b25be5367887bff67b8a30e0222db6833b39d35ec8fd3f0994e606c6`.
  Four-byte load: target `abs 0x300002ac` (`movea.l` at 0xffe67734).
  Byte store: target `[0x300002ac] +0x1e8`, value `0x80` (`move.b` at 0xffe6773c).
  The exit (`rts` at 0xffe67742) reports d0 = `0xffffff80`.

- `0xffed7730`, `root/6.5.13/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.13:l1#0`, version `1.6.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffed7730 to 0xffed7744 (exclusive), 20 bytes.
  Carrier: 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Package: carrier offset 0x0, 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Payload: carrier offset 0x28, 957064 bytes, SHA256 `6e560e68a673766d56d716333f39ae79dca4b8779b59c7c97ef02741c822611d`.
  Read window: SHA256 `0a26c2e1b25be5367887bff67b8a30e0222db6833b39d35ec8fd3f0994e606c6`.
  Four-byte load: target `abs 0x300002ac` (`movea.l` at 0xffed7734).
  Byte store: target `[0x300002ac] +0x1e8`, value `0x80` (`move.b` at 0xffed773c).
  `rts` at 0xffed7742 has recorded d0 = `0xffffff80`.

- `0xffe71e64`, `root/6.5.23/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.23:l1#0`, version `1.26.5`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe71e64 to 0xffe71e7a (exclusive), 22 bytes.
  Carrier: 3763312 bytes, SHA256 `15fe4284aa152ba30d1a8d9c67f63ee2acf1d19b0653068befc89146c12187f1`.
  Package: carrier offset 0x0, 982364 bytes, SHA256 `01f4157793ada930d8f671157cdd27fad86188a92d4a4c32b5c2c8133fcd243b`.
  Payload: carrier offset 0x48, 982292 bytes, SHA256 `85d56d8030146394e6339ca8e34f99d091e9588ecd9a752759a35358c554e5e8`.
  Read window: SHA256 `fcf0a5ee2e967c94ea2d05da09343422e48d422f2835977e4558240dc09120a3`.
  Four-byte load: target `abs 0x30000808` (`movea.l` at 0xffe71e68).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe71e6e).
  Byte store: target `[0x30000808] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffe71e72).
  Recorded d0 at the return is `[sp+4]` (`rts` at 0xffe71e78).

- `0xffe7197c`, `root/6.5.23/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.23:l1#1`, version `1.26.5`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe7197c to 0xffe71992 (exclusive), 22 bytes.
  Carrier: 3763312 bytes, SHA256 `15fe4284aa152ba30d1a8d9c67f63ee2acf1d19b0653068befc89146c12187f1`.
  Package: carrier offset 0xefd5c, 981328 bytes, SHA256 `aea0366c69ecffb773f22102af9e5d2696ec1eff91925377f388345d468380a0`.
  Payload: carrier offset 0xefd88, 981284 bytes, SHA256 `1c9aeb0a2af5bb40c97a727427e01cc159613434ed1a7a670b2dd81566888cf4`.
  Read window: SHA256 `2e82c108cb971a797514f8120b9ea90d0561481bf2e251fb36ab4b71c67fa0d2`.
  Four-byte load: target `abs 0x300008b0` (`movea.l` at 0xffe71980).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe71986).
  Byte store: target `[0x300008b0] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffe7198a).
  At the return (`rts` at 0xffe71990), d0 is `[sp+4]`.

- `0xffea1c50`, `root/6.5.23/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.23:l1#2`, version `1.26.5`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffea1c50 to 0xffea1c66 (exclusive), 22 bytes.
  Carrier: 3763312 bytes, SHA256 `15fe4284aa152ba30d1a8d9c67f63ee2acf1d19b0653068befc89146c12187f1`.
  Package: carrier offset 0x1df6ac, 1799620 bytes, SHA256 `f8faa30fbd2a62abf34dc81f249e3eced855814e03126cd61f23787233c72db5`.
  Payload: carrier offset 0x1df6d8, 1799576 bytes, SHA256 `92705d72b6589449f4852e8d45d40a995d5c54909f9de62af36ea3b5aeb28414`.
  Read window: SHA256 `94402606408060e677a798cd6ee0b98158cf94b2cb700fe6f12b69d682612151`.
  Four-byte load: target `abs 0x30000afc` (`movea.l` at 0xffea1c54).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffea1c5a).
  Byte store: target `[0x30000afc] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffea1c5e).
  The exit (`rts` at 0xffea1c64) reports d0 = `[sp+4]`.

- `0xffe716c0`, `root/6.5.24/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.24:l1#0`, version `1.28.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe716c0 to 0xffe716d6 (exclusive), 22 bytes.
  Carrier: 3759416 bytes, SHA256 `12e1052718a311de3dfd6424820e680fbaedace11a249c463a1ff5bbca2236c1`.
  Package: carrier offset 0x0, 980132 bytes, SHA256 `743c2e9361fcdd2134ae3d68a4773694a6909e4016afbc25530506172432cbd5`.
  Payload: carrier offset 0x48, 980060 bytes, SHA256 `edb297ad427641845d4abf1ffb2c0e82e81e5b26f56cd2c1e3d888352a34fd50`.
  Read window: SHA256 `fcf0a5ee2e967c94ea2d05da09343422e48d422f2835977e4558240dc09120a3`.
  Four-byte load: target `abs 0x30000808` (`movea.l` at 0xffe716c4).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe716ca).
  Byte store: target `[0x30000808] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffe716ce).
  `rts` at 0xffe716d4 has recorded d0 = `[sp+4]`.
