# ARM link controller setup paths in 1.40.5

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
No unresolved computed transfers are reported. Unknown data values and instruction effects remain
explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/link-controller-setup-facts.txt) retain all tool sections.

## `SHUB_WarCheck`

- `SHUB_WarCheck`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4df64 to 0x4e004.
  Established. The absolute byte reads use `0x148f68` and `0x148f69` (`ldrb` at 0x4df80, `ldrb` at
  0x4df98). The unnamed `0x21eb8` call carries `0x1`, `0x109f8c`, `0x81d`, `0x0` (`bl` at 0x4dfe8).
  Workaround policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4df74 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4df78 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x4df7c | load | 4 | `abs 0x4dff0` | `-` |
| `ldrb` at 0x4df80 | load | 1 | `abs 0x148f68` | `-` |
| `ldr` at 0x4df84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4df94 | load | 4 | `abs 0x4dff4` | `-` |
| `ldrb` at 0x4df98 | load | 1 | `abs 0x148f69` | `-` |
| `ldr` at 0x4df9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4dfb8 | load | 4 | `abs 0x4dff8` | `-` |
| `str` at 0x4dfbc | store | 4 | `sp -0x24` | `0x10af48` |
| `ldr` at 0x4dfc0 | load | 4 | `abs 0x4dff0` | `-` |
| `ldrb` at 0x4dfc4 | load | 1 | `abs 0x148f68` | `-` |
| `str` at 0x4dfc8 | store | 4 | `sp -0x20` | `[0x148f68]:1` |
| `ldr` at 0x4dfcc | load | 4 | `abs 0x4dff4` | `-` |
| `ldrb` at 0x4dfd0 | load | 1 | `abs 0x148f69` | `-` |
| `str` at 0x4dfd4 | store | 4 | `sp -0x1c` | `[0x148f69]:1` |
| `ldr` at 0x4dfdc | load | 4 | `abs 0x4dffc` | `-` |
| `ldr` at 0x4dfe0 | load | 4 | `abs 0x4e000` | `-` |
| `ldmdb` at 0x4dfec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4dfec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4dfec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4dfe8 | `(unknown callee) 0x21eb8` | `0x1` | `0x109f8c` | `0x81d` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4dfb4 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4dfec | `r0` | `?` |

## `SHUB_WarStatus`

- `SHUB_WarStatus`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4e004 to 0x4e054.
  Established. Entry `r0 == 0x0` takes `0x4e030`, while nonzero falls through (`beq` at 0x4e020).
  The unnamed `0x1b52c` call carries `r0`, `0x10af90`, `r2`, `r0` (`bl` at 0x4e02c). The absolute
  byte at `0x148f69` is read (`ldrb` at 0x4e034). Status polarity remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4e014 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x4e018 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4e024 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4e028 | load | 4 | `abs 0x4e04c` | `-` |
| `ldr` at 0x4e030 | load | 4 | `abs 0x4e050` | `-` |
| `ldrb` at 0x4e034 | load | 1 | `abs 0x148f69` | `-` |
| `ldmdb` at 0x4e048 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4e048 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4e048 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4e02c | `(unknown callee) 0x1b52c` | `r0` | `0x10af90` | `r2` | `r0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4e020 | `r0 == 0x0` | `r0 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4e048 | `r0` | `?` |

## `SHUB_WarAllow`

- `SHUB_WarAllow`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4e054 to 0x4e07c.
  Established. The absolute byte assignment at `0x148f68` writes `([sp-20]:1 & 0xff)` (`strb` at
  0x4e070). The return reports entry `r0` (`ldmdb` at 0x4e074). Workaround permission meaning
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4e064 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x4e068 | load | 4 | `abs 0x4e078` | `-` |
| `ldrb` at 0x4e06c | load | 1 | `sp -0x14` | `-` |
| `strb` at 0x4e070 | store | 1 | `abs 0x148f68` | `([sp-20]:1 & 0xff)` |
| `ldmdb` at 0x4e074 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4e074 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4e074 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4e074 | `r0` | `r0` |

## `SHUB_WarEnable`

