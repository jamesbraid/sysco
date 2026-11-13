# ARM hub reset and scan callers in 1.40.5

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
These routines have no unresolved computed jumps. Numeric callee names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/hub-reset-scan-facts.txt) retain all tool sections.

## `NL4R_ResetRelease`

- `NL4R_ResetRelease`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x27b3c to 0x27c98.
  Established. Included byte assignments at absolute `0x148bd9` retain `0x0` and `0x1` at separate
  instructions (`strb` at 0x27b7c, `strb` at 0x27c54). Two unnamed `0x281bc` calls use unknown,
  `0x148bdc`, unknown, unknown (`bl` at 0x27bc8, `bl` at 0x27c3c). The later branch takes an unknown
  expression equal to `0x40` to `0x27c74` (`beq` at 0x27c68). Reset-release and selector meanings
  are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x27b4c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x27b54 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x27b58 | load | 4 | `abs 0x27c84` | `-` |
| `ldrb` at 0x27b5c | load | 1 | `abs 0x148bd8` | `-` |
| `ldr` at 0x27b68 | load | 4 | `abs 0x27c88` | `-` |
| `str` at 0x27b6c | store | 4 | `sp -0x24` | `0xfffffed4` |
| `ldr` at 0x27b74 | load | 4 | `abs 0x27c8c` | `-` |
| `strb` at 0x27b7c | store | 1 | `abs 0x148bd9` | `0x0` |
| `ldr` at 0x27b84 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27b94 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x27b98 | load | 4 | `abs 0x27c90` | `-` |
| `ldr` at 0x27b9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x27ba0 | load | 4 | `abs 0x148be0` | `-` |
| `ldr` at 0x27bb0 | load | 4 | `abs 0x27c94` | `-` |
| `ldr` at 0x27bb4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x27bbc | load | 4 | `abs 0x148bdc` | `-` |
| `str` at 0x27bd0 | store | 4 | `sp -0x20` | `ret(0x281bc)` |
| `ldr` at 0x27bd4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x27be0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x27bec | load | 4 | `sp -0x20` | `-` |
| `str` at 0x27bf0 | store | 4 | `sp -0x1c` | `ret(0x281bc)` |
| `ldr` at 0x27bf4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x27bfc | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x27c04 | load | 4 | `abs 0x27c90` | `-` |
| `ldr` at 0x27c08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27c0c | load | 4 | `abs 0x148be0` | `-` |
| `str` at 0x27c1c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x27c24 | load | 4 | `abs 0x27c94` | `-` |
| `ldr` at 0x27c28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27c30 | load | 4 | `abs 0x148bdc` | `-` |
| `str` at 0x27c44 | store | 4 | `sp -0x1c` | `ret(0x281bc)` |
| `ldr` at 0x27c4c | load | 4 | `abs 0x27c8c` | `-` |
| `strb` at 0x27c54 | store | 1 | `abs 0x148bd9` | `0x1` |
| `ldr` at 0x27c6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27c74 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x27c78 | store | 4 | `sp -0x24` | `[sp-28]` |
| `ldr` at 0x27c7c | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x27c80 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x27c80 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x27c80 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x27b80 | `(unknown callee) 0xe3cf8` | `r0` | `r1` | `0x148bd9` | `0x0` |
| `bl` at 0x27bc8 | `(unknown callee) 0x281bc` | `?` | `0x148bdc` | `?` | `?` |
| `bl` at 0x27c3c | `(unknown callee) 0x281bc` | `?` | `0x148bdc` | `?` | `?` |
| `bl` at 0x27c48 | `(unknown callee) 0xe3d64` | `?` | `?` | `?` | `?` |
| `bl` at 0x27c58 | `(unknown callee) 0x4f4d0` | `ret(0xe3d64)` | `?` | `0x148bd9` | `0x1` |
| `bl` at 0x27c70 | `(unknown callee) 0x27db8` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x27b64 | `[0x148bd8]:1 != 0x0` | `[0x148bd8]:1 == 0x0` |
| `bne` at 0x27b8c | `[sp-20] != 0xff` | `[sp-20] == 0xff` |
| `blt` at 0x27ba8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x27bdc | `ret(0x281bc) == 0x0` | `ret(0x281bc) != 0x0` |
| `bne` at 0x27be8 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `blt` at 0x27c14 | `[sp-20] < [0x148be0] (signed, 32-bit)` | `[sp-20] >= [0x148be0] (signed, 32-bit)` |
| `beq` at 0x27c68 | `? == 0x40` | `? != 0x40` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x27c80 | `r0` | `?` |

## `NL4R_SoftReset`

- `NL4R_SoftReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x27c98 to 0x27db8.
  Established. The branch takes entry `r0 != 0xff` to `0x27d5c` (`bne` at 0x27cd8). A later
  signed-less branch takes `r0 < [0x148be0]` to `0x27d7c` (`blt` at 0x27d6c). Both unnamed `0x284f8`
  calls carry unknown, `0x148bdc`, unknown, unknown (`bl` at 0x27d14, `bl` at 0x27d94). The
  four-byte assignment at `sp -0x20` has unknown value on another path (`str` at 0x27d74).
  Soft-reset effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x27ca8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x27cb0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x27cb4 | load | 4 | `abs 0x27da8` | `-` |
| `ldrb` at 0x27cb8 | load | 1 | `abs 0x148bd8` | `-` |
| `ldr` at 0x27cc4 | load | 4 | `abs 0x27dac` | `-` |
| `str` at 0x27cc8 | store | 4 | `sp -0x20` | `0xfffffed4` |
| `ldr` at 0x27cd0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27ce0 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x27ce4 | load | 4 | `abs 0x27db0` | `-` |
| `ldr` at 0x27ce8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27cec | load | 4 | `abs 0x148be0` | `-` |
| `ldr` at 0x27cfc | load | 4 | `abs 0x27db4` | `-` |
| `ldr` at 0x27d00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27d08 | load | 4 | `abs 0x148bdc` | `-` |
| `str` at 0x27d1c | store | 4 | `sp -0x1c` | `ret(0x284f8)` |
| `ldr` at 0x27d20 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x27d2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x27d38 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x27d3c | store | 4 | `sp -0x18` | `ret(0x284f8)` |
| `ldr` at 0x27d40 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27d48 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0x27d50 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x27d54 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x27d5c | load | 4 | `abs 0x27db0` | `-` |
| `ldr` at 0x27d60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27d64 | load | 4 | `abs 0x148be0` | `-` |
| `str` at 0x27d74 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x27d7c | load | 4 | `abs 0x27db4` | `-` |
| `ldr` at 0x27d80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27d88 | load | 4 | `abs 0x148bdc` | `-` |
| `str` at 0x27d9c | store | 4 | `sp -0x20` | `ret(0x284f8)` |
| `ldr` at 0x27da0 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x27da4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x27da4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x27da4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x27d14 | `(unknown callee) 0x284f8` | `?` | `0x148bdc` | `?` | `?` |
| `bl` at 0x27d94 | `(unknown callee) 0x284f8` | `?` | `0x148bdc` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x27cc0 | `[0x148bd8]:1 != 0x0` | `[0x148bd8]:1 == 0x0` |
| `bne` at 0x27cd8 | `r0 != 0xff` | `r0 == 0xff` |
| `blt` at 0x27cf4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x27d28 | `ret(0x284f8) == 0x0` | `ret(0x284f8) != 0x0` |
| `bne` at 0x27d34 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `blt` at 0x27d6c | `r0 < [0x148be0] (signed, 32-bit)` | `r0 >= [0x148be0] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x27da4 | `r0` | `?` |

