# ARM configuration events and record paths in 1.44.0

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
The computed transfer in L1CfgGetCfg remains unresolved; its tables cover known edges only.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/configuration-events-records-facts.txt) retain all tool sections.

## `L1CfgPortEnable`

- `L1CfgPortEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x642e0 to 0x643b8.
  Established. An included word store writes `0x1` to an unknown effective target (`str` at 0x64350). Another writes zero to an unknown target (`str` at 0x64380). The absolute word at
  `0x132780` receives `[0x132780]+1` (`str` at 0x64394). Port enable policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x642f0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x642f4 | load | 4 | `abs 0x643a0` | `-` |
| `ldr` at 0x642fc | load | 4 | `abs 0x643a4` | `-` |
| `ldr` at 0x64300 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x64310 | load | 4 | `abs 0x643a8` | `-` |
| `ldr` at 0x64314 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x64318 | load | 4 | `abs 0x643ac` | `-` |
| `ldr` at 0x6431c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x64324 | load | 4 | `abs 0x643b0` | `-` |
| `ldr` at 0x64328 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x64344 | load | 4 | `abs 0x132ad8` | `-` |
| `str` at 0x64350 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x64354 | load | 4 | `abs 0x643b0` | `-` |
| `ldr` at 0x64358 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x64374 | load | 4 | `abs 0x132ad8` | `-` |
| `str` at 0x64380 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x64384 | load | 4 | `abs 0x643b4` | `-` |
| `ldr` at 0x64388 | load | 4 | `abs 0x643b4` | `-` |
| `ldr` at 0x6438c | load | 4 | `abs 0x132780` | `-` |
| `str` at 0x64394 | store | 4 | `abs 0x132780` | `[0x132780]+1` |
| `ldm` at 0x6439c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x6439c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6439c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6439c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x642f8 | `(unknown callee) 0x619dc` | `0xfc1f4` | `r1` | `r2` | `r3` |
| `bl` at 0x64320 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc204` | `[sp-20]` | `0x12eeb8` |
| `bl` at 0x64398 | `(unknown callee) 0x61a78` | `?` | `0x132a50` | `0x132780` | `[0x132780]+1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6430c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6439c | `r0` | `ret(0x61a78)` |

## `L1CfgGetBrickCfg`

- `L1CfgGetBrickCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x69eec to 0x69f40.
  Established. The absolute words at `0x132af0` and `0x132ae8` are loaded (`ldr` at 0x69f14, `ldr` at 0x69f18). The unnamed `0x65184` call carries `[0x132af0]`, `[0x132ae8]`, entry `r0`, entry `r1`
  (`bl` at 0x69f24); its result is the included integer return value (`ldm` at 0x69f38).
  Configuration lookup semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x69efc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x69f00 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x69f08 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x69f0c | load | 4 | `abs 0x69f3c` | `-` |
