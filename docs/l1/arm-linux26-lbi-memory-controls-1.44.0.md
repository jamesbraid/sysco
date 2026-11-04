# ARM Linux 2.6 memory control routines in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-lbi-memory-controls-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register
state from the analysis rather than a C return type.


Groups summarize static access records by printed shape. Instruction-address order can include different control-flow paths.

## `LBIInit`

- `LBIInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x87038 to 0x87268.
  Established facts:
  Decoded words in [0x8724c, 0x87254), [0x87258, 0x8725c), [0x87260, 0x87268) are not reached from the entry by known edges.
  Call (`bl` at 0x87094): (unknown callee), target 0x9171c, r0=`r0`, r1=`0x1a63cc`, r2=`[sp-160]`, r3=`[0x1a6414]`.
  Word store: target `abs 0x1a6414`, value `ret(0x9171c)` (`str` at 0x870a0, `bl` at 0x87094).
  Call (`bl` at 0x870a8): (unknown callee), target 0x872c8, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x870c4): (unknown callee), target 0x123480, r0=`r0`, r1=`0x1a63cc`, r2=`[sp-160]`, r3=`[sp-28]`.
  Call (`bl` at 0x870c8): (unknown callee), target 0x6e140, r0=`ret(0x123480)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x870d8): (unknown callee), target 0x3e38c, r0=`ret(0x6e140)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x6e140)`.
  Call (`bl` at 0x870ec): (unknown callee), target 0x6e190, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x87110): (unknown callee), target 0x124ff4, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x8711c): (unknown callee), target 0x149840, r0=`[sp-20]`, r1=`[sp-28]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x87144): (unknown callee), target 0x14991c, r0=`[sp-20]`, r1=`0x0`, r2=`0x0`, r3=`ret(0x149840)`.
  Call (`bl` at 0x87164): (unknown callee), target 0x13b298, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x14991c)`.
  Call (`bl` at 0x87188): (unknown callee), target 0x134634, r0=`ret(0x13b298)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13b298)`.
  Call (`bl` at 0x8719c): (unknown callee), target 0x149b04, r0=`sp-152`, r1=`sp-148`, r2=`sp-148`, r3=`sp-152`.
  Call (`bl` at 0x871ac): (unknown callee), target 0x1346fc, r0=`ret(0x149b04)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x149b04)`.
  Call (`bl` at 0x871b4): (unknown callee), target 0x87564, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x871c8): (unknown callee), target 0x29908, r0=`0x167344`, r1=`ret(0x87564)`, r2=`[sp-152]`, r3=`ret(0x87564)`.
  Call (`bl` at 0x871d0): (unknown callee), target 0x872c8, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x871f8): (unknown callee), target 0x1346fc, r0=`ret(0x149b04)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x149b04)`.
  Halfword load: target `abs 0x1a63b0` (`ldrh` at 0x87214).
  Halfword store: target `abs 0x1a63b0`, value `unknown` (`strh` at 0x8721c).
  Recorded r0 = `unknown` (`ldm` at 0x87248).

## `LBIRemove`

- `LBIRemove`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x87268 to 0x872c8.
  Established facts:
  Decoded words in [0x872c0, 0x872c8) are not reached from the entry by known edges.
  Halfword store: target `(unknown effective address)`, value `unknown` (`strh` at 0x87290).
  Word store: target `(unknown effective address)`, value `0x0` (`str` at 0x872a0).
  Call (`bl` at 0x872b8): (unknown callee), target 0x149840, r0=`unknown`, r1=`0xff`, r2=`[sp-20]`, r3=`unknown`.
  Recorded r0 = `ret(0x149840)` (`ldm` at 0x872bc, `bl` at 0x872b8).

## `LBIRevisionInit`

