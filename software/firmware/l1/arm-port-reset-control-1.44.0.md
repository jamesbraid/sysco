# ARM port reset and control paths in 1.44.0

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
No unresolved computed transfers were reported for these routines.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/port-reset-control-facts.txt) retain all tool sections.

## `portInit`

- `portInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3acb8 to 0x3b014.
  Established. The absolute word at `0x1321b0` receives `0x1` (`str` at 0x3ad3c). The word at
  `0x1321ac` receives `ret(0x196d0)` (`str` at 0x3ad70). A word through `[sp-40] +0x8` receives
  `[sp-32]` (`str` at 0x3addc). The unnamed `0x20bb4` call carries `0x1321b4`, `0xf5efc`, `0x1`,
  `0x9` (`bl` at 0x3afe0). Port initialization effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3acc8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x3accc | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x3acd0 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x3acd4 | load | 4 | `abs 0x3afec` | `-` |
| `ldr` at 0x3acd8 | load | 4 | `abs 0x1321ac` | `-` |
| `ldr` at 0x3ace4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3acf0 | load | 4 | `abs 0x3aff0` | `-` |
| `str` at 0x3acf4 | store | 4 | `sp -0x38` | `0xf5e34` |
| `ldr` at 0x3acf8 | load | 4 | `abs 0x3afec` | `-` |
| `ldr` at 0x3acfc | load | 4 | `abs 0x1321ac` | `-` |
| `str` at 0x3ad00 | store | 4 | `sp -0x34` | `[0x1321ac]` |
| `ldr` at 0x3ad04 | load | 4 | `abs 0x3aff4` | `-` |
| `ldr` at 0x3ad08 | load | 4 | `abs 0x1321b0` | `-` |
| `str` at 0x3ad0c | store | 4 | `sp -0x30` | `[0x1321b0]` |
| `ldr` at 0x3ad14 | load | 4 | `abs 0x3aff8` | `-` |
| `ldr` at 0x3ad24 | load | 4 | `abs 0x3aff4` | `-` |
| `ldr` at 0x3ad28 | load | 4 | `abs 0x1321b0` | `-` |
| `ldr` at 0x3ad34 | load | 4 | `abs 0x3aff4` | `-` |
| `str` at 0x3ad3c | store | 4 | `abs 0x1321b0` | `0x1` |
| `ldr` at 0x3ad40 | load | 4 | `abs 0x3aff4` | `-` |
| `ldr` at 0x3ad44 | load | 4 | `abs 0x1321b0` | `-` |
| `str` at 0x3ad58 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x3ad5c | load | 4 | `abs 0x3afec` | `-` |
| `ldr` at 0x3ad60 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3ad70 | store | 4 | `abs 0x1321ac` | `ret(0x196d0)` |
| `ldr` at 0x3ad74 | load | 4 | `abs 0x1321ac` | `-` |
| `ldr` at 0x3ad80 | load | 4 | `abs 0x3affc` | `-` |
| `str` at 0x3ad84 | store | 4 | `sp -0x38` | `0xf5e78` |
| `ldr` at 0x3ad88 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3ad8c | store | 4 | `sp -0x34` | `[sp-44]` |
| `ldr` at 0x3ad94 | load | 4 | `abs 0x3aff8` | `-` |
| `ldr` at 0x3ada4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3adb8 | load | 4 | `abs 0x3afec` | `-` |
| `ldr` at 0x3adbc | load | 4 | `abs 0x1321ac` | `-` |
| `str` at 0x3adc4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x3adc8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3adcc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3add0 | store | 4 | `[sp-40] +0x0` | `?` |
| `ldr` at 0x3add4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3add8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3addc | store | 4 | `[sp-40] +0x8` | `[sp-32]` |
| `ldr` at 0x3ade0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3ade4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3ade8 | load | 4 | `[sp-40] +0x8` | `-` |
| `str` at 0x3adf8 | store | 4 | `[sp-40] +0x4` | `ret(0x196d0)` |
| `ldr` at 0x3adfc | load | 4 | `[sp-40] +0x4` | `-` |
| `ldr` at 0x3ae08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ae10 | load | 4 | `abs 0x3b000` | `-` |
| `ldr` at 0x3ae14 | load | 4 | `abs 0x3b004` | `-` |
| `str` at 0x3ae50 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x3ae54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ae5c | load | 4 | `abs 0x3b000` | `-` |
| `ldr` at 0x3ae60 | load | 4 | `abs 0x3b004` | `-` |
| `ldr` at 0x3ae6c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3ae7c | load | 4 | `abs 0x3b008` | `-` |
| `str` at 0x3ae80 | store | 4 | `(unknown)` | `0xf5ebc` |
| `str` at 0x3ae84 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0x3ae88 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3ae8c | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0x3ae94 | load | 4 | `abs 0x3aff8` | `-` |
| `strb` at 0x3aea8 | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0x3aeac | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x3aeb0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3aeb4 | load | 4 | `[sp-40] +0x8` | `-` |
| `ldr` at 0x3aec0 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3aec4 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x3aed8 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldrb` at 0x3aee0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3aee4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3aefc | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3af00 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x3af14 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldrb` at 0x3af1c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3af28 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3af2c | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x3af40 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldrb` at 0x3af48 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3af4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3af68 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3af6c | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x3af74 | load | 4 | `[sp-40] +0x4` | `-` |
| `strb` at 0x3af80 | store | 1 | `(unknown)` | `0x2` |
| `ldr` at 0x3af88 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3af8c | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x3af94 | load | 4 | `[sp-40] +0x4` | `-` |
| `strb` at 0x3afa0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x3afa4 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3afa8 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x3afb0 | load | 4 | `[sp-40] +0x4` | `-` |
| `strb` at 0x3afbc | store | 1 | `(unknown)` | `0x0` |
| `ldrb` at 0x3afc0 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x3afc8 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldr` at 0x3afd0 | load | 4 | `abs 0x3b00c` | `-` |
| `ldr` at 0x3afd4 | load | 4 | `abs 0x3b010` | `-` |
| `ldm` at 0x3afe8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x3afe8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3afe8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3afe8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3ad20 | `(unknown callee) 0x200a0` | `0x9` | `0xf5e64` | `0x6c` | `0x0` |
| `bl` at 0x3ad68 | `(unknown callee) 0x196d0` | `[sp-44]` | `0x1` | `[0x1321b0]` | `?` |
| `bl` at 0x3ada0 | `(unknown callee) 0x200a0` | `0x9` | `0xf5e64` | `0x78` | `0x0` |
| `bl` at 0x3adf0 | `(unknown callee) 0x196d0` | `[[sp-40]+8]` | `0x4` | `[sp-40]` | `[sp-40]` |
| `bl` at 0x3ae18 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0xf5eb0` | `0xf5eb8` |
| `bl` at 0x3ae24 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x3ae64 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0xf5eb0` | `0xf5eb8` |
| `bl` at 0x3ae74 | `(unknown callee) 0x19808` | `[sp-44]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x3aea0 | `(unknown callee) 0x200a0` | `0x9` | `0xf5e64` | `0x84` | `0x0` |
| `bl` at 0x3aeec | `(unknown callee) 0xca484` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x3af54 | `(unknown callee) 0xca484` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x3afe0 | `(unknown callee) 0x20bb4` | `0x1321b4` | `0xf5efc` | `0x1` | `0x9` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3ace0 | `[0x1321ac] == 0x0` | `[0x1321ac] != 0x0` |
| `bne` at 0x3acec | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0x3ad30 | `[0x1321b0] != 0x0` | `[0x1321b0] == 0x0` |
| `bne` at 0x3ad7c | `[0x1321ac] != 0x0` | `[0x1321ac] == 0x0` |
| `bne` at 0x3ae04 | `[[sp-40]+4] != 0x0` | `[[sp-40]+4] == 0x0` |
| `bge` at 0x3aebc | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x3aef8 | `ret(0xca484) == 0x0` | `ret(0xca484) != 0x0` |
| `beq` at 0x3af24 | `? == 0xff` | `? != 0xff` |
| `bne` at 0x3af60 | `ret(0xca484) != 0x0` | `ret(0xca484) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3afe8 | `r0` | `ret(0x20bb4)` |

## `portEnable`

- `portEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3b014 to 0x3b15c.
  Established. An included byte assignment at an unknown target writes `0x2` (`strb` at 0x3b13c).
  The signed condition `(r1 & 0xff) < [[sp-28]+8]` takes `0x3b104`, while greater or equal values
  fall through (`blt` at 0x3b064). The unnamed `0x200a0` call carries `0x9`, `0xf5e64`, `0xa1`,
  `0x0` (`bl` at 0x3b100). Enable transition policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3b024 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x3b02c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x3b030 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b044 | load | 4 | `abs 0x3b148` | `-` |
