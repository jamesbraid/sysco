# ColdFire L1 pointer-relative write observations

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

The [complete facts](disasm/coldfire-variants/pointer-write-later-images-facts.txt) retain the full analyzed windows.
Auxiliary truncated decoder observations were rejected.
Complete caller and successor contexts were checked separately.
Range ends are exclusive.

- `0xffe73508`, `root/patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`.
  Package `patches/SG0007149:l1#5`, version `1.48.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe73508 to 0xffe735c4 (exclusive), 188 bytes.
  Carrier: 16027376 bytes, SHA256 `536bfa4c6d0240b7afbc25bef94e160bdf1a675016c35e68ec52e1bfb1b451f9`.
  Package: carrier offset 0x8e1250, 1768364 bytes, SHA256 `c83d94ff29874a2a3276cdc052556e40e0c42e1c332e722711cdb4cfadfbb579`.
  Payload: carrier offset 0x8e127c, 1768320 bytes, SHA256 `57d9c981a155eeb1c3ccaaae9d2dee701f553710f5052765f01ca645d279fe9f`.
  Read window: SHA256 `7683259a8c97d2724177158fdf40c3541b162ac5a9b2d5febf77c169a12619fe`.
  Call (`jsr` at 0xffe7350e): (unknown callee), target 0xffe735c4, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe73516): (unknown callee), target 0xffe7310c, stack arg0=`ret(0xffe735c4)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe73566): (unknown callee), target 0xffe929d4, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe735b2): (unknown callee), target 0xffe929d4, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe7351c).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe73522).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe73526).
  Grouped accesses: 2 4-byte load records with target `abs 0x30002ca0`, including `tst.l` at 0xffe73530, `move.l` at 0xffe73570.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000a84`, including `movea.l` at 0xffe7353c, `movea.l` at 0xffe73546, `movea.l` at 0xffe7354c.
  Halfword store: target `[0x30000a84] +0x88`, value `0x4000` (`move.w` at 0xffe73542).
  Word store: target `[0x30000a84] +0x8c`, value `0x0` (`clr.l` at 0xffe73548).
  Halfword store: target `[0x30000a84] +0x92`, value `0x953` (`move.w` at 0xffe73552).
  Byte store: target `[0x30000a84] +0x16`, value `0x8c` (`move.b` at 0xffe73582).
  Halfword store: target `[0x30000a84] +0xa0`, value `0x6000` (`move.w` at 0xffe7358c).
  Word store: target `[0x30000a84] +0xa4`, value `0x0` (`clr.l` at 0xffe73592).
  Halfword store: target `[0x30000a84] +0xaa`, value `0x953` (`move.w` at 0xffe7359c).
  Branch (`ble.b` at 0xffe73534): taken to 0xffe73570 when `[0x30002ca0] <= 0x0 (signed, 32-bit)`, falls through to 0xffe73536 when `[0x30002ca0] > 0x0 (signed, 32-bit)`.
  Branch (`bne.b` at 0xffe73576): taken to 0xffe735bc when `Z == 0`, falls through to 0xffe73578 when `not (Z == 0)`.
  At the return (`rts` at 0xffe735c2), d0 is `unknown`.

- `0xffe5b714`, `root/patches/SG0005762/usr/cpu/firmware/sysco/l1.bin`.
  Package `patches/SG0005762:l1#0`, version `1.32.15`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe5b714 to 0xffe5b7d0 (exclusive), 188 bytes.
  Carrier: 7406448 bytes, SHA256 `2399ea17dfeb2af20fae4286f0cfeeffae6d3d9f637fcc02e880e5689f662656`.
  Package: carrier offset 0x0, 981788 bytes, SHA256 `cfe50f185db4cbb8da444dd49efedbd662c463ae161559ec6d24a8acbff2c7cb`.
  Payload: carrier offset 0x50, 981708 bytes, SHA256 `330ef3074bdfdbbabcffd95fb2c26ea942118cd33e4029e0277d136d63e11e0f`.
  Read window: SHA256 `03f9a7d82343b2b0f4216199db54227ea881052fe120a187e4d389877615ec67`.
  Call (`jsr` at 0xffe5b71a): (unknown callee), target 0xffe5e2d4, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe5b722): (unknown callee), target 0xffe5b02c, stack arg0=`ret(0xffe5e2d4)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe5b772): (unknown callee), target 0xffe6d590, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe5b7be): (unknown callee), target 0xffe6d590, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe5b728).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe5b72e).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe5b732).
  Grouped accesses: 2 4-byte load records with target `abs 0x300025bc`, including `tst.l` at 0xffe5b73c, `move.l` at 0xffe5b77c.
  Grouped accesses: 7 4-byte load records with target `abs 0x300003bc`, including `movea.l` at 0xffe5b748, `movea.l` at 0xffe5b752, `movea.l` at 0xffe5b758.
  Halfword store: target `[0x300003bc] +0x88`, value `0x4000` (`move.w` at 0xffe5b74e).
  Word store: target `[0x300003bc] +0x8c`, value `0x0` (`clr.l` at 0xffe5b754).
  Halfword store: target `[0x300003bc] +0x92`, value `0x953` (`move.w` at 0xffe5b75e).
  Byte store: target `[0x300003bc] +0x16`, value `0x8c` (`move.b` at 0xffe5b78e).
  Halfword store: target `[0x300003bc] +0xa0`, value `0x6000` (`move.w` at 0xffe5b798).
  Word store: target `[0x300003bc] +0xa4`, value `0x0` (`clr.l` at 0xffe5b79e).
  Halfword store: target `[0x300003bc] +0xaa`, value `0x953` (`move.w` at 0xffe5b7a8).
  Branch (`ble.b` at 0xffe5b740): taken to 0xffe5b77c when `[0x300025bc] <= 0x0 (signed, 32-bit)`, falls through to 0xffe5b742 when `[0x300025bc] > 0x0 (signed, 32-bit)`.
  Branch (`bne.b` at 0xffe5b782): taken to 0xffe5b7c8 when `Z == 0`, falls through to 0xffe5b784 when `not (Z == 0)`.
  The exit at (`rts` at 0xffe5b7ce) records d0 = `unknown`.

- `0xffe5b3b8`, `root/patches/SG0005762/usr/cpu/firmware/sysco/l1.bin`.
  Package `patches/SG0005762:l1#1`, version `1.32.15`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe5b3b8 to 0xffe5b474 (exclusive), 188 bytes.
  Carrier: 7406448 bytes, SHA256 `2399ea17dfeb2af20fae4286f0cfeeffae6d3d9f637fcc02e880e5689f662656`.
  Package: carrier offset 0xefb1c, 981728 bytes, SHA256 `fdc8eb9f405f0cc5a8eb72dd9e3e579e28e1f72b574c0508639269d722830922`.
  Payload: carrier offset 0xefb48, 981684 bytes, SHA256 `1aa8d9f5f1ebd4a38b084a4f783f6b3c402fa4ec14b3e08051b8bf913c36dcbc`.
  Read window: SHA256 `56b47ea586444293f0409f3d715748475a9d8ca8339a56f8302dfb7b36e0dad5`.
  Call (`jsr` at 0xffe5b3be): (unknown callee), target 0xffe5df78, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe5b3c6): (unknown callee), target 0xffe5acd0, stack arg0=`ret(0xffe5df78)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe5b416): (unknown callee), target 0xffe6d42c, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe5b462): (unknown callee), target 0xffe6d42c, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe5b3cc).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe5b3d2).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe5b3d6).
  Grouped accesses: 2 4-byte load records with target `abs 0x300026b8`, including `tst.l` at 0xffe5b3e0, `move.l` at 0xffe5b420.
  Grouped accesses: 7 4-byte load records with target `abs 0x300003e8`, including `movea.l` at 0xffe5b3ec, `movea.l` at 0xffe5b3f6, `movea.l` at 0xffe5b3fc.
  Halfword store: target `[0x300003e8] +0x88`, value `0x4000` (`move.w` at 0xffe5b3f2).
  Word store: target `[0x300003e8] +0x8c`, value `0x0` (`clr.l` at 0xffe5b3f8).
  Halfword store: target `[0x300003e8] +0x92`, value `0x953` (`move.w` at 0xffe5b402).
  Byte store: target `[0x300003e8] +0x16`, value `0x8c` (`move.b` at 0xffe5b432).
  Halfword store: target `[0x300003e8] +0xa0`, value `0x6000` (`move.w` at 0xffe5b43c).
  Word store: target `[0x300003e8] +0xa4`, value `0x0` (`clr.l` at 0xffe5b442).
  Halfword store: target `[0x300003e8] +0xaa`, value `0x953` (`move.w` at 0xffe5b44c).
  Branch (`ble.b` at 0xffe5b3e4): taken to 0xffe5b420 when `[0x300026b8] <= 0x0 (signed, 32-bit)`, falls through to 0xffe5b3e6 when `[0x300026b8] > 0x0 (signed, 32-bit)`.
  Branch (`bne.b` at 0xffe5b426): taken to 0xffe5b46c when `Z == 0`, falls through to 0xffe5b428 when `not (Z == 0)`.
  d0 has the literal analysis result `unknown` on return (`rts` at 0xffe5b472).

- `0xffe8e138`, `root/patches/SG0005762/usr/cpu/firmware/sysco/l1.bin`.
  Package `patches/SG0005762:l1#2`, version `1.32.15`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe8e138 to 0xffe8e1f4 (exclusive), 188 bytes.
  Carrier: 7406448 bytes, SHA256 `2399ea17dfeb2af20fae4286f0cfeeffae6d3d9f637fcc02e880e5689f662656`.
  Package: carrier offset 0x1df5fc, 1887360 bytes, SHA256 `7872ba43629634598f82b69c94bc0954b8cd3680c2e5578e2f3c8471c1cd7aa4`.
  Payload: carrier offset 0x1df628, 1887316 bytes, SHA256 `ad6c39f0232f959b55c18e57ec29425f071354c7a7d8bc21abc0ceb136c8c823`.
  Read window: SHA256 `35926513078aa380d5c7068c95d33f116c726da7fae05859e497f76b8b01a179`.
  Call (`jsr` at 0xffe8e13e): (unknown callee), target 0xffe90cf8, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe8e146): (unknown callee), target 0xffe8da50, stack arg0=`ret(0xffe90cf8)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8e196): (unknown callee), target 0xffeadfa4, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8e1e2): (unknown callee), target 0xffeadfa4, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe8e14c).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe8e152).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe8e156).
  Grouped accesses: 2 4-byte load records with target `abs 0x30003298`, including `tst.l` at 0xffe8e160, `move.l` at 0xffe8e1a0.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000a40`, including `movea.l` at 0xffe8e16c, `movea.l` at 0xffe8e176, `movea.l` at 0xffe8e17c.
  Halfword store: target `[0x30000a40] +0x88`, value `0x4000` (`move.w` at 0xffe8e172).
  Word store: target `[0x30000a40] +0x8c`, value `0x0` (`clr.l` at 0xffe8e178).
  Halfword store: target `[0x30000a40] +0x92`, value `0x953` (`move.w` at 0xffe8e182).
  Byte store: target `[0x30000a40] +0x16`, value `0x8c` (`move.b` at 0xffe8e1b2).
  Halfword store: target `[0x30000a40] +0xa0`, value `0x6000` (`move.w` at 0xffe8e1bc).
  Word store: target `[0x30000a40] +0xa4`, value `0x0` (`clr.l` at 0xffe8e1c2).
  Halfword store: target `[0x30000a40] +0xaa`, value `0x953` (`move.w` at 0xffe8e1cc).
  Branch (`ble.b` at 0xffe8e164): taken to 0xffe8e1a0 when `[0x30003298] <= 0x0 (signed, 32-bit)`, falls through to 0xffe8e166 when `[0x30003298] > 0x0 (signed, 32-bit)`.
  Branch (`bne.b` at 0xffe8e1a6): taken to 0xffe8e1ec when `Z == 0`, falls through to 0xffe8e1a8 when `not (Z == 0)`.
  Return expression: d0 = `unknown` (`rts` at 0xffe8e1f2).

