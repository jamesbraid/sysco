# ARM Linux 2.6 VRAM and node configuration routines in 1.44.0

Read from `6.5.30/usr/cpu/firmware/sysco/l1.bin`, package `6.5.30:l1#6` (version 1.44.0),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0xa936a8`.
Carrier SHA-256: `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
Package SHA-256: `e57ab8ecd48858e39ecdd0050a23e444a491c35e87f9d8ebb48881fc25229a06`.
Inner executable SHA-256: `b33154e53b00531623db6ff365e80b1c1825bc9427961c59fdc132c24f0c819e`.
The ELF identifies 32-bit little-endian ARM. Its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1629656 bytes.
Segment SHA-256: `f6491062ef0a2afd254f8510f82b4814c563c9256691f9a2b393cf6e84ba7fd8`.
Analysed routine spans use the exact ELF function symbols and sizes. They can include
trailing literal words after a return.

`sp` denotes the entry stack pointer. Register expressions such as `r0` denote entry
values, and brackets denote loaded values. Decimal numbers inside expressions retain the
exact tool notation. Call candidates use the assumed ARM AAPCS register locations r0
through r3 with unknown arity. Custom calling conventions remain unknown. Call lines
include every reported call instruction and every printed register slot. Unknown values
and predicates include unsupported instruction effects and merged paths. Numeric call targets have unknown callee
names in the flat-image analysis. Role: unknown. Pointee meanings remain unknown.
Selected access and branch facts below cover known control-flow edges. The full fact
sheets retain all reported accesses and implicit stack operations. Separate loads at the
same printed address need not observe the same value. Computed transfers with unresolved
edges are identified under their routines. Effects on omitted paths remain unknown.
In the unreached-range sentences, any path means a path represented in the reported
analysis graph. Unknown computed targets remain unresolved. Range ends are exclusive.
Undecodable byte ranges remain visible in the fact sheets.
Load records with unknown width describe the reported analysis graph. Feasible execution
and coprocessor behavior remain unknown.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-vram-node-configuration-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register state
from the analysis rather than a C return type. Groups summarize static records by printed
access shape. Address order need not describe one execution path.

## `vramInit`

