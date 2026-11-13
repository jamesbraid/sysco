# ARM expander interrupt and control paths in 1.44.0

Read from `6.5.30/usr/cpu/firmware/sysco/l1.bin`, package `6.5.30:l1#4` (version 1.44.0),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53d154`.
Carrier SHA-256: `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
Package SHA-256: `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
Inner executable SHA-256: `8d492b1d00a2fa93aebb48890caf8cacc5a9747e79209e93b841378aabd75162`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1166540 bytes.
Segment SHA-256: `7f2caa3b9c656f2cc1caf5bdaf139038c25b4a58ac8ff8a26ea044818245a46d`. Analysed routine spans use the exact
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
The computed transfer in ioExpInt remains unresolved; its tables cover
known edges only. Each affected entry identifies its computed transfer.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/expander-interrupt-control-facts.txt) retain all tool sections.

## `ioExpClearBit`

- `ioExpClearBit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca538 to 0xca584.
  Established. The unnamed `0xca584` call carries entry `r0`, `(r1 & 0xff)`, `(r2 & 0xff)`, `0x0`
  (`bl` at 0xca570). A byte at `sp -0x16` receives `(r2 & 0xff)` (`strb` at 0xca558). The included
  return record carries `ret(0xca584)` in `r0` (`ldm` at 0xca580). Bit-clear effect remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca548 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xca550 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xca558 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xca55c | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xca560 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xca564 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xca580 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xca580 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xca580 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xca570 | `(unknown callee) 0xca584` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xca580 | `r0` | `ret(0xca584)` |

## `ioExpIsPresent`

- `ioExpIsPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca484 to 0xca4e8.
  Established. A nonzero `ret(0xcb2c0)` takes `0xca4d0`, while zero falls through (`bne` at 0xca4c0). The byte through `ret(0xcb2c0) +0x0` is read on the included path (`ldrb` at 0xca4d4). A
  word at `sp -0x20` receives zero in the other arm (`str` at 0xca4c8). Presence query policy
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca494 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xca49c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xca4a0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xca4a4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xca4b4 | store | 4 | `sp -0x1c` | `ret(0xcb2c0)` |
| `ldr` at 0xca4b8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xca4c8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xca4d0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xca4d4 | load | 1 | `ret(0xcb2c0) +0x0` | `-` |
| `str` at 0xca4d8 | store | 4 | `sp -0x20` | `[ret(0xcb2c0)]:1` |
| `ldr` at 0xca4dc | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xca4e4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xca4e4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xca4e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xca4ac | `(unknown callee) 0xcb2c0` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xca4c0 | `ret(0xcb2c0) != 0x0` | `ret(0xcb2c0) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xca4e4 | `r0` | `[sp-32]` |

## `ioExpCount`

- `ioExpCount`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca434 to 0xca484.
  Established. The absolute word at `0x29733c` is read (`ldr` at 0xca460). The word through `[sp-24] +0x4` is read (`ldr` at 0xca470). The included return record carries `[[sp-24]+4]` in `r0` (`ldm` at 0xca47c). Expander count representation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca444 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xca448 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca45c | load | 4 | `abs 0xca480` | `-` |
| `ldr` at 0xca460 | load | 4 | `abs 0x29733c` | `-` |
| `str` at 0xca468 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xca46c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca470 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldm` at 0xca47c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xca47c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xca47c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xca47c | `r0` | `[[sp-24]+4]` |

## `ioExpSetBit`

- `ioExpSetBit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca4e8 to 0xca538.
  Established. The unnamed `0xca584` call carries entry `r0`, `(r1 & 0xff)`, `(r2 & 0xff)`, `(r2 & 0xff)` (`bl` at 0xca524). The byte at `sp -0x15` receives `(r1 & 0xff)` (`strb` at 0xca500); its
  included return record gives `ret(0xca584)` in integer `r0` (`ldm` at 0xca534). Bit-set semantics
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca4f8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xca500 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xca508 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xca50c | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xca510 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0xca514 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xca518 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xca534 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xca534 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xca534 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xca524 | `(unknown callee) 0xca584` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `(r2 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xca534 | `r0` | `ret(0xca584)` |

## `ioExpName`

- `ioExpName`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcb12c to 0xcb198.
  Established. A nonzero `[ret(0xcb198)+12]` takes `0xcb184`, while zero falls through (`bne` at 0xcb178). The word at `sp -0x20` receives `0x120738` on the included fallback path (`str` at 0xcb180). The unnamed `0xcb198` call carries entry `r0`, `(r1 & 0xff)`, entry `r2`, `(r1 & 0xff)`
  (`bl` at 0xcb154). Expander label selection remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcb13c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xcb144 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xcb148 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcb14c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcb15c | store | 4 | `sp -0x1c` | `ret(0xcb198)` |
| `ldr` at 0xcb160 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcb164 | load | 4 | `ret(0xcb198) +0xc` | `-` |
| `str` at 0xcb168 | store | 4 | `sp -0x20` | `[ret(0xcb198)+12]` |
| `ldr` at 0xcb16c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcb170 | load | 4 | `ret(0xcb198) +0xc` | `-` |
| `ldr` at 0xcb17c | load | 4 | `abs 0xcb194` | `-` |
| `str` at 0xcb180 | store | 4 | `sp -0x20` | `0x120738` |
| `ldr` at 0xcb184 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xcb190 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcb190 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcb190 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcb154 | `(unknown callee) 0xcb198` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcb178 | `[ret(0xcb198)+12] != 0x0` | `[ret(0xcb198)+12] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcb190 | `r0` | `?` |

