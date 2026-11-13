# ARM Linux 2.6 slot reset control routines in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-slot-reset-controls-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register
state from the analysis rather than a C return type.

## `resetHoldSlot`

- `resetHoldSlot`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6b448 to 0x6b5d8.
  Established facts:
  Call (`bl` at 0x6b460): (unknown callee), target 0x3e274, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Branch (`ble` at 0x6b46c): taken to 0x6b490 when `ret(0x3e274) <= 0x0 (signed, 32-bit)`, falls through to 0x6b470 when `ret(0x3e274) > 0x0 (signed, 32-bit)` (source `bl` at 0x6b460).
  Call (`bl` at 0x6b470): (unknown callee), target 0x6e190, r0=`ret(0x3e274)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e274)`.
  Call (`bl` at 0x6b494): (unknown callee), target 0x3e58c, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`bge` at 0x6b4a0): taken to 0x6b4b8 when `ret(0x3e58c) >= 0x0 (signed, 32-bit)`, falls through to 0x6b4a4 when `ret(0x3e58c) < 0x0 (signed, 32-bit)` (source `bl` at 0x6b494).
  Call (`bl` at 0x6b4a8): (unknown callee), target 0x3e58c, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x6b4bc): (unknown callee), target 0x3e58c, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x6b4dc): (unknown callee), target 0x3f474, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x6b4fc): (unknown callee), target 0x3ec6c, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3f474)`.
  Call (`bl` at 0x6b534): (unknown callee), target 0x6e1b0, r0=`[sp-24]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x6b540): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x6b588): (unknown callee), target 0x203f4, r0=`[sp-44]`, r1=`[sp-36]`, r2=`[sp-40]`, r3=`unknown`.
  Call (`bl` at 0x6b594): (unknown callee), target 0x6c04c, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x6b5a8): (unknown callee), target 0x29908, r0=`0x164368`, r1=`ret(0x203f4)`, r2=`ret(0x6c04c)`, r3=`ret(0x6c04c)`.
  Call (`bl` at 0x6b5b0): (unknown callee), target 0x6c6f8, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x6b5c4).

## `resetHoldSlotMask`

- `resetHoldSlotMask`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6b6c4 to 0x6b788.
  Established facts:
  Call (`bl` at 0x6b6e8): (unknown callee), target 0x3e274, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`ble` at 0x6b6f8): taken to 0x6b770 when `ret(0x3e274) <= [sp-24] (signed, 32-bit)`, falls through to 0x6b6fc when `ret(0x3e274) > [sp-24] (signed, 32-bit)` (source `bl` at 0x6b6e8).
  Call (`bl` at 0x6b720): (unknown callee), target 0x3e58c, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0x6b72c): taken to 0x6b760 when `ret(0x3e58c) == 0x0`, falls through to 0x6b730 when `ret(0x3e58c) != 0x0` (source `bl` at 0x6b720).
  Call (`bl` at 0x6b734): (unknown callee), target 0x3ec6c, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x6b774): (unknown callee), target 0x6c12c, r0=`[sp-28]`, r1=`unknown`, r2=`ret(0x3e274)`, r3=`[sp-24]`.
  Recorded r0 = `ret(0x6c12c)` (`ldm` at 0x6b784, `bl` at 0x6b774).

## `resetReleaseSlotMask`

- `resetReleaseSlotMask`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6b788 to 0x6b84c.
  Established facts:
  Call (`bl` at 0x6b7ac): (unknown callee), target 0x3e274, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`ble` at 0x6b7bc): taken to 0x6b834 when `ret(0x3e274) <= [sp-24] (signed, 32-bit)`, falls through to 0x6b7c0 when `ret(0x3e274) > [sp-24] (signed, 32-bit)` (source `bl` at 0x6b7ac).
  Call (`bl` at 0x6b7e4): (unknown callee), target 0x3e58c, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0x6b7f0): taken to 0x6b824 when `ret(0x3e58c) == 0x0`, falls through to 0x6b7f4 when `ret(0x3e58c) != 0x0` (source `bl` at 0x6b7e4).
  Call (`bl` at 0x6b7f8): (unknown callee), target 0x3ec6c, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x6b838): (unknown callee), target 0x6ca54, r0=`[sp-28]`, r1=`unknown`, r2=`ret(0x3e274)`, r3=`[sp-24]`.
  Recorded r0 = `ret(0x6ca54)` (`ldm` at 0x6b848, `bl` at 0x6b838).

## `softResetSlot`

