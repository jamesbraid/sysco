# ARM display and panel control in 1.40.5

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
The selected spans have no unresolved computed transfers. Numeric callee names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/display-panel-control-facts.txt) retain all tool sections.

## `displayProbe`

- `displayProbe`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x516d8 to 0x5188c.
  Established. The unnamed `0xd7530` call carries `0x0`, unknown, `sp-29`, unknown (`bl` at
  0x5172c); its zero result takes `0x51784`, and nonzero falls through (`beq` at 0x51740). Included
  absolute byte assignments write `0x1` at `0x148fd5` (`strb` at 0x51798) and `0x10` at `0x14902a`
  (`strb` at 0x51864). The indirect call has an unknown target and candidates `0x149028`, unknown,
  `0x149028`, `[0x1436d8]` (`mov` at 0x51808). Probe effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x516ec | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x516f0 | load | 4 | `abs 0x5186c` | `-` |
| `ldrb` at 0x516f4 | load | 1 | `abs 0x1434ac` | `-` |
| `ldr` at 0x51700 | load | 4 | `abs 0x51870` | `-` |
| `str` at 0x51704 | store | 4 | `sp -0x28` | `0x10ba40` |
| `ldr` at 0x5170c | load | 4 | `abs 0x51874` | `-` |
| `ldrb` at 0x5171c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x51734 | store | 4 | `sp -0x1c` | `ret(0xd7530)` |
| `ldr` at 0x51738 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x51744 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x51750 | load | 4 | `abs 0x51878` | `-` |
| `str` at 0x51754 | store | 4 | `sp -0x28` | `0x10ba70` |
| `str` at 0x51758 | store | 4 | `sp -0x24` | `ret(0x21a38)` |
| `ldr` at 0x51760 | load | 4 | `abs 0x51874` | `-` |
| `ldrb` at 0x51770 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x51784 | load | 4 | `abs 0x5186c` | `-` |
| `ldrb` at 0x51788 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x5178c | store | 1 | `abs 0x1434ac` | `([sp-17]:1 & 0xff)` |
| `ldr` at 0x51790 | load | 4 | `abs 0x5187c` | `-` |
| `strb` at 0x51798 | store | 1 | `abs 0x148fd5` | `0x1` |
| `ldr` at 0x5179c | load | 4 | `abs 0x5186c` | `-` |
| `ldrb` at 0x517a0 | load | 1 | `abs 0x1434ac` | `-` |
| `str` at 0x517b4 | store | 4 | `sp -0x18` | `ret(0xd7f54)` |
| `ldr` at 0x517b8 | load | 4 | `abs 0x51880` | `-` |
| `ldr` at 0x517bc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x517c0 | load | 1 | `ret(0xd7f54) +0x1` | `-` |
| `strb` at 0x517c4 | store | 1 | `abs 0x149028` | `([ret(0xd7f54)+1]:1 & 0xff)` |
| `ldr` at 0x517c8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x517cc | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x517d8 | load | 4 | `abs 0x51880` | `-` |
| `strb` at 0x517e0 | store | 1 | `abs 0x149029` | `0x76` |
| `ldr` at 0x517e4 | load | 4 | `abs 0x51880` | `-` |
| `ldr` at 0x517e8 | load | 4 | `abs 0x51880` | `-` |
| `ldrb` at 0x517ec | load | 1 | `abs 0x14902c` | `-` |
| `strb` at 0x517f4 | store | 1 | `abs 0x14902c` | `?` |
| `ldr` at 0x517f8 | load | 4 | `abs 0x51884` | `-` |
| `ldr` at 0x517fc | load | 4 | `abs 0x1436d8` | `-` |
| `ldr` at 0x51800 | load | 4 | `abs 0x51880` | `-` |
| `ldr` at 0x51810 | load | 4 | `abs 0x51880` | `-` |
| `strb` at 0x51818 | store | 1 | `abs 0x149029` | `?` |
| `ldr` at 0x5181c | load | 4 | `abs 0x51880` | `-` |
| `ldr` at 0x51824 | load | 4 | `abs 0x51880` | `-` |
| `ldrb` at 0x51828 | load | 1 | `abs 0x14902a` | `-` |
| `ldr` at 0x51834 | load | 4 | `abs 0x51888` | `-` |
| `str` at 0x51838 | store | 4 | `sp -0x28` | `0x10ba9c` |
| `ldr` at 0x5183c | load | 4 | `abs 0x51880` | `-` |
| `ldrb` at 0x51840 | load | 1 | `abs 0x14902a` | `-` |
| `str` at 0x51844 | store | 4 | `sp -0x24` | `[0x14902a]:1` |
| `ldr` at 0x5184c | load | 4 | `abs 0x51874` | `-` |
| `ldr` at 0x5185c | load | 4 | `abs 0x51880` | `-` |
| `strb` at 0x51864 | store | 1 | `abs 0x14902a` | `0x10` |
| `ldmdb` at 0x51868 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x51868 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x51868 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x51718 | `(unknown callee) 0x21eb8` | `0x9` | `0x10ba58` | `0x81` | `0x0` |
| `bl` at 0x5172c | `(unknown callee) 0xd7530` | `0x0` | `?` | `sp-29` | `?` |
| `bl` at 0x51748 | `(unknown callee) 0x21a38` | `ret(0xd7530)` | `?` | `?` | `ret(0xd7530)` |
| `bl` at 0x5176c | `(unknown callee) 0x21eb8` | `0x1` | `0x10ba58` | `0x87` | `0x0` |
| `bl` at 0x5177c | `(unknown callee) 0xd68bc` | `0x0` | `[sp-17]:1` | `?` | `[sp-17]:1` |
| `bl` at 0x517ac | `(unknown callee) 0xd7f54` | `0x0` | `[0x1434ac]:1` | `0x148fd5` | `[0x1434ac]:1` |
| `mov` at 0x51808 | `(unknown callee) (unknown target)` | `0x149028` | `?` | `0x149028` | `[0x1436d8]` |
| `bl` at 0x51820 | `(unknown callee) 0xdbac0` | `0x149028` | `?` | `?` | `0x149028` |
| `bl` at 0x51858 | `(unknown callee) 0x21eb8` | `0x1` | `0x10ba58` | `0xa4` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x516fc | `[0x1434ac]:1 == 0xff` | `[0x1434ac]:1 != 0xff` |
| `beq` at 0x51740 | `ret(0xd7530) == 0x0` | `ret(0xd7530) != 0x0` |
| `bne` at 0x517d4 | `[[sp-24]]:1 != 0x2` | `[[sp-24]]:1 == 0x2` |
| `bls` at 0x51830 | `[0x14902a]:1 <= 0x10 (unsigned, 32-bit)` | `[0x14902a]:1 > 0x10 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x51868 | `r0` | `?` |

## `panelProbe`

- `panelProbe`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5188c to 0x519ec.
  Established. The unnamed `0xd7530` call carries `0x0`, unknown, `sp-25`, unknown (`bl` at
  0x518e0); its zero result takes `0x51938`, while nonzero falls through (`beq` at 0x518f4).
  Included absolute assignments write byte `0x10` at `0x14902a` (`strb` at 0x51958) and word
  `0x52e84` at `0x14903c` (`str` at 0x519ac). Stored pointer roles and panel effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x518a0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x518a4 | load | 4 | `abs 0x519c0` | `-` |
| `ldrb` at 0x518a8 | load | 1 | `abs 0x1434ac` | `-` |
| `ldr` at 0x518b4 | load | 4 | `abs 0x519c4` | `-` |
| `str` at 0x518b8 | store | 4 | `sp -0x24` | `0x10ba40` |
| `ldr` at 0x518c0 | load | 4 | `abs 0x519c8` | `-` |
| `ldrb` at 0x518d0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x518e8 | store | 4 | `sp -0x18` | `ret(0xd7530)` |
| `ldr` at 0x518ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x518f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x51904 | load | 4 | `abs 0x519cc` | `-` |
| `str` at 0x51908 | store | 4 | `sp -0x24` | `0x10ba70` |
| `str` at 0x5190c | store | 4 | `sp -0x20` | `ret(0x21a38)` |
| `ldr` at 0x51914 | load | 4 | `abs 0x519c8` | `-` |
| `ldrb` at 0x51924 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x51938 | load | 4 | `abs 0x519c0` | `-` |
| `ldrb` at 0x5193c | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x51940 | store | 1 | `abs 0x1434ac` | `([sp-17]:1 & 0xff)` |
| `ldr` at 0x51944 | load | 4 | `abs 0x519d0` | `-` |
| `strb` at 0x5194c | store | 1 | `abs 0x148fd5` | `0x1` |
| `ldr` at 0x51950 | load | 4 | `abs 0x519d4` | `-` |
| `strb` at 0x51958 | store | 1 | `abs 0x14902a` | `0x10` |
| `ldr` at 0x5195c | load | 4 | `abs 0x519d4` | `-` |
| `strb` at 0x51964 | store | 1 | `abs 0x14902b` | `0x0` |
| `ldr` at 0x51968 | load | 4 | `abs 0x519d4` | `-` |
| `strb` at 0x51970 | store | 1 | `abs 0x14902c` | `0x0` |
| `ldr` at 0x51974 | load | 4 | `abs 0x519d4` | `-` |
| `strb` at 0x5197c | store | 1 | `abs 0x14902d` | `0x0` |
| `ldr` at 0x51980 | load | 4 | `abs 0x519d4` | `-` |
| `ldr` at 0x51984 | load | 4 | `abs 0x519d8` | `-` |
| `str` at 0x51988 | store | 4 | `abs 0x149030` | `0x148fd7` |
| `ldr` at 0x5198c | load | 4 | `abs 0x519d4` | `-` |
| `ldr` at 0x51990 | load | 4 | `abs 0x519dc` | `-` |
| `str` at 0x51994 | store | 4 | `abs 0x149034` | `0x52e30` |
| `ldr` at 0x51998 | load | 4 | `abs 0x519d4` | `-` |
| `ldr` at 0x5199c | load | 4 | `abs 0x519e0` | `-` |
| `str` at 0x519a0 | store | 4 | `abs 0x149038` | `0x52e60` |
| `ldr` at 0x519a4 | load | 4 | `abs 0x519d4` | `-` |
| `ldr` at 0x519a8 | load | 4 | `abs 0x519e4` | `-` |
| `str` at 0x519ac | store | 4 | `abs 0x14903c` | `0x52e84` |
| `ldr` at 0x519b0 | load | 4 | `abs 0x519d4` | `-` |
| `ldr` at 0x519b4 | load | 4 | `abs 0x519e8` | `-` |
| `str` at 0x519b8 | store | 4 | `abs 0x149040` | `0x52eb4` |
| `ldmdb` at 0x519bc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x519bc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x519bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x518cc | `(unknown callee) 0x21eb8` | `0x9` | `0x10ba58` | `0xb8` | `0x0` |
| `bl` at 0x518e0 | `(unknown callee) 0xd7530` | `0x0` | `?` | `sp-25` | `?` |
| `bl` at 0x518fc | `(unknown callee) 0x21a38` | `ret(0xd7530)` | `?` | `?` | `ret(0xd7530)` |
| `bl` at 0x51920 | `(unknown callee) 0x21eb8` | `0x1` | `0x10ba58` | `0xbe` | `0x0` |
| `bl` at 0x51930 | `(unknown callee) 0xd68bc` | `0x0` | `[sp-17]:1` | `?` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x518b0 | `[0x1434ac]:1 == 0xff` | `[0x1434ac]:1 != 0xff` |
| `beq` at 0x518f4 | `ret(0xd7530) == 0x0` | `ret(0xd7530) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x519bc | `r0` | `?` |