- `0xffe53430`, `root/cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`.
  Package `cd-ist-3.24:l1#0`, version `1.62.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe53430 to 0xffe534ec (exclusive), 188 bytes.
  Carrier: 13258744 bytes, SHA256 `b4b5634949a93f05ad6eb24462e6cd7f10104dd42ff5c2ae8616dd5d87697790`.
  Package: carrier offset 0x0, 948616 bytes, SHA256 `e8f384de115f495ea87f86c045ab40c27a27d9b7c957bd5d39627c49881eaa59`.
  Payload: carrier offset 0x58, 948528 bytes, SHA256 `0f9d1cee2a1c4c7000fd322ccbb4f09f897551825463f7e123c35239905fbad8`.
  Read window: SHA256 `4a59efce33bd9fd199ad687eb63868f233073b09f00b7850ec635eb14b32af22`.
  Call (`jsr` at 0xffe53436): (unknown callee), target 0xffe53a4c, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe5343e): (unknown callee), target 0xffe52e78, stack arg0=`ret(0xffe53a4c)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe5348e): (unknown callee), target 0xffe6580c, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe534da): (unknown callee), target 0xffe6580c, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe53444).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe5344a).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe5344e).
  Grouped accesses: 2 4-byte load records with target `abs 0x30002154`, including `tst.l` at 0xffe53458, `move.l` at 0xffe53498.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000380`, including `movea.l` at 0xffe53464, `movea.l` at 0xffe5346e, `movea.l` at 0xffe53474.
  Halfword store: target `[0x30000380] +0x88`, value `0x4000` (`move.w` at 0xffe5346a).
  Word store: target `[0x30000380] +0x8c`, value `0x0` (`clr.l` at 0xffe53470).
  Halfword store: target `[0x30000380] +0x92`, value `0x953` (`move.w` at 0xffe5347a).
  Byte store: target `[0x30000380] +0x16`, value `0x8c` (`move.b` at 0xffe534aa).
  Halfword store: target `[0x30000380] +0xa0`, value `0x6000` (`move.w` at 0xffe534b4).
  Word store: target `[0x30000380] +0xa4`, value `0x0` (`clr.l` at 0xffe534ba).
  Halfword store: target `[0x30000380] +0xaa`, value `0x953` (`move.w` at 0xffe534c4).
  Branch (`ble.b` at 0xffe5345c): taken to 0xffe53498 when `[0x30002154] <= 0x0 (signed, 32-bit)`, falls through to 0xffe5345e when `[0x30002154] > 0x0 (signed, 32-bit)`.
  Branch (`bne.b` at 0xffe5349e): taken to 0xffe534e4 when `Z == 0`, falls through to 0xffe534a0 when `not (Z == 0)`.
  Returning through (`rts` at 0xffe534ea) leaves d0 `unknown` in this analysis.

- `0xffe7104c`, `root/cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`.
  Package `cd-ist-3.24:l1#1`, version `1.62.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe7104c to 0xffe71108 (exclusive), 188 bytes.
  Carrier: 13258744 bytes, SHA256 `b4b5634949a93f05ad6eb24462e6cd7f10104dd42ff5c2ae8616dd5d87697790`.
  Package: carrier offset 0xe7988, 1772496 bytes, SHA256 `45223c158690b9ea637f65a9710f4af32dd05bc0bd83dd3fc54216042ee0fd2d`.
  Payload: carrier offset 0xe79b4, 1772452 bytes, SHA256 `25561f492def55deebc646f821a1cd5d9b94277e69988c393dfa3df7f8622d7b`.
  Read window: SHA256 `6a7ecd355774c968b537f11742e05fc25eba03112dcc6fc7c4e682845d59d015`.
  Call (`jsr` at 0xffe71052): (unknown callee), target 0xffe7137c, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe7105a): (unknown callee), target 0xffe70c50, stack arg0=`ret(0xffe7137c)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe710aa): (unknown callee), target 0xffe94f30, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe710f6): (unknown callee), target 0xffe94f30, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe71060).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe71066).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe7106a).
  Grouped accesses: 2 4-byte load records with target `abs 0x30002e6c`, including `tst.l` at 0xffe71074, `move.l` at 0xffe710b4.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000b50`, including `movea.l` at 0xffe71080, `movea.l` at 0xffe7108a, `movea.l` at 0xffe71090.
  Halfword store: target `[0x30000b50] +0x88`, value `0x4000` (`move.w` at 0xffe71086).
  Word store: target `[0x30000b50] +0x8c`, value `0x0` (`clr.l` at 0xffe7108c).
  Halfword store: target `[0x30000b50] +0x92`, value `0x953` (`move.w` at 0xffe71096).
  Byte store: target `[0x30000b50] +0x16`, value `0x8c` (`move.b` at 0xffe710c6).
  Halfword store: target `[0x30000b50] +0xa0`, value `0x6000` (`move.w` at 0xffe710d0).
  Word store: target `[0x30000b50] +0xa4`, value `0x0` (`clr.l` at 0xffe710d6).
  Halfword store: target `[0x30000b50] +0xaa`, value `0x953` (`move.w` at 0xffe710e0).
  Branch (`ble.b` at 0xffe71078): taken to 0xffe710b4 when `[0x30002e6c] <= 0x0 (signed, 32-bit)`, falls through to 0xffe7107a when `[0x30002e6c] > 0x0 (signed, 32-bit)`.
  Branch (`bne.b` at 0xffe710ba): taken to 0xffe71100 when `Z == 0`, falls through to 0xffe710bc when `not (Z == 0)`.
  At the return (`rts` at 0xffe71106), d0 is `unknown`.