- `LBIRevisionInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x872c8 to 0x87564.
  Established facts:
  Decoded words in [0x87534, 0x87540), [0x87544, 0x87564) are not reached from the entry by known edges.
  Halfword load: target `[0x1a6414] +0x2a` (`ldrh` at 0x87324).
  Call (`bl` at 0x87350): (unknown callee), target 0x12bf04, r0=`r0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x8738c): (unknown callee), target 0x10ca8c, r0=`unknown`, r1=`unknown`, r2=`[sp-24]`, r3=`unknown`.
  Call (`bl` at 0x873c8): (unknown callee), target 0x10c91c, r0=`unknown`, r1=`0x8`, r2=`0x1a63b2`, r3=`unknown`.
  Call (`bl` at 0x873dc): (unknown callee), target 0x10d2bc, r0=`unknown`, r1=`0x20`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x873f0): (unknown callee), target 0x10d214, r0=`unknown`, r1=`0x80`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x87408): (unknown callee), target 0x10c91c, r0=`unknown`, r1=`0x6`, r2=`0x1a63b2`, r3=`unknown`.
  Call (`bl` at 0x8741c): (unknown callee), target 0x10d214, r0=`unknown`, r1=`0x20`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x87430): (unknown callee), target 0x10d2bc, r0=`unknown`, r1=`0x80`, r2=`unknown`, r3=`unknown`.
  Branch (`bne` at 0x8744c): taken to 0x874a8 when `? != 0x0`, falls through to 0x87450 when `? == 0x0`.
  Grouped accesses: 2 reported word store records with absolute target `0x1a0334`, including target `abs 0x1a0334`, value `unknown` (`str` at 0x874a0), target `abs 0x1a0334`, value `0x7fffff` (`str` at 0x874f8).
  Call (`bl` at 0x8751c): (unknown callee), target 0x818a0, r0=`[sp-24]`, r1=`0x23`, r2=`0x1`, r3=`unknown`.
  Recorded r0 = `[sp-28]` (`ldm` at 0x87530).

## `LBIRevision`

- `LBIRevision`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x875b8 to 0x87644.
  Established facts:
  Decoded words in [0x87640, 0x87644) are not reached from the entry by known edges.
  Call (`bl` at 0x875d0): (unknown callee), target 0x13dfe0, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Word load: target `ret(0x13dfe0) +0x0` (`ldr` at 0x875ec, `bl` at 0x875d0).
  Halfword load: target `(unknown effective address)` (`ldrh` at 0x87628).
  Recorded r0 = `unknown` (`ldm` at 0x87638).

## `LBIName`

- `LBIName`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x87564 to 0x875b8.
  Established facts:
  Decoded words in [0x875b0, 0x875b8) are not reached from the entry by known edges.
  Word load: target `abs 0x1a63cc` (`ldr` at 0x8757c).
  Branch (`bne` at 0x87584): taken to 0x87594 when `[0x1a63cc] != 0x1`, falls through to 0x87588 when `[0x1a63cc] == 0x1`.
  Call (`bl` at 0x87598): (unknown callee), target 0x13ccc0, r0=`r0`, r1=`r1`, r2=`r2`, r3=`[0x1a63cc]`.
  Recorded r0 = `unknown` (`ldm` at 0x875ac).

## `LBIType`

- `LBIType`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x87644 to 0x8767c.
  Established facts:
  Decoded words in [0x87678, 0x8767c) are not reached from the entry by known edges.
  Word load: target `(unknown effective address)` (`ldr` at 0x8766c).
  Recorded r0 = `unknown` (`ldm` at 0x87674).

## `LBIProgram`

