# ARM L1 discovery and configuration entry paths in 1.34.2

Read from `6.5.27/usr/cpu/firmware/sysco/l1.bin`, package `6.5.27:l1#3` (version 1.34.2),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x3b0c1c`.
Carrier SHA-256: `45c872a661d4a4ad20a4cb482fe2aee921ee9ae52784382be125b3d7b7836572`.
Package SHA-256: `d8561deedcf6bae5bef085e0420886534c72bf6c59c54b742d086b9998264128`.
Inner executable SHA-256: `65c32185ef6f284717fd705e751157059ffed1b028bd6195f92eac73749012d3`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1069932 bytes.
Segment SHA-256: `c92d999ce63930093f87780073854fbb74f634d99098b88b7830a55634c9de6c`. Analysed routine spans use the exact
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

[Fact sheets and full listings](disasm/arm-1.34.2-package3/l1-discovery-facts.txt) retain all tool sections.

## `L1CfgReset`

- `L1CfgReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x55b6c to 0x55c3c.
  Established. The first word store clears 0x11a42c (`str` at 0x55b84). The call to 0x18c20 has
  first candidate `[0x11a434]`, second candidate 0x0 and an unknown third candidate (`bl` at
  0x55bb0). The later `beq` takes the zero-word path from 0x11a3f8 to a word save of 0x0 at sp -0x1c
  (`beq` at 0x55be0, `str` at 0x55bf4). The final call to 0x5abf4 has candidates 0x11a434, 0x11a42c,
  0x11a430 and `[[sp-24]+8]` (`bl` at 0x55c24). The globals' configuration-reset meanings are
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x55b7c | load | 4 | `abs 0x55c2c` | `-` |
| `str` at 0x55b84 | store | 4 | `abs 0x11a42c` | `0x0` |
| `ldr` at 0x55b88 | load | 4 | `abs 0x55c2c` | `-` |
| `ldr` at 0x55b8c | load | 4 | `abs 0x55c2c` | `-` |
| `ldr` at 0x55b90 | load | 4 | `abs 0x11a430` | `-` |
| `ldr` at 0x55ba4 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x55bb4 | load | 4 | `abs 0x55c2c` | `-` |
| `str` at 0x55bb8 | store | 4 | `sp -0x18` | `0x11a3a0` |
| `ldr` at 0x55bbc | load | 4 | `abs 0x55c2c` | `-` |
| `ldr` at 0x55bc0 | load | 4 | `abs 0x11a3ac` | `-` |
| `str` at 0x55bc4 | store | 4 | `sp -0x30` | `[0x11a3ac]` |
| `ldr` at 0x55bc8 | load | 4 | `abs 0x55c2c` | `-` |
| `ldr` at 0x55bcc | load | 4 | `abs 0x11a3b0` | `-` |
| `str` at 0x55bd0 | store | 4 | `sp -0x2c` | `[0x11a3b0]` |
| `ldr` at 0x55bd4 | load | 4 | `abs 0x55c2c` | `-` |
| `ldr` at 0x55bd8 | load | 4 | `abs 0x11a3f8` | `-` |
| `str` at 0x55be8 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x55bf4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x55bf8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x55bfc | store | 4 | `sp -0x28` | `[sp-28]` |
| `str` at 0x55c04 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x55c0c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x55c10 | load | 4 | `abs 0x55c30` | `-` |
| `ldr` at 0x55c14 | load | 4 | `abs 0x55c34` | `-` |
| `ldr` at 0x55c18 | load | 4 | `abs 0x55c38` | `-` |
| `ldr` at 0x55c1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x55c20 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldmdb` at 0x55c28 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x55c28 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x55c28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x55bb0 | `(unknown callee) 0x18c20` | `[0x11a434]` | `0x0` | `?` | `?` |
| `bl` at 0x55c24 | `(unknown callee) 0x5abf4` | `0x11a434` | `0x11a42c` | `0x11a430` | `[[sp-24]+8]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x55be0 | `[0x11a3f8] == 0x0` | `[0x11a3f8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x55c28 | `r0` | `ret(0x5abf4)` |

## `L1CfgRefresh`

- `L1CfgRefresh`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x55c3c to 0x55cac.
  Established. Calls to 0x5bba0 include first candidates 0x2, 0xd and 0x14 at the first three sites
  (`bl` at 0x55c4c, `bl` at 0x55c54, `bl` at 0x55c5c). Later sites use first candidates 0x19, 0x1
  and 0x3 (`bl` at 0x55c84, `bl` at 0x55c8c, `bl` at 0x55c94). The last two calls supply 0xe and 0xf
  as their first candidates (`bl` at 0x55c9c, `bl` at 0x55ca4). The return carries `ret(0x5bba0)`
  from the last call (`ldmdb` at 0x55ca8). The numeric request meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x55ca8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x55ca8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x55ca8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x55c4c | `(unknown callee) 0x5bba0` | `0x2` | `r1` | `r2` | `r3` |
| `bl` at 0x55c54 | `(unknown callee) 0x5bba0` | `0xd` | `?` | `?` | `?` |
| `bl` at 0x55c5c | `(unknown callee) 0x5bba0` | `0x14` | `?` | `?` | `?` |
| `bl` at 0x55c64 | `(unknown callee) 0x5bba0` | `0x15` | `?` | `?` | `?` |
| `bl` at 0x55c6c | `(unknown callee) 0x5bba0` | `0x16` | `?` | `?` | `?` |
| `bl` at 0x55c74 | `(unknown callee) 0x5bba0` | `0x17` | `?` | `?` | `?` |
| `bl` at 0x55c7c | `(unknown callee) 0x5bba0` | `0x18` | `?` | `?` | `?` |
| `bl` at 0x55c84 | `(unknown callee) 0x5bba0` | `0x19` | `?` | `?` | `?` |
| `bl` at 0x55c8c | `(unknown callee) 0x5bba0` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x55c94 | `(unknown callee) 0x5bba0` | `0x3` | `?` | `?` | `?` |
| `bl` at 0x55c9c | `(unknown callee) 0x5bba0` | `0xe` | `?` | `?` | `?` |
| `bl` at 0x55ca4 | `(unknown callee) 0x5bba0` | `0xf` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x55ca8 | `r0` | `ret(0x5bba0)` |

## `L1CfgMeshTask`

- `L1CfgMeshTask`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x55cac to 0x55ec4.
  Established. The word read at 0x11a0f4 decides `beq`, whose taken zero path skips the call to
  0x189c8 at 0x55cf8 (`ldr` at 0x55ce8, `beq` at 0x55cf0, `bl` at 0x55cf8). On the other path, `bge`
  takes the signed `ret(0x189c8) >= [0x11a0f4]` path to a word-clear at 0x11a0f4 and a store of
  0x5 at 0x1155f8 (`bge` at 0x55d0c, `str` at 0x55d24, `str` at 0x55d58). Later known stores write
  `[0x11a0e4]` at 0x11a0e0 and 0xf at 0x1155f8 (`str` at 0x55e08, `str` at 0x55e78). The
  global-state meanings and task timing units are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x55cbc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x55cc0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x55cc8 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x55cd0 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x55cd8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x55ce4 | load | 4 | `abs 0x55e9c` | `-` |
| `ldr` at 0x55ce8 | load | 4 | `abs 0x11a0f4` | `-` |
| `ldr` at 0x55d00 | load | 4 | `abs 0x55e9c` | `-` |
| `ldr` at 0x55d04 | load | 4 | `abs 0x11a0f4` | `-` |
| `ldr` at 0x55d14 | load | 4 | `abs 0x55ea0` | `-` |
| `ldr` at 0x55d1c | load | 4 | `abs 0x55e9c` | `-` |
| `str` at 0x55d24 | store | 4 | `abs 0x11a0f4` | `0x0` |
| `ldr` at 0x55d28 | load | 4 | `abs 0x55ea4` | `-` |
| `ldr` at 0x55d2c | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x55d3c | load | 4 | `abs 0x55ea8` | `-` |
| `ldr` at 0x55d40 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x55d44 | load | 4 | `abs 0x55eac` | `-` |
| `ldr` at 0x55d50 | load | 4 | `abs 0x55eb0` | `-` |
| `str` at 0x55d58 | store | 4 | `abs 0x1155f8` | `0x5` |
| `ldr` at 0x55d60 | load | 4 | `abs 0x55eb4` | `-` |
| `ldr` at 0x55d64 | load | 4 | `abs 0x11a0e4` | `-` |
| `ldr` at 0x55d7c | load | 4 | `abs 0x55eb4` | `-` |
| `ldr` at 0x55d80 | load | 4 | `abs 0x11a0e4` | `-` |
| `str` at 0x55d88 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x55d8c | load | 4 | `abs 0x55eb8` | `-` |
| `ldr` at 0x55d90 | load | 4 | `abs 0x11a0e8` | `-` |
| `ldr` at 0x55da8 | load | 4 | `abs 0x55eb8` | `-` |
| `ldr` at 0x55dac | load | 4 | `abs 0x11a0e8` | `-` |
| `str` at 0x55db4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x55db8 | load | 4 | `abs 0x55ebc` | `-` |
| `ldr` at 0x55dbc | load | 4 | `abs 0x11a42c` | `-` |
| `ldr` at 0x55dc8 | load | 4 | `abs 0x55eb0` | `-` |
| `ldr` at 0x55dcc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x55dd0 | load | 4 | `abs 0x1155f8` | `-` |
| `str` at 0x55de4 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x55dec | load | 4 | `abs 0x55ec0` | `-` |
| `ldr` at 0x55df0 | load | 4 | `abs 0x11a0e0` | `-` |
| `ldr` at 0x55dfc | load | 4 | `abs 0x55ec0` | `-` |
| `ldr` at 0x55e00 | load | 4 | `abs 0x55eb4` | `-` |
| `ldr` at 0x55e04 | load | 4 | `abs 0x11a0e4` | `-` |
| `str` at 0x55e08 | store | 4 | `abs 0x11a0e0` | `[0x11a0e4]` |
| `str` at 0x55e14 | store | 4 | `sp -0x28` | `ret(0x1c79c)` |
| `ldr` at 0x55e18 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x55e1c | store | 4 | `sp -0x2c` | `ret(0x1c79c)` |
| `ldr` at 0x55e20 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x55e2c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x55e3c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x55e48 | load | 4 | `abs 0x55eb0` | `-` |
| `ldr` at 0x55e4c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x55e50 | load | 4 | `abs 0x1155f8` | `-` |
| `ldr` at 0x55e64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x55e70 | load | 4 | `abs 0x55eb0` | `-` |
| `str` at 0x55e78 | store | 4 | `abs 0x1155f8` | `0xf` |
| `ldr` at 0x55e80 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x55e94 | store | 4 | `sp -0x24` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x55ce0 | `(unknown callee) 0x1f7b8` | `0x64` | `?` | `?` | `?` |
| `bl` at 0x55cf8 | `(unknown callee) 0x189c8` | `0x0` | `?` | `?` | `[0x11a0f4]` |
| `bl` at 0x55d18 | `(unknown callee) 0x56518` | `0xe61f0` | `?` | `[0x11a0f4]` | `ret(0x189c8)` |
| `bl` at 0x55d48 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe6200` | `0x11a0f4` | `0x116928` |
| `bl` at 0x55d4c | `(unknown callee) 0x57930` | `?` | `?` | `?` | `?` |
| `bl` at 0x55d74 | `(unknown callee) 0x189c8` | `0x0` | `?` | `?` | `[0x11a0e4]` |
| `bl` at 0x55da0 | `(unknown callee) 0x189c8` | `0x0` | `?` | `?` | `[0x11a0e8]` |
| `bl` at 0x55e10 | `(unknown callee) 0x1c79c` | `0x9a` | `?` | `?` | `?` |
| `bl` at 0x55e60 | `(unknown callee) 0x57398` | `ret(0x1c79c)` | `?` | `?` | `?` |
| `bl` at 0x55e8c | `(unknown callee) 0x57398` | `ret(0x1c79c)` | `?` | `?` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x55cf0 | `[0x11a0f4] == 0x0` | `[0x11a0f4] != 0x0` |
| `bge` at 0x55d0c | `ret(0x189c8) >= [0x11a0f4] (signed, 32-bit)` | `ret(0x189c8) < [0x11a0f4] (signed, 32-bit)` |
| `beq` at 0x55d38 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x55d6c | `[0x11a0e4] == 0x0` | `[0x11a0e4] != 0x0` |
| `beq` at 0x55d98 | `[0x11a0e8] == 0x0` | `[0x11a0e8] != 0x0` |
| `beq` at 0x55dc4 | `[0x11a42c] == 0x1` | `[0x11a42c] != 0x1` |
| `blt` at 0x55dd8 | `[sp-28] < [0x1155f8] (signed, 32-bit)` | `[sp-28] >= [0x1155f8] (signed, 32-bit)` |
| `bne` at 0x55df8 | `[0x11a0e0] != 0x0` | `[0x11a0e0] == 0x0` |
| `beq` at 0x55e28 | `ret(0x1c79c) == 0x0` | `ret(0x1c79c) != 0x0` |
| `beq` at 0x55e34 | `ret(0x1c79c) == 0x1` | `ret(0x1c79c) != 0x1` |
| `beq` at 0x55e44 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `bge` at 0x55e58 | `[sp-32] >= [0x1155f8] (signed, 32-bit)` | `[sp-32] < [0x1155f8] (signed, 32-bit)` |
| `ble` at 0x55e6c | `[sp-28] <= 0x3c (signed, 32-bit)` | `[sp-28] > 0x3c (signed, 32-bit)` |
| `beq` at 0x55e88 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|

