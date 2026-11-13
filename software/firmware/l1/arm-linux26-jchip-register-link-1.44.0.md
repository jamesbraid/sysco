# ARM Linux 2.6 register and link routines in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-jchip-register-link-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register
state from the analysis rather than a C return type.


Groups summarize static access records by printed shape. Instruction-address order can include different control-flow paths.

## `JChip_MMR_Read`

- `JChip_MMR_Read`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13d1a0 to 0x13d2b8.
  Established facts:
  Decoded words in [0x13d2a8, 0x13d2b4) are not reached from the entry by known edges.
  Call (`bl` at 0x13d1c4): (unknown callee), target 0x13dfe0, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0x13d1e8): (unknown callee), target 0x134634, r0=`ret(0x13dfe0)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13dfe0)`.
  Call (`bl` at 0x13d1f0): (unknown callee), target 0x13b298, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x13d200): (unknown callee), target 0x1346fc, r0=`ret(0x13b298)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13b298)`.
  Word load: target `[sp-36] +0x0` (`ldr` at 0x13d214).
  Grouped accesses: 2 reported byte load records with printed base expression `[sp-36]` and nonnegative byte-aligned offsets, including target `[sp-36] +0x19` (`ldrb` at 0x13d21c), target `[sp-36] +0x21` (`ldrsb` at 0x13d224).
  Call (`mov` at 0x13d268): (unknown callee), target (unknown target), r0=`[[sp-36]+25]:1`, r1=`unknown`, r2=`0x5`, r3=`[sp-24]`.
  Call (`bl` at 0x13d278): (unknown callee), target 0x1346fc, r0=`ret(unknown)`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`bl` at 0x13d288): (unknown callee), target 0x1346fc, r0=`ret(unknown)`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Recorded r0 = `unknown` (`ldm` at 0x13d29c).

## `JChip_MMR_Write`

- `JChip_MMR_Write`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13d2b8 to 0x13d3d8.
  Established facts:
  Decoded words in [0x13d3c4, 0x13d3d4) are not reached from the entry by known edges.
  Call (`bl` at 0x13d2dc): (unknown callee), target 0x13dfe0, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0x13d300): (unknown callee), target 0x134634, r0=`ret(0x13dfe0)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13dfe0)`.
  Call (`bl` at 0x13d308): (unknown callee), target 0x13b298, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x13d318): (unknown callee), target 0x1346fc, r0=`ret(0x13b298)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13b298)`.
  Grouped accesses: 2 reported byte load records with printed base expression `[sp-40]` and nonnegative byte-aligned offsets, including target `[sp-40] +0x19` (`ldrb` at 0x13d334), target `[sp-40] +0x21` (`ldrsb` at 0x13d33c).
  Word load: target `abs 0x1a0888` (`ldr` at 0x13d378).
  Call (`mov` at 0x13d384): (unknown callee), target (unknown target), r0=`[[sp-40]+25]:1`, r1=`unknown`, r2=`[0x1a0888]`, r3=`[sp-28]`.
  Call (`bl` at 0x13d394): (unknown callee), target 0x1346fc, r0=`ret(unknown)`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`bl` at 0x13d3a4): (unknown callee), target 0x1346fc, r0=`ret(unknown)`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Recorded r0 = `unknown` (`ldm` at 0x13d3b8).

## `JChip_MMR_SC_f`

