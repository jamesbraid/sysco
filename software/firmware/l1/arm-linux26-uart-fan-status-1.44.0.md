# ARM Linux 2.6 UART and fan status in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-uart-fan-status-facts.txt) retain all tool sections.

## `uartClose`

- `uartClose`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10f2bc to 0x10f4b8.
  Established. Separate word assignments clear `[sp-24] +0x34` and `[sp-24] +0x38` (`str` at 0x10f354, `str` at 0x10f390).
  Another path clears both locations (`str` at 0x10f3e8, `str` at 0x10f3f4).
  Two unnamed `0x200b8` calls have four unknown printed argument candidates each (`bl` at 0x10f470, `bl` at 0x10f498).
  The closed hardware resources and effects of unsupported assignments remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10f2cc | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x10f2d0 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x10f2d8 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x10f2e0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x10f2e4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x10f2ec | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x10f2f0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x10f2fc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x10f308 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x10f318 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x10f328 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x10f334 | load | 4 | `r0 +0x34` | `-` |
| `str` at 0x10f344 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x10f34c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x10f354 | store | 4 | `[sp-24] +0x34` | `0x0` |
| `str` at 0x10f35c | store | 4 | `sp -0x2c` | `0x4` |
| `ldr` at 0x10f364 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x10f370 | load | 4 | `r0 +0x38` | `-` |
| `str` at 0x10f380 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x10f388 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x10f390 | store | 4 | `[sp-24] +0x38` | `0x0` |
| `str` at 0x10f398 | store | 4 | `sp -0x2c` | `0x2` |
| `ldr` at 0x10f3a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x10f3ac | load | 4 | `r0 +0x34` | `-` |
| `ldr` at 0x10f3b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x10f3c4 | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0x10f3d8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x10f3e0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x10f3e8 | store | 4 | `[sp-24] +0x34` | `0x0` |
| `ldr` at 0x10f3ec | load | 4 | `sp -0x18` | `-` |
| `str` at 0x10f3f4 | store | 4 | `[sp-24] +0x38` | `0x0` |
| `str` at 0x10f3fc | store | 4 | `sp -0x2c` | `0x6` |
| `ldr` at 0x10f400 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x10f404 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x10f410 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x10f414 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x10f420 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x10f424 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x10f430 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x10f434 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x10f43c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x10f440 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x10f448 | store | 4 | `[sp-24] +0x4` | `?` |
| `str` at 0x10f450 | store | 4 | `[sp-24] +0x0` | `?` |
| `ldr` at 0x10f458 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x10f468 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x10f46c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x10f474 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x10f47c | store | 4 | `[sp-24] +0x0` | `?` |
| `ldr` at 0x10f480 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x10f490 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x10f494 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x10f49c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x10f4a4 | store | 4 | `[sp-24] +0x4` | `?` |
| `ldr` at 0x10f4a8 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x10f4b4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x10f4b4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10f4b4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10f4b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10f32c | `(unknown callee) 0x228ec` | `r0` | `r1` | `r2` | `[sp-48]` |
| `bl` at 0x10f368 | `(unknown callee) 0x228ec` | `r0` | `r1` | `r2` | `[sp-48]` |
| `bl` at 0x10f3a4 | `(unknown callee) 0x228ec` | `r0` | `r1` | `r2` | `[sp-48]` |
| `bl` at 0x10f3bc | `(unknown callee) 0x228ec` | `ret(0x228ec)` | `?` | `ret(0x228ec)` | `[r0+52]` |
| `bl` at 0x10f438 | `(unknown callee) 0x10fc80` | `?` | `?` | `?` | `?` |
| `bl` at 0x10f470 | `(unknown callee) 0x200b8` | `?` | `?` | `?` | `?` |
| `bl` at 0x10f498 | `(unknown callee) 0x200b8` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x10f2f8 | `[sp-48] == 0x1` | `[sp-48] != 0x1` |
| `bgt` at 0x10f304 | `[sp-48] > 0x1 (signed, 32-bit)` | `[sp-48] <= 0x1 (signed, 32-bit)` |
| `beq` at 0x10f310 | `[sp-48] == 0x0` | `[sp-48] != 0x0` |
| `beq` at 0x10f320 | `[sp-48] == 0x2` | `[sp-48] != 0x2` |
| `beq` at 0x10f33c | `[r0+52] == ret(0x228ec)` | `[r0+52] != ret(0x228ec)` |
| `beq` at 0x10f378 | `[r0+56] == ret(0x228ec)` | `[r0+56] != ret(0x228ec)` |
| `bne` at 0x10f3b4 | `[r0+52] != ret(0x228ec)` | `[r0+52] == ret(0x228ec)` |
| `bne` at 0x10f3cc | `[[sp-24]+56] != ret(0x228ec)` | `[[sp-24]+56] == ret(0x228ec)` |
| `bne` at 0x10f40c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x10f41c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x10f42c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x10f464 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x10f48c | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10f4b4 | `r0` | `?` |