## `L1CfgRingTask`

- `L1CfgRingTask`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x55ec4 to 0x56518.
  Established. Two word saves retain `[0x11a428]+156` at sp -0x44 and `[0x11a428]+312` at sp -0x48
  (`str` at 0x55f5c, `str` at 0x55f6c). The body includes an unknown-valued store and a separate
  store of 0x0 at 0x11a0cc (`str` at 0x56110, `str` at 0x56158). Two call sites targeting 0x9c10c
  have candidates 0x11a398, 0x0, 0x64 and 0x2 (`bl` at 0x562b0, `bl` at 0x56464). Later word stores
  with unknown effective targets contain `[0x11a748]` at separate sites (`str` at 0x56304, `str` at
  0x564b8). The ring-message format, globals and task timing are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x55ed4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x55ed8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x55ee4 | load | 4 | `abs 0x564e0` | `-` |
| `ldr` at 0x55ee8 | load | 4 | `abs 0x11a0f4` | `-` |
| `ldr` at 0x55f00 | load | 4 | `abs 0x564e0` | `-` |
| `ldr` at 0x55f04 | load | 4 | `abs 0x11a0f4` | `-` |
| `ldr` at 0x55f14 | load | 4 | `abs 0x564e4` | `-` |
| `ldr` at 0x55f1c | load | 4 | `abs 0x564e0` | `-` |
| `str` at 0x55f24 | store | 4 | `abs 0x11a0f4` | `0x0` |
| `ldr` at 0x55f28 | load | 4 | `abs 0x564e8` | `-` |
| `ldr` at 0x55f2c | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x55f3c | load | 4 | `abs 0x564ec` | `-` |
| `ldr` at 0x55f40 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x55f44 | load | 4 | `abs 0x564f0` | `-` |
| `ldr` at 0x55f50 | load | 4 | `abs 0x564f4` | `-` |
| `ldr` at 0x55f54 | load | 4 | `abs 0x11a428` | `-` |
| `str` at 0x55f5c | store | 4 | `sp -0x44` | `[0x11a428]+156` |
| `ldr` at 0x55f60 | load | 4 | `abs 0x564f4` | `-` |
| `ldr` at 0x55f64 | load | 4 | `abs 0x11a428` | `-` |
| `str` at 0x55f6c | store | 4 | `sp -0x48` | `[0x11a428]+312` |
| `ldr` at 0x55fb0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x55fb4 | load | 4 | `[sp-68] +0x4` | `-` |
| `ldr` at 0x55fc0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x55fc4 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x55fd0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x55fd4 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x55fe0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x55fe4 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x55ff0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x55ff4 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x56000 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x56004 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x56010 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x56014 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x56020 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x56024 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x56030 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x56034 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x56040 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x56044 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x56064 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x56068 | load | 4 | `[sp-72] +0x4` | `-` |
| `ldr` at 0x56074 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x56078 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x56084 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x56088 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x56094 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x56098 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x560a4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x560a8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x560b4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x560b8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x560c4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x560c8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x560d4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x560d8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x560e4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x560e8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x560f4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x560f8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x56108 | load | 4 | `abs 0x564f8` | `-` |
| `str` at 0x56110 | store | 4 | `abs 0x11a0cc` | `?` |
| `ldr` at 0x56118 | load | 4 | `abs 0x564f8` | `-` |
| `ldr` at 0x5611c | load | 4 | `abs 0x11a0cc` | `-` |
| `ldr` at 0x56150 | load | 4 | `abs 0x564f8` | `-` |
| `str` at 0x56158 | store | 4 | `abs 0x11a0cc` | `0x0` |
| `ldr` at 0x5615c | load | 4 | `abs 0x564fc` | `-` |
| `ldr` at 0x56160 | load | 4 | `abs 0x564f4` | `-` |
| `ldr` at 0x56180 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x56184 | load | 4 | `[sp-68] +0x4` | `-` |
| `ldr` at 0x56190 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x56194 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x561a0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x561a4 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x561b0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x561b4 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x561c0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x561c4 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x561d0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x561d4 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x561e0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x561e4 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x561f0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x561f4 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x56200 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x56204 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x56210 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x56214 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x5627c | load | 4 | `abs 0x564fc` | `-` |
| `str` at 0x5628c | store | 4 | `sp -0x54` | `0x2` |
| `ldr` at 0x56290 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x56294 | store | 4 | `sp -0x50` | `[sp-64]` |
| `str` at 0x5629c | store | 4 | `sp -0x4c` | `sp-60` |
| `ldr` at 0x562a0 | load | 4 | `abs 0x56500` | `-` |
| `str` at 0x562b8 | store | 4 | `sp -0x1c` | `ret(0x9c10c)` |
| `ldr` at 0x562bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x562c8 | load | 4 | `abs 0x56504` | `-` |
| `ldr` at 0x562d4 | load | 4 | `abs 0x564e8` | `-` |
| `ldr` at 0x562d8 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x562e8 | load | 4 | `abs 0x564ec` | `-` |
| `ldr` at 0x562ec | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x562f0 | load | 4 | `abs 0x56508` | `-` |
| `ldr` at 0x562f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x562fc | load | 4 | `abs 0x5650c` | `-` |
| `ldr` at 0x56300 | load | 4 | `abs 0x11a748` | `-` |
| `str` at 0x56304 | store | 4 | `(unknown)` | `[0x11a748]` |
| `ldr` at 0x56308 | load | 4 | `abs 0x56510` | `-` |
| `ldr` at 0x5630c | load | 4 | `abs 0x281f74` | `-` |
| `ldr` at 0x56318 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x56334 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x56338 | load | 4 | `[sp-72] +0x4` | `-` |
| `ldr` at 0x56344 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x56348 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x56354 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x56358 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x56364 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x56368 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x56374 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x56378 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x56384 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x56388 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x56394 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x56398 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x563a4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x563a8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x563b4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x563b8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x563c4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x563c8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x56430 | load | 4 | `abs 0x564fc` | `-` |
| `str` at 0x56440 | store | 4 | `sp -0x54` | `0x2` |
| `ldr` at 0x56444 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x56448 | store | 4 | `sp -0x50` | `[sp-64]` |
| `str` at 0x56450 | store | 4 | `sp -0x4c` | `sp-60` |
| `ldr` at 0x56454 | load | 4 | `abs 0x56500` | `-` |
| `str` at 0x5646c | store | 4 | `sp -0x1c` | `ret(0x9c10c)` |
| `ldr` at 0x56470 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5647c | load | 4 | `abs 0x56504` | `-` |
| `ldr` at 0x56488 | load | 4 | `abs 0x564e8` | `-` |
| `ldr` at 0x5648c | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x5649c | load | 4 | `abs 0x564ec` | `-` |
| `ldr` at 0x564a0 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x564a4 | load | 4 | `abs 0x56514` | `-` |
| `ldr` at 0x564ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x564b0 | load | 4 | `abs 0x5650c` | `-` |
| `ldr` at 0x564b4 | load | 4 | `abs 0x11a748` | `-` |
| `str` at 0x564b8 | store | 4 | `(unknown)` | `[0x11a748]` |
| `ldr` at 0x564bc | load | 4 | `abs 0x56510` | `-` |
| `ldr` at 0x564c0 | load | 4 | `abs 0x281f78` | `-` |
| `ldr` at 0x564cc | load | 4 | `sp -0x1c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x55ee0 | `(unknown callee) 0x603b4` | `0x1f4` | `r1` | `r2` | `r3` |
| `bl` at 0x55ef8 | `(unknown callee) 0x189c8` | `0x0` | `?` | `?` | `[0x11a0f4]` |
| `bl` at 0x55f18 | `(unknown callee) 0x56518` | `0xe61f0` | `?` | `[0x11a0f4]` | `ret(0x189c8)` |
| `bl` at 0x55f48 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe6200` | `0x11a0f4` | `0x116928` |
| `bl` at 0x55f4c | `(unknown callee) 0x57930` | `?` | `?` | `?` | `?` |
| `bl` at 0x55f74 | `(unknown callee) 0x63278` | `0x1` | `?` | `?` | `[0x11a428]+312` |
| `bl` at 0x55f88 | `(unknown callee) 0x63278` | `0x2` | `?` | `?` | `ret(0x63278)` |
| `bl` at 0x55fa0 | `(unknown callee) 0x63278` | `0x1` | `?` | `?` | `ret(0x63278)` |
| `bl` at 0x56054 | `(unknown callee) 0x63278` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x5612c | `(unknown callee) 0x63278` | `0x1` | `?` | `?` | `[0x11a0cc]` |
| `bl` at 0x56140 | `(unknown callee) 0x63278` | `0x2` | `?` | `?` | `ret(0x63278)` |
| `bl` at 0x56168 | `(unknown callee) 0x18ae8` | `0x11a0f8` | `0x11a3a0` | `0x70` | `?` |
| `bl` at 0x56170 | `(unknown callee) 0x63278` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x56238 | `(unknown callee) 0x541d4` | `sp-64` | `sp-60` | `0x2` | `sp-64` |
| `bl` at 0x56250 | `(unknown callee) 0x54230` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x56268 | `(unknown callee) 0x54230` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x56284 | `(unknown callee) 0x5b7e4` | `sp-64` | `sp-60` | `0x11a0f8` | `0x5` |
| `bl` at 0x562b0 | `(unknown callee) 0x9c10c` | `0x11a398` | `0x0` | `0x64` | `0x2` |
| `bl` at 0x562cc | `(unknown callee) 0x1d6d8` | `0xe6214` | `?` | `?` | `ret(0x9c10c)` |
| `bl` at 0x562f4 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe6240` | `?` | `0x116928` |
| `bl` at 0x5631c | `(unknown callee) 0x9ba98` | `[0x281f74]+20` | `[sp-28]` | `?` | `[0x281f74]+20` |
| `bl` at 0x56324 | `(unknown callee) 0x63278` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x563ec | `(unknown callee) 0x541d4` | `sp-64` | `sp-60` | `0x2` | `sp-64` |
| `bl` at 0x56404 | `(unknown callee) 0x54230` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x5641c | `(unknown callee) 0x54230` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x56438 | `(unknown callee) 0x5b7e4` | `sp-64` | `sp-60` | `0x11a0f8` | `0x5` |
| `bl` at 0x56464 | `(unknown callee) 0x9c10c` | `0x11a398` | `0x0` | `0x64` | `0x2` |
| `bl` at 0x56480 | `(unknown callee) 0x1d6d8` | `0xe6214` | `?` | `?` | `ret(0x9c10c)` |
| `bl` at 0x564a8 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe6258` | `?` | `0x116928` |
| `bl` at 0x564d0 | `(unknown callee) 0x9ba98` | `[0x281f78]+20` | `[sp-28]` | `?` | `[0x281f78]+20` |
| `bl` at 0x564d8 | `(unknown callee) 0x603b4` | `0x1f4` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x55ef0 | `[0x11a0f4] == 0x0` | `[0x11a0f4] != 0x0` |
| `bge` at 0x55f0c | `ret(0x189c8) >= [0x11a0f4] (signed, 32-bit)` | `ret(0x189c8) < [0x11a0f4] (signed, 32-bit)` |
| `beq` at 0x55f38 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x55f80 | `ret(0x63278) != 0x0` | `ret(0x63278) == 0x0` |
| `bne` at 0x55f94 | `ret(0x63278) != 0x0` | `ret(0x63278) == 0x0` |
| `beq` at 0x55fac | `ret(0x63278) == 0x0` | `ret(0x63278) != 0x0` |
| `beq` at 0x55fbc | `[[sp-68]+4] == 0x0` | `[[sp-68]+4] != 0x0` |
| `beq` at 0x55fcc | `[[sp-68]+16] == 0x35` | `[[sp-68]+16] != 0x35` |
| `beq` at 0x55fdc | `[[sp-68]+16] == 0x4f` | `[[sp-68]+16] != 0x4f` |
| `beq` at 0x55fec | `[[sp-68]+16] == 0x42` | `[[sp-68]+16] != 0x42` |
| `beq` at 0x55ffc | `[[sp-68]+16] == 0x46` | `[[sp-68]+16] != 0x46` |
| `beq` at 0x5600c | `[[sp-68]+16] == 0x4a` | `[[sp-68]+16] != 0x4a` |
| `beq` at 0x5601c | `[[sp-68]+16] == 0x4b` | `[[sp-68]+16] != 0x4b` |
| `beq` at 0x5602c | `[[sp-68]+16] == 0x21` | `[[sp-68]+16] != 0x21` |
| `beq` at 0x5603c | `[[sp-68]+16] == 0x5e` | `[[sp-68]+16] != 0x5e` |
| `bne` at 0x5604c | `[[sp-68]+16] != 0x4c` | `[[sp-68]+16] == 0x4c` |
| `beq` at 0x56060 | `ret(0x63278) == 0x0` | `ret(0x63278) != 0x0` |
| `beq` at 0x56070 | `[[sp-72]+4] == 0x0` | `[[sp-72]+4] != 0x0` |
| `beq` at 0x56080 | `[[sp-72]+16] == 0x35` | `[[sp-72]+16] != 0x35` |
| `beq` at 0x56090 | `[[sp-72]+16] == 0x4f` | `[[sp-72]+16] != 0x4f` |
| `beq` at 0x560a0 | `[[sp-72]+16] == 0x42` | `[[sp-72]+16] != 0x42` |
| `beq` at 0x560b0 | `[[sp-72]+16] == 0x46` | `[[sp-72]+16] != 0x46` |
| `beq` at 0x560c0 | `[[sp-72]+16] == 0x4a` | `[[sp-72]+16] != 0x4a` |
| `beq` at 0x560d0 | `[[sp-72]+16] == 0x4b` | `[[sp-72]+16] != 0x4b` |
| `beq` at 0x560e0 | `[[sp-72]+16] == 0x21` | `[[sp-72]+16] != 0x21` |
| `beq` at 0x560f0 | `[[sp-72]+16] == 0x5e` | `[[sp-72]+16] != 0x5e` |
| `bne` at 0x56100 | `[[sp-72]+16] != 0x4c` | `[[sp-72]+16] == 0x4c` |
| `blt` at 0x56124 | `[0x11a0cc] < 0x0 (signed, 32-bit)` | `[0x11a0cc] >= 0x0 (signed, 32-bit)` |
| `beq` at 0x56138 | `ret(0x63278) == 0x0` | `ret(0x63278) != 0x0` |
| `bne` at 0x5614c | `ret(0x63278) != 0x0` | `ret(0x63278) == 0x0` |
| `beq` at 0x5617c | `ret(0x63278) == 0x0` | `ret(0x63278) != 0x0` |
| `beq` at 0x5618c | `[[sp-68]+4] == 0x0` | `[[sp-68]+4] != 0x0` |
| `beq` at 0x5619c | `[[sp-68]+16] == 0x35` | `[[sp-68]+16] != 0x35` |
| `beq` at 0x561ac | `[[sp-68]+16] == 0x4f` | `[[sp-68]+16] != 0x4f` |
| `beq` at 0x561bc | `[[sp-68]+16] == 0x42` | `[[sp-68]+16] != 0x42` |
| `beq` at 0x561cc | `[[sp-68]+16] == 0x46` | `[[sp-68]+16] != 0x46` |
| `beq` at 0x561dc | `[[sp-68]+16] == 0x4a` | `[[sp-68]+16] != 0x4a` |
| `beq` at 0x561ec | `[[sp-68]+16] == 0x4b` | `[[sp-68]+16] != 0x4b` |
| `beq` at 0x561fc | `[[sp-68]+16] == 0x21` | `[[sp-68]+16] != 0x21` |
| `beq` at 0x5620c | `[[sp-68]+16] == 0x5e` | `[[sp-68]+16] != 0x5e` |
| `beq` at 0x5621c | `[[sp-68]+16] == 0x4c` | `[[sp-68]+16] != 0x4c` |
| `bne` at 0x562c4 | `ret(0x9c10c) != 0x0` | `ret(0x9c10c) == 0x0` |
| `beq` at 0x562e4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x56330 | `ret(0x63278) == 0x0` | `ret(0x63278) != 0x0` |
| `beq` at 0x56340 | `[[sp-72]+4] == 0x0` | `[[sp-72]+4] != 0x0` |
| `beq` at 0x56350 | `[[sp-72]+16] == 0x35` | `[[sp-72]+16] != 0x35` |
| `beq` at 0x56360 | `[[sp-72]+16] == 0x4f` | `[[sp-72]+16] != 0x4f` |
| `beq` at 0x56370 | `[[sp-72]+16] == 0x42` | `[[sp-72]+16] != 0x42` |
| `beq` at 0x56380 | `[[sp-72]+16] == 0x46` | `[[sp-72]+16] != 0x46` |
| `beq` at 0x56390 | `[[sp-72]+16] == 0x4a` | `[[sp-72]+16] != 0x4a` |
| `beq` at 0x563a0 | `[[sp-72]+16] == 0x4b` | `[[sp-72]+16] != 0x4b` |
| `beq` at 0x563b0 | `[[sp-72]+16] == 0x21` | `[[sp-72]+16] != 0x21` |
| `beq` at 0x563c0 | `[[sp-72]+16] == 0x5e` | `[[sp-72]+16] != 0x5e` |
| `beq` at 0x563d0 | `[[sp-72]+16] == 0x4c` | `[[sp-72]+16] != 0x4c` |
| `bne` at 0x56478 | `ret(0x9c10c) != 0x0` | `ret(0x9c10c) == 0x0` |
| `beq` at 0x56498 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|

