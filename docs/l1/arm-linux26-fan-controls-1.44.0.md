# ARM Linux 2.6 fan controls in 1.44.0

Read from `6.5.30/usr/cpu/firmware/sysco/l1.bin`, package `6.5.30:l1#6` (version 1.44.0),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0xa936a8`.
Carrier SHA-256: `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
Package SHA-256: `e57ab8ecd48858e39ecdd0050a23e444a491c35e87f9d8ebb48881fc25229a06`.
Inner executable SHA-256: `b33154e53b00531623db6ff365e80b1c1825bc9427961c59fdc132c24f0c819e`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1629656 bytes.
Segment SHA-256: `f6491062ef0a2afd254f8510f82b4814c563c9256691f9a2b393cf6e84ba7fd8`. Analysed routine spans use the exact
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
No unresolved computed transfers were reported for these routines.
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-fan-controls-facts.txt) retain all tool sections.

## `fanCtrlInit`

- `fanCtrlInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x61c68 to 0x61da0.
  Established. Included word assignments write `ret(0x6e140)` to `0x1a5c9c` and `ret(0x204d8)` to `0x1a5c98` (`str` at 0x61cf8, `str` at 0x61d20).
  A nonzero `[0x1a5c9c]` takes `0x61d54`, while zero falls through (`bne` at 0x61ce8).
  An included byte assignment writes `([sp-32] & 0xff)` to `[sp-36] +0x4` (`strb` at 0x61d80).
  Fan controller record ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x61c78 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x61c7c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x61c80 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x61c84 | load | 4 | `abs 0x61d8c` | `-` |
| `ldr` at 0x61c88 | load | 4 | `abs 0x1a5c98` | `-` |
| `ldr` at 0x61c9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x61ca8 | load | 4 | `abs 0x61d90` | `-` |
| `str` at 0x61cac | store | 4 | `sp -0x38` | `0x163040` |
| `ldr` at 0x61cb0 | load | 4 | `abs 0x61d8c` | `-` |
| `ldr` at 0x61cb4 | load | 4 | `abs 0x1a5c98` | `-` |
| `str` at 0x61cb8 | store | 4 | `sp -0x34` | `[0x1a5c98]` |
| `ldr` at 0x61cbc | load | 4 | `abs 0x61d94` | `-` |
| `ldr` at 0x61cc0 | load | 4 | `abs 0x1a5c9c` | `-` |
| `str` at 0x61cc4 | store | 4 | `sp -0x30` | `[0x1a5c9c]` |
| `ldr` at 0x61ccc | load | 4 | `abs 0x61d98` | `-` |
| `ldr` at 0x61cdc | load | 4 | `abs 0x61d94` | `-` |
| `ldr` at 0x61ce0 | load | 4 | `abs 0x1a5c9c` | `-` |
| `ldr` at 0x61cf4 | load | 4 | `abs 0x61d94` | `-` |
| `str` at 0x61cf8 | store | 4 | `abs 0x1a5c9c` | `ret(0x6e140)` |
| `ldr` at 0x61cfc | load | 4 | `abs 0x61d94` | `-` |
| `ldr` at 0x61d00 | load | 4 | `abs 0x1a5c9c` | `-` |
| `str` at 0x61d08 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x61d0c | load | 4 | `abs 0x61d8c` | `-` |
| `ldr` at 0x61d10 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x61d20 | store | 4 | `abs 0x1a5c98` | `ret(0x204d8)` |
| `ldr` at 0x61d24 | load | 4 | `abs 0x1a5c98` | `-` |
| `ldr` at 0x61d30 | load | 4 | `abs 0x61d9c` | `-` |
| `str` at 0x61d34 | store | 4 | `sp -0x38` | `0x163074` |
| `ldr` at 0x61d38 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x61d3c | store | 4 | `sp -0x34` | `[sp-44]` |
| `ldr` at 0x61d44 | load | 4 | `abs 0x61d98` | `-` |
| `ldr` at 0x61d54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x61d5c | load | 4 | `abs 0x61d8c` | `-` |
| `ldr` at 0x61d60 | load | 4 | `abs 0x1a5c98` | `-` |
| `str` at 0x61d68 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x61d6c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x61d70 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x61d74 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0x61d78 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x61d7c | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x61d80 | store | 1 | `[sp-36] +0x4` | `([sp-32] & 0xff)` |
| `ldm` at 0x61d88 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x61d88 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x61d88 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x61d88 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x61c94 | `(unknown callee) 0x6e190` | `r0` | `r1` | `r2` | `[0x1a5c98]` |
| `bl` at 0x61cd8 | `(unknown callee) 0x22070` | `0x9` | `0x162f88` | `0xb3` | `0x0` |
| `bl` at 0x61cec | `(unknown callee) 0x6e140` | `?` | `?` | `?` | `[0x1a5c9c]` |
| `bl` at 0x61d18 | `(unknown callee) 0x204d8` | `[sp-44]` | `0x1` | `ret(0x6e140)` | `?` |
| `bl` at 0x61d50 | `(unknown callee) 0x22070` | `0x9` | `0x162f88` | `0xbe` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x61c90 | `[0x1a5c98] == 0x0` | `[0x1a5c98] != 0x0` |
| `bne` at 0x61ca4 | `ret(0x6e190) != [sp-24]` | `ret(0x6e190) == [sp-24]` |
| `bne` at 0x61ce8 | `[0x1a5c9c] != 0x0` | `[0x1a5c9c] == 0x0` |
| `bne` at 0x61d2c | `[0x1a5c98] != 0x0` | `[0x1a5c98] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x61d88 | `r0` | `?` |

## `fanCtrlRemove`

- `fanCtrlRemove`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x61da0 to 0x61ef4.
  Established. The signed condition entry `r0 >= [0x1a5c9c]` takes `0x61edc`, while lower falls through (`bge` at 0x61dc4).
  Included assignments clear `[sp-24] +0x4` as a byte and `[sp-24] +0x0` as a word to `0x0` (`strb` at 0x61ecc, `str` at 0x61ed8).
  Fan controller removal effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x61db0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x61db4 | load | 4 | `abs 0x61ee4` | `-` |
| `ldr` at 0x61db8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61dbc | load | 4 | `abs 0x1a5c9c` | `-` |
| `ldr` at 0x61dc8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61dd0 | load | 4 | `abs 0x61ee8` | `-` |
| `ldr` at 0x61dd4 | load | 4 | `abs 0x1a5c98` | `-` |
| `ldrb` at 0x61ddc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x61dec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61df4 | load | 4 | `abs 0x61ee8` | `-` |
| `ldr` at 0x61df8 | load | 4 | `abs 0x1a5c98` | `-` |
| `str` at 0x61e00 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x61e04 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x61e08 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x61e14 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x61e28 | store | 4 | `sp -0x1c` | `ret(0x6e1b0)` |
| `ldr` at 0x61e2c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x61e3c | store | 4 | `sp -0x20` | `ret(0x2031c)+1` |
| `ldr` at 0x61e40 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x61e68 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x61e6c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x61e70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61e74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x61e80 | load | 4 | `abs 0x61eec` | `-` |
| `str` at 0x61e84 | store | 4 | `(unknown)` | `0x1630b0` |
| `str` at 0x61e88 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `str` at 0x61e90 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x61e94 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x61e98 | load | 1 | `[sp-24] +0x4` | `-` |
| `str` at 0x61e9c | store | 4 | `(unknown)` | `[[sp-24]+4]:1` |
| `ldr` at 0x61ea4 | load | 4 | `abs 0x61ef0` | `-` |
| `ldr` at 0x61eb4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x61eb8 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x61ec4 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x61ecc | store | 1 | `[sp-24] +0x4` | `0x0` |
| `ldr` at 0x61ed0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x61ed8 | store | 4 | `[sp-24] +0x0` | `0x0` |
| `ldm` at 0x61ee0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x61ee0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x61ee0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x61e24 | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x61e30 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x61e78 | `(unknown callee) 0x203f4` | `[sp-36]` | `[sp-28]` | `[sp-32]` | `?` |
| `bl` at 0x61eb0 | `(unknown callee) 0x22070` | `0x9` | `0x162f88` | `0xd0` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x61dc4 | `r0 >= [0x1a5c9c] (signed, 32-bit)` | `r0 < [0x1a5c9c] (signed, 32-bit)` |
| `bne` at 0x61de4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x61e10 | `[[sp-24]+4]:1 != 0x0` | `[[sp-24]+4]:1 == 0x0` |
| `beq` at 0x61ec0 | `[[sp-24]+4]:1 == 0x0` | `[[sp-24]+4]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x61ee0 | `r0` | `?` |

## `fanTachInit`

- `fanTachInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x61848 to 0x61ae4.
  Established. Included assignments write byte `([sp-32] & 0xff)` to `[sp-36] +0x4` and word `ret(0x204d8)` to `[sp-36] +0x8` (`strb` at 0x61978, `str` at 0x61998).
  A nonzero `[[sp-36]+8]` takes `0x61a3c`, while zero falls through (`bne` at 0x619a4).
  The unnamed `0x20550` call carries `[[sp-36]+8]`, `0x1`, `[[sp-36]+4]:1`, `[[sp-36]+4]:1` (`bl` at 0x61a54).
  Tachometer state record meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x61858 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x6185c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x61860 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x61864 | load | 4 | `abs 0x61acc` | `-` |
