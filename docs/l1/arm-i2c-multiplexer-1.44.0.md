# ARM I2C multiplexer paths in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package4/i2c-multiplexer-facts.txt) retain all tool sections.

## `i2cMuxInit`

- `i2cMuxInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc7fdc to 0xc8208.
  Established. The absolute word at `0x297338` receives `0x1` (`str` at 0xc8060). The word at
  `0x297334` receives `ret(0x196d0)` (`str` at 0xc8094). The unnamed `0x19748` call carries
  `[[sp-40]+8]`, `0x1`, `[[sp-40]+4]`, `[sp-40]` (`bl` at 0xc81dc). Multiplexer table setup
  semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc7fec | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xc7ff0 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xc7ff4 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0xc7ff8 | load | 4 | `abs 0xc81e8` | `-` |
| `ldr` at 0xc7ffc | load | 4 | `abs 0x297334` | `-` |
| `ldr` at 0xc8008 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc8014 | load | 4 | `abs 0xc81ec` | `-` |
| `str` at 0xc8018 | store | 4 | `sp -0x38` | `0x120238` |
| `ldr` at 0xc801c | load | 4 | `abs 0xc81e8` | `-` |
| `ldr` at 0xc8020 | load | 4 | `abs 0x297334` | `-` |
| `str` at 0xc8024 | store | 4 | `sp -0x34` | `[0x297334]` |
| `ldr` at 0xc8028 | load | 4 | `abs 0xc81f0` | `-` |
| `ldr` at 0xc802c | load | 4 | `abs 0x297338` | `-` |
| `str` at 0xc8030 | store | 4 | `sp -0x30` | `[0x297338]` |
| `ldr` at 0xc8038 | load | 4 | `abs 0xc81f4` | `-` |
| `ldr` at 0xc8048 | load | 4 | `abs 0xc81f0` | `-` |
| `ldr` at 0xc804c | load | 4 | `abs 0x297338` | `-` |
| `ldr` at 0xc8058 | load | 4 | `abs 0xc81f0` | `-` |
| `str` at 0xc8060 | store | 4 | `abs 0x297338` | `0x1` |
| `ldr` at 0xc8064 | load | 4 | `abs 0xc81f0` | `-` |
| `ldr` at 0xc8068 | load | 4 | `abs 0x297338` | `-` |
| `str` at 0xc807c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc8080 | load | 4 | `abs 0xc81e8` | `-` |
| `ldr` at 0xc8084 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc8094 | store | 4 | `abs 0x297334` | `ret(0x196d0)` |
| `ldr` at 0xc8098 | load | 4 | `abs 0x297334` | `-` |
| `ldr` at 0xc80a4 | load | 4 | `abs 0xc81f8` | `-` |
| `str` at 0xc80a8 | store | 4 | `sp -0x38` | `0x120288` |
| `ldr` at 0xc80ac | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc80b0 | store | 4 | `sp -0x34` | `[sp-44]` |
| `ldr` at 0xc80b8 | load | 4 | `abs 0xc81f4` | `-` |
| `ldr` at 0xc80c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc80dc | load | 4 | `abs 0xc81e8` | `-` |
| `ldr` at 0xc80e0 | load | 4 | `abs 0x297334` | `-` |
| `str` at 0xc80e8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xc80ec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc80f0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc80f4 | store | 4 | `[sp-40] +0x0` | `?` |
| `ldr` at 0xc80f8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc80fc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc8100 | store | 4 | `[sp-40] +0x4` | `[sp-32]` |
| `ldr` at 0xc8104 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc8108 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc810c | load | 4 | `[sp-40] +0x4` | `-` |
| `str` at 0xc811c | store | 4 | `[sp-40] +0x8` | `ret(0x196d0)` |
| `ldr` at 0xc8120 | load | 4 | `[sp-40] +0x8` | `-` |
| `ldr` at 0xc812c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc8134 | load | 4 | `abs 0xc81fc` | `-` |
| `ldr` at 0xc8138 | load | 4 | `abs 0xc8200` | `-` |
| `str` at 0xc8174 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc8178 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc8180 | load | 4 | `abs 0xc81fc` | `-` |
| `ldr` at 0xc8184 | load | 4 | `abs 0xc8200` | `-` |
| `ldr` at 0xc8190 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc81a0 | load | 4 | `abs 0xc8204` | `-` |
| `str` at 0xc81a4 | store | 4 | `(unknown)` | `0x1202d0` |
| `str` at 0xc81a8 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0xc81ac | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc81b0 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0xc81b8 | load | 4 | `abs 0xc81f4` | `-` |
| `ldr` at 0xc81c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc81cc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc81d0 | load | 4 | `[sp-40] +0x8` | `-` |
| `ldr` at 0xc81d8 | load | 4 | `[sp-40] +0x4` | `-` |
| `ldm` at 0xc81e4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xc81e4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc81e4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc81e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc8044 | `(unknown callee) 0x200a0` | `0x9` | `0x12026c` | `0x54` | `0x0` |
| `bl` at 0xc808c | `(unknown callee) 0x196d0` | `[sp-44]` | `0x1` | `[0x297338]` | `?` |
| `bl` at 0xc80c4 | `(unknown callee) 0x200a0` | `0x9` | `0x12026c` | `0x60` | `0x0` |
| `bl` at 0xc8114 | `(unknown callee) 0x196d0` | `[[sp-40]+4]` | `0x1` | `[sp-40]` | `[sp-40]` |
| `bl` at 0xc813c | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x1202c4` | `0x1202cc` |
| `bl` at 0xc8148 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xc8188 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x1202c4` | `0x1202cc` |
| `bl` at 0xc8198 | `(unknown callee) 0x19808` | `[sp-44]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xc81c4 | `(unknown callee) 0x200a0` | `0x9` | `0x12026c` | `0x6b` | `0x0` |
| `bl` at 0xc81dc | `(unknown callee) 0x19748` | `[[sp-40]+8]` | `0x1` | `[[sp-40]+4]` | `[sp-40]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc8004 | `[0x297334] == 0x0` | `[0x297334] != 0x0` |
| `bne` at 0xc8010 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xc8054 | `[0x297338] != 0x0` | `[0x297338] == 0x0` |
| `bne` at 0xc80a0 | `[0x297334] != 0x0` | `[0x297334] == 0x0` |
| `bne` at 0xc8128 | `[[sp-40]+8] != 0x0` | `[[sp-40]+8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc81e4 | `r0` | `ret(0x19748)` |

## `i2cMuxNotPresent`

- `i2cMuxNotPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc8208 to 0xc8330.
  Established. The absolute word at `0x297334` is read (`ldr` at 0xc823c). The byte at `((r1 & 0xff) + [[sp-28]+8]) +0x0` receives zero on an included path (`strb` at 0xc8270). The signed condition
  `(r1 & 0xff) >= [[sp-28]+4]` takes `0xc8278`, while smaller values fall through (`bge` at 0xc8258). Presence removal policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc8218 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xc8220 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xc8224 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc8238 | load | 4 | `abs 0xc831c` | `-` |
| `ldr` at 0xc823c | load | 4 | `abs 0x297334` | `-` |
| `str` at 0xc8244 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xc8248 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc824c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc8250 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xc825c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc8260 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc8264 | load | 4 | `[sp-28] +0x8` | `-` |
| `strb` at 0xc8270 | store | 1 | `((r1 & 0xff) + [[sp-28]+8]) +0x0` | `0x0` |
| `ldr` at 0xc8278 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc8280 | load | 4 | `abs 0xc8320` | `-` |
| `ldr` at 0xc8284 | load | 4 | `abs 0xc8324` | `-` |
| `str` at 0xc82c0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xc82c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc82cc | load | 4 | `abs 0xc8320` | `-` |
| `ldr` at 0xc82d0 | load | 4 | `abs 0xc8324` | `-` |
| `ldr` at 0xc82dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc82ec | load | 4 | `abs 0xc8328` | `-` |
| `str` at 0xc82f0 | store | 4 | `(unknown)` | `0x120310` |
| `str` at 0xc82f4 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldrb` at 0xc82f8 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xc82fc | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0xc8304 | load | 4 | `abs 0xc832c` | `-` |
| `ldm` at 0xc8318 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc8318 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc8318 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc8288 | `(unknown callee) 0x48da8` | `r0` | `0x0` | `0x1202c4` | `0x1202cc` |
| `bl` at 0xc8294 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xc82d4 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0x1202c4` | `0x1202cc` |
| `bl` at 0xc82e4 | `(unknown callee) 0x19808` | `[sp-32]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xc8310 | `(unknown callee) 0x200a0` | `0x6` | `0x12026c` | `0x7a` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xc8258 | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc8318 | `r0` | `?` |

## `i2cMuxProbe`

- `i2cMuxProbe`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc8330 to 0xc84c4.
  Established. The unnamed `0xc8674` call carries entry `r0`, `(r1 & 0xff)`, `0x0`, `(r1 & 0xff)`
  (`bl` at 0xc8394). A zero result takes `0xc8490`, while nonzero falls through (`beq` at 0xc83a8).
  An included byte assignment at `([sp-29]:1 + [[sp-36]+8]) +0x0` writes zero (`strb` at 0xc8488).
  Probe result interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc8340 | store | 4 | `sp -0x1c` | `r0` |
| `strb` at 0xc8348 | store | 1 | `sp -0x1d` | `(r1 & 0xff)` |
| `ldr` at 0xc834c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc8360 | load | 4 | `abs 0xc84b0` | `-` |
| `ldr` at 0xc8364 | load | 4 | `abs 0x297334` | `-` |
| `str` at 0xc836c | store | 4 | `sp -0x24` | `?` |
| `ldrb` at 0xc8370 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xc8374 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc8378 | load | 4 | `[sp-36] +0x4` | `-` |
| `ldrb` at 0xc8384 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xc8388 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc839c | store | 4 | `sp -0x28` | `ret(0xc8674)` |
| `ldr` at 0xc83a0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc83ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc83b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc83c0 | load | 4 | `abs 0xc84b4` | `-` |
| `ldr` at 0xc83c4 | load | 4 | `abs 0xc84b8` | `-` |
| `str` at 0xc8400 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xc8404 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc840c | load | 4 | `abs 0xc84b4` | `-` |
| `ldr` at 0xc8410 | load | 4 | `abs 0xc84b8` | `-` |
| `ldr` at 0xc841c | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xc842c | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xc8430 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc8440 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc844c | load | 4 | `abs 0xc84bc` | `-` |
| `str` at 0xc8450 | store | 4 | `(unknown)` | `0x120338` |
| `str` at 0xc8454 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `str` at 0xc8458 | store | 4 | `(unknown)` | `ret(0xc885c)` |
| `str` at 0xc845c | store | 4 | `(unknown)` | `ret(0x1fc1c)` |
| `ldr` at 0xc8464 | load | 4 | `abs 0xc84c0` | `-` |
| `ldr` at 0xc8474 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc8478 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xc847c | load | 4 | `[sp-36] +0x8` | `-` |
| `strb` at 0xc8488 | store | 1 | `([sp-29]:1 + [[sp-36]+8]) +0x0` | `0x0` |
| `str` at 0xc8494 | store | 4 | `sp -0x34` | `0x1` |
| `str` at 0xc84a0 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xc84a4 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0xc84ac | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xc84ac | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xc84ac | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc84ac | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc84ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc8394 | `(unknown callee) 0xc8674` | `r0` | `(r1 & 0xff)` | `0x0` | `(r1 & 0xff)` |
| `bl` at 0xc83c8 | `(unknown callee) 0x48da8` | `[sp-28]` | `0x0` | `0x1202c4` | `0x1202cc` |
| `bl` at 0xc83d4 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xc8414 | `(unknown callee) 0x48da8` | `[sp-28]` | `0x0` | `0x1202c4` | `0x1202cc` |
| `bl` at 0xc8424 | `(unknown callee) 0x19808` | `[sp-48]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xc8438 | `(unknown callee) 0xc885c` | `[sp-28]` | `[sp-29]:1` | `?` | `[sp-29]:1` |
| `bl` at 0xc8444 | `(unknown callee) 0x1fc1c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xc8470 | `(unknown callee) 0x200a0` | `0x1` | `0x12026c` | `0x8c` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xc8380 | `(r1 & 0xff) >= [[sp-36]+4] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-36]+4] (signed, 32-bit)` |
| `beq` at 0xc83a8 | `ret(0xc8674) == 0x0` | `ret(0xc8674) != 0x0` |
| `beq` at 0xc83b4 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc84ac | `r0` | `?` |

## `i2cMuxReset`

- `i2cMuxReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc84c4 to 0xc8538.
  Established. The unnamed `0xbaf50` call carries `0x0`, entry `r1`, entry `r2`, entry `r3` (`bl` at 0xc84d8). A zero result takes `0xc8520`, while nonzero falls through (`beq` at 0xc84ec). The
  unnamed `0xc88c8` call carries `ret(0xbaf50)` and three unknown candidates (`bl` at 0xc8520).
  Multiplexer reset effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc84e0 | store | 4 | `sp -0x14` | `ret(0xbaf50)` |
| `ldr` at 0xc84e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc84f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc84fc | load | 4 | `abs 0xc8530` | `-` |
| `str` at 0xc8500 | store | 4 | `sp -0x1c` | `0x120364` |
| `str` at 0xc8504 | store | 4 | `sp -0x18` | `ret(0x1fc1c)` |
| `ldr` at 0xc850c | load | 4 | `abs 0xc8534` | `-` |
| `ldm` at 0xc852c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc852c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc852c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc84d8 | `(unknown callee) 0xbaf50` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0xc84f4 | `(unknown callee) 0x1fc1c` | `ret(0xbaf50)` | `?` | `?` | `ret(0xbaf50)` |
| `bl` at 0xc8518 | `(unknown callee) 0x200a0` | `0x1` | `0x12026c` | `0xa5` | `0x0` |
| `bl` at 0xc8520 | `(unknown callee) 0xc88c8` | `ret(0xbaf50)` | `?` | `?` | `ret(0xbaf50)` |
| `bl` at 0xc8524 | `(unknown callee) 0xbb094` | `ret(0xc88c8)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc84ec | `ret(0xbaf50) == 0x0` | `ret(0xbaf50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc852c | `r0` | `?` |

## `i2cMuxGetCfg`

- `i2cMuxGetCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc8538 to 0xc859c.
  Established. The word through entry `r1 +0x0` receives `[[sp-28]]` (`str` at 0xc8580). A word
  through `[sp-28] +0x4` is read (`ldr` at 0xc8588). The return reports `[[sp-28]+4]` (`ldm` at 0xc8594). Configuration field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc8548 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc854c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xc8550 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc8564 | load | 4 | `abs 0xc8598` | `-` |
| `ldr` at 0xc8568 | load | 4 | `abs 0x297334` | `-` |
| `str` at 0xc8570 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xc8574 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc8578 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc857c | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xc8580 | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0xc8584 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc8588 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldm` at 0xc8594 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc8594 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc8594 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc8594 | `r0` | `[[sp-28]+4]` |

## `i2cMuxCount`

- `i2cMuxCount`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc859c to 0xc85ec.
  Established. The absolute word at `0x297334` is read (`ldr` at 0xc85c8). A word through `[sp-24] +0x4` is read (`ldr` at 0xc85d8). The return reports `[[sp-24]+4]` (`ldm` at 0xc85e4). Count
  interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc85ac | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc85b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc85c4 | load | 4 | `abs 0xc85e8` | `-` |
| `ldr` at 0xc85c8 | load | 4 | `abs 0x297334` | `-` |
| `str` at 0xc85d0 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xc85d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc85d8 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldm` at 0xc85e4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc85e4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc85e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc85e4 | `r0` | `[[sp-24]+4]` |

## `i2cMuxIsPresent`

- `i2cMuxIsPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc85ec to 0xc8674.
  Established. The signed condition `(r1 & 0xff) < [[sp-28]+4]` takes `0xc864c`, while larger or
  equal values fall through (`blt` at 0xc863c). A byte at `((r1 & 0xff) + [[sp-28]+8]) +0x0` is read
  on an included path (`ldrb` at 0xc865c). The word at `sp -0x20` receives `[((r1 & 0xff) + [[sp-28]+8])]:1` (`str` at 0xc8660). Presence byte meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc85fc | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xc8604 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xc8608 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc861c | load | 4 | `abs 0xc8670` | `-` |
| `ldr` at 0xc8620 | load | 4 | `abs 0x297334` | `-` |
| `str` at 0xc8628 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xc862c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc8630 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc8634 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xc8644 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xc864c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc8650 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc8654 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0xc865c | load | 1 | `((r1 & 0xff) + [[sp-28]+8]) +0x0` | `-` |
| `str` at 0xc8660 | store | 4 | `sp -0x20` | `[((r1 & 0xff) + [[sp-28]+8])]:1` |
| `ldr` at 0xc8664 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xc866c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc866c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc866c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xc863c | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc866c | `r0` | `?` |

## `i2cMuxSelect`

- `i2cMuxSelect`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc8674 to 0xc87ac.
  Established. A word at `sp -0x28` receives `0xfffffc11` on an included path (`str` at 0xc86f4).
  The unnamed `0xc8ba4` call carries `[sp-20]`, `[sp-21]:1`, `[sp-22]:1`, `[sp-21]:1` (`bl` at 0xc8760). The unnamed `0xc8e0c` call carries the same printed candidates (`bl` at 0xc8780).
  Selection policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc8684 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xc868c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xc8694 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0xc8698 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc86ac | load | 4 | `abs 0xc87a4` | `-` |
| `ldr` at 0xc86b0 | load | 4 | `abs 0x297334` | `-` |
| `str` at 0xc86b8 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xc86bc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc86c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc86d0 | store | 4 | `sp -0x20` | `ret(0xc9318)` |
| `ldr` at 0xc86d4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc86d8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc86dc | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0xc86e4 | load | 1 | `([sp-21]:1 + [[sp-28]+8]) +0x0` | `-` |
| `ldr` at 0xc86f0 | load | 4 | `abs 0xc87a8` | `-` |
| `str` at 0xc86f4 | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0xc86fc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc8700 | load | 1 | `ret(0xc9318) +0x1` | `-` |
| `ldr` at 0xc8704 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc8724 | store | 4 | `sp -0x24` | `ret(0xbaf50)` |
| `ldr` at 0xc8728 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc8734 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc8738 | store | 4 | `sp -0x28` | `ret(0xbaf50)` |
| `ldr` at 0xc8740 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc8744 | load | 1 | `[sp-32] +0x1` | `-` |
| `ldrb` at 0xc8750 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xc8754 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xc8758 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc8768 | store | 4 | `sp -0x24` | `ret(0xc8ba4)` |
| `ldrb` at 0xc8770 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xc8774 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xc8778 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc8788 | store | 4 | `sp -0x24` | `ret(0xc8e0c)` |
| `ldr` at 0xc8790 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc8794 | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0xc8798 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xc87a0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc87a0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc87a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc86c8 | `(unknown callee) 0xc9318` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0xc870c | `(unknown callee) 0xbac78` | `[sp-20]` | `[ret(0xc9318)+1]:1` | `[sp-21]:1` | `[ret(0xc9318)+1]:1` |
| `bl` at 0xc871c | `(unknown callee) 0xbaf50` | `?` | `?` | `?` | `?` |
| `bl` at 0xc8760 | `(unknown callee) 0xc8ba4` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0xc8780 | `(unknown callee) 0xc8e0c` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0xc878c | `(unknown callee) 0xbb094` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc86ec | `[([sp-21]:1 + [[sp-28]+8])]:1 != 0x0` | `[([sp-21]:1 + [[sp-28]+8])]:1 == 0x0` |
| `beq` at 0xc8730 | `ret(0xbaf50) == 0x0` | `ret(0xbaf50) != 0x0` |
| `bne` at 0xc874c | `[[sp-32]+1]:1 != 0x0` | `[[sp-32]+1]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc87a0 | `r0` | `?` |

## `i2cMuxRead`

- `i2cMuxRead`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc87ac to 0xc885c.
  Established. The word at `sp -0x28` receives `0xfffffc11` on an included path (`str` at 0xc8828).
  The unnamed `0xc959c` call carries `[sp-20]`, `ret(0xc9318)`, `[sp-28]`, `[([sp-21]:1 + [[sp-32]+8])]:1` (`bl` at 0xc883c). The return reports an unknown value (`ldm` at 0xc8850).
  Multiplexer read behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc87bc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc87c4 | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0xc87c8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xc87cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc87e0 | load | 4 | `abs 0xc8854` | `-` |
| `ldr` at 0xc87e4 | load | 4 | `abs 0x297334` | `-` |
| `str` at 0xc87ec | store | 4 | `sp -0x20` | `?` |
| `ldrb` at 0xc87f0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc87f4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc8804 | store | 4 | `sp -0x24` | `ret(0xc9318)` |
| `ldr` at 0xc8808 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc880c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc8810 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldrb` at 0xc8818 | load | 1 | `([sp-21]:1 + [[sp-32]+8]) +0x0` | `-` |
| `ldr` at 0xc8824 | load | 4 | `abs 0xc8858` | `-` |
| `str` at 0xc8828 | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0xc8830 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc8834 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc8838 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc8844 | store | 4 | `sp -0x28` | `ret(0xc959c)` |
| `ldr` at 0xc8848 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xc8850 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc8850 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc8850 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc87fc | `(unknown callee) 0xc9318` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0xc883c | `(unknown callee) 0xc959c` | `[sp-20]` | `ret(0xc9318)` | `[sp-28]` | `[([sp-21]:1 + [[sp-32]+8])]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc8820 | `[([sp-21]:1 + [[sp-32]+8])]:1 != 0x0` | `[([sp-21]:1 + [[sp-32]+8])]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc8850 | `r0` | `?` |

## `i2cMuxName`

- `i2cMuxName`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc885c to 0xc88c8.
  Established. The unnamed `0xc9318` call carries entry `r0`, `(r1 & 0xff)`, entry `r2`, `(r1 & 0xff)` (`bl` at 0xc8884). A nonzero `[ret(0xc9318)+4]` takes `0xc88b4`, while zero falls through
  (`bne` at 0xc88a8). The word at `sp -0x20` receives `0x120384` on an included path (`str` at 0xc88b0). Label pointer interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc886c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xc8874 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xc8878 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc887c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc888c | store | 4 | `sp -0x1c` | `ret(0xc9318)` |
| `ldr` at 0xc8890 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc8894 | load | 4 | `ret(0xc9318) +0x4` | `-` |
| `str` at 0xc8898 | store | 4 | `sp -0x20` | `[ret(0xc9318)+4]` |
| `ldr` at 0xc889c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc88a0 | load | 4 | `ret(0xc9318) +0x4` | `-` |
| `ldr` at 0xc88ac | load | 4 | `abs 0xc88c4` | `-` |
| `str` at 0xc88b0 | store | 4 | `sp -0x20` | `0x120384` |
| `ldr` at 0xc88b4 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xc88c0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc88c0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc88c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc8884 | `(unknown callee) 0xc9318` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc88a8 | `[ret(0xc9318)+4] != 0x0` | `[ret(0xc9318)+4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc88c0 | `r0` | `?` |

## `i2cMuxResetRaw`

- `i2cMuxResetRaw`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc88c8 to 0xc8ba4.
  Established. Included absolute byte assignments at `0x297330` and `0x297331` write zero (`strb` at
  0xc89b0, `strb` at 0xc8b00). Absolute byte assignments at `0x12da24`, `0x12da25`, `0x12da26` and
  `0x12da27` have unknown values (`strb` at 0xc89bc, `strb` at 0xc89c8, `strb` at 0xc8b0c, `strb` at
  0xc8b18). The unnamed `0xc9008` call at `0xc8b44` carries `[0x297330]:1`, `[0x12da24]:1`,
  `[0x297330]:1`, `[0x12da24]:1` (`bl` at 0xc8b44). Raw reset policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xc88d8 | load | 4 | `abs 0xc8b84` | `-` |
| `ldr` at 0xc88dc | load | 4 | `abs 0x297334` | `-` |
| `str` at 0xc88e0 | store | 4 | `sp -0x14` | `[0x297334]` |
| `str` at 0xc88e8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xc88ec | load | 4 | `abs 0xc8b88` | `-` |
| `ldr` at 0xc88f0 | load | 4 | `abs 0x297338` | `-` |
| `str` at 0xc8900 | store | 4 | `sp -0x24` | `0x0` |
| `strb` at 0xc890c | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0xc8910 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xc8914 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc8918 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc8924 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc8928 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xc892c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xc8934 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc8940 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc8944 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xc894c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xc8954 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xc8960 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0xc8974 | store | 4 | `sp -0x1c` | `ret(0xc9008)` |
| `ldr` at 0xc8978 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc8984 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc8990 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc8994 | store | 4 | `sp -0x18` | `ret(0xc9008)` |
| `ldrb` at 0xc8998 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0xc89a0 | store | 1 | `sp -0x1d` | `?` |
| `ldr` at 0xc89a8 | load | 4 | `abs 0xc8b8c` | `-` |
| `strb` at 0xc89b0 | store | 1 | `abs 0x297330` | `0x0` |
| `ldr` at 0xc89b4 | load | 4 | `abs 0xc8b90` | `-` |
| `strb` at 0xc89bc | store | 1 | `abs 0x12da24` | `?` |
| `ldr` at 0xc89c0 | load | 4 | `abs 0xc8b94` | `-` |
| `strb` at 0xc89c8 | store | 1 | `abs 0x12da25` | `?` |
| `strb` at 0xc89d0 | store | 1 | `sp -0x1e` | `0x0` |
| `ldrb` at 0xc89d4 | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0xc89d8 | load | 4 | `abs 0xc8b88` | `-` |
| `ldr` at 0xc89dc | load | 4 | `abs 0x297338` | `-` |
| `strb` at 0xc89ec | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0xc89f0 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xc89f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc89f8 | load | 4 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xc8a04 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc8a08 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xc8a0c | load | 4 | `[sp-20] +0x8` | `-` |
| `ldrb` at 0xc8a14 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc8a20 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc8a24 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xc8a2c | load | 4 | `[sp-20] +0x0` | `-` |
| `ldrb` at 0xc8a34 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xc8a40 | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0xc8a44 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc8a48 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xc8a50 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldrb` at 0xc8a58 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc8a78 | store | 4 | `sp -0x1c` | `ret(0xbb18c)` |
| `ldr` at 0xc8a7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc8a88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc8a94 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc8a98 | store | 4 | `sp -0x18` | `ret(0xbb18c)` |
| `ldrb` at 0xc8a9c | load | 1 | `sp -0x1e` | `-` |
| `ldrb` at 0xc8aa0 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0xc8ab4 | store | 4 | `sp -0x1c` | `ret(0xc9008)` |
| `ldr` at 0xc8ab8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc8ac4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc8ad0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc8ad4 | store | 4 | `sp -0x18` | `ret(0xc9008)` |
| `ldrb` at 0xc8ad8 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0xc8ae0 | store | 1 | `sp -0x1d` | `?` |
| `ldrb` at 0xc8ae8 | load | 1 | `sp -0x1e` | `-` |
| `strb` at 0xc8af0 | store | 1 | `sp -0x1e` | `?` |
| `ldr` at 0xc8af8 | load | 4 | `abs 0xc8b98` | `-` |
| `strb` at 0xc8b00 | store | 1 | `abs 0x297331` | `0x0` |
| `ldr` at 0xc8b04 | load | 4 | `abs 0xc8b9c` | `-` |
| `strb` at 0xc8b0c | store | 1 | `abs 0x12da26` | `?` |
| `ldr` at 0xc8b10 | load | 4 | `abs 0xc8ba0` | `-` |
| `strb` at 0xc8b18 | store | 1 | `abs 0x12da27` | `?` |
| `ldr` at 0xc8b1c | load | 4 | `abs 0xc8b90` | `-` |
| `ldrb` at 0xc8b20 | load | 1 | `abs 0x12da24` | `-` |
| `ldr` at 0xc8b2c | load | 4 | `abs 0xc8b8c` | `-` |
| `ldrb` at 0xc8b30 | load | 1 | `abs 0x297330` | `-` |
| `ldr` at 0xc8b34 | load | 4 | `abs 0xc8b90` | `-` |
| `ldrb` at 0xc8b38 | load | 1 | `abs 0x12da24` | `-` |
| `str` at 0xc8b4c | store | 4 | `sp -0x1c` | `ret(0xc9008)` |
| `ldr` at 0xc8b50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc8b5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc8b68 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc8b6c | store | 4 | `sp -0x18` | `ret(0xc9008)` |
| `ldr` at 0xc8b70 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc8b74 | store | 4 | `sp -0x24` | `[sp-24]` |
| `ldr` at 0xc8b78 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xc8b80 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc8b80 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc8b80 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc896c | `(unknown callee) 0xc9008` | `0x0` | `?` | `?` | `?` |
| `bl` at 0xc8a60 | `(unknown callee) 0xbac78` | `?` | `?` | `?` | `?` |
| `bl` at 0xc8a70 | `(unknown callee) 0xbb18c` | `?` | `?` | `?` | `?` |
| `bl` at 0xc8aac | `(unknown callee) 0xc9008` | `[sp-30]:1` | `[sp-29]:1` | `[sp-29]:1` | `[sp-30]:1` |
| `bl` at 0xc8b44 | `(unknown callee) 0xc9008` | `[0x297330]:1` | `[0x12da24]:1` | `[0x297330]:1` | `[0x12da24]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc88f8 | `[0x297338] != 0x0` | `[0x297338] == 0x0` |
| `bge` at 0xc8920 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xc893c | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xc895c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xc8980 | `ret(0xc9008) == 0x0` | `ret(0xc9008) != 0x0` |
| `bne` at 0xc898c | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bge` at 0xc89e4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bge` at 0xc8a00 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xc8a1c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc8a3c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc8a84 | `ret(0xbb18c) == 0x0` | `ret(0xbb18c) != 0x0` |
| `bne` at 0xc8a90 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0xc8ac0 | `ret(0xc9008) == 0x0` | `ret(0xc9008) != 0x0` |
| `bne` at 0xc8acc | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0xc8b28 | `[0x12da24]:1 == 0xfe` | `[0x12da24]:1 != 0xfe` |
| `beq` at 0xc8b58 | `ret(0xc9008) == 0x0` | `ret(0xc9008) != 0x0` |
| `bne` at 0xc8b64 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc8b80 | `r0` | `?` |

## `i2cMuxSelectRaw`

- `i2cMuxSelectRaw`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc8ba4 to 0xc8e0c.
  Established. An included byte assignment at `0x297331` writes zero (`strb` at 0xc8cb8). A later
  byte assignment at `0x297330` writes `([sp-24] & 0xff)` (`strb` at 0xc8db0). The unnamed `0xc943c`
  call carries `[sp-36]`, `[sp-26]:1`, `0x12da27`, `[sp-26]:1` (`bl` at 0xc8ce0). Raw selection
  state meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc8bb4 | store | 4 | `sp -0x18` | `r0` |
| `strb` at 0xc8bbc | store | 1 | `sp -0x19` | `(r1 & 0xff)` |
| `strb` at 0xc8bc4 | store | 1 | `sp -0x1a` | `(r2 & 0xff)` |
| `ldr` at 0xc8bc8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc8bdc | load | 4 | `abs 0xc8de0` | `-` |
| `ldr` at 0xc8be0 | load | 4 | `abs 0x297334` | `-` |
| `str` at 0xc8be8 | store | 4 | `sp -0x20` | `?` |
| `ldrb` at 0xc8bec | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xc8bf0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc8c00 | store | 4 | `sp -0x24` | `ret(0xc9318)` |
| `ldr` at 0xc8c04 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc8c08 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xc8c0c | load | 4 | `[sp-32] +0x8` | `-` |
| `ldrb` at 0xc8c14 | load | 1 | `([sp-25]:1 + [[sp-32]+8]) +0x0` | `-` |
| `ldr` at 0xc8c20 | load | 4 | `abs 0xc8de4` | `-` |
| `str` at 0xc8c24 | store | 4 | `sp -0x30` | `0xfffffc11` |
| `ldr` at 0xc8c2c | load | 4 | `abs 0xc8de8` | `-` |
| `ldrb` at 0xc8c30 | load | 1 | `abs 0x297330` | `-` |
| `ldr` at 0xc8c34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc8c40 | load | 4 | `abs 0xc8dec` | `-` |
| `ldrb` at 0xc8c44 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0xc8c48 | load | 1 | `abs 0x12da24` | `-` |
| `ldr` at 0xc8c54 | load | 4 | `abs 0xc8df0` | `-` |
| `ldrb` at 0xc8c58 | load | 1 | `sp -0x1a` | `-` |
| `ldrb` at 0xc8c5c | load | 1 | `abs 0x12da25` | `-` |
| `str` at 0xc8c6c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xc8c74 | load | 4 | `abs 0xc8dec` | `-` |
| `ldrb` at 0xc8c78 | load | 1 | `abs 0x12da24` | `-` |
| `ldr` at 0xc8c84 | load | 4 | `abs 0xc8df0` | `-` |
| `ldrb` at 0xc8c88 | load | 1 | `abs 0x12da25` | `-` |
| `ldr` at 0xc8c94 | load | 4 | `abs 0xc8de8` | `-` |
| `ldrb` at 0xc8c98 | load | 1 | `abs 0x297330` | `-` |
| `ldr` at 0xc8c9c | load | 4 | `abs 0xc8dec` | `-` |
| `ldrb` at 0xc8ca0 | load | 1 | `abs 0x12da24` | `-` |
| `ldr` at 0xc8cb0 | load | 4 | `abs 0xc8df4` | `-` |
| `strb` at 0xc8cb8 | store | 1 | `abs 0x297331` | `0x0` |
| `ldr` at 0xc8cbc | load | 4 | `abs 0xc8df8` | `-` |
| `strb` at 0xc8cc4 | store | 1 | `abs 0x12da26` | `?` |
| `ldr` at 0xc8cc8 | load | 4 | `abs 0xc8dfc` | `-` |
| `strb` at 0xc8cd0 | store | 1 | `abs 0x12da27` | `?` |
| `ldrb` at 0xc8cd4 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xc8cd8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc8ce8 | store | 4 | `sp -0x28` | `ret(0xc943c)` |
| `ldr` at 0xc8cec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc8cf8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc8d00 | load | 4 | `abs 0xc8e00` | `-` |
| `ldr` at 0xc8d04 | load | 4 | `abs 0xc8e04` | `-` |
| `str` at 0xc8d40 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc8d44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc8d4c | load | 4 | `abs 0xc8e00` | `-` |
| `ldr` at 0xc8d50 | load | 4 | `abs 0xc8e04` | `-` |
| `ldr` at 0xc8d5c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc8d6c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc8d78 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc8d7c | load | 1 | `[sp-36] +0x2` | `-` |
| `ldrb` at 0xc8d80 | load | 1 | `sp -0x1a` | `-` |
| `str` at 0xc8d84 | store | 4 | `(unknown)` | `ret(0x1f8e0)` |
| `ldr` at 0xc8d88 | load | 4 | `abs 0xc8e08` | `-` |
| `ldr` at 0xc8d9c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc8da0 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xc8da8 | load | 4 | `abs 0xc8de8` | `-` |
| `ldr` at 0xc8dac | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xc8db0 | store | 1 | `abs 0x297330` | `([sp-24] & 0xff)` |
| `ldr` at 0xc8db4 | load | 4 | `abs 0xc8dec` | `-` |
| `ldrb` at 0xc8db8 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xc8dbc | store | 1 | `abs 0x12da24` | `([sp-25]:1 & 0xff)` |
| `ldr` at 0xc8dc0 | load | 4 | `abs 0xc8df0` | `-` |
| `ldrb` at 0xc8dc4 | load | 1 | `sp -0x1a` | `-` |
| `strb` at 0xc8dc8 | store | 1 | `abs 0x12da25` | `([sp-26]:1 & 0xff)` |
| `str` at 0xc8dd0 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xc8dd4 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0xc8ddc | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xc8ddc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc8ddc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc8ddc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc8bf8 | `(unknown callee) 0xc9318` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0xc8cac | `(unknown callee) 0xc9008` | `[0x297330]:1` | `[0x12da24]:1` | `[0x297330]:1` | `[0x12da24]:1` |
| `bl` at 0xc8ce0 | `(unknown callee) 0xc943c` | `[sp-36]` | `[sp-26]:1` | `0x12da27` | `[sp-26]:1` |
| `bl` at 0xc8d08 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x1202c4` | `0x1202cc` |
| `bl` at 0xc8d14 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xc8d54 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x1202c4` | `0x1202cc` |
| `bl` at 0xc8d64 | `(unknown callee) 0x19808` | `[sp-44]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xc8d70 | `(unknown callee) 0x1f8e0` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xc8d98 | `(unknown callee) 0x1e958` | `0x12038c` | `ret(0x19808)` | `[[sp-36]+2]:1` | `[sp-26]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc8c1c | `[([sp-25]:1 + [[sp-32]+8])]:1 != 0x0` | `[([sp-25]:1 + [[sp-32]+8])]:1 == 0x0` |
| `bne` at 0xc8c3c | `[0x297330]:1 != [sp-24]` | `[0x297330]:1 == [sp-24]` |
| `bne` at 0xc8c50 | `[sp-25]:1 != [0x12da24]:1` | `[sp-25]:1 == [0x12da24]:1` |
| `bne` at 0xc8c64 | `[sp-26]:1 != [0x12da25]:1` | `[sp-26]:1 == [0x12da25]:1` |
| `beq` at 0xc8c80 | `[0x12da24]:1 == 0xfe` | `[0x12da24]:1 != 0xfe` |
| `beq` at 0xc8c90 | `[0x12da25]:1 == 0x0` | `[0x12da25]:1 != 0x0` |
| `beq` at 0xc8cf4 | `ret(0xc943c) == 0x0` | `ret(0xc943c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc8ddc | `r0` | `?` |

## `i2cMuxSelectSubRaw`

- `i2cMuxSelectSubRaw`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc8e0c to 0xc9008.
  Established. Included byte assignments at `0x297331`, `0x12da26` and `0x12da27` write `([sp-24] & 0xff)`, `([sp-25]:1 & 0xff)` and `([sp-26]:1 & 0xff)` respectively (`strb` at 0xc8fb8, `strb` at
  0xc8fc4, `strb` at 0xc8fd0). The unnamed `0xc943c` call carries `ret(0xc9318)`, `[sp-26]:1`,
  unknown, `[sp-26]:1` (`bl` at 0xc8ee8). A zero result takes `0xc8fb0`, while nonzero falls through
  (`beq` at 0xc8efc). Subselection effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc8e1c | store | 4 | `sp -0x18` | `r0` |
| `strb` at 0xc8e24 | store | 1 | `sp -0x19` | `(r1 & 0xff)` |
| `strb` at 0xc8e2c | store | 1 | `sp -0x1a` | `(r2 & 0xff)` |
| `ldr` at 0xc8e30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc8e44 | load | 4 | `abs 0xc8fe8` | `-` |
| `ldr` at 0xc8e48 | load | 4 | `abs 0x297334` | `-` |
| `str` at 0xc8e50 | store | 4 | `sp -0x20` | `?` |
| `ldrb` at 0xc8e54 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xc8e58 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc8e68 | store | 4 | `sp -0x24` | `ret(0xc9318)` |
| `ldr` at 0xc8e6c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc8e70 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xc8e74 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldrb` at 0xc8e7c | load | 1 | `([sp-25]:1 + [[sp-32]+8]) +0x0` | `-` |
| `ldr` at 0xc8e88 | load | 4 | `abs 0xc8fec` | `-` |
| `str` at 0xc8e8c | store | 4 | `sp -0x30` | `0xfffffc11` |
| `ldr` at 0xc8e94 | load | 4 | `abs 0xc8ff0` | `-` |
| `ldrb` at 0xc8e98 | load | 1 | `abs 0x297331` | `-` |
| `ldr` at 0xc8e9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc8ea8 | load | 4 | `abs 0xc8ff4` | `-` |
| `ldrb` at 0xc8eac | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0xc8eb0 | load | 1 | `abs 0x12da26` | `-` |
| `ldr` at 0xc8ebc | load | 4 | `abs 0xc8ff8` | `-` |
| `ldrb` at 0xc8ec0 | load | 1 | `sp -0x1a` | `-` |
| `ldrb` at 0xc8ec4 | load | 1 | `abs 0x12da27` | `-` |
| `str` at 0xc8ed4 | store | 4 | `sp -0x30` | `0x0` |
| `ldrb` at 0xc8edc | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xc8ee0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc8ef0 | store | 4 | `sp -0x28` | `ret(0xc943c)` |
| `ldr` at 0xc8ef4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc8f00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc8f08 | load | 4 | `abs 0xc8ffc` | `-` |
| `ldr` at 0xc8f0c | load | 4 | `abs 0xc9000` | `-` |
| `str` at 0xc8f48 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc8f4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc8f54 | load | 4 | `abs 0xc8ffc` | `-` |
| `ldr` at 0xc8f58 | load | 4 | `abs 0xc9000` | `-` |
| `ldr` at 0xc8f64 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc8f74 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc8f80 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc8f84 | load | 1 | `[sp-36] +0x2` | `-` |
| `ldrb` at 0xc8f88 | load | 1 | `sp -0x1a` | `-` |
| `str` at 0xc8f8c | store | 4 | `(unknown)` | `ret(0x1f8e0)` |
| `ldr` at 0xc8f90 | load | 4 | `abs 0xc9004` | `-` |
| `ldr` at 0xc8fa4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc8fa8 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xc8fb0 | load | 4 | `abs 0xc8ff0` | `-` |
| `ldr` at 0xc8fb4 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xc8fb8 | store | 1 | `abs 0x297331` | `([sp-24] & 0xff)` |
| `ldr` at 0xc8fbc | load | 4 | `abs 0xc8ff4` | `-` |
| `ldrb` at 0xc8fc0 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xc8fc4 | store | 1 | `abs 0x12da26` | `([sp-25]:1 & 0xff)` |
| `ldr` at 0xc8fc8 | load | 4 | `abs 0xc8ff8` | `-` |
| `ldrb` at 0xc8fcc | load | 1 | `sp -0x1a` | `-` |
| `strb` at 0xc8fd0 | store | 1 | `abs 0x12da27` | `([sp-26]:1 & 0xff)` |
| `str` at 0xc8fd8 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xc8fdc | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0xc8fe4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xc8fe4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc8fe4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc8fe4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc8e60 | `(unknown callee) 0xc9318` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0xc8ee8 | `(unknown callee) 0xc943c` | `ret(0xc9318)` | `[sp-26]:1` | `?` | `[sp-26]:1` |
| `bl` at 0xc8f10 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x1202c4` | `0x1202cc` |
| `bl` at 0xc8f1c | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xc8f5c | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x1202c4` | `0x1202cc` |
| `bl` at 0xc8f6c | `(unknown callee) 0x19808` | `[sp-44]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xc8f78 | `(unknown callee) 0x1f8e0` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xc8fa0 | `(unknown callee) 0x1e958` | `0x1203c0` | `ret(0x19808)` | `[[sp-36]+2]:1` | `[sp-26]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc8e84 | `[([sp-25]:1 + [[sp-32]+8])]:1 != 0x0` | `[([sp-25]:1 + [[sp-32]+8])]:1 == 0x0` |
| `bne` at 0xc8ea4 | `[0x297331]:1 != [sp-24]` | `[0x297331]:1 == [sp-24]` |
| `bne` at 0xc8eb8 | `[sp-25]:1 != [0x12da26]:1` | `[sp-25]:1 == [0x12da26]:1` |
| `bne` at 0xc8ecc | `[sp-26]:1 != [0x12da27]:1` | `[sp-26]:1 == [0x12da27]:1` |
| `beq` at 0xc8efc | `ret(0xc943c) == 0x0` | `ret(0xc943c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc8fe4 | `r0` | `?` |

## `i2cMuxDeselectRaw`

- `i2cMuxDeselectRaw`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc9008 to 0xc9238.
  Established. A zero `ret(0xc943c)` takes `0xc9170`, while nonzero falls through (`beq` at 0xc90a4). Included byte assignments at `0x297330` and `0x297331` write zero (`strb` at 0xc91b0,
  `strb` at 0xc91d4). Byte assignments at `0x12da24`, `0x12da25`, `0x12da26` and `0x12da27` have
  unknown values (`strb` at 0xc91bc, `strb` at 0xc91c8, `strb` at 0xc91e0, `strb` at 0xc91ec).
  Deselection state semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc9018 | store | 4 | `sp -0x1c` | `r0` |
| `strb` at 0xc9020 | store | 1 | `sp -0x1d` | `(r1 & 0xff)` |
| `ldr` at 0xc9024 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc9038 | load | 4 | `abs 0xc9204` | `-` |
| `ldr` at 0xc903c | load | 4 | `abs 0x297334` | `-` |
| `str` at 0xc9044 | store | 4 | `sp -0x24` | `?` |
| `ldrb` at 0xc9048 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xc904c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc905c | store | 4 | `sp -0x28` | `ret(0xc9318)` |
| `ldr` at 0xc9060 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc9064 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xc9068 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldrb` at 0xc9070 | load | 1 | `([sp-29]:1 + [[sp-36]+8]) +0x0` | `-` |
| `ldr` at 0xc907c | load | 4 | `abs 0xc9208` | `-` |
| `str` at 0xc9080 | store | 4 | `sp -0x34` | `0xfffffc11` |
| `ldr` at 0xc9088 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc9098 | store | 4 | `sp -0x2c` | `ret(0xc943c)` |
| `ldr` at 0xc909c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc90a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc90b0 | load | 4 | `abs 0xc920c` | `-` |
| `ldr` at 0xc90b4 | load | 4 | `abs 0xc9210` | `-` |
| `str` at 0xc90f0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xc90f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc90fc | load | 4 | `abs 0xc920c` | `-` |
| `ldr` at 0xc9100 | load | 4 | `abs 0xc9210` | `-` |
| `ldr` at 0xc910c | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xc911c | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xc9120 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc9130 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc913c | load | 4 | `abs 0xc9214` | `-` |
| `str` at 0xc9140 | store | 4 | `(unknown)` | `0x1203f8` |
| `str` at 0xc9144 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `str` at 0xc9148 | store | 4 | `(unknown)` | `ret(0xc885c)` |
| `str` at 0xc914c | store | 4 | `(unknown)` | `ret(0x1f8e0)` |
| `ldr` at 0xc9154 | load | 4 | `abs 0xc9218` | `-` |
| `ldr` at 0xc9158 | load | 4 | `abs 0xc921c` | `-` |
| `ldr` at 0xc9164 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc9168 | store | 4 | `sp -0x34` | `[sp-44]` |
| `ldr` at 0xc9170 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xc9174 | load | 1 | `[sp-40] +0x1` | `-` |
| `ldr` at 0xc9180 | load | 4 | `abs 0xc9220` | `-` |
| `ldrb` at 0xc9184 | load | 1 | `abs 0x297330` | `-` |
| `ldr` at 0xc9188 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc9194 | load | 4 | `abs 0xc9224` | `-` |
| `ldrb` at 0xc9198 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0xc919c | load | 1 | `abs 0x12da24` | `-` |
| `ldr` at 0xc91a8 | load | 4 | `abs 0xc9220` | `-` |
| `strb` at 0xc91b0 | store | 1 | `abs 0x297330` | `0x0` |
| `ldr` at 0xc91b4 | load | 4 | `abs 0xc9224` | `-` |
| `strb` at 0xc91bc | store | 1 | `abs 0x12da24` | `?` |
| `ldr` at 0xc91c0 | load | 4 | `abs 0xc9228` | `-` |
| `strb` at 0xc91c8 | store | 1 | `abs 0x12da25` | `?` |
| `ldr` at 0xc91cc | load | 4 | `abs 0xc922c` | `-` |
| `strb` at 0xc91d4 | store | 1 | `abs 0x297331` | `0x0` |
| `ldr` at 0xc91d8 | load | 4 | `abs 0xc9230` | `-` |
| `strb` at 0xc91e0 | store | 1 | `abs 0x12da26` | `?` |
| `ldr` at 0xc91e4 | load | 4 | `abs 0xc9234` | `-` |
| `strb` at 0xc91ec | store | 1 | `abs 0x12da27` | `?` |
| `str` at 0xc91f4 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xc91f8 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0xc9200 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xc9200 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xc9200 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc9200 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc9200 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc9054 | `(unknown callee) 0xc9318` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0xc9090 | `(unknown callee) 0xc943c` | `ret(0xc9318)` | `0x0` | `[sp-29]:1` | `[([sp-29]:1 + [[sp-36]+8])]:1` |
| `bl` at 0xc90b8 | `(unknown callee) 0x48da8` | `[sp-28]` | `0x0` | `0x1202c4` | `0x1202cc` |
| `bl` at 0xc90c4 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xc9104 | `(unknown callee) 0x48da8` | `[sp-28]` | `0x0` | `0x1202c4` | `0x1202cc` |
| `bl` at 0xc9114 | `(unknown callee) 0x19808` | `[sp-48]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xc9128 | `(unknown callee) 0xc885c` | `[sp-28]` | `[sp-29]:1` | `?` | `[sp-29]:1` |
| `bl` at 0xc9134 | `(unknown callee) 0x1f8e0` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xc9160 | `(unknown callee) 0x200a0` | `0x1` | `0x12026c` | `0x1f7` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc9078 | `[([sp-29]:1 + [[sp-36]+8])]:1 != 0x0` | `[([sp-29]:1 + [[sp-36]+8])]:1 == 0x0` |
| `beq` at 0xc90a4 | `ret(0xc943c) == 0x0` | `ret(0xc943c) != 0x0` |
| `bne` at 0xc917c | `[[sp-40]+1]:1 != 0x0` | `[[sp-40]+1]:1 == 0x0` |
| `bne` at 0xc9190 | `[0x297330]:1 != [sp-28]` | `[0x297330]:1 == [sp-28]` |
| `bne` at 0xc91a4 | `[sp-29]:1 != [0x12da24]:1` | `[sp-29]:1 == [0x12da24]:1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc9200 | `r0` | `?` |

## `i2cMuxSetSwitch`

- `i2cMuxSetSwitch`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc9238 to 0xc92f8.
  Established. Unsigned `(r0 & 0xff) <= 0x9` takes `0xc9268`, while larger values fall through
  (`bls` at 0xc9258). Included absolute byte assignments at `0x297330` and `0x297331` write zero
  (`strb` at 0xc9270, `strb` at 0xc9294). The unnamed `0xbaeac` call carries
  `[[sp-17]:1+1180206]:1`, entry `r1`, `0x12022e`, `[[sp-17]:1+1180206]:1` (`bl` at 0xc92c0). Switch
  translation meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xc924c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xc9250 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xc925c | load | 4 | `abs 0xc92d8` | `-` |
| `str` at 0xc9260 | store | 4 | `sp -0x18` | `0xfffff573` |
| `ldr` at 0xc9268 | load | 4 | `abs 0xc92dc` | `-` |
| `strb` at 0xc9270 | store | 1 | `abs 0x297330` | `0x0` |
| `ldr` at 0xc9274 | load | 4 | `abs 0xc92e0` | `-` |
| `strb` at 0xc927c | store | 1 | `abs 0x12da24` | `?` |
| `ldr` at 0xc9280 | load | 4 | `abs 0xc92e4` | `-` |
| `strb` at 0xc9288 | store | 1 | `abs 0x12da25` | `?` |
| `ldr` at 0xc928c | load | 4 | `abs 0xc92e8` | `-` |
| `strb` at 0xc9294 | store | 1 | `abs 0x297331` | `0x0` |
| `ldr` at 0xc9298 | load | 4 | `abs 0xc92ec` | `-` |
| `strb` at 0xc92a0 | store | 1 | `abs 0x12da26` | `?` |
| `ldr` at 0xc92a4 | load | 4 | `abs 0xc92f0` | `-` |
| `strb` at 0xc92ac | store | 1 | `abs 0x12da27` | `?` |
| `ldr` at 0xc92b0 | load | 4 | `abs 0xc92f4` | `-` |
| `ldrb` at 0xc92b4 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xc92b8 | load | 1 | `[sp-17]:1 +0x12022e` | `-` |
| `str` at 0xc92c8 | store | 4 | `sp -0x18` | `ret(0xbaeac)` |
| `ldr` at 0xc92cc | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xc92d4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc92d4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc92d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc92c0 | `(unknown callee) 0xbaeac` | `[[sp-17]:1+1180206]:1` | `r1` | `0x12022e` | `[[sp-17]:1+1180206]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xc9258 | `(r0 & 0xff) <= 0x9 (unsigned, 32-bit)` | `(r0 & 0xff) > 0x9 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc92d4 | `r0` | `?` |
