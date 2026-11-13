# ARM voltage and power-state helpers in 1.44.0

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
Coprocessor transfer widths are unknown. Integer return records do not establish coprocessor results.
The computed transfer in powerMarginName remains unresolved (`ldrls` at 0x39c68);
its tables cover known edges only. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/voltage-power-state-facts.txt) retain all tool sections.

## `powerLowMargin`

- `powerLowMargin`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x39f08 to 0x39f34.
  Established. The byte through entry `r1 +0xf` is read (`ldrb` at 0x39f24). The return reports
  `[r1+15]:1` (`ldm` at 0x39f30). Low-margin record meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x39f18 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x39f1c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x39f20 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x39f24 | load | 1 | `r1 +0xf` | `-` |
| `ldm` at 0x39f30 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x39f30 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x39f30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x39f30 | `r0` | `[r1+15]:1` |

## `powerNormalMargin`

- `powerNormalMargin`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x39f34 to 0x39f60.
  Established. A byte through entry `r1 +0xd` is read (`ldrb` at 0x39f50). The return reports
  `[r1+13]:1` (`ldm` at 0x39f5c). Nominal-margin record interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x39f44 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x39f48 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x39f4c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x39f50 | load | 1 | `r1 +0xd` | `-` |
| `ldm` at 0x39f5c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x39f5c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x39f5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x39f5c | `r0` | `[r1+13]:1` |

## `powerHighMargin`

- `powerHighMargin`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x39f60 to 0x39f8c.
  Established. The byte through entry `r1 +0xe` is read (`ldrb` at 0x39f7c). The return reports
  `[r1+14]:1` (`ldm` at 0x39f88). High-margin field semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x39f70 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x39f74 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x39f78 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x39f7c | load | 1 | `r1 +0xe` | `-` |
| `ldm` at 0x39f88 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x39f88 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x39f88 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x39f88 | `r0` | `[r1+14]:1` |

## `powerLowVoltage`

- `powerLowVoltage`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x39d14 to 0x39d78.
  Established. The coprocessor store at `sp -0x1c` has unknown width and value (`stc` at 0x39d1c).
  The unnamed `0x39ddc` call carries entry `r0`, `r1`, `r2`, `r3` (`bl` at 0x39d38). The unnamed
  `0x39e70` call carries `[sp-32]`, `[sp-36]`, unknown, unknown (`bl` at 0x39d48). Low-voltage
  computation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `stc` at 0x39d1c | store | unknown | `sp -0x1c` | `?` |
| `str` at 0x39d28 | store | 4 | `sp -0x20` | `r0` |
| `str` at 0x39d2c | store | 4 | `sp -0x24` | `r1` |
| `ldr` at 0x39d30 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x39d34 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39d40 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x39d44 | load | 4 | `sp -0x24` | `-` |
| `stc` at 0x39d5c | store | unknown | `(unknown)` | `?` |
| `str` at 0x39d64 | store | 4 | `(unknown)` | `?` |
| `ldc` at 0x39d68 | load | unknown | `(unknown)` | `-` |
| `ldc` at 0x39d6c | load | unknown | `sp -0x1c` | `-` |
| `ldm` at 0x39d74 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x39d74 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x39d74 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x39d38 | `(unknown callee) 0x39ddc` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x39d48 | `(unknown callee) 0x39e70` | `[sp-32]` | `[sp-36]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x39d74 | `r0` | `ret(0x39e70)` |

## `powerHighVoltage`

- `powerHighVoltage`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x39d78 to 0x39ddc.
  Established. The coprocessor assignment at `sp -0x1c` stores an unknown value with unknown width
  (`stc` at 0x39d80). The unnamed `0x39ddc` call carries entry `r0`, `r1`, `r2`, `r3` (`bl` at 0x39d9c). The unnamed `0x39e70` call carries `[sp-32]`, `[sp-36]`, unknown, unknown (`bl` at 0x39dac). High-voltage calculation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `stc` at 0x39d80 | store | unknown | `sp -0x1c` | `?` |
| `str` at 0x39d8c | store | 4 | `sp -0x20` | `r0` |
| `str` at 0x39d90 | store | 4 | `sp -0x24` | `r1` |
| `ldr` at 0x39d94 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x39d98 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39da4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x39da8 | load | 4 | `sp -0x24` | `-` |
| `stc` at 0x39dc0 | store | unknown | `(unknown)` | `?` |
| `str` at 0x39dc8 | store | 4 | `(unknown)` | `?` |
| `ldc` at 0x39dcc | load | unknown | `(unknown)` | `-` |
| `ldc` at 0x39dd0 | load | unknown | `sp -0x1c` | `-` |
| `ldm` at 0x39dd8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x39dd8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x39dd8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x39d9c | `(unknown callee) 0x39ddc` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x39dac | `(unknown callee) 0x39e70` | `[sp-32]` | `[sp-36]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x39dd8 | `r0` | `ret(0x39e70)` |

