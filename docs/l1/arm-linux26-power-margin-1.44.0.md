# ARM Linux 2.6 power margin paths in 1.44.0

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
and pointee meanings remain unknown unless the body establishes them. Tables include all
explicit memory operands on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
Computed transfers in powerGetMargin, power_margin_multi_init, powerGetReading and
powerMarginName leave unresolved edges. Their tables cover known edges only.
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-power-margin-facts.txt) retain all tool sections.

## `powerSetMargin`

- `powerSetMargin`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x560d4 to 0x561e4.
  Established. The unnamed `0x59024` call carries `[sp-20]`, `ret(0x58af0)`, `[sp-25]:1` and `[sp-25]:1` (`bl` at 0x5615c);
  its zero result takes `0x56194`, while nonzero falls through (`beq` at 0x56170).
  Separate word assignments write `0x5` and `0x4` to `0x1a024c` (`str` at 0x5617c, `str` at 0x5619c).
  Power margin control and the meaning of that state remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x560e4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x560e8 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x560f0 | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `ldr` at 0x560f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x560f8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x56110 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x56118 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5611c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x56128 | store | 4 | `sp -0x20` | `ret(0x58af0)` |
| `ldr` at 0x5612c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x56138 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x56144 | store | 4 | `sp -0x28` | `ret(0x58b90)` |
| `ldrb` at 0x5614c | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x56150 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x56154 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x56164 | store | 4 | `sp -0x24` | `ret(0x59024)` |
| `ldr` at 0x56168 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x56174 | load | 4 | `abs 0x561e0` | `-` |
| `str` at 0x5617c | store | 4 | `abs 0x1a024c` | `0x5` |
| `ldr` at 0x56188 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5618c | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0x56194 | load | 4 | `abs 0x561e0` | `-` |
| `str` at 0x5619c | store | 4 | `abs 0x1a024c` | `0x4` |
| `ldr` at 0x561a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x561ac | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x561b8 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x561bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x561c0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x561d0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x561d4 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x561dc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x561dc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x561dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x560fc | `(unknown callee) 0x53218` | `r0` | `r1` | `r2` | `r2` |
| `bl` at 0x56120 | `(unknown callee) 0x58af0` | `[sp-20]` | `[sp-24]` | `?` | `ret(0x53218)` |
| `bl` at 0x5613c | `(unknown callee) 0x58b90` | `[sp-20]` | `?` | `?` | `ret(0x58af0)` |
| `bl` at 0x5615c | `(unknown callee) 0x59024` | `[sp-20]` | `ret(0x58af0)` | `[sp-25]:1` | `[sp-25]:1` |
| `bl` at 0x56184 | `(unknown callee) 0xa461c` | `0x5` | `?` | `0x5` | `0x1a024c` |
| `bl` at 0x561a4 | `(unknown callee) 0xa461c` | `0x4` | `?` | `0x4` | `0x1a024c` |
| `bl` at 0x561b4 | `(unknown callee) 0x59634` | `[sp-20]` | `[sp-32]` | `0x4` | `?` |
| `bl` at 0x561c8 | `(unknown callee) 0x59544` | `[sp-20]` | `[sp-32]` | `[sp-25]:1` | `[sp-25]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x56108 | `ret(0x53218) != 0x0` | `ret(0x53218) == 0x0` |
| `bne` at 0x56134 | `ret(0x58af0) != 0x0` | `ret(0x58af0) == 0x0` |
| `beq` at 0x56170 | `ret(0x59024) == 0x0` | `ret(0x59024) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x561dc | `r0` | `?` |

## `powerGetMargin`

- `powerGetMargin`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x561e4 to 0x5652c.
  Established. A byte read addresses `ret(0x58af0) +0x6`, and a word read addresses `ret(0x58af0) +0x8` (`ldrb` at 0x56238, `ldr` at 0x56248).
  Equality `[ret(0x58af0)+6]:1 == 0xff` takes `0x564fc`, while inequality falls through (`beq` at 0x56240).
  A word assignment places `0xfffff9be` at `sp -0x3c` (`str` at 0x56500).
  The computed transfer remains unresolved, so omitted paths and hardware margin encoding remain Unknown (`ldrls` at 0x56254).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x561f4 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x561f8 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x561fc | store | 4 | `sp -0x24` | `r2` |
| `ldr` at 0x56200 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x56204 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x56210 | store | 4 | `sp -0x28` | `ret(0x58af0)` |
| `ldr` at 0x56214 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x56220 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5622c | store | 4 | `sp -0x3c` | `ret(0x58b90)` |
| `ldr` at 0x56234 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x56238 | load | 1 | `ret(0x58af0) +0x6` | `-` |
| `ldr` at 0x56244 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x56248 | load | 4 | `ret(0x58af0) +0x8` | `-` |
| `ldrls` at 0x56254 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x56460 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x56474 | store | 4 | `sp -0x30` | `ret(0x6e1b0)` |
| `ldr` at 0x56478 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x56488 | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x5648c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x564b4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x564b8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x564bc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x564c0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x564cc | load | 4 | `abs 0x5651c` | `-` |
| `str` at 0x564d0 | store | 4 | `(unknown)` | `0x161c54` |
| `str` at 0x564d4 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x564d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x564dc | load | 4 | `[sp-40] +0x8` | `-` |
| `str` at 0x564e0 | store | 4 | `(unknown)` | `[[sp-40]+8]` |
| `ldr` at 0x564e8 | load | 4 | `abs 0x56520` | `-` |
| `ldr` at 0x564ec | load | 4 | `abs 0x56524` | `-` |
| `ldr` at 0x564fc | load | 4 | `abs 0x56528` | `-` |
| `str` at 0x56500 | store | 4 | `sp -0x3c` | `0xfffff9be` |
| `str` at 0x5650c | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x56510 | load | 4 | `sp -0x3c` | `-` |
| `ldm` at 0x56518 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x56518 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x56518 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x56518 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x56518 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x56208 | `(unknown callee) 0x58af0` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x56224 | `(unknown callee) 0x58b90` | `[sp-28]` | `?` | `?` | `ret(0x58af0)` |
| `bl` at 0x56470 | `(unknown callee) 0x6e1b0` | `[sp-28]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x5647c | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x564c4 | `(unknown callee) 0x203f4` | `[sp-56]` | `[sp-48]` | `[sp-52]` | `?` |
| `bl` at 0x564f4 | `(unknown callee) 0x22070` | `0x9` | `0x16181c` | `0x8e9` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5621c | `ret(0x58af0) != 0x0` | `ret(0x58af0) == 0x0` |
| `beq` at 0x56240 | `[ret(0x58af0)+6]:1 == 0xff` | `[ret(0x58af0)+6]:1 != 0xff` |
| `ldrls` at 0x56254 | `[ret(0x58af0)+8]-1 <= 0x4 (unsigned, 32-bit)` | `[ret(0x58af0)+8]-1 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x56518 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x56254).

## `powerMarginSlot`

- `powerMarginSlot`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5656c to 0x56d20.
  Established. Separate word assignments write `0x5` and `[sp-28]` to `0x1a024c` (`str` at 0x567f4, `str` at 0x56ca8).
  The unnamed `0x56e14` call carries `[sp-72]`, `[sp-20]`, `[sp-40]` and `[sp-28]` (`bl` at 0x568d4).
  Included word assignments clear `[sp-72] +0xc` and `[sp-76] +0xc` (`str` at 0x56930, `str` at 0x56bec).
  A coprocessor read addresses `[sp-72] +0x20` with unknown width (`ldc` at 0x56980).
  Slot-level power margin hardware responsibilities remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5657c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x56580 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x56584 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x56588 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0x56590 | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0x56598 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x565a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x565b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x565c4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x565d0 | store | 4 | `sp -0x50` | `ret(0x3e58c)` |
