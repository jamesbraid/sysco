# ARM Linux 2.6 target and state routines in 1.44.0

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
No unresolved computed transfers were reported for these routines.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-jchip-target-state-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register
state from the analysis rather than a C return type.


Groups summarize static access records by printed shape. Instruction-address order can include different control-flow paths.

## `JChipInit`

- `JChipInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13ba30 to 0x13bf64.
  Established facts:
  Decoded words in [0x13bf28, 0x13bf34), [0x13bf38, 0x13bf3c), [0x13bf40, 0x13bf64) are not reached from the entry by known edges.
  Grouped accesses: 2 reported word load records with absolute target `0x3151c0`, including target `abs 0x3151c0` (`ldr` at 0x13ba4c), target `abs 0x3151c0` (`ldr` at 0x13ba88).
  Branch (`bne` at 0x13ba54): taken to 0x13ba74 when `[0x3151c0] != 0x0`, falls through to 0x13ba58 when `[0x3151c0] == 0x0`.
  Call (`bl` at 0x13ba70): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x191b00`, r2=`0x190`, r3=`0x0`.
  Call (`bl` at 0x13baf8): (unknown callee), target 0x203b8, r0=`[[sp-36]+8]`, r1=`unknown`, r2=`[[sp-36]+12]`, r3=`unknown`.
  Grouped accesses: 5 reported byte store records with printed base expression `[sp-40]` and nonnegative byte-aligned offsets, including target `[sp-40] +0x18`, value `unknown` (`strb` at 0x13bb78), target `[sp-40] +0x19`, value `unknown` (`strb` at 0x13bb98), target `[sp-40] +0x20`, value `0x0` (`strb` at 0x13bbb0).
  Call (`bl` at 0x13bbd4): (unknown callee), target 0x13df54, r0=`unknown`, r1=`unknown`, r2=`[sp-40]`, r3=`unknown`.
  Call (`bl` at 0x13bc10): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x191b00`, r2=`0x1a7`, r3=`0x0`.
  Call (`bl` at 0x13bc90): (unknown callee), target 0x205bc, r0=`[sp-40]+4`, r1=`0x191b44`, r2=`unknown`, r3=`[[[sp-40]]+8]`.
  Call (`bl` at 0x13bcbc): (unknown callee), target 0x205bc, r0=`[sp-40]+4`, r1=`0x191b4c`, r2=`[[[sp-40]]+8]`, r3=`[[sp-40]+24]:1`.
  Call (`mov` at 0x13bd74): (unknown callee), target (unknown target), r0=`[sp-40]`, r1=`unknown`, r2=`0x3`, r3=`[[[sp-40]]+20]`.
  Call (`bl` at 0x13bdac): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x191b00`, r2=`0x1b8`, r3=`0x0`.
  Call (`bl` at 0x13be84): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x191b00`, r2=`0x1c9`, r3=`0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x13bf24).

## `JChipRemoveAll`