- `SHUB_WarEnable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4e07c to 0x4e180.
  Established. The indirect call has an unknown target and candidates `0xff`, `r1`, `r2`,
  `[0x148da0]` (`mov` at 0x4e0cc). The byte assignments at `0x148f69` write `0x1` and `0x0` (`strb` at 0x4e10c, `strb` at 0x4e144). Enable effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4e08c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x4e090 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4e09c | load | 4 | `abs 0x4e154` | `-` |
| `ldrb` at 0x4e0a0 | load | 1 | `abs 0x148f68` | `-` |
| `ldr` at 0x4e0ac | load | 4 | `abs 0x4e158` | `-` |
| `ldr` at 0x4e0b0 | load | 4 | `abs 0x148da0` | `-` |
| `ldr` at 0x4e0bc | load | 4 | `abs 0x4e158` | `-` |
| `ldr` at 0x4e0c0 | load | 4 | `abs 0x148da0` | `-` |
| `ldr` at 0x4e0dc | load | 4 | `abs 0x4e15c` | `-` |
| `str` at 0x4e0e0 | store | 4 | `sp -0x18` | `0x10afa0` |
| `ldr` at 0x4e0e8 | load | 4 | `abs 0x4e160` | `-` |
| `ldr` at 0x4e0ec | load | 4 | `abs 0x4e164` | `-` |
| `ldr` at 0x4e0fc | load | 4 | `abs 0x4e168` | `-` |
| `ldr` at 0x4e104 | load | 4 | `abs 0x4e16c` | `-` |
| `strb` at 0x4e10c | store | 1 | `abs 0x148f69` | `0x1` |
| `ldr` at 0x4e110 | load | 4 | `abs 0x4e170` | `-` |
| `ldr` at 0x4e11c | load | 4 | `abs 0x4e174` | `-` |
| `str` at 0x4e120 | store | 4 | `sp -0x18` | `0x10afcc` |
| `ldr` at 0x4e128 | load | 4 | `abs 0x4e160` | `-` |
| `ldr` at 0x4e12c | load | 4 | `abs 0x4e178` | `-` |
| `ldr` at 0x4e13c | load | 4 | `abs 0x4e16c` | `-` |
| `strb` at 0x4e144 | store | 1 | `abs 0x148f69` | `0x0` |
| `ldr` at 0x4e148 | load | 4 | `abs 0x4e17c` | `-` |
| `ldmdb` at 0x4e150 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4e150 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4e150 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x4e0cc | `(unknown callee) (unknown target)` | `0xff` | `r1` | `r2` | `[0x148da0]` |
| `bl` at 0x4e0f4 | `(unknown callee) 0x21eb8` | `0x1` | `0x109f8c` | `0x844` | `0x0` |
| `bl` at 0x4e100 | `(unknown callee) 0x202d0` | `0x10afc0` | `?` | `?` | `?` |
| `bl` at 0x4e114 | `(unknown callee) 0x22e50` | `0x148f14` | `?` | `0x1` | `0x148f69` |
| `bl` at 0x4e134 | `(unknown callee) 0x21eb8` | `0x3` | `0x109f8c` | `0x84c` | `0x0` |
| `bl` at 0x4e14c | `(unknown callee) 0x202d0` | `0x10b000` | `r1` | `0x0` | `0x148f69` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4e098 | `r0 == 0x0` | `r0 != 0x0` |
| `beq` at 0x4e0a8 | `[0x148f68]:1 == 0x0` | `[0x148f68]:1 != 0x0` |
| `beq` at 0x4e0b8 | `[0x148da0] == 0x0` | `[0x148da0] != 0x0` |
| `beq` at 0x4e0d8 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4e150 | `r0` | `?` |

## `PICInit`

- `PICInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3440c to 0x34698.
  Established. The absolute word assignment at `0x148c80` writes `[sp-20]` (`str` at 0x3455c). The
  byte at `0x148c85` receives `0x1` (`strb` at 0x3457c). The unnamed `0xd7530` call carries `0x0`,
  `[[sp-28]]:1`, `sp-29`, `[[sp-28]]:1` (`bl` at 0x345e0). Controller setup effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3441c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x34420 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x34424 | load | 4 | `abs 0x34670` | `-` |
