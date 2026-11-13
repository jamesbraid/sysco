# ARM Linux 2.6 PSIC, image and network routines in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-psic-image-network-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register
state from the analysis rather than a C return type.

Groups count static records with the same width, access kind and resolved target expression. Address order need not describe one execution path.

## `psicInt`

- `psicInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12bb50 to 0x12bf04.
  Established facts:
  The instructions from 0x12bee4 to 0x12beec are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x12bef0 to 0x12bf04 are not reached by any path from the entry, and their branches are not listed.
  Word load: target `abs 0x1a0844` (`ldr` at 0x12bb80).
  Branch (`bge` at 0x12bb88): taken to 0x12bbb8 when `r0 >= [0x1a0844] (signed, 32-bit)`, falls through to 0x12bb8c when `r0 < [0x1a0844] (signed, 32-bit)`.
  2 such word load accesses with resolved target `abs 0x1a0840`, including target `abs 0x1a0840` (`ldr` at 0x12bba4), target `abs 0x1a0840` (`ldr` at 0x12bbdc).
  Word load: target `(unknown effective address)` (`ldr` at 0x12bbac).
  Branch (`bne` at 0x12bbb4): taken to 0x12bbc4 when `unknown != 0x0`, falls through to 0x12bbb8 when `unknown == 0x0`.
  Call (`bl` at 0x12bbe8): (unknown callee), target 0x6e140, r0=`r0`, r1=`r1`, r2=`unknown`, r3=`unknown`.
  Branch (`ble` at 0x12bbf4): taken to 0x12bc5c when `ret(0x6e140) <= 0x1 (signed, 32-bit)`, falls through to 0x12bbf8 when `ret(0x6e140) > 0x1 (signed, 32-bit)` (source `bl` at 0x12bbe8).
  Call (`bl` at 0x12bbf8): (unknown callee), target 0x6e190, r0=`ret(0x6e140)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x6e140)`.
  Branch (`beq` at 0x12bc08): taken to 0x12bc5c when `ret(0x6e190) == [sp-24]`, falls through to 0x12bc0c when `ret(0x6e190) != [sp-24]` (source `bl` at 0x12bbf8).
  2 such byte load accesses with resolved target `[sp-28] +0x2`, including target `[sp-28] +0x2` (`ldrb` at 0x12bc14), target `[sp-28] +0x2` (`ldrb` at 0x12bc64).
  2 such word load accesses with resolved target `[sp-56] +0x0`, including target `[sp-56] +0x0` (`ldr` at 0x12bc1c), target `[sp-56] +0x0` (`ldr` at 0x12bc6c).
  Word load: target `(unknown effective address)` (`ldr` at 0x12bc24).
  Word load: target `(unknown effective address)` (`ldr` at 0x12bc74).
  Branch (`bgt` at 0x12bc94): taken to 0x12be30 when `gt flags condition (operands unknown)`, falls through to 0x12bc98 when `not (gt flags condition (operands unknown))`.
  Call (`bl` at 0x12bc98): (unknown callee), target 0x134634, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Byte load: target `[sp-28] +0x3` (`ldrb` at 0x12bca0).
  Call (`bl` at 0x12bcb8): (unknown callee), target 0x12c354, r0=`[sp-60]`, r1=`[[sp-28]+3]:1`, r2=`0xd`, r3=`sp-49`.
  Branch (`beq` at 0x12bccc): taken to 0x12bd88 when `ret(0x12c354) == 0x0`, falls through to 0x12bcd0 when `ret(0x12c354) != 0x0` (source `bl` at 0x12bcb8).
  Call (`bl` at 0x12bce0): (unknown callee), target 0x6e1b0, r0=`[sp-24]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x12bcec): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x12bd34): (unknown callee), target 0x203f4, r0=`[sp-72]`, r1=`[sp-64]`, r2=`[sp-68]`, r3=`unknown`.
  Call (`bl` at 0x12bd40): (unknown callee), target 0x21ce0, r0=`[sp-40]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  2 such word load accesses with resolved target `[sp-28] +0xc`, including target `[sp-28] +0xc` (`ldr` at 0x12bd58), target `[sp-28] +0xc` (`ldr` at 0x12beac).
  Call (`bl` at 0x12bd74): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x19045c`, r2=`0x112`, r3=`0x0`.
  Call (`bl` at 0x12bd78): (unknown callee), target 0x1346fc, r0=`ret(0x22070)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x12bd88): (unknown callee), target 0x1346fc, r0=`ret(0x12c354)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x12c354)`.
  Byte load: target `[sp-32] +0x3` (`ldrb` at 0x12bd90).
  Branch (`beq` at 0x12bda4): taken to 0x12be14 when `unknown == 0x0`, falls through to 0x12bda8 when `unknown != 0x0`.
  Call (`bl` at 0x12bda8): (unknown callee), target 0x6e140, r0=`ret(0x1346fc)`, r1=`unknown`, r2=`[[sp-32]+3]:1`, r3=`unknown`.
  Branch (`beq` at 0x12bdb4): taken to 0x12bdcc when `ret(0x6e140) == 0x1`, falls through to 0x12bdb8 when `ret(0x6e140) != 0x1` (source `bl` at 0x12bda8).
  Call (`bl` at 0x12bdb8): (unknown callee), target 0x3e38c, r0=`ret(0x6e140)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x6e140)`.
  Branch (`bne` at 0x12bdc4): taken to 0x12bdcc when `ret(0x3e38c) != 0x0`, falls through to 0x12bdc8 when `ret(0x3e38c) == 0x0` (source `bl` at 0x12bdb8).
  Call (`bl` at 0x12bdcc): (unknown callee), target 0x6e190, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x12bdfc): (unknown callee), target 0x1261c4, r0=`unknown`, r1=`[sp-28]`, r2=`[sp-32]`, r3=`[sp-33]:1`.
  Call (`bl` at 0x12be40): (unknown callee), target 0x6e1b0, r0=`[sp-24]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x12be4c): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x12be94): (unknown callee), target 0x203f4, r0=`[sp-64]`, r1=`[sp-72]`, r2=`[sp-68]`, r3=`unknown`.
  Call (`bl` at 0x12becc): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x19045c`, r2=`0x11f`, r3=`0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x12bee0).

