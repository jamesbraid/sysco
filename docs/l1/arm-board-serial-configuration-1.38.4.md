# ARM board, serial number and configuration helpers in 1.38.4

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
l1ioInit, SerNumLearn and sernumBootCheck have unresolved computed selectors. Accesses and return values cover only known control-flow edges. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/board-serial-configuration-facts.txt) retain all tool sections.

## `IP57_Nmi`

- `IP57_Nmi`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x60b48 to 0x60c24.
  Established. The opening call to `0x60f28` has `(r0 & 0xff)`, `sp-24`, `sp-24` and `(r0 & 0xff)`
  as candidates (`bl` at 0x60b70); its zero result takes the branch and nonzero falls through (`beq`
  at 0x60b84). The final r0 value is unknown (`ldmdb` at 0x60c20). NMI delivery and pulse timing
  remain unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x60b5c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x60b60 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x60b78 | store | 4 | `sp -0x20` | `ret(0x60f28)` |
| `ldr` at 0x60b7c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x60b88 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x60b8c | store | 4 | `sp -0x24` | `ret(0x60f28)` |
| `ldr` at 0x60b94 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x60b98 | load | 1 | `[sp-24] +0x2` | `-` |
| `strb` at 0x60ba0 | store | 1 | `sp -0x19` | `([[sp-24]+2]:1+2 & 0xff)` |
| `ldrb` at 0x60ba4 | load | 1 | `sp -0x19` | `-` |
| `str` at 0x60bbc | store | 4 | `sp -0x20` | `ret(0xd69d4)` |
| `ldr` at 0x60bc0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x60bcc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x60bd0 | store | 4 | `sp -0x24` | `ret(0xd69d4)` |
| `ldrb` at 0x60be0 | load | 1 | `sp -0x19` | `-` |
| `str` at 0x60bf8 | store | 4 | `sp -0x20` | `ret(0xd6988)` |
| `ldr` at 0x60bfc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x60c08 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x60c0c | store | 4 | `sp -0x24` | `ret(0xd6988)` |
| `str` at 0x60c18 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x60c1c | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x60c20 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x60c20 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x60c20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x60b70 | `(unknown callee) 0x60f28` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x60bb4 | `(unknown callee) 0xd69d4` | `0x0` | `([[sp-24]+2]:1+2 & 0xff)` | `0x10` | `([[sp-24]+2]:1+2 & 0xff)` |
| `bl` at 0x60bdc | `(unknown callee) 0x60fc8` | `0xa` | `?` | `?` | `ret(0xd69d4)` |
| `bl` at 0x60bf0 | `(unknown callee) 0xd6988` | `0x0` | `[sp-25]:1` | `0x10` | `[sp-25]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x60b84 | `ret(0x60f28) == 0x0` | `ret(0x60f28) != 0x0` |
| `beq` at 0x60bc8 | `ret(0xd69d4) == 0x0` | `ret(0xd69d4) != 0x0` |
| `beq` at 0x60c04 | `ret(0xd6988) == 0x0` | `ret(0xd6988) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x60c20 | `r0` | `?` |

## `IP57_Int`

- `IP57_Int`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x60c24 to 0x60e98.
  Established. The first call to `0x5d2d0` receives `0x0`, `[sp-24]:1`, `[sp-25]:1` and `[sp-26]:1`
  as candidates (`bl` at 0x60c9c). The signed branch retains an unknown operand against zero (`bge`
  at 0x60ca8). The return expression is unknown (`ldmdb` at 0x60e64). Processor-interrupt decoding
  remains unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x60c38 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x60c3c | load | 4 | `sp +0x4` | `-` |
| `strb` at 0x60c44 | store | 1 | `sp -0x15` | `(r0 & 0xff)` |
| `strb` at 0x60c4c | store | 1 | `sp -0x16` | `(r1 & 0xff)` |
| `strb` at 0x60c54 | store | 1 | `sp -0x17` | `(r2 & 0xff)` |
| `strb` at 0x60c5c | store | 1 | `sp -0x18` | `(r3 & 0xff)` |
| `strb` at 0x60c64 | store | 1 | `sp -0x19` | `([sp] & 0xff)` |
| `strb` at 0x60c6c | store | 1 | `sp -0x1a` | `([sp+4] & 0xff)` |
| `ldrb` at 0x60c70 | load | 1 | `sp -0x1a` | `-` |
| `ldrb` at 0x60c78 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0x60c80 | store | 1 | `sp -0x1b` | `?` |
| `ldrb` at 0x60c84 | load | 1 | `sp -0x18` | `-` |
| `ldrb` at 0x60c88 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0x60c8c | load | 1 | `sp -0x1a` | `-` |
| `ldrsb` at 0x60ca0 | load | 1 | `sp -0x1b` | `-` |
| `ldrb` at 0x60cb4 | load | 1 | `sp -0x18` | `-` |
| `str` at 0x60ccc | store | 4 | `sp -0x20` | `ret(0xd6b9c)` |
| `ldr` at 0x60cd0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x60cdc | load | 1 | `sp -0x18` | `-` |
| `ldr` at 0x60cf0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x60cfc | load | 4 | `abs 0x60e78` | `-` |
| `str` at 0x60d00 | store | 4 | `sp -0x38` | `0x10c648` |
| `str` at 0x60d04 | store | 4 | `sp -0x34` | `ret(0xd7558)` |
| `str` at 0x60d08 | store | 4 | `sp -0x30` | `ret(0x219a8)` |
| `ldr` at 0x60d10 | load | 4 | `abs 0x60e7c` | `-` |
| `ldr` at 0x60d14 | load | 4 | `abs 0x60e80` | `-` |
| `ldrb` at 0x60d24 | load | 1 | `sp -0x1a` | `-` |
| `ldrb` at 0x60d38 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x60d40 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x60d48 | load | 4 | `abs 0x60e84` | `-` |
| `ldrb` at 0x60d5c | load | 1 | `sp -0x17` | `-` |
| `str` at 0x60d74 | store | 4 | `sp -0x20` | `ret(0x52fb8)` |
| `ldr` at 0x60d78 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x60d84 | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x60d98 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x60da4 | load | 4 | `abs 0x60e88` | `-` |
| `str` at 0x60da8 | store | 4 | `sp -0x38` | `0x10c6b0` |
| `str` at 0x60dac | store | 4 | `sp -0x34` | `ret(0x53060)` |
| `str` at 0x60db0 | store | 4 | `sp -0x30` | `ret(0x219a8)` |
| `ldr` at 0x60db8 | load | 4 | `abs 0x60e7c` | `-` |
| `ldr` at 0x60dbc | load | 4 | `abs 0x60e8c` | `-` |
| `ldr` at 0x60dcc | load | 4 | `abs 0x60e90` | `-` |
| `str` at 0x60dd0 | store | 4 | `sp -0x38` | `0x10c6d4` |
| `ldrb` at 0x60dd4 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x60ddc | store | 4 | `sp -0x34` | `[sp-21]:1+48` |
| `ldrb` at 0x60de0 | load | 1 | `sp -0x16` | `-` |
| `str` at 0x60de8 | store | 4 | `sp -0x30` | `[sp-22]:1+65` |
| `ldr` at 0x60dec | load | 4 | `sp -0x24` | `-` |
| `str` at 0x60df0 | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0x60df4 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x60e10 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x60e10 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x60e2c | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x60e2c | load | 4 | `(unknown)` | `-` |
| `str` at 0x60e4c | store | 4 | `sp -0x28` | `ret(0xfb25c)` |
| `ldr` at 0x60e54 | load | 4 | `abs 0x60e7c` | `-` |
| `ldr` at 0x60e58 | load | 4 | `abs 0x60e94` | `-` |
| `ldmdb` at 0x60e64 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x60e64 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x60e64 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x60e64 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x60c9c | `(unknown callee) 0x5d2d0` | `0x0` | `[sp-24]:1` | `[sp-25]:1` | `[sp-26]:1` |
| `bl` at 0x60cb0 | `(unknown callee) 0x60fc8` | `0xa` | `?` | `?` | `?` |
| `bl` at 0x60cc4 | `(unknown callee) 0xd6b9c` | `0x0` | `[sp-24]:1` | `sp-26` | `[sp-24]:1` |
| `bl` at 0x60ce8 | `(unknown callee) 0xd7558` | `0x0` | `[sp-24]:1` | `?` | `[sp-24]:1` |
| `bl` at 0x60cf4 | `(unknown callee) 0x219a8` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x60d1c | `(unknown callee) 0x21e28` | `0x3` | `0x10c53c` | `0x494` | `0x0` |
| `bl` at 0x60d54 | `(unknown callee) 0x2070c` | `0x10c684` | `[sp-21]:1+48` | `[sp-22]:1+65` | `[sp-22]:1+65` |
| `bl` at 0x60d6c | `(unknown callee) 0x52fb8` | `0x0` | `[sp-23]:1` | `sp-36` | `[sp-23]:1` |
| `bl` at 0x60d90 | `(unknown callee) 0x53060` | `0x0` | `[sp-23]:1` | `?` | `[sp-23]:1` |
| `bl` at 0x60d9c | `(unknown callee) 0x219a8` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x60dc4 | `(unknown callee) 0x21e28` | `0x3` | `0x10c53c` | `0x49e` | `0x0` |
| `bl` at 0x60df8 | `(unknown callee) 0xfac7c` | `[sp-36]` | `?` | `?` | `[sp-36]` |
| `bl` at 0x60e14 | `(unknown callee) 0xfacf4` | `ret(0xfac7c)` | `?` | `?` | `?` |
| `bl` at 0x60e30 | `(unknown callee) 0xfa970` | `ret(0xfacf4)` | `?` | `?` | `?` |
| `bl` at 0x60e44 | `(unknown callee) 0xfb25c` | `ret(0xfa970)` | `?` | `?` | `ret(0xfa970)` |
| `bl` at 0x60e60 | `(unknown callee) 0x21e28` | `0x1` | `0x10c53c` | `0x4a1` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x60ca8 | `? >= 0x0 (signed, 32-bit)` | `? < 0x0 (signed, 32-bit)` |
| `beq` at 0x60cd8 | `ret(0xd6b9c) == 0x0` | `ret(0xd6b9c) != 0x0` |
| `bne` at 0x60d34 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x60d80 | `ret(0x52fb8) == 0x0` | `ret(0x52fb8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x60e64 | `r0` | `?` |

## `l1ioInit`

- `l1ioInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x64a1c to 0x65308.
  Established. The entry guard takes `[sp-24] != 0x0` and falls through on zero (`bne` at 0x64a80).
  The first call to `0x20c60` carries entry r0, entry r1, entry r2 and `[sp-24]` as candidates (`bl`
  at 0x64a84). The later computed selector remains unresolved (`ldrls` at 0x64b04). Hardware setup
  on selected edges is unknown beyond the listed known paths.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x64a2c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x64a34 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x64a3c | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x64a44 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0x64a4c | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x64a54 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0x64a5c | store | 4 | `sp -0x5c` | `0x0` |
