# ColdFire L1 pointer-store occurrences in IRIX 6.5.9 through 6.5.22

Each entry describes its exact carrier, package, payload and selected byte position. Addresses are analysis coordinates formed from the stated payload load plus byte offset. Later positions in one payload do not establish additional unique functions, physical flash banks, runtime mapping or image selection. No byte or semantic equivalence between positions is assumed.

The spans are inferred from entry, return and bounded context evidence. They are analyzed windows, not proved function sizes. Hardware roles, pointee meanings, types, fitted silicon and runtime path feasibility remain unknown. ColdFire classification comes from package metadata and decoding uses the 68040 common subset.

`sp` is the stack pointer at the analyzed entry. The six stack slots at each call are candidate argument locations under an assumed m68k-sysv convention. Actual arity and custom conventions remain unknown. A `ret(F)` expression does not identify its producing instruction. Repeated access groups count static shapes and do not establish equal values from separate loads or one unconditional path through different branch arms. Unknown values preserve tool question marks.

The [complete facts](disasm/coldfire-variants/pointer-store-occurrences-6.5.9-6.5.22-facts.txt) retain the complete analyzed windows. Range ends are exclusive. Bounded contexts with clipped instructions or uncertain initial alignment are not proof of predecessor flow or full function extent.

- `0xffe56abc`, `root/6.5.19/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.19:l1#0`, version `1.18.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe56abc to 0xffe56b78 (exclusive), 188 bytes.
  Carrier: 2668016 bytes, SHA256 `9b9fbe440979d0082cc1c41dfa89fcf57136493f359b55aedcded65f2827d562`.
  Package: carrier offset 0x0, 977052 bytes, SHA256 `389a486d2213168fcde72fafaa1bd9efe5bbbde06e52e1f023f5dd76900537c5`.
  Payload: carrier offset 0x40, 976988 bytes, SHA256 `9b636fd9c1b5fe130de21c2db5a40eb3fcd6fc0452887f2ddc8a015d9fad521d`.
  Read window: SHA256 `2df9bb772f360042be91fc0d585a42e98ba8a7f432decad4898ca2c597af8358`.
  Call (`jsr` at 0xffe56ac2): (unknown callee), target 0xffe4c4e8, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe56aca): (unknown callee), target 0xffe56488, stack arg0=`ret(0xffe4c4e8)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe56ad0).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe56ad6).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe56ada).
  Grouped accesses: 2 4-byte load records with target `abs 0x300026c0`, including `tst.l` at 0xffe56ae4, `move.l` at 0xffe56b24.
  Branch (`ble.b` at 0xffe56ae8): taken to 0xffe56b24 when `[0x300026c0] <= 0x0 (signed, 32-bit)`, falls through to 0xffe56aea when `[0x300026c0] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000370`, including `movea.l` at 0xffe56af0, `movea.l` at 0xffe56afa, `movea.l` at 0xffe56b00.
  Halfword store: target `[0x30000370] +0x88`, value `0x4000` (`move.w` at 0xffe56af6).
  Word store: target `[0x30000370] +0x8c`, value `0x0` (`clr.l` at 0xffe56afc).
  Halfword store: target `[0x30000370] +0x92`, value `0x953` (`move.w` at 0xffe56b06).
  Call (`jsr` at 0xffe56b1a): (unknown callee), target 0xffe6e258, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe56b2a): taken to 0xffe56b70 when `Z == 0`, falls through to 0xffe56b2c when `not (Z == 0)`.
  Byte store: target `[0x30000370] +0x16`, value `0x8c` (`move.b` at 0xffe56b36).
  Halfword store: target `[0x30000370] +0xa0`, value `0x6000` (`move.w` at 0xffe56b40).
  Word store: target `[0x30000370] +0xa4`, value `0x0` (`clr.l` at 0xffe56b46).
  Halfword store: target `[0x30000370] +0xaa`, value `0x953` (`move.w` at 0xffe56b50).
  Call (`jsr` at 0xffe56b66): (unknown callee), target 0xffe6e258, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the return (`rts` at 0xffe56b76), d0 is `unknown`.

