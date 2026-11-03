# ARM PCI-X, board and node setup in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package4/pcix-board-node-setup-facts.txt) retain all tool sections.

## `pcixInit`

- `pcixInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4f384 to 0x4f81c.
  Established. Included word stores write `ret(0x196d0)` to `0x13249c` and `[sp-28]` to `[sp-48] +0x0` (`str` at 0x4f440, `str` at 0x4f4a0).
  An indirect call with unknown target carries `[sp-24]` followed by three unknown candidates (`mov` at 0x4f714).
  PCI-X initialization callback responsibilities remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4f394 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x4f398 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x4f39c | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x4f3a0 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x4f3a4 | load | 4 | `abs 0x4f7f4` | `-` |
| `ldr` at 0x4f3a8 | load | 4 | `abs 0x13249c` | `-` |
| `ldr` at 0x4f3b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4f3c0 | load | 4 | `abs 0x4f7f8` | `-` |
| `str` at 0x4f3c4 | store | 4 | `sp -0x48` | `0xf8a70` |
| `ldr` at 0x4f3c8 | load | 4 | `abs 0x4f7f4` | `-` |
| `ldr` at 0x4f3cc | load | 4 | `abs 0x13249c` | `-` |
| `str` at 0x4f3d0 | store | 4 | `sp -0x44` | `[0x13249c]` |
| `ldr` at 0x4f3d4 | load | 4 | `abs 0x4f7fc` | `-` |
| `ldr` at 0x4f3d8 | load | 4 | `abs 0x1324a0` | `-` |
| `str` at 0x4f3dc | store | 4 | `sp -0x40` | `[0x1324a0]` |
| `ldr` at 0x4f3e4 | load | 4 | `abs 0x4f800` | `-` |
| `ldr` at 0x4f3f4 | load | 4 | `abs 0x4f7fc` | `-` |
| `ldr` at 0x4f3f8 | load | 4 | `abs 0x1324a0` | `-` |
| `ldr` at 0x4f404 | load | 4 | `abs 0x4f7fc` | `-` |
| `str` at 0x4f40c | store | 4 | `abs 0x1324a0` | `0x1` |
| `ldr` at 0x4f410 | load | 4 | `abs 0x4f7fc` | `-` |
| `ldr` at 0x4f414 | load | 4 | `abs 0x1324a0` | `-` |
| `str` at 0x4f428 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x4f42c | load | 4 | `abs 0x4f7f4` | `-` |
| `ldr` at 0x4f430 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x4f440 | store | 4 | `abs 0x13249c` | `ret(0x196d0)` |
| `ldr` at 0x4f444 | load | 4 | `abs 0x13249c` | `-` |
| `ldr` at 0x4f450 | load | 4 | `abs 0x4f804` | `-` |
| `str` at 0x4f454 | store | 4 | `sp -0x48` | `0xf8ab4` |
| `ldr` at 0x4f458 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x4f45c | store | 4 | `sp -0x44` | `[sp-52]` |
| `ldr` at 0x4f464 | load | 4 | `abs 0x4f800` | `-` |
| `ldr` at 0x4f474 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4f488 | load | 4 | `abs 0x4f7f4` | `-` |
| `ldr` at 0x4f48c | load | 4 | `abs 0x13249c` | `-` |
| `str` at 0x4f494 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x4f498 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x4f49c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4f4a0 | store | 4 | `[sp-48] +0x0` | `[sp-28]` |
| `ldr` at 0x4f4a4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x4f4a8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4f4ac | store | 4 | `[sp-48] +0x4` | `[sp-32]` |
| `ldr` at 0x4f4b0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x4f4b4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x4f4b8 | store | 4 | `[sp-48] +0x8` | `[sp-36]` |
| `ldr` at 0x4f4bc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x4f4c0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x4f4c4 | store | 4 | `[sp-48] +0xc` | `[sp]` |
| `ldr` at 0x4f4c8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x4f4cc | load | 4 | `sp +0x4` | `-` |
| `str` at 0x4f4d0 | store | 4 | `[sp-48] +0x10` | `[sp+4]` |
| `ldr` at 0x4f4d4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x4f4d8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x4f4dc | store | 4 | `[sp-48] +0x18` | `[sp+8]` |
| `ldr` at 0x4f4e0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x4f4e4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x4f4e8 | load | 4 | `[sp-48] +0x8` | `-` |
| `str` at 0x4f4f8 | store | 4 | `[sp-48] +0x14` | `ret(0x196d0)` |
| `ldr` at 0x4f4fc | load | 4 | `[sp-48] +0x14` | `-` |
| `ldr` at 0x4f508 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4f510 | load | 4 | `abs 0x4f808` | `-` |
| `ldr` at 0x4f514 | load | 4 | `abs 0x4f80c` | `-` |
| `str` at 0x4f550 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x4f554 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4f55c | load | 4 | `abs 0x4f808` | `-` |
| `ldr` at 0x4f560 | load | 4 | `abs 0x4f80c` | `-` |
| `ldr` at 0x4f56c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x4f57c | load | 4 | `abs 0x4f810` | `-` |
| `str` at 0x4f580 | store | 4 | `(unknown)` | `0xf8af8` |
| `str` at 0x4f584 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0x4f588 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x4f58c | load | 4 | `[sp-48] +0x8` | `-` |
| `str` at 0x4f590 | store | 4 | `(unknown)` | `[[sp-48]+8]` |
| `ldr` at 0x4f598 | load | 4 | `abs 0x4f800` | `-` |
| `ldr` at 0x4f5a8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x4f5ac | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x4f5b0 | load | 4 | `[sp-48] +0x14` | `-` |
| `ldr` at 0x4f5b8 | load | 4 | `[sp-48] +0x8` | `-` |
| `str` at 0x4f5c4 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x4f5c8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x4f5cc | load | 4 | `[sp-48] +0x8` | `-` |
| `ldr` at 0x4f5d0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4f5dc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x4f5e0 | load | 4 | `[sp-48] +0x0` | `-` |
| `ldr` at 0x4f5f4 | load | 4 | `abs 0x4f814` | `-` |
| `str` at 0x4f5fc | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x4f600 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x4f604 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4f60c | load | 4 | `[sp-48] +0x14` | `-` |
| `ldr` at 0x4f614 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x4f618 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4f62c | load | 4 | `[sp-48] +0x4` | `-` |
| `ldrb` at 0x4f634 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x4f638 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x4f63c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x4f640 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4f648 | load | 4 | `[sp-48] +0x14` | `-` |
| `ldr` at 0x4f650 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x4f654 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4f668 | load | 4 | `[sp-48] +0x4` | `-` |
| `ldrb` at 0x4f670 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x4f674 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x4f678 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x4f67c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4f684 | load | 4 | `[sp-48] +0x14` | `-` |
| `strb` at 0x4f690 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x4f694 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x4f698 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x4f69c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4f6b4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x4f6b8 | load | 4 | `[sp-52] +0x8` | `-` |
| `ldr` at 0x4f6c4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x4f6c8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x4f6cc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4f6e0 | load | 4 | `[sp-48] +0x4` | `-` |
| `ldr` at 0x4f6e8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x4f6ec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4f6f4 | load | 4 | `[sp-48] +0x14` | `-` |
| `ldr` at 0x4f700 | load | 4 | `[sp-52] +0x8` | `-` |
| `ldr` at 0x4f704 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4f71c | store | 4 | `sp -0x28` | `ret(unknown)` |
| `ldr` at 0x4f720 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4f72c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4f734 | load | 4 | `abs 0x4f808` | `-` |
| `ldr` at 0x4f738 | load | 4 | `abs 0x4f80c` | `-` |
| `str` at 0x4f774 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x4f778 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4f780 | load | 4 | `abs 0x4f808` | `-` |
| `ldr` at 0x4f784 | load | 4 | `abs 0x4f80c` | `-` |
| `ldr` at 0x4f790 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x4f7a0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4f7ac | load | 4 | `abs 0x4f818` | `-` |
| `str` at 0x4f7b0 | store | 4 | `(unknown)` | `0xf8b34` |
| `str` at 0x4f7b4 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0x4f7b8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x4f7bc | store | 4 | `(unknown)` | `[sp-44]` |
| `str` at 0x4f7c0 | store | 4 | `(unknown)` | `ret(0x1fc1c)` |
| `ldr` at 0x4f7c8 | load | 4 | `abs 0x4f800` | `-` |
| `ldr` at 0x4f7d8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x4f7e0 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldm` at 0x4f7f0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x4f7f0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4f7f0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4f7f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4f3f0 | `(unknown callee) 0x200a0` | `0x9` | `0xf8aa0` | `0xa5` | `0x0` |
| `bl` at 0x4f438 | `(unknown callee) 0x196d0` | `[sp-52]` | `0x1` | `[0x1324a0]` | `?` |
| `bl` at 0x4f470 | `(unknown callee) 0x200a0` | `0x9` | `0xf8aa0` | `0xb1` | `0x0` |
| `bl` at 0x4f4f0 | `(unknown callee) 0x196d0` | `[[sp-48]+8]` | `0x8` | `[sp-48]` | `[sp-48]` |
| `bl` at 0x4f518 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0xf8aec` | `0xf8af4` |
| `bl` at 0x4f524 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x4f564 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0xf8aec` | `0xf8af4` |
| `bl` at 0x4f574 | `(unknown callee) 0x19808` | `[sp-52]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x4f5a4 | `(unknown callee) 0x200a0` | `0x9` | `0xf8aa0` | `0xc3` | `0x0` |
| `bl` at 0x4f5bc | `(unknown callee) 0x19748` | `[[sp-48]+20]` | `0x1` | `[[sp-48]+8]` | `[sp-48]` |
| `mov` at 0x4f6a4 | `(unknown callee) (unknown target)` | `[sp-24]` | `[sp-48]` | `?` | `[[sp-52]]` |
| `mov` at 0x4f714 | `(unknown callee) (unknown target)` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x4f73c | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0xf8aec` | `0xf8af4` |
| `bl` at 0x4f748 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x4f788 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0xf8aec` | `0xf8af4` |
| `bl` at 0x4f798 | `(unknown callee) 0x19808` | `[sp-56]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x4f7a4 | `(unknown callee) 0x1fc1c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x4f7d4 | `(unknown callee) 0x200a0` | `0x1` | `0xf8aa0` | `0xd0` | `0x0` |
| `bl` at 0x4f7e8 | `(unknown callee) 0x97fac` | `?` | `?` | `[[sp-48]+8]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4f3b0 | `[0x13249c] == 0x0` | `[0x13249c] != 0x0` |
| `bne` at 0x4f3bc | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0x4f400 | `[0x1324a0] != 0x0` | `[0x1324a0] == 0x0` |
| `bne` at 0x4f44c | `[0x13249c] != 0x0` | `[0x13249c] == 0x0` |
| `bne` at 0x4f504 | `[[sp-48]+20] != 0x0` | `[[sp-48]+20] == 0x0` |
| `ble` at 0x4f5d8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x4f6b0 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x4f6c0 | `[[sp-52]+8] == 0x0` | `[[sp-52]+8] != 0x0` |
| `beq` at 0x4f728 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4f7f0 | `r0` | `ret(0x97fac)` |

