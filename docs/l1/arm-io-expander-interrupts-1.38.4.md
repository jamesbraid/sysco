# ARM I/O-expander interrupts and data paths in 1.38.4

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
Interrupt dispatch and the read/write selectors have unresolved computed transfers. Excluded destinations supply no known-edge arguments or effects. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/io-expander-interrupts-facts.txt) retain all tool sections.

## `ioExpRead`

- `ioExpRead`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd6b9c to 0xd6c2c.
  Established. The byte test takes the read arm for `[ret(0xd76d4)]:1 != 0x0` and falls through on
  equality (`bne` at 0xd6bf8). The equality arm stores word `0xfffffc11` at `sp -0x28` (`str` at
  0xd6c00). The read-arm call to `0xd7c60` carries `[sp-20]`, `[sp-32]`, `[sp-28]`,
  `[ret(0xd76d4)]:1` (`bl` at 0xd6c14). Physical read behavior and the joined r0 return remain
  unknown (`ldmdb` at 0xd6c24).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd6bac | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd6bb4 | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0xd6bb8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xd6bbc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd6bc0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd6bd0 | store | 4 | `sp -0x20` | `ret(0xd75c0)` |
| `ldrb` at 0xd6bd4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd6bd8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd6be8 | store | 4 | `sp -0x24` | `ret(0xd76d4)` |
| `ldr` at 0xd6bec | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xd6bf0 | load | 1 | `ret(0xd76d4) +0x0` | `-` |
| `ldr` at 0xd6bfc | load | 4 | `abs 0xd6c28` | `-` |
| `str` at 0xd6c00 | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0xd6c08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd6c0c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd6c10 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd6c1c | store | 4 | `sp -0x28` | `ret(0xd7c60)` |
| `ldr` at 0xd6c20 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xd6c24 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd6c24 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd6c24 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd6bc8 | `(unknown callee) 0xd75c0` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xd6be0 | `(unknown callee) 0xd76d4` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0xd6c14 | `(unknown callee) 0xd7c60` | `[sp-20]` | `[sp-32]` | `[sp-28]` | `[ret(0xd76d4)]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd6bf8 | `[ret(0xd76d4)]:1 != 0x0` | `[ret(0xd76d4)]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd6c24 | `r0` | `?` |

## `ioExpWriteValue`

- `ioExpWriteValue`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd6c2c to 0xd6c70.
  Established. The call to `0xd76d4` uses entry r0, `(r1 & 0xff)`, entry r2, `(r1 & 0xff)` (`bl` at
  0xd6c54). A byte read at `ret(0xd76d4) +0x1` supplies the returned r0 value `[ret(0xd76d4)+1]:1`
  (`ldrb` at 0xd6c64, `ldmdb` at 0xd6c6c). There are no conditional branches. The byte's
  relationship to an external output remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd6c3c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd6c44 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xd6c48 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd6c4c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd6c5c | store | 4 | `sp -0x1c` | `ret(0xd76d4)` |
| `ldr` at 0xd6c60 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd6c64 | load | 1 | `ret(0xd76d4) +0x1` | `-` |
| `ldmdb` at 0xd6c6c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd6c6c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd6c6c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd6c54 | `(unknown callee) 0xd76d4` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd6c6c | `r0` | `[ret(0xd76d4)+1]:1` |

## `ioExpIntCount`

- `ioExpIntCount`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd6c70 to 0xd6cb4.
  Established. A word is read at `ret(0xd76d4) +0x8`, following the call to `0xd76d4` (`ldr` at
  0xd6ca8, `bl` at 0xd6c98). The r0 return is `[ret(0xd76d4)+8]` (`ldmdb` at 0xd6cb0). Entry r0 is
  first saved as a word at `sp -0x14` (`str` at 0xd6c80). Counter units and the referenced record
  are unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd6c80 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd6c88 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xd6c8c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd6c90 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd6ca0 | store | 4 | `sp -0x1c` | `ret(0xd76d4)` |
| `ldr` at 0xd6ca4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd6ca8 | load | 4 | `ret(0xd76d4) +0x8` | `-` |
| `ldmdb` at 0xd6cb0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd6cb0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd6cb0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd6c98 | `(unknown callee) 0xd76d4` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd6cb0 | `r0` | `[ret(0xd76d4)+8]` |

## `ioExpIntMask`

- `ioExpIntMask`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd6cb4 to 0xd6cf8.
  Established. A byte load addresses `ret(0xd76d4) +0x3` after the only call, to `0xd76d4` (`ldrb`
  at 0xd6cec, `bl` at 0xd6cdc). That byte reaches r0 as `[ret(0xd76d4)+3]:1` (`ldmdb` at 0xd6cf4).
  The saved byte at `sp -0x15` contains `(r1 & 0xff)` (`strb` at 0xd6ccc). Physical interrupt-mask
  semantics are not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd6cc4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd6ccc | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xd6cd0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd6cd4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd6ce4 | store | 4 | `sp -0x1c` | `ret(0xd76d4)` |
