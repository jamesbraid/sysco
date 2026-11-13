# ARM power-bay state and request paths in 1.44.0

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
`PBaySetState` and `PBayCmd` each have one unresolved computed transfer.
Their tables cover known edges only. The other thirteen routines report none.
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/powerbay-state-requests-facts.txt) retain all tool sections.

## `PBayInit`

- `PBayInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9cb7c to 0x9cc80.
  Established. An included halfword store clears absolute `0x133ae8` (`strh` at 0x9cbd4).
  A word assignment places `0x133ae0` at `[0x133af0] +0x8` (`str` at 0x9cbe4).
  The unnamed `0x2066c` call carries `[0x133af0]+4`, `0x112c70`, `0x9d898`, `0x1` (`bl` at 0x9cc30).
  Power-bay initialization ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x9cb94 | load | 4 | `abs 0x9cc70` | `-` |
| `str` at 0x9cba4 | store | 4 | `abs 0x133af0` | `ret(0x1943c)` |
| `ldr` at 0x9cba8 | load | 4 | `abs 0x9cc70` | `-` |
| `ldr` at 0x9cbac | load | 4 | `abs 0x133af0` | `-` |
| `ldr` at 0x9cbbc | load | 4 | `abs 0x9cc74` | `-` |
| `ldr` at 0x9cbcc | load | 4 | `abs 0x9cc74` | `-` |
| `strh` at 0x9cbd4 | store | 2 | `abs 0x133ae8` | `0x0` |
| `ldr` at 0x9cbd8 | load | 4 | `abs 0x9cc70` | `-` |
| `ldr` at 0x9cbdc | load | 4 | `abs 0x133af0` | `-` |
| `ldr` at 0x9cbe0 | load | 4 | `abs 0x9cc74` | `-` |
| `str` at 0x9cbe4 | store | 4 | `[0x133af0] +0x8` | `0x133ae0` |
| `ldr` at 0x9cbe8 | load | 4 | `abs 0x9cc70` | `-` |
| `ldr` at 0x9cbec | load | 4 | `abs 0x133af0` | `-` |
| `ldr` at 0x9cbf4 | load | 4 | `abs 0x9cc70` | `-` |
| `ldr` at 0x9cbf8 | load | 4 | `abs 0x133af0` | `-` |
| `str` at 0x9cbfc | store | 4 | `sp -0x2c` | `[0x133af0]` |
| `str` at 0x9cc04 | store | 4 | `sp -0x28` | `0xa00` |
| `str` at 0x9cc0c | store | 4 | `sp -0x24` | `0x19` |
| `str` at 0x9cc14 | store | 4 | `sp -0x20` | `0x28` |
| `str` at 0x9cc1c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x9cc24 | load | 4 | `abs 0x9cc78` | `-` |
| `ldr` at 0x9cc28 | load | 4 | `abs 0x9cc7c` | `-` |
| `str` at 0x9cc38 | store | 4 | `sp -0x18` | `ret(0x2066c)` |
| `ldr` at 0x9cc3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9cc5c | load | 4 | `abs 0x9cc70` | `-` |
| `ldr` at 0x9cc60 | load | 4 | `abs 0x133af0` | `-` |
| `ldm` at 0x9cc6c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x9cc6c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9cc6c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9cc6c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9cb90 | `(unknown callee) 0x9cc80` | `0x1` | `r1` | `r2` | `r3` |
| `bl` at 0x9cb9c | `(unknown callee) 0x1943c` | `0x104` | `?` | `?` | `?` |
| `bl` at 0x9cbb8 | `(unknown callee) 0x19748` | `[0x133af0]` | `0x0` | `0x104` | `0x133af0` |
| `bl` at 0x9cbc8 | `(unknown callee) 0x19748` | `0x133ae0` | `0x0` | `0x10` | `?` |
| `bl` at 0x9cc30 | `(unknown callee) 0x2066c` | `[0x133af0]+4` | `0x112c70` | `0x9d898` | `0x1` |
| `bl` at 0x9cc50 | `(unknown callee) 0x9d324` | `?` | `0x7` | `?` | `ret(0x2066c)` |
| `bl` at 0x9cc58 | `(unknown callee) 0x9cc80` | `0x5` | `?` | `?` | `?` |
| `bl` at 0x9cc64 | `(unknown callee) 0x1970c` | `[0x133af0]` | `?` | `?` | `0x133af0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x9cc44 | `ret(0x2066c) == 0x0` | `ret(0x2066c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9cc6c | `r0` | `?` |

## `PBaySetState`

- `PBaySetState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9cc80 to 0x9cd5c.
  Established. The absolute word at `0x133adc` receives entry `r0` (`str` at 0x9cc9c).
  An included word store assigns `0x112ce4` to absolute `0x12d820` (`str` at 0x9cd34).
  The integer return candidate preserves entry `r0` (`ldm` at 0x9cd38).
  Power-bay state dispatch remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9cc90 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x9cc94 | load | 4 | `abs 0x9cd3c` | `-` |
| `ldr` at 0x9cc98 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9cc9c | store | 4 | `abs 0x133adc` | `r0` |
| `ldr` at 0x9cca0 | load | 4 | `abs 0x9cd3c` | `-` |
| `ldr` at 0x9cca4 | load | 4 | `abs 0x133adc` | `-` |
| `ldrls` at 0x9ccac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9cd2c | load | 4 | `abs 0x9cd40` | `-` |
| `ldr` at 0x9cd30 | load | 4 | `abs 0x9cd58` | `-` |
| `str` at 0x9cd34 | store | 4 | `abs 0x12d820` | `0x112ce4` |
| `ldm` at 0x9cd38 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x9cd38 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9cd38 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9cd38 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x9ccac | `[0x133adc] <= 0x5 (unsigned, 32-bit)` | `[0x133adc] > 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9cd38 | `r0` | `r0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x9ccac).

## `PBayGetState`

