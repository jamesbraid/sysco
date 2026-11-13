# ARM power reset and state paths in 1.44.0

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
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/power-reset-state-facts.txt) retain all tool sections.

## `powerInit`

- `powerInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x35e44 to 0x36088.
  Established. The absolute word at `0x1321a0` receives `0x1` (`str` at 0x35eb4). The word at
  `0x13219c` receives `ret(0x196d0)` (`str` at 0x35ee8). The unnamed `0x19748` call carries
  `[[sp-44]+4]`, `0x1`, `[[sp-44]+8]`, `[sp-44]` (`bl` at 0x36048). Power table initialization
  effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x35e54 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x35e58 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x35e5c | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x35e60 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x35e64 | load | 4 | `abs 0x3605c` | `-` |
| `ldr` at 0x35e68 | load | 4 | `abs 0x13219c` | `-` |
| `ldr` at 0x35e74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x35e80 | load | 4 | `abs 0x36060` | `-` |
| `str` at 0x35e84 | store | 4 | `sp -0x3c` | `0xf563c` |
| `ldr` at 0x35e8c | load | 4 | `abs 0x36064` | `-` |
| `ldr` at 0x35e90 | load | 4 | `abs 0x36068` | `-` |
| `ldr` at 0x35e9c | load | 4 | `abs 0x3606c` | `-` |
| `ldr` at 0x35ea0 | load | 4 | `abs 0x1321a0` | `-` |
| `ldr` at 0x35eac | load | 4 | `abs 0x3606c` | `-` |
| `str` at 0x35eb4 | store | 4 | `abs 0x1321a0` | `0x1` |
| `ldr` at 0x35eb8 | load | 4 | `abs 0x3606c` | `-` |
| `ldr` at 0x35ebc | load | 4 | `abs 0x1321a0` | `-` |
| `str` at 0x35ed0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x35ed4 | load | 4 | `abs 0x3605c` | `-` |
| `ldr` at 0x35ed8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x35ee8 | store | 4 | `abs 0x13219c` | `ret(0x196d0)` |
| `ldr` at 0x35eec | load | 4 | `abs 0x13219c` | `-` |
| `ldr` at 0x35ef8 | load | 4 | `abs 0x36070` | `-` |
| `str` at 0x35efc | store | 4 | `sp -0x3c` | `0xf5674` |
| `ldr` at 0x35f00 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x35f04 | store | 4 | `sp -0x38` | `[sp-40]` |
| `ldr` at 0x35f0c | load | 4 | `abs 0x36064` | `-` |
| `ldr` at 0x35f10 | load | 4 | `abs 0x36074` | `-` |
| `ldr` at 0x35f1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x35f30 | load | 4 | `abs 0x3605c` | `-` |
| `ldr` at 0x35f34 | load | 4 | `abs 0x13219c` | `-` |
| `str` at 0x35f3c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x35f40 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x35f44 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x35f48 | store | 4 | `[sp-44] +0x0` | `?` |
| `ldr` at 0x35f4c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x35f50 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x35f54 | store | 4 | `[sp-44] +0x8` | `[sp-32]` |
| `ldr` at 0x35f58 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x35f5c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x35f60 | store | 4 | `[sp-44] +0x10` | `[sp-36]` |
| `ldr` at 0x35f64 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x35f68 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x35f6c | store | 4 | `[sp-44] +0x14` | `[sp]` |
| `ldr` at 0x35f70 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x35f74 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x35f78 | load | 4 | `[sp-44] +0x8` | `-` |
| `str` at 0x35f88 | store | 4 | `[sp-44] +0x4` | `ret(0x196d0)` |
| `ldr` at 0x35f8c | load | 4 | `[sp-44] +0x4` | `-` |
| `ldr` at 0x35f98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x35fa0 | load | 4 | `abs 0x36078` | `-` |
| `ldr` at 0x35fa4 | load | 4 | `abs 0x3607c` | `-` |
| `str` at 0x35fe0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x35fe4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x35fec | load | 4 | `abs 0x36078` | `-` |
| `ldr` at 0x35ff0 | load | 4 | `abs 0x3607c` | `-` |
| `ldr` at 0x35ffc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3600c | load | 4 | `abs 0x36080` | `-` |
| `str` at 0x36010 | store | 4 | `(unknown)` | `0xf56c0` |
| `str` at 0x36014 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0x36018 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3601c | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0x36024 | load | 4 | `abs 0x36064` | `-` |
| `ldr` at 0x36028 | load | 4 | `abs 0x36084` | `-` |
| `ldr` at 0x36034 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x36038 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3603c | load | 4 | `[sp-44] +0x4` | `-` |
| `ldr` at 0x36044 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldm` at 0x36058 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x36058 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x36058 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x36058 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x35e98 | `(unknown callee) 0x200a0` | `0x9` | `0xf565c` | `0x1a9` | `0x0` |
| `bl` at 0x35ee0 | `(unknown callee) 0x196d0` | `[sp-40]` | `0x1` | `[0x1321a0]` | `?` |
| `bl` at 0x35f18 | `(unknown callee) 0x200a0` | `0x9` | `0xf565c` | `0x1b3` | `0x0` |
| `bl` at 0x35f80 | `(unknown callee) 0x196d0` | `[[sp-44]+8]` | `0x1` | `[sp-44]` | `[sp-44]` |
| `bl` at 0x35fa8 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0xf56b4` | `0xf56bc` |
| `bl` at 0x35fb4 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x35ff4 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0xf56b4` | `0xf56bc` |
| `bl` at 0x36004 | `(unknown callee) 0x19808` | `[sp-48]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x36030 | `(unknown callee) 0x200a0` | `0x9` | `0xf565c` | `0x1c1` | `0x0` |
| `bl` at 0x36048 | `(unknown callee) 0x19748` | `[[sp-44]+4]` | `0x1` | `[[sp-44]+8]` | `[sp-44]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x35e70 | `[0x13219c] == 0x0` | `[0x13219c] != 0x0` |
| `bne` at 0x35e7c | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0x35ea8 | `[0x1321a0] != 0x0` | `[0x1321a0] == 0x0` |
| `bne` at 0x35ef4 | `[0x13219c] != 0x0` | `[0x13219c] == 0x0` |
| `bne` at 0x35f94 | `[[sp-44]+4] != 0x0` | `[[sp-44]+4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x36058 | `r0` | `0x0` |

## `powerReset`

- `powerReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x362f8 to 0x364c4.
  Established. An included byte assignment through `[sp-24] +0xf` writes `0x5` (`strb` at 0x36460).
  The unnamed `0x3a950` call carries `[sp-20]`, `[sp-28]`, `[[sp-24]+4]`, `[[sp-28]+6]:1` (`bl` at 0x363b8). The unnamed `0x3a430` call carries `[sp-20]`, `[sp-28]`, `[sp-29]:1`, `[sp-29]:1` (`bl` at 0x36474). Power reset state policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x36308 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x3630c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x36320 | load | 4 | `abs 0x364c0` | `-` |
