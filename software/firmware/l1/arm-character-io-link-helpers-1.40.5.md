# ARM character I/O and link helpers in 1.40.5

Read from `6.5.29/usr/cpu/firmware/sysco/l1.bin`, package `6.5.29:l1#4` (version 1.40.5),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53d448`.
Carrier SHA-256: `80f4a616601d187bc3839a2758f2e7f2d0db962ad7fd8f2d96d41bcd15bdf88c`.
Package SHA-256: `4ccd1a6e7054bc3cd56e962a60737d1c90339ffa93fcc8af0912b6c9c7d385dd`.
Inner executable SHA-256: `08ebc648bb298017c7e3d6049983f2a54a8fe56f1bb5738df88b82c38f75ac3a`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1257088 bytes.
Segment SHA-256: `dc7cc5d18fbdf816d328411825c9c01e26c3f8852f1012c7a642aea0767e2665`. Analysed routine spans use the exact
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
No unresolved computed transfers are reported. Unknown data values and instruction effects remain
explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/character-io-link-helpers-facts.txt) retain all tool sections.

## `charioInit`

- `charioInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x23bd8 to 0x23c50.
  Established. The unnamed `0x229b0` call carries `0x146250`, `0xff350`, `0x1`, `0x9` (`bl` at 0x23c28). The absolute word assignment at `0x1462a4` writes `[sp-20]` (`str` at 0x23c34).
  Character I/O setup effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x23be8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x23bec | load | 4 | `abs 0x23c3c` | `-` |