- `JChipRemoveAll`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13bf64 to 0x13c114.
  Established facts:
  Decoded words in [0x13c100, 0x13c104), [0x13c108, 0x13c114) are not reached from the entry by known edges.
  Word load: target `abs 0x3151c0` (`ldr` at 0x13bf80).
  Branch (`blt` at 0x13bf88): taken to 0x13bf98 when `r0 < [0x3151c0] (signed, 32-bit)`, falls through to 0x13bf8c when `r0 >= [0x3151c0] (signed, 32-bit)`.
  Call (`bl` at 0x13bf98): (unknown callee), target 0x134634, r0=`r0`, r1=`r1`, r2=`r0`, r3=`[0x3151c0]`.
  Call (`bl` at 0x13c0b8): (unknown callee), target 0x143fec, r0=`[[sp-36]+40]`, r1=`[[sp-36]+36]`, r2=`[sp-36]`, r3=`[sp-36]`.
  Call (`bl` at 0x13c0d4): (unknown callee), target 0x20514, r0=`[[sp-32]+8]`, r1=`unknown`, r2=`[[sp-32]+12]`, r3=`[sp-32]`.
  Grouped accesses: 2 reported word store records with printed base expression `[sp-32]` and nonnegative word-aligned offsets, including target `[sp-32] +0x8`, value `0x0` (`str` at 0x13c0e0), target `[sp-32] +0xc`, value `0x0` (`str` at 0x13c0ec).
  Call (`bl` at 0x13c0f0): (unknown callee), target 0x1346fc, r0=`ret(0x20514)`, r1=`unknown`, r2=`[sp-32]`, r3=`0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x13c0fc).

## `JChipTargetByName`

- `JChipTargetByName`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13c568 to 0x13ccc0.
  Established facts:
  Decoded words in [0x13cc8c, 0x13ccc0) are not reached from the entry by known edges.
  Call (`bl` at 0x13c580): (unknown callee), target 0x20058, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Grouped accesses: 2 reported byte load records with absolute target `0x191c54`, including target `abs 0x191c54` (`ldrb` at 0x13c590), target `abs 0x191c54` (`ldrb` at 0x13c5b0).
  Call (`bl` at 0x13c5a8): (unknown callee), target 0x20550, r0=`sp-87`, r1=`0x0`, r2=`0x13`, r3=`sp-87`.
  Call (`bl` at 0x13c5c8): (unknown callee), target 0x20550, r0=`sp-107`, r1=`0x0`, r2=`0x13`, r3=`sp-107`.
  Call (`bl` at 0x13c60c): (unknown callee), target 0x203ac, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`bne` at 0x13c624): taken to 0x13c634 when `[[sp-32]]:1 != 0x2e`, falls through to 0x13c628 when `[[sp-32]]:1 == 0x2e`.
  Byte store: target `[sp-32] +0x0`, value `0x3a` (`strb` at 0x13c630).
  Call (`bl` at 0x13c668): (unknown callee), target 0x204fc, r0=`[sp-28]`, r1=`0x191c68`, r2=`sp-88`, r3=`sp-68`.
  Call (`bl` at 0x13c6ac): (unknown callee), target 0x204fc, r0=`[sp-28]`, r1=`0x191c7c`, r2=`sp-108`, r3=`sp-112`.
  Call (`bl` at 0x13c724): (unknown callee), target 0x200ac, r0=`unknown`, r1=`sp-108`, r2=`sp-108`, r3=`unknown`.
  Call (`bl` at 0x13c7f8): (unknown callee), target 0x22070, r0=`0x1`, r1=`0x191b00`, r2=`0x321`, r3=`0x0`.
  Call (`bl` at 0x13c934): (unknown callee), target 0x200ac, r0=`unknown`, r1=`sp-88`, r2=`sp-88`, r3=`unknown`.
  Call (`bl` at 0x13c958): (unknown callee), target 0x200ac, r0=`unknown`, r1=`sp-88`, r2=`sp-88`, r3=`unknown`.
  Call (`bl` at 0x13cb88): (unknown callee), target 0x205bc, r0=`sp-152`, r1=`0x191b44`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x13cbb0): (unknown callee), target 0x205bc, r0=`sp-152`, r1=`0x191b4c`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x13cbc0): (unknown callee), target 0x200ac, r0=`sp-152`, r1=`[sp-28]`, r2=`unknown`, r3=`sp-152`.
  Call (`bl` at 0x13cc00): (unknown callee), target 0x20514, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x13cc74): (unknown callee), target 0x20514, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `[sp-156]` (`ldm` at 0x13cc88).

## `JChipName`

- `JChipName`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13ccc0 to 0x13cdc8.
  Established facts:
  Decoded words in [0x13cdb8, 0x13cdc8) are not reached from the entry by known edges.
  Call (`bl` at 0x13ccd8): (unknown callee), target 0x13dfe0, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0x13cd0c): (unknown callee), target 0x13df54, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x13cd78): (unknown callee), target 0x205bc, r0=`0x315068`, r1=`0x191d6c`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x13cdb4).

## `JChipPortName`

- `JChipPortName`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13cdc8 to 0x13ce4c.
  Established facts:
  Decoded words in [0x13ce3c, 0x13ce4c) are not reached from the entry by known edges.
  Branch (`bne` at 0x13cde4): taken to 0x13cdf4 when `r0 != 0xff`, falls through to 0x13cde8 when `r0 == 0xff`.
  Word load: target `abs 0x3151c0` (`ldr` at 0x13cdfc).
  Branch (`blt` at 0x13ce04): taken to 0x13ce14 when `r0 < [0x3151c0] (signed, 32-bit)`, falls through to 0x13ce08 when `r0 >= [0x3151c0] (signed, 32-bit)`.
  Recorded r0 = `unknown` (`ldm` at 0x13ce38).

## `JChipNumPorts`

- `JChipNumPorts`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13ce4c to 0x13ce6c.
  Established facts:
  Decoded words in [0x13ce68, 0x13ce6c) are not reached from the entry by known edges.
  Word load: target `abs 0x3151c0` (`ldr` at 0x13ce5c).
  Recorded r0 = `[0x3151c0]` (`ldm` at 0x13ce64).

## `JChipGetChipList`

- `JChipGetChipList`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13ce6c to 0x13cffc.
  Established facts:
  Decoded words in [0x13cff0, 0x13cffc) are not reached from the entry by known edges.
  Word load: target `abs 0x3151c0` (`ldr` at 0x13ceac).
  Word load: target `abs 0x3151bc` (`ldr` at 0x13cec4).
  Call (`bl` at 0x13cf24): (unknown callee), target 0x2031c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x13cf58): (unknown callee), target 0x203b8, r0=`[sp-36]`, r1=`[sp-32]+50`, r2=`[sp-32]`, r3=`[sp-32]+50`.
  Call (`bl` at 0x13cf84): (unknown callee), target 0x20634, r0=`[sp-40]`, r1=`[sp-48]+4`, r2=`unknown`, r3=`[sp-48]+4`.
  Call (`bl` at 0x13cf90): (unknown callee), target 0x203c4, r0=`[sp-40]`, r1=`0x191d80`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x13cf98): (unknown callee), target 0x2031c, r0=`[sp-40]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x13cfec).

