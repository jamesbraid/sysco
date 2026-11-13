# ARM link, PIC, power and display helpers in 1.44.0

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
No unresolved computed transfers were reported for these routines.
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/link-pic-power-display-facts.txt) retain all tool sections.

## `nl4r_set_link_enable`

- `nl4r_set_link_enable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2ab3c to 0x2adc4.
  Established. The unnamed `0xe0540` call carries `[sp-20]`, `0x3`, `[sp-24]+10`, `0x0` (`bl` at 0x2ac2c).
  A signed nonnegative `[sp-36]` takes `0x2ac54`, while negative falls through (`bge` at 0x2ac44).
  Included word assignments write `0x1` and `0x0` to `sp -0x20` (`str` at 0x2ac70, `str` at 0x2ac98).
  Link register sequencing beyond the calls remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2ab4c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x2ab50 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x2ab54 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x2ab58 | load | 4 | `abs 0x2ada8` | `-` |
| `ldr` at 0x2ab5c | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x2ab68 | load | 4 | `abs 0x2adac` | `-` |
| `ldr` at 0x2ab6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2ab70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2ab74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2ab7c | load | 4 | `abs 0x2adb0` | `-` |
| `ldr` at 0x2ab80 | load | 4 | `abs 0x132630` | `-` |
| `ldr` at 0x2ab8c | load | 4 | `abs 0x2adac` | `-` |
| `str` at 0x2ab90 | store | 4 | `sp -0x44` | `0xf14f8` |
| `ldr` at 0x2ab94 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ab98 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x2ab9c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2aba0 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x2aba4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2aba8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x2abb0 | load | 4 | `abs 0x2adb4` | `-` |
| `ldr` at 0x2abb4 | load | 4 | `abs 0x2adb8` | `-` |
| `ldr` at 0x2abc0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2abc4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x2abc8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2abd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2abe8 | load | 4 | `abs 0x2ada8` | `-` |
| `ldr` at 0x2abec | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x2abf8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2ac00 | load | 4 | `abs 0x2adbc` | `-` |
| `ldr` at 0x2ac08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2ac14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2ac1c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ac34 | store | 4 | `sp -0x24` | `ret(0xe0540)` |
| `ldr` at 0x2ac3c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x2ac48 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x2ac4c | store | 4 | `sp -0x34` | `[sp-36]` |
| `ldr` at 0x2ac54 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2ac64 | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0x2ac70 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x2ac74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2ac80 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ac98 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x2ac9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2aca8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2acbc | load | 4 | `abs 0x2ada8` | `-` |
| `ldr` at 0x2acc0 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x2accc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2acd4 | load | 4 | `abs 0x2adc0` | `-` |
| `ldr` at 0x2acdc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2ace8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2acf0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ad08 | store | 4 | `sp -0x24` | `ret(0xe0540)` |
| `ldr` at 0x2ad10 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x2ad1c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x2ad20 | store | 4 | `sp -0x34` | `[sp-36]` |
| `str` at 0x2ad2c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x2ad30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2ad3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2ad54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2ad5c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ad74 | store | 4 | `sp -0x24` | `ret(0xe0540)` |
| `ldr` at 0x2ad7c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x2ad88 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x2ad8c | store | 4 | `sp -0x34` | `[sp-36]` |
| `str` at 0x2ad98 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x2ad9c | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x2ada4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x2ada4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x2ada4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2ab78 | `(unknown callee) 0x221f0` | `0xf14f8` | `r0` | `r1` | `r2` |
| `bl` at 0x2abbc | `(unknown callee) 0x200a0` | `0x6` | `0xf115c` | `0x34b` | `0x0` |
| `bl` at 0x2abe4 | `(unknown callee) 0xdfc34` | `?` | `?` | `0x1` | `sp-32` |
| `bl` at 0x2ac0c | `(unknown callee) 0x221f0` | `0xf1530` | `[sp-24]+65` | `[sp-28]` | `[sp-24]+65` |
| `bl` at 0x2ac10 | `(unknown callee) 0xd708c` | `?` | `?` | `?` | `?` |
| `bl` at 0x2ac2c | `(unknown callee) 0xe0540` | `[sp-20]` | `0x3` | `[sp-24]+10` | `0x0` |
| `bl` at 0x2ac38 | `(unknown callee) 0xd70fc` | `ret(0xe0540)` | `?` | `?` | `ret(0xe0540)` |
| `bl` at 0x2ac90 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+18` | `0x0` | `sp-32` |
| `bl` at 0x2acb8 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+18` | `0x2` | `sp-32` |
| `bl` at 0x2ace0 | `(unknown callee) 0x221f0` | `0xf1564` | `[sp-24]+65` | `[sp-28]` | `[sp-24]+65` |
| `bl` at 0x2ace4 | `(unknown callee) 0xd708c` | `?` | `?` | `?` | `?` |
| `bl` at 0x2ad00 | `(unknown callee) 0xe0540` | `[sp-20]` | `0x3` | `[sp-24]+18` | `0x0` |
| `bl` at 0x2ad0c | `(unknown callee) 0xd70fc` | `ret(0xe0540)` | `?` | `?` | `ret(0xe0540)` |
| `bl` at 0x2ad4c | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+18` | `0x0` | `sp-32` |
| `bl` at 0x2ad50 | `(unknown callee) 0xd708c` | `ret(0xdfc34)` | `?` | `?` | `?` |
| `bl` at 0x2ad6c | `(unknown callee) 0xe0540` | `[sp-20]` | `0x3` | `[sp-24]+18` | `0x0` |
| `bl` at 0x2ad78 | `(unknown callee) 0xd70fc` | `ret(0xe0540)` | `?` | `?` | `ret(0xe0540)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x2ab64 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `beq` at 0x2ab88 | `[0x132630] == 0x0` | `[0x132630] != 0x0` |
| `bls` at 0x2abf4 | `[0x29737c] <= 0x1 (unsigned, 32-bit)` | `[0x29737c] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x2ac44 | `[sp-36] >= 0x0 (signed, 32-bit)` | `[sp-36] < 0x0 (signed, 32-bit)` |
| `bne` at 0x2ac5c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bls` at 0x2acc8 | `[0x29737c] <= 0x1 (unsigned, 32-bit)` | `[0x29737c] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x2ad18 | `[sp-36] >= 0x0 (signed, 32-bit)` | `[sp-36] < 0x0 (signed, 32-bit)` |
| `bge` at 0x2ad84 | `[sp-36] >= 0x0 (signed, 32-bit)` | `[sp-36] < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x2ada4 | `r0` | `?` |

## `vortix_set_link_enable`

- `vortix_set_link_enable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x26df8 to 0x26ee8.
  Established. A nonzero `[0x132078]:1` takes `0x26e88`, while zero falls through (`bne` at 0x26e78).
  Included unnamed `0x2ab3c` and `0x47458` calls each carry entry `r0`, `r1`, `r2`, unknown (`bl` at 0x26ea8, `bl` at 0x26ec4).
  Link enable routing policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x26e08 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x26e0c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x26e10 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x26e14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x26e28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x26e3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x26e50 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x26e64 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x26e6c | load | 4 | `abs 0x26ee4` | `-` |
| `ldrb` at 0x26e70 | load | 1 | `abs 0x132078` | `-` |
| `str` at 0x26e80 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x26e88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x26e9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x26ea0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x26ea4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x26eb0 | store | 4 | `sp -0x20` | `ret(0x2ab3c)` |
| `ldr` at 0x26eb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x26ebc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x26ec0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x26ecc | store | 4 | `sp -0x20` | `ret(0x47458)` |
| `ldr` at 0x26ed0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x26ed4 | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0x26ed8 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x26ee0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x26ee0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x26ee0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x26ea8 | `(unknown callee) 0x2ab3c` | `r0` | `r1` | `r2` | `?` |
| `bl` at 0x26ec4 | `(unknown callee) 0x47458` | `r0` | `r1` | `r2` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x26e24 | `? == 0x6` | `? != 0x6` |
| `bne` at 0x26e38 | `? != 0x2` | `? == 0x2` |
| `bne` at 0x26e4c | `? != 0x2` | `? == 0x2` |
| `bgt` at 0x26e58 | `r1 > 0x1 (signed, 32-bit)` | `r1 <= 0x1 (signed, 32-bit)` |
| `bne` at 0x26e78 | `[0x132078]:1 != 0x0` | `[0x132078]:1 == 0x0` |
| `bne` at 0x26e98 | `? != 0x6` | `? == 0x6` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x26ee0 | `r0` | `?` |

## `NL4R_Init`

