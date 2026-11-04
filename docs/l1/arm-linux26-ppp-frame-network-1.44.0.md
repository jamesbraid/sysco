# ARM Linux 2.6 PPP, frame and network routines in 1.44.0

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


[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-ppp-frame-network-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register state
from the analysis rather than a C return type. Groups summarize static records by printed
access shape. Address order need not describe one execution path.

## `L1NetActive`

- `L1NetActive`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x91aec to 0x91b0c.
  Established facts:
  The instructions from 0x91b08 to 0x91b0c are not reached by any path from the entry, and their branches are not listed.
  Byte load: target `abs 0x31a73c` (`ldrb` at 0x91afc).
  Recorded r0 = `[0x31a73c]:1` (`ldm` at 0x91b04).

## `networkGetType`

- `networkGetType`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2a100 to 0x2a120.
  Established facts:
  The instructions from 0x2a11c to 0x2a120 are not reached by any path from the entry, and their branches are not listed.
  Byte load: target `abs 0x1a5974` (`ldrb` at 0x2a110).
  Recorded r0 = `[0x1a5974]:1` (`ldm` at 0x2a118).

## `PPPInit`

- `PPPInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x100280 to 0x100354.
  Established facts:
  Word store: target `[sp-20] +0x8`, value `[sp-24]` (`str` at 0x1002b8).
  Word store: target `[sp-20] +0xc`, value `[sp-28]` (`str` at 0x1002c4).
  Word store: target `[sp-20] +0x18`, value `[sp-32]` (`str` at 0x1002d0).
  Word store: target `[sp-20] +0x1c`, value `[sp]` (`str` at 0x1002dc).
  Word store: target `[sp-20] +0x28`, value `unknown` (`str` at 0x1002f4).
  Word store: target `[sp-20] +0x2c`, value `unknown` (`str` at 0x10030c).
  Word store: target `[sp-20] +0x30`, value `[sp+12]` (`str` at 0x100318).
  Word store: target `[sp-20] +0x34`, value `[sp+16]` (`str` at 0x100324).
  Word store: target `[sp-20] +0x0`, value `[sp+20]` (`str` at 0x100330).
  Word store: target `[sp-20] +0x4`, value `[sp+24]` (`str` at 0x10033c).
  Call (`bl` at 0x1002ac): (unknown callee), target 0x20550, r0=`r0`, r1=`0x0`, r2=`0x3c`, r3=`r3`.
  Call (`bl` at 0x100348): (unknown callee), target 0x100354, r0=`[sp-20]`, r1=`0x3`, r2=`[sp-20]`, r3=`[sp+24]`.
  Recorded r0 = `ret(0x100354)` (`ldm` at 0x100350, `bl` at 0x100348).

## `PPPRead`

- `PPPRead`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1008c8 to 0x101248.
  Established facts:
  The instructions from 0x101234 to 0x101238 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x101244 to 0x101248 are not reached by any path from the entry, and their branches are not listed.
  Word load: target `r0 +0x30` (`ldr` at 0x10094c).
  Word load: target `r0 +0x34` (`ldr` at 0x100958).
  Word store: target `[sp-24] +0x14`, value `[[sp-24]+20]-1` (`str` at 0x100a44).
  Grouped accesses: 2 reported byte store records with printed base expression `([sp-64] + [sp-56])` and resolved offset `+0x0`, including target `([sp-64] + [sp-56]) +0x0`, value `([sp-45]:1 & 0xff)` (`strb` at 0x100a60), target `([sp-64] + [sp-56]) +0x0`, value `0x55` (`strb` at 0x10116c).
  Byte store: target `([sp-64]+1 + [sp-56]) +0x0`, value `unknown` (`strb` at 0x101190).
  Grouped accesses: 4 reported word load records with printed base expression `[sp-24]` and resolved offset `+0x38`, including target `[sp-24] +0x38` (`ldr` at 0x100b24), target `[sp-24] +0x38` (`ldr` at 0x100d4c), target `[sp-24] +0x38` (`ldr` at 0x100de8).
  Grouped accesses: 5 reported word store records with printed base expression `[sp-24]` and resolved offset `+0x38`, including target `[sp-24] +0x38`, value `0x1` (`str` at 0x100d0c), target `[sp-24] +0x38`, value `0x0` (`str` at 0x100d7c), target `[sp-24] +0x38`, value `0x0` (`str` at 0x100eb0).
  Grouped accesses: 2 reported word load records with absolute target `0x1b12e8`, including target `abs 0x1b12e8` (`ldr` at 0x101044), target `abs 0x1b12e8` (`ldr` at 0x101054).
  Call (`bl` at 0x100a0c): (unknown callee), target 0x203b8, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x100aa0): (unknown callee), target 0x203f4, r0=`([sp-56] + [sp-64]+1)+1`, r1=`[[sp-24]+16]`, r2=`[[sp-24]+20]`, r3=`([sp-56] + [sp-64]+1)+1`.
  Call (`bl` at 0x100b7c): (unknown callee), target 0x1001fc, r0=`[sp-24]`, r1=`0xfffffd41`, r2=`([[sp-36]] + [sp-96])`, r3=`[sp-76]`.
  Call (`bl` at 0x100bdc): (unknown callee), target 0x1001fc, r0=`[sp-24]`, r1=`0xfffffd42`, r2=`([[sp-36]] + [sp-96])`, r3=`[sp-76]`.
  Call (`bl` at 0x100c8c): (unknown callee), target 0x10008c, r0=`0xffff`, r1=`([[sp-36]] + [sp-96])`, r2=`[sp-76]`, r3=`sp-52`.
  Call (`bl` at 0x100cf4): (unknown callee), target 0x1001fc, r0=`[sp-24]`, r1=`0xfffffd44`, r2=`([[sp-36]] + [sp-96])`, r3=`[sp-76]`.
  Call (`bl` at 0x100dc0): (unknown callee), target 0x1001fc, r0=`[sp-24]`, r1=`0xfffffd40`, r2=`([[sp-36]] + [sp-96])`, r3=`[sp-76]`.
  Call (`bl` at 0x100dd4): (unknown callee), target 0x20514, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`[sp-56]`.
  Call (`bl` at 0x100e34): (unknown callee), target 0x1001fc, r0=`[sp-24]`, r1=`unknown`, r2=`sp-45`, r3=`0x1`.
  Call (`bl` at 0x100e90): (unknown callee), target 0x203b8, r0=`[[sp-36]]`, r1=`[sp-100]`, r2=`[sp]`, r3=`[sp-36]`.
  Call (`bl` at 0x100ee4): (unknown callee), target 0x1001fc, r0=`[sp-24]`, r1=`0xfffffd3e`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x100ef8): (unknown callee), target 0x20514, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`[sp-56]`.
  Call (`bl` at 0x100f8c): (unknown callee), target 0x1001fc, r0=`[sp-24]`, r1=`unknown`, r2=`([[sp-36]] + [sp-96])`, r3=`[sp-76]`.
  Call (`bl` at 0x101058): (unknown callee), target 0x22944, r0=`[0x1b12e8]`, r1=`unknown`, r2=`unknown`, r3=`0x1b12e8`.
  Call (`mov` at 0x101094): (unknown callee), target (unknown target), r0=`[sp-32]`, r1=`unknown`, r2=`[[sp-24]+12]`, r3=`unknown`.
  Call (`bl` at 0x1010ec): (unknown callee), target 0x1001fc, r0=`[sp-24]`, r1=`[[sp-24]+20]`, r2=`([[sp-36]] + [sp-96])`, r3=`[sp-76]`.
  Call (`bl` at 0x10110c): (unknown callee), target 0x20514, r0=`[sp-56]`, r1=`unknown`, r2=`[sp-24]`, r3=`[sp-56]`.
  Call (`bl` at 0x101148): (unknown callee), target 0x203b8, r0=`[sp-56]`, r1=`[sp-60]+256`, r2=`[sp-64]+3`, r3=`[sp-60]+256`.
  Call (`bl` at 0x101210): (unknown callee), target 0x20514, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0x100ad0): taken to 0x100ad8 when `[sp-45]:1 == 0x7e`, falls through to 0x100ad4 when `[sp-45]:1 != 0x7e`.
  Branch (`beq` at 0x100ae8): taken to 0x100d48 when `[sp-45]:1 == 0x7d`, falls through to 0x100aec when `[sp-45]:1 != 0x7d`.
  Branch (`bgt` at 0x100af4): taken to 0x100b08 when `[sp-45]:1 > 0x7d (signed, 32-bit)`, falls through to 0x100af8 when `[sp-45]:1 <= 0x7d (signed, 32-bit)`.
  Branch (`beq` at 0x100b00): taken to 0x100d64 when `[sp-45]:1 == 0xd`, falls through to 0x100b04 when `[sp-45]:1 != 0xd`.
  Branch (`beq` at 0x100b10): taken to 0x100b18 when `[sp-45]:1 == 0x7e`, falls through to 0x100b14 when `[sp-45]:1 != 0x7e`.
  Branch (`beq` at 0x100b2c): taken to 0x100cf8 when `[[sp-24]+56] == 0x0`, falls through to 0x100b30 when `[[sp-24]+56] != 0x0`.
  Branch (`bne` at 0x100ca4): taken to 0x100cb4 when `unknown != 0xf0b8`, falls through to 0x100ca8 when `unknown == 0xf0b8`.
  Branch (`bne` at 0x100df0): taken to 0x100e44 when `[[sp-24]+56] != 0x0`, falls through to 0x100df4 when `[[sp-24]+56] == 0x0`.
  Branch (`beq` at 0x100e4c): taken to 0x100e64 when `[sp-88] == 0x0`, falls through to 0x100e50 when `[sp-88] != 0x0`.
  Branch (`blo` at 0x101130): taken to 0x101154 when `[sp-64]+3 < [sp-60] (unsigned, 32-bit)`, falls through to 0x101134 when `[sp-64]+3 >= [sp-60] (unsigned, 32-bit)`.
  Recorded r0 = `unknown` (`ldm` at 0x101224).

## `PPPDecodeInit`

- `PPPDecodeInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x101248 to 0x1012c0.
  Established facts:
  Word store: target `[sp-20] +0x0`, value `[sp-32]` (`str` at 0x101280).
  Word store: target `[sp-20] +0x4`, value `[sp]` (`str` at 0x10128c).
  Word store: target `[sp-20] +0x8`, value `[sp-24]` (`str` at 0x101298).
  Word store: target `[sp-20] +0x14`, value `[sp-28]` (`str` at 0x1012a4).
  Call (`bl` at 0x101274): (unknown callee), target 0x20550, r0=`r0`, r1=`0x0`, r2=`0x34`, r3=`r3`.
  Call (`bl` at 0x1012ac): (unknown callee), target 0x1012c0, r0=`[sp-20]`, r1=`unknown`, r2=`[sp-20]`, r3=`[sp-28]`.
  Recorded r0 = `0x0` (`ldm` at 0x1012bc).

## `PPPDecode`

- `PPPDecode`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1012ec to 0x10184c.
  Established facts:
  The instructions from 0x101844 to 0x101848 are not reached by any path from the entry, and their branches are not listed.
  Word load: target `r0 +0x30` (`ldr` at 0x10131c).
  Word store: target `r0 +0x28`, value `0x1` (`str` at 0x101330).
  Word store: target `[sp-20] +0x30`, value `0x0` (`str` at 0x101360).
  Word store: target `[sp-20] +0x28`, value `unknown` (`str` at 0x101808).
  Grouped accesses: 3 reported word store records with printed base expression `[sp-20]` and resolved offset `+0x2c`, including target `[sp-20] +0x2c`, value `0x0` (`str` at 0x10133c), target `[sp-20] +0x2c`, value `0x1` (`str` at 0x1015d4), target `[sp-20] +0x2c`, value `unknown` (`str` at 0x1017f0).
  Grouped accesses: 9 reported word store records with printed base expression `[sp-20]` and resolved offset `+0xc`, including target `[sp-20] +0xc`, value `0x0` (`str` at 0x10134c), target `[sp-20] +0xc`, value `[[sp-20]+8]` (`str` at 0x10139c), target `[sp-20] +0xc`, value `[[sp-20]+12]-1` (`str` at 0x1014c8).
  Grouped accesses: 6 reported word store records with printed base expression `[sp-20]` and resolved offset `+0x1c`, including target `[sp-20] +0x1c`, value `0x0` (`str` at 0x101354), target `[sp-20] +0x1c`, value `[sp-20]+24` (`str` at 0x1013b8), target `[sp-20] +0x1c`, value `0x0` (`str` at 0x1015ac).
  Grouped accesses: 4 reported word store records with printed base expression `[sp-20]` and resolved offset `+0x20`, including target `[sp-20] +0x20`, value `0x0` (`str` at 0x1013a8), target `[sp-20] +0x20`, value `0x0` (`str` at 0x1015fc), target `[sp-20] +0x20`, value `0x0` (`str` at 0x1016e0).
  Grouped accesses: 2 reported word store records with printed base expression `[sp-20]` and resolved offset `+0x24`, including target `[sp-20] +0x24`, value `0x0` (`str` at 0x1013c4), target `[sp-20] +0x24`, value `0x0` (`str` at 0x1016ec).
  Grouped accesses: 2 reported byte store records with printed base expression `[sp-20]` and resolved offset `+0x19`, including target `[sp-20] +0x19`, value `([[[sp-20]+12]-1]:1 & 0xff)` (`strb` at 0x1014d0), target `[sp-20] +0x19`, value `([[sp-20]+24]:1 & 0xff)` (`strb` at 0x101528).
  Grouped accesses: 2 reported byte store records with printed base expression `[sp-20]` and resolved offset `+0x18`, including target `[sp-20] +0x18`, value `([[[sp-20]+12]-1]:1 & 0xff)` (`strb` at 0x1014f0), target `[sp-20] +0x18`, value `([[[sp-20]+12]-1]:1 & 0xff)` (`strb` at 0x101548).
  Grouped accesses: 5 reported word store records with printed base expression `[sp-20]` and resolved offset `+0x10`, including target `[sp-20] +0x10`, value `[[sp-20]+16]-2` (`str` at 0x101504), target `[sp-20] +0x10`, value `[[sp-20]+16]-1` (`str` at 0x10155c), target `[sp-20] +0x10`, value `0x0` (`str` at 0x1015e0).
  Grouped accesses: 3 reported byte load records with printed base expression `[[sp-20]+12]` and resolved offset `-0x1`, including target `[[sp-20]+12] -0x1` (`ldrb` at 0x1014cc), target `[[sp-20]+12] -0x1` (`ldrb` at 0x1014ec), target `[[sp-20]+12] -0x1` (`ldrb` at 0x101544).
  Call (`bl` at 0x10157c): (unknown callee), target 0x10008c, r0=`0xffff`, r1=`[[sp-20]+8]`, r2=`[[sp-20]+16]`, r3=`[sp-20]+24`.
  Call (`bl` at 0x1017d8): (unknown callee), target 0x1001fc, r0=`[sp-20]`, r1=`[sp-28]`, r2=`[[sp-44]+8]`, r3=`[[sp-48]+16]`.
  Branch (`beq` at 0x101324): taken to 0x101364 when `[r0+48] == 0x0`, falls through to 0x101328 when `[r0+48] != 0x0`.
  Branch (`beq` at 0x10142c): taken to 0x1015cc when `[[sp-20]+44] == 0x0`, falls through to 0x101430 when `[[sp-20]+44] != 0x0`.
  Branch (`bne` at 0x10143c): taken to 0x10144c when `[[sp-20]+16] != 0x0`, falls through to 0x101440 when `[[sp-20]+16] == 0x0`.
  Branch (`beq` at 0x101458): taken to 0x101468 when `[[sp-20]+36] == 0x0`, falls through to 0x10145c when `[[sp-20]+36] != 0x0`.
  Branch (`beq` at 0x101474): taken to 0x101494 when `[[sp-20]+16] == 0x0`, falls through to 0x101478 when `[[sp-20]+16] != 0x0`.
  Branch (`bgt` at 0x101484): taken to 0x101494 when `[[sp-20]+16] > 0x2 (signed, 32-bit)`, falls through to 0x101488 when `[[sp-20]+16] <= 0x2 (signed, 32-bit)`.
  Branch (`beq` at 0x1014a0): taken to 0x101768 when `[[sp-20]+16] == 0x0`, falls through to 0x1014a4 when `[[sp-20]+16] != 0x0`.
  Branch (`bne` at 0x1014b0): taken to 0x10150c when `[[sp-20]+32] != 0x0`, falls through to 0x1014b4 when `[[sp-20]+32] == 0x0`.
  Branch (`bne` at 0x101518): taken to 0x101560 when `[[sp-20]+32] != 0x1`, falls through to 0x10151c when `[[sp-20]+32] == 0x1`.
  Branch (`bne` at 0x101594): taken to 0x1015c0 when `unknown != 0xf0b8`, falls through to 0x101598 when `unknown == 0xf0b8`.
  Recorded r0 = `unknown` (`ldm` at 0x101834).

## `IRArg`

- `IRArg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfde7c to 0xfe228.
  Established facts:
  Byte load: target `r0 +0x20` (`ldrb` at 0xfde98).
  Grouped accesses: 2 reported byte load records with printed base expression `[sp-24]` and resolved offset `+0x20`, including target `[sp-24] +0x20` (`ldrb` at 0xfe00c), target `[sp-24] +0x20` (`ldrb` at 0xfe044).
  Grouped accesses: 8 reported word load records with printed base expression `[sp-24]` and resolved offset `+0x8`, including target `[sp-24] +0x8` (`ldr` at 0xfdff8), target `[sp-24] +0x8` (`ldr` at 0xfe024), target `[sp-24] +0x8` (`ldr` at 0xfe04c).
  Halfword load: target `[sp-32] +0x10` (`ldrh` at 0xfe13c).
  Word load: target `[sp-32] +0x0` (`ldr` at 0xfe198).
  Grouped accesses: 2 reported halfword load records with printed base expression `[[sp-32]]` and resolved offset `+0x10`, including target `[[sp-32]] +0x10` (`ldrh` at 0xfe1b0), target `[[sp-32]] +0x10` (`ldrh` at 0xfe1c8).
  Call (`bl` at 0xfe018): (unknown callee), target 0x201fc, r0=`unknown`, r1=`0x28`, r2=`[sp-52]`, r3=`unknown`.
  Call (`bl` at 0xfe058): (unknown callee), target 0x20550, r0=`[[sp-24]+8]`, r1=`0x0`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xfe0d0): (unknown callee), target 0x201fc, r0=`unknown`, r1=`0x24`, r2=`[sp-24]`, r3=`unknown`.
  Call (`bl` at 0xfe164): (unknown callee), target 0x203f4, r0=`[sp-36]`, r1=`[sp-40]`, r2=`[sp-44]`, r3=`unknown`.
  Branch (`bgt` at 0xfdea4): taken to 0xfdeb4 when `[r0+32]:1 > r1 (signed, 32-bit)`, falls through to 0xfdea8 when `[r0+32]:1 <= r1 (signed, 32-bit)`.
  Branch (`beq` at 0xfdf44): taken to 0xfdf58 when `[sp-48]-1 == 0x0`, falls through to 0xfdf48 when `[sp-48]-1 != 0x0`.
  Branch (`bhi` at 0xfdfe4): taken to 0xfdff4 when `[sp-52] > [sp-44] (unsigned, 32-bit)`, falls through to 0xfdfe8 when `[sp-52] <= [sp-44] (unsigned, 32-bit)`.
  Branch (`bne` at 0xfe000): taken to 0xfe05c when `[[sp-24]+8] != 0x0`, falls through to 0xfe004 when `[[sp-24]+8] == 0x0`.
  Branch (`bne` at 0xfe02c): taken to 0xfe03c when `[[sp-24]+8] != 0x0`, falls through to 0xfe030 when `[[sp-24]+8] == 0x0`.
  Branch (`beq` at 0xfe190): taken to 0xfe1f0 when `[sp-52] == 0x0`, falls through to 0xfe194 when `[sp-52] != 0x0`.
  Branch (`blo` at 0xfe1c0): taken to 0xfe1e0 when `[sp-52] < [[[sp-32]]+16]:2-20 (unsigned, 32-bit)`, falls through to 0xfe1c4 when `[sp-52] >= [[[sp-32]]+16]:2-20 (unsigned, 32-bit)`.
  Branch (`beq` at 0xfe1f8): taken to 0xfe200 when `[sp-52] == 0x0`, falls through to 0xfe1fc when `[sp-52] != 0x0`.
  Recorded r0 = `unknown` (`ldm` at 0xfe224).

## `IRFmtClas`

- `IRFmtClas`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfe228 to 0xfe2c4.
  Established facts:
  The instructions from 0xfe2b4 to 0xfe2c4 are not reached by any path from the entry, and their branches are not listed.
  Branch (`beq` at 0xfe24c): taken to 0xfe288 when `r0 == 0x1`, falls through to 0xfe250 when `r0 != 0x1`.
  Branch (`bgt` at 0xfe258): taken to 0xfe26c when `r0 > 0x1 (signed, 32-bit)`, falls through to 0xfe25c when `r0 <= 0x1 (signed, 32-bit)`.
  Branch (`beq` at 0xfe264): taken to 0xfe27c when `r0 == 0x0`, falls through to 0xfe268 when `r0 != 0x0`.
  Branch (`beq` at 0xfe274): taken to 0xfe294 when `r0 == 0x2`, falls through to 0xfe278 when `r0 != 0x2`.
  Recorded r0 = `unknown` (`ldm` at 0xfe2b0).

## `IRFmtAddr`

- `IRFmtAddr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfe2c4 to 0xfe59c.
  Established facts:
  The instructions from 0xfe3e0 to 0xfe4f8 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0xfe570 to 0xfe59c are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0xfe504): (unknown callee), target 0x205bc, r0=`[sp-24]`, r1=`0x1882c4`, r2=`[sp-32]`, r3=`[sp-32]`.
  Call (`bl` at 0xfe50c): (unknown callee), target 0x2031c, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xfe540): (unknown callee), target 0x205bc, r0=`([sp-24] + ret(0x2031c))`, r1=`0x1882cc`, r2=`[sp-36]`, r3=`[sp-40]`.
  Call (`bl` at 0xfe548): (unknown callee), target 0x2031c, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`bge` at 0xfe2ec): taken to 0xfe318 when `r0 >= 0x0 (signed, 32-bit)`, falls through to 0xfe2f0 when `r0 < 0x0 (signed, 32-bit)`.
  Branch (`bne` at 0xfe2fc): taken to 0xfe30c when `unknown != 0x2`, falls through to 0xfe300 when `unknown == 0x2`.
  Branch (`blt` at 0xfe364): taken to 0xfe384 when `r0 < 0x0 (signed, 32-bit)`, falls through to 0xfe368 when `r0 >= 0x0 (signed, 32-bit)`.
  Branch (`bge` at 0xfe39c): taken to 0xfe3bc when `[sp-20] >= 0x0 (signed, 32-bit)`, falls through to 0xfe3a0 when `[sp-20] < 0x0 (signed, 32-bit)`.
  Branch (`ldrls` at 0xfe3d8): taken to (unknown target) when `[sp-32] <= 0xf (unsigned, 32-bit)`, falls through to 0xfe3dc when `[sp-32] > 0xf (unsigned, 32-bit)`.
  Omitted-path effects remain unknown at the unresolved transfer (`ldrls` at 0xfe3d8).
  Known-edge recorded r0 = `[sp-28]` (`ldm` at 0xfe56c).

