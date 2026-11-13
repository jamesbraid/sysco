# ARM chip register and selection paths in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package4/chip-register-selection-facts.txt) retain all tool sections.

## `JChip_MMR_Read`

- `JChip_MMR_Read`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xde16c to 0xde274.
  Established. The indirect call carries `[[sp-36]+25]:1`, unknown, `0x5`, `[sp-24]` (`mov` at 0xde22c).
  Equality `ret(unknown) == 0x1` takes `0xde248`, while inequality falls through (`beq` at 0xde238).
  An included word assignment places `0xfffff7c4` at `sp -0x28` (`str` at 0xde240).
  MMR read completion meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xde17c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xde180 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xde184 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xde188 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xde18c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xde198 | store | 4 | `sp -0x24` | `ret(0xdefac)` |
| `ldr` at 0xde19c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xde1a8 | load | 4 | `abs 0xde25c` | `-` |
| `str` at 0xde1ac | store | 4 | `sp -0x28` | `0xfffff7c6` |
| `ldr` at 0xde1b4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xde1c8 | load | 4 | `abs 0xde260` | `-` |
| `str` at 0xde1cc | store | 4 | `sp -0x28` | `0xfffff7c5` |
| `ldr` at 0xde1d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xde1d8 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xde1dc | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xde1e0 | load | 1 | `[sp-36] +0x19` | `-` |
| `ldr` at 0xde1e4 | load | 4 | `sp -0x24` | `-` |
| `ldrsb` at 0xde1e8 | load | 1 | `[sp-36] +0x21` | `-` |
| `ldr` at 0xde1ec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xde1f0 | store | 4 | `sp -0x3c` | `[sp-28]` |
| `ldr` at 0xde1f4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xde1f8 | store | 4 | `sp -0x38` | `[sp-32]` |
| `ldr` at 0xde1fc | load | 4 | `abs 0xde264` | `-` |
| `str` at 0xde200 | store | 4 | `sp -0x34` | `0x2973ac` |
| `ldr` at 0xde204 | load | 4 | `abs 0xde268` | `-` |
| `str` at 0xde208 | store | 4 | `sp -0x30` | `0x29742c` |
| `ldr` at 0xde20c | load | 4 | `abs 0xde26c` | `-` |
| `str` at 0xde210 | store | 4 | `sp -0x2c` | `0x29c6f8` |
| `ldr` at 0xde214 | load | 4 | `[[sp-36]] +0x14` | `-` |
| `ldr` at 0xde224 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xde23c | load | 4 | `abs 0xde270` | `-` |
| `str` at 0xde240 | store | 4 | `sp -0x28` | `0xfffff7c4` |
| `str` at 0xde24c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xde250 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xde258 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xde258 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xde258 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xde190 | `(unknown callee) 0xdefac` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xde1b8 | `(unknown callee) 0xdcd98` | `ret(0xdefac)` | `?` | `?` | `ret(0xdefac)` |
| `mov` at 0xde22c | `(unknown callee) (unknown target)` | `[[sp-36]+25]:1` | `?` | `0x5` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xde1a4 | `ret(0xdefac) != 0x0` | `ret(0xdefac) == 0x0` |
| `beq` at 0xde1c4 | `ret(0xdcd98) == 0x0` | `ret(0xdcd98) != 0x0` |
| `beq` at 0xde238 | `ret(unknown) == 0x1` | `ret(unknown) != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xde258 | `r0` | `?` |

## `JChip_MMR_Write`

- `JChip_MMR_Write`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xde274 to 0xde384.
  Established. The included indirect call carries `[[sp-40]+25]:1`, unknown, `[0x12da7c]`, `[sp-28]` (`mov` at 0xde338).
  Zero `ret(0xdcd98)` takes `0xde2dc`, while nonzero falls through (`beq` at 0xde2cc).
  A word assignment stores `0xfffff7c5` at `sp -0x2c` on an included path (`str` at 0xde2d4).
  MMR write error interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xde284 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xde288 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xde28c | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xde290 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0xde294 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xde2a0 | store | 4 | `sp -0x28` | `ret(0xdefac)` |
| `ldr` at 0xde2a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xde2b0 | load | 4 | `abs 0xde368` | `-` |
| `str` at 0xde2b4 | store | 4 | `sp -0x2c` | `0xfffff7c6` |
| `ldr` at 0xde2bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xde2d0 | load | 4 | `abs 0xde36c` | `-` |
| `str` at 0xde2d4 | store | 4 | `sp -0x2c` | `0xfffff7c5` |
| `ldr` at 0xde2dc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xde2e0 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0xde2e4 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xde2e8 | load | 1 | `[sp-40] +0x19` | `-` |
| `ldr` at 0xde2ec | load | 4 | `sp -0x28` | `-` |
| `ldrsb` at 0xde2f0 | load | 1 | `[sp-40] +0x21` | `-` |
| `ldr` at 0xde2f4 | load | 4 | `abs 0xde370` | `-` |
| `ldr` at 0xde2f8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xde2fc | store | 4 | `sp -0x40` | `[sp-32]` |
| `ldr` at 0xde300 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xde304 | store | 4 | `sp -0x3c` | `[sp-36]` |
| `ldr` at 0xde308 | load | 4 | `abs 0xde374` | `-` |
| `str` at 0xde30c | store | 4 | `sp -0x38` | `0x2973ac` |
| `ldr` at 0xde310 | load | 4 | `abs 0xde378` | `-` |
| `str` at 0xde314 | store | 4 | `sp -0x34` | `0x29742c` |
| `ldr` at 0xde318 | load | 4 | `abs 0xde37c` | `-` |
| `str` at 0xde31c | store | 4 | `sp -0x30` | `0x29c6f8` |
| `ldr` at 0xde320 | load | 4 | `[[sp-40]] +0x18` | `-` |
| `ldr` at 0xde32c | load | 4 | `abs 0x12da7c` | `-` |
| `ldr` at 0xde330 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xde348 | load | 4 | `abs 0xde380` | `-` |
| `str` at 0xde34c | store | 4 | `sp -0x2c` | `0xfffff7c4` |
| `str` at 0xde358 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xde35c | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0xde364 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xde364 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xde364 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xde364 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xde298 | `(unknown callee) 0xdefac` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xde2c0 | `(unknown callee) 0xdcd98` | `ret(0xdefac)` | `?` | `?` | `ret(0xdefac)` |
| `mov` at 0xde338 | `(unknown callee) (unknown target)` | `[[sp-40]+25]:1` | `?` | `[0x12da7c]` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xde2ac | `ret(0xdefac) != 0x0` | `ret(0xdefac) == 0x0` |
| `beq` at 0xde2cc | `ret(0xdcd98) == 0x0` | `ret(0xdcd98) != 0x0` |
| `beq` at 0xde344 | `ret(unknown) == 0x1` | `ret(unknown) != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xde364 | `r0` | `?` |