## `pcixUp`

- `pcixUp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4fa44 to 0x4fd48.
  Established. Included byte stores write `0x1` and `0x0` to unknown effective targets (`strb` at 0x4fb08, `strb` at 0x4fb60).
  A signed nonnegative `ret(0x51d50)` takes `0x4fcf8`, while negative falls through (`bge` at 0x4fccc).
  PCI-X activation sequencing beyond these calls remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4fa54 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x4fa5c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x4fa64 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0x4fa68 | load | 4 | `abs 0x4fd3c` | `-` |
| `ldr` at 0x4fa6c | load | 4 | `abs 0x1324a0` | `-` |
| `ldrb` at 0x4fa78 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x4fa88 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x4fa90 | load | 4 | `abs 0x4fd40` | `-` |
| `str` at 0x4fa94 | store | 4 | `sp -0x34` | `0xfffff703` |
| `ldr` at 0x4fa98 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x4fa9c | store | 4 | `sp -0x30` | `[sp-52]` |
| `ldr` at 0x4faa4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4fab8 | load | 4 | `abs 0x4fd44` | `-` |
| `ldr` at 0x4fabc | load | 4 | `abs 0x13249c` | `-` |
| `str` at 0x4fac4 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x4fac8 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x4fad8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x4fadc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4fae0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x4fae4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4faf0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4faf4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4fafc | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x4fb08 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x4fb0c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x4fb14 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x4fb1c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4fb28 | store | 4 | `sp -0x28` | `ret(0x508a4)` |
| `str` at 0x4fb30 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x4fb34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4fb38 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x4fb3c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4fb48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4fb4c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4fb54 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x4fb60 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x4fb64 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x4fb6c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x4fb74 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x4fb78 | store | 4 | `sp -0x30` | `?` |
| `ldrb` at 0x4fb80 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x4fb8c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4fb90 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x4fb98 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x4fba4 | store | 1 | `(unknown)` | `0x1` |
| `ldrb` at 0x4fba8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x4fbac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4fbbc | store | 4 | `sp -0x20` | `ret(0x53024)` |
| `ldr` at 0x4fbc0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4fbcc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4fbd0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x4fbd8 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x4fbe4 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x4fbe8 | load | 4 | `abs 0x4fd40` | `-` |
| `str` at 0x4fbec | store | 4 | `sp -0x30` | `0xfffff703` |
| `ldr` at 0x4fbf4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4fbf8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4fc04 | store | 4 | `sp -0x28` | `ret(0x51c34)` |
| `ldr` at 0x4fc08 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4fc0c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x4fc14 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x4fc20 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x4fc24 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x4fc28 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x4fc30 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4fc34 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x4fc3c | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x4fc48 | store | 1 | `(unknown)` | `0x1` |
| `ldrb` at 0x4fc4c | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x4fc50 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x4fc54 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4fc64 | store | 4 | `sp -0x24` | `ret(0x53100)` |
| `ldr` at 0x4fc68 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4fc74 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4fc78 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x4fc80 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x4fc8c | store | 1 | `(unknown)` | `0x0` |
| `ldrb` at 0x4fc90 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x4fc94 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x4fc98 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4fca8 | store | 4 | `sp -0x30` | `ret(0x532e0)` |
| `ldr` at 0x4fcb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4fcb4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x4fcc0 | store | 4 | `sp -0x28` | `ret(0x51d50)` |
| `ldr` at 0x4fcc4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4fcd0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4fcd4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x4fcdc | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x4fce8 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x4fcec | load | 4 | `sp -0x28` | `-` |
| `str` at 0x4fcf0 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x4fcf8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4fcfc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x4fd08 | store | 4 | `sp -0x28` | `ret(0x52ac4)` |
| `ldr` at 0x4fd0c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4fd10 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x4fd18 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x4fd24 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x4fd28 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x4fd2c | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x4fd30 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x4fd38 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4fd38 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4fd38 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4fb20 | `(unknown callee) 0x508a4` | `[sp-20]` | `?` | `[[sp-28]+8]` | `?` |
| `bl` at 0x4fbb4 | `(unknown callee) 0x53024` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x4fbfc | `(unknown callee) 0x51c34` | `[sp-20]` | `ret(0x53024)` | `?` | `ret(0x53024)` |
| `bl` at 0x4fc5c | `(unknown callee) 0x53100` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x4fca0 | `(unknown callee) 0x532e0` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x4fcb8 | `(unknown callee) 0x51d50` | `[sp-20]` | `ret(0x53100)` | `?` | `ret(0x53100)` |
| `bl` at 0x4fd00 | `(unknown callee) 0x52ac4` | `[sp-20]` | `[sp-36]` | `?` | `ret(0x51d50)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x4fa74 | `[0x1324a0] != 0x0` | `[0x1324a0] == 0x0` |
| `bne` at 0x4fa80 | `(r1 & 0xff) != 0xff` | `(r1 & 0xff) == 0xff` |
| `bne` at 0x4fad0 | `[sp-21]:1 != 0xff` | `[sp-21]:1 == 0xff` |
| `ble` at 0x4faec | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ble` at 0x4fb44 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x4fb88 | `[sp-22]:1 != 0xff` | `[sp-22]:1 == 0xff` |
| `bne` at 0x4fbc8 | `ret(0x53024) != 0x0` | `ret(0x53024) == 0x0` |
| `bne` at 0x4fc70 | `ret(0x53100) != 0x0` | `ret(0x53100) == 0x0` |
| `bge` at 0x4fccc | `ret(0x51d50) >= 0x0 (signed, 32-bit)` | `ret(0x51d50) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4fd38 | `r0` | `?` |