## `SHUB_Reset_PreRelease`

- `SHUB_Reset_PreRelease`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4d794 to 0x4dcdc.
  Established. Included unnamed `0xeb5ec` calls carry `[sp-112]`, `0x60`, `sp-28`, `0x0`, and
  `[sp-112]`, `0x62`, `sp-28`, `0x0` (`bl` at 0x4d9cc, `bl` at 0x4da2c). Their zero-result branches
  reach `0x4da0c` and `0x4da6c`, respectively (`beq` at 0x4d9d8, `beq` at 0x4da38). A later call to
  that target uses `[sp-112]`, `0x6`, `[0x148da4]`, `0x0` (`bl` at 0x4dabc). Included four-byte
  stack assignments at `sp -0x74` retain `0xfffff883`, `0xfffff894`, `0xfffff884` and `0xfffff892`
  on different paths (`str` at 0x4d84c, `str` at 0x4da04, `str` at 0x4daf4, `str` at 0x4dc50).
  Pre-release scan semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4d7a8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x4d7b0 | store | 4 | `sp -0x18` | `0x0` |
| `ldrb` at 0x4d7b4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x4d7bc | load | 4 | `abs 0x4dc68` | `-` |
| `ldr` at 0x4d7c0 | load | 4 | `abs 0x148d8c` | `-` |
| `ldr` at 0x4d7c8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x4d7cc | store | 4 | `sp -0x70` | `?` |
| `ldrb` at 0x4d7d0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4d7e0 | store | 4 | `sp -0x18` | `ret(0x4c14c)` |
| `ldr` at 0x4d7e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4d7f0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4d7f4 | store | 4 | `sp -0x74` | `ret(0x4c14c)` |
| `ldrb` at 0x4d7fc | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4d80c | store | 4 | `sp -0x18` | `ret(0x4c560)` |
| `ldr` at 0x4d810 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4d81c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4d820 | store | 4 | `sp -0x74` | `ret(0x4c560)` |
| `ldr` at 0x4d848 | load | 4 | `abs 0x4dc6c` | `-` |
| `str` at 0x4d84c | store | 4 | `sp -0x74` | `0xfffff883` |
| `ldrb` at 0x4d854 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4d868 | store | 4 | `sp -0x18` | `ret(0x4b6cc)` |
| `ldr` at 0x4d86c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4d878 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4d87c | store | 4 | `sp -0x74` | `ret(0x4b6cc)` |
| `ldrb` at 0x4d884 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4d894 | store | 4 | `sp -0x18` | `ret(0x496c0)` |
| `ldr` at 0x4d898 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4d8a4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4d8a8 | store | 4 | `sp -0x74` | `ret(0x496c0)` |
| `ldr` at 0x4d8b0 | load | 4 | `abs 0x4dc70` | `-` |
| `ldr` at 0x4d8b4 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x4d8c0 | load | 4 | `abs 0x4dc74` | `-` |
| `str` at 0x4d8c4 | store | 4 | `sp -0x7c` | `0x10ad60` |
| `ldr` at 0x4d8cc | load | 4 | `abs 0x4dc78` | `-` |
| `ldr` at 0x4d8d0 | load | 4 | `abs 0x4dc7c` | `-` |
| `str` at 0x4d8e0 | store | 4 | `sp -0x1c` | `0x10` |
| `ldr` at 0x4d8e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d8e8 | load | 4 | `abs 0x4dc80` | `-` |
| `ldr` at 0x4d8f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d8fc | load | 4 | `abs 0x4dc80` | `-` |
| `str` at 0x4d908 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x4d90c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d910 | load | 4 | `abs 0x4dc84` | `-` |
| `ldr` at 0x4d91c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d924 | load | 4 | `abs 0x4dc84` | `-` |
| `str` at 0x4d930 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x4d934 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d938 | load | 4 | `abs 0x4dc88` | `-` |
| `ldr` at 0x4d944 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d94c | load | 4 | `abs 0x4dc88` | `-` |
| `str` at 0x4d958 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x4d95c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d96c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4d980 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x4d984 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d994 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4d9a8 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x4d9b4 | store | 4 | `sp -0x7c` | `0xa` |
| `ldr` at 0x4d9b8 | load | 4 | `abs 0x4dc8c` | `-` |
| `str` at 0x4d9bc | store | 4 | `sp -0x78` | `0x10ad74` |
| `ldr` at 0x4d9c0 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x4d9dc | load | 4 | `abs 0x4dc90` | `-` |
| `str` at 0x4d9e0 | store | 4 | `sp -0x7c` | `0x10ad84` |
| `ldrb` at 0x4d9e4 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4d9e8 | store | 4 | `sp -0x78` | `[sp-17]:1` |
| `ldr` at 0x4d9f0 | load | 4 | `abs 0x4dc78` | `-` |
| `ldr` at 0x4d9f4 | load | 4 | `abs 0x4dc94` | `-` |
| `ldr` at 0x4da00 | load | 4 | `abs 0x4dc98` | `-` |
| `str` at 0x4da04 | store | 4 | `sp -0x74` | `0xfffff894` |
| `str` at 0x4da14 | store | 4 | `sp -0x7c` | `0xa` |
| `ldr` at 0x4da18 | load | 4 | `abs 0x4dc9c` | `-` |
| `str` at 0x4da1c | store | 4 | `sp -0x78` | `0x10adb4` |
| `ldr` at 0x4da20 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x4da3c | load | 4 | `abs 0x4dca0` | `-` |
| `str` at 0x4da40 | store | 4 | `sp -0x7c` | `0x10adc4` |
| `ldrb` at 0x4da44 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4da48 | store | 4 | `sp -0x78` | `[sp-17]:1` |
| `ldr` at 0x4da50 | load | 4 | `abs 0x4dc78` | `-` |
| `ldr` at 0x4da54 | load | 4 | `abs 0x4dca4` | `-` |
| `ldr` at 0x4da60 | load | 4 | `abs 0x4dc98` | `-` |
| `str` at 0x4da64 | store | 4 | `sp -0x74` | `0xfffff894` |
| `ldr` at 0x4da6c | load | 4 | `abs 0x4dc70` | `-` |
| `ldr` at 0x4da70 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x4da7c | load | 4 | `abs 0x4dca8` | `-` |
| `str` at 0x4da80 | store | 4 | `sp -0x7c` | `0x10adf4` |
| `ldr` at 0x4da88 | load | 4 | `abs 0x4dc78` | `-` |
| `ldr` at 0x4da8c | load | 4 | `abs 0x4dcac` | `-` |
| `ldr` at 0x4da98 | load | 4 | `abs 0x4dcb0` | `-` |
| `str` at 0x4daa0 | store | 4 | `sp -0x7c` | `0x7e` |
| `ldr` at 0x4daa4 | load | 4 | `abs 0x4dcb4` | `-` |
| `str` at 0x4daa8 | store | 4 | `sp -0x78` | `0x10ae00` |
| `ldr` at 0x4daac | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x4dab4 | load | 4 | `abs 0x148da4` | `-` |
| `ldr` at 0x4dacc | load | 4 | `abs 0x4dcb8` | `-` |
| `str` at 0x4dad0 | store | 4 | `sp -0x7c` | `0x10ae08` |
| `ldrb` at 0x4dad4 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4dad8 | store | 4 | `sp -0x78` | `[sp-17]:1` |
| `ldr` at 0x4dae0 | load | 4 | `abs 0x4dc78` | `-` |
| `ldr` at 0x4dae4 | load | 4 | `abs 0x4dcbc` | `-` |
| `ldr` at 0x4daf0 | load | 4 | `abs 0x4dcc0` | `-` |
| `str` at 0x4daf4 | store | 4 | `sp -0x74` | `0xfffff884` |
| `ldr` at 0x4dafc | load | 4 | `abs 0x4dc70` | `-` |
| `ldr` at 0x4db00 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x4db0c | load | 4 | `abs 0x4dcc4` | `-` |
| `str` at 0x4db10 | store | 4 | `sp -0x7c` | `0x10ae30` |
| `ldr` at 0x4db18 | load | 4 | `abs 0x4dc78` | `-` |
| `ldr` at 0x4db1c | load | 4 | `abs 0x4dcc8` | `-` |
| `str` at 0x4db2c | store | 4 | `sp -0x1c` | `0xff` |
| `ldr` at 0x4db30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4db34 | load | 4 | `abs 0x4dc80` | `-` |
| `ldr` at 0x4db40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4db48 | load | 4 | `abs 0x4dc80` | `-` |
| `str` at 0x4db54 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x4db58 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4db5c | load | 4 | `abs 0x4dc84` | `-` |
| `ldr` at 0x4db68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4db70 | load | 4 | `abs 0x4dc84` | `-` |
| `str` at 0x4db7c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x4db80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4db84 | load | 4 | `abs 0x4dc88` | `-` |
| `ldr` at 0x4db90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4db98 | load | 4 | `abs 0x4dc88` | `-` |
| `str` at 0x4dba4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x4dba8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4dbb8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4dbcc | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x4dbd0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4dbe0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4dbf4 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x4dc00 | store | 4 | `sp -0x7c` | `0x8` |
| `ldr` at 0x4dc04 | load | 4 | `abs 0x4dccc` | `-` |
| `str` at 0x4dc08 | store | 4 | `sp -0x78` | `0x10ae40` |
| `ldr` at 0x4dc0c | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x4dc28 | load | 4 | `abs 0x4dcd0` | `-` |
| `str` at 0x4dc2c | store | 4 | `sp -0x7c` | `0x10ae4c` |
| `ldrb` at 0x4dc30 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4dc34 | store | 4 | `sp -0x78` | `[sp-17]:1` |
| `ldr` at 0x4dc3c | load | 4 | `abs 0x4dc78` | `-` |
| `ldr` at 0x4dc40 | load | 4 | `abs 0x4dcd4` | `-` |
| `ldr` at 0x4dc4c | load | 4 | `abs 0x4dcd8` | `-` |
| `str` at 0x4dc50 | store | 4 | `sp -0x74` | `0xfffff892` |
| `str` at 0x4dc5c | store | 4 | `sp -0x74` | `0x0` |
| `ldr` at 0x4dc60 | load | 4 | `sp -0x74` | `-` |
| `ldmdb` at 0x4dc64 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4dc64 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4dc64 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4d7d8 | `(unknown callee) 0x4c14c` | `(r0 & 0xff)` | `r1` | `?` | `(r0 & 0xff)` |
| `bl` at 0x4d804 | `(unknown callee) 0x4c560` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x4d838 | `(unknown callee) 0xdd3f8` | `0x1` | `0x0` | `sp-108` | `sp-108` |
| `bl` at 0x4d860 | `(unknown callee) 0x4b6cc` | `[sp-17]:1` | `0x1` | `?` | `[sp-17]:1` |
| `bl` at 0x4d88c | `(unknown callee) 0x496c0` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x4d8d8 | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x767` | `0x0` |
| `bl` at 0x4d9cc | `(unknown callee) 0xeb5ec` | `[sp-112]` | `0x60` | `sp-28` | `0x0` |
| `bl` at 0x4d9fc | `(unknown callee) 0x21eb8` | `0x3` | `0x109f8c` | `0x76d` | `0x0` |
| `bl` at 0x4da2c | `(unknown callee) 0xeb5ec` | `[sp-112]` | `0x62` | `sp-28` | `0x0` |
| `bl` at 0x4da5c | `(unknown callee) 0x21eb8` | `0x3` | `0x109f8c` | `0x772` | `0x0` |
| `bl` at 0x4da94 | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x77a` | `0x0` |
| `bl` at 0x4dabc | `(unknown callee) 0xeb5ec` | `[sp-112]` | `0x6` | `[0x148da4]` | `0x0` |
| `bl` at 0x4daec | `(unknown callee) 0x21eb8` | `0x3` | `0x109f8c` | `0x77e` | `0x0` |
| `bl` at 0x4db24 | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x786` | `0x0` |
| `bl` at 0x4dc18 | `(unknown callee) 0xeb5ec` | `[sp-112]` | `0x9` | `sp-28` | `0x0` |
| `bl` at 0x4dc48 | `(unknown callee) 0x21eb8` | `0x3` | `0x109f8c` | `0x78c` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4d7ec | `ret(0x4c14c) == 0x0` | `ret(0x4c14c) != 0x0` |
| `beq` at 0x4d818 | `ret(0x4c560) == 0x0` | `ret(0x4c560) != 0x0` |
| `bne` at 0x4d844 | `ret(0xdd3f8) != 0x0` | `ret(0xdd3f8) == 0x0` |
| `beq` at 0x4d874 | `ret(0x4b6cc) == 0x0` | `ret(0x4b6cc) != 0x0` |
| `beq` at 0x4d8a0 | `ret(0x496c0) == 0x0` | `ret(0x496c0) != 0x0` |
| `beq` at 0x4d8bc | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `beq` at 0x4d9d8 | `ret(0xeb5ec) == 0x0` | `ret(0xeb5ec) != 0x0` |
| `beq` at 0x4da38 | `ret(0xeb5ec) == 0x0` | `ret(0xeb5ec) != 0x0` |
| `beq` at 0x4da78 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `beq` at 0x4dac8 | `ret(0xeb5ec) == 0x0` | `ret(0xeb5ec) != 0x0` |
| `beq` at 0x4db08 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `beq` at 0x4dc24 | `ret(0xeb5ec) == 0x0` | `ret(0xeb5ec) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4dc64 | `r0` | `?` |