- `PBayGetState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9cd5c to 0x9cd7c.
  Established. A word load reads absolute `0x133adc` (`ldr` at 0x9cd6c).
  The integer return record is `[0x133adc]` (`ldm` at 0x9cd74).
  Power-bay state encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x9cd68 | load | 4 | `abs 0x9cd78` | `-` |
| `ldr` at 0x9cd6c | load | 4 | `abs 0x133adc` | `-` |
| `ldm` at 0x9cd74 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9cd74 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9cd74 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9cd74 | `r0` | `[0x133adc]` |

## `PBayInitFunc`

- `PBayInitFunc`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9d898 to 0x9d95c.
  Established. The unnamed `0x20bb4` call carries entry `r1+164`, `0x112da4`, `0x0`, unknown (`bl` at 0x9d8d0);
  its zero result takes `0x9d8f8`, while nonzero falls through (`beq` at 0x9d8dc).
  The halfword assignment to `[[sp-36]+8] +0x4` has unknown data (`strh` at 0x9d938).
  Power-bay startup synchronization remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9d8a8 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x9d8ac | store | 4 | `sp -0x20` | `r1` |
| `ldr` at 0x9d8b0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9d8b4 | store | 4 | `sp -0x24` | `r1` |
| `ldr` at 0x9d8b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9d8c4 | load | 4 | `abs 0x9d958` | `-` |
| `ldr` at 0x9d900 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9d904 | load | 4 | `[sp-36] +0x8` | `-` |
| `strh` at 0x9d938 | store | 2 | `[[sp-36]+8] +0x4` | `?` |
| `ldm` at 0x9d954 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x9d954 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x9d954 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9d954 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9d954 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9d8d0 | `(unknown callee) 0x20bb4` | `r1+164` | `0x112da4` | `0x0` | `?` |
| `bl` at 0x9d8e8 | `(unknown callee) 0x9d324` | `?` | `0x11` | `?` | `ret(0x20bb4)` |
| `bl` at 0x9d8f0 | `(unknown callee) 0x9cc80` | `0x5` | `?` | `?` | `?` |
| `bl` at 0x9d8fc | `(unknown callee) 0x9cc80` | `0x1` | `?` | `?` | `ret(0x20bb4)` |
| `bl` at 0x9d90c | `(unknown callee) 0x1d444` | `0x1` | `?` | `?` | `[sp-36]` |
| `bl` at 0x9d920 | `(unknown callee) 0x1d444` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x9d944 | `(unknown callee) 0x9d324` | `?` | `0x2` | `?` | `?` |
| `bl` at 0x9d94c | `(unknown callee) 0x9cc80` | `0x5` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x9d8dc | `ret(0x20bb4) == 0x0` | `ret(0x20bb4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9d954 | `r0` | `ret(0x9cc80)` |

## `PBayInitIcmb`

- `PBayInitIcmb`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9d95c to 0x9dc3c.
  Established. Included halfword stores clear `[[sp-24]+8] +0x6` and later assign `0x4400` there (`strh` at 0x9d984, `strh` at 0x9da9c).
  The unnamed `0x9dc3c` call carries `[[sp-24]+8]`, `sp-436`, `sp-436`, `[sp-24]` (`bl` at 0x9da04);
  its zero result takes `0x9da74`, while nonzero falls through (`beq` at 0x9da18).
  ICMB discovery request meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9d96c | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x9d978 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9d97c | load | 4 | `[sp-24] +0x8` | `-` |
| `strh` at 0x9d984 | store | 2 | `[[sp-24]+8] +0x6` | `0x0` |
| `ldr` at 0x9d988 | load | 4 | `abs 0x9dc38` | `-` |
| `strh` at 0x9d994 | store | 2 | `sp -0x1b4` | `0x0` |
| `str` at 0x9d99c | store | 4 | `sp -0x1b0` | `0x10` |
| `str` at 0x9d9a4 | store | 4 | `sp -0x11c` | `0x0` |
| `str` at 0x9d9ac | store | 4 | `sp -0x118` | `0xff` |
| `str` at 0x9d9b4 | store | 4 | `sp -0x28` | `0x3` |
| `str` at 0x9d9bc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x9d9c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9d9c8 | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0x9d9d8 | store | 4 | `sp -0x1b0` | `0x2` |
| `str` at 0x9d9e0 | store | 4 | `sp -0x28` | `0x3` |
| `str` at 0x9d9e8 | store | 4 | `sp -0x24` | `0x3` |
| `str` at 0x9d9f0 | store | 4 | `sp -0x11c` | `0x0` |
| `ldr` at 0x9d9f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9d9fc | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0x9da0c | store | 4 | `sp -0x1c` | `ret(0x9dc3c)` |
| `ldr` at 0x9da10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9da1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9da20 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x9da24 | load | 4 | `sp -0x2c` | `-` |
| `strh` at 0x9da2c | store | 2 | `[[sp-24]+8] +0x6` | `(ret(0xa1aa8) & 0xffff)` |
| `str` at 0x9da34 | store | 4 | `sp -0x1b0` | `0x12` |
| `str` at 0x9da3c | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x9da40 | load | 4 | `abs 0x9dc38` | `-` |
| `ldr` at 0x9da44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9da48 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrh` at 0x9da4c | load | 2 | `[[sp-24]+8] +0x6` | `-` |
| `strh` at 0x9da54 | store | 2 | `sp -0x1b4` | `([[[sp-24]+8]+6]:2 & 0xffff)` |
| `str` at 0x9da5c | store | 4 | `sp -0x11c` | `0x0` |
| `ldr` at 0x9da60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9da68 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x9da74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9da78 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrh` at 0x9da7c | load | 2 | `[[sp-24]+8] +0x6` | `-` |
| `ldr` at 0x9da90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9da94 | load | 4 | `[sp-24] +0x8` | `-` |
| `strh` at 0x9da9c | store | 2 | `[[sp-24]+8] +0x6` | `0x4400` |
| `ldr` at 0x9daa4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9daa8 | load | 4 | `[sp-24] +0x8` | `-` |
| `strb` at 0x9dab0 | store | 1 | `[[sp-24]+8] +0x0` | `0x0` |
| `str` at 0x9dab8 | store | 4 | `sp -0x1b0` | `0x13` |
| `str` at 0x9dac0 | store | 4 | `sp -0x24` | `0x3` |
| `str` at 0x9dac8 | store | 4 | `sp -0x11c` | `0x0` |
| `ldr` at 0x9dacc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9dad4 | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0x9dae4 | store | 4 | `sp -0x1c` | `ret(0x9dc3c)` |
| `ldr` at 0x9dae8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9db00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9db04 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x9db08 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x9db0c | load | 1 | `[sp-44] +0x0` | `-` |
| `strb` at 0x9db10 | store | 1 | `[[sp-24]+8] +0x0` | `([[sp-44]]:1 & 0xff)` |
| `str` at 0x9db18 | store | 4 | `sp -0x1b0` | `0x20` |
| `str` at 0x9db20 | store | 4 | `sp -0x11c` | `0x0` |
| `str` at 0x9db28 | store | 4 | `sp -0x118` | `0x0` |
| `ldr` at 0x9db30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9db34 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0x9db38 | load | 1 | `[[sp-24]+8] +0x0` | `-` |
| `strb` at 0x9db40 | store | 1 | `(unknown)` | `([[[sp-24]+8]]:1 & 0xff)` |
| `str` at 0x9db48 | store | 4 | `sp -0x110` | `0x0` |
| `strb` at 0x9db54 | store | 1 | `sp -0x10c` | `0x0` |
| `str` at 0x9db5c | store | 4 | `sp -0xc8` | `0x0` |
| `ldr` at 0x9db60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9db68 | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0x9db78 | store | 4 | `sp -0x1c` | `ret(0x9dc3c)` |
| `ldr` at 0x9db7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9db88 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9db90 | store | 4 | `sp -0x1b8` | `ret(0xa1808)` |
| `ldr` at 0x9db94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9db98 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x9db9c | load | 4 | `sp -0x1b8` | `-` |
| `ldrb` at 0x9dba0 | load | 1 | `ret(0xa1808) +0x1` | `-` |
| `strb` at 0x9dba4 | store | 1 | `[[sp-24]+8] +0x1` | `([ret(0xa1808)+1]:1 & 0xff)` |
| `ldr` at 0x9dba8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9dbac | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x9dbb0 | load | 4 | `sp -0x1b8` | `-` |
| `ldrb` at 0x9dbb4 | load | 1 | `[sp-440] +0x2` | `-` |
| `strb` at 0x9dbb8 | store | 1 | `[[sp-24]+8] +0x2` | `([[sp-440]+2]:1 & 0xff)` |
| `ldr` at 0x9dbbc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9dbc0 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x9dbc4 | load | 4 | `sp -0x1b8` | `-` |
| `ldrb` at 0x9dbc8 | load | 1 | `[sp-440] +0x3` | `-` |
| `strb` at 0x9dbcc | store | 1 | `[[sp-24]+8] +0x3` | `([[sp-440]+3]:1 & 0xff)` |
| `ldr` at 0x9dbd0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9dbd4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0x9dbd8 | load | 1 | `[[sp-24]+8] +0x1` | `-` |
| `ldr` at 0x9dbe4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9dbe8 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0x9dbec | load | 1 | `[[sp-24]+8] +0x1` | `-` |
| `ldr` at 0x9dbf8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9dbfc | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0x9dc00 | load | 1 | `[[sp-24]+8] +0x2` | `-` |
| `ldr` at 0x9dc0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9dc10 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x9dc18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9dc1c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldm` at 0x9dc34 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x9dc34 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9dc34 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9dc34 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9d974 | `(unknown callee) 0x9cc80` | `0x1` | `r1` | `r2` | `r3` |
| `bl` at 0x9d9d0 | `(unknown callee) 0x9dc3c` | `[[sp-24]+8]` | `sp-436` | `sp-436` | `[sp-24]` |
| `bl` at 0x9da04 | `(unknown callee) 0x9dc3c` | `[[sp-24]+8]` | `sp-436` | `sp-436` | `[sp-24]` |
| `bl` at 0x9da28 | `(unknown callee) 0xa1aa8` | `[sp-44]` | `?` | `?` | `[sp-24]` |
| `bl` at 0x9da70 | `(unknown callee) 0x9dc3c` | `[[sp-24]+8]` | `sp-436` | `sp-436` | `[sp-24]` |
| `bl` at 0x9da8c | `(unknown callee) 0x9cc80` | `0x5` | `?` | `?` | `[[[sp-24]+8]+6]:2` |
| `bl` at 0x9dadc | `(unknown callee) 0x9dc3c` | `[[sp-24]+8]` | `sp-436` | `sp-436` | `[sp-24]` |
| `bl` at 0x9daf8 | `(unknown callee) 0x9cc80` | `0x5` | `?` | `?` | `ret(0x9dc3c)` |
| `bl` at 0x9db70 | `(unknown callee) 0x9dc3c` | `[[sp-24]+8]` | `sp-436` | `sp-436` | `[sp-24]` |
| `bl` at 0x9db8c | `(unknown callee) 0xa1808` | `[sp-44]` | `?` | `?` | `ret(0x9dc3c)` |
| `bl` at 0x9dbf4 | `(unknown callee) 0xa11bc` | `[[[sp-24]+8]+1]:1` | `?` | `[[sp-24]+8]` | `[[[sp-24]+8]+1]:1` |
| `bl` at 0x9dc14 | `(unknown callee) 0x9e0f0` | `[[sp-24]+8]` | `?` | `?` | `[sp-24]` |
| `bl` at 0x9dc24 | `(unknown callee) 0x9fef8` | `[[sp-24]+8]` | `0x0` | `?` | `[sp-24]` |
| `bl` at 0x9dc2c | `(unknown callee) 0x9cc80` | `0x2` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x9da18 | `ret(0x9dc3c) == 0x0` | `ret(0x9dc3c) != 0x0` |
| `bne` at 0x9da84 | `[[[sp-24]+8]+6]:2 != 0x0` | `[[[sp-24]+8]+6]:2 == 0x0` |
| `bne` at 0x9daf0 | `ret(0x9dc3c) != 0x0` | `ret(0x9dc3c) == 0x0` |
| `beq` at 0x9db84 | `ret(0x9dc3c) == 0x0` | `ret(0x9dc3c) != 0x0` |
| `beq` at 0x9dbe0 | `[[[sp-24]+8]+1]:1 == 0x0` | `[[[sp-24]+8]+1]:1 != 0x0` |
| `beq` at 0x9dc08 | `[[[sp-24]+8]+2]:1 == 0x0` | `[[[sp-24]+8]+2]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9dc34 | `r0` | `ret(0x9cc80)` |

## `PBayCmd`

- `PBayCmd`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9cd7c to 0x9d324.
  Established. An included word store clears entry `r1 +0x0` (`str` at 0x9cda4).
  The unnamed `0x20cd0` call carries `[0x133af0]+164`, `0x0`, `0x4`, `[0x133af0]+164` (`bl` at 0x9cec0).
  Zero `ret(0x8156c)` takes `0x9cdd4`, while nonzero falls through (`beq` at 0x9cdc4).
  Power-bay command dispatch remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9cd8c | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x9cd90 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x9cd98 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x9cd9c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9cda4 | store | 4 | `r1 +0x0` | `0x0` |
| `ldr` at 0x9cda8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9cdac | load | 4 | `[sp-28] +0x70` | `-` |
| `ldr` at 0x9cdb0 | load | 4 | `[[sp-28]+112] +0x0` | `-` |
| `ldr` at 0x9cdb4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9cdcc | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x9cdd4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9cddc | load | 4 | `[sp-28] +0x70` | `-` |
| `ldr` at 0x9cde4 | load | 4 | `[[sp-28]+112] +0x4` | `-` |
| `ldr` at 0x9ce00 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9ce10 | store | 4 | `[sp-32] +0x0` | `ret(0x1943c)` |
| `ldr` at 0x9ce14 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9ce18 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x9ce1c | load | 4 | `abs 0x9d2fc` | `-` |
| `ldr` at 0x9ce24 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9ce28 | store | 4 | `sp -0x34` | `[sp-36]` |
| `ldr` at 0x9ce30 | load | 4 | `abs 0x9d300` | `-` |
| `ldr` at 0x9ce34 | load | 4 | `abs 0x12d820` | `-` |
| `ldr` at 0x9ce40 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9ce50 | store | 4 | `[sp-32] +0x0` | `ret(0x1943c)` |
| `ldr` at 0x9ce54 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9ce58 | load | 4 | `abs 0x9d300` | `-` |
| `ldr` at 0x9ce5c | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x9ce60 | load | 4 | `abs 0x12d820` | `-` |
| `str` at 0x9ce6c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x9ce74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9ce7c | load | 4 | `[sp-28] +0x70` | `-` |
| `ldr` at 0x9ce84 | load | 4 | `[[sp-28]+112] +0x4` | `-` |
| `ldr` at 0x9ce90 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9ce9c | store | 4 | `sp -0x24` | `ret(0x9d824)` |
| `ldr` at 0x9cea0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9cea4 | store | 4 | `sp -0x34` | `ret(0x9d824)` |
| `ldr` at 0x9ceac | load | 4 | `abs 0x9d304` | `-` |
| `ldr` at 0x9ceb0 | load | 4 | `abs 0x133af0` | `-` |
| `str` at 0x9cec8 | store | 4 | `sp -0x2c` | `ret(0x20cd0)` |
| `ldr` at 0x9cecc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9ced8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9cedc | store | 4 | `sp -0x34` | `[sp-36]` |
| `ldr` at 0x9ceec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9cef4 | load | 4 | `[sp-28] +0x70` | `-` |
| `ldr` at 0x9cefc | load | 4 | `[[sp-28]+112] +0x4` | `-` |
| `ldrls` at 0x9cf08 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9d2ac | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x9d2b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9d2b8 | load | 4 | `[sp-28] +0x70` | `-` |
| `ldr` at 0x9d2c0 | load | 4 | `[[sp-28]+112] +0x4` | `-` |
| `ldr` at 0x9d2d4 | load | 4 | `abs 0x9d304` | `-` |
| `ldr` at 0x9d2d8 | load | 4 | `abs 0x133af0` | `-` |
| `ldr` at 0x9d2e8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9d2ec | store | 4 | `sp -0x34` | `[sp-36]` |
| `ldr` at 0x9d2f0 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x9d2f8 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x9d2f8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x9d2f8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9d2f8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9d2f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9cdb8 | `(unknown callee) 0x8156c` | `[[[sp-28]+112]]` | `[sp-32]` | `r1` | `[[sp-28]+112]` |
| `bl` at 0x9cdf0 | `(unknown callee) 0x9cd5c` | `ret(0x8156c)` | `?` | `0x4` | `[[[sp-28]+112]+4]` |
| `bl` at 0x9ce08 | `(unknown callee) 0x1943c` | `0x50` | `?` | `?` | `ret(0x9cd5c)` |
| `bl` at 0x9ce20 | `(unknown callee) 0x19808` | `[[sp-32]]` | `0x112cf8` | `?` | `[sp-32]` |
| `bl` at 0x9ce48 | `(unknown callee) 0x1943c` | `0x50` | `?` | `0x4` | `[0x12d820]` |
| `bl` at 0x9ce64 | `(unknown callee) 0x19808` | `[[sp-32]]` | `[0x12d820]` | `0x12d820` | `[sp-32]` |
| `bl` at 0x9ce94 | `(unknown callee) 0x9d824` | `[sp-32]` | `?` | `0x4` | `[[[sp-28]+112]+4]` |
| `bl` at 0x9cec0 | `(unknown callee) 0x20cd0` | `[0x133af0]+164` | `0x0` | `0x4` | `[0x133af0]+164` |
| `bl` at 0x9cee8 | `(unknown callee) 0x9cc80` | `0x3` | `?` | `?` | `ret(0x20cd0)` |
| `bl` at 0x9d2d0 | `(unknown callee) 0x9cc80` | `0x2` | `?` | `0x4` | `[[[sp-28]+112]+4]` |
| `bl` at 0x9d2e4 | `(unknown callee) 0x2105c` | `[0x133af0]+164` | `?` | `?` | `[0x133af0]+164` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x9cdc4 | `ret(0x8156c) == 0x0` | `ret(0x8156c) != 0x0` |
| `bne` at 0x9cdec | `[[[sp-28]+112]+4] != 0xf` | `[[[sp-28]+112]+4] == 0xf` |
| `beq` at 0x9cdfc | `ret(0x9cd5c) == 0x5` | `ret(0x9cd5c) != 0x5` |
| `beq` at 0x9ce3c | `[0x12d820] == 0x0` | `[0x12d820] != 0x0` |
| `bne` at 0x9ce8c | `[[[sp-28]+112]+4] != 0x10` | `[[[sp-28]+112]+4] == 0x10` |
| `beq` at 0x9ced4 | `ret(0x20cd0) == 0x0` | `ret(0x20cd0) != 0x0` |
| `ldrls` at 0x9cf08 | `[[[sp-28]+112]+4]-1 <= 0xf (unsigned, 32-bit)` | `[[[sp-28]+112]+4]-1 > 0xf (unsigned, 32-bit)` |
| `beq` at 0x9d2c8 | `[[[sp-28]+112]+4] == 0xf` | `[[[sp-28]+112]+4] != 0xf` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9d2f8 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x9cf08).