## `displayInit`

- `displayInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x519ec to 0x51b08.
  Established. The absolute byte at `0x148fd4` is assigned `0x1` (`strb` at 0x51a44). The unnamed
  `0x52358` call carries `0x1`, unknown, `0x148fd4`, `0x1` (`bl` at 0x51a4c); its zero result takes
  `0x51a90`, and nonzero falls through (`beq` at 0x51a60). The later unnamed `0x51b28` call carries
  `0x32`, `0x10bae8`, `0x0`, unknown (`bl` at 0x51a9c). Initialization effects beyond these calls
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x519fc | load | 4 | `abs 0x51ae4` | `-` |
| `ldrb` at 0x51a00 | load | 1 | `abs 0x1434ac` | `-` |
| `ldr` at 0x51a10 | load | 4 | `abs 0x51ae8` | `-` |
| `ldrb` at 0x51a14 | load | 1 | `abs 0x148fd4` | `-` |
| `ldr` at 0x51a20 | load | 4 | `abs 0x51aec` | `-` |
| `str` at 0x51a24 | store | 4 | `sp -0x1c` | `0x10bab4` |
| `ldr` at 0x51a2c | load | 4 | `abs 0x51af0` | `-` |
| `ldr` at 0x51a3c | load | 4 | `abs 0x51ae8` | `-` |
| `strb` at 0x51a44 | store | 1 | `abs 0x148fd4` | `0x1` |
| `str` at 0x51a54 | store | 4 | `sp -0x14` | `ret(0x52358)` |
| `ldr` at 0x51a58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51a64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51a70 | load | 4 | `abs 0x51af4` | `-` |
| `str` at 0x51a74 | store | 4 | `sp -0x1c` | `0x10bad0` |
| `str` at 0x51a78 | store | 4 | `sp -0x18` | `ret(0x21a38)` |
| `ldr` at 0x51a80 | load | 4 | `abs 0x51af0` | `-` |
| `ldr` at 0x51a84 | load | 4 | `abs 0x51af8` | `-` |
| `ldr` at 0x51a94 | load | 4 | `abs 0x51afc` | `-` |
| `str` at 0x51aa4 | store | 4 | `sp -0x14` | `ret(0x51b28)` |
| `ldr` at 0x51aa8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51ab4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51ac0 | load | 4 | `abs 0x51b00` | `-` |
| `str` at 0x51ac4 | store | 4 | `sp -0x1c` | `0x10baf4` |
| `str` at 0x51ac8 | store | 4 | `sp -0x18` | `ret(0x21a38)` |
| `ldr` at 0x51ad0 | load | 4 | `abs 0x51af0` | `-` |
| `ldr` at 0x51ad4 | load | 4 | `abs 0x51b04` | `-` |
| `ldmdb` at 0x51ae0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x51ae0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x51ae0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x51a38 | `(unknown callee) 0x21eb8` | `0x9` | `0x10ba58` | `0xeb` | `0x0` |
| `bl` at 0x51a4c | `(unknown callee) 0x52358` | `0x1` | `?` | `0x148fd4` | `0x1` |
| `bl` at 0x51a68 | `(unknown callee) 0x21a38` | `ret(0x52358)` | `?` | `?` | `ret(0x52358)` |
| `bl` at 0x51a8c | `(unknown callee) 0x21eb8` | `0x3` | `0x10ba58` | `0x106` | `0x0` |
| `bl` at 0x51a9c | `(unknown callee) 0x51b28` | `0x32` | `0x10bae8` | `0x0` | `?` |
| `bl` at 0x51ab8 | `(unknown callee) 0x21a38` | `ret(0x51b28)` | `?` | `?` | `ret(0x51b28)` |
| `bl` at 0x51adc | `(unknown callee) 0x21eb8` | `0x3` | `0x10ba58` | `0x116` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x51a08 | `[0x1434ac]:1 != 0xff` | `[0x1434ac]:1 == 0xff` |
| `beq` at 0x51a1c | `[0x148fd4]:1 == 0x0` | `[0x148fd4]:1 != 0x0` |
| `beq` at 0x51a60 | `ret(0x52358) == 0x0` | `ret(0x52358) != 0x0` |
| `beq` at 0x51ab0 | `ret(0x51b28) == 0x0` | `ret(0x51b28) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x51ae0 | `r0` | `?` |

## `displayPresent`