- `0xffe7b338`, `root/cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`.
  Package `cd-ist-3.24:l1#3`, version `1.62.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe7b338 to 0xffe7b3f4 (exclusive), 188 bytes.
  Carrier: 13258744 bytes, SHA256 `b4b5634949a93f05ad6eb24462e6cd7f10104dd42ff5c2ae8616dd5d87697790`.
  Package: carrier offset 0x63c654, 1771696 bytes, SHA256 `14b7122d6b6c072d9b4d2425f07ad68041807d535084e8c3501b512e017aa66e`.
  Payload: carrier offset 0x63c680, 1771652 bytes, SHA256 `fdb1a5444e0e11bd6d24bb080b3e5795a2dd340a2146cc1ced74419597d8f339`.
  Read window: SHA256 `a9042e1ec9a133f5802c1bf8229b9a78df7b6dac104717778c7a8497b3d33144`.
  Call (`jsr` at 0xffe7b33e): (unknown callee), target 0xffe7b3f4, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe7b346): (unknown callee), target 0xffe7af3c, stack arg0=`ret(0xffe7b3f4)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe7b396): (unknown callee), target 0xffe93564, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe7b3e2): (unknown callee), target 0xffe93564, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe7b34c).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe7b352).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe7b356).
  Grouped accesses: 2 4-byte load records with target `abs 0x30003438`, including `tst.l` at 0xffe7b360, `move.l` at 0xffe7b3a0.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000bd4`, including `movea.l` at 0xffe7b36c, `movea.l` at 0xffe7b376, `movea.l` at 0xffe7b37c.
  Halfword store: target `[0x30000bd4] +0x88`, value `0x4000` (`move.w` at 0xffe7b372).
  Word store: target `[0x30000bd4] +0x8c`, value `0x0` (`clr.l` at 0xffe7b378).
  Halfword store: target `[0x30000bd4] +0x92`, value `0x953` (`move.w` at 0xffe7b382).
  Byte store: target `[0x30000bd4] +0x16`, value `0x8c` (`move.b` at 0xffe7b3b2).
  Halfword store: target `[0x30000bd4] +0xa0`, value `0x6000` (`move.w` at 0xffe7b3bc).
  Word store: target `[0x30000bd4] +0xa4`, value `0x0` (`clr.l` at 0xffe7b3c2).
  Halfword store: target `[0x30000bd4] +0xaa`, value `0x953` (`move.w` at 0xffe7b3cc).
  Branch (`ble.b` at 0xffe7b364): taken to 0xffe7b3a0 when `[0x30003438] <= 0x0 (signed, 32-bit)`, falls through to 0xffe7b366 when `[0x30003438] > 0x0 (signed, 32-bit)`.
  Branch (`bne.b` at 0xffe7b3a6): taken to 0xffe7b3ec when `Z == 0`, falls through to 0xffe7b3a8 when `not (Z == 0)`.
  The exit at (`rts` at 0xffe7b3f2) records d0 = `unknown`.

- `0xffe58450`, `root/6.5.29/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.29:l1#0`, version `1.40.5`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe58450 to 0xffe5850c (exclusive), 188 bytes.
  Carrier: 9295164 bytes, SHA256 `80f4a616601d187bc3839a2758f2e7f2d0db962ad7fd8f2d96d41bcd15bdf88c`.
  Package: carrier offset 0x0, 982440 bytes, SHA256 `d1724d58c910a9bd1510b531e99c15d94ed1b429687131cd78214bc3876a0037`.
  Payload: carrier offset 0x58, 982352 bytes, SHA256 `d36178e16f6b36f7c1f5940320acbd041cd1ebcae18e5fec7b10734086e98a0e`.
  Read window: SHA256 `fa1f7d9285fa05ef9582f929d54324e99a35caa42424a51e211d6f6d05eaae8a`.
  Call (`jsr` at 0xffe58456): (unknown callee), target 0xffe58a6c, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe5845e): (unknown callee), target 0xffe57e98, stack arg0=`ret(0xffe58a6c)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe584ae): (unknown callee), target 0xffe6dce0, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe584fa): (unknown callee), target 0xffe6dce0, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe58464).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe5846a).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe5846e).
  Grouped accesses: 2 4-byte load records with target `abs 0x30001eac`, including `tst.l` at 0xffe58478, `move.l` at 0xffe584b8.
  Grouped accesses: 7 4-byte load records with target `abs 0x3000031c`, including `movea.l` at 0xffe58484, `movea.l` at 0xffe5848e, `movea.l` at 0xffe58494.
  Halfword store: target `[0x3000031c] +0x88`, value `0x4000` (`move.w` at 0xffe5848a).
  Word store: target `[0x3000031c] +0x8c`, value `0x0` (`clr.l` at 0xffe58490).
  Halfword store: target `[0x3000031c] +0x92`, value `0x953` (`move.w` at 0xffe5849a).
  Byte store: target `[0x3000031c] +0x16`, value `0x8c` (`move.b` at 0xffe584ca).
  Halfword store: target `[0x3000031c] +0xa0`, value `0x6000` (`move.w` at 0xffe584d4).
  Word store: target `[0x3000031c] +0xa4`, value `0x0` (`clr.l` at 0xffe584da).
  Halfword store: target `[0x3000031c] +0xaa`, value `0x953` (`move.w` at 0xffe584e4).
  Branch (`ble.b` at 0xffe5847c): taken to 0xffe584b8 when `[0x30001eac] <= 0x0 (signed, 32-bit)`, falls through to 0xffe5847e when `[0x30001eac] > 0x0 (signed, 32-bit)`.
  Branch (`bne.b` at 0xffe584be): taken to 0xffe58504 when `Z == 0`, falls through to 0xffe584c0 when `not (Z == 0)`.
  d0 has the literal analysis result `unknown` on return (`rts` at 0xffe5850a).

- `0xffe58110`, `root/6.5.29/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.29:l1#1`, version `1.40.5`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe58110 to 0xffe581cc (exclusive), 188 bytes.
  Carrier: 9295164 bytes, SHA256 `80f4a616601d187bc3839a2758f2e7f2d0db962ad7fd8f2d96d41bcd15bdf88c`.
  Package: carrier offset 0xefda8, 982396 bytes, SHA256 `67295cdae5ff370410852805127e6f410676e912b07eab1881c2caca7f70e5c5`.
  Payload: carrier offset 0xefdd4, 982352 bytes, SHA256 `1ba25f46035369eeb52170440fd7a6e8dd4b3ef74024f3e9bbaded05b899f91a`.
  Read window: SHA256 `d516c19fb73587587d0cf3669cdcf4c09f216277f4f76651d6194c4a4a6e9da8`.
  Call (`jsr` at 0xffe58116): (unknown callee), target 0xffe5872c, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe5811e): (unknown callee), target 0xffe57b58, stack arg0=`ret(0xffe5872c)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe5816e): (unknown callee), target 0xffe6db98, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe581ba): (unknown callee), target 0xffe6db98, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe58124).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe5812a).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe5812e).
  Grouped accesses: 2 4-byte load records with target `abs 0x30001fa0`, including `tst.l` at 0xffe58138, `move.l` at 0xffe58178.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000348`, including `movea.l` at 0xffe58144, `movea.l` at 0xffe5814e, `movea.l` at 0xffe58154.
  Halfword store: target `[0x30000348] +0x88`, value `0x4000` (`move.w` at 0xffe5814a).
  Word store: target `[0x30000348] +0x8c`, value `0x0` (`clr.l` at 0xffe58150).
  Halfword store: target `[0x30000348] +0x92`, value `0x953` (`move.w` at 0xffe5815a).
  Byte store: target `[0x30000348] +0x16`, value `0x8c` (`move.b` at 0xffe5818a).
  Halfword store: target `[0x30000348] +0xa0`, value `0x6000` (`move.w` at 0xffe58194).
  Word store: target `[0x30000348] +0xa4`, value `0x0` (`clr.l` at 0xffe5819a).
  Halfword store: target `[0x30000348] +0xaa`, value `0x953` (`move.w` at 0xffe581a4).
  Branch (`ble.b` at 0xffe5813c): taken to 0xffe58178 when `[0x30001fa0] <= 0x0 (signed, 32-bit)`, falls through to 0xffe5813e when `[0x30001fa0] > 0x0 (signed, 32-bit)`.
  Branch (`bne.b` at 0xffe5817e): taken to 0xffe581c4 when `Z == 0`, falls through to 0xffe58180 when `not (Z == 0)`.
  Return expression: d0 = `unknown` (`rts` at 0xffe581ca).

