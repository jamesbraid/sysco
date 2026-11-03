# ARM FRU capture and event helpers in 1.44.0

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
Computed transfers leave unresolved edges in the initialization and status routines, identified below. Tables cover known control-flow edges only.
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/fru-capture-event-helpers-facts.txt) retain all tool sections.

## `L1FRU_2m_Init`

- `L1FRU_2m_Init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x99bb4 to 0x99e30.
  Established. A nonzero `[0x133ab4]` takes `0x99d60`, while zero falls through (`bne` at 0x99bd0).
  Included word stores write `[0x133aac]+1` to `0x133aac` and `0x3` to `0x29a94c` on known edges (`str` at 0x99d9c, `str` at 0x99db8).
  The unnamed `0x19748` call carries `0x29a944`, `0x0`, `0x2c`, `[0x133ab4]` (`bl` at 0x99be0).
  Board-specific initialization effects behind the computed transfer remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x99bc4 | load | 4 | `abs 0x99ddc` | `-` |
| `ldr` at 0x99bc8 | load | 4 | `abs 0x133ab4` | `-` |
| `ldr` at 0x99bd4 | load | 4 | `abs 0x99de0` | `-` |
| `ldrls` at 0x99bf8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99d60 | load | 4 | `abs 0x99e24` | `-` |
| `ldr` at 0x99d64 | load | 4 | `abs 0x133aac` | `-` |
| `ldr` at 0x99d70 | load | 4 | `abs 0x99e28` | `-` |
| `str` at 0x99d74 | store | 4 | `sp -0x1c` | `0x112390` |
| `ldr` at 0x99d7c | load | 4 | `abs 0x99e2c` | `-` |
| `ldr` at 0x99d8c | load | 4 | `abs 0x99e24` | `-` |
| `ldr` at 0x99d90 | load | 4 | `abs 0x99e24` | `-` |
| `ldr` at 0x99d94 | load | 4 | `abs 0x133aac` | `-` |
| `str` at 0x99d9c | store | 4 | `abs 0x133aac` | `[0x133aac]+1` |
| `ldr` at 0x99da0 | load | 4 | `abs 0x99de0` | `-` |
| `ldr` at 0x99da4 | load | 4 | `abs 0x29a94c` | `-` |
| `ldr` at 0x99db0 | load | 4 | `abs 0x99de0` | `-` |
| `str` at 0x99db8 | store | 4 | `abs 0x29a94c` | `0x3` |
| `ldr` at 0x99dc0 | load | 4 | `abs 0x99e24` | `-` |
| `ldr` at 0x99dc4 | load | 4 | `abs 0x99e24` | `-` |
| `ldr` at 0x99dc8 | load | 4 | `abs 0x133aac` | `-` |
| `str` at 0x99dd0 | store | 4 | `abs 0x133aac` | `[0x133aac]+1` |
| `ldm` at 0x99dd8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x99dd8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x99dd8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x99dd8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x99be0 | `(unknown callee) 0x19748` | `0x29a944` | `0x0` | `0x2c` | `[0x133ab4]` |
| `bl` at 0x99be4 | `(unknown callee) 0x488cc` | `ret(0x19748)` | `?` | `?` | `?` |
| `bl` at 0x99d88 | `(unknown callee) 0x200a0` | `0x9` | `0x1123b0` | `0x7e` | `0x0` |
| `bl` at 0x99dbc | `(unknown callee) 0xb0830` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x99bd0 | `[0x133ab4] != 0x0` | `[0x133ab4] == 0x0` |
| `ldrls` at 0x99bf8 | `? <= 0x1a (unsigned, 32-bit)` | `? > 0x1a (unsigned, 32-bit)` |
| `beq` at 0x99d6c | `[0x133aac] == 0x0` | `[0x133aac] != 0x0` |
| `bne` at 0x99dac | `[0x29a94c] != 0x0` | `[0x29a94c] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x99dd8 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x99bf8).

## `L1FRU_2m_Capture`

- `L1FRU_2m_Capture`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x99e94 to 0x9a094.
  Established. Included word stores write `0x1` and `0x2` to `0x29a954`, and clear `0x29a94c` (`str` at 0x99f28, `str` at 0x99f38, `str` at 0x99f44).
  A nonzero `[0x29a94c]` takes `0x9a01c`, while zero falls through (`bne` at 0x99f5c).
  FRU capture mode coding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x99ea4 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x99ea8 | load | 4 | `abs 0x9a090` | `-` |