## `JChip_MMR_SC_f`

- `JChip_MMR_SC_f`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xde384 to 0xde978.
  Established. Unsigned `[0x29737c] <= 0x2` takes `0xde440`, while greater values fall through (`bls` at 0xde400).
  The unnamed `0xde274` call carries `[sp-28]`, `([sp-72] + [sp-76])`, `[sp-64]`, `[sp-68]` (`bl` at 0xde75c);
  its zero result takes `0xde790`, while nonzero falls through (`beq` at 0xde768).
  Scan command register mapping remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xde394 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xde39c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xde3a4 | load | 4 | `r0 +0xc` | `-` |
| `ldr` at 0xde3bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xde3c4 | load | 4 | `r0 +0xc` | `-` |
| `str` at 0xde3d0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xde3d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xde3d8 | load | 4 | `r0 +0x0` | `-` |
| `str` at 0xde3e4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xde3e8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xde3f0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xde3f4 | load | 4 | `abs 0xde960` | `-` |
| `ldr` at 0xde3f8 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xde408 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xde410 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0xde420 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xde428 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0xde434 | load | 4 | `abs 0xde964` | `-` |
| `ldr` at 0xde444 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xde44c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xde45c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xde464 | load | 4 | `(unknown)` | `-` |
| `str` at 0xde480 | store | 4 | `sp -0x1c` | `ret(0xdf1b0)` |
| `ldr` at 0xde484 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xde494 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xde49c | load | 4 | `abs 0xde960` | `-` |
| `ldr` at 0xde4a0 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xde4ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xde4b8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xde4bc | store | 4 | `sp -0x74` | `[sp-32]` |
| `ldr` at 0xde4c0 | load | 4 | `abs 0xde968` | `-` |
| `ldr` at 0xde4c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xde4cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xde4d4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xde4e4 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xde4e8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xde4fc | store | 4 | `sp -0x54` | `0x4` |
| `str` at 0xde508 | store | 4 | `sp -0x54` | `0x0` |
| `ldr` at 0xde50c | load | 4 | `sp -0x54` | `-` |
| `str` at 0xde510 | store | 4 | `sp -0x30` | `[sp-84]` |
| `str` at 0xde518 | store | 4 | `sp -0x34` | `0x2` |
| `ldr` at 0xde51c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xde530 | store | 4 | `sp -0x58` | `0x4` |
| `str` at 0xde53c | store | 4 | `sp -0x58` | `0x2` |
| `ldr` at 0xde540 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xde544 | store | 4 | `sp -0x38` | `[sp-88]` |
| `ldr` at 0xde548 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xde558 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xde55c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xde568 | store | 4 | `sp -0x5c` | `ret(0xeb174)` |
| `ldr` at 0xde570 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xde574 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xde57c | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0xde580 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xde584 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xde590 | store | 4 | `sp -0x5c` | `ret(0xeb174)` |
| `ldr` at 0xde594 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xde598 | store | 4 | `sp -0x3c` | `ret(0xeb174)` |
| `str` at 0xde5a4 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xde5a8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xde5bc | store | 4 | `sp -0x60` | `0x2` |
| `str` at 0xde5c8 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xde5cc | load | 4 | `sp -0x60` | `-` |
| `str` at 0xde5d0 | store | 4 | `sp -0x30` | `[sp-96]` |
| `str` at 0xde5d8 | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0xde5e0 | store | 4 | `sp -0x38` | `0x2` |
| `ldr` at 0xde5e4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xde5f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xde5f8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xde604 | store | 4 | `sp -0x64` | `ret(0xeb174)` |
| `str` at 0xde610 | store | 4 | `sp -0x64` | `0x1` |
| `ldr` at 0xde614 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xde618 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xde61c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xde62c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xde630 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xde634 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xde640 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xde644 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xde64c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xde65c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xde668 | load | 4 | `(unknown)` | `-` |
| `str` at 0xde66c | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xde670 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xde680 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xde688 | store | 4 | `sp -0x68` | `?` |
| `str` at 0xde694 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0xde698 | load | 4 | `sp -0x68` | `-` |
| `str` at 0xde69c | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xde6a0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xde6a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xde6ac | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xde6bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xde6c8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xde6cc | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xde6d0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xde6d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xde6dc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xde6ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xde6f8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xde6fc | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xde700 | load | 4 | `abs 0xde960` | `-` |
| `ldr` at 0xde704 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xde710 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xde71c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xde720 | store | 4 | `sp -0x74` | `[sp-64]` |
| `ldr` at 0xde724 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde728 | store | 4 | `sp -0x70` | `[sp-68]` |
| `ldr` at 0xde72c | load | 4 | `abs 0xde96c` | `-` |
| `ldr` at 0xde734 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xde738 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xde740 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xde744 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xde74c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xde754 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde758 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde76c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xde770 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xde774 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xde780 | store | 4 | `[sp-20] +0x0` | `?` |
| `str` at 0xde788 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xde790 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xde798 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0xde7a4 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xde7a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xde7ac | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xde7b8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xde7bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xde7c4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xde7d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xde7e0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xde7e4 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xde7e8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xde7f8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xde800 | store | 4 | `sp -0x6c` | `?` |
| `str` at 0xde80c | store | 4 | `sp -0x6c` | `0x0` |
| `ldr` at 0xde810 | load | 4 | `sp -0x6c` | `-` |
| `str` at 0xde814 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xde818 | load | 4 | `abs 0xde960` | `-` |
| `ldr` at 0xde81c | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xde828 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xde834 | load | 4 | `abs 0xde970` | `-` |
| `ldr` at 0xde83c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xde840 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xde848 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xde84c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xde85c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xde878 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xde87c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xde880 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xde88c | store | 4 | `[sp-20] +0x0` | `?` |
| `str` at 0xde894 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xde89c | load | 4 | `abs 0xde960` | `-` |
| `ldr` at 0xde8a0 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xde8ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xde8b8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xde8bc | store | 4 | `sp -0x74` | `[sp-64]` |
| `ldr` at 0xde8c0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde8c4 | store | 4 | `sp -0x70` | `[sp-68]` |
| `ldr` at 0xde8c8 | load | 4 | `abs 0xde974` | `-` |
| `ldr` at 0xde8d0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xde8d4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xde8dc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xde8e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xde8e8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xde8f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xde904 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xde908 | store | 4 | `(unknown)` | `[sp-64]` |
| `ldr` at 0xde90c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xde910 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xde918 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xde928 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xde934 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde938 | store | 4 | `(unknown)` | `[sp-68]` |
| `ldr` at 0xde93c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xde944 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0xde950 | store | 4 | `sp -0x50` | `0x1` |
| `ldr` at 0xde954 | load | 4 | `sp -0x50` | `-` |
| `ldm` at 0xde95c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xde95c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xde95c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xde43c | `(unknown callee) 0x221f0` | `0x122204` | `?` | `?` | `?` |
| `bl` at 0xde478 | `(unknown callee) 0xdf1b0` | `?` | `?` | `0x4` | `?` |
| `bl` at 0xde4b0 | `(unknown callee) 0xdde20` | `[sp-28]` | `?` | `?` | `[0x29737c]` |
| `bl` at 0xde4d0 | `(unknown callee) 0x221f0` | `0x122224` | `ret(0xdde20)` | `[sp-28]` | `[sp-36]` |
| `bl` at 0xde560 | `(unknown callee) 0xeb174` | `[sp-32]` | `[sp-88]` | `?` | `?` |
| `bl` at 0xde588 | `(unknown callee) 0xeb174` | `[sp-92]` | `[sp-56]` | `0x2` | `?` |
| `bl` at 0xde5fc | `(unknown callee) 0xeb174` | `[sp-32]` | `0x2` | `?` | `?` |
| `bl` at 0xde714 | `(unknown callee) 0xdde20` | `[sp-28]` | `0x14` | `?` | `[0x29737c]` |
| `bl` at 0xde73c | `(unknown callee) 0x221f0` | `0x12225c` | `ret(0xdde20)` | `[sp-72]` | `[sp-76]` |
| `bl` at 0xde75c | `(unknown callee) 0xde274` | `[sp-28]` | `([sp-72] + [sp-76])` | `[sp-64]` | `[sp-68]` |
| `bl` at 0xde82c | `(unknown callee) 0xdde20` | `[sp-28]` | `0x14` | `?` | `[0x29737c]` |
| `bl` at 0xde844 | `(unknown callee) 0x221f0` | `0x122294` | `ret(0xdde20)` | `[sp-76]` | `[sp-72]` |
| `bl` at 0xde868 | `(unknown callee) 0xde16c` | `[sp-28]` | `?` | `sp-64` | `sp-68` |
| `bl` at 0xde8b0 | `(unknown callee) 0xdde20` | `[sp-28]` | `?` | `?` | `[0x29737c]` |
| `bl` at 0xde8d8 | `(unknown callee) 0x221f0` | `0x1222c0` | `ret(0xdde20)` | `[sp-76]` | `[sp-72]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xde400 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xde48c | `Z == 0` | `not (Z == 0)` |
| `bls` at 0xde4a8 | `[0x29737c] <= 0x1 (unsigned, 32-bit)` | `[0x29737c] > 0x1 (unsigned, 32-bit)` |
| `beq` at 0xde4dc | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `beq` at 0xde4f4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xde528 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xde554 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xde5b4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xde5f0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xde624 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `bge` at 0xde63c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xde67c | `? == 0x0` | `? != 0x0` |
| `bls` at 0xde70c | `[0x29737c] <= 0x1 (unsigned, 32-bit)` | `[0x29737c] > 0x1 (unsigned, 32-bit)` |
| `beq` at 0xde768 | `ret(0xde274) == 0x0` | `ret(0xde274) != 0x0` |
| `bge` at 0xde7b4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xde7f4 | `? == 0x0` | `? != 0x0` |
| `bls` at 0xde824 | `[0x29737c] <= 0x1 (unsigned, 32-bit)` | `[0x29737c] > 0x1 (unsigned, 32-bit)` |
| `beq` at 0xde874 | `ret(0xde16c) == 0x0` | `ret(0xde16c) != 0x0` |
| `bls` at 0xde8a8 | `[0x29737c] <= 0x1 (unsigned, 32-bit)` | `[0x29737c] > 0x1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xde95c | `r0` | `?` |