| `ldr` at 0x565d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x565ec | load | 4 | `abs 0x56cc8` | `-` |
| `str` at 0x565f0 | store | 4 | `sp -0x50` | `0xfffff05e` |
| `ldr` at 0x565f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5660c | load | 4 | `abs 0x56ccc` | `-` |
| `str` at 0x56610 | store | 4 | `sp -0x50` | `0xfffff05c` |
| `ldr` at 0x56618 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5662c | load | 4 | `abs 0x56cd0` | `-` |
| `str` at 0x56630 | store | 4 | `sp -0x50` | `0xfffff05b` |
| `ldr` at 0x56638 | load | 4 | `abs 0x56cd4` | `-` |
| `ldr` at 0x5663c | load | 4 | `abs 0x1a5c18` | `-` |
| `ldr` at 0x56648 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5665c | load | 4 | `abs 0x56cd8` | `-` |
| `ldr` at 0x56660 | load | 4 | `abs 0x1a5c14` | `-` |
| `ldr` at 0x56668 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x56674 | load | 4 | `abs 0x56cdc` | `-` |
| `str` at 0x56678 | store | 4 | `sp -0x50` | `0xfffff9b9` |
| `ldr` at 0x56680 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x56694 | load | 4 | `abs 0x56cd8` | `-` |
| `ldr` at 0x56698 | load | 4 | `abs 0x1a5c14` | `-` |
| `str` at 0x566a0 | store | 4 | `sp -0x40` | `?` |
| `str` at 0x566a8 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x566b0 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x566b4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x566b8 | load | 4 | `[sp-64] +0x8` | `-` |
| `ldr` at 0x566bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x566c8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x566cc | load | 4 | `[sp-64] +0x4` | `-` |
| `ldr` at 0x566d0 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x566d8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x566e4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x566e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x566fc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldrb` at 0x56704 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x56710 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5671c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x56720 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x56730 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x56738 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x5673c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x56744 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x5674c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x5675c | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0x56764 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x5678c | store | 4 | `sp -0x34` | `ret(0x201fc)` |
| `ldr` at 0x56790 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x5679c | load | 4 | `abs 0x56ce0` | `-` |
| `str` at 0x567a0 | store | 4 | `sp -0x68` | `0x161c70` |
| `ldr` at 0x567a4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x567a8 | store | 4 | `sp -0x64` | `[sp-20]` |
| `ldr` at 0x567b0 | load | 4 | `abs 0x56ce4` | `-` |
| `ldr` at 0x567b4 | load | 4 | `abs 0x56ce8` | `-` |
| `ldr` at 0x567c0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x567dc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x567ec | load | 4 | `abs 0x56cec` | `-` |
| `str` at 0x567f4 | store | 4 | `abs 0x1a024c` | `0x5` |
| `str` at 0x56804 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x5680c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x56810 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x56814 | load | 4 | `[sp-64] +0x8` | `-` |
| `ldr` at 0x56818 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x56824 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x56828 | load | 4 | `[sp-64] +0x4` | `-` |
| `ldr` at 0x5682c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x56834 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x56840 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x56844 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x56858 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldrb` at 0x56860 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5686c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x56878 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5687c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5688c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x568a8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x568b0 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x568b4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x568b8 | store | 4 | `(unknown)` | `[sp-32]` |
| `str` at 0x568c0 | store | 4 | `(unknown)` | `sp-68` |
| `ldr` at 0x568c4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x568c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x568cc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x568d0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x568dc | store | 4 | `sp -0x24` | `ret(0x56e14)` |
| `ldr` at 0x568e0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x568ec | load | 4 | `abs 0x56cf0` | `-` |
| `ldr` at 0x568f0 | load | 4 | `abs 0x1a6474` | `-` |
| `ldr` at 0x568fc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x56900 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x56914 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x5691c | load | 4 | `abs 0x56cf4` | `-` |
| `ldr` at 0x56920 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x56928 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x56930 | store | 4 | `[sp-72] +0xc` | `0x0` |
| `ldr` at 0x56934 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x56940 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x56944 | store | 4 | `sp -0x38` | `[sp-36]` |
| `ldr` at 0x5694c | load | 4 | `abs 0x56cf0` | `-` |
| `ldr` at 0x56950 | load | 4 | `abs 0x1a6474` | `-` |
| `ldr` at 0x5695c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x56960 | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0x56964 | load | 1 | `[sp-72] +0x25` | `-` |
| `ldr` at 0x56968 | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0x5696c | load | 1 | `[sp-72] +0x24` | `-` |
| `ldr` at 0x56970 | load | 4 | `abs 0x56cf8` | `-` |
| `ldr` at 0x56974 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x5697c | load | 4 | `sp -0x48` | `-` |
| `ldc` at 0x56980 | load | unknown | `[sp-72] +0x20` | `-` |
| `ldr` at 0x56988 | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0x5698c | load | 1 | `[sp-72] +0x29` | `-` |
| `ldr` at 0x56990 | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0x56994 | load | 1 | `[sp-72] +0x26` | `-` |
| `str` at 0x56998 | store | 4 | `(unknown)` | `[[sp-72]+38]:1` |
| `ldr` at 0x5699c | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0x569a0 | load | 1 | `[sp-72] +0x28` | `-` |
| `str` at 0x569a4 | store | 4 | `(unknown)` | `[[sp-72]+40]:1` |
| `ldr` at 0x569a8 | load | 4 | `abs 0x56cfc` | `-` |
| `stc` at 0x569ac | store | unknown | `(unknown)` | `?` |
| `ldr` at 0x569bc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x569c0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x569c4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x569c8 | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0x569cc | load | 1 | `[sp-72] +0x2c` | `-` |
| `str` at 0x569d0 | store | 4 | `(unknown)` | `[[sp-72]+44]:1` |
| `ldr` at 0x569d4 | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0x569d8 | load | 1 | `[sp-72] +0x2d` | `-` |
| `str` at 0x569dc | store | 4 | `(unknown)` | `[[sp-72]+45]:1` |
| `ldr` at 0x569e0 | load | 4 | `abs 0x56d00` | `-` |
| `ldr` at 0x569e4 | load | 4 | `[sp-72] +0x18` | `-` |
| `ldr` at 0x569e8 | load | 4 | `[sp-72] +0x14` | `-` |
| `ldr` at 0x569ec | load | 4 | `[sp-72] +0x30` | `-` |
| `ldr` at 0x569f4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x56a00 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x56a08 | store | 4 | `[sp-72] +0xc` | `0x0` |
| `ldr` at 0x56a0c | load | 4 | `abs 0x56cf0` | `-` |
| `ldr` at 0x56a10 | load | 4 | `abs 0x1a6474` | `-` |
| `ldr` at 0x56a1c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x56a20 | load | 4 | `abs 0x56d04` | `-` |
| `ldr` at 0x56a24 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x56a30 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x56a38 | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0x56a3c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x56a44 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x56a48 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x56a50 | store | 4 | `sp -0x28` | `?` |
| `str` at 0x56a5c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x56a60 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x56a6c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x56a7c | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0x56a8c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x56a90 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x56a94 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x56aa0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x56abc | load | 4 | `sp -0x34` | `-` |
| `str` at 0x56ac4 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x56ac8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x56acc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x56ae0 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x56af0 | store | 4 | `sp -0x24` | `ret(0x57634)` |
| `ldr` at 0x56af4 | load | 4 | `abs 0x56cf0` | `-` |
| `ldr` at 0x56af8 | load | 4 | `abs 0x1a6474` | `-` |
| `ldr` at 0x56b04 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x56b08 | load | 4 | `abs 0x56d08` | `-` |
| `ldr` at 0x56b0c | load | 4 | `[sp-76] +0x10` | `-` |
| `ldr` at 0x56b14 | load | 4 | `sp -0x4c` | `-` |
| `ldc` at 0x56b18 | load | unknown | `[sp-76] +0x1c` | `-` |
| `stc` at 0x56b20 | store | unknown | `sp -0x58` | `?` |
| `ldr` at 0x56b24 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x56b28 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x56b2c | load | 4 | `[sp-76] +0x18` | `-` |
| `ldr` at 0x56b30 | load | 4 | `[sp-76] +0x30` | `-` |
| `ldr` at 0x56b40 | load | 4 | `abs 0x56d0c` | `-` |
| `str` at 0x56b44 | store | 4 | `sp -0x5c` | `0x161d6c` |
| `ldr` at 0x56b4c | load | 4 | `abs 0x56d10` | `-` |
| `str` at 0x56b50 | store | 4 | `sp -0x5c` | `0x161d74` |
| `ldr` at 0x56b54 | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x56b58 | load | 1 | `[sp-76] +0x29` | `-` |
| `str` at 0x56b5c | store | 4 | `(unknown)` | `[[sp-76]+41]:1` |
| `ldr` at 0x56b60 | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x56b64 | load | 1 | `[sp-76] +0x26` | `-` |
| `str` at 0x56b68 | store | 4 | `(unknown)` | `[[sp-76]+38]:1` |
| `ldr` at 0x56b6c | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x56b70 | load | 1 | `[sp-76] +0x28` | `-` |
| `str` at 0x56b74 | store | 4 | `(unknown)` | `[[sp-76]+40]:1` |
| `ldr` at 0x56b78 | load | 4 | `abs 0x56d14` | `-` |
| `ldm` at 0x56b80 | load | 4 | `sp -0x58` | `-` |
| `ldm` at 0x56b80 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x56b84 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x56b8c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x56b90 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x56b94 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x56b98 | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x56b9c | load | 1 | `[sp-76] +0x2c` | `-` |
| `str` at 0x56ba0 | store | 4 | `(unknown)` | `[[sp-76]+44]:1` |
| `ldr` at 0x56ba4 | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x56ba8 | load | 1 | `[sp-76] +0x2d` | `-` |
| `str` at 0x56bac | store | 4 | `(unknown)` | `[[sp-76]+45]:1` |
| `ldr` at 0x56bb0 | load | 4 | `abs 0x56d00` | `-` |
| `ldr` at 0x56bb4 | load | 4 | `[sp-76] +0x18` | `-` |
| `ldr` at 0x56bb8 | load | 4 | `[sp-76] +0x14` | `-` |
| `ldr` at 0x56bbc | load | 4 | `[sp-76] +0x30` | `-` |
| `ldr` at 0x56bc4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x56bd0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x56bdc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x56be0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x56be4 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x56bec | store | 4 | `[sp-76] +0xc` | `0x0` |
| `ldr` at 0x56bf0 | load | 4 | `abs 0x56cf0` | `-` |
| `ldr` at 0x56bf4 | load | 4 | `abs 0x1a6474` | `-` |
| `ldr` at 0x56c00 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x56c04 | load | 4 | `abs 0x56d18` | `-` |
| `ldr` at 0x56c08 | load | 4 | `[sp-76] +0x10` | `-` |
| `ldr` at 0x56c14 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x56c20 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x56c28 | store | 4 | `[sp-76] +0xc` | `0x0` |
| `ldr` at 0x56c2c | load | 4 | `abs 0x56cf0` | `-` |
| `ldr` at 0x56c30 | load | 4 | `abs 0x1a6474` | `-` |
| `ldr` at 0x56c3c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x56c40 | load | 4 | `abs 0x56d1c` | `-` |
| `ldr` at 0x56c44 | load | 4 | `[sp-76] +0x10` | `-` |
| `ldr` at 0x56c50 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x56c58 | store | 4 | `sp -0x48` | `[sp-72]+1` |
| `ldr` at 0x56c5c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x56c64 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x56c6c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x56c7c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x56c84 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x56c8c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x56c94 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x56ca0 | load | 4 | `abs 0x56cec` | `-` |
| `ldr` at 0x56ca4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x56ca8 | store | 4 | `abs 0x1a024c` | `[sp-28]` |
| `ldr` at 0x56cb4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x56cb8 | store | 4 | `sp -0x50` | `[sp-56]` |
| `ldr` at 0x56cbc | load | 4 | `sp -0x50` | `-` |
| `ldm` at 0x56cc4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x56cc4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x56cc4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5659c | `(unknown callee) 0x6e190` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x565b4 | `(unknown callee) 0x3e58c` | `[sp-20]` | `?` | `ret(0x6e190)` | `[sp-20]` |
| `bl` at 0x565c8 | `(unknown callee) 0x3e58c` | `[sp-20]` | `?` | `?` | `ret(0x3e58c)` |
| `bl` at 0x565dc | `(unknown callee) 0x3e58c` | `[sp-20]` | `?` | `?` | `ret(0x3e58c)` |
| `bl` at 0x565fc | `(unknown callee) 0x3f474` | `[sp-20]` | `?` | `?` | `ret(0x3e58c)` |
| `bl` at 0x5661c | `(unknown callee) 0x3ec6c` | `[sp-20]` | `?` | `?` | `ret(0x3f474)` |
| `bl` at 0x56784 | `(unknown callee) 0x201fc` | `?` | `?` | `?` | `?` |
| `bl` at 0x567bc | `(unknown callee) 0x22070` | `0x9` | `0x16181c` | `0x91c` | `0x0` |
| `bl` at 0x567e8 | `(unknown callee) 0x20550` | `?` | `0x0` | `?` | `?` |
| `bl` at 0x567fc | `(unknown callee) 0xa461c` | `0x5` | `?` | `0x1a024c` | `0x5` |
| `bl` at 0x568d4 | `(unknown callee) 0x56e14` | `[sp-72]` | `[sp-20]` | `[sp-40]` | `[sp-28]` |
| `bl` at 0x56924 | `(unknown callee) 0x243d4` | `0x161c94` | `?` | `?` | `?` |
| `bl` at 0x56978 | `(unknown callee) 0x243d4` | `0x161ca8` | `[[sp-72]+16]` | `[[sp-72]+37]:1` | `[[sp-72]+36]:1` |
| `bl` at 0x569b8 | `(unknown callee) 0x243d4` | `0x161ccc` | `?` | `?` | `[[sp-72]+41]:1` |
| `bl` at 0x569f0 | `(unknown callee) 0x243d4` | `0x161d08` | `[[sp-72]+24]` | `[[sp-72]+20]` | `[[sp-72]+48]` |
| `bl` at 0x56a28 | `(unknown callee) 0x243d4` | `0x161d50` | `[[sp-72]+16]` | `[sp-72]` | `[sp-72]` |
| `bl` at 0x56a84 | `(unknown callee) 0x53a64` | `0x6e` | `?` | `?` | `0x0` |
| `bl` at 0x56ae8 | `(unknown callee) 0x57634` | `?` | `sp-68` | `?` | `sp-68` |
| `bl` at 0x56b10 | `(unknown callee) 0x243d4` | `0x161d60` | `[[sp-76]+16]` | `?` | `[sp-76]` |
| `bl` at 0x56b88 | `(unknown callee) 0x243d4` | `0x161d7c` | `[sp-88]` | `[sp-84]` | `[sp-92]` |
| `bl` at 0x56bc0 | `(unknown callee) 0x243d4` | `0x161d08` | `[[sp-76]+24]` | `[[sp-76]+20]` | `[[sp-76]+48]` |
| `bl` at 0x56c0c | `(unknown callee) 0x243d4` | `0x161dbc` | `[[sp-76]+16]` | `[sp-76]` | `[sp-76]` |
| `bl` at 0x56c48 | `(unknown callee) 0x243d4` | `0x161dd0` | `[[sp-76]+16]` | `[sp-76]` | `[sp-76]` |
| `bl` at 0x56c90 | `(unknown callee) 0x20514` | `[sp-52]` | `?` | `?` | `?` |
| `bl` at 0x56cb0 | `(unknown callee) 0xa461c` | `[sp-28]` | `?` | `0x1a024c` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x565ac | `ret(0x6e190) == [sp-20]` | `ret(0x6e190) != [sp-20]` |
| `bge` at 0x565c0 | `ret(0x3e58c) >= 0x0 (signed, 32-bit)` | `ret(0x3e58c) < 0x0 (signed, 32-bit)` |
| `bne` at 0x565e8 | `ret(0x3e58c) != 0x0` | `ret(0x3e58c) == 0x0` |
| `bne` at 0x56608 | `ret(0x3f474) != 0x0` | `ret(0x3f474) == 0x0` |
| `bne` at 0x56628 | `ret(0x3ec6c) != 0x0` | `ret(0x3ec6c) == 0x0` |
| `beq` at 0x56644 | `[0x1a5c18] == 0x0` | `[0x1a5c18] != 0x0` |
| `bne` at 0x56670 | `? != 0x0` | `? == 0x0` |
| `ble` at 0x566c4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x566e0 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x5670c | `? == 0xff` | `? != 0xff` |
| `beq` at 0x56718 | `[sp-24] == 0x28` | `[sp-24] != 0x28` |
| `beq` at 0x56728 | `[sp-24] == [sp-40]` | `[sp-24] != [sp-40]` |
| `bne` at 0x56754 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x56798 | `ret(0x201fc) != 0x0` | `ret(0x201fc) == 0x0` |
| `ble` at 0x56820 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x5683c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x56868 | `? == 0xff` | `? != 0xff` |
| `beq` at 0x56874 | `[sp-24] == 0x28` | `[sp-24] != 0x28` |
| `beq` at 0x56884 | `[sp-24] == [sp-40]` | `[sp-24] != [sp-40]` |
| `beq` at 0x568e8 | `ret(0x56e14) == 0x0` | `ret(0x56e14) != 0x0` |
| `beq` at 0x568f8 | `[0x1a6474] == 0x0` | `[0x1a6474] != 0x0` |
| `bne` at 0x5693c | `[sp-56] != 0x0` | `[sp-56] == 0x0` |
| `beq` at 0x56958 | `[0x1a6474] == 0x0` | `[0x1a6474] != 0x0` |
| `beq` at 0x569fc | `[sp-68] == 0x0` | `[sp-68] != 0x0` |
| `beq` at 0x56a18 | `[0x1a6474] == 0x0` | `[0x1a6474] != 0x0` |
| `beq` at 0x56a68 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `bgt` at 0x56a74 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bge` at 0x56a9c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x56ad4 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x56b00 | `[0x1a6474] == 0x0` | `[0x1a6474] != 0x0` |
| `ble` at 0x56b3c | `? <= 0x0 (signed, 32-bit)` | `? > 0x0 (signed, 32-bit)` |
| `beq` at 0x56bcc | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x56bd8 | `[sp-56] != 0x0` | `[sp-56] == 0x0` |
| `beq` at 0x56bfc | `[0x1a6474] == 0x0` | `[0x1a6474] != 0x0` |
| `beq` at 0x56c1c | `[sp-68] == 0x0` | `[sp-68] != 0x0` |
| `beq` at 0x56c38 | `[0x1a6474] == 0x0` | `[0x1a6474] != 0x0` |
| `bne` at 0x56c74 | `[sp-72] != 0x0` | `[sp-72] == 0x0` |
| `bne` at 0x56c9c | `[sp-56] != 0x0` | `[sp-56] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x56cc4 | `r0` | `?` |