| `ldr` at 0x69f10 | load | 4 | `abs 0x69f3c` | `-` |
| `ldr` at 0x69f14 | load | 4 | `abs 0x132af0` | `-` |
| `ldr` at 0x69f18 | load | 4 | `abs 0x132ae8` | `-` |
| `ldr` at 0x69f1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x69f20 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x69f28 | store | 4 | `sp -0x1c` | `ret(0x65184)` |
| `ldr` at 0x69f2c | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x69f38 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x69f38 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x69f38 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x69f24 | `(unknown callee) 0x65184` | `[0x132af0]` | `[0x132ae8]` | `r0` | `r1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x69f38 | `r0` | `ret(0x65184)` |

## `L1CfgUpdate`

- `L1CfgUpdate`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6288c to 0x62b50.
  Established. Included byte stores clear `sp -0x94` and `sp -0x93` (`strb` at 0x628b0, `strb` at 0x628b8). A zero `ret(0x69eec)` takes `0x62a78`, while nonzero falls through (`beq` at 0x62a58).
  The unnamed `0xaa20c` call carries `0x132a48`, `0x3e9`, `[sp-152]`, `sp-148` (`bl` at 0x62b24).
  Configuration update policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x628a0 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0x628a4 | load | 4 | `abs 0x62b30` | `-` |
| `strb` at 0x628b0 | store | 1 | `sp -0x94` | `0x0` |
| `strb` at 0x628b8 | store | 1 | `sp -0x93` | `0x0` |
| `str` at 0x628c0 | store | 4 | `sp -0x90` | `0x1` |
| `strh` at 0x628c8 | store | 2 | `sp -0x92` | `0x0` |
| `ldr` at 0x628cc | load | 4 | `sp -0x98` | `-` |
| `str` at 0x628d4 | store | 4 | `sp -0x98` | `[sp-152]+1` |
| `ldr` at 0x628e8 | load | 4 | `abs 0x62b34` | `-` |
| `str` at 0x628f8 | store | 4 | `sp -0x9c` | `0x0` |
| `ldr` at 0x628fc | load | 4 | `abs 0x62b34` | `-` |
| `ldr` at 0x62900 | load | 4 | `abs 0x132ad4` | `-` |
| `ldr` at 0x62904 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x62910 | load | 4 | `abs 0x62b34` | `-` |
| `ldr` at 0x62914 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x62930 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x62938 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x62944 | load | 4 | `abs 0x62b34` | `-` |
| `ldr` at 0x62948 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x62964 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x6296c | load | 4 | `(unknown)` | `-` |
| `str` at 0x6297c | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x62980 | load | 4 | `abs 0x62b34` | `-` |
| `ldr` at 0x62984 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x629a0 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x629a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x629ac | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x629b8 | load | 4 | `abs 0x62b34` | `-` |
| `ldr` at 0x629bc | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x629d8 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x629e0 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x629f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x629fc | load | 4 | `abs 0x62b34` | `-` |
| `ldr` at 0x62a00 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x62a1c | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x62a24 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x62a38 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x62a40 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x62a44 | load | 4 | `(unknown)` | `-` |
| `str` at 0x62a4c | store | 4 | `sp -0xa4` | `ret(0x69eec)` |
| `ldr` at 0x62a50 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x62a6c | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x62a78 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x62a80 | store | 4 | `sp -0xa0` | `[sp-160]+1` |
| `ldr` at 0x62a88 | load | 4 | `sp -0x9c` | `-` |
| `str` at 0x62a90 | store | 4 | `sp -0x9c` | `[sp-156]+1` |
| `ldr` at 0x62a98 | load | 4 | `abs 0x62b38` | `-` |
| `str` at 0x62aa4 | store | 4 | `abs 0x132798` | `ret(0x194f0)` |
| `strb` at 0x62ab0 | store | 1 | `sp -0x94` | `0x0` |
| `strb` at 0x62ab8 | store | 1 | `sp -0x93` | `0x0` |
| `ldr` at 0x62abc | load | 4 | `sp -0x98` | `-` |
| `str` at 0x62ac4 | store | 4 | `sp -0x90` | `[sp-152]-1` |
| `strh` at 0x62acc | store | 2 | `sp -0x92` | `0x0` |
| `ldr` at 0x62ad0 | load | 4 | `abs 0x62b3c` | `-` |
| `ldr` at 0x62ad4 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x62af0 | load | 4 | `abs 0x62b40` | `-` |
| `ldr` at 0x62af4 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x62af8 | load | 4 | `abs 0x62b44` | `-` |
| `ldr` at 0x62afc | load | 4 | `sp -0x98` | `-` |
| `str` at 0x62b10 | store | 4 | `sp -0xa8` | `0x0` |
| `ldr` at 0x62b14 | load | 4 | `abs 0x62b48` | `-` |
| `ldr` at 0x62b18 | load | 4 | `abs 0x62b4c` | `-` |
| `ldr` at 0x62b1c | load | 4 | `sp -0x98` | `-` |
| `ldm` at 0x62b2c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x62b2c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x62b2c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x62b2c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x628a8 | `(unknown callee) 0x619dc` | `0xfbf54` | `r1` | `r2` | `0x0` |
| `bl` at 0x628f0 | `(unknown callee) 0x67280` | `sp-152` | `sp-148` | `0x132a50` | `0x1` |
| `bl` at 0x62a48 | `(unknown callee) 0x69eec` | `?` | `?` | `?` | `?` |
| `bl` at 0x62a74 | `(unknown callee) 0x67280` | `sp-152` | `sp-148` | `ret(0x69eec)` | `0x1` |
| `bl` at 0x62aa0 | `(unknown callee) 0x194f0` | `0x0` | `?` | `[0x132ad4]` | `?` |
| `bl` at 0x62aa8 | `(unknown callee) 0x61a78` | `ret(0x194f0)` | `?` | `?` | `?` |
| `bl` at 0x62ae8 | `(unknown callee) 0x194f0` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x62b04 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfbf60` | `[sp-152]` | `ret(0x194f0)` |
| `bl` at 0x62b24 | `(unknown callee) 0xaa20c` | `0x132a48` | `0x3e9` | `[sp-152]` | `sp-148` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x6290c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x62940 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x62974 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x629b4 | `? <= [sp-160] (signed, 32-bit)` | `? > [sp-160] (signed, 32-bit)` |
| `beq` at 0x62a58 | `ret(0x69eec) == 0x0` | `ret(0x69eec) != 0x0` |
| `beq` at 0x62ae0 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x62b2c | `r0` | `ret(0xaa20c)` |