- `softResetSlot`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6b944 to 0x6bb08.
  Established facts:
  Call (`bl` at 0x6b968): (unknown callee), target 0x3e58c, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r2`.
  Call (`bl` at 0x6b97c): (unknown callee), target 0x3e58c, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x6b990): (unknown callee), target 0x3e58c, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x6b9b0): (unknown callee), target 0x3f474, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x6b9d0): (unknown callee), target 0x3ec6c, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3f474)`.
  Call (`bl` at 0x6ba08): (unknown callee), target 0x6e1b0, r0=`[sp-24]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x6ba14): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x6ba5c): (unknown callee), target 0x203f4, r0=`[sp-44]`, r1=`[sp-36]`, r2=`[sp-40]`, r3=`unknown`.
  Call (`bl` at 0x6ba68): (unknown callee), target 0x6c04c, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x6ba7c): (unknown callee), target 0x29908, r0=`0x16438c`, r1=`ret(0x203f4)`, r2=`ret(0x6c04c)`, r3=`ret(0x6c04c)`.
  Word load: target `abs 0x31a520` (`ldr` at 0x6ba8c).
  Word load: target `abs 0x31a520` (`ldr` at 0x6bab0).
  Call (`mov` at 0x6bad0): (unknown callee), target (unknown target), r0=`[sp-24]`, r1=`[sp-29]:1`, r2=`[sp-29]:1`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x6baf0).

## `resetSource`

- `resetSource`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6c04c to 0x6c12c.
  Established facts:
  Branch (`ldrls` at 0x6c06c): taken to (unknown target) when `r0-1 <= 0x7 (unsigned, 32-bit)`, falls through to 0x6c070 when `r0-1 > 0x7 (unsigned, 32-bit)` (source `ldr` at 0x6c060, `sub` at 0x6c064).
  Omitted-path effects remain unknown at the unresolved transfer (`ldrls` at 0x6c06c).
  Known-edge recorded r0 = `0x164400` (`ldm` at 0x6c104).

## `IP57_ResetHold`

- `IP57_ResetHold`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x83700 to 0x839b0.
  Established facts:
  Call (`bl` at 0x83728): (unknown callee), target 0x84274, r0=`(r0 & 0xff)`, r1=`sp-24`, r2=`sp-24`, r3=`(r0 & 0xff)`.
  Branch (`beq` at 0x8373c): taken to 0x8374c when `ret(0x84274) == 0x0`, falls through to 0x83740 when `ret(0x84274) != 0x0` (source `bl` at 0x83728).
  Word load: target `abs 0x1a0874` (`ldr` at 0x83790).
  Call (`mov` at 0x837a0): (unknown callee), target (unknown target), r0=`unknown`, r1=`unknown`, r2=`[0x1a0874]`, r3=`unknown`.
  Call (`bl` at 0x837a8): (unknown callee), target 0x6aaf0, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x837b0): (unknown callee), target 0x839b0, r0=`0xa`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x837b4): (unknown callee), target 0x5572c, r0=`ret(0x839b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x837c4): (unknown callee), target 0x6dc0c, r0=`ret(0x5572c)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x5572c)`.
  Call (`bl` at 0x837dc): (unknown callee), target 0x136a38, r0=`unknown`, r1=`sp-28`, r2=`sp-28`, r3=`unknown`.
  Call (`bl` at 0x83800): (unknown callee), target 0x6a1fc, r0=`[sp-17]:1`, r1=`unknown`, r2=`unknown`, r3=`[sp-17]:1`.
  Call (`bl` at 0x83838): (unknown callee), target 0x125a58, r0=`0x0`, r1=`[sp-29]:1`, r2=`0xc`, r3=`0x8`.
  Call (`bl` at 0x8386c): (unknown callee), target 0x1259bc, r0=`0x0`, r1=`[sp-29]:1`, r2=`0x10`, r3=`[sp-29]:1`.
  Call (`bl` at 0x838c4): (unknown callee), target 0x7fe94, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x838f8): (unknown callee), target 0x125a0c, r0=`0x0`, r1=`unknown`, r2=`0x4`, r3=`unknown`.
  Call (`bl` at 0x83940): (unknown callee), target 0x125a58, r0=`0x0`, r1=`[sp-29]:1`, r2=`0xc0`, r3=`0x0`.
  Call (`bl` at 0x83974): (unknown callee), target 0x125a0c, r0=`0x0`, r1=`[sp-29]:1`, r2=`0x2`, r3=`[sp-29]:1`.
  Recorded r0 = `unknown` (`ldm` at 0x839a8).

## `IP57_ResetRelease`