| `ldr` at 0x3b048 | load | 4 | `abs 0x1321ac` | `-` |
| `str` at 0x3b050 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x3b054 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3b058 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3b05c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x3b068 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b070 | load | 4 | `abs 0x3b14c` | `-` |
| `ldr` at 0x3b074 | load | 4 | `abs 0x3b150` | `-` |
| `str` at 0x3b0b0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x3b0b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b0bc | load | 4 | `abs 0x3b14c` | `-` |
| `ldr` at 0x3b0c0 | load | 4 | `abs 0x3b150` | `-` |
| `ldr` at 0x3b0cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3b0dc | load | 4 | `abs 0x3b154` | `-` |
| `str` at 0x3b0e0 | store | 4 | `(unknown)` | `0xf5f08` |
| `str` at 0x3b0e4 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldrb` at 0x3b0e8 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x3b0ec | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x3b0f4 | load | 4 | `abs 0x3b158` | `-` |
| `ldr` at 0x3b104 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3b108 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3b110 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x3b118 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3b124 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3b128 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3b130 | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0x3b13c | store | 1 | `(unknown)` | `0x2` |
| `ldm` at 0x3b144 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3b144 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3b144 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3b078 | `(unknown callee) 0x48da8` | `r0` | `0x0` | `0xf5eb0` | `0xf5eb8` |
| `bl` at 0x3b084 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x3b0c4 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0xf5eb0` | `0xf5eb8` |
| `bl` at 0x3b0d4 | `(unknown callee) 0x19808` | `[sp-32]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x3b100 | `(unknown callee) 0x200a0` | `0x9` | `0xf5e64` | `0xa1` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3b064 | `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` |
| `beq` at 0x3b120 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3b144 | `r0` | `?` |

## `portDisable`

- `portDisable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3b15c to 0x3b2a4.
  Established. An included byte assignment at an unknown target writes `0x1` (`strb` at 0x3b284).
  The word through `[sp-28] +0x4` is read (`ldr` at 0x3b258). The unnamed `0x200a0` call carries
  `0x9`, `0xf5e64`, `0xad`, `0x0` (`bl` at 0x3b248). Disable transition meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3b16c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x3b174 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x3b178 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b18c | load | 4 | `abs 0x3b290` | `-` |
