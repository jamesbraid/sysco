# ARM reset and release control paths in 1.44.0

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
The computed transfer in resetSource remains unresolved; its tables cover known edges only.
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/reset-release-control-facts.txt) retain all tool sections.

## `resetHold`

- `resetHold`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x47ba4 to 0x47bf8.
  Established. Equality entry `r0 == 0x5` takes `0x47bdc`, while inequality falls through (`beq` at 0x47bc0).
  The unnamed `0x480f8` call carries `ret(0xd90fc)`, unknown, unknown, unknown (`bl` at 0x47be0).
  The return record reports `ret(0x480f8)` (`ldm` at 0x47bf0).
  Reset hold operation meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x47bb4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x47bb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x47bc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x47bd0 | load | 4 | `abs 0x47bf4` | `-` |
| `ldm` at 0x47bf0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x47bf0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x47bf0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x47bc8 | `(unknown callee) 0x48054` | `r0` | `r1` | `r2` | `r0` |
| `bl` at 0x47bd8 | `(unknown callee) 0x1e958` | `0xf791c` | `ret(0x48054)` | `?` | `ret(0x48054)` |
| `bl` at 0x47bdc | `(unknown callee) 0xd90fc` | `?` | `?` | `?` | `?` |
| `bl` at 0x47be0 | `(unknown callee) 0x480f8` | `ret(0xd90fc)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x47bc0 | `r0 == 0x5` | `r0 != 0x5` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x47bf0 | `r0` | `ret(0x480f8)` |

## `resetRelease`

- `resetRelease`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x47bf8 to 0x47e40.
  Established. The unnamed `0x3b9cc` call carries `0x0`, `0xff`, `0x5`, unknown (`bl` at 0x47dac).
  A zero `[0x297648]` takes `0x47d98`, while nonzero falls through (`beq` at 0x47d78).
  The included indirect call at `0x47d8c` carries `0xff`, unknown, unknown, `[0x297648]` (`mov` at 0x47d8c).
  Reset release hardware sequencing remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x47c08 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x47c18 | store | 4 | `sp -0x18` | `?` |
| `str` at 0x47c20 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x47c24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x47c30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x47c3c | load | 4 | `abs 0x47e38` | `-` |
| `ldr` at 0x47c4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47c64 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x47c68 | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0x47c78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47c84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47c90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47c9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47ca8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47cb4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47cc0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47ccc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47cd8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47ce4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47cf0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47cfc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47d08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47d14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47d20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47d2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47d38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47d44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47d6c | load | 4 | `abs 0x47e3c` | `-` |
| `ldr` at 0x47d70 | load | 4 | `abs 0x297648` | `-` |
| `ldr` at 0x47d7c | load | 4 | `abs 0x47e3c` | `-` |
| `ldr` at 0x47d80 | load | 4 | `abs 0x297648` | `-` |
| `str` at 0x47d94 | store | 4 | `sp -0x1c` | `ret(unknown)` |
| `str` at 0x47db4 | store | 4 | `sp -0x20` | `ret(0x3b9cc)` |
| `ldr` at 0x47db8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x47dc4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x47dd0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x47dd4 | store | 4 | `sp -0x1c` | `ret(0x3b9cc)` |
| `ldr` at 0x47dd8 | load | 4 | `abs 0x47e3c` | `-` |
| `ldr` at 0x47ddc | load | 4 | `abs 0x29764c` | `-` |
| `ldr` at 0x47de8 | load | 4 | `abs 0x47e3c` | `-` |
| `ldr` at 0x47dec | load | 4 | `abs 0x29764c` | `-` |
| `str` at 0x47e00 | store | 4 | `sp -0x20` | `ret(unknown)` |
| `ldr` at 0x47e04 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x47e10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x47e1c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x47e20 | store | 4 | `sp -0x1c` | `ret(unknown)` |
| `ldr` at 0x47e24 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x47e28 | store | 4 | `sp -0x24` | `[sp-28]` |
| `ldr` at 0x47e2c | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x47e34 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x47e34 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x47e34 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x47c0c | `(unknown callee) 0x488cc` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x47c34 | `(unknown callee) 0x48054` | `[sp-20]` | `?` | `?` | `[sp-20]` |
| `bl` at 0x47c44 | `(unknown callee) 0x1e958` | `0xf792c` | `ret(0x48054)` | `?` | `ret(0x48054)` |
| `bl` at 0x47c54 | `(unknown callee) 0xd9010` | `[sp-24]` | `sp-32` | `?` | `sp-32` |
| `bl` at 0x47c74 | `(unknown callee) 0x47b7c` | `0xa` | `?` | `?` | `ret(0xd9010)` |
| `bl` at 0x47d54 | `(unknown callee) 0x77234` | `ret(0x47b7c)` | `?` | `?` | `[sp-24]` |
| `bl` at 0x47d58 | `(unknown callee) 0x7a790` | `ret(0x77234)` | `?` | `?` | `?` |
| `bl` at 0x47d5c | `(unknown callee) 0x5da68` | `?` | `?` | `?` | `?` |
| `bl` at 0x47d68 | `(unknown callee) 0x546c4` | `0xff` | `0x0` | `?` | `?` |
| `mov` at 0x47d8c | `(unknown callee) (unknown target)` | `0xff` | `?` | `?` | `[0x297648]` |
| `bl` at 0x47d9c | `(unknown callee) 0x3a28c` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x47dac | `(unknown callee) 0x3b9cc` | `0x0` | `0xff` | `0x5` | `?` |
| `mov` at 0x47df8 | `(unknown callee) (unknown target)` | `0xff` | `?` | `?` | `[0x29764c]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x47c2c | `[sp-20] == 0x5` | `[sp-20] != 0x5` |
| `bne` at 0x47c60 | `ret(0xd9010) != 0x0` | `ret(0xd9010) == 0x0` |
| `beq` at 0x47c80 | `[sp-24] == 0x43` | `[sp-24] != 0x43` |
| `beq` at 0x47c8c | `[sp-24] == 0x33` | `[sp-24] != 0x33` |
| `beq` at 0x47c98 | `[sp-24] == 0x35` | `[sp-24] != 0x35` |
| `beq` at 0x47ca4 | `[sp-24] == 0x34` | `[sp-24] != 0x34` |
| `beq` at 0x47cb0 | `[sp-24] == 0x41` | `[sp-24] != 0x41` |
| `beq` at 0x47cbc | `[sp-24] == 0x36` | `[sp-24] != 0x36` |
| `beq` at 0x47cc8 | `[sp-24] == 0x39` | `[sp-24] != 0x39` |
| `beq` at 0x47cd4 | `[sp-24] == 0x37` | `[sp-24] != 0x37` |
| `beq` at 0x47ce0 | `[sp-24] == 0x38` | `[sp-24] != 0x38` |
| `beq` at 0x47cec | `[sp-24] == 0x30` | `[sp-24] != 0x30` |
| `beq` at 0x47cf8 | `[sp-24] == 0x51` | `[sp-24] != 0x51` |
| `beq` at 0x47d04 | `[sp-24] == 0x40` | `[sp-24] != 0x40` |
| `beq` at 0x47d10 | `[sp-24] == 0x4f` | `[sp-24] != 0x4f` |
| `beq` at 0x47d1c | `[sp-24] == 0x42` | `[sp-24] != 0x42` |
| `beq` at 0x47d28 | `[sp-24] == 0x46` | `[sp-24] != 0x46` |
| `beq` at 0x47d34 | `[sp-24] == 0x4c` | `[sp-24] != 0x4c` |
| `beq` at 0x47d40 | `[sp-24] == 0x4d` | `[sp-24] != 0x4d` |
| `beq` at 0x47d4c | `[sp-24] == 0x7c` | `[sp-24] != 0x7c` |
| `beq` at 0x47d78 | `[0x297648] == 0x0` | `[0x297648] != 0x0` |
| `beq` at 0x47dc0 | `ret(0x3b9cc) == 0x0` | `ret(0x3b9cc) != 0x0` |
| `bne` at 0x47dcc | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x47de4 | `[0x29764c] == 0x0` | `[0x29764c] != 0x0` |
| `beq` at 0x47e0c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x47e18 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x47e34 | `r0` | `?` |

