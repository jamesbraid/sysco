# ARM storage, PROM and presence helpers in 1.44.0

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
The computed transfers in `vramSet` (`ldrls` at 0x55324) and `vramGet` (`ldrls` at 0x554d8)
leave unresolved selector edges. Access and call tables cover only known edges.
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/vram-prom-presence-facts.txt) retain all tool sections.

## `vramInit`

- `vramInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x54f04 to 0x55188.
  Established. Included word assignments write `[0x132528]+1` to `0x132528` and `[sp-28]` to `0x299844` (`str` at 0x54f58, `str` at 0x55078).
  The unnamed `0x193b8` call carries `0x0`, `0x8000`, `0x3`, `0x1` (`bl` at 0x55000).
  A zero `ret(0x1eeb0)` takes `0x55150`, while nonzero falls through (`beq` at 0x550e4).
  The mapped storage device remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x54f18 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x54f1c | load | 4 | `abs 0x55158` | `-` |
| `ldr` at 0x54f20 | load | 4 | `abs 0x132528` | `-` |
| `ldr` at 0x54f2c | load | 4 | `abs 0x5515c` | `-` |
| `str` at 0x54f30 | store | 4 | `sp -0x34` | `0xf8fb4` |
| `ldr` at 0x54f38 | load | 4 | `abs 0x55160` | `-` |
| `ldr` at 0x54f48 | load | 4 | `abs 0x55158` | `-` |
| `ldr` at 0x54f4c | load | 4 | `abs 0x55158` | `-` |
| `ldr` at 0x54f50 | load | 4 | `abs 0x132528` | `-` |
| `str` at 0x54f58 | store | 4 | `abs 0x132528` | `[0x132528]+1` |
| `ldr` at 0x54f5c | load | 4 | `abs 0x55164` | `-` |
| `ldr` at 0x54f6c | load | 4 | `abs 0x55168` | `-` |
| `ldr` at 0x54f70 | load | 4 | `abs 0x5516c` | `-` |
| `str` at 0x54f7c | store | 4 | `sp -0x18` | `ret(0x1967c)` |
| `ldr` at 0x54f80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x54f94 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0x54fac | load | 4 | `abs 0x55170` | `-` |
| `str` at 0x54fb0 | store | 4 | `sp -0x34` | `0xf8ff4` |
| `ldr` at 0x54fb4 | load | 4 | `abs 0x55174` | `-` |
| `str` at 0x54fb8 | store | 4 | `sp -0x30` | `0xf8fa8` |
| `str` at 0x54fbc | store | 4 | `sp -0x2c` | `ret(0x1f8e0)` |
| `ldr` at 0x54fc0 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `str` at 0x54fc4 | store | 4 | `sp -0x28` | `[ret(0x193d0)]` |
| `ldr` at 0x54fcc | load | 4 | `abs 0x55160` | `-` |
| `ldr` at 0x54fe0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x54fe4 | store | 4 | `sp -0x34` | `ret(0x1967c)` |
| `str` at 0x54fec | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x55004 | store | 4 | `sp -0x1c` | `ret(0x193b8)` |
| `ldr` at 0x55008 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5501c | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0x55034 | load | 4 | `abs 0x55178` | `-` |
| `str` at 0x55038 | store | 4 | `sp -0x34` | `0xf9018` |
| `ldr` at 0x5503c | load | 4 | `abs 0x55174` | `-` |
| `str` at 0x55040 | store | 4 | `sp -0x30` | `0xf8fa8` |
| `str` at 0x55044 | store | 4 | `sp -0x2c` | `ret(0x1f8e0)` |
| `ldr` at 0x55048 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `str` at 0x5504c | store | 4 | `sp -0x28` | `[ret(0x193d0)]` |
| `ldr` at 0x55054 | load | 4 | `abs 0x55160` | `-` |
| `ldr` at 0x55064 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x55070 | load | 4 | `abs 0x5517c` | `-` |
| `ldr` at 0x55074 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x55078 | store | 4 | `abs 0x299844` | `[sp-28]` |
| `ldr` at 0x5507c | load | 4 | `abs 0x55180` | `-` |
| `ldr` at 0x55080 | load | 4 | `abs 0x1324cc` | `-` |
| `ldr` at 0x5508c | load | 4 | `abs 0x55180` | `-` |
| `ldr` at 0x55090 | load | 4 | `abs 0x5517c` | `-` |
| `ldr` at 0x55094 | load | 4 | `abs 0x299844` | `-` |
| `str` at 0x55098 | store | 4 | `abs 0x1324cc` | `[0x299844]` |
| `ldr` at 0x550b8 | load | 4 | `abs 0x55180` | `-` |
| `ldr` at 0x550bc | load | 4 | `abs 0x1324cc` | `-` |
| `ldr` at 0x550c4 | load | 4 | `abs 0x55180` | `-` |
| `ldr` at 0x550cc | load | 4 | `abs 0x1324cc` | `-` |
| `ldr` at 0x550e8 | load | 4 | `abs 0x55184` | `-` |
| `ldrb` at 0x550ec | load | 1 | `abs 0x1324d0` | `-` |
| `ldr` at 0x550f8 | load | 4 | `abs 0x55184` | `-` |
| `strb` at 0x55100 | store | 1 | `abs 0x1324d0` | `0x0` |
| `ldr` at 0x55104 | load | 4 | `abs 0x55164` | `-` |
| `ldr` at 0x55108 | load | 4 | `abs 0x55184` | `-` |
| `ldrb` at 0x5510c | load | 1 | `abs 0x1324d0` | `-` |
| `ldr` at 0x55114 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x55120 | load | 4 | `abs 0x55164` | `-` |
| `ldr` at 0x55124 | load | 4 | `abs 0x55184` | `-` |
| `ldrb` at 0x55128 | load | 1 | `abs 0x1324d0` | `-` |
| `ldr` at 0x55130 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x55138 | load | 4 | `abs 0x55184` | `-` |
| `ldr` at 0x5513c | load | 4 | `abs 0x55184` | `-` |
| `ldrb` at 0x55140 | load | 1 | `abs 0x1324d0` | `-` |
| `strb` at 0x55148 | store | 1 | `abs 0x1324d0` | `([0x1324d0]:1+1 & 0xff)` |
| `ldm` at 0x55154 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x55154 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x55154 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x55154 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x54f44 | `(unknown callee) 0x200a0` | `0x9` | `0xf8fd4` | `0x71` | `0x0` |
| `bl` at 0x54f68 | `(unknown callee) 0x19748` | `0x13252c` | `0x0` | `0x20` | `[0x132528]+1` |
| `bl` at 0x54f74 | `(unknown callee) 0x1967c` | `0xf8fe8` | `0x1002` | `?` | `?` |
| `bl` at 0x54f8c | `(unknown callee) 0x193d0` | `ret(0x1967c)` | `?` | `?` | `ret(0x1967c)` |
| `bl` at 0x54f98 | `(unknown callee) 0x1f8e0` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |
| `bl` at 0x54fa0 | `(unknown callee) 0x193d0` | `ret(0x1f8e0)` | `?` | `?` | `?` |
| `bl` at 0x54fd8 | `(unknown callee) 0x200a0` | `0x3` | `0xf8fd4` | `0x84` | `0x0` |
| `bl` at 0x55000 | `(unknown callee) 0x193b8` | `0x0` | `0x8000` | `0x3` | `0x1` |
| `bl` at 0x55014 | `(unknown callee) 0x193d0` | `ret(0x193b8)` | `?` | `?` | `ret(0x193b8)` |
| `bl` at 0x55020 | `(unknown callee) 0x1f8e0` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |
| `bl` at 0x55028 | `(unknown callee) 0x193d0` | `ret(0x1f8e0)` | `?` | `?` | `?` |
| `bl` at 0x55060 | `(unknown callee) 0x200a0` | `0x3` | `0xf8fd4` | `0x8a` | `0x0` |
| `bl` at 0x55068 | `(unknown callee) 0x1931c` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x5509c | `(unknown callee) 0x1eeb0` | `ret(0x193b8)` | `?` | `?` | `?` |
| `bl` at 0x550b4 | `(unknown callee) 0x55188` | `?` | `?` | `?` | `?` |
| `bl` at 0x550d4 | `(unknown callee) 0x19610` | `[0x1324cc]+128` | `[0x1324cc]` | `0x80` | `[0x1324cc]+128` |
| `bl` at 0x550d8 | `(unknown callee) 0x1eeb0` | `ret(0x19610)` | `?` | `?` | `?` |
| `bl` at 0x55134 | `(unknown callee) 0x1e4ac` | `?` | `?` | `0x13252c` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x54f28 | `[0x132528] == 0x0` | `[0x132528] != 0x0` |
| `bge` at 0x54f88 | `ret(0x1967c) >= 0x0 (signed, 32-bit)` | `ret(0x1967c) < 0x0 (signed, 32-bit)` |
| `bne` at 0x55010 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x55088 | `[0x1324cc] != 0x0` | `[0x1324cc] == 0x0` |
| `beq` at 0x550e4 | `ret(0x1eeb0) == 0x0` | `ret(0x1eeb0) != 0x0` |
| `beq` at 0x550f4 | `[0x1324d0]:1 == 0x0` | `[0x1324d0]:1 != 0x0` |
| `beq` at 0x5511c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x55154 | `r0` | `?` |

## `vramSet`

- `vramSet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x55300 to 0x554b8.
  Established. Word assignments save entry `r0` and `r1` at `sp -0x18` and `sp -0x1c` (`str` at 0x55310, `str` at 0x55314).
  The unsigned condition `r0-1 <= 0x30` selects an unresolved computed transfer, while greater falls through (`ldrls` at 0x55324).
  Storage updates behind that transfer remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x55310 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x55314 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x55318 | load | 4 | `sp -0x18` | `-` |
