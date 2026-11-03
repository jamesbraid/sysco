# ARM temperature access and environment callbacks in 1.38.4

Read from `6.5.28/usr/cpu/firmware/sysco/l1.bin`, package `6.5.28:l1#4` (version 1.38.4),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53cfa0`.
Carrier SHA-256: `e5b65e5f28d7fba6013a8618f55c34bed7f60f7c6f46d7e9c5b8b7b2eabbe7b3`.
Package SHA-256: `c6693d19c6eb8c3fcf00cbb84d6974d1b5b17b930fb5375c1df2df20894edec9`.
Inner executable SHA-256: `f652c04c77cd7c53ca7ae6d6d6f581f11200efa211ff089bfba0d8bd4f718eb8`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1254136 bytes.
Segment SHA-256: `a373bb774d119f083e52d394fbd2701ebfbf35210301a40dcdb4ebdc39bf6bdc`. Analysed routine spans use the exact
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
No unresolved computed transfers were reported for these routines. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/temperature-callbacks-facts.txt) retain all tool sections.

## `tempInit`

- `tempInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52c74 to 0x52d9c.
  Established. The absolute-word test takes its bypass for `[0x148344] != 0x0` and falls through on
  equality to store word `0x1` there (`bne` at 0x52cec, `str` at 0x52cf8). The call to `0x1b3c4` has
  `[sp-40]`, `0x1`, `0x148344`, unknown (`bl` at 0x52d18); its result is stored as a word at
  `0x148340` (`str` at 0x52d20). A later byte store writes `([sp-32]:1 & 0xff)` at `[sp-36] +0x4`
  (`strb` at 0x52d80). Sensor initialization, record layout and the joined r0 value remain unknown
  (`ldmdb` at 0x52d84).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x52c84 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x52c88 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x52c8c | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x52c90 | load | 4 | `abs 0x52d88` | `-` |