## `pcixDown`

- `pcixDown`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4fd48 to 0x50004.
  Established. Included byte stores write `0x1` and `0x0` to unknown effective targets (`strb` at 0x4fe0c, `strb` at 0x4fe64).
  An included unnamed `0x523d8` call carries `[sp-20]`, `ret(0x53100)`, unknown, `ret(0x53100)` (`bl` at 0x4ffbc).
  PCI-X shutdown completion semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4fd58 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x4fd60 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x4fd68 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0x4fd6c | load | 4 | `abs 0x4fff8` | `-` |
| `ldr` at 0x4fd70 | load | 4 | `abs 0x1324a0` | `-` |
| `ldrb` at 0x4fd7c | load | 1 | `sp -0x15` | `-` |
| `str` at 0x4fd8c | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x4fd94 | load | 4 | `abs 0x4fffc` | `-` |
| `str` at 0x4fd98 | store | 4 | `sp -0x34` | `0xfffff703` |
| `ldr` at 0x4fd9c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x4fda0 | store | 4 | `sp -0x30` | `[sp-52]` |
| `ldr` at 0x4fda8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4fdbc | load | 4 | `abs 0x50000` | `-` |
| `ldr` at 0x4fdc0 | load | 4 | `abs 0x13249c` | `-` |
| `str` at 0x4fdc8 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x4fdcc | load | 1 | `sp -0x15` | `-` |
| `str` at 0x4fddc | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x4fde0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4fde4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x4fde8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4fdf4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4fdf8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4fe00 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x4fe0c | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x4fe10 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x4fe18 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x4fe20 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4fe2c | store | 4 | `sp -0x28` | `ret(0x5099c)` |
| `str` at 0x4fe34 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x4fe38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4fe3c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x4fe40 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4fe4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4fe50 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4fe58 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x4fe64 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x4fe68 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x4fe70 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x4fe78 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x4fe7c | store | 4 | `sp -0x30` | `?` |
| `ldrb` at 0x4fe84 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x4fe90 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4fe94 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x4fe9c | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x4fea8 | store | 1 | `(unknown)` | `0x1` |
| `ldrb` at 0x4feac | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x4feb0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4fec0 | store | 4 | `sp -0x20` | `ret(0x53024)` |
| `ldr` at 0x4fec4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4fed0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4fed4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x4fedc | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x4fee8 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x4feec | load | 4 | `abs 0x4fffc` | `-` |
| `str` at 0x4fef0 | store | 4 | `sp -0x30` | `0xfffff703` |
| `ldr` at 0x4fef8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4fefc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4ff08 | store | 4 | `sp -0x28` | `ret(0x522fc)` |
| `ldr` at 0x4ff0c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4ff10 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x4ff18 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x4ff24 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x4ff28 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x4ff2c | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x4ff34 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4ff38 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x4ff40 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x4ff4c | store | 1 | `(unknown)` | `0x1` |
| `ldrb` at 0x4ff50 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x4ff54 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x4ff58 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4ff68 | store | 4 | `sp -0x24` | `ret(0x53100)` |
| `ldr` at 0x4ff6c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4ff78 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4ff7c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x4ff84 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x4ff90 | store | 1 | `(unknown)` | `0x0` |
| `ldrb` at 0x4ff94 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x4ff98 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x4ff9c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4ffac | store | 4 | `sp -0x30` | `ret(0x532e0)` |
| `ldr` at 0x4ffb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4ffb8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x4ffc4 | store | 4 | `sp -0x28` | `ret(0x523d8)` |
| `ldr` at 0x4ffc8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4ffcc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x4ffd4 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x4ffe0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x4ffe4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x4ffe8 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x4ffec | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x4fff4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4fff4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4fff4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4fe24 | `(unknown callee) 0x5099c` | `[sp-20]` | `?` | `[[sp-28]+8]` | `?` |
| `bl` at 0x4feb8 | `(unknown callee) 0x53024` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x4ff00 | `(unknown callee) 0x522fc` | `[sp-20]` | `ret(0x53024)` | `?` | `ret(0x53024)` |
| `bl` at 0x4ff60 | `(unknown callee) 0x53100` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x4ffa4 | `(unknown callee) 0x532e0` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x4ffbc | `(unknown callee) 0x523d8` | `[sp-20]` | `ret(0x53100)` | `?` | `ret(0x53100)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x4fd78 | `[0x1324a0] != 0x0` | `[0x1324a0] == 0x0` |
| `bne` at 0x4fd84 | `(r1 & 0xff) != 0xff` | `(r1 & 0xff) == 0xff` |
| `bne` at 0x4fdd4 | `[sp-21]:1 != 0xff` | `[sp-21]:1 == 0xff` |
| `ble` at 0x4fdf0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ble` at 0x4fe48 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x4fe8c | `[sp-22]:1 != 0xff` | `[sp-22]:1 == 0xff` |
| `bne` at 0x4fecc | `ret(0x53024) != 0x0` | `ret(0x53024) == 0x0` |
| `bne` at 0x4ff74 | `ret(0x53100) != 0x0` | `ret(0x53100) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4fff4 | `r0` | `?` |

## `pcixPresentInt`

- `pcixPresentInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x51268 to 0x51440.
  Established. A byte assignment writes an unknown value to `sp -0x21` (`strb` at 0x512ac).
  The unnamed `0x53570` call carries entry `r0`, `(r1 & 0xff)`, unknown, `(r1 & 0xff)` (`bl` at 0x512d8).
  Presence interrupt mask meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x51278 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x51284 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x5128c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x51294 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `strb` at 0x5129c | store | 1 | `sp -0x22` | `0x0` |
