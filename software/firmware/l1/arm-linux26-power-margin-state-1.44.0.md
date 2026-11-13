# ARM Linux 2.6 power state and NVRAM access in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-power-margin-state-facts.txt) retain all tool sections.

## `powerResetStart`

- `powerResetStart`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x58d48 to 0x58d60.
  Established. Entry `r0` is saved as a word at `sp -0x14` (`str` at 0x58d58).
  The return retains entry `r0` (`ldm` at 0x58d5c).
  A physical reset-start operation is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x58d58 | store | 4 | `sp -0x14` | `r0` |
| `ldm` at 0x58d5c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x58d5c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x58d5c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x58d5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x58d5c | `r0` | `r0` |

## `powerResetDone`

- `powerResetDone`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x58d60 to 0x58df4.
  Established. A byte assignment clears `[sp-24] +0xd` (`strb` at 0x58de0).
  Zero `[0x1a5c18]` takes `0x58de4`, while nonzero falls through (`beq` at 0x58d80).
  The cleared byte's meaning and its relationship to physical power hardware remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x58d70 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x58d74 | load | 4 | `abs 0x58dec` | `-` |
| `ldr` at 0x58d78 | load | 4 | `abs 0x1a5c18` | `-` |
| `ldr` at 0x58d84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58d98 | load | 4 | `abs 0x58df0` | `-` |
| `ldr` at 0x58d9c | load | 4 | `abs 0x1a5c14` | `-` |
| `ldr` at 0x58da4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x58db4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58dc8 | load | 4 | `abs 0x58df0` | `-` |
| `ldr` at 0x58dcc | load | 4 | `abs 0x1a5c14` | `-` |
| `str` at 0x58dd4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x58dd8 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x58de0 | store | 1 | `[sp-24] +0xd` | `0x0` |
| `ldm` at 0x58de8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x58de8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x58de8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x58d80 | `[0x1a5c18] == 0x0` | `[0x1a5c18] != 0x0` |
| `bne` at 0x58dac | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x58de8 | `r0` | `r0` |

## `powerUpInProgress`

- `powerUpInProgress`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x58df4 to 0x58e8c.
  Established. A byte read addresses `[sp-24] +0xe`, and its value is assigned to the word at `sp -0x1c` (`ldrb` at 0x58e70, `str` at 0x58e74).
  Zero `[0x1a5c18]` takes `0x58e44`, while nonzero falls through (`beq` at 0x58e14).
  The return carries `[sp-28]` (`ldm` at 0x58e80).
  Power-progress encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x58e04 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x58e08 | load | 4 | `abs 0x58e84` | `-` |
| `ldr` at 0x58e0c | load | 4 | `abs 0x1a5c18` | `-` |
| `ldr` at 0x58e18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58e2c | load | 4 | `abs 0x58e88` | `-` |
| `ldr` at 0x58e30 | load | 4 | `abs 0x1a5c14` | `-` |
| `ldr` at 0x58e38 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x58e48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58e5c | load | 4 | `abs 0x58e88` | `-` |
| `ldr` at 0x58e60 | load | 4 | `abs 0x1a5c14` | `-` |
| `str` at 0x58e68 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x58e6c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x58e70 | load | 1 | `[sp-24] +0xe` | `-` |
| `str` at 0x58e74 | store | 4 | `sp -0x1c` | `[[sp-24]+14]:1` |
| `ldr` at 0x58e78 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x58e80 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x58e80 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x58e80 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x58e14 | `[0x1a5c18] == 0x0` | `[0x1a5c18] != 0x0` |
| `bne` at 0x58e40 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x58e80 | `r0` | `[sp-28]` |

## `powerUpFirstReset`

