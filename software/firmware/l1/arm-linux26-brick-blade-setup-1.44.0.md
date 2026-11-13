# ARM Linux 2.6 brick and blade setup routines in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-brick-blade-setup-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register
state from the analysis rather than a C return type.

## `brickInit`

- `brickInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6cff8 to 0x6dc0c.
  Established facts:
  Call (`bl` at 0x6d014): (unknown callee), target 0x10a314, r0=`0x6`, r1=`r1`, r2=`r2`, r3=`0x1`.
  Word store: target `abs 0x3182f0`, value `0x1` (`str` at 0x6d020).
  Byte store: target `abs 0x1a5e95`, value `0x2` (`strb` at 0x6d02c).
  Call (`bl` at 0x6d048): (unknown callee), target 0x11ff70, r0=`0x0`, r1=`0xff`, r2=`sp-136`, r3=`sp-136`.
  Call (`bl` at 0x6d074): (unknown callee), target 0x20340, r0=`[sp-60]`, r1=`0x164404`, r2=`0x9`, r3=`[sp-60]`.
  Byte store: target `abs 0x1a5e94`, value `0x40` (`strb` at 0x6d09c).
  Call (`bl` at 0x6d0b0): (unknown callee), target 0x20340, r0=`[sp-140]`, r1=`0x164410`, r2=`0x9`, r3=`ret(0x20340)`.
  Call (`bl` at 0x6d0ec): (unknown callee), target 0x20340, r0=`[sp-140]`, r1=`0x16441c`, r2=`0x7`, r3=`ret(0x20340)`.
  Call (`bl` at 0x6d118): (unknown callee), target 0x20340, r0=`[sp-140]`, r1=`0x164428`, r2=`0x4`, r3=`ret(0x20340)`.
  Call (`bl` at 0x6d144): (unknown callee), target 0x20340, r0=`[sp-140]`, r1=`0x164430`, r2=`0xd`, r3=`ret(0x20340)`.
  Call (`bl` at 0x6d16c): (unknown callee), target 0x20340, r0=`[sp-52]`, r1=`0x164440`, r2=`0xc`, r3=`0x4c`.
  Call (`bl` at 0x6d198): (unknown callee), target 0x20340, r0=`[sp-140]`, r1=`0x164450`, r2=`0xd`, r3=`ret(0x20340)`.
  Call (`bl` at 0x6d1b0): (unknown callee), target 0x120154, r0=`sp-136`, r1=`unknown`, r2=`unknown`, r3=`sp-136`.
  Call (`bl` at 0x6d1b8): (unknown callee), target 0x28230, r0=`0x7f`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x6d1ec): (unknown callee), target 0x11ff70, r0=`0x0`, r1=`0xfe`, r2=`sp-136`, r3=`sp-136`.
  Call (`bl` at 0x6d218): (unknown callee), target 0x20340, r0=`[sp-60]`, r1=`0x164460`, r2=`0x4`, r3=`[sp-60]`.
  Call (`bl` at 0x6d248): (unknown callee), target 0x11ff70, r0=`0x0`, r1=`0xfd`, r2=`sp-136`, r3=`sp-136`.
  Call (`bl` at 0x6d274): (unknown callee), target 0x20340, r0=`[sp-60]`, r1=`0x164460`, r2=`0x4`, r3=`[sp-60]`.
  Call (`bl` at 0x6d298): (unknown callee), target 0x120154, r0=`sp-136`, r1=`unknown`, r2=`unknown`, r3=`sp-136`.
  Call (`bl` at 0x6d2ac): (unknown callee), target 0x20340, r0=`[sp-140]`, r1=`0x164468`, r2=`0x8`, r3=`ret(0x20340)`.
  Call (`bl` at 0x6d2d4): (unknown callee), target 0x20340, r0=`[sp-140]`, r1=`0x164474`, r2=`0xd`, r3=`ret(0x20340)`.
  Call (`bl` at 0x6d2f4): (unknown callee), target 0x120154, r0=`sp-136`, r1=`unknown`, r2=`unknown`, r3=`sp-136`.
  Call (`bl` at 0x6d328): (unknown callee), target 0x11ff70, r0=`0x0`, r1=`[sp-141]:1`, r2=`sp-136`, r3=`[sp-141]:1`.
  Call (`bl` at 0x6d35c): (unknown callee), target 0x20340, r0=`[sp-140]`, r1=`0x164484`, r2=`0x7`, r3=`unknown`.
  Call (`bl` at 0x6d388): (unknown callee), target 0x20340, r0=`[sp-140]`, r1=`0x16448c`, r2=`0xa`, r3=`ret(0x20340)`.
  Call (`bl` at 0x6d3b4): (unknown callee), target 0x20340, r0=`[sp-140]`, r1=`0x164498`, r2=`0xd`, r3=`ret(0x20340)`.
  Call (`bl` at 0x6d3e0): (unknown callee), target 0x20340, r0=`[sp-140]`, r1=`0x1644a8`, r2=`0xe`, r3=`ret(0x20340)`.
  Call (`bl` at 0x6d40c): (unknown callee), target 0x20340, r0=`[sp-140]`, r1=`0x1644b8`, r2=`0x8`, r3=`ret(0x20340)`.
  Call (`bl` at 0x6d438): (unknown callee), target 0x20340, r0=`[sp-140]`, r1=`0x1644c4`, r2=`0x4`, r3=`ret(0x20340)`.
  Call (`bl` at 0x6d464): (unknown callee), target 0x20340, r0=`[sp-140]`, r1=`0x1644cc`, r2=`0x9`, r3=`ret(0x20340)`.
  Call (`bl` at 0x6d490): (unknown callee), target 0x20340, r0=`[sp-140]`, r1=`0x1644d8`, r2=`0xe`, r3=`ret(0x20340)`.
  Call (`bl` at 0x6d4bc): (unknown callee), target 0x20340, r0=`[sp-140]`, r1=`0x1644e8`, r2=`0x7`, r3=`ret(0x20340)`.
  Call (`bl` at 0x6d4e0): (unknown callee), target 0x120154, r0=`sp-136`, r1=`unknown`, r2=`unknown`, r3=`sp-136`.
  Call (`bl` at 0x6d514): (unknown callee), target 0x11ff70, r0=`0x0`, r1=`[sp-141]:1`, r2=`sp-136`, r3=`[sp-141]:1`.
  Call (`bl` at 0x6d540): (unknown callee), target 0x20340, r0=`[sp-60]`, r1=`0x1644f0`, r2=`0x7`, r3=`[sp-60]`.
  Call (`bl` at 0x6d56c): (unknown callee), target 0x20340, r0=`[sp-140]`, r1=`0x1644f8`, r2=`0x4`, r3=`ret(0x20340)`.
  Call (`bl` at 0x6d590): (unknown callee), target 0x120154, r0=`sp-136`, r1=`unknown`, r2=`unknown`, r3=`sp-136`.
  Call (`bl` at 0x6d5c4): (unknown callee), target 0x11ff70, r0=`0x0`, r1=`[sp-141]:1`, r2=`sp-136`, r3=`[sp-141]:1`.
  Call (`bl` at 0x6d5f0): (unknown callee), target 0x20340, r0=`[sp-60]`, r1=`0x164500`, r2=`0xa`, r3=`[sp-60]`.
  Call (`bl` at 0x6d614): (unknown callee), target 0x120154, r0=`sp-136`, r1=`unknown`, r2=`unknown`, r3=`sp-136`.
  Call (`bl` at 0x6d648): (unknown callee), target 0x11ff70, r0=`0x0`, r1=`[sp-141]:1`, r2=`sp-136`, r3=`[sp-141]:1`.
  Call (`bl` at 0x6d674): (unknown callee), target 0x20340, r0=`[sp-60]`, r1=`0x164500`, r2=`0xa`, r3=`[sp-60]`.
  Call (`bl` at 0x6d698): (unknown callee), target 0x120154, r0=`sp-136`, r1=`unknown`, r2=`unknown`, r3=`sp-136`.
  Call (`bl` at 0x6d6cc): (unknown callee), target 0x11ff70, r0=`0x0`, r1=`[sp-141]:1`, r2=`sp-136`, r3=`[sp-141]:1`.
  Call (`bl` at 0x6d6f8): (unknown callee), target 0x20340, r0=`[sp-60]`, r1=`0x16450c`, r2=`0xc`, r3=`[sp-60]`.
  Call (`bl` at 0x6d71c): (unknown callee), target 0x120154, r0=`sp-136`, r1=`unknown`, r2=`unknown`, r3=`sp-136`.
  Call (`bl` at 0x6d750): (unknown callee), target 0x11ff70, r0=`0x0`, r1=`[sp-141]:1`, r2=`sp-136`, r3=`[sp-141]:1`.
  Call (`bl` at 0x6d77c): (unknown callee), target 0x20340, r0=`[sp-60]`, r1=`0x16451c`, r2=`0x9`, r3=`[sp-60]`.
  Call (`bl` at 0x6d7a8): (unknown callee), target 0x20340, r0=`[sp-140]`, r1=`0x164528`, r2=`0x9`, r3=`ret(0x20340)`.
  Call (`bl` at 0x6d7cc): (unknown callee), target 0x120154, r0=`sp-136`, r1=`unknown`, r2=`unknown`, r3=`sp-136`.
  Call (`bl` at 0x6d7f8): (unknown callee), target 0x11ff70, r0=`0x0`, r1=`([sp-142]:1 & 0xff)`, r2=`sp-136`, r3=`([sp-142]:1 & 0xff)`.
  Call (`bl` at 0x6d824): (unknown callee), target 0x20340, r0=`[sp-60]`, r1=`0x16451c`, r2=`0x9`, r3=`[sp-60]`.
  Call (`bl` at 0x6d850): (unknown callee), target 0x20340, r0=`[sp-140]`, r1=`0x164528`, r2=`0x9`, r3=`ret(0x20340)`.
  Call (`bl` at 0x6d874): (unknown callee), target 0x120154, r0=`sp-136`, r1=`unknown`, r2=`unknown`, r3=`sp-136`.
  Call (`bl` at 0x6d8a4): (unknown callee), target 0x28230, r0=`0x9`, r1=`unknown`, r2=`0x3182f0`, r3=`0x2`.
  Branch (`ldrls` at 0x6d8c0): taken to (unknown target) when `? <= 0x5a (unsigned, 32-bit)`, falls through to 0x6d8c4 when `? > 0x5a (unsigned, 32-bit)`.
  Omitted-path effects remain unknown at the unresolved transfer (`ldrls` at 0x6d8c0).
  Call (`bl` at 0x6da94): (unknown callee), target 0x29908, r0=`0x164534`, r1=`unknown`, r2=`unknown`, r3=`[0x1a026c]:1`.
  Call (`bl` at 0x6daa0): (unknown callee), target 0x29908, r0=`0x164568`, r1=`unknown`, r2=`unknown`, r3=`[0x1a026c]:1`.
  Call (`bl` at 0x6daac): (unknown callee), target 0x29908, r0=`0x1645a4`, r1=`unknown`, r2=`unknown`, r3=`[0x1a026c]:1`.
  Call (`bl` at 0x6dab8): (unknown callee), target 0x29908, r0=`0x1645dc`, r1=`unknown`, r2=`unknown`, r3=`[0x1a026c]:1`.
  Call (`bl` at 0x6dabc): (unknown callee), target 0x6dc0c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x6daf8): (unknown callee), target 0x243d4, r0=`0x164610`, r1=`unknown`, r2=`unknown`, r3=`[sp-164]`.
  Call (`bl` at 0x6db00): (unknown callee), target 0x243d4, r0=`0x164624`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x6db08): (unknown callee), target 0x243d4, r0=`0x16466c`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x6db10): (unknown callee), target 0x243d4, r0=`0x1646ac`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x6db18): (unknown callee), target 0x243d4, r0=`0x1646ec`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x6db20): (unknown callee), target 0x243d4, r0=`0x164730`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x6db28): (unknown callee), target 0x243d4, r0=`0x164770`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x6db30): (unknown callee), target 0x243d4, r0=`0x1647ac`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x6db44): (unknown callee), target 0x2624c, r0=`0x9`, r1=`[0x1a026c]:1`, r2=`unknown`, r3=`[0x1a026c]:1`.
  Call (`bl` at 0x6db58): (unknown callee), target 0x20d08, r0=`0x3182f8`, r1=`[0x1a026c]:1`, r2=`unknown`, r3=`[0x1a026c]:1`.
  Known-edge recorded r0 = `ret(0x20d08)` (`ldm` at 0x6db60).

## `brickGetType`

- `brickGetType`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6dc0c to 0x6dc2c.
  Established facts:
  Byte load: target `abs 0x1a026c` (`ldrb` at 0x6dc1c).
  Recorded r0 = `[0x1a026c]:1` (`ldm` at 0x6dc24).

## `brickSetType`

- `brickSetType`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6dc2c to 0x6dc58.
  Established facts:
  Byte store: target `abs 0x1a026c`, value `((r0 & 0xff) & 0xff)` (`strb` at 0x6dc4c).
  Recorded r0 = `r0` (`ldm` at 0x6dc50).

## `brickGetSubType`

- `brickGetSubType`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6dc58 to 0x6dc78.
  Established facts:
  Byte load: target `abs 0x1a5e94` (`ldrb` at 0x6dc68).
  Recorded r0 = `[0x1a5e94]:1` (`ldm` at 0x6dc70).

## `brickSetSubType`

- `brickSetSubType`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6dc78 to 0x6dca4.
  Established facts:
  Byte store: target `abs 0x1a5e94`, value `((r0 & 0xff) & 0xff)` (`strb` at 0x6dc98).
  Recorded r0 = `r0` (`ldm` at 0x6dc9c).

## `brickGetFlashType`

- `brickGetFlashType`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6dca4 to 0x6dcc4.
  Established facts:
  Byte load: target `abs 0x1a5e95` (`ldrb` at 0x6dcb4).
  Recorded r0 = `[0x1a5e95]:1` (`ldm` at 0x6dcbc).

## `brickGetSerialSlab`

- `brickGetSerialSlab`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6dcc4 to 0x6debc.
  Established facts:
  Word load: target `(unknown effective address)` (`ldr` at 0x6dd04).
  Call (`bl` at 0x6dd1c): (unknown callee), target 0x120340, r0=`r0`, r1=`sp-40`, r2=`unknown`, r3=`sp-40`.
  Call (`bl` at 0x6dd38): (unknown callee), target 0x6dc0c, r0=`ret(0x120340)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x120340)`.
  Branch (`ldrls` at 0x6dd58): taken to (unknown target) when `[sp-32] <= 0x5 (unsigned, 32-bit)`, falls through to 0x6dd5c when `[sp-32] > 0x5 (unsigned, 32-bit)`.
  Omitted-path effects remain unknown at the unresolved transfer (`ldrls` at 0x6dd58).
  Call (`bl` at 0x6ddd8): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x164828`, r2=`0x21d`, r3=`0x0`.
  Call (`bl` at 0x6de6c): (unknown callee), target 0x20058, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Word store: target `(unknown effective address)`, value `ret(0x20058)` (`str` at 0x6de74, `bl` at 0x6de6c).
  Known-edge recorded r0 = `[sp-44]` (`ldm` at 0x6dea8).

