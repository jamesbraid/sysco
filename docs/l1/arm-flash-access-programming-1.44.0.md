# ARM flash access and programming paths in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package4/flash-access-programming-facts.txt) retain all tool sections.

## `flash_init`

- `flash_init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbe494 to 0xbe5b4.
  Established. Zero `[0x2972f4]` takes `0xbe4c4`, while nonzero falls through (`beq` at 0xbe4b4).
  The unnamed `0x196dc` call carries `0x124f0e5`, `0x1`, `0x3b6`, `[0x12d99c]` (`bl` at 0xbe50c).
  Included word stores save its result at `0x12d99c` and set `0x2972f4` to `0x1` (`str` at 0xbe518, `str` at 0xbe584).
  Flash initialization dependencies remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbe4a4 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xbe4a8 | load | 4 | `abs 0xbe59c` | `-` |
| `ldr` at 0xbe4ac | load | 4 | `abs 0x2972f4` | `-` |
| `str` at 0xbe4bc | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xbe4c8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xbe4cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe4d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe4e0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe4e8 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xbe4f0 | load | 4 | `abs 0xbe5a0` | `-` |
| `ldr` at 0xbe4f4 | load | 4 | `abs 0x12d99c` | `-` |
| `ldr` at 0xbe500 | load | 4 | `abs 0xbe5a4` | `-` |
| `ldr` at 0xbe508 | load | 4 | `abs 0xbe5a8` | `-` |
| `ldr` at 0xbe514 | load | 4 | `abs 0xbe5a0` | `-` |
| `str` at 0xbe518 | store | 4 | `abs 0x12d99c` | `ret(0x196dc)` |
| `ldr` at 0xbe51c | load | 4 | `abs 0x12d99c` | `-` |
| `ldr` at 0xbe53c | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xbe548 | load | 4 | `abs 0xbe5ac` | `-` |
| `str` at 0xbe54c | store | 4 | `sp -0x2c` | `0x11f6e0` |
| `ldr` at 0xbe550 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `str` at 0xbe554 | store | 4 | `sp -0x28` | `[ret(0x193d0)]` |
| `str` at 0xbe558 | store | 4 | `sp -0x24` | `ret(0x193ac)` |
| `ldr` at 0xbe560 | load | 4 | `abs 0xbe5b0` | `-` |
| `str` at 0xbe574 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xbe57c | load | 4 | `abs 0xbe59c` | `-` |
| `str` at 0xbe584 | store | 4 | `abs 0x2972f4` | `0x1` |
| `str` at 0xbe58c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xbe590 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xbe598 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xbe598 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbe598 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbe598 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbe4dc | `(unknown callee) 0xbdff8` | `?` | `?` | `?` | `?` |
| `bl` at 0xbe50c | `(unknown callee) 0x196dc` | `0x124f0e5` | `0x1` | `0x3b6` | `[0x12d99c]` |
| `bl` at 0xbe528 | `(unknown callee) 0x193d0` | `ret(0x196dc)` | `?` | `ret(0x196dc)` | `[0x12d99c]` |
| `bl` at 0xbe534 | `(unknown callee) 0x193d0` | `ret(0x193d0)` | `?` | `?` | `ret(0x193d0)` |
| `bl` at 0xbe540 | `(unknown callee) 0x193ac` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |
| `bl` at 0xbe56c | `(unknown callee) 0x200a0` | `0x3` | `0x11f5c0` | `0xd9` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xbe4b4 | `[0x2972f4] == 0x0` | `[0x2972f4] != 0x0` |
| `bgt` at 0xbe4d4 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bge` at 0xbe4fc | `[0x12d99c] >= 0x0 (signed, 32-bit)` | `[0x12d99c] < 0x0 (signed, 32-bit)` |
| `bge` at 0xbe524 | `[0x12d99c] >= 0x0 (signed, 32-bit)` | `[0x12d99c] < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbe598 | `r0` | `?` |

## `flash_lock`

- `flash_lock`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbe5b4 to 0xbe66c.
  Established. Signed `[0x12d99c] < 0x0` takes `0xbe648`, while nonnegative values fall through (`blt` at 0xbe5d0).
  The unnamed `0x19724` call carries `[0x12d99c]`, `0x12d984`, `0x2`, `0x12d99c` (`bl` at 0xbe5e4).
  An included word assignment clears `sp -0x18` (`str` at 0xbe64c).
  Lock operation semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xbe5c4 | load | 4 | `abs 0xbe65c` | `-` |
| `ldr` at 0xbe5c8 | load | 4 | `abs 0x12d99c` | `-` |
| `ldr` at 0xbe5d4 | load | 4 | `abs 0xbe65c` | `-` |
| `ldr` at 0xbe5d8 | load | 4 | `abs 0x12d99c` | `-` |
| `ldr` at 0xbe5dc | load | 4 | `abs 0xbe660` | `-` |
| `ldr` at 0xbe608 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xbe614 | load | 4 | `abs 0xbe664` | `-` |
| `str` at 0xbe618 | store | 4 | `sp -0x24` | `0x11f724` |
| `ldr` at 0xbe61c | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `str` at 0xbe620 | store | 4 | `sp -0x20` | `[ret(0x193d0)]` |
| `str` at 0xbe624 | store | 4 | `sp -0x1c` | `ret(0x193ac)` |
| `ldr` at 0xbe62c | load | 4 | `abs 0xbe668` | `-` |
| `str` at 0xbe640 | store | 4 | `sp -0x18` | `?` |
| `str` at 0xbe64c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xbe650 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xbe658 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xbe658 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbe658 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbe658 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbe5e4 | `(unknown callee) 0x19724` | `[0x12d99c]` | `0x12d984` | `0x2` | `0x12d99c` |
| `bl` at 0xbe5f4 | `(unknown callee) 0x193d0` | `ret(0x19724)` | `?` | `?` | `ret(0x19724)` |
| `bl` at 0xbe600 | `(unknown callee) 0x193d0` | `ret(0x193d0)` | `?` | `?` | `ret(0x193d0)` |
| `bl` at 0xbe60c | `(unknown callee) 0x193ac` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |
| `bl` at 0xbe638 | `(unknown callee) 0x200a0` | `0x4` | `0x11f5c0` | `0xea` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xbe5d0 | `[0x12d99c] < 0x0 (signed, 32-bit)` | `[0x12d99c] >= 0x0 (signed, 32-bit)` |
| `bge` at 0xbe5f0 | `ret(0x19724) >= 0x0 (signed, 32-bit)` | `ret(0x19724) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbe658 | `r0` | `?` |

## `flash_unlock`

- `flash_unlock`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbe66c to 0xbe724.
  Established. The unnamed `0x19724` call carries `[0x12d99c]`, `0x12d994`, `0x1`, `0x12d99c` (`bl` at 0xbe69c);
  its signed nonnegative result takes `0xbe700`, while negative results fall through (`bge` at 0xbe6a8).
  A word store assigns `0x0` to `sp -0x18` on an included path (`str` at 0xbe704).
  Flash release behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xbe67c | load | 4 | `abs 0xbe714` | `-` |
