# ARM GPIO and expander paths in 1.44.0

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
Computed transfers in ioExpReset and ioExpPreConfig remain unresolved. Their tables cover
known edges only. Each affected entry identifies its computed transfer.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/gpio-expander-facts.txt) retain all tool sections.

## `l1ioGpioInit`

- `l1ioGpioInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbc854 to 0xbc8d8.
  Established. The absolute word at `0x12d918` receives `ret(0x1967c)` (`str` at 0xbc87c). The
  unnamed `0x1967c` call carries `0x11f1d8`, `0x2`, entry `r2`, entry `r3` (`bl` at 0xbc870). A
  nonnegative signed word at `0x12d918` takes `0xbc8c0`, while negative values fall through (`bge` at 0xbc888). GPIO initialization effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbc864 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xbc868 | load | 4 | `abs 0xbc8c8` | `-` |
| `ldr` at 0xbc878 | load | 4 | `abs 0xbc8cc` | `-` |
| `str` at 0xbc87c | store | 4 | `abs 0x12d918` | `ret(0x1967c)` |
| `ldr` at 0xbc880 | load | 4 | `abs 0x12d918` | `-` |
| `ldr` at 0xbc894 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xbc8a0 | load | 4 | `abs 0xbc8d0` | `-` |
| `str` at 0xbc8a4 | store | 4 | `sp -0x1c` | `0x11f1e8` |
| `str` at 0xbc8a8 | store | 4 | `sp -0x18` | `ret(0x193ac)` |
| `ldr` at 0xbc8b0 | load | 4 | `abs 0xbc8d4` | `-` |
| `ldm` at 0xbc8c4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbc8c4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbc8c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbc870 | `(unknown callee) 0x1967c` | `0x11f1d8` | `0x2` | `r2` | `r3` |
| `bl` at 0xbc88c | `(unknown callee) 0x193d0` | `ret(0x1967c)` | `?` | `ret(0x1967c)` | `[0x12d918]` |
| `bl` at 0xbc898 | `(unknown callee) 0x193ac` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |
| `bl` at 0xbc8bc | `(unknown callee) 0x200a0` | `0x1` | `0x11f200` | `0x34` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xbc888 | `[0x12d918] >= 0x0 (signed, 32-bit)` | `[0x12d918] < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbc8c4 | `r0` | `?` |

## `l1ioGpioReset`

