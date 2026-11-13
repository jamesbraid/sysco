# ARM board and bus reset paths in 1.44.0

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
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/board-bus-reset-facts.txt) retain all tool sections.

## `pcixReset`

- `pcixReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x50004 to 0x5008c.
  Established. The unnamed `0x50a94` call carries entry `r0`, `(r1 & 0xff)`, `(r2 & 0xff)`, `(r1 & 0xff)` (`bl` at 0x50038).
  A zero result takes `0x5005c`, while nonzero falls through (`beq` at 0x5004c).
  The word at `sp -0x20` receives `ret(0x50d60)` on an included path (`str` at 0x5007c).
  PCI-X reset sequencing remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x50014 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x5001c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x50024 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0x50028 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x5002c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x50030 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x50040 | store | 4 | `sp -0x1c` | `ret(0x50a94)` |
| `ldr` at 0x50044 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x50050 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x50054 | store | 4 | `sp -0x20` | `ret(0x50a94)` |
| `ldrb` at 0x50064 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x50068 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x5006c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5007c | store | 4 | `sp -0x20` | `ret(0x50d60)` |
| `ldr` at 0x50080 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x50088 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x50088 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x50088 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x50038 | `(unknown callee) 0x50a94` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `(r1 & 0xff)` |
| `bl` at 0x50060 | `(unknown callee) 0x5008c` | `0x2` | `?` | `?` | `ret(0x50a94)` |
| `bl` at 0x50074 | `(unknown callee) 0x50d60` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5004c | `ret(0x50a94) == 0x0` | `ret(0x50a94) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x50088 | `r0` | `?` |

## `pcixBusInReset`