| `str` at 0x99eb4 | store | 4 | `abs 0x29a950` | `ret(0x194f0)` |
| `ldr` at 0x99eb8 | load | 4 | `abs 0x9a090` | `-` |
| `ldr` at 0x99ebc | load | 4 | `abs 0x29a964` | `-` |
| `ldr` at 0x99ec8 | load | 4 | `abs 0x9a090` | `-` |
| `ldr` at 0x99ecc | load | 4 | `abs 0x29a968` | `-` |
| `str` at 0x99ee0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x99ee8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x99ef4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x99f08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x99f20 | load | 4 | `abs 0x9a090` | `-` |
| `str` at 0x99f28 | store | 4 | `abs 0x29a954` | `0x1` |
| `ldr` at 0x99f30 | load | 4 | `abs 0x9a090` | `-` |
| `str` at 0x99f38 | store | 4 | `abs 0x29a954` | `0x2` |
| `ldr` at 0x99f3c | load | 4 | `abs 0x9a090` | `-` |
| `str` at 0x99f44 | store | 4 | `abs 0x29a94c` | `0x0` |
| `str` at 0x99f4c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x99f50 | load | 4 | `abs 0x9a090` | `-` |
| `ldr` at 0x99f54 | load | 4 | `abs 0x29a94c` | `-` |
| `ldr` at 0x99f60 | load | 4 | `abs 0x9a090` | `-` |
| `ldr` at 0x99f64 | load | 4 | `abs 0x29a958` | `-` |
| `ldr` at 0x99f68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x99f74 | load | 4 | `abs 0x9a090` | `-` |
| `ldr` at 0x99f78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x99f8c | load | 4 | `abs 0x29a95c` | `-` |
| `ldr` at 0x99f94 | load | 4 | `(unknown)` | `-` |
| `str` at 0x99f98 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x99f9c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x99fa8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x99fb8 | load | 4 | `abs 0x9a090` | `-` |
| `ldr` at 0x99fbc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x99fd0 | load | 4 | `abs 0x29a95c` | `-` |
| `ldr` at 0x99fe4 | load | 4 | `abs 0x9a090` | `-` |
| `ldr` at 0x99fe8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x99ffc | load | 4 | `abs 0x29a95c` | `-` |
| `ldr` at 0x9a00c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x9a014 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x9a01c | load | 4 | `abs 0x9a090` | `-` |
| `ldr` at 0x9a020 | load | 4 | `abs 0x29a964` | `-` |
| `ldr` at 0x9a02c | load | 4 | `abs 0x9a090` | `-` |
| `ldr` at 0x9a030 | load | 4 | `abs 0x29a964` | `-` |
| `ldr` at 0x9a03c | load | 4 | `abs 0x9a090` | `-` |
| `ldr` at 0x9a040 | load | 4 | `abs 0x29a968` | `-` |
| `ldr` at 0x9a04c | load | 4 | `abs 0x9a090` | `-` |
| `ldr` at 0x9a050 | load | 4 | `abs 0x29a968` | `-` |
| `ldr` at 0x9a05c | load | 4 | `abs 0x9a090` | `-` |
| `ldr` at 0x9a060 | load | 4 | `abs 0x29a96c` | `-` |
| `ldr` at 0x9a06c | load | 4 | `abs 0x9a090` | `-` |
| `ldr` at 0x9a070 | load | 4 | `abs 0x29a96c` | `-` |
| `str` at 0x9a080 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x9a084 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x9a08c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x9a08c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9a08c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9a08c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x99eb0 | `(unknown callee) 0x194f0` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x99fdc | `(unknown callee) 0x9b600` | `?` | `0x29a944` | `?` | `?` |
| `bl` at 0x9a008 | `(unknown callee) 0x9c704` | `?` | `0x29a944` | `?` | `?` |
| `bl` at 0x9a034 | `(unknown callee) 0x1970c` | `[0x29a964]` | `?` | `?` | `0x29a944` |
| `bl` at 0x9a038 | `(unknown callee) 0x9b5a0` | `?` | `?` | `?` | `?` |
| `bl` at 0x9a054 | `(unknown callee) 0x1970c` | `[0x29a968]` | `?` | `?` | `0x29a944` |
| `bl` at 0x9a058 | `(unknown callee) 0x9b5c0` | `?` | `?` | `?` | `?` |
| `bl` at 0x9a074 | `(unknown callee) 0x1970c` | `[0x29a96c]` | `?` | `?` | `0x29a944` |
| `bl` at 0x9a078 | `(unknown callee) 0x9b5e0` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x99ec4 | `[0x29a964] != 0x0` | `[0x29a964] == 0x0` |
| `bne` at 0x99ed4 | `[0x29a968] != 0x0` | `[0x29a968] == 0x0` |
| `blt` at 0x99ef0 | `[sp-24] < 0x0 (signed, 32-bit)` | `[sp-24] >= 0x0 (signed, 32-bit)` |
| `bne` at 0x99f04 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x99f1c | `? != 0x5` | `? == 0x5` |
| `bne` at 0x99f5c | `[0x29a94c] != 0x0` | `[0x29a94c] == 0x0` |
| `ble` at 0x99f70 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x99fa4 | `[sp-36] == 0x1` | `[sp-36] != 0x1` |
| `beq` at 0x99fb0 | `[sp-36] == 0x2` | `[sp-36] != 0x2` |
| `beq` at 0x9a028 | `[0x29a964] == 0x0` | `[0x29a964] != 0x0` |
| `beq` at 0x9a048 | `[0x29a968] == 0x0` | `[0x29a968] != 0x0` |
| `beq` at 0x9a068 | `[0x29a96c] == 0x0` | `[0x29a96c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9a08c | `r0` | `[sp-32]` |

## `L1FRU_2m_Deadlock`

- `L1FRU_2m_Deadlock`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9a094 to 0x9a14c.
  Established. The unnamed `0x9bb90` call carries unknown, `[sp-20]`, `[sp-24]`, unknown (`bl` at 0x9a124).
  A word store increments `sp -0x1c` with `[sp-28]+1` (`str` at 0x9a130).
  The integer return candidate is `0x0` (`ldm` at 0x9a144).
  FRU deadlock test criteria remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9a0a4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9a0a8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x9a0b0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x9a0b4 | load | 4 | `abs 0x9a148` | `-` |
| `ldr` at 0x9a0b8 | load | 4 | `abs 0x29a958` | `-` |
| `ldr` at 0x9a0bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9a0c8 | load | 4 | `abs 0x9a148` | `-` |
| `ldr` at 0x9a0cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9a0e0 | load | 4 | `abs 0x29a95c` | `-` |
| `ldr` at 0x9a0e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a0f8 | load | 4 | `abs 0x9a148` | `-` |
| `ldr` at 0x9a0fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9a110 | load | 4 | `abs 0x29a95c` | `-` |
| `ldr` at 0x9a118 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a11c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9a120 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9a128 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x9a130 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldm` at 0x9a144 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9a144 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9a144 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9a124 | `(unknown callee) 0x9bb90` | `?` | `[sp-20]` | `[sp-24]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x9a0c4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x9a0f0 | `? == 0x1` | `? != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9a144 | `r0` | `0x0` |

## `L1FRU_2m_Dump`

- `L1FRU_2m_Dump`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9a14c to 0x9a17c.
  Established. The unnamed `0xb7a10` call carries entry `r0`, `0x0`, entry `r1`, entry `r3` (`bl` at 0x9a170).
  The return record preserves `ret(0xb7a10)` (`ldm` at 0x9a178).
  FRU dump output routing remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9a15c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9a160 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x9a164 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9a16c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x9a178 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9a178 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9a178 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9a170 | `(unknown callee) 0xb7a10` | `r0` | `0x0` | `r1` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9a178 | `r0` | `ret(0xb7a10)` |

## `L1FRU_2m_Examine`