## `L1CfgGetBrickType`

- `L1CfgGetBrickType`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x69e60 to 0x69eec.
  Established. A zero `ret(0x65068)` takes `0x69ec0`, while nonzero falls through (`beq` at 0x69eac). The nonzero arm reads the word through `ret(0x65068) +0x8` (`ldr` at 0x69eb4). The
  fallback word at `sp -0x24` receives `0x3f` (`str` at 0x69ec4). Brick type interpretation remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x69e70 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x69e74 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x69e78 | load | 4 | `abs 0x69ee4` | `-` |
| `ldr` at 0x69e80 | load | 4 | `abs 0x69ee8` | `-` |
| `ldr` at 0x69e84 | load | 4 | `abs 0x69ee8` | `-` |
| `ldr` at 0x69e88 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x69e8c | load | 4 | `abs 0x132adc` | `-` |
| `ldr` at 0x69e90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x69e94 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x69ea0 | store | 4 | `sp -0x1c` | `ret(0x65068)` |
| `ldr` at 0x69ea4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x69eb0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x69eb4 | load | 4 | `ret(0x65068) +0x8` | `-` |
| `str` at 0x69eb8 | store | 4 | `sp -0x24` | `[ret(0x65068)+8]` |
| `str` at 0x69ec4 | store | 4 | `sp -0x24` | `0x3f` |
| `ldr` at 0x69ec8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x69ecc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x69ed4 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x69ee0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x69ee0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x69ee0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x69e7c | `(unknown callee) 0x619dc` | `0xfc864` | `r1` | `r2` | `r3` |
| `bl` at 0x69e98 | `(unknown callee) 0x65068` | `[0x132ae4]` | `[0x132adc]` | `[sp-20]` | `[sp-24]` |
| `bl` at 0x69ed0 | `(unknown callee) 0x61a78` | `ret(0x65068)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x69eac | `ret(0x65068) == 0x0` | `ret(0x65068) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x69ee0 | `r0` | `[sp-32]` |

## `L1CfgGetCfg`

- `L1CfgGetCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6a0f8 to 0x6a2ac.
  Established. The computed transfer is taken when unsigned entry `r0 <= 0x28`, with fall-through
  when greater (`ldrls` at 0x6a114). The included fallback writes `0x132cf4` to `sp -0x28` (`str` at 0x6a260). The known-edge return record carries `0x132cf4` in `r0` (`ldm` at 0x6a2a0).
  Configuration selection across unresolved destinations remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6a108 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6a10c | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0x6a114 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6a25c | load | 4 | `abs 0x6a2a8` | `-` |
| `str` at 0x6a260 | store | 4 | `sp -0x28` | `0x132cf4` |
| `ldr` at 0x6a298 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x6a2a0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6a2a0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6a2a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x6a114 | `r0 <= 0x28 (unsigned, 32-bit)` | `r0 > 0x28 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6a2a0 | `r0` | `0x132cf4` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x6a114).

