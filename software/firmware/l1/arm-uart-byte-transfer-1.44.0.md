# ARM UART and I2C byte-transfer paths in 1.44.0

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
Unknown data values and instruction effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/uart-byte-transfer-facts.txt) retain all tool sections.

## `uartInit`

- `uartInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbcfbc to 0xbd18c.
  Established. The absolute word at `0x29aef0` receives `[0x29aef0]+1` (`str` at 0xbd070). Included
  word assignments through `[sp-32] +0x8`, `+0xc` and `+0x2c` write `[sp-20]`, `[sp-24]` and
  `[sp-28]` respectively (`str` at 0xbd08c, `str` at 0xbd098, `str` at 0xbd0a4). The unnamed
  `0x19748` call carries `[sp-32]`, `0x0`, `0x108`, `[0x29aef0]+1` (`bl` at 0xbd080). UART setup
  effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbcfcc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbcfd0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xbcfd4 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xbcfd8 | load | 4 | `abs 0xbd170` | `-` |
| `ldr` at 0xbcfdc | load | 4 | `abs 0x29aef0` | `-` |
| `ldr` at 0xbcfe8 | load | 4 | `abs 0xbd174` | `-` |
| `str` at 0xbcfec | store | 4 | `sp -0x2c` | `0x11f3c8` |
| `ldr` at 0xbcff4 | load | 4 | `abs 0xbd178` | `-` |
| `ldr` at 0xbd004 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbd018 | load | 4 | `abs 0xbd17c` | `-` |
| `str` at 0xbd01c | store | 4 | `sp -0x2c` | `0x11f3f4` |
| `ldr` at 0xbd020 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbd024 | store | 4 | `sp -0x28` | `[sp-20]` |
| `ldr` at 0xbd02c | load | 4 | `abs 0xbd178` | `-` |
| `ldr` at 0xbd03c | load | 4 | `abs 0xbd170` | `-` |
| `ldr` at 0xbd040 | load | 4 | `abs 0x29aef0` | `-` |
| `ldr` at 0xbd054 | load | 4 | `abs 0xbd180` | `-` |
| `str` at 0xbd05c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xbd060 | load | 4 | `abs 0xbd170` | `-` |
| `ldr` at 0xbd064 | load | 4 | `abs 0xbd170` | `-` |
| `ldr` at 0xbd068 | load | 4 | `abs 0x29aef0` | `-` |
| `str` at 0xbd070 | store | 4 | `abs 0x29aef0` | `[0x29aef0]+1` |
| `ldr` at 0xbd074 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbd084 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbd088 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbd08c | store | 4 | `[sp-32] +0x8` | `[sp-20]` |
| `ldr` at 0xbd090 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbd094 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbd098 | store | 4 | `[sp-32] +0xc` | `[sp-24]` |
| `ldr` at 0xbd09c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbd0a0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbd0a4 | store | 4 | `[sp-32] +0x2c` | `[sp-28]` |
| `ldr` at 0xbd0a8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbd0b0 | store | 4 | `[sp-32] +0x0` | `?` |
| `ldr` at 0xbd0b4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbd0bc | store | 4 | `[sp-32] +0x4` | `?` |
| `ldr` at 0xbd0c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbd0cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbd0d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbd0dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbd0e0 | load | 4 | `abs 0xbd184` | `-` |
| `strb` at 0xbd0f0 | store | 1 | `[sp-32] +0x31` | `?` |
| `ldr` at 0xbd0f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbd104 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbd110 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbd114 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbd118 | load | 4 | `abs 0xbd188` | `-` |
| `strb` at 0xbd128 | store | 1 | `[sp-32] +0x31` | `?` |
| `ldr` at 0xbd130 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbd13c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbd148 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbd14c | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xbd15c | store | 1 | `[sp-32] +0x31` | `?` |
| `ldr` at 0xbd160 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xbd16c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbd16c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbd16c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbd000 | `(unknown callee) 0x200a0` | `0x9` | `0x11f3ec` | `0x58` | `0x0` |
| `bl` at 0xbd008 | `(unknown callee) 0xbdd50` | `?` | `?` | `?` | `?` |
| `bl` at 0xbd038 | `(unknown callee) 0x200a0` | `0x9` | `0x11f3ec` | `0x5b` | `0x0` |
| `bl` at 0xbd080 | `(unknown callee) 0x19748` | `[sp-32]` | `0x0` | `0x108` | `[0x29aef0]+1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xbcfe4 | `[0x29aef0] <= 0x13 (signed, 32-bit)` | `[0x29aef0] > 0x13 (signed, 32-bit)` |
| `beq` at 0xbd014 | `ret(0xbdd50) == 0x0` | `ret(0xbdd50) != 0x0` |
| `bls` at 0xbd0c8 | `[sp-20] <= 0x4 (unsigned, 32-bit)` | `[sp-20] > 0x4 (unsigned, 32-bit)` |
| `bhi` at 0xbd0d4 | `[sp-20] > 0xc (unsigned, 32-bit)` | `[sp-20] <= 0xc (unsigned, 32-bit)` |
| `bls` at 0xbd100 | `[sp-20] <= 0xc (unsigned, 32-bit)` | `[sp-20] > 0xc (unsigned, 32-bit)` |
| `bhi` at 0xbd10c | `[sp-20] > 0x10 (unsigned, 32-bit)` | `[sp-20] <= 0x10 (unsigned, 32-bit)` |
| `bls` at 0xbd138 | `[sp-20] <= 0x10 (unsigned, 32-bit)` | `[sp-20] > 0x10 (unsigned, 32-bit)` |
| `bhi` at 0xbd144 | `[sp-20] > 0x16 (unsigned, 32-bit)` | `[sp-20] <= 0x16 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbd16c | `r0` | `[sp-32]` |

## `uartOpen`

- `uartOpen`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbd18c to 0xbd60c.
  Established. A nonzero `ret(0xbdd50)` takes `0xbd1d4`, while zero falls through (`bne` at 0xbd1c4). The unnamed `0x19718` call at `0xbd53c` carries `[sp-60]`, `0x80047503`, `0x400`,
  `[sp-60]` (`bl` at 0xbd53c); its later call at `0xbd5bc` carries `[sp-64]`, `0x80047506`, `0x1`,
  `[sp-64]` (`bl` at 0xbd5bc). UART open request meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbd19c | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0xbd1a0 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0xbd1a8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xbd1ac | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbd1b8 | store | 4 | `sp -0x24` | `ret(0xbdd50)` |
| `ldr` at 0xbd1bc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbd1cc | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xbd1d4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbd1dc | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xbd1e0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbd1ec | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbd1f8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbd208 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbd218 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbd21c | load | 4 | `ret(0xbdd50) +0x34` | `-` |
| `ldr` at 0xbd228 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbd234 | load | 4 | `ret(0xbdd50) +0x34` | `-` |
| `str` at 0xbd244 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xbd24c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbd254 | store | 4 | `(unknown)` | `ret(0x2091c)` |
| `str` at 0xbd25c | store | 4 | `sp -0x2c` | `0x4` |
| `ldr` at 0xbd264 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbd268 | load | 4 | `ret(0xbdd50) +0x38` | `-` |
| `ldr` at 0xbd274 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbd280 | load | 4 | `ret(0xbdd50) +0x38` | `-` |
| `str` at 0xbd290 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xbd298 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbd2a0 | store | 4 | `(unknown)` | `ret(0x2091c)` |
| `str` at 0xbd2a8 | store | 4 | `sp -0x2c` | `0x2` |
| `ldr` at 0xbd2b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbd2b4 | load | 4 | `ret(0xbdd50) +0x34` | `-` |
| `ldr` at 0xbd2c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbd2c4 | load | 4 | `ret(0xbdd50) +0x38` | `-` |
| `ldr` at 0xbd2d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbd2e0 | load | 4 | `ret(0xbdd50) +0x34` | `-` |
| `ldr` at 0xbd2ec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbd2f8 | load | 4 | `[sp-36] +0x38` | `-` |
| `str` at 0xbd30c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xbd314 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbd318 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbd320 | store | 4 | `(unknown)` | `ret(0x2091c)` |
| `ldr` at 0xbd324 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbd328 | store | 4 | `(unknown)` | `?` |
| `str` at 0xbd330 | store | 4 | `sp -0x2c` | `0x6` |
| `ldr` at 0xbd334 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbd340 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbd34c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbd350 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbd35c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbd360 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbd36c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbd370 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbd380 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbd388 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xbd38c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbd394 | store | 4 | `[sp-36] +0x4` | `0x1` |
| `ldr` at 0xbd398 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbd39c | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xbd3ac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbd3b8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbd3c4 | load | 4 | `abs 0xbd5f4` | `-` |
| `ldr` at 0xbd3c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbd3cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbd3d0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbd3dc | store | 4 | `sp -0x30` | `ret(0x1967c)` |
| `ldr` at 0xbd3e0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbd3f4 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xbd408 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xbd420 | load | 4 | `abs 0xbd5f8` | `-` |
| `str` at 0xbd424 | store | 4 | `sp -0x50` | `0x11f424` |
| `ldr` at 0xbd428 | load | 4 | `abs 0xbd5fc` | `-` |
| `str` at 0xbd42c | store | 4 | `sp -0x4c` | `0x11f418` |
| `str` at 0xbd430 | store | 4 | `sp -0x48` | `ret(0x1f8e0)` |
| `ldr` at 0xbd434 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `str` at 0xbd438 | store | 4 | `sp -0x44` | `[ret(0x193d0)]` |
| `ldr` at 0xbd440 | load | 4 | `abs 0xbd600` | `-` |
| `str` at 0xbd454 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xbd458 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbd46c | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xbd47c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbd488 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbd498 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbd49c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xbd4a0 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0xbd4a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbd4b0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbd4c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbd4c4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xbd4c8 | store | 4 | `[sp-36] +0x4` | `?` |
| `ldr` at 0xbd4cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbd4d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbd4dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbd4e8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbd4ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbd4fc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbd50c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbd510 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbd514 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xbd51c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbd520 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbd524 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xbd528 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbd52c | store | 4 | `sp -0x30` | `[sp-60]` |
| `ldr` at 0xbd530 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbd534 | load | 4 | `abs 0xbd604` | `-` |
| `ldr` at 0xbd54c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbd550 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbd55c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xbd560 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbd56c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xbd570 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbd57c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbd58c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbd590 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbd594 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xbd59c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbd5a0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbd5a4 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xbd5a8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbd5ac | store | 4 | `sp -0x30` | `[sp-64]` |
| `ldr` at 0xbd5b0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbd5b4 | load | 4 | `abs 0xbd608` | `-` |
| `ldr` at 0xbd5cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbd5d0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbd5dc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xbd5e0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbd5e4 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xbd5e8 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0xbd5f0 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xbd5f0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xbd5f0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbd5f0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbd5f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbd1b0 | `(unknown callee) 0xbdd50` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0xbd22c | `(unknown callee) 0x2091c` | `ret(0xbdd50)` | `?` | `?` | `[ret(0xbdd50)+52]` |
| `bl` at 0xbd250 | `(unknown callee) 0x2091c` | `?` | `?` | `?` | `[ret(0xbdd50)+52]` |
| `bl` at 0xbd278 | `(unknown callee) 0x2091c` | `ret(0xbdd50)` | `?` | `?` | `[ret(0xbdd50)+56]` |
| `bl` at 0xbd29c | `(unknown callee) 0x2091c` | `?` | `?` | `?` | `[ret(0xbdd50)+56]` |
| `bl` at 0xbd2d8 | `(unknown callee) 0x2091c` | `ret(0xbdd50)` | `?` | `?` | `?` |
| `bl` at 0xbd2f0 | `(unknown callee) 0x2091c` | `ret(0x2091c)` | `?` | `ret(0x2091c)` | `[ret(0xbdd50)+52]` |
| `bl` at 0xbd31c | `(unknown callee) 0x2091c` | `?` | `?` | `?` | `?` |
| `bl` at 0xbd3a4 | `(unknown callee) 0xbde4c` | `[[sp-36]]` | `0xf` | `[sp-36]` | `[sp-36]` |
| `bl` at 0xbd3d4 | `(unknown callee) 0x1967c` | `?` | `[sp-32]` | `0x12d91c` | `[sp-28]` |
| `bl` at 0xbd3ec | `(unknown callee) 0x193d0` | `ret(0x1967c)` | `?` | `?` | `ret(0x1967c)` |
| `bl` at 0xbd400 | `(unknown callee) 0x193d0` | `ret(0x193d0)` | `?` | `?` | `[ret(0x193d0)]` |
| `bl` at 0xbd40c | `(unknown callee) 0x1f8e0` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |
| `bl` at 0xbd414 | `(unknown callee) 0x193d0` | `ret(0x1f8e0)` | `?` | `?` | `?` |
| `bl` at 0xbd44c | `(unknown callee) 0x200a0` | `0x3` | `0x11f3ec` | `0xcf` | `0x0` |
| `bl` at 0xbd464 | `(unknown callee) 0x193d0` | `?` | `?` | `?` | `?` |
| `bl` at 0xbd53c | `(unknown callee) 0x19718` | `[sp-60]` | `0x80047503` | `0x400` | `[sp-60]` |
| `bl` at 0xbd554 | `(unknown callee) 0xbd60c` | `[sp-36]` | `[sp-32]` | `?` | `ret(0x19718)` |
| `bl` at 0xbd5bc | `(unknown callee) 0x19718` | `[sp-64]` | `0x80047506` | `0x1` | `[sp-64]` |
| `bl` at 0xbd5d4 | `(unknown callee) 0xbd60c` | `[sp-36]` | `[sp-32]` | `?` | `ret(0x19718)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xbd1c4 | `ret(0xbdd50) != 0x0` | `ret(0xbdd50) == 0x0` |
| `beq` at 0xbd1e8 | `[sp-56] == 0x1` | `[sp-56] != 0x1` |
| `bgt` at 0xbd1f4 | `[sp-56] > 0x1 (signed, 32-bit)` | `[sp-56] <= 0x1 (signed, 32-bit)` |
| `beq` at 0xbd200 | `[sp-56] == 0x0` | `[sp-56] != 0x0` |
| `beq` at 0xbd210 | `[sp-56] == 0x2` | `[sp-56] != 0x2` |
| `beq` at 0xbd224 | `[ret(0xbdd50)+52] == 0x0` | `[ret(0xbdd50)+52] != 0x0` |
| `beq` at 0xbd23c | `[ret(0xbdd50)+52] == ret(0x2091c)` | `[ret(0xbdd50)+52] != ret(0x2091c)` |
| `beq` at 0xbd270 | `[ret(0xbdd50)+56] == 0x0` | `[ret(0xbdd50)+56] != 0x0` |
| `beq` at 0xbd288 | `[ret(0xbdd50)+56] == ret(0x2091c)` | `[ret(0xbdd50)+56] != ret(0x2091c)` |
| `bne` at 0xbd2bc | `[ret(0xbdd50)+52] != 0x0` | `[ret(0xbdd50)+52] == 0x0` |
| `bne` at 0xbd2cc | `[ret(0xbdd50)+56] != 0x0` | `[ret(0xbdd50)+56] == 0x0` |
| `bne` at 0xbd2e8 | `[ret(0xbdd50)+52] != ret(0x2091c)` | `[ret(0xbdd50)+52] == ret(0x2091c)` |
| `bne` at 0xbd300 | `[[sp-36]+56] != ret(0x2091c)` | `[[sp-36]+56] == ret(0x2091c)` |
| `beq` at 0xbd33c | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xbd348 | `[sp-28] != 0x1` | `[sp-28] == 0x1` |
| `bge` at 0xbd358 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xbd368 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xbd378 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xbd3b4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xbd3c0 | `[sp-44] == 0x0` | `[sp-44] != 0x0` |
| `bge` at 0xbd3e8 | `ret(0x1967c) >= 0x0 (signed, 32-bit)` | `ret(0x1967c) < 0x0 (signed, 32-bit)` |
| `beq` at 0xbd3fc | `[ret(0x193d0)] == 0x4` | `[ret(0x193d0)] != 0x4` |
| `bge` at 0xbd460 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xbd474 | `[ret(0x193d0)] != 0x4` | `[ret(0x193d0)] == 0x4` |
| `beq` at 0xbd484 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0xbd494 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xbd4ac | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0xbd4bc | `? == 0x0` | `? != 0x0` |
| `beq` at 0xbd4d4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xbd4e4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xbd4f4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xbd508 | `? == 0x0` | `? != 0x0` |
| `bge` at 0xbd548 | `ret(0x19718) >= 0x0 (signed, 32-bit)` | `ret(0x19718) < 0x0 (signed, 32-bit)` |
| `beq` at 0xbd568 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xbd578 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xbd588 | `? == 0x0` | `? != 0x0` |
| `bge` at 0xbd5c8 | `ret(0x19718) >= 0x0 (signed, 32-bit)` | `ret(0x19718) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbd5f0 | `r0` | `?` |

## `uartClose`

- `uartClose`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbd60c to 0xbd808.
  Established. The word through `[sp-24] +0x34` receives zero on an included path (`str` at 0xbd6a4). A later included zero word assignment uses `[sp-24] +0x38` (`str` at 0xbd6e0). The
  unnamed `0x2091c` call at `0xbd67c` carries entry `r0`, `r1`, `r2`, `[sp-48]` (`bl` at 0xbd67c).
  Port release policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbd61c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xbd620 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xbd628 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xbd630 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xbd634 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbd63c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xbd640 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbd64c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbd658 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbd668 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbd678 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbd684 | load | 4 | `r0 +0x34` | `-` |
| `str` at 0xbd694 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xbd69c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbd6a4 | store | 4 | `[sp-24] +0x34` | `0x0` |
| `str` at 0xbd6ac | store | 4 | `sp -0x2c` | `0x4` |
| `ldr` at 0xbd6b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbd6c0 | load | 4 | `r0 +0x38` | `-` |
| `str` at 0xbd6d0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xbd6d8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbd6e0 | store | 4 | `[sp-24] +0x38` | `0x0` |
| `str` at 0xbd6e8 | store | 4 | `sp -0x2c` | `0x2` |
| `ldr` at 0xbd6f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbd6fc | load | 4 | `r0 +0x34` | `-` |
| `ldr` at 0xbd708 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbd714 | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0xbd728 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xbd730 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbd738 | store | 4 | `[sp-24] +0x34` | `0x0` |
| `ldr` at 0xbd73c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbd744 | store | 4 | `[sp-24] +0x38` | `0x0` |
| `str` at 0xbd74c | store | 4 | `sp -0x2c` | `0x6` |
| `ldr` at 0xbd750 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbd754 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbd760 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbd764 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbd770 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbd774 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbd780 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbd784 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbd78c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbd790 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbd798 | store | 4 | `[sp-24] +0x4` | `?` |
| `str` at 0xbd7a0 | store | 4 | `[sp-24] +0x0` | `?` |
| `ldr` at 0xbd7a8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbd7b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbd7bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbd7c4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbd7cc | store | 4 | `[sp-24] +0x0` | `?` |
| `ldr` at 0xbd7d0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbd7e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbd7e4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbd7ec | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbd7f4 | store | 4 | `[sp-24] +0x4` | `?` |
| `ldr` at 0xbd7f8 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xbd804 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xbd804 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbd804 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbd804 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbd67c | `(unknown callee) 0x2091c` | `r0` | `r1` | `r2` | `[sp-48]` |
| `bl` at 0xbd6b8 | `(unknown callee) 0x2091c` | `r0` | `r1` | `r2` | `[sp-48]` |
| `bl` at 0xbd6f4 | `(unknown callee) 0x2091c` | `r0` | `r1` | `r2` | `[sp-48]` |
| `bl` at 0xbd70c | `(unknown callee) 0x2091c` | `ret(0x2091c)` | `?` | `ret(0x2091c)` | `[r0+52]` |
| `bl` at 0xbd788 | `(unknown callee) 0xbdf94` | `?` | `?` | `?` | `?` |
| `bl` at 0xbd7c0 | `(unknown callee) 0x1931c` | `?` | `?` | `?` | `?` |
| `bl` at 0xbd7e8 | `(unknown callee) 0x1931c` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xbd648 | `[sp-48] == 0x1` | `[sp-48] != 0x1` |
| `bgt` at 0xbd654 | `[sp-48] > 0x1 (signed, 32-bit)` | `[sp-48] <= 0x1 (signed, 32-bit)` |
| `beq` at 0xbd660 | `[sp-48] == 0x0` | `[sp-48] != 0x0` |
| `beq` at 0xbd670 | `[sp-48] == 0x2` | `[sp-48] != 0x2` |
| `beq` at 0xbd68c | `[r0+52] == ret(0x2091c)` | `[r0+52] != ret(0x2091c)` |
| `beq` at 0xbd6c8 | `[r0+56] == ret(0x2091c)` | `[r0+56] != ret(0x2091c)` |
| `bne` at 0xbd704 | `[r0+52] != ret(0x2091c)` | `[r0+52] == ret(0x2091c)` |
| `bne` at 0xbd71c | `[[sp-24]+56] != ret(0x2091c)` | `[[sp-24]+56] == ret(0x2091c)` |
| `bne` at 0xbd75c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xbd76c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xbd77c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xbd7b4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xbd7dc | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbd804 | `r0` | `?` |

## `uartRead`

- `uartRead`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbd808 to 0xbd9c0.
  Established. A positive signed entry `r2` takes `0xbd83c`, while a nonpositive value falls through
  (`bgt` at 0xbd82c). The unnamed `0x1937c` call carries `[[sp-20]]+1`, `sp-160`, `0x0`, `0x0` (`bl` at 0xbd8f8). The unnamed `0x197fc` call carries `[[sp-20]]`, `[sp-24]`, `[sp-28]`, `[sp-20]` (`bl` at 0xbd930). UART receive semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbd818 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbd81c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xbd820 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xbd824 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbd834 | store | 4 | `sp -0xac` | `0x0` |
| `str` at 0xbd840 | store | 4 | `sp -0xa8` | `sp-160` |
| `str` at 0xbd848 | store | 4 | `sp -0xa4` | `0x0` |
| `ldr` at 0xbd84c | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0xbd858 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0xbd85c | load | 4 | `sp -0xa4` | `-` |
| `str` at 0xbd864 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xbd868 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0xbd870 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0xbd878 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbd87c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbd8b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbd8b8 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xbd8c8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbd8d0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xbd8d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbd8d8 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xbd8e8 | store | 4 | `sp -0xb0` | `0x0` |
| `ldr` at 0xbd910 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `str` at 0xbd918 | store | 4 | `sp -0xac` | `?` |
| `ldr` at 0xbd920 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbd924 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xbd928 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbd92c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbd938 | store | 4 | `sp -0x20` | `ret(0x197fc)` |
| `ldr` at 0xbd93c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbd950 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xbd95c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbd960 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbd964 | load | 4 | `abs 0xbd9bc` | `-` |
| `ldr` at 0xbd968 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xbd96c | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xbd978 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbd984 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbd998 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `str` at 0xbd9a0 | store | 4 | `sp -0xac` | `?` |
| `ldr` at 0xbd9a8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbd9ac | store | 4 | `sp -0xac` | `[sp-32]` |
| `ldr` at 0xbd9b0 | load | 4 | `sp -0xac` | `-` |
| `ldm` at 0xbd9b8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbd9b8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbd9b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbd8f8 | `(unknown callee) 0x1937c` | `[[sp-20]]+1` | `sp-160` | `0x0` | `0x0` |
| `bl` at 0xbd908 | `(unknown callee) 0x193d0` | `ret(0x1937c)` | `?` | `?` | `ret(0x1937c)` |
| `bl` at 0xbd930 | `(unknown callee) 0x197fc` | `[[sp-20]]` | `[sp-24]` | `[sp-28]` | `[sp-20]` |
| `bl` at 0xbd948 | `(unknown callee) 0x193d0` | `ret(0x197fc)` | `?` | `?` | `ret(0x197fc)` |
| `bl` at 0xbd970 | `(unknown callee) 0x1e958` | `0x11f440` | `[[sp-20]+8]` | `[[sp-20]]` | `[sp-20]` |
| `bl` at 0xbd98c | `(unknown callee) 0xbdb28` | `[sp-20]` | `0x1` | `?` | `[sp-32]` |
| `bl` at 0xbd990 | `(unknown callee) 0x193d0` | `ret(0xbdb28)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0xbd82c | `r2 > 0x0 (signed, 32-bit)` | `r2 <= 0x0 (signed, 32-bit)` |
| `bhi` at 0xbd854 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bge` at 0xbd904 | `ret(0x1937c) >= 0x0 (signed, 32-bit)` | `ret(0x1937c) < 0x0 (signed, 32-bit)` |
| `bne` at 0xbd944 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xbd958 | `[ret(0x193d0)] != 0x23` | `[ret(0x193d0)] == 0x23` |
| `bge` at 0xbd980 | `[sp-32] >= 0x0 (signed, 32-bit)` | `[sp-32] < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbd9b8 | `r0` | `?` |

