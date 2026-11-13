# ARM GPIO and flash wrappers in 1.38.4

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
No unresolved computed transfers were reported for these routines. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/gpio-flash-wrappers-facts.txt) retain all tool sections.

## `extIntIRQ`

- `extIntIRQ`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc8a20 to 0xc8ab4.
  Established. The unsigned branch takes its bypass for `(r0 & 0xff) < [0x2ad194]:1` and falls
  through for greater or equal (`blo` at 0xc8a48). The later byte read has unknown target (`ldrb` at
  0xc8a98). The call to `0x21e28` has `0x9`, `0x133148`, `0xd8`, `0x0` (`bl` at 0xc8a6c). IRQ
  mapping and r0 at exit remain unresolved (`ldmdb` at 0xc8aa0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xc8a34 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xc8a38 | load | 4 | `abs 0xc8aa4` | `-` |
| `ldrb` at 0xc8a3c | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xc8a40 | load | 1 | `abs 0x2ad194` | `-` |
| `ldr` at 0xc8a4c | load | 4 | `abs 0xc8aa8` | `-` |
| `str` at 0xc8a50 | store | 4 | `sp -0x1c` | `0x13326c` |
| `ldrb` at 0xc8a54 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xc8a58 | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldr` at 0xc8a60 | load | 4 | `abs 0xc8aac` | `-` |
| `ldrb` at 0xc8a70 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xc8a8c | load | 4 | `abs 0xc8ab0` | `-` |
| `ldr` at 0xc8a90 | load | 4 | `abs 0x2ad190` | `-` |
| `ldrb` at 0xc8a98 | load | 1 | `(unknown)` | `-` |
| `ldmdb` at 0xc8aa0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc8aa0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc8aa0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc8a6c | `(unknown callee) 0x21e28` | `0x9` | `0x133148` | `0xd8` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xc8a48 | `(r0 & 0xff) < [0x2ad194]:1 (unsigned, 32-bit)` | `(r0 & 0xff) >= [0x2ad194]:1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc8aa0 | `r0` | `?` |

## `l1ioGpioInit`

- `l1ioGpioInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc9278 to 0xc92f8.
  Established. The call to `0x1b370` carries `0x133468`, `0x2`, entry r2, entry r3 (`bl` at
  0xc9294). The following word store writes `ret(0x1b370)` at `0x142a00` (`str` at 0xc92a0). The
  signed guard takes its bypass for `[0x142a00] >= 0x0` and falls through when negative (`bge` at
  0xc92ac). Device-open semantics and the common r0 result are not established (`ldmdb` at 0xc92e4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc9288 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc928c | load | 4 | `abs 0xc92e8` | `-` |
| `ldr` at 0xc929c | load | 4 | `abs 0xc92ec` | `-` |
| `str` at 0xc92a0 | store | 4 | `abs 0x142a00` | `ret(0x1b370)` |
| `ldr` at 0xc92a4 | load | 4 | `abs 0x142a00` | `-` |
| `ldr` at 0xc92b8 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xc92c4 | load | 4 | `abs 0xc92f0` | `-` |
| `str` at 0xc92c8 | store | 4 | `sp -0x1c` | `0x133478` |
| `str` at 0xc92cc | store | 4 | `sp -0x18` | `ret(0x1b094)` |
| `ldr` at 0xc92d4 | load | 4 | `abs 0xc92f4` | `-` |
| `ldmdb` at 0xc92e4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc92e4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc92e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc9294 | `(unknown callee) 0x1b370` | `0x133468` | `0x2` | `r2` | `r3` |
| `bl` at 0xc92b0 | `(unknown callee) 0x1b0b8` | `ret(0x1b370)` | `?` | `ret(0x1b370)` | `[0x142a00]` |
| `bl` at 0xc92bc | `(unknown callee) 0x1b094` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0xc92e0 | `(unknown callee) 0x21e28` | `0x1` | `0x133490` | `0x34` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xc92ac | `[0x142a00] >= 0x0 (signed, 32-bit)` | `[0x142a00] < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc92e4 | `r0` | `?` |

## `l1ioGpioReset`

- `l1ioGpioReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc92f8 to 0xc94e0.
  Established. Calls to `0x1b40c` include `[0x142a00]`, `0x40046706`, `sp-28`, `0x142a00` and
  `[0x142a00]`, `0x80046708`, `sp-28`, `0x142a00` (`bl` at 0xc9358, `bl` at 0xc9468). The first
  result branch takes its continuation on zero and falls through on nonzero (`beq` at 0xc9364). Two
  intervening stack-byte stores at `sp -0x1b` have unknown values (`strb` at 0xc93cc, `strb` at
  0xc9418). GPIO reset polarity and r0 at the merged exit remain unknown (`ldmdb` at 0xc94c0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc9308 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc930c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc9314 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xc9318 | load | 4 | `abs 0xc94c4` | `-` |
| `ldr` at 0xc931c | load | 4 | `abs 0x142a00` | `-` |
| `ldr` at 0xc9328 | load | 4 | `abs 0xc94c8` | `-` |
| `str` at 0xc932c | store | 4 | `sp -0x24` | `0xfffff3e4` |
| `ldr` at 0xc9334 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc9338 | load | 1 | `r1 +0x3` | `-` |
| `strb` at 0xc933c | store | 1 | `sp -0x1c` | `([r1+3]:1 & 0xff)` |
| `strb` at 0xc9344 | store | 1 | `sp -0x1b` | `0x0` |
| `ldr` at 0xc9348 | load | 4 | `abs 0xc94c4` | `-` |
| `ldr` at 0xc9350 | load | 4 | `abs 0x142a00` | `-` |
| `ldr` at 0xc9354 | load | 4 | `abs 0xc94cc` | `-` |
| `ldr` at 0xc9370 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `str` at 0xc9374 | store | 4 | `sp -0x20` | `[ret(0x1b0b8)]` |
| `ldr` at 0xc9378 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc937c | load | 1 | `[sp-24] +0x6` | `-` |
| `strb` at 0xc9380 | store | 1 | `sp -0x1b` | `([[sp-24]+6]:1 & 0xff)` |
| `ldr` at 0xc9384 | load | 4 | `abs 0xc94c4` | `-` |
| `ldr` at 0xc938c | load | 4 | `abs 0x142a00` | `-` |
| `ldr` at 0xc9390 | load | 4 | `abs 0xc94d0` | `-` |
| `ldr` at 0xc93a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc93b8 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `str` at 0xc93bc | store | 4 | `sp -0x20` | `[ret(0x1b0b8)]` |
| `ldr` at 0xc93c0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc93c4 | load | 1 | `[sp-24] +0x5` | `-` |
| `strb` at 0xc93cc | store | 1 | `sp -0x1b` | `?` |
| `ldr` at 0xc93d0 | load | 4 | `abs 0xc94c4` | `-` |
| `ldr` at 0xc93d8 | load | 4 | `abs 0x142a00` | `-` |
| `ldr` at 0xc93dc | load | 4 | `abs 0xc94d4` | `-` |
| `ldr` at 0xc93f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc9404 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `str` at 0xc9408 | store | 4 | `sp -0x20` | `[ret(0x1b0b8)]` |
| `ldr` at 0xc940c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc9410 | load | 1 | `[sp-24] +0x4` | `-` |
| `strb` at 0xc9418 | store | 1 | `sp -0x1b` | `?` |
| `ldr` at 0xc941c | load | 4 | `abs 0xc94c4` | `-` |
| `ldr` at 0xc9424 | load | 4 | `abs 0x142a00` | `-` |
| `ldr` at 0xc9428 | load | 4 | `abs 0xc94d8` | `-` |
| `ldr` at 0xc943c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc9450 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `str` at 0xc9454 | store | 4 | `sp -0x20` | `[ret(0x1b0b8)]` |
| `ldr` at 0xc9458 | load | 4 | `abs 0xc94c4` | `-` |
| `ldr` at 0xc9460 | load | 4 | `abs 0x142a00` | `-` |
| `ldr` at 0xc9464 | load | 4 | `abs 0xc94dc` | `-` |
| `ldr` at 0xc9478 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc948c | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `str` at 0xc9490 | store | 4 | `sp -0x20` | `[ret(0x1b0b8)]` |
| `ldr` at 0xc9494 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc94a0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc94ac | store | 4 | `sp -0x24` | `ret(0xc9924)` |
| `str` at 0xc94b8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc94bc | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xc94c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc94c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc94c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc9358 | `(unknown callee) 0x1b40c` | `[0x142a00]` | `0x40046706` | `sp-28` | `0x142a00` |
| `bl` at 0xc9368 | `(unknown callee) 0x1b0b8` | `ret(0x1b40c)` | `?` | `?` | `ret(0x1b40c)` |
| `bl` at 0xc9394 | `(unknown callee) 0x1b40c` | `[0x142a00]` | `0x40046700` | `sp-28` | `0x142a00` |
| `bl` at 0xc93b0 | `(unknown callee) 0x1b0b8` | `ret(0x1b40c)` | `?` | `?` | `[sp-32]` |
| `bl` at 0xc93e0 | `(unknown callee) 0x1b40c` | `[0x142a00]` | `0x40046704` | `sp-28` | `0x142a00` |
| `bl` at 0xc93fc | `(unknown callee) 0x1b0b8` | `ret(0x1b40c)` | `?` | `?` | `[sp-32]` |
| `bl` at 0xc942c | `(unknown callee) 0x1b40c` | `[0x142a00]` | `0x40046702` | `sp-28` | `0x142a00` |
| `bl` at 0xc9448 | `(unknown callee) 0x1b0b8` | `ret(0x1b40c)` | `?` | `?` | `[sp-32]` |
| `bl` at 0xc9468 | `(unknown callee) 0x1b40c` | `[0x142a00]` | `0x80046708` | `sp-28` | `0x142a00` |
| `bl` at 0xc9484 | `(unknown callee) 0x1b0b8` | `ret(0x1b40c)` | `?` | `?` | `[sp-32]` |
| `bl` at 0xc94a4 | `(unknown callee) 0xc9924` | `[sp-32]` | `?` | `?` | `[sp-32]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xc9324 | `[0x142a00] >= 0x0 (signed, 32-bit)` | `[0x142a00] < 0x0 (signed, 32-bit)` |
| `beq` at 0xc9364 | `ret(0x1b40c) == 0x0` | `ret(0x1b40c) != 0x0` |
| `beq` at 0xc93a0 | `ret(0x1b40c) == 0x0` | `ret(0x1b40c) != 0x0` |
| `bne` at 0xc93ac | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xc93ec | `ret(0x1b40c) == 0x0` | `ret(0x1b40c) != 0x0` |
| `bne` at 0xc93f8 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xc9438 | `ret(0x1b40c) == 0x0` | `ret(0x1b40c) != 0x0` |
| `bne` at 0xc9444 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xc9474 | `ret(0x1b40c) == 0x0` | `ret(0x1b40c) != 0x0` |
| `bne` at 0xc9480 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xc949c | `[sp-32] == 0x0` | `[sp-32] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc94c0 | `r0` | `?` |

## `l1ioGpioWrite`

- `l1ioGpioWrite`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc94e0 to 0xc9588.
  Established. The byte saves write `(r2 & 0xff)` at `sp -0x19` and `((r2 & 0xff) & 0xff)` at
  `sp -0x1f` (`strb` at 0xc94fc, `strb` at 0xc952c). The call to `0x1b40c` has `[0x142a00]`,
  `0x40046700`, `sp-32`, `0x142a00` (`bl` at 0xc9540); its zero result takes the continuation and
  nonzero falls through (`beq` at 0xc954c). Output pin semantics and the joined r0 result remain
  unknown (`ldmdb` at 0xc9578).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc94f0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc94f4 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xc94fc | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `ldr` at 0xc9500 | load | 4 | `abs 0xc957c` | `-` |
| `ldr` at 0xc9504 | load | 4 | `abs 0x142a00` | `-` |
| `ldr` at 0xc9510 | load | 4 | `abs 0xc9580` | `-` |
| `str` at 0xc9514 | store | 4 | `sp -0x24` | `0xfffff3e4` |
| `ldr` at 0xc951c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc9520 | load | 1 | `r1 +0x3` | `-` |
| `strb` at 0xc9524 | store | 1 | `sp -0x20` | `([r1+3]:1 & 0xff)` |
| `ldrb` at 0xc9528 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xc952c | store | 1 | `sp -0x1f` | `((r2 & 0xff) & 0xff)` |
| `ldr` at 0xc9530 | load | 4 | `abs 0xc957c` | `-` |
| `ldr` at 0xc9538 | load | 4 | `abs 0x142a00` | `-` |
| `ldr` at 0xc953c | load | 4 | `abs 0xc9584` | `-` |
| `ldr` at 0xc9558 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `str` at 0xc9564 | store | 4 | `sp -0x24` | `ret(0xc9924)` |
| `str` at 0xc9570 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc9574 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xc9578 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc9578 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc9578 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc9540 | `(unknown callee) 0x1b40c` | `[0x142a00]` | `0x40046700` | `sp-32` | `0x142a00` |
| `bl` at 0xc9550 | `(unknown callee) 0x1b0b8` | `ret(0x1b40c)` | `?` | `?` | `ret(0x1b40c)` |
| `bl` at 0xc955c | `(unknown callee) 0xc9924` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xc950c | `[0x142a00] >= 0x0 (signed, 32-bit)` | `[0x142a00] < 0x0 (signed, 32-bit)` |
| `beq` at 0xc954c | `ret(0x1b40c) == 0x0` | `ret(0x1b40c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc9578 | `r0` | `?` |

## `l1ioGpioRead`

- `l1ioGpioRead`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc9588 to 0xc9630.
  Established. The call to `0x1b40c` receives `[0x142a00]`, `0xc0046701`, `sp-32`, `0x142a00` (`bl`
  at 0xc95dc). A zero result takes the arm containing the output store, while nonzero falls through
  (`beq` at 0xc95e8). That byte store writes `([sp-31]:1 & 0xff)` at `[sp-28] +0x0` (`strb` at
  0xc9610). Input pin mapping and r0 at exit are unidentified (`ldmdb` at 0xc9620).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc9598 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc959c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc95a0 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xc95a4 | load | 4 | `abs 0xc9624` | `-` |
| `ldr` at 0xc95a8 | load | 4 | `abs 0x142a00` | `-` |
| `ldr` at 0xc95b4 | load | 4 | `abs 0xc9628` | `-` |
| `str` at 0xc95b8 | store | 4 | `sp -0x24` | `0xfffff3e4` |
| `ldr` at 0xc95c0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc95c4 | load | 1 | `r1 +0x3` | `-` |
| `strb` at 0xc95c8 | store | 1 | `sp -0x20` | `([r1+3]:1 & 0xff)` |
| `ldr` at 0xc95cc | load | 4 | `abs 0xc9624` | `-` |
| `ldr` at 0xc95d4 | load | 4 | `abs 0x142a00` | `-` |
| `ldr` at 0xc95d8 | load | 4 | `abs 0xc962c` | `-` |
| `ldr` at 0xc95f4 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `str` at 0xc9600 | store | 4 | `sp -0x24` | `ret(0xc9924)` |
| `ldr` at 0xc9608 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc960c | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0xc9610 | store | 1 | `[sp-28] +0x0` | `([sp-31]:1 & 0xff)` |
| `str` at 0xc9618 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc961c | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xc9620 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc9620 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc9620 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc95dc | `(unknown callee) 0x1b40c` | `[0x142a00]` | `0xc0046701` | `sp-32` | `0x142a00` |
| `bl` at 0xc95ec | `(unknown callee) 0x1b0b8` | `ret(0x1b40c)` | `?` | `?` | `ret(0x1b40c)` |
| `bl` at 0xc95f8 | `(unknown callee) 0xc9924` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xc95b0 | `[0x142a00] >= 0x0 (signed, 32-bit)` | `[0x142a00] < 0x0 (signed, 32-bit)` |
| `beq` at 0xc95e8 | `ret(0x1b40c) == 0x0` | `ret(0x1b40c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc9620 | `r0` | `?` |

## `l1ioGpioIntSet`

- `l1ioGpioIntSet`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc9630 to 0xc96d8.
  Established. The entry byte at `r1 +0x3` is read before the stack-byte save (`ldrb` at 0xc9670,
  `strb` at 0xc9674). The call to `0x1b40c` carries `[0x142a00]`, `0x40046706`, `sp-32`, `0x142a00`
  (`bl` at 0xc9690); its zero result takes the continuation and nonzero falls through (`beq` at
  0xc969c). Interrupt enable polarity and the common r0 result remain unproved (`ldmdb` at 0xc96c8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc9640 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc9644 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xc964c | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `ldr` at 0xc9650 | load | 4 | `abs 0xc96cc` | `-` |
| `ldr` at 0xc9654 | load | 4 | `abs 0x142a00` | `-` |
| `ldr` at 0xc9660 | load | 4 | `abs 0xc96d0` | `-` |
| `str` at 0xc9664 | store | 4 | `sp -0x24` | `0xfffff3e4` |
| `ldr` at 0xc966c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc9670 | load | 1 | `r1 +0x3` | `-` |
| `strb` at 0xc9674 | store | 1 | `sp -0x20` | `([r1+3]:1 & 0xff)` |
| `ldrb` at 0xc9678 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xc967c | store | 1 | `sp -0x1f` | `((r2 & 0xff) & 0xff)` |
| `ldr` at 0xc9680 | load | 4 | `abs 0xc96cc` | `-` |
| `ldr` at 0xc9688 | load | 4 | `abs 0x142a00` | `-` |
| `ldr` at 0xc968c | load | 4 | `abs 0xc96d4` | `-` |
| `ldr` at 0xc96a8 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `str` at 0xc96b4 | store | 4 | `sp -0x24` | `ret(0xc9924)` |
| `str` at 0xc96c0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc96c4 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xc96c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc96c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc96c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc9690 | `(unknown callee) 0x1b40c` | `[0x142a00]` | `0x40046706` | `sp-32` | `0x142a00` |
| `bl` at 0xc96a0 | `(unknown callee) 0x1b0b8` | `ret(0x1b40c)` | `?` | `?` | `ret(0x1b40c)` |
| `bl` at 0xc96ac | `(unknown callee) 0xc9924` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xc965c | `[0x142a00] >= 0x0 (signed, 32-bit)` | `[0x142a00] < 0x0 (signed, 32-bit)` |
| `beq` at 0xc969c | `ret(0x1b40c) == 0x0` | `ret(0x1b40c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc96c8 | `r0` | `?` |

## `l1ioGpioIntGet`

- `l1ioGpioIntGet`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc96d8 to 0xc9780.
  Established. The byte store at `[sp-28] +0x0` writes `([sp-31]:1 & 0xff)` (`strb` at 0xc9760). The
  prior call to `0x1b40c` has `[0x142a00]`, `0xc0046707`, `sp-32`, `0x142a00` (`bl` at 0xc972c);
  its zero result takes the output-store arm and nonzero falls through (`beq` at 0xc9738). Interrupt
  status encoding and r0 at the common return are unresolved (`ldmdb` at 0xc9770).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc96e8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc96ec | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc96f0 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xc96f4 | load | 4 | `abs 0xc9774` | `-` |
| `ldr` at 0xc96f8 | load | 4 | `abs 0x142a00` | `-` |
| `ldr` at 0xc9704 | load | 4 | `abs 0xc9778` | `-` |
| `str` at 0xc9708 | store | 4 | `sp -0x24` | `0xfffff3e4` |
| `ldr` at 0xc9710 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc9714 | load | 1 | `r1 +0x3` | `-` |
| `strb` at 0xc9718 | store | 1 | `sp -0x20` | `([r1+3]:1 & 0xff)` |
| `ldr` at 0xc971c | load | 4 | `abs 0xc9774` | `-` |
| `ldr` at 0xc9724 | load | 4 | `abs 0x142a00` | `-` |
| `ldr` at 0xc9728 | load | 4 | `abs 0xc977c` | `-` |
| `ldr` at 0xc9744 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `str` at 0xc9750 | store | 4 | `sp -0x24` | `ret(0xc9924)` |
| `ldr` at 0xc9758 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc975c | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0xc9760 | store | 1 | `[sp-28] +0x0` | `([sp-31]:1 & 0xff)` |
| `str` at 0xc9768 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc976c | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xc9770 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc9770 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc9770 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc972c | `(unknown callee) 0x1b40c` | `[0x142a00]` | `0xc0046707` | `sp-32` | `0x142a00` |
| `bl` at 0xc973c | `(unknown callee) 0x1b0b8` | `ret(0x1b40c)` | `?` | `?` | `ret(0x1b40c)` |
| `bl` at 0xc9748 | `(unknown callee) 0xc9924` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xc9700 | `[0x142a00] >= 0x0 (signed, 32-bit)` | `[0x142a00] < 0x0 (signed, 32-bit)` |
| `beq` at 0xc9738 | `ret(0x1b40c) == 0x0` | `ret(0x1b40c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc9770 | `r0` | `?` |

## `l1ioGpioInt`

- `l1ioGpioInt`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc9780 to 0xc9924.
  Established. The call to `0x1b40c` carries `[0x142a00]`, `0x80046708`, `sp-36`, `0x142a00` (`bl`
  at 0xc97f8). A later call to `0xd7168` uses `0x0`, `[sp-24]`, `[sp-28]`, `[sp-29]:1` (`bl` at
  0xc9898). The intervening guard takes its bypass when an unknown value equals `0x0` and falls
  through on nonzero (`beq` at 0xc9878). IRQ acknowledgement semantics and the merged r0 value
  remain unknown (`ldmdb` at 0xc9908).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc9790 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc9794 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc9798 | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0xc979c | store | 1 | `sp -0x1d` | `(r3 & 0xff)` |
| `str` at 0xc97a4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xc97a8 | load | 4 | `abs 0xc990c` | `-` |
| `ldr` at 0xc97ac | load | 4 | `abs 0x142a00` | `-` |
| `ldr` at 0xc97b8 | load | 4 | `abs 0xc9910` | `-` |
| `str` at 0xc97bc | store | 4 | `sp -0x30` | `0xfffff3e4` |
| `ldr` at 0xc97c4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc97c8 | load | 1 | `r1 +0x3` | `-` |
| `strb` at 0xc97cc | store | 1 | `sp -0x24` | `([r1+3]:1 & 0xff)` |
| `str` at 0xc97d4 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xc97d8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc97e8 | load | 4 | `abs 0xc990c` | `-` |
| `ldr` at 0xc97f0 | load | 4 | `abs 0x142a00` | `-` |
| `ldr` at 0xc97f4 | load | 4 | `abs 0xc9914` | `-` |
| `ldr` at 0xc9810 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xc9828 | load | 4 | `abs 0xc9918` | `-` |
| `str` at 0xc982c | store | 4 | `sp -0x3c` | `0x13349c` |
| `ldr` at 0xc9830 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc9834 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xc9838 | store | 4 | `sp -0x38` | `[[sp-24]+12]` |
| `str` at 0xc983c | store | 4 | `sp -0x34` | `ret(0x219a8)` |
| `ldr` at 0xc9844 | load | 4 | `abs 0xc991c` | `-` |
| `ldr` at 0xc9854 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc9858 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xc9860 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc9864 | load | 1 | `sp -0x23` | `-` |
| `ldrb` at 0xc9868 | load | 1 | `[sp-28] +0x3` | `-` |
| `ldrb` at 0xc987c | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0xc9880 | load | 1 | `sp -0x23` | `-` |
| `str` at 0xc9884 | store | 4 | `sp -0x3c` | `[sp-35]:1` |
| `ldr` at 0xc988c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc9890 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc98a0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc98a8 | store | 4 | `sp -0x28` | `([sp-40] + ret(0xd7168))` |
| `ldr` at 0xc98b0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc98b4 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xc98bc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc98c4 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xc98cc | load | 4 | `abs 0xc9920` | `-` |
| `str` at 0xc98d0 | store | 4 | `sp -0x3c` | `0x1334c8` |
| `ldr` at 0xc98d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc98d8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc98dc | store | 4 | `sp -0x38` | `?` |
| `ldrb` at 0xc98e0 | load | 1 | `sp -0x23` | `-` |
| `str` at 0xc98e4 | store | 4 | `sp -0x34` | `[sp-35]:1` |
| `ldr` at 0xc98ec | load | 4 | `abs 0xc991c` | `-` |
| `ldr` at 0xc98fc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc9900 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xc9904 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xc9908 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc9908 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc9908 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc97f8 | `(unknown callee) 0x1b40c` | `[0x142a00]` | `0x80046708` | `sp-36` | `0x142a00` |
| `bl` at 0xc9808 | `(unknown callee) 0x1b0b8` | `ret(0x1b40c)` | `?` | `?` | `ret(0x1b40c)` |
| `bl` at 0xc9814 | `(unknown callee) 0xc9924` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0xc9820 | `(unknown callee) 0x219a8` | `ret(0xc9924)` | `?` | `?` | `ret(0xc9924)` |
| `bl` at 0xc9850 | `(unknown callee) 0x21e28` | `0x3` | `0x133490` | `0xda` | `0x0` |
| `bl` at 0xc9898 | `(unknown callee) 0xd7168` | `0x0` | `[sp-24]` | `[sp-28]` | `[sp-29]:1` |
| `bl` at 0xc98f8 | `(unknown callee) 0x21e28` | `0x3` | `0x133490` | `0xe5` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xc97b4 | `[0x142a00] >= 0x0 (signed, 32-bit)` | `[0x142a00] < 0x0 (signed, 32-bit)` |
| `ble` at 0xc97e0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xc9804 | `ret(0x1b40c) == 0x0` | `ret(0x1b40c) != 0x0` |
| `beq` at 0xc9878 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc9908 | `r0` | `?` |

## `flash_lock`

- `flash_lock`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcaf90 to 0xcb044.
  Established. The call to `0x1b418` has `[0x142a84]`, `0x142a6c`, `0x2`, `0x142a84` (`bl` at
  0xcafc0); its signed result takes the bypass for at least `0x0` and falls through when negative
  (`bge` at 0xcafcc). The earlier signed branch bypasses the call for `[0x142a84] < 0x0` and falls
  through otherwise (`blt` at 0xcafac). Lock ownership and r0 at exit remain unknown (`ldmdb` at
  0xcb030).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xcafa0 | load | 4 | `abs 0xcb034` | `-` |
| `ldr` at 0xcafa4 | load | 4 | `abs 0x142a84` | `-` |
| `ldr` at 0xcafb0 | load | 4 | `abs 0xcb034` | `-` |
| `ldr` at 0xcafb4 | load | 4 | `abs 0x142a84` | `-` |
| `ldr` at 0xcafb8 | load | 4 | `abs 0xcb038` | `-` |
| `ldr` at 0xcafe4 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xcaff0 | load | 4 | `abs 0xcb03c` | `-` |
| `str` at 0xcaff4 | store | 4 | `sp -0x24` | `0x1339b4` |
| `ldr` at 0xcaff8 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `str` at 0xcaffc | store | 4 | `sp -0x20` | `[ret(0x1b0b8)]` |
| `str` at 0xcb000 | store | 4 | `sp -0x1c` | `ret(0x1b094)` |
| `ldr` at 0xcb008 | load | 4 | `abs 0xcb040` | `-` |
| `str` at 0xcb01c | store | 4 | `sp -0x18` | `?` |
| `str` at 0xcb028 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xcb02c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xcb030 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xcb030 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcb030 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcb030 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcafc0 | `(unknown callee) 0x1b418` | `[0x142a84]` | `0x142a6c` | `0x2` | `0x142a84` |
| `bl` at 0xcafd0 | `(unknown callee) 0x1b0b8` | `ret(0x1b418)` | `?` | `?` | `ret(0x1b418)` |
| `bl` at 0xcafdc | `(unknown callee) 0x1b0b8` | `ret(0x1b0b8)` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0xcafe8 | `(unknown callee) 0x1b094` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0xcb014 | `(unknown callee) 0x21e28` | `0x4` | `0x133850` | `0xea` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xcafac | `[0x142a84] < 0x0 (signed, 32-bit)` | `[0x142a84] >= 0x0 (signed, 32-bit)` |
| `bge` at 0xcafcc | `ret(0x1b418) >= 0x0 (signed, 32-bit)` | `ret(0x1b418) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcb030 | `r0` | `?` |

## `flash_unlock`

- `flash_unlock`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcb044 to 0xcb0f8.
  Established. The call to `0x1b418` uses `[0x142a84]`, `0x142a7c`, `0x1`, `0x142a84` (`bl` at
  0xcb074); its nonnegative signed result takes the bypass and a negative result falls through
  (`bge` at 0xcb080). The final call to `0x21e28` carries `0x4`, `0x133850`, `0xf5`, `0x0` (`bl` at
  0xcb0c8). The request semantics and resulting r0 are unestablished (`ldmdb` at 0xcb0e4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xcb054 | load | 4 | `abs 0xcb0e8` | `-` |
| `ldr` at 0xcb058 | load | 4 | `abs 0x142a84` | `-` |
| `ldr` at 0xcb064 | load | 4 | `abs 0xcb0e8` | `-` |
| `ldr` at 0xcb068 | load | 4 | `abs 0x142a84` | `-` |
| `ldr` at 0xcb06c | load | 4 | `abs 0xcb0ec` | `-` |
| `ldr` at 0xcb098 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xcb0a4 | load | 4 | `abs 0xcb0f0` | `-` |
| `str` at 0xcb0a8 | store | 4 | `sp -0x24` | `0x1339f0` |
| `ldr` at 0xcb0ac | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `str` at 0xcb0b0 | store | 4 | `sp -0x20` | `[ret(0x1b0b8)]` |
| `str` at 0xcb0b4 | store | 4 | `sp -0x1c` | `ret(0x1b094)` |
| `ldr` at 0xcb0bc | load | 4 | `abs 0xcb0f4` | `-` |
| `str` at 0xcb0d0 | store | 4 | `sp -0x18` | `?` |
| `str` at 0xcb0dc | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xcb0e0 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xcb0e4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xcb0e4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcb0e4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcb0e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcb074 | `(unknown callee) 0x1b418` | `[0x142a84]` | `0x142a7c` | `0x1` | `0x142a84` |
| `bl` at 0xcb084 | `(unknown callee) 0x1b0b8` | `ret(0x1b418)` | `?` | `?` | `ret(0x1b418)` |
| `bl` at 0xcb090 | `(unknown callee) 0x1b0b8` | `ret(0x1b0b8)` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0xcb09c | `(unknown callee) 0x1b094` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0xcb0c8 | `(unknown callee) 0x21e28` | `0x4` | `0x133850` | `0xf5` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xcb060 | `[0x142a84] < 0x0 (signed, 32-bit)` | `[0x142a84] >= 0x0 (signed, 32-bit)` |
| `bge` at 0xcb080 | `ret(0x1b418) >= 0x0 (signed, 32-bit)` | `ret(0x1b418) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcb0e4 | `r0` | `?` |

## `flash_offset`

- `flash_offset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcb190 to 0xcb21c.
  Established. The call to `0xcb0f8` has entry r0, entry r1, entry r2, entry r3 (`bl` at 0xcb1a8);
  its signed result takes the work arm for at least `0x0` and falls through for negative (`bge` at
  0xcb1bc). The later unsigned comparison has unknown operands, takes the branch for less and falls
  through for greater or equal (`blo` at 0xcb1e4). The return reports r0 `[sp-32]` (`ldmdb` at
  0xcb210). Physical flash-offset interpretation remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcb1a0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xcb1a4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcb1b0 | store | 4 | `sp -0x1c` | `ret(0xcb0f8)` |
| `ldr` at 0xcb1b4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcb1c4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcb1cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcb1d4 | load | 4 | `abs 0xcb214` | `-` |
| `ldr` at 0xcb1d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcb1dc | load | 4 | `(unknown)` | `-` |
| `str` at 0xcb1ec | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcb1f4 | load | 4 | `abs 0xcb218` | `-` |
| `ldr` at 0xcb1f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcb1fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcb200 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcb208 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcb20c | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xcb210 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcb210 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcb210 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcb1a8 | `(unknown callee) 0xcb0f8` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xcb1bc | `ret(0xcb0f8) >= 0x0 (signed, 32-bit)` | `ret(0xcb0f8) < 0x0 (signed, 32-bit)` |
| `blo` at 0xcb1e4 | `? < ? (unsigned, 32-bit)` | `? >= ? (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcb210 | `r0` | `[sp-32]` |

## `flash_read_addr`

- `flash_read_addr`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcb21c to 0xcb330.
  Established. The call to `0xfbbd4` has unknown, `sp-112`, `sp-112`, `[sp-120]` (`bl` at 0xcb288);
  its signed result takes the continuation for at least `0x0` and falls through when negative (`bge`
  at 0xcb294). An earlier call to `0xcb190` carries `[sp-20]`, unknown, unknown, `ret(0xcb0f8)`
  (`bl` at 0xcb25c). The exit reports r0 `[sp-124]` (`ldmdb` at 0xcb318). Flash mapping and physical
  read effects are unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcb22c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xcb230 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcb23c | store | 4 | `sp -0x78` | `ret(0xcb0f8)` |
| `ldr` at 0xcb240 | load | 4 | `sp -0x78` | `-` |
| `str` at 0xcb250 | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0xcb258 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcb264 | store | 4 | `sp -0x74` | `ret(0xcb190)` |
| `ldr` at 0xcb268 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xcb274 | load | 4 | `abs 0xcb31c` | `-` |
| `ldr` at 0xcb278 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xcb280 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcb2a0 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xcb2ac | load | 4 | `abs 0xcb320` | `-` |
| `str` at 0xcb2b0 | store | 4 | `sp -0x88` | `0x133a30` |
| `ldr` at 0xcb2b4 | load | 4 | `sp -0x78` | `-` |
| `str` at 0xcb2b8 | store | 4 | `sp -0x84` | `[sp-120]` |
| `str` at 0xcb2bc | store | 4 | `sp -0x80` | `ret(0x21674)` |
| `ldr` at 0xcb2c4 | load | 4 | `abs 0xcb324` | `-` |
| `ldr` at 0xcb2c8 | load | 4 | `abs 0xcb328` | `-` |
| `str` at 0xcb2d8 | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0xcb2e0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xcb2e4 | load | 4 | `sp -0x74` | `-` |
| `str` at 0xcb2f4 | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0xcb2fc | load | 4 | `abs 0xcb32c` | `-` |
| `ldr` at 0xcb300 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xcb304 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcb308 | load | 4 | `sp -0x74` | `-` |
| `str` at 0xcb310 | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0xcb314 | load | 4 | `sp -0x7c` | `-` |
| `ldmdb` at 0xcb318 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcb318 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcb318 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcb234 | `(unknown callee) 0xcb0f8` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xcb25c | `(unknown callee) 0xcb190` | `[sp-20]` | `?` | `?` | `ret(0xcb0f8)` |
| `bl` at 0xcb288 | `(unknown callee) 0xfbbd4` | `?` | `sp-112` | `sp-112` | `[sp-120]` |
| `bl` at 0xcb298 | `(unknown callee) 0x1b0b8` | `ret(0xfbbd4)` | `?` | `?` | `ret(0xfbbd4)` |
| `bl` at 0xcb2a4 | `(unknown callee) 0x21674` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0xcb2d0 | `(unknown callee) 0x21e28` | `0x3` | `0x133850` | `0x12d` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xcb248 | `ret(0xcb0f8) >= 0x0 (signed, 32-bit)` | `ret(0xcb0f8) < 0x0 (signed, 32-bit)` |
| `ble` at 0xcb270 | `[sp-120] <= 0x0 (signed, 32-bit)` | `[sp-120] > 0x0 (signed, 32-bit)` |
| `bge` at 0xcb294 | `ret(0xfbbd4) >= 0x0 (signed, 32-bit)` | `ret(0xfbbd4) < 0x0 (signed, 32-bit)` |
| `bhi` at 0xcb2ec | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcb318 | `r0` | `[sp-124]` |

## `flash_write_addr`

- `flash_write_addr`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcb330 to 0xcb3cc.
  Established. The second call to `0xcb0f8` carries `[0x2b36b0]`, unknown, unknown, `0x2b36a0` (`bl`
  at 0xcb384). The following branch takes its arm for `[sp-28] != ret(0xcb0f8)` and falls through on
  equality (`bne` at 0xcb394). The call to `0xcb190` uses `[sp-20]`, unknown, unknown,
  `ret(0xcb0f8)` (`bl` at 0xcb370). The return supplies r0 `[sp-32]` (`ldmdb` at 0xcb3c0).
  Writable-window identity remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcb340 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xcb344 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcb350 | store | 4 | `sp -0x1c` | `ret(0xcb0f8)` |
| `ldr` at 0xcb354 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcb364 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcb36c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcb378 | store | 4 | `sp -0x18` | `ret(0xcb190)` |
| `ldr` at 0xcb37c | load | 4 | `abs 0xcb3c4` | `-` |
| `ldr` at 0xcb380 | load | 4 | `abs 0x2b36b0` | `-` |
| `ldr` at 0xcb38c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcb39c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcb3a4 | load | 4 | `abs 0xcb3c8` | `-` |
| `ldr` at 0xcb3a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcb3ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcb3b0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcb3b8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcb3bc | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xcb3c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcb3c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcb3c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcb348 | `(unknown callee) 0xcb0f8` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xcb370 | `(unknown callee) 0xcb190` | `[sp-20]` | `?` | `?` | `ret(0xcb0f8)` |
| `bl` at 0xcb384 | `(unknown callee) 0xcb0f8` | `[0x2b36b0]` | `?` | `?` | `0x2b36a0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xcb35c | `ret(0xcb0f8) >= 0x0 (signed, 32-bit)` | `ret(0xcb0f8) < 0x0 (signed, 32-bit)` |
| `bne` at 0xcb394 | `[sp-28] != ret(0xcb0f8)` | `[sp-28] == ret(0xcb0f8)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcb3c0 | `r0` | `[sp-32]` |

## `flash_read`

- `flash_read`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcb3cc to 0xcb45c.
  Established. The global-value test takes its bypass for `[0x2ad1d4] != 0x0` and falls through on
  zero (`bne` at 0xcb3f4). The later call to `0x1b304` uses `[sp-20]`, `[sp-32]`, `[sp-28]`, unknown
  (`bl` at 0xcb440). The final call reaches `0xcb044` with `ret(0x1b304)` and three unknown
  candidates (`bl` at 0xcb444). Buffer-copy semantics, physical flash access and the common r0
  result are unresolved (`ldmdb` at 0xcb454).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcb3dc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcb3e0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcb3e4 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xcb3e8 | load | 4 | `abs 0xcb458` | `-` |
| `ldr` at 0xcb3ec | load | 4 | `abs 0x2ad1d4` | `-` |
| `str` at 0xcb404 | store | 4 | `sp -0x24` | `ret(0xcae70)` |
| `ldr` at 0xcb408 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcb414 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcb418 | store | 4 | `sp -0x28` | `ret(0xcae70)` |
| `ldr` at 0xcb420 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcb42c | store | 4 | `sp -0x20` | `ret(0xcb21c)` |
| `ldr` at 0xcb434 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcb438 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcb43c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcb44c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xcb450 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xcb454 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcb454 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcb454 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcb3fc | `(unknown callee) 0xcae70` | `0x0` | `r1` | `r2` | `[0x2ad1d4]` |
| `bl` at 0xcb424 | `(unknown callee) 0xcb21c` | `?` | `?` | `?` | `?` |
| `bl` at 0xcb430 | `(unknown callee) 0xcaf90` | `ret(0xcb21c)` | `?` | `?` | `ret(0xcb21c)` |
| `bl` at 0xcb440 | `(unknown callee) 0x1b304` | `[sp-20]` | `[sp-32]` | `[sp-28]` | `?` |
| `bl` at 0xcb444 | `(unknown callee) 0xcb044` | `ret(0x1b304)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcb3f4 | `[0x2ad1d4] != 0x0` | `[0x2ad1d4] == 0x0` |
| `bge` at 0xcb410 | `ret(0xcae70) >= 0x0 (signed, 32-bit)` | `ret(0xcae70) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcb454 | `r0` | `?` |

## `flash_erase`

- `flash_erase`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcb45c to 0xcb48c.
  Established. The three explicit word saves retain entry r0 at `sp -0x14`, entry r1 at `sp -0x18`,
  and entry r2 at `sp -0x1c` (`str` at 0xcb46c, `str` at 0xcb470, `str` at 0xcb474). The exit
  reports r0 `0x0` (`ldmdb` at 0xcb488). Calls and conditional branches are absent from the fact
  sections. Erase activity is not established by this body.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcb46c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcb470 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcb474 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xcb47c | store | 4 | `sp -0x24` | `0x0` |
| `ldmdb` at 0xcb488 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcb488 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcb488 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcb488 | `r0` | `0x0` |