| `ldr` at 0x34428 | load | 4 | `abs 0x148c80` | `-` |
| `ldr` at 0x34434 | load | 4 | `abs 0x34674` | `-` |
| `str` at 0x34438 | store | 4 | `sp -0x34` | `0x106620` |
| `ldr` at 0x3443c | load | 4 | `abs 0x34678` | `-` |
| `ldrb` at 0x34440 | load | 1 | `abs 0x148c84` | `-` |
| `str` at 0x34444 | store | 4 | `sp -0x30` | `[0x148c84]:1` |
| `ldr` at 0x3444c | load | 4 | `abs 0x3467c` | `-` |
| `str` at 0x34460 | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0x34468 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x34470 | load | 4 | `abs 0x34680` | `-` |
| `ldr` at 0x34474 | load | 4 | `abs 0x34684` | `-` |
| `ldr` at 0x34478 | load | 4 | `abs 0x34688` | `-` |
| `str` at 0x34484 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x34488 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3448c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3449c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x344b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x344b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x344cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x344d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x344d8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x344e4 | store | 4 | `sp -0x24` | `ret(0xe9c50)` |
| `ldr` at 0x344e8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x344f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x34500 | load | 4 | `abs 0x3468c` | `-` |
| `str` at 0x34504 | store | 4 | `sp -0x34` | `0x106650` |
| `ldr` at 0x34508 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3451c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x34524 | load | 4 | `(unknown)` | `-` |
| `str` at 0x34528 | store | 4 | `sp -0x30` | `?` |
| `str` at 0x3452c | store | 4 | `sp -0x2c` | `ret(0x21704)` |
| `ldr` at 0x34534 | load | 4 | `abs 0x3467c` | `-` |
| `ldr` at 0x34544 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3454c | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x34554 | load | 4 | `abs 0x34670` | `-` |
| `ldr` at 0x34558 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3455c | store | 4 | `abs 0x148c80` | `[sp-20]` |
| `ldr` at 0x34560 | load | 4 | `abs 0x34678` | `-` |
| `ldrb` at 0x34564 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0x34568 | store | 1 | `abs 0x148c84` | `([sp-24]:1 & 0xff)` |
| `str` at 0x34570 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x34574 | load | 4 | `abs 0x34690` | `-` |
| `strb` at 0x3457c | store | 1 | `abs 0x148c85` | `0x1` |
| `ldr` at 0x34580 | load | 4 | `abs 0x34678` | `-` |
| `ldrb` at 0x34584 | load | 1 | `abs 0x148c84` | `-` |
| `ldr` at 0x34588 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x34594 | load | 4 | `abs 0x34690` | `-` |
| `ldrb` at 0x34598 | load | 1 | `abs 0x148c85` | `-` |
| `ldr` at 0x345a8 | load | 4 | `abs 0x34670` | `-` |
| `ldr` at 0x345ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x345c0 | load | 4 | `abs 0x148c80` | `-` |
| `str` at 0x345c8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x345cc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x345d0 | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0x345e8 | store | 4 | `sp -0x24` | `ret(0xd7530)` |
| `ldr` at 0x345ec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x345f8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x34604 | load | 4 | `abs 0x34694` | `-` |
| `str` at 0x34608 | store | 4 | `sp -0x34` | `0x10666c` |
| `str` at 0x3460c | store | 4 | `sp -0x30` | `ret(0x21a38)` |
| `ldr` at 0x34614 | load | 4 | `abs 0x3467c` | `-` |
| `ldr` at 0x34624 | load | 4 | `abs 0x34690` | `-` |
| `strb` at 0x3462c | store | 1 | `abs 0x148c85` | `0x0` |
| `ldr` at 0x34634 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x34638 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x3463c | load | 1 | `[sp-28] +0x1` | `-` |
| `ldr` at 0x34650 | load | 4 | `abs 0x34690` | `-` |
| `strb` at 0x34658 | store | 1 | `abs 0x148c85` | `0x0` |
| `ldr` at 0x3465c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x34664 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldmdb` at 0x3466c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3466c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3466c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x34458 | `(unknown callee) 0x21eb8` | `0x9` | `0x10663c` | `0x3b` | `0x0` |
| `bl` at 0x3447c | `(unknown callee) 0x58180` | `0x0` | `0x34b44` | `0x34b6c` | `0x34bbc` |
| `bl` at 0x344dc | `(unknown callee) 0xe9c50` | `?` | `?` | `?` | `?` |
| `bl` at 0x344f8 | `(unknown callee) 0x21704` | `ret(0xe9c50)` | `?` | `?` | `ret(0xe9c50)` |
| `bl` at 0x34540 | `(unknown callee) 0x21eb8` | `0x1` | `0x10663c` | `0x49` | `0x0` |
| `bl` at 0x345e0 | `(unknown callee) 0xd7530` | `0x0` | `[[sp-28]]:1` | `sp-29` | `[[sp-28]]:1` |
| `bl` at 0x345fc | `(unknown callee) 0x21a38` | `ret(0xd7530)` | `?` | `?` | `ret(0xd7530)` |
| `bl` at 0x34620 | `(unknown callee) 0x21eb8` | `0x1` | `0x10663c` | `0x5a` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x34430 | `[0x148c80] == 0x0` | `[0x148c80] != 0x0` |
| `blt` at 0x34494 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x344f0 | `ret(0xe9c50) == 0x0` | `ret(0xe9c50) != 0x0` |
| `bge` at 0x34590 | `[sp-40] >= [0x148c84]:1 (signed, 32-bit)` | `[sp-40] < [0x148c84]:1 (signed, 32-bit)` |
| `bne` at 0x345a0 | `[0x148c85]:1 != 0x0` | `[0x148c85]:1 == 0x0` |
| `beq` at 0x345f4 | `ret(0xd7530) == 0x0` | `ret(0xd7530) != 0x0` |
| `bne` at 0x3464c | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3466c | `r0` | `?` |

## `PICActive`

- `PICActive`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x34b44 to 0x34b6c.
  Established. The absolute byte read uses `0x148c85` (`ldrb` at 0x34b5c). The return reports
  `[0x148c85]:1` (`ldmdb` at 0x34b64). Active-state meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x34b54 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x34b58 | load | 4 | `abs 0x34b68` | `-` |
| `ldrb` at 0x34b5c | load | 1 | `abs 0x148c85` | `-` |
| `ldmdb` at 0x34b64 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x34b64 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x34b64 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x34b64 | `r0` | `[0x148c85]:1` |

## `NL4R_Init`

- `NL4R_Init`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x27898 to 0x279d0.
  Established. The absolute byte at `0x148bd8` receives `0x1` (`strb` at 0x278e4). The absolute word
  assignments at `0x148bdc` and `0x148be0` write `[sp-20]` and `[sp-24]` (`str` at 0x278f0, `str` at
  0x278fc). The unnamed `0x37748` call has unknown first three candidates and fourth `[sp-24]` (`bl` at 0x279b0). Initialization roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x278a8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x278ac | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x278b0 | load | 4 | `abs 0x279b8` | `-` |
| `ldrb` at 0x278b4 | load | 1 | `abs 0x148bd8` | `-` |
| `ldr` at 0x278c0 | load | 4 | `abs 0x279bc` | `-` |
| `str` at 0x278c4 | store | 4 | `sp -0x2c` | `0x101c58` |
| `ldr` at 0x278cc | load | 4 | `abs 0x279c0` | `-` |
| `ldr` at 0x278dc | load | 4 | `abs 0x279b8` | `-` |
| `strb` at 0x278e4 | store | 1 | `abs 0x148bd8` | `0x1` |
| `ldr` at 0x278e8 | load | 4 | `abs 0x279c4` | `-` |
| `ldr` at 0x278ec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x278f0 | store | 4 | `abs 0x148bdc` | `[sp-20]` |
| `ldr` at 0x278f4 | load | 4 | `abs 0x279c8` | `-` |
| `ldr` at 0x278f8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x278fc | store | 4 | `abs 0x148be0` | `[sp-24]` |
| `str` at 0x27904 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x27908 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2790c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2791c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x27924 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2792c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x27934 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2793c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x27940 | load | 4 | `(unknown)` | `-` |
| `str` at 0x2794c | store | 4 | `sp -0x20` | `ret(0xe9c50)` |
| `ldr` at 0x27950 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2795c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x27968 | load | 4 | `abs 0x279cc` | `-` |
| `str` at 0x2796c | store | 4 | `sp -0x2c` | `0x101c94` |
| `ldr` at 0x27970 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x27978 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27980 | load | 4 | `(unknown)` | `-` |
| `str` at 0x27984 | store | 4 | `sp -0x28` | `?` |
| `str` at 0x27988 | store | 4 | `sp -0x24` | `ret(0x21704)` |
| `ldr` at 0x27990 | load | 4 | `abs 0x279c0` | `-` |
| `ldr` at 0x279a0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x279a8 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldmdb` at 0x279b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x279b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x279b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x278d8 | `(unknown callee) 0x21eb8` | `0x9` | `0x101c80` | `0x71` | `0x0` |
| `bl` at 0x27944 | `(unknown callee) 0xe9c50` | `?` | `?` | `?` | `?` |
| `bl` at 0x27960 | `(unknown callee) 0x21704` | `ret(0xe9c50)` | `?` | `?` | `ret(0xe9c50)` |
| `bl` at 0x2799c | `(unknown callee) 0x21eb8` | `0x9` | `0x101c80` | `0x81` | `0x0` |
| `bl` at 0x279b0 | `(unknown callee) 0x37748` | `?` | `?` | `?` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x278bc | `[0x148bd8]:1 == 0x0` | `[0x148bd8]:1 != 0x0` |
| `blt` at 0x27914 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x27958 | `ret(0xe9c50) == 0x0` | `ret(0xe9c50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x279b4 | `r0` | `ret(0x37748)` |

## `nl4r_init_icntlr`

- `nl4r_init_icntlr`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x28574 to 0x28a1c.
  Established. The unnamed `0xec86c` call at `0x285e8` carries `[sp-20]`, `0x6`, `0x4f`, `sp-28`
  (`bl` at 0x285e8). The unnamed `0xed170` call at `0x2868c` carries `[sp-20]`, `0x2`, `0x6`, `0x0`
  (`bl` at 0x2868c); its nonnegative result takes `0x286b0`, while negative falls through (`bge` at
  0x286a0). Controller instruction meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x28584 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x28588 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x2858c | load | 4 | `abs 0x28a08` | `-` |
| `ldr` at 0x28590 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x2859c | load | 4 | `abs 0x28a0c` | `-` |
| `ldr` at 0x285a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x285a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x285ac | load | 4 | `sp -0x18` | `-` |
| `str` at 0x285c0 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x285c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x285dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x285f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28604 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28618 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2862c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28640 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28654 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28664 | load | 4 | `abs 0x28a08` | `-` |
| `ldr` at 0x28668 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x28674 | load | 4 | `abs 0x28a10` | `-` |
| `ldr` at 0x2867c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28694 | store | 4 | `sp -0x20` | `ret(0xed170)` |
| `ldr` at 0x28698 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x286a4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x286a8 | store | 4 | `sp -0x2c` | `ret(0xed170)` |
| `str` at 0x286b4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x286bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x286d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x286e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x286f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2870c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28720 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28734 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28748 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28758 | load | 4 | `abs 0x28a08` | `-` |
| `ldr` at 0x2875c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x28768 | load | 4 | `abs 0x28a14` | `-` |
| `ldr` at 0x28770 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28788 | store | 4 | `sp -0x20` | `ret(0xed170)` |
| `ldr` at 0x2878c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x28798 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x2879c | store | 4 | `sp -0x2c` | `ret(0xed170)` |
| `ldr` at 0x287a4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x287b8 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x287c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x287d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x287e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x287fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28810 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28824 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28838 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2884c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28860 | store | 4 | `sp -0x1c` | `0xf` |
| `ldr` at 0x28868 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2887c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28890 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x288a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x288b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x288cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x288e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x288f4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28908 | store | 4 | `sp -0x1c` | `0xf` |
| `ldr` at 0x28910 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28924 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28938 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2894c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28960 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28974 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28988 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2899c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x289ac | load | 4 | `abs 0x28a08` | `-` |
| `ldr` at 0x289b0 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x289bc | load | 4 | `abs 0x28a18` | `-` |
| `ldr` at 0x289c4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x289dc | store | 4 | `sp -0x20` | `ret(0xed170)` |
| `ldr` at 0x289e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x289ec | load | 4 | `sp -0x20` | `-` |
| `str` at 0x289f0 | store | 4 | `sp -0x2c` | `ret(0xed170)` |
| `str` at 0x289fc | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x28a00 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x28a04 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x28a04 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x28a04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x285a8 | `(unknown callee) 0x23fbc` | `0x101e24` | `r0` | `r1` | `[0x2adfd8]` |
| `bl` at 0x285d4 | `(unknown callee) 0xec86c` | `?` | `0x6` | `0x59` | `sp-28` |
| `bl` at 0x285e8 | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x4f` | `sp-28` |
| `bl` at 0x285fc | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x45` | `sp-28` |
| `bl` at 0x28610 | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x3b` | `sp-28` |
| `bl` at 0x28624 | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x31` | `sp-28` |
| `bl` at 0x28638 | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x27` | `sp-28` |
| `bl` at 0x2864c | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x1d` | `sp-28` |
| `bl` at 0x28660 | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x13` | `sp-28` |
| `bl` at 0x28678 | `(unknown callee) 0x23fbc` | `0x101e50` | `?` | `?` | `[0x2adfd8]` |
| `bl` at 0x2868c | `(unknown callee) 0xed170` | `[sp-20]` | `0x2` | `0x6` | `0x0` |
| `bl` at 0x286c8 | `(unknown callee) 0xec86c` | `?` | `0x6` | `0x59` | `sp-28` |
| `bl` at 0x286dc | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x4f` | `sp-28` |
| `bl` at 0x286f0 | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x45` | `sp-28` |
| `bl` at 0x28704 | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x3b` | `sp-28` |
| `bl` at 0x28718 | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x31` | `sp-28` |
| `bl` at 0x2872c | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x27` | `sp-28` |
| `bl` at 0x28740 | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x1d` | `sp-28` |
| `bl` at 0x28754 | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x13` | `sp-28` |
| `bl` at 0x2876c | `(unknown callee) 0x23fbc` | `0x101e88` | `?` | `?` | `[0x2adfd8]` |
| `bl` at 0x28780 | `(unknown callee) 0xed170` | `[sp-20]` | `0x2` | `0x6` | `0x0` |
| `bl` at 0x287cc | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x5c` | `sp-28` |
| `bl` at 0x287e0 | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x52` | `sp-28` |
| `bl` at 0x287f4 | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x48` | `sp-28` |
| `bl` at 0x28808 | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x3e` | `sp-28` |
| `bl` at 0x2881c | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x34` | `sp-28` |
| `bl` at 0x28830 | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x2a` | `sp-28` |
| `bl` at 0x28844 | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x20` | `sp-28` |
| `bl` at 0x28858 | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x16` | `sp-28` |
| `bl` at 0x28874 | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x5b` | `sp-28` |
| `bl` at 0x28888 | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x51` | `sp-28` |
| `bl` at 0x2889c | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x47` | `sp-28` |
| `bl` at 0x288b0 | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x3d` | `sp-28` |
| `bl` at 0x288c4 | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x33` | `sp-28` |
| `bl` at 0x288d8 | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x29` | `sp-28` |
| `bl` at 0x288ec | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x1f` | `sp-28` |
| `bl` at 0x28900 | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x15` | `sp-28` |
| `bl` at 0x2891c | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x5a` | `sp-28` |
| `bl` at 0x28930 | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x50` | `sp-28` |
| `bl` at 0x28944 | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x46` | `sp-28` |
| `bl` at 0x28958 | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x3c` | `sp-28` |
| `bl` at 0x2896c | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x32` | `sp-28` |
| `bl` at 0x28980 | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x28` | `sp-28` |
| `bl` at 0x28994 | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x1e` | `sp-28` |
| `bl` at 0x289a8 | `(unknown callee) 0xec86c` | `[sp-20]` | `0x6` | `0x14` | `sp-28` |
| `bl` at 0x289c0 | `(unknown callee) 0x23fbc` | `0x101ec0` | `?` | `?` | `[0x2adfd8]` |
| `bl` at 0x289d4 | `(unknown callee) 0xed170` | `[sp-20]` | `0x2` | `0x6` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x28598 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `beq` at 0x285b8 | `? == 0x0` | `? != 0x0` |
| `bls` at 0x28670 | `[0x2adfd8] <= 0x1 (unsigned, 32-bit)` | `[0x2adfd8] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x286a0 | `ret(0xed170) >= 0x0 (signed, 32-bit)` | `ret(0xed170) < 0x0 (signed, 32-bit)` |
| `bls` at 0x28764 | `[0x2adfd8] <= 0x1 (unsigned, 32-bit)` | `[0x2adfd8] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x28794 | `ret(0xed170) >= 0x0 (signed, 32-bit)` | `ret(0xed170) < 0x0 (signed, 32-bit)` |
| `beq` at 0x287b0 | `? == 0x0` | `? != 0x0` |
| `bls` at 0x289b8 | `[0x2adfd8] <= 0x1 (unsigned, 32-bit)` | `[0x2adfd8] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x289e8 | `ret(0xed170) >= 0x0 (signed, 32-bit)` | `ret(0xed170) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x28a04 | `r0` | `?` |