- `displayPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x51b08 to 0x51b28.
  Established. The absolute byte at `0x148fd5` is read (`ldrb` at 0x51b18). The return reports
  `[0x148fd5]:1` (`ldmdb` at 0x51b20). Presence-state encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x51b14 | load | 4 | `abs 0x51b24` | `-` |
| `ldrb` at 0x51b18 | load | 1 | `abs 0x148fd5` | `-` |
| `ldmdb` at 0x51b20 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x51b20 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x51b20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x51b20 | `r0` | `[0x148fd5]:1` |

## `displayString`

- `displayString`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x51b28 to 0x51da4.
  Established. The branch takes `r0 != 0x31` to `0x51b84`, and falls through for `0x31` (`bne`
  at 0x51b68). Included byte stack assignments at `sp -0x35` write `0x20` (`strb` at 0x51c68) and
  `0x0` (`strb` at 0x51c74). The first indirect call has an unknown target and candidates
  `[0x149030]`, `[sp-32]`, `[sp-36]`, `[0x149040]` (`mov` at 0x51d38). The later indirect call
  carries `[sp-20]`, `[sp-36]`, unknown, `[0x149038]` (`mov` at 0x51d60). String and hardware
  effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x51b38 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x51b3c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x51b40 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x51b44 | load | 4 | `abs 0x51d84` | `-` |
| `ldrb` at 0x51b48 | load | 1 | `abs 0x148fd4` | `-` |
| `ldr` at 0x51b54 | load | 4 | `abs 0x51d88` | `-` |
| `str` at 0x51b58 | store | 4 | `sp -0x30` | `0xfffff8f7` |
| `ldr` at 0x51b60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51b6c | load | 4 | `abs 0x51d8c` | `-` |
| `str` at 0x51b70 | store | 4 | `sp -0x20` | `0x149048` |
| `ldr` at 0x51b74 | load | 4 | `abs 0x51d90` | `-` |
| `ldrb` at 0x51b78 | load | 1 | `abs 0x14902a` | `-` |
| `str` at 0x51b7c | store | 4 | `sp -0x24` | `[0x14902a]:1` |
| `ldr` at 0x51b84 | load | 4 | `abs 0x51d94` | `-` |
| `ldrb` at 0x51b88 | load | 1 | `abs 0x148fd6` | `-` |
| `ldr` at 0x51b94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x51ba0 | load | 4 | `abs 0x51d98` | `-` |
| `ldr` at 0x51ba4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x51bb0 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x51bb8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x51bc8 | load | 4 | `abs 0x51d9c` | `-` |
| `str` at 0x51bcc | store | 4 | `sp -0x20` | `0x149059` |
| `ldr` at 0x51bd0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x51be0 | load | 4 | `abs 0x51d90` | `-` |
| `ldrb` at 0x51be4 | load | 1 | `abs 0x14902a` | `-` |
| `str` at 0x51bf4 | store | 4 | `sp -0x24` | `0x48` |
| `ldr` at 0x51bfc | load | 4 | `abs 0x51d90` | `-` |
| `ldrb` at 0x51c00 | load | 1 | `abs 0x14902a` | `-` |
| `str` at 0x51c04 | store | 4 | `sp -0x24` | `[0x14902a]:1` |
| `ldr` at 0x51c08 | load | 4 | `abs 0x51d98` | `-` |
| `ldr` at 0x51c0c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x51c18 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x51c1c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x51c20 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x51c30 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x51c34 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x51c40 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x51c44 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x51c4c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x51c50 | load | 4 | `abs 0x51d90` | `-` |
| `ldrb` at 0x51c54 | load | 1 | `abs 0x14902a` | `-` |
| `ldr` at 0x51c58 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x51c68 | store | 1 | `sp -0x35` | `0x20` |
| `strb` at 0x51c74 | store | 1 | `sp -0x35` | `0x0` |
| `ldrb` at 0x51c78 | load | 1 | `sp -0x35` | `-` |
| `ldr` at 0x51c7c | load | 4 | `sp -0x34` | `-` |
| `strb` at 0x51c80 | store | 1 | `(unknown)` | `([sp-53]:1 & 0xff)` |
| `ldr` at 0x51c88 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x51c8c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x51c94 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x51ca4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x51ca8 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x51cb0 | load | 4 | `ret(0x1b55c) +0x0` | `-` |
| `ldrh` at 0x51cb8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x51cc8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x51ccc | load | 1 | `[sp-24] +0x0` | `-` |
| `strb` at 0x51cd0 | store | 1 | `sp -0x3d` | `([[sp-24]]:1 & 0xff)` |
| `strb` at 0x51cdc | store | 1 | `sp -0x3d` | `0x3f` |
| `ldrb` at 0x51ce0 | load | 1 | `sp -0x3d` | `-` |
| `ldr` at 0x51ce4 | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0x51ce8 | store | 1 | `[sp-60] +0x0` | `?` |
| `ldr` at 0x51cec | load | 4 | `sp -0x18` | `-` |
| `str` at 0x51cf4 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x51cf8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x51d00 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x51d08 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x51d0c | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x51d18 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x51d1c | load | 4 | `abs 0x51d90` | `-` |
| `ldr` at 0x51d20 | load | 4 | `abs 0x51d90` | `-` |
| `ldr` at 0x51d24 | load | 4 | `abs 0x149040` | `-` |
| `ldr` at 0x51d28 | load | 4 | `abs 0x149030` | `-` |
| `ldr` at 0x51d2c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x51d30 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x51d3c | load | 4 | `abs 0x51da0` | `-` |
| `ldrb` at 0x51d40 | load | 1 | `abs 0x1490a2` | `-` |
| `ldr` at 0x51d4c | load | 4 | `abs 0x51d90` | `-` |
| `ldr` at 0x51d50 | load | 4 | `abs 0x149038` | `-` |
| `ldr` at 0x51d54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51d58 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x51d64 | store | 4 | `sp -0x44` | `ret(unknown)` |
| `str` at 0x51d70 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x51d74 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x51d78 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x51d7c | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0x51d80 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x51d80 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x51d80 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x51ba8 | `(unknown callee) 0x1b598` | `0x1434b0` | `r1` | `r2` | `r2` |
| `bl` at 0x51bc4 | `(unknown callee) 0x51ff4` | `r0` | `r1` | `r2` | `r2` |
| `bl` at 0x51bd4 | `(unknown callee) 0x1b2bc` | `[sp-24]` | `?` | `?` | `0x149059` |
| `bl` at 0x51c10 | `(unknown callee) 0x1b598` | `0x1434b0` | `[sp-24]` | `ret(0x1b2bc)` | `?` |
| `bl` at 0x51c98 | `(unknown callee) 0x1b55c` | `?` | `?` | `?` | `?` |
| `mov` at 0x51d38 | `(unknown callee) (unknown target)` | `[0x149030]` | `[sp-32]` | `[sp-36]` | `[0x149040]` |
| `mov` at 0x51d60 | `(unknown callee) (unknown target)` | `[sp-20]` | `[sp-36]` | `?` | `[0x149038]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x51b50 | `[0x148fd4]:1 != 0x0` | `[0x148fd4]:1 == 0x0` |
| `bne` at 0x51b68 | `r0 != 0x31` | `r0 == 0x31` |
| `beq` at 0x51b90 | `[0x148fd6]:1 == 0x0` | `[0x148fd6]:1 != 0x0` |
| `bne` at 0x51b9c | `r2 != 0x0` | `r2 == 0x0` |
| `bne` at 0x51bc0 | `r2 != 0x2` | `r2 == 0x2` |
| `bls` at 0x51bec | `ret(0x1b2bc) <= [0x14902a]:1 (unsigned, 32-bit)` | `ret(0x1b2bc) > [0x14902a]:1 (unsigned, 32-bit)` |
| `blt` at 0x51c28 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x51c3c | `Z == 0` | `not (Z == 0)` |
| `bgt` at 0x51c60 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x51cc4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x51d48 | `[0x1490a2]:1 != 0x0` | `[0x1490a2]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x51d80 | `r0` | `?` |

## `displayChar`

- `displayChar`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x51da4 to 0x51f10.
  Established. Included byte assignments write `([sp-22]:1 & 0xff)` at `[sp-21]:1 +0x149048` (`strb`
  at 0x51e6c) and `[sp-21]:1 +0x149059` (`strb` at 0x51e80). The branch takes `[sp-20] != 0x31`
  to the latter write and falls through for `0x31` (`bne` at 0x51e5c). The first indirect call
  has an unknown target and candidates `sp-23`, `sp-22`, `0x1`, `[0x149040]` (`mov` at 0x51ea0).
  Character conversion and device effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x51db4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x51dbc | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x51dc4 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0x51dc8 | load | 4 | `abs 0x51ef8` | `-` |
| `ldrb` at 0x51dcc | load | 1 | `abs 0x148fd4` | `-` |
| `ldr` at 0x51dd8 | load | 4 | `abs 0x51efc` | `-` |
| `str` at 0x51ddc | store | 4 | `sp -0x1c` | `0xfffff8f7` |
| `ldr` at 0x51de4 | load | 4 | `abs 0x51f00` | `-` |
| `ldr` at 0x51de8 | load | 4 | `abs 0x14903c` | `-` |
| `str` at 0x51df8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x51e00 | load | 4 | `abs 0x51f00` | `-` |
| `ldrb` at 0x51e04 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x51e08 | load | 1 | `abs 0x14902a` | `-` |
| `str` at 0x51e18 | store | 4 | `sp -0x1c` | `0x0` |
| `ldrb` at 0x51e2c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x51e34 | load | 4 | `ret(0x1b55c) +0x0` | `-` |
| `ldrh` at 0x51e3c | load | 2 | `(unknown)` | `-` |
| `strb` at 0x51e50 | store | 1 | `sp -0x16` | `0x3f` |
| `ldr` at 0x51e54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51e60 | load | 4 | `abs 0x51f04` | `-` |
| `ldrb` at 0x51e64 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x51e68 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x51e6c | store | 1 | `[sp-21]:1 +0x149048` | `([sp-22]:1 & 0xff)` |
| `ldr` at 0x51e74 | load | 4 | `abs 0x51f08` | `-` |
| `ldrb` at 0x51e78 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x51e7c | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x51e80 | store | 1 | `[sp-21]:1 +0x149059` | `([sp-22]:1 & 0xff)` |
| `ldr` at 0x51e84 | load | 4 | `abs 0x51f00` | `-` |
| `ldr` at 0x51e90 | load | 4 | `abs 0x149040` | `-` |
| `ldr` at 0x51ea4 | load | 4 | `abs 0x51f0c` | `-` |
| `ldrb` at 0x51ea8 | load | 1 | `abs 0x1490a2` | `-` |
| `ldr` at 0x51eb4 | load | 4 | `abs 0x51f00` | `-` |
| `ldrb` at 0x51eb8 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x51ebc | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x51ec0 | load | 4 | `abs 0x14903c` | `-` |
| `ldr` at 0x51ec4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x51ed8 | store | 4 | `sp -0x20` | `ret(unknown)` |
| `str` at 0x51ee4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x51ee8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x51eec | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x51ef0 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x51ef4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x51ef4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x51ef4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x51e20 | `(unknown callee) 0x1b55c` | `r0` | `r1` | `(r1 & 0xff)` | `[0x14902a]:1` |
| `mov` at 0x51ea0 | `(unknown callee) (unknown target)` | `sp-23` | `sp-22` | `0x1` | `[0x149040]` |
| `mov` at 0x51ed4 | `(unknown callee) (unknown target)` | `[sp-20]` | `[sp-21]:1` | `[sp-23]:1` | `[0x14903c]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x51dd4 | `[0x148fd4]:1 != 0x0` | `[0x148fd4]:1 == 0x0` |
| `bne` at 0x51df0 | `[0x14903c] != 0x0` | `[0x14903c] == 0x0` |
| `blo` at 0x51e10 | `(r1 & 0xff) < [0x14902a]:1 (unsigned, 32-bit)` | `(r1 & 0xff) >= [0x14902a]:1 (unsigned, 32-bit)` |
| `bne` at 0x51e48 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x51e5c | `[sp-20] != 0x31` | `[sp-20] == 0x31` |
| `bne` at 0x51eb0 | `[0x1490a2]:1 != 0x0` | `[0x1490a2]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x51ef4 | `r0` | `?` |

## `displayHeatbeat`

- `displayHeatbeat`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x51f10 to 0x51ff4.
  Established. The first branch takes `[0x14903c] == 0x0` to `0x51f44`, and falls through when
  nonzero (`beq` at 0x51f2c). The absolute byte at `0x149020` receives `([0x149020]:1+1 & 0xff)`
  (`strb` at 0x51fa4). The indirect call has an unknown target and candidates `0x31`, unknown,
  unknown, unknown (`mov` at 0x51fc8). Heartbeat display behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x51f20 | load | 4 | `abs 0x51fdc` | `-` |
| `ldr` at 0x51f24 | load | 4 | `abs 0x14903c` | `-` |
| `ldr` at 0x51f30 | load | 4 | `abs 0x51fe0` | `-` |
| `ldrb` at 0x51f34 | load | 1 | `abs 0x1490a2` | `-` |
| `str` at 0x51f48 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x51f50 | load | 4 | `abs 0x51fe4` | `-` |
| `ldr` at 0x51f54 | load | 4 | `abs 0x51fdc` | `-` |
| `ldrb` at 0x51f58 | load | 1 | `abs 0x14902a` | `-` |
| `ldr` at 0x51f68 | load | 4 | `abs 0x51fe8` | `-` |
| `ldr` at 0x51f6c | load | 4 | `abs 0x51fec` | `-` |
| `ldrb` at 0x51f70 | load | 1 | `abs 0x149020` | `-` |
| `ldrb` at 0x51f78 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x51f7c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x51f80 | load | 4 | `abs 0x51fdc` | `-` |
| `ldr` at 0x51f84 | load | 4 | `abs 0x51fdc` | `-` |
| `ldrb` at 0x51f88 | load | 1 | `abs 0x14902a` | `-` |
| `ldr` at 0x51f94 | load | 4 | `abs 0x51ff0` | `-` |
| `ldr` at 0x51f98 | load | 4 | `abs 0x51fec` | `-` |
| `ldrb` at 0x51f9c | load | 1 | `abs 0x149020` | `-` |
| `strb` at 0x51fa4 | store | 1 | `abs 0x149020` | `([0x149020]:1+1 & 0xff)` |
| `ldrb` at 0x51fb0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x51fb4 | load | 4 | `abs 0x14903c` | `-` |
| `str` at 0x51fd0 | store | 4 | `sp -0x14` | `ret(unknown)` |
| `ldr` at 0x51fd4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x51fd8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x51fd8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x51fd8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x51fc8 | `(unknown callee) (unknown target)` | `0x31` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x51f2c | `[0x14903c] == 0x0` | `[0x14903c] != 0x0` |
| `bne` at 0x51f3c | `[0x1490a2]:1 != 0x0` | `[0x1490a2]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x51fd8 | `r0` | `?` |