## `JChipGetType`

- `JChipGetType`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13df54 to 0x13dfe0.
  Established facts:
  Decoded words in [0x13dfd8, 0x13dfe0) are not reached from the entry by known edges.
  Word load: target `abs 0x1a088c` (`ldr` at 0x13df78).
  Word load: target `(unknown effective address)` (`ldr` at 0x13df8c).
  Recorded r0 = `unknown` (`ldm` at 0x13dfd4).

## `JChipGetChip`

- `JChipGetChip`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13dfe0 to 0x13e280.
  Established facts:
  Decoded words in [0x13e278, 0x13e280) are not reached from the entry by known edges.
  Grouped accesses: 3 reported word load records with absolute target `0x3151c0`, including target `abs 0x3151c0` (`ldr` at 0x13e050), target `abs 0x3151c0` (`ldr` at 0x13e08c), target `abs 0x3151c0` (`ldr` at 0x13e0f4).
  Branch (`blt` at 0x13e0fc): taken to 0x13e10c when `[sp-37]:1 < [0x3151c0] (signed, 32-bit)`, falls through to 0x13e100 when `[sp-37]:1 >= [0x3151c0] (signed, 32-bit)`.
  Call (`bl` at 0x13e1cc): (unknown callee), target 0x13c114, r0=`[sp-44]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Byte load: target `(unknown effective address)` (`ldrb` at 0x13e204).
  Recorded r0 = `unknown` (`ldm` at 0x13e274).

## `JChip_TDR_Raw`

- `JChip_TDR_Raw`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13d9cc to 0x13df54.
  Established facts:
  Decoded words in [0x13defc, 0x13df04), [0x13df0c, 0x13df2c), [0x13df30, 0x13df54) are not reached from the entry by known edges.
  Call (`bl` at 0x13d9f0): (unknown callee), target 0x13dfe0, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0x13da30): (unknown callee), target 0x13b298, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13dfe0)`.
  Grouped accesses: 5 reported byte load records with printed base expression `[sp-40]` and nonnegative byte-aligned offsets, including target `[sp-40] +0x21` (`ldrsb` at 0x13daa4), target `[sp-40] +0x19` (`ldrb` at 0x13dab4), target `[sp-40] +0x19` (`ldrb` at 0x13daf4).
  Call (`bl` at 0x13dc7c): (unknown callee), target 0x13e4f0, r0=`[sp-56]`, r1=`[sp-48]`, r2=`[sp-52]`, r3=`sp-24`.
  Call (`bl` at 0x13dc8c): (unknown callee), target 0x12cc8c, r0=`0x1`, r1=`[sp-44]`, r2=`0x31e818`, r3=`unknown`.
  Call (`bl` at 0x13dcc0): (unknown callee), target 0x12d2b8, r0=`0x0`, r1=`0x0`, r2=`0x1`, r3=`[sp-44]`.
  Call (`bl` at 0x13dce8): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x191b00`, r2=`0x4c3`, r3=`0x0`.
  Branch (`bls` at 0x13dd04): taken to 0x13dd24 when `[0x315034] <= 0x2 (unsigned, 32-bit)`, falls through to 0x13dd08 when `[0x315034] > 0x2 (unsigned, 32-bit)`.
  Call (`bl` at 0x13dd20): (unknown callee), target 0x13e868, r0=`[sp+4]`, r1=`0x191e98`, r2=`[sp-56]`, r3=`0x3150a0`.
  Call (`bl` at 0x13ddac): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x191b00`, r2=`0x4ec`, r3=`0x0`.
  Call (`bl` at 0x13dde8): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x191b00`, r2=`0x4f4`, r3=`0x0`.
  Call (`bl` at 0x13de18): (unknown callee), target 0x13e778, r0=`[sp-64]`, r1=`[sp-60]`, r2=`[sp]`, r3=`[sp-28]`.
  Call (`bl` at 0x13de4c): (unknown callee), target 0x12d2b8, r0=`0x1`, r1=`0x1`, r2=`0x1`, r3=`[sp-44]`.
  Call (`bl` at 0x13de74): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x191b00`, r2=`0x4fd`, r3=`0x0`.
  Call (`bl` at 0x13deac): (unknown callee), target 0x13e868, r0=`[sp+4]`, r1=`0x191f74`, r2=`[sp-36]`, r3=`[sp-68]`.
  Call (`bl` at 0x13dee4): (unknown callee), target 0x132444, r0=`[sp-60]`, r1=`0x315120`, r2=`0x0`, r3=`[sp-32]`.
  Recorded r0 = `unknown` (`ldm` at 0x13def8).