## `reset`

- `reset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x47ad8 to 0x47b7c.
  Established. The word at `sp -0x1c` receives `ret(0x480f8)` (`str` at 0x47b18).
  A zero result from that call takes `0x47b34`, while nonzero falls through (`beq` at 0x47b24).
  The unnamed `0x47bf8` call carries `0x5`, unknown, unknown, unknown (`bl` at 0x47b40).
  Reset orchestration policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x47ae8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x47aec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x47af8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x47b04 | load | 4 | `abs 0x47b78` | `-` |
| `str` at 0x47b18 | store | 4 | `sp -0x1c` | `ret(0x480f8)` |
| `ldr` at 0x47b1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x47b28 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x47b2c | store | 4 | `sp -0x20` | `ret(0x480f8)` |
| `str` at 0x47b48 | store | 4 | `sp -0x1c` | `ret(0x47bf8)` |
| `ldr` at 0x47b4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x47b58 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x47b5c | store | 4 | `sp -0x20` | `ret(0x47bf8)` |
| `str` at 0x47b68 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x47b6c | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x47b74 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x47b74 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x47b74 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x47afc | `(unknown callee) 0x48054` | `r0` | `r1` | `r2` | `r0` |
| `bl` at 0x47b0c | `(unknown callee) 0x1e958` | `0xf7910` | `ret(0x48054)` | `?` | `ret(0x48054)` |
| `bl` at 0x47b10 | `(unknown callee) 0x480f8` | `?` | `?` | `?` | `?` |
| `bl` at 0x47b38 | `(unknown callee) 0x47b7c` | `0xa` | `?` | `?` | `ret(0x480f8)` |
| `bl` at 0x47b40 | `(unknown callee) 0x47bf8` | `0x5` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x47af4 | `r0 == 0x5` | `r0 != 0x5` |
| `beq` at 0x47b24 | `ret(0x480f8) == 0x0` | `ret(0x480f8) != 0x0` |
| `beq` at 0x47b54 | `ret(0x47bf8) == 0x0` | `ret(0x47bf8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x47b74 | `r0` | `?` |

## `softReset`

- `softReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x47e40 to 0x47ed0.
  Established. The byte at `sp -0x15` receives `(r1 & 0xff)` (`strb` at 0x47e58).
  A zero `[0x297650]` takes `0x47eb4`, while nonzero falls through (`beq` at 0x47e8c).
  The included indirect call carries three unknown candidates followed by `[0x297650]` (`mov` at 0x47ea4).
  Soft-reset hook behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x47e50 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x47e58 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x47e5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x47e68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x47e74 | load | 4 | `abs 0x47ec8` | `-` |
| `ldr` at 0x47e80 | load | 4 | `abs 0x47ecc` | `-` |
| `ldr` at 0x47e84 | load | 4 | `abs 0x297650` | `-` |
| `ldr` at 0x47e90 | load | 4 | `abs 0x47ecc` | `-` |
| `ldrb` at 0x47e94 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x47e98 | load | 4 | `abs 0x297650` | `-` |
| `str` at 0x47eac | store | 4 | `sp -0x1c` | `ret(unknown)` |
| `str` at 0x47eb8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x47ebc | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x47ec4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x47ec4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x47ec4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x47e6c | `(unknown callee) 0x48054` | `r0` | `r1` | `r2` | `r0` |
| `bl` at 0x47e7c | `(unknown callee) 0x1e958` | `0xf7940` | `ret(0x48054)` | `?` | `ret(0x48054)` |
| `mov` at 0x47ea4 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `[0x297650]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x47e64 | `r0 == 0x5` | `r0 != 0x5` |
| `beq` at 0x47e8c | `[0x297650] == 0x0` | `[0x297650] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x47ec4 | `r0` | `?` |

## `resetSource`

- `resetSource`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x48054 to 0x480f8.
  Established. The computed transfer has an unresolved taken edge for unsigned `r0-1 <= 0x4`, while larger values fall through (`ldrls` at 0x48074).
  On known edges, the word at `sp -0x18` receives `0xf798c` (`str` at 0x480d0).
  The retained return record gives `0xf798c` (`ldm` at 0x480dc).
  Reset-source label selection remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x48064 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x48068 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0x48074 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x480cc | load | 4 | `abs 0x480f4` | `-` |
| `str` at 0x480d0 | store | 4 | `sp -0x18` | `0xf798c` |
| `ldr` at 0x480d4 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x480dc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x480dc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x480dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x48074 | `r0-1 <= 0x4 (unsigned, 32-bit)` | `r0-1 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x480dc | `r0` | `0xf798c` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x48074).