## `psicGetRevision`

- `psicGetRevision`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12bf04 to 0x12c0e0.
  Established facts:
  The instructions from 0x12c0cc to 0x12c0e0 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x12bf28): (unknown callee), target 0x13dfe0, r0=`r0`, r1=`r1`, r2=`r2`, r3=`0x0`.
  Branch (`beq` at 0x12bf3c): taken to 0x12bf58 when `ret(0x13dfe0) == 0x0`, falls through to 0x12bf40 when `ret(0x13dfe0) != 0x0` (source `bl` at 0x12bf28).
  Word load: target `ret(0x13dfe0) +0x0` (`ldr` at 0x12bf44, `bl` at 0x12bf28).
  Halfword load: target `[ret(0x13dfe0)] +0xe` (`ldrh` at 0x12bf48, `bl` at 0x12bf28).
  Branch (`bne` at 0x12bf54): taken to 0x12bf64 when `unknown != 0x0`, falls through to 0x12bf58 when `unknown == 0x0`.
  Call (`bl` at 0x12bf64): (unknown callee), target 0x134634, r0=`ret(0x13dfe0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x12bf8c): (unknown callee), target 0x13d9cc, r0=`[sp-20]`, r1=`0x7`, r2=`sp-32`, r3=`sp-32`.
  Branch (`beq` at 0x12bfa0): taken to 0x12bfb4 when `ret(0x13d9cc) == 0x0`, falls through to 0x12bfa4 when `ret(0x13d9cc) != 0x0` (source `bl` at 0x12bf8c).
  Call (`bl` at 0x12bfa4): (unknown callee), target 0x1346fc, r0=`ret(0x13d9cc)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13d9cc)`.
  Call (`bl` at 0x12bfb4): (unknown callee), target 0x1346fc, r0=`ret(0x13d9cc)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13d9cc)`.
  Halfword store: target `[sp-24] +0x0`, value `([sp-32] & 0xffff)` (`strh` at 0x12c088).
  Halfword load: target `[sp-24] +0x0` (`ldrh` at 0x12c0ac).
  Halfword store: target `(unknown effective address)`, value `([[sp-24]]:2 & 0xffff)` (`strh` at 0x12c0b0).
  Recorded r0 = `unknown` (`ldm` at 0x12c0c4).

## `psicRevision`

- `psicRevision`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12c0e0 to 0x12c16c.
  Established facts:
  The instructions from 0x12c168 to 0x12c16c are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x12c0f8): (unknown callee), target 0x13dfe0, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Branch (`beq` at 0x12c10c): taken to 0x12c128 when `ret(0x13dfe0) == 0x0`, falls through to 0x12c110 when `ret(0x13dfe0) != 0x0` (source `bl` at 0x12c0f8).
  Word load: target `ret(0x13dfe0) +0x0` (`ldr` at 0x12c114, `bl` at 0x12c0f8).
  Word load: target `[ret(0x13dfe0)] +0x0` (`ldr` at 0x12c118, `bl` at 0x12c0f8).
  Branch (`bne` at 0x12c120): taken to 0x12c128 when `[[ret(0x13dfe0)]] != 0x9`, falls through to 0x12c124 when `[[ret(0x13dfe0)]] == 0x9` (source `bl` at 0x12c0f8).
  Halfword load: target `(unknown effective address)` (`ldrh` at 0x12c150).
  Recorded r0 = `unknown` (`ldm` at 0x12c160).

## `imageSyncInit`

- `imageSyncInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12a23c to 0x12a374.
  Established facts:
  The instructions from 0x12a360 to 0x12a374 are not reached by any path from the entry, and their branches are not listed.
  4 such word load accesses with resolved target `abs 0x314ff0`, including target `abs 0x314ff0` (`ldr` at 0x12a25c), target `abs 0x314ff0` (`ldr` at 0x12a288), target `abs 0x314ff0` (`ldr` at 0x12a2f8).
  Branch (`beq` at 0x12a264): taken to 0x12a2b0 when `[0x314ff0] == 0x0`, falls through to 0x12a268 when `[0x314ff0] != 0x0`.
  Call (`bl` at 0x12a268): (unknown callee), target 0x6e190, r0=`r0`, r1=`r1`, r2=`r2`, r3=`[0x314ff0]`.
  Branch (`bne` at 0x12a278): taken to 0x12a2b0 when `ret(0x6e190) != [sp-24]`, falls through to 0x12a27c when `ret(0x6e190) == [sp-24]` (source `bl` at 0x12a268).
  3 such word load accesses with resolved target `abs 0x314ff4`, including target `abs 0x314ff4` (`ldr` at 0x12a294), target `abs 0x314ff4` (`ldr` at 0x12a2b4), target `abs 0x314ff4` (`ldr` at 0x12a2d4).
  Call (`bl` at 0x12a2ac): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x19031c`, r2=`0x48`, r3=`0x0`.
  Branch (`bne` at 0x12a2bc): taken to 0x12a328 when `[0x314ff4] != 0x0`, falls through to 0x12a2c0 when `[0x314ff4] == 0x0`.
  Call (`bl` at 0x12a2c0): (unknown callee), target 0x6e140, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[0x314ff4]`.
  Word store: target `abs 0x314ff4`, value `ret(0x6e140)` (`str` at 0x12a2cc, `bl` at 0x12a2c0).
  Call (`bl` at 0x12a2ec): (unknown callee), target 0x204d8, r0=`[sp-44]`, r1=`0x1`, r2=`ret(0x6e140)`, r3=`unknown`.
  Word store: target `abs 0x314ff0`, value `ret(0x204d8)` (`str` at 0x12a2f4, `bl` at 0x12a2ec).
  Branch (`bne` at 0x12a300): taken to 0x12a328 when `[0x314ff0] != 0x0`, falls through to 0x12a304 when `[0x314ff0] == 0x0`.
  Call (`bl` at 0x12a324): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x19031c`, r2=`0x53`, r3=`0x0`.
  Word store: target `[sp-36] +0x0`, value `unknown` (`str` at 0x12a348).
  Word store: target `[sp-36] +0x4`, value `[sp-32]` (`str` at 0x12a354).
  Recorded r0 = `unknown` (`ldm` at 0x12a35c).

## `imageSyncRemove`

- `imageSyncRemove`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12a374 to 0x12a40c.
  Established facts:
  The instructions from 0x12a404 to 0x12a40c are not reached by any path from the entry, and their branches are not listed.
  Word load: target `abs 0x314ff4` (`ldr` at 0x12a38c).
  Branch (`beq` at 0x12a394): taken to 0x12a3fc when `[0x314ff4] == 0x0`, falls through to 0x12a398 when `[0x314ff4] != 0x0`.
  2 such word load accesses with resolved target `abs 0x314ff0`, including target `abs 0x314ff0` (`ldr` at 0x12a3a4), target `abs 0x314ff0` (`ldr` at 0x12a3c8).
  Word load: target `(unknown effective address)` (`ldr` at 0x12a3ac).
  Branch (`bne` at 0x12a3b4): taken to 0x12a3bc when `unknown != 0x0`, falls through to 0x12a3b8 when `unknown == 0x0`.
  Word load: target `[sp-24] +0x4` (`ldr` at 0x12a3d8).
  Branch (`beq` at 0x12a3e0): taken to 0x12a3fc when `[[sp-24]+4] == 0x0`, falls through to 0x12a3e4 when `[[sp-24]+4] != 0x0`.
  Word store: target `[sp-24] +0x4`, value `0x0` (`str` at 0x12a3ec).
  Word store: target `[sp-24] +0x0`, value `0x0` (`str` at 0x12a3f8).
  Recorded r0 = `r0` (`ldm` at 0x12a400).

## `imageSyncGetCfg`

- `imageSyncGetCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12a40c to 0x12a4e8.
  Established facts:
  The instructions from 0x12a4dc to 0x12a4e4 are not reached by any path from the entry, and their branches are not listed.
  2 such word load accesses with resolved target `abs 0x314ff4`, including target `abs 0x314ff4` (`ldr` at 0x12a428), target `abs 0x314ff4` (`ldr` at 0x12a464).
  Branch (`beq` at 0x12a430): taken to 0x12a454 when `[0x314ff4] == 0x0`, falls through to 0x12a434 when `[0x314ff4] != 0x0`.
  3 such word load accesses with resolved target `abs 0x314ff0`, including target `abs 0x314ff0` (`ldr` at 0x12a440), target `abs 0x314ff0` (`ldr` at 0x12a47c), target `abs 0x314ff0` (`ldr` at 0x12a4a8).
  Word load: target `(unknown effective address)` (`ldr` at 0x12a448).
  Branch (`bne` at 0x12a450): taken to 0x12a460 when `unknown != 0x0`, falls through to 0x12a454 when `unknown == 0x0`.
  Branch (`beq` at 0x12a46c): taken to 0x12a490 when `[0x314ff4] == 0x0`, falls through to 0x12a470 when `[0x314ff4] != 0x0`.
  Word load: target `(unknown effective address)` (`ldr` at 0x12a484).
  Branch (`bne` at 0x12a48c): taken to 0x12a49c when `unknown != 0x0`, falls through to 0x12a490 when `unknown == 0x0`.
  Word load: target `[sp-28] +0x0` (`ldr` at 0x12a4bc).
  Word store: target `r1 +0x0`, value `[[sp-28]]` (`str` at 0x12a4c0).
  Word load: target `[sp-28] +0x4` (`ldr` at 0x12a4c8).
  Recorded r0 = `unknown` (`ldm` at 0x12a4d8).