- `LBIProgram`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8767c to 0x87ab8.
  Established facts:
  Decoded words in [0x87a98, 0x87aa4), [0x87aa8, 0x87ab0) are not reached from the entry by known edges.
  Call (`bl` at 0x87694): (unknown callee), target 0x13ce4c, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Grouped accesses: 2 reported halfword load records with absolute target `0x1a63b0`, including target `abs 0x1a63b0` (`ldrh` at 0x876d0), target `abs 0x1a63b0` (`ldrh` at 0x8797c).
  Call (`bl` at 0x876f0): (unknown callee), target 0x13e280, r0=`[sp-164]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x877ec): (unknown callee), target 0x87564, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x877fc): (unknown callee), target 0x29908, r0=`0x16736c`, r1=`ret(0x87564)`, r2=`unknown`, r3=`ret(0x87564)`.
  Call (`bl` at 0x8780c): (unknown callee), target 0x10ca8c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x8782c): (unknown callee), target 0x10d17c, r0=`unknown`, r1=`0x0`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x8783c): (unknown callee), target 0x14991c, r0=`[sp-168]`, r1=`0x1`, r2=`0x1`, r3=`unknown`.
  Halfword store: target `(unknown effective address)`, value `unknown` (`strh` at 0x8786c).
  Call (`bl` at 0x87880): (unknown callee), target 0x134634, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[sp-152]`.
  Call (`bl` at 0x87888): (unknown callee), target 0x13b298, r0=`[sp-164]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x878d0): (unknown callee), target 0x149e50, r0=`[sp-24]`, r1=`0x100`, r2=`sp-144`, r3=`sp-144`.
  Call (`bl` at 0x878e0): (unknown callee), target 0x1346fc, r0=`ret(0x149e50)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x149e50)`.
  Call (`bl` at 0x878f0): (unknown callee), target 0x1346fc, r0=`ret(0x149e50)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x149e50)`.
  Call (`bl` at 0x87944): (unknown callee), target 0x14991c, r0=`unknown`, r1=`0x0`, r2=`0x0`, r3=`unknown`.
  Halfword store: target `abs 0x1a63b0`, value `unknown` (`strh` at 0x87984).
  Call (`bl` at 0x8799c): (unknown callee), target 0x13b298, r0=`[sp-164]`, r1=`unknown`, r2=`unknown`, r3=`[sp-152]`.
  Call (`bl` at 0x879d0): (unknown callee), target 0x12cc8c, r0=`0x0`, r1=`0x1`, r2=`sp-144`, r3=`sp-144`.
  Call (`bl` at 0x87a1c): (unknown callee), target 0x872c8, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x87a44): (unknown callee), target 0x10ca8c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x87a64): (unknown callee), target 0x10d17c, r0=`unknown`, r1=`0x1`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x87a80): (unknown callee), target 0x87ab8, r0=`[sp-164]`, r1=`0x2`, r2=`unknown`, r3=`[sp-152]`.
  Recorded r0 = `unknown` (`ldm` at 0x87a94).

## `LBIMemInit`

- `LBIMemInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x87ab8 to 0x8828c.
  Established facts:
  Decoded words in [0x87fd4, 0x881cc), [0x88234, 0x88238), [0x8823c, 0x88280), [0x88284, 0x88288) are not reached from the entry by known edges.
  Call (`bl` at 0x87aec): (unknown callee), target 0x13e280, r0=`r0`, r1=`r1`, r2=`r2`, r3=`0x0`.
  Call (`bl` at 0x87af8): (unknown callee), target 0x13ce4c, r0=`ret(0x13e280)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13e280)`.
  Call (`bl` at 0x87b04): (unknown callee), target 0x134634, r0=`ret(0x13ce4c)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13ce4c)`.
  Call (`bl` at 0x87b48): (unknown callee), target 0x818a0, r0=`unknown`, r1=`0x23`, r2=`0x0`, r3=`unknown`.
  Call (`bl` at 0x87b7c): (unknown callee), target 0x8a94c, r0=`[sp-20]`, r1=`0x0`, r2=`unknown`, r3=`[sp-68]`.
  Grouped accesses: 2 reported word load records with absolute target `0x1a640c`, including target `abs 0x1a640c` (`ldr` at 0x87d44), target `abs 0x1a640c` (`ldr` at 0x87de8).
  Call (`bl` at 0x87d50): (unknown callee), target 0x13d9cc, r0=`[sp-20]`, r1=`[0x1a640c]`, r2=`sp-32`, r3=`0x0`.
  Call (`bl` at 0x87d64): (unknown callee), target 0x87564, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13d9cc)`.
  Call (`bl` at 0x87d88): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x1673e8`, r2=`0x1de`, r3=`0x0`.
  Call (`bl` at 0x87d9c): (unknown callee), target 0x8828c, r0=`0x12c`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13d9cc)`.
  Call (`bl` at 0x87df4): (unknown callee), target 0x13d9cc, r0=`[sp-20]`, r1=`[0x1a640c]`, r2=`sp-32`, r3=`sp-32`.
  Call (`bl` at 0x87e08): (unknown callee), target 0x87564, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13d9cc)`.
  Call (`bl` at 0x87e2c): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x1673e8`, r2=`0x1ee`, r3=`0x0`.
  Branch (`ldrls` at 0x87fcc): taken to (unknown target) when `? <= 0x3 (unsigned, 32-bit)`, falls through to 0x87fd0 when `? > 0x3 (unsigned, 32-bit)`.
  Omitted-path effects remain unknown at the unresolved transfer (`ldrls` at 0x87fcc).
  Call (`bl` at 0x88214): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x1673e8`, r2=`0x210`, r3=`0x0`.
  Call (`bl` at 0x88220): (unknown callee), target 0x1346fc, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Known-edge recorded r0 = `[sp-56]` (`ldm` at 0x88230).