## `JChip_MMR_SC_stub`

- `JChip_MMR_SC_stub`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdb8c0 to 0xdb8e0.
  Established. A word assignment preserves entry `r0` at `sp -0x14` (`str` at 0xdb8d0).
  The integer return candidate contains `0x0` (`ldm` at 0xdb8dc).
  MMR stub result meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdb8d0 | store | 4 | `sp -0x14` | `r0` |
| `ldm` at 0xdb8dc | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xdb8dc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdb8dc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdb8dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdb8dc | `r0` | `0x0` |

## `memory_access_function`

- `memory_access_function`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd8a64 to 0xd8d30.
  Established. An included word assignment clears `[sp-20] +0xc` (`str` at 0xd8cf4).
  The unnamed `0x57b08` call carries `0x1`, `[sp-24]`, unknown, unknown (`bl` at 0xd8cb0);
  its zero result takes `0xd8ce4`, while nonzero falls through (`beq` at 0xd8cc4).
  Memory command transfer constraints remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd8a74 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xd8a7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8a84 | load | 4 | `r0 +0xc` | `-` |
| `ldr` at 0xd8a98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8aa0 | load | 4 | `r0 +0xc` | `-` |
| `str` at 0xd8aa8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xd8aac | load | 4 | `sp -0x38` | `-` |
| `str` at 0xd8ab4 | store | 4 | `sp -0x38` | `[sp-56]-2` |
| `ldr` at 0xd8ac0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8ac8 | load | 4 | `r0 +0xc` | `-` |
| `str` at 0xd8ad0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xd8ad4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xd8adc | store | 4 | `sp -0x38` | `[sp-56]-1` |
| `ldr` at 0xd8ae0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xd8ae4 | store | 4 | `sp -0x20` | `[sp-56]` |
| `ldr` at 0xd8ae8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8aec | load | 4 | `r0 +0x0` | `-` |
| `str` at 0xd8af8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd8afc | load | 4 | `abs 0xd8d0c` | `-` |
| `ldr` at 0xd8b00 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xd8b10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8b18 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0xd8b28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8b30 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0xd8b3c | load | 4 | `abs 0xd8d10` | `-` |
| `ldr` at 0xd8b4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8b54 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd8b64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8b6c | load | 4 | `(unknown)` | `-` |
| `str` at 0xd8b88 | store | 4 | `sp -0x18` | `ret(0xdf1b0)` |
| `ldr` at 0xd8b8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd8b98 | load | 4 | `abs 0xd8d14` | `-` |
| `str` at 0xd8ba4 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xd8bac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd8bb8 | load | 4 | `abs 0xd8d18` | `-` |
| `str` at 0xd8bc4 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xd8bcc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd8bd8 | load | 4 | `abs 0xd8d1c` | `-` |
| `str` at 0xd8be4 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xd8bec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd8bfc | load | 4 | `abs 0xd8d20` | `-` |
| `str` at 0xd8c08 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xd8c14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8c1c | load | 4 | `[sp-20] +0x10` | `-` |
| `str` at 0xd8c20 | store | 4 | `sp -0x28` | `[[sp-20]+16]` |
| `ldr` at 0xd8c28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8c30 | load | 4 | `[sp-20] +0x14` | `-` |
| `str` at 0xd8c34 | store | 4 | `sp -0x2c` | `[[sp-20]+20]` |
| `ldr` at 0xd8c38 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd8c40 | store | 4 | `sp -0x30` | `[sp-20]+24` |
| `ldr` at 0xd8c44 | load | 4 | `abs 0xd8d0c` | `-` |
| `ldr` at 0xd8c48 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xd8c54 | load | 4 | `abs 0xd8d24` | `-` |
| `ldr` at 0xd8c58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd8c5c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd8c60 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd8c6c | store | 4 | `sp -0x4c` | `0xff` |
| `ldr` at 0xd8c70 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd8c74 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xd8c78 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd8c7c | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xd8c80 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd8c94 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xd8c98 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd8c9c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xd8ca4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd8cb8 | store | 4 | `sp -0x1c` | `ret(0x57b08)` |
| `ldr` at 0xd8cbc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd8cc8 | load | 4 | `abs 0xd8d28` | `-` |
| `ldr` at 0xd8ccc | load | 4 | `abs 0xd8d2c` | `-` |
| `ldr` at 0xd8cd0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd8cdc | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xd8ce8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd8cf4 | store | 4 | `[sp-20] +0xc` | `0x0` |
| `str` at 0xd8cfc | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0xd8d00 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0xd8d08 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd8d08 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd8d08 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd8b44 | `(unknown callee) 0x221f0` | `0x121590` | `?` | `?` | `?` |
| `bl` at 0xd8b80 | `(unknown callee) 0xdf1b0` | `?` | `?` | `0x4` | `?` |
| `bl` at 0xd8b9c | `(unknown callee) 0x1e958` | `0x1215b8` | `?` | `?` | `ret(0xdf1b0)` |
| `bl` at 0xd8bbc | `(unknown callee) 0x1e958` | `0x1215e4` | `?` | `?` | `[sp-36]` |
| `bl` at 0xd8bdc | `(unknown callee) 0x1e958` | `0x1215fc` | `?` | `?` | `[sp-32]` |
| `bl` at 0xd8c00 | `(unknown callee) 0x1e958` | `0x121618` | `?` | `?` | `?` |
| `bl` at 0xd8c64 | `(unknown callee) 0x221f0` | `0x121638` | `[sp-24]` | `[[sp-20]+16]` | `[[sp-20]+20]` |
| `bl` at 0xd8cb0 | `(unknown callee) 0x57b08` | `0x1` | `[sp-24]` | `?` | `?` |
| `bl` at 0xd8cd4 | `(unknown callee) 0x197a8` | `0x29c468` | `0x121668` | `ret(0x57b08)` | `ret(0x57b08)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd8a90 | `? != 0x0` | `? == 0x0` |
| `bls` at 0xd8b08 | `[0x29737c] <= 0x1 (unsigned, 32-bit)` | `[0x29737c] > 0x1 (unsigned, 32-bit)` |
| `bne` at 0xd8b94 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xd8bb4 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `bne` at 0xd8bd4 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xd8bf8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd8c50 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `beq` at 0xd8cc4 | `ret(0x57b08) == 0x0` | `ret(0x57b08) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd8d08 | `r0` | `?` |