## `power_margin_multi_init`

- `power_margin_multi_init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x56e14 to 0x57634.
  Established. Word assignments write `0x4` to `[sp-24] +0x18` and `[sp-24] +0x14` (`str` at 0x56e80, `str` at 0x56e88).
  Byte assignments write `0x2` and `0x6` to `[sp-24] +0x2c` and `[sp-24] +0x2d` (`strb` at 0x56ec4, `strb` at 0x56ed0).
  The unnamed `0x595b8` call carries `[sp-28]`, `[[sp-24]+12]`, unknown and `[sp-24]` (`bl` at 0x56f54).
  A coprocessor assignment addresses `[sp-24] +0x20` with unknown width and value (`stc` at 0x57348).
  The computed transfer remains unresolved, and initialization effects on hardware remain Unknown (`ldrls` at 0x56f98).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x56e24 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x56e28 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x56e2c | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x56e30 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x56e34 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x56e3c | store | 4 | `[sp+4] +0x0` | `0x0` |
| `ldr` at 0x56e40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x56e50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x56e54 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x56e58 | store | 4 | `[sp-24] +0x0` | `[sp-28]` |
| `ldr` at 0x56e5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x56e60 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x56e64 | store | 4 | `[sp-24] +0x4` | `[sp-32]` |
| `ldr` at 0x56e68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x56e6c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x56e70 | store | 4 | `[sp-24] +0x8` | `[sp-36]` |
| `ldr` at 0x56e74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x56e78 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x56e80 | store | 4 | `[sp-24] +0x18` | `0x4` |
| `str` at 0x56e88 | store | 4 | `[sp-24] +0x14` | `0x4` |
| `ldr` at 0x56e8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x56e90 | load | 4 | `abs 0x57614` | `-` |
| `str` at 0x56e94 | store | 4 | `[sp-24] +0x1c` | `0x0` |
| `ldr` at 0x56e98 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x56ea0 | store | 1 | `[sp-24] +0x25` | `0x0` |
| `ldr` at 0x56ea4 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x56eac | store | 1 | `[sp-24] +0x27` | `0x0` |
| `ldr` at 0x56eb0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x56eb8 | store | 4 | `[sp-24] +0x30` | `0x1` |
| `ldr` at 0x56ebc | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x56ec4 | store | 1 | `[sp-24] +0x2c` | `0x2` |
| `ldr` at 0x56ec8 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x56ed0 | store | 1 | `[sp-24] +0x2d` | `0x6` |
| `ldr` at 0x56ed4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x56ed8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x56edc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x56ee4 | store | 4 | `[sp-24] +0xc` | `ret(0x58af0)` |
| `ldr` at 0x56ee8 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x56ef4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x56f00 | store | 4 | `sp -0x44` | `ret(0x58b90)` |
| `ldr` at 0x56f08 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x56f14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x56f18 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x56f1c | load | 4 | `[[sp-24]+12] +0x8` | `-` |
| `ldr` at 0x56f28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x56f2c | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x56f30 | load | 4 | `[[sp-24]+12] +0x8` | `-` |
| `ldr` at 0x56f3c | load | 4 | `abs 0x57618` | `-` |
| `str` at 0x56f40 | store | 4 | `sp -0x44` | `0xfffff9b8` |
| `ldr` at 0x56f48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x56f4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x56f50 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x56f58 | store | 4 | `sp -0x34` | `ret(0x595b8)` |
| `ldr` at 0x56f5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x56f60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x56f64 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x56f70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x56f74 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldrb` at 0x56f78 | load | 1 | `[[sp-24]+12] +0x6` | `-` |
| `ldr` at 0x56f84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x56f88 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x56f8c | load | 4 | `[[sp-24]+12] +0x8` | `-` |
| `ldrls` at 0x56f98 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x571c8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x571dc | store | 4 | `sp -0x38` | `ret(0x6e1b0)` |
| `ldr` at 0x571e0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x571f0 | store | 4 | `sp -0x3c` | `ret(0x2031c)+1` |
| `ldr` at 0x571f4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x5721c | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x57220 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x57224 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x57228 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x57234 | load | 4 | `abs 0x57620` | `-` |
| `str` at 0x57238 | store | 4 | `(unknown)` | `0x161c54` |
| `str` at 0x5723c | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x57240 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57244 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x57248 | load | 4 | `[[sp-24]+12] +0x8` | `-` |
| `str` at 0x5724c | store | 4 | `(unknown)` | `[[[sp-24]+12]+8]` |
| `ldr` at 0x57254 | load | 4 | `abs 0x57624` | `-` |
| `ldr` at 0x57258 | load | 4 | `abs 0x57628` | `-` |
| `ldr` at 0x57268 | load | 4 | `abs 0x5762c` | `-` |
| `str` at 0x5726c | store | 4 | `sp -0x44` | `0xfffff9be` |
| `ldr` at 0x57274 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x57280 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57284 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57288 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5728c | load | 4 | `[sp-24] +0xc` | `-` |
| `strb` at 0x57294 | store | 1 | `[sp-24] +0x26` | `(ret(0x5896c) & 0xff)` |
| `ldr` at 0x57298 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5729c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x572a0 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x572ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x572b0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x572b4 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x572b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x572bc | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x572c8 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x572d0 | store | 4 | `[sp+4] +0x0` | `0x1` |
| `ldr` at 0x572d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x572d8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x572dc | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x572e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x572e4 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x572f4 | store | 4 | `sp -0x44` | `ret(0x59024)` |
| `ldr` at 0x572fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57300 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5730c | store | 4 | `sp -0x2c` | `ret(0x55c7c)` |
| `ldr` at 0x57310 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5731c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x57320 | load | 4 | `abs 0x57618` | `-` |
| `ldr` at 0x5732c | load | 4 | `abs 0x57630` | `-` |
| `str` at 0x57330 | store | 4 | `sp -0x44` | `0xfffff9bd` |
| `ldr` at 0x57338 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5733c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57340 | load | 4 | `sp -0x20` | `-` |
| `stc` at 0x57348 | store | unknown | `[sp-24] +0x20` | `?` |
| `ldr` at 0x5734c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x57350 | store | 4 | `sp -0x48` | `[sp-36]` |
| `ldr` at 0x57354 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x57360 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x5736c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x5737c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x5738c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57390 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57394 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57398 | load | 4 | `[sp-24] +0xc` | `-` |
| `strb` at 0x573a0 | store | 1 | `[sp-24] +0x26` | `(ret(0x588cc) & 0xff)` |
| `ldr` at 0x573a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x573a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x573ac | load | 4 | `sp -0x20` | `-` |
| `stc` at 0x573b4 | store | unknown | `[sp-24] +0x20` | `?` |
| `ldr` at 0x573bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x573c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x573c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x573c8 | load | 4 | `[sp-24] +0xc` | `-` |
| `strb` at 0x573d0 | store | 1 | `[sp-24] +0x26` | `(ret(0x5896c) & 0xff)` |
| `ldr` at 0x573d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x573dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x573e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x573e4 | load | 4 | `[sp-24] +0xc` | `-` |
| `strb` at 0x573ec | store | 1 | `[sp-24] +0x26` | `(ret(0x58a0c) & 0xff)` |
| `ldr` at 0x573f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x573f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x573f8 | load | 4 | `sp -0x20` | `-` |
| `stc` at 0x57400 | store | unknown | `[sp-24] +0x20` | `?` |
| `ldr` at 0x57408 | load | 4 | `abs 0x5761c` | `-` |
| `str` at 0x5740c | store | 4 | `sp -0x44` | `0xfffff9bc` |
| `ldr` at 0x57414 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57418 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x57424 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57428 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x5742c | load | 4 | `[[sp-24]+12] +0x8` | `-` |
| `ldr` at 0x57438 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5743c | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x57440 | load | 4 | `[[sp-24]+12] +0x8` | `-` |
| `ldr` at 0x57450 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57454 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x57458 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5745c | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x57474 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x57478 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5747c | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x57480 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57484 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x5749c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x574a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x574a4 | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x574b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x574b4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x574b8 | load | 1 | `[sp-24] +0x25` | `-` |
| `ldr` at 0x574bc | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x574c8 | store | 1 | `[sp-24] +0x29` | `?` |
| `ldr` at 0x574cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x574d0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x574d4 | load | 1 | `[sp-24] +0x24` | `-` |
| `ldr` at 0x574d8 | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0x574e4 | store | 1 | `[sp-24] +0x28` | `?` |
| `ldr` at 0x574ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x574f0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x574f4 | load | 1 | `[sp-24] +0x25` | `-` |
| `ldr` at 0x574f8 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x57504 | store | 1 | `[sp-24] +0x29` | `?` |
| `ldr` at 0x57508 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5750c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57510 | load | 1 | `[sp-24] +0x24` | `-` |
| `ldr` at 0x57514 | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0x57520 | store | 1 | `[sp-24] +0x28` | `?` |
| `ldr` at 0x57528 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5752c | load | 1 | `[sp-24] +0x24` | `-` |
| `ldr` at 0x57530 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57534 | load | 1 | `[sp-24] +0x25` | `-` |
| `str` at 0x5753c | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x57540 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57544 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57548 | load | 1 | `[sp-24] +0x25` | `-` |
| `strb` at 0x5754c | store | 1 | `[sp-24] +0x29` | `([[sp-24]+37]:1 & 0xff)` |
| `ldr` at 0x57550 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57554 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57558 | load | 1 | `[sp-24] +0x24` | `-` |
| `strb` at 0x5755c | store | 1 | `[sp-24] +0x28` | `([[sp-24]+36]:1 & 0xff)` |
| `ldr` at 0x57560 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57564 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57568 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5756c | load | 1 | `[sp-24] +0x24` | `-` |
| `ldr` at 0x57570 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57574 | load | 1 | `[sp-24] +0x25` | `-` |
| `ldrb` at 0x5758c | load | 1 | `[sp-24] +0x25` | `-` |
| `strb` at 0x57594 | store | 1 | `[sp-24] +0x26` | `?` |
| `ldr` at 0x57598 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x575a0 | store | 1 | `[sp-24] +0x2d` | `0x2` |
| `ldr` at 0x575a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x575a8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x575ac | load | 1 | `[sp-24] +0x29` | `-` |
| `strb` at 0x575b0 | store | 1 | `[sp-24] +0x2b` | `([[sp-24]+41]:1 & 0xff)` |
| `ldr` at 0x575b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x575b8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x575bc | load | 1 | `[sp-24] +0x28` | `-` |
| `strb` at 0x575c0 | store | 1 | `[sp-24] +0x2a` | `([[sp-24]+40]:1 & 0xff)` |
| `ldr` at 0x575c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x575c8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x575cc | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x575d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x575d4 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x575e4 | store | 4 | `sp -0x2c` | `ret(0x59024)` |
| `ldr` at 0x575e8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x575f4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x575f8 | store | 4 | `sp -0x44` | `ret(0x59024)` |
| `str` at 0x57604 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x57608 | load | 4 | `sp -0x44` | `-` |
| `ldm` at 0x57610 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x57610 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x57610 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x57610 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x56e4c | `(unknown callee) 0x20550` | `[sp-24]` | `0x0` | `0x4` | `0x0` |
| `bl` at 0x56ee0 | `(unknown callee) 0x58af0` | `[sp-28]` | `[sp-32]` | `[sp-24]` | `0x6` |
| `bl` at 0x56ef8 | `(unknown callee) 0x58b90` | `[sp-28]` | `?` | `?` | `[[sp-24]+12]` |
| `bl` at 0x56f54 | `(unknown callee) 0x595b8` | `[sp-28]` | `[[sp-24]+12]` | `?` | `[sp-24]` |
| `bl` at 0x56f6c | `(unknown callee) 0x59634` | `[sp-28]` | `[[sp-24]+12]` | `0x5` | `[sp-24]` |
| `bl` at 0x571d8 | `(unknown callee) 0x6e1b0` | `[sp-28]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x571e4 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x5722c | `(unknown callee) 0x203f4` | `[sp-64]` | `[sp-56]` | `[sp-60]` | `?` |
| `bl` at 0x57260 | `(unknown callee) 0x22070` | `0x9` | `0x16181c` | `0xa2f` | `0x0` |
| `bl` at 0x57290 | `(unknown callee) 0x5896c` | `[sp-28]` | `[[sp-24]+12]` | `?` | `[sp-24]` |
| `bl` at 0x572a8 | `(unknown callee) 0x59634` | `[sp-28]` | `[[sp-24]+12]` | `0x0` | `[sp-24]` |
| `bl` at 0x572c4 | `(unknown callee) 0x59544` | `[sp-28]` | `[[sp-24]+12]` | `[[sp-24]+38]:1` | `[[sp-24]+38]:1` |
| `bl` at 0x572ec | `(unknown callee) 0x59024` | `[sp-28]` | `[[sp-24]+12]` | `[[sp-24]+38]:1` | `[[sp-24]+38]:1` |
| `bl` at 0x57304 | `(unknown callee) 0x55c7c` | `[sp-28]` | `[sp-32]` | `?` | `[sp-36]` |
| `bl` at 0x57344 | `(unknown callee) 0x586fc` | `[sp-28]` | `[sp-32]` | `?` | `?` |
| `bl` at 0x5739c | `(unknown callee) 0x588cc` | `[sp-28]` | `[[sp-24]+12]` | `?` | `[sp-24]` |
| `bl` at 0x573b0 | `(unknown callee) 0x5856c` | `[sp-28]` | `[sp-32]` | `?` | `?` |
| `bl` at 0x573cc | `(unknown callee) 0x5896c` | `[sp-28]` | `[[sp-24]+12]` | `?` | `[sp-24]` |
| `bl` at 0x573e8 | `(unknown callee) 0x58a0c` | `[sp-28]` | `[[sp-24]+12]` | `?` | `[sp-24]` |
| `bl` at 0x573fc | `(unknown callee) 0x58698` | `[sp-28]` | `[sp-32]` | `?` | `?` |
| `bl` at 0x575dc | `(unknown callee) 0x59024` | `[sp-28]` | `[[sp-24]+12]` | `[[sp-24]+38]:1` | `[[sp-24]+38]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x56ef0 | `[[sp-24]+12] != 0x0` | `[[sp-24]+12] == 0x0` |
| `bne` at 0x56f10 | `[sp-36] != 0x6` | `[sp-36] == 0x6` |
| `beq` at 0x56f24 | `[[[sp-24]+12]+8] == 0x1` | `[[[sp-24]+12]+8] != 0x1` |
| `beq` at 0x56f38 | `[[[sp-24]+12]+8] == 0x2` | `[[[sp-24]+12]+8] != 0x2` |
| `beq` at 0x56f80 | `[[[sp-24]+12]+6]:1 == 0xff` | `[[[sp-24]+12]+6]:1 != 0xff` |
| `ldrls` at 0x56f98 | `[[[sp-24]+12]+8]-1 <= 0x4 (unsigned, 32-bit)` | `[[[sp-24]+12]+8]-1 > 0x4 (unsigned, 32-bit)` |
| `bne` at 0x5727c | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `beq` at 0x57318 | `ret(0x55c7c) == 0x1` | `ret(0x55c7c) != 0x1` |
| `beq` at 0x57328 | `ret(0x55c7c) == 0xfffff9b8` | `ret(0x55c7c) != 0xfffff9b8` |
| `beq` at 0x5735c | `[sp-36] == 0x2` | `[sp-36] != 0x2` |
| `bgt` at 0x57368 | `[sp-36] > 0x2 (signed, 32-bit)` | `[sp-36] <= 0x2 (signed, 32-bit)` |
| `beq` at 0x57374 | `[sp-36] == 0x1` | `[sp-36] != 0x1` |
| `beq` at 0x57384 | `[sp-36] == 0x3` | `[sp-36] != 0x3` |
| `beq` at 0x57420 | `[[sp-24]+38]:1 == 0x0` | `[[sp-24]+38]:1 != 0x0` |
| `beq` at 0x57434 | `[[[sp-24]+12]+8] == 0x1` | `[[[sp-24]+12]+8] != 0x1` |
| `beq` at 0x57448 | `[[[sp-24]+12]+8] == 0x2` | `[[[sp-24]+12]+8] != 0x2` |
| `ble` at 0x574ac | `[[sp-24]+48] <= 0x0 (signed, 32-bit)` | `[[sp-24]+48] > 0x0 (signed, 32-bit)` |
| `beq` at 0x575f0 | `ret(0x59024) == 0x0` | `ret(0x59024) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x57610 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x56f98).

## `power_margin_multi`

- `power_margin_multi`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x57634 to 0x57f80.
  Established. Separate word assignments clear `[sp-28] +0x0` and assign `0x1` to that location (`str` at 0x576d8, `str` at 0x57720).
  A byte assignment writes `0x2` to `[sp-24] +0x2d` (`strb` at 0x57ec8).
  The unnamed `0x59024` call carries `[[sp-24]]`, `[[sp-24]+12]`, `[[sp-24]+38]:1` and `[[sp-24]+38]:1` (`bl` at 0x57ee8).
  Power margin adjustment and the meaning of these software fields remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x57644 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x57648 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x5764c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57658 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5765c | load | 4 | `r0 +0xc` | `-` |
| `ldr` at 0x57668 | load | 4 | `abs 0x57f40` | `-` |
| `str` at 0x5766c | store | 4 | `sp -0x30` | `0x161de0` |
| `ldr` at 0x57674 | load | 4 | `abs 0x57f44` | `-` |
| `ldr` at 0x57678 | load | 4 | `abs 0x57f48` | `-` |
| `ldr` at 0x57684 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57688 | load | 4 | `(unknown)` | `-` |
| `str` at 0x5768c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x57690 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57694 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57698 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x576a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x576a4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x576b4 | store | 4 | `sp -0x20` | `ret(0x5801c)` |
| `ldr` at 0x576b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x576c4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x576c8 | store | 4 | `sp -0x28` | `ret(0x5801c)` |
| `ldr` at 0x576d0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x576d8 | store | 4 | `[sp-28] +0x0` | `0x0` |
| `ldr` at 0x576dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x576e0 | load | 4 | `sp -0x18` | `-` |
| `ldc` at 0x576e4 | load | unknown | `[sp-24] +0x1c` | `-` |
| `ldc` at 0x576e8 | load | unknown | `[sp-24] +0x20` | `-` |
| `ldr` at 0x576f8 | load | 4 | `abs 0x57f4c` | `-` |
| `ldr` at 0x576fc | load | 4 | `abs 0x1a6474` | `-` |
| `ldr` at 0x57708 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5770c | load | 4 | `abs 0x57f50` | `-` |
| `ldr` at 0x57710 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x57718 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x57720 | store | 4 | `[sp-28] +0x0` | `0x1` |
| `ldr` at 0x57724 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57728 | load | 4 | `sp -0x18` | `-` |
| `ldc` at 0x5772c | load | unknown | `[sp-24] +0x1c` | `-` |
| `ldc` at 0x57730 | load | unknown | `[sp-24] +0x20` | `-` |
| `ldr` at 0x57740 | load | 4 | `sp -0x18` | `-` |
| `ldc` at 0x57744 | load | unknown | `[sp-24] +0x20` | `-` |
| `ldc` at 0x57748 | load | unknown | `sp -0x24` | `-` |
| `ldr` at 0x57758 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5775c | load | 4 | `[sp-24] +0x18` | `-` |
| `ldr` at 0x57770 | load | 4 | `abs 0x57f4c` | `-` |
| `ldr` at 0x57774 | load | 4 | `abs 0x1a6474` | `-` |
| `ldr` at 0x57780 | load | 4 | `sp -0x18` | `-` |
| `ldc` at 0x57784 | load | unknown | `sp -0x24` | `-` |
| `ldr` at 0x5778c | load | 4 | `abs 0x57f54` | `-` |
| `ldr` at 0x57790 | load | 4 | `[sp-24] +0x10` | `-` |
| `stc` at 0x57794 | store | unknown | `sp -0x38` | `?` |
| `ldr` at 0x577a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x577a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x577a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x577ac | load | 4 | `[sp-24] +0x18` | `-` |
| `ldrb` at 0x577b0 | load | 1 | `[sp-24] +0x26` | `-` |
| `strb` at 0x577b8 | store | 1 | `[sp-24] +0x26` | `?` |
| `ldr` at 0x577bc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x577c4 | store | 4 | `[sp-28] +0x0` | `0x1` |
| `ldr` at 0x577c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x577cc | load | 4 | `sp -0x18` | `-` |
| `ldc` at 0x577d0 | load | unknown | `[sp-24] +0x1c` | `-` |
| `ldc` at 0x577d4 | load | unknown | `[sp-24] +0x20` | `-` |
| `ldr` at 0x577e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x577e8 | load | 4 | `[sp-24] +0x18` | `-` |
| `ldr` at 0x577fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57800 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57804 | load | 4 | `[sp-24] +0x18` | `-` |
| `ldr` at 0x57808 | load | 4 | `[sp-24] +0x14` | `-` |
| `ldr` at 0x57818 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5781c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57820 | load | 4 | `[sp-24] +0x18` | `-` |
| `ldr` at 0x57824 | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x57838 | load | 4 | `abs 0x57f4c` | `-` |
| `ldr` at 0x5783c | load | 4 | `abs 0x1a6474` | `-` |
| `ldr` at 0x57848 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5784c | load | 4 | `abs 0x57f58` | `-` |
| `ldr` at 0x57850 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x57858 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x57860 | store | 4 | `[sp-28] +0x0` | `0x1` |
| `ldr` at 0x57864 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57868 | load | 4 | `sp -0x18` | `-` |
| `ldc` at 0x5786c | load | unknown | `[sp-24] +0x1c` | `-` |
| `ldc` at 0x57870 | load | unknown | `[sp-24] +0x20` | `-` |
| `ldr` at 0x57880 | load | 4 | `sp -0x18` | `-` |
| `ldc` at 0x57884 | load | unknown | `[sp-24] +0x20` | `-` |
| `ldc` at 0x57888 | load | unknown | `sp -0x24` | `-` |
| `ldr` at 0x57898 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5789c | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x578a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x578a4 | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x578ac | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x578b0 | load | 1 | `[sp-24] +0x28` | `-` |
| `ldr` at 0x578bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x578c0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x578c4 | load | 1 | `[sp-24] +0x28` | `-` |
| `ldrb` at 0x578c8 | load | 1 | `[sp-24] +0x24` | `-` |
| `ldr` at 0x578d4 | load | 4 | `abs 0x57f4c` | `-` |
| `ldr` at 0x578d8 | load | 4 | `abs 0x1a6474` | `-` |
| `ldr` at 0x578e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x578e8 | load | 4 | `abs 0x57f5c` | `-` |
| `ldr` at 0x578ec | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x578f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x578f8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x578fc | load | 1 | `[sp-24] +0x28` | `-` |
| `strb` at 0x57900 | store | 1 | `[sp-24] +0x26` | `([[sp-24]+40]:1 & 0xff)` |
| `ldr` at 0x57904 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5790c | store | 4 | `[sp-28] +0x0` | `0x1` |
| `ldr` at 0x57914 | load | 4 | `sp -0x18` | `-` |
| `ldc` at 0x57918 | load | unknown | `[sp-24] +0x20` | `-` |
| `ldc` at 0x5791c | load | unknown | `sp -0x24` | `-` |
| `ldr` at 0x5792c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57930 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x57934 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57938 | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x57940 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57944 | load | 1 | `[sp-24] +0x29` | `-` |
| `ldr` at 0x57950 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57954 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57958 | load | 1 | `[sp-24] +0x29` | `-` |
| `ldrb` at 0x5795c | load | 1 | `[sp-24] +0x25` | `-` |
| `ldr` at 0x57968 | load | 4 | `abs 0x57f4c` | `-` |
| `ldr` at 0x5796c | load | 4 | `abs 0x1a6474` | `-` |
| `ldr` at 0x57978 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5797c | load | 4 | `abs 0x57f60` | `-` |
| `ldr` at 0x57980 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x57988 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x57990 | store | 4 | `[sp-28] +0x0` | `0x1` |
| `ldr` at 0x57994 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57998 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x579a4 | load | 4 | `abs 0x57f4c` | `-` |
| `ldr` at 0x579a8 | load | 4 | `abs 0x1a6474` | `-` |
| `ldr` at 0x579b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x579b8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x579bc | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x579c0 | load | 4 | `abs 0x57f64` | `-` |
| `ldr` at 0x579c4 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x579cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x579d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x579d4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x579d8 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x579dc | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x579e0 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x579f0 | store | 4 | `sp -0x20` | `ret(0x59024)` |
| `ldr` at 0x579f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x57a00 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x57a08 | store | 4 | `[sp-28] +0x0` | `0x0` |
| `ldr` at 0x57a0c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x57a10 | store | 4 | `sp -0x28` | `[sp-32]` |
| `ldr` at 0x57a18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57a1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57a20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57a24 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x57a28 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x57a2c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x57a34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57a38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57a3c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57a40 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x57a44 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x57a48 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x57a58 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x57a60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57a64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57a68 | load | 4 | `[sp-24] +0x18` | `-` |
| `ldr` at 0x57a6c | load | 4 | `[sp-24] +0x14` | `-` |
| `ldr` at 0x57a7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57a80 | load | 4 | `[sp-24] +0x18` | `-` |
| `ldr` at 0x57a94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57a98 | load | 4 | `[sp-24] +0x14` | `-` |
| `ldr` at 0x57aac | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x57ab4 | store | 1 | `[sp-24] +0x2c` | `0x1` |
| `ldr` at 0x57ab8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57abc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57ac0 | load | 4 | `[sp-24] +0x18` | `-` |
| `str` at 0x57ac4 | store | 4 | `[sp-24] +0x14` | `[[sp-24]+24]` |
| `ldr` at 0x57ac8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57acc | load | 4 | `sp -0x18` | `-` |
| `ldc` at 0x57ad0 | load | unknown | `[sp-24] +0x1c` | `-` |
| `ldc` at 0x57ad4 | load | unknown | `[sp-24] +0x20` | `-` |
| `ldr` at 0x57ae4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57ae8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57aec | load | 1 | `[sp-24] +0x26` | `-` |
| `ldrb` at 0x57af0 | load | 1 | `[sp-24] +0x24` | `-` |
| `ldr` at 0x57afc | load | 4 | `abs 0x57f68` | `-` |
| `str` at 0x57b00 | store | 4 | `sp -0x28` | `0xfffff9ba` |
| `ldr` at 0x57b08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57b0c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57b10 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldrb` at 0x57b14 | load | 1 | `[sp-24] +0x28` | `-` |
| `ldr` at 0x57b20 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57b24 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x57b28 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57b2c | load | 1 | `[sp-24] +0x28` | `-` |
| `ldr` at 0x57b30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57b34 | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x57b48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57b4c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57b50 | load | 1 | `[sp-24] +0x28` | `-` |
| `ldrb` at 0x57b54 | load | 1 | `[sp-24] +0x2a` | `-` |
| `ldr` at 0x57b60 | load | 4 | `abs 0x57f4c` | `-` |
| `ldr` at 0x57b64 | load | 4 | `abs 0x1a6474` | `-` |
| `ldr` at 0x57b70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57b74 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57b78 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x57b7c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57b80 | load | 1 | `[sp-24] +0x28` | `-` |
| `ldr` at 0x57b84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57b88 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x57b8c | store | 4 | `(unknown)` | `[[sp-24]+48]` |
| `ldr` at 0x57b90 | load | 4 | `abs 0x57f6c` | `-` |
| `ldr` at 0x57b94 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x57ba4 | load | 4 | `abs 0x57f70` | `-` |
| `str` at 0x57ba8 | store | 4 | `sp -0x28` | `0xfffff9b7` |
| `ldr` at 0x57bb0 | load | 4 | `abs 0x57f4c` | `-` |
| `ldr` at 0x57bb4 | load | 4 | `abs 0x1a6474` | `-` |
| `ldr` at 0x57bc0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57bc4 | load | 4 | `abs 0x57f74` | `-` |
| `ldr` at 0x57bc8 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x57bd0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57bd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57bd8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57bdc | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x57be0 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x57be4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x57bec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57bf0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57bf4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57bf8 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x57bfc | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x57c00 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x57c0c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x57c14 | store | 4 | `[sp-28] +0x0` | `0x1` |
| `str` at 0x57c1c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x57c24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57c28 | load | 4 | `[sp-24] +0x14` | `-` |
| `ldr` at 0x57c3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57c40 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57c44 | load | 1 | `[sp-24] +0x26` | `-` |
| `strb` at 0x57c48 | store | 1 | `[sp-24] +0x29` | `([[sp-24]+38]:1 & 0xff)` |
| `ldr` at 0x57c4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57c50 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57c54 | load | 1 | `[sp-24] +0x28` | `-` |
| `ldr` at 0x57c58 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57c5c | load | 1 | `[sp-24] +0x29` | `-` |
| `ldr` at 0x57c74 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57c78 | load | 1 | `[sp-24] +0x2d` | `-` |
| `str` at 0x57c94 | store | 4 | `[sp-24] +0x18` | `?` |
| `ldr` at 0x57c9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57ca0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57ca4 | load | 1 | `[sp-24] +0x2c` | `-` |
| `str` at 0x57ca8 | store | 4 | `[sp-24] +0x18` | `[[sp-24]+44]:1` |
| `ldr` at 0x57cb0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57cb4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57cb8 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldrb` at 0x57cbc | load | 1 | `[sp-24] +0x25` | `-` |
| `ldr` at 0x57cc8 | load | 4 | `abs 0x57f68` | `-` |
| `str` at 0x57ccc | store | 4 | `sp -0x28` | `0xfffff9ba` |
| `ldr` at 0x57cd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57cd8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57cdc | load | 1 | `[sp-24] +0x26` | `-` |
| `ldrb` at 0x57ce0 | load | 1 | `[sp-24] +0x29` | `-` |
| `ldr` at 0x57cec | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57cf0 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x57cf4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57cf8 | load | 1 | `[sp-24] +0x29` | `-` |
| `ldr` at 0x57cfc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57d00 | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x57d14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57d18 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57d1c | load | 1 | `[sp-24] +0x29` | `-` |
| `ldrb` at 0x57d20 | load | 1 | `[sp-24] +0x2b` | `-` |
| `ldr` at 0x57d2c | load | 4 | `abs 0x57f4c` | `-` |
| `ldr` at 0x57d30 | load | 4 | `abs 0x1a6474` | `-` |
| `ldr` at 0x57d3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57d40 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57d44 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x57d48 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57d4c | load | 1 | `[sp-24] +0x29` | `-` |
| `ldr` at 0x57d50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57d54 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x57d58 | store | 4 | `(unknown)` | `[[sp-24]+48]` |
| `ldr` at 0x57d5c | load | 4 | `abs 0x57f78` | `-` |
| `ldr` at 0x57d60 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x57d70 | load | 4 | `abs 0x57f70` | `-` |
| `str` at 0x57d74 | store | 4 | `sp -0x28` | `0xfffff9b7` |
| `ldr` at 0x57d7c | load | 4 | `abs 0x57f4c` | `-` |
| `ldr` at 0x57d80 | load | 4 | `abs 0x1a6474` | `-` |
| `ldr` at 0x57d8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57d90 | load | 4 | `abs 0x57f74` | `-` |
| `ldr` at 0x57d94 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x57d9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57da0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57da4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57da8 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x57dac | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x57db0 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x57db8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57dbc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57dc0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57dc4 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x57dc8 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x57dcc | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x57dd8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x57de0 | store | 4 | `[sp-28] +0x0` | `0x1` |
| `str` at 0x57de8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x57df0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57df4 | load | 4 | `[sp-24] +0x14` | `-` |
| `ldr` at 0x57e08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57e0c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57e10 | load | 1 | `[sp-24] +0x26` | `-` |
| `strb` at 0x57e14 | store | 1 | `[sp-24] +0x28` | `([[sp-24]+38]:1 & 0xff)` |
| `ldr` at 0x57e18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57e1c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57e20 | load | 1 | `[sp-24] +0x28` | `-` |
| `ldr` at 0x57e24 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57e28 | load | 1 | `[sp-24] +0x29` | `-` |
| `str` at 0x57e3c | store | 4 | `sp -0x2c` | `ret(0x205b0)` |
| `ldr` at 0x57e40 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57e44 | load | 1 | `[sp-24] +0x2d` | `-` |
| `ldr` at 0x57e48 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x57e60 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x57e64 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x57e6c | store | 4 | `[sp-24] +0x18` | `?` |
| `ldr` at 0x57e74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57e78 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57e7c | load | 1 | `[sp-24] +0x2c` | `-` |
| `str` at 0x57e84 | store | 4 | `[sp-24] +0x18` | `?` |
| `ldr` at 0x57e88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57e8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57e90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57e94 | load | 4 | `[sp-24] +0x18` | `-` |
| `ldr` at 0x57e98 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x57ea0 | store | 4 | `[sp-24] +0x18` | `?` |
| `ldr` at 0x57ea4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57ea8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57eac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57eb0 | load | 4 | `[sp-24] +0x18` | `-` |
| `ldrb` at 0x57eb4 | load | 1 | `[sp-24] +0x26` | `-` |
| `strb` at 0x57ebc | store | 1 | `[sp-24] +0x26` | `(([[sp-24]+38]:1 + [[sp-24]+24]) & 0xff)` |
| `ldr` at 0x57ec0 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x57ec8 | store | 1 | `[sp-24] +0x2d` | `0x2` |
| `ldr` at 0x57ecc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57ed0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57ed4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57ed8 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x57edc | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x57ee0 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x57ef0 | store | 4 | `sp -0x20` | `ret(0x59024)` |
| `ldr` at 0x57ef4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x57f00 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x57f04 | store | 4 | `sp -0x28` | `ret(0x59024)` |
| `ldr` at 0x57f0c | load | 4 | `abs 0x57f4c` | `-` |
| `ldr` at 0x57f10 | load | 4 | `abs 0x1a6474` | `-` |
| `ldr` at 0x57f1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57f20 | load | 4 | `abs 0x57f7c` | `-` |
| `ldr` at 0x57f24 | load | 4 | `[sp-24] +0x10` | `-` |
| `str` at 0x57f30 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x57f34 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x57f3c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x57f3c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x57f3c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x57f3c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x57680 | `(unknown callee) 0x22070` | `0x9` | `0x16181c` | `0xa95` | `0x0` |
| `bl` at 0x576ac | `(unknown callee) 0x5801c` | `?` | `?` | `0x0` | `?` |
| `bl` at 0x57714 | `(unknown callee) 0x243d4` | `0x161e1c` | `[[sp-24]+16]` | `[sp-24]` | `[sp-24]` |
| `bl` at 0x57760 | `(unknown callee) 0x205b0` | `[[sp-24]+24]` | `?` | `[sp-24]` | `[sp-24]` |
| `bl` at 0x5779c | `(unknown callee) 0x243d4` | `0x161e3c` | `[[sp-24]+16]` | `?` | `?` |
| `bl` at 0x577ec | `(unknown callee) 0x205b0` | `[[sp-24]+24]` | `?` | `[sp-24]` | `[sp-24]` |
| `bl` at 0x57854 | `(unknown callee) 0x243d4` | `0x161e74` | `[[sp-24]+16]` | `[sp-24]` | `[sp-24]` |
| `bl` at 0x578f0 | `(unknown callee) 0x243d4` | `0x161eac` | `[[sp-24]+16]` | `[sp-24]` | `[sp-24]` |
| `bl` at 0x57984 | `(unknown callee) 0x243d4` | `0x161ed8` | `[[sp-24]+16]` | `[sp-24]` | `[sp-24]` |
| `bl` at 0x579c8 | `(unknown callee) 0x243d4` | `0x161f14` | `[[sp-24]+16]` | `[[sp-24]+38]:1` | `[sp-24]` |
| `bl` at 0x579e8 | `(unknown callee) 0x59024` | `[[sp-24]]` | `[[sp-24]+12]` | `[[sp-24]+38]:1` | `[[sp-24]+38]:1` |
| `bl` at 0x57a30 | `(unknown callee) 0x59634` | `[[sp-24]]` | `[[sp-24]+12]` | `[[sp-24]+8]` | `[sp-24]` |
| `bl` at 0x57a50 | `(unknown callee) 0x59544` | `[[sp-24]]` | `[[sp-24]+12]` | `[[sp-24]+38]:1` | `[[sp-24]+38]:1` |
| `bl` at 0x57a84 | `(unknown callee) 0x205b0` | `[[sp-24]+24]` | `[[sp-24]+24]` | `[sp-24]` | `[sp-24]` |
| `bl` at 0x57a9c | `(unknown callee) 0x205b0` | `[[sp-24]+20]` | `?` | `?` | `[sp-24]` |
| `bl` at 0x57ba0 | `(unknown callee) 0x243d4` | `0x161f34` | `[[sp-24]+16]` | `[[sp-24]+38]:1` | `[[sp-24]+40]:1` |
| `bl` at 0x57bcc | `(unknown callee) 0x243d4` | `0x161f64` | `[[sp-24]+16]` | `[sp-24]` | `[sp-24]` |
| `bl` at 0x57be8 | `(unknown callee) 0x59634` | `[[sp-24]]` | `[[sp-24]+12]` | `[[sp-24]+8]` | `[sp-24]` |
| `bl` at 0x57c08 | `(unknown callee) 0x59544` | `[[sp-24]]` | `[[sp-24]+12]` | `[[sp-24]+38]:1` | `[[sp-24]+38]:1` |
| `bl` at 0x57c2c | `(unknown callee) 0x205b0` | `[[sp-24]+20]` | `[[sp-24]+38]:1` | `[[sp-24]+40]:1` | `[sp-24]` |
| `bl` at 0x57c68 | `(unknown callee) 0x205b0` | `?` | `?` | `[[sp-24]+40]:1` | `?` |
| `bl` at 0x57c84 | `(unknown callee) 0x14fe64` | `ret(0x205b0)` | `[[sp-24]+45]:1` | `ret(0x205b0)` | `[[sp-24]+45]:1` |
| `bl` at 0x57d6c | `(unknown callee) 0x243d4` | `0x161f8c` | `[[sp-24]+16]` | `[[sp-24]+38]:1` | `[[sp-24]+41]:1` |
| `bl` at 0x57d98 | `(unknown callee) 0x243d4` | `0x161f64` | `[[sp-24]+16]` | `[sp-24]` | `[sp-24]` |
| `bl` at 0x57db4 | `(unknown callee) 0x59634` | `[[sp-24]]` | `[[sp-24]+12]` | `[[sp-24]+8]` | `[sp-24]` |
| `bl` at 0x57dd4 | `(unknown callee) 0x59544` | `[[sp-24]]` | `[[sp-24]+12]` | `[[sp-24]+38]:1` | `[[sp-24]+38]:1` |
| `bl` at 0x57df8 | `(unknown callee) 0x205b0` | `[[sp-24]+20]` | `[[sp-24]+38]:1` | `[[sp-24]+41]:1` | `[sp-24]` |
| `bl` at 0x57e34 | `(unknown callee) 0x205b0` | `?` | `?` | `[[sp-24]+40]:1` | `?` |
| `bl` at 0x57e50 | `(unknown callee) 0x14fe64` | `ret(0x205b0)` | `[[sp-24]+45]:1` | `?` | `[[sp-24]+45]:1` |
| `bl` at 0x57ee8 | `(unknown callee) 0x59024` | `[[sp-24]]` | `[[sp-24]+12]` | `[[sp-24]+38]:1` | `[[sp-24]+38]:1` |
| `bl` at 0x57f28 | `(unknown callee) 0x243d4` | `0x161fbc` | `[[sp-24]+16]` | `?` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x57654 | `r0 == 0x0` | `r0 != 0x0` |
| `bne` at 0x57664 | `[r0+12] != 0x0` | `[r0+12] == 0x0` |
| `beq` at 0x576c0 | `ret(0x5801c) == 0x0` | `ret(0x5801c) != 0x0` |
| `beq` at 0x576f0 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x57704 | `[0x1a6474] == 0x0` | `[0x1a6474] != 0x0` |
| `bmi` at 0x57738 | `N == 1` | `not (N == 1)` |
| `bls` at 0x57750 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x5776c | `ret(0x205b0) != 0x1` | `ret(0x205b0) == 0x1` |
| `beq` at 0x5777c | `[0x1a6474] == 0x0` | `[0x1a6474] != 0x0` |
| `bge` at 0x577dc | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x577f8 | `ret(0x205b0) != 0x1` | `ret(0x205b0) == 0x1` |
| `bne` at 0x57814 | `([[sp-24]+24] + [[sp-24]+20]) != 0x0` | `([[sp-24]+24] + [[sp-24]+20]) == 0x0` |
| `bgt` at 0x57830 | `? > 0x0 (signed, 32-bit)` | `? <= 0x0 (signed, 32-bit)` |
| `beq` at 0x57844 | `[0x1a6474] == 0x0` | `[0x1a6474] != 0x0` |
| `bmi` at 0x57878 | `N == 1` | `not (N == 1)` |
| `bgt` at 0x57890 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0x578b8 | `([[sp-24]+38]:1 + [[sp-24]+48]) != [[sp-24]+40]:1` | `([[sp-24]+38]:1 + [[sp-24]+48]) == [[sp-24]+40]:1` |
| `beq` at 0x578d0 | `[[sp-24]+40]:1 == [[sp-24]+36]:1` | `[[sp-24]+40]:1 != [[sp-24]+36]:1` |
| `beq` at 0x578e0 | `[0x1a6474] == 0x0` | `[0x1a6474] != 0x0` |
| `bmi` at 0x57924 | `N == 1` | `not (N == 1)` |
| `bne` at 0x5794c | `? != [[sp-24]+41]:1` | `? == [[sp-24]+41]:1` |
| `beq` at 0x57964 | `[[sp-24]+41]:1 == [[sp-24]+37]:1` | `[[sp-24]+41]:1 != [[sp-24]+37]:1` |
| `beq` at 0x57974 | `[0x1a6474] == 0x0` | `[0x1a6474] != 0x0` |
| `beq` at 0x579a0 | `[[sp-28]] == 0x0` | `[[sp-28]] != 0x0` |
| `beq` at 0x579b0 | `[0x1a6474] == 0x0` | `[0x1a6474] != 0x0` |
| `beq` at 0x579fc | `ret(0x59024) == 0x0` | `ret(0x59024) != 0x0` |
| `bge` at 0x57a78 | `? >= 0x0 (signed, 32-bit)` | `? < 0x0 (signed, 32-bit)` |
| `bne` at 0x57a90 | `ret(0x205b0) != 0x2` | `ret(0x205b0) == 0x2` |
| `bne` at 0x57aa8 | `ret(0x205b0) != 0x2` | `ret(0x205b0) == 0x2` |
| `bmi` at 0x57adc | `N == 1` | `not (N == 1)` |
| `bne` at 0x57af8 | `[[sp-24]+38]:1 != [[sp-24]+36]:1` | `[[sp-24]+38]:1 == [[sp-24]+36]:1` |
| `beq` at 0x57b1c | `[[sp-24]+38]:1 == [[sp-24]+40]:1` | `[[sp-24]+38]:1 != [[sp-24]+40]:1` |
| `beq` at 0x57b40 | `[[sp-24]+38]:1 == ([[sp-24]+40]:1 + [[sp-24]+48])` | `[[sp-24]+38]:1 != ([[sp-24]+40]:1 + [[sp-24]+48])` |
| `bne` at 0x57b5c | `[[sp-24]+40]:1 != [[sp-24]+42]:1` | `[[sp-24]+40]:1 == [[sp-24]+42]:1` |
| `beq` at 0x57b6c | `[0x1a6474] == 0x0` | `[0x1a6474] != 0x0` |
| `beq` at 0x57bbc | `[0x1a6474] == 0x0` | `[0x1a6474] != 0x0` |
| `ble` at 0x57c38 | `ret(0x205b0) <= 0x3 (signed, 32-bit)` | `ret(0x205b0) > 0x3 (signed, 32-bit)` |
| `bne` at 0x57cc4 | `[[sp-24]+38]:1 != [[sp-24]+37]:1` | `[[sp-24]+38]:1 == [[sp-24]+37]:1` |
| `beq` at 0x57ce8 | `[[sp-24]+38]:1 == [[sp-24]+41]:1` | `[[sp-24]+38]:1 != [[sp-24]+41]:1` |
| `beq` at 0x57d0c | `[[sp-24]+38]:1 == ?` | `[[sp-24]+38]:1 != ?` |
| `bne` at 0x57d28 | `[[sp-24]+41]:1 != [[sp-24]+43]:1` | `[[sp-24]+41]:1 == [[sp-24]+43]:1` |
| `beq` at 0x57d38 | `[0x1a6474] == 0x0` | `[0x1a6474] != 0x0` |
| `beq` at 0x57d88 | `[0x1a6474] == 0x0` | `[0x1a6474] != 0x0` |
| `ble` at 0x57e04 | `ret(0x205b0) <= 0x3 (signed, 32-bit)` | `ret(0x205b0) > 0x3 (signed, 32-bit)` |
| `beq` at 0x57efc | `ret(0x59024) == 0x0` | `ret(0x59024) != 0x0` |
| `beq` at 0x57f18 | `[0x1a6474] == 0x0` | `[0x1a6474] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x57f3c | `r0` | `?` |