- `pcixBusInReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x506f0 to 0x508a4.
  Established. A nonzero `[0x1324a0]` takes `0x50728`, while zero falls through (`bne` at 0x50718).
  Included word stores assign `0xfffff703` and `0xfffff702` to `sp -0x38` (`str` at 0x50720, `str` at 0x507c8).
  The unnamed `0x53658` call carries `[sp-20]`, `[sp-36]`, `sp-44`, `sp-44` (`bl` at 0x507e0).
  Bus reset-state interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x50700 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x50708 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x5070c | load | 4 | `abs 0x50894` | `-` |
| `ldr` at 0x50710 | load | 4 | `abs 0x1324a0` | `-` |
| `ldr` at 0x5071c | load | 4 | `abs 0x50898` | `-` |
| `str` at 0x50720 | store | 4 | `sp -0x38` | `0xfffff703` |
| `ldr` at 0x50728 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5073c | load | 4 | `abs 0x5089c` | `-` |
| `ldr` at 0x50740 | load | 4 | `abs 0x13249c` | `-` |
| `str` at 0x50748 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x5074c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x50750 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x50760 | store | 4 | `sp -0x20` | `ret(0x53024)` |
| `ldr` at 0x50764 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x50770 | load | 4 | `abs 0x50898` | `-` |
| `str` at 0x50774 | store | 4 | `sp -0x38` | `0xfffff703` |
| `str` at 0x50780 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x50784 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x50788 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5078c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x50798 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5079c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x507a0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x507a4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x507b0 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0x507b8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x507c4 | load | 4 | `abs 0x508a0` | `-` |
| `str` at 0x507c8 | store | 4 | `sp -0x38` | `0xfffff702` |
| `ldr` at 0x507d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x507d8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x507e8 | store | 4 | `sp -0x30` | `ret(0x53658)` |
| `ldr` at 0x507ec | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x507f8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x507fc | store | 4 | `sp -0x38` | `ret(0x53658)` |
| `ldrb` at 0x50810 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x50830 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x50850 | load | 1 | `(unknown)` | `-` |
| `str` at 0x50868 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x50870 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x50878 | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `str` at 0x50884 | store | 4 | `sp -0x38` | `0x1` |
| `ldr` at 0x50888 | load | 4 | `sp -0x38` | `-` |
| `ldm` at 0x50890 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x50890 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x50890 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x50758 | `(unknown callee) 0x53024` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x507e0 | `(unknown callee) 0x53658` | `[sp-20]` | `[sp-36]` | `sp-44` | `sp-44` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x50718 | `[0x1324a0] != 0x0` | `[0x1324a0] == 0x0` |
| `bne` at 0x5076c | `ret(0x53024) != 0x0` | `ret(0x53024) == 0x0` |
| `ble` at 0x50794 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x507c0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x507f4 | `ret(0x53658) == 0x0` | `ret(0x53658) != 0x0` |
| `beq` at 0x50820 | `? == 0x3` | `? != 0x3` |
| `bne` at 0x50840 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x50860 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x50890 | `r0` | `?` |

## `pcixResetHold`

- `pcixResetHold`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x50a94 to 0x50d60.
  Established. The unnamed `0x524e8` call at `0x50d3c` carries `[sp-20]`, `ret(0x53100)`, `0x0`, `ret(0x53100)` (`bl` at 0x50d3c).
  A nonzero `[0x1324a0]` takes `0x50afc`, while zero falls through (`bne` at 0x50acc).
  An included byte store has unknown target and value (`strb` at 0x50bf8).
  PCI-X hold descriptor policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x50aa4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x50aac | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x50ab4 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x50abc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x50ac0 | load | 4 | `abs 0x50d54` | `-` |
| `ldr` at 0x50ac4 | load | 4 | `abs 0x1324a0` | `-` |
| `ldrb` at 0x50ad0 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x50ae0 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x50ae8 | load | 4 | `abs 0x50d58` | `-` |
| `str` at 0x50aec | store | 4 | `sp -0x38` | `0xfffff703` |
| `ldr` at 0x50af0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x50af4 | store | 4 | `sp -0x34` | `[sp-56]` |
| `ldr` at 0x50afc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50b10 | load | 4 | `abs 0x50d5c` | `-` |
| `ldr` at 0x50b14 | load | 4 | `abs 0x13249c` | `-` |
| `str` at 0x50b1c | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x50b20 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x50b30 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x50b34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x50b38 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x50b3c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x50b48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x50b4c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x50b54 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x50b5c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x50b70 | store | 4 | `sp -0x2c` | `ret(0x524e8)` |
| `ldr` at 0x50b74 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x50b80 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x50b8c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x50b90 | store | 4 | `sp -0x28` | `ret(0x524e8)` |
| `ldr` at 0x50b94 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x50b9c | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `str` at 0x50ba8 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x50bac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x50bb0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x50bb4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x50bc0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x50bc4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x50bcc | load | 4 | `[sp-28] +0x14` | `-` |
| `ldr` at 0x50bd4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x50bd8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x50bec | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x50bf4 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x50bf8 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x50bfc | load | 4 | `sp -0x30` | `-` |
| `str` at 0x50c04 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x50c0c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x50c10 | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x50c18 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x50c24 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x50c28 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x50c38 | store | 4 | `sp -0x20` | `ret(0x53024)` |
| `ldr` at 0x50c3c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x50c48 | load | 4 | `abs 0x50d58` | `-` |
| `str` at 0x50c4c | store | 4 | `sp -0x34` | `0xfffff703` |
| `ldr` at 0x50c54 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x50c58 | load | 1 | `ret(0x53024) +0x1` | `-` |
| `str` at 0x50c5c | store | 4 | `sp -0x30` | `[ret(0x53024)+1]:1` |
| `ldr` at 0x50c60 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x50c64 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x50c68 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x50c6c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x50c74 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x50c80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x50c84 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x50c8c | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x50c94 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x50ca8 | store | 4 | `sp -0x2c` | `ret(0x524e8)` |
| `ldr` at 0x50cac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x50cb8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x50cc4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x50cc8 | store | 4 | `sp -0x28` | `ret(0x524e8)` |
| `ldr` at 0x50ccc | load | 4 | `sp -0x30` | `-` |
| `str` at 0x50cd4 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x50cdc | load | 4 | `sp -0x28` | `-` |
| `str` at 0x50ce0 | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x50ce8 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x50cec | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x50cf0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x50d00 | store | 4 | `sp -0x24` | `ret(0x53100)` |
| `ldr` at 0x50d04 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x50d10 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x50d14 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x50d18 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x50d28 | store | 4 | `sp -0x34` | `ret(0x532e0)` |
| `ldr` at 0x50d30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50d34 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x50d44 | store | 4 | `sp -0x34` | `ret(0x524e8)` |
| `ldr` at 0x50d48 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x50d50 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x50d50 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x50d50 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x50b68 | `(unknown callee) 0x524e8` | `[sp-20]` | `?` | `0x1` | `?` |
| `bl` at 0x50c30 | `(unknown callee) 0x53024` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x50ca0 | `(unknown callee) 0x524e8` | `[sp-20]` | `?` | `0x1` | `?` |
| `bl` at 0x50cf8 | `(unknown callee) 0x53100` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x50d20 | `(unknown callee) 0x532e0` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x50d3c | `(unknown callee) 0x524e8` | `[sp-20]` | `ret(0x53100)` | `0x0` | `ret(0x53100)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x50acc | `[0x1324a0] != 0x0` | `[0x1324a0] == 0x0` |
| `bne` at 0x50ad8 | `(r1 & 0xff) != 0xff` | `(r1 & 0xff) == 0xff` |
| `bne` at 0x50b28 | `[sp-21]:1 != 0xff` | `[sp-21]:1 == 0xff` |
| `ble` at 0x50b44 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x50b7c | `ret(0x524e8) == 0x0` | `ret(0x524e8) != 0x0` |
| `bne` at 0x50b88 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `ble` at 0x50bbc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x50c20 | `[sp-22]:1 != 0xff` | `[sp-22]:1 == 0xff` |
| `bne` at 0x50c44 | `ret(0x53024) != 0x0` | `ret(0x53024) == 0x0` |
| `ble` at 0x50c7c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x50cb4 | `ret(0x524e8) == 0x0` | `ret(0x524e8) != 0x0` |
| `bne` at 0x50cc0 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0x50d0c | `ret(0x53100) != 0x0` | `ret(0x53100) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x50d50 | `r0` | `?` |

## `pcixResetRelease`

- `pcixResetRelease`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x50d60 to 0x50f68.
  Established. The word through `[sp-28] +0x8` is read (`ldr` at 0x50e04).
  A nonzero `ret(0x53100)` takes `0x50f3c`, while zero falls through (`bne` at 0x50f18).
  The unnamed `0x52ac4` call carries `[sp-20]`, `ret(0x53100)`, unknown, `ret(0x53100)` (`bl` at 0x50f44).
  PCI-X release callback effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x50d70 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x50d78 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x50d80 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x50d88 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x50d8c | load | 4 | `abs 0x50f5c` | `-` |
| `ldr` at 0x50d90 | load | 4 | `abs 0x1324a0` | `-` |
| `ldrb` at 0x50d9c | load | 1 | `sp -0x15` | `-` |
| `str` at 0x50dac | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x50db4 | load | 4 | `abs 0x50f60` | `-` |
| `str` at 0x50db8 | store | 4 | `sp -0x38` | `0xfffff703` |
| `ldr` at 0x50dbc | load | 4 | `sp -0x38` | `-` |
| `str` at 0x50dc0 | store | 4 | `sp -0x34` | `[sp-56]` |
| `ldr` at 0x50dc8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50ddc | load | 4 | `abs 0x50f64` | `-` |
| `ldr` at 0x50de0 | load | 4 | `abs 0x13249c` | `-` |
| `str` at 0x50de8 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x50dec | load | 1 | `sp -0x15` | `-` |
| `str` at 0x50dfc | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x50e00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x50e04 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x50e08 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x50e14 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x50e18 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x50e2c | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x50e34 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x50e44 | store | 4 | `sp -0x2c` | `ret(0x52910)` |
| `ldr` at 0x50e48 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x50e54 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x50e60 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x50e64 | store | 4 | `sp -0x28` | `ret(0x52910)` |
| `ldr` at 0x50e68 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x50e70 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x50e78 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x50e7c | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x50e84 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x50e90 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x50e94 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x50ea4 | store | 4 | `sp -0x24` | `ret(0x53024)` |
| `ldr` at 0x50ea8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x50eb4 | load | 4 | `abs 0x50f60` | `-` |
| `str` at 0x50eb8 | store | 4 | `sp -0x34` | `0xfffff703` |
| `ldr` at 0x50ec0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50ec4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x50ed0 | store | 4 | `sp -0x2c` | `ret(0x52910)` |
| `ldr` at 0x50ed4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x50ee0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x50ee4 | store | 4 | `sp -0x28` | `ret(0x52910)` |
| `ldr` at 0x50ee8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x50eec | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x50ef4 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x50ef8 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x50efc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x50f0c | store | 4 | `sp -0x20` | `ret(0x53100)` |
| `ldr` at 0x50f10 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x50f1c | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x50f20 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x50f24 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x50f34 | store | 4 | `sp -0x34` | `ret(0x532e0)` |
| `ldr` at 0x50f3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50f40 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x50f4c | store | 4 | `sp -0x34` | `ret(0x52ac4)` |
| `ldr` at 0x50f50 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x50f58 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x50f58 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x50f58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x50e3c | `(unknown callee) 0x52910` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x50e9c | `(unknown callee) 0x53024` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x50ec8 | `(unknown callee) 0x52910` | `[sp-20]` | `ret(0x53024)` | `?` | `ret(0x53024)` |
| `bl` at 0x50f04 | `(unknown callee) 0x53100` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x50f2c | `(unknown callee) 0x532e0` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x50f44 | `(unknown callee) 0x52ac4` | `[sp-20]` | `ret(0x53100)` | `?` | `ret(0x53100)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x50d98 | `[0x1324a0] != 0x0` | `[0x1324a0] == 0x0` |
| `bne` at 0x50da4 | `(r1 & 0xff) != 0xff` | `(r1 & 0xff) == 0xff` |
| `bne` at 0x50df4 | `[sp-21]:1 != 0xff` | `[sp-21]:1 == 0xff` |
| `ble` at 0x50e10 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x50e50 | `ret(0x52910) == 0x0` | `ret(0x52910) != 0x0` |
| `bne` at 0x50e5c | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0x50e8c | `[sp-22]:1 != 0xff` | `[sp-22]:1 == 0xff` |
| `bne` at 0x50eb0 | `ret(0x53024) != 0x0` | `ret(0x53024) == 0x0` |
| `beq` at 0x50edc | `ret(0x52910) == 0x0` | `ret(0x52910) != 0x0` |
| `bne` at 0x50f18 | `ret(0x53100) != 0x0` | `ret(0x53100) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x50f58 | `r0` | `?` |

## `TIOResetHold`

- `TIOResetHold`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4b56c to 0x4b778.
  Established. An included byte store clears absolute `0x132494` (`strb` at 0x4b5ac).
  The unnamed `0xca584` call carries `0x0`, `[[sp-24]]:1`, `0x60`, `0x0` (`bl` at 0x4b69c).
  A zero result takes `0x4b6c8`, while nonzero falls through (`beq` at 0x4b6b0).
  TIO hold-time output effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4b57c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4b584 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x4b588 | load | 4 | `abs 0x4b764` | `-` |
| `ldr` at 0x4b58c | load | 4 | `abs 0x132490` | `-` |
| `ldr` at 0x4b598 | load | 4 | `abs 0x4b768` | `-` |
| `str` at 0x4b59c | store | 4 | `sp -0x2c` | `0xfffff768` |
| `ldr` at 0x4b5a4 | load | 4 | `abs 0x4b76c` | `-` |
| `strb` at 0x4b5ac | store | 1 | `abs 0x132494` | `0x0` |
| `str` at 0x4b5b4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x4b5b8 | load | 4 | `abs 0x4b770` | `-` |
| `ldrb` at 0x4b5bc | load | 1 | `abs 0x299842` | `-` |
| `ldr` at 0x4b5c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4b5cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4b5d4 | load | 4 | `abs 0x4b774` | `-` |
| `ldr` at 0x4b5d8 | load | 4 | `abs 0x132498` | `-` |
| `ldrb` at 0x4b5e0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x4b5ec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4b600 | load | 4 | `abs 0x4b764` | `-` |
| `ldr` at 0x4b604 | load | 4 | `abs 0x132490` | `-` |
| `str` at 0x4b60c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x4b610 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4b614 | load | 1 | `(unknown)` | `-` |
| `str` at 0x4b62c | store | 4 | `sp -0x20` | `ret(0xca878)` |
| `ldr` at 0x4b630 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4b63c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4b648 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4b64c | store | 4 | `sp -0x1c` | `ret(0xca878)` |
| `ldr` at 0x4b650 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4b660 | store | 4 | `sp -0x20` | `ret(0x4e03c)` |
| `ldr` at 0x4b664 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4b670 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4b67c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4b680 | store | 4 | `sp -0x1c` | `ret(0x4e03c)` |
| `ldr` at 0x4b684 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4b688 | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0x4b6a4 | store | 4 | `sp -0x20` | `ret(0xca584)` |
| `ldr` at 0x4b6a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4b6b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4b6c0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4b6c4 | store | 4 | `sp -0x1c` | `ret(0xca584)` |
| `ldr` at 0x4b6c8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4b6cc | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0x4b6e4 | store | 4 | `sp -0x20` | `ret(0xca538)` |
| `ldr` at 0x4b6e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4b6f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4b700 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4b704 | store | 4 | `sp -0x1c` | `ret(0xca538)` |
| `ldr` at 0x4b708 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x4b710 | store | 4 | `sp -0x24` | `?` |
| `str` at 0x4b72c | store | 4 | `sp -0x20` | `ret(0x50a94)` |
| `ldr` at 0x4b730 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4b73c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4b748 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4b74c | store | 4 | `sp -0x1c` | `ret(0x50a94)` |
| `ldr` at 0x4b750 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4b754 | store | 4 | `sp -0x2c` | `[sp-28]` |
| `ldr` at 0x4b758 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x4b760 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4b760 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4b760 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4b624 | `(unknown callee) 0xca878` | `0x0` | `?` | `0x3` | `?` |
| `bl` at 0x4b658 | `(unknown callee) 0x4e03c` | `[sp-24]` | `0x0` | `?` | `?` |
| `bl` at 0x4b69c | `(unknown callee) 0xca584` | `0x0` | `[[sp-24]]:1` | `0x60` | `0x0` |
| `bl` at 0x4b6dc | `(unknown callee) 0xca538` | `0x0` | `[[sp-24]]:1` | `0x4` | `[[sp-24]]:1` |
| `bl` at 0x4b724 | `(unknown callee) 0x50a94` | `0x0` | `0xff` | `0xff` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x4b594 | `[0x132490] != 0x0` | `[0x132490] == 0x0` |
| `ble` at 0x4b5c8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x4b5e8 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x4b638 | `ret(0xca878) == 0x0` | `ret(0xca878) != 0x0` |
| `bne` at 0x4b644 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x4b66c | `ret(0x4e03c) == 0x0` | `ret(0x4e03c) != 0x0` |
| `bne` at 0x4b678 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x4b6b0 | `ret(0xca584) == 0x0` | `ret(0xca584) != 0x0` |
| `bne` at 0x4b6bc | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x4b6f0 | `ret(0xca538) == 0x0` | `ret(0xca538) != 0x0` |
| `bne` at 0x4b6fc | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x4b738 | `ret(0x50a94) == 0x0` | `ret(0x50a94) != 0x0` |
| `bne` at 0x4b744 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4b760 | `r0` | `?` |

## `TIOResetRelease`

- `TIOResetRelease`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4b778 to 0x4ba54.
  Established. The absolute byte at `0x132494` receives `0x1` (`strb` at 0x4b984).
  The unnamed `0x50d60` call carries `0x0`, `0xff`, `0xff`, `ret(0x51558)` (`bl` at 0x4b958);
  its zero result takes `0x4b97c`, while nonzero falls through (`beq` at 0x4b96c).
  TIO release-time board policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4b788 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x4b78c | load | 4 | `abs 0x4ba40` | `-` |
| `ldr` at 0x4b790 | load | 4 | `abs 0x132490` | `-` |
| `ldr` at 0x4b79c | load | 4 | `abs 0x4ba44` | `-` |
| `str` at 0x4b7a0 | store | 4 | `sp -0x20` | `0xfffff768` |
| `str` at 0x4b7bc | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x4b7d0 | store | 4 | `sp -0x18` | `ret(0x51440)` |
| `ldr` at 0x4b7d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4b7e0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4b7e4 | store | 4 | `sp -0x20` | `ret(0x51440)` |
| `str` at 0x4b7fc | store | 4 | `sp -0x18` | `ret(0x51558)` |
| `ldr` at 0x4b800 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4b80c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4b810 | store | 4 | `sp -0x20` | `ret(0x51558)` |
| `str` at 0x4b81c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x4b820 | load | 4 | `abs 0x4ba48` | `-` |
| `ldrb` at 0x4b824 | load | 1 | `abs 0x299842` | `-` |
| `ldr` at 0x4b828 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4b834 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4b83c | load | 4 | `abs 0x4ba4c` | `-` |
| `ldr` at 0x4b840 | load | 4 | `abs 0x132498` | `-` |
| `ldrb` at 0x4b848 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x4b858 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4b86c | store | 4 | `sp -0x18` | `ret(0x4c75c)` |
| `ldr` at 0x4b874 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4b880 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4b884 | store | 4 | `sp -0x20` | `[sp-24]` |
| `ldr` at 0x4b88c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4b894 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x4b8a0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x4b8a4 | load | 4 | `abs 0x4ba48` | `-` |
| `ldrb` at 0x4b8a8 | load | 1 | `abs 0x299842` | `-` |
| `ldr` at 0x4b8ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4b8b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4b8c0 | load | 4 | `abs 0x4ba4c` | `-` |
| `ldr` at 0x4b8c4 | load | 4 | `abs 0x132498` | `-` |
| `ldrb` at 0x4b8cc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x4b8dc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4b8f0 | store | 4 | `sp -0x18` | `ret(0x4d3f4)` |
| `ldr` at 0x4b8f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4b904 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4b908 | store | 4 | `sp -0x20` | `[sp-24]` |
| `ldr` at 0x4b910 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4b918 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x4b930 | store | 4 | `sp -0x18` | `ret(0x51558)` |
| `ldr` at 0x4b934 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4b940 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4b944 | store | 4 | `sp -0x20` | `ret(0x51558)` |
| `str` at 0x4b960 | store | 4 | `sp -0x18` | `ret(0x50d60)` |
| `ldr` at 0x4b964 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4b970 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4b974 | store | 4 | `sp -0x20` | `ret(0x50d60)` |
| `ldr` at 0x4b97c | load | 4 | `abs 0x4ba50` | `-` |
| `strb` at 0x4b984 | store | 1 | `abs 0x132494` | `0x1` |
| `str` at 0x4b98c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x4b990 | load | 4 | `abs 0x4ba48` | `-` |
| `ldrb` at 0x4b994 | load | 1 | `abs 0x299842` | `-` |
| `ldr` at 0x4b998 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4b9a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4b9ac | load | 4 | `abs 0x4ba4c` | `-` |
| `ldr` at 0x4b9b0 | load | 4 | `abs 0x132498` | `-` |
| `ldrb` at 0x4b9b8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x4b9c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4b9d8 | load | 4 | `abs 0x4ba40` | `-` |
| `ldr` at 0x4b9dc | load | 4 | `abs 0x132490` | `-` |
| `ldrb` at 0x4b9e4 | load | 1 | `(unknown)` | `-` |
| `str` at 0x4ba00 | store | 4 | `sp -0x18` | `ret(0xca910)` |
| `ldr` at 0x4ba04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4ba10 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4ba14 | store | 4 | `sp -0x20` | `ret(0xca910)` |
| `ldr` at 0x4ba1c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4ba24 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x4ba30 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x4ba34 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x4ba3c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4ba3c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4ba3c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4b7a8 | `(unknown callee) 0x375e4` | `r0` | `r1` | `r2` | `[0x132490]` |
| `bl` at 0x4b7c8 | `(unknown callee) 0x51440` | `0x0` | `?` | `?` | `ret(0x375e4)` |
| `bl` at 0x4b7f4 | `(unknown callee) 0x51558` | `0x0` | `0x0` | `?` | `ret(0x51440)` |
| `bl` at 0x4b854 | `(unknown callee) 0xd708c` | `?` | `?` | `?` | `?` |
| `bl` at 0x4b864 | `(unknown callee) 0x4c75c` | `?` | `?` | `?` | `?` |
| `bl` at 0x4b870 | `(unknown callee) 0xd70fc` | `ret(0x4c75c)` | `?` | `?` | `ret(0x4c75c)` |
| `bl` at 0x4b8d8 | `(unknown callee) 0xd708c` | `?` | `?` | `?` | `?` |
| `bl` at 0x4b8e8 | `(unknown callee) 0x4d3f4` | `?` | `?` | `?` | `?` |
| `bl` at 0x4b8f4 | `(unknown callee) 0xd70fc` | `ret(0x4d3f4)` | `?` | `?` | `ret(0x4d3f4)` |
| `bl` at 0x4b928 | `(unknown callee) 0x51558` | `0x0` | `0x1` | `[0x299842]:1` | `?` |
| `bl` at 0x4b958 | `(unknown callee) 0x50d60` | `0x0` | `0xff` | `0xff` | `ret(0x51558)` |
| `bl` at 0x4b9f8 | `(unknown callee) 0xca910` | `0x0` | `?` | `0x2` | `0x2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x4b798 | `[0x132490] != 0x0` | `[0x132490] == 0x0` |
| `bne` at 0x4b7b4 | `ret(0x375e4) != 0x0` | `ret(0x375e4) == 0x0` |
| `beq` at 0x4b7dc | `ret(0x51440) == 0x0` | `ret(0x51440) != 0x0` |
| `beq` at 0x4b808 | `ret(0x51558) == 0x0` | `ret(0x51558) != 0x0` |
| `ble` at 0x4b830 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x4b850 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x4b87c | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `ble` at 0x4b8b4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x4b8d4 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x4b900 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0x4b93c | `ret(0x51558) == 0x0` | `ret(0x51558) != 0x0` |
| `beq` at 0x4b96c | `ret(0x50d60) == 0x0` | `ret(0x50d60) != 0x0` |
| `ble` at 0x4b9a0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x4b9c0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x4ba0c | `ret(0xca910) == 0x0` | `ret(0xca910) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4ba3c | `r0` | `?` |

## `TIOPciResetInt`

- `TIOPciResetInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4bcfc to 0x4be1c.
  Established. Entry `r1`, `r2`, and `r3` are narrowed into bytes at `sp -0x15`, `sp -0x16`, and `sp -0x17` (`strb` at 0x4bd18, `strb` at 0x4bd20, `strb` at 0x4bd28).
  The byte at absolute `0x299842` is read (`ldrb` at 0x4bd38).
  The unnamed `0x4e0d0` call at `0x4bddc` has four unknown candidates (`bl` at 0x4bddc).
  PCI reset interrupt routing remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4bd0c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x4bd18 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x4bd20 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x4bd28 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `str` at 0x4bd30 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x4bd34 | load | 4 | `abs 0x4be08` | `-` |