| `ldr` at 0x52c94 | load | 4 | `abs 0x148340` | `-` |
| `ldr` at 0x52ca0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x52cac | load | 4 | `abs 0x52d8c` | `-` |
| `str` at 0x52cb0 | store | 4 | `sp -0x34` | `0x10b200` |
| `ldr` at 0x52cb4 | load | 4 | `abs 0x52d88` | `-` |
| `ldr` at 0x52cb8 | load | 4 | `abs 0x148340` | `-` |
| `str` at 0x52cbc | store | 4 | `sp -0x30` | `[0x148340]` |
| `ldr` at 0x52cc0 | load | 4 | `abs 0x52d90` | `-` |
| `ldr` at 0x52cc4 | load | 4 | `abs 0x148344` | `-` |
| `str` at 0x52cc8 | store | 4 | `sp -0x2c` | `[0x148344]` |
| `ldr` at 0x52cd0 | load | 4 | `abs 0x52d94` | `-` |
| `ldr` at 0x52ce0 | load | 4 | `abs 0x52d90` | `-` |
| `ldr` at 0x52ce4 | load | 4 | `abs 0x148344` | `-` |
| `ldr` at 0x52cf0 | load | 4 | `abs 0x52d90` | `-` |
| `str` at 0x52cf8 | store | 4 | `abs 0x148344` | `0x1` |
| `ldr` at 0x52cfc | load | 4 | `abs 0x52d90` | `-` |
| `ldr` at 0x52d00 | load | 4 | `abs 0x148344` | `-` |
| `str` at 0x52d08 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x52d0c | load | 4 | `abs 0x52d88` | `-` |
| `ldr` at 0x52d10 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x52d20 | store | 4 | `abs 0x148340` | `ret(0x1b3c4)` |
| `ldr` at 0x52d24 | load | 4 | `abs 0x148340` | `-` |
| `ldr` at 0x52d30 | load | 4 | `abs 0x52d98` | `-` |
| `str` at 0x52d34 | store | 4 | `sp -0x34` | `0x10b244` |
| `ldr` at 0x52d38 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x52d3c | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x52d44 | load | 4 | `abs 0x52d94` | `-` |
| `ldr` at 0x52d54 | load | 4 | `abs 0x52d88` | `-` |
| `ldr` at 0x52d58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x52d60 | load | 4 | `abs 0x148340` | `-` |
| `str` at 0x52d68 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x52d6c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x52d70 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x52d74 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0x52d78 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x52d7c | load | 1 | `sp -0x20` | `-` |
| `strb` at 0x52d80 | store | 1 | `[sp-36] +0x4` | `([sp-32]:1 & 0xff)` |
| `ldmdb` at 0x52d84 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x52d84 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x52d84 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x52d84 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x52cdc | `(unknown callee) 0x21e28` | `0x9` | `0x10b230` | `0x3a` | `0x0` |
| `bl` at 0x52d18 | `(unknown callee) 0x1b3c4` | `[sp-40]` | `0x1` | `0x148344` | `?` |
| `bl` at 0x52d50 | `(unknown callee) 0x21e28` | `0x9` | `0x10b230` | `0x46` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x52c9c | `[0x148340] == 0x0` | `[0x148340] != 0x0` |
| `bne` at 0x52ca8 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0x52cec | `[0x148344] != 0x0` | `[0x148344] == 0x0` |
| `bne` at 0x52d2c | `[0x148340] != 0x0` | `[0x148340] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x52d84 | `r0` | `?` |

## `tempGetCfg`

- `tempGetCfg`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52d9c to 0x52df0.
  Established. The explicit output word receives `[[sp-28]]` at entry `r1 +0x0` (`str` at 0x52dd8).
  A later byte load addresses `[sp-28] +0x4` and supplies the r0 return `[[sp-28]+4]:1` (`ldrb` at
  0x52de0, `ldmdb` at 0x52de8). No calls or conditional branches are reported. Configuration-field
  meanings remain unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x52dac | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x52db0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x52db4 | load | 4 | `abs 0x52dec` | `-` |
| `ldr` at 0x52db8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x52dc0 | load | 4 | `abs 0x148340` | `-` |
| `str` at 0x52dc8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x52dcc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x52dd0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52dd4 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x52dd8 | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0x52ddc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x52de0 | load | 1 | `[sp-28] +0x4` | `-` |
| `ldmdb` at 0x52de8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x52de8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x52de8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x52de8 | `r0` | `[[sp-28]+4]:1` |

## `tempCount`

- `tempCount`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52df0 to 0x52e30.
  Established. A byte load addresses `[sp-24] +0x4`, following a word read at `0x148340` (`ldrb` at
  0x52e20, `ldr` at 0x52e10). The returned r0 value is `[[sp-24]+4]:1` (`ldmdb` at 0x52e28). Entry
  r0 is first saved as a word at `sp -0x14` (`str` at 0x52e00). Count units and the referenced array
  layout remain unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x52e00 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x52e04 | load | 4 | `abs 0x52e2c` | `-` |
| `ldr` at 0x52e08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x52e10 | load | 4 | `abs 0x148340` | `-` |
| `str` at 0x52e18 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x52e1c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x52e20 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldmdb` at 0x52e28 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x52e28 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x52e28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x52e28 | `r0` | `[[sp-24]+4]:1` |

## `tempIsPresent`

- `tempIsPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52e30 to 0x52e7c.
  Established. The call to `0x53190` has entry r0, `(r1 & 0xff)`, entry r2, `(r1 & 0xff)` (`bl` at
  0x52e58). A byte read at `ret(0x53190) +0x0` precedes the call to `0xcc494` with `[sp-20]`,
  `[ret(0x53190)]:1`, unknown, `[ret(0x53190)]:1` (`ldrb` at 0x52e60, `bl` at 0x52e6c). The final r0
  value is `ret(0xcc494)` (`ldmdb` at 0x52e78). Physical sensor-presence interpretation is not
  established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x52e40 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x52e48 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x52e4c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x52e50 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x52e60 | load | 1 | `ret(0x53190) +0x0` | `-` |
| `ldr` at 0x52e64 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x52e78 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x52e78 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x52e78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x52e58 | `(unknown callee) 0x53190` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x52e6c | `(unknown callee) 0xcc494` | `[sp-20]` | `[ret(0x53190)]:1` | `?` | `[ret(0x53190)]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x52e78 | `r0` | `ret(0xcc494)` |