## `displayLock`

- `displayLock`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x51ff4 to 0x52014.
  Established. The absolute byte assignment at `0x148fd6` writes `0x1` (`strb` at 0x52008). The
  return preserves entry `r0` (`ldmdb` at 0x5200c). Lock-state consumers remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x52000 | load | 4 | `abs 0x52010` | `-` |
| `strb` at 0x52008 | store | 1 | `abs 0x148fd6` | `0x1` |
| `ldmdb` at 0x5200c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5200c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5200c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5200c | `r0` | `r0` |

## `displayUnlock`

- `displayUnlock`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52014 to 0x52048.
  Established. The byte at absolute `0x148fd6` is cleared (`strb` at 0x52028). The unnamed `0x51b28`
  call carries `0x32`, `0x1434b0`, `0x0`, `0x148fd6` (`bl` at 0x52038); its result is reported at
  the return (`ldmdb` at 0x5203c). Display effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x52020 | load | 4 | `abs 0x52040` | `-` |
| `strb` at 0x52028 | store | 1 | `abs 0x148fd6` | `0x0` |
| `ldr` at 0x52030 | load | 4 | `abs 0x52044` | `-` |
| `ldmdb` at 0x5203c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5203c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5203c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x52038 | `(unknown callee) 0x51b28` | `0x32` | `0x1434b0` | `0x0` | `0x148fd6` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5203c | `r0` | `ret(0x51b28)` |

## `displayLedOn`

- `displayLedOn`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52048 to 0x52154.
  Established. The first branch takes `[0x148fd4]:1 != 0x0` to `0x52078`, while zero falls
  through (`bne` at 0x52068). Included unnamed `0xd7368` calls carry `0x0`, `[0x1434ac]:1`, `0x40`,
  `[0x1434ac]:1` (`bl` at 0x520bc) and `0x0`, `[0x1434ac]:1`, `0x10`, `[0x1434ac]:1` (`bl` at
  0x520e0). LED control effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x52058 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x5205c | load | 4 | `abs 0x52140` | `-` |
| `ldrb` at 0x52060 | load | 1 | `abs 0x148fd4` | `-` |
| `ldr` at 0x5206c | load | 4 | `abs 0x52144` | `-` |
| `str` at 0x52070 | store | 4 | `sp -0x18` | `0xfffff8f7` |
| `ldr` at 0x52078 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5207c | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0x52080 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5208c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52098 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x520a8 | load | 4 | `abs 0x52148` | `-` |
| `ldrb` at 0x520ac | load | 1 | `abs 0x1434ac` | `-` |
| `str` at 0x520c4 | store | 4 | `sp -0x18` | `ret(0xd7368)` |
| `ldr` at 0x520cc | load | 4 | `abs 0x52148` | `-` |
| `ldrb` at 0x520d0 | load | 1 | `abs 0x1434ac` | `-` |
| `str` at 0x520e8 | store | 4 | `sp -0x18` | `ret(0xd7368)` |
| `ldr` at 0x520f0 | load | 4 | `abs 0x52148` | `-` |
| `ldrb` at 0x520f4 | load | 1 | `abs 0x1434ac` | `-` |
| `str` at 0x5210c | store | 4 | `sp -0x18` | `ret(0xd7368)` |
| `ldr` at 0x52114 | load | 4 | `abs 0x5214c` | `-` |
| `str` at 0x52118 | store | 4 | `sp -0x24` | `0x10bb0c` |
| `ldr` at 0x5211c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x52120 | store | 4 | `sp -0x20` | `r0` |
| `ldr` at 0x52128 | load | 4 | `abs 0x52150` | `-` |
| `ldr` at 0x52138 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5213c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5213c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5213c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x520bc | `(unknown callee) 0xd7368` | `0x0` | `[0x1434ac]:1` | `0x40` | `[0x1434ac]:1` |
| `bl` at 0x520e0 | `(unknown callee) 0xd7368` | `0x0` | `[0x1434ac]:1` | `0x10` | `[0x1434ac]:1` |
| `bl` at 0x52104 | `(unknown callee) 0xd7368` | `0x0` | `[0x1434ac]:1` | `0x20` | `[0x1434ac]:1` |
| `bl` at 0x52134 | `(unknown callee) 0x21eb8` | `0x9` | `0x10ba58` | `0x218` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x52068 | `[0x148fd4]:1 != 0x0` | `[0x148fd4]:1 == 0x0` |
| `beq` at 0x52088 | `r0 == 0x1` | `r0 != 0x1` |
| `blo` at 0x52094 | `r0 < 0x1 (unsigned, 32-bit)` | `r0 >= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x520a0 | `r0 == 0x2` | `r0 != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5213c | `r0` | `?` |