## `imageSyncCount`

- `imageSyncCount`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12a4e8 to 0x12a5b0.
  Established facts:
  The instructions from 0x12a5a4 to 0x12a5ac are not reached by any path from the entry, and their branches are not listed.
  2 such word load accesses with resolved target `abs 0x314ff4`, including target `abs 0x314ff4` (`ldr` at 0x12a500), target `abs 0x314ff4` (`ldr` at 0x12a53c).
  Branch (`beq` at 0x12a508): taken to 0x12a52c when `[0x314ff4] == 0x0`, falls through to 0x12a50c when `[0x314ff4] != 0x0`.
  3 such word load accesses with resolved target `abs 0x314ff0`, including target `abs 0x314ff0` (`ldr` at 0x12a518), target `abs 0x314ff0` (`ldr` at 0x12a554), target `abs 0x314ff0` (`ldr` at 0x12a580).
  Word load: target `(unknown effective address)` (`ldr` at 0x12a520).
  Branch (`bne` at 0x12a528): taken to 0x12a538 when `unknown != 0x0`, falls through to 0x12a52c when `unknown == 0x0`.
  Branch (`beq` at 0x12a544): taken to 0x12a568 when `[0x314ff4] == 0x0`, falls through to 0x12a548 when `[0x314ff4] != 0x0`.
  Word load: target `(unknown effective address)` (`ldr` at 0x12a55c).
  Branch (`bne` at 0x12a564): taken to 0x12a574 when `unknown != 0x0`, falls through to 0x12a568 when `unknown == 0x0`.
  Word load: target `[sp-24] +0x4` (`ldr` at 0x12a590).
  Recorded r0 = `unknown` (`ldm` at 0x12a5a0).