- `NL4R_Init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2906c to 0x291a4.
  Established. Included assignments write byte `0x1` to `0x13208c`, word `[sp-20]` to `0x132090` and word `[sp-24]` to `0x132094` (`strb` at 0x290b8, `str` at 0x290c4, `str` at 0x290d0).
  A zero `ret(0xdcfd0)` takes `0x29170`, while nonzero falls through (`beq` at 0x29128).
  Router setup callback responsibilities remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2907c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x29080 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x29084 | load | 4 | `abs 0x2918c` | `-` |
| `ldrb` at 0x29088 | load | 1 | `abs 0x13208c` | `-` |
| `ldr` at 0x29094 | load | 4 | `abs 0x29190` | `-` |
| `str` at 0x29098 | store | 4 | `sp -0x2c` | `0xf1134` |
| `ldr` at 0x290a0 | load | 4 | `abs 0x29194` | `-` |
| `ldr` at 0x290b0 | load | 4 | `abs 0x2918c` | `-` |
| `strb` at 0x290b8 | store | 1 | `abs 0x13208c` | `0x1` |
| `ldr` at 0x290bc | load | 4 | `abs 0x29198` | `-` |
| `ldr` at 0x290c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x290c4 | store | 4 | `abs 0x132090` | `[sp-20]` |
| `ldr` at 0x290c8 | load | 4 | `abs 0x2919c` | `-` |
| `ldr` at 0x290cc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x290d0 | store | 4 | `abs 0x132094` | `[sp-24]` |
| `str` at 0x290d8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x290dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x290e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x290ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x290f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x290fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x29104 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2910c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x29110 | load | 4 | `(unknown)` | `-` |
| `str` at 0x2911c | store | 4 | `sp -0x20` | `ret(0xdcfd0)` |
| `ldr` at 0x29120 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2912c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x29138 | load | 4 | `abs 0x291a0` | `-` |
| `str` at 0x2913c | store | 4 | `sp -0x2c` | `0xf1170` |
| `ldr` at 0x29140 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x29148 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29150 | load | 4 | `(unknown)` | `-` |
| `str` at 0x29154 | store | 4 | `sp -0x28` | `?` |
| `str` at 0x29158 | store | 4 | `sp -0x24` | `ret(0x1f8e0)` |
| `ldr` at 0x29160 | load | 4 | `abs 0x29194` | `-` |
| `ldr` at 0x29170 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x29178 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldm` at 0x29188 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x29188 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x29188 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x290ac | `(unknown callee) 0x200a0` | `0x9` | `0xf115c` | `0x73` | `0x0` |
| `bl` at 0x29114 | `(unknown callee) 0xdcfd0` | `?` | `?` | `?` | `?` |
| `bl` at 0x29130 | `(unknown callee) 0x1f8e0` | `ret(0xdcfd0)` | `?` | `?` | `ret(0xdcfd0)` |
| `bl` at 0x2916c | `(unknown callee) 0x200a0` | `0x9` | `0xf115c` | `0x83` | `0x0` |
| `bl` at 0x29180 | `(unknown callee) 0x35cec` | `?` | `?` | `?` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x29090 | `[0x13208c]:1 == 0x0` | `[0x13208c]:1 != 0x0` |
| `bge` at 0x290e8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x29128 | `ret(0xdcfd0) == 0x0` | `ret(0xdcfd0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x29188 | `r0` | `ret(0x35cec)` |

## `nl4r_is_scramble_needed`

- `nl4r_is_scramble_needed`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2adc4 to 0x2ae94.
  Established. Word reads address `ret(0x6a328) +0x10` and `ret(0x6a328) +0x18` (`ldr` at 0x2ae4c, `ldr` at 0x2ae68).
  A nonzero `[ret(0x6a328)+24]` takes `0x2ae80`, while zero falls through (`bne` at 0x2ae70).
  The unnamed `0x1d444` call carries `0x98` followed by three unknown candidates (`bl` at 0x2ae00).
  Scrambling requirement meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2add4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x2add8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x2adf4 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x2ae04 | store | 4 | `sp -0x20` | `ret(0x1d444)` |
| `ldr` at 0x2ae08 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2ae0c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2ae28 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x2ae30 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2ae44 | store | 4 | `sp -0x24` | `ret(0x6a328)` |
| `ldr` at 0x2ae48 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x2ae4c | load | 4 | `ret(0x6a328) +0x10` | `-` |
| `str` at 0x2ae5c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x2ae64 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x2ae68 | load | 4 | `ret(0x6a328) +0x18` | `-` |
| `str` at 0x2ae78 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x2ae84 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x2ae88 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x2ae90 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x2ae90 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x2ae90 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2addc | `(unknown callee) 0x488cc` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x2ae00 | `(unknown callee) 0x1d444` | `0x98` | `?` | `?` | `?` |
| `bl` at 0x2ae3c | `(unknown callee) 0x6a328` | `[sp-24]+1` | `?` | `ret(0x1d444)` | `[sp-24]+1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x2adec | `? == 0x44` | `? != 0x44` |
| `beq` at 0x2ae20 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x2ae54 | `[ret(0x6a328)+16] == 0x44` | `[ret(0x6a328)+16] != 0x44` |
| `bne` at 0x2ae70 | `[ret(0x6a328)+24] != 0x0` | `[ret(0x6a328)+24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x2ae90 | `r0` | `?` |

## `nl4r_is_connected_to_nl4r`

- `nl4r_is_connected_to_nl4r`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2ae94 to 0x2af90.
  Established. The unsigned condition `[sp-24] > 0x4` takes `0x2aeec`, while lower or equal falls through (`bhi` at 0x2aedc).
  Included word reads address `ret(0x6a328) +0x10` (`ldr` at 0x2af50, `ldr` at 0x2af60).
  Equality `[ret(0x6a328)+16] == 0x44` takes `0x2af70`, while inequality falls through (`beq` at 0x2af58).
  Link peer identity encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2aea4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x2aea8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x2aeac | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2aeb0 | store | 4 | `sp -0x24` | `r1` |
| `ldr` at 0x2aec8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2aed4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2aee4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x2aeec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2aef8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2af00 | store | 4 | `sp -0x2c` | `[sp-24]-5` |
| `str` at 0x2af0c | store | 4 | `sp -0x2c` | `0x3` |
| `ldr` at 0x2af10 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x2af14 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x2af18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2af28 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2af30 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x2af34 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x2af48 | store | 4 | `sp -0x20` | `ret(0x6a328)` |
| `ldr` at 0x2af4c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2af50 | load | 4 | `ret(0x6a328) +0x10` | `-` |
| `ldr` at 0x2af5c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2af60 | load | 4 | `ret(0x6a328) +0x10` | `-` |
| `str` at 0x2af74 | store | 4 | `sp -0x28` | `0x1` |
| `str` at 0x2af80 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x2af84 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x2af8c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x2af8c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x2af8c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2aeb4 | `(unknown callee) 0x488cc` | `r0` | `r1` | `r2` | `r1` |
| `bl` at 0x2af40 | `(unknown callee) 0x6a328` | `[sp-36]+1` | `?` | `?` | `[sp-36]+1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x2aec4 | `? != 0x40` | `? == 0x40` |
| `beq` at 0x2aed0 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `bhi` at 0x2aedc | `[sp-24] > 0x4 (unsigned, 32-bit)` | `[sp-24] <= 0x4 (unsigned, 32-bit)` |
| `beq` at 0x2aef4 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0x2af58 | `[ret(0x6a328)+16] == 0x44` | `[ret(0x6a328)+16] != 0x44` |
| `beq` at 0x2af68 | `[ret(0x6a328)+16] == 0x40` | `[ret(0x6a328)+16] != 0x40` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x2af8c | `r0` | `?` |

## `PICInit`