| `ldrb` at 0x4bd38 | load | 1 | `abs 0x299842` | `-` |
| `ldr` at 0x4bd3c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4bd48 | load | 4 | `abs 0x4be0c` | `-` |
| `str` at 0x4bd4c | store | 4 | `sp -0x28` | `0xf83f4` |
| `ldrb` at 0x4bd50 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x4bd54 | store | 4 | `sp -0x24` | `(r1 & 0xff)` |
| `ldr` at 0x4bd5c | load | 4 | `abs 0x4be10` | `-` |
| `ldr` at 0x4bd60 | load | 4 | `abs 0x4be14` | `-` |
| `ldr` at 0x4bd74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4bd90 | load | 4 | `abs 0x4be18` | `-` |
| `ldr` at 0x4bd94 | load | 4 | `abs 0x132490` | `-` |
| `str` at 0x4bda0 | store | 4 | `sp -0x20` | `?` |
| `str` at 0x4bda4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x4bda8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4bdac | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x4bdb0 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x4bdc0 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x4bdd0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4bdd4 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x4bde0 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x4bdf0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4bdf4 | load | 1 | `(unknown)` | `-` |
| `ldm` at 0x4be04 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4be04 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4be04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4bd68 | `(unknown callee) 0x200a0` | `0x1` | `0xf8308` | `0x265` | `0x0` |
| `bl` at 0x4bddc | `(unknown callee) 0x4e0d0` | `?` | `?` | `?` | `?` |
| `bl` at 0x4bdfc | `(unknown callee) 0x4e0d0` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0x4bd44 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x4bdb8 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x4bdcc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x4bdec | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4be04 | `r0` | `?` |