## `uartWrite`

- `uartWrite`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbd9c0 to 0xbda54.
  Established. The word through entry `r0 +0x4` is read (`ldr` at 0xbd9f8). The unnamed `0x192e0`
  call carries `[r0+4]`, entry `r1`, entry `r2`, entry `r0` (`bl` at 0xbda04); its nonnegative
  signed result takes `0xbda40`, while a negative result falls through (`bge` at 0xbda18). UART
  transmit effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbd9d0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbd9d4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xbd9d8 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xbd9dc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbd9ec | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xbd9f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbd9f8 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0xbd9fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbda00 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbda0c | store | 4 | `sp -0x20` | `ret(0x192e0)` |
| `ldr` at 0xbda10 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbda1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbda30 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `str` at 0xbda38 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xbda40 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbda44 | store | 4 | `sp -0x24` | `ret(0x192e0)` |
| `ldr` at 0xbda48 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xbda50 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbda50 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbda50 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbda04 | `(unknown callee) 0x192e0` | `[r0+4]` | `r1` | `r2` | `r0` |
| `bl` at 0xbda24 | `(unknown callee) 0xbdb28` | `[sp-20]` | `0x2` | `?` | `ret(0x192e0)` |
| `bl` at 0xbda28 | `(unknown callee) 0x193d0` | `ret(0xbdb28)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0xbd9e4 | `r2 > 0x0 (signed, 32-bit)` | `r2 <= 0x0 (signed, 32-bit)` |
| `bge` at 0xbda18 | `ret(0x192e0) >= 0x0 (signed, 32-bit)` | `ret(0x192e0) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbda50 | `r0` | `?` |

