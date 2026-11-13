# ARM board-operation records in 1.38.4

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
One routine has unresolved computed-transfer destinations; its tables and return cover only known edges. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/board-operation-records-facts.txt) retain all tool sections.

## `brickInitBrickOps`

- `brickInitBrickOps`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1b910 to 0x1bb4c.
  Established. The computed selector takes its arm for unsigned `r1-43 <= 0x51` and falls through
  for greater (`ldrls` at 0x1b934); its taken destinations are unresolved and excluded from the fact
  sections (`ldrls` at 0x1b934). The reported known-edge call to `0x24588` has entry r0, entry r1,
  entry r2, `r1-43` (`bl` at 0x1bb28). The known-edge return propagates `ret(0xaf5ac)` (`ldmdb` at
  0x1bb48). Board selection on excluded paths remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1b920 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x1b924 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x1b928 | load | 4 | `sp -0x18` | `-` |
| `ldrls` at 0x1b934 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1bb20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1bb24 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x1bb48 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x1bb48 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x1bb48 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1bb28 | `(unknown callee) 0x24588` | `r0` | `r1` | `r2` | `r1-43` |
| `bl` at 0x1bb2c | `(unknown callee) 0xda5f0` | `ret(0x24588)` | `?` | `?` | `?` |
| `bl` at 0x1bb30 | `(unknown callee) 0xe77f4` | `ret(0xda5f0)` | `?` | `?` | `?` |
| `bl` at 0x1bb34 | `(unknown callee) 0xf36e0` | `ret(0xe77f4)` | `?` | `?` | `?` |
| `bl` at 0x1bb38 | `(unknown callee) 0xe8ffc` | `ret(0xf36e0)` | `?` | `?` | `?` |
| `bl` at 0x1bb3c | `(unknown callee) 0xa2ff8` | `ret(0xe8ffc)` | `?` | `?` | `?` |
| `bl` at 0x1bb40 | `(unknown callee) 0xa6740` | `ret(0xa2ff8)` | `?` | `?` | `?` |
| `bl` at 0x1bb44 | `(unknown callee) 0xaf5ac` | `ret(0xa6740)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x1b934 | `r1-43 <= 0x51 (unsigned, 32-bit)` | `r1-43 > 0x51 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x1bb48 | `r0` | `ret(0xaf5ac)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x1b934).

## `Unknown_BrickOpsInit`

- `Unknown_BrickOpsInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24588 to 0x24664.
  Established. The word store at entry `r0 +0x4` writes `0x24500` (`str` at 0x245ac). A byte store
  writes `0x4e` at `[sp-20] +0xc` (`strb` at 0x245b8). Later word stores clear `[sp-20] +0xd0` and
  `[sp-20] +0xd4` (`str` at 0x24648, `str` at 0x24654). The exit retains entry r0 (`ldmdb` at
  0x24658). Field purposes and callback invocation remain unestablished.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x24598 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2459c | load | 4 | `abs 0x2465c` | `-` |
| `str` at 0x245a0 | store | 4 | `sp -0x18` | `0x1423e4` |
| `ldr` at 0x245a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x245a8 | load | 4 | `abs 0x24660` | `-` |
| `str` at 0x245ac | store | 4 | `r0 +0x4` | `0x24500` |
| `ldr` at 0x245b0 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x245b8 | store | 1 | `[sp-20] +0xc` | `0x4e` |
| `ldr` at 0x245bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x245c4 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x245c8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x245d0 | store | 4 | `[sp-20] +0xa0` | `0xff` |
| `ldr` at 0x245d4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x245dc | store | 4 | `[sp-20] +0xa4` | `0xff` |
| `ldr` at 0x245e0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x245e8 | store | 4 | `[sp-20] +0xa8` | `0xff` |
| `ldr` at 0x245ec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x245f4 | store | 4 | `[sp-20] +0xac` | `0xff` |
| `ldr` at 0x245f8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x24600 | store | 4 | `[sp-20] +0x88` | `0x0` |
| `ldr` at 0x24604 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2460c | store | 4 | `[sp-20] +0x90` | `0x0` |
| `ldr` at 0x24610 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x24618 | store | 4 | `[sp-20] +0x94` | `0x0` |
| `ldr` at 0x2461c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x24624 | store | 4 | `[sp-20] +0x98` | `0x0` |
| `ldr` at 0x24628 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x24630 | store | 4 | `[sp-20] +0xc4` | `0x0` |
| `ldr` at 0x24634 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2463c | store | 4 | `[sp-20] +0xc8` | `0x0` |
| `ldr` at 0x24640 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x24648 | store | 4 | `[sp-20] +0xd0` | `0x0` |
| `ldr` at 0x2464c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x24654 | store | 4 | `[sp-20] +0xd4` | `0x0` |
| `ldmdb` at 0x24658 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x24658 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x24658 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x24658 | `r0` | `r0` |

## `Vortix_BrickOpsInit`

