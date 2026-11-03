# ARM packet capture and dispatch paths in 1.44.0

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
The computed transfer in B2BReqRsp remains unresolved; its tables cover known edges only.
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/packet-capture-dispatch-facts.txt) retain all tool sections.

## `SysComCaptureIn`

- `SysComCaptureIn`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa2e34 to 0xa34a4.
  Established. The signed condition `? > [sp-24]` takes `0xa2f74`, while less or equal values fall through (`bgt` at 0xa2f6c).
  The unnamed `0x19610` call carries unknown, `[sp-28]`, `[sp-32]`, unknown (`bl` at 0xa31cc).
  An included word store has both an unknown effective target and an unknown value (`str` at 0xa3484).
  Inbound capture record semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa2e44 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa2e48 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa2e4c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xa2e50 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xa2e54 | load | 4 | `abs 0xa3490` | `-` |
| `ldr` at 0xa2e58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa2e7c | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa2e84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa2eb0 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xa2eb8 | load | 1 | `(unknown)` | `-` |
| `str` at 0xa2ebc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xa2ec0 | load | 4 | `abs 0xa3490` | `-` |
| `ldr` at 0xa2ec4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa2ee8 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa2ef0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa2f1c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa2f24 | load | 4 | `abs 0xa3494` | `-` |
| `ldrh` at 0xa2f28 | load | 2 | `(unknown)` | `-` |
| `str` at 0xa2f2c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xa2f30 | load | 4 | `abs 0xa3490` | `-` |
| `ldr` at 0xa2f34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa2f58 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa2f60 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa2f64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa2f74 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa2f80 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xa2f84 | load | 4 | `abs 0xa3490` | `-` |
| `ldr` at 0xa2f88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa2fac | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa2fb4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa2fe0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa2fe8 | load | 4 | `abs 0xa3490` | `-` |
| `ldr` at 0xa2fec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3010 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa3018 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa3044 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xa304c | load | 1 | `(unknown)` | `-` |
| `strb` at 0xa306c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xa3070 | load | 4 | `abs 0xa3490` | `-` |
| `ldr` at 0xa3074 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3098 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa30a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa30cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa30d4 | load | 4 | `abs 0xa3494` | `-` |
| `ldr` at 0xa30d8 | load | 4 | `abs 0xa3490` | `-` |
| `ldr` at 0xa30dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3100 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa3108 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa3134 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa313c | load | 4 | `abs 0xa3494` | `-` |
| `ldrh` at 0xa3140 | load | 2 | `(unknown)` | `-` |
| `strh` at 0xa3148 | store | 2 | `(unknown)` | `?` |
| `ldr` at 0xa314c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa3178 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa3184 | load | 4 | `abs 0xa3490` | `-` |
| `ldr` at 0xa3188 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa31ac | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa31b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa31c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa31c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa31d0 | load | 4 | `abs 0xa3490` | `-` |
| `ldr` at 0xa31d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa31f8 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa3200 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa322c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa3238 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa3244 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xa3248 | store | 1 | `(unknown)` | `([sp-32] & 0xff)` |
| `ldr` at 0xa324c | load | 4 | `abs 0xa3490` | `-` |
| `ldr` at 0xa3250 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3274 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa327c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa32a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa32b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa32b4 | load | 4 | `abs 0xa3498` | `-` |
| `ldr` at 0xa32c4 | load | 4 | `sp -0x28` | `-` |
| `strh` at 0xa32c8 | store | 2 | `(unknown)` | `([sp-40] & 0xffff)` |
| `ldr` at 0xa32cc | load | 4 | `abs 0xa3490` | `-` |
| `ldr` at 0xa32d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa32f4 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa32fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa3328 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa3330 | load | 4 | `abs 0xa349c` | `-` |
| `ldr` at 0xa3334 | load | 4 | `abs 0xa3490` | `-` |
| `ldr` at 0xa3338 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa335c | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa3364 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa3390 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa3398 | load | 4 | `abs 0xa349c` | `-` |
| `ldr` at 0xa339c | load | 4 | `(unknown)` | `-` |
| `str` at 0xa33a4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xa33a8 | load | 4 | `abs 0xa3490` | `-` |
| `ldr` at 0xa33ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa33d0 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa33d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa3404 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa340c | load | 4 | `abs 0xa34a0` | `-` |
| `ldr` at 0xa3410 | load | 4 | `abs 0xa3490` | `-` |
| `ldr` at 0xa3414 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3438 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa3440 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa346c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa3474 | load | 4 | `abs 0xa34a0` | `-` |
| `ldr` at 0xa3478 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa347c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa3484 | store | 4 | `(unknown)` | `?` |
| `ldm` at 0xa348c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa348c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa348c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa31cc | `(unknown callee) 0x19610` | `?` | `[sp-28]` | `[sp-32]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0xa2f6c | `? > [sp-24] (signed, 32-bit)` | `? <= [sp-24] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa348c | `r0` | `0x29a5d4` |

## `SysComCaptureOut`

- `SysComCaptureOut`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa34a4 to 0xa3950.
  Established. The absolute word at `0x29a5e8` is read at multiple sites, including `0xa34ec` and `0xa3558` (`ldr` at 0xa34ec, `ldr` at 0xa3558).
  An included byte store writes `([sp-32] & 0xff)` to an unknown target (`strb` at 0xa38b8).
  A halfword store writes `([sp-40] & 0xffff)` to an unknown target (`strh` at 0xa3938).
  Outbound capture layout remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa34b4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa34b8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa34bc | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xa34c0 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xa34c4 | load | 4 | `abs 0xa3944` | `-` |
| `ldr` at 0xa34c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa34ec | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa34f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa3520 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xa3528 | load | 1 | `(unknown)` | `-` |
| `str` at 0xa352c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xa3530 | load | 4 | `abs 0xa3944` | `-` |
| `ldr` at 0xa3534 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3558 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa3560 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa358c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa3594 | load | 4 | `abs 0xa3948` | `-` |
| `ldrh` at 0xa3598 | load | 2 | `(unknown)` | `-` |
| `str` at 0xa359c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xa35a0 | load | 4 | `abs 0xa3944` | `-` |
| `ldr` at 0xa35a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa35c8 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa35d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa35d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa35e4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa35f0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xa35f4 | load | 4 | `abs 0xa3944` | `-` |
| `ldr` at 0xa35f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa361c | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa3624 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa3650 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa3658 | load | 4 | `abs 0xa3944` | `-` |
| `ldr` at 0xa365c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3680 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa3688 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa36b4 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xa36bc | load | 1 | `(unknown)` | `-` |
| `strb` at 0xa36dc | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xa36e0 | load | 4 | `abs 0xa3944` | `-` |
| `ldr` at 0xa36e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3708 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa3710 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa373c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa3744 | load | 4 | `abs 0xa3948` | `-` |
| `ldr` at 0xa3748 | load | 4 | `abs 0xa3944` | `-` |
| `ldr` at 0xa374c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3770 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa3778 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa37a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa37ac | load | 4 | `abs 0xa3948` | `-` |
| `ldrh` at 0xa37b0 | load | 2 | `(unknown)` | `-` |
| `strh` at 0xa37b8 | store | 2 | `(unknown)` | `?` |
| `ldr` at 0xa37bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa37e8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa37f4 | load | 4 | `abs 0xa3944` | `-` |
| `ldr` at 0xa37f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa381c | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa3824 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa3834 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa3838 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa3840 | load | 4 | `abs 0xa3944` | `-` |
| `ldr` at 0xa3844 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3868 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa3870 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa389c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa38a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa38b4 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xa38b8 | store | 1 | `(unknown)` | `([sp-32] & 0xff)` |
| `ldr` at 0xa38bc | load | 4 | `abs 0xa3944` | `-` |
| `ldr` at 0xa38c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa38e4 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa38ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa3918 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa3920 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa3924 | load | 4 | `abs 0xa394c` | `-` |
| `ldr` at 0xa3934 | load | 4 | `sp -0x28` | `-` |
| `strh` at 0xa3938 | store | 2 | `(unknown)` | `([sp-40] & 0xffff)` |
| `ldm` at 0xa3940 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa3940 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa3940 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa383c | `(unknown callee) 0x19610` | `?` | `[sp-28]` | `[sp-32]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0xa35dc | `? > [sp-24] (signed, 32-bit)` | `? <= [sp-24] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa3940 | `r0` | `0x29a5d4` |

## `SysComCaptureReset`

- `SysComCaptureReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa3950 to 0xa3cd0.
  Established. A nonzero `[0x133b70]` takes `0xa3b48`, while zero falls through (`bne` at 0xa396c).
  An included word assignment at `0x133b70` writes `0x1` (`str` at 0xa3978).
  Included absolute word stores write `0xa2e34` to `0x133830` and `0xa34a4` to `0x133834` (`str` at 0xa3b38, `str` at 0xa3b44).
  Capture buffer reset policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xa3960 | load | 4 | `abs 0xa3cb4` | `-` |
| `ldr` at 0xa3964 | load | 4 | `abs 0x133b70` | `-` |
| `ldr` at 0xa3970 | load | 4 | `abs 0xa3cb4` | `-` |
| `str` at 0xa3978 | store | 4 | `abs 0x133b70` | `0x1` |
| `ldr` at 0xa397c | load | 4 | `abs 0xa3cb8` | `-` |
| `ldr` at 0xa3980 | load | 4 | `abs 0x29a5dc` | `-` |
| `ldr` at 0xa3984 | load | 4 | `[0x29a5dc] +0x10` | `-` |
| `str` at 0xa3998 | store | 4 | `sp -0x1c` | `0x14` |
| `str` at 0xa39a4 | store | 4 | `sp -0x1c` | `0x20` |
| `str` at 0xa39ac | store | 4 | `sp -0x1c` | `0x8` |
| `str` at 0xa39b4 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xa39b8 | load | 4 | `abs 0xa3cb8` | `-` |
| `ldr` at 0xa39bc | load | 4 | `abs 0x29a5e4` | `-` |
| `ldr` at 0xa39c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa39cc | load | 4 | `abs 0xa3cb8` | `-` |
| `ldr` at 0xa39d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa39f4 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa39fc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa3a00 | store | 4 | `(unknown)` | `[sp-28]` |
| `ldr` at 0xa3a04 | load | 4 | `abs 0xa3cb8` | `-` |
| `ldr` at 0xa3a08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa3a2c | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa3a34 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa3a6c | store | 4 | `(unknown)` | `ret(0x1943c)` |
| `ldr` at 0xa3a70 | load | 4 | `abs 0xa3cb8` | `-` |
| `ldr` at 0xa3a74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa3a98 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa3aa0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa3ad8 | store | 4 | `(unknown)` | `ret(0x1943c)` |
| `ldr` at 0xa3adc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa3ae4 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0xa3aec | load | 4 | `abs 0xa3cb8` | `-` |
| `ldr` at 0xa3af0 | load | 4 | `abs 0x29a5e4` | `-` |
| `ldr` at 0xa3af4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa3b24 | load | 4 | `abs 0xa3cbc` | `-` |
| `ldr` at 0xa3b30 | load | 4 | `abs 0xa3cc0` | `-` |
| `ldr` at 0xa3b34 | load | 4 | `abs 0xa3cc4` | `-` |
| `str` at 0xa3b38 | store | 4 | `abs 0x133830` | `0xa2e34` |
| `ldr` at 0xa3b3c | load | 4 | `abs 0xa3cc8` | `-` |
| `ldr` at 0xa3b40 | load | 4 | `abs 0xa3ccc` | `-` |
| `str` at 0xa3b44 | store | 4 | `abs 0x133834` | `0xa34a4` |
| `str` at 0xa3b4c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xa3b50 | load | 4 | `abs 0xa3cb8` | `-` |
| `ldr` at 0xa3b54 | load | 4 | `abs 0x29a5e4` | `-` |
| `ldr` at 0xa3b58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa3b64 | load | 4 | `abs 0xa3cb8` | `-` |
| `ldr` at 0xa3b68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa3b8c | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa3b94 | load | 4 | `abs 0xa3cb8` | `-` |
| `ldr` at 0xa3b98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa3bbc | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa3bc4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa3bf0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa3c00 | load | 4 | `abs 0xa3cb8` | `-` |
| `ldr` at 0xa3c04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa3c28 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa3c30 | load | 4 | `abs 0xa3cb8` | `-` |
| `ldr` at 0xa3c34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa3c58 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa3c60 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa3c8c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa3c9c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa3ca4 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldm` at 0xa3cb0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xa3cb0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa3cb0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa3cb0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa3a64 | `(unknown callee) 0x1943c` | `?` | `[sp-24]` | `[sp-28]` | `?` |
| `bl` at 0xa3ad0 | `(unknown callee) 0x1943c` | `?` | `[sp-24]` | `[sp-28]` | `?` |
| `bl` at 0xa3b2c | `(unknown callee) 0x1e958` | `0x113350` | `?` | `?` | `?` |
| `bl` at 0xa3bfc | `(unknown callee) 0x19748` | `?` | `0x0` | `?` | `?` |
| `bl` at 0xa3c98 | `(unknown callee) 0x19748` | `?` | `0x0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa396c | `[0x133b70] != 0x0` | `[0x133b70] == 0x0` |
| `beq` at 0xa398c | `[[0x29a5dc]+16] == 0x40` | `[[0x29a5dc]+16] != 0x40` |
| `ble` at 0xa39c8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ble` at 0xa3b60 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa3cb0 | `r0` | `?` |

## `SysComCaptureDump`

- `SysComCaptureDump`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa3cd0 to 0xa4308.
  Established. The signed condition `[0x29a5e4] > r0` takes `0xa3d34`, while less or equal values fall through (`bgt` at 0xa3cfc).
  An included word store through entry `r2 +0x0` writes `ret(0x1943c)` (`str` at 0xa3d10).
  A zero `ret(0x192d4)` takes `0xa41c8`, while nonzero falls through (`beq` at 0xa4164).
  Capture dump formatting remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa3ce0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xa3ce4 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xa3ce8 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0xa3cec | load | 4 | `abs 0xa42d8` | `-` |
