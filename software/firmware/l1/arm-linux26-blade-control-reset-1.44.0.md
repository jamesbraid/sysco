# ARM Linux 2.6 blade control and reset routines in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-blade-control-reset-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register
state from the analysis rather than a C return type.

## `bladeIsConfigured`

- `bladeIsConfigured`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3ec6c to 0x3ed90.
  Established facts:
  Call (`bl` at 0x3ec80): (unknown callee), target 0x3e38c, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0x3ec90): (unknown callee), target 0x6e190, r0=`ret(0x3e38c)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e38c)`.
  Word load: target `abs 0x1a5aa0` (`ldr` at 0x3ecd4).
  Branch (`beq` at 0x3ecdc): taken to 0x3ed44 when `[0x1a5aa0] == 0x0`, falls through to 0x3ece0 when `[0x1a5aa0] != 0x0`.
  Recorded r0 = `[sp-24]` (`ldm` at 0x3ed80).

## `bladeConfiguredMask`

- `bladeConfiguredMask`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3ed90 to 0x3ee14.
  Established facts:
  Word load: target `abs 0x19e6e0` (`ldr` at 0x3edb8).
  Call (`bl` at 0x3edc8): (unknown callee), target 0x3ec6c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[0x19e6e0]`.
  Branch (`beq` at 0x3edd4): taken to 0x3edf0 when `ret(0x3ec6c) == 0x0`, falls through to 0x3edd8 when `ret(0x3ec6c) != 0x0` (source `bl` at 0x3edc8).
  Recorded r0 = `unknown` (`ldm` at 0x3ee0c).

## `bladeIsReadyToConfigure`

- `bladeIsReadyToConfigure`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3ee14 to 0x3ef70.
  Established facts:
  Call (`bl` at 0x3ee28): (unknown callee), target 0x3e38c, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0x3ee38): (unknown callee), target 0x6e190, r0=`ret(0x3e38c)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e38c)`.
  Word load: target `abs 0x1a5ab0` (`ldr` at 0x3eea0).
  Call (`bl` at 0x3eec4): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0x3eed0): taken to 0x3ef50 when `ret(0x3e58c) == 0x0`, falls through to 0x3eed4 when `ret(0x3e58c) != 0x0` (source `bl` at 0x3eec4).
  Call (`bl` at 0x3eed8): (unknown callee), target 0x3f474, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Recorded r0 = `[sp-24]` (`ldm` at 0x3ef60).

## `bladeInitAllHardware`

- `bladeInitAllHardware`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3ef70 to 0x3f064.
  Established facts:
  Branch (`beq` at 0x3ef94): taken to 0x3efa4 when `[0x1a5aa0] == 0x0`, falls through to 0x3ef98 when `[0x1a5aa0] != 0x0`.
  Call (`bl` at 0x3efc4): (unknown callee), target 0x3e58c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[0x19e6e0]`.
  Call (`bl` at 0x3efd8): (unknown callee), target 0x3f474, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x3f01c): (unknown callee), target 0x3e904, r0=`[sp-20]`, r1=`0x15a9a4`, r2=`unknown`, r3=`unknown`.
  Word store: target `abs 0x19e6e4`, value `0x0` (`str` at 0x3f038).
  Recorded r0 = `0x0` (`ldm` at 0x3f04c).

## `bladeEnable`