- `JChip_MMR_SC_f`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13d3d8 to 0x13d9cc.
  Established facts:
  Decoded words in [0x13d9b4, 0x13d9cc) are not reached from the entry by known edges.
  Grouped accesses: 5 reported word load records with printed base expression `r0` and nonnegative word-aligned offsets, including target `r0 +0xc` (`ldr` at 0x13d3f8), target `r0 +0xc` (`ldr` at 0x13d418), target `r0 +0x0` (`ldr` at 0x13d42c).
  Branch (`bls` at 0x13d454): taken to 0x13d494 when `[0x315034] <= 0x2 (unsigned, 32-bit)`, falls through to 0x13d458 when `[0x315034] > 0x2 (unsigned, 32-bit)`.
  Call (`bl` at 0x13d490): (unknown callee), target 0x243d4, r0=`0x191d84`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x13d4cc): (unknown callee), target 0x13e5b0, r0=`unknown`, r1=`unknown`, r2=`0x4`, r3=`unknown`.
  Call (`bl` at 0x13d504): (unknown callee), target 0x13ccc0, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`[0x315034]`.
  Call (`bl` at 0x13d524): (unknown callee), target 0x243d4, r0=`0x191da4`, r1=`ret(0x13ccc0)`, r2=`[sp-28]`, r3=`[sp-36]`.
  Call (`bl` at 0x13d5b4): (unknown callee), target 0x14fe64, r0=`[sp-32]`, r1=`[sp-88]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x13d5dc): (unknown callee), target 0x14fe64, r0=`[sp-92]`, r1=`[sp-56]`, r2=`0x2`, r3=`unknown`.
  Call (`bl` at 0x13d650): (unknown callee), target 0x14fe64, r0=`[sp-32]`, r1=`0x2`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x13d768): (unknown callee), target 0x13ccc0, r0=`[sp-28]`, r1=`0x14`, r2=`unknown`, r3=`[0x315034]`.
  Call (`bl` at 0x13d790): (unknown callee), target 0x243d4, r0=`0x191ddc`, r1=`ret(0x13ccc0)`, r2=`[sp-72]`, r3=`[sp-76]`.
  Call (`bl` at 0x13d7b0): (unknown callee), target 0x13d2b8, r0=`[sp-28]`, r1=`([sp-72] + [sp-76])`, r2=`[sp-64]`, r3=`[sp-68]`.
  Call (`bl` at 0x13d880): (unknown callee), target 0x13ccc0, r0=`[sp-28]`, r1=`0x14`, r2=`unknown`, r3=`[0x315034]`.
  Call (`bl` at 0x13d898): (unknown callee), target 0x243d4, r0=`0x191e14`, r1=`ret(0x13ccc0)`, r2=`[sp-76]`, r3=`[sp-72]`.
  Call (`bl` at 0x13d8bc): (unknown callee), target 0x13d1a0, r0=`[sp-28]`, r1=`unknown`, r2=`sp-64`, r3=`sp-68`.
  Call (`bl` at 0x13d904): (unknown callee), target 0x13ccc0, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`[0x315034]`.
  Call (`bl` at 0x13d92c): (unknown callee), target 0x243d4, r0=`0x191e40`, r1=`ret(0x13ccc0)`, r2=`[sp-76]`, r3=`[sp-72]`.
  Word store: target `(unknown effective address)`, value `[sp-64]` (`str` at 0x13d95c).
  Recorded r0 = `unknown` (`ldm` at 0x13d9b0).

## `JChip_read_dmin_dmax`

- `JChip_read_dmin_dmax`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1475b8 to 0x147860.
  Established facts:
  Decoded words in [0x147618, 0x1476d0), [0x1476dc, 0x14783c), [0x147848, 0x147850), [0x147858, 0x147860) are not reached from the entry by known edges.
  Word load: target `abs 0x315034` (`ldr` at 0x1475dc).
  Branch (`bls` at 0x1475e4): taken to 0x1475fc when `[0x315034] <= 0x1 (unsigned, 32-bit)`, falls through to 0x1475e8 when `[0x315034] > 0x1 (unsigned, 32-bit)`.
  Call (`bl` at 0x1475f8): (unknown callee), target 0x243d4, r0=`0x195104`, r1=`r0`, r2=`r1`, r3=`r2`.
  Branch (`ldrls` at 0x147610): taken to (unknown target) when `? <= 0x5 (unsigned, 32-bit)`, falls through to 0x147614 when `? > 0x5 (unsigned, 32-bit)`.
  Omitted-path effects remain unknown at the unresolved transfer (`ldrls` at 0x147610).
  Known-edge recorded r0 = `0xfffff7c0` (`ldm` at 0x147844).

## `JChip_read_dmcout`

- `JChip_read_dmcout`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x147024 to 0x147368.
  Established facts:
  Decoded words in [0x14708c, 0x147144), [0x147150, 0x147348), [0x147354, 0x14735c) are not reached from the entry by known edges.
  Word load: target `abs 0x315034` (`ldr` at 0x147048).
  Branch (`bls` at 0x147050): taken to 0x147064 when `[0x315034] <= 0x1 (unsigned, 32-bit)`, falls through to 0x147054 when `[0x315034] > 0x1 (unsigned, 32-bit)`.
  Call (`bl` at 0x147060): (unknown callee), target 0x243d4, r0=`0x1950d4`, r1=`r0`, r2=`r1`, r3=`[0x315034]`.
  Branch (`ldrls` at 0x147084): taken to (unknown target) when `? <= 0x5 (unsigned, 32-bit)`, falls through to 0x147088 when `? > 0x5 (unsigned, 32-bit)`.
  Omitted-path effects remain unknown at the unresolved transfer (`ldrls` at 0x147084).
  Known-edge recorded r0 = `0xfffff7c0` (`ldm` at 0x147350).

## `JChip_read_link_state`

- `JChip_read_link_state`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x147860 to 0x147c00.
  Established facts:
  Decoded words in [0x1478c0, 0x1479bc), [0x1479c8, 0x147bd8), [0x147be4, 0x147bec), [0x147bf0, 0x147bf4), [0x147bf8, 0x147c00) are not reached from the entry by known edges.
  Word load: target `abs 0x315034` (`ldr` at 0x147884).
  Branch (`bls` at 0x14788c): taken to 0x1478a4 when `[0x315034] <= 0x1 (unsigned, 32-bit)`, falls through to 0x147890 when `[0x315034] > 0x1 (unsigned, 32-bit)`.
  Call (`bl` at 0x1478a0): (unknown callee), target 0x243d4, r0=`0x195104`, r1=`r0`, r2=`r1`, r3=`r2`.
  Branch (`ldrls` at 0x1478b8): taken to (unknown target) when `? <= 0x5 (unsigned, 32-bit)`, falls through to 0x1478bc when `? > 0x5 (unsigned, 32-bit)`.
  Omitted-path effects remain unknown at the unresolved transfer (`ldrls` at 0x1478b8).
  Known-edge recorded r0 = `0xfffff7c0` (`ldm` at 0x147be0).

## `JChip_compare_dmcout`

- `JChip_compare_dmcout`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x147368 to 0x1475b8.
  Established facts:
  Branch (`bne` at 0x1473b4): taken to 0x1473d4 when `[sp-20] != [sp-28]`, falls through to 0x1473b8 when `[sp-20] == [sp-28]`.
  Call (`bl` at 0x1473ec): (unknown callee), target 0x147024, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x147428): (unknown callee), target 0x147024, r0=`[sp-28]`, r1=`[sp-32]`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x147444): (unknown callee), target 0x20514, r0=`[sp-44]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x147024)`.
  Branch (`bls` at 0x147520): taken to 0x147554 when `[sp-56] <= [sp] (unsigned, 32-bit)`, falls through to 0x147524 when `[sp-56] > [sp] (unsigned, 32-bit)`.
  Call (`bl` at 0x147598): (unknown callee), target 0x20514, r0=`[sp-40]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x1475a0): (unknown callee), target 0x20514, r0=`[sp-44]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x1475b4).

## `JChip_ir_load`

- `JChip_ir_load`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13f880 to 0x13fd34.
  Established facts:
  Decoded words in [0x13fce8, 0x13fd08), [0x13fd10, 0x13fd18), [0x13fd1c, 0x13fd34) are not reached from the entry by known edges.
  Call (`bl` at 0x13f8d4): (unknown callee), target 0x243d4, r0=`0x19241c`, r1=`r0`, r2=`r1`, r3=`r2`.
  Call (`bl` at 0x13f8f4): (unknown callee), target 0x134634, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[sp-68]`.
  Call (`bl` at 0x13f8fc): (unknown callee), target 0x13dfe0, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x13f920): (unknown callee), target 0x205bc, r0=`0x31e818`, r1=`0x192454`, r2=`[sp-20]`, r3=`ret(0x13dfe0)`.
  Call (`bl` at 0x13f93c): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x191b00`, r2=`0x764`, r3=`0x0`.
  Call (`bl` at 0x13f94c): (unknown callee), target 0x1346fc, r0=`ret(0x22070)`, r1=`unknown`, r2=`unknown`, r3=`[sp-68]`.
  Call (`bl` at 0x13f960): (unknown callee), target 0x13b298, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13dfe0)`.
  Call (`bl` at 0x13f97c): (unknown callee), target 0x1346fc, r0=`ret(0x13b298)`, r1=`unknown`, r2=`unknown`, r3=`[sp-68]`.
  Grouped accesses: 5 reported byte load records with printed base expression `[sp-48]` and nonnegative byte-aligned offsets, including target `[sp-48] +0x21` (`ldrsb` at 0x13f9d4), target `[sp-48] +0x19` (`ldrb` at 0x13f9e4), target `[sp-48] +0x19` (`ldrb` at 0x13fa24).
  Call (`bl` at 0x13fac4): (unknown callee), target 0x12cdbc, r0=`0x1`, r1=`unknown`, r2=`0x31e818`, r3=`unknown`.
  Call (`bl` at 0x13faec): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x191b00`, r2=`0x788`, r3=`0x0`.
  Call (`bl` at 0x13fafc): (unknown callee), target 0x1346fc, r0=`ret(0x22070)`, r1=`unknown`, r2=`unknown`, r3=`[sp-68]`.
  Call (`bl` at 0x13fb24): (unknown callee), target 0x243d4, r0=`0x192484`, r1=`[sp-64]`, r2=`unknown`, r3=`[0x315034]`.
  Grouped accesses: 2 reported word store records with absolute target `0x31509c`, including target `abs 0x31509c`, value `0x0` (`str` at 0x13fb54), target `abs 0x31509c`, value `[0x31509c]+1` (`str` at 0x13fb98).
  Call (`bl` at 0x13fbbc): (unknown callee), target 0x13262c, r0=`0x0`, r1=`0x20`, r2=`[sp-60]`, r3=`[[sp-52]+4]:2`.
  Call (`bl` at 0x13fbd8): (unknown callee), target 0x132444, r0=`0x0`, r1=`[sp-60]`, r2=`[sp-36]`, r3=`[sp-56]`.
  Call (`bl` at 0x13fc0c): (unknown callee), target 0x12d2b8, r0=`0x0`, r1=`0x1`, r2=`0x1`, r3=`[sp-64]`.
  Call (`bl` at 0x13fc34): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x191b00`, r2=`0x79d`, r3=`0x0`.
  Call (`bl` at 0x13fc44): (unknown callee), target 0x1346fc, r0=`ret(0x22070)`, r1=`unknown`, r2=`unknown`, r3=`[sp-68]`.
  Call (`bl` at 0x13fc98): (unknown callee), target 0x243d4, r0=`0x1924bc`, r1=`[sp-64]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x13fcac): (unknown callee), target 0x14bab4, r0=`0x0`, r1=`0x192518`, r2=`[sp-32]`, r3=`[sp-56]`.
  Call (`bl` at 0x13fcc0): (unknown callee), target 0x14bab4, r0=`0x0`, r1=`0x192524`, r2=`[sp-32]`, r3=`[sp-60]`.
  Call (`bl` at 0x13fcd0): (unknown callee), target 0x1346fc, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[sp-68]`.
  Recorded r0 = `unknown` (`ldm` at 0x13fce4).