## `JChip_MMR_SC_stub`

- `JChip_MMR_SC_stub`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x139d6c to 0x139d8c.
  Established facts:
  Recorded r0 = `0x0` (`ldm` at 0x139d88).

## `JChip_link_summary_stub`

- `JChip_link_summary_stub`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x139dac to 0x139e00.
  Established facts:
  Decoded words in [0x139df8, 0x139e00) are not reached from the entry by known edges.
  Call (`bl` at 0x139dd0): (unknown callee), target 0x293f8, r0=`0x191420`, r1=`r1`, r2=`r2`, r3=`r3`.
  Byte store: target `[sp-32] +0x0`, value `0x0` (`strb` at 0x139ddc).
  Call (`bl` at 0x139de4): (unknown callee), target 0x293f8, r0=`0x191424`, r1=`unknown`, r2=`[sp-32]`, r3=`0x0`.
  Recorded r0 = `0x0` (`ldm` at 0x139df4).

## `JChip_link_summary_f`

- `JChip_link_summary_f`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x147c00 to 0x1480d8.
  Established facts:
  Decoded words in [0x1480a4, 0x1480a8), [0x1480ac, 0x1480d8) are not reached from the entry by known edges.
  Byte store: target `r3 +0x0`, value `0x0` (`strb` at 0x147c50).
  Branch (`bne` at 0x147c5c): taken to 0x147c6c when `[sp-20] != 0x0`, falls through to 0x147c60 when `[sp-20] == 0x0`.
  Call (`bl` at 0x147c70): (unknown callee), target 0x13dfe0, r0=`[sp-20]`, r1=`r1`, r2=`r3`, r3=`[sp-20]`.
  Call (`bl` at 0x147ca8): (unknown callee), target 0x134634, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x147cf4): (unknown callee), target 0x147860, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0x0`, r3=`unknown`.
  Call (`bl` at 0x147d14): (unknown callee), target 0x205bc, r0=`[sp-32]`, r1=`0x19518c`, r2=`unknown`, r3=`ret(0x147860)`.
  Call (`bl` at 0x147d18): (unknown callee), target 0x1346fc, r0=`ret(0x205bc)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x147d28): (unknown callee), target 0x1346fc, r0=`ret(0x147860)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x147860)`.
  Byte load: target `[[sp-72]] +0x10` (`ldrb` at 0x147d70).
  Call (`bl` at 0x147dcc): (unknown callee), target 0x205bc, r0=`[sp-96]`, r1=`0x1951b0`, r2=`[sp-68]`, r3=`[sp-64]`.
  Call (`bl` at 0x147dd4): (unknown callee), target 0x2031c, r0=`[sp-96]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x147e28): (unknown callee), target 0x205bc, r0=`[sp-96]`, r1=`0x1951dc`, r2=`unknown`, r3=`[sp-28]`.
  Call (`bl` at 0x147e30): (unknown callee), target 0x2031c, r0=`[sp-96]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x147fac): (unknown callee), target 0x205bc, r0=`[sp-96]`, r1=`[sp-116]`, r2=`[sp-100]`, r3=`unknown`.
  Call (`bl` at 0x147fb4): (unknown callee), target 0x2031c, r0=`[sp-96]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x148004): (unknown callee), target 0x205bc, r0=`([sp-96] + ret(0x2031c))`, r1=`[sp-120]`, r2=`[sp-56]`, r3=`[sp-52]`.
  Call (`bl` at 0x14800c): (unknown callee), target 0x2031c, r0=`[sp-96]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x14802c): (unknown callee), target 0x203c4, r0=`([sp-96] + ret(0x2031c))`, r1=`[sp-108]`, r2=`ret(0x2031c)`, r3=`([sp-96] + ret(0x2031c))`.
  Call (`bl` at 0x148034): (unknown callee), target 0x2031c, r0=`[sp-96]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x148070): (unknown callee), target 0x205bc, r0=`[sp-96]`, r1=`0x19525c`, r2=`unknown`, r3=`[sp-56]`.
  Call (`bl` at 0x148078): (unknown callee), target 0x2031c, r0=`[sp-96]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x1480a0).