- `PICInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x34350 to 0x345d8.
  Established. The unnamed `0x4f81c` call carries `0x0`, `0x34aa8`, `0x34ad0`, `0x34b24` (`bl` at 0x343c0).
  Included assignments write word `[sp-20]` to `0x132128` and byte `0x1` to `0x13212d` (`str` at 0x3449c, `strb` at 0x344bc).
  The unnamed `0xca70c` call carries `0x0`, `[[sp-28]]:1`, `sp-29`, `[[sp-28]]:1` (`bl` at 0x3451c).
  PIC setup callback ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x34360 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x34364 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x34368 | load | 4 | `abs 0x345b0` | `-` |
| `ldr` at 0x3436c | load | 4 | `abs 0x132128` | `-` |
| `ldr` at 0x34378 | load | 4 | `abs 0x345b4` | `-` |
| `str` at 0x3437c | store | 4 | `sp -0x34` | `0xf500c` |
| `ldr` at 0x34380 | load | 4 | `abs 0x345b8` | `-` |
| `ldrb` at 0x34384 | load | 1 | `abs 0x13212c` | `-` |
| `str` at 0x34388 | store | 4 | `sp -0x30` | `[0x13212c]:1` |
| `ldr` at 0x34390 | load | 4 | `abs 0x345bc` | `-` |
| `str` at 0x343a4 | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0x343ac | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x343b4 | load | 4 | `abs 0x345c0` | `-` |
| `ldr` at 0x343b8 | load | 4 | `abs 0x345c4` | `-` |
| `ldr` at 0x343bc | load | 4 | `abs 0x345c8` | `-` |
| `str` at 0x343c8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x343cc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x343d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x343dc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x343f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x343f8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3440c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x34414 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x34418 | load | 4 | `(unknown)` | `-` |
| `str` at 0x34424 | store | 4 | `sp -0x24` | `ret(0xdcfd0)` |
| `ldr` at 0x34428 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x34434 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x34440 | load | 4 | `abs 0x345cc` | `-` |
| `str` at 0x34444 | store | 4 | `sp -0x34` | `0xf503c` |
| `ldr` at 0x34448 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3445c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x34464 | load | 4 | `(unknown)` | `-` |
| `str` at 0x34468 | store | 4 | `sp -0x30` | `?` |
| `str` at 0x3446c | store | 4 | `sp -0x2c` | `ret(0x1f8e0)` |
| `ldr` at 0x34474 | load | 4 | `abs 0x345bc` | `-` |
| `ldr` at 0x34484 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3448c | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x34494 | load | 4 | `abs 0x345b0` | `-` |
| `ldr` at 0x34498 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3449c | store | 4 | `abs 0x132128` | `[sp-20]` |
| `ldr` at 0x344a0 | load | 4 | `abs 0x345b8` | `-` |
| `ldr` at 0x344a4 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x344a8 | store | 1 | `abs 0x13212c` | `([sp-24] & 0xff)` |
| `str` at 0x344b0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x344b4 | load | 4 | `abs 0x345d0` | `-` |
| `strb` at 0x344bc | store | 1 | `abs 0x13212d` | `0x1` |
| `ldr` at 0x344c0 | load | 4 | `abs 0x345b8` | `-` |
| `ldrb` at 0x344c4 | load | 1 | `abs 0x13212c` | `-` |
| `ldr` at 0x344c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x344d4 | load | 4 | `abs 0x345d0` | `-` |
| `ldrb` at 0x344d8 | load | 1 | `abs 0x13212d` | `-` |
| `ldr` at 0x344e4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x344f8 | load | 4 | `abs 0x345b0` | `-` |
| `ldr` at 0x344fc | load | 4 | `abs 0x132128` | `-` |
| `str` at 0x34504 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x34508 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3450c | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0x34524 | store | 4 | `sp -0x24` | `ret(0xca70c)` |
| `ldr` at 0x34528 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x34534 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x34540 | load | 4 | `abs 0x345d4` | `-` |
| `str` at 0x34544 | store | 4 | `sp -0x34` | `0xf5058` |
| `str` at 0x34548 | store | 4 | `sp -0x30` | `ret(0x1fc1c)` |
| `ldr` at 0x34550 | load | 4 | `abs 0x345bc` | `-` |
| `ldr` at 0x34560 | load | 4 | `abs 0x345d0` | `-` |
| `strb` at 0x34568 | store | 1 | `abs 0x13212d` | `0x0` |
| `ldr` at 0x34570 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x34574 | load | 1 | `[sp-28] +0x1` | `-` |
| `ldrb` at 0x34578 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x3458c | load | 4 | `abs 0x345d0` | `-` |
| `strb` at 0x34594 | store | 1 | `abs 0x13212d` | `0x0` |
| `ldr` at 0x34598 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x345a0 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldm` at 0x345ac | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x345ac | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x345ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3439c | `(unknown callee) 0x200a0` | `0x9` | `0xf5028` | `0x3b` | `0x0` |
| `bl` at 0x343c0 | `(unknown callee) 0x4f81c` | `0x0` | `0x34aa8` | `0x34ad0` | `0x34b24` |
| `bl` at 0x3441c | `(unknown callee) 0xdcfd0` | `?` | `?` | `?` | `?` |
| `bl` at 0x34438 | `(unknown callee) 0x1f8e0` | `ret(0xdcfd0)` | `?` | `?` | `ret(0xdcfd0)` |
| `bl` at 0x34480 | `(unknown callee) 0x200a0` | `0x1` | `0xf5028` | `0x49` | `0x0` |
| `bl` at 0x3451c | `(unknown callee) 0xca70c` | `0x0` | `[[sp-28]]:1` | `sp-29` | `[[sp-28]]:1` |
| `bl` at 0x34538 | `(unknown callee) 0x1fc1c` | `ret(0xca70c)` | `?` | `?` | `ret(0xca70c)` |
| `bl` at 0x3455c | `(unknown callee) 0x200a0` | `0x1` | `0xf5028` | `0x5a` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x34374 | `[0x132128] == 0x0` | `[0x132128] != 0x0` |
| `bge` at 0x343d8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x34430 | `ret(0xdcfd0) == 0x0` | `ret(0xdcfd0) != 0x0` |
| `ble` at 0x344d0 | `[0x13212c]:1 <= [sp-40] (signed, 32-bit)` | `[0x13212c]:1 > [sp-40] (signed, 32-bit)` |
| `beq` at 0x344e0 | `[0x13212d]:1 == 0x0` | `[0x13212d]:1 != 0x0` |
| `beq` at 0x34530 | `ret(0xca70c) == 0x0` | `ret(0xca70c) != 0x0` |
| `bne` at 0x34588 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x345ac | `r0` | `?` |

## `LinkErrQuery`

- `LinkErrQuery`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x35698 to 0x35cec.
  Established. The unnamed `0x1943c` call carries `0x5dc`, entry `r1`, entry `r2`, entry `r3` (`bl` at 0x356b4);
  its result is written as a word to `r0 +0x0` (`str` at 0x356bc).
  A byte assignment writes `0x0` to `[[sp-24]] +0x0` (`strb` at 0x356d4).
  A zero `ret(0x1d444)` takes `0x35714`, while nonzero falls through (`beq` at 0x35704).
  Link-error report field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x356a8 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x356ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x356b0 | load | 4 | `abs 0x35b38` | `-` |
