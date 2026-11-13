# ARM board-operation setup paths in 1.44.0

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
Computed transfers in brickInit and brickInitBrickOps leave unresolved destinations.
Their tables and returns cover known edges only. Undecodable byte ranges remain visible
in the fact sheets. Stored numeric addresses do not establish calls to those addresses.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/board-operations-setup-facts.txt) retain all tool sections.

## `Athena_BrickOpsInit`

- `Athena_BrickOpsInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2ebac to 0x2ed34.
  Established. The word through entry `r0 +0x0` receives `0x2e814` (`str` at 0x2ebd0).
  Other included word stores write `0xff` through `[sp-20] +0xa0` and `[sp-20] +0xac` (`str` at 0x2ec00, `str` at 0x2ec24).
  The byte through `[sp-20] +0xc` is cleared (`strb` at 0x2ebe8).
  The board-operation field roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2ebbc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2ebc0 | load | 4 | `abs 0x2ed14` | `-` |
| `str` at 0x2ebc4 | store | 4 | `sp -0x18` | `0x12d642` |
| `ldr` at 0x2ebc8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2ebcc | load | 4 | `abs 0x2ed18` | `-` |
| `str` at 0x2ebd0 | store | 4 | `r0 +0x0` | `0x2e814` |
| `ldr` at 0x2ebd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2ebd8 | load | 4 | `abs 0x2ed1c` | `-` |
| `str` at 0x2ebdc | store | 4 | `[sp-20] +0x4` | `0x2e600` |
| `ldr` at 0x2ebe0 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x2ebe8 | store | 1 | `[sp-20] +0xc` | `0x0` |
| `ldr` at 0x2ebec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ebf4 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x2ebf8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ec00 | store | 4 | `[sp-20] +0xa0` | `0xff` |
| `ldr` at 0x2ec04 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ec0c | store | 4 | `[sp-20] +0xa4` | `0xff` |
| `ldr` at 0x2ec10 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ec18 | store | 4 | `[sp-20] +0xa8` | `0xff` |
| `ldr` at 0x2ec1c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ec24 | store | 4 | `[sp-20] +0xac` | `0xff` |
| `ldr` at 0x2ec28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2ec2c | load | 4 | `abs 0x2ed20` | `-` |
| `str` at 0x2ec30 | store | 4 | `[sp-20] +0x14` | `0xf3804` |
| `ldr` at 0x2ec34 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ec3c | store | 4 | `[sp-20] +0x88` | `0x0` |
| `ldr` at 0x2ec40 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ec48 | store | 4 | `[sp-20] +0x90` | `0x0` |
| `ldr` at 0x2ec4c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ec54 | store | 4 | `[sp-20] +0x94` | `0x0` |
| `ldr` at 0x2ec58 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ec60 | store | 4 | `[sp-20] +0x98` | `0x0` |
| `ldr` at 0x2ec64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2ec68 | load | 4 | `abs 0x2ed24` | `-` |
| `str` at 0x2ec6c | store | 4 | `[sp-20] +0xc4` | `0x4b56c` |
| `ldr` at 0x2ec70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2ec74 | load | 4 | `abs 0x2ed28` | `-` |
| `str` at 0x2ec78 | store | 4 | `[sp-20] +0xc8` | `0x4b778` |
| `ldr` at 0x2ec7c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ec84 | store | 4 | `[sp-20] +0xd0` | `0x0` |
| `ldr` at 0x2ec88 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ec90 | store | 4 | `[sp-20] +0xd4` | `0x0` |
| `ldr` at 0x2ec94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2ec98 | load | 4 | `abs 0x2ed2c` | `-` |
| `str` at 0x2ec9c | store | 4 | `[sp-20] +0xec` | `0x2e028` |
| `ldr` at 0x2eca0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2eca4 | load | 4 | `abs 0x2ed30` | `-` |
| `str` at 0x2eca8 | store | 4 | `[sp-20] +0xf0` | `0x4ba7c` |
| `ldr` at 0x2ecac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ecb4 | store | 4 | `[sp-20] +0x68` | `0x2d` |
| `ldr` at 0x2ecb8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ecc0 | store | 4 | `[sp-20] +0x6c` | `0x32` |
| `ldr` at 0x2ecc4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2eccc | store | 4 | `[sp-20] +0x70` | `0x37` |
| `ldr` at 0x2ecd0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ecd8 | store | 4 | `[sp-20] +0x74` | `0x3c` |
| `ldr` at 0x2ecdc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ece4 | store | 4 | `[sp-20] +0x78` | `0x2b` |
| `ldr` at 0x2ece8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ecf0 | store | 4 | `[sp-20] +0x18` | `0x0` |
| `ldr` at 0x2ecf4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ecfc | store | 4 | `[sp-20] +0x1c` | `0x0` |
| `ldr` at 0x2ed00 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ed08 | store | 4 | `[sp-20] +0x20` | `0x0` |
| `ldm` at 0x2ed10 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x2ed10 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x2ed10 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x2ed10 | `r0` | `r0` |

## `OpusTio_BrickOpsInit`

- `OpusTio_BrickOpsInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2d920 to 0x2dad0.
  Established. The byte through `[sp-20] +0xc` receives `0x52` (`strb` at 0x2d95c).
  Included word assignments through `[sp-20] +0x88` and `[sp-20] +0x8c` write `0x2` and `0x1` (`str` at 0x2da10, `str` at 0x2da04).
  The word through `[sp-20] +0xf0` receives `0x2cd04` (`str` at 0x2da88).
  Operation selection meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2d930 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2d934 | load | 4 | `abs 0x2da94` | `-` |
| `str` at 0x2d938 | store | 4 | `sp -0x18` | `0x12d5ec` |
| `ldr` at 0x2d93c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d940 | load | 4 | `abs 0x2da98` | `-` |
| `str` at 0x2d944 | store | 4 | `r0 +0x0` | `0x2d8c8` |
| `ldr` at 0x2d948 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d94c | load | 4 | `abs 0x2da9c` | `-` |
| `str` at 0x2d950 | store | 4 | `[sp-20] +0x4` | `0x2d61c` |
| `ldr` at 0x2d954 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x2d95c | store | 1 | `[sp-20] +0xc` | `0x52` |
| `ldr` at 0x2d960 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d968 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x2d96c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d974 | store | 4 | `[sp-20] +0xa0` | `0x11` |
| `ldr` at 0x2d978 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d980 | store | 4 | `[sp-20] +0xa4` | `0xff` |
| `ldr` at 0x2d984 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d98c | store | 4 | `[sp-20] +0xa8` | `0xff` |
| `ldr` at 0x2d990 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d998 | store | 4 | `[sp-20] +0xac` | `0xff` |
| `ldr` at 0x2d99c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d9a0 | load | 4 | `abs 0x2daa0` | `-` |
| `str` at 0x2d9a4 | store | 4 | `[sp-20] +0x14` | `0xf2ce8` |
| `ldr` at 0x2d9a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d9ac | load | 4 | `abs 0x2daa4` | `-` |
| `str` at 0x2d9b0 | store | 4 | `[sp-20] +0xb0` | `0xfaf70` |
| `ldr` at 0x2d9b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d9b8 | load | 4 | `abs 0x2daa8` | `-` |
| `str` at 0x2d9bc | store | 4 | `[sp-20] +0x7c` | `0x2c828` |
| `ldr` at 0x2d9c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d9c8 | store | 4 | `[sp-20] +0x68` | `0x1e` |
| `ldr` at 0x2d9cc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d9d4 | store | 4 | `[sp-20] +0x6c` | `0x2d` |
| `ldr` at 0x2d9d8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d9e0 | store | 4 | `[sp-20] +0x70` | `0x32` |
| `ldr` at 0x2d9e4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d9ec | store | 4 | `[sp-20] +0x74` | `0x37` |
| `ldr` at 0x2d9f0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d9f8 | store | 4 | `[sp-20] +0x78` | `0x17` |
| `ldr` at 0x2d9fc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2da04 | store | 4 | `[sp-20] +0x8c` | `0x1` |
| `ldr` at 0x2da08 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2da10 | store | 4 | `[sp-20] +0x88` | `0x2` |
| `ldr` at 0x2da14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2da18 | load | 4 | `abs 0x2daac` | `-` |
| `str` at 0x2da1c | store | 4 | `[sp-20] +0x90` | `0x56824` |
| `ldr` at 0x2da20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2da24 | load | 4 | `abs 0x2dab0` | `-` |
| `str` at 0x2da28 | store | 4 | `[sp-20] +0x94` | `0x56914` |
| `ldr` at 0x2da2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2da30 | load | 4 | `abs 0x2dab4` | `-` |
| `str` at 0x2da34 | store | 4 | `[sp-20] +0x98` | `0x568a0` |
| `ldr` at 0x2da38 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2da40 | store | 4 | `[sp-20] +0xb4` | `0x1` |
| `ldr` at 0x2da44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2da48 | load | 4 | `abs 0x2dab8` | `-` |
| `str` at 0x2da4c | store | 4 | `[sp-20] +0xc4` | `0x2c51c` |
| `ldr` at 0x2da50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2da54 | load | 4 | `abs 0x2dabc` | `-` |
| `str` at 0x2da58 | store | 4 | `[sp-20] +0xc8` | `0x2c5dc` |
| `ldr` at 0x2da5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2da60 | load | 4 | `abs 0x2dac0` | `-` |
| `str` at 0x2da64 | store | 4 | `[sp-20] +0xd0` | `0x2c698` |
| `ldr` at 0x2da68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2da6c | load | 4 | `abs 0x2dac4` | `-` |
| `str` at 0x2da70 | store | 4 | `[sp-20] +0xd4` | `0x2c6ec` |
| `ldr` at 0x2da74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2da78 | load | 4 | `abs 0x2dac8` | `-` |
| `str` at 0x2da7c | store | 4 | `[sp-20] +0xec` | `0x2cdf4` |
| `ldr` at 0x2da80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2da84 | load | 4 | `abs 0x2dacc` | `-` |
| `str` at 0x2da88 | store | 4 | `[sp-20] +0xf0` | `0x2cd04` |
| `ldm` at 0x2da90 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x2da90 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x2da90 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x2da90 | `r0` | `r0` |