- `l1ioGpioReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbc8d8 to 0xbcac4.
  Established. The unnamed `0x19718` call at `0xbc938` carries `[0x12d918]`, `0x40046706`, `sp-28`,
  `0x12d918` (`bl` at 0xbc938). An included byte assignment at `sp -0x1b` writes zero (`strb` at 0xbc924). A later byte assignment there writes `([[sp-24]+6]:1 & 0xff)` (`strb` at 0xbc960). GPIO
  reset sequencing remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbc8e8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbc8ec | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xbc8f4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xbc8f8 | load | 4 | `abs 0xbcaa8` | `-` |
| `ldr` at 0xbc8fc | load | 4 | `abs 0x12d918` | `-` |
| `ldr` at 0xbc908 | load | 4 | `abs 0xbcaac` | `-` |
| `str` at 0xbc90c | store | 4 | `sp -0x24` | `0xfffff3e4` |
| `ldr` at 0xbc914 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xbc918 | load | 1 | `r1 +0x3` | `-` |
| `strb` at 0xbc91c | store | 1 | `sp -0x1c` | `([r1+3]:1 & 0xff)` |
| `strb` at 0xbc924 | store | 1 | `sp -0x1b` | `0x0` |
| `ldr` at 0xbc928 | load | 4 | `abs 0xbcaa8` | `-` |
| `ldr` at 0xbc930 | load | 4 | `abs 0x12d918` | `-` |
| `ldr` at 0xbc934 | load | 4 | `abs 0xbcab0` | `-` |
| `ldr` at 0xbc950 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `str` at 0xbc954 | store | 4 | `sp -0x20` | `[ret(0x193d0)]` |
| `ldr` at 0xbc958 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xbc95c | load | 1 | `[sp-24] +0x6` | `-` |
| `strb` at 0xbc960 | store | 1 | `sp -0x1b` | `([[sp-24]+6]:1 & 0xff)` |
| `ldr` at 0xbc964 | load | 4 | `abs 0xbcaa8` | `-` |
| `ldr` at 0xbc96c | load | 4 | `abs 0x12d918` | `-` |
| `ldr` at 0xbc970 | load | 4 | `abs 0xbcab4` | `-` |
| `ldr` at 0xbc984 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbc998 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `str` at 0xbc99c | store | 4 | `sp -0x20` | `[ret(0x193d0)]` |
| `ldr` at 0xbc9a0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xbc9a4 | load | 1 | `[sp-24] +0x5` | `-` |
| `strb` at 0xbc9ac | store | 1 | `sp -0x1b` | `?` |
| `ldr` at 0xbc9b0 | load | 4 | `abs 0xbcaa8` | `-` |
| `ldr` at 0xbc9b8 | load | 4 | `abs 0x12d918` | `-` |
| `ldr` at 0xbc9bc | load | 4 | `abs 0xbcab8` | `-` |
| `ldr` at 0xbc9d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbc9e4 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `str` at 0xbc9e8 | store | 4 | `sp -0x20` | `[ret(0x193d0)]` |
| `ldr` at 0xbc9ec | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xbc9f0 | load | 1 | `[sp-24] +0x4` | `-` |
| `strb` at 0xbc9f8 | store | 1 | `sp -0x1b` | `?` |
| `ldr` at 0xbc9fc | load | 4 | `abs 0xbcaa8` | `-` |
| `ldr` at 0xbca04 | load | 4 | `abs 0x12d918` | `-` |
| `ldr` at 0xbca08 | load | 4 | `abs 0xbcabc` | `-` |
| `ldr` at 0xbca1c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbca30 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `str` at 0xbca34 | store | 4 | `sp -0x20` | `[ret(0x193d0)]` |
| `ldr` at 0xbca38 | load | 4 | `abs 0xbcaa8` | `-` |
| `ldr` at 0xbca40 | load | 4 | `abs 0x12d918` | `-` |
| `ldr` at 0xbca44 | load | 4 | `abs 0xbcac0` | `-` |
| `ldr` at 0xbca58 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbca6c | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `str` at 0xbca70 | store | 4 | `sp -0x20` | `[ret(0x193d0)]` |
| `ldr` at 0xbca74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbca80 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbca8c | store | 4 | `sp -0x24` | `ret(0xbcf18)` |
| `str` at 0xbca98 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xbca9c | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xbcaa4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbcaa4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbcaa4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbc938 | `(unknown callee) 0x19718` | `[0x12d918]` | `0x40046706` | `sp-28` | `0x12d918` |
| `bl` at 0xbc948 | `(unknown callee) 0x193d0` | `ret(0x19718)` | `?` | `?` | `ret(0x19718)` |
| `bl` at 0xbc974 | `(unknown callee) 0x19718` | `[0x12d918]` | `0x40046700` | `sp-28` | `0x12d918` |
| `bl` at 0xbc990 | `(unknown callee) 0x193d0` | `ret(0x19718)` | `?` | `?` | `[sp-32]` |
| `bl` at 0xbc9c0 | `(unknown callee) 0x19718` | `[0x12d918]` | `0x40046704` | `sp-28` | `0x12d918` |
| `bl` at 0xbc9dc | `(unknown callee) 0x193d0` | `ret(0x19718)` | `?` | `?` | `[sp-32]` |
| `bl` at 0xbca0c | `(unknown callee) 0x19718` | `[0x12d918]` | `0x40046702` | `sp-28` | `0x12d918` |
| `bl` at 0xbca28 | `(unknown callee) 0x193d0` | `ret(0x19718)` | `?` | `?` | `[sp-32]` |
| `bl` at 0xbca48 | `(unknown callee) 0x19718` | `[0x12d918]` | `0x80046708` | `sp-28` | `0x12d918` |
| `bl` at 0xbca64 | `(unknown callee) 0x193d0` | `ret(0x19718)` | `?` | `?` | `[sp-32]` |
| `bl` at 0xbca84 | `(unknown callee) 0xbcf18` | `[sp-32]` | `?` | `?` | `[sp-32]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xbc904 | `[0x12d918] >= 0x0 (signed, 32-bit)` | `[0x12d918] < 0x0 (signed, 32-bit)` |
| `beq` at 0xbc944 | `ret(0x19718) == 0x0` | `ret(0x19718) != 0x0` |
| `beq` at 0xbc980 | `ret(0x19718) == 0x0` | `ret(0x19718) != 0x0` |
| `bne` at 0xbc98c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xbc9cc | `ret(0x19718) == 0x0` | `ret(0x19718) != 0x0` |
| `bne` at 0xbc9d8 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xbca18 | `ret(0x19718) == 0x0` | `ret(0x19718) != 0x0` |
| `bne` at 0xbca24 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xbca54 | `ret(0x19718) == 0x0` | `ret(0x19718) != 0x0` |
| `bne` at 0xbca60 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xbca7c | `[sp-32] == 0x0` | `[sp-32] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbcaa4 | `r0` | `?` |

## `l1ioGpioRead`

- `l1ioGpioRead`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbcb70 to 0xbcc1c.
  Established. The unnamed `0x19718` call carries `[0x12d918]`, `0xc0046701`, `sp-32`, `0x12d918`
  (`bl` at 0xbcbc4); its zero result takes `0xbcbf0`, while nonzero falls through (`beq` at 0xbcbd0). An included byte assignment through `[sp-28] +0x0` writes `([sp-31]:1 & 0xff)` (`strb` at 0xbcbf8). GPIO read protocol remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbcb80 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbcb84 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xbcb88 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xbcb8c | load | 4 | `abs 0xbcc10` | `-` |
| `ldr` at 0xbcb90 | load | 4 | `abs 0x12d918` | `-` |
| `ldr` at 0xbcb9c | load | 4 | `abs 0xbcc14` | `-` |
| `str` at 0xbcba0 | store | 4 | `sp -0x24` | `0xfffff3e4` |
| `ldr` at 0xbcba8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xbcbac | load | 1 | `r1 +0x3` | `-` |
| `strb` at 0xbcbb0 | store | 1 | `sp -0x20` | `([r1+3]:1 & 0xff)` |
| `ldr` at 0xbcbb4 | load | 4 | `abs 0xbcc10` | `-` |
| `ldr` at 0xbcbbc | load | 4 | `abs 0x12d918` | `-` |
| `ldr` at 0xbcbc0 | load | 4 | `abs 0xbcc18` | `-` |
| `ldr` at 0xbcbdc | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `str` at 0xbcbe8 | store | 4 | `sp -0x24` | `ret(0xbcf18)` |
| `ldr` at 0xbcbf0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbcbf4 | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0xbcbf8 | store | 1 | `[sp-28] +0x0` | `([sp-31]:1 & 0xff)` |
| `str` at 0xbcc00 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xbcc04 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xbcc0c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbcc0c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbcc0c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbcbc4 | `(unknown callee) 0x19718` | `[0x12d918]` | `0xc0046701` | `sp-32` | `0x12d918` |
| `bl` at 0xbcbd4 | `(unknown callee) 0x193d0` | `ret(0x19718)` | `?` | `?` | `ret(0x19718)` |
| `bl` at 0xbcbe0 | `(unknown callee) 0xbcf18` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xbcb98 | `[0x12d918] >= 0x0 (signed, 32-bit)` | `[0x12d918] < 0x0 (signed, 32-bit)` |
| `beq` at 0xbcbd0 | `ret(0x19718) == 0x0` | `ret(0x19718) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbcc0c | `r0` | `?` |