| `ldr` at 0xbe680 | load | 4 | `abs 0x12d99c` | `-` |
| `ldr` at 0xbe68c | load | 4 | `abs 0xbe714` | `-` |
| `ldr` at 0xbe690 | load | 4 | `abs 0x12d99c` | `-` |
| `ldr` at 0xbe694 | load | 4 | `abs 0xbe718` | `-` |
| `ldr` at 0xbe6c0 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xbe6cc | load | 4 | `abs 0xbe71c` | `-` |
| `str` at 0xbe6d0 | store | 4 | `sp -0x24` | `0x11f760` |
| `ldr` at 0xbe6d4 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `str` at 0xbe6d8 | store | 4 | `sp -0x20` | `[ret(0x193d0)]` |
| `str` at 0xbe6dc | store | 4 | `sp -0x1c` | `ret(0x193ac)` |
| `ldr` at 0xbe6e4 | load | 4 | `abs 0xbe720` | `-` |
| `str` at 0xbe6f8 | store | 4 | `sp -0x18` | `?` |
| `str` at 0xbe704 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xbe708 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xbe710 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xbe710 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbe710 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbe710 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbe69c | `(unknown callee) 0x19724` | `[0x12d99c]` | `0x12d994` | `0x1` | `0x12d99c` |
| `bl` at 0xbe6ac | `(unknown callee) 0x193d0` | `ret(0x19724)` | `?` | `?` | `ret(0x19724)` |
| `bl` at 0xbe6b8 | `(unknown callee) 0x193d0` | `ret(0x193d0)` | `?` | `?` | `ret(0x193d0)` |
| `bl` at 0xbe6c4 | `(unknown callee) 0x193ac` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |
| `bl` at 0xbe6f0 | `(unknown callee) 0x200a0` | `0x4` | `0x11f5c0` | `0xf5` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xbe688 | `[0x12d99c] < 0x0 (signed, 32-bit)` | `[0x12d99c] >= 0x0 (signed, 32-bit)` |
| `bge` at 0xbe6a8 | `ret(0x19724) >= 0x0 (signed, 32-bit)` | `ret(0x19724) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbe710 | `r0` | `?` |

## `flash_addr_to_index`

- `flash_addr_to_index`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbe724 to 0xbe7c0.
  Established. A nonzero `[0x2972f4]` takes `0xbe770`, while zero falls through (`bne` at 0xbe744).
  The unnamed `0xbe494` call carries `0x0`, entry `r1`, entry `r2`, `[0x2972f4]` (`bl` at 0xbe74c).
  An included word assignment saves `[sp-24]` at `sp -0x20` (`str` at 0xbe7ac).
  Address index interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbe734 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xbe738 | load | 4 | `abs 0xbe7bc` | `-` |
| `ldr` at 0xbe73c | load | 4 | `abs 0x2972f4` | `-` |
| `str` at 0xbe754 | store | 4 | `sp -0x1c` | `ret(0xbe494)` |
| `ldr` at 0xbe758 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe764 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe768 | store | 4 | `sp -0x20` | `ret(0xbe494)` |
| `ldr` at 0xbe770 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbe77c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xbe780 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe78c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbe7a0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xbe7a8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbe7ac | store | 4 | `sp -0x20` | `[sp-24]` |
| `ldr` at 0xbe7b0 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xbe7b8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbe7b8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbe7b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbe74c | `(unknown callee) 0xbe494` | `0x0` | `r1` | `r2` | `[0x2972f4]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xbe744 | `[0x2972f4] != 0x0` | `[0x2972f4] == 0x0` |
| `bge` at 0xbe760 | `ret(0xbe494) >= 0x0 (signed, 32-bit)` | `ret(0xbe494) < 0x0 (signed, 32-bit)` |
| `blt` at 0xbe788 | `[sp-24] < 0x0 (signed, 32-bit)` | `[sp-24] >= 0x0 (signed, 32-bit)` |
| `bgt` at 0xbe794 | `[sp-24] > 0x2 (signed, 32-bit)` | `[sp-24] <= 0x2 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbe7b8 | `r0` | `[sp-32]` |

## `flash_offset`

- `flash_offset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbe7c0 to 0xbe850.
  Established. The unnamed `0xbe724` call carries entry `r0`, `r1`, `r2`, `r3` (`bl` at 0xbe7d8);
  its signed nonnegative result takes `0xbe7fc`, while negative results fall through (`bge` at 0xbe7ec).
  The included indexed word loads have unknown effective targets (`ldr` at 0xbe80c, `ldr` at 0xbe830).
  Offset calculation limits remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbe7d0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xbe7d4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbe7e0 | store | 4 | `sp -0x1c` | `ret(0xbe724)` |
| `ldr` at 0xbe7e4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe7f4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xbe7fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe804 | load | 4 | `abs 0xbe848` | `-` |
| `ldr` at 0xbe808 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe80c | load | 4 | `(unknown)` | `-` |
| `str` at 0xbe81c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xbe824 | load | 4 | `abs 0xbe84c` | `-` |
| `ldr` at 0xbe828 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe82c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe830 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbe838 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xbe83c | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xbe844 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbe844 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbe844 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbe7d8 | `(unknown callee) 0xbe724` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xbe7ec | `ret(0xbe724) >= 0x0 (signed, 32-bit)` | `ret(0xbe724) < 0x0 (signed, 32-bit)` |
| `blo` at 0xbe814 | `? < ? (unsigned, 32-bit)` | `? >= ? (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbe844 | `r0` | `[sp-32]` |

## `flash_read_addr`

- `flash_read_addr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbe850 to 0xbe968.
  Established. Signed `ret(0xbe724) >= 0x0` takes `0xbe88c`, while negative results fall through (`bge` at 0xbe87c).
  The unnamed `0xeb530` call carries unknown, `sp-112`, `sp-112`, `[sp-120]` (`bl` at 0xbe8bc).
  An included word store retains `ret(0xbe7c0)` at `sp -0x74` (`str` at 0xbe898).
  Read address mapping remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbe860 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xbe864 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbe870 | store | 4 | `sp -0x78` | `ret(0xbe724)` |
| `ldr` at 0xbe874 | load | 4 | `sp -0x78` | `-` |
| `str` at 0xbe884 | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0xbe88c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbe898 | store | 4 | `sp -0x74` | `ret(0xbe7c0)` |
| `ldr` at 0xbe89c | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xbe8a8 | load | 4 | `abs 0xbe954` | `-` |
| `ldr` at 0xbe8ac | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xbe8b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbe8d4 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xbe8e0 | load | 4 | `abs 0xbe958` | `-` |
| `str` at 0xbe8e4 | store | 4 | `sp -0x88` | `0x11f7a0` |
| `ldr` at 0xbe8e8 | load | 4 | `sp -0x78` | `-` |
| `str` at 0xbe8ec | store | 4 | `sp -0x84` | `[sp-120]` |
| `str` at 0xbe8f0 | store | 4 | `sp -0x80` | `ret(0x1f8e0)` |
| `ldr` at 0xbe8f8 | load | 4 | `abs 0xbe95c` | `-` |
| `ldr` at 0xbe8fc | load | 4 | `abs 0xbe960` | `-` |
| `str` at 0xbe90c | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0xbe914 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbe918 | load | 4 | `sp -0x74` | `-` |
| `str` at 0xbe928 | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0xbe930 | load | 4 | `abs 0xbe964` | `-` |
| `ldr` at 0xbe934 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xbe938 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbe93c | load | 4 | `sp -0x74` | `-` |
| `str` at 0xbe944 | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0xbe948 | load | 4 | `sp -0x7c` | `-` |
| `ldm` at 0xbe950 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbe950 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbe950 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbe868 | `(unknown callee) 0xbe724` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xbe890 | `(unknown callee) 0xbe7c0` | `[sp-20]` | `?` | `?` | `ret(0xbe724)` |
| `bl` at 0xbe8bc | `(unknown callee) 0xeb530` | `?` | `sp-112` | `sp-112` | `[sp-120]` |
| `bl` at 0xbe8cc | `(unknown callee) 0x193d0` | `ret(0xeb530)` | `?` | `?` | `ret(0xeb530)` |
| `bl` at 0xbe8d8 | `(unknown callee) 0x1f8e0` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |
| `bl` at 0xbe904 | `(unknown callee) 0x200a0` | `0x3` | `0x11f5c0` | `0x12d` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xbe87c | `ret(0xbe724) >= 0x0 (signed, 32-bit)` | `ret(0xbe724) < 0x0 (signed, 32-bit)` |
| `ble` at 0xbe8a4 | `[sp-120] <= 0x0 (signed, 32-bit)` | `[sp-120] > 0x0 (signed, 32-bit)` |
| `bge` at 0xbe8c8 | `ret(0xeb530) >= 0x0 (signed, 32-bit)` | `ret(0xeb530) < 0x0 (signed, 32-bit)` |
| `bhi` at 0xbe920 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbe950 | `r0` | `[sp-124]` |

## `flash_write_addr`

- `flash_write_addr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbe968 to 0xbea08.
  Established. An included word load reads absolute `0x29c3a8` (`ldr` at 0xbe9b8).
  The unnamed `0xbe724` call carries `[0x29c3a8]`, unknown, unknown, `0x29c398` (`bl` at 0xbe9bc).
  Inequality `ret(0xbe724) != [sp-28]` takes `0xbe9dc`, while equality falls through (`bne` at 0xbe9cc).
  Write address eligibility remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbe978 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xbe97c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbe988 | store | 4 | `sp -0x1c` | `ret(0xbe724)` |
| `ldr` at 0xbe98c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe99c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xbe9a4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbe9b0 | store | 4 | `sp -0x18` | `ret(0xbe7c0)` |
| `ldr` at 0xbe9b4 | load | 4 | `abs 0xbea00` | `-` |
| `ldr` at 0xbe9b8 | load | 4 | `abs 0x29c3a8` | `-` |
| `ldr` at 0xbe9c4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe9d4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xbe9dc | load | 4 | `abs 0xbea04` | `-` |
| `ldr` at 0xbe9e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe9e4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbe9e8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbe9f0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xbe9f4 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xbe9fc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbe9fc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbe9fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbe980 | `(unknown callee) 0xbe724` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xbe9a8 | `(unknown callee) 0xbe7c0` | `[sp-20]` | `?` | `?` | `ret(0xbe724)` |
| `bl` at 0xbe9bc | `(unknown callee) 0xbe724` | `[0x29c3a8]` | `?` | `?` | `0x29c398` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xbe994 | `ret(0xbe724) >= 0x0 (signed, 32-bit)` | `ret(0xbe724) < 0x0 (signed, 32-bit)` |
| `bne` at 0xbe9cc | `ret(0xbe724) != [sp-28]` | `ret(0xbe724) == [sp-28]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbe9fc | `r0` | `[sp-32]` |

## `flash_read`

- `flash_read`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbea08 to 0xbea9c.
  Established. Nonzero `[0x2972f4]` takes `0xbea5c`, while zero falls through (`bne` at 0xbea30).
  The unnamed `0x19610` call carries `[sp-20]`, `[sp-32]`, `[sp-28]`, unknown (`bl` at 0xbea7c).
  An included word assignment clears `sp -0x28` (`str` at 0xbea88).
  Flash read transfer semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbea18 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbea1c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xbea20 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xbea24 | load | 4 | `abs 0xbea98` | `-` |
