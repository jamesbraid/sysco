# ARM Linux 2.6 I2C and power configuration routines in 1.44.0

Read from `6.5.30/usr/cpu/firmware/sysco/l1.bin`, package `6.5.30:l1#6` (version 1.44.0),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0xa936a8`.
Carrier SHA-256: `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
Package SHA-256: `e57ab8ecd48858e39ecdd0050a23e444a491c35e87f9d8ebb48881fc25229a06`.
Inner executable SHA-256: `b33154e53b00531623db6ff365e80b1c1825bc9427961c59fdc132c24f0c819e`.
The ELF identifies 32-bit little-endian ARM. Its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1629656 bytes.
Segment SHA-256: `f6491062ef0a2afd254f8510f82b4814c563c9256691f9a2b393cf6e84ba7fd8`. Analysed routine spans use the exact
ELF function symbols and sizes. They can include trailing literal words after a return.

`sp` denotes the entry stack pointer. Register expressions such as `r0` denote entry
values, and brackets denote loaded values. Decimal numbers inside expressions retain the
exact tool notation. Call candidates use the assumed ARM AAPCS
register locations r0 through r3 with unknown arity. Custom calling conventions remain unknown.
Call lines include every reported call instruction and every printed register slot.
Unknown values and predicates include unsupported instruction effects and merged paths.
Numeric call targets have unknown callee names in the flat-image analysis. Role: unknown. Pointee meanings remain unknown. Selected access and branch facts below cover known control-flow edges. The
full fact sheets retain all reported accesses and implicit stack operations. Separate loads at the same printed address need not observe the same value.
No unresolved computed transfers were reported for these routines.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-i2c-power-config-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register
state from the analysis rather than a C return type.


Groups summarize static access records by printed shape. Instruction-address order can include different control-flow paths.

## `Unknown_i2cConfig`

- `Unknown_i2cConfig`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x29e14 to 0x29e74.
  Established facts:
  Call (`bl` at 0x29e30): (unknown callee), target 0x11f7c8, r0=`0x0`, r1=`0x153628`, r2=`0x9`, r3=`r3`.
  Call (`bl` at 0x29e54): (unknown callee), target 0x11fca8, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x29e68): (unknown callee), target 0x59920, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `ret(0x59920)` (`ldm` at 0x29e6c, `bl` at 0x29e68).

## `Dorado_i2cConfig`