- `IP57_ResetRelease`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x839d8 to 0x83de4.
  Established facts:
  Call (`bl` at 0x83a00): (unknown callee), target 0x84274, r0=`(r0 & 0xff)`, r1=`sp-24`, r2=`sp-24`, r3=`(r0 & 0xff)`.
  Word load: target `abs 0x1a0874` (`ldr` at 0x83a68).
  Call (`mov` at 0x83a78): (unknown callee), target (unknown target), r0=`unknown`, r1=`unknown`, r2=`[0x1a0874]`, r3=`unknown`.
  Call (`bl` at 0x83a84): (unknown callee), target 0x6a220, r0=`[sp-17]:1`, r1=`unknown`, r2=`unknown`, r3=`[sp-17]:1`.
  Call (`bl` at 0x83ab8): (unknown callee), target 0x1259bc, r0=`0x0`, r1=`[sp-29]:1`, r2=`0x2`, r3=`[sp-29]:1`.
  Call (`bl` at 0x83ae0): (unknown callee), target 0x839b0, r0=`0xa`, r1=`unknown`, r2=`unknown`, r3=`ret(0x1259bc)`.
  Call (`bl` at 0x83af4): (unknown callee), target 0x125a0c, r0=`0x0`, r1=`[sp-29]:1`, r2=`0x2`, r3=`[sp-29]:1`.
  Call (`bl` at 0x83b1c): (unknown callee), target 0x839b0, r0=`0xa`, r1=`unknown`, r2=`unknown`, r3=`ret(0x125a0c)`.
  Call (`bl` at 0x83b30): (unknown callee), target 0x1259bc, r0=`0x0`, r1=`[sp-29]:1`, r2=`0x2`, r3=`[sp-29]:1`.
  Call (`bl` at 0x83b58): (unknown callee), target 0x839b0, r0=`0x15`, r1=`unknown`, r2=`unknown`, r3=`ret(0x1259bc)`.
  Call (`bl` at 0x83b64): (unknown callee), target 0x69d80, r0=`[sp-17]:1`, r1=`unknown`, r2=`unknown`, r3=`[sp-17]:1`.
  Call (`bl` at 0x83b98): (unknown callee), target 0x12cc8c, r0=`0x1`, r1=`0x0`, r2=`sp-112`, r3=`sp-112`.
  Call (`bl` at 0x83bac): (unknown callee), target 0x1259bc, r0=`0x0`, r1=`[sp-29]:1`, r2=`0xc0`, r3=`[sp-29]:1`.
  Call (`bl` at 0x83bd4): (unknown callee), target 0x839b0, r0=`0x3`, r1=`unknown`, r2=`unknown`, r3=`ret(0x1259bc)`.
  Call (`bl` at 0x83c0c): (unknown callee), target 0x7fe94, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x83c40): (unknown callee), target 0x1259bc, r0=`0x0`, r1=`unknown`, r2=`0x4`, r3=`unknown`.
  Call (`bl` at 0x83c78): (unknown callee), target 0x839b0, r0=`0x1`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x83c8c): (unknown callee), target 0x1259bc, r0=`0x0`, r1=`[sp-29]:1`, r2=`0x4`, r3=`[sp-29]:1`.
  Word load: target `abs 0x1a0254` (`ldr` at 0x83cb4).
  Call (`bl` at 0x83cb8): (unknown callee), target 0x839b0, r0=`[0x1a0254]`, r1=`unknown`, r2=`unknown`, r3=`0x1a0254`.
  Call (`bl` at 0x83cd8): (unknown callee), target 0x6a76c, r0=`[sp-17]:1`, r1=`unknown`, r2=`unknown`, r3=`[sp-17]:1`.
  Call (`bl` at 0x83cf8): (unknown callee), target 0x839b0, r0=`0xa`, r1=`unknown`, r2=`unknown`, r3=`ret(0x6a76c)`.
  Word load: target `abs 0x1a5ca0` (`ldr` at 0x83d20).
  Call (`bl` at 0x83d78): (unknown callee), target 0x22070, r0=`0x4`, r1=`0x166a08`, r2=`0x43a`, r3=`0x0`.
  Call (`bl` at 0x83da4): (unknown callee), target 0x12cc8c, r0=`0x1`, r1=`0x0`, r2=`sp-112`, r3=`sp-112`.
  Call (`bl` at 0x83dac): (unknown callee), target 0x6aaf0, r0=`0x1`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x83dc0).

## `IP57_SoftReset`

