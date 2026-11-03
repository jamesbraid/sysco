# ARM character I/O and node control in 1.38.4

Read from `6.5.28/usr/cpu/firmware/sysco/l1.bin`, package `6.5.28:l1#4` (version 1.38.4),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53cfa0`.
Carrier SHA-256: `e5b65e5f28d7fba6013a8618f55c34bed7f60f7c6f46d7e9c5b8b7b2eabbe7b3`.
Package SHA-256: `c6693d19c6eb8c3fcf00cbb84d6974d1b5b17b930fb5375c1df2df20894edec9`.
Inner executable SHA-256: `f652c04c77cd7c53ca7ae6d6d6f581f11200efa211ff089bfba0d8bd4f718eb8`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1254136 bytes.
Segment SHA-256: `a373bb774d119f083e52d394fbd2701ebfbf35210301a40dcdb4ebdc39bf6bdc`. Analysed routine spans use the exact
ELF function symbols and sizes, with exclusive end addresses. They can include trailing literal
words after a return.

`sp` denotes the entry stack pointer. Register expressions such as `r0` denote entry
values, and brackets denote loaded values. Decimal numbers inside expressions retain the
exact tool notation. Call candidates use the assumed ARM AAPCS
register locations r0 through r3 with unknown arity. Stack stores appear in the access tables.
Unknown values and predicates include unsupported instruction effects and merged paths.
Numeric call targets have unknown callee names in the flat-image analysis. Device roles
and pointee meanings remain unknown unless the body establishes them. Tables include all
explicit memory operands on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
Accesses and return values cover known control-flow edges. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/character-io-node-control-facts.txt) retain all tool sections.

## `charioInit`

- `charioInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x23b48 to 0x23bc0.
  Established. A word read tests absolute `0x145584`, with zero following the branch and nonzero
  falling through (`ldr` at 0x23b60, `beq` at 0x23b68). The subsequent call to `0x22920` has
  candidates `0x145530`, `0xfe9b4`, `0x1` and `0x9` (`bl` at 0x23b98). A word store then writes
  `[sp-20]` at `0x145584` (`str` at 0x23ba4). The resource represented by that global remains
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x23b58 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x23b5c | load | 4 | `abs 0x23bac` | `-` |
| `ldr` at 0x23b60 | load | 4 | `abs 0x145584` | `-` |
| `ldr` at 0x23b6c | load | 4 | `abs 0x23bb0` | `-` |
| `str` at 0x23b70 | store | 4 | `sp -0x1c` | `0xfe974` |
| `ldr` at 0x23b78 | load | 4 | `abs 0x23bb4` | `-` |
| `ldr` at 0x23b88 | load | 4 | `abs 0x23bb8` | `-` |
| `ldr` at 0x23b8c | load | 4 | `abs 0x23bbc` | `-` |
| `ldr` at 0x23b9c | load | 4 | `abs 0x23bac` | `-` |
| `ldr` at 0x23ba0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x23ba4 | store | 4 | `abs 0x145584` | `[sp-20]` |
| `ldmdb` at 0x23ba8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x23ba8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x23ba8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x23b84 | `(unknown callee) 0x21e28` | `0x9` | `0xfe998` | `0x73` | `0x0` |
| `bl` at 0x23b98 | `(unknown callee) 0x22920` | `0x145530` | `0xfe9b4` | `0x1` | `0x9` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x23b68 | `[0x145584] == 0x0` | `[0x145584] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x23ba8 | `r0` | `ret(0x22920)` |

## `charioOpen`

- `charioOpen`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x23bc0 to 0x23d74.
  Established. Local word stores initialise `sp -0x1c` to `0x1` and `sp -0x24` to `0x0` (`str` at
  0x23bd8, `str` at 0x23be0). The first branch takes a zero result from `0x22a38` and falls through
  on nonzero (`beq` at 0x23bf8). Later paths write `ret(0x1b124)` and `ret(0xc9b90)` to absolute
  `0x145588` (`str` at 0x23c80, `str` at 0x23cc0). The exit carries `[0x145588]`
  (`ldmdb` at 0x23d50). The global's port meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x23bd0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x23bd8 | store | 4 | `sp -0x1c` | `0x1` |
| `str` at 0x23be0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x23be4 | load | 4 | `abs 0x23d54` | `-` |
| `ldr` at 0x23bfc | load | 4 | `abs 0x23d58` | `-` |
| `ldr` at 0x23c00 | load | 4 | `abs 0x1452d8` | `-` |
| `str` at 0x23c10 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x23c14 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x23c18 | store | 4 | `sp -0x20` | `[sp-36]` |
| `ldr` at 0x23c24 | load | 4 | `abs 0x23d5c` | `-` |
| `str` at 0x23c28 | store | 4 | `sp -0x30` | `0xfe9c0` |
| `ldr` at 0x23c2c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x23c30 | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0x23c38 | load | 4 | `abs 0x23d60` | `-` |
| `ldr` at 0x23c48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x23c54 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x23c5c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x23c60 | load | 4 | `abs 0x23d64` | `-` |
| `ldr` at 0x23c64 | load | 4 | `abs 0x145584` | `-` |
| `ldr` at 0x23c70 | load | 4 | `abs 0x23d68` | `-` |
| `str` at 0x23c80 | store | 4 | `abs 0x145588` | `ret(0x1b124)` |
| `ldr` at 0x23c84 | load | 4 | `abs 0x23d68` | `-` |
| `ldr` at 0x23c88 | load | 4 | `abs 0x145588` | `-` |
| `ldr` at 0x23c98 | load | 4 | `abs 0x23d6c` | `-` |
| `str` at 0x23ca0 | store | 4 | `abs 0x14558c` | `0x1` |
| `ldr` at 0x23ca8 | load | 4 | `abs 0x23d64` | `-` |
| `ldr` at 0x23cac | load | 4 | `abs 0x145584` | `-` |
| `ldr` at 0x23cb0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x23cbc | load | 4 | `abs 0x23d68` | `-` |
| `str` at 0x23cc0 | store | 4 | `abs 0x145588` | `ret(0xc9b90)` |
| `ldr` at 0x23cc4 | load | 4 | `abs 0x145588` | `-` |
| `ldr` at 0x23cd0 | load | 4 | `abs 0x23d6c` | `-` |
| `str` at 0x23cd8 | store | 4 | `abs 0x14558c` | `0x1` |
| `str` at 0x23ce0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x23ce4 | load | 4 | `abs 0x23d54` | `-` |
| `ldr` at 0x23cf8 | load | 4 | `abs 0x23d58` | `-` |
| `ldr` at 0x23cfc | load | 4 | `abs 0x1452d8` | `-` |
| `str` at 0x23d0c | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x23d10 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x23d14 | store | 4 | `sp -0x20` | `[sp-40]` |
| `ldr` at 0x23d20 | load | 4 | `abs 0x23d70` | `-` |
| `str` at 0x23d24 | store | 4 | `sp -0x30` | `0xfe9e4` |
| `ldr` at 0x23d28 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x23d2c | store | 4 | `sp -0x2c` | `[sp-40]` |
| `ldr` at 0x23d34 | load | 4 | `abs 0x23d60` | `-` |
| `ldr` at 0x23d44 | load | 4 | `abs 0x23d68` | `-` |
| `ldr` at 0x23d48 | load | 4 | `abs 0x145588` | `-` |
| `ldmdb` at 0x23d50 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x23d50 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x23d50 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x23d50 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x23bec | `(unknown callee) 0x22a38` | `0x145530` | `?` | `r2` | `0x0` |
| `bl` at 0x23c44 | `(unknown callee) 0x21e28` | `0x9` | `0xfe998` | `0x93` | `0x0` |
| `bl` at 0x23c78 | `(unknown callee) 0x1b124` | `0x108` | `?` | `?` | `[0x145584]` |
| `bl` at 0x23c94 | `(unknown callee) 0x1b43c` | `[0x145588]` | `0x0` | `0x108` | `0x145588` |
| `bl` at 0x23cb4 | `(unknown callee) 0xc9b90` | `[0x145584]` | `[sp-28]` | `?` | `0x145584` |
| `bl` at 0x23ce8 | `(unknown callee) 0x22dc0` | `0x145530` | `?` | `?` | `0x0` |
| `bl` at 0x23d40 | `(unknown callee) 0x21e28` | `0x9` | `0xfe998` | `0xab` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x23bf8 | `ret(0x22a38) == 0x0` | `ret(0x22a38) != 0x0` |
| `beq` at 0x23c08 | `[0x1452d8] == 0x1` | `[0x1452d8] != 0x1` |
| `beq` at 0x23c20 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0x23c50 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `bne` at 0x23c6c | `[0x145584] != 0x0` | `[0x145584] == 0x0` |
| `beq` at 0x23ccc | `[0x145588] == 0x0` | `[0x145588] != 0x0` |
| `beq` at 0x23cf4 | `ret(0x22dc0) == 0x0` | `ret(0x22dc0) != 0x0` |
| `beq` at 0x23d04 | `[0x1452d8] == 0x1` | `[0x1452d8] != 0x1` |
| `beq` at 0x23d1c | `[sp-40] == 0x0` | `[sp-40] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x23d50 | `r0` | `[0x145588]` |