## `imageSyncClear`

- `imageSyncClear`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12a6b8 to 0x12a7a4.
  Established facts:
  Call (`bl` at 0x12a6e0): (unknown callee), target 0x12a8e8, r0=`r0`, r1=`(r1 & 0xff)`, r2=`r2`, r3=`(r1 & 0xff)`.
  Branch (`bne` at 0x12a6f4): taken to 0x12a704 when `ret(0x12a8e8) != 0x0`, falls through to 0x12a6f8 when `ret(0x12a8e8) == 0x0` (source `bl` at 0x12a6e0).
  Byte load: target `ret(0x12a8e8) +0x0` (`ldrb` at 0x12a708, `bl` at 0x12a6e0).
  Call (`bl` at 0x12a714): (unknown callee), target 0x10a4ec, r0=`[sp-20]`, r1=`[ret(0x12a8e8)]:1`, r2=`unknown`, r3=`[ret(0x12a8e8)]:1`.
  2 such byte load accesses with resolved target `[sp-28] +0x1`, including target `[sp-28] +0x1` (`ldrb` at 0x12a724), target `[sp-28] +0x1` (`ldrb` at 0x12a778).
  Call (`bl` at 0x12a734): (unknown callee), target 0x10ad34, r0=`unknown`, r1=`[[sp-28]+1]:1`, r2=`0x1`, r3=`[[sp-28]+1]:1`.
  Branch (`beq` at 0x12a748): taken to 0x12a758 when `ret(0x10ad34) == 0x0`, falls through to 0x12a74c when `ret(0x10ad34) != 0x0` (source `bl` at 0x12a734).
  Byte load: target `[sp-28] +0x0` (`ldrb` at 0x12a75c).
  Call (`bl` at 0x12a768): (unknown callee), target 0x10a4ec, r0=`[sp-20]`, r1=`[[sp-28]]:1`, r2=`unknown`, r3=`[[sp-28]]:1`.
  Call (`bl` at 0x12a788): (unknown callee), target 0x10ad34, r0=`unknown`, r1=`[[sp-28]+1]:1`, r2=`0x0`, r3=`[[sp-28]+1]:1`.
  Recorded r0 = `unknown` (`ldm` at 0x12a79c).