- `powerUpFirstReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x58e8c to 0x58f24.
  Established. The byte at `[sp-24] +0xd` is read into a word assignment at `sp -0x1c` (`ldrb` at 0x58f08, `str` at 0x58f0c).
  Zero `[0x1a5c18]` takes `0x58edc`, while nonzero falls through (`beq` at 0x58eac).
  Meaning of the first-reset state remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x58e9c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x58ea0 | load | 4 | `abs 0x58f1c` | `-` |
| `ldr` at 0x58ea4 | load | 4 | `abs 0x1a5c18` | `-` |
| `ldr` at 0x58eb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58ec4 | load | 4 | `abs 0x58f20` | `-` |
| `ldr` at 0x58ec8 | load | 4 | `abs 0x1a5c14` | `-` |
| `ldr` at 0x58ed0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x58ee0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58ef4 | load | 4 | `abs 0x58f20` | `-` |
| `ldr` at 0x58ef8 | load | 4 | `abs 0x1a5c14` | `-` |
| `str` at 0x58f00 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x58f04 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x58f08 | load | 1 | `[sp-24] +0xd` | `-` |
| `str` at 0x58f0c | store | 4 | `sp -0x1c` | `[[sp-24]+13]:1` |
| `ldr` at 0x58f10 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x58f18 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x58f18 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x58f18 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x58eac | `[0x1a5c18] == 0x0` | `[0x1a5c18] != 0x0` |
| `bne` at 0x58ed8 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x58f18 | `r0` | `[sp-28]` |

## `powerGetIndexOfConfig`

- `powerGetIndexOfConfig`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x58a38 to 0x58af0.
  Established. An included word read addresses `[sp-32] +0x8` (`ldr` at 0x58a80).
  The word at `sp -0x1c` receives `[sp-28]+1` on an included path (`str` at 0x58ad0).
  The return carries `[sp-36]` (`ldm` at 0x58ae8).
  Configuration index meaning and comparisons with unknown operands remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x58a48 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x58a4c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x58a50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58a64 | load | 4 | `abs 0x58aec` | `-` |
| `ldr` at 0x58a68 | load | 4 | `abs 0x1a5c14` | `-` |
| `str` at 0x58a70 | store | 4 | `sp -0x20` | `?` |
| `str` at 0x58a78 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x58a7c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x58a80 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0x58a84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58a90 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x58a94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58aa8 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x58ab0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x58abc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x58ac0 | store | 4 | `sp -0x24` | `[sp-28]` |
| `ldr` at 0x58ac8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x58ad0 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x58adc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x58ae0 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x58ae8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x58ae8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x58ae8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x58a8c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x58ab8 | `? != [sp-24]` | `? == [sp-24]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x58ae8 | `r0` | `[sp-36]` |

## `powerGetMarginValue`

- `powerGetMarginValue`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x594c8 to 0x59544.
  Established. Equality `ret(0x6e190) == [sp-20]` takes `0x5951c`, while inequality falls through (`beq` at 0x594f0).
  The taken path includes an unnamed `0x28230` call carrying `[[sp-24]+16]:1+130`, unknown, `ret(0x6e190)` and `[[sp-24]+16]:1+130` (`bl` at 0x5952c).
  Margin-value units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x594d8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x594dc | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x594e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x594f4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x594f8 | load | 1 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x59504 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x59514 | store | 4 | `sp -0x1c` | `ret(0x42428)` |
