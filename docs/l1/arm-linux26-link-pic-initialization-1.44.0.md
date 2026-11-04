# ARM Linux 2.6 link and PIC initialization routines in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-link-pic-initialization-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register
state from the analysis rather than a C return type.

Groups count static records with the same width, access kind and resolved target expression. Address order need not describe one execution path.

## `NL4R_Cfg_Init`

- `NL4R_Cfg_Init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2d214 to 0x2d5a8.
  Established facts:
  The instructions from 0x2d590 to 0x2d5a8 are not reached by any path from the entry, and their branches are not listed.
  3 such word load accesses with resolved target `abs 0x1a5998`, including target `abs 0x1a5998` (`ldr` at 0x2d234), target `abs 0x1a5998` (`ldr` at 0x2d284), target `abs 0x1a5998` (`ldr` at 0x2d2cc).
  Branch (`bne` at 0x2d23c): taken to 0x2d2b4 when `[0x1a5998] != 0x0`, falls through to 0x2d240 when `[0x1a5998] == 0x0`.
  Call (`bl` at 0x2d240): (unknown callee), target 0x6e140, r0=`r0`, r1=`r1`, r2=`r2`, r3=`[0x1a5998]`.
  Word store: target `abs 0x1a599c` (`str` at 0x2d24c).
  Word load: target `abs 0x1a599c` (`ldr` at 0x2d254).
  Call (`bl` at 0x2d278): (unknown callee), target 0x204d8, r0=`[sp-44]`, r1=`0x1`, r2=`[0x1a599c]`, r3=`unknown`.
  Word store: target `abs 0x1a5998` (`str` at 0x2d280).
  Branch (`bne` at 0x2d28c): taken to 0x2d2b4 when `[0x1a5998] != 0x0`, falls through to 0x2d290 when `[0x1a5998] == 0x0`.
  Call (`bl` at 0x2d2b0): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x155f2c`, r2=`0x88`, r3=`0x0`.
  Word store: target `[sp-52] +0x0`, value `unknown` (`str` at 0x2d2e0).
  Word store: target `[sp-52] +0x8`, value `[sp-32]` (`str` at 0x2d2ec).
  3 such word load accesses with resolved target `[sp-52] +0x8`, including target `[sp-52] +0x8` (`ldr` at 0x2d2f8), target `[sp-52] +0x8` (`ldr` at 0x2d394), target `[sp-52] +0x8` (`ldr` at 0x2d3bc).
  Call (`bl` at 0x2d300): (unknown callee), target 0x204d8, r0=`[[sp-52]+8]`, r1=`0x4`, r2=`[sp-52]`, r3=`[sp-52]`.
  Word store: target `[sp-52] +0x4` (`str` at 0x2d308).
  4 such word load accesses with resolved target `[sp-52] +0x4`, including target `[sp-52] +0x4` (`ldr` at 0x2d30c), target `[sp-52] +0x4` (`ldr` at 0x2d3d8), target `[sp-52] +0x4` (`ldr` at 0x2d420).
  Branch (`bne` at 0x2d314): taken to 0x2d3b0 when `[[sp-52]+4] != 0x0`, falls through to 0x2d318 when `[[sp-52]+4] == 0x0`.
  Call (`bl` at 0x2d328): (unknown callee), target 0x6e1b0, r0=`[sp-24]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x2d334): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2d37c): (unknown callee), target 0x203f4, r0=`[sp-64]`, r1=`[sp-56]`, r2=`[sp-60]`, r3=`unknown`.
  Call (`bl` at 0x2d3ac): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x155f2c`, r2=`0x94`, r3=`0x0`.
  Branch (`ble` at 0x2d3c8): taken to 0x2d568 when `le flags condition (operands unknown)`, falls through to 0x2d3cc when `not (le flags condition (operands unknown))`.
  Byte store: target `(unknown effective address)`, value `([sp-24] & 0xff)` (`strb` at 0x2d3e4).
  Byte load: target `(unknown effective address)` (`ldrb` at 0x2d3f8).
  Call (`bl` at 0x2d404): (unknown callee), target 0x125958, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Byte store: target `(unknown effective address)`, value `0x1` (`strb` at 0x2d42c).
  Byte load: target `(unknown effective address)` (`ldrb` at 0x2d444).
  Branch (`beq` at 0x2d44c): taken to 0x2d558 when `unknown == 0x0`, falls through to 0x2d450 when `unknown != 0x0`.
  Call (`bl` at 0x2d450): (unknown callee), target 0x6e140, r0=`ret(0x125958)`, r1=`[sp-52]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2d460): (unknown callee), target 0x6e190, r0=`ret(0x6e140)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x6e140)`.
  2 such word load accesses with resolved target `[sp-52] +0x0`, including target `[sp-52] +0x0` (`ldr` at 0x2d480), target `[sp-52] +0x0` (`ldr` at 0x2d4cc).
  Word load: target `(unknown effective address)` (`ldr` at 0x2d488).
  Word load: target `(unknown effective address)` (`ldr` at 0x2d4d4).
  Word load: target `(unknown effective address)` (`ldr` at 0x2d4f8).
  Call (`bl` at 0x2d4fc): (unknown callee), target 0x13ba30, r0=`[sp-68]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2d518): (unknown callee), target 0x219a4, r0=`ret(0x13ba30)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13ba30)`.
  Word load: target `(unknown effective address)` (`ldr` at 0x2d538).
  Call (`bl` at 0x2d554): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x155f2c`, r2=`0xa1`, r3=`0x0`.
  Call (`bl` at 0x2d568): (unknown callee), target 0x6e190, r0=`unknown`, r1=`unknown`, r2=`[[sp-52]+8]`, r3=`unknown`.
  Call (`bl` at 0x2d57c): (unknown callee), target 0x3d9dc, r0=`ret(0x6e190)`, r1=`unknown`, r2=`ret(0x6e190)`, r3=`[sp-24]`.
  Recorded r0 = `0x0` (`ldm` at 0x2d58c).

## `NL4R_Remove`

- `NL4R_Remove`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2d5a8 to 0x2d68c.
  Established facts:
  The instructions from 0x2d680 to 0x2d688 are not reached by any path from the entry, and their branches are not listed.
  Word load: target `abs 0x1a599c` (`ldr` at 0x2d5c4).
  Branch (`bge` at 0x2d5cc): taken to 0x2d5fc when `r0 >= [0x1a599c] (signed, 32-bit)`, falls through to 0x2d5d0 when `r0 < [0x1a599c] (signed, 32-bit)`.
  2 such word load accesses with resolved target `abs 0x1a5998`, including target `abs 0x1a5998` (`ldr` at 0x2d5e8), target `abs 0x1a5998` (`ldr` at 0x2d620).
  Word load: target `(unknown effective address)` (`ldr` at 0x2d5f0).
  Branch (`bne` at 0x2d5f8): taken to 0x2d608 when `unknown != 0x0`, falls through to 0x2d5fc when `unknown == 0x0`.
  Word load: target `[sp-24] +0x8` (`ldr` at 0x2d630).
  Branch (`beq` at 0x2d638): taken to 0x2d66c when `[[sp-24]+8] == 0x0`, falls through to 0x2d63c when `[[sp-24]+8] != 0x0`.
  Word store: target `[sp-24] +0x8`, value `0x0` (`str` at 0x2d644).
  Word store: target `[sp-24] +0x0`, value `0x0` (`str` at 0x2d650).
  Word load: target `[sp-24] +0x4` (`ldr` at 0x2d658).
  Call (`bl` at 0x2d65c): (unknown callee), target 0x20514, r0=`[[sp-24]+4]`, r1=`r1`, r2=`[sp-24]`, r3=`[sp-24]`.
  Word store: target `[sp-24] +0x4`, value `0x0` (`str` at 0x2d668).
  Recorded r0 = `unknown` (`ldm` at 0x2d67c).

## `NL4R_Init`

- `NL4R_Init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2d68c to 0x2d8ec.
  Established facts:
  The instructions from 0x2d8d8 to 0x2d8e0 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x2d8e4 to 0x2d8ec are not reached by any path from the entry, and their branches are not listed.
  Word load: target `abs 0x1a599c` (`ldr` at 0x2d6a8).
  Branch (`bge` at 0x2d6b0): taken to 0x2d6e0 when `r0 >= [0x1a599c] (signed, 32-bit)`, falls through to 0x2d6b4 when `r0 < [0x1a599c] (signed, 32-bit)`.
  2 such word load accesses with resolved target `abs 0x1a5998`, including target `abs 0x1a5998` (`ldr` at 0x2d6cc), target `abs 0x1a5998` (`ldr` at 0x2d704).
  Word load: target `(unknown effective address)` (`ldr` at 0x2d6d4).
  Branch (`bne` at 0x2d6dc): taken to 0x2d6ec when `unknown != 0x0`, falls through to 0x2d6e0 when `unknown == 0x0`.
  Word load: target `[sp-44] +0x8` (`ldr` at 0x2d71c).
  Branch (`ble` at 0x2d728): taken to 0x2d8c4 when `le flags condition (operands unknown)`, falls through to 0x2d72c when `not (le flags condition (operands unknown))`.
  Word load: target `[sp-44] +0x0` (`ldr` at 0x2d740).
  Word load: target `[sp-44] +0x4` (`ldr` at 0x2d758).
  Call (`bl` at 0x2d764): (unknown callee), target 0x6e140, r0=`unknown`, r1=`[sp-44]`, r2=`unknown`, r3=`unknown`.
  2 such byte load accesses with resolved target `[sp-80] +0x0`, including target `[sp-80] +0x0` (`ldrb` at 0x2d778), target `[sp-80] +0x0` (`ldrb` at 0x2d7ac).
  Call (`bl` at 0x2d77c): (unknown callee), target 0x6e190, r0=`ret(0x6e140)`, r1=`unknown`, r2=`unknown`, r3=`[sp-80]`.
  2 such word load accesses with resolved target `[sp-76] +0x4`, including target `[sp-76] +0x4` (`ldr` at 0x2d790), target `[sp-76] +0x4` (`ldr` at 0x2d7c8).
  Byte load: target `[sp-80] +0x1` (`ldrb` at 0x2d7dc).
  Branch (`bne` at 0x2d7e4): taken to 0x2d7ec when `[[sp-80]+1]:1 != 0x0`, falls through to 0x2d7e8 when `[[sp-80]+1]:1 == 0x0`.
  2 such byte load accesses with resolved target `[sp-76] +0x1`, including target `[sp-76] +0x1` (`ldrb` at 0x2d7f0), target `[sp-76] +0x1` (`ldrb` at 0x2d824).
  Call (`bl` at 0x2d7fc): (unknown callee), target 0x125958, r0=`[sp-24]`, r1=`[[sp-76]+1]:1`, r2=`unknown`, r3=`[[sp-76]+1]:1`.
  Call (`bl` at 0x2d810): (unknown callee), target 0x558d8, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x125958)`.
  Call (`bl` at 0x2d834): (unknown callee), target 0x125b8c, r0=`[sp-24]`, r1=`[[sp-76]+1]:1`, r2=`sp-85`, r3=`[[sp-76]+1]:1`.
  Call (`bl` at 0x2d850): (unknown callee), target 0x219a4, r0=`ret(0x125b8c)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x125b8c)`.
  Call (`bl` at 0x2d87c): (unknown callee), target 0x22070, r0=`0x1`, r1=`0x155f2c`, r2=`0xe6`, r3=`0x0`.
  Branch (`bne` at 0x2d890): taken to 0x2d8b4 when `unknown != 0xc`, falls through to 0x2d894 when `unknown == 0xc`.
  Byte store: target `[sp-80] +0x2`, value `0x1` (`strb` at 0x2d89c).
  Byte load: target `[sp-76] +0x2` (`ldrb` at 0x2d8a4).
  Call (`bl` at 0x2d8b0): (unknown callee), target 0xb8924, r0=`[sp-24]`, r1=`[[sp-76]+2]:1`, r2=`[sp-80]`, r3=`[[sp-76]+2]:1`.
  Recorded r0 = `unknown` (`ldm` at 0x2d8d4).

## `NL4R_Target`

- `NL4R_Target`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2d8ec to 0x2da40.
  Established facts:
  The instructions from 0x2da34 to 0x2da3c are not reached by any path from the entry, and their branches are not listed.
  Word load: target `abs 0x1a599c` (`ldr` at 0x2d90c).
  Branch (`bge` at 0x2d914): taken to 0x2d944 when `r0 >= [0x1a599c] (signed, 32-bit)`, falls through to 0x2d918 when `r0 < [0x1a599c] (signed, 32-bit)`.
  2 such word load accesses with resolved target `abs 0x1a5998`, including target `abs 0x1a5998` (`ldr` at 0x2d930), target `abs 0x1a5998` (`ldr` at 0x2d968).
  Word load: target `(unknown effective address)` (`ldr` at 0x2d938).
  Branch (`bne` at 0x2d940): taken to 0x2d950 when `unknown != 0x0`, falls through to 0x2d944 when `unknown == 0x0`.
  Word load: target `[sp-28] +0x8` (`ldr` at 0x2d978).
  Branch (`bgt` at 0x2d984): taken to 0x2d994 when `[[sp-28]+8] > r1 (signed, 32-bit)`, falls through to 0x2d988 when `[[sp-28]+8] <= r1 (signed, 32-bit)`.
  Call (`bl` at 0x2d994): (unknown callee), target 0x6e140, r0=`r0`, r1=`r1`, r2=`[[sp-28]+8]`, r3=`r1`.
  Call (`bl` at 0x2d9a4): (unknown callee), target 0x6e190, r0=`ret(0x6e140)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x6e140)`.
  2 such word load accesses with resolved target `[sp-28] +0x0`, including target `[sp-28] +0x0` (`ldr` at 0x2d9c4), target `[sp-28] +0x0` (`ldr` at 0x2da10).
  Word load: target `(unknown effective address)` (`ldr` at 0x2d9cc).
  Word load: target `(unknown effective address)` (`ldr` at 0x2da18).
  Recorded r0 = `unknown` (`ldm` at 0x2da30).

## `NL4R_IsRunning`

- `NL4R_IsRunning`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2da40 to 0x2db1c.
  Established facts:
  The instructions from 0x2db10 to 0x2db18 are not reached by any path from the entry, and their branches are not listed.
  Word load: target `abs 0x1a599c` (`ldr` at 0x2da60).
  Branch (`bge` at 0x2da68): taken to 0x2da98 when `r0 >= [0x1a599c] (signed, 32-bit)`, falls through to 0x2da6c when `r0 < [0x1a599c] (signed, 32-bit)`.
  2 such word load accesses with resolved target `abs 0x1a5998`, including target `abs 0x1a5998` (`ldr` at 0x2da84), target `abs 0x1a5998` (`ldr` at 0x2dabc).
  Word load: target `(unknown effective address)` (`ldr` at 0x2da8c).
  Branch (`bne` at 0x2da94): taken to 0x2daa4 when `unknown != 0x0`, falls through to 0x2da98 when `unknown == 0x0`.
  Word load: target `[sp-28] +0x8` (`ldr` at 0x2dacc).
  Branch (`bgt` at 0x2dad8): taken to 0x2dae8 when `[[sp-28]+8] > r1 (signed, 32-bit)`, falls through to 0x2dadc when `[[sp-28]+8] <= r1 (signed, 32-bit)`.
  Word load: target `[sp-28] +0x4` (`ldr` at 0x2daf4).
  Byte load: target `(unknown effective address)` (`ldrb` at 0x2dafc).
  Recorded r0 = `unknown` (`ldm` at 0x2db0c).

## `nl4r_init_icntlr`

- `nl4r_init_icntlr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2eca4 to 0x2f150.
  Established facts:
  The instructions from 0x2f13c to 0x2f150 are not reached by any path from the entry, and their branches are not listed.
  4 such word load accesses with resolved target `abs 0x315034`, including target `abs 0x315034` (`ldr` at 0x2ecc0), target `abs 0x315034` (`ldr` at 0x2ed98), target `abs 0x315034` (`ldr` at 0x2ee8c).
  Branch (`beq` at 0x2ecc8): taken to 0x2ecdc when `[0x315034] == 0x0`, falls through to 0x2eccc when `[0x315034] != 0x0`.
  Call (`bl` at 0x2ecd8): (unknown callee), target 0x243d4, r0=`0x1561cc`, r1=`r0`, r2=`r1`, r3=`[0x315034]`.
  Branch (`beq` at 0x2ece8): taken to 0x2ede0 when `unknown == 0x0`, falls through to 0x2ecec when `unknown != 0x0`.
  Call (`bl` at 0x2ed04): (unknown callee), target 0x13f048, r0=`unknown`, r1=`0x6`, r2=`0x59`, r3=`sp-28`.
  Call (`bl` at 0x2ed18): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x4f`, r3=`sp-28`.
  Call (`bl` at 0x2ed2c): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x45`, r3=`sp-28`.
  Call (`bl` at 0x2ed40): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x3b`, r3=`sp-28`.
  Call (`bl` at 0x2ed54): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x31`, r3=`sp-28`.
  Call (`bl` at 0x2ed68): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x27`, r3=`sp-28`.
  Call (`bl` at 0x2ed7c): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x1d`, r3=`sp-28`.
  Call (`bl` at 0x2ed90): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x13`, r3=`sp-28`.
  Branch (`bls` at 0x2eda0): taken to 0x2edac when `[0x315034] <= 0x1 (unsigned, 32-bit)`, falls through to 0x2eda4 when `[0x315034] > 0x1 (unsigned, 32-bit)`.
  Call (`bl` at 0x2eda8): (unknown callee), target 0x243d4, r0=`0x1561f8`, r1=`unknown`, r2=`unknown`, r3=`[0x315034]`.
  Call (`bl` at 0x2edbc): (unknown callee), target 0x13fd34, r0=`[sp-20]`, r1=`0x2`, r2=`0x6`, r3=`0x0`.
  Branch (`bge` at 0x2edd0): taken to 0x2ede0 when `ret(0x13fd34) >= 0x0 (signed, 32-bit)`, falls through to 0x2edd4 when `ret(0x13fd34) < 0x0 (signed, 32-bit)` (source `bl` at 0x2edbc).
  Call (`bl` at 0x2edf8): (unknown callee), target 0x13f048, r0=`unknown`, r1=`0x6`, r2=`0x59`, r3=`sp-28`.
  Call (`bl` at 0x2ee0c): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x4f`, r3=`sp-28`.
  Call (`bl` at 0x2ee20): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x45`, r3=`sp-28`.
  Call (`bl` at 0x2ee34): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x3b`, r3=`sp-28`.
  Call (`bl` at 0x2ee48): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x31`, r3=`sp-28`.
  Call (`bl` at 0x2ee5c): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x27`, r3=`sp-28`.
  Call (`bl` at 0x2ee70): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x1d`, r3=`sp-28`.
  Call (`bl` at 0x2ee84): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x13`, r3=`sp-28`.
  Branch (`bls` at 0x2ee94): taken to 0x2eea0 when `[0x315034] <= 0x1 (unsigned, 32-bit)`, falls through to 0x2ee98 when `[0x315034] > 0x1 (unsigned, 32-bit)`.
  Call (`bl` at 0x2ee9c): (unknown callee), target 0x243d4, r0=`0x156230`, r1=`unknown`, r2=`unknown`, r3=`[0x315034]`.
  Call (`bl` at 0x2eeb0): (unknown callee), target 0x13fd34, r0=`[sp-20]`, r1=`0x2`, r2=`0x6`, r3=`0x0`.
  Branch (`bge` at 0x2eec4): taken to 0x2eed4 when `ret(0x13fd34) >= 0x0 (signed, 32-bit)`, falls through to 0x2eec8 when `ret(0x13fd34) < 0x0 (signed, 32-bit)` (source `bl` at 0x2eeb0).
  Branch (`beq` at 0x2eee0): taken to 0x2f128 when `unknown == 0x0`, falls through to 0x2eee4 when `unknown != 0x0`.
  Call (`bl` at 0x2eefc): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x5c`, r3=`sp-28`.
  Call (`bl` at 0x2ef10): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x52`, r3=`sp-28`.
  Call (`bl` at 0x2ef24): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x48`, r3=`sp-28`.
  Call (`bl` at 0x2ef38): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x3e`, r3=`sp-28`.
  Call (`bl` at 0x2ef4c): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x34`, r3=`sp-28`.
  Call (`bl` at 0x2ef60): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x2a`, r3=`sp-28`.
  Call (`bl` at 0x2ef74): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x20`, r3=`sp-28`.
  Call (`bl` at 0x2ef88): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x16`, r3=`sp-28`.
  Call (`bl` at 0x2efa4): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x5b`, r3=`sp-28`.
  Call (`bl` at 0x2efb8): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x51`, r3=`sp-28`.
  Call (`bl` at 0x2efcc): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x47`, r3=`sp-28`.
  Call (`bl` at 0x2efe0): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x3d`, r3=`sp-28`.
  Call (`bl` at 0x2eff4): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x33`, r3=`sp-28`.
  Call (`bl` at 0x2f008): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x29`, r3=`sp-28`.
  Call (`bl` at 0x2f01c): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x1f`, r3=`sp-28`.
  Call (`bl` at 0x2f030): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x15`, r3=`sp-28`.
  Call (`bl` at 0x2f04c): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x5a`, r3=`sp-28`.
  Call (`bl` at 0x2f060): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x50`, r3=`sp-28`.
  Call (`bl` at 0x2f074): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x46`, r3=`sp-28`.
  Call (`bl` at 0x2f088): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x3c`, r3=`sp-28`.
  Call (`bl` at 0x2f09c): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x32`, r3=`sp-28`.
  Call (`bl` at 0x2f0b0): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x28`, r3=`sp-28`.
  Call (`bl` at 0x2f0c4): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x1e`, r3=`sp-28`.
  Call (`bl` at 0x2f0d8): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x6`, r2=`0x14`, r3=`sp-28`.
  Branch (`bls` at 0x2f0e8): taken to 0x2f0f4 when `[0x315034] <= 0x1 (unsigned, 32-bit)`, falls through to 0x2f0ec when `[0x315034] > 0x1 (unsigned, 32-bit)`.
  Call (`bl` at 0x2f0f0): (unknown callee), target 0x243d4, r0=`0x156268`, r1=`unknown`, r2=`unknown`, r3=`[0x315034]`.
  Call (`bl` at 0x2f104): (unknown callee), target 0x13fd34, r0=`[sp-20]`, r1=`0x2`, r2=`0x6`, r3=`0x0`.
  Branch (`bge` at 0x2f118): taken to 0x2f128 when `ret(0x13fd34) >= 0x0 (signed, 32-bit)`, falls through to 0x2f11c when `ret(0x13fd34) < 0x0 (signed, 32-bit)` (source `bl` at 0x2f104).
  Recorded r0 = `unknown` (`ldm` at 0x2f138).