- `L1FRU_2m_Examine`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9ad54 to 0x9b3fc.
  Established. The signed comparison `[sp-76] > [sp-176]` takes `0x9b280`, while less or equal falls through (`bgt` at 0x9b268).
  Word stores copy `[0x133aa0]` to `[sp-24] +0x0` and clear `0x133aa0` (`str` at 0x9b39c, `str` at 0x9b3a8).
  The unnamed `0xb3c24` call carries `0x29a960`, unknown, `0x133aa0`, `0x0` (`bl` at 0x9b3b0).
  FRU examination ranking policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9ad64 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9ad68 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x9ad70 | store | 4 | `sp -0x44` | `0x0` |
| `str` at 0x9ad78 | store | 4 | `sp -0x48` | `?` |
| `str` at 0x9ad80 | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0x9ad88 | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0x9ad90 | store | 4 | `sp -0x54` | `0x0` |
| `str` at 0x9ada0 | store | 4 | `sp -0xac` | `ret(0x6a0f8)` |
| `ldr` at 0x9ada8 | load | 4 | `abs 0x9b3bc` | `-` |
| `ldr` at 0x9adac | load | 4 | `abs 0x29a94c` | `-` |
| `ldr` at 0x9adbc | load | 4 | `abs 0x9b3bc` | `-` |
| `ldr` at 0x9adc0 | load | 4 | `abs 0x29a954` | `-` |
| `ldr` at 0x9adcc | load | 4 | `sp -0x50` | `-` |
| `str` at 0x9add4 | store | 4 | `sp -0x50` | `[sp-80]+1` |
| `ldr` at 0x9addc | load | 4 | `abs 0x9b3bc` | `-` |
| `ldr` at 0x9ade0 | load | 4 | `abs 0x29a954` | `-` |
| `ldr` at 0x9adec | load | 4 | `sp -0x54` | `-` |
| `str` at 0x9adf4 | store | 4 | `sp -0x54` | `[sp-84]+1` |
| `ldr` at 0x9adf8 | load | 4 | `abs 0x9b3bc` | `-` |
| `ldr` at 0x9adfc | load | 4 | `abs 0x29a950` | `-` |
| `str` at 0x9ae00 | store | 4 | `sp -0x48` | `[0x29a950]` |
| `ldr` at 0x9ae04 | load | 4 | `abs 0x9b3bc` | `-` |
| `ldr` at 0x9ae08 | load | 4 | `abs 0x29a950` | `-` |
| `str` at 0x9ae0c | store | 4 | `sp -0x4c` | `[0x29a950]` |
| `ldr` at 0x9ae10 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x9ae18 | store | 4 | `sp -0x44` | `[sp-68]+1` |
| `ldr` at 0x9ae20 | load | 4 | `abs 0x9b3bc` | `-` |
| `ldr` at 0x9ae24 | load | 4 | `abs 0x29a94c` | `-` |
| `ldr` at 0x9ae34 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x9ae38 | load | 4 | `sp -0xac` | `-` |
| `str` at 0x9ae3c | store | 4 | `sp -0xcc` | `ret(0xb7424)` |
| `ldr` at 0x9ae44 | load | 4 | `abs 0x9b3c0` | `-` |
| `ldr` at 0x9ae48 | load | 4 | `[sp-172] +0x8` | `-` |
| `ldr` at 0x9ae4c | load | 4 | `[sp-172] +0xc` | `-` |
| `str` at 0x9ae6c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x9ae70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9ae7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9ae80 | load | 4 | `[sp-20] +0x414` | `-` |
| `ldr` at 0x9ae84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9ae90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9ae94 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9ae98 | load | 4 | `abs 0x9b3c4` | `-` |
| `ldr` at 0x9aea8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9aeac | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x9aeb0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x9aeb4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x9aec4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x9aec8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x9aedc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x9aee0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x9aef0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9aef4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9aef8 | store | 4 | `sp -0xb4` | `?` |
| `ldr` at 0x9af00 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x9af0c | store | 4 | `sp -0xb4` | `ret(0xaadf4)` |
| `str` at 0x9af20 | store | 4 | `sp -0xcc` | `sp-48` |
| `str` at 0x9af28 | store | 4 | `sp -0xc8` | `sp-52` |
| `str` at 0x9af30 | store | 4 | `sp -0xc4` | `sp-56` |
| `str` at 0x9af38 | store | 4 | `sp -0xc0` | `sp-60` |
| `str` at 0x9af40 | store | 4 | `sp -0xbc` | `sp-64` |
| `str` at 0x9af48 | store | 4 | `sp -0xb8` | `sp-168` |
| `ldr` at 0x9af4c | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x9af60 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x9af6c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x9af7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9af80 | load | 4 | `[sp-28] +0x1c` | `-` |
| `ldr` at 0x9af90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9af94 | load | 4 | `[sp-28] +0x1c` | `-` |
| `str` at 0x9afa0 | store | 4 | `sp -0xcc` | `ret(0xb7424)` |
| `ldr` at 0x9afa4 | load | 4 | `abs 0x9b3c0` | `-` |
| `ldr` at 0x9afc0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x9afd0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x9afd8 | store | 4 | `sp -0x44` | `[sp-68]+1` |
| `ldr` at 0x9afdc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9afe0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9afec | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9aff0 | store | 4 | `sp -0x48` | `[sp-40]` |
| `ldr` at 0x9aff4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9aff8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x9b004 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9b008 | store | 4 | `sp -0x4c` | `[sp-40]` |
| `ldr` at 0x9b00c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9b018 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x9b020 | store | 4 | `sp -0x50` | `[sp-80]+1` |
| `ldr` at 0x9b028 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9b034 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x9b03c | store | 4 | `sp -0x54` | `[sp-84]+1` |
| `ldr` at 0x9b040 | load | 4 | `abs 0x9b3c8` | `-` |
| `ldr` at 0x9b044 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x9b04c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9b054 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x9b05c | load | 4 | `abs 0x9b3bc` | `-` |
| `ldr` at 0x9b060 | load | 4 | `abs 0x29a960` | `-` |
| `ldr` at 0x9b068 | load | 4 | `abs 0x9b3bc` | `-` |
| `ldr` at 0x9b070 | load | 4 | `abs 0x9b3cc` | `-` |
| `ldr` at 0x9b078 | load | 4 | `abs 0x29a960` | `-` |
| `ldr` at 0x9b080 | load | 4 | `abs 0x9b3bc` | `-` |
| `ldr` at 0x9b088 | load | 4 | `abs 0x9b3cc` | `-` |
| `ldr` at 0x9b090 | load | 4 | `abs 0x29a960` | `-` |
| `ldr` at 0x9b098 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x9b0ac | store | 4 | `sp -0xb0` | `ret(0x194f0)` |
| `ldr` at 0x9b0b0 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x9b0bc | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x9b0cc | load | 4 | `abs 0x9b3d0` | `-` |
| `ldr` at 0x9b0dc | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x9b0ec | load | 4 | `abs 0x9b3d4` | `-` |
| `ldr` at 0x9b100 | load | 4 | `abs 0x9b3d8` | `-` |
| `ldr` at 0x9b128 | load | 4 | `abs 0x9b3dc` | `-` |
| `ldr` at 0x9b164 | load | 4 | `abs 0x9b3e0` | `-` |
| `ldr` at 0x9b188 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x9b18c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9b194 | load | 4 | `abs 0x9b3e4` | `-` |
| `ldr` at 0x9b1ac | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x9b1b0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9b1b8 | load | 4 | `abs 0x9b3e8` | `-` |
| `ldr` at 0x9b1d0 | load | 4 | `abs 0x9b3e8` | `-` |
| `ldr` at 0x9b1fc | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x9b200 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9b208 | load | 4 | `abs 0x9b3e8` | `-` |
| `str` at 0x9b234 | store | 4 | `sp -0xcc` | `?` |
| `ldr` at 0x9b238 | load | 4 | `abs 0x9b3ec` | `-` |
| `ldr` at 0x9b25c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x9b260 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x9b26c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9b270 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x9b284 | load | 4 | `sp -0xb0` | `-` |
| `str` at 0x9b288 | store | 4 | `sp -0xcc` | `[sp-176]` |
| `ldr` at 0x9b290 | load | 4 | `abs 0x9b3f0` | `-` |
| `ldr` at 0x9b294 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9b298 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x9b2bc | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x9b2c0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x9b2c8 | load | 4 | `abs 0x9b3e4` | `-` |
| `ldr` at 0x9b2e0 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x9b2e4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x9b2ec | load | 4 | `abs 0x9b3e8` | `-` |
| `ldr` at 0x9b304 | load | 4 | `abs 0x9b3e8` | `-` |
| `ldr` at 0x9b330 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x9b334 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x9b33c | load | 4 | `abs 0x9b3e8` | `-` |
| `str` at 0x9b368 | store | 4 | `sp -0xcc` | `?` |
| `ldr` at 0x9b36c | load | 4 | `abs 0x9b3f4` | `-` |
| `ldr` at 0x9b390 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9b394 | load | 4 | `abs 0x9b3f8` | `-` |
| `ldr` at 0x9b398 | load | 4 | `abs 0x133aa0` | `-` |
| `str` at 0x9b39c | store | 4 | `[sp-24] +0x0` | `[0x133aa0]` |
| `ldr` at 0x9b3a0 | load | 4 | `abs 0x9b3f8` | `-` |
| `str` at 0x9b3a8 | store | 4 | `abs 0x133aa0` | `0x0` |
| `ldr` at 0x9b3ac | load | 4 | `abs 0x9b3c8` | `-` |
| `ldm` at 0x9b3b8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9b3b8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9b3b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9ad98 | `(unknown callee) 0x6a0f8` | `0x0` | `r1` | `r2` | `0x0` |
| `bl` at 0x9ada4 | `(unknown callee) 0x9ac00` | `ret(0x6a0f8)` | `?` | `?` | `ret(0x6a0f8)` |
| `bl` at 0x9ae28 | `(unknown callee) 0xb7424` | `[0x29a94c]` | `?` | `?` | `0x29a944` |
| `bl` at 0x9ae50 | `(unknown callee) 0x197a8` | `sp-164` | `0x1123cc` | `[[sp-172]+8]` | `[[sp-172]+12]` |
| `bl` at 0x9ae64 | `(unknown callee) 0x9ac48` | `0x0` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x9af08 | `(unknown callee) 0xaadf4` | `?` | `0x1` | `[sp-20]` | `?` |
| `bl` at 0x9af5c | `(unknown callee) 0xb6e88` | `[sp-180]` | `sp-36` | `sp-40` | `sp-44` |
| `bl` at 0x9af70 | `(unknown callee) 0xb7424` | `[sp-60]` | `?` | `?` | `[sp-60]` |
| `bl` at 0x9afa8 | `(unknown callee) 0x197a8` | `sp-164` | `0x1123cc` | `?` | `?` |
| `bl` at 0x9afbc | `(unknown callee) 0x9ac48` | `0x0` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x9b048 | `(unknown callee) 0xb3d90` | `0x29a960` | `[sp-168]` | `[sp-40]` | `?` |
| `bl` at 0x9b064 | `(unknown callee) 0xb3f94` | `[0x29a960]` | `?` | `?` | `0x29a944` |
| `bl` at 0x9b07c | `(unknown callee) 0xb427c` | `0x0` | `0x9ac48` | `0x0` | `[0x29a960]` |
| `bl` at 0x9b094 | `(unknown callee) 0xb427c` | `0x2` | `0x9ac48` | `0x0` | `[0x29a960]` |
| `bl` at 0x9b0a8 | `(unknown callee) 0x194f0` | `0x0` | `?` | `?` | `[sp-68]` |
| `bl` at 0x9b0d4 | `(unknown callee) 0x9ac48` | `0x0` | `0x1123e4` | `?` | `[sp-84]` |
| `bl` at 0x9b0f4 | `(unknown callee) 0x9ac48` | `0x0` | `0x11241c` | `?` | `[sp-80]` |
| `bl` at 0x9b108 | `(unknown callee) 0x9ac48` | `0x0` | `0x112444` | `?` | `?` |
| `bl` at 0x9b114 | `(unknown callee) 0x192f8` | `sp-72` | `?` | `?` | `sp-72` |
| `bl` at 0x9b130 | `(unknown callee) 0x19454` | `sp-164` | `0x3c` | `0x11246c` | `ret(0x192f8)` |
| `bl` at 0x9b144 | `(unknown callee) 0x9ac48` | `0x0` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x9b150 | `(unknown callee) 0x192f8` | `sp-76` | `?` | `?` | `sp-76` |
| `bl` at 0x9b16c | `(unknown callee) 0x19454` | `sp-164` | `0x3c` | `0x11248c` | `ret(0x192f8)` |
| `bl` at 0x9b180 | `(unknown callee) 0x9ac48` | `0x0` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x9b244 | `(unknown callee) 0x197a8` | `sp-164` | `0x1124ac` | `?` | `?` |
| `bl` at 0x9b258 | `(unknown callee) 0x9ac48` | `0x0` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x9b29c | `(unknown callee) 0x197a8` | `sp-164` | `0x1124c8` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x9b2b0 | `(unknown callee) 0x9ac48` | `0x0` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x9b378 | `(unknown callee) 0x197a8` | `sp-164` | `0x1124ec` | `?` | `?` |
| `bl` at 0x9b38c | `(unknown callee) 0x9ac48` | `0x0` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x9b3b0 | `(unknown callee) 0xb3c24` | `0x29a960` | `?` | `0x133aa0` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x9adb4 | `[0x29a94c] == 0x0` | `[0x29a94c] != 0x0` |
| `bne` at 0x9adc8 | `[0x29a954] != 0x1` | `[0x29a954] == 0x1` |
| `bne` at 0x9ade8 | `[0x29a954] != 0x2` | `[0x29a954] == 0x2` |
| `beq` at 0x9ae78 | `[sp-20] == 0x0` | `[sp-20] != 0x0` |
| `ble` at 0x9ae8c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x9aebc | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x9aed4 | `? == 0x1` | `? != 0x1` |
| `bne` at 0x9aeec | `? != 0x0` | `? == 0x0` |
| `beq` at 0x9af68 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `beq` at 0x9afc8 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `bge` at 0x9afe8 | `[sp-40] >= [sp-72] (signed, 32-bit)` | `[sp-40] < [sp-72] (signed, 32-bit)` |
| `ble` at 0x9b000 | `[sp-40] <= [sp-76] (signed, 32-bit)` | `[sp-40] > [sp-76] (signed, 32-bit)` |
| `bne` at 0x9b014 | `[sp-44] != 0x1` | `[sp-44] == 0x1` |
| `bne` at 0x9b030 | `[sp-44] != 0x2` | `[sp-44] == 0x2` |
| `beq` at 0x9b0a0 | `[sp-68] == 0x0` | `[sp-68] != 0x0` |
| `beq` at 0x9b0b8 | `[sp-80] == 0x0` | `[sp-80] != 0x0` |
| `beq` at 0x9b0c4 | `[sp-84] == 0x0` | `[sp-84] != 0x0` |
| `beq` at 0x9b0e4 | `[sp-80] == 0x0` | `[sp-80] != 0x0` |
| `bgt` at 0x9b268 | `[sp-76] > [sp-176] (signed, 32-bit)` | `[sp-76] <= [sp-176] (signed, 32-bit)` |
| `bgt` at 0x9b278 | `[sp-72] > [sp-176] (signed, 32-bit)` | `[sp-72] <= [sp-176] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9b3b8 | `r0` | `ret(0xb3c24)` |

## `L1FRU_2m_Attach`

- `L1FRU_2m_Attach`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9b3fc to 0x9b5a0.
  Established. Inequality `[sp-20] != 0x1` takes `0x9b500`, while equality falls through (`bne` at 0x9b448).
  The unnamed `0xb51ec` call carries `[sp-48]`, `[sp-40]`, `[sp-44]`, `[0x29a94c]` (`bl` at 0x9b4d0).
  A later unnamed `0x5ebd0` call carries `sp-24`, `[sp-28]`, `[sp-32]`, `[sp-36]` (`bl` at 0x9b578).
  FRU attachment format remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9b40c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9b410 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x9b414 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x9b420 | store | 4 | `sp -0x28` | `ret(0x1d444)` |
| `str` at 0x9b42c | store | 4 | `sp -0x2c` | `ret(0x1d444)` |
| `str` at 0x9b43c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x9b440 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9b44c | load | 4 | `abs 0x9b594` | `-` |
| `ldr` at 0x9b450 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9b458 | load | 4 | `abs 0x9b598` | `-` |
| `ldr` at 0x9b45c | load | 4 | `abs 0x9b598` | `-` |
| `ldr` at 0x9b460 | load | 4 | `abs 0x29a950` | `-` |
| `str` at 0x9b464 | store | 4 | `sp -0x58` | `[0x29a950]` |
| `ldr` at 0x9b468 | load | 4 | `abs 0x9b598` | `-` |
| `ldr` at 0x9b46c | load | 4 | `abs 0x29a954` | `-` |
| `str` at 0x9b470 | store | 4 | `sp -0x54` | `[0x29a954]` |
| `ldr` at 0x9b474 | load | 4 | `abs 0x9b598` | `-` |
| `ldr` at 0x9b478 | load | 4 | `abs 0x29a958` | `-` |
| `str` at 0x9b47c | store | 4 | `sp -0x50` | `[0x29a958]` |
| `ldr` at 0x9b480 | load | 4 | `abs 0x9b598` | `-` |
| `ldr` at 0x9b484 | load | 4 | `abs 0x29a95c` | `-` |
| `str` at 0x9b488 | store | 4 | `sp -0x4c` | `[0x29a95c]` |
| `ldr` at 0x9b48c | load | 4 | `abs 0x9b598` | `-` |
| `ldr` at 0x9b490 | load | 4 | `abs 0x29a964` | `-` |
| `str` at 0x9b494 | store | 4 | `sp -0x48` | `[0x29a964]` |
| `ldr` at 0x9b498 | load | 4 | `abs 0x9b598` | `-` |
| `ldr` at 0x9b49c | load | 4 | `abs 0x29a968` | `-` |
| `str` at 0x9b4a0 | store | 4 | `sp -0x44` | `[0x29a968]` |
| `ldr` at 0x9b4a4 | load | 4 | `abs 0x9b598` | `-` |
| `ldr` at 0x9b4a8 | load | 4 | `abs 0x29a96c` | `-` |
| `str` at 0x9b4ac | store | 4 | `sp -0x40` | `[0x29a96c]` |
| `str` at 0x9b4b4 | store | 4 | `sp -0x3c` | `sp-32` |
| `str` at 0x9b4bc | store | 4 | `sp -0x38` | `sp-36` |
| `ldr` at 0x9b4c0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x9b4c4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9b4c8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9b4cc | load | 4 | `abs 0x29a94c` | `-` |
| `ldr` at 0x9b4d8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9b4dc | store | 4 | `sp -0x58` | `[sp-36]` |
| `ldr` at 0x9b4e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9b4ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9b4f4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9b4f8 | store | 4 | `sp -0x34` | `[sp-24]` |
| `ldr` at 0x9b500 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9b50c | load | 4 | `abs 0x9b59c` | `-` |
| `ldr` at 0x9b510 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9b518 | load | 4 | `abs 0x9b598` | `-` |
| `ldr` at 0x9b51c | load | 4 | `abs 0x9b598` | `-` |
| `ldr` at 0x9b520 | load | 4 | `abs 0x29a950` | `-` |
| `str` at 0x9b524 | store | 4 | `sp -0x58` | `[0x29a950]` |
| `ldr` at 0x9b528 | load | 4 | `abs 0x9b598` | `-` |
| `ldr` at 0x9b52c | load | 4 | `abs 0x29a954` | `-` |
| `str` at 0x9b530 | store | 4 | `sp -0x54` | `[0x29a954]` |
| `ldr` at 0x9b534 | load | 4 | `abs 0x9b598` | `-` |
| `ldr` at 0x9b538 | load | 4 | `abs 0x29a960` | `-` |
| `str` at 0x9b53c | store | 4 | `sp -0x50` | `[0x29a960]` |
| `str` at 0x9b544 | store | 4 | `sp -0x4c` | `sp-32` |
| `str` at 0x9b54c | store | 4 | `sp -0x48` | `sp-36` |
| `ldr` at 0x9b550 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x9b554 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9b558 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9b55c | load | 4 | `abs 0x29a94c` | `-` |
| `ldr` at 0x9b56c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9b570 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9b574 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9b57c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9b580 | store | 4 | `sp -0x34` | `[sp-24]` |
| `ldr` at 0x9b588 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x9b590 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9b590 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9b590 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9b41c | `(unknown callee) 0x1d444` | `0x1` | `r1` | `r2` | `r3` |
| `bl` at 0x9b428 | `(unknown callee) 0x1d444` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x9b430 | `(unknown callee) 0x488cc` | `ret(0x1d444)` | `?` | `?` | `?` |
| `bl` at 0x9b454 | `(unknown callee) 0x1e958` | `0x112508` | `[sp-24]` | `?` | `[sp-20]` |
| `bl` at 0x9b4d0 | `(unknown callee) 0xb51ec` | `[sp-48]` | `[sp-40]` | `[sp-44]` | `[0x29a94c]` |
| `bl` at 0x9b4f0 | `(unknown callee) 0x5ee70` | `0x3` | `sp-24` | `[sp-28]` | `[sp-32]` |
| `bl` at 0x9b514 | `(unknown callee) 0x1e958` | `0x112514` | `[sp-24]` | `?` | `[sp-20]` |
| `bl` at 0x9b560 | `(unknown callee) 0xb5f0c` | `[sp-48]` | `[sp-40]` | `[sp-44]` | `[0x29a94c]` |
| `bl` at 0x9b578 | `(unknown callee) 0x5ebd0` | `sp-24` | `[sp-28]` | `[sp-32]` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x9b448 | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `bne` at 0x9b508 | `[sp-20] != 0x2` | `[sp-20] == 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9b590 | `r0` | `[sp-52]` |