## `JChipSetup`

- `JChipSetup`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdcd10 to 0xdcd3c.
  Established. The absolute word at `0x12da70` receives `0xde384` (`str` at 0xdcd24).
  The integer return record is `0x0` (`ldm` at 0xdcd30).
  Registered callback ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xdcd1c | load | 4 | `abs 0xdcd34` | `-` |
| `ldr` at 0xdcd20 | load | 4 | `abs 0xdcd38` | `-` |
| `str` at 0xdcd24 | store | 4 | `abs 0x12da70` | `0xde384` |
| `ldm` at 0xdcd30 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdcd30 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdcd30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdcd30 | `r0` | `0x0` |

## `JChipTargetSetup`

- `JChipTargetSetup`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdcd3c to 0xdcd98.
  Established. Nonzero `ret(0xdefac)` takes `0xdcd78`, while zero falls through (`bne` at 0xdcd68).
  An included word assignment writes `0xfffff7c6` to `sp -0x1c` (`str` at 0xdcd70).
  The unnamed `0xdcd98` call carries `ret(0xdefac)`, unknown, unknown, `ret(0xdefac)` (`bl` at 0xdcd7c).
  Target setup failure meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdcd4c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xdcd50 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdcd5c | store | 4 | `sp -0x18` | `ret(0xdefac)` |