## `PBayError`

- `PBayError`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9d324 to 0x9d364.
  Established. The unnamed `0x222f0` call carries `[0x12eeb8]`, `0x112d70`, entry `r0`, entry `r1` (`bl` at 0x9d350);
  its result is the integer return candidate (`ldm` at 0x9d358).
  Error category interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9d334 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9d338 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x9d33c | load | 4 | `abs 0x9d35c` | `-` |
| `ldr` at 0x9d340 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x9d344 | load | 4 | `abs 0x9d360` | `-` |
| `ldr` at 0x9d348 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9d34c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x9d358 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9d358 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9d358 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9d350 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0x112d70` | `r0` | `r1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9d358 | `r0` | `ret(0x222f0)` |

## `PBayPollHwStatus`

- `PBayPollHwStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9e63c to 0x9eccc.
  Established. Nonzero `ret(0x1d444)` takes `0x9e67c`, while zero falls through (`bne` at 0x9e674).
  Equality `ret(0x9cd5c) == 0x2` takes `0x9e690`, while inequality falls through (`beq` at 0x9e688).
  The unnamed `0x20cd0` call carries `[sp-20]+164`, `0x0`, unknown, `[sp-20]+164` (`bl` at 0x9e6a0).
  Hardware status polling event meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9e64c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9e654 | store | 4 | `sp -0x4c` | `0x1` |
| `str` at 0x9e65c | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0x9e668 | store | 4 | `sp -0x18` | `ret(0x1d444)` |
| `ldr` at 0x9e66c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9e690 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9e6c0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x9e6c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9e6d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9e6dc | load | 4 | `[sp-20] +0x8` | `-` |
| `str` at 0x9e6ec | store | 4 | `sp -0x24` | `ret(0xa149c)` |
| `ldr` at 0x9e6f0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9e70c | store | 4 | `sp -0x20` | `?` |
| `str` at 0x9e714 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x9e71c | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0x9e720 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9e730 | store | 4 | `sp -0xac` | `0x0` |
| `ldr` at 0x9e738 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9e740 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x9e748 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x9e758 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x9e76c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9e770 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9e778 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x9e77c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9e780 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9e790 | load | 4 | `abs 0x9eca4` | `-` |
| `str` at 0x9e794 | store | 4 | `sp -0xac` | `0x112df8` |
| `ldr` at 0x9e798 | load | 4 | `abs 0x9eca8` | `-` |
| `str` at 0x9e79c | store | 4 | `sp -0x44` | `0xbbf` |
| `str` at 0x9e7a4 | store | 4 | `sp -0x48` | `0x7500` |
| `ldr` at 0x9e7ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9e7b0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9e7c0 | load | 4 | `abs 0x9ecac` | `-` |
| `str` at 0x9e7c4 | store | 4 | `sp -0xac` | `0x112e04` |
| `str` at 0x9e7cc | store | 4 | `sp -0x44` | `0xbc0` |
| `str` at 0x9e7d4 | store | 4 | `sp -0x48` | `0x7400` |
| `ldr` at 0x9e7d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9e7e4 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x9e7f8 | load | 4 | `abs 0x9ecb0` | `-` |
| `ldr` at 0x9e7fc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9e800 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x9e80c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x9e810 | store | 4 | `sp -0xb4` | `[sp-76]` |
| `ldr` at 0x9e814 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x9e818 | store | 4 | `sp -0xb0` | `[sp-80]` |
| `ldr` at 0x9e820 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9e824 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9e830 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9e838 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x9e840 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9e844 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9e850 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x9e868 | store | 4 | `sp -0x58` | `0x3` |
| `str` at 0x9e870 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0x9e874 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9e880 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9e88c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9e894 | load | 4 | `[sp-20] +0x8` | `-` |
| `str` at 0x9e8a4 | store | 4 | `sp -0x24` | `ret(0xa149c)` |
| `ldr` at 0x9e8a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9e8bc | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x9e8cc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x9e8dc | load | 4 | `abs 0x9ecb4` | `-` |
| `ldr` at 0x9e8e0 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0x9e8ec | store | 1 | `[sp-44] +0x133af8` | `0x0` |
| `str` at 0x9e8f4 | store | 4 | `sp -0x58` | `0x3` |
| `ldr` at 0x9e8f8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9e900 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x9e90c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9e91c | load | 4 | `abs 0x9ecb4` | `-` |
| `ldr` at 0x9e920 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x9e928 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x9e92c | load | 1 | `[sp-44] +0x133af8` | `-` |
| `strb` at 0x9e934 | store | 1 | `sp -0x39` | `?` |
| `ldrb` at 0x9e938 | load | 1 | `sp -0x39` | `-` |
| `str` at 0x9e948 | store | 4 | `sp -0x58` | `0x3` |
| `ldr` at 0x9e94c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9e954 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x9e960 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x9e970 | load | 1 | `sp -0x39` | `-` |
| `ldrb` at 0x9e974 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x9e97c | store | 1 | `sp -0x39` | `?` |
| `ldrb` at 0x9e980 | load | 1 | `sp -0x39` | `-` |
| `ldr` at 0x9e990 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9e9a4 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x9e9b0 | store | 4 | `sp -0x58` | `[sp-88]-1` |
| `str` at 0x9e9c0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x9e9c8 | load | 4 | `abs 0x9ecb8` | `-` |
| `str` at 0x9e9cc | store | 4 | `sp -0xac` | `0x112e20` |
| `ldr` at 0x9e9d0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9e9dc | store | 4 | `sp -0x44` | `[sp-44]+427` |
| `ldr` at 0x9e9e0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9e9e8 | store | 4 | `sp -0x48` | `[sp-44]+28672` |
| `ldrb` at 0x9e9f0 | load | 1 | `sp -0x39` | `-` |
| `ldr` at 0x9ea00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9ea10 | load | 4 | `abs 0x9ecbc` | `-` |
| `str` at 0x9ea14 | store | 4 | `sp -0xac` | `0x112e3c` |
| `ldr` at 0x9ea18 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9ea24 | store | 4 | `sp -0x44` | `[sp-44]+433` |
| `ldr` at 0x9ea28 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9ea30 | store | 4 | `sp -0x48` | `[sp-44]+28928` |
| `ldrb` at 0x9ea38 | load | 1 | `sp -0x39` | `-` |
| `ldr` at 0x9ea48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9ea58 | load | 4 | `abs 0x9ecc0` | `-` |
| `str` at 0x9ea5c | store | 4 | `sp -0xac` | `0x112e54` |
| `ldr` at 0x9ea60 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9ea6c | store | 4 | `sp -0x44` | `[sp-44]+439` |
| `ldr` at 0x9ea70 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9ea78 | store | 4 | `sp -0x48` | `[sp-44]+29184` |
| `str` at 0x9ea84 | store | 4 | `sp -0xac` | `0x0` |
| `ldr` at 0x9ea88 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x9ea9c | load | 4 | `abs 0x9ecb0` | `-` |
| `ldr` at 0x9eaa0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9eaa4 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x9eab0 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x9eab4 | store | 4 | `sp -0xb4` | `[sp-76]` |
| `ldr` at 0x9eab8 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x9eabc | store | 4 | `sp -0xb0` | `[sp-80]` |
| `ldr` at 0x9eac4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9eac8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9ead4 | load | 4 | `abs 0x9ecb4` | `-` |
| `ldr` at 0x9ead8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9eae4 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x9eaf4 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x9eaf8 | store | 1 | `[sp-44] +0x133af8` | `?` |
| `str` at 0x9eb00 | store | 4 | `sp -0x58` | `0x3` |
| `ldr` at 0x9eb04 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9eb0c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x9eb14 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9eb28 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0x9eb2c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9eb3c | store | 4 | `sp -0x58` | `0x2` |
| `str` at 0x9eb44 | store | 4 | `sp -0x54` | `0x1` |
| `ldr` at 0x9eb48 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x9eb58 | store | 4 | `sp -0x54` | `0x0` |
| `ldr` at 0x9eb5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9eb60 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0x9eb64 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9eb70 | store | 4 | `sp -0x40` | `ret(0x9efc8)` |
| `ldr` at 0x9eb74 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x9eb80 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x9eb90 | load | 4 | `abs 0x9ecc4` | `-` |
| `ldr` at 0x9eb94 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9eb9c | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0x9eba0 | load | 1 | `[sp-44] +0x133aff` | `-` |
| `strb` at 0x9ebb0 | store | 1 | `sp -0x39` | `?` |
| `ldrb` at 0x9ebb4 | load | 1 | `sp -0x39` | `-` |
| `ldr` at 0x9ebc4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x9ebd8 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x9ebe4 | store | 4 | `sp -0x58` | `[sp-88]-1` |
| `str` at 0x9ebf4 | store | 4 | `sp -0x54` | `0x1` |
| `ldr` at 0x9ec0c | load | 4 | `abs 0x9ecc8` | `-` |
| `ldr` at 0x9ec10 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9ec18 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9ec24 | store | 4 | `sp -0x44` | `[sp-44]+445` |
| `ldr` at 0x9ec28 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9ec30 | store | 4 | `sp -0x48` | `[sp-44]+29440` |
| `ldr` at 0x9ec38 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x9ec3c | store | 4 | `sp -0xb4` | `[sp-76]` |
| `ldr` at 0x9ec40 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x9ec44 | store | 4 | `sp -0xb0` | `[sp-80]` |
| `ldr` at 0x9ec4c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9ec50 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9ec5c | load | 4 | `abs 0x9ecc4` | `-` |
| `ldr` at 0x9ec60 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9ec68 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x9ec6c | store | 1 | `[sp-44] +0x133aff` | `?` |
| `ldr` at 0x9ec74 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9ec7c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x9ec8c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x9eca0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9eca0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9eca0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9e664 | `(unknown callee) 0x1d444` | `0x55` | `r1` | `r2` | `0x0` |
| `bl` at 0x9e67c | `(unknown callee) 0x9cd5c` | `ret(0x1d444)` | `?` | `?` | `ret(0x1d444)` |
| `bl` at 0x9e6a0 | `(unknown callee) 0x20cd0` | `[sp-20]+164` | `0x0` | `?` | `[sp-20]+164` |
| `bl` at 0x9e6b8 | `(unknown callee) 0x9cc80` | `0x3` | `?` | `?` | `ret(0x20cd0)` |
| `bl` at 0x9e6e4 | `(unknown callee) 0xa149c` | `[[sp-20]+8]` | `sp-56` | `sp-56` | `[sp-20]` |
| `bl` at 0x9e700 | `(unknown callee) 0x1d444` | `0x69` | `?` | `?` | `ret(0xa149c)` |
| `bl` at 0x9e804 | `(unknown callee) 0x197a8` | `sp-168` | `0x112e0c` | `[sp-44]` | `[sp-172]` |
| `bl` at 0x9e82c | `(unknown callee) 0x6f93c` | `0x0` | `[sp-72]` | `[sp-68]` | `sp-168` |
| `bl` at 0x9e860 | `(unknown callee) 0x1b5f0` | `0x69` | `?` | `[sp-32]` | `?` |
| `bl` at 0x9e89c | `(unknown callee) 0xa149c` | `[[sp-20]+8]` | `sp-56` | `sp-56` | `[sp-20]` |
| `bl` at 0x9eaa8 | `(unknown callee) 0x197a8` | `sp-168` | `0x112e0c` | `[sp-44]` | `?` |
| `bl` at 0x9ead0 | `(unknown callee) 0x6f93c` | `0x0` | `[sp-72]` | `[sp-68]` | `sp-168` |
| `bl` at 0x9eb68 | `(unknown callee) 0x9efc8` | `[[sp-20]+8]` | `?` | `?` | `[sp-20]` |
| `bl` at 0x9ebfc | `(unknown callee) 0x9e0c8` | `0x64` | `[sp-88]` | `[sp-88]-1` | `0x1` |
| `bl` at 0x9ec14 | `(unknown callee) 0x197a8` | `sp-168` | `0x112e64` | `[sp-44]` | `sp-168` |
| `bl` at 0x9ec58 | `(unknown callee) 0x6f93c` | `0x0` | `[sp-44]+29440` | `[sp-44]+445` | `sp-168` |
| `bl` at 0x9ec88 | `(unknown callee) 0x9cc80` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x9ec98 | `(unknown callee) 0x2105c` | `[sp-20]+164` | `?` | `?` | `[sp-20]+164` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x9e674 | `ret(0x1d444) != 0x0` | `ret(0x1d444) == 0x0` |
| `beq` at 0x9e688 | `ret(0x9cd5c) == 0x2` | `ret(0x9cd5c) != 0x2` |
| `beq` at 0x9e6ac | `ret(0x20cd0) == 0x0` | `ret(0x20cd0) != 0x0` |
| `beq` at 0x9e6d0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9e6f8 | `ret(0xa149c) == 0x0` | `ret(0xa149c) != 0x0` |
| `bgt` at 0x9e728 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x9e768 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x9e78c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x9e7bc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9e7e0 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `beq` at 0x9e7ec | `[sp-172] == 0x0` | `[sp-172] != 0x0` |
| `beq` at 0x9e84c | `Z == 1` | `not (Z == 1)` |
| `bgt` at 0x9e87c | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0x9e888 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x9e8b0 | `ret(0xa149c) != 0x0` | `ret(0xa149c) == 0x0` |
| `beq` at 0x9e8d8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x9e940 | `[sp-57]:1 != 0x0` | `[sp-57]:1 == 0x0` |
| `beq` at 0x9e98c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9e99c | `? == 0x0` | `? != 0x0` |
| `ble` at 0x9e9b8 | `[sp-88] <= 0x0 (signed, 32-bit)` | `[sp-88] > 0x0 (signed, 32-bit)` |
| `beq` at 0x9e9fc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9ea0c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9ea44 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9ea54 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9ea90 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x9eb20 | `? == 0x0` | `? != 0x0` |
| `bgt` at 0x9eb34 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x9eb50 | `Z == 1` | `not (Z == 1)` |
| `blt` at 0x9eb7c | `ret(0x9efc8) < 0x0 (signed, 32-bit)` | `ret(0x9efc8) >= 0x0 (signed, 32-bit)` |
| `bne` at 0x9eb8c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x9ebbc | `[sp-57]:1 != 0x0` | `[sp-57]:1 == 0x0` |
| `beq` at 0x9ebd0 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x9ebec | `[sp-88] <= 0x0 (signed, 32-bit)` | `[sp-88] > 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9eca0 | `r0` | `?` |