| `str` at 0x64a64 | store | 4 | `sp -0x60` | `0x0` |
| `str` at 0x64a6c | store | 4 | `sp -0x64` | `0x0` |
| `str` at 0x64a74 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0x64a78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x64a94 | load | 4 | `abs 0x6528c` | `-` |
| `ldr` at 0x64a98 | load | 4 | `abs 0x65290` | `-` |
| `str` at 0x64aa4 | store | 4 | `sp -0x6c` | `ret(0x1b370)` |
| `ldr` at 0x64aa8 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x64abc | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0x64ac8 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x64ad0 | load | 4 | `abs 0x65294` | `-` |
| `ldr` at 0x64adc | load | 4 | `sp -0x6c` | `-` |
| `str` at 0x64ae8 | store | 4 | `sp -0x70` | `0x0` |
| `ldrls` at 0x64b04 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x64cf0 | load | 4 | `abs 0x652bc` | `-` |
| `str` at 0x64cf4 | store | 4 | `sp -0x80` | `0x10cfb0` |
| `ldr` at 0x64cf8 | load | 4 | `abs 0x652c0` | `-` |
| `str` at 0x64cfc | store | 4 | `sp -0x7c` | `0x10cfe8` |
| `ldr` at 0x64d04 | load | 4 | `abs 0x652c4` | `-` |
| `str` at 0x64d18 | store | 4 | `sp -0x70` | `0x0` |
| `ldr` at 0x65284 | load | 4 | `sp -0x70` | `-` |
| `ldmdb` at 0x65288 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x65288 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x65288 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x65288 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x64a84 | `(unknown callee) 0x20c60` | `r0` | `r1` | `r2` | `[sp-24]` |
| `bl` at 0x64a9c | `(unknown callee) 0x1b370` | `0x10ce34` | `0x802` | `?` | `ret(0x20c60)` |
| `bl` at 0x64ab4 | `(unknown callee) 0x1b0b8` | `ret(0x1b370)` | `?` | `?` | `ret(0x1b370)` |
| `bl` at 0x64acc | `(unknown callee) 0x1b004` | `[sp-108]` | `?` | `?` | `[ret(0x1b0b8)]` |
| `bl` at 0x64ad4 | `(unknown callee) 0x2070c` | `0x10ce44` | `?` | `?` | `?` |
| `bl` at 0x64ae0 | `(unknown callee) 0x1b004` | `?` | `?` | `?` | `?` |
| `bl` at 0x64af0 | `(unknown callee) 0x4f090` | `?` | `?` | `?` | `?` |
| `bl` at 0x64d10 | `(unknown callee) 0x21e28` | `0x4` | `0x10cff4` | `0xa3` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x64a80 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x64a90 | `ret(0x20c60) == 0x0` | `ret(0x20c60) != 0x0` |
| `bge` at 0x64ab0 | `ret(0x1b370) >= 0x0 (signed, 32-bit)` | `ret(0x1b370) < 0x0 (signed, 32-bit)` |
| `bne` at 0x64ac4 | `[ret(0x1b0b8)] != 0x13` | `[ret(0x1b0b8)] == 0x13` |
| `ldrls` at 0x64b04 | `? <= 0x51 (unsigned, 32-bit)` | `? > 0x51 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x65288 | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x64b04).

## `LpbkInit`

- `LpbkInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa3040 to 0xa3114.
  Established. The global guard takes `[0x149908] == 0x0` and falls through on nonzero (`beq` at
  0xa305c). The opening call to `0x21e28` has `0x9`, `0x124d44`, `0x2a` and `0x0` as candidates
  (`bl` at 0xa3078). The common r0 value is unknown (`ldmdb` at 0xa30f4). Loopback setup and
  ownership remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xa3050 | load | 4 | `abs 0xa30f8` | `-` |
| `ldr` at 0xa3054 | load | 4 | `abs 0x149908` | `-` |
| `ldr` at 0xa3060 | load | 4 | `abs 0xa30fc` | `-` |
| `str` at 0xa3064 | store | 4 | `sp -0x1c` | `0x124d20` |
| `ldr` at 0xa306c | load | 4 | `abs 0xa3100` | `-` |
| `ldr` at 0xa307c | load | 4 | `abs 0xa3104` | `-` |
| `ldr` at 0xa308c | load | 4 | `abs 0xa3104` | `-` |
| `ldr` at 0xa3090 | load | 4 | `abs 0xa3108` | `-` |
| `ldr` at 0xa3098 | load | 4 | `abs 0xa310c` | `-` |
| `str` at 0xa30a4 | store | 4 | `sp -0x14` | `ret(0xb4178)` |
| `ldr` at 0xa30a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa30b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa30c0 | load | 4 | `abs 0xa3110` | `-` |
| `str` at 0xa30c4 | store | 4 | `sp -0x1c` | `0x124d5c` |
| `str` at 0xa30c8 | store | 4 | `sp -0x18` | `ret(0x21674)` |
| `ldr` at 0xa30d0 | load | 4 | `abs 0xa3100` | `-` |
| `ldr` at 0xa30e0 | load | 4 | `abs 0xa30f8` | `-` |
| `ldr` at 0xa30e4 | load | 4 | `abs 0xa30f8` | `-` |
| `ldr` at 0xa30e8 | load | 4 | `abs 0x149908` | `-` |
| `str` at 0xa30f0 | store | 4 | `abs 0x149908` | `[0x149908]+1` |
| `ldmdb` at 0xa30f4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa30f4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa30f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa3078 | `(unknown callee) 0x21e28` | `0x9` | `0x124d44` | `0x2a` | `0x0` |
| `bl` at 0xa3088 | `(unknown callee) 0xb553c` | `0x2b19c0` | `0x12` | `0x0` | `?` |
| `bl` at 0xa309c | `(unknown callee) 0xb4178` | `0x2b19c0` | `0xffff012` | `0x1` | `0xa3114` |
| `bl` at 0xa30b8 | `(unknown callee) 0x21674` | `ret(0xb4178)` | `?` | `?` | `ret(0xb4178)` |
| `bl` at 0xa30dc | `(unknown callee) 0x21e28` | `0x9` | `0x124d44` | `0x38` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa305c | `[0x149908] == 0x0` | `[0x149908] != 0x0` |
| `beq` at 0xa30b0 | `ret(0xb4178) == 0x0` | `ret(0xb4178) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa30f4 | `r0` | `?` |

## `pcixcmdInit`

- `pcixcmdInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa4b38 to 0xa4bb8.
  Established. The guard takes `[0x14990c] != 0x0` and falls through on zero (`bne` at 0xa4b50). The
  call to `0xbb764` carries `0x1251ec`, `0x19`, entry r2 and `[0x14990c]` as candidates (`bl` at
  0xa4b5c). The final r0 value and command-table interpretation remain unknown (`ldmdb` at 0xa4b94).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xa4b44 | load | 4 | `abs 0xa4b98` | `-` |
| `ldr` at 0xa4b48 | load | 4 | `abs 0x14990c` | `-` |
| `ldr` at 0xa4b54 | load | 4 | `abs 0xa4b9c` | `-` |
| `ldr` at 0xa4b60 | load | 4 | `abs 0xa4ba0` | `-` |
| `ldr` at 0xa4b64 | load | 4 | `abs 0xa4ba4` | `-` |
| `str` at 0xa4b68 | store | 4 | `abs 0x1497d4` | `0x57b88` |
| `ldr` at 0xa4b6c | load | 4 | `abs 0xa4ba8` | `-` |
| `ldr` at 0xa4b70 | load | 4 | `abs 0xa4bac` | `-` |
| `str` at 0xa4b74 | store | 4 | `abs 0x1497d8` | `0x57e90` |
| `ldr` at 0xa4b78 | load | 4 | `abs 0xa4bb0` | `-` |
| `ldr` at 0xa4b7c | load | 4 | `abs 0xa4bb4` | `-` |
| `str` at 0xa4b80 | store | 4 | `abs 0x1497dc` | `0x58150` |
| `ldr` at 0xa4b84 | load | 4 | `abs 0xa4b98` | `-` |
| `str` at 0xa4b8c | store | 4 | `abs 0x14990c` | `0x1` |
| `ldmdb` at 0xa4b94 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa4b94 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa4b94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa4b5c | `(unknown callee) 0xbb764` | `0x1251ec` | `0x19` | `r2` | `[0x14990c]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa4b50 | `[0x14990c] != 0x0` | `[0x14990c] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa4b94 | `r0` | `?` |

## `SerNumLearn`

- `SerNumLearn`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xae97c to 0xaeeb4.
  Established. The opening call to `0x1f1dc` has `0xd`, entry r1, entry r2 and entry r3 as
  candidates (`bl` at 0xae998). The first comparison takes `[sp-140] == 0x52` and falls through on
  inequality (`beq` at 0xae9d0). The later selector remains unresolved (`ldrls` at 0xaeac8).
  Serial-number learning policy and merged r0 value are unknown beyond known edges.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xae98c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xae990 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xae99c | store | 4 | `sp -0x1c` | `ret(0x1f1dc)` |