| `ldr` at 0x3b190 | load | 4 | `abs 0x1321ac` | `-` |
| `str` at 0x3b198 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x3b19c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3b1a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3b1a4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x3b1b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b1b8 | load | 4 | `abs 0x3b294` | `-` |
| `ldr` at 0x3b1bc | load | 4 | `abs 0x3b298` | `-` |
| `str` at 0x3b1f8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x3b1fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b204 | load | 4 | `abs 0x3b294` | `-` |
| `ldr` at 0x3b208 | load | 4 | `abs 0x3b298` | `-` |
| `ldr` at 0x3b214 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3b224 | load | 4 | `abs 0x3b29c` | `-` |
| `str` at 0x3b228 | store | 4 | `(unknown)` | `0xf5f2c` |
| `str` at 0x3b22c | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldrb` at 0x3b230 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x3b234 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x3b23c | load | 4 | `abs 0x3b2a0` | `-` |
| `ldr` at 0x3b24c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3b250 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3b258 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x3b260 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3b26c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3b270 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3b278 | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0x3b284 | store | 1 | `(unknown)` | `0x1` |
| `ldm` at 0x3b28c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3b28c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3b28c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3b1c0 | `(unknown callee) 0x48da8` | `r0` | `0x0` | `0xf5eb0` | `0xf5eb8` |
| `bl` at 0x3b1cc | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x3b20c | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0xf5eb0` | `0xf5eb8` |
| `bl` at 0x3b21c | `(unknown callee) 0x19808` | `[sp-32]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x3b248 | `(unknown callee) 0x200a0` | `0x9` | `0xf5e64` | `0xad` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3b1ac | `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` |
| `beq` at 0x3b268 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3b28c | `r0` | `?` |

## `portIsEnabled`

- `portIsEnabled`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3b2a4 to 0x3b3d8.
  Established. The signed condition `(r1 & 0xff) < [[sp-28]+8]` takes `0x3b394`, while greater or
  equal values fall through (`blt` at 0x3b2f4). The absolute word at `0x1321ac` is read (`ldr` at 0x3b2d8). The unnamed `0x200a0` call carries `0x9`, `0xf5e64`, `0xb9`, `0x0` (`bl` at 0x3b390).
  Port enable query semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3b2b4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x3b2bc | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x3b2c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b2d4 | load | 4 | `abs 0x3b3c4` | `-` |
| `ldr` at 0x3b2d8 | load | 4 | `abs 0x1321ac` | `-` |
| `str` at 0x3b2e0 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x3b2e4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3b2e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3b2ec | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x3b2f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b300 | load | 4 | `abs 0x3b3c8` | `-` |
| `ldr` at 0x3b304 | load | 4 | `abs 0x3b3cc` | `-` |
| `str` at 0x3b340 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x3b344 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b34c | load | 4 | `abs 0x3b3c8` | `-` |
| `ldr` at 0x3b350 | load | 4 | `abs 0x3b3cc` | `-` |
| `ldr` at 0x3b35c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3b36c | load | 4 | `abs 0x3b3d0` | `-` |
| `str` at 0x3b370 | store | 4 | `(unknown)` | `0xf5f50` |
| `str` at 0x3b374 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldrb` at 0x3b378 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x3b37c | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x3b384 | load | 4 | `abs 0x3b3d4` | `-` |
| `ldr` at 0x3b394 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3b398 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3b3a0 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x3b3a8 | load | 1 | `(unknown)` | `-` |
| `ldm` at 0x3b3c0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3b3c0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3b3c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3b308 | `(unknown callee) 0x48da8` | `r0` | `0x0` | `0xf5eb0` | `0xf5eb8` |
| `bl` at 0x3b314 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x3b354 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0xf5eb0` | `0xf5eb8` |
| `bl` at 0x3b364 | `(unknown callee) 0x19808` | `[sp-32]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x3b390 | `(unknown callee) 0x200a0` | `0x9` | `0xf5e64` | `0xb9` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3b2f4 | `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3b3c0 | `r0` | `?` |

## `portPresent`

- `portPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3b3d8 to 0x3b500.
  Established. An included byte assignment at an unknown target writes `0x2` (`strb` at 0x3b4e0).
  The unnamed `0x48da8` call at `0x3b43c` carries entry `r0`, `0x0`, `0xf5eb0`, `0xf5eb8` (`bl` at 0x3b43c). The word through `[sp-28] +0x8` is read (`ldr` at 0x3b420). Port presence policy remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3b3e8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x3b3f0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x3b3f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b408 | load | 4 | `abs 0x3b4ec` | `-` |
| `ldr` at 0x3b40c | load | 4 | `abs 0x1321ac` | `-` |
| `str` at 0x3b414 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x3b418 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3b41c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3b420 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x3b42c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b434 | load | 4 | `abs 0x3b4f0` | `-` |
| `ldr` at 0x3b438 | load | 4 | `abs 0x3b4f4` | `-` |
| `str` at 0x3b474 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x3b478 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b480 | load | 4 | `abs 0x3b4f0` | `-` |
| `ldr` at 0x3b484 | load | 4 | `abs 0x3b4f4` | `-` |
| `ldr` at 0x3b490 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3b4a0 | load | 4 | `abs 0x3b4f8` | `-` |
| `str` at 0x3b4a4 | store | 4 | `(unknown)` | `0xf5f78` |
| `str` at 0x3b4a8 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldrb` at 0x3b4ac | load | 1 | `sp -0x15` | `-` |
| `str` at 0x3b4b0 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x3b4b8 | load | 4 | `abs 0x3b4fc` | `-` |
| `ldr` at 0x3b4c8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3b4cc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3b4d4 | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0x3b4e0 | store | 1 | `(unknown)` | `0x2` |
| `ldm` at 0x3b4e8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3b4e8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3b4e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3b43c | `(unknown callee) 0x48da8` | `r0` | `0x0` | `0xf5eb0` | `0xf5eb8` |
| `bl` at 0x3b448 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x3b488 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0xf5eb0` | `0xf5eb8` |
| `bl` at 0x3b498 | `(unknown callee) 0x19808` | `[sp-32]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x3b4c4 | `(unknown callee) 0x200a0` | `0x9` | `0xf5e64` | `0xc8` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3b428 | `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3b4e8 | `r0` | `?` |

## `portNotPresent`

- `portNotPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3b500 to 0x3b628.
  Established. An included byte assignment at an unknown target writes zero (`strb` at 0x3b608). The
  unnamed `0x19808` call carries `[sp-32]`, `ret(0x48da8)`, unknown, `ret(0x48da8)` (`bl` at 0x3b5c0). The absolute word at `0x1321ac` is read (`ldr` at 0x3b534). Port removal behavior
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3b510 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x3b518 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x3b51c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b530 | load | 4 | `abs 0x3b614` | `-` |
| `ldr` at 0x3b534 | load | 4 | `abs 0x1321ac` | `-` |
| `str` at 0x3b53c | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x3b540 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3b544 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3b548 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x3b554 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b55c | load | 4 | `abs 0x3b618` | `-` |
| `ldr` at 0x3b560 | load | 4 | `abs 0x3b61c` | `-` |
| `str` at 0x3b59c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x3b5a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b5a8 | load | 4 | `abs 0x3b618` | `-` |
| `ldr` at 0x3b5ac | load | 4 | `abs 0x3b61c` | `-` |
| `ldr` at 0x3b5b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3b5c8 | load | 4 | `abs 0x3b620` | `-` |
| `str` at 0x3b5cc | store | 4 | `(unknown)` | `0xf5f9c` |
| `str` at 0x3b5d0 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldrb` at 0x3b5d4 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x3b5d8 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x3b5e0 | load | 4 | `abs 0x3b624` | `-` |
| `ldr` at 0x3b5f0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3b5f4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3b5fc | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0x3b608 | store | 1 | `(unknown)` | `0x0` |
| `ldm` at 0x3b610 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3b610 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3b610 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3b564 | `(unknown callee) 0x48da8` | `r0` | `0x0` | `0xf5eb0` | `0xf5eb8` |
| `bl` at 0x3b570 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x3b5b0 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0xf5eb0` | `0xf5eb8` |
| `bl` at 0x3b5c0 | `(unknown callee) 0x19808` | `[sp-32]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x3b5ec | `(unknown callee) 0x200a0` | `0x9` | `0xf5e64` | `0xd3` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3b550 | `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3b610 | `r0` | `?` |

## `portIsPresent`

- `portIsPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3b628 to 0x3b75c.
  Established. The signed condition `(r1 & 0xff) < [[sp-28]+8]` takes `0x3b718`, while greater or
  equal values fall through (`blt` at 0x3b678). The unnamed `0x200a0` call carries `0x9`, `0xf5e64`,
  `0xde`, `0x0` (`bl` at 0x3b714). An included byte read has unknown effective target (`ldrb` at 0x3b72c). Port detection meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3b638 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x3b640 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x3b644 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b658 | load | 4 | `abs 0x3b748` | `-` |