## `displayLedOff`

- `displayLedOff`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52154 to 0x52264.
  Established. The unnamed `0xd731c` call at the first selector arm carries `0x0`, `[0x1434ac]:1`,
  `0x40`, `[0x1434ac]:1` (`bl` at 0x521c8). A separate call to that numeric target carries `0x0`,
  `[0x1434ac]:1`, `0x20`, `[0x1434ac]:1` (`bl` at 0x52210). The word assignment at `sp -0x18`
  retains `ret(0xd731c)` (`str` at 0x52218). Hardware clearing behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x52164 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x52168 | load | 4 | `abs 0x5224c` | `-` |
| `ldrb` at 0x5216c | load | 1 | `abs 0x148fd4` | `-` |
| `ldr` at 0x52178 | load | 4 | `abs 0x52250` | `-` |
| `str` at 0x5217c | store | 4 | `sp -0x18` | `0xfffff8f7` |
| `ldr` at 0x52184 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x52188 | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0x5218c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52198 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x521a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x521b4 | load | 4 | `abs 0x52254` | `-` |
| `ldrb` at 0x521b8 | load | 1 | `abs 0x1434ac` | `-` |
| `str` at 0x521d0 | store | 4 | `sp -0x18` | `ret(0xd731c)` |
| `ldr` at 0x521d8 | load | 4 | `abs 0x52254` | `-` |
| `ldrb` at 0x521dc | load | 1 | `abs 0x1434ac` | `-` |
| `str` at 0x521f4 | store | 4 | `sp -0x18` | `ret(0xd731c)` |
| `ldr` at 0x521fc | load | 4 | `abs 0x52254` | `-` |
| `ldrb` at 0x52200 | load | 1 | `abs 0x1434ac` | `-` |
| `str` at 0x52218 | store | 4 | `sp -0x18` | `ret(0xd731c)` |
| `ldr` at 0x52220 | load | 4 | `abs 0x52258` | `-` |
| `str` at 0x52224 | store | 4 | `sp -0x24` | `0x10bb0c` |
| `ldr` at 0x52228 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5222c | store | 4 | `sp -0x20` | `r0` |
| `ldr` at 0x52234 | load | 4 | `abs 0x5225c` | `-` |
| `ldr` at 0x52238 | load | 4 | `abs 0x52260` | `-` |
| `ldr` at 0x52244 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x52248 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x52248 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x52248 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x521c8 | `(unknown callee) 0xd731c` | `0x0` | `[0x1434ac]:1` | `0x40` | `[0x1434ac]:1` |
| `bl` at 0x521ec | `(unknown callee) 0xd731c` | `0x0` | `[0x1434ac]:1` | `0x10` | `[0x1434ac]:1` |
| `bl` at 0x52210 | `(unknown callee) 0xd731c` | `0x0` | `[0x1434ac]:1` | `0x20` | `[0x1434ac]:1` |
| `bl` at 0x52240 | `(unknown callee) 0x21eb8` | `0x9` | `0x10ba58` | `0x235` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x52174 | `[0x148fd4]:1 != 0x0` | `[0x148fd4]:1 == 0x0` |
| `beq` at 0x52194 | `r0 == 0x1` | `r0 != 0x1` |
| `blo` at 0x521a0 | `r0 < 0x1 (unsigned, 32-bit)` | `r0 >= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x521ac | `r0 == 0x2` | `r0 != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x52248 | `r0` | `?` |

## `displayLedStatus`

- `displayLedStatus`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52264 to 0x52358.
  Established. The first branch takes `[0x1434ac]:1 != 0xff` to `0x52294`, and falls through for
  `0xff` (`bne` at 0x52284). Included byte stack assignments at `sp -0x15` write `0x40` (`strb` at
  0x522c8), `0x10` (`strb` at 0x522d4), and `0x20` (`strb` at 0x522e0). The unnamed `0xd75c0` call
  carries `0x0`, `[0x1434ac]:1`, unknown, `[0x1434ac]:1` (`bl` at 0x5231c). LED status encoding
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x52274 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x52278 | load | 4 | `abs 0x52348` | `-` |
| `ldrb` at 0x5227c | load | 1 | `abs 0x1434ac` | `-` |
| `ldr` at 0x52288 | load | 4 | `abs 0x5234c` | `-` |
| `str` at 0x5228c | store | 4 | `sp -0x1c` | `0xfffff8f8` |
| `ldr` at 0x52294 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x52298 | store | 4 | `sp -0x20` | `r0` |
| `ldr` at 0x5229c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x522a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x522b4 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x522c8 | store | 1 | `sp -0x15` | `0x40` |
| `strb` at 0x522d4 | store | 1 | `sp -0x15` | `0x10` |
| `strb` at 0x522e0 | store | 1 | `sp -0x15` | `0x20` |
| `ldr` at 0x522e8 | load | 4 | `abs 0x52350` | `-` |
| `str` at 0x522ec | store | 4 | `sp -0x28` | `0x10bb0c` |
| `ldr` at 0x522f0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x522f4 | store | 4 | `sp -0x24` | `r0` |
| `ldr` at 0x522fc | load | 4 | `abs 0x52354` | `-` |
| `ldr` at 0x5230c | load | 4 | `abs 0x52348` | `-` |
| `ldrb` at 0x52310 | load | 1 | `abs 0x1434ac` | `-` |
| `ldrb` at 0x52324 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x5233c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x52340 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x52344 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x52344 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x52344 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x52308 | `(unknown callee) 0x21eb8` | `0x9` | `0x10ba58` | `0x258` | `0x0` |
| `bl` at 0x5231c | `(unknown callee) 0xd75c0` | `0x0` | `[0x1434ac]:1` | `?` | `[0x1434ac]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x52284 | `[0x1434ac]:1 != 0xff` | `[0x1434ac]:1 == 0xff` |
| `beq` at 0x522a4 | `r0 == 0x1` | `r0 != 0x1` |
| `blo` at 0x522b0 | `r0 < 0x1 (unsigned, 32-bit)` | `r0 >= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x522bc | `r0 == 0x2` | `r0 != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x52344 | `r0` | `?` |

## `displayStatus`

- `displayStatus`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52564 to 0x52660.
  Established. Included absolute byte assignments write `0x2e` at `0x149022` (`strb` at 0x525b8) and
  `0x0` at `0x149024` (`strb` at 0x525dc). Four-byte assignments retain `0x149048` at `[sp-20] +0x0`
  (`str` at 0x525e8), `0x149059` at `[sp-20] +0x4` (`str` at 0x525f4), and `0x149021` at `[sp-20] +0x8` (`str` at 0x52600). The unnamed `0xd7530` call carries `0x0`, `[0x1434ac]:1`, `[sp-20]+16`,
  `[sp-20]+16` (`bl` at 0x52630). Record interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x52574 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x52578 | load | 4 | `abs 0x52644` | `-` |
| `ldrb` at 0x5257c | load | 1 | `abs 0x148fd4` | `-` |
| `ldr` at 0x52588 | load | 4 | `abs 0x52648` | `-` |
| `str` at 0x5258c | store | 4 | `sp -0x18` | `0xfffff8f7` |
| `ldr` at 0x52594 | load | 4 | `abs 0x5264c` | `-` |
| `ldr` at 0x52598 | load | 4 | `abs 0x52650` | `-` |
| `ldrb` at 0x5259c | load | 1 | `abs 0x14902b` | `-` |
| `strb` at 0x525ac | store | 1 | `abs 0x149021` | `?` |
| `ldr` at 0x525b0 | load | 4 | `abs 0x5264c` | `-` |
| `strb` at 0x525b8 | store | 1 | `abs 0x149022` | `0x2e` |
| `ldr` at 0x525bc | load | 4 | `abs 0x5264c` | `-` |
| `ldr` at 0x525c0 | load | 4 | `abs 0x52650` | `-` |
| `ldrb` at 0x525c4 | load | 1 | `abs 0x14902b` | `-` |
| `strb` at 0x525d0 | store | 1 | `abs 0x149023` | `?` |
| `ldr` at 0x525d4 | load | 4 | `abs 0x5264c` | `-` |
| `strb` at 0x525dc | store | 1 | `abs 0x149024` | `0x0` |
| `ldr` at 0x525e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x525e4 | load | 4 | `abs 0x52654` | `-` |
| `str` at 0x525e8 | store | 4 | `[sp-20] +0x0` | `0x149048` |
| `ldr` at 0x525ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x525f0 | load | 4 | `abs 0x52658` | `-` |
| `str` at 0x525f4 | store | 4 | `[sp-20] +0x4` | `0x149059` |
| `ldr` at 0x525f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x525fc | load | 4 | `abs 0x5264c` | `-` |
| `str` at 0x52600 | store | 4 | `[sp-20] +0x8` | `0x149021` |
| `ldr` at 0x52604 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x52608 | load | 4 | `abs 0x52650` | `-` |
| `ldrb` at 0x5260c | load | 1 | `abs 0x14902b` | `-` |
| `strb` at 0x52610 | store | 1 | `[sp-20] +0xc` | `([0x14902b]:1 & 0xff)` |
| `ldr` at 0x52614 | load | 4 | `abs 0x5265c` | `-` |
| `ldrb` at 0x52618 | load | 1 | `abs 0x1434ac` | `-` |
| `ldr` at 0x5261c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x52638 | store | 4 | `sp -0x18` | `ret(0xd7530)` |
| `ldr` at 0x5263c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x52640 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x52640 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x52640 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x52630 | `(unknown callee) 0xd7530` | `0x0` | `[0x1434ac]:1` | `[sp-20]+16` | `[sp-20]+16` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x52584 | `[0x148fd4]:1 != 0x0` | `[0x148fd4]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x52640 | `r0` | `?` |