## `SHUB_Reset_Hold`

- `SHUB_Reset_Hold`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x46d3c to 0x46d60.
  Established. The byte at `sp -0x11` receives `(r0 & 0xff)` (`strb` at 0x46d50).
  The integer return record is `0x0` (`ldm` at 0x46d5c).
  No call or conditional branch is present in the retained span.
  Hardware reset action remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x46d50 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldm` at 0x46d5c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x46d5c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x46d5c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x46d5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x46d5c | `r0` | `0x0` |

## `SHUB_Reset_PreRelease`

- `SHUB_Reset_PreRelease`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x46d60 to 0x472ac.
  Established. The unnamed `0xde978` call at `0x46f98` carries `[sp-112]`, `0x60`, `sp-28`, `0x0` (`bl` at 0x46f98);
  its zero result takes `0x46fd8`, while nonzero falls through (`beq` at 0x46fa4).
  An included word store assigns `0xfffff894` to `sp -0x74` (`str` at 0x46fd0).
  Pre-release scan programming meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x46d74 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x46d7c | store | 4 | `sp -0x18` | `0x0` |
| `ldrb` at 0x46d80 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x46d88 | load | 4 | `abs 0x47238` | `-` |
| `ldr` at 0x46d8c | load | 4 | `abs 0x132228` | `-` |
| `ldr` at 0x46d94 | load | 4 | `(unknown)` | `-` |
| `str` at 0x46d98 | store | 4 | `sp -0x70` | `?` |
| `ldrb` at 0x46d9c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x46dac | store | 4 | `sp -0x18` | `ret(0x45714)` |
| `ldr` at 0x46db0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x46dbc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x46dc0 | store | 4 | `sp -0x74` | `ret(0x45714)` |
| `ldrb` at 0x46dc8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x46dd8 | store | 4 | `sp -0x18` | `ret(0x45b28)` |
| `ldr` at 0x46ddc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x46de8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x46dec | store | 4 | `sp -0x74` | `ret(0x45b28)` |
| `ldr` at 0x46e14 | load | 4 | `abs 0x4723c` | `-` |
| `str` at 0x46e18 | store | 4 | `sp -0x74` | `0xfffff883` |
| `ldrb` at 0x46e20 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x46e34 | store | 4 | `sp -0x18` | `ret(0x44c98)` |
| `ldr` at 0x46e38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x46e44 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x46e48 | store | 4 | `sp -0x74` | `ret(0x44c98)` |
| `ldrb` at 0x46e50 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x46e60 | store | 4 | `sp -0x18` | `ret(0x43088)` |
| `ldr` at 0x46e64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x46e70 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x46e74 | store | 4 | `sp -0x74` | `ret(0x43088)` |
| `ldr` at 0x46e7c | load | 4 | `abs 0x47240` | `-` |
| `ldr` at 0x46e80 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x46e8c | load | 4 | `abs 0x47244` | `-` |
| `str` at 0x46e90 | store | 4 | `sp -0x7c` | `0xf75c4` |
| `ldr` at 0x46e98 | load | 4 | `abs 0x47248` | `-` |
| `ldr` at 0x46e9c | load | 4 | `abs 0x4724c` | `-` |
| `str` at 0x46eac | store | 4 | `sp -0x1c` | `0x10` |
| `ldr` at 0x46eb0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x46eb4 | load | 4 | `abs 0x47250` | `-` |
| `ldr` at 0x46ec0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x46ec8 | load | 4 | `abs 0x47250` | `-` |
| `str` at 0x46ed4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x46ed8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x46edc | load | 4 | `abs 0x47254` | `-` |
| `ldr` at 0x46ee8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x46ef0 | load | 4 | `abs 0x47254` | `-` |
| `str` at 0x46efc | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x46f00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x46f04 | load | 4 | `abs 0x47258` | `-` |
| `ldr` at 0x46f10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x46f18 | load | 4 | `abs 0x47258` | `-` |
| `str` at 0x46f24 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x46f28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x46f38 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x46f4c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x46f50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x46f60 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x46f74 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x46f80 | store | 4 | `sp -0x7c` | `0xa` |
| `ldr` at 0x46f84 | load | 4 | `abs 0x4725c` | `-` |
| `str` at 0x46f88 | store | 4 | `sp -0x78` | `0xf75d8` |
| `ldr` at 0x46f8c | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x46fa8 | load | 4 | `abs 0x47260` | `-` |
| `str` at 0x46fac | store | 4 | `sp -0x7c` | `0xf75e8` |
| `ldrb` at 0x46fb0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x46fb4 | store | 4 | `sp -0x78` | `[sp-17]:1` |
| `ldr` at 0x46fbc | load | 4 | `abs 0x47248` | `-` |
| `ldr` at 0x46fc0 | load | 4 | `abs 0x47264` | `-` |
| `ldr` at 0x46fcc | load | 4 | `abs 0x47268` | `-` |
| `str` at 0x46fd0 | store | 4 | `sp -0x74` | `0xfffff894` |
| `str` at 0x46fe0 | store | 4 | `sp -0x7c` | `0xa` |
| `ldr` at 0x46fe4 | load | 4 | `abs 0x4726c` | `-` |
| `str` at 0x46fe8 | store | 4 | `sp -0x78` | `0xf7618` |
| `ldr` at 0x46fec | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x47008 | load | 4 | `abs 0x47270` | `-` |
| `str` at 0x4700c | store | 4 | `sp -0x7c` | `0xf7628` |
| `ldrb` at 0x47010 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x47014 | store | 4 | `sp -0x78` | `[sp-17]:1` |
| `ldr` at 0x4701c | load | 4 | `abs 0x47248` | `-` |
| `ldr` at 0x47020 | load | 4 | `abs 0x47274` | `-` |
| `ldr` at 0x4702c | load | 4 | `abs 0x47268` | `-` |
| `str` at 0x47030 | store | 4 | `sp -0x74` | `0xfffff894` |
| `ldr` at 0x47038 | load | 4 | `abs 0x47240` | `-` |
| `ldr` at 0x4703c | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x47048 | load | 4 | `abs 0x47278` | `-` |
| `str` at 0x4704c | store | 4 | `sp -0x7c` | `0xf7658` |
| `ldr` at 0x47054 | load | 4 | `abs 0x47248` | `-` |
| `ldr` at 0x47058 | load | 4 | `abs 0x4727c` | `-` |
| `ldr` at 0x47064 | load | 4 | `abs 0x47280` | `-` |
| `str` at 0x4706c | store | 4 | `sp -0x7c` | `0x7e` |
| `ldr` at 0x47070 | load | 4 | `abs 0x47284` | `-` |
| `str` at 0x47074 | store | 4 | `sp -0x78` | `0xf7664` |
| `ldr` at 0x47078 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x47080 | load | 4 | `abs 0x132240` | `-` |
| `ldr` at 0x47098 | load | 4 | `abs 0x47288` | `-` |
| `str` at 0x4709c | store | 4 | `sp -0x7c` | `0xf766c` |
| `ldrb` at 0x470a0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x470a4 | store | 4 | `sp -0x78` | `[sp-17]:1` |
| `ldr` at 0x470ac | load | 4 | `abs 0x47248` | `-` |
| `ldr` at 0x470b0 | load | 4 | `abs 0x4728c` | `-` |
| `ldr` at 0x470bc | load | 4 | `abs 0x47290` | `-` |
| `str` at 0x470c0 | store | 4 | `sp -0x74` | `0xfffff884` |
| `ldr` at 0x470c8 | load | 4 | `abs 0x47240` | `-` |
| `ldr` at 0x470cc | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x470d8 | load | 4 | `abs 0x47294` | `-` |
| `str` at 0x470dc | store | 4 | `sp -0x7c` | `0xf7694` |
| `ldr` at 0x470e4 | load | 4 | `abs 0x47248` | `-` |
| `ldr` at 0x470e8 | load | 4 | `abs 0x47298` | `-` |
| `str` at 0x470f8 | store | 4 | `sp -0x1c` | `0xff` |
| `ldr` at 0x470fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x47100 | load | 4 | `abs 0x47250` | `-` |
| `ldr` at 0x4710c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x47114 | load | 4 | `abs 0x47250` | `-` |
| `str` at 0x47120 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x47124 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x47128 | load | 4 | `abs 0x47254` | `-` |
| `ldr` at 0x47134 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4713c | load | 4 | `abs 0x47254` | `-` |
| `str` at 0x47148 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x4714c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x47150 | load | 4 | `abs 0x47258` | `-` |
| `ldr` at 0x4715c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x47164 | load | 4 | `abs 0x47258` | `-` |
| `str` at 0x47170 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x47174 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x47184 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x47198 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x4719c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x471ac | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x471c0 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x471cc | store | 4 | `sp -0x7c` | `0x8` |
| `ldr` at 0x471d0 | load | 4 | `abs 0x4729c` | `-` |
| `str` at 0x471d4 | store | 4 | `sp -0x78` | `0xf76a4` |
| `ldr` at 0x471d8 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x471f4 | load | 4 | `abs 0x472a0` | `-` |
| `str` at 0x471f8 | store | 4 | `sp -0x7c` | `0xf76b0` |
| `ldrb` at 0x471fc | load | 1 | `sp -0x11` | `-` |
| `str` at 0x47200 | store | 4 | `sp -0x78` | `[sp-17]:1` |
| `ldr` at 0x47208 | load | 4 | `abs 0x47248` | `-` |
| `ldr` at 0x4720c | load | 4 | `abs 0x472a4` | `-` |
| `ldr` at 0x47218 | load | 4 | `abs 0x472a8` | `-` |
| `str` at 0x4721c | store | 4 | `sp -0x74` | `0xfffff892` |
| `str` at 0x47228 | store | 4 | `sp -0x74` | `0x0` |
| `ldr` at 0x4722c | load | 4 | `sp -0x74` | `-` |
| `ldm` at 0x47234 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x47234 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x47234 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x46da4 | `(unknown callee) 0x45714` | `(r0 & 0xff)` | `r1` | `?` | `(r0 & 0xff)` |
| `bl` at 0x46dd0 | `(unknown callee) 0x45b28` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x46e04 | `(unknown callee) 0xd0814` | `0x1` | `0x0` | `sp-108` | `sp-108` |
| `bl` at 0x46e2c | `(unknown callee) 0x44c98` | `[sp-17]:1` | `0x1` | `?` | `[sp-17]:1` |
| `bl` at 0x46e58 | `(unknown callee) 0x43088` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x46ea4 | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x767` | `0x0` |
| `bl` at 0x46f98 | `(unknown callee) 0xde978` | `[sp-112]` | `0x60` | `sp-28` | `0x0` |
| `bl` at 0x46fc8 | `(unknown callee) 0x200a0` | `0x3` | `0xf67f0` | `0x76d` | `0x0` |
| `bl` at 0x46ff8 | `(unknown callee) 0xde978` | `[sp-112]` | `0x62` | `sp-28` | `0x0` |
| `bl` at 0x47028 | `(unknown callee) 0x200a0` | `0x3` | `0xf67f0` | `0x772` | `0x0` |
| `bl` at 0x47060 | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x77a` | `0x0` |
| `bl` at 0x47088 | `(unknown callee) 0xde978` | `[sp-112]` | `0x6` | `[0x132240]` | `0x0` |
| `bl` at 0x470b8 | `(unknown callee) 0x200a0` | `0x3` | `0xf67f0` | `0x77e` | `0x0` |
| `bl` at 0x470f0 | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x786` | `0x0` |
| `bl` at 0x471e4 | `(unknown callee) 0xde978` | `[sp-112]` | `0x9` | `sp-28` | `0x0` |
| `bl` at 0x47214 | `(unknown callee) 0x200a0` | `0x3` | `0xf67f0` | `0x78c` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x46db8 | `ret(0x45714) == 0x0` | `ret(0x45714) != 0x0` |
| `beq` at 0x46de4 | `ret(0x45b28) == 0x0` | `ret(0x45b28) != 0x0` |
| `bne` at 0x46e10 | `ret(0xd0814) != 0x0` | `ret(0xd0814) == 0x0` |
| `beq` at 0x46e40 | `ret(0x44c98) == 0x0` | `ret(0x44c98) != 0x0` |
| `beq` at 0x46e6c | `ret(0x43088) == 0x0` | `ret(0x43088) != 0x0` |
| `beq` at 0x46e88 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `beq` at 0x46fa4 | `ret(0xde978) == 0x0` | `ret(0xde978) != 0x0` |
| `beq` at 0x47004 | `ret(0xde978) == 0x0` | `ret(0xde978) != 0x0` |
| `beq` at 0x47044 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `beq` at 0x47094 | `ret(0xde978) == 0x0` | `ret(0xde978) != 0x0` |
| `beq` at 0x470d4 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `beq` at 0x471f0 | `ret(0xde978) == 0x0` | `ret(0xde978) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x47234 | `r0` | `?` |

## `SHUB_Reset_PostRelease`

- `SHUB_Reset_PostRelease`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x472ac to 0x47458.
  Established. The unnamed `0x4182c` call at `0x4734c` carries `[sp-17]:1`, `0x500303a0`, `0x7`, `0x0` (`bl` at 0x4734c);
  its nonzero result takes `0x47368`, while zero falls through (`bne` at 0x47358).
  A later word store writes `ret(0x422ec)` to `sp -0x18` (`str` at 0x47408).
  Post-release register programming meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x472c0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x472c8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x472d4 | load | 4 | `abs 0x4742c` | `-` |
| `ldrb` at 0x472d8 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x472f8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x47304 | load | 4 | `abs 0x47430` | `-` |
| `ldr` at 0x47308 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x47314 | load | 4 | `abs 0x47434` | `-` |
| `str` at 0x47318 | store | 4 | `sp -0x20` | `0xf76dc` |
| `ldrb` at 0x4731c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x47320 | store | 4 | `sp -0x1c` | `[sp-17]:1` |
| `ldr` at 0x47328 | load | 4 | `abs 0x47438` | `-` |
| `ldr` at 0x4732c | load | 4 | `abs 0x4743c` | `-` |
| `ldrb` at 0x47338 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x47340 | load | 4 | `abs 0x47440` | `-` |
| `str` at 0x47360 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x47368 | load | 4 | `abs 0x47430` | `-` |
| `ldr` at 0x4736c | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x47378 | load | 4 | `abs 0x47444` | `-` |
| `str` at 0x4737c | store | 4 | `sp -0x20` | `0xf76fc` |
| `ldrb` at 0x47380 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x47384 | store | 4 | `sp -0x1c` | `[sp-17]:1` |
| `ldr` at 0x4738c | load | 4 | `abs 0x47438` | `-` |
| `ldr` at 0x47390 | load | 4 | `abs 0x47448` | `-` |
| `ldrb` at 0x4739c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x473a4 | load | 4 | `abs 0x4744c` | `-` |
| `str` at 0x473c4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x473cc | load | 4 | `abs 0x47430` | `-` |
| `ldr` at 0x473d0 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x473dc | load | 4 | `abs 0x47450` | `-` |
| `str` at 0x473e0 | store | 4 | `sp -0x20` | `0xf771c` |
| `ldr` at 0x473e8 | load | 4 | `abs 0x47438` | `-` |
| `ldrb` at 0x473f8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x47408 | store | 4 | `sp -0x18` | `ret(0x422ec)` |
| `ldrb` at 0x4740c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x47410 | load | 4 | `abs 0x47454` | `-` |
| `ldr` at 0x4741c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x47428 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x47428 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x47428 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x472d0 | `(unknown callee) 0x428b0` | `0x1` | `r1` | `r2` | `0x0` |
| `bl` at 0x47334 | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x7b5` | `0x0` |
| `bl` at 0x4734c | `(unknown callee) 0x4182c` | `[sp-17]:1` | `0x500303a0` | `0x7` | `0x0` |
| `bl` at 0x47398 | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x7ba` | `0x0` |
| `bl` at 0x473b0 | `(unknown callee) 0x4182c` | `[sp-17]:1` | `0x500305a0` | `0x7` | `0x0` |
| `bl` at 0x473f4 | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x7c0` | `0x0` |
| `bl` at 0x47400 | `(unknown callee) 0x422ec` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x47418 | `(unknown callee) 0x1e958` | `0xf775c` | `[sp-17]:1` | `?` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x47300 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x47310 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `bne` at 0x47358 | `ret(0x4182c) != 0x0` | `ret(0x4182c) == 0x0` |
| `beq` at 0x47374 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `bne` at 0x473bc | `ret(0x4182c) != 0x0` | `ret(0x4182c) == 0x0` |
| `beq` at 0x473d8 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x47428 | `r0` | `[sp-24]` |