## `brickGetSerial`

- `brickGetSerial`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6debc to 0x6e140.
  Established facts:
  Word load: target `abs 0x1a5e98` (`ldr` at 0x6ded8).
  Call (`bl` at 0x6dee4): (unknown callee), target 0x6e190, r0=`r0`, r1=`r1`, r2=`r2`, r3=`[0x1a5e98]`.
  Call (`bl` at 0x6def8): (unknown callee), target 0x120340, r0=`ret(0x6e190)`, r1=`sp-32`, r2=`sp-32`, r3=`ret(0x6e190)`.
  Byte load: target `abs 0x1a026c` (`ldrb` at 0x6df18).
  Branch (`ldrls` at 0x6df24): taken to (unknown target) when `[0x1a026c]:1-35 <= 0x5a (unsigned, 32-bit)`, falls through to 0x6df28 when `[0x1a026c]:1-35 > 0x5a (unsigned, 32-bit)`.
  Omitted-path effects remain unknown at the unresolved transfer (`ldrls` at 0x6df24).
  Call (`bl` at 0x6e114): (unknown callee), target 0x20058, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Word store: target `abs 0x1a5e98`, value `ret(0x20058)` (`str` at 0x6e11c, `bl` at 0x6e114).
  Known-edge recorded r0 = `unknown` (`ldm` at 0x6e134).

