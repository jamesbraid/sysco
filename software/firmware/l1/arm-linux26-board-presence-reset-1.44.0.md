# ARM Linux 2.6 board presence, reset and field helpers in 1.44.0

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
Computed transfers with unresolved edges are identified under their routines. Effects on omitted paths remain unknown.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-board-presence-reset-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register
state from the analysis rather than a C return type.


Groups summarize static access records by printed shape. Instruction-address order can include different control-flow paths.

## `QDRTR_BrickOpsInit`

- `QDRTR_BrickOpsInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4d354 to 0x4d420.
  Established facts:
  Word store: target `r0 +0x0`, value `0x4d074` (`str` at 0x4d378).
  Grouped accesses: 10 reported word store records with printed base expression `[sp-20]` and nonnegative word-aligned offsets, including target `[sp-20] +0x4`, value `0x4cf04` (`str` at 0x4d384), target `[sp-20] +0x14`, value `0x0` (`str` at 0x4d39c), target `[sp-20] +0x16c`, value `0x38a40` (`str` at 0x4d3a8).
  Byte store: target `[sp-20] +0x10`, value `0x0` (`strb` at 0x4d390).
  Recorded r0 = `r0` (`ldm` at 0x4d404).

## `IP67_BrickOpsInit`

- `IP67_BrickOpsInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3be48 to 0x3c040.
  Established facts:
  Grouped accesses: 29 reported word store records with printed base expression `[sp-20]` and nonnegative word-aligned offsets, including target `[sp-20] +0x4`, value `0x3bc50` (`str` at 0x3be78), target `[sp-20] +0x14`, value `0x0` (`str` at 0x3be90), target `[sp-20] +0x110`, value `0x9` (`str` at 0x3be9c).
  Byte store: target `[sp-20] +0x10`, value `0x54` (`strb` at 0x3be84).
  Call (`bl` at 0x3bf00): (unknown callee), target 0x203f4, r0=`0x318314`, r1=`0x159d0c`, r2=`0x10`, r3=`0x3b3d8`.
  Recorded r0 = `ret(0x203f4)` (`ldm` at 0x3bfec, `bl` at 0x3bf00).

## `Unknown_BrickOpsInit`

- `Unknown_BrickOpsInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x29eac to 0x29f9c.
  Established facts:
  Grouped accesses: 14 reported word store records with printed base expression `[sp-20]` and nonnegative word-aligned offsets, including target `[sp-20] +0x4`, value `0x29e14` (`str` at 0x29edc), target `[sp-20] +0x14`, value `0x0` (`str` at 0x29ef4), target `[sp-20] +0x110`, value `0xff` (`str` at 0x29f00).
  Byte store: target `[sp-20] +0x10`, value `0x4e` (`strb` at 0x29ee8).
  Recorded r0 = `r0` (`ldm` at 0x29f8c).

## `brickInitBrickOps`

- `brickInitBrickOps`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x20d08 to 0x20f8c.
  Established facts:
  Branch (`ldrls` at 0x20d2c): taken to (unknown target) when `r1-36 <= 0x59 (unsigned, 32-bit)`, falls through to 0x20d30 when `r1-36 > 0x59 (unsigned, 32-bit)` (source `sub` at 0x20d24).
  Omitted-path effects remain unknown at the unresolved transfer (`ldrls` at 0x20d2c).
  Call (`bl` at 0x20f64): (unknown callee), target 0x29eac, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r1-36`.
  Call (`bl` at 0x20f68): (unknown callee), target 0x129b0c, r0=`ret(0x29eac)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x20f6c): (unknown callee), target 0x139344, r0=`ret(0x129b0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x20f70): (unknown callee), target 0x146cd0, r0=`ret(0x139344)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x20f74): (unknown callee), target 0x13b26c, r0=`ret(0x146cd0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x20f78): (unknown callee), target 0xe05b4, r0=`ret(0x13b26c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x20f7c): (unknown callee), target 0xe56fc, r0=`ret(0xe05b4)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x20f80): (unknown callee), target 0xeef1c, r0=`ret(0xe56fc)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Known-edge recorded r0 = `ret(0xeef1c)` (`ldm` at 0x20f88, `bl` at 0x20f80).

## `ip69BladeIsPresent`

- `ip69BladeIsPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4443c to 0x4446c.
  Established facts:
  Byte load: target `(r0 & 0xff) +0x31a524` (`ldrb` at 0x4445c).
  Recorded r0 = `[(r0 & 0xff)+3253540]:1` (`ldm` at 0x44464).

## `ip69BladeNotPresent`

- `ip69BladeNotPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4440c to 0x4443c.
  Established facts:
  Byte store: target `(r0 & 0xff) +0x31a524`, value `0x0` (`strb` at 0x44430).
  Recorded r0 = `r0` (`ldm` at 0x44434).

## `ip69BladePresent`

- `ip69BladePresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x443dc to 0x4440c.
  Established facts:
  Byte store: target `(r0 & 0xff) +0x31a524`, value `0x1` (`strb` at 0x44400).
  Recorded r0 = `r0` (`ldm` at 0x44404).