| `ldr` at 0x23bf0 | load | 4 | `abs 0x1462a4` | `-` |
| `ldr` at 0x23bfc | load | 4 | `abs 0x23c40` | `-` |
| `str` at 0x23c00 | store | 4 | `sp -0x1c` | `0xff310` |
| `ldr` at 0x23c08 | load | 4 | `abs 0x23c44` | `-` |
| `ldr` at 0x23c18 | load | 4 | `abs 0x23c48` | `-` |
| `ldr` at 0x23c1c | load | 4 | `abs 0x23c4c` | `-` |
| `ldr` at 0x23c2c | load | 4 | `abs 0x23c3c` | `-` |
| `ldr` at 0x23c30 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x23c34 | store | 4 | `abs 0x1462a4` | `[sp-20]` |
| `ldmdb` at 0x23c38 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x23c38 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x23c38 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x23c14 | `(unknown callee) 0x21eb8` | `0x9` | `0xff334` | `0x73` | `0x0` |
| `bl` at 0x23c28 | `(unknown callee) 0x229b0` | `0x146250` | `0xff350` | `0x1` | `0x9` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x23bf8 | `[0x1462a4] == 0x0` | `[0x1462a4] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x23c38 | `r0` | `ret(0x229b0)` |

## `charioOpen`

- `charioOpen`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x23c50 to 0x23e04.
  Established. The absolute word at `0x1462a4` is read (`ldr` at 0x23cf4); its nonzero value takes
  `0x23d38`, while zero falls through (`bne` at 0x23cfc). The unnamed `0xca524` call carries
  `[0x1462a4]`, `[sp-28]`, unknown, `0x1462a4` (`bl` at 0x23d44). Open-mode interpretation remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x23c60 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x23c68 | store | 4 | `sp -0x1c` | `0x1` |
| `str` at 0x23c70 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x23c74 | load | 4 | `abs 0x23de4` | `-` |
| `ldr` at 0x23c8c | load | 4 | `abs 0x23de8` | `-` |
| `ldr` at 0x23c90 | load | 4 | `abs 0x145ff8` | `-` |
| `str` at 0x23ca0 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x23ca4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x23ca8 | store | 4 | `sp -0x20` | `[sp-36]` |
| `ldr` at 0x23cb4 | load | 4 | `abs 0x23dec` | `-` |
| `str` at 0x23cb8 | store | 4 | `sp -0x30` | `0xff35c` |
| `ldr` at 0x23cbc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x23cc0 | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0x23cc8 | load | 4 | `abs 0x23df0` | `-` |
| `ldr` at 0x23cd8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x23ce4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x23cec | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x23cf0 | load | 4 | `abs 0x23df4` | `-` |
| `ldr` at 0x23cf4 | load | 4 | `abs 0x1462a4` | `-` |
| `ldr` at 0x23d00 | load | 4 | `abs 0x23df8` | `-` |
| `str` at 0x23d10 | store | 4 | `abs 0x1462a8` | `ret(0x1b1b4)` |
| `ldr` at 0x23d14 | load | 4 | `abs 0x23df8` | `-` |
| `ldr` at 0x23d18 | load | 4 | `abs 0x1462a8` | `-` |
| `ldr` at 0x23d28 | load | 4 | `abs 0x23dfc` | `-` |
| `str` at 0x23d30 | store | 4 | `abs 0x1462ac` | `0x1` |
| `ldr` at 0x23d38 | load | 4 | `abs 0x23df4` | `-` |
| `ldr` at 0x23d3c | load | 4 | `abs 0x1462a4` | `-` |
| `ldr` at 0x23d40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x23d4c | load | 4 | `abs 0x23df8` | `-` |
| `str` at 0x23d50 | store | 4 | `abs 0x1462a8` | `ret(0xca524)` |
| `ldr` at 0x23d54 | load | 4 | `abs 0x1462a8` | `-` |
| `ldr` at 0x23d60 | load | 4 | `abs 0x23dfc` | `-` |
| `str` at 0x23d68 | store | 4 | `abs 0x1462ac` | `0x1` |
| `str` at 0x23d70 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x23d74 | load | 4 | `abs 0x23de4` | `-` |
| `ldr` at 0x23d88 | load | 4 | `abs 0x23de8` | `-` |
| `ldr` at 0x23d8c | load | 4 | `abs 0x145ff8` | `-` |
| `str` at 0x23d9c | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x23da0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x23da4 | store | 4 | `sp -0x20` | `[sp-40]` |
| `ldr` at 0x23db0 | load | 4 | `abs 0x23e00` | `-` |
| `str` at 0x23db4 | store | 4 | `sp -0x30` | `0xff380` |
| `ldr` at 0x23db8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x23dbc | store | 4 | `sp -0x2c` | `[sp-40]` |
| `ldr` at 0x23dc4 | load | 4 | `abs 0x23df0` | `-` |
| `ldr` at 0x23dd4 | load | 4 | `abs 0x23df8` | `-` |
| `ldr` at 0x23dd8 | load | 4 | `abs 0x1462a8` | `-` |
| `ldmdb` at 0x23de0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x23de0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x23de0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x23de0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x23c7c | `(unknown callee) 0x22ac8` | `0x146250` | `?` | `r2` | `0x0` |
| `bl` at 0x23cd4 | `(unknown callee) 0x21eb8` | `0x9` | `0xff334` | `0x93` | `0x0` |
| `bl` at 0x23d08 | `(unknown callee) 0x1b1b4` | `0x108` | `?` | `?` | `[0x1462a4]` |
| `bl` at 0x23d24 | `(unknown callee) 0x1b4cc` | `[0x1462a8]` | `0x0` | `0x108` | `0x1462a8` |
| `bl` at 0x23d44 | `(unknown callee) 0xca524` | `[0x1462a4]` | `[sp-28]` | `?` | `0x1462a4` |
| `bl` at 0x23d78 | `(unknown callee) 0x22e50` | `0x146250` | `?` | `?` | `0x0` |
| `bl` at 0x23dd0 | `(unknown callee) 0x21eb8` | `0x9` | `0xff334` | `0xab` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x23c88 | `ret(0x22ac8) == 0x0` | `ret(0x22ac8) != 0x0` |
| `beq` at 0x23c98 | `[0x145ff8] == 0x1` | `[0x145ff8] != 0x1` |
| `beq` at 0x23cb0 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0x23ce0 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `bne` at 0x23cfc | `[0x1462a4] != 0x0` | `[0x1462a4] == 0x0` |
| `beq` at 0x23d5c | `[0x1462a8] == 0x0` | `[0x1462a8] != 0x0` |
| `beq` at 0x23d84 | `ret(0x22e50) == 0x0` | `ret(0x22e50) != 0x0` |
| `beq` at 0x23d94 | `[0x145ff8] == 0x1` | `[0x145ff8] != 0x1` |
| `beq` at 0x23dac | `[sp-40] == 0x0` | `[sp-40] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x23de0 | `r0` | `[0x1462a8]` |

