# ARM Linux 2.6 node and front-panel state routines in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-node-frontpanel-state-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register
state from the analysis rather than a C return type.


Groups summarize static access records by printed shape. Instruction-address order can include different control-flow paths.

## `nodeCount`

- `nodeCount`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x80e40 to 0x80e60.
  Established facts:
  Decoded words in [0x80e5c, 0x80e60) are not reached from the entry by known edges.
  Word load: target `abs 0x31841c` (`ldr` at 0x80e50).
  Recorded r0 = `[0x31841c]` (`ldm` at 0x80e58).

## `nodePresent`

- `nodePresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x80e60 to 0x80ee8.
  Established facts:
  Decoded words in [0x80ee4, 0x80ee8) are not reached from the entry by known edges.
  Grouped accesses: 2 reported word load records with absolute target `0x318420`, including target `abs 0x318420` (`ldr` at 0x80e7c), target `abs 0x318420` (`ldr` at 0x80e90).
  Branch (`beq` at 0x80e84): taken to 0x80eac when `[0x318420] == 0x0`, falls through to 0x80e88 when `[0x318420] != 0x0`.
  Call (`mov` at 0x80e9c): (unknown callee), target (unknown target), r0=`(r0 & 0xff)`, r1=`r1`, r2=`(r0 & 0xff)`, r3=`[0x318420]`.
  Recorded r0 = `unknown` (`ldm` at 0x80ee0).

## `nodeEnabled`

- `nodeEnabled`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x80ee8 to 0x80f88.
  Established facts:
  Decoded words in [0x80f84, 0x80f88) are not reached from the entry by known edges.
  Branch (`bge` at 0x80f10): taken to 0x80f70 when `(r0 & 0xff) >= [0x31841c] (signed, 32-bit)`, falls through to 0x80f14 when `(r0 & 0xff) < [0x31841c] (signed, 32-bit)`.
  Call (`bl` at 0x80f1c): (unknown callee), target 0x80e60, r0=`(r0 & 0xff)`, r1=`r1`, r2=`(r0 & 0xff)`, r3=`(r0 & 0xff)`.
  Call (`bl` at 0x80f30): (unknown callee), target 0x28230, r0=`0x70`, r1=`unknown`, r2=`unknown`, r3=`ret(0x80e60)`.
  Recorded r0 = `unknown` (`ldm` at 0x80f80).

## `nodeDisable`

- `nodeDisable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x80f88 to 0x8107c.
  Established facts:
  Decoded words in [0x81078, 0x8107c) are not reached from the entry by known edges.
  Call (`bl` at 0x80fa8): (unknown callee), target 0x5572c, r0=`r0`, r1=`r1`, r2=`r2`, r3=`0x0`.
  Branch (`bne` at 0x80fcc): taken to 0x81054 when `[sp-17]:1 != 0xff`, falls through to 0x80fd0 when `[sp-17]:1 == 0xff`.
  Word load: target `abs 0x31841c` (`ldr` at 0x80fe0).
  Call (`bl` at 0x80ff4): (unknown callee), target 0x80e60, r0=`unknown`, r1=`unknown`, r2=`[0x31841c]`, r3=`unknown`.
  Call (`bl` at 0x8100c): (unknown callee), target 0x812d4, r0=`[sp-29]:1`, r1=`unknown`, r2=`unknown`, r3=`[sp-29]:1`.
  Call (`bl` at 0x8105c): (unknown callee), target 0x812d4, r0=`[sp-17]:1`, r1=`unknown`, r2=`unknown`, r3=`[sp-17]:1`.
  Recorded r0 = `unknown` (`ldm` at 0x81070).

## `nodeEnable`