## `L1CfgGetBrickIndex`

- `L1CfgGetBrickIndex`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x69f40 to 0x6a038.
  Established. An included word assignment at `sp -0x1c` writes `0x1` (`str` at 0x69fec). The
  absolute word at `0x132ae4` is read at two sites (`ldr` at 0x69fa0, `ldr` at 0x69fd0). The
  included return record carries `[sp-36]` in `r0` (`ldm` at 0x6a030). Brick index search semantics
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x69f50 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x69f54 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x69f5c | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x69f64 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x69f68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x69f74 | load | 4 | `abs 0x6a034` | `-` |
| `ldr` at 0x69f78 | load | 4 | `abs 0x132adc` | `-` |
| `ldr` at 0x69f7c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x69f88 | load | 4 | `abs 0x6a034` | `-` |
| `ldr` at 0x69f8c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x69fa0 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x69fa8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x69fac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x69fb8 | load | 4 | `abs 0x6a034` | `-` |
| `ldr` at 0x69fbc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x69fd0 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x69fd8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x69fdc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x69fec | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x69ff0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x69ff8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x6a000 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6a00c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x6a014 | store | 4 | `sp -0x24` | `?` |
| `str` at 0x6a020 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x6a024 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x6a030 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6a030 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6a030 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x69f70 | `Z == 0` | `not (Z == 0)` |
| `ble` at 0x69f84 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x69fb4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x69fe4 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x6a008 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6a030 | `r0` | `[sp-36]` |

## `L1CfgEventSystemConsole`

- `L1CfgEventSystemConsole`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6a6f4 to 0x6a714.
  Established. The unnamed `0x6a474` call receives `0x1`, `0x0`, `0x0`, entry `r3` (`bl` at 0x6a70c). The return record carries its result in `r0` (`ldm` at 0x6a710). System console event
  meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldm` at 0x6a710 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6a710 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6a710 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6a70c | `(unknown callee) 0x6a474` | `0x1` | `0x0` | `0x0` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6a710 | `r0` | `ret(0x6a474)` |

## `L1CfgEventConsoleEnable`

- `L1CfgEventConsoleEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6a734 to 0x6a754.
  Established. The unnamed `0x6a474` call carries `0x3`, `0x0`, `0x0`, entry `r3` (`bl` at 0x6a74c);
  its result reaches the included `r0` return record (`ldm` at 0x6a750). Console enable event
  effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldm` at 0x6a750 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6a750 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6a750 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6a74c | `(unknown callee) 0x6a474` | `0x3` | `0x0` | `0x0` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6a750 | `r0` | `ret(0x6a474)` |

## `L1CfgEventConsoleSubchannel`

- `L1CfgEventConsoleSubchannel`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6a714 to 0x6a734.
  Established. The unnamed `0x6a474` call passes `0x2`, `0x0`, `0x0`, entry `r3` (`bl` at 0x6a72c).
  The integer return record gives `ret(0x6a474)` (`ldm` at 0x6a730). Subchannel event policy remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldm` at 0x6a730 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6a730 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6a730 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6a72c | `(unknown callee) 0x6a474` | `0x2` | `0x0` | `0x0` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6a730 | `r0` | `ret(0x6a474)` |

## `L1CfgEventMarginState`