- `0xffe78ec8`, `root/6.5.29/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.29:l1#2`, version `1.40.5`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe78ec8 to 0xffe78f84 (exclusive), 188 bytes.
  Carrier: 9295164 bytes, SHA256 `80f4a616601d187bc3839a2758f2e7f2d0db962ad7fd8f2d96d41bcd15bdf88c`.
  Package: carrier offset 0x1dfb24, 1754868 bytes, SHA256 `728c3b39b0aaebd3f8e0ee49e7c42f0d3877d4b8cff52cb06fc0ea4606612802`.
  Payload: carrier offset 0x1dfb50, 1754824 bytes, SHA256 `6d104cefdb289169aa434cde1e6641c316128c7d81149d27fbe448b172d00459`.
  Read window: SHA256 `93f1d3cb2b5f5ef910c6a79182ac4296e1ad965e63816eac7db53111ee5e1e6c`.
  Call (`jsr` at 0xffe78ece): (unknown callee), target 0xffe794e4, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe78ed6): (unknown callee), target 0xffe78910, stack arg0=`ret(0xffe794e4)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe78f26): (unknown callee), target 0xffe90af0, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe78f72): (unknown callee), target 0xffe90af0, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe78edc).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe78ee2).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe78ee6).
  Grouped accesses: 2 4-byte load records with target `abs 0x3000289c`, including `tst.l` at 0xffe78ef0, `move.l` at 0xffe78f30.
  Grouped accesses: 7 4-byte load records with target `abs 0x3000092c`, including `movea.l` at 0xffe78efc, `movea.l` at 0xffe78f06, `movea.l` at 0xffe78f0c.
  Halfword store: target `[0x3000092c] +0x88`, value `0x4000` (`move.w` at 0xffe78f02).
  Word store: target `[0x3000092c] +0x8c`, value `0x0` (`clr.l` at 0xffe78f08).
  Halfword store: target `[0x3000092c] +0x92`, value `0x953` (`move.w` at 0xffe78f12).
  Byte store: target `[0x3000092c] +0x16`, value `0x8c` (`move.b` at 0xffe78f42).
  Halfword store: target `[0x3000092c] +0xa0`, value `0x6000` (`move.w` at 0xffe78f4c).
  Word store: target `[0x3000092c] +0xa4`, value `0x0` (`clr.l` at 0xffe78f52).
  Halfword store: target `[0x3000092c] +0xaa`, value `0x953` (`move.w` at 0xffe78f5c).
  Branch (`ble.b` at 0xffe78ef4): taken to 0xffe78f30 when `[0x3000289c] <= 0x0 (signed, 32-bit)`, falls through to 0xffe78ef6 when `[0x3000289c] > 0x0 (signed, 32-bit)`.
  Branch (`bne.b` at 0xffe78f36): taken to 0xffe78f7c when `Z == 0`, falls through to 0xffe78f38 when `not (Z == 0)`.
  Returning through (`rts` at 0xffe78f82) leaves d0 `unknown` in this analysis.

- `0xffe737d8`, `root/6.5.29/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.29:l1#3`, version `1.40.5`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe737d8 to 0xffe73894 (exclusive), 188 bytes.
  Carrier: 9295164 bytes, SHA256 `80f4a616601d187bc3839a2758f2e7f2d0db962ad7fd8f2d96d41bcd15bdf88c`.
  Package: carrier offset 0x38c218, 1774128 bytes, SHA256 `17388cb36d1c3b9ac35e33e5964e0641334b6b24dd1f787d7d81d0fe3c494a2c`.
  Payload: carrier offset 0x38c244, 1774084 bytes, SHA256 `2028cd13279e10c398b6e1b86e9db5d8fb7802cdddb4de5912588c8da736ecc0`.
  Read window: SHA256 `417ff9d9efe04f3c48fdaf1c2920ad8c450a83e314b018096cfadaac3e904c11`.
  Call (`jsr` at 0xffe737de): (unknown callee), target 0xffe73b08, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe737e6): (unknown callee), target 0xffe733dc, stack arg0=`ret(0xffe73b08)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe73836): (unknown callee), target 0xffe95428, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe73882): (unknown callee), target 0xffe95428, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe737ec).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe737f2).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe737f6).
  Grouped accesses: 2 4-byte load records with target `abs 0x30003338`, including `tst.l` at 0xffe73800, `move.l` at 0xffe73840.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000ba8`, including `movea.l` at 0xffe7380c, `movea.l` at 0xffe73816, `movea.l` at 0xffe7381c.
  Halfword store: target `[0x30000ba8] +0x88`, value `0x4000` (`move.w` at 0xffe73812).
  Word store: target `[0x30000ba8] +0x8c`, value `0x0` (`clr.l` at 0xffe73818).
  Halfword store: target `[0x30000ba8] +0x92`, value `0x953` (`move.w` at 0xffe73822).
  Byte store: target `[0x30000ba8] +0x16`, value `0x8c` (`move.b` at 0xffe73852).
  Halfword store: target `[0x30000ba8] +0xa0`, value `0x6000` (`move.w` at 0xffe7385c).
  Word store: target `[0x30000ba8] +0xa4`, value `0x0` (`clr.l` at 0xffe73862).
  Halfword store: target `[0x30000ba8] +0xaa`, value `0x953` (`move.w` at 0xffe7386c).
  Branch (`ble.b` at 0xffe73804): taken to 0xffe73840 when `[0x30003338] <= 0x0 (signed, 32-bit)`, falls through to 0xffe73806 when `[0x30003338] > 0x0 (signed, 32-bit)`.
  Branch (`bne.b` at 0xffe73846): taken to 0xffe7388c when `Z == 0`, falls through to 0xffe73848 when `not (Z == 0)`.
  At the return (`rts` at 0xffe73892), d0 is `unknown`.