| `ldr` at 0xd6ce8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd6cec | load | 1 | `ret(0xd76d4) +0x3` | `-` |
| `ldmdb` at 0xd6cf4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd6cf4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd6cf4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd6cdc | `(unknown callee) 0xd76d4` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd6cf4 | `r0` | `[ret(0xd76d4)+3]:1` |

## `ioExpIntDisable`

- `ioExpIntDisable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd6cf8 to 0xd6d40.
  Established. The sole call reaches `0xd7e6c` with entry r0, `(r1 & 0xff)`, `(r2 & 0xff)`, `0x0`
  (`bl` at 0xd6d30); its result supplies r0 at the return (`ldmdb` at 0xd6d3c). An earlier byte
  store saves `(r2 & 0xff)` at `sp -0x16` (`strb` at 0xd6d18). The body does not establish how
  external interrupts are disabled.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd6d08 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd6d10 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xd6d18 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xd6d1c | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xd6d20 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xd6d24 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd6d3c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd6d3c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd6d3c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd6d30 | `(unknown callee) 0xd7e6c` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd6d3c | `r0` | `ret(0xd7e6c)` |

## `ioExpIntEnable`

- `ioExpIntEnable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd6d40 to 0xd6d88.
  Established. The call to `0xd7e6c` carries entry r0, `(r1 & 0xff)`, `(r2 & 0xff)`, `0x1` (`bl` at
  0xd6d78). The returned r0 value is `ret(0xd7e6c)` (`ldmdb` at 0xd6d84). Byte stores retain `(r1 &
  0xff)` at `sp -0x15` and `(r2 & 0xff)` at `sp -0x16` (`strb` at 0xd6d58, `strb` at 0xd6d60).
  Hardware enable effects remain unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd6d50 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd6d58 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xd6d60 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xd6d64 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xd6d68 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xd6d6c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd6d84 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd6d84 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd6d84 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd6d78 | `(unknown callee) 0xd7e6c` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd6d84 | `r0` | `ret(0xd7e6c)` |

## `ioExpIntSet`

- `ioExpIntSet`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd6d88 to 0xd6dd4.
  Established. The call candidates for `0xd7e6c` are entry r0, `(r1 & 0xff)`, `(r2 & 0xff)`, entry
  r3 (`bl` at 0xd6dc4). Entry r3 is saved as a word at `sp -0x1c` (`str` at 0xd6d9c); its callee
  result is returned in r0 (`ldmdb` at 0xd6dd0). There are no conditional branches, and the physical
  selection remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd6d98 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd6d9c | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0xd6da4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xd6dac | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xd6db0 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xd6db4 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xd6db8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd6dc0 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xd6dd0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd6dd0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd6dd0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd6dc4 | `(unknown callee) 0xd7e6c` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd6dd0 | `r0` | `ret(0xd7e6c)` |

## `ioExpInt`

- `ioExpInt`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd6dd4 to 0xd6f84.
  Established. The computed transfer takes an unresolved edge for `[[sp-28]]:1 <= 0x4` unsigned and
  falls through for greater (`ldrls` at 0xd6e78). The known default path calls `0x21e28` with `0x1`,
  `0x13470c`, `0x2d3`, `0x0` (`bl` at 0xd6f58). A byte store advances `sp -0x21` with `([sp-33]:1+1
  & 0xff)` (`strb` at 0xd6f64). Excluded interrupt cases and the joined r0 value are unknown
  (`ldmdb` at 0xd6f74).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd6de4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd6dec | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xd6df4 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0xd6dfc | store | 4 | `sp -0x28` | `0x0` |
| `ldrb` at 0xd6e00 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0xd6e04 | store | 1 | `sp -0x21` | `((r1 & 0xff) & 0xff)` |
| `ldrb` at 0xd6e08 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0xd6e0c | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0xd6e1c | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xd6e20 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd6e30 | store | 4 | `sp -0x1c` | `ret(0xd75c0)` |
| `ldrb` at 0xd6e34 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xd6e38 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd6e48 | store | 4 | `sp -0x20` | `ret(0xd76d4)` |
| `ldr` at 0xd6e4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd6e50 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xd6e5c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd6e60 | load | 1 | `ret(0xd76d4) +0x0` | `-` |
| `ldr` at 0xd6e6c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd6e70 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0xd6e78 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd6f34 | load | 4 | `abs 0xd6f78` | `-` |
| `str` at 0xd6f38 | store | 4 | `sp -0x30` | `0x1348a8` |
| `ldr` at 0xd6f3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd6f40 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xd6f44 | store | 4 | `sp -0x2c` | `[[sp-28]+12]` |
| `ldr` at 0xd6f4c | load | 4 | `abs 0xd6f7c` | `-` |
| `ldr` at 0xd6f50 | load | 4 | `abs 0xd6f80` | `-` |
| `ldrb` at 0xd6f5c | load | 1 | `sp -0x21` | `-` |
| `strb` at 0xd6f64 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldr` at 0xd6f6c | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xd6f74 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd6f74 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd6f74 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd6e28 | `(unknown callee) 0xd75c0` | `?` | `?` | `?` | `?` |
| `bl` at 0xd6e40 | `(unknown callee) 0xd76d4` | `[sp-20]` | `[sp-33]:1` | `?` | `[sp-33]:1` |
| `bl` at 0xd6f58 | `(unknown callee) 0x21e28` | `0x1` | `0x13470c` | `0x2d3` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xd6e14 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0xd6e58 | `[[sp-28]+8] == 0x0` | `[[sp-28]+8] != 0x0` |
| `beq` at 0xd6e68 | `[ret(0xd76d4)]:1 == 0x0` | `[ret(0xd76d4)]:1 != 0x0` |
| `ldrls` at 0xd6e78 | `[[sp-28]]:1 <= 0x4 (unsigned, 32-bit)` | `[[sp-28]]:1 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd6f74 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xd6e78).