| `str` at 0xae9ac | store | 4 | `sp -0x8c` | `?` |
| `str` at 0xae9b8 | store | 4 | `sp -0x90` | `ret(0x1f1dc)` |
| `str` at 0xae9c4 | store | 4 | `sp -0x94` | `ret(0x1f1dc)` |
| `ldr` at 0xae9c8 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xae9d4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xae9e0 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xae9ec | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xae9fc | load | 4 | `abs 0xaee78` | `-` |
| `ldr` at 0xaea00 | load | 4 | `abs 0x14829c` | `-` |
| `str` at 0xaea10 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xaea18 | load | 4 | `abs 0xaee78` | `-` |
| `ldr` at 0xaea1c | load | 4 | `abs 0x14829c` | `-` |
| `ldr` at 0xaea28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaea2c | load | 4 | `abs 0xaee7c` | `-` |
| `str` at 0xaea3c | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xaea44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaea4c | load | 4 | `abs 0xaee80` | `-` |
| `ldr` at 0xaea50 | load | 4 | `abs 0x149a44` | `-` |
| `ldr` at 0xaea60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaea6c | load | 4 | `abs 0xaee84` | `-` |
| `ldr` at 0xaea80 | load | 4 | `abs 0xaee80` | `-` |
| `str` at 0xaea88 | store | 4 | `abs 0x149a44` | `0x0` |
| `str` at 0xaea90 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xaea98 | load | 4 | `abs 0xaee88` | `-` |
| `ldr` at 0xaea9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaeaa4 | load | 4 | `abs 0xaee80` | `-` |
| `ldr` at 0xaeaa8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xaeaac | store | 4 | `abs 0x149a44` | `[sp-24]` |
| `ldr` at 0xaeab4 | load | 4 | `abs 0xaee88` | `-` |
| `str` at 0xaeab8 | store | 4 | `sp -0x14` | `0x2b1b48` |
| `ldr` at 0xaeabc | load | 4 | `sp -0x8c` | `-` |
| `ldrls` at 0xaeac8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xaed00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaed04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaed18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaed1c | load | 4 | `abs 0xaee94` | `-` |
| `ldr` at 0xaed30 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xaed34 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xaed40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaed58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaed68 | load | 4 | `abs 0xaee98` | `-` |
| `ldr` at 0xaed6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaed70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaed78 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xaed80 | store | 4 | `sp -0x9c` | `?` |
| `ldr` at 0xaed84 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0xaed90 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0xaed9c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0xaedac | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0xaedb8 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0xaedd0 | load | 4 | `abs 0xaee9c` | `-` |
| `ldr` at 0xaede4 | load | 4 | `abs 0xaeea0` | `-` |
| `ldr` at 0xaedf8 | load | 4 | `abs 0xaeea4` | `-` |
| `ldr` at 0xaee0c | load | 4 | `abs 0xaeea8` | `-` |
| `ldr` at 0xaee24 | load | 4 | `abs 0xaeeac` | `-` |
| `ldr` at 0xaee30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaee34 | load | 4 | `abs 0xaee7c` | `-` |
| `str` at 0xaee50 | store | 4 | `sp -0x98` | `0x1` |
| `ldr` at 0xaee58 | load | 4 | `abs 0xaeeb0` | `-` |
| `ldr` at 0xaee60 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaee6c | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xaee70 | load | 4 | `sp -0x98` | `-` |
| `ldmdb` at 0xaee74 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaee74 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaee74 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xae998 | `(unknown callee) 0x1f1dc` | `0xd` | `r1` | `r2` | `r3` |
| `bl` at 0xae9a0 | `(unknown callee) 0x4f090` | `ret(0x1f1dc)` | `?` | `?` | `?` |
| `bl` at 0xae9b4 | `(unknown callee) 0x1f1dc` | `0x1` | `?` | `?` | `?` |
| `bl` at 0xae9c0 | `(unknown callee) 0x1f1dc` | `0x2` | `?` | `?` | `?` |
| `bl` at 0xaea70 | `(unknown callee) 0x1aff8` | `[sp-20]+1` | `0x127358` | `?` | `[sp-20]+1` |
| `bl` at 0xaeaa0 | `(unknown callee) 0x1b508` | `0x2b1b48` | `[sp-20]` | `?` | `ret(0x1aff8)` |
| `bl` at 0xaed08 | `(unknown callee) 0x1aff8` | `[sp-28]` | `?` | `?` | `[sp-140]-48` |
| `bl` at 0xaed20 | `(unknown callee) 0x1aff8` | `[sp-28]` | `0x127410` | `?` | `ret(0x1aff8)` |
| `bl` at 0xaed44 | `(unknown callee) 0xaeeb4` | `[sp-20]` | `?` | `?` | `[[sp-28]]:1` |
| `bl` at 0xaed5c | `(unknown callee) 0x1d38c` | `0xd` | `[sp-20]` | `?` | `ret(0xaeeb4)` |
| `bl` at 0xaed74 | `(unknown callee) 0x1b49c` | `sp-128` | `0x12741c` | `[sp-24]` | `[sp-20]` |
| `bl` at 0xaedd4 | `(unknown callee) 0x1b2d4` | `sp-128` | `0x12745c` | `?` | `sp-128` |
| `bl` at 0xaede8 | `(unknown callee) 0x1b2d4` | `sp-128` | `0x127464` | `?` | `sp-128` |
| `bl` at 0xaedfc | `(unknown callee) 0x1b2d4` | `sp-128` | `0x127470` | `?` | `sp-128` |
| `bl` at 0xaee10 | `(unknown callee) 0x1b2d4` | `sp-128` | `0x12747c` | `?` | `sp-128` |
| `bl` at 0xaee1c | `(unknown callee) 0x20240` | `sp-128` | `?` | `?` | `sp-128` |
| `bl` at 0xaee2c | `(unknown callee) 0x23f2c` | `0x127488` | `sp-128` | `?` | `sp-128` |
| `bl` at 0xaee48 | `(unknown callee) 0x751c4` | `?` | `?` | `[sp-24]` | `?` |
| `bl` at 0xaee5c | `(unknown callee) 0x20240` | `0x12748c` | `?` | `?` | `ret(0xaeeb4)` |
| `bl` at 0xaee64 | `(unknown callee) 0x20240` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xae9d0 | `[sp-140] == 0x52` | `[sp-140] != 0x52` |
| `beq` at 0xae9dc | `[sp-140] == 0x44` | `[sp-140] != 0x44` |
| `beq` at 0xae9e8 | `[sp-140] == 0x36` | `[sp-140] != 0x36` |
| `beq` at 0xae9f4 | `[sp-140] == 0x40` | `[sp-140] != 0x40` |
| `bne` at 0xaea08 | `[0x14829c] != 0x2` | `[0x14829c] == 0x2` |
| `bne` at 0xaea24 | `[0x14829c] != 0x1` | `[0x14829c] == 0x1` |
| `bgt` at 0xaea34 | `[sp-24] > 0x3fff (signed, 32-bit)` | `[sp-24] <= 0x3fff (signed, 32-bit)` |
| `blt` at 0xaea5c | `? < ? (signed, 32-bit)` | `? >= ? (signed, 32-bit)` |
| `bne` at 0xaea7c | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |
| `ldrls` at 0xaeac8 | `[sp-140]-48 <= 0x4c (unsigned, 32-bit)` | `[sp-140]-48 > 0x4c (unsigned, 32-bit)` |
| `beq` at 0xaed14 | `ret(0x1aff8) == 0x0` | `ret(0x1aff8) != 0x0` |
| `beq` at 0xaed2c | `ret(0x1aff8) == 0x0` | `ret(0x1aff8) != 0x0` |
| `beq` at 0xaed3c | `[[sp-28]]:1 == 0x0` | `[[sp-28]]:1 != 0x0` |
| `beq` at 0xaed50 | `ret(0xaeeb4) == 0x0` | `ret(0xaeeb4) != 0x0` |
| `beq` at 0xaed8c | `[sp-156] == 0x2000` | `[sp-156] != 0x2000` |
| `bgt` at 0xaed98 | `[sp-156] > 0x2000 (signed, 32-bit)` | `[sp-156] <= 0x2000 (signed, 32-bit)` |
| `beq` at 0xaeda4 | `[sp-156] == 0x1000` | `[sp-156] != 0x1000` |
| `beq` at 0xaedb4 | `[sp-156] == 0x4000` | `[sp-156] != 0x4000` |
| `beq` at 0xaedc0 | `[sp-156] == 0x8000` | `[sp-156] != 0x8000` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaee74 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xaeac8).

## `SerNumValid`

- `SerNumValid`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaeeb4 to 0xaf134.
  Established. The initial guard takes `[0x2ae50c]:1 != 0x0` and falls through on zero (`bne` at
  0xaeed4). The call to `0x1b22c` receives entry r0, entry r1, entry r2 and `[0x2ae50c]:1` as
  candidates (`bl` at 0xaeee8). The final r0 expression and validity encoding remain unknown
  (`ldmdb` at 0xaf124).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaeec4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xaeec8 | load | 4 | `abs 0xaf128` | `-` |
| `ldrb` at 0xaeecc | load | 1 | `abs 0x2ae50c` | `-` |
| `str` at 0xaeedc | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xaeee4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaeefc | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xaef04 | load | 4 | `abs 0xaf128` | `-` |
| `ldrb` at 0xaef08 | load | 1 | `abs 0x2ae50c` | `-` |
| `strb` at 0xaef0c | store | 1 | `sp -0x1c` | `([0x2ae50c]:1 & 0xff)` |
| `ldr` at 0xaef14 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaef18 | load | 1 | `[sp-20] +0x0` | `-` |
| `str` at 0xaef38 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xaef40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaef4c | load | 4 | `abs 0xaf12c` | `-` |
| `ldr` at 0xaef60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaef6c | load | 4 | `abs 0xaf130` | `-` |
| `str` at 0xaef84 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xaef90 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaef98 | load | 1 | `[sp-20] +0x1` | `-` |
| `ldr` at 0xaefa8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaefb0 | load | 1 | `[sp-20] +0x1` | `-` |
| `ldr` at 0xaefcc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaefd4 | load | 1 | `[sp-20] +0x2` | `-` |
| `ldr` at 0xaefdc | load | 4 | `ret(0x1b4cc) +0x0` | `-` |
| `ldrh` at 0xaefe4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xaf004 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaf00c | load | 1 | `[sp-20] +0x3` | `-` |
| `ldr` at 0xaf014 | load | 4 | `ret(0x1b4cc) +0x0` | `-` |
| `ldrh` at 0xaf01c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xaf03c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaf044 | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xaf04c | load | 4 | `ret(0x1b4cc) +0x0` | `-` |
| `ldrh` at 0xaf054 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xaf074 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaf07c | load | 1 | `[sp-20] +0x5` | `-` |
| `ldr` at 0xaf084 | load | 4 | `ret(0x1b4cc) +0x0` | `-` |
| `ldrh` at 0xaf08c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xaf0ac | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaf0b4 | load | 1 | `[sp-20] +0x6` | `-` |
| `ldr` at 0xaf0bc | load | 4 | `ret(0x1b4cc) +0x0` | `-` |
| `ldrh` at 0xaf0c4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xaf0e4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaf0ec | load | 1 | `[sp-20] +0x7` | `-` |
| `ldr` at 0xaf0f4 | load | 4 | `ret(0x1b4cc) +0x0` | `-` |
| `ldrh` at 0xaf0fc | load | 2 | `(unknown)` | `-` |
| `str` at 0xaf110 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xaf11c | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xaf120 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xaf124 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaf124 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaf124 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaeee8 | `(unknown callee) 0x1b22c` | `r0` | `r1` | `r2` | `[0x2ae50c]:1` |
| `bl` at 0xaef24 | `(unknown callee) 0x1af8c` | `sp-28` | `[[sp-20]]:1` | `sp-28` | `[[sp-20]]:1` |
| `bl` at 0xaef50 | `(unknown callee) 0x1aff8` | `[sp-20]+1` | `0x1274b4` | `?` | `[sp-20]+1` |
| `bl` at 0xaef70 | `(unknown callee) 0x1aff8` | `[sp-20]+1` | `0x127360` | `?` | `[sp-20]+1` |
| `bl` at 0xaefbc | `(unknown callee) 0x1b4cc` | `ret(0x1aff8)` | `?` | `0x1` | `[[sp-20]+1]:1` |
| `bl` at 0xaeff4 | `(unknown callee) 0x1b4cc` | `ret(0x1b4cc)` | `ret(0x1b4cc)` | `?` | `?` |
| `bl` at 0xaf02c | `(unknown callee) 0x1b4cc` | `ret(0x1b4cc)` | `ret(0x1b4cc)` | `?` | `?` |
| `bl` at 0xaf064 | `(unknown callee) 0x1b4cc` | `ret(0x1b4cc)` | `ret(0x1b4cc)` | `?` | `?` |
| `bl` at 0xaf09c | `(unknown callee) 0x1b4cc` | `ret(0x1b4cc)` | `ret(0x1b4cc)` | `?` | `?` |
| `bl` at 0xaf0d4 | `(unknown callee) 0x1b4cc` | `ret(0x1b4cc)` | `ret(0x1b4cc)` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xaeed4 | `[0x2ae50c]:1 != 0x0` | `[0x2ae50c]:1 == 0x0` |
| `beq` at 0xaeef4 | `ret(0x1b22c) == 0x8` | `ret(0x1b22c) != 0x8` |
| `bne` at 0xaef30 | `ret(0x1af8c) != 0x0` | `ret(0x1af8c) == 0x0` |
| `beq` at 0xaef5c | `ret(0x1aff8) == 0x0` | `ret(0x1aff8) != 0x0` |
| `bne` at 0xaef7c | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |
| `bls` at 0xaefa0 | `[[sp-20]+1]:1 <= 0x2f (unsigned, 32-bit)` | `[[sp-20]+1]:1 > 0x2f (unsigned, 32-bit)` |
| `bhi` at 0xaefb8 | `[[sp-20]+1]:1 > 0x33 (unsigned, 32-bit)` | `[[sp-20]+1]:1 <= 0x33 (unsigned, 32-bit)` |
| `beq` at 0xaeff0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xaf028 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xaf060 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xaf098 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xaf0d0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xaf108 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaf124 | `r0` | `?` |