| `ldr` at 0x61868 | load | 4 | `abs 0x1a5c90` | `-` |
| `ldr` at 0x6187c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x61888 | load | 4 | `abs 0x61ad0` | `-` |
| `str` at 0x6188c | store | 4 | `sp -0x40` | `0x162f54` |
| `ldr` at 0x61890 | load | 4 | `abs 0x61acc` | `-` |
| `ldr` at 0x61894 | load | 4 | `abs 0x1a5c90` | `-` |
| `str` at 0x61898 | store | 4 | `sp -0x3c` | `[0x1a5c90]` |
| `ldr` at 0x6189c | load | 4 | `abs 0x61ad4` | `-` |
| `ldr` at 0x618a0 | load | 4 | `abs 0x1a5c94` | `-` |
| `str` at 0x618a4 | store | 4 | `sp -0x38` | `[0x1a5c94]` |
| `ldr` at 0x618ac | load | 4 | `abs 0x61ad8` | `-` |
| `ldr` at 0x618bc | load | 4 | `abs 0x61ad4` | `-` |
| `ldr` at 0x618c0 | load | 4 | `abs 0x1a5c94` | `-` |
| `ldr` at 0x618d4 | load | 4 | `abs 0x61ad4` | `-` |
| `str` at 0x618d8 | store | 4 | `abs 0x1a5c94` | `ret(0x6e140)` |
| `ldr` at 0x618dc | load | 4 | `abs 0x61ad4` | `-` |
| `ldr` at 0x618e0 | load | 4 | `abs 0x1a5c94` | `-` |
| `str` at 0x618f4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x618f8 | load | 4 | `abs 0x61acc` | `-` |
| `ldr` at 0x618fc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x6190c | store | 4 | `abs 0x1a5c90` | `ret(0x204d8)` |
| `ldr` at 0x61910 | load | 4 | `abs 0x1a5c90` | `-` |
| `ldr` at 0x6191c | load | 4 | `abs 0x61adc` | `-` |
| `str` at 0x61920 | store | 4 | `sp -0x40` | `0x162f9c` |
| `ldr` at 0x61924 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x61928 | store | 4 | `sp -0x3c` | `[sp-44]` |
| `ldr` at 0x61930 | load | 4 | `abs 0x61ad8` | `-` |
| `ldr` at 0x61940 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x61954 | load | 4 | `abs 0x61acc` | `-` |
| `ldr` at 0x61958 | load | 4 | `abs 0x1a5c90` | `-` |
| `str` at 0x61960 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x61964 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x61968 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6196c | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0x61970 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x61974 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x61978 | store | 1 | `[sp-36] +0x4` | `([sp-32] & 0xff)` |
| `ldr` at 0x6197c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x61980 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x61984 | load | 1 | `[sp-36] +0x4` | `-` |
| `str` at 0x61998 | store | 4 | `[sp-36] +0x8` | `ret(0x204d8)` |
| `ldr` at 0x6199c | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0x619a8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x619bc | store | 4 | `sp -0x2c` | `ret(0x6e1b0)` |
| `ldr` at 0x619c0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x619d0 | store | 4 | `sp -0x30` | `ret(0x2031c)+1` |
| `ldr` at 0x619d4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x619fc | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x61a00 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x61a04 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x61a08 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x61a14 | load | 4 | `abs 0x61ae0` | `-` |
| `str` at 0x61a18 | store | 4 | `(unknown)` | `0x162fd8` |
| `str` at 0x61a1c | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x61a20 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x61a24 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0x61a2c | load | 4 | `abs 0x61ad8` | `-` |
| `ldr` at 0x61a3c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x61a40 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x61a44 | load | 1 | `[sp-36] +0x4` | `-` |
| `ldr` at 0x61a48 | load | 4 | `[sp-36] +0x8` | `-` |
| `str` at 0x61a5c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x61a60 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x61a64 | load | 1 | `[sp-36] +0x4` | `-` |
| `ldr` at 0x61a68 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x61a74 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x61a78 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x61a8c | load | 4 | `[sp-36] +0x0` | `-` |
| `ldrb` at 0x61a94 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x61aa0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x61aa8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x61ab4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x61abc | store | 4 | `sp -0x28` | `?` |
| `ldm` at 0x61ac8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x61ac8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x61ac8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x61ac8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x61874 | `(unknown callee) 0x6e190` | `r0` | `r1` | `r2` | `[0x1a5c90]` |
| `bl` at 0x618b8 | `(unknown callee) 0x22070` | `0x9` | `0x162f88` | `0x71` | `0x0` |
| `bl` at 0x618cc | `(unknown callee) 0x6e140` | `?` | `?` | `?` | `[0x1a5c94]` |
| `bl` at 0x61904 | `(unknown callee) 0x204d8` | `[sp-44]` | `0x1` | `[0x1a5c94]` | `?` |
| `bl` at 0x6193c | `(unknown callee) 0x22070` | `0x9` | `0x162f88` | `0x7c` | `0x0` |
| `bl` at 0x61990 | `(unknown callee) 0x204d8` | `[[sp-36]+4]:1` | `0x1` | `[sp-36]` | `[[sp-36]+4]:1` |
| `bl` at 0x619b8 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x619c4 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x61a0c | `(unknown callee) 0x203f4` | `[sp-52]` | `[sp-44]` | `[sp-48]` | `?` |
| `bl` at 0x61a38 | `(unknown callee) 0x22070` | `0x9` | `0x162f88` | `0x87` | `0x0` |
| `bl` at 0x61a54 | `(unknown callee) 0x20550` | `[[sp-36]+8]` | `0x1` | `[[sp-36]+4]:1` | `[[sp-36]+4]:1` |
| `bl` at 0x61ab0 | `(unknown callee) 0x61ef4` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x61870 | `[0x1a5c90] == 0x0` | `[0x1a5c90] != 0x0` |
| `bne` at 0x61884 | `ret(0x6e190) != [sp-24]` | `ret(0x6e190) == [sp-24]` |
| `bne` at 0x618c8 | `[0x1a5c94] != 0x0` | `[0x1a5c94] == 0x0` |
| `bne` at 0x61918 | `[0x1a5c90] != 0x0` | `[0x1a5c90] == 0x0` |
| `bne` at 0x619a4 | `[[sp-36]+8] != 0x0` | `[[sp-36]+8] == 0x0` |
| `ble` at 0x61a70 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x61a9c | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x61ac8 | `r0` | `?` |

## `fanTachRemove`

- `fanTachRemove`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x61ae4 to 0x61c68.
  Established. A byte assignment clears `[sp-24] +0x4`, while separate word assignments clear `[sp-24] +0x0` and `[sp-24] +0x8` (`strb` at 0x61c28, `str` at 0x61c34, `str` at 0x61c4c).
  The unnamed `0x20514` call carries `[[sp-24]+8]`, unknown, `[sp-24]` and `[sp-24]` (`bl` at 0x61c40).
  Signed entry `r0 >= [0x1a5c94]` takes `0x61c50`, while lower falls through (`bge` at 0x61b08).
  Tachometer removal hardware responsibilities remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x61af4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x61af8 | load | 4 | `abs 0x61c58` | `-` |
| `ldr` at 0x61afc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61b00 | load | 4 | `abs 0x1a5c94` | `-` |
| `ldr` at 0x61b0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61b20 | load | 4 | `abs 0x61c5c` | `-` |
| `ldr` at 0x61b24 | load | 4 | `abs 0x1a5c90` | `-` |
| `ldrb` at 0x61b2c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x61b3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61b50 | load | 4 | `abs 0x61c5c` | `-` |
| `ldr` at 0x61b54 | load | 4 | `abs 0x1a5c90` | `-` |
| `str` at 0x61b5c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x61b60 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x61b64 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x61b70 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x61b84 | store | 4 | `sp -0x1c` | `ret(0x6e1b0)` |
| `ldr` at 0x61b88 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x61b98 | store | 4 | `sp -0x20` | `ret(0x2031c)+1` |
| `ldr` at 0x61b9c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x61bc4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x61bc8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x61bcc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61bd0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x61bdc | load | 4 | `abs 0x61c60` | `-` |
| `str` at 0x61be0 | store | 4 | `(unknown)` | `0x163018` |
| `str` at 0x61be4 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `str` at 0x61bec | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x61bf0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x61bf4 | load | 1 | `[sp-24] +0x4` | `-` |
| `str` at 0x61bf8 | store | 4 | `(unknown)` | `[[sp-24]+4]:1` |
| `ldr` at 0x61c00 | load | 4 | `abs 0x61c64` | `-` |
| `ldr` at 0x61c10 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x61c14 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x61c20 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x61c28 | store | 1 | `[sp-24] +0x4` | `0x0` |
| `ldr` at 0x61c2c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x61c34 | store | 4 | `[sp-24] +0x0` | `0x0` |
| `ldr` at 0x61c38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x61c3c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x61c44 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x61c4c | store | 4 | `[sp-24] +0x8` | `0x0` |
| `ldm` at 0x61c54 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x61c54 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x61c54 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x61b80 | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x61b8c | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x61bd4 | `(unknown callee) 0x203f4` | `[sp-36]` | `[sp-28]` | `[sp-32]` | `?` |
| `bl` at 0x61c0c | `(unknown callee) 0x22070` | `0x9` | `0x162f88` | `0x99` | `0x0` |
| `bl` at 0x61c40 | `(unknown callee) 0x20514` | `[[sp-24]+8]` | `?` | `[sp-24]` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x61b08 | `r0 >= [0x1a5c94] (signed, 32-bit)` | `r0 < [0x1a5c94] (signed, 32-bit)` |
| `bne` at 0x61b34 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x61b6c | `[[sp-24]+4]:1 != 0x0` | `[[sp-24]+4]:1 == 0x0` |
| `beq` at 0x61c1c | `[[sp-24]+4]:1 == 0x0` | `[[sp-24]+4]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x61c54 | `r0` | `?` |

## `fanTachNotPresent`

- `fanTachNotPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x61ef4 to 0x62064.
  Established. Equality `(r1 & 0xff) == 0xff` takes `0x6204c`, while inequality falls through (`beq` at 0x61f58).
  The unsigned condition `[[sp-28]+4]:1 > (r1 & 0xff)` takes `0x62034`, while less than or equal falls through (`bhi` at 0x61f90).
  A byte assignment clears an unknown effective address (`strb` at 0x62048).
  The absent-state record and hardware meaning remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x61f04 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x61f0c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x61f10 | load | 4 | `abs 0x62054` | `-` |