## `nl4r_setup_llp`

- `nl4r_setup_llp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2f650 to 0x2fabc.
  Established facts:
  The instructions from 0x2fab4 to 0x2fabc are not reached by any path from the entry, and their branches are not listed.
  Word load: target `abs 0x315034` (`ldr` at 0x2f670).
  Branch (`beq` at 0x2f678): taken to 0x2f68c when `[0x315034] == 0x0`, falls through to 0x2f67c when `[0x315034] != 0x0`.
  Call (`bl` at 0x2f688): (unknown callee), target 0x243d4, r0=`0x1563c8`, r1=`r0`, r2=`r1`, r3=`[0x315034]`.
  Branch (`beq` at 0x2f694): taken to 0x2f76c when `Z == 1`, falls through to 0x2f698 when `not (Z == 1)`.
  Call (`bl` at 0x2f6bc): (unknown callee), target 0x13f048, r0=`unknown`, r1=`unknown`, r2=`0x0`, r3=`sp-32`.
  Call (`bl` at 0x2f6dc): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]+10`, r2=`0x3`, r3=`sp-32`.
  Call (`bl` at 0x2f704): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]+10`, r2=`0x1f`, r3=`sp-32`.
  Call (`bl` at 0x2f724): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]+10`, r2=`0x20`, r3=`sp-32`.
  Call (`bl` at 0x2f744): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]+10`, r2=`0x21`, r3=`sp-32`.
  Call (`bl` at 0x2f764): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]+10`, r2=`0x22`, r3=`sp-32`.
  Call (`bl` at 0x2f790): (unknown callee), target 0x13f048, r0=`unknown`, r1=`unknown`, r2=`0x0`, r3=`sp-32`.
  Call (`bl` at 0x2f7b0): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]+10`, r2=`0x3`, r3=`sp-32`.
  Call (`bl` at 0x2f7d8): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]+10`, r2=`0x1f`, r3=`sp-32`.
  Call (`bl` at 0x2f7f8): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]+10`, r2=`0x20`, r3=`sp-32`.
  Call (`bl` at 0x2f818): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]+10`, r2=`0x21`, r3=`sp-32`.
  Call (`bl` at 0x2f838): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]+10`, r2=`0x22`, r3=`sp-32`.
  Call (`bl` at 0x2f860): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]+10`, r2=`0x23`, r3=`sp-32`.
  Call (`bl` at 0x2f880): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]+10`, r2=`0x24`, r3=`sp-32`.
  Call (`bl` at 0x2f8a0): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]+10`, r2=`0x25`, r3=`sp-32`.
  Call (`bl` at 0x2f8c0): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]+10`, r2=`0x26`, r3=`sp-32`.
  Call (`bl` at 0x2f8e8): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]+10`, r2=`0x2`, r3=`sp-32`.
  Call (`bl` at 0x2f908): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]+10`, r2=`0x4`, r3=`sp-32`.
  Call (`bl` at 0x2f930): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]+10`, r2=`0x5`, r3=`sp-32`.
  Call (`bl` at 0x2f958): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]+10`, r2=`0x6`, r3=`sp-32`.
  Branch (`beq` at 0x2f964): taken to 0x2f974 when `[sp-28] == 0x0`, falls through to 0x2f968 when `[sp-28] != 0x0`.
  Call (`bl` at 0x2f9a0): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]+18`, r2=`0x2`, r3=`sp-32`.
  Call (`bl` at 0x2f9c8): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]+18`, r2=`0x4`, r3=`sp-32`.
  Call (`bl` at 0x2f9f0): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]+18`, r2=`0x5`, r3=`sp-32`.
  Branch (`beq` at 0x2f9fc): taken to 0x2fa54 when `[sp-28] == 0x0`, falls through to 0x2fa00 when `[sp-28] != 0x0`.
  Call (`bl` at 0x2fa24): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]+26`, r2=`0x0`, r3=`sp-32`.
  Call (`bl` at 0x2fa4c): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]+26`, r2=`0x4`, r3=`sp-32`.
  Call (`bl` at 0x2fa78): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]+26`, r2=`0x0`, r3=`sp-32`.
  Call (`bl` at 0x2faa0): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]+26`, r2=`0x4`, r3=`sp-32`.
  Recorded r0 = `0x0` (`ldm` at 0x2fab0).

