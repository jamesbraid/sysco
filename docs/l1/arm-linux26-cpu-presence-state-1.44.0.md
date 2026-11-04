# ARM Linux 2.6 CPU presence and state routines in 1.44.0

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
values, and brackets denote loaded values. Decimal numbers inside expressions retain tool notation except the address constant noted below. Call candidates use the assumed ARM AAPCS
register locations r0 through r3 with unknown arity. Custom calling conventions remain unknown.
Call lines include every reported call instruction and every printed register slot.
Unknown values and predicates include unsupported instruction effects and merged paths.
Numeric call targets have unknown callee names in the flat-image analysis. Role: unknown. Pointee meanings remain unknown. Selected access and branch facts below cover known control-flow edges. The
full fact sheets retain all reported accesses and implicit stack operations. Separate loads at the same printed address need not observe the same value.
No unresolved computed transfers were reported for these routines.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-cpu-presence-state-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register
state from the analysis rather than a C return type.


The return expression for `IP57_GetCpuMask` uses the hexadecimal constant from its byte access (`ldrb` at 0x84264).

Groups summarize static access records by printed shape. Instruction-address order can include different control-flow paths.

## `cpuCount`

- `cpuCount`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7fbc8 to 0x7fcdc.
  Established facts:
  Decoded words in [0x7fcd4, 0x7fcdc) are not reached from the entry by known edges.
  Call (`bl` at 0x7fbdc): (unknown callee), target 0x6e190, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Branch (`beq` at 0x7fbec): taken to 0x7fcb0 when `ret(0x6e190) == [sp-20]`, falls through to 0x7fbf0 when `ret(0x6e190) != [sp-20]` (source `bl` at 0x7fbdc).
  Call (`bl` at 0x7fbf4): (unknown callee), target 0x3ee14, r0=`[sp-20]`, r1=`unknown`, r2=`ret(0x6e190)`, r3=`[sp-20]`.
  Call (`bl` at 0x7fc08): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3ee14)`.
  Call (`bl` at 0x7fc1c): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x7fc30): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x7fc50): (unknown callee), target 0x3f474, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x7fc70): (unknown callee), target 0x3ec6c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3f474)`.
  Word load: target `abs 0x31a520` (`ldr` at 0x7fc98).
  Word load: target `abs 0x3183f0` (`ldr` at 0x7fcb4).
  Recorded r0 = `unknown` (`ldm` at 0x7fcc4).

## `cpuPresent`

- `cpuPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7fcdc to 0x7fe94.
  Established facts:
  Decoded words in [0x7fe8c, 0x7fe94) are not reached from the entry by known edges.
  Call (`bl` at 0x7fcf8): (unknown callee), target 0x6e190, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r1`.
  Call (`bl` at 0x7fd10): (unknown callee), target 0x3ee14, r0=`[sp-20]`, r1=`unknown`, r2=`ret(0x6e190)`, r3=`[sp-20]`.
  Call (`bl` at 0x7fd24): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3ee14)`.
  Call (`bl` at 0x7fd38): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x7fd4c): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x7fd6c): (unknown callee), target 0x3f474, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x7fd8c): (unknown callee), target 0x3ec6c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3f474)`.
  Call (`mov` at 0x7fdf8): (unknown callee), target (unknown target), r0=`[sp-20]`, r1=`[sp-21]:1`, r2=`[sp-21]:1`, r3=`unknown`.
  Branch (`beq` at 0x7fe20): taken to 0x7fe48 when `[0x3183f8] == 0x0`, falls through to 0x7fe24 when `[0x3183f8] != 0x0`.
  Call (`mov` at 0x7fe38): (unknown callee), target (unknown target), r0=`[sp-21]:1`, r1=`unknown`, r2=`[sp-21]:1`, r3=`[0x3183f8]`.
  Word load: target `abs 0x3183f0` (`ldr` at 0x7fe54).
  Recorded r0 = `unknown` (`ldm` at 0x7fe7c).

## `cpuEnable`