- `Dorado_i2cConfig`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3589c to 0x35b30.
  Established facts:
  Call (`bl` at 0x358b0): (unknown callee), target 0x10a5a8, r0=`0x356bc`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0x358c0): (unknown callee), target 0x122e6c, r0=`0x0`, r1=`0x15891c`, r2=`0x2`, r3=`unknown`.
  Call (`bl` at 0x358c8): (unknown callee), target 0x10e47c, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x358d8): (unknown callee), target 0x127538, r0=`0x0`, r1=`0x15892c`, r2=`0x2`, r3=`unknown`.
  Call (`bl` at 0x358e8): (unknown callee), target 0x1247d4, r0=`0x0`, r1=`0x158a6c`, r2=`0x16`, r3=`unknown`.
  Call (`bl` at 0x358f8): (unknown callee), target 0x11f7c8, r0=`0x0`, r1=`0x158c64`, r2=`0x14`, r3=`unknown`.
  Call (`bl` at 0x35908): (unknown callee), target 0x111324, r0=`0x0`, r1=`0x158d90`, r2=`0x7`, r3=`unknown`.
  Call (`bl` at 0x35924): (unknown callee), target 0x52878, r0=`0x0`, r1=`0x158e68`, r2=`0xe`, r3=`0x34004`.
  Call (`bl` at 0x35934): (unknown callee), target 0x72138, r0=`0x0`, r1=`0x15942c`, r2=`0x7`, r3=`unknown`.
  Call (`bl` at 0x35944): (unknown callee), target 0x7e08c, r0=`0x0`, r1=`0x158ff0`, r2=`0x4`, r3=`unknown`.
  Call (`bl` at 0x35954): (unknown callee), target 0x61c68, r0=`0x0`, r1=`0x159010`, r2=`0x5`, r3=`unknown`.
  Call (`bl` at 0x35964): (unknown callee), target 0x61848, r0=`0x0`, r1=`0x1590a0`, r2=`0xb`, r3=`unknown`.
  Call (`bl` at 0x35978): (unknown callee), target 0x35d98, r0=`0x0`, r1=`0x159134`, r2=`0x5`, r3=`0x14`.
  Call (`bl` at 0x35984): (unknown callee), target 0x13b90c, r0=`0x19e634`, r1=`0x2`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x359a8): (unknown callee), target 0x8201c, r0=`0x159620`, r1=`0x1`, r2=`0x35464`, r3=`0x0`.
  Call (`bl` at 0x359b8): (unknown callee), target 0x59ea4, r0=`0x0`, r1=`0x159588`, r2=`0x6`, r3=`unknown`.
  Call (`bl` at 0x359c0): (unknown callee), target 0x70214, r0=`0x15`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x359c4): (unknown callee), target 0x33e00, r0=`ret(0x70214)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x359c8): (unknown callee), target 0x59920, r0=`ret(0x33e00)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x359cc): (unknown callee), target 0x33f54, r0=`ret(0x59920)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x359d8): (unknown callee), target 0x71b94, r0=`0x159634`, r1=`0x3`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x359ec): (unknown callee), target 0x730cc, r0=`0x0`, r1=`0x1594b8`, r2=`0x2`, r3=`0x1`.
  Call (`bl` at 0x359f4): (unknown callee), target 0x735ec, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x35a20): (unknown callee), target 0x78b38, r0=`0x0`, r1=`0x1`, r2=`0x1594e8`, r3=`0x4`.
  Call (`bl` at 0x35a2c): (unknown callee), target 0x10b350, r0=`0x19e644`, r1=`0x3`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x35a40): (unknown callee), target 0x10b4e8, r0=`0x1289e8`, r1=`0x0`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x35a54): (unknown callee), target 0x10b4e8, r0=`0x114238`, r1=`0x0`, r2=`0x0`, r3=`0x2`.
  Call (`bl` at 0x35a68): (unknown callee), target 0x10b4e8, r0=`0x125de0`, r1=`0x2`, r2=`0x0`, r3=`0x3`.
  Byte load: target `abs 0x1a59f1` (`ldrb` at 0x35a70).
  Branch (`beq` at 0x35a78): taken to 0x35aa4 when `[0x1a59f1]:1 == 0x0`, falls through to 0x35a7c when `[0x1a59f1]:1 != 0x0`.
  Call (`bl` at 0x35a8c): (unknown callee), target 0x10b4e8, r0=`0x1289e8`, r1=`0x1`, r2=`0x1`, r3=`0x1`.
  Call (`bl` at 0x35aa0): (unknown callee), target 0x10b4e8, r0=`0x114238`, r1=`0x1`, r2=`0x3`, r3=`0x6`.
  Call (`bl` at 0x35ab4): (unknown callee), target 0x10b4e8, r0=`0x125de0`, r1=`0x0`, r2=`0x15`, r3=`0x15`.
  Recorded r0 = `ret(0x10b4e8)` (`ldm` at 0x35abc, `bl` at 0x35ab4).

## `Gemini_i2cConfig`

- `Gemini_i2cConfig`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2cda0 to 0x2d00c.
  Established facts:
  Call (`bl` at 0x2cdb4): (unknown callee), target 0x10a5a8, r0=`0x2cbc0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0x2cdc4): (unknown callee), target 0x122e6c, r0=`0x0`, r1=`0x15548c`, r2=`0x2`, r3=`unknown`.
  Call (`bl` at 0x2cdcc): (unknown callee), target 0x10e47c, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2cddc): (unknown callee), target 0x127538, r0=`0x0`, r1=`0x1554a4`, r2=`0x2`, r3=`unknown`.
  Call (`bl` at 0x2cdec): (unknown callee), target 0x1247d4, r0=`0x0`, r1=`0x1555b8`, r2=`0x12`, r3=`unknown`.
  Call (`bl` at 0x2cdfc): (unknown callee), target 0x11f7c8, r0=`0x0`, r1=`0x155770`, r2=`0x15`, r3=`unknown`.
  Call (`bl` at 0x2ce0c): (unknown callee), target 0x111324, r0=`0x0`, r1=`0x1558a8`, r2=`0x7`, r3=`unknown`.
  Call (`bl` at 0x2ce28): (unknown callee), target 0x52878, r0=`0x0`, r1=`0x1559c0`, r2=`0x14`, r3=`0x2bc8c`.
  Call (`bl` at 0x2ce38): (unknown callee), target 0x72138, r0=`0x0`, r1=`0x155d08`, r2=`0x7`, r3=`unknown`.
  Call (`bl` at 0x2ce48): (unknown callee), target 0x7e08c, r0=`0x0`, r1=`0x155bf0`, r2=`0x5`, r3=`unknown`.
  Call (`bl` at 0x2ce58): (unknown callee), target 0x61c68, r0=`0x0`, r1=`0x155c18`, r2=`0x4`, r3=`unknown`.
  Call (`bl` at 0x2ce68): (unknown callee), target 0x61848, r0=`0x0`, r1=`0x155c9c`, r2=`0x9`, r3=`unknown`.
  Call (`bl` at 0x2ce74): (unknown callee), target 0x13b90c, r0=`0x19e4cc`, r1=`0x2`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2ce98): (unknown callee), target 0x8201c, r0=`0x155eac`, r1=`0x1`, r2=`0x2c96c`, r3=`0x0`.
  Call (`bl` at 0x2cea8): (unknown callee), target 0x59ea4, r0=`0x0`, r1=`0x155e10`, r2=`0x6`, r3=`unknown`.
  Call (`bl` at 0x2ceb4): (unknown callee), target 0x61078, r0=`0x155ea0`, r1=`0x1`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2cebc): (unknown callee), target 0x70214, r0=`0x11`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2cec0): (unknown callee), target 0x2b8c8, r0=`ret(0x70214)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2cec4): (unknown callee), target 0x59920, r0=`ret(0x2b8c8)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2cec8): (unknown callee), target 0x2bbdc, r0=`ret(0x59920)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2cedc): (unknown callee), target 0x730cc, r0=`0x0`, r1=`0x155d8c`, r2=`0x2`, r3=`0x1`.
  Call (`bl` at 0x2cee4): (unknown callee), target 0x735ec, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2cef4): (unknown callee), target 0x12a23c, r0=`0x0`, r1=`0x155dc8`, r2=`0x1`, r3=`unknown`.
  Call (`bl` at 0x2cf00): (unknown callee), target 0x10b350, r0=`0x19e4dc`, r1=`0x4`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2cf14): (unknown callee), target 0x10b4e8, r0=`0x125de0`, r1=`0x0`, r2=`0x11`, r3=`0x11`.
  Call (`bl` at 0x2cf28): (unknown callee), target 0x10b4e8, r0=`0x114238`, r1=`0x0`, r2=`0x0`, r3=`0x1`.
  Call (`bl` at 0x2cf3c): (unknown callee), target 0x10b4e8, r0=`0x125de0`, r1=`0x3`, r2=`0x0`, r3=`0x3`.
  Byte load: target `abs 0x1a5989` (`ldrb` at 0x2cf44).
  Branch (`beq` at 0x2cf4c): taken to 0x2cf78 when `[0x1a5989]:1 == 0x0`, falls through to 0x2cf50 when `[0x1a5989]:1 != 0x0`.
  Call (`bl` at 0x2cf60): (unknown callee), target 0x10b4e8, r0=`0x1289e8`, r1=`0x1`, r2=`0x1`, r3=`0x1`.
  Call (`bl` at 0x2cf74): (unknown callee), target 0x10b4e8, r0=`0x114238`, r1=`0x1`, r2=`0x3`, r3=`0x6`.
  Call (`bl` at 0x2cf88): (unknown callee), target 0x10b4e8, r0=`0x1289e8`, r1=`0x2`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x2cf9c): (unknown callee), target 0x10b4e8, r0=`0x114238`, r1=`0x2`, r2=`0x2`, r3=`0x2`.
  Recorded r0 = `ret(0x10b4e8)` (`ldm` at 0x2cfa4, `bl` at 0x2cf9c).

## `Vortix_i2cConfig`

- `Vortix_i2cConfig`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2b298 to 0x2b5b0.
  Established facts:
  Call (`bl` at 0x2b2ac): (unknown callee), target 0x10a5a8, r0=`0x2b080`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0x2b2bc): (unknown callee), target 0x122e6c, r0=`0x0`, r1=`0x1537ac`, r2=`0x5`, r3=`unknown`.
  Call (`bl` at 0x2b2cc): (unknown callee), target 0x127538, r0=`0x0`, r1=`0x1537e4`, r2=`0x6`, r3=`unknown`.
  Call (`bl` at 0x2b2dc): (unknown callee), target 0x1247d4, r0=`0x0`, r1=`0x153afc`, r2=`0x36`, r3=`unknown`.
  Call (`bl` at 0x2b2ec): (unknown callee), target 0x11f7c8, r0=`0x0`, r1=`0x154184`, r2=`0x48`, r3=`unknown`.
  Call (`bl` at 0x2b2fc): (unknown callee), target 0x111324, r0=`0x0`, r1=`0x154544`, r2=`0x13`, r3=`unknown`.
  Call (`bl` at 0x2b308): (unknown callee), target 0x11163c, r0=`0x0`, r1=`0x40`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2b324): (unknown callee), target 0x52878, r0=`0x0`, r1=`0x154794`, r2=`0x29`, r3=`0x2a2f4`.
  Call (`bl` at 0x2b334): (unknown callee), target 0x7e08c, r0=`0x0`, r1=`0x154c10`, r2=`0xe`, r3=`unknown`.
  Call (`bl` at 0x2b344): (unknown callee), target 0x72138, r0=`0x0`, r1=`0x154cc8`, r2=`0x13`, r3=`unknown`.
  Call (`bl` at 0x2b354): (unknown callee), target 0x61c68, r0=`0x0`, r1=`0x154c80`, r2=`0x1`, r3=`unknown`.
  Call (`bl` at 0x2b364): (unknown callee), target 0x61848, r0=`0x0`, r1=`0x154ca4`, r2=`0x3`, r3=`unknown`.
  Call (`bl` at 0x2b370): (unknown callee), target 0x13b90c, r0=`0x19e428`, r1=`0x6`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2b380): (unknown callee), target 0x2d214, r0=`0x0`, r1=`0x154e5c`, r2=`0x2`, r3=`unknown`.
  Call (`bl` at 0x2b3a4): (unknown callee), target 0x8201c, r0=`0x154e0c`, r1=`0x4`, r2=`0x2b058`, r3=`0x0`.
  Call (`bl` at 0x2b3b4): (unknown callee), target 0x59ea4, r0=`0x0`, r1=`0x154f8c`, r2=`0x1a`, r3=`unknown`.
  Call (`bl` at 0x2b3c0): (unknown callee), target 0x61078, r0=`0x154dac`, r1=`0x4`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2b3c8): (unknown callee), target 0x70214, r0=`0x9`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2b3cc): (unknown callee), target 0x2a120, r0=`ret(0x70214)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2b3d0): (unknown callee), target 0x59920, r0=`ret(0x2a120)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2b3d4): (unknown callee), target 0x2a210, r0=`ret(0x59920)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2b3e0): (unknown callee), target 0x10b350, r0=`0x19e458`, r1=`0x6`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2b3f4): (unknown callee), target 0x10b4e8, r0=`0x1289e8`, r1=`0x0`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x2b408): (unknown callee), target 0x10b4e8, r0=`0x125de0`, r1=`0x0`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x2b41c): (unknown callee), target 0x10b4e8, r0=`0x114238`, r1=`0x0`, r2=`0x0`, r3=`0x1`.
  Call (`bl` at 0x2b430): (unknown callee), target 0x10b4e8, r0=`0x1289e8`, r1=`0x1`, r2=`0x1`, r3=`0x1`.
  Call (`bl` at 0x2b444): (unknown callee), target 0x10b4e8, r0=`0x125de0`, r1=`0x1`, r2=`0x9`, r3=`0x9`.
  Call (`bl` at 0x2b458): (unknown callee), target 0x10b4e8, r0=`0x114238`, r1=`0x1`, r2=`0x2`, r3=`0x2`.
  Call (`bl` at 0x2b460): (unknown callee), target 0x80e60, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0x2b46c): taken to 0x2b498 when `ret(0x80e60) == 0x0`, falls through to 0x2b470 when `ret(0x80e60) != 0x0` (source `bl` at 0x2b460).
  Call (`bl` at 0x2b480): (unknown callee), target 0x10b4e8, r0=`0x1289e8`, r1=`0x2`, r2=`0x2`, r3=`0x2`.
  Call (`bl` at 0x2b494): (unknown callee), target 0x10b4e8, r0=`0x114238`, r1=`0x2`, r2=`0x3`, r3=`0x6`.
  Call (`bl` at 0x2b49c): (unknown callee), target 0x80e60, r0=`0x1`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0x2b4a8): taken to 0x2b4d4 when `ret(0x80e60) == 0x0`, falls through to 0x2b4ac when `ret(0x80e60) != 0x0` (source `bl` at 0x2b49c).
  Call (`bl` at 0x2b4bc): (unknown callee), target 0x10b4e8, r0=`0x1289e8`, r1=`0x3`, r2=`0x3`, r3=`0x3`.
  Call (`bl` at 0x2b4d0): (unknown callee), target 0x10b4e8, r0=`0x114238`, r1=`0x3`, r2=`0x7`, r3=`0xa`.
  Call (`bl` at 0x2b4d8): (unknown callee), target 0x80e60, r0=`0x2`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2b4f8): (unknown callee), target 0x10b4e8, r0=`0x1289e8`, r1=`0x4`, r2=`0x4`, r3=`0x4`.
  Call (`bl` at 0x2b50c): (unknown callee), target 0x10b4e8, r0=`0x114238`, r1=`0x4`, r2=`0xb`, r3=`0xe`.
  Call (`bl` at 0x2b514): (unknown callee), target 0x80e60, r0=`0x3`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0x2b520): taken to 0x2b54c when `ret(0x80e60) == 0x0`, falls through to 0x2b524 when `ret(0x80e60) != 0x0` (source `bl` at 0x2b514).
  Call (`bl` at 0x2b534): (unknown callee), target 0x10b4e8, r0=`0x1289e8`, r1=`0x5`, r2=`0x5`, r3=`0x5`.
  Call (`bl` at 0x2b548): (unknown callee), target 0x10b4e8, r0=`0x114238`, r1=`0x5`, r2=`0xf`, r3=`0x12`.
  Recorded r0 = `unknown` (`ldm` at 0x2b550).

