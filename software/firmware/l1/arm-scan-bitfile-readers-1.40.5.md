# ARM scan bitfile and header readers in 1.40.5

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
Unknown data values and instruction effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/scan-bitfile-readers-facts.txt) retain all tool sections.

## `bitfile_open`

- `bitfile_open`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfa454 to 0xfa854.
  Established. The word at `ret(0xfad50) +0x1c` receives `ret(0x1b4f0)` (`str` at 0xfa4b0).
  A
  nonzero `ret(0xfad50)` takes `0xfa4a0`, while zero falls through (`bne` at 0xfa484).
  The unnamed
  `0xfa8f4` call carries `[[sp-296]+28]`, `sp-29`, `sp-288`, `[sp-296]` (`bl` at 0xfa544).
  Bitfile
  opening policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfa464 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xfa468 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xfa46c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xfa478 | store | 4 | `sp -0x128` | `ret(0xfad50)` |
| `ldr` at 0xfa47c | load | 4 | `sp -0x128` | `-` |
| `ldr` at 0xfa488 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfa48c | load | 4 | `abs 0xfa83c` | `-` |
| `str` at 0xfa498 | store | 4 | `sp -0x12c` | `0x0` |
| `ldr` at 0xfa4a0 | load | 4 | `sp -0x128` | `-` |
| `ldr` at 0xfa4a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfa4a8 | load | 4 | `abs 0xfa840` | `-` |
| `str` at 0xfa4b0 | store | 4 | `ret(0xfad50) +0x1c` | `ret(0x1b4f0)` |
| `ldr` at 0xfa4b4 | load | 4 | `ret(0xfad50) +0x1c` | `-` |
| `ldr` at 0xfa4c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfa4c4 | load | 4 | `abs 0xfa844` | `-` |
| `str` at 0xfa4dc | store | 4 | `sp -0x12c` | `0x0` |
| `ldr` at 0xfa4e4 | load | 4 | `sp -0x128` | `-` |
| `ldr` at 0xfa4e8 | load | 4 | `[sp-296] +0x1c` | `-` |
| `ldr` at 0xfa4fc | load | 4 | `sp -0x128` | `-` |
| `ldr` at 0xfa500 | load | 4 | `[sp-296] +0x1c` | `-` |
| `ldr` at 0xfa514 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfa518 | load | 4 | `abs 0xfa848` | `-` |
| `str` at 0xfa524 | store | 4 | `sp -0x12c` | `0x0` |
| `ldr` at 0xfa52c | load | 4 | `sp -0x128` | `-` |
| `ldr` at 0xfa538 | load | 4 | `[sp-296] +0x1c` | `-` |
| `ldr` at 0xfa554 | load | 4 | `sp -0x128` | `-` |
| `ldr` at 0xfa558 | load | 4 | `[sp-296] +0x1c` | `-` |
| `ldr` at 0xfa56c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfa570 | load | 4 | `abs 0xfa84c` | `-` |
| `str` at 0xfa57c | store | 4 | `sp -0x12c` | `0x0` |
| `ldr` at 0xfa584 | load | 4 | `sp -0x128` | `-` |
| `str` at 0xfa598 | store | 4 | `[sp-296] +0x4` | `ret(0x1b034)` |
| `ldr` at 0xfa59c | load | 4 | `[sp-296] +0x4` | `-` |
| `ldr` at 0xfa5a8 | load | 4 | `sp -0x128` | `-` |
| `ldr` at 0xfa5ac | load | 4 | `[sp-296] +0x1c` | `-` |
| `ldr` at 0xfa5c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfa5c4 | load | 4 | `abs 0xfa850` | `-` |
| `str` at 0xfa5d0 | store | 4 | `sp -0x12c` | `0x0` |
| `ldr` at 0xfa5d8 | load | 4 | `sp -0x128` | `-` |
| `ldr` at 0xfa5e4 | load | 4 | `[sp-296] +0x1c` | `-` |
| `ldr` at 0xfa600 | load | 4 | `sp -0x128` | `-` |
| `ldr` at 0xfa604 | load | 4 | `[sp-296] +0x1c` | `-` |
| `ldr` at 0xfa618 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfa61c | load | 4 | `abs 0xfa84c` | `-` |
| `str` at 0xfa628 | store | 4 | `sp -0x12c` | `0x0` |
| `ldr` at 0xfa630 | load | 4 | `sp -0x128` | `-` |
| `str` at 0xfa644 | store | 4 | `[sp-296] +0x8` | `ret(0x1b034)` |
| `ldr` at 0xfa648 | load | 4 | `[sp-296] +0x8` | `-` |
| `ldr` at 0xfa654 | load | 4 | `sp -0x128` | `-` |
| `ldr` at 0xfa658 | load | 4 | `[sp-296] +0x1c` | `-` |
| `ldr` at 0xfa66c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfa670 | load | 4 | `abs 0xfa850` | `-` |
| `str` at 0xfa67c | store | 4 | `sp -0x12c` | `0x0` |
| `ldr` at 0xfa684 | load | 4 | `sp -0x128` | `-` |
| `ldr` at 0xfa690 | load | 4 | `[sp-296] +0x1c` | `-` |
| `ldr` at 0xfa6ac | load | 4 | `sp -0x128` | `-` |
| `ldr` at 0xfa6b0 | load | 4 | `[sp-296] +0x1c` | `-` |
| `ldr` at 0xfa6c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfa6c8 | load | 4 | `abs 0xfa84c` | `-` |
| `str` at 0xfa6d4 | store | 4 | `sp -0x12c` | `0x0` |
| `ldr` at 0xfa6dc | load | 4 | `sp -0x128` | `-` |
| `str` at 0xfa6f0 | store | 4 | `[sp-296] +0x10` | `ret(0x1b034)` |
| `ldr` at 0xfa6f4 | load | 4 | `[sp-296] +0x10` | `-` |
| `ldr` at 0xfa700 | load | 4 | `sp -0x128` | `-` |
| `ldr` at 0xfa704 | load | 4 | `[sp-296] +0x1c` | `-` |
| `ldr` at 0xfa718 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfa71c | load | 4 | `abs 0xfa850` | `-` |
| `str` at 0xfa728 | store | 4 | `sp -0x12c` | `0x0` |
| `ldr` at 0xfa730 | load | 4 | `sp -0x128` | `-` |
| `ldr` at 0xfa73c | load | 4 | `[sp-296] +0x1c` | `-` |
| `ldr` at 0xfa758 | load | 4 | `sp -0x128` | `-` |
| `ldr` at 0xfa75c | load | 4 | `[sp-296] +0x1c` | `-` |
| `ldr` at 0xfa770 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfa774 | load | 4 | `abs 0xfa84c` | `-` |
| `str` at 0xfa780 | store | 4 | `sp -0x12c` | `0x0` |
| `ldr` at 0xfa788 | load | 4 | `sp -0x128` | `-` |
| `str` at 0xfa79c | store | 4 | `[sp-296] +0x14` | `ret(0x1b034)` |
| `ldr` at 0xfa7a0 | load | 4 | `[sp-296] +0x14` | `-` |
| `ldr` at 0xfa7ac | load | 4 | `sp -0x128` | `-` |
| `ldr` at 0xfa7b0 | load | 4 | `[sp-296] +0x1c` | `-` |
| `ldr` at 0xfa7c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfa7c8 | load | 4 | `abs 0xfa850` | `-` |
| `str` at 0xfa7d4 | store | 4 | `sp -0x12c` | `0x0` |
| `ldr` at 0xfa7dc | load | 4 | `sp -0x128` | `-` |
| `ldr` at 0xfa7e0 | load | 4 | `sp -0x128` | `-` |
| `ldr` at 0xfa7e4 | load | 4 | `[sp-296] +0x1c` | `-` |
| `str` at 0xfa7ec | store | 4 | `[sp-296] +0x18` | `ret(0xfaa3c)` |
| `ldr` at 0xfa7f0 | load | 4 | `[sp-296] +0x18` | `-` |
| `ldr` at 0xfa7fc | load | 4 | `sp -0x128` | `-` |
| `ldr` at 0xfa800 | load | 4 | `[sp-296] +0x1c` | `-` |
| `ldr` at 0xfa814 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfa818 | load | 4 | `abs 0xfa84c` | `-` |
| `str` at 0xfa824 | store | 4 | `sp -0x12c` | `0x0` |
| `ldr` at 0xfa82c | load | 4 | `sp -0x128` | `-` |
| `str` at 0xfa830 | store | 4 | `sp -0x12c` | `[sp-296]` |
| `ldr` at 0xfa834 | load | 4 | `sp -0x12c` | `-` |
| `ldmdb` at 0xfa838 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xfa838 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xfa838 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xfa838 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xfa470 | `(unknown callee) 0xfad50` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xfa490 | `(unknown callee) 0x1b52c` | `[sp-28]` | `0x13acc0` | `?` | `ret(0xfad50)` |
| `bl` at 0xfa4ac | `(unknown callee) 0x1b4f0` | `[sp-24]` | `0x13acf0` | `?` | `ret(0xfad50)` |
| `bl` at 0xfa4c8 | `(unknown callee) 0x1b52c` | `[sp-28]` | `0x13acf4` | `?` | `[ret(0xfad50)+28]` |
| `bl` at 0xfa4d4 | `(unknown callee) 0xfadec` | `sp-296` | `?` | `?` | `sp-296` |
| `bl` at 0xfa4ec | `(unknown callee) 0xfa854` | `[[sp-296]+28]` | `?` | `?` | `[sp-296]` |
| `bl` at 0xfa504 | `(unknown callee) 0x1b3a0` | `[[sp-296]+28]` | `?` | `?` | `[sp-296]` |
| `bl` at 0xfa510 | `(unknown callee) 0xfadec` | `sp-296` | `?` | `?` | `sp-296` |
| `bl` at 0xfa51c | `(unknown callee) 0x1b52c` | `[sp-28]` | `0x13ad1c` | `?` | `?` |
| `bl` at 0xfa544 | `(unknown callee) 0xfa8f4` | `[[sp-296]+28]` | `sp-29` | `sp-288` | `[sp-296]` |
| `bl` at 0xfa55c | `(unknown callee) 0x1b3a0` | `[[sp-296]+28]` | `?` | `?` | `[sp-296]` |
| `bl` at 0xfa568 | `(unknown callee) 0xfadec` | `sp-296` | `?` | `?` | `sp-296` |
| `bl` at 0xfa574 | `(unknown callee) 0x1b52c` | `[sp-28]` | `0x13ad48` | `?` | `?` |
| `bl` at 0xfa590 | `(unknown callee) 0x1b034` | `sp-288` | `?` | `?` | `sp-288` |
| `bl` at 0xfa5b0 | `(unknown callee) 0x1b3a0` | `[[sp-296]+28]` | `?` | `?` | `[sp-296]` |
| `bl` at 0xfa5bc | `(unknown callee) 0xfadec` | `sp-296` | `?` | `?` | `sp-296` |
| `bl` at 0xfa5c8 | `(unknown callee) 0x1b52c` | `[sp-28]` | `0x13ad78` | `?` | `?` |
| `bl` at 0xfa5f0 | `(unknown callee) 0xfa8f4` | `[[sp-296]+28]` | `sp-29` | `sp-288` | `[sp-296]` |
| `bl` at 0xfa608 | `(unknown callee) 0x1b3a0` | `[[sp-296]+28]` | `?` | `?` | `[sp-296]` |
| `bl` at 0xfa614 | `(unknown callee) 0xfadec` | `sp-296` | `?` | `?` | `sp-296` |
| `bl` at 0xfa620 | `(unknown callee) 0x1b52c` | `[sp-28]` | `0x13ad48` | `?` | `?` |
| `bl` at 0xfa63c | `(unknown callee) 0x1b034` | `sp-288` | `?` | `?` | `sp-288` |
| `bl` at 0xfa65c | `(unknown callee) 0x1b3a0` | `[[sp-296]+28]` | `?` | `?` | `[sp-296]` |
| `bl` at 0xfa668 | `(unknown callee) 0xfadec` | `sp-296` | `?` | `?` | `sp-296` |
| `bl` at 0xfa674 | `(unknown callee) 0x1b52c` | `[sp-28]` | `0x13ad78` | `?` | `?` |
| `bl` at 0xfa69c | `(unknown callee) 0xfa8f4` | `[[sp-296]+28]` | `sp-29` | `sp-288` | `[sp-296]` |
| `bl` at 0xfa6b4 | `(unknown callee) 0x1b3a0` | `[[sp-296]+28]` | `?` | `?` | `[sp-296]` |
| `bl` at 0xfa6c0 | `(unknown callee) 0xfadec` | `sp-296` | `?` | `?` | `sp-296` |
| `bl` at 0xfa6cc | `(unknown callee) 0x1b52c` | `[sp-28]` | `0x13ad48` | `?` | `?` |
| `bl` at 0xfa6e8 | `(unknown callee) 0x1b034` | `sp-288` | `?` | `?` | `sp-288` |
| `bl` at 0xfa708 | `(unknown callee) 0x1b3a0` | `[[sp-296]+28]` | `?` | `?` | `[sp-296]` |
| `bl` at 0xfa714 | `(unknown callee) 0xfadec` | `sp-296` | `?` | `?` | `sp-296` |
| `bl` at 0xfa720 | `(unknown callee) 0x1b52c` | `[sp-28]` | `0x13ad78` | `?` | `?` |
| `bl` at 0xfa748 | `(unknown callee) 0xfa8f4` | `[[sp-296]+28]` | `sp-29` | `sp-288` | `[sp-296]` |
| `bl` at 0xfa760 | `(unknown callee) 0x1b3a0` | `[[sp-296]+28]` | `?` | `?` | `[sp-296]` |
| `bl` at 0xfa76c | `(unknown callee) 0xfadec` | `sp-296` | `?` | `?` | `sp-296` |
| `bl` at 0xfa778 | `(unknown callee) 0x1b52c` | `[sp-28]` | `0x13ad48` | `?` | `?` |
| `bl` at 0xfa794 | `(unknown callee) 0x1b034` | `sp-288` | `?` | `?` | `sp-288` |
| `bl` at 0xfa7b4 | `(unknown callee) 0x1b3a0` | `[[sp-296]+28]` | `?` | `?` | `[sp-296]` |
| `bl` at 0xfa7c0 | `(unknown callee) 0xfadec` | `sp-296` | `?` | `?` | `sp-296` |
| `bl` at 0xfa7cc | `(unknown callee) 0x1b52c` | `[sp-28]` | `0x13ad78` | `?` | `?` |
| `bl` at 0xfa7e8 | `(unknown callee) 0xfaa3c` | `[[sp-296]+28]` | `?` | `?` | `[sp-296]` |
| `bl` at 0xfa804 | `(unknown callee) 0x1b3a0` | `[[sp-296]+28]` | `?` | `?` | `[sp-296]` |
| `bl` at 0xfa810 | `(unknown callee) 0xfadec` | `sp-296` | `?` | `?` | `sp-296` |
| `bl` at 0xfa81c | `(unknown callee) 0x1b52c` | `[sp-28]` | `0x13ad48` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xfa484 | `ret(0xfad50) != 0x0` | `ret(0xfad50) == 0x0` |
| `bne` at 0xfa4bc | `[ret(0xfad50)+28] != 0x0` | `[ret(0xfad50)+28] == 0x0` |
| `beq` at 0xfa4f8 | `ret(0xfa854) == 0x0` | `ret(0xfa854) != 0x0` |
| `beq` at 0xfa550 | `ret(0xfa8f4) == 0x0` | `ret(0xfa8f4) != 0x0` |
| `bne` at 0xfa5a4 | `[[sp-296]+4] != 0x0` | `[[sp-296]+4] == 0x0` |
| `beq` at 0xfa5fc | `ret(0xfa8f4) == 0x0` | `ret(0xfa8f4) != 0x0` |
| `bne` at 0xfa650 | `[[sp-296]+8] != 0x0` | `[[sp-296]+8] == 0x0` |
| `beq` at 0xfa6a8 | `ret(0xfa8f4) == 0x0` | `ret(0xfa8f4) != 0x0` |
| `bne` at 0xfa6fc | `[[sp-296]+16] != 0x0` | `[[sp-296]+16] == 0x0` |
| `beq` at 0xfa754 | `ret(0xfa8f4) == 0x0` | `ret(0xfa8f4) != 0x0` |
| `bne` at 0xfa7a8 | `[[sp-296]+20] != 0x0` | `[[sp-296]+20] == 0x0` |
| `bgt` at 0xfa7f8 | `[[sp-296]+24] > 0x0 (signed, 32-bit)` | `[[sp-296]+24] <= 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xfa838 | `r0` | `?` |

## `bitfile_read_hdr_prefix`

- `bitfile_read_hdr_prefix`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfa854 to 0xfa8f4.
  Established. The unnamed `0x1b394` call carries `sp-60`, `0x13ada5`, `0xd`, `0x13ada5` (`bl` at 0xfa880).
  Equality of `ret(0x1b1f0)` and `[sp-24]` takes `0xfa8c8`, while inequality falls through
  (`beq` at 0xfa8b8).
  The return reports `[sp-64]` (`ldmdb` at 0xfa8ec).
  Header prefix
  interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfa864 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xfa868 | load | 4 | `abs 0xfa8f0` | `-` |
| `str` at 0xfa888 | store | 4 | `sp -0x18` | `0xd` |
| `ldr` at 0xfa898 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfa89c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xfa8a8 | store | 4 | `sp -0x1c` | `ret(0x1b1f0)` |
| `ldr` at 0xfa8ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfa8b0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xfa8c0 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xfa8d8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xfa8e4 | store | 4 | `sp -0x40` | `ret(0x1b304)` |
| `ldr` at 0xfa8e8 | load | 4 | `sp -0x40` | `-` |
| `ldmdb` at 0xfa8ec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xfa8ec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xfa8ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xfa880 | `(unknown callee) 0x1b394` | `sp-60` | `0x13ada5` | `0xd` | `0x13ada5` |
| `bl` at 0xfa8a0 | `(unknown callee) 0x1b1f0` | `sp-44` | `0x1` | `0xd` | `[sp-20]` |
| `bl` at 0xfa8dc | `(unknown callee) 0x1b304` | `sp-44` | `sp-60` | `[sp-24]` | `sp-44` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xfa8b8 | `ret(0x1b1f0) == [sp-24]` | `ret(0x1b1f0) != [sp-24]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xfa8ec | `r0` | `[sp-64]` |