## `NL4S_BrickOpsInit`

- `NL4S_BrickOpsInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2b500 to 0x2b530.
  Established. The word at `sp -0x18` receives `0x12d53a` (`str` at 0x2b518).
  The unnamed `0x2b37c` call carries entry `r0`, entry `r1`, entry `r2`, `0x12d53a` (`bl` at 0x2b520);
  its result reaches the included `r0` return record (`ldm` at 0x2b528).
  Board setup behavior in the callee remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2b510 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2b514 | load | 4 | `abs 0x2b52c` | `-` |
| `str` at 0x2b518 | store | 4 | `sp -0x18` | `0x12d53a` |
| `ldr` at 0x2b51c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x2b528 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x2b528 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x2b528 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2b520 | `(unknown callee) 0x2b37c` | `r0` | `r1` | `r2` | `0x12d53a` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x2b528 | `r0` | `ret(0x2b37c)` |

## `NL4R_BrickOpsInit`

- `NL4R_BrickOpsInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2b4c0 to 0x2b500.
  Established. The word through entry `r0 +0x0` receives `0x2b248` (`str` at 0x2b4e4).
  The unnamed `0x2b37c` call carries `[sp-20]`, entry `r1`, entry `r0`, `0x2b248` (`bl` at 0x2b4ec).
  The included return record gives `ret(0x2b37c)` (`ldm` at 0x2b4f4).
  Board-operation wiring remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2b4d0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2b4d4 | load | 4 | `abs 0x2b4f8` | `-` |
| `str` at 0x2b4d8 | store | 4 | `sp -0x18` | `0x12d534` |
| `ldr` at 0x2b4dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2b4e0 | load | 4 | `abs 0x2b4fc` | `-` |
| `str` at 0x2b4e4 | store | 4 | `r0 +0x0` | `0x2b248` |
| `ldr` at 0x2b4e8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x2b4f4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x2b4f4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x2b4f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2b4ec | `(unknown callee) 0x2b37c` | `[sp-20]` | `r1` | `r0` | `0x2b248` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x2b4f4 | `r0` | `ret(0x2b37c)` |

## `brickInit`

