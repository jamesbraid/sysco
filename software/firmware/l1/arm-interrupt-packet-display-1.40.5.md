# ARM interrupt, packet and display paths in 1.40.5

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

[Fact sheets and full listings](disasm/arm-1.40.5-package4/interrupt-packet-display-facts.txt) retain all tool sections.

## `extIntInit`

- `extIntInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc8df0 to 0xc8f8c.
  Established. The byte assignment at `0x2adf10` retains `([sp-28]:1 & 0xff)` (`strb` at 0xc8e70). A
  four-byte assignment at `0x2adf0c` retains `ret(0x1b454)` (`str` at 0xc8e90). The earlier branch
  takes `[0x2adf10]:1 == 0x0` to `0xc8e40` and falls through for nonzero values (`beq` at 0xc8e14).
  Initialisation roles and capacity semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc8e00 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xc8e04 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xc8e08 | load | 4 | `abs 0xc8f74` | `-` |
| `ldrb` at 0xc8e0c | load | 1 | `abs 0x2adf10` | `-` |
| `ldr` at 0xc8e18 | load | 4 | `abs 0xc8f78` | `-` |
| `str` at 0xc8e1c | store | 4 | `sp -0x2c` | `0x133c6c` |
| `ldr` at 0xc8e20 | load | 4 | `abs 0xc8f74` | `-` |
| `ldrb` at 0xc8e24 | load | 1 | `abs 0x2adf10` | `-` |
| `str` at 0xc8e28 | store | 4 | `sp -0x28` | `[0x2adf10]:1` |
| `ldr` at 0xc8e30 | load | 4 | `abs 0xc8f7c` | `-` |
| `ldr` at 0xc8e40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc8e4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc8e58 | load | 4 | `abs 0xc8f80` | `-` |
| `str` at 0xc8e5c | store | 4 | `sp -0x18` | `0x1436dc` |
| `str` at 0xc8e64 | store | 4 | `sp -0x1c` | `0x6` |
| `ldr` at 0xc8e68 | load | 4 | `abs 0xc8f74` | `-` |
| `ldrb` at 0xc8e6c | load | 1 | `sp -0x1c` | `-` |
| `strb` at 0xc8e70 | store | 1 | `abs 0x2adf10` | `([sp-28]:1 & 0xff)` |
| `ldr` at 0xc8e74 | load | 4 | `abs 0xc8f84` | `-` |
| `ldr` at 0xc8e78 | load | 4 | `abs 0xc8f74` | `-` |
| `ldrb` at 0xc8e7c | load | 1 | `abs 0x2adf10` | `-` |
| `str` at 0xc8e90 | store | 4 | `abs 0x2adf0c` | `ret(0x1b454)` |
| `ldr` at 0xc8e94 | load | 4 | `abs 0x2adf0c` | `-` |
| `ldr` at 0xc8ea0 | load | 4 | `abs 0xc8f88` | `-` |
| `str` at 0xc8ea4 | store | 4 | `sp -0x2c` | `0x133ca4` |
| `ldr` at 0xc8ea8 | load | 4 | `abs 0xc8f74` | `-` |
| `ldrb` at 0xc8eac | load | 1 | `abs 0x2adf10` | `-` |
| `str` at 0xc8ec0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xc8ec8 | load | 4 | `abs 0xc8f7c` | `-` |
| `str` at 0xc8edc | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xc8ee0 | load | 4 | `abs 0xc8f74` | `-` |
| `ldrb` at 0xc8ee4 | load | 1 | `abs 0x2adf10` | `-` |
| `ldr` at 0xc8ee8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc8ef8 | load | 4 | `abs 0xc8f84` | `-` |
| `ldr` at 0xc8efc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc8f10 | load | 4 | `abs 0x2adf0c` | `-` |
| `str` at 0xc8f18 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xc8f1c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc8f20 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc8f28 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc8f30 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xc8f34 | store | 1 | `[sp-36] +0x5` | `?` |
| `ldr` at 0xc8f38 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc8f3c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc8f44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc8f4c | load | 4 | `(unknown)` | `-` |
| `str` at 0xc8f50 | store | 4 | `[sp-36] +0xc` | `?` |
| `ldr` at 0xc8f54 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc8f5c | store | 4 | `[sp-36] +0x8` | `0x0` |
| `ldr` at 0xc8f60 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc8f68 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldmdb` at 0xc8f70 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc8f70 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc8f70 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc8f70 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc8e3c | `(unknown callee) 0x21eb8` | `0x9` | `0x133c98` | `0x60` | `0x0` |
| `bl` at 0xc8e88 | `(unknown callee) 0x1b454` | `[0x2adf10]:1` | `0x38` | `0x2adf10` | `[0x2adf10]:1` |
| `bl` at 0xc8ed4 | `(unknown callee) 0x21eb8` | `0x9` | `0x133c98` | `0x70` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc8e14 | `[0x2adf10]:1 == 0x0` | `[0x2adf10]:1 != 0x0` |
| `beq` at 0xc8e48 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xc8e54 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xc8e9c | `[0x2adf0c] != 0x0` | `[0x2adf0c] == 0x0` |
| `blt` at 0xc8ef0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc8f70 | `r0` | `?` |