## `SHUB_Allow_Resets`

- `SHUB_Allow_Resets`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x42078 to 0x422ec.
  Established. Included word stores assign `0xfffff893` to `sp -0x20` (`str` at 0x420f4, `str` at 0x42124, `str` at 0x42164, `str` at 0x421a8).
  The unnamed `0x418a4` call at `0x42150` carries `[sp-17]:1`, `0x50000010`, `sp-44`, `sp-48` (`bl` at 0x42150);
  its nonzero result takes `0x4216c`, while zero falls through (`bne` at 0x4215c).
  Reset-enable register contract remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4208c | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x42090 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x42098 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x420a0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x420a4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x420b4 | store | 4 | `sp -0x28` | `?` |
| `str` at 0x420bc | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x420c0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x420cc | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x420d4 | load | 4 | `abs 0x422cc` | `-` |
| `ldr` at 0x420dc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x420f0 | load | 4 | `abs 0x422d0` | `-` |
| `str` at 0x420f4 | store | 4 | `sp -0x20` | `0xfffff893` |
| `ldrb` at 0x420fc | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x42104 | load | 4 | `abs 0x422d4` | `-` |
| `ldr` at 0x4210c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x42120 | load | 4 | `abs 0x422d0` | `-` |
| `str` at 0x42124 | store | 4 | `sp -0x20` | `0xfffff893` |
| `ldr` at 0x4212c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x42138 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x42148 | load | 4 | `abs 0x422cc` | `-` |
| `ldr` at 0x42160 | load | 4 | `abs 0x422d0` | `-` |
| `str` at 0x42164 | store | 4 | `sp -0x20` | `0xfffff893` |
| `ldr` at 0x4216c | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x4217c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x4218c | load | 4 | `abs 0x422d4` | `-` |
| `ldr` at 0x421a4 | load | 4 | `abs 0x422d0` | `-` |
| `str` at 0x421a8 | store | 4 | `sp -0x20` | `0xfffff893` |
| `ldr` at 0x421b0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x421c4 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x421cc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x421d4 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x421dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x421e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x421f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42200 | load | 4 | `abs 0x422d8` | `-` |
| `str` at 0x42204 | store | 4 | `sp -0x38` | `0xf6804` |
| `ldr` at 0x4220c | load | 4 | `abs 0x422dc` | `-` |
| `str` at 0x42210 | store | 4 | `sp -0x38` | `0xf6810` |
| `ldr` at 0x42214 | load | 4 | `abs 0x422e0` | `-` |
| `ldr` at 0x42218 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x4221c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x42224 | load | 4 | `abs 0x422e4` | `-` |
| `ldr` at 0x42228 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x42234 | load | 4 | `abs 0x422e0` | `-` |
| `str` at 0x42238 | store | 4 | `sp -0x4c` | `0xf6818` |
| `ldr` at 0x4223c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42248 | load | 4 | `abs 0x422d8` | `-` |
| `str` at 0x4224c | store | 4 | `sp -0x3c` | `0xf6804` |
| `ldr` at 0x42254 | load | 4 | `abs 0x422dc` | `-` |
| `str` at 0x42258 | store | 4 | `sp -0x3c` | `0xf6810` |
| `ldr` at 0x4225c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x42260 | store | 4 | `sp -0x48` | `[sp-60]` |
| `ldr` at 0x42264 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x42268 | store | 4 | `sp -0x44` | `[sp-28]` |
| `ldr` at 0x42270 | load | 4 | `abs 0x422e8` | `-` |
| `ldr` at 0x42280 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x42290 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x42298 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x422a8 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x422b0 | load | 4 | `abs 0x422d0` | `-` |
| `str` at 0x422b4 | store | 4 | `sp -0x40` | `0xfffff893` |
| `ldr` at 0x422b8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x422bc | store | 4 | `sp -0x34` | `[sp-64]` |
| `ldr` at 0x422c0 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x422c8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x422c8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x422c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x420e0 | `(unknown callee) 0x4182c` | `?` | `0x50000010` | `0x0` | `[sp-40]` |
| `bl` at 0x42110 | `(unknown callee) 0x4182c` | `[sp-17]:1` | `0x50002010` | `0x0` | `[sp-40]` |
| `bl` at 0x42150 | `(unknown callee) 0x418a4` | `[sp-17]:1` | `0x50000010` | `sp-44` | `sp-48` |
| `bl` at 0x42194 | `(unknown callee) 0x418a4` | `[sp-17]:1` | `0x50002010` | `sp-44` | `sp-48` |
| `bl` at 0x42220 | `(unknown callee) 0x1e958` | `0xf6818` | `[sp-56]` | `?` | `?` |
| `bl` at 0x4227c | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x1b0` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0x420c8 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0x420ec | `ret(0x4182c) != 0x0` | `ret(0x4182c) == 0x0` |
| `bne` at 0x4211c | `ret(0x4182c) != 0x0` | `ret(0x4182c) == 0x0` |
| `bne` at 0x42134 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bne` at 0x4215c | `ret(0x418a4) != 0x0` | `ret(0x418a4) == 0x0` |
| `bne` at 0x42174 | `[sp-48] != 0x1` | `[sp-48] == 0x1` |
| `bne` at 0x421a0 | `ret(0x418a4) != 0x0` | `ret(0x418a4) == 0x0` |
| `bne` at 0x421b8 | `[sp-48] != 0x1` | `[sp-48] == 0x1` |
| `ble` at 0x421e4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x421f0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x421fc | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x42230 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `beq` at 0x42244 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0x42288 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x422a0 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x422c8 | `r0` | `?` |