- `brickInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x48194 to 0x488cc.
  Established. An absolute word store writes `0x1` to `0x297578` (`str` at 0x481bc).
  A byte store writes `0x2` to `0x132419` (`strb` at 0x481c8).
  The condition `[0x12d704]:1 != 0x3f` takes `0x4884c`, while equality falls through (`bne` at 0x48694).
  Board identification beyond the unresolved dispatch remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x481a8 | store | 4 | `sp -0x94` | `0x1` |
| `ldr` at 0x481b4 | load | 4 | `abs 0x4887c` | `-` |
| `str` at 0x481bc | store | 4 | `abs 0x297578` | `0x1` |
| `ldr` at 0x481c0 | load | 4 | `abs 0x48880` | `-` |
| `strb` at 0x481c8 | store | 1 | `abs 0x132419` | `0x2` |
| `strb` at 0x481d0 | store | 1 | `sp -0x8e` | `?` |
| `ldr` at 0x481f4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x481f8 | store | 4 | `sp -0x8c` | `[sp-60]` |
| `ldr` at 0x48204 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x48208 | load | 4 | `abs 0x48884` | `-` |
| `strb` at 0x48224 | store | 1 | `sp -0x8e` | `?` |
| `str` at 0x4822c | store | 4 | `sp -0x94` | `0x0` |
| `ldr` at 0x48230 | load | 4 | `abs 0x48888` | `-` |
| `strb` at 0x48238 | store | 1 | `abs 0x132418` | `0x40` |
| `ldr` at 0x48240 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x48244 | load | 4 | `abs 0x4888c` | `-` |
| `ldr` at 0x4825c | load | 4 | `abs 0x48890` | `-` |
| `strb` at 0x48264 | store | 1 | `abs 0x12d704` | `0x4c` |
| `ldr` at 0x48268 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x4826c | load | 4 | `abs 0x48894` | `-` |
| `ldr` at 0x48284 | load | 4 | `abs 0x48888` | `-` |
| `strb` at 0x4828c | store | 1 | `abs 0x132418` | `0x7` |
| `ldr` at 0x48294 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x48298 | load | 4 | `abs 0x48898` | `-` |
| `strb` at 0x482b4 | store | 1 | `sp -0x8e` | `?` |
| `ldr` at 0x482bc | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x482c0 | load | 4 | `abs 0x4889c` | `-` |
| `strb` at 0x482dc | store | 1 | `sp -0x8e` | `?` |
| `ldr` at 0x482ec | load | 4 | `abs 0x48890` | `-` |
| `ldrb` at 0x482f0 | load | 1 | `abs 0x12d704` | `-` |
| `ldrb` at 0x482fc | load | 1 | `sp -0x8e` | `-` |
| `strb` at 0x48300 | store | 1 | `sp -0x8d` | `([sp-142]:1 & 0xff)` |
| `strb` at 0x48308 | store | 1 | `sp -0x8e` | `?` |
| `ldrb` at 0x4830c | load | 1 | `sp -0x8d` | `-` |
| `ldr` at 0x4832c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x48330 | store | 4 | `sp -0x8c` | `[sp-60]` |
| `strb` at 0x48340 | store | 1 | `sp -0x95` | `?` |
| `ldr` at 0x48344 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x48348 | load | 4 | `abs 0x488a0` | `-` |
| `ldr` at 0x48360 | load | 4 | `abs 0x48890` | `-` |
| `strb` at 0x48368 | store | 1 | `abs 0x12d704` | `0x44` |
| `ldr` at 0x48370 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x48374 | load | 4 | `abs 0x488a4` | `-` |
| `ldr` at 0x4838c | load | 4 | `abs 0x48890` | `-` |
| `strb` at 0x48394 | store | 1 | `abs 0x12d704` | `0x40` |
| `ldr` at 0x4839c | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x483a0 | load | 4 | `abs 0x488a8` | `-` |
| `ldr` at 0x483b8 | load | 4 | `abs 0x48890` | `-` |
| `strb` at 0x483c0 | store | 1 | `abs 0x12d704` | `0x42` |
| `ldr` at 0x483c8 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x483cc | load | 4 | `abs 0x488ac` | `-` |
| `ldr` at 0x483e4 | load | 4 | `abs 0x48890` | `-` |
| `strb` at 0x483ec | store | 1 | `abs 0x12d704` | `0x46` |
| `ldr` at 0x483f4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x483f8 | load | 4 | `abs 0x488b0` | `-` |
| `ldr` at 0x48410 | load | 4 | `abs 0x48890` | `-` |
| `strb` at 0x48418 | store | 1 | `abs 0x12d704` | `0x53` |
| `ldr` at 0x48420 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x48424 | load | 4 | `abs 0x488b4` | `-` |
| `ldr` at 0x4843c | load | 4 | `abs 0x48890` | `-` |
| `strb` at 0x48444 | store | 1 | `abs 0x12d704` | `0x4d` |
| `ldr` at 0x48454 | load | 4 | `abs 0x48890` | `-` |
| `ldrb` at 0x48458 | load | 1 | `abs 0x12d704` | `-` |
| `ldrb` at 0x48464 | load | 1 | `sp -0x8e` | `-` |
| `strb` at 0x48468 | store | 1 | `sp -0x8d` | `([sp-142]:1 & 0xff)` |
| `strb` at 0x48470 | store | 1 | `sp -0x8e` | `?` |
| `ldrb` at 0x48474 | load | 1 | `sp -0x8d` | `-` |
| `ldr` at 0x48494 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x48498 | store | 4 | `sp -0x8c` | `[sp-60]` |
| `ldr` at 0x484a4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x484a8 | load | 4 | `abs 0x488b8` | `-` |
| `ldr` at 0x484c0 | load | 4 | `abs 0x48890` | `-` |
| `strb` at 0x484c8 | store | 1 | `abs 0x12d704` | `0x5e` |
| `ldr` at 0x484d0 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x484d4 | load | 4 | `abs 0x488bc` | `-` |
| `ldr` at 0x484ec | load | 4 | `abs 0x48890` | `-` |
| `strb` at 0x484f4 | store | 1 | `abs 0x12d704` | `0x35` |
| `ldr` at 0x48504 | load | 4 | `abs 0x48890` | `-` |
| `ldrb` at 0x48508 | load | 1 | `abs 0x12d704` | `-` |
| `ldrb` at 0x48514 | load | 1 | `sp -0x8e` | `-` |
| `strb` at 0x48518 | store | 1 | `sp -0x8d` | `([sp-142]:1 & 0xff)` |
| `strb` at 0x48520 | store | 1 | `sp -0x8e` | `?` |
| `ldrb` at 0x48524 | load | 1 | `sp -0x8d` | `-` |
| `ldr` at 0x48544 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x48548 | store | 4 | `sp -0x8c` | `[sp-60]` |
| `ldr` at 0x48554 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x48558 | load | 4 | `abs 0x488c0` | `-` |
| `ldr` at 0x48570 | load | 4 | `abs 0x48890` | `-` |
| `strb` at 0x48578 | store | 1 | `abs 0x12d704` | `0x4a` |
| `ldr` at 0x48588 | load | 4 | `abs 0x48890` | `-` |
| `ldrb` at 0x4858c | load | 1 | `abs 0x12d704` | `-` |
| `ldrb` at 0x48598 | load | 1 | `sp -0x8e` | `-` |
| `strb` at 0x4859c | store | 1 | `sp -0x8d` | `([sp-142]:1 & 0xff)` |
| `strb` at 0x485a4 | store | 1 | `sp -0x8e` | `?` |
| `ldrb` at 0x485a8 | load | 1 | `sp -0x8d` | `-` |
| `ldr` at 0x485c8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x485cc | store | 4 | `sp -0x8c` | `[sp-60]` |
| `ldr` at 0x485d8 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x485dc | load | 4 | `abs 0x488c0` | `-` |
| `ldr` at 0x485f4 | load | 4 | `abs 0x48890` | `-` |
| `strb` at 0x485fc | store | 1 | `abs 0x12d704` | `0x4a` |
| `ldr` at 0x4860c | load | 4 | `abs 0x48890` | `-` |
| `ldrb` at 0x48610 | load | 1 | `abs 0x12d704` | `-` |
| `ldrb` at 0x4861c | load | 1 | `sp -0x8e` | `-` |
| `strb` at 0x48620 | store | 1 | `sp -0x8d` | `([sp-142]:1 & 0xff)` |
| `ldrb` at 0x48624 | load | 1 | `sp -0x8d` | `-` |
| `ldr` at 0x48644 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x48648 | store | 4 | `sp -0x8c` | `[sp-60]` |
| `ldr` at 0x48654 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x48658 | load | 4 | `abs 0x488c4` | `-` |
| `ldr` at 0x48670 | load | 4 | `abs 0x48890` | `-` |
| `strb` at 0x48678 | store | 1 | `abs 0x12d704` | `0x2b` |
| `ldr` at 0x48688 | load | 4 | `abs 0x48890` | `-` |
| `ldrb` at 0x4868c | load | 1 | `abs 0x12d704` | `-` |
| `ldr` at 0x48698 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x486a4 | load | 4 | `abs 0x4887c` | `-` |
| `str` at 0x486ac | store | 4 | `abs 0x297578` | `0x2` |
| `strb` at 0x486bc | store | 1 | `sp -0x95` | `(ret(0x1d444) & 0xff)` |
| `ldrb` at 0x486c0 | load | 1 | `sp -0x95` | `-` |
| `ldrls` at 0x486d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4884c | load | 4 | `abs 0x48890` | `-` |
| `ldrb` at 0x48850 | load | 1 | `abs 0x12d704` | `-` |
| `ldr` at 0x48860 | load | 4 | `abs 0x48890` | `-` |
| `ldrb` at 0x48864 | load | 1 | `abs 0x12d704` | `-` |
| `ldr` at 0x48868 | load | 4 | `abs 0x488c8` | `-` |
| `ldm` at 0x48878 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x48878 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x48878 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x481b0 | `(unknown callee) 0xbaaa0` | `0x6` | `r1` | `r2` | `0x1` |
| `bl` at 0x481e4 | `(unknown callee) 0xc541c` | `0x0` | `0xff` | `sp-136` | `sp-136` |
| `bl` at 0x48210 | `(unknown callee) 0x1955c` | `[sp-60]` | `0xf7994` | `0x9` | `[sp-60]` |
| `bl` at 0x4824c | `(unknown callee) 0x1955c` | `[sp-140]` | `0xf79a0` | `0xd` | `ret(0x1955c)` |
| `bl` at 0x48274 | `(unknown callee) 0x1955c` | `[sp-52]` | `0xf79b0` | `0xc` | `0x4c` |
| `bl` at 0x482a0 | `(unknown callee) 0x1955c` | `[sp-140]` | `0xf79c0` | `0x8` | `ret(0x1955c)` |
| `bl` at 0x482c8 | `(unknown callee) 0x1955c` | `[sp-140]` | `0xf79cc` | `0xd` | `ret(0x1955c)` |
| `bl` at 0x482e8 | `(unknown callee) 0xc554c` | `sp-136` | `?` | `?` | `sp-136` |
| `bl` at 0x4831c | `(unknown callee) 0xc541c` | `0x0` | `[sp-141]:1` | `sp-136` | `[sp-141]:1` |
| `bl` at 0x48350 | `(unknown callee) 0x1955c` | `[sp-140]` | `0xf79dc` | `0x7` | `?` |
| `bl` at 0x4837c | `(unknown callee) 0x1955c` | `[sp-140]` | `0xf79e4` | `0xa` | `ret(0x1955c)` |
| `bl` at 0x483a8 | `(unknown callee) 0x1955c` | `[sp-140]` | `0xf79f0` | `0xd` | `ret(0x1955c)` |
| `bl` at 0x483d4 | `(unknown callee) 0x1955c` | `[sp-140]` | `0xf7a00` | `0xe` | `ret(0x1955c)` |
| `bl` at 0x48400 | `(unknown callee) 0x1955c` | `[sp-140]` | `0xf7a10` | `0x8` | `ret(0x1955c)` |
| `bl` at 0x4842c | `(unknown callee) 0x1955c` | `[sp-140]` | `0xf7a1c` | `0x4` | `ret(0x1955c)` |
| `bl` at 0x48450 | `(unknown callee) 0xc554c` | `sp-136` | `?` | `?` | `sp-136` |
| `bl` at 0x48484 | `(unknown callee) 0xc541c` | `0x0` | `[sp-141]:1` | `sp-136` | `[sp-141]:1` |
| `bl` at 0x484b0 | `(unknown callee) 0x1955c` | `[sp-60]` | `0xf7a24` | `0x7` | `[sp-60]` |
| `bl` at 0x484dc | `(unknown callee) 0x1955c` | `[sp-140]` | `0xf7a2c` | `0x4` | `ret(0x1955c)` |
| `bl` at 0x48500 | `(unknown callee) 0xc554c` | `sp-136` | `?` | `?` | `sp-136` |
| `bl` at 0x48534 | `(unknown callee) 0xc541c` | `0x0` | `[sp-141]:1` | `sp-136` | `[sp-141]:1` |
| `bl` at 0x48560 | `(unknown callee) 0x1955c` | `[sp-60]` | `0xf7a34` | `0xa` | `[sp-60]` |
| `bl` at 0x48584 | `(unknown callee) 0xc554c` | `sp-136` | `?` | `?` | `sp-136` |
| `bl` at 0x485b8 | `(unknown callee) 0xc541c` | `0x0` | `[sp-141]:1` | `sp-136` | `[sp-141]:1` |
| `bl` at 0x485e4 | `(unknown callee) 0x1955c` | `[sp-60]` | `0xf7a34` | `0xa` | `[sp-60]` |
| `bl` at 0x48608 | `(unknown callee) 0xc554c` | `sp-136` | `?` | `?` | `sp-136` |
| `bl` at 0x48634 | `(unknown callee) 0xc541c` | `0x0` | `([sp-142]:1 & 0xff)` | `sp-136` | `([sp-142]:1 & 0xff)` |
| `bl` at 0x48660 | `(unknown callee) 0x1955c` | `[sp-60]` | `0xf7a40` | `0xc` | `[sp-60]` |
| `bl` at 0x48684 | `(unknown callee) 0xc554c` | `sp-136` | `?` | `?` | `sp-136` |
| `bl` at 0x486b4 | `(unknown callee) 0x1d444` | `0x9` | `?` | `0x297578` | `0x2` |
| `bl` at 0x4885c | `(unknown callee) 0x1b5f0` | `0x9` | `[0x12d704]:1` | `?` | `[0x12d704]:1` |
| `bl` at 0x48870 | `(unknown callee) 0x19c04` | `0x297580` | `[0x12d704]:1` | `?` | `[0x12d704]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x481f0 | `ret(0xc541c) != 0x0` | `ret(0xc541c) == 0x0` |
| `beq` at 0x48200 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `bne` at 0x4821c | `ret(0x1955c) != 0x0` | `ret(0x1955c) == 0x0` |
| `bne` at 0x48258 | `ret(0x1955c) != 0x0` | `ret(0x1955c) == 0x0` |
| `bne` at 0x48280 | `ret(0x1955c) != 0x0` | `ret(0x1955c) == 0x0` |
| `bne` at 0x482ac | `ret(0x1955c) != 0x0` | `ret(0x1955c) == 0x0` |
| `bne` at 0x482d4 | `ret(0x1955c) != 0x0` | `ret(0x1955c) == 0x0` |
| `bne` at 0x482f8 | `[0x12d704]:1 != 0x3f` | `[0x12d704]:1 == 0x3f` |
| `bne` at 0x48328 | `ret(0xc541c) != 0x0` | `ret(0xc541c) == 0x0` |
| `beq` at 0x48338 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `bne` at 0x4835c | `ret(0x1955c) != 0x0` | `ret(0x1955c) == 0x0` |
| `bne` at 0x48388 | `ret(0x1955c) != 0x0` | `ret(0x1955c) == 0x0` |
| `bne` at 0x483b4 | `ret(0x1955c) != 0x0` | `ret(0x1955c) == 0x0` |
| `bne` at 0x483e0 | `ret(0x1955c) != 0x0` | `ret(0x1955c) == 0x0` |
| `bne` at 0x4840c | `ret(0x1955c) != 0x0` | `ret(0x1955c) == 0x0` |
| `bne` at 0x48438 | `ret(0x1955c) != 0x0` | `ret(0x1955c) == 0x0` |
| `bne` at 0x48460 | `[0x12d704]:1 != 0x3f` | `[0x12d704]:1 == 0x3f` |
| `bne` at 0x48490 | `ret(0xc541c) != 0x0` | `ret(0xc541c) == 0x0` |
| `beq` at 0x484a0 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `bne` at 0x484bc | `ret(0x1955c) != 0x0` | `ret(0x1955c) == 0x0` |
| `bne` at 0x484e8 | `ret(0x1955c) != 0x0` | `ret(0x1955c) == 0x0` |
| `bne` at 0x48510 | `[0x12d704]:1 != 0x3f` | `[0x12d704]:1 == 0x3f` |
| `bne` at 0x48540 | `ret(0xc541c) != 0x0` | `ret(0xc541c) == 0x0` |
| `beq` at 0x48550 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `bne` at 0x4856c | `ret(0x1955c) != 0x0` | `ret(0x1955c) == 0x0` |
| `bne` at 0x48594 | `[0x12d704]:1 != 0x3f` | `[0x12d704]:1 == 0x3f` |
| `bne` at 0x485c4 | `ret(0xc541c) != 0x0` | `ret(0xc541c) == 0x0` |
| `beq` at 0x485d4 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `bne` at 0x485f0 | `ret(0x1955c) != 0x0` | `ret(0x1955c) == 0x0` |
| `bne` at 0x48618 | `[0x12d704]:1 != 0x3f` | `[0x12d704]:1 == 0x3f` |
| `bne` at 0x48640 | `ret(0xc541c) != 0x0` | `ret(0xc541c) == 0x0` |
| `beq` at 0x48650 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `bne` at 0x4866c | `ret(0x1955c) != 0x0` | `ret(0x1955c) == 0x0` |
| `bne` at 0x48694 | `[0x12d704]:1 != 0x3f` | `[0x12d704]:1 == 0x3f` |
| `beq` at 0x486a0 | `[sp-148] == 0x0` | `[sp-148] != 0x0` |
| `ldrls` at 0x486d0 | `? <= 0x59 (unsigned, 32-bit)` | `? > 0x59 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x48878 | `r0` | `ret(0x19c04)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x486d0).