- `L1CfgEventMarginState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6a7b0 to 0x6a80c.
  Established. Equality `[ret(0x6a0f8)+56] == [sp-20]` takes `0x6a804`, while inequality falls
  through (`beq` at 0x6a7e4). The latter arm writes `[sp-20]` through `ret(0x6a0f8) +0x38` (`str` at 0x6a7f0); its unnamed `0x6a474` call carries `0x5`, `0x0`, `0x0`, `[sp-20]` (`bl` at 0x6a800).
  Margin event interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6a7c0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6a7d0 | store | 4 | `sp -0x18` | `ret(0x6a0f8)` |
| `ldr` at 0x6a7d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a7d8 | load | 4 | `ret(0x6a0f8) +0x38` | `-` |
| `ldr` at 0x6a7dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6a7e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a7ec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6a7f0 | store | 4 | `ret(0x6a0f8) +0x38` | `[sp-20]` |
| `ldm` at 0x6a808 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6a808 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6a808 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6a7c8 | `(unknown callee) 0x6a0f8` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x6a800 | `(unknown callee) 0x6a474` | `0x5` | `0x0` | `0x0` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6a7e4 | `[ret(0x6a0f8)+56] == [sp-20]` | `[ret(0x6a0f8)+56] != [sp-20]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6a808 | `r0` | `?` |

## `L1CfgEventEnvironmental`

- `L1CfgEventEnvironmental`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6aa60 to 0x6aabc.
  Established. The condition `[sp-20] == [ret(0x6a0f8)+92]` takes `0x6aab4`, while inequality falls
  through (`beq` at 0x6aa94). An included word store writes `[sp-20]` through `ret(0x6a0f8) +0x5c`
  (`str` at 0x6aaa0). The unnamed `0x6a474` call receives `0x8`, `0x0`, `0x0`, `[sp-20]` (`bl` at 0x6aab0). Environmental event behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6aa70 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6aa80 | store | 4 | `sp -0x18` | `ret(0x6a0f8)` |