## `ioExpPortInt`

- `ioExpPortInt`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd6f84 to 0xd7168.
  Established. Three comparisons take the skip arm on inequality and fall through on equality:
  `[sp-21]:1 != [[sp-28]+1]:1`, `[sp-22]:1 != [[sp-28]+2]:1`, and `[sp-23]:1 != [[sp-28]+3]:1`
  (`bne` at 0xd7044, `bne` at 0xd7058, `bne` at 0xd706c). The call to `0xd7168` has `[sp-20]`,
  `[sp-28]`, `ret(0xd76d4)`, `[sp-33]:1` (`bl` at 0xd7088). A separate word store writes `[sp-24]:1`
  at `sp -0x44` (`str` at 0xd7078). Port identity, argument count and the merged r0 return are
  unknown (`ldmdb` at 0xd7150).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd6f94 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xd6f9c | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xd6fa4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xd6fac | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0xd6fb4 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `strb` at 0xd6fbc | store | 1 | `sp -0x18` | `([sp] & 0xff)` |
| `ldr` at 0xd6fc0 | load | 4 | `abs 0xd7154` | `-` |
| `ldr` at 0xd6fc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd6fd8 | load | 4 | `abs 0x2ad21c` | `-` |
| `str` at 0xd6fe0 | store | 4 | `sp -0x28` | `?` |
| `strb` at 0xd6fe8 | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0xd6fec | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xd6ff0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd6ff4 | load | 4 | `[sp-40] +0x4` | `-` |
| `ldrb` at 0xd7004 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xd7008 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd7018 | store | 4 | `sp -0x1c` | `ret(0xd75c0)` |
| `ldrb` at 0xd701c | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xd7020 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd7030 | store | 4 | `sp -0x20` | `ret(0xd76d4)` |
| `ldr` at 0xd7034 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd7038 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xd703c | load | 1 | `[sp-28] +0x1` | `-` |
| `ldr` at 0xd7048 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd704c | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0xd7050 | load | 1 | `[sp-28] +0x2` | `-` |
| `ldr` at 0xd705c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd7060 | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0xd7064 | load | 1 | `[sp-28] +0x3` | `-` |
| `ldrb` at 0xd7070 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0xd7074 | load | 1 | `sp -0x18` | `-` |
| `str` at 0xd7078 | store | 4 | `sp -0x44` | `[sp-24]:1` |
| `ldr` at 0xd707c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd7080 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd7084 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd7090 | store | 4 | `sp -0x30` | `ret(0xd7168)` |
| `ldrb` at 0xd7098 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0xd70a0 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldr` at 0xd70a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd70b0 | load | 4 | `abs 0xd7158` | `-` |
| `ldr` at 0xd70b4 | load | 4 | `abs 0xd715c` | `-` |
| `str` at 0xd70e0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd70e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd70ec | load | 4 | `abs 0xd7158` | `-` |
| `ldr` at 0xd70f0 | load | 4 | `abs 0xd715c` | `-` |
| `ldr` at 0xd70fc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd710c | load | 4 | `abs 0xd7160` | `-` |
| `str` at 0xd7110 | store | 4 | `(unknown)` | `0x1348d4` |
| `str` at 0xd7114 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldrb` at 0xd7118 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xd711c | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldrb` at 0xd7120 | load | 1 | `sp -0x16` | `-` |
| `str` at 0xd7124 | store | 4 | `(unknown)` | `[sp-22]:1` |
| `ldrb` at 0xd7128 | load | 1 | `sp -0x17` | `-` |
| `str` at 0xd712c | store | 4 | `(unknown)` | `[sp-23]:1` |
| `ldr` at 0xd7134 | load | 4 | `abs 0xd7164` | `-` |
| `str` at 0xd7148 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xd714c | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xd7150 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd7150 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd7150 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd7010 | `(unknown callee) 0xd75c0` | `?` | `?` | `[[sp-40]+4]` | `?` |
| `bl` at 0xd7028 | `(unknown callee) 0xd76d4` | `[sp-20]` | `[sp-33]:1` | `?` | `[sp-33]:1` |
| `bl` at 0xd7088 | `(unknown callee) 0xd7168` | `[sp-20]` | `[sp-28]` | `ret(0xd76d4)` | `[sp-33]:1` |
| `bl` at 0xd70b8 | `(unknown callee) 0x4f564` | `?` | `0x0` | `0x134760` | `0x134768` |
| `bl` at 0xd70c4 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xd70f4 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x134760` | `0x134768` |
| `bl` at 0xd7104 | `(unknown callee) 0x1b508` | `[sp-44]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xd7140 | `(unknown callee) 0x21e28` | `0x1` | `0x13470c` | `0x2f4` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xd6ffc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xd7044 | `[sp-21]:1 != [[sp-28]+1]:1` | `[sp-21]:1 == [[sp-28]+1]:1` |
| `bne` at 0xd7058 | `[sp-22]:1 != [[sp-28]+2]:1` | `[sp-22]:1 == [[sp-28]+2]:1` |
| `bne` at 0xd706c | `[sp-23]:1 != [[sp-28]+3]:1` | `[sp-23]:1 == [[sp-28]+3]:1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd7150 | `r0` | `?` |