| `ldrls` at 0x55324 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x554b0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x554b0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x554b0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x554b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x55324 | `r0-1 <= 0x30 (unsigned, 32-bit)` | `r0-1 > 0x30 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x554b0 | `r0` | `r0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x55324).

## `vramGet`

- `vramGet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x554b8 to 0x55640.
  Established. The computed transfer is selected when `r0-1 <= 0x30` unsigned, with greater falling through (`ldrls` at 0x554d8).
  On the known fallback, a word assignment writes `0x0` to `sp -0x18` (`str` at 0x5562c).
  The retained return candidate is `0x0` (`ldm` at 0x55638).
  Results through unresolved selector edges remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x554c8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x554cc | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0x554d8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x5562c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x55630 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x55638 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x55638 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x55638 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x554d8 | `r0-1 <= 0x30 (unsigned, 32-bit)` | `r0-1 > 0x30 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x55638 | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x554d8).

## `promIceInit`

- `promIceInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e690 to 0x3e760.
  Established. The unnamed `0x19748` call carries `[0x299734]`, `0x1`, `[sp-28]`, `0x299734` (`bl` at 0x3e724).
  Included word stores write `ret(0x196d0)` to `0x299734`, `[sp-24]` to `0x299730` and `[sp-28]` to `0x29972c` (`str` at 0x3e6e8, `str` at 0x3e730, `str` at 0x3e73c).
  PROM interface storage layout remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3e6a0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x3e6a4 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x3e6a8 | load | 4 | `abs 0x3e748` | `-` |
| `ldr` at 0x3e6ac | load | 4 | `abs 0x299730` | `-` |
| `ldr` at 0x3e6b8 | load | 4 | `abs 0x3e74c` | `-` |
| `str` at 0x3e6bc | store | 4 | `sp -0x20` | `0xf645c` |
| `ldr` at 0x3e6c4 | load | 4 | `abs 0x3e750` | `-` |
| `ldr` at 0x3e6d4 | load | 4 | `abs 0x3e754` | `-` |
| `ldr` at 0x3e6d8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3e6e8 | store | 4 | `abs 0x299734` | `ret(0x196d0)` |
| `ldr` at 0x3e6ec | load | 4 | `abs 0x299734` | `-` |
| `ldr` at 0x3e6f8 | load | 4 | `abs 0x3e758` | `-` |
| `str` at 0x3e6fc | store | 4 | `sp -0x20` | `0xf6498` |
| `ldr` at 0x3e704 | load | 4 | `abs 0x3e750` | `-` |
| `ldr` at 0x3e714 | load | 4 | `abs 0x3e754` | `-` |
| `ldr` at 0x3e718 | load | 4 | `abs 0x299734` | `-` |
| `ldr` at 0x3e720 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3e728 | load | 4 | `abs 0x3e748` | `-` |
| `ldr` at 0x3e72c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3e730 | store | 4 | `abs 0x299730` | `[sp-24]` |
| `ldr` at 0x3e734 | load | 4 | `abs 0x3e75c` | `-` |
| `ldr` at 0x3e738 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3e73c | store | 4 | `abs 0x29972c` | `[sp-28]` |
| `ldm` at 0x3e744 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x3e744 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3e744 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3e744 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3e6d0 | `(unknown callee) 0x200a0` | `0x9` | `0xf6480` | `0x39` | `0x0` |
| `bl` at 0x3e6e0 | `(unknown callee) 0x196d0` | `?` | `0x1` | `?` | `?` |
| `bl` at 0x3e710 | `(unknown callee) 0x200a0` | `0x9` | `0xf6480` | `0x40` | `0x0` |
| `bl` at 0x3e724 | `(unknown callee) 0x19748` | `[0x299734]` | `0x1` | `[sp-28]` | `0x299734` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3e6b4 | `[0x299730] == 0x0` | `[0x299730] != 0x0` |
| `bne` at 0x3e6f4 | `[0x299734] != 0x0` | `[0x299734] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3e744 | `r0` | `ret(0x19748)` |