## `L1FRU_2m_PortStatus`

- `L1FRU_2m_PortStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9b5a0 to 0x9b5c0.
  Established. The unnamed `0x3c560` call carries `0x0`, `0x29a964`, entry `r2`, entry `r3` (`bl` at 0x9b5b4).
  The integer return candidate is `ret(0x3c560)` (`ldm` at 0x9b5b8).
  FRU port status storage interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x9b5b0 | load | 4 | `abs 0x9b5bc` | `-` |
| `ldm` at 0x9b5b8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9b5b8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9b5b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9b5b4 | `(unknown callee) 0x3c560` | `0x0` | `0x29a964` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9b5b8 | `r0` | `ret(0x3c560)` |

## `L1FRU_2m_EnvStatus`

- `L1FRU_2m_EnvStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9b5c0 to 0x9b5e0.
  Established. The unnamed `0x767f8` call carries `0x29a968`, `0x1`, entry `r2`, entry `r3` (`bl` at 0x9b5d4).
  The integer return candidate is `ret(0x767f8)` (`ldm` at 0x9b5d8).
  FRU environment result encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x9b5cc | load | 4 | `abs 0x9b5dc` | `-` |
| `ldm` at 0x9b5d8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9b5d8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9b5d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9b5d4 | `(unknown callee) 0x767f8` | `0x29a968` | `0x1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9b5d8 | `r0` | `ret(0x767f8)` |

## `L1FRU_2m_MgnStatus`

- `L1FRU_2m_MgnStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9b5e0 to 0x9b600.
  Established. The unnamed `0x88894` call carries `0x29a96c`, `0x1`, entry `r2`, entry `r3` (`bl` at 0x9b5f4).
  The integer return candidate is `ret(0x88894)` (`ldm` at 0x9b5f8).
  FRU margin result domain remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x9b5ec | load | 4 | `abs 0x9b5fc` | `-` |