## `l1ioGpioWrite`

- `l1ioGpioWrite`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbcac4 to 0xbcb70.
  Established. The unnamed `0x19718` call carries `[0x12d918]`, `0x40046700`, `sp-32`, `0x12d918`
  (`bl` at 0xbcb24). The byte through entry `r1 +0x3` is read (`ldrb` at 0xbcb04). An included byte
  assignment at `sp -0x1f` writes `((r2 & 0xff) & 0xff)` (`strb` at 0xbcb10). GPIO write behavior
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbcad4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbcad8 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xbcae0 | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `ldr` at 0xbcae4 | load | 4 | `abs 0xbcb64` | `-` |
| `ldr` at 0xbcae8 | load | 4 | `abs 0x12d918` | `-` |
| `ldr` at 0xbcaf4 | load | 4 | `abs 0xbcb68` | `-` |
| `str` at 0xbcaf8 | store | 4 | `sp -0x24` | `0xfffff3e4` |
| `ldr` at 0xbcb00 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xbcb04 | load | 1 | `r1 +0x3` | `-` |
| `strb` at 0xbcb08 | store | 1 | `sp -0x20` | `([r1+3]:1 & 0xff)` |
| `ldrb` at 0xbcb0c | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xbcb10 | store | 1 | `sp -0x1f` | `((r2 & 0xff) & 0xff)` |
| `ldr` at 0xbcb14 | load | 4 | `abs 0xbcb64` | `-` |
| `ldr` at 0xbcb1c | load | 4 | `abs 0x12d918` | `-` |
| `ldr` at 0xbcb20 | load | 4 | `abs 0xbcb6c` | `-` |
| `ldr` at 0xbcb3c | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `str` at 0xbcb48 | store | 4 | `sp -0x24` | `ret(0xbcf18)` |
| `str` at 0xbcb54 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xbcb58 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xbcb60 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbcb60 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbcb60 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbcb24 | `(unknown callee) 0x19718` | `[0x12d918]` | `0x40046700` | `sp-32` | `0x12d918` |
| `bl` at 0xbcb34 | `(unknown callee) 0x193d0` | `ret(0x19718)` | `?` | `?` | `ret(0x19718)` |
| `bl` at 0xbcb40 | `(unknown callee) 0xbcf18` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xbcaf0 | `[0x12d918] >= 0x0 (signed, 32-bit)` | `[0x12d918] < 0x0 (signed, 32-bit)` |
| `beq` at 0xbcb30 | `ret(0x19718) == 0x0` | `ret(0x19718) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbcb60 | `r0` | `?` |

## `l1ioGpioIntSet`

- `l1ioGpioIntSet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbcc1c to 0xbccc8.
  Established. The unnamed `0x19718` call carries `[0x12d918]`, `0x40046706`, `sp-32`, `0x12d918`
  (`bl` at 0xbcc7c). The byte at `sp -0x20` receives `([r1+3]:1 & 0xff)` (`strb` at 0xbcc60). A zero
  call result takes `0xbcca8`, while nonzero falls through (`beq` at 0xbcc88). Interrupt
  configuration semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbcc2c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbcc30 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xbcc38 | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `ldr` at 0xbcc3c | load | 4 | `abs 0xbccbc` | `-` |
| `ldr` at 0xbcc40 | load | 4 | `abs 0x12d918` | `-` |
| `ldr` at 0xbcc4c | load | 4 | `abs 0xbccc0` | `-` |
| `str` at 0xbcc50 | store | 4 | `sp -0x24` | `0xfffff3e4` |
| `ldr` at 0xbcc58 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xbcc5c | load | 1 | `r1 +0x3` | `-` |
| `strb` at 0xbcc60 | store | 1 | `sp -0x20` | `([r1+3]:1 & 0xff)` |
| `ldrb` at 0xbcc64 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xbcc68 | store | 1 | `sp -0x1f` | `((r2 & 0xff) & 0xff)` |
| `ldr` at 0xbcc6c | load | 4 | `abs 0xbccbc` | `-` |
| `ldr` at 0xbcc74 | load | 4 | `abs 0x12d918` | `-` |
| `ldr` at 0xbcc78 | load | 4 | `abs 0xbccc4` | `-` |
| `ldr` at 0xbcc94 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `str` at 0xbcca0 | store | 4 | `sp -0x24` | `ret(0xbcf18)` |
| `str` at 0xbccac | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xbccb0 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xbccb8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbccb8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbccb8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbcc7c | `(unknown callee) 0x19718` | `[0x12d918]` | `0x40046706` | `sp-32` | `0x12d918` |
| `bl` at 0xbcc8c | `(unknown callee) 0x193d0` | `ret(0x19718)` | `?` | `?` | `ret(0x19718)` |
| `bl` at 0xbcc98 | `(unknown callee) 0xbcf18` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xbcc48 | `[0x12d918] >= 0x0 (signed, 32-bit)` | `[0x12d918] < 0x0 (signed, 32-bit)` |
| `beq` at 0xbcc88 | `ret(0x19718) == 0x0` | `ret(0x19718) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbccb8 | `r0` | `?` |

## `l1ioGpioIntGet`

- `l1ioGpioIntGet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbccc8 to 0xbcd74.
  Established. The unnamed `0x19718` call carries `[0x12d918]`, `0xc0046707`, `sp-32`, `0x12d918`
  (`bl` at 0xbcd1c). An included byte assignment through `[sp-28] +0x0` writes `([sp-31]:1 & 0xff)`
  (`strb` at 0xbcd50). A word at `sp -0x24` receives `0xfffff3e4` in the included fallback arm
  (`str` at 0xbccf8). Interrupt read interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbccd8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbccdc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xbcce0 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xbcce4 | load | 4 | `abs 0xbcd68` | `-` |
