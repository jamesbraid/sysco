# ARM Linux 2.6 serial interrupt control routines in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-serial-interrupt-controls-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register
state from the analysis rather than a C return type.


Groups summarize static access records by printed shape. Instruction-address order can include different control-flow paths.

## `serIntInit`

- `serIntInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10c524 to 0x10c774.
  Established facts:
  Grouped accesses: 5 reported word load records with absolute target `0x314f2c`, including target `abs 0x314f2c` (`ldr` at 0x10c54c), target `abs 0x314f2c` (`ldr` at 0x10c564), target `abs 0x314f2c` (`ldr` at 0x10c63c).
  Call (`bl` at 0x10c57c): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x18ce30`, r2=`0x80`, r3=`0x0`.
  Call (`bl` at 0x10c588): (unknown callee), target 0x2046c, r0=`0x18ce3c`, r1=`0x1002`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x10c5a0): (unknown callee), target 0x20184, r0=`ret(0x2046c)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x2046c)`.
  Call (`bl` at 0x10c5bc): (unknown callee), target 0x244d4, r0=`[0x1a288c]`, r1=`0x18ce48`, r2=`[ret(0x20184)]`, r3=`0x1a288c`.
  Call (`bl` at 0x10c5c4): (unknown callee), target 0x204b4, r0=`0x1`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x10c5ec): (unknown callee), target 0x2016c, r0=`0x0`, r1=`0x100`, r2=`0x3`, r3=`0x1`.
  Call (`bl` at 0x10c600): (unknown callee), target 0x20184, r0=`ret(0x2016c)`, r1=`unknown`, r2=`unknown`, r3=`[0x314f34]`.
  Call (`bl` at 0x10c61c): (unknown callee), target 0x244d4, r0=`[0x1a288c]`, r1=`0x18ce70`, r2=`[ret(0x20184)]`, r3=`0x1a288c`.
  Call (`bl` at 0x10c624): (unknown callee), target 0x204b4, r0=`0x1`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Word store: target `abs 0x314f2c`, value `[sp-22]:1` (`str` at 0x10c630).
  Call (`bl` at 0x10c644): (unknown callee), target 0x204d8, r0=`[0x314f2c]`, r1=`0x38`, r2=`0x314f2c`, r3=`0x314f2c`.
  Branch (`bne` at 0x10c654): taken to 0x10c690 when `[0x314f28] != 0x0`, falls through to 0x10c658 when `[0x314f28] == 0x0`.
  Call (`bl` at 0x10c68c): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x18ce30`, r2=`0x9b`, r3=`0x0`.
  Call (`bl` at 0x10c6f0): (unknown callee), target 0x205bc, r0=`[sp-36]+8`, r1=`0x18cee0`, r2=`[sp-28]`, r3=`[sp-36]+8`.
  Call (`bl` at 0x10c6fc): (unknown callee), target 0x10d950, r0=`[sp-36]`, r1=`0x0`, r2=`unknown`, r3=`unknown`.
  Byte store: target `abs 0x314f30`, value `([sp-21]:1 & 0xff)` (`strb` at 0x10c718).
  Call (`bl` at 0x10c730): (unknown callee), target 0x10b4e8, r0=`0x10db2c`, r1=`[sp-21]:1`, r2=`0x0`, r3=`0x0`.
  Recorded r0 = `ret(0x10b4e8)` (`ldm` at 0x10c738, `bl` at 0x10c730).

## `serIntRegister`

- `serIntRegister`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10c774 to 0x10c91c.
  Established facts:
  Call (`bl` at 0x10c7dc): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x18ce30`, r2=`0xc2`, r3=`0x0`.
  Grouped accesses: 2 reported word store records with printed base expression `[sp-36]` and nonnegative word-aligned offsets, including target `[sp-36] +0xc`, value `[sp-24]` (`str` at 0x10c80c), target `[sp-36] +0x34`, value `ret(0x204d8)` (`str` at 0x10c874) (source `bl` at 0x10c870).
  Grouped accesses: 2 reported halfword store records with printed base expression `[sp-36]` and nonnegative halfword-aligned offsets, including target `[sp-36] +0x4`, value `([sp-28]:2 & 0xffff)` (`strh` at 0x10c818), target `[sp-36] +0x6`, value `([sp-30]:2 & 0xffff)` (`strh` at 0x10c858).
  Call (`bl` at 0x10c870): (unknown callee), target 0x204d8, r0=`[[sp-36]+2]:1`, r1=`0x8`, r2=`[sp-36]`, r3=`[[sp-36]+2]:1`.
  Call (`bl` at 0x10c8b8): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x18ce30`, r2=`0xd4`, r3=`0x0`.
  Halfword store: target `(unknown effective address)`, value `unknown` (`strh` at 0x10c8ec).
  Call (`bl` at 0x10c8f8): (unknown callee), target 0x10d950, r0=`[sp-36]`, r1=`0x1`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `ret(0x10d950)` (`ldm` at 0x10c900, `bl` at 0x10c8f8).

## `serIntSetNumBits`

- `serIntSetNumBits`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10c91c to 0x10c9f0.
  Established facts:
  Branch (`blt` at 0x10c948): taken to 0x10c970 when `(r0 & 0xff) < [0x314f2c] (signed, 32-bit)`, falls through to 0x10c94c when `(r0 & 0xff) >= [0x314f2c] (signed, 32-bit)`.
  Call (`bl` at 0x10c96c): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x18ce30`, r2=`0xee`, r3=`0x0`.
  Byte store: target `[sp-28] +0x2`, value `([sp-24] & 0xff)` (`strb` at 0x10c99c).
  Halfword store: target `(unknown effective address)`, value `unknown` (`strh` at 0x10c9d0).
  Recorded r0 = `unknown` (`ldm` at 0x10c9d8).