- `IP57_SoftReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x83de4 to 0x83ec4.
  Established facts:
  Call (`bl` at 0x83e0c): (unknown callee), target 0x84274, r0=`(r0 & 0xff)`, r1=`sp-24`, r2=`sp-24`, r3=`(r0 & 0xff)`.
  Branch (`beq` at 0x83e20): taken to 0x83e30 when `ret(0x84274) == 0x0`, falls through to 0x83e24 when `ret(0x84274) != 0x0` (source `bl` at 0x83e0c).
  Call (`bl` at 0x83e50): (unknown callee), target 0x125a0c, r0=`0x0`, r1=`([[sp-24]+2]:1+2 & 0xff)`, r2=`0x8`, r3=`([[sp-24]+2]:1+2 & 0xff)`.
  Branch (`beq` at 0x83e64): taken to 0x83e74 when `ret(0x125a0c) == 0x0`, falls through to 0x83e68 when `ret(0x125a0c) != 0x0` (source `bl` at 0x83e50).
  Call (`bl` at 0x83e78): (unknown callee), target 0x839b0, r0=`0xa`, r1=`unknown`, r2=`unknown`, r3=`ret(0x125a0c)`.
  Call (`bl` at 0x83e8c): (unknown callee), target 0x1259bc, r0=`0x0`, r1=`[sp-25]:1`, r2=`0x8`, r3=`[sp-25]:1`.
  Recorded r0 = `unknown` (`ldm` at 0x83ec0).

## `PICResetHold`

- `PICResetHold`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3c2c8 to 0x3c488.
  Established facts:
  Word load: target `abs 0x1a5a34` (`ldr` at 0x3c2e0).
  Branch (`bne` at 0x3c2e8): taken to 0x3c2f8 when `[0x1a5a34] != 0x0`, falls through to 0x3c2ec when `[0x1a5a34] == 0x0` (source `ldr` at 0x3c2e0).
  Call (`bl` at 0x3c354): (unknown callee), target 0x125a0c, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3c3a4): (unknown callee), target 0x125a0c, r0=`0x0`, r1=`[[sp-24]+2]:1`, r2=`[[sp-24]+3]:1`, r3=`[[sp-24]+3]:1`.
  Call (`bl` at 0x3c3f4): (unknown callee), target 0x125a0c, r0=`0x0`, r1=`[[sp-24]+4]:1`, r2=`[[sp-24]+5]:1`, r3=`[[sp-24]+5]:1`.
  Byte store: target `abs 0x1a5a39`, value `0x0` (`strb` at 0x3c430).
  Call (`bl` at 0x3c440): (unknown callee), target 0x7a50c, r0=`0x0`, r1=`0xff`, r2=`0xff`, r3=`0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x3c474).

## `PICResetRelease`

- `PICResetRelease`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3c488 to 0x3c770.
  Established facts:
  Word load: target `abs 0x1a5a34` (`ldr` at 0x3c4a8).
  Branch (`bne` at 0x3c4b0): taken to 0x3c4c0 when `[0x1a5a34] != 0x0`, falls through to 0x3c4b4 when `[0x1a5a34] == 0x0` (source `ldr` at 0x3c4a8).
  Call (`bl` at 0x3c4c0): (unknown callee), target 0x5572c, r0=`r0`, r1=`r1`, r2=`r2`, r3=`[0x1a5a34]`.
  Call (`bl` at 0x3c4e4): (unknown callee), target 0x7b064, r0=`0x0`, r1=`0x0`, r2=`unknown`, r3=`ret(0x5572c)`.
  Call (`bl` at 0x3c588): (unknown callee), target 0x1259bc, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3c5e4): (unknown callee), target 0x1259bc, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3c628): (unknown callee), target 0x3c770, r0=`0xa`, r1=`unknown`, r2=`[0x1a5a38]:1`, r3=`[sp-28]`.
  Call (`bl` at 0x3c688): (unknown callee), target 0x1259bc, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3c6c4): (unknown callee), target 0x7b064, r0=`0x0`, r1=`0x1`, r2=`[0x1a5a38]:1`, r3=`unknown`.
  Call (`bl` at 0x3c6ec): (unknown callee), target 0x7af1c, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`ret(0x7b064)`.
  Call (`bl` at 0x3c71c): (unknown callee), target 0x7a808, r0=`0x0`, r1=`0xff`, r2=`0xff`, r3=`ret(0x7af1c)`.
  Byte store: target `abs 0x1a5a39`, value `0x1` (`strb` at 0x3c748).
  Recorded r0 = `unknown` (`ldm` at 0x3c75c).

## `NL4R_ResetHold`