## `uartWrite`

- `uartWrite`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10f690 to 0x10f724.
  Established. An included word read addresses entry `r0 +0x4` (`ldr` at 0x10f6c8).
  The unnamed `0x2007c` call carries `[r0+4]`, entry `r1`, entry `r2` and entry `r0` (`bl` at 0x10f6d4);
  its signed nonnegative result takes `0x10f710`, while negative falls through (`bge` at 0x10f6e8).
  The fall-through path includes an unnamed `0x10f7f8` call carrying `[sp-20]`, `0x2`, unknown and `ret(0x2007c)` (`bl` at 0x10f6f4).
  UART transmit effects and error encoding remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10f6a0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x10f6a4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x10f6a8 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x10f6ac | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x10f6bc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x10f6c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x10f6c8 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0x10f6cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x10f6d0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x10f6dc | store | 4 | `sp -0x20` | `ret(0x2007c)` |
| `ldr` at 0x10f6e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x10f6ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x10f700 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `str` at 0x10f708 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x10f710 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x10f714 | store | 4 | `sp -0x24` | `ret(0x2007c)` |
| `ldr` at 0x10f718 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x10f720 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10f720 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10f720 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10f6d4 | `(unknown callee) 0x2007c` | `[r0+4]` | `r1` | `r2` | `r0` |
| `bl` at 0x10f6f4 | `(unknown callee) 0x10f7f8` | `[sp-20]` | `0x2` | `?` | `ret(0x2007c)` |
| `bl` at 0x10f6f8 | `(unknown callee) 0x20184` | `ret(0x10f7f8)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0x10f6b4 | `r2 > 0x0 (signed, 32-bit)` | `r2 <= 0x0 (signed, 32-bit)` |
| `bge` at 0x10f6e8 | `ret(0x2007c) >= 0x0 (signed, 32-bit)` | `ret(0x2007c) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10f720 | `r0` | `?` |

## `uartGetConfig`

- `uartGetConfig`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10f724 to 0x10f764.
  Established. Word reads address entry `r0` offsets `+0x10`, `+0x14`, `+0x18` and `+0x1c` (`ldm` at 0x10f74c).
  Word assignments write the corresponding loaded values to entry `r1` offsets `+0x0`, `+0x4`, `+0x8` and `+0xc` (`stm` at 0x10f750).
  Further word reads address entry `r0` offsets `+0x20`, `+0x24` and `+0x28`, and word assignments write those loaded values to entry `r1` offsets `+0x10`, `+0x14` and `+0x18` (`ldm` at 0x10f754, `stm` at 0x10f758).
  Configuration field meanings and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10f734 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x10f738 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x10f73c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x10f740 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x10f74c | load | 4 | `r0 +0x10` | `-` |
| `ldm` at 0x10f74c | load | 4 | `r0 +0x14` | `-` |
| `ldm` at 0x10f74c | load | 4 | `r0 +0x18` | `-` |
| `ldm` at 0x10f74c | load | 4 | `r0 +0x1c` | `-` |
| `stm` at 0x10f750 | store | 4 | `r1 +0x0` | `[r0+16]` |
| `stm` at 0x10f750 | store | 4 | `r1 +0x4` | `[r0+20]` |
| `stm` at 0x10f750 | store | 4 | `r1 +0x8` | `[r0+24]` |
| `stm` at 0x10f750 | store | 4 | `r1 +0xc` | `[r0+28]` |
| `ldm` at 0x10f754 | load | 4 | `r0 +0x20` | `-` |
| `ldm` at 0x10f754 | load | 4 | `r0 +0x24` | `-` |
| `ldm` at 0x10f754 | load | 4 | `r0 +0x28` | `-` |
| `stm` at 0x10f758 | store | 4 | `r1 +0x10` | `[r0+32]` |
| `stm` at 0x10f758 | store | 4 | `r1 +0x14` | `[r0+36]` |
| `stm` at 0x10f758 | store | 4 | `r1 +0x18` | `[r0+40]` |
| `ldm` at 0x10f760 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10f760 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10f760 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10f760 | `r0` | `[r0+32]` |