## `PA_BrickOpsInit`

- `PA_BrickOpsInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2bf6c to 0x2c0e0.
  Established. Included word assignments through `[sp-20] +0x68` and `[sp-20] +0x78` write `0x1e` and `0x1b` (`str` at 0x2bffc, `str` at 0x2c02c).
  The word through `[sp-20] +0x7c` receives `0x2b748` (`str` at 0x2c0b0).
  Board configuration field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2bf7c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2bf80 | load | 4 | `abs 0x2c0bc` | `-` |
| `str` at 0x2bf84 | store | 4 | `sp -0x18` | `0x12d570` |
| `ldr` at 0x2bf88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2bf8c | load | 4 | `abs 0x2c0c0` | `-` |
| `str` at 0x2bf90 | store | 4 | `r0 +0x0` | `0x2bd14` |
| `ldr` at 0x2bf94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2bf98 | load | 4 | `abs 0x2c0c4` | `-` |
| `str` at 0x2bf9c | store | 4 | `[sp-20] +0x4` | `0x2bb14` |
| `ldr` at 0x2bfa0 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x2bfa8 | store | 1 | `[sp-20] +0xc` | `0x0` |
| `ldr` at 0x2bfac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2bfb4 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x2bfb8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2bfc0 | store | 4 | `[sp-20] +0xa0` | `0xff` |
| `ldr` at 0x2bfc4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2bfcc | store | 4 | `[sp-20] +0xa4` | `0xff` |
| `ldr` at 0x2bfd0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2bfd8 | store | 4 | `[sp-20] +0xa8` | `0xff` |
| `ldr` at 0x2bfdc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2bfe4 | store | 4 | `[sp-20] +0xac` | `0xff` |
| `ldr` at 0x2bfe8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2bfec | load | 4 | `abs 0x2c0c8` | `-` |
| `str` at 0x2bff0 | store | 4 | `[sp-20] +0x14` | `0xf21c8` |
| `ldr` at 0x2bff4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2bffc | store | 4 | `[sp-20] +0x68` | `0x1e` |
| `ldr` at 0x2c000 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2c008 | store | 4 | `[sp-20] +0x6c` | `0x23` |
| `ldr` at 0x2c00c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2c014 | store | 4 | `[sp-20] +0x70` | `0x28` |
| `ldr` at 0x2c018 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2c020 | store | 4 | `[sp-20] +0x74` | `0x2d` |
| `ldr` at 0x2c024 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2c02c | store | 4 | `[sp-20] +0x78` | `0x1b` |
| `ldr` at 0x2c030 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2c038 | store | 4 | `[sp-20] +0x88` | `0x0` |
| `ldr` at 0x2c03c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2c044 | store | 4 | `[sp-20] +0x90` | `0x0` |
| `ldr` at 0x2c048 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2c050 | store | 4 | `[sp-20] +0x94` | `0x0` |
| `ldr` at 0x2c054 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2c05c | store | 4 | `[sp-20] +0x98` | `0x0` |
| `ldr` at 0x2c060 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2c064 | load | 4 | `abs 0x2c0cc` | `-` |
| `str` at 0x2c068 | store | 4 | `[sp-20] +0xc4` | `0x4b56c` |
| `ldr` at 0x2c06c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2c070 | load | 4 | `abs 0x2c0d0` | `-` |
| `str` at 0x2c074 | store | 4 | `[sp-20] +0xc8` | `0x4b778` |
| `ldr` at 0x2c078 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2c080 | store | 4 | `[sp-20] +0xd0` | `0x0` |
| `ldr` at 0x2c084 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2c08c | store | 4 | `[sp-20] +0xd4` | `0x0` |
| `ldr` at 0x2c090 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2c094 | load | 4 | `abs 0x2c0d4` | `-` |
| `str` at 0x2c098 | store | 4 | `[sp-20] +0xec` | `0x2bd58` |
| `ldr` at 0x2c09c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2c0a0 | load | 4 | `abs 0x2c0d8` | `-` |
| `str` at 0x2c0a4 | store | 4 | `[sp-20] +0xf0` | `0x4ba7c` |
| `ldr` at 0x2c0a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2c0ac | load | 4 | `abs 0x2c0dc` | `-` |
| `str` at 0x2c0b0 | store | 4 | `[sp-20] +0x7c` | `0x2b748` |
| `ldm` at 0x2c0b8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x2c0b8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x2c0b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x2c0b8 | `r0` | `r0` |

## `Vortix_BrickOpsInit`

- `Vortix_BrickOpsInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x27548 to 0x27754.
  Established. The unnamed `0x19610` call at `0x27574` carries `sp-96`, `0xf0490`, `0x4c`, `0xf0490` (`bl` at 0x27574);
  its later call carries `[sp-20]+24`, `sp-96`, `0x4c`, `[sp-20]+24` (`bl` at 0x27640).
  An included word assignment through `[sp-20] +0x88` writes `0x8` (`str` at 0x27664).
  Operation record semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x27558 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2755c | load | 4 | `abs 0x2770c` | `-` |