## `powerGetReading`

- `powerGetReading`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5801c to 0x58280.
  Established. Byte reads address `ret(0x58af0) +0x4` and `ret(0x58af0) +0x5` (`ldrb` at 0x58078, `ldrb` at 0x58080).
  The unnamed `0x112d9c` call carries `[sp-36]`, `[ret(0x58af0)+4]:1`, `[ret(0x58af0)+5]:1` and `sp-54` (`bl` at 0x58098).
  A halfword assignment writes `([sp-54]:2 & 0xffff)` to `[sp-44] +0x0` (`strh` at 0x580d0).
  A coprocessor assignment addresses `[sp-48] +0x0` with unknown width and value (`stc` at 0x58110).
  The computed transfer remains unresolved, and physical reading units and conversion remain Unknown (`ldrls` at 0x58148).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `stc` at 0x58024 | store | unknown | `sp -0x20` | `?` |
| `str` at 0x58030 | store | 4 | `sp -0x24` | `r0` |
| `str` at 0x58034 | store | 4 | `sp -0x28` | `r1` |
| `str` at 0x58038 | store | 4 | `sp -0x2c` | `r2` |
| `str` at 0x5803c | store | 4 | `sp -0x30` | `r3` |
| `ldr` at 0x58040 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x58044 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x58050 | store | 4 | `sp -0x34` | `ret(0x58af0)` |
| `ldr` at 0x58054 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x58060 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5806c | store | 4 | `sp -0x40` | `ret(0x58b90)` |
| `ldr` at 0x58074 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x58078 | load | 1 | `ret(0x58af0) +0x4` | `-` |
| `ldr` at 0x5807c | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x58080 | load | 1 | `ret(0x58af0) +0x5` | `-` |
| `ldr` at 0x58088 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x580a0 | store | 4 | `sp -0x3c` | `ret(0x112d9c)` |
| `ldr` at 0x580a4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x580b0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x580b4 | store | 4 | `sp -0x40` | `ret(0x112d9c)` |
| `ldr` at 0x580bc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x580c8 | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0x580cc | load | 2 | `sp -0x36` | `-` |
| `strh` at 0x580d0 | store | 2 | `[sp-44] +0x0` | `([sp-54]:2 & 0xffff)` |
| `ldr` at 0x580d4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x580e0 | load | 4 | `sp -0x30` | `-` |
| `ldrh` at 0x580e4 | load | 2 | `sp -0x36` | `-` |
| `ldr` at 0x580ec | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x580f0 | load | 1 | `[sp-52] +0x4` | `-` |
| `ldr` at 0x580f4 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x580f8 | load | 1 | `[sp-52] +0x5` | `-` |
| `ldr` at 0x580fc | load | 4 | `sp -0x24` | `-` |
| `stc` at 0x58110 | store | unknown | `[sp-48] +0x0` | `?` |
| `ldr` at 0x58114 | load | 4 | `abs 0x5827c` | `-` |
| `ldr` at 0x58118 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x5811c | load | 1 | `[sp-52] +0x0` | `-` |
| `ldrb` at 0x5813c | load | 1 | `(unknown)` | `-` |
| `ldrls` at 0x58148 | load | 4 | `(unknown)` | `-` |
| `str` at 0x58248 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x5824c | load | 4 | `sp -0x40` | `-` |
| `ldc` at 0x58250 | load | unknown | `sp -0x20` | `-` |
| `ldm` at 0x58258 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x58258 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x58258 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x58258 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x58048 | `(unknown callee) 0x58af0` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x58064 | `(unknown callee) 0x58b90` | `[sp-36]` | `?` | `?` | `ret(0x58af0)` |
| `bl` at 0x58098 | `(unknown callee) 0x112d9c` | `[sp-36]` | `[ret(0x58af0)+4]:1` | `[ret(0x58af0)+5]:1` | `sp-54` |
| `bl` at 0x58108 | `(unknown callee) 0x113d20` | `[sp-36]` | `[[sp-52]+4]:1` | `[[sp-52]+5]:1` | `[[sp-52]+5]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5805c | `ret(0x58af0) != 0x0` | `ret(0x58af0) == 0x0` |
| `beq` at 0x580ac | `ret(0x112d9c) == 0x0` | `ret(0x112d9c) != 0x0` |
| `beq` at 0x580c4 | `[sp-44] == 0x0` | `[sp-44] != 0x0` |
| `beq` at 0x580dc | `[sp-48] == 0x0` | `[sp-48] != 0x0` |
| `ldrls` at 0x58148 | `? <= 0x5 (unsigned, 32-bit)` | `? > 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x58258 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x58148).