## `uartSetConfig`

- `uartSetConfig`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10f764 to 0x10f7f8.
  Established. An included byte read addresses entry `r1 +0x16` (`ldrb` at 0x10f780).
  The condition `[r1+22]:1 != 0x1` takes `0x10f798`, while equality falls through (`bne` at 0x10f788).
  The fall-through path assigns `0x2` to the byte at entry `r1 +0x16` (`strb` at 0x10f794).
  UART configuration field meanings and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10f774 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x10f778 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x10f77c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x10f780 | load | 1 | `r1 +0x16` | `-` |
| `ldr` at 0x10f78c | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x10f794 | store | 1 | `r1 +0x16` | `0x2` |
| `ldr` at 0x10f798 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x10f79c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x10f7a8 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x10f7b0 | store | 1 | `(unknown)` | `0x2` |
| `ldr` at 0x10f7b4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x10f7b8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x10f7c4 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x10f7cc | store | 1 | `(unknown)` | `0x2` |
| `ldr` at 0x10f7d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x10f7d4 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x10f7e0 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x10f7e0 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x10f7e0 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x10f7e0 | load | 4 | `(unknown)` | `-` |
| `stm` at 0x10f7e4 | store | 4 | `(unknown)` | `?` |
| `stm` at 0x10f7e4 | store | 4 | `(unknown)` | `?` |
| `stm` at 0x10f7e4 | store | 4 | `(unknown)` | `?` |
| `stm` at 0x10f7e4 | store | 4 | `(unknown)` | `?` |
| `ldm` at 0x10f7e8 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x10f7e8 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x10f7e8 | load | 4 | `(unknown)` | `-` |
| `stm` at 0x10f7ec | store | 4 | `(unknown)` | `?` |
| `stm` at 0x10f7ec | store | 4 | `(unknown)` | `?` |
| `stm` at 0x10f7ec | store | 4 | `(unknown)` | `?` |
| `ldm` at 0x10f7f4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10f7f4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10f7f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x10f788 | `[r1+22]:1 != 0x1` | `[r1+22]:1 == 0x1` |
| `bne` at 0x10f7a4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x10f7c0 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10f7f4 | `r0` | `?` |

## `uartFlush`

- `uartFlush`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10f7f8 to 0x10f8a4.
  Established. Separate word reads address entry `r0 +0x4` and entry `r0 +0x0` (`ldr` at 0x10f824, `ldr` at 0x10f834).
  The unnamed `0x20520` call carries `[sp-32]`, `0x80047502`, entry `r1` and `[sp-32]` (`bl` at 0x10f850);
  its signed nonnegative result takes `0x10f890`, while negative falls through (`bge` at 0x10f85c).
  UART flush request semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10f808 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x10f80c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x10f810 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x10f820 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x10f824 | load | 4 | `r0 +0x4` | `-` |
| `str` at 0x10f828 | store | 4 | `sp -0x20` | `[r0+4]` |
| `ldr` at 0x10f830 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x10f834 | load | 4 | `r0 +0x0` | `-` |
| `str` at 0x10f838 | store | 4 | `sp -0x20` | `[r0]` |
| `ldr` at 0x10f83c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x10f840 | store | 4 | `sp -0x1c` | `[sp-32]` |
| `ldr` at 0x10f844 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x10f848 | load | 4 | `abs 0x10f898` | `-` |
| `ldr` at 0x10f84c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x10f868 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x10f874 | load | 4 | `abs 0x10f89c` | `-` |
| `ldr` at 0x10f878 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x10f87c | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x10f880 | load | 4 | `abs 0x10f8a0` | `-` |
| `ldr` at 0x10f884 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldm` at 0x10f894 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10f894 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10f894 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10f850 | `(unknown callee) 0x20520` | `[sp-32]` | `0x80047502` | `r1` | `[sp-32]` |
| `bl` at 0x10f860 | `(unknown callee) 0x20184` | `ret(0x20520)` | `?` | `?` | `ret(0x20520)` |
| `bl` at 0x10f86c | `(unknown callee) 0x219a4` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x10f88c | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x18d968` | `[[sp-20]+8]` | `ret(0x219a4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x10f81c | `? == 0x0` | `? != 0x0` |
| `bge` at 0x10f85c | `ret(0x20520) >= 0x0 (signed, 32-bit)` | `ret(0x20520) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10f894 | `r0` | `?` |