| `ldr` at 0x3b65c | load | 4 | `abs 0x1321ac` | `-` |
| `str` at 0x3b664 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x3b668 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3b66c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3b670 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x3b67c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b684 | load | 4 | `abs 0x3b74c` | `-` |
| `ldr` at 0x3b688 | load | 4 | `abs 0x3b750` | `-` |
| `str` at 0x3b6c4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x3b6c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b6d0 | load | 4 | `abs 0x3b74c` | `-` |
| `ldr` at 0x3b6d4 | load | 4 | `abs 0x3b750` | `-` |
| `ldr` at 0x3b6e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3b6f0 | load | 4 | `abs 0x3b754` | `-` |
| `str` at 0x3b6f4 | store | 4 | `(unknown)` | `0xf5fc4` |
| `str` at 0x3b6f8 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldrb` at 0x3b6fc | load | 1 | `sp -0x15` | `-` |
| `str` at 0x3b700 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x3b708 | load | 4 | `abs 0x3b758` | `-` |
| `ldr` at 0x3b718 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3b71c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3b724 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x3b72c | load | 1 | `(unknown)` | `-` |
| `ldm` at 0x3b744 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3b744 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3b744 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3b68c | `(unknown callee) 0x48da8` | `r0` | `0x0` | `0xf5eb0` | `0xf5eb8` |
| `bl` at 0x3b698 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x3b6d8 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0xf5eb0` | `0xf5eb8` |
| `bl` at 0x3b6e8 | `(unknown callee) 0x19808` | `[sp-32]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x3b714 | `(unknown callee) 0x200a0` | `0x9` | `0xf5e64` | `0xde` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3b678 | `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3b744 | `r0` | `?` |

## `portReset`

- `portReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3b75c to 0x3b968.
  Established. Separate included byte assignments through `[sp-44] +0x0` write `0x3` (`strb` at 0x3b83c, `strb` at 0x3b868). A zero `ret(0x3cdd8)` takes `0x3b938`, while nonzero falls through
  (`beq` at 0x3b888). The unnamed `0x3cdd8` call carries `[sp-24]`, `[sp-40]`, unknown, unknown
  (`bl` at 0x3b874). Reset sequencing remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3b76c | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x3b770 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3b784 | load | 4 | `abs 0x3b950` | `-` |
| `ldr` at 0x3b788 | load | 4 | `abs 0x1321ac` | `-` |
| `str` at 0x3b790 | store | 4 | `sp -0x24` | `?` |
| `strb` at 0x3b798 | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0x3b79c | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x3b7a0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3b7a4 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0x3b7b0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x3b7b4 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x3b7bc | load | 4 | `[sp-36] +0x4` | `-` |
| `ldrb` at 0x3b7c4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3b7d0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x3b7d4 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x3b7e8 | load | 4 | `[sp-36] +0x0` | `-` |
| `str` at 0x3b7f0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x3b7f4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x3b7f8 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x3b800 | load | 4 | `[sp-36] +0x4` | `-` |
| `str` at 0x3b808 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x3b80c | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0x3b810 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x3b820 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3b834 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0x3b83c | store | 1 | `[sp-44] +0x0` | `0x3` |
| `ldr` at 0x3b844 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3b848 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3b860 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0x3b868 | store | 1 | `[sp-44] +0x0` | `0x3` |
| `ldr` at 0x3b86c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3b870 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3b87c | store | 4 | `sp -0x1c` | `ret(0x3cdd8)` |
| `ldr` at 0x3b880 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3b88c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3b894 | load | 4 | `abs 0x3b954` | `-` |
| `ldr` at 0x3b898 | load | 4 | `abs 0x3b958` | `-` |
| `str` at 0x3b8d4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x3b8d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3b8e0 | load | 4 | `abs 0x3b954` | `-` |
| `ldr` at 0x3b8e4 | load | 4 | `abs 0x3b958` | `-` |
| `ldr` at 0x3b8f0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3b900 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3b90c | load | 4 | `abs 0x3b95c` | `-` |
| `str` at 0x3b910 | store | 4 | `(unknown)` | `0xf5fec` |
| `str` at 0x3b914 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldrb` at 0x3b918 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x3b91c | store | 4 | `(unknown)` | `[sp-29]:1` |
| `str` at 0x3b920 | store | 4 | `(unknown)` | `ret(0x1fc1c)` |
| `ldr` at 0x3b928 | load | 4 | `abs 0x3b960` | `-` |
| `ldr` at 0x3b92c | load | 4 | `abs 0x3b964` | `-` |
| `ldrb` at 0x3b938 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x3b940 | store | 1 | `sp -0x1d` | `?` |
| `ldm` at 0x3b94c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x3b94c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3b94c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3b94c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3b824 | `(unknown callee) 0x37674` | `?` | `[sp-36]` | `?` | `?` |
| `bl` at 0x3b850 | `(unknown callee) 0x3e418` | `?` | `?` | `0x2` | `?` |
| `bl` at 0x3b874 | `(unknown callee) 0x3cdd8` | `[sp-24]` | `[sp-40]` | `?` | `?` |
| `bl` at 0x3b89c | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0xf5eb0` | `0xf5eb8` |
| `bl` at 0x3b8a8 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x3b8e8 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0xf5eb0` | `0xf5eb8` |
| `bl` at 0x3b8f8 | `(unknown callee) 0x19808` | `[sp-48]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x3b904 | `(unknown callee) 0x1fc1c` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x3b934 | `(unknown callee) 0x200a0` | `0x3` | `0xf5e64` | `0x10b` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x3b7ac | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bls` at 0x3b7cc | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x3b81c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x3b830 | `ret(0x37674) == 0x0` | `ret(0x37674) != 0x0` |
| `bne` at 0x3b85c | `ret(0x3e418) != 0x1` | `ret(0x3e418) == 0x1` |
| `beq` at 0x3b888 | `ret(0x3cdd8) == 0x0` | `ret(0x3cdd8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3b94c | `r0` | `?` |

## `portGetCfg`

- `portGetCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3b968 to 0x3b9cc.
  Established. The word through entry `r1 +0x0` receives `[[sp-28]]` (`str` at 0x3b9b0). The
  included return record gives integer `r0` as `[[sp-28]+8]` (`ldm` at 0x3b9c4). The absolute word
  at `0x1321ac` is read (`ldr` at 0x3b998). Port configuration layout remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3b978 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3b97c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x3b980 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b994 | load | 4 | `abs 0x3b9c8` | `-` |