## `uartGetConfig`

- `uartGetConfig`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbda54 to 0xbda94.
  Established. Four word reads use entry `r0 +0x10`, `+0x14`, `+0x18` and `+0x1c` (`ldm` at 0xbda7c). The words through entry `r1 +0x0`, `+0x4`, `+0x8` and `+0xc` receive `[r0+16]`,
  `[r0+20]`, `[r0+24]` and `[r0+28]` respectively (`stm` at 0xbda80). Three further word reads use
  entry `r0 +0x20`, `+0x24` and `+0x28` (`ldm` at 0xbda84). Configuration field meanings remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbda64 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbda68 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xbda6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbda70 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xbda7c | load | 4 | `r0 +0x10` | `-` |
| `ldm` at 0xbda7c | load | 4 | `r0 +0x14` | `-` |
| `ldm` at 0xbda7c | load | 4 | `r0 +0x18` | `-` |
| `ldm` at 0xbda7c | load | 4 | `r0 +0x1c` | `-` |
| `stm` at 0xbda80 | store | 4 | `r1 +0x0` | `[r0+16]` |
| `stm` at 0xbda80 | store | 4 | `r1 +0x4` | `[r0+20]` |
| `stm` at 0xbda80 | store | 4 | `r1 +0x8` | `[r0+24]` |
| `stm` at 0xbda80 | store | 4 | `r1 +0xc` | `[r0+28]` |
| `ldm` at 0xbda84 | load | 4 | `r0 +0x20` | `-` |
| `ldm` at 0xbda84 | load | 4 | `r0 +0x24` | `-` |
| `ldm` at 0xbda84 | load | 4 | `r0 +0x28` | `-` |
| `stm` at 0xbda88 | store | 4 | `r1 +0x10` | `[r0+32]` |
| `stm` at 0xbda88 | store | 4 | `r1 +0x14` | `[r0+36]` |
| `stm` at 0xbda88 | store | 4 | `r1 +0x18` | `[r0+40]` |
| `ldm` at 0xbda90 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbda90 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbda90 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbda90 | `r0` | `[r0+32]` |