- `vramInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x81480 to 0x8170c.
  Established facts:
  The instructions from 0x816d8 to 0x8170c are not reached by any path from the entry, and their branches are not listed.
  Word store: target `abs 0x1a62b0`, value `[0x1a62b0]+1` (`str` at 0x814d4).
  Word store: target `abs 0x31a720`, value `[sp-28]` (`str` at 0x815f4).
  Word store: target `abs 0x1a6254`, value `[0x31a720]` (`str` at 0x81614).
  Grouped accesses: 2 reported byte store records with absolute target `0x1a6258`, including target `abs 0x1a6258`, value `0x0` (`strb` at 0x81680), target `abs 0x1a6258`, value `([0x1a6258]:1+1 & 0xff)` (`strb` at 0x816c8).
  Call (`bl` at 0x814c0): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x1667d8`, r2=`0x71`, r3=`0x0`.
  Call (`bl` at 0x814e4): (unknown callee), target 0x20550, r0=`0x1a62b4`, r1=`0x0`, r2=`0x20`, r3=`[0x1a62b0]+1`.
  Call (`bl` at 0x814f0): (unknown callee), target 0x2046c, r0=`0x1667ec`, r1=`0x1002`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x81508): (unknown callee), target 0x20184, r0=`ret(0x2046c)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x2046c)`.
  Call (`bl` at 0x81514): (unknown callee), target 0x219a4, r0=`[ret(0x20184)]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x20184)`.
  Call (`bl` at 0x8151c): (unknown callee), target 0x20184, r0=`ret(0x219a4)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x81554): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x1667d8`, r2=`0x84`, r3=`0x0`.
  Call (`bl` at 0x8157c): (unknown callee), target 0x2016c, r0=`0x0`, r1=`0x8000`, r2=`0x3`, r3=`0x1`.
  Call (`bl` at 0x81590): (unknown callee), target 0x20184, r0=`ret(0x2016c)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x2016c)`.
  Call (`bl` at 0x8159c): (unknown callee), target 0x219a4, r0=`[ret(0x20184)]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x20184)`.
  Call (`bl` at 0x815a4): (unknown callee), target 0x20184, r0=`ret(0x219a4)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x815dc): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x1667d8`, r2=`0x8a`, r3=`0x0`.
  Call (`bl` at 0x815e4): (unknown callee), target 0x200b8, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x81618): (unknown callee), target 0x20fd8, r0=`ret(0x2016c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x81630): (unknown callee), target 0x8170c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x81654): (unknown callee), target 0x203f4, r0=`[0x1a6254]+5260`, r1=`[0x1a6254]`, r2=`0x148c`, r3=`[0x1a6254]+5260`.
  Call (`bl` at 0x81658): (unknown callee), target 0x20fd8, r0=`ret(0x203f4)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x816b4): (unknown callee), target 0x293f8, r0=`unknown`, r1=`unknown`, r2=`0x1a62b4`, r3=`unknown`.
  Branch (`beq` at 0x814a4): taken to 0x814c4 when `[0x1a62b0] == 0x0`, falls through to 0x814a8 when `[0x1a62b0] != 0x0`.
  Branch (`bge` at 0x81504): taken to 0x8155c when `ret(0x2046c) >= 0x0 (signed, 32-bit)`, falls through to 0x81508 when `ret(0x2046c) < 0x0 (signed, 32-bit)` (source `bl` at 0x814f0).
  Branch (`bne` at 0x81604): taken to 0x81618 when `[0x1a6254] != 0x0`, falls through to 0x81608 when `[0x1a6254] == 0x0`.
  Branch (`beq` at 0x81664): taken to 0x816d0 when `ret(0x20fd8) == 0x0`, falls through to 0x81668 when `ret(0x20fd8) != 0x0` (source `bl` at 0x81658).
  Branch (`beq` at 0x81674): taken to 0x816d0 when `[0x1a6258]:1 == 0x0`, falls through to 0x81678 when `[0x1a6258]:1 != 0x0`.
  Branch (`beq` at 0x8169c): taken to 0x816d0 when `unknown == 0x0`, falls through to 0x816a0 when `unknown != 0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x816d4).

## `vramSlotSet`

- `vramSlotSet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x818a0 to 0x81b18.
  Established facts:
  The instructions from 0x818d0 to 0x81af8 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x81b00 to 0x81b18 are not reached by any path from the entry, and their branches are not listed.
  Branch (`ldrls` at 0x818c8): taken to (unknown target) when `r1-1 <= 0x24 (unsigned, 32-bit)`, falls through to 0x818cc when `r1-1 > 0x24 (unsigned, 32-bit)`.
  Omitted-path effects remain unknown at the unresolved transfer (`ldrls` at 0x818c8).
  Known-edge recorded r0 = `r0` (`ldm` at 0x81afc).

## `vramSlotGet`