| `ldr` at 0x6aa84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6aa88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6aa8c | load | 4 | `ret(0x6a0f8) +0x5c` | `-` |
| `ldr` at 0x6aa98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6aa9c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6aaa0 | store | 4 | `ret(0x6a0f8) +0x5c` | `[sp-20]` |
| `ldm` at 0x6aab8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6aab8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6aab8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6aa78 | `(unknown callee) 0x6a0f8` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x6aab0 | `(unknown callee) 0x6a474` | `0x8` | `0x0` | `0x0` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6aa94 | `[sp-20] == [ret(0x6a0f8)+92]` | `[sp-20] != [ret(0x6a0f8)+92]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6aab8 | `r0` | `?` |

## `L1CfgEventNVRAM`

- `L1CfgEventNVRAM`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6aabc to 0x6aaec.
  Established. Entry `r0` and entry `r1` are saved as words at `sp -0x14` and `sp -0x18` (`str` at 0x6aacc, `str` at 0x6aad0). The unnamed `0x6a474` call carries `0x66`, entry `r0`, entry `r1`,
  entry `r3` (`bl` at 0x6aae0). NVRAM notification semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6aacc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6aad0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x6aad8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6aadc | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x6aae8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6aae8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6aae8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6aae0 | `(unknown callee) 0x6a474` | `0x66` | `r0` | `r1` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6aae8 | `r0` | `ret(0x6a474)` |

## `L1CfgEventSystemSN`

- `L1CfgEventSystemSN`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6a9b0 to 0x6aa60.
  Established. A zero `[sp-20]` takes `0x6aa28`, while nonzero falls through (`beq` at 0x6aa04). The
  unnamed `0x19808` call at `0x6aa3c` carries `0x29a978`, `ret(0x1d444)`, unknown, `ret(0x1d444)`
  (`bl` at 0x6aa3c). The included return record gives `ret(0x60bf0)` (`ldm` at 0x6aa58). Serial
  number propagation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6a9c0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6a9d0 | store | 4 | `sp -0x18` | `ret(0x6a0f8)` |
| `str` at 0x6a9d8 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0x6a9e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a9fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6aa14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6aa34 | load | 4 | `abs 0x6aa5c` | `-` |
| `ldm` at 0x6aa58 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6aa58 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6aa58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6a9c8 | `(unknown callee) 0x6a0f8` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x6a9e0 | `(unknown callee) 0x1d444` | `0xd` | `?` | `?` | `0x0` |
| `bl` at 0x6a9f8 | `(unknown callee) 0x19808` | `[sp-24]+76` | `ret(0x1d444)` | `ret(0x1d444)` | `[sp-24]+76` |
| `bl` at 0x6aa0c | `(unknown callee) 0x1d444` | `0xd` | `?` | `?` | `[sp-20]` |
| `bl` at 0x6aa24 | `(unknown callee) 0x19808` | `[sp-24]+100` | `ret(0x1d444)` | `[sp-24]+100` | `ret(0x1d444)` |
| `bl` at 0x6aa2c | `(unknown callee) 0x1d444` | `0xd` | `?` | `?` | `?` |
| `bl` at 0x6aa3c | `(unknown callee) 0x19808` | `0x29a978` | `ret(0x1d444)` | `?` | `ret(0x1d444)` |
| `bl` at 0x6aa4c | `(unknown callee) 0x6a474` | `0x7` | `0x0` | `0x0` | `?` |
| `bl` at 0x6aa50 | `(unknown callee) 0x60bf0` | `ret(0x6a474)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6aa04 | `[sp-20] == 0x0` | `[sp-20] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6aa58 | `r0` | `ret(0x60bf0)` |

## `L1CfgEventPartition`

- `L1CfgEventPartition`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6a80c to 0x6a9b0.
  Established. Equality `[ret(0x6a0f8)+60] == [sp-20]` takes `0x6a9a8`, while inequality falls
  through (`beq` at 0x6a840). The latter path writes `[sp-20]` through `ret(0x6a0f8) +0x3c` (`str` at 0x6a84c). Included calls of unnamed `0x676c0` carry first candidates `0x1`, `0x3`, `0xe`, `0xf`
  at separate sites (`bl` at 0x6a98c, `bl` at 0x6a994, `bl` at 0x6a99c, `bl` at 0x6a9a4). Partition
  event effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6a81c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6a82c | store | 4 | `sp -0x18` | `ret(0x6a0f8)` |
| `ldr` at 0x6a830 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a834 | load | 4 | `ret(0x6a0f8) +0x3c` | `-` |
| `ldr` at 0x6a838 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6a844 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a848 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6a84c | store | 4 | `ret(0x6a0f8) +0x3c` | `[sp-20]` |
| `ldr` at 0x6a864 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a868 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x6a874 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a878 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x6a884 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a888 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x6a894 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a898 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x6a8a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a8a8 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x6a8b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a8b8 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x6a8c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a8c8 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x6a8d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a8d8 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x6a8e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a8e8 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x6a8f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a8f8 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x6a904 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a908 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x6a914 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a918 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x6a924 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a928 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x6a934 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a938 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x6a944 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a948 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x6a954 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a958 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x6a964 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a968 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x6a974 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a978 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldm` at 0x6a9ac | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6a9ac | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6a9ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6a824 | `(unknown callee) 0x6a0f8` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x6a850 | `(unknown callee) 0x5f70c` | `ret(0x6a0f8)` | `?` | `ret(0x6a0f8)` | `[sp-20]` |
| `bl` at 0x6a860 | `(unknown callee) 0x6a474` | `0x6` | `0x0` | `0x0` | `?` |
| `bl` at 0x6a98c | `(unknown callee) 0x676c0` | `0x1` | `?` | `?` | `[[sp-24]+16]` |
| `bl` at 0x6a994 | `(unknown callee) 0x676c0` | `0x3` | `?` | `?` | `?` |
| `bl` at 0x6a99c | `(unknown callee) 0x676c0` | `0xe` | `?` | `?` | `?` |
| `bl` at 0x6a9a4 | `(unknown callee) 0x676c0` | `0xf` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6a840 | `[ret(0x6a0f8)+60] == [sp-20]` | `[ret(0x6a0f8)+60] != [sp-20]` |
| `beq` at 0x6a870 | `[[sp-24]+16] == 0x43` | `[[sp-24]+16] != 0x43` |
| `beq` at 0x6a880 | `[[sp-24]+16] == 0x33` | `[[sp-24]+16] != 0x33` |
| `beq` at 0x6a890 | `[[sp-24]+16] == 0x35` | `[[sp-24]+16] != 0x35` |
| `beq` at 0x6a8a0 | `[[sp-24]+16] == 0x34` | `[[sp-24]+16] != 0x34` |
| `beq` at 0x6a8b0 | `[[sp-24]+16] == 0x41` | `[[sp-24]+16] != 0x41` |
| `beq` at 0x6a8c0 | `[[sp-24]+16] == 0x36` | `[[sp-24]+16] != 0x36` |
| `beq` at 0x6a8d0 | `[[sp-24]+16] == 0x39` | `[[sp-24]+16] != 0x39` |
| `beq` at 0x6a8e0 | `[[sp-24]+16] == 0x37` | `[[sp-24]+16] != 0x37` |
| `beq` at 0x6a8f0 | `[[sp-24]+16] == 0x38` | `[[sp-24]+16] != 0x38` |
| `beq` at 0x6a900 | `[[sp-24]+16] == 0x30` | `[[sp-24]+16] != 0x30` |
| `beq` at 0x6a910 | `[[sp-24]+16] == 0x51` | `[[sp-24]+16] != 0x51` |
| `beq` at 0x6a920 | `[[sp-24]+16] == 0x40` | `[[sp-24]+16] != 0x40` |
| `beq` at 0x6a930 | `[[sp-24]+16] == 0x4f` | `[[sp-24]+16] != 0x4f` |
| `beq` at 0x6a940 | `[[sp-24]+16] == 0x42` | `[[sp-24]+16] != 0x42` |
| `beq` at 0x6a950 | `[[sp-24]+16] == 0x46` | `[[sp-24]+16] != 0x46` |
| `beq` at 0x6a960 | `[[sp-24]+16] == 0x4c` | `[[sp-24]+16] != 0x4c` |
| `beq` at 0x6a970 | `[[sp-24]+16] == 0x4d` | `[[sp-24]+16] != 0x4d` |
| `beq` at 0x6a980 | `[[sp-24]+16] == 0x7c` | `[[sp-24]+16] != 0x7c` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6a9ac | `r0` | `?` |

