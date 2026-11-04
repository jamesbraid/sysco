# ColdFire L1 pointer-store occurrences in IRIX 6.5.26 through 6.5.29 and IST 3.24

Each entry describes its exact carrier, package, payload and selected byte position. Addresses are analysis coordinates formed from the stated payload load plus byte offset. Later positions in one payload do not establish additional unique functions, physical flash banks, runtime mapping or image selection. No byte or semantic equivalence between positions is assumed.

The spans are inferred from entry, return and bounded context evidence. They are analyzed windows, not proved function sizes. Hardware roles, pointee meanings, types, fitted silicon and runtime path feasibility remain unknown. ColdFire classification comes from package metadata and decoding uses the 68040 common subset.

`sp` is the stack pointer at the analyzed entry. The six stack slots at each call are candidate argument locations under an assumed m68k-sysv convention. Actual arity and custom conventions remain unknown. A `ret(F)` expression does not identify its producing instruction. Repeated access groups count static shapes and do not establish equal values from separate loads or one unconditional path through different branch arms. Unknown values preserve tool question marks.

The [complete facts](disasm/coldfire-variants/pointer-store-occurrences-6.5.26-ist-facts.txt) retain the complete analyzed windows. Range ends are exclusive. Bounded contexts with clipped instructions or uncertain initial alignment are not proof of predecessor flow or full function extent.

- `0xfff7ff7c`, `root/6.5.26/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.26:l1#2`, version `1.32.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xfff7ff7c to 0xfff80038 (exclusive), 188 bytes.
  Carrier: 3846136 bytes, SHA256 `1fdcc3409f4e0633335d9b51d506818ce971dacbe6ababe1d4fbcbd52855e3dc`.
  Package: carrier offset 0x1debcc, 1885228 bytes, SHA256 `84466adaef25b1382edb5db0569655ab340dea79b2adde07cf9499f400ca4837`.
  Payload: carrier offset 0x1debf8, 1885184 bytes, SHA256 `9793fe0aac389db6d98cbd2fe0c6d96aa70dcc526d0cfe5810d554316931d981`.
  Read window: SHA256 `b359091e7de4642a0ff24e957412b25a3749051d2ad6758d75e710edc8925fdd`.
  Call (`jsr` at 0xfff7ff82): (unknown callee), target 0xfff808e8, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xfff7ff8a): (unknown callee), target 0xfff7f894, stack arg0=`ret(0xfff808e8)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xfff7ff90).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xfff7ff96).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xfff7ff9a).
  Grouped accesses: 2 4-byte load records with target `abs 0x30003260`, including `tst.l` at 0xfff7ffa4, `move.l` at 0xfff7ffe4.
  Branch (`ble.b` at 0xfff7ffa8): taken to 0xfff7ffe4 when `[0x30003260] <= 0x0 (signed, 32-bit)`, falls through to 0xfff7ffaa when `[0x30003260] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000a7c`, including `movea.l` at 0xfff7ffb0, `movea.l` at 0xfff7ffba, `movea.l` at 0xfff7ffc0.
  Halfword store: target `[0x30000a7c] +0x88`, value `0x4000` (`move.w` at 0xfff7ffb6).
  Word store: target `[0x30000a7c] +0x8c`, value `0x0` (`clr.l` at 0xfff7ffbc).
  Halfword store: target `[0x30000a7c] +0x92`, value `0x953` (`move.w` at 0xfff7ffc6).
  Call (`jsr` at 0xfff7ffda): (unknown callee), target 0xfff9d758, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xfff7ffea): taken to 0xfff80030 when `Z == 0`, falls through to 0xfff7ffec when `not (Z == 0)`.
  Byte store: target `[0x30000a7c] +0x16`, value `0x8c` (`move.b` at 0xfff7fff6).
  Halfword store: target `[0x30000a7c] +0xa0`, value `0x6000` (`move.w` at 0xfff80000).
  Word store: target `[0x30000a7c] +0xa4`, value `0x0` (`clr.l` at 0xfff80006).
  Halfword store: target `[0x30000a7c] +0xaa`, value `0x953` (`move.w` at 0xfff80010).
  Call (`jsr` at 0xfff80026): (unknown callee), target 0xfff9d758, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the return (`rts` at 0xfff80036), d0 is `unknown`.