## `sernumInit`

- `sernumInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x50d2c to 0x51298.
  Established. The first call to `0x4f090` has entry r0, entry r1, entry r2 and `0x0` as candidates
  (`bl` at 0x50d44). The global guard takes `[0x2ae608] == 0x0` and falls through on nonzero (`beq`
  at 0x50d84). The merged r0 value and serial-number initialisation rules remain unknown (`ldmdb` at
  0x51278).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x50d40 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x50d50 | store | 4 | `sp -0x14` | `?` |
| `str` at 0x50d5c | store | 4 | `sp -0x18` | `ret(0x1f1dc)` |
| `str` at 0x50d68 | store | 4 | `sp -0x20` | `ret(0x1f1dc)` |
| `str` at 0x50d74 | store | 4 | `sp -0x24` | `ret(0x1f1dc)` |
| `ldr` at 0x50d78 | load | 4 | `abs 0x5127c` | `-` |
| `ldr` at 0x50d7c | load | 4 | `abs 0x2ae608` | `-` |
| `ldr` at 0x50d88 | load | 4 | `abs 0x5127c` | `-` |
| `ldr` at 0x50d8c | load | 4 | `abs 0x2ae608` | `-` |
| `str` at 0x50dac | store | 4 | `sp -0x28` | `ret(0x4f2f0)` |
| `ldr` at 0x50db0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x50dbc | load | 4 | `abs 0x51280` | `-` |
| `str` at 0x50dc0 | store | 4 | `sp -0xb0` | `0x10ac84` |
| `ldr` at 0x50dc8 | load | 4 | `abs 0x51284` | `-` |
| `str` at 0x50ddc | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x50de0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50dec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50df8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50e04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50e10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50e1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50e28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50e34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50e40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50e4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50e58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50e70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50e7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50e88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50e94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50ea0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50eac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50eb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50ec4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50ed0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50edc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50ee8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50ef4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50f00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50f0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50f18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50f24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50f30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50f3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50f48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50f54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50f60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50f6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50f90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50f9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50fa8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50fb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50fc4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x50fd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50fdc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50fe8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50ff4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5101c | load | 4 | `abs 0x5127c` | `-` |
| `ldrb` at 0x51020 | load | 1 | `abs 0x2ae50c` | `-` |
| `ldr` at 0x5102c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x51030 | load | 4 | `abs 0x5127c` | `-` |
| `ldrb` at 0x51034 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldrb` at 0x51038 | load | 1 | `abs 0x2ae50c` | `-` |
| `ldr` at 0x51044 | load | 4 | `abs 0x51288` | `-` |
| `ldm` at 0x5104c | load | 4 | `abs 0x10aee0` | `-` |
| `ldm` at 0x5104c | load | 4 | `abs 0x10aee4` | `-` |
| `ldm` at 0x5104c | load | 4 | `abs 0x10aee8` | `-` |
| `stm` at 0x51050 | store | 4 | `sp -0xac` | `[0x10aee0]` |
| `stm` at 0x51050 | store | 4 | `sp -0xa8` | `[0x10aee4]` |
| `strb` at 0x51054 | store | 1 | `sp -0xa4` | `([0x10aee8] & 0xff)` |
| `ldr` at 0x51058 | load | 4 | `abs 0x5127c` | `-` |
| `ldrb` at 0x5105c | load | 1 | `abs 0x2ae50c` | `-` |
| `strb` at 0x51060 | store | 1 | `sp -0xac` | `([0x2ae50c]:1 & 0xff)` |
| `ldr` at 0x51074 | load | 4 | `abs 0x5128c` | `-` |
| `ldr` at 0x5107c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x51080 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x5108c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x51090 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x510a0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x510a8 | load | 1 | `[sp-32] +0x1` | `-` |
| `ldr` at 0x510b8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x510c0 | load | 1 | `[sp-32] +0x1` | `-` |
| `ldr` at 0x510dc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x510e4 | load | 1 | `[sp-32] +0x2` | `-` |
| `ldr` at 0x510ec | load | 4 | `ret(0x1b4cc) +0x0` | `-` |
| `ldrh` at 0x510f4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x51114 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x5111c | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x51124 | load | 4 | `ret(0x1b4cc) +0x0` | `-` |
| `ldrh` at 0x5112c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x5114c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x51154 | load | 1 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x5115c | load | 4 | `ret(0x1b4cc) +0x0` | `-` |
| `ldrh` at 0x51164 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x51184 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x5118c | load | 1 | `[sp-32] +0x5` | `-` |
| `ldr` at 0x51194 | load | 4 | `ret(0x1b4cc) +0x0` | `-` |
| `ldrh` at 0x5119c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x511bc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x511c4 | load | 1 | `[sp-32] +0x6` | `-` |
| `ldr` at 0x511cc | load | 4 | `ret(0x1b4cc) +0x0` | `-` |
| `ldrh` at 0x511d4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x511f4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x511fc | load | 1 | `[sp-32] +0x7` | `-` |
| `ldr` at 0x51204 | load | 4 | `ret(0x1b4cc) +0x0` | `-` |
| `ldrh` at 0x5120c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x51220 | load | 4 | `abs 0x51290` | `-` |
| `ldr` at 0x51228 | load | 4 | `abs 0x5128c` | `-` |
| `ldr` at 0x51230 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5123c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x51240 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x51258 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x51268 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x51270 | load | 4 | `abs 0x51294` | `-` |
| `ldmdb` at 0x51278 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x51278 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x51278 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x50d44 | `(unknown callee) 0x4f090` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x50d58 | `(unknown callee) 0x1f1dc` | `0x3d` | `?` | `?` | `?` |
| `bl` at 0x50d64 | `(unknown callee) 0x1f1dc` | `0xd` | `?` | `?` | `?` |
| `bl` at 0x50d70 | `(unknown callee) 0x1f1dc` | `0xf` | `?` | `?` | `?` |
| `mov` at 0x50d94 | `(unknown callee) (unknown target)` | `ret(0x1f1dc)` | `?` | `?` | `[0x2ae608]` |
| `bl` at 0x50da4 | `(unknown callee) 0x1d38c` | `0x54` | `0x1` | `?` | `[0x2ae608]` |
| `bl` at 0x50da8 | `(unknown callee) 0x4f2f0` | `ret(0x1d38c)` | `?` | `?` | `?` |
| `bl` at 0x50dd4 | `(unknown callee) 0x21e28` | `0x1` | `0x10acac` | `0x300` | `0x0` |
| `bl` at 0x50e6c | `(unknown callee) 0x1d38c` | `0x50` | `0x0` | `?` | `[sp-20]` |
| `bl` at 0x50f80 | `(unknown callee) 0x1f1dc` | `0x50` | `?` | `?` | `[sp-20]` |
| `bl` at 0x51008 | `(unknown callee) 0x1f1dc` | `0x3e` | `?` | `?` | `[sp-20]` |
| `bl` at 0x51070 | `(unknown callee) 0x1d38c` | `0xd` | `sp-172` | `[0x10aee8]` | `sp-172` |
| `bl` at 0x51078 | `(unknown callee) 0x20240` | `0x10aeec` | `?` | `?` | `?` |
| `bl` at 0x510cc | `(unknown callee) 0x1b4cc` | `?` | `?` | `0x1` | `[[sp-32]+1]:1` |
| `bl` at 0x51104 | `(unknown callee) 0x1b4cc` | `ret(0x1b4cc)` | `ret(0x1b4cc)` | `?` | `?` |
| `bl` at 0x5113c | `(unknown callee) 0x1b4cc` | `ret(0x1b4cc)` | `ret(0x1b4cc)` | `?` | `?` |
| `bl` at 0x51174 | `(unknown callee) 0x1b4cc` | `ret(0x1b4cc)` | `ret(0x1b4cc)` | `?` | `?` |
| `bl` at 0x511ac | `(unknown callee) 0x1b4cc` | `ret(0x1b4cc)` | `ret(0x1b4cc)` | `?` | `?` |
| `bl` at 0x511e4 | `(unknown callee) 0x1b4cc` | `ret(0x1b4cc)` | `ret(0x1b4cc)` | `?` | `?` |
| `bl` at 0x51224 | `(unknown callee) 0x1d38c` | `0xd` | `0x10af08` | `?` | `?` |
| `bl` at 0x5122c | `(unknown callee) 0x20240` | `0x10aeec` | `?` | `?` | `?` |
| `bl` at 0x51248 | `(unknown callee) 0x1b250` | `[sp-36]` | `[sp-40]` | `0x6` | `[sp-40]` |
| `bl` at 0x5126c | `(unknown callee) 0x1d38c` | `0xf` | `[sp-40]` | `?` | `[sp-28]` |
| `bl` at 0x51274 | `(unknown callee) 0x20240` | `0x10af14` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x50d84 | `[0x2ae608] == 0x0` | `[0x2ae608] != 0x0` |
| `bne` at 0x50db8 | `ret(0x4f2f0) != 0x0` | `ret(0x4f2f0) == 0x0` |
| `beq` at 0x50de8 | `[sp-20] == 0x52` | `[sp-20] != 0x52` |
| `beq` at 0x50df4 | `[sp-20] == 0x44` | `[sp-20] != 0x44` |
| `beq` at 0x50e00 | `[sp-20] == 0x36` | `[sp-20] != 0x36` |
| `beq` at 0x50e0c | `[sp-20] == 0x35` | `[sp-20] != 0x35` |
| `beq` at 0x50e18 | `[sp-20] == 0x40` | `[sp-20] != 0x40` |
| `beq` at 0x50e24 | `[sp-20] == 0x4f` | `[sp-20] != 0x4f` |
| `beq` at 0x50e30 | `[sp-20] == 0x46` | `[sp-20] != 0x46` |
| `beq` at 0x50e3c | `[sp-20] == 0x48` | `[sp-20] != 0x48` |
| `beq` at 0x50e48 | `[sp-20] == 0x42` | `[sp-20] != 0x42` |
| `beq` at 0x50e54 | `[sp-20] == 0x4c` | `[sp-20] != 0x4c` |
| `beq` at 0x50e60 | `[sp-20] == 0x4d` | `[sp-20] != 0x4d` |
| `beq` at 0x50e78 | `[sp-20] == 0x52` | `[sp-20] != 0x52` |
| `beq` at 0x50e84 | `[sp-20] == 0x44` | `[sp-20] != 0x44` |
| `beq` at 0x50e90 | `[sp-20] == 0x36` | `[sp-20] != 0x36` |
| `beq` at 0x50e9c | `[sp-20] == 0x35` | `[sp-20] != 0x35` |
| `beq` at 0x50ea8 | `[sp-20] == 0x40` | `[sp-20] != 0x40` |
| `beq` at 0x50eb4 | `[sp-20] == 0x4f` | `[sp-20] != 0x4f` |
| `beq` at 0x50ec0 | `[sp-20] == 0x46` | `[sp-20] != 0x46` |
| `beq` at 0x50ecc | `[sp-20] == 0x48` | `[sp-20] != 0x48` |
| `beq` at 0x50ed8 | `[sp-20] == 0x42` | `[sp-20] != 0x42` |
| `beq` at 0x50ee4 | `[sp-20] == 0x4c` | `[sp-20] != 0x4c` |
| `bne` at 0x50ef0 | `[sp-20] != 0x4d` | `[sp-20] == 0x4d` |
| `beq` at 0x50efc | `[sp-20] == 0x52` | `[sp-20] != 0x52` |
| `beq` at 0x50f08 | `[sp-20] == 0x44` | `[sp-20] != 0x44` |
| `beq` at 0x50f14 | `[sp-20] == 0x36` | `[sp-20] != 0x36` |
| `beq` at 0x50f20 | `[sp-20] == 0x35` | `[sp-20] != 0x35` |
| `beq` at 0x50f2c | `[sp-20] == 0x40` | `[sp-20] != 0x40` |
| `beq` at 0x50f38 | `[sp-20] == 0x4f` | `[sp-20] != 0x4f` |
| `beq` at 0x50f44 | `[sp-20] == 0x46` | `[sp-20] != 0x46` |
| `beq` at 0x50f50 | `[sp-20] == 0x48` | `[sp-20] != 0x48` |
| `beq` at 0x50f5c | `[sp-20] == 0x42` | `[sp-20] != 0x42` |
| `beq` at 0x50f68 | `[sp-20] == 0x4c` | `[sp-20] != 0x4c` |
| `beq` at 0x50f74 | `[sp-20] == 0x4d` | `[sp-20] != 0x4d` |
| `bne` at 0x50f8c | `ret(0x1f1dc) != 0x0` | `ret(0x1f1dc) == 0x0` |
| `beq` at 0x50f98 | `[sp-20] == 0x52` | `[sp-20] != 0x52` |
| `beq` at 0x50fa4 | `[sp-20] == 0x44` | `[sp-20] != 0x44` |
| `beq` at 0x50fb0 | `[sp-20] == 0x36` | `[sp-20] != 0x36` |
| `beq` at 0x50fbc | `[sp-20] == 0x40` | `[sp-20] != 0x40` |
| `beq` at 0x50fcc | `[sp-24] == 0x1` | `[sp-24] != 0x1` |
| `beq` at 0x50fd8 | `[sp-20] == 0x52` | `[sp-20] != 0x52` |
| `beq` at 0x50fe4 | `[sp-20] == 0x44` | `[sp-20] != 0x44` |
| `beq` at 0x50ff0 | `[sp-20] == 0x36` | `[sp-20] != 0x36` |
| `beq` at 0x50ffc | `[sp-20] == 0x40` | `[sp-20] != 0x40` |
| `bne` at 0x51014 | `ret(0x1f1dc) != 0x0` | `ret(0x1f1dc) == 0x0` |
| `beq` at 0x51028 | `[0x2ae50c]:1 == 0x0` | `[0x2ae50c]:1 != 0x0` |
| `beq` at 0x51040 | `[[sp-32]]:1 == [0x2ae50c]:1` | `[[sp-32]]:1 != [0x2ae50c]:1` |
| `bls` at 0x51088 | `[[sp-32]]:1 <= 0x4b (unsigned, 32-bit)` | `[[sp-32]]:1 > 0x4b (unsigned, 32-bit)` |
| `bhi` at 0x51098 | `[[sp-32]]:1 > 0x52 (unsigned, 32-bit)` | `[[sp-32]]:1 <= 0x52 (unsigned, 32-bit)` |
| `bls` at 0x510b0 | `[[sp-32]+1]:1 <= 0x2f (unsigned, 32-bit)` | `[[sp-32]+1]:1 > 0x2f (unsigned, 32-bit)` |
| `bhi` at 0x510c8 | `[[sp-32]+1]:1 > 0x33 (unsigned, 32-bit)` | `[[sp-32]+1]:1 <= 0x33 (unsigned, 32-bit)` |
| `beq` at 0x51100 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x51138 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x51170 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x511a8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x511e0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x51218 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x51238 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `beq` at 0x51254 | `ret(0x1b250) == 0x0` | `ret(0x1b250) != 0x0` |
| `bne` at 0x51260 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x51278 | `r0` | `?` |