| `ldm` at 0x9b5f8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9b5f8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9b5f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9b5f4 | `(unknown callee) 0x88894` | `0x29a96c` | `0x1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9b5f8 | `r0` | `ret(0x88894)` |

## `FRUInit`

- `FRUInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb0830 to 0xb0840.
  Established. The return record preserves entry `r0` (`ldm` at 0xb083c).
  FRU initialization side effects beyond the stack operations remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldm` at 0xb083c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb083c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb083c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb083c | `r0` | `r0` |

## `FRUStatus`

- `FRUStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7424 to 0xb7504.
  Established. The unsigned condition entry `r0 <= 0x8` takes an unresolved target, while greater falls through (`ldrls` at 0xb7440).
  A word assignment writes `0x11e7e0` to `sp -0x18` on known edges (`str` at 0xb74d0).
  The integer return candidate on those edges is `0x11e7e0` (`ldm` at 0xb74dc).
  FRU status table mapping remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb7434 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb7438 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0xb7440 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb74cc | load | 4 | `abs 0xb7500` | `-` |
| `str` at 0xb74d0 | store | 4 | `sp -0x18` | `0x11e7e0` |
| `ldr` at 0xb74d4 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xb74dc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb74dc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb74dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xb7440 | `r0 <= 0x8 (unsigned, 32-bit)` | `r0 > 0x8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb74dc | `r0` | `0x11e7e0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xb7440).