- `NL4R_ResetHold`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2db1c to 0x2dca4.
  Established facts:
  Word load: target `abs 0x1a599c` (`ldr` at 0x2db44).
  Branch (`bge` at 0x2db4c): taken to 0x2db7c when `r0 >= [0x1a599c] (signed, 32-bit)`, falls through to 0x2db50 when `r0 < [0x1a599c] (signed, 32-bit)` (source `ldr` at 0x2db44).
  Call (`bl` at 0x2dc48): (unknown callee), target 0x2e624, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x2dc94).

## `NL4R_ResetRelease`

- `NL4R_ResetRelease`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2dca4 to 0x2de34.
  Established facts:
  Word load: target `abs 0x1a599c` (`ldr` at 0x2dccc).
  Branch (`bge` at 0x2dcd4): taken to 0x2dd04 when `r0 >= [0x1a599c] (signed, 32-bit)`, falls through to 0x2dcd8 when `r0 < [0x1a599c] (signed, 32-bit)` (source `ldr` at 0x2dccc).
  Call (`bl` at 0x2dda4): (unknown callee), target 0x134634, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2ddd4): (unknown callee), target 0x2e898, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2de00): (unknown callee), target 0x1346fc, r0=`ret(0x2e898)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x2de24).

## `NL4R_SoftReset`

- `NL4R_SoftReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2de34 to 0x2e08c.
  Established facts:
  Word load: target `abs 0x1a599c` (`ldr` at 0x2de5c).
  Branch (`bge` at 0x2de64): taken to 0x2de94 when `r0 >= [0x1a599c] (signed, 32-bit)`, falls through to 0x2de68 when `r0 < [0x1a599c] (signed, 32-bit)` (source `ldr` at 0x2de5c).
  Call (`bl` at 0x2df74): (unknown callee), target 0x125a0c, r0=`unknown`, r1=`unknown`, r2=`0x8`, r3=`unknown`.
  Call (`bl` at 0x2dfc4): (unknown callee), target 0x2e08c, r0=`0xa`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2e020): (unknown callee), target 0x1259bc, r0=`unknown`, r1=`unknown`, r2=`0x8`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x2e07c).

## `promIceReset`

- `promIceReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x61198 to 0x612c8.
  Established facts:
  Word load: target `abs 0x31a608` (`ldr` at 0x611b8).
  Call (`bl` at 0x611cc): (unknown callee), target 0x61324, r0=`unknown`, r1=`unknown`, r2=`[0x31a608]`, r3=`unknown`.
  Word load: target `abs 0x31a60c` (`ldr` at 0x611e8).
  Call (`bl` at 0x61200): (unknown callee), target 0x28230, r0=`[sp-25]:1+108`, r1=`unknown`, r2=`unknown`, r3=`[sp-25]:1+108`.
  Byte load: target `[sp-20] +0x1` (`ldrb` at 0x61214).
  Call (`bl` at 0x61254): (unknown callee), target 0x125a58, r0=`0x0`, r1=`[[sp-20]]:1`, r2=`[[sp-20]+1]:1`, r3=`unknown`.
  Call (`bl` at 0x61270): (unknown callee), target 0x21ce0, r0=`ret(0x125a58)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x125a58)`.
  Call (`bl` at 0x6129c): (unknown callee), target 0x22070, r0=`0x1`, r1=`0x162ee0`, r2=`0x68`, r3=`0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x612b4).

## `psicReset`

- `psicReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12b2b8 to 0x12b4e4.
  Established facts:
  Byte load: target `[sp-24] +0x2` (`ldrb` at 0x12b358).
  Call (`bl` at 0x12b36c): (unknown callee), target 0x6e140, r0=`r0`, r1=`[sp-40]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x12b37c): (unknown callee), target 0x6e190, r0=`ret(0x6e140)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x6e140)`.
  Byte load: target `[sp-24] +0x3` (`ldrb` at 0x12b3dc).
  Byte load: target `[sp-24] +0x6` (`ldrb` at 0x12b3e4).
  Call (`bl` at 0x12b3f4): (unknown callee), target 0x12c16c, r0=`unknown`, r1=`[[sp-24]+3]:1`, r2=`0xa`, r3=`[[sp-24]+6]:1`.
  Call (`bl` at 0x12b414): (unknown callee), target 0x6e140, r0=`ret(0x12c16c)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x12c16c)`.
  Call (`bl` at 0x12b424): (unknown callee), target 0x6e190, r0=`ret(0x6e140)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x6e140)`.
  Call (`bl` at 0x12b498): (unknown callee), target 0x12c16c, r0=`unknown`, r1=`[[sp-24]+3]:1`, r2=`0xc`, r3=`0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x12b4d4).