## `tempSetLimits`

- `tempSetLimits`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52e7c to 0x52ef8.
  Established. The second call targets `0xccb28` with `[sp-20]`, `[ret(0x53190)]:1`,
  `[ret(0x53190)+1]:1`, `[sp-22]:1` (`bl` at 0x52ee8). A separate prior word store writes
  `[sp-23]:1` at `sp -0x20` (`str` at 0x52edc). The callee result reaches r0 at the return (`ldmdb`
  at 0x52ef4). Limit units, argument count and external threshold changes remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x52e8c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x52e98 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x52ea0 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x52ea8 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldrb` at 0x52eac | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x52eb0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x52ec0 | store | 4 | `sp -0x1c` | `ret(0x53190)` |
| `ldr` at 0x52ec4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x52ec8 | load | 1 | `ret(0x53190) +0x0` | `-` |
| `ldr` at 0x52ecc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x52ed0 | load | 1 | `ret(0x53190) +0x1` | `-` |
| `ldrb` at 0x52ed4 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x52ed8 | load | 1 | `sp -0x17` | `-` |
| `str` at 0x52edc | store | 4 | `sp -0x20` | `[sp-23]:1` |
| `ldr` at 0x52ee0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x52ef4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x52ef4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x52ef4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x52eb8 | `(unknown callee) 0x53190` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x52ee8 | `(unknown callee) 0xccb28` | `[sp-20]` | `[ret(0x53190)]:1` | `[ret(0x53190)+1]:1` | `[sp-22]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x52ef4 | `r0` | `ret(0xccb28)` |

## `tempIntDisable`

- `tempIntDisable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52ef8 to 0x52f58.
  Established. Byte reads address `ret(0x53190) +0x0` and `ret(0x53190) +0x1` (`ldrb` at 0x52f30,
  `ldrb` at 0x52f38). The call to `0xcdbc0` has `[sp-20]`, `[ret(0x53190)]:1`, `[ret(0x53190)+1]:1`,
  `[ret(0x53190)+1]:1` (`bl` at 0x52f48); its result supplies r0 at the return (`ldmdb` at 0x52f54).
  Physical interrupt-disable effects cannot be assigned from this wrapper.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x52f08 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x52f10 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x52f14 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x52f18 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x52f28 | store | 4 | `sp -0x1c` | `ret(0x53190)` |
| `ldr` at 0x52f2c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x52f30 | load | 1 | `ret(0x53190) +0x0` | `-` |
| `ldr` at 0x52f34 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x52f38 | load | 1 | `ret(0x53190) +0x1` | `-` |
| `ldr` at 0x52f3c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x52f54 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x52f54 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x52f54 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x52f20 | `(unknown callee) 0x53190` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x52f48 | `(unknown callee) 0xcdbc0` | `[sp-20]` | `[ret(0x53190)]:1` | `[ret(0x53190)+1]:1` | `[ret(0x53190)+1]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x52f54 | `r0` | `ret(0xcdbc0)` |

## `tempIntEnable`

