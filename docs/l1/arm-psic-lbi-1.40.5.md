# ARM PSIC and LBI scan routines in 1.40.5

Read from `6.5.29/usr/cpu/firmware/sysco/l1.bin`, package `6.5.29:l1#4` (version 1.40.5),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53d448`.
Carrier SHA-256: `80f4a616601d187bc3839a2758f2e7f2d0db962ad7fd8f2d96d41bcd15bdf88c`.
Package SHA-256: `4ccd1a6e7054bc3cd56e962a60737d1c90339ffa93fcc8af0912b6c9c7d385dd`.
Inner executable SHA-256: `08ebc648bb298017c7e3d6049983f2a54a8fe56f1bb5738df88b82c38f75ac3a`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1257088 bytes.
Segment SHA-256: `dc7cc5d18fbdf816d328411825c9c01e26c3f8852f1012c7a642aea0767e2665`. Analysed routine spans use the exact
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
These routines have no unresolved computed transfers. Indirect call targets and numeric callee names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/psic-lbi-facts.txt) retain all tool sections.

## `psicInit`

- `psicInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdc038 to 0xdc258.
  Established. The equality branch takes `[0x143828] == 0x2adfb8` to `0xdc0a8` (`beq` at 0xdc064).
  Included four-byte global assignments write `0x1` at `0x14382c` and `ret(0x1b454)` at `0x143828`
  (`str` at 0xdc0c0, `str` at 0xdc0f4). The unnamed `0x1b4cc` call has `[[sp-36]+8]`, `0x1`,
  `[[sp-36]+4]`, `[sp-36]` (`bl` at 0xdc22c). Record layout and PSIC initialization effects are
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdc048 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xdc04c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xdc050 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0xdc054 | load | 4 | `abs 0xdc234` | `-` |
| `ldr` at 0xdc058 | load | 4 | `abs 0x143828` | `-` |
| `ldr` at 0xdc05c | load | 4 | `abs 0xdc238` | `-` |
| `ldr` at 0xdc068 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdc074 | load | 4 | `abs 0xdc23c` | `-` |
| `str` at 0xdc078 | store | 4 | `sp -0x34` | `0x135ac0` |
| `ldr` at 0xdc07c | load | 4 | `abs 0xdc234` | `-` |
| `ldr` at 0xdc080 | load | 4 | `abs 0x143828` | `-` |
| `str` at 0xdc084 | store | 4 | `sp -0x30` | `[0x143828]` |
| `ldr` at 0xdc088 | load | 4 | `abs 0xdc240` | `-` |
| `ldr` at 0xdc08c | load | 4 | `abs 0x14382c` | `-` |
| `str` at 0xdc090 | store | 4 | `sp -0x2c` | `[0x14382c]` |
| `ldr` at 0xdc098 | load | 4 | `abs 0xdc244` | `-` |
| `ldr` at 0xdc0a8 | load | 4 | `abs 0xdc240` | `-` |
| `ldr` at 0xdc0ac | load | 4 | `abs 0x14382c` | `-` |
| `ldr` at 0xdc0b8 | load | 4 | `abs 0xdc240` | `-` |
| `str` at 0xdc0c0 | store | 4 | `abs 0x14382c` | `0x1` |
| `ldr` at 0xdc0c4 | load | 4 | `abs 0xdc240` | `-` |
| `ldr` at 0xdc0c8 | load | 4 | `abs 0x14382c` | `-` |
| `str` at 0xdc0dc | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xdc0e0 | load | 4 | `abs 0xdc234` | `-` |
| `ldr` at 0xdc0e4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xdc0f4 | store | 4 | `abs 0x143828` | `ret(0x1b454)` |
| `ldr` at 0xdc0f8 | load | 4 | `abs 0x143828` | `-` |
| `ldr` at 0xdc104 | load | 4 | `abs 0xdc248` | `-` |
| `str` at 0xdc108 | store | 4 | `sp -0x34` | `0x135af8` |
| `ldr` at 0xdc10c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xdc110 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xdc118 | load | 4 | `abs 0xdc244` | `-` |
| `ldr` at 0xdc128 | load | 4 | `abs 0xdc234` | `-` |
| `ldr` at 0xdc12c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdc140 | load | 4 | `abs 0x143828` | `-` |
| `str` at 0xdc148 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xdc14c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdc150 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdc154 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0xdc158 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdc15c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdc160 | store | 4 | `[sp-36] +0x4` | `[sp-32]` |
| `ldr` at 0xdc164 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdc168 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdc16c | load | 4 | `[sp-36] +0x4` | `-` |
| `str` at 0xdc17c | store | 4 | `[sp-36] +0x8` | `ret(0x1b454)` |
| `ldr` at 0xdc180 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0xdc18c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdc194 | load | 4 | `abs 0xdc24c` | `-` |
| `ldr` at 0xdc198 | load | 4 | `abs 0xdc250` | `-` |
| `str` at 0xdc1c4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xdc1c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdc1d0 | load | 4 | `abs 0xdc24c` | `-` |
| `ldr` at 0xdc1d4 | load | 4 | `abs 0xdc250` | `-` |
| `ldr` at 0xdc1e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdc1f0 | load | 4 | `abs 0xdc254` | `-` |
| `str` at 0xdc1f4 | store | 4 | `(unknown)` | `0x135b3c` |
| `str` at 0xdc1f8 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0xdc1fc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdc200 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0xdc208 | load | 4 | `abs 0xdc244` | `-` |
| `ldr` at 0xdc218 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdc21c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdc220 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0xdc228 | load | 4 | `[sp-36] +0x4` | `-` |
| `ldmdb` at 0xdc230 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xdc230 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdc230 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdc230 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdc0a4 | `(unknown callee) 0x21eb8` | `0x9` | `0x135af0` | `0x58` | `0x0` |
| `bl` at 0xdc0ec | `(unknown callee) 0x1b454` | `[sp-40]` | `0x1` | `[0x14382c]` | `?` |
| `bl` at 0xdc124 | `(unknown callee) 0x21eb8` | `0x9` | `0x135af0` | `0x64` | `0x0` |
| `bl` at 0xdc174 | `(unknown callee) 0x1b454` | `[[sp-36]+4]` | `0x1` | `[sp-36]` | `[sp-36]` |
| `bl` at 0xdc19c | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x135b30` | `0x135b38` |
| `bl` at 0xdc1a8 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xdc1d8 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x135b30` | `0x135b38` |
| `bl` at 0xdc1e8 | `(unknown callee) 0x1b598` | `[sp-40]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xdc214 | `(unknown callee) 0x21eb8` | `0x9` | `0x135af0` | `0x70` | `0x0` |
| `bl` at 0xdc22c | `(unknown callee) 0x1b4cc` | `[[sp-36]+8]` | `0x1` | `[[sp-36]+4]` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xdc064 | `[0x143828] == 0x2adfb8` | `[0x143828] != 0x2adfb8` |
| `bne` at 0xdc070 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xdc0b4 | `[0x14382c] != 0x0` | `[0x14382c] == 0x0` |
| `bne` at 0xdc100 | `[0x143828] != 0x0` | `[0x143828] == 0x0` |
| `bne` at 0xdc188 | `[[sp-36]+8] != 0x0` | `[[sp-36]+8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdc230 | `r0` | `ret(0x1b4cc)` |

## `psicRead`

- `psicRead`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdc4b0 to 0xdc534.
  Established. Separate byte reads target `r1 +0x2` and `r1 +0x3` (`ldrb` at 0xdc4f8, `ldrb` at
  0xdc50c). The unnamed `0xdca34` call has unknown, `[r1+3]:1`, `0xa`, `r2` (`bl` at 0xdc520). The
  exit carries `ret(0xdca34)` (`ldmdb` at 0xdc52c). Read-register identities and peripheral effects
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdc4c0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdc4c4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xdc4c8 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xdc4cc | load | 4 | `abs 0xdc530` | `-` |
| `ldr` at 0xdc4d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc4e4 | load | 4 | `abs 0x143828` | `-` |
| `str` at 0xdc4ec | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xdc4f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc4f4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdc4f8 | load | 1 | `r1 +0x2` | `-` |
| `ldr` at 0xdc500 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xdc508 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdc50c | load | 1 | `r1 +0x3` | `-` |
| `ldr` at 0xdc510 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdc51c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xdc52c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdc52c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdc52c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdc520 | `(unknown callee) 0xdca34` | `?` | `[r1+3]:1` | `0xa` | `r2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdc52c | `r0` | `ret(0xdca34)` |

## `psicWrite`

- `psicWrite`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdc424 to 0xdc4b0.
  Established. The entry-r2 low byte is assigned at `sp -0x19` (`strb` at 0xdc440). Included byte
  loads read `r1 +0x2` and `r1 +0x3` (`ldrb` at 0xdc470, `ldrb` at 0xdc484). The unnamed `0xdc86c`
  call has unknown, `[r1+3]:1`, `0xa`, `(r2 & 0xff)` (`bl` at 0xdc49c). Write address decoding and
  hardware effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdc434 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdc438 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xdc440 | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `ldr` at 0xdc444 | load | 4 | `abs 0xdc4ac` | `-` |
| `ldr` at 0xdc448 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc45c | load | 4 | `abs 0x143828` | `-` |
| `str` at 0xdc464 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xdc468 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc46c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdc470 | load | 1 | `r1 +0x2` | `-` |
| `ldr` at 0xdc478 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xdc480 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdc484 | load | 1 | `r1 +0x3` | `-` |
| `ldrb` at 0xdc488 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xdc48c | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0xdc4a8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdc4a8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdc4a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdc49c | `(unknown callee) 0xdc86c` | `?` | `[r1+3]:1` | `0xa` | `(r2 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdc4a8 | `r0` | `ret(0xdc86c)` |

## `psicIntGet`

- `psicIntGet`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdc5c0 to 0xdc644.
  Established. Byte reads at `r1 +0x2` and `r1 +0x3` precede a four-byte load at an unknown
  effective target (`ldrb` at 0xdc608, `ldrb` at 0xdc61c, `ldr` at 0xdc620). The unnamed `0xdca34`
  call uses unknown, `[r1+3]:1`, `0xc`, `r2` (`bl` at 0xdc630). The exit carries `ret(0xdca34)`
  (`ldmdb` at 0xdc63c). Interrupt-register meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdc5d0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdc5d4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xdc5d8 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xdc5dc | load | 4 | `abs 0xdc640` | `-` |
| `ldr` at 0xdc5e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc5f4 | load | 4 | `abs 0x143828` | `-` |
| `str` at 0xdc5fc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xdc600 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc604 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdc608 | load | 1 | `r1 +0x2` | `-` |
| `ldr` at 0xdc610 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xdc618 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdc61c | load | 1 | `r1 +0x3` | `-` |
| `ldr` at 0xdc620 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdc62c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xdc63c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdc63c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdc63c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdc630 | `(unknown callee) 0xdca34` | `?` | `[r1+3]:1` | `0xc` | `r2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdc63c | `r0` | `ret(0xdca34)` |

## `psicIntSet`

- `psicIntSet`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdc534 to 0xdc5c0.
  Established. A byte store at `sp -0x19` retains `(r2 & 0xff)` (`strb` at 0xdc550). The unnamed
  `0xdc86c` call carries unknown, `[r1+3]:1`, `0xc`, `(r2 & 0xff)` (`bl` at 0xdc5ac); its result
  reaches the exit (`ldmdb` at 0xdc5b8). No conditional branches appear in this span.
  Interrupt-enable semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdc544 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdc548 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xdc550 | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `ldr` at 0xdc554 | load | 4 | `abs 0xdc5bc` | `-` |
| `ldr` at 0xdc558 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc56c | load | 4 | `abs 0x143828` | `-` |
| `str` at 0xdc574 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xdc578 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc57c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdc580 | load | 1 | `r1 +0x2` | `-` |
| `ldr` at 0xdc588 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xdc590 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdc594 | load | 1 | `r1 +0x3` | `-` |
| `ldrb` at 0xdc598 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xdc59c | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0xdc5b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdc5b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdc5b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdc5ac | `(unknown callee) 0xdc86c` | `?` | `[r1+3]:1` | `0xc` | `(r2 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdc5b8 | `r0` | `ret(0xdc86c)` |

## `psicInt`

- `psicInt`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdc644 to 0xdc86c.
  Established. The unnamed `0xdca34` call has `[sp-44]`, unknown, `0xd`, `sp-57` (`bl` at 0xdc6e4);
  its zero result takes the branch to `0xdc7a8` (`beq` at 0xdc6f8). A later unnamed `0xd7afc` call
  carries `0x0`, `[sp-28]`, `[sp-32]`, `[sp-33]:1` (`bl` at 0xdc7e0). A four-byte assignment at `sp -0x34` retains `([sp-52] + ret(0xd7afc))` (`str` at 0xdc7f0). Interrupt servicing effects and
  counts remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdc654 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xdc658 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xdc65c | store | 4 | `sp -0x20` | `r2` |
| `strb` at 0xdc660 | store | 1 | `sp -0x21` | `(r3 & 0xff)` |
| `ldr` at 0xdc664 | load | 4 | `abs 0xdc854` | `-` |
| `ldr` at 0xdc668 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdc67c | load | 4 | `abs 0x143828` | `-` |
| `str` at 0xdc684 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xdc688 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdc68c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xdc690 | load | 1 | `r1 +0x2` | `-` |
| `ldr` at 0xdc698 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0xdc6a0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xdc6a4 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xdc6ac | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0xdc6b4 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xdc6b8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xdc6c8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xdc6cc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xdc6d4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xdc6ec | store | 4 | `sp -0x30` | `ret(0xdca34)` |
| `ldr` at 0xdc6f0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdc6fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdc704 | load | 4 | `abs 0xdc858` | `-` |
| `ldr` at 0xdc708 | load | 4 | `abs 0xdc85c` | `-` |
| `str` at 0xdc734 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xdc738 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdc740 | load | 4 | `abs 0xdc858` | `-` |
| `ldr` at 0xdc744 | load | 4 | `abs 0xdc85c` | `-` |
| `ldr` at 0xdc750 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdc760 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdc76c | load | 4 | `abs 0xdc860` | `-` |
| `str` at 0xdc770 | store | 4 | `(unknown)` | `0x135b78` |
| `str` at 0xdc774 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0xdc778 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdc77c | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xdc780 | store | 4 | `(unknown)` | `[[sp-28]+12]` |
| `str` at 0xdc784 | store | 4 | `(unknown)` | `ret(0x21a38)` |
| `ldr` at 0xdc78c | load | 4 | `abs 0xdc864` | `-` |
| `ldr` at 0xdc79c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xdc7a0 | store | 4 | `sp -0x44` | `[sp-52]` |
| `ldr` at 0xdc7a8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xdc7ac | load | 1 | `sp -0x39` | `-` |
| `ldrb` at 0xdc7b0 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldrb` at 0xdc7c4 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0xdc7c8 | load | 1 | `sp -0x39` | `-` |
| `str` at 0xdc7cc | store | 4 | `sp -0x54` | `[sp-57]:1` |
| `ldr` at 0xdc7d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdc7d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc7e8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xdc7f0 | store | 4 | `sp -0x34` | `([sp-52] + ret(0xd7afc))` |
| `ldr` at 0xdc7f8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xdc7fc | store | 4 | `sp -0x44` | `[sp-52]` |
| `ldr` at 0xdc804 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xdc80c | store | 4 | `sp -0x38` | `[sp-56]+1` |
| `ldr` at 0xdc814 | load | 4 | `abs 0xdc868` | `-` |
| `str` at 0xdc818 | store | 4 | `sp -0x54` | `0x135ba0` |
| `ldr` at 0xdc81c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdc820 | load | 4 | `(unknown)` | `-` |
| `str` at 0xdc824 | store | 4 | `sp -0x50` | `?` |
| `ldrb` at 0xdc828 | load | 1 | `sp -0x39` | `-` |
| `str` at 0xdc82c | store | 4 | `sp -0x4c` | `[sp-57]:1` |
| `ldr` at 0xdc834 | load | 4 | `abs 0xdc864` | `-` |
| `ldr` at 0xdc844 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xdc848 | store | 4 | `sp -0x44` | `[sp-52]` |
| `ldr` at 0xdc84c | load | 4 | `sp -0x44` | `-` |
| `ldmdb` at 0xdc850 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xdc850 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdc850 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdc850 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdc6e4 | `(unknown callee) 0xdca34` | `[sp-44]` | `?` | `0xd` | `sp-57` |
| `bl` at 0xdc70c | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x135b30` | `0x135b38` |
| `bl` at 0xdc718 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xdc748 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x135b30` | `0x135b38` |
| `bl` at 0xdc758 | `(unknown callee) 0x1b598` | `[sp-64]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xdc764 | `(unknown callee) 0x21a38` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0xdc798 | `(unknown callee) 0x21eb8` | `0x3` | `0x135af0` | `0xe0` | `0x0` |
| `bl` at 0xdc7e0 | `(unknown callee) 0xd7afc` | `0x0` | `[sp-28]` | `[sp-32]` | `[sp-33]:1` |
| `bl` at 0xdc840 | `(unknown callee) 0x21eb8` | `0x3` | `0x135af0` | `0xeb` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xdc6c0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xdc6f8 | `ret(0xdca34) == 0x0` | `ret(0xdca34) != 0x0` |
| `beq` at 0xdc7c0 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdc850 | `r0` | `[sp-52]` |

## `lbi_tapmux_init`

- `lbi_tapmux_init`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf6004 to 0xf612c.
  Established. Included four-byte global assignments write `0x2` at `0x2ae1cc` and `ret(0x1b454)` at
  `0x2ae1c8` (`str` at 0xf6094, `str` at 0xf60bc). The unnamed `0x1b454` call has `[sp-36]`, `0x1`,
  `0x2ae1cc`, unknown (`bl` at 0xf60b4). A later four-byte store has unknown effective target and
  value (`str` at 0xf6108). TAP multiplexer initialization effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf6014 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xf6018 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xf601c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf6028 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xf602c | load | 4 | `abs 0xf6118` | `-` |
| `ldr` at 0xf6030 | load | 4 | `abs 0x2ae1c8` | `-` |
| `ldr` at 0xf603c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf6048 | load | 4 | `abs 0xf611c` | `-` |
| `str` at 0xf604c | store | 4 | `sp -0x30` | `0x13a470` |
| `ldr` at 0xf6050 | load | 4 | `abs 0xf6118` | `-` |
| `ldr` at 0xf6054 | load | 4 | `abs 0x2ae1c8` | `-` |
| `str` at 0xf6058 | store | 4 | `sp -0x2c` | `[0x2ae1c8]` |
| `ldr` at 0xf605c | load | 4 | `abs 0xf6120` | `-` |
| `ldr` at 0xf6060 | load | 4 | `abs 0x2ae1cc` | `-` |
| `str` at 0xf6064 | store | 4 | `sp -0x28` | `[0x2ae1cc]` |
| `ldr` at 0xf606c | load | 4 | `abs 0xf6124` | `-` |
| `ldr` at 0xf607c | load | 4 | `abs 0xf6120` | `-` |
| `ldr` at 0xf6080 | load | 4 | `abs 0x2ae1cc` | `-` |
| `ldr` at 0xf608c | load | 4 | `abs 0xf6120` | `-` |
| `str` at 0xf6094 | store | 4 | `abs 0x2ae1cc` | `0x2` |
| `ldr` at 0xf6098 | load | 4 | `abs 0xf6120` | `-` |
| `ldr` at 0xf609c | load | 4 | `abs 0x2ae1cc` | `-` |
| `str` at 0xf60a4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf60a8 | load | 4 | `abs 0xf6118` | `-` |
| `ldr` at 0xf60ac | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf60bc | store | 4 | `abs 0x2ae1c8` | `ret(0x1b454)` |
| `ldr` at 0xf60c0 | load | 4 | `abs 0x2ae1c8` | `-` |
| `ldr` at 0xf60cc | load | 4 | `abs 0xf6128` | `-` |
| `str` at 0xf60d0 | store | 4 | `sp -0x30` | `0x13a4c0` |
| `ldr` at 0xf60d4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf60d8 | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0xf60e0 | load | 4 | `abs 0xf6124` | `-` |
| `ldr` at 0xf60f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf60f8 | load | 4 | `abs 0xf6118` | `-` |
| `ldr` at 0xf60fc | load | 4 | `abs 0x2ae1c8` | `-` |
| `ldr` at 0xf6104 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf6108 | store | 4 | `(unknown)` | `?` |
| `ldmdb` at 0xf6114 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xf6114 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf6114 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf6114 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf6078 | `(unknown callee) 0x21eb8` | `0x9` | `0x13a4a8` | `0x52` | `0x0` |
| `bl` at 0xf60b4 | `(unknown callee) 0x1b454` | `[sp-36]` | `0x1` | `0x2ae1cc` | `?` |
| `bl` at 0xf60ec | `(unknown callee) 0x21eb8` | `0x9` | `0x13a4a8` | `0x5e` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xf6038 | `[0x2ae1c8] == 0x0` | `[0x2ae1c8] != 0x0` |
| `bne` at 0xf6044 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0xf6088 | `[0x2ae1cc] != 0x0` | `[0x2ae1cc] == 0x0` |
| `bne` at 0xf60c8 | `[0x2ae1c8] != 0x0` | `[0x2ae1c8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf6114 | `r0` | `0x0` |

## `lbi_tapmux_select`

- `lbi_tapmux_select`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf612c to 0xf61cc.
  Established. Zero entry `r1` takes the branch to `0xf6178`, and zero entry `r2` takes the next
  branch to `0xf6190` (`beq` at 0xf6168, `beq` at 0xf6180). Three byte stores at `sp -0x21` retain
  `0x0`, `0x10` and unknown at separate sites (`strb` at 0xf615c, `strb` at 0xf6174, `strb` at
  0xf618c). The unnamed `0xd73b0` call has `[sp-32]`, unknown, `0x50`, unknown (`bl` at 0xf61b8).
  Multiplexer selection encoding and external effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf613c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf6140 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf6144 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xf6148 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xf6154 | store | 4 | `sp -0x20` | `?` |
| `strb` at 0xf615c | store | 1 | `sp -0x21` | `0x0` |
| `ldr` at 0xf6160 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xf616c | load | 1 | `sp -0x21` | `-` |
| `strb` at 0xf6174 | store | 1 | `sp -0x21` | `0x10` |
| `ldr` at 0xf6178 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xf6184 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0xf618c | store | 1 | `sp -0x21` | `?` |
| `ldr` at 0xf6190 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf6198 | load | 4 | `abs 0xf61c8` | `-` |
| `ldr` at 0xf619c | load | 4 | `abs 0x2ae1c8` | `-` |
| `ldrb` at 0xf61a4 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xf61a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf61ac | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0xf61c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf61c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf61c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf61b8 | `(unknown callee) 0xd73b0` | `[sp-32]` | `?` | `0x50` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xf6168 | `r1 == 0x0` | `r1 != 0x0` |
| `beq` at 0xf6180 | `r2 == 0x0` | `r2 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf61c4 | `r0` | `ret(0xd73b0)` |

## `lbi_jtag_verify`

- `lbi_jtag_verify`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf61cc to 0xf6550.
  Established. The first four-byte pointee store clears `r1 +0x0` (`str` at 0xf6208). Included
  four-byte writes at unknown targets retain `0xa800ffea` and `0xabcdef01` (`str` at 0xf6274, `str`
  at 0xf6288). The unnamed `0xdd8e8` call has `0x1`, `0x1`, `0x1`, `[sp-56]` (`bl` at 0xf62e4). A
  later equality branch takes `? == 0x180005` to `0xf64dc` (`beq` at 0xf64b0). Verification data
  meaning and device identity remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf61dc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf61e0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf61e4 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xf61e8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xf61f4 | store | 4 | `sp -0x20` | `?` |
| `str` at 0xf61fc | store | 4 | `sp -0x38` | `0x1` |
| `ldr` at 0xf6200 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf6208 | store | 4 | `r1 +0x0` | `0x0` |
| `ldr` at 0xf6210 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf6214 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf622c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf6238 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf623c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf6254 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf6260 | store | 4 | `sp -0x24` | `0x40` |
| `ldr` at 0xf6270 | load | 4 | `abs 0xf6530` | `-` |
| `str` at 0xf6274 | store | 4 | `(unknown)` | `0xa800ffea` |
| `ldr` at 0xf6284 | load | 4 | `abs 0xf6534` | `-` |
| `str` at 0xf6288 | store | 4 | `(unknown)` | `0xabcdef01` |
| `str` at 0xf629c | store | 4 | `(unknown)` | `0x0` |
| `str` at 0xf62b0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xf62b4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf62b8 | store | 4 | `sp -0x4c` | `[sp-36]` |
| `str` at 0xf62c0 | store | 4 | `sp -0x48` | `sp-44` |
| `str` at 0xf62c8 | store | 4 | `sp -0x44` | `sp-52` |
| `ldr` at 0xf62cc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf62d0 | store | 4 | `sp -0x40` | `[sp-28]` |
| `ldr` at 0xf62e0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf62f8 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf6318 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf631c | load | 4 | `abs 0xf6538` | `-` |
| `ldr` at 0xf6334 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf633c | load | 4 | `abs 0xf6538` | `-` |
| `str` at 0xf6348 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf6364 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf6368 | load | 4 | `abs 0xf653c` | `-` |
| `ldr` at 0xf6380 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf6388 | load | 4 | `abs 0xf653c` | `-` |
| `str` at 0xf6394 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf63b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf63b4 | load | 4 | `abs 0xf6540` | `-` |
| `ldr` at 0xf63cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf63d4 | load | 4 | `abs 0xf6540` | `-` |
| `str` at 0xf63e0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf63fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf6418 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf642c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf6448 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf6464 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf6478 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf647c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf648c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf6490 | store | 4 | `[sp-24] +0x0` | `?` |
| `ldr` at 0xf64a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf64a8 | load | 4 | `abs 0xf6544` | `-` |
| `ldr` at 0xf64c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf64c4 | load | 4 | `abs 0xf6548` | `-` |
| `ldr` at 0xf64c8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf64d4 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf64e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf64ec | load | 4 | `abs 0xf6530` | `-` |
| `ldr` at 0xf6504 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf6508 | load | 4 | `abs 0xf654c` | `-` |
| `ldr` at 0xf650c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf6518 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf6524 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xf6528 | load | 4 | `sp -0x3c` | `-` |
| `ldmdb` at 0xf652c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf652c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf652c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf6218 | `(unknown callee) 0xdd33c` | `0x0` | `[sp-56]` | `[sp-28]` | `0x0` |
| `bl` at 0xf6240 | `(unknown callee) 0xdd3f8` | `0x1` | `[sp-56]` | `[sp-28]` | `ret(0xdd33c)` |
| `bl` at 0xf62e4 | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `[sp-56]` |
| `bl` at 0xf64cc | `(unknown callee) 0x1b52c` | `[sp-28]` | `0x13a500` | `?` | `?` |
| `bl` at 0xf6510 | `(unknown callee) 0x1b52c` | `[sp-28]` | `0x13a52c` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf6224 | `ret(0xdd33c) != 0x0` | `ret(0xdd33c) == 0x0` |
| `bne` at 0xf624c | `ret(0xdd3f8) != 0x0` | `ret(0xdd3f8) == 0x0` |
| `bne` at 0xf62f0 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `beq` at 0xf64b0 | `? == 0x180005` | `? != 0x180005` |
| `beq` at 0xf64f4 | `? == 0xa800ffea` | `? != 0xa800ffea` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf652c | `r0` | `[sp-60]` |

## `lbi_jtag_configure`

- `lbi_jtag_configure`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf6550 to 0xf6c98.
  Established. Two four-byte stores through `[sp-56] +0x0` write `0xa0000000` and `0xfc000000` at
  separate sites (`str` at 0xf6664, `str` at 0xf6a00). The unnamed `0xf6c98` call has `[sp-52]`,
  `[sp-56]`, `[sp-32]`, unknown (`bl` at 0xf6620); its zero result takes the branch to `0xf6654`
  (`beq` at 0xf662c). The loop branch takes `[[sp-52]+32] < [[sp-52]+24]`, signed 32-bit, to
  `0xf68ac` (`blt` at 0xf68a4). Configuration tokens and peripheral effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf6560 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf6564 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf6568 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xf656c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xf6570 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xf657c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf6580 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf6594 | store | 4 | `sp -0x38` | `ret(0x1b1b4)` |
| `ldr` at 0xf6598 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf65a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf65a8 | load | 4 | `abs 0xf6c70` | `-` |
| `str` at 0xf65b4 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf65bc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf65c4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xf65c8 | load | 4 | `abs 0xf6c74` | `-` |
| `ldr` at 0xf65d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf65d4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf65e0 | store | 4 | `sp -0x34` | `ret(0xfa454)` |
| `ldr` at 0xf65e4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xf65f4 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf6600 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf6608 | load | 4 | `abs 0xf6c78` | `-` |
| `ldr` at 0xf660c | load | 4 | `abs 0x145ae4` | `-` |
| `ldr` at 0xf6614 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf6618 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf661c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf6630 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf6634 | load | 4 | `[sp-52] +0x1c` | `-` |
| `str` at 0xf664c | store | 4 | `sp -0x48` | `?` |
| `str` at 0xf6658 | store | 4 | `sp -0x28` | `0x6` |
| `ldr` at 0xf665c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf6664 | store | 4 | `[sp-56] +0x0` | `0xa0000000` |
| `str` at 0xf6678 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xf667c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf6680 | store | 4 | `sp -0x58` | `[sp-40]` |
| `ldr` at 0xf6684 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf6688 | store | 4 | `sp -0x54` | `[sp-56]` |
| `str` at 0xf6690 | store | 4 | `sp -0x50` | `sp-64` |
| `ldr` at 0xf6694 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf6698 | store | 4 | `sp -0x4c` | `[sp-32]` |
| `str` at 0xf66c0 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf66e0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf66e8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf6704 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf6708 | load | 4 | `abs 0xf6c7c` | `-` |
| `ldr` at 0xf6720 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf6728 | load | 4 | `abs 0xf6c7c` | `-` |
| `str` at 0xf6734 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf6750 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf6754 | load | 4 | `abs 0xf6c80` | `-` |
| `ldr` at 0xf676c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf6774 | load | 4 | `abs 0xf6c80` | `-` |
| `str` at 0xf6780 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf679c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf67a0 | load | 4 | `abs 0xf6c84` | `-` |
| `ldr` at 0xf67b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf67c0 | load | 4 | `abs 0xf6c84` | `-` |
| `str` at 0xf67cc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf67e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf6804 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf6818 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf6834 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf6850 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf6864 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf6874 | load | 4 | `abs 0xf6c88` | `-` |
| `ldr` at 0xf6878 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf6880 | load | 4 | `abs 0xf6c8c` | `-` |
| `str` at 0xf688c | store | 4 | `sp -0x44` | `0x1` |
| `ldr` at 0xf6890 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf6894 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf6898 | load | 4 | `[sp-52] +0x20` | `-` |
| `ldr` at 0xf689c | load | 4 | `[sp-52] +0x18` | `-` |
| `ldr` at 0xf68ac | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf68b0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf68b4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf68bc | store | 4 | `sp -0x30` | `ret(0xfab74)` |
| `ldr` at 0xf68c0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xf68cc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf68d0 | load | 4 | `[sp-52] +0x1c` | `-` |
| `str` at 0xf68e8 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf68f0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xf68f8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xf68fc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf6900 | store | 4 | `sp -0x58` | `[sp-40]` |
| `ldr` at 0xf6904 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf6908 | store | 4 | `sp -0x54` | `[sp-56]` |
| `ldr` at 0xf690c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf6910 | store | 4 | `sp -0x50` | `[sp-56]` |
| `ldr` at 0xf6914 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf6918 | store | 4 | `sp -0x4c` | `[sp-32]` |
| `ldr` at 0xf6920 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xf6940 | store | 4 | `sp -0x48` | `?` |
| `str` at 0xf694c | store | 4 | `sp -0x44` | `0x6` |
| `ldr` at 0xf6960 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf6974 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf6978 | load | 4 | `[sp-52] +0x1c` | `-` |
| `str` at 0xf6990 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf6998 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf699c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf69a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf69b4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf69b8 | load | 4 | `[sp-52] +0x1c` | `-` |
| `str` at 0xf69d0 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf69d8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf69dc | load | 4 | `[sp-52] +0x1c` | `-` |
| `str` at 0xf69f4 | store | 4 | `sp -0x28` | `0x6` |
| `ldr` at 0xf69f8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf6a00 | store | 4 | `[sp-56] +0x0` | `0xfc000000` |
| `str` at 0xf6a14 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xf6a18 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf6a1c | store | 4 | `sp -0x58` | `[sp-40]` |
| `ldr` at 0xf6a20 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf6a24 | store | 4 | `sp -0x54` | `[sp-56]` |
| `str` at 0xf6a2c | store | 4 | `sp -0x50` | `sp-64` |
| `ldr` at 0xf6a30 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf6a34 | store | 4 | `sp -0x4c` | `[sp-32]` |
| `str` at 0xf6a5c | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf6a7c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf6a84 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf6aa0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf6aa4 | load | 4 | `abs 0xf6c7c` | `-` |
| `ldr` at 0xf6abc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf6ac4 | load | 4 | `abs 0xf6c7c` | `-` |
| `str` at 0xf6ad0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf6aec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf6af0 | load | 4 | `abs 0xf6c80` | `-` |
| `ldr` at 0xf6b08 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf6b10 | load | 4 | `abs 0xf6c80` | `-` |
| `str` at 0xf6b1c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf6b38 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf6b3c | load | 4 | `abs 0xf6c84` | `-` |
| `ldr` at 0xf6b54 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf6b5c | load | 4 | `abs 0xf6c84` | `-` |
| `str` at 0xf6b68 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf6b84 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf6ba0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf6bb4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf6bd0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf6bec | load | 4 | `(unknown)` | `-` |
| `str` at 0xf6c00 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf6c10 | load | 4 | `abs 0xf6c90` | `-` |
| `ldr` at 0xf6c14 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf6c28 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf6c44 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf6c48 | load | 4 | `abs 0xf6c94` | `-` |
| `ldr` at 0xf6c4c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf6c58 | store | 4 | `sp -0x48` | `?` |
| `str` at 0xf6c64 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xf6c68 | load | 4 | `sp -0x48` | `-` |
| `ldmdb` at 0xf6c6c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf6c6c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf6c6c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf658c | `(unknown callee) 0x1b1b4` | `?` | `r1` | `r2` | `?` |
| `bl` at 0xf65ac | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x13a558` | `?` | `ret(0x1b1b4)` |
| `bl` at 0xf65cc | `(unknown callee) 0x23fbc` | `0x13a588` | `?` | `?` | `?` |
| `bl` at 0xf65d8 | `(unknown callee) 0xfa454` | `[sp-24]` | `[sp-32]` | `?` | `?` |
| `bl` at 0xf6604 | `(unknown callee) 0xfaed0` | `0x0` | `ret(0xfa454)` | `?` | `ret(0xfa454)` |
| `bl` at 0xf6610 | `(unknown callee) 0x1b0ac` | `[0x145ae4]` | `?` | `?` | `0x145ae4` |
| `bl` at 0xf6620 | `(unknown callee) 0xf6c98` | `[sp-52]` | `[sp-56]` | `[sp-32]` | `?` |
| `bl` at 0xf6638 | `(unknown callee) 0x1b3a0` | `[[sp-52]+28]` | `?` | `?` | `[sp-52]` |
| `bl` at 0xf6644 | `(unknown callee) 0xfadec` | `sp-52` | `?` | `?` | `sp-52` |
| `bl` at 0xf66ac | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf687c | `(unknown callee) 0x23fbc` | `0x13a5b0` | `?` | `?` | `?` |
| `bl` at 0xf6884 | `(unknown callee) 0x23fbc` | `0x13a5d4` | `?` | `?` | `?` |
| `bl` at 0xf68b8 | `(unknown callee) 0xfab74` | `[sp-52]` | `[sp-44]` | `[sp-56]` | `[[sp-52]+24]` |
| `bl` at 0xf68d4 | `(unknown callee) 0x1b3a0` | `[[sp-52]+28]` | `?` | `?` | `[sp-52]` |
| `bl` at 0xf68e0 | `(unknown callee) 0xfadec` | `sp-52` | `?` | `?` | `sp-52` |
| `bl` at 0xf692c | `(unknown callee) 0xdd8e8` | `0x1` | `[sp-68]` | `0x6` | `0x0` |
| `bl` at 0xf6964 | `(unknown callee) 0xdd51c` | `0x6` | `0x0` | `0x0` | `[sp-32]` |
| `bl` at 0xf697c | `(unknown callee) 0x1b3a0` | `[[sp-52]+28]` | `?` | `?` | `[sp-52]` |
| `bl` at 0xf6988 | `(unknown callee) 0xfadec` | `sp-52` | `?` | `?` | `sp-52` |
| `bl` at 0xf69a4 | `(unknown callee) 0xf7354` | `[sp-52]` | `[sp-56]` | `[sp-32]` | `ret(0xdd51c)` |
| `bl` at 0xf69bc | `(unknown callee) 0x1b3a0` | `[[sp-52]+28]` | `?` | `?` | `[sp-52]` |
| `bl` at 0xf69c8 | `(unknown callee) 0xfadec` | `sp-52` | `?` | `?` | `sp-52` |
| `bl` at 0xf69e0 | `(unknown callee) 0x1b3a0` | `[[sp-52]+28]` | `?` | `?` | `[sp-52]` |
| `bl` at 0xf69ec | `(unknown callee) 0xfadec` | `sp-52` | `?` | `?` | `sp-52` |
| `bl` at 0xf6a48 | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf6c18 | `(unknown callee) 0x23fbc` | `0x13a60c` | `?` | `?` | `?` |
| `bl` at 0xf6c50 | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x13a630` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf65a0 | `ret(0x1b1b4) != 0x0` | `ret(0x1b1b4) == 0x0` |
| `bne` at 0xf65ec | `ret(0xfa454) != 0x0` | `ret(0xfa454) == 0x0` |
| `beq` at 0xf662c | `ret(0xf6c98) == 0x0` | `ret(0xf6c98) != 0x0` |
| `bne` at 0xf66b8 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `blt` at 0xf68a4 | `[[sp-52]+32] < [[sp-52]+24] (signed, 32-bit)` | `[[sp-52]+32] >= [[sp-52]+24] (signed, 32-bit)` |
| `bne` at 0xf68c8 | `ret(0xfab74) != 0x0` | `ret(0xfab74) == 0x0` |
| `bne` at 0xf6938 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bne` at 0xf6970 | `ret(0xdd51c) != 0x0` | `ret(0xdd51c) == 0x0` |
| `beq` at 0xf69b0 | `ret(0xf7354) == 0x0` | `ret(0xf7354) != 0x0` |
| `bne` at 0xf6a54 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `beq` at 0xf6c34 | `? == 0x25` | `? != 0x25` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf6c6c | `r0` | `[sp-72]` |

## `lbi_configure_pre_commands`

- `lbi_configure_pre_commands`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf6c98 to 0xf7354.
  Established. Included four-byte stores through `[sp-24] +0x0` write `0x90000000`, `0xa800ffea` and
  `0xaa995566` at separate sites (`str` at 0xf6cec, `str` at 0xf6d90, `str` at 0xf7294). A four-byte
  write through `[sp-24] +0x4` retains `0xabcdef01` (`str` at 0xf6da4). The unnamed `0xdd51c` call
  has `0x1`, `0x0`, `0x2710`, `[sp-28]` (`bl` at 0xf7018). Configuration command encoding and
  hardware effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf6ca8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf6cac | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf6cb0 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xf6cbc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf6cd4 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf6ce0 | store | 4 | `sp -0x20` | `0x6` |
| `ldr` at 0xf6ce4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf6cec | store | 4 | `[sp-24] +0x0` | `0x90000000` |
| `str` at 0xf6cf4 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xf6cf8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf6cfc | store | 4 | `sp -0x4c` | `[sp-32]` |
| `ldr` at 0xf6d00 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf6d04 | store | 4 | `sp -0x48` | `[sp-24]` |
| `str` at 0xf6d0c | store | 4 | `sp -0x44` | `sp-56` |
| `ldr` at 0xf6d10 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf6d14 | store | 4 | `sp -0x40` | `[sp-28]` |
| `str` at 0xf6d3c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf6d44 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf6d4c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xf6d50 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf6d60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf6d64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf6d68 | load | 4 | `abs 0xf731c` | `-` |
| `ldr` at 0xf6d6c | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xf6d78 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf6d84 | store | 4 | `sp -0x20` | `0x40` |
| `ldr` at 0xf6d88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf6d8c | load | 4 | `abs 0xf7320` | `-` |
| `str` at 0xf6d90 | store | 4 | `[sp-24] +0x0` | `0xa800ffea` |
| `ldr` at 0xf6d98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf6da0 | load | 4 | `abs 0xf7324` | `-` |
| `str` at 0xf6da4 | store | 4 | `[sp-24] +0x4` | `0xabcdef01` |
| `str` at 0xf6dac | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0xf6db4 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xf6db8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf6dbc | store | 4 | `sp -0x4c` | `[sp-32]` |
| `ldr` at 0xf6dc0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf6dc4 | store | 4 | `sp -0x48` | `[sp-24]` |
| `str` at 0xf6dcc | store | 4 | `sp -0x44` | `sp-56` |
| `ldr` at 0xf6dd0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf6dd4 | store | 4 | `sp -0x40` | `[sp-28]` |
| `str` at 0xf6dfc | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf6e04 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf6e08 | load | 4 | `abs 0xf7328` | `-` |
| `ldr` at 0xf6e14 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf6e1c | load | 4 | `abs 0xf7328` | `-` |
| `str` at 0xf6e28 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xf6e2c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf6e30 | load | 4 | `abs 0xf732c` | `-` |
| `ldr` at 0xf6e3c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf6e44 | load | 4 | `abs 0xf732c` | `-` |
| `str` at 0xf6e50 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xf6e54 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf6e58 | load | 4 | `abs 0xf7330` | `-` |
| `ldr` at 0xf6e64 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf6e6c | load | 4 | `abs 0xf7330` | `-` |
| `str` at 0xf6e78 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xf6e7c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf6e8c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf6ea0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xf6ea4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf6eb4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf6ec8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xf6ecc | load | 4 | `abs 0xf7334` | `-` |
| `ldr` at 0xf6ed0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf6ed8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf6edc | load | 4 | `abs 0xf7338` | `-` |
| `ldr` at 0xf6ee4 | load | 4 | `abs 0xf733c` | `-` |
| `ldr` at 0xf6ef0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf6ef4 | load | 4 | `abs 0xf7340` | `-` |
| `ldr` at 0xf6ef8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf6f04 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf6f0c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf6f10 | load | 4 | `abs 0xf7320` | `-` |
| `ldr` at 0xf6f1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf6f20 | load | 4 | `abs 0xf7344` | `-` |
| `ldr` at 0xf6f24 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xf6f30 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf6f3c | store | 4 | `sp -0x20` | `0x6` |
| `ldr` at 0xf6f40 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf6f48 | store | 4 | `[sp-24] +0x0` | `0xfc000000` |
| `str` at 0xf6f50 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xf6f54 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf6f58 | store | 4 | `sp -0x4c` | `[sp-32]` |
| `ldr` at 0xf6f5c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf6f60 | store | 4 | `sp -0x48` | `[sp-24]` |
| `str` at 0xf6f68 | store | 4 | `sp -0x44` | `sp-56` |
| `ldr` at 0xf6f6c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf6f70 | store | 4 | `sp -0x40` | `[sp-28]` |
| `str` at 0xf6f98 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf6fa4 | store | 4 | `sp -0x20` | `0x6` |
| `ldr` at 0xf6fa8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf6fb0 | store | 4 | `[sp-24] +0x0` | `0xd0000000` |
| `str` at 0xf6fb8 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xf6fbc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf6fc0 | store | 4 | `sp -0x4c` | `[sp-32]` |
| `ldr` at 0xf6fc4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf6fc8 | store | 4 | `sp -0x48` | `[sp-24]` |
| `str` at 0xf6fd0 | store | 4 | `sp -0x44` | `sp-56` |
| `ldr` at 0xf6fd4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf6fd8 | store | 4 | `sp -0x40` | `[sp-28]` |
| `str` at 0xf7000 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf7010 | load | 4 | `abs 0xf7348` | `-` |
| `ldr` at 0xf7014 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf702c | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf7038 | store | 4 | `sp -0x20` | `0x6` |
| `ldr` at 0xf703c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf7044 | store | 4 | `[sp-24] +0x0` | `0xa0000000` |
| `str` at 0xf704c | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xf7050 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf7054 | store | 4 | `sp -0x4c` | `[sp-32]` |
| `ldr` at 0xf7058 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf705c | store | 4 | `sp -0x48` | `[sp-24]` |
| `str` at 0xf7064 | store | 4 | `sp -0x44` | `sp-56` |
| `ldr` at 0xf7068 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf706c | store | 4 | `sp -0x40` | `[sp-28]` |
| `str` at 0xf7094 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf70a0 | store | 4 | `sp -0x20` | `0xc0` |
| `ldr` at 0xf70a4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf70ac | store | 4 | `[sp-24] +0x0` | `?` |
| `ldr` at 0xf70b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf70bc | load | 4 | `abs 0xf734c` | `-` |
| `str` at 0xf70c0 | store | 4 | `[sp-24] +0x4` | `0xaa995566` |
| `ldr` at 0xf70c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf70d0 | load | 4 | `abs 0xf7350` | `-` |
| `str` at 0xf70d4 | store | 4 | `[sp-24] +0x8` | `0x30008001` |
| `ldr` at 0xf70dc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf70e8 | store | 4 | `[sp-24] +0xc` | `0x7` |
| `ldr` at 0xf70f0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf70fc | store | 4 | `[sp-24] +0x10` | `0x0` |
| `ldr` at 0xf7104 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf7110 | store | 4 | `[sp-24] +0x14` | `0x0` |
| `ldr` at 0xf7114 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf7118 | store | 4 | `sp -0x4c` | `[sp-32]` |
| `ldr` at 0xf711c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf7120 | store | 4 | `sp -0x48` | `[sp-24]` |
| `str` at 0xf7128 | store | 4 | `sp -0x44` | `sp-56` |
| `ldr` at 0xf712c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf7130 | store | 4 | `sp -0x40` | `[sp-28]` |
| `str` at 0xf7158 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf7164 | store | 4 | `sp -0x20` | `0x6` |
| `ldr` at 0xf7168 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf7170 | store | 4 | `[sp-24] +0x0` | `0xb0000000` |
| `str` at 0xf7178 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xf717c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf7180 | store | 4 | `sp -0x4c` | `[sp-32]` |
| `ldr` at 0xf7184 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf7188 | store | 4 | `sp -0x48` | `[sp-24]` |
| `str` at 0xf7190 | store | 4 | `sp -0x44` | `sp-56` |
| `ldr` at 0xf7194 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf7198 | store | 4 | `sp -0x40` | `[sp-28]` |
| `str` at 0xf71c0 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf71d4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf71ec | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf71fc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf7214 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf7220 | store | 4 | `sp -0x20` | `0x6` |
| `ldr` at 0xf7224 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf722c | store | 4 | `[sp-24] +0x0` | `0xa0000000` |
| `str` at 0xf7234 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xf7238 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf723c | store | 4 | `sp -0x4c` | `[sp-32]` |
| `ldr` at 0xf7240 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf7244 | store | 4 | `sp -0x48` | `[sp-24]` |
| `str` at 0xf724c | store | 4 | `sp -0x44` | `sp-56` |
| `ldr` at 0xf7250 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf7254 | store | 4 | `sp -0x40` | `[sp-28]` |
| `str` at 0xf727c | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf7288 | store | 4 | `sp -0x20` | `0x60` |
| `ldr` at 0xf728c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf7290 | load | 4 | `abs 0xf734c` | `-` |
| `str` at 0xf7294 | store | 4 | `[sp-24] +0x0` | `0xaa995566` |
| `ldr` at 0xf729c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf72a8 | store | 4 | `[sp-24] +0x4` | `0x0` |
| `ldr` at 0xf72b0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf72bc | store | 4 | `[sp-24] +0x8` | `0x0` |
| `ldr` at 0xf72c0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf72c4 | store | 4 | `sp -0x4c` | `[sp-32]` |
| `ldr` at 0xf72c8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf72cc | store | 4 | `sp -0x48` | `[sp-24]` |
| `str` at 0xf72d4 | store | 4 | `sp -0x44` | `sp-56` |
| `ldr` at 0xf72d8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf72dc | store | 4 | `sp -0x40` | `[sp-28]` |
| `str` at 0xf7304 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf7310 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xf7314 | load | 4 | `sp -0x3c` | `-` |
| `ldmdb` at 0xf7318 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf7318 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf7318 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf6cc0 | `(unknown callee) 0xdd3f8` | `0x1` | `0x0` | `r2` | `r3` |
| `bl` at 0xf6d28 | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf6d70 | `(unknown callee) 0x1b52c` | `[sp-28]` | `0x13a65c` | `[[sp-24]]` | `[sp-24]` |
| `bl` at 0xf6de8 | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf6ed4 | `(unknown callee) 0x23fbc` | `0x13a688` | `[sp-56]` | `?` | `?` |
| `bl` at 0xf6efc | `(unknown callee) 0x1b52c` | `[sp-28]` | `0x13a500` | `[sp-56]` | `?` |
| `bl` at 0xf6f28 | `(unknown callee) 0x1b52c` | `[sp-28]` | `0x13a52c` | `[sp-52]` | `0xa800ffea` |
| `bl` at 0xf6f84 | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf6fec | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf7018 | `(unknown callee) 0xdd51c` | `0x1` | `0x0` | `0x2710` | `[sp-28]` |
| `bl` at 0xf7080 | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf7144 | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf71ac | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf71d8 | `(unknown callee) 0xdd51c` | `0x1` | `0x0` | `0xc` | `[sp-28]` |
| `bl` at 0xf7200 | `(unknown callee) 0xdd3f8` | `0x1` | `0x0` | `[sp-28]` | `ret(0xdd51c)` |
| `bl` at 0xf7268 | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf72f0 | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf6ccc | `ret(0xdd3f8) != 0x0` | `ret(0xdd3f8) == 0x0` |
| `bne` at 0xf6d34 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `beq` at 0xf6d5c | `? == 0x80000000` | `? != 0x80000000` |
| `bne` at 0xf6df4 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `beq` at 0xf6eec | `? == 0x1400093` | `? != 0x1400093` |
| `beq` at 0xf6f18 | `[sp-52] == 0xa800ffea` | `[sp-52] != 0xa800ffea` |
| `bne` at 0xf6f90 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bne` at 0xf6ff8 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bne` at 0xf7024 | `ret(0xdd51c) != 0x0` | `ret(0xdd51c) == 0x0` |
| `bne` at 0xf708c | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bne` at 0xf7150 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bne` at 0xf71b8 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bne` at 0xf71e4 | `ret(0xdd51c) != 0x0` | `ret(0xdd51c) == 0x0` |
| `bne` at 0xf720c | `ret(0xdd3f8) != 0x0` | `ret(0xdd3f8) == 0x0` |
| `bne` at 0xf7274 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bne` at 0xf72fc | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf7318 | `r0` | `[sp-60]` |

## `lbi_configure_post_commands`

- `lbi_configure_post_commands`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf7354 to 0xf7680.
  Established. Four-byte stores through `[sp-24] +0x0` include `0x30000000` and `0xfc000000` (`str`
  at 0xf73d0, `str` at 0xf7464). The unnamed `0xdd3f8` call has `0x1`, `0x0`, `r2`, `r3` (`bl` at
  0xf737c); its nonzero result takes the branch to `0xf7398` (`bne` at 0xf7388). The exit carries
  `[sp-60]` (`ldmdb` at 0xf767c). Post-configuration command meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf7364 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf7368 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf736c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xf7378 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf7390 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf73a0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf73b8 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf73c4 | store | 4 | `sp -0x20` | `0x6` |
| `ldr` at 0xf73c8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf73d0 | store | 4 | `[sp-24] +0x0` | `0x30000000` |
| `str` at 0xf73d8 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xf73dc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf73e0 | store | 4 | `sp -0x4c` | `[sp-32]` |
| `ldr` at 0xf73e4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf73e8 | store | 4 | `sp -0x48` | `[sp-24]` |
| `str` at 0xf73f0 | store | 4 | `sp -0x44` | `sp-56` |
| `ldr` at 0xf73f4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf73f8 | store | 4 | `sp -0x40` | `[sp-28]` |
| `str` at 0xf7420 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf7434 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf744c | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf7458 | store | 4 | `sp -0x20` | `0x6` |
| `ldr` at 0xf745c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf7464 | store | 4 | `[sp-24] +0x0` | `0xfc000000` |
| `str` at 0xf746c | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xf7470 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf7474 | store | 4 | `sp -0x4c` | `[sp-32]` |
| `ldr` at 0xf7478 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf747c | store | 4 | `sp -0x48` | `[sp-24]` |
| `str` at 0xf7484 | store | 4 | `sp -0x44` | `sp-56` |
| `ldr` at 0xf7488 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf748c | store | 4 | `sp -0x40` | `[sp-28]` |
| `str` at 0xf74b4 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf74c0 | store | 4 | `sp -0x20` | `0x6` |
| `ldr` at 0xf74c4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf74cc | store | 4 | `[sp-24] +0x0` | `0xfc000000` |
| `str` at 0xf74d4 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xf74d8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf74dc | store | 4 | `sp -0x4c` | `[sp-32]` |
| `ldr` at 0xf74e0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf74e4 | store | 4 | `sp -0x48` | `[sp-24]` |
| `str` at 0xf74ec | store | 4 | `sp -0x44` | `sp-56` |
| `ldr` at 0xf74f0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf74f4 | store | 4 | `sp -0x40` | `[sp-28]` |
| `str` at 0xf751c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf752c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf7544 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf7554 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf756c | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf7578 | store | 4 | `sp -0x20` | `0x6` |
| `ldr` at 0xf757c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf7584 | store | 4 | `[sp-24] +0x0` | `0x30000000` |
| `str` at 0xf758c | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xf7590 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf7594 | store | 4 | `sp -0x4c` | `[sp-32]` |
| `ldr` at 0xf7598 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf759c | store | 4 | `sp -0x48` | `[sp-24]` |
| `str` at 0xf75a4 | store | 4 | `sp -0x44` | `sp-56` |
| `ldr` at 0xf75a8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf75ac | store | 4 | `sp -0x40` | `[sp-28]` |
| `str` at 0xf75d4 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf75e8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf7600 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf760c | store | 4 | `sp -0x20` | `0x6` |
| `ldr` at 0xf7610 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf7618 | store | 4 | `[sp-24] +0x0` | `0xfc000000` |
| `str` at 0xf7620 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xf7624 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf7628 | store | 4 | `sp -0x4c` | `[sp-32]` |
| `ldr` at 0xf762c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf7630 | store | 4 | `sp -0x48` | `[sp-24]` |
| `str` at 0xf7638 | store | 4 | `sp -0x44` | `sp-56` |
| `ldr` at 0xf763c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf7640 | store | 4 | `sp -0x40` | `[sp-28]` |
| `str` at 0xf7668 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf7674 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xf7678 | load | 4 | `sp -0x3c` | `-` |
| `ldmdb` at 0xf767c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf767c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf767c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf737c | `(unknown callee) 0xdd3f8` | `0x1` | `0x0` | `r2` | `r3` |
| `bl` at 0xf73a4 | `(unknown callee) 0xdd3f8` | `0x1` | `0x0` | `[sp-28]` | `ret(0xdd3f8)` |
| `bl` at 0xf740c | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf7438 | `(unknown callee) 0xdd51c` | `0x1` | `0x0` | `0xc` | `[sp-28]` |
| `bl` at 0xf74a0 | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf7508 | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf7530 | `(unknown callee) 0xdd3f8` | `0x1` | `0x0` | `[sp-28]` | `ret(0xdd8e8)` |
| `bl` at 0xf7558 | `(unknown callee) 0xdd3f8` | `0x1` | `0x0` | `[sp-28]` | `ret(0xdd3f8)` |
| `bl` at 0xf75c0 | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf75ec | `(unknown callee) 0xdd51c` | `0x1` | `0x0` | `0xc` | `[sp-28]` |
| `bl` at 0xf7654 | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf7388 | `ret(0xdd3f8) != 0x0` | `ret(0xdd3f8) == 0x0` |
| `bne` at 0xf73b0 | `ret(0xdd3f8) != 0x0` | `ret(0xdd3f8) == 0x0` |
| `bne` at 0xf7418 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bne` at 0xf7444 | `ret(0xdd51c) != 0x0` | `ret(0xdd51c) == 0x0` |
| `bne` at 0xf74ac | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bne` at 0xf7514 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bne` at 0xf753c | `ret(0xdd3f8) != 0x0` | `ret(0xdd3f8) == 0x0` |
| `bne` at 0xf7564 | `ret(0xdd3f8) != 0x0` | `ret(0xdd3f8) == 0x0` |
| `bne` at 0xf75cc | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bne` at 0xf75f8 | `ret(0xdd51c) != 0x0` | `ret(0xdd51c) == 0x0` |
| `bne` at 0xf7660 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf767c | `r0` | `[sp-60]` |

## `llpcmd_to_tdr`

- `llpcmd_to_tdr`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf562c to 0xf5a84.
  Established. Included four-byte reads target `[sp-28] +0x4`, `[sp-28] +0x10` and `[sp-28] +0xc`
  (`ldr` at 0xf5660, `ldr` at 0xf56b0, `ldr` at 0xf56f0). The first unnamed `0xec86c` call has
  `[sp-20]`, `[sp-24]`, `0x0`, `sp-36`, while the last has `[sp-20]`, `[sp-24]`, `0xc`, `sp-36`
  (`bl` at 0xf5680, `bl` at 0xf5a7c). An included four-byte store at an unknown target retains
  `[[sp-28]+40]` (`str` at 0xf5a68). Link-command field encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf563c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf5640 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf5644 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xf5658 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5660 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xf566c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf5674 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5678 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf5694 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xf56a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf56b0 | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xf56bc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf56d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf56e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf56f0 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xf5700 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf5708 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf570c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5728 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5730 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xf573c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf5744 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5748 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5764 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf576c | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xf5774 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf577c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5780 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf579c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf57a4 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xf57b0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf57b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf57bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf57d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf57e0 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xf57e8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf57f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf57f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5810 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5818 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xf5824 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf582c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5830 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf584c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5854 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xf5860 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf5868 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf586c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5888 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5890 | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xf5898 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf58a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf58a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf58c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf58c8 | load | 4 | `[sp-28] +0x14` | `-` |
| `str` at 0xf58cc | store | 4 | `(unknown)` | `[[sp-28]+20]` |
| `ldr` at 0xf58d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf58d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf58f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf58fc | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xf5908 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf5910 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5914 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5930 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5938 | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xf5944 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf594c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5950 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf596c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5974 | load | 4 | `[sp-28] +0x18` | `-` |
| `str` at 0xf5984 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf598c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5990 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf59ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf59b4 | load | 4 | `[sp-28] +0x18` | `-` |
| `str` at 0xf59c0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf59c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf59cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf59e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf59f0 | load | 4 | `[sp-28] +0x1c` | `-` |
| `str` at 0xf59f4 | store | 4 | `(unknown)` | `[[sp-28]+28]` |
| `ldr` at 0xf5a08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5a10 | load | 4 | `[sp-28] +0x20` | `-` |
| `str` at 0xf5a14 | store | 4 | `(unknown)` | `[[sp-28]+32]` |
| `ldr` at 0xf5a1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5a20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5a3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5a44 | load | 4 | `[sp-28] +0x24` | `-` |
| `str` at 0xf5a48 | store | 4 | `(unknown)` | `[[sp-28]+36]` |
| `ldr` at 0xf5a5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5a64 | load | 4 | `[sp-28] +0x28` | `-` |
| `str` at 0xf5a68 | store | 4 | `(unknown)` | `[[sp-28]+40]` |
| `ldr` at 0xf5a70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5a74 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xf5a80 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf5a80 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf5a80 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf5680 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]` | `0x0` | `sp-36` |
| `bl` at 0xf5714 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]` | `0x1` | `sp-36` |
| `bl` at 0xf5750 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]` | `0x2` | `sp-36` |
| `bl` at 0xf5788 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]` | `0x3` | `sp-36` |
| `bl` at 0xf57c4 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]` | `0x4` | `sp-36` |
| `bl` at 0xf57fc | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]` | `0x5` | `sp-36` |
| `bl` at 0xf5838 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]` | `0x6` | `sp-36` |
| `bl` at 0xf5874 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]` | `0x7` | `sp-36` |
| `bl` at 0xf58ac | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]` | `0x9` | `sp-36` |
| `bl` at 0xf58e0 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]` | `0x8` | `sp-36` |
| `bl` at 0xf591c | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]` | `0xa` | `sp-36` |
| `bl` at 0xf5958 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]` | `0xb` | `sp-36` |
| `bl` at 0xf5998 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]` | `0xe` | `sp-36` |
| `bl` at 0xf59d4 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]` | `0xf` | `sp-36` |
| `bl` at 0xf5a28 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]` | `0xd` | `sp-36` |
| `bl` at 0xf5a7c | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]` | `0xc` | `sp-36` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf5a80 | `r0` | `ret(0xec86c)` |

## `llpcmd_from_tdr`

- `llpcmd_from_tdr`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf5a84 to 0xf6004.
  Established. The first unnamed `0xec33c` call carries `r0`, `r1`, `0x0`, `sp-36`, while the last
  has `[sp-20]`, `[sp-24]`, `0xc`, `sp-36` (`bl` at 0xf5ab0, `bl` at 0xf5fbc). Included four-byte
  stores through `[sp-28] +0xc` and `[sp-28] +0x28` retain `[sp-40]` and unknown (`str` at 0xf5b84,
  `str` at 0xf5ffc). The exit value is `sp-16` (`ldmdb` at 0xf6000). Field reconstruction and result
  meaning remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf5a94 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf5a98 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf5a9c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xf5aa4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5aa8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5ab8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5acc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf5adc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5ae4 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xf5af0 | store | 4 | `[sp-28] +0x4` | `?` |
| `ldr` at 0xf5af8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5afc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5b14 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf5b20 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xf5b28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5b30 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xf5b40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5b48 | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xf5b54 | store | 4 | `[sp-28] +0x10` | `?` |
| `ldr` at 0xf5b64 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf5b70 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xf5b78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5b80 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf5b84 | store | 4 | `[sp-28] +0xc` | `[sp-40]` |
| `ldr` at 0xf5b8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5b90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5ba0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5bb4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf5bc4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5bcc | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xf5bd8 | store | 4 | `[sp-28] +0x4` | `?` |
| `ldr` at 0xf5be0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5be4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5bf4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5c08 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf5c14 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5c1c | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xf5c28 | store | 4 | `[sp-28] +0x8` | `?` |
| `ldr` at 0xf5c30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5c34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5c44 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5c58 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf5c68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5c70 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xf5c7c | store | 4 | `[sp-28] +0x4` | `?` |
| `ldr` at 0xf5c84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5c88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5c98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5cac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf5cb8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5cc0 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xf5ccc | store | 4 | `[sp-28] +0x4` | `?` |
| `ldr` at 0xf5cd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5cd8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5ce8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5cfc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf5d0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5d14 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xf5d20 | store | 4 | `[sp-28] +0x8` | `?` |
| `ldr` at 0xf5d28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5d2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5d3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5d50 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf5d60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5d68 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xf5d74 | store | 4 | `[sp-28] +0x8` | `?` |
| `ldr` at 0xf5d7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5d80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5d90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5da4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf5db0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5db8 | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xf5dc4 | store | 4 | `[sp-28] +0x10` | `?` |
| `ldr` at 0xf5dcc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5dd0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5de0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5df4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf5df8 | store | 4 | `[sp-28] +0x14` | `?` |
| `ldr` at 0xf5e00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5e04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5e14 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5e28 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf5e38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5e40 | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xf5e4c | store | 4 | `[sp-28] +0x10` | `?` |
| `ldr` at 0xf5e54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5e58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5e68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5e7c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf5e8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5e94 | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xf5ea0 | store | 4 | `[sp-28] +0x10` | `?` |
| `ldr` at 0xf5ea8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5eac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5ebc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5ec8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5ed0 | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0xf5ee8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf5efc | store | 4 | `[sp-28] +0x18` | `?` |
| `ldr` at 0xf5f04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5f08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5f18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5f24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5f2c | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0xf5f44 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf5f54 | store | 4 | `[sp-28] +0x18` | `?` |
| `ldr` at 0xf5f5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5f60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5f70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5f84 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf5f88 | store | 4 | `[sp-28] +0x1c` | `?` |
| `ldr` at 0xf5f90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5fa4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf5fa8 | store | 4 | `[sp-28] +0x20` | `?` |
| `ldr` at 0xf5fb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5fb4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5fc4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5fd8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf5fdc | store | 4 | `[sp-28] +0x24` | `?` |
| `ldr` at 0xf5fe4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5ff8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf5ffc | store | 4 | `[sp-28] +0x28` | `?` |
| `ldmdb` at 0xf6000 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf6000 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf6000 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf5ab0 | `(unknown callee) 0xec33c` | `r0` | `r1` | `0x0` | `sp-36` |
| `bl` at 0xf5b04 | `(unknown callee) 0xec33c` | `[sp-20]` | `[sp-24]` | `0x1` | `sp-36` |
| `bl` at 0xf5b98 | `(unknown callee) 0xec33c` | `[sp-20]` | `[sp-24]` | `0x2` | `sp-36` |
| `bl` at 0xf5bec | `(unknown callee) 0xec33c` | `[sp-20]` | `[sp-24]` | `0x3` | `sp-36` |
| `bl` at 0xf5c3c | `(unknown callee) 0xec33c` | `[sp-20]` | `[sp-24]` | `0x4` | `sp-36` |
| `bl` at 0xf5c90 | `(unknown callee) 0xec33c` | `[sp-20]` | `[sp-24]` | `0x5` | `sp-36` |
| `bl` at 0xf5ce0 | `(unknown callee) 0xec33c` | `[sp-20]` | `[sp-24]` | `0x6` | `sp-36` |
| `bl` at 0xf5d34 | `(unknown callee) 0xec33c` | `[sp-20]` | `[sp-24]` | `0x7` | `sp-36` |
| `bl` at 0xf5d88 | `(unknown callee) 0xec33c` | `[sp-20]` | `[sp-24]` | `0x9` | `sp-36` |
| `bl` at 0xf5dd8 | `(unknown callee) 0xec33c` | `[sp-20]` | `[sp-24]` | `0x8` | `sp-36` |
| `bl` at 0xf5e0c | `(unknown callee) 0xec33c` | `[sp-20]` | `[sp-24]` | `0xa` | `sp-36` |
| `bl` at 0xf5e60 | `(unknown callee) 0xec33c` | `[sp-20]` | `[sp-24]` | `0xb` | `sp-36` |
| `bl` at 0xf5eb4 | `(unknown callee) 0xec33c` | `[sp-20]` | `[sp-24]` | `0xe` | `sp-36` |
| `bl` at 0xf5f10 | `(unknown callee) 0xec33c` | `[sp-20]` | `[sp-24]` | `0xf` | `sp-36` |
| `bl` at 0xf5f68 | `(unknown callee) 0xec33c` | `[sp-20]` | `[sp-24]` | `0xd` | `sp-36` |
| `bl` at 0xf5fbc | `(unknown callee) 0xec33c` | `[sp-20]` | `[sp-24]` | `0xc` | `sp-36` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf6000 | `r0` | `sp-16` |

## `JChip_TDR_Raw`

- `JChip_TDR_Raw`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeb5ec to 0xebb6c.
  Established. A halfword load reads `[[sp-40]] +0xe`, and a later byte read targets `[[sp-40]] +0xc` (`ldrh` at 0xeb674, `ldrb` at 0xeb69c). The branch takes entry-stacked `[sp] <= 0x400`,
  signed 32-bit, to `0xeba18` (`ble` at 0xeb9a8). Included unnamed `0xdd8e8` calls have `0x0`,
  `0x0`, `0x1`, `[sp-44]`, and `0x1`, `0x1`, `0x1`, `[sp-44]` (`bl` at 0xeb8e0, `bl` at 0xeba6c).
  Raw scan transfer layout and hardware effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeb5fc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xeb600 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xeb604 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xeb608 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xeb60c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xeb618 | store | 4 | `sp -0x28` | `ret(0xebc18)` |
| `str` at 0xeb620 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xeb624 | load | 4 | `abs 0xebb18` | `-` |
| `str` at 0xeb628 | store | 4 | `sp -0x44` | `0x2ae008` |
| `ldr` at 0xeb62c | load | 4 | `abs 0xebb1c` | `-` |
| `str` at 0xeb630 | store | 4 | `sp -0x48` | `0x2ae088` |
| `ldr` at 0xeb634 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeb640 | load | 4 | `abs 0xebb20` | `-` |
| `str` at 0xeb644 | store | 4 | `sp -0x4c` | `0xfffff7c6` |
| `ldr` at 0xeb64c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeb660 | load | 4 | `abs 0xebb24` | `-` |
| `str` at 0xeb664 | store | 4 | `sp -0x4c` | `0xfffff7c5` |
| `ldr` at 0xeb66c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeb670 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldrh` at 0xeb674 | load | 2 | `[[sp-40]] +0xe` | `-` |
| `str` at 0xeb688 | store | 4 | `sp -0x2c` | `0x1` |
| `str` at 0xeb690 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xeb694 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeb698 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldrb` at 0xeb69c | load | 1 | `[[sp-40]] +0xc` | `-` |
| `str` at 0xeb6a0 | store | 4 | `sp -0x34` | `[[[sp-40]]+12]:1` |
| `ldr` at 0xeb6a4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xeb6a8 | store | 4 | `sp -0x38` | `[[[sp-40]]+12]:1` |
| `str` at 0xeb6b0 | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0xeb6b8 | store | 4 | `sp -0x40` | `0x1` |
| `ldr` at 0xeb6c0 | load | 4 | `sp -0x28` | `-` |
| `ldrsb` at 0xeb6c4 | load | 1 | `[sp-40] +0x21` | `-` |
| `str` at 0xeb6c8 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xeb6cc | load | 4 | `abs 0xebb28` | `-` |
| `ldr` at 0xeb6d0 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xeb6d4 | load | 1 | `[sp-40] +0x19` | `-` |
| `ldr` at 0xeb6f4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xeb6fc | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xeb704 | load | 2 | `(unknown)` | `-` |
| `str` at 0xeb708 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xeb70c | load | 4 | `abs 0xebb28` | `-` |
| `ldr` at 0xeb710 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xeb714 | load | 1 | `[sp-40] +0x19` | `-` |
| `ldr` at 0xeb734 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xeb73c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xeb744 | load | 1 | `(unknown)` | `-` |
| `str` at 0xeb748 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xeb74c | load | 4 | `abs 0xebb28` | `-` |
| `ldr` at 0xeb750 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xeb754 | load | 1 | `[sp-40] +0x19` | `-` |
| `ldr` at 0xeb774 | load | 4 | `(unknown)` | `-` |
| `str` at 0xeb778 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xeb77c | load | 4 | `abs 0xebb28` | `-` |
| `ldr` at 0xeb780 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xeb784 | load | 1 | `[sp-40] +0x19` | `-` |
| `ldr` at 0xeb7a4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xeb7a8 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xeb7ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb7b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb7bc | load | 4 | `abs 0xebb2c` | `-` |
| `ldr` at 0xeb7c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb7d0 | load | 4 | `abs 0xebb2c` | `-` |
| `str` at 0xeb7dc | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xeb7e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb7e4 | load | 4 | `abs 0xebb30` | `-` |
| `ldr` at 0xeb7f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb7f8 | load | 4 | `abs 0xebb30` | `-` |
| `str` at 0xeb804 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xeb808 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb80c | load | 4 | `abs 0xebb34` | `-` |
| `ldr` at 0xeb818 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb820 | load | 4 | `abs 0xebb34` | `-` |
| `str` at 0xeb82c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xeb830 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb840 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xeb854 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xeb858 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb868 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xeb87c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xeb884 | load | 4 | `abs 0xebb18` | `-` |
| `str` at 0xeb888 | store | 4 | `sp -0x5c` | `0x2ae008` |
| `ldr` at 0xeb88c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xeb890 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xeb894 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xeb8a4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xeb8a8 | load | 4 | `abs 0xebb38` | `-` |
| `ldr` at 0xeb8b0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xeb8b4 | store | 4 | `sp -0x5c` | `[sp-56]` |
| `ldr` at 0xeb8b8 | load | 4 | `abs 0xebb18` | `-` |
| `str` at 0xeb8bc | store | 4 | `sp -0x58` | `0x2ae008` |
| `ldr` at 0xeb8c0 | load | 4 | `abs 0xebb1c` | `-` |
| `str` at 0xeb8c4 | store | 4 | `sp -0x54` | `0x2ae088` |
| `ldr` at 0xeb8c8 | load | 4 | `abs 0xebb38` | `-` |
| `str` at 0xeb8cc | store | 4 | `sp -0x50` | `0x2b47f8` |
| `ldr` at 0xeb8dc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xeb8f0 | load | 4 | `abs 0xebb3c` | `-` |
| `str` at 0xeb8f4 | store | 4 | `sp -0x5c` | `0x13713c` |
| `ldr` at 0xeb8fc | load | 4 | `abs 0xebb40` | `-` |
| `ldr` at 0xeb900 | load | 4 | `abs 0xebb44` | `-` |
| `ldr` at 0xeb90c | load | 4 | `abs 0xebb48` | `-` |
| `str` at 0xeb910 | store | 4 | `sp -0x4c` | `0xfffff7ca` |
| `ldr` at 0xeb918 | load | 4 | `abs 0xebb4c` | `-` |
| `ldr` at 0xeb91c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xeb928 | load | 4 | `abs 0xebb1c` | `-` |
| `str` at 0xeb92c | store | 4 | `sp -0x5c` | `0x2ae088` |
| `ldr` at 0xeb930 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xeb934 | load | 4 | `abs 0xebb50` | `-` |
| `ldr` at 0xeb938 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xeb93c | load | 4 | `abs 0xebb18` | `-` |
| `ldr` at 0xeb944 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeb950 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeb960 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeb96c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xeb970 | store | 4 | `sp -0x1c` | `[sp-32]` |
| `ldr` at 0xeb974 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xeb980 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xeb984 | store | 4 | `sp -0x44` | `[sp-28]` |
| `ldr` at 0xeb988 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeb994 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xeb998 | store | 4 | `sp -0x48` | `[sp-32]` |
| `ldr` at 0xeb9a0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xeb9ac | load | 4 | `abs 0xebb54` | `-` |
| `str` at 0xeb9b0 | store | 4 | `sp -0x5c` | `0x137168` |
| `ldr` at 0xeb9b4 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xeb9b8 | store | 4 | `sp -0x58` | `[sp]` |
| `ldr` at 0xeb9c0 | load | 4 | `abs 0xebb40` | `-` |
| `ldr` at 0xeb9c4 | load | 4 | `abs 0xebb58` | `-` |
| `ldr` at 0xeb9d0 | load | 4 | `abs 0xebb48` | `-` |
| `str` at 0xeb9d4 | store | 4 | `sp -0x4c` | `0xfffff7ca` |
| `ldr` at 0xeb9dc | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xeb9e8 | load | 4 | `abs 0xebb5c` | `-` |
| `str` at 0xeb9ec | store | 4 | `sp -0x5c` | `0x1371b8` |
| `ldr` at 0xeb9f0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xeb9f4 | store | 4 | `sp -0x58` | `[sp]` |
| `ldr` at 0xeb9fc | load | 4 | `abs 0xebb40` | `-` |
| `ldr` at 0xeba00 | load | 4 | `abs 0xebb60` | `-` |
| `ldr` at 0xeba0c | load | 4 | `abs 0xebb48` | `-` |
| `str` at 0xeba10 | store | 4 | `sp -0x4c` | `0xfffff7ca` |
| `str` at 0xeba1c | store | 4 | `sp -0x5c` | `sp-36` |
| `ldr` at 0xeba20 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xeba24 | store | 4 | `sp -0x58` | `[sp-68]` |
| `ldr` at 0xeba28 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xeba2c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xeba30 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xeba34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeba3c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xeba40 | store | 4 | `sp -0x5c` | `[sp-36]` |
| `ldr` at 0xeba44 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xeba48 | store | 4 | `sp -0x58` | `[sp-68]` |
| `ldr` at 0xeba4c | load | 4 | `sp -0x48` | `-` |
| `str` at 0xeba50 | store | 4 | `sp -0x54` | `[sp-72]` |
| `ldr` at 0xeba54 | load | 4 | `abs 0xebb38` | `-` |
| `str` at 0xeba58 | store | 4 | `sp -0x50` | `0x2b47f8` |
| `ldr` at 0xeba68 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xeba7c | load | 4 | `abs 0xebb64` | `-` |
| `str` at 0xeba80 | store | 4 | `sp -0x5c` | `0x137218` |
| `ldr` at 0xeba88 | load | 4 | `abs 0xebb40` | `-` |
| `ldr` at 0xeba98 | load | 4 | `abs 0xebb48` | `-` |
| `str` at 0xeba9c | store | 4 | `sp -0x4c` | `0xfffff7ca` |
| `ldr` at 0xebaa4 | load | 4 | `abs 0xebb4c` | `-` |
| `ldr` at 0xebaa8 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xebab4 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xebab8 | store | 4 | `sp -0x5c` | `[sp-72]` |
| `ldr` at 0xebabc | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xebac0 | load | 4 | `abs 0xebb68` | `-` |
| `ldr` at 0xebac4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xebac8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xebad0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xebadc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xebae0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xebaec | load | 4 | `sp +0x0` | `-` |
| `str` at 0xebaf0 | store | 4 | `sp -0x5c` | `[sp]` |
| `ldr` at 0xebaf4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xebaf8 | load | 4 | `abs 0xebb1c` | `-` |
| `ldr` at 0xebb00 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xebb0c | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0xebb10 | load | 4 | `sp -0x4c` | `-` |
| `ldmdb` at 0xebb14 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xebb14 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xebb14 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeb610 | `(unknown callee) 0xebc18` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xeb650 | `(unknown callee) 0xe9a1c` | `ret(0xebc18)` | `?` | `?` | `ret(0xebc18)` |
| `bl` at 0xeb89c | `(unknown callee) 0xebd58` | `[sp-56]` | `[sp-48]` | `[sp-52]` | `sp-24` |
| `bl` at 0xeb8ac | `(unknown callee) 0xdd33c` | `0x1` | `[sp-44]` | `0x2b47f8` | `?` |
| `bl` at 0xeb8e0 | `(unknown callee) 0xdd8e8` | `0x0` | `0x0` | `0x1` | `[sp-44]` |
| `bl` at 0xeb908 | `(unknown callee) 0x21eb8` | `0x3` | `0x136ed4` | `0x376` | `0x0` |
| `bl` at 0xeb940 | `(unknown callee) 0xec0d0` | `[sp+4]` | `0x137160` | `[sp-56]` | `0x2ae008` |
| `bl` at 0xeb9cc | `(unknown callee) 0x21eb8` | `0x3` | `0x136ed4` | `0x39f` | `0x0` |
| `bl` at 0xeba08 | `(unknown callee) 0x21eb8` | `0x3` | `0x136ed4` | `0x3a7` | `0x0` |
| `bl` at 0xeba38 | `(unknown callee) 0xebfe0` | `[sp-64]` | `[sp-60]` | `[sp]` | `[sp-28]` |
| `bl` at 0xeba6c | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `[sp-44]` |
| `bl` at 0xeba94 | `(unknown callee) 0x21eb8` | `0x3` | `0x136ed4` | `0x3b0` | `0x0` |
| `bl` at 0xebacc | `(unknown callee) 0xec0d0` | `[sp+4]` | `0x13723c` | `[sp-36]` | `[sp-68]` |
| `bl` at 0xebb04 | `(unknown callee) 0xe2b10` | `[sp-60]` | `0x2ae088` | `0x0` | `[sp-32]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xeb63c | `ret(0xebc18) != 0x0` | `ret(0xebc18) == 0x0` |
| `beq` at 0xeb65c | `ret(0xe9a1c) == 0x0` | `ret(0xe9a1c) != 0x0` |
| `beq` at 0xeb680 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xeb7b4 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xeb8ec | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xeb924 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xeb94c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xeb958 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0xeb968 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xeb97c | `[sp-64] != 0x1` | `[sp-64] == 0x1` |
| `beq` at 0xeb990 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `ble` at 0xeb9a8 | `[sp] <= 0x400 (signed, 32-bit)` | `[sp] > 0x400 (signed, 32-bit)` |
| `ble` at 0xeb9e4 | `[sp] <= 0x400 (signed, 32-bit)` | `[sp] > 0x400 (signed, 32-bit)` |
| `bne` at 0xeba78 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xebab0 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xebad8 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `beq` at 0xebae8 | `[sp-72] == [sp-32]` | `[sp-72] != [sp-32]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xebb14 | `r0` | `?` |
