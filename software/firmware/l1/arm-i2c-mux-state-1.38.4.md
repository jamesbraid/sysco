# ARM I2C locking and multiplexer state in 1.38.4

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

[Fact sheets and full listings](disasm/arm-1.38.4-package4/i2c-mux-state-facts.txt) retain all tool sections.

## `i2cLock`

- `i2cLock`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc7a50 to 0xc7b90.
  Established. The result of `0x1b280` is written as a word to absolute `0x2ad188` (`bl` at 0xc7ab8,
  `str` at 0xc7ac4). An indirect call has unknown callee and target with candidates `[sp-17]:1`,
  unknown, `[sp-17]:1`, `[0x1429c8]` (`mov` at 0xc7b1c); its zero result takes the arm that stores
  byte `([sp-17]:1 & 0xff)` at `0x2ad18c`, while nonzero falls through to `0xc7b90` (`beq` at
  0xc7b30, `strb` at 0xc7b58, `bl` at 0xc7b34). The physical lock operation and merged r0 return
  remain unknown (`ldmdb` at 0xc7b68).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xc7a64 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xc7a68 | load | 4 | `abs 0xc7b6c` | `-` |
| `str` at 0xc7a74 | store | 4 | `sp -0x18` | `ret(0x1b1f0)` |
| `ldr` at 0xc7a78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc7a8c | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xc7a98 | load | 4 | `abs 0xc7b70` | `-` |
| `str` at 0xc7a9c | store | 4 | `sp -0x24` | `0x133010` |
| `str` at 0xc7aa0 | store | 4 | `sp -0x20` | `ret(0x21674)` |
| `ldr` at 0xc7aa8 | load | 4 | `abs 0xc7b74` | `-` |
| `ldr` at 0xc7aac | load | 4 | `abs 0xc7b78` | `-` |
| `ldr` at 0xc7ac0 | load | 4 | `abs 0xc7b7c` | `-` |
| `str` at 0xc7ac4 | store | 4 | `abs 0x2ad188` | `ret(0x1b280)` |
| `ldrb` at 0xc7ac8 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xc7ad4 | load | 4 | `abs 0xc7b80` | `-` |
| `ldr` at 0xc7ad8 | load | 4 | `abs 0x1429c8` | `-` |
| `ldr` at 0xc7ae4 | load | 4 | `abs 0xc7b84` | `-` |
| `str` at 0xc7ae8 | store | 4 | `sp -0x24` | `0x13302c` |
| `ldrb` at 0xc7aec | load | 1 | `sp -0x11` | `-` |
| `str` at 0xc7af0 | store | 4 | `sp -0x20` | `[sp-17]:1` |
| `ldr` at 0xc7af8 | load | 4 | `abs 0xc7b74` | `-` |
| `ldr` at 0xc7afc | load | 4 | `abs 0xc7b88` | `-` |
| `ldr` at 0xc7b08 | load | 4 | `abs 0xc7b80` | `-` |
| `ldrb` at 0xc7b0c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xc7b10 | load | 4 | `abs 0x1429c8` | `-` |
| `str` at 0xc7b24 | store | 4 | `sp -0x18` | `ret(unknown)` |
| `ldr` at 0xc7b28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc7b40 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `str` at 0xc7b48 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xc7b50 | load | 4 | `abs 0xc7b8c` | `-` |
| `ldrb` at 0xc7b54 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0xc7b58 | store | 1 | `abs 0x2ad18c` | `([sp-17]:1 & 0xff)` |
| `str` at 0xc7b60 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xc7b64 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xc7b68 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc7b68 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc7b68 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc7a6c | `(unknown callee) 0x1b1f0` | `0x2ad16c` | `r1` | `r2` | `r0` |
| `bl` at 0xc7a84 | `(unknown callee) 0x1b0b8` | `ret(0x1b1f0)` | `?` | `?` | `ret(0x1b1f0)` |
| `bl` at 0xc7a90 | `(unknown callee) 0x21674` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0xc7ab4 | `(unknown callee) 0x21e28` | `0x9` | `0x132f1c` | `0x141` | `0x0` |
| `bl` at 0xc7ab8 | `(unknown callee) 0x1b280` | `?` | `?` | `?` | `?` |
| `bl` at 0xc7b04 | `(unknown callee) 0x21e28` | `0x9` | `0x132f1c` | `0x14a` | `0x0` |
| `mov` at 0xc7b1c | `(unknown callee) (unknown target)` | `[sp-17]:1` | `?` | `[sp-17]:1` | `[0x1429c8]` |
| `bl` at 0xc7b34 | `(unknown callee) 0xc7b90` | `ret(unknown)` | `?` | `?` | `ret(unknown)` |
| `bl` at 0xc7b38 | `(unknown callee) 0x1b0b8` | `ret(0xc7b90)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xc7a80 | `ret(0x1b1f0) >= 0x0 (signed, 32-bit)` | `ret(0x1b1f0) < 0x0 (signed, 32-bit)` |
| `beq` at 0xc7ad0 | `[sp-17]:1 == 0x0` | `[sp-17]:1 != 0x0` |
| `bne` at 0xc7ae0 | `[0x1429c8] != 0x0` | `[0x1429c8] == 0x0` |
| `beq` at 0xc7b30 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc7b68 | `r0` | `?` |

## `i2cUnlock`

- `i2cUnlock`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc7b90 to 0xc7c84.
  Established. An absolute word store clears `0x2ad188` (`str` at 0xc7ba8). A zero byte at
  `0x2ad18c` takes the bypass and nonzero falls through to a second test (`beq` at 0xc7bb8).
  That test takes its bypass for `[0x2ad184] == 0x0` and otherwise reaches an unknown call with
  `[0x2ad18c]:1`, entry r1, `[0x2ad184]`, `[0x2ad18c]:1` (`beq` at 0xc7bc8, `mov` at 0xc7be4). A
  later byte store clears `0x2ad18c` (`strb` at 0xc7c0c). The unlock semantics and joined return are
  unproven (`ldmdb` at 0xc7c60).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xc7ba0 | load | 4 | `abs 0xc7c64` | `-` |
| `str` at 0xc7ba8 | store | 4 | `abs 0x2ad188` | `0x0` |
| `ldr` at 0xc7bac | load | 4 | `abs 0xc7c68` | `-` |
| `ldrb` at 0xc7bb0 | load | 1 | `abs 0x2ad18c` | `-` |
| `ldr` at 0xc7bbc | load | 4 | `abs 0xc7c6c` | `-` |
| `ldr` at 0xc7bc0 | load | 4 | `abs 0x2ad184` | `-` |
| `ldr` at 0xc7bcc | load | 4 | `abs 0xc7c6c` | `-` |
| `ldr` at 0xc7bd0 | load | 4 | `abs 0xc7c68` | `-` |
| `ldrb` at 0xc7bd4 | load | 1 | `abs 0x2ad18c` | `-` |
| `ldr` at 0xc7bd8 | load | 4 | `abs 0x2ad184` | `-` |
| `str` at 0xc7bec | store | 4 | `sp -0x14` | `ret(unknown)` |
| `ldr` at 0xc7bf0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc7bfc | load | 4 | `abs 0xc7c70` | `-` |
| `ldr` at 0xc7c04 | load | 4 | `abs 0xc7c68` | `-` |
| `strb` at 0xc7c0c | store | 1 | `abs 0x2ad18c` | `0x0` |
| `ldr` at 0xc7c10 | load | 4 | `abs 0xc7c74` | `-` |
| `str` at 0xc7c1c | store | 4 | `sp -0x14` | `ret(0x1b46c)` |
| `ldr` at 0xc7c20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc7c34 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xc7c40 | load | 4 | `abs 0xc7c78` | `-` |
| `str` at 0xc7c44 | store | 4 | `sp -0x1c` | `0x13305c` |
| `str` at 0xc7c48 | store | 4 | `sp -0x18` | `ret(0x21674)` |
| `ldr` at 0xc7c50 | load | 4 | `abs 0xc7c7c` | `-` |
| `ldr` at 0xc7c54 | load | 4 | `abs 0xc7c80` | `-` |
| `ldmdb` at 0xc7c60 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc7c60 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc7c60 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0xc7be4 | `(unknown callee) (unknown target)` | `[0x2ad18c]:1` | `r1` | `[0x2ad184]` | `[0x2ad18c]:1` |
| `bl` at 0xc7c00 | `(unknown callee) 0x20240` | `0x133044` | `?` | `?` | `ret(unknown)` |
| `bl` at 0xc7c14 | `(unknown callee) 0x1b46c` | `0x2ad16c` | `?` | `?` | `?` |
| `bl` at 0xc7c2c | `(unknown callee) 0x1b0b8` | `ret(0x1b46c)` | `?` | `?` | `ret(0x1b46c)` |
| `bl` at 0xc7c38 | `(unknown callee) 0x21674` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0xc7c5c | `(unknown callee) 0x21e28` | `0x9` | `0x132f1c` | `0x175` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc7bb8 | `[0x2ad18c]:1 == 0x0` | `[0x2ad18c]:1 != 0x0` |
| `beq` at 0xc7bc8 | `[0x2ad184] == 0x0` | `[0x2ad184] != 0x0` |
| `beq` at 0xc7bf8 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bge` at 0xc7c28 | `ret(0x1b46c) >= 0x0 (signed, 32-bit)` | `ret(0x1b46c) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc7c60 | `r0` | `?` |

## `i2cGetSwitch`

- `i2cGetSwitch`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc7a14 to 0xc7a50.
  Established. A byte load addresses `[0x2b21a0] +0x0` (`ldrb` at 0xc7a34). The following byte store
  has entry `r0 +0x0` as its target but an unknown value (`strb` at 0xc7a3c). The reported r0 return
  is `0x0` (`ldmdb` at 0xc7a48). No call or conditional branch is reported, and the physical
  switch-state interpretation is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc7a24 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc7a28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc7a2c | load | 4 | `abs 0xc7a4c` | `-` |
| `ldr` at 0xc7a30 | load | 4 | `abs 0x2b21a0` | `-` |
| `ldrb` at 0xc7a34 | load | 1 | `[0x2b21a0] +0x0` | `-` |
| `strb` at 0xc7a3c | store | 1 | `r0 +0x0` | `?` |
| `ldmdb` at 0xc7a48 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc7a48 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc7a48 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc7a48 | `r0` | `0x0` |

## `i2cBus`

- `i2cBus`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc7784 to 0xc77ac.
  Established. Entry r0 is saved as a word and `(r1 & 0xff)` as a byte at `sp -0x14` and `sp -0x15`,
  respectively (`str` at 0xc7794, `strb` at 0xc779c). The return reports `(r1 & 0xff)` in r0
  (`ldmdb` at 0xc77a8). Calls and conditional branches are absent from the facts. A bus-selection
  effect is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc7794 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xc779c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xc77a0 | load | 1 | `sp -0x15` | `-` |
| `ldmdb` at 0xc77a8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc77a8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc77a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc77a8 | `r0` | `(r1 & 0xff)` |

## `i2cInitialize`

- `i2cInitialize`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x40f20 to 0x40f80.
  Established. The first call to `0xd4a3c` receives entry r0 through r3 (`bl` at 0x40f2c). The next
  call reaches `0x410cc`, whose zero result takes the bypass and nonzero falls through to `0xd6340`
  with `0x0`, unknown, unknown, `ret(0x410cc)` (`bl` at 0x40f30, `beq` at 0x40f3c, `bl` at 0x40f44).
  The final call targets `0x5144c` with `ret(0x41c24)`, unknown, unknown, unknown and supplies r0 at
  return (`bl` at 0x40f78, `ldmdb` at 0x40f7c). The individual hardware-initialization effects
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x40f7c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x40f7c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x40f7c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x40f2c | `(unknown callee) 0xd4a3c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x40f30 | `(unknown callee) 0x410cc` | `ret(0xd4a3c)` | `?` | `?` | `?` |
| `bl` at 0x40f44 | `(unknown callee) 0xd6340` | `0x0` | `?` | `?` | `ret(0x410cc)` |
| `bl` at 0x40f4c | `(unknown callee) 0xd1a50` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x40f54 | `(unknown callee) 0xcc4d8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x40f5c | `(unknown callee) 0x3c640` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x40f64 | `(unknown callee) 0x5c708` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x40f6c | `(unknown callee) 0x456f4` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x40f74 | `(unknown callee) 0x41c24` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x40f78 | `(unknown callee) 0x5144c` | `ret(0x41c24)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x40f3c | `ret(0x410cc) == 0x0` | `ret(0x410cc) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x40f7c | `r0` | `ret(0x5144c)` |

## `i2c_soft_l1_reset_check`

- `i2c_soft_l1_reset_check`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x40f80 to 0x410cc.
  Established. Four alternative word stores at `[sp-20] +0x0` write `0x10898c`, `0x1089a8`,
  `0x1089cc`, and `0x1089fc` (`str` at 0x41010, `str` at 0x4102c, `str` at 0x41060, `str` at
  0x41084). The call to `0x4658c` has `0x0`, `0xff`, unknown, unknown (`bl` at 0x40fc8). A later
  signed test takes the continuing edge for `ret(0x3d970) >= 0x0` and falls through on negative
  (`bge` at 0x41004). The returned r0 value is `[sp-36]`, while reset criteria and stored-object
  meanings remain unknown (`ldmdb` at 0x410b8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x40f90 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x40f98 | store | 4 | `sp -0x18` | `0x0` |
| `strb` at 0x40fb8 | store | 1 | `sp -0x19` | `0x1` |
| `strb` at 0x40fd0 | store | 1 | `sp -0x19` | `(ret(0x4658c) & 0xff)` |
| `str` at 0x40fe0 | store | 4 | `sp -0x20` | `ret(0x3d970)` |
| `ldr` at 0x40fe4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x40ff4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x40ffc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x41008 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4100c | load | 4 | `abs 0x410bc` | `-` |
| `str` at 0x41010 | store | 4 | `[sp-20] +0x0` | `0x10898c` |
| `ldrb` at 0x41018 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x41024 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41028 | load | 4 | `abs 0x410c0` | `-` |
| `str` at 0x4102c | store | 4 | `[sp-20] +0x0` | `0x1089a8` |
| `ldr` at 0x41058 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4105c | load | 4 | `abs 0x410c4` | `-` |
| `str` at 0x41060 | store | 4 | `[sp-20] +0x0` | `0x1089cc` |
| `ldr` at 0x4107c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41080 | load | 4 | `abs 0x410c8` | `-` |
| `str` at 0x41084 | store | 4 | `[sp-20] +0x0` | `0x1089fc` |
| `ldr` at 0x41088 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4108c | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0x4109c | store | 4 | `sp -0x28` | `?` |
| `str` at 0x410a8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x410ac | load | 4 | `sp -0x28` | `-` |
| `str` at 0x410b0 | store | 4 | `sp -0x24` | `[sp-40]` |
| `ldr` at 0x410b4 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x410b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x410b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x410b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x40f9c | `(unknown callee) 0x4f090` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x40fc8 | `(unknown callee) 0x4658c` | `0x0` | `0xff` | `?` | `?` |
| `bl` at 0x40fd8 | `(unknown callee) 0x3d970` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x41034 | `(unknown callee) 0x51560` | `ret(0x3d970)` | `?` | `?` | `[sp-25]:1` |
| `bl` at 0x41048 | `(unknown callee) 0x51cbc` | `0x0` | `?` | `?` | `ret(0x51560)` |
| `bl` at 0x4106c | `(unknown callee) 0x51cbc` | `0x2` | `?` | `?` | `ret(0x51cbc)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x40fac | `? == 0x4f` | `? != 0x4f` |
| `bne` at 0x40fec | `ret(0x3d970) != 0x0` | `ret(0x3d970) == 0x0` |
| `bge` at 0x41004 | `ret(0x3d970) >= 0x0 (signed, 32-bit)` | `ret(0x3d970) < 0x0 (signed, 32-bit)` |
| `bne` at 0x41020 | `[sp-25]:1 != 0x0` | `[sp-25]:1 == 0x0` |
| `beq` at 0x41040 | `ret(0x51560) == 0x0` | `ret(0x51560) != 0x0` |
| `beq` at 0x41054 | `ret(0x51cbc) == 0x1` | `ret(0x51cbc) != 0x1` |
| `bne` at 0x41078 | `ret(0x51cbc) != 0x1` | `ret(0x51cbc) == 0x1` |
| `beq` at 0x41094 | `[[sp-20]] == 0x0` | `[[sp-20]] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x410b8 | `r0` | `[sp-36]` |

## `i2cMuxInit`

- `i2cMuxInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd4590 to 0xd47a8.
  Established. On the zero arm of the test for `[0x2ad218]`, a word store writes `0x1` there (`bne`
  at 0xd4608, `str` at 0xd4614). The call to `0x1b3c4` carries `[sp-44]`, `0x1`, `[0x2ad218]`,
  unknown, with its result stored as a word at absolute `0x2ad214` (`bl` at 0xd4640, `str` at
  0xd4648). Later word stores put unknown, `[sp-32]`, and `ret(0x1b3c4)` at `[sp-40] +0x0`, `+0x4`,
  and `+0x8`, respectively (`str` at 0xd46a8, `str` at 0xd46b4, `str` at 0xd46d0). The final call to
  `0x1b43c` supplies the r0 return (`bl` at 0xd4780, `ldmdb` at 0xd4784). The multiplexer record and
  physical initialization roles are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd45a0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xd45a4 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xd45a8 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0xd45ac | load | 4 | `abs 0xd4788` | `-` |
