# ARM IP65 node and power callers in 1.40.5

Read from `6.5.29/usr/cpu/firmware/sysco/l1.bin`, package `6.5.29:l1#4` (version 1.40.5), inner
executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53d448`.
Carrier SHA-256: `80f4a616601d187bc3839a2758f2e7f2d0db962ad7fd8f2d96d41bcd15bdf88c`.
Package SHA-256: `4ccd1a6e7054bc3cd56e962a60737d1c90339ffa93fcc8af0912b6c9c7d385dd`.
Inner executable SHA-256: `08ebc648bb298017c7e3d6049983f2a54a8fe56f1bb5738df88b82c38f75ac3a`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at virtual
address `0x8000`, file offset `0x0`, and contains 1257088 bytes.
Segment SHA-256: `dc7cc5d18fbdf816d328411825c9c01e26c3f8852f1012c7a642aea0767e2665`. Analysed
routine spans use the exact ELF function symbols and sizes. They can include trailing literal words
after a return.

`sp` denotes the entry stack pointer. Register expressions such as `r0` denote entry values, and
brackets denote loaded values. Decimal numbers inside expressions retain the exact tool notation.
Call candidates use the assumed ARM AAPCS register locations r0 through r3 with unknown arity. Stack
stores appear in the access tables. Unknown values and predicates include unsupported instruction
effects and merged paths. Numeric call targets have unknown callee names in the flat-image analysis.
Device roles and pointee meanings remain unknown unless the body establishes them. Tables include
all explicit memory operands on known control-flow edges. Implicit stack saves appear in the full
fact sheets. Separate loads at the same printed address need not observe the same value. These
routines have no unresolved computed transfers in the fact sheets. Numeric callee effects remain
unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/ip65-node-power-facts.txt) retain all
tool sections.

## `IP65_NodeRemove`

- `IP65_NodeRemove`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x646c4 to 0x647d8.
  Established. The unnamed `0x65204` call has candidates `(r0 & 0xff)`, `sp-24`, `sp-24`,
  `(r0 & 0xff)` (`bl` at 0x646ec); its zero result takes the branch to `0x64710` (`beq` at 0x64700).
  An unnamed `0x648e0` call then carries `[sp-17]:1`, `0x1`, `0x1`, `[sp-17]:1` (`bl` at
  0x64720). Later byte stores include `0xf` at `[sp-17]:1 +0x2b16c8` (`strb` at 0x647c0). The two
  signed-less loop predicates have unknown operands (`blt` at 0x64744, `blt` at 0x6478c).
  Node-removal effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x646d8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x646dc | load | 1 | `sp -0x11` | `-` |
| `str` at 0x646f4 | store | 4 | `sp -0x1c` | `ret(0x65204)` |
| `ldr` at 0x646f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x64704 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x64708 | store | 4 | `sp -0x24` | `ret(0x65204)` |
| `ldrb` at 0x64710 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x64724 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x64728 | load | 1 | `[sp-24] +0x2` | `-` |
| `strb` at 0x6472c | store | 1 | `sp -0x1d` | `([[sp-24]+2]:1 & 0xff)` |
| `ldrb` at 0x64730 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x64734 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x64738 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0x6474c | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x6475c | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x64764 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldr` at 0x6476c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x64770 | load | 1 | `[sp-24] +0x0` | `-` |
| `strb` at 0x64774 | store | 1 | `sp -0x1d` | `([[sp-24]]:1 & 0xff)` |
| `ldrb` at 0x64778 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x6477c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x64780 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldrb` at 0x64794 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x647a4 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x647ac | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldr` at 0x647b4 | load | 4 | `abs 0x647d4` | `-` |
| `ldrb` at 0x647b8 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x647c0 | store | 1 | `[sp-17]:1 +0x2b16c8` | `0xf` |
| `str` at 0x647c8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x647cc | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x647d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x647d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x647d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x646ec | `(unknown callee) 0x65204` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x64720 | `(unknown callee) 0x648e0` | `[sp-17]:1` | `0x1` | `0x1` | `[sp-17]:1` |
| `bl` at 0x64758 | `(unknown callee) 0xd21b4` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x647a0 | `(unknown callee) 0xd513c` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x64700 | `ret(0x65204) == 0x0` | `ret(0x65204) != 0x0` |
| `blt` at 0x64744 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x6478c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x647d0 | `r0` | `?` |

