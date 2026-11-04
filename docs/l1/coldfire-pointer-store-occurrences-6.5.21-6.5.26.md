# ColdFire L1 pointer-store occurrences in IRIX 6.5.21 through 6.5.26

Each entry describes its exact carrier, package, payload and selected byte position. Addresses are analysis coordinates formed from the stated payload load plus byte offset. Later positions in one payload do not establish additional unique functions, physical flash banks, runtime mapping or image selection. No byte or semantic equivalence between positions is assumed.

The spans are inferred from entry, return and bounded context evidence. They are analyzed windows, not proved function sizes. Hardware roles, pointee meanings, types, fitted silicon and runtime path feasibility remain unknown. ColdFire classification comes from package metadata and decoding uses the 68040 common subset.

`sp` is the stack pointer at the analyzed entry. The six stack slots at each call are candidate argument locations under an assumed m68k-sysv convention. Actual arity and custom conventions remain unknown. A `ret(F)` expression does not identify its producing instruction. Repeated access groups count static shapes and do not establish equal values from separate loads or one unconditional path through different branch arms. Unknown values preserve tool question marks.

The [complete facts](disasm/coldfire-variants/pointer-store-occurrences-6.5.21-6.5.26-facts.txt) retain the complete analyzed windows. Range ends are exclusive. Bounded contexts with clipped instructions or uncertain initial alignment are not proof of predecessor flow or full function extent.