| `str` at 0x356bc | store | 4 | `r0 +0x0` | `ret(0x1943c)` |
| `ldr` at 0x356c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x356c4 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x356c8 | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `ldr` at 0x356cc | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x356d4 | store | 1 | `[[sp-24]] +0x0` | `0x0` |
| `ldr` at 0x356d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x356e8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x356f0 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x35708 | load | 4 | `abs 0x35b3c` | `-` |
| `str` at 0x3570c | store | 4 | `sp -0x48` | `0xf5134` |
| `ldr` at 0x35714 | load | 4 | `abs 0x35b40` | `-` |
| `str` at 0x35718 | store | 4 | `sp -0x48` | `0xf513c` |
| `ldr` at 0x3571c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x35720 | load | 4 | `abs 0x35b44` | `-` |
| `ldr` at 0x35724 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3572c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3573c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x35744 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x35748 | load | 4 | `abs 0x35b48` | `-` |
| `ldr` at 0x3574c | load | 4 | `abs 0x12d6ec` | `-` |
| `ldr` at 0x35758 | load | 4 | `abs 0x35b4c` | `-` |
| `str` at 0x3575c | store | 4 | `sp -0x4c` | `0xf5164` |
| `ldr` at 0x35764 | load | 4 | `abs 0x35b50` | `-` |
| `str` at 0x35768 | store | 4 | `sp -0x4c` | `0xf516c` |
| `ldr` at 0x3576c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x35770 | load | 4 | `abs 0x35b54` | `-` |
| `ldr` at 0x35774 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x3577c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3578c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x35794 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x35798 | load | 4 | `abs 0x35b58` | `-` |
| `ldr` at 0x3579c | load | 4 | `abs 0x12d6f0` | `-` |
| `stc` at 0x357b4 | store | unknown | `sp -0x24` | `?` |
| `ldr` at 0x357b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x357bc | load | 4 | `abs 0x35b5c` | `-` |
| `ldr` at 0x357c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x357d4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x357dc | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldc` at 0x357e0 | load | unknown | `sp -0x24` | `-` |
| `ldc` at 0x357e4 | load | unknown | `abs 0x35b30` | `-` |
| `ldc` at 0x357f4 | load | unknown | `sp -0x24` | `-` |
| `ldc` at 0x357f8 | load | unknown | `abs 0x35b30` | `-` |
| `ldr` at 0x35804 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x35808 | load | 4 | `abs 0x35b60` | `-` |
| `stc` at 0x3580c | store | unknown | `sp -0x64` | `?` |
| `ldc` at 0x3581c | load | unknown | `sp -0x24` | `-` |
| `ldc` at 0x35820 | load | unknown | `abs 0x35b34` | `-` |
| `ldc` at 0x35830 | load | unknown | `sp -0x24` | `-` |
| `ldc` at 0x35834 | load | unknown | `abs 0x35b34` | `-` |
| `ldr` at 0x35840 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x35844 | load | 4 | `abs 0x35b64` | `-` |
| `stc` at 0x35848 | store | unknown | `sp -0x64` | `?` |
| `ldc` at 0x35858 | load | unknown | `sp -0x24` | `-` |
| `ldr` at 0x35860 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x35864 | load | 4 | `abs 0x35b68` | `-` |
| `stc` at 0x35868 | store | unknown | `sp -0x64` | `?` |
| `ldr` at 0x35874 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x35884 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3588c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x35890 | load | 4 | `abs 0x35b58` | `-` |
| `ldr` at 0x35894 | load | 4 | `abs 0x35b6c` | `-` |
| `ldr` at 0x35898 | load | 4 | `abs 0x12d6f0` | `-` |
| `ldr` at 0x3589c | load | 4 | `abs 0x12d6f4` | `-` |
| `stc` at 0x358b8 | store | unknown | `sp -0x28` | `?` |
| `ldr` at 0x358bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x358c0 | load | 4 | `abs 0x35b70` | `-` |
| `ldr` at 0x358c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x358d8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x358e0 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldc` at 0x358e4 | load | unknown | `sp -0x28` | `-` |
| `ldc` at 0x358e8 | load | unknown | `abs 0x35b30` | `-` |
| `ldc` at 0x358f8 | load | unknown | `sp -0x28` | `-` |
| `ldc` at 0x358fc | load | unknown | `abs 0x35b30` | `-` |
| `ldr` at 0x35908 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3590c | load | 4 | `abs 0x35b60` | `-` |
| `stc` at 0x35910 | store | unknown | `(unknown)` | `?` |
| `ldc` at 0x35920 | load | unknown | `sp -0x28` | `-` |
| `ldc` at 0x35924 | load | unknown | `abs 0x35b34` | `-` |
| `ldc` at 0x35934 | load | unknown | `sp -0x28` | `-` |
| `ldc` at 0x35938 | load | unknown | `abs 0x35b34` | `-` |
| `ldr` at 0x35944 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x35948 | load | 4 | `abs 0x35b64` | `-` |
| `stc` at 0x3594c | store | unknown | `(unknown)` | `?` |
| `ldc` at 0x3595c | load | unknown | `sp -0x28` | `-` |
| `ldc` at 0x3596c | load | unknown | `sp -0x24` | `-` |
| `ldc` at 0x35970 | load | unknown | `abs 0x35b30` | `-` |
| `ldr` at 0x3597c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x35980 | load | 4 | `abs 0x35b60` | `-` |
| `stc` at 0x35984 | store | unknown | `(unknown)` | `?` |
| `ldc` at 0x35994 | load | unknown | `sp -0x28` | `-` |
| `ldr` at 0x3599c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x359a0 | load | 4 | `abs 0x35b68` | `-` |
| `stc` at 0x359a4 | store | unknown | `(unknown)` | `?` |
| `ldr` at 0x359b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x359c0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x359c8 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x359cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x359d0 | load | 4 | `abs 0x35b74` | `-` |
| `ldr` at 0x359d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x359e8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x359f0 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x359f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x359f8 | load | 4 | `abs 0x35b78` | `-` |
| `ldr` at 0x35a00 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x35a10 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x35a18 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x35a1c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x35a20 | load | 4 | `abs 0x35b7c` | `-` |
| `ldr` at 0x35a28 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x35a38 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x35a40 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x35a44 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x35a48 | load | 4 | `abs 0x35b80` | `-` |
| `ldr` at 0x35a50 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x35a60 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x35a68 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `str` at 0x35a70 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x35a74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x35a80 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x35a98 | load | 1 | `sp -0x44` | `-` |
| `ldr` at 0x35aa4 | load | 4 | `abs 0x35ba4` | `-` |
| `str` at 0x35aa8 | store | 4 | `(unknown)` | `0xf537c` |
| `ldr` at 0x35aac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x35ab0 | load | 4 | `abs 0x35b84` | `-` |
| `ldr` at 0x35ab4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x35ab8 | load | 4 | `abs 0x35ba4` | `-` |
| `ldrb` at 0x35ac4 | load | 1 | `sp -0x43` | `-` |
| `ldrb` at 0x35ac8 | load | 1 | `sp -0x42` | `-` |
| `str` at 0x35acc | store | 4 | `(unknown)` | `[sp-66]:1` |
| `ldr` at 0x35ad0 | load | 4 | `abs 0x35b88` | `-` |
| `ldr` at 0x35ad4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x35ad8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x35adc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x35ae0 | load | 4 | `abs 0x35b8c` | `-` |
| `ldr` at 0x35ae4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x35ae8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x35aec | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x35af0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x35af4 | load | 4 | `abs 0x35b90` | `-` |
| `ldr` at 0x35af8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x35b04 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x35b14 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x35b1c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x35b20 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x35b28 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x35bac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x35bb0 | load | 4 | `abs 0x35b94` | `-` |
| `ldr` at 0x35bb8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x35bc8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x35bd0 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x35bd4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x35bd8 | load | 4 | `abs 0x35b98` | `-` |
| `ldr` at 0x35be0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x35bf0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x35bf8 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x35bfc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x35c00 | load | 4 | `abs 0x35b9c` | `-` |
| `ldr` at 0x35c08 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x35c18 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x35c20 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `str` at 0x35c28 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x35c2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x35c38 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x35c50 | load | 1 | `sp -0x40` | `-` |
| `ldr` at 0x35c5c | load | 4 | `abs 0x35ba4` | `-` |
| `str` at 0x35c60 | store | 4 | `(unknown)` | `0xf537c` |
| `ldr` at 0x35c64 | load | 4 | `abs 0x35ba4` | `-` |
| `str` at 0x35c68 | store | 4 | `(unknown)` | `0xf537c` |
| `ldr` at 0x35c6c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x35c70 | load | 4 | `abs 0x35ba0` | `-` |
| `ldr` at 0x35c74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x35c78 | load | 4 | `abs 0x35ba4` | `-` |
| `ldr` at 0x35c84 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x35c88 | store | 4 | `(unknown)` | `[sp-52]` |
| `ldr` at 0x35c8c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x35c90 | store | 4 | `(unknown)` | `[sp-60]` |
| `ldm` at 0x35c98 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x35c98 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x35c9c | store | 4 | `(unknown)` | `[sp-48]` |
| `str` at 0x35ca0 | store | 4 | `(unknown)` | `[sp-44]` |
| `ldr` at 0x35ca4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x35ca8 | load | 4 | `abs 0x35ba8` | `-` |
| `ldr` at 0x35cac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x35cb0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x35cb8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x35cc8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x35cd0 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x35cd4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x35cdc | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldm` at 0x35ce8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x35ce8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x35ce8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x35ce8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x356b4 | `(unknown callee) 0x1943c` | `0x5dc` | `r1` | `r2` | `r3` |
| `bl` at 0x356dc | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `[[sp-24]]` | `0x0` |
| `bl` at 0x356f8 | `(unknown callee) 0x1d444` | `0x9b` | `?` | `ret(0x19538)` | `([sp-32] + ret(0x19538))` |
| `bl` at 0x35728 | `(unknown callee) 0x197a8` | `[sp-32]` | `0xf5148` | `[sp-72]` | `?` |
| `bl` at 0x35730 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x35778 | `(unknown callee) 0x197a8` | `([sp-32] + ret(0x19538))` | `0xf5174` | `[sp-76]` | `?` |
| `bl` at 0x35780 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x357c0 | `(unknown callee) 0x197a8` | `[sp-32]` | `0xf5194` | `[0x12d6f0]` | `?` |
| `bl` at 0x357c8 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x35814 | `(unknown callee) 0x197a8` | `[sp-32]` | `0xf51b8` | `?` | `?` |
| `bl` at 0x35850 | `(unknown callee) 0x197a8` | `[sp-32]` | `0xf51c8` | `?` | `?` |
| `bl` at 0x35870 | `(unknown callee) 0x197a8` | `[sp-32]` | `0xf51d8` | `?` | `?` |
| `bl` at 0x35878 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x358c4 | `(unknown callee) 0x197a8` | `[sp-32]` | `0xf51e4` | `?` | `?` |
| `bl` at 0x358cc | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x35918 | `(unknown callee) 0x197a8` | `[sp-32]` | `0xf51b8` | `?` | `?` |
| `bl` at 0x35954 | `(unknown callee) 0x197a8` | `[sp-32]` | `0xf51c8` | `?` | `?` |
| `bl` at 0x3598c | `(unknown callee) 0x197a8` | `[sp-32]` | `0xf51b8` | `?` | `?` |
| `bl` at 0x359ac | `(unknown callee) 0x197a8` | `[sp-32]` | `0xf51d8` | `?` | `?` |
| `bl` at 0x359b4 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x359d4 | `(unknown callee) 0x197a8` | `([sp-32] + ret(0x19538))` | `0xf5204` | `ret(0x19538)` | `([sp-32] + ret(0x19538))` |
| `bl` at 0x359dc | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x359fc | `(unknown callee) 0x197a8` | `([sp-32] + ret(0x19538))` | `0xf5220` | `ret(0x19538)` | `([sp-32] + ret(0x19538))` |
| `bl` at 0x35a04 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x35a24 | `(unknown callee) 0x197a8` | `([sp-32] + ret(0x19538))` | `0xf524c` | `ret(0x19538)` | `([sp-32] + ret(0x19538))` |
| `bl` at 0x35a2c | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x35a4c | `(unknown callee) 0x197a8` | `([sp-32] + ret(0x19538))` | `0xf5274` | `ret(0x19538)` | `([sp-32] + ret(0x19538))` |
| `bl` at 0x35a54 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x35a94 | `(unknown callee) 0x34c90` | `?` | `sp-68` | `sp-68` | `?` |
| `bl` at 0x35abc | `(unknown callee) 0x197a8` | `[sp-32]` | `0xf529c` | `[sp-28]` | `0xf537c` |
| `bl` at 0x35b00 | `(unknown callee) 0x197a8` | `[sp-32]` | `0xf52b4` | `[sp-28]` | `[sp-67]:1` |
| `bl` at 0x35b08 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x35bb4 | `(unknown callee) 0x197a8` | `([sp-32] + ret(0x19538))` | `0xf52d4` | `ret(0x19538)` | `?` |
| `bl` at 0x35bbc | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x35bdc | `(unknown callee) 0x197a8` | `([sp-32] + ret(0x19538))` | `0xf52f4` | `ret(0x19538)` | `([sp-32] + ret(0x19538))` |
| `bl` at 0x35be4 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x35c04 | `(unknown callee) 0x197a8` | `([sp-32] + ret(0x19538))` | `0xf532c` | `ret(0x19538)` | `([sp-32] + ret(0x19538))` |
| `bl` at 0x35c0c | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x35c4c | `(unknown callee) 0x34ebc` | `?` | `sp-64` | `sp-64` | `?` |
| `bl` at 0x35c7c | `(unknown callee) 0x197a8` | `[sp-32]` | `0xf5364` | `[sp-28]` | `0xf537c` |
| `bl` at 0x35cb4 | `(unknown callee) 0x197a8` | `[sp-32]` | `0xf5388` | `[sp-28]` | `[sp-56]` |
| `bl` at 0x35cbc | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x35704 | `ret(0x1d444) == 0x0` | `ret(0x1d444) != 0x0` |
| `beq` at 0x35754 | `[0x12d6ec] == 0x0` | `[0x12d6ec] != 0x0` |
| `bge` at 0x357ec | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bge` at 0x35828 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bge` at 0x358f0 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bge` at 0x3592c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x35964 | `Z == 1` | `not (Z == 1)` |
| `bgt` at 0x35a7c | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x35aa0 | `[sp-68]:1 == 0x0` | `[sp-68]:1 != 0x0` |
| `bgt` at 0x35c34 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x35c58 | `[sp-64]:1 == 0x0` | `[sp-64]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x35ce8 | `r0` | `ret(0x19538)` |

## `LinkErrReadBER`

- `LinkErrReadBER`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x34ebc to 0x35344.
  Established. The unnamed `0xde16c` call carries `[sp-36]`, `0x1000`, `sp-76`, `sp-80` (`bl` at 0x34f20).
  A zero result takes `0x34f48`, while nonzero falls through (`beq` at 0x34f34).
  Included word assignments write `[sp-104]` to `[sp-28] +0xc`, `[sp-100]` to `[sp-28] +0x8` and `[sp-60]` to `[sp-28] +0x4` (`str` at 0x35140, `str` at 0x3514c, `str` at 0x35158).
  Bit-error measurement units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x34ed0 | store | 4 | `sp -0x1c` | `r1` |
| `strb` at 0x34ed4 | store | 1 | `sp -0x15` | `(r0 & 0xff)` |
| `str` at 0x34edc | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x34ee0 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x34ee8 | store | 1 | `r1 +0x0` | `0x1` |
| `str` at 0x34ef0 | store | 4 | `sp -0x24` | `0x600` |
| `str` at 0x34ef8 | store | 4 | `sp -0x4c` | `?` |
| `str` at 0x34f00 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x34f10 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x34f28 | store | 4 | `sp -0x20` | `ret(0xde16c)` |
| `ldr` at 0x34f2c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x34f3c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x34f40 | store | 4 | `sp -0x80` | `[sp-32]` |
| `ldr` at 0x34f4c | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x34f5c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x34f64 | load | 4 | `abs 0x3532c` | `-` |
| `str` at 0x34f6c | store | 4 | `sp -0x28` | `?` |
| `ldrb` at 0x34f70 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x34f78 | load | 4 | `abs 0x35330` | `-` |
| `str` at 0x34f80 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0x34f84 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x34f8c | load | 4 | `abs 0x35334` | `-` |
| `str` at 0x34f94 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x34fa4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x34fa8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x34fbc | store | 4 | `sp -0x20` | `ret(0xde16c)` |
| `ldr` at 0x34fc0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x34fd0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x34fd4 | store | 4 | `sp -0x80` | `[sp-32]` |
| `ldr` at 0x34fe4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x34fe8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x34ffc | store | 4 | `sp -0x20` | `ret(0xde16c)` |
| `ldr` at 0x35000 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x35010 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x35014 | store | 4 | `sp -0x80` | `[sp-32]` |
| `ldr` at 0x35024 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x35028 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x3503c | store | 4 | `sp -0x20` | `ret(0xde16c)` |
| `ldr` at 0x35040 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x35050 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x35054 | store | 4 | `sp -0x80` | `[sp-32]` |
| `ldr` at 0x3505c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x35060 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x35074 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x35088 | store | 4 | `sp -0x80` | `0x0` |
| `ldr` at 0x35090 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x35098 | store | 1 | `[sp-28] +0x0` | `0x0` |
| `ldr` at 0x3509c | load | 4 | `sp -0x58` | `-` |
| `str` at 0x350b0 | store | 4 | `sp -0x84` | `0x1` |
| `str` at 0x350bc | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x350c0 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x350c4 | store | 4 | `sp -0x38` | `[sp-132]` |
| `ldr` at 0x350c8 | load | 4 | `abs 0x35338` | `-` |
| `ldr` at 0x350cc | load | 4 | `sp -0x60` | `-` |
| `str` at 0x350d4 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x350d8 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x350e4 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x350e8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x350f0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x350fc | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x35100 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3510c | load | 4 | `sp -0x68` | `-` |
| `str` at 0x35114 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x35118 | load | 4 | `sp -0x68` | `-` |
| `str` at 0x35120 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0x35124 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x3512c | load | 4 | `sp -0x48` | `-` |
| `str` at 0x35134 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0x35138 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3513c | load | 4 | `sp -0x68` | `-` |
| `str` at 0x35140 | store | 4 | `[sp-28] +0xc` | `[sp-104]` |
| `ldr` at 0x35144 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x35148 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x3514c | store | 4 | `[sp-28] +0x8` | `[sp-100]` |
| `ldr` at 0x35150 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x35154 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x35158 | store | 4 | `[sp-28] +0x4` | `[sp-60]` |
| `ldr` at 0x3515c | load | 4 | `sp -0x68` | `-` |
| `str` at 0x35160 | store | 4 | `sp -0x6c` | `[sp-104]` |
| `ldr` at 0x35164 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x35168 | store | 4 | `sp -0x70` | `[sp-100]` |
| `ldr` at 0x3516c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x35170 | store | 4 | `sp -0x74` | `[sp-60]` |
| `ldr` at 0x35174 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x35180 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x3518c | load | 4 | `sp -0x70` | `-` |
| `str` at 0x35194 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x35198 | load | 4 | `sp -0x70` | `-` |
| `str` at 0x351a0 | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0x351a4 | load | 4 | `sp -0x6c` | `-` |
| `str` at 0x351ac | store | 4 | `sp -0x6c` | `?` |
| `ldr` at 0x351b0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x351bc | load | 4 | `sp -0x6c` | `-` |
| `str` at 0x351c4 | store | 4 | `sp -0x6c` | `?` |
| `ldr` at 0x351c8 | load | 4 | `sp -0x74` | `-` |
| `str` at 0x351d0 | store | 4 | `sp -0x74` | `?` |
| `ldr` at 0x351d8 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x351e4 | load | 4 | `sp -0x70` | `-` |
| `ldm` at 0x351f4 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x351f4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x351f8 | store | 4 | `sp -0x7c` | `?` |
| `str` at 0x351fc | store | 4 | `sp -0x78` | `?` |
| `ldr` at 0x35200 | load | 4 | `abs 0x3533c` | `-` |
| `ldr` at 0x3520c | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x35218 | load | 4 | `sp -0x74` | `-` |
| `stc` at 0x35220 | store | unknown | `sp -0x8c` | `?` |
| `ldr` at 0x35224 | load | 4 | `sp -0x74` | `-` |
| `ldc` at 0x35230 | load | unknown | `abs 0x3530c` | `-` |
| `ldc` at 0x35234 | load | unknown | `sp -0x8c` | `-` |
| `stc` at 0x3523c | store | unknown | `sp -0x8c` | `?` |
| `ldr` at 0x35240 | load | 4 | `sp -0x6c` | `-` |
| `stc` at 0x35248 | store | unknown | `sp -0x94` | `?` |
| `ldr` at 0x3524c | load | 4 | `sp -0x6c` | `-` |
| `ldc` at 0x35258 | load | unknown | `abs 0x3530c` | `-` |
| `ldc` at 0x3525c | load | unknown | `sp -0x94` | `-` |
| `stc` at 0x35264 | store | unknown | `sp -0x94` | `?` |
| `ldc` at 0x35268 | load | unknown | `sp -0x8c` | `-` |
| `ldc` at 0x3526c | load | unknown | `sp -0x94` | `-` |
| `ldc` at 0x35274 | load | unknown | `abs 0x35314` | `-` |
| `stc` at 0x3527c | store | unknown | `sp -0x7c` | `?` |
| `ldm` at 0x35288 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x35288 | load | 4 | `(unknown)` | `-` |
| `str` at 0x3528c | store | 4 | `sp -0x7c` | `?` |
| `str` at 0x35290 | store | 4 | `sp -0x78` | `?` |
| `ldr` at 0x35294 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x3529c | load | 4 | `sp -0x7c` | `-` |
| `ldm` at 0x3529c | load | 4 | `sp -0x78` | `-` |
| `str` at 0x352a0 | store | 4 | `[sp-28] +0x10` | `[sp-124]` |
| `str` at 0x352a4 | store | 4 | `[sp-28] +0x14` | `[sp-120]` |
| `ldr` at 0x352a8 | load | 4 | `sp -0x3c` | `-` |
| `ldc` at 0x352b4 | load | unknown | `sp -0x7c` | `-` |
| `ldc` at 0x352b8 | load | unknown | `abs 0x35324` | `-` |
| `ldr` at 0x352c8 | load | 4 | `abs 0x35340` | `-` |
| `ldm` at 0x352d0 | load | 4 | `sp -0x7c` | `-` |
| `ldm` at 0x352d0 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x352dc | store | 4 | `sp -0x80` | `0x1` |
| `ldr` at 0x352e4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x352e8 | store | 4 | `sp -0x80` | `[sp-52]` |
| `str` at 0x352f4 | store | 4 | `sp -0x80` | `0x0` |
| `ldr` at 0x352f8 | load | 4 | `sp -0x80` | `-` |
| `ldm` at 0x35300 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x35300 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x35300 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x35300 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x34f04 | `(unknown callee) 0xd708c` | `r0` | `r1` | `r1` | `?` |
| `bl` at 0x34f20 | `(unknown callee) 0xde16c` | `[sp-36]` | `0x1000` | `sp-76` | `sp-80` |
| `bl` at 0x34f38 | `(unknown callee) 0xd70fc` | `ret(0xde16c)` | `?` | `?` | `ret(0xde16c)` |
| `bl` at 0x34f48 | `(unknown callee) 0xd70fc` | `ret(0xde16c)` | `?` | `?` | `ret(0xde16c)` |
| `bl` at 0x34f98 | `(unknown callee) 0xd708c` | `ret(0xd70fc)` | `?` | `?` | `?` |
| `bl` at 0x34fb4 | `(unknown callee) 0xde16c` | `[sp-36]` | `[sp-40]` | `sp-84` | `sp-88` |
| `bl` at 0x34fcc | `(unknown callee) 0xd70fc` | `ret(0xde16c)` | `?` | `?` | `ret(0xde16c)` |
| `bl` at 0x34ff4 | `(unknown callee) 0xde16c` | `[sp-36]` | `[sp-44]` | `sp-92` | `sp-96` |
| `bl` at 0x3500c | `(unknown callee) 0xd70fc` | `ret(0xde16c)` | `?` | `?` | `ret(0xde16c)` |
| `bl` at 0x35034 | `(unknown callee) 0xde16c` | `[sp-36]` | `[sp-48]` | `sp-100` | `sp-104` |
| `bl` at 0x3504c | `(unknown callee) 0xd70fc` | `ret(0xde16c)` | `?` | `?` | `ret(0xde16c)` |
| `bl` at 0x3506c | `(unknown callee) 0xde274` | `[sp-36]` | `[sp-48]` | `0x0` | `0x0` |
| `bl` at 0x35070 | `(unknown callee) 0xd70fc` | `ret(0xde274)` | `?` | `?` | `?` |
| `bl` at 0x35204 | `(unknown callee) 0x1e958` | `0xf507c` | `?` | `[sp-28]` | `?` |
| `bl` at 0x352d4 | `(unknown callee) 0x1e958` | `0xf5094` | `[sp-124]` | `[sp-120]` | `[sp-60]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x34f34 | `ret(0xde16c) == 0x0` | `ret(0xde16c) != 0x0` |
| `beq` at 0x34f58 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x34fc8 | `ret(0xde16c) == 0x0` | `ret(0xde16c) != 0x0` |
| `beq` at 0x35008 | `ret(0xde16c) == 0x0` | `ret(0xde16c) != 0x0` |
| `beq` at 0x35048 | `ret(0xde16c) == 0x0` | `ret(0xde16c) != 0x0` |
| `bne` at 0x35080 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x350a8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x35108 | `[sp-132] == 0x0` | `[sp-132] != 0x0` |
| `bls` at 0x3517c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x35188 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x351b8 | `[sp-72] == 0x0` | `[sp-72] != 0x0` |
| `bne` at 0x351e0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x351ec | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x35214 | `Z == 1` | `not (Z == 1)` |
| `bge` at 0x3522c | `[sp-116] >= 0x0 (signed, 32-bit)` | `[sp-116] < 0x0 (signed, 32-bit)` |
| `bge` at 0x35254 | `[sp-108] >= 0x0 (signed, 32-bit)` | `[sp-108] < 0x0 (signed, 32-bit)` |
| `beq` at 0x352b0 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `bgt` at 0x352c0 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x35300 | `r0` | `?` |