## `LBIMemRead`

- `LBIMemRead`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x882b4 to 0x88e24.
  Established facts:
  Decoded words in [0x88db0, 0x88dd0), [0x88dd4, 0x88df0), [0x88df4, 0x88e18), [0x88e1c, 0x88e24) are not reached from the entry by known edges.
  Call (`bl` at 0x882d4): (unknown callee), target 0x13ce4c, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Branch (`bne` at 0x88308): taken to 0x8850c when `[0x1a63cc] != 0x1`, falls through to 0x8830c when `[0x1a63cc] == 0x1`.
  Grouped accesses: 2 reported halfword store records with printed base expression `[0x1a6414]` and nonnegative halfword-aligned offsets, including target `[0x1a6414] +0x62`, value `([sp-36] & 0xffff)` (`strh` at 0x8832c), target `[0x1a6414] +0x64`, value `unknown` (`strh` at 0x88348).
  Grouped accesses: 5 reported halfword load records with printed base expression `[0x1a6414]` and nonnegative halfword-aligned offsets, including target `[0x1a6414] +0x64` (`ldrh` at 0x8835c), target `[0x1a6414] +0x62` (`ldrh` at 0x8837c), target `[0x1a6414] +0x66` (`ldrh` at 0x88428).
  Call (`bl` at 0x883b4): (unknown callee), target 0x87564, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`[sp-36]`.
  Call (`bl` at 0x883ec): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x1673e8`, r2=`0x237`, r3=`0x0`.
  Call (`bl` at 0x884bc): (unknown callee), target 0x87564, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`[sp-36]`.
  Call (`bl` at 0x884fc): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x1673e8`, r2=`0x242`, r3=`0x0`.
  Call (`bl` at 0x8852c): (unknown callee), target 0x13e280, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x8858c): (unknown callee), target 0x6e140, r0=`unknown`, r1=`unknown`, r2=`0x1a63b2`, r3=`unknown`.
  Call (`bl` at 0x8859c): (unknown callee), target 0x3e38c, r0=`ret(0x6e140)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x6e140)`.
  Call (`bl` at 0x885b0): (unknown callee), target 0x6e190, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x885c4): (unknown callee), target 0x3e38c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x885f0): (unknown callee), target 0x8f0a4, r0=`[sp-68]`, r1=`[sp-72]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x88634): (unknown callee), target 0x134634, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x88734): (unknown callee), target 0x13d9cc, r0=`[sp-20]`, r1=`[0x1a63fc]`, r2=`sp-36`, r3=`0x0`.
  Call (`bl` at 0x88744): (unknown callee), target 0x1346fc, r0=`ret(0x13d9cc)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13d9cc)`.
  Call (`bl` at 0x8874c): (unknown callee), target 0x87564, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x88770): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x1673e8`, r2=`0x267`, r3=`0x0`.
  Call (`bl` at 0x887b0): (unknown callee), target 0x13d9cc, r0=`[sp-20]`, r1=`[0x1a6400]`, r2=`0x0`, r3=`sp-36`.
  Call (`bl` at 0x887c4): (unknown callee), target 0x87564, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13d9cc)`.
  Call (`bl` at 0x887e8): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x1673e8`, r2=`0x26e`, r3=`0x0`.
  Call (`bl` at 0x888e4): (unknown callee), target 0x87564, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`[sp-36]`.
  Call (`bl` at 0x8891c): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x1673e8`, r2=`0x273`, r3=`0x0`.
  Call (`bl` at 0x88970): (unknown callee), target 0x13d9cc, r0=`[sp-20]`, r1=`[0x1a6408]`, r2=`0x0`, r3=`[sp-76]`.
  Call (`bl` at 0x88980): (unknown callee), target 0x1346fc, r0=`ret(0x13d9cc)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13d9cc)`.
  Call (`bl` at 0x88988): (unknown callee), target 0x87564, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x889ac): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x1673e8`, r2=`0x27d`, r3=`0x0`.
  Call (`bl` at 0x889ec): (unknown callee), target 0x13d9cc, r0=`[sp-20]`, r1=`[0x1a6400]`, r2=`0x0`, r3=`sp-36`.
  Call (`bl` at 0x88a00): (unknown callee), target 0x87564, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13d9cc)`.
  Call (`bl` at 0x88a24): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x1673e8`, r2=`0x284`, r3=`0x0`.
  Call (`bl` at 0x88b28): (unknown callee), target 0x87564, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`[sp-36]`.
  Call (`bl` at 0x88b68): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x1673e8`, r2=`0x289`, r3=`0x0`.
  Call (`bl` at 0x88b6c): (unknown callee), target 0x1346fc, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x88da8).

## `LBIMemWrite`

- `LBIMemWrite`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x88e24 to 0x89a60.
  Established facts:
  Decoded words in [0x899e8, 0x89a04), [0x89a08, 0x89a0c), [0x89a10, 0x89a2c), [0x89a30, 0x89a54), [0x89a58, 0x89a60) are not reached from the entry by known edges.
  Call (`bl` at 0x88e4c): (unknown callee), target 0x13ce4c, r0=`r0`, r1=`r1`, r2=`r2`, r3=`0x0`.
  Branch (`bne` at 0x88ea8): taken to 0x890ac when `[0x1a63cc] != 0x1`, falls through to 0x88eac when `[0x1a63cc] == 0x1`.
  Grouped accesses: 3 reported halfword store records with printed base expression `[0x1a6414]` and nonnegative halfword-aligned offsets, including target `[0x1a6414] +0x62`, value `([sp-40] & 0xffff)` (`strh` at 0x88ecc), target `[0x1a6414] +0x64`, value `unknown` (`strh` at 0x88ee8), target `[0x1a6414] +0x66`, value `unknown` (`strh` at 0x88fcc).
  Call (`bl` at 0x88f54): (unknown callee), target 0x87564, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`[sp-40]`.
  Call (`bl` at 0x88f8c): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x1673e8`, r2=`0x2bf`, r3=`0x0`.
  Call (`bl` at 0x8905c): (unknown callee), target 0x87564, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`[sp-40]`.
  Call (`bl` at 0x8909c): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x1673e8`, r2=`0x2ca`, r3=`0x0`.
  Call (`bl` at 0x890cc): (unknown callee), target 0x13e280, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x8914c): (unknown callee), target 0x6e140, r0=`unknown`, r1=`unknown`, r2=`0x1a63b2`, r3=`unknown`.
  Call (`bl` at 0x8915c): (unknown callee), target 0x3e38c, r0=`ret(0x6e140)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x6e140)`.
  Call (`bl` at 0x89170): (unknown callee), target 0x6e190, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x89184): (unknown callee), target 0x3e38c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x891b0): (unknown callee), target 0x8f0a4, r0=`[sp-80]`, r1=`[sp-84]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x894c4): (unknown callee), target 0x134634, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x895c4): (unknown callee), target 0x13d9cc, r0=`[sp-20]`, r1=`[0x1a63fc]`, r2=`sp-40`, r3=`0x0`.
  Call (`bl` at 0x895d8): (unknown callee), target 0x87564, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13d9cc)`.
  Call (`bl` at 0x895fc): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x1673e8`, r2=`0x2fe`, r3=`0x0`.
  Call (`bl` at 0x8963c): (unknown callee), target 0x13d9cc, r0=`[sp-20]`, r1=`[0x1a6400]`, r2=`0x0`, r3=`sp-40`.
  Call (`bl` at 0x89650): (unknown callee), target 0x87564, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13d9cc)`.
  Call (`bl` at 0x89674): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x1673e8`, r2=`0x306`, r3=`0x0`.
  Call (`bl` at 0x89764): (unknown callee), target 0x87564, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`[sp-40]`.
  Call (`bl` at 0x8979c): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x1673e8`, r2=`0x30c`, r3=`0x0`.
  Call (`bl` at 0x897c8): (unknown callee), target 0x13d9cc, r0=`[sp-20]`, r1=`[0x1a6404]`, r2=`[sp-36]`, r3=`[sp-36]`.
  Call (`bl` at 0x897dc): (unknown callee), target 0x87564, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13d9cc)`.
  Call (`bl` at 0x89800): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x1673e8`, r2=`0x316`, r3=`0x0`.
  Call (`bl` at 0x89840): (unknown callee), target 0x13d9cc, r0=`[sp-20]`, r1=`[0x1a6400]`, r2=`0x0`, r3=`sp-40`.
  Call (`bl` at 0x89854): (unknown callee), target 0x87564, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13d9cc)`.
  Call (`bl` at 0x89878): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x1673e8`, r2=`0x31e`, r3=`0x0`.
  Call (`bl` at 0x8997c): (unknown callee), target 0x87564, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`[sp-40]`.
  Call (`bl` at 0x899bc): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x1673e8`, r2=`0x324`, r3=`0x0`.
  Call (`bl` at 0x899c8): (unknown callee), target 0x8a94c, r0=`[sp-20]`, r1=`0x0`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x899cc): (unknown callee), target 0x1346fc, r0=`ret(0x8a94c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x899e0).

## `LBIMemReadToFile`

- `LBIMemReadToFile`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x89a60 to 0x89c14.
  Established facts:
  Decoded words in [0x89c0c, 0x89c14) are not reached from the entry by known edges.
  Call (`bl` at 0x89aa4): (unknown callee), target 0x2046c, r0=`r3`, r1=`0x242`, r2=`0x1a4`, r3=`0x0`.
  Call (`bl` at 0x89acc): (unknown callee), target 0x201fc, r0=`[sp-52]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x2046c)`.
  Call (`bl` at 0x89ae8): (unknown callee), target 0x200b8, r0=`[sp-40]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x201fc)`.
  Call (`bl` at 0x89b64): (unknown callee), target 0x882b4, r0=`[sp-20]`, r1=`[sp-24]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x89b8c): (unknown callee), target 0x2007c, r0=`[sp-40]`, r1=`[sp-56]`, r2=`[sp-48]`, r3=`ret(0x882b4)`.
  Branch (`beq` at 0x89ba4): taken to 0x89bbc when `ret(0x2007c) == [sp-48]`, falls through to 0x89ba8 when `ret(0x2007c) != [sp-48]` (source `bl` at 0x89b8c).
  Call (`bl` at 0x89bb4): (unknown callee), target 0x243d4, r0=`0x1676fc`, r1=`[sp-48]`, r2=`ret(0x2007c)`, r3=`[sp-48]`.
  Call (`bl` at 0x89bec): (unknown callee), target 0x20514, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x89bf4): (unknown callee), target 0x200b8, r0=`[sp-40]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x89c08).

## `LBIMemWriteFromFile`

- `LBIMemWriteFromFile`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x89c14 to 0x89e18.
  Established facts:
  Decoded words in [0x89e14, 0x89e18) are not reached from the entry by known edges.
  Call (`bl` at 0x89c54): (unknown callee), target 0x2046c, r0=`r3`, r1=`0x0`, r2=`r2`, r3=`0x0`.
  Call (`bl` at 0x89c7c): (unknown callee), target 0x201fc, r0=`[sp-52]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x2046c)`.
  Call (`bl` at 0x89c98): (unknown callee), target 0x200b8, r0=`[sp-40]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x201fc)`.
  Call (`bl` at 0x89cc0): (unknown callee), target 0x15023c, r0=`[sp-40]`, r1=`sp-144`, r2=`unknown`, r3=`sp-144`.
  Call (`bl` at 0x89cd8): (unknown callee), target 0x20184, r0=`ret(0x15023c)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x15023c)`.
  Word load: target `ret(0x20184) +0x0` (`ldr` at 0x89ce0, `bl` at 0x89cd8).
  Call (`bl` at 0x89d54): (unknown callee), target 0x20628, r0=`[sp-40]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x89d7c): (unknown callee), target 0x243d4, r0=`0x167718`, r1=`[sp-48]`, r2=`ret(0x20628)`, r3=`[sp-48]`.
  Call (`bl` at 0x89db0): (unknown callee), target 0x88e24, r0=`[sp-20]`, r1=`[sp-24]`, r2=`[sp-56]`, r3=`unknown`.
  Call (`bl` at 0x89df4): (unknown callee), target 0x20514, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x89dfc): (unknown callee), target 0x200b8, r0=`[sp-40]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x89e10).

## `LBILoadPROM`

- `LBILoadPROM`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x89e18 to 0x8a1e4.
  Established facts:
  Decoded words in [0x8a1cc, 0x8a1e4) are not reached from the entry by known edges.
  Call (`bl` at 0x89e40): (unknown callee), target 0x13ce4c, r0=`r0`, r1=`r1`, r2=`r2`, r3=`0xff000000`.
  Call (`bl` at 0x89e58): (unknown callee), target 0x150220, r0=`[sp-24]`, r1=`sp-120`, r2=`unknown`, r3=`sp-120`.
  Call (`bl` at 0x89e74): (unknown callee), target 0x20184, r0=`ret(0x150220)`, r1=`unknown`, r2=`unknown`, r3=`[sp-28]`.
  Call (`bl` at 0x89e80): (unknown callee), target 0x219a4, r0=`[ret(0x20184)]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x20184)`.
  Call (`bl` at 0x89e94): (unknown callee), target 0x205bc, r0=`[sp-28]`, r1=`0x167734`, r2=`ret(0x219a4)`, r3=`ret(0x219a4)`.
  Call (`bl` at 0x89e98): (unknown callee), target 0x20184, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x89eb0): (unknown callee), target 0x13e280, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x150220)`.
  Halfword load: target `(unknown effective address)` (`ldrh` at 0x89f8c).
  Call (`bl` at 0x8a068): (unknown callee), target 0x205bc, r0=`sp-296`, r1=`0x167754`, r2=`[sp-24]`, r3=`[sp-140]`.
  Call (`bl` at 0x8a094): (unknown callee), target 0x205bc, r0=`sp-296`, r1=`0x167754`, r2=`[sp-24]`, r3=`[sp-140]`.
  Call (`bl` at 0x8a0c8): (unknown callee), target 0xc500c, r0=`unknown`, r1=`sp-296`, r2=`[sp-32]`, r3=`unknown`.
  Call (`bl` at 0x8a0dc): (unknown callee), target 0x89c14, r0=`[sp-20]`, r1=`[sp-132]`, r2=`[sp-76]`, r3=`[sp-24]`.
  Call (`bl` at 0x8a0f8): (unknown callee), target 0x219a4, r0=`ret(0x89c14)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x89c14)`.
  Call (`bl` at 0x8a120): (unknown callee), target 0x205bc, r0=`[sp-28]`, r1=`0x167798`, r2=`[sp-24]`, r3=`[sp-140]`.
  Call (`bl` at 0x8a12c): (unknown callee), target 0x13e280, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x89c14)`.
  Call (`bl` at 0x8a138): (unknown callee), target 0x13ce4c, r0=`ret(0x13e280)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13e280)`.
  Call (`bl` at 0x8a160): (unknown callee), target 0x205bc, r0=`[sp-28]`, r1=`0x1677c8`, r2=`[sp-24]`, r3=`[sp-140]`.
  Call (`bl` at 0x8a1a4): (unknown callee), target 0x818a0, r0=`unknown`, r1=`0x23`, r2=`0x1`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x8a1c8).

## `LBIMemPattern`

- `LBIMemPattern`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8a1e4 to 0x8a360.
  Established facts:
  Call (`bl` at 0x8a294): (unknown callee), target 0x8bbf0, r0=`[sp-32]`, r1=`sp-4128`, r2=`[sp-8240]`, r3=`0x0`.
  Call (`bl` at 0x8a2d8): (unknown callee), target 0x88e24, r0=`[sp-20]`, r1=`([sp-24] + [sp-8228])`, r2=`sp-4128`, r3=`[sp-8240]`.
  Recorded r0 = `unknown` (`ldm` at 0x8a34c).

## `LBIMemClear`

- `LBIMemClear`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8a360 to 0x8a520.
  Established facts:
  Call (`bl` at 0x8a38c): (unknown callee), target 0x134634, r0=`r0`, r1=`r1`, r2=`r2`, r3=`0x0`.
  Call (`bl` at 0x8a3ac): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x52`, r2=`0x0`, r3=`sp-36`.
  Call (`bl` at 0x8a3c0): (unknown callee), target 0x13fd34, r0=`[sp-20]`, r1=`0x3`, r2=`0x52`, r3=`0x0`.
  Call (`bl` at 0x8a3e8): (unknown callee), target 0x13f880, r0=`[sp-20]`, r1=`0x0`, r2=`0x54`, r3=`ret(0x13fd34)`.
  Call (`bl` at 0x8a448): (unknown callee), target 0x12d210, r0=`[sp-48]`, r1=`0x3`, r2=`sp-36`, r3=`sp-40`.
  Call (`bl` at 0x8a484): (unknown callee), target 0x12d168, r0=`[sp-48]`, r1=`[sp-32]`, r2=`sp-168`, r3=`sp-168`.
  Call (`bl` at 0x8a4e4): (unknown callee), target 0x12d210, r0=`[sp-48]`, r1=`0x4`, r2=`sp-36`, r3=`sp-40`.
  Call (`bl` at 0x8a504): (unknown callee), target 0x8a94c, r0=`[sp-20]`, r1=`0x0`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x8a508): (unknown callee), target 0x1346fc, r0=`ret(0x8a94c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `[sp-172]` (`ldm` at 0x8a518).