## `serIntRemove`

- `serIntRemove`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10c9f0 to 0x10ca8c.
  Established facts:
  Call (`bl` at 0x10ca10): (unknown callee), target 0x10ca8c, r0=`(r0 & 0xff)`, r1=`r1`, r2=`r2`, r3=`(r0 & 0xff)`.
  Branch (`beq` at 0x10ca1c): taken to 0x10ca80 when `ret(0x10ca8c) == 0x0`, falls through to 0x10ca20 when `ret(0x10ca8c) != 0x0` (source `bl` at 0x10ca10).
  Call (`bl` at 0x10ca4c): (unknown callee), target 0x10d950, r0=`[sp-24]`, r1=`0x0`, r2=`unknown`, r3=`unknown`.
  Word store: target `[sp-24] +0xc`, value `0x0` (`str` at 0x10ca58).
  Grouped accesses: 2 reported halfword store records with printed base expression `[sp-24]` and nonnegative halfword-aligned offsets, including target `[sp-24] +0x4`, value `0x0` (`strh` at 0x10ca64), target `[sp-24] +0x6`, value `0x0` (`strh` at 0x10ca7c).
  Byte store: target `[sp-24] +0x2`, value `0x0` (`strb` at 0x10ca70).
  Recorded r0 = `unknown` (`ldm` at 0x10ca84).

## `serIntIsRegistered`

- `serIntIsRegistered`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10ca8c to 0x10cb0c.
  Established facts:
  Branch (`bge` at 0x10cab4): taken to 0x10cae4 when `(r0 & 0xff) >= [0x314f2c] (signed, 32-bit)`, falls through to 0x10cab8 when `(r0 & 0xff) < [0x314f2c] (signed, 32-bit)`.
  Word load: target `(unknown effective address)` (`ldr` at 0x10cad8).
  Branch (`bne` at 0x10cae0): taken to 0x10caf0 when `? != 0x0`, falls through to 0x10cae4 when `? == 0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x10cb00).

## `serIntCount`

- `serIntCount`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10cb0c to 0x10cb9c.
  Established facts:
  Word load: target `abs 0x314f2c` (`ldr` at 0x10cb2c).
  Call (`bl` at 0x10cb58): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x18ce30`, r2=`0x124`, r3=`0x0`.
  Word load: target `abs 0x314f28` (`ldr` at 0x10cb74).
  Word load: target `(unknown effective address)` (`ldr` at 0x10cb7c).
  Recorded r0 = `unknown` (`ldm` at 0x10cb88).

## `serIntBitCount`