| `ldr` at 0xbea28 | load | 4 | `abs 0x2972f4` | `-` |
| `str` at 0xbea40 | store | 4 | `sp -0x24` | `ret(0xbe494)` |
| `ldr` at 0xbea44 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbea50 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbea54 | store | 4 | `sp -0x28` | `ret(0xbe494)` |
| `ldr` at 0xbea5c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbea68 | store | 4 | `sp -0x20` | `ret(0xbe850)` |
| `ldr` at 0xbea70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbea74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbea78 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbea88 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xbea8c | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xbea94 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbea94 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbea94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbea38 | `(unknown callee) 0xbe494` | `0x0` | `r1` | `r2` | `[0x2972f4]` |
| `bl` at 0xbea60 | `(unknown callee) 0xbe850` | `?` | `?` | `?` | `?` |
| `bl` at 0xbea6c | `(unknown callee) 0xbe5b4` | `ret(0xbe850)` | `?` | `?` | `ret(0xbe850)` |
| `bl` at 0xbea7c | `(unknown callee) 0x19610` | `[sp-20]` | `[sp-32]` | `[sp-28]` | `?` |
| `bl` at 0xbea80 | `(unknown callee) 0xbe66c` | `ret(0x19610)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xbea30 | `[0x2972f4] != 0x0` | `[0x2972f4] == 0x0` |
| `bge` at 0xbea4c | `ret(0xbe494) >= 0x0 (signed, 32-bit)` | `ret(0xbe494) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbea94 | `r0` | `?` |

## `flash_erase`

- `flash_erase`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbea9c to 0xbead0.
  Established. Word stores preserve entry `r0`, `r1`, `r2` at `sp -0x14`, `sp -0x18`, `sp -0x1c` (`str` at 0xbeaac, `str` at 0xbeab0, `str` at 0xbeab4).
  Another word assignment clears `sp -0x24` (`str` at 0xbeabc).
  The integer return record contains `0x0` (`ldm` at 0xbeacc).
  Erasure behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbeaac | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbeab0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xbeab4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xbeabc | store | 4 | `sp -0x24` | `0x0` |
| `ldm` at 0xbeacc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbeacc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbeacc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbeacc | `r0` | `0x0` |

## `flash_program`

- `flash_program`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbead0 to 0xbef2c.
  Established. The unnamed `0x19718` call carries unknown, `0xc00c4d04`, `sp-92`, `[sp-40]` (`bl` at 0xbec44);
  its zero result takes `0xbec9c`, while nonzero falls through (`beq` at 0xbec50).
  A word store assigns `[sp-20]` to `[sp-32] +0x0` on an included path (`str` at 0xbeec8).
  Programming request and result meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbeae0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbeae4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xbeae8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xbeaec | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xbeaf4 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xbeaf8 | load | 4 | `abs 0xbeee4` | `-` |
