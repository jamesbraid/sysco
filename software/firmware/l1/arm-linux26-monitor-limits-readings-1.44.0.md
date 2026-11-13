# ARM Linux 2.6 monitor limits and readings in 1.44.0

Read from `6.5.30/usr/cpu/firmware/sysco/l1.bin`, package `6.5.30:l1#6` (version 1.44.0),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0xa936a8`.
Carrier SHA-256: `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
Package SHA-256: `e57ab8ecd48858e39ecdd0050a23e444a491c35e87f9d8ebb48881fc25229a06`.
Inner executable SHA-256: `b33154e53b00531623db6ff365e80b1c1825bc9427961c59fdc132c24f0c819e`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1629656 bytes.
Segment SHA-256: `f6491062ef0a2afd254f8510f82b4814c563c9256691f9a2b393cf6e84ba7fd8`. Analysed routine spans use the exact
ELF function symbols and sizes. They can include trailing literal words after a return.

`sp` denotes the entry stack pointer. Register expressions such as `r0` denote entry
values, and brackets denote loaded values. Decimal numbers inside expressions retain the
exact tool notation. Call candidates use the assumed ARM AAPCS
register locations r0 through r3 with unknown arity. Stack stores appear in the access tables.
Unknown values and predicates include unsupported instruction effects and merged paths.
Numeric call targets have unknown callee names in the flat-image analysis. Device roles
and pointee meanings remain unknown unless the body establishes them. Tables include the
explicit memory accesses reported on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
Computed transfers in monitorReset and the limit and reading routines leave unresolved
edges. Their tables cover known edges only.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-monitor-limits-readings-facts.txt) retain all tool sections.

## `monitorRemove`