## `JChip_tdr_load`

- `JChip_tdr_load`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13fd34 to 0x140744.
  Established facts:
  Decoded words in [0x1406c4, 0x1406e4), [0x1406ec, 0x1406f4), [0x1406f8, 0x140744) are not reached from the entry by known edges.
  Call (`bl` at 0x13fd8c): (unknown callee), target 0x243d4, r0=`0x192530`, r1=`r0`, r2=`r1`, r3=`r2`.
  Call (`bl` at 0x13fdac): (unknown callee), target 0x134634, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[sp-212]`.
  Call (`bl` at 0x13fdb4): (unknown callee), target 0x13dfe0, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x13fdd8): (unknown callee), target 0x205bc, r0=`0x31e818`, r1=`0x192454`, r2=`[sp-20]`, r3=`ret(0x13dfe0)`.
  Call (`bl` at 0x13fdf4): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x191b00`, r2=`0x7ce`, r3=`0x0`.
  Call (`bl` at 0x13fe04): (unknown callee), target 0x1346fc, r0=`ret(0x22070)`, r1=`unknown`, r2=`unknown`, r3=`[sp-212]`.
  Call (`bl` at 0x13fe18): (unknown callee), target 0x13b298, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13dfe0)`.
  Call (`bl` at 0x13fe34): (unknown callee), target 0x1346fc, r0=`ret(0x13b298)`, r1=`unknown`, r2=`unknown`, r3=`[sp-212]`.
  Call (`bl` at 0x13feec): (unknown callee), target 0x12cdbc, r0=`0x1`, r1=`unknown`, r2=`0x31e818`, r3=`unknown`.
  Call (`bl` at 0x13ff14): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x191b00`, r2=`0x7ed`, r3=`0x0`.
  Call (`bl` at 0x13ff24): (unknown callee), target 0x1346fc, r0=`ret(0x22070)`, r1=`unknown`, r2=`unknown`, r3=`[sp-212]`.
  Call (`bl` at 0x13ff4c): (unknown callee), target 0x243d4, r0=`0x192484`, r1=`[sp-208]`, r2=`unknown`, r3=`[0x315034]`.
  Call (`bl` at 0x13ff6c): (unknown callee), target 0x13f880, r0=`[sp-20]`, r1=`0x0`, r2=`[sp-28]`, r3=`unknown`.
  Call (`bl` at 0x13ff9c): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x191b00`, r2=`0x7fa`, r3=`0x0`.
  Call (`bl` at 0x13ffac): (unknown callee), target 0x1346fc, r0=`ret(0x22070)`, r1=`unknown`, r2=`unknown`, r3=`[sp-212]`.
  Call (`bl` at 0x13ffdc): (unknown callee), target 0x144090, r0=`0x0`, r1=`0x19256c`, r2=`[sp-64]`, r3=`0x4`.
  Grouped accesses: 10 reported word store records with absolute target `0x31509c`, including target `abs 0x31509c`, value `0x0` (`str` at 0x140028), target `abs 0x31509c`, value `[0x31509c]+1` (`str` at 0x14006c), target `abs 0x31509c`, value `0x0` (`str` at 0x1403d8).
  Branch (`bhs` at 0x14003c): taken to 0x140074 when `[0x31509c] >= [sp-44] (unsigned, 32-bit)`, falls through to 0x140040 when `[0x31509c] < [sp-44] (unsigned, 32-bit)`.
  Call (`bl` at 0x140098): (unknown callee), target 0x132444, r0=`0x0`, r1=`[[sp-64]+16]`, r2=`[sp-52]`, r3=`[sp-72]`.
  Call (`bl` at 0x1400dc): (unknown callee), target 0x12d2b8, r0=`0x1`, r1=`0x1`, r2=`0x6`, r3=`[sp-208]`.
  Call (`bl` at 0x140104): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x191b00`, r2=`0x80e`, r3=`0x0`.
  Call (`bl` at 0x140114): (unknown callee), target 0x1346fc, r0=`ret(0x22070)`, r1=`unknown`, r2=`unknown`, r3=`[sp-212]`.
  Call (`bl` at 0x140140): (unknown callee), target 0x243d4, r0=`0x192588`, r1=`[sp-208]`, r2=`[sp-36]`, r3=`[0x315034]`.
  Call (`bl` at 0x140154): (unknown callee), target 0x14bab4, r0=`0x0`, r1=`0x1925e4`, r2=`[sp-44]`, r3=`[sp-72]`.
  Call (`bl` at 0x140168): (unknown callee), target 0x14bab4, r0=`0x0`, r1=`0x1925f8`, r2=`[sp-44]`, r3=`[sp-76]`.
  Call (`bl` at 0x14018c): (unknown callee), target 0x132444, r0=`[sp-52]`, r1=`[sp-76]`, r2=`[sp-52]`, r3=`[sp-72]`.
  Call (`bl` at 0x14019c): (unknown callee), target 0x144e3c, r0=`[sp-64]`, r1=`[sp-52]`, r2=`[sp-72]`, r3=`unknown`.
  Call (`bl` at 0x1401d4): (unknown callee), target 0x132444, r0=`[sp-52]`, r1=`[sp-72]`, r2=`0x0`, r3=`[[sp-64]+16]`.
  Call (`bl` at 0x14021c): (unknown callee), target 0x12d2b8, r0=`0x1`, r1=`unknown`, r2=`0x1`, r3=`[sp-208]`.
  Call (`bl` at 0x140244): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x191b00`, r2=`0x82b`, r3=`0x0`.
  Call (`bl` at 0x140254): (unknown callee), target 0x1346fc, r0=`ret(0x22070)`, r1=`unknown`, r2=`unknown`, r3=`[sp-212]`.
  Call (`bl` at 0x1402a4): (unknown callee), target 0x243d4, r0=`0x192628`, r1=`[sp-220]`, r2=`[sp-208]`, r3=`[sp-36]`.
  Call (`bl` at 0x1402b8): (unknown callee), target 0x14bab4, r0=`0x0`, r1=`0x19267c`, r2=`[sp-44]`, r3=`[sp-72]`.
  Call (`bl` at 0x1402cc): (unknown callee), target 0x14bab4, r0=`0x0`, r1=`0x192688`, r2=`[sp-44]`, r3=`[sp-76]`.
  Call (`bl` at 0x1402f0): (unknown callee), target 0x144090, r0=`0x0`, r1=`0x192694`, r2=`[sp-64]`, r3=`0x4`.
  Call (`bl` at 0x140334): (unknown callee), target 0x12d2b8, r0=`0x1`, r1=`0x1`, r2=`0x1`, r3=`[sp-208]`.
  Call (`bl` at 0x14035c): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x191b00`, r2=`0x83f`, r3=`0x0`.
  Call (`bl` at 0x14036c): (unknown callee), target 0x1346fc, r0=`ret(0x22070)`, r1=`unknown`, r2=`unknown`, r3=`[sp-212]`.
  Call (`bl` at 0x140398): (unknown callee), target 0x243d4, r0=`0x1926b0`, r1=`[sp-208]`, r2=`[sp-36]`, r3=`[0x315034]`.
  Call (`bl` at 0x1403ac): (unknown callee), target 0x14bab4, r0=`0x0`, r1=`0x19267c`, r2=`[sp-44]`, r3=`[sp-72]`.
  Call (`bl` at 0x1403c0): (unknown callee), target 0x14bab4, r0=`0x0`, r1=`0x192688`, r2=`[sp-44]`, r3=`[sp-76]`.
  Call (`bl` at 0x14043c): (unknown callee), target 0x144f4c, r0=`[sp-64]`, r1=`0x4`, r2=`[sp-52]`, r3=`sp-204`.
  Call (`bl` at 0x140460): (unknown callee), target 0x14bab4, r0=`0x0`, r1=`0x19270c`, r2=`[sp-44]`, r3=`sp-204`.
  Call (`bl` at 0x140628): (unknown callee), target 0x14bab4, r0=`0x0`, r1=`0x19271c`, r2=`[sp-44]`, r3=`sp-204`.
  Grouped accesses: 2 reported word store records with printed base expression `[sp-32]` and nonnegative word-aligned offsets, including target `[sp-32] +0x0`, value `0x0` (`str` at 0x14063c), target `[sp-32] +0x0`, value `0x1` (`str` at 0x14068c).
  Call (`bl` at 0x1406ac): (unknown callee), target 0x1346fc, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[sp-212]`.
  Recorded r0 = `unknown` (`ldm` at 0x1406c0).

## `JChipGetCfgTarget`

- `JChipGetCfgTarget`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13e5b0 to 0x13e778.
  Established facts:
  Decoded words in [0x13e764, 0x13e778) are not reached from the entry by known edges.
  Word load: target `abs 0x3151bc` (`ldr` at 0x13e5e0).
  Call (`bl` at 0x13e708): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x191b00`, r2=`0x5b1`, r3=`0x0`.
  Grouped accesses: 2 reported byte load records with printed base expression `[sp-40]` and nonnegative byte-aligned offsets, including target `[sp-40] +0x19` (`ldrb` at 0x13e710), target `[sp-40] +0x18` (`ldrb` at 0x13e730).
  Recorded r0 = `[sp-44]` (`ldm` at 0x13e760).

