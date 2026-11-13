# ARM I2C entry paths in 1.40.5

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
The fact sheets report no unresolved computed transfers. Indirect call targets and callee effects
remain unknown. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/i2c-paths-facts.txt) retain all tool sections.

## `i2cInit`

- `i2cInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc7ea4 to 0xc807c.
  Established. Four-byte stores at absolute `0x2b2f20` first write `ret(0x1b130)` and later
  `[0x2b2f20]+36` (`str` at 0xc7f64, `str` at 0xc7fbc). The visible unnamed call to `0x1b130` has
  candidates `0x0`, `0x100`, `0x3`, `0x1` (`bl` at 0xc7f60). The value `0x56000000` is stored at `sp
  -0x24` (`str` at 0xc7f4c). The branch to the
  final loop is taken when `[0x1436d0] >= 0x0 (signed, 32-bit)` (`bge` at 0xc7fe0). Initialization
  effects inside callees and hardware identities are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc7eb4 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xc7eb8 | load | 4 | `abs 0xc8054` | `-` |
| `ldr` at 0xc7ebc | load | 4 | `abs 0x1436d0` | `-` |
| `ldr` at 0xc7ec8 | load | 4 | `abs 0xc8058` | `-` |
| `str` at 0xc7ecc | store | 4 | `sp -0x28` | `0x133a54` |
| `ldr` at 0xc7ed4 | load | 4 | `abs 0xc805c` | `-` |
| `ldr` at 0xc7ee4 | load | 4 | `abs 0xc8060` | `-` |
| `ldr` at 0xc7ee8 | load | 4 | `abs 0xc8064` | `-` |
| `str` at 0xc7ef4 | store | 4 | `sp -0x1c` | `ret(0x1b400)` |
| `ldr` at 0xc7ef8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc7f10 | load | 4 | `abs 0xc8068` | `-` |
| `str` at 0xc7f14 | store | 4 | `sp -0x28` | `0x133a80` |
| `ldr` at 0xc7f18 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `str` at 0xc7f1c | store | 4 | `sp -0x24` | `[ret(0x1b148)]` |
| `ldr` at 0xc7f24 | load | 4 | `abs 0xc805c` | `-` |
| `ldr` at 0xc7f3c | load | 4 | `abs 0xc806c` | `-` |
| `ldr` at 0xc7f40 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc7f44 | store | 4 | `sp -0x28` | `?` |
| `str` at 0xc7f4c | store | 4 | `sp -0x24` | `0x56000000` |
| `str` at 0xc7f64 | store | 4 | `abs 0x2b2f20` | `ret(0x1b130)` |
| `ldr` at 0xc7f68 | load | 4 | `abs 0x2b2f20` | `-` |
| `ldr` at 0xc7f80 | load | 4 | `abs 0xc8070` | `-` |
| `str` at 0xc7f84 | store | 4 | `sp -0x28` | `0x133aa0` |
| `ldr` at 0xc7f88 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `str` at 0xc7f8c | store | 4 | `sp -0x24` | `[ret(0x1b148)]` |
| `ldr` at 0xc7f94 | load | 4 | `abs 0xc805c` | `-` |
| `ldr` at 0xc7fac | load | 4 | `abs 0xc806c` | `-` |
| `ldr` at 0xc7fb0 | load | 4 | `abs 0xc806c` | `-` |
| `ldr` at 0xc7fb4 | load | 4 | `abs 0x2b2f20` | `-` |
| `str` at 0xc7fbc | store | 4 | `abs 0x2b2f20` | `[0x2b2f20]+36` |
| `ldr` at 0xc7fc0 | load | 4 | `abs 0xc8074` | `-` |
| `ldr` at 0xc7fd0 | load | 4 | `abs 0xc8054` | `-` |
| `str` at 0xc7fd4 | store | 4 | `abs 0x1436d0` | `ret(0x1b400)` |
| `ldr` at 0xc7fd8 | load | 4 | `abs 0x1436d0` | `-` |
| `ldr` at 0xc7fec | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xc7ff8 | load | 4 | `abs 0xc8078` | `-` |
| `str` at 0xc7ffc | store | 4 | `sp -0x28` | `0x133ad8` |
| `str` at 0xc8000 | store | 4 | `sp -0x24` | `ret(0x21704)` |
| `ldr` at 0xc8008 | load | 4 | `abs 0xc805c` | `-` |
| `strb` at 0xc801c | store | 1 | `sp -0x1d` | `0x1` |
| `ldrb` at 0xc8020 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0xc8030 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0xc8040 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0xc8048 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldmdb` at 0xc8050 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc8050 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc8050 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc8050 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc7ee0 | `(unknown callee) 0x21eb8` | `0x9` | `0x133a6c` | `0x4d` | `0x0` |
| `bl` at 0xc7eec | `(unknown callee) 0x1b400` | `0x133a74` | `0x1002` | `?` | `?` |
| `bl` at 0xc7f04 | `(unknown callee) 0x1b148` | `ret(0x1b400)` | `?` | `?` | `ret(0x1b400)` |
| `bl` at 0xc7f30 | `(unknown callee) 0x21eb8` | `0x9` | `0x133a6c` | `0x53` | `0x0` |
| `bl` at 0xc7f38 | `(unknown callee) 0x1b43c` | `0x1` | `?` | `?` | `?` |
| `bl` at 0xc7f60 | `(unknown callee) 0x1b130` | `0x0` | `0x100` | `0x3` | `0x1` |
| `bl` at 0xc7f74 | `(unknown callee) 0x1b148` | `ret(0x1b130)` | `?` | `?` | `[0x2b2f20]` |
| `bl` at 0xc7fa0 | `(unknown callee) 0x21eb8` | `0x9` | `0x133a6c` | `0x5b` | `0x0` |
| `bl` at 0xc7fa8 | `(unknown callee) 0x1b43c` | `0x1` | `?` | `?` | `?` |
| `bl` at 0xc7fc8 | `(unknown callee) 0x1b400` | `0x133acc` | `0x2` | `0x2b2f20` | `[0x2b2f20]+36` |
| `bl` at 0xc7fe4 | `(unknown callee) 0x1b148` | `ret(0x1b400)` | `?` | `ret(0x1b400)` | `[0x1436d0]` |
| `bl` at 0xc7ff0 | `(unknown callee) 0x21704` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0xc8014 | `(unknown callee) 0x21eb8` | `0x9` | `0x133a6c` | `0x65` | `0x0` |
| `bl` at 0xc8038 | `(unknown callee) 0xd60f4` | `?` | `?` | `?` | `?` |
| `bl` at 0xc803c | `(unknown callee) 0xc860c` | `ret(0xd60f4)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xc7ec4 | `[0x1436d0] < 0x0 (signed, 32-bit)` | `[0x1436d0] >= 0x0 (signed, 32-bit)` |
| `bge` at 0xc7f00 | `ret(0x1b400) >= 0x0 (signed, 32-bit)` | `ret(0x1b400) < 0x0 (signed, 32-bit)` |
| `bne` at 0xc7f70 | `Z == 0` | `not (Z == 0)` |
| `bge` at 0xc7fe0 | `[0x1436d0] >= 0x0 (signed, 32-bit)` | `[0x1436d0] < 0x0 (signed, 32-bit)` |
| `bls` at 0xc8028 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc8050 | `r0` | `?` |

## `i2cRead`

- `i2cRead`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc8740 to 0xc878c.
  Established. The entry stores the low byte `(r0 & 0xff)` at `sp -0x11`, the two-byte `(r2 &
  0xffff)` at `sp -0x1a`, and four-byte `r1` at `sp -0x18` (`strb` at 0xc8758, `strh` at 0xc875c,
  `str` at 0xc8754); its sole unnamed call to `0xc8a2c` carries candidates `0x1`, `(r0 & 0xff)`,
  unknown, `[sp-24]` (`bl` at 0xc877c). The exit reports `ret(0xc8a2c)` (`ldmdb` at 0xc8788). Slave
  and transfer meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc8754 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xc8758 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strh` at 0xc875c | store | 2 | `sp -0x1a` | `(r2 & 0xffff)` |
