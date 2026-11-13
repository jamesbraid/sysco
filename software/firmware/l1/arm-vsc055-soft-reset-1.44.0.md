# ARM VSC055 and soft-reset paths in 1.44.0

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
The computed transfer in ioExpSoftResetInt remains unresolved; its tables cover known edges
only. The affected entry identifies its computed transfer.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/vsc055-soft-reset-facts.txt) retain all tool sections.

## `vsc055_Init`

- `vsc055_Init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcbf9c to 0xcc1d0.
  Established. The word at `0x12da2c` receives `0x1` (`str` at 0xcc024). The word at `0x12da28`
  receives `ret(0x196d0)` (`str` at 0xcc058). The unnamed `0x196d0` call at `0xcc0d8` carries
  `[[sp-36]+4]`, `0x1`, `[sp-36]`, `[sp-36]` (`bl` at 0xcc0d8). Device initialization meaning
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcbfac | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xcbfb0 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xcbfb4 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0xcbfb8 | load | 4 | `abs 0xcc1ac` | `-` |
| `ldr` at 0xcbfbc | load | 4 | `abs 0x12da28` | `-` |
| `ldr` at 0xcbfc0 | load | 4 | `abs 0xcc1b0` | `-` |
| `ldr` at 0xcbfcc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcbfd8 | load | 4 | `abs 0xcc1b4` | `-` |
| `str` at 0xcbfdc | store | 4 | `sp -0x34` | `0x1208b8` |
| `ldr` at 0xcbfe0 | load | 4 | `abs 0xcc1ac` | `-` |
| `ldr` at 0xcbfe4 | load | 4 | `abs 0x12da28` | `-` |
| `str` at 0xcbfe8 | store | 4 | `sp -0x30` | `[0x12da28]` |
| `ldr` at 0xcbfec | load | 4 | `abs 0xcc1b8` | `-` |
| `ldr` at 0xcbff0 | load | 4 | `abs 0x12da2c` | `-` |
| `str` at 0xcbff4 | store | 4 | `sp -0x2c` | `[0x12da2c]` |
| `ldr` at 0xcbffc | load | 4 | `abs 0xcc1bc` | `-` |
| `ldr` at 0xcc00c | load | 4 | `abs 0xcc1b8` | `-` |
| `ldr` at 0xcc010 | load | 4 | `abs 0x12da2c` | `-` |
| `ldr` at 0xcc01c | load | 4 | `abs 0xcc1b8` | `-` |
| `str` at 0xcc024 | store | 4 | `abs 0x12da2c` | `0x1` |
| `ldr` at 0xcc028 | load | 4 | `abs 0xcc1b8` | `-` |
| `ldr` at 0xcc02c | load | 4 | `abs 0x12da2c` | `-` |
| `str` at 0xcc040 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xcc044 | load | 4 | `abs 0xcc1ac` | `-` |
| `ldr` at 0xcc048 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xcc058 | store | 4 | `abs 0x12da28` | `ret(0x196d0)` |
| `ldr` at 0xcc05c | load | 4 | `abs 0x12da28` | `-` |
| `ldr` at 0xcc068 | load | 4 | `abs 0xcc1c0` | `-` |
| `str` at 0xcc06c | store | 4 | `sp -0x34` | `0x120904` |
| `ldr` at 0xcc070 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xcc074 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xcc07c | load | 4 | `abs 0xcc1bc` | `-` |
| `ldr` at 0xcc08c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcc0a0 | load | 4 | `abs 0xcc1ac` | `-` |
| `ldr` at 0xcc0a4 | load | 4 | `abs 0x12da28` | `-` |
| `str` at 0xcc0ac | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xcc0b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcc0b4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcc0b8 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0xcc0bc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcc0c0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcc0c4 | store | 4 | `[sp-36] +0x4` | `[sp-32]` |
| `ldr` at 0xcc0c8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcc0cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcc0d0 | load | 4 | `[sp-36] +0x4` | `-` |
| `str` at 0xcc0e0 | store | 4 | `[sp-36] +0x8` | `ret(0x196d0)` |
| `ldr` at 0xcc0e4 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0xcc0f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcc0f8 | load | 4 | `abs 0xcc1c4` | `-` |
| `ldr` at 0xcc0fc | load | 4 | `abs 0xcc1c8` | `-` |
| `str` at 0xcc138 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xcc13c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcc144 | load | 4 | `abs 0xcc1c4` | `-` |
| `ldr` at 0xcc148 | load | 4 | `abs 0xcc1c8` | `-` |
| `ldr` at 0xcc154 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcc164 | load | 4 | `abs 0xcc1cc` | `-` |
| `str` at 0xcc168 | store | 4 | `(unknown)` | `0x120948` |
| `str` at 0xcc16c | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0xcc170 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcc174 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0xcc17c | load | 4 | `abs 0xcc1bc` | `-` |
| `ldr` at 0xcc18c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcc190 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcc194 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0xcc19c | load | 4 | `[sp-36] +0x4` | `-` |
| `ldm` at 0xcc1a8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xcc1a8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcc1a8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcc1a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcc008 | `(unknown callee) 0x200a0` | `0x9` | `0x1208e8` | `0x47` | `0x0` |
| `bl` at 0xcc050 | `(unknown callee) 0x196d0` | `[sp-40]` | `0x1` | `[0x12da2c]` | `?` |
| `bl` at 0xcc088 | `(unknown callee) 0x200a0` | `0x9` | `0x1208e8` | `0x53` | `0x0` |
| `bl` at 0xcc0d8 | `(unknown callee) 0x196d0` | `[[sp-36]+4]` | `0x1` | `[sp-36]` | `[sp-36]` |
| `bl` at 0xcc100 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x12093c` | `0x120944` |
| `bl` at 0xcc10c | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xcc14c | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x12093c` | `0x120944` |
| `bl` at 0xcc15c | `(unknown callee) 0x19808` | `[sp-40]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xcc188 | `(unknown callee) 0x200a0` | `0x9` | `0x1208e8` | `0x5f` | `0x0` |
| `bl` at 0xcc1a0 | `(unknown callee) 0x19748` | `[[sp-36]+8]` | `0x1` | `[[sp-36]+4]` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xcbfc8 | `[0x12da28] == 0x297344` | `[0x12da28] != 0x297344` |
| `bne` at 0xcbfd4 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xcc018 | `[0x12da2c] != 0x0` | `[0x12da2c] == 0x0` |
| `bne` at 0xcc064 | `[0x12da28] != 0x0` | `[0x12da28] == 0x0` |
| `bne` at 0xcc0ec | `[[sp-36]+8] != 0x0` | `[[sp-36]+8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcc1a8 | `r0` | `ret(0x19748)` |

## `vsc055_Reset`

- `vsc055_Reset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcc600 to 0xcc8fc.
  Established. The signed condition `(r1 & 0xff) <= [[sp-28]+4]` takes `0xcc6f0`, while greater
  values fall through (`ble` at 0xcc650). The unnamed `0xbb498` call carries unknown,
  `[[sp-32]+1]:1`, `0xfc`, `0x80` (`bl` at 0xcc770). The unnamed `0xbb5c4` call carries unknown,
  `[[sp-32]+1]:1`, `0xff`, `sp-33` (`bl` at 0xcc7cc). Device reset transaction semantics remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcc610 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xcc618 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xcc61c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc630 | load | 4 | `abs 0xcc8e0` | `-` |