- `monitorRemove`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x11157c to 0x11163c.
  Established. Word stores write 0 at [sp-24] +8, [sp-24] and [sp-24] +4, and a byte store writes 0 at [sp-24] +0xc (`str` at 0x1115f8, `str` at 0x111604, `str` at 0x11161c, `strb` at 0x111628).
  The sole call to unnamed 0x20514 has candidate r0/r1/r2/r3 values [[sp-24]+4]/entry r1/[sp-24]/[sp-24] (`bl` at 0x111610).
  A prior computed word load has unknown effective address, leaving the modified software object unidentified (`ldr` at 0x1115b8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x11158c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x111590 | load | 4 | `abs 0x111634` | `-` |
| `ldr` at 0x111594 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x111598 | load | 4 | `abs 0x314fa4` | `-` |
| `ldr` at 0x1115a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1115ac | load | 4 | `abs 0x111638` | `-` |
| `ldr` at 0x1115b0 | load | 4 | `abs 0x314fa0` | `-` |
| `ldr` at 0x1115b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1115c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1115d0 | load | 4 | `abs 0x111638` | `-` |
| `ldr` at 0x1115d4 | load | 4 | `abs 0x314fa0` | `-` |
| `str` at 0x1115dc | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x1115e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1115e4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x1115f0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x1115f8 | store | 4 | `[sp-24] +0x8` | `0x0` |
| `ldr` at 0x1115fc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x111604 | store | 4 | `[sp-24] +0x0` | `0x0` |
| `ldr` at 0x111608 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11160c | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x111614 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11161c | store | 4 | `[sp-24] +0x4` | `0x0` |
| `ldr` at 0x111620 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x111628 | store | 1 | `[sp-24] +0xc` | `0x0` |
| `ldm` at 0x111630 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x111630 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x111630 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x111610 | `(unknown callee) 0x20514` | `[[sp-24]+4]` | `r1` | `[sp-24]` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x1115a0 | `r0 >= [0x314fa4] (signed, 32-bit)` | `r0 < [0x314fa4] (signed, 32-bit)` |
| `bne` at 0x1115c0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x1115ec | `[[sp-24]+8] == 0x0` | `[[sp-24]+8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x111630 | `r0` | `?` |

## `monitorSetFlags`

- `monitorSetFlags`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x11163c to 0x1116c4.
  Established. A word store saves entry r0 at sp-0x14, and a byte store saves (entry r1 & 0xff) at sp-0x15 (`str` at 0x11164c, `strb` at 0x111654).
  The signed comparison branches to 0x1116b4 when entry r0 is at least [0x314fa4], falling through otherwise (`bge` at 0x111668).
  A later byte store writes ((entry r1 & 0xff) & 0xff) at [sp-28] +0xc, but the field's flag meaning is Unknown (`strb` at 0x1116b0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x11164c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x111654 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x111658 | load | 4 | `abs 0x1116bc` | `-` |
| `ldr` at 0x11165c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x111660 | load | 4 | `abs 0x314fa4` | `-` |
| `ldr` at 0x11166c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x111674 | load | 4 | `abs 0x1116c0` | `-` |
| `ldr` at 0x111678 | load | 4 | `abs 0x314fa0` | `-` |
| `ldr` at 0x111680 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x111690 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x111698 | load | 4 | `abs 0x1116c0` | `-` |
| `ldr` at 0x11169c | load | 4 | `abs 0x314fa0` | `-` |
| `str` at 0x1116a4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x1116a8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x1116ac | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x1116b0 | store | 1 | `[sp-28] +0xc` | `((r1 & 0xff) & 0xff)` |
| `ldm` at 0x1116b8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1116b8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1116b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x111668 | `r0 >= [0x314fa4] (signed, 32-bit)` | `r0 < [0x314fa4] (signed, 32-bit)` |
| `bne` at 0x111688 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1116b8 | `r0` | `r0` |

## `monitorNotPresent`

- `monitorNotPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1116c4 to 0x11181c.
  Established. A signed comparison branches to 0x111770 when (entry r1 & 0xff) is at least [[sp-28]+8], while its fall-through reaches a byte store of 0 at an unknown effective address (`bge` at 0x111740, `strb` at 0x111768).
  The call to unnamed 0x6e1b0 has candidate r0/r1/r2/r3 values entry r0/1/0/0 (`bl` at 0x111780).
  The later call to unnamed 0x22070 has candidate values 6/0x18dff0/0x1ec/0 (`bl` at 0x111800).
  The stored zero does not establish a physical absence indication.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1116d4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x1116dc | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x1116e0 | load | 4 | `abs 0x11180c` | `-` |
| `ldr` at 0x1116e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1116e8 | load | 4 | `abs 0x314fa4` | `-` |
| `ldr` at 0x1116f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1116fc | load | 4 | `abs 0x111810` | `-` |
| `ldr` at 0x111700 | load | 4 | `abs 0x314fa0` | `-` |
| `ldr` at 0x111708 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x111718 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x111720 | load | 4 | `abs 0x111810` | `-` |
| `ldr` at 0x111724 | load | 4 | `abs 0x314fa0` | `-` |
| `str` at 0x11172c | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x111730 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x111734 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x111738 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x111744 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x111748 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x11175c | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0x111768 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x111770 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x111784 | store | 4 | `sp -0x20` | `ret(0x6e1b0)` |
| `ldr` at 0x111788 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x111798 | store | 4 | `sp -0x24` | `ret(0x2031c)+1` |
| `ldr` at 0x11179c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1117c4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x1117c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x1117cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x1117d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1117dc | load | 4 | `abs 0x111814` | `-` |
| `str` at 0x1117e0 | store | 4 | `(unknown)` | `0x18e080` |
| `str` at 0x1117e4 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldrb` at 0x1117e8 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x1117ec | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x1117f4 | load | 4 | `abs 0x111818` | `-` |
| `ldm` at 0x111808 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x111808 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x111808 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x111780 | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x11178c | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x1117d4 | `(unknown callee) 0x203f4` | `[sp-40]` | `[sp-32]` | `[sp-36]` | `?` |
| `bl` at 0x111800 | `(unknown callee) 0x22070` | `0x6` | `0x18dff0` | `0x1ec` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x1116f0 | `r0 >= [0x314fa4] (signed, 32-bit)` | `r0 < [0x314fa4] (signed, 32-bit)` |
| `bne` at 0x111710 | `? != 0x0` | `? == 0x0` |
| `bge` at 0x111740 | `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x111808 | `r0` | `?` |

## `monitorPresent`

- `monitorPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x11181c to 0x111978.
  Established. Word loads read absolute 0x314fa4 and 0x314fa0, and a later word load reads [sp-28] +4 (`ldr` at 0x111840, `ldr` at 0x111858, `ldr` at 0x1118b4).
  A byte store writes 1 at an unknown effective address (`strb` at 0x1118c0).
  The call to unnamed 0x22070 has candidate r0/r1/r2/r3 values 6/0x18dff0/0x1fd/0 (`bl` at 0x111958).
  The byte destination and any physical presence meaning remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x11182c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x111834 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x111838 | load | 4 | `abs 0x111964` | `-` |
| `ldr` at 0x11183c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x111840 | load | 4 | `abs 0x314fa4` | `-` |
| `ldr` at 0x11184c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x111854 | load | 4 | `abs 0x111968` | `-` |
| `ldr` at 0x111858 | load | 4 | `abs 0x314fa0` | `-` |
| `ldr` at 0x111860 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x111870 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x111878 | load | 4 | `abs 0x111968` | `-` |
| `ldr` at 0x11187c | load | 4 | `abs 0x314fa0` | `-` |
| `str` at 0x111884 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x111888 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x11188c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x111890 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x11189c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x1118a0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x1118b4 | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0x1118c0 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x1118c8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1118dc | store | 4 | `sp -0x20` | `ret(0x6e1b0)` |
| `ldr` at 0x1118e0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x1118f0 | store | 4 | `sp -0x24` | `ret(0x2031c)+1` |
| `ldr` at 0x1118f4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x11191c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x111920 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x111924 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x111928 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x111934 | load | 4 | `abs 0x11196c` | `-` |
| `str` at 0x111938 | store | 4 | `(unknown)` | `0x18e0a8` |
| `str` at 0x11193c | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldrb` at 0x111940 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x111944 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x11194c | load | 4 | `abs 0x111970` | `-` |
| `ldr` at 0x111950 | load | 4 | `abs 0x111974` | `-` |
| `ldm` at 0x111960 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x111960 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x111960 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1118d8 | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x1118e4 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x11192c | `(unknown callee) 0x203f4` | `[sp-40]` | `[sp-32]` | `[sp-36]` | `?` |
| `bl` at 0x111958 | `(unknown callee) 0x22070` | `0x6` | `0x18dff0` | `0x1fd` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x111848 | `r0 >= [0x314fa4] (signed, 32-bit)` | `r0 < [0x314fa4] (signed, 32-bit)` |
| `bne` at 0x111868 | `? != 0x0` | `? == 0x0` |
| `bge` at 0x111898 | `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x111960 | `r0` | `?` |

## `monitorIsPresent`

- `monitorIsPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x111978 to 0x111a1c.
  Established. The signed comparison branches to 0x1119c8 when entry r0 is at least [0x314fa4], and that path jumps to 0x111a08 (`bge` at 0x1119a4, `b` at 0x1119c8).
  The call to unnamed 0x114838 has candidate r0/r1/r2/r3 values entry r0/(entry r1 & 0xff)/unknown/(entry r1 & 0xff) (`bl` at 0x1119f0).
  A byte load reads ret(0x114838), and its loaded value is stored at sp-0x24 (`ldrb` at 0x111a00, `str` at 0x111a04).
  The recorded r0 return is [sp-36], without an established physical-presence interpretation (`ldm` at 0x111a10).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x111988 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x111990 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x111994 | load | 4 | `abs 0x111a14` | `-` |
| `ldr` at 0x111998 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x11199c | load | 4 | `abs 0x314fa4` | `-` |
| `ldr` at 0x1119a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1119b0 | load | 4 | `abs 0x111a18` | `-` |
| `ldr` at 0x1119b4 | load | 4 | `abs 0x314fa0` | `-` |
| `ldr` at 0x1119bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1119cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1119d4 | load | 4 | `abs 0x111a18` | `-` |
| `ldr` at 0x1119d8 | load | 4 | `abs 0x314fa0` | `-` |
| `str` at 0x1119e0 | store | 4 | `sp -0x20` | `?` |
| `ldrb` at 0x1119e4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x1119e8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1119f8 | store | 4 | `sp -0x1c` | `ret(0x114838)` |
| `ldr` at 0x1119fc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x111a00 | load | 1 | `ret(0x114838) +0x0` | `-` |
| `str` at 0x111a04 | store | 4 | `sp -0x24` | `[ret(0x114838)]:1` |
| `ldr` at 0x111a08 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x111a10 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x111a10 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x111a10 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1119f0 | `(unknown callee) 0x114838` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x1119a4 | `r0 >= [0x314fa4] (signed, 32-bit)` | `r0 < [0x314fa4] (signed, 32-bit)` |
| `bne` at 0x1119c4 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x111a10 | `r0` | `[sp-36]` |

## `monitorReset`

- `monitorReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x111a1c to 0x111db8.
  Established. Known-edge byte stores write 0 at an unknown effective address and at [sp-32] (`strb` at 0x111b00, `strb` at 0x111b0c).
  A call to unnamed 0x114838 has candidate r0/r1/r2/r3 values [sp-24]/[sp-33]:1/unknown/[sp-33]:1 (`bl` at 0x111ac0).
  The byte store at sp-0x21 writes ([sp-33]:1+1 & 0xff) before the backward transfer to 0x111a88 (`strb` at 0x111d90, `b` at 0x111d94).
  The computed transfer is unresolved (`ldrls` at 0x111b30), so omitted effects and any physical reset actions remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x111a2c | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x111a30 | load | 4 | `abs 0x111da0` | `-` |
| `ldr` at 0x111a34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x111a38 | load | 4 | `abs 0x314fa4` | `-` |
| `ldr` at 0x111a44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x111a4c | load | 4 | `abs 0x111da4` | `-` |
| `ldr` at 0x111a50 | load | 4 | `abs 0x314fa0` | `-` |
| `ldr` at 0x111a58 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x111a68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x111a70 | load | 4 | `abs 0x111da4` | `-` |
| `ldr` at 0x111a74 | load | 4 | `abs 0x314fa0` | `-` |
| `str` at 0x111a7c | store | 4 | `sp -0x2c` | `?` |
| `strb` at 0x111a84 | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0x111a88 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x111a8c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x111a90 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldrb` at 0x111a9c | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x111aa0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x111ab0 | store | 4 | `sp -0x1c` | `ret(0x114714)` |
| `ldrb` at 0x111ab4 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x111ab8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x111ac8 | store | 4 | `sp -0x20` | `ret(0x114838)` |
| `ldr` at 0x111acc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x111ad0 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x111adc | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x111ae0 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x111af4 | load | 4 | `[sp-44] +0x4` | `-` |
| `strb` at 0x111b00 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x111b04 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x111b0c | store | 1 | `[sp-32] +0x0` | `0x0` |
| `ldr` at 0x111b10 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x111b14 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x111b20 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x111b24 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0x111b30 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x111c3c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x111c50 | store | 4 | `sp -0x30` | `ret(0x6e1b0)` |
| `ldr` at 0x111c54 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x111c64 | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x111c68 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x111c90 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x111c94 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x111c98 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x111c9c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x111ca8 | load | 4 | `abs 0x111da8` | `-` |
| `str` at 0x111cac | store | 4 | `(unknown)` | `0x18e0d0` |
| `str` at 0x111cb0 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x111cb4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x111cb8 | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0x111cbc | store | 4 | `(unknown)` | `[[sp-28]]:1` |
| `ldr` at 0x111cc4 | load | 4 | `abs 0x111dac` | `-` |
| `ldr` at 0x111cc8 | load | 4 | `abs 0x111db0` | `-` |
| `ldr` at 0x111cd4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x111ce0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x111cf4 | store | 4 | `sp -0x38` | `ret(0x6e1b0)` |
| `ldr` at 0x111cf8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x111d08 | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x111d0c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x111d34 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x111d38 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x111d3c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x111d40 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x111d4c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x111d58 | load | 4 | `abs 0x111db4` | `-` |
| `str` at 0x111d5c | store | 4 | `(unknown)` | `0x18e0e8` |
| `str` at 0x111d60 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x111d64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x111d68 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0x111d6c | store | 4 | `(unknown)` | `[[sp-28]+8]` |
| `str` at 0x111d70 | store | 4 | `(unknown)` | `ret(0x219a4)` |
| `ldr` at 0x111d78 | load | 4 | `abs 0x111dac` | `-` |
| `ldrb` at 0x111d88 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x111d90 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldm` at 0x111d9c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x111d9c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x111d9c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x111d9c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x111aa8 | `(unknown callee) 0x114714` | `?` | `?` | `[[sp-44]+8]` | `?` |
| `bl` at 0x111ac0 | `(unknown callee) 0x114838` | `[sp-24]` | `[sp-33]:1` | `?` | `[sp-33]:1` |
| `bl` at 0x111c4c | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x111c58 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x111ca0 | `(unknown callee) 0x203f4` | `[sp-56]` | `[sp-48]` | `[sp-52]` | `?` |
| `bl` at 0x111cd0 | `(unknown callee) 0x22070` | `0x9` | `0x18dff0` | `0x256` | `0x0` |
| `bl` at 0x111cf0 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x111cfc | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x111d44 | `(unknown callee) 0x203f4` | `[sp-48]` | `[sp-56]` | `[sp-52]` | `?` |
| `bl` at 0x111d50 | `(unknown callee) 0x219a4` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x111d84 | `(unknown callee) 0x22070` | `0x1` | `0x18dff0` | `0x25c` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x111a40 | `r0 >= [0x314fa4] (signed, 32-bit)` | `r0 < [0x314fa4] (signed, 32-bit)` |
| `bne` at 0x111a60 | `? != 0x0` | `? == 0x0` |
| `bge` at 0x111a98 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x111ad8 | `[[sp-28]]:1 != 0x63` | `[[sp-28]]:1 == 0x63` |
| `beq` at 0x111b1c | `Z == 1` | `not (Z == 1)` |
| `ldrls` at 0x111b30 | `[[sp-28]]:1-1 <= 0x8 (unsigned, 32-bit)` | `[[sp-28]]:1-1 > 0x8 (unsigned, 32-bit)` |
| `beq` at 0x111cdc | `[sp-40] == 0x0` | `[sp-40] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x111d9c | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x111b30).