| `ldr` at 0x27578 | load | 4 | `abs 0x27710` | `-` |
| `str` at 0x2757c | store | 4 | `sp -0x64` | `0x12d4a0` |
| `ldr` at 0x27580 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27584 | load | 4 | `abs 0x27714` | `-` |
| `str` at 0x27588 | store | 4 | `[sp-20] +0x0` | `0x2743c` |
| `ldr` at 0x2758c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27590 | load | 4 | `abs 0x27718` | `-` |
| `str` at 0x27594 | store | 4 | `[sp-20] +0x4` | `0x27128` |
| `ldr` at 0x27598 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x275a0 | store | 1 | `[sp-20] +0xc` | `0x4e` |
| `ldr` at 0x275a4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x275ac | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x275b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x275b8 | store | 4 | `[sp-20] +0xa0` | `0x19` |
| `ldr` at 0x275bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x275c4 | store | 4 | `[sp-20] +0xa4` | `0x22` |
| `ldr` at 0x275c8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x275d0 | store | 4 | `[sp-20] +0xa8` | `0x2b` |
| `ldr` at 0x275d4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x275dc | store | 4 | `[sp-20] +0xac` | `0x34` |
| `ldr` at 0x275e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x275e4 | load | 4 | `abs 0x2771c` | `-` |
| `str` at 0x275e8 | store | 4 | `[sp-20] +0xb0` | `0xfaf70` |
| `ldr` at 0x275ec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x275f4 | store | 4 | `[sp-20] +0x68` | `0x1e` |
| `ldr` at 0x275f8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27600 | store | 4 | `[sp-20] +0x6c` | `0x2d` |
| `ldr` at 0x27604 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2760c | store | 4 | `[sp-20] +0x70` | `0x32` |
| `ldr` at 0x27610 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27618 | store | 4 | `[sp-20] +0x74` | `0x37` |
| `ldr` at 0x2761c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27624 | store | 4 | `[sp-20] +0x78` | `0x19` |
| `ldr` at 0x27628 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27644 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27648 | load | 4 | `abs 0x27720` | `-` |
| `str` at 0x2764c | store | 4 | `[sp-20] +0x7c` | `0x26798` |
| `ldr` at 0x27650 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27658 | store | 4 | `[sp-20] +0x8c` | `0x1` |
| `ldr` at 0x2765c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27664 | store | 4 | `[sp-20] +0x88` | `0x8` |
| `ldr` at 0x27668 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2766c | load | 4 | `abs 0x27724` | `-` |
| `str` at 0x27670 | store | 4 | `[sp-20] +0x90` | `0x56824` |
| `ldr` at 0x27674 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27678 | load | 4 | `abs 0x27728` | `-` |
| `str` at 0x2767c | store | 4 | `[sp-20] +0x94` | `0x56914` |
| `ldr` at 0x27680 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27684 | load | 4 | `abs 0x2772c` | `-` |
| `str` at 0x27688 | store | 4 | `[sp-20] +0x98` | `0x568a0` |
| `ldr` at 0x2768c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27694 | store | 4 | `[sp-20] +0xb4` | `0x4` |
| `ldr` at 0x27698 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2769c | load | 4 | `abs 0x27730` | `-` |
| `str` at 0x276a0 | store | 4 | `[sp-20] +0xb8` | `0x26450` |
| `ldr` at 0x276a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x276a8 | load | 4 | `abs 0x27734` | `-` |
| `str` at 0x276ac | store | 4 | `[sp-20] +0xbc` | `0x264bc` |
| `ldr` at 0x276b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x276b4 | load | 4 | `abs 0x27738` | `-` |
| `str` at 0x276b8 | store | 4 | `[sp-20] +0xc0` | `0x26528` |
| `ldr` at 0x276bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x276c0 | load | 4 | `abs 0x2773c` | `-` |
| `str` at 0x276c4 | store | 4 | `[sp-20] +0xc4` | `0x26204` |
| `ldr` at 0x276c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x276cc | load | 4 | `abs 0x27740` | `-` |
| `str` at 0x276d0 | store | 4 | `[sp-20] +0xc8` | `0x262bc` |
| `ldr` at 0x276d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x276d8 | load | 4 | `abs 0x27744` | `-` |
| `str` at 0x276dc | store | 4 | `[sp-20] +0xd0` | `0x26398` |
| `ldr` at 0x276e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x276e4 | load | 4 | `abs 0x27748` | `-` |
| `str` at 0x276e8 | store | 4 | `[sp-20] +0xd4` | `0x26418` |
| `ldr` at 0x276ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x276f0 | load | 4 | `abs 0x2774c` | `-` |
| `str` at 0x276f4 | store | 4 | `[sp-20] +0xec` | `0x26b58` |
| `ldr` at 0x276f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x276fc | load | 4 | `abs 0x27750` | `-` |
| `str` at 0x27700 | store | 4 | `[sp-20] +0xf0` | `0x26df8` |
| `ldm` at 0x27708 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x27708 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x27708 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x27574 | `(unknown callee) 0x19610` | `sp-96` | `0xf0490` | `0x4c` | `0xf0490` |
| `bl` at 0x27640 | `(unknown callee) 0x19610` | `[sp-20]+24` | `sp-96` | `0x4c` | `[sp-20]+24` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x27708 | `r0` | `ret(0x19610)` |

## `Dorado_BrickOpsInit`

- `Dorado_BrickOpsInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x30a84 to 0x30c94.
  Established. The word through entry `r0 +0x0` receives `0x30a2c` (`str` at 0x30aa8).
  Included word stores through `[sp-20] +0xfc` and `[sp-20] +0xcc` write `0x32e54` and `0x2f0f0` (`str` at 0x30b68, `str` at 0x30c04).
  Device meanings of these assignments remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x30a94 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x30a98 | load | 4 | `abs 0x30c40` | `-` |
| `str` at 0x30a9c | store | 4 | `sp -0x18` | `0x12d674` |
| `ldr` at 0x30aa0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x30aa4 | load | 4 | `abs 0x30c44` | `-` |
| `str` at 0x30aa8 | store | 4 | `r0 +0x0` | `0x30a2c` |
| `ldr` at 0x30aac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x30ab0 | load | 4 | `abs 0x30c48` | `-` |
| `str` at 0x30ab4 | store | 4 | `[sp-20] +0x4` | `0x307a4` |
| `ldr` at 0x30ab8 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x30ac0 | store | 1 | `[sp-20] +0xc` | `0x52` |
| `ldr` at 0x30ac4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x30acc | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x30ad0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x30ad8 | store | 4 | `[sp-20] +0xa0` | `0x13` |
| `ldr` at 0x30adc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x30ae4 | store | 4 | `[sp-20] +0xa4` | `0xff` |
| `ldr` at 0x30ae8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x30af0 | store | 4 | `[sp-20] +0xa8` | `0xff` |
| `ldr` at 0x30af4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x30afc | store | 4 | `[sp-20] +0xac` | `0xff` |
| `ldr` at 0x30b00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x30b04 | load | 4 | `abs 0x30c4c` | `-` |
| `str` at 0x30b08 | store | 4 | `[sp-20] +0x14` | `0xf4598` |
| `ldr` at 0x30b0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x30b10 | load | 4 | `abs 0x30c50` | `-` |
| `str` at 0x30b14 | store | 4 | `[sp-20] +0xb0` | `0xfaf70` |
| `ldr` at 0x30b18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x30b1c | load | 4 | `abs 0x30c54` | `-` |
| `str` at 0x30b20 | store | 4 | `[sp-20] +0x7c` | `0x2f500` |
| `ldr` at 0x30b24 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x30b2c | store | 4 | `[sp-20] +0x68` | `0x47` |
| `ldr` at 0x30b30 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x30b38 | store | 4 | `[sp-20] +0x6c` | `0x48` |
| `ldr` at 0x30b3c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x30b44 | store | 4 | `[sp-20] +0x70` | `0x49` |
| `ldr` at 0x30b48 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x30b50 | store | 4 | `[sp-20] +0x74` | `0x4b` |
| `ldr` at 0x30b54 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x30b5c | store | 4 | `[sp-20] +0x78` | `0x45` |
| `ldr` at 0x30b60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x30b64 | load | 4 | `abs 0x30c58` | `-` |
| `str` at 0x30b68 | store | 4 | `[sp-20] +0xfc` | `0x32e54` |
| `ldr` at 0x30b6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x30b70 | load | 4 | `abs 0x30c5c` | `-` |
| `str` at 0x30b74 | store | 4 | `[sp-20] +0x80` | `0x3276c` |
| `ldr` at 0x30b78 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x30b80 | store | 4 | `[sp-20] +0x8c` | `0x1` |
| `ldr` at 0x30b84 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x30b8c | store | 4 | `[sp-20] +0x88` | `0x2` |
| `ldr` at 0x30b90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x30b94 | load | 4 | `abs 0x30c60` | `-` |
| `str` at 0x30b98 | store | 4 | `[sp-20] +0x90` | `0x56824` |
| `ldr` at 0x30b9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x30ba0 | load | 4 | `abs 0x30c64` | `-` |
| `str` at 0x30ba4 | store | 4 | `[sp-20] +0x94` | `0x56914` |
| `ldr` at 0x30ba8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x30bac | load | 4 | `abs 0x30c68` | `-` |
| `str` at 0x30bb0 | store | 4 | `[sp-20] +0x98` | `0x568a0` |
| `ldr` at 0x30bb4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x30bbc | store | 4 | `[sp-20] +0xb4` | `0x1` |
| `ldr` at 0x30bc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x30bc4 | load | 4 | `abs 0x30c6c` | `-` |
| `str` at 0x30bc8 | store | 4 | `[sp-20] +0xb8` | `0x2f320` |
| `ldr` at 0x30bcc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x30bd0 | load | 4 | `abs 0x30c70` | `-` |
| `str` at 0x30bd4 | store | 4 | `[sp-20] +0xbc` | `0x2f374` |
| `ldr` at 0x30bd8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x30bdc | load | 4 | `abs 0x30c74` | `-` |
| `str` at 0x30be0 | store | 4 | `[sp-20] +0xc0` | `0x2f3c8` |
| `ldr` at 0x30be4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x30be8 | load | 4 | `abs 0x30c78` | `-` |
| `str` at 0x30bec | store | 4 | `[sp-20] +0xc4` | `0x2ef9c` |
| `ldr` at 0x30bf0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x30bf4 | load | 4 | `abs 0x30c7c` | `-` |
| `str` at 0x30bf8 | store | 4 | `[sp-20] +0xc8` | `0x2f048` |
| `ldr` at 0x30bfc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x30c00 | load | 4 | `abs 0x30c80` | `-` |
| `str` at 0x30c04 | store | 4 | `[sp-20] +0xcc` | `0x2f0f0` |
| `ldr` at 0x30c08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x30c0c | load | 4 | `abs 0x30c84` | `-` |
| `str` at 0x30c10 | store | 4 | `[sp-20] +0xd0` | `0x2f278` |
| `ldr` at 0x30c14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x30c18 | load | 4 | `abs 0x30c88` | `-` |
| `str` at 0x30c1c | store | 4 | `[sp-20] +0xd4` | `0x2f2cc` |
| `ldr` at 0x30c20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x30c24 | load | 4 | `abs 0x30c8c` | `-` |
| `str` at 0x30c28 | store | 4 | `[sp-20] +0xec` | `0x2ffec` |
| `ldr` at 0x30c2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x30c30 | load | 4 | `abs 0x30c90` | `-` |
| `str` at 0x30c34 | store | 4 | `[sp-20] +0xf0` | `0x2f9f0` |
| `ldm` at 0x30c3c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x30c3c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x30c3c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x30c3c | `r0` | `r0` |

## `TIOTest_BrickOpsInit`

- `TIOTest_BrickOpsInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2ea60 to 0x2ebac.
  Established. The word through `[sp-20] +0x18` receives `0x5` (`str` at 0x2eb68).
  Separate word stores write `0x5` through `[sp-20] +0x1c` and `0x8` through `[sp-20] +0x20` (`str` at 0x2eb74, `str` at 0x2eb80).
  Test-operation record interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2ea70 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2ea74 | load | 4 | `abs 0x2eb8c` | `-` |