## `uartSetConfig`

- `uartSetConfig`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbda94 to 0xbdb28.
  Established. The byte through entry `r1 +0x16` is read (`ldrb` at 0xbdab0); its inequality to
  `0x1` takes `0xbdac8`, while equality falls through (`bne` at 0xbdab8). An included byte
  assignment at entry `r1 +0x16` writes `0x2` (`strb` at 0xbdac4). Configuration normalization
  meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbdaa4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbdaa8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xbdaac | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xbdab0 | load | 1 | `r1 +0x16` | `-` |
| `ldr` at 0xbdabc | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xbdac4 | store | 1 | `r1 +0x16` | `0x2` |
| `ldr` at 0xbdac8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xbdacc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbdad8 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xbdae0 | store | 1 | `(unknown)` | `0x2` |
| `ldr` at 0xbdae4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xbdae8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbdaf4 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xbdafc | store | 1 | `(unknown)` | `0x2` |
| `ldr` at 0xbdb00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbdb04 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xbdb10 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xbdb10 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xbdb10 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xbdb10 | load | 4 | `(unknown)` | `-` |
| `stm` at 0xbdb14 | store | 4 | `(unknown)` | `?` |
| `stm` at 0xbdb14 | store | 4 | `(unknown)` | `?` |
| `stm` at 0xbdb14 | store | 4 | `(unknown)` | `?` |
| `stm` at 0xbdb14 | store | 4 | `(unknown)` | `?` |
| `ldm` at 0xbdb18 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xbdb18 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xbdb18 | load | 4 | `(unknown)` | `-` |
| `stm` at 0xbdb1c | store | 4 | `(unknown)` | `?` |
| `stm` at 0xbdb1c | store | 4 | `(unknown)` | `?` |
| `stm` at 0xbdb1c | store | 4 | `(unknown)` | `?` |
| `ldm` at 0xbdb24 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbdb24 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbdb24 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xbdab8 | `[r1+22]:1 != 0x1` | `[r1+22]:1 == 0x1` |
| `bne` at 0xbdad4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xbdaf0 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbdb24 | `r0` | `?` |