| `ldr` at 0xa3cf0 | load | 4 | `abs 0x29a5e4` | `-` |
| `ldr` at 0xa3cf4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa3d00 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa3d10 | store | 4 | `r2 +0x0` | `ret(0x1943c)` |
| `ldr` at 0xa3d14 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa3d18 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xa3d1c | load | 4 | `abs 0xa42dc` | `-` |
| `ldr` at 0xa3d20 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa3d2c | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xa3d34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa3d40 | load | 4 | `abs 0xa42d8` | `-` |
| `ldr` at 0xa3d44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa3d68 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa3d70 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa3d74 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xa3d7c | load | 4 | `abs 0xa42d8` | `-` |
| `ldr` at 0xa3d80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa3da4 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa3dac | load | 4 | `(unknown)` | `-` |
| `str` at 0xa3db0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xa3db4 | load | 4 | `abs 0xa42d8` | `-` |
| `ldr` at 0xa3db8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa3ddc | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0xa3de4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa3de8 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xa3dec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa3df8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa3e08 | store | 4 | `[sp-32] +0x0` | `ret(0x1943c)` |
| `ldr` at 0xa3e0c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa3e10 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xa3e14 | load | 4 | `abs 0xa42e0` | `-` |
| `str` at 0xa3e20 | store | 4 | `sp -0x4c` | `?` |
| `str` at 0xa3e2c | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0xa3e34 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xa3e38 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa3e3c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xa3e4c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xa3e50 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa3e5c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa3e88 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa3e94 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xa3ea0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa3eb0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xa3ebc | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xa3ec0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xa3ec8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xa3ed0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa3ed8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xa3ee0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa3ee4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xa3ef8 | store | 4 | `[sp-32] +0x0` | `ret(0x1943c)` |
| `ldr` at 0xa3efc | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0xa3f00 | store | 4 | `sp -0x40` | `[[sp-32]]` |
| `str` at 0xa3f08 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xa3f0c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa3f10 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa3f1c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa3f48 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa3f50 | load | 4 | `abs 0xa42e4` | `-` |
| `ldr` at 0xa3f54 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa3f58 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xa3f5c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa3f88 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa3f90 | load | 4 | `abs 0xa42e8` | `-` |
| `ldr` at 0xa3f94 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa3f98 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xa3f9c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa3fa0 | load | 4 | `abs 0xa42ec` | `-` |
| `ldr` at 0xa3fa4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa3fa8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa3fb0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa3fc0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa3fc8 | store | 4 | `sp -0x40` | `([sp-64] + ret(0x19538))` |
| `ldr` at 0xa3fcc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa3ff8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xa4000 | load | 1 | `(unknown)` | `-` |
| `str` at 0xa4008 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xa4010 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xa4014 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa4020 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa404c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa4058 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xa4064 | load | 1 | `(unknown)` | `-` |
| `str` at 0xa4068 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xa406c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xa4078 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa40a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa40ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa40b0 | load | 4 | `abs 0xa42f0` | `-` |
| `ldrh` at 0xa40c0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xa40c4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa40c8 | load | 4 | `abs 0xa42f4` | `-` |
| `ldr` at 0xa40d4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa40e4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa40ec | store | 4 | `sp -0x40` | `([sp-64] + ret(0x19538))` |
| `str` at 0xa40f4 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xa40f8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xa40fc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xa4108 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa4134 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa413c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa4148 | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0xa4150 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa4168 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa4194 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa419c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa41a8 | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0xa41b0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa41b4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa41b8 | load | 4 | `abs 0xa42f8` | `-` |
| `ldr` at 0xa41c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa41f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa41fc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa4208 | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0xa4210 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa4214 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa4218 | load | 4 | `abs 0xa42fc` | `-` |
| `ldr` at 0xa4224 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa422c | store | 4 | `sp -0x40` | `[sp-64]+3` |
| `ldr` at 0xa4230 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xa4238 | store | 4 | `sp -0x48` | `[sp-72]+1` |
| `ldr` at 0xa4240 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa4244 | load | 4 | `abs 0xa4300` | `-` |
| `ldr` at 0xa424c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa425c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa4264 | store | 4 | `sp -0x40` | `([sp-64] + ret(0x19538))` |
| `ldr` at 0xa4268 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xa4288 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xa428c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xa4294 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xa429c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa42a4 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xa42ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa42b0 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xa42b4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa42c0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa42c4 | load | 4 | `abs 0xa4304` | `-` |
| `ldr` at 0xa42cc | load | 4 | `sp -0x4c` | `-` |
| `ldm` at 0xa42d4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xa42d4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa42d4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa42d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa3d08 | `(unknown callee) 0x1943c` | `0x50` | `r1` | `[0x29a5e4]` | `r0` |
| `bl` at 0xa3d24 | `(unknown callee) 0x197a8` | `[[sp-32]]` | `0x113368` | `[sp-24]` | `[sp-32]` |
| `bl` at 0xa3e00 | `(unknown callee) 0x1943c` | `0x50` | `[sp-24]` | `?` | `[sp-36]` |
| `bl` at 0xa3e18 | `(unknown callee) 0x19808` | `[[sp-32]]` | `0x113388` | `?` | `[sp-32]` |
| `bl` at 0xa3ef0 | `(unknown callee) 0x1943c` | `?` | `?` | `?` | `?` |
| `bl` at 0xa3fac | `(unknown callee) 0x197a8` | `?` | `0x1133bc` | `[sp-24]` | `?` |
| `bl` at 0xa3fb4 | `(unknown callee) 0x19538` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xa40d0 | `(unknown callee) 0x197a8` | `?` | `0x1133e4` | `?` | `?` |
| `bl` at 0xa40d8 | `(unknown callee) 0x19538` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xa4158 | `(unknown callee) 0x192d4` | `?` | `?` | `?` | `?` |
| `bl` at 0xa41c0 | `(unknown callee) 0x197a8` | `[sp-64]` | `0x1133ec` | `?` | `?` |
| `bl` at 0xa4220 | `(unknown callee) 0x197a8` | `[sp-64]` | `0x1133f4` | `?` | `?` |
| `bl` at 0xa4248 | `(unknown callee) 0x19808` | `?` | `0x1133fc` | `?` | `[sp-68]` |
| `bl` at 0xa4250 | `(unknown callee) 0x19538` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xa42c8 | `(unknown callee) 0x195e0` | `?` | `0x113400` | `[[sp-32]]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0xa3cfc | `[0x29a5e4] > r0 (signed, 32-bit)` | `[0x29a5e4] <= r0 (signed, 32-bit)` |
| `bne` at 0xa3d3c | `r1 != 0x0` | `r1 == 0x0` |
| `bne` at 0xa3df4 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `bge` at 0xa3e44 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bgt` at 0xa3e58 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bge` at 0xa3f18 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bgt` at 0xa401c | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0xa4074 | `[sp-68] == 0x0` | `[sp-68] != 0x0` |
| `bge` at 0xa4104 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xa4164 | `ret(0x192d4) == 0x0` | `ret(0x192d4) != 0x0` |
| `bne` at 0xa42bc | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa42d4 | `r0` | `[sp-76]` |

## `IRouterFreePacket`

- `IRouterFreePacket`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa8b98 to 0xa8f60.
  Established. The halfword through `[sp-20] +0x4` receives `([sp-40]:2 & 0xffff)` (`strh` at 0xa8d68).
  An included absolute word store clears `0x133c1c` (`str` at 0xa8e6c).
  Another writes `[0x133c14]-1` to `0x133c14` (`str` at 0xa8e94).
  Packet lifetime and reference semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa8ba8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa8bac | load | 4 | `abs 0xa8f1c` | `-` |
