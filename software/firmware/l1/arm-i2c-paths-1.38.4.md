# ARM I2C entry paths in 1.38.4

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
The fact sheets report no unresolved computed transfers. The indirect call in `i2cSelectBus`
has an unknown target and a known continuation at `0xc7cc4` (`mov` at 0xc7cbc, 0xc7cc0).
Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/i2c-paths-facts.txt) retain all tool sections.

## `i2cInit`

- `i2cInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc75ac to 0xc7784.
  Established. The first signed test takes its bypass for `[0x1429c4] < 0x0` and falls through for
  greater-or-equal (`blt` at 0xc75cc). A call to `0x1b0a0` carries `0x0`, `0x100`, `0x3`, `0x1`, and
  its result is stored as a word at absolute `0x2b21a0` (`bl` at 0xc7668, `str` at 0xc766c). A later
  word store replaces that location with `[0x2b21a0]+36` (`str` at 0xc76c4). Near the end, calls
  reach `0xd5760` with four unknown candidates and `0xc7d14` with `ret(0xd5760)`, unknown, unknown,
  unknown (`bl` at 0xc7740, `bl` at 0xc7744). Physical initialization and the returned r0 value
  remain unknown (`ldmdb` at 0xc7758).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc75bc | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xc75c0 | load | 4 | `abs 0xc775c` | `-` |
| `ldr` at 0xc75c4 | load | 4 | `abs 0x1429c4` | `-` |
| `ldr` at 0xc75d0 | load | 4 | `abs 0xc7760` | `-` |
| `str` at 0xc75d4 | store | 4 | `sp -0x28` | `0x132f04` |
| `ldr` at 0xc75dc | load | 4 | `abs 0xc7764` | `-` |
| `ldr` at 0xc75ec | load | 4 | `abs 0xc7768` | `-` |
| `ldr` at 0xc75f0 | load | 4 | `abs 0xc776c` | `-` |
| `str` at 0xc75fc | store | 4 | `sp -0x1c` | `ret(0x1b370)` |
| `ldr` at 0xc7600 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc7618 | load | 4 | `abs 0xc7770` | `-` |
| `str` at 0xc761c | store | 4 | `sp -0x28` | `0x132f30` |
| `ldr` at 0xc7620 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `str` at 0xc7624 | store | 4 | `sp -0x24` | `[ret(0x1b0b8)]` |
| `ldr` at 0xc762c | load | 4 | `abs 0xc7764` | `-` |
| `ldr` at 0xc7644 | load | 4 | `abs 0xc7774` | `-` |
| `ldr` at 0xc7648 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc764c | store | 4 | `sp -0x28` | `?` |
| `str` at 0xc7654 | store | 4 | `sp -0x24` | `0x56000000` |
| `str` at 0xc766c | store | 4 | `abs 0x2b21a0` | `ret(0x1b0a0)` |
| `ldr` at 0xc7670 | load | 4 | `abs 0x2b21a0` | `-` |
| `ldr` at 0xc7688 | load | 4 | `abs 0xc7778` | `-` |
| `str` at 0xc768c | store | 4 | `sp -0x28` | `0x132f50` |
| `ldr` at 0xc7690 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `str` at 0xc7694 | store | 4 | `sp -0x24` | `[ret(0x1b0b8)]` |
| `ldr` at 0xc769c | load | 4 | `abs 0xc7764` | `-` |
| `ldr` at 0xc76b4 | load | 4 | `abs 0xc7774` | `-` |
| `ldr` at 0xc76b8 | load | 4 | `abs 0xc7774` | `-` |
| `ldr` at 0xc76bc | load | 4 | `abs 0x2b21a0` | `-` |
| `str` at 0xc76c4 | store | 4 | `abs 0x2b21a0` | `[0x2b21a0]+36` |
| `ldr` at 0xc76c8 | load | 4 | `abs 0xc777c` | `-` |
| `ldr` at 0xc76d8 | load | 4 | `abs 0xc775c` | `-` |
| `str` at 0xc76dc | store | 4 | `abs 0x1429c4` | `ret(0x1b370)` |
| `ldr` at 0xc76e0 | load | 4 | `abs 0x1429c4` | `-` |
| `ldr` at 0xc76f4 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xc7700 | load | 4 | `abs 0xc7780` | `-` |
| `str` at 0xc7704 | store | 4 | `sp -0x28` | `0x132f88` |
| `str` at 0xc7708 | store | 4 | `sp -0x24` | `ret(0x21674)` |
| `ldr` at 0xc7710 | load | 4 | `abs 0xc7764` | `-` |
| `strb` at 0xc7724 | store | 1 | `sp -0x1d` | `0x1` |
| `ldrb` at 0xc7728 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0xc7738 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0xc7748 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0xc7750 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldmdb` at 0xc7758 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc7758 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc7758 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc7758 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc75e8 | `(unknown callee) 0x21e28` | `0x9` | `0x132f1c` | `0x4d` | `0x0` |
| `bl` at 0xc75f4 | `(unknown callee) 0x1b370` | `0x132f24` | `0x1002` | `?` | `?` |
| `bl` at 0xc760c | `(unknown callee) 0x1b0b8` | `ret(0x1b370)` | `?` | `?` | `ret(0x1b370)` |
| `bl` at 0xc7638 | `(unknown callee) 0x21e28` | `0x9` | `0x132f1c` | `0x53` | `0x0` |
| `bl` at 0xc7640 | `(unknown callee) 0x1b3ac` | `0x1` | `?` | `?` | `?` |
| `bl` at 0xc7668 | `(unknown callee) 0x1b0a0` | `0x0` | `0x100` | `0x3` | `0x1` |
| `bl` at 0xc767c | `(unknown callee) 0x1b0b8` | `ret(0x1b0a0)` | `?` | `?` | `[0x2b21a0]` |
| `bl` at 0xc76a8 | `(unknown callee) 0x21e28` | `0x9` | `0x132f1c` | `0x5b` | `0x0` |
| `bl` at 0xc76b0 | `(unknown callee) 0x1b3ac` | `0x1` | `?` | `?` | `?` |
| `bl` at 0xc76d0 | `(unknown callee) 0x1b370` | `0x132f7c` | `0x2` | `0x2b21a0` | `[0x2b21a0]+36` |
| `bl` at 0xc76ec | `(unknown callee) 0x1b0b8` | `ret(0x1b370)` | `?` | `ret(0x1b370)` | `[0x1429c4]` |
| `bl` at 0xc76f8 | `(unknown callee) 0x21674` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0xc771c | `(unknown callee) 0x21e28` | `0x9` | `0x132f1c` | `0x65` | `0x0` |
| `bl` at 0xc7740 | `(unknown callee) 0xd5760` | `?` | `?` | `?` | `?` |
| `bl` at 0xc7744 | `(unknown callee) 0xc7d14` | `ret(0xd5760)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xc75cc | `[0x1429c4] < 0x0 (signed, 32-bit)` | `[0x1429c4] >= 0x0 (signed, 32-bit)` |
| `bge` at 0xc7608 | `ret(0x1b370) >= 0x0 (signed, 32-bit)` | `ret(0x1b370) < 0x0 (signed, 32-bit)` |
| `bne` at 0xc7678 | `Z == 0` | `not (Z == 0)` |
| `bge` at 0xc76e8 | `[0x1429c4] >= 0x0 (signed, 32-bit)` | `[0x1429c4] < 0x0 (signed, 32-bit)` |
| `bls` at 0xc7730 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc7758 | `r0` | `?` |

## `i2cRead`

- `i2cRead`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc7e48 to 0xc7e94.
  Established. The sole call reaches `0xc8134` with `0x1`, `(r0 & 0xff)`, unknown, `[sp-24]` (`bl`
  at 0xc7e84). Before it, a word store puts `(r2 & 0xffff)` at `sp -0x20` (`str` at 0xc7e70). The
  result of that call supplies r0 at the return (`ldmdb` at 0xc7e90). No conditional branch is
  reported, and the transfer protocol is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc7e5c | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xc7e60 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strh` at 0xc7e64 | store | 2 | `sp -0x1a` | `(r2 & 0xffff)` |