## `nl4r_is_scramble_needed`

- `nl4r_is_scramble_needed`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2fabc to 0x2fc94.
  Established facts:
  Call (`bl` at 0x2fad4): (unknown callee), target 0x6dc0c, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Branch (`beq` at 0x2faf4): taken to 0x2fbc8 when `[sp-28] == 0x44`, falls through to 0x2faf8 when `[sp-28] != 0x44`.
  Branch (`bgt` at 0x2fb00): taken to 0x2fb20 when `[sp-28] > 0x44 (signed, 32-bit)`, falls through to 0x2fb04 when `[sp-28] <= 0x44 (signed, 32-bit)`.
  Branch (`beq` at 0x2fb0c): taken to 0x2fb30 when `[sp-28] == 0x2d`, falls through to 0x2fb10 when `[sp-28] != 0x2d`.
  Branch (`beq` at 0x2fb18): taken to 0x2fb54 when `[sp-28] == 0x2e`, falls through to 0x2fb1c when `[sp-28] != 0x2e`.
  Branch (`beq` at 0x2fb28): taken to 0x2fb54 when `[sp-28] == 0x7d`, falls through to 0x2fb2c when `[sp-28] != 0x7d`.
  Branch (`bne` at 0x2fb5c): taken to 0x2fb6c when `[sp-28] != 0x2e`, falls through to 0x2fb60 when `[sp-28] == 0x2e`.
  Call (`bl` at 0x2fbcc): (unknown callee), target 0x28230, r0=`0x98`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0x2fbec): taken to 0x2fbfc when `unknown == 0x0`, falls through to 0x2fbf0 when `unknown != 0x0`.
  Call (`bl` at 0x2fc08): (unknown callee), target 0xa3eac, r0=`[sp-24]+1`, r1=`unknown`, r2=`ret(0x28230)`, r3=`[sp-24]+1`.
  5 such word load accesses with resolved target `ret(0xa3eac) +0x10`, including target `ret(0xa3eac) +0x10` (`ldr` at 0x2fc18, `bl` at 0x2fc08), target `ret(0xa3eac) +0x10` (`ldr` at 0x2fc28, `bl` at 0x2fc08), target `ret(0xa3eac) +0x10` (`ldr` at 0x2fc38, `bl` at 0x2fc08).
  Branch (`beq` at 0x2fc20): taken to 0x2fc64 when `[ret(0xa3eac)+16] == 0x44`, falls through to 0x2fc24 when `[ret(0xa3eac)+16] != 0x44` (source `bl` at 0x2fc08).
  Branch (`beq` at 0x2fc30): taken to 0x2fc64 when `[ret(0xa3eac)+16] == 0x40`, falls through to 0x2fc34 when `[ret(0xa3eac)+16] != 0x40` (source `bl` at 0x2fc08).
  Branch (`beq` at 0x2fc40): taken to 0x2fc64 when `[ret(0xa3eac)+16] == 0x7d`, falls through to 0x2fc44 when `[ret(0xa3eac)+16] != 0x7d` (source `bl` at 0x2fc08).
  Branch (`beq` at 0x2fc50): taken to 0x2fc64 when `[ret(0xa3eac)+16] == 0x2e`, falls through to 0x2fc54 when `[ret(0xa3eac)+16] != 0x2e` (source `bl` at 0x2fc08).
  Branch (`bne` at 0x2fc60): taken to 0x2fc74 when `[ret(0xa3eac)+16] != 0x2d`, falls through to 0x2fc64 when `[ret(0xa3eac)+16] == 0x2d` (source `bl` at 0x2fc08).
  Word load: target `ret(0xa3eac) +0x18` (`ldr` at 0x2fc68, `bl` at 0x2fc08).
  Branch (`bne` at 0x2fc70): taken to 0x2fc80 when `[ret(0xa3eac)+24] != 0x0`, falls through to 0x2fc74 when `[ret(0xa3eac)+24] == 0x0` (source `bl` at 0x2fc08).
  Recorded r0 = `unknown` (`ldm` at 0x2fc90).