## `displayInt`

- `displayInt`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52660 to 0x52c90.
  Established. The first branch takes `(r1 & 0xff) == [0x1434ac]:1` to `0x526c8`, and falls
  through when unequal (`beq` at 0x526a0). The unnamed `0x4e918` call carries `0x2`, `0xff`,
  unknown, unknown (`bl` at 0x52a30); its zero result takes `0x52a74`, while nonzero falls through
  (`beq` at 0x52a44). The later unnamed `0x4e54c` call carries `0x2` and three unknown candidates
  (`bl` at 0x52b7c). Interrupt and power effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x52670 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x5267c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x52684 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x5268c | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldr` at 0x52690 | load | 4 | `abs 0x52c14` | `-` |
| `ldrb` at 0x52694 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x52698 | load | 1 | `abs 0x1434ac` | `-` |
| `ldr` at 0x526a4 | load | 4 | `abs 0x52c18` | `-` |
| `str` at 0x526a8 | store | 4 | `sp -0x24` | `0x10bb3c` |
| `ldrb` at 0x526ac | load | 1 | `sp -0x15` | `-` |
| `str` at 0x526b0 | store | 4 | `sp -0x20` | `(r1 & 0xff)` |
| `ldr` at 0x526b8 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x526bc | load | 4 | `abs 0x52c20` | `-` |
| `ldr` at 0x526c8 | load | 4 | `abs 0x52c24` | `-` |
| `ldrb` at 0x526cc | load | 1 | `abs 0x14902c` | `-` |
| `ldrb` at 0x526dc | load | 1 | `sp -0x17` | `-` |
| `strb` at 0x526e4 | store | 1 | `sp -0x17` | `?` |
| `ldrb` at 0x526e8 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x526f8 | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x5272c | load | 4 | `abs 0x52c28` | `-` |
| `str` at 0x52730 | store | 4 | `sp -0x24` | `0x10bb64` |
| `ldr` at 0x52738 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x5273c | load | 4 | `abs 0x52c2c` | `-` |
| `ldr` at 0x5274c | load | 4 | `abs 0x52c30` | `-` |
| `str` at 0x52768 | store | 4 | `sp -0x1c` | `ret(0x3d890)` |
| `ldr` at 0x5276c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52778 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52784 | load | 4 | `abs 0x52c34` | `-` |
| `str` at 0x52788 | store | 4 | `sp -0x24` | `0x10bbb4` |
| `str` at 0x5278c | store | 4 | `sp -0x20` | `ret(0x21a38)` |
| `ldr` at 0x52794 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x52798 | load | 4 | `abs 0x52c38` | `-` |
| `str` at 0x527b8 | store | 4 | `sp -0x1c` | `ret(0x3d008)` |
| `ldr` at 0x527bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x527c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x527d4 | load | 4 | `abs 0x52c3c` | `-` |
| `str` at 0x527d8 | store | 4 | `sp -0x24` | `0x10bbcc` |
| `str` at 0x527dc | store | 4 | `sp -0x20` | `ret(0x21a38)` |
| `ldr` at 0x527e4 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x527e8 | load | 4 | `abs 0x52c40` | `-` |
| `ldrb` at 0x527f4 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x52804 | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x529a8 | load | 4 | `abs 0x52c44` | `-` |
| `str` at 0x529ac | store | 4 | `sp -0x24` | `0x10bbe0` |
| `ldr` at 0x529b4 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x529b8 | load | 4 | `abs 0x52c48` | `-` |
| `ldr` at 0x529c8 | load | 4 | `abs 0x52c4c` | `-` |
| `ldr` at 0x529dc | load | 4 | `abs 0x52c50` | `-` |
| `str` at 0x529ec | store | 4 | `sp -0x1c` | `ret(0x51b28)` |
| `ldr` at 0x529f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x529fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52a08 | load | 4 | `abs 0x52c54` | `-` |
| `str` at 0x52a0c | store | 4 | `sp -0x24` | `0x10baf4` |
| `str` at 0x52a10 | store | 4 | `sp -0x20` | `ret(0x21a38)` |
| `ldr` at 0x52a18 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x52a1c | load | 4 | `abs 0x52c58` | `-` |
| `str` at 0x52a38 | store | 4 | `sp -0x1c` | `ret(0x4e918)` |
| `ldr` at 0x52a3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52a48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52a54 | load | 4 | `abs 0x52c5c` | `-` |
| `str` at 0x52a58 | store | 4 | `sp -0x24` | `0x10bc30` |
| `str` at 0x52a5c | store | 4 | `sp -0x20` | `ret(0x21a38)` |
| `ldr` at 0x52a64 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x52a68 | load | 4 | `abs 0x52c60` | `-` |
| `ldr` at 0x52a78 | load | 4 | `abs 0x52c64` | `-` |
| `str` at 0x52a88 | store | 4 | `sp -0x1c` | `ret(0x51b28)` |
| `ldr` at 0x52a8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52a98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52aa4 | load | 4 | `abs 0x52c54` | `-` |
| `str` at 0x52aa8 | store | 4 | `sp -0x24` | `0x10baf4` |
| `str` at 0x52aac | store | 4 | `sp -0x20` | `ret(0x21a38)` |
| `ldr` at 0x52ab4 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x52ab8 | load | 4 | `abs 0x52c68` | `-` |
| `ldrb` at 0x52ac4 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x52ad4 | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x52af8 | load | 4 | `abs 0x52c6c` | `-` |
| `str` at 0x52afc | store | 4 | `sp -0x24` | `0x10bc4c` |
| `ldr` at 0x52b04 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x52b08 | load | 4 | `abs 0x52c70` | `-` |
| `ldr` at 0x52b18 | load | 4 | `abs 0x52c74` | `-` |
| `ldr` at 0x52b2c | load | 4 | `abs 0x52c78` | `-` |
| `str` at 0x52b3c | store | 4 | `sp -0x1c` | `ret(0x51b28)` |
| `ldr` at 0x52b40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52b4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52b58 | load | 4 | `abs 0x52c54` | `-` |
| `str` at 0x52b5c | store | 4 | `sp -0x24` | `0x10baf4` |
| `str` at 0x52b60 | store | 4 | `sp -0x20` | `ret(0x21a38)` |
| `ldr` at 0x52b68 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x52b6c | load | 4 | `abs 0x52c7c` | `-` |
| `str` at 0x52b84 | store | 4 | `sp -0x1c` | `ret(0x4e54c)` |
| `ldr` at 0x52b88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52b94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52ba0 | load | 4 | `abs 0x52c80` | `-` |
| `str` at 0x52ba4 | store | 4 | `sp -0x24` | `0x10bca4` |
| `str` at 0x52ba8 | store | 4 | `sp -0x20` | `ret(0x21a38)` |
| `ldr` at 0x52bb0 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x52bb4 | load | 4 | `abs 0x52c84` | `-` |
| `ldr` at 0x52bc4 | load | 4 | `abs 0x52c88` | `-` |
| `str` at 0x52bd4 | store | 4 | `sp -0x1c` | `ret(0x51b28)` |
| `ldr` at 0x52bd8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52be4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52bf0 | load | 4 | `abs 0x52c54` | `-` |
| `str` at 0x52bf4 | store | 4 | `sp -0x24` | `0x10baf4` |
| `str` at 0x52bf8 | store | 4 | `sp -0x20` | `ret(0x21a38)` |
| `ldr` at 0x52c00 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x52c04 | load | 4 | `abs 0x52c8c` | `-` |
| `ldmdb` at 0x52c10 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x52c10 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x52c10 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x526c4 | `(unknown callee) 0x21eb8` | `0x9` | `0x10ba58` | `0x2de` | `0x0` |
| `bl` at 0x52708 | `(unknown callee) 0x3dce4` | `?` | `?` | `?` | `?` |
| `bl` at 0x5271c | `(unknown callee) 0x1f26c` | `0x77` | `?` | `?` | `ret(0x3dce4)` |
| `bl` at 0x52744 | `(unknown callee) 0x21eb8` | `0x6` | `0x10ba58` | `0x2fe` | `0x0` |
| `bl` at 0x52754 | `(unknown callee) 0x51b28` | `0x32` | `0x10bba0` | `0x1` | `?` |
| `bl` at 0x52760 | `(unknown callee) 0x3d890` | `0x2` | `?` | `?` | `ret(0x1f26c)` |
| `bl` at 0x5277c | `(unknown callee) 0x21a38` | `ret(0x3d890)` | `?` | `?` | `ret(0x3d890)` |
| `bl` at 0x527a0 | `(unknown callee) 0x21eb8` | `0x3` | `0x10ba58` | `0x303` | `0x0` |
| `bl` at 0x527b0 | `(unknown callee) 0x3d008` | `0x1` | `0x2` | `?` | `ret(0x3dce4)` |
| `bl` at 0x527cc | `(unknown callee) 0x21a38` | `ret(0x3d008)` | `?` | `?` | `ret(0x3d008)` |
| `bl` at 0x527f0 | `(unknown callee) 0x21eb8` | `0x3` | `0x10ba58` | `0x307` | `0x0` |
| `bl` at 0x52814 | `(unknown callee) 0x4f4d0` | `?` | `?` | `?` | `?` |
| `bl` at 0x52828 | `(unknown callee) 0x4f4d0` | `ret(0x4f4d0)` | `?` | `?` | `?` |
| `bl` at 0x5283c | `(unknown callee) 0x4f4d0` | `ret(0x4f4d0)` | `?` | `?` | `?` |
| `bl` at 0x52850 | `(unknown callee) 0x4f4d0` | `ret(0x4f4d0)` | `?` | `?` | `?` |
| `bl` at 0x52864 | `(unknown callee) 0x4f4d0` | `ret(0x4f4d0)` | `?` | `?` | `?` |
| `bl` at 0x52878 | `(unknown callee) 0x4f4d0` | `ret(0x4f4d0)` | `?` | `?` | `?` |
| `bl` at 0x5288c | `(unknown callee) 0x4f4d0` | `ret(0x4f4d0)` | `?` | `?` | `?` |
| `bl` at 0x528a0 | `(unknown callee) 0x4f4d0` | `ret(0x4f4d0)` | `?` | `?` | `?` |
| `bl` at 0x528b4 | `(unknown callee) 0x4f4d0` | `ret(0x4f4d0)` | `?` | `?` | `?` |
| `bl` at 0x528c8 | `(unknown callee) 0x4f4d0` | `ret(0x4f4d0)` | `?` | `?` | `?` |
| `bl` at 0x528dc | `(unknown callee) 0x4f4d0` | `ret(0x4f4d0)` | `?` | `?` | `?` |
| `bl` at 0x528f0 | `(unknown callee) 0x4f4d0` | `ret(0x4f4d0)` | `?` | `?` | `?` |
| `bl` at 0x52904 | `(unknown callee) 0x4f4d0` | `ret(0x4f4d0)` | `?` | `?` | `?` |
| `bl` at 0x52918 | `(unknown callee) 0x4f4d0` | `ret(0x4f4d0)` | `?` | `?` | `?` |
| `bl` at 0x5292c | `(unknown callee) 0x4f4d0` | `ret(0x4f4d0)` | `?` | `?` | `?` |
| `bl` at 0x52940 | `(unknown callee) 0x4f4d0` | `ret(0x4f4d0)` | `?` | `?` | `?` |
| `bl` at 0x52954 | `(unknown callee) 0x4f4d0` | `ret(0x4f4d0)` | `?` | `?` | `?` |
| `bl` at 0x52968 | `(unknown callee) 0x4f4d0` | `ret(0x4f4d0)` | `?` | `?` | `?` |
| `bl` at 0x5297c | `(unknown callee) 0x4f4d0` | `ret(0x4f4d0)` | `?` | `?` | `?` |
| `bl` at 0x52998 | `(unknown callee) 0x1f26c` | `0x78` | `?` | `?` | `?` |
| `bl` at 0x529c0 | `(unknown callee) 0x21eb8` | `0x6` | `0x10ba58` | `0x313` | `0x0` |
| `bl` at 0x529d0 | `(unknown callee) 0x51b28` | `0x32` | `0x10bc14` | `0x1` | `?` |
| `bl` at 0x529e4 | `(unknown callee) 0x51b28` | `0x32` | `0x10bc28` | `0x1` | `ret(0x1f26c)` |
| `bl` at 0x52a00 | `(unknown callee) 0x21a38` | `ret(0x51b28)` | `?` | `?` | `ret(0x51b28)` |
| `bl` at 0x52a24 | `(unknown callee) 0x21eb8` | `0x3` | `0x10ba58` | `0x317` | `0x0` |
| `bl` at 0x52a30 | `(unknown callee) 0x4e918` | `0x2` | `0xff` | `?` | `?` |
| `bl` at 0x52a4c | `(unknown callee) 0x21a38` | `ret(0x4e918)` | `?` | `?` | `ret(0x4e918)` |
| `bl` at 0x52a70 | `(unknown callee) 0x21eb8` | `0x3` | `0x10ba58` | `0x31b` | `0x0` |
| `bl` at 0x52a80 | `(unknown callee) 0x51b28` | `0x32` | `0x10bc40` | `0x1` | `?` |
| `bl` at 0x52a9c | `(unknown callee) 0x21a38` | `ret(0x51b28)` | `?` | `?` | `ret(0x51b28)` |
| `bl` at 0x52ac0 | `(unknown callee) 0x21eb8` | `0x3` | `0x10ba58` | `0x31e` | `0x0` |
| `bl` at 0x52ae8 | `(unknown callee) 0x1f26c` | `0x79` | `?` | `?` | `?` |
| `bl` at 0x52b10 | `(unknown callee) 0x21eb8` | `0x6` | `0x10ba58` | `0x329` | `0x0` |
| `bl` at 0x52b20 | `(unknown callee) 0x51b28` | `0x32` | `0x10bc84` | `0x1` | `?` |
| `bl` at 0x52b34 | `(unknown callee) 0x51b28` | `0x32` | `0x10bc98` | `0x1` | `ret(0x1f26c)` |
| `bl` at 0x52b50 | `(unknown callee) 0x21a38` | `ret(0x51b28)` | `?` | `?` | `ret(0x51b28)` |
| `bl` at 0x52b74 | `(unknown callee) 0x21eb8` | `0x3` | `0x10ba58` | `0x32e` | `0x0` |
| `bl` at 0x52b7c | `(unknown callee) 0x4e54c` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x52b98 | `(unknown callee) 0x21a38` | `ret(0x4e54c)` | `?` | `?` | `ret(0x4e54c)` |
| `bl` at 0x52bbc | `(unknown callee) 0x21eb8` | `0x3` | `0x10ba58` | `0x332` | `0x0` |
| `bl` at 0x52bcc | `(unknown callee) 0x51b28` | `0x32` | `0x10bcb4` | `0x1` | `?` |
| `bl` at 0x52be8 | `(unknown callee) 0x21a38` | `ret(0x51b28)` | `?` | `?` | `ret(0x51b28)` |
| `bl` at 0x52c0c | `(unknown callee) 0x21eb8` | `0x3` | `0x10ba58` | `0x335` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x526a0 | `(r1 & 0xff) == [0x1434ac]:1` | `(r1 & 0xff) != [0x1434ac]:1` |
| `beq` at 0x526d8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x526f4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x52704 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x52714 | `ret(0x3dce4) == 0x0` | `ret(0x3dce4) != 0x0` |
| `beq` at 0x52728 | `ret(0x1f26c) == 0x0` | `ret(0x1f26c) != 0x0` |
| `beq` at 0x52774 | `ret(0x3d890) == 0x0` | `ret(0x3d890) != 0x0` |
| `beq` at 0x527c4 | `ret(0x3d008) == 0x0` | `ret(0x3d008) != 0x0` |
| `beq` at 0x52800 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x52810 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x52824 | `? == 0x43` | `? != 0x43` |
| `beq` at 0x52838 | `? == 0x33` | `? != 0x33` |
| `beq` at 0x5284c | `? == 0x35` | `? != 0x35` |
| `beq` at 0x52860 | `? == 0x34` | `? != 0x34` |
| `beq` at 0x52874 | `? == 0x41` | `? != 0x41` |
| `beq` at 0x52888 | `? == 0x36` | `? != 0x36` |
| `beq` at 0x5289c | `? == 0x39` | `? != 0x39` |
| `beq` at 0x528b0 | `? == 0x37` | `? != 0x37` |
| `beq` at 0x528c4 | `? == 0x38` | `? != 0x38` |
| `beq` at 0x528d8 | `? == 0x30` | `? != 0x30` |
| `beq` at 0x528ec | `? == 0x51` | `? != 0x51` |
| `beq` at 0x52900 | `? == 0x40` | `? != 0x40` |
| `beq` at 0x52914 | `? == 0x4f` | `? != 0x4f` |
| `beq` at 0x52928 | `? == 0x42` | `? != 0x42` |
| `beq` at 0x5293c | `? == 0x46` | `? != 0x46` |
| `beq` at 0x52950 | `? == 0x48` | `? != 0x48` |
| `beq` at 0x52964 | `? == 0x4c` | `? != 0x4c` |
| `beq` at 0x52978 | `? == 0x4d` | `? != 0x4d` |
| `beq` at 0x5298c | `? == 0x7c` | `? != 0x7c` |
| `beq` at 0x529a4 | `ret(0x1f26c) == 0x0` | `ret(0x1f26c) != 0x0` |
| `beq` at 0x529f8 | `ret(0x51b28) == 0x0` | `ret(0x51b28) != 0x0` |
| `beq` at 0x52a44 | `ret(0x4e918) == 0x0` | `ret(0x4e918) != 0x0` |
| `beq` at 0x52a94 | `ret(0x51b28) == 0x0` | `ret(0x51b28) != 0x0` |
| `beq` at 0x52ad0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x52ae0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x52af4 | `ret(0x1f26c) == 0x0` | `ret(0x1f26c) != 0x0` |
| `beq` at 0x52b48 | `ret(0x51b28) == 0x0` | `ret(0x51b28) != 0x0` |
| `beq` at 0x52b90 | `ret(0x4e54c) == 0x0` | `ret(0x4e54c) != 0x0` |
| `beq` at 0x52be0 | `ret(0x51b28) == 0x0` | `ret(0x51b28) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x52c10 | `r0` | `?` |