## `ioExpIntHandler`

- `ioExpIntHandler`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd7168 to 0xd7474.
  Established. The zero result of `0xd7c60` takes the update arm, while nonzero falls through (`beq`
  at 0xd71e8). That arm writes word `[[sp-32]+8]+1` at `[sp-32] +0x8` (`str` at 0xd72a8); its
  indirect call has unknown target and candidates `[sp-24]`, `[sp-33]:1`, unknown, `[[sp-32]+2]:1`,
  with a known continuation (`mov` at 0xd72e0). A different path stores byte `([r2+5]:1-1 & 0xff)`
  at entry `r2 +0x5` (`strb` at 0xd732c). Physical interrupt handling and the joined return remain
  unknown (`ldmdb` at 0xd7454).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd7178 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xd717c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xd7180 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0xd7184 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xd7188 | store | 1 | `sp -0x21` | `(r3 & 0xff)` |
| `strb` at 0xd7190 | store | 1 | `sp -0x22` | `([sp] & 0xff)` |
| `ldr` at 0xd7194 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd7198 | load | 4 | `r1 +0x8` | `-` |
| `ldr` at 0xd71a4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd71a8 | load | 1 | `sp -0x22` | `-` |
| `ldrb` at 0xd71ac | load | 1 | `r2 +0x3` | `-` |
| `ldr` at 0xd71c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd71c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd71cc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd71dc | store | 4 | `sp -0x28` | `ret(0xd7c60)` |
| `ldr` at 0xd71e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd71ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd71f4 | load | 4 | `abs 0xd7458` | `-` |
| `ldr` at 0xd71f8 | load | 4 | `abs 0xd745c` | `-` |
| `str` at 0xd7224 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd7228 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd7230 | load | 4 | `abs 0xd7458` | `-` |
| `ldr` at 0xd7234 | load | 4 | `abs 0xd745c` | `-` |
| `ldr` at 0xd7240 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd7250 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd725c | load | 4 | `abs 0xd7460` | `-` |
| `str` at 0xd7260 | store | 4 | `(unknown)` | `0x13490c` |
| `str` at 0xd7264 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0xd7268 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd726c | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xd7270 | store | 4 | `(unknown)` | `[[sp-28]+12]` |
| `str` at 0xd7274 | store | 4 | `(unknown)` | `ret(0x219a8)` |
| `ldr` at 0xd727c | load | 4 | `abs 0xd7464` | `-` |
| `str` at 0xd7290 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xd7298 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd729c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd72a0 | load | 4 | `[sp-32] +0x8` | `-` |
| `str` at 0xd72a8 | store | 4 | `[sp-32] +0x8` | `[[sp-32]+8]+1` |
| `ldr` at 0xd72ac | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd72b0 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xd72b4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd72b8 | load | 1 | `sp -0x22` | `-` |
| `ldrb` at 0xd72bc | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0xd72c8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd72cc | load | 1 | `[sp-32] +0x2` | `-` |
| `ldr` at 0xd72d0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xd72d4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd72e8 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xd72f0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd72f4 | load | 1 | `sp -0x22` | `-` |
| `ldrb` at 0xd72f8 | load | 1 | `r2 +0x4` | `-` |
| `ldr` at 0xd730c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd7310 | load | 1 | `r2 +0x5` | `-` |
| `ldr` at 0xd731c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd7320 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd7324 | load | 1 | `r2 +0x5` | `-` |
| `strb` at 0xd732c | store | 1 | `r2 +0x5` | `([r2+5]:1-1 & 0xff)` |
| `str` at 0xd7334 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xd733c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd7344 | load | 4 | `abs 0xd7458` | `-` |
| `ldr` at 0xd7348 | load | 4 | `abs 0xd745c` | `-` |
| `str` at 0xd7374 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd7378 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd7380 | load | 4 | `abs 0xd7458` | `-` |
| `ldr` at 0xd7384 | load | 4 | `abs 0xd745c` | `-` |
| `ldr` at 0xd7390 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd73a0 | load | 4 | `abs 0xd7468` | `-` |
| `str` at 0xd73a4 | store | 4 | `(unknown)` | `0x134930` |
| `str` at 0xd73a8 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0xd73ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd73b0 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xd73b4 | store | 4 | `(unknown)` | `[[sp-28]+12]` |
| `ldrb` at 0xd73b8 | load | 1 | `sp -0x22` | `-` |
| `str` at 0xd73bc | store | 4 | `(unknown)` | `[sp-34]:1` |
| `ldr` at 0xd73c0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd73c4 | load | 1 | `[sp-32] +0x3` | `-` |
| `str` at 0xd73c8 | store | 4 | `(unknown)` | `[[sp-32]+3]:1` |
| `ldr` at 0xd73cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd73d0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xd73dc | load | 4 | `abs 0xd746c` | `-` |
| `str` at 0xd73e0 | store | 4 | `sp -0x34` | `0x134974` |
| `ldr` at 0xd73e8 | load | 4 | `abs 0xd7470` | `-` |
| `str` at 0xd73ec | store | 4 | `sp -0x34` | `0x134978` |
| `ldr` at 0xd73f0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xd73f4 | store | 4 | `(unknown)` | `[sp-52]` |
| `ldr` at 0xd73fc | load | 4 | `abs 0xd7464` | `-` |
| `ldrb` at 0xd740c | load | 1 | `sp -0x21` | `-` |
| `str` at 0xd7410 | store | 4 | `sp -0x38` | `[sp-33]:1` |
| `ldr` at 0xd7414 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd7418 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0xd7424 | load | 1 | `sp -0x22` | `-` |
| `str` at 0xd7428 | store | 4 | `sp -0x3c` | `[sp-34]:1` |
| `str` at 0xd7434 | store | 4 | `sp -0x3c` | `0xff` |
| `ldr` at 0xd7438 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd743c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd7440 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd744c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xd7450 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xd7454 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd7454 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd7454 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd7454 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd71d4 | `(unknown callee) 0xd7c60` | `r0` | `r1` | `r2+2` | `r2+2` |
| `bl` at 0xd71fc | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x134760` | `0x134768` |
| `bl` at 0xd7208 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xd7238 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x134760` | `0x134768` |
| `bl` at 0xd7248 | `(unknown callee) 0x1b508` | `[sp-44]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xd7254 | `(unknown callee) 0x219a8` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xd7288 | `(unknown callee) 0x21e28` | `0x3` | `0x13470c` | `0x308` | `0x0` |
| `mov` at 0xd72e0 | `(unknown callee) (unknown target)` | `[sp-24]` | `[sp-33]:1` | `?` | `[[sp-32]+2]:1` |
| `bl` at 0xd734c | `(unknown callee) 0x4f564` | `r0` | `0x0` | `0x134760` | `0x134768` |
| `bl` at 0xd7358 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xd7388 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x134760` | `0x134768` |
| `bl` at 0xd7398 | `(unknown callee) 0x1b508` | `[sp-44]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xd7408 | `(unknown callee) 0x21e28` | `0x1` | `0x13470c` | `0x324` | `0x0` |
| `bl` at 0xd7444 | `(unknown callee) 0xd6cf8` | `[sp-24]` | `[sp-33]:1` | `[sp-60]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd71a0 | `[r1+8] == 0x0` | `[r1+8] != 0x0` |
| `beq` at 0xd71bc | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd71e8 | `ret(0xd7c60) == 0x0` | `ret(0xd7c60) != 0x0` |
| `beq` at 0xd7308 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd7318 | `[r2+5]:1 == 0x0` | `[r2+5]:1 != 0x0` |
| `beq` at 0xd73d8 | `[[sp-28]+8] == 0x0` | `[[sp-28]+8] != 0x0` |
| `beq` at 0xd7420 | `[[sp-28]+8] == 0x0` | `[[sp-28]+8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd7454 | `r0` | `?` |

## `ioExpIgnoreInt`

- `ioExpIgnoreInt`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd7474 to 0xd7558.
  Established. The final call targets `0x21e28` with `0x6`, `0x13470c`, `0x337`, `0x0` (`bl` at
  0xd753c); its result remains in r0 at the return (`ldmdb` at 0xd7540). Before that call, three
  word stores write `[sp-21]:1`, `[sp-22]:1`, `[sp-23]:1` to unknown targets, respectively
  (`str` at 0xd7518, `str` at 0xd7520, `str` at 0xd7528). No conditional branches are reported. This
  body does not prove physical interrupt masking.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd7484 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd7490 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xd7498 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0xd74a0 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldr` at 0xd74a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd74ac | load | 4 | `abs 0xd7544` | `-` |