## `promIcePresent`

- `promIcePresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e93c to 0x3e9a0.
  Established. A signed `(r0 & 0xff) >= [0x29972c]` takes `0x3e984`, while lower falls through (`bge` at 0x3e964).
  The included byte read addresses `((r0 & 0xff) + [0x299734]) +0x0` (`ldrb` at 0x3e978).
  Presence flag meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x3e950 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x3e954 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x3e958 | load | 4 | `abs 0x3e998` | `-` |
| `ldr` at 0x3e95c | load | 4 | `abs 0x29972c` | `-` |
| `ldrb` at 0x3e968 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x3e96c | load | 4 | `abs 0x3e99c` | `-` |
| `ldr` at 0x3e970 | load | 4 | `abs 0x299734` | `-` |
| `ldrb` at 0x3e978 | load | 1 | `((r0 & 0xff) + [0x299734]) +0x0` | `-` |
| `str` at 0x3e97c | store | 4 | `sp -0x18` | `[((r0 & 0xff) + [0x299734])]:1` |
| `str` at 0x3e988 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x3e98c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x3e994 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3e994 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3e994 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x3e964 | `(r0 & 0xff) >= [0x29972c] (signed, 32-bit)` | `(r0 & 0xff) < [0x29972c] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3e994 | `r0` | `?` |