## `PBaySdrGetDpsStateHdr`

- `PBaySdrGetDpsStateHdr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9eccc to 0x9ed18.
  Established. The unnamed `0x19808` call carries `ret(0x1943c)`, `0x112e94`, unknown, `ret(0x1943c)` (`bl` at 0x9ecf4).
  The subsequent unnamed `0x195e0` call carries `[sp-20]`, `0x112eb4`, unknown, unknown (`bl` at 0x9ed00).
  Supply-state header fields remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9ece8 | store | 4 | `sp -0x14` | `ret(0x1943c)` |
| `ldr` at 0x9ecec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9ecf0 | load | 4 | `abs 0x9ed10` | `-` |
| `ldr` at 0x9ecf8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9ecfc | load | 4 | `abs 0x9ed14` | `-` |
| `ldr` at 0x9ed04 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x9ed0c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x9ed0c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9ed0c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9ed0c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9ece0 | `(unknown callee) 0x1943c` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0x9ecf4 | `(unknown callee) 0x19808` | `ret(0x1943c)` | `0x112e94` | `?` | `ret(0x1943c)` |
| `bl` at 0x9ed00 | `(unknown callee) 0x195e0` | `[sp-20]` | `0x112eb4` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9ed0c | `r0` | `[sp-20]` |

## `PBaySdrGetDpsState`

- `PBaySdrGetDpsState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9ed18 to 0x9ef7c.
  Established. An included byte store writes `([sp-24]+48 & 0xff)` at `[sp-40] +0x1` (`strb` at 0x9edd4).
  The unnamed `0x9dc3c` call carries `[sp-20]`, `sp-456`, unknown, `sp-456` (`bl` at 0x9ee00);
  its nonzero result takes `0x9ee30`, while zero falls through (`bne` at 0x9ee14).
  Supply-state response bits remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9ed28 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9ed2c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x9ed30 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x9ed34 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x9ed3c | store | 4 | `r2 +0x0` | `0x0` |
