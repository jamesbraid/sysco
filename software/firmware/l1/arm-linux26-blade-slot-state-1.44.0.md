# ARM Linux 2.6 blade slot and state routines in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-blade-slot-state-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register
state from the analysis rather than a C return type.

## `bladeSlotType`

- `bladeSlotType`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3de6c to 0x3deec.
  Established facts:
  Branch (`blt` at 0x3de88): taken to 0x3dea4 when `r0 < 0x0 (signed, 32-bit)`, falls through to 0x3de8c when `r0 >= 0x0 (signed, 32-bit)`.
  Word load: target `abs 0x19e6e0` (`ldr` at 0x3de94).
  Branch (`bge` at 0x3de9c): taken to 0x3dea4 when `r0 >= [0x19e6e0] (signed, 32-bit)`, falls through to 0x3dea0 when `r0 < [0x19e6e0] (signed, 32-bit)`.
  Recorded r0 = `unknown` (`ldm` at 0x3dee0).

## `bladeGetType`

- `bladeGetType`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3deec to 0x3dff8.
  Established facts:
  Call (`bl` at 0x3df00): (unknown callee), target 0x6e190, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0x3df24): (unknown callee), target 0x3ee14, r0=`[sp-20]`, r1=`unknown`, r2=`ret(0x6e190)`, r3=`[sp-20]`.
  Branch (`bgt` at 0x3df30): taken to 0x3dfbc when `ret(0x3ee14) > 0x0 (signed, 32-bit)`, falls through to 0x3df34 when `ret(0x3ee14) <= 0x0 (signed, 32-bit)` (source `bl` at 0x3df24).
  Call (`bl` at 0x3df38): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3ee14)`.
  Call (`bl` at 0x3df4c): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x3df60): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x3df80): (unknown callee), target 0x3f474, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x3dfa0): (unknown callee), target 0x3ec6c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3f474)`.
  Word load: target `abs 0x31a520` (`ldr` at 0x3dfc8).
  Halfword load: target `(unknown effective address)` (`ldrh` at 0x3dfd4).
  Recorded r0 = `unknown` (`ldm` at 0x3dfe4).

## `bladeGetSubType`

- `bladeGetSubType`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3dff8 to 0x3e104.
  Established facts:
  Call (`bl` at 0x3e00c): (unknown callee), target 0x6e190, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0x3e030): (unknown callee), target 0x3ee14, r0=`[sp-20]`, r1=`unknown`, r2=`ret(0x6e190)`, r3=`[sp-20]`.
  Branch (`bgt` at 0x3e03c): taken to 0x3e0c8 when `ret(0x3ee14) > 0x0 (signed, 32-bit)`, falls through to 0x3e040 when `ret(0x3ee14) <= 0x0 (signed, 32-bit)` (source `bl` at 0x3e030).
  Call (`bl` at 0x3e044): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3ee14)`.
  Call (`bl` at 0x3e058): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x3e06c): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x3e08c): (unknown callee), target 0x3f474, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x3e0ac): (unknown callee), target 0x3ec6c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3f474)`.
  Word load: target `abs 0x31a520` (`ldr` at 0x3e0d4).
  Halfword load: target `(unknown effective address)` (`ldrh` at 0x3e0e0).
  Recorded r0 = `unknown` (`ldm` at 0x3e0f0).

## `bladeGetClass`

- `bladeGetClass`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e104 to 0x3e210.
  Established facts:
  Call (`bl` at 0x3e118): (unknown callee), target 0x6e190, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0x3e13c): (unknown callee), target 0x3ee14, r0=`[sp-20]`, r1=`unknown`, r2=`ret(0x6e190)`, r3=`[sp-20]`.
  Branch (`bgt` at 0x3e148): taken to 0x3e1d4 when `ret(0x3ee14) > 0x0 (signed, 32-bit)`, falls through to 0x3e14c when `ret(0x3ee14) <= 0x0 (signed, 32-bit)` (source `bl` at 0x3e13c).
  Call (`bl` at 0x3e150): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3ee14)`.
  Call (`bl` at 0x3e164): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x3e178): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x3e198): (unknown callee), target 0x3f474, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x3e1b8): (unknown callee), target 0x3ec6c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3f474)`.
  Word load: target `abs 0x31a520` (`ldr` at 0x3e1e0).
  Halfword load: target `(unknown effective address)` (`ldrh` at 0x3e1ec).
  Recorded r0 = `unknown` (`ldm` at 0x3e1fc).

## `bladeSingleSlotHack`

- `bladeSingleSlotHack`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e210 to 0x3e274.
  Established facts:
  Word store: target `abs 0x1a5aa0`, value `0x1` (`str` at 0x3e234).
  Word store: target `abs 0x1a5aa4`, value `[sp-28]` (`str` at 0x3e240).
  Word store: target `abs 0x1a5aa8`, value `[sp-20]` (`str` at 0x3e24c).
  Word store: target `abs 0x1a5aac`, value `[sp-24]` (`str` at 0x3e258).
  Recorded r0 = `r0` (`ldm` at 0x3e260).

## `bladeCheckState`

