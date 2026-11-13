# ColdFire byte stores and branching memory windows

Each entry describes its exact carrier, package, payload and selected byte position. Addresses are analysis coordinates formed from the stated payload load plus byte offset. Later positions in one payload do not establish additional unique functions, physical flash banks, runtime mapping or image selection. No byte or semantic equivalence between positions is assumed.

The spans are inferred from entry, return and bounded context evidence. They are analyzed windows, not proved function sizes. Hardware roles, pointee meanings, types, fitted silicon and runtime path feasibility remain unknown. ColdFire classification comes from package metadata and decoding uses the 68040 common subset.

`sp` is the stack pointer at the analyzed entry. The six stack slots at each call are candidate argument locations under an assumed m68k-sysv convention. Actual arity and custom conventions remain unknown. A `ret(F)` expression does not identify its producing instruction. Repeated access groups count static shapes and do not establish equal values from separate loads or one unconditional path through different branch arms. Unknown values preserve tool question marks.

The [complete facts](disasm/coldfire-variants/byte-stores-branching-memory-facts.txt) retain the complete analyzed windows. Range ends are exclusive. Bounded contexts with clipped instructions or uncertain initial alignment are not proof of predecessor flow or full function extent.

- `0xffeb6ad4`, `root/patches/SG0005762/usr/cpu/firmware/sysco/l1.bin`.
  Package `patches/SG0005762:l1#2`, version `1.32.15`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffeb6ad4 to 0xffeb6aea (exclusive), 22 bytes.
  Carrier: 7406448 bytes, SHA256 `2399ea17dfeb2af20fae4286f0cfeeffae6d3d9f637fcc02e880e5689f662656`.
  Package: carrier offset 0x1df5fc, 1887360 bytes, SHA256 `7872ba43629634598f82b69c94bc0954b8cd3680c2e5578e2f3c8471c1cd7aa4`.
  Payload: carrier offset 0x1df628, 1887316 bytes, SHA256 `ad6c39f0232f959b55c18e57ec29425f071354c7a7d8bc21abc0ceb136c8c823`.
  Read window: SHA256 `575d2a1201c4405f51b2f3076f9684ba0882be632d0ae1bf7250fc8388c036c2`.
  Four-byte load: target `abs 0x30000a40` (`movea.l` at 0xffeb6ad8).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffeb6ade).
  Byte store: target `[0x30000a40] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffeb6ae2).
  At the return (`rts` at 0xffeb6ae8), d0 is `[sp+4]`.

- `0xffe72430`, `root/patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`.
  Package `patches/SG0007149:l1#0`, version `1.48.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe72430 to 0xffe72446 (exclusive), 22 bytes.
  Carrier: 16027376 bytes, SHA256 `536bfa4c6d0240b7afbc25bef94e160bdf1a675016c35e68ec52e1bfb1b451f9`.
  Package: carrier offset 0x0, 982480 bytes, SHA256 `19126693d5496e0891c357937d2dd2732964d6fce35d232cd9aed9c661d158ad`.
  Payload: carrier offset 0x68, 982376 bytes, SHA256 `bfb24887a95c026e6a1a6d5cb64f9341891a64908bc550ebb6c4538a54fc0e62`.
  Read window: SHA256 `d8cac666ab097c98176536d75bfba92436ccf66293a8c73be2d7f61379c40418`.
  Four-byte load: target `abs 0x300002bc` (`movea.l` at 0xffe72434).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe7243a).
  Byte store: target `[0x300002bc] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffe7243e).
  The exit (`rts` at 0xffe72444) reports d0 = `[sp+4]`.

- `0xffe722e0`, `root/patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`.
  Package `patches/SG0007149:l1#1`, version `1.48.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe722e0 to 0xffe722f6 (exclusive), 22 bytes.
  Carrier: 16027376 bytes, SHA256 `536bfa4c6d0240b7afbc25bef94e160bdf1a675016c35e68ec52e1bfb1b451f9`.
  Package: carrier offset 0xefdd0, 982412 bytes, SHA256 `38ee84025c55df1eed72fd54492de57c80b19efca16046b1809f697dec2f3260`.
  Payload: carrier offset 0xefdfc, 982368 bytes, SHA256 `de6218d069df04903f058b9233c92441d5ae673262e7bb2a5182b6ab2b624dc4`.
  Read window: SHA256 `3c3fe93a41c4b77e9abc896118377b9d5af483e2cf22089808add3b82a28d301`.
  Four-byte load: target `abs 0x300002e8` (`movea.l` at 0xffe722e4).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe722ea).
  Byte store: target `[0x300002e8] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffe722ee).
  `rts` at 0xffe722f4 has recorded d0 = `[sp+4]`.

- `0xffe9548c`, `root/patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`.
  Package `patches/SG0007149:l1#2`, version `1.48.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe9548c to 0xffe954a2 (exclusive), 22 bytes.
  Carrier: 16027376 bytes, SHA256 `536bfa4c6d0240b7afbc25bef94e160bdf1a675016c35e68ec52e1bfb1b451f9`.
  Package: carrier offset 0x1dfb5c, 1755564 bytes, SHA256 `0fb3794227c1ae743f7f0cdd5993df2469b76c71cbf485789deac09838a7cc16`.
  Payload: carrier offset 0x1dfb88, 1755520 bytes, SHA256 `13f01e4aae803cd12f181fb1e6ad28e6dc72ae477013e30d1f7b5aff6aa49a81`.
  Read window: SHA256 `89904c2743603d571eeeed97a16ee904f6b71294f88fb23b18913021c2eb7660`.
  Four-byte load: target `abs 0x300008bc` (`movea.l` at 0xffe95490).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe95496).
  Byte store: target `[0x300008bc] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffe9549a).
  Recorded d0 at the return is `[sp+4]` (`rts` at 0xffe954a0).

- `0xffe9c294`, `root/patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`.
  Package `patches/SG0007149:l1#3`, version `1.48.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe9c294 to 0xffe9c2aa (exclusive), 22 bytes.
  Carrier: 16027376 bytes, SHA256 `536bfa4c6d0240b7afbc25bef94e160bdf1a675016c35e68ec52e1bfb1b451f9`.
  Package: carrier offset 0x38c508, 1772620 bytes, SHA256 `cfbcc8c2175c46fb04f6b52c012f601a2dea8200511faf9b5cfef5fc4cfd00bc`.
  Payload: carrier offset 0x38c534, 1772576 bytes, SHA256 `1a8b082fa594a182b1e7a44c135409c29758297641afb9048149ae010038bfdd`.
  Read window: SHA256 `57a07135f45c035a9584ce1719a6f889c72b559cd8ca39bbe6ce53a2ffa6a430`.
  Four-byte load: target `abs 0x30000a50` (`movea.l` at 0xffe9c298).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe9c29e).
  Byte store: target `[0x30000a50] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffe9c2a2).
  At the return (`rts` at 0xffe9c2a8), d0 is `[sp+4]`.

- `0xffe9804c`, `root/patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`.
  Package `patches/SG0007149:l1#5`, version `1.48.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe9804c to 0xffe98062 (exclusive), 22 bytes.
  Carrier: 16027376 bytes, SHA256 `536bfa4c6d0240b7afbc25bef94e160bdf1a675016c35e68ec52e1bfb1b451f9`.
  Package: carrier offset 0x8e1250, 1768364 bytes, SHA256 `c83d94ff29874a2a3276cdc052556e40e0c42e1c332e722711cdb4cfadfbb579`.
  Payload: carrier offset 0x8e127c, 1768320 bytes, SHA256 `57d9c981a155eeb1c3ccaaae9d2dee701f553710f5052765f01ca645d279fe9f`.
  Read window: SHA256 `3ac6d141d4737ccd958cdf10b6ea760b0f900369c56af90d6243b1d5985826e6`.
  Four-byte load: target `abs 0x30000a84` (`movea.l` at 0xffe98050).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe98056).
  Byte store: target `[0x30000a84] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffe9805a).
  The exit (`rts` at 0xffe98060) reports d0 = `[sp+4]`.

- `0xffee2430`, `root/6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#0`, version `1.44.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffee2430 to 0xffee2446 (exclusive), 22 bytes.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x0, 982480 bytes, SHA256 `c391b1c9925c25fa5040d54e9adc51c8cb2ed9b5626a8734938bbc19267e4f08`.
  Payload: carrier offset 0x68, 982376 bytes, SHA256 `3c8a7aa93eb38502289cf0c2921ea91e6800228005bd8abd67425ce615f7b364`.
  Read window: SHA256 `d8cac666ab097c98176536d75bfba92436ccf66293a8c73be2d7f61379c40418`.
  Four-byte load: target `abs 0x300002bc` (`movea.l` at 0xffee2434).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffee243a).
  Byte store: target `[0x300002bc] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffee243e).
  `rts` at 0xffee2444 has recorded d0 = `[sp+4]`.

- `0xfff8548c`, `root/6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#2`, version `1.44.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xfff8548c to 0xfff854a2 (exclusive), 22 bytes.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x1dfb5c, 1755564 bytes, SHA256 `22e340c4657850647ce06e36173f6e671359ed8d26a6468c3c0ef6bde644540c`.
  Payload: carrier offset 0x1dfb88, 1755520 bytes, SHA256 `ac81f501f01c6789e756ab1f5ec69fadb1e4c04225511d541340831686e04e70`.
  Read window: SHA256 `89904c2743603d571eeeed97a16ee904f6b71294f88fb23b18913021c2eb7660`.
  Four-byte load: target `abs 0x300008bc` (`movea.l` at 0xfff85490).
  Four-byte load: target `sp +0x4` (`move.l` at 0xfff85496).
  Byte store: target `[0x300008bc] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xfff8549a).
  Recorded d0 at the return is `[sp+4]` (`rts` at 0xfff854a0).

- `0xfff8c294`, `root/6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#3`, version `1.44.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xfff8c294 to 0xfff8c2aa (exclusive), 22 bytes.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x38c508, 1772620 bytes, SHA256 `40d905ba620ebba0c8259b12410dfe42d21d18cad06477b2873f201059af2fe4`.
  Payload: carrier offset 0x38c534, 1772576 bytes, SHA256 `d13e4959222bad3308e71ba9c854efae01770afb942a9e562212eaeb19fff6fa`.
  Read window: SHA256 `57a07135f45c035a9584ce1719a6f889c72b559cd8ca39bbe6ce53a2ffa6a430`.
  Four-byte load: target `abs 0x30000a50` (`movea.l` at 0xfff8c298).
  Four-byte load: target `sp +0x4` (`move.l` at 0xfff8c29e).
  Byte store: target `[0x30000a50] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xfff8c2a2).
  At the return (`rts` at 0xfff8c2a8), d0 is `[sp+4]`.

- `0xfff8adbc`, `root/6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#5`, version `1.44.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xfff8adbc to 0xfff8add2 (exclusive), 22 bytes.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x8e1250, 1778776 bytes, SHA256 `562aa31da2e8d4dfbb94274fb383b078c78c25d84b5c84c747b11ed9d481a68f`.
  Payload: carrier offset 0x8e127c, 1778732 bytes, SHA256 `5d4521de0baec59a9592cee0aa871cc67cd309c85f4dd303a290e937f49dbf8c`.
  Read window: SHA256 `f296062bc1c203972a63483de8ebf268b3abcb856e2f8dd17023fde9e10c5e46`.
  Four-byte load: target `abs 0x30000bd4` (`movea.l` at 0xfff8adc0).
  Four-byte load: target `sp +0x4` (`move.l` at 0xfff8adc6).
  Byte store: target `[0x30000bd4] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xfff8adca).
  The exit (`rts` at 0xfff8add0) reports d0 = `[sp+4]`.

- `0xffe4c680`, `root/6.5.10/usr/cpu/firmware/l1.bin`.
  Package `6.5.10:l1#0`, version `1.1.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe4c680 to 0xffe4c742 (exclusive), 194 bytes.
  Carrier: 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Package: carrier offset 0x0, 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Payload: carrier offset 0x28, 961564 bytes, SHA256 `5b9d6771ee2405c5d556304b713f1a6b1deaa9bfd49009cd21856cd266eb3ef9`.
  Read window: SHA256 `57346df9eae13c3049ffcb0f2f4d24881fa585ea35b1eaa6dd28de21ba694ee0`.
  Four-byte load: target `abs 0x30101990` (`move.l` at 0xffe4c688).
  Branch (`beq.b` at 0xffe4c694): taken to 0xffe4c6ca when `[0x30101990] == 0xa1b2c3d4`, falls through to 0xffe4c696 when `[0x30101990] != 0xa1b2c3d4`.
  Branch (`beq.b` at 0xffe4c6a6): taken to 0xffe4c6ac when `Z == 1`, falls through to 0xffe4c6a8 when `not (Z == 1)`.
  Byte store: target `(unknown effective address through a5)`, value `0x0` (`clr.b` at 0xffe4c6a8).
  Jump (`bra.b` at 0xffe4c6aa): target 0xffe4c6a2.
  Four-byte store: target `abs 0x30101990`, value `0xa1b2c3d4` (`move.l` at 0xffe4c6b2).
  Two-byte store: target `abs 0x30101994`, value `0x1` (`move.w` at 0xffe4c6ba).
  Two-byte store: target `abs 0x30101996`, value `0x4d8` (`move.w` at 0xffe4c6c4).
  Four-byte load: target `abs 0x30101800` (`move.l` at 0xffe4c6ca).
  Four-byte store: target `abs 0x301019b4`, value `[0x30101800]` (`move.l` at 0xffe4c6d0).
  Grouped accesses: 3 4-byte load records with target `abs 0x30000490`, including `movea.l` at 0xffe4c6d6, `movea.l` at 0xffe4c6f2, `movea.l` at 0xffe4c6f8.
  Byte load: target `[0x30000490] +0x40` (`move.b` at 0xffe4c6dc).
  Four-byte store: target `abs 0x301019ac`, value `unknown` (`move.l` at 0xffe4c6ec).
  Byte load: target `[0x30000490] +0x40` (`move.b` at 0xffe4c6fe).
  Byte store: target `[0x30000490] +0x40`, value `unknown` (`move.b` at 0xffe4c702).
  Four-byte load: target `abs 0x301019ac` (`tst.l` at 0xffe4c706).
  Branch (`beq.b` at 0xffe4c70c): taken to 0xffe4c738 when `[0x301019ac] == 0x0`, falls through to 0xffe4c70e when `[0x301019ac] != 0x0`.
  Four-byte load: target `abs 0x301019b4` (`tst.l` at 0xffe4c70e).
  Branch (`bne.b` at 0xffe4c714): taken to 0xffe4c738 when `[0x301019b4] != 0x0`, falls through to 0xffe4c716 when `[0x301019b4] == 0x0`.
  Grouped accesses: 2 4-byte load records with target `abs 0x30101998`, including `tst.l` at 0xffe4c716, `move.l` at 0xffe4c724.
  Branch (`beq.b` at 0xffe4c71c): taken to 0xffe4c738 when `[0x30101998] == 0x0`, falls through to 0xffe4c71e when `[0x30101998] != 0x0`.
  Four-byte load: target `abs 0x3010199c` (`move.l` at 0xffe4c71e).
  Call (`bsr.w` at 0xffe4c730): (unknown callee), target 0xffe4c9e8, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  `rts` at 0xffe4c740 has recorded d0 = `unknown`.