## `bitfile_read_hdr_section`

- `bitfile_read_hdr_section`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfa8f4 to 0xfaa3c.
  Established. Included byte assignments write zero at an unknown target and at `([sp-28] + [sp-32]) +0x0` (`strb` at 0xfa96c, `strb` at 0xfaa28).
  The byte through `[sp-24] +0x0` receives `([sp-40]:1 & 0xff)` (`strb` at 0xfa984).
  The unnamed `0x1b1f0` call carries `[sp-28]`, `0x1`, `[sp-32]`,
  `[sp-20]` (`bl` at 0xfa9f0).
  Section format remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfa904 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xfa908 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xfa90c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xfa914 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xfa924 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xfa928 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xfa934 | store | 4 | `sp -0x24` | `ret(0x1b1f0)` |
| `ldr` at 0xfa938 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xfa93c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xfa94c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xfa958 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xfa96c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xfa970 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfa97c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xfa980 | load | 1 | `sp -0x28` | `-` |
| `strb` at 0xfa984 | store | 1 | `[sp-24] +0x0` | `([sp-40]:1 & 0xff)` |
| `str` at 0xfa98c | store | 4 | `sp -0x20` | `0x2` |
| `ldr` at 0xfa99c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xfa9a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xfa9ac | store | 4 | `sp -0x24` | `ret(0x1b1f0)` |
| `ldr` at 0xfa9b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xfa9b4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xfa9c4 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0xfa9cc | load | 1 | `sp -0x28` | `-` |
| `ldrb` at 0xfa9d4 | load | 1 | `sp -0x27` | `-` |
| `str` at 0xfa9dc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xfa9e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfa9e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xfa9ec | load | 4 | `sp -0x14` | `-` |
| `str` at 0xfa9f8 | store | 4 | `sp -0x24` | `ret(0x1b1f0)` |
| `ldr` at 0xfa9fc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xfaa00 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xfaa10 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xfaa18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfaa1c | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xfaa28 | store | 1 | `([sp-28] + [sp-32]) +0x0` | `0x0` |
| `str` at 0xfaa30 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xfaa34 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xfaa38 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xfaa38 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xfaa38 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xfa92c | `(unknown callee) 0x1b1f0` | `sp-40` | `0x1` | `0x1` | `r0` |
| `bl` at 0xfa9a4 | `(unknown callee) 0x1b1f0` | `sp-40` | `0x1` | `0x2` | `[sp-20]` |
| `bl` at 0xfa9f0 | `(unknown callee) 0x1b1f0` | `[sp-28]` | `0x1` | `[sp-32]` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xfa944 | `ret(0x1b1f0) == [sp-32]` | `ret(0x1b1f0) != [sp-32]` |
| `beq` at 0xfa978 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0xfa9bc | `ret(0x1b1f0) == [sp-32]` | `ret(0x1b1f0) != [sp-32]` |
| `beq` at 0xfaa08 | `ret(0x1b1f0) == [sp-32]` | `ret(0x1b1f0) != [sp-32]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xfaa38 | `r0` | `[sp-44]` |

## `bitfile_read_hdr_length`

- `bitfile_read_hdr_length`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfaa3c to 0xfab74.
  Established. The byte at `sp -0x20` is read (`ldrb` at 0xfaa94);
  its equality to `0x65` takes
  `0xfaaac`, while inequality falls through (`beq` at 0xfaa9c).
  The unnamed `0x1b1f0` call carries
  `sp-32`, `0x1`, `0x4`, `[sp-20]` (`bl` at 0xfaac8).
  Length reconstruction semantics remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfaa4c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xfaa54 | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0xfaa64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfaa68 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xfaa74 | store | 4 | `sp -0x1c` | `ret(0x1b1f0)` |
| `ldr` at 0xfaa78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfaa7c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xfaa8c | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0xfaa94 | load | 1 | `sp -0x20` | `-` |
| `str` at 0xfaaa4 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xfaab0 | store | 4 | `sp -0x18` | `0x4` |
| `ldr` at 0xfaac0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfaac4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xfaad0 | store | 4 | `sp -0x1c` | `ret(0x1b1f0)` |
| `ldr` at 0xfaad4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfaad8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xfaae8 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xfaaf4 | store | 4 | `sp -0x24` | `0x0` |
| `ldrb` at 0xfaaf8 | load | 1 | `sp -0x20` | `-` |
| `str` at 0xfab00 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xfab04 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xfab08 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xfab10 | store | 4 | `sp -0x24` | `?` |
| `ldrb` at 0xfab14 | load | 1 | `sp -0x1f` | `-` |
| `str` at 0xfab1c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xfab20 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xfab24 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xfab2c | store | 4 | `sp -0x24` | `?` |
| `ldrb` at 0xfab30 | load | 1 | `sp -0x1e` | `-` |
| `str` at 0xfab38 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xfab3c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xfab40 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xfab48 | store | 4 | `sp -0x24` | `?` |
| `ldrb` at 0xfab4c | load | 1 | `sp -0x1d` | `-` |
| `str` at 0xfab50 | store | 4 | `sp -0x28` | `[sp-29]:1` |
| `ldr` at 0xfab54 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xfab58 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xfab60 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xfab64 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xfab68 | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0xfab6c | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xfab70 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xfab70 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xfab70 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xfaa6c | `(unknown callee) 0x1b1f0` | `sp-32` | `0x1` | `0x1` | `r0` |
| `bl` at 0xfaac8 | `(unknown callee) 0x1b1f0` | `sp-32` | `0x1` | `0x4` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xfaa84 | `ret(0x1b1f0) == [sp-24]` | `ret(0x1b1f0) != [sp-24]` |
| `beq` at 0xfaa9c | `[sp-32]:1 == 0x65` | `[sp-32]:1 != 0x65` |
| `beq` at 0xfaae0 | `ret(0x1b1f0) == [sp-24]` | `ret(0x1b1f0) != [sp-24]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xfab70 | `r0` | `[sp-44]` |