- `bladeEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3f064 to 0x3f270.
  Established facts:
  Branch (`bne` at 0x3f0e0): taken to 0x3f158 when `[0x19e6e4] != 0x0`, falls through to 0x3f0e4 when `[0x19e6e4] == 0x0`.
  Byte store: target `(unknown effective address)`, value `unknown` (`strb` at 0x3f154).
  Byte store: target `(unknown effective address)`, value `unknown` (`strb` at 0x3f1a0).
  Call (`bl` at 0x3f1d8): (unknown callee), target 0x2624c, r0=`[sp-20]+180`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Byte store: target `(unknown effective address)`, value `0x0` (`strb` at 0x3f23c).
  Call (`bl` at 0x3f248): (unknown callee), target 0x3f5fc, r0=`[sp-20]`, r1=`0x1`, r2=`unknown`, r3=`0x0`.
  Recorded r0 = `[sp-24]` (`ldm` at 0x3f25c).

## `bladeDisable`

- `bladeDisable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3f270 to 0x3f474.
  Established facts:
  Call (`bl` at 0x3f2e4): (unknown callee), target 0x3ee14, r0=`[sp-20]`, r1=`r1`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3f2f8): (unknown callee), target 0x558d8, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3ee14)`.
  Branch (`bne` at 0x3f320): taken to 0x3f398 when `[0x19e6e4] != 0x0`, falls through to 0x3f324 when `[0x19e6e4] == 0x0`.
  Byte store: target `(unknown effective address)`, value `unknown` (`strb` at 0x3f394).
  Byte store: target `(unknown effective address)`, value `unknown` (`strb` at 0x3f3e0).
  Call (`bl` at 0x3f418): (unknown callee), target 0x2624c, r0=`[sp-20]+180`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3f420): (unknown callee), target 0x3ee14, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3f448): (unknown callee), target 0x3fcc8, r0=`[sp-20]`, r1=`0x1`, r2=`unknown`, r3=`[0x1a5aa0]`.
  Recorded r0 = `[sp-28]` (`ldm` at 0x3f45c).

## `bladeIsEnabled`

- `bladeIsEnabled`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3f474 to 0x3f5fc.
  Established facts:
  Call (`bl` at 0x3f488): (unknown callee), target 0x3e38c, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0x3f498): (unknown callee), target 0x6e190, r0=`ret(0x3e38c)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e38c)`.
  Call (`bl` at 0x3f4dc): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`[sp-20]`, r3=`[0x19e6e0]`.
  Branch (`beq` at 0x3f548): taken to 0x3f5b0 when `[0x1a5aa0] == 0x0`, falls through to 0x3f54c when `[0x1a5aa0] != 0x0`.
  Byte load: target `(unknown effective address)` (`ldrb` at 0x3f56c).
  Recorded r0 = `[sp-24]` (`ldm` at 0x3f5ec).

## `bladeUpdatePresentMask`

- `bladeUpdatePresentMask`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4001c to 0x402b4.
  Established facts:
  Byte store: target `(unknown effective address)`, value `unknown` (`strb` at 0x40138).
  Call (`bl` at 0x40150): (unknown callee), target 0x3fcc8, r0=`[sp-24]`, r1=`0x0`, r2=`unknown`, r3=`[sp-28]+1`.
  Byte store: target `(unknown effective address)`, value `unknown` (`strb` at 0x40200).
  Call (`bl` at 0x40214): (unknown callee), target 0x3fff4, r0=`0x32`, r1=`unknown`, r2=`unknown`, r3=`[sp-32]`.
  Call (`bl` at 0x40234): (unknown callee), target 0x3f5fc, r0=`[sp-24]`, r1=`0x0`, r2=`unknown`, r3=`[sp-28]+1`.
  Branch (`beq` at 0x40254): taken to 0x40290 when `[0x1a5aa0] == 0x0`, falls through to 0x40258 when `[0x1a5aa0] != 0x0`.
  Call (`bl` at 0x4028c): (unknown callee), target 0x22070, r0=`0x6`, r1=`0x15a8a8`, r2=`0x3e9`, r3=`0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x40294).

## `blade_state_msg`

- `blade_state_msg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4036c to 0x404ec.
  Established facts:
  Branch (`ldrls` at 0x40388): taken to (unknown target) when `r0 <= 0xe (unsigned, 32-bit)`, falls through to 0x4038c when `r0 > 0xe (unsigned, 32-bit)`.
  Omitted-path effects remain unknown at the unresolved transfer (`ldrls` at 0x40388).
  Known-edge recorded r0 = `0x15abd0` (`ldm` at 0x404a8).

## `bladeQuery`

- `bladeQuery`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x404ec to 0x408c8.
  Established facts:
  Call (`bl` at 0x40530): (unknown callee), target 0x201fc, r0=`unknown`, r1=`r1`, r2=`[0x19e6e0]`, r3=`unknown`.
  Word store: target `r1 +0x0`, value `ret(0x201fc)` (`str` at 0x40538, `bl` at 0x40530).
  Call (`bl` at 0x4054c): (unknown callee), target 0x205bc, r0=`[r1]`, r1=`0x15abe0`, r2=`unknown`, r3=`[r1]`.
  Call (`bl` at 0x40554): (unknown callee), target 0x2031c, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0x40594): taken to 0x405d0 when `[0x1a5aa0] == 0x0`, falls through to 0x40598 when `[0x1a5aa0] != 0x0`.
  Call (`bl` at 0x405d4): (unknown callee), target 0x3e58c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[0x1a5aa0]`.
  Call (`bl` at 0x406b8): (unknown callee), target 0x4036c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x40704): (unknown callee), target 0x205bc, r0=`sp-64`, r1=`0x15ac70`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x40718): (unknown callee), target 0x402b4, r0=`[sp-36]`, r1=`unknown`, r2=`unknown`, r3=`[sp-44]`.
  Call (`bl` at 0x40848): (unknown callee), target 0x205bc, r0=`[sp-32]`, r1=`0x15aca4`, r2=`[sp-36]`, r3=`[sp-84]`.
  Call (`bl` at 0x40850): (unknown callee), target 0x2031c, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x4088c): (unknown callee), target 0x205bc, r0=`([sp-32] + ret(0x2031c))`, r1=`0x15acdc`, r2=`unknown`, r3=`[sp-40]`.
  Recorded r0 = `unknown` (`ldm` at 0x40894).