- `0xffe55544`, `root/6.5.28/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.28:l1#0`, version `1.38.4`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe55544 to 0xffe55600 (exclusive), 188 bytes.
  Carrier: 9293972 bytes, SHA256 `e5b65e5f28d7fba6013a8618f55c34bed7f60f7c6f46d7e9c5b8b7b2eabbe7b3`.
  Package: carrier offset 0x0, 982424 bytes, SHA256 `1eb7cb1ad9e96ce311f598424e9b1a836c9c04f585798d659a4e1ddefe362d4b`.
  Payload: carrier offset 0x58, 982336 bytes, SHA256 `56e24f817b86ba5d2a0b48c7a322d287e7481a6a1173db19136547872804bf26`.
  Read window: SHA256 `23565840ae1c40eee4a19925c537420dd6a5af6bc242d1e0aa806f3d3962a440`.
  Call (`jsr` at 0xffe5554a): (unknown callee), target 0xffe55b60, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe55552): (unknown callee), target 0xffe54f8c, stack arg0=`ret(0xffe55b60)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe555a2): (unknown callee), target 0xffe6dcd0, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe555ee): (unknown callee), target 0xffe6dcd0, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe55558).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe5555e).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe55562).
  Grouped accesses: 2 4-byte load records with target `abs 0x30001e40`, including `tst.l` at 0xffe5556c, `move.l` at 0xffe555ac.
  Grouped accesses: 7 4-byte load records with target `abs 0x300002bc`, including `movea.l` at 0xffe55578, `movea.l` at 0xffe55582, `movea.l` at 0xffe55588.
  Halfword store: target `[0x300002bc] +0x88`, value `0x4000` (`move.w` at 0xffe5557e).
  Word store: target `[0x300002bc] +0x8c`, value `0x0` (`clr.l` at 0xffe55584).
  Halfword store: target `[0x300002bc] +0x92`, value `0x953` (`move.w` at 0xffe5558e).
  Byte store: target `[0x300002bc] +0x16`, value `0x8c` (`move.b` at 0xffe555be).
  Halfword store: target `[0x300002bc] +0xa0`, value `0x6000` (`move.w` at 0xffe555c8).
  Word store: target `[0x300002bc] +0xa4`, value `0x0` (`clr.l` at 0xffe555ce).
  Halfword store: target `[0x300002bc] +0xaa`, value `0x953` (`move.w` at 0xffe555d8).
  Branch (`ble.b` at 0xffe55570): taken to 0xffe555ac when `[0x30001e40] <= 0x0 (signed, 32-bit)`, falls through to 0xffe55572 when `[0x30001e40] > 0x0 (signed, 32-bit)`.
  Branch (`bne.b` at 0xffe555b2): taken to 0xffe555f8 when `Z == 0`, falls through to 0xffe555b4 when `not (Z == 0)`.
  The exit at (`rts` at 0xffe555fe) records d0 = `unknown`.

- `0xffe55194`, `root/6.5.28/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.28:l1#1`, version `1.38.4`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe55194 to 0xffe55250 (exclusive), 188 bytes.
  Carrier: 9293972 bytes, SHA256 `e5b65e5f28d7fba6013a8618f55c34bed7f60f7c6f46d7e9c5b8b7b2eabbe7b3`.
  Package: carrier offset 0xefd98, 982380 bytes, SHA256 `ea2b4e11ada0feac76aefd64e259195de58fca3acf9ec4184bf2253c4b8b4d08`.
  Payload: carrier offset 0xefdc4, 982336 bytes, SHA256 `ac159ab4f345a13eb60f0a0cca6c1d5860a166315e65af27e019685d97a1f8e0`.
  Read window: SHA256 `3ace1988f34c0fbf4cc62fcdfbad66f1487b1edc06ede6b4428aecc3a0e586a9`.
  Call (`jsr` at 0xffe5519a): (unknown callee), target 0xffe557b0, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe551a2): (unknown callee), target 0xffe54bdc, stack arg0=`ret(0xffe557b0)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe551f2): (unknown callee), target 0xffe6db88, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe5523e): (unknown callee), target 0xffe6db88, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe551a8).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe551ae).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe551b2).
  Grouped accesses: 2 4-byte load records with target `abs 0x30001f30`, including `tst.l` at 0xffe551bc, `move.l` at 0xffe551fc.
  Grouped accesses: 7 4-byte load records with target `abs 0x300002e8`, including `movea.l` at 0xffe551c8, `movea.l` at 0xffe551d2, `movea.l` at 0xffe551d8.
  Halfword store: target `[0x300002e8] +0x88`, value `0x4000` (`move.w` at 0xffe551ce).
  Word store: target `[0x300002e8] +0x8c`, value `0x0` (`clr.l` at 0xffe551d4).
  Halfword store: target `[0x300002e8] +0x92`, value `0x953` (`move.w` at 0xffe551de).
  Byte store: target `[0x300002e8] +0x16`, value `0x8c` (`move.b` at 0xffe5520e).
  Halfword store: target `[0x300002e8] +0xa0`, value `0x6000` (`move.w` at 0xffe55218).
  Word store: target `[0x300002e8] +0xa4`, value `0x0` (`clr.l` at 0xffe5521e).
  Halfword store: target `[0x300002e8] +0xaa`, value `0x953` (`move.w` at 0xffe55228).
  Branch (`ble.b` at 0xffe551c0): taken to 0xffe551fc when `[0x30001f30] <= 0x0 (signed, 32-bit)`, falls through to 0xffe551c2 when `[0x30001f30] > 0x0 (signed, 32-bit)`.
  Branch (`bne.b` at 0xffe55202): taken to 0xffe55248 when `Z == 0`, falls through to 0xffe55204 when `not (Z == 0)`.
  d0 has the literal analysis result `unknown` on return (`rts` at 0xffe5524e).

- `0xffe75754`, `root/6.5.28/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.28:l1#2`, version `1.38.4`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe75754 to 0xffe75810 (exclusive), 188 bytes.
  Carrier: 9293972 bytes, SHA256 `e5b65e5f28d7fba6013a8618f55c34bed7f60f7c6f46d7e9c5b8b7b2eabbe7b3`.
  Package: carrier offset 0x1dfb04, 1754852 bytes, SHA256 `ea36aeea83c9a9aa85c4bc761d80c960851e77a88d69967aa33d58e96f5294f3`.
  Payload: carrier offset 0x1dfb30, 1754808 bytes, SHA256 `4530117d97540ee5a199d8cb8480cd1799ca11b9bf3538d46217ddb13743008a`.
  Read window: SHA256 `a837d672534908fa484cacffcb2754e55ca5937b8c20c8803ac7f9c1bac14b69`.
  Call (`jsr` at 0xffe7575a): (unknown callee), target 0xffe75d70, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe75762): (unknown callee), target 0xffe7519c, stack arg0=`ret(0xffe75d70)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe757b2): (unknown callee), target 0xffe90ae0, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe757fe): (unknown callee), target 0xffe90ae0, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe75768).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe7576e).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe75772).
  Grouped accesses: 2 4-byte load records with target `abs 0x3000282c`, including `tst.l` at 0xffe7577c, `move.l` at 0xffe757bc.
  Grouped accesses: 7 4-byte load records with target `abs 0x300008bc`, including `movea.l` at 0xffe75788, `movea.l` at 0xffe75792, `movea.l` at 0xffe75798.
  Halfword store: target `[0x300008bc] +0x88`, value `0x4000` (`move.w` at 0xffe7578e).
  Word store: target `[0x300008bc] +0x8c`, value `0x0` (`clr.l` at 0xffe75794).
  Halfword store: target `[0x300008bc] +0x92`, value `0x953` (`move.w` at 0xffe7579e).
  Byte store: target `[0x300008bc] +0x16`, value `0x8c` (`move.b` at 0xffe757ce).
  Halfword store: target `[0x300008bc] +0xa0`, value `0x6000` (`move.w` at 0xffe757d8).
  Word store: target `[0x300008bc] +0xa4`, value `0x0` (`clr.l` at 0xffe757de).
  Halfword store: target `[0x300008bc] +0xaa`, value `0x953` (`move.w` at 0xffe757e8).
  Branch (`ble.b` at 0xffe75780): taken to 0xffe757bc when `[0x3000282c] <= 0x0 (signed, 32-bit)`, falls through to 0xffe75782 when `[0x3000282c] > 0x0 (signed, 32-bit)`.
  Branch (`bne.b` at 0xffe757c2): taken to 0xffe75808 when `Z == 0`, falls through to 0xffe757c4 when `not (Z == 0)`.
  Return expression: d0 = `unknown` (`rts` at 0xffe7580e).

- `0xffe7345c`, `root/6.5.28/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.28:l1#3`, version `1.38.4`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe7345c to 0xffe73518 (exclusive), 188 bytes.
  Carrier: 9293972 bytes, SHA256 `e5b65e5f28d7fba6013a8618f55c34bed7f60f7c6f46d7e9c5b8b7b2eabbe7b3`.
  Package: carrier offset 0x38c1e8, 1772984 bytes, SHA256 `2094d418e7129ff920233c3f5385fc7bb6bb6e65cbd203164e10250c539b2e1c`.
  Payload: carrier offset 0x38c214, 1772940 bytes, SHA256 `c09284761067f8587cbe2029bca484448b73c1e2c43870af5fd5cb6fa5d0f358`.
  Read window: SHA256 `ec6617633a636c2b7d039ff03874fdcb9e8075bdbb04e9a755ec7085249d8622`.
  Call (`jsr` at 0xffe73462): (unknown callee), target 0xffe7378c, stack arg0=`a5`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe7346a): (unknown callee), target 0xffe73060, stack arg0=`ret(0xffe7378c)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe734ba): (unknown callee), target 0xffe950ac, stack arg0=`0x0`, stack arg1=`0x40000000`, stack arg2=`0x2`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe73506): (unknown callee), target 0xffe950ac, stack arg0=`0x1`, stack arg1=`0x60000000`, stack arg2=`0x3`, stack arg3=`sp-8`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `sp -0x8`, value `0x0` (`clr.b` at 0xffe73470).
  Byte store: target `sp -0x7`, value `0x5` (`move.b` at 0xffe73476).
  Byte store: target `sp -0x6`, value `0x0` (`clr.b` at 0xffe7347a).
  Grouped accesses: 2 4-byte load records with target `abs 0x300032f4`, including `tst.l` at 0xffe73484, `move.l` at 0xffe734c4.
  Grouped accesses: 7 4-byte load records with target `abs 0x30000ba8`, including `movea.l` at 0xffe73490, `movea.l` at 0xffe7349a, `movea.l` at 0xffe734a0.
  Halfword store: target `[0x30000ba8] +0x88`, value `0x4000` (`move.w` at 0xffe73496).
  Word store: target `[0x30000ba8] +0x8c`, value `0x0` (`clr.l` at 0xffe7349c).
  Halfword store: target `[0x30000ba8] +0x92`, value `0x953` (`move.w` at 0xffe734a6).
  Byte store: target `[0x30000ba8] +0x16`, value `0x8c` (`move.b` at 0xffe734d6).
  Halfword store: target `[0x30000ba8] +0xa0`, value `0x6000` (`move.w` at 0xffe734e0).
  Word store: target `[0x30000ba8] +0xa4`, value `0x0` (`clr.l` at 0xffe734e6).
  Halfword store: target `[0x30000ba8] +0xaa`, value `0x953` (`move.w` at 0xffe734f0).
  Branch (`ble.b` at 0xffe73488): taken to 0xffe734c4 when `[0x300032f4] <= 0x0 (signed, 32-bit)`, falls through to 0xffe7348a when `[0x300032f4] > 0x0 (signed, 32-bit)`.
  Branch (`bne.b` at 0xffe734ca): taken to 0xffe73510 when `Z == 0`, falls through to 0xffe734cc when `not (Z == 0)`.
  Returning through (`rts` at 0xffe73516) leaves d0 `unknown` in this analysis.