## `sernumBootCheck`

- `sernumBootCheck`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x503e0 to 0x50960.
  Established. The opening call to `0x1f1dc` carries `0xd`, entry r1, entry r2 and `0x0` as
  candidates (`bl` at 0x503fc). The first branch takes `ret(0x1f1dc) == 0x0` and falls through on
  nonzero (`beq` at 0x50420). The later selector remains unresolved (`ldrls` at 0x50548). Boot
  acceptance and serial-number classifications remain unknown beyond known edges.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x503f4 | store | 4 | `sp -0x14` | `0x0` |
| `str` at 0x50400 | store | 4 | `sp -0x18` | `ret(0x1f1dc)` |
| `str` at 0x5040c | store | 4 | `sp -0x20` | `ret(0x1f1dc)` |
| `str` at 0x50428 | store | 4 | `sp -0xc4` | `0x0` |
| `str` at 0x50434 | store | 4 | `sp -0x1c` | `ret(0x4f2f0)` |
| `ldr` at 0x50438 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x50444 | load | 4 | `abs 0x508e4` | `-` |
| `str` at 0x50448 | store | 4 | `sp -0xd0` | `0x10ac84` |
| `ldr` at 0x50450 | load | 4 | `abs 0x508e8` | `-` |
| `ldr` at 0x50454 | load | 4 | `abs 0x508ec` | `-` |
| `str` at 0x50464 | store | 4 | `sp -0xc4` | `0x0` |
| `ldr` at 0x5046c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x50470 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x50488 | load | 4 | `abs 0x508f0` | `-` |
| `ldr` at 0x50490 | load | 4 | `abs 0x508f4` | `-` |
| `ldr` at 0x50494 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x50498 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x504a0 | load | 4 | `abs 0x508f8` | `-` |
| `str` at 0x504a4 | store | 4 | `sp -0xc4` | `0xfffff95c` |
| `ldr` at 0x504ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x504b8 | load | 4 | `abs 0x508fc` | `-` |
| `ldr` at 0x504cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x504d8 | load | 4 | `abs 0x50900` | `-` |
| `ldr` at 0x504ec | load | 4 | `abs 0x50904` | `-` |
| `ldr` at 0x504f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x504f8 | load | 4 | `abs 0x50908` | `-` |
| `str` at 0x504fc | store | 4 | `sp -0xc4` | `0xfffff95a` |
| `ldr` at 0x50504 | load | 4 | `abs 0x5090c` | `-` |
| `ldrb` at 0x50508 | load | 1 | `abs 0x2ae50c` | `-` |
| `ldr` at 0x50514 | load | 4 | `abs 0x5090c` | `-` |
| `ldr` at 0x50518 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5051c | load | 1 | `abs 0x2ae50c` | `-` |
| `ldrb` at 0x50520 | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0x50530 | store | 4 | `sp -0x14` | `0x1` |
| `ldr` at 0x50538 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5053c | load | 1 | `[sp-24] +0x0` | `-` |
| `ldrls` at 0x50548 | load | 4 | `(unknown)` | `-` |
| `str` at 0x50578 | store | 4 | `sp -0x14` | `0xa` |
| `ldr` at 0x50580 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x50588 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0x50598 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x505a0 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0x505bc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x505c4 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0x505cc | load | 4 | `ret(0x1b4cc) +0x0` | `-` |
| `ldrh` at 0x505d4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x505f4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x505fc | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0x50604 | load | 4 | `ret(0x1b4cc) +0x0` | `-` |
| `ldrh` at 0x5060c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x5062c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x50634 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x5063c | load | 4 | `ret(0x1b4cc) +0x0` | `-` |
| `ldrh` at 0x50644 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x50664 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5066c | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0x50674 | load | 4 | `ret(0x1b4cc) +0x0` | `-` |
| `ldrh` at 0x5067c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x5069c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x506a4 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0x506ac | load | 4 | `ret(0x1b4cc) +0x0` | `-` |
| `ldrh` at 0x506b4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x506d4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x506dc | load | 1 | `[sp-24] +0x7` | `-` |
| `ldr` at 0x506e4 | load | 4 | `ret(0x1b4cc) +0x0` | `-` |
| `ldrh` at 0x506ec | load | 2 | `(unknown)` | `-` |
| `str` at 0x50700 | store | 4 | `sp -0x14` | `0xb` |
| `ldr` at 0x50704 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50710 | load | 4 | `abs 0x50910` | `-` |
| `ldr` at 0x50718 | load | 4 | `abs 0x50914` | `-` |
| `ldr` at 0x5071c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x50724 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x50728 | store | 4 | `sp -0xc8` | `[sp-20]` |
| `ldr` at 0x5072c | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x50738 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x50744 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x50754 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x50764 | load | 4 | `abs 0x5090c` | `-` |
| `ldrb` at 0x50768 | load | 1 | `abs 0x2ae50c` | `-` |
| `ldr` at 0x5076c | load | 4 | `abs 0x50918` | `-` |
| `ldr` at 0x5077c | load | 4 | `abs 0x5091c` | `-` |
| `ldr` at 0x50788 | load | 4 | `abs 0x50920` | `-` |
| `ldr` at 0x50794 | load | 4 | `abs 0x50924` | `-` |
| `ldr` at 0x5079c | load | 4 | `abs 0x50928` | `-` |
| `str` at 0x507a0 | store | 4 | `sp -0xc4` | `0xfffff959` |
| `ldr` at 0x507a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x507ac | load | 4 | `abs 0x5092c` | `-` |
| `ldr` at 0x507c0 | load | 4 | `abs 0x50930` | `-` |
| `ldr` at 0x507d0 | load | 4 | `abs 0x50934` | `-` |
| `ldr` at 0x507d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x507d8 | load | 4 | `abs 0x5092c` | `-` |
| `ldr` at 0x507e0 | load | 4 | `abs 0x50938` | `-` |
| `ldr` at 0x507e4 | load | 4 | `abs 0x149a44` | `-` |
| `str` at 0x507ec | store | 4 | `sp -0xcc` | `?` |
| `ldr` at 0x507f0 | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0x507fc | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0x50808 | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0x50818 | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0x50830 | load | 4 | `abs 0x5093c` | `-` |
| `ldr` at 0x50844 | load | 4 | `abs 0x50940` | `-` |
| `ldr` at 0x50858 | load | 4 | `abs 0x50944` | `-` |
| `ldr` at 0x5086c | load | 4 | `abs 0x50948` | `-` |
| `str` at 0x50870 | store | 4 | `sp -0xc4` | `0xfffff95b` |
| `ldr` at 0x50878 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x50884 | load | 4 | `abs 0x5094c` | `-` |
| `ldr` at 0x50898 | load | 4 | `abs 0x50950` | `-` |
| `ldr` at 0x5089c | load | 4 | `abs 0x5094c` | `-` |
| `ldr` at 0x508b0 | load | 4 | `abs 0x50954` | `-` |
| `ldr` at 0x508b8 | load | 4 | `abs 0x50958` | `-` |
| `ldr` at 0x508bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x508c0 | load | 4 | `abs 0x5092c` | `-` |
| `ldr` at 0x508c8 | load | 4 | `abs 0x5095c` | `-` |
| `str` at 0x508cc | store | 4 | `sp -0xc4` | `0xfffff958` |
| `str` at 0x508d8 | store | 4 | `sp -0xc4` | `0x0` |
| `ldr` at 0x508dc | load | 4 | `sp -0xc4` | `-` |
| `ldmdb` at 0x508e0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x508e0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x508e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x503fc | `(unknown callee) 0x1f1dc` | `0xd` | `r1` | `r2` | `0x0` |
| `bl` at 0x50408 | `(unknown callee) 0x1f1dc` | `0xf` | `?` | `?` | `?` |
| `bl` at 0x50414 | `(unknown callee) 0x1f1dc` | `0x50` | `?` | `?` | `?` |
| `bl` at 0x50430 | `(unknown callee) 0x4f2f0` | `ret(0x1f1dc)` | `?` | `?` | `ret(0x1f1dc)` |
| `bl` at 0x5045c | `(unknown callee) 0x21e28` | `0x1` | `0x10acac` | `0x22e` | `0x0` |
| `bl` at 0x50478 | `(unknown callee) 0x1b250` | `[sp-32]` | `ret(0x4f2f0)` | `0x6` | `ret(0x4f2f0)` |
| `bl` at 0x5048c | `(unknown callee) 0x20240` | `0x10acc4` | `?` | `?` | `ret(0x1b250)` |
| `bl` at 0x5049c | `(unknown callee) 0x2070c` | `0x10ace4` | `[sp-32]` | `[sp-28]` | `?` |
| `bl` at 0x504bc | `(unknown callee) 0x1aff8` | `[sp-24]+1` | `0x10ad08` | `?` | `[sp-24]+1` |
| `bl` at 0x504dc | `(unknown callee) 0x1aff8` | `[sp-24]+1` | `0x10ad10` | `?` | `[sp-24]+1` |
| `bl` at 0x504f4 | `(unknown callee) 0x2070c` | `0x10ad18` | `[sp-24]` | `?` | `ret(0x1aff8)` |
| `bl` at 0x505ac | `(unknown callee) 0x1b4cc` | `ret(0x1aff8)` | `?` | `0x1` | `[[sp-24]+1]:1` |
| `bl` at 0x505e4 | `(unknown callee) 0x1b4cc` | `ret(0x1b4cc)` | `ret(0x1b4cc)` | `?` | `?` |
| `bl` at 0x5061c | `(unknown callee) 0x1b4cc` | `ret(0x1b4cc)` | `ret(0x1b4cc)` | `?` | `?` |
| `bl` at 0x50654 | `(unknown callee) 0x1b4cc` | `ret(0x1b4cc)` | `ret(0x1b4cc)` | `?` | `?` |
| `bl` at 0x5068c | `(unknown callee) 0x1b4cc` | `ret(0x1b4cc)` | `ret(0x1b4cc)` | `?` | `?` |
| `bl` at 0x506c4 | `(unknown callee) 0x1b4cc` | `ret(0x1b4cc)` | `ret(0x1b4cc)` | `?` | `?` |
| `bl` at 0x50714 | `(unknown callee) 0x20240` | `0x10ad38` | `?` | `?` | `?` |
| `bl` at 0x50720 | `(unknown callee) 0x2070c` | `0x10ad50` | `[sp-24]` | `?` | `?` |
| `bl` at 0x50774 | `(unknown callee) 0x2070c` | `0x10ad5c` | `[0x2ae50c]:1` | `?` | `[0x2ae50c]:1` |
| `bl` at 0x50780 | `(unknown callee) 0x2070c` | `0x10ad7c` | `?` | `?` | `[sp-20]` |
| `bl` at 0x5078c | `(unknown callee) 0x2070c` | `0x10adb8` | `?` | `?` | `[sp-20]` |
| `bl` at 0x50798 | `(unknown callee) 0x2070c` | `0x10adf8` | `?` | `?` | `[sp-20]` |
| `bl` at 0x507b0 | `(unknown callee) 0x1aff8` | `[sp-24]` | `0x2b1b48` | `?` | `?` |
| `bl` at 0x507c4 | `(unknown callee) 0x20240` | `0x10ae20` | `?` | `?` | `ret(0x1aff8)` |
| `bl` at 0x507dc | `(unknown callee) 0x1b49c` | `sp-112` | `0x10ae40` | `[sp-24]` | `0x2b1b48` |
| `bl` at 0x50834 | `(unknown callee) 0x1b2d4` | `sp-112` | `0x10ae68` | `?` | `sp-112` |
| `bl` at 0x50848 | `(unknown callee) 0x1b2d4` | `sp-112` | `0x10ae7c` | `?` | `sp-112` |
| `bl` at 0x5085c | `(unknown callee) 0x1b2d4` | `sp-112` | `0x10ae8c` | `?` | `sp-112` |
| `bl` at 0x50868 | `(unknown callee) 0x20240` | `sp-112` | `?` | `?` | `sp-112` |
| `bl` at 0x50888 | `(unknown callee) 0x1aff8` | `[sp-24]+1` | `0x10ae9c` | `?` | `[sp-24]+1` |
| `bl` at 0x508a0 | `(unknown callee) 0x1aff8` | `0x2b1b49` | `0x10ae9c` | `?` | `ret(0x1aff8)` |
| `bl` at 0x508b4 | `(unknown callee) 0x20240` | `0x10aea4` | `?` | `?` | `ret(0x1aff8)` |
| `bl` at 0x508c4 | `(unknown callee) 0x2070c` | `0x10aeb8` | `[sp-24]` | `0x2b1b48` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x50420 | `ret(0x1f1dc) == 0x0` | `ret(0x1f1dc) != 0x0` |
| `bne` at 0x50440 | `ret(0x4f2f0) != 0x0` | `ret(0x4f2f0) == 0x0` |
| `beq` at 0x50484 | `ret(0x1b250) == 0x0` | `ret(0x1b250) != 0x0` |
| `beq` at 0x504c8 | `ret(0x1aff8) == 0x0` | `ret(0x1aff8) != 0x0` |
| `bne` at 0x504e8 | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |
| `beq` at 0x50510 | `[0x2ae50c]:1 == 0x0` | `[0x2ae50c]:1 != 0x0` |
| `beq` at 0x50528 | `[0x2ae50c]:1 == [[sp-24]]:1` | `[0x2ae50c]:1 != [[sp-24]]:1` |
| `ldrls` at 0x50548 | `[[sp-24]]:1-76 <= 0x8 (unsigned, 32-bit)` | `[[sp-24]]:1-76 > 0x8 (unsigned, 32-bit)` |
| `bls` at 0x50590 | `[[sp-24]+1]:1 <= 0x2f (unsigned, 32-bit)` | `[[sp-24]+1]:1 > 0x2f (unsigned, 32-bit)` |
| `bhi` at 0x505a8 | `[[sp-24]+1]:1 > 0x33 (unsigned, 32-bit)` | `[[sp-24]+1]:1 <= 0x33 (unsigned, 32-bit)` |
| `beq` at 0x505e0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x50618 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x50650 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x50688 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x506c0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x506f8 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x5070c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x50734 | `[sp-20] == 0xa` | `[sp-20] != 0xa` |
| `bgt` at 0x50740 | `[sp-20] > 0xa (signed, 32-bit)` | `[sp-20] <= 0xa (signed, 32-bit)` |
| `beq` at 0x5074c | `[sp-20] == 0x1` | `[sp-20] != 0x1` |
| `beq` at 0x5075c | `[sp-20] == 0xb` | `[sp-20] != 0xb` |
| `beq` at 0x507bc | `ret(0x1aff8) == 0x0` | `ret(0x1aff8) != 0x0` |
| `beq` at 0x507f8 | `[sp-204] == 0x2000` | `[sp-204] != 0x2000` |
| `bgt` at 0x50804 | `[sp-204] > 0x2000 (signed, 32-bit)` | `[sp-204] <= 0x2000 (signed, 32-bit)` |
| `beq` at 0x50810 | `[sp-204] == 0x1000` | `[sp-204] != 0x1000` |
| `beq` at 0x50820 | `[sp-204] == 0x4000` | `[sp-204] != 0x4000` |
| `bne` at 0x50894 | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |
| `bne` at 0x508ac | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x508e0 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x50548).