## `panelInt`

- `panelInt`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52c90 to 0x52e30.
  Established. The entry byte assignments retain `(r1 & 0xff)` at `sp -0x15` (`strb` at 0x52cac),
  `(r2 & 0xff)` at `sp -0x16` (`strb` at 0x52cb4), and `(r3 & 0xff)` at `sp -0x17` (`strb` at
  0x52cbc). The unnamed `0x1f26c` call carries `0x77`, unknown, unknown, `ret(0x3dce4)` (`bl` at
  0x52d2c); its zero result takes `0x52d6c`, and nonzero falls through (`beq` at 0x52d38). Panel
  event interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x52ca0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x52cac | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x52cb4 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x52cbc | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldr` at 0x52cc0 | load | 4 | `abs 0x52e08` | `-` |
| `ldrb` at 0x52cc4 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x52cc8 | load | 1 | `abs 0x1434ac` | `-` |
| `ldr` at 0x52cd4 | load | 4 | `abs 0x52e0c` | `-` |
| `str` at 0x52cd8 | store | 4 | `sp -0x24` | `0x10bb3c` |
| `ldrb` at 0x52cdc | load | 1 | `sp -0x15` | `-` |
| `str` at 0x52ce0 | store | 4 | `sp -0x20` | `(r1 & 0xff)` |
| `ldr` at 0x52ce8 | load | 4 | `abs 0x52e10` | `-` |
| `ldr` at 0x52cec | load | 4 | `abs 0x52e14` | `-` |
| `ldrb` at 0x52cf8 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x52d08 | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x52d3c | load | 4 | `abs 0x52e18` | `-` |
| `str` at 0x52d40 | store | 4 | `sp -0x24` | `0x10bb64` |
| `ldr` at 0x52d48 | load | 4 | `abs 0x52e10` | `-` |
| `ldr` at 0x52d4c | load | 4 | `abs 0x52e1c` | `-` |
| `ldr` at 0x52d5c | load | 4 | `abs 0x52e20` | `-` |
| `str` at 0x52d78 | store | 4 | `sp -0x1c` | `ret(0x3d890)` |
| `ldr` at 0x52d7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52d88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52d94 | load | 4 | `abs 0x52e24` | `-` |
| `str` at 0x52d98 | store | 4 | `sp -0x24` | `0x10bbb4` |
| `str` at 0x52d9c | store | 4 | `sp -0x20` | `ret(0x21a38)` |
| `ldr` at 0x52da4 | load | 4 | `abs 0x52e10` | `-` |
| `ldr` at 0x52da8 | load | 4 | `abs 0x52e28` | `-` |
| `str` at 0x52dc8 | store | 4 | `sp -0x1c` | `ret(0x3d008)` |
| `ldr` at 0x52dcc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52dd8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52de4 | load | 4 | `abs 0x52e2c` | `-` |
| `str` at 0x52de8 | store | 4 | `sp -0x24` | `0x10bbcc` |
| `str` at 0x52dec | store | 4 | `sp -0x20` | `ret(0x21a38)` |
| `ldr` at 0x52df4 | load | 4 | `abs 0x52e10` | `-` |
| `ldmdb` at 0x52e04 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x52e04 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x52e04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x52cf4 | `(unknown callee) 0x21eb8` | `0x9` | `0x10ba58` | `0x391` | `0x0` |
| `bl` at 0x52d18 | `(unknown callee) 0x3dce4` | `?` | `?` | `?` | `?` |
| `bl` at 0x52d2c | `(unknown callee) 0x1f26c` | `0x77` | `?` | `?` | `ret(0x3dce4)` |
| `bl` at 0x52d54 | `(unknown callee) 0x21eb8` | `0x6` | `0x10ba58` | `0x39d` | `0x0` |
| `bl` at 0x52d64 | `(unknown callee) 0x51b28` | `0x32` | `0x10bba0` | `0x1` | `?` |
| `bl` at 0x52d70 | `(unknown callee) 0x3d890` | `0x2` | `?` | `?` | `ret(0x1f26c)` |
| `bl` at 0x52d8c | `(unknown callee) 0x21a38` | `ret(0x3d890)` | `?` | `?` | `ret(0x3d890)` |
| `bl` at 0x52db0 | `(unknown callee) 0x21eb8` | `0x3` | `0x10ba58` | `0x3a1` | `0x0` |
| `bl` at 0x52dc0 | `(unknown callee) 0x3d008` | `0x1` | `0x2` | `?` | `ret(0x3dce4)` |
| `bl` at 0x52ddc | `(unknown callee) 0x21a38` | `ret(0x3d008)` | `?` | `?` | `ret(0x3d008)` |
| `bl` at 0x52e00 | `(unknown callee) 0x21eb8` | `0x3` | `0x10ba58` | `0x3a4` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x52cd0 | `(r1 & 0xff) == [0x1434ac]:1` | `(r1 & 0xff) != [0x1434ac]:1` |
| `beq` at 0x52d04 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x52d14 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x52d24 | `ret(0x3dce4) == 0x0` | `ret(0x3dce4) != 0x0` |
| `beq` at 0x52d38 | `ret(0x1f26c) == 0x0` | `ret(0x1f26c) != 0x0` |
| `beq` at 0x52d84 | `ret(0x3d890) == 0x0` | `ret(0x3d890) != 0x0` |
| `beq` at 0x52dd4 | `ret(0x3d008) == 0x0` | `ret(0x3d008) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x52e04 | `r0` | `?` |