## `charioClose`

- `charioClose`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x23d94 to 0x23ec4.
  Established. A zero `ret(0x22a38)` takes the opening branch and nonzero falls through (`beq` at
  0x23dc8). A later decision takes `[0x145584] != 0x0` and falls through on zero (`bne` at 0x23dfc).
  The fall-through path includes word clears of `0x145588` and `0x14558c` (`str` at 0x23e14, `str`
  at 0x23e20). The external close operation and its ownership rules remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x23da8 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x23dac | load | 4 | `abs 0x23ea8` | `-` |
| `str` at 0x23dbc | store | 4 | `sp -0x18` | `ret(0x22a38)` |
| `ldr` at 0x23dc0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x23dcc | load | 4 | `abs 0x23eac` | `-` |
| `str` at 0x23dd0 | store | 4 | `sp -0x20` | `0xfe9c0` |
| `ldr` at 0x23dd4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x23dd8 | store | 4 | `sp -0x1c` | `ret(0x22a38)` |
| `ldr` at 0x23de0 | load | 4 | `abs 0x23eb0` | `-` |
| `ldr` at 0x23df0 | load | 4 | `abs 0x23eb4` | `-` |
| `ldr` at 0x23df4 | load | 4 | `abs 0x145584` | `-` |
| `ldr` at 0x23e00 | load | 4 | `abs 0x23eb8` | `-` |
| `ldr` at 0x23e04 | load | 4 | `abs 0x145588` | `-` |
| `ldr` at 0x23e0c | load | 4 | `abs 0x23eb8` | `-` |
| `str` at 0x23e14 | store | 4 | `abs 0x145588` | `0x0` |
| `ldr` at 0x23e18 | load | 4 | `abs 0x23ebc` | `-` |
| `str` at 0x23e20 | store | 4 | `abs 0x14558c` | `0x0` |
| `ldr` at 0x23e28 | load | 4 | `abs 0x23ebc` | `-` |
| `ldr` at 0x23e2c | load | 4 | `abs 0x14558c` | `-` |
| `ldr` at 0x23e38 | load | 4 | `abs 0x23eb8` | `-` |
| `ldr` at 0x23e3c | load | 4 | `abs 0x145588` | `-` |
| `str` at 0x23e4c | store | 4 | `sp -0x14` | `ret(0xca008)` |
| `ldr` at 0x23e50 | load | 4 | `abs 0x23ebc` | `-` |
| `str` at 0x23e58 | store | 4 | `abs 0x14558c` | `0x0` |
| `ldr` at 0x23e5c | load | 4 | `abs 0x23ea8` | `-` |
| `str` at 0x23e68 | store | 4 | `sp -0x18` | `ret(0x22dc0)` |
| `ldr` at 0x23e6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x23e78 | load | 4 | `abs 0x23ec0` | `-` |
| `str` at 0x23e7c | store | 4 | `sp -0x20` | `0xfe9e4` |
| `ldr` at 0x23e80 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x23e84 | store | 4 | `sp -0x1c` | `ret(0x22dc0)` |
| `ldr` at 0x23e8c | load | 4 | `abs 0x23eb0` | `-` |
| `ldr` at 0x23e9c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x23ea4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x23ea4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x23ea4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x23db4 | `(unknown callee) 0x22a38` | `0x145530` | `?` | `r2` | `0x0` |
| `bl` at 0x23dec | `(unknown callee) 0x21e28` | `0x9` | `0xfe998` | `0xce` | `0x0` |
| `bl` at 0x23e08 | `(unknown callee) 0x1b400` | `[0x145588]` | `?` | `?` | `0x145588` |
| `bl` at 0x23e44 | `(unknown callee) 0xca008` | `[0x145588]` | `0x1` | `?` | `0x145588` |
| `bl` at 0x23e60 | `(unknown callee) 0x22dc0` | `0x145530` | `?` | `?` | `?` |
| `bl` at 0x23e98 | `(unknown callee) 0x21e28` | `0x9` | `0xfe998` | `0xe5` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x23dc8 | `ret(0x22a38) == 0x0` | `ret(0x22a38) != 0x0` |
| `bne` at 0x23dfc | `[0x145584] != 0x0` | `[0x145584] == 0x0` |
| `beq` at 0x23e34 | `[0x14558c] == 0x0` | `[0x14558c] != 0x0` |
| `beq` at 0x23e74 | `ret(0x22dc0) == 0x0` | `ret(0x22dc0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x23ea4 | `r0` | `[sp-20]` |

## `charioLock`

- `charioLock`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x23ec4 to 0x23f10.
  Established. The call to `0x24248` has entry r0 through r3 as candidates (`bl` at 0x23edc); its zero result
  takes the branch, while nonzero falls through (`beq` at 0x23ee8). The fall-through store clears
  `sp -0x18`, whereas the taken arm writes `[0x145588]` there (`str` at 0x23ef0, `str` at 0x23f00).
  Lock semantics cannot be established from these local effects and remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x23ed4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x23ed8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x23ef0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x23ef8 | load | 4 | `abs 0x23f0c` | `-` |
| `ldr` at 0x23efc | load | 4 | `abs 0x145588` | `-` |
| `str` at 0x23f00 | store | 4 | `sp -0x18` | `[0x145588]` |
| `ldr` at 0x23f04 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x23f08 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x23f08 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x23f08 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x23edc | `(unknown callee) 0x24248` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x23ee8 | `ret(0x24248) == 0x0` | `ret(0x24248) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x23f08 | `r0` | `[sp-24]` |