| `str` at 0xa8bbc | store | 4 | `sp -0x20` | `ret(0x20cd0)` |
| `ldr` at 0xa8bc0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa8bcc | load | 4 | `abs 0xa8f20` | `-` |
| `str` at 0xa8bd0 | store | 4 | `sp -0x34` | `0x113e78` |
| `ldr` at 0xa8bd4 | load | 4 | `abs 0xa8f1c` | `-` |
| `str` at 0xa8bd8 | store | 4 | `sp -0x30` | `0x133c78` |
| `ldr` at 0xa8bdc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa8be0 | store | 4 | `sp -0x2c` | `ret(0x20cd0)` |
| `ldr` at 0xa8be8 | load | 4 | `abs 0xa8f24` | `-` |
| `ldr` at 0xa8bec | load | 4 | `abs 0xa8f28` | `-` |
| `ldr` at 0xa8bf8 | load | 4 | `abs 0xa8f2c` | `-` |
| `ldr` at 0xa8bfc | load | 4 | `abs 0x133c04` | `-` |
| `ldr` at 0xa8c08 | load | 4 | `abs 0xa8f30` | `-` |
| `ldr` at 0xa8c0c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xa8c10 | load | 2 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xa8c24 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0xa8c28 | load | 4 | `abs 0xa8f34` | `-` |
| `ldr` at 0xa8c2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8c34 | load | 4 | `abs 0xa8f30` | `-` |
| `ldr` at 0xa8c38 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0xa8c40 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xa8c44 | load | 2 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xa8c64 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa8c68 | load | 1 | `[sp-20] +0x16` | `-` |
| `ldr` at 0xa8c6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8c70 | load | 4 | `abs 0xa8f38` | `-` |
| `ldr` at 0xa8c78 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0xa8c84 | load | 4 | `abs 0xa8f1c` | `-` |
| `str` at 0xa8c90 | store | 4 | `sp -0x20` | `ret(0x2105c)` |
| `ldr` at 0xa8c94 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa8ca0 | load | 4 | `abs 0xa8f3c` | `-` |
| `str` at 0xa8ca4 | store | 4 | `sp -0x34` | `0x113ed0` |
| `ldr` at 0xa8ca8 | load | 4 | `abs 0xa8f1c` | `-` |
| `str` at 0xa8cac | store | 4 | `sp -0x30` | `0x133c78` |
| `ldr` at 0xa8cb0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa8cb4 | store | 4 | `sp -0x2c` | `ret(0x2105c)` |
| `ldr` at 0xa8cbc | load | 4 | `abs 0xa8f24` | `-` |
| `ldr` at 0xa8cc0 | load | 4 | `abs 0xa8f40` | `-` |
| `str` at 0xa8cd0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xa8cd8 | load | 4 | `abs 0xa8f44` | `-` |
| `ldr` at 0xa8cdc | load | 4 | `abs 0x133c1c` | `-` |
| `ldr` at 0xa8ce8 | load | 4 | `abs 0xa8f44` | `-` |
| `ldr` at 0xa8cec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8cf0 | load | 4 | `abs 0x133c1c` | `-` |
| `ldr` at 0xa8cfc | load | 4 | `abs 0xa8f30` | `-` |
| `ldr` at 0xa8d00 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xa8d04 | load | 2 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xa8d18 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0xa8d1c | load | 4 | `abs 0xa8f48` | `-` |
| `ldr` at 0xa8d20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8d28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8d2c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xa8d30 | load | 2 | `[sp-20] +0x4` | `-` |
| `ldrh` at 0xa8d3c | load | 2 | `[sp-20] +0x4` | `-` |
| `strh` at 0xa8d40 | store | 2 | `sp -0x28` | `([[sp-20]+4]:2 & 0xffff)` |
| `ldr` at 0xa8d44 | load | 4 | `abs 0xa8f4c` | `-` |
| `ldrh` at 0xa8d50 | load | 2 | `sp -0x28` | `-` |
| `strh` at 0xa8d60 | store | 2 | `sp -0x28` | `?` |
| `ldrh` at 0xa8d64 | load | 2 | `sp -0x28` | `-` |
| `strh` at 0xa8d68 | store | 2 | `[sp-20] +0x4` | `([sp-40]:2 & 0xffff)` |
| `ldr` at 0xa8d88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8d8c | load | 4 | `[sp-20] +0x8` | `-` |
| `str` at 0xa8d9c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xa8da0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa8da4 | load | 1 | `[sp-20] +0x20` | `-` |
| `ldr` at 0xa8da8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa8db4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8db8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa8dc0 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xa8dc8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa8dd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8dd8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa8de0 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xa8de8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa8df0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa8df8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xa8e00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8e04 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xa8e0c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa8e10 | store | 4 | `sp -0x1c` | `[sp-20]` |
| `ldr` at 0xa8e14 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa8e20 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa8e24 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xa8e28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa8e2c | load | 4 | `(unknown)` | `-` |
| `str` at 0xa8e30 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xa8e34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8e40 | load | 4 | `abs 0xa8f44` | `-` |
| `ldr` at 0xa8e44 | load | 4 | `abs 0x133c1c` | `-` |
| `ldr` at 0xa8e50 | load | 4 | `abs 0xa8f44` | `-` |
| `ldr` at 0xa8e54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8e58 | load | 4 | `abs 0x133c1c` | `-` |
| `ldr` at 0xa8e64 | load | 4 | `abs 0xa8f44` | `-` |
| `str` at 0xa8e6c | store | 4 | `abs 0x133c1c` | `0x0` |
| `ldr` at 0xa8e70 | load | 4 | `abs 0xa8f30` | `-` |
| `ldr` at 0xa8e74 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0xa8e78 | load | 4 | `abs 0xa8f50` | `-` |
| `ldr` at 0xa8e7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8e84 | load | 4 | `abs 0xa8f54` | `-` |
| `ldr` at 0xa8e88 | load | 4 | `abs 0xa8f54` | `-` |
| `ldr` at 0xa8e8c | load | 4 | `abs 0x133c14` | `-` |
| `str` at 0xa8e94 | store | 4 | `abs 0x133c14` | `[0x133c14]-1` |
| `ldr` at 0xa8e98 | load | 4 | `abs 0xa8f2c` | `-` |
| `ldr` at 0xa8e9c | load | 4 | `abs 0x133c04` | `-` |
| `ldr` at 0xa8ea8 | load | 4 | `abs 0xa8f30` | `-` |
| `ldr` at 0xa8eac | load | 4 | `abs 0xa8f54` | `-` |
| `ldr` at 0xa8eb0 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0xa8eb4 | load | 4 | `abs 0xa8f58` | `-` |
| `ldr` at 0xa8eb8 | load | 4 | `abs 0x133c14` | `-` |
| `ldr` at 0xa8ec0 | load | 4 | `abs 0xa8f1c` | `-` |
| `str` at 0xa8ecc | store | 4 | `sp -0x20` | `ret(0x2105c)` |
| `ldr` at 0xa8ed0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa8edc | load | 4 | `abs 0xa8f3c` | `-` |
| `str` at 0xa8ee0 | store | 4 | `sp -0x34` | `0x113ed0` |
| `ldr` at 0xa8ee4 | load | 4 | `abs 0xa8f1c` | `-` |
| `str` at 0xa8ee8 | store | 4 | `sp -0x30` | `0x133c78` |
| `ldr` at 0xa8eec | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa8ef0 | store | 4 | `sp -0x2c` | `ret(0x2105c)` |
| `ldr` at 0xa8ef8 | load | 4 | `abs 0xa8f24` | `-` |
| `ldr` at 0xa8efc | load | 4 | `abs 0xa8f5c` | `-` |
| `str` at 0xa8f0c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xa8f10 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xa8f18 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa8f18 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa8f18 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa8bb4 | `(unknown callee) 0x20cd0` | `0x133c78` | `?` | `r2` | `r3` |
| `bl` at 0xa8bf4 | `(unknown callee) 0x200a0` | `0x9` | `0x113ea4` | `0x3d9` | `0x0` |
| `bl` at 0xa8c30 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0x113f34` | `[sp-20]` | `?` |
| `bl` at 0xa8c3c | `(unknown callee) 0x19334` | `[0x12eeb8]` | `?` | `?` | `0x12eeb8` |
| `bl` at 0xa8c5c | `(unknown callee) 0x2091c` | `?` | `?` | `?` | `?` |
| `bl` at 0xa8c80 | `(unknown callee) 0x1e958` | `0x113f44` | `[[sp-20]+22]:1` | `[[sp-20]+28]` | `ret(0x2091c)` |
| `bl` at 0xa8c88 | `(unknown callee) 0x2105c` | `0x133c78` | `?` | `?` | `?` |
| `bl` at 0xa8cc8 | `(unknown callee) 0x200a0` | `0x9` | `0x113ea4` | `0x3e6` | `0x0` |
| `bl` at 0xa8d24 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0x113f70` | `[sp-20]` | `?` |
| `bl` at 0xa8dec | `(unknown callee) 0x1970c` | `?` | `[sp-20]` | `?` | `?` |
| `bl` at 0xa8e08 | `(unknown callee) 0x1970c` | `[[sp-20]+8]` | `?` | `[[sp-20]+32]:1` | `[sp-20]` |
| `bl` at 0xa8e38 | `(unknown callee) 0x1970c` | `?` | `?` | `?` | `?` |
| `bl` at 0xa8e80 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0x113f7c` | `[sp-20]` | `0x12eeb8` |
| `bl` at 0xa8ebc | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0x113f88` | `[0x133c14]` | `0x12eeb8` |
| `bl` at 0xa8ec4 | `(unknown callee) 0x2105c` | `0x133c78` | `?` | `?` | `?` |
| `bl` at 0xa8f04 | `(unknown callee) 0x200a0` | `0x9` | `0x113ea4` | `0x416` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa8bc8 | `ret(0x20cd0) == 0x0` | `ret(0x20cd0) != 0x0` |
| `beq` at 0xa8c04 | `[0x133c04] == 0x0` | `[0x133c04] != 0x0` |
| `bne` at 0xa8c58 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xa8c9c | `ret(0x2105c) == 0x0` | `ret(0x2105c) != 0x0` |
| `beq` at 0xa8ce4 | `[0x133c1c] == 0x0` | `[0x133c1c] != 0x0` |
| `bne` at 0xa8cf8 | `[sp-20] != [0x133c1c]` | `[sp-20] == [0x133c1c]` |
| `bne` at 0xa8d84 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xa8d94 | `[[sp-20]+8] == 0x0` | `[[sp-20]+8] != 0x0` |
| `ble` at 0xa8db0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xa8dd0 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xa8e1c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xa8e4c | `[0x133c1c] == 0x0` | `[0x133c1c] != 0x0` |
| `bne` at 0xa8e60 | `[sp-20] != [0x133c1c]` | `[sp-20] == [0x133c1c]` |
| `beq` at 0xa8ea4 | `[0x133c04] == 0x0` | `[0x133c04] != 0x0` |
| `beq` at 0xa8ed8 | `ret(0x2105c) == 0x0` | `ret(0x2105c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa8f18 | `r0` | `0x0` |

## `IRouterSetFrameByteOrder`

- `IRouterSetFrameByteOrder`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaab28 to 0xaadf4.
  Established. Entry `r1 != 0x2` takes `0xaacb0`, while equality falls through (`bne` at 0xaab6c).
  Separate included halfword stores through entry `r0 +0x10` write `(ret(0x19514) & 0xffff)` and `(ret(0x19730) & 0xffff)` (`strh` at 0xaab88, `strh` at 0xaacc8).
  Byte-order transformation details remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaab38 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xaab3c | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xaab40 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xaab44 | load | 1 | `r0 +0x6` | `-` |
| `ldr` at 0xaab54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaab64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaab70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaab74 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xaab78 | load | 2 | `r0 +0x10` | `-` |
| `strh` at 0xaab88 | store | 2 | `r0 +0x10` | `(ret(0x19514) & 0xffff)` |
| `ldr` at 0xaab8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaab90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaab94 | load | 4 | `[sp-24] +0x18` | `-` |
| `str` at 0xaaba0 | store | 4 | `[sp-24] +0x18` | `ret(0x19280)` |
| `ldr` at 0xaaba4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaaba8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaabac | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xaabb8 | store | 4 | `[sp-24] +0x1c` | `ret(0x19280)` |
| `ldr` at 0xaabbc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaabc0 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xaabc4 | load | 2 | `[sp-24] +0x22` | `-` |
| `strh` at 0xaabd4 | store | 2 | `[sp-24] +0x22` | `(ret(0x19514) & 0xffff)` |
| `str` at 0xaabdc | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xaabe0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xaabe4 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0xaabe8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xaabf4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaabf8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xaac0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaac10 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xaac24 | load | 2 | `(unknown)` | `-` |
| `strh` at 0xaac34 | store | 2 | `(unknown)` | `(ret(0x19514) & 0xffff)` |
| `ldr` at 0xaac38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaac3c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xaac50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaac54 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xaac68 | load | 4 | `(unknown)` | `-` |
| `str` at 0xaac74 | store | 4 | `(unknown)` | `ret(0x19280)` |
| `ldr` at 0xaac78 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xaac80 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xaac88 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xaac8c | load | 1 | `[sp-24] +0x6` | `-` |
| `strb` at 0xaac90 | store | 1 | `sp -0x24` | `([[sp-24]+6]:1 & 0xff)` |
| `ldrb` at 0xaac94 | load | 1 | `sp -0x24` | `-` |
| `strb` at 0xaaca0 | store | 1 | `sp -0x24` | `?` |
| `ldrb` at 0xaaca4 | load | 1 | `sp -0x24` | `-` |
| `strb` at 0xaaca8 | store | 1 | `[sp-24] +0x6` | `([sp-36]:1 & 0xff)` |
| `ldr` at 0xaacb0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaacb4 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xaacb8 | load | 2 | `r0 +0x10` | `-` |
| `strh` at 0xaacc8 | store | 2 | `r0 +0x10` | `(ret(0x19730) & 0xffff)` |
| `ldr` at 0xaaccc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaacd0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaacd4 | load | 4 | `[sp-24] +0x18` | `-` |
| `str` at 0xaace0 | store | 4 | `[sp-24] +0x18` | `ret(0x19388)` |
| `ldr` at 0xaace4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaace8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaacec | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xaacf8 | store | 4 | `[sp-24] +0x1c` | `ret(0x19388)` |
| `ldr` at 0xaacfc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaad00 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xaad04 | load | 2 | `[sp-24] +0x22` | `-` |
| `strh` at 0xaad14 | store | 2 | `[sp-24] +0x22` | `(ret(0x19730) & 0xffff)` |
| `str` at 0xaad1c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xaad20 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xaad24 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0xaad28 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xaad34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaad38 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xaad4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaad50 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xaad64 | load | 2 | `(unknown)` | `-` |
| `strh` at 0xaad74 | store | 2 | `(unknown)` | `(ret(0x19730) & 0xffff)` |
| `ldr` at 0xaad78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaad7c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xaad90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaad94 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xaada8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xaadb4 | store | 4 | `(unknown)` | `ret(0x19388)` |
| `ldr` at 0xaadb8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xaadc0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xaadc8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xaadcc | load | 1 | `[sp-24] +0x6` | `-` |
| `strb` at 0xaadd0 | store | 1 | `sp -0x28` | `([[sp-24]+6]:1 & 0xff)` |
| `ldrb` at 0xaadd4 | load | 1 | `sp -0x28` | `-` |
| `strb` at 0xaade0 | store | 1 | `sp -0x28` | `?` |
| `ldrb` at 0xaade4 | load | 1 | `sp -0x28` | `-` |
| `strb` at 0xaade8 | store | 1 | `[sp-24] +0x6` | `([sp-40]:1 & 0xff)` |
| `ldm` at 0xaadf0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xaadf0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xaadf0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xaadf0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaab80 | `(unknown callee) 0x19514` | `[r0+16]:2` | `r1` | `?` | `[r0+16]:2` |
| `bl` at 0xaab98 | `(unknown callee) 0x19280` | `[[sp-24]+24]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xaabb0 | `(unknown callee) 0x19280` | `[[sp-24]+28]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xaabcc | `(unknown callee) 0x19514` | `[[sp-24]+34]:2` | `?` | `?` | `[[sp-24]+34]:2` |
| `bl` at 0xaac2c | `(unknown callee) 0x19514` | `?` | `0x24` | `[sp-24]` | `?` |
| `bl` at 0xaac6c | `(unknown callee) 0x19280` | `?` | `0x28` | `[sp-24]` | `?` |
| `bl` at 0xaacc0 | `(unknown callee) 0x19730` | `[r0+16]:2` | `r1` | `?` | `[r0+16]:2` |
| `bl` at 0xaacd8 | `(unknown callee) 0x19388` | `[[sp-24]+24]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xaacf0 | `(unknown callee) 0x19388` | `[[sp-24]+28]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xaad0c | `(unknown callee) 0x19730` | `[[sp-24]+34]:2` | `?` | `?` | `[[sp-24]+34]:2` |
| `bl` at 0xaad6c | `(unknown callee) 0x19730` | `?` | `0x24` | `[sp-24]` | `?` |
| `bl` at 0xaadac | `(unknown callee) 0x19388` | `?` | `0x28` | `[sp-24]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xaab5c | `? != r1` | `? == r1` |
| `bne` at 0xaab6c | `r1 != 0x2` | `r1 == 0x2` |
| `ble` at 0xaabf0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ble` at 0xaad30 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xaadf0 | `r0` | `?` |

## `SMPCmdIRouter`

- `SMPCmdIRouter`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8e46c to 0x8e77c.
  Established. A zero `ret(0xa8b38)` takes `0x8e4e4`, while nonzero falls through (`beq` at 0x8e4b4).
  The absolute word at `0x13390c` receives zero on an included path (`str` at 0x8e6e4).
  The unnamed `0xaa090` call carries `0x29a6b4`, `0x0`, `[sp-2104]`, `[sp-32]` (`bl` at 0x8e708).
  Command dispatch policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8e480 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x8e484 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x8e498 | load | 4 | `abs 0x8e754` | `-` |
| `str` at 0x8e4a8 | store | 4 | `sp -0x1c` | `ret(0xa8b38)` |
| `ldr` at 0x8e4ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8e4b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8e4c4 | load | 4 | `abs 0x8e758` | `-` |
| `str` at 0x8e4c8 | store | 4 | `sp -0x848` | `0x10a4e4` |
| `str` at 0x8e4cc | store | 4 | `sp -0x844` | `ret(0x1f8e0)` |
| `ldr` at 0x8e4d4 | load | 4 | `abs 0x8e75c` | `-` |
| `ldr` at 0x8e4d8 | load | 4 | `abs 0x8e760` | `-` |
| `ldr` at 0x8e4e4 | load | 4 | `sp -0x838` | `-` |
| `ldrb` at 0x8e4e8 | load | 1 | `[sp-2104] +0x20` | `-` |
| `str` at 0x8e4f8 | store | 4 | `sp -0x834` | `0x0` |
| `ldr` at 0x8e500 | load | 4 | `sp -0x838` | `-` |
| `ldrb` at 0x8e504 | load | 1 | `[sp-2104] +0x2c` | `-` |
| `ldr` at 0x8e514 | load | 4 | `sp -0x838` | `-` |
| `ldr` at 0x8e518 | load | 4 | `[sp-2104] +0x30` | `-` |
| `str` at 0x8e51c | store | 4 | `sp -0x840` | `[[sp-2104]+48]` |
| `ldr` at 0x8e524 | load | 4 | `sp -0x838` | `-` |
| `str` at 0x8e530 | store | 4 | `sp -0x840` | `ret(0xaadf4)` |
| `ldr` at 0x8e534 | load | 4 | `sp -0x840` | `-` |
| `str` at 0x8e538 | store | 4 | `sp -0x834` | `[sp-2112]` |
| `ldr` at 0x8e548 | load | 4 | `sp -0x834` | `-` |
| `ldr` at 0x8e550 | load | 4 | `sp -0x838` | `-` |
| `str` at 0x8e55c | store | 4 | `sp -0x1c` | `ret(0x8d178)` |
| `ldr` at 0x8e578 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8e580 | load | 4 | `sp -0x82c` | `-` |
| `str` at 0x8e590 | store | 4 | `sp -0x830` | `0x0` |
| `ldr` at 0x8e594 | load | 4 | `sp -0x830` | `-` |
| `ldr` at 0x8e59c | load | 4 | `sp -0x82c` | `-` |
| `ldr` at 0x8e5a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8e5b0 | load | 4 | `sp -0x830` | `-` |
| `ldr` at 0x8e5b8 | load | 4 | `sp -0x82c` | `-` |
| `ldr` at 0x8e5c0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8e5c4 | store | 4 | `sp -0x83c` | `?` |
| `ldr` at 0x8e5c8 | load | 4 | `sp -0x830` | `-` |
| `ldr` at 0x8e5d0 | load | 4 | `sp -0x82c` | `-` |
| `ldr` at 0x8e5d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8e5f8 | load | 4 | `sp -0x830` | `-` |
| `ldr` at 0x8e600 | load | 4 | `sp -0x82c` | `-` |
| `str` at 0x8e60c | store | 4 | `sp -0x848` | `0x0` |
| `ldr` at 0x8e610 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8e61c | load | 4 | `sp -0x830` | `-` |
| `str` at 0x8e624 | store | 4 | `sp -0x830` | `[sp-2096]+1` |
| `ldr` at 0x8e62c | load | 4 | `sp -0x82c` | `-` |
| `ldr` at 0x8e634 | load | 4 | `abs 0x8e764` | `-` |
| `ldr` at 0x8e638 | load | 4 | `abs 0x133ad0` | `-` |
| `ldr` at 0x8e644 | load | 4 | `abs 0x8e768` | `-` |
| `ldr` at 0x8e648 | load | 4 | `abs 0x13390c` | `-` |
| `ldr` at 0x8e658 | load | 4 | `abs 0x8e764` | `-` |
| `ldr` at 0x8e668 | load | 4 | `abs 0x133ad0` | `-` |
| `ldr` at 0x8e670 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x8e684 | store | 4 | `sp -0x20` | `ret(unknown)` |
| `ldr` at 0x8e688 | load | 4 | `abs 0x8e764` | `-` |
| `ldr` at 0x8e68c | load | 4 | `abs 0x133ad0` | `-` |
| `ldr` at 0x8e698 | load | 4 | `abs 0x8e768` | `-` |
| `ldr` at 0x8e69c | load | 4 | `abs 0x13390c` | `-` |
| `ldr` at 0x8e6ac | load | 4 | `abs 0x8e764` | `-` |
| `ldr` at 0x8e6bc | load | 4 | `abs 0x133ad0` | `-` |
| `ldr` at 0x8e6c4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x8e6d8 | store | 4 | `sp -0x20` | `ret(unknown)` |
| `ldr` at 0x8e6dc | load | 4 | `abs 0x8e768` | `-` |
| `str` at 0x8e6e4 | store | 4 | `abs 0x13390c` | `0x0` |
| `str` at 0x8e6f4 | store | 4 | `sp -0x848` | `sp-2088` |
| `ldr` at 0x8e6f8 | load | 4 | `abs 0x8e76c` | `-` |
| `ldr` at 0x8e700 | load | 4 | `sp -0x838` | `-` |
| `ldr` at 0x8e704 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x8e710 | store | 4 | `sp -0x1c` | `ret(0xaa090)` |
| `ldr` at 0x8e714 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8e720 | load | 4 | `abs 0x8e770` | `-` |
| `ldr` at 0x8e724 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x8e728 | load | 4 | `abs 0x8e774` | `-` |
| `ldr` at 0x8e730 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8e73c | load | 4 | `abs 0x8e778` | `-` |
| `ldr` at 0x8e748 | load | 4 | `sp -0x838` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8e4a0 | `(unknown callee) 0xa8b38` | `sp-2104` | `0x29a6bc` | `?` | `sp-2104` |
| `bl` at 0x8e4bc | `(unknown callee) 0x1f8e0` | `ret(0xa8b38)` | `?` | `?` | `ret(0xa8b38)` |
| `bl` at 0x8e4e0 | `(unknown callee) 0x200a0` | `0x9` | `0x10a2d8` | `0x19b4` | `0x0` |
| `bl` at 0x8e52c | `(unknown callee) 0xaadf4` | `[sp-2104]` | `0x1` | `?` | `?` |
| `bl` at 0x8e554 | `(unknown callee) 0x8d178` | `?` | `sp-2092` | `[sp-2104]` | `sp-2092` |
| `bl` at 0x8e57c | `(unknown callee) 0x5eac8` | `sp-32` | `sp-2088` | `ret(0x8d178)` | `sp-2088` |
| `bl` at 0x8e5dc | `(unknown callee) 0x19538` | `?` | `?` | `?` | `?` |
| `bl` at 0x8e618 | `(unknown callee) 0x5ed50` | `sp-32` | `sp-2088` | `?` | `ret(0x19538)` |
| `bl` at 0x8e630 | `(unknown callee) 0x1970c` | `[sp-2092]` | `?` | `?` | `?` |
| `mov` at 0x8e67c | `(unknown callee) (unknown target)` | `0x1` | `[sp-32]` | `sp-2088` | `sp-2088` |
| `mov` at 0x8e6d0 | `(unknown callee) (unknown target)` | `0x2` | `[sp-32]` | `sp-2088` | `sp-2088` |
| `bl` at 0x8e708 | `(unknown callee) 0xaa090` | `0x29a6b4` | `0x0` | `[sp-2104]` | `[sp-32]` |
| `bl` at 0x8e72c | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0x10a508` | `?` | `0x12eeb8` |
| `bl` at 0x8e734 | `(unknown callee) 0x1f8e0` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x8e744 | `(unknown callee) 0x1e958` | `0x10a528` | `ret(0x1f8e0)` | `?` | `ret(0x1f8e0)` |
| `bl` at 0x8e74c | `(unknown callee) 0xa8b98` | `[sp-2104]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x8e4b4 | `ret(0xa8b38) == 0x0` | `ret(0xa8b38) != 0x0` |
| `beq` at 0x8e4f0 | `[[sp-2104]+32]:1 == 0x2` | `[[sp-2104]+32]:1 != 0x2` |
| `bne` at 0x8e510 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8e588 | `[sp-2092] == 0x0` | `[sp-2092] != 0x0` |
| `beq` at 0x8e5ac | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x8e640 | `[0x133ad0] == 0x0` | `[0x133ad0] != 0x0` |
| `beq` at 0x8e654 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x8e694 | `[0x133ad0] == 0x0` | `[0x133ad0] != 0x0` |
| `beq` at 0x8e6a8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x8e71c | `ret(0xaa090) == 0x0` | `ret(0xaa090) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|