| `ldr` at 0xd45b0 | load | 4 | `abs 0x2ad214` | `-` |
| `ldr` at 0xd45bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd45c8 | load | 4 | `abs 0xd478c` | `-` |
| `str` at 0xd45cc | store | 4 | `sp -0x38` | `0x1344c8` |
| `ldr` at 0xd45d0 | load | 4 | `abs 0xd4788` | `-` |
| `ldr` at 0xd45d4 | load | 4 | `abs 0x2ad214` | `-` |
| `str` at 0xd45d8 | store | 4 | `sp -0x34` | `[0x2ad214]` |
| `ldr` at 0xd45dc | load | 4 | `abs 0xd4790` | `-` |
| `ldr` at 0xd45e0 | load | 4 | `abs 0x2ad218` | `-` |
| `str` at 0xd45e4 | store | 4 | `sp -0x30` | `[0x2ad218]` |
| `ldr` at 0xd45ec | load | 4 | `abs 0xd4794` | `-` |
| `ldr` at 0xd45fc | load | 4 | `abs 0xd4790` | `-` |
| `ldr` at 0xd4600 | load | 4 | `abs 0x2ad218` | `-` |
| `ldr` at 0xd460c | load | 4 | `abs 0xd4790` | `-` |
| `str` at 0xd4614 | store | 4 | `abs 0x2ad218` | `0x1` |
| `ldr` at 0xd4618 | load | 4 | `abs 0xd4790` | `-` |
| `ldr` at 0xd461c | load | 4 | `abs 0x2ad218` | `-` |
| `str` at 0xd4630 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd4634 | load | 4 | `abs 0xd4788` | `-` |
| `ldr` at 0xd4638 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd4648 | store | 4 | `abs 0x2ad214` | `ret(0x1b3c4)` |
| `ldr` at 0xd464c | load | 4 | `abs 0x2ad214` | `-` |
| `ldr` at 0xd4658 | load | 4 | `abs 0xd4798` | `-` |
| `str` at 0xd465c | store | 4 | `sp -0x38` | `0x134518` |
| `ldr` at 0xd4660 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd4664 | store | 4 | `sp -0x34` | `[sp-44]` |
| `ldr` at 0xd466c | load | 4 | `abs 0xd4794` | `-` |
| `ldr` at 0xd467c | load | 4 | `abs 0xd4788` | `-` |
| `ldr` at 0xd4680 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd4694 | load | 4 | `abs 0x2ad214` | `-` |
| `str` at 0xd469c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xd46a0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd46a4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd46a8 | store | 4 | `[sp-40] +0x0` | `?` |
| `ldr` at 0xd46ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd46b0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd46b4 | store | 4 | `[sp-40] +0x4` | `[sp-32]` |
| `ldr` at 0xd46b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd46bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd46c0 | load | 4 | `[sp-40] +0x4` | `-` |
| `str` at 0xd46d0 | store | 4 | `[sp-40] +0x8` | `ret(0x1b3c4)` |
| `ldr` at 0xd46d4 | load | 4 | `[sp-40] +0x8` | `-` |
| `ldr` at 0xd46e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd46e8 | load | 4 | `abs 0xd479c` | `-` |
| `ldr` at 0xd46ec | load | 4 | `abs 0xd47a0` | `-` |
| `str` at 0xd4718 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd471c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd4724 | load | 4 | `abs 0xd479c` | `-` |
| `ldr` at 0xd4728 | load | 4 | `abs 0xd47a0` | `-` |
| `ldr` at 0xd4734 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd4744 | load | 4 | `abs 0xd47a4` | `-` |
| `str` at 0xd4748 | store | 4 | `(unknown)` | `0x134560` |
| `str` at 0xd474c | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0xd4750 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd4754 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0xd475c | load | 4 | `abs 0xd4794` | `-` |
| `ldr` at 0xd476c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd4770 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd4774 | load | 4 | `[sp-40] +0x8` | `-` |
| `ldr` at 0xd477c | load | 4 | `[sp-40] +0x4` | `-` |
| `ldmdb` at 0xd4784 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd4784 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd4784 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd4784 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd45f8 | `(unknown callee) 0x21e28` | `0x9` | `0x1344fc` | `0x54` | `0x0` |
| `bl` at 0xd4640 | `(unknown callee) 0x1b3c4` | `[sp-44]` | `0x1` | `[0x2ad218]` | `?` |
| `bl` at 0xd4678 | `(unknown callee) 0x21e28` | `0x9` | `0x1344fc` | `0x60` | `0x0` |
| `bl` at 0xd46c8 | `(unknown callee) 0x1b3c4` | `[[sp-40]+4]` | `0x1` | `[sp-40]` | `[sp-40]` |
| `bl` at 0xd46f0 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x134554` | `0x13455c` |
| `bl` at 0xd46fc | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xd472c | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x134554` | `0x13455c` |
| `bl` at 0xd473c | `(unknown callee) 0x1b508` | `[sp-44]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xd4768 | `(unknown callee) 0x21e28` | `0x9` | `0x1344fc` | `0x6b` | `0x0` |
| `bl` at 0xd4780 | `(unknown callee) 0x1b43c` | `[[sp-40]+8]` | `0x1` | `[[sp-40]+4]` | `[sp-40]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd45b8 | `[0x2ad214] == 0x0` | `[0x2ad214] != 0x0` |
| `bne` at 0xd45c4 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xd4608 | `[0x2ad218] != 0x0` | `[0x2ad218] == 0x0` |
| `bne` at 0xd4654 | `[0x2ad214] != 0x0` | `[0x2ad214] == 0x0` |
| `bne` at 0xd46dc | `[[sp-40]+8] != 0x0` | `[[sp-40]+8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd4784 | `r0` | `ret(0x1b43c)` |

## `i2cMuxReset`

- `i2cMuxReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd4a3c to 0xd4aac.
  Established. A call to `0xc7a50` carries `0x0`, entry r1, entry r2, entry r3 (`bl` at 0xd4a50);
  its zero result takes the route through `0xd4e24` and then `0xc7b90`, while nonzero falls through to
  `0x219a8` (`beq` at 0xd4a64, `bl` at 0xd4a98, `bl` at 0xd4a9c, `bl` at 0xd4a6c). The call to
  `0xc7b90` has `ret(0xd4e24)`, unknown, unknown, unknown (`bl` at 0xd4a9c). Physical reset behavior
  and the joined r0 value remain unknown (`ldmdb` at 0xd4aa0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd4a58 | store | 4 | `sp -0x14` | `ret(0xc7a50)` |
| `ldr` at 0xd4a5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd4a68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd4a74 | load | 4 | `abs 0xd4aa4` | `-` |
| `str` at 0xd4a78 | store | 4 | `sp -0x1c` | `0x1345f4` |
| `str` at 0xd4a7c | store | 4 | `sp -0x18` | `ret(0x219a8)` |
| `ldr` at 0xd4a84 | load | 4 | `abs 0xd4aa8` | `-` |
| `ldmdb` at 0xd4aa0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd4aa0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd4aa0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd4a50 | `(unknown callee) 0xc7a50` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0xd4a6c | `(unknown callee) 0x219a8` | `ret(0xc7a50)` | `?` | `?` | `ret(0xc7a50)` |
| `bl` at 0xd4a90 | `(unknown callee) 0x21e28` | `0x1` | `0x1344fc` | `0xa5` | `0x0` |
| `bl` at 0xd4a98 | `(unknown callee) 0xd4e24` | `ret(0xc7a50)` | `?` | `?` | `ret(0xc7a50)` |
| `bl` at 0xd4a9c | `(unknown callee) 0xc7b90` | `ret(0xd4e24)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd4a64 | `ret(0xc7a50) == 0x0` | `ret(0xc7a50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd4aa0 | `r0` | `?` |

## `i2cMuxGetCfg`

- `i2cMuxGetCfg`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd4aac to 0xd4b0c.
  Established. A word store writes `[[sp-28]]` through entry `r1 +0x0` (`str` at 0xd4af4). The next
  word read addresses `[sp-28] +0x4`, and its value reaches the r0 return as `[[sp-28]+4]` (`ldr` at
  0xd4afc, `ldmdb` at 0xd4b04). There are no calls or conditional branches. Configuration-field
  meanings are not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd4abc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd4ac0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xd4ac4 | load | 4 | `abs 0xd4b08` | `-` |
| `ldr` at 0xd4ac8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd4adc | load | 4 | `abs 0x2ad214` | `-` |
| `str` at 0xd4ae4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xd4ae8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd4aec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd4af0 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xd4af4 | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0xd4af8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd4afc | load | 4 | `[sp-28] +0x4` | `-` |
| `ldmdb` at 0xd4b04 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd4b04 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd4b04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd4b04 | `r0` | `[[sp-28]+4]` |

## `i2cMuxCount`

- `i2cMuxCount`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd4b0c to 0xd4b58.
  Established. The absolute word at `0x2ad214` is loaded while constructing an indexed target (`ldr`
  at 0xd4b38). The final word read addresses `[sp-24] +0x4`, and r0 returns `[[sp-24]+4]` (`ldr` at
  0xd4b48, `ldmdb` at 0xd4b50). The facts contain no calls or conditional branches. The count units
  and referenced object remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd4b1c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xd4b20 | load | 4 | `abs 0xd4b54` | `-` |
| `ldr` at 0xd4b24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd4b38 | load | 4 | `abs 0x2ad214` | `-` |
| `str` at 0xd4b40 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xd4b44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd4b48 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldmdb` at 0xd4b50 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd4b50 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd4b50 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd4b50 | `r0` | `[[sp-24]+4]` |

## `i2cMuxIsPresent`

- `i2cMuxIsPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd4b58 to 0xd4bdc.
  Established. The signed bound takes its continuation for `(r1 & 0xff) < [[sp-28]+4]` and falls
  through for greater-or-equal (`blt` at 0xd4ba8). The fall-through arm stores word `0x0` at `sp
  -0x20` (`str` at 0xd4bb0). The continuing arm instead stores word `[((r1 & 0xff) +
  [[sp-28]+8])]:1` there (`str` at 0xd4bcc). The physical presence criterion and joined r0 return
  remain unknown (`ldmdb` at 0xd4bd4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd4b68 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd4b70 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xd4b74 | load | 4 | `abs 0xd4bd8` | `-` |
| `ldr` at 0xd4b78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd4b8c | load | 4 | `abs 0x2ad214` | `-` |
| `str` at 0xd4b94 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xd4b98 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd4b9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd4ba0 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xd4bb0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xd4bb8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd4bbc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd4bc0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0xd4bc8 | load | 1 | `((r1 & 0xff) + [[sp-28]+8]) +0x0` | `-` |
| `str` at 0xd4bcc | store | 4 | `sp -0x20` | `[((r1 & 0xff) + [[sp-28]+8])]:1` |
| `ldr` at 0xd4bd0 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xd4bd4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd4bd4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd4bd4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xd4ba8 | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd4bd4 | `r0` | `?` |

## `i2cMuxLastChannel`

- `i2cMuxLastChannel`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd581c to 0xd583c.
  Established. A byte read addresses absolute `0x142b0d` (`ldrb` at 0xd582c); its value moves to r0
  and is returned as `[0x142b0d]:1` (`mov` at 0xd5830, `ldmdb` at 0xd5834). No explicit stores or
  calls occur. The channel encoding is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xd5828 | load | 4 | `abs 0xd5838` | `-` |
| `ldrb` at 0xd582c | load | 1 | `abs 0x142b0d` | `-` |
| `ldmdb` at 0xd5834 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd5834 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd5834 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd5834 | `r0` | `[0x142b0d]:1` |

## `i2cMuxName`

- `i2cMuxName`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd4dbc to 0xd4e24.
  Established. The only call targets `0xd583c` with entry r0, `(r1 & 0xff)`, entry r2, `(r1 & 0xff)`
  (`bl` at 0xd4de4). The word at `ret(0xd583c) +0x4` is read before a branch that takes the bypass
  for nonzero and falls through on zero (`ldr` at 0xd4e00, `bne` at 0xd4e08). The zero arm saves
  word `0x134614` at `sp -0x20` (`str` at 0xd4e10). The name representation and merged r0 return
  remain unknown (`ldmdb` at 0xd4e1c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd4dcc | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd4dd4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xd4dd8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd4ddc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd4dec | store | 4 | `sp -0x1c` | `ret(0xd583c)` |
| `ldr` at 0xd4df0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd4df4 | load | 4 | `ret(0xd583c) +0x4` | `-` |
| `str` at 0xd4df8 | store | 4 | `sp -0x20` | `[ret(0xd583c)+4]` |
| `ldr` at 0xd4dfc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd4e00 | load | 4 | `ret(0xd583c) +0x4` | `-` |
| `ldr` at 0xd4e0c | load | 4 | `abs 0xd4e20` | `-` |
| `str` at 0xd4e10 | store | 4 | `sp -0x20` | `0x134614` |
| `ldr` at 0xd4e14 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xd4e1c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd4e1c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd4e1c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd4de4 | `(unknown callee) 0xd583c` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd4e08 | `[ret(0xd583c)+4] != 0x0` | `[ret(0xd583c)+4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd4e1c | `r0` | `?` |

## `i2cMuxRead`

- `i2cMuxRead`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd4d10 to 0xd4dbc.
  Established. An indexed byte read has target `([sp-21]:1 + [[sp-32]+8]) +0x0` (`ldrb` at 0xd4d7c);
  its nonzero value takes the call arm and zero falls through to store word `0xfffffc11` at `sp
  -0x28` (`bne` at 0xd4d84, `str` at 0xd4d8c). The call to `0xd5aa8` carries `[sp-20]`,
  `ret(0xd583c)`, `[sp-28]`, `[([sp-21]:1 + [[sp-32]+8])]:1` (`bl` at 0xd4da0). The physical read
  action and joined r0 result are unknown (`ldmdb` at 0xd4db0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd4d20 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd4d28 | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0xd4d2c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xd4d30 | load | 4 | `abs 0xd4db4` | `-` |
| `ldr` at 0xd4d34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd4d48 | load | 4 | `abs 0x2ad214` | `-` |
| `str` at 0xd4d50 | store | 4 | `sp -0x20` | `?` |
| `ldrb` at 0xd4d54 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd4d58 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd4d68 | store | 4 | `sp -0x24` | `ret(0xd583c)` |
| `ldr` at 0xd4d6c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd4d70 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd4d74 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldrb` at 0xd4d7c | load | 1 | `([sp-21]:1 + [[sp-32]+8]) +0x0` | `-` |
| `ldr` at 0xd4d88 | load | 4 | `abs 0xd4db8` | `-` |
| `str` at 0xd4d8c | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0xd4d94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd4d98 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd4d9c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd4da8 | store | 4 | `sp -0x28` | `ret(0xd5aa8)` |
| `ldr` at 0xd4dac | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xd4db0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd4db0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd4db0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd4d60 | `(unknown callee) 0xd583c` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0xd4da0 | `(unknown callee) 0xd5aa8` | `[sp-20]` | `ret(0xd583c)` | `[sp-28]` | `[([sp-21]:1 + [[sp-32]+8])]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd4d84 | `[([sp-21]:1 + [[sp-32]+8])]:1 != 0x0` | `[([sp-21]:1 + [[sp-32]+8])]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd4db0 | `r0` | `?` |

## `i2cMuxSetSwitch`

- `i2cMuxSetSwitch`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd5760 to 0xd581c.
  Established. The unsigned guard takes its continuation for `(r0 & 0xff) <= 0x9` and falls through
  above that bound (`bls` at 0xd5780). The continuation clears bytes `0x2ad210` and `0x2ad211`,
  while four other byte stores write unknown values at `0x142b0c`, `0x142b0d`, `0x142b0e`,
  `0x142b0f` (`strb` at 0xd5798, `strb` at 0xd57bc, `strb` at 0xd57a4, `strb` at 0xd57b0, `strb` at
  0xd57c8, `strb` at 0xd57d4); its call to `0xc79ac` has `[[sp-17]:1+1262782]:1`, entry r1,
  `0x1344be`, `[[sp-17]:1+1262782]:1` (`bl` at 0xd57e8). Switch encoding and the merged r0 return
  remain unproven (`ldmdb` at 0xd57f8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xd5774 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xd5778 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xd5784 | load | 4 | `abs 0xd57fc` | `-` |
| `str` at 0xd5788 | store | 4 | `sp -0x18` | `0xfffff573` |
| `ldr` at 0xd5790 | load | 4 | `abs 0xd5800` | `-` |
| `strb` at 0xd5798 | store | 1 | `abs 0x2ad210` | `0x0` |
| `ldr` at 0xd579c | load | 4 | `abs 0xd5804` | `-` |
| `strb` at 0xd57a4 | store | 1 | `abs 0x142b0c` | `?` |
| `ldr` at 0xd57a8 | load | 4 | `abs 0xd5808` | `-` |
| `strb` at 0xd57b0 | store | 1 | `abs 0x142b0d` | `?` |
| `ldr` at 0xd57b4 | load | 4 | `abs 0xd580c` | `-` |
| `strb` at 0xd57bc | store | 1 | `abs 0x2ad211` | `0x0` |
| `ldr` at 0xd57c0 | load | 4 | `abs 0xd5810` | `-` |
| `strb` at 0xd57c8 | store | 1 | `abs 0x142b0e` | `?` |
| `ldr` at 0xd57cc | load | 4 | `abs 0xd5814` | `-` |
| `strb` at 0xd57d4 | store | 1 | `abs 0x142b0f` | `?` |
| `ldr` at 0xd57d8 | load | 4 | `abs 0xd5818` | `-` |
| `ldrb` at 0xd57dc | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xd57e0 | load | 1 | `[sp-17]:1 +0x1344be` | `-` |
| `str` at 0xd57f0 | store | 4 | `sp -0x18` | `ret(0xc79ac)` |
| `ldr` at 0xd57f4 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xd57f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd57f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd57f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd57e8 | `(unknown callee) 0xc79ac` | `[[sp-17]:1+1262782]:1` | `r1` | `0x1344be` | `[[sp-17]:1+1262782]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xd5780 | `(r0 & 0xff) <= 0x9 (unsigned, 32-bit)` | `(r0 & 0xff) > 0x9 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd57f8 | `r0` | `?` |