- `Vortix_BrickOpsInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x25d14 to 0x25f1c.
  Established. The two calls to `0x1b304` have `sp-96`, `0x100618`, `0x4c`, `0x100618` and
  `[sp-20]+24`, `sp-96`, `0x4c`, `[sp-20]+24`, respectively (`bl` at 0x25d40, `bl` at 0x25e0c). Word
  stores write `0x25334` at `[sp-20] +0xec` and `0x255d4` at `[sp-20] +0xf0` (`str` at 0x25ec0,
  `str` at 0x25ecc). The return propagates `ret(0x1b304)` (`ldmdb` at 0x25ed0). Board-record layout
  meaning and callee semantics are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x25d24 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x25d28 | load | 4 | `abs 0x25ed4` | `-` |
| `ldr` at 0x25d44 | load | 4 | `abs 0x25ed8` | `-` |
| `str` at 0x25d48 | store | 4 | `sp -0x64` | `0x142450` |
| `ldr` at 0x25d4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25d50 | load | 4 | `abs 0x25edc` | `-` |
| `str` at 0x25d54 | store | 4 | `[sp-20] +0x0` | `0x25c08` |
| `ldr` at 0x25d58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25d5c | load | 4 | `abs 0x25ee0` | `-` |
| `str` at 0x25d60 | store | 4 | `[sp-20] +0x4` | `0x258f8` |
| `ldr` at 0x25d64 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x25d6c | store | 1 | `[sp-20] +0xc` | `0x4e` |
| `ldr` at 0x25d70 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25d78 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x25d7c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25d84 | store | 4 | `[sp-20] +0xa0` | `0x19` |
| `ldr` at 0x25d88 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25d90 | store | 4 | `[sp-20] +0xa4` | `0x22` |
| `ldr` at 0x25d94 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25d9c | store | 4 | `[sp-20] +0xa8` | `0x2b` |
| `ldr` at 0x25da0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25da8 | store | 4 | `[sp-20] +0xac` | `0x34` |
| `ldr` at 0x25dac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25db0 | load | 4 | `abs 0x25ee4` | `-` |
| `str` at 0x25db4 | store | 4 | `[sp-20] +0xb0` | `0x10e394` |
| `ldr` at 0x25db8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25dc0 | store | 4 | `[sp-20] +0x68` | `0x1e` |
| `ldr` at 0x25dc4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25dcc | store | 4 | `[sp-20] +0x6c` | `0x2d` |
| `ldr` at 0x25dd0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25dd8 | store | 4 | `[sp-20] +0x70` | `0x32` |
| `ldr` at 0x25ddc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25de4 | store | 4 | `[sp-20] +0x74` | `0x37` |
| `ldr` at 0x25de8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25df0 | store | 4 | `[sp-20] +0x78` | `0x19` |
| `ldr` at 0x25df4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25e10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25e14 | load | 4 | `abs 0x25ee8` | `-` |
| `str` at 0x25e18 | store | 4 | `[sp-20] +0x7c` | `0x24f74` |
| `ldr` at 0x25e1c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25e24 | store | 4 | `[sp-20] +0x8c` | `0x1` |
| `ldr` at 0x25e28 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25e30 | store | 4 | `[sp-20] +0x88` | `0x8` |
| `ldr` at 0x25e34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25e38 | load | 4 | `abs 0x25eec` | `-` |
| `str` at 0x25e3c | store | 4 | `[sp-20] +0x90` | `0x5fdfc` |
| `ldr` at 0x25e40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25e44 | load | 4 | `abs 0x25ef0` | `-` |
| `str` at 0x25e48 | store | 4 | `[sp-20] +0x94` | `0x5fee4` |
| `ldr` at 0x25e4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25e50 | load | 4 | `abs 0x25ef4` | `-` |
| `str` at 0x25e54 | store | 4 | `[sp-20] +0x98` | `0x5fe74` |
| `ldr` at 0x25e58 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25e60 | store | 4 | `[sp-20] +0xb4` | `0x4` |
| `ldr` at 0x25e64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25e68 | load | 4 | `abs 0x25ef8` | `-` |
| `str` at 0x25e6c | store | 4 | `[sp-20] +0xb8` | `0x24c3c` |
| `ldr` at 0x25e70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25e74 | load | 4 | `abs 0x25efc` | `-` |
| `str` at 0x25e78 | store | 4 | `[sp-20] +0xbc` | `0x24ca4` |
| `ldr` at 0x25e7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25e80 | load | 4 | `abs 0x25f00` | `-` |
| `str` at 0x25e84 | store | 4 | `[sp-20] +0xc0` | `0x24d0c` |
| `ldr` at 0x25e88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25e8c | load | 4 | `abs 0x25f04` | `-` |
| `str` at 0x25e90 | store | 4 | `[sp-20] +0xc4` | `0x24a0c` |
| `ldr` at 0x25e94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25e98 | load | 4 | `abs 0x25f08` | `-` |
| `str` at 0x25e9c | store | 4 | `[sp-20] +0xc8` | `0x24abc` |
| `ldr` at 0x25ea0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25ea4 | load | 4 | `abs 0x25f0c` | `-` |
| `str` at 0x25ea8 | store | 4 | `[sp-20] +0xd0` | `0x24b90` |
| `ldr` at 0x25eac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25eb0 | load | 4 | `abs 0x25f10` | `-` |
| `str` at 0x25eb4 | store | 4 | `[sp-20] +0xd4` | `0x24c08` |
| `ldr` at 0x25eb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25ebc | load | 4 | `abs 0x25f14` | `-` |
| `str` at 0x25ec0 | store | 4 | `[sp-20] +0xec` | `0x25334` |
| `ldr` at 0x25ec4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25ec8 | load | 4 | `abs 0x25f18` | `-` |
| `str` at 0x25ecc | store | 4 | `[sp-20] +0xf0` | `0x255d4` |
| `ldmdb` at 0x25ed0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x25ed0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x25ed0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x25d40 | `(unknown callee) 0x1b304` | `sp-96` | `0x100618` | `0x4c` | `0x100618` |
| `bl` at 0x25e0c | `(unknown callee) 0x1b304` | `[sp-20]+24` | `sp-96` | `0x4c` | `[sp-20]+24` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x25ed0 | `r0` | `ret(0x1b304)` |

## `Gemini_BrickOpsInit`

- `Gemini_BrickOpsInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2765c to 0x27808.
  Established. The first non-stack word write places `0x27604` at entry `r0 +0x0` (`str` at
  0x27680). The byte store at `[sp-20] +0xc` writes `0x0` (`strb` at 0x27698). Near the end, word
  writes place `0x26bf4` at `[sp-20] +0xec` and `0x26b08` at `[sp-20] +0xf0` (`str` at 0x277b8,
  `str` at 0x277c4). Entry r0 survives at the exit (`ldmdb` at 0x277c8). Physical configuration and
  stored-address roles remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2766c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x27670 | load | 4 | `abs 0x277cc` | `-` |
| `str` at 0x27674 | store | 4 | `sp -0x18` | `0x1424c4` |
| `ldr` at 0x27678 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2767c | load | 4 | `abs 0x277d0` | `-` |
| `str` at 0x27680 | store | 4 | `r0 +0x0` | `0x27604` |
| `ldr` at 0x27684 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27688 | load | 4 | `abs 0x277d4` | `-` |
| `str` at 0x2768c | store | 4 | `[sp-20] +0x4` | `0x273a8` |
| `ldr` at 0x27690 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x27698 | store | 1 | `[sp-20] +0xc` | `0x0` |
| `ldr` at 0x2769c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x276a4 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x276a8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x276b0 | store | 4 | `[sp-20] +0xa0` | `0xf` |
| `ldr` at 0x276b4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x276bc | store | 4 | `[sp-20] +0xa4` | `0xff` |
| `ldr` at 0x276c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x276c8 | store | 4 | `[sp-20] +0xa8` | `0xff` |
| `ldr` at 0x276cc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x276d4 | store | 4 | `[sp-20] +0xac` | `0xff` |
| `ldr` at 0x276d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x276dc | load | 4 | `abs 0x277d8` | `-` |
| `str` at 0x276e0 | store | 4 | `[sp-20] +0x14` | `0x1012b8` |
| `ldr` at 0x276e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x276e8 | load | 4 | `abs 0x277dc` | `-` |
| `str` at 0x276ec | store | 4 | `[sp-20] +0xb0` | `0x10e394` |
| `ldr` at 0x276f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x276f4 | load | 4 | `abs 0x277e0` | `-` |
| `str` at 0x276f8 | store | 4 | `[sp-20] +0x7c` | `0x26624` |
| `ldr` at 0x276fc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27704 | store | 4 | `[sp-20] +0x68` | `0x1e` |
| `ldr` at 0x27708 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27710 | store | 4 | `[sp-20] +0x6c` | `0x2d` |
| `ldr` at 0x27714 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2771c | store | 4 | `[sp-20] +0x70` | `0x32` |
| `ldr` at 0x27720 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27728 | store | 4 | `[sp-20] +0x74` | `0x37` |
| `ldr` at 0x2772c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27734 | store | 4 | `[sp-20] +0x78` | `0x17` |
| `ldr` at 0x27738 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27740 | store | 4 | `[sp-20] +0x8c` | `0x1` |
| `ldr` at 0x27744 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2774c | store | 4 | `[sp-20] +0x88` | `0x2` |
| `ldr` at 0x27750 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27754 | load | 4 | `abs 0x277e4` | `-` |
| `str` at 0x27758 | store | 4 | `[sp-20] +0x90` | `0x5fdfc` |
| `ldr` at 0x2775c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27760 | load | 4 | `abs 0x277e8` | `-` |
| `str` at 0x27764 | store | 4 | `[sp-20] +0x94` | `0x5fee4` |
| `ldr` at 0x27768 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2776c | load | 4 | `abs 0x277ec` | `-` |
| `str` at 0x27770 | store | 4 | `[sp-20] +0x98` | `0x5fe74` |
| `ldr` at 0x27774 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2777c | store | 4 | `[sp-20] +0xb4` | `0x1` |
| `ldr` at 0x27780 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27784 | load | 4 | `abs 0x277f0` | `-` |
| `str` at 0x27788 | store | 4 | `[sp-20] +0xc4` | `0x26330` |
| `ldr` at 0x2778c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27790 | load | 4 | `abs 0x277f4` | `-` |
| `str` at 0x27794 | store | 4 | `[sp-20] +0xc8` | `0x263e0` |
| `ldr` at 0x27798 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2779c | load | 4 | `abs 0x277f8` | `-` |
| `str` at 0x277a0 | store | 4 | `[sp-20] +0xd0` | `0x264a4` |
| `ldr` at 0x277a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x277a8 | load | 4 | `abs 0x277fc` | `-` |
| `str` at 0x277ac | store | 4 | `[sp-20] +0xd4` | `0x264f4` |
| `ldr` at 0x277b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x277b4 | load | 4 | `abs 0x27800` | `-` |
| `str` at 0x277b8 | store | 4 | `[sp-20] +0xec` | `0x26bf4` |
| `ldr` at 0x277bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x277c0 | load | 4 | `abs 0x27804` | `-` |
| `str` at 0x277c4 | store | 4 | `[sp-20] +0xf0` | `0x26b08` |
| `ldmdb` at 0x277c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x277c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x277c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x277c8 | `r0` | `r0` |

