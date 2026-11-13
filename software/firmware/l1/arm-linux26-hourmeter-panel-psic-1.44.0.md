# ARM Linux 2.6 hourmeter, panel and PSIC routines in 1.44.0

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
In the unreached-range sentences, any path means a path represented in the reported analysis graph.
Unknown computed targets remain unresolved. Range ends are exclusive.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-hourmeter-panel-psic-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register
state from the analysis rather than a C return type.

Groups summarize static records by printed access shape. Address order need not describe one execution path.

## `hourmeterTaskInit`

- `hourmeterTaskInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4bcc4 to 0x4bd40.
  Established facts:
  The instructions from 0x4bd2c to 0x4bd40 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x4bce4): (unknown callee), target 0x22b84, r0=`0x1a5ba8`, r1=`0x15f21c`, r2=`0x0`, r3=`0x9`.
  Call (`bl` at 0x4bd20): (unknown callee), target 0x22540, r0=`0x1a5bfc`, r1=`0x15f228`, r2=`0x4bd40`, r3=`0x1`.
  Recorded r0 = `ret(0x22540)` (`ldm` at 0x4bd28, `bl` at 0x4bd20).

## `hourmeterUpdate`

- `hourmeterUpdate`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4bd40 to 0x4bf70.
  Established facts:
  Call (`bl` at 0x4bd78): (unknown callee), target 0x6e190, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x4bd88): (unknown callee), target 0x81b18, r0=`ret(0x6e190)`, r1=`0x24`, r2=`unknown`, r3=`ret(0x6e190)`.
  Call (`bl` at 0x4bda0): (unknown callee), target 0x28230, r0=`0xc9`, r1=`unknown`, r2=`unknown`, r3=`ret(0x81b18)`.
  Call (`bl` at 0x4bda8): (unknown callee), target 0x6e190, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x4bdb8): (unknown callee), target 0x81b18, r0=`ret(0x6e190)`, r1=`0x25`, r2=`unknown`, r3=`ret(0x6e190)`.
  Call (`bl` at 0x4bdd0): (unknown callee), target 0x28230, r0=`0xca`, r1=`unknown`, r2=`unknown`, r3=`ret(0x81b18)`.
  Call (`bl` at 0x4bdd8): (unknown callee), target 0x5572c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x4be44): (unknown callee), target 0x2624c, r0=`0xc9`, r1=`[sp-28]`, r2=`[sp-36]+2`, r3=`unknown`.
  Call (`bl` at 0x4be50): (unknown callee), target 0x2624c, r0=`0xca`, r1=`[sp-32]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x4be78): (unknown callee), target 0x4ed88, r0=`0x0`, r1=`0x4`, r2=`unknown`, r3=`0x0`.
  Call (`bl` at 0x4be84): (unknown callee), target 0x6e190, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x4be98): (unknown callee), target 0x818a0, r0=`ret(0x6e190)`, r1=`0x24`, r2=`[sp-28]`, r3=`ret(0x6e190)`.
  Call (`bl` at 0x4be9c): (unknown callee), target 0x6e190, r0=`ret(0x818a0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x4beb0): (unknown callee), target 0x818a0, r0=`ret(0x6e190)`, r1=`0x25`, r2=`[sp-32]`, r3=`ret(0x6e190)`.
  Call (`bl` at 0x4beb8): (unknown callee), target 0x6e190, r0=`ret(0x5572c)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x5572c)`.
  Call (`bl` at 0x4bec8): (unknown callee), target 0x81b18, r0=`ret(0x6e190)`, r1=`0x24`, r2=`unknown`, r3=`ret(0x6e190)`.
  Call (`bl` at 0x4bee0): (unknown callee), target 0x2624c, r0=`0xc9`, r1=`[sp-28]`, r2=`unknown`, r3=`ret(0x81b18)`.
  Call (`bl` at 0x4bee4): (unknown callee), target 0x6e190, r0=`ret(0x2624c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x4bef8): (unknown callee), target 0x818a0, r0=`ret(0x6e190)`, r1=`0x24`, r2=`0x0`, r3=`ret(0x6e190)`.
  Call (`bl` at 0x4befc): (unknown callee), target 0x6e190, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x4bf0c): (unknown callee), target 0x81b18, r0=`ret(0x6e190)`, r1=`0x25`, r2=`unknown`, r3=`ret(0x6e190)`.
  Call (`bl` at 0x4bf24): (unknown callee), target 0x2624c, r0=`0xca`, r1=`[sp-32]`, r2=`unknown`, r3=`ret(0x81b18)`.
  Call (`bl` at 0x4bf28): (unknown callee), target 0x6e190, r0=`ret(0x2624c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x4bf3c): (unknown callee), target 0x818a0, r0=`ret(0x6e190)`, r1=`0x25`, r2=`0x0`, r3=`ret(0x6e190)`.
  Call (`bl` at 0x4bf64): (unknown callee), target 0x4bf70, r0=`0xc8`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  The returns section reports no exit for this span.

## `hourmeterSet`

- `hourmeterSet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4c3ec to 0x4c4a4.
  Established facts:
  Call (`bl` at 0x4c448): (unknown callee), target 0x6e190, r0=`r0`, r1=`r1`, r2=`unknown`, r3=`[sp-20]`.
  Call (`bl` at 0x4c45c): (unknown callee), target 0x818a0, r0=`ret(0x6e190)`, r1=`0x24`, r2=`[sp-28]`, r3=`ret(0x6e190)`.
  Call (`bl` at 0x4c468): (unknown callee), target 0x2624c, r0=`0xc9`, r1=`[sp-28]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x4c470): (unknown callee), target 0x6e190, r0=`r0`, r1=`r1`, r2=`unknown`, r3=`[sp-20]`.
  Call (`bl` at 0x4c484): (unknown callee), target 0x818a0, r0=`ret(0x6e190)`, r1=`0x25`, r2=`[sp-28]`, r3=`ret(0x6e190)`.
  Call (`bl` at 0x4c490): (unknown callee), target 0x2624c, r0=`0xca`, r1=`[sp-28]`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `0x0` (`ldm` at 0x4c4a0).

## `hourmeterSetLong`

- `hourmeterSetLong`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4c4a4 to 0x4c5a0.
  Established facts:
  Call (`bl` at 0x4c544): (unknown callee), target 0x6e190, r0=`r0`, r1=`r1`, r2=`unknown`, r3=`[sp-20]`.
  Call (`bl` at 0x4c558): (unknown callee), target 0x818a0, r0=`ret(0x6e190)`, r1=`0x24`, r2=`[sp-36]`, r3=`ret(0x6e190)`.
  Call (`bl` at 0x4c564): (unknown callee), target 0x2624c, r0=`0xc9`, r1=`[sp-36]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x4c56c): (unknown callee), target 0x6e190, r0=`r0`, r1=`r1`, r2=`unknown`, r3=`[sp-20]`.
  Call (`bl` at 0x4c580): (unknown callee), target 0x818a0, r0=`ret(0x6e190)`, r1=`0x25`, r2=`[sp-36]`, r3=`ret(0x6e190)`.
  Call (`bl` at 0x4c58c): (unknown callee), target 0x2624c, r0=`0xca`, r1=`[sp-36]`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `0x0` (`ldm` at 0x4c59c).