| `ldr` at 0xdcd60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdcd6c | load | 4 | `abs 0xdcd94` | `-` |
| `str` at 0xdcd70 | store | 4 | `sp -0x1c` | `0xfffff7c6` |
| `ldr` at 0xdcd78 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdcd84 | store | 4 | `sp -0x1c` | `ret(0xdcd98)` |
| `ldr` at 0xdcd88 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xdcd90 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdcd90 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdcd90 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdcd54 | `(unknown callee) 0xdefac` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xdcd7c | `(unknown callee) 0xdcd98` | `ret(0xdefac)` | `?` | `?` | `ret(0xdefac)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdcd68 | `ret(0xdefac) != 0x0` | `ret(0xdefac) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdcd90 | `r0` | `?` |

## `JChipPortInit`

- `JChipPortInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdceac to 0xdcfd0.
  Established. Included absolute word assignments save `ret(0x196d0)` at `0x2974ac` and `[sp-28]` at `0x2974b0` (`str` at 0xdcf08, `str` at 0xdcf2c).
  The unnamed `0x196d0` call carries entry `r1`, `0x10`, entry `r2`, unknown (`bl` at 0xdcf00).
  Nonzero `[0x2974ac]` takes `0xdcf24`, while zero falls through (`bne` at 0xdcf14).
  Port record layout remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdcebc | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xdcec0 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xdcec4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdced0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdcedc | load | 4 | `abs 0xdcfc0` | `-` |
| `str` at 0xdceec | store | 4 | `sp -0x28` | `ret(0xdceac)` |
| `ldr` at 0xdcef4 | load | 4 | `abs 0xdcfc4` | `-` |
| `ldr` at 0xdcef8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdcf08 | store | 4 | `abs 0x2974ac` | `ret(0x196d0)` |
| `ldr` at 0xdcf0c | load | 4 | `abs 0x2974ac` | `-` |
| `ldr` at 0xdcf18 | load | 4 | `abs 0xdcfc8` | `-` |
| `str` at 0xdcf1c | store | 4 | `sp -0x28` | `0xfffff7cc` |
| `ldr` at 0xdcf24 | load | 4 | `abs 0xdcfcc` | `-` |
| `ldr` at 0xdcf28 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdcf2c | store | 4 | `abs 0x2974b0` | `[sp-28]` |
| `str` at 0xdcf34 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xdcf38 | load | 4 | `abs 0xdcfcc` | `-` |
| `ldr` at 0xdcf3c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdcf40 | load | 4 | `abs 0x2974b0` | `-` |
| `ldr` at 0xdcf4c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdcf54 | load | 4 | `abs 0xdcfc4` | `-` |
| `ldr` at 0xdcf58 | load | 4 | `abs 0x2974ac` | `-` |
| `str` at 0xdcf60 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xdcf64 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdcf68 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdcf70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdcf78 | load | 4 | `(unknown)` | `-` |
| `str` at 0xdcf7c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xdcf80 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdcf84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdcf8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdcf94 | load | 4 | `(unknown)` | `-` |
| `str` at 0xdcf98 | store | 4 | `[sp-36] +0x4` | `?` |
| `ldr` at 0xdcf9c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdcfa4 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `str` at 0xdcfb0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xdcfb4 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xdcfbc | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xdcfbc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdcfbc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdcfbc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdcee4 | `(unknown callee) 0xdceac` | `0x2973a0` | `0x1` | `r2` | `r1` |
| `bl` at 0xdcf00 | `(unknown callee) 0x196d0` | `r1` | `0x10` | `r2` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdcecc | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xdced8 | `r1 != 0x0` | `r1 == 0x0` |
| `bne` at 0xdcf14 | `[0x2974ac] != 0x0` | `[0x2974ac] == 0x0` |
| `bge` at 0xdcf48 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdcfbc | `r0` | `?` |

## `JChipGetType`

- `JChipGetType`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdef00 to 0xdefac.
  Established. An included word load fetches absolute `0x12da80` (`ldr` at 0xdef24).
  The word at `sp -0x18` receives `[sp-24]+1` (`str` at 0xdef88).
  A word assignment clears `sp -0x1c` on an included path (`str` at 0xdef94).
  Chip type table selection remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdef10 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdef18 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xdef1c | load | 4 | `abs 0xdefa4` | `-` |
| `ldr` at 0xdef20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdef24 | load | 4 | `abs 0x12da80` | `-` |
| `ldr` at 0xdef30 | load | 4 | `abs 0xdefa8` | `-` |
| `ldr` at 0xdef34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdef4c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdef50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdef5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdef70 | load | 4 | `abs 0xdefa8` | `-` |
| `str` at 0xdef78 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xdef80 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdef88 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0xdef94 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xdef98 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xdefa0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdefa0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdefa0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xdef2c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xdef58 | `? != [sp-20]` | `? == [sp-20]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdefa0 | `r0` | `?` |