## `NL4R_BrickOpsInit`

- `NL4R_BrickOpsInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x29a70 to 0x29aac.
  Established. The non-stack word store writes `0x297fc` at entry `r0 +0x0` (`str` at 0x29a94). The
  call to `0x2992c` uses `[sp-20]`, entry r1, entry r0, `0x297fc` (`bl` at 0x29a9c); its result
  reaches the exit as `ret(0x2992c)` (`ldmdb` at 0x29aa0). The body alone does not establish the
  stored address hardware role.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x29a80 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x29a84 | load | 4 | `abs 0x29aa4` | `-` |
| `str` at 0x29a88 | store | 4 | `sp -0x18` | `0x1424e4` |
| `ldr` at 0x29a8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29a90 | load | 4 | `abs 0x29aa8` | `-` |
| `str` at 0x29a94 | store | 4 | `r0 +0x0` | `0x297fc` |
| `ldr` at 0x29a98 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x29aa0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x29aa0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x29aa0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x29a9c | `(unknown callee) 0x2992c` | `[sp-20]` | `r1` | `r0` | `0x297fc` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x29aa0 | `r0` | `ret(0x2992c)` |

## `NL4S_BrickOpsInit`

- `NL4S_BrickOpsInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x29aac to 0x29ad8.
  Established. The single call reaches `0x2992c` with entry r0, entry r1, entry r2, `0x1424ea` (`bl`
  at 0x29acc). The exit propagates `ret(0x2992c)` (`ldmdb` at 0x29ad0). No conditional branch
  appears in the facts. Board personality and the callee effects remain unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x29abc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x29ac0 | load | 4 | `abs 0x29ad4` | `-` |
| `str` at 0x29ac4 | store | 4 | `sp -0x18` | `0x1424ea` |
| `ldr` at 0x29ac8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x29ad0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x29ad0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x29ad0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x29acc | `(unknown callee) 0x2992c` | `r0` | `r1` | `r2` | `0x1424ea` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x29ad0 | `r0` | `ret(0x2992c)` |

## `PA_BrickOpsInit`

- `PA_BrickOpsInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2a528 to 0x2a698.
  Established. The word write at entry `r0 +0x0` installs `0x2a2d8` (`str` at 0x2a54c). Later word
  stores write `0x53c18` at `[sp-20] +0xf0` and `0x29d10` at `[sp-20] +0x7c` (`str` at 0x2a660,
  `str` at 0x2a66c). The return retains entry r0 (`ldmdb` at 0x2a670). There are no call or
  conditional-branch facts. The values do not establish physical peripheral initialization.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2a538 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2a53c | load | 4 | `abs 0x2a674` | `-` |
| `str` at 0x2a540 | store | 4 | `sp -0x18` | `0x142520` |
| `ldr` at 0x2a544 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a548 | load | 4 | `abs 0x2a678` | `-` |
| `str` at 0x2a54c | store | 4 | `r0 +0x0` | `0x2a2d8` |
| `ldr` at 0x2a550 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a554 | load | 4 | `abs 0x2a67c` | `-` |
| `str` at 0x2a558 | store | 4 | `[sp-20] +0x4` | `0x2a0dc` |
| `ldr` at 0x2a55c | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x2a564 | store | 1 | `[sp-20] +0xc` | `0x0` |
| `ldr` at 0x2a568 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a570 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x2a574 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a57c | store | 4 | `[sp-20] +0xa0` | `0xff` |
| `ldr` at 0x2a580 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a588 | store | 4 | `[sp-20] +0xa4` | `0xff` |
| `ldr` at 0x2a58c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a594 | store | 4 | `[sp-20] +0xa8` | `0xff` |
| `ldr` at 0x2a598 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a5a0 | store | 4 | `[sp-20] +0xac` | `0xff` |
| `ldr` at 0x2a5a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a5a8 | load | 4 | `abs 0x2a680` | `-` |
| `str` at 0x2a5ac | store | 4 | `[sp-20] +0x14` | `0x102350` |
| `ldr` at 0x2a5b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a5b8 | store | 4 | `[sp-20] +0x68` | `0x1e` |
| `ldr` at 0x2a5bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a5c4 | store | 4 | `[sp-20] +0x6c` | `0x23` |
| `ldr` at 0x2a5c8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a5d0 | store | 4 | `[sp-20] +0x70` | `0x28` |
| `ldr` at 0x2a5d4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a5dc | store | 4 | `[sp-20] +0x74` | `0x2d` |
| `ldr` at 0x2a5e0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a5e8 | store | 4 | `[sp-20] +0x78` | `0x1b` |
| `ldr` at 0x2a5ec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a5f4 | store | 4 | `[sp-20] +0x88` | `0x0` |
| `ldr` at 0x2a5f8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a600 | store | 4 | `[sp-20] +0x90` | `0x0` |
| `ldr` at 0x2a604 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a60c | store | 4 | `[sp-20] +0x94` | `0x0` |
| `ldr` at 0x2a610 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a618 | store | 4 | `[sp-20] +0x98` | `0x0` |
| `ldr` at 0x2a61c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a620 | load | 4 | `abs 0x2a684` | `-` |
| `str` at 0x2a624 | store | 4 | `[sp-20] +0xc4` | `0x53708` |
| `ldr` at 0x2a628 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a62c | load | 4 | `abs 0x2a688` | `-` |
| `str` at 0x2a630 | store | 4 | `[sp-20] +0xc8` | `0x53914` |
| `ldr` at 0x2a634 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a63c | store | 4 | `[sp-20] +0xd0` | `0x0` |
| `ldr` at 0x2a640 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a648 | store | 4 | `[sp-20] +0xd4` | `0x0` |
| `ldr` at 0x2a64c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a650 | load | 4 | `abs 0x2a68c` | `-` |
| `str` at 0x2a654 | store | 4 | `[sp-20] +0xec` | `0x2a31c` |
| `ldr` at 0x2a658 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a65c | load | 4 | `abs 0x2a690` | `-` |
| `str` at 0x2a660 | store | 4 | `[sp-20] +0xf0` | `0x53c18` |
| `ldr` at 0x2a664 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a668 | load | 4 | `abs 0x2a694` | `-` |
| `str` at 0x2a66c | store | 4 | `[sp-20] +0x7c` | `0x29d10` |
| `ldmdb` at 0x2a670 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2a670 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2a670 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2a670 | `r0` | `r0` |

## `OpusTio_BrickOpsInit`

- `OpusTio_BrickOpsInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2beb0 to 0x2c05c.
  Established. The initial non-stack word store writes `0x2be58` at entry `r0 +0x0` (`str` at
  0x2bed4). The byte write at `[sp-20] +0xc` carries `0x52` (`strb` at 0x2beec). Two later word
  stores place `0x2b394` at `[sp-20] +0xec` and `0x2b2a8` at `[sp-20] +0xf0` (`str` at 0x2c00c,
  `str` at 0x2c018). The exit keeps entry r0 (`ldmdb` at 0x2c01c). TIO callback identities and field
  meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2bec0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2bec4 | load | 4 | `abs 0x2c020` | `-` |