| `ldr` at 0xcc634 | load | 4 | `abs 0x12da28` | `-` |
| `str` at 0xcc63c | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xcc640 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcc644 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcc648 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xcc654 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc65c | load | 4 | `abs 0xcc8e4` | `-` |
| `ldr` at 0xcc660 | load | 4 | `abs 0xcc8e8` | `-` |
| `str` at 0xcc69c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xcc6a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc6a8 | load | 4 | `abs 0xcc8e4` | `-` |
| `ldr` at 0xcc6ac | load | 4 | `abs 0xcc8e8` | `-` |
| `ldr` at 0xcc6b8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcc6c8 | load | 4 | `abs 0xcc8ec` | `-` |
| `str` at 0xcc6cc | store | 4 | `(unknown)` | `0x1209c8` |
| `str` at 0xcc6d0 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldrb` at 0xcc6d4 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xcc6d8 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0xcc6e0 | load | 4 | `abs 0xcc8f0` | `-` |
| `ldr` at 0xcc6f0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcc6f4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcc6f8 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0xcc700 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xcc710 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcc714 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcc71c | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xcc724 | store | 4 | `sp -0x20` | `?` |
| `str` at 0xcc72c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xcc730 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcc73c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcc740 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xcc744 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc758 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcc75c | load | 1 | `[sp-32] +0x1` | `-` |
| `str` at 0xcc778 | store | 4 | `sp -0x28` | `ret(0xbb498)` |
| `ldr` at 0xcc77c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcc794 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcc798 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xcc79c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc7b0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcc7b4 | load | 1 | `[sp-32] +0x1` | `-` |
| `str` at 0xcc7d4 | store | 4 | `sp -0x28` | `ret(0xbb5c4)` |
| `ldr` at 0xcc7d8 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xcc7e8 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xcc7f8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xcc800 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xcc808 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc810 | load | 4 | `abs 0xcc8e4` | `-` |
| `ldr` at 0xcc814 | load | 4 | `abs 0xcc8e8` | `-` |
| `str` at 0xcc850 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xcc854 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc85c | load | 4 | `abs 0xcc8e4` | `-` |
| `ldr` at 0xcc860 | load | 4 | `abs 0xcc8e8` | `-` |
| `ldr` at 0xcc86c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xcc878 | store | 4 | `sp -0x34` | `ret(0x19808)` |
| `ldr` at 0xcc87c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcc888 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xcc890 | store | 4 | `sp -0x38` | `ret(0x1fc1c)` |
| `ldr` at 0xcc898 | load | 4 | `abs 0xcc8f4` | `-` |
| `str` at 0xcc89c | store | 4 | `sp -0x38` | `0x1209e0` |
| `ldr` at 0xcc8a0 | load | 4 | `abs 0xcc8f8` | `-` |
| `str` at 0xcc8a4 | store | 4 | `(unknown)` | `0x1209f8` |
| `ldr` at 0xcc8a8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xcc8ac | store | 4 | `(unknown)` | `[sp-52]` |
| `ldr` at 0xcc8b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcc8b4 | load | 4 | `[sp-32] +0x4` | `-` |
| `str` at 0xcc8b8 | store | 4 | `(unknown)` | `[[sp-32]+4]` |
| `ldr` at 0xcc8bc | load | 4 | `sp -0x38` | `-` |
| `str` at 0xcc8c0 | store | 4 | `(unknown)` | `[sp-56]` |
| `ldr` at 0xcc8c8 | load | 4 | `abs 0xcc8f0` | `-` |
| `ldm` at 0xcc8dc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcc8dc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcc8dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcc664 | `(unknown callee) 0x48da8` | `r0` | `0x0` | `0x12093c` | `0x120944` |
| `bl` at 0xcc670 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xcc6b0 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0x12093c` | `0x120944` |
| `bl` at 0xcc6c0 | `(unknown callee) 0x19808` | `[sp-48]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xcc6ec | `(unknown callee) 0x200a0` | `0x9` | `0x1208e8` | `0xc1` | `0x0` |
| `bl` at 0xcc74c | `(unknown callee) 0xbac78` | `?` | `[[sp-32]]:1` | `?` | `[[sp-32]]:1` |
| `bl` at 0xcc770 | `(unknown callee) 0xbb498` | `?` | `[[sp-32]+1]:1` | `0xfc` | `0x80` |
| `bl` at 0xcc790 | `(unknown callee) 0xcc8fc` | `0x2` | `?` | `?` | `ret(0xbb498)` |
| `bl` at 0xcc7a4 | `(unknown callee) 0xbac78` | `[sp-20]` | `[[sp-32]]:1` | `?` | `[[sp-32]]:1` |
| `bl` at 0xcc7cc | `(unknown callee) 0xbb5c4` | `?` | `[[sp-32]+1]:1` | `0xff` | `sp-33` |
| `bl` at 0xcc818 | `(unknown callee) 0x48da8` | `?` | `0x0` | `0x12093c` | `0x120944` |
| `bl` at 0xcc824 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xcc864 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0x12093c` | `0x120944` |
| `bl` at 0xcc874 | `(unknown callee) 0x19808` | `[sp-48]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xcc88c | `(unknown callee) 0x1fc1c` | `[sp-40]` | `?` | `?` | `[sp-40]` |
| `bl` at 0xcc8d4 | `(unknown callee) 0x200a0` | `0x3` | `0x1208e8` | `0xe1` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xcc650 | `(r1 & 0xff) <= [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) > [[sp-28]+4] (signed, 32-bit)` |
| `bne` at 0xcc708 | `Z == 0` | `not (Z == 0)` |
| `bgt` at 0xcc738 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0xcc784 | `ret(0xbb498) == 0x0` | `ret(0xbb498) != 0x0` |
| `beq` at 0xcc7e0 | `ret(0xbb5c4) == 0x0` | `ret(0xbb5c4) != 0x0` |
| `bne` at 0xcc7f0 | `[sp-33]:1 != 0x21` | `[sp-33]:1 == 0x21` |
| `beq` at 0xcc884 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcc8dc | `r0` | `?` |

## `vsc055_ResetAll`

- `vsc055_ResetAll`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcc564 to 0xcc600.
  Established. The absolute word at `0x12da28` is read (`ldr` at 0xcc590). The byte at `sp -0x19`
  starts at zero (`strb` at 0xcc5a0). The unnamed `0xcc600` call has four unknown candidates (`bl`
  at 0xcc5e0). Device reset traversal remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcc574 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xcc578 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc58c | load | 4 | `abs 0xcc5fc` | `-` |
| `ldr` at 0xcc590 | load | 4 | `abs 0x12da28` | `-` |
| `str` at 0xcc598 | store | 4 | `sp -0x18` | `?` |
| `strb` at 0xcc5a0 | store | 1 | `sp -0x19` | `0x0` |
| `ldrb` at 0xcc5a4 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xcc5a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcc5ac | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xcc5b8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcc5bc | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xcc5c0 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0xcc5c8 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xcc5d4 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xcc5d8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xcc5e4 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xcc5ec | store | 1 | `sp -0x19` | `?` |
| `ldm` at 0xcc5f8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcc5f8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcc5f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcc5e0 | `(unknown callee) 0xcc600` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xcc5b4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xcc5d0 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcc5f8 | `r0` | `?` |

## `vsc055_PortSoftReset`

- `vsc055_PortSoftReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcca74 to 0xccdf4.
  Established. The unnamed `0xbb5c4` call carries unknown, `[[sp-24]+2]:1`, unknown, `sp-27` (`bl`
  at 0xccad8); its zero result takes `0xccafc`, while nonzero falls through (`beq` at 0xccaec). The
  unnamed `0xbb498` call at `0xccd38` carries unknown, `[[sp-24]+2]:1`, unknown, `[sp-28]:1` (`bl`
  at 0xccd38). Port soft-reset ordering remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcca84 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcca88 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xcca90 | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `ldr` at 0xcca94 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcca98 | load | 1 | `r1 +0x1` | `-` |