| `ldr` at 0xbcce8 | load | 4 | `abs 0x12d918` | `-` |
| `ldr` at 0xbccf4 | load | 4 | `abs 0xbcd6c` | `-` |
| `str` at 0xbccf8 | store | 4 | `sp -0x24` | `0xfffff3e4` |
| `ldr` at 0xbcd00 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xbcd04 | load | 1 | `r1 +0x3` | `-` |
| `strb` at 0xbcd08 | store | 1 | `sp -0x20` | `([r1+3]:1 & 0xff)` |
| `ldr` at 0xbcd0c | load | 4 | `abs 0xbcd68` | `-` |
| `ldr` at 0xbcd14 | load | 4 | `abs 0x12d918` | `-` |
| `ldr` at 0xbcd18 | load | 4 | `abs 0xbcd70` | `-` |
| `ldr` at 0xbcd34 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `str` at 0xbcd40 | store | 4 | `sp -0x24` | `ret(0xbcf18)` |
| `ldr` at 0xbcd48 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbcd4c | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0xbcd50 | store | 1 | `[sp-28] +0x0` | `([sp-31]:1 & 0xff)` |
| `str` at 0xbcd58 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xbcd5c | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xbcd64 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbcd64 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbcd64 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbcd1c | `(unknown callee) 0x19718` | `[0x12d918]` | `0xc0046707` | `sp-32` | `0x12d918` |
| `bl` at 0xbcd2c | `(unknown callee) 0x193d0` | `ret(0x19718)` | `?` | `?` | `ret(0x19718)` |
| `bl` at 0xbcd38 | `(unknown callee) 0xbcf18` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xbccf0 | `[0x12d918] >= 0x0 (signed, 32-bit)` | `[0x12d918] < 0x0 (signed, 32-bit)` |
| `beq` at 0xbcd28 | `ret(0x19718) == 0x0` | `ret(0x19718) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbcd64 | `r0` | `?` |

## `l1ioGpioInt`

- `l1ioGpioInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbcd74 to 0xbcf18.
  Established. The unnamed `0x19718` call carries `[0x12d918]`, `0x80046708`, `sp-36`, `0x12d918`
  (`bl` at 0xbcde8). The unnamed `0xcad04` call carries `0x0`, `[sp-24]`, `[sp-28]`, `[sp-29]:1`
  (`bl` at 0xbce88). An included word assignment at `sp -0x28` writes `([sp-40] + ret(0xcad04))`
  (`str` at 0xbce98). GPIO interrupt processing remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbcd84 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbcd88 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xbcd8c | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0xbcd90 | store | 1 | `sp -0x1d` | `(r3 & 0xff)` |
| `str` at 0xbcd98 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xbcd9c | load | 4 | `abs 0xbcf00` | `-` |
| `ldr` at 0xbcda0 | load | 4 | `abs 0x12d918` | `-` |
| `ldr` at 0xbcdac | load | 4 | `abs 0xbcf04` | `-` |
| `str` at 0xbcdb0 | store | 4 | `sp -0x30` | `0xfffff3e4` |
| `ldr` at 0xbcdb8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xbcdbc | load | 1 | `r1 +0x3` | `-` |
| `strb` at 0xbcdc0 | store | 1 | `sp -0x24` | `([r1+3]:1 & 0xff)` |
| `str` at 0xbcdc8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xbcdcc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbcdd8 | load | 4 | `abs 0xbcf00` | `-` |
| `ldr` at 0xbcde0 | load | 4 | `abs 0x12d918` | `-` |
| `ldr` at 0xbcde4 | load | 4 | `abs 0xbcf08` | `-` |
| `ldr` at 0xbce00 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xbce18 | load | 4 | `abs 0xbcf0c` | `-` |
| `str` at 0xbce1c | store | 4 | `sp -0x3c` | `0x11f20c` |
| `ldr` at 0xbce20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbce24 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xbce28 | store | 4 | `sp -0x38` | `[[sp-24]+12]` |
| `str` at 0xbce2c | store | 4 | `sp -0x34` | `ret(0x1fc1c)` |
| `ldr` at 0xbce34 | load | 4 | `abs 0xbcf10` | `-` |
| `ldr` at 0xbce44 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xbce48 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xbce50 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbce54 | load | 1 | `sp -0x23` | `-` |
| `ldrb` at 0xbce58 | load | 1 | `[sp-28] +0x3` | `-` |
| `ldrb` at 0xbce6c | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0xbce70 | load | 1 | `sp -0x23` | `-` |
| `str` at 0xbce74 | store | 4 | `sp -0x3c` | `[sp-35]:1` |
| `ldr` at 0xbce7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbce80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbce90 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xbce98 | store | 4 | `sp -0x28` | `([sp-40] + ret(0xcad04))` |
| `ldr` at 0xbcea0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xbcea4 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xbceac | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbceb4 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xbcebc | load | 4 | `abs 0xbcf14` | `-` |
| `str` at 0xbcec0 | store | 4 | `sp -0x3c` | `0x11f238` |
| `ldr` at 0xbcec4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbcec8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbcecc | store | 4 | `sp -0x38` | `?` |
| `ldrb` at 0xbced0 | load | 1 | `sp -0x23` | `-` |
| `str` at 0xbced4 | store | 4 | `sp -0x34` | `[sp-35]:1` |
| `ldr` at 0xbcedc | load | 4 | `abs 0xbcf10` | `-` |
| `ldr` at 0xbceec | load | 4 | `sp -0x28` | `-` |
| `str` at 0xbcef0 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xbcef4 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0xbcefc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbcefc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbcefc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbcde8 | `(unknown callee) 0x19718` | `[0x12d918]` | `0x80046708` | `sp-36` | `0x12d918` |
| `bl` at 0xbcdf8 | `(unknown callee) 0x193d0` | `ret(0x19718)` | `?` | `?` | `ret(0x19718)` |
| `bl` at 0xbce04 | `(unknown callee) 0xbcf18` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |
| `bl` at 0xbce10 | `(unknown callee) 0x1fc1c` | `ret(0xbcf18)` | `?` | `?` | `ret(0xbcf18)` |
| `bl` at 0xbce40 | `(unknown callee) 0x200a0` | `0x3` | `0x11f200` | `0xda` | `0x0` |
| `bl` at 0xbce88 | `(unknown callee) 0xcad04` | `0x0` | `[sp-24]` | `[sp-28]` | `[sp-29]:1` |
| `bl` at 0xbcee8 | `(unknown callee) 0x200a0` | `0x3` | `0x11f200` | `0xe5` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xbcda8 | `[0x12d918] >= 0x0 (signed, 32-bit)` | `[0x12d918] < 0x0 (signed, 32-bit)` |
| `bgt` at 0xbcdd4 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0xbcdf4 | `ret(0x19718) == 0x0` | `ret(0x19718) != 0x0` |
| `beq` at 0xbce68 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbcefc | `r0` | `?` |