- `0xffebc680`, `root/6.5.10/usr/cpu/firmware/l1.bin`.
  Package `6.5.10:l1#0`, version `1.1.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffebc680 to 0xffebc742 (exclusive), 194 bytes.
  Carrier: 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Package: carrier offset 0x0, 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Payload: carrier offset 0x28, 961564 bytes, SHA256 `5b9d6771ee2405c5d556304b713f1a6b1deaa9bfd49009cd21856cd266eb3ef9`.
  Read window: SHA256 `57346df9eae13c3049ffcb0f2f4d24881fa585ea35b1eaa6dd28de21ba694ee0`.
  Four-byte load: target `abs 0x30101990` (`move.l` at 0xffebc688).
  Branch (`beq.b` at 0xffebc694): taken to 0xffebc6ca when `[0x30101990] == 0xa1b2c3d4`, falls through to 0xffebc696 when `[0x30101990] != 0xa1b2c3d4`.
  Branch (`beq.b` at 0xffebc6a6): taken to 0xffebc6ac when `Z == 1`, falls through to 0xffebc6a8 when `not (Z == 1)`.
  Byte store: target `(unknown effective address through a5)`, value `0x0` (`clr.b` at 0xffebc6a8).
  Jump (`bra.b` at 0xffebc6aa): target 0xffebc6a2.
  Four-byte store: target `abs 0x30101990`, value `0xa1b2c3d4` (`move.l` at 0xffebc6b2).
  Two-byte store: target `abs 0x30101994`, value `0x1` (`move.w` at 0xffebc6ba).
  Two-byte store: target `abs 0x30101996`, value `0x4d8` (`move.w` at 0xffebc6c4).
  Four-byte load: target `abs 0x30101800` (`move.l` at 0xffebc6ca).
  Four-byte store: target `abs 0x301019b4`, value `[0x30101800]` (`move.l` at 0xffebc6d0).
  Grouped accesses: 3 4-byte load records with target `abs 0x30000490`, including `movea.l` at 0xffebc6d6, `movea.l` at 0xffebc6f2, `movea.l` at 0xffebc6f8.
  Byte load: target `[0x30000490] +0x40` (`move.b` at 0xffebc6dc).
  Four-byte store: target `abs 0x301019ac`, value `unknown` (`move.l` at 0xffebc6ec).
  Byte load: target `[0x30000490] +0x40` (`move.b` at 0xffebc6fe).
  Byte store: target `[0x30000490] +0x40`, value `unknown` (`move.b` at 0xffebc702).
  Four-byte load: target `abs 0x301019ac` (`tst.l` at 0xffebc706).
  Branch (`beq.b` at 0xffebc70c): taken to 0xffebc738 when `[0x301019ac] == 0x0`, falls through to 0xffebc70e when `[0x301019ac] != 0x0`.
  Four-byte load: target `abs 0x301019b4` (`tst.l` at 0xffebc70e).
  Branch (`bne.b` at 0xffebc714): taken to 0xffebc738 when `[0x301019b4] != 0x0`, falls through to 0xffebc716 when `[0x301019b4] == 0x0`.
  Grouped accesses: 2 4-byte load records with target `abs 0x30101998`, including `tst.l` at 0xffebc716, `move.l` at 0xffebc724.
  Branch (`beq.b` at 0xffebc71c): taken to 0xffebc738 when `[0x30101998] == 0x0`, falls through to 0xffebc71e when `[0x30101998] != 0x0`.
  Four-byte load: target `abs 0x3010199c` (`move.l` at 0xffebc71e).
  Call (`bsr.w` at 0xffebc730): (unknown callee), target 0xffebc9e8, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Recorded d0 at the return is `unknown` (`rts` at 0xffebc740).

- `0xffe53dbc`, `root/6.5.12/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.12:l1#0`, version `1.4.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe53dbc to 0xffe53e7e (exclusive), 194 bytes.
  Carrier: 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Package: carrier offset 0x0, 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Payload: carrier offset 0x28, 947796 bytes, SHA256 `7084835eb61932052dc28531fe420d397a42df805f59b49d47bec8a95f6dcc93`.
  Read window: SHA256 `292be4042b1dc7f7e31046bc01713a9f9cac126adaed583046c8ec6e5ec973d1`.
  Four-byte load: target `abs 0x30101990` (`move.l` at 0xffe53dc4).
  Branch (`beq.b` at 0xffe53dd0): taken to 0xffe53e06 when `[0x30101990] == 0xa1b2c3d4`, falls through to 0xffe53dd2 when `[0x30101990] != 0xa1b2c3d4`.
  Branch (`beq.b` at 0xffe53de2): taken to 0xffe53de8 when `Z == 1`, falls through to 0xffe53de4 when `not (Z == 1)`.
  Byte store: target `(unknown effective address through a5)`, value `0x0` (`clr.b` at 0xffe53de4).
  Jump (`bra.b` at 0xffe53de6): target 0xffe53dde.
  Four-byte store: target `abs 0x30101990`, value `0xa1b2c3d4` (`move.l` at 0xffe53dee).
  Two-byte store: target `abs 0x30101994`, value `0x1` (`move.w` at 0xffe53df6).
  Two-byte store: target `abs 0x30101996`, value `0x4d8` (`move.w` at 0xffe53e00).
  Four-byte load: target `abs 0x30101800` (`move.l` at 0xffe53e06).
  Four-byte store: target `abs 0x301019b4`, value `[0x30101800]` (`move.l` at 0xffe53e0c).
  Grouped accesses: 3 4-byte load records with target `abs 0x30000198`, including `movea.l` at 0xffe53e12, `movea.l` at 0xffe53e2e, `movea.l` at 0xffe53e34.
  Byte load: target `[0x30000198] +0x40` (`move.b` at 0xffe53e18).
  Four-byte store: target `abs 0x301019ac`, value `unknown` (`move.l` at 0xffe53e28).
  Byte load: target `[0x30000198] +0x40` (`move.b` at 0xffe53e3a).
  Byte store: target `[0x30000198] +0x40`, value `unknown` (`move.b` at 0xffe53e3e).
  Four-byte load: target `abs 0x301019ac` (`tst.l` at 0xffe53e42).
  Branch (`beq.b` at 0xffe53e48): taken to 0xffe53e74 when `[0x301019ac] == 0x0`, falls through to 0xffe53e4a when `[0x301019ac] != 0x0`.
  Four-byte load: target `abs 0x301019b4` (`tst.l` at 0xffe53e4a).
  Branch (`bne.b` at 0xffe53e50): taken to 0xffe53e74 when `[0x301019b4] != 0x0`, falls through to 0xffe53e52 when `[0x301019b4] == 0x0`.
  Grouped accesses: 2 4-byte load records with target `abs 0x30101998`, including `tst.l` at 0xffe53e52, `move.l` at 0xffe53e60.
  Branch (`beq.b` at 0xffe53e58): taken to 0xffe53e74 when `[0x30101998] == 0x0`, falls through to 0xffe53e5a when `[0x30101998] != 0x0`.
  Four-byte load: target `abs 0x3010199c` (`move.l` at 0xffe53e5a).
  Call (`bsr.w` at 0xffe53e6c): (unknown callee), target 0xffe54124, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the return (`rts` at 0xffe53e7c), d0 is `unknown`.