## `nl4r_setup_llp`

- `nl4r_setup_llp`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x28e24 to 0x2928c.
  Established. The word assignment at `sp -0x20` writes `0x190` (`str` at 0x2910c). The unnamed
  `0xec86c` call at `0x2912c` carries `[sp-20]`, `[sp-24]+10`, `0x6`, `sp-32` (`bl` at 0x2912c). The
  return reports `0x0` (`ldmdb` at 0x29280). LLP setup effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x28e34 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x28e38 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x28e3c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x28e40 | load | 4 | `abs 0x29284` | `-` |
| `ldr` at 0x28e44 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x28e50 | load | 4 | `abs 0x29288` | `-` |
| `ldr` at 0x28e54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28e58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28e60 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x28e70 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x28e74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28e80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28e94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28ea0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28eb8 | store | 4 | `sp -0x20` | `0x18` |
| `ldr` at 0x28ebc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28ec8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28edc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28ee8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28efc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28f08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28f1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28f28 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28f44 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x28f48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28f54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28f68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28f74 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28f8c | store | 4 | `sp -0x20` | `0xf` |
| `ldr` at 0x28f90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28f9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28fb0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28fbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28fd0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28fdc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28ff0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28ffc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x29014 | store | 4 | `sp -0x20` | `0x3f` |
| `ldr` at 0x29018 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29024 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29038 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29044 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29058 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29064 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29078 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29084 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2909c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x290a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x290ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x290c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x290cc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x290e4 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x290e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x290f4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2910c | store | 4 | `sp -0x20` | `0x190` |
| `ldr` at 0x29110 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2911c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29130 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x29140 | store | 4 | `sp -0x28` | `0x3` |
| `str` at 0x2914c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x29150 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x29154 | store | 4 | `sp -0x20` | `[sp-40]` |
| `ldr` at 0x29158 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29164 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2917c | store | 4 | `sp -0x20` | `0x2` |
| `ldr` at 0x29180 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2918c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x291a4 | store | 4 | `sp -0x20` | `0x2` |
| `ldr` at 0x291a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x291b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x291c8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x291d8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x291dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x291e8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x29200 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x29204 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29210 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2922c | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x29230 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2923c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x29254 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x29258 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29264 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x29280 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x29280 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x29280 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x28e5c | `(unknown callee) 0x23fbc` | `0x101ff0` | `r0` | `r1` | `[0x2adfd8]` |
| `bl` at 0x28e90 | `(unknown callee) 0xec86c` | `?` | `?` | `0x0` | `sp-32` |
| `bl` at 0x28eb0 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+10` | `0x3` | `sp-32` |
| `bl` at 0x28ed8 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+10` | `0x1f` | `sp-32` |
| `bl` at 0x28ef8 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+10` | `0x20` | `sp-32` |
| `bl` at 0x28f18 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+10` | `0x21` | `sp-32` |
| `bl` at 0x28f38 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+10` | `0x22` | `sp-32` |
| `bl` at 0x28f64 | `(unknown callee) 0xec86c` | `?` | `?` | `0x0` | `sp-32` |
| `bl` at 0x28f84 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+10` | `0x3` | `sp-32` |
| `bl` at 0x28fac | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+10` | `0x1f` | `sp-32` |
| `bl` at 0x28fcc | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+10` | `0x20` | `sp-32` |
| `bl` at 0x28fec | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+10` | `0x21` | `sp-32` |
| `bl` at 0x2900c | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+10` | `0x22` | `sp-32` |
| `bl` at 0x29034 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+10` | `0x23` | `sp-32` |
| `bl` at 0x29054 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+10` | `0x24` | `sp-32` |
| `bl` at 0x29074 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+10` | `0x25` | `sp-32` |
| `bl` at 0x29094 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+10` | `0x26` | `sp-32` |
| `bl` at 0x290bc | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+10` | `0x2` | `sp-32` |
| `bl` at 0x290dc | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+10` | `0x4` | `sp-32` |
| `bl` at 0x29104 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+10` | `0x5` | `sp-32` |
| `bl` at 0x2912c | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+10` | `0x6` | `sp-32` |
| `bl` at 0x29174 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+18` | `0x2` | `sp-32` |
| `bl` at 0x2919c | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+18` | `0x4` | `sp-32` |
| `bl` at 0x291c4 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+18` | `0x5` | `sp-32` |
| `bl` at 0x291f8 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+26` | `0x0` | `sp-32` |
| `bl` at 0x29220 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+26` | `0x4` | `sp-32` |
| `bl` at 0x2924c | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+26` | `0x0` | `sp-32` |
| `bl` at 0x29274 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+26` | `0x4` | `sp-32` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x28e4c | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `beq` at 0x28e68 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x29138 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `beq` at 0x291d0 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x29280 | `r0` | `0x0` |

## `nl4r_set_link_enable`

- `nl4r_set_link_enable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2928c to 0x29510.
  Established. The unnamed `0xec86c` call at `0x293e0` carries `[sp-20]`, `[sp-24]+18`, `0x0`,
  `sp-32` (`bl` at 0x293e0). A later call to that target carries `[sp-20]`, `[sp-24]+18`, `0x2`,
  `sp-32` (`bl` at 0x29408). The signed condition `[sp-36] >= 0x0` takes `0x29478`, while negative
  falls through (`bge` at 0x29468). Link-enable semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2929c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x292a0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x292a4 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x292a8 | load | 4 | `abs 0x294f4` | `-` |