| `str` at 0x2ea78 | store | 4 | `sp -0x18` | `0x12d63c` |
| `ldr` at 0x2ea7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2ea80 | load | 4 | `abs 0x2eb90` | `-` |
| `str` at 0x2ea84 | store | 4 | `r0 +0x0` | `0x2e814` |
| `ldr` at 0x2ea88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2ea8c | load | 4 | `abs 0x2eb94` | `-` |
| `str` at 0x2ea90 | store | 4 | `[sp-20] +0x4` | `0x2e3e8` |
| `ldr` at 0x2ea94 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x2ea9c | store | 1 | `[sp-20] +0xc` | `0x0` |
| `ldr` at 0x2eaa0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2eaa8 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x2eaac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2eab4 | store | 4 | `[sp-20] +0xa0` | `0xff` |
| `ldr` at 0x2eab8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2eac0 | store | 4 | `[sp-20] +0xa4` | `0xff` |
| `ldr` at 0x2eac4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2eacc | store | 4 | `[sp-20] +0xa8` | `0xff` |
| `ldr` at 0x2ead0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ead8 | store | 4 | `[sp-20] +0xac` | `0xff` |
| `ldr` at 0x2eadc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2eae0 | load | 4 | `abs 0x2eb98` | `-` |
| `str` at 0x2eae4 | store | 4 | `[sp-20] +0x14` | `0xf3804` |
| `ldr` at 0x2eae8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2eaf0 | store | 4 | `[sp-20] +0x88` | `0x0` |
| `ldr` at 0x2eaf4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2eafc | store | 4 | `[sp-20] +0x90` | `0x0` |
| `ldr` at 0x2eb00 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2eb08 | store | 4 | `[sp-20] +0x94` | `0x0` |
| `ldr` at 0x2eb0c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2eb14 | store | 4 | `[sp-20] +0x98` | `0x0` |
| `ldr` at 0x2eb18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2eb1c | load | 4 | `abs 0x2eb9c` | `-` |
| `str` at 0x2eb20 | store | 4 | `[sp-20] +0xc4` | `0x4b56c` |
| `ldr` at 0x2eb24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2eb28 | load | 4 | `abs 0x2eba0` | `-` |
| `str` at 0x2eb2c | store | 4 | `[sp-20] +0xc8` | `0x4b778` |
| `ldr` at 0x2eb30 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2eb38 | store | 4 | `[sp-20] +0xd0` | `0x0` |
| `ldr` at 0x2eb3c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2eb44 | store | 4 | `[sp-20] +0xd4` | `0x0` |
| `ldr` at 0x2eb48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2eb4c | load | 4 | `abs 0x2eba4` | `-` |
| `str` at 0x2eb50 | store | 4 | `[sp-20] +0xec` | `0x2de3c` |
| `ldr` at 0x2eb54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2eb58 | load | 4 | `abs 0x2eba8` | `-` |
| `str` at 0x2eb5c | store | 4 | `[sp-20] +0xf0` | `0x4ba7c` |
| `ldr` at 0x2eb60 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2eb68 | store | 4 | `[sp-20] +0x18` | `0x5` |
| `ldr` at 0x2eb6c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2eb74 | store | 4 | `[sp-20] +0x1c` | `0x5` |
| `ldr` at 0x2eb78 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2eb80 | store | 4 | `[sp-20] +0x20` | `0x8` |
| `ldm` at 0x2eb88 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x2eb88 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x2eb88 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x2eb88 | `r0` | `r0` |

## `Gemini_BrickOpsInit`

