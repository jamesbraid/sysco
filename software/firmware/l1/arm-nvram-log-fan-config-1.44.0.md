# ARM NVRAM log and fan configuration paths in 1.44.0

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
Unknown data values and instruction effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/nvram-log-fan-config-facts.txt) retain all tool sections.

## `nvramRead`

- `nvramRead`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1ed7c to 0x1edf0.
  Established. The signed condition `(r0 + r2) <= 0x1fff` takes `0x1edbc`, while larger values fall
  through (`ble` at 0x1edac). The unnamed `0x19610` call has four unknown candidates (`bl` at 0x1edd4). The return reports `[sp-28]` (`ldm` at 0x1ede4). NVRAM range handling remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1ed8c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x1ed90 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x1ed94 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x1ed98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1ed9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1eda4 | load | 4 | `abs 0x1ede8` | `-` |
| `ldr` at 0x1edb0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1edb8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x1edbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1edc0 | load | 4 | `abs 0x1edec` | `-` |
| `ldr` at 0x1edc8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1edd0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1edd8 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x1ede4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1ede4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1ede4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1edd4 | `(unknown callee) 0x19610` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x1edac | `(r0 + r2) <= 0x1fff (signed, 32-bit)` | `(r0 + r2) > 0x1fff (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1ede4 | `r0` | `[sp-28]` |

## `nvramWrite`

- `nvramWrite`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1edf0 to 0x1ee64.
  Established. The signed condition `(r0 + r2) <= 0x1fff` takes `0x1ee30`, while larger values fall
  through (`ble` at 0x1ee20). The word at `sp -0x1c` receives an unknown value on an included path
  (`str` at 0x1ee2c). The unnamed `0x19610` call has four unknown candidates (`bl` at 0x1ee48).
  NVRAM write effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1ee00 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x1ee04 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x1ee08 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x1ee0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1ee10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1ee18 | load | 4 | `abs 0x1ee5c` | `-` |
| `ldr` at 0x1ee24 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1ee2c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x1ee30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1ee34 | load | 4 | `abs 0x1ee60` | `-` |
| `ldr` at 0x1ee40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1ee44 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1ee4c | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x1ee58 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1ee58 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1ee58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1ee48 | `(unknown callee) 0x19610` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x1ee20 | `(r0 + r2) <= 0x1fff (signed, 32-bit)` | `(r0 + r2) > 0x1fff (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1ee58 | `r0` | `[sp-28]` |

## `nvramLogReset`

- `nvramLogReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1e40c to 0x1e4ac.
  Established. Included word assignments through `[0x12eedc] +0x78c` and `+0x790` write
  `[[0x12eedc]+1924]` (`str` at 0x1e42c, `str` at 0x1e444). The byte through `[0x12eedc] +0x7a3`
  receives `(ret(0x1ffa8) & 0xff)` (`strb` at 0x1e46c). The unnamed `0x19610` call carries
  `[0x12eedc]+1956`, `[0x12eedc]+1920`, `0x24`, `[0x12eedc]+1920` (`bl` at 0x1e498). Log reset
  format remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x1e418 | load | 4 | `abs 0x1e4a4` | `-` |
| `ldr` at 0x1e41c | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e420 | load | 4 | `abs 0x1e4a4` | `-` |
| `ldr` at 0x1e424 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e428 | load | 4 | `[0x12eedc] +0x784` | `-` |
| `str` at 0x1e42c | store | 4 | `[0x12eedc] +0x78c` | `[[0x12eedc]+1924]` |
| `ldr` at 0x1e430 | load | 4 | `abs 0x1e4a4` | `-` |
| `ldr` at 0x1e434 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e438 | load | 4 | `abs 0x1e4a4` | `-` |
| `ldr` at 0x1e43c | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e440 | load | 4 | `[0x12eedc] +0x784` | `-` |
| `str` at 0x1e444 | store | 4 | `[0x12eedc] +0x790` | `[[0x12eedc]+1924]` |
| `ldr` at 0x1e448 | load | 4 | `abs 0x1e4a4` | `-` |
| `ldr` at 0x1e44c | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e450 | load | 4 | `abs 0x1e4a8` | `-` |
| `ldr` at 0x1e454 | load | 4 | `abs 0x1e4a4` | `-` |
| `ldr` at 0x1e458 | load | 4 | `abs 0x12eedc` | `-` |
| `strb` at 0x1e46c | store | 1 | `[0x12eedc] +0x7a3` | `(ret(0x1ffa8) & 0xff)` |
| `ldr` at 0x1e470 | load | 4 | `abs 0x1e4a4` | `-` |
| `ldr` at 0x1e474 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e480 | load | 4 | `abs 0x1e4a4` | `-` |
| `ldr` at 0x1e484 | load | 4 | `abs 0x12eedc` | `-` |
| `ldm` at 0x1e4a0 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x1e4a0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x1e4a0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1e4a0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1e4a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1e468 | `(unknown callee) 0x1ffa8` | `[0x12eedc]+1920` | `0x24` | `[0x12eedc]` | `[0x12eedc]+1920` |
| `bl` at 0x1e498 | `(unknown callee) 0x19610` | `[0x12eedc]+1956` | `[0x12eedc]+1920` | `0x24` | `[0x12eedc]+1920` |
| `bl` at 0x1e49c | `(unknown callee) 0x19da8` | `ret(0x19610)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1e4a0 | `r0` | `ret(0x19da8)` |

## `nvramLogInsert`

- `nvramLogInsert`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1e4ac to 0x1e958.
  Established. A nonzero absolute word `[0x12eedc]` takes `0x1e50c`, while zero falls through (`bne`
  at 0x1e504). The byte through `[[0x12eedc]+1936] +0x0` receives `0xa` on an included path (`strb`
  at 0x1e828). The byte through `[0x12eedc] +0x7a3` receives `(ret(0x1ffa8) & 0xff)` (`strb` at 0x1e890). Log insertion policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1e4bc | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x1e4cc | store | 4 | `sp -0x24` | `ret(0x194f0)` |
| `str` at 0x1e4d4 | store | 4 | `sp -0x28` | `sp-36` |
| `str` at 0x1e4dc | store | 4 | `sp -0x2c` | `0x5` |
| `ldr` at 0x1e4e0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x1e4ec | store | 4 | `sp -0x30` | `ret(0x19538)` |
| `strb` at 0x1e4f4 | store | 1 | `sp -0x31` | `0x0` |
| `ldr` at 0x1e4f8 | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e4fc | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e50c | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e510 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e514 | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e518 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e51c | load | 4 | `[0x12eedc] +0x788` | `-` |
| `ldr` at 0x1e520 | load | 4 | `[0x12eedc] +0x784` | `-` |
| `str` at 0x1e528 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x1e530 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1e534 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x1e540 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x1e550 | store | 1 | `sp -0x31` | `0x1` |
| `ldr` at 0x1e554 | load | 4 | `abs 0x1e940` | `-` |
| `ldr` at 0x1e56c | load | 4 | `abs 0x1e944` | `-` |
| `ldr` at 0x1e570 | load | 4 | `abs 0x12f3c0` | `-` |
| `ldr` at 0x1e580 | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e584 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e588 | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e58c | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e590 | load | 4 | `[0x12eedc] +0x790` | `-` |
| `ldr` at 0x1e594 | load | 4 | `[0x12eedc] +0x78c` | `-` |
| `ldr` at 0x1e5a0 | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e5a4 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e5a8 | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e5ac | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e5b0 | load | 4 | `[0x12eedc] +0x78c` | `-` |
| `ldr` at 0x1e5b4 | load | 4 | `[0x12eedc] +0x784` | `-` |
| `ldr` at 0x1e5bc | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e5c0 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e5c4 | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e5c8 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e5cc | load | 4 | `[0x12eedc] +0x788` | `-` |
| `ldr` at 0x1e5d0 | load | 4 | `[0x12eedc] +0x790` | `-` |
| `str` at 0x1e5dc | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x1e5e4 | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e5e8 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e5ec | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e5f0 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e5f4 | load | 4 | `[0x12eedc] +0x790` | `-` |
| `ldr` at 0x1e5f8 | load | 4 | `[0x12eedc] +0x78c` | `-` |
| `ldr` at 0x1e604 | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e608 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e60c | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e610 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e614 | load | 4 | `[0x12eedc] +0x78c` | `-` |
| `ldr` at 0x1e618 | load | 4 | `[0x12eedc] +0x790` | `-` |
| `str` at 0x1e620 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x1e624 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x1e628 | load | 4 | `sp -0x2c` | `-` |
| `ldrsb` at 0x1e630 | load | 1 | `sp -0x31` | `-` |
| `ldr` at 0x1e638 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x1e644 | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e648 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e654 | load | 4 | `[0x12eedc] +0x78c` | `-` |
| `ldrb` at 0x1e658 | load | 1 | `[[0x12eedc]+1932] +0x0` | `-` |
| `str` at 0x1e660 | store | 4 | `[0x12eedc] +0x78c` | `[[0x12eedc]+1932]+1` |
| `ldr` at 0x1e670 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x1e678 | store | 4 | `sp -0x38` | `[sp-56]+1` |
| `ldr` at 0x1e67c | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e680 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e684 | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e688 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e68c | load | 4 | `[0x12eedc] +0x78c` | `-` |
| `ldr` at 0x1e690 | load | 4 | `[0x12eedc] +0x788` | `-` |
| `ldr` at 0x1e69c | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e6a0 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e6a4 | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e6a8 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e6ac | load | 4 | `[0x12eedc] +0x784` | `-` |
| `str` at 0x1e6b0 | store | 4 | `[0x12eedc] +0x78c` | `[[0x12eedc]+1924]` |
| `ldr` at 0x1e6b8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x1e6c0 | store | 4 | `sp -0x38` | `[sp-56]+1` |
| `ldr` at 0x1e6c8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x1e6d0 | store | 4 | `sp -0x2c` | `[sp-44]-1` |
| `ldr` at 0x1e6dc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1e6e8 | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e6ec | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e6f4 | load | 4 | `[0x12eedc] +0x790` | `-` |
| `strb` at 0x1e700 | store | 1 | `[[0x12eedc]+1936] +0x0` | `?` |
| `str` at 0x1e708 | store | 4 | `[0x12eedc] +0x790` | `[[0x12eedc]+1936]+1` |
| `ldr` at 0x1e710 | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e714 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e71c | load | 4 | `[0x12eedc] +0x790` | `-` |
| `ldr` at 0x1e724 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x1e728 | load | 1 | `[sp-40] +0x0` | `-` |
| `str` at 0x1e730 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `strb` at 0x1e738 | store | 1 | `[[0x12eedc]+1936] +0x0` | `([[sp-40]]:1 & 0xff)` |
| `str` at 0x1e740 | store | 4 | `[0x12eedc] +0x790` | `[[0x12eedc]+1936]+1` |
| `ldr` at 0x1e744 | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e748 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e74c | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e750 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e754 | load | 4 | `[0x12eedc] +0x790` | `-` |
| `ldr` at 0x1e758 | load | 4 | `[0x12eedc] +0x788` | `-` |
| `ldr` at 0x1e764 | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e768 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e76c | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e770 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e774 | load | 4 | `[0x12eedc] +0x784` | `-` |
| `str` at 0x1e778 | store | 4 | `[0x12eedc] +0x790` | `[[0x12eedc]+1924]` |
| `ldr` at 0x1e780 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x1e788 | store | 4 | `sp -0x30` | `[sp-48]-1` |
| `ldr` at 0x1e794 | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e798 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e7a0 | load | 4 | `[0x12eedc] +0x790` | `-` |
| `ldr` at 0x1e7a8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x1e7ac | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0x1e7b4 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `strb` at 0x1e7bc | store | 1 | `[[0x12eedc]+1936] +0x0` | `([[sp-28]]:1 & 0xff)` |
| `str` at 0x1e7c4 | store | 4 | `[0x12eedc] +0x790` | `[[0x12eedc]+1936]+1` |
| `ldr` at 0x1e7c8 | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e7cc | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e7d0 | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e7d4 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e7d8 | load | 4 | `[0x12eedc] +0x790` | `-` |
| `ldr` at 0x1e7dc | load | 4 | `[0x12eedc] +0x788` | `-` |
| `ldr` at 0x1e7e8 | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e7ec | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e7f0 | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e7f4 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e7f8 | load | 4 | `[0x12eedc] +0x784` | `-` |
| `str` at 0x1e7fc | store | 4 | `[0x12eedc] +0x790` | `[[0x12eedc]+1924]` |
| `ldrsb` at 0x1e804 | load | 1 | `sp -0x31` | `-` |
| `ldr` at 0x1e810 | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e814 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e81c | load | 4 | `[0x12eedc] +0x790` | `-` |
| `strb` at 0x1e828 | store | 1 | `[[0x12eedc]+1936] +0x0` | `0xa` |
| `str` at 0x1e830 | store | 4 | `[0x12eedc] +0x790` | `[[0x12eedc]+1936]+1` |
| `ldr` at 0x1e834 | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e838 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e83c | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e840 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e844 | load | 4 | `[0x12eedc] +0x790` | `-` |
| `ldr` at 0x1e848 | load | 4 | `[0x12eedc] +0x788` | `-` |
| `ldr` at 0x1e854 | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e858 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e85c | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e860 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e864 | load | 4 | `[0x12eedc] +0x784` | `-` |
| `str` at 0x1e868 | store | 4 | `[0x12eedc] +0x790` | `[[0x12eedc]+1924]` |
| `ldr` at 0x1e86c | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e870 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e874 | load | 4 | `abs 0x1e948` | `-` |
| `ldr` at 0x1e878 | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e87c | load | 4 | `abs 0x12eedc` | `-` |
| `strb` at 0x1e890 | store | 1 | `[0x12eedc] +0x7a3` | `(ret(0x1ffa8) & 0xff)` |
| `ldr` at 0x1e894 | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e898 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e8a4 | load | 4 | `abs 0x1e93c` | `-` |
| `ldr` at 0x1e8a8 | load | 4 | `abs 0x12eedc` | `-` |
| `str` at 0x1e8c8 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x1e8cc | load | 4 | `abs 0x1e940` | `-` |
| `ldr` at 0x1e8e0 | load | 4 | `abs 0x1e944` | `-` |
| `ldr` at 0x1e8e4 | load | 4 | `abs 0x12f3c0` | `-` |
| `str` at 0x1e8f4 | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0x1e8f8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x1e8fc | store | 4 | `sp -0x20` | `[sp-60]` |
| `ldr` at 0x1e908 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x1e914 | load | 4 | `abs 0x1e94c` | `-` |
| `str` at 0x1e918 | store | 4 | `sp -0x44` | `0xeb988` |
| `str` at 0x1e91c | store | 4 | `sp -0x40` | `ret(0x1f8e0)` |
| `ldr` at 0x1e924 | load | 4 | `abs 0x1e950` | `-` |
| `ldr` at 0x1e928 | load | 4 | `abs 0x1e954` | `-` |
| `ldm` at 0x1e938 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x1e938 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x1e938 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1e938 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1e938 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1e4c4 | `(unknown callee) 0x194f0` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x1e4e4 | `(unknown callee) 0x19538` | `[sp-28]` | `?` | `?` | `0x5` |
| `bl` at 0x1e55c | `(unknown callee) 0x20cd0` | `0x12eeec` | `?` | `[sp-28]` | `?` |
| `bl` at 0x1e88c | `(unknown callee) 0x1ffa8` | `[0x12eedc]+1920` | `0x24` | `?` | `[0x12eedc]+1920` |
| `bl` at 0x1e8bc | `(unknown callee) 0x19610` | `[0x12eedc]+1956` | `[0x12eedc]+1920` | `0x24` | `[0x12eedc]+1920` |
| `bl` at 0x1e8c0 | `(unknown callee) 0x19da8` | `ret(0x19610)` | `?` | `?` | `?` |
| `bl` at 0x1e8d0 | `(unknown callee) 0x2105c` | `0x12eeec` | `?` | `?` | `0x0` |
| `bl` at 0x1e90c | `(unknown callee) 0x1f8e0` | `[sp-60]` | `?` | `?` | `[sp-60]` |
| `bl` at 0x1e930 | `(unknown callee) 0x200a0` | `0x9` | `0xeb690` | `0x715` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x1e504 | `[0x12eedc] != 0x0` | `[0x12eedc] == 0x0` |
| `beq` at 0x1e548 | `? == 0xa` | `? != 0xa` |
| `beq` at 0x1e568 | `ret(0x20cd0) == 0x0` | `ret(0x20cd0) != 0x0` |
| `beq` at 0x1e578 | `[0x12f3c0] == 0x1` | `[0x12f3c0] != 0x1` |
| `bls` at 0x1e59c | `[[0x12eedc]+1936] <= [[0x12eedc]+1932] (unsigned, 32-bit)` | `[[0x12eedc]+1936] > [[0x12eedc]+1932] (unsigned, 32-bit)` |
| `bhs` at 0x1e600 | `[[0x12eedc]+1936] >= [[0x12eedc]+1932] (unsigned, 32-bit)` | `[[0x12eedc]+1936] < [[0x12eedc]+1932] (unsigned, 32-bit)` |
| `blt` at 0x1e640 | `? < [sp-56] (signed, 32-bit)` | `? >= [sp-56] (signed, 32-bit)` |
| `beq` at 0x1e66c | `? == 0xa` | `? != 0xa` |
| `blo` at 0x1e698 | `[[0x12eedc]+1932] < [[0x12eedc]+1928] (unsigned, 32-bit)` | `[[0x12eedc]+1932] >= [[0x12eedc]+1928] (unsigned, 32-bit)` |
| `beq` at 0x1e6d8 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x1e6e4 | `[sp-44] != 0x4` | `[sp-44] == 0x4` |
| `blo` at 0x1e760 | `[[0x12eedc]+1936] < [[0x12eedc]+1928] (unsigned, 32-bit)` | `[[0x12eedc]+1936] >= [[0x12eedc]+1928] (unsigned, 32-bit)` |
| `beq` at 0x1e790 | `Z == 1` | `not (Z == 1)` |
| `blo` at 0x1e7e4 | `[[0x12eedc]+1936] < [[0x12eedc]+1928] (unsigned, 32-bit)` | `[[0x12eedc]+1936] >= [[0x12eedc]+1928] (unsigned, 32-bit)` |
| `beq` at 0x1e80c | `? == 0x0` | `? != 0x0` |
| `blo` at 0x1e850 | `[[0x12eedc]+1936] < [[0x12eedc]+1928] (unsigned, 32-bit)` | `[[0x12eedc]+1936] >= [[0x12eedc]+1928] (unsigned, 32-bit)` |
| `beq` at 0x1e8dc | `ret(0x2105c) == 0x0` | `ret(0x2105c) != 0x0` |
| `beq` at 0x1e8ec | `[0x12f3c0] == 0x1` | `[0x12f3c0] != 0x1` |
| `beq` at 0x1e904 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1e938 | `r0` | `?` |

## `nvramLogFinsert`

- `nvramLogFinsert`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1e958 to 0x1e9a0.
  Established. The word at `sp -0x24` receives `sp-12` (`str` at 0x1e970). The unnamed `0x19298`
  call carries `sp-156`, entry `r0`, `sp-12`, `sp-156` (`bl` at 0x1e984). The unnamed `0x1e4ac` call
  carries `sp-156`, unknown, unknown, `sp-156` (`bl` at 0x1e990). Formatted log handling remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1e970 | store | 4 | `sp -0x24` | `sp-12` |
| `ldr` at 0x1e97c | load | 4 | `sp -0x10` | `-` |
| `ldr` at 0x1e980 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x1e99c | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x1e99c | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x1e99c | load | 4 | `sp -0x18` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1e984 | `(unknown callee) 0x19298` | `sp-156` | `r0` | `sp-12` | `sp-156` |
| `bl` at 0x1e990 | `(unknown callee) 0x1e4ac` | `sp-156` | `?` | `?` | `sp-156` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1e99c | `r0` | `?` |

## `nvramLogFetch`

- `nvramLogFetch`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1e9a0 to 0x1ed7c.
  Established. Included word reads use `[0x12eedc] +0x790` and `+0x78c` (`ldr` at 0x1e9c8, `ldr` at 0x1e9cc). The byte through `ret(0x1943c) +0x0` receives zero on an included path (`strb` at 0x1eaa0). The unnamed `0x19454` call carries `([sp-20] + [sp-68])`, `0x13`, `0xeb9b0`,
  `ret(0x192f8)` (`bl` at 0x1ec88). Log extraction semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1e9b4 | store | 4 | `sp -0x44` | `0x1` |
| `ldr` at 0x1e9b8 | load | 4 | `abs 0x1ed74` | `-` |
| `ldr` at 0x1e9bc | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e9c0 | load | 4 | `abs 0x1ed74` | `-` |
| `ldr` at 0x1e9c4 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e9c8 | load | 4 | `[0x12eedc] +0x790` | `-` |
| `ldr` at 0x1e9cc | load | 4 | `[0x12eedc] +0x78c` | `-` |
| `ldr` at 0x1e9d8 | load | 4 | `abs 0x1ed74` | `-` |
| `ldr` at 0x1e9dc | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e9e0 | load | 4 | `[0x12eedc] +0x78c` | `-` |
| `str` at 0x1e9e4 | store | 4 | `sp -0x18` | `[[0x12eedc]+1932]` |
| `ldr` at 0x1e9e8 | load | 4 | `abs 0x1ed74` | `-` |
| `ldr` at 0x1e9ec | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1e9f0 | load | 4 | `[0x12eedc] +0x790` | `-` |
| `str` at 0x1e9f4 | store | 4 | `sp -0x1c` | `[[0x12eedc]+1936]` |
| `str` at 0x1e9fc | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x1ea04 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x1ea0c | load | 4 | `abs 0x1ed74` | `-` |
| `ldr` at 0x1ea10 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1ea14 | load | 4 | `abs 0x1ed74` | `-` |
| `ldr` at 0x1ea18 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1ea1c | load | 4 | `[0x12eedc] +0x790` | `-` |
| `ldr` at 0x1ea20 | load | 4 | `[0x12eedc] +0x78c` | `-` |
| `ldr` at 0x1ea2c | load | 4 | `abs 0x1ed74` | `-` |
| `ldr` at 0x1ea30 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1ea34 | load | 4 | `[0x12eedc] +0x78c` | `-` |
| `str` at 0x1ea38 | store | 4 | `sp -0x18` | `[[0x12eedc]+1932]` |
| `ldr` at 0x1ea3c | load | 4 | `abs 0x1ed74` | `-` |
| `ldr` at 0x1ea40 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1ea44 | load | 4 | `[0x12eedc] +0x788` | `-` |
| `str` at 0x1ea48 | store | 4 | `sp -0x1c` | `[[0x12eedc]+1928]` |
| `ldr` at 0x1ea4c | load | 4 | `abs 0x1ed74` | `-` |
| `ldr` at 0x1ea50 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1ea54 | load | 4 | `[0x12eedc] +0x784` | `-` |
| `str` at 0x1ea58 | store | 4 | `sp -0x20` | `[[0x12eedc]+1924]` |
| `ldr` at 0x1ea5c | load | 4 | `abs 0x1ed74` | `-` |
| `ldr` at 0x1ea60 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1ea64 | load | 4 | `[0x12eedc] +0x790` | `-` |
| `str` at 0x1ea68 | store | 4 | `sp -0x24` | `[[0x12eedc]+1936]` |
| `str` at 0x1ea7c | store | 4 | `sp -0x14` | `ret(0x1943c)` |
| `ldr` at 0x1ea80 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1ea90 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x1ea98 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x1eaa0 | store | 1 | `ret(0x1943c) +0x0` | `0x0` |
| `ldr` at 0x1eaa4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1eaa8 | store | 4 | `sp -0x48` | `[sp-20]` |
| `str` at 0x1eab4 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x1eab8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x1eabc | store | 4 | `sp -0x2c` | `[[0x12eedc]+1932]` |
| `ldr` at 0x1eac0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x1eac4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x1eac8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1ead4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x1ead8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x1eadc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x1eae0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x1eaec | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x1eaf8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x1eb00 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x1eb08 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x1eb0c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x1eb18 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x1eb20 | store | 4 | `sp -0x44` | `?` |
| `str` at 0x1eb28 | store | 4 | `sp -0x34` | `0x4` |
| `ldr` at 0x1eb30 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x1eb38 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x1eb3c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x1eb44 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x1eb4c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1eb50 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x1eb60 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x1eb64 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x1eb68 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1eb6c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x1eb70 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x1eb74 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x1eb78 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1eb7c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x1eb84 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x1eb90 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x1eb98 | store | 4 | `sp -0x4c` | `?` |
| `str` at 0x1eba4 | store | 4 | `sp -0x4c` | `0x64` |
| `ldr` at 0x1eba8 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x1ebac | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x1ebb0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x1ebbc | store | 4 | `sp -0x14` | `ret(0x1943c)` |
| `ldr` at 0x1ebc0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1ebd0 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0x1ebdc | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0x1ebe4 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x1ebe8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x1ebec | store | 4 | `sp -0x2c` | `[sp-24]` |
| `ldr` at 0x1ebf0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x1ebf4 | store | 4 | `sp -0x30` | `[sp-28]` |
| `ldr` at 0x1ebf8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1ec04 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x1ec08 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x1ec0c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x1ec10 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x1ec1c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x1ec2c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x1ec30 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x1ec38 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x1ec3c | store | 1 | `[sp-60] +0x0` | `?` |
| `str` at 0x1ec44 | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0x1ec48 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x1ec50 | store | 4 | `sp -0x34` | `[sp-52]-1` |
| `ldr` at 0x1ec6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1ec70 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x1ec80 | load | 4 | `abs 0x1ed78` | `-` |
| `ldr` at 0x1ec8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1ec90 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x1eca8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x1ecb0 | store | 4 | `sp -0x44` | `([sp-68] + ret(0x19538))` |
| `ldr` at 0x1ecb8 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x1ecbc | load | 1 | `(unknown)` | `-` |
| `str` at 0x1eccc | store | 4 | `sp -0x3c` | `sp-56` |
| `str` at 0x1ecd4 | store | 4 | `sp -0x34` | `0x4` |
| `ldr` at 0x1ece0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x1ece8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1ecf0 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x1ecf4 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x1ecf8 | store | 1 | `(unknown)` | `?` |
| `str` at 0x1ed00 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x1ed04 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x1ed0c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x1ed14 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1ed18 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x1ed28 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x1ed2c | store | 4 | `sp -0x2c` | `[sp-32]` |
| `ldr` at 0x1ed30 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1ed34 | store | 4 | `sp -0x30` | `[sp-36]` |
| `ldr` at 0x1ed38 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x1ed3c | store | 4 | `sp -0x2c` | `[sp-32]` |
| `ldr` at 0x1ed40 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1ed44 | store | 4 | `sp -0x30` | `[sp-36]` |
| `ldr` at 0x1ed4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1ed50 | load | 4 | `sp -0x44` | `-` |
| `strb` at 0x1ed5c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x1ed60 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1ed64 | store | 4 | `sp -0x48` | `[sp-20]` |
| `ldr` at 0x1ed68 | load | 4 | `sp -0x48` | `-` |
| `ldm` at 0x1ed70 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1ed70 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1ed70 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1ea74 | `(unknown callee) 0x1943c` | `0x1` | `r1` | `[[0x12eedc]+1936]` | `[[0x12eedc]+1932]` |
| `bl` at 0x1ebb4 | `(unknown callee) 0x1943c` | `?` | `r1` | `?` | `?` |
| `bl` at 0x1ec64 | `(unknown callee) 0x192f8` | `sp-56` | `[sp-60]` | `[sp-60]+1` | `sp-56` |
| `bl` at 0x1ec88 | `(unknown callee) 0x19454` | `([sp-20] + [sp-68])` | `0x13` | `0xeb9b0` | `ret(0x192f8)` |
| `bl` at 0x1ec9c | `(unknown callee) 0x19538` | `([sp-20] + [sp-68])` | `?` | `[sp-20]` | `([sp-20] + [sp-68])` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0x1e9d4 | `[[0x12eedc]+1936] <= [[0x12eedc]+1932] (unsigned, 32-bit)` | `[[0x12eedc]+1936] > [[0x12eedc]+1932] (unsigned, 32-bit)` |
| `bhs` at 0x1ea28 | `[[0x12eedc]+1936] >= [[0x12eedc]+1932] (unsigned, 32-bit)` | `[[0x12eedc]+1936] < [[0x12eedc]+1932] (unsigned, 32-bit)` |
| `bne` at 0x1ea88 | `ret(0x1943c) != 0x0` | `ret(0x1943c) == 0x0` |
| `beq` at 0x1ead0 | `Z == 1` | `not (Z == 1)` |
| `bhs` at 0x1eae8 | `C == 1` | `not (C == 1)` |
| `beq` at 0x1eaf4 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x1eb14 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x1eb58 | `Z == 0` | `not (Z == 0)` |
| `ble` at 0x1eb8c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x1ebc8 | `ret(0x1943c) != 0x0` | `ret(0x1943c) == 0x0` |
| `beq` at 0x1ec00 | `Z == 1` | `not (Z == 1)` |
| `bhs` at 0x1ec18 | `C == 1` | `not (C == 1)` |
| `beq` at 0x1ec24 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x1ec58 | `[sp-52]-1 != 0x0` | `[sp-52]-1 == 0x0` |
| `bne` at 0x1ecc4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x1ed20 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1ed70 | `r0` | `?` |

## `fanTachInit`

- `fanTachInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3ee60 to 0x3f100.
  Established. The absolute word at `0x132210` receives `0x1` (`str` at 0x3eee4). The word at
  `0x13220c` receives `ret(0x196d0)` (`str` at 0x3ef18). The byte through `[sp-36] +0x4` receives
  `([sp-32] & 0xff)` (`strb` at 0x3ef84). Tachometer initialization effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3ee70 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x3ee74 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x3ee78 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x3ee7c | load | 4 | `abs 0x3f0e0` | `-` |
| `ldr` at 0x3ee80 | load | 4 | `abs 0x13220c` | `-` |
| `ldr` at 0x3ee8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ee98 | load | 4 | `abs 0x3f0e4` | `-` |
| `str` at 0x3ee9c | store | 4 | `sp -0x38` | `0xf64f4` |
| `ldr` at 0x3eea0 | load | 4 | `abs 0x3f0e0` | `-` |
| `ldr` at 0x3eea4 | load | 4 | `abs 0x13220c` | `-` |
| `str` at 0x3eea8 | store | 4 | `sp -0x34` | `[0x13220c]` |
| `ldr` at 0x3eeac | load | 4 | `abs 0x3f0e8` | `-` |
| `ldr` at 0x3eeb0 | load | 4 | `abs 0x132210` | `-` |
| `str` at 0x3eeb4 | store | 4 | `sp -0x30` | `[0x132210]` |
| `ldr` at 0x3eebc | load | 4 | `abs 0x3f0ec` | `-` |
| `ldr` at 0x3eecc | load | 4 | `abs 0x3f0e8` | `-` |
| `ldr` at 0x3eed0 | load | 4 | `abs 0x132210` | `-` |
| `ldr` at 0x3eedc | load | 4 | `abs 0x3f0e8` | `-` |
| `str` at 0x3eee4 | store | 4 | `abs 0x132210` | `0x1` |
| `ldr` at 0x3eee8 | load | 4 | `abs 0x3f0e8` | `-` |
| `ldr` at 0x3eeec | load | 4 | `abs 0x132210` | `-` |
| `str` at 0x3ef00 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x3ef04 | load | 4 | `abs 0x3f0e0` | `-` |
| `ldr` at 0x3ef08 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3ef18 | store | 4 | `abs 0x13220c` | `ret(0x196d0)` |
| `ldr` at 0x3ef1c | load | 4 | `abs 0x13220c` | `-` |
| `ldr` at 0x3ef28 | load | 4 | `abs 0x3f0f0` | `-` |
| `str` at 0x3ef2c | store | 4 | `sp -0x38` | `0xf653c` |
| `ldr` at 0x3ef30 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3ef34 | store | 4 | `sp -0x34` | `[sp-44]` |
| `ldr` at 0x3ef3c | load | 4 | `abs 0x3f0ec` | `-` |
| `ldr` at 0x3ef4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ef60 | load | 4 | `abs 0x3f0e0` | `-` |
| `ldr` at 0x3ef64 | load | 4 | `abs 0x13220c` | `-` |
| `str` at 0x3ef6c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x3ef70 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3ef74 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3ef78 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0x3ef7c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3ef80 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x3ef84 | store | 1 | `[sp-36] +0x4` | `([sp-32] & 0xff)` |
| `ldr` at 0x3ef88 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3ef8c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x3ef90 | load | 1 | `[sp-36] +0x4` | `-` |
| `str` at 0x3efa4 | store | 4 | `[sp-36] +0x8` | `ret(0x196d0)` |
| `ldr` at 0x3efa8 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0x3efb4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3efbc | load | 4 | `abs 0x3f0f4` | `-` |
| `ldr` at 0x3efc0 | load | 4 | `abs 0x3f0f8` | `-` |
| `str` at 0x3effc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x3f000 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f008 | load | 4 | `abs 0x3f0f4` | `-` |
| `ldr` at 0x3f00c | load | 4 | `abs 0x3f0f8` | `-` |
| `ldr` at 0x3f018 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3f028 | load | 4 | `abs 0x3f0fc` | `-` |
| `str` at 0x3f02c | store | 4 | `(unknown)` | `0xf6584` |
| `str` at 0x3f030 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0x3f034 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3f038 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0x3f040 | load | 4 | `abs 0x3f0ec` | `-` |
| `ldr` at 0x3f050 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3f054 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x3f058 | load | 1 | `[sp-36] +0x4` | `-` |
| `ldr` at 0x3f05c | load | 4 | `[sp-36] +0x8` | `-` |
| `str` at 0x3f070 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x3f074 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x3f078 | load | 1 | `[sp-36] +0x4` | `-` |
| `ldr` at 0x3f07c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3f088 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3f08c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3f0a0 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldrb` at 0x3f0a8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3f0b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3f0bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f0c8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3f0d0 | store | 4 | `sp -0x28` | `?` |
| `ldm` at 0x3f0dc | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x3f0dc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3f0dc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3f0dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3eec8 | `(unknown callee) 0x200a0` | `0x9` | `0xf6528` | `0x4f` | `0x0` |
| `bl` at 0x3ef10 | `(unknown callee) 0x196d0` | `[sp-44]` | `0x1` | `[0x132210]` | `?` |
| `bl` at 0x3ef48 | `(unknown callee) 0x200a0` | `0x9` | `0xf6528` | `0x5b` | `0x0` |
| `bl` at 0x3ef9c | `(unknown callee) 0x196d0` | `[[sp-36]+4]:1` | `0x1` | `[sp-36]` | `[[sp-36]+4]:1` |
| `bl` at 0x3efc4 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0xf6578` | `0xf6580` |
| `bl` at 0x3efd0 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x3f010 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0xf6578` | `0xf6580` |
| `bl` at 0x3f020 | `(unknown callee) 0x19808` | `[sp-44]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x3f04c | `(unknown callee) 0x200a0` | `0x9` | `0xf6528` | `0x66` | `0x0` |
| `bl` at 0x3f068 | `(unknown callee) 0x19748` | `[[sp-36]+8]` | `0x1` | `[[sp-36]+4]:1` | `[[sp-36]+4]:1` |
| `bl` at 0x3f0c4 | `(unknown callee) 0x3f22c` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3ee88 | `[0x13220c] == 0x0` | `[0x13220c] != 0x0` |
| `bne` at 0x3ee94 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0x3eed8 | `[0x132210] != 0x0` | `[0x132210] == 0x0` |
| `bne` at 0x3ef24 | `[0x13220c] != 0x0` | `[0x13220c] == 0x0` |
| `bne` at 0x3efb0 | `[[sp-36]+8] != 0x0` | `[[sp-36]+8] == 0x0` |
| `ble` at 0x3f084 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x3f0b0 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3f0dc | `r0` | `?` |

## `fanCtrlInit`

- `fanCtrlInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3f100 to 0x3f22c.
  Established. The absolute word at `0x132218` receives `0x1` (`str` at 0x3f184). The word at
  `0x132214` receives `ret(0x196d0)` (`str` at 0x3f1ac). The byte through `[sp-36] +0x4` receives
  `([sp-32] & 0xff)` (`strb` at 0x3f20c). Fan control table meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3f110 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x3f114 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x3f118 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x3f11c | load | 4 | `abs 0x3f218` | `-` |
| `ldr` at 0x3f120 | load | 4 | `abs 0x132214` | `-` |
| `ldr` at 0x3f12c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f138 | load | 4 | `abs 0x3f21c` | `-` |
| `str` at 0x3f13c | store | 4 | `sp -0x38` | `0xf65c4` |
| `ldr` at 0x3f140 | load | 4 | `abs 0x3f218` | `-` |
| `ldr` at 0x3f144 | load | 4 | `abs 0x132214` | `-` |
| `str` at 0x3f148 | store | 4 | `sp -0x34` | `[0x132214]` |
| `ldr` at 0x3f14c | load | 4 | `abs 0x3f220` | `-` |
| `ldr` at 0x3f150 | load | 4 | `abs 0x132218` | `-` |
| `str` at 0x3f154 | store | 4 | `sp -0x30` | `[0x132218]` |
| `ldr` at 0x3f15c | load | 4 | `abs 0x3f224` | `-` |
| `ldr` at 0x3f16c | load | 4 | `abs 0x3f220` | `-` |
| `ldr` at 0x3f170 | load | 4 | `abs 0x132218` | `-` |
| `ldr` at 0x3f17c | load | 4 | `abs 0x3f220` | `-` |
| `str` at 0x3f184 | store | 4 | `abs 0x132218` | `0x1` |
| `ldr` at 0x3f188 | load | 4 | `abs 0x3f220` | `-` |
| `ldr` at 0x3f18c | load | 4 | `abs 0x132218` | `-` |
| `str` at 0x3f194 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x3f198 | load | 4 | `abs 0x3f218` | `-` |
| `ldr` at 0x3f19c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3f1ac | store | 4 | `abs 0x132214` | `ret(0x196d0)` |
| `ldr` at 0x3f1b0 | load | 4 | `abs 0x132214` | `-` |
| `ldr` at 0x3f1bc | load | 4 | `abs 0x3f228` | `-` |
| `str` at 0x3f1c0 | store | 4 | `sp -0x38` | `0xf65f8` |
| `ldr` at 0x3f1c4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3f1c8 | store | 4 | `sp -0x34` | `[sp-44]` |
| `ldr` at 0x3f1d0 | load | 4 | `abs 0x3f224` | `-` |
| `ldr` at 0x3f1e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f1e8 | load | 4 | `abs 0x3f218` | `-` |
| `ldr` at 0x3f1ec | load | 4 | `abs 0x132214` | `-` |
| `str` at 0x3f1f4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x3f1f8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3f1fc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3f200 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0x3f204 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3f208 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x3f20c | store | 1 | `[sp-36] +0x4` | `([sp-32] & 0xff)` |
| `ldm` at 0x3f214 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x3f214 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3f214 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3f214 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3f168 | `(unknown callee) 0x200a0` | `0x9` | `0xf6528` | `0x81` | `0x0` |
| `bl` at 0x3f1a4 | `(unknown callee) 0x196d0` | `[sp-44]` | `0x1` | `0x132218` | `?` |
| `bl` at 0x3f1dc | `(unknown callee) 0x200a0` | `0x9` | `0xf6528` | `0x8d` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3f128 | `[0x132214] == 0x0` | `[0x132214] != 0x0` |
| `bne` at 0x3f134 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0x3f178 | `[0x132218] != 0x0` | `[0x132218] == 0x0` |
| `bne` at 0x3f1b8 | `[0x132214] != 0x0` | `[0x132214] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3f214 | `r0` | `?` |

## `fanTachNotPresent`

- `fanTachNotPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3f22c to 0x3f290.
  Established. The unnamed `0x40fb4` call carries entry `r0`, `(r1 & 0xff)`, entry `r2`, `(r1 & 0xff)` (`bl` at 0x3f254). A nonzero result takes `0x3f270`, while zero falls through (`bne` at 0x3f268). The byte at `([sp-21]:1 + [ret(0x40fb4)+8]) +0x0` receives zero on an included path
  (`strb` at 0x3f284). Absent-tachometer state interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3f23c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x3f244 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x3f248 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3f24c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3f25c | store | 4 | `sp -0x1c` | `ret(0x40fb4)` |
| `ldr` at 0x3f260 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f270 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f274 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3f278 | load | 4 | `ret(0x40fb4) +0x8` | `-` |
| `strb` at 0x3f284 | store | 1 | `([sp-21]:1 + [ret(0x40fb4)+8]) +0x0` | `0x0` |
| `ldm` at 0x3f28c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3f28c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3f28c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3f254 | `(unknown callee) 0x40fb4` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3f268 | `ret(0x40fb4) != 0x0` | `ret(0x40fb4) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3f28c | `r0` | `ret(0x40fb4)` |

## `fanTachPresent`

- `fanTachPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3f290 to 0x3f2f4.
  Established. The word through `ret(0x40fb4) +0x8` is read (`ldr` at 0x3f2dc). The byte at
  `([sp-21]:1 + [ret(0x40fb4)+8]) +0x0` receives zero on an included path (`strb` at 0x3f2e8). The
  return reports `ret(0x40fb4)` (`ldm` at 0x3f2f0). Present-tachometer state meaning remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3f2a0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x3f2a8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x3f2ac | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3f2b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3f2c0 | store | 4 | `sp -0x1c` | `ret(0x40fb4)` |
| `ldr` at 0x3f2c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f2d4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f2d8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3f2dc | load | 4 | `ret(0x40fb4) +0x8` | `-` |
| `strb` at 0x3f2e8 | store | 1 | `([sp-21]:1 + [ret(0x40fb4)+8]) +0x0` | `0x0` |
| `ldm` at 0x3f2f0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3f2f0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3f2f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3f2b8 | `(unknown callee) 0x40fb4` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3f2cc | `ret(0x40fb4) != 0x0` | `ret(0x40fb4) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3f2f0 | `r0` | `ret(0x40fb4)` |

## `fanTachIsPresent`

- `fanTachIsPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3f2f4 to 0x3f364.
  Established. A nonzero `ret(0x40fb4)` takes `0x3f340`, while zero falls through (`bne` at 0x3f330). The byte at `([sp-21]:1 + [ret(0x40fb4)+8]) +0x0` is read on an included path (`ldrb` at 0x3f350). The word at `sp -0x20` receives `[([sp-21]:1 + [ret(0x40fb4)+8])]:1` (`str` at 0x3f354).
  Tachometer presence classification remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3f304 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x3f30c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x3f310 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3f314 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3f324 | store | 4 | `sp -0x1c` | `ret(0x40fb4)` |
| `ldr` at 0x3f328 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3f338 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x3f340 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f344 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3f348 | load | 4 | `ret(0x40fb4) +0x8` | `-` |
| `ldrb` at 0x3f350 | load | 1 | `([sp-21]:1 + [ret(0x40fb4)+8]) +0x0` | `-` |
| `str` at 0x3f354 | store | 4 | `sp -0x20` | `[([sp-21]:1 + [ret(0x40fb4)+8])]:1` |
| `ldr` at 0x3f358 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x3f360 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3f360 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3f360 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3f31c | `(unknown callee) 0x40fb4` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3f330 | `ret(0x40fb4) != 0x0` | `ret(0x40fb4) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3f360 | `r0` | `?` |

## `fanGetTachCfg`

- `fanGetTachCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3fa14 to 0x3fa84.
  Established. A zero entry `r1` takes `0x3fa6c`, while nonzero falls through (`beq` at 0x3fa58).
  The word through entry `r1 +0x0` receives `[[sp-28]]` on an included path (`str` at 0x3fa68). The
  return reports `[[sp-28]+4]:1` (`ldm` at 0x3fa7c). Tachometer configuration layout remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3fa24 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3fa28 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x3fa2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3fa40 | load | 4 | `abs 0x3fa80` | `-` |
| `ldr` at 0x3fa44 | load | 4 | `abs 0x13220c` | `-` |
| `str` at 0x3fa4c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x3fa50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3fa5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3fa60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fa64 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x3fa68 | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0x3fa6c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fa70 | load | 1 | `[sp-28] +0x4` | `-` |
| `ldm` at 0x3fa7c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3fa7c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3fa7c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3fa58 | `r1 == 0x0` | `r1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3fa7c | `r0` | `[[sp-28]+4]:1` |

## `fanLocation`

- `fanLocation`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x40cd8 to 0x40d44.
  Established. The unnamed `0x40d44` call carries entry `r0`, `(r1 & 0xff)`, entry `r2`, `(r1 & 0xff)` (`bl` at 0x40d00). A nonzero `[ret(0x40d44)+4]` takes `0x40d30`, while zero falls through
  (`bne` at 0x40d24). The word at `sp -0x20` receives `0xf6750` on an included path (`str` at 0x40d2c). Fan location labels remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x40ce8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x40cf0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x40cf4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x40cf8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x40d08 | store | 4 | `sp -0x1c` | `ret(0x40d44)` |
| `ldr` at 0x40d0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x40d10 | load | 4 | `ret(0x40d44) +0x4` | `-` |
| `str` at 0x40d14 | store | 4 | `sp -0x20` | `[ret(0x40d44)+4]` |
| `ldr` at 0x40d18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x40d1c | load | 4 | `ret(0x40d44) +0x4` | `-` |
| `ldr` at 0x40d28 | load | 4 | `abs 0x40d40` | `-` |
| `str` at 0x40d2c | store | 4 | `sp -0x20` | `0xf6750` |
| `ldr` at 0x40d30 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x40d3c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x40d3c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x40d3c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x40d00 | `(unknown callee) 0x40d44` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x40d24 | `[ret(0x40d44)+4] != 0x0` | `[ret(0x40d44)+4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x40d3c | `r0` | `?` |

## `i2cMuxLastChannel`

- `i2cMuxLastChannel`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc92f8 to 0xc9318.
  Established. The absolute byte at `0x12da25` is read (`ldrb` at 0xc9308). The return reports
  `[0x12da25]:1` (`ldm` at 0xc9310). Last-channel interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xc9304 | load | 4 | `abs 0xc9314` | `-` |
| `ldrb` at 0xc9308 | load | 1 | `abs 0x12da25` | `-` |
| `ldm` at 0xc9310 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc9310 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc9310 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc9310 | `r0` | `[0x12da25]:1` |

## `powerGetCfg`

- `powerGetCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x364c4 to 0x36528.
  Established. The absolute word at `0x13219c` is read (`ldr` at 0x364f4). The word through entry
  `r1 +0x0` receives `[[sp-28]]` (`str` at 0x3650c). The return reports `[[sp-28]+8]` (`ldm` at 0x36520). Power configuration meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x364d4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x364d8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x364dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x364f0 | load | 4 | `abs 0x36524` | `-` |
| `ldr` at 0x364f4 | load | 4 | `abs 0x13219c` | `-` |
| `str` at 0x364fc | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x36500 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x36504 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x36508 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x3650c | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0x36510 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x36514 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldm` at 0x36520 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x36520 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x36520 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x36520 | `r0` | `[[sp-28]+8]` |