## `FRUEventInit`

- `FRUEventInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb3c24 to 0xb3c9c.
  Established. A zero `[r0]` takes `0xb3c88`, while nonzero falls through (`beq` at 0xb3c44).
  The unnamed `0x1970c` call carries four unknown candidates (`bl` at 0xb3c78).
  A word store clears an unknown effective target (`str` at 0xb3c90).
  Event-list ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb3c34 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb3c38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3c3c | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0xb3c48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3c4c | load | 4 | `r0 +0x0` | `-` |
| `str` at 0xb3c50 | store | 4 | `sp -0x18` | `[r0]` |
| `str` at 0xb3c58 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xb3c5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb3c68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb3c6c | load | 4 | `(unknown)` | `-` |
| `str` at 0xb3c70 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xb3c74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb3c7c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb3c80 | store | 4 | `sp -0x18` | `[sp-28]` |
| `ldr` at 0xb3c88 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb3c90 | store | 4 | `(unknown)` | `0x0` |
| `ldm` at 0xb3c98 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb3c98 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb3c98 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb3c78 | `(unknown callee) 0x1970c` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb3c44 | `[r0] == 0x0` | `[r0] != 0x0` |
| `beq` at 0xb3c64 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb3c98 | `r0` | `?` |

## `FRUEventMerge`

- `FRUEventMerge`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb3d90 to 0xb3dec.
  Established. A word store clears an unknown effective target (`str` at 0xb3dd0).
  The unnamed `0xb3c9c` call carries `[sp-20]`, `[sp-28]`, unknown, `0x0` (`bl` at 0xb3ddc).
  Event merge ordering remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb3da0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb3da4 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xb3da8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb3db4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb3db8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xb3dbc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb3dc0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb3dc4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xb3dc8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb3dd0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xb3dd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3dd8 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xb3de8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb3de8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb3de8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb3ddc | `(unknown callee) 0xb3c9c` | `[sp-20]` | `[sp-28]` | `?` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb3db0 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb3de8 | `r0` | `?` |

## `FRUEventCreate`

- `FRUEventCreate`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb3dec to 0xb3f94.
  Established. Included byte stores clear `[sp-36] +0x67` and `[sp-36] +0xa7`, and a word store clears `[sp-36] +0xa8` (`strb` at 0xb3f4c, `strb` at 0xb3f70, `str` at 0xb3f7c).
  The unnamed `0x19760` calls carry `[sp-36]+40`, `[sp+64]`, `0x40`, `[sp-36]+40` and `[sp-36]+104`, `[sp+68]`, `0x40`, `[sp-36]+104` (`bl` at 0xb3f40, `bl` at 0xb3f64).
  Event payload field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb3dfc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb3e00 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb3e04 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb3e08 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xb3e18 | store | 4 | `sp -0x24` | `ret(0x1943c)` |
| `ldr` at 0xb3e1c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3e2c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3e30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb3e3c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3e40 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xb3e44 | store | 1 | `[sp-36] +0x8` | `([sp-28] & 0xff)` |
| `ldr` at 0xb3e48 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3e4c | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xb3e50 | store | 1 | `[sp-36] +0x9` | `([sp-32] & 0xff)` |
| `ldr` at 0xb3e54 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3e58 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xb3e5c | store | 1 | `[sp-36] +0xa` | `([sp] & 0xff)` |
| `ldr` at 0xb3e60 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3e64 | load | 4 | `sp +0x4` | `-` |
| `strb` at 0xb3e68 | store | 1 | `[sp-36] +0xb` | `([sp+4] & 0xff)` |
| `ldr` at 0xb3e6c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3e70 | load | 4 | `sp +0x8` | `-` |
| `strb` at 0xb3e74 | store | 1 | `[sp-36] +0xc` | `([sp+8] & 0xff)` |
| `ldr` at 0xb3e78 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3e7c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xb3e80 | store | 4 | `[sp-36] +0x10` | `[sp+12]` |
| `ldr` at 0xb3e84 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3e88 | load | 4 | `sp +0x10` | `-` |
| `strb` at 0xb3e8c | store | 1 | `[sp-36] +0xd` | `([sp+16] & 0xff)` |
| `ldr` at 0xb3e90 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3e94 | load | 4 | `sp +0x14` | `-` |
| `strh` at 0xb3e98 | store | 2 | `[sp-36] +0x18` | `([sp+20] & 0xffff)` |
| `ldr` at 0xb3e9c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3ea0 | load | 4 | `sp +0x18` | `-` |
| `strh` at 0xb3ea4 | store | 2 | `[sp-36] +0x1a` | `([sp+24] & 0xffff)` |
| `ldr` at 0xb3ea8 | load | 4 | `sp -0x24` | `-` |
| `strh` at 0xb3eb0 | store | 2 | `[sp-36] +0x1c` | `0x0` |
| `ldr` at 0xb3eb4 | load | 4 | `sp -0x24` | `-` |
| `strh` at 0xb3ebc | store | 2 | `[sp-36] +0x1e` | `0x0` |
| `ldr` at 0xb3ec0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3ec4 | load | 4 | `sp +0x1c` | `-` |
| `strb` at 0xb3ec8 | store | 1 | `[sp-36] +0x14` | `([sp+28] & 0xff)` |
| `ldr` at 0xb3ecc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3ed0 | load | 4 | `sp +0x20` | `-` |
| `strb` at 0xb3ed4 | store | 1 | `[sp-36] +0x20` | `([sp+32] & 0xff)` |
| `ldr` at 0xb3ed8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3edc | load | 4 | `sp +0x24` | `-` |
| `strb` at 0xb3ee0 | store | 1 | `[sp-36] +0x15` | `([sp+36] & 0xff)` |
| `ldr` at 0xb3ee4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3ee8 | load | 4 | `sp +0x28` | `-` |
| `strb` at 0xb3eec | store | 1 | `[sp-36] +0x21` | `([sp+40] & 0xff)` |
| `ldr` at 0xb3ef0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3ef4 | load | 4 | `sp +0x2c` | `-` |
| `strb` at 0xb3ef8 | store | 1 | `[sp-36] +0x22` | `([sp+44] & 0xff)` |
| `ldr` at 0xb3efc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3f00 | load | 4 | `sp +0x30` | `-` |
| `strb` at 0xb3f04 | store | 1 | `[sp-36] +0x23` | `([sp+48] & 0xff)` |
| `ldr` at 0xb3f08 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3f0c | load | 4 | `sp +0x34` | `-` |
| `strb` at 0xb3f10 | store | 1 | `[sp-36] +0x24` | `([sp+52] & 0xff)` |
| `ldr` at 0xb3f14 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3f18 | load | 4 | `sp +0x38` | `-` |
| `strb` at 0xb3f1c | store | 1 | `[sp-36] +0x26` | `([sp+56] & 0xff)` |
| `ldr` at 0xb3f20 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3f24 | load | 4 | `sp +0x3c` | `-` |
| `strb` at 0xb3f28 | store | 1 | `[sp-36] +0x25` | `([sp+60] & 0xff)` |
| `ldr` at 0xb3f2c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3f38 | load | 4 | `sp +0x40` | `-` |
| `ldr` at 0xb3f44 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xb3f4c | store | 1 | `[sp-36] +0x67` | `0x0` |
| `ldr` at 0xb3f50 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3f5c | load | 4 | `sp +0x44` | `-` |
| `ldr` at 0xb3f68 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xb3f70 | store | 1 | `[sp-36] +0xa7` | `0x0` |
| `ldr` at 0xb3f74 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb3f7c | store | 4 | `[sp-36] +0xa8` | `0x0` |
| `ldr` at 0xb3f80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3f84 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xb3f90 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb3f90 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb3f90 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb3e10 | `(unknown callee) 0x1943c` | `0xac` | `r1` | `r2` | `r3` |
| `bl` at 0xb3e38 | `(unknown callee) 0x19610` | `ret(0x1943c)` | `[sp-24]` | `0x8` | `ret(0x1943c)` |
| `bl` at 0xb3f40 | `(unknown callee) 0x19760` | `[sp-36]+40` | `[sp+64]` | `0x40` | `[sp-36]+40` |
| `bl` at 0xb3f64 | `(unknown callee) 0x19760` | `[sp-36]+104` | `[sp+68]` | `0x40` | `[sp-36]+104` |
| `bl` at 0xb3f88 | `(unknown callee) 0xb3c9c` | `[sp-20]` | `[sp-36]` | `[sp-36]` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb3e24 | `ret(0x1943c) != 0x0` | `ret(0x1943c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb3f90 | `r0` | `?` |

## `FRUEventAdd`

- `FRUEventAdd`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb3c9c to 0xb3d90.
  Established. The signed comparison `ret(0x19580) > 0x0` takes `0xb3d14`, while nonpositive falls through (`bgt` at 0xb3cf0).
  Included word stores write `[[sp-20]]` to `[sp-24] +0xa8` and `[sp-24]` to `[sp-28] +0xa8` (`str` at 0xb3d00, `str` at 0xb3d84).
  Event ordering key interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb3cac | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb3cb0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xb3cb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3cb8 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0xb3cc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3cc8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb3ccc | store | 4 | `r0 +0x0` | `r1` |
| `ldr` at 0xb3cd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3cd8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb3cdc | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0xb3cf4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb3cf8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3cfc | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xb3d00 | store | 4 | `[sp-24] +0xa8` | `[[sp-20]]` |
| `ldr` at 0xb3d04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3d08 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb3d0c | store | 4 | `[sp-20] +0x0` | `[sp-24]` |
| `ldr` at 0xb3d14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3d18 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xb3d1c | store | 4 | `sp -0x1c` | `[[sp-20]]` |
| `ldr` at 0xb3d20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3d24 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xb3d28 | load | 4 | `[[sp-20]] +0xa8` | `-` |
| `str` at 0xb3d2c | store | 4 | `sp -0x20` | `[[[sp-20]]+168]` |
| `ldr` at 0xb3d30 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb3d3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb3d40 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb3d58 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb3d5c | store | 4 | `sp -0x1c` | `[sp-32]` |
| `ldr` at 0xb3d60 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb3d64 | load | 4 | `[sp-32] +0xa8` | `-` |
| `str` at 0xb3d68 | store | 4 | `sp -0x20` | `[[sp-32]+168]` |
| `ldr` at 0xb3d70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb3d74 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb3d78 | store | 4 | `[sp-24] +0xa8` | `?` |
| `ldr` at 0xb3d7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb3d80 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb3d84 | store | 4 | `[sp-28] +0xa8` | `[sp-24]` |
| `ldm` at 0xb3d8c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb3d8c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb3d8c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb3ce4 | `(unknown callee) 0x19580` | `r1` | `[r0]` | `0x8` | `r0` |
| `bl` at 0xb3d48 | `(unknown callee) 0x19580` | `[sp-24]` | `?` | `0x8` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb3cc0 | `[r0] != 0x0` | `[r0] == 0x0` |
| `bgt` at 0xb3cf0 | `ret(0x19580) > 0x0 (signed, 32-bit)` | `ret(0x19580) <= 0x0 (signed, 32-bit)` |
| `beq` at 0xb3d38 | `Z == 1` | `not (Z == 1)` |
| `blt` at 0xb3d54 | `ret(0x19580) < 0x0 (signed, 32-bit)` | `ret(0x19580) >= 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb3d8c | `r0` | `?` |