| `ldr` at 0x3b998 | load | 4 | `abs 0x1321ac` | `-` |
| `str` at 0x3b9a0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x3b9a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3b9a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3b9ac | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x3b9b0 | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0x3b9b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3b9b8 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldm` at 0x3b9c4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3b9c4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3b9c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3b9c4 | `r0` | `[[sp-28]+8]` |

## `portUpInternal`

- `portUpInternal`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3b9cc to 0x3bcc4.
  Established. The condition `(r1 & 0xff) != 0xff` takes `0x3ba40`, while equality falls through
  (`bne` at 0x3ba24). Included word assignments at `sp -0x38` write `0xfffffb4e` and `0xfffffb4d` in
  separate arms (`str` at 0x3ba94, `str` at 0x3bac0). The unnamed `0x3cdd8` call carries `[sp-20]`,
  unknown, unknown, unknown (`bl` at 0x3bc68). Port activation protocol remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3b9dc | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x3b9e4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x3b9ec | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x3b9f4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x3b9f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3ba0c | load | 4 | `abs 0x3bcb8` | `-` |
| `ldr` at 0x3ba10 | load | 4 | `abs 0x1321ac` | `-` |
| `str` at 0x3ba18 | store | 4 | `sp -0x34` | `?` |
| `ldrb` at 0x3ba1c | load | 1 | `sp -0x15` | `-` |
| `str` at 0x3ba2c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x3ba30 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3ba34 | load | 4 | `[sp-52] +0x8` | `-` |
| `str` at 0x3ba38 | store | 4 | `sp -0x30` | `[[sp-52]+8]` |
| `ldrb` at 0x3ba40 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3ba44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3ba5c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3ba68 | store | 4 | `sp -0x38` | `ret(0x3d8bc)` |
| `ldr` at 0x3ba70 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x3ba74 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3ba7c | load | 4 | `[sp-52] +0x4` | `-` |
| `ldrb` at 0x3ba84 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3ba90 | load | 4 | `abs 0x3bcbc` | `-` |
| `str` at 0x3ba94 | store | 4 | `sp -0x38` | `0xfffffb4e` |
| `ldr` at 0x3ba9c | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x3baa0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3baa8 | load | 4 | `[sp-52] +0x4` | `-` |
| `ldrb` at 0x3bab0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3babc | load | 4 | `abs 0x3bcc0` | `-` |
| `str` at 0x3bac0 | store | 4 | `sp -0x38` | `0xfffffb4d` |
| `ldrb` at 0x3bac8 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x3bacc | store | 4 | `sp -0x2c` | `[sp-21]:1` |
| `ldrb` at 0x3bad0 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x3bad8 | store | 4 | `sp -0x30` | `[sp-21]:1+1` |
| `ldr` at 0x3badc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3bae0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x3bae4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3bae8 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x3baf4 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x3bb00 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3bb04 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3bb18 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldrh` at 0x3bb20 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x3bb30 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3bb34 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3bb48 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x3bb50 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x3bb60 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x3bb6c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3bb70 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3bb84 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldrh` at 0x3bb8c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x3bb9c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3bba0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3bbb4 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x3bbbc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3bbcc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3bbd0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3bbd8 | load | 4 | `[sp-52] +0x4` | `-` |
| `ldrb` at 0x3bbe0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3bbec | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3bbf0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3bc04 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x3bc0c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3bc1c | store | 4 | `sp -0x24` | `ret(0x3db08)` |
| `ldr` at 0x3bc20 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3bc2c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3bc38 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3bc3c | store | 4 | `sp -0x20` | `ret(0x3db08)` |
| `ldr` at 0x3bc40 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3bc44 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3bc58 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x3bc60 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3bc70 | store | 4 | `sp -0x24` | `ret(0x3cdd8)` |
| `ldr` at 0x3bc74 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3bc80 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3bc8c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3bc90 | store | 4 | `sp -0x20` | `ret(0x3cdd8)` |
| `ldr` at 0x3bc94 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3bc9c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x3bca4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3bca8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x3bcac | load | 4 | `sp -0x38` | `-` |
| `ldm` at 0x3bcb4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3bcb4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3bcb4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3ba4c | `(unknown callee) 0x3d824` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0x3ba60 | `(unknown callee) 0x3d8bc` | `[sp-20]` | `?` | `?` | `ret(0x3d824)` |
| `bl` at 0x3bc14 | `(unknown callee) 0x3db08` | `?` | `?` | `?` | `?` |
| `bl` at 0x3bc68 | `(unknown callee) 0x3cdd8` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3ba24 | `(r1 & 0xff) != 0xff` | `(r1 & 0xff) == 0xff` |
| `bne` at 0x3ba58 | `ret(0x3d824) != 0x0` | `ret(0x3d824) == 0x0` |
| `bne` at 0x3ba8c | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x3bab8 | `? > 0x1 (unsigned, 32-bit)` | `? <= 0x1 (unsigned, 32-bit)` |
| `bge` at 0x3baf0 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x3bafc | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x3bb2c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x3bb58 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x3bb68 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x3bb98 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x3bbc4 | `Z == 0` | `not (Z == 0)` |
| `bls` at 0x3bbe8 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x3bc28 | `ret(0x3db08) == 0x0` | `ret(0x3db08) != 0x0` |
| `bne` at 0x3bc34 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x3bc7c | `ret(0x3cdd8) == 0x0` | `ret(0x3cdd8) != 0x0` |
| `bne` at 0x3bc88 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3bcb4 | `r0` | `?` |

## `portDownInternal`

- `portDownInternal`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3bcc4 to 0x3bf68.
  Established. The condition `(r1 & 0xff) != 0xff` takes `0x3bd38`, while equality falls through
  (`bne` at 0x3bd1c). The unnamed `0x3ddec` call has four unknown candidates (`bl` at 0x3bf0c). A
  word at `sp -0x24` receives `ret(0x3ddec)` (`str` at 0x3bf14). Port deactivation behavior remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3bcd4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x3bcdc | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x3bce4 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x3bcec | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x3bcf0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3bd04 | load | 4 | `abs 0x3bf5c` | `-` |
| `ldr` at 0x3bd08 | load | 4 | `abs 0x1321ac` | `-` |
| `str` at 0x3bd10 | store | 4 | `sp -0x34` | `?` |
| `ldrb` at 0x3bd14 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x3bd24 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x3bd28 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3bd2c | load | 4 | `[sp-52] +0x8` | `-` |
| `str` at 0x3bd30 | store | 4 | `sp -0x30` | `[[sp-52]+8]` |
| `ldrb` at 0x3bd38 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3bd3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3bd54 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3bd60 | store | 4 | `sp -0x38` | `ret(0x3d8bc)` |
| `ldr` at 0x3bd68 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x3bd6c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3bd74 | load | 4 | `[sp-52] +0x4` | `-` |
| `ldrb` at 0x3bd7c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3bd88 | load | 4 | `abs 0x3bf60` | `-` |
| `str` at 0x3bd8c | store | 4 | `sp -0x38` | `0xfffffb4e` |
| `ldr` at 0x3bd94 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x3bd98 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3bda0 | load | 4 | `[sp-52] +0x4` | `-` |
| `ldrb` at 0x3bda8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3bdb4 | load | 4 | `abs 0x3bf64` | `-` |
| `str` at 0x3bdb8 | store | 4 | `sp -0x38` | `0xfffffb4d` |
| `ldrb` at 0x3bdc0 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x3bdc4 | store | 4 | `sp -0x2c` | `[sp-21]:1` |
| `ldrb` at 0x3bdc8 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x3bdd0 | store | 4 | `sp -0x30` | `[sp-21]:1+1` |
| `ldr` at 0x3bdd4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3bdd8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x3bddc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3bde0 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x3bdec | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x3bdf8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3bdfc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3be10 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldrh` at 0x3be18 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x3be28 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3be2c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3be40 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x3be48 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x3be58 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x3be64 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3be68 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3be7c | load | 4 | `[sp-52] +0x0` | `-` |
| `ldrh` at 0x3be84 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x3be94 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3be98 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3beac | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x3beb4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3bec4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3bec8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3bed0 | load | 4 | `[sp-52] +0x4` | `-` |
| `ldrb` at 0x3bed8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3bee4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3bee8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3befc | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x3bf04 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3bf14 | store | 4 | `sp -0x24` | `ret(0x3ddec)` |
| `ldr` at 0x3bf18 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3bf24 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3bf30 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3bf34 | store | 4 | `sp -0x20` | `ret(0x3ddec)` |
| `ldr` at 0x3bf38 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3bf40 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x3bf48 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3bf4c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x3bf50 | load | 4 | `sp -0x38` | `-` |
| `ldm` at 0x3bf58 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3bf58 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3bf58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3bd44 | `(unknown callee) 0x3d824` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0x3bd58 | `(unknown callee) 0x3d8bc` | `[sp-20]` | `?` | `?` | `ret(0x3d824)` |
| `bl` at 0x3bf0c | `(unknown callee) 0x3ddec` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3bd1c | `(r1 & 0xff) != 0xff` | `(r1 & 0xff) == 0xff` |
| `bne` at 0x3bd50 | `ret(0x3d824) != 0x0` | `ret(0x3d824) == 0x0` |
| `bne` at 0x3bd84 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x3bdb0 | `? > 0x1 (unsigned, 32-bit)` | `? <= 0x1 (unsigned, 32-bit)` |
| `bge` at 0x3bde8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x3bdf4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x3be24 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x3be50 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x3be60 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x3be90 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x3bebc | `Z == 0` | `not (Z == 0)` |
| `bls` at 0x3bee0 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x3bf20 | `ret(0x3ddec) == 0x0` | `ret(0x3ddec) != 0x0` |
| `bne` at 0x3bf2c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3bf58 | `r0` | `?` |