- `nodeEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8107c to 0x81170.
  Established facts:
  Decoded words in [0x8116c, 0x81170) are not reached from the entry by known edges.
  Call (`bl` at 0x8109c): (unknown callee), target 0x5572c, r0=`r0`, r1=`r1`, r2=`r2`, r3=`0x0`.
  Branch (`bne` at 0x810c0): taken to 0x81148 when `[sp-17]:1 != 0xff`, falls through to 0x810c4 when `[sp-17]:1 == 0xff`.
  Word load: target `abs 0x31841c` (`ldr` at 0x810d4).
  Call (`bl` at 0x810e8): (unknown callee), target 0x80e60, r0=`unknown`, r1=`unknown`, r2=`[0x31841c]`, r3=`unknown`.
  Call (`bl` at 0x81100): (unknown callee), target 0x813a8, r0=`[sp-29]:1`, r1=`unknown`, r2=`unknown`, r3=`[sp-29]:1`.
  Call (`bl` at 0x81150): (unknown callee), target 0x813a8, r0=`[sp-17]:1`, r1=`unknown`, r2=`unknown`, r3=`[sp-17]:1`.
  Recorded r0 = `unknown` (`ldm` at 0x81164).

## `nodeOp`

- `nodeOp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x81170 to 0x812d4.
  Established facts:
  Decoded words in [0x812c4, 0x812c8) are not reached from the entry by known edges.
  Branch (`bne` at 0x8119c): taken to 0x8122c when `(r1 & 0xff) != 0xff`, falls through to 0x811a0 when `(r1 & 0xff) == 0xff`.
  Grouped accesses: 2 reported word load records with absolute target `0x31841c`, including target `abs 0x31841c` (`ldr` at 0x811b0), target `abs 0x31841c` (`ldr` at 0x81234).
  Call (`bl` at 0x811c4): (unknown callee), target 0x80ee8, r0=`unknown`, r1=`unknown`, r2=`[0x31841c]`, r3=`unknown`.
  Call (`mov` at 0x811e4): (unknown callee), target (unknown target), r0=`[sp-21]:1`, r1=`unknown`, r2=`[sp-20]`, r3=`[sp-21]:1`.
  Call (`bl` at 0x81254): (unknown callee), target 0x80ee8, r0=`(r1 & 0xff)`, r1=`r1`, r2=`[0x31841c]`, r3=`(r1 & 0xff)`.
  Call (`mov` at 0x81274): (unknown callee), target (unknown target), r0=`[sp-21]:1`, r1=`unknown`, r2=`[sp-20]`, r3=`[sp-21]:1`.
  Call (`bl` at 0x8128c): (unknown callee), target 0x80e60, r0=`[sp-21]:1`, r1=`unknown`, r2=`unknown`, r3=`[sp-21]:1`.
  Recorded r0 = `unknown` (`ldm` at 0x812c0).

## `cpuEnabled`

- `cpuEnabled`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7fe94 to 0x800e8.
  Established facts:
  Decoded words in [0x800e0, 0x800e8) are not reached from the entry by known edges.
  Call (`bl` at 0x7feb0): (unknown callee), target 0x6e190, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r1`.
  Call (`bl` at 0x7fec8): (unknown callee), target 0x3ee14, r0=`[sp-20]`, r1=`unknown`, r2=`ret(0x6e190)`, r3=`[sp-20]`.
  Call (`bl` at 0x7fedc): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3ee14)`.
  Call (`bl` at 0x7fef0): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x7ff04): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x7ff24): (unknown callee), target 0x3f474, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x7ff44): (unknown callee), target 0x3ec6c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3f474)`.
  Word load: target `abs 0x31a520` (`ldr` at 0x7ff70).
  Call (`bl` at 0x7ff94): (unknown callee), target 0x7fcdc, r0=`[sp-20]`, r1=`[sp-21]:1`, r2=`unknown`, r3=`[sp-21]:1`.
  Call (`bl` at 0x7ffb8): (unknown callee), target 0x42428, r0=`[sp-20]`, r1=`[sp-21]:1+501`, r2=`unknown`, r3=`[sp-21]:1+501`.
  Call (`bl` at 0x7ffe0): (unknown callee), target 0x3e3ac, r0=`ret(0x6e190)`, r1=`unknown`, r2=`ret(0x6e190)`, r3=`[sp-20]`.
  Call (`bl` at 0x7fffc): (unknown callee), target 0x7fcdc, r0=`[sp-20]`, r1=`[sp-21]:1`, r2=`unknown`, r3=`[sp-21]:1`.
  Call (`bl` at 0x80034): (unknown callee), target 0x42428, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Word load: target `abs 0x3183f0` (`ldr` at 0x80064).
  Call (`bl` at 0x8007c): (unknown callee), target 0x7fcdc, r0=`[sp-20]`, r1=`[sp-21]:1`, r2=`[0x3183f0]`, r3=`[sp-21]:1`.
  Call (`bl` at 0x80098): (unknown callee), target 0x28230, r0=`[sp-21]:1+88`, r1=`unknown`, r2=`unknown`, r3=`[sp-21]:1+88`.
  Recorded r0 = `unknown` (`ldm` at 0x800d0).

## `IP57_NodeRemove`

- `IP57_NodeRemove`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82a38 to 0x82b48.
  Established facts:
  Decoded words in [0x82b44, 0x82b48) are not reached from the entry by known edges.
  Call (`bl` at 0x82a60): (unknown callee), target 0x84274, r0=`(r0 & 0xff)`, r1=`sp-24`, r2=`sp-24`, r3=`(r0 & 0xff)`.
  Call (`bl` at 0x82a94): (unknown callee), target 0x82cb0, r0=`[sp-17]:1`, r1=`0x1`, r2=`0x1`, r3=`[sp-17]:1`.
  Grouped accesses: 4 reported byte load records with printed base expression `[sp-24]` and nonnegative byte-aligned offsets, including target `[sp-24] +0x3` (`ldrb` at 0x82a9c), target `[sp-24] +0x3` (`ldrb` at 0x82aac), target `[sp-24] +0x0` (`ldrb` at 0x82ae0).
  Call (`bl` at 0x82ac8): (unknown callee), target 0x11fbc0, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x82b0c): (unknown callee), target 0x12315c, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Byte store: target `[sp-17]:1 +0x31a724`, value `0xf` (`strb` at 0x82b2c).
  Recorded r0 = `unknown` (`ldm` at 0x82b40).

## `IP57_NodePresent`

- `IP57_NodePresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x830e0 to 0x8311c.
  Established facts:
  Decoded words in [0x83118, 0x8311c) are not reached from the entry by known edges.
  Byte load: target `(r0 & 0xff) +0x31a724` (`ldrb` at 0x83100).
  Recorded r0 = `unknown` (`ldm` at 0x83114).