## `uartSendBreak`

- `uartSendBreak`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10f8a4 to 0x10f920.
  Established. The unnamed `0x20520` call carries `[r0+4]`, `0x80047504`, `0x0` and entry `r0` (`bl` at 0x10f8cc);
  its signed nonnegative result takes `0x10f90c`, while negative falls through (`bge` at 0x10f8d8).
  A word read on the fall-through path addresses `0x1a288c` (`ldr` at 0x10f8f8).
  Break signaling and duration remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10f8b4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x10f8b8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x10f8bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x10f8c0 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0x10f8c4 | load | 4 | `abs 0x10f914` | `-` |
| `ldr` at 0x10f8e4 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x10f8f0 | load | 4 | `abs 0x10f918` | `-` |
| `ldr` at 0x10f8f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x10f8f8 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x10f8fc | load | 4 | `abs 0x10f91c` | `-` |
| `ldr` at 0x10f900 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldm` at 0x10f910 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10f910 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10f910 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10f8cc | `(unknown callee) 0x20520` | `[r0+4]` | `0x80047504` | `0x0` | `r0` |
| `bl` at 0x10f8dc | `(unknown callee) 0x20184` | `ret(0x20520)` | `?` | `?` | `ret(0x20520)` |
| `bl` at 0x10f8e8 | `(unknown callee) 0x219a4` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x10f908 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x18d990` | `[[sp-20]+8]` | `ret(0x219a4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x10f8d8 | `ret(0x20520) >= 0x0 (signed, 32-bit)` | `ret(0x20520) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10f910 | `r0` | `?` |

## `uartShowStatus`