## `JChipGetChip`

- `JChipGetChip`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdefac to 0xdf0f0.
  Established. A word load reads absolute `0x2974ac` (`ldr` at 0xdefd4).
  An included byte read has an unknown effective target (`ldrb` at 0xdf088).
  A word assignment clears `sp -0x28` on an included path (`str` at 0xdf0dc).
  Chip target lookup indexing remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdefbc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xdefc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdefd0 | load | 4 | `abs 0xdf0ec` | `-` |
| `ldr` at 0xdefd4 | load | 4 | `abs 0x2974ac` | `-` |
| `str` at 0xdefdc | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xdefe0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdefec | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xdeff0 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xdf004 | store | 1 | `sp -0x21` | `?` |
| `str` at 0xdf00c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xdf010 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdf014 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0xdf018 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdf024 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdf028 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdf044 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xdf04c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf050 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf054 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdf060 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdf064 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdf080 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0xdf088 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xdf08c | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xdf098 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdf09c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdf0b8 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xdf0c0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xdf0c8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdf0d0 | store | 4 | `sp -0x18` | `?` |
| `str` at 0xdf0dc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xdf0e0 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xdf0e8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdf0e8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdf0e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xdf020 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xdf05c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xdf094 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdf0e8 | `r0` | `?` |

## `JChipGetNextChip`

- `JChipGetNextChip`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xde004 to 0xde16c.
  Established. Signed entry `r0 < 0x0` takes `0xde048`, while nonnegative values fall through (`blt` at 0xde02c).
  An included word load reads absolute `0x2974b0` (`ldr` at 0xde054).
  A word assignment sets `sp -0x2c` to `0x1` on an included path (`str` at 0xde12c).
  Chip iteration order remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xde014 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xde018 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xde020 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xde024 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xde030 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xde034 | store | 4 | `sp -0x20` | `r0` |
| `ldr` at 0xde038 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xde040 | store | 4 | `sp -0x24` | `r0+1` |
| `str` at 0xde04c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xde050 | load | 4 | `abs 0xde164` | `-` |
| `ldr` at 0xde054 | load | 4 | `abs 0x2974b0` | `-` |
| `str` at 0xde058 | store | 4 | `sp -0x24` | `[0x2974b0]` |
| `ldr` at 0xde05c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xde060 | store | 4 | `sp -0x1c` | `[sp-32]` |
| `ldr` at 0xde064 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xde068 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xde074 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xde07c | load | 4 | `abs 0xde168` | `-` |
| `ldr` at 0xde080 | load | 4 | `abs 0x2974ac` | `-` |
| `str` at 0xde088 | store | 4 | `sp -0x30` | `?` |
| `str` at 0xde090 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xde094 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xde098 | load | 4 | `[sp-48] +0xc` | `-` |
| `ldr` at 0xde09c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xde0a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xde0b4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xde0c4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xde0c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xde0e4 | load | 4 | `[sp-48] +0x8` | `-` |
| `str` at 0xde0ec | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xde0f4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xde0f8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xde114 | load | 4 | `[sp-48] +0x8` | `-` |
| `ldr` at 0xde11c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xde12c | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xde130 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xde138 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xde140 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xde148 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0xde154 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xde158 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0xde160 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xde160 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xde160 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xde02c | `r0 < 0x0 (signed, 32-bit)` | `r0 >= 0x0 (signed, 32-bit)` |
| `bge` at 0xde070 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `ble` at 0xde0a4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xde0b0 | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0xde0bc | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xde124 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xde160 | `r0` | `?` |