## `OpusTio_i2cConfig`

- `OpusTio_i2cConfig`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x32620 to 0x328d4.
  Established facts:
  Call (`bl` at 0x32634): (unknown callee), target 0x10a5a8, r0=`0x32440`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0x32644): (unknown callee), target 0x122e6c, r0=`0x0`, r1=`0x157254`, r2=`0x2`, r3=`unknown`.
  Call (`bl` at 0x3264c): (unknown callee), target 0x10e47c, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3265c): (unknown callee), target 0x127538, r0=`0x0`, r1=`0x15726c`, r2=`0x2`, r3=`unknown`.
  Call (`bl` at 0x3266c): (unknown callee), target 0x1247d4, r0=`0x0`, r1=`0x157398`, r2=`0x14`, r3=`unknown`.
  Call (`bl` at 0x3267c): (unknown callee), target 0x11f7c8, r0=`0x0`, r1=`0x157578`, r2=`0x16`, r3=`unknown`.
  Call (`bl` at 0x3268c): (unknown callee), target 0x111324, r0=`0x0`, r1=`0x1576bc`, r2=`0x7`, r3=`unknown`.
  Call (`bl` at 0x326a8): (unknown callee), target 0x52878, r0=`0x0`, r1=`0x1577d0`, r2=`0x13`, r3=`0x314ac`.
  Call (`bl` at 0x326b8): (unknown callee), target 0x72138, r0=`0x0`, r1=`0x157afc`, r2=`0x7`, r3=`unknown`.
  Call (`bl` at 0x326c8): (unknown callee), target 0x7e08c, r0=`0x0`, r1=`0x1579e4`, r2=`0x5`, r3=`unknown`.
  Call (`bl` at 0x326d8): (unknown callee), target 0x61c68, r0=`0x0`, r1=`0x157a0c`, r2=`0x4`, r3=`unknown`.
  Call (`bl` at 0x326e8): (unknown callee), target 0x61848, r0=`0x0`, r1=`0x157a90`, r2=`0x9`, r3=`unknown`.
  Call (`bl` at 0x326f4): (unknown callee), target 0x13b90c, r0=`0x19e5a4`, r1=`0x2`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x32718): (unknown callee), target 0x8201c, r0=`0x157c54`, r1=`0x1`, r2=`0x321f0`, r3=`0x0`.
  Call (`bl` at 0x32728): (unknown callee), target 0x59ea4, r0=`0x0`, r1=`0x157be8`, r2=`0x4`, r3=`unknown`.
  Call (`bl` at 0x32734): (unknown callee), target 0x61078, r0=`0x157c48`, r1=`0x1`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3273c): (unknown callee), target 0x70214, r0=`0x13`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x32740): (unknown callee), target 0x310c8, r0=`ret(0x70214)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x32744): (unknown callee), target 0x59920, r0=`ret(0x310c8)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x32748): (unknown callee), target 0x313f8, r0=`ret(0x59920)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 2 reported byte load records with absolute target `0x1a59aa`, including target `abs 0x1a59aa` (`ldrb` at 0x32750), target `abs 0x1a59aa` (`ldrb` at 0x32828).
  Call (`bl` at 0x3276c): (unknown callee), target 0x730cc, r0=`0x0`, r1=`0x157b74`, r2=`0x1`, r3=`0x1`.
  Call (`bl` at 0x32774): (unknown callee), target 0x735ec, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x327a0): (unknown callee), target 0x78b38, r0=`0x0`, r1=`0x1`, r2=`0x157b8c`, r3=`0x2`.
  Call (`bl` at 0x327ac): (unknown callee), target 0x10b350, r0=`0x19e5b4`, r1=`0x4`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x327c0): (unknown callee), target 0x10b4e8, r0=`0x125de0`, r1=`0x0`, r2=`0x13`, r3=`0x13`.
  Call (`bl` at 0x327d4): (unknown callee), target 0x10b4e8, r0=`0x114238`, r1=`0x0`, r2=`0x0`, r3=`0x1`.
  Call (`bl` at 0x327e8): (unknown callee), target 0x10b4e8, r0=`0x125de0`, r1=`0x3`, r2=`0x0`, r3=`0x3`.
  Branch (`beq` at 0x327f8): taken to 0x32824 when `[0x1a59a9]:1 == 0x0`, falls through to 0x327fc when `[0x1a59a9]:1 != 0x0`.
  Call (`bl` at 0x3280c): (unknown callee), target 0x10b4e8, r0=`0x1289e8`, r1=`0x1`, r2=`0x1`, r3=`0x1`.
  Call (`bl` at 0x32820): (unknown callee), target 0x10b4e8, r0=`0x114238`, r1=`0x1`, r2=`0x3`, r3=`0x6`.
  Branch (`beq` at 0x32830): taken to 0x3285c when `[0x1a59aa]:1 == 0x0`, falls through to 0x32834 when `[0x1a59aa]:1 != 0x0`.
  Call (`bl` at 0x32844): (unknown callee), target 0x10b4e8, r0=`0x1289e8`, r1=`0x2`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x32858): (unknown callee), target 0x10b4e8, r0=`0x114238`, r1=`0x2`, r2=`0x2`, r3=`0x2`.
  Recorded r0 = `ret(0x10b4e8)` (`ldm` at 0x32860, `bl` at 0x327e8, `bl` at 0x32820, `bl` at 0x32858).

## `i2cMuxName`

- `i2cMuxName`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1239fc to 0x123a68.
  Established facts:
  Call (`bl` at 0x123a24): (unknown callee), target 0x1244a0, r0=`r0`, r1=`(r1 & 0xff)`, r2=`r2`, r3=`(r1 & 0xff)`.
  Grouped accesses: 2 reported word load records with printed base expression `ret(0x1244a0)` and nonnegative word-aligned offsets, including target `ret(0x1244a0) +0x4` (`ldr` at 0x123a34), target `ret(0x1244a0) +0x4` (`ldr` at 0x123a40) (source `bl` at 0x123a24).
  Branch (`bne` at 0x123a48): taken to 0x123a54 when `[ret(0x1244a0)+4] != 0x0`, falls through to 0x123a4c when `[ret(0x1244a0)+4] == 0x0` (source `bl` at 0x123a24).
  Recorded r0 = `unknown` (`ldm` at 0x123a60).

## `i2cMuxCount`

- `i2cMuxCount`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1235f8 to 0x1236e8.
  Established facts:
  Branch (`bge` at 0x12361c): taken to 0x12364c when `r0 >= [0x314fb8] (signed, 32-bit)`, falls through to 0x123620 when `r0 < [0x314fb8] (signed, 32-bit)`.
  Grouped accesses: 3 reported word load records with absolute target `0x314fb4`, including target `abs 0x314fb4` (`ldr` at 0x123638), target `abs 0x314fb4` (`ldr` at 0x123680), target `abs 0x314fb4` (`ldr` at 0x1236b8).
  Branch (`beq` at 0x123664): taken to 0x123694 when `[0x314fb8] == 0x0`, falls through to 0x123668 when `[0x314fb8] != 0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x1236d8).