| `ldr` at 0x36324 | load | 4 | `abs 0x13219c` | `-` |
| `str` at 0x3632c | store | 4 | `sp -0x18` | `?` |
| `str` at 0x36334 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x36338 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x36340 | store | 1 | `[sp-24] +0xf` | `?` |
| `str` at 0x36348 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x3634c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x36350 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x36354 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x36360 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x36364 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x36378 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x36380 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x36384 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x36388 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x3638c | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x36394 | load | 1 | `([[sp-24]+4] + [sp-48]) +0x0` | `-` |
| `ldr` at 0x363a0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x363a4 | load | 1 | `[sp-28] +0x6` | `-` |
| `ldr` at 0x363b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x363b4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x363c0 | store | 4 | `sp -0x24` | `ret(0x3a950)` |
| `ldr` at 0x363c4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x363c8 | store | 4 | `sp -0x34` | `ret(0x3a950)` |
| `ldr` at 0x363cc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x363d8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x363e4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x363f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x363f4 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x36400 | store | 1 | `sp -0x1d` | `(ret(0x3a8d8) & 0xff)` |
| `str` at 0x3640c | store | 4 | `sp -0x24` | `0x2` |
| `ldr` at 0x36410 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x36414 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x36420 | store | 1 | `sp -0x1d` | `(ret(0x39f34) & 0xff)` |
| `ldr` at 0x36424 | load | 4 | `sp -0x18` | `-` |
| `ldrsb` at 0x36428 | load | 1 | `[sp-24] +0xf` | `-` |
| `ldr` at 0x36434 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x36438 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x3643c | store | 1 | `[sp-24] +0xf` | `([sp-36] & 0xff)` |
| `ldr` at 0x36444 | load | 4 | `sp -0x18` | `-` |
| `ldrsb` at 0x36448 | load | 1 | `[sp-24] +0xf` | `-` |
| `ldr` at 0x3644c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x36458 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x36460 | store | 1 | `[sp-24] +0xf` | `0x5` |
| `ldrb` at 0x36464 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x36468 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3646c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3647c | store | 4 | `sp -0x2c` | `ret(0x3a430)` |
| `ldr` at 0x36480 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3648c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x36498 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3649c | store | 4 | `sp -0x28` | `ret(0x3a430)` |
| `ldr` at 0x364a0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x364a8 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x364b0 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x364bc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x364bc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x364bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x363b8 | `(unknown callee) 0x3a950` | `[sp-20]` | `[sp-28]` | `[[sp-24]+4]` | `[[sp-28]+6]:1` |
| `bl` at 0x363f8 | `(unknown callee) 0x3a8d8` | `[sp-20]` | `[sp-28]` | `?` | `ret(0x3a950)` |
| `bl` at 0x36418 | `(unknown callee) 0x39f34` | `[sp-20]` | `[sp-28]` | `?` | `?` |
| `bl` at 0x36474 | `(unknown callee) 0x3a430` | `[sp-20]` | `[sp-28]` | `[sp-29]:1` | `[sp-29]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x3635c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x3639c | `[([[sp-24]+4] + [sp-48])]:1 == 0x0` | `[([[sp-24]+4] + [sp-48])]:1 != 0x0` |
| `beq` at 0x363ac | `[[sp-28]+6]:1 == 0xff` | `[[sp-28]+6]:1 != 0xff` |
| `beq` at 0x363d4 | `ret(0x3a950) == 0x0` | `ret(0x3a950) != 0x0` |
| `blt` at 0x363e0 | `ret(0x3a950) < 0x0 (signed, 32-bit)` | `ret(0x3a950) >= 0x0 (signed, 32-bit)` |
| `bgt` at 0x363ec | `ret(0x3a950) > 0x4 (signed, 32-bit)` | `ret(0x3a950) <= 0x4 (signed, 32-bit)` |
| `bne` at 0x36430 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x36454 | `? == [sp-36]` | `? != [sp-36]` |
| `beq` at 0x36488 | `ret(0x3a430) == 0x0` | `ret(0x3a430) != 0x0` |
| `bne` at 0x36494 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x364bc | `r0` | `[sp-40]` |

## `powerCycle`

- `powerCycle`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x37558 to 0x375c4.
  Established. The unnamed `0x37174` call carries entry `r0`, `r1`, `r2`, `r3` (`bl` at 0x37570). A
  zero result takes `0x37594`, while nonzero falls through (`beq` at 0x37584). The unnamed `0x368ec`
  call carries `0x1`, `[sp-20]`, unknown, unknown (`bl` at 0x375a4). Power cycle timing and effects
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x37568 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x3756c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x37578 | store | 4 | `sp -0x18` | `ret(0x37174)` |
| `ldr` at 0x3757c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x37588 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3758c | store | 4 | `sp -0x1c` | `ret(0x37174)` |
| `ldr` at 0x375a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x375ac | store | 4 | `sp -0x18` | `ret(0x368ec)` |
| `ldr` at 0x375b0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x375b4 | store | 4 | `sp -0x1c` | `ret(0x368ec)` |
| `ldr` at 0x375b8 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x375c0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x375c0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x375c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x37570 | `(unknown callee) 0x37174` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x37598 | `(unknown callee) 0x368c4` | `0xc8` | `?` | `?` | `ret(0x37174)` |
| `bl` at 0x375a4 | `(unknown callee) 0x368ec` | `0x1` | `[sp-20]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x37584 | `ret(0x37174) == 0x0` | `ret(0x37174) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x375c0 | `r0` | `?` |

## `powerState`

- `powerState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x37960 to 0x37b4c.
  Established. The unnamed `0x398cc` call carries `[sp-20]`, `ret(0x39f8c)`, `0x0`, `sp-40` (`bl` at 0x37ab4). The coprocessor store at `sp -0x34` has unknown width and value (`stc` at 0x37ac4).
  Included word assignments at `sp -0x30` write `0x1` and `0x2` in separate paths (`str` at 0x37b18,
  `str` at 0x37b24). Power state thresholds remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x37970 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x37974 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x37978 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3798c | load | 4 | `abs 0x37b44` | `-` |