## `nl4r_is_connected_to_nl4r`

- `nl4r_is_connected_to_nl4r`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2fc94 to 0x2fe74.
  Established facts:
  Call (`bl` at 0x2fcb4): (unknown callee), target 0x6dc0c, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r1`.
  Branch (`bne` at 0x2fcc4): taken to 0x2fd3c when `unknown != 0x2e`, falls through to 0x2fcc8 when `unknown == 0x2e`.
  Branch (`bne` at 0x2fcd0): taken to 0x2fce0 when `[sp-24] != 0x0`, falls through to 0x2fcd4 when `[sp-24] == 0x0`.
  Branch (`beq` at 0x2fce8): taken to 0x2fd04 when `[sp-24] == 0x0`, falls through to 0x2fcec when `[sp-24] != 0x0`.
  Branch (`bhi` at 0x2fcf4): taken to 0x2fd04 when `[sp-24] > 0x5 (unsigned, 32-bit)`, falls through to 0x2fcf8 when `[sp-24] <= 0x5 (unsigned, 32-bit)`.
  Call (`bl` at 0x2fd3c): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`bne` at 0x2fd4c): taken to 0x2fdc4 when `unknown != 0x40`, falls through to 0x2fd50 when `unknown == 0x40`.
  Branch (`beq` at 0x2fd58): taken to 0x2fd74 when `[sp-24] == 0x0`, falls through to 0x2fd5c when `[sp-24] != 0x0`.
  Branch (`bhi` at 0x2fd64): taken to 0x2fd74 when `[sp-24] > 0x4 (unsigned, 32-bit)`, falls through to 0x2fd68 when `[sp-24] <= 0x4 (unsigned, 32-bit)`.
  Branch (`beq` at 0x2fd7c): taken to 0x2fd90 when `[sp-24] == 0x0`, falls through to 0x2fd80 when `[sp-24] != 0x0`.
  Call (`bl` at 0x2fdc4): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`bne` at 0x2fdd4): taken to 0x2fdf8 when `unknown != 0x2d`, falls through to 0x2fdd8 when `unknown == 0x2d`.
  Call (`bl` at 0x2fe04): (unknown callee), target 0xa3eac, r0=`[sp-36]+1`, r1=`unknown`, r2=`unknown`, r3=`[sp-36]+1`.
  4 such word load accesses with resolved target `ret(0xa3eac) +0x10`, including target `ret(0xa3eac) +0x10` (`ldr` at 0x2fe14, `bl` at 0x2fe04), target `ret(0xa3eac) +0x10` (`ldr` at 0x2fe24, `bl` at 0x2fe04), target `ret(0xa3eac) +0x10` (`ldr` at 0x2fe34, `bl` at 0x2fe04).
  Branch (`beq` at 0x2fe1c): taken to 0x2fe54 when `[ret(0xa3eac)+16] == 0x44`, falls through to 0x2fe20 when `[ret(0xa3eac)+16] != 0x44` (source `bl` at 0x2fe04).
  Branch (`beq` at 0x2fe2c): taken to 0x2fe54 when `[ret(0xa3eac)+16] == 0x40`, falls through to 0x2fe30 when `[ret(0xa3eac)+16] != 0x40` (source `bl` at 0x2fe04).
  Branch (`beq` at 0x2fe3c): taken to 0x2fe54 when `[ret(0xa3eac)+16] == 0x2e`, falls through to 0x2fe40 when `[ret(0xa3eac)+16] != 0x2e` (source `bl` at 0x2fe04).
  Branch (`beq` at 0x2fe4c): taken to 0x2fe54 when `[ret(0xa3eac)+16] == 0x2d`, falls through to 0x2fe50 when `[ret(0xa3eac)+16] != 0x2d` (source `bl` at 0x2fe04).
  Recorded r0 = `unknown` (`ldm` at 0x2fe70).

## `TIOTest_SetSystemAceMode`

- `TIOTest_SetSystemAceMode`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x338cc to 0x33998.
  Established facts:
  Call (`bl` at 0x338e0): (unknown callee), target 0x28230, r0=`0x99`, r1=`r1`, r2=`r2`, r3=`r3`.
  Branch (`beq` at 0x338f8): taken to 0x33908 when `ret(0x28230) == 0x0`, falls through to 0x338fc when `ret(0x28230) != 0x0` (source `bl` at 0x338e0).
  Call (`bl` at 0x33920): (unknown callee), target 0x125a58, r0=`0x0`, r1=`0xb`, r2=`0x80`, r3=`[sp-32]`.
  Branch (`beq` at 0x33934): taken to 0x33944 when `[sp-20] == 0x0`, falls through to 0x33938 when `[sp-20] != 0x0`.
  Call (`bl` at 0x3395c): (unknown callee), target 0x125a58, r0=`0x0`, r1=`0xc`, r2=`0x38`, r3=`[sp-36]`.
  Branch (`beq` at 0x33970): taken to 0x33988 when `ret(0x125a58) == 0x0`, falls through to 0x33974 when `ret(0x125a58) != 0x0` (source `bl` at 0x3395c).
  Branch (`bne` at 0x3397c): taken to 0x33988 when `[sp-28] != 0x0`, falls through to 0x33980 when `[sp-28] == 0x0`.
  Recorded r0 = `[sp-28]` (`ldm` at 0x33994).

## `AthenaCommPollInit`

- `AthenaCommPollInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x33998 to 0x339f8.
  Established facts:
  The instructions from 0x339ec to 0x339f8 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x339e0): (unknown callee), target 0x22540, r0=`0x1a59ac`, r1=`0x158670`, r2=`0x339f8`, r3=`0x1`.
  Recorded r0 = `ret(0x22540)` (`ldm` at 0x339e8, `bl` at 0x339e0).