| `ldr` at 0xcca9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xccab0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xccab4 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xccab8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xccabc | load | 1 | `[sp-24] +0x3` | `-` |
| `str` at 0xccae0 | store | 4 | `sp -0x20` | `ret(0xbb5c4)` |
| `ldr` at 0xccae4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xccaf0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xccaf4 | store | 4 | `sp -0x28` | `ret(0xbb5c4)` |
| `ldr` at 0xccafc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xccb00 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldrb` at 0xccb04 | load | 1 | `sp -0x1b` | `-` |
| `strb` at 0xccb1c | store | 1 | `sp -0x1a` | `?` |
| `ldr` at 0xccb20 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xccb24 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xccb2c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xccb30 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldrb` at 0xccb38 | load | 1 | `sp -0x1b` | `-` |
| `ldrb` at 0xccb48 | load | 1 | `sp -0x1a` | `-` |
| `strb` at 0xccb50 | store | 1 | `sp -0x1a` | `?` |
| `ldr` at 0xccb54 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xccb58 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xccb5c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xccb60 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldrb` at 0xccb68 | load | 1 | `sp -0x1b` | `-` |
| `ldrb` at 0xccb74 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0xccb84 | load | 1 | `sp -0x1a` | `-` |
| `strb` at 0xccb8c | store | 1 | `sp -0x1a` | `?` |
| `ldrb` at 0xccb90 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xccb9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xccba0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xccba4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xccba8 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldrb` at 0xccbac | load | 1 | `[sp-24] +0x5` | `-` |
| `ldrb` at 0xccbbc | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0xccbc4 | load | 1 | `[sp-24] +0x4` | `-` |
| `strb` at 0xccbcc | store | 1 | `sp -0x1c` | `?` |
| `ldr` at 0xccbd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xccbd8 | load | 4 | `abs 0xccde8` | `-` |
| `ldr` at 0xccbdc | load | 4 | `abs 0xccdec` | `-` |
| `str` at 0xccc18 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xccc1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xccc24 | load | 4 | `abs 0xccde8` | `-` |
| `ldr` at 0xccc28 | load | 4 | `abs 0xccdec` | `-` |
| `ldr` at 0xccc34 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xccc44 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xccc48 | load | 1 | `sp -0x1a` | `-` |
| `ldrb` at 0xccc4c | load | 1 | `sp -0x1b` | `-` |
| `str` at 0xccc50 | store | 4 | `(unknown)` | `[sp-27]:1` |
| `ldrb` at 0xccc54 | load | 1 | `sp -0x1c` | `-` |
| `str` at 0xccc58 | store | 4 | `(unknown)` | `[sp-28]:1` |
| `ldrb` at 0xccc5c | load | 1 | `sp -0x19` | `-` |
| `str` at 0xccc60 | store | 4 | `(unknown)` | `[sp-25]:1` |
| `ldr` at 0xccc64 | load | 4 | `abs 0xccdf0` | `-` |
| `ldr` at 0xccc6c | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xccc78 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xccc7c | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xccc80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xccc94 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xccc98 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xccc9c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xccca0 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0xccca4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xccca8 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldrb` at 0xcccac | load | 1 | `sp -0x1b` | `-` |
| `ldrb` at 0xcccb8 | load | 1 | `sp -0x19` | `-` |
| `str` at 0xcccd8 | store | 4 | `sp -0x20` | `ret(0xbb498)` |
| `ldr` at 0xcccdc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xccce8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcccec | store | 4 | `sp -0x28` | `ret(0xbb498)` |
| `ldr` at 0xcccf4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcccf8 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xcccfc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xccd10 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xccd14 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xccd18 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xccd1c | load | 1 | `[sp-24] +0x3` | `-` |
| `ldrb` at 0xccd28 | load | 1 | `sp -0x1c` | `-` |
| `str` at 0xccd40 | store | 4 | `sp -0x20` | `ret(0xbb498)` |
| `ldr` at 0xccd44 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xccd50 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xccd54 | store | 4 | `sp -0x28` | `ret(0xbb498)` |
| `ldr` at 0xccd5c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xccd60 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xccd64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xccd78 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xccd7c | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xccd80 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xccd84 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0xccd88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xccd8c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xccd90 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldrb` at 0xccd94 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0xccd9c | load | 1 | `[sp-24] +0x4` | `-` |
| `str` at 0xccdb8 | store | 4 | `sp -0x20` | `ret(0xbb498)` |
| `ldr` at 0xccdbc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xccdc8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xccdcc | store | 4 | `sp -0x28` | `ret(0xbb498)` |
| `str` at 0xccdd8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xccddc | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xccde4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xccde4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xccde4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xccaa4 | `(unknown callee) 0xbac78` | `r0` | `[r1+1]:1` | `r2` | `[r1+1]:1` |
| `bl` at 0xccad8 | `(unknown callee) 0xbb5c4` | `?` | `[[sp-24]+2]:1` | `?` | `sp-27` |
| `bl` at 0xccbe0 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0x12093c` | `0x120944` |
| `bl` at 0xccbec | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xccc2c | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0x12093c` | `0x120944` |
| `bl` at 0xccc3c | `(unknown callee) 0x19808` | `[sp-36]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xccc74 | `(unknown callee) 0x1e958` | `0x120a1c` | `ret(0x19808)` | `[[sp-24]+12]` | `[sp-26]:1` |
| `bl` at 0xccc88 | `(unknown callee) 0xbac78` | `[sp-20]` | `[[sp-24]+1]:1` | `?` | `[[sp-24]+1]:1` |
| `bl` at 0xcccd0 | `(unknown callee) 0xbb498` | `?` | `[[sp-24]+2]:1` | `[[sp-24]+3]:1` | `?` |
| `bl` at 0xccd04 | `(unknown callee) 0xbac78` | `[sp-20]` | `[[sp-24]+1]:1` | `?` | `[[sp-24]+1]:1` |
| `bl` at 0xccd38 | `(unknown callee) 0xbb498` | `?` | `[[sp-24]+2]:1` | `?` | `[sp-28]:1` |
| `bl` at 0xccd6c | `(unknown callee) 0xbac78` | `[sp-20]` | `[[sp-24]+1]:1` | `?` | `[[sp-24]+1]:1` |
| `bl` at 0xccdb0 | `(unknown callee) 0xbb498` | `?` | `[[sp-24]+2]:1` | `[[sp-24]+3]:1` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xccaec | `ret(0xbb5c4) == 0x0` | `ret(0xbb5c4) != 0x0` |
| `beq` at 0xccb44 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xccb80 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xccb98 | `[sp-26]:1 == 0x0` | `[sp-26]:1 != 0x0` |
| `beq` at 0xccce4 | `ret(0xbb498) == 0x0` | `ret(0xbb498) != 0x0` |
| `beq` at 0xccd4c | `ret(0xbb498) == 0x0` | `ret(0xbb498) != 0x0` |
| `beq` at 0xccdc4 | `ret(0xbb498) == 0x0` | `ret(0xbb498) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xccde4 | `r0` | `?` |