| `ldr` at 0x9ed40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9ed4c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9ed60 | store | 4 | `sp -0x1d0` | `0x0` |
| `ldr` at 0x9ed68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9ed70 | load | 4 | `abs 0x9ef48` | `-` |
| `str` at 0x9ed78 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x9ed7c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x9ed80 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x9ed90 | store | 4 | `sp -0x1d0` | `0x0` |
| `str` at 0x9eda4 | store | 4 | `sp -0x28` | `ret(0x1943c)` |
| `ldr` at 0x9eda8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9edac | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9edb0 | store | 4 | `[sp-28] +0x0` | `ret(0x1943c)` |
| `ldr` at 0x9edb4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9edb8 | load | 4 | `abs 0x9ef4c` | `-` |
| `ldr` at 0x9edc4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9edcc | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x9edd4 | store | 1 | `[sp-40] +0x1` | `([sp-24]+48 & 0xff)` |
| `ldr` at 0x9eddc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x9ede0 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x9ede4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9edf8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9ee08 | store | 4 | `sp -0x1cc` | `ret(0x9dc3c)` |
| `ldr` at 0x9ee0c | load | 4 | `sp -0x1cc` | `-` |
| `ldr` at 0x9ee18 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9ee1c | load | 4 | `abs 0x9ef50` | `-` |
| `str` at 0x9ee28 | store | 4 | `sp -0x1d0` | `?` |
| `ldr` at 0x9ee30 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x9ee38 | store | 4 | `sp -0x24` | `ret(0xa1808)` |
| `ldr` at 0x9ee3c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x9ee40 | load | 1 | `ret(0xa1808) +0x2` | `-` |
| `strb` at 0x9ee44 | store | 1 | `sp -0x2d` | `([ret(0xa1808)+2]:1 & 0xff)` |
| `ldrb` at 0x9ee48 | load | 1 | `sp -0x2d` | `-` |
| `ldrb` at 0x9ee58 | load | 1 | `sp -0x2d` | `-` |
| `ldr` at 0x9ee68 | load | 4 | `abs 0x9ef54` | `-` |
| `str` at 0x9ee6c | store | 4 | `sp -0x2c` | `0x112ef0` |
| `ldr` at 0x9ee74 | load | 4 | `abs 0x9ef58` | `-` |
| `str` at 0x9ee78 | store | 4 | `sp -0x2c` | `0x112ef8` |
| `ldr` at 0x9ee7c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9ee80 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x9ee88 | load | 1 | `sp -0x2d` | `-` |
| `ldr` at 0x9ee98 | load | 4 | `abs 0x9ef5c` | `-` |
| `str` at 0x9ee9c | store | 4 | `sp -0x2c` | `0x112f00` |
| `ldr` at 0x9eea4 | load | 4 | `abs 0x9ef60` | `-` |
| `str` at 0x9eea8 | store | 4 | `sp -0x2c` | `0x112f0c` |
| `ldr` at 0x9eeac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9eeb0 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x9eeb8 | load | 1 | `sp -0x2d` | `-` |
| `ldr` at 0x9eec8 | load | 4 | `abs 0x9ef64` | `-` |
| `str` at 0x9eecc | store | 4 | `sp -0x2c` | `0x112f18` |
| `ldr` at 0x9eed4 | load | 4 | `abs 0x9ef68` | `-` |
| `str` at 0x9eed8 | store | 4 | `sp -0x2c` | `0x112f24` |
| `ldr` at 0x9eedc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9eee0 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x9eee8 | load | 1 | `sp -0x2d` | `-` |
| `ldr` at 0x9eef8 | load | 4 | `abs 0x9ef6c` | `-` |
| `str` at 0x9eefc | store | 4 | `sp -0x2c` | `0x112f30` |
| `ldr` at 0x9ef04 | load | 4 | `abs 0x9ef70` | `-` |
| `str` at 0x9ef08 | store | 4 | `sp -0x2c` | `0x112f38` |
| `ldr` at 0x9ef0c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9ef10 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9ef1c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9ef20 | load | 4 | `abs 0x9ef74` | `-` |
| `ldr` at 0x9ef28 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9ef2c | load | 4 | `abs 0x9ef78` | `-` |
| `str` at 0x9ef38 | store | 4 | `sp -0x1d0` | `0x0` |
| `ldr` at 0x9ef3c | load | 4 | `sp -0x1d0` | `-` |
| `ldm` at 0x9ef44 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9ef44 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9ef44 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9ed9c | `(unknown callee) 0x1943c` | `0x50` | `r1` | `?` | `[[sp-32]]:1` |
| `bl` at 0x9edbc | `(unknown callee) 0x19808` | `[sp-40]` | `0x112ed8` | `[sp-28]` | `ret(0x1943c)` |
| `bl` at 0x9edf0 | `(unknown callee) 0xa0960` | `[sp-20]` | `sp-456` | `[[sp-32]]:1` | `[[sp-32]]:1` |
| `bl` at 0x9ee00 | `(unknown callee) 0x9dc3c` | `[sp-20]` | `sp-456` | `?` | `sp-456` |
| `bl` at 0x9ee20 | `(unknown callee) 0x195e0` | `[sp-40]` | `0x112ee0` | `?` | `ret(0x9dc3c)` |
| `bl` at 0x9ee34 | `(unknown callee) 0xa1808` | `[sp-64]` | `?` | `?` | `ret(0x9dc3c)` |
| `bl` at 0x9ee84 | `(unknown callee) 0x195e0` | `[sp-40]` | `[sp-44]` | `?` | `?` |
| `bl` at 0x9eeb4 | `(unknown callee) 0x195e0` | `[sp-40]` | `[sp-44]` | `?` | `?` |
| `bl` at 0x9eee4 | `(unknown callee) 0x195e0` | `[sp-40]` | `[sp-44]` | `?` | `?` |
| `bl` at 0x9ef14 | `(unknown callee) 0x195e0` | `[sp-40]` | `[sp-44]` | `?` | `?` |
| `bl` at 0x9ef24 | `(unknown callee) 0x195e0` | `[sp-40]` | `0x112f40` | `?` | `?` |
| `bl` at 0x9ef30 | `(unknown callee) 0x195e0` | `[sp-40]` | `0x112f4c` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x9ed48 | `[sp-24] <= 0x0 (signed, 32-bit)` | `[sp-24] > 0x0 (signed, 32-bit)` |
| `bgt` at 0x9ed54 | `[sp-24] > 0x6 (signed, 32-bit)` | `[sp-24] <= 0x6 (signed, 32-bit)` |
| `bne` at 0x9ed88 | `[[sp-32]]:1 != 0xff` | `[[sp-32]]:1 == 0xff` |
| `bne` at 0x9ee14 | `ret(0x9dc3c) != 0x0` | `ret(0x9dc3c) == 0x0` |
| `bne` at 0x9ee54 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x9ee64 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9ee94 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9eec4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9eef4 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9ef44 | `r0` | `?` |