## `uartFlush`

- `uartFlush`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbdb28 to 0xbdbd4.
  Established. Included word reads use entry `r0 +0x4` and `r0 +0x0` (`ldr` at 0xbdb54, `ldr` at 0xbdb64). The unnamed `0x19718` call carries `[sp-32]`, `0x80047502`, entry `r1`, `[sp-32]` (`bl` at 0xbdb80). A nonnegative signed result takes `0xbdbc0`, while a negative result falls through
  (`bge` at 0xbdb8c). Flush request behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbdb38 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbdb3c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xbdb40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbdb50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbdb54 | load | 4 | `r0 +0x4` | `-` |
| `str` at 0xbdb58 | store | 4 | `sp -0x20` | `[r0+4]` |
| `ldr` at 0xbdb60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbdb64 | load | 4 | `r0 +0x0` | `-` |
| `str` at 0xbdb68 | store | 4 | `sp -0x20` | `[r0]` |
| `ldr` at 0xbdb6c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbdb70 | store | 4 | `sp -0x1c` | `[sp-32]` |
| `ldr` at 0xbdb74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbdb78 | load | 4 | `abs 0xbdbc8` | `-` |
| `ldr` at 0xbdb7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbdb98 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xbdba4 | load | 4 | `abs 0xbdbcc` | `-` |
| `ldr` at 0xbdba8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbdbac | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0xbdbb0 | load | 4 | `abs 0xbdbd0` | `-` |
| `ldr` at 0xbdbb4 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldm` at 0xbdbc4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbdbc4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbdbc4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbdb80 | `(unknown callee) 0x19718` | `[sp-32]` | `0x80047502` | `r1` | `[sp-32]` |
| `bl` at 0xbdb90 | `(unknown callee) 0x193d0` | `ret(0x19718)` | `?` | `?` | `ret(0x19718)` |
| `bl` at 0xbdb9c | `(unknown callee) 0x1f8e0` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |
| `bl` at 0xbdbbc | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0x11f464` | `[[sp-20]+8]` | `ret(0x1f8e0)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xbdb4c | `? == 0x0` | `? != 0x0` |
| `bge` at 0xbdb8c | `ret(0x19718) >= 0x0 (signed, 32-bit)` | `ret(0x19718) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbdbc4 | `r0` | `?` |