- `vramSlotGet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x81b18 to 0x81d50.
  Established facts:
  The instructions from 0x81b44 to 0x81d2c are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x81d40 to 0x81d50 are not reached by any path from the entry, and their branches are not listed.
  Branch (`ldrls` at 0x81b3c): taken to (unknown target) when `r1-1 <= 0x24 (unsigned, 32-bit)`, falls through to 0x81b40 when `r1-1 > 0x24 (unsigned, 32-bit)`.
  Omitted-path effects remain unknown at the unresolved transfer (`ldrls` at 0x81b3c).
  Known-edge recorded r0 = `0x0` (`ldm` at 0x81d3c).

## `IP57_Cfg_Init`

- `IP57_Cfg_Init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8201c to 0x82228.
  Established facts:
  The instructions from 0x82200 to 0x82228 are not reached by any path from the entry, and their branches are not listed.
  Byte store: target `abs 0x1a62e0`, value `0x1` (`strb` at 0x82070).
  Word store: target `abs 0x1a62e4`, value `[sp-28]` (`str` at 0x8207c).
  Word store: target `abs 0x1a62e8`, value `[sp-24]` (`str` at 0x82088).
  Word store: target `abs 0x1a62ec`, value `0xc` (`str` at 0x82094).
  Word store: target `abs 0x31a728`, value `ret(0x201fc)` (`str` at 0x820b0, `bl` at 0x820a8).
  Byte store: target `(unknown effective address)`, value `unknown` (`strb` at 0x82144).
  Byte store: target `[sp-48] +0x1`, value `([[sp-44]+3]:1+5 & 0xff)` (`strb` at 0x82158).
  Word store: target `[sp-48] +0x4`, value `[[sp-44]+12]` (`str` at 0x82168).
  Word store: target `[sp-48] +0x8`, value `0x200` (`str` at 0x82174).
  Word store: target `[sp-48] +0xc`, value `unknown` (`str` at 0x82188).
  Call (`bl` at 0x82064): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x166a08`, r2=`0x91`, r3=`0x0`.
  Call (`bl` at 0x820a8): (unknown callee), target 0x201fc, r0=`unknown`, r1=`unknown`, r2=`0x1a62ec`, r3=`unknown`.
  Call (`bl` at 0x820d8): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x166a08`, r2=`0x9d`, r3=`0x0`.
  Call (`bl` at 0x821a0): (unknown callee), target 0x82660, r0=`0x1a62f8`, r1=`unknown`, r2=`unknown`, r3=`[0x1a62e4]`.
  Call (`bl` at 0x821e8): (unknown callee), target 0x647a8, r0=`[0x31a728]`, r1=`[0x1a62e4]`, r2=`[0x1a62ec]`, r3=`0x0`.
  Call (`bl` at 0x821f4): (unknown callee), target 0x82228, r0=`[sp]`, r1=`[sp+4]`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0x82048): taken to 0x82068 when `[0x1a62e0]:1 == 0x0`, falls through to 0x8204c when `[0x1a62e0]:1 != 0x0`.
  Branch (`bne` at 0x820bc): taken to 0x820dc when `[0x31a728] != 0x0`, falls through to 0x820c0 when `[0x31a728] == 0x0`.
  Branch (`bge` at 0x820f4): taken to 0x8219c when `ge flags condition (operands unknown)`, falls through to 0x820f8 when `not (ge flags condition (operands unknown))`.
  Recorded r0 = `ret(0x82228)` (`ldm` at 0x821fc, `bl` at 0x821f4).

## `IP57_Set_MemCfg`

- `IP57_Set_MemCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82228 to 0x822ec.
  Established facts:
  The instructions from 0x822dc to 0x822ec are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x8229c): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x166a08`, r2=`0xd4`, r3=`0x0`.
  Call (`bl` at 0x822d0): (unknown callee), target 0x64c58, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0x82248): taken to 0x822a0 when `r0 == 0x0`, falls through to 0x8224c when `r0 != 0x0`.
  Branch (`bne` at 0x82254): taken to 0x822a0 when `r1 != 0x0`, falls through to 0x82258 when `r1 == 0x0`.
  Branch (`beq` at 0x82260): taken to 0x82268 when `r0 == 0x64`, falls through to 0x82264 when `r0 != 0x64`.
  Branch (`beq` at 0x822a8): taken to 0x822b8 when `Z == 1`, falls through to 0x822ac when `not (Z == 1)`.
  Branch (`bne` at 0x822b4): taken to 0x822c8 when `Z == 0`, falls through to 0x822b8 when `not (Z == 0)`.
  Recorded r0 = `ret(0x64c58)` (`ldm` at 0x822d8, `bl` at 0x822d0).

## `IP67_Cfg_Init`