- `cpuEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x80210 to 0x80338.
  Established facts:
  Call (`bl` at 0x80238): (unknown callee), target 0x558d8, r0=`r0`, r1=`r1`, r2=`r2`, r3=`0x0`.
  Branch (`bne` at 0x8025c): taken to 0x80310 when `[sp-21]:1 != 0xff`, falls through to 0x80260 when `[sp-21]:1 == 0xff`.
  Call (`bl` at 0x80264): (unknown callee), target 0x7fbc8, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`[sp-21]:1`.
  Call (`bl` at 0x802ac): (unknown callee), target 0x7fcdc, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x802c8): (unknown callee), target 0x805e8, r0=`[sp-20]`, r1=`[sp-33]:1`, r2=`unknown`, r3=`[sp-33]:1`.
  Call (`bl` at 0x8031c): (unknown callee), target 0x805e8, r0=`[sp-20]`, r1=`[sp-21]:1`, r2=`unknown`, r3=`[sp-21]:1`.
  Recorded r0 = `unknown` (`ldm` at 0x80330).

## `cpuDisable`

- `cpuDisable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x800e8 to 0x80210.
  Established facts:
  Call (`bl` at 0x80110): (unknown callee), target 0x558d8, r0=`r0`, r1=`r1`, r2=`r2`, r3=`0x0`.
  Branch (`bne` at 0x80134): taken to 0x801e8 when `[sp-21]:1 != 0xff`, falls through to 0x80138 when `[sp-21]:1 == 0xff`.
  Call (`bl` at 0x8013c): (unknown callee), target 0x7fbc8, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`[sp-21]:1`.
  Call (`bl` at 0x80184): (unknown callee), target 0x7fcdc, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x801a0): (unknown callee), target 0x80338, r0=`[sp-20]`, r1=`[sp-33]:1`, r2=`unknown`, r3=`[sp-33]:1`.
  Call (`bl` at 0x801f4): (unknown callee), target 0x80338, r0=`[sp-20]`, r1=`[sp-21]:1`, r2=`unknown`, r3=`[sp-21]:1`.
  Recorded r0 = `unknown` (`ldm` at 0x80208).

## `cpuSetState`

- `cpuSetState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x80a78 to 0x80b8c.
  Established facts:
  Branch (`bne` at 0x80aac): taken to 0x80b64 when `(r1 & 0xff) != 0xff`, falls through to 0x80ab0 when `(r1 & 0xff) == 0xff`.
  Call (`bl` at 0x80ab4): (unknown callee), target 0x7fbc8, r0=`r0`, r1=`r1`, r2=`r2`, r3=`(r1 & 0xff)`.
  Call (`bl` at 0x80afc): (unknown callee), target 0x7fcdc, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x80b1c): (unknown callee), target 0x80898, r0=`[sp-20]`, r1=`[sp-33]:1`, r2=`[sp-22]:1`, r3=`[sp-33]:1`.
  Call (`bl` at 0x80b74): (unknown callee), target 0x80898, r0=`r0`, r1=`(r1 & 0xff)`, r2=`(r2 & 0xff)`, r3=`(r1 & 0xff)`.
  Recorded r0 = `unknown` (`ldm` at 0x80b88).

## `cpuState`

- `cpuState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x80b8c to 0x80d68.
  Established facts:
  Decoded words in [0x80d5c, 0x80d60), [0x80d64, 0x80d68) are not reached from the entry by known edges.
  Call (`bl` at 0x80ba8): (unknown callee), target 0x6e190, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r1`.
  Call (`bl` at 0x80bc0): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`ret(0x6e190)`, r3=`[sp-20]`.
  Call (`bl` at 0x80bd4): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x80be8): (unknown callee), target 0x3e58c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x80c08): (unknown callee), target 0x3f474, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x80c28): (unknown callee), target 0x3ec6c, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3f474)`.
  Word load: target `abs 0x31a520` (`ldr` at 0x80c54).
  Call (`bl` at 0x80c88): (unknown callee), target 0x81b18, r0=`[sp-20]`, r1=`[sp-21]:1+10`, r2=`unknown`, r3=`[sp-21]:1+10`.
  Call (`bl` at 0x80c9c): (unknown callee), target 0x3e3ac, r0=`ret(0x6e190)`, r1=`unknown`, r2=`ret(0x6e190)`, r3=`[sp-20]`.
  Word load: target `abs 0x3183f0` (`ldr` at 0x80cd4).
  Call (`bl` at 0x80cec): (unknown callee), target 0x3e3ac, r0=`ret(0x3e3ac)`, r1=`unknown`, r2=`[0x3183f0]`, r3=`[sp-21]:1`.
  Call (`bl` at 0x80d0c): (unknown callee), target 0x81b18, r0=`[sp-28]`, r1=`[sp-32]+10`, r2=`unknown`, r3=`[sp-32]+10`.
  Call (`bl` at 0x80d1c): (unknown callee), target 0x6e190, r0=`ret(0x3e3ac)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e3ac)`.
  Call (`bl` at 0x80d34): (unknown callee), target 0x81b18, r0=`ret(0x6e190)`, r1=`[sp-21]:1+10`, r2=`ret(0x6e190)`, r3=`[sp-21]:1+10`.
  Recorded r0 = `unknown` (`ldm` at 0x80d4c).

## `cpuStateString`

- `cpuStateString`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x80d68 to 0x80e40.
  Established facts:
  Decoded words in [0x80e34, 0x80e40) are not reached from the entry by known edges.
  Call (`bl` at 0x80d90): (unknown callee), target 0x80b8c, r0=`r0`, r1=`(r1 & 0xff)`, r2=`r2`, r3=`(r1 & 0xff)`.
  Branch (`bge` at 0x80db4): taken to 0x80de4 when `ge flags condition (operands unknown)`, falls through to 0x80db8 when `not (ge flags condition (operands unknown))`.
  Recorded r0 = `unknown` (`ldm` at 0x80e30).