| `str` at 0x2bec8 | store | 4 | `sp -0x18` | `0x14259c` |
| `ldr` at 0x2becc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2bed0 | load | 4 | `abs 0x2c024` | `-` |
| `str` at 0x2bed4 | store | 4 | `r0 +0x0` | `0x2be58` |
| `ldr` at 0x2bed8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2bedc | load | 4 | `abs 0x2c028` | `-` |
| `str` at 0x2bee0 | store | 4 | `[sp-20] +0x4` | `0x2bbb0` |
| `ldr` at 0x2bee4 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x2beec | store | 1 | `[sp-20] +0xc` | `0x52` |
| `ldr` at 0x2bef0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2bef8 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x2befc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2bf04 | store | 4 | `[sp-20] +0xa0` | `0x11` |
| `ldr` at 0x2bf08 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2bf10 | store | 4 | `[sp-20] +0xa4` | `0xff` |
| `ldr` at 0x2bf14 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2bf1c | store | 4 | `[sp-20] +0xa8` | `0xff` |
| `ldr` at 0x2bf20 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2bf28 | store | 4 | `[sp-20] +0xac` | `0xff` |
| `ldr` at 0x2bf2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2bf30 | load | 4 | `abs 0x2c02c` | `-` |
| `str` at 0x2bf34 | store | 4 | `[sp-20] +0x14` | `0x102e70` |
| `ldr` at 0x2bf38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2bf3c | load | 4 | `abs 0x2c030` | `-` |
| `str` at 0x2bf40 | store | 4 | `[sp-20] +0xb0` | `0x10e394` |
| `ldr` at 0x2bf44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2bf48 | load | 4 | `abs 0x2c034` | `-` |
| `str` at 0x2bf4c | store | 4 | `[sp-20] +0x7c` | `0x2adc4` |
| `ldr` at 0x2bf50 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2bf58 | store | 4 | `[sp-20] +0x68` | `0x1e` |
| `ldr` at 0x2bf5c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2bf64 | store | 4 | `[sp-20] +0x6c` | `0x2d` |
| `ldr` at 0x2bf68 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2bf70 | store | 4 | `[sp-20] +0x70` | `0x32` |
| `ldr` at 0x2bf74 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2bf7c | store | 4 | `[sp-20] +0x74` | `0x37` |
| `ldr` at 0x2bf80 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2bf88 | store | 4 | `[sp-20] +0x78` | `0x17` |
| `ldr` at 0x2bf8c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2bf94 | store | 4 | `[sp-20] +0x8c` | `0x1` |
| `ldr` at 0x2bf98 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2bfa0 | store | 4 | `[sp-20] +0x88` | `0x2` |
| `ldr` at 0x2bfa4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2bfa8 | load | 4 | `abs 0x2c038` | `-` |
| `str` at 0x2bfac | store | 4 | `[sp-20] +0x90` | `0x5fdfc` |
| `ldr` at 0x2bfb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2bfb4 | load | 4 | `abs 0x2c03c` | `-` |
| `str` at 0x2bfb8 | store | 4 | `[sp-20] +0x94` | `0x5fee4` |
| `ldr` at 0x2bfbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2bfc0 | load | 4 | `abs 0x2c040` | `-` |
| `str` at 0x2bfc4 | store | 4 | `[sp-20] +0x98` | `0x5fe74` |
| `ldr` at 0x2bfc8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2bfd0 | store | 4 | `[sp-20] +0xb4` | `0x1` |
| `ldr` at 0x2bfd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2bfd8 | load | 4 | `abs 0x2c044` | `-` |
| `str` at 0x2bfdc | store | 4 | `[sp-20] +0xc4` | `0x2aacc` |
| `ldr` at 0x2bfe0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2bfe4 | load | 4 | `abs 0x2c048` | `-` |
| `str` at 0x2bfe8 | store | 4 | `[sp-20] +0xc8` | `0x2ab88` |
| `ldr` at 0x2bfec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2bff0 | load | 4 | `abs 0x2c04c` | `-` |
| `str` at 0x2bff4 | store | 4 | `[sp-20] +0xd0` | `0x2ac40` |
| `ldr` at 0x2bff8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2bffc | load | 4 | `abs 0x2c050` | `-` |
| `str` at 0x2c000 | store | 4 | `[sp-20] +0xd4` | `0x2ac90` |
| `ldr` at 0x2c004 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2c008 | load | 4 | `abs 0x2c054` | `-` |
| `str` at 0x2c00c | store | 4 | `[sp-20] +0xec` | `0x2b394` |
| `ldr` at 0x2c010 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2c014 | load | 4 | `abs 0x2c058` | `-` |
| `str` at 0x2c018 | store | 4 | `[sp-20] +0xf0` | `0x2b2a8` |
| `ldmdb` at 0x2c01c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2c01c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2c01c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2c01c | `r0` | `r0` |

## `TIOTest_BrickOpsInit`

- `TIOTest_BrickOpsInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2cf94 to 0x2d0dc.
  Established. Word stores include `0x2cd78` at entry `r0 +0x0`, `0x5` at `[sp-20] +0x1c` and `0x8`
  at `[sp-20] +0x20` (`str` at 0x2cfb8, `str` at 0x2d0a8, `str` at 0x2d0b4). The byte write at
  `[sp-20] +0xc` is `0x0` (`strb` at 0x2cfd0); its exit retains entry r0 (`ldmdb` at 0x2d0b8).
  Test-board field semantics and hardware effects are unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2cfa4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2cfa8 | load | 4 | `abs 0x2d0bc` | `-` |
| `str` at 0x2cfac | store | 4 | `sp -0x18` | `0x1425ec` |
| `ldr` at 0x2cfb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2cfb4 | load | 4 | `abs 0x2d0c0` | `-` |
| `str` at 0x2cfb8 | store | 4 | `r0 +0x0` | `0x2cd78` |
| `ldr` at 0x2cfbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2cfc0 | load | 4 | `abs 0x2d0c4` | `-` |
| `str` at 0x2cfc4 | store | 4 | `[sp-20] +0x4` | `0x2c954` |
| `ldr` at 0x2cfc8 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x2cfd0 | store | 1 | `[sp-20] +0xc` | `0x0` |
| `ldr` at 0x2cfd4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2cfdc | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x2cfe0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2cfe8 | store | 4 | `[sp-20] +0xa0` | `0xff` |
| `ldr` at 0x2cfec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2cff4 | store | 4 | `[sp-20] +0xa4` | `0xff` |
| `ldr` at 0x2cff8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d000 | store | 4 | `[sp-20] +0xa8` | `0xff` |
| `ldr` at 0x2d004 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d00c | store | 4 | `[sp-20] +0xac` | `0xff` |
| `ldr` at 0x2d010 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d014 | load | 4 | `abs 0x2d0c8` | `-` |
| `str` at 0x2d018 | store | 4 | `[sp-20] +0x14` | `0x103954` |
| `ldr` at 0x2d01c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d024 | store | 4 | `[sp-20] +0x88` | `0x0` |
| `ldr` at 0x2d028 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d030 | store | 4 | `[sp-20] +0x90` | `0x0` |
| `ldr` at 0x2d034 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d03c | store | 4 | `[sp-20] +0x94` | `0x0` |
| `ldr` at 0x2d040 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d048 | store | 4 | `[sp-20] +0x98` | `0x0` |
| `ldr` at 0x2d04c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d050 | load | 4 | `abs 0x2d0cc` | `-` |
| `str` at 0x2d054 | store | 4 | `[sp-20] +0xc4` | `0x53708` |
| `ldr` at 0x2d058 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d05c | load | 4 | `abs 0x2d0d0` | `-` |
| `str` at 0x2d060 | store | 4 | `[sp-20] +0xc8` | `0x53914` |
| `ldr` at 0x2d064 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d06c | store | 4 | `[sp-20] +0xd0` | `0x0` |
| `ldr` at 0x2d070 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d078 | store | 4 | `[sp-20] +0xd4` | `0x0` |
| `ldr` at 0x2d07c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d080 | load | 4 | `abs 0x2d0d4` | `-` |
| `str` at 0x2d084 | store | 4 | `[sp-20] +0xec` | `0x2c3b4` |
| `ldr` at 0x2d088 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d08c | load | 4 | `abs 0x2d0d8` | `-` |
| `str` at 0x2d090 | store | 4 | `[sp-20] +0xf0` | `0x53c18` |
| `ldr` at 0x2d094 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d09c | store | 4 | `[sp-20] +0x18` | `0x5` |
| `ldr` at 0x2d0a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d0a8 | store | 4 | `[sp-20] +0x1c` | `0x5` |
| `ldr` at 0x2d0ac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d0b4 | store | 4 | `[sp-20] +0x20` | `0x8` |
| `ldmdb` at 0x2d0b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2d0b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2d0b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2d0b8 | `r0` | `r0` |

## `Athena_BrickOpsInit`

- `Athena_BrickOpsInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2d0dc to 0x2d260.
  Established. The word writes include `0x2cd78` at entry `r0 +0x0` and `0x2cb68` at `[sp-20] +0x4`
  (`str` at 0x2d100, `str` at 0x2d10c). Later word stores clear `[sp-20] +0x1c` and `[sp-20] +0x20`
  (`str` at 0x2d22c, `str` at 0x2d238). The return carries entry r0 (`ldmdb` at 0x2d23c).
  Board-specific field meanings are not established by these constants.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2d0ec | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2d0f0 | load | 4 | `abs 0x2d240` | `-` |