- `tempIntEnable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52f58 to 0x52fb8.
  Established. The call to `0xcdc08` carries `[sp-20]`, `[ret(0x53190)]:1`, `[ret(0x53190)+1]:1`,
  `[ret(0x53190)+1]:1` (`bl` at 0x52fa8). An earlier word store saves `ret(0x53190)` at `sp -0x1c`
  (`str` at 0x52f88). The r0 return is `ret(0xcdc08)` (`ldmdb` at 0x52fb4). No conditional branches
  appear, and physical interrupt-enable semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x52f68 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x52f70 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x52f74 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x52f78 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x52f88 | store | 4 | `sp -0x1c` | `ret(0x53190)` |
| `ldr` at 0x52f8c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x52f90 | load | 1 | `ret(0x53190) +0x0` | `-` |
| `ldr` at 0x52f94 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x52f98 | load | 1 | `ret(0x53190) +0x1` | `-` |
| `ldr` at 0x52f9c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x52fb4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x52fb4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x52fb4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x52f80 | `(unknown callee) 0x53190` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x52fa8 | `(unknown callee) 0xcdc08` | `[sp-20]` | `[ret(0x53190)]:1` | `[ret(0x53190)+1]:1` | `[ret(0x53190)+1]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x52fb4 | `r0` | `ret(0xcdc08)` |

## `tempRead`