| `ldr` at 0xd74b0 | load | 4 | `abs 0xd7548` | `-` |
| `str` at 0xd74dc | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xd74e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd74e8 | load | 4 | `abs 0xd7544` | `-` |
| `ldr` at 0xd74ec | load | 4 | `abs 0xd7548` | `-` |
| `ldr` at 0xd74f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd7508 | load | 4 | `abs 0xd754c` | `-` |
| `str` at 0xd750c | store | 4 | `(unknown)` | `0x134988` |
| `str` at 0xd7510 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldrb` at 0xd7514 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xd7518 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldrb` at 0xd751c | load | 1 | `sp -0x16` | `-` |
| `str` at 0xd7520 | store | 4 | `(unknown)` | `[sp-22]:1` |
| `ldrb` at 0xd7524 | load | 1 | `sp -0x17` | `-` |
| `str` at 0xd7528 | store | 4 | `(unknown)` | `[sp-23]:1` |
| `ldr` at 0xd7530 | load | 4 | `abs 0xd7550` | `-` |
| `ldr` at 0xd7534 | load | 4 | `abs 0xd7554` | `-` |
| `ldmdb` at 0xd7540 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd7540 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd7540 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd74b4 | `(unknown callee) 0x4f564` | `r0` | `0x0` | `0x134760` | `0x134768` |
| `bl` at 0xd74c0 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xd74f0 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x134760` | `0x134768` |
| `bl` at 0xd7500 | `(unknown callee) 0x1b508` | `[sp-28]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xd753c | `(unknown callee) 0x21e28` | `0x6` | `0x13470c` | `0x337` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd7540 | `r0` | `ret(0x21e28)` |