## `ioExpInit`

- `ioExpInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc9658 to 0xc990c.
  Established. The absolute word at `0x297340` receives `0x1` (`str` at 0xc96dc). The word at
  `0x29733c` receives `ret(0x196d0)` (`str` at 0xc9710). The unnamed `0x196d0` call at `0xc9790`
  carries `[[sp-36]+4]`, `0xc`, `[sp-36]`, `[sp-36]` (`bl` at 0xc9790). Expander initialization
  roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc9668 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xc966c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xc9670 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0xc9674 | load | 4 | `abs 0xc98ec` | `-` |
| `ldr` at 0xc9678 | load | 4 | `abs 0x29733c` | `-` |
| `ldr` at 0xc9684 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc9690 | load | 4 | `abs 0xc98f0` | `-` |
| `str` at 0xc9694 | store | 4 | `sp -0x38` | `0x12044c` |
| `ldr` at 0xc9698 | load | 4 | `abs 0xc98ec` | `-` |
| `ldr` at 0xc969c | load | 4 | `abs 0x29733c` | `-` |
| `str` at 0xc96a0 | store | 4 | `sp -0x34` | `[0x29733c]` |
| `ldr` at 0xc96a4 | load | 4 | `abs 0xc98f4` | `-` |
| `ldr` at 0xc96a8 | load | 4 | `abs 0x297340` | `-` |
| `str` at 0xc96ac | store | 4 | `sp -0x30` | `[0x297340]` |
| `ldr` at 0xc96b4 | load | 4 | `abs 0xc98f8` | `-` |
| `ldr` at 0xc96c4 | load | 4 | `abs 0xc98f4` | `-` |
| `ldr` at 0xc96c8 | load | 4 | `abs 0x297340` | `-` |
| `ldr` at 0xc96d4 | load | 4 | `abs 0xc98f4` | `-` |
| `str` at 0xc96dc | store | 4 | `abs 0x297340` | `0x1` |
| `ldr` at 0xc96e0 | load | 4 | `abs 0xc98f4` | `-` |
| `ldr` at 0xc96e4 | load | 4 | `abs 0x297340` | `-` |
| `str` at 0xc96f8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc96fc | load | 4 | `abs 0xc98ec` | `-` |
| `ldr` at 0xc9700 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc9710 | store | 4 | `abs 0x29733c` | `ret(0x196d0)` |
| `ldr` at 0xc9714 | load | 4 | `abs 0x29733c` | `-` |
| `ldr` at 0xc9720 | load | 4 | `abs 0xc98fc` | `-` |
| `str` at 0xc9724 | store | 4 | `sp -0x38` | `0x120498` |
| `ldr` at 0xc9728 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc972c | store | 4 | `sp -0x34` | `[sp-44]` |
| `ldr` at 0xc9734 | load | 4 | `abs 0xc98f8` | `-` |
| `ldr` at 0xc9744 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc9758 | load | 4 | `abs 0xc98ec` | `-` |
| `ldr` at 0xc975c | load | 4 | `abs 0x29733c` | `-` |
| `str` at 0xc9764 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xc9768 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc976c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc9770 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0xc9774 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9778 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc977c | store | 4 | `[sp-36] +0x4` | `[sp-32]` |
| `ldr` at 0xc9780 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9784 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9788 | load | 4 | `[sp-36] +0x4` | `-` |
| `str` at 0xc9798 | store | 4 | `[sp-36] +0x8` | `ret(0x196d0)` |
| `ldr` at 0xc979c | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0xc97a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc97b0 | load | 4 | `abs 0xc9900` | `-` |
| `ldr` at 0xc97b4 | load | 4 | `abs 0xc9904` | `-` |
| `str` at 0xc97f0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc97f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc97fc | load | 4 | `abs 0xc9900` | `-` |
| `ldr` at 0xc9800 | load | 4 | `abs 0xc9904` | `-` |
| `ldr` at 0xc980c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc981c | load | 4 | `abs 0xc9908` | `-` |
| `str` at 0xc9820 | store | 4 | `(unknown)` | `0x1204dc` |
| `str` at 0xc9824 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0xc9828 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc982c | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0xc9834 | load | 4 | `abs 0xc98f8` | `-` |
| `str` at 0xc9848 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xc984c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9850 | load | 4 | `[sp-36] +0x4` | `-` |
| `ldr` at 0xc9854 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc9860 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9864 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc986c | load | 4 | `[sp-36] +0x0` | `-` |
| `ldrb` at 0xc9874 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc9880 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9884 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc9898 | load | 4 | `[sp-36] +0x8` | `-` |
| `strb` at 0xc98a4 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xc98ac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc98b0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc98c4 | load | 4 | `[sp-36] +0x8` | `-` |
| `strb` at 0xc98d0 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0xc98d4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc98dc | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldm` at 0xc98e8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xc98e8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc98e8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc98e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc96c0 | `(unknown callee) 0x200a0` | `0x9` | `0x12047c` | `0x54` | `0x0` |
| `bl` at 0xc9708 | `(unknown callee) 0x196d0` | `[sp-44]` | `0x1` | `[0x297340]` | `?` |
| `bl` at 0xc9740 | `(unknown callee) 0x200a0` | `0x9` | `0x12047c` | `0x60` | `0x0` |
| `bl` at 0xc9790 | `(unknown callee) 0x196d0` | `[[sp-36]+4]` | `0xc` | `[sp-36]` | `[sp-36]` |
| `bl` at 0xc97b8 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x1204d0` | `0x1204d8` |
| `bl` at 0xc97c4 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xc9804 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x1204d0` | `0x1204d8` |
| `bl` at 0xc9814 | `(unknown callee) 0x19808` | `[sp-44]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xc9840 | `(unknown callee) 0x200a0` | `0x9` | `0x12047c` | `0x6c` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc9680 | `[0x29733c] == 0x0` | `[0x29733c] != 0x0` |
| `bne` at 0xc968c | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xc96d0 | `[0x297340] != 0x0` | `[0x297340] == 0x0` |
| `bne` at 0xc971c | `[0x29733c] != 0x0` | `[0x29733c] == 0x0` |
| `bne` at 0xc97a4 | `[[sp-36]+8] != 0x0` | `[[sp-36]+8] == 0x0` |
| `ble` at 0xc985c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xc987c | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc98e8 | `r0` | `?` |

## `ioExpReset`

- `ioExpReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc9b7c to 0xc9d4c.
  Established. An included byte assignment through `[sp-32] +0x5` writes `0x5` (`strb` at 0xc9d00).
  A later byte assignment through `[sp-32] +0x3` writes zero (`strb` at 0xc9d0c). The unnamed
  `0xcbab0` call carries `[sp-20]`, `[sp-21]:1`, `[[sp-28]+7]:1`, `0x1` (`bl` at 0xc9d38). Expander
  reset policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc9b8c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xc9b94 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xc9b98 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc9b9c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc9bac | store | 4 | `sp -0x1c` | `ret(0xcb198)` |