## `ioExpGetCfg`

- `ioExpGetCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca3d0 to 0xca434.
  Established. The word through entry `r1 +0x0` receives `[[sp-28]]` (`str` at 0xca418). The
  absolute word at `0x29733c` is read (`ldr` at 0xca400). The included return record gives
  `[[sp-28]+4]` in `r0` (`ldm` at 0xca42c). Configuration accessor meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca3e0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xca3e4 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xca3e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca3fc | load | 4 | `abs 0xca430` | `-` |
| `ldr` at 0xca400 | load | 4 | `abs 0x29733c` | `-` |
| `str` at 0xca408 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xca40c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca410 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xca414 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xca418 | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0xca41c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xca420 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldm` at 0xca42c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xca42c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xca42c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xca42c | `r0` | `[[sp-28]+4]` |

## `ioExpGetBit`

- `ioExpGetBit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca688 to 0xca70c.
  Established. The unnamed `0xca70c` call carries entry `r0`, `(r1 & 0xff)`, `sp-29`, `(r1 & 0xff)`
  (`bl` at 0xca6bc). A zero result takes `0xca6e0`, while nonzero falls through (`beq` at 0xca6d0).
  An included word assignment at `sp -0x24` writes `ret(0xca70c)` (`str` at 0xca6d8). Bit-test
  interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca698 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xca6a0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xca6a8 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xca6ac | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xca6b4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xca6c4 | store | 4 | `sp -0x1c` | `ret(0xca70c)` |
| `ldr` at 0xca6c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xca6d4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xca6d8 | store | 4 | `sp -0x24` | `ret(0xca70c)` |
| `ldrb` at 0xca6e0 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0xca6e4 | load | 1 | `sp -0x16` | `-` |
| `str` at 0xca6fc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xca700 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xca708 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xca708 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xca708 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xca6bc | `(unknown callee) 0xca70c` | `r0` | `(r1 & 0xff)` | `sp-29` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xca6d0 | `ret(0xca70c) == 0x0` | `ret(0xca70c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xca708 | `r0` | `?` |

## `ioExpIntMask`

- `ioExpIntMask`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca830 to 0xca878.
  Established. The byte through `ret(0xcb2c0) +0x3` is read (`ldrb` at 0xca868). The unnamed
  `0xcb2c0` call carries entry `r0`, `(r1 & 0xff)`, entry `r2`, `(r1 & 0xff)` (`bl` at 0xca858). The
  included return record gives `[ret(0xcb2c0)+3]:1` in `r0` (`ldm` at 0xca874). Interrupt mask
  semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca840 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xca848 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xca84c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xca850 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xca860 | store | 4 | `sp -0x1c` | `ret(0xcb2c0)` |
| `ldr` at 0xca864 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xca868 | load | 1 | `ret(0xcb2c0) +0x3` | `-` |
| `ldm` at 0xca874 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xca874 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xca874 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xca858 | `(unknown callee) 0xcb2c0` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xca874 | `r0` | `[ret(0xcb2c0)+3]:1` |

## `ioExpIntCount`

- `ioExpIntCount`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca7e8 to 0xca830.
  Established. The word through `ret(0xcb2c0) +0x8` is read (`ldr` at 0xca820). The byte at `sp -0x15` receives `(r1 & 0xff)` (`strb` at 0xca800). The included return record gives
  `[ret(0xcb2c0)+8]` in `r0` (`ldm` at 0xca82c). Interrupt count interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca7f8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xca800 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xca804 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xca808 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xca818 | store | 4 | `sp -0x1c` | `ret(0xcb2c0)` |
| `ldr` at 0xca81c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xca820 | load | 4 | `ret(0xcb2c0) +0x8` | `-` |
| `ldm` at 0xca82c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xca82c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xca82c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xca810 | `(unknown callee) 0xcb2c0` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xca82c | `r0` | `[ret(0xcb2c0)+8]` |

## `ioExpIntDisable`

- `ioExpIntDisable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca878 to 0xca8c4.
  Established. The unnamed `0xcbab0` call carries entry `r0`, `(r1 & 0xff)`, `(r2 & 0xff)`, `0x0`
  (`bl` at 0xca8b0); its included return record carries `ret(0xcbab0)` in integer `r0` (`ldm` at 0xca8c0). The byte at `sp -0x16` receives `(r2 & 0xff)` (`strb` at 0xca898). Interrupt disable
  behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca888 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xca890 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xca898 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xca89c | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xca8a0 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xca8a4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xca8c0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xca8c0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xca8c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xca8b0 | `(unknown callee) 0xcbab0` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xca8c0 | `r0` | `ret(0xcbab0)` |

## `ioExpIntEnable`

- `ioExpIntEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca8c4 to 0xca910.
  Established. The unnamed `0xcbab0` call carries entry `r0`, `(r1 & 0xff)`, `(r2 & 0xff)`, `0x1`
  (`bl` at 0xca8fc). The byte at `sp -0x16` receives `(r2 & 0xff)` (`strb` at 0xca8e4). The included
  return record carries `ret(0xcbab0)` in integer `r0` (`ldm` at 0xca90c). Interrupt enable effects
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca8d4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xca8dc | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xca8e4 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xca8e8 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xca8ec | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xca8f0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xca90c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xca90c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xca90c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xca8fc | `(unknown callee) 0xcbab0` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xca90c | `r0` | `ret(0xcbab0)` |

## `ioExpIntSet`

- `ioExpIntSet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca910 to 0xca960.
  Established. The unnamed `0xcbab0` call carries entry `r0`, `(r1 & 0xff)`, `(r2 & 0xff)`, entry
  `r3` (`bl` at 0xca94c). The word at `sp -0x1c` receives entry `r3` (`str` at 0xca924). The
  included return record gives `ret(0xcbab0)` in `r0` (`ldm` at 0xca95c). Interrupt setting policy
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca920 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xca924 | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0xca92c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xca934 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xca938 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xca93c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xca940 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca948 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xca95c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xca95c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xca95c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xca94c | `(unknown callee) 0xcbab0` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xca95c | `r0` | `ret(0xcbab0)` |

## `ioExpInt`

- `ioExpInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca960 to 0xcab10.
  Established. A zero `[[sp-28]+8]` takes `0xcaae4`, while nonzero falls through (`beq` at 0xca9e0).
  The byte at `sp -0x21` receives `([sp-33]:1+1 & 0xff)` on an included path (`strb` at 0xcaaec).
  The unnamed `0x200a0` call carries `0x1`, `0x12047c`, `0x2d3`, `0x0` (`bl` at 0xcaae0). Expander
  interrupt dispatch remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca970 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xca978 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xca980 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0xca988 | store | 4 | `sp -0x28` | `0x0` |
| `ldrb` at 0xca98c | load | 1 | `sp -0x15` | `-` |
| `strb` at 0xca990 | store | 1 | `sp -0x21` | `((r1 & 0xff) & 0xff)` |
| `ldrb` at 0xca994 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0xca998 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0xca9a4 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xca9a8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xca9b8 | store | 4 | `sp -0x1c` | `ret(0xcb198)` |
| `ldrb` at 0xca9bc | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xca9c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xca9d0 | store | 4 | `sp -0x20` | `ret(0xcb2c0)` |
| `ldr` at 0xca9d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xca9d8 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xca9e4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xca9e8 | load | 1 | `ret(0xcb2c0) +0x0` | `-` |
| `ldr` at 0xca9f4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xca9f8 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0xcaa00 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcaabc | load | 4 | `abs 0xcab04` | `-` |
| `str` at 0xcaac0 | store | 4 | `sp -0x30` | `0x120618` |
| `ldr` at 0xcaac4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcaac8 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xcaacc | store | 4 | `sp -0x2c` | `[[sp-28]+12]` |
| `ldr` at 0xcaad4 | load | 4 | `abs 0xcab08` | `-` |
| `ldr` at 0xcaad8 | load | 4 | `abs 0xcab0c` | `-` |
| `ldrb` at 0xcaae4 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0xcaaec | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldr` at 0xcaaf4 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xcab00 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcab00 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcab00 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xca9b0 | `(unknown callee) 0xcb198` | `?` | `?` | `?` | `?` |
| `bl` at 0xca9c8 | `(unknown callee) 0xcb2c0` | `[sp-20]` | `[sp-33]:1` | `?` | `[sp-33]:1` |
| `bl` at 0xcaae0 | `(unknown callee) 0x200a0` | `0x1` | `0x12047c` | `0x2d3` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhi` at 0xca9a0 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0xca9e0 | `[[sp-28]+8] == 0x0` | `[[sp-28]+8] != 0x0` |
| `beq` at 0xca9f0 | `[ret(0xcb2c0)]:1 == 0x0` | `[ret(0xcb2c0)]:1 != 0x0` |
| `ldrls` at 0xcaa00 | `[[sp-28]]:1 <= 0x4 (unsigned, 32-bit)` | `[[sp-28]]:1 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcab00 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xcaa00).

## `ioExpIgnoreInt`

- `ioExpIgnoreInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcb034 to 0xcb12c.
  Established. The unnamed `0x48da8` call at `0xcb074` carries entry `r0`, `0x0`, `0x1204d0`,
  `0x1204d8` (`bl` at 0xcb074). An included word assignment at an unknown target writes `0x1206f8`
  (`str` at 0xcb0dc). The final unnamed `0x200a0` call carries `0x6`, `0x12047c`, `0x337`, `0x0`
  (`bl` at 0xcb10c). Ignored-interrupt reporting remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcb044 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xcb050 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xcb058 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0xcb060 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldr` at 0xcb064 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcb06c | load | 4 | `abs 0xcb118` | `-` |
| `ldr` at 0xcb070 | load | 4 | `abs 0xcb11c` | `-` |
| `str` at 0xcb0ac | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xcb0b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcb0b8 | load | 4 | `abs 0xcb118` | `-` |
| `ldr` at 0xcb0bc | load | 4 | `abs 0xcb11c` | `-` |
| `ldr` at 0xcb0c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcb0d8 | load | 4 | `abs 0xcb120` | `-` |
| `str` at 0xcb0dc | store | 4 | `(unknown)` | `0x1206f8` |
| `str` at 0xcb0e0 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldrb` at 0xcb0e4 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xcb0e8 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldrb` at 0xcb0ec | load | 1 | `sp -0x16` | `-` |
| `str` at 0xcb0f0 | store | 4 | `(unknown)` | `[sp-22]:1` |
| `ldrb` at 0xcb0f4 | load | 1 | `sp -0x17` | `-` |
| `str` at 0xcb0f8 | store | 4 | `(unknown)` | `[sp-23]:1` |
| `ldr` at 0xcb100 | load | 4 | `abs 0xcb124` | `-` |
| `ldr` at 0xcb104 | load | 4 | `abs 0xcb128` | `-` |
| `ldm` at 0xcb114 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcb114 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcb114 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcb074 | `(unknown callee) 0x48da8` | `r0` | `0x0` | `0x1204d0` | `0x1204d8` |
| `bl` at 0xcb080 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xcb0c0 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0x1204d0` | `0x1204d8` |
| `bl` at 0xcb0d0 | `(unknown callee) 0x19808` | `[sp-28]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xcb10c | `(unknown callee) 0x200a0` | `0x6` | `0x12047c` | `0x337` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcb114 | `r0` | `ret(0x200a0)` |

## `ioExpPortInt`

- `ioExpPortInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcab10 to 0xcad04.
  Established. The inequality `[[sp-28]+1]:1 != [sp-21]:1` takes `0xcac20`, while equality falls
  through (`bne` at 0xcabcc). The unnamed `0xcad04` call carries `[sp-20]`, `[sp-28]`,
  `ret(0xcb2c0)`, `[sp-33]:1` (`bl` at 0xcac10). The word at `sp -0x44` receives `[sp-24]:1` before
  that included call (`str` at 0xcac00). Port interrupt matching remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcab20 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xcab28 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xcab30 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xcab38 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0xcab40 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `strb` at 0xcab48 | store | 1 | `sp -0x18` | `([sp] & 0xff)` |
| `ldr` at 0xcab4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcab60 | load | 4 | `abs 0xcacf0` | `-` |
| `ldr` at 0xcab64 | load | 4 | `abs 0x29733c` | `-` |
| `str` at 0xcab6c | store | 4 | `sp -0x28` | `?` |
| `strb` at 0xcab74 | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0xcab78 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xcab7c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcab80 | load | 4 | `[sp-40] +0x4` | `-` |
| `ldrb` at 0xcab8c | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xcab90 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcaba0 | store | 4 | `sp -0x1c` | `ret(0xcb198)` |
| `ldrb` at 0xcaba4 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xcaba8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcabb8 | store | 4 | `sp -0x20` | `ret(0xcb2c0)` |
| `ldr` at 0xcabbc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcabc0 | load | 1 | `[sp-28] +0x1` | `-` |
| `ldrb` at 0xcabc4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcabd0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcabd4 | load | 1 | `[sp-28] +0x2` | `-` |
| `ldrb` at 0xcabd8 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xcabe4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcabe8 | load | 1 | `[sp-28] +0x3` | `-` |
| `ldrb` at 0xcabec | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0xcabf8 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0xcabfc | load | 1 | `sp -0x18` | `-` |
| `str` at 0xcac00 | store | 4 | `sp -0x44` | `[sp-24]:1` |
| `ldr` at 0xcac04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcac08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcac0c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcac18 | store | 4 | `sp -0x30` | `ret(0xcad04)` |
| `ldrb` at 0xcac20 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0xcac28 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldr` at 0xcac30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcac38 | load | 4 | `abs 0xcacf4` | `-` |
| `ldr` at 0xcac3c | load | 4 | `abs 0xcacf8` | `-` |
| `str` at 0xcac78 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xcac7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcac84 | load | 4 | `abs 0xcacf4` | `-` |
| `ldr` at 0xcac88 | load | 4 | `abs 0xcacf8` | `-` |
| `ldr` at 0xcac94 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcaca4 | load | 4 | `abs 0xcacfc` | `-` |
| `str` at 0xcaca8 | store | 4 | `(unknown)` | `0x120644` |
| `str` at 0xcacac | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldrb` at 0xcacb0 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xcacb4 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldrb` at 0xcacb8 | load | 1 | `sp -0x16` | `-` |
| `str` at 0xcacbc | store | 4 | `(unknown)` | `[sp-22]:1` |
| `ldrb` at 0xcacc0 | load | 1 | `sp -0x17` | `-` |
| `str` at 0xcacc4 | store | 4 | `(unknown)` | `[sp-23]:1` |
| `ldr` at 0xcaccc | load | 4 | `abs 0xcad00` | `-` |
| `str` at 0xcace0 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xcace4 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0xcacec | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcacec | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcacec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcab98 | `(unknown callee) 0xcb198` | `?` | `?` | `[[sp-40]+4]` | `?` |
| `bl` at 0xcabb0 | `(unknown callee) 0xcb2c0` | `[sp-20]` | `[sp-33]:1` | `?` | `[sp-33]:1` |
| `bl` at 0xcac10 | `(unknown callee) 0xcad04` | `[sp-20]` | `[sp-28]` | `ret(0xcb2c0)` | `[sp-33]:1` |
| `bl` at 0xcac40 | `(unknown callee) 0x48da8` | `?` | `0x0` | `0x1204d0` | `0x1204d8` |
| `bl` at 0xcac4c | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xcac8c | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0x1204d0` | `0x1204d8` |
| `bl` at 0xcac9c | `(unknown callee) 0x19808` | `[sp-44]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xcacd8 | `(unknown callee) 0x200a0` | `0x1` | `0x12047c` | `0x2f4` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xcab88 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xcabcc | `[[sp-28]+1]:1 != [sp-21]:1` | `[[sp-28]+1]:1 == [sp-21]:1` |
| `bne` at 0xcabe0 | `[[sp-28]+2]:1 != [sp-22]:1` | `[[sp-28]+2]:1 == [sp-22]:1` |
| `bne` at 0xcabf4 | `[[sp-28]+3]:1 != [sp-23]:1` | `[[sp-28]+3]:1 == [sp-23]:1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcacec | `r0` | `?` |