## `panelProbe`

- `panelProbe`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x703e8 to 0x70560.
  Established facts:
  The instructions from 0x70534 to 0x70560 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x70428): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x16507c`, r2=`0xb7`, r3=`0x0`.
  Call (`bl` at 0x7042c): (unknown callee), target 0x6e190, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x70448): (unknown callee), target 0x125b8c, r0=`ret(0x6e190)`, r1=`[sp-17]:1`, r2=`sp-25`, r3=`ret(0x6e190)`.
  Call (`bl` at 0x70464): (unknown callee), target 0x21ce0, r0=`ret(0x125b8c)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x125b8c)`.
  Call (`bl` at 0x70488): (unknown callee), target 0x22070, r0=`0x1`, r1=`0x16507c`, r2=`0xbd`, r3=`0x0`.
  Call (`bl` at 0x7048c): (unknown callee), target 0x6e190, r0=`ret(0x22070)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x704a0): (unknown callee), target 0x124cb8, r0=`ret(0x6e190)`, r1=`[sp-17]:1`, r2=`[sp-17]:1`, r3=`ret(0x6e190)`.
  Byte store: target `abs 0x1a0270`, value `([sp-17]:1 & 0xff)` (`strb` at 0x704b0).
  Byte store: target `abs 0x1a6115`, value `0x1` (`strb` at 0x704bc).
  Byte store: target `abs 0x1a616a`, value `0x10` (`strb` at 0x704c8).
  Byte store: target `abs 0x1a616b`, value `0x0` (`strb` at 0x704d4).
  Byte store: target `abs 0x1a616c`, value `0x0` (`strb` at 0x704e0).
  Byte store: target `abs 0x1a616d`, value `0x0` (`strb` at 0x704ec).
  Word store: target `abs 0x1a6170`, value `0x1a6117` (`str` at 0x704f8).
  Word store: target `abs 0x1a6174`, value `0x71ab4` (`str` at 0x70504).
  Word store: target `abs 0x1a6178`, value `0x71b14` (`str` at 0x70510).
  Word store: target `abs 0x1a617c`, value `0x71b3c` (`str` at 0x7051c).
  Word store: target `abs 0x1a6180`, value `0x71b70` (`str` at 0x70528).
  Recorded r0 = `unknown` (`ldm` at 0x70530).