## `promIceNotPresent`

- `promIceNotPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e760 to 0x3e7b0.
  Established. A signed `(r0 & 0xff) < [0x29972c]` falls through, while greater or equal takes `0x3e7a4` (`bge` at 0x3e788).
  The fall-through byte store writes `0x0` to `((r0 & 0xff) + [0x299734]) +0x0` (`strb` at 0x3e7a0).
  The cleared flag role remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x3e774 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x3e778 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x3e77c | load | 4 | `abs 0x3e7a8` | `-` |
| `ldr` at 0x3e780 | load | 4 | `abs 0x29972c` | `-` |
| `ldrb` at 0x3e78c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x3e790 | load | 4 | `abs 0x3e7ac` | `-` |
| `ldr` at 0x3e794 | load | 4 | `abs 0x299734` | `-` |
| `strb` at 0x3e7a0 | store | 1 | `((r0 & 0xff) + [0x299734]) +0x0` | `0x0` |
| `ldm` at 0x3e7a4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x3e7a4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3e7a4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3e7a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x3e788 | `(r0 & 0xff) >= [0x29972c] (signed, 32-bit)` | `(r0 & 0xff) < [0x29972c] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3e7a4 | `r0` | `r0` |

## `promIceGetCfg`

- `promIceGetCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e8e0 to 0x3e91c.
  Established. A word assignment copies `[0x299730]` to `r0 +0x0` (`str` at 0x3e900).
  The integer return record supplies `[0x29972c]` (`ldm` at 0x3e910).
  Configuration pointee fields remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3e8f0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x3e8f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3e8f8 | load | 4 | `abs 0x3e914` | `-` |
| `ldr` at 0x3e8fc | load | 4 | `abs 0x299730` | `-` |
| `str` at 0x3e900 | store | 4 | `r0 +0x0` | `[0x299730]` |
| `ldr` at 0x3e904 | load | 4 | `abs 0x3e918` | `-` |
| `ldr` at 0x3e908 | load | 4 | `abs 0x29972c` | `-` |
| `ldm` at 0x3e910 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x3e910 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3e910 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3e910 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3e910 | `r0` | `[0x29972c]` |

## `promIceDisable`

- `promIceDisable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3eb8c to 0x3ec60.
  Established. An entry selector `(r0 & 0xff)` unequal to `0xff` takes `0x3ec3c`, while equality falls through (`bne` at 0x3ebb4).
  The unnamed `0x3ed14` call carries `(r0 & 0xff)`, entry `r1`, entry `r2`, `(r0 & 0xff)` (`bl` at 0x3ec44).
  Disable callback policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x3eba0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x3eba8 | store | 4 | `sp -0x18` | `0x0` |