## `powerGetName`

- `powerGetName`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x58280 to 0x583c4.
  Established. Included word reads address `0x1a5c18` and `0x1a5c14` (`ldr` at 0x5829c, `ldr` at 0x582c0).
  Zero `[0x1a5c18]` takes `0x582d4`, while nonzero falls through (`beq` at 0x582a4).
  The taken path assigns `0x161fcc` to `sp -0x20` (`str` at 0x582d8).
  Power name record ownership and hardware identity remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x58290 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x58294 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x58298 | load | 4 | `abs 0x583b4` | `-` |
| `ldr` at 0x5829c | load | 4 | `abs 0x1a5c18` | `-` |
| `ldr` at 0x582a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x582bc | load | 4 | `abs 0x583b8` | `-` |
| `ldr` at 0x582c0 | load | 4 | `abs 0x1a5c14` | `-` |
| `ldr` at 0x582c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x582d4 | load | 4 | `abs 0x583bc` | `-` |
| `str` at 0x582d8 | store | 4 | `sp -0x20` | `0x161fcc` |
| `ldr` at 0x582e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x582f4 | load | 4 | `abs 0x583b8` | `-` |
| `ldr` at 0x582f8 | load | 4 | `abs 0x1a5c14` | `-` |
| `str` at 0x58300 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x58304 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58308 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5831c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x58324 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x58330 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58334 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x58348 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x58350 | load | 4 | `(unknown)` | `-` |
| `str` at 0x58354 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x5835c | load | 4 | `abs 0x583c0` | `-` |
| `ldr` at 0x58360 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58364 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x58378 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x58380 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x583a0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x583a4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x583a8 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x583b0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x583b0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x583b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x582a4 | `[0x1a5c18] == 0x0` | `[0x1a5c18] != 0x0` |
| `bne` at 0x582d0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x5832c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x583b0 | `r0` | `?` |