## `extIntRegister`

- `extIntRegister`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc8f8c to 0xc90d8.
  Established. Included assignments write `[sp-20]` at `[sp-28] +0x0` (`str` at 0xc9090),
  `([sp-22]:1 & 0xff)` at `[sp-28] +0x4` (`strb` at 0xc909c), and `([sp-23]:1 & 0xff)` at `[sp-28] +0x5` (`strb` at 0xc90a8). The branch takes unsigned `[[sp-32]+44]:1 <= 0x2` to `0xc9050` and
  falls through above that bound (`bls` at 0xc9024). Registration fields and callback roles remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc8f9c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xc8fa8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xc8fb0 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0xc8fb8 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldr` at 0xc8fbc | load | 4 | `abs 0xc90c4` | `-` |
| `ldrb` at 0xc8fc0 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xc8fc4 | load | 1 | `abs 0x2adf10` | `-` |
| `ldr` at 0xc8fd0 | load | 4 | `abs 0xc90c8` | `-` |
| `str` at 0xc8fd4 | store | 4 | `sp -0x28` | `0x133cd4` |
| `ldrb` at 0xc8fd8 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xc8fdc | store | 4 | `sp -0x24` | `(r1 & 0xff)` |
| `ldr` at 0xc8fe4 | load | 4 | `abs 0xc90cc` | `-` |
| `ldr` at 0xc8ff4 | load | 4 | `abs 0xc90d0` | `-` |
| `ldrb` at 0xc8ff8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc900c | load | 4 | `abs 0x2adf0c` | `-` |
| `str` at 0xc9014 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xc9018 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc901c | load | 1 | `[sp-32] +0x2c` | `-` |
| `ldr` at 0xc9028 | load | 4 | `abs 0xc90d4` | `-` |
| `str` at 0xc902c | store | 4 | `sp -0x28` | `0x133d18` |
| `ldr` at 0xc9030 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc9034 | load | 1 | `[sp-32] +0x2c` | `-` |
| `str` at 0xc9038 | store | 4 | `sp -0x24` | `[[sp-32]+44]:1` |
| `ldr` at 0xc9040 | load | 4 | `abs 0xc90cc` | `-` |
| `ldrb` at 0xc9050 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc9064 | load | 4 | `abs 0xc90d0` | `-` |
| `ldr` at 0xc9068 | load | 4 | `abs 0x2adf0c` | `-` |
| `ldrb` at 0xc9070 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc9078 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc9084 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xc9088 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc908c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc9090 | store | 4 | `[sp-28] +0x0` | `[sp-20]` |
| `ldr` at 0xc9094 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc9098 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0xc909c | store | 1 | `[sp-28] +0x4` | `([sp-22]:1 & 0xff)` |
| `ldr` at 0xc90a0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc90a4 | load | 1 | `sp -0x17` | `-` |
| `strb` at 0xc90a8 | store | 1 | `[sp-28] +0x5` | `([sp-23]:1 & 0xff)` |
| `ldr` at 0xc90ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc90b0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc90b4 | load | 1 | `[sp-32] +0x2c` | `-` |
| `strb` at 0xc90bc | store | 1 | `[sp-32] +0x2c` | `([[sp-32]+44]:1+1 & 0xff)` |
| `ldmdb` at 0xc90c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc90c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc90c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc8ff0 | `(unknown callee) 0x21eb8` | `0x9` | `0x133c98` | `0x8e` | `0x0` |
| `bl` at 0xc904c | `(unknown callee) 0x21eb8` | `0x9` | `0x133c98` | `0x96` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xc8fcc | `(r1 & 0xff) < [0x2adf10]:1 (unsigned, 32-bit)` | `(r1 & 0xff) >= [0x2adf10]:1 (unsigned, 32-bit)` |
| `bls` at 0xc9024 | `[[sp-32]+44]:1 <= 0x2 (unsigned, 32-bit)` | `[[sp-32]+44]:1 > 0x2 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc90c0 | `r0` | `?` |