| `ldrb` at 0x3ebac | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x3ebbc | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0x3ebc0 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x3ebc4 | load | 4 | `abs 0x3ec5c` | `-` |
| `ldr` at 0x3ebc8 | load | 4 | `abs 0x29972c` | `-` |
| `ldrb` at 0x3ebd4 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x3ebec | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x3ebfc | store | 4 | `sp -0x1c` | `ret(0x3ed14)` |
| `ldr` at 0x3ec00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3ec0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ec18 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3ec1c | store | 4 | `sp -0x18` | `ret(0x3ed14)` |
| `ldrb` at 0x3ec20 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x3ec28 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldr` at 0x3ec30 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3ec34 | store | 4 | `sp -0x24` | `?` |
| `ldrb` at 0x3ec3c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x3ec4c | store | 4 | `sp -0x24` | `ret(0x3ed14)` |
| `ldr` at 0x3ec50 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x3ec58 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3ec58 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3ec58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3ebdc | `(unknown callee) 0x3e93c` | `?` | `?` | `[0x29972c]` | `?` |
| `bl` at 0x3ebf4 | `(unknown callee) 0x3ed14` | `[sp-29]:1` | `?` | `?` | `[sp-29]:1` |
| `bl` at 0x3ec44 | `(unknown callee) 0x3ed14` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3ebb4 | `(r0 & 0xff) != 0xff` | `(r0 & 0xff) == 0xff` |
| `bge` at 0x3ebd0 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x3ebe8 | `ret(0x3e93c) == 0x0` | `ret(0x3e93c) != 0x0` |
| `beq` at 0x3ec08 | `ret(0x3ed14) == 0x0` | `ret(0x3ed14) != 0x0` |
| `bne` at 0x3ec14 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3ec58 | `r0` | `?` |

## `IP57_GetCpuMask`

- `IP57_GetCpuMask`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x57934 to 0x57964.
  Established. The indexed byte read addresses `(r0 & 0xff) +0x13256c` (`ldrb` at 0x57954).
  The return candidate is `[(r0 & 0xff)+1254764]:1` (`ldm` at 0x5795c).
  CPU mask bit ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x57948 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x5794c | load | 4 | `abs 0x57960` | `-` |
| `ldrb` at 0x57950 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x57954 | load | 1 | `(r0 & 0xff) +0x13256c` | `-` |
| `ldm` at 0x5795c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x5795c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x5795c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x5795c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x5795c | `r0` | `[(r0 & 0xff)+1254764]:1` |

## `IP57_CpuEnable`

- `IP57_CpuEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x56914 to 0x56988.
  Established. A zero `ret(0x56824)` takes `0x56970`, while nonzero falls through (`beq` at 0x56940).
  The unnamed `0x56b4c` call has four unknown candidates (`bl` at 0x56960).
  An included word assignment writes `0xfffff50e` to `sp -0x18` (`str` at 0x56974).
  Processor enable result meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x56928 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x5692c | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x56944 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x56950 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x56968 | store | 4 | `sp -0x18` | `ret(0x56b4c)` |