- `uartShowStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10f920 to 0x10fa20.
  Established. Halfword reads address `[sp-44]` offsets `+0x9c`, `+0x9e`, `+0xa0` and `+0x104` (`ldrh` at 0x10f9bc, `ldrh` at 0x10f9c8, `ldrh` at 0x10f9d4, `ldrh` at 0x10f9ec).
  The unnamed `0x205bc` call carries `[sp-40]`, `0x18d9bc`, `[sp-36]` and `[[sp-44]+44]` (`bl` at 0x10fa04).
  A later word assignment clears `sp -0x30` (`str` at 0x10fa0c).
  UART status field meanings and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10f930 | store | 4 | `sp -0x20` | `r0` |
| `str` at 0x10f934 | store | 4 | `sp -0x24` | `r1` |
| `str` at 0x10f938 | store | 4 | `sp -0x28` | `r2` |
| `ldr` at 0x10f93c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x10f948 | store | 4 | `sp -0x2c` | `ret(0x10fa20)` |
| `ldr` at 0x10f94c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x10f95c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x10f964 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x10f978 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x10f97c | load | 4 | `[sp-44] +0x98` | `-` |
| `ldr` at 0x10f988 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x10f99c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x10f9a0 | load | 4 | `[sp-44] +0x100` | `-` |
| `ldr` at 0x10f9ac | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x10f9b0 | store | 4 | `sp -0x50` | `ret(0x10fad4)` |
| `str` at 0x10f9b4 | store | 4 | `sp -0x4c` | `ret(0x10faf8)` |
| `ldr` at 0x10f9b8 | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0x10f9bc | load | 2 | `[sp-44] +0x9c` | `-` |
| `str` at 0x10f9c0 | store | 4 | `sp -0x48` | `[[sp-44]+156]:2` |
| `ldr` at 0x10f9c4 | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0x10f9c8 | load | 2 | `[sp-44] +0x9e` | `-` |
| `str` at 0x10f9cc | store | 4 | `sp -0x44` | `[[sp-44]+158]:2` |
| `ldr` at 0x10f9d0 | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0x10f9d4 | load | 2 | `[sp-44] +0xa0` | `-` |
| `str` at 0x10f9d8 | store | 4 | `sp -0x40` | `[[sp-44]+160]:2` |
| `str` at 0x10f9dc | store | 4 | `sp -0x3c` | `ret(0x10fad4)` |
| `str` at 0x10f9e0 | store | 4 | `sp -0x38` | `ret(0x10faf8)` |
| `ldr` at 0x10f9e4 | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0x10f9ec | load | 2 | `[sp-44] +0x104` | `-` |
| `str` at 0x10f9f0 | store | 4 | `sp -0x34` | `[[sp-44]+260]:2` |
| `ldr` at 0x10f9f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x10f9f8 | load | 4 | `abs 0x10fa1c` | `-` |
| `ldr` at 0x10f9fc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x10fa00 | load | 4 | `[sp-44] +0x2c` | `-` |
| `str` at 0x10fa0c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x10fa10 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x10fa18 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x10fa18 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x10fa18 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x10fa18 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10fa18 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10fa18 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10f940 | `(unknown callee) 0x10fa20` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x10f970 | `(unknown callee) 0x10fad4` | `ret(0x10fa20)+60` | `?` | `?` | `ret(0x10fa20)+60` |
| `bl` at 0x10f980 | `(unknown callee) 0x10faf8` | `[[sp-44]+152]` | `?` | `?` | `[sp-44]` |
| `bl` at 0x10f994 | `(unknown callee) 0x10fad4` | `[sp-44]+164` | `?` | `?` | `[sp-44]+164` |
| `bl` at 0x10f9a4 | `(unknown callee) 0x10faf8` | `[[sp-44]+256]` | `?` | `?` | `[sp-44]` |
| `bl` at 0x10fa04 | `(unknown callee) 0x205bc` | `[sp-40]` | `0x18d9bc` | `[sp-36]` | `[[sp-44]+44]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x10f954 | `ret(0x10fa20) != 0x0` | `ret(0x10fa20) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10fa18 | `r0` | `[sp-48]` |

## `fanStatus`

- `fanStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6369c to 0x638f4.
  Established. The signed condition entry `r0 >= [0x1a5c94]` takes `0x636f8`, while lower falls through (`bge` at 0x636c8).
  One word assignment places `0xfffffa85` at `sp -0x3c` (`str` at 0x636fc).
  The unnamed `0x646d4` call carries `[sp-20]`, `[sp-21]:1`, `sp-29` and `sp-30` (`bl` at 0x63810).
  A later unsigned condition `[sp-56] <= 0x64` takes `0x638b8`, while higher falls through (`bls` at 0x638a8).
  Fan status encoding and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x636ac | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x636b4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x636b8 | load | 4 | `abs 0x638dc` | `-` |