- `tempRead`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52fb8 to 0x53060.
  Established. The call to `0xcd130` uses `[sp-20]`, `[ret(0x53190)]:1`, `[ret(0x53190)+1]:1`,
  `sp-33` (`bl` at 0x53014); its zero result takes the read-result arm and nonzero falls through
  (`beq` at 0x53028). The nonzero arm stores word `0x0` at `[sp-28] +0x0`, and the zero arm stores
  an unknown word at that target (`str` at 0x53034, `str` at 0x5304c). The r0 return is `[sp-44]`,
  while temperature units and device-read semantics remain unidentified (`ldmdb` at 0x5305c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x52fc8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x52fd0 | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0x52fd4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x52fd8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x52fdc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x52fec | store | 4 | `sp -0x20` | `ret(0x53190)` |
| `ldr` at 0x52ff0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x52ff4 | load | 1 | `ret(0x53190) +0x0` | `-` |
| `ldr` at 0x52ff8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x52ffc | load | 1 | `ret(0x53190) +0x1` | `-` |
| `ldr` at 0x53004 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5301c | store | 4 | `sp -0x28` | `ret(0xcd130)` |
| `ldr` at 0x53020 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5302c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x53034 | store | 4 | `[sp-28] +0x0` | `0x0` |
| `ldr` at 0x53038 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x5303c | store | 4 | `sp -0x2c` | `[sp-40]` |
| `ldr` at 0x53044 | load | 4 | `sp -0x1c` | `-` |
| `ldrsb` at 0x53048 | load | 1 | `sp -0x21` | `-` |
| `str` at 0x5304c | store | 4 | `[sp-28] +0x0` | `?` |
| `str` at 0x53054 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x53058 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x5305c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5305c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5305c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x52fe4 | `(unknown callee) 0x53190` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x53014 | `(unknown callee) 0xcd130` | `[sp-20]` | `[ret(0x53190)]:1` | `[ret(0x53190)+1]:1` | `sp-33` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x53028 | `ret(0xcd130) == 0x0` | `ret(0xcd130) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5305c | `r0` | `[sp-44]` |

## `tempLocation`

- `tempLocation`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x53060 to 0x530d4.
  Established. The word test takes the bypass for `[ret(0x53190)+4] != 0x0` and falls through on
  equality (`bne` at 0x530ac). The fall-through call to `0xcdf64` has `[sp-20]`, `[ret(0x53190)]:1`,
  unknown, `[ret(0x53190)]:1` (`bl` at 0x530c0); its result is saved as a word at `sp -0x20` (`str`
  at 0x530c4). Location encoding and the merged r0 value are unknown (`ldmdb` at 0x530d0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x53070 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x53078 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x5307c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x53080 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x53090 | store | 4 | `sp -0x1c` | `ret(0x53190)` |
| `ldr` at 0x53094 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x53098 | load | 4 | `ret(0x53190) +0x4` | `-` |
| `str` at 0x5309c | store | 4 | `sp -0x20` | `[ret(0x53190)+4]` |
| `ldr` at 0x530a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x530a4 | load | 4 | `ret(0x53190) +0x4` | `-` |
| `ldr` at 0x530b0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x530b4 | load | 1 | `ret(0x53190) +0x0` | `-` |
| `ldr` at 0x530b8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x530c4 | store | 4 | `sp -0x20` | `ret(0xcdf64)` |
| `ldr` at 0x530c8 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x530d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x530d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x530d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x53088 | `(unknown callee) 0x53190` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x530c0 | `(unknown callee) 0xcdf64` | `[sp-20]` | `[ret(0x53190)]:1` | `?` | `[ret(0x53190)]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x530ac | `[ret(0x53190)+4] != 0x0` | `[ret(0x53190)+4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x530d0 | `r0` | `?` |

## `tempType`

- `tempType`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x530d4 to 0x53190.
  Established. The byte test takes the later-call arm for `[ret(0x53190)+2]:1 == 0x0` and falls
  through on nonzero (`beq` at 0x53114). That arm calls `0xcc7c8` with `[sp-20]`, `sp-32`, unknown,
  `sp-32` (`bl` at 0x53134). A signed bound then takes the zero-result arm for `[[sp-28]]:1 >=
  ret(0xcc7c8)` and falls through for less (`bge` at 0x53150). The later indexed byte read has
  unknown target (`ldrb` at 0x53174). Type encodings and the joined r0 return remain unproven
  (`ldmdb` at 0x5318c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x530e4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x530ec | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x530f0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x530f4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x53104 | store | 4 | `sp -0x1c` | `ret(0x53190)` |
| `ldr` at 0x53108 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5310c | load | 1 | `ret(0x53190) +0x2` | `-` |
| `ldr` at 0x53118 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5311c | load | 1 | `ret(0x53190) +0x2` | `-` |
| `str` at 0x53120 | store | 4 | `sp -0x28` | `[ret(0x53190)+2]:1` |
| `ldr` at 0x5312c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5313c | store | 4 | `sp -0x24` | `ret(0xcc7c8)` |
| `ldr` at 0x53140 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x53144 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x53148 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x53154 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x53158 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x5316c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x53174 | load | 1 | `(unknown)` | `-` |
| `str` at 0x53178 | store | 4 | `sp -0x28` | `?` |
| `str` at 0x53184 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x53188 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x5318c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5318c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5318c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x530fc | `(unknown callee) 0x53190` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x53134 | `(unknown callee) 0xcc7c8` | `[sp-20]` | `sp-32` | `?` | `sp-32` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x53114 | `[ret(0x53190)+2]:1 == 0x0` | `[ret(0x53190)+2]:1 != 0x0` |
| `bge` at 0x53150 | `[[sp-28]]:1 >= ret(0xcc7c8) (signed, 32-bit)` | `[[sp-28]]:1 < ret(0xcc7c8) (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5318c | `r0` | `?` |

## `EnvTempInt`

- `EnvTempInt`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7dc00 to 0x7de7c.
  Established. Two byte stores write `0x1` and `0x0` at `[sp-28] +0x1`, with the second conditional
  on the call result (`strb` at 0x7de1c, `strb` at 0x7de3c). The intervening call to `0x22dc0` has
  `0x148ed4`, unknown, `[sp-28]`, `0x1` (`bl` at 0x7de24); its zero result takes the final path and
  nonzero falls through to the zero-byte store (`beq` at 0x7de30, `strb` at 0x7de3c). The earlier
  indexed byte reads have unknown targets (`ldrb` at 0x7dc58, `ldrb` at 0x7dc7c). Sensor identity,
  interrupt delivery and the joined r0 value remain unknown (`ldmdb` at 0x7de4c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7dc10 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x7dc18 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x7dc20 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x7dc28 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x7dc2c | load | 4 | `abs 0x7de50` | `-` |
| `ldr` at 0x7dc30 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7dc34 | load | 4 | `abs 0x1490fc` | `-` |
| `ldr` at 0x7dc44 | load | 4 | `abs 0x7de50` | `-` |
| `ldr` at 0x7dc48 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7dc50 | load | 4 | `abs 0x149108` | `-` |
| `ldrb` at 0x7dc58 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x7dc5c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7dc68 | load | 4 | `abs 0x7de50` | `-` |
| `ldr` at 0x7dc6c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7dc74 | load | 4 | `abs 0x149108` | `-` |
| `ldrb` at 0x7dc7c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x7dc80 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x7dc90 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7dc98 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x7dca0 | load | 4 | `abs 0x7de50` | `-` |
| `ldr` at 0x7dca4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7dca8 | load | 4 | `abs 0x1490fc` | `-` |
| `ldr` at 0x7dcb4 | load | 4 | `abs 0x7de54` | `-` |
| `str` at 0x7dcb8 | store | 4 | `sp -0x34` | `0x110b3c` |
| `ldrb` at 0x7dcbc | load | 1 | `sp -0x15` | `-` |
| `str` at 0x7dcc0 | store | 4 | `sp -0x30` | `(r1 & 0xff)` |
| `ldrb` at 0x7dcc4 | load | 1 | `sp -0x16` | `-` |
| `str` at 0x7dcc8 | store | 4 | `sp -0x2c` | `(r2 & 0xff)` |
| `ldr` at 0x7dcd0 | load | 4 | `abs 0x7de58` | `-` |
| `str` at 0x7dce4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x7dcec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7dcf4 | load | 4 | `abs 0x7de5c` | `-` |
| `str` at 0x7dcfc | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x7dd00 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7dd04 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7dd10 | load | 4 | `abs 0x7de50` | `-` |
| `ldr` at 0x7dd14 | load | 4 | `abs 0x1490e8` | `-` |
| `ldr` at 0x7dd20 | load | 4 | `abs 0x7de50` | `-` |
| `ldr` at 0x7dd24 | load | 4 | `abs 0x1490ec` | `-` |
| `ldr` at 0x7dd30 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7dd34 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7dd38 | load | 4 | `abs 0x7de50` | `-` |
| `ldr` at 0x7dd3c | load | 4 | `abs 0x7de50` | `-` |
| `ldr` at 0x7dd40 | load | 4 | `abs 0x7de60` | `-` |
| `ldr` at 0x7dd48 | load | 4 | `abs 0x1490e8` | `-` |
| `ldr` at 0x7dd4c | load | 4 | `abs 0x1490ec` | `-` |
| `ldrb` at 0x7dd54 | load | 1 | `sp -0x20` | `-` |
| `ldr` at 0x7dd68 | load | 4 | `abs 0x7de64` | `-` |
| `str` at 0x7dd6c | store | 4 | `sp -0x34` | `0x110b98` |
| `str` at 0x7dd70 | store | 4 | `sp -0x30` | `ret(0x53060)` |
| `ldrb` at 0x7dd74 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x7dd78 | store | 4 | `sp -0x2c` | `[sp-21]:1` |
| `ldrb` at 0x7dd7c | load | 1 | `sp -0x16` | `-` |
| `str` at 0x7dd80 | store | 4 | `sp -0x28` | `[sp-22]:1` |
| `ldr` at 0x7dd88 | load | 4 | `abs 0x7de58` | `-` |
| `ldr` at 0x7dd8c | load | 4 | `abs 0x7de68` | `-` |
| `str` at 0x7dd9c | store | 4 | `sp -0x24` | `0x0` |
| `ldrb` at 0x7dda4 | load | 1 | `sp -0x20` | `-` |
| `ldr` at 0x7ddb4 | load | 4 | `abs 0x7de6c` | `-` |
| `ldr` at 0x7ddb8 | load | 4 | `abs 0x148df0` | `-` |
| `ldr` at 0x7ddc8 | load | 4 | `abs 0x7de70` | `-` |
| `ldr` at 0x7ddcc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7ddd4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7ddd8 | load | 1 | `[sp-28] +0x1` | `-` |
| `ldr` at 0x7dde4 | load | 4 | `abs 0x7de6c` | `-` |
| `ldr` at 0x7dde8 | load | 4 | `abs 0x148df0` | `-` |
| `ldr` at 0x7ddf8 | load | 4 | `abs 0x7de74` | `-` |
| `ldr` at 0x7ddfc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7de04 | load | 1 | `sp -0x20` | `-` |
| `ldr` at 0x7de14 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x7de1c | store | 1 | `[sp-28] +0x1` | `0x1` |
| `ldr` at 0x7de20 | load | 4 | `abs 0x7de78` | `-` |
| `ldr` at 0x7de34 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x7de3c | store | 1 | `[sp-28] +0x1` | `0x0` |
| `str` at 0x7de44 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x7de48 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x7de4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x7de4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x7de4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7dcdc | `(unknown callee) 0x21e28` | `0x1` | `0x1106f4` | `0x5a0` | `0x0` |
| `bl` at 0x7dd50 | `(unknown callee) 0x2070c` | `0x110b78` | `?` | `[0x1490e8]` | `[0x1490ec]` |
| `bl` at 0x7dd60 | `(unknown callee) 0x53060` | `0x0` | `[sp-32]:1` | `?` | `[sp-32]:1` |
| `bl` at 0x7dd94 | `(unknown callee) 0x21e28` | `0x1` | `0x1106f4` | `0x5af` | `0x0` |
| `bl` at 0x7ddb0 | `(unknown callee) 0x52ef8` | `0x0` | `[sp-32]:1` | `?` | `[sp-32]:1` |
| `bl` at 0x7ddd0 | `(unknown callee) 0x23f2c` | `0x110bd0` | `[sp-32]` | `?` | `?` |
| `bl` at 0x7de00 | `(unknown callee) 0x23f2c` | `0x110be4` | `[sp-32]` | `?` | `?` |
| `bl` at 0x7de10 | `(unknown callee) 0x52ef8` | `0x0` | `[sp-32]:1` | `?` | `[sp-32]:1` |
| `bl` at 0x7de24 | `(unknown callee) 0x22dc0` | `0x148ed4` | `?` | `[sp-28]` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0x7dc3c | `C == 0` | `not (C == 0)` |
| `bne` at 0x7dc64 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x7dc88 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0x7dcb0 | `C == 0` | `not (C == 0)` |
| `bne` at 0x7dd0c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x7dd1c | `[0x1490e8] != 0x0` | `[0x1490e8] == 0x0` |
| `bne` at 0x7dd2c | `[0x1490ec] != 0x0` | `[0x1490ec] == 0x0` |
| `beq` at 0x7ddc4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x7dde0 | `[[sp-28]+1]:1 != 0x0` | `[[sp-28]+1]:1 == 0x0` |
| `beq` at 0x7ddf4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7de30 | `ret(0x22dc0) == 0x0` | `ret(0x22dc0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x7de4c | `r0` | `?` |

## `EnvTemp`

- `EnvTemp`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8204c to 0x82084.
  Established. The first call reaches `0x1d38c` with `0x7`, unknown, entry r2, unknown (`bl` at
  0x82078). The next call to `0x82140` has `ret(0x1d38c)` and three unknown candidates (`bl` at
  0x8207c); its result supplies r0 at the return (`ldmdb` at 0x82080). The predicated value
  selection is not resolved into a boolean by the facts, and external temperature policy remains
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8205c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x82060 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x82080 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x82080 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x82080 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x82078 | `(unknown callee) 0x1d38c` | `0x7` | `?` | `r2` | `?` |
| `bl` at 0x8207c | `(unknown callee) 0x82140` | `ret(0x1d38c)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x82080 | `r0` | `ret(0x82140)` |

## `EnvTestTemp`

- `EnvTestTemp`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82328 to 0x823b0.
  Established. The absolute word at `0x148e04` receives `0x1` (`str` at 0x82350). Further word
  stores write `[sp-20]` at `0x148e08`, `[sp-24]` at `0x148e0c`, `[sp-28]` at `0x148e10`, `[sp-32]`
  at `0x148e14`, and `[sp]` at `0x148e18` (`str` at 0x8235c, `str` at 0x82368, `str` at 0x82374,
  `str` at 0x82380, `str` at 0x8238c). The call to `0x82140` uses entry r0, entry r1, `0x148e18`,
  `[sp]` (`bl` at 0x82390). The r0 return is `ret(0x82140)`, and test-mode field meanings and units
  remain unidentified (`ldmdb` at 0x82394).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x82338 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x8233c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x82340 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x82344 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x82348 | load | 4 | `abs 0x82398` | `-` |
| `str` at 0x82350 | store | 4 | `abs 0x148e04` | `0x1` |
| `ldr` at 0x82354 | load | 4 | `abs 0x8239c` | `-` |
| `ldr` at 0x82358 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x8235c | store | 4 | `abs 0x148e08` | `[sp-20]` |
| `ldr` at 0x82360 | load | 4 | `abs 0x823a0` | `-` |
| `ldr` at 0x82364 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x82368 | store | 4 | `abs 0x148e0c` | `[sp-24]` |
| `ldr` at 0x8236c | load | 4 | `abs 0x823a4` | `-` |
| `ldr` at 0x82370 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x82374 | store | 4 | `abs 0x148e10` | `[sp-28]` |
| `ldr` at 0x82378 | load | 4 | `abs 0x823a8` | `-` |
| `ldr` at 0x8237c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x82380 | store | 4 | `abs 0x148e14` | `[sp-32]` |
| `ldr` at 0x82384 | load | 4 | `abs 0x823ac` | `-` |
| `ldr` at 0x82388 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x8238c | store | 4 | `abs 0x148e18` | `[sp]` |
| `ldmdb` at 0x82394 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x82394 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x82394 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x82390 | `(unknown callee) 0x82140` | `r0` | `r1` | `0x148e18` | `[sp]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x82394 | `r0` | `ret(0x82140)` |

## `EnvTempConfigure`

- `EnvTempConfigure`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82484 to 0x824b0.
  Established. The first call targets `0x7d670` with entry r0, entry r1, entry r2, entry r3 (`bl` at
  0x82498); its result becomes the second candidate of the call to `0x7d398`, whose full candidates
  are `[sp-20]`, `ret(0x7d670)`, unknown, `ret(0x7d670)` (`bl` at 0x824a8). The return has r0
  `ret(0x7d398)` (`ldmdb` at 0x824ac). Threshold configuration and physical sensor effects remain
  unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x82494 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x824a0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x824ac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x824ac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x824ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x82498 | `(unknown callee) 0x7d670` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x824a8 | `(unknown callee) 0x7d398` | `[sp-20]` | `ret(0x7d670)` | `?` | `ret(0x7d670)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x824ac | `r0` | `ret(0x7d398)` |

## `EnvAutoFanControl`

- `EnvAutoFanControl`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x824e4 to 0x82504.
  Established. The explicit word store writes `0x1` at `0x148e00` (`str` at 0x824f8). The return
  retains entry r0 (`ldmdb` at 0x824fc). No calls or conditional branches are reported. A
  fan-control mode or physical speed change is not established by this store alone.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x824f0 | load | 4 | `abs 0x82500` | `-` |
| `str` at 0x824f8 | store | 4 | `abs 0x148e00` | `0x1` |
| `ldmdb` at 0x824fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x824fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x824fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x824fc | `r0` | `r0` |