| `ldrb` at 0xc8760 | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0xc8764 | load | 2 | `sp -0x1a` | `-` |
| `str` at 0xc8768 | store | 4 | `sp -0x20` | `(r2 & 0xffff)` |
| `ldr` at 0xc8778 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xc8788 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc8788 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc8788 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc877c | `(unknown callee) 0xc8a2c` | `0x1` | `(r0 & 0xff)` | `?` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc8788 | `r0` | `ret(0xc8a2c)` |

## `i2cWrite`

- `i2cWrite`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc8698 to 0xc86e4.
  Established. Three explicit stores preserve four-byte `r1` at `sp -0x18`, byte `(r0 & 0xff)` at
  `sp -0x11`, and two-byte `(r2 & 0xffff)` at `sp -0x1a` (`str` at 0xc86ac, `strb` at 0xc86b0,
  `strh` at 0xc86b4). The unnamed call to `0xc8a2c` has candidates `0x0`, `(r0 & 0xff)`, unknown,
  `[sp-24]` (`bl` at 0xc86d4); its returned expression reaches the exit (`ldmdb` at 0xc86e0). Write
  extent and bus action are not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc86ac | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xc86b0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strh` at 0xc86b4 | store | 2 | `sp -0x1a` | `(r2 & 0xffff)` |
| `ldrb` at 0xc86b8 | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0xc86bc | load | 2 | `sp -0x1a` | `-` |
| `str` at 0xc86c0 | store | 4 | `sp -0x20` | `(r2 & 0xffff)` |
| `ldr` at 0xc86d0 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xc86e0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc86e0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc86e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc86d4 | `(unknown callee) 0xc8a2c` | `0x0` | `(r0 & 0xff)` | `?` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc86e0 | `r0` | `ret(0xc8a2c)` |

## `i2cReadOffset`

- `i2cReadOffset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc878c to 0xc87e8.
  Established. The byte stores put `(r0 & 0xff)` at `sp -0x11` and `(r1 & 0xff)` at `sp -0x12`
  (`strb` at 0xc87a8, `strb` at 0xc87b0). A four-byte store at `sp -0x20` holds `(r3 & 0xffff)`
  (`str` at 0xc87c4). The visible unnamed call to `0xc8a2c` carries `0x1`, `(r0 & 0xff)`, `(r1 &
  0xff)`, `r2` (`bl` at 0xc87d8). No conditional branch is reported. Address and offset semantics
  are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc879c | store | 4 | `sp -0x18` | `r2` |