## `JChipGetPortMask`

- `JChipGetPortMask`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13e280 to 0x13e4f0.
  Established facts:
  Decoded words in [0x13e4e8, 0x13e4f0) are not reached from the entry by known edges.
  Grouped accesses: 2 reported word load records with absolute target `0x3151c0`, including target `abs 0x3151c0` (`ldr` at 0x13e2f8), target `abs 0x3151c0` (`ldr` at 0x13e334).
  Word load: target `abs 0x3151bc` (`ldr` at 0x13e3dc).
  Call (`bl` at 0x13e454): (unknown callee), target 0x13c114, r0=`[sp-48]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Byte load: target `(unknown effective address)` (`ldrb` at 0x13e48c).
  Recorded r0 = `[sp-24]` (`ldm` at 0x13e4e4).

## `JChipPortInit`

- `JChipPortInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13b90c to 0x13ba30.
  Established facts:
  Decoded words in [0x13ba20, 0x13ba28), [0x13ba2c, 0x13ba30) are not reached from the entry by known edges.
  Branch (`bne` at 0x13b92c): taken to 0x13b954 when `r0 != 0x0`, falls through to 0x13b930 when `r0 == 0x0`.
  Call (`bl` at 0x13b944): (unknown callee), target 0x13b90c, r0=`0x315060`, r1=`0x1`, r2=`r2`, r3=`r1`.
  Call (`bl` at 0x13b960): (unknown callee), target 0x204d8, r0=`r1`, r1=`0x10`, r2=`r2`, r3=`unknown`.
  Branch (`bne` at 0x13b974): taken to 0x13b984 when `[0x3151bc] != 0x0`, falls through to 0x13b978 when `[0x3151bc] == 0x0`.
  Word store: target `abs 0x3151c0`, value `[sp-28]` (`str` at 0x13b98c).
  Recorded r0 = `unknown` (`ldm` at 0x13ba1c).