## `LpbkIRouter`

- `LpbkIRouter`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x96598 to 0x969fc.
  Established. The condition `[sp-100] == 0x2` takes `0x96824`, while inequality falls through (`beq` at 0x96634).
  An included halfword store through `[sp-92] +0x2` writes `(ret(0x19538)+1 & 0xffff)` (`strh` at 0x96974).
  The unnamed `0xaa424` call carries `0x29a844`, `sp-24`, `0x0`, `0x1` (`bl` at 0x969b4).
  Loopback packet construction remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x965a8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x965ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x965b0 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x965b4 | store | 4 | `sp -0x18` | `[r0+28]` |
| `str` at 0x965bc | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x965c0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x965c4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x965c8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x965d4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x965e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x965e4 | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0x965f8 | load | 1 | `(unknown)` | `-` |
| `str` at 0x96600 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0x96604 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x96610 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x9661c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x9662c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x9663c | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x96658 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x9665c | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x96678 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x9667c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x96694 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x96698 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9669c | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0x966b0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x966c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x966c4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x966d8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x966dc | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x966e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x966e8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x966f0 | store | 4 | `sp -0x48` | `ret(0xaadf4)` |
| `ldr` at 0x966f4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x966f8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x966fc | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0x96700 | load | 4 | `sp -0x40` | `-` |
| `strh` at 0x9671c | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x96724 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x96740 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x96744 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x96760 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x96764 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x9677c | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x96780 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x96784 | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0x96798 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x967a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x967ac | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x967c0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x967c4 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x967cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x967d0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x967d8 | store | 4 | `sp -0x50` | `ret(0xaadf4)` |
| `ldr` at 0x967dc | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x967e0 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x967e4 | store | 4 | `[sp-76] +0x0` | `?` |
| `ldr` at 0x967e8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x96800 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x96804 | load | 4 | `sp -0x40` | `-` |
| `ldrh` at 0x96818 | load | 2 | `(unknown)` | `-` |
| `strh` at 0x9681c | store | 2 | `(unknown)` | `?` |
| `ldr` at 0x96824 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x96840 | store | 1 | `(unknown)` | `0x2` |
| `ldr` at 0x96844 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x96860 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x96864 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x9687c | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x96880 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x96884 | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0x96898 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x968a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x968ac | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x968c0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x968c4 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x968cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x968d0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x968d8 | store | 4 | `sp -0x58` | `ret(0xaadf4)` |
| `ldr` at 0x968dc | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x968e0 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x968e4 | store | 4 | `[sp-84] +0x0` | `?` |
| `ldr` at 0x968e8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x96900 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x96904 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x96908 | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0x9691c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x9692c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x96930 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x96944 | load | 4 | `(unknown)` | `-` |
| `str` at 0x96948 | store | 4 | `sp -0x60` | `?` |
| `ldr` at 0x96950 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x96954 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x9695c | store | 4 | `sp -0x60` | `ret(0xaadf4)` |
| `ldr` at 0x96960 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x96970 | load | 4 | `sp -0x5c` | `-` |
| `strh` at 0x96974 | store | 2 | `[sp-92] +0x2` | `(ret(0x19538)+1 & 0xffff)` |
| `ldr` at 0x96978 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x96980 | store | 4 | `sp -0x40` | `?` |
| `str` at 0x96990 | store | 4 | `sp -0x70` | `0x2` |
| `ldr` at 0x96994 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x96998 | store | 4 | `sp -0x6c` | `?` |
| `str` at 0x969a0 | store | 4 | `sp -0x68` | `sp-60` |
| `ldr` at 0x969a4 | load | 4 | `abs 0x969f4` | `-` |
| `str` at 0x969bc | store | 4 | `sp -0x1c` | `ret(0xaa424)` |
| `ldr` at 0x969c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x969cc | load | 4 | `abs 0x969f8` | `-` |
| `ldr` at 0x969d8 | load | 4 | `abs 0x969f4` | `-` |
| `ldr` at 0x969dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x969e4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x969f0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x969f0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x969f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x966ec | `(unknown callee) 0xaadf4` | `[sp-20]` | `[sp-64]` | `[sp-20]` | `?` |
| `bl` at 0x967d4 | `(unknown callee) 0xaadf4` | `[sp-20]` | `[sp-64]` | `[sp-20]` | `?` |
| `bl` at 0x968d4 | `(unknown callee) 0xaadf4` | `[sp-20]` | `[sp-64]` | `[sp-20]` | `?` |
| `bl` at 0x96958 | `(unknown callee) 0xaadf4` | `[sp-20]` | `[sp-64]` | `[sp-20]` | `?` |
| `bl` at 0x96964 | `(unknown callee) 0x19538` | `?` | `?` | `?` | `?` |
| `bl` at 0x969b4 | `(unknown callee) 0xaa424` | `0x29a844` | `sp-24` | `0x0` | `0x1` |
| `bl` at 0x969d0 | `(unknown callee) 0x1e4ac` | `0x110b08` | `?` | `?` | `ret(0xaa424)` |
| `bl` at 0x969e0 | `(unknown callee) 0xa9c40` | `0x29a844` | `ret(0xaa424)` | `?` | `ret(0xaa424)` |
| `bl` at 0x969e8 | `(unknown callee) 0xa8b98` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x965d0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bgt` at 0x965dc | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x9660c | `[sp-100] == 0x1` | `[sp-100] != 0x1` |
| `bgt` at 0x96618 | `[sp-100] > 0x1 (signed, 32-bit)` | `[sp-100] <= 0x1 (signed, 32-bit)` |
| `beq` at 0x96624 | `[sp-100] == 0x0` | `[sp-100] != 0x0` |
| `beq` at 0x96634 | `[sp-100] == 0x2` | `[sp-100] != 0x2` |
| `bne` at 0x966bc | `? != 0x0` | `? == 0x0` |
| `bne` at 0x967a4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x968a4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x96928 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x969c8 | `ret(0xaa424) != 0x0` | `ret(0xaa424) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x969f0 | `r0` | `ret(0xa8b98)` |

## `EvtIRouter`

- `EvtIRouter`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5e8c8 to 0x5e9c0.
  Established. The word through entry `r0 +0x1c` is read (`ldr` at 0x5e8e0).
  The condition `[r0+22]:1 != 0x0` takes `0x5e9b0`, while zero falls through (`bne` at 0x5e954).
  An included byte store writes `0x2` through entry `r0 +0x16` (`strb` at 0x5e974).
  Event dispatch semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5e8d8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x5e8dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5e8e0 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x5e8e4 | store | 4 | `sp -0x18` | `[r0+28]` |
| `ldr` at 0x5e8e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5e8f4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5e8fc | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x5e900 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x5e908 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x5e910 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5e918 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x5e91c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x5e920 | store | 4 | `sp -0x1c` | `[sp-40]` |
| `ldr` at 0x5e924 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5e934 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x5e938 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5e944 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x5e948 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x5e94c | load | 1 | `r0 +0x16` | `-` |
| `ldr` at 0x5e958 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x5e95c | load | 2 | `r0 +0x22` | `-` |
| `ldr` at 0x5e96c | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x5e974 | store | 1 | `r0 +0x16` | `0x2` |
| `ldr` at 0x5e978 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5e97c | store | 4 | `sp -0x2c` | `[sp-28]` |
| `ldr` at 0x5e980 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5e98c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5e99c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5e9a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5e9b0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x5e9bc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x5e9bc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x5e9bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5e9a0 | `(unknown callee) 0x681e8` | `[sp-20]` | `r1` | `0x2` | `[sp-28]` |
| `bl` at 0x5e9ac | `(unknown callee) 0x7a568` | `[sp-20]` | `r1` | `0x2` | `[sp-28]` |
| `bl` at 0x5e9b4 | `(unknown callee) 0xa8b98` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x5e8f0 | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `bne` at 0x5e954 | `[r0+22]:1 != 0x0` | `[r0+22]:1 == 0x0` |
| `beq` at 0x5e964 | `[r0+34]:2 == 0x0` | `[r0+34]:2 != 0x0` |
| `beq` at 0x5e988 | `[sp-28] == 0x4` | `[sp-28] != 0x4` |
| `beq` at 0x5e994 | `[sp-28] == 0xe` | `[sp-28] != 0xe` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x5e9bc | `r0` | `ret(0xa8b98)` |

## `IRouterWritePacket`

- `IRouterWritePacket`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaa95c to 0xaab28.
  Established. A nonzero `ret(0x1943c)` takes `0xaaa18`, while zero falls through (`bne` at 0xaa9d4).
  The included indirect call carries `[sp-28]`, `[sp-52]+16`, `[sp-40]`, `[sp-52]+16` (`mov` at 0xaaa58).
  An included stack word at `sp -0x3c` receives `0xfffffd92` (`str` at 0xaaacc).
  Packet write error handling remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaa96c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xaa970 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xaa974 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xaa978 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xaa97c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xaa980 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0xaa984 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xaa990 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xaa994 | load | 2 | `(unknown)` | `-` |