- `serIntBitCount`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10cb9c to 0x10cc94.
  Established facts:
  Call (`bl` at 0x10cbec): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x18ce30`, r2=`0x131`, r3=`0x0`.
  Byte load: target `[sp-28] +0x2` (`ldrb` at 0x10cc18).
  Branch (`bgt` at 0x10cc24): taken to 0x10cc54 when `[[sp-28]+2]:1 > [sp-24] (signed, 32-bit)`, falls through to 0x10cc28 when `[[sp-28]+2]:1 <= [sp-24] (signed, 32-bit)`.
  Call (`bl` at 0x10cc50): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x18ce30`, r2=`0x137`, r3=`0x0`.
  Word load: target `[sp-28] +0x34` (`ldr` at 0x10cc60).
  Word load: target `(unknown effective address)` (`ldr` at 0x10cc68).
  Recorded r0 = `unknown` (`ldm` at 0x10cc74).

## `serIntMaxInts`

- `serIntMaxInts`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10cc94 to 0x10ccb4.
  Established facts:
  Word load: target `abs 0x314f2c` (`ldr` at 0x10cca4).
  Recorded r0 = `[0x314f2c]` (`ldm` at 0x10ccac).

## `serIntIgnoreSpurious`

- `serIntIgnoreSpurious`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10ccb4 to 0x10cd78.
  Established facts:
  Call (`bl` at 0x10cd04): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x18ce30`, r2=`0x149`, r3=`0x0`.
  Grouped accesses: 2 reported word load records with absolute target `0x314f28`, including target `abs 0x314f28` (`ldr` at 0x10cd20), target `abs 0x314f28` (`ldr` at 0x10cd40).
  Byte load: target `(unknown effective address)` (`ldrb` at 0x10cd4c).
  Byte store: target `(unknown effective address)`, value `unknown` (`strb` at 0x10cd58).
  Recorded r0 = `unknown` (`ldm` at 0x10cd60).

## `serIntBitIgnoreSpurious`

- `serIntBitIgnoreSpurious`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10cd78 to 0x10ce94.
  Established facts:
  Call (`bl` at 0x10cdcc): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x18ce30`, r2=`0x155`, r3=`0x0`.
  Branch (`bgt` at 0x10ce04): taken to 0x10ce34 when `[[sp-32]+2]:1 > [sp-24] (signed, 32-bit)`, falls through to 0x10ce08 when `[[sp-32]+2]:1 <= [sp-24] (signed, 32-bit)`.
  Call (`bl` at 0x10ce30): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x18ce30`, r2=`0x15b`, r3=`0x0`.
  Grouped accesses: 2 reported word load records with printed base expression `[sp-32]` and nonnegative word-aligned offsets, including target `[sp-32] +0x34` (`ldr` at 0x10ce40), target `[sp-32] +0x34` (`ldr` at 0x10ce54).
  Byte load: target `(unknown effective address)` (`ldrb` at 0x10ce60).
  Byte store: target `(unknown effective address)`, value `unknown` (`strb` at 0x10ce6c).
  Recorded r0 = `unknown` (`ldm` at 0x10ce74).

## `serIntQuery`

- `serIntQuery`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10ce94 to 0x10d17c.
  Established facts:
  Branch (`blt` at 0x10cecc): taken to 0x10cef0 when `(r0 & 0xff) < [0x314f2c] (signed, 32-bit)`, falls through to 0x10ced0 when `(r0 & 0xff) >= [0x314f2c] (signed, 32-bit)`.
  Call (`bl` at 0x10cee0): (unknown callee), target 0x205bc, r0=`0x314758`, r1=`0x18d11c`, r2=`(r0 & 0xff)`, r3=`(r0 & 0xff)`.
  Byte store: target `abs 0x314758`, value `0x0` (`strb` at 0x10cef8).
  Grouped accesses: 4 reported byte load records with printed base expression `[sp-36]` and nonnegative byte-aligned offsets, including target `[sp-36] +0x2` (`ldrb` at 0x10cf24), target `[sp-36] +0x0` (`ldrb` at 0x10cf34), target `[sp-36] +0x1` (`ldrb` at 0x10cf84).
  Call (`bl` at 0x10cf54): (unknown callee), target 0x205bc, r0=`[sp-32]`, r1=`0x18d13c`, r2=`[sp-24]`, r3=`[[sp-36]]:1`.
  Call (`bl` at 0x10cf5c): (unknown callee), target 0x2031c, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x10cfdc): (unknown callee), target 0x205bc, r0=`([sp-32] + ret(0x2031c))`, r1=`0x18d158`, r2=`[sp-56]`, r3=`[[sp-36]+32]`.
  Call (`bl` at 0x10cfec): (unknown callee), target 0x203c4, r0=`([sp-32] + ret(0x2031c))`, r1=`0x18d17c`, r2=`ret(0x2031c)`, r3=`[sp-28]`.
  Call (`bl` at 0x10cff4): (unknown callee), target 0x2031c, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x10d06c): (unknown callee), target 0x205bc, r0=`[sp-32]`, r1=`0x18d180`, r2=`[sp-24]`, r3=`unknown`.
  Call (`bl` at 0x10d074): (unknown callee), target 0x2031c, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x10d0fc): (unknown callee), target 0x205bc, r0=`([sp-32] + ret(0x2031c))`, r1=`0x18d194`, r2=`[sp-60]`, r3=`[[sp-48]+1]:1`.
  Call (`bl` at 0x10d10c): (unknown callee), target 0x203c4, r0=`([sp-32] + ret(0x2031c))`, r1=`0x18d17c`, r2=`ret(0x2031c)`, r3=`[sp-28]`.
  Call (`bl` at 0x10d114): (unknown callee), target 0x2031c, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `0x314758` (`ldm` at 0x10d14c).

## `serIntEnable`

- `serIntEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10d17c to 0x10d214.
  Established facts:
  Branch (`blt` at 0x10d1a8): taken to 0x10d1d0 when `(r0 & 0xff) < [0x314f2c] (signed, 32-bit)`, falls through to 0x10d1ac when `(r0 & 0xff) >= [0x314f2c] (signed, 32-bit)`.
  Call (`bl` at 0x10d1cc): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x18ce30`, r2=`0x1a0`, r3=`0x0`.
  Word load: target `abs 0x314f28` (`ldr` at 0x10d1e8).
  Call (`bl` at 0x10d1f8): (unknown callee), target 0x10d950, r0=`unknown`, r1=`[sp-24]`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `ret(0x10d950)` (`ldm` at 0x10d200, `bl` at 0x10d1f8).

## `serIntSetMask`

- `serIntSetMask`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10d214 to 0x10d2bc.
  Established facts:
  Branch (`blt` at 0x10d244): taken to 0x10d26c when `(r0 & 0xff) < [0x314f2c] (signed, 32-bit)`, falls through to 0x10d248 when `(r0 & 0xff) >= [0x314f2c] (signed, 32-bit)`.
  Call (`bl` at 0x10d268): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x18ce30`, r2=`0x1ab`, r3=`0x0`.
  Word load: target `abs 0x314f28` (`ldr` at 0x10d284).
  Call (`bl` at 0x10d29c): (unknown callee), target 0x10d6f4, r0=`unknown`, r1=`[sp-20]:2`, r2=`0x1`, r3=`unknown`.
  Recorded r0 = `ret(0x10d6f4)` (`ldm` at 0x10d2a4, `bl` at 0x10d29c).