- `IP67_Cfg_Init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x822ec to 0x824a4.
  Established facts:
  The instructions from 0x8247c to 0x824a4 are not reached by any path from the entry, and their branches are not listed.
  Byte store: target `abs 0x1a62e0`, value `0x1` (`strb` at 0x82340).
  Word store: target `abs 0x1a62e4`, value `0x1` (`str` at 0x8234c).
  Word store: target `abs 0x1a62e8`, value `[sp-24]` (`str` at 0x82358).
  Word store: target `abs 0x1a62ec`, value `0x8` (`str` at 0x82364).
  Word store: target `abs 0x31a728`, value `ret(0x201fc)` (`str` at 0x82378, `bl` at 0x82370).
  Byte store: target `[0x31a728] +0x0`, value `([[0x1a62e8]+3]:1+1 & 0xff)` (`strb` at 0x823bc).
  Byte store: target `[0x31a728] +0x1`, value `([[0x1a62e8]+3]:1+5 & 0xff)` (`strb` at 0x823d8).
  Word store: target `[0x31a728] +0x4`, value `[[0x1a62e8]+12]` (`str` at 0x823f0).
  Word store: target `[0x31a728] +0x8`, value `0xff000000` (`str` at 0x82400).
  Word store: target `[0x31a728] +0xc`, value `[0x1669dc]` (`str` at 0x82414).
  Call (`bl` at 0x82334): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x166a08`, r2=`0xf3`, r3=`0x0`.
  Call (`bl` at 0x82370): (unknown callee), target 0x201fc, r0=`0x10`, r1=`unknown`, r2=`0x1a62ec`, r3=`0x8`.
  Call (`bl` at 0x823a0): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x166a08`, r2=`0xfe`, r3=`0x0`.
  Call (`bl` at 0x8241c): (unknown callee), target 0x82660, r0=`0x1a62f8`, r1=`unknown`, r2=`[0x31a728]`, r3=`[0x1669dc]`.
  Call (`bl` at 0x82464): (unknown callee), target 0x647a8, r0=`[0x31a728]`, r1=`[0x1a62e4]`, r2=`[0x1a62ec]`, r3=`0x0`.
  Call (`bl` at 0x82470): (unknown callee), target 0x64c58, r0=`[sp-36]`, r1=`[sp]`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0x82318): taken to 0x82338 when `[0x1a62e0]:1 == 0x0`, falls through to 0x8231c when `[0x1a62e0]:1 != 0x0`.
  Branch (`bne` at 0x82384): taken to 0x823a4 when `[0x31a728] != 0x0`, falls through to 0x82388 when `[0x31a728] == 0x0`.
  Recorded r0 = `ret(0x64c58)` (`ldm` at 0x82478, `bl` at 0x82470).

## `IP69_Cfg_Init`

- `IP69_Cfg_Init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x824a4 to 0x82660.
  Established facts:
  The instructions from 0x82634 to 0x82660 are not reached by any path from the entry, and their branches are not listed.
  Byte store: target `abs 0x1a62e0`, value `0x1` (`strb` at 0x824f8).
  Word store: target `abs 0x1a62e4`, value `0x1` (`str` at 0x82504).
  Word store: target `abs 0x1a62e8`, value `[sp-24]` (`str` at 0x82510).
  Word store: target `abs 0x1a62ec`, value `0x4` (`str` at 0x8251c).
  Word store: target `abs 0x31a728`, value `ret(0x201fc)` (`str` at 0x82530, `bl` at 0x82528).
  Byte store: target `[0x31a728] +0x0`, value `([[0x1a62e8]+3]:1+1 & 0xff)` (`strb` at 0x82574).
  Byte store: target `[0x31a728] +0x1`, value `([[0x1a62e8]+3]:1+5 & 0xff)` (`strb` at 0x82590).
  Word store: target `[0x31a728] +0x4`, value `[[0x1a62e8]+12]` (`str` at 0x825a8).
  Word store: target `[0x31a728] +0x8`, value `0xff000000` (`str` at 0x825b8).
  Word store: target `[0x31a728] +0xc`, value `[0x1669dc]` (`str` at 0x825cc).
  Call (`bl` at 0x824ec): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x166a08`, r2=`0x12d`, r3=`0x0`.
  Call (`bl` at 0x82528): (unknown callee), target 0x201fc, r0=`0x10`, r1=`unknown`, r2=`0x1a62ec`, r3=`0x4`.
  Call (`bl` at 0x82558): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x166a08`, r2=`0x138`, r3=`0x0`.
  Call (`bl` at 0x825d4): (unknown callee), target 0x82660, r0=`0x1a62f8`, r1=`unknown`, r2=`[0x31a728]`, r3=`[0x1669dc]`.
  Call (`bl` at 0x8261c): (unknown callee), target 0x647a8, r0=`[0x31a728]`, r1=`[0x1a62e4]`, r2=`[0x1a62ec]`, r3=`0x0`.
  Call (`bl` at 0x82628): (unknown callee), target 0x64c58, r0=`[sp-36]`, r1=`[sp]`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0x824d0): taken to 0x824f0 when `[0x1a62e0]:1 == 0x0`, falls through to 0x824d4 when `[0x1a62e0]:1 != 0x0`.
  Branch (`bne` at 0x8253c): taken to 0x8255c when `[0x31a728] != 0x0`, falls through to 0x82540 when `[0x31a728] == 0x0`.
  Recorded r0 = `ret(0x64c58)` (`ldm` at 0x82630, `bl` at 0x82628).