## `IP57_ResetHold`

- `IP57_ResetHold`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x56e04 to 0x570a8.
  Established. The included indirect call carries unknown, unknown, `[0x12da68]`, unknown (`mov` at 0x56e9c).
  The unnamed `0xca584` call at `0x56f34` carries `0x0`, `[sp-29]:1`, `0xc`, `0x8` (`bl` at 0x56f34);
  its zero result takes `0x56f58`, while nonzero falls through (`beq` at 0x56f48).
  Board hold output mapping remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x56e18 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x56e1c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x56e34 | store | 4 | `sp -0x1c` | `ret(0x57964)` |
| `ldr` at 0x56e38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x56e44 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x56e48 | store | 4 | `sp -0x24` | `ret(0x57964)` |
| `ldr` at 0x56e50 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x56e54 | load | 1 | `[sp-24] +0x2` | `-` |
| `strb` at 0x56e5c | store | 1 | `sp -0x1d` | `([[sp-24]+2]:1+2 & 0xff)` |
| `ldr` at 0x56e60 | load | 4 | `abs 0x570a4` | `-` |
| `ldr` at 0x56e64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x56e68 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x56e7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x56e80 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x56e8c | load | 4 | `abs 0x12da68` | `-` |
| `ldr` at 0x56ee8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x56eec | store | 4 | `sp -0x24` | `[sp-28]` |
| `ldrb` at 0x56ef4 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x56f04 | store | 4 | `sp -0x1c` | `ret(0x46d3c)` |
| `ldr` at 0x56f08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x56f14 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x56f18 | store | 4 | `sp -0x24` | `ret(0x46d3c)` |
| `ldrb` at 0x56f20 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x56f3c | store | 4 | `sp -0x1c` | `ret(0xca584)` |
| `ldr` at 0x56f40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x56f4c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x56f50 | store | 4 | `sp -0x24` | `ret(0xca584)` |
| `ldrb` at 0x56f58 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x56f70 | store | 4 | `sp -0x1c` | `ret(0xca4e8)` |
| `ldr` at 0x56f74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x56f80 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x56f84 | store | 4 | `sp -0x24` | `ret(0xca4e8)` |
| `strb` at 0x56f90 | store | 1 | `sp -0x1e` | `0x0` |
| `ldrb` at 0x56f94 | load | 1 | `sp -0x1e` | `-` |
| `ldrb` at 0x56fa0 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x56fac | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x56fcc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x56fd0 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0x56fd4 | load | 1 | `sp -0x1e` | `-` |
| `str` at 0x56ff8 | store | 4 | `sp -0x1c` | `ret(0xca538)` |
| `ldr` at 0x56ffc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57008 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5700c | store | 4 | `sp -0x24` | `ret(0xca538)` |
| `ldrb` at 0x57014 | load | 1 | `sp -0x1e` | `-` |
| `strb` at 0x5701c | store | 1 | `sp -0x1e` | `([sp-30]:1+1 & 0xff)` |
| `ldrb` at 0x57024 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x57040 | store | 4 | `sp -0x1c` | `ret(0xca584)` |
| `ldr` at 0x57044 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57050 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x57054 | store | 4 | `sp -0x24` | `ret(0xca584)` |
| `ldrb` at 0x5705c | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x57074 | store | 4 | `sp -0x1c` | `ret(0xca538)` |
| `ldr` at 0x57078 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57084 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x57088 | store | 4 | `sp -0x24` | `ret(0xca538)` |
| `str` at 0x57094 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x57098 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x570a0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x570a0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x570a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x56e2c | `(unknown callee) 0x57964` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `mov` at 0x56e9c | `(unknown callee) (unknown target)` | `?` | `?` | `[0x12da68]` | `?` |
| `bl` at 0x56ea4 | `(unknown callee) 0x47630` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x56eac | `(unknown callee) 0x570a8` | `0xa` | `?` | `?` | `?` |
| `bl` at 0x56eb0 | `(unknown callee) 0x375e4` | `ret(0x570a8)` | `?` | `?` | `?` |
| `bl` at 0x56ec0 | `(unknown callee) 0x488cc` | `ret(0x375e4)` | `?` | `?` | `ret(0x375e4)` |
| `bl` at 0x56ed8 | `(unknown callee) 0xd9010` | `?` | `sp-28` | `sp-28` | `?` |
| `bl` at 0x56efc | `(unknown callee) 0x46d3c` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x56f34 | `(unknown callee) 0xca584` | `0x0` | `[sp-29]:1` | `0xc` | `0x8` |
| `bl` at 0x56f68 | `(unknown callee) 0xca4e8` | `0x0` | `[sp-29]:1` | `0x10` | `[sp-29]:1` |
| `bl` at 0x56fbc | `(unknown callee) 0x542cc` | `?` | `?` | `?` | `?` |
| `bl` at 0x56ff0 | `(unknown callee) 0xca538` | `0x0` | `?` | `0x4` | `?` |
| `bl` at 0x57038 | `(unknown callee) 0xca584` | `0x0` | `[sp-29]:1` | `0xc0` | `0x0` |
| `bl` at 0x5706c | `(unknown callee) 0xca538` | `0x0` | `[sp-29]:1` | `0x2` | `[sp-29]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x56e40 | `ret(0x57964) == 0x0` | `ret(0x57964) != 0x0` |
| `beq` at 0x56ebc | `ret(0x375e4) == 0x0` | `ret(0x375e4) != 0x0` |
| `bne` at 0x56ee4 | `ret(0xd9010) != 0x0` | `ret(0xd9010) == 0x0` |
| `beq` at 0x56f10 | `ret(0x46d3c) == 0x0` | `ret(0x46d3c) != 0x0` |
| `beq` at 0x56f48 | `ret(0xca584) == 0x0` | `ret(0xca584) != 0x0` |
| `beq` at 0x56f7c | `ret(0xca4e8) == 0x0` | `ret(0xca4e8) != 0x0` |
| `bhi` at 0x56f9c | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0x56fc8 | `ret(0x542cc) == 0x0` | `ret(0x542cc) != 0x0` |
| `beq` at 0x57004 | `ret(0xca538) == 0x0` | `ret(0xca538) != 0x0` |
| `beq` at 0x5704c | `ret(0xca584) == 0x0` | `ret(0xca584) != 0x0` |
| `beq` at 0x57080 | `ret(0xca538) == 0x0` | `ret(0xca538) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x570a0 | `r0` | `?` |