## `powerMarginName`

- `powerMarginName`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x583c4 to 0x584d8.
  Established. An included byte read addresses `ret(0x58af0) +0x6` (`ldrb` at 0x5840c).
  Equality `[ret(0x58af0)+6]:1 == 0xff` takes `0x584a0`, while inequality falls through (`beq` at 0x58414).
  Separate word assignments place `0x162008` and `0x162010` at `sp -0x20` (`str` at 0x58498, `str` at 0x584a4).
  The computed transfer remains unresolved, and omitted name choices and hardware margin meaning remain Unknown (`ldrls` at 0x5842c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x583d4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x583d8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x583dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x583e0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x583ec | store | 4 | `sp -0x1c` | `ret(0x58af0)` |
| `ldr` at 0x583f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x583fc | load | 4 | `abs 0x584b4` | `-` |
| `str` at 0x58400 | store | 4 | `sp -0x20` | `0x161fd8` |
| `ldr` at 0x58408 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5840c | load | 1 | `ret(0x58af0) +0x6` | `-` |
| `ldr` at 0x58418 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5841c | load | 4 | `sp -0x1c` | `-` |
| `ldrls` at 0x5842c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x58494 | load | 4 | `abs 0x584d0` | `-` |
| `str` at 0x58498 | store | 4 | `sp -0x20` | `0x162008` |
| `ldr` at 0x584a0 | load | 4 | `abs 0x584d4` | `-` |
| `str` at 0x584a4 | store | 4 | `sp -0x20` | `0x162010` |
| `ldr` at 0x584a8 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x584b0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x584b0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x584b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x583e4 | `(unknown callee) 0x58af0` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x58420 | `(unknown callee) 0x595b8` | `[sp-20]` | `ret(0x58af0)` | `?` | `[ret(0x58af0)+6]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x583f8 | `ret(0x58af0) != 0x0` | `ret(0x58af0) == 0x0` |
| `beq` at 0x58414 | `[ret(0x58af0)+6]:1 == 0xff` | `[ret(0x58af0)+6]:1 != 0xff` |
| `ldrls` at 0x5842c | `ret(0x595b8) <= 0x5 (unsigned, 32-bit)` | `ret(0x595b8) > 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x584b0 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x5842c).