## `ip69_soft_reset`

- `ip69_soft_reset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x442b8 to 0x442fc.
  Established facts:
  Call (`bl` at 0x442dc): (unknown callee), target 0x81170, r0=`0x83de4`, r1=`unknown`, r2=`r2`, r3=`unknown`.
  Recorded r0 = `ret(0x81170)` (`ldm` at 0x442f4, `bl` at 0x442dc).

## `imageSyncResetHold`

- `imageSyncResetHold`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12a5b0 to 0x12a634.
  Established facts:
  Call (`bl` at 0x12a5d8): (unknown callee), target 0x12a8e8, r0=`r0`, r1=`(r1 & 0xff)`, r2=`r2`, r3=`(r1 & 0xff)`.
  Branch (`bne` at 0x12a5ec): taken to 0x12a5fc when `ret(0x12a8e8) != 0x0`, falls through to 0x12a5f0 when `ret(0x12a8e8) == 0x0` (source `bl` at 0x12a5d8).
  Word load: target `abs 0x12a630` (`ldr` at 0x12a5f0).
  Grouped accesses: 2 reported byte load records with printed base expression `ret(0x12a8e8)` and nonnegative byte-aligned offsets, including target `ret(0x12a8e8) +0x2` (`ldrb` at 0x12a600), target `ret(0x12a8e8) +0x3` (`ldrb` at 0x12a608) (source `bl` at 0x12a5d8).
  Call (`bl` at 0x12a618): (unknown callee), target 0x125a0c, r0=`[sp-20]`, r1=`[ret(0x12a8e8)+2]:1`, r2=`[ret(0x12a8e8)+3]:1`, r3=`[ret(0x12a8e8)+3]:1`.
  Recorded r0 = `unknown` (`ldm` at 0x12a62c).

## `imageSyncResetRelease`

- `imageSyncResetRelease`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12a634 to 0x12a6b8.
  Established facts:
  Call (`bl` at 0x12a65c): (unknown callee), target 0x12a8e8, r0=`r0`, r1=`(r1 & 0xff)`, r2=`r2`, r3=`(r1 & 0xff)`.
  Branch (`bne` at 0x12a670): taken to 0x12a680 when `ret(0x12a8e8) != 0x0`, falls through to 0x12a674 when `ret(0x12a8e8) == 0x0` (source `bl` at 0x12a65c).
  Word load: target `abs 0x12a6b4` (`ldr` at 0x12a674).
  Grouped accesses: 2 reported byte load records with printed base expression `ret(0x12a8e8)` and nonnegative byte-aligned offsets, including target `ret(0x12a8e8) +0x2` (`ldrb` at 0x12a684), target `ret(0x12a8e8) +0x3` (`ldrb` at 0x12a68c) (source `bl` at 0x12a65c).
  Call (`bl` at 0x12a69c): (unknown callee), target 0x1259bc, r0=`[sp-20]`, r1=`[ret(0x12a8e8)+2]:1`, r2=`[ret(0x12a8e8)+3]:1`, r3=`[ret(0x12a8e8)+3]:1`.
  Recorded r0 = `unknown` (`ldm` at 0x12a6b0).

## `psicResetAll`

- `psicResetAll`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12b4e4 to 0x12b5d0.
  Established facts:
  Call (`bl` at 0x12b50c): (unknown callee), target 0x12576c, r0=`r0`, r1=`sp-24`, r2=`r2`, r3=`sp-24`.
  Byte load: target `(unknown effective address)` (`ldrb` at 0x12b540).
  Call (`bl` at 0x12b55c): (unknown callee), target 0x125958, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0x12b568): taken to 0x12b5b0 when `ret(0x125958) == 0x0`, falls through to 0x12b56c when `ret(0x125958) != 0x0` (source `bl` at 0x12b55c).
  Call (`bl` at 0x12b584): (unknown callee), target 0x12b2b8, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0x12b598): taken to 0x12b5b0 when `ret(0x12b2b8) == 0x0`, falls through to 0x12b59c when `ret(0x12b2b8) != 0x0` (source `bl` at 0x12b584).
  Recorded r0 = `[sp-36]` (`ldm` at 0x12b5cc).

## `portReset`

- `portReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5b3f0 to 0x5b69c.
  Established facts:
  Word load: target `abs 0x1a5c2c` (`ldr` at 0x5b40c).
  Branch (`bge` at 0x5b414): taken to 0x5b680 when `r0 >= [0x1a5c2c] (signed, 32-bit)`, falls through to 0x5b418 when `r0 < [0x1a5c2c] (signed, 32-bit)` (source `ldr` at 0x5b40c).
  Call (`bl` at 0x5b530): (unknown callee), target 0x605e8, r0=`unknown`, r1=`unknown`, r2=`sp-52`, r3=`sp-48`.
  Call (`bl` at 0x5b54c): (unknown callee), target 0x558d8, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5b564): (unknown callee), target 0x60d28, r0=`[sp-24]`, r1=`[sp-56]`, r2=`unknown`, r3=`ret(0x558d8)`.
  Branch (`bne` at 0x5b570): taken to 0x5b5ac when `ret(0x60d28) != 0x1`, falls through to 0x5b574 when `ret(0x60d28) == 0x1` (source `bl` at 0x5b564).
  Grouped accesses: 2 reported byte store records with printed base expression `[sp-60]` and nonnegative byte-aligned offsets, including target `[sp-60] +0x0`, value `0x3` (`strb` at 0x5b57c), target `[sp-60] +0x0`, value `0x3` (`strb` at 0x5b5a8).
  Call (`bl` at 0x5b590): (unknown callee), target 0x60a94, r0=`[sp-24]`, r1=`[sp-56]`, r2=`0x2`, r3=`unknown`.
  Branch (`bne` at 0x5b59c): taken to 0x5b5ac when `ret(0x60a94) != 0x1`, falls through to 0x5b5a0 when `ret(0x60a94) == 0x1` (source `bl` at 0x5b590).
  Call (`bl` at 0x5b5b4): (unknown callee), target 0x5e570, r0=`[sp-24]`, r1=`[sp-56]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5b5dc): (unknown callee), target 0x6e1b0, r0=`[sp-24]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x5b5e8): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5b630): (unknown callee), target 0x203f4, r0=`[sp-72]`, r1=`[sp-64]`, r2=`[sp-68]`, r3=`unknown`.
  Call (`bl` at 0x5b63c): (unknown callee), target 0x21ce0, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5b66c): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x1621c4`, r2=`0x1f3`, r3=`0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x5b684).