## `SHUB_ScanResetNI`

- `SHUB_ScanResetNI`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x428d8 to 0x42a78.
  Established. The byte at absolute `0x132224` is read (`ldrb` at 0x42904).
  The unnamed `0x42488` call at `0x42980` carries `[sp-28]`, `0x0`, `0x1`, unknown (`bl` at 0x42980);
  its zero result takes `0x429a0`, while nonzero falls through (`beq` at 0x42994).
  Scan reset link selection remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x428ec | store | 4 | `sp -0x14` | `0x0` |
| `str` at 0x428f4 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x428fc | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x42900 | load | 4 | `abs 0x42a70` | `-` |
| `ldrb` at 0x42904 | load | 1 | `abs 0x132224` | `-` |
| `ldr` at 0x42908 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x42914 | load | 4 | `abs 0x42a74` | `-` |
| `ldr` at 0x42918 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x42938 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x42948 | load | 4 | `abs 0x42a74` | `-` |
| `ldr` at 0x4294c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x4296c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x42974 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x42988 | store | 4 | `sp -0x18` | `ret(0x42488)` |
| `ldr` at 0x4298c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x42998 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4299c | store | 4 | `sp -0x14` | `ret(0x42488)` |
| `ldr` at 0x429a0 | load | 4 | `abs 0x42a74` | `-` |
| `ldr` at 0x429a4 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x429c4 | load | 2 | `(unknown)` | `-` |
| `str` at 0x429d8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x429dc | load | 4 | `abs 0x42a74` | `-` |
| `ldr` at 0x429e0 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x42a00 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x42a08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x42a24 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x42a34 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x42a38 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x42a3c | store | 4 | `sp -0x18` | `[sp-32]` |
| `ldr` at 0x42a48 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x42a4c | store | 4 | `sp -0x14` | `[sp-32]` |
| `ldr` at 0x42a50 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x42a58 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x42a60 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x42a6c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x42a6c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x42a6c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x42980 | `(unknown callee) 0x42488` | `[sp-28]` | `0x0` | `0x1` | `?` |
| `bl` at 0x42a14 | `(unknown callee) 0x42488` | `[sp-28]` | `0x1` | `0x1` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x42910 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x42944 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x42994 | `ret(0x42488) == 0x0` | `ret(0x42488) != 0x0` |
| `bne` at 0x429d0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x42a20 | `ret(0x42488) == 0x0` | `ret(0x42488) != 0x0` |
| `bne` at 0x42a2c | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `beq` at 0x42a44 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x42a6c | `r0` | `?` |