## `bladeSetState`

- `bladeSetState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x408c8 to 0x40bbc.
  Established facts:
  Call (`bl` at 0x408e8): (unknown callee), target 0x6e190, r0=`r0`, r1=`r1`, r2=`r2`, r3=`0x0`.
  Call (`bl` at 0x4090c): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`ret(0x6e190)`, r3=`[sp-20]`.
  Call (`bl` at 0x40920): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x40934): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x40954): (unknown callee), target 0x3f474, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x40974): (unknown callee), target 0x3ec6c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3f474)`.
  Word load: target `abs 0x31a520` (`ldr` at 0x409a4).
  Branch (`beq` at 0x40a50): taken to 0x40ad4 when `[sp-24] == 0xd`, falls through to 0x40a54 when `[sp-24] != 0xd`.
  Call (`bl` at 0x40a98): (unknown callee), target 0x40bbc, r0=`[sp-20]`, r1=`0x0`, r2=`0xff`, r3=`[sp-28]`.
  Call (`bl` at 0x40acc): (unknown callee), target 0x40bbc, r0=`[sp-20]`, r1=`0x0`, r2=`[sp-44]`, r3=`unknown`.
  Call (`bl` at 0x40aec): (unknown callee), target 0x40bbc, r0=`[sp-20]`, r1=`0x1`, r2=`0xff`, r3=`[sp-28]`.
  Call (`bl` at 0x40b00): (unknown callee), target 0x40bbc, r0=`[sp-20]`, r1=`0x1`, r2=`0x0`, r3=`[sp-28]`.
  Call (`bl` at 0x40b20): (unknown callee), target 0x40bbc, r0=`[sp-20]`, r1=`0x0`, r2=`0xff`, r3=`[sp-28]`.
  Call (`bl` at 0x40b54): (unknown callee), target 0x40bbc, r0=`[sp-20]`, r1=`[sp-48]`, r2=`0x1`, r3=`unknown`.
  Byte store: target `(unknown effective address)`, value `([sp-24] & 0xff)` (`strb` at 0x40b8c).
  Recorded r0 = `unknown` (`ldm` at 0x40ba0).

## `bladeSetLEDs`

- `bladeSetLEDs`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x40bbc to 0x40edc.
  Established facts:
  Call (`bl` at 0x40c48): (unknown callee), target 0x3ee14, r0=`[sp-20]`, r1=`r1`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x40c5c): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3ee14)`.
  Call (`bl` at 0x40c70): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x40c84): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x40ca4): (unknown callee), target 0x3f474, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x40cc4): (unknown callee), target 0x3ec6c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3f474)`.
  Byte load: target `[sp-28] +0x71` (`ldrb` at 0x40d0c).
  Branch (`beq` at 0x40d14): taken to 0x40dd0 when `[[sp-28]+113]:1 == 0x0`, falls through to 0x40d18 when `[[sp-28]+113]:1 != 0x0`.
  Call (`bl` at 0x40db0): (unknown callee), target 0x125a58, r0=`[sp-20]`, r1=`[[sp-28]+112]:1`, r2=`([[sp-28]+113]:1 & 0xff)`, r3=`[sp-37]:1`.
  Byte load: target `[sp-28] +0x72` (`ldrb` at 0x40de0).
  Call (`bl` at 0x40e84): (unknown callee), target 0x125a58, r0=`[sp-20]`, r1=`[[sp-28]+112]:1`, r2=`([[sp-28]+114]:1 & 0xff)`, r3=`[sp-37]:1`.
  Recorded r0 = `[sp-44]` (`ldm` at 0x40ec0).

## `bladeGetLEDs`