## `JChipName`

- `JChipName`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdde20 to 0xdde74.
  Established. The unnamed `0xdefac` call carries entry `r0`, `r1`, `r2`, `r3` (`bl` at 0xdde38);
  its zero result takes `0xdde60`, while nonzero falls through (`beq` at 0xdde4c).
  A word assignment places `ret(0xdefac)+4` at `sp -0x1c` (`str` at 0xdde58).
  Name record ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdde30 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xdde34 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdde40 | store | 4 | `sp -0x18` | `ret(0xdefac)` |
| `ldr` at 0xdde44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdde50 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdde58 | store | 4 | `sp -0x1c` | `ret(0xdefac)+4` |
| `str` at 0xdde64 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xdde68 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xdde70 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdde70 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdde70 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdde38 | `(unknown callee) 0xdefac` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xdde4c | `ret(0xdefac) == 0x0` | `ret(0xdefac) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdde70 | `r0` | `?` |

## `JChipGetCfgTarget`

- `JChipGetCfgTarget`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdf1b0 to 0xdf374.
  Established. A word load fetches absolute `0x2974ac` (`ldr` at 0xdf1e0).
  The unnamed `0x200a0` call carries `0x9`, `0x12208c`, `0x3d0`, `0x0` (`bl` at 0xdf308).
  An included word assignment stores `0x1223fc` at `sp -0x40` (`str` at 0xdf298).
  Configuration target encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xdf1c8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xdf1d0 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldrb` at 0xdf1d4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xdf1dc | load | 4 | `abs 0xdf364` | `-` |
| `ldr` at 0xdf1e0 | load | 4 | `abs 0x2974ac` | `-` |
| `str` at 0xdf1e8 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xdf1f0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xdf1f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdf1f8 | load | 4 | `[sp-36] +0xc` | `-` |
| `ldr` at 0xdf1fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdf208 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdf20c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdf228 | load | 4 | `[sp-36] +0x8` | `-` |
| `str` at 0xdf230 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xdf234 | load | 4 | `sp -0x28` | `-` |
| `ldrsb` at 0xdf238 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xdf23c | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xdf248 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdf24c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf250 | load | 4 | `abs 0xdf368` | `-` |
| `ldrb` at 0xdf254 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xdf274 | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xdf27c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xdf284 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xdf288 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf294 | load | 4 | `abs 0xdf36c` | `-` |
| `str` at 0xdf298 | store | 4 | `sp -0x40` | `0x1223fc` |
| `ldr` at 0xdf29c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdf2a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf2a4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xdf2a8 | store | 4 | `sp -0x3c` | `?` |
| `ldrb` at 0xdf2ac | load | 1 | `sp -0x11` | `-` |
| `str` at 0xdf2b0 | store | 4 | `sp -0x38` | `(r0 & 0xff)` |
| `ldrb` at 0xdf2b4 | load | 1 | `sp -0x12` | `-` |
| `str` at 0xdf2b8 | store | 4 | `sp -0x34` | `(r1 & 0xff)` |
| `ldr` at 0xdf2bc | load | 4 | `abs 0xdf368` | `-` |
| `ldrb` at 0xdf2c0 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xdf2e0 | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xdf2e8 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xdf2f0 | load | 1 | `(unknown)` | `-` |
| `str` at 0xdf2f4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xdf2fc | load | 4 | `abs 0xdf370` | `-` |
| `ldr` at 0xdf30c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xdf310 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xdf318 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdf31c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf320 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf32c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xdf330 | load | 1 | `(unknown)` | `-` |
| `str` at 0xdf338 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xdf340 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdf348 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0xdf354 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xdf358 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0xdf360 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdf360 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdf360 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdf308 | `(unknown callee) 0x200a0` | `0x9` | `0x12208c` | `0x3d0` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xdf204 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xdf244 | `? != (r1 & 0xff)` | `? == (r1 & 0xff)` |
| `beq` at 0xdf290 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdf360 | `r0` | `[sp-44]` |

## `build_sic_tdi`

- `build_sic_tdi`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd4b48 to 0xd4da0.
  Established. An included word store clears entry `r3 +0x0` (`str` at 0xd4bc8).
  Unsigned `([sp-36] + [sp-24]) <= 0x1f` takes `0xd4d68`, while greater values fall through (`bls` at 0xd4d1c).
  A word assignment increments `sp -0x28` with `[sp-40]+1` (`str` at 0xd4d44).
  SIC input packing rules remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd4b58 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd4b5c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xd4b60 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xd4b64 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xd4b6c | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0xd4b74 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xd4b78 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd4b7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd4b8c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd4b94 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xd4b9c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xd4ba0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xd4ba8 | store | 4 | `sp -0x34` | `?` |