| `ldr` at 0x56970 | load | 4 | `abs 0x56984` | `-` |
| `str` at 0x56974 | store | 4 | `sp -0x18` | `0xfffff50e` |
| `ldr` at 0x56978 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x56980 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x56980 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x56980 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x56934 | `(unknown callee) 0x56824` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x56960 | `(unknown callee) 0x56b4c` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x56940 | `ret(0x56824) == 0x0` | `ret(0x56824) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x56980 | `r0` | `?` |

## `IP57_CpuDisable`

- `IP57_CpuDisable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x568a0 to 0x56914.
  Established. The unnamed `0x56824` call carries `(r0 & 0xff)`, entry `r1`, entry `r2`, `(r0 & 0xff)` (`bl` at 0x568c0).
  A zero result takes `0x568fc`, while nonzero falls through (`beq` at 0x568cc).
  Another unnamed `0x56988` call has four unknown candidates (`bl` at 0x568ec).
  Processor disable effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x568b4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x568b8 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x568d0 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x568dc | load | 1 | `sp -0x11` | `-` |
| `str` at 0x568f4 | store | 4 | `sp -0x18` | `ret(0x56988)` |
| `ldr` at 0x568fc | load | 4 | `abs 0x56910` | `-` |
| `str` at 0x56900 | store | 4 | `sp -0x18` | `0xfffff50e` |
| `ldr` at 0x56904 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x5690c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x5690c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x5690c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x568c0 | `(unknown callee) 0x56824` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x568ec | `(unknown callee) 0x56988` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x568cc | `ret(0x56824) == 0x0` | `ret(0x56824) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x5690c | `r0` | `?` |

## `cpuCount`

- `cpuCount`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x54224 to 0x54244.
  Established. The word read addresses `0x297608` (`ldr` at 0x54234);
  its loaded value `[0x297608]` is the integer return candidate (`ldm` at 0x5423c).
  The count source and units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x54230 | load | 4 | `abs 0x54240` | `-` |
| `ldr` at 0x54234 | load | 4 | `abs 0x297608` | `-` |
| `ldm` at 0x5423c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x5423c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x5423c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x5423c | `r0` | `[0x297608]` |

## `nodeCount`

- `nodeCount`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x548c4 to 0x548e4.
  Established. A word is read from `0x297634` (`ldr` at 0x548d4).
  The integer return preserves `[0x297634]` (`ldm` at 0x548dc).
  Node count synchronization remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x548d0 | load | 4 | `abs 0x548e0` | `-` |
| `ldr` at 0x548d4 | load | 4 | `abs 0x297634` | `-` |
| `ldm` at 0x548dc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x548dc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x548dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x548dc | `r0` | `[0x297634]` |

## `IP57_NodePresent`

- `IP57_NodePresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x567e8 to 0x56824.
  Established. An included byte read addresses `(r0 & 0xff) +0x299848` (`ldrb` at 0x56808).
  The integer return candidate is unknown (`ldm` at 0x5681c).
  The table entry encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x567fc | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x56800 | load | 4 | `abs 0x56820` | `-` |
| `ldrb` at 0x56804 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x56808 | load | 1 | `(r0 & 0xff) +0x299848` | `-` |
| `ldm` at 0x5681c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x5681c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x5681c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x5681c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x5681c | `r0` | `?` |

## `PICActive`

- `PICActive`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x34aa8 to 0x34ad0.
  Established. A byte read addresses `0x13212d` (`ldrb` at 0x34ac0).
  The integer return record carries `[0x13212d]:1` (`ldm` at 0x34ac8).
  PIC activity flag ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x34ab8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x34abc | load | 4 | `abs 0x34acc` | `-` |
| `ldrb` at 0x34ac0 | load | 1 | `abs 0x13212d` | `-` |
| `ldm` at 0x34ac8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x34ac8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x34ac8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x34ac8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x34ac8 | `r0` | `[0x13212d]:1` |