## `ioExpGetCfgIndex`

- `ioExpGetCfgIndex`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd75c0 to 0xd76d4.
  Established. The signed bound takes the final path for `(r1 & 0xff) < [[sp-28]+4]` and falls
  through for greater or equal (`blt` at 0xd7610). The fall-through path includes `0x21e28` with
  `0x9`, `0x13470c`, `0x357`, `0x0` (`bl` at 0xd769c). The final word read addresses `[sp-28] +0x0`
  (`ldr` at 0xd76ac). The indexed result and record layout remain unknown in the returned r0 value
  (`ldmdb` at 0xd76b8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd75d0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd75d8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xd75dc | load | 4 | `abs 0xd76bc` | `-` |
| `ldr` at 0xd75e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd75f4 | load | 4 | `abs 0x2ad21c` | `-` |
| `str` at 0xd75fc | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xd7600 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd7604 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd7608 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xd7614 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd761c | load | 4 | `abs 0xd76c0` | `-` |
| `ldr` at 0xd7620 | load | 4 | `abs 0xd76c4` | `-` |
| `str` at 0xd764c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xd7650 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd7658 | load | 4 | `abs 0xd76c0` | `-` |
| `ldr` at 0xd765c | load | 4 | `abs 0xd76c4` | `-` |
| `ldr` at 0xd7668 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd7678 | load | 4 | `abs 0xd76c8` | `-` |
| `str` at 0xd767c | store | 4 | `(unknown)` | `0x1349d0` |
| `str` at 0xd7680 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldrb` at 0xd7684 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xd7688 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0xd7690 | load | 4 | `abs 0xd76cc` | `-` |
| `ldr` at 0xd7694 | load | 4 | `abs 0xd76d0` | `-` |
| `ldr` at 0xd76a0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd76a4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd76ac | load | 4 | `[sp-28] +0x0` | `-` |
| `ldmdb` at 0xd76b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd76b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd76b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd7624 | `(unknown callee) 0x4f564` | `r0` | `0x0` | `0x134760` | `0x134768` |
| `bl` at 0xd7630 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xd7660 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x134760` | `0x134768` |
| `bl` at 0xd7670 | `(unknown callee) 0x1b508` | `[sp-32]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xd769c | `(unknown callee) 0x21e28` | `0x9` | `0x13470c` | `0x357` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xd7610 | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd76b8 | `r0` | `?` |

## `ioExpGetValIndex`

- `ioExpGetValIndex`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd76d4 to 0xd77f4.
  Established. The signed comparison takes the final path for `(r1 & 0xff) < [[sp-28]+4]` and falls
  through for greater or equal (`blt` at 0xd7724); its fall-through includes a call to `0x21e28`
  with `0x9`, `0x13470c`, `0x365`, `0x0` (`bl` at 0xd77b0). A final word read addresses `[sp-28]
  +0x8` (`ldr` at 0xd77cc). The r0 result is unknown, so the indexed record extent is not
  established (`ldmdb` at 0xd77d8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd76e4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd76ec | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xd76f0 | load | 4 | `abs 0xd77dc` | `-` |
| `ldr` at 0xd76f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd7708 | load | 4 | `abs 0x2ad21c` | `-` |
| `str` at 0xd7710 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xd7714 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd7718 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd771c | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xd7728 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd7730 | load | 4 | `abs 0xd77e0` | `-` |
| `ldr` at 0xd7734 | load | 4 | `abs 0xd77e4` | `-` |
| `str` at 0xd7760 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xd7764 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd776c | load | 4 | `abs 0xd77e0` | `-` |
| `ldr` at 0xd7770 | load | 4 | `abs 0xd77e4` | `-` |
| `ldr` at 0xd777c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd778c | load | 4 | `abs 0xd77e8` | `-` |
| `str` at 0xd7790 | store | 4 | `(unknown)` | `0x1349d0` |
| `str` at 0xd7794 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldrb` at 0xd7798 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xd779c | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0xd77a4 | load | 4 | `abs 0xd77ec` | `-` |
| `ldr` at 0xd77a8 | load | 4 | `abs 0xd77f0` | `-` |
| `ldr` at 0xd77b4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd77b8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd77cc | load | 4 | `[sp-28] +0x8` | `-` |
| `ldmdb` at 0xd77d8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd77d8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd77d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd7738 | `(unknown callee) 0x4f564` | `r0` | `0x0` | `0x134760` | `0x134768` |
| `bl` at 0xd7744 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xd7774 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x134760` | `0x134768` |
| `bl` at 0xd7784 | `(unknown callee) 0x1b508` | `[sp-32]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xd77b0 | `(unknown callee) 0x21e28` | `0x9` | `0x13470c` | `0x365` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xd7724 | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd77d8 | `r0` | `?` |

## `ioExpWriteExp`

- `ioExpWriteExp`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd77f4 to 0xd7c60.
  Established. Two byte stores address `[sp-28] +0x1`, with unknown values (`strb` at 0xd79a4,
  `strb` at 0xd79bc). The computed transfer takes an unresolved edge for `[[sp-24]]:1 <= 0x4`
  unsigned and falls through above the bound (`ldrls` at 0xd79cc). The known default arm calls
  `0x21e28` with `0x1`, `0x13470c`, `0x3b1`, `0x0`, then stores word `0x0` at `sp -0x2c` (`bl` at
  0xd7c28, `str` at 0xd7c30). The known-edge return has r0 `0x0` (`ldmdb` at 0xd7c38).
  Excluded write cases and physical effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd7804 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd7808 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xd780c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xd7810 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xd7814 | store | 1 | `sp -0x1d` | `(r3 & 0xff)` |
| `strb` at 0xd781c | store | 1 | `sp -0x1e` | `([sp] & 0xff)` |
| `ldr` at 0xd7820 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd7824 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0xd7828 | load | 1 | `r1 +0x4` | `-` |
| `ldr` at 0xd783c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd7844 | load | 4 | `abs 0xd7c3c` | `-` |
| `ldr` at 0xd7848 | load | 4 | `abs 0xd7c40` | `-` |
| `str` at 0xd7874 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xd7878 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd7880 | load | 4 | `abs 0xd7c3c` | `-` |
| `ldr` at 0xd7884 | load | 4 | `abs 0xd7c40` | `-` |
| `ldr` at 0xd7890 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd78a0 | load | 4 | `abs 0xd7c44` | `-` |
| `str` at 0xd78a4 | store | 4 | `(unknown)` | `0x1349f4` |
| `str` at 0xd78a8 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0xd78ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd78b0 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xd78b4 | store | 4 | `(unknown)` | `[[sp-24]+12]` |
| `ldrb` at 0xd78b8 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0xd78bc | store | 4 | `(unknown)` | `[sp-29]:1` |
| `ldr` at 0xd78c4 | load | 4 | `abs 0xd7c48` | `-` |
| `ldr` at 0xd78c8 | load | 4 | `abs 0xd7c4c` | `-` |
| `ldrb` at 0xd78d4 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0xd78dc | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0xd78ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd78f4 | load | 4 | `abs 0xd7c3c` | `-` |
| `ldr` at 0xd78f8 | load | 4 | `abs 0xd7c40` | `-` |
| `str` at 0xd7924 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xd7928 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd7930 | load | 4 | `abs 0xd7c3c` | `-` |
| `ldr` at 0xd7934 | load | 4 | `abs 0xd7c40` | `-` |
| `ldr` at 0xd7940 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd7950 | load | 4 | `abs 0xd7c50` | `-` |
| `str` at 0xd7954 | store | 4 | `(unknown)` | `0x134a2c` |
| `str` at 0xd7958 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0xd795c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd7960 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xd7964 | store | 4 | `(unknown)` | `[[sp-24]+12]` |
| `ldrb` at 0xd7968 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0xd796c | store | 4 | `(unknown)` | `[sp-29]:1` |
| `ldrb` at 0xd7970 | load | 1 | `sp -0x1e` | `-` |
| `str` at 0xd7974 | store | 4 | `(unknown)` | `[sp-30]:1` |
| `ldr` at 0xd797c | load | 4 | `abs 0xd7c48` | `-` |
| `ldr` at 0xd7980 | load | 4 | `abs 0xd7c54` | `-` |
| `ldr` at 0xd798c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd7990 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd7994 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0xd799c | load | 1 | `[sp-28] +0x1` | `-` |
| `strb` at 0xd79a4 | store | 1 | `[sp-28] +0x1` | `?` |
| `ldr` at 0xd79a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd79ac | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd79b0 | load | 1 | `[sp-28] +0x1` | `-` |
| `ldrb` at 0xd79b4 | load | 1 | `sp -0x1e` | `-` |
| `strb` at 0xd79bc | store | 1 | `[sp-28] +0x1` | `?` |
| `ldr` at 0xd79c0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd79c4 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldrls` at 0xd79cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd7b90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd7b98 | load | 4 | `abs 0xd7c3c` | `-` |
| `ldr` at 0xd7b9c | load | 4 | `abs 0xd7c40` | `-` |
| `str` at 0xd7bc8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xd7bcc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd7bd4 | load | 4 | `abs 0xd7c3c` | `-` |
| `ldr` at 0xd7bd8 | load | 4 | `abs 0xd7c40` | `-` |
| `ldr` at 0xd7be4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd7bf4 | load | 4 | `abs 0xd7c58` | `-` |
| `str` at 0xd7bf8 | store | 4 | `(unknown)` | `0x134a64` |
| `ldr` at 0xd7bfc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd7c00 | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0xd7c04 | store | 4 | `(unknown)` | `[[sp-24]]:1` |
| `str` at 0xd7c08 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0xd7c0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd7c10 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xd7c14 | store | 4 | `(unknown)` | `[[sp-24]+12]` |
| `ldr` at 0xd7c1c | load | 4 | `abs 0xd7c48` | `-` |
| `ldr` at 0xd7c20 | load | 4 | `abs 0xd7c5c` | `-` |
| `str` at 0xd7c30 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xd7c34 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xd7c38 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd7c38 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd7c38 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd784c | `(unknown callee) 0x4f564` | `r0` | `0x0` | `0x134760` | `0x134768` |
| `bl` at 0xd7858 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xd7888 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x134760` | `0x134768` |
| `bl` at 0xd7898 | `(unknown callee) 0x1b508` | `[sp-40]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xd78d0 | `(unknown callee) 0x21e28` | `0x9` | `0x13470c` | `0x37d` | `0x0` |
| `bl` at 0xd78fc | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x134760` | `0x134768` |
| `bl` at 0xd7908 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xd7938 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x134760` | `0x134768` |
| `bl` at 0xd7948 | `(unknown callee) 0x1b508` | `[sp-40]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xd7988 | `(unknown callee) 0x21e28` | `0x9` | `0x13470c` | `0x381` | `0x0` |
| `bl` at 0xd7ba0 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x134760` | `0x134768` |
| `bl` at 0xd7bac | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xd7bdc | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x134760` | `0x134768` |
| `bl` at 0xd7bec | `(unknown callee) 0x1b508` | `[sp-40]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xd7c28 | `(unknown callee) 0x21e28` | `0x1` | `0x13470c` | `0x3b1` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd7838 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd78e8 | `? == 0x0` | `? != 0x0` |
| `ldrls` at 0xd79cc | `[[sp-24]]:1 <= 0x4 (unsigned, 32-bit)` | `[[sp-24]]:1 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd7c38 | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xd79cc).