- `bladeGetLEDs`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x40edc to 0x41194.
  Established facts:
  Call (`bl` at 0x40f60): (unknown callee), target 0x3ee14, r0=`[sp-20]`, r1=`r1`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x40f74): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3ee14)`.
  Call (`bl` at 0x40f88): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x40f9c): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x40fbc): (unknown callee), target 0x3f474, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x40fdc): (unknown callee), target 0x3ec6c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3f474)`.
  Call (`bl` at 0x41030): (unknown callee), target 0x125c20, r0=`[sp-20]`, r1=`[[sp-32]+112]:1`, r2=`unknown`, r3=`[[sp-32]+112]:1`.
  Byte store: target `[sp-24] +0x0`, value `unknown` (`strb` at 0x4108c).
  Byte store: target `[sp-24] +0x0`, value `unknown` (`strb` at 0x410b4).
  Call (`bl` at 0x410d4): (unknown callee), target 0x125c20, r0=`[sp-20]`, r1=`[[sp-32]+112]:1`, r2=`unknown`, r3=`[[sp-32]+112]:1`.
  Byte store: target `[sp-24] +0x0`, value `unknown` (`strb` at 0x4113c).
  Byte store: target `[sp-24] +0x0`, value `unknown` (`strb` at 0x41164).
  Recorded r0 = `[sp-48]` (`ldm` at 0x41178).

## `resetBrick`

- `resetBrick`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6af98 to 0x6b040.
  Established facts:
  Branch (`beq` at 0x6afb4): taken to 0x6afd0 when `r0 == 0x8`, falls through to 0x6afb8 when `r0 != 0x8`.
  Call (`bl` at 0x6afbc): (unknown callee), target 0x6c04c, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r0`.
  Call (`bl` at 0x6afcc): (unknown callee), target 0x29908, r0=`0x16432c`, r1=`ret(0x6c04c)`, r2=`unknown`, r3=`ret(0x6c04c)`.
  Call (`bl` at 0x6afd4): (unknown callee), target 0x6c6f8, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0x6afe8): taken to 0x6aff8 when `ret(0x6c6f8) == 0x0`, falls through to 0x6afec when `ret(0x6c6f8) != 0x0` (source `bl` at 0x6afd4).
  Call (`bl` at 0x6affc): (unknown callee), target 0x6b040, r0=`0xa`, r1=`unknown`, r2=`unknown`, r3=`ret(0x6c6f8)`.
  Call (`bl` at 0x6b004): (unknown callee), target 0x6c72c, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x6b038).

## `resetSlot`

- `resetSlot`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6b068 to 0x6b248.
  Established facts:
  Call (`bl` at 0x6b080): (unknown callee), target 0x3e274, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0x6b090): (unknown callee), target 0x6e190, r0=`ret(0x3e274)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e274)`.
  Call (`bl` at 0x6b0b4): (unknown callee), target 0x3e58c, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x6b0c8): (unknown callee), target 0x3e58c, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x6b0dc): (unknown callee), target 0x3e58c, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x6b0fc): (unknown callee), target 0x3f474, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x6b11c): (unknown callee), target 0x3ec6c, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3f474)`.
  Branch (`beq` at 0x6b140): taken to 0x6b1cc when `[sp-28] == 0x8`, falls through to 0x6b144 when `[sp-28] != 0x8`.
  Call (`bl` at 0x6b154): (unknown callee), target 0x6e1b0, r0=`[sp-24]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x6b160): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x6b1a8): (unknown callee), target 0x203f4, r0=`[sp-44]`, r1=`[sp-36]`, r2=`[sp-40]`, r3=`unknown`.
  Call (`bl` at 0x6b1b4): (unknown callee), target 0x6c04c, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x6b1c8): (unknown callee), target 0x29908, r0=`0x164338`, r1=`ret(0x203f4)`, r2=`ret(0x6c04c)`, r3=`ret(0x6c04c)`.
  Call (`bl` at 0x6b1d0): (unknown callee), target 0x6c6f8, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0x6b1e4): taken to 0x6b1f4 when `ret(0x6c6f8) == 0x0`, falls through to 0x6b1e8 when `ret(0x6c6f8) != 0x0` (source `bl` at 0x6b1d0).
  Call (`bl` at 0x6b1f8): (unknown callee), target 0x6b040, r0=`0xa`, r1=`unknown`, r2=`unknown`, r3=`ret(0x6c6f8)`.
  Call (`bl` at 0x6b200): (unknown callee), target 0x6c72c, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x6b234).