- `0xffecdf3c`, `root/6.5.27/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.27:l1#0`, version `1.34.2`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffecdf3c to 0xffecdff8 (exclusive), 188 bytes.
  Carrier: 7523600 bytes, SHA256 `45c872a661d4a4ad20a4cb482fe2aee921ee9ae52784382be125b3d7b7836572`.
  Package: carrier offset 0x0, 981832 bytes, SHA256 `d3968aabd51125c60c759766ceed4c9fc66321e66e4e562d235bc47b16efb14d`.
  Payload: carrier offset 0x50, 981752 bytes, SHA256 `117f4126e840ee43eac0d96eb2aede9ff454f78b7c32a3aa42b68fcbe034e91b`.
  Read window: SHA256 `a440f78ccf4b099ac7a2c206fd23da0089803c826166c597a08ef8ca9d7689d5`.
  Call (`jsr` at 0xffecdf42): (unknown callee), target 0xffece8a8, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffecdf4a): (unknown callee), target 0xffecd854, stack arg0=`ret(0xffece8a8)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffecdf50).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffecdf56).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffecdf5a).
  Grouped accesses: 2 4-byte load records with target `abs 0x300025e8`, including `tst.l` at 0xffecdf64, `move.l` at 0xffecdfa4.
  Branch (`ble.b` at 0xffecdf68): taken to 0xffecdfa4 when `[0x300025e8] <= 0x0 (signed, 32-bit)`, falls through to 0xffecdf6a when `[0x300025e8] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000410`, including `movea.l` at 0xffecdf70, `movea.l` at 0xffecdf7a, `movea.l` at 0xffecdf80.
  Halfword store: target `[0x30000410] +0x88`, value `0x4000` (`move.w` at 0xffecdf76).
  Word store: target `[0x30000410] +0x8c`, value `0x0` (`clr.l` at 0xffecdf7c).
  Halfword store: target `[0x30000410] +0x92`, value `0x953` (`move.w` at 0xffecdf86).
  Call (`jsr` at 0xffecdf9a): (unknown callee), target 0xffedda4c, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffecdfaa): taken to 0xffecdff0 when `Z == 0`, falls through to 0xffecdfac when `not (Z == 0)`.
  Byte store: target `[0x30000410] +0x16`, value `0x8c` (`move.b` at 0xffecdfb6).
  Halfword store: target `[0x30000410] +0xa0`, value `0x6000` (`move.w` at 0xffecdfc0).
  Word store: target `[0x30000410] +0xa4`, value `0x0` (`clr.l` at 0xffecdfc6).
  Halfword store: target `[0x30000410] +0xaa`, value `0x953` (`move.w` at 0xffecdfd0).
  Call (`jsr` at 0xffecdfe6): (unknown callee), target 0xffedda4c, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  The exit (`rts` at 0xffecdff6) reports d0 = `unknown`.

- `0xffecdbe0`, `root/6.5.27/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.27:l1#1`, version `1.34.2`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffecdbe0 to 0xffecdc9c (exclusive), 188 bytes.
  Carrier: 7523600 bytes, SHA256 `45c872a661d4a4ad20a4cb482fe2aee921ee9ae52784382be125b3d7b7836572`.
  Package: carrier offset 0xefb48, 981764 bytes, SHA256 `28f6fdb9ba631c1fc226bf9d30b1812e4d5df62e06959929711f7340d3ed1c1d`.
  Payload: carrier offset 0xefb74, 981720 bytes, SHA256 `38a4b37992f18dfaaf632b9051ebd921251d6776f17c90b93ab353f59589af77`.
  Read window: SHA256 `93db7871c2af6c71138feb72c408308c23ba20fe6a1745dd7357a7062fcc1a70`.
  Call (`jsr` at 0xffecdbe6): (unknown callee), target 0xffece54c, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffecdbee): (unknown callee), target 0xffecd4f8, stack arg0=`ret(0xffece54c)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffecdbf4).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffecdbfa).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffecdbfe).
  Grouped accesses: 2 4-byte load records with target `abs 0x300026dc`, including `tst.l` at 0xffecdc08, `move.l` at 0xffecdc48.
  Branch (`ble.b` at 0xffecdc0c): taken to 0xffecdc48 when `[0x300026dc] <= 0x0 (signed, 32-bit)`, falls through to 0xffecdc0e when `[0x300026dc] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x3000043c`, including `movea.l` at 0xffecdc14, `movea.l` at 0xffecdc1e, `movea.l` at 0xffecdc24.
  Halfword store: target `[0x3000043c] +0x88`, value `0x4000` (`move.w` at 0xffecdc1a).
  Word store: target `[0x3000043c] +0x8c`, value `0x0` (`clr.l` at 0xffecdc20).
  Halfword store: target `[0x3000043c] +0x92`, value `0x953` (`move.w` at 0xffecdc2a).
  Call (`jsr` at 0xffecdc3e): (unknown callee), target 0xffedd8e8, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffecdc4e): taken to 0xffecdc94 when `Z == 0`, falls through to 0xffecdc50 when `not (Z == 0)`.
  Byte store: target `[0x3000043c] +0x16`, value `0x8c` (`move.b` at 0xffecdc5a).
  Halfword store: target `[0x3000043c] +0xa0`, value `0x6000` (`move.w` at 0xffecdc64).
  Word store: target `[0x3000043c] +0xa4`, value `0x0` (`clr.l` at 0xffecdc6a).
  Halfword store: target `[0x3000043c] +0xaa`, value `0x953` (`move.w` at 0xffecdc74).
  Call (`jsr` at 0xffecdc8a): (unknown callee), target 0xffedd8e8, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  `rts` at 0xffecdc9a has recorded d0 = `unknown`.

- `0xfff80f98`, `root/6.5.27/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.27:l1#2`, version `1.34.2`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xfff80f98 to 0xfff81054 (exclusive), 188 bytes.
  Carrier: 7523600 bytes, SHA256 `45c872a661d4a4ad20a4cb482fe2aee921ee9ae52784382be125b3d7b7836572`.
  Package: carrier offset 0x1df64c, 1906128 bytes, SHA256 `594a95e42bcabd09a02f4606b39227845ec66070af182a311caa4a0489d87398`.
  Payload: carrier offset 0x1df678, 1906084 bytes, SHA256 `c98c5c63dfded2699bcda8f99352e918b27d3a323447ee576e36ab89fe2e31cf`.
  Read window: SHA256 `76d3341cd602f74256e864c1c43fa7420eed25e2e1eedbd6a6fed14f518d579f`.
  Call (`jsr` at 0xfff80f9e): (unknown callee), target 0xfff81904, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xfff80fa6): (unknown callee), target 0xfff808b0, stack arg0=`ret(0xfff81904)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xfff80fac).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xfff80fb2).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xfff80fb6).
  Grouped accesses: 2 4-byte load records with target `abs 0x300032dc`, including `tst.l` at 0xfff80fc0, `move.l` at 0xfff81000.
  Branch (`ble.b` at 0xfff80fc4): taken to 0xfff81000 when `[0x300032dc] <= 0x0 (signed, 32-bit)`, falls through to 0xfff80fc6 when `[0x300032dc] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000a8c`, including `movea.l` at 0xfff80fcc, `movea.l` at 0xfff80fd6, `movea.l` at 0xfff80fdc.
  Halfword store: target `[0x30000a8c] +0x88`, value `0x4000` (`move.w` at 0xfff80fd2).
  Word store: target `[0x30000a8c] +0x8c`, value `0x0` (`clr.l` at 0xfff80fd8).
  Halfword store: target `[0x30000a8c] +0x92`, value `0x953` (`move.w` at 0xfff80fe2).
  Call (`jsr` at 0xfff80ff6): (unknown callee), target 0xfffa145c, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xfff81006): taken to 0xfff8104c when `Z == 0`, falls through to 0xfff81008 when `not (Z == 0)`.
  Byte store: target `[0x30000a8c] +0x16`, value `0x8c` (`move.b` at 0xfff81012).
  Halfword store: target `[0x30000a8c] +0xa0`, value `0x6000` (`move.w` at 0xfff8101c).
  Word store: target `[0x30000a8c] +0xa4`, value `0x0` (`clr.l` at 0xfff81022).
  Halfword store: target `[0x30000a8c] +0xaa`, value `0x953` (`move.w` at 0xfff8102c).
  Call (`jsr` at 0xfff81042): (unknown callee), target 0xfffa145c, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Recorded d0 at the return is `unknown` (`rts` at 0xfff81052).

- `0xffec5544`, `root/6.5.28/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.28:l1#0`, version `1.38.4`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffec5544 to 0xffec5600 (exclusive), 188 bytes.
  Carrier: 9293972 bytes, SHA256 `e5b65e5f28d7fba6013a8618f55c34bed7f60f7c6f46d7e9c5b8b7b2eabbe7b3`.
  Package: carrier offset 0x0, 982424 bytes, SHA256 `1eb7cb1ad9e96ce311f598424e9b1a836c9c04f585798d659a4e1ddefe362d4b`.
  Payload: carrier offset 0x58, 982336 bytes, SHA256 `56e24f817b86ba5d2a0b48c7a322d287e7481a6a1173db19136547872804bf26`.
  Read window: SHA256 `9e9ada0af4d6bbd7206670eec2f8eafe4ffbdac067f6d441d13cde0be5df4c8f`.
  Call (`jsr` at 0xffec554a): (unknown callee), target 0xffec5b60, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffec5552): (unknown callee), target 0xffec4f8c, stack arg0=`ret(0xffec5b60)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffec5558).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffec555e).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffec5562).
  Grouped accesses: 2 4-byte load records with target `abs 0x30001e40`, including `tst.l` at 0xffec556c, `move.l` at 0xffec55ac.
  Branch (`ble.b` at 0xffec5570): taken to 0xffec55ac when `[0x30001e40] <= 0x0 (signed, 32-bit)`, falls through to 0xffec5572 when `[0x30001e40] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x300002bc`, including `movea.l` at 0xffec5578, `movea.l` at 0xffec5582, `movea.l` at 0xffec5588.
  Halfword store: target `[0x300002bc] +0x88`, value `0x4000` (`move.w` at 0xffec557e).
  Word store: target `[0x300002bc] +0x8c`, value `0x0` (`clr.l` at 0xffec5584).
  Halfword store: target `[0x300002bc] +0x92`, value `0x953` (`move.w` at 0xffec558e).
  Call (`jsr` at 0xffec55a2): (unknown callee), target 0xffeddcd0, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffec55b2): taken to 0xffec55f8 when `Z == 0`, falls through to 0xffec55b4 when `not (Z == 0)`.
  Byte store: target `[0x300002bc] +0x16`, value `0x8c` (`move.b` at 0xffec55be).
  Halfword store: target `[0x300002bc] +0xa0`, value `0x6000` (`move.w` at 0xffec55c8).
  Word store: target `[0x300002bc] +0xa4`, value `0x0` (`clr.l` at 0xffec55ce).
  Halfword store: target `[0x300002bc] +0xaa`, value `0x953` (`move.w` at 0xffec55d8).
  Call (`jsr` at 0xffec55ee): (unknown callee), target 0xffeddcd0, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the return (`rts` at 0xffec55fe), d0 is `unknown`.

- `0xffec5194`, `root/6.5.28/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.28:l1#1`, version `1.38.4`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffec5194 to 0xffec5250 (exclusive), 188 bytes.
  Carrier: 9293972 bytes, SHA256 `e5b65e5f28d7fba6013a8618f55c34bed7f60f7c6f46d7e9c5b8b7b2eabbe7b3`.
  Package: carrier offset 0xefd98, 982380 bytes, SHA256 `ea2b4e11ada0feac76aefd64e259195de58fca3acf9ec4184bf2253c4b8b4d08`.
  Payload: carrier offset 0xefdc4, 982336 bytes, SHA256 `ac159ab4f345a13eb60f0a0cca6c1d5860a166315e65af27e019685d97a1f8e0`.
  Read window: SHA256 `1bb25f2884dd114d9d1e4d538ce7616670249a49fe3e1dd161683b2d42a503d8`.
  Call (`jsr` at 0xffec519a): (unknown callee), target 0xffec57b0, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffec51a2): (unknown callee), target 0xffec4bdc, stack arg0=`ret(0xffec57b0)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffec51a8).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffec51ae).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffec51b2).
  Grouped accesses: 2 4-byte load records with target `abs 0x30001f30`, including `tst.l` at 0xffec51bc, `move.l` at 0xffec51fc.
  Branch (`ble.b` at 0xffec51c0): taken to 0xffec51fc when `[0x30001f30] <= 0x0 (signed, 32-bit)`, falls through to 0xffec51c2 when `[0x30001f30] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x300002e8`, including `movea.l` at 0xffec51c8, `movea.l` at 0xffec51d2, `movea.l` at 0xffec51d8.
  Halfword store: target `[0x300002e8] +0x88`, value `0x4000` (`move.w` at 0xffec51ce).
  Word store: target `[0x300002e8] +0x8c`, value `0x0` (`clr.l` at 0xffec51d4).
  Halfword store: target `[0x300002e8] +0x92`, value `0x953` (`move.w` at 0xffec51de).
  Call (`jsr` at 0xffec51f2): (unknown callee), target 0xffeddb88, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffec5202): taken to 0xffec5248 when `Z == 0`, falls through to 0xffec5204 when `not (Z == 0)`.
  Byte store: target `[0x300002e8] +0x16`, value `0x8c` (`move.b` at 0xffec520e).
  Halfword store: target `[0x300002e8] +0xa0`, value `0x6000` (`move.w` at 0xffec5218).
  Word store: target `[0x300002e8] +0xa4`, value `0x0` (`clr.l` at 0xffec521e).
  Halfword store: target `[0x300002e8] +0xaa`, value `0x953` (`move.w` at 0xffec5228).
  Call (`jsr` at 0xffec523e): (unknown callee), target 0xffeddb88, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  The exit (`rts` at 0xffec524e) reports d0 = `unknown`.

- `0xfff65754`, `root/6.5.28/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.28:l1#2`, version `1.38.4`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xfff65754 to 0xfff65810 (exclusive), 188 bytes.
  Carrier: 9293972 bytes, SHA256 `e5b65e5f28d7fba6013a8618f55c34bed7f60f7c6f46d7e9c5b8b7b2eabbe7b3`.
  Package: carrier offset 0x1dfb04, 1754852 bytes, SHA256 `ea36aeea83c9a9aa85c4bc761d80c960851e77a88d69967aa33d58e96f5294f3`.
  Payload: carrier offset 0x1dfb30, 1754808 bytes, SHA256 `4530117d97540ee5a199d8cb8480cd1799ca11b9bf3538d46217ddb13743008a`.
  Read window: SHA256 `fb373c50b5ca52a0980d9d61c45beb1a7feeb192e8ce524050f9241e742cca2b`.
  Call (`jsr` at 0xfff6575a): (unknown callee), target 0xfff65d70, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xfff65762): (unknown callee), target 0xfff6519c, stack arg0=`ret(0xfff65d70)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xfff65768).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xfff6576e).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xfff65772).
  Grouped accesses: 2 4-byte load records with target `abs 0x3000282c`, including `tst.l` at 0xfff6577c, `move.l` at 0xfff657bc.
  Branch (`ble.b` at 0xfff65780): taken to 0xfff657bc when `[0x3000282c] <= 0x0 (signed, 32-bit)`, falls through to 0xfff65782 when `[0x3000282c] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x300008bc`, including `movea.l` at 0xfff65788, `movea.l` at 0xfff65792, `movea.l` at 0xfff65798.
  Halfword store: target `[0x300008bc] +0x88`, value `0x4000` (`move.w` at 0xfff6578e).
  Word store: target `[0x300008bc] +0x8c`, value `0x0` (`clr.l` at 0xfff65794).
  Halfword store: target `[0x300008bc] +0x92`, value `0x953` (`move.w` at 0xfff6579e).
  Call (`jsr` at 0xfff657b2): (unknown callee), target 0xfff80ae0, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xfff657c2): taken to 0xfff65808 when `Z == 0`, falls through to 0xfff657c4 when `not (Z == 0)`.
  Byte store: target `[0x300008bc] +0x16`, value `0x8c` (`move.b` at 0xfff657ce).
  Halfword store: target `[0x300008bc] +0xa0`, value `0x6000` (`move.w` at 0xfff657d8).
  Word store: target `[0x300008bc] +0xa4`, value `0x0` (`clr.l` at 0xfff657de).
  Halfword store: target `[0x300008bc] +0xaa`, value `0x953` (`move.w` at 0xfff657e8).
  Call (`jsr` at 0xfff657fe): (unknown callee), target 0xfff80ae0, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  `rts` at 0xfff6580e has recorded d0 = `unknown`.

- `0xfff6345c`, `root/6.5.28/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.28:l1#3`, version `1.38.4`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xfff6345c to 0xfff63518 (exclusive), 188 bytes.
  Carrier: 9293972 bytes, SHA256 `e5b65e5f28d7fba6013a8618f55c34bed7f60f7c6f46d7e9c5b8b7b2eabbe7b3`.
  Package: carrier offset 0x38c1e8, 1772984 bytes, SHA256 `2094d418e7129ff920233c3f5385fc7bb6bb6e65cbd203164e10250c539b2e1c`.
  Payload: carrier offset 0x38c214, 1772940 bytes, SHA256 `c09284761067f8587cbe2029bca484448b73c1e2c43870af5fd5cb6fa5d0f358`.
  Read window: SHA256 `4e40ad33644a329dd767a07d1eb2c26cf76813aec25c29eaf03f24746b891e4d`.
  Call (`jsr` at 0xfff63462): (unknown callee), target 0xfff6378c, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xfff6346a): (unknown callee), target 0xfff63060, stack arg0=`ret(0xfff6378c)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xfff63470).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xfff63476).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xfff6347a).
  Grouped accesses: 2 4-byte load records with target `abs 0x300032f4`, including `tst.l` at 0xfff63484, `move.l` at 0xfff634c4.
  Branch (`ble.b` at 0xfff63488): taken to 0xfff634c4 when `[0x300032f4] <= 0x0 (signed, 32-bit)`, falls through to 0xfff6348a when `[0x300032f4] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000ba8`, including `movea.l` at 0xfff63490, `movea.l` at 0xfff6349a, `movea.l` at 0xfff634a0.
  Halfword store: target `[0x30000ba8] +0x88`, value `0x4000` (`move.w` at 0xfff63496).
  Word store: target `[0x30000ba8] +0x8c`, value `0x0` (`clr.l` at 0xfff6349c).
  Halfword store: target `[0x30000ba8] +0x92`, value `0x953` (`move.w` at 0xfff634a6).
  Call (`jsr` at 0xfff634ba): (unknown callee), target 0xfff850ac, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xfff634ca): taken to 0xfff63510 when `Z == 0`, falls through to 0xfff634cc when `not (Z == 0)`.
  Byte store: target `[0x30000ba8] +0x16`, value `0x8c` (`move.b` at 0xfff634d6).
  Halfword store: target `[0x30000ba8] +0xa0`, value `0x6000` (`move.w` at 0xfff634e0).
  Word store: target `[0x30000ba8] +0xa4`, value `0x0` (`clr.l` at 0xfff634e6).
  Halfword store: target `[0x30000ba8] +0xaa`, value `0x953` (`move.w` at 0xfff634f0).
  Call (`jsr` at 0xfff63506): (unknown callee), target 0xfff850ac, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Recorded d0 at the return is `unknown` (`rts` at 0xfff63516).

- `0xffec8450`, `root/6.5.29/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.29:l1#0`, version `1.40.5`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffec8450 to 0xffec850c (exclusive), 188 bytes.
  Carrier: 9295164 bytes, SHA256 `80f4a616601d187bc3839a2758f2e7f2d0db962ad7fd8f2d96d41bcd15bdf88c`.
  Package: carrier offset 0x0, 982440 bytes, SHA256 `d1724d58c910a9bd1510b531e99c15d94ed1b429687131cd78214bc3876a0037`.
  Payload: carrier offset 0x58, 982352 bytes, SHA256 `d36178e16f6b36f7c1f5940320acbd041cd1ebcae18e5fec7b10734086e98a0e`.
  Read window: SHA256 `bcb4860b47a5b756b3ec82a1988fa5c1f65dfc1d32fe5705ad294a29bc7d8b9e`.
  Call (`jsr` at 0xffec8456): (unknown callee), target 0xffec8a6c, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffec845e): (unknown callee), target 0xffec7e98, stack arg0=`ret(0xffec8a6c)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffec8464).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffec846a).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffec846e).
  Grouped accesses: 2 4-byte load records with target `abs 0x30001eac`, including `tst.l` at 0xffec8478, `move.l` at 0xffec84b8.
  Branch (`ble.b` at 0xffec847c): taken to 0xffec84b8 when `[0x30001eac] <= 0x0 (signed, 32-bit)`, falls through to 0xffec847e when `[0x30001eac] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x3000031c`, including `movea.l` at 0xffec8484, `movea.l` at 0xffec848e, `movea.l` at 0xffec8494.
  Halfword store: target `[0x3000031c] +0x88`, value `0x4000` (`move.w` at 0xffec848a).
  Word store: target `[0x3000031c] +0x8c`, value `0x0` (`clr.l` at 0xffec8490).
  Halfword store: target `[0x3000031c] +0x92`, value `0x953` (`move.w` at 0xffec849a).
  Call (`jsr` at 0xffec84ae): (unknown callee), target 0xffeddce0, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffec84be): taken to 0xffec8504 when `Z == 0`, falls through to 0xffec84c0 when `not (Z == 0)`.
  Byte store: target `[0x3000031c] +0x16`, value `0x8c` (`move.b` at 0xffec84ca).
  Halfword store: target `[0x3000031c] +0xa0`, value `0x6000` (`move.w` at 0xffec84d4).
  Word store: target `[0x3000031c] +0xa4`, value `0x0` (`clr.l` at 0xffec84da).
  Halfword store: target `[0x3000031c] +0xaa`, value `0x953` (`move.w` at 0xffec84e4).
  Call (`jsr` at 0xffec84fa): (unknown callee), target 0xffeddce0, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the return (`rts` at 0xffec850a), d0 is `unknown`.

- `0xffec8110`, `root/6.5.29/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.29:l1#1`, version `1.40.5`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffec8110 to 0xffec81cc (exclusive), 188 bytes.
  Carrier: 9295164 bytes, SHA256 `80f4a616601d187bc3839a2758f2e7f2d0db962ad7fd8f2d96d41bcd15bdf88c`.
  Package: carrier offset 0xefda8, 982396 bytes, SHA256 `67295cdae5ff370410852805127e6f410676e912b07eab1881c2caca7f70e5c5`.
  Payload: carrier offset 0xefdd4, 982352 bytes, SHA256 `1ba25f46035369eeb52170440fd7a6e8dd4b3ef74024f3e9bbaded05b899f91a`.
  Read window: SHA256 `dd845bbf34505d58ea1d9db2e3d9242ee083a50f749a14d85973f89083ad3118`.
  Call (`jsr` at 0xffec8116): (unknown callee), target 0xffec872c, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffec811e): (unknown callee), target 0xffec7b58, stack arg0=`ret(0xffec872c)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffec8124).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffec812a).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffec812e).
  Grouped accesses: 2 4-byte load records with target `abs 0x30001fa0`, including `tst.l` at 0xffec8138, `move.l` at 0xffec8178.
  Branch (`ble.b` at 0xffec813c): taken to 0xffec8178 when `[0x30001fa0] <= 0x0 (signed, 32-bit)`, falls through to 0xffec813e when `[0x30001fa0] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000348`, including `movea.l` at 0xffec8144, `movea.l` at 0xffec814e, `movea.l` at 0xffec8154.
  Halfword store: target `[0x30000348] +0x88`, value `0x4000` (`move.w` at 0xffec814a).
  Word store: target `[0x30000348] +0x8c`, value `0x0` (`clr.l` at 0xffec8150).
  Halfword store: target `[0x30000348] +0x92`, value `0x953` (`move.w` at 0xffec815a).
  Call (`jsr` at 0xffec816e): (unknown callee), target 0xffeddb98, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffec817e): taken to 0xffec81c4 when `Z == 0`, falls through to 0xffec8180 when `not (Z == 0)`.
  Byte store: target `[0x30000348] +0x16`, value `0x8c` (`move.b` at 0xffec818a).
  Halfword store: target `[0x30000348] +0xa0`, value `0x6000` (`move.w` at 0xffec8194).
  Word store: target `[0x30000348] +0xa4`, value `0x0` (`clr.l` at 0xffec819a).
  Halfword store: target `[0x30000348] +0xaa`, value `0x953` (`move.w` at 0xffec81a4).
  Call (`jsr` at 0xffec81ba): (unknown callee), target 0xffeddb98, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  The exit (`rts` at 0xffec81ca) reports d0 = `unknown`.

- `0xfff68ec8`, `root/6.5.29/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.29:l1#2`, version `1.40.5`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xfff68ec8 to 0xfff68f84 (exclusive), 188 bytes.
  Carrier: 9295164 bytes, SHA256 `80f4a616601d187bc3839a2758f2e7f2d0db962ad7fd8f2d96d41bcd15bdf88c`.
  Package: carrier offset 0x1dfb24, 1754868 bytes, SHA256 `728c3b39b0aaebd3f8e0ee49e7c42f0d3877d4b8cff52cb06fc0ea4606612802`.
  Payload: carrier offset 0x1dfb50, 1754824 bytes, SHA256 `6d104cefdb289169aa434cde1e6641c316128c7d81149d27fbe448b172d00459`.
  Read window: SHA256 `5d72d40ee0165419edfb0abf93926fc8212c2c4cea6364e04cfc7a2acb427355`.
  Call (`jsr` at 0xfff68ece): (unknown callee), target 0xfff694e4, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xfff68ed6): (unknown callee), target 0xfff68910, stack arg0=`ret(0xfff694e4)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xfff68edc).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xfff68ee2).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xfff68ee6).
  Grouped accesses: 2 4-byte load records with target `abs 0x3000289c`, including `tst.l` at 0xfff68ef0, `move.l` at 0xfff68f30.
  Branch (`ble.b` at 0xfff68ef4): taken to 0xfff68f30 when `[0x3000289c] <= 0x0 (signed, 32-bit)`, falls through to 0xfff68ef6 when `[0x3000289c] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x3000092c`, including `movea.l` at 0xfff68efc, `movea.l` at 0xfff68f06, `movea.l` at 0xfff68f0c.
  Halfword store: target `[0x3000092c] +0x88`, value `0x4000` (`move.w` at 0xfff68f02).
  Word store: target `[0x3000092c] +0x8c`, value `0x0` (`clr.l` at 0xfff68f08).
  Halfword store: target `[0x3000092c] +0x92`, value `0x953` (`move.w` at 0xfff68f12).
  Call (`jsr` at 0xfff68f26): (unknown callee), target 0xfff80af0, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xfff68f36): taken to 0xfff68f7c when `Z == 0`, falls through to 0xfff68f38 when `not (Z == 0)`.
  Byte store: target `[0x3000092c] +0x16`, value `0x8c` (`move.b` at 0xfff68f42).
  Halfword store: target `[0x3000092c] +0xa0`, value `0x6000` (`move.w` at 0xfff68f4c).
  Word store: target `[0x3000092c] +0xa4`, value `0x0` (`clr.l` at 0xfff68f52).
  Halfword store: target `[0x3000092c] +0xaa`, value `0x953` (`move.w` at 0xfff68f5c).
  Call (`jsr` at 0xfff68f72): (unknown callee), target 0xfff80af0, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  `rts` at 0xfff68f82 has recorded d0 = `unknown`.

- `0xfff637d8`, `root/6.5.29/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.29:l1#3`, version `1.40.5`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xfff637d8 to 0xfff63894 (exclusive), 188 bytes.
  Carrier: 9295164 bytes, SHA256 `80f4a616601d187bc3839a2758f2e7f2d0db962ad7fd8f2d96d41bcd15bdf88c`.
  Package: carrier offset 0x38c218, 1774128 bytes, SHA256 `17388cb36d1c3b9ac35e33e5964e0641334b6b24dd1f787d7d81d0fe3c494a2c`.
  Payload: carrier offset 0x38c244, 1774084 bytes, SHA256 `2028cd13279e10c398b6e1b86e9db5d8fb7802cdddb4de5912588c8da736ecc0`.
  Read window: SHA256 `98cccb2d3e7e2f93c88b8312d0c552120b9cde4f42455156252f04d343929568`.
  Call (`jsr` at 0xfff637de): (unknown callee), target 0xfff63b08, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xfff637e6): (unknown callee), target 0xfff633dc, stack arg0=`ret(0xfff63b08)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xfff637ec).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xfff637f2).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xfff637f6).
  Grouped accesses: 2 4-byte load records with target `abs 0x30003338`, including `tst.l` at 0xfff63800, `move.l` at 0xfff63840.
  Branch (`ble.b` at 0xfff63804): taken to 0xfff63840 when `[0x30003338] <= 0x0 (signed, 32-bit)`, falls through to 0xfff63806 when `[0x30003338] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000ba8`, including `movea.l` at 0xfff6380c, `movea.l` at 0xfff63816, `movea.l` at 0xfff6381c.
  Halfword store: target `[0x30000ba8] +0x88`, value `0x4000` (`move.w` at 0xfff63812).
  Word store: target `[0x30000ba8] +0x8c`, value `0x0` (`clr.l` at 0xfff63818).
  Halfword store: target `[0x30000ba8] +0x92`, value `0x953` (`move.w` at 0xfff63822).
  Call (`jsr` at 0xfff63836): (unknown callee), target 0xfff85428, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xfff63846): taken to 0xfff6388c when `Z == 0`, falls through to 0xfff63848 when `not (Z == 0)`.
  Byte store: target `[0x30000ba8] +0x16`, value `0x8c` (`move.b` at 0xfff63852).
  Halfword store: target `[0x30000ba8] +0xa0`, value `0x6000` (`move.w` at 0xfff6385c).
  Word store: target `[0x30000ba8] +0xa4`, value `0x0` (`clr.l` at 0xfff63862).
  Halfword store: target `[0x30000ba8] +0xaa`, value `0x953` (`move.w` at 0xfff6386c).
  Call (`jsr` at 0xfff63882): (unknown callee), target 0xfff85428, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Recorded d0 at the return is `unknown` (`rts` at 0xfff63892).

- `0xffec3430`, `root/cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`.
  Package `cd-ist-3.24:l1#0`, version `1.62.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffec3430 to 0xffec34ec (exclusive), 188 bytes.
  Carrier: 13258744 bytes, SHA256 `b4b5634949a93f05ad6eb24462e6cd7f10104dd42ff5c2ae8616dd5d87697790`.
  Package: carrier offset 0x0, 948616 bytes, SHA256 `e8f384de115f495ea87f86c045ab40c27a27d9b7c957bd5d39627c49881eaa59`.
  Payload: carrier offset 0x58, 948528 bytes, SHA256 `0f9d1cee2a1c4c7000fd322ccbb4f09f897551825463f7e123c35239905fbad8`.
  Read window: SHA256 `8d299b92b269ae92c01f3c1c028e860dcbc5feef902e67a493c7aefd991bb31f`.
  Call (`jsr` at 0xffec3436): (unknown callee), target 0xffec3a4c, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffec343e): (unknown callee), target 0xffec2e78, stack arg0=`ret(0xffec3a4c)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffec3444).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffec344a).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffec344e).
  Grouped accesses: 2 4-byte load records with target `abs 0x30002154`, including `tst.l` at 0xffec3458, `move.l` at 0xffec3498.
  Branch (`ble.b` at 0xffec345c): taken to 0xffec3498 when `[0x30002154] <= 0x0 (signed, 32-bit)`, falls through to 0xffec345e when `[0x30002154] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000380`, including `movea.l` at 0xffec3464, `movea.l` at 0xffec346e, `movea.l` at 0xffec3474.
  Halfword store: target `[0x30000380] +0x88`, value `0x4000` (`move.w` at 0xffec346a).
  Word store: target `[0x30000380] +0x8c`, value `0x0` (`clr.l` at 0xffec3470).
  Halfword store: target `[0x30000380] +0x92`, value `0x953` (`move.w` at 0xffec347a).
  Call (`jsr` at 0xffec348e): (unknown callee), target 0xffed580c, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffec349e): taken to 0xffec34e4 when `Z == 0`, falls through to 0xffec34a0 when `not (Z == 0)`.
  Byte store: target `[0x30000380] +0x16`, value `0x8c` (`move.b` at 0xffec34aa).
  Halfword store: target `[0x30000380] +0xa0`, value `0x6000` (`move.w` at 0xffec34b4).
  Word store: target `[0x30000380] +0xa4`, value `0x0` (`clr.l` at 0xffec34ba).
  Halfword store: target `[0x30000380] +0xaa`, value `0x953` (`move.w` at 0xffec34c4).
  Call (`jsr` at 0xffec34da): (unknown callee), target 0xffed580c, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the return (`rts` at 0xffec34ea), d0 is `unknown`.

- `0xfff6104c`, `root/cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`.
  Package `cd-ist-3.24:l1#1`, version `1.62.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xfff6104c to 0xfff61108 (exclusive), 188 bytes.
  Carrier: 13258744 bytes, SHA256 `b4b5634949a93f05ad6eb24462e6cd7f10104dd42ff5c2ae8616dd5d87697790`.
  Package: carrier offset 0xe7988, 1772496 bytes, SHA256 `45223c158690b9ea637f65a9710f4af32dd05bc0bd83dd3fc54216042ee0fd2d`.
  Payload: carrier offset 0xe79b4, 1772452 bytes, SHA256 `25561f492def55deebc646f821a1cd5d9b94277e69988c393dfa3df7f8622d7b`.
  Read window: SHA256 `9cbe955a25645862c44c0ee9460f451e7a8e999a08a4578cff0aa72a294d501d`.
  Call (`jsr` at 0xfff61052): (unknown callee), target 0xfff6137c, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xfff6105a): (unknown callee), target 0xfff60c50, stack arg0=`ret(0xfff6137c)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xfff61060).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xfff61066).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xfff6106a).
  Grouped accesses: 2 4-byte load records with target `abs 0x30002e6c`, including `tst.l` at 0xfff61074, `move.l` at 0xfff610b4.
  Branch (`ble.b` at 0xfff61078): taken to 0xfff610b4 when `[0x30002e6c] <= 0x0 (signed, 32-bit)`, falls through to 0xfff6107a when `[0x30002e6c] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000b50`, including `movea.l` at 0xfff61080, `movea.l` at 0xfff6108a, `movea.l` at 0xfff61090.
  Halfword store: target `[0x30000b50] +0x88`, value `0x4000` (`move.w` at 0xfff61086).
  Word store: target `[0x30000b50] +0x8c`, value `0x0` (`clr.l` at 0xfff6108c).
  Halfword store: target `[0x30000b50] +0x92`, value `0x953` (`move.w` at 0xfff61096).
  Call (`jsr` at 0xfff610aa): (unknown callee), target 0xfff84f30, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xfff610ba): taken to 0xfff61100 when `Z == 0`, falls through to 0xfff610bc when `not (Z == 0)`.
  Byte store: target `[0x30000b50] +0x16`, value `0x8c` (`move.b` at 0xfff610c6).
  Halfword store: target `[0x30000b50] +0xa0`, value `0x6000` (`move.w` at 0xfff610d0).
  Word store: target `[0x30000b50] +0xa4`, value `0x0` (`clr.l` at 0xfff610d6).
  Halfword store: target `[0x30000b50] +0xaa`, value `0x953` (`move.w` at 0xfff610e0).
  Call (`jsr` at 0xfff610f6): (unknown callee), target 0xfff84f30, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  The exit (`rts` at 0xfff61106) reports d0 = `unknown`.

- `0xfff6b338`, `root/cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`.
  Package `cd-ist-3.24:l1#3`, version `1.62.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xfff6b338 to 0xfff6b3f4 (exclusive), 188 bytes.
  Carrier: 13258744 bytes, SHA256 `b4b5634949a93f05ad6eb24462e6cd7f10104dd42ff5c2ae8616dd5d87697790`.
  Package: carrier offset 0x63c654, 1771696 bytes, SHA256 `14b7122d6b6c072d9b4d2425f07ad68041807d535084e8c3501b512e017aa66e`.
  Payload: carrier offset 0x63c680, 1771652 bytes, SHA256 `fdb1a5444e0e11bd6d24bb080b3e5795a2dd340a2146cc1ced74419597d8f339`.
  Read window: SHA256 `1712fb92ebd064482a333fd83881a3ce3aeafc15dfe59d62e60f16d23b3eb2d7`.
  Call (`jsr` at 0xfff6b33e): (unknown callee), target 0xfff6b3f4, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xfff6b346): (unknown callee), target 0xfff6af3c, stack arg0=`ret(0xfff6b3f4)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xfff6b34c).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xfff6b352).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xfff6b356).
  Grouped accesses: 2 4-byte load records with target `abs 0x30003438`, including `tst.l` at 0xfff6b360, `move.l` at 0xfff6b3a0.
  Branch (`ble.b` at 0xfff6b364): taken to 0xfff6b3a0 when `[0x30003438] <= 0x0 (signed, 32-bit)`, falls through to 0xfff6b366 when `[0x30003438] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000bd4`, including `movea.l` at 0xfff6b36c, `movea.l` at 0xfff6b376, `movea.l` at 0xfff6b37c.
  Halfword store: target `[0x30000bd4] +0x88`, value `0x4000` (`move.w` at 0xfff6b372).
  Word store: target `[0x30000bd4] +0x8c`, value `0x0` (`clr.l` at 0xfff6b378).
  Halfword store: target `[0x30000bd4] +0x92`, value `0x953` (`move.w` at 0xfff6b382).
  Call (`jsr` at 0xfff6b396): (unknown callee), target 0xfff83564, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xfff6b3a6): taken to 0xfff6b3ec when `Z == 0`, falls through to 0xfff6b3a8 when `not (Z == 0)`.
  Byte store: target `[0x30000bd4] +0x16`, value `0x8c` (`move.b` at 0xfff6b3b2).
  Halfword store: target `[0x30000bd4] +0xa0`, value `0x6000` (`move.w` at 0xfff6b3bc).
  Word store: target `[0x30000bd4] +0xa4`, value `0x0` (`clr.l` at 0xfff6b3c2).
  Halfword store: target `[0x30000bd4] +0xaa`, value `0x953` (`move.w` at 0xfff6b3cc).
  Call (`jsr` at 0xfff6b3e2): (unknown callee), target 0xfff83564, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  `rts` at 0xfff6b3f2 has recorded d0 = `unknown`.