## `PBaySdrGetMain`

- `PBaySdrGetMain`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9f1fc to 0x9f420.
  Established. Nonzero `[0x133b08]` takes `0x9f23c`, while zero falls through (`bne` at 0x9f22c).
  The unnamed `0xa149c` call carries `[sp-20]`, `sp-508`, entry `r1`, `sp-508` (`bl` at 0x9f248).
  An included byte assignment writes `([sp-500] & 0xff)` at `ret(0xa1808) +0x3` (`strb` at 0x9f394).
  Main power-bay response layout remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9f20c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9f210 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x9f214 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9f21c | store | 4 | `r1 +0x0` | `0x0` |
| `ldr` at 0x9f220 | load | 4 | `abs 0x9f404` | `-` |
| `ldr` at 0x9f224 | load | 4 | `abs 0x133b08` | `-` |
| `str` at 0x9f234 | store | 4 | `sp -0x200` | `0x0` |
| `ldr` at 0x9f240 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9f250 | store | 4 | `sp -0x1f4` | `ret(0xa149c)` |
| `ldr` at 0x9f254 | load | 4 | `sp -0x1f4` | `-` |
| `str` at 0x9f264 | store | 4 | `sp -0x200` | `0x0` |
| `str` at 0x9f278 | store | 4 | `sp -0x20` | `ret(0x1943c)` |
| `ldr` at 0x9f27c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9f280 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9f284 | store | 4 | `[sp-24] +0x0` | `ret(0x1943c)` |
| `ldr` at 0x9f288 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x9f290 | store | 1 | `[sp-32] +0x0` | `0x0` |
| `str` at 0x9f298 | store | 4 | `sp -0x1f0` | `0x0` |
| `ldr` at 0x9f29c | load | 4 | `abs 0x9f404` | `-` |
| `ldr` at 0x9f2a0 | load | 4 | `sp -0x1f0` | `-` |
| `ldr` at 0x9f2a4 | load | 4 | `abs 0x133b08` | `-` |
| `ldr` at 0x9f2b4 | load | 4 | `abs 0x9f408` | `-` |
| `ldr` at 0x9f2b8 | load | 4 | `sp -0x1f0` | `-` |
| `ldrb` at 0x9f2d0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x9f2d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f2e4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9f2f4 | store | 4 | `sp -0x1ec` | `ret(0x9dc3c)` |
| `ldr` at 0x9f2f8 | load | 4 | `sp -0x1ec` | `-` |
| `ldr` at 0x9f304 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9f308 | load | 4 | `abs 0x9f40c` | `-` |
| `str` at 0x9f314 | store | 4 | `sp -0x200` | `?` |
| `ldr` at 0x9f31c | load | 4 | `sp -0x1f0` | `-` |
| `ldr` at 0x9f330 | load | 4 | `abs 0x9f410` | `-` |
| `str` at 0x9f344 | store | 4 | `sp -0x24` | `ret(0xa0d9c)` |
| `ldr` at 0x9f348 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x9f34c | load | 1 | `ret(0xa0d9c) +0x0` | `-` |
| `ldr` at 0x9f358 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9f35c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9f368 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9f36c | load | 4 | `abs 0x9f414` | `-` |
| `ldr` at 0x9f374 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9f378 | load | 4 | `abs 0x9f418` | `-` |
| `ldr` at 0x9f380 | load | 4 | `sp -0x60` | `-` |
| `str` at 0x9f388 | store | 4 | `sp -0x1c` | `ret(0xa1808)` |
| `ldr` at 0x9f38c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f390 | load | 4 | `sp -0x1f4` | `-` |
| `strb` at 0x9f394 | store | 1 | `ret(0xa1808) +0x3` | `([sp-500] & 0xff)` |
| `ldr` at 0x9f398 | load | 4 | `sp -0x1f0` | `-` |
| `ldr` at 0x9f3ac | load | 4 | `abs 0x9f410` | `-` |
| `ldr` at 0x9f3b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f3c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9f3d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9f3d8 | load | 4 | `abs 0x9f41c` | `-` |
| `ldr` at 0x9f3e0 | load | 4 | `sp -0x1f0` | `-` |
| `str` at 0x9f3e8 | store | 4 | `sp -0x1f0` | `[sp-496]+1` |
| `str` at 0x9f3f4 | store | 4 | `sp -0x200` | `0x0` |
| `ldr` at 0x9f3f8 | load | 4 | `sp -0x200` | `-` |
| `ldm` at 0x9f400 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9f400 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9f400 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9f248 | `(unknown callee) 0xa149c` | `[sp-20]` | `sp-508` | `r1` | `sp-508` |
| `bl` at 0x9f270 | `(unknown callee) 0x1943c` | `0x100` | `?` | `?` | `ret(0xa149c)` |
| `bl` at 0x9f2dc | `(unknown callee) 0xa0960` | `[sp-20]` | `sp-488` | `?` | `?` |
| `bl` at 0x9f2ec | `(unknown callee) 0x9dc3c` | `[sp-20]` | `sp-488` | `?` | `sp-488` |
| `bl` at 0x9f30c | `(unknown callee) 0x195e0` | `[sp-32]` | `0x112ee0` | `?` | `ret(0x9dc3c)` |
| `bl` at 0x9f33c | `(unknown callee) 0xa0d9c` | `?` | `?` | `?` | `?` |
| `bl` at 0x9f360 | `(unknown callee) 0x195e0` | `[sp-32]` | `ret(0xa0d9c)` | `?` | `[ret(0xa0d9c)]:1` |
| `bl` at 0x9f370 | `(unknown callee) 0x195e0` | `[sp-32]` | `0x112fa0` | `?` | `[ret(0xa0d9c)]:1` |
| `bl` at 0x9f37c | `(unknown callee) 0x195e0` | `[sp-32]` | `0x112fb0` | `?` | `?` |
| `bl` at 0x9f384 | `(unknown callee) 0xa1808` | `[sp-96]` | `?` | `?` | `?` |
| `bl` at 0x9f3c0 | `(unknown callee) 0xa0cf4` | `[sp-28]` | `?` | `sp-76` | `?` |
| `bl` at 0x9f3d0 | `(unknown callee) 0x195e0` | `[sp-32]` | `sp-76` | `?` | `sp-76` |
| `bl` at 0x9f3dc | `(unknown callee) 0x195e0` | `[sp-32]` | `0x112f4c` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x9f22c | `[0x133b08] != 0x0` | `[0x133b08] == 0x0` |
| `bne` at 0x9f25c | `ret(0xa149c) != 0x0` | `ret(0xa149c) == 0x0` |
| `bge` at 0x9f2ac | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x9f300 | `ret(0x9dc3c) != 0x0` | `ret(0x9dc3c) == 0x0` |
| `beq` at 0x9f354 | `[ret(0xa0d9c)]:1 == 0x0` | `[ret(0xa0d9c)]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9f400 | `r0` | `?` |

## `PBaySdrGetDcPortStateHdr`

- `PBaySdrGetDcPortStateHdr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9ef7c to 0x9efc8.
  Established. The word at `sp -0x14` receives `ret(0x1943c)` (`str` at 0x9ef98).
  The unnamed `0x195e0` call carries `[sp-20]`, `0x112f64`, unknown, unknown (`bl` at 0x9efb0).
  DC-port state header meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9ef98 | store | 4 | `sp -0x14` | `ret(0x1943c)` |