| `ldr` at 0x292ac | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x292b8 | load | 4 | `abs 0x294f8` | `-` |
| `ldr` at 0x292bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x292c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x292c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x292cc | load | 4 | `abs 0x294fc` | `-` |
| `ldr` at 0x292d0 | load | 4 | `abs 0x14928c` | `-` |
| `ldr` at 0x292dc | load | 4 | `abs 0x294f8` | `-` |
| `str` at 0x292e0 | store | 4 | `sp -0x44` | `0x10201c` |
| `ldr` at 0x292e4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x292e8 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x292ec | load | 4 | `sp -0x18` | `-` |
| `str` at 0x292f0 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x292f4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x292f8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x29300 | load | 4 | `abs 0x29500` | `-` |
| `ldr` at 0x29304 | load | 4 | `abs 0x29504` | `-` |
| `ldr` at 0x29310 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x29314 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x29318 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29324 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29338 | load | 4 | `abs 0x294f4` | `-` |
| `ldr` at 0x2933c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x29348 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29350 | load | 4 | `abs 0x29508` | `-` |
| `ldr` at 0x29358 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x29364 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2936c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x29384 | store | 4 | `sp -0x24` | `ret(0xed170)` |
| `ldr` at 0x2938c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x29398 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x2939c | store | 4 | `sp -0x34` | `[sp-36]` |
| `ldr` at 0x293a4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x293b4 | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0x293c0 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x293c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x293d0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x293e8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x293ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x293f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2940c | load | 4 | `abs 0x294f4` | `-` |
| `ldr` at 0x29410 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x2941c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29424 | load | 4 | `abs 0x2950c` | `-` |
| `ldr` at 0x2942c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x29438 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29440 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x29458 | store | 4 | `sp -0x24` | `ret(0xed170)` |
| `ldr` at 0x29460 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x2946c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x29470 | store | 4 | `sp -0x34` | `[sp-36]` |
| `str` at 0x2947c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x29480 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2948c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x294a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x294ac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x294c4 | store | 4 | `sp -0x24` | `ret(0xed170)` |
| `ldr` at 0x294cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x294d8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x294dc | store | 4 | `sp -0x34` | `[sp-36]` |
| `str` at 0x294e8 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x294ec | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x294f0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x294f0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x294f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x292c8 | `(unknown callee) 0x23fbc` | `0x10201c` | `r0` | `r1` | `r2` |
| `bl` at 0x2930c | `(unknown callee) 0x21eb8` | `0x6` | `0x101c80` | `0x343` | `0x0` |
| `bl` at 0x29334 | `(unknown callee) 0xec86c` | `?` | `?` | `0x1` | `sp-32` |
| `bl` at 0x2935c | `(unknown callee) 0x23fbc` | `0x102054` | `[sp-24]+65` | `[sp-28]` | `[sp-24]+65` |
| `bl` at 0x29360 | `(unknown callee) 0xe3cf8` | `?` | `?` | `?` | `?` |
| `bl` at 0x2937c | `(unknown callee) 0xed170` | `[sp-20]` | `0x3` | `[sp-24]+10` | `0x0` |
| `bl` at 0x29388 | `(unknown callee) 0xe3d64` | `ret(0xed170)` | `?` | `?` | `ret(0xed170)` |
| `bl` at 0x293e0 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+18` | `0x0` | `sp-32` |
| `bl` at 0x29408 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+18` | `0x2` | `sp-32` |
| `bl` at 0x29430 | `(unknown callee) 0x23fbc` | `0x102088` | `[sp-24]+65` | `[sp-28]` | `[sp-24]+65` |
| `bl` at 0x29434 | `(unknown callee) 0xe3cf8` | `?` | `?` | `?` | `?` |
| `bl` at 0x29450 | `(unknown callee) 0xed170` | `[sp-20]` | `0x3` | `[sp-24]+18` | `0x0` |
| `bl` at 0x2945c | `(unknown callee) 0xe3d64` | `ret(0xed170)` | `?` | `?` | `ret(0xed170)` |
| `bl` at 0x2949c | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+18` | `0x0` | `sp-32` |
| `bl` at 0x294a0 | `(unknown callee) 0xe3cf8` | `ret(0xec86c)` | `?` | `?` | `?` |
| `bl` at 0x294bc | `(unknown callee) 0xed170` | `[sp-20]` | `0x3` | `[sp-24]+18` | `0x0` |
| `bl` at 0x294c8 | `(unknown callee) 0xe3d64` | `ret(0xed170)` | `?` | `?` | `ret(0xed170)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x292b4 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `beq` at 0x292d8 | `[0x14928c] == 0x0` | `[0x14928c] != 0x0` |
| `bls` at 0x29344 | `[0x2adfd8] <= 0x1 (unsigned, 32-bit)` | `[0x2adfd8] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x29394 | `[sp-36] >= 0x0 (signed, 32-bit)` | `[sp-36] < 0x0 (signed, 32-bit)` |
| `bne` at 0x293ac | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bls` at 0x29418 | `[0x2adfd8] <= 0x1 (unsigned, 32-bit)` | `[0x2adfd8] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x29468 | `[sp-36] >= 0x0 (signed, 32-bit)` | `[sp-36] < 0x0 (signed, 32-bit)` |
| `bge` at 0x294d4 | `[sp-36] >= 0x0 (signed, 32-bit)` | `[sp-36] < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x294f0 | `r0` | `?` |