## `athenaComm`

- `athenaComm`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x339f8 to 0x33aac.
  Established facts:
  The instructions from 0x33aa0 to 0x33aac are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x33a14): (unknown callee), target 0x74020, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0x33a20): taken to 0x33a10 when `ret(0x74020) == 0x0`, falls through to 0x33a24 when `ret(0x74020) != 0x0` (source `bl` at 0x33a14).
  Call (`bl` at 0x33a34): (unknown callee), target 0x125b8c, r0=`0x0`, r1=`0x9`, r2=`sp-25`, r3=`sp-25`.
  Branch (`beq` at 0x33a4c): taken to 0x33a6c when `unknown == 0x0`, falls through to 0x33a50 when `unknown != 0x0`.
  Call (`bl` at 0x33a68): (unknown callee), target 0x22070, r0=`0x1`, r1=`0x158518`, r2=`0x8ca`, r3=`0x0`.
  Branch (`beq` at 0x33a78): taken to 0x33a94 when `unknown == 0x0`, falls through to 0x33a7c when `unknown != 0x0`.
  Call (`bl` at 0x33a8c): (unknown callee), target 0x743d4, r0=`0x0`, r1=`0x9`, r2=`0x2`, r3=`[sp-25]:1`.
  Call (`bl` at 0x33a98): (unknown callee), target 0x33aac, r0=`0xfa`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  The returns section reports no exit for this span.