## `serIntClearMask`

- `serIntClearMask`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10d2bc to 0x10d364.
  Established facts:
  Word load: target `abs 0x314f2c` (`ldr` at 0x10d2e4).
  Call (`bl` at 0x10d310): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x18ce30`, r2=`0x1b7`, r3=`0x0`.
  Word load: target `abs 0x314f28` (`ldr` at 0x10d32c).
  Call (`bl` at 0x10d344): (unknown callee), target 0x10d6f4, r0=`unknown`, r1=`[sp-20]:2`, r2=`0x0`, r3=`unknown`.
  Recorded r0 = `ret(0x10d6f4)` (`ldm` at 0x10d34c, `bl` at 0x10d344).

## `serIntGetMask`

- `serIntGetMask`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10d364 to 0x10d408.
  Established facts:
  Branch (`blt` at 0x10d390): taken to 0x10d3b8 when `(r0 & 0xff) < [0x314f2c] (signed, 32-bit)`, falls through to 0x10d394 when `(r0 & 0xff) >= [0x314f2c] (signed, 32-bit)`.
  Call (`bl` at 0x10d3b4): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x18ce30`, r2=`0x1c5`, r3=`0x0`.
  Halfword load: target `[sp-28] +0x6` (`ldrh` at 0x10d3e4).
  Halfword store: target `[sp-24] +0x0`, value `([[sp-28]+6]:2 & 0xffff)` (`strh` at 0x10d3e8).
  Recorded r0 = `unknown` (`ldm` at 0x10d3f0).