## `SHUB_Reset_PostRelease`

- `SHUB_Reset_PostRelease`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4dcdc to 0x4de84.
  Established. Two unnamed `0x47e9c` calls have `[sp-17]:1`, `0x500303a0`, `0x7`, `0x0`, and
  `[sp-17]:1`, `0x500305a0`, `0x7`, `0x0` (`bl` at 0x4dd7c, `bl` at 0x4dde0). Their nonzero results
  take branches to `0x4dd98` and `0x4de28`, respectively (`bne` at 0x4dd88, `bne` at 0x4ddec). A
  later unnamed `0x4894c` call carries `[sp-17]:1`, unknown, unknown, `[sp-17]:1` (`bl` at 0x4de30).
  Post-release register and scan meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4dcf0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x4dcf8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x4dd04 | load | 4 | `abs 0x4de58` | `-` |
| `ldrb` at 0x4dd08 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x4dd28 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x4dd34 | load | 4 | `abs 0x4de5c` | `-` |
| `ldr` at 0x4dd38 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x4dd44 | load | 4 | `abs 0x4de60` | `-` |
| `str` at 0x4dd48 | store | 4 | `sp -0x20` | `0x10ae78` |
| `ldrb` at 0x4dd4c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4dd50 | store | 4 | `sp -0x1c` | `[sp-17]:1` |
| `ldr` at 0x4dd58 | load | 4 | `abs 0x4de64` | `-` |
| `ldr` at 0x4dd5c | load | 4 | `abs 0x4de68` | `-` |
| `ldrb` at 0x4dd68 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x4dd70 | load | 4 | `abs 0x4de6c` | `-` |
| `str` at 0x4dd90 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x4dd98 | load | 4 | `abs 0x4de5c` | `-` |
| `ldr` at 0x4dd9c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x4dda8 | load | 4 | `abs 0x4de70` | `-` |
| `str` at 0x4ddac | store | 4 | `sp -0x20` | `0x10ae98` |
| `ldrb` at 0x4ddb0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4ddb4 | store | 4 | `sp -0x1c` | `[sp-17]:1` |
| `ldr` at 0x4ddbc | load | 4 | `abs 0x4de64` | `-` |
| `ldr` at 0x4ddc0 | load | 4 | `abs 0x4de74` | `-` |
| `ldrb` at 0x4ddcc | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x4ddd4 | load | 4 | `abs 0x4de78` | `-` |
| `str` at 0x4ddf4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x4ddfc | load | 4 | `abs 0x4de5c` | `-` |
| `ldr` at 0x4de00 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x4de0c | load | 4 | `abs 0x4de7c` | `-` |
| `str` at 0x4de10 | store | 4 | `sp -0x20` | `0x10aeb8` |
| `ldr` at 0x4de18 | load | 4 | `abs 0x4de64` | `-` |
| `ldrb` at 0x4de28 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4de38 | store | 4 | `sp -0x18` | `ret(0x4894c)` |
| `ldrb` at 0x4de3c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x4de40 | load | 4 | `abs 0x4de80` | `-` |
| `ldr` at 0x4de4c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x4de54 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4de54 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4de54 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4dd00 | `(unknown callee) 0x4e524` | `0x1` | `r1` | `r2` | `0x0` |
| `bl` at 0x4dd64 | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x7b5` | `0x0` |
| `bl` at 0x4dd7c | `(unknown callee) 0x47e9c` | `[sp-17]:1` | `0x500303a0` | `0x7` | `0x0` |
| `bl` at 0x4ddc8 | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x7ba` | `0x0` |
| `bl` at 0x4dde0 | `(unknown callee) 0x47e9c` | `[sp-17]:1` | `0x500305a0` | `0x7` | `0x0` |
| `bl` at 0x4de24 | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x7c0` | `0x0` |
| `bl` at 0x4de30 | `(unknown callee) 0x4894c` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x4de48 | `(unknown callee) 0x2079c` | `0x10aef8` | `[sp-17]:1` | `?` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x4dd30 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x4dd40 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `bne` at 0x4dd88 | `ret(0x47e9c) != 0x0` | `ret(0x47e9c) == 0x0` |
| `beq` at 0x4dda4 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `bne` at 0x4ddec | `ret(0x47e9c) != 0x0` | `ret(0x47e9c) == 0x0` |
| `beq` at 0x4de08 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4de54 | `r0` | `[sp-24]` |

## `SHUB_Reset_Hold`

- `SHUB_Reset_Hold`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4d770 to 0x4d794.
  Established. The only explicit store saves byte `(r0 & 0xff)` at `sp -0x11` (`strb` at 0x4d784).
  The exit carries `0x0` (`ldmdb` at 0x4d790). The span has no calls or conditional branches. Hub
  reset-hold signaling remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4d784 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldmdb` at 0x4d790 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4d790 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4d790 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4d790 | `r0` | `0x0` |