- `Gemini_BrickOpsInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x28ebc to 0x2906c.
  Established. The word through entry `r0 +0x0` receives `0x28e64` (`str` at 0x28ee0).
  An included word assignment through `[sp-20] +0xa0` writes `0xf` (`str` at 0x28f10).
  The word through `[sp-20] +0xec` receives `0x28444` (`str` at 0x29018).
  The assigned operation roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x28ecc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x28ed0 | load | 4 | `abs 0x29030` | `-` |
| `str` at 0x28ed4 | store | 4 | `sp -0x18` | `0x12d514` |
| `ldr` at 0x28ed8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28edc | load | 4 | `abs 0x29034` | `-` |
| `str` at 0x28ee0 | store | 4 | `r0 +0x0` | `0x28e64` |
| `ldr` at 0x28ee4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28ee8 | load | 4 | `abs 0x29038` | `-` |
| `str` at 0x28eec | store | 4 | `[sp-20] +0x4` | `0x28c04` |
| `ldr` at 0x28ef0 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x28ef8 | store | 1 | `[sp-20] +0xc` | `0x0` |
| `ldr` at 0x28efc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28f04 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x28f08 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28f10 | store | 4 | `[sp-20] +0xa0` | `0xf` |
| `ldr` at 0x28f14 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28f1c | store | 4 | `[sp-20] +0xa4` | `0xff` |
| `ldr` at 0x28f20 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28f28 | store | 4 | `[sp-20] +0xa8` | `0xff` |
| `ldr` at 0x28f2c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28f34 | store | 4 | `[sp-20] +0xac` | `0xff` |
| `ldr` at 0x28f38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28f3c | load | 4 | `abs 0x2903c` | `-` |
| `str` at 0x28f40 | store | 4 | `[sp-20] +0x14` | `0xf1130` |
| `ldr` at 0x28f44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28f48 | load | 4 | `abs 0x29040` | `-` |
| `str` at 0x28f4c | store | 4 | `[sp-20] +0xb0` | `0xfaf70` |
| `ldr` at 0x28f50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28f54 | load | 4 | `abs 0x29044` | `-` |
| `str` at 0x28f58 | store | 4 | `[sp-20] +0x7c` | `0x27e78` |
| `ldr` at 0x28f5c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28f64 | store | 4 | `[sp-20] +0x68` | `0x1e` |
| `ldr` at 0x28f68 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28f70 | store | 4 | `[sp-20] +0x6c` | `0x2d` |
| `ldr` at 0x28f74 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28f7c | store | 4 | `[sp-20] +0x70` | `0x32` |
| `ldr` at 0x28f80 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28f88 | store | 4 | `[sp-20] +0x74` | `0x37` |
| `ldr` at 0x28f8c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28f94 | store | 4 | `[sp-20] +0x78` | `0x17` |
| `ldr` at 0x28f98 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28fa0 | store | 4 | `[sp-20] +0x8c` | `0x1` |
| `ldr` at 0x28fa4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28fac | store | 4 | `[sp-20] +0x88` | `0x2` |
| `ldr` at 0x28fb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28fb4 | load | 4 | `abs 0x29048` | `-` |
| `str` at 0x28fb8 | store | 4 | `[sp-20] +0x90` | `0x56824` |
| `ldr` at 0x28fbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28fc0 | load | 4 | `abs 0x2904c` | `-` |
| `str` at 0x28fc4 | store | 4 | `[sp-20] +0x94` | `0x56914` |
| `ldr` at 0x28fc8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28fcc | load | 4 | `abs 0x29050` | `-` |
| `str` at 0x28fd0 | store | 4 | `[sp-20] +0x98` | `0x568a0` |
| `ldr` at 0x28fd4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28fdc | store | 4 | `[sp-20] +0xb4` | `0x1` |
| `ldr` at 0x28fe0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28fe4 | load | 4 | `abs 0x29054` | `-` |
| `str` at 0x28fe8 | store | 4 | `[sp-20] +0xc4` | `0x27b70` |
| `ldr` at 0x28fec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28ff0 | load | 4 | `abs 0x29058` | `-` |
| `str` at 0x28ff4 | store | 4 | `[sp-20] +0xc8` | `0x27c24` |
| `ldr` at 0x28ff8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28ffc | load | 4 | `abs 0x2905c` | `-` |
| `str` at 0x29000 | store | 4 | `[sp-20] +0xd0` | `0x27cec` |
| `ldr` at 0x29004 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29008 | load | 4 | `abs 0x29060` | `-` |
| `str` at 0x2900c | store | 4 | `[sp-20] +0xd4` | `0x27d40` |
| `ldr` at 0x29010 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29014 | load | 4 | `abs 0x29064` | `-` |
| `str` at 0x29018 | store | 4 | `[sp-20] +0xec` | `0x28444` |
| `ldr` at 0x2901c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29020 | load | 4 | `abs 0x29068` | `-` |
| `str` at 0x29024 | store | 4 | `[sp-20] +0xf0` | `0x28354` |
| `ldm` at 0x2902c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x2902c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x2902c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x2902c | `r0` | `r0` |

## `sn1sc_reset_brick`

- `sn1sc_reset_brick`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd9010 to 0xd90fc.
  Established. A nonzero `[0x297378]` takes `0xd9074`, while zero falls through (`bne` at 0xd9058).
  The indirect call carries unknown, `0x1`, `0x29c468`, `[0x29766c]` (`mov` at 0xd90a0);
  its result is stored as a word through `[sp-24] +0x0` (`str` at 0xd90ac).
  Brick reset effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd9020 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd9024 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xd902c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xd9030 | load | 4 | `abs 0xd90e4` | `-` |
| `ldr` at 0xd9034 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xd9040 | load | 4 | `abs 0xd90e8` | `-` |
| `ldr` at 0xd9044 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd904c | load | 4 | `abs 0xd90ec` | `-` |
| `ldr` at 0xd9050 | load | 4 | `abs 0x297378` | `-` |
| `ldr` at 0xd905c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd9060 | load | 4 | `abs 0xd90f0` | `-` |
| `str` at 0xd9064 | store | 4 | `(unknown)` | `0xfffff7cc` |
| `str` at 0xd906c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xd9078 | load | 4 | `abs 0xd90f4` | `-` |
| `ldr` at 0xd907c | load | 4 | `abs 0x29766c` | `-` |
| `ldr` at 0xd9088 | load | 4 | `abs 0xd90f4` | `-` |
| `ldr` at 0xd908c | load | 4 | `abs 0x29766c` | `-` |
| `ldr` at 0xd9098 | load | 4 | `abs 0xd90f8` | `-` |
| `ldr` at 0xd90a8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd90ac | store | 4 | `[sp-24] +0x0` | `ret(unknown)` |
| `ldr` at 0xd90b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd90b4 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xd90c4 | store | 4 | `sp -0x48` | `?` |
| `str` at 0xd90d4 | store | 4 | `sp -0x48` | `0x1` |
| `ldr` at 0xd90d8 | load | 4 | `sp -0x48` | `-` |
| `ldm` at 0xd90e0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd90e0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd90e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd9048 | `(unknown callee) 0x221f0` | `0x12167c` | `r0` | `r2` | `[0x29737c]` |
| `bl` at 0xd9074 | `(unknown callee) 0xd90fc` | `?` | `?` | `?` | `[0x297378]` |
| `mov` at 0xd90a0 | `(unknown callee) (unknown target)` | `?` | `0x1` | `0x29c468` | `[0x29766c]` |
| `bl` at 0xd90cc | `(unknown callee) 0xd90fc` | `ret(0xd90fc)` | `?` | `?` | `[0x29766c]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd903c | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `bne` at 0xd9058 | `[0x297378] != 0x0` | `[0x297378] == 0x0` |
| `beq` at 0xd9084 | `[0x29766c] == 0x0` | `[0x29766c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd90e0 | `r0` | `?` |

## `IP67_BrickOpsInit`

- `IP67_BrickOpsInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x34158 to 0x34350.
  Established. The byte through `[sp-20] +0xc` receives `0x54` (`strb` at 0x34194).
  The unnamed `0x19610` call carries `0x297598`, `0xf490c`, `0x10`, `0x33704` (`bl` at 0x34210).
  The word through `[sp-20] +0x84` receives `0x556d0` (`str` at 0x341f4).
  Hardware operation mapping remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x34168 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x3416c | load | 4 | `abs 0x34300` | `-` |
| `str` at 0x34170 | store | 4 | `sp -0x18` | `0x12d6e4` |
| `ldr` at 0x34174 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x34178 | load | 4 | `abs 0x34304` | `-` |
| `str` at 0x3417c | store | 4 | `r0 +0x0` | `0x33f1c` |
| `ldr` at 0x34180 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x34184 | load | 4 | `abs 0x34308` | `-` |
| `str` at 0x34188 | store | 4 | `[sp-20] +0x4` | `0x33f64` |
| `ldr` at 0x3418c | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x34194 | store | 1 | `[sp-20] +0xc` | `0x54` |
| `ldr` at 0x34198 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x341a0 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x341a4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x341ac | store | 4 | `[sp-20] +0xa0` | `0x9` |
| `ldr` at 0x341b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x341b8 | store | 4 | `[sp-20] +0xa4` | `0xa` |
| `ldr` at 0x341bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x341c4 | store | 4 | `[sp-20] +0xa8` | `0xff` |
| `ldr` at 0x341c8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x341d0 | store | 4 | `[sp-20] +0xac` | `0xff` |
| `ldr` at 0x341d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x341d8 | load | 4 | `abs 0x3430c` | `-` |
| `str` at 0x341dc | store | 4 | `[sp-20] +0x14` | `0xf5008` |
| `ldr` at 0x341e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x341e4 | load | 4 | `abs 0x34310` | `-` |
| `str` at 0x341e8 | store | 4 | `[sp-20] +0xb0` | `0xfaf70` |
| `ldr` at 0x341ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x341f0 | load | 4 | `abs 0x34314` | `-` |
| `str` at 0x341f4 | store | 4 | `[sp-20] +0x84` | `0x556d0` |
| `ldr` at 0x341f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x341fc | load | 4 | `abs 0x34318` | `-` |
| `str` at 0x34200 | store | 4 | `[sp-20] +0x7c` | `0x33704` |
| `ldr` at 0x34204 | load | 4 | `abs 0x3431c` | `-` |
| `ldr` at 0x34208 | load | 4 | `abs 0x34320` | `-` |
| `ldr` at 0x34214 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3421c | store | 4 | `[sp-20] +0x68` | `0x1e` |
| `ldr` at 0x34220 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x34228 | store | 4 | `[sp-20] +0x6c` | `0x2d` |
| `ldr` at 0x3422c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x34234 | store | 4 | `[sp-20] +0x70` | `0x32` |
| `ldr` at 0x34238 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x34240 | store | 4 | `[sp-20] +0x74` | `0x37` |
| `ldr` at 0x34244 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3424c | store | 4 | `[sp-20] +0x78` | `0x17` |
| `ldr` at 0x34250 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x34254 | load | 4 | `abs 0x34324` | `-` |
| `str` at 0x34258 | store | 4 | `[sp-20] +0xfc` | `0x32e54` |
| `ldr` at 0x3425c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x34260 | load | 4 | `abs 0x34328` | `-` |
| `str` at 0x34264 | store | 4 | `[sp-20] +0x80` | `0x3276c` |
| `ldr` at 0x34268 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x34270 | store | 4 | `[sp-20] +0x8c` | `0x1` |
| `ldr` at 0x34274 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3427c | store | 4 | `[sp-20] +0x88` | `0x2` |
| `ldr` at 0x34280 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x34284 | load | 4 | `abs 0x3432c` | `-` |
| `str` at 0x34288 | store | 4 | `[sp-20] +0x90` | `0x56824` |
| `ldr` at 0x3428c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x34290 | load | 4 | `abs 0x34330` | `-` |
| `str` at 0x34294 | store | 4 | `[sp-20] +0x94` | `0x56914` |
| `ldr` at 0x34298 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3429c | load | 4 | `abs 0x34334` | `-` |
| `str` at 0x342a0 | store | 4 | `[sp-20] +0x98` | `0x568a0` |
| `ldr` at 0x342a4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x342ac | store | 4 | `[sp-20] +0xb4` | `0x1` |
| `ldr` at 0x342b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x342b4 | load | 4 | `abs 0x34338` | `-` |
| `str` at 0x342b8 | store | 4 | `[sp-20] +0xc4` | `0x333c0` |
| `ldr` at 0x342bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x342c0 | load | 4 | `abs 0x3433c` | `-` |
| `str` at 0x342c4 | store | 4 | `[sp-20] +0xc8` | `0x3344c` |
| `ldr` at 0x342c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x342cc | load | 4 | `abs 0x34340` | `-` |
| `str` at 0x342d0 | store | 4 | `[sp-20] +0xd0` | `0x33574` |
| `ldr` at 0x342d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x342d8 | load | 4 | `abs 0x34344` | `-` |
| `str` at 0x342dc | store | 4 | `[sp-20] +0xd4` | `0x335c8` |
| `ldr` at 0x342e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x342e4 | load | 4 | `abs 0x34348` | `-` |
| `str` at 0x342e8 | store | 4 | `[sp-20] +0xec` | `0x33bb8` |
| `ldr` at 0x342ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x342f0 | load | 4 | `abs 0x3434c` | `-` |
| `str` at 0x342f4 | store | 4 | `[sp-20] +0xf0` | `0x33a60` |
| `ldm` at 0x342fc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x342fc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x342fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x34210 | `(unknown callee) 0x19610` | `0x297598` | `0xf490c` | `0x10` | `0x33704` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x342fc | `r0` | `ret(0x19610)` |