## `IP65_NodeDisable`

- `IP65_NodeDisable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x647d8 to 0x64810.
  Established. The sole call targets unnamed `0x648e0` with `(r0 & 0xff)`, `0x0`, `0x1`,
  `(r0 & 0xff)` (`bl` at 0x64800). The exit carries `ret(0x648e0)` (`ldmdb` at 0x6480c). The body
  has no conditional branch. Disable signaling remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x647ec | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x647f0 | load | 1 | `sp -0x11` | `-` |
| `ldmdb` at 0x6480c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6480c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6480c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x64800 | `(unknown callee) 0x648e0` | `(r0 & 0xff)` | `0x0` | `0x1` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6480c | `r0` | `ret(0x648e0)` |

## `IP65_NodeEnable`

- `IP65_NodeEnable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x64810 to 0x64844.
  Established. Entry `r0` is saved as byte `(r0 & 0xff)` at `sp -0x11` (`strb` at 0x64824). The
  unnamed call to `0x64aa4` carries `(r0 & 0xff)`, `0x0`, `r2`, `(r0 & 0xff)` (`bl` at 0x64834);
  its result reaches the return register (`ldmdb` at 0x64840). Node-enable effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x64824 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x64828 | load | 1 | `sp -0x11` | `-` |
| `ldmdb` at 0x64840 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x64840 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x64840 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x64834 | `(unknown callee) 0x64aa4` | `(r0 & 0xff)` | `0x0` | `r2` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x64840 | `r0` | `ret(0x64aa4)` |

## `IP65_NodePresent`

- `IP65_NodePresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x64cc4 to 0x64d00.
  Established. A byte read uses `(r0 & 0xff) +0x2b16c8` (`ldrb` at 0x64ce4). The exit value remains
  unknown in the fact sheet (`ldmdb` at 0x64cf8). There are no calls or conditional branches in this
  span. Presence interpretation is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x64cd8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x64cdc | load | 4 | `abs 0x64cfc` | `-` |
| `ldrb` at 0x64ce0 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x64ce4 | load | 1 | `(r0 & 0xff) +0x2b16c8` | `-` |
| `ldmdb` at 0x64cf8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x64cf8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x64cf8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x64cf8 | `r0` | `?` |

## `IP65_CpuPresent`

- `IP65_CpuPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x64d00 to 0x64d54.
  Established. The unnamed `0x1f26c` call uses `0x7b`, `r1`, `r2`, `r0` (`bl` at 0x64d1c). A
  result unequal to `0x1` takes the branch to `0x64d38` (`bne` at 0x64d28). That arm includes a byte
  read at `[sp-17]:1 +0x14927c`, while the fall-through assigns four-byte zero at `sp -0x18`
  (`ldrb` at 0x64d40, `str` at 0x64d30). CPU-presence policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x64d14 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x64d30 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x64d38 | load | 4 | `abs 0x64d50` | `-` |
| `ldrb` at 0x64d3c | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x64d40 | load | 1 | `[sp-17]:1 +0x14927c` | `-` |
| `str` at 0x64d44 | store | 4 | `sp -0x18` | `[[sp-17]:1+1348220]:1` |
| `ldr` at 0x64d48 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x64d4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x64d4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x64d4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x64d1c | `(unknown callee) 0x1f26c` | `0x7b` | `r1` | `r2` | `r0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x64d28 | `ret(0x1f26c) != 0x1` | `ret(0x1f26c) == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x64d4c | `r0` | `?` |

## `IP65_CpuDisable`

- `IP65_CpuDisable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x64d54 to 0x64db0.
  Established. The first unnamed call targets `0x64d00` with `(r0 & 0xff)`, `r1`, `r2`,
  `(r0 & 0xff)` (`bl` at 0x64d74). A zero result takes the branch to `0x64d9c`, where a four-byte
  stack assignment retains `0xfffff252` (`beq` at 0x64d80, `str` at 0x64da0). The other arm calls
  unnamed `0x64e0c` with `[sp-17]:1`, unknown, unknown, `[sp-17]:1` (`bl` at 0x64d8c). CPU-disable
  effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x64d68 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x64d6c | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x64d84 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x64d94 | store | 4 | `sp -0x18` | `ret(0x64e0c)` |
| `ldr` at 0x64d9c | load | 4 | `abs 0x64dac` | `-` |
| `str` at 0x64da0 | store | 4 | `sp -0x18` | `0xfffff252` |
| `ldr` at 0x64da4 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x64da8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x64da8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x64da8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x64d74 | `(unknown callee) 0x64d00` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x64d8c | `(unknown callee) 0x64e0c` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x64d80 | `ret(0x64d00) == 0x0` | `ret(0x64d00) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x64da8 | `r0` | `?` |