## `SHUB_Reset_MCI`

- `SHUB_Reset_MCI`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4820c to 0x4822c.
  Established. A four-byte absolute assignment writes `0x0` at `0x148da8` (`str` at 0x48220). The
  return preserves entry `r0` (`ldmdb` at 0x48224). No call or conditional branch appears. MCI state
  semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x48218 | load | 4 | `abs 0x48228` | `-` |
| `str` at 0x48220 | store | 4 | `abs 0x148da8` | `0x0` |
| `ldmdb` at 0x48224 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x48224 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x48224 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x48224 | `r0` | `r0` |

## `SHUB_Reset_FSBCfg`

- `SHUB_Reset_FSBCfg`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4867c to 0x4869c.
  Established. The body assigns four-byte zero at absolute `0x148db0` (`str` at 0x48690); its exit
  retains entry `r0` (`ldmdb` at 0x48694). There are no calls or conditional branches. Configuration
  reset effects beyond the observed assignment are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x48688 | load | 4 | `abs 0x48698` | `-` |
| `str` at 0x48690 | store | 4 | `abs 0x148db0` | `0x0` |
| `ldmdb` at 0x48694 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x48694 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x48694 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x48694 | `r0` | `r0` |

## `SHUB_Reset_WRT90`

- `SHUB_Reset_WRT90`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x48314 to 0x48334.
  Established. Four-byte `0x0` is stored at absolute `0x148dac` (`str` at 0x48328). The exit carries
  entry `r0` (`ldmdb` at 0x4832c). The call and conditional-branch lists are empty. The state
  represented by that absolute address remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x48320 | load | 4 | `abs 0x48330` | `-` |
| `str` at 0x48328 | store | 4 | `abs 0x148dac` | `0x0` |
| `ldmdb` at 0x4832c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4832c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4832c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4832c | `r0` | `r0` |

## `SHUB_Allow_Resets`

- `SHUB_Allow_Resets`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x486d8 to 0x4894c.
  Established. The unnamed `0x47e9c` calls carry unknown, `0x50000010`, `0x0`, `[sp-40]`, and
  `[sp-17]:1`, `0x50002010`, `0x0`, `[sp-40]` (`bl` at 0x48744, `bl` at 0x48774). Later unnamed
  `0x47f10` calls use `[sp-17]:1`, `0x50000010`, `sp-44`, `sp-48`, and `[sp-17]:1`, `0x50002010`,
  `sp-44`, `sp-48` (`bl` at 0x487b4, `bl` at 0x487f8). Each nonzero result takes its branch to
  `0x487d0` or `0x48814`, respectively (`bne` at 0x487c0, `bne` at 0x48804). Reset permission and
  scan-register meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x486ec | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x486f0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x486f8 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x48700 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x48704 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x48714 | store | 4 | `sp -0x28` | `?` |