## `JChipBrickSetup`

- `JChipBrickSetup`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdcd98 to 0xdceac.
  Established. The signed byte through entry `r0 +0x21` is loaded (`ldrsb` at 0xdcdb0).
  A zero `[0x29766c]` takes `0xdce4c`, while nonzero falls through (`beq` at 0xdcde0).
  The included indirect call at `0xdce28` carries unknown, `0x0`, `0x29c6f8`, unknown (`mov` at 0xdce28).
  Board setup policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdcda8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xdcdac | load | 4 | `sp -0x14` | `-` |
| `ldrsb` at 0xdcdb0 | load | 1 | `r0 +0x21` | `-` |
| `ldr` at 0xdcdbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdcdc0 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldrh` at 0xdcdc4 | load | 2 | `[[sp-20]] +0xe` | `-` |
| `ldr` at 0xdcdd4 | load | 4 | `abs 0xdce9c` | `-` |
| `ldr` at 0xdcdd8 | load | 4 | `abs 0x29766c` | `-` |
| `ldr` at 0xdcde4 | load | 4 | `abs 0xdce9c` | `-` |
| `ldr` at 0xdcde8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xdcdec | load | 1 | `[sp-20] +0x19` | `-` |
| `ldr` at 0xdcdf4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdcdf8 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xdcdfc | load | 4 | `[[sp-20]] +0x0` | `-` |
| `ldr` at 0xdce08 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xdce0c | load | 1 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xdce14 | load | 4 | `abs 0x29766c` | `-` |
| `ldr` at 0xdce20 | load | 4 | `abs 0xdcea0` | `-` |
| `str` at 0xdce30 | store | 4 | `sp -0x18` | `ret(unknown)` |
| `ldr` at 0xdce34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdce40 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdce44 | store | 4 | `sp -0x1c` | `ret(unknown)` |
| `ldr` at 0xdce4c | load | 4 | `abs 0xdcea4` | `-` |
| `str` at 0xdce50 | store | 4 | `sp -0x1c` | `0xfffff7c5` |
| `ldr` at 0xdce58 | load | 4 | `abs 0xdcea8` | `-` |
| `ldr` at 0xdce5c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xdce60 | load | 1 | `[sp-20] +0x19` | `-` |
| `ldr` at 0xdce6c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xdce70 | load | 1 | `[sp-20] +0x19` | `-` |
| `ldr` at 0xdce74 | load | 4 | `abs 0x12da68` | `-` |
| `str` at 0xdce8c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xdce90 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xdce98 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdce98 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdce98 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0xdce28 | `(unknown callee) (unknown target)` | `?` | `0x0` | `0x29c6f8` | `?` |
| `mov` at 0xdce84 | `(unknown callee) (unknown target)` | `?` | `[[sp-20]+25]:1` | `[0x12da68]` | `[[sp-20]+25]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdcdb8 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xdcdd0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xdcde0 | `[0x29766c] == 0x0` | `[0x29766c] != 0x0` |
| `beq` at 0xdce3c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdce98 | `r0` | `?` |

## `Unknown_BrickOpsInit`

- `Unknown_BrickOpsInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x228e8 to 0x229c8.
  Established. The word through entry `r0 +0x4` receives `0x227d8` (`str` at 0x2290c).
  The byte through `[sp-20] +0xc` receives `0x4e` (`strb` at 0x22918).
  Included word stores clear `[sp-20] +0xc4` and `[sp-20] +0xd4` (`str` at 0x22990, `str` at 0x229b4).
  Fallback board operation meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x228f8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x228fc | load | 4 | `abs 0x229c0` | `-` |
| `str` at 0x22900 | store | 4 | `sp -0x18` | `0x12d3e4` |
| `ldr` at 0x22904 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x22908 | load | 4 | `abs 0x229c4` | `-` |
| `str` at 0x2290c | store | 4 | `r0 +0x4` | `0x227d8` |
| `ldr` at 0x22910 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x22918 | store | 1 | `[sp-20] +0xc` | `0x4e` |
| `ldr` at 0x2291c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x22924 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x22928 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x22930 | store | 4 | `[sp-20] +0xa0` | `0xff` |
| `ldr` at 0x22934 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2293c | store | 4 | `[sp-20] +0xa4` | `0xff` |
| `ldr` at 0x22940 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x22948 | store | 4 | `[sp-20] +0xa8` | `0xff` |
| `ldr` at 0x2294c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x22954 | store | 4 | `[sp-20] +0xac` | `0xff` |
| `ldr` at 0x22958 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x22960 | store | 4 | `[sp-20] +0x88` | `0x0` |
| `ldr` at 0x22964 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2296c | store | 4 | `[sp-20] +0x90` | `0x0` |
| `ldr` at 0x22970 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x22978 | store | 4 | `[sp-20] +0x94` | `0x0` |
| `ldr` at 0x2297c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x22984 | store | 4 | `[sp-20] +0x98` | `0x0` |
| `ldr` at 0x22988 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x22990 | store | 4 | `[sp-20] +0xc4` | `0x0` |
| `ldr` at 0x22994 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2299c | store | 4 | `[sp-20] +0xc8` | `0x0` |
| `ldr` at 0x229a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x229a8 | store | 4 | `[sp-20] +0xd0` | `0x0` |
| `ldr` at 0x229ac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x229b4 | store | 4 | `[sp-20] +0xd4` | `0x0` |
| `ldm` at 0x229bc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x229bc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x229bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x229bc | `r0` | `r0` |

## `brickInitBrickOps`

- `brickInitBrickOps`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x19c04 to 0x19da8.
  Established. The computed transfer is taken when unsigned `r1-43 <= 0x33`, with fall-through when greater (`ldrls` at 0x19c28).
  The included unnamed `0x228e8` call carries entry `r0`, entry `r1`, entry `r2`, `r1-43` (`bl` at 0x19d80).
  The return record on known edges gives `ret(0xa2a18)` (`ldm` at 0x19da4).
  Board selection across unresolved destinations remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x19c14 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x19c18 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x19c1c | load | 4 | `sp -0x18` | `-` |
| `ldrls` at 0x19c28 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x19d78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x19d7c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x19da4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x19da4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x19da4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x19d80 | `(unknown callee) 0x228e8` | `r0` | `r1` | `r2` | `r1-43` |
| `bl` at 0x19d84 | `(unknown callee) 0xce308` | `ret(0x228e8)` | `?` | `?` | `?` |
| `bl` at 0x19d88 | `(unknown callee) 0xdb4e8` | `ret(0xce308)` | `?` | `?` | `?` |
| `bl` at 0x19d8c | `(unknown callee) 0xe69f4` | `ret(0xdb4e8)` | `?` | `?` | `?` |
| `bl` at 0x19d90 | `(unknown callee) 0xdcd10` | `ret(0xe69f4)` | `?` | `?` | `?` |
| `bl` at 0x19d94 | `(unknown callee) 0x96478` | `ret(0xdcd10)` | `?` | `?` | `?` |
| `bl` at 0x19d98 | `(unknown callee) 0x99bb4` | `ret(0x96478)` | `?` | `?` | `?` |
| `bl` at 0x19d9c | `(unknown callee) 0xa2a18` | `ret(0x99bb4)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x19c28 | `r1-43 <= 0x33 (unsigned, 32-bit)` | `r1-43 > 0x33 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x19da4 | `r0` | `ret(0xa2a18)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x19c28).