## `vsc055_IntSet`

- `vsc055_IntSet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xccf3c to 0xcd058.
  Established. The unnamed `0xbb498` call carries unknown, `[[sp-24]+2]:1`, `[sp-34]:1`, `[sp-35]:1`
  (`bl` at 0xcd000); its zero result takes `0xcd024`, while nonzero falls through (`beq` at 0xcd014). The word at `sp -0x2c` receives zero on an included path (`str` at 0xcd044). Device
  interrupt configuration remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xccf4c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xccf50 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xccf54 | store | 4 | `sp -0x20` | `r3` |
| `strb` at 0xccf5c | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `ldr` at 0xccf60 | load | 4 | `abs 0xcd054` | `-` |
| `ldr` at 0xccf64 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xccf6c | load | 1 | `r3 +0x12da30` | `-` |
| `strb` at 0xccf70 | store | 1 | `sp -0x23` | `([r3+1235504]:1 & 0xff)` |
| `strb` at 0xccf78 | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0xccf7c | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0xccf88 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xccf98 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xccf9c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xccfa8 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0xccfc0 | store | 1 | `sp -0x22` | `?` |
| `ldr` at 0xccfc4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xccfc8 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xccfcc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xccfe0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xccfe4 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0xccfe8 | load | 1 | `sp -0x22` | `-` |
| `ldrb` at 0xccfec | load | 1 | `sp -0x23` | `-` |
| `str` at 0xcd008 | store | 4 | `sp -0x28` | `ret(0xbb498)` |
| `ldr` at 0xcd00c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcd018 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xcd01c | store | 4 | `sp -0x2c` | `ret(0xbb498)` |
| `ldrb` at 0xcd024 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xcd02c | store | 1 | `sp -0x19` | `?` |
| `ldrb` at 0xcd030 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0xcd038 | store | 1 | `sp -0x21` | `?` |
| `str` at 0xcd044 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xcd048 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0xcd050 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcd050 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcd050 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xccfd4 | `(unknown callee) 0xbac78` | `[sp-20]` | `[[sp-24]+1]:1` | `?` | `[[sp-24]+1]:1` |
| `bl` at 0xcd000 | `(unknown callee) 0xbb498` | `?` | `[[sp-24]+2]:1` | `[sp-34]:1` | `[sp-35]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhi` at 0xccf84 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0xccf94 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xcd014 | `ret(0xbb498) == 0x0` | `ret(0xbb498) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcd050 | `r0` | `?` |

## `vsc055_Int`

- `vsc055_Int`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcd1a0 to 0xcd278.
  Established. The word at `sp -0x24` starts at zero (`str` at 0xcd1ec). An included assignment
  there writes `([sp-36] + ret(0xcd278))` (`str` at 0xcd250). The unnamed `0xcd278` call has four
  unknown candidates (`bl` at 0xcd240). Interrupt traversal effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcd1b0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xcd1b8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xcd1c0 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0xcd1c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcd1d8 | load | 4 | `abs 0xcd274` | `-` |
| `ldr` at 0xcd1dc | load | 4 | `abs 0x12da28` | `-` |
| `str` at 0xcd1e4 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0xcd1ec | store | 4 | `sp -0x24` | `0x0` |
| `ldrb` at 0xcd1f0 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xcd1f4 | store | 4 | `sp -0x20` | `(r1 & 0xff)` |
| `ldrb` at 0xcd1f8 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xcd1fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcd208 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcd20c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xcd210 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcd218 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xcd224 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcd228 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcd230 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xcd238 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcd248 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcd250 | store | 4 | `sp -0x24` | `([sp-36] + ret(0xcd278))` |
| `ldr` at 0xcd254 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcd25c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcd264 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xcd270 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcd270 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcd270 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcd240 | `(unknown callee) 0xcd278` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xcd204 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xcd220 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcd270 | `r0` | `?` |

## `vsc055_PortReset`

- `vsc055_PortReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcc924 to 0xcca74.
  Established. The unnamed `0xbb498` call at `0xcc9e4` carries unknown, `[[sp-24]+2]:1`,
  `[[sp-24]+3]:1`, `[sp-30]:1` (`bl` at 0xcc9e4); its later call carries unknown, `[[sp-24]+2]:1`,
  unknown, `[sp-29]:1` (`bl` at 0xcca34). A zero later result takes `0xcca60`, while nonzero falls
  through (`beq` at 0xcca48). Port reset protocol remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcc934 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcc938 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xcc93c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc950 | load | 4 | `abs 0xcca70` | `-` |
| `ldr` at 0xcc954 | load | 4 | `abs 0x12da28` | `-` |
| `str` at 0xcc95c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xcc960 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcc964 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcc968 | load | 1 | `r1 +0x5` | `-` |
| `ldrb` at 0xcc96c | load | 1 | `r1 +0x6` | `-` |
| `strb` at 0xcc974 | store | 1 | `sp -0x1e` | `?` |
| `ldr` at 0xcc978 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcc97c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcc980 | load | 1 | `r1 +0x5` | `-` |
| `ldr` at 0xcc988 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcc990 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldrb` at 0xcc998 | load | 1 | `r1 +0x4` | `-` |
| `strb` at 0xcc9a0 | store | 1 | `sp -0x1d` | `?` |
| `ldr` at 0xcc9a4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcc9a8 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xcc9ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc9c0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcc9c4 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xcc9c8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcc9cc | load | 1 | `[sp-24] +0x3` | `-` |
| `ldrb` at 0xcc9d0 | load | 1 | `sp -0x1e` | `-` |
| `str` at 0xcc9ec | store | 4 | `sp -0x24` | `ret(0xbb498)` |
| `ldr` at 0xcc9f0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcc9f4 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xcc9f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcca0c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcca10 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xcca14 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcca18 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldrb` at 0xcca24 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0xcca3c | store | 4 | `sp -0x28` | `ret(0xbb498)` |
| `ldr` at 0xcca40 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcca4c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcca58 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xcca5c | store | 4 | `sp -0x24` | `ret(0xbb498)` |
| `ldr` at 0xcca60 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xcca6c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcca6c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcca6c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcc9b4 | `(unknown callee) 0xbac78` | `[sp-20]` | `[[sp-24]+1]:1` | `[r1+4]:1` | `[[sp-24]+1]:1` |
| `bl` at 0xcc9e4 | `(unknown callee) 0xbb498` | `?` | `[[sp-24]+2]:1` | `[[sp-24]+3]:1` | `[sp-30]:1` |
| `bl` at 0xcca00 | `(unknown callee) 0xbac78` | `[sp-20]` | `[[sp-24]+1]:1` | `?` | `[[sp-24]+1]:1` |
| `bl` at 0xcca34 | `(unknown callee) 0xbb498` | `?` | `[[sp-24]+2]:1` | `?` | `[sp-29]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xcca48 | `ret(0xbb498) == 0x0` | `ret(0xbb498) != 0x0` |
| `bne` at 0xcca54 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcca6c | `r0` | `[sp-36]` |