| `ldrb` at 0xc9bb0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc9bb4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc9bc4 | store | 4 | `sp -0x20` | `ret(0xcb2c0)` |
| `ldr` at 0xc9bc8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc9bcc | load | 1 | `ret(0xcb2c0) +0x0` | `-` |
| `ldr` at 0xc9bd8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc9bdc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc9be0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc9be4 | load | 1 | `[sp-28] +0x6` | `-` |
| `ldrb` at 0xc9be8 | load | 1 | `[sp-28] +0x4` | `-` |
| `strb` at 0xc9bf0 | store | 1 | `ret(0xcb2c0) +0x1` | `?` |
| `ldr` at 0xc9bf4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc9bf8 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0xc9c00 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc9cbc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9cc8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc9cd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc9cd4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc9ce4 | store | 4 | `sp -0x24` | `ret(0xcb894)` |
| `ldr` at 0xc9ce8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc9cec | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc9cf0 | load | 1 | `[sp-32] +0x3` | `-` |
| `strb` at 0xc9cf4 | store | 1 | `[sp-32] +0x4` | `([[sp-32]+3]:1 & 0xff)` |
| `ldr` at 0xc9cf8 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xc9d00 | store | 1 | `[sp-32] +0x5` | `0x5` |
| `ldr` at 0xc9d04 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xc9d0c | store | 1 | `[sp-32] +0x3` | `0x0` |
| `ldr` at 0xc9d10 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc9d1c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc9d20 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc9d24 | load | 1 | `[sp-28] +0x7` | `-` |
| `ldr` at 0xc9d28 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc9d40 | store | 4 | `sp -0x24` | `ret(0xcbab0)` |
| `ldm` at 0xc9d48 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc9d48 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc9d48 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc9ba4 | `(unknown callee) 0xcb198` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xc9bbc | `(unknown callee) 0xcb2c0` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0xc9cdc | `(unknown callee) 0xcb894` | `[sp-20]` | `[sp-28]` | `[sp-32]+2` | `[sp-32]+2` |
| `bl` at 0xc9d38 | `(unknown callee) 0xcbab0` | `[sp-20]` | `[sp-21]:1` | `[[sp-28]+7]:1` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc9bd4 | `[ret(0xcb2c0)]:1 == 0x0` | `[ret(0xcb2c0)]:1 != 0x0` |
| `ldrls` at 0xc9c00 | `[[sp-28]]:1 <= 0x4 (unsigned, 32-bit)` | `[[sp-28]]:1 > 0x4 (unsigned, 32-bit)` |
| `bne` at 0xc9cc4 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `bne` at 0xc9d18 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc9d48 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xc9c00).

## `ioExpHardReset`