## `IRFmtPkt`

- `IRFmtPkt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfe59c to 0xfe66c.
  Established facts:
  The instructions from 0xfe668 to 0xfe66c are not reached by any path from the entry, and their branches are not listed.
  Word load: target `r1 +0x18` (`ldr` at 0xfe5c0).
  Word load: target `[sp-24] +0x1c` (`ldr` at 0xfe5d4).
  Byte load: target `[sp-24] +0x16` (`ldrb` at 0xfe5e4).
  Byte load: target `[sp-24] +0x14` (`ldrb` at 0xfe5f8).
  Byte load: target `[sp-24] +0x21` (`ldrb` at 0xfe600).
  Byte load: target `[sp-24] +0x20` (`ldrb` at 0xfe614).
  Byte load: target `[sp-24] +0x13` (`ldrb` at 0xfe620).
  Halfword load: target `[sp-24] +0x22` (`ldrh` at 0xfe62c).
  Call (`bl` at 0xfe5c8): (unknown callee), target 0xfe2c4, r0=`[r1+24]`, r1=`sp-60`, r2=`sp-60`, r3=`r1`.
  Call (`bl` at 0xfe5dc): (unknown callee), target 0xfe2c4, r0=`[[sp-24]+28]`, r1=`sp-92`, r2=`sp-92`, r3=`[sp-24]`.
  Call (`bl` at 0xfe5ec): (unknown callee), target 0xfe228, r0=`[[sp-24]+22]:1`, r1=`unknown`, r2=`unknown`, r3=`[[sp-24]+22]:1`.
  Call (`bl` at 0xfe654): (unknown callee), target 0x205bc, r0=`[sp-28]`, r1=`0x1882e0`, r2=`[sp-20]`, r3=`unknown`.
  Recorded r0 = `[sp-28]` (`ldm` at 0xfe664).

## `IRLogFrame`

- `IRLogFrame`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfe66c to 0xfe780.
  Established facts:
  The instructions from 0xfe774 to 0xfe780 are not reached by any path from the entry, and their branches are not listed.
  Word load: target `r1 +0x0` (`ldr` at 0xfe6ac).
  Halfword load: target `r1 +0x10` (`ldrh` at 0xfe6b8).
  Byte load: target `r1 +0x12` (`ldrb` at 0xfe6c4).
  Byte load: target `r1 +0x13` (`ldrb` at 0xfe6d0).
  Byte load: target `r1 +0x14` (`ldrb` at 0xfe6dc).
  Grouped accesses: 2 reported byte load records with printed base expression `r1` and resolved offset `+0x15`, including target `r1 +0x15` (`ldrsb` at 0xfe6e8), target `r1 +0x15` (`ldrb` at 0xfe6f8).
  Byte load: target `r1 +0x16` (`ldrb` at 0xfe708).
  Word load: target `r1 +0x18` (`ldr` at 0xfe714).
  Word load: target `r1 +0x1c` (`ldr` at 0xfe720).
  Byte load: target `r1 +0x20` (`ldrb` at 0xfe72c).
  Byte load: target `r1 +0x21` (`ldrb` at 0xfe738).
  Halfword load: target `r1 +0x22` (`ldrh` at 0xfe744).
  Call (`bl` at 0xfe694): (unknown callee), target 0x293f8, r0=`0x188308`, r1=`r1`, r2=`r2`, r3=`r1`.
  Call (`bl` at 0xfe69c): (unknown callee), target 0x293f8, r0=`0x188358`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xfe75c): (unknown callee), target 0x205bc, r0=`sp-152`, r1=`0x1883a8`, r2=`r0`, r3=`r1`.
  Call (`bl` at 0xfe768): (unknown callee), target 0x293f8, r0=`sp-152`, r1=`unknown`, r2=`unknown`, r3=`sp-152`.
  Branch (`bne` at 0xfe68c): taken to 0xfe6a4 when `r1 != 0x0`, falls through to 0xfe690 when `r1 == 0x0`.
  Recorded r0 = `ret(0x293f8)` (`ldm` at 0xfe770, `bl` at 0xfe69c, `bl` at 0xfe768).

## `IRPrintFrame`

- `IRPrintFrame`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfe780 to 0xfe89c.
  Established facts:
  The instructions from 0xfe88c to 0xfe89c are not reached by any path from the entry, and their branches are not listed.
  Grouped accesses: 3 reported word load records with absolute target `0x1a288c`, including target `abs 0x1a288c` (`ldr` at 0xfe7a8), target `abs 0x1a288c` (`ldr` at 0xfe7b8), target `abs 0x1a288c` (`ldr` at 0xfe870).
  Word load: target `r1 +0x0` (`ldr` at 0xfe7d0).
  Halfword load: target `r1 +0x10` (`ldrh` at 0xfe7dc).
  Byte load: target `r1 +0x12` (`ldrb` at 0xfe7e8).
  Byte load: target `r1 +0x13` (`ldrb` at 0xfe7f4).
  Byte load: target `r1 +0x14` (`ldrb` at 0xfe800).
  Grouped accesses: 2 reported byte load records with printed base expression `r1` and resolved offset `+0x15`, including target `r1 +0x15` (`ldrsb` at 0xfe80c), target `r1 +0x15` (`ldrb` at 0xfe81c).
  Byte load: target `r1 +0x16` (`ldrb` at 0xfe82c).
  Word load: target `r1 +0x18` (`ldr` at 0xfe838).
  Word load: target `r1 +0x1c` (`ldr` at 0xfe844).
  Byte load: target `r1 +0x20` (`ldrb` at 0xfe850).
  Byte load: target `r1 +0x21` (`ldrb` at 0xfe85c).
  Halfword load: target `r1 +0x22` (`ldrh` at 0xfe868).
  Call (`bl` at 0xfe7b0): (unknown callee), target 0x244d4, r0=`[0x1a288c]`, r1=`0x1883f8`, r2=`r2`, r3=`0x1a288c`.
  Call (`bl` at 0xfe7c0): (unknown callee), target 0x244d4, r0=`[0x1a288c]`, r1=`0x188448`, r2=`unknown`, r3=`0x1a288c`.
  Call (`bl` at 0xfe880): (unknown callee), target 0x244d4, r0=`[0x1a288c]`, r1=`0x188498`, r2=`r0`, r3=`r1`.
  Branch (`bne` at 0xfe7a0): taken to 0xfe7c8 when `r1 != 0x0`, falls through to 0xfe7a4 when `r1 == 0x0`.
  Recorded r0 = `ret(0x244d4)` (`ldm` at 0xfe888, `bl` at 0xfe7c0, `bl` at 0xfe880).

## `IRFmtError`

- `IRFmtError`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfe89c to 0xfe8f8.
  Established facts:
  The instructions from 0xfe8f4 to 0xfe8f8 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0xfe8cc): (unknown callee), target 0x219a4, r0=`unknown`, r1=`r1`, r2=`unknown`, r3=`r0`.
  Branch (`beq` at 0xfe8b8): taken to 0xfe8dc when `r0 == 0x0`, falls through to 0xfe8bc when `r0 != 0x0`.
  Recorded r0 = `unknown` (`ldm` at 0xfe8ec).

## `IRPktSize`

- `IRPktSize`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfe8f8 to 0xfe954.
  Established facts:
  Halfword load: target `(unknown effective address)` (`ldrh` at 0xfe924).
  Word load: target `(unknown effective address)` (`ldr` at 0xfe938).
  Branch (`beq` at 0xfe91c): taken to 0xfe944 when `Z == 1`, falls through to 0xfe920 when `not (Z == 1)`.
  Recorded r0 = `unknown` (`ldm` at 0xfe950).

## `SocketRead`

- `SocketRead`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xffc7c to 0xffd68.
  Established facts:
  Word store: target `ret(0x20184) +0x0`, value `0x0` (`str` at 0xffcbc, `bl` at 0xffcac).
  Word load: target `ret(0x20184) +0x0` (`ldr` at 0xffce0, `bl` at 0xffcd8).
  Call (`bl` at 0xffcac): (unknown callee), target 0x20184, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xffccc): (unknown callee), target 0x20628, r0=`[sp-32]`, r1=`[sp-36]`, r2=`[sp-20]`, r3=`0x0`.
  Call (`bl` at 0xffcd8): (unknown callee), target 0x20184, r0=`ret(0x20628)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x20628)`.
  Branch (`ble` at 0xffca8): taken to 0xffd4c when `le flags condition (operands unknown)`, falls through to 0xffcac when `not (le flags condition (operands unknown))`.
  Branch (`bne` at 0xffce8): taken to 0xffd00 when `[ret(0x20184)] != 0xb`, falls through to 0xffcec when `[ret(0x20184)] == 0xb` (source `bl` at 0xffcd8).
  Branch (`bge` at 0xffd08): taken to 0xffd18 when `[sp-24] >= 0x0 (signed, 32-bit)`, falls through to 0xffd0c when `[sp-24] < 0x0 (signed, 32-bit)`.
  Branch (`bne` at 0xffd20): taken to 0xffd28 when `[sp-24] != 0x0`, falls through to 0xffd24 when `[sp-24] == 0x0`.
  Recorded r0 = `[sp-28]` (`ldm` at 0xffd64).