## `Abacus_DCM_Reset`

- `Abacus_DCM_Reset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4747c to 0x47564.
  Established facts:
  Call (`bl` at 0x474c8): (unknown callee), target 0x1259bc, r0=`r0`, r1=`0x3`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0x474dc): taken to 0x474f4 when `ret(0x1259bc) == 0x0`, falls through to 0x474e0 when `ret(0x1259bc) != 0x0` (source `bl` at 0x474c8).
  Call (`bl` at 0x47510): (unknown callee), target 0x125a0c, r0=`[sp-20]`, r1=`0x3`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0x47544): taken to 0x47550 when `[sp-28] == 0x0`, falls through to 0x47548 when `[sp-28] != 0x0`.
  Call (`bl` at 0x4754c): (unknown callee), target 0x47564, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`[sp-28]`.
  Recorded r0 = `unknown` (`ldm` at 0x47560).

## `JChip_reset_field`

- `JChip_reset_field`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13f614 to 0x13f6bc.
  Established facts:
  Word load: target `abs 0x315034` (`ldr` at 0x13f634).
  Branch (`bls` at 0x13f63c): taken to 0x13f654 when `[0x315034] <= 0x2 (unsigned, 32-bit)`, falls through to 0x13f640 when `[0x315034] > 0x2 (unsigned, 32-bit)` (source `ldr` at 0x13f634).
  Call (`bl` at 0x13f650): (unknown callee), target 0x243d4, r0=`0x192380`, r1=`r0`, r2=`r1`, r3=`r2`.
  Call (`bl` at 0x13f658): (unknown callee), target 0x13dfe0, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`bne` at 0x13f66c): taken to 0x13f67c when `ret(0x13dfe0) != 0x0`, falls through to 0x13f670 when `ret(0x13dfe0) == 0x0` (source `bl` at 0x13f658).
  Word load: target `abs 0x13f6b8` (`ldr` at 0x13f670).
  Word load: target `ret(0x13dfe0) +0x28` (`ldr` at 0x13f688, `bl` at 0x13f658).
  Call (`bl` at 0x13f698): (unknown callee), target 0x144ca4, r0=`unknown`, r1=`[sp-28]`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x13f6ac).

## `tdrdata_reset_field`

- `tdrdata_reset_field`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x144ca4 to 0x144d78.
  Established facts:
  Grouped accesses: 5 reported word load records with printed base expression `r0` and nonnegative word-aligned offsets, including target `r0 +0x8` (`ldr` at 0x144cc0), target `r0 +0x8` (`ldr` at 0x144ce8), target `r0 +0x8` (`ldr` at 0x144cf0).
  Grouped accesses: 4 reported word load records with printed base expression `[r0+8]` and nonnegative word-aligned offsets, including target `[r0+8] +0x4` (`ldr` at 0x144cd8), target `[r0+8] +0x4` (`ldr` at 0x144d08), target `[r0+8] +0x4` (`ldr` at 0x144d34).
  Call (`bl` at 0x144d50): (unknown callee), target 0x132444, r0=`unknown`, r1=`[[r0+8]+8]`, r2=`unknown`, r3=`[r0+16]`.
  Halfword store: target `(unknown effective address)`, value `0x0` (`strh` at 0x144d6c).
  Recorded r0 = `ret(0x132444)` (`ldm` at 0x144d74, `bl` at 0x144d50).