## `charioUnlock`

- `charioUnlock`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x23f10 to 0x23f2c.
  Established. The sole call targets `0x24368` with entry r0, r1, r2 and r3 as candidates (`bl` at
  0x23f1c). The exit preserves its result as `ret(0x24368)` (`ldmdb` at 0x23f28). Whether this
  releases an external resource remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x23f28 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x23f28 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x23f28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x23f1c | `(unknown callee) 0x24368` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x23f28 | `r0` | `ret(0x24368)` |

## `chario_redirect`

- `chario_redirect`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x244b8 to 0x24500.
  Established. Zero entry r0 takes the branch and nonzero falls through (`beq` at 0x244d4). The
  nonzero arm writes entry r0 to absolute `0x1423e0`, and the zero arm writes `0xca3c0` to the same
  address (`str` at 0x244e0, `str` at 0x244f0). The recorded exit still carries entry r0 (`ldmdb` at
  0x244f4). The destination's use for redirection remains unknown here.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x244c8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x244cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x244d8 | load | 4 | `abs 0x244f8` | `-` |
| `ldr` at 0x244dc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x244e0 | store | 4 | `abs 0x1423e0` | `r0` |
| `ldr` at 0x244e8 | load | 4 | `abs 0x244f8` | `-` |
| `ldr` at 0x244ec | load | 4 | `abs 0x244fc` | `-` |
| `str` at 0x244f0 | store | 4 | `abs 0x1423e0` | `0xca3c0` |
| `ldmdb` at 0x244f4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x244f4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x244f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x244d4 | `r0 == 0x0` | `r0 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x244f4 | `r0` | `r0` |

## `nodeCount`

- `nodeCount`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5dea8 to 0x5dec8.
  Established. One word load reads absolute `0x2ae5b4` (`ldr` at 0x5deb8), and the recorded exit
  carries `[0x2ae5b4]` (`ldmdb` at 0x5dec0). The interpretation of that word as a node count remains
  unknown in this body.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x5deb4 | load | 4 | `abs 0x5dec4` | `-` |
| `ldr` at 0x5deb8 | load | 4 | `abs 0x2ae5b4` | `-` |
| `ldmdb` at 0x5dec0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5dec0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5dec0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5dec0 | `r0` | `[0x2ae5b4]` |

## `nodePresent`

- `nodePresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5dec8 to 0x5df4c.
  Established. The opening byte store saves `(r0 & 0xff)` at `sp -0x11` (`strb` at 0x5dedc). A word
  load reads `0x2ae5b8` (`ldr` at 0x5dee4). A zero `[0x2ae5b8]` takes the branch and nonzero falls through
  (`beq` at 0x5deec). The indirect call candidates include `(r0 & 0xff)`, entry r1, `(r0 & 0xff)` and
  `[0x2ae5b8]` (`mov` at 0x5df04); its target and the meaning of presence remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5dedc | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x5dee0 | load | 4 | `abs 0x5df48` | `-` |