| `str` at 0x4871c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x48720 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x48730 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x48738 | load | 4 | `abs 0x4892c` | `-` |
| `ldr` at 0x48740 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x48754 | load | 4 | `abs 0x48930` | `-` |
| `str` at 0x48758 | store | 4 | `sp -0x20` | `0xfffff893` |
| `ldrb` at 0x48760 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x48768 | load | 4 | `abs 0x48934` | `-` |
| `ldr` at 0x48770 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x48784 | load | 4 | `abs 0x48930` | `-` |
| `str` at 0x48788 | store | 4 | `sp -0x20` | `0xfffff893` |
| `ldr` at 0x48790 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4879c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x487ac | load | 4 | `abs 0x4892c` | `-` |
| `ldr` at 0x487c4 | load | 4 | `abs 0x48930` | `-` |
| `str` at 0x487c8 | store | 4 | `sp -0x20` | `0xfffff893` |
| `ldr` at 0x487d0 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x487e0 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x487f0 | load | 4 | `abs 0x48934` | `-` |
| `ldr` at 0x48808 | load | 4 | `abs 0x48930` | `-` |
| `str` at 0x4880c | store | 4 | `sp -0x20` | `0xfffff893` |
| `ldr` at 0x48814 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x48828 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x48830 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x48838 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x48840 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4884c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x48858 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x48864 | load | 4 | `abs 0x48938` | `-` |
| `str` at 0x48868 | store | 4 | `sp -0x38` | `0x109fa0` |
| `ldr` at 0x48870 | load | 4 | `abs 0x4893c` | `-` |
| `str` at 0x48874 | store | 4 | `sp -0x38` | `0x109fac` |
| `ldr` at 0x48878 | load | 4 | `abs 0x48940` | `-` |
| `ldr` at 0x4887c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x48880 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x48888 | load | 4 | `abs 0x48944` | `-` |
| `ldr` at 0x4888c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x48898 | load | 4 | `abs 0x48940` | `-` |
| `str` at 0x4889c | store | 4 | `sp -0x4c` | `0x109fb4` |
| `ldr` at 0x488a0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x488ac | load | 4 | `abs 0x48938` | `-` |
| `str` at 0x488b0 | store | 4 | `sp -0x3c` | `0x109fa0` |
| `ldr` at 0x488b8 | load | 4 | `abs 0x4893c` | `-` |
| `str` at 0x488bc | store | 4 | `sp -0x3c` | `0x109fac` |
| `ldr` at 0x488c0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x488c4 | store | 4 | `sp -0x48` | `[sp-60]` |
| `ldr` at 0x488c8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x488cc | store | 4 | `sp -0x44` | `[sp-28]` |
| `ldr` at 0x488d4 | load | 4 | `abs 0x48948` | `-` |
| `ldr` at 0x488e4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x488f4 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x488fc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x48908 | load | 4 | `abs 0x48930` | `-` |
| `str` at 0x4890c | store | 4 | `sp -0x40` | `0xfffff893` |
| `str` at 0x48918 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x4891c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x48920 | store | 4 | `sp -0x34` | `[sp-64]` |
| `ldr` at 0x48924 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x48928 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x48928 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x48928 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x48744 | `(unknown callee) 0x47e9c` | `?` | `0x50000010` | `0x0` | `[sp-40]` |
| `bl` at 0x48774 | `(unknown callee) 0x47e9c` | `[sp-17]:1` | `0x50002010` | `0x0` | `[sp-40]` |
| `bl` at 0x487b4 | `(unknown callee) 0x47f10` | `[sp-17]:1` | `0x50000010` | `sp-44` | `sp-48` |
| `bl` at 0x487f8 | `(unknown callee) 0x47f10` | `[sp-17]:1` | `0x50002010` | `sp-44` | `sp-48` |
| `bl` at 0x48884 | `(unknown callee) 0x2079c` | `0x109fb4` | `[sp-56]` | `?` | `?` |
| `bl` at 0x488e0 | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x1b0` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x48728 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x48750 | `ret(0x47e9c) != 0x0` | `ret(0x47e9c) == 0x0` |
| `bne` at 0x48780 | `ret(0x47e9c) != 0x0` | `ret(0x47e9c) == 0x0` |
| `bne` at 0x48798 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bne` at 0x487c0 | `ret(0x47f10) != 0x0` | `ret(0x47f10) == 0x0` |
| `bne` at 0x487d8 | `[sp-48] != 0x1` | `[sp-48] == 0x1` |
| `bne` at 0x48804 | `ret(0x47f10) != 0x0` | `ret(0x47f10) == 0x0` |
| `bne` at 0x4881c | `[sp-48] != 0x1` | `[sp-48] == 0x1` |
| `ble` at 0x48848 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x48854 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x48860 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x48894 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `beq` at 0x488a8 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0x488ec | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x48904 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x48928 | `r0` | `?` |

## `SHUB_ScanResetNI`

- `SHUB_ScanResetNI`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x48f14 to 0x490b4.
  Established. Four halfword reads have unknown effective addresses (`ldrh` at 0x48f78, `ldrh` at
  0x48fac, `ldrh` at 0x49004, `ldrh` at 0x49040). The unnamed `0x48ae8` calls use `[sp-28]`, `0x0`,
  `0x1`, unknown, and `[sp-28]`, `0x1`, `0x1`, unknown (`bl` at 0x48fc0, `bl` at 0x49054). The first
  zero result takes the branch to `0x48fe0`, and the second zero result takes its branch to
  `0x49078` (`beq` at 0x48fd4, `beq` at 0x49060). Network-interface reset effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x48f28 | store | 4 | `sp -0x14` | `0x0` |
| `str` at 0x48f30 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x48f38 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x48f3c | load | 4 | `abs 0x490ac` | `-` |
| `ldrb` at 0x48f40 | load | 1 | `abs 0x148d88` | `-` |
| `ldr` at 0x48f44 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x48f54 | load | 4 | `abs 0x490b0` | `-` |
| `ldr` at 0x48f58 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x48f78 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x48f88 | load | 4 | `abs 0x490b0` | `-` |
| `ldr` at 0x48f8c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x48fac | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x48fb4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x48fc8 | store | 4 | `sp -0x18` | `ret(0x48ae8)` |
| `ldr` at 0x48fcc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x48fd8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x48fdc | store | 4 | `sp -0x14` | `ret(0x48ae8)` |
| `ldr` at 0x48fe0 | load | 4 | `abs 0x490b0` | `-` |
| `ldr` at 0x48fe4 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x49004 | load | 2 | `(unknown)` | `-` |
| `str` at 0x49018 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x4901c | load | 4 | `abs 0x490b0` | `-` |
| `ldr` at 0x49020 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x49040 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x49048 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x49064 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x49074 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x49078 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4907c | store | 4 | `sp -0x18` | `[sp-32]` |
| `ldr` at 0x49088 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4908c | store | 4 | `sp -0x14` | `[sp-32]` |
| `ldr` at 0x49090 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x49098 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x490a0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x490a8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x490a8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x490a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x48fc0 | `(unknown callee) 0x48ae8` | `[sp-28]` | `0x0` | `0x1` | `?` |
| `bl` at 0x49054 | `(unknown callee) 0x48ae8` | `[sp-28]` | `0x1` | `0x1` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x48f4c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x48f84 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x48fd4 | `ret(0x48ae8) == 0x0` | `ret(0x48ae8) != 0x0` |
| `bne` at 0x49010 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x49060 | `ret(0x48ae8) == 0x0` | `ret(0x48ae8) != 0x0` |
| `bne` at 0x4906c | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `beq` at 0x49084 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x490a8 | `r0` | `?` |

## `SHUB_ScanResetLLP`