## `brickNumSlots`

- `brickNumSlots`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6e140 to 0x6e190.
  Established facts:
  Call (`bl` at 0x6e150): (unknown callee), target 0x3e274, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Branch (`ble` at 0x6e164): taken to 0x6e178 when `ret(0x3e274) <= 0x0 (signed, 32-bit)`, falls through to 0x6e168 when `ret(0x3e274) > 0x0 (signed, 32-bit)` (source `bl` at 0x6e150).
  Recorded r0 = `unknown` (`ldm` at 0x6e18c).

## `brickSlot`

- `brickSlot`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6e190 to 0x6e1b0.
  Established facts:
  Call (`bl` at 0x6e19c): (unknown callee), target 0x6e140, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Recorded r0 = `ret(0x6e140)-1` (`ldm` at 0x6e1ac).

## `brickSlotLabel`

- `brickSlotLabel`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6e1b0 to 0x6e3b8.
  Established facts:
  Call (`bl` at 0x6e1d0): (unknown callee), target 0x6e140, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Branch (`bgt` at 0x6e1dc): taken to 0x6e1f8 when `ret(0x6e140) > 0x1 (signed, 32-bit)`, falls through to 0x6e1e0 when `ret(0x6e140) <= 0x1 (signed, 32-bit)` (source `bl` at 0x6e1d0).
  Byte store: target `abs 0x1a5e9c`, value `0x0` (`strb` at 0x6e1e8).
  Call (`bl` at 0x6e204): (unknown callee), target 0x3e274, r0=`ret(0x6e140)`, r1=`unknown`, r2=`unknown`, r3=`[sp-24]`.
  Call (`bl` at 0x6e21c): (unknown callee), target 0x3dd9c, r0=`[sp-20]`, r1=`unknown`, r2=`ret(0x3e274)`, r3=`[sp-20]`.
  Call (`bl` at 0x6e274): (unknown callee), target 0x205bc, r0=`0x1a5e9c`, r1=`0x164844`, r2=`unknown`, r3=`ret(0x3dd9c)`.
  Call (`bl` at 0x6e2c4): (unknown callee), target 0x205bc, r0=`0x1a5e9c`, r1=`0x16484c`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x6e2cc): (unknown callee), target 0x3e274, r0=`ret(0x6e140)`, r1=`unknown`, r2=`unknown`, r3=`[sp-24]`.
  Call (`bl` at 0x6e2e4): (unknown callee), target 0x3dd9c, r0=`[sp-20]`, r1=`unknown`, r2=`ret(0x3e274)`, r3=`[sp-20]`.
  Call (`bl` at 0x6e33c): (unknown callee), target 0x205bc, r0=`0x1a5e9c`, r1=`0x164854`, r2=`unknown`, r3=`ret(0x3dd9c)`.
  Call (`bl` at 0x6e38c): (unknown callee), target 0x205bc, r0=`0x1a5e9c`, r1=`0x16484c`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `0x1a5e9c` (`ldm` at 0x6e3a0).