## `imageSyncRead`

- `imageSyncRead`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12a7a4 to 0x12a840.
  Established facts:
  Call (`bl` at 0x12a7d0): (unknown callee), target 0x12a8e8, r0=`r0`, r1=`(r1 & 0xff)`, r2=`r2`, r3=`(r1 & 0xff)`.
  Branch (`bne` at 0x12a7e4): taken to 0x12a7f4 when `ret(0x12a8e8) != 0x0`, falls through to 0x12a7e8 when `ret(0x12a8e8) == 0x0` (source `bl` at 0x12a7d0).
  Byte load: target `ret(0x12a8e8) +0x0` (`ldrb` at 0x12a7f8, `bl` at 0x12a7d0).
  Call (`bl` at 0x12a804): (unknown callee), target 0x10a4ec, r0=`[sp-20]`, r1=`[ret(0x12a8e8)]:1`, r2=`unknown`, r3=`[ret(0x12a8e8)]:1`.
  Byte load: target `[sp-32] +0x1` (`ldrb` at 0x12a814).
  Call (`bl` at 0x12a824): (unknown callee), target 0x10ae68, r0=`unknown`, r1=`[[sp-32]+1]:1`, r2=`[sp-28]`, r3=`[[sp-32]+1]:1`.
  Recorded r0 = `unknown` (`ldm` at 0x12a838).