| `ldrb` at 0x512a0 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x512a4 | load | 1 | `sp -0x17` | `-` |
| `strb` at 0x512ac | store | 1 | `sp -0x21` | `?` |
| `ldrb` at 0x512b0 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x512c4 | store | 1 | `sp -0x22` | `0x1` |
| `ldrb` at 0x512c8 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x512cc | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0x512d0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x512e0 | store | 4 | `sp -0x1c` | `ret(0x53570)` |
| `ldr` at 0x512e4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x512e8 | load | 1 | `ret(0x53570) +0x6` | `-` |
| `ldrb` at 0x512fc | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x5130c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51314 | load | 4 | `abs 0x51430` | `-` |
| `ldr` at 0x51318 | load | 4 | `abs 0x51434` | `-` |
| `str` at 0x51354 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x51358 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51360 | load | 4 | `abs 0x51430` | `-` |
| `ldr` at 0x51364 | load | 4 | `abs 0x51434` | `-` |
| `ldr` at 0x51370 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x51380 | load | 4 | `abs 0x51438` | `-` |
| `ldrb` at 0x51390 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x513a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x513b0 | load | 4 | `abs 0x51430` | `-` |
| `ldr` at 0x513b4 | load | 4 | `abs 0x51434` | `-` |
| `str` at 0x513f0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x513f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x513fc | load | 4 | `abs 0x51430` | `-` |
| `ldr` at 0x51400 | load | 4 | `abs 0x51434` | `-` |
| `ldr` at 0x5140c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5141c | load | 4 | `abs 0x5143c` | `-` |
| `ldm` at 0x5142c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x5142c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x5142c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x512d8 | `(unknown callee) 0x53570` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0x5131c | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0xf8aec` | `0xf8af4` |
| `bl` at 0x51328 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x51368 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0xf8aec` | `0xf8af4` |
| `bl` at 0x51378 | `(unknown callee) 0x19808` | `[sp-40]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x51388 | `(unknown callee) 0x1e958` | `0xf8c28` | `ret(0x19808)` | `?` | `ret(0x19808)` |
| `bl` at 0x513b8 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0xf8aec` | `0xf8af4` |
| `bl` at 0x513c4 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x51404 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0xf8aec` | `0xf8af4` |
| `bl` at 0x51414 | `(unknown callee) 0x19808` | `[sp-40]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x51424 | `(unknown callee) 0x1e958` | `0xf8c60` | `ret(0x19808)` | `?` | `ret(0x19808)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x512bc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x512f4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x51308 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x513a4 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x5142c | `r0` | `?` |

## `pcixInt`

- `pcixInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x50f68 to 0x51268.
  Established. A nonzero `[0x1324a0]` takes `0x50fac`, while zero falls through (`bne` at 0x50fa4).
  Included unnamed calls carry `[sp-20]`, `[sp-32]`, unknown, unknown to `0x523d8` and `[sp-20]`, `[sp-32]`, `0x1`, unknown to `0x53f84` (`bl` at 0x51164, `bl` at 0x511cc).
  PCI-X interrupt recovery policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x50f78 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x50f84 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x50f8c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x50f94 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldr` at 0x50f98 | load | 4 | `abs 0x51230` | `-` |
| `ldr` at 0x50f9c | load | 4 | `abs 0x1324a0` | `-` |
| `ldr` at 0x50fac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50fc0 | load | 4 | `abs 0x51234` | `-` |
| `ldr` at 0x50fc4 | load | 4 | `abs 0x13249c` | `-` |
| `str` at 0x50fcc | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x50fd0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x50fd4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x50fe4 | store | 4 | `sp -0x20` | `ret(0x533c4)` |
| `ldr` at 0x50fe8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x50ff4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50ffc | load | 4 | `abs 0x51238` | `-` |
| `ldr` at 0x51000 | load | 4 | `abs 0x5123c` | `-` |
| `str` at 0x5103c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x51040 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51048 | load | 4 | `abs 0x51238` | `-` |
| `ldr` at 0x5104c | load | 4 | `abs 0x5123c` | `-` |
| `ldr` at 0x51058 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x51068 | load | 4 | `abs 0x51240` | `-` |
| `str` at 0x5106c | store | 4 | `(unknown)` | `0xf8b5c` |
| `str` at 0x51070 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldrb` at 0x51074 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x51078 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x51080 | load | 4 | `abs 0x51244` | `-` |
| `ldr` at 0x51084 | load | 4 | `abs 0x51248` | `-` |
| `ldrb` at 0x51090 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x51094 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x51098 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x510a8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x510ac | load | 1 | `ret(0x533c4) +0x6` | `-` |
| `ldr` at 0x510bc | load | 4 | `abs 0x5124c` | `-` |
| `str` at 0x510c0 | store | 4 | `sp -0x38` | `0xf8b8c` |
| `ldr` at 0x510c4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x510c8 | load | 1 | `ret(0x533c4) +0x0` | `-` |
| `str` at 0x510cc | store | 4 | `sp -0x34` | `[ret(0x533c4)]:1` |
| `ldr` at 0x510d0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x510d4 | load | 1 | `ret(0x533c4) +0x1` | `-` |
| `str` at 0x510d8 | store | 4 | `sp -0x30` | `[ret(0x533c4)+1]:1` |
| `ldr` at 0x510e0 | load | 4 | `abs 0x51244` | `-` |
| `ldr` at 0x510e4 | load | 4 | `abs 0x51250` | `-` |
| `ldrb` at 0x510f0 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x510f4 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x510f8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x51108 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x51118 | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x51128 | load | 4 | `abs 0x51254` | `-` |
| `str` at 0x5112c | store | 4 | `sp -0x38` | `0xf8bb8` |
| `ldr` at 0x51130 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x51134 | load | 1 | `ret(0x533c4) +0x0` | `-` |
| `str` at 0x51138 | store | 4 | `sp -0x34` | `[ret(0x533c4)]:1` |
| `ldr` at 0x5113c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x51140 | load | 1 | `ret(0x533c4) +0x1` | `-` |
| `str` at 0x51144 | store | 4 | `sp -0x30` | `[ret(0x533c4)+1]:1` |
| `ldr` at 0x5114c | load | 4 | `abs 0x51244` | `-` |
| `ldr` at 0x51150 | load | 4 | `abs 0x51258` | `-` |
| `ldr` at 0x5115c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51160 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5116c | store | 4 | `sp -0x24` | `ret(0x523d8)` |
| `ldr` at 0x51170 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5117c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x51188 | load | 4 | `abs 0x5125c` | `-` |
| `str` at 0x5118c | store | 4 | `sp -0x38` | `0xf8bd8` |
| `ldr` at 0x51190 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x51194 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x51198 | store | 4 | `sp -0x34` | `[[sp-32]]:1` |
| `ldr` at 0x5119c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x511a0 | load | 1 | `[sp-32] +0x1` | `-` |
| `str` at 0x511a4 | store | 4 | `sp -0x30` | `[[sp-32]+1]:1` |
| `str` at 0x511a8 | store | 4 | `sp -0x2c` | `ret(0x1fc1c)` |
| `ldr` at 0x511b0 | load | 4 | `abs 0x51244` | `-` |
| `ldr` at 0x511c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x511c4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x511d4 | store | 4 | `sp -0x24` | `ret(0x53f84)` |
| `ldr` at 0x511d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x511e4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x511f0 | load | 4 | `abs 0x51260` | `-` |
| `str` at 0x511f4 | store | 4 | `sp -0x38` | `0xf8c00` |
| `ldr` at 0x511f8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x511fc | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x51200 | store | 4 | `sp -0x34` | `[[sp-32]]:1` |
| `ldr` at 0x51204 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x51208 | load | 1 | `[sp-32] +0x1` | `-` |
| `str` at 0x5120c | store | 4 | `sp -0x30` | `[[sp-32]+1]:1` |
| `str` at 0x51210 | store | 4 | `sp -0x2c` | `ret(0x1fc1c)` |
| `ldr` at 0x51218 | load | 4 | `abs 0x51244` | `-` |
| `ldr` at 0x5121c | load | 4 | `abs 0x51264` | `-` |
| `ldm` at 0x5122c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x5122c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x5122c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x50fdc | `(unknown callee) 0x533c4` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x51004 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0xf8aec` | `0xf8af4` |
| `bl` at 0x51010 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x51050 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0xf8aec` | `0xf8af4` |
| `bl` at 0x51060 | `(unknown callee) 0x19808` | `[sp-40]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x5108c | `(unknown callee) 0x200a0` | `0x1` | `0xf8aa0` | `0x31b` | `0x0` |
| `bl` at 0x510a0 | `(unknown callee) 0xca878` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x510ec | `(unknown callee) 0x200a0` | `0x1` | `0xf8aa0` | `0x326` | `0x0` |
| `bl` at 0x51100 | `(unknown callee) 0xca878` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x51158 | `(unknown callee) 0x200a0` | `0x1` | `0xf8aa0` | `0x331` | `0x0` |
| `bl` at 0x51164 | `(unknown callee) 0x523d8` | `[sp-20]` | `[sp-32]` | `?` | `?` |
| `bl` at 0x51180 | `(unknown callee) 0x1fc1c` | `ret(0x523d8)` | `?` | `?` | `ret(0x523d8)` |
| `bl` at 0x511bc | `(unknown callee) 0x200a0` | `0x1` | `0xf8aa0` | `0x338` | `0x0` |
| `bl` at 0x511cc | `(unknown callee) 0x53f84` | `[sp-20]` | `[sp-32]` | `0x1` | `?` |
| `bl` at 0x511e8 | `(unknown callee) 0x1fc1c` | `ret(0x53f84)` | `?` | `?` | `ret(0x53f84)` |
| `bl` at 0x51224 | `(unknown callee) 0x200a0` | `0x1` | `0xf8aa0` | `0x33f` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x50fa4 | `[0x1324a0] != 0x0` | `[0x1324a0] == 0x0` |
| `bne` at 0x50ff0 | `ret(0x533c4) != 0x0` | `ret(0x533c4) == 0x0` |
| `beq` at 0x510b8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x51114 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x51124 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x51178 | `ret(0x523d8) == 0x0` | `ret(0x523d8) != 0x0` |
| `beq` at 0x511e0 | `ret(0x53f84) == 0x0` | `ret(0x53f84) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x5122c | `r0` | `?` |