## `IP67_Cfg_Init`

- `IP67_Cfg_Init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x55bdc to 0x55d94.
  Established. Included assignments write byte `0x1` to `0x132558`, word `0x1` to `0x13255c` and word `0x8` to `0x132564` (`strb` at 0x55c30, `str` at 0x55c3c, `str` at 0x55c54).
  The unnamed `0x412f4` call carries `[0x29984c]`, `[0x13255c]`, `[0x132564]`, `0x0` (`bl` at 0x55d54).
  IP67 configuration ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x55bec | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x55bf0 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x55bf4 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x55bf8 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x55bfc | load | 4 | `abs 0x55d6c` | `-` |
| `ldrb` at 0x55c00 | load | 1 | `abs 0x132558` | `-` |
| `ldr` at 0x55c0c | load | 4 | `abs 0x55d70` | `-` |
| `str` at 0x55c10 | store | 4 | `sp -0x38` | `0xf9290` |
| `ldr` at 0x55c18 | load | 4 | `abs 0x55d74` | `-` |
| `ldr` at 0x55c28 | load | 4 | `abs 0x55d6c` | `-` |
| `strb` at 0x55c30 | store | 1 | `abs 0x132558` | `0x1` |
| `ldr` at 0x55c34 | load | 4 | `abs 0x55d78` | `-` |
| `str` at 0x55c3c | store | 4 | `abs 0x13255c` | `0x1` |
| `ldr` at 0x55c40 | load | 4 | `abs 0x55d7c` | `-` |
| `ldr` at 0x55c44 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x55c48 | store | 4 | `abs 0x132560` | `[sp-24]` |
| `ldr` at 0x55c4c | load | 4 | `abs 0x55d80` | `-` |
| `str` at 0x55c54 | store | 4 | `abs 0x132564` | `0x8` |
| `ldr` at 0x55c58 | load | 4 | `abs 0x55d84` | `-` |
| `str` at 0x55c68 | store | 4 | `abs 0x29984c` | `ret(0x1943c)` |
| `ldr` at 0x55c6c | load | 4 | `abs 0x29984c` | `-` |
| `ldr` at 0x55c78 | load | 4 | `abs 0x55d88` | `-` |
| `str` at 0x55c7c | store | 4 | `sp -0x38` | `0xf9218` |
| `ldr` at 0x55c84 | load | 4 | `abs 0x55d74` | `-` |
| `ldr` at 0x55c94 | load | 4 | `abs 0x55d84` | `-` |
| `ldr` at 0x55c98 | load | 4 | `abs 0x29984c` | `-` |
| `ldr` at 0x55c9c | load | 4 | `abs 0x55d7c` | `-` |
| `ldr` at 0x55ca0 | load | 4 | `abs 0x132560` | `-` |
| `ldrb` at 0x55ca4 | load | 1 | `[0x132560] +0x3` | `-` |
| `strb` at 0x55cac | store | 1 | `[0x29984c] +0x0` | `([[0x132560]+3]:1+1 & 0xff)` |
| `ldr` at 0x55cb0 | load | 4 | `abs 0x55d84` | `-` |
| `ldr` at 0x55cb4 | load | 4 | `abs 0x29984c` | `-` |
| `ldr` at 0x55cb8 | load | 4 | `abs 0x55d7c` | `-` |
| `ldr` at 0x55cbc | load | 4 | `abs 0x132560` | `-` |
| `ldrb` at 0x55cc0 | load | 1 | `[0x132560] +0x3` | `-` |
| `strb` at 0x55cc8 | store | 1 | `[0x29984c] +0x1` | `([[0x132560]+3]:1+5 & 0xff)` |
| `ldr` at 0x55ccc | load | 4 | `abs 0x55d84` | `-` |
| `ldr` at 0x55cd0 | load | 4 | `abs 0x29984c` | `-` |
| `ldr` at 0x55cd4 | load | 4 | `abs 0x55d7c` | `-` |
| `ldr` at 0x55cd8 | load | 4 | `abs 0x132560` | `-` |
| `ldr` at 0x55cdc | load | 4 | `[0x132560] +0xc` | `-` |
| `str` at 0x55ce0 | store | 4 | `[0x29984c] +0x4` | `[[0x132560]+12]` |
| `ldr` at 0x55ce4 | load | 4 | `abs 0x55d84` | `-` |
| `ldr` at 0x55ce8 | load | 4 | `abs 0x29984c` | `-` |
| `str` at 0x55cf0 | store | 4 | `[0x29984c] +0x8` | `0xff000000` |
| `ldr` at 0x55cf4 | load | 4 | `abs 0x55d84` | `-` |
| `ldr` at 0x55cf8 | load | 4 | `abs 0x29984c` | `-` |
| `ldr` at 0x55cfc | load | 4 | `abs 0x55d8c` | `-` |
| `ldr` at 0x55d00 | load | 4 | `abs 0xf91d8` | `-` |
| `str` at 0x55d04 | store | 4 | `[0x29984c] +0xc` | `[0xf91d8]` |
| `ldr` at 0x55d08 | load | 4 | `abs 0x55d90` | `-` |
| `ldr` at 0x55d10 | load | 4 | `abs 0x55d84` | `-` |
| `ldr` at 0x55d14 | load | 4 | `abs 0x55d78` | `-` |
| `ldr` at 0x55d18 | load | 4 | `abs 0x55d80` | `-` |
| `str` at 0x55d20 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x55d24 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x55d28 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0x55d2c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55d30 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0x55d34 | load | 4 | `abs 0x55d90` | `-` |
| `str` at 0x55d38 | store | 4 | `sp -0x2c` | `0x132570` |
| `str` at 0x55d40 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x55d44 | load | 4 | `abs 0x29984c` | `-` |
| `ldr` at 0x55d48 | load | 4 | `abs 0x13255c` | `-` |
| `ldr` at 0x55d4c | load | 4 | `abs 0x132564` | `-` |
| `ldr` at 0x55d58 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x55d5c | load | 4 | `sp +0x0` | `-` |
| `ldm` at 0x55d68 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x55d68 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x55d68 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x55d68 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x55c24 | `(unknown callee) 0x200a0` | `0x9` | `0xf9204` | `0xf1` | `0x0` |
| `bl` at 0x55c60 | `(unknown callee) 0x1943c` | `0x10` | `?` | `0x132564` | `0x8` |
| `bl` at 0x55c90 | `(unknown callee) 0x200a0` | `0x9` | `0xf9204` | `0xfc` | `0x0` |
| `bl` at 0x55d0c | `(unknown callee) 0x55d94` | `0x132570` | `?` | `[0x29984c]` | `[0xf91d8]` |
| `bl` at 0x55d54 | `(unknown callee) 0x412f4` | `[0x29984c]` | `[0x13255c]` | `[0x132564]` | `0x0` |
| `bl` at 0x55d60 | `(unknown callee) 0x417a4` | `[sp-36]` | `[sp]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x55c08 | `[0x132558]:1 == 0x0` | `[0x132558]:1 != 0x0` |
| `bne` at 0x55c74 | `[0x29984c] != 0x0` | `[0x29984c] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x55d68 | `r0` | `ret(0x417a4)` |

## `APwrInit`

- `APwrInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa55ac to 0xa57c8.
  Established. A nonzero `ret(0x1d444)` takes `0xa56c0`, while zero falls through (`bne` at 0xa56b8).
  The unnamed `0xa76a4` call carries `0x29a9e0`, `0xffff014`, `0x2`, `0x29a9e8` (`bl` at 0xa5738).
  Included word assignments write `0x4b0` to `sp -0x2c` and `0x19` to `sp -0x28` (`str` at 0xa5768, `str` at 0xa5770).
  Power setup parameter units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa55c8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xa55cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa55d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa55e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa55f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa55fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa5608 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa5614 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa5620 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa562c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa5638 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa5644 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa5650 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa565c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa5668 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa5674 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa5680 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa568c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa5698 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa56c0 | load | 4 | `abs 0xa57a0` | `-` |