## `IP65_CpuEnable`

- `IP65_CpuEnable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x64db0 to 0x64e0c.
  Established. A zero `ret(0x64d00)` takes the branch to `0x64df8` (`beq` at 0x64ddc); its
  fall-through includes unnamed `0x64f3c` with `[sp-17]:1`, unknown, unknown, `[sp-17]:1` (`bl` at
  0x64de8). The two arms store `ret(0x64f3c)` and `0xfffff252` as four bytes at `sp -0x18` (`str` at
  0x64df0, `str` at 0x64dfc). CPU-enable behavior is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x64dc4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x64dc8 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x64de0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x64df0 | store | 4 | `sp -0x18` | `ret(0x64f3c)` |
| `ldr` at 0x64df8 | load | 4 | `abs 0x64e08` | `-` |
| `str` at 0x64dfc | store | 4 | `sp -0x18` | `0xfffff252` |
| `ldr` at 0x64e00 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x64e04 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x64e04 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x64e04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x64dd0 | `(unknown callee) 0x64d00` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x64de8 | `(unknown callee) 0x64f3c` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x64ddc | `ret(0x64d00) == 0x0` | `ret(0x64d00) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x64e04 | `r0` | `?` |

## `IP65_PowerUp`

- `IP65_PowerUp`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x64fe8 to 0x65044.
  Established. The call to unnamed `0x65204` carries `(r0 & 0xff)`, `sp-24`, `sp-24`,
  `(r0 & 0xff)` (`bl` at 0x65010). The branch takes zero to `0x65034` and falls through on nonzero
  (`beq` at 0x65024). The arms assign four-byte `ret(0x65204)` or `0x0` at `sp -0x20`, and the exit
  carries `[sp-32]` (`str` at 0x6502c, `str` at 0x65038, `ldmdb` at 0x65040). Physical power-up
  behavior is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x64ffc | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x65000 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x65018 | store | 4 | `sp -0x1c` | `ret(0x65204)` |
| `ldr` at 0x6501c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65028 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6502c | store | 4 | `sp -0x20` | `ret(0x65204)` |
| `str` at 0x65038 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x6503c | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x65040 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x65040 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x65040 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x65010 | `(unknown callee) 0x65204` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x65024 | `ret(0x65204) == 0x0` | `ret(0x65204) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x65040 | `r0` | `[sp-32]` |

## `IP65_PowerDown`

- `IP65_PowerDown`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x65044 to 0x650a0.
  Established. An unnamed `0x65204` call supplies `(r0 & 0xff)`, `sp-24`, `sp-24`, `(r0 & 0xff)`
  (`bl` at 0x6506c). The zero-result branch reaches `0x65090` (`beq` at 0x65080). The nonzero arm
  stores four-byte `ret(0x65204)` at `sp -0x20`, while the zero arm stores `0x0` there (`str` at
  0x65088, `str` at 0x65094). Power-down effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x65058 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x6505c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x65074 | store | 4 | `sp -0x1c` | `ret(0x65204)` |
| `ldr` at 0x65078 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65084 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x65088 | store | 4 | `sp -0x20` | `ret(0x65204)` |
| `str` at 0x65094 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x65098 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x6509c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6509c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6509c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6506c | `(unknown callee) 0x65204` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x65080 | `ret(0x65204) == 0x0` | `ret(0x65204) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6509c | `r0` | `[sp-32]` |

## `IP65_ResetHold`