## `L1CfgUpdateState`

- `L1CfgUpdateState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x67094 to 0x67280.
  Established. The byte through entry `r0 +0x24` is read (`ldrb` at 0x670ac). An included absolute
  word store writes `ret(0x194f0)` to `0x13279c` (`str` at 0x67154). The unnamed `0x66a10` call
  carries `[sp-404]`, `[sp-400]`, `sp-412`, `0x1` (`bl` at 0x67248). Configuration state
  incorporation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x670a4 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x670a8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x670ac | load | 1 | `r0 +0x24` | `-` |
| `ldr` at 0x670bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x670c0 | load | 4 | `r0 +0x28` | `-` |
| `str` at 0x670c4 | store | 4 | `sp -0x1a0` | `[r0+40]` |
| `ldr` at 0x670cc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x670d8 | store | 4 | `sp -0x1a0` | `ret(0xaadf4)` |
| `ldr` at 0x670dc | load | 4 | `sp -0x1a0` | `-` |
| `str` at 0x670e0 | store | 4 | `sp -0x120` | `[sp-416]` |
| `ldr` at 0x670e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x670e8 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0x670f8 | store | 4 | `sp -0x124` | `?` |
| `ldr` at 0x670fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x67100 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0x6710c | store | 4 | `sp -0x128` | `?` |
| `ldr` at 0x67110 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x67114 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x67118 | store | 4 | `sp -0x12c` | `[[sp-24]+12]` |
| `ldr` at 0x6711c | load | 4 | `abs 0x67268` | `-` |
| `ldr` at 0x67120 | load | 4 | `abs 0x132a58` | `-` |
| `ldr` at 0x67124 | load | 4 | `sp -0x124` | `-` |
| `ldr` at 0x67130 | load | 4 | `abs 0x67268` | `-` |
| `ldr` at 0x67134 | load | 4 | `abs 0x132a5c` | `-` |
| `ldr` at 0x67138 | load | 4 | `sp -0x128` | `-` |
| `ldr` at 0x67148 | load | 4 | `abs 0x6726c` | `-` |
| `str` at 0x67154 | store | 4 | `abs 0x13279c` | `ret(0x194f0)` |
| `ldr` at 0x67158 | load | 4 | `abs 0x67270` | `-` |
| `ldr` at 0x67160 | load | 4 | `abs 0x67274` | `-` |
| `ldr` at 0x67164 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x67180 | load | 4 | `abs 0x67278` | `-` |
| `ldr` at 0x67184 | load | 4 | `sp -0x12c` | `-` |
| `str` at 0x67188 | store | 4 | `sp -0x1ac` | `[sp-300]` |
| `str` at 0x6718c | store | 4 | `sp -0x1a8` | `ret(0x194f0)` |
| `ldr` at 0x67190 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x67194 | load | 4 | `abs 0x6727c` | `-` |
| `ldr` at 0x67198 | load | 4 | `sp -0x124` | `-` |
| `ldr` at 0x6719c | load | 4 | `sp -0x128` | `-` |
| `str` at 0x671a8 | store | 4 | `sp -0x11c` | `0x0` |
| `ldr` at 0x671ac | load | 4 | `sp -0x11c` | `-` |
| `ldr` at 0x671b0 | load | 4 | `sp -0x120` | `-` |
| `ldr` at 0x671bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x671c0 | load | 4 | `sp -0x11c` | `-` |
| `ldrb` at 0x671d4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x671e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x671e8 | load | 4 | `sp -0x11c` | `-` |
| `ldr` at 0x671fc | load | 4 | `(unknown)` | `-` |
| `str` at 0x67200 | store | 4 | `sp -0x1a4` | `?` |
| `ldr` at 0x67208 | load | 4 | `sp -0x11c` | `-` |
| `ldr` at 0x67210 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6721c | store | 4 | `sp -0x1a4` | `ret(0xaadf4)` |
| `ldr` at 0x67228 | load | 4 | `sp -0x1a4` | `-` |
| `ldr` at 0x67238 | load | 4 | `sp -0x194` | `-` |
| `ldr` at 0x6723c | load | 4 | `sp -0x190` | `-` |
| `ldr` at 0x6724c | load | 4 | `sp -0x11c` | `-` |
| `str` at 0x67254 | store | 4 | `sp -0x11c` | `[sp-284]+1` |
| `ldm` at 0x67264 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x67264 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x67264 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x67264 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x670d4 | `(unknown callee) 0xaadf4` | `r0` | `0x0` | `r2` | `?` |
| `bl` at 0x67150 | `(unknown callee) 0x194f0` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x6715c | `(unknown callee) 0x619dc` | `0xfc390` | `?` | `?` | `?` |
| `bl` at 0x67178 | `(unknown callee) 0x194f0` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x671a0 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc3a4` | `[sp-292]` | `[sp-296]` |
| `bl` at 0x67218 | `(unknown callee) 0xaadf4` | `[sp-24]` | `?` | `[sp-24]` | `?` |
| `bl` at 0x67230 | `(unknown callee) 0x674d0` | `sp-412` | `?` | `0x70` | `sp-412` |
| `bl` at 0x67248 | `(unknown callee) 0x66a10` | `[sp-404]` | `[sp-400]` | `sp-412` | `0x1` |
| `bl` at 0x6725c | `(unknown callee) 0x61a78` | `?` | `?` | `?` | `[sp-288]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x670b8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x6712c | `[0x132a58] != [sp-292]` | `[0x132a58] == [sp-292]` |
| `bne` at 0x67140 | `[0x132a5c] != [sp-296]` | `[0x132a5c] == [sp-296]` |
| `beq` at 0x67170 | `? == 0x0` | `? != 0x0` |
| `bge` at 0x671b8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x671e0 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x67264 | `r0` | `?` |