| `str` at 0x2d0f4 | store | 4 | `sp -0x18` | `0x1425f2` |
| `ldr` at 0x2d0f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d0fc | load | 4 | `abs 0x2d244` | `-` |
| `str` at 0x2d100 | store | 4 | `r0 +0x0` | `0x2cd78` |
| `ldr` at 0x2d104 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d108 | load | 4 | `abs 0x2d248` | `-` |
| `str` at 0x2d10c | store | 4 | `[sp-20] +0x4` | `0x2cb68` |
| `ldr` at 0x2d110 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x2d118 | store | 1 | `[sp-20] +0xc` | `0x0` |
| `ldr` at 0x2d11c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d124 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x2d128 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d130 | store | 4 | `[sp-20] +0xa0` | `0xff` |
| `ldr` at 0x2d134 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d13c | store | 4 | `[sp-20] +0xa4` | `0xff` |
| `ldr` at 0x2d140 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d148 | store | 4 | `[sp-20] +0xa8` | `0xff` |
| `ldr` at 0x2d14c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d154 | store | 4 | `[sp-20] +0xac` | `0xff` |
| `ldr` at 0x2d158 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d15c | load | 4 | `abs 0x2d24c` | `-` |
| `str` at 0x2d160 | store | 4 | `[sp-20] +0x14` | `0x103954` |
| `ldr` at 0x2d164 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d16c | store | 4 | `[sp-20] +0x88` | `0x0` |
| `ldr` at 0x2d170 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d178 | store | 4 | `[sp-20] +0x90` | `0x0` |
| `ldr` at 0x2d17c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d184 | store | 4 | `[sp-20] +0x94` | `0x0` |
| `ldr` at 0x2d188 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d190 | store | 4 | `[sp-20] +0x98` | `0x0` |
| `ldr` at 0x2d194 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d198 | load | 4 | `abs 0x2d250` | `-` |
| `str` at 0x2d19c | store | 4 | `[sp-20] +0xc4` | `0x53708` |
| `ldr` at 0x2d1a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d1a4 | load | 4 | `abs 0x2d254` | `-` |
| `str` at 0x2d1a8 | store | 4 | `[sp-20] +0xc8` | `0x53914` |
| `ldr` at 0x2d1ac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d1b4 | store | 4 | `[sp-20] +0xd0` | `0x0` |
| `ldr` at 0x2d1b8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d1c0 | store | 4 | `[sp-20] +0xd4` | `0x0` |
| `ldr` at 0x2d1c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d1c8 | load | 4 | `abs 0x2d258` | `-` |
| `str` at 0x2d1cc | store | 4 | `[sp-20] +0xec` | `0x2c59c` |
| `ldr` at 0x2d1d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d1d4 | load | 4 | `abs 0x2d25c` | `-` |
| `str` at 0x2d1d8 | store | 4 | `[sp-20] +0xf0` | `0x53c18` |
| `ldr` at 0x2d1dc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d1e4 | store | 4 | `[sp-20] +0x68` | `0x2d` |
| `ldr` at 0x2d1e8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d1f0 | store | 4 | `[sp-20] +0x6c` | `0x32` |
| `ldr` at 0x2d1f4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d1fc | store | 4 | `[sp-20] +0x70` | `0x37` |
| `ldr` at 0x2d200 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d208 | store | 4 | `[sp-20] +0x74` | `0x3c` |
| `ldr` at 0x2d20c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d214 | store | 4 | `[sp-20] +0x78` | `0x2b` |
| `ldr` at 0x2d218 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d220 | store | 4 | `[sp-20] +0x18` | `0x0` |
| `ldr` at 0x2d224 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d22c | store | 4 | `[sp-20] +0x1c` | `0x0` |
| `ldr` at 0x2d230 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d238 | store | 4 | `[sp-20] +0x20` | `0x0` |
| `ldmdb` at 0x2d23c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2d23c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2d23c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2d23c | `r0` | `r0` |

## `Dorado_BrickOpsInit`