## `nl4r_is_scramble_needed`

- `nl4r_is_scramble_needed`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x29510 to 0x295d8.
  Established. The unnamed `0x753bc` call carries `[sp-24]+1`, unknown, `ret(0x1f26c)`, `[sp-24]+1`
  (`bl` at 0x29584). The word read at `ret(0x753bc) +0x10` supplies the comparison expression (`ldr` at 0x29594). Equality `[ret(0x753bc)+16] == 0x44` takes `0x295ac`, while inequality falls through
  (`beq` at 0x2959c). Scrambling requirements remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x29520 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x29524 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x29540 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x29550 | store | 4 | `sp -0x20` | `ret(0x1f26c)` |
| `ldr` at 0x29554 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x29558 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x29570 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x29578 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2958c | store | 4 | `sp -0x24` | `ret(0x753bc)` |
| `ldr` at 0x29590 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x29594 | load | 4 | `ret(0x753bc) +0x10` | `-` |
| `str` at 0x295a4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x295ac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x295b0 | load | 4 | `ret(0x753bc) +0x18` | `-` |
| `str` at 0x295c0 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x295cc | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x295d0 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x295d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x295d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x295d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x29528 | `(unknown callee) 0x4f4d0` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x2954c | `(unknown callee) 0x1f26c` | `0x98` | `?` | `?` | `?` |
| `bl` at 0x29584 | `(unknown callee) 0x753bc` | `[sp-24]+1` | `?` | `ret(0x1f26c)` | `[sp-24]+1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x29538 | `? == 0x44` | `? != 0x44` |
| `bne` at 0x29568 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x2959c | `[ret(0x753bc)+16] == 0x44` | `[ret(0x753bc)+16] != 0x44` |
| `bne` at 0x295b8 | `[ret(0x753bc)+24] != 0x0` | `[ret(0x753bc)+24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x295d4 | `r0` | `?` |

## `JChip_MMR_SC_stub`

- `JChip_MMR_SC_stub`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe8564 to 0xe8584.
  Established. The word assignment at `sp -0x14` writes `r0` (`str` at 0xe8574). The reported return
  value is `0x0` (`ldmdb` at 0xe8580). MMR side effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe8574 | store | 4 | `sp -0x14` | `r0` |
| `ldmdb` at 0xe8580 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe8580 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe8580 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe8580 | `r0` | `0x0` |

## `JTAG_LLP_SC_stub`

- `JTAG_LLP_SC_stub`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe8584 to 0xe85a4.
  Established. Entry `r0` is saved as a word at `sp -0x14` (`str` at 0xe8594). The return reports
  `0x0` (`ldmdb` at 0xe85a0). LLP control effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe8594 | store | 4 | `sp -0x14` | `r0` |
| `ldmdb` at 0xe85a0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe85a0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe85a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe85a0 | `r0` | `0x0` |

## `JChip_link_summary_stub`

- `JChip_link_summary_stub`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe85a4 to 0xe85f4.
  Established. The unnamed `0x202d0` call at `0xe85c8` carries `0x136868`, `r1`, `r2`, `r3` (`bl` at 0xe85c8). A byte at `[sp-32] +0x0` receives `0x0` (`strb` at 0xe85d4). The return reports `0x0`
  (`ldmdb` at 0xe85e8). Link-summary contents remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe85b4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe85b8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe85bc | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe85c0 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xe85c4 | load | 4 | `abs 0xe85ec` | `-` |
| `ldr` at 0xe85cc | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xe85d4 | store | 1 | `[sp-32] +0x0` | `0x0` |
| `ldr` at 0xe85d8 | load | 4 | `abs 0xe85f0` | `-` |
| `ldmdb` at 0xe85e8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe85e8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe85e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe85c8 | `(unknown callee) 0x202d0` | `0x136868` | `r1` | `r2` | `r3` |
| `bl` at 0xe85dc | `(unknown callee) 0x202d0` | `0x13686c` | `?` | `[sp-32]` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe85e8 | `r0` | `0x0` |

## `JTAG_Cfg_L3SetCfg_stub`

- `JTAG_Cfg_L3SetCfg_stub`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe8510 to 0xe8530.
  Established. The word store at `sp -0x14` retains `r0` (`str` at 0xe8520). The reported return is
  `0x1` (`ldmdb` at 0xe852c). Configuration effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe8520 | store | 4 | `sp -0x14` | `r0` |
| `ldmdb` at 0xe852c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe852c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe852c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe852c | `r0` | `0x1` |