| `ldr` at 0xa56e8 | load | 4 | `abs 0xa57a4` | `-` |
| `ldr` at 0xa56f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa56f8 | load | 4 | `abs 0xa57a8` | `-` |
| `ldr` at 0xa56fc | load | 4 | `abs 0xa57ac` | `-` |
| `str` at 0xa570c | store | 4 | `sp -0x18` | `ret(0xa8af0)` |
| `ldr` at 0xa5710 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa571c | load | 4 | `abs 0xa57b0` | `-` |
| `ldr` at 0xa5728 | load | 4 | `abs 0xa57a0` | `-` |
| `ldr` at 0xa572c | load | 4 | `abs 0xa57b4` | `-` |
| `ldr` at 0xa5734 | load | 4 | `abs 0xa57a8` | `-` |
| `str` at 0xa5740 | store | 4 | `sp -0x18` | `ret(0xa76a4)` |
| `ldr` at 0xa5744 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa5750 | load | 4 | `abs 0xa57b8` | `-` |
| `ldr` at 0xa575c | load | 4 | `abs 0xa57a0` | `-` |
| `str` at 0xa5760 | store | 4 | `sp -0x30` | `0x29a9e0` |
| `str` at 0xa5768 | store | 4 | `sp -0x2c` | `0x4b0` |
| `str` at 0xa5770 | store | 4 | `sp -0x28` | `0x19` |
| `str` at 0xa5778 | store | 4 | `sp -0x24` | `0xa` |
| `str` at 0xa5780 | store | 4 | `sp -0x20` | `0x9` |
| `ldr` at 0xa5784 | load | 4 | `abs 0xa57bc` | `-` |
| `ldr` at 0xa5788 | load | 4 | `abs 0xa57c0` | `-` |
| `ldr` at 0xa578c | load | 4 | `abs 0xa57c4` | `-` |
| `ldm` at 0xa579c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xa579c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa579c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa579c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa55bc | `(unknown callee) 0x488cc` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xa56ac | `(unknown callee) 0x1d444` | `0x32` | `?` | `?` | `[sp-28]` |
| `bl` at 0xa56cc | `(unknown callee) 0xa89d0` | `0x29a9e0` | `0x14` | `0x0` | `ret(0x1d444)` |
| `bl` at 0xa56d4 | `(unknown callee) 0x1d444` | `0x1` | `?` | `?` | `?` |
| `bl` at 0xa56e0 | `(unknown callee) 0x1d444` | `0x2` | `?` | `?` | `?` |
| `bl` at 0xa56f4 | `(unknown callee) 0x5f954` | `0x133bdc` | `ret(0x1d444)` | `[sp-28]` | `ret(0x1d444)` |
| `bl` at 0xa5704 | `(unknown callee) 0xa8af0` | `0x29a9e8` | `0x1137c8` | `0x1` | `?` |
| `bl` at 0xa5720 | `(unknown callee) 0x1e958` | `0x1137d4` | `?` | `?` | `ret(0xa8af0)` |
| `bl` at 0xa5738 | `(unknown callee) 0xa76a4` | `0x29a9e0` | `0xffff014` | `0x2` | `0x29a9e8` |
| `bl` at 0xa5754 | `(unknown callee) 0x1e958` | `0x113800` | `?` | `?` | `ret(0xa76a4)` |
| `bl` at 0xa5794 | `(unknown callee) 0x2066c` | `0x29aa80` | `0x11382c` | `0xa57c8` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa55d4 | `[sp-28] == 0x43` | `[sp-28] != 0x43` |
| `beq` at 0xa55e0 | `[sp-28] == 0x33` | `[sp-28] != 0x33` |
| `beq` at 0xa55ec | `[sp-28] == 0x35` | `[sp-28] != 0x35` |
| `beq` at 0xa55f8 | `[sp-28] == 0x34` | `[sp-28] != 0x34` |
| `beq` at 0xa5604 | `[sp-28] == 0x41` | `[sp-28] != 0x41` |
| `beq` at 0xa5610 | `[sp-28] == 0x36` | `[sp-28] != 0x36` |
| `beq` at 0xa561c | `[sp-28] == 0x39` | `[sp-28] != 0x39` |
| `beq` at 0xa5628 | `[sp-28] == 0x37` | `[sp-28] != 0x37` |
| `beq` at 0xa5634 | `[sp-28] == 0x38` | `[sp-28] != 0x38` |
| `beq` at 0xa5640 | `[sp-28] == 0x30` | `[sp-28] != 0x30` |
| `beq` at 0xa564c | `[sp-28] == 0x51` | `[sp-28] != 0x51` |
| `beq` at 0xa5658 | `[sp-28] == 0x40` | `[sp-28] != 0x40` |
| `beq` at 0xa5664 | `[sp-28] == 0x4f` | `[sp-28] != 0x4f` |
| `beq` at 0xa5670 | `[sp-28] == 0x42` | `[sp-28] != 0x42` |
| `beq` at 0xa567c | `[sp-28] == 0x46` | `[sp-28] != 0x46` |
| `beq` at 0xa5688 | `[sp-28] == 0x4c` | `[sp-28] != 0x4c` |
| `beq` at 0xa5694 | `[sp-28] == 0x4d` | `[sp-28] != 0x4d` |
| `beq` at 0xa56a0 | `[sp-28] == 0x7c` | `[sp-28] != 0x7c` |
| `bne` at 0xa56b8 | `ret(0x1d444) != 0x0` | `ret(0x1d444) == 0x0` |
| `beq` at 0xa5718 | `ret(0xa8af0) == 0x0` | `ret(0xa8af0) != 0x0` |
| `beq` at 0xa574c | `ret(0xa76a4) == 0x0` | `ret(0xa76a4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa579c | `r0` | `?` |

## `athenaComm`

- `athenaComm`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2e984 to 0x2ea38.
  Established. A zero `ret(0x4ba54)` takes `0x2e99c`, while nonzero falls through (`beq` at 0x2e9ac).
  The unnamed `0xca70c` call carries `0x0`, `0x9`, `sp-25`, `sp-25` (`bl` at 0x2e9c0).
  Another unnamed `0x4bbf0` call carries `0x0`, `0x9`, `0x2`, `[sp-25]:1` (`bl` at 0x2ea18).
  Polling peripheral semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2e994 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x2e998 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x2e9c8 | store | 1 | `sp -0x1a` | `(ret(0xca70c) & 0xff)` |
| `ldrb` at 0x2e9cc | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0x2e9dc | load | 4 | `abs 0x2ea2c` | `-` |
| `str` at 0x2e9e0 | store | 4 | `sp -0x20` | `0xf37d0` |
| `ldr` at 0x2e9e8 | load | 4 | `abs 0x2ea30` | `-` |
| `ldr` at 0x2e9ec | load | 4 | `abs 0x2ea34` | `-` |
| `ldrb` at 0x2e9f8 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0x2ea08 | load | 1 | `sp -0x19` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2e9a0 | `(unknown callee) 0x4ba54` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x2e9c0 | `(unknown callee) 0xca70c` | `0x0` | `0x9` | `sp-25` | `sp-25` |
| `bl` at 0x2e9f4 | `(unknown callee) 0x200a0` | `0x1` | `0xf366c` | `0x8c5` | `0x0` |
| `bl` at 0x2ea18 | `(unknown callee) 0x4bbf0` | `0x0` | `0x9` | `0x2` | `[sp-25]:1` |
| `bl` at 0x2ea24 | `(unknown callee) 0x2ea38` | `0xfa` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x2e9ac | `ret(0x4ba54) == 0x0` | `ret(0x4ba54) != 0x0` |
| `beq` at 0x2e9d8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x2ea04 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|

## `AthenaCommPollInit`

- `AthenaCommPollInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2e924 to 0x2e984.
  Established. The unnamed `0x2066c` call carries `0x1320a4`, `0xf37c4`, `0x2e984`, `0x1` (`bl` at 0x2e96c).
  Included word assignments write `0x800` to `sp -0x24` and `0x9` to `sp -0x18` (`str` at 0x2e940, `str` at 0x2e958).
  Communication task scheduling parameters remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2e938 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x2e940 | store | 4 | `sp -0x24` | `0x800` |