- `Dorado_BrickOpsInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2ee88 to 0x2f094.
  Established. The first non-stack word store places `0x2ee30` at entry `r0 +0x0` (`str` at
  0x2eeac). A byte store writes `0x52` at `[sp-20] +0xc` (`strb` at 0x2eec4). Final word writes
  install `0x2e400` at `[sp-20] +0xec` and `0x2de30` at `[sp-20] +0xf0` (`str` at 0x2f02c, `str` at
  0x2f038). The exit retains entry r0 (`ldmdb` at 0x2f03c). Stored-address meanings and hardware
  roles remain unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2ee98 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2ee9c | load | 4 | `abs 0x2f040` | `-` |
| `str` at 0x2eea0 | store | 4 | `sp -0x18` | `0x142624` |
| `ldr` at 0x2eea4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2eea8 | load | 4 | `abs 0x2f044` | `-` |
| `str` at 0x2eeac | store | 4 | `r0 +0x0` | `0x2ee30` |
| `ldr` at 0x2eeb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2eeb4 | load | 4 | `abs 0x2f048` | `-` |
| `str` at 0x2eeb8 | store | 4 | `[sp-20] +0x4` | `0x2ebb0` |
| `ldr` at 0x2eebc | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x2eec4 | store | 1 | `[sp-20] +0xc` | `0x52` |
| `ldr` at 0x2eec8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2eed0 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x2eed4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2eedc | store | 4 | `[sp-20] +0xa0` | `0x13` |
| `ldr` at 0x2eee0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2eee8 | store | 4 | `[sp-20] +0xa4` | `0xff` |
| `ldr` at 0x2eeec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2eef4 | store | 4 | `[sp-20] +0xa8` | `0xff` |
| `ldr` at 0x2eef8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ef00 | store | 4 | `[sp-20] +0xac` | `0xff` |
| `ldr` at 0x2ef04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2ef08 | load | 4 | `abs 0x2f04c` | `-` |
| `str` at 0x2ef0c | store | 4 | `[sp-20] +0x14` | `0x104658` |
| `ldr` at 0x2ef10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2ef14 | load | 4 | `abs 0x2f050` | `-` |
| `str` at 0x2ef18 | store | 4 | `[sp-20] +0xb0` | `0x10e394` |
| `ldr` at 0x2ef1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2ef20 | load | 4 | `abs 0x2f054` | `-` |
| `str` at 0x2ef24 | store | 4 | `[sp-20] +0x7c` | `0x2d938` |
| `ldr` at 0x2ef28 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ef30 | store | 4 | `[sp-20] +0x68` | `0x47` |
| `ldr` at 0x2ef34 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ef3c | store | 4 | `[sp-20] +0x6c` | `0x48` |
| `ldr` at 0x2ef40 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ef48 | store | 4 | `[sp-20] +0x70` | `0x49` |
| `ldr` at 0x2ef4c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ef54 | store | 4 | `[sp-20] +0x74` | `0x4b` |
| `ldr` at 0x2ef58 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ef60 | store | 4 | `[sp-20] +0x78` | `0x45` |
| `ldr` at 0x2ef64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2ef68 | load | 4 | `abs 0x2f058` | `-` |
| `str` at 0x2ef6c | store | 4 | `[sp-20] +0xfc` | `0x32a6c` |
| `ldr` at 0x2ef70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2ef74 | load | 4 | `abs 0x2f05c` | `-` |
| `str` at 0x2ef78 | store | 4 | `[sp-20] +0x80` | `0x322d0` |
| `ldr` at 0x2ef7c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ef84 | store | 4 | `[sp-20] +0x8c` | `0x1` |
| `ldr` at 0x2ef88 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ef90 | store | 4 | `[sp-20] +0x88` | `0x2` |
| `ldr` at 0x2ef94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2ef98 | load | 4 | `abs 0x2f060` | `-` |
| `str` at 0x2ef9c | store | 4 | `[sp-20] +0x90` | `0x5fdfc` |
| `ldr` at 0x2efa0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2efa4 | load | 4 | `abs 0x2f064` | `-` |
| `str` at 0x2efa8 | store | 4 | `[sp-20] +0x94` | `0x5fee4` |
| `ldr` at 0x2efac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2efb0 | load | 4 | `abs 0x2f068` | `-` |
| `str` at 0x2efb4 | store | 4 | `[sp-20] +0x98` | `0x5fe74` |
| `ldr` at 0x2efb8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2efc0 | store | 4 | `[sp-20] +0xb4` | `0x1` |
| `ldr` at 0x2efc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2efc8 | load | 4 | `abs 0x2f06c` | `-` |
| `str` at 0x2efcc | store | 4 | `[sp-20] +0xb8` | `0x2d764` |
| `ldr` at 0x2efd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2efd4 | load | 4 | `abs 0x2f070` | `-` |
| `str` at 0x2efd8 | store | 4 | `[sp-20] +0xbc` | `0x2d7b4` |
| `ldr` at 0x2efdc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2efe0 | load | 4 | `abs 0x2f074` | `-` |
| `str` at 0x2efe4 | store | 4 | `[sp-20] +0xc0` | `0x2d804` |
| `ldr` at 0x2efe8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2efec | load | 4 | `abs 0x2f078` | `-` |
| `str` at 0x2eff0 | store | 4 | `[sp-20] +0xc4` | `0x2d41c` |
| `ldr` at 0x2eff4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2eff8 | load | 4 | `abs 0x2f07c` | `-` |
| `str` at 0x2effc | store | 4 | `[sp-20] +0xc8` | `0x2d4c4` |
| `ldr` at 0x2f000 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2f004 | load | 4 | `abs 0x2f080` | `-` |
| `str` at 0x2f008 | store | 4 | `[sp-20] +0xcc` | `0x2d568` |
| `ldr` at 0x2f00c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2f010 | load | 4 | `abs 0x2f084` | `-` |
| `str` at 0x2f014 | store | 4 | `[sp-20] +0xd0` | `0x2d6c4` |
| `ldr` at 0x2f018 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2f01c | load | 4 | `abs 0x2f088` | `-` |
| `str` at 0x2f020 | store | 4 | `[sp-20] +0xd4` | `0x2d714` |
| `ldr` at 0x2f024 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2f028 | load | 4 | `abs 0x2f08c` | `-` |
| `str` at 0x2f02c | store | 4 | `[sp-20] +0xec` | `0x2e400` |
| `ldr` at 0x2f030 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2f034 | load | 4 | `abs 0x2f090` | `-` |
| `str` at 0x2f038 | store | 4 | `[sp-20] +0xf0` | `0x2de30` |
| `ldmdb` at 0x2f03c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2f03c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2f03c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2f03c | `r0` | `r0` |

## `OpusIP65_BrickOpsInit`

- `OpusIP65_BrickOpsInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3074c to 0x308f8.
  Established. The word store at entry `r0 +0x0` writes `0x306f4` (`str` at 0x30770). The byte write
  at `[sp-20] +0xc` carries `0x52` (`strb` at 0x30788). Word stores near the exit write `0x2fcb4` at
  `[sp-20] +0xec` and `0x2fb54` at `[sp-20] +0xf0` (`str` at 0x308a8, `str` at 0x308b4). Entry r0
  reaches the return (`ldmdb` at 0x308b8). IP65 callback effects and board-field interpretation are
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3075c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x30760 | load | 4 | `abs 0x308bc` | `-` |
| `str` at 0x30764 | store | 4 | `sp -0x18` | `0x1426a0` |
| `ldr` at 0x30768 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3076c | load | 4 | `abs 0x308c0` | `-` |
| `str` at 0x30770 | store | 4 | `r0 +0x0` | `0x306f4` |
| `ldr` at 0x30774 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x30778 | load | 4 | `abs 0x308c4` | `-` |
| `str` at 0x3077c | store | 4 | `[sp-20] +0x4` | `0x3046c` |
| `ldr` at 0x30780 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x30788 | store | 1 | `[sp-20] +0xc` | `0x52` |
| `ldr` at 0x3078c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x30794 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x30798 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x307a0 | store | 4 | `[sp-20] +0xa0` | `0xff` |
| `ldr` at 0x307a4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x307ac | store | 4 | `[sp-20] +0xa4` | `0xff` |
| `ldr` at 0x307b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x307b8 | store | 4 | `[sp-20] +0xa8` | `0xff` |
| `ldr` at 0x307bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x307c4 | store | 4 | `[sp-20] +0xac` | `0xff` |
| `ldr` at 0x307c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x307cc | load | 4 | `abs 0x308c8` | `-` |
| `str` at 0x307d0 | store | 4 | `[sp-20] +0x14` | `0x105038` |
| `ldr` at 0x307d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x307d8 | load | 4 | `abs 0x308cc` | `-` |
| `str` at 0x307dc | store | 4 | `[sp-20] +0xb0` | `0x10e394` |
| `ldr` at 0x307e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x307e4 | load | 4 | `abs 0x308d0` | `-` |
| `str` at 0x307e8 | store | 4 | `[sp-20] +0x7c` | `0x2f670` |
| `ldr` at 0x307ec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x307f4 | store | 4 | `[sp-20] +0x68` | `0x1e` |
| `ldr` at 0x307f8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x30800 | store | 4 | `[sp-20] +0x6c` | `0x2d` |
| `ldr` at 0x30804 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3080c | store | 4 | `[sp-20] +0x70` | `0x32` |
| `ldr` at 0x30810 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x30818 | store | 4 | `[sp-20] +0x74` | `0x37` |
| `ldr` at 0x3081c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x30824 | store | 4 | `[sp-20] +0x78` | `0x17` |
| `ldr` at 0x30828 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x30830 | store | 4 | `[sp-20] +0x8c` | `0x1` |
| `ldr` at 0x30834 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3083c | store | 4 | `[sp-20] +0x88` | `0x1` |
| `ldr` at 0x30840 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x30844 | load | 4 | `abs 0x308d4` | `-` |
| `str` at 0x30848 | store | 4 | `[sp-20] +0x90` | `0x64474` |
| `ldr` at 0x3084c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x30850 | load | 4 | `abs 0x308d8` | `-` |
| `str` at 0x30854 | store | 4 | `[sp-20] +0x94` | `0x64524` |
| `ldr` at 0x30858 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3085c | load | 4 | `abs 0x308dc` | `-` |
| `str` at 0x30860 | store | 4 | `[sp-20] +0x98` | `0x644c8` |
| `ldr` at 0x30864 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3086c | store | 4 | `[sp-20] +0xb4` | `0x1` |
| `ldr` at 0x30870 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x30874 | load | 4 | `abs 0x308e0` | `-` |
| `str` at 0x30878 | store | 4 | `[sp-20] +0xc4` | `0x2f45c` |
| `ldr` at 0x3087c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x30880 | load | 4 | `abs 0x308e4` | `-` |
| `str` at 0x30884 | store | 4 | `[sp-20] +0xc8` | `0x2f518` |
| `ldr` at 0x30888 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3088c | load | 4 | `abs 0x308e8` | `-` |
| `str` at 0x30890 | store | 4 | `[sp-20] +0xd0` | `0x2f5d0` |
| `ldr` at 0x30894 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x30898 | load | 4 | `abs 0x308ec` | `-` |
| `str` at 0x3089c | store | 4 | `[sp-20] +0xd4` | `0x2f620` |
| `ldr` at 0x308a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x308a4 | load | 4 | `abs 0x308f0` | `-` |
| `str` at 0x308a8 | store | 4 | `[sp-20] +0xec` | `0x2fcb4` |
| `ldr` at 0x308ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x308b0 | load | 4 | `abs 0x308f4` | `-` |
| `str` at 0x308b4 | store | 4 | `[sp-20] +0xf0` | `0x2fb54` |
| `ldmdb` at 0x308b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x308b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x308b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x308b8 | `r0` | `r0` |