## `IP57_PowerUp`

- `IP57_PowerUp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x83518 to 0x83624.
  Established facts:
  Call (`bl` at 0x83548): (unknown callee), target 0x84274, r0=`(r0 & 0xff)`, r1=`sp-24`, r2=`sp-24`, r3=`(r0 & 0xff)`.
  Branch (`beq` at 0x8355c): taken to 0x8356c when `ret(0x84274) == 0x0`, falls through to 0x83560 when `ret(0x84274) != 0x0` (source `bl` at 0x83548).
  Call (`bl` at 0x835a0): (unknown callee), target 0x7fe94, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0x835ac): taken to 0x83600 when `ret(0x7fe94) == 0x0`, falls through to 0x835b0 when `ret(0x7fe94) != 0x0` (source `bl` at 0x835a0).
  Byte load: target `[sp-24] +0x2` (`ldrb` at 0x835b4).
  Call (`bl` at 0x835d4): (unknown callee), target 0x125d44, r0=`0x0`, r1=`unknown`, r2=`0x80`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x83620).

## `IP57_PowerDown`

- `IP57_PowerDown`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x83624 to 0x83700.
  Established facts:
  Call (`bl` at 0x83654): (unknown callee), target 0x84274, r0=`(r0 & 0xff)`, r1=`sp-24`, r2=`sp-24`, r3=`(r0 & 0xff)`.
  Branch (`beq` at 0x83668): taken to 0x83678 when `ret(0x84274) == 0x0`, falls through to 0x8366c when `ret(0x84274) != 0x0` (source `bl` at 0x83654).
  Byte load: target `[sp-24] +0x2` (`ldrb` at 0x83690).
  Call (`bl` at 0x836b0): (unknown callee), target 0x125cf8, r0=`0x0`, r1=`unknown`, r2=`0x80`, r3=`unknown`.
  Branch (`beq` at 0x836c4): taken to 0x836dc when `ret(0x125cf8) == 0x0`, falls through to 0x836c8 when `ret(0x125cf8) != 0x0` (source `bl` at 0x836b0).
  Recorded r0 = `unknown` (`ldm` at 0x836fc).

## `env_pwrdwn`

- `env_pwrdwn`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaa248 to 0xaa490.
  Established facts:
  Call (`bl` at 0xaa28c): (unknown callee), target 0x408c8, r0=`r0`, r1=`0xe`, r2=`[0x16be78]`, r3=`0x1e`.
  Call (`bl` at 0xaa298): (unknown callee), target 0x22ca0, r0=`0x1a72ac`, r1=`0x0`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xaa2bc): (unknown callee), target 0x205bc, r0=`sp-128`, r1=`0x16be88`, r2=`[sp-132]`, r3=`sp-128`.
  Call (`bl` at 0xaa314): (unknown callee), target 0xa9740, r0=`0x2`, r1=`0x4101`, r2=`unknown`, r3=`sp-128`.
  Call (`bl` at 0xaa328): (unknown callee), target 0xaa490, r0=`0x1f4`, r1=`unknown`, r2=`unknown`, r3=`[sp-132]-5`.
  Call (`bl` at 0xaa32c): (unknown callee), target 0x5572c, r0=`ret(0xaa490)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xaa348): (unknown callee), target 0x205bc, r0=`sp-128`, r1=`0x16beac`, r2=`unknown`, r3=`sp-128`.
  Call (`bl` at 0xaa370): (unknown callee), target 0xa9740, r0=`0x2`, r1=`0x4102`, r2=`0x163`, r3=`sp-128`.
  Call (`bl` at 0xaa378): (unknown callee), target 0x2302c, r0=`0x1a72ac`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xaa388): (unknown callee), target 0x22ca0, r0=`0x1a72ac`, r1=`0x0`, r2=`unknown`, r3=`ret(0x5572c)`.
  Call (`bl` at 0xaa3a4): (unknown callee), target 0x205bc, r0=`sp-128`, r1=`0x16bed4`, r2=`unknown`, r3=`sp-128`.
  Call (`bl` at 0xaa3cc): (unknown callee), target 0xa9740, r0=`0x2`, r1=`0x4102`, r2=`0x163`, r3=`sp-128`.
  Call (`bl` at 0xaa3d4): (unknown callee), target 0x2302c, r0=`0x1a72ac`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xaa3f8): (unknown callee), target 0x205bc, r0=`sp-128`, r1=`0x16bf04`, r2=`unknown`, r3=`sp-128`.
  Call (`bl` at 0xaa420): (unknown callee), target 0xa9740, r0=`0x2`, r1=`0x4102`, r2=`0x164`, r3=`sp-128`.
  Grouped accesses: 2 reported word store records with absolute target `0x1a0380`, including target `abs 0x1a0380`, value `0x0` (`str` at 0xaa42c), target `abs 0x1a0380`, value `0x1` (`str` at 0xaa44c).
  Word load: target `abs 0x31a5fc` (`ldr` at 0xaa434).
  Call (`mov` at 0xaa440): (unknown callee), target (unknown target), r0=`0x7`, r1=`unknown`, r2=`0x1a0380`, r3=`[0x31a5fc]`.
  Call (`bl` at 0xaa454): (unknown callee), target 0x2302c, r0=`0x1a72ac`, r1=`unknown`, r2=`0x1a0380`, r3=`0x1`.
  Recorded r0 = `unknown` (`ldm` at 0xaa45c).