## `KeyLogInsert`

- `KeyLogInsert`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x50960 to 0x50b54.
  Established. The first call to `0x1f1dc` has `0x51`, entry r1, entry r2 and `sp-28` as candidates
  (`bl` at 0x5097c). The return records `0x0` (`ldmdb` at 0x50b4c). The log record format, wrap rule
  and storage ownership remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `stm` at 0x50974 | store | 4 | `sp -0x1c` | `r0` |
| `stm` at 0x50974 | store | 4 | `sp -0x18` | `r1` |
| `stm` at 0x50974 | store | 4 | `sp -0x14` | `r2` |
| `strb` at 0x50984 | store | 1 | `sp -0x1d` | `(ret(0x1f1dc) & 0xff)` |
| `str` at 0x50990 | store | 4 | `sp -0x28` | `ret(0x1f1dc)` |
| `str` at 0x5099c | store | 4 | `sp -0x24` | `ret(0x1f1dc)` |
| `ldrb` at 0x509a0 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x509b0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x509b8 | store | 4 | `sp -0x28` | `?` |
| `ldrb` at 0x509bc | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x509cc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x509d4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x509dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x509e0 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x509f8 | store | 1 | `[sp-36] +0x0` | `?` |
| `str` at 0x50a00 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x50a08 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x50a0c | load | 1 | `sp -0x1c` | `-` |
| `strb` at 0x50a1c | store | 1 | `[sp-36] +0x1` | `?` |
| `str` at 0x50a24 | store | 4 | `sp -0x24` | `[sp-36]+2` |
| `ldr` at 0x50a2c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x50a30 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x50a48 | store | 1 | `[sp-36] +0x2` | `?` |
| `str` at 0x50a50 | store | 4 | `sp -0x24` | `[sp-36]+3` |
| `ldr` at 0x50a58 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x50a5c | load | 1 | `sp -0x18` | `-` |
| `strb` at 0x50a6c | store | 1 | `[sp-36] +0x3` | `?` |
| `str` at 0x50a74 | store | 4 | `sp -0x24` | `[sp-36]+4` |
| `ldr` at 0x50a7c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x50a80 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x50a98 | store | 1 | `[sp-36] +0x4` | `?` |
| `str` at 0x50aa0 | store | 4 | `sp -0x24` | `[sp-36]+5` |
| `ldr` at 0x50aa8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x50aac | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x50ac4 | store | 1 | `[sp-36] +0x5` | `?` |
| `str` at 0x50acc | store | 4 | `sp -0x24` | `[sp-36]+6` |
| `ldr` at 0x50ad4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x50ad8 | load | 1 | `sp -0x14` | `-` |
| `strb` at 0x50ae8 | store | 1 | `[sp-36] +0x6` | `?` |
| `str` at 0x50af0 | store | 4 | `sp -0x24` | `[sp-36]+7` |
| `ldrb` at 0x50af4 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x50afc | load | 4 | `abs 0x50b50` | `-` |
| `strb` at 0x50b24 | store | 1 | `sp -0x1d` | `?` |
| `ldrb` at 0x50b28 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x50b3c | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x50b4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x50b4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x50b4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5097c | `(unknown callee) 0x1f1dc` | `0x51` | `r1` | `r2` | `sp-28` |
| `bl` at 0x5098c | `(unknown callee) 0x1f1dc` | `0x52` | `?` | `?` | `ret(0x1f1dc)` |
| `bl` at 0x50998 | `(unknown callee) 0x1f1dc` | `0x53` | `?` | `?` | `?` |
| `bl` at 0x50b34 | `(unknown callee) 0x1d38c` | `0x51` | `[sp-29]:1` | `?` | `[sp-29]:1` |
| `bl` at 0x50b40 | `(unknown callee) 0x1d38c` | `0x52` | `[sp-40]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x50b4c | `r0` | `0x0` |

## `KeyInLog`

- `KeyInLog`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x50b54 to 0x50d2c.
  Established. The opening call targets `0x1f1dc` with `0x52`, entry r1, entry r2 and `sp-28` as
  candidates (`bl` at 0x50b70). The loop comparison retains unknown operands (`ble` at 0x50b94). The
  final r0 expression and key-match encoding remain unknown (`ldmdb` at 0x50d28).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `stm` at 0x50b68 | store | 4 | `sp -0x1c` | `r0` |
| `stm` at 0x50b68 | store | 4 | `sp -0x18` | `r1` |
| `stm` at 0x50b68 | store | 4 | `sp -0x14` | `r2` |
| `str` at 0x50b74 | store | 4 | `sp -0x20` | `ret(0x1f1dc)` |
| `str` at 0x50b80 | store | 4 | `sp -0x2c` | `ret(0x1f1dc)` |
| `str` at 0x50b88 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x50b8c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x50b9c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x50bac | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x50bb0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x50bb4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x50bbc | load | 1 | `([sp-44] + [sp-36]) +0x0` | `-` |
| `str` at 0x50bc8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x50bd0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x50bd4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x50be0 | load | 1 | `([sp-44] + [sp-36]) +0x1` | `-` |
| `ldr` at 0x50be4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x50bec | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x50bf0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x50bf4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x50c04 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x50c08 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x50c14 | load | 1 | `([sp-44] + [sp-36]) +0x2` | `-` |
| `str` at 0x50c20 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x50c28 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x50c2c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x50c38 | load | 1 | `([sp-44] + [sp-36]) +0x3` | `-` |
| `ldr` at 0x50c3c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x50c44 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x50c48 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x50c4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x50c5c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x50c60 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x50c6c | load | 1 | `([sp-44] + [sp-36]) +0x4` | `-` |
| `str` at 0x50c78 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x50c80 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x50c84 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x50c90 | load | 1 | `([sp-44] + [sp-36]) +0x5` | `-` |
| `ldr` at 0x50c9c | load | 4 | `sp -0x38` | `-` |
| `str` at 0x50ca4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x50cac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x50cb0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x50cbc | load | 1 | `([sp-44] + [sp-36]) +0x6` | `-` |
| `ldr` at 0x50cc0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x50cc8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x50ccc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x50cd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50cdc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x50ce0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x50cf8 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0x50d04 | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0x50d0c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x50d14 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `str` at 0x50d20 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x50d24 | load | 4 | `sp -0x3c` | `-` |
| `ldmdb` at 0x50d28 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x50d28 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x50d28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x50b70 | `(unknown callee) 0x1f1dc` | `0x52` | `r1` | `r2` | `sp-28` |
| `bl` at 0x50b7c | `(unknown callee) 0x1f1dc` | `0x53` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x50b94 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x50bfc | `[sp-48] != [sp-28]` | `[sp-48] == [sp-28]` |
| `bne` at 0x50c54 | `[sp-52] != [sp-24]` | `[sp-52] == [sp-24]` |
| `bne` at 0x50cd8 | `[sp-56] != [sp-20]` | `[sp-56] == [sp-20]` |
| `beq` at 0x50cf0 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x50d28 | `r0` | `?` |

## `sernumGetKey`

- `sernumGetKey`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4f694 to 0x4f804.
  Established. The call to `0x4f090` receives entry r0 through r3 as candidates (`bl` at 0x4f6ac).
  The first comparison retains an unknown operand against `0x5e` (`bne` at 0x4f6dc). The return
  expression is unknown (`ldmdb` at 0x4f7f0). Key fields and validity meanings remain unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4f6a4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4f6a8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x4f6b8 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x4f6e0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4f6e8 | store | 4 | `sp -0x1c` | `[sp-28]+480` |
| `ldr` at 0x4f6ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4f6f0 | load | 4 | `[sp-28] +0x1c` | `-` |
| `ldr` at 0x4f6fc | load | 4 | `abs 0x4f7f4` | `-` |
| `str` at 0x4f708 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0x4f710 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4f714 | load | 4 | `[sp-28] +0x70` | `-` |
| `str` at 0x4f718 | store | 4 | `sp -0x3c` | `[[sp-28]+112]` |
| `ldr` at 0x4f71c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4f720 | load | 4 | `[sp-28] +0x6c` | `-` |
| `str` at 0x4f724 | store | 4 | `sp -0x38` | `[[sp-28]+108]` |
| `ldr` at 0x4f728 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4f72c | load | 4 | `[sp-28] +0x74` | `-` |
| `str` at 0x4f730 | store | 4 | `sp -0x40` | `[[sp-28]+116]` |
| `ldr` at 0x4f734 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x4f738 | store | 4 | `sp -0x2c` | `[[sp-28]+108]` |
| `ldr` at 0x4f73c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x4f740 | store | 4 | `sp -0x28` | `[[sp-28]+112]` |
| `ldr` at 0x4f744 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x4f748 | store | 4 | `sp -0x20` | `[[sp-28]+116]` |
| `ldr` at 0x4f74c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x4f758 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x4f758 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x4f758 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x4f758 | load | 4 | `sp -0x20` | `-` |
| `stm` at 0x4f75c | store | 4 | `[sp-20] +0x0` | `[[sp-28]+108]` |
| `stm` at 0x4f75c | store | 4 | `[sp-20] +0x4` | `[[sp-28]+112]` |
| `stm` at 0x4f75c | store | 4 | `[sp-20] +0x8` | `[sp-36]` |
| `stm` at 0x4f75c | store | 4 | `[sp-20] +0xc` | `[[sp-28]+116]` |
| `ldr` at 0x4f760 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x4f764 | load | 4 | `abs 0x4f7f8` | `-` |
| `ldr` at 0x4f770 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x4f774 | load | 4 | `abs 0x4f7f8` | `-` |
| `ldr` at 0x4f780 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x4f784 | load | 4 | `abs 0x4f7f8` | `-` |
| `ldr` at 0x4f790 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x4f79c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x4f7a0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x4f7b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4f7bc | load | 4 | `abs 0x4f7fc` | `-` |
| `ldr` at 0x4f7c4 | load | 4 | `abs 0x4f800` | `-` |
| `ldr` at 0x4f7c8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x4f7cc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x4f7d0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x4f7dc | store | 4 | `sp -0x50` | `0x2` |
| `str` at 0x4f7e8 | store | 4 | `sp -0x50` | `0x1` |
| `ldr` at 0x4f7ec | load | 4 | `sp -0x50` | `-` |
| `ldmdb` at 0x4f7f0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4f7f0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4f7f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4f6ac | `(unknown callee) 0x4f090` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x4f6c8 | `(unknown callee) 0xd1dfc` | `0x0` | `sp-28` | `?` | `sp-28` |
| `bl` at 0x4f6cc | `(unknown callee) 0x4f090` | `ret(0xd1dfc)` | `?` | `?` | `?` |
| `bl` at 0x4f700 | `(unknown callee) 0x20240` | `0x10ab5c` | `?` | `?` | `[[sp-28]+28]` |
| `bl` at 0x4f7c0 | `(unknown callee) 0x23f2c` | `0x10ab90` | `[[sp-28]+112]` | `?` | `[sp-24]` |
| `bl` at 0x4f7d4 | `(unknown callee) 0x2070c` | `0x10abc0` | `[sp-56]` | `[sp-60]` | `[sp-64]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x4f6dc | `? != 0x5e` | `? == 0x5e` |
| `beq` at 0x4f6f8 | `[[sp-28]+28] == 0x0` | `[[sp-28]+28] != 0x0` |
| `bhi` at 0x4f76c | `[sp-60] > 0xea60 (unsigned, 32-bit)` | `[sp-60] <= 0xea60 (unsigned, 32-bit)` |
| `bhi` at 0x4f77c | `[sp-56] > 0xea60 (unsigned, 32-bit)` | `[sp-56] <= 0xea60 (unsigned, 32-bit)` |
| `bhi` at 0x4f78c | `[sp-64] > 0xea60 (unsigned, 32-bit)` | `[sp-64] <= 0xea60 (unsigned, 32-bit)` |
| `bne` at 0x4f798 | `[sp-56] != 0x2` | `[sp-56] == 0x2` |
| `bhi` at 0x4f7a8 | `[sp-64] > [sp-60] (unsigned, 32-bit)` | `[sp-64] <= [sp-60] (unsigned, 32-bit)` |
| `beq` at 0x4f7b8 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4f7f0 | `r0` | `?` |