| `ldr` at 0x9ef9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9efa0 | load | 4 | `abs 0x9efc0` | `-` |
| `ldr` at 0x9efa8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9efac | load | 4 | `abs 0x9efc4` | `-` |
| `ldr` at 0x9efb4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x9efbc | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x9efbc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9efbc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9efbc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9ef90 | `(unknown callee) 0x1943c` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0x9efa4 | `(unknown callee) 0x19808` | `ret(0x1943c)` | `0x112f50` | `?` | `ret(0x1943c)` |
| `bl` at 0x9efb0 | `(unknown callee) 0x195e0` | `[sp-20]` | `0x112f64` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9efbc | `r0` | `[sp-20]` |

## `PBaySdrGetDcPort`

- `PBaySdrGetDcPort`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9efc8 to 0x9f0ac.
  Established. Signed entry `r1 <= 0x0` takes `0x9effc`, while positive values fall through (`ble` at 0x9efe8).
  The unnamed `0x9dc3c` call carries `[sp-20]`, `sp-440`, unknown, `sp-440` (`bl` at 0x9f060).
  A byte read fetches `ret(0xa1808) +0x2` (`ldrb` at 0x9f094).
  DC-port response interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9efd8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9efdc | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x9efe0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9efec | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9f000 | store | 4 | `sp -0x1c0` | `?` |