- `bladeCheckState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e2d0 to 0x3e38c.
  Established facts:
  Call (`bl` at 0x3e2e8): (unknown callee), target 0x3e58c, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Branch (`bge` at 0x3e2f4): taken to 0x3e30c when `ret(0x3e58c) >= 0x0 (signed, 32-bit)`, falls through to 0x3e2f8 when `ret(0x3e58c) < 0x0 (signed, 32-bit)`.
  Call (`bl` at 0x3e2fc): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x3e310): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x3e330): (unknown callee), target 0x3f474, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x3e350): (unknown callee), target 0x3ec6c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3f474)`.
  Branch (`bne` at 0x3e35c): taken to 0x3e36c when `ret(0x3ec6c) != 0x0`, falls through to 0x3e360 when `ret(0x3ec6c) == 0x0` (source `bl` at 0x3e350).
  Recorded r0 = `unknown` (`ldm` at 0x3e37c).

## `bladeNumSlots`

- `bladeNumSlots`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e274 to 0x3e2d0.
  Established facts:
  Branch (`beq` at 0x3e290): taken to 0x3e2b0 when `[0x1a5aa0] == 0x0`, falls through to 0x3e294 when `[0x1a5aa0] != 0x0`.
  Branch (`bne` at 0x3e2a0): taken to 0x3e2b0 when `[0x1a5aa4] != 0x0`, falls through to 0x3e2a4 when `[0x1a5aa4] == 0x0`.
  Word load: target `abs 0x19e6e0` (`ldr` at 0x3e2b4).
  Recorded r0 = `unknown` (`ldm` at 0x3e2c0).

## `bladeIsSSHack`

- `bladeIsSSHack`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e38c to 0x3e3ac.
  Established facts:
  Recorded r0 = `[0x1a5aa0]` (`ldm` at 0x3e3a4).

## `bladeRealNumSlots`

- `bladeRealNumSlots`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e3ac to 0x3e3cc.
  Established facts:
  Recorded r0 = `[0x19e6e0]` (`ldm` at 0x3e3c4).

## `bladeSSHackSlot`

- `bladeSSHackSlot`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e458 to 0x3e498.
  Established facts:
  Call (`bl` at 0x3e484): (unknown callee), target 0x3e3cc, r0=`r0`, r1=`sp-24`, r2=`sp-28`, r3=`sp-32`.
  Recorded r0 = `[sp-24]` (`ldm` at 0x3e494).

## `bladeSSHackSlab`

- `bladeSSHackSlab`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e498 to 0x3e4d8.
  Established facts:
  Call (`bl` at 0x3e4c4): (unknown callee), target 0x3e3cc, r0=`r0`, r1=`sp-24`, r2=`sp-28`, r3=`sp-32`.
  Recorded r0 = `[sp-28]` (`ldm` at 0x3e4d4).

## `bladeSSHackMaxSlabs`

- `bladeSSHackMaxSlabs`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e4d8 to 0x3e514.
  Established facts:
  Call (`bl` at 0x3e500): (unknown callee), target 0x3e3cc, r0=`0x0`, r1=`sp-20`, r2=`sp-24`, r3=`sp-28`.
  Recorded r0 = `[sp-28]` (`ldm` at 0x3e510).

## `bladePresent`

- `bladePresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e514 to 0x3e58c.
  Established facts:
  Branch (`blt` at 0x3e538): taken to 0x3e548 when `r0 < [0x19e6e0] (signed, 32-bit)`, falls through to 0x3e53c when `r0 >= [0x19e6e0] (signed, 32-bit)`.
  Byte store: target `(unknown effective address)`, value `0xb` (`strb` at 0x3e56c).
  Recorded r0 = `[sp-24]` (`ldm` at 0x3e580).

## `bladeIsPresent`

- `bladeIsPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e58c to 0x3e6b0.
  Established facts:
  Call (`bl` at 0x3e5a0): (unknown callee), target 0x3e38c, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0x3e5b0): (unknown callee), target 0x6e190, r0=`ret(0x3e38c)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e38c)`.
  Branch (`blt` at 0x3e5e0): taken to 0x3e5f0 when `[sp-20] < [0x19e6e0] (signed, 32-bit)`, falls through to 0x3e5e4 when `[sp-20] >= [0x19e6e0] (signed, 32-bit)`.
  Word load: target `abs 0x1a5aa0` (`ldr` at 0x3e5f4).
  Branch (`beq` at 0x3e5fc): taken to 0x3e664 when `[0x1a5aa0] == 0x0`, falls through to 0x3e600 when `[0x1a5aa0] != 0x0`.
  Recorded r0 = `[sp-24]` (`ldm` at 0x3e6a0).

## `bladePresentMask`

- `bladePresentMask`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e6b0 to 0x3e734.
  Established facts:
  Word load: target `abs 0x19e6e0` (`ldr` at 0x3e6d8).
  Call (`bl` at 0x3e6e8): (unknown callee), target 0x3e58c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[0x19e6e0]`.
  Branch (`beq` at 0x3e6f4): taken to 0x3e710 when `ret(0x3e58c) == 0x0`, falls through to 0x3e6f8 when `ret(0x3e58c) != 0x0` (source `bl` at 0x3e6e8).
  Recorded r0 = `unknown` (`ldm` at 0x3e72c).