| `ldrb` at 0xc7e68 | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0xc7e6c | load | 2 | `sp -0x1a` | `-` |
| `str` at 0xc7e70 | store | 4 | `sp -0x20` | `(r2 & 0xffff)` |
| `ldr` at 0xc7e80 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xc7e90 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc7e90 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc7e90 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc7e84 | `(unknown callee) 0xc8134` | `0x1` | `(r0 & 0xff)` | `?` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc7e90 | `r0` | `ret(0xc8134)` |

## `i2cWrite`

- `i2cWrite`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc7da0 to 0xc7dec.
  Established. Entry r1 is retained as a word at `sp -0x18`, and `(r2 & 0xffff)` is stored as a word
  at `sp -0x20` (`str` at 0xc7db4, `str` at 0xc7dc8). The call to `0xc8134` carries `0x0`, `(r0 &
  0xff)`, unknown, `[sp-24]` (`bl` at 0xc7ddc); its result reaches the r0 return (`ldmdb` at
  0xc7de8). A physical bus-write interpretation is not established by this wrapper.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc7db4 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xc7db8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strh` at 0xc7dbc | store | 2 | `sp -0x1a` | `(r2 & 0xffff)` |
| `ldrb` at 0xc7dc0 | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0xc7dc4 | load | 2 | `sp -0x1a` | `-` |
| `str` at 0xc7dc8 | store | 4 | `sp -0x20` | `(r2 & 0xffff)` |
| `ldr` at 0xc7dd8 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xc7de8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc7de8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc7de8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc7ddc | `(unknown callee) 0xc8134` | `0x0` | `(r0 & 0xff)` | `?` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc7de8 | `r0` | `ret(0xc8134)` |

## `i2cReadOffset`

- `i2cReadOffset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc7e94 to 0xc7ef0.
  Established. The call to `0xc8134` has `0x1`, `(r0 & 0xff)`, `(r1 & 0xff)`, entry r2 (`bl` at
  0xc7ee0). A prior word store puts `(r3 & 0xffff)` at `sp -0x20` (`str` at 0xc7ecc). The
  returned r0 value is `ret(0xc8134)` (`ldmdb` at 0xc7eec). The offset format and callee argument
  count remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc7ea4 | store | 4 | `sp -0x18` | `r2` |