## `vsc055_Disable`

- `vsc055_Disable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcc39c to 0xcc564.
  Established. The signed condition `(r1 & 0xff) >= [[sp-28]+4]` takes `0xcc4ac`, while smaller
  values fall through (`bge` at 0xcc3ec). An included byte assignment through `((r1 & 0xff) + [[sp-28]+8]) +0x0` writes zero (`strb` at 0xcc404). The unnamed `0xc9a44` call carries `[sp-20]`,
  unknown, `[sp-32]`, unknown (`bl` at 0xcc48c). Device disable effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcc3ac | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xcc3b4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xcc3b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc3cc | load | 4 | `abs 0xcc550` | `-` |
| `ldr` at 0xcc3d0 | load | 4 | `abs 0x12da28` | `-` |
| `str` at 0xcc3d8 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xcc3dc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcc3e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcc3e4 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xcc3f0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcc3f4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcc3f8 | load | 4 | `[sp-28] +0x8` | `-` |
| `strb` at 0xcc404 | store | 1 | `((r1 & 0xff) + [[sp-28]+8]) +0x0` | `0x0` |
| `ldr` at 0xcc408 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcc40c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcc414 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xcc41c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcc424 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcc434 | store | 4 | `sp -0x28` | `ret(0xca3d0)` |
| `strb` at 0xcc43c | store | 1 | `sp -0x29` | `0x0` |
| `ldrb` at 0xcc440 | load | 1 | `sp -0x29` | `-` |
| `ldr` at 0xcc444 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcc450 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcc454 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcc458 | load | 1 | `[sp-36] +0x1` | `-` |
| `ldrb` at 0xcc45c | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xcc468 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcc46c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcc470 | load | 1 | `[sp-36] +0x2` | `-` |
| `ldrb` at 0xcc474 | load | 1 | `[sp-32] +0x1` | `-` |
| `ldrb` at 0xcc480 | load | 1 | `sp -0x29` | `-` |
| `ldr` at 0xcc484 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc490 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcc498 | store | 4 | `sp -0x24` | `[sp-36]+16` |
| `ldrb` at 0xcc49c | load | 1 | `sp -0x29` | `-` |
| `strb` at 0xcc4a4 | store | 1 | `sp -0x29` | `?` |
| `ldr` at 0xcc4ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc4b4 | load | 4 | `abs 0xcc554` | `-` |
| `ldr` at 0xcc4b8 | load | 4 | `abs 0xcc558` | `-` |
| `str` at 0xcc4f4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xcc4f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc500 | load | 4 | `abs 0xcc554` | `-` |
| `ldr` at 0xcc504 | load | 4 | `abs 0xcc558` | `-` |
| `ldr` at 0xcc510 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcc520 | load | 4 | `abs 0xcc55c` | `-` |
| `str` at 0xcc524 | store | 4 | `(unknown)` | `0x1209a4` |
| `str` at 0xcc528 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldrb` at 0xcc52c | load | 1 | `sp -0x15` | `-` |
| `str` at 0xcc530 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0xcc538 | load | 4 | `abs 0xcc560` | `-` |
| `ldm` at 0xcc54c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcc54c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcc54c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcc42c | `(unknown callee) 0xca3d0` | `[sp-20]` | `sp-36` | `?` | `sp-36` |
| `bl` at 0xcc48c | `(unknown callee) 0xc9a44` | `[sp-20]` | `?` | `[sp-32]` | `?` |
| `bl` at 0xcc4bc | `(unknown callee) 0x48da8` | `r0` | `0x0` | `0x12093c` | `0x120944` |
| `bl` at 0xcc4c8 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xcc508 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0x12093c` | `0x120944` |
| `bl` at 0xcc518 | `(unknown callee) 0x19808` | `[sp-48]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xcc544 | `(unknown callee) 0x200a0` | `0x6` | `0x1208e8` | `0x9e` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xcc3ec | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` |
| `bge` at 0xcc44c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xcc464 | `[[sp-36]+1]:1 != [[sp-32]]:1` | `[[sp-36]+1]:1 == [[sp-32]]:1` |
| `bne` at 0xcc47c | `[[sp-36]+2]:1 != [[sp-32]+1]:1` | `[[sp-36]+2]:1 == [[sp-32]+1]:1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcc54c | `r0` | `?` |

## `vsc055_Enable`