## `extIntSetPollTimeout`

- `extIntSetPollTimeout`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc90d8 to 0xc9170.
  Established. A four-byte assignment at `[sp-32] +0x8` retains `[sp-24]` (`str` at 0xc9158). The
  branch takes unsigned `(r0 & 0xff) < [0x2adf10]:1` to `0xc912c` and falls through for greater or
  equal values (`blo` at 0xc9104). Timeout field meaning and units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc90ec | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xc90f0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xc90f4 | load | 4 | `abs 0xc9160` | `-` |
| `ldrb` at 0xc90f8 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xc90fc | load | 1 | `abs 0x2adf10` | `-` |
| `ldr` at 0xc9108 | load | 4 | `abs 0xc9164` | `-` |
| `str` at 0xc910c | store | 4 | `sp -0x28` | `0x133cd4` |
| `ldrb` at 0xc9110 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xc9114 | store | 4 | `sp -0x24` | `(r0 & 0xff)` |
| `ldr` at 0xc911c | load | 4 | `abs 0xc9168` | `-` |
| `ldr` at 0xc912c | load | 4 | `abs 0xc916c` | `-` |
| `ldrb` at 0xc9130 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xc9144 | load | 4 | `abs 0x2adf0c` | `-` |
| `str` at 0xc914c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xc9150 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc9154 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc9158 | store | 4 | `[sp-32] +0x8` | `[sp-24]` |
| `ldmdb` at 0xc915c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc915c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc915c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc9128 | `(unknown callee) 0x21eb8` | `0x9` | `0x133c98` | `0xac` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xc9104 | `(r0 & 0xff) < [0x2adf10]:1 (unsigned, 32-bit)` | `(r0 & 0xff) >= [0x2adf10]:1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc915c | `r0` | `?` |

## `extIntEnable`

- `extIntEnable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc9170 to 0xc924c.
  Established. A byte assignment at `[sp-24] +0x4` writes `0x1` (`strb` at 0xc91e0). The branch
  takes `[[sp-24]+44]:1 != 0x0` to `0xc91d8` (`bne` at 0xc91d0). The unnamed `0x22480` call carries
  `[sp-24]+16`, `[[sp-24]+12]`, `0xc95ec`, `0x1` (`bl` at 0xc9228). Interrupt task and enable
  semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc9184 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0xc9188 | load | 4 | `abs 0xc9240` | `-` |
| `ldrb` at 0xc918c | load | 1 | `abs 0x2adf10` | `-` |
| `ldr` at 0xc9190 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc91a0 | load | 4 | `abs 0xc9244` | `-` |
| `ldr` at 0xc91a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc91b8 | load | 4 | `abs 0x2adf0c` | `-` |
| `str` at 0xc91c0 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xc91c4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc91c8 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0xc91d8 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xc91e0 | store | 1 | `[sp-24] +0x4` | `0x1` |
| `ldr` at 0xc91e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc91ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc91f0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc91f4 | store | 4 | `sp -0x2c` | `[sp-24]` |
| `str` at 0xc91fc | store | 4 | `sp -0x28` | `0x1000` |
| `str` at 0xc9204 | store | 4 | `sp -0x24` | `0xa` |
| `str` at 0xc920c | store | 4 | `sp -0x20` | `0xa` |
| `str` at 0xc9214 | store | 4 | `sp -0x1c` | `0x9` |
| `ldr` at 0xc921c | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xc9220 | load | 4 | `abs 0xc9248` | `-` |
| `ldr` at 0xc922c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc9234 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldmdb` at 0xc923c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc923c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc923c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc9228 | `(unknown callee) 0x22480` | `[sp-24]+16` | `[[sp-24]+12]` | `0xc95ec` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xc9198 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xc91d0 | `[[sp-24]+44]:1 != 0x0` | `[[sp-24]+44]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc923c | `r0` | `?` |