| `ldr` at 0x636bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x636c0 | load | 4 | `abs 0x1a5c94` | `-` |
| `ldr` at 0x636cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x636e0 | load | 4 | `abs 0x638e0` | `-` |
| `ldr` at 0x636e4 | load | 4 | `abs 0x1a5c90` | `-` |
| `ldrb` at 0x636ec | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x636f8 | load | 4 | `abs 0x638e4` | `-` |
| `str` at 0x636fc | store | 4 | `sp -0x3c` | `0xfffffa85` |
| `ldr` at 0x63704 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x63718 | load | 4 | `abs 0x638e0` | `-` |
| `ldr` at 0x6371c | load | 4 | `abs 0x1a5c90` | `-` |
| `str` at 0x63724 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x63728 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x6372c | load | 1 | `[sp-44] +0x4` | `-` |
| `ldr` at 0x63738 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6374c | store | 4 | `sp -0x30` | `ret(0x6e1b0)` |
| `ldr` at 0x63750 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x63760 | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x63764 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x6378c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x63790 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x63794 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x63798 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x637a4 | load | 4 | `abs 0x638e8` | `-` |
| `str` at 0x637a8 | store | 4 | `(unknown)` | `0x163018` |
| `str` at 0x637ac | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `str` at 0x637b4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x637b8 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x637bc | load | 1 | `[sp-44] +0x4` | `-` |
| `str` at 0x637c0 | store | 4 | `(unknown)` | `[[sp-44]+4]:1` |
| `ldr` at 0x637c8 | load | 4 | `abs 0x638ec` | `-` |
| `ldr` at 0x637cc | load | 4 | `abs 0x638f0` | `-` |
| `ldr` at 0x637d8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x637f0 | store | 4 | `sp -0x3c` | `0x0` |
| `ldrb` at 0x637f8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x63804 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x63818 | store | 4 | `sp -0x24` | `ret(0x646d4)` |
| `ldr` at 0x6381c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x63828 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6382c | store | 4 | `sp -0x3c` | `ret(0x646d4)` |
| `ldrb` at 0x63834 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x63838 | store | 1 | `sp -0x1f` | `([sp-29]:1 & 0xff)` |
| `ldrb` at 0x6383c | load | 1 | `sp -0x1f` | `-` |
| `ldrb` at 0x63840 | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x6384c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x63850 | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x63854 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldrb` at 0x6385c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x6386c | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x63874 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x63884 | store | 4 | `sp -0x24` | `ret(0x64238)` |
| `ldr` at 0x63888 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x63894 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x63898 | store | 4 | `sp -0x3c` | `ret(0x64238)` |
| `ldr` at 0x638a0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x638b0 | store | 4 | `sp -0x3c` | `0x1` |
| `ldrb` at 0x638b8 | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0x638c0 | store | 1 | `sp -0x1f` | `?` |
| `str` at 0x638cc | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x638d0 | load | 4 | `sp -0x3c` | `-` |
| `ldm` at 0x638d8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x638d8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x638d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x63748 | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x63754 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x6379c | `(unknown callee) 0x203f4` | `[sp-56]` | `[sp-48]` | `[sp-52]` | `?` |
| `bl` at 0x637d4 | `(unknown callee) 0x22070` | `0x9` | `0x162f88` | `0x212` | `0x0` |
| `bl` at 0x637dc | `(unknown callee) 0x558d8` | `?` | `?` | `?` | `?` |
| `bl` at 0x63810 | `(unknown callee) 0x646d4` | `[sp-20]` | `[sp-21]:1` | `sp-29` | `sp-30` |
| `bl` at 0x6387c | `(unknown callee) 0x64238` | `[sp-20]` | `?` | `sp-56` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x636c8 | `r0 >= [0x1a5c94] (signed, 32-bit)` | `r0 < [0x1a5c94] (signed, 32-bit)` |
| `bne` at 0x636f4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x63734 | `[[sp-44]+4]:1 != 0x0` | `[[sp-44]+4]:1 == 0x0` |
| `bne` at 0x637e8 | `ret(0x558d8) != 0x0` | `ret(0x558d8) == 0x0` |
| `beq` at 0x63824 | `ret(0x646d4) == 0x0` | `ret(0x646d4) != 0x0` |
| `bhs` at 0x63848 | `C == 1` | `not (C == 1)` |
| `bne` at 0x63864 | `Z == 0` | `not (Z == 0)` |
| `bge` at 0x63890 | `ret(0x64238) >= 0x0 (signed, 32-bit)` | `ret(0x64238) < 0x0 (signed, 32-bit)` |
| `bls` at 0x638a8 | `[sp-56] <= 0x64 (unsigned, 32-bit)` | `[sp-56] > 0x64 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x638d8 | `r0` | `?` |

## `fanStatusAll`

- `fanStatusAll`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x638f4 to 0x639ec.
  Established. The unnamed `0x6369c` call carries `[sp-20]`, `0xff`, `ret(0x6e140)` and `[sp-20]` (`bl` at 0x63938).
  Equality `ret(0x6369c) == 0xfffffa85` takes `0x639c4`, while inequality falls through (`beq` at 0x63958).
  A word assignment increments `[sp-20]` by one at `sp -0x14` (`str` at 0x639cc).
  Aggregated fan status meaning and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x63908 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x63910 | store | 4 | `sp -0x1c` | `0x1` |