## `portName`

- `portName`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3c3f8 to 0x3c474.
  Established. The unnamed `0x3d824` call carries entry `r0`, `(r1 & 0xff)`, entry `r2`, `(r1 & 0xff)` (`bl` at 0x3c420).
  A zero call result takes `0x3c458`, while nonzero falls through (`beq` at 0x3c434). A word at `sp -0x20` receives `0xf602c` in the included fallback arm (`str` at 0x3c45c). Port naming rules remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3c408 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x3c410 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x3c414 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3c418 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3c428 | store | 4 | `sp -0x1c` | `ret(0x3d824)` |
| `ldr` at 0x3c42c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c438 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c43c | load | 4 | `ret(0x3d824) +0x8` | `-` |
| `ldr` at 0x3c448 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c44c | load | 4 | `ret(0x3d824) +0x8` | `-` |
| `str` at 0x3c450 | store | 4 | `sp -0x20` | `[ret(0x3d824)+8]` |
| `ldr` at 0x3c458 | load | 4 | `abs 0x3c470` | `-` |
| `str` at 0x3c45c | store | 4 | `sp -0x20` | `0xf602c` |
| `ldr` at 0x3c460 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x3c46c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3c46c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3c46c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3c420 | `(unknown callee) 0x3d824` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3c434 | `ret(0x3d824) == 0x0` | `ret(0x3d824) != 0x0` |
| `beq` at 0x3c444 | `[ret(0x3d824)+8] == 0x0` | `[ret(0x3d824)+8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3c46c | `r0` | `?` |

## `portState`

- `portState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3c474 to 0x3c560.
  Established. Four word assignments at `sp -0x34`, `sp -0x30`, `sp -0x2c`, `sp -0x28` receive
  `[0xf6078]`, `[0xf607c]`, `[0xf6080]`, `[0xf6084]` (`stm` at 0x3c4c4). The signed condition `(r1 & 0xff) >= [[sp-28]+8]` takes `0x3c508`, while smaller values fall through (`bge` at 0x3c4e0). An
  included word assignment at `sp -0x38` writes `0xf602c` (`str` at 0x3c50c). State label meanings
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3c484 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x3c48c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x3c490 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3c4a4 | load | 4 | `abs 0x3c554` | `-` |
| `ldr` at 0x3c4a8 | load | 4 | `abs 0x1321ac` | `-` |
| `str` at 0x3c4b0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x3c4b4 | load | 4 | `abs 0x3c558` | `-` |
| `ldm` at 0x3c4c0 | load | 4 | `abs 0xf6078` | `-` |
| `ldm` at 0x3c4c0 | load | 4 | `abs 0xf607c` | `-` |
| `ldm` at 0x3c4c0 | load | 4 | `abs 0xf6080` | `-` |
| `ldm` at 0x3c4c0 | load | 4 | `abs 0xf6084` | `-` |
| `stm` at 0x3c4c4 | store | 4 | `sp -0x34` | `[0xf6078]` |
| `stm` at 0x3c4c4 | store | 4 | `sp -0x30` | `[0xf607c]` |
| `stm` at 0x3c4c4 | store | 4 | `sp -0x2c` | `[0xf6080]` |
| `stm` at 0x3c4c4 | store | 4 | `sp -0x28` | `[0xf6084]` |
| `ldm` at 0x3c4c8 | load | 4 | `abs 0xf6088` | `-` |
| `ldm` at 0x3c4c8 | load | 4 | `abs 0xf608c` | `-` |
| `stm` at 0x3c4cc | store | 4 | `sp -0x24` | `[0xf6088]` |
| `stm` at 0x3c4cc | store | 4 | `sp -0x20` | `[0xf608c]` |
| `ldrb` at 0x3c4d0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3c4d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c4d8 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x3c4e4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c4e8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3c4f0 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x3c4f8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3c508 | load | 4 | `abs 0x3c55c` | `-` |
| `str` at 0x3c50c | store | 4 | `sp -0x38` | `0xf602c` |
| `ldr` at 0x3c514 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c518 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3c520 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x3c528 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3c540 | load | 4 | `(unknown)` | `-` |
| `str` at 0x3c544 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x3c548 | load | 4 | `sp -0x38` | `-` |
| `ldm` at 0x3c550 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3c550 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3c550 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x3c4e0 | `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)` |
| `bhi` at 0x3c500 | `? > 0x5 (unsigned, 32-bit)` | `? <= 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3c550 | `r0` | `?` |

## `portQuery`

- `portQuery`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3c560 to 0x3c914.
  Established. The unnamed `0x3b968` call carries entry `r0`, `sp-36`, entry `r2`, `sp-36` (`bl` at 0x3c58c). A nonzero result takes `0x3c5cc`, while zero falls through (`bne` at 0x3c5a0). Included
  word assignments through `[sp-28] +0x0` write `ret(0x1943c)` in separate arms (`str` at 0x3c5b4,
  `str` at 0x3c5f0). Query result formatting remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3c570 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x3c574 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x3c57c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x3c584 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3c594 | store | 4 | `sp -0x30` | `ret(0x3b968)` |
| `ldr` at 0x3c598 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3c5a4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3c5b4 | store | 4 | `[sp-28] +0x0` | `ret(0x1943c)` |
| `ldr` at 0x3c5b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c5bc | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3c5c0 | load | 4 | `abs 0x3c8e4` | `-` |
| `ldr` at 0x3c5cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c5d0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x3c5f0 | store | 4 | `[sp-28] +0x0` | `ret(0x1943c)` |
| `ldr` at 0x3c5f4 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x3c5f8 | store | 4 | `sp -0x34` | `[[sp-28]]` |
| `ldr` at 0x3c5fc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3c600 | load | 4 | `abs 0x3c8e8` | `-` |
| `ldr` at 0x3c608 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3c618 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x3c620 | store | 4 | `sp -0x34` | `([sp-52] + ret(0x19538))` |
| `str` at 0x3c628 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x3c62c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3c630 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3c63c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3c648 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3c658 | store | 4 | `sp -0x20` | `ret(0x3bf68)` |
| `ldr` at 0x3c65c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3c668 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3c66c | load | 4 | `abs 0x3c8ec` | `-` |
| `ldr` at 0x3c678 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3c68c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3c694 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3c698 | load | 4 | `abs 0x3c8f0` | `-` |
| `ldr` at 0x3c69c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3c6a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3c6b4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3c6c8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3c6d0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3c6d4 | load | 4 | `abs 0x3c8f4` | `-` |
| `ldr` at 0x3c6d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3c6e8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3c6fc | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x3c704 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x3c714 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3c728 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3c730 | store | 4 | `sp -0x38` | `?` |
| `ldrb` at 0x3c734 | load | 1 | `sp -0x28` | `-` |
| `str` at 0x3c738 | store | 4 | `sp -0x3c` | `[sp-40]:1` |
| `ldrb` at 0x3c73c | load | 1 | `sp -0x28` | `-` |
| `ldr` at 0x3c750 | load | 4 | `abs 0x3c8f8` | `-` |
| `str` at 0x3c754 | store | 4 | `sp -0x40` | `0xf6154` |
| `ldr` at 0x3c75c | load | 4 | `abs 0x3c8fc` | `-` |
| `str` at 0x3c760 | store | 4 | `sp -0x40` | `0xf6158` |
| `ldr` at 0x3c764 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x3c768 | store | 4 | `sp -0x6c` | `[sp-64]` |
| `ldr` at 0x3c76c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3c770 | load | 4 | `abs 0x3c900` | `-` |
| `ldr` at 0x3c774 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3c778 | load | 4 | `[sp-56] +0x8` | `-` |
| `ldr` at 0x3c77c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3c788 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3c79c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3c7a4 | store | 4 | `sp -0x44` | `?` |
| `ldrb` at 0x3c7a8 | load | 1 | `sp -0x28` | `-` |
| `str` at 0x3c7ac | store | 4 | `sp -0x48` | `[sp-40]:1` |
| `ldrb` at 0x3c7b0 | load | 1 | `sp -0x28` | `-` |
| `ldr` at 0x3c7c4 | load | 4 | `abs 0x3c904` | `-` |
| `str` at 0x3c7c8 | store | 4 | `sp -0x4c` | `0xf618c` |
| `ldr` at 0x3c7d0 | load | 4 | `abs 0x3c908` | `-` |
| `str` at 0x3c7d4 | store | 4 | `sp -0x4c` | `0xf6194` |
| `ldr` at 0x3c7d8 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x3c7dc | store | 4 | `sp -0x6c` | `[sp-76]` |
| `ldrb` at 0x3c7e0 | load | 1 | `sp -0x28` | `-` |
| `ldr` at 0x3c7f4 | load | 4 | `abs 0x3c904` | `-` |
| `str` at 0x3c7f8 | store | 4 | `sp -0x50` | `0xf618c` |
| `ldr` at 0x3c800 | load | 4 | `abs 0x3c908` | `-` |
| `str` at 0x3c804 | store | 4 | `sp -0x50` | `0xf6194` |
| `ldr` at 0x3c808 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x3c80c | store | 4 | `sp -0x68` | `[sp-80]` |
| `ldrb` at 0x3c810 | load | 1 | `sp -0x28` | `-` |
| `ldrb` at 0x3c824 | load | 1 | `sp -0x28` | `-` |
| `ldr` at 0x3c838 | load | 4 | `abs 0x3c90c` | `-` |
| `str` at 0x3c83c | store | 4 | `sp -0x58` | `0xf619c` |
| `ldr` at 0x3c844 | load | 4 | `abs 0x3c8fc` | `-` |
| `str` at 0x3c848 | store | 4 | `sp -0x58` | `0xf6158` |
| `ldr` at 0x3c84c | load | 4 | `sp -0x58` | `-` |
| `str` at 0x3c850 | store | 4 | `sp -0x54` | `[sp-88]` |
| `ldr` at 0x3c858 | load | 4 | `abs 0x3c8f8` | `-` |
| `str` at 0x3c85c | store | 4 | `sp -0x54` | `0xf6154` |
| `ldr` at 0x3c860 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x3c864 | store | 4 | `sp -0x64` | `?` |
| `ldrb` at 0x3c868 | load | 1 | `sp -0x28` | `-` |
| `ldr` at 0x3c87c | load | 4 | `abs 0x3c8f8` | `-` |
| `str` at 0x3c880 | store | 4 | `sp -0x5c` | `0xf6154` |
| `ldr` at 0x3c888 | load | 4 | `abs 0x3c8fc` | `-` |
| `str` at 0x3c88c | store | 4 | `sp -0x5c` | `0xf6158` |
| `ldr` at 0x3c890 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x3c894 | store | 4 | `sp -0x60` | `[sp-92]` |
| `ldr` at 0x3c898 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3c89c | load | 4 | `abs 0x3c910` | `-` |
| `ldr` at 0x3c8a0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x3c8a4 | load | 4 | `[sp-68] +0x8` | `-` |
| `ldr` at 0x3c8a8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3c8b0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3c8c0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x3c8c8 | store | 4 | `sp -0x34` | `([sp-52] + ret(0x19538))` |
| `ldr` at 0x3c8cc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3c8d4 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldm` at 0x3c8e0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x3c8e0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3c8e0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3c8e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3c58c | `(unknown callee) 0x3b968` | `r0` | `sp-36` | `r2` | `sp-36` |
| `bl` at 0x3c5ac | `(unknown callee) 0x1943c` | `0x50` | `?` | `?` | `ret(0x3b968)` |
| `bl` at 0x3c5c4 | `(unknown callee) 0x19808` | `[[sp-28]]` | `0xf6090` | `?` | `[sp-28]` |
| `bl` at 0x3c5e8 | `(unknown callee) 0x1943c` | `?` | `?` | `ret(0x3b968)` | `?` |
| `bl` at 0x3c604 | `(unknown callee) 0x197a8` | `[[sp-28]]` | `0xf60a8` | `?` | `[[sp-28]]` |
| `bl` at 0x3c60c | `(unknown callee) 0x19538` | `[sp-52]` | `?` | `?` | `?` |
| `bl` at 0x3c650 | `(unknown callee) 0x3bf68` | `[sp-24]` | `?` | `sp-40` | `?` |
| `bl` at 0x3c6a0 | `(unknown callee) 0x197a8` | `[sp-52]` | `0xf6120` | `?` | `?` |
| `bl` at 0x3c6ac | `(unknown callee) 0x1f8e0` | `ret(0x3bf68)` | `?` | `ret(0x3bf68)` | `0xfffffb4e` |
| `bl` at 0x3c6e0 | `(unknown callee) 0x197a8` | `[sp-52]` | `0xf6134` | `?` | `ret(0x1f8e0)` |
| `bl` at 0x3c780 | `(unknown callee) 0x197a8` | `[sp-52]` | `0xf615c` | `[[sp-56]+8]` | `[sp-40]:1` |
| `bl` at 0x3c8ac | `(unknown callee) 0x197a8` | `[sp-52]` | `0xf61a0` | `[[sp-68]+8]` | `[sp-40]:1` |
| `bl` at 0x3c8b4 | `(unknown callee) 0x19538` | `[sp-52]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3c5a0 | `ret(0x3b968) != 0x0` | `ret(0x3b968) == 0x0` |
| `bge` at 0x3c638 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x3c664 | `ret(0x3bf68) == 0x0` | `ret(0x3bf68) != 0x0` |
| `bne` at 0x3c674 | `ret(0x3bf68) != 0xfffffb4e` | `ret(0x3bf68) == 0xfffffb4e` |
| `beq` at 0x3c710 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x3c74c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x3c7c0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x3c7f0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x3c820 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x3c834 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x3c878 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3c8e0 | `r0` | `?` |

## `fp_led_on_raw`

- `fp_led_on_raw`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4aef8 to 0x4b008.
  Established. The unnamed `0xca584` call carries `0x0`, unknown, unknown, unknown (`bl` at 0x4afa0); its zero result takes `0x4afc0`, while nonzero falls through (`beq` at 0x4afb4). The
  byte at `0x132486` receives an unknown value on an included path (`strb` at 0x4afd8). Raw panel
  enable behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4af08 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4af10 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x4af14 | load | 4 | `abs 0x4affc` | `-` |
| `ldr` at 0x4af18 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4af1c | load | 4 | `abs 0x132480` | `-` |
| `ldr` at 0x4af28 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4af30 | load | 4 | `abs 0x4b000` | `-` |
| `ldr` at 0x4af34 | load | 4 | `abs 0x13247c` | `-` |
| `ldrb` at 0x4af3c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x4af40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4af50 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4af58 | load | 4 | `abs 0x4b000` | `-` |
| `ldr` at 0x4af5c | load | 4 | `abs 0x13247c` | `-` |
| `ldrb` at 0x4af64 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x4af68 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4af70 | load | 4 | `abs 0x4b000` | `-` |
| `ldr` at 0x4af74 | load | 4 | `abs 0x13247c` | `-` |
| `ldrb` at 0x4af7c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x4af80 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4af88 | load | 4 | `abs 0x4b000` | `-` |
| `ldr` at 0x4af8c | load | 4 | `abs 0x13247c` | `-` |
| `ldrb` at 0x4af94 | load | 1 | `(unknown)` | `-` |
| `str` at 0x4afa8 | store | 4 | `sp -0x1c` | `ret(0xca584)` |
| `ldr` at 0x4afac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4afb8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4afbc | store | 4 | `sp -0x18` | `ret(0xca584)` |
| `ldr` at 0x4afc0 | load | 4 | `abs 0x4b004` | `-` |
| `ldr` at 0x4afc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4afc8 | load | 4 | `abs 0x4b004` | `-` |
| `ldrb` at 0x4afd0 | load | 1 | `abs 0x132486` | `-` |
| `strb` at 0x4afd8 | store | 1 | `abs 0x132486` | `?` |
| `ldr` at 0x4afdc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4afe4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x4afec | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x4aff8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4aff8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4aff8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4afa0 | `(unknown callee) 0xca584` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x4af24 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x4af4c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x4afb4 | `ret(0xca584) == 0x0` | `ret(0xca584) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4aff8 | `r0` | `[sp-24]` |