## `powerState`

- `powerState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x55e9c to 0x560d4.
  Established facts:
  Word load: target `abs 0x1a5c18` (`ldr` at 0x55eb8).
  Branch (`beq` at 0x55ec0): taken to 0x55ef0 when `[0x1a5c18] == 0x0`, falls through to 0x55ec4 when `[0x1a5c18] != 0x0`.
  Call (`bl` at 0x55f28): (unknown callee), target 0x53218, r0=`r0`, r1=`r1`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x55f88): (unknown callee), target 0x125c20, r0=`[sp-20]`, r1=`[[sp-40]+2]:1`, r2=`unknown`, r3=`[[sp-40]+2]:1`.
  Call (`bl` at 0x55fa8): (unknown callee), target 0x58bfc, r0=`[sp-40]`, r1=`unknown`, r2=`ret(0x125c20)`, r3=`unknown`.
  Call (`bl` at 0x55fcc): (unknown callee), target 0x58bfc, r0=`[sp-40]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`bne` at 0x56004): taken to 0x56014 when `[[sp-40]+4]:1 != 0xff`, falls through to 0x56008 when `[[sp-40]+4]:1 == 0xff`.
  Call (`bl` at 0x5601c): (unknown callee), target 0x58a38, r0=`[sp-20]`, r1=`[sp-40]`, r2=`unknown`, r3=`[[sp-40]+4]:1`.
  Call (`bl` at 0x56038): (unknown callee), target 0x5801c, r0=`[sp-20]`, r1=`ret(0x58a38)`, r2=`0x0`, r3=`sp-36`.
  Call (`bl` at 0x56044): (unknown callee), target 0x586fc, r0=`[sp-20]`, r1=`[sp-24]`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x560b4).