## `extIntMaxInts`

- `extIntMaxInts`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc924c to 0xc926c.
  Established. A byte absolute read targets `0x2adf10` (`ldrb` at 0xc925c). The reported return is
  `[0x2adf10]:1` (`ldmdb` at 0xc9264). Interrupt capacity and byte encoding remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xc9258 | load | 4 | `abs 0xc9268` | `-` |
| `ldrb` at 0xc925c | load | 1 | `abs 0x2adf10` | `-` |
| `ldmdb` at 0xc9264 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc9264 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc9264 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc9264 | `r0` | `[0x2adf10]:1` |

## `extIntCount`

- `extIntCount`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc926c to 0xc92f8.
  Established. The branch takes unsigned `(r0 & 0xff) < [0x2adf10]:1` to `0xc92bc` and falls through
  for greater or equal values (`blo` at 0xc9294). A later four-byte read has an unknown effective
  target (`ldr` at 0xc92dc). Counter location and the reported unknown return remain Unknown
  (`ldmdb` at 0xc92e4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xc9280 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xc9284 | load | 4 | `abs 0xc92e8` | `-` |
| `ldrb` at 0xc9288 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xc928c | load | 1 | `abs 0x2adf10` | `-` |
| `ldr` at 0xc9298 | load | 4 | `abs 0xc92ec` | `-` |
| `str` at 0xc929c | store | 4 | `sp -0x1c` | `0x133d44` |
| `ldrb` at 0xc92a0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xc92a4 | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldr` at 0xc92ac | load | 4 | `abs 0xc92f0` | `-` |
| `ldrb` at 0xc92bc | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xc92d0 | load | 4 | `abs 0xc92f4` | `-` |
| `ldr` at 0xc92d4 | load | 4 | `abs 0x2adf0c` | `-` |
| `ldr` at 0xc92dc | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0xc92e4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc92e4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc92e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc92b8 | `(unknown callee) 0x21eb8` | `0x9` | `0x133c98` | `0xd2` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xc9294 | `(r0 & 0xff) < [0x2adf10]:1 (unsigned, 32-bit)` | `(r0 & 0xff) >= [0x2adf10]:1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc92e4 | `r0` | `?` |

## `extIntName`

- `extIntName`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc92f8 to 0xc9384.
  Established. The four-byte absolute read targets `0x2adf0c` (`ldr` at 0xc9360). The next four-byte
  read has an unknown effective target (`ldr` at 0xc9368). The prior branch takes unsigned `(r0 & 0xff) < [0x2adf10]:1` to `0xc9348` (`blo` at 0xc9320). Name representation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xc930c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xc9310 | load | 4 | `abs 0xc9374` | `-` |
| `ldrb` at 0xc9314 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xc9318 | load | 1 | `abs 0x2adf10` | `-` |
| `ldr` at 0xc9324 | load | 4 | `abs 0xc9378` | `-` |
| `str` at 0xc9328 | store | 4 | `sp -0x1c` | `0x133d80` |
| `ldrb` at 0xc932c | load | 1 | `sp -0x11` | `-` |
| `str` at 0xc9330 | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldr` at 0xc9338 | load | 4 | `abs 0xc937c` | `-` |
| `ldrb` at 0xc9348 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xc935c | load | 4 | `abs 0xc9380` | `-` |
| `ldr` at 0xc9360 | load | 4 | `abs 0x2adf0c` | `-` |
| `ldr` at 0xc9368 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0xc9370 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc9370 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc9370 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc9344 | `(unknown callee) 0x21eb8` | `0x9` | `0x133c98` | `0xdd` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xc9320 | `(r0 & 0xff) < [0x2adf10]:1 (unsigned, 32-bit)` | `(r0 & 0xff) >= [0x2adf10]:1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc9370 | `r0` | `?` |

## `extIntIRQ`

- `extIntIRQ`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc9384 to 0xc9410.
  Established. A byte read has an unknown effective target (`ldrb` at 0xc93f4). The earlier branch
  takes unsigned `(r0 & 0xff) < [0x2adf10]:1` to `0xc93d4` (`blo` at 0xc93ac). The unnamed `0x21eb8`
  call carries `0x9`, `0x133c98`, `0xe8`, `0x0` (`bl` at 0xc93d0). IRQ mapping remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xc9398 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xc939c | load | 4 | `abs 0xc9400` | `-` |
| `ldrb` at 0xc93a0 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xc93a4 | load | 1 | `abs 0x2adf10` | `-` |
| `ldr` at 0xc93b0 | load | 4 | `abs 0xc9404` | `-` |
| `str` at 0xc93b4 | store | 4 | `sp -0x1c` | `0x133dbc` |
| `ldrb` at 0xc93b8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xc93bc | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldr` at 0xc93c4 | load | 4 | `abs 0xc9408` | `-` |
| `ldrb` at 0xc93d4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xc93e8 | load | 4 | `abs 0xc940c` | `-` |
| `ldr` at 0xc93ec | load | 4 | `abs 0x2adf0c` | `-` |
| `ldrb` at 0xc93f4 | load | 1 | `(unknown)` | `-` |
| `ldmdb` at 0xc93fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc93fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc93fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc93d0 | `(unknown callee) 0x21eb8` | `0x9` | `0x133c98` | `0xe8` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xc93ac | `(r0 & 0xff) < [0x2adf10]:1 (unsigned, 32-bit)` | `(r0 & 0xff) >= [0x2adf10]:1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc93fc | `r0` | `?` |

## `extIntIsPolling`

- `extIntIsPolling`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc9410 to 0xc949c.
  Established. A four-byte absolute read targets `0x2adf0c` (`ldr` at 0xc9478). The following byte
  read has an unknown effective target (`ldrb` at 0xc9480). The branch takes unsigned `(r0 & 0xff) < [0x2adf10]:1` to `0xc9460` (`blo` at 0xc9438). Polling-state encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xc9424 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xc9428 | load | 4 | `abs 0xc948c` | `-` |
| `ldrb` at 0xc942c | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xc9430 | load | 1 | `abs 0x2adf10` | `-` |
| `ldr` at 0xc943c | load | 4 | `abs 0xc9490` | `-` |
| `str` at 0xc9440 | store | 4 | `sp -0x1c` | `0x133df4` |
| `ldrb` at 0xc9444 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xc9448 | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldr` at 0xc9450 | load | 4 | `abs 0xc9494` | `-` |
| `ldrb` at 0xc9460 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xc9474 | load | 4 | `abs 0xc9498` | `-` |
| `ldr` at 0xc9478 | load | 4 | `abs 0x2adf0c` | `-` |
| `ldrb` at 0xc9480 | load | 1 | `(unknown)` | `-` |
| `ldmdb` at 0xc9488 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc9488 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc9488 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc945c | `(unknown callee) 0x21eb8` | `0x9` | `0x133c98` | `0xf2` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xc9438 | `(r0 & 0xff) < [0x2adf10]:1 (unsigned, 32-bit)` | `(r0 & 0xff) >= [0x2adf10]:1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc9488 | `r0` | `?` |

## `extIntPollingMask`

- `extIntPollingMask`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc949c to 0xc9530.
  Established. Included four-byte stack assignments at `sp -0x14` and `sp -0x18` write `0x0` (`str`
  at 0xc94b0, `str` at 0xc94b8). A byte read has an unknown effective target (`ldrb` at 0xc94f4).
  The later four-byte stack assignment at `sp -0x18` writes `[sp-24]+1` (`str` at 0xc9514). Polling
  mask representation and final result remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc94b0 | store | 4 | `sp -0x14` | `0x0` |
| `str` at 0xc94b8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xc94bc | load | 4 | `abs 0xc9528` | `-` |
| `ldrb` at 0xc94c0 | load | 1 | `abs 0x2adf10` | `-` |
| `ldr` at 0xc94c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc94d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc94e8 | load | 4 | `abs 0xc952c` | `-` |
| `ldr` at 0xc94ec | load | 4 | `abs 0x2adf0c` | `-` |
| `ldrb` at 0xc94f4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc94f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc9500 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc9508 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xc950c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc9514 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0xc951c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc9524 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc9524 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc9524 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xc94cc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc9524 | `r0` | `?` |

## `extIntIgnoreSpurious`

- `extIntIgnoreSpurious`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc9530 to 0xc95ec.
  Established. A byte store has unknown effective target and value (`strb` at 0xc95d0). The earlier
  branch takes unsigned `(r0 & 0xff) < [0x2adf10]:1` to `0xc9584` and falls through for greater or
  equal values (`blo` at 0xc955c). The unnamed `0x21eb8` call has `0x9`, `0x133c98`, `0x10b`, `0x0`
  (`bl` at 0xc9580). Spurious-interrupt accounting remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc9544 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xc9548 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xc954c | load | 4 | `abs 0xc95d8` | `-` |
| `ldrb` at 0xc9550 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xc9554 | load | 1 | `abs 0x2adf10` | `-` |
| `ldr` at 0xc9560 | load | 4 | `abs 0xc95dc` | `-` |
| `str` at 0xc9564 | store | 4 | `sp -0x20` | `0x133e30` |
| `ldrb` at 0xc9568 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xc956c | store | 4 | `sp -0x1c` | `(r0 & 0xff)` |
| `ldr` at 0xc9574 | load | 4 | `abs 0xc95e0` | `-` |
| `ldr` at 0xc9578 | load | 4 | `abs 0xc95e4` | `-` |
| `ldrb` at 0xc9584 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xc9598 | load | 4 | `abs 0xc95e8` | `-` |
| `ldr` at 0xc959c | load | 4 | `abs 0x2adf0c` | `-` |
| `ldrb` at 0xc95a4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xc95b8 | load | 4 | `abs 0xc95e8` | `-` |
| `ldr` at 0xc95bc | load | 4 | `abs 0x2adf0c` | `-` |
| `ldrb` at 0xc95c4 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xc95c8 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xc95d0 | store | 1 | `(unknown)` | `?` |
| `ldmdb` at 0xc95d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc95d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc95d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc9580 | `(unknown callee) 0x21eb8` | `0x9` | `0x133c98` | `0x10b` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xc955c | `(r0 & 0xff) < [0x2adf10]:1 (unsigned, 32-bit)` | `(r0 & 0xff) >= [0x2adf10]:1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc95d4 | `r0` | `?` |

## `PktGenRead`

- `PktGenRead`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x642c8 to 0x64394.
  Established. The unnamed `0x62a5c` call has `0x0`, `[sp-20]`, `[sp-24]`, `[sp-28]` (`bl` at
  0x64340); its nonzero result takes the branch to `0x64388` (`bne` at 0x64354). Included four-byte
  output assignments at `[sp+8] +0x0` and `[sp+12] +0x0` have unknown values (`str` at 0x6436c,
  `str` at 0x64384). Packet read mode and reply layout remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x642d8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x642dc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x642e0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x642e4 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x642f4 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x642f8 | store | 4 | `(unknown)` | `[sp]` |
| `ldr` at 0x64308 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x6430c | store | 4 | `(unknown)` | `[sp+4]` |
| `ldr` at 0x64310 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x64314 | store | 4 | `sp -0x44` | `[sp-32]` |
| `str` at 0x6431c | store | 4 | `sp -0x40` | `0x1` |
| `str` at 0x64324 | store | 4 | `sp -0x3c` | `sp-44` |
| `str` at 0x6432c | store | 4 | `sp -0x38` | `sp-52` |
| `ldr` at 0x64334 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x64338 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6433c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x64348 | store | 4 | `sp -0x24` | `ret(0x62a5c)` |
| `ldr` at 0x6434c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x64358 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x64368 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6436c | store | 4 | `[sp+8] +0x0` | `?` |
| `ldr` at 0x64370 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x64380 | load | 4 | `(unknown)` | `-` |
| `str` at 0x64384 | store | 4 | `[sp+12] +0x0` | `?` |
| `ldr` at 0x64388 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x64390 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x64390 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x64390 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x64340 | `(unknown callee) 0x62a5c` | `0x0` | `[sp-20]` | `[sp-24]` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x64354 | `ret(0x62a5c) != 0x0` | `ret(0x62a5c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x64390 | `r0` | `?` |

## `PktGenWrite`

- `PktGenWrite`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x64394 to 0x6444c.
  Established. The unnamed `0x62a5c` call carries `0x1`, `[sp-20]`, `[sp-24]`, `[sp-28]` (`bl` at
  0x64434). Included four-byte stores with unknown effective targets retain `[sp]` and `[sp+8]`
  (`str` at 0x643c4, `str` at 0x643ec). The reported result is `ret(0x62a5c)` (`ldmdb` at 0x64448).
  Packet write selector and call effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x643a4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x643a8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x643ac | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x643b0 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x643c0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x643c4 | store | 4 | `(unknown)` | `[sp]` |
| `ldr` at 0x643d4 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x643d8 | store | 4 | `(unknown)` | `[sp+4]` |
| `ldr` at 0x643e8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x643ec | store | 4 | `(unknown)` | `[sp+8]` |
| `ldr` at 0x643fc | load | 4 | `sp +0xc` | `-` |
| `str` at 0x64400 | store | 4 | `(unknown)` | `[sp+12]` |
| `ldr` at 0x64404 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x64408 | store | 4 | `sp -0x44` | `[sp-32]` |
| `str` at 0x64410 | store | 4 | `sp -0x40` | `0x1` |
| `str` at 0x64418 | store | 4 | `sp -0x3c` | `sp-44` |
| `str` at 0x64420 | store | 4 | `sp -0x38` | `sp-52` |
| `ldr` at 0x64428 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6442c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x64430 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6443c | store | 4 | `sp -0x24` | `ret(0x62a5c)` |
| `ldr` at 0x64440 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x64448 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x64448 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x64448 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x64434 | `(unknown callee) 0x62a5c` | `0x1` | `[sp-20]` | `[sp-24]` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x64448 | `r0` | `ret(0x62a5c)` |

## `displayReset`

- `displayReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52358 to 0x52564.
  Established. The byte assignments at `0x1490a2` write `0x1` and later `0x0` (`strb` at 0x523ec,
  `strb` at 0x5248c). An indirect call has an unknown target and candidate arguments `0x1490a3`,
  `0x10ba38`, `0x4`, `[0x149040]` (`mov` at 0x52480). The branch takes `ret(0xd73b0) == 0x0` to
  `0x524d4` and falls through for nonzero results (`beq` at 0x524c4). Display and device reset
  effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x52368 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x5236c | load | 4 | `abs 0x52534` | `-` |
| `ldrb` at 0x52370 | load | 1 | `abs 0x148fd4` | `-` |
| `ldr` at 0x5237c | load | 4 | `abs 0x52538` | `-` |
| `str` at 0x52380 | store | 4 | `sp -0x20` | `0xfffff8f7` |
| `ldr` at 0x52388 | load | 4 | `abs 0x5253c` | `-` |
| `ldrb` at 0x5238c | load | 1 | `abs 0x1434ac` | `-` |
| `strb` at 0x523a4 | store | 1 | `sp -0x15` | `?` |
| `ldr` at 0x523a8 | load | 4 | `abs 0x5253c` | `-` |
| `ldrb` at 0x523ac | load | 1 | `abs 0x1434ac` | `-` |
| `str` at 0x523c8 | store | 4 | `sp -0x1c` | `ret(0xd73b0)` |
| `ldr` at 0x523cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x523d8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x523dc | store | 4 | `sp -0x20` | `ret(0xd73b0)` |
| `ldr` at 0x523e4 | load | 4 | `abs 0x52540` | `-` |
| `strb` at 0x523ec | store | 1 | `abs 0x1490a2` | `0x1` |
| `ldr` at 0x523f0 | load | 4 | `abs 0x52544` | `-` |
| `ldr` at 0x523f4 | load | 4 | `abs 0x5253c` | `-` |
| `ldrb` at 0x523f8 | load | 1 | `abs 0x1434ac` | `-` |
| `ldr` at 0x523fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x52418 | load | 4 | `abs 0x149034` | `-` |
| `str` at 0x5242c | store | 4 | `sp -0x1c` | `ret(unknown)` |
| `ldr` at 0x52430 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5243c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52448 | load | 4 | `abs 0x52548` | `-` |
| `str` at 0x5244c | store | 4 | `sp -0x28` | `0x10bad0` |
| `str` at 0x52450 | store | 4 | `sp -0x24` | `ret(0x21a38)` |
| `ldr` at 0x52458 | load | 4 | `abs 0x5254c` | `-` |
| `ldr` at 0x5245c | load | 4 | `abs 0x52550` | `-` |
| `ldr` at 0x52468 | load | 4 | `abs 0x52544` | `-` |
| `ldr` at 0x5246c | load | 4 | `abs 0x149040` | `-` |
| `ldr` at 0x52470 | load | 4 | `abs 0x52554` | `-` |
| `ldr` at 0x52474 | load | 4 | `abs 0x52558` | `-` |
| `ldr` at 0x52484 | load | 4 | `abs 0x52540` | `-` |
| `strb` at 0x5248c | store | 1 | `abs 0x1490a2` | `0x0` |
| `ldr` at 0x52494 | load | 4 | `abs 0x5253c` | `-` |
| `ldrb` at 0x52498 | load | 1 | `abs 0x1434ac` | `-` |
| `ldrb` at 0x5249c | load | 1 | `sp -0x15` | `-` |
| `str` at 0x524b8 | store | 4 | `sp -0x1c` | `ret(0xd73b0)` |
| `ldr` at 0x524bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x524c8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x524cc | store | 4 | `sp -0x20` | `ret(0xd73b0)` |
| `ldr` at 0x524d4 | load | 4 | `abs 0x5255c` | `-` |
| `ldrb` at 0x524d8 | load | 1 | `abs 0x149048` | `-` |
| `ldr` at 0x524e8 | load | 4 | `abs 0x5255c` | `-` |
| `str` at 0x524f8 | store | 4 | `sp -0x1c` | `ret(0x51b28)` |
| `ldr` at 0x524fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52508 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5250c | store | 4 | `sp -0x20` | `ret(0x51b28)` |
| `ldr` at 0x52518 | load | 4 | `abs 0x52560` | `-` |
| `str` at 0x52528 | store | 4 | `sp -0x20` | `ret(0x51b28)` |
| `ldr` at 0x5252c | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x52530 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x52530 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x52530 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x52398 | `(unknown callee) 0xd75c0` | `0x0` | `[0x1434ac]:1` | `r2` | `[0x1434ac]:1` |
| `bl` at 0x523c0 | `(unknown callee) 0xd73b0` | `0x0` | `[0x1434ac]:1` | `0x70` | `0x0` |
| `mov` at 0x52424 | `(unknown callee) (unknown target)` | `[0x1434ac]:1` | `?` | `[0x149034]` | `?` |
| `bl` at 0x52440 | `(unknown callee) 0x21a38` | `ret(unknown)` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x52464 | `(unknown callee) 0x21eb8` | `0x3` | `0x10ba58` | `0x27f` | `0x0` |
| `mov` at 0x52480 | `(unknown callee) (unknown target)` | `0x1490a3` | `0x10ba38` | `0x4` | `[0x149040]` |
| `bl` at 0x52490 | `(unknown callee) 0x52014` | `ret(unknown)` | `?` | `0x1490a2` | `0x0` |
| `bl` at 0x524b0 | `(unknown callee) 0xd73b0` | `0x0` | `[0x1434ac]:1` | `0x70` | `[sp-21]:1` |
| `bl` at 0x524f0 | `(unknown callee) 0x51b28` | `0x31` | `0x149048` | `0x0` | `[0x149048]:1` |
| `bl` at 0x52520 | `(unknown callee) 0x51b28` | `0x32` | `0x10bb2c` | `0x1` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x52378 | `[0x148fd4]:1 != 0x0` | `[0x148fd4]:1 == 0x0` |
| `beq` at 0x523d4 | `ret(0xd73b0) == 0x0` | `ret(0xd73b0) != 0x0` |
| `beq` at 0x52438 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x524c4 | `ret(0xd73b0) == 0x0` | `ret(0xd73b0) != 0x0` |
| `beq` at 0x524e0 | `[0x149048]:1 == 0x0` | `[0x149048]:1 != 0x0` |
| `beq` at 0x52504 | `ret(0x51b28) == 0x0` | `ret(0x51b28) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x52530 | `r0` | `?` |