## `IP57_Probe`

- `IP57_Probe`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82720 to 0x829bc.
  Established facts:
  The instructions from 0x829a4 to 0x829bc are not reached by any path from the entry, and their branches are not listed.
  Grouped accesses: 2 reported byte store records with printed base expression `[sp-17]:1` and resolved offset `+0x1a62f4`, including target `[sp-17]:1 +0x1a62f4`, value `0x0` (`strb` at 0x82780), target `[sp-17]:1 +0x1a62f4`, value `unknown` (`strb` at 0x828e8).
  Grouped accesses: 2 reported byte store records with printed base expression `[sp-17]:1` and resolved offset `+0x31a724`, including target `[sp-17]:1 +0x31a724`, value `0x2` (`strb` at 0x82790), target `[sp-17]:1 +0x31a724`, value `0x1` (`strb` at 0x82950).
  Grouped accesses: 2 reported byte load records with printed base expression `[sp-24]` and resolved offset `+0x3`, including target `[sp-24] +0x3` (`ldrb` at 0x82798), target `[sp-24] +0x3` (`ldrb` at 0x827ac).
  Word load: target `abs 0x1a62ec` (`ldr` at 0x827b4).
  Grouped accesses: 2 reported byte load records with printed base expression `[sp-24]` and resolved offset `+0x2`, including target `[sp-24] +0x2` (`ldrb` at 0x82800), target `[sp-24] +0x2` (`ldrb` at 0x82820).
  Byte load: target `[sp-17]:1 +0x1a62f4` (`ldrb` at 0x828e0).
  Call (`bl` at 0x82750): (unknown callee), target 0x84274, r0=`(r0 & 0xff)`, r1=`sp-24`, r2=`sp-24`, r3=`(r0 & 0xff)`.
  Call (`bl` at 0x827d4): (unknown callee), target 0x11fca8, r0=`0x0`, r1=`unknown`, r2=`[[sp-24]+3]:1`, r3=`unknown`.
  Call (`bl` at 0x82818): (unknown callee), target 0x243d4, r0=`0x166ab4`, r1=`unknown`, r2=`[[sp-24]+2]:1`, r3=`unknown`.
  Call (`bl` at 0x82840): (unknown callee), target 0x125b8c, r0=`0x0`, r1=`unknown`, r2=`sp-25`, r3=`unknown`.
  Call (`bl` at 0x8287c): (unknown callee), target 0x21ce0, r0=`ret(0x125b8c)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x125b8c)`.
  Call (`bl` at 0x828a8): (unknown callee), target 0x22070, r0=`0x1`, r1=`0x166a08`, r2=`0x1a4`, r3=`0x0`.
  Call (`bl` at 0x828bc): (unknown callee), target 0x8337c, r0=`[sp-17]:1`, r1=`[sp-33]:1`, r2=`[sp-33]:1`, r3=`[sp-17]:1`.
  Call (`bl` at 0x8291c): (unknown callee), target 0x7fe94, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x8293c): (unknown callee), target 0x83280, r0=`[sp-17]:1`, r1=`[sp-33]:1`, r2=`[sp-33]:1`, r3=`[sp-17]:1`.
  Call (`bl` at 0x8296c): (unknown callee), target 0x80ee8, r0=`[sp-17]:1`, r1=`unknown`, r2=`unknown`, r3=`[sp-17]:1`.
  Call (`bl` at 0x8298c): (unknown callee), target 0x82cb0, r0=`[sp-17]:1`, r1=`0x1`, r2=`0x0`, r3=`[sp-17]:1`.
  Branch (`beq` at 0x82764): taken to 0x82774 when `ret(0x84274) == 0x0`, falls through to 0x82768 when `ret(0x84274) != 0x0` (source `bl` at 0x82750).
  Branch (`bge` at 0x827c4): taken to 0x827e8 when `ge flags condition (operands unknown)`, falls through to 0x827c8 when `not (ge flags condition (operands unknown))`.
  Branch (`bhi` at 0x827f8): taken to 0x82964 when `hi flags condition (operands unknown)`, falls through to 0x827fc when `not (hi flags condition (operands unknown))`.
  Branch (`bne` at 0x82854): taken to 0x8286c when `ret(0x125b8c) != 0x0`, falls through to 0x82858 when `ret(0x125b8c) == 0x0` (source `bl` at 0x82840).
  Branch (`bne` at 0x82864): taken to 0x8286c when `unknown != 0x0`, falls through to 0x82868 when `unknown == 0x0`.
  Branch (`beq` at 0x82874): taken to 0x828ac when `ret(0x125b8c) == 0x0`, falls through to 0x82878 when `ret(0x125b8c) != 0x0` (source `bl` at 0x82840).
  Branch (`beq` at 0x828f8): taken to 0x82944 when `unknown == 0x0`, falls through to 0x828fc when `unknown != 0x0`.
  Branch (`bne` at 0x82928): taken to 0x82944 when `ret(0x7fe94) != 0x0`, falls through to 0x8292c when `ret(0x7fe94) == 0x0` (source `bl` at 0x8291c).
  Branch (`bne` at 0x82978): taken to 0x82990 when `ret(0x80ee8) != 0x0`, falls through to 0x8297c when `ret(0x80ee8) == 0x0` (source `bl` at 0x8296c).
  Recorded r0 = `unknown` (`ldm` at 0x829a0).