## `imageSyncName`

- `imageSyncName`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12a840 to 0x12a8e8.
  Established facts:
  The instructions from 0x12a8dc to 0x12a8e8 are not reached by any path from the entry, and their branches are not listed.
  Word load: target `abs 0x314ff0` (`ldr` at 0x12a868).
  Call (`bl` at 0x12a880): (unknown callee), target 0x12a8e8, r0=`r0`, r1=`(r1 & 0xff)`, r2=`unknown`, r3=`(r1 & 0xff)`.
  Branch (`bne` at 0x12a894): taken to 0x12a8a4 when `ret(0x12a8e8) != 0x0`, falls through to 0x12a898 when `ret(0x12a8e8) == 0x0` (source `bl` at 0x12a880).
  2 such word load accesses with resolved target `ret(0x12a8e8) +0x4`, including target `ret(0x12a8e8) +0x4` (`ldr` at 0x12a8a8, `bl` at 0x12a880), target `ret(0x12a8e8) +0x4` (`ldr` at 0x12a8b4, `bl` at 0x12a880).
  Branch (`bne` at 0x12a8bc): taken to 0x12a8c8 when `[ret(0x12a8e8)+4] != 0x0`, falls through to 0x12a8c0 when `[ret(0x12a8e8)+4] == 0x0` (source `bl` at 0x12a880).
  Recorded r0 = `unknown` (`ldm` at 0x12a8d8).

## `init_slot`

- `init_slot`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf1424 to 0xf1480.
  Established facts:
  Word store: target `r0 +0x0`, value `r1` (`str` at 0xf1444).
  Call (`bl` at 0xf146c): (unknown callee), target 0x201fc, r0=`unknown`, r1=`r1`, r2=`[sp-28]`, r3=`unknown`.
  Word store: target `[sp-24] +0x4`, value `ret(0x201fc)` (`str` at 0xf1474, `bl` at 0xf146c).
  Recorded r0 = `ret(0x201fc)` (`ldm` at 0xf147c, `bl` at 0xf146c).

## `init_asic`