| `ldr` at 0x5951c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x59520 | load | 1 | `[sp-24] +0x10` | `-` |
| `str` at 0x59534 | store | 4 | `sp -0x1c` | `ret(0x28230)` |
| `ldr` at 0x59538 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x59540 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x59540 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x59540 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x594e0 | `(unknown callee) 0x6e190` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x5950c | `(unknown callee) 0x42428` | `[sp-20]` | `[[sp-24]+16]:1+533` | `ret(0x6e190)` | `[[sp-24]+16]:1+533` |
| `bl` at 0x5952c | `(unknown callee) 0x28230` | `[[sp-24]+16]:1+130` | `?` | `ret(0x6e190)` | `[[sp-24]+16]:1+130` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x594f0 | `ret(0x6e190) == [sp-20]` | `ret(0x6e190) != [sp-20]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x59540 | `r0` | `?` |

## `powerSetMarginValue`

- `powerSetMarginValue`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x59544 to 0x595b8.
  Established. The unnamed `0x41d74` call carries `[sp-20]`, `[[sp-24]+16]:1+533`, `[sp-28]` and `[[sp-24]+16]:1+533` (`bl` at 0x59590).
  A separate path calls unnamed `0x2624c` with `[[sp-24]+16]:1+130`, `[sp-28]`, `ret(0x6e190)` and `[[sp-24]+16]:1+130` (`bl` at 0x595ac).
  The physical setting represented by these candidates remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x59554 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x59558 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x5955c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x59568 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59574 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x59578 | load | 1 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x59584 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5958c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59598 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5959c | load | 1 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x595a8 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x595b4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x595b4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x595b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x59560 | `(unknown callee) 0x6e190` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x59590 | `(unknown callee) 0x41d74` | `[sp-20]` | `[[sp-24]+16]:1+533` | `[sp-28]` | `[[sp-24]+16]:1+533` |
| `bl` at 0x595ac | `(unknown callee) 0x2624c` | `[[sp-24]+16]:1+130` | `[sp-28]` | `ret(0x6e190)` | `[[sp-24]+16]:1+130` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x59570 | `ret(0x6e190) == [sp-20]` | `ret(0x6e190) != [sp-20]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x595b4 | `r0` | `?` |

## `powerGetMarginState`

- `powerGetMarginState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x595b8 to 0x59634.
  Established. Included byte reads address `[sp-24] +0x10` (`ldrb` at 0x595e8, `ldrb` at 0x59610).
  An unnamed `0x42428` call carries `[sp-20]`, `[[sp-24]+16]:1+517`, `ret(0x6e190)` and `[[sp-24]+16]:1+517` (`bl` at 0x595fc).
  The two return-value paths merge into an unknown integer return (`ldm` at 0x59630).
  Margin-state encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x595c8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x595cc | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x595d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x595e4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x595e8 | load | 1 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x595f4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x59604 | store | 4 | `sp -0x1c` | `ret(0x42428)` |
| `ldr` at 0x5960c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x59610 | load | 1 | `[sp-24] +0x10` | `-` |
| `str` at 0x59624 | store | 4 | `sp -0x1c` | `ret(0x28230)` |
| `ldr` at 0x59628 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x59630 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x59630 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x59630 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x595d0 | `(unknown callee) 0x6e190` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x595fc | `(unknown callee) 0x42428` | `[sp-20]` | `[[sp-24]+16]:1+517` | `ret(0x6e190)` | `[[sp-24]+16]:1+517` |
| `bl` at 0x5961c | `(unknown callee) 0x28230` | `[[sp-24]+16]:1+20` | `?` | `ret(0x6e190)` | `[[sp-24]+16]:1+20` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x595e0 | `ret(0x6e190) == [sp-20]` | `ret(0x6e190) != [sp-20]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x59630 | `r0` | `?` |

## `powerSetMarginState`

- `powerSetMarginState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x59634 to 0x596a8.
  Established. The condition `ret(0x6e190) == [sp-20]` takes `0x59688`, while inequality falls through (`beq` at 0x59660).
  That taken path includes an unnamed `0x2624c` call carrying `[[sp-24]+16]:1+20`, `[sp-28]`, `ret(0x6e190)` and `[[sp-24]+16]:1+20` (`bl` at 0x5969c).
  Register selection at the power hardware remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x59644 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x59648 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x5964c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x59658 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59664 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x59668 | load | 1 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x59674 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5967c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59688 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5968c | load | 1 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x59698 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x596a4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x596a4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x596a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x59650 | `(unknown callee) 0x6e190` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x59680 | `(unknown callee) 0x41d74` | `[sp-20]` | `[[sp-24]+16]:1+517` | `[sp-28]` | `[[sp-24]+16]:1+517` |
| `bl` at 0x5969c | `(unknown callee) 0x2624c` | `[[sp-24]+16]:1+20` | `[sp-28]` | `ret(0x6e190)` | `[[sp-24]+16]:1+20` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x59660 | `ret(0x6e190) == [sp-20]` | `ret(0x6e190) != [sp-20]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x596a4 | `r0` | `?` |