## `powerNominalVoltage`

- `powerNominalVoltage`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x39ddc to 0x39e70.
  Established. The absolute word at `0x13219c` is read (`ldr` at 0x39e0c). A word at `sp -0x20`
  receives an unknown value (`str` at 0x39e58). The coprocessor load at `sp -0x20` has unknown width
  (`ldc` at 0x39e5c). Nominal voltage representation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x39dec | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x39df0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x39df4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x39e08 | load | 4 | `abs 0x39e68` | `-` |
| `ldr` at 0x39e0c | load | 4 | `abs 0x13219c` | `-` |
| `str` at 0x39e14 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x39e18 | load | 4 | `abs 0x39e6c` | `-` |
| `ldr` at 0x39e1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x39e20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39e34 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x39e3c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x39e54 | load | 4 | `(unknown)` | `-` |
| `str` at 0x39e58 | store | 4 | `sp -0x20` | `?` |
| `ldc` at 0x39e5c | load | unknown | `sp -0x20` | `-` |
| `ldm` at 0x39e64 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x39e64 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x39e64 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x39e64 | `r0` | `0xf5498` |

## `powerMarginPercent`

- `powerMarginPercent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x39e70 to 0x39f08.
  Established. Included word assignments at `sp -0x20` write `0x3cdd67ba` and `0x3d4ccccd` in
  separate paths (`str` at 0x39ee0, `str` at 0x39eec). The coprocessor load at `sp -0x20` has
  unknown width (`ldc` at 0x39ef0). The return reports entry `r0` in the integer result register
  (`ldm` at 0x39ef8). Percentage representation and coprocessor result remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x39e80 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x39e84 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x39e88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x39e9c | load | 4 | `abs 0x39efc` | `-` |
| `ldr` at 0x39ea0 | load | 4 | `abs 0x13219c` | `-` |
| `str` at 0x39ea8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x39eac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x39eb0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39ec4 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x39ecc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x39edc | load | 4 | `abs 0x39f00` | `-` |
| `str` at 0x39ee0 | store | 4 | `sp -0x20` | `0x3cdd67ba` |
| `ldr` at 0x39ee8 | load | 4 | `abs 0x39f04` | `-` |
| `str` at 0x39eec | store | 4 | `sp -0x20` | `0x3d4ccccd` |
| `ldc` at 0x39ef0 | load | unknown | `sp -0x20` | `-` |
| `ldm` at 0x39ef8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x39ef8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x39ef8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x39ed4 | `? == 0xe` | `? != 0xe` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x39ef8 | `r0` | `r0` |

## `powerMarginName`

- `powerMarginName`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x39c00 to 0x39d14.
  Established. A nonzero `ret(0x3a044)` takes `0x39c44`, while zero falls through (`bne` at 0x39c34). The byte through `ret(0x3a044) +0x6` is read (`ldrb` at 0x39c48). The unnamed `0x3a950`
  call carries `[sp-20]`, `ret(0x3a044)`, unknown, `[ret(0x3a044)+6]:1` (`bl` at 0x39c5c). Margin
  label interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x39c10 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x39c14 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x39c18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x39c1c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x39c28 | store | 4 | `sp -0x1c` | `ret(0x3a044)` |
| `ldr` at 0x39c2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x39c38 | load | 4 | `abs 0x39cf0` | `-` |
| `str` at 0x39c3c | store | 4 | `sp -0x20` | `0xf5cb8` |
| `ldr` at 0x39c44 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x39c48 | load | 1 | `ret(0x3a044) +0x6` | `-` |
| `ldr` at 0x39c54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x39c58 | load | 4 | `sp -0x1c` | `-` |
| `ldrls` at 0x39c68 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x39cd0 | load | 4 | `abs 0x39d0c` | `-` |
| `str` at 0x39cd4 | store | 4 | `sp -0x20` | `0xf5ce8` |
| `ldr` at 0x39cdc | load | 4 | `abs 0x39d10` | `-` |
| `str` at 0x39ce0 | store | 4 | `sp -0x20` | `0xf5cf0` |
| `ldr` at 0x39ce4 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x39cec | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x39cec | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x39cec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x39c20 | `(unknown callee) 0x3a044` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x39c5c | `(unknown callee) 0x3a950` | `[sp-20]` | `ret(0x3a044)` | `?` | `[ret(0x3a044)+6]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x39c34 | `ret(0x3a044) != 0x0` | `ret(0x3a044) == 0x0` |
| `beq` at 0x39c50 | `[ret(0x3a044)+6]:1 == 0xff` | `[ret(0x3a044)+6]:1 != 0xff` |
| `ldrls` at 0x39c68 | `ret(0x3a950) <= 0x5 (unsigned, 32-bit)` | `ret(0x3a950) > 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x39cec | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x39c68).

## `powerResetStart`

- `powerResetStart`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3a274 to 0x3a28c.
  Established. The word at `sp -0x14` receives entry `r0` (`str` at 0x3a284). The return reports
  entry `r0` (`ldm` at 0x3a288). Reset-start effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3a284 | store | 4 | `sp -0x14` | `r0` |
| `ldm` at 0x3a288 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x3a288 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3a288 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3a288 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3a288 | `r0` | `r0` |

## `powerResetDone`

- `powerResetDone`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3a28c to 0x3a2d0.
  Established. The absolute word at `0x13219c` is read (`ldr` at 0x3a2b8). An included byte
  assignment at an unknown target writes zero (`strb` at 0x3a2c4). The return reports entry `r0`
  (`ldm` at 0x3a2c8). Reset-completion state meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3a29c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x3a2a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3a2b4 | load | 4 | `abs 0x3a2cc` | `-` |
| `ldr` at 0x3a2b8 | load | 4 | `abs 0x13219c` | `-` |
| `strb` at 0x3a2c4 | store | 1 | `(unknown)` | `0x0` |
| `ldm` at 0x3a2c8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x3a2c8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3a2c8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3a2c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3a2c8 | `r0` | `r0` |

## `powerUpInProgress`

- `powerUpInProgress`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3a2d0 to 0x3a314.
  Established. The absolute word at `0x13219c` is read (`ldr` at 0x3a2fc). A byte at an unknown
  target is read (`ldrb` at 0x3a304). The return reports an unknown value (`ldm` at 0x3a30c).
  Power-up progress classification remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3a2e0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x3a2e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3a2f8 | load | 4 | `abs 0x3a310` | `-` |
| `ldr` at 0x3a2fc | load | 4 | `abs 0x13219c` | `-` |
| `ldrb` at 0x3a304 | load | 1 | `(unknown)` | `-` |
| `ldm` at 0x3a30c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x3a30c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3a30c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3a30c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3a30c | `r0` | `?` |

## `powerUpFirstReset`

- `powerUpFirstReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3a314 to 0x3a358.
  Established. The word at `sp -0x14` receives entry `r0` (`str` at 0x3a324). A byte at an unknown
  target is read (`ldrb` at 0x3a348). The return reports an unknown value (`ldm` at 0x3a350).
  First-reset flag interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3a324 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x3a328 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3a33c | load | 4 | `abs 0x3a354` | `-` |