## `PICInit`

- `PICInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3c040 to 0x3c2c8.
  Established facts:
  The instructions from 0x3c2a0 to 0x3c2c8 are not reached by any path from the entry, and their branches are not listed.
  2 such word load accesses with resolved target `abs 0x1a5a34`, including target `abs 0x1a5a34` (`ldr` at 0x3c05c), target `abs 0x1a5a34` (`ldr` at 0x3c1ec).
  Branch (`beq` at 0x3c064): taken to 0x3c090 when `[0x1a5a34] == 0x0`, falls through to 0x3c068 when `[0x1a5a34] != 0x0`.
  2 such byte load accesses with resolved target `abs 0x1a5a38`, including target `abs 0x1a5a38` (`ldrb` at 0x3c074), target `abs 0x1a5a38` (`ldrb` at 0x3c1b4).
  Call (`bl` at 0x3c08c): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x15a4a0`, r2=`0x3b`, r3=`0x0`.
  Call (`bl` at 0x3c0b0): (unknown callee), target 0x790b4, r0=`0x0`, r1=`0x3c798`, r2=`0x3c7c0`, r3=`0x3c814`.
  Branch (`bge` at 0x3c0c8): taken to 0x3c184 when `ge flags condition (operands unknown)`, falls through to 0x3c0cc when `not (ge flags condition (operands unknown))`.
  Word load: target `(unknown effective address)` (`ldr` at 0x3c104).
  Word load: target `(unknown effective address)` (`ldr` at 0x3c108).
  Call (`bl` at 0x3c10c): (unknown callee), target 0x13ba30, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0x3c120): taken to 0x3c174 when `ret(0x13ba30) == 0x0`, falls through to 0x3c124 when `ret(0x13ba30) != 0x0` (source `bl` at 0x3c10c).
  Call (`bl` at 0x3c128): (unknown callee), target 0x219a4, r0=`ret(0x13ba30)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13ba30)`.
  Word load: target `(unknown effective address)` (`ldr` at 0x3c154).
  Call (`bl` at 0x3c170): (unknown callee), target 0x22070, r0=`0x1`, r1=`0x15a4a0`, r2=`0x49`, r3=`0x0`.
  Word store: target `abs 0x1a5a34`, value `[sp-20]` (`str` at 0x3c18c).
  Byte store: target `abs 0x1a5a38`, value `([sp-24] & 0xff)` (`strb` at 0x3c198).
  3 such byte store accesses with resolved target `abs 0x1a5a39`, including target `abs 0x1a5a39`, value `0x1` (`strb` at 0x3c1ac), target `abs 0x1a5a39`, value `0x0` (`strb` at 0x3c258), target `abs 0x1a5a39`, value `0x0` (`strb` at 0x3c284).
  Branch (`ble` at 0x3c1c0): taken to 0x3c298 when `[0x1a5a38]:1 <= [sp-40] (signed, 32-bit)`, falls through to 0x3c1c4 when `[0x1a5a38]:1 > [sp-40] (signed, 32-bit)`.
  Byte load: target `abs 0x1a5a39` (`ldrb` at 0x3c1c8).
  Branch (`beq` at 0x3c1d0): taken to 0x3c298 when `[0x1a5a39]:1 == 0x0`, falls through to 0x3c1d4 when `[0x1a5a39]:1 != 0x0`.
  Byte load: target `[sp-28] +0x0` (`ldrb` at 0x3c1fc).
  Call (`bl` at 0x3c20c): (unknown callee), target 0x125b8c, r0=`0x0`, r1=`[[sp-28]]:1`, r2=`sp-29`, r3=`[[sp-28]]:1`.
  Branch (`beq` at 0x3c220): taken to 0x3c260 when `ret(0x125b8c) == 0x0`, falls through to 0x3c224 when `ret(0x125b8c) != 0x0` (source `bl` at 0x3c20c).
  Call (`bl` at 0x3c228): (unknown callee), target 0x21ce0, r0=`ret(0x125b8c)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x125b8c)`.
  Call (`bl` at 0x3c24c): (unknown callee), target 0x22070, r0=`0x1`, r1=`0x15a4a0`, r2=`0x5a`, r3=`0x0`.
  Byte load: target `[sp-28] +0x1` (`ldrb` at 0x3c264).
  Branch (`bne` at 0x3c278): taken to 0x3c288 when `unknown != 0x0`, falls through to 0x3c27c when `unknown == 0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x3c29c).

## `PICActive`

- `PICActive`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3c798 to 0x3c7c0.
  Established facts:
  The instructions from 0x3c7bc to 0x3c7c0 are not reached by any path from the entry, and their branches are not listed.
  Byte load: target `abs 0x1a5a39` (`ldrb` at 0x3c7b0).
  Recorded r0 = `[0x1a5a39]:1` (`ldm` at 0x3c7b8).

## `nmiSlot`

- `nmiSlot`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6bd10 to 0x6bed4.
  Established facts:
  The instructions from 0x6becc to 0x6bed4 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x6bd34): (unknown callee), target 0x3e58c, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r2`.
  Branch (`bge` at 0x6bd40): taken to 0x6bd58 when `ret(0x3e58c) >= 0x0 (signed, 32-bit)`, falls through to 0x6bd44 when `ret(0x3e58c) < 0x0 (signed, 32-bit)` (source `bl` at 0x6bd34).
  Call (`bl` at 0x6bd48): (unknown callee), target 0x3e58c, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x6bd5c): (unknown callee), target 0x3e58c, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Branch (`bne` at 0x6bd68): taken to 0x6bd78 when `ret(0x3e58c) != 0x0`, falls through to 0x6bd6c when `ret(0x3e58c) == 0x0` (source `bl` at 0x6bd5c).
  Call (`bl` at 0x6bd7c): (unknown callee), target 0x3f474, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Branch (`bne` at 0x6bd88): taken to 0x6bd98 when `ret(0x3f474) != 0x0`, falls through to 0x6bd8c when `ret(0x3f474) == 0x0` (source `bl` at 0x6bd7c).
  Call (`bl` at 0x6bd9c): (unknown callee), target 0x3ec6c, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3f474)`.
  Branch (`bne` at 0x6bda8): taken to 0x6bdb8 when `ret(0x3ec6c) != 0x0`, falls through to 0x6bdac when `ret(0x3ec6c) == 0x0` (source `bl` at 0x6bd9c).
  Branch (`beq` at 0x6bdc0): taken to 0x6be4c when `[sp-28] == 0x8`, falls through to 0x6bdc4 when `[sp-28] != 0x8`.
  Call (`bl` at 0x6bdd4): (unknown callee), target 0x6e1b0, r0=`[sp-24]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x6bde0): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x6be28): (unknown callee), target 0x203f4, r0=`[sp-44]`, r1=`[sp-36]`, r2=`[sp-40]`, r3=`unknown`.
  Call (`bl` at 0x6be34): (unknown callee), target 0x6c04c, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x6be48): (unknown callee), target 0x29908, r0=`0x1643bc`, r1=`ret(0x203f4)`, r2=`ret(0x6c04c)`, r3=`ret(0x6c04c)`.
  2 such word load accesses with resolved target `abs 0x31a520`, including target `abs 0x31a520` (`ldr` at 0x6be58), target `abs 0x31a520` (`ldr` at 0x6be7c).
  Word load: target `(unknown effective address)` (`ldr` at 0x6be60).
  Word load: target `(unknown effective address)` (`ldr` at 0x6be64).
  Branch (`beq` at 0x6be6c): taken to 0x6beac when `unknown == 0x0`, falls through to 0x6be70 when `unknown != 0x0`.
  Word load: target `(unknown effective address)` (`ldr` at 0x6be84).
  Word load: target `(unknown effective address)` (`ldr` at 0x6be8c).
  Call (`mov` at 0x6be9c): (unknown callee), target (unknown target), r0=`[sp-24]`, r1=`[sp-29]:1`, r2=`[sp-29]:1`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x6bebc).