## `IP57_ResetRelease`

- `IP57_ResetRelease`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x570d0 to 0x574d0.
  Established. The unnamed `0xca4e8` call at `0x571a8` carries `0x0`, `[sp-29]:1`, `0x2`, `[sp-29]:1` (`bl` at 0x571a8);
  its zero result takes `0x571cc`, while nonzero falls through (`beq` at 0x571bc).
  The word at absolute `0x12d6fc` is read (`ldr` at 0x573a0).
  Board release timing units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x570e4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x570e8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x57100 | store | 4 | `sp -0x1c` | `ret(0x57964)` |
| `ldr` at 0x57104 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57110 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x57114 | store | 4 | `sp -0x78` | `ret(0x57964)` |
| `ldr` at 0x5711c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57120 | load | 1 | `[sp-24] +0x2` | `-` |
| `strb` at 0x57128 | store | 1 | `sp -0x1d` | `([[sp-24]+2]:1+2 & 0xff)` |
| `ldr` at 0x5712c | load | 4 | `abs 0x574b0` | `-` |
| `ldr` at 0x57130 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57134 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x57148 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5714c | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x57158 | load | 4 | `abs 0x12da68` | `-` |
| `ldrb` at 0x5716c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5717c | store | 4 | `sp -0x1c` | `ret(0x46d60)` |
| `ldr` at 0x57180 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5718c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x57190 | store | 4 | `sp -0x78` | `ret(0x46d60)` |
| `ldrb` at 0x57198 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x571b0 | store | 4 | `sp -0x1c` | `ret(0xca4e8)` |
| `ldr` at 0x571b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x571c0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x571c4 | store | 4 | `sp -0x78` | `ret(0xca4e8)` |
| `ldrb` at 0x571d4 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x571ec | store | 4 | `sp -0x1c` | `ret(0xca538)` |
| `ldr` at 0x571f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x571fc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x57200 | store | 4 | `sp -0x78` | `ret(0xca538)` |
| `ldrb` at 0x57210 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x57228 | store | 4 | `sp -0x1c` | `ret(0xca4e8)` |
| `ldr` at 0x5722c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57238 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5723c | store | 4 | `sp -0x78` | `ret(0xca4e8)` |
| `ldrb` at 0x5724c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5725c | store | 4 | `sp -0x1c` | `ret(0x468c0)` |
| `ldr` at 0x57260 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5726c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x57270 | store | 4 | `sp -0x78` | `ret(0x468c0)` |
| `ldrb` at 0x5728c | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x572a4 | store | 4 | `sp -0x1c` | `ret(0xca4e8)` |
| `ldr` at 0x572a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x572b4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x572b8 | store | 4 | `sp -0x78` | `ret(0xca4e8)` |
| `strb` at 0x572cc | store | 1 | `sp -0x1e` | `0x0` |
| `ldrb` at 0x572d0 | load | 1 | `sp -0x1e` | `-` |
| `ldrb` at 0x572dc | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x572e8 | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x57308 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5730c | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0x57310 | load | 1 | `sp -0x1e` | `-` |
| `str` at 0x57334 | store | 4 | `sp -0x1c` | `ret(0xca4e8)` |
| `ldr` at 0x57338 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57344 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x57348 | store | 4 | `sp -0x78` | `ret(0xca4e8)` |
| `ldrb` at 0x57350 | load | 1 | `sp -0x1e` | `-` |
| `strb` at 0x57358 | store | 1 | `sp -0x1e` | `([sp-30]:1+1 & 0xff)` |
| `ldrb` at 0x57368 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x57380 | store | 4 | `sp -0x1c` | `ret(0xca4e8)` |
| `ldr` at 0x57384 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57390 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x57394 | store | 4 | `sp -0x78` | `ret(0xca4e8)` |
| `ldr` at 0x5739c | load | 4 | `abs 0x574b4` | `-` |
| `ldr` at 0x573a0 | load | 4 | `abs 0x12d6fc` | `-` |
| `str` at 0x573ac | store | 4 | `sp -0x74` | `0x0` |
| `ldr` at 0x573b0 | load | 4 | `sp -0x74` | `-` |
| `ldrb` at 0x573bc | load | 1 | `sp -0x11` | `-` |
| `str` at 0x573cc | store | 4 | `sp -0x1c` | `ret(0x472ac)` |
| `ldr` at 0x573d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x573e8 | load | 4 | `sp -0x74` | `-` |
| `str` at 0x573f0 | store | 4 | `sp -0x74` | `[sp-116]+1` |
| `ldr` at 0x573f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57404 | load | 4 | `abs 0x574b8` | `-` |
| `ldr` at 0x57408 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x5740c | load | 4 | `abs 0x13221c` | `-` |
| `ldr` at 0x5741c | load | 4 | `abs 0x574bc` | `-` |
| `str` at 0x57420 | store | 4 | `sp -0x88` | `0xf92cc` |
| `ldr` at 0x57424 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57430 | load | 4 | `abs 0x574c0` | `-` |
| `str` at 0x57434 | store | 4 | `sp -0x7c` | `0xf92fc` |
| `ldr` at 0x5743c | load | 4 | `abs 0x574c4` | `-` |
| `str` at 0x57440 | store | 4 | `sp -0x7c` | `0xf9304` |
| `ldr` at 0x57444 | load | 4 | `sp -0x7c` | `-` |
| `str` at 0x57448 | store | 4 | `sp -0x84` | `[sp-124]` |
| `ldr` at 0x5744c | load | 4 | `sp -0x74` | `-` |
| `str` at 0x57450 | store | 4 | `sp -0x80` | `?` |
| `ldr` at 0x57458 | load | 4 | `abs 0x574c8` | `-` |
| `ldr` at 0x5745c | load | 4 | `abs 0x574cc` | `-` |
| `ldr` at 0x57468 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57474 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x57478 | store | 4 | `sp -0x78` | `[sp-28]` |
| `str` at 0x574a0 | store | 4 | `sp -0x78` | `0x0` |
| `ldr` at 0x574a4 | load | 4 | `sp -0x78` | `-` |
| `ldm` at 0x574ac | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x574ac | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x574ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x570f8 | `(unknown callee) 0x57964` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `mov` at 0x57168 | `(unknown callee) (unknown target)` | `?` | `?` | `[0x12da68]` | `?` |
| `bl` at 0x57174 | `(unknown callee) 0x46d60` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x571a8 | `(unknown callee) 0xca4e8` | `0x0` | `[sp-29]:1` | `0x2` | `[sp-29]:1` |
| `bl` at 0x571d0 | `(unknown callee) 0x570a8` | `0xa` | `?` | `?` | `ret(0xca4e8)` |
| `bl` at 0x571e4 | `(unknown callee) 0xca538` | `0x0` | `[sp-29]:1` | `0x2` | `[sp-29]:1` |
| `bl` at 0x5720c | `(unknown callee) 0x570a8` | `0xa` | `?` | `?` | `ret(0xca538)` |
| `bl` at 0x57220 | `(unknown callee) 0xca4e8` | `0x0` | `[sp-29]:1` | `0x2` | `[sp-29]:1` |
| `bl` at 0x57248 | `(unknown callee) 0x570a8` | `0x15` | `?` | `?` | `ret(0xca4e8)` |
| `bl` at 0x57254 | `(unknown callee) 0x468c0` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x57288 | `(unknown callee) 0xd0754` | `0x1` | `0x0` | `sp-112` | `sp-112` |
| `bl` at 0x5729c | `(unknown callee) 0xca4e8` | `0x0` | `[sp-29]:1` | `0xc0` | `[sp-29]:1` |
| `bl` at 0x572c4 | `(unknown callee) 0x570a8` | `0x3` | `?` | `?` | `ret(0xca4e8)` |
| `bl` at 0x572f8 | `(unknown callee) 0x542cc` | `?` | `?` | `?` | `?` |
| `bl` at 0x5732c | `(unknown callee) 0xca4e8` | `0x0` | `?` | `0x4` | `?` |
| `bl` at 0x57364 | `(unknown callee) 0x570a8` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x57378 | `(unknown callee) 0xca4e8` | `0x0` | `[sp-29]:1` | `0x4` | `[sp-29]:1` |
| `bl` at 0x573a4 | `(unknown callee) 0x570a8` | `[0x12d6fc]` | `?` | `?` | `0x12d6fc` |
| `bl` at 0x573c4 | `(unknown callee) 0x472ac` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x573e4 | `(unknown callee) 0x570a8` | `0xa` | `?` | `?` | `ret(0x472ac)` |
| `bl` at 0x57464 | `(unknown callee) 0x200a0` | `0x4` | `0xf9204` | `0x3fb` | `0x0` |
| `bl` at 0x57490 | `(unknown callee) 0xd0754` | `0x1` | `0x0` | `sp-112` | `sp-112` |
| `bl` at 0x57498 | `(unknown callee) 0x47630` | `0x1` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5710c | `ret(0x57964) == 0x0` | `ret(0x57964) != 0x0` |
| `beq` at 0x57188 | `ret(0x46d60) == 0x0` | `ret(0x46d60) != 0x0` |
| `beq` at 0x571bc | `ret(0xca4e8) == 0x0` | `ret(0xca4e8) != 0x0` |
| `beq` at 0x571f8 | `ret(0xca538) == 0x0` | `ret(0xca538) != 0x0` |
| `beq` at 0x57234 | `ret(0xca4e8) == 0x0` | `ret(0xca4e8) != 0x0` |
| `beq` at 0x57268 | `ret(0x468c0) == 0x0` | `ret(0x468c0) != 0x0` |
| `beq` at 0x572b0 | `ret(0xca4e8) == 0x0` | `ret(0xca4e8) != 0x0` |
| `bhi` at 0x572d8 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0x57304 | `ret(0x542cc) == 0x0` | `ret(0x542cc) != 0x0` |
| `beq` at 0x57340 | `ret(0xca4e8) == 0x0` | `ret(0xca4e8) != 0x0` |
| `beq` at 0x5738c | `ret(0xca4e8) == 0x0` | `ret(0xca4e8) != 0x0` |
| `bgt` at 0x573b8 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0x573d8 | `ret(0x472ac) != 0x0` | `ret(0x472ac) == 0x0` |
| `bne` at 0x57400 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bgt` at 0x57414 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x5742c | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `beq` at 0x57470 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x574ac | `r0` | `?` |

## `IP57_SoftReset`

- `IP57_SoftReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x574d0 to 0x575b0.
  Established. The unnamed `0xca538` call carries `0x0`, `([[sp-24]+2]:1+2 & 0xff)`, `0x8`, `([[sp-24]+2]:1+2 & 0xff)` (`bl` at 0x5753c);
  its zero result takes `0x57560`, while nonzero falls through (`beq` at 0x57550).
  The byte at `sp -0x19` receives `([[sp-24]+2]:1+2 & 0xff)` (`strb` at 0x57528).
  Board soft-reset bus operations remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x574e4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x574e8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x57500 | store | 4 | `sp -0x20` | `ret(0x57964)` |