## `bladeInit`

- `bladeInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3db34 to 0x3dce4.
  Established facts:
  Word store: target `abs 0x19e6e0`, value `r0` (`str` at 0x3db50).
  Call (`bl` at 0x3db80): (unknown callee), target 0x204d8, r0=`[0x19e6e0]`, r1=`0x3c`, r2=`[0x19e6e0]`, r3=`0x19e6e0`.
  Word store: target `abs 0x1a5ab0`, value `ret(0x204d8)` (`str` at 0x3db88, `bl` at 0x3db80).
  Call (`bl` at 0x3dbcc): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x15a8a8`, r2=`0x93`, r3=`0x0`.
  Byte store: target `(unknown effective address)`, value `0x0` (`strb` at 0x3dc10).
  Call (`bl` at 0x3dc40): (unknown callee), target 0x28230, r0=`[sp-32]+180`, r1=`unknown`, r2=`unknown`, r3=`[sp-32]+180`.
  Call (`bl` at 0x3dc6c): (unknown callee), target 0x204d8, r0=`[0x19e6e0]`, r1=`0x4`, r2=`unknown`, r3=`0x19e6e0`.
  Call (`bl` at 0x3dcac): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x15a8a8`, r2=`0x9e`, r3=`0x0`.
  Call (`bl` at 0x3dcb4): (unknown callee), target 0x41814, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3dcb8): (unknown callee), target 0xe3638, r0=`ret(0x41814)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `0x0` (`ldm` at 0x3dcc8).

## `bladeSetSlotConfig`

- `bladeSetSlotConfig`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3dce4 to 0x3dd9c.
  Established facts:
  Word store: target `(unknown effective address)`, value `[sp-24]` (`str` at 0x3dd54).
  Word store: target `(unknown effective address)`, value `[sp-28]` (`str` at 0x3dd7c).
  Recorded r0 = `[sp-32]` (`ldm` at 0x3dd90).

## `bladeSlotName`

- `bladeSlotName`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3dd9c to 0x3de44.
  Established facts:
  Word load: target `abs 0x19e6e0` (`ldr` at 0x3ddc4).
  Branch (`bgt` at 0x3ddcc): taken to 0x3ddd4 when `r0 > [0x19e6e0] (signed, 32-bit)`, falls through to 0x3ddd0 when `r0 <= [0x19e6e0] (signed, 32-bit)`.
  Word load: target `(unknown effective address)` (`ldr` at 0x3de20).
  Recorded r0 = `unknown` (`ldm` at 0x3de30).

## `bladeName`

- `bladeName`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3de44 to 0x3de6c.
  Established facts:
  Call (`bl` at 0x3de5c): (unknown callee), target 0x402b4, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Recorded r0 = `ret(0x402b4)` (`ldm` at 0x3de68).