## `charioClose`

- `charioClose`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x23e24 to 0x23f54.
  Established. The unnamed `0xca99c` call carries `[0x1462a8]`, `0x1`, unknown, `0x1462a8` (`bl` at 0x23ed4). The absolute word assignment at `0x1462ac` writes `0x0` (`str` at 0x23ee8). Close
  effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x23e38 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x23e3c | load | 4 | `abs 0x23f38` | `-` |
| `str` at 0x23e4c | store | 4 | `sp -0x18` | `ret(0x22ac8)` |
| `ldr` at 0x23e50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x23e5c | load | 4 | `abs 0x23f3c` | `-` |
| `str` at 0x23e60 | store | 4 | `sp -0x20` | `0xff35c` |
| `ldr` at 0x23e64 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x23e68 | store | 4 | `sp -0x1c` | `ret(0x22ac8)` |
| `ldr` at 0x23e70 | load | 4 | `abs 0x23f40` | `-` |
| `ldr` at 0x23e80 | load | 4 | `abs 0x23f44` | `-` |
| `ldr` at 0x23e84 | load | 4 | `abs 0x1462a4` | `-` |
| `ldr` at 0x23e90 | load | 4 | `abs 0x23f48` | `-` |
| `ldr` at 0x23e94 | load | 4 | `abs 0x1462a8` | `-` |
| `ldr` at 0x23e9c | load | 4 | `abs 0x23f48` | `-` |
| `str` at 0x23ea4 | store | 4 | `abs 0x1462a8` | `0x0` |
| `ldr` at 0x23ea8 | load | 4 | `abs 0x23f4c` | `-` |
| `str` at 0x23eb0 | store | 4 | `abs 0x1462ac` | `0x0` |
| `ldr` at 0x23eb8 | load | 4 | `abs 0x23f4c` | `-` |
| `ldr` at 0x23ebc | load | 4 | `abs 0x1462ac` | `-` |
| `ldr` at 0x23ec8 | load | 4 | `abs 0x23f48` | `-` |
| `ldr` at 0x23ecc | load | 4 | `abs 0x1462a8` | `-` |
| `str` at 0x23edc | store | 4 | `sp -0x14` | `ret(0xca99c)` |
| `ldr` at 0x23ee0 | load | 4 | `abs 0x23f4c` | `-` |
| `str` at 0x23ee8 | store | 4 | `abs 0x1462ac` | `0x0` |
| `ldr` at 0x23eec | load | 4 | `abs 0x23f38` | `-` |
| `str` at 0x23ef8 | store | 4 | `sp -0x18` | `ret(0x22e50)` |
| `ldr` at 0x23efc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x23f08 | load | 4 | `abs 0x23f50` | `-` |
| `str` at 0x23f0c | store | 4 | `sp -0x20` | `0xff380` |
| `ldr` at 0x23f10 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x23f14 | store | 4 | `sp -0x1c` | `ret(0x22e50)` |
| `ldr` at 0x23f1c | load | 4 | `abs 0x23f40` | `-` |
| `ldr` at 0x23f2c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x23f34 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x23f34 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x23f34 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x23e44 | `(unknown callee) 0x22ac8` | `0x146250` | `?` | `r2` | `0x0` |
| `bl` at 0x23e7c | `(unknown callee) 0x21eb8` | `0x9` | `0xff334` | `0xce` | `0x0` |
| `bl` at 0x23e98 | `(unknown callee) 0x1b490` | `[0x1462a8]` | `?` | `?` | `0x1462a8` |
| `bl` at 0x23ed4 | `(unknown callee) 0xca99c` | `[0x1462a8]` | `0x1` | `?` | `0x1462a8` |
| `bl` at 0x23ef0 | `(unknown callee) 0x22e50` | `0x146250` | `?` | `?` | `?` |
| `bl` at 0x23f28 | `(unknown callee) 0x21eb8` | `0x9` | `0xff334` | `0xe5` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x23e58 | `ret(0x22ac8) == 0x0` | `ret(0x22ac8) != 0x0` |
| `bne` at 0x23e8c | `[0x1462a4] != 0x0` | `[0x1462a4] == 0x0` |
| `beq` at 0x23ec4 | `[0x1462ac] == 0x0` | `[0x1462ac] != 0x0` |
| `beq` at 0x23f04 | `ret(0x22e50) == 0x0` | `ret(0x22e50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x23f34 | `r0` | `[sp-20]` |

## `charioLock`

- `charioLock`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x23f54 to 0x23fa0.
  Established. The unnamed `0x242d8` call carries `r0`, `r1`, `r2`, `r3` (`bl` at 0x23f6c); its zero
  result takes `0x23f88`, while nonzero falls through (`beq` at 0x23f78). The absolute word at
  `0x1462a8` is read (`ldr` at 0x23f8c). Lock ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x23f64 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x23f68 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x23f80 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x23f88 | load | 4 | `abs 0x23f9c` | `-` |
| `ldr` at 0x23f8c | load | 4 | `abs 0x1462a8` | `-` |
| `str` at 0x23f90 | store | 4 | `sp -0x18` | `[0x1462a8]` |
| `ldr` at 0x23f94 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x23f98 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x23f98 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x23f98 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x23f6c | `(unknown callee) 0x242d8` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x23f78 | `ret(0x242d8) == 0x0` | `ret(0x242d8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x23f98 | `r0` | `[sp-24]` |

## `charioUnlock`

- `charioUnlock`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x23fa0 to 0x23fbc.
  Established. The unnamed `0x243f8` call uses `r0`, `r1`, `r2`, `r3` (`bl` at 0x23fac); its result
  is reported by the return (`ldmdb` at 0x23fb8). Lock-release semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x23fb8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x23fb8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x23fb8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x23fac | `(unknown callee) 0x243f8` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x23fb8 | `r0` | `ret(0x243f8)` |

## `chario_redirect`

- `chario_redirect`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24548 to 0x24590.
  Established. Entry `r0 == 0x0` takes `0x24578`, while nonzero falls through (`beq` at 0x24564).
  The absolute word assignments at `0x1430e8` write `r0` and `0xcad54` (`str` at 0x24570, `str` at 0x24580). Redirection behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x24558 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2455c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x24568 | load | 4 | `abs 0x24588` | `-` |
| `ldr` at 0x2456c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x24570 | store | 4 | `abs 0x1430e8` | `r0` |
| `ldr` at 0x24578 | load | 4 | `abs 0x24588` | `-` |
| `ldr` at 0x2457c | load | 4 | `abs 0x2458c` | `-` |
| `str` at 0x24580 | store | 4 | `abs 0x1430e8` | `0xcad54` |
| `ldmdb` at 0x24584 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x24584 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x24584 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x24564 | `r0 == 0x0` | `r0 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x24584 | `r0` | `r0` |

## `ipmiInit`

- `ipmiInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x21d40 to 0x21d50.
  Established. The return reports entry `r0` (`ldmdb` at 0x21d4c). The same instruction reads words
  at `sp -0x10`, `sp -0xc` and `sp -0x8` (`ldmdb` at 0x21d4c). IPMI initialization effects remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x21d4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x21d4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x21d4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x21d4c | `r0` | `r0` |

## `ipmiCheckCSum`

- `ipmiCheckCSum`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x21d50 to 0x21dc4.
  Established. The byte assignment at `sp -0x19` initially writes `0x0` (`strb` at 0x21d6c). A later
  byte load has an unknown effective address (`ldrb` at 0x21da0). The updated byte assignment at `sp -0x19` has an unknown value (`strb` at 0x21da8). Checksum result interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x21d60 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x21d64 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x21d6c | store | 1 | `sp -0x19` | `0x0` |
| `ldr` at 0x21d74 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x21d80 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x21d94 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x21d9c | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0x21da0 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x21da8 | store | 1 | `sp -0x19` | `?` |
| `str` at 0x21db0 | store | 4 | `sp -0x14` | `?` |
| `ldrb` at 0x21db8 | load | 1 | `sp -0x19` | `-` |
| `ldmdb` at 0x21dc0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x21dc0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x21dc0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x21d88 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x21dc0 | `r0` | `?` |

## `ipmiCalcCSum`

- `ipmiCalcCSum`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x21dc4 to 0x21e3c.
  Established. The word assignment at `sp -0x18` has an unknown value (`str` at 0x21dec). The byte
  load at `0x21e0c` has an unknown effective target (`ldrb` at 0x21e0c). The return reports an
  unknown value (`ldmdb` at 0x21e38). Checksum calculation meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x21dd4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x21dd8 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x21de0 | store | 1 | `sp -0x19` | `0x0` |
| `ldr` at 0x21de4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x21dec | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x21e00 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x21e08 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0x21e0c | load | 1 | `(unknown)` | `-` |
| `strb` at 0x21e14 | store | 1 | `sp -0x19` | `?` |
| `str` at 0x21e1c | store | 4 | `sp -0x14` | `?` |
| `ldrb` at 0x21e24 | load | 1 | `sp -0x19` | `-` |
| `ldmdb` at 0x21e38 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x21e38 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x21e38 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x21df4 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x21e38 | `r0` | `?` |

## `AthenaCommPollInit`

- `AthenaCommPollInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2cf14 to 0x2cf70.
  Established. The word assignments at `sp -0x24` and `sp -0x18` write `0x800` and `0x9` (`str` at
  0x2cf30, `str` at 0x2cf48). The unnamed `0x22480` call carries `0x148bf0`, `0x1042e8`, `0x2cf70`,
  `0x1` (`bl` at 0x2cf5c). Poll setup effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2cf28 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x2cf30 | store | 4 | `sp -0x24` | `0x800` |
| `str` at 0x2cf38 | store | 4 | `sp -0x20` | `0xa` |
| `str` at 0x2cf40 | store | 4 | `sp -0x1c` | `0xa` |
| `str` at 0x2cf48 | store | 4 | `sp -0x18` | `0x9` |
| `ldr` at 0x2cf4c | load | 4 | `abs 0x2cf64` | `-` |
| `ldr` at 0x2cf50 | load | 4 | `abs 0x2cf68` | `-` |
| `ldr` at 0x2cf54 | load | 4 | `abs 0x2cf6c` | `-` |
| `ldmdb` at 0x2cf60 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2cf60 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2cf60 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2cf5c | `(unknown callee) 0x22480` | `0x148bf0` | `0x1042e8` | `0x2cf70` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2cf60 | `r0` | `ret(0x22480)` |

## `athenaComm`

- `athenaComm`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2cf70 to 0x2d024.
  Established. The unnamed `0xd7530` call carries `0x0`, `0x9`, `sp-25`, `sp-25` (`bl` at 0x2cfac).
  The unnamed `0x54598` call carries `0x0`, `0x9`, `0x2`, `[sp-25]:1` (`bl` at 0x2d004).
  Communication state meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2cf80 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x2cf84 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x2cfb4 | store | 1 | `sp -0x1a` | `(ret(0xd7530) & 0xff)` |
| `ldrb` at 0x2cfb8 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0x2cfc8 | load | 4 | `abs 0x2d018` | `-` |
| `str` at 0x2cfcc | store | 4 | `sp -0x20` | `0x1042f4` |
| `ldr` at 0x2cfd4 | load | 4 | `abs 0x2d01c` | `-` |
| `ldr` at 0x2cfd8 | load | 4 | `abs 0x2d020` | `-` |
| `ldrb` at 0x2cfe4 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0x2cff4 | load | 1 | `sp -0x19` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2cf8c | `(unknown callee) 0x54404` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x2cfac | `(unknown callee) 0xd7530` | `0x0` | `0x9` | `sp-25` | `sp-25` |
| `bl` at 0x2cfe0 | `(unknown callee) 0x21eb8` | `0x1` | `0x104190` | `0x8c5` | `0x0` |
| `bl` at 0x2d004 | `(unknown callee) 0x54598` | `0x0` | `0x9` | `0x2` | `[sp-25]:1` |
| `bl` at 0x2d010 | `(unknown callee) 0x2d2f0` | `0xfa` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x2cf98 | `ret(0x54404) == 0x0` | `ret(0x54404) != 0x0` |
| `beq` at 0x2cfc4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x2cff0 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|

## `networkGetType`

- `networkGetType`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24854 to 0x24874.
  Established. The absolute byte at `0x148bc0` is read (`ldrb` at 0x24864). The return reports
  `[0x148bc0]:1` (`ldmdb` at 0x2486c). Network-type encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x24860 | load | 4 | `abs 0x24870` | `-` |
| `ldrb` at 0x24864 | load | 1 | `abs 0x148bc0` | `-` |
| `ldmdb` at 0x2486c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2486c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2486c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2486c | `r0` | `[0x148bc0]:1` |

## `NL4RTarget`

- `NL4RTarget`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x279d0 to 0x27a0c.
  Established. The absolute word at `0x148bdc` is loaded (`ldr` at 0x279f4). The subsequent word
  read has an unknown effective address (`ldr` at 0x279fc). The reported return value is unknown
  (`ldmdb` at 0x27a04). Target selection remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x279e4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x279e8 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x279f0 | load | 4 | `abs 0x27a08` | `-` |
| `ldr` at 0x279f4 | load | 4 | `abs 0x148bdc` | `-` |
| `ldr` at 0x279fc | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x27a04 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x27a04 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x27a04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x27a04 | `r0` | `?` |

## `NL4R_set_link_enable`

- `NL4R_set_link_enable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x27fa8 to 0x2802c.
  Established. The absolute byte read uses `0x148bd9` (`ldrb` at 0x27fe8). A nonzero value takes
  `0x28000`, while zero falls through (`bne` at 0x27ff0). The unnamed `0x2928c` call carries `r0`,
  `r1`, `r2`, `[0x148bd9]:1` (`bl` at 0x2800c). Link-enable effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x27fb8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x27fbc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x27fc0 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x27fc4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27fdc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x27fe4 | load | 4 | `abs 0x28028` | `-` |
| `ldrb` at 0x27fe8 | load | 1 | `abs 0x148bd9` | `-` |
| `str` at 0x27ff8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x28000 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28004 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28008 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x28014 | store | 4 | `sp -0x20` | `ret(0x2928c)` |
| `ldr` at 0x28018 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x2801c | store | 4 | `sp -0x24` | `ret(0x2928c)` |
| `ldr` at 0x28020 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x28024 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x28024 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x28024 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2800c | `(unknown callee) 0x2928c` | `r0` | `r1` | `r2` | `[0x148bd9]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x27fd4 | `? == 0x6` | `? != 0x6` |
| `bne` at 0x27ff0 | `[0x148bd9]:1 != 0x0` | `[0x148bd9]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x28024 | `r0` | `?` |

## `vortix_set_link_enable`

- `vortix_set_link_enable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x25664 to 0x25750.
  Established. The signed condition `r1 > 0x1` takes `0x256cc`, while smaller-or-equal values fall
  through (`bgt` at 0x256c4). The absolute byte at `0x148bc4` is read (`ldrb` at 0x256dc). The
  unnamed `0x4de84` call carries `r0`, `r1`, `r2`, unknown (`bl` at 0x25730). Link-control effects
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x25674 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x25678 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x2567c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x25680 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25694 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x256a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x256bc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x256d0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x256d8 | load | 4 | `abs 0x2574c` | `-` |
| `ldrb` at 0x256dc | load | 1 | `abs 0x148bc4` | `-` |
| `str` at 0x256ec | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x256f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25708 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2570c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x25710 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2571c | store | 4 | `sp -0x20` | `ret(0x2928c)` |
| `ldr` at 0x25724 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25728 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2572c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x25738 | store | 4 | `sp -0x20` | `ret(0x4de84)` |
| `ldr` at 0x2573c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x25740 | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0x25744 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x25748 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x25748 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x25748 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x25714 | `(unknown callee) 0x2928c` | `r0` | `r1` | `r2` | `?` |
| `bl` at 0x25730 | `(unknown callee) 0x4de84` | `r0` | `r1` | `r2` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x25690 | `? == 0x6` | `? != 0x6` |
| `bne` at 0x256a4 | `? != 0x2` | `? == 0x2` |
| `bne` at 0x256b8 | `? != 0x2` | `? == 0x2` |
| `bgt` at 0x256c4 | `r1 > 0x1 (signed, 32-bit)` | `r1 <= 0x1 (signed, 32-bit)` |
| `bne` at 0x256e4 | `[0x148bc4]:1 != 0x0` | `[0x148bc4]:1 == 0x0` |
| `bne` at 0x25704 | `? != 0x6` | `? == 0x6` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x25748 | `r0` | `?` |
