# ColdFire L1 pointer writes in IRIX 6.5.23 through 6.5.27

Each entry applies to its exact carrier, package and uncompressed payload.
The payload is mapped at the stated load address.
ColdFire classification comes from the package metadata.
The 68040 common-subset decoder does not independently establish the silicon identity.

The analysis spans are inferred prologue-to-first-RTS windows, not proved function sizes.
Their complete decoded contents and bounded entry, caller and epilogue contexts were read.
The two reported conditional branch targets lie within each selected span.
Indirect effects, hardware roles and any additional function extent remain unknown.
No cross-image code equivalence is assumed.

`sp` is the entry stack pointer.
The six stack slots at each call are candidate locations under an assumed m68k-sysv convention.
Arity and any custom ABI remain unknown.
A `ret(F)` expression does not identify its producing instruction.
Types, pointee meanings and runtime path feasibility remain unknown.
Grouped accesses describe static instruction shapes.
Separate loads do not establish equal loaded values, and conditional arms do not form one unconditional sequence.
Values printed as `unknown` stay unresolved.

The [complete facts](disasm/coldfire-variants/pointer-write-6.5.23-6.5.27-facts.txt) retain the full analyzed windows.
Auxiliary truncated decoder observations were rejected.
Complete caller and successor contexts were checked separately.
Range ends are exclusive.