| `ldr` at 0x61f14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61f18 | load | 4 | `abs 0x1a5c94` | `-` |
| `ldr` at 0x61f24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61f38 | load | 4 | `abs 0x62058` | `-` |
| `ldr` at 0x61f3c | load | 4 | `abs 0x1a5c90` | `-` |
| `ldrb` at 0x61f44 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x61f50 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x61f5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61f70 | load | 4 | `abs 0x62058` | `-` |
| `ldr` at 0x61f74 | load | 4 | `abs 0x1a5c90` | `-` |
| `str` at 0x61f7c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x61f80 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x61f84 | load | 1 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x61f88 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x61f94 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x61fa8 | store | 4 | `sp -0x20` | `ret(0x6e1b0)` |
| `ldr` at 0x61fac | load | 4 | `sp -0x20` | `-` |
| `str` at 0x61fbc | store | 4 | `sp -0x24` | `ret(0x2031c)+1` |
| `ldr` at 0x61fc0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x61fe8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x61fec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x61ff0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x61ff4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x62000 | load | 4 | `abs 0x6205c` | `-` |
| `str` at 0x62004 | store | 4 | `(unknown)` | `0x163018` |
| `str` at 0x62008 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldrb` at 0x6200c | load | 1 | `sp -0x15` | `-` |
| `str` at 0x62010 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x62014 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x62018 | load | 1 | `[sp-28] +0x4` | `-` |
| `str` at 0x6201c | store | 4 | `(unknown)` | `[[sp-28]+4]:1` |
| `ldr` at 0x62024 | load | 4 | `abs 0x62060` | `-` |
| `ldr` at 0x62034 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x62038 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x6203c | load | 4 | `[sp-28] +0x8` | `-` |
| `strb` at 0x62048 | store | 1 | `(unknown)` | `0x0` |
| `ldm` at 0x62050 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x62050 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x62050 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x61fa4 | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x61fb0 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x61ff8 | `(unknown callee) 0x203f4` | `[sp-40]` | `[sp-32]` | `[sp-36]` | `?` |
| `bl` at 0x62030 | `(unknown callee) 0x22070` | `0x9` | `0x162f88` | `0xe1` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x61f20 | `r0 >= [0x1a5c94] (signed, 32-bit)` | `r0 < [0x1a5c94] (signed, 32-bit)` |
| `beq` at 0x61f4c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x61f58 | `(r1 & 0xff) == 0xff` | `(r1 & 0xff) != 0xff` |
| `bhi` at 0x61f90 | `[[sp-28]+4]:1 > (r1 & 0xff) (unsigned, 32-bit)` | `[[sp-28]+4]:1 <= (r1 & 0xff) (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x62050 | `r0` | `?` |

## `fanTachPresent`

- `fanTachPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x62064 to 0x621d4.
  Established. The signed comparison entry `r0 >= [0x1a5c94]` takes `0x621bc`, while lower falls through (`bge` at 0x62090).
  The unnamed `0x6e1b0` call carries entry `r0`, `0x1`, `0x0` and `0x0` (`bl` at 0x62114).
  A later byte assignment writes zero to an unknown effective address (`strb` at 0x621b8).
  Presence marking on the physical fan remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x62074 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x6207c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x62080 | load | 4 | `abs 0x621c4` | `-` |
| `ldr` at 0x62084 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62088 | load | 4 | `abs 0x1a5c94` | `-` |
| `ldr` at 0x62094 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x620a8 | load | 4 | `abs 0x621c8` | `-` |
| `ldr` at 0x620ac | load | 4 | `abs 0x1a5c90` | `-` |
| `ldrb` at 0x620b4 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x620c0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x620cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x620e0 | load | 4 | `abs 0x621c8` | `-` |
| `ldr` at 0x620e4 | load | 4 | `abs 0x1a5c90` | `-` |
| `str` at 0x620ec | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x620f0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x620f4 | load | 1 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x620f8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x62104 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x62118 | store | 4 | `sp -0x20` | `ret(0x6e1b0)` |
| `ldr` at 0x6211c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x6212c | store | 4 | `sp -0x24` | `ret(0x2031c)+1` |
| `ldr` at 0x62130 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x62158 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x6215c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62160 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x62164 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x62170 | load | 4 | `abs 0x621cc` | `-` |
| `str` at 0x62174 | store | 4 | `(unknown)` | `0x163018` |
| `str` at 0x62178 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldrb` at 0x6217c | load | 1 | `sp -0x15` | `-` |
| `str` at 0x62180 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x62184 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x62188 | load | 1 | `[sp-28] +0x4` | `-` |
| `str` at 0x6218c | store | 4 | `(unknown)` | `[[sp-28]+4]:1` |
| `ldr` at 0x62194 | load | 4 | `abs 0x621d0` | `-` |
| `ldr` at 0x621a4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x621a8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x621ac | load | 4 | `[sp-28] +0x8` | `-` |
| `strb` at 0x621b8 | store | 1 | `(unknown)` | `0x0` |
| `ldm` at 0x621c0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x621c0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x621c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x62114 | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x62120 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x62168 | `(unknown callee) 0x203f4` | `[sp-40]` | `[sp-32]` | `[sp-36]` | `?` |
| `bl` at 0x621a0 | `(unknown callee) 0x22070` | `0x9` | `0x162f88` | `0xea` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x62090 | `r0 >= [0x1a5c94] (signed, 32-bit)` | `r0 < [0x1a5c94] (signed, 32-bit)` |
| `beq` at 0x620bc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x620c8 | `(r1 & 0xff) == 0xff` | `(r1 & 0xff) != 0xff` |
| `bhi` at 0x62100 | `[[sp-28]+4]:1 > (r1 & 0xff) (unsigned, 32-bit)` | `[[sp-28]+4]:1 <= (r1 & 0xff) (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x621c0 | `r0` | `?` |

## `fanTachIsPresent`

- `fanTachIsPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x621d4 to 0x6235c.
  Established. A word assignment stores `0xfffffa85` at `sp -0x2c` (`str` at 0x62244).
  The unsigned comparison `[[sp-28]+4]:1 > (r1 & 0xff)` takes `0x62324`, while less than or equal falls through (`bhi` at 0x62280).
  A byte read has an unknown effective address (`ldrb` at 0x62334).
  Tachometer presence encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x621e4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x621ec | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x621f0 | load | 4 | `abs 0x62348` | `-` |
| `ldr` at 0x621f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x621f8 | load | 4 | `abs 0x1a5c94` | `-` |
| `ldr` at 0x62204 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62218 | load | 4 | `abs 0x6234c` | `-` |
| `ldr` at 0x6221c | load | 4 | `abs 0x1a5c90` | `-` |
| `ldrb` at 0x62224 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x62230 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x62240 | load | 4 | `abs 0x62350` | `-` |
| `str` at 0x62244 | store | 4 | `sp -0x2c` | `0xfffffa85` |
| `ldr` at 0x6224c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62260 | load | 4 | `abs 0x6234c` | `-` |
| `ldr` at 0x62264 | load | 4 | `abs 0x1a5c90` | `-` |
| `str` at 0x6226c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x62270 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x62274 | load | 1 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x62278 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x62284 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x62298 | store | 4 | `sp -0x20` | `ret(0x6e1b0)` |
| `ldr` at 0x6229c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x622ac | store | 4 | `sp -0x24` | `ret(0x2031c)+1` |
| `ldr` at 0x622b0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x622d8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x622dc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x622e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x622e4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x622f0 | load | 4 | `abs 0x62354` | `-` |
| `str` at 0x622f4 | store | 4 | `(unknown)` | `0x163018` |
| `str` at 0x622f8 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldrb` at 0x622fc | load | 1 | `sp -0x15` | `-` |
| `str` at 0x62300 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x62304 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x62308 | load | 1 | `[sp-28] +0x4` | `-` |
| `str` at 0x6230c | store | 4 | `(unknown)` | `[[sp-28]+4]:1` |
| `ldr` at 0x62314 | load | 4 | `abs 0x62358` | `-` |
| `ldr` at 0x62324 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x62328 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x6232c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0x62334 | load | 1 | `(unknown)` | `-` |
| `str` at 0x62338 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x6233c | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x62344 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x62344 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x62344 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x62294 | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x622a0 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x622e8 | `(unknown callee) 0x203f4` | `[sp-40]` | `[sp-32]` | `[sp-36]` | `?` |
| `bl` at 0x62320 | `(unknown callee) 0x22070` | `0x9` | `0x162f88` | `0xf4` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x62200 | `r0 >= [0x1a5c94] (signed, 32-bit)` | `r0 < [0x1a5c94] (signed, 32-bit)` |
| `beq` at 0x6222c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x62238 | `(r1 & 0xff) == 0xff` | `(r1 & 0xff) != 0xff` |
| `bhi` at 0x62280 | `[[sp-28]+4]:1 > (r1 & 0xff) (unsigned, 32-bit)` | `[[sp-28]+4]:1 <= (r1 & 0xff) (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x62344 | `r0` | `?` |

## `fanGetTachCfg`

- `fanGetTachCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x62bfc to 0x62cc0.
  Established. A word assignment writes `[[sp-28]]` to entry `r1 +0x0` (`str` at 0x62c9c).
  A byte read addresses `[sp-28] +0x4`, and its value is assigned to the word at `sp -0x20` (`ldrb` at 0x62ca4, `str` at 0x62ca8).
  Zero entry `r1` takes `0x62ca0`, while nonzero falls through (`beq` at 0x62c8c).
  Fan configuration field ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x62c0c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x62c10 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x62c14 | load | 4 | `abs 0x62cb8` | `-` |
| `ldr` at 0x62c18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62c1c | load | 4 | `abs 0x1a5c94` | `-` |
| `ldr` at 0x62c28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62c3c | load | 4 | `abs 0x62cbc` | `-` |
| `ldr` at 0x62c40 | load | 4 | `abs 0x1a5c90` | `-` |
| `ldrb` at 0x62c48 | load | 1 | `(unknown)` | `-` |
| `str` at 0x62c58 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x62c60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62c74 | load | 4 | `abs 0x62cbc` | `-` |
| `ldr` at 0x62c78 | load | 4 | `abs 0x1a5c90` | `-` |
| `str` at 0x62c80 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x62c84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x62c90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x62c94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62c98 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x62c9c | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0x62ca0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x62ca4 | load | 1 | `[sp-28] +0x4` | `-` |
| `str` at 0x62ca8 | store | 4 | `sp -0x20` | `[[sp-28]+4]:1` |
| `ldr` at 0x62cac | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x62cb4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x62cb4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x62cb4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x62c24 | `r0 >= [0x1a5c94] (signed, 32-bit)` | `r0 < [0x1a5c94] (signed, 32-bit)` |
| `bne` at 0x62c50 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x62c8c | `r1 == 0x0` | `r1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x62cb4 | `r0` | `?` |

## `fanReset`

- `fanReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6235c to 0x62bfc.
  Established. Included byte reads address `[sp-32] +0x3`, `[sp-32] +0x4` and `[sp-32] +0x7` (`ldrb` at 0x62874, `ldrb` at 0x6287c, `ldrb` at 0x62884).
  The unnamed `0x125a58` call carries `[sp-28]`, `[[sp-32]+3]:1`, `[[sp-32]+5]:1` and `[sp-37]:1` (`bl` at 0x626c8).
  Another unnamed `0x11366c` call carries `[sp-28]`, `[[sp-32]+3]:1`, `[[sp-32]+4]:1` and `[[sp-32]+7]:1` (`bl` at 0x6288c).
  The reset sequence implemented by those callees remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6236c | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0x62370 | load | 4 | `abs 0x62bbc` | `-` |
| `ldr` at 0x62374 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62378 | load | 4 | `abs 0x1a5c94` | `-` |
| `ldr` at 0x62384 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62398 | load | 4 | `abs 0x62bc0` | `-` |
| `ldr` at 0x6239c | load | 4 | `abs 0x1a5c90` | `-` |
| `ldrb` at 0x623a4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x623b0 | load | 4 | `abs 0x62bc4` | `-` |
| `str` at 0x623b4 | store | 4 | `sp -0x60` | `0xfffffa85` |
| `ldr` at 0x623bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x623d0 | load | 4 | `abs 0x62bc0` | `-` |
| `ldr` at 0x623d4 | load | 4 | `abs 0x1a5c90` | `-` |
| `str` at 0x623dc | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x623e0 | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0x623e4 | load | 1 | `[sp-72] +0x4` | `-` |
| `ldr` at 0x623f0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x62404 | store | 4 | `sp -0x50` | `ret(0x6e1b0)` |
| `ldr` at 0x62408 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x62418 | store | 4 | `sp -0x54` | `ret(0x2031c)+1` |
| `ldr` at 0x6241c | load | 4 | `sp -0x54` | `-` |
| `str` at 0x62444 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x62448 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x6244c | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x62450 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x6245c | load | 4 | `abs 0x62bc8` | `-` |
| `str` at 0x62460 | store | 4 | `(unknown)` | `0x163018` |
| `str` at 0x62464 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `str` at 0x6246c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x62470 | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0x62474 | load | 1 | `[sp-72] +0x4` | `-` |
| `str` at 0x62478 | store | 4 | `(unknown)` | `[[sp-72]+4]:1` |
| `ldr` at 0x62480 | load | 4 | `abs 0x62bcc` | `-` |
| `ldr` at 0x62484 | load | 4 | `abs 0x62bd0` | `-` |
| `ldr` at 0x62490 | load | 4 | `abs 0x62bd4` | `-` |
| `ldr` at 0x62494 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62498 | load | 4 | `abs 0x1a5c9c` | `-` |
| `ldr` at 0x624a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x624ac | load | 4 | `abs 0x62bd8` | `-` |
| `ldr` at 0x624b0 | load | 4 | `abs 0x1a5c98` | `-` |
| `ldrb` at 0x624b8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x624c4 | load | 4 | `abs 0x62bc4` | `-` |
| `str` at 0x624c8 | store | 4 | `sp -0x60` | `0xfffffa85` |
| `ldr` at 0x624d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x624d8 | load | 4 | `abs 0x62bd8` | `-` |
| `ldr` at 0x624dc | load | 4 | `abs 0x1a5c98` | `-` |
| `str` at 0x624e4 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x624e8 | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x624ec | load | 1 | `[sp-76] +0x4` | `-` |
| `ldr` at 0x624f8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6250c | store | 4 | `sp -0x58` | `ret(0x6e1b0)` |
| `ldr` at 0x62510 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x62520 | store | 4 | `sp -0x54` | `ret(0x2031c)+1` |
| `ldr` at 0x62524 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x6254c | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x62550 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x62554 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x62558 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x62564 | load | 4 | `abs 0x62bdc` | `-` |
| `str` at 0x62568 | store | 4 | `(unknown)` | `0x1630b0` |
| `str` at 0x6256c | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `str` at 0x62574 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x62578 | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x6257c | load | 1 | `[sp-76] +0x4` | `-` |
| `str` at 0x62580 | store | 4 | `(unknown)` | `[[sp-76]+4]:1` |
| `ldr` at 0x62588 | load | 4 | `abs 0x62bcc` | `-` |
| `ldr` at 0x6258c | load | 4 | `abs 0x62be0` | `-` |
| `ldr` at 0x6259c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x625ac | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x625b0 | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x625b4 | load | 1 | `[sp-76] +0x4` | `-` |
| `ldr` at 0x625b8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x625c4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x625c8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x625dc | load | 4 | `[sp-76] +0x0` | `-` |
| `str` at 0x625e4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x625e8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x625ec | load | 1 | `(unknown)` | `-` |
| `str` at 0x625f0 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0x625f4 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x62600 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x6260c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x6261c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x6262c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x62630 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x62640 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x62644 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x62648 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62684 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x62688 | load | 1 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x62690 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x62698 | load | 1 | `[sp-32] +0x5` | `-` |
| `strb` at 0x626a0 | store | 1 | `sp -0x25` | `?` |
| `ldr` at 0x626a4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x626a8 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x626ac | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x626b0 | load | 1 | `[sp-32] +0x5` | `-` |
| `ldrb` at 0x626b4 | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0x626b8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x626d0 | store | 4 | `sp -0x2c` | `ret(0x125a58)` |
| `ldr` at 0x626d4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x626e0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x626f4 | store | 4 | `sp -0x58` | `ret(0x6e1b0)` |
| `ldr` at 0x626f8 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x62708 | store | 4 | `sp -0x54` | `ret(0x2031c)+1` |
| `ldr` at 0x6270c | load | 4 | `sp -0x54` | `-` |
| `str` at 0x62734 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x62738 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x6273c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x62740 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x6274c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x62750 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x62754 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62764 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x62770 | load | 4 | `abs 0x62be4` | `-` |
| `str` at 0x62774 | store | 4 | `(unknown)` | `0x1630d8` |
| `str` at 0x62778 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `str` at 0x6277c | store | 4 | `(unknown)` | `ret(0x1265c4)` |
| `str` at 0x62780 | store | 4 | `(unknown)` | `ret(0x21ce0)` |
| `ldr` at 0x62788 | load | 4 | `abs 0x62bcc` | `-` |
| `ldr` at 0x6278c | load | 4 | `abs 0x62be8` | `-` |
| `ldr` at 0x6279c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x627a0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x627a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x627c0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x627c4 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x627d8 | load | 4 | `sp -0x44` | `-` |
| `ldrb` at 0x627e0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x627ec | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x627f0 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x62804 | load | 4 | `sp -0x44` | `-` |
| `ldrb` at 0x6280c | load | 1 | `(unknown)` | `-` |
| `str` at 0x62810 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x62814 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x62818 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x6282c | load | 4 | `sp -0x44` | `-` |
| `ldrb` at 0x62834 | load | 1 | `(unknown)` | `-` |
| `str` at 0x62838 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x6283c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x62844 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6284c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x62850 | load | 1 | `[sp-32] +0x4` | `-` |
| `str` at 0x62858 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x6285c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62870 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x62874 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x62878 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x6287c | load | 1 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x62880 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x62884 | load | 1 | `[sp-32] +0x7` | `-` |
| `ldr` at 0x62888 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x62894 | store | 4 | `sp -0x2c` | `ret(0x11366c)` |
| `ldr` at 0x62898 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x628a4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x628b8 | store | 4 | `sp -0x58` | `ret(0x6e1b0)` |
| `ldr` at 0x628bc | load | 4 | `sp -0x58` | `-` |
| `str` at 0x628cc | store | 4 | `sp -0x54` | `ret(0x2031c)+1` |
| `ldr` at 0x628d0 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x628f8 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x628fc | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x62900 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x62904 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x62910 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x62914 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x62918 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62928 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x62934 | load | 4 | `abs 0x62bec` | `-` |
| `str` at 0x62938 | store | 4 | `(unknown)` | `0x16310c` |
| `str` at 0x6293c | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `str` at 0x62940 | store | 4 | `(unknown)` | `ret(0x1144ec)` |
| `str` at 0x62944 | store | 4 | `(unknown)` | `ret(0x21ce0)` |
| `ldr` at 0x6294c | load | 4 | `abs 0x62bcc` | `-` |
| `ldr` at 0x62950 | load | 4 | `abs 0x62bf0` | `-` |
| `ldr` at 0x62960 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x62974 | store | 4 | `sp -0x58` | `ret(0x6e1b0)` |
| `ldr` at 0x62978 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x62988 | store | 4 | `sp -0x54` | `ret(0x2031c)+1` |
| `ldr` at 0x6298c | load | 4 | `sp -0x54` | `-` |
| `str` at 0x629b4 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x629b8 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x629bc | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x629c0 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x629cc | load | 4 | `abs 0x62bf4` | `-` |
| `str` at 0x629d0 | store | 4 | `(unknown)` | `0x163138` |
| `str` at 0x629d4 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x629d8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x629dc | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x629e0 | store | 4 | `(unknown)` | `[[sp-32]]:1` |
| `ldr` at 0x629e8 | load | 4 | `abs 0x62bcc` | `-` |
| `ldr` at 0x629ec | load | 4 | `abs 0x62bf8` | `-` |
| `ldr` at 0x629f8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x62a00 | store | 4 | `sp -0x30` | `?` |
| `str` at 0x62a0c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x62a10 | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0x62a14 | load | 1 | `[sp-72] +0x4` | `-` |
| `ldr` at 0x62a18 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x62a24 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x62a28 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x62a3c | load | 4 | `[sp-72] +0x0` | `-` |
| `ldrb` at 0x62a44 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x62a58 | load | 4 | `sp -0x44` | `-` |
| `ldrb` at 0x62a60 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x62a70 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x62a74 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x62a88 | load | 4 | `[sp-72] +0x0` | `-` |
| `ldrb` at 0x62a90 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x62aa4 | load | 4 | `sp -0x44` | `-` |
| `ldrb` at 0x62aac | load | 1 | `(unknown)` | `-` |
| `str` at 0x62ab0 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x62ab4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x62ab8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x62acc | load | 4 | `[sp-72] +0x0` | `-` |
| `ldrb` at 0x62ad4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x62ae8 | load | 4 | `sp -0x44` | `-` |
| `ldrb` at 0x62af0 | load | 1 | `(unknown)` | `-` |
| `str` at 0x62af4 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x62af8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x62afc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x62b10 | load | 4 | `[sp-72] +0x0` | `-` |
| `ldrb` at 0x62b18 | load | 1 | `(unknown)` | `-` |
| `str` at 0x62b1c | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x62b20 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x62b24 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x62b38 | load | 4 | `[sp-72] +0x0` | `-` |
| `ldrb` at 0x62b40 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x62b54 | load | 4 | `sp -0x44` | `-` |
| `ldrh` at 0x62b5c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x62b60 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x62b70 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x62b74 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x62b7c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x62b84 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x62b88 | store | 4 | `(unknown)` | `[sp-92]` |
| `ldr` at 0x62b8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62b90 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x62b98 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x62ba0 | store | 4 | `sp -0x30` | `?` |
| `str` at 0x62bac | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0x62bb0 | load | 4 | `sp -0x60` | `-` |
| `ldm` at 0x62bb8 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x62bb8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x62bb8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x62bb8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x62bb8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x62400 | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x6240c | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x62454 | `(unknown callee) 0x203f4` | `[sp-88]` | `[sp-80]` | `[sp-84]` | `?` |
| `bl` at 0x6248c | `(unknown callee) 0x22070` | `0x9` | `0x162f88` | `0x10a` | `0x0` |
| `bl` at 0x62508 | `(unknown callee) 0x6e1b0` | `?` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x62514 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x6255c | `(unknown callee) 0x203f4` | `[sp-80]` | `[sp-88]` | `[sp-84]` | `?` |
| `bl` at 0x62594 | `(unknown callee) 0x22070` | `0x9` | `0x162f88` | `0x10b` | `0x0` |
| `bl` at 0x625a4 | `(unknown callee) 0x111db8` | `?` | `sp-68` | `?` | `sp-68` |
| `bl` at 0x62650 | `(unknown callee) 0x125958` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x62668 | `(unknown callee) 0x28230` | `0x4` | `?` | `?` | `ret(0x125958)` |
| `bl` at 0x6267c | `(unknown callee) 0x28230` | `0x5` | `?` | `?` | `ret(0x28230)` |
| `bl` at 0x626c8 | `(unknown callee) 0x125a58` | `[sp-28]` | `[[sp-32]+3]:1` | `[[sp-32]+5]:1` | `[sp-37]:1` |
| `bl` at 0x626f0 | `(unknown callee) 0x6e1b0` | `[sp-28]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x626fc | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x62744 | `(unknown callee) 0x203f4` | `[sp-80]` | `[sp-88]` | `[sp-84]` | `?` |
| `bl` at 0x6275c | `(unknown callee) 0x1265c4` | `[sp-28]` | `[[sp-32]+3]:1` | `?` | `[[sp-32]+3]:1` |
| `bl` at 0x62768 | `(unknown callee) 0x21ce0` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0x62794 | `(unknown callee) 0x22070` | `0x1` | `0x162f88` | `0x12d` | `0x0` |
| `bl` at 0x627ac | `(unknown callee) 0x111978` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x6286c | `(unknown callee) 0x1291a8` | `[sp-28]` | `?` | `?` | `[[sp-32]+4]:1` |
| `bl` at 0x6288c | `(unknown callee) 0x11366c` | `[sp-28]` | `[[sp-32]+3]:1` | `[[sp-32]+4]:1` | `[[sp-32]+7]:1` |
| `bl` at 0x628b4 | `(unknown callee) 0x6e1b0` | `[sp-28]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x628c0 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x62908 | `(unknown callee) 0x203f4` | `[sp-80]` | `[sp-88]` | `[sp-84]` | `?` |
| `bl` at 0x62920 | `(unknown callee) 0x1144ec` | `[sp-28]` | `[[sp-32]+3]:1` | `?` | `[[sp-32]+3]:1` |
| `bl` at 0x6292c | `(unknown callee) 0x21ce0` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0x62958 | `(unknown callee) 0x22070` | `0x1` | `0x162f88` | `0x145` | `0x0` |
| `bl` at 0x62970 | `(unknown callee) 0x6e1b0` | `[sp-28]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x6297c | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x629c4 | `(unknown callee) 0x203f4` | `[sp-80]` | `[sp-88]` | `[sp-84]` | `?` |
| `bl` at 0x629f4 | `(unknown callee) 0x22070` | `0x1` | `0x162f88` | `0x151` | `0x0` |
| `bl` at 0x62b94 | `(unknown callee) 0x129264` | `[sp-28]` | `?` | `?` | `[sp-80]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x62380 | `r0 >= [0x1a5c94] (signed, 32-bit)` | `r0 < [0x1a5c94] (signed, 32-bit)` |
| `bne` at 0x623ac | `? != 0x0` | `? == 0x0` |
| `bne` at 0x623ec | `[[sp-72]+4]:1 != 0x0` | `[[sp-72]+4]:1 == 0x0` |
| `bge` at 0x624a0 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x624c0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x624f4 | `[[sp-76]+4]:1 != 0x0` | `[[sp-76]+4]:1 == 0x0` |
| `ble` at 0x625c0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x625fc | `Z == 1` | `not (Z == 1)` |
| `bgt` at 0x62608 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x62614 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x62624 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x62638 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x6265c | `ret(0x125958) != 0x0` | `ret(0x125958) == 0x0` |
| `beq` at 0x62674 | `ret(0x28230) == 0x0` | `ret(0x28230) != 0x0` |
| `beq` at 0x626dc | `ret(0x125a58) == 0x0` | `ret(0x125a58) != 0x0` |
| `bne` at 0x627b8 | `ret(0x111978) != 0x0` | `ret(0x111978) == 0x0` |
| `bne` at 0x627e8 | `? != 0x6` | `? == 0x6` |
| `beq` at 0x628a0 | `ret(0x11366c) == 0x0` | `ret(0x11366c) != 0x0` |
| `ble` at 0x62a20 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x62a68 | `? == 0x6` | `? != 0x6` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x62bb8 | `r0` | `?` |

## `fanOn`

- `fanOn`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x62cc0 to 0x631ac.
  Established. The unnamed `0x125a0c` call carries `[sp-20]`, `[ret(0x645b0)+1]:1`, `[ret(0x645b0)+2]:1` and `[ret(0x645b0)+2]:1` (`bl` at 0x62f10);
  its zero result takes `0x6315c`, while nonzero falls through (`beq` at 0x62f24).
  A separate unnamed `0x11366c` call carries `[sp-20]`, `[ret(0x645b0)+3]:1`, `[ret(0x645b0)+4]:1` and `[ret(0x645b0)+7]:1` (`bl` at 0x62ff4).
  Fan-enable hardware behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x62cd0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x62cd8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x62cdc | load | 4 | `abs 0x63180` | `-` |
| `ldr` at 0x62ce0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62ce4 | load | 4 | `abs 0x1a5c94` | `-` |
| `ldr` at 0x62cf0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62d04 | load | 4 | `abs 0x63184` | `-` |
| `ldr` at 0x62d08 | load | 4 | `abs 0x1a5c90` | `-` |
| `ldrb` at 0x62d10 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x62d1c | load | 4 | `abs 0x63188` | `-` |
| `str` at 0x62d20 | store | 4 | `sp -0x3c` | `0xfffffa85` |
| `ldr` at 0x62d28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62d3c | load | 4 | `abs 0x63184` | `-` |
| `ldr` at 0x62d40 | load | 4 | `abs 0x1a5c90` | `-` |
| `str` at 0x62d48 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x62d4c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x62d50 | load | 1 | `[sp-44] +0x4` | `-` |
| `ldr` at 0x62d5c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x62d70 | store | 4 | `sp -0x30` | `ret(0x6e1b0)` |
| `ldr` at 0x62d74 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x62d84 | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x62d88 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x62db0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x62db4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x62db8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x62dbc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x62dc8 | load | 4 | `abs 0x6318c` | `-` |
| `str` at 0x62dcc | store | 4 | `(unknown)` | `0x163018` |
| `str` at 0x62dd0 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `str` at 0x62dd8 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x62ddc | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x62de0 | load | 1 | `[sp-44] +0x4` | `-` |
| `str` at 0x62de4 | store | 4 | `(unknown)` | `[[sp-44]+4]:1` |
| `ldr` at 0x62dec | load | 4 | `abs 0x63190` | `-` |
| `ldr` at 0x62df0 | load | 4 | `abs 0x63194` | `-` |
| `ldrb` at 0x62dfc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x62e08 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x62e1c | store | 4 | `sp -0x24` | `ret(0x646d4)` |
| `ldr` at 0x62e20 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x62e2c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x62e30 | store | 4 | `sp -0x3c` | `ret(0x646d4)` |
| `ldrb` at 0x62e38 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x62e3c | store | 1 | `sp -0x1f` | `([sp-29]:1 & 0xff)` |
| `ldrb` at 0x62e40 | load | 1 | `sp -0x1f` | `-` |
| `ldrb` at 0x62e44 | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x62e50 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x62e54 | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x62e58 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldrb` at 0x62e60 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x62e70 | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x62e74 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x62e84 | load | 1 | `ret(0x64480) +0x0` | `-` |
| `ldr` at 0x62e88 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x62e98 | store | 4 | `sp -0x1c` | `ret(0x645b0)` |
| `ldr` at 0x62e9c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x62ea0 | load | 1 | `ret(0x645b0) +0x0` | `-` |
| `str` at 0x62ea4 | store | 4 | `sp -0x40` | `[ret(0x645b0)]:1` |
| `ldr` at 0x62ea8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x62eb4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x62ec0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x62ed0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x62ee0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x62ee4 | load | 1 | `ret(0x645b0) +0x1` | `-` |
| `ldr` at 0x62ef4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x62ef8 | load | 1 | `ret(0x645b0) +0x1` | `-` |
| `ldr` at 0x62efc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x62f00 | load | 1 | `ret(0x645b0) +0x2` | `-` |
| `ldr` at 0x62f04 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x62f18 | store | 4 | `sp -0x28` | `ret(0x125a0c)` |
| `ldr` at 0x62f1c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62f28 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x62f3c | store | 4 | `sp -0x38` | `ret(0x6e1b0)` |
| `ldr` at 0x62f40 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x62f50 | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x62f54 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x62f7c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x62f80 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x62f84 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x62f88 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x62f94 | load | 4 | `abs 0x63198` | `-` |
| `str` at 0x62f98 | store | 4 | `(unknown)` | `0x16315c` |
| `str` at 0x62f9c | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x62fa0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x62fa4 | load | 1 | `[sp-28] +0x1` | `-` |
| `str` at 0x62fa8 | store | 4 | `(unknown)` | `[[sp-28]+1]:1` |
| `ldr` at 0x62fb0 | load | 4 | `abs 0x63190` | `-` |
| `ldr` at 0x62fb4 | load | 4 | `abs 0x6319c` | `-` |
| `ldr` at 0x62fc0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x62fcc | load | 4 | `sp -0x28` | `-` |
| `str` at 0x62fd0 | store | 4 | `sp -0x24` | `[sp-40]` |
| `ldr` at 0x62fd8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x62fdc | load | 1 | `ret(0x645b0) +0x3` | `-` |
| `ldr` at 0x62fe0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x62fe4 | load | 1 | `ret(0x645b0) +0x4` | `-` |
| `ldr` at 0x62fe8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x62fec | load | 1 | `ret(0x645b0) +0x7` | `-` |
| `ldr` at 0x62ff0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x62ffc | store | 4 | `sp -0x28` | `ret(0x11366c)` |
| `ldr` at 0x63000 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6300c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x63020 | store | 4 | `sp -0x38` | `ret(0x6e1b0)` |
| `ldr` at 0x63024 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x63034 | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x63038 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x63060 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x63064 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x63068 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6306c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x63078 | load | 4 | `abs 0x63198` | `-` |
| `str` at 0x6307c | store | 4 | `(unknown)` | `0x16315c` |
| `str` at 0x63080 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x63084 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x63088 | load | 1 | `[sp-28] +0x4` | `-` |
| `str` at 0x6308c | store | 4 | `(unknown)` | `[[sp-28]+4]:1` |
| `ldr` at 0x63094 | load | 4 | `abs 0x63190` | `-` |
| `ldr` at 0x63098 | load | 4 | `abs 0x631a0` | `-` |
| `ldr` at 0x630a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x630b0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x630b4 | store | 4 | `sp -0x24` | `[sp-40]` |
| `ldr` at 0x630bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x630d0 | store | 4 | `sp -0x38` | `ret(0x6e1b0)` |
| `ldr` at 0x630d4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x630e4 | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x630e8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x63110 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x63114 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x63118 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6311c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x63128 | load | 4 | `abs 0x631a4` | `-` |
| `str` at 0x6312c | store | 4 | `(unknown)` | `0x163180` |
| `str` at 0x63130 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x63134 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x63138 | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0x6313c | store | 4 | `(unknown)` | `[[sp-28]]:1` |
| `ldr` at 0x63144 | load | 4 | `abs 0x63190` | `-` |
| `ldr` at 0x63154 | load | 4 | `abs 0x631a8` | `-` |
| `str` at 0x63158 | store | 4 | `sp -0x24` | `0xfffffa86` |
| `ldrb` at 0x6315c | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0x63164 | store | 1 | `sp -0x1f` | `?` |
| `ldr` at 0x6316c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x63170 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x63174 | load | 4 | `sp -0x3c` | `-` |
| `ldm` at 0x6317c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6317c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6317c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x62d6c | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x62d78 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x62dc0 | `(unknown callee) 0x203f4` | `[sp-56]` | `[sp-48]` | `[sp-52]` | `?` |
| `bl` at 0x62df8 | `(unknown callee) 0x22070` | `0x9` | `0x162f88` | `0x18a` | `0x0` |
| `bl` at 0x62e14 | `(unknown callee) 0x646d4` | `?` | `?` | `sp-29` | `sp-30` |
| `bl` at 0x62e7c | `(unknown callee) 0x64480` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x62e90 | `(unknown callee) 0x645b0` | `[sp-20]` | `[ret(0x64480)]:1` | `?` | `[ret(0x64480)]:1` |
| `bl` at 0x62f10 | `(unknown callee) 0x125a0c` | `[sp-20]` | `[ret(0x645b0)+1]:1` | `[ret(0x645b0)+2]:1` | `[ret(0x645b0)+2]:1` |
| `bl` at 0x62f38 | `(unknown callee) 0x6e1b0` | `[sp-20]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x62f44 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x62f8c | `(unknown callee) 0x203f4` | `[sp-48]` | `[sp-56]` | `[sp-52]` | `?` |
| `bl` at 0x62fbc | `(unknown callee) 0x22070` | `0x1` | `0x162f88` | `0x19f` | `0x0` |
| `bl` at 0x62ff4 | `(unknown callee) 0x11366c` | `[sp-20]` | `[ret(0x645b0)+3]:1` | `[ret(0x645b0)+4]:1` | `[ret(0x645b0)+7]:1` |
| `bl` at 0x6301c | `(unknown callee) 0x6e1b0` | `[sp-20]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x63028 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x63070 | `(unknown callee) 0x203f4` | `[sp-48]` | `[sp-56]` | `[sp-52]` | `?` |
| `bl` at 0x630a0 | `(unknown callee) 0x22070` | `0x1` | `0x162f88` | `0x1ab` | `0x0` |
| `bl` at 0x630cc | `(unknown callee) 0x6e1b0` | `[sp-20]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x630d8 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x63120 | `(unknown callee) 0x203f4` | `[sp-48]` | `[sp-56]` | `[sp-52]` | `?` |
| `bl` at 0x63150 | `(unknown callee) 0x22070` | `0x1` | `0x162f88` | `0x1b8` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x62cec | `r0 >= [0x1a5c94] (signed, 32-bit)` | `r0 < [0x1a5c94] (signed, 32-bit)` |
| `bne` at 0x62d18 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x62d58 | `[[sp-44]+4]:1 != 0x0` | `[[sp-44]+4]:1 == 0x0` |
| `beq` at 0x62e28 | `ret(0x646d4) == 0x0` | `ret(0x646d4) != 0x0` |
| `bhs` at 0x62e4c | `C == 1` | `not (C == 1)` |
| `bne` at 0x62e68 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x62eb0 | `[ret(0x645b0)]:1 == 0x2` | `[ret(0x645b0)]:1 != 0x2` |
| `bgt` at 0x62ebc | `[ret(0x645b0)]:1 > 0x2 (signed, 32-bit)` | `[ret(0x645b0)]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0x62ec8 | `[ret(0x645b0)]:1 == 0x1` | `[ret(0x645b0)]:1 != 0x1` |
| `beq` at 0x62ed8 | `[ret(0x645b0)]:1 == 0x63` | `[ret(0x645b0)]:1 != 0x63` |
| `bne` at 0x62eec | `[ret(0x645b0)+1]:1 != 0xff` | `[ret(0x645b0)+1]:1 == 0xff` |
| `beq` at 0x62f24 | `ret(0x125a0c) == 0x0` | `ret(0x125a0c) != 0x0` |
| `bne` at 0x62fc8 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `beq` at 0x63008 | `ret(0x11366c) == 0x0` | `ret(0x11366c) != 0x0` |
| `bne` at 0x630ac | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6317c | `r0` | `?` |

## `fanOff`

- `fanOff`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x631ac to 0x6369c.
  Established. The unnamed `0x1259bc` call carries `[sp-20]`, `[ret(0x645b0)+1]:1`, `[ret(0x645b0)+2]:1` and `[ret(0x645b0)+2]:1` (`bl` at 0x633fc).
  Another unnamed `0x11366c` call carries `[sp-20]`, `[ret(0x645b0)+3]:1`, `[ret(0x645b0)+4]:1` and `0x0` (`bl` at 0x634e4).
  A word assignment places `0xfffffa86` at `sp -0x24` on another path (`str` at 0x63648).
  Fan-disable polarity and hardware effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x631bc | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x631c4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x631c8 | load | 4 | `abs 0x63670` | `-` |
| `ldr` at 0x631cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x631d0 | load | 4 | `abs 0x1a5c94` | `-` |
| `ldr` at 0x631dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x631f0 | load | 4 | `abs 0x63674` | `-` |
| `ldr` at 0x631f4 | load | 4 | `abs 0x1a5c90` | `-` |
| `ldrb` at 0x631fc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x63208 | load | 4 | `abs 0x63678` | `-` |
| `str` at 0x6320c | store | 4 | `sp -0x3c` | `0xfffffa85` |
| `ldr` at 0x63214 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x63228 | load | 4 | `abs 0x63674` | `-` |
| `ldr` at 0x6322c | load | 4 | `abs 0x1a5c90` | `-` |
| `str` at 0x63234 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x63238 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x6323c | load | 1 | `[sp-44] +0x4` | `-` |
| `ldr` at 0x63248 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6325c | store | 4 | `sp -0x30` | `ret(0x6e1b0)` |
| `ldr` at 0x63260 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x63270 | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x63274 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x6329c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x632a0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x632a4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x632a8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x632b4 | load | 4 | `abs 0x6367c` | `-` |
| `str` at 0x632b8 | store | 4 | `(unknown)` | `0x163018` |
| `str` at 0x632bc | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `str` at 0x632c4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x632c8 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x632cc | load | 1 | `[sp-44] +0x4` | `-` |
| `str` at 0x632d0 | store | 4 | `(unknown)` | `[[sp-44]+4]:1` |
| `ldr` at 0x632d8 | load | 4 | `abs 0x63680` | `-` |
| `ldrb` at 0x632e8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x632f4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x63308 | store | 4 | `sp -0x24` | `ret(0x646d4)` |
| `ldr` at 0x6330c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x63318 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6331c | store | 4 | `sp -0x3c` | `ret(0x646d4)` |
| `ldrb` at 0x63324 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x63328 | store | 1 | `sp -0x1f` | `([sp-29]:1 & 0xff)` |
| `ldrb` at 0x6332c | load | 1 | `sp -0x1f` | `-` |
| `ldrb` at 0x63330 | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x6333c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x63340 | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x63344 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldrb` at 0x6334c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x6335c | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x63360 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x63370 | load | 1 | `ret(0x64480) +0x0` | `-` |
| `ldr` at 0x63374 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x63384 | store | 4 | `sp -0x1c` | `ret(0x645b0)` |
| `ldr` at 0x63388 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x6338c | load | 1 | `ret(0x645b0) +0x0` | `-` |
| `str` at 0x63390 | store | 4 | `sp -0x40` | `[ret(0x645b0)]:1` |
| `ldr` at 0x63394 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x633a0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x633ac | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x633bc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x633cc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x633d0 | load | 1 | `ret(0x645b0) +0x1` | `-` |
| `ldr` at 0x633e0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x633e4 | load | 1 | `ret(0x645b0) +0x1` | `-` |
| `ldr` at 0x633e8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x633ec | load | 1 | `ret(0x645b0) +0x2` | `-` |
| `ldr` at 0x633f0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x63404 | store | 4 | `sp -0x28` | `ret(0x1259bc)` |
| `ldr` at 0x63408 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x63414 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x63428 | store | 4 | `sp -0x38` | `ret(0x6e1b0)` |
| `ldr` at 0x6342c | load | 4 | `sp -0x38` | `-` |
| `str` at 0x6343c | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x63440 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x63468 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x6346c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x63470 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x63474 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x63480 | load | 4 | `abs 0x63684` | `-` |
| `str` at 0x63484 | store | 4 | `(unknown)` | `0x1631a4` |
| `str` at 0x63488 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x6348c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x63490 | load | 1 | `[sp-28] +0x1` | `-` |
| `str` at 0x63494 | store | 4 | `(unknown)` | `[[sp-28]+1]:1` |
| `ldr` at 0x6349c | load | 4 | `abs 0x63680` | `-` |
| `ldr` at 0x634a0 | load | 4 | `abs 0x63688` | `-` |
| `ldr` at 0x634ac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x634b8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x634bc | store | 4 | `sp -0x24` | `[sp-40]` |
| `ldr` at 0x634c4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x634c8 | load | 1 | `ret(0x645b0) +0x3` | `-` |
| `ldr` at 0x634cc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x634d0 | load | 1 | `ret(0x645b0) +0x4` | `-` |
| `ldr` at 0x634d4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x634ec | store | 4 | `sp -0x28` | `ret(0x11366c)` |
| `ldr` at 0x634f0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x634fc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x63510 | store | 4 | `sp -0x38` | `ret(0x6e1b0)` |
| `ldr` at 0x63514 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x63524 | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x63528 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x63550 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x63554 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x63558 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6355c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x63568 | load | 4 | `abs 0x63684` | `-` |
| `str` at 0x6356c | store | 4 | `(unknown)` | `0x1631a4` |
| `str` at 0x63570 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x63574 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x63578 | load | 1 | `[sp-28] +0x3` | `-` |
| `str` at 0x6357c | store | 4 | `(unknown)` | `[[sp-28]+3]:1` |
| `ldr` at 0x63584 | load | 4 | `abs 0x63680` | `-` |
| `ldr` at 0x63588 | load | 4 | `abs 0x6368c` | `-` |
| `ldr` at 0x63594 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x635a0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x635a4 | store | 4 | `sp -0x24` | `[sp-40]` |
| `ldr` at 0x635ac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x635c0 | store | 4 | `sp -0x38` | `ret(0x6e1b0)` |
| `ldr` at 0x635c4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x635d4 | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x635d8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x63600 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x63604 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x63608 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6360c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x63618 | load | 4 | `abs 0x63690` | `-` |
| `str` at 0x6361c | store | 4 | `(unknown)` | `0x1631c8` |
| `str` at 0x63620 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x63624 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x63628 | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0x6362c | store | 4 | `(unknown)` | `[[sp-28]]:1` |
| `ldr` at 0x63634 | load | 4 | `abs 0x63680` | `-` |
| `ldr` at 0x63638 | load | 4 | `abs 0x63694` | `-` |
| `ldr` at 0x63644 | load | 4 | `abs 0x63698` | `-` |
| `str` at 0x63648 | store | 4 | `sp -0x24` | `0xfffffa86` |
| `ldrb` at 0x6364c | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0x63654 | store | 1 | `sp -0x1f` | `?` |
| `ldr` at 0x6365c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x63660 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x63664 | load | 4 | `sp -0x3c` | `-` |
| `ldm` at 0x6366c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6366c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6366c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x63258 | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x63264 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x632ac | `(unknown callee) 0x203f4` | `[sp-56]` | `[sp-48]` | `[sp-52]` | `?` |
| `bl` at 0x632e4 | `(unknown callee) 0x22070` | `0x9` | `0x162f88` | `0x1d0` | `0x0` |
| `bl` at 0x63300 | `(unknown callee) 0x646d4` | `?` | `?` | `sp-29` | `sp-30` |
| `bl` at 0x63368 | `(unknown callee) 0x64480` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x6337c | `(unknown callee) 0x645b0` | `[sp-20]` | `[ret(0x64480)]:1` | `?` | `[ret(0x64480)]:1` |
| `bl` at 0x633fc | `(unknown callee) 0x1259bc` | `[sp-20]` | `[ret(0x645b0)+1]:1` | `[ret(0x645b0)+2]:1` | `[ret(0x645b0)+2]:1` |
| `bl` at 0x63424 | `(unknown callee) 0x6e1b0` | `[sp-20]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x63430 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x63478 | `(unknown callee) 0x203f4` | `[sp-48]` | `[sp-56]` | `[sp-52]` | `?` |
| `bl` at 0x634a8 | `(unknown callee) 0x22070` | `0x1` | `0x162f88` | `0x1e2` | `0x0` |
| `bl` at 0x634e4 | `(unknown callee) 0x11366c` | `[sp-20]` | `[ret(0x645b0)+3]:1` | `[ret(0x645b0)+4]:1` | `0x0` |
| `bl` at 0x6350c | `(unknown callee) 0x6e1b0` | `[sp-20]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x63518 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x63560 | `(unknown callee) 0x203f4` | `[sp-48]` | `[sp-56]` | `[sp-52]` | `?` |
| `bl` at 0x63590 | `(unknown callee) 0x22070` | `0x1` | `0x162f88` | `0x1ee` | `0x0` |
| `bl` at 0x635bc | `(unknown callee) 0x6e1b0` | `[sp-20]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x635c8 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x63610 | `(unknown callee) 0x203f4` | `[sp-48]` | `[sp-56]` | `[sp-52]` | `?` |
| `bl` at 0x63640 | `(unknown callee) 0x22070` | `0x1` | `0x162f88` | `0x1fb` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x631d8 | `r0 >= [0x1a5c94] (signed, 32-bit)` | `r0 < [0x1a5c94] (signed, 32-bit)` |
| `bne` at 0x63204 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x63244 | `[[sp-44]+4]:1 != 0x0` | `[[sp-44]+4]:1 == 0x0` |
| `beq` at 0x63314 | `ret(0x646d4) == 0x0` | `ret(0x646d4) != 0x0` |
| `bhs` at 0x63338 | `C == 1` | `not (C == 1)` |
| `bne` at 0x63354 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x6339c | `[ret(0x645b0)]:1 == 0x2` | `[ret(0x645b0)]:1 != 0x2` |
| `bgt` at 0x633a8 | `[ret(0x645b0)]:1 > 0x2 (signed, 32-bit)` | `[ret(0x645b0)]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0x633b4 | `[ret(0x645b0)]:1 == 0x1` | `[ret(0x645b0)]:1 != 0x1` |
| `beq` at 0x633c4 | `[ret(0x645b0)]:1 == 0x63` | `[ret(0x645b0)]:1 != 0x63` |
| `bne` at 0x633d8 | `[ret(0x645b0)+1]:1 != 0xff` | `[ret(0x645b0)+1]:1 == 0xff` |
| `beq` at 0x63410 | `ret(0x1259bc) == 0x0` | `ret(0x1259bc) != 0x0` |
| `bne` at 0x634b4 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `beq` at 0x634f8 | `ret(0x11366c) == 0x0` | `ret(0x11366c) != 0x0` |
| `bne` at 0x6359c | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6366c | `r0` | `?` |

## `fanSetSpeedRaw`

- `fanSetSpeedRaw`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x639ec to 0x63d8c.
  Established. The unnamed `0x2624c` call carries `0x4`, `0x1` and two unknown candidates (`bl` at 0x63c54).
  A later unnamed `0x11366c` call carries `[sp-20]`, `[ret(0x645b0)+3]:1`, `[ret(0x645b0)+4]:1` and `[sp-22]:1` (`bl` at 0x63cf4).
  An included byte assignment writes an unknown value to `sp -0x20` (`strb` at 0x63c38).
  Raw speed units and device behavior remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x639fc | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x63a04 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x63a0c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0x63a10 | load | 4 | `abs 0x63d6c` | `-` |
| `ldr` at 0x63a14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x63a18 | load | 4 | `abs 0x1a5c94` | `-` |
| `ldr` at 0x63a24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x63a38 | load | 4 | `abs 0x63d70` | `-` |
| `ldr` at 0x63a3c | load | 4 | `abs 0x1a5c90` | `-` |
| `ldrb` at 0x63a44 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x63a50 | load | 4 | `abs 0x63d74` | `-` |
| `str` at 0x63a54 | store | 4 | `sp -0x3c` | `0xfffffa85` |
| `ldr` at 0x63a5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x63a70 | load | 4 | `abs 0x63d70` | `-` |
| `ldr` at 0x63a74 | load | 4 | `abs 0x1a5c90` | `-` |
| `str` at 0x63a7c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x63a80 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x63a84 | load | 1 | `[sp-44] +0x4` | `-` |
| `ldr` at 0x63a90 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x63aa4 | store | 4 | `sp -0x30` | `ret(0x6e1b0)` |
| `ldr` at 0x63aa8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x63ab8 | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x63abc | load | 4 | `sp -0x34` | `-` |
| `str` at 0x63ae4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x63ae8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x63aec | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x63af0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x63afc | load | 4 | `abs 0x63d78` | `-` |
| `str` at 0x63b00 | store | 4 | `(unknown)` | `0x163018` |
| `str` at 0x63b04 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `str` at 0x63b0c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x63b10 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x63b14 | load | 1 | `[sp-44] +0x4` | `-` |
| `str` at 0x63b18 | store | 4 | `(unknown)` | `[[sp-44]+4]:1` |
| `ldr` at 0x63b20 | load | 4 | `abs 0x63d7c` | `-` |
| `ldr` at 0x63b24 | load | 4 | `abs 0x63d80` | `-` |
| `ldrb` at 0x63b30 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x63b3c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x63b50 | store | 4 | `sp -0x24` | `ret(0x646d4)` |
| `ldr` at 0x63b54 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x63b60 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x63b64 | store | 4 | `sp -0x3c` | `ret(0x646d4)` |
| `ldrb` at 0x63b6c | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x63b70 | store | 1 | `sp -0x1f` | `([sp-29]:1 & 0xff)` |
| `ldrb` at 0x63b74 | load | 1 | `sp -0x1f` | `-` |
| `ldrb` at 0x63b78 | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x63b84 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x63b88 | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x63b8c | load | 4 | `[sp-44] +0x8` | `-` |
| `ldrb` at 0x63b94 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x63ba4 | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x63ba8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x63bb8 | load | 1 | `ret(0x64480) +0x0` | `-` |
| `ldr` at 0x63bbc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x63bcc | store | 4 | `sp -0x1c` | `ret(0x645b0)` |
| `ldr` at 0x63bd0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x63bd4 | load | 1 | `ret(0x645b0) +0x0` | `-` |
| `str` at 0x63bd8 | store | 4 | `sp -0x40` | `[ret(0x645b0)]:1` |
| `ldr` at 0x63bdc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x63be8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x63bf4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x63c04 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x63c14 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x63c18 | load | 1 | `ret(0x645b0) +0x3` | `-` |
| `ldr` at 0x63c24 | load | 4 | `abs 0x63d84` | `-` |
| `str` at 0x63c28 | store | 4 | `sp -0x3c` | `0xfffffa87` |
| `ldrb` at 0x63c30 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x63c38 | store | 1 | `sp -0x20` | `?` |
| `ldrb` at 0x63c3c | load | 1 | `sp -0x20` | `-` |
| `ldrb` at 0x63c58 | load | 1 | `sp -0x20` | `-` |
| `ldr` at 0x63c5c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x63c60 | load | 1 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x63c68 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x63c70 | load | 1 | `[sp-28] +0x5` | `-` |
| `strb` at 0x63c78 | store | 1 | `sp -0x20` | `?` |
| `ldr` at 0x63c7c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x63c80 | load | 1 | `[sp-28] +0x3` | `-` |
| `ldr` at 0x63c84 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x63c88 | load | 1 | `[sp-28] +0x5` | `-` |
| `ldrb` at 0x63c8c | load | 1 | `sp -0x20` | `-` |
| `ldr` at 0x63c90 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x63ca8 | store | 4 | `sp -0x28` | `ret(0x125a58)` |
| `ldr` at 0x63cac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x63cb8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x63cc4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x63cc8 | store | 4 | `sp -0x24` | `ret(0x125a58)` |
| `ldr` at 0x63cd0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x63cd4 | load | 1 | `ret(0x645b0) +0x3` | `-` |
| `ldr` at 0x63cd8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x63cdc | load | 1 | `ret(0x645b0) +0x4` | `-` |
| `ldrb` at 0x63ce0 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x63ce4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x63cfc | store | 4 | `sp -0x28` | `ret(0x11366c)` |
| `ldr` at 0x63d00 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x63d0c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x63d18 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x63d1c | store | 4 | `sp -0x24` | `ret(0x11366c)` |
| `ldrb` at 0x63d20 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x63d40 | load | 4 | `abs 0x63d88` | `-` |
| `str` at 0x63d44 | store | 4 | `sp -0x24` | `0xfffffa86` |
| `ldrb` at 0x63d48 | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0x63d50 | store | 1 | `sp -0x1f` | `?` |
| `ldr` at 0x63d58 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x63d5c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x63d60 | load | 4 | `sp -0x3c` | `-` |
| `ldm` at 0x63d68 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x63d68 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x63d68 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x63aa0 | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x63aac | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x63af4 | `(unknown callee) 0x203f4` | `[sp-56]` | `[sp-48]` | `[sp-52]` | `?` |
| `bl` at 0x63b2c | `(unknown callee) 0x22070` | `0x9` | `0x162f88` | `0x269` | `0x0` |
| `bl` at 0x63b48 | `(unknown callee) 0x646d4` | `?` | `?` | `sp-29` | `sp-30` |
| `bl` at 0x63bb0 | `(unknown callee) 0x64480` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x63bc4 | `(unknown callee) 0x645b0` | `[sp-20]` | `[ret(0x64480)]:1` | `?` | `[ret(0x64480)]:1` |
| `bl` at 0x63c48 | `(unknown callee) 0x2624c` | `0x5` | `[sp-32]:1` | `?` | `[sp-32]:1` |
| `bl` at 0x63c54 | `(unknown callee) 0x2624c` | `0x4` | `0x1` | `?` | `?` |
| `bl` at 0x63ca0 | `(unknown callee) 0x125a58` | `[sp-20]` | `[[sp-28]+3]:1` | `[[sp-28]+5]:1` | `[sp-32]:1` |
| `bl` at 0x63cf4 | `(unknown callee) 0x11366c` | `[sp-20]` | `[ret(0x645b0)+3]:1` | `[ret(0x645b0)+4]:1` | `[sp-22]:1` |
| `bl` at 0x63d2c | `(unknown callee) 0x2624c` | `0x5` | `[sp-22]:1` | `?` | `[sp-22]:1` |
| `bl` at 0x63d38 | `(unknown callee) 0x2624c` | `0x4` | `0x1` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x63a20 | `r0 >= [0x1a5c94] (signed, 32-bit)` | `r0 < [0x1a5c94] (signed, 32-bit)` |
| `bne` at 0x63a4c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x63a8c | `[[sp-44]+4]:1 != 0x0` | `[[sp-44]+4]:1 == 0x0` |
| `beq` at 0x63b5c | `ret(0x646d4) == 0x0` | `ret(0x646d4) != 0x0` |
| `bhs` at 0x63b80 | `C == 1` | `not (C == 1)` |
| `bne` at 0x63b9c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x63be4 | `[ret(0x645b0)]:1 == 0x2` | `[ret(0x645b0)]:1 != 0x2` |
| `bgt` at 0x63bf0 | `[ret(0x645b0)]:1 > 0x2 (signed, 32-bit)` | `[ret(0x645b0)]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0x63bfc | `[ret(0x645b0)]:1 == 0x1` | `[ret(0x645b0)]:1 != 0x1` |
| `beq` at 0x63c0c | `[ret(0x645b0)]:1 == 0x63` | `[ret(0x645b0)]:1 != 0x63` |
| `bne` at 0x63c20 | `[ret(0x645b0)+3]:1 != 0xff` | `[ret(0x645b0)+3]:1 == 0xff` |
| `beq` at 0x63cb4 | `ret(0x125a58) == 0x0` | `ret(0x125a58) != 0x0` |
| `bne` at 0x63cc0 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `beq` at 0x63d08 | `ret(0x11366c) == 0x0` | `ret(0x11366c) != 0x0` |
| `bne` at 0x63d14 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x63d68 | `r0` | `?` |

## `fanSetSpeed`

- `fanSetSpeed`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x63d8c to 0x640a8.
  Established. Included byte reads address `ret(0x645b0) +0x7`, `ret(0x645b0) +0x8` and `ret(0x645b0) +0x9` on separate paths (`ldrb` at 0x63ff8, `ldrb` at 0x64008, `ldrb` at 0x64018).
  Equality `[sp-22]:1 == 0x1` takes `0x64004`, while inequality falls through (`beq` at 0x63fc4).
  The unnamed `0x639ec` call carries `[sp-20]`, `[sp-32]:1`, unknown and `[sp-32]:1` (`bl` at 0x6403c).
  The level-to-speed hardware mapping remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x63d9c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x63da4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x63dac | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0x63db0 | load | 4 | `abs 0x6408c` | `-` |
| `ldr` at 0x63db4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x63db8 | load | 4 | `abs 0x1a5c94` | `-` |
| `ldr` at 0x63dc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x63dd8 | load | 4 | `abs 0x64090` | `-` |
| `ldr` at 0x63ddc | load | 4 | `abs 0x1a5c90` | `-` |
| `ldrb` at 0x63de4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x63df0 | load | 4 | `abs 0x64094` | `-` |
| `str` at 0x63df4 | store | 4 | `sp -0x3c` | `0xfffffa85` |
| `ldr` at 0x63dfc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x63e10 | load | 4 | `abs 0x64090` | `-` |
| `ldr` at 0x63e14 | load | 4 | `abs 0x1a5c90` | `-` |
| `str` at 0x63e1c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x63e20 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x63e24 | load | 1 | `[sp-44] +0x4` | `-` |
| `ldr` at 0x63e30 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x63e44 | store | 4 | `sp -0x30` | `ret(0x6e1b0)` |
| `ldr` at 0x63e48 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x63e58 | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x63e5c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x63e84 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x63e88 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x63e8c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x63e90 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x63e9c | load | 4 | `abs 0x64098` | `-` |
| `str` at 0x63ea0 | store | 4 | `(unknown)` | `0x163018` |
| `str` at 0x63ea4 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `str` at 0x63eac | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x63eb0 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x63eb4 | load | 1 | `[sp-44] +0x4` | `-` |
| `str` at 0x63eb8 | store | 4 | `(unknown)` | `[[sp-44]+4]:1` |
| `ldr` at 0x63ec0 | load | 4 | `abs 0x6409c` | `-` |
| `ldr` at 0x63ec4 | load | 4 | `abs 0x640a0` | `-` |
| `ldr` at 0x63ed0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x63edc | load | 4 | `abs 0x64094` | `-` |
| `str` at 0x63ee0 | store | 4 | `sp -0x3c` | `0xfffffa85` |
| `ldrb` at 0x63ee8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x63ef4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x63f08 | store | 4 | `sp -0x24` | `ret(0x646d4)` |
| `ldr` at 0x63f0c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x63f18 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x63f1c | store | 4 | `sp -0x3c` | `ret(0x646d4)` |
| `ldrb` at 0x63f24 | load | 1 | `sp -0x1e` | `-` |
| `strb` at 0x63f28 | store | 1 | `sp -0x20` | `([sp-30]:1 & 0xff)` |
| `ldrb` at 0x63f2c | load | 1 | `sp -0x20` | `-` |
| `ldrb` at 0x63f30 | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x63f3c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x63f40 | load | 1 | `sp -0x20` | `-` |
| `ldr` at 0x63f44 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldrb` at 0x63f4c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x63f5c | load | 1 | `sp -0x20` | `-` |
| `ldr` at 0x63f60 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x63f70 | load | 1 | `ret(0x64480) +0x0` | `-` |
| `ldr` at 0x63f74 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x63f84 | store | 4 | `sp -0x1c` | `ret(0x645b0)` |
| `ldr` at 0x63f88 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x63f8c | load | 1 | `ret(0x645b0) +0x3` | `-` |
| `ldrb` at 0x63f98 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x63fa8 | load | 4 | `abs 0x640a4` | `-` |
| `str` at 0x63fac | store | 4 | `sp -0x3c` | `0xfffffa87` |
| `ldrb` at 0x63fb4 | load | 1 | `sp -0x16` | `-` |
| `str` at 0x63fb8 | store | 4 | `sp -0x40` | `[sp-22]:1` |
| `ldr` at 0x63fbc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x63fc8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x63fd4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x63fe4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x63ff4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x63ff8 | load | 1 | `ret(0x645b0) +0x7` | `-` |
| `strb` at 0x63ffc | store | 1 | `sp -0x1d` | `([ret(0x645b0)+7]:1 & 0xff)` |
| `ldr` at 0x64004 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x64008 | load | 1 | `ret(0x645b0) +0x8` | `-` |
| `strb` at 0x6400c | store | 1 | `sp -0x1d` | `([ret(0x645b0)+8]:1 & 0xff)` |
| `ldr` at 0x64014 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x64018 | load | 1 | `ret(0x645b0) +0x9` | `-` |
| `strb` at 0x6401c | store | 1 | `sp -0x1d` | `([ret(0x645b0)+9]:1 & 0xff)` |
| `ldrb` at 0x64024 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x64028 | store | 1 | `sp -0x1d` | `([sp-22]:1 & 0xff)` |
| `ldrb` at 0x6402c | load | 1 | `sp -0x20` | `-` |
| `ldrb` at 0x64030 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x64034 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x64044 | store | 4 | `sp -0x28` | `ret(0x639ec)` |
| `ldr` at 0x64048 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x64054 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x64060 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x64064 | store | 4 | `sp -0x24` | `ret(0x639ec)` |
| `ldrb` at 0x64068 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0x64070 | store | 1 | `sp -0x20` | `?` |
| `ldr` at 0x64078 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6407c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x64080 | load | 4 | `sp -0x3c` | `-` |
| `ldm` at 0x64088 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x64088 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x64088 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x63e40 | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x63e4c | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x63e94 | `(unknown callee) 0x203f4` | `[sp-56]` | `[sp-48]` | `[sp-52]` | `?` |
| `bl` at 0x63ecc | `(unknown callee) 0x22070` | `0x9` | `0x162f88` | `0x2bb` | `0x0` |
| `bl` at 0x63f00 | `(unknown callee) 0x646d4` | `?` | `?` | `sp-30` | `sp-31` |
| `bl` at 0x63f68 | `(unknown callee) 0x64480` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x63f7c | `(unknown callee) 0x645b0` | `[sp-20]` | `[ret(0x64480)]:1` | `?` | `[ret(0x64480)]:1` |
| `bl` at 0x6403c | `(unknown callee) 0x639ec` | `[sp-20]` | `[sp-32]:1` | `?` | `[sp-32]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x63dc0 | `r0 >= [0x1a5c94] (signed, 32-bit)` | `r0 < [0x1a5c94] (signed, 32-bit)` |
| `bne` at 0x63dec | `? != 0x0` | `? == 0x0` |
| `bne` at 0x63e2c | `[[sp-44]+4]:1 != 0x0` | `[[sp-44]+4]:1 == 0x0` |
| `bne` at 0x63ed8 | `[sp-44] != 0x0` | `[sp-44] == 0x0` |
| `beq` at 0x63f14 | `ret(0x646d4) == 0x0` | `ret(0x646d4) != 0x0` |
| `bhs` at 0x63f38 | `C == 1` | `not (C == 1)` |
| `bne` at 0x63f54 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x63f94 | `[ret(0x645b0)+3]:1 != 0xff` | `[ret(0x645b0)+3]:1 == 0xff` |
| `bne` at 0x63fa0 | `[sp-21]:1 != 0xff` | `[sp-21]:1 == 0xff` |
| `beq` at 0x63fc4 | `[sp-22]:1 == 0x1` | `[sp-22]:1 != 0x1` |
| `bgt` at 0x63fd0 | `[sp-22]:1 > 0x1 (signed, 32-bit)` | `[sp-22]:1 <= 0x1 (signed, 32-bit)` |
| `beq` at 0x63fdc | `[sp-22]:1 == 0x0` | `[sp-22]:1 != 0x0` |
| `beq` at 0x63fec | `[sp-22]:1 == 0x2` | `[sp-22]:1 != 0x2` |
| `beq` at 0x64050 | `ret(0x639ec) == 0x0` | `ret(0x639ec) != 0x0` |
| `bne` at 0x6405c | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x64088 | `r0` | `?` |

## `fanGetSpeed`

- `fanGetSpeed`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x640a8 to 0x64238.
  Established. Equality `[ret(0x645b0)]:1 == 0x63` takes `0x641fc`, while inequality falls through (`beq` at 0x64160).
  That taken path clears the byte at `[sp-28] +0x0` (`strb` at 0x64204).
  The unnamed `0x113a3c` call carries `[sp-20]`, `[ret(0x645b0)+3]:1`, `[ret(0x645b0)+4]:1` and `[sp-28]` (`bl` at 0x641ec).
  Reported speed units and hardware sampling remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x640b8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x640c0 | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0x640c4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x640c8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x640cc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x640dc | store | 4 | `sp -0x20` | `ret(0x64480)` |
| `ldrb` at 0x640e0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x640e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x640fc | load | 4 | `abs 0x6422c` | `-` |
| `str` at 0x64100 | store | 4 | `sp -0x2c` | `0xfffffa85` |
| `ldr` at 0x64108 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x6410c | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x64110 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x64120 | store | 4 | `sp -0x24` | `ret(0x645b0)` |
| `ldr` at 0x64124 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x64128 | load | 1 | `ret(0x645b0) +0x0` | `-` |
| `str` at 0x6412c | store | 4 | `sp -0x30` | `[ret(0x645b0)]:1` |
| `ldr` at 0x64130 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x6413c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x64148 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x64158 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x64168 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x6416c | load | 1 | `ret(0x645b0) +0x3` | `-` |
| `ldr` at 0x64178 | load | 4 | `abs 0x64230` | `-` |
| `str` at 0x6417c | store | 4 | `sp -0x2c` | `0xfffffa87` |
| `ldr` at 0x64184 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x64188 | load | 1 | `ret(0x645b0) +0x3` | `-` |
| `ldr` at 0x6418c | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x6419c | store | 1 | `sp -0x25` | `(ret(0x125c20) & 0xff)` |
| `ldr` at 0x641a0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x641a4 | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0x641ac | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x641b0 | load | 1 | `[sp-36] +0x5` | `-` |
| `ldr` at 0x641b8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x641bc | load | 1 | `[sp-36] +0x4` | `-` |
| `strb` at 0x641c4 | store | 1 | `[sp-28] +0x0` | `?` |
| `ldr` at 0x641cc | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x641d0 | load | 1 | `ret(0x645b0) +0x3` | `-` |
| `ldr` at 0x641d4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x641d8 | load | 1 | `ret(0x645b0) +0x4` | `-` |
| `ldr` at 0x641dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x641e8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x641f4 | store | 4 | `sp -0x2c` | `ret(0x113a3c)` |
| `ldr` at 0x641fc | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x64204 | store | 1 | `[sp-28] +0x0` | `0x0` |
| `ldr` at 0x6420c | load | 4 | `abs 0x64234` | `-` |
| `str` at 0x64210 | store | 4 | `sp -0x2c` | `0xfffffa86` |
| `str` at 0x6421c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x64220 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x64228 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x64228 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x64228 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x640d4 | `(unknown callee) 0x64480` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x640ec | `(unknown callee) 0x621d4` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x64118 | `(unknown callee) 0x645b0` | `[sp-20]` | `[[sp-32]]:1` | `?` | `[[sp-32]]:1` |
| `bl` at 0x64194 | `(unknown callee) 0x125c20` | `[sp-20]` | `[ret(0x645b0)+3]:1` | `?` | `[ret(0x645b0)+3]:1` |
| `bl` at 0x641ec | `(unknown callee) 0x113a3c` | `[sp-20]` | `[ret(0x645b0)+3]:1` | `[ret(0x645b0)+4]:1` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x640f8 | `ret(0x621d4) != 0x0` | `ret(0x621d4) == 0x0` |
| `beq` at 0x64138 | `[ret(0x645b0)]:1 == 0x2` | `[ret(0x645b0)]:1 != 0x2` |
| `bgt` at 0x64144 | `[ret(0x645b0)]:1 > 0x2 (signed, 32-bit)` | `[ret(0x645b0)]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0x64150 | `[ret(0x645b0)]:1 == 0x1` | `[ret(0x645b0)]:1 != 0x1` |
| `beq` at 0x64160 | `[ret(0x645b0)]:1 == 0x63` | `[ret(0x645b0)]:1 != 0x63` |
| `bne` at 0x64174 | `[ret(0x645b0)+3]:1 != 0xff` | `[ret(0x645b0)+3]:1 == 0xff` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x64228 | `r0` | `?` |

## `fanGetReading`

- `fanGetReading`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x64238 to 0x64414.
  Established. The unnamed `0x1134bc` call carries `[sp-20]`, `[[sp-40]+1]:1`, `[[sp-40]+2]:1` and `[sp-28]` (`bl` at 0x6432c).
  Separate word assignments place `0x3f000000` and `0x40800000` at `sp -0x24` on different paths (`str` at 0x642e4, `str` at 0x64308).
  A coprocessor store to `sp -0x3c` has unknown width and value (`stc` at 0x6434c).
  The reading conversion and physical tachometer scale remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x64248 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x64250 | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0x64254 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x64258 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x6425c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6426c | store | 4 | `sp -0x28` | `ret(0x64480)` |
| `ldrb` at 0x64270 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x64274 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6428c | load | 4 | `abs 0x64400` | `-` |
| `str` at 0x64290 | store | 4 | `sp -0x2c` | `0xfffffa85` |
| `ldr` at 0x64298 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x6429c | load | 1 | `[sp-40] +0x8` | `-` |
| `str` at 0x642a4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x642a8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x642b4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x642c0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x642d0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x642e0 | load | 4 | `abs 0x64404` | `-` |
| `str` at 0x642e4 | store | 4 | `sp -0x24` | `0x3f000000` |
| `ldr` at 0x642ec | load | 4 | `abs 0x64408` | `-` |
| `str` at 0x642f0 | store | 4 | `sp -0x24` | `0x3f800000` |
| `ldr` at 0x642f8 | load | 4 | `abs 0x6440c` | `-` |
| `str` at 0x642fc | store | 4 | `sp -0x24` | `0x40000000` |
| `ldr` at 0x64304 | load | 4 | `abs 0x64410` | `-` |
| `str` at 0x64308 | store | 4 | `sp -0x24` | `0x40800000` |
| `ldr` at 0x6430c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x64310 | load | 1 | `[sp-40] +0x1` | `-` |
| `ldr` at 0x64314 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x64318 | load | 1 | `[sp-40] +0x2` | `-` |
| `ldr` at 0x6431c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x64328 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x64334 | store | 4 | `sp -0x20` | `ret(0x1134bc)` |
| `ldr` at 0x64338 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6433c | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0x64340 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x64344 | load | 4 | `[sp-28] +0x0` | `-` |
| `stc` at 0x6434c | store | unknown | `sp -0x3c` | `?` |
| `ldr` at 0x64350 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldc` at 0x6435c | load | unknown | `abs 0x643f4` | `-` |
| `ldc` at 0x64360 | load | unknown | `sp -0x3c` | `-` |
| `stc` at 0x64368 | store | unknown | `sp -0x3c` | `?` |
| `ldc` at 0x6436c | load | unknown | `sp -0x3c` | `-` |
| `ldc` at 0x64374 | load | unknown | `sp -0x24` | `-` |
| `stc` at 0x6437c | store | unknown | `sp -0x40` | `?` |
| `ldc` at 0x64380 | load | unknown | `abs 0x643fc` | `-` |
| `ldc` at 0x64384 | load | unknown | `sp -0x40` | `-` |
| `ldc` at 0x64390 | load | unknown | `sp -0x40` | `-` |
| `ldr` at 0x64398 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x6439c | store | 4 | `[sp-52] +0x0` | `?` |
| `ldc` at 0x643a4 | load | unknown | `abs 0x643fc` | `-` |
| `ldc` at 0x643a8 | load | unknown | `sp -0x40` | `-` |
| `ldr` at 0x643b4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x643b8 | store | 4 | `[sp-52] +0x0` | `?` |
| `ldr` at 0x643bc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x643c0 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x643c8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x643cc | store | 4 | `[sp-52] +0x0` | `?` |
| `ldr` at 0x643d0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x643d4 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x643d8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x643dc | store | 4 | `[sp-52] +0x0` | `[[sp-52]]` |
| `ldr` at 0x643e0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x643e4 | store | 4 | `sp -0x2c` | `[sp-32]` |
| `ldr` at 0x643e8 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x643f0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x643f0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x643f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x64264 | `(unknown callee) 0x64480` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x6427c | `(unknown callee) 0x621d4` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x6432c | `(unknown callee) 0x1134bc` | `[sp-20]` | `[[sp-40]+1]:1` | `[[sp-40]+2]:1` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x64288 | `ret(0x621d4) != 0x0` | `ret(0x621d4) == 0x0` |
| `beq` at 0x642b0 | `[sp-48] == 0x4` | `[sp-48] != 0x4` |
| `bgt` at 0x642bc | `[sp-48] > 0x4 (signed, 32-bit)` | `[sp-48] <= 0x4 (signed, 32-bit)` |
| `beq` at 0x642c8 | `[sp-48] == 0x1` | `[sp-48] != 0x1` |
| `beq` at 0x642d8 | `[sp-48] == 0x8` | `[sp-48] != 0x8` |
| `bge` at 0x64358 | `[[sp-28]] >= 0x0 (signed, 32-bit)` | `[[sp-28]] < 0x0 (signed, 32-bit)` |
| `bge` at 0x6438c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x643f0 | `r0` | `?` |