## `uartSendBreak`

- `uartSendBreak`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbdbd4 to 0xbdc50.
  Established. The unnamed `0x19718` call carries `[r0+4]`, `0x80047504`, `0x0`, entry `r0` (`bl` at 0xbdbfc); its nonnegative signed result takes `0xbdc3c`, while a negative result falls through
  (`bge` at 0xbdc08). The absolute word at `0x12eeb8` is read on an included path (`ldr` at 0xbdc28). Break request effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbdbe4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbdbe8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xbdbec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbdbf0 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0xbdbf4 | load | 4 | `abs 0xbdc44` | `-` |
| `ldr` at 0xbdc14 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xbdc20 | load | 4 | `abs 0xbdc48` | `-` |
| `ldr` at 0xbdc24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbdc28 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0xbdc2c | load | 4 | `abs 0xbdc4c` | `-` |
| `ldr` at 0xbdc30 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldm` at 0xbdc40 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbdc40 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbdc40 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbdbfc | `(unknown callee) 0x19718` | `[r0+4]` | `0x80047504` | `0x0` | `r0` |
| `bl` at 0xbdc0c | `(unknown callee) 0x193d0` | `ret(0x19718)` | `?` | `?` | `ret(0x19718)` |
| `bl` at 0xbdc18 | `(unknown callee) 0x1f8e0` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |
| `bl` at 0xbdc38 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0x11f48c` | `[[sp-20]+8]` | `ret(0x1f8e0)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xbdc08 | `ret(0x19718) >= 0x0 (signed, 32-bit)` | `ret(0x19718) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbdc40 | `r0` | `?` |

## `uartShowStatus`