- `0xffec3dbc`, `root/6.5.12/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.12:l1#0`, version `1.4.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffec3dbc to 0xffec3e7e (exclusive), 194 bytes.
  Carrier: 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Package: carrier offset 0x0, 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Payload: carrier offset 0x28, 947796 bytes, SHA256 `7084835eb61932052dc28531fe420d397a42df805f59b49d47bec8a95f6dcc93`.
  Read window: SHA256 `292be4042b1dc7f7e31046bc01713a9f9cac126adaed583046c8ec6e5ec973d1`.
  Four-byte load: target `abs 0x30101990` (`move.l` at 0xffec3dc4).
  Branch (`beq.b` at 0xffec3dd0): taken to 0xffec3e06 when `[0x30101990] == 0xa1b2c3d4`, falls through to 0xffec3dd2 when `[0x30101990] != 0xa1b2c3d4`.
  Branch (`beq.b` at 0xffec3de2): taken to 0xffec3de8 when `Z == 1`, falls through to 0xffec3de4 when `not (Z == 1)`.
  Byte store: target `(unknown effective address through a5)`, value `0x0` (`clr.b` at 0xffec3de4).
  Jump (`bra.b` at 0xffec3de6): target 0xffec3dde.
  Four-byte store: target `abs 0x30101990`, value `0xa1b2c3d4` (`move.l` at 0xffec3dee).
  Two-byte store: target `abs 0x30101994`, value `0x1` (`move.w` at 0xffec3df6).
  Two-byte store: target `abs 0x30101996`, value `0x4d8` (`move.w` at 0xffec3e00).
  Four-byte load: target `abs 0x30101800` (`move.l` at 0xffec3e06).
  Four-byte store: target `abs 0x301019b4`, value `[0x30101800]` (`move.l` at 0xffec3e0c).
  Grouped accesses: 3 4-byte load records with target `abs 0x30000198`, including `movea.l` at 0xffec3e12, `movea.l` at 0xffec3e2e, `movea.l` at 0xffec3e34.
  Byte load: target `[0x30000198] +0x40` (`move.b` at 0xffec3e18).
  Four-byte store: target `abs 0x301019ac`, value `unknown` (`move.l` at 0xffec3e28).
  Byte load: target `[0x30000198] +0x40` (`move.b` at 0xffec3e3a).
  Byte store: target `[0x30000198] +0x40`, value `unknown` (`move.b` at 0xffec3e3e).
  Four-byte load: target `abs 0x301019ac` (`tst.l` at 0xffec3e42).
  Branch (`beq.b` at 0xffec3e48): taken to 0xffec3e74 when `[0x301019ac] == 0x0`, falls through to 0xffec3e4a when `[0x301019ac] != 0x0`.
  Four-byte load: target `abs 0x301019b4` (`tst.l` at 0xffec3e4a).
  Branch (`bne.b` at 0xffec3e50): taken to 0xffec3e74 when `[0x301019b4] != 0x0`, falls through to 0xffec3e52 when `[0x301019b4] == 0x0`.
  Grouped accesses: 2 4-byte load records with target `abs 0x30101998`, including `tst.l` at 0xffec3e52, `move.l` at 0xffec3e60.
  Branch (`beq.b` at 0xffec3e58): taken to 0xffec3e74 when `[0x30101998] == 0x0`, falls through to 0xffec3e5a when `[0x30101998] != 0x0`.
  Four-byte load: target `abs 0x3010199c` (`move.l` at 0xffec3e5a).
  Call (`bsr.w` at 0xffec3e6c): (unknown callee), target 0xffec4124, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  The exit (`rts` at 0xffec3e7c) reports d0 = `unknown`.

- `0xffe5a5f0`, `root/6.5.13/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.13:l1#0`, version `1.6.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe5a5f0 to 0xffe5a6b2 (exclusive), 194 bytes.
  Carrier: 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Package: carrier offset 0x0, 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Payload: carrier offset 0x28, 957064 bytes, SHA256 `6e560e68a673766d56d716333f39ae79dca4b8779b59c7c97ef02741c822611d`.
  Read window: SHA256 `736b340cc121c6246894b4dbe7eb41e0fb03ab118a4bbded097339bb62443e37`.
  Four-byte load: target `abs 0x30101990` (`move.l` at 0xffe5a5f8).
  Branch (`beq.b` at 0xffe5a604): taken to 0xffe5a63a when `[0x30101990] == 0xa1b2c3d4`, falls through to 0xffe5a606 when `[0x30101990] != 0xa1b2c3d4`.
  Branch (`beq.b` at 0xffe5a616): taken to 0xffe5a61c when `Z == 1`, falls through to 0xffe5a618 when `not (Z == 1)`.
  Byte store: target `(unknown effective address through a5)`, value `0x0` (`clr.b` at 0xffe5a618).
  Jump (`bra.b` at 0xffe5a61a): target 0xffe5a612.
  Four-byte store: target `abs 0x30101990`, value `0xa1b2c3d4` (`move.l` at 0xffe5a622).
  Two-byte store: target `abs 0x30101994`, value `0x1` (`move.w` at 0xffe5a62a).
  Two-byte store: target `abs 0x30101996`, value `0x4d8` (`move.w` at 0xffe5a634).
  Four-byte load: target `abs 0x30101800` (`move.l` at 0xffe5a63a).
  Four-byte store: target `abs 0x301019b4`, value `[0x30101800]` (`move.l` at 0xffe5a640).
  Grouped accesses: 3 4-byte load records with target `abs 0x300002ac`, including `movea.l` at 0xffe5a646, `movea.l` at 0xffe5a662, `movea.l` at 0xffe5a668.
  Byte load: target `[0x300002ac] +0x40` (`move.b` at 0xffe5a64c).
  Four-byte store: target `abs 0x301019ac`, value `unknown` (`move.l` at 0xffe5a65c).
  Byte load: target `[0x300002ac] +0x40` (`move.b` at 0xffe5a66e).
  Byte store: target `[0x300002ac] +0x40`, value `unknown` (`move.b` at 0xffe5a672).
  Four-byte load: target `abs 0x301019ac` (`tst.l` at 0xffe5a676).
  Branch (`beq.b` at 0xffe5a67c): taken to 0xffe5a6a8 when `[0x301019ac] == 0x0`, falls through to 0xffe5a67e when `[0x301019ac] != 0x0`.
  Four-byte load: target `abs 0x301019b4` (`tst.l` at 0xffe5a67e).
  Branch (`bne.b` at 0xffe5a684): taken to 0xffe5a6a8 when `[0x301019b4] != 0x0`, falls through to 0xffe5a686 when `[0x301019b4] == 0x0`.
  Grouped accesses: 2 4-byte load records with target `abs 0x30101998`, including `tst.l` at 0xffe5a686, `move.l` at 0xffe5a694.
  Branch (`beq.b` at 0xffe5a68c): taken to 0xffe5a6a8 when `[0x30101998] == 0x0`, falls through to 0xffe5a68e when `[0x30101998] != 0x0`.
  Four-byte load: target `abs 0x3010199c` (`move.l` at 0xffe5a68e).
  Call (`bsr.w` at 0xffe5a6a0): (unknown callee), target 0xffe5a958, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  `rts` at 0xffe5a6b0 has recorded d0 = `unknown`.