- `vsc055_Enable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcc1d0 to 0xcc39c.
  Established. The unnamed `0xca3d0` call carries entry `r0`, `sp-36`, unknown, `sp-36` (`bl` at 0xcc248). An included byte assignment through `([sp-21]:1 + [[sp-28]+8]) +0x0` writes `0x1`
  (`strb` at 0xcc2dc). The word at `sp -0x24` receives `[sp-36]+16` in the loop (`str` at 0xcc2b4).
  Device enable policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcc1e0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xcc1e8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xcc1ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc200 | load | 4 | `abs 0xcc388` | `-` |
| `ldr` at 0xcc204 | load | 4 | `abs 0x12da28` | `-` |
| `str` at 0xcc20c | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xcc210 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcc214 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcc218 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xcc224 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcc228 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcc230 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xcc238 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcc240 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcc250 | store | 4 | `sp -0x28` | `ret(0xca3d0)` |
| `strb` at 0xcc258 | store | 1 | `sp -0x29` | `0x0` |
| `ldrb` at 0xcc25c | load | 1 | `sp -0x29` | `-` |
| `ldr` at 0xcc260 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcc26c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcc270 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcc274 | load | 1 | `[sp-36] +0x1` | `-` |
| `ldrb` at 0xcc278 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xcc284 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcc288 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcc28c | load | 1 | `[sp-36] +0x2` | `-` |
| `ldrb` at 0xcc290 | load | 1 | `[sp-32] +0x1` | `-` |
| `ldrb` at 0xcc29c | load | 1 | `sp -0x29` | `-` |
| `ldr` at 0xcc2a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc2ac | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcc2b4 | store | 4 | `sp -0x24` | `[sp-36]+16` |
| `ldrb` at 0xcc2b8 | load | 1 | `sp -0x29` | `-` |
| `strb` at 0xcc2c0 | store | 1 | `sp -0x29` | `?` |
| `ldr` at 0xcc2c8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcc2cc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcc2d0 | load | 4 | `[sp-28] +0x8` | `-` |
| `strb` at 0xcc2dc | store | 1 | `([sp-21]:1 + [[sp-28]+8]) +0x0` | `0x1` |
| `ldr` at 0xcc2e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc2ec | load | 4 | `abs 0xcc38c` | `-` |
| `ldr` at 0xcc2f0 | load | 4 | `abs 0xcc390` | `-` |
| `str` at 0xcc32c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xcc330 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc338 | load | 4 | `abs 0xcc38c` | `-` |
| `ldr` at 0xcc33c | load | 4 | `abs 0xcc390` | `-` |
| `ldr` at 0xcc348 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcc358 | load | 4 | `abs 0xcc394` | `-` |
| `str` at 0xcc35c | store | 4 | `(unknown)` | `0x120984` |
| `str` at 0xcc360 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldrb` at 0xcc364 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xcc368 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0xcc370 | load | 4 | `abs 0xcc398` | `-` |
| `ldm` at 0xcc384 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcc384 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcc384 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcc248 | `(unknown callee) 0xca3d0` | `r0` | `sp-36` | `?` | `sp-36` |
| `bl` at 0xcc2a8 | `(unknown callee) 0xc990c` | `[sp-20]` | `?` | `[sp-32]` | `?` |
| `bl` at 0xcc2f4 | `(unknown callee) 0x48da8` | `r0` | `0x0` | `0x12093c` | `0x120944` |
| `bl` at 0xcc300 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xcc340 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0x12093c` | `0x120944` |
| `bl` at 0xcc350 | `(unknown callee) 0x19808` | `[sp-48]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xcc37c | `(unknown callee) 0x200a0` | `0x6` | `0x1208e8` | `0x7e` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xcc220 | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` |
| `bge` at 0xcc268 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xcc280 | `[[sp-36]+1]:1 != [[sp-32]]:1` | `[[sp-36]+1]:1 == [[sp-32]]:1` |
| `bne` at 0xcc298 | `[[sp-36]+2]:1 != [[sp-32]+1]:1` | `[[sp-36]+2]:1 == [[sp-32]+1]:1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcc384 | `r0` | `?` |

## `vsc055_IntGet`

- `vsc055_IntGet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcd058 to 0xcd1a0.
  Established. The byte through entry `r2 +0x0` receives zero (`strb` at 0xcd07c). The unnamed
  `0xbb5c4` call carries unknown, `[[sp-24]+2]:1`, `[sp-30]:1`, `sp-31` (`bl` at 0xcd0fc). A zero
  result takes `0xcd120`, while nonzero falls through (`beq` at 0xcd110). Interrupt status decoding
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcd068 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcd06c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcd070 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xcd074 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xcd07c | store | 1 | `r2 +0x0` | `0x0` |
| `strb` at 0xcd084 | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0xcd088 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xcd094 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcd098 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldrb` at 0xcd0a4 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0xcd0bc | store | 1 | `sp -0x1e` | `?` |
| `ldr` at 0xcd0c0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcd0c4 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xcd0c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcd0dc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcd0e0 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0xcd0e4 | load | 1 | `sp -0x1e` | `-` |
| `str` at 0xcd104 | store | 4 | `sp -0x24` | `ret(0xbb5c4)` |
| `ldr` at 0xcd108 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcd114 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcd118 | store | 4 | `sp -0x28` | `ret(0xbb5c4)` |
| `ldrb` at 0xcd120 | load | 1 | `sp -0x1f` | `-` |
| `str` at 0xcd128 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xcd12c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcd13c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcd144 | load | 4 | `abs 0xcd19c` | `-` |
| `ldr` at 0xcd158 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcd15c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcd160 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0xcd16c | load | 1 | `[sp-28] +0x0` | `-` |
| `strb` at 0xcd174 | store | 1 | `[sp-28] +0x0` | `?` |
| `ldrb` at 0xcd178 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0xcd180 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `str` at 0xcd18c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xcd190 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xcd198 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcd198 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcd198 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcd0d0 | `(unknown callee) 0xbac78` | `[sp-20]` | `[[sp-24]+1]:1` | `?` | `[[sp-24]+1]:1` |
| `bl` at 0xcd0fc | `(unknown callee) 0xbb5c4` | `?` | `[[sp-24]+2]:1` | `[sp-30]:1` | `sp-31` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhi` at 0xcd090 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0xcd110 | `ret(0xbb5c4) == 0x0` | `ret(0xbb5c4) != 0x0` |
| `bhi` at 0xcd134 | `[sp-44] > 0xe (unsigned, 32-bit)` | `[sp-44] <= 0xe (unsigned, 32-bit)` |
| `bne` at 0xcd150 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcd198 | `r0` | `?` |

## `vsc055_Present`

- `vsc055_Present`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcceb4 to 0xccf3c.
  Established. The signed condition `(r1 & 0xff) < [[sp-28]+4]` takes `0xccf14`, while greater or
  equal values fall through (`blt` at 0xccf04). The byte through `((r1 & 0xff) + [[sp-28]+8]) +0x0`
  is read on an included path (`ldrb` at 0xccf24). The absolute word at `0x12da28` is read (`ldr` at 0xccee8). Device presence flag meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xccec4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xccecc | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xcced0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xccee4 | load | 4 | `abs 0xccf38` | `-` |
| `ldr` at 0xccee8 | load | 4 | `abs 0x12da28` | `-` |
| `str` at 0xccef0 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xccef4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xccef8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xccefc | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xccf0c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xccf14 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xccf18 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xccf1c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0xccf24 | load | 1 | `((r1 & 0xff) + [[sp-28]+8]) +0x0` | `-` |
| `str` at 0xccf28 | store | 4 | `sp -0x20` | `[((r1 & 0xff) + [[sp-28]+8])]:1` |
| `ldr` at 0xccf2c | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xccf34 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xccf34 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xccf34 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xccf04 | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xccf34 | `r0` | `?` |

## `ioExpSoftResetData`

- `ioExpSoftResetData`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc9efc to 0xca10c.
  Established. A zero `ret(0xbacf4)` takes `0xc9f84`, while nonzero falls through (`beq` at 0xc9f48). The unnamed `0xcb894` call carries `[sp-24]`, `[sp-28]`, `ret(0xcb2c0)+2`,
  `ret(0xcb2c0)+2` (`bl` at 0xc9ff4). An included byte assignment through `[sp-32] +0x1` writes an
  unknown value (`strb` at 0xca0c8). Soft-reset data recovery remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc9f0c | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xc9f10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc9f24 | load | 4 | `abs 0xca0f0` | `-` |
| `ldr` at 0xc9f28 | load | 4 | `abs 0x29733c` | `-` |
| `str` at 0xc9f30 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xc9f3c | store | 4 | `sp -0x28` | `ret(0xbacf4)` |
| `ldr` at 0xc9f40 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc9f4c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc9f58 | load | 4 | `abs 0xca0f4` | `-` |
| `str` at 0xc9f5c | store | 4 | `sp -0x3c` | `0x1205a8` |
| `str` at 0xc9f60 | store | 4 | `sp -0x38` | `ret(0x1f8e0)` |
| `ldr` at 0xc9f68 | load | 4 | `abs 0xca0f8` | `-` |
| `ldr` at 0xc9f6c | load | 4 | `abs 0xca0fc` | `-` |
| `str` at 0xc9f7c | store | 4 | `sp -0x34` | `?` |
| `strb` at 0xc9f88 | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0xc9f8c | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xc9f90 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc9f94 | load | 4 | `[sp-44] +0x4` | `-` |
| `ldrb` at 0xc9fa0 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xc9fa4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc9fb4 | store | 4 | `sp -0x1c` | `ret(0xcb198)` |
| `ldrb` at 0xc9fb8 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xc9fbc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc9fcc | store | 4 | `sp -0x20` | `ret(0xcb2c0)` |
| `ldr` at 0xc9fd0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc9fd4 | load | 1 | `ret(0xcb2c0) +0x0` | `-` |
| `ldr` at 0xc9fe0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc9fe8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc9fec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc9ffc | store | 4 | `sp -0x28` | `ret(0xcb894)` |
| `ldr` at 0xca000 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xca00c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca014 | load | 4 | `abs 0xca100` | `-` |
| `ldr` at 0xca018 | load | 4 | `abs 0xca104` | `-` |
| `str` at 0xca054 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xca058 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca060 | load | 4 | `abs 0xca100` | `-` |
| `ldr` at 0xca064 | load | 4 | `abs 0xca104` | `-` |
| `ldr` at 0xca070 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xca080 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xca08c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xca090 | load | 4 | `abs 0xca108` | `-` |
| `ldr` at 0xca098 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xca0a8 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xca0b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xca0b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xca0b8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xca0bc | load | 1 | `[sp-32] +0x2` | `-` |
| `ldrb` at 0xca0c0 | load | 1 | `[sp-28] +0x4` | `-` |
| `strb` at 0xca0c8 | store | 1 | `[sp-32] +0x1` | `?` |
| `ldrb` at 0xca0cc | load | 1 | `sp -0x21` | `-` |
| `strb` at 0xca0d4 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `str` at 0xca0e0 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xca0e4 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0xca0ec | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xca0ec | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xca0ec | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xca0ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc9f34 | `(unknown callee) 0xbacf4` | `r0` | `r1` | `?` | `?` |
| `bl` at 0xc9f50 | `(unknown callee) 0x1f8e0` | `ret(0xbacf4)` | `?` | `?` | `ret(0xbacf4)` |
| `bl` at 0xc9f74 | `(unknown callee) 0x200a0` | `0x1` | `0x12047c` | `0x13d` | `0x0` |
| `bl` at 0xc9fac | `(unknown callee) 0xcb198` | `[sp-24]` | `?` | `[[sp-44]+4]` | `?` |
| `bl` at 0xc9fc4 | `(unknown callee) 0xcb2c0` | `[sp-24]` | `[sp-33]:1` | `?` | `[sp-33]:1` |
| `bl` at 0xc9ff4 | `(unknown callee) 0xcb894` | `[sp-24]` | `[sp-28]` | `ret(0xcb2c0)+2` | `ret(0xcb2c0)+2` |
| `bl` at 0xca01c | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x1204d0` | `0x1204d8` |
| `bl` at 0xca028 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xca068 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x1204d0` | `0x1204d8` |
| `bl` at 0xca078 | `(unknown callee) 0x19808` | `[sp-48]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xca084 | `(unknown callee) 0x1f8e0` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xca0a0 | `(unknown callee) 0x1e958` | `0x1205d4` | `ret(0x19808)` | `[[sp-28]+12]` | `ret(0x1f8e0)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc9f48 | `ret(0xbacf4) == 0x0` | `ret(0xbacf4) != 0x0` |
| `bge` at 0xc9f9c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xc9fdc | `[ret(0xcb2c0)]:1 == 0x0` | `[ret(0xcb2c0)]:1 != 0x0` |
| `beq` at 0xca008 | `ret(0xcb894) == 0x0` | `ret(0xcb894) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xca0ec | `r0` | `?` |