| `str` at 0xaa998 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xaa99c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xaa9a0 | load | 1 | `(unknown)` | `-` |
| `str` at 0xaa9b0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xaa9b4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xaa9c8 | store | 4 | `sp -0x34` | `ret(0x1943c)` |
| `ldr` at 0xaa9cc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xaa9d8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xaa9dc | load | 1 | `[sp-32] +0x16` | `-` |
| `str` at 0xaa9ec | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xaa9f0 | load | 4 | `abs 0xaab1c` | `-` |
| `ldr` at 0xaa9f4 | load | 4 | `abs 0xaab20` | `-` |
| `ldr` at 0xaa9f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xaaa04 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xaaa0c | load | 4 | `abs 0xaab20` | `-` |
| `str` at 0xaaa10 | store | 4 | `sp -0x38` | `0xfffffda4` |
| `ldr` at 0xaaa18 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaaa20 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xaaa24 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xaaa30 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xaaa3c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xaaa44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaaa48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaaa50 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xaaa60 | store | 4 | `sp -0x30` | `ret(unknown)` |
| `ldr` at 0xaaa64 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xaaa68 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaaa74 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xaaa80 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xaaa84 | load | 1 | `[sp-32] +0x16` | `-` |
| `str` at 0xaaa94 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xaaa98 | load | 4 | `abs 0xaab1c` | `-` |
| `ldr` at 0xaaaa0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xaaaac | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xaaab4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xaaabc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xaaac8 | load | 4 | `abs 0xaab24` | `-` |
| `str` at 0xaaacc | store | 4 | `sp -0x3c` | `0xfffffd92` |
| `str` at 0xaaad8 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xaaadc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xaaae0 | store | 4 | `sp -0x38` | `[sp-60]` |
| `ldr` at 0xaaae8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xaaaf0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xaaaf4 | load | 4 | `[sp-36] +0x0` | `-` |
| `str` at 0xaaaf8 | store | 4 | `sp -0x24` | `[[sp-36]]` |
| `ldr` at 0xaab00 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xaab0c | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xaab10 | load | 4 | `sp -0x38` | `-` |
| `ldm` at 0xaab18 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xaab18 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xaab18 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaa9c0 | `(unknown callee) 0x1943c` | `?` | `?` | `?` | `?` |
| `bl` at 0xaaa00 | `(unknown callee) 0xaa090` | `0x29aaac` | `0xfffffda4` | `[sp-32]` | `0x0` |
| `bl` at 0xaaa08 | `(unknown callee) 0xa8b98` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xaaa2c | `(unknown callee) 0x19610` | `ret(0x1943c)` | `[sp-36]` | `[sp-40]+16` | `[sp-40]+16` |
| `bl` at 0xaaa38 | `(unknown callee) 0xaab28` | `[sp-52]` | `0x1` | `?` | `?` |
| `mov` at 0xaaa58 | `(unknown callee) (unknown target)` | `[sp-28]` | `[sp-52]+16` | `[sp-40]` | `[sp-52]+16` |
| `bl` at 0xaaaa8 | `(unknown callee) 0xaa090` | `0x29aaac` | `?` | `[sp-32]` | `0x0` |
| `bl` at 0xaaab0 | `(unknown callee) 0x1970c` | `[sp-52]` | `?` | `?` | `?` |
| `bl` at 0xaaab8 | `(unknown callee) 0xa8b98` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xaaaec | `(unknown callee) 0x1970c` | `[sp-52]` | `?` | `ret(unknown)` | `[sp-40]` |
| `bl` at 0xaab04 | `(unknown callee) 0xa8b98` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xaa98c | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xaa9d4 | `ret(0x1943c) != 0x0` | `ret(0x1943c) == 0x0` |
| `bne` at 0xaa9e4 | `[[sp-32]+22]:1 != 0x0` | `[[sp-32]+22]:1 == 0x0` |
| `beq` at 0xaaa70 | `ret(unknown) == [sp-40]` | `ret(unknown) != [sp-40]` |
| `bge` at 0xaaa7c | `ret(unknown) >= 0x0 (signed, 32-bit)` | `ret(unknown) < 0x0 (signed, 32-bit)` |
| `bne` at 0xaaa8c | `[[sp-32]+22]:1 != 0x0` | `[[sp-32]+22]:1 == 0x0` |
| `bne` at 0xaaac4 | `[sp-48] != 0x0` | `[sp-48] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xaab18 | `r0` | `?` |

## `IRouterUnregister`

- `IRouterUnregister`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa7b24 to 0xa7e88.
  Established. The absolute word at `0x29ae18` receives `[[sp-36]+40]` on an included path (`str` at 0xa7cdc).
  A separate included word assignment writes `[[sp-48]+24]` to `0x29aaa8` (`str` at 0xa7db4).
  A zero `ret(0x2105c)` takes `0xa7e4c`, while nonzero falls through (`beq` at 0xa7e1c).
  Registration removal policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa7b34 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa7b38 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa7b3c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xa7b40 | load | 4 | `abs 0xa7e5c` | `-` |
| `ldr` at 0xa7b44 | load | 4 | `abs 0x29ae18` | `-` |
| `str` at 0xa7b48 | store | 4 | `sp -0x24` | `[0x29ae18]` |
| `str` at 0xa7b50 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xa7b58 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xa7b5c | load | 4 | `abs 0xa7e60` | `-` |
| `ldr` at 0xa7b60 | load | 4 | `abs 0x29aaa8` | `-` |
| `str` at 0xa7b64 | store | 4 | `sp -0x30` | `[0x29aaa8]` |
| `str` at 0xa7b6c | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0xa7b74 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xa7b78 | load | 4 | `abs 0xa7e64` | `-` |
| `str` at 0xa7b88 | store | 4 | `sp -0x3c` | `ret(0x20cd0)` |
| `ldr` at 0xa7b8c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa7b98 | load | 4 | `abs 0xa7e68` | `-` |
| `str` at 0xa7b9c | store | 4 | `sp -0x48` | `0x113e78` |
| `ldr` at 0xa7ba0 | load | 4 | `abs 0xa7e64` | `-` |
| `str` at 0xa7ba4 | store | 4 | `sp -0x44` | `0x133c24` |
| `ldr` at 0xa7ba8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xa7bac | store | 4 | `sp -0x40` | `ret(0x20cd0)` |
| `ldr` at 0xa7bb4 | load | 4 | `abs 0xa7e6c` | `-` |
| `ldr` at 0xa7bb8 | load | 4 | `abs 0xa7e70` | `-` |
| `ldr` at 0xa7bc4 | load | 4 | `abs 0xa7e74` | `-` |
| `ldr` at 0xa7bc8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7bcc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7bd0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa7bd8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa7bec | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xa7bf0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa7bfc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7c08 | load | 4 | `abs 0xa7e78` | `-` |
| `ldr` at 0xa7c0c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa7c24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7c28 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa7c2c | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xa7c38 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa7c4c | load | 4 | `abs 0xa7e78` | `-` |
| `ldr` at 0xa7c64 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa7c6c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xa7c74 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa7c80 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa7c84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7c88 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xa7c8c | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xa7c98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7ca4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa7ca8 | load | 4 | `[sp-36] +0x4` | `-` |
| `ldr` at 0xa7cac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7cbc | load | 4 | `abs 0xa7e5c` | `-` |
| `ldr` at 0xa7cc0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa7cc4 | load | 4 | `abs 0x29ae18` | `-` |
| `ldr` at 0xa7cd0 | load | 4 | `abs 0xa7e5c` | `-` |
| `ldr` at 0xa7cd4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa7cd8 | load | 4 | `[sp-36] +0x28` | `-` |
| `str` at 0xa7cdc | store | 4 | `abs 0x29ae18` | `[[sp-36]+40]` |
| `ldr` at 0xa7ce4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa7ce8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa7cec | load | 4 | `[sp-36] +0x28` | `-` |
| `str` at 0xa7cf0 | store | 4 | `[sp-40] +0x28` | `[[sp-36]+40]` |
| `ldr` at 0xa7cf4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa7cf8 | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0xa7cfc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa7d00 | load | 4 | `[sp-36] +0x28` | `-` |
| `str` at 0xa7d04 | store | 4 | `sp -0x24` | `[[sp-36]+40]` |
| `ldr` at 0xa7d08 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa7d14 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa7d18 | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0xa7d1c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa7d20 | load | 4 | `[sp-36] +0x28` | `-` |
| `str` at 0xa7d24 | store | 4 | `sp -0x24` | `[[sp-36]+40]` |
| `ldr` at 0xa7d2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7d38 | load | 4 | `abs 0xa7e7c` | `-` |
| `ldr` at 0xa7d3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7d40 | load | 4 | `abs 0x29aa84` | `-` |
| `ldr` at 0xa7d44 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xa7d50 | load | 4 | `abs 0xa7e7c` | `-` |
| `ldr` at 0xa7d60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa7d70 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa7d7c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa7d80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7d84 | load | 4 | `[sp-48] +0x0` | `-` |
| `ldr` at 0xa7d88 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xa7d94 | load | 4 | `abs 0xa7e60` | `-` |
| `ldr` at 0xa7d98 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa7d9c | load | 4 | `abs 0x29aaa8` | `-` |
| `ldr` at 0xa7da8 | load | 4 | `abs 0xa7e60` | `-` |
| `ldr` at 0xa7dac | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa7db0 | load | 4 | `[sp-48] +0x18` | `-` |
| `str` at 0xa7db4 | store | 4 | `abs 0x29aaa8` | `[[sp-48]+24]` |
| `ldr` at 0xa7dbc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa7dc0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa7dc4 | load | 4 | `[sp-48] +0x18` | `-` |
| `str` at 0xa7dc8 | store | 4 | `[sp-52] +0x18` | `[[sp-48]+24]` |
| `ldr` at 0xa7dcc | load | 4 | `sp -0x30` | `-` |
| `str` at 0xa7dd0 | store | 4 | `sp -0x38` | `[sp-48]` |
| `ldr` at 0xa7dd4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa7dd8 | load | 4 | `[sp-48] +0x18` | `-` |
| `str` at 0xa7ddc | store | 4 | `sp -0x30` | `[[sp-48]+24]` |
| `ldr` at 0xa7de0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa7dec | load | 4 | `sp -0x30` | `-` |
| `str` at 0xa7df0 | store | 4 | `sp -0x34` | `[sp-48]` |
| `ldr` at 0xa7df4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa7df8 | load | 4 | `[sp-48] +0x18` | `-` |
| `str` at 0xa7dfc | store | 4 | `sp -0x30` | `[[sp-48]+24]` |
| `ldr` at 0xa7e04 | load | 4 | `abs 0xa7e64` | `-` |
| `str` at 0xa7e10 | store | 4 | `sp -0x3c` | `ret(0x2105c)` |
| `ldr` at 0xa7e14 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa7e20 | load | 4 | `abs 0xa7e80` | `-` |
| `str` at 0xa7e24 | store | 4 | `sp -0x48` | `0x113ed0` |
| `ldr` at 0xa7e28 | load | 4 | `abs 0xa7e64` | `-` |
| `str` at 0xa7e2c | store | 4 | `sp -0x44` | `0x133c24` |
| `ldr` at 0xa7e30 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xa7e34 | store | 4 | `sp -0x40` | `ret(0x2105c)` |
| `ldr` at 0xa7e3c | load | 4 | `abs 0xa7e6c` | `-` |
| `ldr` at 0xa7e40 | load | 4 | `abs 0xa7e84` | `-` |
| `ldm` at 0xa7e58 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa7e58 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa7e58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa7b80 | `(unknown callee) 0x20cd0` | `0x133c24` | `?` | `r2` | `0x0` |
| `bl` at 0xa7bc0 | `(unknown callee) 0x200a0` | `0x9` | `0x113ea4` | `0x167` | `0x0` |
| `bl` at 0xa7bd4 | `(unknown callee) 0x1e958` | `0x113f00` | `[sp-20]` | `[sp-24]` | `[sp-28]` |
| `bl` at 0xa7c60 | `(unknown callee) 0x19748` | `?` | `0x0` | `0x24` | `?` |
| `bl` at 0xa7d0c | `(unknown callee) 0x1970c` | `[sp-36]` | `[[sp-36]]` | `[[sp-36]+40]` | `[[sp-36]+40]` |
| `bl` at 0xa7d5c | `(unknown callee) 0x19748` | `0x29aa84` | `0x0` | `0x24` | `[[sp-20]]` |
| `bl` at 0xa7de4 | `(unknown callee) 0x1970c` | `[sp-48]` | `[[sp-48]]` | `[[sp-48]+24]` | `[[sp-48]+24]` |
| `bl` at 0xa7e08 | `(unknown callee) 0x2105c` | `0x133c24` | `?` | `?` | `?` |
| `bl` at 0xa7e48 | `(unknown callee) 0x200a0` | `0x9` | `0x113ea4` | `0x1a6` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa7b94 | `ret(0x20cd0) == 0x0` | `ret(0x20cd0) != 0x0` |
| `beq` at 0xa7be4 | `? == 0x0` | `? != 0x0` |
| `bgt` at 0xa7bf8 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0xa7c04 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bne` at 0xa7c34 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xa7c7c | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `bne` at 0xa7c94 | `[[sp-36]] != [[sp-20]]` | `[[sp-36]] == [[sp-20]]` |
| `beq` at 0xa7ca0 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0xa7cb4 | `[[sp-36]+4] == [sp-24]` | `[[sp-36]+4] != [sp-24]` |
| `bne` at 0xa7ccc | `[sp-36] != [0x29ae18]` | `[sp-36] == [0x29ae18]` |
| `bne` at 0xa7d34 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bne` at 0xa7d4c | `[0x29aa84] != [[sp-20]]` | `[0x29aa84] == [[sp-20]]` |
| `beq` at 0xa7d6c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa7d78 | `[sp-48] == 0x0` | `[sp-48] != 0x0` |
| `bne` at 0xa7d90 | `[[sp-48]] != [[sp-20]]` | `[[sp-48]] == [[sp-20]]` |
| `bne` at 0xa7da4 | `[sp-48] != [0x29aaa8]` | `[sp-48] == [0x29aaa8]` |
| `beq` at 0xa7e1c | `ret(0x2105c) == 0x0` | `ret(0x2105c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa7e58 | `r0` | `0x0` |

## `IRouterReadPacket`

- `IRouterReadPacket`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaa4f0 to 0xaa95c.
  Established. The unnamed `0x1943c` call carries `0x410`, unknown, unknown, `0x400` (`bl` at 0xaa540).
  A positive signed `ret(unknown)` takes `0xaa6e8`, while nonpositive values fall through (`bgt` at 0xaa5cc).
  An included word store through `[sp-32] +0x0` writes `[sp-40]` (`str` at 0xaa90c).
  Frame reassembly and read error semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaa500 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xaa504 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xaa508 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xaa50c | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xaa514 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xaa51c | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0xaa524 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0xaa52c | store | 4 | `sp -0x3c` | `0x410` |
| `ldr` at 0xaa530 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xaa538 | store | 4 | `sp -0x40` | `0x400` |
| `ldr` at 0xaa53c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xaa548 | store | 4 | `sp -0x2c` | `ret(0x1943c)` |
| `ldr` at 0xaa54c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xaa558 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaa564 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xaa568 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xaa56c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaa570 | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0xaa574 | store | 4 | `sp -0x28` | `[[sp-40]]` |
| `ldr` at 0xaa578 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xaa584 | load | 4 | `abs 0xaa92c` | `-` |
| `str` at 0xaa588 | store | 4 | `sp -0x60` | `0xfffffda4` |
| `ldr` at 0xaa590 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xaa5a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaa5a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaa5b0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xaa5c0 | store | 4 | `sp -0x24` | `ret(unknown)` |
| `ldr` at 0xaa5c4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xaa5d0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xaa5d4 | store | 4 | `sp -0x64` | `ret(unknown)` |
| `ldr` at 0xaa5d8 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xaa5e4 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xaa5f8 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xaa600 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xaa604 | store | 4 | `sp -0x44` | `[sp-36]` |
| `ldr` at 0xaa60c | load | 4 | `abs 0xaa930` | `-` |
| `str` at 0xaa610 | store | 4 | `sp -0x44` | `0xfffffd9c` |
| `ldr` at 0xaa614 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaa620 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaa624 | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0xaa628 | store | 4 | `sp -0x48` | `[[sp-40]]` |
| `ldr` at 0xaa62c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xaa638 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xaa63c | load | 4 | `(unknown)` | `-` |
| `str` at 0xaa640 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xaa644 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xaa64c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xaa650 | store | 4 | `sp -0x48` | `[sp-76]` |
| `ldr` at 0xaa658 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xaa660 | store | 4 | `[sp-40] +0x0` | `0x0` |
| `ldr` at 0xaa664 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaa670 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xaa674 | load | 1 | `[sp-40] +0x16` | `-` |
| `str` at 0xaa684 | store | 4 | `sp -0x70` | `0x0` |
| `ldr` at 0xaa688 | load | 4 | `abs 0xaa934` | `-` |
| `ldr` at 0xaa68c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xaa690 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaa69c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xaa6a8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xaa6b0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaa6bc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xaa6c0 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xaa6c4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaa6c8 | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0xaa6cc | store | 4 | `sp -0x28` | `[[sp-40]]` |
| `ldr` at 0xaa6d0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xaa6dc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xaa6e0 | store | 4 | `sp -0x60` | `?` |
| `ldr` at 0xaa6e8 | load | 4 | `abs 0xaa938` | `-` |
| `ldr` at 0xaa6ec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xaa6f0 | load | 4 | `abs 0x133c00` | `-` |
| `ldr` at 0xaa6f8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaa704 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xaa708 | load | 1 | `[sp-44] +0x15` | `-` |
| `ldr` at 0xaa71c | load | 4 | `abs 0xaa93c` | `-` |
| `ldr` at 0xaa720 | load | 4 | `abs 0x133c08` | `-` |
| `ldr` at 0xaa738 | load | 4 | `abs 0xaa940` | `-` |
| `ldr` at 0xaa73c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xaa744 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xaa750 | store | 4 | `sp -0x60` | `?` |
| `ldr` at 0xaa758 | load | 4 | `abs 0xaa944` | `-` |
| `ldr` at 0xaa75c | load | 4 | `abs 0x133c04` | `-` |
| `ldr` at 0xaa768 | load | 4 | `abs 0xaa948` | `-` |
| `ldr` at 0xaa76c | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0xaa770 | load | 4 | `abs 0xaa94c` | `-` |
| `ldr` at 0xaa774 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xaa77c | load | 4 | `abs 0xaa948` | `-` |
| `ldr` at 0xaa780 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0xaa788 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xaa78c | store | 4 | `sp -0x28` | `[sp-44]` |
| `ldr` at 0xaa794 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xaa798 | load | 1 | `[sp-44] +0x15` | `-` |
| `ldr` at 0xaa7a0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xaa7b0 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0xaa7b4 | load | 4 | `abs 0xaa93c` | `-` |
| `ldr` at 0xaa7b8 | load | 4 | `abs 0x133c08` | `-` |
| `ldr` at 0xaa7d0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaa7dc | load | 4 | `abs 0xaa93c` | `-` |
| `ldr` at 0xaa7e0 | load | 4 | `abs 0x133c08` | `-` |
| `ldr` at 0xaa7f4 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xaa800 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xaa808 | load | 4 | `abs 0xaa950` | `-` |
| `ldr` at 0xaa81c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaa824 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xaa828 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xaa82c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaa830 | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0xaa834 | store | 4 | `sp -0x28` | `[[sp-40]]` |
| `ldr` at 0xaa838 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xaa844 | load | 4 | `abs 0xaa93c` | `-` |
| `ldr` at 0xaa848 | load | 4 | `abs 0x133c08` | `-` |
| `ldr` at 0xaa854 | load | 4 | `abs 0xaa954` | `-` |
| `ldr` at 0xaa858 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xaa860 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xaa868 | load | 4 | `abs 0xaa958` | `-` |
| `str` at 0xaa86c | store | 4 | `sp -0x60` | `0xfffffd9a` |
| `ldr` at 0xaa874 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xaa878 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xaa87c | store | 4 | `[sp-48] +0x0` | `[sp-44]` |
| `ldr` at 0xaa880 | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0xaa884 | load | 2 | `[sp-44] +0x4` | `-` |
| `strh` at 0xaa888 | store | 2 | `sp -0x68` | `([[sp-44]+4]:2 & 0xffff)` |
| `ldrh` at 0xaa88c | load | 2 | `sp -0x68` | `-` |
| `strh` at 0xaa898 | store | 2 | `sp -0x68` | `?` |
| `ldrh` at 0xaa89c | load | 2 | `sp -0x68` | `-` |
| `strh` at 0xaa8a0 | store | 2 | `[sp-44] +0x4` | `([sp-104]:2 & 0xffff)` |
| `ldr` at 0xaa8a4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xaa8ac | store | 4 | `[sp-44] +0x0` | `0x0` |
| `ldr` at 0xaa8b0 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xaa8b4 | load | 1 | `[sp-44] +0x15` | `-` |
| `str` at 0xaa8c0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xaa8c4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xaa8c8 | store | 4 | `sp -0x30` | `[sp-44]` |
| `ldr` at 0xaa8cc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xaa8dc | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0xaa8e0 | load | 2 | `[sp-40] +0x4` | `-` |
| `strh` at 0xaa8e4 | store | 2 | `sp -0x6c` | `([[sp-40]+4]:2 & 0xffff)` |
| `ldrh` at 0xaa8e8 | load | 2 | `sp -0x6c` | `-` |
| `strh` at 0xaa8f8 | store | 2 | `sp -0x6c` | `?` |
| `ldrh` at 0xaa8fc | load | 2 | `sp -0x6c` | `-` |
| `strh` at 0xaa900 | store | 2 | `[sp-40] +0x4` | `([sp-108]:2 & 0xffff)` |
| `ldr` at 0xaa904 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xaa908 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xaa90c | store | 4 | `[sp-32] +0x0` | `[sp-40]` |
| `str` at 0xaa91c | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xaa920 | load | 4 | `sp -0x60` | `-` |
| `ldm` at 0xaa928 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xaa928 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xaa928 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaa540 | `(unknown callee) 0x1943c` | `0x410` | `?` | `?` | `0x400` |
| `bl` at 0xaa57c | `(unknown callee) 0x1970c` | `[sp-40]` | `?` | `?` | `[[sp-40]]` |
| `bl` at 0xaa59c | `(unknown callee) 0x19748` | `ret(0x1943c)` | `0x0` | `0x2c` | `ret(0x1943c)` |
| `mov` at 0xaa5b8 | `(unknown callee) (unknown target)` | `[sp-28]` | `sp-44` | `[sp-64]` | `sp-44` |
| `bl` at 0xaa648 | `(unknown callee) 0x1970c` | `?` | `?` | `?` | `?` |
| `bl` at 0xaa698 | `(unknown callee) 0xaa090` | `0x29aaac` | `?` | `[sp-40]` | `0x0` |
| `bl` at 0xaa6ac | `(unknown callee) 0x1970c` | `[sp-44]` | `?` | `?` | `[sp-44]` |
| `bl` at 0xaa6d4 | `(unknown callee) 0x1970c` | `[sp-40]` | `?` | `?` | `[[sp-40]]` |
| `bl` at 0xaa6f4 | `(unknown callee) 0xaab28` | `[sp-44]` | `[0x133c00]` | `?` | `0x133c00` |
| `bl` at 0xaa734 | `(unknown callee) 0xab5e4` | `0x0` | `0x0` | `?` | `[0x133c08]` |
| `bl` at 0xaa740 | `(unknown callee) 0xab5e4` | `0x114070` | `[sp-44]` | `?` | `?` |
| `bl` at 0xaa748 | `(unknown callee) 0x1970c` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xaa778 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0x114078` | `[sp-44]` | `0x12eeb8` |
| `bl` at 0xaa784 | `(unknown callee) 0x19334` | `[0x12eeb8]` | `?` | `?` | `0x12eeb8` |
| `bl` at 0xaa7cc | `(unknown callee) 0xab5e4` | `0x0` | `0x0` | `?` | `[0x133c08]` |
| `bl` at 0xaa810 | `(unknown callee) 0x197a8` | `sp-92` | `0x114084` | `?` | `?` |
| `bl` at 0xaa820 | `(unknown callee) 0xab5e4` | `sp-92` | `[sp-40]` | `?` | `sp-92` |
| `bl` at 0xaa83c | `(unknown callee) 0x1970c` | `[sp-40]` | `?` | `?` | `[[sp-40]]` |
| `bl` at 0xaa85c | `(unknown callee) 0xab5e4` | `0x11408c` | `[sp-44]` | `?` | `[0x133c08]` |
| `bl` at 0xaa864 | `(unknown callee) 0x1970c` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xaa914 | `(unknown callee) 0xa7e88` | `0x1` | `([[sp-40]+4]:2 & 0xffff)` | `[sp-32]` | `[sp-40]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xaa554 | `ret(0x1943c) != 0x0` | `ret(0x1943c) == 0x0` |
| `beq` at 0xaa560 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `bgt` at 0xaa5cc | `ret(unknown) > 0x0 (signed, 32-bit)` | `ret(unknown) <= 0x0 (signed, 32-bit)` |
| `beq` at 0xaa5e0 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xaa5ec | `[sp-100] == 0x0` | `[sp-100] != 0x0` |
| `beq` at 0xaa61c | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `beq` at 0xaa634 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xaa66c | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `bne` at 0xaa67c | `[[sp-40]+22]:1 != 0x0` | `[[sp-40]+22]:1 == 0x0` |
| `beq` at 0xaa6a4 | `[sp-44] == 0x0` | `[sp-44] != 0x0` |
| `beq` at 0xaa6b8 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `bne` at 0xaa700 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0xaa718 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xaa728 | `[0x133c08] == 0x0` | `[0x133c08] != 0x0` |
| `beq` at 0xaa764 | `[0x133c04] == 0x0` | `[0x133c04] != 0x0` |
| `beq` at 0xaa7a8 | `? == [sp-56]` | `? != [sp-56]` |
| `beq` at 0xaa7c0 | `[0x133c08] == 0x0` | `[0x133c08] != 0x0` |
| `beq` at 0xaa7d8 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `beq` at 0xaa7e8 | `[0x133c08] == 0x0` | `[0x133c08] != 0x0` |
| `beq` at 0xaa850 | `[0x133c08] == 0x0` | `[0x133c08] != 0x0` |
| `beq` at 0xaa8d4 | `[sp-56] == 0x0` | `[sp-56] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xaa928 | `r0` | `?` |

## `L1CfgDiscoveryResponse`

- `L1CfgDiscoveryResponse`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x61b04 to 0x62048.
  Established. The word through entry `r0 +0xc` is read (`ldr` at 0x61b1c).
  An included word store through `[sp-36] +0x90` writes `[sp-40]` (`str` at 0x61b94).
  The unnamed `0xaadf4` call at `0x61d94` carries `[sp-24]`, `0x5`, unknown, unknown (`bl` at 0x61d94).
  Discovery response record updates remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x61b14 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x61b18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x61b1c | load | 4 | `r0 +0xc` | `-` |
| `str` at 0x61b20 | store | 4 | `sp -0x1c` | `[r0+12]` |
| `ldr` at 0x61b24 | load | 4 | `abs 0x62024` | `-` |
| `ldr` at 0x61b2c | load | 4 | `abs 0x62028` | `-` |
| `ldr` at 0x61b30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61b4c | load | 4 | `abs 0x132ad8` | `-` |
| `str` at 0x61b54 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x61b58 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x61b5c | load | 1 | `[sp-24] +0x44` | `-` |
| `ldr` at 0x61b6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x61b70 | load | 4 | `[sp-24] +0x48` | `-` |
| `str` at 0x61b74 | store | 4 | `sp -0x28` | `[[sp-24]+72]` |
| `ldr` at 0x61b7c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x61b88 | store | 4 | `sp -0x28` | `ret(0xaadf4)` |
| `ldr` at 0x61b8c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x61b90 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x61b94 | store | 4 | `[sp-36] +0x90` | `[sp-40]` |
| `ldr` at 0x61b98 | load | 4 | `abs 0x6202c` | `-` |
| `ldr` at 0x61b9c | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x61bb4 | load | 4 | `abs 0x62030` | `-` |
| `ldr` at 0x61bb8 | load | 4 | `abs 0x62028` | `-` |
| `ldr` at 0x61bbc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61bd8 | load | 4 | `abs 0x132ad8` | `-` |
| `str` at 0x61be0 | store | 4 | `sp -0x34` | `ret(0x194f0)` |
| `ldr` at 0x61be4 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x61be8 | load | 4 | `abs 0x62034` | `-` |
| `ldr` at 0x61bec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61bf0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x61bf8 | load | 4 | `abs 0x62028` | `-` |
| `ldr` at 0x61bfc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61c18 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x61c20 | load | 4 | `abs 0x62028` | `-` |
| `ldr` at 0x61c24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61c40 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x61c48 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x61c4c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x61c58 | load | 4 | `abs 0x62028` | `-` |
| `ldr` at 0x61c5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61c78 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x61c80 | load | 4 | `abs 0x62028` | `-` |
| `ldr` at 0x61c84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61ca0 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x61ca8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x61cc4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x61cc8 | load | 4 | `abs 0x62028` | `-` |
| `ldr` at 0x61ccc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61ce8 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x61cf0 | load | 4 | `abs 0x62028` | `-` |
| `ldr` at 0x61cf4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61d10 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x61d18 | load | 4 | `abs 0x62028` | `-` |
| `ldr` at 0x61d1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61d38 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x61d40 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x61d54 | load | 4 | `(unknown)` | `-` |
| `str` at 0x61d64 | store | 4 | `(unknown)` | `ret(0x195d4)` |
| `ldr` at 0x61d68 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x61d6c | load | 1 | `[sp-24] +0x4c` | `-` |
| `ldr` at 0x61d7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x61d80 | load | 4 | `[sp-24] +0x50` | `-` |
| `str` at 0x61d84 | store | 4 | `sp -0x2c` | `[[sp-24]+80]` |
| `ldr` at 0x61d8c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x61d98 | store | 4 | `sp -0x2c` | `ret(0xaadf4)` |
| `ldr` at 0x61d9c | load | 4 | `abs 0x62028` | `-` |
| `ldr` at 0x61da0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61dbc | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x61dc4 | load | 4 | `abs 0x62028` | `-` |
| `ldr` at 0x61dc8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61de4 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x61dec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x61e00 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x61e04 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x61e10 | load | 4 | `abs 0x6202c` | `-` |
| `ldr` at 0x61e14 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x61e24 | load | 4 | `abs 0x62038` | `-` |
| `ldr` at 0x61e28 | load | 4 | `abs 0x12d788` | `-` |
| `ldr` at 0x61e2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61e40 | load | 4 | `abs 0x62030` | `-` |
| `ldr` at 0x61e44 | load | 4 | `abs 0x62028` | `-` |
| `ldr` at 0x61e48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61e64 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x61e6c | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x61e70 | load | 4 | `abs 0x6203c` | `-` |
| `ldr` at 0x61e74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61e78 | load | 4 | `(unknown)` | `-` |
| `str` at 0x61e84 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x61e88 | load | 4 | `abs 0x62028` | `-` |
| `ldr` at 0x61e8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61ea8 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x61eb0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x61eb4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x61ec0 | load | 4 | `abs 0x62030` | `-` |
| `ldr` at 0x61ec4 | load | 4 | `abs 0x62028` | `-` |
| `ldr` at 0x61ec8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61ee4 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x61eec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x61f00 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x61f08 | load | 4 | `abs 0x62028` | `-` |
| `ldr` at 0x61f0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61f28 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x61f30 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x61f44 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x61f4c | load | 4 | `abs 0x62028` | `-` |
| `ldr` at 0x61f50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61f6c | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x61f74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x61f88 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x61f90 | load | 4 | `(unknown)` | `-` |
| `str` at 0x61f94 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x61f98 | load | 4 | `abs 0x62028` | `-` |
| `ldr` at 0x61f9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61fb8 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x61fc0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x61fd4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x61fdc | load | 4 | `(unknown)` | `-` |
| `str` at 0x61fe0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x61fe4 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x61fe8 | load | 4 | `abs 0x62040` | `-` |
| `ldr` at 0x61fec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x61ff0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x61ff8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x62000 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0x62008 | load | 4 | `abs 0x62030` | `-` |
| `ldr` at 0x6200c | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x62010 | load | 4 | `abs 0x62044` | `-` |
| `ldm` at 0x62020 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x62020 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x62020 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x62020 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x61b28 | `(unknown callee) 0x619dc` | `0xfbe14` | `r1` | `r2` | `[r0+12]` |
| `bl` at 0x61b84 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x4` | `?` | `?` |
| `bl` at 0x61bb0 | `(unknown callee) 0x194f0` | `0x0` | `?` | `[sp-36]` | `?` |
| `bl` at 0x61bf4 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfbe2c` | `[sp-28]` | `?` |
| `bl` at 0x61d5c | `(unknown callee) 0x195d4` | `?` | `?` | `?` | `?` |
| `bl` at 0x61d94 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x5` | `?` | `?` |
| `bl` at 0x61e0c | `(unknown callee) 0x19610` | `?` | `[sp-44]` | `?` | `?` |
| `bl` at 0x61e7c | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfbe64` | `[sp-28]` | `?` |
| `bl` at 0x61ff4 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfbe74` | `?` | `?` |
| `bl` at 0x62014 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfbe88` | `?` | `0x12eeb8` |
| `bl` at 0x62018 | `(unknown callee) 0x62e24` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x61b68 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x61ba8 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x61c54 | `? <= ? (signed, 32-bit)` | `? > ? (signed, 32-bit)` |
| `bne` at 0x61d78 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x61e20 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x61e3c | `? == 0x0` | `? != 0x0` |
| `ble` at 0x61ebc | `? <= [sp-32] (signed, 32-bit)` | `? > [sp-32] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x62020 | `r0` | `ret(0x62e24)` |

## `L1CfgProcessRequest`

- `L1CfgProcessRequest`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x678e0 to 0x67f9c.
  Established. Equality `[sp-116] == 0x3e8` takes `0x679d0`, while inequality falls through (`beq` at 0x679c8).
  An included absolute word assignment writes `ret(0x194f0)` to `0x132794` (`str` at 0x679dc).
  The unnamed `0xaa090` call carries `0x132a48`, `0x0`, `[sp-24]`, `[sp-100]` (`bl` at 0x67dd0).
  Configuration request protocol meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x678f0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x678f8 | store | 4 | `sp -0x64` | `0x0` |
| `ldr` at 0x678fc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x67900 | load | 1 | `r0 +0x24` | `-` |
| `ldr` at 0x67910 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x67914 | load | 4 | `r0 +0x28` | `-` |
| `str` at 0x67918 | store | 4 | `sp -0x8c` | `[r0+40]` |
| `ldr` at 0x67920 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6792c | store | 4 | `sp -0x8c` | `ret(0xaadf4)` |
| `ldr` at 0x67930 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x67934 | store | 4 | `sp -0x74` | `[sp-140]` |
| `ldr` at 0x67948 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x67950 | load | 4 | `sp -0x74` | `-` |
| `str` at 0x67954 | store | 4 | `sp -0x98` | `[sp-116]` |
| `ldr` at 0x67958 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x67964 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x67970 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6797c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x67988 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x67998 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x679a8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x679b4 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x679c0 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x679d0 | load | 4 | `abs 0x67f5c` | `-` |
| `str` at 0x679dc | store | 4 | `abs 0x132794` | `ret(0x194f0)` |
| `ldr` at 0x679e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x679ec | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x679f0 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x679fc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x67a00 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x67a10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x67a14 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x67a18 | store | 4 | `sp -0x90` | `[[sp-24]+48]` |
| `ldr` at 0x67a20 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x67a2c | store | 4 | `sp -0x90` | `ret(0xaadf4)` |
| `ldr` at 0x67a30 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x67a34 | store | 4 | `sp -0x78` | `[sp-144]` |
| `ldr` at 0x67a38 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x67a48 | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0x67a4c | load | 4 | `abs 0x67f60` | `-` |
| `ldr` at 0x67a50 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x67a64 | load | 4 | `abs 0x67f60` | `-` |
| `ldr` at 0x67a68 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x67a74 | store | 4 | `sp -0x7c` | `0x1` |
| `ldr` at 0x67a78 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x67a94 | store | 4 | `sp -0x7c` | `0x1` |
| `ldr` at 0x67a98 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x67aac | load | 4 | `abs 0x67f64` | `-` |
| `ldr` at 0x67ab4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x67ab8 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x67ac4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x67ac8 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x67ad8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x67adc | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0x67ae0 | store | 4 | `sp -0x94` | `[[sp-24]+56]` |
| `ldr` at 0x67ae8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x67af4 | store | 4 | `sp -0x94` | `ret(0xaadf4)` |
| `ldr` at 0x67af8 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x67afc | store | 4 | `sp -0x7c` | `[sp-148]` |
| `ldr` at 0x67b00 | load | 4 | `abs 0x67f68` | `-` |
| `ldr` at 0x67b04 | load | 4 | `abs 0x67f6c` | `-` |
| `ldr` at 0x67b08 | load | 4 | `abs 0x132adc` | `-` |
| `ldr` at 0x67b0c | load | 4 | `abs 0x1329e4` | `-` |
| `ldr` at 0x67b18 | load | 4 | `abs 0x67f70` | `-` |
| `ldr` at 0x67b1c | load | 4 | `abs 0x1329e8` | `-` |
| `ldr` at 0x67b28 | load | 4 | `abs 0x67f70` | `-` |
| `ldr` at 0x67b2c | load | 4 | `abs 0x1329e8` | `-` |
| `ldr` at 0x67b34 | load | 4 | `abs 0x67f74` | `-` |
| `ldr` at 0x67b38 | load | 4 | `abs 0x1329ec` | `-` |
| `ldr` at 0x67b44 | load | 4 | `abs 0x67f74` | `-` |
| `ldr` at 0x67b48 | load | 4 | `abs 0x1329ec` | `-` |
| `ldr` at 0x67b50 | load | 4 | `abs 0x67f6c` | `-` |
| `ldr` at 0x67b54 | load | 4 | `abs 0x67f68` | `-` |
| `ldr` at 0x67b58 | load | 4 | `abs 0x132adc` | `-` |
| `str` at 0x67b5c | store | 4 | `abs 0x1329e4` | `[0x132adc]` |
| `ldr` at 0x67b60 | load | 4 | `abs 0x67f70` | `-` |
| `ldr` at 0x67b64 | load | 4 | `abs 0x67f6c` | `-` |
| `ldr` at 0x67b68 | load | 4 | `abs 0x1329e4` | `-` |
| `str` at 0x67b74 | store | 4 | `abs 0x1329e8` | `ret(0x1943c)` |
| `ldr` at 0x67b78 | load | 4 | `abs 0x67f74` | `-` |
| `ldr` at 0x67b7c | load | 4 | `abs 0x67f6c` | `-` |
| `ldr` at 0x67b80 | load | 4 | `abs 0x1329e4` | `-` |
| `str` at 0x67ba0 | store | 4 | `abs 0x1329ec` | `ret(0x1943c)` |
| `ldr` at 0x67bbc | load | 4 | `sp -0x7c` | `-` |
| `str` at 0x67bcc | store | 4 | `sp -0x84` | `0x0` |
| `str` at 0x67bd4 | store | 4 | `sp -0x88` | `0x0` |
| `str` at 0x67bdc | store | 4 | `sp -0x80` | `0x0` |
| `ldr` at 0x67be0 | load | 4 | `abs 0x67f68` | `-` |
| `ldr` at 0x67be4 | load | 4 | `abs 0x132adc` | `-` |
| `ldr` at 0x67be8 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x67bf4 | load | 4 | `abs 0x67f68` | `-` |
| `ldr` at 0x67bf8 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x67c0c | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x67c14 | load | 4 | `abs 0x67f68` | `-` |
| `ldr` at 0x67c18 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x67c2c | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x67c34 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x67c38 | load | 4 | `(unknown)` | `-` |
| `str` at 0x67c44 | store | 4 | `sp -0x78` | `ret(0x69eec)` |
| `ldr` at 0x67c48 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x67c54 | load | 4 | `abs 0x67f70` | `-` |
| `ldr` at 0x67c58 | load | 4 | `abs 0x1329e8` | `-` |
| `ldr` at 0x67c5c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x67c64 | load | 4 | `sp -0x80` | `-` |
| `strb` at 0x67c68 | store | 1 | `([0x1329e8] + [sp-136]) +0x0` | `([sp-128] & 0xff)` |
| `ldr` at 0x67c6c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x67c80 | load | 4 | `abs 0x67f74` | `-` |
| `ldr` at 0x67c84 | load | 4 | `abs 0x1329ec` | `-` |
| `ldr` at 0x67c90 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x67c9c | load | 4 | `sp -0x88` | `-` |
| `str` at 0x67ca4 | store | 4 | `sp -0x88` | `[sp-136]+1` |
| `ldr` at 0x67ca8 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x67cb0 | store | 4 | `sp -0x80` | `[sp-128]+1` |
| `ldr` at 0x67cc8 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x67ce0 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x67d08 | load | 4 | `abs 0x67f70` | `-` |
| `ldr` at 0x67d14 | load | 4 | `abs 0x1329e8` | `-` |
| `ldr` at 0x67d18 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x67d28 | load | 4 | `abs 0x67f74` | `-` |
| `ldr` at 0x67d34 | load | 4 | `abs 0x1329ec` | `-` |
| `ldr` at 0x67d38 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x67d54 | load | 4 | `abs 0x67f68` | `-` |
| `ldr` at 0x67d70 | load | 4 | `abs 0x67f78` | `-` |
| `ldr` at 0x67d8c | load | 4 | `abs 0x67f78` | `-` |
| `ldr` at 0x67da8 | load | 4 | `abs 0x67f78` | `-` |
| `str` at 0x67dbc | store | 4 | `sp -0xa4` | `sp-96` |
| `ldr` at 0x67dc0 | load | 4 | `abs 0x67f7c` | `-` |
| `ldr` at 0x67dc8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x67dcc | load | 4 | `sp -0x64` | `-` |
| `str` at 0x67dd8 | store | 4 | `sp -0x68` | `ret(0xaa090)` |
| `ldr` at 0x67de0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x67dfc | load | 4 | `abs 0x67f68` | `-` |
| `ldr` at 0x67e18 | load | 4 | `abs 0x67f78` | `-` |
| `ldr` at 0x67e24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x67e28 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0x67e2c | store | 4 | `sp -0x20` | `[[sp-24]+28]` |
| `ldr` at 0x67e30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x67e34 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x67e38 | store | 4 | `sp -0x70` | `[[sp-24]+12]` |
| `ldr` at 0x67e3c | load | 4 | `abs 0x67f80` | `-` |
| `ldr` at 0x67e40 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x67e50 | load | 4 | `abs 0x67f84` | `-` |
| `ldr` at 0x67e54 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x67e58 | load | 4 | `abs 0x67f88` | `-` |
| `ldr` at 0x67e5c | load | 4 | `sp -0x70` | `-` |
| `str` at 0x67e6c | store | 4 | `sp -0xa4` | `0x2` |
| `ldr` at 0x67e70 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x67e74 | store | 4 | `sp -0xa0` | `[sp-100]` |
| `str` at 0x67e7c | store | 4 | `sp -0x9c` | `sp-96` |
| `ldr` at 0x67e80 | load | 4 | `abs 0x67f7c` | `-` |
| `ldr` at 0x67e88 | load | 4 | `sp -0x70` | `-` |
| `str` at 0x67e98 | store | 4 | `sp -0x1c` | `ret(0xaa424)` |
| `ldr` at 0x67e9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x67ea8 | load | 4 | `abs 0x67f84` | `-` |
| `ldr` at 0x67eac | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x67eb0 | load | 4 | `abs 0x67f8c` | `-` |
| `ldr` at 0x67ebc | load | 4 | `abs 0x67f80` | `-` |
| `ldr` at 0x67ec0 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x67ed0 | load | 4 | `abs 0x67f84` | `-` |
| `ldr` at 0x67ed4 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x67ed8 | load | 4 | `abs 0x67f90` | `-` |
| `ldr` at 0x67edc | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x67ee4 | load | 4 | `abs 0x67f94` | `-` |
| `ldr` at 0x67ee8 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x67efc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x67f08 | load | 4 | `abs 0x67f94` | `-` |
| `ldr` at 0x67f0c | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x67f20 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x67f2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x67f40 | load | 4 | `abs 0x67f98` | `-` |
| `ldr` at 0x67f44 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x67f4c | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x67f58 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x67f58 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x67f58 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x67f58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x67928 | `(unknown callee) 0xaadf4` | `r0` | `0x0` | `r2` | `?` |
| `bl` at 0x6794c | `(unknown callee) 0x5eac8` | `sp-100` | `sp-96` | `[sp-140]` | `sp-100` |
| `bl` at 0x679d8 | `(unknown callee) 0x194f0` | `0x0` | `?` | `?` | `[sp-116]` |
| `bl` at 0x679e4 | `(unknown callee) 0x61b04` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x67a28 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x67a54 | `(unknown callee) 0x19310` | `0x132ab4` | `[sp-144]` | `?` | `0x0` |
| `bl` at 0x67a6c | `(unknown callee) 0x19808` | `0x132ab4` | `[sp-120]` | `?` | `ret(0x19310)` |
| `bl` at 0x67a80 | `(unknown callee) 0xa1de8` | `[sp-120]` | `0x8000` | `?` | `?` |
| `bl` at 0x67aa8 | `(unknown callee) 0x6a9b0` | `0x1` | `?` | `?` | `[sp-124]` |
| `bl` at 0x67ab0 | `(unknown callee) 0x619dc` | `0xfc434` | `?` | `?` | `?` |
| `bl` at 0x67af0 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x2` | `?` | `?` |
| `bl` at 0x67b30 | `(unknown callee) 0x1970c` | `[0x1329e8]` | `[0x132adc]` | `0x1329e4` | `0x1329e8` |
| `bl` at 0x67b4c | `(unknown callee) 0x1970c` | `[0x1329ec]` | `?` | `?` | `0x1329ec` |
| `bl` at 0x67b6c | `(unknown callee) 0x1943c` | `[0x1329e4]` | `?` | `0x1329e4` | `0x1329e4` |
| `bl` at 0x67b98 | `(unknown callee) 0x1943c` | `?` | `?` | `[0x1329e4]` | `?` |
| `bl` at 0x67bb8 | `(unknown callee) 0x5eac8` | `sp-100` | `sp-96` | `0x7` | `sp-100` |
| `bl` at 0x67c3c | `(unknown callee) 0x69eec` | `?` | `?` | `?` | `?` |
| `bl` at 0x67c98 | `(unknown callee) 0x19610` | `?` | `[sp-120]` | `0x70` | `?` |
| `bl` at 0x67ccc | `(unknown callee) 0x5eb28` | `sp-100` | `sp-96` | `[sp-124]` | `sp-100` |
| `bl` at 0x67ce4 | `(unknown callee) 0x5eb28` | `sp-100` | `sp-96` | `[sp-136]` | `sp-100` |
| `bl` at 0x67cfc | `(unknown callee) 0x5eb28` | `sp-100` | `sp-96` | `0x70` | `sp-100` |
| `bl` at 0x67d1c | `(unknown callee) 0x5eca4` | `sp-100` | `sp-96` | `[0x1329e8]` | `[sp-136]` |
| `bl` at 0x67d3c | `(unknown callee) 0x67280` | `sp-100` | `sp-96` | `[0x1329ec]` | `[sp-136]` |
| `bl` at 0x67d5c | `(unknown callee) 0x67280` | `sp-100` | `sp-96` | `0x132a50` | `0x1` |
| `bl` at 0x67d78 | `(unknown callee) 0x67280` | `sp-100` | `sp-96` | `0x132cf4` | `0x1` |
| `bl` at 0x67d94 | `(unknown callee) 0x67280` | `sp-100` | `sp-96` | `0x132cf4` | `0x1` |
| `bl` at 0x67db0 | `(unknown callee) 0x67280` | `sp-100` | `sp-96` | `0x132cf4` | `0x1` |
| `bl` at 0x67db4 | `(unknown callee) 0x61a78` | `?` | `?` | `?` | `?` |
| `bl` at 0x67dd0 | `(unknown callee) 0xaa090` | `0x132a48` | `0x0` | `[sp-24]` | `[sp-100]` |
| `bl` at 0x67de8 | `(unknown callee) 0x643b8` | `[sp-24]` | `0x1` | `?` | `[sp-116]` |
| `bl` at 0x67e04 | `(unknown callee) 0x67280` | `sp-100` | `sp-96` | `0x132a50` | `0x1` |
| `bl` at 0x67e20 | `(unknown callee) 0x67280` | `sp-100` | `sp-96` | `0x132cf4` | `0x1` |
| `bl` at 0x67e60 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc44c` | `[[sp-24]+12]` | `0x12eeb8` |
| `bl` at 0x67e90 | `(unknown callee) 0xaa424` | `0x132a48` | `sp-32` | `?` | `0x1` |
| `bl` at 0x67eb4 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc46c` | `?` | `0x12eeb8` |
| `bl` at 0x67ee0 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc4a4` | `[sp-112]` | `0x12eeb8` |
| `bl` at 0x67f30 | `(unknown callee) 0xa9d18` | `?` | `?` | `0x29a550` | `?` |
| `bl` at 0x67f48 | `(unknown callee) 0x1e958` | `0xfc4c8` | `[sp-112]` | `?` | `ret(0xa9d18)` |
| `bl` at 0x67f50 | `(unknown callee) 0xa8b98` | `[sp-28]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x6790c | `? != 0x0` | `? == 0x0` |
| `bgt` at 0x67960 | `[sp-116] > 0xf (signed, 32-bit)` | `[sp-116] <= 0xf (signed, 32-bit)` |
| `bge` at 0x6796c | `[sp-116] >= 0xc (signed, 32-bit)` | `[sp-116] < 0xc (signed, 32-bit)` |
| `beq` at 0x67978 | `[sp-116] == 0x6` | `[sp-116] != 0x6` |
| `bgt` at 0x67984 | `[sp-116] > 0x6 (signed, 32-bit)` | `[sp-116] <= 0x6 (signed, 32-bit)` |
| `blt` at 0x67990 | `[sp-116] < 0x1 (signed, 32-bit)` | `[sp-116] >= 0x1 (signed, 32-bit)` |
| `beq` at 0x679a0 | `[sp-116] == 0x9` | `[sp-116] != 0x9` |
| `blt` at 0x679b0 | `[sp-116] < 0x14 (signed, 32-bit)` | `[sp-116] >= 0x14 (signed, 32-bit)` |
| `ble` at 0x679bc | `[sp-116] <= 0x19 (signed, 32-bit)` | `[sp-116] > 0x19 (signed, 32-bit)` |
| `beq` at 0x679c8 | `[sp-116] == 0x3e8` | `[sp-116] != 0x3e8` |
| `bls` at 0x679f8 | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` |
| `bne` at 0x67a0c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x67a40 | `[sp-144] == 0x0` | `[sp-144] != 0x0` |
| `beq` at 0x67a60 | `ret(0x19310) == 0x0` | `ret(0x19310) != 0x0` |
| `beq` at 0x67a8c | `ret(0xa1de8) == 0x0` | `ret(0xa1de8) != 0x0` |
| `beq` at 0x67aa0 | `[sp-124] == 0x0` | `[sp-124] != 0x0` |
| `bls` at 0x67ac0 | `[[sp-24]+32]:1 <= 0x2 (unsigned, 32-bit)` | `[[sp-24]+32]:1 > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x67ad4 | `? != 0x0` | `? == 0x0` |
| `ble` at 0x67b14 | `[0x132adc] <= [0x1329e4] (signed, 32-bit)` | `[0x132adc] > [0x1329e4] (signed, 32-bit)` |
| `beq` at 0x67b24 | `[0x1329e8] == 0x0` | `[0x1329e8] != 0x0` |
| `beq` at 0x67b40 | `[0x1329ec] == 0x0` | `[0x1329ec] != 0x0` |
| `bne` at 0x67bc4 | `[sp-124] != 0x0` | `[sp-124] == 0x0` |
| `ble` at 0x67bf0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x67c50 | `ret(0x69eec) == 0x0` | `ret(0x69eec) != 0x0` |
| `beq` at 0x67e4c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x67ea4 | `ret(0xaa424) != 0x0` | `ret(0xaa424) == 0x0` |
| `beq` at 0x67ecc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x67f04 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x67f3c | `ret(0xa9d18) == 0x0` | `ret(0xa9d18) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x67f58 | `r0` | `?` |

## `B2BReqRsp`

- `B2BReqRsp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6ee64 to 0x6f4d4.
  Established. The signed condition `[[sp-20]+24] >= 0x0` takes `0x6eedc`, while negative values fall through (`bge` at 0x6eeb8).
  The computed transfer is taken for unsigned `[sp-60]-1 <= 0xe`, with fall-through when greater (`ldrls` at 0x6ef60).
  The unnamed `0xaa090` call at `0x6f430` carries `0x29a550`, `0xfffffd9e`, `[sp-20]`, `0x0` (`bl` at 0x6f430).
  Board-link request routing across unresolved destinations remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6ee74 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6ee7c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x6ee80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6ee84 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x6ee94 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x6ee98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6ee9c | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x6eea8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x6eeac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6eeb0 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x6eebc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6eec0 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x6eec8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x6eecc | load | 4 | `sp -0x38` | `-` |
| `str` at 0x6eed4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x6eedc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6eee0 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x6eee8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x6eeec | load | 4 | `sp -0x38` | `-` |
| `str` at 0x6eef0 | store | 4 | `sp -0x2c` | `[sp-56]` |
| `ldr` at 0x6eef4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6eef8 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x6ef04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6ef08 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x6ef1c | store | 4 | `sp -0x3c` | `0xe` |
| `str` at 0x6ef28 | store | 4 | `sp -0x3c` | `0xf` |
| `ldr` at 0x6ef30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6ef34 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x6ef3c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x6ef40 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x6ef48 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x6ef4c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x6ef50 | store | 4 | `sp -0x30` | `[sp-60]` |
| `ldr` at 0x6ef54 | load | 4 | `sp -0x30` | `-` |
| `ldrls` at 0x6ef60 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6f214 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6f220 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f22c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6f238 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f244 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6f250 | load | 4 | `abs 0x6f4a8` | `-` |
| `ldr` at 0x6f258 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f25c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x6f274 | store | 4 | `sp -0x34` | `ret(0x6a038)` |
| `ldr` at 0x6f278 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x6f284 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x6f288 | load | 4 | `ret(0x6a038) +0x10` | `-` |
| `str` at 0x6f28c | store | 4 | `sp -0x20` | `[ret(0x6a038)+16]` |
| `ldr` at 0x6f290 | load | 4 | `abs 0x6f49c` | `-` |
| `ldr` at 0x6f294 | load | 4 | `abs 0x132efc` | `-` |
| `ldr` at 0x6f2a4 | load | 4 | `abs 0x6f4a0` | `-` |
| `ldr` at 0x6f2a8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x6f2ac | store | 4 | `sp -0x48` | `[ret(0x6a038)+16]` |
| `ldr` at 0x6f2b0 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6f2b4 | load | 4 | `abs 0x6f4ac` | `-` |
| `ldr` at 0x6f2b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f2bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6f2c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6f2d4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6f2e0 | store | 4 | `sp -0x20` | `ret(0x6f5c8)` |
| `ldr` at 0x6f2e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6f2f4 | load | 4 | `abs 0x6f49c` | `-` |
| `ldr` at 0x6f2f8 | load | 4 | `abs 0x132efc` | `-` |
| `ldr` at 0x6f308 | load | 4 | `abs 0x6f4a0` | `-` |
| `ldr` at 0x6f30c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x6f310 | store | 4 | `sp -0x48` | `ret(0x6f5c8)` |
| `ldr` at 0x6f314 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6f318 | load | 4 | `abs 0x6f4b0` | `-` |
| `ldr` at 0x6f31c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f320 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6f328 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6f33c | load | 4 | `abs 0x6f49c` | `-` |
| `ldr` at 0x6f340 | load | 4 | `abs 0x132efc` | `-` |
| `ldr` at 0x6f350 | load | 4 | `abs 0x6f4a0` | `-` |
| `ldr` at 0x6f354 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6f358 | load | 4 | `abs 0x6f4b4` | `-` |
| `ldr` at 0x6f35c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6f364 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x6f368 | load | 1 | `[sp-20] +0x16` | `-` |
| `str` at 0x6f378 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x6f37c | load | 4 | `abs 0x6f4b8` | `-` |
| `ldr` at 0x6f380 | load | 4 | `abs 0x6f4bc` | `-` |
| `ldr` at 0x6f384 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f390 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f39c | load | 4 | `abs 0x6f4b8` | `-` |
| `ldr` at 0x6f3a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6f3b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6f3c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6f3cc | store | 4 | `sp -0x18` | `ret(0xa9d18)` |
| `ldr` at 0x6f3d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6f3dc | load | 4 | `abs 0x6f49c` | `-` |
| `ldr` at 0x6f3e0 | load | 4 | `abs 0x132efc` | `-` |
| `ldr` at 0x6f3f0 | load | 4 | `abs 0x6f4a0` | `-` |
| `ldr` at 0x6f3f4 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6f3f8 | load | 4 | `abs 0x6f4c0` | `-` |
| `ldr` at 0x6f3fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6f400 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6f408 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x6f40c | load | 1 | `[sp-20] +0x16` | `-` |
| `str` at 0x6f41c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x6f420 | load | 4 | `abs 0x6f4b8` | `-` |
| `ldr` at 0x6f424 | load | 4 | `abs 0x6f4c4` | `-` |
| `ldr` at 0x6f428 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f434 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6f43c | store | 4 | `sp -0x40` | `ret(0x1f8e0)` |
| `ldr` at 0x6f440 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x6f444 | load | 1 | `[sp-20] +0x16` | `-` |
| `ldr` at 0x6f450 | load | 4 | `abs 0x6f4c8` | `-` |
| `str` at 0x6f454 | store | 4 | `sp -0x44` | `0xfd1e0` |
| `ldr` at 0x6f45c | load | 4 | `abs 0x6f4cc` | `-` |
| `str` at 0x6f460 | store | 4 | `sp -0x44` | `0xfd1e4` |
| `ldr` at 0x6f464 | load | 4 | `abs 0x6f4d0` | `-` |
| `ldr` at 0x6f468 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6f46c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6f470 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x6f478 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x6f484 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6f484 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6f484 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6f254 | `(unknown callee) 0x619dc` | `0xfd130` | `r1` | `r2` | `[sp-40]` |
| `bl` at 0x6f260 | `(unknown callee) 0x69f40` | `[sp-36]` | `[sp-40]` | `?` | `?` |
| `bl` at 0x6f26c | `(unknown callee) 0x6a038` | `ret(0x69f40)` | `?` | `?` | `ret(0x69f40)` |
| `bl` at 0x6f2c0 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfd13c` | `[sp-36]` | `[sp-40]` |
| `bl` at 0x6f2c4 | `(unknown callee) 0x61a78` | `?` | `?` | `?` | `?` |
| `bl` at 0x6f2d8 | `(unknown callee) 0x6f5c8` | `[sp-20]` | `?` | `?` | `[sp-32]` |
| `bl` at 0x6f324 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfd160` | `[sp-36]` | `[sp-40]` |
| `bl` at 0x6f32c | `(unknown callee) 0x6f4d4` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x6f360 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfd19c` | `[sp-32]` | `0x12eeb8` |
| `bl` at 0x6f38c | `(unknown callee) 0xaa090` | `0x29a550` | `0xfffffda0` | `[sp-20]` | `0x0` |
| `bl` at 0x6f394 | `(unknown callee) 0xa8b98` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x6f3c4 | `(unknown callee) 0xa9d18` | `?` | `[sp-20]` | `0x29a550` | `?` |
| `bl` at 0x6f404 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfd1b8` | `[sp-32]` | `ret(0xa9d18)` |
| `bl` at 0x6f430 | `(unknown callee) 0xaa090` | `0x29a550` | `0xfffffd9e` | `[sp-20]` | `0x0` |
| `bl` at 0x6f438 | `(unknown callee) 0x1f8e0` | `?` | `?` | `?` | `?` |
| `bl` at 0x6f474 | `(unknown callee) 0x1e958` | `0xfd1e8` | `[sp-32]` | `[sp-68]` | `ret(0x1f8e0)` |
| `bl` at 0x6f47c | `(unknown callee) 0xa8b98` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x6eeb8 | `[[sp-20]+24] >= 0x0 (signed, 32-bit)` | `[[sp-20]+24] < 0x0 (signed, 32-bit)` |
| `bge` at 0x6ef00 | `[[sp-20]+24] >= 0x0 (signed, 32-bit)` | `[[sp-20]+24] < 0x0 (signed, 32-bit)` |
| `bne` at 0x6ef14 | `? != 0x2` | `? == 0x2` |
| `ldrls` at 0x6ef60 | `[sp-60]-1 <= 0xe (unsigned, 32-bit)` | `[sp-60]-1 > 0xe (unsigned, 32-bit)` |
| `bne` at 0x6f21c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x6f228 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0x6f234 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `beq` at 0x6f240 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x6f24c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x6f280 | `ret(0x6a038) == 0x0` | `ret(0x6a038) != 0x0` |
| `beq` at 0x6f2a0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6f2d0 | `Z == 0` | `not (Z == 0)` |
| `bgt` at 0x6f2ec | `ret(0x6f5c8) > 0x0 (signed, 32-bit)` | `ret(0x6f5c8) <= 0x0 (signed, 32-bit)` |
| `beq` at 0x6f304 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6f338 | `ret(0x6f4d4) != 0x0` | `ret(0x6f4d4) == 0x0` |
| `beq` at 0x6f34c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6f370 | `[[sp-20]+22]:1 != 0x0` | `[[sp-20]+22]:1 == 0x0` |
| `beq` at 0x6f3d8 | `ret(0xa9d18) == 0x0` | `ret(0xa9d18) != 0x0` |
| `beq` at 0x6f3ec | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6f414 | `[[sp-20]+22]:1 != 0x0` | `[[sp-20]+22]:1 == 0x0` |
| `bne` at 0x6f44c | `[[sp-20]+22]:1 != 0x0` | `[[sp-20]+22]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6f484 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x6ef60).