## `panelInt`

- `panelInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x718f8 to 0x71ab4.
  Established facts:
  The instructions from 0x71a8c to 0x71ab4 are not reached by any path from the entry, and their branches are not listed.
  Byte load: target `abs 0x1a0270` (`ldrb` at 0x71930).
  Branch (`beq` at 0x71938): taken to 0x71960 when `(r1 & 0xff) == [0x1a0270]:1`, falls through to 0x7193c when `(r1 & 0xff) != [0x1a0270]:1`.
  Call (`bl` at 0x7195c): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x16507c`, r2=`0x390`, r3=`0x0`.
  Call (`bl` at 0x71980): (unknown callee), target 0x5572c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x71994): (unknown callee), target 0x28230, r0=`0x77`, r1=`unknown`, r2=`unknown`, r3=`ret(0x5572c)`.
  Call (`bl` at 0x719bc): (unknown callee), target 0x22070, r0=`0x6`, r1=`0x16507c`, r2=`0x39c`, r3=`0x0`.
  Call (`bl` at 0x719cc): (unknown callee), target 0x706a0, r0=`0x32`, r1=`0x1651c4`, r2=`0x1`, r3=`unknown`.
  Call (`mov` at 0x719e4): (unknown callee), target (unknown target), r0=`0x5`, r1=`unknown`, r2=`unknown`, r3=`[0x31a5fc]`.
  Call (`bl` at 0x71a00): (unknown callee), target 0x21ce0, r0=`ret(unknown)`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`bl` at 0x71a24): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x16507c`, r2=`0x3a0`, r3=`0x0`.
  Call (`mov` at 0x71a40): (unknown callee), target (unknown target), r0=`0x1`, r1=`0x5`, r2=`unknown`, r3=`[0x31a600]`.
  Call (`bl` at 0x71a5c): (unknown callee), target 0x21ce0, r0=`ret(unknown)`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`bl` at 0x71a80): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x16507c`, r2=`0x3a3`, r3=`0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x71a88).

## `pcf2119_Init_stub`

- `pcf2119_Init_stub`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10b338 to 0x10b350.
  Established facts:
  Recorded r0 = `r0` (`ldm` at 0x10b34c).

## `pcf2119_Init_f`

- `pcf2119_Init_f`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x129a64 to 0x129b0c.
  Established facts:
  The instructions from 0x129af4 to 0x129b0c are not reached by any path from the entry, and their branches are not listed.
  Grouped accesses: 3 reported byte store records with printed base expression `[sp-20]` and nonnegative byte-aligned offsets, including target `[sp-20] +0x3`, value `0x30` (`strb` at 0x129a8c), target `[sp-20] +0x4`, value `0x1` (`strb` at 0x129a98), target `[sp-20] +0x5`, value `unknown` (`strb` at 0x129aa4).
  Grouped accesses: 5 reported word store records with printed base expression `[sp-20]` and nonnegative word-aligned offsets, including target `[sp-20] +0x8`, value `0x31e44b` (`str` at 0x129ab0), target `[sp-20] +0xc`, value `0x129b30` (`str` at 0x129abc), target `[sp-20] +0x10`, value `0x129d60` (`str` at 0x129ac8).
  Word store: target `abs 0x31e444`, value `[sp-20]` (`str` at 0x129aec).
  Recorded r0 = `r0` (`ldm` at 0x129af0).

## `pcf2119_Setup`

- `pcf2119_Setup`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x129b0c to 0x129b30.
  Established facts:
  The instructions from 0x129b28 to 0x129b30 are not reached by any path from the entry, and their branches are not listed.
  Word store: target `abs 0x1a04c4`, value `0x129a64` (`str` at 0x129b20).
  Recorded r0 = `r0` (`ldm` at 0x129b24).

## `psicInit`

- `psicInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12af68 to 0x12b1ec.
  Established facts:
  The instructions from 0x12b1d0 to 0x12b1ec are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x12af98): (unknown callee), target 0x6e140, r0=`r0`, r1=`r1`, r2=`[0x1a0840]`, r3=`0x314ffc`.
  Word store: target `abs 0x1a0844`, value `ret(0x6e140)` (`str` at 0x12afa4, `bl` at 0x12af98).
  Call (`bl` at 0x12afd0): (unknown callee), target 0x204d8, r0=`[sp-40]`, r1=`0x1`, r2=`[0x1a0844]`, r3=`unknown`.
  Word store: target `abs 0x1a0840`, value `ret(0x204d8)` (`str` at 0x12afd8, `bl` at 0x12afd0).
  Call (`bl` at 0x12b008): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x19045c`, r2=`0x68`, r3=`0x0`.
  Call (`bl` at 0x12b058): (unknown callee), target 0x204d8, r0=`[[sp-36]+4]`, r1=`0x1`, r2=`[sp-36]`, r3=`[sp-36]`.
  Call (`bl` at 0x12b080): (unknown callee), target 0x6e1b0, r0=`[sp-24]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x12b08c): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x12b0d4): (unknown callee), target 0x203f4, r0=`[sp-48]`, r1=`[sp-40]`, r2=`[sp-44]`, r3=`unknown`.
  Call (`bl` at 0x12b100): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x19045c`, r2=`0x74`, r3=`0x0`.
  Call (`bl` at 0x12b118): (unknown callee), target 0x20550, r0=`[[sp-36]+8]`, r1=`0x1`, r2=`[[sp-36]+4]`, r3=`[sp-36]`.
  Call (`bl` at 0x12b138): (unknown callee), target 0x6e140, r0=`ret(0x20550)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x12b148): (unknown callee), target 0x6e190, r0=`ret(0x6e140)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x6e140)`.
  Call (`bl` at 0x12b1c4): (unknown callee), target 0x12bf04, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x12b1cc).