| `str` at 0x2e948 | store | 4 | `sp -0x20` | `0xa` |
| `str` at 0x2e950 | store | 4 | `sp -0x1c` | `0xa` |
| `str` at 0x2e958 | store | 4 | `sp -0x18` | `0x9` |
| `ldr` at 0x2e95c | load | 4 | `abs 0x2e978` | `-` |
| `ldr` at 0x2e960 | load | 4 | `abs 0x2e97c` | `-` |
| `ldr` at 0x2e964 | load | 4 | `abs 0x2e980` | `-` |
| `ldm` at 0x2e974 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x2e974 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x2e974 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2e96c | `(unknown callee) 0x2066c` | `0x1320a4` | `0xf37c4` | `0x2e984` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x2e974 | `r0` | `ret(0x2066c)` |

## `t1_displayInit`

- `t1_displayInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcee80 to 0xcef24.
  Established. Included byte stores write `0xc` to `r0 +0x2` and `0x20` to `[sp-20] +0x5` (`strb` at 0xcee9c, `strb` at 0xceec0).
  A word assignment writes `[sp-20]` to `0x297358` (`str` at 0xcef08).
  Display initialization record fields remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcee90 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xcee94 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xcee9c | store | 1 | `r0 +0x2` | `0xc` |
| `ldr` at 0xceea0 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xceea8 | store | 1 | `[sp-20] +0x3` | `0x0` |
| `ldr` at 0xceeac | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xceeb4 | store | 1 | `[sp-20] +0x4` | `0x0` |
| `ldr` at 0xceeb8 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xceec0 | store | 1 | `[sp-20] +0x5` | `0x20` |
| `ldr` at 0xceec4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xceec8 | load | 4 | `abs 0xcef10` | `-` |
| `str` at 0xceecc | store | 4 | `[sp-20] +0x8` | `0x29c419` |
| `ldr` at 0xceed0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xceed4 | load | 4 | `abs 0xcef14` | `-` |
| `str` at 0xceed8 | store | 4 | `[sp-20] +0xc` | `0xcef24` |
| `ldr` at 0xceedc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xceee0 | load | 4 | `abs 0xcef18` | `-` |
| `str` at 0xceee4 | store | 4 | `[sp-20] +0x10` | `0xcf0cc` |
| `ldr` at 0xceee8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xceef0 | store | 4 | `[sp-20] +0x14` | `0x0` |
| `ldr` at 0xceef4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xceef8 | load | 4 | `abs 0xcef1c` | `-` |
| `str` at 0xceefc | store | 4 | `[sp-20] +0x18` | `0xcf29c` |
| `ldr` at 0xcef00 | load | 4 | `abs 0xcef20` | `-` |
| `ldr` at 0xcef04 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcef08 | store | 4 | `abs 0x297358` | `[sp-20]` |
| `ldm` at 0xcef0c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xcef0c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcef0c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcef0c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcef0c | `r0` | `r0` |

## `pcf2119_Init_f`

- `pcf2119_Init_f`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xce260 to 0xce308.
  Established. Included byte assignments write `0x10` to `r0 +0x2`, `0x30` to `[sp-20] +0x3` and `0x1` to `[sp-20] +0x4` (`strb` at 0xce27c, `strb` at 0xce288, `strb` at 0xce294).
  Another byte assignment has unknown value at `[sp-20] +0x5` (`strb` at 0xce2a0).
  A word assignment writes `[sp-20]` to `0x29c3c4` (`str` at 0xce2e8).
  Peripheral configuration field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xce270 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xce274 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xce27c | store | 1 | `r0 +0x2` | `0x10` |
| `ldr` at 0xce280 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xce288 | store | 1 | `[sp-20] +0x3` | `0x30` |
| `ldr` at 0xce28c | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xce294 | store | 1 | `[sp-20] +0x4` | `0x1` |
| `ldr` at 0xce298 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xce2a0 | store | 1 | `[sp-20] +0x5` | `?` |
| `ldr` at 0xce2a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce2a8 | load | 4 | `abs 0xce2f0` | `-` |
| `str` at 0xce2ac | store | 4 | `[sp-20] +0x8` | `0x29c3cb` |
| `ldr` at 0xce2b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce2b4 | load | 4 | `abs 0xce2f4` | `-` |
| `str` at 0xce2b8 | store | 4 | `[sp-20] +0xc` | `0xce32c` |
| `ldr` at 0xce2bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce2c0 | load | 4 | `abs 0xce2f8` | `-` |
| `str` at 0xce2c4 | store | 4 | `[sp-20] +0x10` | `0xce558` |
| `ldr` at 0xce2c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce2cc | load | 4 | `abs 0xce2fc` | `-` |
| `str` at 0xce2d0 | store | 4 | `[sp-20] +0x14` | `0xce5dc` |
| `ldr` at 0xce2d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce2d8 | load | 4 | `abs 0xce300` | `-` |
| `str` at 0xce2dc | store | 4 | `[sp-20] +0x18` | `0xce67c` |
| `ldr` at 0xce2e0 | load | 4 | `abs 0xce304` | `-` |
| `ldr` at 0xce2e4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xce2e8 | store | 4 | `abs 0x29c3c4` | `[sp-20]` |
| `ldm` at 0xce2ec | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xce2ec | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xce2ec | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xce2ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xce2ec | `r0` | `r0` |

## `pcf2119_Init_stub`

- `pcf2119_Init_stub`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbba0c to 0xbba24.
  Established. A word assignment saves entry `r0` at `sp -0x14` (`str` at 0xbba1c).
  The integer return record preserves entry `r0` (`ldm` at 0xbba20).
  This stub has Unknown device responsibility.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbba1c | store | 4 | `sp -0x14` | `r0` |
| `ldm` at 0xbba20 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xbba20 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbba20 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbba20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbba20 | `r0` | `r0` |