- `SHUB_ScanResetLLP`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x490b4 to 0x49510.
  Established. Two unnamed `0x47e9c` calls have unknown, `0x500303b0`, `0x1`, unknown, and
  `[sp-28]`, `0x500305b0`, `0x1`, unknown (`bl` at 0x49120, `bl` at 0x4914c). A later pair to that
  target carries unknown, `0x50000008`, `0x0`, `0x1`, and `[sp-28]`, `0x50002008`, `0x0`, `0x1`
  (`bl` at 0x492d0, `bl` at 0x492fc). Nonzero results from that later pair take branches to
  `0x492ec` and `0x49318` (`bne` at 0x492dc, `bne` at 0x49308). Link-reset registers and delay units
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x490c4 | load | 4 | `abs 0x494c0` | `-` |
| `ldr` at 0x490c8 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x490d4 | load | 4 | `abs 0x494c4` | `-` |
| `str` at 0x490d8 | store | 4 | `sp -0x2c` | `0x10a16c` |
| `ldr` at 0x490e0 | load | 4 | `abs 0x494c8` | `-` |
| `ldr` at 0x490e4 | load | 4 | `abs 0x494cc` | `-` |
| `str` at 0x490f4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x490f8 | load | 4 | `abs 0x494d0` | `-` |
| `ldrb` at 0x490fc | load | 1 | `abs 0x148d88` | `-` |
| `ldr` at 0x49100 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x49110 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x49114 | load | 4 | `abs 0x494d4` | `-` |
| `ldr` at 0x49130 | load | 4 | `abs 0x494d8` | `-` |
| `str` at 0x49134 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x4913c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x49140 | load | 4 | `abs 0x494dc` | `-` |
| `ldr` at 0x4915c | load | 4 | `abs 0x494d8` | `-` |
| `str` at 0x49160 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x49168 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x49170 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x49178 | load | 4 | `abs 0x494e0` | `-` |
| `str` at 0x49184 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x49188 | load | 4 | `abs 0x494d0` | `-` |
| `ldrb` at 0x4918c | load | 1 | `abs 0x148d88` | `-` |
| `ldr` at 0x49190 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x491a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x491ac | load | 4 | `abs 0x494d4` | `-` |
| `ldr` at 0x491c8 | load | 4 | `abs 0x494d8` | `-` |
| `str` at 0x491cc | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x491d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x491e4 | load | 4 | `abs 0x494e4` | `-` |
| `str` at 0x491e8 | store | 4 | `sp -0x2c` | `0x10a180` |
| `ldr` at 0x491ec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x491f0 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x491f4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x491f8 | store | 4 | `sp -0x24` | `[sp-20]` |
| `ldr` at 0x49200 | load | 4 | `abs 0x494c8` | `-` |
| `ldr` at 0x49204 | load | 4 | `abs 0x494e8` | `-` |
| `ldr` at 0x49220 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x49224 | load | 4 | `abs 0x494dc` | `-` |
| `ldr` at 0x49240 | load | 4 | `abs 0x494d8` | `-` |
| `str` at 0x49244 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x4924c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4925c | load | 4 | `abs 0x494ec` | `-` |
| `str` at 0x49260 | store | 4 | `sp -0x2c` | `0x10a1b0` |
| `ldr` at 0x49264 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x49268 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x4926c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x49270 | store | 4 | `sp -0x24` | `[sp-20]` |
| `ldr` at 0x49278 | load | 4 | `abs 0x494c8` | `-` |
| `ldr` at 0x4927c | load | 4 | `abs 0x494f0` | `-` |
| `ldr` at 0x49290 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x49298 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x492a4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x492a8 | load | 4 | `abs 0x494d0` | `-` |
| `ldrb` at 0x492ac | load | 1 | `abs 0x148d88` | `-` |
| `ldr` at 0x492b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x492c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x492e0 | load | 4 | `abs 0x494d8` | `-` |
| `str` at 0x492e4 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x492ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x492f0 | load | 4 | `abs 0x494f4` | `-` |
| `ldr` at 0x4930c | load | 4 | `abs 0x494d8` | `-` |
| `str` at 0x49310 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x49318 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x49320 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x49334 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x49338 | load | 4 | `abs 0x494d0` | `-` |
| `ldrb` at 0x4933c | load | 1 | `abs 0x148d88` | `-` |
| `ldr` at 0x49340 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x49358 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x49378 | load | 4 | `abs 0x494d8` | `-` |
| `str` at 0x4937c | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x49384 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x49390 | load | 4 | `abs 0x494f8` | `-` |
| `ldr` at 0x49394 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x493b4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x493c4 | load | 4 | `abs 0x494fc` | `-` |
| `str` at 0x493c8 | store | 4 | `sp -0x2c` | `0x10a1e0` |
| `ldr` at 0x493cc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x493d0 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x493d4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x493d8 | store | 4 | `sp -0x24` | `[sp-24]` |
| `ldr` at 0x493e0 | load | 4 | `abs 0x494c8` | `-` |
| `ldr` at 0x493e4 | load | 4 | `abs 0x49500` | `-` |
| `ldr` at 0x49400 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x49404 | load | 4 | `abs 0x49504` | `-` |
| `ldr` at 0x49420 | load | 4 | `abs 0x494d8` | `-` |
| `str` at 0x49424 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x4942c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x49438 | load | 4 | `abs 0x494f8` | `-` |
| `ldr` at 0x4943c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x4945c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x4946c | load | 4 | `abs 0x49508` | `-` |
| `str` at 0x49470 | store | 4 | `sp -0x2c` | `0x10a210` |
| `ldr` at 0x49474 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x49478 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x4947c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x49480 | store | 4 | `sp -0x24` | `[sp-24]` |
| `ldr` at 0x49488 | load | 4 | `abs 0x494c8` | `-` |
| `ldr` at 0x4948c | load | 4 | `abs 0x4950c` | `-` |
| `ldr` at 0x494a0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x494a8 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x494b4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x494b8 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x494bc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x494bc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x494bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x490ec | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x289` | `0x0` |
| `bl` at 0x49120 | `(unknown callee) 0x47e9c` | `?` | `0x500303b0` | `0x1` | `?` |
| `bl` at 0x4914c | `(unknown callee) 0x47e9c` | `[sp-28]` | `0x500305b0` | `0x1` | `?` |
| `bl` at 0x4917c | `(unknown callee) 0x4e524` | `0x1c2` | `?` | `[0x148d88]:1` | `?` |
| `bl` at 0x491b8 | `(unknown callee) 0x47f10` | `?` | `0x500303b0` | `sp-20` | `sp-24` |
| `bl` at 0x4920c | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x29b` | `0x0` |
| `bl` at 0x49214 | `(unknown callee) 0x4e524` | `0xa` | `?` | `?` | `?` |
| `bl` at 0x49230 | `(unknown callee) 0x47f10` | `[sp-28]` | `0x500305b0` | `sp-20` | `sp-24` |
| `bl` at 0x49284 | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x2a2` | `0x0` |
| `bl` at 0x4928c | `(unknown callee) 0x4e524` | `0xa` | `?` | `?` | `?` |
| `bl` at 0x492d0 | `(unknown callee) 0x47e9c` | `?` | `0x50000008` | `0x0` | `0x1` |
| `bl` at 0x492fc | `(unknown callee) 0x47e9c` | `[sp-28]` | `0x50002008` | `0x0` | `0x1` |
| `bl` at 0x4932c | `(unknown callee) 0x4e524` | `0xa` | `?` | `[0x148d88]:1` | `?` |
| `bl` at 0x49368 | `(unknown callee) 0x47f10` | `?` | `0x50000000` | `sp-20` | `sp-24` |
| `bl` at 0x493ec | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x2bb` | `0x0` |
| `bl` at 0x493f4 | `(unknown callee) 0x4e524` | `0xa` | `?` | `?` | `?` |
| `bl` at 0x49410 | `(unknown callee) 0x47f10` | `[sp-28]` | `0x50002000` | `sp-20` | `sp-24` |
| `bl` at 0x49494 | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x2c2` | `0x0` |
| `bl` at 0x4949c | `(unknown callee) 0x4e524` | `0xa` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x490d0 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `blt` at 0x49108 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x4912c | `ret(0x47e9c) != 0x0` | `ret(0x47e9c) == 0x0` |
| `bne` at 0x49158 | `ret(0x47e9c) != 0x0` | `ret(0x47e9c) == 0x0` |
| `blt` at 0x49198 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x491c4 | `ret(0x47f10) != 0x0` | `ret(0x47f10) == 0x0` |
| `bne` at 0x491e0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x4923c | `ret(0x47f10) != 0x0` | `ret(0x47f10) == 0x0` |
| `bne` at 0x49258 | `? != 0x0` | `? == 0x0` |
| `blt` at 0x492b8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x492dc | `ret(0x47e9c) != 0x0` | `ret(0x47e9c) == 0x0` |
| `bne` at 0x49308 | `ret(0x47e9c) != 0x0` | `ret(0x47e9c) == 0x0` |
| `blt` at 0x49348 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x49374 | `ret(0x47f10) != 0x0` | `ret(0x47f10) == 0x0` |
| `beq` at 0x4938c | `[sp-24] == 0x9` | `[sp-24] != 0x9` |
| `beq` at 0x493c0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x4941c | `ret(0x47f10) != 0x0` | `ret(0x47f10) == 0x0` |
| `beq` at 0x49434 | `[sp-24] == 0x9` | `[sp-24] != 0x9` |
| `beq` at 0x49468 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x494bc | `r0` | `?` |

## `SHUB_ScanSetRackSlot`

- `SHUB_ScanSetRackSlot`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4894c to 0x48ae8.
  Established. The unnamed `0x75194` call has `0x0`, `r1`, `r2`, `0x6c` (`bl` at 0x4896c). Later
  four-byte reads have unknown effective addresses (`ldr` at 0x489a8, `ldr` at 0x489bc, `ldr` at
  0x489cc, `ldr` at 0x489f0). The unnamed `0x47e9c` call carries `[sp-20]`, `0x101d0100`, `[sp-28]`,
  unknown (`bl` at 0x48a7c); its result unequal to `0x1` takes the branch to `0x48a98` (`bne` at
  0x48a88). Rack and slot field encodings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4895c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x48964 | store | 1 | `sp -0x21` | `0x6c` |
| `str` at 0x48974 | store | 4 | `sp -0x28` | `ret(0x75194)` |
| `ldr` at 0x48978 | load | 4 | `abs 0x48ac8` | `-` |
| `ldr` at 0x4897c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x48988 | load | 4 | `abs 0x48acc` | `-` |
| `str` at 0x4898c | store | 4 | `sp -0x38` | `0x109fec` |
| `ldr` at 0x48994 | load | 4 | `abs 0x48ad0` | `-` |
| `ldr` at 0x48998 | load | 4 | `abs 0x48ad4` | `-` |
| `ldr` at 0x489a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x489a8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x489b4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x489b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x489bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x489c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x489cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x489dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x489ec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x489f0 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x48a00 | load | 1 | `sp -0x21` | `-` |
| `str` at 0x48a08 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x48a0c | load | 4 | `abs 0x48ac8` | `-` |
| `ldr` at 0x48a10 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x48a1c | load | 4 | `abs 0x48ad8` | `-` |
| `str` at 0x48a20 | store | 4 | `sp -0x38` | `0x10a004` |
| `ldr` at 0x48a24 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x48a28 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0x48a2c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x48a30 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0x48a38 | load | 4 | `abs 0x48ad0` | `-` |
| `str` at 0x48a4c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x48a50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x48a60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x48a68 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x48a70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x48a74 | load | 4 | `abs 0x48adc` | `-` |
| `ldr` at 0x48a78 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x48a90 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x48a98 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x48aa0 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x48aa8 | load | 4 | `abs 0x48ae0` | `-` |
| `ldr` at 0x48aac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x48ab0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x48ab8 | load | 4 | `abs 0x48ae4` | `-` |
| `str` at 0x48abc | store | 4 | `sp -0x2c` | `0xfffff88e` |
| `ldr` at 0x48ac0 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x48ac4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x48ac4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x48ac4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4896c | `(unknown callee) 0x75194` | `0x0` | `r1` | `r2` | `0x6c` |
| `bl` at 0x489a0 | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x1cd` | `0x0` |
| `bl` at 0x48a44 | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x1d8` | `0x0` |
| `bl` at 0x48a7c | `(unknown callee) 0x47e9c` | `[sp-20]` | `0x101d0100` | `[sp-28]` | `?` |
| `bl` at 0x48ab4 | `(unknown callee) 0x2079c` | `0x10a038` | `[sp-20]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x48984 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `beq` at 0x48a18 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `ble` at 0x48a58 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x48a88 | `ret(0x47e9c) != 0x1` | `ret(0x47e9c) == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x48ac4 | `r0` | `?` |

## `portReset`

- `portReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x42028 to 0x42224.
  Established. Included byte stores assign `0x3` at `[sp-44] +0x0` in separate paths (`strb` at
  0x4210c, `strb` at 0x42138). The unnamed `0x43658` call has `[sp-24]`, `[sp-40]`, unknown, unknown
  (`bl` at 0x42144); its zero result takes the branch to `0x421f8` (`beq` at 0x42158). Four-byte
  stores later retain `0x109788`, `ret(0x1b598)`, `[sp-29]:1` and `ret(0x21a38)` at unknown
  effective addresses (`str` at 0x421d0, `str` at 0x421d4, `str` at 0x421dc, `str` at 0x421e0). Port
  reset effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x42038 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x4203c | load | 4 | `abs 0x4220c` | `-` |