| `str` at 0xd4bb4 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xd4bbc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xd4bc0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd4bc8 | store | 4 | `r3 +0x0` | `0x0` |
| `str` at 0xd4bd0 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xd4bd4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd4bd8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd4be4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd4bec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd4bf4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd4bf8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xd4c00 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd4c04 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd4c08 | load | 4 | `abs 0xd4d94` | `-` |
| `ldr` at 0xd4c14 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd4c1c | load | 4 | `abs 0xd4d94` | `-` |
| `str` at 0xd4c28 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd4c2c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd4c30 | load | 4 | `abs 0xd4d98` | `-` |
| `ldr` at 0xd4c3c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd4c44 | load | 4 | `abs 0xd4d98` | `-` |
| `str` at 0xd4c50 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd4c54 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd4c58 | load | 4 | `abs 0xd4d9c` | `-` |
| `ldr` at 0xd4c64 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd4c6c | load | 4 | `abs 0xd4d9c` | `-` |
| `str` at 0xd4c78 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd4c7c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd4c8c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd4ca0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd4ca4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd4cb4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd4cc8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd4ccc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd4cd4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd4cdc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd4ce4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd4cec | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd4cf0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd4cf8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd4d00 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd4d04 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd4d08 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd4d10 | store | 4 | `sp -0x24` | `([sp-36] + [sp-24])` |
| `ldr` at 0xd4d14 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd4d20 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd4d28 | store | 4 | `sp -0x24` | `([sp-36] + [sp-24])-32` |
| `ldr` at 0xd4d2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd4d30 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd4d38 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd4d3c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd4d44 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xd4d48 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd4d50 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd4d58 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd4d5c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd4d64 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd4d68 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xd4d70 | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `ldr` at 0xd4d78 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd4d80 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xd4d90 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd4d90 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd4d90 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhs` at 0xd4b84 | `C == 1` | `not (C == 1)` |
| `bhs` at 0xd4be0 | `C == 1` | `not (C == 1)` |
| `bls` at 0xd4d1c | `([sp-36] + [sp-24]) <= 0x1f (unsigned, 32-bit)` | `([sp-36] + [sp-24]) > 0x1f (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd4d90 | `r0` | `?` |

## `add_sic_tdi`

- `add_sic_tdi`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd4da0 to 0xd4ff0.
  Established. A word assignment writes `([[sp-32]] + [sp-24])` to `[sp-32] +0x0` (`str` at 0xd4fcc).
  Unsigned `([sp-40] + [sp-24]) <= 0x20` takes `0xd4fb8`, while greater values fall through (`bls` at 0xd4f6c).
  The integer return candidate is `[[sp-32]]` (`ldm` at 0xd4fe0).
  Appended SIC bit interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd4db0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd4db4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xd4db8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xd4dbc | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xd4dc4 | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0xd4dcc | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xd4dd0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd4dd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd4de4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd4dec | load | 4 | `sp -0x38` | `-` |
| `str` at 0xd4df4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xd4df8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xd4e00 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xd4e08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd4e0c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xd4e14 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd4e18 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd4e1c | load | 4 | `abs 0xd4fe4` | `-` |
| `ldr` at 0xd4e28 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd4e30 | load | 4 | `abs 0xd4fe4` | `-` |
| `str` at 0xd4e3c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd4e40 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd4e44 | load | 4 | `abs 0xd4fe8` | `-` |
| `ldr` at 0xd4e50 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd4e58 | load | 4 | `abs 0xd4fe8` | `-` |
| `str` at 0xd4e64 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd4e68 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd4e6c | load | 4 | `abs 0xd4fec` | `-` |
| `ldr` at 0xd4e78 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd4e80 | load | 4 | `abs 0xd4fec` | `-` |
| `str` at 0xd4e8c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd4e90 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd4ea0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd4eb4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd4eb8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd4ec8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd4edc | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd4ee0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd4ee4 | load | 4 | `r3 +0x0` | `-` |
| `str` at 0xd4ee8 | store | 4 | `sp -0x28` | `[r3]` |
| `str` at 0xd4ef0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xd4ef4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd4f00 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd4f08 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xd4f0c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd4f14 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd4f1c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd4f24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd4f2c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd4f34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd4f3c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd4f40 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd4f48 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd4f50 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd4f54 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd4f58 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd4f60 | store | 4 | `sp -0x28` | `([sp-40] + [sp-24])` |
| `ldr` at 0xd4f64 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd4f70 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd4f78 | store | 4 | `sp -0x28` | `([sp-40] + [sp-24])-32` |
| `ldr` at 0xd4f7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd4f80 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd4f88 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd4f8c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd4f94 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xd4f98 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd4fa0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd4fa8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd4fac | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd4fb4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd4fb8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd4fbc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd4fc0 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xd4fc4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd4fcc | store | 4 | `[sp-32] +0x0` | `([[sp-32]] + [sp-24])` |
| `ldr` at 0xd4fd0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd4fd4 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldm` at 0xd4fe0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd4fe0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd4fe0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhs` at 0xd4ddc | `C == 1` | `not (C == 1)` |
| `bls` at 0xd4efc | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bls` at 0xd4f6c | `([sp-40] + [sp-24]) <= 0x20 (unsigned, 32-bit)` | `([sp-40] + [sp-24]) > 0x20 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd4fe0 | `r0` | `[[sp-32]]` |