## `powerLowVoltage`

- `powerLowVoltage`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5856c to 0x58698.
  Established. The unnamed `0xa3c7c` call carries `0x0`, entry `r1`, unknown and unknown (`bl` at 0x585b0).
  An included word read addresses `ret(0xa3c7c) +0x24` (`ldr` at 0x585cc).
  Separate unnamed `0x584d8` and `0x58808` calls each carry `[sp-32]`, `[sp-36]`, unknown and unknown (`bl` at 0x5865c, `bl` at 0x5866c).
  Coprocessor arithmetic effects, voltage units and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `stc` at 0x58574 | store | unknown | `sp -0x1c` | `?` |
| `str` at 0x58580 | store | 4 | `sp -0x20` | `r0` |
| `str` at 0x58584 | store | 4 | `sp -0x24` | `r1` |
| `ldr` at 0x58588 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5859c | load | 4 | `abs 0x58694` | `-` |
| `ldr` at 0x585a0 | load | 4 | `abs 0x1a5c14` | `-` |
| `str` at 0x585a8 | store | 4 | `sp -0x28` | `?` |
| `str` at 0x585b8 | store | 4 | `sp -0x2c` | `ret(0xa3c7c)` |
| `ldr` at 0x585bc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x585c8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x585cc | load | 4 | `ret(0xa3c7c) +0x24` | `-` |
| `ldr` at 0x585dc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x585e0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x585f4 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldrb` at 0x585fc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x58608 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5860c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x58620 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldrb` at 0x58628 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x58638 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5863c | load | 4 | `sp -0x24` | `-` |
| `stc` at 0x58644 | store | unknown | `(unknown)` | `?` |
| `str` at 0x5864c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x58654 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x58658 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x58664 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x58668 | load | 4 | `sp -0x24` | `-` |
| `stc` at 0x58680 | store | unknown | `sp -0x30` | `?` |
| `ldc` at 0x58684 | load | unknown | `sp -0x30` | `-` |
| `ldc` at 0x58688 | load | unknown | `sp -0x1c` | `-` |
| `ldm` at 0x58690 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x58690 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x58690 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x585b0 | `(unknown callee) 0xa3c7c` | `0x0` | `r1` | `?` | `?` |
| `bl` at 0x58640 | `(unknown callee) 0x584d8` | `[sp-32]` | `[sp-36]` | `?` | `?` |
| `bl` at 0x5865c | `(unknown callee) 0x584d8` | `[sp-32]` | `[sp-36]` | `?` | `?` |
| `bl` at 0x5866c | `(unknown callee) 0x58808` | `[sp-32]` | `[sp-36]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x585c4 | `ret(0xa3c7c) == 0x0` | `ret(0xa3c7c) != 0x0` |
| `beq` at 0x585d8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x58604 | `? == 0xe` | `? != 0xe` |
| `beq` at 0x58630 | `? == 0x22` | `? != 0x22` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x58690 | `r0` | `?` |

## `powerHighVoltage`

- `powerHighVoltage`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x58698 to 0x586fc.
  Established. The unnamed `0x584d8` call carries entry `r0`, entry `r1`, entry `r2` and entry `r3` (`bl` at 0x586bc).
  The unnamed `0x58808` call carries `[sp-32]`, `[sp-36]`, unknown and unknown (`bl` at 0x586cc).
  A coprocessor assignment has unknown effective address, width and value, and a later coprocessor read has unknown effective address and width (`stc` at 0x586e0, `ldc` at 0x586ec).
  High-voltage computation and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `stc` at 0x586a0 | store | unknown | `sp -0x1c` | `?` |
| `str` at 0x586ac | store | 4 | `sp -0x20` | `r0` |
| `str` at 0x586b0 | store | 4 | `sp -0x24` | `r1` |
| `ldr` at 0x586b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x586b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x586c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x586c8 | load | 4 | `sp -0x24` | `-` |
| `stc` at 0x586e0 | store | unknown | `(unknown)` | `?` |
| `str` at 0x586e8 | store | 4 | `(unknown)` | `?` |
| `ldc` at 0x586ec | load | unknown | `(unknown)` | `-` |
| `ldc` at 0x586f0 | load | unknown | `sp -0x1c` | `-` |
| `ldm` at 0x586f8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x586f8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x586f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x586bc | `(unknown callee) 0x584d8` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x586cc | `(unknown callee) 0x58808` | `[sp-32]` | `[sp-36]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x586f8 | `r0` | `ret(0x58808)` |