| `strb` at 0xc7eb0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xc7eb8 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strh` at 0xc7ebc | store | 2 | `sp -0x1a` | `(r3 & 0xffff)` |
| `ldrb` at 0xc7ec0 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xc7ec4 | load | 1 | `sp -0x12` | `-` |
| `ldrh` at 0xc7ec8 | load | 2 | `sp -0x1a` | `-` |
| `str` at 0xc7ecc | store | 4 | `sp -0x20` | `(r3 & 0xffff)` |
| `ldr` at 0xc7edc | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xc7eec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc7eec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc7eec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc7ee0 | `(unknown callee) 0xc8134` | `0x1` | `(r0 & 0xff)` | `(r1 & 0xff)` | `r2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc7eec | `r0` | `ret(0xc8134)` |

## `i2cWriteOffset`

- `i2cWriteOffset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc7dec to 0xc7e48.
  Established. A halfword store retains `(r3 & 0xffff)` at `sp -0x1a`, and a later word store copies
  that value to `sp -0x20` (`strh` at 0xc7e14, `str` at 0xc7e24). The only call carries `0x0`, `(r0
  & 0xff)`, `(r1 & 0xff)`, entry r2 to `0xc8134` (`bl` at 0xc7e38). The r0 return retains its result
  (`ldmdb` at 0xc7e44). The wrapper establishes no physical transaction grammar.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc7dfc | store | 4 | `sp -0x18` | `r2` |
| `strb` at 0xc7e08 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xc7e10 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strh` at 0xc7e14 | store | 2 | `sp -0x1a` | `(r3 & 0xffff)` |
| `ldrb` at 0xc7e18 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xc7e1c | load | 1 | `sp -0x12` | `-` |
| `ldrh` at 0xc7e20 | load | 2 | `sp -0x1a` | `-` |
| `str` at 0xc7e24 | store | 4 | `sp -0x20` | `(r3 & 0xffff)` |
| `ldr` at 0xc7e34 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xc7e44 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc7e44 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc7e44 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc7e38 | `(unknown callee) 0xc8134` | `0x0` | `(r0 & 0xff)` | `(r1 & 0xff)` | `r2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc7e44 | `r0` | `ret(0xc8134)` |

## `i2cReadByte`

- `i2cReadByte`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc801c to 0xc80a0.
  Established. A zero result from `0xc7a50` takes the continuing edge, while nonzero falls through
  to save `ret(0xc7a50)` as a word at `sp -0x20` (`bl` at 0xc8048, `beq` at 0xc805c, `str` at
  0xc8064). On continuation, the call to `0xc7e48` has `[sp-18]:1`, `[sp-24]`, `0x1`, `[sp-18]:1`
  (`bl` at 0xc807c). The following call to `0xc7b90` carries `0x0`, unknown, unknown, `ret(0xc7e48)`
  (`bl` at 0xc808c). The joined return and locking or bus-read semantics remain unknown (`ldmdb` at
  0xc809c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc8030 | store | 4 | `sp -0x18` | `r2` |
| `strb` at 0xc8034 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xc803c | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldrb` at 0xc8040 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xc8050 | store | 4 | `sp -0x1c` | `ret(0xc7a50)` |
| `ldr` at 0xc8054 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc8060 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc8064 | store | 4 | `sp -0x20` | `ret(0xc7a50)` |
| `ldrb` at 0xc806c | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xc8074 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc8084 | store | 4 | `sp -0x1c` | `ret(0xc7e48)` |
| `ldr` at 0xc8090 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc8094 | store | 4 | `sp -0x20` | `[sp-28]` |
| `ldr` at 0xc8098 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xc809c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc809c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc809c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc8048 | `(unknown callee) 0xc7a50` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xc807c | `(unknown callee) 0xc7e48` | `[sp-18]:1` | `[sp-24]` | `0x1` | `[sp-18]:1` |
| `bl` at 0xc808c | `(unknown callee) 0xc7b90` | `0x0` | `?` | `?` | `ret(0xc7e48)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc805c | `ret(0xc7a50) == 0x0` | `ret(0xc7a50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc809c | `r0` | `?` |

## `i2cWriteByte`

- `i2cWriteByte`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc7ef0 to 0xc7f7c.
  Established. The first call to `0xc7a50` carries `(r0 & 0xff)`, entry r1, entry r2, `(r0 & 0xff)`
  (`bl` at 0xc7f20); its zero result takes the continuation and nonzero falls through to an early
  saved result (`beq` at 0xc7f34, `str` at 0xc7f3c). The continuing call to `0xc7da0` uses
  `[sp-18]:1`, `sp-19`, `0x1`, `[sp-18]:1` (`bl` at 0xc7f58). A later call targets `0xc7b90` with
  `0x0`, unknown, unknown, `ret(0xc7da0)` (`bl` at 0xc7f68). The returned value and physical
  byte-write effect are unproven (`ldmdb` at 0xc7f78).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xc7f04 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xc7f0c | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0xc7f14 | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `ldrb` at 0xc7f18 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xc7f28 | store | 4 | `sp -0x18` | `ret(0xc7a50)` |
| `ldr` at 0xc7f2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc7f38 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc7f3c | store | 4 | `sp -0x1c` | `ret(0xc7a50)` |
| `ldrb` at 0xc7f44 | load | 1 | `sp -0x12` | `-` |
| `str` at 0xc7f60 | store | 4 | `sp -0x18` | `ret(0xc7da0)` |
| `ldr` at 0xc7f6c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc7f70 | store | 4 | `sp -0x1c` | `[sp-24]` |
| `ldr` at 0xc7f74 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xc7f78 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc7f78 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc7f78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc7f20 | `(unknown callee) 0xc7a50` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xc7f58 | `(unknown callee) 0xc7da0` | `[sp-18]:1` | `sp-19` | `0x1` | `[sp-18]:1` |
| `bl` at 0xc7f68 | `(unknown callee) 0xc7b90` | `0x0` | `?` | `?` | `ret(0xc7da0)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc7f34 | `ret(0xc7a50) == 0x0` | `ret(0xc7a50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc7f78 | `r0` | `?` |

## `i2cReadByteOffset`

- `i2cReadByteOffset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc80a0 to 0xc8134.
  Established. The initial branch takes its continuing edge for `ret(0xc7a50) == 0x0` and falls
  through on nonzero (`beq` at 0xc80e8). The continuing call to `0xc7e94` carries `[sp-18]:1`,
  `[sp-19]:1`, `[sp-24]`, `0x1` (`bl` at 0xc8110); its result is saved as a word at `sp -0x1c`
  before the call to `0xc7b90`, which has `0x0`, unknown, unknown, `ret(0xc7e94)` (`str` at 0xc8118,
  `bl` at 0xc8120). The joined r0 value and offset-read interpretation remain unknown (`ldmdb` at
  0xc8130).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc80b0 | store | 4 | `sp -0x18` | `r3` |
| `strb` at 0xc80b8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xc80c0 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0xc80c8 | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `ldrb` at 0xc80cc | load | 1 | `sp -0x11` | `-` |
| `str` at 0xc80dc | store | 4 | `sp -0x1c` | `ret(0xc7a50)` |
| `ldr` at 0xc80e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc80ec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc80f0 | store | 4 | `sp -0x20` | `ret(0xc7a50)` |
| `ldrb` at 0xc80f8 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0xc80fc | load | 1 | `sp -0x13` | `-` |
| `ldr` at 0xc8108 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc8118 | store | 4 | `sp -0x1c` | `ret(0xc7e94)` |
| `ldr` at 0xc8124 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc8128 | store | 4 | `sp -0x20` | `[sp-28]` |
| `ldr` at 0xc812c | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xc8130 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc8130 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc8130 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc80d4 | `(unknown callee) 0xc7a50` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xc8110 | `(unknown callee) 0xc7e94` | `[sp-18]:1` | `[sp-19]:1` | `[sp-24]` | `0x1` |
| `bl` at 0xc8120 | `(unknown callee) 0xc7b90` | `0x0` | `?` | `?` | `ret(0xc7e94)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc80e8 | `ret(0xc7a50) == 0x0` | `ret(0xc7a50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc8130 | `r0` | `?` |

## `i2cWriteByteOffset`

- `i2cWriteByteOffset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc7f7c to 0xc801c.
  Established. Four byte stores retain `(r0 & 0xff)`, `(r1 & 0xff)`, `(r2 & 0xff)`, `(r3 & 0xff)` at
  `sp -0x11` through `sp -0x14`, respectively (`strb` at 0xc7f94, `strb` at 0xc7f9c, `strb` at
  0xc7fa4, `strb` at 0xc7fac). After `0xc7a50`, a zero result takes the continuation and nonzero
  falls through to the early result (`bl` at 0xc7fb8, `beq` at 0xc7fcc, `str` at 0xc7fd4). The
  continuing call to `0xc7dec` has `[sp-18]:1`, `[sp-19]:1`, `sp-20`, `0x1` (`bl` at 0xc7ff8). The
  bus operation and merged r0 return remain unproven (`ldmdb` at 0xc8018).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xc7f94 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xc7f9c | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0xc7fa4 | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `strb` at 0xc7fac | store | 1 | `sp -0x14` | `(r3 & 0xff)` |
| `ldrb` at 0xc7fb0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xc7fc0 | store | 4 | `sp -0x18` | `ret(0xc7a50)` |
| `ldr` at 0xc7fc4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc7fd0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc7fd4 | store | 4 | `sp -0x1c` | `ret(0xc7a50)` |
| `ldrb` at 0xc7fdc | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0xc7fe0 | load | 1 | `sp -0x13` | `-` |
| `str` at 0xc8000 | store | 4 | `sp -0x18` | `ret(0xc7dec)` |
| `ldr` at 0xc800c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc8010 | store | 4 | `sp -0x1c` | `[sp-24]` |
| `ldr` at 0xc8014 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xc8018 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc8018 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc8018 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc7fb8 | `(unknown callee) 0xc7a50` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xc7ff8 | `(unknown callee) 0xc7dec` | `[sp-18]:1` | `[sp-19]:1` | `sp-20` | `0x1` |
| `bl` at 0xc8008 | `(unknown callee) 0xc7b90` | `0x0` | `?` | `?` | `ret(0xc7dec)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc7fcc | `ret(0xc7a50) == 0x0` | `ret(0xc7a50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc8018 | `r0` | `?` |

## `i2cReset`

- `i2cReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc7d14 to 0xc7da0.
  Established. The loop contains a call to `0x1b40c` with `[0x1429c4]`, `0x782`, `0x0`, `0x1429c4`
  (`bl` at 0xc7d4c); its signed nonnegative result takes the continuation and a negative result
  falls through to `0x1b0b8` (`bge` at 0xc7d58, `bl` at 0xc7d5c). The later call to `0x24028`
  uses `[0x144dc8]`, `0x133094`, `[ret(0x1b0b8)]`, `0x144dc8` (`bl` at 0xc7d78). Physical reset
  behavior and the r0 return are unknown (`ldmdb` at 0xc7d8c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc7d28 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0xc7d2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc7d3c | load | 4 | `abs 0xc7d90` | `-` |
| `ldr` at 0xc7d40 | load | 4 | `abs 0x1429c4` | `-` |
| `ldr` at 0xc7d44 | load | 4 | `abs 0xc7d94` | `-` |
| `ldr` at 0xc7d68 | load | 4 | `abs 0xc7d98` | `-` |
| `ldr` at 0xc7d6c | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0xc7d70 | load | 4 | `abs 0xc7d9c` | `-` |
| `ldr` at 0xc7d74 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xc7d7c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc7d84 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldmdb` at 0xc7d8c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc7d8c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc7d8c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc7d4c | `(unknown callee) 0x1b40c` | `[0x1429c4]` | `0x782` | `0x0` | `0x1429c4` |
| `bl` at 0xc7d5c | `(unknown callee) 0x1b0b8` | `ret(0x1b40c)` | `?` | `?` | `ret(0x1b40c)` |
| `bl` at 0xc7d78 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x133094` | `[ret(0x1b0b8)]` | `0x144dc8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xc7d34 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bge` at 0xc7d58 | `ret(0x1b40c) >= 0x0 (signed, 32-bit)` | `ret(0x1b40c) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc7d8c | `r0` | `?` |

## `i2cResetBusSelect`

- `i2cResetBusSelect`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc77fc to 0xc7860.
  Established. The absolute word at `0x1429c8` is read before a branch that takes its bypass on zero
  and falls through on nonzero (`ldr` at 0xc7810, `beq` at 0xc7818). The reached call to `0xc7a50`
  carries `0x1`, entry r1, entry r2, `[0x1429c8]` (`bl` at 0xc7820); its zero result takes the route
  to `0xc7b90`, whose candidates are `0x0`, unknown, unknown, `ret(0xc7a50)` (`beq` at 0xc7834, `bl`
  at 0xc7848). The merged return and bus-selection reset are unknown (`ldmdb` at 0xc7858).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xc780c | load | 4 | `abs 0xc785c` | `-` |
| `ldr` at 0xc7810 | load | 4 | `abs 0x1429c8` | `-` |
| `str` at 0xc7828 | store | 4 | `sp -0x14` | `ret(0xc7a50)` |
| `ldr` at 0xc782c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc7838 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc783c | store | 4 | `sp -0x18` | `ret(0xc7a50)` |
| `str` at 0xc7850 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xc7854 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xc7858 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc7858 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc7858 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc7820 | `(unknown callee) 0xc7a50` | `0x1` | `r1` | `r2` | `[0x1429c8]` |
| `bl` at 0xc7848 | `(unknown callee) 0xc7b90` | `0x0` | `?` | `?` | `ret(0xc7a50)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc7818 | `[0x1429c8] == 0x0` | `[0x1429c8] != 0x0` |
| `beq` at 0xc7834 | `ret(0xc7a50) == 0x0` | `ret(0xc7a50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc7858 | `r0` | `?` |

## `i2cSelectBus`

- `i2cSelectBus`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc7c84 to 0xc7d14.
  Established. The branch takes the fallback for `[0x1429c8] == 0x0` and falls through on nonzero
  (`beq` at 0xc7ca8). That fall-through route calls an unknown target with `(r0 & 0xff)`, entry r1,
  `(r0 & 0xff)`, `[0x1429c8]` (`mov` at 0xc7cc0); its result is stored as a word at `sp -0x18`
  (`str` at 0xc7cc8). The fallback instead stores word `0x0` there (`str` at 0xc7cf8). The actual
  selector, hardware action and joined r0 return remain unknown (`ldmdb` at 0xc7d00).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xc7c98 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xc7c9c | load | 4 | `abs 0xc7d04` | `-` |
| `ldr` at 0xc7ca0 | load | 4 | `abs 0x1429c8` | `-` |
| `ldr` at 0xc7cac | load | 4 | `abs 0xc7d04` | `-` |
| `ldrb` at 0xc7cb0 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xc7cb4 | load | 4 | `abs 0x1429c8` | `-` |
| `str` at 0xc7cc8 | store | 4 | `sp -0x18` | `ret(unknown)` |
| `ldr` at 0xc7cd0 | load | 4 | `abs 0xc7d08` | `-` |
| `str` at 0xc7cd4 | store | 4 | `sp -0x20` | `0x133078` |
| `ldrb` at 0xc7cd8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xc7cdc | store | 4 | `sp -0x1c` | `(r0 & 0xff)` |
| `ldr` at 0xc7ce4 | load | 4 | `abs 0xc7d0c` | `-` |
| `ldr` at 0xc7ce8 | load | 4 | `abs 0xc7d10` | `-` |
| `str` at 0xc7cf8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xc7cfc | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xc7d00 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc7d00 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc7d00 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0xc7cc0 | `(unknown callee) (unknown target)` | `(r0 & 0xff)` | `r1` | `(r0 & 0xff)` | `[0x1429c8]` |
| `bl` at 0xc7cf0 | `(unknown callee) 0x21e28` | `0x1` | `0x132f1c` | `0x187` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc7ca8 | `[0x1429c8] == 0x0` | `[0x1429c8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc7d00 | `r0` | `?` |

## `i2cSetBusSelect`

- `i2cSetBusSelect`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc77ac to 0xc77d4.
  Established. A word store copies entry r0 to absolute `0x1429c8` (`str` at 0xc77c8). The only
  other explicit store saves that same entry value at `sp -0x14` (`str` at 0xc77bc). The r0 return
  preserves entry r0 (`ldmdb` at 0xc77cc). No calls or conditional branches are reported, and the
  stored value is not proven to be a callback or device address.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc77bc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc77c0 | load | 4 | `abs 0xc77d0` | `-` |
| `ldr` at 0xc77c4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc77c8 | store | 4 | `abs 0x1429c8` | `r0` |
| `ldmdb` at 0xc77cc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc77cc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc77cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc77cc | `r0` | `r0` |

## `i2cSetBusDeselect`

- `i2cSetBusDeselect`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc77d4 to 0xc77fc.
  Established. Entry r0 is saved at `sp -0x14` and then written as a word to absolute `0x2ad184`
  (`str` at 0xc77e4, `str` at 0xc77f0). It survives as the reported r0 return (`ldmdb` at 0xc77f4).
  The body contains no calls or conditional branches. The deselection role and stored pointer
  interpretation remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc77e4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc77e8 | load | 4 | `abs 0xc77f8` | `-` |
| `ldr` at 0xc77ec | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc77f0 | store | 4 | `abs 0x2ad184` | `r0` |
| `ldmdb` at 0xc77f4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc77f4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc77f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc77f4 | `r0` | `r0` |

## `i2cSetSwitch`

- `i2cSetSwitch`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc79ac to 0xc7a14.
  Established. The final word store writes `(r0 & 0xff)` through `[0x2b21a0] +0x0` (`str` at
  0xc79fc). It follows a loop whose deciding branch retains a less-or-equal flags predicate with
  unknown operands, following its edge when that predicate holds and falling through otherwise (`ble`
  at 0xc79d8). The body returns `0x0` in r0 (`ldmdb` at 0xc7a08). Calls are absent, and the physical
  switch register meaning and delay duration are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xc79c0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0xc79c8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xc79cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc79d0 | load | 4 | `abs 0xc7a0c` | `-` |
| `ldr` at 0xc79e0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc79e8 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xc79f0 | load | 4 | `abs 0xc7a10` | `-` |
| `ldr` at 0xc79f4 | load | 4 | `abs 0x2b21a0` | `-` |
| `ldrb` at 0xc79f8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xc79fc | store | 4 | `[0x2b21a0] +0x0` | `(r0 & 0xff)` |
| `ldmdb` at 0xc7a08 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc7a08 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc7a08 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xc79d8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc7a08 | `r0` | `0x0` |