| `ldr` at 0x57504 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x57510 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x57514 | store | 4 | `sp -0x24` | `ret(0x57964)` |
| `ldr` at 0x5751c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57520 | load | 1 | `[sp-24] +0x2` | `-` |
| `strb` at 0x57528 | store | 1 | `sp -0x19` | `([[sp-24]+2]:1+2 & 0xff)` |
| `ldrb` at 0x5752c | load | 1 | `sp -0x19` | `-` |
| `str` at 0x57544 | store | 4 | `sp -0x20` | `ret(0xca538)` |
| `ldr` at 0x57548 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x57554 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x57558 | store | 4 | `sp -0x24` | `ret(0xca538)` |
| `ldrb` at 0x57568 | load | 1 | `sp -0x19` | `-` |
| `str` at 0x57580 | store | 4 | `sp -0x20` | `ret(0xca4e8)` |
| `ldr` at 0x57584 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x57590 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x57594 | store | 4 | `sp -0x24` | `ret(0xca4e8)` |
| `str` at 0x575a0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x575a4 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x575ac | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x575ac | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x575ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x574f8 | `(unknown callee) 0x57964` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x5753c | `(unknown callee) 0xca538` | `0x0` | `([[sp-24]+2]:1+2 & 0xff)` | `0x8` | `([[sp-24]+2]:1+2 & 0xff)` |
| `bl` at 0x57564 | `(unknown callee) 0x570a8` | `0xa` | `?` | `?` | `ret(0xca538)` |
| `bl` at 0x57578 | `(unknown callee) 0xca4e8` | `0x0` | `[sp-25]:1` | `0x8` | `[sp-25]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5750c | `ret(0x57964) == 0x0` | `ret(0x57964) != 0x0` |
| `beq` at 0x57550 | `ret(0xca538) == 0x0` | `ret(0xca538) != 0x0` |
| `beq` at 0x5758c | `ret(0xca4e8) == 0x0` | `ret(0xca4e8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x575ac | `r0` | `?` |

## `PICResetHold`

- `PICResetHold`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x345d8 to 0x34798.
  Established. An included absolute byte store clears `0x13212d` (`strb` at 0x34740).
  The unnamed `0xca538` call at `0x346b4` carries `0x0`, `[[sp-24]+2]:1`, `[[sp-24]+3]:1`, `[[sp-24]+3]:1` (`bl` at 0x346b4);
  its zero result takes `0x346d8`, while nonzero falls through (`beq` at 0x346c8).
  PIC hold descriptor operations remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x345e8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x345ec | load | 4 | `abs 0x34788` | `-` |
| `ldr` at 0x345f0 | load | 4 | `abs 0x132128` | `-` |
| `ldr` at 0x345fc | load | 4 | `abs 0x3478c` | `-` |
| `str` at 0x34600 | store | 4 | `sp -0x24` | `0xfffff1f0` |
| `str` at 0x3460c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x34610 | load | 4 | `abs 0x34790` | `-` |
| `ldrb` at 0x34614 | load | 1 | `abs 0x13212c` | `-` |
| `ldr` at 0x34618 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x34624 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x34638 | load | 4 | `abs 0x34788` | `-` |
| `ldr` at 0x3463c | load | 4 | `abs 0x132128` | `-` |
| `str` at 0x34644 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x34648 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3464c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x34650 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x34654 | load | 1 | `(unknown)` | `-` |
| `str` at 0x3466c | store | 4 | `sp -0x1c` | `ret(0xca538)` |
| `ldr` at 0x34670 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3467c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x34680 | store | 4 | `sp -0x24` | `ret(0xca538)` |
| `ldr` at 0x34688 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3468c | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0x34698 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3469c | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0x346a0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x346a4 | load | 1 | `[sp-24] +0x3` | `-` |
| `str` at 0x346bc | store | 4 | `sp -0x1c` | `ret(0xca538)` |
| `ldr` at 0x346c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x346cc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x346d0 | store | 4 | `sp -0x24` | `ret(0xca538)` |
| `ldr` at 0x346d8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x346dc | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x346e8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x346ec | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x346f0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x346f4 | load | 1 | `[sp-24] +0x5` | `-` |
| `str` at 0x3470c | store | 4 | `sp -0x1c` | `ret(0xca538)` |
| `ldr` at 0x34710 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3471c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x34720 | store | 4 | `sp -0x24` | `ret(0xca538)` |
| `ldr` at 0x34728 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x34730 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0x34738 | load | 4 | `abs 0x34794` | `-` |
| `strb` at 0x34740 | store | 1 | `abs 0x13212d` | `0x0` |
| `str` at 0x34758 | store | 4 | `sp -0x1c` | `ret(0x50a94)` |
| `ldr` at 0x3475c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x34768 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3476c | store | 4 | `sp -0x24` | `ret(0x50a94)` |
| `str` at 0x34778 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x3477c | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x34784 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x34784 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x34784 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x34664 | `(unknown callee) 0xca538` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x346b4 | `(unknown callee) 0xca538` | `0x0` | `[[sp-24]+2]:1` | `[[sp-24]+3]:1` | `[[sp-24]+3]:1` |
| `bl` at 0x34704 | `(unknown callee) 0xca538` | `0x0` | `[[sp-24]+4]:1` | `[[sp-24]+5]:1` | `[[sp-24]+5]:1` |
| `bl` at 0x34750 | `(unknown callee) 0x50a94` | `0x0` | `0xff` | `0xff` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x345f8 | `[0x132128] != 0x0` | `[0x132128] == 0x0` |
| `ble` at 0x34620 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x34678 | `ret(0xca538) == 0x0` | `ret(0xca538) != 0x0` |
| `beq` at 0x34694 | `[[sp-24]+2]:1 == 0xff` | `[[sp-24]+2]:1 != 0xff` |
| `beq` at 0x346c8 | `ret(0xca538) == 0x0` | `ret(0xca538) != 0x0` |
| `beq` at 0x346e4 | `[[sp-24]+4]:1 == 0xff` | `[[sp-24]+4]:1 != 0xff` |
| `beq` at 0x34718 | `ret(0xca538) == 0x0` | `ret(0xca538) != 0x0` |
| `beq` at 0x34764 | `ret(0x50a94) == 0x0` | `ret(0x50a94) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x34784 | `r0` | `?` |

## `PICResetRelease`

- `PICResetRelease`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x34798 to 0x34a80.
  Established. The absolute byte at `0x13212d` receives `0x1` (`strb` at 0x34a58).
  A nonzero `[0x132128]` takes `0x347d0`, while zero falls through (`bne` at 0x347c0).
  The unnamed `0x50d60` call carries `0x0`, `0xff`, `0xff`, `ret(0x51440)` (`bl` at 0x34a2c).
  PIC release ordering constraints remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x347a8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x347b0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x347b4 | load | 4 | `abs 0x34a70` | `-` |
| `ldr` at 0x347b8 | load | 4 | `abs 0x132128` | `-` |
| `ldr` at 0x347c4 | load | 4 | `abs 0x34a74` | `-` |
| `str` at 0x347c8 | store | 4 | `sp -0x2c` | `0xfffff1f0` |
| `str` at 0x347e4 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0x347fc | store | 4 | `sp -0x20` | `ret(0x51558)` |
| `ldr` at 0x34800 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3480c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x34810 | store | 4 | `sp -0x2c` | `ret(0x51558)` |
| `str` at 0x3481c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x34820 | load | 4 | `abs 0x34a78` | `-` |
| `ldrb` at 0x34824 | load | 1 | `abs 0x13212c` | `-` |
| `ldr` at 0x34828 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x34838 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x3483c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x34850 | load | 4 | `abs 0x34a70` | `-` |
| `ldr` at 0x34854 | load | 4 | `abs 0x132128` | `-` |
| `str` at 0x3485c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x34860 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x34864 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x34870 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x34878 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x3487c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x34880 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x34884 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x34888 | load | 1 | `(unknown)` | `-` |
| `str` at 0x348a0 | store | 4 | `sp -0x20` | `ret(0xca4e8)` |
| `ldr` at 0x348a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x348b0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x348b4 | store | 4 | `sp -0x2c` | `ret(0xca4e8)` |
| `ldr` at 0x348bc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x348c0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x348cc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x348d4 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x348d8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x348dc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x348e0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x348e4 | load | 1 | `(unknown)` | `-` |
| `str` at 0x348fc | store | 4 | `sp -0x20` | `ret(0xca4e8)` |
| `ldr` at 0x34900 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3490c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x34910 | store | 4 | `sp -0x2c` | `ret(0xca4e8)` |
| `ldr` at 0x34918 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x34920 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x34928 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x34940 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x34944 | load | 4 | `abs 0x34a78` | `-` |
| `ldrb` at 0x34948 | load | 1 | `abs 0x13212c` | `-` |
| `ldr` at 0x3494c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x34958 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3496c | load | 4 | `abs 0x34a70` | `-` |
| `ldr` at 0x34970 | load | 4 | `abs 0x132128` | `-` |
| `str` at 0x34978 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x3497c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x34980 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x34984 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x34988 | load | 1 | `(unknown)` | `-` |
| `str` at 0x349a0 | store | 4 | `sp -0x20` | `ret(0xca4e8)` |
| `ldr` at 0x349a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x349b0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x349b4 | store | 4 | `sp -0x2c` | `ret(0xca4e8)` |
| `ldr` at 0x349bc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x349c4 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `str` at 0x349dc | store | 4 | `sp -0x20` | `ret(0x51558)` |
| `ldr` at 0x349e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x349ec | load | 4 | `sp -0x20` | `-` |
| `str` at 0x349f0 | store | 4 | `sp -0x2c` | `ret(0x51558)` |
| `str` at 0x34a04 | store | 4 | `sp -0x20` | `ret(0x51440)` |
| `ldr` at 0x34a08 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x34a14 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x34a18 | store | 4 | `sp -0x2c` | `ret(0x51440)` |
| `str` at 0x34a34 | store | 4 | `sp -0x20` | `ret(0x50d60)` |
| `ldr` at 0x34a38 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x34a44 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x34a48 | store | 4 | `sp -0x2c` | `ret(0x50d60)` |
| `ldr` at 0x34a50 | load | 4 | `abs 0x34a7c` | `-` |
| `strb` at 0x34a58 | store | 1 | `abs 0x13212d` | `0x1` |
| `str` at 0x34a60 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x34a64 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x34a6c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x34a6c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x34a6c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x347d0 | `(unknown callee) 0x375e4` | `r0` | `r1` | `r2` | `[0x132128]` |
| `bl` at 0x347f4 | `(unknown callee) 0x51558` | `0x0` | `0x0` | `?` | `ret(0x375e4)` |
| `bl` at 0x34898 | `(unknown callee) 0xca4e8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x348f4 | `(unknown callee) 0xca4e8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x34938 | `(unknown callee) 0x34a80` | `0xa` | `?` | `[0x13212c]:1` | `[sp-28]` |
| `bl` at 0x34998 | `(unknown callee) 0xca4e8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x349d4 | `(unknown callee) 0x51558` | `0x0` | `0x1` | `[0x13212c]:1` | `?` |
| `bl` at 0x349fc | `(unknown callee) 0x51440` | `0x0` | `?` | `?` | `ret(0x51558)` |
| `bl` at 0x34a2c | `(unknown callee) 0x50d60` | `0x0` | `0xff` | `0xff` | `ret(0x51440)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x347c0 | `[0x132128] != 0x0` | `[0x132128] == 0x0` |
| `bne` at 0x347dc | `ret(0x375e4) != 0x0` | `ret(0x375e4) == 0x0` |
| `beq` at 0x34808 | `ret(0x51558) == 0x0` | `ret(0x51558) != 0x0` |
| `ble` at 0x34830 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x3486c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x348ac | `ret(0xca4e8) == 0x0` | `ret(0xca4e8) != 0x0` |
| `beq` at 0x348c8 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x34908 | `ret(0xca4e8) == 0x0` | `ret(0xca4e8) != 0x0` |
| `beq` at 0x34930 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `ble` at 0x34954 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x349ac | `ret(0xca4e8) == 0x0` | `ret(0xca4e8) != 0x0` |
| `beq` at 0x349e8 | `ret(0x51558) == 0x0` | `ret(0x51558) != 0x0` |
| `beq` at 0x34a10 | `ret(0x51440) == 0x0` | `ret(0x51440) != 0x0` |
| `beq` at 0x34a40 | `ret(0x50d60) == 0x0` | `ret(0x50d60) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x34a6c | `r0` | `?` |

## `NL4R_ResetHold`

- `NL4R_ResetHold`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x291e0 to 0x29310.
  Established. The absolute byte at `0x13208d` is cleared (`strb` at 0x29220).
  A nonzero `[0x13208c]:1` takes `0x29218`, while zero falls through (`bne` at 0x29208).
  The unnamed `0x2980c` call at `0x29264` has four unknown candidates (`bl` at 0x29264).
  Reset hold descriptor traversal remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x291f0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x291f8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x291fc | load | 4 | `abs 0x292fc` | `-` |
| `ldrb` at 0x29200 | load | 1 | `abs 0x13208c` | `-` |
| `ldr` at 0x2920c | load | 4 | `abs 0x29300` | `-` |
| `str` at 0x29210 | store | 4 | `sp -0x20` | `0xfffffed4` |
| `ldr` at 0x29218 | load | 4 | `abs 0x29304` | `-` |
| `strb` at 0x29220 | store | 1 | `abs 0x13208d` | `0x0` |
| `ldr` at 0x29224 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x29234 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x29238 | load | 4 | `abs 0x29308` | `-` |
| `ldr` at 0x2923c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29240 | load | 4 | `abs 0x132094` | `-` |
| `ldr` at 0x2924c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29254 | load | 4 | `abs 0x2930c` | `-` |
| `ldr` at 0x29258 | load | 4 | `abs 0x132090` | `-` |
| `str` at 0x2926c | store | 4 | `sp -0x1c` | `ret(0x2980c)` |
| `ldr` at 0x29270 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2927c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29288 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2928c | store | 4 | `sp -0x18` | `ret(0x2980c)` |
| `ldr` at 0x29290 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x29298 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0x292a0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x292a4 | store | 4 | `sp -0x20` | `[sp-24]` |
| `ldr` at 0x292ac | load | 4 | `abs 0x29308` | `-` |
| `ldr` at 0x292b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x292b4 | load | 4 | `abs 0x132094` | `-` |
| `str` at 0x292c4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x292cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x292d4 | load | 4 | `abs 0x2930c` | `-` |
| `ldr` at 0x292d8 | load | 4 | `abs 0x132090` | `-` |
| `str` at 0x292ec | store | 4 | `sp -0x20` | `ret(0x2980c)` |
| `ldr` at 0x292f0 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x292f8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x292f8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x292f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x29264 | `(unknown callee) 0x2980c` | `?` | `?` | `?` | `?` |
| `bl` at 0x292e4 | `(unknown callee) 0x2980c` | `?` | `r1` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x29208 | `[0x13208c]:1 != 0x0` | `[0x13208c]:1 == 0x0` |
| `bne` at 0x2922c | `[sp-20] != 0xff` | `[sp-20] == 0xff` |
| `bge` at 0x29248 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x29278 | `ret(0x2980c) == 0x0` | `ret(0x2980c) != 0x0` |
| `bne` at 0x29284 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `blt` at 0x292bc | `[sp-20] < [0x132094] (signed, 32-bit)` | `[sp-20] >= [0x132094] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x292f8 | `r0` | `?` |

## `NL4R_ResetRelease`

- `NL4R_ResetRelease`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x29310 to 0x2946c.
  Established. Included byte stores assign `0x0` and `0x1` to absolute `0x13208d` (`strb` at 0x29350, `strb` at 0x29424).
  The unnamed `0xd708c` call carries entry `r0`, entry `r1`, `0x13208d`, `0x0` (`bl` at 0x29354).
  A nonzero `[0x13208c]:1` takes `0x29348`, while zero falls through (`bne` at 0x29338).
  Release-time device synchronization remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x29320 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x29328 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x2932c | load | 4 | `abs 0x29458` | `-` |
| `ldrb` at 0x29330 | load | 1 | `abs 0x13208c` | `-` |
| `ldr` at 0x2933c | load | 4 | `abs 0x2945c` | `-` |
| `str` at 0x29340 | store | 4 | `sp -0x24` | `0xfffffed4` |
| `ldr` at 0x29348 | load | 4 | `abs 0x29460` | `-` |
| `strb` at 0x29350 | store | 1 | `abs 0x13208d` | `0x0` |
| `ldr` at 0x29358 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x29368 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x2936c | load | 4 | `abs 0x29464` | `-` |
| `ldr` at 0x29370 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29374 | load | 4 | `abs 0x132094` | `-` |
| `ldr` at 0x29380 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29388 | load | 4 | `abs 0x29468` | `-` |
| `ldr` at 0x2938c | load | 4 | `abs 0x132090` | `-` |
| `str` at 0x293a0 | store | 4 | `sp -0x20` | `ret(0x29998)` |
| `ldr` at 0x293a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x293b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x293bc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x293c0 | store | 4 | `sp -0x1c` | `ret(0x29998)` |
| `ldr` at 0x293c4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x293cc | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x293d4 | load | 4 | `abs 0x29464` | `-` |
| `ldr` at 0x293d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x293dc | load | 4 | `abs 0x132094` | `-` |
| `str` at 0x293ec | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x293f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x293fc | load | 4 | `abs 0x29468` | `-` |
| `ldr` at 0x29400 | load | 4 | `abs 0x132090` | `-` |
| `str` at 0x29414 | store | 4 | `sp -0x1c` | `ret(0x29998)` |
| `ldr` at 0x2941c | load | 4 | `abs 0x29460` | `-` |
| `strb` at 0x29424 | store | 1 | `abs 0x13208d` | `0x1` |
| `ldr` at 0x2943c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29444 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x29448 | store | 4 | `sp -0x24` | `[sp-28]` |
| `ldr` at 0x2944c | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x29454 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x29454 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x29454 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x29354 | `(unknown callee) 0xd708c` | `r0` | `r1` | `0x13208d` | `0x0` |
| `bl` at 0x29398 | `(unknown callee) 0x29998` | `?` | `?` | `?` | `?` |
| `bl` at 0x2940c | `(unknown callee) 0x29998` | `?` | `?` | `?` | `?` |
| `bl` at 0x29418 | `(unknown callee) 0xd70fc` | `?` | `?` | `?` | `?` |
| `bl` at 0x29428 | `(unknown callee) 0x488cc` | `ret(0xd70fc)` | `?` | `0x13208d` | `0x1` |
| `bl` at 0x29440 | `(unknown callee) 0x2958c` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x29338 | `[0x13208c]:1 != 0x0` | `[0x13208c]:1 == 0x0` |
| `bne` at 0x29360 | `[sp-20] != 0xff` | `[sp-20] == 0xff` |
| `bge` at 0x2937c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x293ac | `ret(0x29998) == 0x0` | `ret(0x29998) != 0x0` |
| `bne` at 0x293b8 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `blt` at 0x293e4 | `[sp-20] < [0x132094] (signed, 32-bit)` | `[sp-20] >= [0x132094] (signed, 32-bit)` |
| `beq` at 0x29438 | `? == 0x40` | `? != 0x40` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x29454 | `r0` | `?` |

## `NL4R_SoftReset`

- `NL4R_SoftReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2946c to 0x2958c.
  Established. The absolute byte at `0x13208c` is read (`ldrb` at 0x2948c).
  Entry `r0 != 0xff` takes `0x2952c`, while equality falls through (`bne` at 0x294ac).
  The word at `sp -0x20` receives `ret(0x29cd8)` at the included assignment (`str` at 0x2956c).
  Per-descriptor soft-reset behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2947c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x29484 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x29488 | load | 4 | `abs 0x2957c` | `-` |
| `ldrb` at 0x2948c | load | 1 | `abs 0x13208c` | `-` |
| `ldr` at 0x29498 | load | 4 | `abs 0x29580` | `-` |
| `str` at 0x2949c | store | 4 | `sp -0x20` | `0xfffffed4` |
| `ldr` at 0x294a4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x294b4 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x294b8 | load | 4 | `abs 0x29584` | `-` |
| `ldr` at 0x294bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x294c0 | load | 4 | `abs 0x132094` | `-` |
| `ldr` at 0x294cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x294d4 | load | 4 | `abs 0x29588` | `-` |
| `ldr` at 0x294d8 | load | 4 | `abs 0x132090` | `-` |
| `str` at 0x294ec | store | 4 | `sp -0x1c` | `ret(0x29cd8)` |
| `ldr` at 0x294f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x294fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29508 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2950c | store | 4 | `sp -0x18` | `ret(0x29cd8)` |
| `ldr` at 0x29510 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x29518 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0x29520 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x29524 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x2952c | load | 4 | `abs 0x29584` | `-` |
| `ldr` at 0x29530 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29534 | load | 4 | `abs 0x132094` | `-` |
| `str` at 0x29544 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x2954c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29554 | load | 4 | `abs 0x29588` | `-` |
| `ldr` at 0x29558 | load | 4 | `abs 0x132090` | `-` |
| `str` at 0x2956c | store | 4 | `sp -0x20` | `ret(0x29cd8)` |
| `ldr` at 0x29570 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x29578 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x29578 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x29578 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x294e4 | `(unknown callee) 0x29cd8` | `?` | `?` | `?` | `?` |
| `bl` at 0x29564 | `(unknown callee) 0x29cd8` | `?` | `r1` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x29494 | `[0x13208c]:1 != 0x0` | `[0x13208c]:1 == 0x0` |
| `bne` at 0x294ac | `r0 != 0xff` | `r0 == 0xff` |
| `bge` at 0x294c8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x294f8 | `ret(0x29cd8) == 0x0` | `ret(0x29cd8) != 0x0` |
| `bne` at 0x29504 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `blt` at 0x2953c | `r0 < [0x132094] (signed, 32-bit)` | `r0 >= [0x132094] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x29578 | `r0` | `?` |