## `powerNominalVoltage`

- `powerNominalVoltage`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x586fc to 0x58808.
  Established. An included word read addresses `0x1a5c14` (`ldr` at 0x5872c).
  Zero `ret(0xa3c7c)` takes `0x587e0`, while nonzero falls through (`beq` at 0x58750).
  Separate unnamed `0x58698` and `0x584d8` calls each carry `[sp-20]`, `[sp-24]`, unknown and unknown (`bl` at 0x587cc, `bl` at 0x587e8).
  Nominal-voltage selection, coprocessor result values and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5870c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x58710 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x58714 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58728 | load | 4 | `abs 0x58804` | `-` |
| `ldr` at 0x5872c | load | 4 | `abs 0x1a5c14` | `-` |
| `str` at 0x58734 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x58744 | store | 4 | `sp -0x20` | `ret(0xa3c7c)` |
| `ldr` at 0x58748 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x58754 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x58758 | load | 4 | `ret(0xa3c7c) +0x24` | `-` |
| `ldr` at 0x58768 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5876c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x58780 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x58788 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x58794 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58798 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x587ac | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x587b4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x587c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x587c8 | load | 4 | `sp -0x18` | `-` |
| `stc` at 0x587d0 | store | unknown | `sp -0x28` | `?` |
| `str` at 0x587d8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x587e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x587e4 | load | 4 | `sp -0x18` | `-` |
| `stc` at 0x587ec | store | unknown | `sp -0x28` | `?` |
| `str` at 0x587f4 | store | 4 | `sp -0x24` | `?` |
| `ldc` at 0x587f8 | load | unknown | `sp -0x24` | `-` |
| `ldm` at 0x58800 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x58800 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x58800 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5873c | `(unknown callee) 0xa3c7c` | `0x0` | `r1` | `?` | `?` |
| `bl` at 0x587cc | `(unknown callee) 0x58698` | `[sp-20]` | `[sp-24]` | `?` | `?` |
| `bl` at 0x587e8 | `(unknown callee) 0x584d8` | `[sp-20]` | `[sp-24]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x58750 | `ret(0xa3c7c) == 0x0` | `ret(0xa3c7c) != 0x0` |
| `beq` at 0x58764 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x58790 | `? == 0xe` | `? != 0xe` |
| `beq` at 0x587bc | `? == 0x22` | `? != 0x22` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x58800 | `r0` | `?` |

## `powerMarginPercent`

- `powerMarginPercent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x58808 to 0x588a0.
  Established. Separate word assignments place `0x3cdd67ba` and `0x3d4ccccd` at `sp -0x20` (`str` at 0x58878, `str` at 0x58884).
  The condition `? == 0xe` takes `0x58874`, while `? != 0xe` falls through (`beq` at 0x5886c).
  A coprocessor read addresses `sp -0x20` with unknown width (`ldc` at 0x58888).
  Percentage representation, coprocessor result and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x58818 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x5881c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x58820 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58834 | load | 4 | `abs 0x58894` | `-` |
| `ldr` at 0x58838 | load | 4 | `abs 0x1a5c14` | `-` |
| `str` at 0x58840 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x58844 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58848 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5885c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x58864 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x58874 | load | 4 | `abs 0x58898` | `-` |
| `str` at 0x58878 | store | 4 | `sp -0x20` | `0x3cdd67ba` |
| `ldr` at 0x58880 | load | 4 | `abs 0x5889c` | `-` |
| `str` at 0x58884 | store | 4 | `sp -0x20` | `0x3d4ccccd` |
| `ldc` at 0x58888 | load | unknown | `sp -0x20` | `-` |
| `ldm` at 0x58890 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x58890 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x58890 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5886c | `? == 0xe` | `? != 0xe` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x58890 | `r0` | `r0` |

## `powerLowMargin`

- `powerLowMargin`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x588cc to 0x5896c.
  Established. An included byte read addresses `[sp-24] +0xf` (`ldrb` at 0x58958).
  The unnamed `0x588a0` call carries `[sp-20]`, `[sp-24]`, unknown and `[[sp-24]]:1` (`bl` at 0x58940).
  Equality `[[sp-24]]:1 == 0xe` takes `0x58938`, while inequality falls through (`beq` at 0x58920).
  Low-margin field encoding and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x588dc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x588e0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x588f0 | store | 4 | `sp -0x1c` | `ret(0xa3c7c)` |
| `ldr` at 0x588f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58900 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58904 | load | 4 | `ret(0xa3c7c) +0x24` | `-` |
| `ldr` at 0x58914 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x58918 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x58924 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x58928 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x58938 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5893c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5894c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x58954 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x58958 | load | 1 | `[sp-24] +0xf` | `-` |
| `str` at 0x5895c | store | 4 | `sp -0x20` | `[[sp-24]+15]:1` |
| `ldr` at 0x58960 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x58968 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x58968 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x58968 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x588e8 | `(unknown callee) 0xa3c7c` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x58940 | `(unknown callee) 0x588a0` | `[sp-20]` | `[sp-24]` | `?` | `[[sp-24]]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x588fc | `ret(0xa3c7c) == 0x0` | `ret(0xa3c7c) != 0x0` |
| `beq` at 0x58910 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x58920 | `[[sp-24]]:1 == 0xe` | `[[sp-24]]:1 != 0xe` |
| `beq` at 0x58930 | `[[sp-24]]:1 == 0x22` | `[[sp-24]]:1 != 0x22` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x58968 | `r0` | `?` |

## `powerNormalMargin`

- `powerNormalMargin`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5896c to 0x58a0c.
  Established. An included byte read addresses `[sp-24] +0xd` (`ldrb` at 0x589f8).
  The unnamed `0x58a0c` call carries `[sp-20]`, `[sp-24]`, unknown and `[[sp-24]]:1` (`bl` at 0x589e0).
  Equality `[[sp-24]]:1 == 0x22` takes `0x589d8`, while inequality falls through (`beq` at 0x589d0).
  Normal-margin field encoding and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5897c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x58980 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x58990 | store | 4 | `sp -0x1c` | `ret(0xa3c7c)` |
| `ldr` at 0x58994 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x589a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x589a4 | load | 4 | `ret(0xa3c7c) +0x24` | `-` |
| `ldr` at 0x589b4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x589b8 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x589c4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x589c8 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x589d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x589dc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x589ec | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x589f4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x589f8 | load | 1 | `[sp-24] +0xd` | `-` |
| `str` at 0x589fc | store | 4 | `sp -0x20` | `[[sp-24]+13]:1` |
| `ldr` at 0x58a00 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x58a08 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x58a08 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x58a08 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x58988 | `(unknown callee) 0xa3c7c` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x589e0 | `(unknown callee) 0x58a0c` | `[sp-20]` | `[sp-24]` | `?` | `[[sp-24]]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5899c | `ret(0xa3c7c) == 0x0` | `ret(0xa3c7c) != 0x0` |
| `beq` at 0x589b0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x589c0 | `[[sp-24]]:1 == 0xe` | `[[sp-24]]:1 != 0xe` |
| `beq` at 0x589d0 | `[[sp-24]]:1 == 0x22` | `[[sp-24]]:1 != 0x22` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x58a08 | `r0` | `?` |

## `powerHighMargin`

- `powerHighMargin`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x58a0c to 0x58a38.
  Established. A byte read addresses entry `r1 +0xe` (`ldrb` at 0x58a28).
  The integer return candidate is `[r1+14]:1` (`ldm` at 0x58a34).
  High-margin field encoding and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x58a1c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x58a20 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x58a24 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x58a28 | load | 1 | `r1 +0xe` | `-` |
| `ldm` at 0x58a34 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x58a34 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x58a34 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x58a34 | `r0` | `[r1+14]:1` |