- `ioExpHardReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc9e74 to 0xc9efc.
  Established. The absolute word at `0x29733c` is read (`ldr` at 0xc9ea0). The unnamed `0xcc564`
  call carries entry `r0`, entry `r1`, unknown, unknown (`bl` at 0xc9eb0). The byte at `sp -0x19`
  starts at zero and receives `([sp-25]:1+1 & 0xff)` in the loop (`strb` at 0xc9eb8, `strb` at 0xc9ee8). Hard reset sequencing remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc9e84 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc9e88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc9e9c | load | 4 | `abs 0xc9ef8` | `-` |
| `ldr` at 0xc9ea0 | load | 4 | `abs 0x29733c` | `-` |
| `str` at 0xc9ea8 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xc9eac | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xc9eb8 | store | 1 | `sp -0x19` | `0x0` |
| `ldrb` at 0xc9ebc | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xc9ec0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc9ec4 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldrb` at 0xc9ed0 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xc9ed4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc9ee0 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xc9ee8 | store | 1 | `sp -0x19` | `([sp-25]:1+1 & 0xff)` |
| `ldm` at 0xc9ef4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc9ef4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc9ef4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc9eb0 | `(unknown callee) 0xcc564` | `r0` | `r1` | `?` | `?` |
| `bl` at 0xc9edc | `(unknown callee) 0xc9b7c` | `[sp-20]` | `?` | `[[sp-24]+4]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xc9ecc | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc9ef4 | `r0` | `?` |

## `ioExpPreConfig`

- `ioExpPreConfig`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc9d4c to 0xc9e74.
  Established. A zero byte through `ret(0xcb2c0) +0x0` takes `0xc9e60`, while nonzero falls through
  (`beq` at 0xc9dac). The included byte assignment through `ret(0xcb2c0) +0x1` writes an unknown
  value (`strb` at 0xc9dc8). The unnamed `0xcb894` call carries `[sp-20]`, `[sp-28]`, `[sp-32]+2`,
  `[sp-32]+2` (`bl` at 0xc9e54). Expander preparation effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc9d5c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xc9d64 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `str` at 0xc9d6c | store | 4 | `sp -0x24` | `0x0` |
| `ldrb` at 0xc9d70 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc9d74 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc9d84 | store | 4 | `sp -0x1c` | `ret(0xcb198)` |
| `ldrb` at 0xc9d88 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc9d8c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc9d9c | store | 4 | `sp -0x20` | `ret(0xcb2c0)` |
| `ldr` at 0xc9da0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc9da4 | load | 1 | `ret(0xcb2c0) +0x0` | `-` |
| `ldr` at 0xc9db0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc9db4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc9db8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc9dbc | load | 1 | `[sp-28] +0x6` | `-` |
| `ldrb` at 0xc9dc0 | load | 1 | `[sp-28] +0x4` | `-` |
| `strb` at 0xc9dc8 | store | 1 | `ret(0xcb2c0) +0x1` | `?` |
| `ldr` at 0xc9dcc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc9dd0 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0xc9ddc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc9e34 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9e40 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc9e48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc9e4c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc9e5c | store | 4 | `sp -0x24` | `ret(0xcb894)` |
| `ldm` at 0xc9e64 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc9e64 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc9e64 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc9d7c | `(unknown callee) 0xcb198` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xc9d94 | `(unknown callee) 0xcb2c0` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0xc9e54 | `(unknown callee) 0xcb894` | `[sp-20]` | `[sp-28]` | `[sp-32]+2` | `[sp-32]+2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc9dac | `[ret(0xcb2c0)]:1 == 0x0` | `[ret(0xcb2c0)]:1 != 0x0` |
| `ldrls` at 0xc9ddc | `[[sp-28]]:1-1 <= 0x3 (unsigned, 32-bit)` | `[[sp-28]]:1-1 > 0x3 (unsigned, 32-bit)` |
| `bne` at 0xc9e3c | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc9e64 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xc9ddc).

## `ioExpPresent`

- `ioExpPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc990c to 0xc9a44.
  Established. An included byte assignment at an unknown target writes `0x1` (`strb` at 0xc9984).
  The signed condition `(r1 & 0xff) >= [[sp-28]+4]` takes `0xc998c`, while smaller values fall
  through (`bge` at 0xc995c). The unnamed `0x200a0` call carries `0x6`, `0x12047c`, `0x82`, `0x0`
  (`bl` at 0xc9a24). Expander presence interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc991c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xc9924 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xc9928 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc993c | load | 4 | `abs 0xc9a30` | `-` |
| `ldr` at 0xc9940 | load | 4 | `abs 0x29733c` | `-` |
| `str` at 0xc9948 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xc994c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc9950 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc9954 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xc9960 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc9964 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc9978 | load | 4 | `[sp-28] +0x8` | `-` |
| `strb` at 0xc9984 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0xc998c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc9994 | load | 4 | `abs 0xc9a34` | `-` |
| `ldr` at 0xc9998 | load | 4 | `abs 0xc9a38` | `-` |
| `str` at 0xc99d4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xc99d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc99e0 | load | 4 | `abs 0xc9a34` | `-` |
| `ldr` at 0xc99e4 | load | 4 | `abs 0xc9a38` | `-` |
| `ldr` at 0xc99f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc9a00 | load | 4 | `abs 0xc9a3c` | `-` |
| `str` at 0xc9a04 | store | 4 | `(unknown)` | `0x120514` |
| `str` at 0xc9a08 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldrb` at 0xc9a0c | load | 1 | `sp -0x15` | `-` |
| `str` at 0xc9a10 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0xc9a18 | load | 4 | `abs 0xc9a40` | `-` |
| `ldm` at 0xc9a2c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc9a2c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc9a2c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc999c | `(unknown callee) 0x48da8` | `r0` | `0x0` | `0x1204d0` | `0x1204d8` |
| `bl` at 0xc99a8 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xc99e8 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0x1204d0` | `0x1204d8` |
| `bl` at 0xc99f8 | `(unknown callee) 0x19808` | `[sp-32]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xc9a24 | `(unknown callee) 0x200a0` | `0x6` | `0x12047c` | `0x82` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xc995c | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc9a2c | `r0` | `?` |

## `ioExpNotPresent`