## `IP67_BrickOpsInit`

- `IP67_BrickOpsInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x33e0c to 0x33fe0.
  Established. The byte store writes `0x54` at `[sp-20] +0xc` (`strb` at 0x33e48). The only call
  targets `0x1b304` with `0x2ae518`, `0x105444`, `0x10`, `0x33408` (`bl` at 0x33ec4). Later word
  writes install `0x33890` at `[sp-20] +0xec` and `0x33764` at `[sp-20] +0xf0` (`str` at 0x33f84,
  `str` at 0x33f90). The return propagates `ret(0x1b304)` (`ldmdb` at 0x33f94). Board-record and
  numeric callee meanings remain unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x33e1c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x33e20 | load | 4 | `abs 0x33f98` | `-` |
| `str` at 0x33e24 | store | 4 | `sp -0x18` | `0x14270c` |
| `ldr` at 0x33e28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x33e2c | load | 4 | `abs 0x33f9c` | `-` |
| `str` at 0x33e30 | store | 4 | `r0 +0x0` | `0x33bf4` |
| `ldr` at 0x33e34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x33e38 | load | 4 | `abs 0x33fa0` | `-` |
| `str` at 0x33e3c | store | 4 | `[sp-20] +0x4` | `0x33c3c` |
| `ldr` at 0x33e40 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x33e48 | store | 1 | `[sp-20] +0xc` | `0x54` |
| `ldr` at 0x33e4c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x33e54 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x33e58 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x33e60 | store | 4 | `[sp-20] +0xa0` | `0x9` |
| `ldr` at 0x33e64 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x33e6c | store | 4 | `[sp-20] +0xa4` | `0xa` |
| `ldr` at 0x33e70 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x33e78 | store | 4 | `[sp-20] +0xa8` | `0xff` |
| `ldr` at 0x33e7c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x33e84 | store | 4 | `[sp-20] +0xac` | `0xff` |
| `ldr` at 0x33e88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x33e8c | load | 4 | `abs 0x33fa4` | `-` |
| `str` at 0x33e90 | store | 4 | `[sp-20] +0x14` | `0x105b10` |
| `ldr` at 0x33e94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x33e98 | load | 4 | `abs 0x33fa8` | `-` |
| `str` at 0x33e9c | store | 4 | `[sp-20] +0xb0` | `0x10e394` |
| `ldr` at 0x33ea0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x33ea4 | load | 4 | `abs 0x33fac` | `-` |
| `str` at 0x33ea8 | store | 4 | `[sp-20] +0x84` | `0x5ec98` |
| `ldr` at 0x33eac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x33eb0 | load | 4 | `abs 0x33fb0` | `-` |
| `str` at 0x33eb4 | store | 4 | `[sp-20] +0x7c` | `0x33408` |
| `ldr` at 0x33eb8 | load | 4 | `abs 0x33fb4` | `-` |
| `ldr` at 0x33ebc | load | 4 | `abs 0x33fb8` | `-` |
| `ldr` at 0x33ec8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x33ed0 | store | 4 | `[sp-20] +0x68` | `0x1e` |
| `ldr` at 0x33ed4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x33edc | store | 4 | `[sp-20] +0x6c` | `0x2d` |
| `ldr` at 0x33ee0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x33ee8 | store | 4 | `[sp-20] +0x70` | `0x32` |
| `ldr` at 0x33eec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x33ef4 | store | 4 | `[sp-20] +0x74` | `0x37` |
| `ldr` at 0x33ef8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x33f00 | store | 4 | `[sp-20] +0x78` | `0x17` |
| `ldr` at 0x33f04 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x33f0c | store | 4 | `[sp-20] +0x8c` | `0x1` |
| `ldr` at 0x33f10 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x33f18 | store | 4 | `[sp-20] +0x88` | `0x2` |
| `ldr` at 0x33f1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x33f20 | load | 4 | `abs 0x33fbc` | `-` |
| `str` at 0x33f24 | store | 4 | `[sp-20] +0x90` | `0x5fdfc` |
| `ldr` at 0x33f28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x33f2c | load | 4 | `abs 0x33fc0` | `-` |
| `str` at 0x33f30 | store | 4 | `[sp-20] +0x94` | `0x5fee4` |
| `ldr` at 0x33f34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x33f38 | load | 4 | `abs 0x33fc4` | `-` |
| `str` at 0x33f3c | store | 4 | `[sp-20] +0x98` | `0x5fe74` |
| `ldr` at 0x33f40 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x33f48 | store | 4 | `[sp-20] +0xb4` | `0x1` |
| `ldr` at 0x33f4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x33f50 | load | 4 | `abs 0x33fc8` | `-` |
| `str` at 0x33f54 | store | 4 | `[sp-20] +0xc4` | `0x330d8` |
| `ldr` at 0x33f58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x33f5c | load | 4 | `abs 0x33fcc` | `-` |
| `str` at 0x33f60 | store | 4 | `[sp-20] +0xc8` | `0x33160` |
| `ldr` at 0x33f64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x33f68 | load | 4 | `abs 0x33fd0` | `-` |
| `str` at 0x33f6c | store | 4 | `[sp-20] +0xd0` | `0x33284` |
| `ldr` at 0x33f70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x33f74 | load | 4 | `abs 0x33fd4` | `-` |
| `str` at 0x33f78 | store | 4 | `[sp-20] +0xd4` | `0x332d4` |
| `ldr` at 0x33f7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x33f80 | load | 4 | `abs 0x33fd8` | `-` |
| `str` at 0x33f84 | store | 4 | `[sp-20] +0xec` | `0x33890` |
| `ldr` at 0x33f88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x33f8c | load | 4 | `abs 0x33fdc` | `-` |
| `str` at 0x33f90 | store | 4 | `[sp-20] +0xf0` | `0x33764` |
| `ldmdb` at 0x33f94 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x33f94 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x33f94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x33ec4 | `(unknown callee) 0x1b304` | `0x2ae518` | `0x105444` | `0x10` | `0x33408` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x33f94 | `r0` | `ret(0x1b304)` |

## `Daytona_BrickOpsInit`

- `Daytona_BrickOpsInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x36058 to 0x361b8.
  Established. The word store at entry `r0 +0x0` writes `0x35a08`, while another at `[sp-20] +0x4`
  writes `0x357e0` (`str` at 0x3607c, `str` at 0x36088). The byte write at `[sp-20] +0xc` carries
  `0x0` (`strb` at 0x36094). Later word stores place `0x35a6c` at `[sp-20] +0xec` and `0x53c18` at
  `[sp-20] +0xf0` (`str` at 0x36184, `str` at 0x36190). The exit preserves entry r0 (`ldmdb` at
  0x36194). Callback behavior and physical board initialization remain unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x36068 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x3606c | load | 4 | `abs 0x36198` | `-` |