| `str` at 0x63918 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x63924 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x63930 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x63940 | store | 4 | `sp -0x20` | `ret(0x6369c)` |
| `ldr` at 0x63944 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x63948 | store | 4 | `sp -0x28` | `ret(0x6369c)` |
| `ldr` at 0x6394c | load | 4 | `abs 0x639e8` | `-` |
| `ldr` at 0x63950 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6395c | load | 4 | `abs 0x639e8` | `-` |
| `ldr` at 0x63960 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6396c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x63980 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x63984 | store | 4 | `sp -0x24` | `ret(0x6369c)` |
| `ldr` at 0x6398c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6399c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x639a0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x639a4 | store | 4 | `sp -0x18` | `ret(0x6369c)` |
| `ldr` at 0x639ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x639b0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x639c0 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x639c4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x639cc | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0x639d4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x639d8 | store | 4 | `sp -0x24` | `[sp-24]` |
| `ldr` at 0x639dc | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x639e4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x639e4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x639e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6391c | `(unknown callee) 0x6e140` | `?` | `?` | `?` | `?` |
| `bl` at 0x63938 | `(unknown callee) 0x6369c` | `[sp-20]` | `0xff` | `ret(0x6e140)` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x6392c | `ret(0x6e140) <= [sp-20] (signed, 32-bit)` | `ret(0x6e140) > [sp-20] (signed, 32-bit)` |
| `beq` at 0x63958 | `ret(0x6369c) == 0xfffffa85` | `ret(0x6369c) != 0xfffffa85` |
| `blt` at 0x63968 | `ret(0x6369c) < 0xfffffa85 (signed, 32-bit)` | `ret(0x6369c) >= 0xfffffa85 (signed, 32-bit)` |
| `bhi` at 0x63978 | `ret(0x6369c) > 0x1 (unsigned, 32-bit)` | `ret(0x6369c) <= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x63994 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `beq` at 0x639b8 | `[sp-24] == ret(0x6369c)` | `[sp-24] != ret(0x6369c)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x639e4 | `r0` | `?` |

## `fanLocation`

- `fanLocation`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x64414 to 0x64480.
  Established. The unnamed `0x64480` call carries entry `r0`, `(r1 & 0xff)`, entry `r2` and `(r1 & 0xff)` (`bl` at 0x6443c).
  Included word reads address `ret(0x64480) +0x4` (`ldr` at 0x6444c, `ldr` at 0x64458).
  Nonzero `[ret(0x64480)+4]` takes `0x6446c`, while zero falls through (`bne` at 0x64460).
  The fall-through path assigns `0x1631e8` to `sp -0x20`, and the hardware location represented remains Unknown (`str` at 0x64468).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x64424 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x6442c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x64430 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x64434 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x64444 | store | 4 | `sp -0x1c` | `ret(0x64480)` |
| `ldr` at 0x64448 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6444c | load | 4 | `ret(0x64480) +0x4` | `-` |
| `str` at 0x64450 | store | 4 | `sp -0x20` | `[ret(0x64480)+4]` |
| `ldr` at 0x64454 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x64458 | load | 4 | `ret(0x64480) +0x4` | `-` |
| `ldr` at 0x64464 | load | 4 | `abs 0x6447c` | `-` |
| `str` at 0x64468 | store | 4 | `sp -0x20` | `0x1631e8` |
| `ldr` at 0x6446c | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x64478 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x64478 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x64478 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6443c | `(unknown callee) 0x64480` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x64460 | `[ret(0x64480)+4] != 0x0` | `[ret(0x64480)+4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x64478 | `r0` | `?` |

## `vsc055_FanPWMInit`

- `vsc055_FanPWMInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1291a8 to 0x129264.
  Established. An unnamed `0x12576c` call carries entry `r0`, `sp-32`, entry `r2` and `sp-32` (`bl` at 0x1291e4).
  The unnamed `0x10a4ec` call carries `[sp-20]`, `[sp-21]:1`, `[sp-41]:1` and `[sp-21]:1` (`bl` at 0x129210).
  A later unnamed `0x10adc4` call carries unknown, `[sp-22]:1`, unknown and `[sp-41]:1`, and its result is the integer return candidate (`bl` at 0x129248, `ldm` at 0x129260).
  PWM initialization register identity and physical fan effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1291b8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x1291bc | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0x1291c4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x1291cc | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x1291d4 | store | 1 | `sp -0x29` | `0x0` |