- `init_asic`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf1480 to 0xf17c4.
  Established facts:
  The instructions from 0xf17ac to 0xf17c4 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0xf14b4): (unknown callee), target 0x20550, r0=`r0`, r1=`0x0`, r2=`0x34`, r3=`0x0`.
  Branch (`beq` at 0xf14c8): taken to 0xf15d4 when `[sp-28] == 0x2`, falls through to 0xf14cc when `[sp-28] != 0x2`.
  Branch (`bgt` at 0xf14d4): taken to 0xf14e8 when `[sp-28] > 0x2 (signed, 32-bit)`, falls through to 0xf14d8 when `[sp-28] <= 0x2 (signed, 32-bit)`.
  Branch (`beq` at 0xf14e0): taken to 0xf14f8 when `[sp-28] == 0x0`, falls through to 0xf14e4 when `[sp-28] != 0x0`.
  Branch (`beq` at 0xf14f0): taken to 0xf16b4 when `[sp-28] == 0x3`, falls through to 0xf14f4 when `[sp-28] != 0x3`.
  3 such word store accesses with resolved target `[sp-24] +0x2c`, including target `[sp-24] +0x2c`, value `0x185574` (`str` at 0xf1500), target `[sp-24] +0x2c`, value `0x186834` (`str` at 0xf15dc), target `[sp-24] +0x2c`, value `0x185ea8` (`str` at 0xf16bc).
  3 such word store accesses with resolved target `[sp-24] +0x28`, including target `[sp-24] +0x28`, value `0x35` (`str` at 0xf150c), target `[sp-24] +0x28`, value `0x38` (`str` at 0xf15e8), target `[sp-24] +0x28`, value `0x64` (`str` at 0xf16c8).
  3 such word load accesses with resolved target `[sp-24] +0x28`, including target `[sp-24] +0x28` (`ldr` at 0xf1514), target `[sp-24] +0x28` (`ldr` at 0xf15f0), target `[sp-24] +0x28` (`ldr` at 0xf16d0).
  Call (`bl` at 0xf1534): (unknown callee), target 0x201fc, r0=`[sp-40]`, r1=`unknown`, r2=`[[sp-24]+40]`, r3=`unknown`.
  3 such word store accesses with resolved target `[sp-24] +0x30`, including target `[sp-24] +0x30` (`str` at 0xf153c), target `[sp-24] +0x30` (`str` at 0xf1618), target `[sp-24] +0x30` (`str` at 0xf16f8).
  Call (`bl` at 0xf1548): (unknown callee), target 0x654f0, r0=`[sp-36]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x201fc)`.
  3 such word store accesses with resolved target `[sp-24] +0x24`, including target `[sp-24] +0x24` (`str` at 0xf154c), target `[sp-24] +0x24` (`str` at 0xf162c), target `[sp-24] +0x24` (`str` at 0xf170c).
  Call (`bl` at 0xf1560): (unknown callee), target 0x6e1b0, r0=`[sp-32]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0xf156c): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf15b4): (unknown callee), target 0x203f4, r0=`[sp-52]`, r1=`[sp-44]`, r2=`[sp-48]`, r3=`unknown`.
  Call (`bl` at 0xf15cc): (unknown callee), target 0x205bc, r0=`[sp-24]`, r1=`0x186bb4`, r2=`ret(0x203f4)`, r3=`[sp-36]`.
  Call (`bl` at 0xf1610): (unknown callee), target 0x201fc, r0=`[sp-40]`, r1=`unknown`, r2=`[[sp-24]+40]`, r3=`unknown`.
  Call (`bl` at 0xf1628): (unknown callee), target 0x740c4, r0=`[sp-32]`, r1=`[sp-36]`, r2=`unknown`, r3=`ret(0x201fc)`.
  Call (`bl` at 0xf1640): (unknown callee), target 0x6e1b0, r0=`[sp-32]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0xf164c): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf1694): (unknown callee), target 0x203f4, r0=`[sp-44]`, r1=`[sp-52]`, r2=`[sp-48]`, r3=`unknown`.
  Call (`bl` at 0xf16ac): (unknown callee), target 0x205bc, r0=`[sp-24]`, r1=`0x186bc0`, r2=`ret(0x203f4)`, r3=`[sp-36]`.
  Call (`bl` at 0xf16f0): (unknown callee), target 0x201fc, r0=`[sp-40]`, r1=`unknown`, r2=`[[sp-24]+40]`, r3=`unknown`.
  Call (`bl` at 0xf1708): (unknown callee), target 0x8ef38, r0=`[sp-32]`, r1=`[sp-36]`, r2=`unknown`, r3=`ret(0x201fc)`.
  Call (`bl` at 0xf1720): (unknown callee), target 0x6e1b0, r0=`[sp-32]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0xf172c): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf1774): (unknown callee), target 0x203f4, r0=`[sp-44]`, r1=`[sp-52]`, r2=`[sp-48]`, r3=`unknown`.
  Call (`bl` at 0xf178c): (unknown callee), target 0x205bc, r0=`[sp-24]`, r1=`0x186bcc`, r2=`ret(0x203f4)`, r3=`[sp-36]`.
  Word load: target `[sp-24] +0x30` (`ldr` at 0xf1794).
  Call (`bl` at 0xf17a0): (unknown callee), target 0x20550, r0=`[[sp-24]+48]`, r1=`0x0`, r2=`[sp-40]`, r3=`[sp-24]`.
  Recorded r0 = `ret(0x20550)` (`ldm` at 0xf17a8, `bl` at 0xf17a0).

## `daytona_monitor_int`

- `daytona_monitor_int`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x467c4 to 0x46824.
  Established facts:
  Call (`bl` at 0x46810): (unknown callee), target 0x114238, r0=`(r1 & 0xff)-3`, r1=`(r2 & 0xff)`, r2=`(r3 & 0xff)`, r3=`(r1 & 0xff)-3`.
  Recorded r0 = `ret(0x114238)` (`ldm` at 0x46820, `bl` at 0x46810).

## `L1NetInit`

- `L1NetInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x91954 to 0x91aec.
  Established facts:
  The instructions from 0x91aa8 to 0x91aec are not reached by any path from the entry, and their branches are not listed.
  2 such word load accesses with resolved target `abs 0x1a6454`, including target `abs 0x1a6454` (`ldr` at 0x91968), target `abs 0x1a6454` (`ldr` at 0x91998).
  Branch (`beq` at 0x91970): taken to 0x91990 when `[0x1a6454] == 0x0`, falls through to 0x91974 when `[0x1a6454] != 0x0`.
  Call (`bl` at 0x9198c): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x1686e0`, r2=`0x43`, r3=`0x0`.
  Word store: target `abs 0x1a6454`, value `[0x1a6454]+1` (`str` at 0x919a0).
  Call (`bl` at 0x919b0): (unknown callee), target 0x20550, r0=`0x31a734`, r1=`0x0`, r2=`0x110`, r3=`[0x1a6454]+1`.
  Call (`bl` at 0x919c0): (unknown callee), target 0xfb5cc, r0=`0x31a734`, r1=`0x11`, r2=`0x0`, r3=`unknown`.
  Call (`bl` at 0x919d0): (unknown callee), target 0xfb6ec, r0=`0x31a744`, r1=`0x1686e8`, r2=`0x400`, r3=`unknown`.
  Branch (`beq` at 0x919e4): taken to 0x91a14 when `ret(0xfb6ec) == 0x0`, falls through to 0x919e8 when `ret(0xfb6ec) != 0x0` (source `bl` at 0x919d0).
  Call (`bl` at 0x919ec): (unknown callee), target 0x219a4, r0=`ret(0xfb6ec)`, r1=`unknown`, r2=`unknown`, r3=`ret(0xfb6ec)`.
  Call (`bl` at 0x91a10): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x1686e0`, r2=`0x54`, r3=`0x0`.
  Call (`bl` at 0x91a24): (unknown callee), target 0x22b84, r0=`0x31a7e8`, r1=`0x168718`, r2=`0x1`, r3=`0x9`.
  Call (`bl` at 0x91a60): (unknown callee), target 0x22540, r0=`0x31a840`, r1=`0x168724`, r2=`0x9219c`, r3=`0x0`.
  Call (`bl` at 0x91a9c): (unknown callee), target 0x22540, r0=`0x31a83c`, r1=`0x168730`, r2=`0x91b0c`, r3=`0x0`.
  Recorded r0 = `ret(0x22540)` (`ldm` at 0x91aa4, `bl` at 0x91a9c).

## `networkInit`

- `networkInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x29f9c to 0x29ff4.
  Established facts:
  The instructions from 0x29fec to 0x29ff4 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x29fac): (unknown callee), target 0x28230, r0=`0x34`, r1=`r1`, r2=`r2`, r3=`r3`.
  Branch (`beq` at 0x29fb8): taken to 0x29fd8 when `ret(0x28230) == 0x0`, falls through to 0x29fbc when `ret(0x28230) != 0x0` (source `bl` at 0x29fac).
  Call (`bl` at 0x29fc0): (unknown callee), target 0x29908, r0=`0x153694`, r1=`unknown`, r2=`unknown`, r3=`ret(0x28230)`.
  Call (`bl` at 0x29fcc): (unknown callee), target 0x2624c, r0=`0x34`, r1=`0x0`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x29fd0): (unknown callee), target 0x29ff4, r0=`ret(0x2624c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x29fd8): (unknown callee), target 0x29ff4, r0=`ret(0x28230)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x28230)`.
  Word store: target `abs 0x19e424`, value `0x0` (`str` at 0x29fe4).
  Recorded r0 = `ret(0x29ff4)` (`ldm` at 0x29fe8, `bl` at 0x29fd0, `bl` at 0x29fd8).