| `ldr` at 0x9f008 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9f010 | load | 4 | `abs 0x9f0a8` | `-` |
| `str` at 0x9f018 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x9f01c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x9f020 | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0x9f030 | store | 4 | `sp -0x1c0` | `?` |
| `ldr` at 0x9f03c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x9f040 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x9f044 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f058 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9f068 | store | 4 | `sp -0x1bc` | `ret(0x9dc3c)` |
| `ldr` at 0x9f06c | load | 4 | `sp -0x1bc` | `-` |
| `str` at 0x9f07c | store | 4 | `sp -0x1c0` | `?` |
| `ldr` at 0x9f084 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x9f08c | store | 4 | `sp -0x20` | `ret(0xa1808)` |
| `ldr` at 0x9f090 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x9f094 | load | 1 | `ret(0xa1808) +0x2` | `-` |
| `str` at 0x9f098 | store | 4 | `sp -0x1c0` | `[ret(0xa1808)+2]:1` |
| `ldr` at 0x9f09c | load | 4 | `sp -0x1c0` | `-` |
| `ldm` at 0x9f0a4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9f0a4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9f0a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9f050 | `(unknown callee) 0xa0960` | `r0` | `sp-440` | `[[sp-28]]:1` | `[[sp-28]]:1` |
| `bl` at 0x9f060 | `(unknown callee) 0x9dc3c` | `[sp-20]` | `sp-440` | `?` | `sp-440` |
| `bl` at 0x9f088 | `(unknown callee) 0xa1808` | `[sp-48]` | `?` | `?` | `ret(0x9dc3c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x9efe8 | `r1 <= 0x0 (signed, 32-bit)` | `r1 > 0x0 (signed, 32-bit)` |
| `bgt` at 0x9eff4 | `r1 > 0x8 (signed, 32-bit)` | `r1 <= 0x8 (signed, 32-bit)` |
| `bne` at 0x9f028 | `[[sp-28]]:1 != 0xff` | `[[sp-28]]:1 == 0xff` |
| `bne` at 0x9f074 | `ret(0x9dc3c) != 0x0` | `ret(0x9dc3c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9f0a4 | `r0` | `[sp-448]` |

## `PBaySdrGetDcPortState`

- `PBaySdrGetDcPortState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9f0ac to 0x9f1fc.
  Established. An included word assignment clears entry `r2 +0x0` (`str` at 0x9f0d0).
  A byte store writes `([sp-24]+48 & 0xff)` at `[sp-40] +0x3` (`strb` at 0x9f13c).
  Signed `[sp-36] >= 0x0` takes `0x9f164`, while negative values fall through (`bge` at 0x9f148).
  DC-port status flags remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9f0bc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9f0c0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x9f0c4 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x9f0c8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x9f0d0 | store | 4 | `r2 +0x0` | `0x0` |
| `ldr` at 0x9f0d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f0d8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9f0e4 | store | 4 | `sp -0x24` | `ret(0x9efc8)` |
| `ldr` at 0x9f0e8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9f0f8 | store | 4 | `sp -0x1cc` | `0x0` |
| `str` at 0x9f10c | store | 4 | `sp -0x28` | `ret(0x1943c)` |
| `ldr` at 0x9f110 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f114 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9f118 | store | 4 | `[sp-28] +0x0` | `ret(0x1943c)` |
| `ldr` at 0x9f11c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9f120 | load | 4 | `abs 0x9f1e0` | `-` |
| `ldr` at 0x9f12c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9f134 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x9f13c | store | 1 | `[sp-40] +0x3` | `([sp-24]+48 & 0xff)` |
| `ldr` at 0x9f140 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9f14c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9f150 | load | 4 | `abs 0x9f1e4` | `-` |
| `str` at 0x9f15c | store | 4 | `sp -0x1cc` | `?` |
| `ldr` at 0x9f164 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9f174 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9f178 | load | 4 | `abs 0x9f1e8` | `-` |
| `ldr` at 0x9f184 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9f188 | load | 4 | `abs 0x9f1ec` | `-` |
| `ldr` at 0x9f190 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9f1a0 | load | 4 | `abs 0x9f1f0` | `-` |
| `str` at 0x9f1a4 | store | 4 | `sp -0x1d0` | `0x112f30` |
| `ldr` at 0x9f1ac | load | 4 | `abs 0x9f1f4` | `-` |
| `str` at 0x9f1b0 | store | 4 | `sp -0x1d0` | `0x112f9c` |
| `ldr` at 0x9f1b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9f1b8 | load | 4 | `sp -0x1d0` | `-` |
| `ldr` at 0x9f1c0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9f1c4 | load | 4 | `abs 0x9f1f8` | `-` |
| `str` at 0x9f1d0 | store | 4 | `sp -0x1cc` | `0x0` |
| `ldr` at 0x9f1d4 | load | 4 | `sp -0x1cc` | `-` |
| `ldm` at 0x9f1dc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9f1dc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9f1dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9f0dc | `(unknown callee) 0x9efc8` | `[sp-20]` | `[sp-24]` | `r2` | `0x0` |
| `bl` at 0x9f104 | `(unknown callee) 0x1943c` | `0x50` | `?` | `?` | `ret(0x9efc8)` |
| `bl` at 0x9f124 | `(unknown callee) 0x19808` | `[sp-40]` | `0x112f80` | `[sp-28]` | `ret(0x1943c)` |
| `bl` at 0x9f154 | `(unknown callee) 0x195e0` | `[sp-40]` | `0x112ee0` | `[sp-40]+3` | `[sp-36]` |
| `bl` at 0x9f17c | `(unknown callee) 0x195e0` | `[sp-40]` | `0x112f88` | `[sp-40]+3` | `?` |
| `bl` at 0x9f18c | `(unknown callee) 0x195e0` | `[sp-40]` | `0x112f90` | `[sp-40]+3` | `?` |
| `bl` at 0x9f1bc | `(unknown callee) 0x195e0` | `[sp-40]` | `[sp-464]` | `?` | `?` |
| `bl` at 0x9f1c8 | `(unknown callee) 0x195e0` | `[sp-40]` | `0x112f4c` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x9f0f0 | `Z == 0` | `not (Z == 0)` |
| `bge` at 0x9f148 | `[sp-36] >= 0x0 (signed, 32-bit)` | `[sp-36] < 0x0 (signed, 32-bit)` |
| `beq` at 0x9f170 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9f19c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9f1dc | `r0` | `?` |

## `PBaySdrGetFruHdr`

- `PBaySdrGetFruHdr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9f420 to 0x9f46c.
  Established. The unnamed `0x1943c` call carries `0xa0`, entry `r1`, entry `r2`, entry `r3` (`bl` at 0x9f434).
  A subsequent unnamed `0x19808` call carries `ret(0x1943c)`, `0x112fb4`, unknown, `ret(0x1943c)` (`bl` at 0x9f448).
  FRU header layout remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9f43c | store | 4 | `sp -0x14` | `ret(0x1943c)` |
| `ldr` at 0x9f440 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f444 | load | 4 | `abs 0x9f464` | `-` |
| `ldr` at 0x9f44c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f450 | load | 4 | `abs 0x9f468` | `-` |
| `ldr` at 0x9f458 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x9f460 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x9f460 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9f460 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9f460 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9f434 | `(unknown callee) 0x1943c` | `0xa0` | `r1` | `r2` | `r3` |
| `bl` at 0x9f448 | `(unknown callee) 0x19808` | `ret(0x1943c)` | `0x112fb4` | `?` | `ret(0x1943c)` |
| `bl` at 0x9f454 | `(unknown callee) 0x195e0` | `[sp-20]` | `0x112ff8` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9f460 | `r0` | `[sp-20]` |