| `ldr` at 0x42040 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x42054 | load | 4 | `abs 0x148d10` | `-` |
| `str` at 0x4205c | store | 4 | `sp -0x24` | `?` |
| `strb` at 0x42064 | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0x42068 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x4206c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42070 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0x42080 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x42084 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x4208c | load | 4 | `[sp-36] +0x4` | `-` |
| `ldrb` at 0x42094 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x420a0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x420a4 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x420b8 | load | 4 | `[sp-36] +0x0` | `-` |
| `str` at 0x420c0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x420c4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x420c8 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x420d0 | load | 4 | `[sp-36] +0x4` | `-` |
| `str` at 0x420d8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x420dc | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0x420e0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x420f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x42104 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0x4210c | store | 1 | `[sp-44] +0x0` | `0x3` |
| `ldr` at 0x42114 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x42118 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x42130 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0x42138 | store | 1 | `[sp-44] +0x0` | `0x3` |
| `ldr` at 0x4213c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x42140 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x4214c | store | 4 | `sp -0x1c` | `ret(0x43658)` |
| `ldr` at 0x42150 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4215c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x42164 | load | 4 | `abs 0x42210` | `-` |
| `ldr` at 0x42168 | load | 4 | `abs 0x42214` | `-` |
| `str` at 0x42194 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x42198 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x421a0 | load | 4 | `abs 0x42210` | `-` |
| `ldr` at 0x421a4 | load | 4 | `abs 0x42214` | `-` |
| `ldr` at 0x421b0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x421c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x421cc | load | 4 | `abs 0x42218` | `-` |
| `str` at 0x421d0 | store | 4 | `(unknown)` | `0x109788` |
| `str` at 0x421d4 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldrb` at 0x421d8 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x421dc | store | 4 | `(unknown)` | `[sp-29]:1` |
| `str` at 0x421e0 | store | 4 | `(unknown)` | `ret(0x21a38)` |
| `ldr` at 0x421e8 | load | 4 | `abs 0x4221c` | `-` |
| `ldr` at 0x421ec | load | 4 | `abs 0x42220` | `-` |
| `ldrb` at 0x421f8 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x42200 | store | 1 | `sp -0x1d` | `?` |
| `ldmdb` at 0x42208 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x42208 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x42208 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x42208 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x420f4 | `(unknown callee) 0x3dd74` | `?` | `[sp-36]` | `?` | `?` |
| `bl` at 0x42120 | `(unknown callee) 0x44bf4` | `?` | `?` | `0x2` | `?` |
| `bl` at 0x42144 | `(unknown callee) 0x43658` | `[sp-24]` | `[sp-40]` | `?` | `?` |
| `bl` at 0x4216c | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x10964c` | `0x109654` |
| `bl` at 0x42178 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x421a8 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x10964c` | `0x109654` |
| `bl` at 0x421b8 | `(unknown callee) 0x1b598` | `[sp-48]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x421c4 | `(unknown callee) 0x21a38` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x421f4 | `(unknown callee) 0x21eb8` | `0x3` | `0x109600` | `0x10b` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x42078 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bls` at 0x4209c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x420ec | `? == 0x0` | `? != 0x0` |
| `beq` at 0x42100 | `ret(0x3dd74) == 0x0` | `ret(0x3dd74) != 0x0` |
| `bne` at 0x4212c | `ret(0x44bf4) != 0x1` | `ret(0x44bf4) == 0x1` |
| `beq` at 0x42158 | `ret(0x43658) == 0x0` | `ret(0x43658) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x42208 | `r0` | `?` |

