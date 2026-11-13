# ARM routing and packet setup paths in 1.44.0

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
No unresolved computed transfers were reported for these routines. Undecodable byte ranges
remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/routing-packet-setup-facts.txt) retain all tool sections.

## `L1NetInit`

- `L1NetInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5afa0 to 0x5b138.
  Established. The absolute word at `0x132624` receives `[0x132624]+1` (`str` at 0x5afec).
  A zero `ret(0xa8af0)` takes `0x5b060`, while nonzero falls through (`beq` at 0x5b030).
  The unnamed `0xa89d0` call carries `0x299858`, `0x11`, `0x0`, unknown (`bl` at 0x5b00c).
  Network setup policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x5afb0 | load | 4 | `abs 0x5b0f4` | `-` |
| `ldr` at 0x5afb4 | load | 4 | `abs 0x132624` | `-` |
| `ldr` at 0x5afc0 | load | 4 | `abs 0x5b0f8` | `-` |
| `str` at 0x5afc4 | store | 4 | `sp -0x30` | `0xf9db0` |
| `ldr` at 0x5afcc | load | 4 | `abs 0x5b0fc` | `-` |
| `ldr` at 0x5afdc | load | 4 | `abs 0x5b0f4` | `-` |
| `ldr` at 0x5afe0 | load | 4 | `abs 0x5b0f4` | `-` |
| `ldr` at 0x5afe4 | load | 4 | `abs 0x132624` | `-` |
| `str` at 0x5afec | store | 4 | `abs 0x132624` | `[0x132624]+1` |
| `ldr` at 0x5aff0 | load | 4 | `abs 0x5b100` | `-` |
| `ldr` at 0x5b000 | load | 4 | `abs 0x5b100` | `-` |
| `ldr` at 0x5b010 | load | 4 | `abs 0x5b104` | `-` |
| `ldr` at 0x5b014 | load | 4 | `abs 0x5b108` | `-` |
| `str` at 0x5b024 | store | 4 | `sp -0x14` | `ret(0xa8af0)` |
| `ldr` at 0x5b028 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5b034 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5b040 | load | 4 | `abs 0x5b10c` | `-` |
| `str` at 0x5b044 | store | 4 | `sp -0x30` | `0xf9de0` |
| `str` at 0x5b048 | store | 4 | `sp -0x2c` | `ret(0x1f8e0)` |
| `ldr` at 0x5b050 | load | 4 | `abs 0x5b0fc` | `-` |
| `ldr` at 0x5b060 | load | 4 | `abs 0x5b110` | `-` |
| `ldr` at 0x5b064 | load | 4 | `abs 0x5b114` | `-` |
| `str` at 0x5b078 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x5b07c | load | 4 | `abs 0x5b118` | `-` |
| `str` at 0x5b080 | store | 4 | `sp -0x2c` | `0x5dc` |
| `str` at 0x5b088 | store | 4 | `sp -0x28` | `0xf` |
| `str` at 0x5b090 | store | 4 | `sp -0x24` | `0xa` |
| `str` at 0x5b098 | store | 4 | `sp -0x20` | `0x9` |
| `ldr` at 0x5b09c | load | 4 | `abs 0x5b11c` | `-` |
| `ldr` at 0x5b0a0 | load | 4 | `abs 0x5b120` | `-` |
| `ldr` at 0x5b0a4 | load | 4 | `abs 0x5b124` | `-` |
| `str` at 0x5b0b4 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x5b0b8 | load | 4 | `abs 0x5b128` | `-` |
| `str` at 0x5b0bc | store | 4 | `sp -0x2c` | `0xc18` |
| `str` at 0x5b0c4 | store | 4 | `sp -0x28` | `0xf` |
| `str` at 0x5b0cc | store | 4 | `sp -0x24` | `0xa` |
| `str` at 0x5b0d4 | store | 4 | `sp -0x20` | `0x9` |
| `ldr` at 0x5b0d8 | load | 4 | `abs 0x5b12c` | `-` |
| `ldr` at 0x5b0dc | load | 4 | `abs 0x5b130` | `-` |
| `ldr` at 0x5b0e0 | load | 4 | `abs 0x5b134` | `-` |
| `ldm` at 0x5b0f0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x5b0f0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x5b0f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5afd8 | `(unknown callee) 0x200a0` | `0x9` | `0xf9dcc` | `0x43` | `0x0` |
| `bl` at 0x5affc | `(unknown callee) 0x19748` | `0x299858` | `0x0` | `0x104` | `[0x132624]+1` |
| `bl` at 0x5b00c | `(unknown callee) 0xa89d0` | `0x299858` | `0x11` | `0x0` | `?` |
| `bl` at 0x5b01c | `(unknown callee) 0xa8af0` | `0x299868` | `0xf9dd4` | `0x64` | `?` |
| `bl` at 0x5b038 | `(unknown callee) 0x1f8e0` | `ret(0xa8af0)` | `?` | `?` | `ret(0xa8af0)` |
| `bl` at 0x5b05c | `(unknown callee) 0x200a0` | `0x9` | `0xf9dcc` | `0x54` | `0x0` |
| `bl` at 0x5b070 | `(unknown callee) 0x20bb4` | `0x299900` | `0xf9e04` | `0x1` | `0x9` |
| `bl` at 0x5b0ac | `(unknown callee) 0x2066c` | `0x299958` | `0xf9e10` | `0x5b75c` | `0x0` |
| `bl` at 0x5b0e8 | `(unknown callee) 0x2066c` | `0x299954` | `0xf9e1c` | `0x5b158` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5afbc | `[0x132624] == 0x0` | `[0x132624] != 0x0` |
| `beq` at 0x5b030 | `ret(0xa8af0) == 0x0` | `ret(0xa8af0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x5b0f0 | `r0` | `ret(0x2066c)` |

## `L1NetActive`

- `L1NetActive`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5b138 to 0x5b158.
  Established. The absolute byte at `0x299860` is read (`ldrb` at 0x5b148).
  The included `r0` return record carries `[0x299860]:1` (`ldm` at 0x5b150).
  Network active-state meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x5b144 | load | 4 | `abs 0x5b154` | `-` |
| `ldrb` at 0x5b148 | load | 1 | `abs 0x299860` | `-` |
| `ldm` at 0x5b150 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x5b150 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x5b150 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x5b150 | `r0` | `[0x299860]:1` |

## `B2BActive`

- `B2BActive`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6f4d4 to 0x6f580.
  Established. The stack word at `sp -0x18` is first cleared (`str` at 0x6f4ec).
  An included later assignment writes `0x1` there (`str` at 0x6f568).
  The included return record reports an unknown integer in `r0` (`ldm` at 0x6f578).
  Board-link availability criteria remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6f4e4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6f4ec | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x6f4f0 | load | 4 | `abs 0x6f57c` | `-` |
| `ldr` at 0x6f4f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f508 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6f514 | load | 4 | `abs 0x6f57c` | `-` |
| `ldr` at 0x6f518 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f52c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6f530 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6f53c | load | 4 | `abs 0x6f57c` | `-` |
| `ldr` at 0x6f540 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f554 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6f558 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6f568 | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0x6f56c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x6f578 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6f578 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6f578 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6f510 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6f538 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6f560 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6f578 | `r0` | `?` |

## `B2BConfigured`

- `B2BConfigured`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6f580 to 0x6f5c8.
  Established. Entry `r0` is saved as a word at `sp -0x14` (`str` at 0x6f590).
  The word load at `0x6f5ac` has an unknown effective target (`ldr` at 0x6f5ac).
  The included `r0` return value is unknown (`ldm` at 0x6f5c0).
  Board-link configuration interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6f590 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6f594 | load | 4 | `abs 0x6f5c4` | `-` |
| `ldr` at 0x6f598 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f5ac | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x6f5c0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x6f5c0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6f5c0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6f5c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6f5c0 | `r0` | `?` |

## `RouteInit`

- `RouteInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa44f0 to 0xa45c8.
  Established. The unnamed `0xa75f8` call carries `0x29a984`, `0xffff017`, `0x1`, `0xa45c8` (`bl` at 0xa454c);
  its zero result takes `0xa4590`, while nonzero falls through (`beq` at 0xa4560).
  The absolute word at `0x133bcc` receives `[0x133bcc]+1` (`str` at 0xa45a0).
  Route registration behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xa4500 | load | 4 | `abs 0xa45ac` | `-` |
| `ldr` at 0xa4504 | load | 4 | `abs 0x133bcc` | `-` |
| `ldr` at 0xa4510 | load | 4 | `abs 0xa45b0` | `-` |
| `str` at 0xa4514 | store | 4 | `sp -0x1c` | `0x113460` |
| `ldr` at 0xa451c | load | 4 | `abs 0xa45b4` | `-` |
| `ldr` at 0xa452c | load | 4 | `abs 0xa45b8` | `-` |
| `ldr` at 0xa453c | load | 4 | `abs 0xa45b8` | `-` |
| `ldr` at 0xa4540 | load | 4 | `abs 0xa45bc` | `-` |
| `ldr` at 0xa4548 | load | 4 | `abs 0xa45c0` | `-` |
| `str` at 0xa4554 | store | 4 | `sp -0x14` | `ret(0xa75f8)` |
| `ldr` at 0xa4558 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa4564 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa4570 | load | 4 | `abs 0xa45c4` | `-` |
| `str` at 0xa4574 | store | 4 | `sp -0x1c` | `0x11349c` |
| `str` at 0xa4578 | store | 4 | `sp -0x18` | `ret(0x1f8e0)` |
| `ldr` at 0xa4580 | load | 4 | `abs 0xa45b4` | `-` |
| `ldr` at 0xa4590 | load | 4 | `abs 0xa45ac` | `-` |
| `ldr` at 0xa4594 | load | 4 | `abs 0xa45ac` | `-` |
| `ldr` at 0xa4598 | load | 4 | `abs 0x133bcc` | `-` |
| `str` at 0xa45a0 | store | 4 | `abs 0x133bcc` | `[0x133bcc]+1` |
| `ldm` at 0xa45a8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa45a8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa45a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa4528 | `(unknown callee) 0x200a0` | `0x9` | `0x113484` | `0x2a` | `0x0` |
| `bl` at 0xa4538 | `(unknown callee) 0xa89d0` | `0x29a984` | `0x17` | `0x0` | `?` |
| `bl` at 0xa454c | `(unknown callee) 0xa75f8` | `0x29a984` | `0xffff017` | `0x1` | `0xa45c8` |
| `bl` at 0xa4568 | `(unknown callee) 0x1f8e0` | `ret(0xa75f8)` | `?` | `?` | `ret(0xa75f8)` |
| `bl` at 0xa458c | `(unknown callee) 0x200a0` | `0x9` | `0x113484` | `0x38` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa450c | `[0x133bcc] == 0x0` | `[0x133bcc] != 0x0` |
| `beq` at 0xa4560 | `ret(0xa75f8) == 0x0` | `ret(0xa75f8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa45a8 | `r0` | `?` |

## `RouteIRouter`

- `RouteIRouter`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa45c8 to 0xa4790.
  Established. The word through entry `r0 +0x1c` is read (`ldr` at 0xa45e0).
  The unnamed `0xaa424` call carries `0x29a984`, `sp-24`, `0x0`, `0x1` (`bl` at 0xa4730).
  A word store through `ret(0xaa424) +0xc` writes `[[sp-20]+12]` on an included path (`str` at 0xa4760).
  Route packet transformation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa45d8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa45dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa45e0 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0xa45e4 | store | 4 | `sp -0x18` | `[r0+28]` |
| `str` at 0xa45ec | store | 4 | `sp -0x220` | `0x0` |
| `ldr` at 0xa45f0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa45f4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa45f8 | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0xa4604 | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0xa4610 | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0xa4614 | load | 4 | `abs 0xa4780` | `-` |
| `strb` at 0xa462c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xa4630 | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0xa4634 | load | 4 | `abs 0xa4780` | `-` |
| `strb` at 0xa464c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xa4650 | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0xa4654 | load | 4 | `abs 0xa4784` | `-` |
| `str` at 0xa4668 | store | 4 | `sp -0x224` | `?` |
| `ldr` at 0xa466c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa4670 | load | 4 | `sp -0x220` | `-` |
| `ldrb` at 0xa4684 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa4694 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa4698 | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0xa46ac | load | 4 | `(unknown)` | `-` |
| `str` at 0xa46b0 | store | 4 | `sp -0x228` | `?` |
| `ldr` at 0xa46b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa46bc | load | 4 | `sp -0x220` | `-` |
| `str` at 0xa46c4 | store | 4 | `sp -0x228` | `ret(0xaadf4)` |
| `ldr` at 0xa46c8 | load | 4 | `sp -0x228` | `-` |
| `ldr` at 0xa46cc | load | 4 | `sp -0x224` | `-` |
| `str` at 0xa46d0 | store | 4 | `[sp-548] +0x0` | `?` |
| `ldr` at 0xa46d4 | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0xa46d8 | load | 4 | `abs 0xa4780` | `-` |
| `strh` at 0xa46f0 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0xa46f4 | load | 4 | `sp -0x220` | `-` |
| `str` at 0xa46fc | store | 4 | `sp -0x220` | `[sp-544]+1` |
| `str` at 0xa470c | store | 4 | `sp -0x234` | `0x2` |
| `ldr` at 0xa4710 | load | 4 | `sp -0x220` | `-` |
| `str` at 0xa4714 | store | 4 | `sp -0x230` | `?` |
| `str` at 0xa471c | store | 4 | `sp -0x22c` | `sp-540` |
| `ldr` at 0xa4720 | load | 4 | `abs 0xa4788` | `-` |
| `str` at 0xa4738 | store | 4 | `sp -0x1c` | `ret(0xaa424)` |
| `ldr` at 0xa473c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa4748 | load | 4 | `abs 0xa478c` | `-` |
| `ldr` at 0xa4754 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa4758 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa475c | load | 4 | `[sp-20] +0xc` | `-` |
| `str` at 0xa4760 | store | 4 | `ret(0xaa424) +0xc` | `[[sp-20]+12]` |
| `ldr` at 0xa4764 | load | 4 | `abs 0xa4788` | `-` |
| `ldr` at 0xa4768 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa4770 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xa477c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa477c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa477c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa46c0 | `(unknown callee) 0xaadf4` | `?` | `?` | `?` | `?` |
| `bl` at 0xa4730 | `(unknown callee) 0xaa424` | `0x29a984` | `sp-24` | `0x0` | `0x1` |
| `bl` at 0xa474c | `(unknown callee) 0x1e4ac` | `0x1134cc` | `?` | `?` | `ret(0xaa424)` |
| `bl` at 0xa476c | `(unknown callee) 0xa9c40` | `0x29a984` | `[sp-28]` | `[[sp-20]+12]` | `ret(0xaa424)` |
| `bl` at 0xa4774 | `(unknown callee) 0xa8b98` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xa4600 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bgt` at 0xa460c | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0xa4690 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xa4744 | `ret(0xaa424) != 0x0` | `ret(0xaa424) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa477c | `r0` | `ret(0xa8b98)` |

## `RouteProcessIn`

- `RouteProcessIn`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa4790 to 0xa4940.
  Established. The byte through entry `r0 +0x16` is read (`ldrb` at 0xa47ac).
  The condition `[0x133bd0] != 0x0` takes `0xa47e4`, while zero falls through (`bne` at 0xa47cc).
  An included absolute word store writes `ret(0x6a0f8)` to `0x133bd0` (`str` at 0xa47e0).
  Inbound route field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa47a0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xa47a4 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xa47a8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa47ac | load | 1 | `r0 +0x16` | `-` |
| `str` at 0xa47bc | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xa47c0 | load | 4 | `abs 0xa493c` | `-` |
| `ldr` at 0xa47c4 | load | 4 | `abs 0x133bd0` | `-` |
| `ldr` at 0xa47dc | load | 4 | `abs 0xa493c` | `-` |
| `str` at 0xa47e0 | store | 4 | `abs 0x133bd0` | `ret(0x6a0f8)` |
| `str` at 0xa47e8 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xa47f0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xa47f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa4800 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa480c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4810 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xa4824 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa4834 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4838 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa484c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa485c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4860 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa4874 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa487c | load | 4 | `abs 0xa493c` | `-` |
| `ldr` at 0xa4880 | load | 4 | `abs 0x133bd0` | `-` |
| `ldr` at 0xa4884 | load | 4 | `[0x133bd0] +0xc` | `-` |
| `ldr` at 0xa4894 | load | 4 | `abs 0xa493c` | `-` |
| `ldr` at 0xa4898 | load | 4 | `abs 0x133bd0` | `-` |
| `ldr` at 0xa489c | load | 4 | `[0x133bd0] +0x8` | `-` |
| `ldr` at 0xa48ac | load | 4 | `abs 0xa493c` | `-` |
| `ldr` at 0xa48b0 | load | 4 | `abs 0x133bd0` | `-` |
| `ldr` at 0xa48b4 | load | 4 | `[0x133bd0] +0x10` | `-` |
| `ldr` at 0xa48c4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa48d4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xa48d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa48dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa48f0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xa48f4 | store | 4 | `(unknown)` | `[sp-44]` |
| `ldr` at 0xa48f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa48fc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa4918 | store | 4 | `(unknown)` | `ret(0x194f0)` |
| `str` at 0xa4920 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0xa4924 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa492c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldm` at 0xa4938 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xa4938 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa4938 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa4938 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa47d4 | `(unknown callee) 0x6a0f8` | `0x0` | `r1` | `r2` | `[0x133bd0]` |
| `bl` at 0xa4864 | `(unknown callee) 0xaadf4` | `?` | `?` | `?` | `?` |
| `bl` at 0xa4914 | `(unknown callee) 0x194f0` | `0x0` | `0x30` | `[sp-24]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa47cc | `[0x133bd0] != 0x0` | `[0x133bd0] == 0x0` |
| `bne` at 0xa47fc | `Z == 0` | `not (Z == 0)` |
| `bgt` at 0xa4808 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0xa4830 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xa4854 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xa4870 | `ret(0xaadf4) != 0x0` | `ret(0xaadf4) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa4938 | `r0` | `?` |

## `RouteProcessOut`

- `RouteProcessOut`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa4940 to 0xa4aa4.
  Established. The byte through entry `r0 +0x16` is read (`ldrb` at 0xa495c).
  An included word assignment at `sp -0x1c` writes `0x3f` (`str` at 0xa49a0).
  A later store writes `[sp-40]` to an unknown word target (`str` at 0xa4a7c).
  Outbound route modification policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa4950 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa4954 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xa4958 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa495c | load | 1 | `r0 +0x16` | `-` |
| `str` at 0xa496c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xa4970 | load | 4 | `abs 0xa4aa0` | `-` |
| `ldr` at 0xa4974 | load | 4 | `abs 0x133bd4` | `-` |
| `ldr` at 0xa498c | load | 4 | `abs 0xa4aa0` | `-` |
| `str` at 0xa4990 | store | 4 | `abs 0x133bd4` | `ret(0x6a0f8)` |
| `str` at 0xa4998 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xa49a0 | store | 4 | `sp -0x1c` | `0x3f` |
| `ldr` at 0xa49a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa49b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa49bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa49c0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa49d4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa49e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa49e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa49fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa4a0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa4a10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa4a28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa4a2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa4a40 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa4a44 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xa4a48 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa4a50 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa4a5c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xa4a60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa4a64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa4a78 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa4a7c | store | 4 | `(unknown)` | `[sp-40]` |
| `str` at 0xa4a84 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xa4a88 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa4a90 | store | 4 | `sp -0x1c` | `[sp-28]-1` |
| `ldm` at 0xa4a9c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa4a9c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa4a9c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa4984 | `(unknown callee) 0x6a0f8` | `0x0` | `r1` | `r2` | `[0x133bd4]` |
| `bl` at 0xa4a14 | `(unknown callee) 0xaadf4` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa497c | `[0x133bd4] != 0x0` | `[0x133bd4] == 0x0` |
| `bne` at 0xa49ac | `Z == 0` | `not (Z == 0)` |
| `blt` at 0xa49b8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xa49e0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xa4a04 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xa4a20 | `ret(0xaadf4) != 0x0` | `ret(0xaadf4) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa4a9c | `r0` | `?` |

## `SysComCfg`

- `SysComCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa4308 to 0xa44f0.
  Established. The absolute word at `0x29a5e4` is read at two sites (`ldr` at 0xa4324, `ldr` at 0xa435c).
  An included word store through entry `r0 +0x0` writes `ret(0x1943c)` (`str` at 0xa4344).
  Equality `[sp-36] == 0x1` takes `0xa43fc`, while inequality falls through (`beq` at 0xa43e4).
  Communication configuration formatting remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa4318 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xa431c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4320 | load | 4 | `abs 0xa44d4` | `-` |
| `ldr` at 0xa4324 | load | 4 | `abs 0x29a5e4` | `-` |
| `str` at 0xa4344 | store | 4 | `r0 +0x0` | `ret(0x1943c)` |
| `ldr` at 0xa4348 | load | 4 | `r0 +0x0` | `-` |
| `str` at 0xa434c | store | 4 | `sp -0x1c` | `[r0]` |
| `str` at 0xa4354 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xa4358 | load | 4 | `abs 0xa44d4` | `-` |
| `ldr` at 0xa435c | load | 4 | `abs 0x29a5e4` | `-` |
| `ldr` at 0xa4360 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa436c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa4370 | load | 4 | `abs 0xa44d8` | `-` |
| `ldr` at 0xa4374 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa437c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa438c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa4394 | store | 4 | `sp -0x1c` | `([sp-28] + ret(0x19538))` |
| `ldr` at 0xa4398 | load | 4 | `abs 0xa44d4` | `-` |
| `ldr` at 0xa439c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa43c0 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa43c8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa43cc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xa43d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa43dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa43ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa43f0 | load | 4 | `abs 0xa44dc` | `-` |
| `ldr` at 0xa43fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa4400 | load | 4 | `abs 0xa44e0` | `-` |
| `ldr` at 0xa4408 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa4418 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa4420 | store | 4 | `sp -0x1c` | `([sp-28] + ret(0x19538))` |
| `ldr` at 0xa4424 | load | 4 | `abs 0xa44d4` | `-` |
| `ldr` at 0xa4428 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa444c | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa4454 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa4458 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xa445c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa4468 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa4478 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa447c | load | 4 | `abs 0xa44e4` | `-` |
| `ldr` at 0xa4488 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa448c | load | 4 | `abs 0xa44e8` | `-` |
| `ldr` at 0xa4494 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa4498 | load | 4 | `abs 0xa44ec` | `-` |
| `ldr` at 0xa44a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa44b0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa44b8 | store | 4 | `sp -0x1c` | `([sp-28] + ret(0x19538))` |
| `ldr` at 0xa44bc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa44c4 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldm` at 0xa44d0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xa44d0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa44d0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa44d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa433c | `(unknown callee) 0x1943c` | `?` | `r1` | `[0x29a5e4]` | `?` |
| `bl` at 0xa4378 | `(unknown callee) 0x197a8` | `?` | `0x113414` | `?` | `?` |
| `bl` at 0xa4380 | `(unknown callee) 0x19538` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0xa43f4 | `(unknown callee) 0x19808` | `[sp-28]` | `0x113420` | `?` | `[sp-36]` |
| `bl` at 0xa4404 | `(unknown callee) 0x19808` | `[sp-28]` | `0x113430` | `?` | `[sp-36]` |
| `bl` at 0xa440c | `(unknown callee) 0x19538` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0xa4480 | `(unknown callee) 0x19808` | `[sp-28]` | `0x113440` | `?` | `[sp-40]` |
| `bl` at 0xa4490 | `(unknown callee) 0x19808` | `[sp-28]` | `0x113450` | `?` | `[sp-40]` |
| `bl` at 0xa449c | `(unknown callee) 0x195e0` | `[sp-28]` | `0x1133fc` | `?` | `?` |
| `bl` at 0xa44a4 | `(unknown callee) 0x19538` | `[sp-28]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xa4368 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xa43d8 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0xa43e4 | `[sp-36] == 0x1` | `[sp-36] != 0x1` |
| `beq` at 0xa4464 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `beq` at 0xa4470 | `[sp-40] == 0x1` | `[sp-40] != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa44d0 | `r0` | `?` |

## `IRouterInit`

- `IRouterInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa88fc to 0xa89d0.
  Established. A zero `[sp-21]:1` takes `0xa8934`, while nonzero falls through (`beq` at 0xa8920).
  Separate included word stores write `0x1` and `0x2` to `0x133c00` (`str` at 0xa892c, `str` at 0xa893c).
  The unnamed `0xa89d0` call carries `0x29aaac`, `0x1`, `0x3`, unknown (`bl` at 0xa8978).
  Router initialization semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa890c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa8914 | store | 4 | `sp -0x18` | `0x1` |
| `ldrb` at 0xa8918 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xa8924 | load | 4 | `abs 0xa89b4` | `-` |
| `str` at 0xa892c | store | 4 | `abs 0x133c00` | `0x1` |
| `ldr` at 0xa8934 | load | 4 | `abs 0xa89b4` | `-` |
| `str` at 0xa893c | store | 4 | `abs 0x133c00` | `0x2` |
| `str` at 0xa8944 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xa8948 | load | 4 | `abs 0xa89b8` | `-` |
| `ldr` at 0xa894c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa8968 | store | 4 | `abs 0x29aab4` | `?` |
| `ldr` at 0xa896c | load | 4 | `abs 0xa89bc` | `-` |
| `ldr` at 0xa897c | load | 4 | `abs 0xa89c0` | `-` |
| `ldr` at 0xa8980 | load | 4 | `abs 0xa89c4` | `-` |
| `ldr` at 0xa8990 | load | 4 | `abs 0xa89c8` | `-` |
| `ldr` at 0xa8994 | load | 4 | `abs 0xa89cc` | `-` |
| `ldr` at 0xa89a4 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xa89b0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa89b0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa89b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa8978 | `(unknown callee) 0xa89d0` | `0x29aaac` | `0x1` | `0x3` | `?` |
| `bl` at 0xa898c | `(unknown callee) 0x20bb4` | `0x133c24` | `0x113f1c` | `0x1` | `0x9` |
| `bl` at 0xa89a0 | `(unknown callee) 0x20bb4` | `0x133c78` | `0x113f28` | `0x1` | `0x9` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa8920 | `[sp-21]:1 == 0x0` | `[sp-21]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa89b0 | `r0` | `[sp-24]` |

## `IRouterInitID`

- `IRouterInitID`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa89d0 to 0xa8abc.
  Established. The signed condition `[[sp-20]] >= 0x0` takes `0xa8a60`, while negative values fall through (`bge` at 0xa8a30).
  The byte through `[sp-20] +0x4` receives `([sp-28] & 0xff)` (`strb` at 0xa8a98).
  The halfword through `[sp-20] +0x6` is cleared (`strh` at 0xa8aa4).
  Router identity field interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa89e0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa89e4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa89e8 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xa89ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa89f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa89f4 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0xa8a00 | load | 4 | `abs 0xa8ab8` | `-` |
| `ldr` at 0xa8a04 | load | 4 | `abs 0x29aab4` | `-` |
| `str` at 0xa8a18 | store | 4 | `r0 +0x0` | `?` |
| `ldr` at 0xa8a1c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa8a20 | store | 4 | `sp -0x20` | `[sp-20]` |
| `ldr` at 0xa8a24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8a28 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xa8a34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8a38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8a3c | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xa8a44 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa8a54 | store | 4 | `[sp-20] +0x0` | `?` |
| `str` at 0xa8a58 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xa8a60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8a64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8a68 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xa8a70 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa8a7c | store | 4 | `[sp-20] +0x0` | `?` |
| `str` at 0xa8a80 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xa8a84 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa8a88 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa8a8c | store | 4 | `[sp-32] +0x0` | `[sp-36]` |
| `ldr` at 0xa8a90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8a94 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xa8a98 | store | 1 | `[sp-20] +0x4` | `([sp-28] & 0xff)` |
| `ldr` at 0xa8a9c | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xa8aa4 | store | 2 | `[sp-20] +0x6` | `0x0` |
| `ldm` at 0xa8ab4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa8ab4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa8ab4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xa8a30 | `[[sp-20]] >= 0x0 (signed, 32-bit)` | `[[sp-20]] < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa8ab4 | `r0` | `0x0` |

## `IRouterMakePacket`

- `IRouterMakePacket`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaa424 to 0xaa4f0.
  Established. The halfword through entry `r0 +0x6` receives `([r0+6]:2+1 & 0xffff)` (`strh` at 0xaa474).
  The unnamed `0xa80e0` call carries `sp-36`, `[sp-32]`, `[sp-40]`, `[r0+4]:1` (`bl` at 0xaa4c0);
  its zero result takes `0xaa4d8`, while nonzero falls through (`beq` at 0xaa4cc).
  Packet construction semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaa434 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xaa438 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xaa43c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xaa440 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xaa448 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xaa450 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaa454 | load | 1 | `r0 +0x4` | `-` |
| `ldr` at 0xaa458 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xaa45c | store | 4 | `sp -0x4c` | `r2` |
| `str` at 0xaa464 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xaa468 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xaa46c | load | 2 | `r0 +0x6` | `-` |
| `strh` at 0xaa474 | store | 2 | `r0 +0x6` | `([r0+6]:2+1 & 0xffff)` |
| `str` at 0xaa480 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xaa484 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xaa488 | store | 4 | `sp -0x40` | `[sp-24]` |
| `ldr` at 0xaa48c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaa490 | store | 4 | `sp -0x3c` | `[sp-20]` |
| `ldr` at 0xaa494 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xaa498 | store | 4 | `sp -0x38` | `[sp+4]` |
| `ldr` at 0xaa49c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xaa4a0 | store | 4 | `sp -0x34` | `[sp+8]` |
| `str` at 0xaa4a8 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0xaa4b0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xaa4b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xaa4b8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xaa4d4 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xaa4dc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xaa4e0 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xaa4ec | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xaa4ec | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xaa4ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaa4c0 | `(unknown callee) 0xa80e0` | `sp-36` | `[sp-32]` | `[sp-40]` | `[r0+4]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xaa4cc | `ret(0xa80e0) == 0x0` | `ret(0xa80e0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xaa4ec | `r0` | `[sp-36]` |

## `IRouterSendPacket`

- `IRouterSendPacket`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa9c40 to 0xa9d18.
  Established. The byte through entry `r1 +0x16` is loaded (`ldrb` at 0xa9c5c);
  its equality with `0x2` takes `0xa9ce0`, while inequality falls through (`beq` at 0xa9c84).
  The unnamed `0xaa090` call carries `0x29aaac`, `ret(0xa9240)`, `[sp-24]`, `0x0` (`bl` at 0xa9cd8).
  Packet send result policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa9c50 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa9c54 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xa9c58 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa9c5c | load | 1 | `r1 +0x16` | `-` |
| `str` at 0xa9c60 | store | 4 | `sp -0x20` | `[r1+22]:1` |
| `ldr` at 0xa9c64 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa9c70 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa9c7c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa9c8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa9c90 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa9ca0 | store | 4 | `sp -0x1c` | `ret(0xa9240)` |
| `ldr` at 0xa9ca4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa9cb0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa9cb4 | load | 1 | `[sp-24] +0x16` | `-` |
| `str` at 0xa9cc4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xa9cc8 | load | 4 | `abs 0xa9d14` | `-` |
| `ldr` at 0xa9ccc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa9cd0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa9ce0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa9ce4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa9cf0 | store | 4 | `sp -0x1c` | `ret(0xa98b0)` |
| `str` at 0xa9cf8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xa9cfc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa9d04 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xa9d10 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa9d10 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa9d10 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa9c98 | `(unknown callee) 0xa9240` | `r0` | `r1` | `0x1` | `[r1+22]:1` |
| `bl` at 0xa9cd8 | `(unknown callee) 0xaa090` | `0x29aaac` | `ret(0xa9240)` | `[sp-24]` | `0x0` |
| `bl` at 0xa9ce8 | `(unknown callee) 0xa98b0` | `r0` | `r1` | `r2` | `[r1+22]:1` |
| `bl` at 0xa9d00 | `(unknown callee) 0xa8b98` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xa9c6c | `[r1+22]:1 < 0x0 (signed, 32-bit)` | `[r1+22]:1 >= 0x0 (signed, 32-bit)` |
| `ble` at 0xa9c78 | `[r1+22]:1 <= 0x1 (signed, 32-bit)` | `[r1+22]:1 > 0x1 (signed, 32-bit)` |
| `beq` at 0xa9c84 | `[r1+22]:1 == 0x2` | `[r1+22]:1 != 0x2` |
| `beq` at 0xa9cac | `ret(0xa9240) == 0x0` | `ret(0xa9240) != 0x0` |
| `bne` at 0xa9cbc | `[[sp-24]+22]:1 != 0x0` | `[[sp-24]+22]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa9d10 | `r0` | `[sp-28]` |

## `IRouterRequest`

- `IRouterRequest`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa9f4c to 0xaa090.
  Established. The halfword through entry `r0 +0x6` is read (`ldrh` at 0xa9f80).
  An included word store through `[sp+8] +0x0` writes `[r0+6]:2` (`str` at 0xa9f84).
  The unnamed `0xa9240` call carries `[sp-20]`, `[sp-40]`, `0x1`, `ret(0xa80e0)` (`bl` at 0xaa068).
  Request submission behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa9f5c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa9f60 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa9f64 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xa9f68 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xa9f6c | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xa9f78 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xa9f7c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xa9f80 | load | 2 | `r0 +0x6` | `-` |
| `str` at 0xa9f84 | store | 4 | `[sp+8] +0x0` | `[r0+6]:2` |
| `str` at 0xa9f8c | store | 4 | `sp -0x30` | `sp-40` |
| `ldr` at 0xa9f90 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa9f94 | load | 1 | `(unknown)` | `-` |
| `str` at 0xa9f98 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xa9f9c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa9fa0 | store | 4 | `sp -0x5c` | `?` |
| `str` at 0xa9fa8 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xa9fac | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xa9fb0 | load | 2 | `(unknown)` | `-` |
| `strh` at 0xa9fb8 | store | 2 | `(unknown)` | `?` |
| `str` at 0xa9fc4 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xa9fc8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa9fcc | store | 4 | `sp -0x50` | `[sp-24]` |
| `ldr` at 0xa9fd0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa9fdc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa9fe0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xa9fec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa9ff0 | store | 4 | `sp -0x38` | `[sp-28]` |
| `ldr` at 0xa9ff8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa9ffc | store | 4 | `sp -0x38` | `[sp-20]` |
| `ldr` at 0xaa000 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xaa004 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xaa008 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xaa00c | store | 4 | `sp -0x48` | `[sp]` |
| `ldr` at 0xaa010 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xaa014 | store | 4 | `sp -0x44` | `[sp+4]` |
| `ldr` at 0xaa018 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xaa01c | store | 4 | `sp -0x40` | `[sp+12]` |
| `str` at 0xaa024 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xaa028 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xaa034 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xaa040 | store | 4 | `sp -0x24` | `ret(0xa80e0)` |
| `ldr` at 0xaa044 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xaa050 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xaa054 | store | 4 | `sp -0x2c` | `ret(0xa80e0)` |
| `ldr` at 0xaa05c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaa060 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xaa070 | store | 4 | `sp -0x24` | `ret(0xa9240)` |
| `ldr` at 0xaa074 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaa07c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xaa080 | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0xaa084 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0xaa08c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xaa08c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xaa08c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaa038 | `(unknown callee) 0xa80e0` | `[sp-48]` | `0x0` | `0x0` | `[sp-52]` |
| `bl` at 0xaa068 | `(unknown callee) 0xa9240` | `[sp-20]` | `[sp-40]` | `0x1` | `ret(0xa80e0)` |
| `bl` at 0xaa078 | `(unknown callee) 0xa8b98` | `[sp-40]` | `?` | `?` | `ret(0xa9240)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa9f74 | `[sp+8] == 0x0` | `[sp+8] != 0x0` |
| `beq` at 0xa9fd8 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `beq` at 0xa9fe8 | `[[sp-28]] == 0x0` | `[[sp-28]] != 0x0` |
| `beq` at 0xaa04c | `ret(0xa80e0) == 0x0` | `ret(0xa80e0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xaa08c | `r0` | `?` |

## `IRouterResponse`

- `IRouterResponse`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaa090 to 0xaa20c.
  Established. A zero entry `r0` takes `0xaa0d4`, while nonzero falls through (`beq` at 0xaa0c0).
  The halfword through entry `r2 +0x22` is read (`ldrh` at 0xaa0e4).
  The unnamed `0xa80e0` call carries `[sp-48]`, `0x1`, `0x0`, `[sp-52]` (`bl` at 0xaa1b0).
  Response packet encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaa0a0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xaa0a4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xaa0a8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xaa0ac | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xaa0b4 | store | 4 | `sp -0x30` | `sp-36` |
| `ldr` at 0xaa0b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaa0c4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaa0c8 | load | 1 | `r0 +0x4` | `-` |
| `str` at 0xaa0cc | store | 4 | `sp -0x34` | `[r0+4]:1` |
| `ldr` at 0xaa0d4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xaa0d8 | load | 1 | `r2 +0x17` | `-` |
| `str` at 0xaa0dc | store | 4 | `sp -0x34` | `[r2+23]:1` |
| `ldr` at 0xaa0e0 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xaa0e4 | load | 2 | `r2 +0x22` | `-` |
| `str` at 0xaa0e8 | store | 4 | `sp -0x5c` | `[r2+34]:2` |
| `ldr` at 0xaa0ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaa0f8 | load | 4 | `abs 0xaa208` | `-` |
| `str` at 0xaa128 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xaa12c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xaa130 | load | 1 | `[sp-28] +0x14` | `-` |
| `ldr` at 0xaa134 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xaa138 | load | 1 | `[sp-28] +0x21` | `-` |
| `str` at 0xaa144 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xaa148 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xaa150 | store | 4 | `sp -0x50` | `[sp-28]+28` |
| `ldr` at 0xaa154 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaa158 | store | 4 | `sp -0x38` | `[sp-20]` |
| `ldr` at 0xaa15c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaa168 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xaa170 | store | 4 | `sp -0x38` | `[sp-28]+24` |
| `ldr` at 0xaa174 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xaa178 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xaa17c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xaa180 | store | 4 | `sp -0x48` | `[sp-32]` |
| `ldr` at 0xaa184 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xaa188 | store | 4 | `sp -0x44` | `[sp]` |
| `ldr` at 0xaa18c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaa190 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xaa194 | store | 4 | `sp -0x40` | `[[sp-28]+12]` |
| `str` at 0xaa19c | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xaa1a0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xaa1ac | load | 4 | `sp -0x34` | `-` |
| `str` at 0xaa1b8 | store | 4 | `sp -0x28` | `ret(0xa80e0)` |
| `ldr` at 0xaa1bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaa1c8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xaa1cc | store | 4 | `sp -0x2c` | `ret(0xa80e0)` |
| `ldr` at 0xaa1d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaa1d8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xaa1e8 | store | 4 | `sp -0x28` | `ret(0xa9240)` |
| `ldr` at 0xaa1ec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xaa1f4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xaa1f8 | store | 4 | `sp -0x2c` | `[sp-40]` |
| `ldr` at 0xaa1fc | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0xaa204 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xaa204 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xaa204 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaa1b0 | `(unknown callee) 0xa80e0` | `[sp-48]` | `0x1` | `0x0` | `[sp-52]` |
| `bl` at 0xaa1e0 | `(unknown callee) 0xa9240` | `[sp-20]` | `[sp-36]` | `0x1` | `ret(0xa80e0)` |
| `bl` at 0xaa1f0 | `(unknown callee) 0xa8b98` | `[sp-36]` | `?` | `?` | `ret(0xa9240)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xaa0c0 | `r0 == 0x0` | `r0 != 0x0` |
| `bne` at 0xaa164 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `beq` at 0xaa1c4 | `ret(0xa80e0) == 0x0` | `ret(0xa80e0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xaa204 | `r0` | `?` |