- `uartShowStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbdc50 to 0xbdd50.
  Established. The unnamed `0xbdd50` call carries entry `r0`, `r1`, `r2`, `r3` (`bl` at 0xbdc70);
  its nonzero result takes `0xbdc94`, while zero falls through (`bne` at 0xbdc84). Included halfword
  reads use `[sp-44] +0x9c`, `+0x9e`, `+0xa0` and `+0x104` (`ldrh` at 0xbdcec, `ldrh` at 0xbdcf8,
  `ldrh` at 0xbdd04, `ldrh` at 0xbdd1c). Status field interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbdc60 | store | 4 | `sp -0x20` | `r0` |
| `str` at 0xbdc64 | store | 4 | `sp -0x24` | `r1` |
| `str` at 0xbdc68 | store | 4 | `sp -0x28` | `r2` |
| `ldr` at 0xbdc6c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbdc78 | store | 4 | `sp -0x2c` | `ret(0xbdd50)` |
| `ldr` at 0xbdc7c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbdc8c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xbdc94 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbdca8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbdcac | load | 4 | `[sp-44] +0x98` | `-` |
| `ldr` at 0xbdcb8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbdccc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbdcd0 | load | 4 | `[sp-44] +0x100` | `-` |
| `ldr` at 0xbdcdc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbdce0 | store | 4 | `sp -0x50` | `ret(0xbde04)` |
| `str` at 0xbdce4 | store | 4 | `sp -0x4c` | `ret(0xbde28)` |
| `ldr` at 0xbdce8 | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0xbdcec | load | 2 | `[sp-44] +0x9c` | `-` |
| `str` at 0xbdcf0 | store | 4 | `sp -0x48` | `[[sp-44]+156]:2` |
| `ldr` at 0xbdcf4 | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0xbdcf8 | load | 2 | `[sp-44] +0x9e` | `-` |
| `str` at 0xbdcfc | store | 4 | `sp -0x44` | `[[sp-44]+158]:2` |
| `ldr` at 0xbdd00 | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0xbdd04 | load | 2 | `[sp-44] +0xa0` | `-` |
| `str` at 0xbdd08 | store | 4 | `sp -0x40` | `[[sp-44]+160]:2` |
| `str` at 0xbdd0c | store | 4 | `sp -0x3c` | `ret(0xbde04)` |
| `str` at 0xbdd10 | store | 4 | `sp -0x38` | `ret(0xbde28)` |
| `ldr` at 0xbdd14 | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0xbdd1c | load | 2 | `[sp-44] +0x104` | `-` |
| `str` at 0xbdd20 | store | 4 | `sp -0x34` | `[[sp-44]+260]:2` |
| `ldr` at 0xbdd24 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbdd28 | load | 4 | `abs 0xbdd4c` | `-` |
| `ldr` at 0xbdd2c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbdd30 | load | 4 | `[sp-44] +0x2c` | `-` |
| `str` at 0xbdd3c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xbdd40 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0xbdd48 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xbdd48 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xbdd48 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xbdd48 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbdd48 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbdd48 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbdc70 | `(unknown callee) 0xbdd50` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xbdca0 | `(unknown callee) 0xbde04` | `ret(0xbdd50)+60` | `?` | `?` | `ret(0xbdd50)+60` |
| `bl` at 0xbdcb0 | `(unknown callee) 0xbde28` | `[[sp-44]+152]` | `?` | `?` | `[sp-44]` |
| `bl` at 0xbdcc4 | `(unknown callee) 0xbde04` | `[sp-44]+164` | `?` | `?` | `[sp-44]+164` |
| `bl` at 0xbdcd4 | `(unknown callee) 0xbde28` | `[[sp-44]+256]` | `?` | `?` | `[sp-44]` |
| `bl` at 0xbdd34 | `(unknown callee) 0x197a8` | `[sp-40]` | `0x11f4b8` | `[sp-36]` | `[[sp-44]+44]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xbdc84 | `ret(0xbdd50) != 0x0` | `ret(0xbdd50) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbdd48 | `r0` | `[sp-48]` |

## `i2cWriteByte`

- `i2cWriteByte`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbb408 to 0xbb498.
  Established. The byte at `sp -0x13` receives `(r2 & 0xff)` (`strb` at 0xbb42c). A zero
  `ret(0xbaf50)` takes `0xbb45c`, while nonzero falls through (`beq` at 0xbb44c). The unnamed
  `0xbb2a8` call carries `[sp-18]:1`, `sp-19`, `0x1`, `[sp-18]:1` (`bl` at 0xbb470). Byte write
  policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xbb41c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xbb424 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0xbb42c | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `ldrb` at 0xbb430 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xbb440 | store | 4 | `sp -0x18` | `ret(0xbaf50)` |
| `ldr` at 0xbb444 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb450 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbb454 | store | 4 | `sp -0x1c` | `ret(0xbaf50)` |
| `ldrb` at 0xbb45c | load | 1 | `sp -0x12` | `-` |
| `str` at 0xbb478 | store | 4 | `sp -0x18` | `ret(0xbb2a8)` |
| `ldr` at 0xbb484 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbb488 | store | 4 | `sp -0x1c` | `[sp-24]` |
| `ldr` at 0xbb48c | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xbb494 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbb494 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbb494 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbb438 | `(unknown callee) 0xbaf50` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xbb470 | `(unknown callee) 0xbb2a8` | `[sp-18]:1` | `sp-19` | `0x1` | `[sp-18]:1` |
| `bl` at 0xbb480 | `(unknown callee) 0xbb094` | `0x0` | `?` | `?` | `ret(0xbb2a8)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xbb44c | `ret(0xbaf50) == 0x0` | `ret(0xbaf50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbb494 | `r0` | `?` |

## `i2cWriteByteOffset`

- `i2cWriteByteOffset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbb498 to 0xbb53c.
  Established. The byte at `sp -0x14` receives `(r3 & 0xff)` (`strb` at 0xbb4c8). The unnamed
  `0xbb2f8` call carries `[sp-18]:1`, `[sp-19]:1`, `sp-20`, `0x1` (`bl` at 0xbb514). The unnamed
  `0xbb094` call carries `0x0`, unknown, unknown, `ret(0xbb2f8)` (`bl` at 0xbb524). Offset write
  completion semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xbb4b0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xbb4b8 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0xbb4c0 | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `strb` at 0xbb4c8 | store | 1 | `sp -0x14` | `(r3 & 0xff)` |