- `IP65_ResetHold`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x650a0 to 0x650c4.
  Established. The only explicit store saves byte `(r0 & 0xff)` at `sp -0x11` (`strb` at 0x650b4).
  This span makes no calls and contains no conditional branch. The exit carries `0x0` (`ldmdb` at
  0x650c0). Reset-hold signaling is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x650b4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldmdb` at 0x650c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x650c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x650c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x650c0 | `r0` | `0x0` |

## `IP65_ResetRelease`

- `IP65_ResetRelease`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x650c4 to 0x650e8.
  Established. The span ends with return-register value `0x0` (`ldmdb` at 0x650e4). Before that, a
  byte assignment at `sp -0x11` retains `(r0 & 0xff)` (`strb` at 0x650d8). No call or conditional
  branch appears. Reset-release behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x650d8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldmdb` at 0x650e4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x650e4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x650e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x650e4 | `r0` | `0x0` |

## `IP65_SoftReset`

- `IP65_SoftReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x650e8 to 0x6510c.
  Established. Byte `(r0 & 0xff)` is written to `sp -0x11` (`strb` at 0x650fc). The call and
  conditional-branch lists are empty. The return value is `0x0` (`ldmdb` at 0x65108). A soft-reset
  mechanism is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x650fc | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldmdb` at 0x65108 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x65108 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x65108 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x65108 | `r0` | `0x0` |

## `IP65_Nmi`

- `IP65_Nmi`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6510c to 0x65130.
  Established. No call or conditional branch occurs in this body; its explicit byte store retains
  `(r0 & 0xff)` at `sp -0x11`, and the exit carries `0x0` (`strb` at 0x65120, `ldmdb` at 0x6512c).
  NMI delivery remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x65120 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldmdb` at 0x6512c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6512c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6512c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6512c | `r0` | `0x0` |

## `IP65_Int`

- `IP65_Int`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x65130 to 0x65174.
  Established. A four-byte load reads entry `sp +0x0` (`ldr` at 0x65144). Five byte assignments
  retain `(r0 & 0xff)`, `(r1 & 0xff)`, `(r2 & 0xff)`, `(r3 & 0xff)` and `([sp] & 0xff)` at
  `sp -0x11` through `sp -0x15`, respectively (`strb` at 0x6514c, `strb` at 0x65154, `strb` at
  0x6515c, `strb` at 0x65164, `strb` at 0x6516c). The exit preserves entry `r0` (`ldmdb` at
  0x65170). Interrupt handling beyond these stack accesses is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x65144 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0x6514c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x65154 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0x6515c | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `strb` at 0x65164 | store | 1 | `sp -0x14` | `(r3 & 0xff)` |
| `strb` at 0x6516c | store | 1 | `sp -0x15` | `([sp] & 0xff)` |
| `ldmdb` at 0x65170 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x65170 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x65170 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x65170 | `r0` | `r0` |

## `IP65_Get_Cfg`

- `IP65_Get_Cfg`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x65174 to 0x651d4.
  Established. Nonzero `[0x149274]` takes the branch to `0x651a4` (`bne` at 0x65194). The zero
  fall-through writes four-byte `0xfffff254` at `sp -0x18` (`str` at 0x6519c). The nonzero arm
  assigns four-byte `[0x149274]` at `r0 +0x0`, then reads four-byte absolute `0x149270` (`str` at
  0x651b0, `ldr` at 0x651b8). Configuration pointer and result meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x65184 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x65188 | load | 4 | `abs 0x651c8` | `-` |
| `ldr` at 0x6518c | load | 4 | `abs 0x149274` | `-` |
| `ldr` at 0x65198 | load | 4 | `abs 0x651cc` | `-` |
| `str` at 0x6519c | store | 4 | `sp -0x18` | `0xfffff254` |
| `ldr` at 0x651a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x651a8 | load | 4 | `abs 0x651c8` | `-` |
| `ldr` at 0x651ac | load | 4 | `abs 0x149274` | `-` |
| `str` at 0x651b0 | store | 4 | `r0 +0x0` | `[0x149274]` |
| `ldr` at 0x651b4 | load | 4 | `abs 0x651d0` | `-` |
| `ldr` at 0x651b8 | load | 4 | `abs 0x149270` | `-` |
| `str` at 0x651bc | store | 4 | `sp -0x18` | `[0x149270]` |
| `ldr` at 0x651c0 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x651c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x651c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x651c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x65194 | `[0x149274] != 0x0` | `[0x149274] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x651c4 | `r0` | `?` |