## `psicRemove`

- `psicRemove`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12b1ec to 0x12b2b8.
  Established facts:
  The instructions from 0x12b2b0 to 0x12b2b8 are not reached by any path from the entry, and their branches are not listed.
  Grouped accesses: 3 reported word store records with printed base expression `[sp-24]` and nonnegative word-aligned offsets, including target `[sp-24] +0x4`, value `0x0` (`str` at 0x12b280), target `[sp-24] +0x0`, value `0x0` (`str` at 0x12b28c), target `[sp-24] +0x8`, value `0x0` (`str` at 0x12b2a4).
  Call (`bl` at 0x12b298): (unknown callee), target 0x20514, r0=`[[sp-24]+8]`, r1=`r1`, r2=`[sp-24]`, r3=`[sp-24]`.
  Recorded r0 = `unknown` (`ldm` at 0x12b2ac).

## `psicWrite`

- `psicWrite`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12b5d0 to 0x12b734.
  Established facts:
  The instructions from 0x12b728 to 0x12b730 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x12b660): (unknown callee), target 0x6e140, r0=`r0`, r1=`r1`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x12b670): (unknown callee), target 0x6e190, r0=`ret(0x6e140)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x6e140)`.
  Grouped accesses: 3 reported byte load records with printed base expression `[sp-24]` and nonnegative byte-aligned offsets, including target `[sp-24] +0x2` (`ldrb` at 0x12b68c), target `[sp-24] +0x2` (`ldrb` at 0x12b6dc), target `[sp-24] +0x3` (`ldrb` at 0x12b6f8).
  Call (`bl` at 0x12b710): (unknown callee), target 0x12c16c, r0=`[sp-40]`, r1=`[[sp-24]+3]:1`, r2=`0xa`, r3=`[sp-25]:1`.
  Recorded r0 = `unknown` (`ldm` at 0x12b724).