| `strb` at 0xc87a8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xc87b0 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strh` at 0xc87b4 | store | 2 | `sp -0x1a` | `(r3 & 0xffff)` |
| `ldrb` at 0xc87b8 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xc87bc | load | 1 | `sp -0x12` | `-` |
| `ldrh` at 0xc87c0 | load | 2 | `sp -0x1a` | `-` |
| `str` at 0xc87c4 | store | 4 | `sp -0x20` | `(r3 & 0xffff)` |
| `ldr` at 0xc87d4 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xc87e4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc87e4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc87e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc87d8 | `(unknown callee) 0xc8a2c` | `0x1` | `(r0 & 0xff)` | `(r1 & 0xff)` | `r2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc87e4 | `r0` | `ret(0xc8a2c)` |

## `i2cWriteOffset`

- `i2cWriteOffset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc86e4 to 0xc8740.
  Established. The four-byte entry store preserves `r2` at `sp -0x18` (`str` at 0xc86f4). Two byte
  stores at `sp -0x11` and `sp -0x12` preserve `(r0 & 0xff)` and `(r1 & 0xff)` (`strb` at 0xc8700,
  `strb` at 0xc8708). The unnamed call to `0xc8a2c` has candidates `0x0`, `(r0 & 0xff)`, `(r1 &
  0xff)`, `r2` (`bl` at 0xc8730). The exit carries `ret(0xc8a2c)` (`ldmdb` at 0xc873c). Command and
  transfer effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc86f4 | store | 4 | `sp -0x18` | `r2` |
| `strb` at 0xc8700 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xc8708 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strh` at 0xc870c | store | 2 | `sp -0x1a` | `(r3 & 0xffff)` |
| `ldrb` at 0xc8710 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xc8714 | load | 1 | `sp -0x12` | `-` |
| `ldrh` at 0xc8718 | load | 2 | `sp -0x1a` | `-` |
| `str` at 0xc871c | store | 4 | `sp -0x20` | `(r3 & 0xffff)` |
| `ldr` at 0xc872c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xc873c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc873c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc873c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc8730 | `(unknown callee) 0xc8a2c` | `0x0` | `(r0 & 0xff)` | `(r1 & 0xff)` | `r2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc873c | `r0` | `ret(0xc8a2c)` |

## `i2cReadByte`

- `i2cReadByte`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc8914 to 0xc8998.
  Established. The first unnamed call to `0xc8348` carries `(r0 & 0xff)`, `r1`, `r2`, `(r0 & 0xff)`
  (`bl` at 0xc8940). A zero result takes the branch to a second unnamed call (`beq` at 0xc8954),
  whose target `0xc8740` has candidates `[sp-18]:1`, `[sp-24]`, `0x1`, `[sp-18]:1` (`bl` at
  0xc8974). Another call to `0xc8488` carries candidate zero `0x0` (`bl` at 0xc8984). The merged
  return is unknown (`ldmdb` at 0xc8994). Bus selection and one-byte transfer semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc8928 | store | 4 | `sp -0x18` | `r2` |
| `strb` at 0xc892c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xc8934 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldrb` at 0xc8938 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xc8948 | store | 4 | `sp -0x1c` | `ret(0xc8348)` |
| `ldr` at 0xc894c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc8958 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc895c | store | 4 | `sp -0x20` | `ret(0xc8348)` |
| `ldrb` at 0xc8964 | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xc896c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc897c | store | 4 | `sp -0x1c` | `ret(0xc8740)` |
| `ldr` at 0xc8988 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc898c | store | 4 | `sp -0x20` | `[sp-28]` |
| `ldr` at 0xc8990 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xc8994 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc8994 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc8994 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc8940 | `(unknown callee) 0xc8348` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xc8974 | `(unknown callee) 0xc8740` | `[sp-18]:1` | `[sp-24]` | `0x1` | `[sp-18]:1` |
| `bl` at 0xc8984 | `(unknown callee) 0xc8488` | `0x0` | `?` | `?` | `ret(0xc8740)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc8954 | `ret(0xc8348) == 0x0` | `ret(0xc8348) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc8994 | `r0` | `?` |