| `ldr` at 0x37990 | load | 4 | `abs 0x13219c` | `-` |
| `str` at 0x37998 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x3799c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x379a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x379b4 | load | 4 | `abs 0x37b48` | `-` |
| `str` at 0x379b8 | store | 4 | `sp -0x30` | `0xfffff9b9` |
| `ldr` at 0x379c0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x379c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x379d8 | load | 4 | `[sp-44] +0x0` | `-` |
| `str` at 0x379e0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x379e4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x379e8 | load | 1 | `[sp-32] +0x2` | `-` |
| `ldr` at 0x379f4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x379f8 | load | 1 | `[sp-32] +0x2` | `-` |
| `ldr` at 0x379fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x37a0c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x37a10 | load | 1 | `[sp-32] +0x3` | `-` |
| `str` at 0x37a1c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x37a20 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37a34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x37a44 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37a58 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x37a6c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x37a74 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x37a78 | load | 1 | `[sp-32] +0x4` | `-` |
| `str` at 0x37a88 | store | 4 | `sp -0x30` | `0x2` |
| `ldr` at 0x37a90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x37a94 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37aa4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x37ab8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x37abc | load | 4 | `sp -0x18` | `-` |
| `stc` at 0x37ac4 | store | unknown | `sp -0x34` | `?` |
| `str` at 0x37acc | store | 4 | `sp -0x24` | `?` |
| `ldc` at 0x37ad0 | load | unknown | `sp -0x28` | `-` |
| `ldc` at 0x37ad8 | load | unknown | `sp -0x24` | `-` |
| `ldc` at 0x37ae0 | load | unknown | `abs 0x37b34` | `-` |
| `ldc` at 0x37af0 | load | unknown | `sp -0x28` | `-` |
| `ldc` at 0x37af8 | load | unknown | `sp -0x24` | `-` |
| `ldc` at 0x37b00 | load | unknown | `abs 0x37b3c` | `-` |
| `str` at 0x37b18 | store | 4 | `sp -0x30` | `0x1` |
| `str` at 0x37b24 | store | 4 | `sp -0x30` | `0x2` |
| `ldr` at 0x37b28 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x37b30 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x37b30 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x37b30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x379a4 | `(unknown callee) 0x36528` | `r0` | `r1` | `?` | `?` |
| `bl` at 0x37a04 | `(unknown callee) 0xca7a0` | `[sp-20]` | `[[sp-32]+2]:1` | `?` | `[[sp-32]+2]:1` |
| `bl` at 0x37a24 | `(unknown callee) 0x3a150` | `[sp-32]` | `?` | `ret(0xca7a0)` | `?` |
| `bl` at 0x37a48 | `(unknown callee) 0x3a150` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x37a98 | `(unknown callee) 0x39f8c` | `[sp-20]` | `[sp-32]` | `?` | `[[sp-32]+4]:1` |
| `bl` at 0x37ab4 | `(unknown callee) 0x398cc` | `[sp-20]` | `ret(0x39f8c)` | `0x0` | `sp-40` |
| `bl` at 0x37ac0 | `(unknown callee) 0x39ddc` | `[sp-20]` | `[sp-24]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x379b0 | `ret(0x36528) != 0x0` | `ret(0x36528) == 0x0` |
| `beq` at 0x379f0 | `[[sp-32]+2]:1 == 0xff` | `[[sp-32]+2]:1 != 0xff` |
| `beq` at 0x37a30 | `ret(0x3a150) == 0x0` | `ret(0x3a150) != 0x0` |
| `bne` at 0x37a3c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0x37a54 | `ret(0x3a150) != 0x0` | `ret(0x3a150) == 0x0` |
| `bne` at 0x37a60 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0x37a80 | `[[sp-32]+4]:1 != 0xff` | `[[sp-32]+4]:1 == 0xff` |
| `bgt` at 0x37aec | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bmi` at 0x37b0c | `N == 1` | `not (N == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x37b30 | `r0` | `?` |

## `powerStatusSlot`

- `powerStatusSlot`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x37674 to 0x3783c.
  Established. The byte at `([[sp-44]+4] + [sp-24]) +0x0` is read (`ldrb` at 0x3770c). The unnamed
  `0xca7a0` call carries `[sp-20]`, `[[sp-48]+2]:1`, unknown, `[[sp-48]+2]:1` (`bl` at 0x37780). A
  word at `sp -0x1c` receives `[sp-28]+1` on an included path (`str` at 0x377c8). Slot status
  aggregation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x37684 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x37688 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3769c | load | 4 | `abs 0x37838` | `-` |
| `ldr` at 0x376a0 | load | 4 | `abs 0x13219c` | `-` |
| `str` at 0x376a8 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0x376b0 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x376b8 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x376c0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x376c4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x376c8 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldr` at 0x376cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x376d8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x376dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x376f0 | load | 4 | `[sp-44] +0x0` | `-` |
| `str` at 0x376f8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x376fc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x37700 | load | 4 | `[sp-44] +0x4` | `-` |
| `ldr` at 0x37704 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3770c | load | 1 | `([[sp-44]+4] + [sp-24]) +0x0` | `-` |
| `ldr` at 0x37718 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x3771c | load | 1 | `[sp-48] +0x2` | `-` |
| `ldr` at 0x37728 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x3772c | load | 1 | `[sp-48] +0x1` | `-` |
| `ldr` at 0x37730 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x37734 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldr` at 0x37740 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x37744 | load | 1 | `[sp-48] +0x1` | `-` |
| `ldr` at 0x37748 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x37764 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3776c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0x37770 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x37774 | load | 1 | `[sp-48] +0x2` | `-` |
| `ldr` at 0x37778 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x37788 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x3778c | load | 1 | `[sp-48] +0x3` | `-` |
| `str` at 0x377a4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x377a8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x377b4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x377c0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x377c8 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x377cc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x377d4 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x377dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x377e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x377ec | load | 4 | `sp -0x20` | `-` |
| `str` at 0x377fc | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0x37804 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x37814 | store | 4 | `sp -0x38` | `?` |
| `str` at 0x37820 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x37824 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x37828 | store | 4 | `sp -0x34` | `[sp-56]` |
| `ldr` at 0x3782c | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x37834 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x37834 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x37834 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x37750 | `(unknown callee) 0x3783c` | `[sp-20]` | `[[sp-48]+1]:1` | `[[sp-48]+1]:1` | `[[sp-48]+1]:1` |
| `bl` at 0x37780 | `(unknown callee) 0xca7a0` | `[sp-20]` | `[[sp-48]+2]:1` | `?` | `[[sp-48]+2]:1` |
| `bl` at 0x377ac | `(unknown callee) 0x3a150` | `[sp-48]` | `?` | `ret(0xca7a0)` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x376d4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x37714 | `[([[sp-44]+4] + [sp-24])]:1 == 0x0` | `[([[sp-44]+4] + [sp-24])]:1 != 0x0` |
| `beq` at 0x37724 | `[[sp-48]+2]:1 == 0xff` | `[[sp-48]+2]:1 != 0xff` |
| `bge` at 0x3773c | `[[sp-48]+1]:1 >= [[sp-44]+8] (signed, 32-bit)` | `[[sp-48]+1]:1 < [[sp-44]+8] (signed, 32-bit)` |
| `bgt` at 0x3775c | `ret(0x3783c) > 0x0 (signed, 32-bit)` | `ret(0x3783c) <= 0x0 (signed, 32-bit)` |
| `bne` at 0x377bc | `ret(0x3a150) != [sp-36]` | `ret(0x3a150) == [sp-36]` |
| `beq` at 0x377e4 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x377f4 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x3780c | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x37834 | `r0` | `[sp-52]` |

## `powerMarginAll`

- `powerMarginAll`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x37fb4 to 0x3801c.
  Established. The unnamed `0x3801c` call carries `0x0`, `0x21`, entry `r0`, `0x0` (`bl` at 0x37fd8). A nonzero result takes `0x38004`, while zero falls through (`bne` at 0x37fec). The
  absolute word at `0x12d6f8` receives `[sp-20]` on an included path (`str` at 0x37ff8). Aggregate
  margin policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x37fc4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x37fd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x37fd4 | load | 4 | `abs 0x38014` | `-` |
| `str` at 0x37fe0 | store | 4 | `sp -0x18` | `ret(0x3801c)` |
| `ldr` at 0x37fe4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x37ff0 | load | 4 | `abs 0x38018` | `-` |
| `ldr` at 0x37ff4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x37ff8 | store | 4 | `abs 0x12d6f8` | `[sp-20]` |
| `ldr` at 0x38004 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x38010 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x38010 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x38010 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x37fd8 | `(unknown callee) 0x3801c` | `0x0` | `0x21` | `r0` | `0x0` |
| `bl` at 0x38000 | `(unknown callee) 0x6a7b0` | `[sp-20]` | `?` | `0x12d6f8` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x37fec | `ret(0x3801c) != 0x0` | `ret(0x3801c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x38010 | `r0` | `?` |

## `powerQuery`

- `powerQuery`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x88894 to 0x88ca4.
  Established. Included word assignments through `[sp-24] +0x0` write `ret(0x1943c)` (`str` at 0x888e4, `str` at 0x88924). A nonzero `ret(0x364c4)` takes `0x888fc`, while zero falls through
  (`bne` at 0x888d0). The coprocessor store at `sp -0x64` has unknown width and value (`stc` at 0x88b18). Power report formatting meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x888a4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x888a8 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x888b0 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x888c4 | store | 4 | `sp -0x28` | `ret(0x364c4)` |
| `ldr` at 0x888c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x888d4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x888e4 | store | 4 | `[sp-24] +0x0` | `ret(0x1943c)` |
| `ldr` at 0x888e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x888ec | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x888f0 | load | 4 | `abs 0x88c60` | `-` |
| `ldr` at 0x888fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x88900 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x88924 | store | 4 | `[sp-24] +0x0` | `ret(0x1943c)` |
| `ldr` at 0x88928 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x8892c | store | 4 | `sp -0x4c` | `[[sp-24]]` |
| `ldr` at 0x88930 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8893c | load | 4 | `abs 0x88c64` | `-` |
| `str` at 0x88940 | store | 4 | `sp -0x50` | `0x108ffc` |
| `ldr` at 0x88948 | load | 4 | `abs 0x88c68` | `-` |
| `str` at 0x8894c | store | 4 | `sp -0x50` | `0x102184` |
| `ldr` at 0x88950 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8895c | load | 4 | `abs 0x88c6c` | `-` |
| `str` at 0x88960 | store | 4 | `sp -0x54` | `0x109004` |
| `ldr` at 0x88968 | load | 4 | `abs 0x88c68` | `-` |
| `str` at 0x8896c | store | 4 | `sp -0x54` | `0x102184` |
| `ldr` at 0x88970 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x88974 | load | 4 | `abs 0x88c70` | `-` |
| `ldr` at 0x88978 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x8897c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x88984 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x88994 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x8899c | store | 4 | `sp -0x4c` | `([sp-76] + ret(0x19538))` |
| `str` at 0x889a4 | store | 4 | `sp -0x2c` | `0x0` |
| `strb` at 0x889ac | store | 1 | `sp -0x2d` | `?` |
| `ldr` at 0x889b0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x889b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x889c4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x889d0 | store | 4 | `sp -0x34` | `ret(0x39b0c)` |
| `ldr` at 0x889d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x889e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x889f4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x889fc | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x88a0c | load | 1 | `sp -0x2d` | `-` |
| `strb` at 0x88a14 | store | 1 | `sp -0x2d` | `([sp-45]:1+1 & 0xff)` |
| `ldr` at 0x88a1c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x88a34 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x88a40 | store | 4 | `sp -0x40` | `ret(0x3783c)` |
| `ldr` at 0x88a44 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x88a48 | load | 4 | `abs 0x88c74` | `-` |
| `ldr` at 0x88a54 | load | 4 | `abs 0x88c78` | `-` |
| `str` at 0x88a58 | store | 4 | `sp -0x44` | `0x109070` |
| `ldr` at 0x88a60 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x88a6c | load | 4 | `abs 0x88c7c` | `-` |
| `str` at 0x88a70 | store | 4 | `sp -0x58` | `0x109074` |
| `ldr` at 0x88a78 | load | 4 | `abs 0x88c80` | `-` |
| `str` at 0x88a7c | store | 4 | `sp -0x58` | `0x109078` |
| `ldr` at 0x88a80 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x88a84 | store | 4 | `sp -0x44` | `[sp-88]` |
| `ldr` at 0x88a88 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x88a9c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x88aa4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x88ab0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x88ab4 | load | 4 | `abs 0x88c84` | `-` |
| `ldr` at 0x88ab8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x88abc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x88ad0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x88ae0 | store | 4 | `sp -0x20` | `ret(0x398cc)` |
| `ldr` at 0x88ae4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x88af0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x88af4 | store | 4 | `sp -0x64` | `ret(0x398cc)` |
| `ldr` at 0x88af8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x88afc | load | 4 | `abs 0x88c88` | `-` |
| `ldr` at 0x88b00 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x88b04 | load | 4 | `sp -0x44` | `-` |
| `ldc` at 0x88b10 | load | unknown | `sp -0x48` | `-` |
| `stc` at 0x88b18 | store | unknown | `sp -0x64` | `?` |
| `ldr` at 0x88b1c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x88b20 | load | 4 | `abs 0x88c8c` | `-` |
| `ldr` at 0x88b24 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x88b28 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x88b30 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x88b40 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x88b48 | store | 4 | `sp -0x4c` | `([sp-76] + ret(0x19538))` |
| `ldr` at 0x88b4c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x88b60 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x88b68 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x88b74 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x88b78 | load | 4 | `abs 0x88c90` | `-` |
| `ldr` at 0x88b88 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x88b94 | store | 4 | `sp -0x38` | `ret(0x39c00)` |
| `ldr` at 0x88ba0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x88bb0 | store | 4 | `sp -0x20` | `ret(0x37c5c)` |
| `ldr` at 0x88bb4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x88bc0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x88bc4 | load | 4 | `abs 0x88c94` | `-` |
| `ldr` at 0x88bc8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x88bcc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x88bd8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x88be4 | load | 4 | `abs 0x88c98` | `-` |
| `str` at 0x88be8 | store | 4 | `sp -0x5c` | `0x1090ec` |
| `ldr` at 0x88bf0 | load | 4 | `abs 0x88c9c` | `-` |
| `str` at 0x88bf4 | store | 4 | `sp -0x5c` | `0x1090fc` |
| `ldrb` at 0x88bf8 | load | 1 | `sp -0x39` | `-` |
| `ldrb` at 0x88bfc | load | 1 | `sp -0x2d` | `-` |
| `str` at 0x88c00 | store | 4 | `sp -0x64` | `[sp-45]:1` |
| `ldr` at 0x88c04 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x88c08 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x88c0c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x88c1c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x88c20 | load | 4 | `abs 0x88ca0` | `-` |
| `ldr` at 0x88c24 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x88c2c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x88c3c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x88c44 | store | 4 | `sp -0x4c` | `([sp-76] + ret(0x19538))` |
| `ldr` at 0x88c48 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x88c50 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldm` at 0x88c5c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x88c5c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x88c5c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x88c5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x888c0 | `(unknown callee) 0x364c4` | `0x0` | `sp-36` | `r2` | `sp-36` |
| `bl` at 0x888dc | `(unknown callee) 0x1943c` | `0x50` | `?` | `?` | `ret(0x364c4)` |
| `bl` at 0x888f4 | `(unknown callee) 0x19808` | `[[sp-24]]` | `0x108fd4` | `?` | `[sp-24]` |
| `bl` at 0x8891c | `(unknown callee) 0x1943c` | `?` | `?` | `ret(0x364c4)` | `?` |
| `bl` at 0x88980 | `(unknown callee) 0x197a8` | `[[sp-24]]` | `0x10900c` | `[sp-80]` | `[sp-84]` |
| `bl` at 0x88988 | `(unknown callee) 0x19538` | `[sp-76]` | `?` | `?` | `?` |
| `bl` at 0x889c8 | `(unknown callee) 0x39b0c` | `0x0` | `[sp-44]` | `[sp-44]` | `[sp-40]` |
| `bl` at 0x88a20 | `(unknown callee) 0x36528` | `0x0` | `[sp-44]` | `?` | `[sp-45]:1+1` |
| `bl` at 0x88a38 | `(unknown callee) 0x3783c` | `0x0` | `[sp-44]` | `?` | `ret(0x36528)` |
| `bl` at 0x88ac0 | `(unknown callee) 0x197a8` | `[sp-76]` | `0x10907c` | `[sp-52]` | `[sp-68]` |
| `bl` at 0x88ad8 | `(unknown callee) 0x398cc` | `0x0` | `[sp-44]` | `0x0` | `sp-72` |
| `bl` at 0x88b08 | `(unknown callee) 0x197a8` | `[sp-76]` | `0x109098` | `[sp-52]` | `[sp-68]` |
| `bl` at 0x88b2c | `(unknown callee) 0x197a8` | `[sp-76]` | `0x1090b4` | `[sp-52]` | `[sp-68]` |
| `bl` at 0x88b34 | `(unknown callee) 0x19538` | `[sp-76]` | `?` | `?` | `?` |
| `bl` at 0x88b7c | `(unknown callee) 0x197a8` | `[sp-76]` | `0x1090cc` | `?` | `?` |
| `bl` at 0x88b8c | `(unknown callee) 0x39c00` | `0x0` | `[sp-44]` | `?` | `?` |
| `bl` at 0x88ba8 | `(unknown callee) 0x37c5c` | `0x0` | `[sp-44]` | `sp-57` | `sp-57` |
| `bl` at 0x88bd0 | `(unknown callee) 0x197a8` | `[sp-76]` | `0x1090d8` | `[sp-56]` | `ret(0x37c5c)` |
| `bl` at 0x88c14 | `(unknown callee) 0x197a8` | `[sp-76]` | `[sp-92]` | `[sp-56]` | `[sp-57]:1` |
| `bl` at 0x88c28 | `(unknown callee) 0x197a8` | `[sp-76]` | `0x109108` | `[sp-52]` | `ret(0x36528)` |
| `bl` at 0x88c30 | `(unknown callee) 0x19538` | `[sp-76]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x888d0 | `ret(0x364c4) != 0x0` | `ret(0x364c4) == 0x0` |
| `beq` at 0x88938 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `beq` at 0x88958 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `bhs` at 0x889bc | `[sp-44] >= [sp-40] (unsigned, 32-bit)` | `[sp-44] < [sp-40] (unsigned, 32-bit)` |
| `beq` at 0x889dc | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `bne` at 0x88a04 | `? != 0xff` | `? == 0xff` |
| `beq` at 0x88a2c | `ret(0x36528) == 0x0` | `ret(0x36528) != 0x0` |
| `bne` at 0x88a50 | `ret(0x3783c) != 0xfffff9b8` | `ret(0x3783c) == 0xfffff9b8` |
| `bne` at 0x88a68 | `ret(0x3783c) != 0x1` | `ret(0x3783c) == 0x1` |
| `bne` at 0x88aac | `? != 0xff` | `? == 0xff` |
| `beq` at 0x88aec | `ret(0x398cc) == 0x0` | `ret(0x398cc) != 0x0` |
| `bne` at 0x88b70 | `? != 0xff` | `? == 0xff` |
| `beq` at 0x88bbc | `ret(0x37c5c) == 0x0` | `ret(0x37c5c) != 0x0` |
| `beq` at 0x88be0 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x88c5c | `r0` | `?` |

## `EnvPower`

- `EnvPower`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x76be8 to 0x76c10.
  Established. The unnamed `0x1b5f0` call carries `0x7d`, entry `r0`, entry `r2`, entry `r3` (`bl` at 0x76c04). The unnamed `0x76ca4` call carries `ret(0x1b5f0)` and three unknown candidates (`bl` at 0x76c08). The return reports `ret(0x76ca4)` (`ldm` at 0x76c0c). Environment power request
  meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x76bf8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x76c00 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x76c0c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x76c0c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x76c0c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x76c0c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x76c04 | `(unknown callee) 0x1b5f0` | `0x7d` | `r0` | `r2` | `r3` |
| `bl` at 0x76c08 | `(unknown callee) 0x76ca4` | `ret(0x1b5f0)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x76c0c | `r0` | `ret(0x76ca4)` |

## `EnvPowerDown`

- `EnvPowerDown`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x76f60 to 0x76f80.
  Established. The word at `sp -0x14` receives entry `r0` (`str` at 0x76f70). The unnamed `0x7111c`
  call carries entry `r0`, `r1`, `r2`, `r3` (`bl` at 0x76f78). The return reports `ret(0x7111c)`
  (`ldm` at 0x76f7c). Environment power-down effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x76f70 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x76f74 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x76f7c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x76f7c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x76f7c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x76f7c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x76f78 | `(unknown callee) 0x7111c` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x76f7c | `r0` | `ret(0x7111c)` |

## `EnvPowerDownDone`

- `EnvPowerDownDone`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x76f98 to 0x76fb4.
  Established. The unnamed `0x75654` call carries `0x4`, entry `r1`, entry `r2`, entry `r3` (`bl` at 0x76fa8). The unnamed `0x7a790` call carries `ret(0x75654)` and three unknown candidates (`bl` at 0x76fac). The return reports `ret(0x7a790)` (`ldm` at 0x76fb0). Power-down completion policy
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldm` at 0x76fb0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x76fb0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x76fb0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x76fa8 | `(unknown callee) 0x75654` | `0x4` | `r1` | `r2` | `r3` |
| `bl` at 0x76fac | `(unknown callee) 0x7a790` | `ret(0x75654)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x76fb0 | `r0` | `ret(0x7a790)` |

## `pcixPowerUp`

- `pcixPowerUp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x508a4 to 0x5099c.
  Established. The absolute word at `0x1324a0` is read (`ldr` at 0x508c4); its nonzero value takes
  `0x508dc`, while zero falls through (`bne` at 0x508cc). The unnamed `0x51c34` call carries
  `[sp-20]` and three unknown candidates (`bl` at 0x50944). PCI-X power-up sequencing remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x508b4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x508bc | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x508c0 | load | 4 | `abs 0x50994` | `-` |
| `ldr` at 0x508c4 | load | 4 | `abs 0x1324a0` | `-` |
| `str` at 0x508d4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x508dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x508f0 | load | 4 | `abs 0x50998` | `-` |
| `ldr` at 0x508f4 | load | 4 | `abs 0x13249c` | `-` |
| `str` at 0x508fc | store | 4 | `sp -0x18` | `?` |
| `str` at 0x50904 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x50908 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5090c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x50910 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5091c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x50920 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x50934 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x5093c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5094c | store | 4 | `sp -0x20` | `ret(0x51c34)` |
| `ldr` at 0x50950 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5095c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x50968 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5096c | store | 4 | `sp -0x1c` | `ret(0x51c34)` |
| `ldr` at 0x50970 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x50978 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x50980 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x50984 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x50988 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x50990 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x50990 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x50990 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x50944 | `(unknown callee) 0x51c34` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x508cc | `[0x1324a0] != 0x0` | `[0x1324a0] == 0x0` |
| `ble` at 0x50918 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x50958 | `ret(0x51c34) == 0x0` | `ret(0x51c34) != 0x0` |
| `bne` at 0x50964 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x50990 | `r0` | `?` |

## `pcixPowerDown`

- `pcixPowerDown`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5099c to 0x50a94.
  Established. A nonzero absolute word `[0x1324a0]` takes `0x509d4`, while zero falls through (`bne` at 0x509c4).
  The unnamed `0x522fc` call carries `[sp-20]` and three unknown candidates (`bl` at 0x50a3c). The word at `sp -0x1c` receives `ret(0x522fc)` on an included path (`str` at 0x50a64).
  PCI-X power-down effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x509ac | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x509b4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x509b8 | load | 4 | `abs 0x50a8c` | `-` |
| `ldr` at 0x509bc | load | 4 | `abs 0x1324a0` | `-` |
| `str` at 0x509cc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x509d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x509e8 | load | 4 | `abs 0x50a90` | `-` |
| `ldr` at 0x509ec | load | 4 | `abs 0x13249c` | `-` |
| `str` at 0x509f4 | store | 4 | `sp -0x18` | `?` |
| `str` at 0x509fc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x50a00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x50a04 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x50a08 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x50a14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x50a18 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x50a2c | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x50a34 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x50a44 | store | 4 | `sp -0x20` | `ret(0x522fc)` |
| `ldr` at 0x50a48 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x50a54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x50a60 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x50a64 | store | 4 | `sp -0x1c` | `ret(0x522fc)` |
| `ldr` at 0x50a68 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x50a70 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x50a78 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x50a7c | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x50a80 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x50a88 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x50a88 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x50a88 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x50a3c | `(unknown callee) 0x522fc` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x509c4 | `[0x1324a0] != 0x0` | `[0x1324a0] == 0x0` |
| `ble` at 0x50a10 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x50a50 | `ret(0x522fc) == 0x0` | `ret(0x522fc) != 0x0` |
| `bne` at 0x50a5c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x50a88 | `r0` | `?` |

## `IP57_PowerUp`

- `IP57_PowerUp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x56c20 to 0x56d28.
  Established. The unnamed `0x57964` call carries `(r0 & 0xff)`, `sp-24`, `sp-24`, `(r0 & 0xff)`
  (`bl` at 0x56c50). A zero result takes `0x56c74`, while nonzero falls through (`beq` at 0x56c64).
  The unnamed `0xca8c4` call carries `0x0`, unknown, `0x80`, unknown (`bl` at 0x56cd8). Board
  power-up behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x56c34 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x56c3c | store | 4 | `sp -0x1c` | `0x0` |
| `ldrb` at 0x56c40 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x56c58 | store | 4 | `sp -0x20` | `ret(0x57964)` |
| `ldr` at 0x56c5c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x56c68 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x56c6c | store | 4 | `sp -0x28` | `ret(0x57964)` |
| `strb` at 0x56c78 | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0x56c7c | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0x56c88 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x56c94 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x56cb4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x56cb8 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0x56cbc | load | 1 | `sp -0x21` | `-` |
| `str` at 0x56ce0 | store | 4 | `sp -0x20` | `ret(0xca8c4)` |
| `ldr` at 0x56ce4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x56cf0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x56cfc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x56d00 | store | 4 | `sp -0x1c` | `ret(0xca8c4)` |
| `ldrb` at 0x56d04 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x56d0c | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldr` at 0x56d14 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x56d18 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x56d1c | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x56d24 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x56d24 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x56d24 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x56c50 | `(unknown callee) 0x57964` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x56ca4 | `(unknown callee) 0x542cc` | `?` | `?` | `?` | `?` |
| `bl` at 0x56cd8 | `(unknown callee) 0xca8c4` | `0x0` | `?` | `0x80` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x56c64 | `ret(0x57964) == 0x0` | `ret(0x57964) != 0x0` |
| `bhi` at 0x56c84 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0x56cb0 | `ret(0x542cc) == 0x0` | `ret(0x542cc) != 0x0` |
| `beq` at 0x56cec | `ret(0xca8c4) == 0x0` | `ret(0xca8c4) != 0x0` |
| `bne` at 0x56cf8 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x56d24 | `r0` | `?` |

## `IP57_PowerDown`

- `IP57_PowerDown`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x56d28 to 0x56e04.
  Established. The unnamed `0x57964` call carries `(r0 & 0xff)`, `sp-24`, `sp-24`, `(r0 & 0xff)`
  (`bl` at 0x56d58). The byte through `[sp-24] +0x2` is read (`ldrb` at 0x56d94). The unnamed
  `0xca878` call carries `0x0`, unknown, `0x80`, unknown (`bl` at 0x56db4). Board power-down
  behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x56d3c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x56d44 | store | 4 | `sp -0x1c` | `0x0` |
| `ldrb` at 0x56d48 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x56d60 | store | 4 | `sp -0x20` | `ret(0x57964)` |
| `ldr` at 0x56d64 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x56d70 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x56d74 | store | 4 | `sp -0x28` | `ret(0x57964)` |
| `strb` at 0x56d80 | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0x56d84 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x56d90 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x56d94 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0x56d98 | load | 1 | `sp -0x21` | `-` |
| `str` at 0x56dbc | store | 4 | `sp -0x20` | `ret(0xca878)` |
| `ldr` at 0x56dc0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x56dcc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x56dd8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x56ddc | store | 4 | `sp -0x1c` | `ret(0xca878)` |
| `ldrb` at 0x56de0 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x56de8 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldr` at 0x56df0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x56df4 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x56df8 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x56e00 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x56e00 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x56e00 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x56d58 | `(unknown callee) 0x57964` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x56db4 | `(unknown callee) 0xca878` | `0x0` | `?` | `0x80` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x56d6c | `ret(0x57964) == 0x0` | `ret(0x57964) != 0x0` |
| `bhi` at 0x56d8c | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0x56dc8 | `ret(0xca878) == 0x0` | `ret(0xca878) != 0x0` |
| `bne` at 0x56dd4 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x56e00 | `r0` | `?` |

## `L1CfgEventPowerState`

- `L1CfgEventPowerState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6a754 to 0x6a7b0.
  Established. The word through `ret(0x6a0f8) +0x34` receives `[sp-20]` on an included path (`str` at 0x6a794). Equality of `[ret(0x6a0f8)+52]` and `[sp-20]` takes `0x6a7a8`, while inequality falls
  through (`beq` at 0x6a788). The unnamed `0x6a474` call carries `0x4`, `0x0`, `0x0`, `[sp-20]`
  (`bl` at 0x6a7a4). Power state event meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6a764 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6a774 | store | 4 | `sp -0x18` | `ret(0x6a0f8)` |
| `ldr` at 0x6a778 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a77c | load | 4 | `ret(0x6a0f8) +0x34` | `-` |
| `ldr` at 0x6a780 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6a78c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a790 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6a794 | store | 4 | `ret(0x6a0f8) +0x34` | `[sp-20]` |
| `ldm` at 0x6a7ac | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6a7ac | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6a7ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6a76c | `(unknown callee) 0x6a0f8` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x6a7a4 | `(unknown callee) 0x6a474` | `0x4` | `0x0` | `0x0` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6a788 | `[ret(0x6a0f8)+52] == [sp-20]` | `[ret(0x6a0f8)+52] != [sp-20]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6a7ac | `r0` | `?` |