## `IP57_Init`

- `IP57_Init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x829bc to 0x82a38.
  Established facts:
  Call (`bl` at 0x829dc): (unknown callee), target 0x80e60, r0=`(r0 & 0xff)`, r1=`r1`, r2=`r2`, r3=`(r0 & 0xff)`.
  Call (`bl` at 0x829f4): (unknown callee), target 0x80ee8, r0=`[sp-17]:1`, r1=`unknown`, r2=`unknown`, r3=`[sp-17]:1`.
  Call (`bl` at 0x82a14): (unknown callee), target 0x82cb0, r0=`[sp-17]:1`, r1=`0x0`, r2=`0x1`, r3=`[sp-17]:1`.
  Branch (`beq` at 0x829e8): taken to 0x82a24 when `ret(0x80e60) == 0x0`, falls through to 0x829ec when `ret(0x80e60) != 0x0` (source `bl` at 0x829dc).
  Branch (`bne` at 0x82a00): taken to 0x82a24 when `ret(0x80ee8) != 0x0`, falls through to 0x82a04 when `ret(0x80ee8) == 0x0` (source `bl` at 0x829f4).
  Recorded r0 = `unknown` (`ldm` at 0x82a34).

## `IP57_Nmi`

- `IP57_Nmi`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x83ec4 to 0x83fa4.
  Established facts:
  Byte load: target `[sp-24] +0x2` (`ldrb` at 0x83f14).
  Call (`bl` at 0x83eec): (unknown callee), target 0x84274, r0=`(r0 & 0xff)`, r1=`sp-24`, r2=`sp-24`, r3=`(r0 & 0xff)`.
  Call (`bl` at 0x83f30): (unknown callee), target 0x125a0c, r0=`0x0`, r1=`([[sp-24]+2]:1+2 & 0xff)`, r2=`0x10`, r3=`([[sp-24]+2]:1+2 & 0xff)`.
  Call (`bl` at 0x83f58): (unknown callee), target 0x839b0, r0=`0xa`, r1=`unknown`, r2=`unknown`, r3=`ret(0x125a0c)`.
  Call (`bl` at 0x83f6c): (unknown callee), target 0x1259bc, r0=`0x0`, r1=`[sp-25]:1`, r2=`0x10`, r3=`[sp-25]:1`.
  Branch (`beq` at 0x83f00): taken to 0x83f10 when `ret(0x84274) == 0x0`, falls through to 0x83f04 when `ret(0x84274) != 0x0` (source `bl` at 0x83eec).
  Branch (`beq` at 0x83f44): taken to 0x83f54 when `ret(0x125a0c) == 0x0`, falls through to 0x83f48 when `ret(0x125a0c) != 0x0` (source `bl` at 0x83f30).
  Branch (`beq` at 0x83f80): taken to 0x83f90 when `ret(0x1259bc) == 0x0`, falls through to 0x83f84 when `ret(0x1259bc) != 0x0` (source `bl` at 0x83f6c).
  Recorded r0 = `unknown` (`ldm` at 0x83fa0).

## `IP57_Int`

- `IP57_Int`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x83fa4 to 0x841e0.
  Established facts:
  The instructions from 0x841b4 to 0x841e0 are not reached by any path from the entry, and their branches are not listed.
  Load with unknown width: target `abs 0x841b4` (`ldc` at 0x84180).
  Load with unknown width: target `abs 0x841bc` (`ldc` at 0x84188).
  Call (`bl` at 0x84020): (unknown callee), target 0x7f3cc, r0=`0x0`, r1=`[sp-24]:1`, r2=`[sp-25]:1`, r3=`[sp-26]:1`.
  Call (`bl` at 0x84034): (unknown callee), target 0x839b0, r0=`0xa`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x84048): (unknown callee), target 0x125b8c, r0=`0x0`, r1=`[sp-24]:1`, r2=`sp-26`, r3=`[sp-24]:1`.
  Call (`bl` at 0x8406c): (unknown callee), target 0x1265c4, r0=`0x0`, r1=`[sp-24]:1`, r2=`unknown`, r3=`[sp-24]:1`.
  Call (`bl` at 0x84078): (unknown callee), target 0x21ce0, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x840a0): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x166a08`, r2=`0x4d3`, r3=`0x0`.
  Call (`bl` at 0x840d8): (unknown callee), target 0x29908, r0=`0x166b74`, r1=`[sp-21]:1+48`, r2=`[sp-22]:1+65`, r3=`[sp-22]:1+65`.
  Call (`bl` at 0x840f0): (unknown callee), target 0x72c6c, r0=`0x0`, r1=`[sp-23]:1`, r2=`sp-36`, r3=`[sp-23]:1`.
  Call (`bl` at 0x84114): (unknown callee), target 0x72dd8, r0=`0x0`, r1=`[sp-23]:1`, r2=`unknown`, r3=`[sp-23]:1`.
  Call (`bl` at 0x84120): (unknown callee), target 0x21ce0, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x84148): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x166a08`, r2=`0x4dd`, r3=`0x0`.
  Call (`bl` at 0x841a8): (unknown callee), target 0x22070, r0=`0x1`, r1=`0x166a08`, r2=`0x4e0`, r3=`0x0`.
  Branch (`bge` at 0x8402c): taken to 0x841ac when `unknown >= 0x0 (signed, 32-bit)`, falls through to 0x84030 when `unknown < 0x0 (signed, 32-bit)`.
  Branch (`beq` at 0x8405c): taken to 0x840a8 when `ret(0x125b8c) == 0x0`, falls through to 0x84060 when `ret(0x125b8c) != 0x0` (source `bl` at 0x84048).
  Branch (`bne` at 0x840b8): taken to 0x840e0 when `unknown != 0x0`, falls through to 0x840bc when `unknown == 0x0`.
  Branch (`beq` at 0x84104): taken to 0x84150 when `ret(0x72c6c) == 0x0`, falls through to 0x84108 when `ret(0x72c6c) != 0x0` (source `bl` at 0x840f0).
  Recorded r0 = `unknown` (`ldm` at 0x841b0).

## `IP57_Get_Cfg`

- `IP57_Get_Cfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x841e0 to 0x84244.
  Established facts:
  The instructions from 0x84238 to 0x8423c are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x84240 to 0x84244 are not reached by any path from the entry, and their branches are not listed.
  Grouped accesses: 2 reported word load records with absolute target `0x1a62e8`, including target `abs 0x1a62e8` (`ldr` at 0x841f8), target `abs 0x1a62e8` (`ldr` at 0x84218).
  Word store: target `r0 +0x0`, value `[0x1a62e8]` (`str` at 0x8421c).
  Word load: target `abs 0x1a62e4` (`ldr` at 0x84224).
  Branch (`bne` at 0x84200): taken to 0x84210 when `[0x1a62e8] != 0x0`, falls through to 0x84204 when `[0x1a62e8] == 0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x84234).

## `pkr`

- `pkr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x84318 to 0x8439c.
  Established facts:
  The instructions from 0x84394 to 0x8439c are not reached by any path from the entry, and their branches are not listed.
  Word load: target `abs 0x31a730` (`ldr` at 0x8433c).
  Word load: target `[sp-28] +0x0` (`ldr` at 0x84378).
  Word load: target `[sp-32] +0x0` (`ldr` at 0x8437c).
  Call (`mov` at 0x84354): (unknown callee), target (unknown target), r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0x84380): (unknown callee), target 0x29908, r0=`0x166dc4`, r1=`[sp-24]`, r2=`[[sp-28]]`, r3=`[[sp-32]]`.
  Recorded r0 = `[sp-36]` (`ldm` at 0x84390).