## `pcixSetM66EN`

- `pcixSetM66EN`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x51558 to 0x516e0.
  Established. The unnamed `0x54014` call carries `[sp-20]`, unknown, `[sp-24]`, `[sp-48]+1` (`bl` at 0x51688).
  A zero `ret(0x54014)` takes `0x516b4`, while nonzero falls through (`beq` at 0x5169c).
  A word assignment increments `sp -0x30` with `[sp-48]+1` (`str` at 0x51678).
  PCI-X frequency-enable policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x51568 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x5156c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x51574 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x5157c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x51580 | load | 4 | `abs 0x516d8` | `-` |
| `ldr` at 0x51584 | load | 4 | `abs 0x1324a0` | `-` |
| `str` at 0x51594 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x5159c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x515b0 | load | 4 | `abs 0x516dc` | `-` |
| `ldr` at 0x515b4 | load | 4 | `abs 0x13249c` | `-` |
| `str` at 0x515bc | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x515c4 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x515c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x515cc | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x515d0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x515dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x515e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x515e8 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0x515f0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x515f4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x515f8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5160c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x51610 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x51620 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x51624 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x51630 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x51634 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5163c | load | 4 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0x51644 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x51650 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5165c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x51670 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x51678 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x5167c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51680 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x51684 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x51690 | store | 4 | `sp -0x28` | `ret(0x54014)` |
| `ldr` at 0x51694 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x516a0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x516ac | load | 4 | `sp -0x28` | `-` |
| `str` at 0x516b0 | store | 4 | `sp -0x24` | `ret(0x54014)` |
| `ldr` at 0x516b4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x516bc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x516c4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x516c8 | store | 4 | `sp -0x34` | `[sp-36]` |
| `ldr` at 0x516cc | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x516d4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x516d4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x516d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5166c | `(unknown callee) 0x5008c` | `0xa` | `[sp-28]` | `?` | `[sp-48]` |
| `bl` at 0x51688 | `(unknown callee) 0x54014` | `[sp-20]` | `?` | `[sp-24]` | `[sp-48]+1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5158c | `[0x1324a0] != 0x0` | `[0x1324a0] == 0x0` |
| `ble` at 0x515d8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x51604 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x5161c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x5162c | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x5164c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x51658 | `[sp-24] != 0x1` | `[sp-24] == 0x1` |
| `bne` at 0x51664 | `[sp-48] != 0x0` | `[sp-48] == 0x0` |
| `beq` at 0x5169c | `ret(0x54014) == 0x0` | `ret(0x54014) != 0x0` |
| `bne` at 0x516a8 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x516d4 | `r0` | `?` |

## `pcixGetSlotCfg`

- `pcixGetSlotCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4f91c to 0x4f9b0.
  Established. A word store writes `[[sp-28]+12]` to `[sp-24] +0x0` (`str` at 0x4f98c).
  A zero `[sp-24]` takes `0x4f990`, while nonzero falls through (`beq` at 0x4f97c).
  An included word read addresses `[sp-28] +0x10` (`ldr` at 0x4f994).
  PCI-X slot configuration ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4f92c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4f930 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x4f934 | load | 4 | `abs 0x4f9a8` | `-` |