## `psicRead`

- `psicRead`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12b734 to 0x12b890.
  Established facts:
  The instructions from 0x12b884 to 0x12b88c are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x12b7c0): (unknown callee), target 0x6e140, r0=`r0`, r1=`r1`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x12b7d0): (unknown callee), target 0x6e190, r0=`ret(0x6e140)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x6e140)`.
  Grouped accesses: 3 reported byte load records with printed base expression `[sp-24]` and nonnegative byte-aligned offsets, including target `[sp-24] +0x2` (`ldrb` at 0x12b7ec), target `[sp-24] +0x2` (`ldrb` at 0x12b83c), target `[sp-24] +0x3` (`ldrb` at 0x12b858).
  Call (`bl` at 0x12b86c): (unknown callee), target 0x12c354, r0=`[sp-40]`, r1=`[[sp-24]+3]:1`, r2=`0xa`, r3=`[sp-28]`.
  Recorded r0 = `unknown` (`ldm` at 0x12b880).

## `psicIntSet`

- `psicIntSet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12b890 to 0x12b9f4.
  Established facts:
  The instructions from 0x12b9e8 to 0x12b9f0 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x12b920): (unknown callee), target 0x6e140, r0=`r0`, r1=`r1`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x12b930): (unknown callee), target 0x6e190, r0=`ret(0x6e140)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x6e140)`.
  Grouped accesses: 3 reported byte load records with printed base expression `[sp-24]` and nonnegative byte-aligned offsets, including target `[sp-24] +0x2` (`ldrb` at 0x12b94c), target `[sp-24] +0x2` (`ldrb` at 0x12b99c), target `[sp-24] +0x3` (`ldrb` at 0x12b9b8).
  Call (`bl` at 0x12b9d0): (unknown callee), target 0x12c16c, r0=`[sp-40]`, r1=`[[sp-24]+3]:1`, r2=`0xc`, r3=`[sp-25]:1`.
  Recorded r0 = `unknown` (`ldm` at 0x12b9e4).

## `psicIntGet`

- `psicIntGet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12b9f4 to 0x12bb50.
  Established facts:
  The instructions from 0x12bb44 to 0x12bb4c are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x12ba80): (unknown callee), target 0x6e140, r0=`r0`, r1=`r1`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x12ba90): (unknown callee), target 0x6e190, r0=`ret(0x6e140)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x6e140)`.
  Grouped accesses: 3 reported byte load records with printed base expression `[sp-24]` and nonnegative byte-aligned offsets, including target `[sp-24] +0x2` (`ldrb` at 0x12baac), target `[sp-24] +0x2` (`ldrb` at 0x12bafc), target `[sp-24] +0x3` (`ldrb` at 0x12bb18).
  Call (`bl` at 0x12bb2c): (unknown callee), target 0x12c354, r0=`[sp-40]`, r1=`[[sp-24]+3]:1`, r2=`0xc`, r3=`[sp-28]`.
  Recorded r0 = `unknown` (`ldm` at 0x12bb40).