## `ioExpReadExp`

- `ioExpReadExp`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd7c60 to 0xd7e6c.
  Established. The selector takes an unresolved edge for `[r1]:1 <= 0x4` unsigned and falls through
  for greater (`ldrls` at 0xd7c88). The retained default arm calls `0x21e28` with `0x1`, `0x13470c`,
  `0x3d5`, `0x0` (`bl` at 0xd7e44). It stores word `0x0` at `sp -0x24`, which supplies r0 `0x0` at
  the return (`str` at 0xd7e4c, `ldmdb` at 0xd7e54). Excluded read destinations and the external
  device protocol remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd7c70 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd7c74 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xd7c78 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xd7c7c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd7c80 | load | 1 | `r1 +0x0` | `-` |
| `ldrls` at 0xd7c88 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd7dac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd7db4 | load | 4 | `abs 0xd7e58` | `-` |
| `ldr` at 0xd7db8 | load | 4 | `abs 0xd7e5c` | `-` |
| `str` at 0xd7de4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xd7de8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd7df0 | load | 4 | `abs 0xd7e58` | `-` |
| `ldr` at 0xd7df4 | load | 4 | `abs 0xd7e5c` | `-` |
| `ldr` at 0xd7e00 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd7e10 | load | 4 | `abs 0xd7e60` | `-` |
| `str` at 0xd7e14 | store | 4 | `(unknown)` | `0x134a64` |
| `ldr` at 0xd7e18 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd7e1c | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0xd7e20 | store | 4 | `(unknown)` | `[[sp-24]]:1` |
| `str` at 0xd7e24 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0xd7e28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd7e2c | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xd7e30 | store | 4 | `(unknown)` | `[[sp-24]+12]` |
| `ldr` at 0xd7e38 | load | 4 | `abs 0xd7e64` | `-` |
| `ldr` at 0xd7e3c | load | 4 | `abs 0xd7e68` | `-` |
| `str` at 0xd7e4c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xd7e50 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xd7e54 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd7e54 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd7e54 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd7dbc | `(unknown callee) 0x4f564` | `r0` | `0x0` | `0x134760` | `0x134768` |
| `bl` at 0xd7dc8 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xd7df8 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x134760` | `0x134768` |
| `bl` at 0xd7e08 | `(unknown callee) 0x1b508` | `[sp-32]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xd7e44 | `(unknown callee) 0x21e28` | `0x1` | `0x13470c` | `0x3d5` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xd7c88 | `[r1]:1 <= 0x4 (unsigned, 32-bit)` | `[r1]:1 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd7e54 | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xd7c88).