- `0xffe6ede0`, `root/6.5.19/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.19:l1#1`, version `1.18.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe6ede0 to 0xffe6ee9c (exclusive), 188 bytes.
  Carrier: 2668016 bytes, SHA256 `9b9fbe440979d0082cc1c41dfa89fcf57136493f359b55aedcded65f2827d562`.
  Package: carrier offset 0xee89c, 1690964 bytes, SHA256 `f3bcc8c59d2bd3da42046f540b7a93125398e6bf126e69a1c670a45d87ce03f7`.
  Payload: carrier offset 0xee8c8, 1690920 bytes, SHA256 `5ffb4512edcd71b473fcfff88b4baba1656e2bb79ed0808fa9105ba4f7648f1a`.
  Read window: SHA256 `f72864ddd5cc53fe01d1ba2f3c30c8d4fdb49006e44bdd86f7375ed7bd86962c`.
  Call (`jsr` at 0xffe6ede6): (unknown callee), target 0xffe57f7c, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe6edee): (unknown callee), target 0xffe6e7ac, stack arg0=`ret(0xffe57f7c)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe6edf4).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe6edfa).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe6edfe).
  Grouped accesses: 2 4-byte load records with target `abs 0x30002824`, including `tst.l` at 0xffe6ee08, `move.l` at 0xffe6ee48.
  Branch (`ble.b` at 0xffe6ee0c): taken to 0xffe6ee48 when `[0x30002824] <= 0x0 (signed, 32-bit)`, falls through to 0xffe6ee0e when `[0x30002824] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x3000048c`, including `movea.l` at 0xffe6ee14, `movea.l` at 0xffe6ee1e, `movea.l` at 0xffe6ee24.
  Halfword store: target `[0x3000048c] +0x88`, value `0x4000` (`move.w` at 0xffe6ee1a).
  Word store: target `[0x3000048c] +0x8c`, value `0x0` (`clr.l` at 0xffe6ee20).
  Halfword store: target `[0x3000048c] +0x92`, value `0x953` (`move.w` at 0xffe6ee2a).
  Call (`jsr` at 0xffe6ee3e): (unknown callee), target 0xffe87ccc, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe6ee4e): taken to 0xffe6ee94 when `Z == 0`, falls through to 0xffe6ee50 when `not (Z == 0)`.
  Byte store: target `[0x3000048c] +0x16`, value `0x8c` (`move.b` at 0xffe6ee5a).
  Halfword store: target `[0x3000048c] +0xa0`, value `0x6000` (`move.w` at 0xffe6ee64).
  Word store: target `[0x3000048c] +0xa4`, value `0x0` (`clr.l` at 0xffe6ee6a).
  Halfword store: target `[0x3000048c] +0xaa`, value `0x953` (`move.w` at 0xffe6ee74).
  Call (`jsr` at 0xffe6ee8a): (unknown callee), target 0xffe87ccc, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  The exit (`rts` at 0xffe6ee9a) reports d0 = `unknown`.

- `0xffe5f770`, `root/6.5.20/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.20:l1#0`, version `1.20.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe5f770 to 0xffe5f82c (exclusive), 188 bytes.
  Carrier: 2752144 bytes, SHA256 `adc39a287c525f1d6ac9bb8812671546f26c9029809d3bd487f5f8ab7318f94d`.
  Package: carrier offset 0x0, 982548 bytes, SHA256 `422d287101c8919f17eaf903e9e69401f30ee481ce43e445290cc1007ba9b2f9`.
  Payload: carrier offset 0x40, 982484 bytes, SHA256 `4cbaeb5a69e89e79595fefde6bdf506b2a529c180deb9bfb01cad7d92a2287a3`.
  Read window: SHA256 `5bf92bd320482ed6ba528b8ce60e6d41b94278604320545692a4cc44d8e05e2e`.
  Call (`jsr` at 0xffe5f776): (unknown callee), target 0xffe45a1c, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe5f77e): (unknown callee), target 0xffe5f0e4, stack arg0=`ret(0xffe45a1c)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe5f784).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe5f78a).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe5f78e).
  Grouped accesses: 2 4-byte load records with target `abs 0x300024d4`, including `tst.l` at 0xffe5f798, `move.l` at 0xffe5f7d8.
  Branch (`ble.b` at 0xffe5f79c): taken to 0xffe5f7d8 when `[0x300024d4] <= 0x0 (signed, 32-bit)`, falls through to 0xffe5f79e when `[0x300024d4] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x3000083c`, including `movea.l` at 0xffe5f7a4, `movea.l` at 0xffe5f7ae, `movea.l` at 0xffe5f7b4.
  Halfword store: target `[0x3000083c] +0x88`, value `0x4000` (`move.w` at 0xffe5f7aa).
  Word store: target `[0x3000083c] +0x8c`, value `0x0` (`clr.l` at 0xffe5f7b0).
  Halfword store: target `[0x3000083c] +0x92`, value `0x953` (`move.w` at 0xffe5f7ba).
  Call (`jsr` at 0xffe5f7ce): (unknown callee), target 0xffe6f71c, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe5f7de): taken to 0xffe5f824 when `Z == 0`, falls through to 0xffe5f7e0 when `not (Z == 0)`.
  Byte store: target `[0x3000083c] +0x16`, value `0x8c` (`move.b` at 0xffe5f7ea).
  Halfword store: target `[0x3000083c] +0xa0`, value `0x6000` (`move.w` at 0xffe5f7f4).
  Word store: target `[0x3000083c] +0xa4`, value `0x0` (`clr.l` at 0xffe5f7fa).
  Halfword store: target `[0x3000083c] +0xaa`, value `0x953` (`move.w` at 0xffe5f804).
  Call (`jsr` at 0xffe5f81a): (unknown callee), target 0xffe6f71c, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  `rts` at 0xffe5f82a has recorded d0 = `unknown`.