## `L1CfgLock`

- `L1CfgLock`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x56518 to 0x5659c.
  Established. The call to 0x1fb00 has first candidate 0x11a340 and second candidate 0x1e (`bl` at
  0x56534). `beq` takes its zero-result path past the call to 0x1eed0 (`beq` at 0x56548, `bl` at
  0x56574). The final call targets 0x18cd4 with first candidate 0x281ef8 and second candidate
  `[sp-20]` (`bl` at 0x56580); its result reaches the return in r0 (`ldmdb` at 0x56584). The lock
  mechanism and time unit are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x56528 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x5652c | load | 4 | `abs 0x56588` | `-` |
| `str` at 0x5653c | store | 4 | `sp -0x18` | `ret(0x1fb00)` |
| `ldr` at 0x56540 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5654c | load | 4 | `abs 0x5658c` | `-` |
| `str` at 0x56550 | store | 4 | `sp -0x24` | `0xe6270` |
| `ldr` at 0x56554 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x56558 | store | 4 | `sp -0x20` | `ret(0x1fb00)` |
| `ldr` at 0x5655c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x56560 | store | 4 | `sp -0x1c` | `[sp-20]` |
| `ldr` at 0x56568 | load | 4 | `abs 0x56590` | `-` |
| `ldr` at 0x5656c | load | 4 | `abs 0x56594` | `-` |
| `ldr` at 0x56578 | load | 4 | `abs 0x56598` | `-` |
| `ldr` at 0x5657c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x56584 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x56584 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x56584 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x56534 | `(unknown callee) 0x1fb00` | `0x11a340` | `0x1e` | `r2` | `r3` |
| `bl` at 0x56574 | `(unknown callee) 0x1eed0` | `0x9` | `0xe6174` | `0x22f` | `0x0` |
| `bl` at 0x56580 | `(unknown callee) 0x18cd4` | `0x281ef8` | `[sp-20]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x56548 | `ret(0x1fb00) == 0x0` | `ret(0x1fb00) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x56584 | `r0` | `ret(0x18cd4)` |