| `ldr` at 0x4f938 | load | 4 | `abs 0x1324a0` | `-` |
| `str` at 0x4f948 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x4f950 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4f964 | load | 4 | `abs 0x4f9ac` | `-` |
| `ldr` at 0x4f968 | load | 4 | `abs 0x13249c` | `-` |
| `str` at 0x4f970 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x4f974 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4f980 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4f984 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4f988 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0x4f98c | store | 4 | `[sp-24] +0x0` | `[[sp-28]+12]` |
| `ldr` at 0x4f990 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4f994 | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0x4f998 | store | 4 | `sp -0x20` | `[[sp-28]+16]` |
| `ldr` at 0x4f99c | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x4f9a4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4f9a4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4f9a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x4f940 | `[0x1324a0] != 0x0` | `[0x1324a0] == 0x0` |
| `beq` at 0x4f97c | `[sp-24] == 0x0` | `[sp-24] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4f9a4 | `r0` | `?` |

## `pcixSlotIsPresent`

- `pcixSlotIsPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x51b64 to 0x51c34.
  Established. A nonzero `[0x1324a0]` takes `0x51bc0`, while zero falls through (`bne` at 0x51bb0).
  Included word reads address `[sp-28] +0xc` and `[sp-28] +0x14` (`ldr` at 0x51bf4, `ldr` at 0x51c04).
  The subsequent byte reads have unknown effective targets (`ldrb` at 0x51bfc, `ldrb` at 0x51c0c).
  Slot-presence flag encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x51b74 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x51b7c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x51b80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51b94 | load | 4 | `abs 0x51c2c` | `-` |
| `ldr` at 0x51b98 | load | 4 | `abs 0x13249c` | `-` |
| `str` at 0x51ba0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x51ba4 | load | 4 | `abs 0x51c30` | `-` |
| `ldr` at 0x51ba8 | load | 4 | `abs 0x1324a0` | `-` |
| `str` at 0x51bb8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x51bc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51bd4 | load | 4 | `abs 0x51c2c` | `-` |
| `ldr` at 0x51bd8 | load | 4 | `abs 0x13249c` | `-` |
| `str` at 0x51be0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x51be4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x51be8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x51bec | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x51bf4 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldrb` at 0x51bfc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x51c04 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0x51c0c | load | 1 | `(unknown)` | `-` |
| `str` at 0x51c1c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x51c20 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x51c28 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x51c28 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x51c28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x51bb0 | `[0x1324a0] != 0x0` | `[0x1324a0] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x51c28 | `r0` | `?` |

## `IP57_Init`