| `ldr` at 0xbeafc | load | 4 | `abs 0x2972f4` | `-` |
| `str` at 0xbeb14 | store | 4 | `sp -0x40` | `ret(0xbe494)` |
| `ldr` at 0xbeb18 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbeb24 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbeb28 | store | 4 | `sp -0x60` | `ret(0xbe494)` |
| `ldr` at 0xbeb34 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbeb40 | store | 4 | `sp -0x28` | `ret(0xbe724)` |
| `ldr` at 0xbeb44 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbeb50 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbeb5c | store | 4 | `sp -0x30` | `ret(0xbe968)` |
| `ldr` at 0xbeb60 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbeb70 | load | 4 | `abs 0xbeee8` | `-` |
| `str` at 0xbeb74 | store | 4 | `sp -0x2c` | `0xfffffc7c` |
| `ldr` at 0xbeb7c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbeb88 | store | 4 | `sp -0x34` | `ret(0xbe7c0)` |
| `ldr` at 0xbeb8c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbeb98 | load | 4 | `abs 0xbeeec` | `-` |
| `ldr` at 0xbeb9c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbeba4 | load | 4 | `abs 0x29c3b8` | `-` |
| `ldr` at 0xbebb8 | load | 4 | `abs 0xbeef0` | `-` |
| `str` at 0xbebbc | store | 4 | `sp -0x70` | `0x11f7d8` |
| `ldr` at 0xbebc0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbebc4 | store | 4 | `sp -0x6c` | `[sp-20]` |
| `ldr` at 0xbebcc | load | 4 | `abs 0xbeef4` | `-` |
| `ldr` at 0xbebd0 | load | 4 | `abs 0xbeef8` | `-` |
| `str` at 0xbebe0 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xbebec | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xbebf0 | load | 4 | `abs 0xbeeec` | `-` |
| `ldr` at 0xbebf4 | load | 4 | `abs 0x29c3b8` | `-` |
| `ldr` at 0xbebf8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbec00 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbec10 | store | 4 | `sp -0x5c` | `0x0` |
| `str` at 0xbec18 | store | 4 | `sp -0x58` | `0x10` |
| `str` at 0xbec20 | store | 4 | `sp -0x54` | `sp-80` |
| `ldr` at 0xbec24 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbec28 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0xbec2c | load | 4 | `abs 0xbeefc` | `-` |
| `ldr` at 0xbec30 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbec38 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbec3c | load | 4 | `abs 0xbef00` | `-` |
| `ldr` at 0xbec5c | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xbec68 | load | 4 | `abs 0xbef04` | `-` |
| `str` at 0xbec6c | store | 4 | `sp -0x70` | `0x11f814` |
| `ldr` at 0xbec70 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbec74 | store | 4 | `sp -0x6c` | `[sp-64]` |
| `str` at 0xbec78 | store | 4 | `sp -0x68` | `ret(0x1f8e0)` |
| `ldr` at 0xbec80 | load | 4 | `abs 0xbeef4` | `-` |
| `ldr` at 0xbec84 | load | 4 | `abs 0xbef08` | `-` |
| `str` at 0xbec94 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0xbec9c | load | 1 | `sp -0x4b` | `-` |
| `ldr` at 0xbeca8 | load | 4 | `abs 0xbeeec` | `-` |
| `ldr` at 0xbecac | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbecb0 | load | 4 | `abs 0x29c3b8` | `-` |
| `str` at 0xbecb8 | store | 4 | `sp -0x34` | `([sp-52] + [0x29c3b8])` |
| `ldr` at 0xbecbc | load | 4 | `abs 0xbeeec` | `-` |
| `ldr` at 0xbecc0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbecc4 | load | 4 | `abs 0x29c3b8` | `-` |
| `str` at 0xbeccc | store | 4 | `sp -0x30` | `([sp-48] + [0x29c3b8])` |
| `ldr` at 0xbecd0 | load | 4 | `abs 0xbeeec` | `-` |
| `ldr` at 0xbecd4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbecd8 | load | 4 | `abs 0x29c3b8` | `-` |
| `str` at 0xbece0 | store | 4 | `sp -0x40` | `([sp-64] + [0x29c3b8])` |
| `ldr` at 0xbece8 | load | 4 | `abs 0xbef0c` | `-` |
| `ldr` at 0xbecec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbecf0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbecf4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbed14 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xbed20 | load | 4 | `abs 0xbef10` | `-` |
| `str` at 0xbed24 | store | 4 | `sp -0x70` | `0x11f848` |
| `ldr` at 0xbed28 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbed2c | store | 4 | `sp -0x6c` | `[sp-20]` |
| `str` at 0xbed30 | store | 4 | `sp -0x68` | `ret(0x1f8e0)` |
| `ldr` at 0xbed38 | load | 4 | `abs 0xbeef4` | `-` |
| `ldr` at 0xbed3c | load | 4 | `abs 0xbef14` | `-` |
| `str` at 0xbed4c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbed54 | load | 4 | `abs 0xbef0c` | `-` |
| `ldr` at 0xbed58 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbed5c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbed60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbed64 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbed70 | store | 4 | `sp -0x2c` | `ret(0x192e0)` |
| `ldr` at 0xbed74 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbed78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbed8c | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xbed98 | load | 4 | `abs 0xbef18` | `-` |
| `str` at 0xbed9c | store | 4 | `sp -0x70` | `0x11f874` |
| `ldr` at 0xbeda0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbeda4 | store | 4 | `sp -0x6c` | `[sp-20]` |
| `ldr` at 0xbeda8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbedac | store | 4 | `sp -0x68` | `[sp-44]` |
| `str` at 0xbedb0 | store | 4 | `sp -0x64` | `ret(0x1f8e0)` |
| `ldr` at 0xbedb8 | load | 4 | `abs 0xbeef4` | `-` |
| `ldr` at 0xbedbc | load | 4 | `abs 0xbef1c` | `-` |
| `str` at 0xbedcc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbedd4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbedd8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbedf8 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xbee04 | load | 4 | `abs 0xbef20` | `-` |
| `str` at 0xbee08 | store | 4 | `sp -0x70` | `0x11f8a4` |
| `ldr` at 0xbee0c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbee10 | store | 4 | `sp -0x6c` | `[sp-20]` |
| `str` at 0xbee14 | store | 4 | `sp -0x68` | `ret(0x1f8e0)` |
| `ldr` at 0xbee1c | load | 4 | `abs 0xbeef4` | `-` |
| `ldr` at 0xbee20 | load | 4 | `abs 0xbef24` | `-` |
| `str` at 0xbee30 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xbee3c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xbee40 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbee44 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbee50 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbee54 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbee5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbee60 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xbee68 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xbee6c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbee78 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbee7c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbee84 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xbee88 | load | 4 | `abs 0xbef28` | `-` |
| `str` at 0xbee8c | store | 4 | `sp -0x2c` | `0xfffffc7a` |
| `ldr` at 0xbee94 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbee9c | store | 4 | `sp -0x24` | `?` |
| `str` at 0xbeeac | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xbeeb4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbeec0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbeec4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbeec8 | store | 4 | `[sp-32] +0x0` | `[sp-20]` |
| `ldr` at 0xbeed0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbeed4 | store | 4 | `sp -0x60` | `[sp-44]` |
| `ldr` at 0xbeed8 | load | 4 | `sp -0x60` | `-` |
| `ldm` at 0xbeee0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbeee0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbeee0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbeb0c | `(unknown callee) 0xbe494` | `0x0` | `r1` | `r2` | `[0x2972f4]` |
| `bl` at 0xbeb30 | `(unknown callee) 0xbe5b4` | `?` | `?` | `?` | `?` |
| `bl` at 0xbeb38 | `(unknown callee) 0xbe724` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0xbeb54 | `(unknown callee) 0xbe968` | `[sp-20]` | `?` | `?` | `ret(0xbe724)` |
| `bl` at 0xbeb80 | `(unknown callee) 0xbe7c0` | `[sp-20]` | `?` | `?` | `ret(0xbe968)` |
| `bl` at 0xbeba8 | `(unknown callee) 0xeb29c` | `ret(0xbe7c0)` | `[0x29c3b8]` | `ret(0xbe7c0)` | `0x29c398` |
| `bl` at 0xbebd8 | `(unknown callee) 0x200a0` | `0x3` | `0x11f5c0` | `0x193` | `0x0` |
| `bl` at 0xbec44 | `(unknown callee) 0x19718` | `?` | `0xc00c4d04` | `sp-92` | `[sp-40]` |
| `bl` at 0xbec54 | `(unknown callee) 0x193d0` | `ret(0x19718)` | `?` | `?` | `ret(0x19718)` |
| `bl` at 0xbec60 | `(unknown callee) 0x1f8e0` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |
| `bl` at 0xbec8c | `(unknown callee) 0x200a0` | `0x3` | `0x11f5c0` | `0x19e` | `0x0` |
| `bl` at 0xbecfc | `(unknown callee) 0x195f8` | `?` | `?` | `0x0` | `0x12d9ac` |
| `bl` at 0xbed0c | `(unknown callee) 0x193d0` | `ret(0x195f8)` | `?` | `?` | `ret(0x195f8)` |
| `bl` at 0xbed18 | `(unknown callee) 0x1f8e0` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |
| `bl` at 0xbed44 | `(unknown callee) 0x200a0` | `0x3` | `0x11f5c0` | `0x1af` | `0x0` |
| `bl` at 0xbed68 | `(unknown callee) 0x192e0` | `?` | `[sp-24]` | `[sp-28]` | `[sp-40]` |
| `bl` at 0xbed84 | `(unknown callee) 0x193d0` | `ret(0x192e0)` | `?` | `ret(0x192e0)` | `[sp-28]` |
| `bl` at 0xbed90 | `(unknown callee) 0x1f8e0` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |
| `bl` at 0xbedc4 | `(unknown callee) 0x200a0` | `0x3` | `0x11f5c0` | `0x1b5` | `0x0` |
| `bl` at 0xbede0 | `(unknown callee) 0x194c0` | `[sp-48]` | `[sp-28]` | `0x6` | `[sp-28]` |
| `bl` at 0xbedf0 | `(unknown callee) 0x193d0` | `ret(0x194c0)` | `?` | `?` | `ret(0x194c0)` |
| `bl` at 0xbedfc | `(unknown callee) 0x1f8e0` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |
| `bl` at 0xbee28 | `(unknown callee) 0x200a0` | `0x3` | `0x11f5c0` | `0x1bd` | `0x0` |
| `bl` at 0xbeea4 | `(unknown callee) 0xbe66c` | `ret(0x194c0)` | `?` | `?` | `[sp-28]` |
| `bl` at 0xbeecc | `(unknown callee) 0xbe66c` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xbeb04 | `[0x2972f4] != 0x0` | `[0x2972f4] == 0x0` |
| `bge` at 0xbeb20 | `ret(0xbe494) >= 0x0 (signed, 32-bit)` | `ret(0xbe494) < 0x0 (signed, 32-bit)` |
| `beq` at 0xbeb4c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xbeb68 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xbeb94 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `beq` at 0xbebb4 | `ret(0xeb29c) == 0x0` | `ret(0xeb29c) != 0x0` |
| `bls` at 0xbec08 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0xbec50 | `ret(0x19718) == 0x0` | `ret(0x19718) != 0x0` |
| `beq` at 0xbeca4 | `[sp-75]:1 == 0xff` | `[sp-75]:1 != 0xff` |
| `bge` at 0xbed08 | `ret(0x195f8) >= 0x0 (signed, 32-bit)` | `ret(0x195f8) < 0x0 (signed, 32-bit)` |
| `beq` at 0xbed80 | `ret(0x192e0) == [sp-28]` | `ret(0x192e0) != [sp-28]` |
| `bge` at 0xbedec | `ret(0x194c0) >= 0x0 (signed, 32-bit)` | `ret(0x194c0) < 0x0 (signed, 32-bit)` |
| `bhs` at 0xbee4c | `C == 1` | `not (C == 1)` |
| `beq` at 0xbee74 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xbeebc | `[sp-32] == 0x0` | `[sp-32] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbeee0 | `r0` | `?` |

## `flash_set_boot_image`

- `flash_set_boot_image`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbef2c to 0xbf154.
  Established. Included byte assignments put `0x41` and `0x42` at `sp -0x15` on separate paths (`strb` at 0xbefe0, `strb` at 0xbf000).
  The unnamed `0x192e0` call carries `ret(0x1967c)`, `sp-21`, `0x1`, `sp-21` (`bl` at 0xbf0b0).
  Equality `ret(0x192e0) == 0x1` takes `0xbf108`, while inequality falls through (`beq` at 0xbf0bc).
  Boot image selection persistence remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbef3c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xbef40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbef4c | load | 4 | `abs 0xbf124` | `-` |
| `ldr` at 0xbef68 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xbef7c | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xbef88 | load | 4 | `abs 0xbf128` | `-` |
| `str` at 0xbef8c | store | 4 | `sp -0x2c` | `0x11f8e4` |
| `ldr` at 0xbef90 | load | 4 | `abs 0xbf124` | `-` |
| `str` at 0xbef94 | store | 4 | `sp -0x28` | `0x11f8d0` |
| `str` at 0xbef98 | store | 4 | `sp -0x24` | `ret(0x1f8e0)` |
| `ldr` at 0xbefa0 | load | 4 | `abs 0xbf12c` | `-` |
| `str` at 0xbefb4 | store | 4 | `sp -0x20` | `?` |
| `str` at 0xbefc0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xbefc8 | load | 4 | `abs 0xbf130` | `-` |
| `ldr` at 0xbefcc | load | 4 | `abs 0x29c3a0` | `-` |
| `ldr` at 0xbefd0 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xbefe0 | store | 1 | `sp -0x15` | `0x41` |
| `ldr` at 0xbefe8 | load | 4 | `abs 0xbf130` | `-` |
| `ldr` at 0xbefec | load | 4 | `abs 0x29c3a4` | `-` |
| `ldr` at 0xbeff0 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xbf000 | store | 1 | `sp -0x15` | `0x42` |
| `ldr` at 0xbf008 | load | 4 | `abs 0xbf134` | `-` |
| `str` at 0xbf00c | store | 4 | `sp -0x2c` | `0x11f910` |
| `ldr` at 0xbf010 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbf014 | store | 4 | `sp -0x28` | `r0` |
| `ldr` at 0xbf01c | load | 4 | `abs 0xbf12c` | `-` |
| `ldr` at 0xbf020 | load | 4 | `abs 0xbf138` | `-` |
| `str` at 0xbf030 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xbf038 | load | 4 | `abs 0xbf124` | `-` |
| `ldr` at 0xbf03c | load | 4 | `abs 0xbf13c` | `-` |
| `str` at 0xbf048 | store | 4 | `sp -0x1c` | `ret(0x1967c)` |
| `ldr` at 0xbf04c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf060 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xbf06c | load | 4 | `abs 0xbf140` | `-` |
| `str` at 0xbf070 | store | 4 | `sp -0x2c` | `0x11f938` |
| `ldr` at 0xbf074 | load | 4 | `abs 0xbf144` | `-` |
| `str` at 0xbf078 | store | 4 | `sp -0x28` | `0x11f964` |
| `str` at 0xbf07c | store | 4 | `sp -0x24` | `ret(0x1f8e0)` |
| `ldr` at 0xbf084 | load | 4 | `abs 0xbf12c` | `-` |
| `ldr` at 0xbf088 | load | 4 | `abs 0xbf148` | `-` |
| `str` at 0xbf098 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xbf0a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf0c8 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xbf0d4 | load | 4 | `abs 0xbf14c` | `-` |
| `str` at 0xbf0d8 | store | 4 | `sp -0x2c` | `0x11f97c` |
| `ldr` at 0xbf0dc | load | 4 | `abs 0xbf124` | `-` |
| `str` at 0xbf0e0 | store | 4 | `sp -0x28` | `0x11f8d0` |
| `str` at 0xbf0e4 | store | 4 | `sp -0x24` | `ret(0x1f8e0)` |
| `ldr` at 0xbf0ec | load | 4 | `abs 0xbf12c` | `-` |
| `ldr` at 0xbf0f0 | load | 4 | `abs 0xbf150` | `-` |
| `str` at 0xbf100 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xbf108 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbf114 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xbf118 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xbf120 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbf120 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbf120 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbef50 | `(unknown callee) 0x19358` | `0x11f8d0` | `r1` | `r2` | `r0` |
| `bl` at 0xbef60 | `(unknown callee) 0x193d0` | `ret(0x19358)` | `?` | `?` | `ret(0x19358)` |
| `bl` at 0xbef74 | `(unknown callee) 0x193d0` | `ret(0x193d0)` | `?` | `?` | `[ret(0x193d0)]` |
| `bl` at 0xbef80 | `(unknown callee) 0x1f8e0` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |
| `bl` at 0xbefac | `(unknown callee) 0x200a0` | `0x3` | `0x11f5c0` | `0x1e8` | `0x0` |
| `bl` at 0xbf028 | `(unknown callee) 0x200a0` | `0x3` | `0x11f5c0` | `0x1f2` | `0x0` |
| `bl` at 0xbf040 | `(unknown callee) 0x1967c` | `0x11f8d0` | `0x241` | `?` | `?` |
| `bl` at 0xbf058 | `(unknown callee) 0x193d0` | `ret(0x1967c)` | `?` | `?` | `ret(0x1967c)` |
| `bl` at 0xbf064 | `(unknown callee) 0x1f8e0` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |
| `bl` at 0xbf090 | `(unknown callee) 0x200a0` | `0x3` | `0x11f5c0` | `0x1f7` | `0x0` |
| `bl` at 0xbf0b0 | `(unknown callee) 0x192e0` | `ret(0x1967c)` | `sp-21` | `0x1` | `sp-21` |
| `bl` at 0xbf0c0 | `(unknown callee) 0x193d0` | `ret(0x192e0)` | `?` | `?` | `ret(0x192e0)` |
| `bl` at 0xbf0cc | `(unknown callee) 0x1f8e0` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |
| `bl` at 0xbf0f8 | `(unknown callee) 0x200a0` | `0x3` | `0x11f5c0` | `0x1fd` | `0x0` |
| `bl` at 0xbf10c | `(unknown callee) 0x1931c` | `[sp-28]` | `?` | `?` | `ret(0x192e0)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xbef48 | `r0 != 0x0` | `r0 == 0x0` |
| `bge` at 0xbef5c | `ret(0x19358) >= 0x0 (signed, 32-bit)` | `ret(0x19358) < 0x0 (signed, 32-bit)` |
| `beq` at 0xbef70 | `[ret(0x193d0)] == 0x2` | `[ret(0x193d0)] != 0x2` |
| `bne` at 0xbefd8 | `[0x29c3a0] != r0` | `[0x29c3a0] == r0` |
| `bne` at 0xbeff8 | `[0x29c3a4] != r0` | `[0x29c3a4] == r0` |
| `bge` at 0xbf054 | `ret(0x1967c) >= 0x0 (signed, 32-bit)` | `ret(0x1967c) < 0x0 (signed, 32-bit)` |
| `beq` at 0xbf0bc | `ret(0x192e0) == 0x1` | `ret(0x192e0) != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbf120 | `r0` | `?` |

## `flash_set_default_image`

- `flash_set_default_image`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbf154 to 0xbf408.
  Established. Word writes to `[0x29c3b4] +0x0` and `[0x29c3b4] +0x4` have unknown data (`str` at 0xbf180, `str` at 0xbf1ac).
  Included byte stores put `0x41` and `0x42` at `sp -0x19` on separate paths (`strb` at 0xbf250, `strb` at 0xbf270).
  The unnamed `0x192e0` call carries `ret(0x1967c)`, `sp-25`, `0x1`, `sp-25` (`bl` at 0xbf320).
  Default image flag semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbf164 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xbf168 | load | 4 | `abs 0xbf3d8` | `-` |
| `ldr` at 0xbf16c | load | 4 | `abs 0x29c3b4` | `-` |
| `ldr` at 0xbf170 | load | 4 | `abs 0xbf3d8` | `-` |
| `ldr` at 0xbf174 | load | 4 | `abs 0x29c3b4` | `-` |
| `ldr` at 0xbf178 | load | 4 | `[0x29c3b4] +0x0` | `-` |
| `str` at 0xbf180 | store | 4 | `[0x29c3b4] +0x0` | `?` |
| `ldr` at 0xbf184 | load | 4 | `abs 0xbf3d8` | `-` |
| `ldr` at 0xbf18c | load | 4 | `abs 0x29c3b4` | `-` |
| `ldr` at 0xbf194 | load | 4 | `abs 0xbf3d8` | `-` |
| `ldr` at 0xbf19c | load | 4 | `abs 0x29c3b4` | `-` |
| `ldr` at 0xbf1a4 | load | 4 | `[0x29c3b4] +0x4` | `-` |
| `str` at 0xbf1ac | store | 4 | `[0x29c3b4] +0x4` | `?` |
| `ldr` at 0xbf1b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbf1bc | load | 4 | `abs 0xbf3dc` | `-` |
| `ldr` at 0xbf1d8 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xbf1ec | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xbf1f8 | load | 4 | `abs 0xbf3e0` | `-` |
| `str` at 0xbf1fc | store | 4 | `sp -0x2c` | `0x11f9a8` |
| `ldr` at 0xbf200 | load | 4 | `abs 0xbf3dc` | `-` |
| `str` at 0xbf204 | store | 4 | `sp -0x28` | `0x11f964` |
| `str` at 0xbf208 | store | 4 | `sp -0x24` | `ret(0x1f8e0)` |
| `ldr` at 0xbf210 | load | 4 | `abs 0xbf3e4` | `-` |
| `ldr` at 0xbf214 | load | 4 | `abs 0xbf3e8` | `-` |
| `str` at 0xbf224 | store | 4 | `sp -0x20` | `?` |
| `str` at 0xbf230 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xbf238 | load | 4 | `abs 0xbf3d8` | `-` |
| `ldr` at 0xbf23c | load | 4 | `abs 0x29c3a0` | `-` |
| `ldr` at 0xbf240 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xbf250 | store | 1 | `sp -0x19` | `0x41` |
| `ldr` at 0xbf258 | load | 4 | `abs 0xbf3d8` | `-` |
| `ldr` at 0xbf25c | load | 4 | `abs 0x29c3a4` | `-` |
| `ldr` at 0xbf260 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xbf270 | store | 1 | `sp -0x19` | `0x42` |
| `ldr` at 0xbf278 | load | 4 | `abs 0xbf3ec` | `-` |
| `str` at 0xbf27c | store | 4 | `sp -0x2c` | `0x11f9d8` |
| `ldr` at 0xbf280 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbf284 | store | 4 | `sp -0x28` | `[sp-20]` |
| `ldr` at 0xbf28c | load | 4 | `abs 0xbf3e4` | `-` |
| `ldr` at 0xbf290 | load | 4 | `abs 0xbf3f0` | `-` |
| `str` at 0xbf2a0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xbf2a8 | load | 4 | `abs 0xbf3dc` | `-` |
| `ldr` at 0xbf2ac | load | 4 | `abs 0xbf3f4` | `-` |
| `str` at 0xbf2b8 | store | 4 | `sp -0x18` | `ret(0x1967c)` |
| `ldr` at 0xbf2bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbf2d0 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xbf2dc | load | 4 | `abs 0xbf3f8` | `-` |
| `str` at 0xbf2e0 | store | 4 | `sp -0x2c` | `0x11f938` |
| `ldr` at 0xbf2e4 | load | 4 | `abs 0xbf3dc` | `-` |
| `str` at 0xbf2e8 | store | 4 | `sp -0x28` | `0x11f964` |
| `str` at 0xbf2ec | store | 4 | `sp -0x24` | `ret(0x1f8e0)` |
| `ldr` at 0xbf2f4 | load | 4 | `abs 0xbf3e4` | `-` |
| `str` at 0xbf308 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xbf314 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbf338 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xbf344 | load | 4 | `abs 0xbf3fc` | `-` |
| `str` at 0xbf348 | store | 4 | `sp -0x2c` | `0x11fa04` |
| `ldr` at 0xbf34c | load | 4 | `abs 0xbf3dc` | `-` |
| `str` at 0xbf350 | store | 4 | `sp -0x28` | `0x11f964` |
| `str` at 0xbf354 | store | 4 | `sp -0x24` | `ret(0x1f8e0)` |
| `ldr` at 0xbf35c | load | 4 | `abs 0xbf3e4` | `-` |
| `ldr` at 0xbf360 | load | 4 | `abs 0xbf400` | `-` |
| `str` at 0xbf370 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xbf378 | load | 4 | `abs 0xbf3d8` | `-` |
| `ldrb` at 0xbf37c | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xbf380 | load | 4 | `abs 0xbf404` | `-` |
| `ldr` at 0xbf388 | load | 4 | `abs 0x29c3b4` | `-` |
| `ldr` at 0xbf394 | load | 4 | `abs 0xbf3d8` | `-` |
| `ldrb` at 0xbf398 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xbf39c | load | 4 | `abs 0xbf404` | `-` |
| `ldr` at 0xbf3a4 | load | 4 | `abs 0x29c3b4` | `-` |
| `ldr` at 0xbf3b0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbf3b8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xbf3bc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbf3c8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xbf3cc | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xbf3d4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbf3d4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbf3d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbf1c0 | `(unknown callee) 0x19358` | `0x11f964` | `[0x29c3b4]+4` | `0x4` | `[sp-20]` |
| `bl` at 0xbf1d0 | `(unknown callee) 0x193d0` | `ret(0x19358)` | `?` | `?` | `ret(0x19358)` |
| `bl` at 0xbf1e4 | `(unknown callee) 0x193d0` | `ret(0x193d0)` | `?` | `?` | `[ret(0x193d0)]` |
| `bl` at 0xbf1f0 | `(unknown callee) 0x1f8e0` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |
| `bl` at 0xbf21c | `(unknown callee) 0x200a0` | `0x3` | `0x11f5c0` | `0x215` | `0x0` |
| `bl` at 0xbf298 | `(unknown callee) 0x200a0` | `0x3` | `0x11f5c0` | `0x21f` | `0x0` |
| `bl` at 0xbf2b0 | `(unknown callee) 0x1967c` | `0x11f964` | `0x241` | `?` | `?` |
| `bl` at 0xbf2c8 | `(unknown callee) 0x193d0` | `ret(0x1967c)` | `?` | `?` | `ret(0x1967c)` |
| `bl` at 0xbf2d4 | `(unknown callee) 0x1f8e0` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |
| `bl` at 0xbf300 | `(unknown callee) 0x200a0` | `0x3` | `0x11f5c0` | `0x224` | `0x0` |
| `bl` at 0xbf320 | `(unknown callee) 0x192e0` | `ret(0x1967c)` | `sp-25` | `0x1` | `sp-25` |
| `bl` at 0xbf330 | `(unknown callee) 0x193d0` | `ret(0x192e0)` | `?` | `?` | `ret(0x192e0)` |
| `bl` at 0xbf33c | `(unknown callee) 0x1f8e0` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |
| `bl` at 0xbf368 | `(unknown callee) 0x200a0` | `0x3` | `0x11f5c0` | `0x22a` | `0x0` |
| `bl` at 0xbf3c0 | `(unknown callee) 0x1931c` | `[sp-24]` | `0x29c398` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xbf1b8 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bge` at 0xbf1cc | `ret(0x19358) >= 0x0 (signed, 32-bit)` | `ret(0x19358) < 0x0 (signed, 32-bit)` |
| `beq` at 0xbf1e0 | `[ret(0x193d0)] == 0x2` | `[ret(0x193d0)] != 0x2` |
| `bne` at 0xbf248 | `[0x29c3a0] != [sp-20]` | `[0x29c3a0] == [sp-20]` |
| `bne` at 0xbf268 | `[0x29c3a4] != [sp-20]` | `[0x29c3a4] == [sp-20]` |
| `bge` at 0xbf2c4 | `ret(0x1967c) >= 0x0 (signed, 32-bit)` | `ret(0x1967c) < 0x0 (signed, 32-bit)` |
| `beq` at 0xbf32c | `ret(0x192e0) == 0x1` | `ret(0x192e0) != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbf3d4 | `r0` | `?` |

## `flash_get_default_image`

- `flash_get_default_image`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbf408 to 0xbf638.
  Established. Included word stores write unknown values at `[0x29c3b4] +0x0` and `[0x29c3b4] +0x4` (`str` at 0xbf430, `str` at 0xbf45c).
  The unnamed `0x197fc` call carries `ret(0x1967c)`, `sp-25`, `0x1`, `sp-25` (`bl` at 0xbf4f8).
  Equality `ret(0x197fc) == 0x1` takes `0xbf550`, while inequality falls through (`beq` at 0xbf504).
  Default selection readback interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xbf418 | load | 4 | `abs 0xbf618` | `-` |
| `ldr` at 0xbf41c | load | 4 | `abs 0x29c3b4` | `-` |
| `ldr` at 0xbf420 | load | 4 | `abs 0xbf618` | `-` |
| `ldr` at 0xbf424 | load | 4 | `abs 0x29c3b4` | `-` |
| `ldr` at 0xbf428 | load | 4 | `[0x29c3b4] +0x0` | `-` |
| `str` at 0xbf430 | store | 4 | `[0x29c3b4] +0x0` | `?` |
| `ldr` at 0xbf434 | load | 4 | `abs 0xbf618` | `-` |
| `ldr` at 0xbf43c | load | 4 | `abs 0x29c3b4` | `-` |
| `ldr` at 0xbf444 | load | 4 | `abs 0xbf618` | `-` |
| `ldr` at 0xbf44c | load | 4 | `abs 0x29c3b4` | `-` |
| `ldr` at 0xbf454 | load | 4 | `[0x29c3b4] +0x4` | `-` |
| `str` at 0xbf45c | store | 4 | `[0x29c3b4] +0x4` | `?` |
| `ldr` at 0xbf460 | load | 4 | `abs 0xbf61c` | `-` |
| `str` at 0xbf470 | store | 4 | `sp -0x14` | `ret(0x1967c)` |
| `ldr` at 0xbf474 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbf488 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `str` at 0xbf498 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xbf4a8 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xbf4b4 | load | 4 | `abs 0xbf620` | `-` |
| `str` at 0xbf4b8 | store | 4 | `sp -0x2c` | `0x11fa30` |
| `ldr` at 0xbf4bc | load | 4 | `abs 0xbf61c` | `-` |
| `str` at 0xbf4c0 | store | 4 | `sp -0x28` | `0x11f964` |
| `str` at 0xbf4c4 | store | 4 | `sp -0x24` | `ret(0x1f8e0)` |
| `ldr` at 0xbf4cc | load | 4 | `abs 0xbf624` | `-` |
| `str` at 0xbf4e0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xbf4ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbf510 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xbf51c | load | 4 | `abs 0xbf628` | `-` |
| `str` at 0xbf520 | store | 4 | `sp -0x2c` | `0x11fa60` |
| `ldr` at 0xbf524 | load | 4 | `abs 0xbf61c` | `-` |
| `str` at 0xbf528 | store | 4 | `sp -0x28` | `0x11f964` |
| `str` at 0xbf52c | store | 4 | `sp -0x24` | `ret(0x1f8e0)` |
| `ldr` at 0xbf534 | load | 4 | `abs 0xbf624` | `-` |
| `ldr` at 0xbf538 | load | 4 | `abs 0xbf62c` | `-` |
| `str` at 0xbf548 | store | 4 | `sp -0x20` | `?` |
| `ldrb` at 0xbf550 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xbf55c | load | 4 | `abs 0xbf618` | `-` |
| `ldr` at 0xbf560 | load | 4 | `abs 0x29c3a0` | `-` |
| `str` at 0xbf564 | store | 4 | `sp -0x18` | `[0x29c3a0]` |
| `ldrb` at 0xbf56c | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xbf578 | load | 4 | `abs 0xbf618` | `-` |
| `ldr` at 0xbf57c | load | 4 | `abs 0x29c3a4` | `-` |
| `str` at 0xbf580 | store | 4 | `sp -0x18` | `[0x29c3a4]` |
| `ldr` at 0xbf588 | load | 4 | `abs 0xbf630` | `-` |
| `str` at 0xbf58c | store | 4 | `sp -0x2c` | `0x11fa94` |
| `ldrb` at 0xbf590 | load | 1 | `sp -0x19` | `-` |
| `str` at 0xbf594 | store | 4 | `sp -0x28` | `[sp-25]:1` |
| `ldr` at 0xbf59c | load | 4 | `abs 0xbf624` | `-` |
| `str` at 0xbf5b0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xbf5b8 | load | 4 | `abs 0xbf618` | `-` |
| `ldrb` at 0xbf5bc | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xbf5c0 | load | 4 | `abs 0xbf634` | `-` |
| `ldr` at 0xbf5c8 | load | 4 | `abs 0x29c3b4` | `-` |
| `ldr` at 0xbf5d4 | load | 4 | `abs 0xbf618` | `-` |
| `ldrb` at 0xbf5d8 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xbf5dc | load | 4 | `abs 0xbf634` | `-` |
| `ldr` at 0xbf5e4 | load | 4 | `abs 0x29c3b4` | `-` |
| `ldr` at 0xbf5f0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbf5f8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xbf5fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbf604 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbf608 | store | 4 | `sp -0x20` | `[sp-24]` |
| `ldr` at 0xbf60c | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xbf614 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbf614 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbf614 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbf468 | `(unknown callee) 0x1967c` | `0x11f964` | `0x0` | `0x4` | `?` |
| `bl` at 0xbf480 | `(unknown callee) 0x193d0` | `ret(0x1967c)` | `?` | `?` | `ret(0x1967c)` |
| `bl` at 0xbf4a0 | `(unknown callee) 0x193d0` | `ret(0x193d0)` | `?` | `?` | `[ret(0x193d0)]` |
| `bl` at 0xbf4ac | `(unknown callee) 0x1f8e0` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |
| `bl` at 0xbf4d8 | `(unknown callee) 0x200a0` | `0x3` | `0x11f5c0` | `0x248` | `0x0` |
| `bl` at 0xbf4f8 | `(unknown callee) 0x197fc` | `ret(0x1967c)` | `sp-25` | `0x1` | `sp-25` |
| `bl` at 0xbf508 | `(unknown callee) 0x193d0` | `ret(0x197fc)` | `?` | `?` | `ret(0x197fc)` |
| `bl` at 0xbf514 | `(unknown callee) 0x1f8e0` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |
| `bl` at 0xbf540 | `(unknown callee) 0x200a0` | `0x3` | `0x11f5c0` | `0x24e` | `0x0` |
| `bl` at 0xbf5a8 | `(unknown callee) 0x200a0` | `0x3` | `0x11f5c0` | `0x258` | `0x0` |
| `bl` at 0xbf600 | `(unknown callee) 0x1931c` | `[sp-20]` | `0x29c398` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xbf47c | `ret(0x1967c) >= 0x0 (signed, 32-bit)` | `ret(0x1967c) < 0x0 (signed, 32-bit)` |
| `bne` at 0xbf490 | `[ret(0x193d0)] != 0x2` | `[ret(0x193d0)] == 0x2` |
| `beq` at 0xbf504 | `ret(0x197fc) == 0x1` | `ret(0x197fc) != 0x1` |
| `bne` at 0xbf558 | `[sp-25]:1 != 0x41` | `[sp-25]:1 == 0x41` |
| `bne` at 0xbf574 | `[sp-25]:1 != 0x42` | `[sp-25]:1 == 0x42` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbf614 | `r0` | `?` |

## `FlashValidateImage`

- `FlashValidateImage`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9802c to 0x984b4.
  Established. A byte assignment clears absolute `0x133a38` (`strb` at 0x98064).
  Included word stores place `0x133a38` at `[sp] +0x0` (`str` at 0x98424, `str` at 0x98468).
  Unsigned `[sp-52] <= 0x1f` takes `0x98164`, while greater values fall through (`bls` at 0x9813c).
  Image validity criteria remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9803c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x98040 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x98044 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x98048 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x9804c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x98050 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x98058 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x9805c | load | 4 | `abs 0x98480` | `-` |
| `strb` at 0x98064 | store | 1 | `abs 0x133a38` | `0x0` |
| `ldr` at 0x98068 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x98074 | load | 4 | `abs 0x98480` | `-` |
| `ldr` at 0x98078 | load | 4 | `abs 0x98484` | `-` |
| `ldr` at 0x9807c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x98088 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9808c | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0x98098 | store | 4 | `sp -0x2c` | `ret(0x19280)` |
| `ldr` at 0x9809c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x980a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x980ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x980b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x980b8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x980c8 | load | 4 | `abs 0x98480` | `-` |
| `ldr` at 0x980cc | load | 4 | `abs 0x98488` | `-` |
| `ldr` at 0x980d0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x980dc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x980f0 | load | 4 | `abs 0x98480` | `-` |
| `ldr` at 0x980f4 | load | 4 | `abs 0x9848c` | `-` |
| `ldr` at 0x980f8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x98104 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x98108 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x98110 | store | 4 | `sp -0x3c` | `([sp-44] + [sp-36])` |
| `ldr` at 0x98114 | load | 4 | `sp -0x3c` | `-` |
| `ldrh` at 0x98118 | load | 2 | `([sp-44] + [sp-36]) +0x2` | `-` |
| `str` at 0x98130 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x98134 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x98140 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x98144 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x9814c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x98150 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x98164 | load | 4 | `abs 0x98480` | `-` |
| `ldr` at 0x98168 | load | 4 | `abs 0x98490` | `-` |
| `ldr` at 0x9816c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x98178 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9817c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x98198 | load | 4 | `abs 0x98480` | `-` |
| `ldr` at 0x9819c | load | 4 | `abs 0x98494` | `-` |
| `ldr` at 0x981a0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x981b0 | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0x981b8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x981bc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x981c0 | store | 4 | `sp -0x30` | `[sp-60]` |
| `ldr` at 0x981c4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x981c8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x981d4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x981d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x981e8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x981f0 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x19280))` |
| `ldr` at 0x981f4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x981fc | store | 4 | `sp -0x38` | `[sp-56]+4` |
| `ldr` at 0x98200 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x98208 | store | 4 | `sp -0x30` | `[sp-48]+4` |
| `ldr` at 0x98210 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9821c | load | 4 | `abs 0x98480` | `-` |
| `ldr` at 0x98220 | load | 4 | `abs 0x98498` | `-` |
| `ldr` at 0x9822c | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x98238 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x9823c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x98240 | store | 4 | `[sp+4] +0x0` | `[sp-60]` |
| `ldr` at 0x98244 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x98248 | load | 4 | `[sp-60] +0x18` | `-` |
| `str` at 0x98254 | store | 4 | `sp -0x2c` | `ret(0x19280)` |
| `ldr` at 0x98258 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x98264 | load | 4 | `abs 0x98480` | `-` |
| `ldr` at 0x98268 | load | 4 | `abs 0x9849c` | `-` |
| `ldr` at 0x98274 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x98278 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x98280 | store | 4 | `sp -0x40` | `(ret(0x19280) + [sp-36])` |
| `ldr` at 0x98284 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x98288 | store | 4 | `sp -0x30` | `[sp-20]` |
| `str` at 0x98290 | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0x98298 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x9829c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x982a0 | load | 4 | `[sp-60] +0xc` | `-` |
| `ldr` at 0x982b0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x982bc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x982c0 | load | 4 | `[sp-48] +0x0` | `-` |
| `ldr` at 0x982d0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x982d8 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x19280))` |
| `ldr` at 0x982dc | load | 4 | `sp -0x38` | `-` |
| `str` at 0x982e4 | store | 4 | `sp -0x38` | `[sp-56]+4` |
| `ldr` at 0x982e8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x982f0 | store | 4 | `sp -0x30` | `[sp-48]+4` |
| `ldr` at 0x982f8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x982fc | load | 4 | `[sp-60] +0x20` | `-` |
| `ldr` at 0x9830c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x98318 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x9831c | load | 4 | `[sp-60] +0x20` | `-` |
| `ldr` at 0x98330 | load | 4 | `abs 0x98480` | `-` |
| `ldr` at 0x98334 | load | 4 | `abs 0x984a0` | `-` |
| `str` at 0x98340 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x98348 | load | 4 | `abs 0x98480` | `-` |
| `ldr` at 0x9834c | load | 4 | `abs 0x984a4` | `-` |
| `ldr` at 0x98358 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x98364 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x98368 | load | 4 | `[sp-64] +0x4` | `-` |
| `str` at 0x98374 | store | 4 | `sp -0x34` | `ret(0x19280)` |
| `str` at 0x9837c | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0x98384 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x98388 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x9838c | store | 4 | `sp -0x30` | `[sp-64]` |
| `ldr` at 0x98390 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x98394 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x983a0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x983a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x983b4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x983bc | store | 4 | `sp -0x28` | `([sp-40] + ret(0x19280))` |
| `ldr` at 0x983c0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x983c8 | store | 4 | `sp -0x38` | `[sp-56]+4` |
| `ldr` at 0x983cc | load | 4 | `sp -0x30` | `-` |
| `str` at 0x983d4 | store | 4 | `sp -0x30` | `[sp-48]+4` |
| `ldr` at 0x983dc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x983e8 | load | 4 | `abs 0x98480` | `-` |
| `ldr` at 0x983ec | load | 4 | `abs 0x984a8` | `-` |
| `ldr` at 0x983f8 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x98404 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x98408 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x9840c | store | 4 | `[sp+8] +0x0` | `[sp-64]` |
| `ldr` at 0x98410 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x9841c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x98420 | load | 4 | `abs 0x98480` | `-` |
| `str` at 0x98424 | store | 4 | `[sp] +0x0` | `0x133a38` |
| `str` at 0x9842c | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x98434 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x98440 | load | 4 | `abs 0x984ac` | `-` |
| `ldr` at 0x98444 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x98448 | load | 4 | `abs 0x984b0` | `-` |
| `ldr` at 0x9844c | load | 4 | `abs 0x98480` | `-` |
| `ldr` at 0x98454 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x98460 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x98464 | load | 4 | `abs 0x98480` | `-` |
| `str` at 0x98468 | store | 4 | `[sp] +0x0` | `0x133a38` |
| `str` at 0x98470 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x98474 | load | 4 | `sp -0x44` | `-` |
| `ldm` at 0x9847c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9847c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9847c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x98080 | `(unknown callee) 0x197a8` | `0x133a38` | `0x111dfc` | `[sp-20]` | `[sp-20]` |
| `bl` at 0x98090 | `(unknown callee) 0x19280` | `[[sp-20]]` | `r1` | `0x133a38` | `[sp-20]` |
| `bl` at 0x980d4 | `(unknown callee) 0x197a8` | `0x133a38` | `0x111e1c` | `ret(0x19280)` | `?` |
| `bl` at 0x980e0 | `(unknown callee) 0xbe850` | `ret(0x19280)` | `?` | `([sp-24] + [sp-28])` | `ret(0x19280)` |
| `bl` at 0x980fc | `(unknown callee) 0x197a8` | `0x133a38` | `0x111e3c` | `[sp-44]` | `ret(0xbe850)` |
| `bl` at 0x98120 | `(unknown callee) 0x19514` | `[([sp-44] + [sp-36])+2]:2` | `?` | `[sp-44]` | `[([sp-44] + [sp-36])+2]:2` |
| `bl` at 0x98170 | `(unknown callee) 0x197a8` | `0x133a38` | `0x111e78` | `[sp-52]` | `?` |
| `bl` at 0x98188 | `(unknown callee) 0xbe850` | `([sp-44] + [sp-52])` | `([sp-44] + [sp-52])` | `[sp-44]` | `([sp-44] + [sp-52])` |
| `bl` at 0x981a4 | `(unknown callee) 0x197a8` | `0x133a38` | `0x111e9c` | `[sp-52]` | `ret(0xbe850)` |
| `bl` at 0x981dc | `(unknown callee) 0x19280` | `?` | `?` | `?` | `?` |
| `bl` at 0x98224 | `(unknown callee) 0x19808` | `0x133a38` | `0x111ee4` | `?` | `?` |
| `bl` at 0x9824c | `(unknown callee) 0x19280` | `[[sp-60]+24]` | `?` | `?` | `[sp-60]` |
| `bl` at 0x9826c | `(unknown callee) 0x19808` | `0x133a38` | `0x111f00` | `?` | `ret(0x19280)` |
| `bl` at 0x982a4 | `(unknown callee) 0x19280` | `[[sp-60]+12]` | `?` | `ret(0x19280)` | `[sp-60]` |
| `bl` at 0x982c4 | `(unknown callee) 0x19280` | `[[sp-48]]` | `?` | `ret(0x19280)` | `[sp-48]` |
| `bl` at 0x98300 | `(unknown callee) 0x19280` | `[[sp-60]+32]` | `?` | `ret(0x19280)` | `[sp-60]` |
| `bl` at 0x98320 | `(unknown callee) 0x19280` | `[[sp-60]+32]` | `?` | `ret(0x19280)` | `[sp-60]` |
| `bl` at 0x98338 | `(unknown callee) 0x19808` | `0x133a38` | `0x111f1c` | `?` | `ret(0x19280)` |
| `bl` at 0x98350 | `(unknown callee) 0x19808` | `0x133a38` | `0x111f30` | `?` | `ret(0x19280)` |
| `bl` at 0x9836c | `(unknown callee) 0x19280` | `[[sp-64]+4]` | `?` | `?` | `[sp-64]` |
| `bl` at 0x983a8 | `(unknown callee) 0x19280` | `?` | `?` | `?` | `?` |
| `bl` at 0x983f0 | `(unknown callee) 0x19808` | `0x133a38` | `0x111f44` | `?` | `?` |
| `bl` at 0x98450 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0x111f64` | `0x133a38` | `0x12eeb8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x98070 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0x980a8 | `ret(0x19280) < [sp-24] (unsigned, 32-bit)` | `ret(0x19280) >= [sp-24] (unsigned, 32-bit)` |
| `blo` at 0x980c0 | `([sp-24] + [sp-28]) < ret(0x19280) (unsigned, 32-bit)` | `([sp-24] + [sp-28]) >= ret(0x19280) (unsigned, 32-bit)` |
| `bne` at 0x980ec | `Z == 0` | `not (Z == 0)` |
| `bls` at 0x9813c | `[sp-52] <= 0x1f (unsigned, 32-bit)` | `[sp-52] > 0x1f (unsigned, 32-bit)` |
| `bhi` at 0x9815c | `([sp-44] + [sp-52]) > ([sp-24] + [sp-28]) (unsigned, 32-bit)` | `([sp-44] + [sp-52]) <= ([sp-24] + [sp-28]) (unsigned, 32-bit)` |
| `bne` at 0x98194 | `Z == 0` | `not (Z == 0)` |
| `bhs` at 0x981d0 | `C == 1` | `not (C == 1)` |
| `beq` at 0x98218 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x98234 | `[sp+4] == 0x0` | `[sp+4] != 0x0` |
| `bne` at 0x98260 | `ret(0x19280) != 0x0` | `ret(0x19280) == 0x0` |
| `bhs` at 0x982b8 | `[sp-56] >= ret(0x19280) (unsigned, 32-bit)` | `[sp-56] < ret(0x19280) (unsigned, 32-bit)` |
| `beq` at 0x98314 | `ret(0x19280) == [sp-40]` | `ret(0x19280) != [sp-40]` |
| `bne` at 0x9832c | `ret(0x19280) != 0x0` | `ret(0x19280) == 0x0` |
| `beq` at 0x98360 | `[sp-64] == 0x0` | `[sp-64] != 0x0` |
| `bhs` at 0x9839c | `C == 1` | `not (C == 1)` |
| `beq` at 0x983e4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x98400 | `[sp+8] == 0x0` | `[sp+8] != 0x0` |
| `beq` at 0x98418 | `[sp] == 0x0` | `[sp] != 0x0` |
| `beq` at 0x9843c | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `beq` at 0x9845c | `[sp] == 0x0` | `[sp] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9847c | `r0` | `[sp-68]` |