## `JChipGetNextChip`

- `JChipGetNextChip`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13cffc to 0x13d1a0.
  Established facts:
  Decoded words in [0x13d198, 0x13d1a0) are not reached from the entry by known edges.
  Branch (`blt` at 0x13d02c): taken to 0x13d048 when `r0 < 0x0 (signed, 32-bit)`, falls through to 0x13d030 when `r0 >= 0x0 (signed, 32-bit)`.
  Word load: target `abs 0x3151bc` (`ldr` at 0x13d080).
  Call (`bl` at 0x13d114): (unknown callee), target 0x13c114, r0=`[sp-28]`, r1=`[[sp-60]]`, r2=`unknown`, r3=`[sp-60]`.
  Word store: target `[sp-32] +0x0`, value `[sp-36]` (`str` at 0x13d13c).
  Recorded r0 = `unknown` (`ldm` at 0x13d194).

## `JChipTargetSetup`

- `JChipTargetSetup`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13b298 to 0x13b6c8.
  Established facts:
  Decoded words in [0x13b6b8, 0x13b6c0) are not reached from the entry by known edges.
  Grouped accesses: 2 reported word load records with absolute target `0x3151c0`, including target `abs 0x3151c0` (`ldr` at 0x13b30c), target `abs 0x3151c0` (`ldr` at 0x13b4d0).
  Call (`bl` at 0x13b37c): (unknown callee), target 0x13c114, r0=`[sp-36]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x13b560): (unknown callee), target 0x13c114, r0=`[sp-52]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x13b61c): (unknown callee), target 0x13dfe0, r0=`r0`, r1=`r1`, r2=`r2`, r3=`unknown`.
  Call (`bl` at 0x13b64c): (unknown callee), target 0x13b6c8, r0=`unknown`, r1=`[sp-30]:2`, r2=`unknown`, r3=`[sp-30]:2`.
  Byte load: target `[sp-28] +0x21` (`ldrsb` at 0x13b674).
  Halfword load: target `[[sp-28]] +0xe` (`ldrh` at 0x13b688).
  Recorded r0 = `unknown` (`ldm` at 0x13b6b4).