## `pkw`

- `pkw`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8439c to 0x84418.
  Established facts:
  The instructions from 0x84410 to 0x84418 are not reached by any path from the entry, and their branches are not listed.
  Word load: target `abs 0x31a72c` (`ldr` at 0x843c0).
  Call (`mov` at 0x843d8): (unknown callee), target (unknown target), r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0x843fc): (unknown callee), target 0x29908, r0=`0x166dd4`, r1=`[sp-24]`, r2=`[sp-28]`, r3=`[sp-32]`.
  Recorded r0 = `[sp-36]` (`ldm` at 0x8440c).

## `PktGenRead`

- `PktGenRead`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x86eac to 0x86f7c.
  Established facts:
  Word store: target `(unknown effective address)`, value `[sp]` (`str` at 0x86edc).
  Word store: target `(unknown effective address)`, value `[sp+4]` (`str` at 0x86ef0).
  Word load: target `(unknown effective address)` (`ldr` at 0x86f4c).
  Word load: target `(unknown effective address)` (`ldr` at 0x86f64).
  Word store: target `[sp+8] +0x0`, value `unknown` (`str` at 0x86f50).
  Word store: target `[sp+12] +0x0`, value `unknown` (`str` at 0x86f68).
  Call (`bl` at 0x86f24): (unknown callee), target 0x85548, r0=`0x0`, r1=`[sp-20]`, r2=`[sp-24]`, r3=`[sp-28]`.
  Branch (`bne` at 0x86f38): taken to 0x86f6c when `ret(0x85548) != 0x0`, falls through to 0x86f3c when `ret(0x85548) == 0x0` (source `bl` at 0x86f24).
  Recorded r0 = `unknown` (`ldm` at 0x86f78).