## `FlashTask`

- `FlashTask`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x98648 to 0x9871c.
  Established. An included absolute word store writes `[0x133a34]-1` to `0x133a34` (`str` at 0x986ac).
  The unnamed `0xa8b38` call carries `sp-32`, `0x29a8ac`, unknown, `sp-32` (`bl` at 0x98670);
  its zero result takes `0x9868c`, while nonzero falls through (`beq` at 0x98684).
  Flash task message ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x98658 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9865c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x98668 | load | 4 | `abs 0x98708` | `-` |
| `str` at 0x98678 | store | 4 | `sp -0x1c` | `ret(0xa8b38)` |
| `ldr` at 0x9867c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9868c | load | 4 | `abs 0x9870c` | `-` |
| `ldr` at 0x98690 | load | 4 | `abs 0x133a34` | `-` |
| `ldr` at 0x9869c | load | 4 | `abs 0x9870c` | `-` |
| `ldr` at 0x986a0 | load | 4 | `abs 0x9870c` | `-` |
| `ldr` at 0x986a4 | load | 4 | `abs 0x133a34` | `-` |
| `str` at 0x986ac | store | 4 | `abs 0x133a34` | `[0x133a34]-1` |
| `ldr` at 0x986b4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x986b8 | load | 1 | `[sp-32] +0x16` | `-` |
| `ldr` at 0x986c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x986d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x986dc | load | 4 | `abs 0x98710` | `-` |
| `str` at 0x986e0 | store | 4 | `sp -0x28` | `0x112004` |
| `str` at 0x986e4 | store | 4 | `sp -0x24` | `ret(0x1f8e0)` |
| `ldr` at 0x986ec | load | 4 | `abs 0x98714` | `-` |
| `ldr` at 0x986f0 | load | 4 | `abs 0x98718` | `-` |
| `ldr` at 0x986fc | load | 4 | `sp -0x20` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x98670 | `(unknown callee) 0xa8b38` | `sp-32` | `0x29a8ac` | `?` | `sp-32` |
| `bl` at 0x986c8 | `(unknown callee) 0x98744` | `[sp-32]` | `?` | `?` | `[[sp-32]+22]:1` |
| `bl` at 0x986d4 | `(unknown callee) 0x1f8e0` | `ret(0xa8b38)` | `?` | `?` | `ret(0xa8b38)` |
| `bl` at 0x986f8 | `(unknown callee) 0x200a0` | `0x3` | `0x111f90` | `0xc13` | `0x0` |
| `bl` at 0x98700 | `(unknown callee) 0xa8b98` | `[sp-32]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x98684 | `ret(0xa8b38) == 0x0` | `ret(0xa8b38) != 0x0` |
| `beq` at 0x98698 | `[0x133a34] == 0x0` | `[0x133a34] != 0x0` |
| `bne` at 0x986c0 | `[[sp-32]+22]:1 != 0x0` | `[[sp-32]+22]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