## `IP57_CpuPresent`

- `IP57_CpuPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8311c to 0x83198.
  Established facts:
  Decoded words in [0x83194, 0x83198) are not reached from the entry by known edges.
  Call (`bl` at 0x83150): (unknown callee), target 0x28230, r0=`0x7b`, r1=`r1`, r2=`r2`, r3=`unknown`.
  Branch (`bne` at 0x8315c): taken to 0x8316c when `ret(0x28230) != 0x1`, falls through to 0x83160 when `ret(0x28230) == 0x1` (source `bl` at 0x83150).
  Byte load: target `[sp-18]:1 +0x1a62f4` (`ldrb` at 0x83174).
  Recorded r0 = `unknown` (`ldm` at 0x83190).

## `IP57_CpuEnable`

- `IP57_CpuEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8320c to 0x83280.
  Established facts:
  Call (`bl` at 0x8322c): (unknown callee), target 0x8311c, r0=`(r0 & 0xff)`, r1=`r1`, r2=`r2`, r3=`(r0 & 0xff)`.
  Branch (`beq` at 0x83238): taken to 0x83268 when `ret(0x8311c) == 0x0`, falls through to 0x8323c when `ret(0x8311c) != 0x0` (source `bl` at 0x8322c).
  Call (`bl` at 0x83258): (unknown callee), target 0x83444, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x83278).

## `IP57_CpuDisable`

- `IP57_CpuDisable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x83198 to 0x8320c.
  Established facts:
  Call (`bl` at 0x831b8): (unknown callee), target 0x8311c, r0=`(r0 & 0xff)`, r1=`r1`, r2=`r2`, r3=`(r0 & 0xff)`.
  Branch (`beq` at 0x831c4): taken to 0x831f4 when `ret(0x8311c) == 0x0`, falls through to 0x831c8 when `ret(0x8311c) != 0x0` (source `bl` at 0x831b8).
  Call (`bl` at 0x831e4): (unknown callee), target 0x83280, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x83204).

## `IP57_GetCpuMask`

- `IP57_GetCpuMask`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x84244 to 0x84274.
  Established facts:
  Decoded words in [0x84270, 0x84274) are not reached from the entry by known edges.
  Byte load: target `(r0 & 0xff) +0x1a62f4` (`ldrb` at 0x84264).
  Recorded r0 = `[(r0 & 0xff)+0x1a62f4]:1` (`ldm` at 0x8426c).

## `promIceGetCfg`

- `promIceGetCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x612c8 to 0x61304.
  Established facts:
  Decoded words in [0x612fc, 0x61304) are not reached from the entry by known edges.
  Word load: target `abs 0x31a60c` (`ldr` at 0x612e4).
  Word store: target `r0 +0x0`, value `[0x31a60c]` (`str` at 0x612e8).
  Recorded r0 = `[0x31a608]` (`ldm` at 0x612f8).

## `promIceCount`

- `promIceCount`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x61304 to 0x61324.
  Established facts:
  Decoded words in [0x61320, 0x61324) are not reached from the entry by known edges.
  Word load: target `abs 0x31a608` (`ldr` at 0x61314).
  Recorded r0 = `[0x31a608]` (`ldm` at 0x6131c).

## `promIcePresent`

- `promIcePresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x61324 to 0x61388.
  Established facts:
  Decoded words in [0x61380, 0x61388) are not reached from the entry by known edges.
  Branch (`bge` at 0x6134c): taken to 0x6136c when `(r0 & 0xff) >= [0x31a608] (signed, 32-bit)`, falls through to 0x61350 when `(r0 & 0xff) < [0x31a608] (signed, 32-bit)`.
  Word load: target `abs 0x31a610` (`ldr` at 0x61358).
  Byte load: target `((r0 & 0xff) + [0x31a610]) +0x0` (`ldrb` at 0x61360).
  Recorded r0 = `unknown` (`ldm` at 0x6137c).

## `promIceNotPresent`

- `promIceNotPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x61148 to 0x61198.
  Established facts:
  Decoded words in [0x61190, 0x61198) are not reached from the entry by known edges.
  Word load: target `abs 0x31a608` (`ldr` at 0x61168).
  Branch (`bge` at 0x61170): taken to 0x6118c when `(r0 & 0xff) >= [0x31a608] (signed, 32-bit)`, falls through to 0x61174 when `(r0 & 0xff) < [0x31a608] (signed, 32-bit)`.
  Byte store: target `((r0 & 0xff) + [0x31a610]) +0x0`, value `0x0` (`strb` at 0x61188).
  Recorded r0 = `r0` (`ldm` at 0x6118c).