## `i2cWriteByte`

- `i2cWriteByte`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc87e8 to 0xc8874.
  Established. Three byte stores preserve `(r0 & 0xff)`, `(r1 & 0xff)`, `(r2 & 0xff)` at `sp -0x11`,
  `-0x12`, `-0x13` (`strb` at 0xc87fc, `strb` at 0xc8804, `strb` at 0xc880c). The first call-result
  zero takes the branch to the later arm (`beq` at 0xc882c); its unnamed call to `0xc8698` has
  candidates `[sp-18]:1`, `sp-19`, `0x1`, `[sp-18]:1` (`bl` at 0xc8850). The final unnamed call to
  `0xc8488` carries `0x0` in candidate zero (`bl` at 0xc8860). The bus operation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xc87fc | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xc8804 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0xc880c | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `ldrb` at 0xc8810 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xc8820 | store | 4 | `sp -0x18` | `ret(0xc8348)` |
| `ldr` at 0xc8824 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc8830 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc8834 | store | 4 | `sp -0x1c` | `ret(0xc8348)` |
| `ldrb` at 0xc883c | load | 1 | `sp -0x12` | `-` |
| `str` at 0xc8858 | store | 4 | `sp -0x18` | `ret(0xc8698)` |
| `ldr` at 0xc8864 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc8868 | store | 4 | `sp -0x1c` | `[sp-24]` |
| `ldr` at 0xc886c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xc8870 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc8870 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc8870 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc8818 | `(unknown callee) 0xc8348` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xc8850 | `(unknown callee) 0xc8698` | `[sp-18]:1` | `sp-19` | `0x1` | `[sp-18]:1` |
| `bl` at 0xc8860 | `(unknown callee) 0xc8488` | `0x0` | `?` | `?` | `ret(0xc8698)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc882c | `ret(0xc8348) == 0x0` | `ret(0xc8348) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc8870 | `r0` | `?` |

## `i2cReadByteOffset`