## `ioExpSoftResetInt`

- `ioExpSoftResetInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca10c to 0xca3d0.
  Established. An included byte assignment through `[sp-28] +0x5` writes `0x5` (`strb` at 0xca244).
  The indirect call carries `[sp-20]`, `[sp-24]`, `[sp-28]+3`, `[sp-28]+3` (`mov` at 0xca280). The
  unnamed `0xca878` call carries `[sp-20]`, `[sp-29]:1`, `0xff`, `[sp-29]:1` (`bl` at 0xca388).
  Soft-reset interrupt policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca11c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xca120 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca134 | load | 4 | `abs 0xca3b0` | `-` |
| `ldr` at 0xca138 | load | 4 | `abs 0x29733c` | `-` |
| `str` at 0xca140 | store | 4 | `sp -0x28` | `?` |
| `strb` at 0xca148 | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0xca14c | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xca150 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xca154 | load | 4 | `[sp-40] +0x4` | `-` |
| `ldrb` at 0xca160 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xca164 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xca174 | store | 4 | `sp -0x18` | `ret(0xcb198)` |
| `ldrb` at 0xca178 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xca17c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xca18c | store | 4 | `sp -0x1c` | `ret(0xcb2c0)` |
| `ldr` at 0xca190 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xca194 | load | 1 | `ret(0xcb2c0) +0x0` | `-` |
| `ldr` at 0xca1a0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xca1a4 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldrls` at 0xca1b0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xca228 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xca22c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xca230 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xca234 | load | 1 | `ret(0xcb2c0) +0x3` | `-` |
| `strb` at 0xca238 | store | 1 | `ret(0xcb2c0) +0x4` | `([ret(0xcb2c0)+3]:1 & 0xff)` |
| `ldr` at 0xca23c | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xca244 | store | 1 | `[sp-28] +0x5` | `0x5` |
| `ldr` at 0xca248 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xca24c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xca250 | load | 1 | `[sp-24] +0x7` | `-` |
| `strb` at 0xca254 | store | 1 | `[sp-28] +0x3` | `([[sp-24]+7]:1 & 0xff)` |
| `ldr` at 0xca258 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca264 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xca26c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca270 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca274 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xca294 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xca29c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xca2a0 | load | 1 | `[sp-28] +0x3` | `-` |
| `ldr` at 0xca2a4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xca2a8 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xca2bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca2c4 | load | 4 | `abs 0xca3c4` | `-` |
| `ldr` at 0xca2c8 | load | 4 | `abs 0xca3c8` | `-` |
| `str` at 0xca304 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xca308 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca310 | load | 4 | `abs 0xca3c4` | `-` |
| `ldr` at 0xca314 | load | 4 | `abs 0xca3c8` | `-` |
| `ldr` at 0xca320 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xca330 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca334 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xca338 | load | 1 | `[sp-28] +0x3` | `-` |
| `ldr` at 0xca33c | load | 4 | `abs 0xca3cc` | `-` |
| `ldr` at 0xca344 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xca350 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xca358 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xca35c | load | 1 | `[sp-28] +0x3` | `-` |
| `ldr` at 0xca368 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca36c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0xca378 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xca37c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xca38c | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0xca394 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `str` at 0xca3a0 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xca3a4 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0xca3ac | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xca3ac | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xca3ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xca16c | `(unknown callee) 0xcb198` | `?` | `?` | `[[sp-40]+4]` | `?` |
| `bl` at 0xca184 | `(unknown callee) 0xcb2c0` | `[sp-20]` | `[sp-29]:1` | `?` | `[sp-29]:1` |
| `mov` at 0xca280 | `(unknown callee) (unknown target)` | `[sp-20]` | `[sp-24]` | `[sp-28]+3` | `[sp-28]+3` |
| `bl` at 0xca2cc | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0x1204d0` | `0x1204d8` |
| `bl` at 0xca2d8 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xca318 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0x1204d0` | `0x1204d8` |
| `bl` at 0xca328 | `(unknown callee) 0x19808` | `[sp-44]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xca348 | `(unknown callee) 0x1e958` | `0x1205fc` | `ret(0x19808)` | `[[sp-24]+12]` | `[[sp-28]+3]:1` |
| `bl` at 0xca388 | `(unknown callee) 0xca878` | `[sp-20]` | `[sp-29]:1` | `0xff` | `[sp-29]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xca15c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xca19c | `[ret(0xcb2c0)]:1 == 0x0` | `[ret(0xcb2c0)]:1 != 0x0` |
| `ldrls` at 0xca1b0 | `[[sp-24]]:1-1 <= 0x3 (unsigned, 32-bit)` | `[[sp-24]]:1-1 > 0x3 (unsigned, 32-bit)` |
| `beq` at 0xca260 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0xca28c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0xca2b8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xca364 | `[[sp-28]+3]:1 == 0x0` | `[[sp-28]+3]:1 != 0x0` |
| `bne` at 0xca374 | `[[sp-24]+8] != 0x0` | `[[sp-24]+8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xca3ac | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xca1b0).

## `ioExpGetCfgIndex`

- `ioExpGetCfgIndex`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcb198 to 0xcb2c0.
  Established. The signed condition `(r1 & 0xff) < [[sp-28]+4]` takes `0xcb288`, while greater or
  equal values fall through (`blt` at 0xcb1e8). The word through `[sp-28] +0x0` is read (`ldr` at 0xcb294). The unnamed `0x200a0` call carries `0x9`, `0x12047c`, `0x357`, `0x0` (`bl` at 0xcb284).
  Configuration index lookup remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcb1a8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xcb1b0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xcb1b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcb1c8 | load | 4 | `abs 0xcb2a8` | `-` |
| `ldr` at 0xcb1cc | load | 4 | `abs 0x29733c` | `-` |
| `str` at 0xcb1d4 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xcb1d8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcb1dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcb1e0 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xcb1ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcb1f4 | load | 4 | `abs 0xcb2ac` | `-` |
| `ldr` at 0xcb1f8 | load | 4 | `abs 0xcb2b0` | `-` |
| `str` at 0xcb234 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcb238 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcb240 | load | 4 | `abs 0xcb2ac` | `-` |
| `ldr` at 0xcb244 | load | 4 | `abs 0xcb2b0` | `-` |
| `ldr` at 0xcb250 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcb260 | load | 4 | `abs 0xcb2b4` | `-` |
| `str` at 0xcb264 | store | 4 | `(unknown)` | `0x120740` |
| `str` at 0xcb268 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldrb` at 0xcb26c | load | 1 | `sp -0x15` | `-` |
| `str` at 0xcb270 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0xcb278 | load | 4 | `abs 0xcb2b8` | `-` |
| `ldr` at 0xcb27c | load | 4 | `abs 0xcb2bc` | `-` |
| `ldr` at 0xcb288 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcb28c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcb294 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldm` at 0xcb2a4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcb2a4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcb2a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcb1fc | `(unknown callee) 0x48da8` | `r0` | `0x0` | `0x1204d0` | `0x1204d8` |
| `bl` at 0xcb208 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xcb248 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0x1204d0` | `0x1204d8` |
| `bl` at 0xcb258 | `(unknown callee) 0x19808` | `[sp-32]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xcb284 | `(unknown callee) 0x200a0` | `0x9` | `0x12047c` | `0x357` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xcb1e8 | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcb2a4 | `r0` | `?` |

## `ioExpGetValIndex`

- `ioExpGetValIndex`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcb2c0 to 0xcb3f4.
  Established. The signed condition `(r1 & 0xff) < [[sp-28]+4]` takes `0xcb3b0`, while greater or
  equal values fall through (`blt` at 0xcb310). The word through `[sp-28] +0x8` is read (`ldr` at 0xcb3c8). The unnamed `0x200a0` call carries `0x9`, `0x12047c`, `0x365`, `0x0` (`bl` at 0xcb3ac).
  State index lookup semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcb2d0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xcb2d8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xcb2dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcb2f0 | load | 4 | `abs 0xcb3dc` | `-` |
| `ldr` at 0xcb2f4 | load | 4 | `abs 0x29733c` | `-` |
| `str` at 0xcb2fc | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xcb300 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcb304 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcb308 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xcb314 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcb31c | load | 4 | `abs 0xcb3e0` | `-` |
| `ldr` at 0xcb320 | load | 4 | `abs 0xcb3e4` | `-` |
| `str` at 0xcb35c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcb360 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcb368 | load | 4 | `abs 0xcb3e0` | `-` |
| `ldr` at 0xcb36c | load | 4 | `abs 0xcb3e4` | `-` |
| `ldr` at 0xcb378 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcb388 | load | 4 | `abs 0xcb3e8` | `-` |
| `str` at 0xcb38c | store | 4 | `(unknown)` | `0x120740` |
| `str` at 0xcb390 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldrb` at 0xcb394 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xcb398 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0xcb3a0 | load | 4 | `abs 0xcb3ec` | `-` |
| `ldr` at 0xcb3a4 | load | 4 | `abs 0xcb3f0` | `-` |
| `ldr` at 0xcb3b0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcb3b4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcb3c8 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldm` at 0xcb3d8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcb3d8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcb3d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcb324 | `(unknown callee) 0x48da8` | `r0` | `0x0` | `0x1204d0` | `0x1204d8` |
| `bl` at 0xcb330 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xcb370 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0x1204d0` | `0x1204d8` |
| `bl` at 0xcb380 | `(unknown callee) 0x19808` | `[sp-32]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xcb3ac | `(unknown callee) 0x200a0` | `0x9` | `0x12047c` | `0x365` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xcb310 | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcb3d8 | `r0` | `?` |