## `JChipSetup`

- `JChipSetup`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13b26c to 0x13b298.
  Established facts:
  Decoded words in [0x13b290, 0x13b298) are not reached from the entry by known edges.
  Word store: target `abs 0x1a087c`, value `0x13d3d8` (`str` at 0x13b280).
  Recorded r0 = `0x0` (`ldm` at 0x13b28c).

## `jchip_dump_io`

- `jchip_dump_io`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13e868 to 0x13e948.
  Established facts:
  Decoded words in [0x13e940, 0x13e948) are not reached from the entry by known edges.
  Call (`bl` at 0x13e89c): (unknown callee), target 0x205bc, r0=`sp-72`, r1=`0x191fe0`, r2=`r0`, r3=`r1`.
  Call (`bl` at 0x13e8dc): (unknown callee), target 0x14bab4, r0=`0x0`, r1=`sp-72`, r2=`unknown`, r3=`[sp-32]`.
  Call (`bl` at 0x13e8f4): (unknown callee), target 0x205bc, r0=`sp-72`, r1=`0x191fec`, r2=`[sp-20]`, r3=`[sp-24]`.
  Call (`bl` at 0x13e934): (unknown callee), target 0x14bab4, r0=`0x0`, r1=`sp-72`, r2=`unknown`, r3=`[sp]`.
  Recorded r0 = `ret(0x14bab4)` (`ldm` at 0x13e93c, `bl` at 0x13e934).