## `IP57_NodeEnable`

- `IP57_NodeEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82b80 to 0x82bb4.
  Established facts:
  Call (`bl` at 0x82ba4): (unknown callee), target 0x82e6c, r0=`(r0 & 0xff)`, r1=`0x0`, r2=`r2`, r3=`(r0 & 0xff)`.
  Recorded r0 = `ret(0x82e6c)` (`ldm` at 0x82bb0, `bl` at 0x82ba4).

## `IP57_NodeDisable`

- `IP57_NodeDisable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82b48 to 0x82b80.
  Established facts:
  Call (`bl` at 0x82b70): (unknown callee), target 0x82cb0, r0=`(r0 & 0xff)`, r1=`0x0`, r2=`0x1`, r3=`(r0 & 0xff)`.
  Recorded r0 = `ret(0x82cb0)` (`ldm` at 0x82b7c, `bl` at 0x82b70).

## `frontpanelLEDInit`

- `frontpanelLEDInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x71b94 to 0x71c20.
  Established facts:
  Decoded words in [0x71c10, 0x71c20) are not reached from the entry by known edges.
  Branch (`beq` at 0x71bb8): taken to 0x71bf0 when `[0x1a61f4] == 0x0`, falls through to 0x71bbc when `[0x1a61f4] != 0x0`.
  Call (`bl` at 0x71bec): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x165318`, r2=`0x3a`, r3=`0x0`.
  Word store: target `abs 0x1a61f4`, value `unknown` (`str` at 0x71bf8).
  Word store: target `abs 0x1a61f8`, value `[sp-24]` (`str` at 0x71c04).
  Recorded r0 = `unknown` (`ldm` at 0x71c0c).

## `frontpanelLEDOn`

- `frontpanelLEDOn`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x71c20 to 0x71c90.
  Established facts:
  Decoded words in [0x71c8c, 0x71c90) are not reached from the entry by known edges.
  Branch (`bne` at 0x71c40): taken to 0x71c50 when `r0 != 0x4`, falls through to 0x71c44 when `r0 == 0x4`.
  Word store: target `abs 0x1a61e8`, value `0x0` (`str` at 0x71c58).
  Call (`bl` at 0x71c6c): (unknown callee), target 0x71c90, r0=`0xff`, r1=`r1`, r2=`0x0`, r3=`[sp-24]`.
  Call (`bl` at 0x71c74): (unknown callee), target 0x71f28, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x71c88).

## `frontpanelLEDOff`

- `frontpanelLEDOff`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x71c90 to 0x71ce8.
  Established facts:
  Decoded words in [0x71ce4, 0x71ce8) are not reached from the entry by known edges.
  Branch (`bne` at 0x71cac): taken to 0x71cbc when `r0 != 0x4`, falls through to 0x71cb0 when `r0 == 0x4`.
  Word store: target `abs 0x1a61e8`, value `0x0` (`str` at 0x71cc4).
  Call (`bl` at 0x71ccc): (unknown callee), target 0x72038, r0=`[sp-20]`, r1=`r1`, r2=`0x0`, r3=`0x1a61e8`.
  Recorded r0 = `unknown` (`ldm` at 0x71ce0).

## `frontpanelLEDBlink`

- `frontpanelLEDBlink`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x71ce8 to 0x71dd4.
  Established facts:
  Decoded words in [0x71dbc, 0x71dd4) are not reached from the entry by known edges.
  Call (`bl` at 0x71d14): (unknown callee), target 0x71c90, r0=`0xff`, r1=`r1`, r2=`r2`, r3=`r2`.
  Word store: target `abs 0x1a61e8`, value `0x1` (`str` at 0x71d20).
  Byte store: target `abs 0x1a61fc`, value `([sp-20] & 0xff)` (`strb` at 0x71d2c).
  Grouped accesses: 2 reported byte store records with absolute target `0x1a61fd`, including target `abs 0x1a61fd`, value `([[0x1a61f4]+2]:1 & 0xff)` (`strb` at 0x71d50), target `abs 0x1a61fd`, value `([sp-20] & 0xff)` (`strb` at 0x71da4).
  Grouped accesses: 2 reported byte store records with absolute target `0x1a61ed`, including target `abs 0x1a61ed`, value `0x1` (`strb` at 0x71d5c), target `abs 0x1a61ed`, value `0x0` (`strb` at 0x71d98).
  Recorded r0 = `0x0` (`ldm` at 0x71db8).