| `ldrb` at 0xbb4cc | load | 1 | `sp -0x11` | `-` |
| `str` at 0xbb4dc | store | 4 | `sp -0x18` | `ret(0xbaf50)` |
| `ldr` at 0xbb4e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb4ec | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbb4f0 | store | 4 | `sp -0x1c` | `ret(0xbaf50)` |
| `ldrb` at 0xbb4f8 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0xbb4fc | load | 1 | `sp -0x13` | `-` |
| `str` at 0xbb51c | store | 4 | `sp -0x18` | `ret(0xbb2f8)` |
| `ldr` at 0xbb528 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbb52c | store | 4 | `sp -0x1c` | `[sp-24]` |
| `ldr` at 0xbb530 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xbb538 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbb538 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbb538 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbb4d4 | `(unknown callee) 0xbaf50` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xbb514 | `(unknown callee) 0xbb2f8` | `[sp-18]:1` | `[sp-19]:1` | `sp-20` | `0x1` |
| `bl` at 0xbb524 | `(unknown callee) 0xbb094` | `0x0` | `?` | `?` | `ret(0xbb2f8)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xbb4e8 | `ret(0xbaf50) == 0x0` | `ret(0xbaf50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbb538 | `r0` | `?` |

## `i2cReadByte`

- `i2cReadByte`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbb53c to 0xbb5c4.
  Established. The word at `sp -0x18` receives entry `r2` (`str` at 0xbb550). The unnamed `0xbb358`
  call carries `[sp-18]:1`, `[sp-24]`, `0x1`, `[sp-18]:1` (`bl` at 0xbb59c). The word at `sp -0x1c`
  receives `ret(0xbb358)` (`str` at 0xbb5a4). Byte read result interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbb550 | store | 4 | `sp -0x18` | `r2` |
| `strb` at 0xbb554 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xbb55c | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldrb` at 0xbb560 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xbb570 | store | 4 | `sp -0x1c` | `ret(0xbaf50)` |
| `ldr` at 0xbb574 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbb580 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbb584 | store | 4 | `sp -0x20` | `ret(0xbaf50)` |
| `ldrb` at 0xbb58c | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xbb594 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbb5a4 | store | 4 | `sp -0x1c` | `ret(0xbb358)` |
| `ldr` at 0xbb5b0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbb5b4 | store | 4 | `sp -0x20` | `[sp-28]` |
| `ldr` at 0xbb5b8 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xbb5c0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbb5c0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbb5c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbb568 | `(unknown callee) 0xbaf50` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xbb59c | `(unknown callee) 0xbb358` | `[sp-18]:1` | `[sp-24]` | `0x1` | `[sp-18]:1` |
| `bl` at 0xbb5ac | `(unknown callee) 0xbb094` | `0x0` | `?` | `?` | `ret(0xbb358)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xbb57c | `ret(0xbaf50) == 0x0` | `ret(0xbaf50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbb5c0 | `r0` | `?` |

## `i2cReadByteOffset`

- `i2cReadByteOffset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbb5c4 to 0xbb65c.
  Established. Entry `r3` is assigned to the word at `sp -0x18` (`str` at 0xbb5d4). A zero
  `ret(0xbaf50)` takes `0xbb61c`, while nonzero falls through (`beq` at 0xbb60c). The unnamed
  `0xbb3a8` call carries `[sp-18]:1`, `[sp-19]:1`, `[sp-24]`, `0x1` (`bl` at 0xbb634). Offset read
  completion policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbb5d4 | store | 4 | `sp -0x18` | `r3` |
| `strb` at 0xbb5dc | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xbb5e4 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0xbb5ec | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `ldrb` at 0xbb5f0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xbb600 | store | 4 | `sp -0x1c` | `ret(0xbaf50)` |
| `ldr` at 0xbb604 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbb610 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbb614 | store | 4 | `sp -0x20` | `ret(0xbaf50)` |
| `ldrb` at 0xbb61c | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0xbb620 | load | 1 | `sp -0x13` | `-` |
| `ldr` at 0xbb62c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbb63c | store | 4 | `sp -0x1c` | `ret(0xbb3a8)` |
| `ldr` at 0xbb648 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbb64c | store | 4 | `sp -0x20` | `[sp-28]` |
| `ldr` at 0xbb650 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xbb658 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbb658 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbb658 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbb5f8 | `(unknown callee) 0xbaf50` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xbb634 | `(unknown callee) 0xbb3a8` | `[sp-18]:1` | `[sp-19]:1` | `[sp-24]` | `0x1` |
| `bl` at 0xbb644 | `(unknown callee) 0xbb094` | `0x0` | `?` | `?` | `ret(0xbb3a8)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xbb60c | `ret(0xbaf50) == 0x0` | `ret(0xbaf50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbb658 | `r0` | `?` |

## `i2cInitialize`

- `i2cInitialize`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3a9c8 to 0x3aa28.
  Established. The unnamed `0xc84c4` call carries entry `r0`, `r1`, `r2`, `r3` (`bl` at 0x3a9d4). A
  zero `ret(0x3ab78)` takes `0x3a9f0`, while nonzero falls through (`beq` at 0x3a9e4). The unnamed
  `0x3f364` call carries `0x0` and three unknown candidates (`bl` at 0x3aa14). Peripheral
  initialization ordering effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldm` at 0x3aa24 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3aa24 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3aa24 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3a9d4 | `(unknown callee) 0xc84c4` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x3a9d8 | `(unknown callee) 0x3ab78` | `ret(0xc84c4)` | `?` | `?` | `?` |
| `bl` at 0x3a9ec | `(unknown callee) 0xc9e74` | `0x0` | `?` | `?` | `ret(0x3ab78)` |
| `bl` at 0x3a9f4 | `(unknown callee) 0xc5388` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x3a9fc | `(unknown callee) 0xbfb64` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x3aa04 | `(unknown callee) 0x362f8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x3aa0c | `(unknown callee) 0x24d04` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x3aa14 | `(unknown callee) 0x3f364` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x3aa1c | `(unknown callee) 0x3b75c` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x3aa20 | `(unknown callee) 0x22ce4` | `ret(0x3b75c)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3a9e4 | `ret(0x3ab78) == 0x0` | `ret(0x3ab78) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3aa24 | `r0` | `ret(0x22ce4)` |