- `0xffe5df3c`, `root/6.5.27/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.27:l1#0`, version `1.34.2`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe5df3c to 0xffe5dff8 (exclusive), 188 bytes.
  Carrier: 7523600 bytes, SHA256 `45c872a661d4a4ad20a4cb482fe2aee921ee9ae52784382be125b3d7b7836572`.
  Package: carrier offset 0x0, 981832 bytes, SHA256 `d3968aabd51125c60c759766ceed4c9fc66321e66e4e562d235bc47b16efb14d`.
  Payload: carrier offset 0x50, 981752 bytes, SHA256 `117f4126e840ee43eac0d96eb2aede9ff454f78b7c32a3aa42b68fcbe034e91b`.
  Read window: SHA256 `f0cfcb6dad72e9712bb94ea1390f3aa74655c25e711bf4deabcf1ee8d949c99c`.
  Call (`jsr` at 0xffe5df42): (unknown callee), target 0xffe5e8a8, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe5df4a): (unknown callee), target 0xffe5d854, stack arg0=`ret(0xffe5e8a8)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe5df50).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe5df56).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe5df5a).
  Grouped accesses: 2 4-byte load records with target `abs 0x300025e8`, including `tst.l` at 0xffe5df64, `move.l` at 0xffe5dfa4.
  Branch (`ble.b` at 0xffe5df68): taken to 0xffe5dfa4 when `[0x300025e8] <= 0x0 (signed, 32-bit)`, falls through to 0xffe5df6a when `[0x300025e8] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000410`, including `movea.l` at 0xffe5df70, `movea.l` at 0xffe5df7a, `movea.l` at 0xffe5df80.
  Halfword store: target `[0x30000410] +0x88`, value `0x4000` (`move.w` at 0xffe5df76).
  Word store: target `[0x30000410] +0x8c`, value `0x0` (`clr.l` at 0xffe5df7c).
  Halfword store: target `[0x30000410] +0x92`, value `0x953` (`move.w` at 0xffe5df86).
  Call (`jsr` at 0xffe5df9a): (unknown callee), target 0xffe6da4c, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe5dfaa): taken to 0xffe5dff0 when `Z == 0`, falls through to 0xffe5dfac when `not (Z == 0)`.
  Byte store: target `[0x30000410] +0x16`, value `0x8c` (`move.b` at 0xffe5dfb6).
  Halfword store: target `[0x30000410] +0xa0`, value `0x6000` (`move.w` at 0xffe5dfc0).
  Word store: target `[0x30000410] +0xa4`, value `0x0` (`clr.l` at 0xffe5dfc6).
  Halfword store: target `[0x30000410] +0xaa`, value `0x953` (`move.w` at 0xffe5dfd0).
  Call (`jsr` at 0xffe5dfe6): (unknown callee), target 0xffe6da4c, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the return (`rts` at 0xffe5dff6), d0 is `unknown`.

- `0xffe5dbe0`, `root/6.5.27/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.27:l1#1`, version `1.34.2`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe5dbe0 to 0xffe5dc9c (exclusive), 188 bytes.
  Carrier: 7523600 bytes, SHA256 `45c872a661d4a4ad20a4cb482fe2aee921ee9ae52784382be125b3d7b7836572`.
  Package: carrier offset 0xefb48, 981764 bytes, SHA256 `28f6fdb9ba631c1fc226bf9d30b1812e4d5df62e06959929711f7340d3ed1c1d`.
  Payload: carrier offset 0xefb74, 981720 bytes, SHA256 `38a4b37992f18dfaaf632b9051ebd921251d6776f17c90b93ab353f59589af77`.
  Read window: SHA256 `e9e348cc62b1773d14a132fcb7ef744259d5a6a4b9c5cd284434fb43c5515aa3`.
  Call (`jsr` at 0xffe5dbe6): (unknown callee), target 0xffe5e54c, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe5dbee): (unknown callee), target 0xffe5d4f8, stack arg0=`ret(0xffe5e54c)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe5dbf4).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe5dbfa).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe5dbfe).
  Grouped accesses: 2 4-byte load records with target `abs 0x300026dc`, including `tst.l` at 0xffe5dc08, `move.l` at 0xffe5dc48.
  Branch (`ble.b` at 0xffe5dc0c): taken to 0xffe5dc48 when `[0x300026dc] <= 0x0 (signed, 32-bit)`, falls through to 0xffe5dc0e when `[0x300026dc] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x3000043c`, including `movea.l` at 0xffe5dc14, `movea.l` at 0xffe5dc1e, `movea.l` at 0xffe5dc24.
  Halfword store: target `[0x3000043c] +0x88`, value `0x4000` (`move.w` at 0xffe5dc1a).
  Word store: target `[0x3000043c] +0x8c`, value `0x0` (`clr.l` at 0xffe5dc20).
  Halfword store: target `[0x3000043c] +0x92`, value `0x953` (`move.w` at 0xffe5dc2a).
  Call (`jsr` at 0xffe5dc3e): (unknown callee), target 0xffe6d8e8, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe5dc4e): taken to 0xffe5dc94 when `Z == 0`, falls through to 0xffe5dc50 when `not (Z == 0)`.
  Byte store: target `[0x3000043c] +0x16`, value `0x8c` (`move.b` at 0xffe5dc5a).
  Halfword store: target `[0x3000043c] +0xa0`, value `0x6000` (`move.w` at 0xffe5dc64).
  Word store: target `[0x3000043c] +0xa4`, value `0x0` (`clr.l` at 0xffe5dc6a).
  Halfword store: target `[0x3000043c] +0xaa`, value `0x953` (`move.w` at 0xffe5dc74).
  Call (`jsr` at 0xffe5dc8a): (unknown callee), target 0xffe6d8e8, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  The exit (`rts` at 0xffe5dc9a) reports d0 = `unknown`.

- `0xffe90f98`, `root/6.5.27/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.27:l1#2`, version `1.34.2`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe90f98 to 0xffe91054 (exclusive), 188 bytes.
  Carrier: 7523600 bytes, SHA256 `45c872a661d4a4ad20a4cb482fe2aee921ee9ae52784382be125b3d7b7836572`.
  Package: carrier offset 0x1df64c, 1906128 bytes, SHA256 `594a95e42bcabd09a02f4606b39227845ec66070af182a311caa4a0489d87398`.
  Payload: carrier offset 0x1df678, 1906084 bytes, SHA256 `c98c5c63dfded2699bcda8f99352e918b27d3a323447ee576e36ab89fe2e31cf`.
  Read window: SHA256 `3c859a056282d94d4e47c18fce29419cdcd5e9c072e354accf1a96ac19f452fe`.
  Call (`jsr` at 0xffe90f9e): (unknown callee), target 0xffe91904, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe90fa6): (unknown callee), target 0xffe908b0, stack arg0=`ret(0xffe91904)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe90fac).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe90fb2).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe90fb6).
  Grouped accesses: 2 4-byte load records with target `abs 0x300032dc`, including `tst.l` at 0xffe90fc0, `move.l` at 0xffe91000.
  Branch (`ble.b` at 0xffe90fc4): taken to 0xffe91000 when `[0x300032dc] <= 0x0 (signed, 32-bit)`, falls through to 0xffe90fc6 when `[0x300032dc] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000a8c`, including `movea.l` at 0xffe90fcc, `movea.l` at 0xffe90fd6, `movea.l` at 0xffe90fdc.
  Halfword store: target `[0x30000a8c] +0x88`, value `0x4000` (`move.w` at 0xffe90fd2).
  Word store: target `[0x30000a8c] +0x8c`, value `0x0` (`clr.l` at 0xffe90fd8).
  Halfword store: target `[0x30000a8c] +0x92`, value `0x953` (`move.w` at 0xffe90fe2).
  Call (`jsr` at 0xffe90ff6): (unknown callee), target 0xffeb145c, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe91006): taken to 0xffe9104c when `Z == 0`, falls through to 0xffe91008 when `not (Z == 0)`.
  Byte store: target `[0x30000a8c] +0x16`, value `0x8c` (`move.b` at 0xffe91012).
  Halfword store: target `[0x30000a8c] +0xa0`, value `0x6000` (`move.w` at 0xffe9101c).
  Word store: target `[0x30000a8c] +0xa4`, value `0x0` (`clr.l` at 0xffe91022).
  Halfword store: target `[0x30000a8c] +0xaa`, value `0x953` (`move.w` at 0xffe9102c).
  Call (`jsr` at 0xffe91042): (unknown callee), target 0xffeb145c, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  `rts` at 0xffe91052 has recorded d0 = `unknown`.

- `0xffe5d208`, `root/6.5.26/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.26:l1#0`, version `1.32.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe5d208 to 0xffe5d2c4 (exclusive), 188 bytes.
  Carrier: 3846136 bytes, SHA256 `1fdcc3409f4e0633335d9b51d506818ce971dacbe6ababe1d4fbcbd52855e3dc`.
  Package: carrier offset 0x0, 978796 bytes, SHA256 `69539d85355ecdbbf84627ba00e20cfbc7662fbbebb071c8ac525a43150aaa0d`.
  Payload: carrier offset 0x48, 978724 bytes, SHA256 `92a0694b91b363f898d9d22a3072fc2f9932830b683e2b071f3df45f7329f07e`.
  Read window: SHA256 `80614b5f3c10957fe7956add0de6b3fdc7ed32b8f83e9b6ea7ed6ff8ded579f1`.
  Call (`jsr` at 0xffe5d20e): (unknown callee), target 0xffe5db74, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe5d216): (unknown callee), target 0xffe5cb20, stack arg0=`ret(0xffe5db74)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe5d21c).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe5d222).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe5d226).
  Grouped accesses: 2 4-byte load records with target `abs 0x30002584`, including `tst.l` at 0xffe5d230, `move.l` at 0xffe5d270.
  Branch (`ble.b` at 0xffe5d234): taken to 0xffe5d270 when `[0x30002584] <= 0x0 (signed, 32-bit)`, falls through to 0xffe5d236 when `[0x30002584] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x300003f8`, including `movea.l` at 0xffe5d23c, `movea.l` at 0xffe5d246, `movea.l` at 0xffe5d24c.
  Halfword store: target `[0x300003f8] +0x88`, value `0x4000` (`move.w` at 0xffe5d242).
  Word store: target `[0x300003f8] +0x8c`, value `0x0` (`clr.l` at 0xffe5d248).
  Halfword store: target `[0x300003f8] +0x92`, value `0x953` (`move.w` at 0xffe5d252).
  Call (`jsr` at 0xffe5d266): (unknown callee), target 0xffe6c9f4, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe5d276): taken to 0xffe5d2bc when `Z == 0`, falls through to 0xffe5d278 when `not (Z == 0)`.
  Byte store: target `[0x300003f8] +0x16`, value `0x8c` (`move.b` at 0xffe5d282).
  Halfword store: target `[0x300003f8] +0xa0`, value `0x6000` (`move.w` at 0xffe5d28c).
  Word store: target `[0x300003f8] +0xa4`, value `0x0` (`clr.l` at 0xffe5d292).
  Halfword store: target `[0x300003f8] +0xaa`, value `0x953` (`move.w` at 0xffe5d29c).
  Call (`jsr` at 0xffe5d2b2): (unknown callee), target 0xffe6c9f4, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Recorded d0 at the return is `unknown` (`rts` at 0xffe5d2c2).

- `0xffe5dbb0`, `root/6.5.26/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.26:l1#1`, version `1.32.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe5dbb0 to 0xffe5dc6c (exclusive), 188 bytes.
  Carrier: 3846136 bytes, SHA256 `1fdcc3409f4e0633335d9b51d506818ce971dacbe6ababe1d4fbcbd52855e3dc`.
  Package: carrier offset 0xeef6c, 982112 bytes, SHA256 `81744f9b80dc588fc8c607763d38aa7fe926adf437202b23befbd9932ec14368`.
  Payload: carrier offset 0xeef98, 982068 bytes, SHA256 `fe1b451b03bf7c29e2e09011317c999cba37b2ea2c8bd89f316497d756cc56a9`.
  Read window: SHA256 `65f2ec9ad357aef522ab8cf06ab6b0ed6882ea921e5e9f801b5113bc0e26e3ae`.
  Call (`jsr` at 0xffe5dbb6): (unknown callee), target 0xffe5e51c, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe5dbbe): (unknown callee), target 0xffe5d4c8, stack arg0=`ret(0xffe5e51c)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe5dbc4).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe5dbca).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe5dbce).
  Grouped accesses: 2 4-byte load records with target `abs 0x30002690`, including `tst.l` at 0xffe5dbd8, `move.l` at 0xffe5dc18.
  Branch (`ble.b` at 0xffe5dbdc): taken to 0xffe5dc18 when `[0x30002690] <= 0x0 (signed, 32-bit)`, falls through to 0xffe5dbde when `[0x30002690] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000434`, including `movea.l` at 0xffe5dbe4, `movea.l` at 0xffe5dbee, `movea.l` at 0xffe5dbf4.
  Halfword store: target `[0x30000434] +0x88`, value `0x4000` (`move.w` at 0xffe5dbea).
  Word store: target `[0x30000434] +0x8c`, value `0x0` (`clr.l` at 0xffe5dbf0).
  Halfword store: target `[0x30000434] +0x92`, value `0x953` (`move.w` at 0xffe5dbfa).
  Call (`jsr` at 0xffe5dc0e): (unknown callee), target 0xffe6d594, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe5dc1e): taken to 0xffe5dc64 when `Z == 0`, falls through to 0xffe5dc20 when `not (Z == 0)`.
  Byte store: target `[0x30000434] +0x16`, value `0x8c` (`move.b` at 0xffe5dc2a).
  Halfword store: target `[0x30000434] +0xa0`, value `0x6000` (`move.w` at 0xffe5dc34).
  Word store: target `[0x30000434] +0xa4`, value `0x0` (`clr.l` at 0xffe5dc3a).
  Halfword store: target `[0x30000434] +0xaa`, value `0x953` (`move.w` at 0xffe5dc44).
  Call (`jsr` at 0xffe5dc5a): (unknown callee), target 0xffe6d594, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the return (`rts` at 0xffe5dc6a), d0 is `unknown`.

- `0xffe8ff7c`, `root/6.5.26/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.26:l1#2`, version `1.32.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe8ff7c to 0xffe90038 (exclusive), 188 bytes.
  Carrier: 3846136 bytes, SHA256 `1fdcc3409f4e0633335d9b51d506818ce971dacbe6ababe1d4fbcbd52855e3dc`.
  Package: carrier offset 0x1debcc, 1885228 bytes, SHA256 `84466adaef25b1382edb5db0569655ab340dea79b2adde07cf9499f400ca4837`.
  Payload: carrier offset 0x1debf8, 1885184 bytes, SHA256 `9793fe0aac389db6d98cbd2fe0c6d96aa70dcc526d0cfe5810d554316931d981`.
  Read window: SHA256 `0e967ce834dc4dc080ac0c65ba979e0e843e09445fefeda326a35caf2131654b`.
  Call (`jsr` at 0xffe8ff82): (unknown callee), target 0xffe908e8, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe8ff8a): (unknown callee), target 0xffe8f894, stack arg0=`ret(0xffe908e8)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe8ff90).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe8ff96).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe8ff9a).
  Grouped accesses: 2 4-byte load records with target `abs 0x30003260`, including `tst.l` at 0xffe8ffa4, `move.l` at 0xffe8ffe4.
  Branch (`ble.b` at 0xffe8ffa8): taken to 0xffe8ffe4 when `[0x30003260] <= 0x0 (signed, 32-bit)`, falls through to 0xffe8ffaa when `[0x30003260] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000a7c`, including `movea.l` at 0xffe8ffb0, `movea.l` at 0xffe8ffba, `movea.l` at 0xffe8ffc0.
  Halfword store: target `[0x30000a7c] +0x88`, value `0x4000` (`move.w` at 0xffe8ffb6).
  Word store: target `[0x30000a7c] +0x8c`, value `0x0` (`clr.l` at 0xffe8ffbc).
  Halfword store: target `[0x30000a7c] +0x92`, value `0x953` (`move.w` at 0xffe8ffc6).
  Call (`jsr` at 0xffe8ffda): (unknown callee), target 0xffead758, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe8ffea): taken to 0xffe90030 when `Z == 0`, falls through to 0xffe8ffec when `not (Z == 0)`.
  Byte store: target `[0x30000a7c] +0x16`, value `0x8c` (`move.b` at 0xffe8fff6).
  Halfword store: target `[0x30000a7c] +0xa0`, value `0x6000` (`move.w` at 0xffe90000).
  Word store: target `[0x30000a7c] +0xa4`, value `0x0` (`clr.l` at 0xffe90006).
  Halfword store: target `[0x30000a7c] +0xaa`, value `0x953` (`move.w` at 0xffe90010).
  Call (`jsr` at 0xffe90026): (unknown callee), target 0xffead758, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  The exit (`rts` at 0xffe90036) reports d0 = `unknown`.

- `0xffe572e8`, `root/6.5.25/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.25:l1#0`, version `1.30.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe572e8 to 0xffe573a4 (exclusive), 188 bytes.
  Carrier: 3817584 bytes, SHA256 `239378307da84254c56ed7499818aa372fe0aabd54e668da931da34f13ac620d`.
  Package: carrier offset 0x0, 979900 bytes, SHA256 `bc2358995619da07a0363c20e1ed39794146b9ad478b1b3e5068aca530c3dfc9`.
  Payload: carrier offset 0x48, 979828 bytes, SHA256 `a13d2aa10e7903bf53ab9b8395c23d4ebcf47eb3c09bb126cabfdf3f423e8b98`.
  Read window: SHA256 `700dab80456626794be8d1476b26c903156629821b5cbef9edb7a5ea3a594d77`.
  Call (`jsr` at 0xffe572ee): (unknown callee), target 0xffe579bc, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe572f6): (unknown callee), target 0xffe56c00, stack arg0=`ret(0xffe579bc)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe572fc).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe57302).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe57306).
  Grouped accesses: 2 4-byte load records with target `abs 0x30001e38`, including `tst.l` at 0xffe57310, `move.l` at 0xffe57350.
  Branch (`ble.b` at 0xffe57314): taken to 0xffe57350 when `[0x30001e38] <= 0x0 (signed, 32-bit)`, falls through to 0xffe57316 when `[0x30001e38] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000320`, including `movea.l` at 0xffe5731c, `movea.l` at 0xffe57326, `movea.l` at 0xffe5732c.
  Halfword store: target `[0x30000320] +0x88`, value `0x4000` (`move.w` at 0xffe57322).
  Word store: target `[0x30000320] +0x8c`, value `0x0` (`clr.l` at 0xffe57328).
  Halfword store: target `[0x30000320] +0x92`, value `0x953` (`move.w` at 0xffe57332).
  Call (`jsr` at 0xffe57346): (unknown callee), target 0xffe6d238, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe57356): taken to 0xffe5739c when `Z == 0`, falls through to 0xffe57358 when `not (Z == 0)`.
  Byte store: target `[0x30000320] +0x16`, value `0x8c` (`move.b` at 0xffe57362).
  Halfword store: target `[0x30000320] +0xa0`, value `0x6000` (`move.w` at 0xffe5736c).
  Word store: target `[0x30000320] +0xa4`, value `0x0` (`clr.l` at 0xffe57372).
  Halfword store: target `[0x30000320] +0xaa`, value `0x953` (`move.w` at 0xffe5737c).
  Call (`jsr` at 0xffe57392): (unknown callee), target 0xffe6d238, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  `rts` at 0xffe573a2 has recorded d0 = `unknown`.

- `0xffe570fc`, `root/6.5.25/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.25:l1#1`, version `1.30.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe570fc to 0xffe571b8 (exclusive), 188 bytes.
  Carrier: 3817584 bytes, SHA256 `239378307da84254c56ed7499818aa372fe0aabd54e668da931da34f13ac620d`.
  Package: carrier offset 0xef3bc, 979984 bytes, SHA256 `0c31235ae9a306523cbb55de14016bc052fa2f4ae889defc8b6de3099e19c112`.
  Payload: carrier offset 0xef3e8, 979940 bytes, SHA256 `d592fe64ddd586ff8ffef2cea25d5e55e38c41114174b22d106a211524055ef0`.
  Read window: SHA256 `54d273dc72fe1f5be89c82171429dd5f0a5cc96a26542c635849d31e0f0a6403`.
  Call (`jsr` at 0xffe57102): (unknown callee), target 0xffe577d0, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe5710a): (unknown callee), target 0xffe56a14, stack arg0=`ret(0xffe577d0)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe57110).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe57116).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe5711a).
  Grouped accesses: 2 4-byte load records with target `abs 0x30001f30`, including `tst.l` at 0xffe57124, `move.l` at 0xffe57164.
  Branch (`ble.b` at 0xffe57128): taken to 0xffe57164 when `[0x30001f30] <= 0x0 (signed, 32-bit)`, falls through to 0xffe5712a when `[0x30001f30] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000354`, including `movea.l` at 0xffe57130, `movea.l` at 0xffe5713a, `movea.l` at 0xffe57140.
  Halfword store: target `[0x30000354] +0x88`, value `0x4000` (`move.w` at 0xffe57136).
  Word store: target `[0x30000354] +0x8c`, value `0x0` (`clr.l` at 0xffe5713c).
  Halfword store: target `[0x30000354] +0x92`, value `0x953` (`move.w` at 0xffe57146).
  Call (`jsr` at 0xffe5715a): (unknown callee), target 0xffe6d250, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe5716a): taken to 0xffe571b0 when `Z == 0`, falls through to 0xffe5716c when `not (Z == 0)`.
  Byte store: target `[0x30000354] +0x16`, value `0x8c` (`move.b` at 0xffe57176).
  Halfword store: target `[0x30000354] +0xa0`, value `0x6000` (`move.w` at 0xffe57180).
  Word store: target `[0x30000354] +0xa4`, value `0x0` (`clr.l` at 0xffe57186).
  Halfword store: target `[0x30000354] +0xaa`, value `0x953` (`move.w` at 0xffe57190).
  Call (`jsr` at 0xffe571a6): (unknown callee), target 0xffe6d250, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Recorded d0 at the return is `unknown` (`rts` at 0xffe571b6).

- `0xffe87028`, `root/6.5.25/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.25:l1#2`, version `1.30.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe87028 to 0xffe870e4 (exclusive), 188 bytes.
  Carrier: 3817584 bytes, SHA256 `239378307da84254c56ed7499818aa372fe0aabd54e668da931da34f13ac620d`.
  Package: carrier offset 0x1de7cc, 1857700 bytes, SHA256 `718b73d44aa415958aa1899cc03977a0fc5c1f659184012a4c2c33771cd500e3`.
  Payload: carrier offset 0x1de7f8, 1857656 bytes, SHA256 `01ddeae2fbe3704ee85ad90d827a6325a0ada649e91d7dfc593a1a14f80e07b5`.
  Read window: SHA256 `ecda2355f217880cc7eeecd3fbeb76a195df7bb0d8ab5af926ca2f6bde708b9d`.
  Call (`jsr` at 0xffe8702e): (unknown callee), target 0xffe876fc, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe87036): (unknown callee), target 0xffe86940, stack arg0=`ret(0xffe876fc)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe8703c).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe87042).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe87046).
  Grouped accesses: 2 4-byte load records with target `abs 0x300025b0`, including `tst.l` at 0xffe87050, `move.l` at 0xffe87090.
  Branch (`ble.b` at 0xffe87054): taken to 0xffe87090 when `[0x300025b0] <= 0x0 (signed, 32-bit)`, falls through to 0xffe87056 when `[0x300025b0] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x300004b0`, including `movea.l` at 0xffe8705c, `movea.l` at 0xffe87066, `movea.l` at 0xffe8706c.
  Halfword store: target `[0x300004b0] +0x88`, value `0x4000` (`move.w` at 0xffe87062).
  Word store: target `[0x300004b0] +0x8c`, value `0x0` (`clr.l` at 0xffe87068).
  Halfword store: target `[0x300004b0] +0x92`, value `0x953` (`move.w` at 0xffe87072).
  Call (`jsr` at 0xffe87086): (unknown callee), target 0xffeaaafc, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe87096): taken to 0xffe870dc when `Z == 0`, falls through to 0xffe87098 when `not (Z == 0)`.
  Byte store: target `[0x300004b0] +0x16`, value `0x8c` (`move.b` at 0xffe870a2).
  Halfword store: target `[0x300004b0] +0xa0`, value `0x6000` (`move.w` at 0xffe870ac).
  Word store: target `[0x300004b0] +0xa4`, value `0x0` (`clr.l` at 0xffe870b2).
  Halfword store: target `[0x300004b0] +0xaa`, value `0x953` (`move.w` at 0xffe870bc).
  Call (`jsr` at 0xffe870d2): (unknown callee), target 0xffeaaafc, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the return (`rts` at 0xffe870e2), d0 is `unknown`.

- `0xffe5efbc`, `root/6.5.24/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.24:l1#0`, version `1.28.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe5efbc to 0xffe5f078 (exclusive), 188 bytes.
  Carrier: 3759416 bytes, SHA256 `12e1052718a311de3dfd6424820e680fbaedace11a249c463a1ff5bbca2236c1`.
  Package: carrier offset 0x0, 980132 bytes, SHA256 `743c2e9361fcdd2134ae3d68a4773694a6909e4016afbc25530506172432cbd5`.
  Payload: carrier offset 0x48, 980060 bytes, SHA256 `edb297ad427641845d4abf1ffb2c0e82e81e5b26f56cd2c1e3d888352a34fd50`.
  Read window: SHA256 `4ccaffc8b230bce6081747da4ddf8b2f8394fc2bba4f8f77c799831ccbdc1b03`.
  Call (`jsr` at 0xffe5efc2): (unknown callee), target 0xffe578d4, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe5efca): (unknown callee), target 0xffe5e8d4, stack arg0=`ret(0xffe578d4)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe5efd0).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe5efd6).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe5efda).
  Grouped accesses: 2 4-byte load records with target `abs 0x300021d8`, including `tst.l` at 0xffe5efe4, `move.l` at 0xffe5f024.
  Branch (`ble.b` at 0xffe5efe8): taken to 0xffe5f024 when `[0x300021d8] <= 0x0 (signed, 32-bit)`, falls through to 0xffe5efea when `[0x300021d8] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000808`, including `movea.l` at 0xffe5eff0, `movea.l` at 0xffe5effa, `movea.l` at 0xffe5f000.
  Halfword store: target `[0x30000808] +0x88`, value `0x4000` (`move.w` at 0xffe5eff6).
  Word store: target `[0x30000808] +0x8c`, value `0x0` (`clr.l` at 0xffe5effc).
  Halfword store: target `[0x30000808] +0x92`, value `0x953` (`move.w` at 0xffe5f006).
  Call (`jsr` at 0xffe5f01a): (unknown callee), target 0xffe6cd48, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe5f02a): taken to 0xffe5f070 when `Z == 0`, falls through to 0xffe5f02c when `not (Z == 0)`.
  Byte store: target `[0x30000808] +0x16`, value `0x8c` (`move.b` at 0xffe5f036).
  Halfword store: target `[0x30000808] +0xa0`, value `0x6000` (`move.w` at 0xffe5f040).
  Word store: target `[0x30000808] +0xa4`, value `0x0` (`clr.l` at 0xffe5f046).
  Halfword store: target `[0x30000808] +0xaa`, value `0x953` (`move.w` at 0xffe5f050).
  Call (`jsr` at 0xffe5f066): (unknown callee), target 0xffe6cd48, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  The exit (`rts` at 0xffe5f076) reports d0 = `unknown`.

- `0xffe5e8d4`, `root/6.5.24/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.24:l1#1`, version `1.28.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe5e8d4 to 0xffe5e990 (exclusive), 188 bytes.
  Carrier: 3759416 bytes, SHA256 `12e1052718a311de3dfd6424820e680fbaedace11a249c463a1ff5bbca2236c1`.
  Package: carrier offset 0xef4a4, 979096 bytes, SHA256 `1f3ca14b06175e39d38af6315bef38a6ab31ff2178a5cfd05943593bf37f64a7`.
  Payload: carrier offset 0xef4d0, 979052 bytes, SHA256 `72ad6ba3005bd3a464774c90dcfe60e333e18e06d10812724c72c2ddd62a0351`.
  Read window: SHA256 `f7944199cce5172fbf9c60bf4fe3d494973d373c495f9eb5850af63c700f421f`.
  Call (`jsr` at 0xffe5e8da): (unknown callee), target 0xffe571ec, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe5e8e2): (unknown callee), target 0xffe5e1ec, stack arg0=`ret(0xffe571ec)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe5e8e8).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe5e8ee).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe5e8f2).
  Grouped accesses: 2 4-byte load records with target `abs 0x30002348`, including `tst.l` at 0xffe5e8fc, `move.l` at 0xffe5e93c.
  Branch (`ble.b` at 0xffe5e900): taken to 0xffe5e93c when `[0x30002348] <= 0x0 (signed, 32-bit)`, falls through to 0xffe5e902 when `[0x30002348] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x300008b0`, including `movea.l` at 0xffe5e908, `movea.l` at 0xffe5e912, `movea.l` at 0xffe5e918.
  Halfword store: target `[0x300008b0] +0x88`, value `0x4000` (`move.w` at 0xffe5e90e).
  Word store: target `[0x300008b0] +0x8c`, value `0x0` (`clr.l` at 0xffe5e914).
  Halfword store: target `[0x300008b0] +0x92`, value `0x953` (`move.w` at 0xffe5e91e).
  Call (`jsr` at 0xffe5e932): (unknown callee), target 0xffe6c864, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe5e942): taken to 0xffe5e988 when `Z == 0`, falls through to 0xffe5e944 when `not (Z == 0)`.
  Byte store: target `[0x300008b0] +0x16`, value `0x8c` (`move.b` at 0xffe5e94e).
  Halfword store: target `[0x300008b0] +0xa0`, value `0x6000` (`move.w` at 0xffe5e958).
  Word store: target `[0x300008b0] +0xa4`, value `0x0` (`clr.l` at 0xffe5e95e).
  Halfword store: target `[0x300008b0] +0xaa`, value `0x953` (`move.w` at 0xffe5e968).
  Call (`jsr` at 0xffe5e97e): (unknown callee), target 0xffe6c864, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  `rts` at 0xffe5e98e has recorded d0 = `unknown`.

- `0xffe82cec`, `root/6.5.24/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.24:l1#2`, version `1.28.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe82cec to 0xffe82da8 (exclusive), 188 bytes.
  Carrier: 3759416 bytes, SHA256 `12e1052718a311de3dfd6424820e680fbaedace11a249c463a1ff5bbca2236c1`.
  Package: carrier offset 0x1de53c, 1800188 bytes, SHA256 `391a349a2940f18047f404a69cbf7641e25c67c2d7739e3739ec54e473b9d6a7`.
  Payload: carrier offset 0x1de568, 1800144 bytes, SHA256 `1f088a070cf4398029a936d96860ad27d7e2bf2ed2ad35b871f07d84398be8c5`.
  Read window: SHA256 `ccb69e9a3ae6db57d7e90f85d6dec5578fa54e2b8d805dd512f5db0f52509193`.
  Call (`jsr` at 0xffe82cf2): (unknown callee), target 0xffe7b604, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe82cfa): (unknown callee), target 0xffe82604, stack arg0=`ret(0xffe7b604)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe82d00).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe82d06).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe82d0a).
  Grouped accesses: 2 4-byte load records with target `abs 0x30002b58`, including `tst.l` at 0xffe82d14, `move.l` at 0xffe82d54.
  Branch (`ble.b` at 0xffe82d18): taken to 0xffe82d54 when `[0x30002b58] <= 0x0 (signed, 32-bit)`, falls through to 0xffe82d1a when `[0x30002b58] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000b18`, including `movea.l` at 0xffe82d20, `movea.l` at 0xffe82d2a, `movea.l` at 0xffe82d30.
  Halfword store: target `[0x30000b18] +0x88`, value `0x4000` (`move.w` at 0xffe82d26).
  Word store: target `[0x30000b18] +0x8c`, value `0x0` (`clr.l` at 0xffe82d2c).
  Halfword store: target `[0x30000b18] +0x92`, value `0x953` (`move.w` at 0xffe82d36).
  Call (`jsr` at 0xffe82d4a): (unknown callee), target 0xffe9d5e8, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe82d5a): taken to 0xffe82da0 when `Z == 0`, falls through to 0xffe82d5c when `not (Z == 0)`.
  Byte store: target `[0x30000b18] +0x16`, value `0x8c` (`move.b` at 0xffe82d66).
  Halfword store: target `[0x30000b18] +0xa0`, value `0x6000` (`move.w` at 0xffe82d70).
  Word store: target `[0x30000b18] +0xa4`, value `0x0` (`clr.l` at 0xffe82d76).
  Halfword store: target `[0x30000b18] +0xaa`, value `0x953` (`move.w` at 0xffe82d80).
  Call (`jsr` at 0xffe82d96): (unknown callee), target 0xffe9d5e8, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Recorded d0 at the return is `unknown` (`rts` at 0xffe82da6).

- `0xffe5f62c`, `root/6.5.23/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.23:l1#0`, version `1.26.5`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe5f62c to 0xffe5f6e8 (exclusive), 188 bytes.
  Carrier: 3763312 bytes, SHA256 `15fe4284aa152ba30d1a8d9c67f63ee2acf1d19b0653068befc89146c12187f1`.
  Package: carrier offset 0x0, 982364 bytes, SHA256 `01f4157793ada930d8f671157cdd27fad86188a92d4a4c32b5c2c8133fcd243b`.
  Payload: carrier offset 0x48, 982292 bytes, SHA256 `85d56d8030146394e6339ca8e34f99d091e9588ecd9a752759a35358c554e5e8`.
  Read window: SHA256 `065e2202d26203bf165309beff05c36b0ed2e99c5dd4bd5b97737fa07eb97187`.
  Call (`jsr` at 0xffe5f632): (unknown callee), target 0xffe57f64, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe5f63a): (unknown callee), target 0xffe5ef44, stack arg0=`ret(0xffe57f64)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe5f640).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe5f646).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe5f64a).
  Grouped accesses: 2 4-byte load records with target `abs 0x300021d8`, including `tst.l` at 0xffe5f654, `move.l` at 0xffe5f694.
  Branch (`ble.b` at 0xffe5f658): taken to 0xffe5f694 when `[0x300021d8] <= 0x0 (signed, 32-bit)`, falls through to 0xffe5f65a when `[0x300021d8] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000808`, including `movea.l` at 0xffe5f660, `movea.l` at 0xffe5f66a, `movea.l` at 0xffe5f670.
  Halfword store: target `[0x30000808] +0x88`, value `0x4000` (`move.w` at 0xffe5f666).
  Word store: target `[0x30000808] +0x8c`, value `0x0` (`clr.l` at 0xffe5f66c).
  Halfword store: target `[0x30000808] +0x92`, value `0x953` (`move.w` at 0xffe5f676).
  Call (`jsr` at 0xffe5f68a): (unknown callee), target 0xffe6d428, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe5f69a): taken to 0xffe5f6e0 when `Z == 0`, falls through to 0xffe5f69c when `not (Z == 0)`.
  Byte store: target `[0x30000808] +0x16`, value `0x8c` (`move.b` at 0xffe5f6a6).
  Halfword store: target `[0x30000808] +0xa0`, value `0x6000` (`move.w` at 0xffe5f6b0).
  Word store: target `[0x30000808] +0xa4`, value `0x0` (`clr.l` at 0xffe5f6b6).
  Halfword store: target `[0x30000808] +0xaa`, value `0x953` (`move.w` at 0xffe5f6c0).
  Call (`jsr` at 0xffe5f6d6): (unknown callee), target 0xffe6d428, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the return (`rts` at 0xffe5f6e6), d0 is `unknown`.

- `0xffe5ef40`, `root/6.5.23/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.23:l1#1`, version `1.26.5`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe5ef40 to 0xffe5effc (exclusive), 188 bytes.
  Carrier: 3763312 bytes, SHA256 `15fe4284aa152ba30d1a8d9c67f63ee2acf1d19b0653068befc89146c12187f1`.
  Package: carrier offset 0xefd5c, 981328 bytes, SHA256 `aea0366c69ecffb773f22102af9e5d2696ec1eff91925377f388345d468380a0`.
  Payload: carrier offset 0xefd88, 981284 bytes, SHA256 `1c9aeb0a2af5bb40c97a727427e01cc159613434ed1a7a670b2dd81566888cf4`.
  Read window: SHA256 `643339a372db78a990cc4557f95a214181b80e18ebd980e68be296f6ffd5395e`.
  Call (`jsr` at 0xffe5ef46): (unknown callee), target 0xffe57878, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe5ef4e): (unknown callee), target 0xffe5e858, stack arg0=`ret(0xffe57878)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe5ef54).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe5ef5a).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe5ef5e).
  Grouped accesses: 2 4-byte load records with target `abs 0x30002348`, including `tst.l` at 0xffe5ef68, `move.l` at 0xffe5efa8.
  Branch (`ble.b` at 0xffe5ef6c): taken to 0xffe5efa8 when `[0x30002348] <= 0x0 (signed, 32-bit)`, falls through to 0xffe5ef6e when `[0x30002348] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x300008b0`, including `movea.l` at 0xffe5ef74, `movea.l` at 0xffe5ef7e, `movea.l` at 0xffe5ef84.
  Halfword store: target `[0x300008b0] +0x88`, value `0x4000` (`move.w` at 0xffe5ef7a).
  Word store: target `[0x300008b0] +0x8c`, value `0x0` (`clr.l` at 0xffe5ef80).
  Halfword store: target `[0x300008b0] +0x92`, value `0x953` (`move.w` at 0xffe5ef8a).
  Call (`jsr` at 0xffe5ef9e): (unknown callee), target 0xffe6cf40, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe5efae): taken to 0xffe5eff4 when `Z == 0`, falls through to 0xffe5efb0 when `not (Z == 0)`.
  Byte store: target `[0x300008b0] +0x16`, value `0x8c` (`move.b` at 0xffe5efba).
  Halfword store: target `[0x300008b0] +0xa0`, value `0x6000` (`move.w` at 0xffe5efc4).
  Word store: target `[0x300008b0] +0xa4`, value `0x0` (`clr.l` at 0xffe5efca).
  Halfword store: target `[0x300008b0] +0xaa`, value `0x953` (`move.w` at 0xffe5efd4).
  Call (`jsr` at 0xffe5efea): (unknown callee), target 0xffe6cf40, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  The exit (`rts` at 0xffe5effa) reports d0 = `unknown`.

- `0xffe82778`, `root/6.5.23/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.23:l1#2`, version `1.26.5`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe82778 to 0xffe82834 (exclusive), 188 bytes.
  Carrier: 3763312 bytes, SHA256 `15fe4284aa152ba30d1a8d9c67f63ee2acf1d19b0653068befc89146c12187f1`.
  Package: carrier offset 0x1df6ac, 1799620 bytes, SHA256 `f8faa30fbd2a62abf34dc81f249e3eced855814e03126cd61f23787233c72db5`.
  Payload: carrier offset 0x1df6d8, 1799576 bytes, SHA256 `92705d72b6589449f4852e8d45d40a995d5c54909f9de62af36ea3b5aeb28414`.
  Read window: SHA256 `1fa3782d2172767fa3395357b37f96b81f9279101a3dc9286df7d18077fa3d85`.
  Call (`jsr` at 0xffe8277e): (unknown callee), target 0xffe7b0b0, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe82786): (unknown callee), target 0xffe82090, stack arg0=`ret(0xffe7b0b0)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe8278c).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe82792).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe82796).
  Grouped accesses: 2 4-byte load records with target `abs 0x30002b40`, including `tst.l` at 0xffe827a0, `move.l` at 0xffe827e0.
  Branch (`ble.b` at 0xffe827a4): taken to 0xffe827e0 when `[0x30002b40] <= 0x0 (signed, 32-bit)`, falls through to 0xffe827a6 when `[0x30002b40] > 0x0 (signed, 32-bit)`.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000afc`, including `movea.l` at 0xffe827ac, `movea.l` at 0xffe827b6, `movea.l` at 0xffe827bc.
  Halfword store: target `[0x30000afc] +0x88`, value `0x4000` (`move.w` at 0xffe827b2).
  Word store: target `[0x30000afc] +0x8c`, value `0x0` (`clr.l` at 0xffe827b8).
  Halfword store: target `[0x30000afc] +0x92`, value `0x953` (`move.w` at 0xffe827c2).
  Call (`jsr` at 0xffe827d6): (unknown callee), target 0xffe9d214, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe827e6): taken to 0xffe8282c when `Z == 0`, falls through to 0xffe827e8 when `not (Z == 0)`.
  Byte store: target `[0x30000afc] +0x16`, value `0x8c` (`move.b` at 0xffe827f2).
  Halfword store: target `[0x30000afc] +0xa0`, value `0x6000` (`move.w` at 0xffe827fc).
  Word store: target `[0x30000afc] +0xa4`, value `0x0` (`clr.l` at 0xffe82802).
  Halfword store: target `[0x30000afc] +0xaa`, value `0x953` (`move.w` at 0xffe8280c).
  Call (`jsr` at 0xffe82822): (unknown callee), target 0xffe9d214, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  `rts` at 0xffe82832 has recorded d0 = `unknown`.