## `SHUB_ScanResetLLP`

- `SHUB_ScanResetLLP`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x42a78 to 0x42ed0.
  Established. A zero `[0x29737c]` takes `0x42ab4`, while nonzero falls through (`beq` at 0x42a94).
  Included word stores write `0xfffff88c` to `sp -0x20` (`str` at 0x42af4, `str` at 0x42b20).
  The unnamed `0x4182c` call at `0x42c90` carries unknown, `0x50000008`, `0x0`, `0x1` (`bl` at 0x42c90).
  LLP reset register meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x42a88 | load | 4 | `abs 0x42e80` | `-` |
| `ldr` at 0x42a8c | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x42a98 | load | 4 | `abs 0x42e84` | `-` |
| `str` at 0x42a9c | store | 4 | `sp -0x2c` | `0xf69d0` |
| `ldr` at 0x42aa4 | load | 4 | `abs 0x42e88` | `-` |
| `ldr` at 0x42aa8 | load | 4 | `abs 0x42e8c` | `-` |
| `str` at 0x42ab8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x42abc | load | 4 | `abs 0x42e90` | `-` |
| `ldrb` at 0x42ac0 | load | 1 | `abs 0x132224` | `-` |
| `ldr` at 0x42ac4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x42ad0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x42ad4 | load | 4 | `abs 0x42e94` | `-` |
| `ldr` at 0x42af0 | load | 4 | `abs 0x42e98` | `-` |
| `str` at 0x42af4 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x42afc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x42b00 | load | 4 | `abs 0x42e9c` | `-` |
| `ldr` at 0x42b1c | load | 4 | `abs 0x42e98` | `-` |
| `str` at 0x42b20 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x42b28 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x42b30 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x42b38 | load | 4 | `abs 0x42ea0` | `-` |
| `str` at 0x42b44 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x42b48 | load | 4 | `abs 0x42e90` | `-` |
| `ldrb` at 0x42b4c | load | 1 | `abs 0x132224` | `-` |
| `ldr` at 0x42b50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x42b64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x42b68 | load | 4 | `abs 0x42e94` | `-` |
| `ldr` at 0x42b84 | load | 4 | `abs 0x42e98` | `-` |
| `str` at 0x42b88 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x42b90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x42ba4 | load | 4 | `abs 0x42ea4` | `-` |
| `str` at 0x42ba8 | store | 4 | `sp -0x2c` | `0xf69e4` |
| `ldr` at 0x42bac | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x42bb0 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x42bb4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x42bb8 | store | 4 | `sp -0x24` | `[sp-20]` |
| `ldr` at 0x42bc0 | load | 4 | `abs 0x42e88` | `-` |
| `ldr` at 0x42bc4 | load | 4 | `abs 0x42ea8` | `-` |
| `ldr` at 0x42be0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x42be4 | load | 4 | `abs 0x42e9c` | `-` |
| `ldr` at 0x42c00 | load | 4 | `abs 0x42e98` | `-` |
| `str` at 0x42c04 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x42c0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x42c20 | load | 4 | `abs 0x42eac` | `-` |
| `str` at 0x42c24 | store | 4 | `sp -0x2c` | `0xf6a14` |
| `ldr` at 0x42c28 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x42c2c | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x42c30 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x42c34 | store | 4 | `sp -0x24` | `[sp-20]` |
| `ldr` at 0x42c3c | load | 4 | `abs 0x42e88` | `-` |
| `ldr` at 0x42c40 | load | 4 | `abs 0x42eb0` | `-` |
| `ldr` at 0x42c54 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x42c5c | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x42c68 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x42c6c | load | 4 | `abs 0x42e90` | `-` |
| `ldrb` at 0x42c70 | load | 1 | `abs 0x132224` | `-` |
| `ldr` at 0x42c74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x42c80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x42ca0 | load | 4 | `abs 0x42e98` | `-` |
| `str` at 0x42ca4 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x42cac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x42cb0 | load | 4 | `abs 0x42eb4` | `-` |
| `ldr` at 0x42ccc | load | 4 | `abs 0x42e98` | `-` |
| `str` at 0x42cd0 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x42cd8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x42ce0 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x42cf4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x42cf8 | load | 4 | `abs 0x42e90` | `-` |
| `ldrb` at 0x42cfc | load | 1 | `abs 0x132224` | `-` |
| `ldr` at 0x42d00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x42d14 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x42d34 | load | 4 | `abs 0x42e98` | `-` |
| `str` at 0x42d38 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x42d40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x42d4c | load | 4 | `abs 0x42eb8` | `-` |
| `ldr` at 0x42d50 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x42d70 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x42d80 | load | 4 | `abs 0x42ebc` | `-` |
| `str` at 0x42d84 | store | 4 | `sp -0x2c` | `0xf6a44` |
| `ldr` at 0x42d88 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x42d8c | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x42d90 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x42d94 | store | 4 | `sp -0x24` | `[sp-24]` |
| `ldr` at 0x42d9c | load | 4 | `abs 0x42e88` | `-` |
| `ldr` at 0x42da0 | load | 4 | `abs 0x42ec0` | `-` |
| `ldr` at 0x42dbc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x42dc0 | load | 4 | `abs 0x42ec4` | `-` |
| `ldr` at 0x42ddc | load | 4 | `abs 0x42e98` | `-` |
| `str` at 0x42de0 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x42de8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x42df4 | load | 4 | `abs 0x42eb8` | `-` |
| `ldr` at 0x42df8 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x42e18 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x42e28 | load | 4 | `abs 0x42ec8` | `-` |
| `str` at 0x42e2c | store | 4 | `sp -0x2c` | `0xf6a74` |
| `ldr` at 0x42e30 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x42e34 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x42e38 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x42e3c | store | 4 | `sp -0x24` | `[sp-24]` |
| `ldr` at 0x42e44 | load | 4 | `abs 0x42e88` | `-` |
| `ldr` at 0x42e48 | load | 4 | `abs 0x42ecc` | `-` |
| `ldr` at 0x42e5c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x42e64 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x42e70 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x42e74 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x42e7c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x42e7c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x42e7c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x42ab0 | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x289` | `0x0` |
| `bl` at 0x42ae0 | `(unknown callee) 0x4182c` | `?` | `0x500303b0` | `0x1` | `?` |
| `bl` at 0x42b0c | `(unknown callee) 0x4182c` | `[sp-28]` | `0x500305b0` | `0x1` | `?` |
| `bl` at 0x42b3c | `(unknown callee) 0x428b0` | `0x1c2` | `?` | `[0x132224]:1` | `?` |
| `bl` at 0x42b74 | `(unknown callee) 0x418a4` | `?` | `0x500303b0` | `sp-20` | `sp-24` |
| `bl` at 0x42bcc | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x29b` | `0x0` |
| `bl` at 0x42bd4 | `(unknown callee) 0x428b0` | `0xa` | `?` | `?` | `?` |
| `bl` at 0x42bf0 | `(unknown callee) 0x418a4` | `[sp-28]` | `0x500305b0` | `sp-20` | `sp-24` |
| `bl` at 0x42c48 | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x2a2` | `0x0` |
| `bl` at 0x42c50 | `(unknown callee) 0x428b0` | `0xa` | `?` | `?` | `?` |
| `bl` at 0x42c90 | `(unknown callee) 0x4182c` | `?` | `0x50000008` | `0x0` | `0x1` |
| `bl` at 0x42cbc | `(unknown callee) 0x4182c` | `[sp-28]` | `0x50002008` | `0x0` | `0x1` |
| `bl` at 0x42cec | `(unknown callee) 0x428b0` | `0xa` | `?` | `[0x132224]:1` | `?` |
| `bl` at 0x42d24 | `(unknown callee) 0x418a4` | `?` | `0x50000000` | `sp-20` | `sp-24` |
| `bl` at 0x42da8 | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x2bb` | `0x0` |
| `bl` at 0x42db0 | `(unknown callee) 0x428b0` | `0xa` | `?` | `?` | `?` |
| `bl` at 0x42dcc | `(unknown callee) 0x418a4` | `[sp-28]` | `0x50002000` | `sp-20` | `sp-24` |
| `bl` at 0x42e50 | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x2c2` | `0x0` |
| `bl` at 0x42e58 | `(unknown callee) 0x428b0` | `0xa` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x42a94 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `ble` at 0x42acc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x42aec | `ret(0x4182c) != 0x0` | `ret(0x4182c) == 0x0` |
| `bne` at 0x42b18 | `ret(0x4182c) != 0x0` | `ret(0x4182c) == 0x0` |
| `ble` at 0x42b58 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x42b80 | `ret(0x418a4) != 0x0` | `ret(0x418a4) == 0x0` |
| `beq` at 0x42ba0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x42bfc | `ret(0x418a4) != 0x0` | `ret(0x418a4) == 0x0` |
| `beq` at 0x42c1c | `? == 0x0` | `? != 0x0` |
| `ble` at 0x42c7c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x42c9c | `ret(0x4182c) != 0x0` | `ret(0x4182c) == 0x0` |
| `bne` at 0x42cc8 | `ret(0x4182c) != 0x0` | `ret(0x4182c) == 0x0` |
| `ble` at 0x42d08 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x42d30 | `ret(0x418a4) != 0x0` | `ret(0x418a4) == 0x0` |
| `beq` at 0x42d48 | `[sp-24] == 0x9` | `[sp-24] != 0x9` |
| `beq` at 0x42d7c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x42dd8 | `ret(0x418a4) != 0x0` | `ret(0x418a4) == 0x0` |
| `beq` at 0x42df0 | `[sp-24] == 0x9` | `[sp-24] != 0x9` |
| `beq` at 0x42e24 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x42e7c | `r0` | `?` |

## `SHUB_Reset_MCI`

- `SHUB_Reset_MCI`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x41bb0 to 0x41bd0.
  Established. An absolute word store clears `0x132244` (`str` at 0x41bc4).
  The return record preserves entry `r0` (`ldm` at 0x41bc8).
  The reset-related meaning of this stored word remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x41bbc | load | 4 | `abs 0x41bcc` | `-` |
| `str` at 0x41bc4 | store | 4 | `abs 0x132244` | `0x0` |
| `ldm` at 0x41bc8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x41bc8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x41bc8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x41bc8 | `r0` | `r0` |

## `SHUB_Reset_FSBCfg`

- `SHUB_Reset_FSBCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4201c to 0x4203c.
  Established. The word at absolute `0x13224c` is assigned `0x0` (`str` at 0x42030).
  Entry `r0` remains the reported return value (`ldm` at 0x42034).
  FSB configuration reset behavior beyond this assignment is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x42028 | load | 4 | `abs 0x42038` | `-` |