- `i2cReadByteOffset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc8998 to 0xc8a2c.
  Established. The entry stores four-byte `r3` at `sp -0x18` and byte `(r2 & 0xff)` at `sp -0x13`
  (`str` at 0xc89a8, `strb` at 0xc89c0). The zero result of the first unnamed call takes the later
  arm (`beq` at 0xc89e0). That arm calls `0xc878c` with candidates `[sp-18]:1`, `[sp-19]:1`,
  `[sp-24]`, `0x1` (`bl` at 0xc8a08). A later call to `0xc8488` has `0x0`, unknown, unknown,
  `ret(0xc878c)` (`bl` at 0xc8a18). The function name does not establish offset or byte-transfer
  meaning.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc89a8 | store | 4 | `sp -0x18` | `r3` |
| `strb` at 0xc89b0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xc89b8 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0xc89c0 | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `ldrb` at 0xc89c4 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xc89d4 | store | 4 | `sp -0x1c` | `ret(0xc8348)` |
| `ldr` at 0xc89d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc89e4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc89e8 | store | 4 | `sp -0x20` | `ret(0xc8348)` |
| `ldrb` at 0xc89f0 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0xc89f4 | load | 1 | `sp -0x13` | `-` |
| `ldr` at 0xc8a00 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc8a10 | store | 4 | `sp -0x1c` | `ret(0xc878c)` |
| `ldr` at 0xc8a1c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc8a20 | store | 4 | `sp -0x20` | `[sp-28]` |
| `ldr` at 0xc8a24 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xc8a28 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc8a28 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc8a28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc89cc | `(unknown callee) 0xc8348` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xc8a08 | `(unknown callee) 0xc878c` | `[sp-18]:1` | `[sp-19]:1` | `[sp-24]` | `0x1` |
| `bl` at 0xc8a18 | `(unknown callee) 0xc8488` | `0x0` | `?` | `?` | `ret(0xc878c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc89e0 | `ret(0xc8348) == 0x0` | `ret(0xc8348) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc8a28 | `r0` | `?` |

## `i2cWriteByteOffset`

- `i2cWriteByteOffset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc8874 to 0xc8914.
  Established. Four byte stores copy the low bytes of entry registers `r0`, `r1`, `r2`, `r3` at `sp
  -0x11`, `-0x12`, `-0x13`, `-0x14` (`strb` at 0xc888c, `strb` at 0xc8894, `strb` at 0xc889c, `strb`
  at 0xc88a4). The first unnamed call-result zero takes its later arm (`beq` at 0xc88c4). The
  unnamed target `0xc86e4` then has candidates `[sp-18]:1`, `[sp-19]:1`, `sp-20`, `0x1` (`bl` at
  0xc88f0). The tracked return remains unknown (`ldmdb` at 0xc8910). Record and transfer meanings
  are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xc888c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xc8894 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0xc889c | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `strb` at 0xc88a4 | store | 1 | `sp -0x14` | `(r3 & 0xff)` |
| `ldrb` at 0xc88a8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xc88b8 | store | 4 | `sp -0x18` | `ret(0xc8348)` |
| `ldr` at 0xc88bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc88c8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc88cc | store | 4 | `sp -0x1c` | `ret(0xc8348)` |
| `ldrb` at 0xc88d4 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0xc88d8 | load | 1 | `sp -0x13` | `-` |
| `str` at 0xc88f8 | store | 4 | `sp -0x18` | `ret(0xc86e4)` |
| `ldr` at 0xc8904 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc8908 | store | 4 | `sp -0x1c` | `[sp-24]` |
| `ldr` at 0xc890c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xc8910 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc8910 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc8910 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc88b0 | `(unknown callee) 0xc8348` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xc88f0 | `(unknown callee) 0xc86e4` | `[sp-18]:1` | `[sp-19]:1` | `sp-20` | `0x1` |
| `bl` at 0xc8900 | `(unknown callee) 0xc8488` | `0x0` | `?` | `?` | `ret(0xc86e4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc88c4 | `ret(0xc8348) == 0x0` | `ret(0xc8348) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc8910 | `r0` | `?` |

## `i2cReset`

- `i2cReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc860c to 0xc8698.
  Established. The four-byte local is initialized to zero at `sp -0x14` and later updated to
  `[sp-20]+1` (`str` at 0xc8620, `str` at 0xc867c). The unnamed call to `0x1b49c` carries
  `[0x1436d0]`, `0x782`, `0x0`, `0x1436d0` (`bl` at 0xc8644); its nonnegative result takes the
  branch past two further calls (`bge` at 0xc8650). The loop predicate retains unknown operands
  (`ble` at 0xc862c). Neither a reset action nor a retry count follows from the tracked values.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc8620 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0xc8624 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc8634 | load | 4 | `abs 0xc8688` | `-` |
| `ldr` at 0xc8638 | load | 4 | `abs 0x1436d0` | `-` |
| `ldr` at 0xc863c | load | 4 | `abs 0xc868c` | `-` |
| `ldr` at 0xc8660 | load | 4 | `abs 0xc8690` | `-` |
| `ldr` at 0xc8664 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0xc8668 | load | 4 | `abs 0xc8694` | `-` |
| `ldr` at 0xc866c | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xc8674 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc867c | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldmdb` at 0xc8684 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc8684 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc8684 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc8644 | `(unknown callee) 0x1b49c` | `[0x1436d0]` | `0x782` | `0x0` | `0x1436d0` |
| `bl` at 0xc8654 | `(unknown callee) 0x1b148` | `ret(0x1b49c)` | `?` | `?` | `ret(0x1b49c)` |
| `bl` at 0xc8670 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x133be4` | `[ret(0x1b148)]` | `0x145ae8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xc862c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bge` at 0xc8650 | `ret(0x1b49c) >= 0x0 (signed, 32-bit)` | `ret(0x1b49c) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc8684 | `r0` | `?` |

## `i2cResetBusSelect`

- `i2cResetBusSelect`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc80f4 to 0xc8158.
  Established. The first branch takes its bypass arm for `[0x1436d4] == 0x0` (`beq` at 0xc8110). The
  other arm calls unnamed target `0xc8348` with `0x1`, `r1`, `r2`, `[0x1436d4]` (`bl` at 0xc8118). A
  zero result takes the path to unnamed target `0xc8488` (`beq` at 0xc812c, `bl` at 0xc8140). A
  four-byte zero assignment at `sp -0x18` also lies on the bypass path (`str` at 0xc8148). The
  merged exit is unknown (`ldmdb` at 0xc8150). Selection-reset behavior is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xc8104 | load | 4 | `abs 0xc8154` | `-` |
| `ldr` at 0xc8108 | load | 4 | `abs 0x1436d4` | `-` |
| `str` at 0xc8120 | store | 4 | `sp -0x14` | `ret(0xc8348)` |
| `ldr` at 0xc8124 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc8130 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc8134 | store | 4 | `sp -0x18` | `ret(0xc8348)` |
| `str` at 0xc8148 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xc814c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xc8150 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc8150 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc8150 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc8118 | `(unknown callee) 0xc8348` | `0x1` | `r1` | `r2` | `[0x1436d4]` |
| `bl` at 0xc8140 | `(unknown callee) 0xc8488` | `0x0` | `?` | `?` | `ret(0xc8348)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc8110 | `[0x1436d4] == 0x0` | `[0x1436d4] != 0x0` |
| `beq` at 0xc812c | `ret(0xc8348) == 0x0` | `ret(0xc8348) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc8150 | `r0` | `?` |

## `i2cSelectBus`

- `i2cSelectBus`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc857c to 0xc860c.
  Established. The branch takes the fallback arm when `[0x1436d4] == 0x0` (`beq` at 0xc85a0). The
  other arm loads four bytes at absolute `0x1436d4` (`ldr` at 0xc85ac) before an unknown callee
  receives candidates `(r0 & 0xff)`, `r1`, `(r0 & 0xff)`, `[0x1436d4]` (`mov` at 0xc85b8). A
  four-byte local store records `ret(unknown)` at `sp -0x18` (`str` at 0xc85c0). The fallback
  unnamed call has candidates `0x1`, `0x133a6c`, `0x187`, `0x0` (`bl` at 0xc85e8). Callback identity
  and bus-selection effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xc8590 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xc8594 | load | 4 | `abs 0xc85fc` | `-` |
| `ldr` at 0xc8598 | load | 4 | `abs 0x1436d4` | `-` |
| `ldr` at 0xc85a4 | load | 4 | `abs 0xc85fc` | `-` |
| `ldrb` at 0xc85a8 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xc85ac | load | 4 | `abs 0x1436d4` | `-` |
| `str` at 0xc85c0 | store | 4 | `sp -0x18` | `ret(unknown)` |
| `ldr` at 0xc85c8 | load | 4 | `abs 0xc8600` | `-` |
| `str` at 0xc85cc | store | 4 | `sp -0x20` | `0x133bc8` |
| `ldrb` at 0xc85d0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xc85d4 | store | 4 | `sp -0x1c` | `(r0 & 0xff)` |
| `ldr` at 0xc85dc | load | 4 | `abs 0xc8604` | `-` |
| `ldr` at 0xc85e0 | load | 4 | `abs 0xc8608` | `-` |
| `str` at 0xc85f0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xc85f4 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xc85f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc85f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc85f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0xc85b8 | `(unknown callee) (unknown target)` | `(r0 & 0xff)` | `r1` | `(r0 & 0xff)` | `[0x1436d4]` |
| `bl` at 0xc85e8 | `(unknown callee) 0x21eb8` | `0x1` | `0x133a6c` | `0x187` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc85a0 | `[0x1436d4] == 0x0` | `[0x1436d4] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc85f8 | `r0` | `?` |

## `i2cSetBusSelect`

- `i2cSetBusSelect`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc80a4 to 0xc80cc.
  Established. The four-byte entry value `r0` is first stored at `sp -0x14` (`str` at 0xc80b4), then
  assigned at absolute `0x1436d4` (`str` at 0xc80c0); its exit retains `r0` (`ldmdb` at 0xc80c4). No
  call or conditional branch is reported. A callback-registration role is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc80b4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc80b8 | load | 4 | `abs 0xc80c8` | `-` |
| `ldr` at 0xc80bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc80c0 | store | 4 | `abs 0x1436d4` | `r0` |
| `ldmdb` at 0xc80c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc80c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc80c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc80c4 | `r0` | `r0` |

## `i2cSetBusDeselect`

- `i2cSetBusDeselect`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc80cc to 0xc80f4.
  Established. The explicit four-byte stores preserve `r0` at `sp -0x14` and write it to absolute
  `0x2adf00` (`str` at 0xc80dc, `str` at 0xc80e8). The return also carries `r0` (`ldmdb` at
  0xc80ec). These assignments do not identify a deselection callback or a hardware window.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc80dc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc80e0 | load | 4 | `abs 0xc80f0` | `-` |
| `ldr` at 0xc80e4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc80e8 | store | 4 | `abs 0x2adf00` | `r0` |
| `ldmdb` at 0xc80ec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc80ec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc80ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc80ec | `r0` | `r0` |

## `i2cSetSwitch`

- `i2cSetSwitch`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc82a4 to 0xc830c.
  Established. The entry byte `(r0 & 0xff)` is stored at `sp -0x11` (`strb` at 0xc82b8); its loop
  branch retains an unknown comparison (`ble` at 0xc82d0). After the loop, a four-byte load reads
  absolute `0x2b2f20` (`ldr` at 0xc82ec), and a four-byte store writes `(r0 & 0xff)` at `[0x2b2f20]
  +0x0` (`str` at 0xc82f4). The exit value is zero (`ldmdb` at 0xc8300). The pointee identity and
  switch encoding are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xc82b8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0xc82c0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xc82c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc82c8 | load | 4 | `abs 0xc8304` | `-` |
| `ldr` at 0xc82d8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc82e0 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xc82e8 | load | 4 | `abs 0xc8308` | `-` |
| `ldr` at 0xc82ec | load | 4 | `abs 0x2b2f20` | `-` |
| `ldrb` at 0xc82f0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xc82f4 | store | 4 | `[0x2b2f20] +0x0` | `(r0 & 0xff)` |
| `ldmdb` at 0xc8300 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc8300 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc8300 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xc82d0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc8300 | `r0` | `0x0` |