- `ioExpNotPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc9a44 to 0xc9b7c.
  Established. An included byte assignment at an unknown target writes zero (`strb` at 0xc9abc). The
  unnamed `0x48da8` call at `0xc9ad4` carries entry `r0`, `0x0`, `0x1204d0`, `0x1204d8` (`bl` at 0xc9ad4). The absolute word at `0x29733c` is read (`ldr` at 0xc9a78). Expander removal semantics
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc9a54 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xc9a5c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xc9a60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc9a74 | load | 4 | `abs 0xc9b68` | `-` |
| `ldr` at 0xc9a78 | load | 4 | `abs 0x29733c` | `-` |
| `str` at 0xc9a80 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xc9a84 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc9a88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc9a8c | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xc9a98 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc9a9c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc9ab0 | load | 4 | `[sp-28] +0x8` | `-` |
| `strb` at 0xc9abc | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xc9ac4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc9acc | load | 4 | `abs 0xc9b6c` | `-` |
| `ldr` at 0xc9ad0 | load | 4 | `abs 0xc9b70` | `-` |
| `str` at 0xc9b0c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xc9b10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc9b18 | load | 4 | `abs 0xc9b6c` | `-` |
| `ldr` at 0xc9b1c | load | 4 | `abs 0xc9b70` | `-` |
| `ldr` at 0xc9b28 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc9b38 | load | 4 | `abs 0xc9b74` | `-` |
| `str` at 0xc9b3c | store | 4 | `(unknown)` | `0x120538` |
| `str` at 0xc9b40 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldrb` at 0xc9b44 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xc9b48 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0xc9b50 | load | 4 | `abs 0xc9b78` | `-` |
| `ldm` at 0xc9b64 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc9b64 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc9b64 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc9ad4 | `(unknown callee) 0x48da8` | `r0` | `0x0` | `0x1204d0` | `0x1204d8` |
| `bl` at 0xc9ae0 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xc9b20 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0x1204d0` | `0x1204d8` |
| `bl` at 0xc9b30 | `(unknown callee) 0x19808` | `[sp-32]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xc9b5c | `(unknown callee) 0x200a0` | `0x6` | `0x12047c` | `0x91` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xc9a94 | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc9b64 | `r0` | `?` |

## `ioExpRead`

- `ioExpRead`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca70c to 0xca7a0.
  Established. A nonzero byte through `ret(0xcb2c0) +0x0` takes `0xca778`, while zero falls through
  (`bne` at 0xca768). An included word assignment at `sp -0x28` writes `0xfffffc11` (`str` at 0xca770). The unnamed `0xcb894` call carries `[sp-20]`, `[sp-32]`, `[sp-28]`, `[ret(0xcb2c0)]:1`
  (`bl` at 0xca784). Expander read protocol remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca71c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xca724 | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0xca728 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xca72c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xca730 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xca740 | store | 4 | `sp -0x20` | `ret(0xcb198)` |
| `ldrb` at 0xca744 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xca748 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xca758 | store | 4 | `sp -0x24` | `ret(0xcb2c0)` |
| `ldr` at 0xca75c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xca760 | load | 1 | `ret(0xcb2c0) +0x0` | `-` |
| `ldr` at 0xca76c | load | 4 | `abs 0xca79c` | `-` |
| `str` at 0xca770 | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0xca778 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca77c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xca780 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xca78c | store | 4 | `sp -0x28` | `ret(0xcb894)` |
| `ldr` at 0xca790 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xca798 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xca798 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xca798 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xca738 | `(unknown callee) 0xcb198` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xca750 | `(unknown callee) 0xcb2c0` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0xca784 | `(unknown callee) 0xcb894` | `[sp-20]` | `[sp-32]` | `[sp-28]` | `[ret(0xcb2c0)]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xca768 | `[ret(0xcb2c0)]:1 != 0x0` | `[ret(0xcb2c0)]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xca798 | `r0` | `?` |

## `ioExpWrite`

- `ioExpWrite`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca584 to 0xca688.
  Established. A zero `ret(0xbaf50)` takes `0xca644`, while nonzero falls through (`beq` at 0xca634).
  The unnamed `0xcb3f4` call carries `[sp-20]`, `[sp-28]`, `[sp-32]`, `[sp-22]:1` (`bl` at 0xca660). The word at `sp -0x2c` receives `[sp-23]:1` before that included call (`str` at 0xca64c). Expander write effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca594 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xca5a0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xca5a8 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0xca5b0 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldrb` at 0xca5b4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xca5b8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xca5c8 | store | 4 | `sp -0x1c` | `ret(0xcb198)` |
| `ldrb` at 0xca5cc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xca5d0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xca5e0 | store | 4 | `sp -0x20` | `ret(0xcb2c0)` |
| `ldr` at 0xca5e4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xca5e8 | load | 1 | `ret(0xcb2c0) +0x0` | `-` |
| `ldr` at 0xca5f4 | load | 4 | `abs 0xca684` | `-` |
| `str` at 0xca5f8 | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0xca600 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xca604 | load | 1 | `[sp-28] +0x1` | `-` |
| `ldr` at 0xca608 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xca628 | store | 4 | `sp -0x24` | `ret(0xbaf50)` |
| `ldr` at 0xca62c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca638 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xca63c | store | 4 | `sp -0x28` | `ret(0xbaf50)` |
| `ldrb` at 0xca644 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0xca648 | load | 1 | `sp -0x17` | `-` |
| `str` at 0xca64c | store | 4 | `sp -0x2c` | `[sp-23]:1` |
| `ldr` at 0xca650 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca654 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xca658 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xca668 | store | 4 | `sp -0x24` | `ret(0xcb3f4)` |
| `ldr` at 0xca670 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xca674 | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0xca678 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xca680 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xca680 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xca680 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xca5c0 | `(unknown callee) 0xcb198` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xca5d8 | `(unknown callee) 0xcb2c0` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0xca610 | `(unknown callee) 0xbac78` | `[sp-20]` | `[[sp-28]+1]:1` | `?` | `[[sp-28]+1]:1` |
| `bl` at 0xca620 | `(unknown callee) 0xbaf50` | `?` | `?` | `?` | `?` |
| `bl` at 0xca660 | `(unknown callee) 0xcb3f4` | `[sp-20]` | `[sp-28]` | `[sp-32]` | `[sp-22]:1` |
| `bl` at 0xca66c | `(unknown callee) 0xbb094` | `ret(0xcb3f4)` | `?` | `?` | `ret(0xcb3f4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xca5f0 | `[ret(0xcb2c0)]:1 != 0x0` | `[ret(0xcb2c0)]:1 == 0x0` |
| `beq` at 0xca634 | `ret(0xbaf50) == 0x0` | `ret(0xbaf50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xca680 | `r0` | `?` |