| `str` at 0x42030 | store | 4 | `abs 0x13224c` | `0x0` |
| `ldm` at 0x42034 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x42034 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x42034 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x42034 | `r0` | `r0` |

## `SHUB_Reset_WRT90`

- `SHUB_Reset_WRT90`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x41cb8 to 0x41cd8.
  Established. The included absolute word assignment writes `0x0` to `0x132248` (`str` at 0x41ccc).
  The return record gives entry `r0` (`ldm` at 0x41cd0).
  This body does not establish the pointee meaning, which remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x41cc4 | load | 4 | `abs 0x41cd4` | `-` |
| `str` at 0x41ccc | store | 4 | `abs 0x132248` | `0x0` |
| `ldm` at 0x41cd0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x41cd0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x41cd0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x41cd0 | `r0` | `r0` |

## `SHUB_NICfg_Override`

- `SHUB_NICfg_Override`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4191c to 0x4196c.
  Established. Entry `r0` and `r1` are saved in words at `sp -0x14` and `sp -0x18` (`str` at 0x4192c, `str` at 0x41930).
  A later word load has an unknown effective target (`ldr` at 0x41958).
  The integer return value is unknown (`ldm` at 0x41964).
  Configuration override table selection remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4192c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x41930 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x41934 | load | 4 | `abs 0x41968` | `-` |
| `ldr` at 0x41938 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4194c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41958 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x41964 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x41964 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x41964 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x41964 | `r0` | `?` |