## `bitfile_read_data`

- `bitfile_read_data`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfab74 to 0xfad50.
  Established. Included word reads use entry `r0 +0x18` and `r0 +0x20` (`ldr` at 0xfab98, `ldr` at 0xfab9c).
  The unnamed `0x1b1f0` call carries `[sp-28]`, `0x1`, `[sp-32]`, `[[sp-20]+28]` (`bl` at 0xfabd4).
  The byte at `([sp-32] + [sp-52]) +0x0` receives zero (`strb` at 0xfac54).
  Data
  transformation meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfab84 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xfab88 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xfab8c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xfab90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfab94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfab98 | load | 4 | `r0 +0x18` | `-` |
| `ldr` at 0xfab9c | load | 4 | `r0 +0x20` | `-` |
| `str` at 0xfaba4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xfaba8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xfabac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfabb8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xfabbc | store | 4 | `sp -0x20` | `[sp-24]` |
| `ldr` at 0xfabc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfabc4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfabcc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xfabd0 | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0xfabdc | store | 4 | `sp -0x24` | `ret(0x1b1f0)` |
| `ldr` at 0xfabe0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xfabe4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xfabf4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xfabfc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xfac04 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xfac08 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xfac18 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xfac1c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xfac20 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xfac24 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xfac2c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xfac40 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xfac48 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xfac54 | store | 1 | `([sp-32] + [sp-52]) +0x0` | `0x0` |
| `str` at 0xfac5c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xfac60 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xfac68 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xfac70 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xfac78 | store | 4 | `sp -0x20` | `?` |
| `str` at 0xfac80 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xfac84 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xfac88 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xfac8c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xfac90 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xfaca0 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xfaca4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xfacb0 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xfacb8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xfacc8 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xfacd0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xface0 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xface8 | load | 1 | `(unknown)` | `-` |
| `str` at 0xfacf0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xfacf4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xfacfc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfad04 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xfad08 | store | 4 | `(unknown)` | `[sp-40]` |
| `ldr` at 0xfad0c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xfad14 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xfad18 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xfad20 | store | 4 | `sp -0x34` | `[sp-52]+4` |
| `ldr` at 0xfad28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfad2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfad30 | load | 4 | `[sp-20] +0x20` | `-` |
| `ldr` at 0xfad34 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xfad3c | store | 4 | `[sp-20] +0x20` | `?` |
| `ldr` at 0xfad40 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xfad44 | store | 4 | `sp -0x38` | `[sp-36]` |
| `ldr` at 0xfad48 | load | 4 | `sp -0x38` | `-` |
| `ldmdb` at 0xfad4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xfad4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xfad4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xfabd4 | `(unknown callee) 0x1b1f0` | `[sp-28]` | `0x1` | `[sp-32]` | `[[sp-20]+28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xfabb4 | `[sp-32] <= [sp-24] (unsigned, 32-bit)` | `[sp-32] > [sp-24] (unsigned, 32-bit)` |
| `beq` at 0xfabec | `ret(0x1b1f0) == [sp-32]` | `ret(0x1b1f0) != [sp-32]` |
| `beq` at 0xfac10 | `[sp-48] == 0x0` | `[sp-48] != 0x0` |
| `blo` at 0xfac34 | `[sp-44] < ? (unsigned, 32-bit)` | `[sp-44] >= ? (unsigned, 32-bit)` |
| `blo` at 0xfac98 | `C == 0` | `not (C == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xfad4c | `r0` | `[sp-56]` |

## `bitfile_create`

- `bitfile_create`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfad50 to 0xfadec.
  Established. The unnamed `0x1b1b4` call carries `0x24`, entry `r1`, `r2`, `r3` (`bl` at 0xfad68);
  its nonzero result takes `0xfad8c`, while zero falls through (`bne` at 0xfad7c).
  The word through
  `[sp-28] +0x0` receives `ret(0x1b034)` (`str` at 0xfadb8).
  Created object ownership remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfad60 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xfad70 | store | 4 | `sp -0x1c` | `ret(0x1b1b4)` |
| `ldr` at 0xfad74 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xfad84 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xfad8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfad9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfada8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfadac | load | 4 | `sp -0x18` | `-` |
| `str` at 0xfadb8 | store | 4 | `[sp-28] +0x0` | `ret(0x1b034)` |
| `ldr` at 0xfadbc | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xfadc8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xfadd4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xfaddc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xfade0 | store | 4 | `sp -0x20` | `[sp-28]` |
| `ldr` at 0xfade4 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xfade8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xfade8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xfade8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xfade8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xfad68 | `(unknown callee) 0x1b1b4` | `0x24` | `r1` | `r2` | `r3` |
| `bl` at 0xfad98 | `(unknown callee) 0x1b4cc` | `ret(0x1b1b4)` | `0x0` | `0x24` | `ret(0x1b1b4)` |
| `bl` at 0xfadb0 | `(unknown callee) 0x1b034` | `[sp-24]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xfadcc | `(unknown callee) 0x1b490` | `[sp-28]` | `?` | `?` | `[[sp-28]]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xfad7c | `ret(0x1b1b4) != 0x0` | `ret(0x1b1b4) == 0x0` |
| `beq` at 0xfada4 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `bne` at 0xfadc4 | `[[sp-28]] != 0x0` | `[[sp-28]] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xfade8 | `r0` | `?` |

## `bitfile_destroy`

- `bitfile_destroy`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfadec to 0xfaed0.
  Established. The word through entry `r0 +0x0` is read, then a word through `[r0] +0x0` is read
  (`ldr` at 0xfae04, `ldr` at 0xfae08).
  The unnamed `0x1b490` call carries `[[r0]]`, entry `r1`,
  entry `r2`, `[r0]` (`bl` at 0xfae20).
  A word through `[sp-20] +0x0` receives zero (`str` at 0xfaec8).
  Destruction effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfadfc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xfae00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfae04 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0xfae08 | load | 4 | `[r0] +0x0` | `-` |
| `ldr` at 0xfae14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfae18 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0xfae1c | load | 4 | `[r0] +0x0` | `-` |
| `ldr` at 0xfae24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfae28 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xfae2c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xfae38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfae3c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xfae40 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xfae48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfae4c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xfae50 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xfae5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfae60 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xfae64 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xfae6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfae70 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xfae74 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xfae80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfae84 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xfae88 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xfae90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfae94 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xfae98 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xfaea4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfaea8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xfaeac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xfaeb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfaeb8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xfaec0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xfaec8 | store | 4 | `[sp-20] +0x0` | `0x0` |
| `ldmdb` at 0xfaecc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xfaecc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xfaecc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xfae20 | `(unknown callee) 0x1b490` | `[[r0]]` | `r1` | `r2` | `[r0]` |
| `bl` at 0xfae44 | `(unknown callee) 0x1b490` | `?` | `?` | `?` | `?` |
| `bl` at 0xfae68 | `(unknown callee) 0x1b490` | `?` | `?` | `?` | `?` |
| `bl` at 0xfae8c | `(unknown callee) 0x1b490` | `?` | `?` | `?` | `?` |
| `bl` at 0xfaeb0 | `(unknown callee) 0x1b490` | `?` | `?` | `?` | `?` |
| `bl` at 0xfaebc | `(unknown callee) 0x1b490` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xfae10 | `[[r0]] == 0x0` | `[[r0]] != 0x0` |
| `beq` at 0xfae34 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xfae58 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xfae7c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xfaea0 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xfaecc | `r0` | `ret(0x1b490)` |

## `bitfile_display`

- `bitfile_display`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfaed0 to 0xfb010.
  Established. Entry `r0 != 0x0` takes `0xfaf08`, while zero falls through (`bne` at 0xfaef8).
  The
  absolute word at `0x145ae4` is read (`ldr` at 0xfaf00).
  The unnamed `0x240b8` call carries
  `[sp-20]`, `0x13ae44`, `[[sp-24]+24]`, `[[sp-24]+24]` (`bl` at 0xfafb4).
  Display field meanings
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfaee0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xfaee4 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xfaee8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xfaeec | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0xfaef0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfaefc | load | 4 | `abs 0xfafe4` | `-` |
| `ldr` at 0xfaf00 | load | 4 | `abs 0x145ae4` | `-` |
| `str` at 0xfaf04 | store | 4 | `sp -0x1c` | `[0x145ae4]` |
| `ldr` at 0xfaf08 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xfaf0c | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xfaf10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfaf14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfaf18 | load | 4 | `abs 0xfafe8` | `-` |
| `ldr` at 0xfaf1c | load | 4 | `r1 +0x0` | `-` |
| `ldr` at 0xfaf24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfaf28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfaf2c | load | 4 | `abs 0xfafec` | `-` |
| `ldr` at 0xfaf30 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xfaf38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfaf3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfaf40 | load | 4 | `abs 0xfaff0` | `-` |
| `ldr` at 0xfaf44 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xfaf4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfaf50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfaf54 | load | 4 | `abs 0xfaff4` | `-` |
| `ldr` at 0xfaf58 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xfaf60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfaf64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfaf68 | load | 4 | `abs 0xfaff8` | `-` |
| `ldr` at 0xfaf6c | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xfaf74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfaf78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfaf7c | load | 4 | `abs 0xfaffc` | `-` |
| `ldr` at 0xfaf80 | load | 4 | `[sp-24] +0x14` | `-` |
| `ldr` at 0xfaf88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfaf8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfaf90 | load | 4 | `abs 0xfb000` | `-` |
| `ldr` at 0xfaf94 | load | 4 | `[sp-24] +0x1c` | `-` |
| `ldr` at 0xfaf9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfafa0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfafa4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfafa8 | load | 4 | `abs 0xfb004` | `-` |
| `ldr` at 0xfafac | load | 4 | `[sp-24] +0x18` | `-` |
| `ldr` at 0xfafb0 | load | 4 | `[sp-24] +0x18` | `-` |
| `ldr` at 0xfafb8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfafbc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfafc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfafc4 | load | 4 | `abs 0xfb008` | `-` |
| `ldr` at 0xfafc8 | load | 4 | `[sp-24] +0x20` | `-` |
| `ldr` at 0xfafcc | load | 4 | `[sp-24] +0x20` | `-` |
| `ldr` at 0xfafd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfafd8 | load | 4 | `abs 0xfb00c` | `-` |
| `ldmdb` at 0xfafe0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xfafe0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xfafe0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xfaf20 | `(unknown callee) 0x240b8` | `?` | `0x13adb4` | `[r1]` | `r1` |
| `bl` at 0xfaf34 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x13adc8` | `[[sp-24]+4]` | `[sp-24]` |
| `bl` at 0xfaf48 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x13addc` | `[[sp-24]+8]` | `[sp-24]` |
| `bl` at 0xfaf5c | `(unknown callee) 0x240b8` | `[sp-20]` | `0x13adf0` | `[[sp-24]+12]` | `[sp-24]` |
| `bl` at 0xfaf70 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x13ae04` | `[[sp-24]+16]` | `[sp-24]` |
| `bl` at 0xfaf84 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x13ae18` | `[[sp-24]+20]` | `[sp-24]` |
| `bl` at 0xfaf98 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x13ae2c` | `[[sp-24]+28]` | `[sp-24]` |
| `bl` at 0xfafb4 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x13ae44` | `[[sp-24]+24]` | `[[sp-24]+24]` |
| `bl` at 0xfafd0 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x13ae60` | `[[sp-24]+32]` | `[[sp-24]+32]` |
| `bl` at 0xfafdc | `(unknown callee) 0x240b8` | `[sp-20]` | `0x13ae7c` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xfaef8 | `r0 != 0x0` | `r0 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xfafe0 | `r0` | `ret(0x240b8)` |

## `initbh`

- `initbh`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe2060 to 0xe20b4.
  Established. The word through entry `r0 +0x0` receives zero (`str` at 0xe207c).
  Additional zero
  word assignments use `[sp-20] +0x4`, `+0x8`, `+0xc` and `+0x10` (`str` at 0xe2088, `str` at 0xe2094, `str` at 0xe20a0, `str` at 0xe20ac).
  Entry `r0` is the reported return value (`ldmdb` at 0xe20b0).
  Header record interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe2070 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xe2074 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe207c | store | 4 | `r0 +0x0` | `0x0` |
| `ldr` at 0xe2080 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe2088 | store | 4 | `[sp-20] +0x4` | `0x0` |
| `ldr` at 0xe208c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe2094 | store | 4 | `[sp-20] +0x8` | `0x0` |
| `ldr` at 0xe2098 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe20a0 | store | 4 | `[sp-20] +0xc` | `0x0` |
| `ldr` at 0xe20a4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe20ac | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldmdb` at 0xe20b0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe20b0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe20b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe20b0 | `r0` | `r0` |

## `freebh`

- `freebh`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe20b4 to 0xe2104.
  Established. Included word reads use entry `r0 +0x0` and `[sp-20] +0x4` (`ldr` at 0xe20cc, `ldr`
  at 0xe20d8).
  The unnamed `0xe2060` call carries `[sp-20]` and three unknown candidates (`bl` at 0xe20fc).
  The return reports `ret(0xe2060)` (`ldmdb` at 0xe2100).
  Resource release semantics
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe20c4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xe20c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe20cc | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0xe20d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe20d8 | load | 4 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xe20e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe20e4 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xe20ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe20f0 | load | 4 | `[sp-20] +0xc` | `-` |
| `ldr` at 0xe20f8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe2100 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe2100 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe2100 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe20d0 | `(unknown callee) 0x1b490` | `[r0]` | `r1` | `r2` | `r0` |
| `bl` at 0xe20dc | `(unknown callee) 0x1b490` | `[[sp-20]+4]` | `?` | `?` | `[sp-20]` |
| `bl` at 0xe20e8 | `(unknown callee) 0x1b490` | `[[sp-20]+8]` | `?` | `?` | `[sp-20]` |
| `bl` at 0xe20f4 | `(unknown callee) 0x1b490` | `[[sp-20]+12]` | `?` | `?` | `[sp-20]` |
| `bl` at 0xe20fc | `(unknown callee) 0xe2060` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe2100 | `r0` | `ret(0xe2060)` |

## `readhead`

- `readhead`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe2104 to 0xe22f8.
  Established. The unnamed `0xe22f8` call carries entry `r1`, entry `r1`, entry `r2`, entry `r3`
  (`bl` at 0xe2120).
  A zero result takes `0xe2144`, while nonzero falls through (`beq` at 0xe2134).
  Included word assignments through `[sp-24] +0x0` and `[sp-24] +0xc` write `ret(0x1b1b4)` (`str` at 0xe2180, `str` at 0xe2294).
  Header construction policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe2114 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe2118 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xe211c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe2128 | store | 4 | `sp -0x20` | `ret(0xe22f8)` |
| `ldr` at 0xe212c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe2138 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe213c | store | 4 | `sp -0x24` | `ret(0xe22f8)` |
| `ldr` at 0xe2148 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe2154 | store | 4 | `sp -0x20` | `ret(0xe2388)` |
| `ldr` at 0xe2158 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe2164 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe2168 | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0xe2170 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe2174 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe2180 | store | 4 | `[sp-24] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xe2184 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe2188 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xe218c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe2190 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe219c | store | 4 | `sp -0x20` | `ret(0xe2444)` |
| `ldr` at 0xe21a4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe21b0 | store | 4 | `sp -0x20` | `ret(0xe2388)` |
| `ldr` at 0xe21b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe21c0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe21c4 | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0xe21cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe21d0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe21dc | store | 4 | `[sp-24] +0x4` | `ret(0x1b1b4)` |
| `ldr` at 0xe21e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe21e4 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xe21e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe21ec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe21f8 | store | 4 | `sp -0x20` | `ret(0xe2444)` |
| `ldr` at 0xe2200 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe220c | store | 4 | `sp -0x20` | `ret(0xe2388)` |
| `ldr` at 0xe2210 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe221c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe2220 | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0xe2228 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe222c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe2238 | store | 4 | `[sp-24] +0x8` | `ret(0x1b1b4)` |
| `ldr` at 0xe223c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe2240 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xe2244 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe2248 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe2254 | store | 4 | `sp -0x20` | `ret(0xe2444)` |
| `ldr` at 0xe225c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe2268 | store | 4 | `sp -0x20` | `ret(0xe2388)` |
| `ldr` at 0xe226c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe2278 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe227c | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0xe2284 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe2288 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe2294 | store | 4 | `[sp-24] +0xc` | `ret(0x1b1b4)` |
| `ldr` at 0xe2298 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe229c | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xe22a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe22a4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe22b0 | store | 4 | `sp -0x20` | `ret(0xe2444)` |
| `ldr` at 0xe22b4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe22c0 | store | 4 | `sp -0x20` | `ret(0xe24c8)` |
| `ldr` at 0xe22c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe22d0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe22d4 | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0xe22dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe22e0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe22e4 | store | 4 | `[sp-24] +0x10` | `[sp-32]` |
| `str` at 0xe22ec | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe22f0 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xe22f4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe22f4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe22f4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe22f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe2120 | `(unknown callee) 0xe22f8` | `r1` | `r1` | `r2` | `r3` |
| `bl` at 0xe214c | `(unknown callee) 0xe2388` | `0x0` | `[sp-28]` | `?` | `ret(0xe22f8)` |
| `bl` at 0xe2178 | `(unknown callee) 0x1b1b4` | `[sp-32]` | `?` | `?` | `ret(0xe2388)` |
| `bl` at 0xe2194 | `(unknown callee) 0xe2444` | `[[sp-24]]` | `[sp-32]` | `[sp-28]` | `[sp-24]` |
| `bl` at 0xe21a8 | `(unknown callee) 0xe2388` | `0x0` | `[sp-28]` | `?` | `ret(0xe2444)` |
| `bl` at 0xe21d4 | `(unknown callee) 0x1b1b4` | `[sp-32]` | `?` | `?` | `ret(0xe2388)` |
| `bl` at 0xe21f0 | `(unknown callee) 0xe2444` | `[[sp-24]+4]` | `[sp-32]` | `[sp-28]` | `[sp-24]` |
| `bl` at 0xe2204 | `(unknown callee) 0xe2388` | `0x0` | `[sp-28]` | `?` | `ret(0xe2444)` |
| `bl` at 0xe2230 | `(unknown callee) 0x1b1b4` | `[sp-32]` | `?` | `?` | `ret(0xe2388)` |
| `bl` at 0xe224c | `(unknown callee) 0xe2444` | `[[sp-24]+8]` | `[sp-32]` | `[sp-28]` | `[sp-24]` |
| `bl` at 0xe2260 | `(unknown callee) 0xe2388` | `0x0` | `[sp-28]` | `?` | `ret(0xe2444)` |
| `bl` at 0xe228c | `(unknown callee) 0x1b1b4` | `[sp-32]` | `?` | `?` | `ret(0xe2388)` |
| `bl` at 0xe22a8 | `(unknown callee) 0xe2444` | `[[sp-24]+12]` | `[sp-32]` | `[sp-28]` | `[sp-24]` |
| `bl` at 0xe22b8 | `(unknown callee) 0xe24c8` | `[sp-28]` | `?` | `?` | `ret(0xe2444)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe2134 | `ret(0xe22f8) == 0x0` | `ret(0xe22f8) != 0x0` |
| `bne` at 0xe2160 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xe21bc | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xe2218 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xe2274 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xe22cc | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe22f4 | `r0` | `?` |

## `readhead13`

- `readhead13`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe22f8 to 0xe2388.
  Established. The unnamed `0x1b1f0` call carries `sp-40`, `0x1`, `0xd`, entry `r0` (`bl` at 0xe2320).
  A result equal to `0xd` takes `0xe2340`, while other results fall through (`beq` at 0xe2330).
  The unnamed `0x1b2e0` call carries `sp-40`, `0x143848`, `0xd`, `sp-40` (`bl` at 0xe2350).
  Header comparison meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe2308 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xe231c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe2324 | store | 4 | `sp -0x18` | `ret(0x1b1f0)` |
| `ldr` at 0xe2328 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe2338 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe2348 | load | 4 | `abs 0xe2384` | `-` |
| `str` at 0xe2358 | store | 4 | `sp -0x18` | `ret(0x1b2e0)` |
| `ldr` at 0xe235c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe236c | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xe2378 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe237c | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xe2380 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe2380 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe2380 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe2320 | `(unknown callee) 0x1b1f0` | `sp-40` | `0x1` | `0xd` | `r0` |
| `bl` at 0xe2350 | `(unknown callee) 0x1b2e0` | `sp-40` | `0x143848` | `0xd` | `sp-40` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe2330 | `ret(0x1b1f0) == 0xd` | `ret(0x1b1f0) != 0xd` |
| `beq` at 0xe2364 | `ret(0x1b2e0) == 0x0` | `ret(0x1b2e0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe2380 | `r0` | `[sp-44]` |

## `readsecthead`

- `readsecthead`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe2388 to 0xe2444.
  Established. Entry `r0 != 0x0` takes `0xe23b4`, while zero falls through (`bne` at 0xe23a8).
  The
  byte through `[sp-20] +0x1` is read (`ldrb` at 0xe23e4).
  The unnamed `0x1b1f0` call carries
  `sp-33`, `0x1`, `0x1`, `[sp-24]` (`bl` at 0xe2414).
  Section header layout remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe2398 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe239c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xe23a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe23b0 | store | 4 | `sp -0x14` | `sp-32` |
| `ldr` at 0xe23b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe23c0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe23c8 | store | 4 | `sp -0x1c` | `ret(0x1b1f0)` |
| `ldr` at 0xe23cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe23dc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xe23e4 | load | 1 | `[sp-20] +0x1` | `-` |
| `str` at 0xe23f8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe2410 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe2418 | store | 4 | `sp -0x1c` | `ret(0x1b1f0)` |
| `ldr` at 0xe241c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe242c | store | 4 | `sp -0x28` | `?` |
| `ldrb` at 0xe2434 | load | 1 | `sp -0x21` | `-` |
| `str` at 0xe2438 | store | 4 | `sp -0x28` | `[sp-33]:1` |
| `ldr` at 0xe243c | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xe2440 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe2440 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe2440 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe23c4 | `(unknown callee) 0x1b1f0` | `?` | `0x1` | `0x2` | `r1` |
| `bl` at 0xe2414 | `(unknown callee) 0x1b1f0` | `sp-33` | `0x1` | `0x1` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe23a8 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xe23d4 | `ret(0x1b1f0) != 0x2` | `ret(0x1b1f0) == 0x2` |
| `bne` at 0xe23ec | `[[sp-20]+1]:1 != 0x0` | `[[sp-20]+1]:1 == 0x0` |
| `beq` at 0xe2424 | `ret(0x1b1f0) == 0x1` | `ret(0x1b1f0) != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe2440 | `r0` | `[sp-40]` |

## `readsection`

- `readsection`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe2444 to 0xe24c8.
  Established. The unnamed `0x1b1f0` call carries entry `r0`, `0x1`, entry `r1`, entry `r2` (`bl` at 0xe2470).
  Inequality of its result and `[sp-24]` takes `0xe24ac`, while equality falls through
  (`bne` at 0xe2484).
  The byte read at an unknown target remains explicit (`ldrb` at 0xe249c).
  Section validation policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe2454 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe2458 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe245c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xe2460 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe2468 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe246c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe2474 | store | 4 | `sp -0x20` | `ret(0x1b1f0)` |
| `ldr` at 0xe2478 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe247c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe248c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe2490 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xe249c | load | 1 | `(unknown)` | `-` |
| `str` at 0xe24b0 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xe24bc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe24c0 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xe24c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe24c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe24c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe2470 | `(unknown callee) 0x1b1f0` | `r0` | `0x1` | `r1` | `r2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe2484 | `ret(0x1b1f0) != [sp-24]` | `ret(0x1b1f0) == [sp-24]` |
| `bne` at 0xe24a4 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe24c4 | `r0` | `[sp-36]` |

## `readlength`

- `readlength`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe24c8 to 0xe259c.
  Established. The unnamed `0x1b2e0` call carries `sp-24`, `0x13610c`, `0x2`, `sp-24` (`bl` at 0xe2514).
  A nonzero result takes `0xe2528`, while zero falls through (`bne` at 0xe2520).
  Included
  byte reads use `sp -0x1c`, `sp -0x1b` and `sp -0x1a` (`ldrb` at 0xe2568, `ldrb` at 0xe2570, `ldrb`
  at 0xe257c).
  Encoded length interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe24d8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xe24ec | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe24f4 | store | 4 | `sp -0x24` | `ret(0x1b1f0)` |
| `ldr` at 0xe24f8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe250c | load | 4 | `abs 0xe2598` | `-` |
| `str` at 0xe252c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe2544 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe254c | store | 4 | `sp -0x24` | `ret(0x1b1f0)` |
| `ldr` at 0xe2550 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe2560 | store | 4 | `sp -0x28` | `?` |
| `ldrb` at 0xe2568 | load | 1 | `sp -0x1c` | `-` |
| `ldrb` at 0xe2570 | load | 1 | `sp -0x1b` | `-` |
| `ldrb` at 0xe257c | load | 1 | `sp -0x1a` | `-` |
| `str` at 0xe2584 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe2588 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe258c | store | 4 | `sp -0x28` | `[sp-32]` |
| `ldr` at 0xe2590 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xe2594 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe2594 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe2594 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe24f0 | `(unknown callee) 0x1b1f0` | `sp-24` | `0x1` | `0x2` | `r0` |
| `bl` at 0xe2514 | `(unknown callee) 0x1b2e0` | `sp-24` | `0x13610c` | `0x2` | `sp-24` |
| `bl` at 0xe2548 | `(unknown callee) 0x1b1f0` | `sp-28` | `0x1` | `0x3` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe2500 | `ret(0x1b1f0) != 0x2` | `ret(0x1b1f0) == 0x2` |
| `bne` at 0xe2520 | `ret(0x1b2e0) != 0x0` | `ret(0x1b2e0) == 0x0` |
| `beq` at 0xe2558 | `ret(0x1b1f0) == 0x3` | `ret(0x1b1f0) != 0x3` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe2594 | `r0` | `[sp-40]` |