## `powerQuery`

- `powerQuery`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcc97c to 0xccf48.
  Established facts:
  Call (`bl` at 0xcc9a0): (unknown callee), target 0x6e140, r0=`r0`, r1=`r1`, r2=`r2`, r3=`0x0`.
  Call (`bl` at 0xcc9f8): (unknown callee), target 0x530f4, r0=`unknown`, r1=`sp-52`, r2=`unknown`, r3=`sp-52`.
  Call (`bl` at 0xcca30): (unknown callee), target 0x201fc, r0=`0x50`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 2 reported word load records with printed base expression `[sp-28]` and nonnegative word-aligned offsets, including target `[sp-28] +0x0` (`ldr` at 0xcca40), target `[sp-28] +0x0` (`ldr` at 0xcca7c).
  Call (`bl` at 0xcca48): (unknown callee), target 0x20634, r0=`[[sp-28]]`, r1=`0x17822c`, r2=`unknown`, r3=`[sp-28]`.
  Call (`bl` at 0xcca70): (unknown callee), target 0x201fc, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xccad8): (unknown callee), target 0x530f4, r0=`unknown`, r1=`sp-52`, r2=`unknown`, r3=`sp-52`.
  Call (`bl` at 0xccb4c): (unknown callee), target 0x205bc, r0=`[sp-96]`, r1=`0x178264`, r2=`[sp-144]`, r3=`[sp-148]`.
  Call (`bl` at 0xccb54): (unknown callee), target 0x2031c, r0=`[sp-96]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`bhs` at 0xccb90): taken to 0xccee8 when `[sp-64] >= [sp-56] (unsigned, 32-bit)`, falls through to 0xccb94 when `[sp-64] < [sp-56] (unsigned, 32-bit)`.
  Call (`bl` at 0xccba4): (unknown callee), target 0x6e1b0, r0=`[sp-40]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0xccbb0): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xccbf8): (unknown callee), target 0x203f4, r0=`[sp-140]`, r1=`[sp-132]`, r2=`[sp-136]`, r3=`unknown`.
  Call (`bl` at 0xccc08): (unknown callee), target 0x58280, r0=`[sp-40]`, r1=`[sp-64]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xccc24): (unknown callee), target 0x205bc, r0=`sp-128`, r1=`0x1779ac`, r2=`ret(0x203f4)`, r3=`ret(0x58280)`.
  Branch (`beq` at 0xccc30): taken to 0xccc60 when `[sp-32] == 0x0`, falls through to 0xccc34 when `[sp-32] != 0x0`.
  Call (`bl` at 0xccc74): (unknown callee), target 0x53218, r0=`[sp-40]`, r1=`[sp-64]`, r2=`unknown`, r3=`[sp-65]:1+1`.
  Call (`bl` at 0xccc94): (unknown callee), target 0x205bc, r0=`[sp-96]`, r1=`0x1782c8`, r2=`sp-128`, r3=`sp-128`.
  Call (`bl` at 0xccc9c): (unknown callee), target 0x2031c, r0=`[sp-96]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xcccc0): (unknown callee), target 0x55c7c, r0=`[sp-40]`, r1=`[sp-64]`, r2=`unknown`, r3=`ret(0x53218)`.
  Call (`bl` at 0xccd4c): (unknown callee), target 0x205bc, r0=`[sp-96]`, r1=`0x1782e8`, r2=`sp-128`, r3=`[sp-88]`.
  Call (`bl` at 0xccd64): (unknown callee), target 0x5801c, r0=`[sp-40]`, r1=`[sp-64]`, r2=`0x0`, r3=`sp-92`.
  Call (`bl` at 0xccd80): (unknown callee), target 0x21ce0, r0=`ret(0x5801c)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x5801c)`.
  Call (`bl` at 0xccda8): (unknown callee), target 0x205bc, r0=`[sp-96]`, r1=`0x178304`, r2=`sp-128`, r3=`[sp-88]`.
  Call (`bl` at 0xccdd0): (unknown callee), target 0x205bc, r0=`[sp-96]`, r1=`0x178324`, r2=`sp-128`, r3=`[sp-88]`.
  Call (`bl` at 0xccdd8): (unknown callee), target 0x2031c, r0=`[sp-96]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xcce20): (unknown callee), target 0x205bc, r0=`[sp-96]`, r1=`0x178340`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xcce30): (unknown callee), target 0x583c4, r0=`[sp-40]`, r1=`[sp-64]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xcce4c): (unknown callee), target 0x561e4, r0=`[sp-40]`, r1=`[sp-64]`, r2=`sp-77`, r3=`sp-77`.
  Call (`bl` at 0xcce74): (unknown callee), target 0x205bc, r0=`[sp-96]`, r1=`0x17834c`, r2=`[sp-76]`, r3=`ret(0x561e4)`.
  Call (`bl` at 0xcceb8): (unknown callee), target 0x205bc, r0=`[sp-96]`, r1=`[sp-156]`, r2=`[sp-76]`, r3=`[sp-77]:1`.
  Call (`bl` at 0xccec0): (unknown callee), target 0x2031c, r0=`[sp-96]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0xccefc).

## `NL4R_SysComInt`

- `NL4R_SysComInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2e0b4 to 0x2e1e4.
  Established facts:
  Call (`bl` at 0x2e0f8): (unknown callee), target 0xb8cd4, r0=`r0`, r1=`(r1 & 0xff)`, r2=`r2`, r3=`(r1 & 0xff)`.
  Branch (`bge` at 0x2e10c): taken to 0x2e1a8 when `ret(0xb8cd4) >= 0x0 (signed, 32-bit)`, falls through to 0x2e110 when `ret(0xb8cd4) < 0x0 (signed, 32-bit)` (source `bl` at 0x2e0f8).
  Call (`bl` at 0x2e120): (unknown callee), target 0x6e1b0, r0=`[sp-20]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x2e12c): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2e174): (unknown callee), target 0x203f4, r0=`[sp-48]`, r1=`[sp-40]`, r2=`[sp-44]`, r3=`unknown`.
  Call (`bl` at 0x2e1a0): (unknown callee), target 0x22070, r0=`0x1`, r1=`0x155f2c`, r2=`0x199`, r3=`0x0`.
  Call (`bl` at 0x2e1cc): (unknown callee), target 0xb8420, r0=`[sp-20]`, r1=`ret(0xb8cd4)`, r2=`[sp-23]:1`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x2e1d4).

## `networkSerial`

- `networkSerial`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x29ff4 to 0x2a100.
  Established facts:
  Call (`bl` at 0x2a004): (unknown callee), target 0x6dc0c, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Branch (`beq` at 0x2a044): taken to 0x2a068 when `[sp-28] == 0x4f`, falls through to 0x2a048 when `[sp-28] != 0x4f`.
  Call (`bl` at 0x2a098): (unknown callee), target 0x125a58, r0=`0x0`, r1=`0x5`, r2=`0x6e`, r3=`0x44`.
  Branch (`beq` at 0x2a0ac): taken to 0x2a0e0 when `ret(0x125a58) == 0x0`, falls through to 0x2a0b0 when `ret(0x125a58) != 0x0` (source `bl` at 0x2a098).
  Call (`bl` at 0x2a0b4): (unknown callee), target 0x21ce0, r0=`ret(0x125a58)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x125a58)`.
  Call (`bl` at 0x2a0d8): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x153704`, r2=`0x90`, r3=`0x0`.
  Byte store: target `abs 0x1a5974`, value `0x1` (`strb` at 0x2a0e8).
  Recorded r0 = `unknown` (`ldm` at 0x2a0f0).

## `SerialQueryAll`

- `SerialQueryAll`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf54a8 to 0xf54e4.
  Established facts:
  Call (`bl` at 0xf54d0): (unknown callee), target 0xf4d30, r0=`unknown`, r1=`unknown`, r2=`[sp-24]`, r3=`[sp-20]`.
  Recorded r0 = `ret(0xf4d30)` (`ldm` at 0xf54e0, `bl` at 0xf54d0).