## `SysComCfg`

- `SysComCfg`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb0eb8 to 0xb10a0.
  Established. The first call to `0x1b124` has unknown, entry r1, `[0x2b1730]` and unknown as
  candidates (`bl` at 0xb0eec). The loop decision retains unknown operands (`blt` at 0xb0f18). The
  final r0 expression and configuration output format remain unknown (`ldmdb` at 0xb1080).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb0ec8 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xb0ecc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0ed0 | load | 4 | `abs 0xb1084` | `-` |
| `ldr` at 0xb0ed4 | load | 4 | `abs 0x2b1730` | `-` |
| `str` at 0xb0ef4 | store | 4 | `r0 +0x0` | `ret(0x1b124)` |
| `ldr` at 0xb0ef8 | load | 4 | `r0 +0x0` | `-` |
| `str` at 0xb0efc | store | 4 | `sp -0x1c` | `[r0]` |
| `str` at 0xb0f04 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xb0f08 | load | 4 | `abs 0xb1084` | `-` |
| `ldr` at 0xb0f0c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb0f10 | load | 4 | `abs 0x2b1730` | `-` |
| `ldr` at 0xb0f20 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb0f24 | load | 4 | `abs 0xb1088` | `-` |
| `ldr` at 0xb0f28 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb0f30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb0f40 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb0f48 | store | 4 | `sp -0x1c` | `([sp-28] + ret(0x1b22c))` |
| `ldr` at 0xb0f4c | load | 4 | `abs 0xb1084` | `-` |
| `ldr` at 0xb0f50 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb0f74 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xb0f7c | load | 4 | `(unknown)` | `-` |
| `str` at 0xb0f80 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb0f84 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb0f90 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb0fa0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb0fa4 | load | 4 | `abs 0xb108c` | `-` |
| `ldr` at 0xb0fb0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb0fb4 | load | 4 | `abs 0xb1090` | `-` |
| `ldr` at 0xb0fbc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb0fcc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb0fd4 | store | 4 | `sp -0x1c` | `([sp-28] + ret(0x1b22c))` |
| `ldr` at 0xb0fd8 | load | 4 | `abs 0xb1084` | `-` |
| `ldr` at 0xb0fdc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1000 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xb1008 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb100c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb1010 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb101c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb102c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1030 | load | 4 | `abs 0xb1094` | `-` |
| `ldr` at 0xb103c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1040 | load | 4 | `abs 0xb1098` | `-` |
| `ldr` at 0xb1048 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb104c | load | 4 | `abs 0xb109c` | `-` |
| `ldr` at 0xb1054 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1064 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb106c | store | 4 | `sp -0x1c` | `([sp-28] + ret(0x1b22c))` |
| `ldr` at 0xb1070 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb1078 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldmdb` at 0xb1080 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb1080 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb1080 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb1080 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb0eec | `(unknown callee) 0x1b124` | `?` | `r1` | `[0x2b1730]` | `?` |
| `bl` at 0xb0f2c | `(unknown callee) 0x1b49c` | `?` | `0x127698` | `?` | `[0x2b1730]` |
| `bl` at 0xb0f34 | `(unknown callee) 0x1b22c` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0xb0fa8 | `(unknown callee) 0x1b508` | `[sp-28]` | `0x1276a4` | `?` | `[sp-36]` |
| `bl` at 0xb0fb8 | `(unknown callee) 0x1b508` | `[sp-28]` | `0x1276b4` | `?` | `[sp-36]` |
| `bl` at 0xb0fc0 | `(unknown callee) 0x1b22c` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0xb1034 | `(unknown callee) 0x1b508` | `[sp-28]` | `0x1276c4` | `?` | `[sp-40]` |
| `bl` at 0xb1044 | `(unknown callee) 0x1b508` | `[sp-28]` | `0x1276d4` | `?` | `[sp-40]` |
| `bl` at 0xb1050 | `(unknown callee) 0x1b2d4` | `[sp-28]` | `0x127680` | `?` | `?` |
| `bl` at 0xb1058 | `(unknown callee) 0x1b22c` | `[sp-28]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xb0f18 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xb0f8c | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0xb0f98 | `[sp-36] == 0x1` | `[sp-36] != 0x1` |
| `beq` at 0xb1018 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `beq` at 0xb1024 | `[sp-40] == 0x1` | `[sp-40] != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb1080 | `r0` | `?` |

## `SysComSimStart`

- `SysComSimStart`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaf708 to 0xaf7e8.
  Established. The initial branch takes `[0x149a4c] != 0x0` and falls through on zero (`bne` at
  0xaf724). The call to `0x22920` carries `0x149a58`, `0x127518`, `0x0` and `0x9` as candidates
  (`bl` at 0xaf738). The merged r0 value and simulation-resource ownership remain unknown (`ldmdb`
  at 0xaf7c4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xaf718 | load | 4 | `abs 0xaf7c8` | `-` |
| `ldr` at 0xaf71c | load | 4 | `abs 0x149a4c` | `-` |
| `ldr` at 0xaf728 | load | 4 | `abs 0xaf7cc` | `-` |
| `ldr` at 0xaf72c | load | 4 | `abs 0xaf7d0` | `-` |
| `str` at 0xaf740 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xaf748 | store | 4 | `sp -0x24` | `0x800` |
| `str` at 0xaf750 | store | 4 | `sp -0x20` | `0x19` |
| `str` at 0xaf758 | store | 4 | `sp -0x1c` | `0xa` |
| `str` at 0xaf760 | store | 4 | `sp -0x18` | `0x9` |
| `ldr` at 0xaf764 | load | 4 | `abs 0xaf7d4` | `-` |
| `ldr` at 0xaf768 | load | 4 | `abs 0xaf7d8` | `-` |
| `ldr` at 0xaf76c | load | 4 | `abs 0xaf7dc` | `-` |
| `ldr` at 0xaf778 | load | 4 | `abs 0xaf7c8` | `-` |
| `str` at 0xaf780 | store | 4 | `abs 0x149a4c` | `0x1` |
| `ldr` at 0xaf784 | load | 4 | `abs 0xaf7cc` | `-` |
| `str` at 0xaf790 | store | 4 | `sp -0x14` | `ret(0x22dc0)` |
| `ldr` at 0xaf794 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf7a0 | load | 4 | `abs 0xaf7e0` | `-` |
| `str` at 0xaf7a4 | store | 4 | `sp -0x28` | `0x127530` |
| `ldr` at 0xaf7a8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaf7ac | store | 4 | `sp -0x24` | `ret(0x22dc0)` |
| `ldr` at 0xaf7b4 | load | 4 | `abs 0xaf7e4` | `-` |
| `ldmdb` at 0xaf7c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaf7c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaf7c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaf738 | `(unknown callee) 0x22920` | `0x149a58` | `0x127518` | `0x0` | `0x9` |
| `bl` at 0xaf774 | `(unknown callee) 0x223f0` | `0x149a54` | `0x127524` | `0xaf808` | `0x0` |
| `bl` at 0xaf788 | `(unknown callee) 0x22dc0` | `0x149a58` | `?` | `?` | `?` |
| `bl` at 0xaf7c0 | `(unknown callee) 0x21e28` | `0x9` | `0x12755c` | `0xbd` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xaf724 | `[0x149a4c] != 0x0` | `[0x149a4c] == 0x0` |
| `beq` at 0xaf79c | `ret(0x22dc0) == 0x0` | `ret(0x22dc0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaf7c4 | `r0` | `?` |

## `SysComSimStop`

- `SysComSimStop`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaf7e8 to 0xaf808.
  Established. A word store clears absolute `0x14970c` (`str` at 0xaf7fc). Entry r0 survives to the
  recorded return (`ldmdb` at 0xaf800). The external stop action implied by the symbol is not
  established by this isolated store.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xaf7f4 | load | 4 | `abs 0xaf804` | `-` |
| `str` at 0xaf7fc | store | 4 | `abs 0x14970c` | `0x0` |
| `ldmdb` at 0xaf800 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaf800 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaf800 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaf800 | `r0` | `r0` |