| `str` at 0x36070 | store | 4 | `sp -0x18` | `0x142750` |
| `ldr` at 0x36074 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x36078 | load | 4 | `abs 0x3619c` | `-` |
| `str` at 0x3607c | store | 4 | `r0 +0x0` | `0x35a08` |
| `ldr` at 0x36080 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x36084 | load | 4 | `abs 0x361a0` | `-` |
| `str` at 0x36088 | store | 4 | `[sp-20] +0x4` | `0x357e0` |
| `ldr` at 0x3608c | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x36094 | store | 1 | `[sp-20] +0xc` | `0x0` |
| `ldr` at 0x36098 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x360a0 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x360a4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x360ac | store | 4 | `[sp-20] +0xa0` | `0xff` |
| `ldr` at 0x360b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x360b8 | store | 4 | `[sp-20] +0xa4` | `0xff` |
| `ldr` at 0x360bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x360c4 | store | 4 | `[sp-20] +0xa8` | `0xff` |
| `ldr` at 0x360c8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x360d0 | store | 4 | `[sp-20] +0xac` | `0xff` |
| `ldr` at 0x360d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x360d8 | load | 4 | `abs 0x361a4` | `-` |
| `str` at 0x360dc | store | 4 | `[sp-20] +0x14` | `0x106a84` |
| `ldr` at 0x360e0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x360e8 | store | 4 | `[sp-20] +0x68` | `0x1e` |
| `ldr` at 0x360ec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x360f4 | store | 4 | `[sp-20] +0x6c` | `0x23` |
| `ldr` at 0x360f8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x36100 | store | 4 | `[sp-20] +0x70` | `0x28` |
| `ldr` at 0x36104 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3610c | store | 4 | `[sp-20] +0x74` | `0x2d` |
| `ldr` at 0x36110 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x36118 | store | 4 | `[sp-20] +0x78` | `0x1b` |
| `ldr` at 0x3611c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x36124 | store | 4 | `[sp-20] +0x88` | `0x0` |
| `ldr` at 0x36128 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x36130 | store | 4 | `[sp-20] +0x90` | `0x0` |
| `ldr` at 0x36134 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3613c | store | 4 | `[sp-20] +0x94` | `0x0` |
| `ldr` at 0x36140 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x36148 | store | 4 | `[sp-20] +0x98` | `0x0` |
| `ldr` at 0x3614c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x36150 | load | 4 | `abs 0x361a8` | `-` |
| `str` at 0x36154 | store | 4 | `[sp-20] +0xc4` | `0x53708` |
| `ldr` at 0x36158 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3615c | load | 4 | `abs 0x361ac` | `-` |
| `str` at 0x36160 | store | 4 | `[sp-20] +0xc8` | `0x53914` |
| `ldr` at 0x36164 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3616c | store | 4 | `[sp-20] +0xd0` | `0x0` |
| `ldr` at 0x36170 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x36178 | store | 4 | `[sp-20] +0xd4` | `0x0` |
| `ldr` at 0x3617c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x36180 | load | 4 | `abs 0x361b0` | `-` |
| `str` at 0x36184 | store | 4 | `[sp-20] +0xec` | `0x35a6c` |
| `ldr` at 0x36188 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3618c | load | 4 | `abs 0x361b4` | `-` |
| `str` at 0x36190 | store | 4 | `[sp-20] +0xf0` | `0x53c18` |
| `ldmdb` at 0x36194 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x36194 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x36194 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x36194 | `r0` | `r0` |

## `IP63_BrickOpsInit`

- `IP63_BrickOpsInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x382bc to 0x3844c.
  Established. The opening non-stack word writes carry `0x37bb4` at entry `r0 +0x0` and `0x379b8` at
  `[sp-20] +0x4` (`str` at 0x382e0, `str` at 0x382ec). Near the end, word stores place `0x37d74` at
  `[sp-20] +0xec` and `0x3b0f4` at `[sp-20] +0xf0` (`str` at 0x38400, `str` at 0x3840c). The return
  retains entry r0 (`ldmdb` at 0x38410). No calls or conditional branches appear in the facts. The
  values do not identify hardware initialization effects.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x382cc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x382d0 | load | 4 | `abs 0x38414` | `-` |
| `str` at 0x382d4 | store | 4 | `sp -0x18` | `0x142788` |
| `ldr` at 0x382d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x382dc | load | 4 | `abs 0x38418` | `-` |
| `str` at 0x382e0 | store | 4 | `r0 +0x0` | `0x37bb4` |
| `ldr` at 0x382e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x382e8 | load | 4 | `abs 0x3841c` | `-` |
| `str` at 0x382ec | store | 4 | `[sp-20] +0x4` | `0x379b8` |
| `ldr` at 0x382f0 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x382f8 | store | 1 | `[sp-20] +0xc` | `0x0` |
| `ldr` at 0x382fc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x38304 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x38308 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x38310 | store | 4 | `[sp-20] +0xa0` | `0xff` |
| `ldr` at 0x38314 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3831c | store | 4 | `[sp-20] +0xa4` | `0xff` |
| `ldr` at 0x38320 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x38328 | store | 4 | `[sp-20] +0xa8` | `0xff` |
| `ldr` at 0x3832c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x38334 | store | 4 | `[sp-20] +0xac` | `0xff` |
| `ldr` at 0x38338 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x38340 | store | 4 | `[sp-20] +0x68` | `0x1e` |
| `ldr` at 0x38344 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3834c | store | 4 | `[sp-20] +0x6c` | `0x23` |
| `ldr` at 0x38350 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x38358 | store | 4 | `[sp-20] +0x70` | `0x28` |
| `ldr` at 0x3835c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x38364 | store | 4 | `[sp-20] +0x74` | `0x2d` |
| `ldr` at 0x38368 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x38370 | store | 4 | `[sp-20] +0x78` | `0x1b` |
| `ldr` at 0x38374 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3837c | store | 4 | `[sp-20] +0xb4` | `0x1` |
| `ldr` at 0x38380 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x38388 | store | 4 | `[sp-20] +0x88` | `0x1` |
| `ldr` at 0x3838c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x38390 | load | 4 | `abs 0x38420` | `-` |
| `str` at 0x38394 | store | 4 | `[sp-20] +0x90` | `0x38134` |
| `ldr` at 0x38398 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3839c | load | 4 | `abs 0x38424` | `-` |
| `str` at 0x383a0 | store | 4 | `[sp-20] +0x94` | `0x38108` |
| `ldr` at 0x383a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x383a8 | load | 4 | `abs 0x38428` | `-` |
| `str` at 0x383ac | store | 4 | `[sp-20] +0x98` | `0x380dc` |
| `ldr` at 0x383b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x383b4 | load | 4 | `abs 0x3842c` | `-` |
| `str` at 0x383b8 | store | 4 | `[sp-20] +0x9c` | `0x38028` |
| `ldr` at 0x383bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x383c0 | load | 4 | `abs 0x38430` | `-` |
| `str` at 0x383c4 | store | 4 | `[sp-20] +0xb0` | `0x10e394` |
| `ldr` at 0x383c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x383cc | load | 4 | `abs 0x38434` | `-` |
| `str` at 0x383d0 | store | 4 | `[sp-20] +0xc4` | `0x37fd8` |
| `ldr` at 0x383d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x383d8 | load | 4 | `abs 0x38438` | `-` |
| `str` at 0x383dc | store | 4 | `[sp-20] +0xc8` | `0x38000` |
| `ldr` at 0x383e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x383e4 | load | 4 | `abs 0x3843c` | `-` |
| `str` at 0x383e8 | store | 4 | `[sp-20] +0xd0` | `0x3826c` |
| `ldr` at 0x383ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x383f0 | load | 4 | `abs 0x38440` | `-` |
| `str` at 0x383f4 | store | 4 | `[sp-20] +0xd4` | `0x38294` |
| `ldr` at 0x383f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x383fc | load | 4 | `abs 0x38444` | `-` |
| `str` at 0x38400 | store | 4 | `[sp-20] +0xec` | `0x37d74` |
| `ldr` at 0x38404 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x38408 | load | 4 | `abs 0x38448` | `-` |
| `str` at 0x3840c | store | 4 | `[sp-20] +0xf0` | `0x3b0f4` |
| `ldmdb` at 0x38410 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x38410 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x38410 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x38410 | `r0` | `r0` |