| `ldr` at 0x5dee4 | load | 4 | `abs 0x2ae5b8` | `-` |
| `ldr` at 0x5def0 | load | 4 | `abs 0x5df48` | `-` |
| `ldrb` at 0x5def4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x5def8 | load | 4 | `abs 0x2ae5b8` | `-` |
| `str` at 0x5df0c | store | 4 | `sp -0x18` | `ret(unknown)` |
| `ldrb` at 0x5df14 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5df18 | store | 4 | `sp -0x1c` | `(r0 & 0xff)` |
| `ldr` at 0x5df1c | load | 4 | `abs 0x5df48` | `-` |
| `ldr` at 0x5df20 | load | 4 | `abs 0x2ae5b4` | `-` |
| `ldr` at 0x5df24 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5df34 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x5df38 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5df3c | store | 4 | `sp -0x18` | `[sp-28]` |
| `ldr` at 0x5df40 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5df44 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5df44 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5df44 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x5df04 | `(unknown callee) (unknown target)` | `(r0 & 0xff)` | `r1` | `(r0 & 0xff)` | `[0x2ae5b8]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5deec | `[0x2ae5b8] == 0x0` | `[0x2ae5b8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5df44 | `r0` | `?` |

## `nodeDisable`

- `nodeDisable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5dfe8 to 0x5e0dc.
  Established. The first result test takes `ret(0x3d8e0) == 0x0` and falls through on nonzero (`beq`
  at 0x5e014). The later selector takes `[sp-17]:1 != 0xff` and falls through on equality (`bne` at
  0x5e02c). A word store records `0xfffff5d6` at `sp -0x24` (`str` at 0x5e01c). The physical disable
  effect and final merged r0 value are unknown (`ldmdb` at 0x5e0d0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5dffc | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x5e004 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x5e018 | load | 4 | `abs 0x5e0d4` | `-` |
| `str` at 0x5e01c | store | 4 | `sp -0x24` | `0xfffff5d6` |
| `ldrb` at 0x5e024 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x5e034 | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0x5e038 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x5e03c | load | 4 | `abs 0x5e0d8` | `-` |
| `ldr` at 0x5e040 | load | 4 | `abs 0x2ae5b4` | `-` |
| `ldrb` at 0x5e050 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x5e068 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x5e078 | store | 4 | `sp -0x1c` | `ret(0x5e334)` |
| `ldr` at 0x5e07c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5e088 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5e094 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5e098 | store | 4 | `sp -0x18` | `ret(0x5e334)` |
| `ldrb` at 0x5e09c | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x5e0a4 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldr` at 0x5e0ac | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5e0b0 | store | 4 | `sp -0x24` | `[sp-24]` |
| `ldrb` at 0x5e0b8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5e0c8 | store | 4 | `sp -0x24` | `ret(0x5e334)` |
| `ldr` at 0x5e0cc | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x5e0d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5e0d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5e0d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5e008 | `(unknown callee) 0x3d8e0` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x5e058 | `(unknown callee) 0x5dec8` | `?` | `?` | `[0x2ae5b4]` | `?` |
| `bl` at 0x5e070 | `(unknown callee) 0x5e334` | `[sp-29]:1` | `?` | `?` | `[sp-29]:1` |
| `bl` at 0x5e0c0 | `(unknown callee) 0x5e334` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5e014 | `ret(0x3d8e0) == 0x0` | `ret(0x3d8e0) != 0x0` |
| `bne` at 0x5e02c | `[sp-17]:1 != 0xff` | `[sp-17]:1 == 0xff` |
| `blt` at 0x5e048 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x5e064 | `ret(0x5dec8) == 0x0` | `ret(0x5dec8) != 0x0` |
| `beq` at 0x5e084 | `ret(0x5e334) == 0x0` | `ret(0x5e334) != 0x0` |
| `bne` at 0x5e090 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5e0d0 | `r0` | `?` |

## `nodeEnable`

- `nodeEnable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5e0dc to 0x5e1d0.
  Established. A byte store saves `(r0 & 0xff)` at `sp -0x11` (`strb` at 0x5e0f0). A word store
  subsequently places `0xfffff5d6` at `sp -0x24` (`str` at 0x5e110). The selector takes `[sp-17]:1
  != 0xff` and falls through on equality (`bne` at 0x5e120). The later word save contains
  `ret(0x5e404)` at `sp -0x1c` (`str` at 0x5e16c). The enable mechanism and its polarity remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5e0f0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x5e0f8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x5e10c | load | 4 | `abs 0x5e1c8` | `-` |
| `str` at 0x5e110 | store | 4 | `sp -0x24` | `0xfffff5d6` |
| `ldrb` at 0x5e118 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x5e128 | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0x5e12c | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x5e130 | load | 4 | `abs 0x5e1cc` | `-` |
| `ldr` at 0x5e134 | load | 4 | `abs 0x2ae5b4` | `-` |
| `ldrb` at 0x5e144 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x5e15c | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x5e16c | store | 4 | `sp -0x1c` | `ret(0x5e404)` |
| `ldr` at 0x5e170 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5e17c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5e188 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5e18c | store | 4 | `sp -0x18` | `ret(0x5e404)` |
| `ldrb` at 0x5e190 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x5e198 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldr` at 0x5e1a0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5e1a4 | store | 4 | `sp -0x24` | `[sp-24]` |
| `ldrb` at 0x5e1ac | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5e1bc | store | 4 | `sp -0x24` | `ret(0x5e404)` |
| `ldr` at 0x5e1c0 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x5e1c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5e1c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5e1c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5e0fc | `(unknown callee) 0x3d8e0` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x5e14c | `(unknown callee) 0x5dec8` | `?` | `?` | `[0x2ae5b4]` | `?` |
| `bl` at 0x5e164 | `(unknown callee) 0x5e404` | `[sp-29]:1` | `?` | `?` | `[sp-29]:1` |
| `bl` at 0x5e1b4 | `(unknown callee) 0x5e404` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5e108 | `ret(0x3d8e0) == 0x0` | `ret(0x3d8e0) != 0x0` |
| `bne` at 0x5e120 | `[sp-17]:1 != 0xff` | `[sp-17]:1 == 0xff` |
| `blt` at 0x5e13c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x5e158 | `ret(0x5dec8) == 0x0` | `ret(0x5dec8) != 0x0` |
| `beq` at 0x5e178 | `ret(0x5e404) == 0x0` | `ret(0x5e404) != 0x0` |
| `bne` at 0x5e184 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5e1c4 | `r0` | `?` |

## `nodeOp`

- `nodeOp`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5e1d0 to 0x5e334.
  Established. The initial selector takes `(r1 & 0xff) != 0xff` and falls through on equality (`bne`
  at 0x5e1fc). The fall-through path includes a byte clear at `sp -0x15` (`strb` at 0x5e204). One
  indirect call has candidates `[sp-21]:1`, unknown, `[sp-20]` and `[sp-21]:1` (`mov` at 0x5e248);
  its target and the node operation it performs remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5e1e0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x5e1e8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `str` at 0x5e1f0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldrb` at 0x5e1f4 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x5e204 | store | 1 | `sp -0x15` | `0x0` |
| `ldrb` at 0x5e208 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5e20c | load | 4 | `abs 0x5e324` | `-` |
| `ldr` at 0x5e210 | load | 4 | `abs 0x2ae5b4` | `-` |
| `ldrb` at 0x5e220 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x5e238 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5e23c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5e250 | store | 4 | `sp -0x20` | `ret(unknown)` |
| `ldr` at 0x5e254 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5e260 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5e26c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5e270 | store | 4 | `sp -0x1c` | `ret(unknown)` |
| `ldrb` at 0x5e274 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x5e27c | store | 1 | `sp -0x15` | `([sp-21]:1+1 & 0xff)` |
| `ldr` at 0x5e284 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5e288 | store | 4 | `sp -0x24` | `?` |
| `ldrb` at 0x5e290 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5e294 | load | 4 | `abs 0x5e324` | `-` |
| `ldr` at 0x5e298 | load | 4 | `abs 0x2ae5b4` | `-` |
| `ldr` at 0x5e2a4 | load | 4 | `abs 0x5e328` | `-` |
| `str` at 0x5e2a8 | store | 4 | `sp -0x24` | `0xfffff5d7` |
| `ldrb` at 0x5e2b0 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x5e2c8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5e2cc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5e2e0 | store | 4 | `sp -0x24` | `ret(unknown)` |
| `ldrb` at 0x5e2e8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5e300 | load | 4 | `abs 0x5e32c` | `-` |
| `str` at 0x5e304 | store | 4 | `sp -0x28` | `0xfffff5d5` |
| `ldr` at 0x5e30c | load | 4 | `abs 0x5e330` | `-` |
| `str` at 0x5e310 | store | 4 | `sp -0x28` | `0xfffff5d4` |
| `ldr` at 0x5e314 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x5e318 | store | 4 | `sp -0x24` | `[sp-40]` |
| `ldr` at 0x5e31c | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x5e320 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5e320 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5e320 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5e228 | `(unknown callee) 0x5df4c` | `?` | `?` | `[0x2ae5b4]` | `?` |
| `mov` at 0x5e248 | `(unknown callee) (unknown target)` | `[sp-21]:1` | `?` | `[sp-20]` | `[sp-21]:1` |
| `bl` at 0x5e2b8 | `(unknown callee) 0x5df4c` | `(r1 & 0xff)` | `r1` | `[0x2ae5b4]` | `(r1 & 0xff)` |
| `mov` at 0x5e2d8 | `(unknown callee) (unknown target)` | `[sp-21]:1` | `?` | `[sp-20]` | `[sp-21]:1` |
| `bl` at 0x5e2f0 | `(unknown callee) 0x5dec8` | `[sp-21]:1` | `?` | `?` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5e1fc | `(r1 & 0xff) != 0xff` | `(r1 & 0xff) == 0xff` |
| `blt` at 0x5e218 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x5e234 | `ret(0x5df4c) == 0x0` | `ret(0x5df4c) != 0x0` |
| `beq` at 0x5e25c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x5e268 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `blt` at 0x5e2a0 | `(r1 & 0xff) < [0x2ae5b4] (signed, 32-bit)` | `(r1 & 0xff) >= [0x2ae5b4] (signed, 32-bit)` |
| `beq` at 0x5e2c4 | `ret(0x5df4c) == 0x0` | `ret(0x5df4c) != 0x0` |
| `beq` at 0x5e2fc | `ret(0x5dec8) == 0x0` | `ret(0x5dec8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5e320 | `r0` | `?` |

## `IP57_NodeRemove`

- `IP57_NodeRemove`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5f708 to 0x5f81c.
  Established. The opening call to `0x60f28` has candidates `(r0 & 0xff)`, `sp-24`, `sp-24` and
  `(r0 & 0xff)` (`bl` at 0x5f730). Zero takes the branch and nonzero falls through (`beq` at 0x5f744). A
  later call to `0x5f978` has `[sp-17]:1`, `0x1`, `0x1` and `[sp-17]:1` as candidates (`bl` at
  0x5f764). The later byte store writes `0xf` at `[sp-17]:1 +0x2b08e0` (`strb` at 0x5f804). Removal
  semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5f71c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x5f720 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5f738 | store | 4 | `sp -0x1c` | `ret(0x60f28)` |
| `ldr` at 0x5f73c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5f748 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5f74c | store | 4 | `sp -0x24` | `ret(0x60f28)` |
| `ldrb` at 0x5f754 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x5f768 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5f76c | load | 1 | `[sp-24] +0x3` | `-` |
| `strb` at 0x5f770 | store | 1 | `sp -0x1d` | `([[sp-24]+3]:1 & 0xff)` |
| `ldrb` at 0x5f774 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x5f778 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5f77c | load | 1 | `[sp-24] +0x3` | `-` |
| `ldrb` at 0x5f790 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x5f7a0 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x5f7a8 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldr` at 0x5f7b0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5f7b4 | load | 1 | `[sp-24] +0x0` | `-` |
| `strb` at 0x5f7b8 | store | 1 | `sp -0x1d` | `([[sp-24]]:1 & 0xff)` |
| `ldrb` at 0x5f7bc | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x5f7c0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5f7c4 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldrb` at 0x5f7d8 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x5f7e8 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x5f7f0 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldr` at 0x5f7f8 | load | 4 | `abs 0x5f818` | `-` |
| `ldrb` at 0x5f7fc | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x5f804 | store | 1 | `[sp-17]:1 +0x2b08e0` | `0xf` |
| `str` at 0x5f80c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x5f810 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x5f814 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5f814 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5f814 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5f730 | `(unknown callee) 0x60f28` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x5f764 | `(unknown callee) 0x5f978` | `[sp-17]:1` | `0x1` | `0x1` | `[sp-17]:1` |
| `bl` at 0x5f79c | `(unknown callee) 0xd1820` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x5f7e4 | `(unknown callee) 0xd47a8` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5f744 | `ret(0x60f28) == 0x0` | `ret(0x60f28) != 0x0` |
| `blt` at 0x5f788 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x5f7d0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5f814 | `r0` | `?` |

## `IP57_NodeDisable`

- `IP57_NodeDisable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5f81c to 0x5f854.
  Established. A byte save records `(r0 & 0xff)` at `sp -0x11` (`strb` at 0x5f830). The call to
  `0x5f978` has candidates `(r0 & 0xff)`, `0x0`, `0x1` and `(r0 & 0xff)` (`bl` at 0x5f844); its result
  is retained at the exit (`ldmdb` at 0x5f850). The constants' hardware effect remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5f830 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x5f834 | load | 1 | `sp -0x11` | `-` |
| `ldmdb` at 0x5f850 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5f850 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5f850 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5f844 | `(unknown callee) 0x5f978` | `(r0 & 0xff)` | `0x0` | `0x1` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5f850 | `r0` | `ret(0x5f978)` |

## `IP57_NodeEnable`

- `IP57_NodeEnable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5f854 to 0x5f888.
  Established. The call to `0x5fb3c` has candidates `(r0 & 0xff)`, `0x0`, entry r2 and `(r0 & 0xff)`
  (`bl` at 0x5f878). The body forwards `ret(0x5fb3c)` to the recorded return (`ldmdb` at 0x5f884).
  An enable action cannot be proved without the callee's effects, so the role remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5f868 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x5f86c | load | 1 | `sp -0x11` | `-` |
| `ldmdb` at 0x5f884 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5f884 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5f884 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5f878 | `(unknown callee) 0x5fb3c` | `(r0 & 0xff)` | `0x0` | `r2` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5f884 | `r0` | `ret(0x5fb3c)` |

## `IP57_NodePresent`

- `IP57_NodePresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5fdc0 to 0x5fdfc.
  Established. The byte load uses the effective expression `(r0 & 0xff) +0x2b08e0` (`ldrb` at
  0x5fde0). Conditional moves select their values after the comparison, but the final r0 expression
  remains unknown in the fact sheet (`moveq` at 0x5fde8, `movne` at 0x5fdec, `ldmdb` at 0x5fdf4).
  This span alone does not establish a physical presence indication.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5fdd4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x5fdd8 | load | 4 | `abs 0x5fdf8` | `-` |
| `ldrb` at 0x5fddc | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x5fde0 | load | 1 | `(r0 & 0xff) +0x2b08e0` | `-` |
| `ldmdb` at 0x5fdf4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5fdf4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5fdf4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5fdf4 | `r0` | `?` |