- `0xfff68790`, `root/6.5.21/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.21:l1#1`, version `1.22.2`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xfff68790 to 0xfff6884c (exclusive), 188 bytes.
  Carrier: 2766272 bytes, SHA256 `d5aaba306f71d9bfe0aa83d5029e14f2b9cf7485a4c0c689297fc4f71d136fbb`.
  Package: carrier offset 0xef9d0, 1784816 bytes, SHA256 `d597012cce4081272d0987ef729afeb15d40a0de6590702331370b07e76dcfae`.
  Payload: carrier offset 0xef9fc, 1784772 bytes, SHA256 `c960b88c14cba5aaca4d5efed9325e5830948f33cb623bf8dbc9887a4b1a8c26`.
  Read window: SHA256 `235a711f7a2715acb68a7ff13a3299b3c0647377c2d4df96d664f6ad5525f34d`.
  Call (`jsr` at 0xfff68796): (unknown callee), target 0xfff547c8, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xfff6879e): (unknown callee), target 0xfff680f4, stack arg0=`ret(0xfff547c8)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xfff687a4).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xfff687aa).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xfff687ae).
  Grouped accesses: 2 4-byte load records with target `abs 0x30002524`, including `tst.l` at 0xfff687b8, `move.l` at 0xfff687f8.
  Branch (`ble.b` at 0xfff687bc): taken to 0xfff687f8 when `[0x30002524] <= 0x0 (signed, 32-bit)`, falls through to 0xfff687be when `[0x30002524] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x300004f8`, including `movea.l` at 0xfff687c4, `movea.l` at 0xfff687ce, `movea.l` at 0xfff687d4.
  Halfword store: target `[0x300004f8] +0x88`, value `0x4000` (`move.w` at 0xfff687ca).
  Word store: target `[0x300004f8] +0x8c`, value `0x0` (`clr.l` at 0xfff687d0).
  Halfword store: target `[0x300004f8] +0x92`, value `0x953` (`move.w` at 0xfff687da).
  Call (`jsr` at 0xfff687ee): (unknown callee), target 0xfff8ad30, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xfff687fe): taken to 0xfff68844 when `Z == 0`, falls through to 0xfff68800 when `not (Z == 0)`.
  Byte store: target `[0x300004f8] +0x16`, value `0x8c` (`move.b` at 0xfff6880a).
  Halfword store: target `[0x300004f8] +0xa0`, value `0x6000` (`move.w` at 0xfff68814).
  Word store: target `[0x300004f8] +0xa4`, value `0x0` (`clr.l` at 0xfff6881a).
  Halfword store: target `[0x300004f8] +0xaa`, value `0x953` (`move.w` at 0xfff68824).
  Call (`jsr` at 0xfff6883a): (unknown callee), target 0xfff8ad30, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the return (`rts` at 0xfff6884a), d0 is `unknown`.

- `0xffecdb64`, `root/6.5.22/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.22:l1#0`, version `1.24.8`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffecdb64 to 0xffecdc20 (exclusive), 188 bytes.
  Carrier: 3723916 bytes, SHA256 `5b90997945dabdc099162d5237e302472f5c3c80a808a1de5abb246c290a963b`.
  Package: carrier offset 0x0, 977968 bytes, SHA256 `5660c129f64680c9639f50d5f3d7c61428912f2270dde66185a7acccdcf64755`.
  Payload: carrier offset 0x48, 977896 bytes, SHA256 `0fe5e128ec92fb73681f4b9c7e148b1a68de5cf5199c150251c4703beef4e13f`.
  Read window: SHA256 `c49785168f4cbeeb30897cb1043b3b1da27de37daafafa4bccdb219a578c381a`.
  Call (`jsr` at 0xffecdb6a): (unknown callee), target 0xffece270, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffecdb72): (unknown callee), target 0xffecd47c, stack arg0=`ret(0xffece270)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffecdb78).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffecdb7e).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffecdb82).
  Grouped accesses: 2 4-byte load records with target `abs 0x30002890`, including `tst.l` at 0xffecdb8c, `move.l` at 0xffecdbcc.
  Branch (`ble.b` at 0xffecdb90): taken to 0xffecdbcc when `[0x30002890] <= 0x0 (signed, 32-bit)`, falls through to 0xffecdb92 when `[0x30002890] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x300008b4`, including `movea.l` at 0xffecdb98, `movea.l` at 0xffecdba2, `movea.l` at 0xffecdba8.
  Halfword store: target `[0x300008b4] +0x88`, value `0x4000` (`move.w` at 0xffecdb9e).
  Word store: target `[0x300008b4] +0x8c`, value `0x0` (`clr.l` at 0xffecdba4).
  Halfword store: target `[0x300008b4] +0x92`, value `0x953` (`move.w` at 0xffecdbae).
  Call (`jsr` at 0xffecdbc2): (unknown callee), target 0xffede95c, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffecdbd2): taken to 0xffecdc18 when `Z == 0`, falls through to 0xffecdbd4 when `not (Z == 0)`.
  Byte store: target `[0x300008b4] +0x16`, value `0x8c` (`move.b` at 0xffecdbde).
  Halfword store: target `[0x300008b4] +0xa0`, value `0x6000` (`move.w` at 0xffecdbe8).
  Word store: target `[0x300008b4] +0xa4`, value `0x0` (`clr.l` at 0xffecdbee).
  Halfword store: target `[0x300008b4] +0xaa`, value `0x953` (`move.w` at 0xffecdbf8).
  Call (`jsr` at 0xffecdc0e): (unknown callee), target 0xffede95c, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  The exit (`rts` at 0xffecdc1e) reports d0 = `unknown`.

- `0xffecc700`, `root/6.5.22/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.22:l1#1`, version `1.24.8`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffecc700 to 0xffecc7bc (exclusive), 188 bytes.
  Carrier: 3723916 bytes, SHA256 `5b90997945dabdc099162d5237e302472f5c3c80a808a1de5abb246c290a963b`.
  Package: carrier offset 0xeec30, 973412 bytes, SHA256 `8ce4c2cf2d93012d81b4ba6bc7a1deb66de83a32c5cb3052715b3a5d96fb7971`.
  Payload: carrier offset 0xeec5c, 973368 bytes, SHA256 `1640737f85acb71067aab42164e36704e6f1510cd8346bbdca7526e8a63c6b12`.
  Read window: SHA256 `c0803d67ce5f32cc653c9b597c11f3e9a694a68ab05f945c5aa1325e4bb7a89b`.
  Call (`jsr` at 0xffecc706): (unknown callee), target 0xffecce0c, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffecc70e): (unknown callee), target 0xffecc018, stack arg0=`ret(0xffecce0c)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffecc714).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffecc71a).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffecc71e).
  Grouped accesses: 2 4-byte load records with target `abs 0x30002948`, including `tst.l` at 0xffecc728, `move.l` at 0xffecc768.
  Branch (`ble.b` at 0xffecc72c): taken to 0xffecc768 when `[0x30002948] <= 0x0 (signed, 32-bit)`, falls through to 0xffecc72e when `[0x30002948] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x3000094c`, including `movea.l` at 0xffecc734, `movea.l` at 0xffecc73e, `movea.l` at 0xffecc744.
  Halfword store: target `[0x3000094c] +0x88`, value `0x4000` (`move.w` at 0xffecc73a).
  Word store: target `[0x3000094c] +0x8c`, value `0x0` (`clr.l` at 0xffecc740).
  Halfword store: target `[0x3000094c] +0x92`, value `0x953` (`move.w` at 0xffecc74a).
  Call (`jsr` at 0xffecc75e): (unknown callee), target 0xffedd6fc, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffecc76e): taken to 0xffecc7b4 when `Z == 0`, falls through to 0xffecc770 when `not (Z == 0)`.
  Byte store: target `[0x3000094c] +0x16`, value `0x8c` (`move.b` at 0xffecc77a).
  Halfword store: target `[0x3000094c] +0xa0`, value `0x6000` (`move.w` at 0xffecc784).
  Word store: target `[0x3000094c] +0xa4`, value `0x0` (`clr.l` at 0xffecc78a).
  Halfword store: target `[0x3000094c] +0xaa`, value `0x953` (`move.w` at 0xffecc794).
  Call (`jsr` at 0xffecc7aa): (unknown callee), target 0xffedd6fc, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  `rts` at 0xffecc7ba has recorded d0 = `unknown`.

- `0xfff6f234`, `root/6.5.22/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.22:l1#2`, version `1.24.8`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xfff6f234 to 0xfff6f2f0 (exclusive), 188 bytes.
  Carrier: 3723916 bytes, SHA256 `5b90997945dabdc099162d5237e302472f5c3c80a808a1de5abb246c290a963b`.
  Package: carrier offset 0x1dc694, 1772536 bytes, SHA256 `6d06a281a09291d82228d78cfc1c0f2419eff91a940d388bf6bc3b406fbf32ff`.
  Payload: carrier offset 0x1dc6c0, 1772492 bytes, SHA256 `953894c5c4e86f03e78ada263bebd12e0e7af316fe0733a8bc606f2d94ce66da`.
  Read window: SHA256 `56bdc9d4303ceb1c210a63996e0302c634a0d65d1ccca48bd66c759ef3c1fcc2`.
  Call (`jsr` at 0xfff6f23a): (unknown callee), target 0xfff6f940, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xfff6f242): (unknown callee), target 0xfff6eb4c, stack arg0=`ret(0xfff6f940)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xfff6f248).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xfff6f24e).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xfff6f252).
  Grouped accesses: 2 4-byte load records with target `abs 0x30002e80`, including `tst.l` at 0xfff6f25c, `move.l` at 0xfff6f29c.
  Branch (`ble.b` at 0xfff6f260): taken to 0xfff6f29c when `[0x30002e80] <= 0x0 (signed, 32-bit)`, falls through to 0xfff6f262 when `[0x30002e80] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000b08`, including `movea.l` at 0xfff6f268, `movea.l` at 0xfff6f272, `movea.l` at 0xfff6f278.
  Halfword store: target `[0x30000b08] +0x88`, value `0x4000` (`move.w` at 0xfff6f26e).
  Word store: target `[0x30000b08] +0x8c`, value `0x0` (`clr.l` at 0xfff6f274).
  Halfword store: target `[0x30000b08] +0x92`, value `0x953` (`move.w` at 0xfff6f27e).
  Call (`jsr` at 0xfff6f292): (unknown callee), target 0xfff894d8, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xfff6f2a2): taken to 0xfff6f2e8 when `Z == 0`, falls through to 0xfff6f2a4 when `not (Z == 0)`.
  Byte store: target `[0x30000b08] +0x16`, value `0x8c` (`move.b` at 0xfff6f2ae).
  Halfword store: target `[0x30000b08] +0xa0`, value `0x6000` (`move.w` at 0xfff6f2b8).
  Word store: target `[0x30000b08] +0xa4`, value `0x0` (`clr.l` at 0xfff6f2be).
  Halfword store: target `[0x30000b08] +0xaa`, value `0x953` (`move.w` at 0xfff6f2c8).
  Call (`jsr` at 0xfff6f2de): (unknown callee), target 0xfff894d8, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Recorded d0 at the return is `unknown` (`rts` at 0xfff6f2ee).

- `0xffecf62c`, `root/6.5.23/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.23:l1#0`, version `1.26.5`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffecf62c to 0xffecf6e8 (exclusive), 188 bytes.
  Carrier: 3763312 bytes, SHA256 `15fe4284aa152ba30d1a8d9c67f63ee2acf1d19b0653068befc89146c12187f1`.
  Package: carrier offset 0x0, 982364 bytes, SHA256 `01f4157793ada930d8f671157cdd27fad86188a92d4a4c32b5c2c8133fcd243b`.
  Payload: carrier offset 0x48, 982292 bytes, SHA256 `85d56d8030146394e6339ca8e34f99d091e9588ecd9a752759a35358c554e5e8`.
  Read window: SHA256 `9fd962ec309eb6eeb6e1031778189d88d468e7299ea0e1c6dc8bd510a2a95140`.
  Call (`jsr` at 0xffecf632): (unknown callee), target 0xffec7f64, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffecf63a): (unknown callee), target 0xffecef44, stack arg0=`ret(0xffec7f64)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffecf640).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffecf646).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffecf64a).
  Grouped accesses: 2 4-byte load records with target `abs 0x300021d8`, including `tst.l` at 0xffecf654, `move.l` at 0xffecf694.
  Branch (`ble.b` at 0xffecf658): taken to 0xffecf694 when `[0x300021d8] <= 0x0 (signed, 32-bit)`, falls through to 0xffecf65a when `[0x300021d8] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000808`, including `movea.l` at 0xffecf660, `movea.l` at 0xffecf66a, `movea.l` at 0xffecf670.
  Halfword store: target `[0x30000808] +0x88`, value `0x4000` (`move.w` at 0xffecf666).
  Word store: target `[0x30000808] +0x8c`, value `0x0` (`clr.l` at 0xffecf66c).
  Halfword store: target `[0x30000808] +0x92`, value `0x953` (`move.w` at 0xffecf676).
  Call (`jsr` at 0xffecf68a): (unknown callee), target 0xffedd428, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffecf69a): taken to 0xffecf6e0 when `Z == 0`, falls through to 0xffecf69c when `not (Z == 0)`.
  Byte store: target `[0x30000808] +0x16`, value `0x8c` (`move.b` at 0xffecf6a6).
  Halfword store: target `[0x30000808] +0xa0`, value `0x6000` (`move.w` at 0xffecf6b0).
  Word store: target `[0x30000808] +0xa4`, value `0x0` (`clr.l` at 0xffecf6b6).
  Halfword store: target `[0x30000808] +0xaa`, value `0x953` (`move.w` at 0xffecf6c0).
  Call (`jsr` at 0xffecf6d6): (unknown callee), target 0xffedd428, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the return (`rts` at 0xffecf6e6), d0 is `unknown`.

- `0xffecef40`, `root/6.5.23/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.23:l1#1`, version `1.26.5`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffecef40 to 0xffeceffc (exclusive), 188 bytes.
  Carrier: 3763312 bytes, SHA256 `15fe4284aa152ba30d1a8d9c67f63ee2acf1d19b0653068befc89146c12187f1`.
  Package: carrier offset 0xefd5c, 981328 bytes, SHA256 `aea0366c69ecffb773f22102af9e5d2696ec1eff91925377f388345d468380a0`.
  Payload: carrier offset 0xefd88, 981284 bytes, SHA256 `1c9aeb0a2af5bb40c97a727427e01cc159613434ed1a7a670b2dd81566888cf4`.
  Read window: SHA256 `287ff918ca6d3070f4d3725726c1f6de4b1c3d7c7b850f9264cd41e379d3c954`.
  Call (`jsr` at 0xffecef46): (unknown callee), target 0xffec7878, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffecef4e): (unknown callee), target 0xffece858, stack arg0=`ret(0xffec7878)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffecef54).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffecef5a).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffecef5e).
  Grouped accesses: 2 4-byte load records with target `abs 0x30002348`, including `tst.l` at 0xffecef68, `move.l` at 0xffecefa8.
  Branch (`ble.b` at 0xffecef6c): taken to 0xffecefa8 when `[0x30002348] <= 0x0 (signed, 32-bit)`, falls through to 0xffecef6e when `[0x30002348] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x300008b0`, including `movea.l` at 0xffecef74, `movea.l` at 0xffecef7e, `movea.l` at 0xffecef84.
  Halfword store: target `[0x300008b0] +0x88`, value `0x4000` (`move.w` at 0xffecef7a).
  Word store: target `[0x300008b0] +0x8c`, value `0x0` (`clr.l` at 0xffecef80).
  Halfword store: target `[0x300008b0] +0x92`, value `0x953` (`move.w` at 0xffecef8a).
  Call (`jsr` at 0xffecef9e): (unknown callee), target 0xffedcf40, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffecefae): taken to 0xffeceff4 when `Z == 0`, falls through to 0xffecefb0 when `not (Z == 0)`.
  Byte store: target `[0x300008b0] +0x16`, value `0x8c` (`move.b` at 0xffecefba).
  Halfword store: target `[0x300008b0] +0xa0`, value `0x6000` (`move.w` at 0xffecefc4).
  Word store: target `[0x300008b0] +0xa4`, value `0x0` (`clr.l` at 0xffecefca).
  Halfword store: target `[0x300008b0] +0xaa`, value `0x953` (`move.w` at 0xffecefd4).
  Call (`jsr` at 0xffecefea): (unknown callee), target 0xffedcf40, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  The exit (`rts` at 0xffeceffa) reports d0 = `unknown`.

- `0xfff72778`, `root/6.5.23/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.23:l1#2`, version `1.26.5`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xfff72778 to 0xfff72834 (exclusive), 188 bytes.
  Carrier: 3763312 bytes, SHA256 `15fe4284aa152ba30d1a8d9c67f63ee2acf1d19b0653068befc89146c12187f1`.
  Package: carrier offset 0x1df6ac, 1799620 bytes, SHA256 `f8faa30fbd2a62abf34dc81f249e3eced855814e03126cd61f23787233c72db5`.
  Payload: carrier offset 0x1df6d8, 1799576 bytes, SHA256 `92705d72b6589449f4852e8d45d40a995d5c54909f9de62af36ea3b5aeb28414`.
  Read window: SHA256 `06d73bc4b5b011c788126571c2788e22c960aff800bf599f7c8f918219ed78ec`.
  Call (`jsr` at 0xfff7277e): (unknown callee), target 0xfff6b0b0, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xfff72786): (unknown callee), target 0xfff72090, stack arg0=`ret(0xfff6b0b0)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xfff7278c).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xfff72792).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xfff72796).
  Grouped accesses: 2 4-byte load records with target `abs 0x30002b40`, including `tst.l` at 0xfff727a0, `move.l` at 0xfff727e0.
  Branch (`ble.b` at 0xfff727a4): taken to 0xfff727e0 when `[0x30002b40] <= 0x0 (signed, 32-bit)`, falls through to 0xfff727a6 when `[0x30002b40] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000afc`, including `movea.l` at 0xfff727ac, `movea.l` at 0xfff727b6, `movea.l` at 0xfff727bc.
  Halfword store: target `[0x30000afc] +0x88`, value `0x4000` (`move.w` at 0xfff727b2).
  Word store: target `[0x30000afc] +0x8c`, value `0x0` (`clr.l` at 0xfff727b8).
  Halfword store: target `[0x30000afc] +0x92`, value `0x953` (`move.w` at 0xfff727c2).
  Call (`jsr` at 0xfff727d6): (unknown callee), target 0xfff8d214, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xfff727e6): taken to 0xfff7282c when `Z == 0`, falls through to 0xfff727e8 when `not (Z == 0)`.
  Byte store: target `[0x30000afc] +0x16`, value `0x8c` (`move.b` at 0xfff727f2).
  Halfword store: target `[0x30000afc] +0xa0`, value `0x6000` (`move.w` at 0xfff727fc).
  Word store: target `[0x30000afc] +0xa4`, value `0x0` (`clr.l` at 0xfff72802).
  Halfword store: target `[0x30000afc] +0xaa`, value `0x953` (`move.w` at 0xfff7280c).
  Call (`jsr` at 0xfff72822): (unknown callee), target 0xfff8d214, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  `rts` at 0xfff72832 has recorded d0 = `unknown`.

- `0xffecefbc`, `root/6.5.24/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.24:l1#0`, version `1.28.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffecefbc to 0xffecf078 (exclusive), 188 bytes.
  Carrier: 3759416 bytes, SHA256 `12e1052718a311de3dfd6424820e680fbaedace11a249c463a1ff5bbca2236c1`.
  Package: carrier offset 0x0, 980132 bytes, SHA256 `743c2e9361fcdd2134ae3d68a4773694a6909e4016afbc25530506172432cbd5`.
  Payload: carrier offset 0x48, 980060 bytes, SHA256 `edb297ad427641845d4abf1ffb2c0e82e81e5b26f56cd2c1e3d888352a34fd50`.
  Read window: SHA256 `244719180e4c93cdba59b3dadae85d3fa1e143b9953e2886d9ecfd29bff8bbed`.
  Call (`jsr` at 0xffecefc2): (unknown callee), target 0xffec78d4, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffecefca): (unknown callee), target 0xffece8d4, stack arg0=`ret(0xffec78d4)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffecefd0).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffecefd6).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffecefda).
  Grouped accesses: 2 4-byte load records with target `abs 0x300021d8`, including `tst.l` at 0xffecefe4, `move.l` at 0xffecf024.
  Branch (`ble.b` at 0xffecefe8): taken to 0xffecf024 when `[0x300021d8] <= 0x0 (signed, 32-bit)`, falls through to 0xffecefea when `[0x300021d8] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000808`, including `movea.l` at 0xffeceff0, `movea.l` at 0xffeceffa, `movea.l` at 0xffecf000.
  Halfword store: target `[0x30000808] +0x88`, value `0x4000` (`move.w` at 0xffeceff6).
  Word store: target `[0x30000808] +0x8c`, value `0x0` (`clr.l` at 0xffeceffc).
  Halfword store: target `[0x30000808] +0x92`, value `0x953` (`move.w` at 0xffecf006).
  Call (`jsr` at 0xffecf01a): (unknown callee), target 0xffedcd48, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffecf02a): taken to 0xffecf070 when `Z == 0`, falls through to 0xffecf02c when `not (Z == 0)`.
  Byte store: target `[0x30000808] +0x16`, value `0x8c` (`move.b` at 0xffecf036).
  Halfword store: target `[0x30000808] +0xa0`, value `0x6000` (`move.w` at 0xffecf040).
  Word store: target `[0x30000808] +0xa4`, value `0x0` (`clr.l` at 0xffecf046).
  Halfword store: target `[0x30000808] +0xaa`, value `0x953` (`move.w` at 0xffecf050).
  Call (`jsr` at 0xffecf066): (unknown callee), target 0xffedcd48, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Recorded d0 at the return is `unknown` (`rts` at 0xffecf076).

- `0xffece8d4`, `root/6.5.24/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.24:l1#1`, version `1.28.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffece8d4 to 0xffece990 (exclusive), 188 bytes.
  Carrier: 3759416 bytes, SHA256 `12e1052718a311de3dfd6424820e680fbaedace11a249c463a1ff5bbca2236c1`.
  Package: carrier offset 0xef4a4, 979096 bytes, SHA256 `1f3ca14b06175e39d38af6315bef38a6ab31ff2178a5cfd05943593bf37f64a7`.
  Payload: carrier offset 0xef4d0, 979052 bytes, SHA256 `72ad6ba3005bd3a464774c90dcfe60e333e18e06d10812724c72c2ddd62a0351`.
  Read window: SHA256 `8b1127c0259029876264caa5ff1ec0e8ec8fc4f5ee3fd5c45ec8f067fb7c57e8`.
  Call (`jsr` at 0xffece8da): (unknown callee), target 0xffec71ec, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffece8e2): (unknown callee), target 0xffece1ec, stack arg0=`ret(0xffec71ec)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffece8e8).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffece8ee).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffece8f2).
  Grouped accesses: 2 4-byte load records with target `abs 0x30002348`, including `tst.l` at 0xffece8fc, `move.l` at 0xffece93c.
  Branch (`ble.b` at 0xffece900): taken to 0xffece93c when `[0x30002348] <= 0x0 (signed, 32-bit)`, falls through to 0xffece902 when `[0x30002348] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x300008b0`, including `movea.l` at 0xffece908, `movea.l` at 0xffece912, `movea.l` at 0xffece918.
  Halfword store: target `[0x300008b0] +0x88`, value `0x4000` (`move.w` at 0xffece90e).
  Word store: target `[0x300008b0] +0x8c`, value `0x0` (`clr.l` at 0xffece914).
  Halfword store: target `[0x300008b0] +0x92`, value `0x953` (`move.w` at 0xffece91e).
  Call (`jsr` at 0xffece932): (unknown callee), target 0xffedc864, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffece942): taken to 0xffece988 when `Z == 0`, falls through to 0xffece944 when `not (Z == 0)`.
  Byte store: target `[0x300008b0] +0x16`, value `0x8c` (`move.b` at 0xffece94e).
  Halfword store: target `[0x300008b0] +0xa0`, value `0x6000` (`move.w` at 0xffece958).
  Word store: target `[0x300008b0] +0xa4`, value `0x0` (`clr.l` at 0xffece95e).
  Halfword store: target `[0x300008b0] +0xaa`, value `0x953` (`move.w` at 0xffece968).
  Call (`jsr` at 0xffece97e): (unknown callee), target 0xffedc864, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the return (`rts` at 0xffece98e), d0 is `unknown`.

- `0xfff72cec`, `root/6.5.24/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.24:l1#2`, version `1.28.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xfff72cec to 0xfff72da8 (exclusive), 188 bytes.
  Carrier: 3759416 bytes, SHA256 `12e1052718a311de3dfd6424820e680fbaedace11a249c463a1ff5bbca2236c1`.
  Package: carrier offset 0x1de53c, 1800188 bytes, SHA256 `391a349a2940f18047f404a69cbf7641e25c67c2d7739e3739ec54e473b9d6a7`.
  Payload: carrier offset 0x1de568, 1800144 bytes, SHA256 `1f088a070cf4398029a936d96860ad27d7e2bf2ed2ad35b871f07d84398be8c5`.
  Read window: SHA256 `d01ad8460c6b46fd9c3a082515260f27c24f5b93b821f351a910f993c488b5c5`.
  Call (`jsr` at 0xfff72cf2): (unknown callee), target 0xfff6b604, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xfff72cfa): (unknown callee), target 0xfff72604, stack arg0=`ret(0xfff6b604)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xfff72d00).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xfff72d06).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xfff72d0a).
  Grouped accesses: 2 4-byte load records with target `abs 0x30002b58`, including `tst.l` at 0xfff72d14, `move.l` at 0xfff72d54.
  Branch (`ble.b` at 0xfff72d18): taken to 0xfff72d54 when `[0x30002b58] <= 0x0 (signed, 32-bit)`, falls through to 0xfff72d1a when `[0x30002b58] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000b18`, including `movea.l` at 0xfff72d20, `movea.l` at 0xfff72d2a, `movea.l` at 0xfff72d30.
  Halfword store: target `[0x30000b18] +0x88`, value `0x4000` (`move.w` at 0xfff72d26).
  Word store: target `[0x30000b18] +0x8c`, value `0x0` (`clr.l` at 0xfff72d2c).
  Halfword store: target `[0x30000b18] +0x92`, value `0x953` (`move.w` at 0xfff72d36).
  Call (`jsr` at 0xfff72d4a): (unknown callee), target 0xfff8d5e8, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xfff72d5a): taken to 0xfff72da0 when `Z == 0`, falls through to 0xfff72d5c when `not (Z == 0)`.
  Byte store: target `[0x30000b18] +0x16`, value `0x8c` (`move.b` at 0xfff72d66).
  Halfword store: target `[0x30000b18] +0xa0`, value `0x6000` (`move.w` at 0xfff72d70).
  Word store: target `[0x30000b18] +0xa4`, value `0x0` (`clr.l` at 0xfff72d76).
  Halfword store: target `[0x30000b18] +0xaa`, value `0x953` (`move.w` at 0xfff72d80).
  Call (`jsr` at 0xfff72d96): (unknown callee), target 0xfff8d5e8, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  The exit (`rts` at 0xfff72da6) reports d0 = `unknown`.

- `0xffec72e8`, `root/6.5.25/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.25:l1#0`, version `1.30.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffec72e8 to 0xffec73a4 (exclusive), 188 bytes.
  Carrier: 3817584 bytes, SHA256 `239378307da84254c56ed7499818aa372fe0aabd54e668da931da34f13ac620d`.
  Package: carrier offset 0x0, 979900 bytes, SHA256 `bc2358995619da07a0363c20e1ed39794146b9ad478b1b3e5068aca530c3dfc9`.
  Payload: carrier offset 0x48, 979828 bytes, SHA256 `a13d2aa10e7903bf53ab9b8395c23d4ebcf47eb3c09bb126cabfdf3f423e8b98`.
  Read window: SHA256 `306aae48705e6e5aba0ca89f8cc897c41e1718211a7934911d00f0f05d24dcfb`.
  Call (`jsr` at 0xffec72ee): (unknown callee), target 0xffec79bc, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffec72f6): (unknown callee), target 0xffec6c00, stack arg0=`ret(0xffec79bc)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffec72fc).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffec7302).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffec7306).
  Grouped accesses: 2 4-byte load records with target `abs 0x30001e38`, including `tst.l` at 0xffec7310, `move.l` at 0xffec7350.
  Branch (`ble.b` at 0xffec7314): taken to 0xffec7350 when `[0x30001e38] <= 0x0 (signed, 32-bit)`, falls through to 0xffec7316 when `[0x30001e38] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000320`, including `movea.l` at 0xffec731c, `movea.l` at 0xffec7326, `movea.l` at 0xffec732c.
  Halfword store: target `[0x30000320] +0x88`, value `0x4000` (`move.w` at 0xffec7322).
  Word store: target `[0x30000320] +0x8c`, value `0x0` (`clr.l` at 0xffec7328).
  Halfword store: target `[0x30000320] +0x92`, value `0x953` (`move.w` at 0xffec7332).
  Call (`jsr` at 0xffec7346): (unknown callee), target 0xffedd238, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffec7356): taken to 0xffec739c when `Z == 0`, falls through to 0xffec7358 when `not (Z == 0)`.
  Byte store: target `[0x30000320] +0x16`, value `0x8c` (`move.b` at 0xffec7362).
  Halfword store: target `[0x30000320] +0xa0`, value `0x6000` (`move.w` at 0xffec736c).
  Word store: target `[0x30000320] +0xa4`, value `0x0` (`clr.l` at 0xffec7372).
  Halfword store: target `[0x30000320] +0xaa`, value `0x953` (`move.w` at 0xffec737c).
  Call (`jsr` at 0xffec7392): (unknown callee), target 0xffedd238, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  `rts` at 0xffec73a2 has recorded d0 = `unknown`.

- `0xffec70fc`, `root/6.5.25/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.25:l1#1`, version `1.30.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffec70fc to 0xffec71b8 (exclusive), 188 bytes.
  Carrier: 3817584 bytes, SHA256 `239378307da84254c56ed7499818aa372fe0aabd54e668da931da34f13ac620d`.
  Package: carrier offset 0xef3bc, 979984 bytes, SHA256 `0c31235ae9a306523cbb55de14016bc052fa2f4ae889defc8b6de3099e19c112`.
  Payload: carrier offset 0xef3e8, 979940 bytes, SHA256 `d592fe64ddd586ff8ffef2cea25d5e55e38c41114174b22d106a211524055ef0`.
  Read window: SHA256 `9dabc700d7e55dffbdb492aa61d19718bcc576d89a42af737341c43e5d4231fb`.
  Call (`jsr` at 0xffec7102): (unknown callee), target 0xffec77d0, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffec710a): (unknown callee), target 0xffec6a14, stack arg0=`ret(0xffec77d0)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffec7110).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffec7116).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffec711a).
  Grouped accesses: 2 4-byte load records with target `abs 0x30001f30`, including `tst.l` at 0xffec7124, `move.l` at 0xffec7164.
  Branch (`ble.b` at 0xffec7128): taken to 0xffec7164 when `[0x30001f30] <= 0x0 (signed, 32-bit)`, falls through to 0xffec712a when `[0x30001f30] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000354`, including `movea.l` at 0xffec7130, `movea.l` at 0xffec713a, `movea.l` at 0xffec7140.
  Halfword store: target `[0x30000354] +0x88`, value `0x4000` (`move.w` at 0xffec7136).
  Word store: target `[0x30000354] +0x8c`, value `0x0` (`clr.l` at 0xffec713c).
  Halfword store: target `[0x30000354] +0x92`, value `0x953` (`move.w` at 0xffec7146).
  Call (`jsr` at 0xffec715a): (unknown callee), target 0xffedd250, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffec716a): taken to 0xffec71b0 when `Z == 0`, falls through to 0xffec716c when `not (Z == 0)`.
  Byte store: target `[0x30000354] +0x16`, value `0x8c` (`move.b` at 0xffec7176).
  Halfword store: target `[0x30000354] +0xa0`, value `0x6000` (`move.w` at 0xffec7180).
  Word store: target `[0x30000354] +0xa4`, value `0x0` (`clr.l` at 0xffec7186).
  Halfword store: target `[0x30000354] +0xaa`, value `0x953` (`move.w` at 0xffec7190).
  Call (`jsr` at 0xffec71a6): (unknown callee), target 0xffedd250, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Recorded d0 at the return is `unknown` (`rts` at 0xffec71b6).

- `0xfff77028`, `root/6.5.25/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.25:l1#2`, version `1.30.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xfff77028 to 0xfff770e4 (exclusive), 188 bytes.
  Carrier: 3817584 bytes, SHA256 `239378307da84254c56ed7499818aa372fe0aabd54e668da931da34f13ac620d`.
  Package: carrier offset 0x1de7cc, 1857700 bytes, SHA256 `718b73d44aa415958aa1899cc03977a0fc5c1f659184012a4c2c33771cd500e3`.
  Payload: carrier offset 0x1de7f8, 1857656 bytes, SHA256 `01ddeae2fbe3704ee85ad90d827a6325a0ada649e91d7dfc593a1a14f80e07b5`.
  Read window: SHA256 `ca725e19da116d5f0cb82a3dfb9b7b1f183c85aff3505dce413df45f052d961c`.
  Call (`jsr` at 0xfff7702e): (unknown callee), target 0xfff776fc, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xfff77036): (unknown callee), target 0xfff76940, stack arg0=`ret(0xfff776fc)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xfff7703c).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xfff77042).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xfff77046).
  Grouped accesses: 2 4-byte load records with target `abs 0x300025b0`, including `tst.l` at 0xfff77050, `move.l` at 0xfff77090.
  Branch (`ble.b` at 0xfff77054): taken to 0xfff77090 when `[0x300025b0] <= 0x0 (signed, 32-bit)`, falls through to 0xfff77056 when `[0x300025b0] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x300004b0`, including `movea.l` at 0xfff7705c, `movea.l` at 0xfff77066, `movea.l` at 0xfff7706c.
  Halfword store: target `[0x300004b0] +0x88`, value `0x4000` (`move.w` at 0xfff77062).
  Word store: target `[0x300004b0] +0x8c`, value `0x0` (`clr.l` at 0xfff77068).
  Halfword store: target `[0x300004b0] +0x92`, value `0x953` (`move.w` at 0xfff77072).
  Call (`jsr` at 0xfff77086): (unknown callee), target 0xfff9aafc, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xfff77096): taken to 0xfff770dc when `Z == 0`, falls through to 0xfff77098 when `not (Z == 0)`.
  Byte store: target `[0x300004b0] +0x16`, value `0x8c` (`move.b` at 0xfff770a2).
  Halfword store: target `[0x300004b0] +0xa0`, value `0x6000` (`move.w` at 0xfff770ac).
  Word store: target `[0x300004b0] +0xa4`, value `0x0` (`clr.l` at 0xfff770b2).
  Halfword store: target `[0x300004b0] +0xaa`, value `0x953` (`move.w` at 0xfff770bc).
  Call (`jsr` at 0xfff770d2): (unknown callee), target 0xfff9aafc, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the return (`rts` at 0xfff770e2), d0 is `unknown`.

- `0xffecd208`, `root/6.5.26/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.26:l1#0`, version `1.32.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffecd208 to 0xffecd2c4 (exclusive), 188 bytes.
  Carrier: 3846136 bytes, SHA256 `1fdcc3409f4e0633335d9b51d506818ce971dacbe6ababe1d4fbcbd52855e3dc`.
  Package: carrier offset 0x0, 978796 bytes, SHA256 `69539d85355ecdbbf84627ba00e20cfbc7662fbbebb071c8ac525a43150aaa0d`.
  Payload: carrier offset 0x48, 978724 bytes, SHA256 `92a0694b91b363f898d9d22a3072fc2f9932830b683e2b071f3df45f7329f07e`.
  Read window: SHA256 `2d95325a47d5a4a558aa656b6aca1e7a6c6e65d035bb08647b7fa8679922213b`.
  Call (`jsr` at 0xffecd20e): (unknown callee), target 0xffecdb74, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffecd216): (unknown callee), target 0xffeccb20, stack arg0=`ret(0xffecdb74)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffecd21c).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffecd222).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffecd226).
  Grouped accesses: 2 4-byte load records with target `abs 0x30002584`, including `tst.l` at 0xffecd230, `move.l` at 0xffecd270.
  Branch (`ble.b` at 0xffecd234): taken to 0xffecd270 when `[0x30002584] <= 0x0 (signed, 32-bit)`, falls through to 0xffecd236 when `[0x30002584] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x300003f8`, including `movea.l` at 0xffecd23c, `movea.l` at 0xffecd246, `movea.l` at 0xffecd24c.
  Halfword store: target `[0x300003f8] +0x88`, value `0x4000` (`move.w` at 0xffecd242).
  Word store: target `[0x300003f8] +0x8c`, value `0x0` (`clr.l` at 0xffecd248).
  Halfword store: target `[0x300003f8] +0x92`, value `0x953` (`move.w` at 0xffecd252).
  Call (`jsr` at 0xffecd266): (unknown callee), target 0xffedc9f4, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffecd276): taken to 0xffecd2bc when `Z == 0`, falls through to 0xffecd278 when `not (Z == 0)`.
  Byte store: target `[0x300003f8] +0x16`, value `0x8c` (`move.b` at 0xffecd282).
  Halfword store: target `[0x300003f8] +0xa0`, value `0x6000` (`move.w` at 0xffecd28c).
  Word store: target `[0x300003f8] +0xa4`, value `0x0` (`clr.l` at 0xffecd292).
  Halfword store: target `[0x300003f8] +0xaa`, value `0x953` (`move.w` at 0xffecd29c).
  Call (`jsr` at 0xffecd2b2): (unknown callee), target 0xffedc9f4, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  The exit (`rts` at 0xffecd2c2) reports d0 = `unknown`.

- `0xffecdbb0`, `root/6.5.26/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.26:l1#1`, version `1.32.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffecdbb0 to 0xffecdc6c (exclusive), 188 bytes.
  Carrier: 3846136 bytes, SHA256 `1fdcc3409f4e0633335d9b51d506818ce971dacbe6ababe1d4fbcbd52855e3dc`.
  Package: carrier offset 0xeef6c, 982112 bytes, SHA256 `81744f9b80dc588fc8c607763d38aa7fe926adf437202b23befbd9932ec14368`.
  Payload: carrier offset 0xeef98, 982068 bytes, SHA256 `fe1b451b03bf7c29e2e09011317c999cba37b2ea2c8bd89f316497d756cc56a9`.
  Read window: SHA256 `3e268b7b7b9110568a37baf92061be333d6e7ee7c82d1fe125f2804e1fa71727`.
  Call (`jsr` at 0xffecdbb6): (unknown callee), target 0xffece51c, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffecdbbe): (unknown callee), target 0xffecd4c8, stack arg0=`ret(0xffece51c)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffecdbc4).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffecdbca).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffecdbce).
  Grouped accesses: 2 4-byte load records with target `abs 0x30002690`, including `tst.l` at 0xffecdbd8, `move.l` at 0xffecdc18.
  Branch (`ble.b` at 0xffecdbdc): taken to 0xffecdc18 when `[0x30002690] <= 0x0 (signed, 32-bit)`, falls through to 0xffecdbde when `[0x30002690] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000434`, including `movea.l` at 0xffecdbe4, `movea.l` at 0xffecdbee, `movea.l` at 0xffecdbf4.
  Halfword store: target `[0x30000434] +0x88`, value `0x4000` (`move.w` at 0xffecdbea).
  Word store: target `[0x30000434] +0x8c`, value `0x0` (`clr.l` at 0xffecdbf0).
  Halfword store: target `[0x30000434] +0x92`, value `0x953` (`move.w` at 0xffecdbfa).
  Call (`jsr` at 0xffecdc0e): (unknown callee), target 0xffedd594, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffecdc1e): taken to 0xffecdc64 when `Z == 0`, falls through to 0xffecdc20 when `not (Z == 0)`.
  Byte store: target `[0x30000434] +0x16`, value `0x8c` (`move.b` at 0xffecdc2a).
  Halfword store: target `[0x30000434] +0xa0`, value `0x6000` (`move.w` at 0xffecdc34).
  Word store: target `[0x30000434] +0xa4`, value `0x0` (`clr.l` at 0xffecdc3a).
  Halfword store: target `[0x30000434] +0xaa`, value `0x953` (`move.w` at 0xffecdc44).
  Call (`jsr` at 0xffecdc5a): (unknown callee), target 0xffedd594, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  `rts` at 0xffecdc6a has recorded d0 = `unknown`.