## `SysComReset`

- `SysComReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82fac to 0x83128.
  Established. Four-byte absolute reads include `0x2b24b0` and `0x2b24b4` (`ldr` at 0x82fcc, `ldr`
  at 0x83004). Two unnamed `0xbb164` calls and one unnamed `0xcaeb0` call each have unknown, `0x3`,
  unknown, unknown (`bl` at 0x83090, `bl` at 0x8310c, `bl` at 0x830cc). A four-byte counter update
  at `sp -0x14` retains `[sp-20]+1` (`str` at 0x83118). Communication reset effects and record
  meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x82fc0 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x82fc4 | load | 4 | `abs 0x83124` | `-` |
| `ldr` at 0x82fc8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x82fcc | load | 4 | `abs 0x2b24b0` | `-` |
| `ldr` at 0x82fdc | load | 4 | `abs 0x83124` | `-` |
| `ldr` at 0x82fe0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x83004 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x8300c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x83018 | load | 4 | `abs 0x83124` | `-` |
| `ldr` at 0x8301c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x83040 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x83048 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x83054 | load | 4 | `abs 0x83124` | `-` |
| `ldr` at 0x83058 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8307c | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x83094 | load | 4 | `abs 0x83124` | `-` |
| `ldr` at 0x83098 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x830bc | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x830c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x830d0 | load | 4 | `abs 0x83124` | `-` |
| `ldr` at 0x830d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x830f8 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x83110 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x83118 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldmdb` at 0x83120 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x83120 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x83120 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x83090 | `(unknown callee) 0xbb164` | `?` | `0x3` | `?` | `?` |
| `bl` at 0x830cc | `(unknown callee) 0xcaeb0` | `?` | `0x3` | `?` | `?` |
| `bl` at 0x8310c | `(unknown callee) 0xbb164` | `?` | `0x3` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x82fd4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x83014 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x83050 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x83120 | `r0` | `?` |

## `SysComCaptureReset`

- `SysComCaptureReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb0de4 to 0xb1168.
  Established. A nonzero `[0x14a7cc]` takes the branch to `0xb0fe0` (`bne` at 0xb0e00). The other
  arm assigns four-byte `0x1` at absolute `0x14a7cc` (`str` at 0xb0e0c). Included four-byte absolute
  stores later assign `0xb02d0` at `0x14a48c` and `0xb093c` at `0x14a490` (`str` at 0xb0fd0, `str`
  at 0xb0fdc). Those stored numbers do not prove calls. Capture-buffer and callback meanings remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xb0df4 | load | 4 | `abs 0xb114c` | `-` |
| `ldr` at 0xb0df8 | load | 4 | `abs 0x14a7cc` | `-` |
| `ldr` at 0xb0e04 | load | 4 | `abs 0xb114c` | `-` |
| `str` at 0xb0e0c | store | 4 | `abs 0x14a7cc` | `0x1` |
| `ldr` at 0xb0e10 | load | 4 | `abs 0xb1150` | `-` |
| `ldr` at 0xb0e14 | load | 4 | `abs 0x2b24a8` | `-` |
| `ldr` at 0xb0e18 | load | 4 | `[0x2b24a8] +0x10` | `-` |
| `str` at 0xb0e2c | store | 4 | `sp -0x1c` | `0x14` |
| `str` at 0xb0e38 | store | 4 | `sp -0x1c` | `0x20` |
| `str` at 0xb0e40 | store | 4 | `sp -0x1c` | `0x8` |
| `str` at 0xb0e48 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xb0e4c | load | 4 | `abs 0xb1150` | `-` |
| `ldr` at 0xb0e50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0e54 | load | 4 | `abs 0x2b24b0` | `-` |
| `ldr` at 0xb0e64 | load | 4 | `abs 0xb1150` | `-` |
| `ldr` at 0xb0e68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0e8c | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb0e94 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb0e98 | store | 4 | `(unknown)` | `[sp-28]` |
| `ldr` at 0xb0e9c | load | 4 | `abs 0xb1150` | `-` |
| `ldr` at 0xb0ea0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0ec4 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb0ecc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb0f04 | store | 4 | `(unknown)` | `ret(0x1b1b4)` |
| `ldr` at 0xb0f08 | load | 4 | `abs 0xb1150` | `-` |
| `ldr` at 0xb0f0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0f30 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb0f38 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb0f70 | store | 4 | `(unknown)` | `ret(0x1b1b4)` |
| `ldr` at 0xb0f74 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb0f7c | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0xb0f84 | load | 4 | `abs 0xb1150` | `-` |
| `ldr` at 0xb0f88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb0f8c | load | 4 | `abs 0x2b24b0` | `-` |
| `ldr` at 0xb0fbc | load | 4 | `abs 0xb1154` | `-` |
| `ldr` at 0xb0fc8 | load | 4 | `abs 0xb1158` | `-` |
| `ldr` at 0xb0fcc | load | 4 | `abs 0xb115c` | `-` |
| `str` at 0xb0fd0 | store | 4 | `abs 0x14a48c` | `0xb02d0` |
| `ldr` at 0xb0fd4 | load | 4 | `abs 0xb1160` | `-` |
| `ldr` at 0xb0fd8 | load | 4 | `abs 0xb1164` | `-` |
| `str` at 0xb0fdc | store | 4 | `abs 0x14a490` | `0xb093c` |
| `str` at 0xb0fe4 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xb0fe8 | load | 4 | `abs 0xb1150` | `-` |
| `ldr` at 0xb0fec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0ff0 | load | 4 | `abs 0x2b24b0` | `-` |
| `ldr` at 0xb1000 | load | 4 | `abs 0xb1150` | `-` |
| `ldr` at 0xb1004 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb1028 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb1030 | load | 4 | `abs 0xb1150` | `-` |
| `ldr` at 0xb1034 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb1058 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb1060 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb108c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb109c | load | 4 | `abs 0xb1150` | `-` |
| `ldr` at 0xb10a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb10c4 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb10cc | load | 4 | `abs 0xb1150` | `-` |
| `ldr` at 0xb10d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb10f4 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb10fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb1128 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb1138 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb1140 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldmdb` at 0xb1148 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb1148 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb1148 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb1148 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb0efc | `(unknown callee) 0x1b1b4` | `?` | `[sp-24]` | `[sp-28]` | `?` |
| `bl` at 0xb0f68 | `(unknown callee) 0x1b1b4` | `?` | `[sp-24]` | `[sp-28]` | `?` |
| `bl` at 0xb0fc4 | `(unknown callee) 0x2079c` | `0x128124` | `?` | `?` | `?` |
| `bl` at 0xb1098 | `(unknown callee) 0x1b4cc` | `?` | `0x0` | `?` | `?` |
| `bl` at 0xb1134 | `(unknown callee) 0x1b4cc` | `?` | `0x0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb0e00 | `[0x14a7cc] != 0x0` | `[0x14a7cc] == 0x0` |
| `beq` at 0xb0e20 | `[[0x2b24a8]+16] == 0x40` | `[[0x2b24a8]+16] != 0x40` |
| `blt` at 0xb0e5c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0xb0ff8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb1148 | `r0` | `?` |