- `0xffe59fec`, `root/6.5.21/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.21:l1#0`, version `1.22.2`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe59fec to 0xffe5a0a8 (exclusive), 188 bytes.
  Carrier: 2766272 bytes, SHA256 `d5aaba306f71d9bfe0aa83d5029e14f2b9cf7485a4c0c689297fc4f71d136fbb`.
  Package: carrier offset 0x0, 981456 bytes, SHA256 `9a9930f67c71e90a89751858eb80145e5c94bfcf4a25ff050943034f16d97d34`.
  Payload: carrier offset 0x40, 981392 bytes, SHA256 `2ee201adbe9fde941fa147cf8c1b5cc3d3b32026f9dfa108e1cde0f06fd88a2b`.
  Read window: SHA256 `7b487290c90ce814340b9343183ad4aea15c931f30a931ad12bd446216dc8fe7`.
  Call (`jsr` at 0xffe59ff2): (unknown callee), target 0xffe4a148, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe59ffa): (unknown callee), target 0xffe59950, stack arg0=`ret(0xffe4a148)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe5a000).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe5a006).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe5a00a).
  Grouped accesses: 2 4-byte load records with target `abs 0x30001efc`, including `tst.l` at 0xffe5a014, `move.l` at 0xffe5a054.
  Branch (`ble.b` at 0xffe5a018): taken to 0xffe5a054 when `[0x30001efc] <= 0x0 (signed, 32-bit)`, falls through to 0xffe5a01a when `[0x30001efc] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000324`, including `movea.l` at 0xffe5a020, `movea.l` at 0xffe5a02a, `movea.l` at 0xffe5a030.
  Halfword store: target `[0x30000324] +0x88`, value `0x4000` (`move.w` at 0xffe5a026).
  Word store: target `[0x30000324] +0x8c`, value `0x0` (`clr.l` at 0xffe5a02c).
  Halfword store: target `[0x30000324] +0x92`, value `0x953` (`move.w` at 0xffe5a036).
  Call (`jsr` at 0xffe5a04a): (unknown callee), target 0xffe6f6e0, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe5a05a): taken to 0xffe5a0a0 when `Z == 0`, falls through to 0xffe5a05c when `not (Z == 0)`.
  Byte store: target `[0x30000324] +0x16`, value `0x8c` (`move.b` at 0xffe5a066).
  Halfword store: target `[0x30000324] +0xa0`, value `0x6000` (`move.w` at 0xffe5a070).
  Word store: target `[0x30000324] +0xa4`, value `0x0` (`clr.l` at 0xffe5a076).
  Halfword store: target `[0x30000324] +0xaa`, value `0x953` (`move.w` at 0xffe5a080).
  Call (`jsr` at 0xffe5a096): (unknown callee), target 0xffe6f6e0, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Recorded d0 at the return is `unknown` (`rts` at 0xffe5a0a6).

- `0xffe78790`, `root/6.5.21/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.21:l1#1`, version `1.22.2`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe78790 to 0xffe7884c (exclusive), 188 bytes.
  Carrier: 2766272 bytes, SHA256 `d5aaba306f71d9bfe0aa83d5029e14f2b9cf7485a4c0c689297fc4f71d136fbb`.
  Package: carrier offset 0xef9d0, 1784816 bytes, SHA256 `d597012cce4081272d0987ef729afeb15d40a0de6590702331370b07e76dcfae`.
  Payload: carrier offset 0xef9fc, 1784772 bytes, SHA256 `c960b88c14cba5aaca4d5efed9325e5830948f33cb623bf8dbc9887a4b1a8c26`.
  Read window: SHA256 `311b0d9ab58602826af5d9b45570ec609b147fd9373145ea42089655da498fc8`.
  Call (`jsr` at 0xffe78796): (unknown callee), target 0xffe647c8, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe7879e): (unknown callee), target 0xffe780f4, stack arg0=`ret(0xffe647c8)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe787a4).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe787aa).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe787ae).
  Grouped accesses: 2 4-byte load records with target `abs 0x30002524`, including `tst.l` at 0xffe787b8, `move.l` at 0xffe787f8.
  Branch (`ble.b` at 0xffe787bc): taken to 0xffe787f8 when `[0x30002524] <= 0x0 (signed, 32-bit)`, falls through to 0xffe787be when `[0x30002524] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x300004f8`, including `movea.l` at 0xffe787c4, `movea.l` at 0xffe787ce, `movea.l` at 0xffe787d4.
  Halfword store: target `[0x300004f8] +0x88`, value `0x4000` (`move.w` at 0xffe787ca).
  Word store: target `[0x300004f8] +0x8c`, value `0x0` (`clr.l` at 0xffe787d0).
  Halfword store: target `[0x300004f8] +0x92`, value `0x953` (`move.w` at 0xffe787da).
  Call (`jsr` at 0xffe787ee): (unknown callee), target 0xffe9ad30, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe787fe): taken to 0xffe78844 when `Z == 0`, falls through to 0xffe78800 when `not (Z == 0)`.
  Byte store: target `[0x300004f8] +0x16`, value `0x8c` (`move.b` at 0xffe7880a).
  Halfword store: target `[0x300004f8] +0xa0`, value `0x6000` (`move.w` at 0xffe78814).
  Word store: target `[0x300004f8] +0xa4`, value `0x0` (`clr.l` at 0xffe7881a).
  Halfword store: target `[0x300004f8] +0xaa`, value `0x953` (`move.w` at 0xffe78824).
  Call (`jsr` at 0xffe7883a): (unknown callee), target 0xffe9ad30, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the return (`rts` at 0xffe7884a), d0 is `unknown`.

- `0xffe5db64`, `root/6.5.22/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.22:l1#0`, version `1.24.8`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe5db64 to 0xffe5dc20 (exclusive), 188 bytes.
  Carrier: 3723916 bytes, SHA256 `5b90997945dabdc099162d5237e302472f5c3c80a808a1de5abb246c290a963b`.
  Package: carrier offset 0x0, 977968 bytes, SHA256 `5660c129f64680c9639f50d5f3d7c61428912f2270dde66185a7acccdcf64755`.
  Payload: carrier offset 0x48, 977896 bytes, SHA256 `0fe5e128ec92fb73681f4b9c7e148b1a68de5cf5199c150251c4703beef4e13f`.
  Read window: SHA256 `57086835248788ee0034b7178542d9ddb27b027be205ca72ef0ddf55c45b056f`.
  Call (`jsr` at 0xffe5db6a): (unknown callee), target 0xffe5e270, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe5db72): (unknown callee), target 0xffe5d47c, stack arg0=`ret(0xffe5e270)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe5db78).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe5db7e).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe5db82).
  Grouped accesses: 2 4-byte load records with target `abs 0x30002890`, including `tst.l` at 0xffe5db8c, `move.l` at 0xffe5dbcc.
  Branch (`ble.b` at 0xffe5db90): taken to 0xffe5dbcc when `[0x30002890] <= 0x0 (signed, 32-bit)`, falls through to 0xffe5db92 when `[0x30002890] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x300008b4`, including `movea.l` at 0xffe5db98, `movea.l` at 0xffe5dba2, `movea.l` at 0xffe5dba8.
  Halfword store: target `[0x300008b4] +0x88`, value `0x4000` (`move.w` at 0xffe5db9e).
  Word store: target `[0x300008b4] +0x8c`, value `0x0` (`clr.l` at 0xffe5dba4).
  Halfword store: target `[0x300008b4] +0x92`, value `0x953` (`move.w` at 0xffe5dbae).
  Call (`jsr` at 0xffe5dbc2): (unknown callee), target 0xffe6e95c, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe5dbd2): taken to 0xffe5dc18 when `Z == 0`, falls through to 0xffe5dbd4 when `not (Z == 0)`.
  Byte store: target `[0x300008b4] +0x16`, value `0x8c` (`move.b` at 0xffe5dbde).
  Halfword store: target `[0x300008b4] +0xa0`, value `0x6000` (`move.w` at 0xffe5dbe8).
  Word store: target `[0x300008b4] +0xa4`, value `0x0` (`clr.l` at 0xffe5dbee).
  Halfword store: target `[0x300008b4] +0xaa`, value `0x953` (`move.w` at 0xffe5dbf8).
  Call (`jsr` at 0xffe5dc0e): (unknown callee), target 0xffe6e95c, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  The exit (`rts` at 0xffe5dc1e) reports d0 = `unknown`.

- `0xffe5c700`, `root/6.5.22/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.22:l1#1`, version `1.24.8`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe5c700 to 0xffe5c7bc (exclusive), 188 bytes.
  Carrier: 3723916 bytes, SHA256 `5b90997945dabdc099162d5237e302472f5c3c80a808a1de5abb246c290a963b`.
  Package: carrier offset 0xeec30, 973412 bytes, SHA256 `8ce4c2cf2d93012d81b4ba6bc7a1deb66de83a32c5cb3052715b3a5d96fb7971`.
  Payload: carrier offset 0xeec5c, 973368 bytes, SHA256 `1640737f85acb71067aab42164e36704e6f1510cd8346bbdca7526e8a63c6b12`.
  Read window: SHA256 `98dd5c7a635076d57dc2f9531df23572bbef316eccbe8cf756b8f9a0065643db`.
  Call (`jsr` at 0xffe5c706): (unknown callee), target 0xffe5ce0c, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe5c70e): (unknown callee), target 0xffe5c018, stack arg0=`ret(0xffe5ce0c)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe5c714).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe5c71a).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe5c71e).
  Grouped accesses: 2 4-byte load records with target `abs 0x30002948`, including `tst.l` at 0xffe5c728, `move.l` at 0xffe5c768.
  Branch (`ble.b` at 0xffe5c72c): taken to 0xffe5c768 when `[0x30002948] <= 0x0 (signed, 32-bit)`, falls through to 0xffe5c72e when `[0x30002948] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x3000094c`, including `movea.l` at 0xffe5c734, `movea.l` at 0xffe5c73e, `movea.l` at 0xffe5c744.
  Halfword store: target `[0x3000094c] +0x88`, value `0x4000` (`move.w` at 0xffe5c73a).
  Word store: target `[0x3000094c] +0x8c`, value `0x0` (`clr.l` at 0xffe5c740).
  Halfword store: target `[0x3000094c] +0x92`, value `0x953` (`move.w` at 0xffe5c74a).
  Call (`jsr` at 0xffe5c75e): (unknown callee), target 0xffe6d6fc, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe5c76e): taken to 0xffe5c7b4 when `Z == 0`, falls through to 0xffe5c770 when `not (Z == 0)`.
  Byte store: target `[0x3000094c] +0x16`, value `0x8c` (`move.b` at 0xffe5c77a).
  Halfword store: target `[0x3000094c] +0xa0`, value `0x6000` (`move.w` at 0xffe5c784).
  Word store: target `[0x3000094c] +0xa4`, value `0x0` (`clr.l` at 0xffe5c78a).
  Halfword store: target `[0x3000094c] +0xaa`, value `0x953` (`move.w` at 0xffe5c794).
  Call (`jsr` at 0xffe5c7aa): (unknown callee), target 0xffe6d6fc, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  `rts` at 0xffe5c7ba has recorded d0 = `unknown`.

- `0xffe7f234`, `root/6.5.22/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.22:l1#2`, version `1.24.8`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe7f234 to 0xffe7f2f0 (exclusive), 188 bytes.
  Carrier: 3723916 bytes, SHA256 `5b90997945dabdc099162d5237e302472f5c3c80a808a1de5abb246c290a963b`.
  Package: carrier offset 0x1dc694, 1772536 bytes, SHA256 `6d06a281a09291d82228d78cfc1c0f2419eff91a940d388bf6bc3b406fbf32ff`.
  Payload: carrier offset 0x1dc6c0, 1772492 bytes, SHA256 `953894c5c4e86f03e78ada263bebd12e0e7af316fe0733a8bc606f2d94ce66da`.
  Read window: SHA256 `bd8d08e70291784cfec201924dbe3e2af0857c55a191b8c66d8b91957bf8034c`.
  Call (`jsr` at 0xffe7f23a): (unknown callee), target 0xffe7f940, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe7f242): (unknown callee), target 0xffe7eb4c, stack arg0=`ret(0xffe7f940)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe7f248).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe7f24e).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe7f252).
  Grouped accesses: 2 4-byte load records with target `abs 0x30002e80`, including `tst.l` at 0xffe7f25c, `move.l` at 0xffe7f29c.
  Branch (`ble.b` at 0xffe7f260): taken to 0xffe7f29c when `[0x30002e80] <= 0x0 (signed, 32-bit)`, falls through to 0xffe7f262 when `[0x30002e80] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000b08`, including `movea.l` at 0xffe7f268, `movea.l` at 0xffe7f272, `movea.l` at 0xffe7f278.
  Halfword store: target `[0x30000b08] +0x88`, value `0x4000` (`move.w` at 0xffe7f26e).
  Word store: target `[0x30000b08] +0x8c`, value `0x0` (`clr.l` at 0xffe7f274).
  Halfword store: target `[0x30000b08] +0x92`, value `0x953` (`move.w` at 0xffe7f27e).
  Call (`jsr` at 0xffe7f292): (unknown callee), target 0xffe994d8, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe7f2a2): taken to 0xffe7f2e8 when `Z == 0`, falls through to 0xffe7f2a4 when `not (Z == 0)`.
  Byte store: target `[0x30000b08] +0x16`, value `0x8c` (`move.b` at 0xffe7f2ae).
  Halfword store: target `[0x30000b08] +0xa0`, value `0x6000` (`move.w` at 0xffe7f2b8).
  Word store: target `[0x30000b08] +0xa4`, value `0x0` (`clr.l` at 0xffe7f2be).
  Halfword store: target `[0x30000b08] +0xaa`, value `0x953` (`move.w` at 0xffe7f2c8).
  Call (`jsr` at 0xffe7f2de): (unknown callee), target 0xffe994d8, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Recorded d0 at the return is `unknown` (`rts` at 0xffe7f2ee).

- `0xffe42060`, `root/6.5.9/usr/cpu/firmware/l1.bin`.
  Package `6.5.9:l1#0`, version `1.0.2`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe42060 to 0xffe420f6 (exclusive), 150 bytes.
  Carrier: 927304 bytes, SHA256 `346ebde8121eda2b390df35b37bca75a28c5bbfc44dd5f329b91d56455b575e8`.
  Package: carrier offset 0x0, 927304 bytes, SHA256 `346ebde8121eda2b390df35b37bca75a28c5bbfc44dd5f329b91d56455b575e8`.
  Payload: carrier offset 0x20, 927272 bytes, SHA256 `8a177e55cfdd58f8d233ea3c6e5f8b1a5a9bcaf10a20e95cbde444bd34da6545`.
  Read window: SHA256 `565d9967d55e44261942c7c8521acf04f54bfdea2527dba96a38baed6a84f213`.
  Call (`jsr` at 0xffe42066): (unknown callee), target 0xffe47428, stack arg0=`d7`, stack arg1=`a6`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe42076): (unknown callee), target 0xffe420f8, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bgt.b` at 0xffe4208a): taken to 0xffe420a0 when `gt flags condition (operands unknown)`, falls through to 0xffe4208c when `not (gt flags condition (operands unknown))`.
  Branch (`beq.b` at 0xffe4208c): taken to 0xffe420b2 when `Z == 1`, falls through to 0xffe4208e when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe42094): taken to 0xffe420b2 when `Z == 1`, falls through to 0xffe42096 when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe42098): taken to 0xffe420b2 when `Z == 1`, falls through to 0xffe4209a when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe4209c): taken to 0xffe420b2 when `Z == 1`, falls through to 0xffe4209e when `not (Z == 1)`.
  Jump (`bra.b` at 0xffe4209e): target 0xffe420ee.
  Branch (`beq.b` at 0xffe420a2): taken to 0xffe420b2 when `Z == 1`, falls through to 0xffe420a4 when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe420aa): taken to 0xffe420b2 when `Z == 1`, falls through to 0xffe420ac when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe420ae): taken to 0xffe420b2 when `Z == 1`, falls through to 0xffe420b0 when `not (Z == 1)`.
  Jump (`bra.b` at 0xffe420b0): target 0xffe420ee.
  Grouped accesses: 3 4-byte load records with target `abs 0x30000480`, including `movea.l` at 0xffe420b2, `movea.l` at 0xffe420c0, `movea.l` at 0xffe420ca.
  Halfword store: target `[0x30000480] +0x88`, value `unknown` (`move.w` at 0xffe420bc).
  Word store: target `[0x30000480] +0x8c`, value `0x0` (`clr.l` at 0xffe420c6).
  Halfword store: target `[0x30000480] +0x92`, value `unknown` (`move.w` at 0xffe420d4).
  Call (`jsr` at 0xffe420e4): (unknown callee), target 0xffe543d4, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the return (`rts` at 0xffe420f4), d0 is `unknown`.

- `0xfff52790`, `root/6.5.18/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.18:l1#1`, version `1.16.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xfff52790 to 0xfff5284c (exclusive), 188 bytes.
  Carrier: 2627188 bytes, SHA256 `496d9a1ec82d7e5d676032cd68630d928119bbcfd111936ed3c960287823f930`.
  Package: carrier offset 0xecc74, 1657344 bytes, SHA256 `a1c8be76d07641312949343ecca5280c8b8ba1203ca1ad4be703d9d8da70a290`.
  Payload: carrier offset 0xecca0, 1657300 bytes, SHA256 `ea5a7a1053b90e7691c97e22f77d65015389a15d2da4c1c7f4ac03f919a52abd`.
  Read window: SHA256 `9dad7b8d24a45f58be09154a53cc3aaa0ef93ae83247906788a5fbdf9fb4e635`.
  Call (`jsr` at 0xfff52796): (unknown callee), target 0xfff43b90, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xfff5279e): (unknown callee), target 0xfff5219c, stack arg0=`ret(0xfff43b90)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xfff527a4).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xfff527aa).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xfff527ae).
  Grouped accesses: 2 4-byte load records with target `abs 0x30002f34`, including `tst.l` at 0xfff527b8, `move.l` at 0xfff527f8.
  Branch (`ble.b` at 0xfff527bc): taken to 0xfff527f8 when `[0x30002f34] <= 0x0 (signed, 32-bit)`, falls through to 0xfff527be when `[0x30002f34] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000340`, including `movea.l` at 0xfff527c4, `movea.l` at 0xfff527ce, `movea.l` at 0xfff527d4.
  Halfword store: target `[0x30000340] +0x88`, value `0x4000` (`move.w` at 0xfff527ca).
  Word store: target `[0x30000340] +0x8c`, value `0x0` (`clr.l` at 0xfff527d0).
  Halfword store: target `[0x30000340] +0x92`, value `0x953` (`move.w` at 0xfff527da).
  Call (`jsr` at 0xfff527ee): (unknown callee), target 0xfff711fc, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xfff527fe): taken to 0xfff52844 when `Z == 0`, falls through to 0xfff52800 when `not (Z == 0)`.
  Byte store: target `[0x30000340] +0x16`, value `0x8c` (`move.b` at 0xfff5280a).
  Halfword store: target `[0x30000340] +0xa0`, value `0x6000` (`move.w` at 0xfff52814).
  Word store: target `[0x30000340] +0xa4`, value `0x0` (`clr.l` at 0xfff5281a).
  Halfword store: target `[0x30000340] +0xaa`, value `0x953` (`move.w` at 0xfff52824).
  Call (`jsr` at 0xfff5283a): (unknown callee), target 0xfff711fc, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  The exit (`rts` at 0xfff5284a) reports d0 = `unknown`.

- `0xffec6abc`, `root/6.5.19/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.19:l1#0`, version `1.18.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffec6abc to 0xffec6b78 (exclusive), 188 bytes.
  Carrier: 2668016 bytes, SHA256 `9b9fbe440979d0082cc1c41dfa89fcf57136493f359b55aedcded65f2827d562`.
  Package: carrier offset 0x0, 977052 bytes, SHA256 `389a486d2213168fcde72fafaa1bd9efe5bbbde06e52e1f023f5dd76900537c5`.
  Payload: carrier offset 0x40, 976988 bytes, SHA256 `9b636fd9c1b5fe130de21c2db5a40eb3fcd6fc0452887f2ddc8a015d9fad521d`.
  Read window: SHA256 `23142c61a5dd11d0e814ac2c080ae2e25079fb791ef8c0c1eec59a36726985ea`.
  Call (`jsr` at 0xffec6ac2): (unknown callee), target 0xffebc4e8, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffec6aca): (unknown callee), target 0xffec6488, stack arg0=`ret(0xffebc4e8)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffec6ad0).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffec6ad6).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffec6ada).
  Grouped accesses: 2 4-byte load records with target `abs 0x300026c0`, including `tst.l` at 0xffec6ae4, `move.l` at 0xffec6b24.
  Branch (`ble.b` at 0xffec6ae8): taken to 0xffec6b24 when `[0x300026c0] <= 0x0 (signed, 32-bit)`, falls through to 0xffec6aea when `[0x300026c0] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000370`, including `movea.l` at 0xffec6af0, `movea.l` at 0xffec6afa, `movea.l` at 0xffec6b00.
  Halfword store: target `[0x30000370] +0x88`, value `0x4000` (`move.w` at 0xffec6af6).
  Word store: target `[0x30000370] +0x8c`, value `0x0` (`clr.l` at 0xffec6afc).
  Halfword store: target `[0x30000370] +0x92`, value `0x953` (`move.w` at 0xffec6b06).
  Call (`jsr` at 0xffec6b1a): (unknown callee), target 0xffede258, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffec6b2a): taken to 0xffec6b70 when `Z == 0`, falls through to 0xffec6b2c when `not (Z == 0)`.
  Byte store: target `[0x30000370] +0x16`, value `0x8c` (`move.b` at 0xffec6b36).
  Halfword store: target `[0x30000370] +0xa0`, value `0x6000` (`move.w` at 0xffec6b40).
  Word store: target `[0x30000370] +0xa4`, value `0x0` (`clr.l` at 0xffec6b46).
  Halfword store: target `[0x30000370] +0xaa`, value `0x953` (`move.w` at 0xffec6b50).
  Call (`jsr` at 0xffec6b66): (unknown callee), target 0xffede258, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  `rts` at 0xffec6b76 has recorded d0 = `unknown`.

- `0xfff5ede0`, `root/6.5.19/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.19:l1#1`, version `1.18.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xfff5ede0 to 0xfff5ee9c (exclusive), 188 bytes.
  Carrier: 2668016 bytes, SHA256 `9b9fbe440979d0082cc1c41dfa89fcf57136493f359b55aedcded65f2827d562`.
  Package: carrier offset 0xee89c, 1690964 bytes, SHA256 `f3bcc8c59d2bd3da42046f540b7a93125398e6bf126e69a1c670a45d87ce03f7`.
  Payload: carrier offset 0xee8c8, 1690920 bytes, SHA256 `5ffb4512edcd71b473fcfff88b4baba1656e2bb79ed0808fa9105ba4f7648f1a`.
  Read window: SHA256 `48edc82b48d1674e15bd3e1fe44fea68eb6889fb13d0de40a7e1e9efddd95ca5`.
  Call (`jsr` at 0xfff5ede6): (unknown callee), target 0xfff47f7c, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xfff5edee): (unknown callee), target 0xfff5e7ac, stack arg0=`ret(0xfff47f7c)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xfff5edf4).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xfff5edfa).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xfff5edfe).
  Grouped accesses: 2 4-byte load records with target `abs 0x30002824`, including `tst.l` at 0xfff5ee08, `move.l` at 0xfff5ee48.
  Branch (`ble.b` at 0xfff5ee0c): taken to 0xfff5ee48 when `[0x30002824] <= 0x0 (signed, 32-bit)`, falls through to 0xfff5ee0e when `[0x30002824] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x3000048c`, including `movea.l` at 0xfff5ee14, `movea.l` at 0xfff5ee1e, `movea.l` at 0xfff5ee24.
  Halfword store: target `[0x3000048c] +0x88`, value `0x4000` (`move.w` at 0xfff5ee1a).
  Word store: target `[0x3000048c] +0x8c`, value `0x0` (`clr.l` at 0xfff5ee20).
  Halfword store: target `[0x3000048c] +0x92`, value `0x953` (`move.w` at 0xfff5ee2a).
  Call (`jsr` at 0xfff5ee3e): (unknown callee), target 0xfff77ccc, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xfff5ee4e): taken to 0xfff5ee94 when `Z == 0`, falls through to 0xfff5ee50 when `not (Z == 0)`.
  Byte store: target `[0x3000048c] +0x16`, value `0x8c` (`move.b` at 0xfff5ee5a).
  Halfword store: target `[0x3000048c] +0xa0`, value `0x6000` (`move.w` at 0xfff5ee64).
  Word store: target `[0x3000048c] +0xa4`, value `0x0` (`clr.l` at 0xfff5ee6a).
  Halfword store: target `[0x3000048c] +0xaa`, value `0x953` (`move.w` at 0xfff5ee74).
  Call (`jsr` at 0xfff5ee8a): (unknown callee), target 0xfff77ccc, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Recorded d0 at the return is `unknown` (`rts` at 0xfff5ee9a).

- `0xffecf770`, `root/6.5.20/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.20:l1#0`, version `1.20.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffecf770 to 0xffecf82c (exclusive), 188 bytes.
  Carrier: 2752144 bytes, SHA256 `adc39a287c525f1d6ac9bb8812671546f26c9029809d3bd487f5f8ab7318f94d`.
  Package: carrier offset 0x0, 982548 bytes, SHA256 `422d287101c8919f17eaf903e9e69401f30ee481ce43e445290cc1007ba9b2f9`.
  Payload: carrier offset 0x40, 982484 bytes, SHA256 `4cbaeb5a69e89e79595fefde6bdf506b2a529c180deb9bfb01cad7d92a2287a3`.
  Read window: SHA256 `d3c770ecc2522e74fae66d3098a062fda24a8e01d0f8600e6441e586fd1971e4`.
  Call (`jsr` at 0xffecf776): (unknown callee), target 0xffeb5a1c, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffecf77e): (unknown callee), target 0xffecf0e4, stack arg0=`ret(0xffeb5a1c)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffecf784).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffecf78a).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffecf78e).
  Grouped accesses: 2 4-byte load records with target `abs 0x300024d4`, including `tst.l` at 0xffecf798, `move.l` at 0xffecf7d8.
  Branch (`ble.b` at 0xffecf79c): taken to 0xffecf7d8 when `[0x300024d4] <= 0x0 (signed, 32-bit)`, falls through to 0xffecf79e when `[0x300024d4] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x3000083c`, including `movea.l` at 0xffecf7a4, `movea.l` at 0xffecf7ae, `movea.l` at 0xffecf7b4.
  Halfword store: target `[0x3000083c] +0x88`, value `0x4000` (`move.w` at 0xffecf7aa).
  Word store: target `[0x3000083c] +0x8c`, value `0x0` (`clr.l` at 0xffecf7b0).
  Halfword store: target `[0x3000083c] +0x92`, value `0x953` (`move.w` at 0xffecf7ba).
  Call (`jsr` at 0xffecf7ce): (unknown callee), target 0xffedf71c, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffecf7de): taken to 0xffecf824 when `Z == 0`, falls through to 0xffecf7e0 when `not (Z == 0)`.
  Byte store: target `[0x3000083c] +0x16`, value `0x8c` (`move.b` at 0xffecf7ea).
  Halfword store: target `[0x3000083c] +0xa0`, value `0x6000` (`move.w` at 0xffecf7f4).
  Word store: target `[0x3000083c] +0xa4`, value `0x0` (`clr.l` at 0xffecf7fa).
  Halfword store: target `[0x3000083c] +0xaa`, value `0x953` (`move.w` at 0xffecf804).
  Call (`jsr` at 0xffecf81a): (unknown callee), target 0xffedf71c, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the return (`rts` at 0xffecf82a), d0 is `unknown`.

- `0xfff68c7c`, `root/6.5.20/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.20:l1#1`, version `1.20.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xfff68c7c to 0xfff68d38 (exclusive), 188 bytes.
  Carrier: 2752144 bytes, SHA256 `adc39a287c525f1d6ac9bb8812671546f26c9029809d3bd487f5f8ab7318f94d`.
  Package: carrier offset 0xefe14, 1769596 bytes, SHA256 `fa86b77f661d6f17243158a26aadf41a842ebd5e89248c73423e05c16e945ce9`.
  Payload: carrier offset 0xefe40, 1769552 bytes, SHA256 `ae841c9d40a0f5bfa1224e27e43c002300ed1704df032e3a97db6bd1a77f1c28`.
  Read window: SHA256 `ae92da2d6baa66c68937105227ecc4c27db3fdca4fb35cc5e82052ca9593d4a7`.
  Call (`jsr` at 0xfff68c82): (unknown callee), target 0xfff4114c, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xfff68c8a): (unknown callee), target 0xfff685f0, stack arg0=`ret(0xfff4114c)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xfff68c90).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xfff68c96).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xfff68c9a).
  Grouped accesses: 2 4-byte load records with target `abs 0x30002678`, including `tst.l` at 0xfff68ca4, `move.l` at 0xfff68ce4.
  Branch (`ble.b` at 0xfff68ca8): taken to 0xfff68ce4 when `[0x30002678] <= 0x0 (signed, 32-bit)`, falls through to 0xfff68caa when `[0x30002678] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000974`, including `movea.l` at 0xfff68cb0, `movea.l` at 0xfff68cba, `movea.l` at 0xfff68cc0.
  Halfword store: target `[0x30000974] +0x88`, value `0x4000` (`move.w` at 0xfff68cb6).
  Word store: target `[0x30000974] +0x8c`, value `0x0` (`clr.l` at 0xfff68cbc).
  Halfword store: target `[0x30000974] +0x92`, value `0x953` (`move.w` at 0xfff68cc6).
  Call (`jsr` at 0xfff68cda): (unknown callee), target 0xfff8759c, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xfff68cea): taken to 0xfff68d30 when `Z == 0`, falls through to 0xfff68cec when `not (Z == 0)`.
  Byte store: target `[0x30000974] +0x16`, value `0x8c` (`move.b` at 0xfff68cf6).
  Halfword store: target `[0x30000974] +0xa0`, value `0x6000` (`move.w` at 0xfff68d00).
  Word store: target `[0x30000974] +0xa4`, value `0x0` (`clr.l` at 0xfff68d06).
  Halfword store: target `[0x30000974] +0xaa`, value `0x953` (`move.w` at 0xfff68d10).
  Call (`jsr` at 0xfff68d26): (unknown callee), target 0xfff8759c, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  The exit (`rts` at 0xfff68d36) reports d0 = `unknown`.

- `0xffec9fec`, `root/6.5.21/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.21:l1#0`, version `1.22.2`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffec9fec to 0xffeca0a8 (exclusive), 188 bytes.
  Carrier: 2766272 bytes, SHA256 `d5aaba306f71d9bfe0aa83d5029e14f2b9cf7485a4c0c689297fc4f71d136fbb`.
  Package: carrier offset 0x0, 981456 bytes, SHA256 `9a9930f67c71e90a89751858eb80145e5c94bfcf4a25ff050943034f16d97d34`.
  Payload: carrier offset 0x40, 981392 bytes, SHA256 `2ee201adbe9fde941fa147cf8c1b5cc3d3b32026f9dfa108e1cde0f06fd88a2b`.
  Read window: SHA256 `3f7d13f88665a87d57b8b980c7c6581aa0410afb0dff32f402271b7fb1647a97`.
  Call (`jsr` at 0xffec9ff2): (unknown callee), target 0xffeba148, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffec9ffa): (unknown callee), target 0xffec9950, stack arg0=`ret(0xffeba148)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffeca000).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffeca006).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffeca00a).
  Grouped accesses: 2 4-byte load records with target `abs 0x30001efc`, including `tst.l` at 0xffeca014, `move.l` at 0xffeca054.
  Branch (`ble.b` at 0xffeca018): taken to 0xffeca054 when `[0x30001efc] <= 0x0 (signed, 32-bit)`, falls through to 0xffeca01a when `[0x30001efc] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000324`, including `movea.l` at 0xffeca020, `movea.l` at 0xffeca02a, `movea.l` at 0xffeca030.
  Halfword store: target `[0x30000324] +0x88`, value `0x4000` (`move.w` at 0xffeca026).
  Word store: target `[0x30000324] +0x8c`, value `0x0` (`clr.l` at 0xffeca02c).
  Halfword store: target `[0x30000324] +0x92`, value `0x953` (`move.w` at 0xffeca036).
  Call (`jsr` at 0xffeca04a): (unknown callee), target 0xffedf6e0, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffeca05a): taken to 0xffeca0a0 when `Z == 0`, falls through to 0xffeca05c when `not (Z == 0)`.
  Byte store: target `[0x30000324] +0x16`, value `0x8c` (`move.b` at 0xffeca066).
  Halfword store: target `[0x30000324] +0xa0`, value `0x6000` (`move.w` at 0xffeca070).
  Word store: target `[0x30000324] +0xa4`, value `0x0` (`clr.l` at 0xffeca076).
  Halfword store: target `[0x30000324] +0xaa`, value `0x953` (`move.w` at 0xffeca080).
  Call (`jsr` at 0xffeca096): (unknown callee), target 0xffedf6e0, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  `rts` at 0xffeca0a6 has recorded d0 = `unknown`.
