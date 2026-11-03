# ARM regulator interrupts and environmental state in 1.44.0

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
No unresolved computed transfers were reported for these routines.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/regulator-environment-facts.txt) retain all tool sections.

## `vrmIntDisable`

- `vrmIntDisable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24f6c to 0x25170.
  Established. The byte through `[sp-32] +0x9` receives zero (`strb` at 0x24fac). The unnamed
  `0xca878` call carries `[sp-28]`, unknown, unknown, unknown (`bl` at 0x2506c); its zero result
  takes `0x25144`, while nonzero falls through (`beq` at 0x25080). Regulator interrupt disable
  effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x24f7c | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0x24f80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x24f94 | load | 4 | `abs 0x2515c` | `-` |
| `ldr` at 0x24f98 | load | 4 | `abs 0x132068` | `-` |
| `str` at 0x24fa0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x24fa4 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x24fac | store | 1 | `[sp-32] +0x9` | `0x0` |
| `str` at 0x24fb4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x24fb8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x24fbc | load | 1 | `[sp-32] +0x8` | `-` |
| `ldr` at 0x24fc0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x24fcc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x24fd0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x24fd8 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0x24fe0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x24fe4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x24fe8 | load | 4 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x24fec | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x24ff4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x25000 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x25004 | load | 4 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x25008 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x25010 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x2501c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x25020 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x25024 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x25040 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x25044 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x25048 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x2504c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x25050 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x25054 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x25060 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x25074 | store | 4 | `sp -0x24` | `ret(0xca878)` |
| `ldr` at 0x25078 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x25084 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2508c | load | 4 | `abs 0x25160` | `-` |
| `ldr` at 0x25090 | load | 4 | `abs 0x25164` | `-` |
| `str` at 0x250cc | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x250d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x250d8 | load | 4 | `abs 0x25160` | `-` |
| `ldr` at 0x250dc | load | 4 | `abs 0x25164` | `-` |
| `ldr` at 0x250e8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x250f8 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x250fc | load | 1 | `[sp-44] +0x0` | `-` |
| `ldr` at 0x25100 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x25110 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x2511c | load | 4 | `abs 0x25168` | `-` |
| `str` at 0x25120 | store | 4 | `(unknown)` | `0xee6b8` |
| `str` at 0x25124 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `str` at 0x25128 | store | 4 | `(unknown)` | `ret(0xcb12c)` |
| `str` at 0x2512c | store | 4 | `(unknown)` | `ret(0x1fc1c)` |
| `ldr` at 0x25134 | load | 4 | `abs 0x2516c` | `-` |
| `ldr` at 0x25144 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x2514c | store | 4 | `sp -0x28` | `?` |
| `ldm` at 0x25158 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x25158 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x25158 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x25158 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x25158 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2502c | `(unknown callee) 0xca484` | `[sp-28]` | `?` | `[[sp-32]+4]` | `?` |
| `bl` at 0x2506c | `(unknown callee) 0xca878` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x25094 | `(unknown callee) 0x48da8` | `[sp-28]` | `0x0` | `0xee604` | `0xee60c` |
| `bl` at 0x250a0 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x250e0 | `(unknown callee) 0x48da8` | `[sp-28]` | `0x0` | `0xee604` | `0xee60c` |
| `bl` at 0x250f0 | `(unknown callee) 0x19808` | `[sp-48]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x25108 | `(unknown callee) 0xcb12c` | `[sp-28]` | `[[sp-44]]:1` | `?` | `[[sp-44]]:1` |
| `bl` at 0x25114 | `(unknown callee) 0x1fc1c` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0x25140 | `(unknown callee) 0x200a0` | `0x3` | `0xee5b8` | `0xbc` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x24fc8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x24ffc | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x25018 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x25038 | `ret(0xca484) != 0x0` | `ret(0xca484) == 0x0` |
| `beq` at 0x25080 | `ret(0xca878) == 0x0` | `ret(0xca878) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x25158 | `r0` | `?` |

## `vrmIntEnable`

- `vrmIntEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24da8 to 0x24f6c.
  Established. The byte through `[sp-32] +0x9` receives `0x1` (`strb` at 0x24de8). The unnamed
  `0xca8c4` call carries `[sp-28]`, unknown, unknown, unknown (`bl` at 0x24e68); its zero result
  takes `0x24f40`, while nonzero falls through (`beq` at 0x24e7c). Regulator interrupt enable policy
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x24db8 | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0x24dbc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x24dd0 | load | 4 | `abs 0x24f58` | `-` |
| `ldr` at 0x24dd4 | load | 4 | `abs 0x132068` | `-` |
| `str` at 0x24ddc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x24de0 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x24de8 | store | 1 | `[sp-32] +0x9` | `0x1` |
| `str` at 0x24df0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x24df4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x24df8 | load | 1 | `[sp-32] +0x8` | `-` |
| `ldr` at 0x24dfc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x24e08 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x24e0c | load | 4 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x24e10 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x24e18 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x24e24 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x24e28 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x24e30 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0x24e38 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x24e3c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x24e40 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x24e44 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x24e48 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x24e4c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x24e50 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x24e5c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x24e70 | store | 4 | `sp -0x24` | `ret(0xca8c4)` |
| `ldr` at 0x24e74 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x24e80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x24e88 | load | 4 | `abs 0x24f5c` | `-` |
| `ldr` at 0x24e8c | load | 4 | `abs 0x24f60` | `-` |
| `str` at 0x24ec8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x24ecc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x24ed4 | load | 4 | `abs 0x24f5c` | `-` |
| `ldr` at 0x24ed8 | load | 4 | `abs 0x24f60` | `-` |
| `ldr` at 0x24ee4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x24ef4 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x24ef8 | load | 1 | `[sp-44] +0x0` | `-` |
| `ldr` at 0x24efc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x24f0c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x24f18 | load | 4 | `abs 0x24f64` | `-` |
| `str` at 0x24f1c | store | 4 | `(unknown)` | `0xee680` |
| `str` at 0x24f20 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `str` at 0x24f24 | store | 4 | `(unknown)` | `ret(0xcb12c)` |
| `str` at 0x24f28 | store | 4 | `(unknown)` | `ret(0x1fc1c)` |
| `ldr` at 0x24f30 | load | 4 | `abs 0x24f68` | `-` |
| `ldr` at 0x24f40 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x24f48 | store | 4 | `sp -0x28` | `?` |
| `ldm` at 0x24f54 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x24f54 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x24f54 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x24f54 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x24f54 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x24e68 | `(unknown callee) 0xca8c4` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x24e90 | `(unknown callee) 0x48da8` | `[sp-28]` | `0x0` | `0xee604` | `0xee60c` |
| `bl` at 0x24e9c | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x24edc | `(unknown callee) 0x48da8` | `[sp-28]` | `0x0` | `0xee604` | `0xee60c` |
| `bl` at 0x24eec | `(unknown callee) 0x19808` | `[sp-48]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x24f04 | `(unknown callee) 0xcb12c` | `[sp-28]` | `[[sp-44]]:1` | `?` | `[[sp-44]]:1` |
| `bl` at 0x24f10 | `(unknown callee) 0x1fc1c` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0x24f3c | `(unknown callee) 0x200a0` | `0x3` | `0xee5b8` | `0xa3` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x24e04 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x24e20 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x24e7c | `ret(0xca8c4) == 0x0` | `ret(0xca8c4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x24f54 | `r0` | `?` |

## `vrmPresent`

- `vrmPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x252d0 to 0x253d8.
  Established. The unsigned condition `[[sp-28]+8]:1 <= (r1 & 0xff)` takes `0x253c0`, while greater
  values fall through (`bls` at 0x25320). The unnamed `0xca70c` call carries entry `r0`,
  `[[sp-36]]:1`, `sp-29`, `[[sp-36]]:1` (`bl` at 0x25388). An included word assignment at `sp -0x28`
  writes `0x1` (`str` at 0x2536c). Regulator presence interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x252e0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x252e8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x252ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25300 | load | 4 | `abs 0x253d4` | `-` |
| `ldr` at 0x25304 | load | 4 | `abs 0x132068` | `-` |
| `str` at 0x2530c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x25310 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x25314 | load | 1 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0x25318 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x25324 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x25328 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x2532c | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x25334 | load | 1 | `((r1 & 0xff) + [[sp-28]+4]) +0x0` | `-` |
| `ldr` at 0x25340 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x25344 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x2534c | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x25354 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x25358 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x2535c | load | 1 | `[sp-36] +0x3` | `-` |
| `str` at 0x2536c | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x25374 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x25378 | load | 1 | `[sp-36] +0x0` | `-` |
| `ldr` at 0x25380 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2539c | store | 4 | `sp -0x28` | `?` |
| `ldrb` at 0x253a4 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x253a8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x253b8 | store | 4 | `sp -0x28` | `ret(0x25248)` |
| `str` at 0x253c4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x253c8 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x253d0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x253d0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x253d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x25388 | `(unknown callee) 0xca70c` | `r0` | `[[sp-36]]:1` | `sp-29` | `[[sp-36]]:1` |
| `bl` at 0x253b0 | `(unknown callee) 0x25248` | `[sp-36]` | `[sp-29]:1` | `?` | `[sp-29]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0x25320 | `[[sp-28]+8]:1 <= (r1 & 0xff) (unsigned, 32-bit)` | `[[sp-28]+8]:1 > (r1 & 0xff) (unsigned, 32-bit)` |
| `beq` at 0x2533c | `[((r1 & 0xff) + [[sp-28]+4])]:1 == 0x0` | `[((r1 & 0xff) + [[sp-28]+4])]:1 != 0x0` |
| `bne` at 0x25364 | `[[sp-36]+3]:1 != 0x0` | `[[sp-36]+3]:1 == 0x0` |
| `beq` at 0x25394 | `ret(0xca70c) == 0x0` | `ret(0xca70c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x253d0 | `r0` | `[sp-40]` |

## `vrmEnabled`

- `vrmEnabled`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x25170 to 0x251c0.
  Established. The absolute word at `0x132068` is read (`ldr` at 0x2519c). The byte through `[sp-24] +0x9` is read (`ldrb` at 0x251ac). The included return record carries `[[sp-24]+9]:1` in `r0`
  (`ldm` at 0x251b8). Regulator enable flag meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x25180 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x25184 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25198 | load | 4 | `abs 0x251bc` | `-` |
| `ldr` at 0x2519c | load | 4 | `abs 0x132068` | `-` |
| `str` at 0x251a4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x251a8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x251ac | load | 1 | `[sp-24] +0x9` | `-` |
| `ldm` at 0x251b8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x251b8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x251b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x251b8 | `r0` | `[[sp-24]+9]:1` |

## `vrmOK`

- `vrmOK`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x253d8 to 0x255ac.
  Established. A nonzero `ret(0x252d0)` takes `0x25470`, while zero falls through (`bne` at 0x25460). The unnamed `0xca70c` call carries `[sp-20]`, `[[sp-36]]:1`, `sp-29`, `[[sp-36]]:1`
  (`bl` at 0x2549c). Separate included assignments at `sp -0x28` write zero and `0x1` (`str` at 0x25560, `str` at 0x2556c). Regulator health semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x253e8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x253f0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x253f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25408 | load | 4 | `abs 0x255a8` | `-` |
| `ldr` at 0x2540c | load | 4 | `abs 0x132068` | `-` |
| `str` at 0x25414 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x25418 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x2541c | load | 1 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0x25420 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x2542c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x25430 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x25434 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x2543c | load | 1 | `((r1 & 0xff) + [[sp-28]+4]) +0x0` | `-` |
| `ldrb` at 0x25448 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x2544c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25468 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x25470 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x25474 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x2547c | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x25484 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x25488 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x2548c | load | 1 | `[sp-36] +0x0` | `-` |
| `ldr` at 0x25494 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x254b0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x254b8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x254bc | load | 1 | `[sp-36] +0x1` | `-` |
| `ldr` at 0x254c0 | load | 4 | `sp -0x14` | `-` |
| `ldc` at 0x254d0 | load | unknown | `abs 0x255a4` | `-` |
| `ldrb` at 0x254e0 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x254e8 | store | 1 | `sp -0x1d` | `?` |
| `ldr` at 0x254ec | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x254f0 | load | 1 | `[sp-36] +0x5` | `-` |
| `ldr` at 0x25500 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x25504 | load | 1 | `[sp-36] +0x4` | `-` |
| `ldrb` at 0x25510 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x25514 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x25530 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x25534 | load | 1 | `[sp-36] +0x3` | `-` |
| `ldrb` at 0x25540 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x25544 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x25560 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x2556c | store | 4 | `sp -0x28` | `0x1` |
| `ldrb` at 0x25574 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x25578 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x25588 | store | 4 | `sp -0x28` | `ret(0x251c0)` |
| `str` at 0x25594 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x25598 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x255a0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x255a0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x255a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x25454 | `(unknown callee) 0x252d0` | `r0` | `(r1 & 0xff)` | `(r1 & 0xff)` | `(r1 & 0xff)` |
| `bl` at 0x2549c | `(unknown callee) 0xca70c` | `[sp-20]` | `[[sp-36]]:1` | `sp-29` | `[[sp-36]]:1` |
| `bl` at 0x254c8 | `(unknown callee) 0x39ddc` | `[sp-20]` | `[[sp-36]+1]:1` | `?` | `[[sp-36]+1]:1` |
| `bl` at 0x2551c | `(unknown callee) 0x251c0` | `[sp-36]` | `[sp-29]:1` | `?` | `[sp-29]:1` |
| `bl` at 0x2554c | `(unknown callee) 0x25248` | `[sp-36]` | `[sp-29]:1` | `?` | `[sp-29]:1` |
| `bl` at 0x25580 | `(unknown callee) 0x251c0` | `[sp-36]` | `[sp-29]:1` | `?` | `[sp-29]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0x25428 | `[[sp-28]+8]:1 <= (r1 & 0xff) (unsigned, 32-bit)` | `[[sp-28]+8]:1 > (r1 & 0xff) (unsigned, 32-bit)` |
| `beq` at 0x25444 | `[((r1 & 0xff) + [[sp-28]+4])]:1 == 0x0` | `[((r1 & 0xff) + [[sp-28]+4])]:1 != 0x0` |
| `bne` at 0x25460 | `ret(0x252d0) != 0x0` | `ret(0x252d0) == 0x0` |
| `beq` at 0x254a8 | `ret(0xca70c) == 0x0` | `ret(0xca70c) != 0x0` |
| `beq` at 0x254d8 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x254fc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x2550c | `[[sp-36]+4]:1 == 0x0` | `[[sp-36]+4]:1 != 0x0` |
| `bne` at 0x25528 | `ret(0x251c0) != 0x0` | `ret(0x251c0) == 0x0` |
| `beq` at 0x2553c | `[[sp-36]+3]:1 == 0x0` | `[[sp-36]+3]:1 != 0x0` |
| `bne` at 0x25558 | `ret(0x25248) != 0x0` | `ret(0x25248) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x255a0 | `r0` | `[sp-40]` |

## `vrmAllPresent`

- `vrmAllPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x255ac to 0x25744.
  Established. The word at `sp -0x20` starts at `0x1` (`str` at 0x255e8). A nonzero `ret(0x252d0)`
  takes `0x2570c`, while zero falls through (`bne` at 0x25624). An included assignment to that stack
  word writes zero (`str` at 0x25708). The unnamed `0x200a0` call carries `0x1`, `0xee5b8`, `0x12b`,
  `0x0` (`bl` at 0x25700). Aggregate presence policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x255bc | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x255c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x255d4 | load | 4 | `abs 0x2572c` | `-` |
| `ldr` at 0x255d8 | load | 4 | `abs 0x132068` | `-` |
| `str` at 0x255e0 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x255e8 | store | 4 | `sp -0x20` | `0x1` |
| `str` at 0x255f0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x255f4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x255f8 | load | 1 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x255fc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x25608 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x25610 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x25628 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x25630 | load | 4 | `abs 0x25730` | `-` |
| `ldr` at 0x25634 | load | 4 | `abs 0x25734` | `-` |
| `str` at 0x25670 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x25674 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2567c | load | 4 | `abs 0x25730` | `-` |
| `ldr` at 0x25680 | load | 4 | `abs 0x25734` | `-` |
| `ldr` at 0x2568c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x2569c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x256a0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x256a8 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x256b0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x256b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x256c4 | load | 4 | `abs 0x25738` | `-` |
| `str` at 0x256c8 | store | 4 | `(unknown)` | `0xee6f0` |
| `str` at 0x256cc | store | 4 | `(unknown)` | `ret(0x19808)` |
| `str` at 0x256d0 | store | 4 | `(unknown)` | `ret(0x39b0c)` |
| `ldr` at 0x256d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x256d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x256e0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x256e8 | load | 1 | `(unknown)` | `-` |
| `str` at 0x256ec | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x256f4 | load | 4 | `abs 0x2573c` | `-` |
| `ldr` at 0x256f8 | load | 4 | `abs 0x25740` | `-` |
| `str` at 0x25708 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x2570c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x25714 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x2571c | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x25728 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x25728 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x25728 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x25728 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x25618 | `(unknown callee) 0x252d0` | `?` | `?` | `[[sp-28]+8]:1` | `?` |
| `bl` at 0x25638 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0xee604` | `0xee60c` |
| `bl` at 0x25644 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x25684 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0xee604` | `0xee60c` |
| `bl` at 0x25694 | `(unknown callee) 0x19808` | `[sp-40]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x256bc | `(unknown callee) 0x39b0c` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x25700 | `(unknown callee) 0x200a0` | `0x1` | `0xee5b8` | `0x12b` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x25604 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x25624 | `ret(0x252d0) != 0x0` | `ret(0x252d0) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x25728 | `r0` | `?` |

## `vrmAllOK`

- `vrmAllOK`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x25744 to 0x25914.
  Established. A nonzero `ret(0x253d8)` takes `0x258dc`, while zero falls through (`bne` at 0x257bc).
  The word at `sp -0x20` starts at `0x1` and receives zero on an included path (`str` at 0x25780, `str` at 0x258d8).
  The coprocessor load from `0x258fc` has unknown transfer width (`ldc`
  at 0x257e8). Aggregate regulator health behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x25754 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x25758 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2576c | load | 4 | `abs 0x25900` | `-` |
| `ldr` at 0x25770 | load | 4 | `abs 0x132068` | `-` |
| `str` at 0x25778 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x25780 | store | 4 | `sp -0x20` | `0x1` |
| `str` at 0x25788 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x2578c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x25790 | load | 1 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x25794 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x257a0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x257a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x257c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x257c4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x257cc | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x257d4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x257d8 | load | 4 | `sp -0x18` | `-` |
| `ldc` at 0x257e8 | load | unknown | `abs 0x258fc` | `-` |
| `ldr` at 0x257f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x25800 | load | 4 | `abs 0x25904` | `-` |
| `ldr` at 0x25804 | load | 4 | `abs 0x25908` | `-` |
| `str` at 0x25840 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x25844 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2584c | load | 4 | `abs 0x25904` | `-` |
| `ldr` at 0x25850 | load | 4 | `abs 0x25908` | `-` |
| `ldr` at 0x2585c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x2586c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x25870 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x25878 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x25880 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x25884 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x25894 | load | 4 | `abs 0x2590c` | `-` |
| `str` at 0x25898 | store | 4 | `(unknown)` | `0xee718` |
| `str` at 0x2589c | store | 4 | `(unknown)` | `ret(0x19808)` |
| `str` at 0x258a0 | store | 4 | `(unknown)` | `ret(0x39b0c)` |
| `ldr` at 0x258a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x258a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x258b0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x258b8 | load | 1 | `(unknown)` | `-` |
| `str` at 0x258bc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x258c4 | load | 4 | `abs 0x25910` | `-` |
| `str` at 0x258d8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x258dc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x258e4 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x258ec | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x258f8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x258f8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x258f8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x258f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x257b0 | `(unknown callee) 0x253d8` | `?` | `?` | `[[sp-28]+8]:1` | `?` |
| `bl` at 0x257e0 | `(unknown callee) 0x39ddc` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x25808 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0xee604` | `0xee60c` |
| `bl` at 0x25814 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x25854 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0xee604` | `0xee60c` |
| `bl` at 0x25864 | `(unknown callee) 0x19808` | `[sp-44]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x2588c | `(unknown callee) 0x39b0c` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x258d0 | `(unknown callee) 0x200a0` | `0x1` | `0xee5b8` | `0x148` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x2579c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x257bc | `ret(0x253d8) != 0x0` | `ret(0x253d8) == 0x0` |
| `bne` at 0x257f0 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x258f8 | `r0` | `?` |

## `EnvTempInt`

- `EnvTempInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x72c2c to 0x72eb8.
  Established. The absolute word at `0x133220` is read (`ldr` at 0x72c5c). Included byte assignments
  through `[sp-28] +0x1` write `0x1` and zero at separate sites (`strb` at 0x72e50, `strb` at 0x72e70). The unnamed `0x2105c` call carries `0x132ff8`, unknown, `[sp-28]`, `0x1` (`bl` at 0x72e58). Environmental temperature interrupt handling remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x72c3c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x72c44 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x72c4c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x72c54 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x72c58 | load | 4 | `abs 0x72e88` | `-` |
| `ldr` at 0x72c5c | load | 4 | `abs 0x133220` | `-` |
| `ldr` at 0x72c60 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x72c6c | load | 4 | `abs 0x72e88` | `-` |
| `ldr` at 0x72c70 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x72c78 | load | 4 | `abs 0x13322c` | `-` |
| `ldrb` at 0x72c80 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x72c84 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x72c90 | load | 4 | `abs 0x72e88` | `-` |
| `ldr` at 0x72c94 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x72c9c | load | 4 | `abs 0x13322c` | `-` |
| `ldrb` at 0x72ca4 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x72ca8 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x72cb8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x72cc0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x72cc8 | load | 4 | `abs 0x72e88` | `-` |
| `ldr` at 0x72ccc | load | 4 | `abs 0x133220` | `-` |
| `ldr` at 0x72cd0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x72cdc | load | 4 | `abs 0x72e8c` | `-` |
| `str` at 0x72ce0 | store | 4 | `sp -0x34` | `0xfd718` |
| `ldrb` at 0x72ce4 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x72ce8 | store | 4 | `sp -0x30` | `(r1 & 0xff)` |
| `ldrb` at 0x72cec | load | 1 | `sp -0x16` | `-` |
| `str` at 0x72cf0 | store | 4 | `sp -0x2c` | `(r2 & 0xff)` |
| `ldr` at 0x72cf8 | load | 4 | `abs 0x72e90` | `-` |
| `ldr` at 0x72cfc | load | 4 | `abs 0x72e94` | `-` |
| `str` at 0x72d0c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x72d14 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x72d1c | load | 4 | `abs 0x72e98` | `-` |
| `str` at 0x72d24 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x72d28 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x72d2c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x72d38 | load | 4 | `abs 0x72e88` | `-` |
| `ldr` at 0x72d3c | load | 4 | `abs 0x13320c` | `-` |
| `ldr` at 0x72d48 | load | 4 | `abs 0x72e88` | `-` |
| `ldr` at 0x72d4c | load | 4 | `abs 0x133210` | `-` |
| `ldr` at 0x72d58 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x72d5c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x72d60 | load | 4 | `abs 0x72e88` | `-` |
| `ldr` at 0x72d64 | load | 4 | `abs 0x72e88` | `-` |
| `ldr` at 0x72d68 | load | 4 | `abs 0x72e9c` | `-` |
| `ldr` at 0x72d70 | load | 4 | `abs 0x13320c` | `-` |
| `ldr` at 0x72d74 | load | 4 | `abs 0x133210` | `-` |
| `ldr` at 0x72d7c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x72d94 | load | 4 | `abs 0x72ea0` | `-` |
| `str` at 0x72d98 | store | 4 | `sp -0x34` | `0xfd774` |
| `str` at 0x72d9c | store | 4 | `sp -0x30` | `ret(0x24628)` |
| `ldrb` at 0x72da0 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x72da4 | store | 4 | `sp -0x2c` | `[sp-21]:1` |
| `ldrb` at 0x72da8 | load | 1 | `sp -0x16` | `-` |
| `str` at 0x72dac | store | 4 | `sp -0x28` | `[sp-22]:1` |
| `ldr` at 0x72db4 | load | 4 | `abs 0x72e90` | `-` |
| `ldr` at 0x72db8 | load | 4 | `abs 0x72ea4` | `-` |
| `str` at 0x72dc8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x72dd0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x72de4 | load | 4 | `abs 0x72ea8` | `-` |
| `ldr` at 0x72de8 | load | 4 | `abs 0x132f14` | `-` |
| `ldr` at 0x72df8 | load | 4 | `abs 0x72eac` | `-` |
| `ldr` at 0x72dfc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x72e04 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x72e08 | load | 1 | `[sp-28] +0x1` | `-` |
| `ldr` at 0x72e14 | load | 4 | `abs 0x72ea8` | `-` |
| `ldr` at 0x72e18 | load | 4 | `abs 0x132f14` | `-` |
| `ldr` at 0x72e28 | load | 4 | `abs 0x72eb0` | `-` |
| `ldr` at 0x72e2c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x72e34 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x72e48 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x72e50 | store | 1 | `[sp-28] +0x1` | `0x1` |
| `ldr` at 0x72e54 | load | 4 | `abs 0x72eb4` | `-` |
| `ldr` at 0x72e68 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x72e70 | store | 1 | `[sp-28] +0x1` | `0x0` |
| `str` at 0x72e78 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x72e7c | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x72e84 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x72e84 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x72e84 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x72d04 | `(unknown callee) 0x200a0` | `0x1` | `0xfd2d0` | `0x592` | `0x0` |
| `bl` at 0x72d78 | `(unknown callee) 0x1e958` | `0xfd754` | `?` | `[0x13320c]` | `[0x133210]` |
| `bl` at 0x72d8c | `(unknown callee) 0x24628` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x72dc0 | `(unknown callee) 0x200a0` | `0x1` | `0xfd2d0` | `0x5a1` | `0x0` |
| `bl` at 0x72de0 | `(unknown callee) 0x244b4` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x72e00 | `(unknown callee) 0x221f0` | `0xfd7ac` | `[sp-32]` | `?` | `?` |
| `bl` at 0x72e30 | `(unknown callee) 0x221f0` | `0xfd7c0` | `[sp-32]` | `?` | `?` |
| `bl` at 0x72e44 | `(unknown callee) 0x244b4` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x72e58 | `(unknown callee) 0x2105c` | `0x132ff8` | `?` | `[sp-28]` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0x72c68 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x72c8c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x72cb0 | `Z == 0` | `not (Z == 0)` |
| `bhi` at 0x72cd8 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0x72d34 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x72d44 | `[0x13320c] != 0x0` | `[0x13320c] == 0x0` |
| `bne` at 0x72d54 | `[0x133210] != 0x0` | `[0x133210] == 0x0` |
| `beq` at 0x72df4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x72e10 | `[[sp-28]+1]:1 != 0x0` | `[[sp-28]+1]:1 == 0x0` |
| `beq` at 0x72e24 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x72e64 | `ret(0x2105c) == 0x0` | `ret(0x2105c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x72e84 | `r0` | `?` |

## `EnvTempConfigure`

- `EnvTempConfigure`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x76ff0 to 0x7701c.
  Established. The unnamed `0x72674` call carries all four entry registers (`bl` at 0x77004). The
  following unnamed `0x72378` call carries `[sp-20]`, `ret(0x72674)`, unknown, `ret(0x72674)` (`bl`
  at 0x77014). The included return record gives `ret(0x72378)` in `r0` (`ldm` at 0x77018).
  Temperature configuration selection remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x77000 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x7700c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x77018 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x77018 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x77018 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x77018 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x77004 | `(unknown callee) 0x72674` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x77014 | `(unknown callee) 0x72378` | `[sp-20]` | `ret(0x72674)` | `?` | `ret(0x72674)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x77018 | `r0` | `ret(0x72378)` |

## `EnvTemp`

- `EnvTemp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x76bb0 to 0x76be8.
  Established. The unnamed `0x1b5f0` call carries `0x7`, unknown, entry `r2`, unknown (`bl` at 0x76bdc). The following unnamed `0x76ca4` call carries `ret(0x1b5f0)`, unknown, unknown, unknown
  (`bl` at 0x76be0). The included return record gives `ret(0x76ca4)` in `r0` (`ldm` at 0x76be4).
  Environmental temperature setting remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x76bc0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x76bc4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x76be4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x76be4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x76be4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x76be4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x76bdc | `(unknown callee) 0x1b5f0` | `0x7` | `?` | `r2` | `?` |
| `bl` at 0x76be0 | `(unknown callee) 0x76ca4` | `ret(0x1b5f0)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x76be4 | `r0` | `ret(0x76ca4)` |

## `EnvTestTemp`

- `EnvTestTemp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x76e90 to 0x76f1c.
  Established. The absolute word at `0x132f18` receives `0x1` (`str` at 0x76eb8). The word at
  `0x132f2c` receives `[sp]` (`str` at 0x76ef4). The unnamed `0x76ca4` call carries entry `r0`,
  entry `r1`, `0x132f2c`, `[sp]` (`bl` at 0x76ef8). Temperature test setup effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x76ea0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x76ea4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x76ea8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x76eac | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x76eb0 | load | 4 | `abs 0x76f04` | `-` |
| `str` at 0x76eb8 | store | 4 | `abs 0x132f18` | `0x1` |
| `ldr` at 0x76ebc | load | 4 | `abs 0x76f08` | `-` |
| `ldr` at 0x76ec0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x76ec4 | store | 4 | `abs 0x132f1c` | `[sp-20]` |
| `ldr` at 0x76ec8 | load | 4 | `abs 0x76f0c` | `-` |
| `ldr` at 0x76ecc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x76ed0 | store | 4 | `abs 0x132f20` | `[sp-24]` |
| `ldr` at 0x76ed4 | load | 4 | `abs 0x76f10` | `-` |
| `ldr` at 0x76ed8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x76edc | store | 4 | `abs 0x132f24` | `[sp-28]` |
| `ldr` at 0x76ee0 | load | 4 | `abs 0x76f14` | `-` |
| `ldr` at 0x76ee4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x76ee8 | store | 4 | `abs 0x132f28` | `[sp-32]` |
| `ldr` at 0x76eec | load | 4 | `abs 0x76f18` | `-` |
| `ldr` at 0x76ef0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x76ef4 | store | 4 | `abs 0x132f2c` | `[sp]` |
| `ldm` at 0x76f00 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x76f00 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x76f00 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x76ef8 | `(unknown callee) 0x76ca4` | `r0` | `r1` | `0x132f2c` | `[sp]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x76f00 | `r0` | `ret(0x76ca4)` |

## `EnvAltitude`

- `EnvAltitude`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x76c7c to 0x76ca4.
  Established. The unnamed `0x1b5f0` call carries `0xc`, entry `r0`, entry `r2`, entry `r3` (`bl` at 0x76c98). The subsequent unnamed `0x76ca4` call carries `ret(0x1b5f0)`, unknown, unknown, unknown
  (`bl` at 0x76c9c). The included return record gives `ret(0x76ca4)` in `r0` (`ldm` at 0x76ca0).
  Altitude setting semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x76c8c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x76c94 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x76ca0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x76ca0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x76ca0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x76ca0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x76c98 | `(unknown callee) 0x1b5f0` | `0xc` | `r0` | `r2` | `r3` |
| `bl` at 0x76c9c | `(unknown callee) 0x76ca4` | `ret(0x1b5f0)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x76ca0 | `r0` | `ret(0x76ca4)` |

## `EnvReset`

- `EnvReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x76ca4 to 0x76d68.
  Established. A nonzero word at `0x132f10` takes `0x76cc8`, while zero falls through (`bne` at 0x76cc0). The word at `0x133208` receives an unknown value (`str` at 0x76ce8). Separate included
  assignments at `sp -0x18` write `0x6`, `0x2` and `0x1` (`str` at 0x76d34, `str` at 0x76d40, `str`
  at 0x76d4c). Environmental reset policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x76cb4 | load | 4 | `abs 0x76d5c` | `-` |
| `ldr` at 0x76cb8 | load | 4 | `abs 0x132f10` | `-` |
| `ldr` at 0x76ccc | load | 4 | `abs 0x76d60` | `-` |
| `str` at 0x76ce8 | store | 4 | `abs 0x133208` | `?` |
| `ldr` at 0x76cf0 | load | 4 | `abs 0x76d64` | `-` |
| `ldr` at 0x76cf4 | load | 4 | `abs 0x297604` | `-` |
| `ldr` at 0x76d00 | load | 4 | `abs 0x76d64` | `-` |
| `ldr` at 0x76d04 | load | 4 | `abs 0x297604` | `-` |
| `ldr` at 0x76d10 | load | 4 | `abs 0x76d60` | `-` |
| `ldr` at 0x76d14 | load | 4 | `abs 0x133208` | `-` |
| `ldr` at 0x76d20 | load | 4 | `abs 0x76d60` | `-` |
| `ldr` at 0x76d24 | load | 4 | `abs 0x133214` | `-` |
| `str` at 0x76d34 | store | 4 | `sp -0x18` | `0x6` |
| `str` at 0x76d40 | store | 4 | `sp -0x18` | `0x2` |
| `str` at 0x76d4c | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0x76d50 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x76d58 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x76d58 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x76d58 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x76d58 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x76d58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x76cc8 | `(unknown callee) 0x7056c` | `r0` | `r1` | `r2` | `[0x132f10]` |
| `bl` at 0x76cd4 | `(unknown callee) 0x1d444` | `0x6` | `?` | `?` | `?` |
| `bl` at 0x76cec | `(unknown callee) 0x7136c` | `ret(0x1d444)` | `?` | `?` | `?` |
| `mov` at 0x76d0c | `(unknown callee) (unknown target)` | `ret(0x7136c)` | `?` | `?` | `[0x297604]` |
| `bl` at 0x76d54 | `(unknown callee) 0x6aa60` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x76cc0 | `[0x132f10] != 0x0` | `[0x132f10] == 0x0` |
| `beq` at 0x76cfc | `[0x297604] == 0x0` | `[0x297604] != 0x0` |
| `bne` at 0x76d1c | `[0x133208] != 0x0` | `[0x133208] == 0x0` |
| `beq` at 0x76d2c | `[0x133214] == 0x0` | `[0x133214] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x76d58 | `r0` | `?` |

## `EnvState`

- `EnvState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x75654 to 0x75740.
  Established. Included word assignments at `0x13320c` write `0x1` and zero in separate arms (`str`
  at 0x756d0, `str` at 0x756f0). The word at `0x133210` receives `0x1` on another included path
  (`str` at 0x756fc). The included return record gives integer `0x0` in `r0` (`ldm` at 0x75730).
  Environmental state transition meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x75664 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x75668 | load | 4 | `abs 0x75734` | `-` |
| `ldr` at 0x7566c | load | 4 | `abs 0x132f14` | `-` |
| `ldr` at 0x7567c | load | 4 | `abs 0x75738` | `-` |
| `ldr` at 0x75680 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75688 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7568c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x75690 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7569c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x756a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x756b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x756c8 | load | 4 | `abs 0x7573c` | `-` |
| `str` at 0x756d0 | store | 4 | `abs 0x13320c` | `0x1` |
| `ldr` at 0x756d4 | load | 4 | `abs 0x7573c` | `-` |
| `str` at 0x756dc | store | 4 | `abs 0x133210` | `0x0` |
| `ldr` at 0x756e8 | load | 4 | `abs 0x7573c` | `-` |
| `str` at 0x756f0 | store | 4 | `abs 0x13320c` | `0x0` |
| `ldr` at 0x756f4 | load | 4 | `abs 0x7573c` | `-` |
| `str` at 0x756fc | store | 4 | `abs 0x133210` | `0x1` |
| `ldr` at 0x75708 | load | 4 | `abs 0x7573c` | `-` |
| `str` at 0x75710 | store | 4 | `abs 0x13320c` | `0x0` |
| `ldr` at 0x75718 | load | 4 | `abs 0x7573c` | `-` |
| `str` at 0x75720 | store | 4 | `abs 0x133210` | `0x0` |
| `ldm` at 0x75730 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x75730 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x75730 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x75684 | `(unknown callee) 0x221f0` | `0xfdb88` | `r0` | `r2` | `?` |
| `bl` at 0x756e0 | `(unknown callee) 0x76ca4` | `?` | `?` | `0x133208` | `0x0` |
| `bl` at 0x75700 | `(unknown callee) 0x76ca4` | `?` | `?` | `0x133208` | `0x1` |
| `bl` at 0x75714 | `(unknown callee) 0x76ca4` | `?` | `?` | `0x133208` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x75678 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x75698 | `Z == 1` | `not (Z == 1)` |
| `bgt` at 0x756a4 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x756b0 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x756c0 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x75730 | `r0` | `0x0` |

## `EnvMessages`

- `EnvMessages`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x77070 to 0x7713c.
  Established. Entry `r0 == 0x1` takes `0x770ec`, while inequality falls through (`beq` at 0x77094).
  Included word assignments at `0x132f34` write `0x1` and zero in separate arms (`str` at 0x770f4,
  `str` at 0x7710c). The included return record carries `[sp-24]` in `r0` (`ldm` at 0x77124).
  Message selection policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x77080 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x77084 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x77088 | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0x7708c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x77098 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x770a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x770b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x770c4 | load | 4 | `abs 0x77128` | `-` |
| `ldr` at 0x770c8 | load | 4 | `abs 0x132f34` | `-` |
| `ldr` at 0x770d4 | load | 4 | `abs 0x7712c` | `-` |
| `str` at 0x770d8 | store | 4 | `sp -0x18` | `0xfe338` |
| `ldr` at 0x770e0 | load | 4 | `abs 0x77130` | `-` |
| `str` at 0x770e4 | store | 4 | `sp -0x18` | `0xfe35c` |
| `ldr` at 0x770ec | load | 4 | `abs 0x77128` | `-` |
| `str` at 0x770f4 | store | 4 | `abs 0x132f34` | `0x1` |
| `ldr` at 0x770f8 | load | 4 | `abs 0x77134` | `-` |
| `str` at 0x770fc | store | 4 | `sp -0x18` | `0xfe380` |
| `ldr` at 0x77104 | load | 4 | `abs 0x77128` | `-` |
| `str` at 0x7710c | store | 4 | `abs 0x132f34` | `0x0` |
| `ldr` at 0x77110 | load | 4 | `abs 0x77138` | `-` |
| `str` at 0x77114 | store | 4 | `sp -0x18` | `0xfe3ac` |
| `ldr` at 0x7711c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x77124 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x77124 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x77124 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x77094 | `r0 == 0x1` | `r0 != 0x1` |
| `bgt` at 0x770a0 | `r0 > 0x1 (signed, 32-bit)` | `r0 <= 0x1 (signed, 32-bit)` |
| `beq` at 0x770ac | `r0 == 0x0` | `r0 != 0x0` |
| `beq` at 0x770bc | `r0 == 0x2` | `r0 != 0x2` |
| `beq` at 0x770d0 | `[0x132f34] == 0x0` | `[0x132f34] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x77124 | `r0` | `[sp-24]` |