## `monitorGetCfg`

- `monitorGetCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x111db8 to 0x111e58.
  Established. A word load reads [sp-28], followed by a word store of [[sp-28]] at entry r1 (`ldr` at 0x111e30, `str` at 0x111e34).
  Another word load reads [sp-28] +8, and its value is stored at sp-0x20 (`ldr` at 0x111e3c, `str` at 0x111e40).
  A separate path stores 0 at sp-0x20, while the recorded r0 return is unknown (`str` at 0x111e08, `ldm` at 0x111e4c).
  The returned words have no established configuration semantics.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x111dc8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x111dcc | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x111dd0 | load | 4 | `abs 0x111e50` | `-` |
| `ldr` at 0x111dd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x111dd8 | load | 4 | `abs 0x314fa4` | `-` |
| `ldr` at 0x111de4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x111dec | load | 4 | `abs 0x111e54` | `-` |
| `ldr` at 0x111df0 | load | 4 | `abs 0x314fa0` | `-` |
| `ldr` at 0x111df8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x111e08 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x111e10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x111e18 | load | 4 | `abs 0x111e54` | `-` |
| `ldr` at 0x111e1c | load | 4 | `abs 0x314fa0` | `-` |
| `str` at 0x111e24 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x111e28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x111e2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x111e30 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x111e34 | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0x111e38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x111e3c | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0x111e40 | store | 4 | `sp -0x20` | `[[sp-28]+8]` |
| `ldr` at 0x111e44 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x111e4c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x111e4c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x111e4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x111de0 | `r0 >= [0x314fa4] (signed, 32-bit)` | `r0 < [0x314fa4] (signed, 32-bit)` |
| `bne` at 0x111e00 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x111e4c | `r0` | `?` |

## `monitorCount`

- `monitorCount`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x111e58 to 0x111ee4.
  Established. Word loads read absolute 0x314fa4 and 0x314fa0 (`ldr` at 0x111e74, `ldr` at 0x111e8c).
  One path stores 0 at sp-0x1c, while another loads [sp-24] +8 and stores that word at sp-0x1c (`str` at 0x111ea4, `ldr` at 0x111ec8, `str` at 0x111ecc).
  No calls are emitted and the recorded r0 return is unknown, leaving a count interpretation unestablished (`ldm` at 0x111ed8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x111e68 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x111e6c | load | 4 | `abs 0x111edc` | `-` |
| `ldr` at 0x111e70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x111e74 | load | 4 | `abs 0x314fa4` | `-` |
| `ldr` at 0x111e80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x111e88 | load | 4 | `abs 0x111ee0` | `-` |
| `ldr` at 0x111e8c | load | 4 | `abs 0x314fa0` | `-` |
| `ldr` at 0x111e94 | load | 4 | `(unknown)` | `-` |
| `str` at 0x111ea4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x111eac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x111eb4 | load | 4 | `abs 0x111ee0` | `-` |
| `ldr` at 0x111eb8 | load | 4 | `abs 0x314fa0` | `-` |
| `str` at 0x111ec0 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x111ec4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x111ec8 | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0x111ecc | store | 4 | `sp -0x1c` | `[[sp-24]+8]` |
| `ldr` at 0x111ed0 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x111ed8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x111ed8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x111ed8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x111e7c | `r0 >= [0x314fa4] (signed, 32-bit)` | `r0 < [0x314fa4] (signed, 32-bit)` |
| `bne` at 0x111e9c | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x111ed8 | `r0` | `?` |

## `monitorSetVoltageLimits`

- `monitorSetVoltageLimits`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x111ee4 to 0x112330.
  Established. Known-edge byte stores save masked entry r1/r2/r3 and ([sp] & 0xff) at sp-0x15, sp-0x16, sp-0x17 and sp-0x18 (`strb` at 0x111f04, `strb` at 0x111f0c, `strb` at 0x111f14, `strb` at 0x111f1c).
  A byte load reads [sp-28], and known-edge word stores write 0xfffffc11 and 0xfffff830 at sp-0x2c on separate paths (`ldrb` at 0x111f70, `str` at 0x111f64, `str` at 0x11226c).
  The computed transfers are unresolved (`ldrls` at 0x111fb8, `ldrls` at 0x112074, `ldrls` at 0x11211c), leaving omitted effects unknown.
  Their selector bytes do not establish units, register identities or limit behavior.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x111ef4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x111efc | load | 4 | `sp +0x0` | `-` |
| `strb` at 0x111f04 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x111f0c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x111f14 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `strb` at 0x111f1c | store | 1 | `sp -0x18` | `([sp] & 0xff)` |
| `ldrb` at 0x111f20 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x111f24 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x111f34 | store | 4 | `sp -0x1c` | `ret(0x114714)` |
| `ldrb` at 0x111f38 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x111f3c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x111f4c | store | 4 | `sp -0x20` | `ret(0x114838)` |
| `ldr` at 0x111f50 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x111f54 | load | 1 | `ret(0x114838) +0x0` | `-` |
| `ldr` at 0x111f60 | load | 4 | `abs 0x112328` | `-` |
| `str` at 0x111f64 | store | 4 | `sp -0x2c` | `0xfffffc11` |
| `ldr` at 0x111f6c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x111f70 | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0x111f74 | store | 4 | `sp -0x30` | `[[sp-28]]:1` |
| `ldr` at 0x111f78 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x111f84 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x111f90 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x111fa0 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x111fb0 | load | 1 | `sp -0x16` | `-` |
| `ldrls` at 0x111fb8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x112064 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0x11206c | load | 1 | `sp -0x16` | `-` |
| `ldrls` at 0x112074 | load | 4 | `(unknown)` | `-` |
| `str` at 0x112108 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0x112110 | load | 1 | `sp -0x16` | `-` |
| `ldrls` at 0x11211c | load | 4 | `(unknown)` | `-` |
| `str` at 0x112260 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x112268 | load | 4 | `abs 0x11232c` | `-` |
| `str` at 0x11226c | store | 4 | `sp -0x2c` | `0xfffff830` |
| `ldr` at 0x11231c | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x112324 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x112324 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x112324 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x111f2c | `(unknown callee) 0x114714` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x111f44 | `(unknown callee) 0x114838` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x111f5c | `[ret(0x114838)]:1 != 0x0` | `[ret(0x114838)]:1 == 0x0` |
| `beq` at 0x111f80 | `[[sp-28]]:1 == 0x2` | `[[sp-28]]:1 != 0x2` |
| `bgt` at 0x111f8c | `[[sp-28]]:1 > 0x2 (signed, 32-bit)` | `[[sp-28]]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0x111f98 | `[[sp-28]]:1 == 0x1` | `[[sp-28]]:1 != 0x1` |
| `beq` at 0x111fa8 | `[[sp-28]]:1 == 0x7` | `[[sp-28]]:1 != 0x7` |
| `ldrls` at 0x111fb8 | `[sp-22]:1 <= 0x9 (unsigned, 32-bit)` | `[sp-22]:1 > 0x9 (unsigned, 32-bit)` |
| `ldrls` at 0x112074 | `[sp-22]:1 <= 0x8 (unsigned, 32-bit)` | `[sp-22]:1 > 0x8 (unsigned, 32-bit)` |
| `ldrls` at 0x11211c | `[sp-22]:1-7 <= 0xc (unsigned, 32-bit)` | `[sp-22]:1-7 > 0xc (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x112324 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x111fb8).

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x112074).

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x11211c).

## `monitorGetTempLimits`

- `monitorGetTempLimits`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x112330 to 0x112738.
  Established. The call to unnamed 0x114714 has candidate r0/r1/r2/r3 values entry r0/(entry r1 & 0xff)/entry r2/(entry r1 & 0xff) (`bl` at 0x112364).
  Known-edge byte loads read ret(0x114838) and [sp-32], while word stores write 0xfffffc11 and 0xfffff830 at sp-0x30 (`ldrb` at 0x11238c, `ldrb` at 0x1123a8, `str` at 0x11239c, `str` at 0x112684).
  The computed transfer is unresolved (`ldrls` at 0x1123b4), so omitted effects and the returned fields' limit interpretation remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x112340 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x112344 | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0x11234c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x112354 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0x112358 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x11235c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x11236c | store | 4 | `sp -0x20` | `ret(0x114714)` |
| `ldrb` at 0x112370 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x112374 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x112384 | store | 4 | `sp -0x24` | `ret(0x114838)` |
| `ldr` at 0x112388 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x11238c | load | 1 | `ret(0x114838) +0x0` | `-` |
| `ldr` at 0x112398 | load | 4 | `abs 0x112730` | `-` |
| `str` at 0x11239c | store | 4 | `sp -0x30` | `0xfffffc11` |
| `ldr` at 0x1123a4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x1123a8 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldrls` at 0x1123b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x112680 | load | 4 | `abs 0x112734` | `-` |
| `str` at 0x112684 | store | 4 | `sp -0x30` | `0xfffff830` |
| `ldr` at 0x112724 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x11272c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x11272c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x11272c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x112364 | `(unknown callee) 0x114714` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x11237c | `(unknown callee) 0x114838` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x112394 | `[ret(0x114838)]:1 != 0x0` | `[ret(0x114838)]:1 == 0x0` |
| `ldrls` at 0x1123b4 | `[[sp-32]]:1-1 <= 0x7 (unsigned, 32-bit)` | `[[sp-32]]:1-1 > 0x7 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x11272c | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x1123b4).

## `monitorSetTempLimits`

- `monitorSetTempLimits`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x112738 to 0x112c20.
  Established. A word load reads the word at entry sp, followed by a byte store of ([sp] & 0xff) at sp-0x18 (`ldr` at 0x112750, `strb` at 0x112770).
  The call to unnamed 0x114838 has candidate r0/r1/r2/r3 values [sp-20]/[sp-21]:1/unknown/[sp-21]:1 (`bl` at 0x112798).
  The computed transfer is unresolved (`ldrls` at 0x1127d0), leaving omitted effects unknown.
  Known-edge word stores write 0xfffffc11 and 0xfffff830 at sp-0x2c, without establishing temperature-limit semantics (`str` at 0x1127b8, `str` at 0x112b20).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x112748 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x112750 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0x112758 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x112760 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x112768 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `strb` at 0x112770 | store | 1 | `sp -0x18` | `([sp] & 0xff)` |
| `ldrb` at 0x112774 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x112778 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x112788 | store | 4 | `sp -0x1c` | `ret(0x114714)` |
| `ldrb` at 0x11278c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x112790 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1127a0 | store | 4 | `sp -0x20` | `ret(0x114838)` |
| `ldr` at 0x1127a4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x1127a8 | load | 1 | `ret(0x114838) +0x0` | `-` |
| `ldr` at 0x1127b4 | load | 4 | `abs 0x112c18` | `-` |
| `str` at 0x1127b8 | store | 4 | `sp -0x2c` | `0xfffffc11` |
| `ldr` at 0x1127c0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x1127c4 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0x1127d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x112b1c | load | 4 | `abs 0x112c1c` | `-` |
| `str` at 0x112b20 | store | 4 | `sp -0x2c` | `0xfffff830` |
| `ldr` at 0x112c0c | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x112c14 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x112c14 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x112c14 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x112780 | `(unknown callee) 0x114714` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x112798 | `(unknown callee) 0x114838` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x1127b0 | `[ret(0x114838)]:1 != 0x0` | `[ret(0x114838)]:1 == 0x0` |
| `ldrls` at 0x1127d0 | `[[sp-28]]:1-1 <= 0x7 (unsigned, 32-bit)` | `[[sp-28]]:1-1 > 0x7 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x112c14 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x1127d0).