| `ldr` at 0x3a340 | load | 4 | `abs 0x13219c` | `-` |
| `ldrb` at 0x3a348 | load | 1 | `(unknown)` | `-` |
| `ldm` at 0x3a350 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x3a350 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3a350 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3a350 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3a350 | `r0` | `?` |

## `EnvPowerUpStart`

- `EnvPowerUpStart`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x76f1c to 0x76f34.
  Established. The unnamed `0x75654` call carries `0x2`, entry `r1`, entry `r2`, entry `r3` (`bl` at 0x76f2c). The return reports `ret(0x75654)` (`ldm` at 0x76f30). Power-up start notification
  meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldm` at 0x76f30 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x76f30 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x76f30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x76f2c | `(unknown callee) 0x75654` | `0x2` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x76f30 | `r0` | `ret(0x75654)` |

## `EnvPowerUpDone`

- `EnvPowerUpDone`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x76f34 to 0x76f60.
  Established. The absolute word at `0x132f40` receives `ret(0x194f0)` (`str` at 0x76f4c). The
  unnamed `0x75654` call carries `0x4` and three unknown candidates (`bl` at 0x76f54). The return
  reports `ret(0x75654)` (`ldm` at 0x76f58). Power-up completion handling remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x76f40 | load | 4 | `abs 0x76f5c` | `-` |
| `str` at 0x76f4c | store | 4 | `abs 0x132f40` | `ret(0x194f0)` |
| `ldm` at 0x76f58 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x76f58 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x76f58 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x76f58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x76f48 | `(unknown callee) 0x194f0` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x76f54 | `(unknown callee) 0x75654` | `0x4` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x76f58 | `r0` | `ret(0x75654)` |

## `EnvPowerDownStart`

- `EnvPowerDownStart`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x76f80 to 0x76f98.
  Established. The unnamed `0x75654` call carries `0x3`, entry `r1`, entry `r2`, entry `r3` (`bl` at 0x76f90). The return reports `ret(0x75654)` (`ldm` at 0x76f94). Power-down start effects remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldm` at 0x76f94 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x76f94 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x76f94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x76f90 | `(unknown callee) 0x75654` | `0x3` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x76f94 | `r0` | `ret(0x75654)` |