## `JChipBrickSetup`

- `JChipBrickSetup`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13b6c8 to 0x13b90c.
  Established facts:
  Decoded words in [0x13b8ec, 0x13b8f0), [0x13b8fc, 0x13b908) are not reached from the entry by known edges.
  Grouped accesses: 4 reported byte load records with printed base expression `r0` and nonnegative byte-aligned offsets, including target `r0 +0x21` (`ldrsb` at 0x13b6e8), target `r0 +0x19` (`ldrb` at 0x13b71c), target `r0 +0x19` (`ldrb` at 0x13b744).
  Grouped accesses: 3 reported word load records with absolute target `0x1a0874`, including target `abs 0x1a0874` (`ldr` at 0x13b720), target `abs 0x1a0874` (`ldr` at 0x13b830), target `abs 0x1a0874` (`ldr` at 0x13b8a8).
  Call (`mov` at 0x13b730): (unknown callee), target (unknown target), r0=`(r1 & 0xffff)`, r1=`[r0+25]:1`, r2=`[0x1a0874]`, r3=`[r0+25]:1`.
  Call (`bl` at 0x13b770): (unknown callee), target 0x3e274, r0=`r0`, r1=`r1`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x13b790): (unknown callee), target 0x3ee14, r0=`[[sp-20]+25]:1`, r1=`unknown`, r2=`unknown`, r3=`[[sp-20]+25]:1`.
  Call (`bl` at 0x13b7a4): (unknown callee), target 0x3e58c, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3ee14)`.
  Call (`bl` at 0x13b7b8): (unknown callee), target 0x3e58c, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x13b7cc): (unknown callee), target 0x3e58c, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x13b7ec): (unknown callee), target 0x3f474, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0x13b80c): (unknown callee), target 0x3ec6c, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3f474)`.
  Call (`mov` at 0x13b840): (unknown callee), target (unknown target), r0=`[sp-22]:2`, r1=`[sp-32]`, r2=`[sp-22]:2`, r3=`[0x1a0874]`.
  Call (`mov` at 0x13b86c): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`0x31e818`, r2=`unknown`, r3=`unknown`.
  Branch (`bne` at 0x13b888): taken to 0x13b898 when `[0x318454] != 0x0`, falls through to 0x13b88c when `[0x318454] == 0x0`.
  Call (`mov` at 0x13b8b8): (unknown callee), target (unknown target), r0=`[sp-22]:2`, r1=`[[sp-20]+25]:1`, r2=`[0x1a0874]`, r3=`[[sp-20]+25]:1`.
  Call (`mov` at 0x13b8d4): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`0x0`, r2=`0x31e818`, r3=`[0x318454]`.
  Recorded r0 = `unknown` (`ldm` at 0x13b8e8).