## `L1CfgUnlock`

- `L1CfgUnlock`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5659c to 0x56610.
  Established. The first call to 0x18cd4 receives first candidate 0x281ef8 and second candidate 0xe6294 (`bl` at
  0x565b4). The call to 0x1fe64 then uses first candidate 0x11a340 (`bl` at 0x565bc). `beq` takes
  that call's zero-result path to the return, while nonzero fall-through reaches 0x1eed0 with
  candidates 0x9, 0xe6174, 0x23c and 0x0 (`beq` at 0x565d0, `bl` at 0x565f4). The physical
  synchronization behavior is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x565ac | load | 4 | `abs 0x565fc` | `-` |
| `ldr` at 0x565b0 | load | 4 | `abs 0x56600` | `-` |
| `ldr` at 0x565b8 | load | 4 | `abs 0x56604` | `-` |
| `str` at 0x565c4 | store | 4 | `sp -0x14` | `ret(0x1fe64)` |
| `ldr` at 0x565c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x565d4 | load | 4 | `abs 0x56608` | `-` |
| `str` at 0x565d8 | store | 4 | `sp -0x1c` | `0xe62a0` |
| `ldr` at 0x565dc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x565e0 | store | 4 | `sp -0x18` | `ret(0x1fe64)` |
| `ldr` at 0x565e8 | load | 4 | `abs 0x5660c` | `-` |
| `ldmdb` at 0x565f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x565f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x565f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x565b4 | `(unknown callee) 0x18cd4` | `0x281ef8` | `0xe6294` | `r2` | `r3` |
| `bl` at 0x565bc | `(unknown callee) 0x1fe64` | `0x11a340` | `?` | `?` | `?` |
| `bl` at 0x565f4 | `(unknown callee) 0x1eed0` | `0x9` | `0xe6174` | `0x23c` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x565d0 | `ret(0x1fe64) == 0x0` | `ret(0x1fe64) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x565f8 | `r0` | `?` |

## `L1CfgDiscoveryResponse`

- `L1CfgDiscoveryResponse`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x56610 to 0x56b54.
  Established. A word loaded at entry r0 +0xc is saved at sp -0x1c (`ldr` at 0x56628, `str` at
  0x5662c). The later word store at `[sp-36] +0x90` contains `[sp-40]` (`str` at 0x566a0). The `ble`
  taken predicate compares unknown operands signed, and its fall-through reaches an unknown-valued
  word store with unknown effective target (`ble` at 0x56760, `str` at 0x567d0). Another word store
  with unknown effective target contains `ret(0x18aac)` (`str` at 0x56870). The final call targets
  0x57930 (`bl` at 0x56b28). Discovery-record meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x56620 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x56624 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x56628 | load | 4 | `r0 +0xc` | `-` |
| `str` at 0x5662c | store | 4 | `sp -0x1c` | `[r0+12]` |
| `ldr` at 0x56630 | load | 4 | `abs 0x56b30` | `-` |
| `ldr` at 0x56638 | load | 4 | `abs 0x56b34` | `-` |
| `ldr` at 0x5663c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x56658 | load | 4 | `abs 0x11a428` | `-` |
| `str` at 0x56660 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x56664 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x56668 | load | 1 | `[sp-24] +0x44` | `-` |
| `ldr` at 0x56678 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5667c | load | 4 | `[sp-24] +0x48` | `-` |
| `str` at 0x56680 | store | 4 | `sp -0x28` | `[[sp-24]+72]` |
| `ldr` at 0x56688 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x56694 | store | 4 | `sp -0x28` | `ret(0x9ca84)` |
| `ldr` at 0x56698 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5669c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x566a0 | store | 4 | `[sp-36] +0x90` | `[sp-40]` |
| `ldr` at 0x566a4 | load | 4 | `abs 0x56b38` | `-` |
| `ldr` at 0x566a8 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x566c0 | load | 4 | `abs 0x56b3c` | `-` |
| `ldr` at 0x566c4 | load | 4 | `abs 0x56b34` | `-` |
| `ldr` at 0x566c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x566e4 | load | 4 | `abs 0x11a428` | `-` |
| `str` at 0x566ec | store | 4 | `sp -0x34` | `ret(0x189c8)` |
| `ldr` at 0x566f0 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x566f4 | load | 4 | `abs 0x56b40` | `-` |
| `ldr` at 0x566f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x566fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x56704 | load | 4 | `abs 0x56b34` | `-` |
| `ldr` at 0x56708 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x56724 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5672c | load | 4 | `abs 0x56b34` | `-` |
| `ldr` at 0x56730 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5674c | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x56754 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x56758 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x56764 | load | 4 | `abs 0x56b34` | `-` |
| `ldr` at 0x56768 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x56784 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5678c | load | 4 | `abs 0x56b34` | `-` |
| `ldr` at 0x56790 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x567ac | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x567b4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x567d0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x567d4 | load | 4 | `abs 0x56b34` | `-` |
| `ldr` at 0x567d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x567f4 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x567fc | load | 4 | `abs 0x56b34` | `-` |
| `ldr` at 0x56800 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5681c | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x56824 | load | 4 | `abs 0x56b34` | `-` |
| `ldr` at 0x56828 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x56844 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5684c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x56860 | load | 4 | `(unknown)` | `-` |
| `str` at 0x56870 | store | 4 | `(unknown)` | `ret(0x18aac)` |
| `ldr` at 0x56874 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x56878 | load | 1 | `[sp-24] +0x4c` | `-` |
| `ldr` at 0x56888 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5688c | load | 4 | `[sp-24] +0x50` | `-` |
| `str` at 0x56890 | store | 4 | `sp -0x2c` | `[[sp-24]+80]` |
| `ldr` at 0x56898 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x568a4 | store | 4 | `sp -0x2c` | `ret(0x9ca84)` |
| `ldr` at 0x568a8 | load | 4 | `abs 0x56b34` | `-` |
| `ldr` at 0x568ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x568c8 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x568d0 | load | 4 | `abs 0x56b34` | `-` |
| `ldr` at 0x568d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x568f0 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x568f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5690c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x56910 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5691c | load | 4 | `abs 0x56b38` | `-` |
| `ldr` at 0x56920 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x56930 | load | 4 | `abs 0x56b44` | `-` |
| `ldr` at 0x56934 | load | 4 | `abs 0x1155f4` | `-` |
| `ldr` at 0x56938 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5694c | load | 4 | `abs 0x56b3c` | `-` |
| `ldr` at 0x56950 | load | 4 | `abs 0x56b34` | `-` |
| `ldr` at 0x56954 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x56970 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x56978 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x5697c | load | 4 | `abs 0x56b48` | `-` |
| `ldr` at 0x56980 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x56984 | load | 4 | `(unknown)` | `-` |
| `str` at 0x56990 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x56994 | load | 4 | `abs 0x56b34` | `-` |
| `ldr` at 0x56998 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x569b4 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x569bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x569c0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x569d0 | load | 4 | `abs 0x56b3c` | `-` |
| `ldr` at 0x569d4 | load | 4 | `abs 0x56b34` | `-` |
| `ldr` at 0x569d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x569f4 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x569fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x56a10 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x56a18 | load | 4 | `abs 0x56b34` | `-` |
| `ldr` at 0x56a1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x56a38 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x56a40 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x56a54 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x56a5c | load | 4 | `abs 0x56b34` | `-` |
| `ldr` at 0x56a60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x56a7c | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x56a84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x56a98 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x56aa0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x56aa4 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x56aa8 | load | 4 | `abs 0x56b34` | `-` |
| `ldr` at 0x56aac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x56ac8 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x56ad0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x56ae4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x56aec | load | 4 | `(unknown)` | `-` |
| `str` at 0x56af0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x56af4 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x56af8 | load | 4 | `abs 0x56b4c` | `-` |
| `ldr` at 0x56afc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x56b00 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x56b08 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x56b10 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0x56b18 | load | 4 | `abs 0x56b3c` | `-` |
| `ldr` at 0x56b1c | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x56b20 | load | 4 | `abs 0x56b50` | `-` |
| `ldmdb` at 0x56b2c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x56b2c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x56b2c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x56b2c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x56634 | `(unknown callee) 0x56518` | `0xe62c4` | `r1` | `r2` | `[r0+12]` |
| `bl` at 0x56690 | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x4` | `?` | `?` |
| `bl` at 0x566bc | `(unknown callee) 0x189c8` | `0x0` | `?` | `[sp-36]` | `?` |
| `bl` at 0x56700 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe62dc` | `[sp-28]` | `?` |
| `bl` at 0x56868 | `(unknown callee) 0x18aac` | `?` | `?` | `?` | `?` |
| `bl` at 0x568a0 | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x5` | `?` | `?` |
| `bl` at 0x56918 | `(unknown callee) 0x18ae8` | `?` | `[sp-44]` | `?` | `?` |
| `bl` at 0x56988 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe6314` | `[sp-28]` | `?` |
| `bl` at 0x56b04 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe6324` | `?` | `?` |
| `bl` at 0x56b24 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe6338` | `[sp-32]` | `0x116928` |
| `bl` at 0x56b28 | `(unknown callee) 0x57930` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x56674 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x566b4 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x56760 | `? <= ? (signed, 32-bit)` | `? > ? (signed, 32-bit)` |
| `bne` at 0x56884 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x5692c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x56948 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x569c8 | `[sp-32] < ? (signed, 32-bit)` | `[sp-32] >= ? (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x56b2c | `r0` | `ret(0x57930)` |

## `L1CfgDiscoveryRequest`

- `L1CfgDiscoveryRequest`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x56b54 to 0x57398.
  Established. The setup includes byte and halfword stores of 0x0 at unknown effective targets, plus
  a word store of 0x3e8 at an unknown target (`strb` at 0x56bf4, `strb` at 0x56c14, `strh` at
  0x56c54, `str` at 0x56c34). Later `beq` sites take paths for the separately loaded word at
  0x11a3b0 equal to 0x52 or 0x44 (`ldr` at 0x56e1c, `beq` at 0x56e24, `ldr` at 0x56e2c, `beq` at
  0x56e34). One path includes a call to 0x18920 with first candidate 0x18 and a call to 0x18ae8 with
  third candidate 0x18 (`bl` at 0x56f38, `bl` at 0x56f6c). The later call to 0x9c10c has candidates
  0x11a398, sp-28, 0x0 and 0x0 (`bl` at 0x572c0). Packet-field meanings, target addresses left
  unknown and ownership semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x56b64 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x56b6c | store | 4 | `sp -0x78` | `0x0` |
| `ldr` at 0x56b70 | load | 4 | `abs 0x57360` | `-` |
| `ldr` at 0x56b74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x56b90 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x56b98 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x56ba8 | load | 4 | `abs 0x57364` | `-` |
| `ldr` at 0x56bac | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x56bbc | load | 4 | `abs 0x57368` | `-` |
| `ldr` at 0x56bc0 | load | 4 | `abs 0x57360` | `-` |
| `ldr` at 0x56bc4 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x56bc8 | load | 4 | `abs 0x5736c` | `-` |
| `ldr` at 0x56bcc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x56bd0 | load | 4 | `abs 0x11a42c` | `-` |
| `ldr` at 0x56bd8 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x56bf4 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x56bf8 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x56c14 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x56c18 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x56c34 | store | 4 | `(unknown)` | `0x3e8` |
| `ldr` at 0x56c38 | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x56c54 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x56c58 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x56c60 | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x56c64 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x56c80 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x56c84 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x56ca0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x56ca4 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x56cbc | load | 4 | `abs 0x57360` | `-` |
| `ldr` at 0x56cc0 | load | 4 | `abs 0x11a3a8` | `-` |
| `str` at 0x56cc4 | store | 4 | `(unknown)` | `[0x11a3a8]` |
| `ldr` at 0x56cc8 | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x56ce4 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x56ce8 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x56cf0 | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x56cf4 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x56d10 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x56d14 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x56d30 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x56d34 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x56d4c | load | 4 | `abs 0x57360` | `-` |
| `ldr` at 0x56d50 | load | 4 | `abs 0x11a3ac` | `-` |
| `str` at 0x56d54 | store | 4 | `(unknown)` | `[0x11a3ac]` |
| `ldr` at 0x56d58 | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x56d74 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x56d78 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x56d80 | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x56d84 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x56da0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x56da4 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x56dc0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x56dc4 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x56ddc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x56de0 | store | 4 | `(unknown)` | `[sp-20]` |
| `ldr` at 0x56de4 | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x56e00 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x56e04 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x56e0c | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x56e10 | load | 4 | `abs 0x57370` | `-` |
| `ldr` at 0x56e18 | load | 4 | `abs 0x57360` | `-` |
| `ldr` at 0x56e1c | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x56e28 | load | 4 | `abs 0x57360` | `-` |
| `ldr` at 0x56e2c | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x56e3c | load | 4 | `abs 0x57360` | `-` |
| `ldr` at 0x56e40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x56e5c | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x56e64 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x56e70 | load | 4 | `abs 0x57360` | `-` |
| `ldr` at 0x56e74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x56e90 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x56e98 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x56ea8 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x56ec4 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x56ec8 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x56ee4 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x56ee8 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x56f04 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x56f08 | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x56f24 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x56f28 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x56f30 | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `str` at 0x56f40 | store | 4 | `sp -0x18` | `ret(0x18920)` |
| `ldr` at 0x56f44 | load | 4 | `abs 0x57360` | `-` |
| `ldr` at 0x56f48 | load | 4 | `abs 0x57360` | `-` |
| `ldr` at 0x56f4c | load | 4 | `abs 0x11a3a8` | `-` |
| `ldr` at 0x56f50 | load | 4 | `abs 0x11a3ac` | `-` |
| `str` at 0x56f5c | store | 4 | `sp -0x7c` | `ret(0x5e764)` |
| `ldr` at 0x56f60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x56f64 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x56f70 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x56f8c | store | 1 | `(unknown)` | `0x9` |
| `ldr` at 0x56f90 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x56fac | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x56fb0 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x56fc8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x56fcc | store | 4 | `(unknown)` | `[sp-24]` |
| `ldr` at 0x56fd0 | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x56fec | store | 2 | `(unknown)` | `0x18` |
| `ldr` at 0x56ff0 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x56ff8 | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x57000 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x5701c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x57020 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x5703c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x57040 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x57058 | load | 4 | `abs 0x57360` | `-` |
| `ldr` at 0x5705c | load | 4 | `abs 0x11a42c` | `-` |
| `str` at 0x57060 | store | 4 | `(unknown)` | `[0x11a42c]` |
| `ldr` at 0x57064 | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x57080 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x57084 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x5708c | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x57090 | load | 4 | `abs 0x57360` | `-` |
| `ldr` at 0x57094 | load | 4 | `abs 0x11a42c` | `-` |
| `str` at 0x570b4 | store | 4 | `sp -0x18` | `ret(0x18920)` |
| `ldr` at 0x570b8 | load | 4 | `abs 0x57360` | `-` |
| `ldr` at 0x570bc | load | 4 | `abs 0x57360` | `-` |
| `ldr` at 0x570c0 | load | 4 | `abs 0x11a42c` | `-` |
| `ldr` at 0x570d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x570d8 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x570e4 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x57100 | store | 1 | `(unknown)` | `0x9` |
| `ldr` at 0x57104 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x57120 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x57124 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x5713c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x57140 | store | 4 | `(unknown)` | `[sp-24]` |
| `ldr` at 0x57144 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x5715c | load | 4 | `abs 0x57360` | `-` |
| `ldr` at 0x57160 | load | 4 | `abs 0x11a42c` | `-` |
| `strh` at 0x57174 | store | 2 | `(unknown)` | `?` |
| `ldr` at 0x57178 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x57180 | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x57188 | load | 4 | `abs 0x57374` | `-` |
| `str` at 0x5718c | store | 4 | `sp -0x1c` | `0xffff00e` |
| `ldr` at 0x57190 | load | 4 | `abs 0x57364` | `-` |
| `ldr` at 0x57194 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x571a4 | load | 4 | `abs 0x57378` | `-` |
| `ldr` at 0x571a8 | load | 4 | `abs 0x1155f4` | `-` |
| `ldr` at 0x571ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x571c0 | load | 4 | `abs 0x57368` | `-` |
| `ldr` at 0x571c4 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x571c8 | load | 4 | `abs 0x5737c` | `-` |
| `ldr` at 0x571cc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x571d8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x571dc | load | 4 | `abs 0x57360` | `-` |
| `ldr` at 0x571e0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x571e4 | load | 4 | `abs 0x11a42c` | `-` |
| `ldr` at 0x571f4 | load | 4 | `abs 0x57368` | `-` |
| `ldr` at 0x571f8 | load | 4 | `abs 0x57360` | `-` |
| `ldr` at 0x571fc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x57210 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x57218 | load | 4 | `abs 0x57360` | `-` |
| `ldr` at 0x5721c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x57230 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x57238 | load | 4 | `abs 0x57360` | `-` |
| `ldr` at 0x5723c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x57250 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x57258 | load | 4 | `(unknown)` | `-` |
| `str` at 0x5725c | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x57260 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x57264 | load | 4 | `abs 0x57380` | `-` |
| `ldr` at 0x57268 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5726c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x57274 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5727c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x57284 | load | 4 | `abs 0x57368` | `-` |
| `ldr` at 0x57288 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x5728c | load | 4 | `abs 0x57384` | `-` |
| `str` at 0x5729c | store | 4 | `sp -0x88` | `0x2` |
| `ldr` at 0x572a0 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x572a4 | store | 4 | `sp -0x84` | `[sp-120]` |
| `str` at 0x572ac | store | 4 | `sp -0x80` | `sp-116` |
| `ldr` at 0x572b0 | load | 4 | `abs 0x57388` | `-` |
| `str` at 0x572c8 | store | 4 | `sp -0x20` | `ret(0x9c10c)` |
| `ldr` at 0x572cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x572d8 | load | 4 | `abs 0x57368` | `-` |
| `ldr` at 0x572dc | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x572e0 | load | 4 | `abs 0x5738c` | `-` |
| `ldr` at 0x572ec | load | 4 | `abs 0x57390` | `-` |
| `ldr` at 0x572f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x57304 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x57310 | load | 4 | `abs 0x57390` | `-` |
| `ldr` at 0x57314 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x57328 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x57334 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x57348 | load | 4 | `abs 0x57394` | `-` |
| `ldr` at 0x5734c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x57354 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x5735c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5735c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5735c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x56bd4 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe633c` | `[sp-20]` | `[0x11a42c]` |
| `bl` at 0x56e14 | `(unknown callee) 0x56518` | `0xe636c` | `sp-16` | `?` | `[sp-120]+1` |
| `bl` at 0x56f38 | `(unknown callee) 0x18920` | `0x18` | `sp-16` | `?` | `[sp-120]+1` |
| `bl` at 0x56f54 | `(unknown callee) 0x5e764` | `[0x11a3a8]` | `[0x11a3ac]` | `0x11a3a0` | `0x11a3a0` |
| `bl` at 0x56f6c | `(unknown callee) 0x18ae8` | `[sp-24]` | `ret(0x5e764)` | `0x18` | `ret(0x5e764)` |
| `bl` at 0x570ac | `(unknown callee) 0x18920` | `?` | `sp-16` | `[0x11a42c]` | `?` |
| `bl` at 0x570e0 | `(unknown callee) 0x18ae8` | `ret(0x18920)` | `[0x11a434]` | `?` | `?` |
| `bl` at 0x57184 | `(unknown callee) 0x5659c` | `ret(0x18ae8)` | `?` | `?` | `[sp-120]+1` |
| `bl` at 0x571d0 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe6384` | `[sp-20]` | `0x116928` |
| `bl` at 0x57270 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe6390` | `?` | `?` |
| `bl` at 0x57290 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe6338` | `?` | `0x116928` |
| `bl` at 0x572c0 | `(unknown callee) 0x9c10c` | `0x11a398` | `sp-28` | `0x0` | `0x0` |
| `bl` at 0x572e4 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe63a0` | `?` | `0x116928` |
| `bl` at 0x57338 | `(unknown callee) 0x9ba98` | `?` | `ret(0x9c10c)` | `0x50` | `?` |
| `bl` at 0x57350 | `(unknown callee) 0x1dba4` | `0xe63d4` | `[sp-20]` | `?` | `ret(0x9ba98)` |
| `bl` at 0x57358 | `(unknown callee) 0x9a998` | `[sp-32]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x56ba0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x56bb8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x56e24 | `[0x11a3b0] == 0x52` | `[0x11a3b0] != 0x52` |
| `beq` at 0x56e34 | `[0x11a3b0] == 0x44` | `[0x11a3b0] != 0x44` |
| `beq` at 0x56e6c | `? == 0x52` | `? != 0x52` |
| `beq` at 0x56ea0 | `? == 0x44` | `? != 0x44` |
| `beq` at 0x571a0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x571bc | `? == 0x0` | `? != 0x0` |
| `blt` at 0x571ec | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x572d4 | `ret(0x9c10c) != 0x0` | `ret(0x9c10c) == 0x0` |
| `beq` at 0x5730c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x57344 | `ret(0x9ba98) == 0x0` | `ret(0x9ba98) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5735c | `r0` | `?` |

## `L1CfgUpdate`

- `L1CfgUpdate`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x57398 to 0x57654.
  Established. Initial stack stores include a word 0x1 at sp -0x90 and a halfword 0x0 at sp -0x92
  (`str` at 0x573cc, `strh` at 0x573d4). After the call to 0x5e5a8, `beq` takes the zero-result path
  past a call to 0x5b7e4 (`bl` at 0x5755c, `beq` at 0x5756c, `bl` at 0x57588). A later call to
  0x189c8 supplies the word store at 0x11a0e8 (`bl` at 0x575b4, `str` at 0x575b8). The final call to
  0x9bf04 has candidates 0x11a398, 0x3e9, `[sp-152]` and sp-148 (`bl` at 0x5762c). The update format
  and numeric request role are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x573ac | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0x573b0 | load | 4 | `abs 0x57634` | `-` |
| `strb` at 0x573bc | store | 1 | `sp -0x94` | `0x0` |
| `strb` at 0x573c4 | store | 1 | `sp -0x93` | `0x0` |
| `str` at 0x573cc | store | 4 | `sp -0x90` | `0x1` |
| `strh` at 0x573d4 | store | 2 | `sp -0x92` | `0x0` |
| `ldr` at 0x573d8 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x573e0 | store | 4 | `sp -0x98` | `[sp-152]+1` |
| `ldr` at 0x573f4 | load | 4 | `abs 0x57638` | `-` |
| `str` at 0x57404 | store | 4 | `sp -0x9c` | `0x0` |
| `ldr` at 0x57408 | load | 4 | `abs 0x57638` | `-` |
| `ldr` at 0x5740c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x57410 | load | 4 | `abs 0x11a424` | `-` |
| `ldr` at 0x57420 | load | 4 | `abs 0x57638` | `-` |
| `ldr` at 0x57424 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x57440 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x57448 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x57454 | load | 4 | `abs 0x57638` | `-` |
| `ldr` at 0x57458 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x57474 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5747c | load | 4 | `(unknown)` | `-` |
| `str` at 0x5748c | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x57490 | load | 4 | `abs 0x57638` | `-` |
| `ldr` at 0x57494 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x574b0 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x574b8 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x574bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x574cc | load | 4 | `abs 0x57638` | `-` |
| `ldr` at 0x574d0 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x574ec | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x574f4 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x57508 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x57510 | load | 4 | `abs 0x57638` | `-` |
| `ldr` at 0x57514 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x57530 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x57538 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x5754c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x57554 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x57558 | load | 4 | `(unknown)` | `-` |
| `str` at 0x57560 | store | 4 | `sp -0xa4` | `ret(0x5e5a8)` |
| `ldr` at 0x57564 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x57580 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x5758c | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x57594 | store | 4 | `sp -0xa0` | `[sp-160]+1` |
| `ldr` at 0x5759c | load | 4 | `sp -0x9c` | `-` |
| `str` at 0x575a4 | store | 4 | `sp -0x9c` | `[sp-156]+1` |
| `ldr` at 0x575ac | load | 4 | `abs 0x5763c` | `-` |
| `str` at 0x575b8 | store | 4 | `abs 0x11a0e8` | `ret(0x189c8)` |
| `strb` at 0x575c4 | store | 1 | `sp -0x94` | `0x0` |
| `strb` at 0x575cc | store | 1 | `sp -0x93` | `0x0` |
| `ldr` at 0x575d0 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x575d8 | store | 4 | `sp -0x90` | `[sp-152]-1` |
| `strh` at 0x575e0 | store | 2 | `sp -0x92` | `0x0` |
| `ldr` at 0x575e4 | load | 4 | `abs 0x57640` | `-` |
| `ldr` at 0x575e8 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x57604 | load | 4 | `abs 0x57644` | `-` |
| `ldr` at 0x57608 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x5760c | load | 4 | `abs 0x57648` | `-` |
| `ldr` at 0x57610 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x57620 | load | 4 | `abs 0x5764c` | `-` |
| `ldr` at 0x57624 | load | 4 | `abs 0x57650` | `-` |
| `ldr` at 0x57628 | load | 4 | `sp -0x98` | `-` |
| `ldmdb` at 0x57630 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x57630 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x57630 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x57630 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x573b4 | `(unknown callee) 0x56518` | `0xe6404` | `r1` | `r2` | `0x0` |
| `bl` at 0x573fc | `(unknown callee) 0x5b7e4` | `sp-152` | `sp-148` | `0x11a3a0` | `0x1` |
| `bl` at 0x5755c | `(unknown callee) 0x5e5a8` | `?` | `?` | `?` | `?` |
| `bl` at 0x57588 | `(unknown callee) 0x5b7e4` | `sp-152` | `sp-148` | `ret(0x5e5a8)` | `0x1` |
| `bl` at 0x575b4 | `(unknown callee) 0x189c8` | `0x0` | `?` | `?` | `[0x11a424]` |
| `bl` at 0x575bc | `(unknown callee) 0x5659c` | `ret(0x189c8)` | `?` | `?` | `?` |
| `bl` at 0x575fc | `(unknown callee) 0x189c8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x57618 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe6410` | `[sp-152]` | `ret(0x189c8)` |
| `bl` at 0x5762c | `(unknown callee) 0x9bf04` | `0x11a398` | `0x3e9` | `[sp-152]` | `sp-148` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x57418 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x57450 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x57484 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x574c4 | `[sp-160] < ? (signed, 32-bit)` | `[sp-160] >= ? (signed, 32-bit)` |
| `beq` at 0x5756c | `ret(0x5e5a8) == 0x0` | `ret(0x5e5a8) != 0x0` |
| `beq` at 0x575f4 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x57630 | `r0` | `ret(0x9bf04)` |

## `L1CfgRingEvent`

- `L1CfgRingEvent`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x58ba4 to 0x58da4.
  Established. A word store at `[sp-20] +0x38` contains `[sp-24]` (`str` at 0x58c74). A later word
  store at `[sp-20] +0x30` contains `[[sp-20]+48]+1` (`str` at 0x58cd0). Two `bne` sites take paths
  past the final state-store arm for `[sp-28] != [0x11a3a8]` and `[sp-32] != [0x11a3ac]` (`bne` at
  0x58ce4, `bne` at 0x58cf8). That arm includes a word store of `[sp-44]` at `[sp-40] +0x0` (`str`
  at 0x58d40). The ring-event fields and state meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x58bb4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x58bb8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x58bbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58bc0 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x58bd0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x58bd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58bd8 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x58be4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x58be8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x58bec | load | 1 | `r0 +0x24` | `-` |
| `ldr` at 0x58bfc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58c00 | load | 4 | `r0 +0x28` | `-` |
| `str` at 0x58c04 | store | 4 | `sp -0x30` | `[r0+40]` |
| `ldr` at 0x58c0c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58c18 | store | 4 | `sp -0x30` | `ret(0x9ca84)` |
| `ldr` at 0x58c1c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x58c2c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x58c30 | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0x58c40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58c44 | load | 4 | `[sp-20] +0x30` | `-` |
| `ldr` at 0x58c54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58c6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58c70 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x58c74 | store | 4 | `[sp-20] +0x38` | `[sp-24]` |
| `ldr` at 0x58c78 | load | 4 | `abs 0x58d8c` | `-` |
| `ldr` at 0x58c7c | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x58c8c | load | 4 | `abs 0x58d90` | `-` |
| `ldr` at 0x58c90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58c94 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x58ca4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58ca8 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x58cb4 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x58cb8 | load | 4 | `abs 0x58d94` | `-` |
| `ldr` at 0x58cc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58cc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58cc8 | load | 4 | `[sp-20] +0x30` | `-` |
| `str` at 0x58cd0 | store | 4 | `[sp-20] +0x30` | `[[sp-20]+48]+1` |
| `ldr` at 0x58cd4 | load | 4 | `abs 0x58d98` | `-` |
| `ldr` at 0x58cd8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58cdc | load | 4 | `abs 0x11a3a8` | `-` |
| `ldr` at 0x58ce8 | load | 4 | `abs 0x58d98` | `-` |
| `ldr` at 0x58cec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x58cf0 | load | 4 | `abs 0x11a3ac` | `-` |
| `ldr` at 0x58cfc | load | 4 | `abs 0x58d9c` | `-` |
| `str` at 0x58d00 | store | 4 | `sp -0x28` | `0x11a0cc` |
| `ldr` at 0x58d04 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x58d08 | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0x58d18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58d1c | load | 4 | `[sp-20] +0x30` | `-` |
| `str` at 0x58d20 | store | 4 | `sp -0x2c` | `[[sp-20]+48]` |
| `ldr` at 0x58d28 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58d34 | store | 4 | `sp -0x2c` | `ret(0x9ca84)` |
| `ldr` at 0x58d38 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x58d3c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x58d40 | store | 4 | `[sp-40] +0x0` | `[sp-44]` |
| `ldr` at 0x58d44 | load | 4 | `abs 0x58d8c` | `-` |
| `ldr` at 0x58d48 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x58d58 | load | 4 | `abs 0x58d90` | `-` |
| `ldr` at 0x58d5c | load | 4 | `abs 0x58d9c` | `-` |
| `ldr` at 0x58d60 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x58d64 | load | 4 | `abs 0x58da0` | `-` |
| `ldr` at 0x58d68 | load | 4 | `abs 0x11a0cc` | `-` |
| `str` at 0x58d74 | store | 4 | `sp -0x24` | `?` |
| `str` at 0x58d80 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x58d84 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x58d88 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x58d88 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x58d88 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x58c14 | `(unknown callee) 0x9ca84` | `r0` | `0x0` | `r2` | `?` |
| `bl` at 0x58c5c | `(unknown callee) 0x9ca84` | `[sp-20]` | `0x1` | `[sp-48]` | `?` |
| `bl` at 0x58cbc | `(unknown callee) 0x21010` | `[0x116928]` | `0xe6660` | `?` | `?` |
| `bl` at 0x58d30 | `(unknown callee) 0x9ca84` | `[sp-20]` | `0x1` | `[sp-32]` | `?` |
| `bl` at 0x58d6c | `(unknown callee) 0x21010` | `[0x116928]` | `0xe6688` | `[0x11a0cc]` | `0x116928` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x58bf8 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x58c28 | `[sp-48]-1 > 0x1 (unsigned, 32-bit)` | `[sp-48]-1 <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0x58c3c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x58c4c | `[[sp-20]+48] != 0x0` | `[[sp-20]+48] == 0x0` |
| `bne` at 0x58c68 | `ret(0x9ca84) != 0x0` | `ret(0x9ca84) == 0x0` |
| `beq` at 0x58c88 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x58ce4 | `[sp-28] != [0x11a3a8]` | `[sp-28] == [0x11a3a8]` |
| `bne` at 0x58cf8 | `[sp-32] != [0x11a3ac]` | `[sp-32] == [0x11a3ac]` |
| `bne` at 0x58d14 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x58d54 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x58d88 | `r0` | `?` |

## `L1CfgPortEnable`

- `L1CfgPortEnable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x58da4 to 0x58e7c.
  Established. The first call targets 0x56518 with first candidate 0xe66a4 (`bl` at 0x58dbc). Later
  word stores write 0x1 and 0x0 to unknown effective targets at separate sites (`str` at 0x58e14,
  `str` at 0x58e44). The word read at 0x11a0d0 is incremented in the stored expression
  `[0x11a0d0]+1` back at that address (`ldr` at 0x58e50, `str` at 0x58e58). The last call targets
  0x5659c (`bl` at 0x58e5c). The physical port-enable and global-state roles are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x58db4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x58db8 | load | 4 | `abs 0x58e64` | `-` |
| `ldr` at 0x58dc0 | load | 4 | `abs 0x58e68` | `-` |
| `ldr` at 0x58dc4 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x58dd4 | load | 4 | `abs 0x58e6c` | `-` |
| `ldr` at 0x58dd8 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x58ddc | load | 4 | `abs 0x58e70` | `-` |
| `ldr` at 0x58de0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58de8 | load | 4 | `abs 0x58e74` | `-` |
| `ldr` at 0x58dec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58e08 | load | 4 | `abs 0x11a428` | `-` |
| `str` at 0x58e14 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x58e18 | load | 4 | `abs 0x58e74` | `-` |
| `ldr` at 0x58e1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58e38 | load | 4 | `abs 0x11a428` | `-` |
| `str` at 0x58e44 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x58e48 | load | 4 | `abs 0x58e78` | `-` |
| `ldr` at 0x58e4c | load | 4 | `abs 0x58e78` | `-` |
| `ldr` at 0x58e50 | load | 4 | `abs 0x11a0d0` | `-` |
| `str` at 0x58e58 | store | 4 | `abs 0x11a0d0` | `[0x11a0d0]+1` |
| `ldmdb` at 0x58e60 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x58e60 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x58e60 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x58dbc | `(unknown callee) 0x56518` | `0xe66a4` | `r1` | `r2` | `r3` |
| `bl` at 0x58de4 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe66b4` | `[sp-20]` | `0x116928` |
| `bl` at 0x58e5c | `(unknown callee) 0x5659c` | `?` | `0x11a3a0` | `0x11a0d0` | `[0x11a0d0]+1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x58dd0 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x58e60 | `r0` | `ret(0x5659c)` |

## `L1CfgFindBrick`

- `L1CfgFindBrick`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x59b44 to 0x59c60.
  Established. Two word stores initialize sp -0x24 and sp -0x28 to zero before the loop tests (`str`
  at 0x59b68, `str` at 0x59b70). The early `beq` takes the zero-entry-r0 path to the join (`beq` at
  0x59b88). Two later `bne` sites retain only Z == 0 and take paths past the word store of 0x1 at sp
  -0x28 (`bne` at 0x59bc8, `bne` at 0x59bf4, `str` at 0x59bfc). The final word saves at sp -0x2c
  include unknown values and a separate 0x0 (`str` at 0x59c38, `str` at 0x59c44, `str` at 0x59c50).
  Search-result interpretation and iteration count are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x59b54 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x59b58 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x59b5c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x59b60 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0x59b68 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x59b70 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x59b74 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x59b80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59b8c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59b90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x59ba0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59bb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59bbc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x59bc0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59bcc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59be0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59be8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x59bec | load | 4 | `sp -0x20` | `-` |
| `str` at 0x59bfc | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x59c00 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x59c08 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x59c10 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x59c1c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59c30 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x59c38 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x59c3c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x59c44 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0x59c50 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x59c54 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x59c5c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x59c5c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x59c5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x59b7c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x59b88 | `r0 == 0x0` | `r0 != 0x0` |
| `blt` at 0x59b98 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x59bc8 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x59bf4 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x59c18 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x59c5c | `r0` | `?` |

## `L1CfgFindState`

- `L1CfgFindState`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x59c60 to 0x59db8.
  Established. The first two `bne` sites compare entry r2 with `[0x11a3a8]` and entry r3 with
  `[0x11a3ac]` (`bne` at 0x59c90, `bne` at 0x59ca4). Falling through both reaches a word store of
  0x11a3a0 at sp -0x2c (`str` at 0x59cac). The other path later includes a word store of 0x1 at sp
  -0x28 and a store of `[sp-48]-112` at sp -0x30 (`str` at 0x59d4c, `str` at 0x59d94). The merged
  return is unknown (`ldmdb` at 0x59db0). The state-record interpretation remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x59c70 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x59c74 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x59c78 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x59c7c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x59c80 | load | 4 | `abs 0x59db4` | `-` |
| `ldr` at 0x59c84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59c88 | load | 4 | `abs 0x11a3a8` | `-` |
| `ldr` at 0x59c94 | load | 4 | `abs 0x59db4` | `-` |
| `ldr` at 0x59c98 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x59c9c | load | 4 | `abs 0x11a3ac` | `-` |
| `ldr` at 0x59ca8 | load | 4 | `abs 0x59db4` | `-` |
| `str` at 0x59cac | store | 4 | `sp -0x2c` | `0x11a3a0` |
| `str` at 0x59cb8 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x59cc0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x59cc4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x59cd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59cdc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59ce0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x59cf0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59d04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59d0c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x59d10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59d1c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59d30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59d38 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x59d3c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x59d4c | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x59d50 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x59d58 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x59d60 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x59d6c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59d80 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x59d88 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x59d8c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x59d94 | store | 4 | `sp -0x30` | `[sp-48]-112` |
| `str` at 0x59da0 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x59da4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x59da8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x59dac | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x59db0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x59db0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x59db0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x59c90 | `r2 != [0x11a3a8]` | `r2 == [0x11a3a8]` |
| `bne` at 0x59ca4 | `r3 != [0x11a3ac]` | `r3 == [0x11a3ac]` |
| `bne` at 0x59ccc | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x59cd8 | `Z == 1` | `not (Z == 1)` |
| `blt` at 0x59ce8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x59d18 | `? != [sp-28]` | `? == [sp-28]` |
| `bne` at 0x59d44 | `? != [sp-32]` | `? == [sp-32]` |
| `beq` at 0x59d68 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x59db0 | `r0` | `?` |

## `L1CfgGetL2`

- `L1CfgGetL2`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5e958 to 0x5e9d0.
  Established. The first `beq` takes the zero-entry-r0 path past the store of `[0x11a414]` at entry
  r0 +0x0 (`beq` at 0x5e97c, `str` at 0x5e98c). The next two word stores contain `[0x11a418]` and
  `[0x11a41c]`, each at an unknown effective target (`str` at 0x5e9a8, `str` at 0x5e9c4). Their
  deciding `beq` sites retain only Z == 1 (`beq` at 0x5e998, `beq` at 0x5e9b4). The return carries
  entry r0 (`ldmdb` at 0x5e9c8). The three global values' L2 meanings are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5e968 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x5e96c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x5e970 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x5e974 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5e980 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5e984 | load | 4 | `abs 0x5e9cc` | `-` |
| `ldr` at 0x5e988 | load | 4 | `abs 0x11a414` | `-` |
| `str` at 0x5e98c | store | 4 | `r0 +0x0` | `[0x11a414]` |
| `ldr` at 0x5e990 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5e99c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5e9a0 | load | 4 | `abs 0x5e9cc` | `-` |
| `ldr` at 0x5e9a4 | load | 4 | `abs 0x11a418` | `-` |
| `str` at 0x5e9a8 | store | 4 | `(unknown)` | `[0x11a418]` |
| `ldr` at 0x5e9ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5e9b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5e9bc | load | 4 | `abs 0x5e9cc` | `-` |
| `ldr` at 0x5e9c0 | load | 4 | `abs 0x11a41c` | `-` |
| `str` at 0x5e9c4 | store | 4 | `(unknown)` | `[0x11a41c]` |
| `ldmdb` at 0x5e9c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5e9c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5e9c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5e97c | `r0 == 0x0` | `r0 != 0x0` |
| `beq` at 0x5e998 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x5e9b4 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5e9c8 | `r0` | `r0` |

## `L1CfgProcessResponse`

- `L1CfgProcessResponse`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5c49c to 0x5c56c.
  Established. The byte loaded at entry r0 +0x13 decides `bne`, whose taken nonzero path reaches the
  return (`ldrb` at 0x5c4b4, `bne` at 0x5c4bc, `ldmdb` at 0x5c558). A later `beq` takes the `[sp-36]
  == 0x3e8` path to 0x189c8 and a word store of its result at 0x11a0e4 (`beq` at 0x5c504, `bl`
  at 0x5c540, `str` at 0x5c544). Unequal fall-through instead reaches 0x58e7c with first candidate
  `[sp-24]` and second candidate 0x0 (`bl` at 0x5c554). The response-code meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5c4ac | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x5c4b0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5c4b4 | load | 1 | `r0 +0x13` | `-` |
| `ldr` at 0x5c4c0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5c4c4 | load | 1 | `r0 +0x24` | `-` |
| `ldr` at 0x5c4d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5c4d8 | load | 4 | `r0 +0x28` | `-` |
| `str` at 0x5c4dc | store | 4 | `sp -0x24` | `[r0+40]` |
| `ldr` at 0x5c4e4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5c4f0 | store | 4 | `sp -0x24` | `ret(0x9ca84)` |
| `ldr` at 0x5c4f4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5c4f8 | store | 4 | `sp -0x20` | `[sp-36]` |
| `ldr` at 0x5c4fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5c50c | load | 4 | `abs 0x5c55c` | `-` |
| `ldr` at 0x5c510 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x5c520 | load | 4 | `abs 0x5c560` | `-` |
| `ldr` at 0x5c524 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5c528 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x5c52c | load | 4 | `abs 0x5c564` | `-` |
| `ldr` at 0x5c530 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x5c538 | load | 4 | `abs 0x5c568` | `-` |
| `str` at 0x5c544 | store | 4 | `abs 0x11a0e4` | `ret(0x189c8)` |
| `ldr` at 0x5c54c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5c558 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x5c558 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5c558 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5c558 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5c4ec | `(unknown callee) 0x9ca84` | `r0` | `0x0` | `r2` | `?` |
| `bl` at 0x5c534 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe69a4` | `[[sp-24]+12]` | `0x116928` |
| `bl` at 0x5c540 | `(unknown callee) 0x189c8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x5c554 | `(unknown callee) 0x58e7c` | `[sp-24]` | `0x0` | `?` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5c4bc | `[r0+19]:1 != 0x0` | `[r0+19]:1 == 0x0` |
| `bne` at 0x5c4d0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x5c504 | `[sp-36] == 0x3e8` | `[sp-36] != 0x3e8` |
| `beq` at 0x5c51c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5c558 | `r0` | `?` |