## `monitorSetTachLimit`

- `monitorSetTachLimit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x112c20 to 0x112d9c.
  Established. A word store saves entry r3 at sp-0x1c, and byte stores save masked entry r1/r2 at sp-0x15 and sp-0x16 (`str` at 0x112c34, `strb` at 0x112c3c, `strb` at 0x112c44).
  Known-edge byte loads read ret(0x114838) and [sp-32] (`ldrb` at 0x112c7c, `ldrb` at 0x112c98).
  The computed transfer is unresolved (`ldrls` at 0x112ca4), and the known-edge r0 return is unknown (`ldm` at 0x112d90).
  Omitted effects and any tachometer or limit meaning remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x112c30 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x112c34 | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0x112c3c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x112c44 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0x112c48 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x112c4c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x112c5c | store | 4 | `sp -0x20` | `ret(0x114714)` |
| `ldrb` at 0x112c60 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x112c64 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x112c74 | store | 4 | `sp -0x24` | `ret(0x114838)` |
| `ldr` at 0x112c78 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x112c7c | load | 1 | `ret(0x114838) +0x0` | `-` |
| `ldr` at 0x112c88 | load | 4 | `abs 0x112d94` | `-` |
| `str` at 0x112c8c | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0x112c94 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x112c98 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldrls` at 0x112ca4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x112d80 | load | 4 | `abs 0x112d98` | `-` |
| `str` at 0x112d84 | store | 4 | `sp -0x28` | `0xfffff830` |
| `ldr` at 0x112d88 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x112d90 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x112d90 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x112d90 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x112c54 | `(unknown callee) 0x114714` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x112c6c | `(unknown callee) 0x114838` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x112c84 | `[ret(0x114838)]:1 != 0x0` | `[ret(0x114838)]:1 == 0x0` |
| `ldrls` at 0x112ca4 | `[[sp-32]]:1-1 <= 0x7 (unsigned, 32-bit)` | `[[sp-32]]:1-1 > 0x7 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x112d90 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x112ca4).

## `monitorReadVoltage`

- `monitorReadVoltage`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x112d9c to 0x113108.
  Established. A known-edge halfword store writes 0 at entry r3 (`strh` at 0x112dcc).
  The call to unnamed 0x114714 has candidate r0/r1/r2/r3 values [sp-20]/[sp-21]:1/entry r3/[sp-21]:1 (`bl` at 0x112ddc).
  Known-edge word stores write 0xfffffc11 and 0xfffff830 at sp-0x30 (`str` at 0x112e14, `str` at 0x113078).
  The computed transfer is unresolved (`ldrls` at 0x112e2c).
  Omitted effects and any physical reading or scale remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x112dac | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x112db0 | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0x112db8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x112dc0 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0x112dc4 | load | 4 | `sp -0x1c` | `-` |
| `strh` at 0x112dcc | store | 2 | `r3 +0x0` | `0x0` |
| `ldrb` at 0x112dd0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x112dd4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x112de4 | store | 4 | `sp -0x20` | `ret(0x114714)` |
| `ldrb` at 0x112de8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x112dec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x112dfc | store | 4 | `sp -0x24` | `ret(0x114838)` |
| `ldr` at 0x112e00 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x112e04 | load | 1 | `ret(0x114838) +0x0` | `-` |
| `ldr` at 0x112e10 | load | 4 | `abs 0x113100` | `-` |
| `str` at 0x112e14 | store | 4 | `sp -0x30` | `0xfffffc11` |
| `ldr` at 0x112e1c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x112e20 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldrls` at 0x112e2c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x113074 | load | 4 | `abs 0x113104` | `-` |
| `str` at 0x113078 | store | 4 | `sp -0x30` | `0xfffff830` |
| `ldr` at 0x1130f4 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x1130fc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1130fc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1130fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x112ddc | `(unknown callee) 0x114714` | `[sp-20]` | `[sp-21]:1` | `r3` | `[sp-21]:1` |
| `bl` at 0x112df4 | `(unknown callee) 0x114838` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x112e0c | `[ret(0x114838)]:1 != 0x0` | `[ret(0x114838)]:1 == 0x0` |
| `ldrls` at 0x112e2c | `[[sp-32]]:1-1 <= 0x8 (unsigned, 32-bit)` | `[[sp-32]]:1-1 > 0x8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1130fc | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x112e2c).

## `monitorReadTemp`

- `monitorReadTemp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x113108 to 0x1134bc.
  Established. A known-edge byte store writes 0 at entry r3, and a later byte load reads ret(0x114838) (`strb` at 0x113138, `ldrb` at 0x113170);
  its nonzero loaded value takes the branch to 0x113188, while the other path stores 0xfffffc11 at sp-0x30 (`bne` at 0x113178, `str` at 0x113180).
  The computed transfer is unresolved (`ldrls` at 0x113198), so omitted effects remain unknown.
  The tool records no definite returned value or measurement units on the known edges (`ldm` at 0x1134b0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x113118 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x11311c | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0x113124 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x11312c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0x113130 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x113138 | store | 1 | `r3 +0x0` | `0x0` |
| `ldrb` at 0x11313c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x113140 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x113150 | store | 4 | `sp -0x20` | `ret(0x114714)` |
| `ldrb` at 0x113154 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x113158 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x113168 | store | 4 | `sp -0x24` | `ret(0x114838)` |
| `ldr` at 0x11316c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x113170 | load | 1 | `ret(0x114838) +0x0` | `-` |
| `ldr` at 0x11317c | load | 4 | `abs 0x1134b4` | `-` |
| `str` at 0x113180 | store | 4 | `sp -0x30` | `0xfffffc11` |
| `ldr` at 0x113188 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x11318c | load | 1 | `[sp-32] +0x0` | `-` |
| `ldrls` at 0x113198 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x113430 | load | 4 | `abs 0x1134b8` | `-` |
| `str` at 0x113434 | store | 4 | `sp -0x30` | `0xfffff830` |
| `ldr` at 0x1134a8 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x1134b0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1134b0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1134b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x113148 | `(unknown callee) 0x114714` | `[sp-20]` | `[sp-21]:1` | `r3` | `[sp-21]:1` |
| `bl` at 0x113160 | `(unknown callee) 0x114838` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x113178 | `[ret(0x114838)]:1 != 0x0` | `[ret(0x114838)]:1 == 0x0` |
| `ldrls` at 0x113198 | `[[sp-32]]:1-1 <= 0x8 (unsigned, 32-bit)` | `[[sp-32]]:1-1 > 0x8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1134b0 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x113198).

## `monitorReadTach`

- `monitorReadTach`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1134bc to 0x11366c.
  Established. A known-edge word store writes 0 at entry r3 (`str` at 0x1134ec).
  The call to unnamed 0x114838 has candidate r0/r1/r2/r3 values [sp-20]/[sp-21]:1/unknown/[sp-21]:1 (`bl` at 0x113514).
  A byte load reads [sp-32], and the known-edge fallback stores 0xfffff830 at sp-0x28 (`ldrb` at 0x113540, `str` at 0x113654).
  The computed transfer is unresolved (`ldrls` at 0x11354c), leaving omitted effects and the software fields' physical-reading interpretation unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1134cc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x1134d0 | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0x1134d8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x1134e0 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0x1134e4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x1134ec | store | 4 | `r3 +0x0` | `0x0` |
| `ldrb` at 0x1134f0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x1134f4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x113504 | store | 4 | `sp -0x20` | `ret(0x114714)` |
| `ldrb` at 0x113508 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x11350c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x11351c | store | 4 | `sp -0x24` | `ret(0x114838)` |
| `ldr` at 0x113520 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x113524 | load | 1 | `ret(0x114838) +0x0` | `-` |
| `ldr` at 0x113530 | load | 4 | `abs 0x113664` | `-` |
| `str` at 0x113534 | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0x11353c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x113540 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldrls` at 0x11354c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x113650 | load | 4 | `abs 0x113668` | `-` |
| `str` at 0x113654 | store | 4 | `sp -0x28` | `0xfffff830` |
| `ldr` at 0x113658 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x113660 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x113660 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x113660 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1134fc | `(unknown callee) 0x114714` | `[sp-20]` | `[sp-21]:1` | `r3` | `[sp-21]:1` |
| `bl` at 0x113514 | `(unknown callee) 0x114838` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x11352c | `[ret(0x114838)]:1 != 0x0` | `[ret(0x114838)]:1 == 0x0` |
| `ldrls` at 0x11354c | `[[sp-32]]:1-1 <= 0x8 (unsigned, 32-bit)` | `[[sp-32]]:1-1 > 0x8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x113660 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x11354c).