## `powerHackMargin185High`

- `powerHackMargin185High`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x596a8 to 0x598a8.
  Established. Equality `[[sp-24]]:1 == 0xe` takes `0x597b8`, while inequality falls through (`beq` at 0x597a4).
  A later test of `[[sp-24]]:1 == 0x22` takes the same target, while inequality falls through (`beq` at 0x597b0).
  Included byte assignments put `(ret(0x588cc) & 0xff)` and `(ret(0x5896c) & 0xff)` at `sp -0x19` on separate paths (`strb` at 0x59820, `strb` at 0x59838).
  The high-margin hardware workaround remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x596b8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x596c0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x596c4 | load | 4 | `abs 0x5989c` | `-` |
| `ldr` at 0x596c8 | load | 4 | `abs 0x1a5c18` | `-` |
| `ldr` at 0x596d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x596e8 | load | 4 | `abs 0x598a0` | `-` |
| `ldr` at 0x596ec | load | 4 | `abs 0x1a5c14` | `-` |
| `ldr` at 0x596f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x59700 | load | 4 | `abs 0x598a4` | `-` |
| `str` at 0x59704 | store | 4 | `sp -0x34` | `0xfffff9b9` |
| `ldr` at 0x5970c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59720 | load | 4 | `abs 0x598a0` | `-` |
| `ldr` at 0x59724 | load | 4 | `abs 0x1a5c14` | `-` |
| `str` at 0x5972c | store | 4 | `sp -0x2c` | `?` |
| `str` at 0x59734 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x59738 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5973c | load | 4 | `[sp-44] +0x8` | `-` |
| `ldr` at 0x59740 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5974c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x59750 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x59764 | load | 4 | `[sp-44] +0x0` | `-` |
| `str` at 0x5976c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x59770 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x59774 | load | 4 | `[sp-44] +0x4` | `-` |
| `ldr` at 0x59778 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x59780 | load | 1 | `([[sp-44]+4] + [sp-40]) +0x0` | `-` |
| `ldr` at 0x59790 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x59794 | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0x59798 | store | 4 | `sp -0x38` | `[[sp-24]]:1` |
| `ldr` at 0x5979c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x597a8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x597b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x597bc | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x597c8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x597cc | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0x597d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x597dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x597f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x597f4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x59800 | store | 4 | `sp -0x30` | `ret(0x595b8)` |
| `ldr` at 0x59804 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x59810 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59814 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x59820 | store | 1 | `sp -0x19` | `(ret(0x588cc) & 0xff)` |
| `ldr` at 0x59828 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5982c | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x59838 | store | 1 | `sp -0x19` | `(ret(0x5896c) & 0xff)` |
| `ldrb` at 0x5983c | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x59840 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59844 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x59854 | store | 4 | `sp -0x24` | `ret(0x59024)` |
| `ldr` at 0x59858 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59864 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x59870 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x59874 | store | 4 | `sp -0x20` | `ret(0x59024)` |
| `ldr` at 0x59878 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x59880 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x59888 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5988c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x59890 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x59898 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x59898 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x59898 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x597e0 | `(unknown callee) 0x595b8` | `[sp-20]` | `[sp-24]` | `[[sp-44]+4]` | `[[sp-24]+6]:1` |
| `bl` at 0x597f8 | `(unknown callee) 0x595b8` | `[sp-20]` | `[sp-24]` | `?` | `ret(0x595b8)` |
| `bl` at 0x59818 | `(unknown callee) 0x588cc` | `[sp-20]` | `[sp-24]` | `?` | `ret(0x595b8)` |
| `bl` at 0x59830 | `(unknown callee) 0x5896c` | `[sp-20]` | `[sp-24]` | `?` | `ret(0x595b8)` |
| `bl` at 0x5984c | `(unknown callee) 0x59024` | `[sp-20]` | `[sp-24]` | `[sp-25]:1` | `[sp-25]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x596d0 | `[0x1a5c18] == 0x0` | `[0x1a5c18] != 0x0` |
| `bne` at 0x596fc | `? != 0x0` | `? == 0x0` |
| `ble` at 0x59748 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x59788 | `[([[sp-44]+4] + [sp-40])]:1 != 0x0` | `[([[sp-44]+4] + [sp-40])]:1 == 0x0` |
| `beq` at 0x597a4 | `[[sp-24]]:1 == 0xe` | `[[sp-24]]:1 != 0xe` |
| `beq` at 0x597b0 | `[[sp-24]]:1 == 0x22` | `[[sp-24]]:1 != 0x22` |
| `beq` at 0x597c4 | `[[sp-24]+8] == 0x0` | `[[sp-24]+8] != 0x0` |
| `beq` at 0x597d4 | `[[sp-24]+6]:1 == 0xff` | `[[sp-24]+6]:1 != 0xff` |
| `beq` at 0x597ec | `ret(0x595b8) == 0x3` | `ret(0x595b8) != 0x3` |
| `bne` at 0x5980c | `ret(0x595b8) != 0x1` | `ret(0x595b8) == 0x1` |
| `beq` at 0x59860 | `ret(0x59024) == 0x0` | `ret(0x59024) != 0x0` |
| `bne` at 0x5986c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x59898 | `r0` | `?` |

## `i2c_soft_reset_power_check`

- `i2c_soft_reset_power_check`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x59a1c to 0x59cdc.
  Established. Included word assignments write `0x162038`, `0x162054` and `0x162078` to `[sp-20] +0x0` on separate paths (`str` at 0x59bf8, `str` at 0x59c14, `str` at 0x59c30).
  Nonzero `ret(0x5572c)` takes `0x59a54`, while zero falls through (`bne` at 0x59a44).
  The unnamed `0x6369c` call carries `[sp-24]`, `0xff`, unknown and `ret(0x3ee14)` (`bl` at 0x59aa4).
  Whether these checks permit a physical I2C reset remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x59a2c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x59a34 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x59a4c | store | 4 | `sp -0x2c` | `?` |
| `str` at 0x59a58 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x59a60 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x59a6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x59a78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59a84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x59a9c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x59aac | store | 4 | `sp -0x24` | `ret(0x6369c)` |
| `ldr` at 0x59ab0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59ab4 | load | 4 | `abs 0x59cc4` | `-` |
| `ldr` at 0x59ac0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x59ac8 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0x59ad4 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x59adc | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x59ae8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x59af4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x59b04 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x59b08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x59b20 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x59b2c | store | 4 | `sp -0x24` | `ret(0x558d8)` |
| `ldr` at 0x59b30 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x59b34 | store | 4 | `sp -0x30` | `ret(0x558d8)` |
| `ldr` at 0x59b38 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x59b44 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x59b50 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x59b60 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x59b78 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x59b88 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x59b94 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x59b98 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x59bac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x59bbc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x59bc4 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x59bcc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x59bdc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x59be4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x59bf0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59bf4 | load | 4 | `abs 0x59cc8` | `-` |
| `str` at 0x59bf8 | store | 4 | `[sp-20] +0x0` | `0x162038` |
| `ldr` at 0x59c00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59c0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59c10 | load | 4 | `abs 0x59ccc` | `-` |
| `str` at 0x59c14 | store | 4 | `[sp-20] +0x0` | `0x162054` |
| `ldr` at 0x59c1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59c28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59c2c | load | 4 | `abs 0x59cd0` | `-` |
| `str` at 0x59c30 | store | 4 | `[sp-20] +0x0` | `0x162078` |
| `ldr` at 0x59c5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59c60 | load | 4 | `abs 0x59cd4` | `-` |
| `str` at 0x59c64 | store | 4 | `[sp-20] +0x0` | `0x16209c` |
| `ldr` at 0x59c80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59c84 | load | 4 | `abs 0x59cd8` | `-` |
| `str` at 0x59c88 | store | 4 | `[sp-20] +0x0` | `0x1620cc` |
| `ldr` at 0x59c8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59c90 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0x59ca0 | store | 4 | `sp -0x34` | `?` |
| `str` at 0x59cac | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x59cb0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x59cb4 | store | 4 | `sp -0x2c` | `[sp-52]` |
| `ldr` at 0x59cb8 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x59cc0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x59cc0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x59cc0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x59a38 | `(unknown callee) 0x5572c` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x59a64 | `(unknown callee) 0x6e140` | `?` | `?` | `?` | `?` |
| `bl` at 0x59a88 | `(unknown callee) 0x3ee14` | `[sp-24]` | `?` | `ret(0x6e140)` | `[sp-28]` |
| `bl` at 0x59aa4 | `(unknown callee) 0x6369c` | `[sp-24]` | `0xff` | `?` | `ret(0x3ee14)` |
| `bl` at 0x59ae0 | `(unknown callee) 0x6e140` | `?` | `?` | `?` | `?` |
| `bl` at 0x59b0c | `(unknown callee) 0x3ee14` | `[sp-24]` | `?` | `ret(0x6e140)` | `0x0` |
| `bl` at 0x59b24 | `(unknown callee) 0x558d8` | `[sp-24]` | `?` | `?` | `ret(0x3ee14)` |
| `bl` at 0x59b70 | `(unknown callee) 0x6e190` | `ret(0x558d8)` | `?` | `?` | `ret(0x558d8)` |
| `bl` at 0x59ba8 | `(unknown callee) 0x55700` | `0x1` | `?` | `?` | `[sp-32]` |
| `bl` at 0x59bb0 | `(unknown callee) 0x558d8` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x59bb8 | `(unknown callee) 0x55700` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x59c38 | `(unknown callee) 0x70680` | `ret(0x6e140)` | `?` | `ret(0x6e140)` | `[sp-28]` |
| `bl` at 0x59c4c | `(unknown callee) 0x70e18` | `0x0` | `?` | `?` | `ret(0x70680)` |
| `bl` at 0x59c70 | `(unknown callee) 0x70e18` | `0x2` | `?` | `?` | `ret(0x70e18)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x59a44 | `ret(0x5572c) != 0x0` | `ret(0x5572c) == 0x0` |
| `ble` at 0x59a74 | `ret(0x6e140) <= [sp-24] (signed, 32-bit)` | `ret(0x6e140) > [sp-24] (signed, 32-bit)` |
| `beq` at 0x59a80 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `bgt` at 0x59a94 | `ret(0x3ee14) > 0x0 (signed, 32-bit)` | `ret(0x3ee14) <= 0x0 (signed, 32-bit)` |
| `beq` at 0x59abc | `ret(0x6369c) == 0xfffffa85` | `ret(0x6369c) != 0xfffffa85` |
| `ble` at 0x59af0 | `ret(0x6e140) <= [sp-24] (signed, 32-bit)` | `ret(0x6e140) > [sp-24] (signed, 32-bit)` |
| `bne` at 0x59afc | `[sp-32] != 0x1` | `[sp-32] == 0x1` |
| `bgt` at 0x59b18 | `ret(0x3ee14) > 0x0 (signed, 32-bit)` | `ret(0x3ee14) <= 0x0 (signed, 32-bit)` |
| `beq` at 0x59b40 | `ret(0x558d8) == 0x0` | `ret(0x558d8) != 0x0` |
| `bgt` at 0x59b4c | `ret(0x558d8) > 0x0 (signed, 32-bit)` | `ret(0x558d8) <= 0x0 (signed, 32-bit)` |
| `beq` at 0x59b58 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x59b68 | `ret(0x558d8) == 0x1` | `ret(0x558d8) != 0x1` |
| `bne` at 0x59b80 | `ret(0x6e190) != [sp-24]` | `ret(0x6e190) == [sp-24]` |
| `beq` at 0x59ba0 | `[sp-32] == 0x1` | `[sp-32] != 0x1` |
| `bne` at 0x59bd4 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bge` at 0x59bec | `[sp-32] >= 0x0 (signed, 32-bit)` | `[sp-32] < 0x0 (signed, 32-bit)` |
| `bne` at 0x59c08 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bge` at 0x59c24 | `[sp-28] >= 0x0 (signed, 32-bit)` | `[sp-28] < 0x0 (signed, 32-bit)` |
| `beq` at 0x59c44 | `ret(0x70680) == 0x0` | `ret(0x70680) != 0x0` |
| `beq` at 0x59c58 | `ret(0x70e18) == 0x1` | `ret(0x70e18) != 0x1` |
| `bne` at 0x59c7c | `ret(0x70e18) != 0x1` | `ret(0x70e18) == 0x1` |
| `beq` at 0x59c98 | `[[sp-20]] == 0x0` | `[[sp-20]] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x59cc0 | `r0` | `[sp-44]` |

## `nvramRead`

- `nvramRead`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x29d2c to 0x29da0.
  Established. The signed condition `(r0 + r2) <= 0x1fff` takes `0x29d6c`, while greater falls through (`ble` at 0x29d5c).
  An unnamed `0x203f4` call has four unknown argument candidates (`bl` at 0x29d84).
  The return carries `[sp-28]` (`ldm` at 0x29d94).
  NVRAM source selection and physical transfer behavior remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x29d3c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x29d40 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x29d44 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x29d48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29d4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x29d54 | load | 4 | `abs 0x29d98` | `-` |
| `ldr` at 0x29d60 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x29d68 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x29d6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29d70 | load | 4 | `abs 0x29d9c` | `-` |
| `ldr` at 0x29d78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29d80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x29d88 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x29d94 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x29d94 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x29d94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x29d84 | `(unknown callee) 0x203f4` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x29d5c | `(r0 + r2) <= 0x1fff (signed, 32-bit)` | `(r0 + r2) > 0x1fff (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x29d94 | `r0` | `[sp-28]` |

## `nvramWrite`

- `nvramWrite`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x29da0 to 0x29e14.
  Established. Entry `r2` is saved as a word at `sp -0x1c` (`str` at 0x29db8).
  Signed `(r0 + r2) <= 0x1fff` takes `0x29de0`, while greater falls through (`ble` at 0x29dd0).
  The unnamed `0x203f4` call has four unknown argument candidates (`bl` at 0x29df8).
  Physical write completion and persistence remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x29db0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x29db4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x29db8 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x29dbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29dc0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x29dc8 | load | 4 | `abs 0x29e0c` | `-` |
| `ldr` at 0x29dd4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x29ddc | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x29de0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29de4 | load | 4 | `abs 0x29e10` | `-` |
| `ldr` at 0x29df0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29df4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x29dfc | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x29e08 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x29e08 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x29e08 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x29df8 | `(unknown callee) 0x203f4` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x29dd0 | `(r0 + r2) <= 0x1fff (signed, 32-bit)` | `(r0 + r2) > 0x1fff (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x29e08 | `r0` | `[sp-28]` |

## `nvramLogReset`

- `nvramLogReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x29340 to 0x293f8.
  Established. Separate word assignments copy `[[0x1a58dc]+1932]` to `[0x1a58dc] +0x794` and `[0x1a58dc] +0x798` (`str` at 0x29360, `str` at 0x29378).
  A byte assignment writes `(ret(0x52800) & 0xff)` to `[0x1a58dc] +0x7ab` (`strb` at 0x293a4).
  An unnamed `0x203f4` call carries `[0x1a58dc]+1964`, `[0x1a58dc]+1928`, `0x24` and `[0x1a58dc]+1928` (`bl` at 0x293d4).
  Persistent log-record field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x2934c | load | 4 | `abs 0x293ec` | `-` |
| `ldr` at 0x29350 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x29354 | load | 4 | `abs 0x293ec` | `-` |
| `ldr` at 0x29358 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x2935c | load | 4 | `[0x1a58dc] +0x78c` | `-` |
| `str` at 0x29360 | store | 4 | `[0x1a58dc] +0x794` | `[[0x1a58dc]+1932]` |
| `ldr` at 0x29364 | load | 4 | `abs 0x293ec` | `-` |
| `ldr` at 0x29368 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x2936c | load | 4 | `abs 0x293ec` | `-` |
| `ldr` at 0x29370 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x29374 | load | 4 | `[0x1a58dc] +0x78c` | `-` |
| `str` at 0x29378 | store | 4 | `[0x1a58dc] +0x798` | `[[0x1a58dc]+1932]` |
| `ldr` at 0x2937c | load | 4 | `abs 0x293ec` | `-` |
| `ldr` at 0x29380 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x29384 | load | 4 | `abs 0x293f0` | `-` |
| `ldr` at 0x29388 | load | 4 | `abs 0x293ec` | `-` |
| `ldr` at 0x2938c | load | 4 | `abs 0x1a58dc` | `-` |
| `strb` at 0x293a4 | store | 1 | `[0x1a58dc] +0x7ab` | `(ret(0x52800) & 0xff)` |
| `ldr` at 0x293a8 | load | 4 | `abs 0x293ec` | `-` |
| `ldr` at 0x293ac | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x293b8 | load | 4 | `abs 0x293ec` | `-` |
| `ldr` at 0x293bc | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x293dc | load | 4 | `abs 0x293f4` | `-` |
| `ldr` at 0x293e0 | load | 4 | `abs 0x1a58ec` | `-` |
| `ldm` at 0x293e8 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x293e8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x293e8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x293e8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x293e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x293a0 | `(unknown callee) 0x52800` | `[0x1a58dc]+1928` | `0x24` | `[0x1a58dc]` | `[0x1a58dc]+1928` |
| `bl` at 0x293d4 | `(unknown callee) 0x203f4` | `[0x1a58dc]+1964` | `[0x1a58dc]+1928` | `0x24` | `[0x1a58dc]+1928` |
| `bl` at 0x293d8 | `(unknown callee) 0x249d4` | `ret(0x203f4)` | `?` | `?` | `?` |
| `bl` at 0x293e4 | `(unknown callee) 0x51c0c` | `[0x1a58ec]` | `?` | `?` | `0x1a58ec` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x293e8 | `r0` | `ret(0x51c0c)` |

## `nvramLogFinsert`

- `nvramLogFinsert`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x29908 to 0x29950.
  Established. The unnamed `0x20034` call carries `sp-156`, entry `r0`, `sp-12` and `sp-156` (`bl` at 0x29934).
  A following unnamed `0x293f8` call carries `sp-156`, two unknown candidates and `sp-156` (`bl` at 0x29940).
  The integer return remains unknown (`ldm` at 0x2994c).
  Formatting and persistent insertion semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x29920 | store | 4 | `sp -0x24` | `sp-12` |
| `ldr` at 0x2992c | load | 4 | `sp -0x10` | `-` |
| `ldr` at 0x29930 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x2994c | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x2994c | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x2994c | load | 4 | `sp -0x18` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x29934 | `(unknown callee) 0x20034` | `sp-156` | `r0` | `sp-12` | `sp-156` |
| `bl` at 0x29940 | `(unknown callee) 0x293f8` | `sp-156` | `?` | `?` | `sp-156` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x2994c | `r0` | `?` |