- `IP57_Init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x560cc to 0x56148.
  Established. A zero `ret(0x548e4)` takes `0x56134`, while nonzero falls through (`beq` at 0x560f8).
  The unnamed `0x563c0` call carries `[sp-17]:1`, `0x0`, `0x1`, `[sp-17]:1` (`bl` at 0x56124).
  IP57 initialization enable policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x560e0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x560e4 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x560fc | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x56114 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5612c | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x56138 | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0x5613c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x56144 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x56144 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x56144 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x560ec | `(unknown callee) 0x548e4` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x56104 | `(unknown callee) 0x5496c` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x56124 | `(unknown callee) 0x563c0` | `[sp-17]:1` | `0x0` | `0x1` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x560f8 | `ret(0x548e4) == 0x0` | `ret(0x548e4) != 0x0` |
| `bne` at 0x56110 | `ret(0x5496c) != 0x0` | `ret(0x5496c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x56144 | `r0` | `?` |

## `IP57_Probe`

- `IP57_Probe`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x55e54 to 0x560cc.
  Established. A zero `ret(0x57964)` takes `0x55ea8`, while nonzero falls through (`beq` at 0x55e98).
  The unnamed `0xca70c` call carries `0x0`, unknown, `sp-25`, unknown (`bl` at 0x55f54).
  A later unnamed `0x563c0` call carries `[sp-17]:1`, `0x1`, `0x0`, `[sp-17]:1` (`bl` at 0x5609c).
  IP57 probe acceptance criteria remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x55e68 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x55e70 | store | 4 | `sp -0x20` | `0x0` |
| `ldrb` at 0x55e74 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x55e8c | store | 4 | `sp -0x20` | `ret(0x57964)` |
| `ldr` at 0x55e90 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55ea0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x55ea8 | load | 4 | `abs 0x560b4` | `-` |
| `ldrb` at 0x55eac | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x55eb4 | store | 1 | `[sp-17]:1 +0x13256c` | `0x0` |
| `ldr` at 0x55eb8 | load | 4 | `abs 0x560b8` | `-` |
| `ldrb` at 0x55ebc | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x55ec4 | store | 1 | `[sp-17]:1 +0x299848` | `0x2` |
| `ldr` at 0x55ec8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x55ecc | load | 1 | `[sp-24] +0x3` | `-` |
| `strb` at 0x55ed4 | store | 1 | `sp -0x21` | `([[sp-24]+3]:1+5 & 0xff)` |
| `ldrb` at 0x55ed8 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x55edc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x55ee0 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0x55ee4 | load | 4 | `abs 0x560bc` | `-` |
| `ldr` at 0x55ee8 | load | 4 | `abs 0x132564` | `-` |
| `ldrb` at 0x55efc | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0x55f0c | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x55f14 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `strb` at 0x55f20 | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0x55f24 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x55f30 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x55f34 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0x55f38 | load | 1 | `sp -0x21` | `-` |
| `str` at 0x55f5c | store | 4 | `sp -0x20` | `ret(0xca70c)` |
| `ldr` at 0x55f60 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x55f6c | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x55f80 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x55f8c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x55f98 | load | 4 | `abs 0x560c0` | `-` |
| `str` at 0x55f9c | store | 4 | `sp -0x34` | `0xf92b0` |
| `ldrb` at 0x55fa0 | load | 1 | `sp -0x21` | `-` |
| `str` at 0x55fa4 | store | 4 | `sp -0x30` | `[sp-33]:1` |
| `str` at 0x55fa8 | store | 4 | `sp -0x2c` | `ret(0x1fc1c)` |
| `ldr` at 0x55fb0 | load | 4 | `abs 0x560c4` | `-` |
| `ldr` at 0x55fb4 | load | 4 | `abs 0x560c8` | `-` |
| `ldrb` at 0x55fc0 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x55fc4 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x55fd8 | load | 4 | `abs 0x560b4` | `-` |
| `ldrb` at 0x55fdc | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x55fe0 | load | 4 | `abs 0x560b4` | `-` |
| `ldrb` at 0x55fe4 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x55fe8 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0x55ff4 | load | 1 | `[sp-17]:1 +0x13256c` | `-` |
| `strb` at 0x55ffc | store | 1 | `[sp-17]:1 +0x13256c` | `?` |
| `ldrb` at 0x56000 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0x56010 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x5601c | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0x5603c | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x56040 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x56054 | load | 4 | `abs 0x560b8` | `-` |
| `ldrb` at 0x56058 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x56060 | store | 1 | `[sp-17]:1 +0x299848` | `0x1` |
| `ldrb` at 0x56064 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x5606c | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldrb` at 0x56074 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x5608c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x560a4 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x560a8 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x560b0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x560b0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x560b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x55e84 | `(unknown callee) 0x57964` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x55f08 | `(unknown callee) 0xc51dc` | `0x0` | `?` | `[[sp-24]+3]:1` | `?` |
| `bl` at 0x55f54 | `(unknown callee) 0xca70c` | `0x0` | `?` | `sp-25` | `?` |
| `bl` at 0x55f90 | `(unknown callee) 0x1fc1c` | `ret(0xca70c)` | `?` | `?` | `ret(0xca70c)` |
| `bl` at 0x55fbc | `(unknown callee) 0x200a0` | `0x1` | `0xf9204` | `0x165` | `0x0` |
| `bl` at 0x55fd0 | `(unknown callee) 0x56a84` | `[sp-17]:1` | `[sp-33]:1` | `[sp-33]:1` | `[sp-17]:1` |
| `bl` at 0x5602c | `(unknown callee) 0x542cc` | `?` | `0x13256c` | `?` | `?` |
| `bl` at 0x5604c | `(unknown callee) 0x56988` | `[sp-17]:1` | `[sp-33]:1` | `[sp-33]:1` | `[sp-17]:1` |
| `bl` at 0x5607c | `(unknown callee) 0x5496c` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x5609c | `(unknown callee) 0x563c0` | `[sp-17]:1` | `0x1` | `0x0` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x55e98 | `ret(0x57964) == 0x0` | `ret(0x57964) != 0x0` |
| `bge` at 0x55ef8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bhi` at 0x55f2c | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0x55f68 | `ret(0xca70c) != 0x0` | `ret(0xca70c) == 0x0` |
| `bne` at 0x55f78 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x55f88 | `ret(0xca70c) == 0x0` | `ret(0xca70c) != 0x0` |
| `beq` at 0x5600c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x56038 | `ret(0x542cc) != 0x0` | `ret(0x542cc) == 0x0` |
| `bne` at 0x56088 | `ret(0x5496c) != 0x0` | `ret(0x5496c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x560b0 | `r0` | `?` |

## `IP57_Cfg_Init`

- `IP57_Cfg_Init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5590c to 0x55b18.
  Established. Included stores write `0x1` as a byte to `0x132558`, `[sp-28]` as a word to `0x13255c`, `[sp-24]` as a word to `0x132560` and `0xc` as a word to `0x132564` (`strb` at 0x55960, `str` at 0x5596c, `str` at 0x55978, `str` at 0x55984).
  The unnamed `0x412f4` call carries `[0x29984c]`, `[0x13255c]`, `[0x132564]`, `0x0` (`bl` at 0x55ad8).
  IP57 configuration record meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5591c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x55920 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x55924 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x55928 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x5592c | load | 4 | `abs 0x55af0` | `-` |
| `ldrb` at 0x55930 | load | 1 | `abs 0x132558` | `-` |
| `ldr` at 0x5593c | load | 4 | `abs 0x55af4` | `-` |
| `str` at 0x55940 | store | 4 | `sp -0x44` | `0xf91e8` |
| `ldr` at 0x55948 | load | 4 | `abs 0x55af8` | `-` |
| `ldr` at 0x55958 | load | 4 | `abs 0x55af0` | `-` |
| `strb` at 0x55960 | store | 1 | `abs 0x132558` | `0x1` |
| `ldr` at 0x55964 | load | 4 | `abs 0x55afc` | `-` |
| `ldr` at 0x55968 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5596c | store | 4 | `abs 0x13255c` | `[sp-28]` |
| `ldr` at 0x55970 | load | 4 | `abs 0x55b00` | `-` |
| `ldr` at 0x55974 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x55978 | store | 4 | `abs 0x132560` | `[sp-24]` |
| `ldr` at 0x5597c | load | 4 | `abs 0x55b04` | `-` |
| `str` at 0x55984 | store | 4 | `abs 0x132564` | `0xc` |
| `ldr` at 0x55988 | load | 4 | `abs 0x55b08` | `-` |
| `ldr` at 0x5598c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x559a0 | store | 4 | `abs 0x29984c` | `ret(0x1943c)` |
| `ldr` at 0x559a4 | load | 4 | `abs 0x29984c` | `-` |
| `ldr` at 0x559b0 | load | 4 | `abs 0x55b0c` | `-` |
| `str` at 0x559b4 | store | 4 | `sp -0x44` | `0xf9218` |
| `ldr` at 0x559bc | load | 4 | `abs 0x55af8` | `-` |
| `str` at 0x559d0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x559d4 | load | 4 | `abs 0x55afc` | `-` |
| `ldr` at 0x559d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x559dc | load | 4 | `abs 0x13255c` | `-` |
| `ldr` at 0x559e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x559fc | load | 4 | `abs 0x55b00` | `-` |
| `ldr` at 0x55a00 | load | 4 | `abs 0x132560` | `-` |
| `str` at 0x55a08 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x55a0c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x55a14 | load | 4 | `abs 0x55b08` | `-` |
| `ldr` at 0x55a18 | load | 4 | `abs 0x29984c` | `-` |
| `str` at 0x55a20 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x55a24 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x55a28 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x55a2c | load | 1 | `(unknown)` | `-` |
| `strb` at 0x55a34 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x55a38 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x55a3c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x55a40 | load | 1 | `[sp-44] +0x3` | `-` |
| `strb` at 0x55a48 | store | 1 | `[sp-48] +0x1` | `([[sp-44]+3]:1+5 & 0xff)` |
| `ldr` at 0x55a4c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x55a50 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x55a54 | load | 4 | `[sp-44] +0xc` | `-` |
| `str` at 0x55a58 | store | 4 | `[sp-48] +0x4` | `[[sp-44]+12]` |
| `ldr` at 0x55a5c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x55a64 | store | 4 | `[sp-48] +0x8` | `0x200` |
| `ldr` at 0x55a68 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x55a6c | load | 4 | `abs 0x55b10` | `-` |
| `ldr` at 0x55a70 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x55a74 | load | 4 | `(unknown)` | `-` |
| `str` at 0x55a78 | store | 4 | `[sp-48] +0xc` | `?` |
| `ldr` at 0x55a7c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x55a84 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x55a8c | load | 4 | `abs 0x55b14` | `-` |
| `ldr` at 0x55a94 | load | 4 | `abs 0x55b08` | `-` |
| `ldr` at 0x55a98 | load | 4 | `abs 0x55afc` | `-` |
| `ldr` at 0x55a9c | load | 4 | `abs 0x55b04` | `-` |
| `str` at 0x55aa4 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x55aa8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55aac | store | 4 | `sp -0x40` | `[sp-32]` |
| `ldr` at 0x55ab0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x55ab4 | store | 4 | `sp -0x3c` | `[sp-36]` |
| `ldr` at 0x55ab8 | load | 4 | `abs 0x55b14` | `-` |
| `str` at 0x55abc | store | 4 | `sp -0x38` | `0x132570` |
| `str` at 0x55ac4 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x55ac8 | load | 4 | `abs 0x29984c` | `-` |
| `ldr` at 0x55acc | load | 4 | `abs 0x13255c` | `-` |
| `ldr` at 0x55ad0 | load | 4 | `abs 0x132564` | `-` |
| `ldr` at 0x55adc | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x55ae0 | load | 4 | `sp +0x4` | `-` |
| `ldm` at 0x55aec | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x55aec | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x55aec | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x55aec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x55954 | `(unknown callee) 0x200a0` | `0x9` | `0xf9204` | `0x91` | `0x0` |
| `bl` at 0x55998 | `(unknown callee) 0x1943c` | `?` | `?` | `0x132564` | `?` |
| `bl` at 0x559c8 | `(unknown callee) 0x200a0` | `0x9` | `0xf9204` | `0x9d` | `0x0` |
| `bl` at 0x55a90 | `(unknown callee) 0x55d94` | `0x132570` | `?` | `?` | `[0x13255c]` |
| `bl` at 0x55ad8 | `(unknown callee) 0x412f4` | `[0x29984c]` | `[0x13255c]` | `[0x132564]` | `0x0` |
| `bl` at 0x55ae4 | `(unknown callee) 0x55b18` | `[sp]` | `[sp+4]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x55938 | `[0x132558]:1 == 0x0` | `[0x132558]:1 != 0x0` |
| `bne` at 0x559ac | `[0x29984c] != 0x0` | `[0x29984c] == 0x0` |
| `bge` at 0x559e4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x55aec | `r0` | `ret(0x55b18)` |

## `IP57_Set_MemCfg`

- `IP57_Set_MemCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x55b18 to 0x55bdc.
  Established. Equality entry `r0 == 0x64` takes `0x55b58`, while inequality falls through (`beq` at 0x55b50).
  Included word assignments write `0xf90e8` to `sp -0x14` and `0x6` to `sp -0x18` (`str` at 0x55b5c, `str` at 0x55b64).
  The unnamed `0x417a4` call has four unknown candidates (`bl` at 0x55bc0).
  Memory configuration selector meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x55b28 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x55b2c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x55b30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55b3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x55b48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55b58 | load | 4 | `abs 0x55bcc` | `-` |
| `str` at 0x55b5c | store | 4 | `sp -0x14` | `0xf90e8` |
| `str` at 0x55b64 | store | 4 | `sp -0x18` | `0x6` |
| `ldr` at 0x55b6c | load | 4 | `abs 0x55bd0` | `-` |
| `str` at 0x55b70 | store | 4 | `sp -0x20` | `0xf9254` |
| `ldr` at 0x55b74 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x55b78 | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0x55b80 | load | 4 | `abs 0x55bd4` | `-` |
| `ldr` at 0x55b90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55b9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x55ba8 | load | 4 | `abs 0x55bd8` | `-` |
| `str` at 0x55bac | store | 4 | `sp -0x14` | `0xf9148` |
| `str` at 0x55bb4 | store | 4 | `sp -0x18` | `0x7` |
| `ldr` at 0x55bb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55bbc | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x55bc8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x55bc8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x55bc8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x55b8c | `(unknown callee) 0x200a0` | `0x9` | `0xf9204` | `0xd4` | `0x0` |
| `bl` at 0x55bc0 | `(unknown callee) 0x417a4` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x55b38 | `r0 == 0x0` | `r0 != 0x0` |
| `bne` at 0x55b44 | `r1 != 0x0` | `r1 == 0x0` |
| `beq` at 0x55b50 | `r0 == 0x64` | `r0 != 0x64` |
| `beq` at 0x55b98 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x55ba4 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x55bc8 | `r0` | `ret(0x417a4)` |

## `IP57_NodeRemove`

- `IP57_NodeRemove`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x56148 to 0x56258.
  Established. A zero `ret(0x57964)` takes `0x56194`, while nonzero falls through (`beq` at 0x56184).
  The unnamed `0x563c0` call carries `[sp-17]:1`, `0x1`, `0x1`, `[sp-17]:1` (`bl` at 0x561a4).
  Included unnamed `0xc5140` and `0xc8208` calls each carry `0x0` followed by three unknown candidates (`bl` at 0x561d8, `bl` at 0x5621c).
  Node-removal peripheral ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5615c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x56160 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x56178 | store | 4 | `sp -0x1c` | `ret(0x57964)` |
| `ldr` at 0x5617c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x56188 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5618c | store | 4 | `sp -0x24` | `ret(0x57964)` |
| `ldrb` at 0x56194 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x561a8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x561ac | load | 1 | `[sp-24] +0x3` | `-` |
| `strb` at 0x561b0 | store | 1 | `sp -0x1d` | `([[sp-24]+3]:1 & 0xff)` |
| `ldrb` at 0x561b4 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x561b8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x561bc | load | 1 | `[sp-24] +0x3` | `-` |
| `ldrb` at 0x561cc | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x561dc | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x561e4 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldr` at 0x561ec | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x561f0 | load | 1 | `[sp-24] +0x0` | `-` |
| `strb` at 0x561f4 | store | 1 | `sp -0x1d` | `([[sp-24]]:1 & 0xff)` |
| `ldrb` at 0x561f8 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x561fc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x56200 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldrb` at 0x56210 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x56220 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x56228 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldr` at 0x56230 | load | 4 | `abs 0x56254` | `-` |
| `ldrb` at 0x56234 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x5623c | store | 1 | `[sp-17]:1 +0x299848` | `0xf` |
| `str` at 0x56244 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x56248 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x56250 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x56250 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x56250 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x56170 | `(unknown callee) 0x57964` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x561a4 | `(unknown callee) 0x563c0` | `[sp-17]:1` | `0x1` | `0x1` | `[sp-17]:1` |
| `bl` at 0x561d8 | `(unknown callee) 0xc5140` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x5621c | `(unknown callee) 0xc8208` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x56184 | `ret(0x57964) == 0x0` | `ret(0x57964) != 0x0` |
| `bge` at 0x561c8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bge` at 0x5620c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x56250 | `r0` | `?` |

## `IP57_NodeEnable`

- `IP57_NodeEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x56290 to 0x562c4.
  Established. The unnamed `0x56578` call carries `(r0 & 0xff)`, `0x0`, entry `r2`, `(r0 & 0xff)` (`bl` at 0x562b4).
  The integer return candidate is `ret(0x56578)` (`ldm` at 0x562c0).
  IP57 node enable effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x562a4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x562a8 | load | 1 | `sp -0x11` | `-` |
| `ldm` at 0x562c0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x562c0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x562c0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x562c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x562b4 | `(unknown callee) 0x56578` | `(r0 & 0xff)` | `0x0` | `r2` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x562c0 | `r0` | `ret(0x56578)` |

## `IP57_NodeDisable`

- `IP57_NodeDisable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x56258 to 0x56290.
  Established. The unnamed `0x563c0` call carries `(r0 & 0xff)`, `0x0`, `0x1`, `(r0 & 0xff)` (`bl` at 0x56280).
  The return record preserves `ret(0x563c0)` (`ldm` at 0x5628c).
  IP57 node disable scope remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5626c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x56270 | load | 1 | `sp -0x11` | `-` |
| `ldm` at 0x5628c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x5628c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x5628c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x5628c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x56280 | `(unknown callee) 0x563c0` | `(r0 & 0xff)` | `0x0` | `0x1` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x5628c | `r0` | `ret(0x563c0)` |