| `ldr` at 0x1291dc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1291ec | store | 4 | `sp -0x24` | `ret(0x12576c)` |
| `ldr` at 0x1291f0 | load | 4 | `sp +0x0` | `-` |
| `ldrb` at 0x1291f8 | load | 1 | `sp -0x29` | `-` |
| `strb` at 0x129200 | store | 1 | `sp -0x29` | `?` |
| `ldrb` at 0x129204 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x129208 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x12921c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x129220 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x129238 | load | 1 | `sp -0x29` | `-` |
| `str` at 0x129250 | store | 4 | `sp -0x28` | `ret(0x10adc4)` |
| `ldr` at 0x129254 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x129260 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x129260 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x129260 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1291e4 | `(unknown callee) 0x12576c` | `r0` | `sp-32` | `r2` | `sp-32` |
| `bl` at 0x129210 | `(unknown callee) 0x10a4ec` | `[sp-20]` | `[sp-21]:1` | `[sp-41]:1` | `[sp-21]:1` |
| `bl` at 0x129248 | `(unknown callee) 0x10adc4` | `?` | `[sp-22]:1` | `?` | `[sp-41]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x129260 | `r0` | `ret(0x10adc4)` |

## `vsc055_FanMonitorInit`

- `vsc055_FanMonitorInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x129264 to 0x129378.
  Established. An unnamed `0x12576c` call carries entry `r0`, `sp-32`, entry `r2` and `sp-32` (`bl` at 0x1292a0).
  One unnamed `0x10adc4` call carries unknown, `[sp-22]:1`, `[sp-41]:1` and `0xff` (`bl` at 0x1292ec).
  A later call to the same unnamed target carries unknown, `[sp-22]:1`, `[sp-41]:1` and `[sp-42]:1`, and its result is the integer return candidate (`bl` at 0x12935c, `ldm` at 0x129374).
  Monitor initialization register identity and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x129274 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x129278 | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0x129280 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x129288 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x129290 | store | 1 | `sp -0x2a` | `0x0` |
| `ldr` at 0x129298 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1292a8 | store | 4 | `sp -0x24` | `ret(0x12576c)` |
| `ldr` at 0x1292ac | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x1292b8 | store | 1 | `sp -0x29` | `?` |
| `ldrb` at 0x1292bc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x1292c0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x1292d4 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x1292d8 | load | 1 | `sp -0x29` | `-` |
| `str` at 0x1292f4 | store | 4 | `sp -0x28` | `ret(0x10adc4)` |
| `ldrb` at 0x1292f8 | load | 1 | `sp -0x2a` | `-` |
| `strb` at 0x129304 | store | 1 | `sp -0x2a` | `?` |
| `ldr` at 0x129308 | load | 4 | `sp +0x0` | `-` |
| `ldrb` at 0x12930c | load | 1 | `sp -0x2a` | `-` |
| `strb` at 0x129314 | store | 1 | `sp -0x2a` | `?` |
| `ldr` at 0x129318 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x129324 | store | 1 | `sp -0x29` | `?` |
| `ldrb` at 0x129328 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x12932c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x129340 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x129344 | load | 1 | `sp -0x29` | `-` |
| `ldrb` at 0x129348 | load | 1 | `sp -0x2a` | `-` |
| `str` at 0x129364 | store | 4 | `sp -0x28` | `ret(0x10adc4)` |
| `ldr` at 0x129368 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x129374 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x129374 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x129374 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1292a0 | `(unknown callee) 0x12576c` | `r0` | `sp-32` | `r2` | `sp-32` |
| `bl` at 0x1292c8 | `(unknown callee) 0x10a4ec` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x1292ec | `(unknown callee) 0x10adc4` | `?` | `[sp-22]:1` | `[sp-41]:1` | `0xff` |
| `bl` at 0x129334 | `(unknown callee) 0x10a4ec` | `[sp-20]` | `[sp-21]:1` | `[sp-42]:1` | `[sp-21]:1` |
| `bl` at 0x12935c | `(unknown callee) 0x10adc4` | `?` | `[sp-22]:1` | `[sp-41]:1` | `[sp-42]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x129374 | `r0` | `ret(0x10adc4)` |
