# ARM expander and PCI-X control paths in 1.44.0

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
`ioExpWriteExp` and `ioExpReadExp` each have one unresolved computed transfer.
Their tables cover known edges only. The other thirteen routines report none.
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/expander-pcix-control-facts.txt) retain all tool sections.

## `ioExpWriteExp`

- `ioExpWriteExp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcb3f4 to 0xcb894.
  Established. Included byte writes target `[sp-28] +0x1` with unknown values (`strb` at 0xcb5c4, `strb` at 0xcb5dc).
  The unnamed `0x48da8` call carries entry `r0`, `0x0`, `0x1204d0`, `0x1204d8` (`bl` at 0xcb44c).
  Expander write dispatch and mask interpretation remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcb404 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcb408 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcb40c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xcb410 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xcb414 | store | 1 | `sp -0x1d` | `(r3 & 0xff)` |
| `strb` at 0xcb41c | store | 1 | `sp -0x1e` | `([sp] & 0xff)` |
| `ldr` at 0xcb420 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcb424 | load | 1 | `r1 +0x4` | `-` |
| `ldrb` at 0xcb428 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xcb43c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcb444 | load | 4 | `abs 0xcb870` | `-` |
| `ldr` at 0xcb448 | load | 4 | `abs 0xcb874` | `-` |
| `str` at 0xcb484 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xcb488 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcb490 | load | 4 | `abs 0xcb870` | `-` |
| `ldr` at 0xcb494 | load | 4 | `abs 0xcb874` | `-` |
| `ldr` at 0xcb4a0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcb4b0 | load | 4 | `abs 0xcb878` | `-` |
| `str` at 0xcb4b4 | store | 4 | `(unknown)` | `0x120764` |
| `str` at 0xcb4b8 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0xcb4bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcb4c0 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xcb4c4 | store | 4 | `(unknown)` | `[[sp-24]+12]` |
| `ldrb` at 0xcb4c8 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0xcb4cc | store | 4 | `(unknown)` | `[sp-29]:1` |
| `ldr` at 0xcb4d4 | load | 4 | `abs 0xcb87c` | `-` |
| `ldr` at 0xcb4d8 | load | 4 | `abs 0xcb880` | `-` |
| `ldrb` at 0xcb4e4 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0xcb4ec | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0xcb4fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcb504 | load | 4 | `abs 0xcb870` | `-` |
| `ldr` at 0xcb508 | load | 4 | `abs 0xcb874` | `-` |
| `str` at 0xcb544 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xcb548 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcb550 | load | 4 | `abs 0xcb870` | `-` |
| `ldr` at 0xcb554 | load | 4 | `abs 0xcb874` | `-` |
| `ldr` at 0xcb560 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcb570 | load | 4 | `abs 0xcb884` | `-` |
| `str` at 0xcb574 | store | 4 | `(unknown)` | `0x12079c` |
| `str` at 0xcb578 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0xcb57c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcb580 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xcb584 | store | 4 | `(unknown)` | `[[sp-24]+12]` |
| `ldrb` at 0xcb588 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0xcb58c | store | 4 | `(unknown)` | `[sp-29]:1` |
| `ldrb` at 0xcb590 | load | 1 | `sp -0x1e` | `-` |
| `str` at 0xcb594 | store | 4 | `(unknown)` | `[sp-30]:1` |
| `ldr` at 0xcb59c | load | 4 | `abs 0xcb87c` | `-` |
| `ldr` at 0xcb5a0 | load | 4 | `abs 0xcb888` | `-` |
| `ldr` at 0xcb5ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcb5b0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcb5b4 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0xcb5bc | load | 1 | `[sp-28] +0x1` | `-` |
| `strb` at 0xcb5c4 | store | 1 | `[sp-28] +0x1` | `?` |
| `ldr` at 0xcb5c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcb5cc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcb5d0 | load | 1 | `[sp-28] +0x1` | `-` |
| `ldrb` at 0xcb5d4 | load | 1 | `sp -0x1e` | `-` |
| `strb` at 0xcb5dc | store | 1 | `[sp-28] +0x1` | `?` |
| `ldr` at 0xcb5e0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcb5e4 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldrls` at 0xcb5ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcb7b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcb7b8 | load | 4 | `abs 0xcb870` | `-` |
| `ldr` at 0xcb7bc | load | 4 | `abs 0xcb874` | `-` |
| `str` at 0xcb7f8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xcb7fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcb804 | load | 4 | `abs 0xcb870` | `-` |
| `ldr` at 0xcb808 | load | 4 | `abs 0xcb874` | `-` |
| `ldr` at 0xcb814 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcb824 | load | 4 | `abs 0xcb88c` | `-` |
| `str` at 0xcb828 | store | 4 | `(unknown)` | `0x1207d4` |
| `ldr` at 0xcb82c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcb830 | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0xcb834 | store | 4 | `(unknown)` | `[[sp-24]]:1` |
| `str` at 0xcb838 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0xcb83c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcb840 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xcb844 | store | 4 | `(unknown)` | `[[sp-24]+12]` |
| `ldr` at 0xcb84c | load | 4 | `abs 0xcb87c` | `-` |
| `ldr` at 0xcb850 | load | 4 | `abs 0xcb890` | `-` |
| `str` at 0xcb860 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xcb864 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0xcb86c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcb86c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcb86c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcb44c | `(unknown callee) 0x48da8` | `r0` | `0x0` | `0x1204d0` | `0x1204d8` |
| `bl` at 0xcb458 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xcb498 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0x1204d0` | `0x1204d8` |
| `bl` at 0xcb4a8 | `(unknown callee) 0x19808` | `[sp-40]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xcb4e0 | `(unknown callee) 0x200a0` | `0x9` | `0x12047c` | `0x37d` | `0x0` |
| `bl` at 0xcb50c | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0x1204d0` | `0x1204d8` |
| `bl` at 0xcb518 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xcb558 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0x1204d0` | `0x1204d8` |
| `bl` at 0xcb568 | `(unknown callee) 0x19808` | `[sp-40]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xcb5a8 | `(unknown callee) 0x200a0` | `0x9` | `0x12047c` | `0x381` | `0x0` |
| `bl` at 0xcb7c0 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0x1204d0` | `0x1204d8` |
| `bl` at 0xcb7cc | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xcb80c | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0x1204d0` | `0x1204d8` |
| `bl` at 0xcb81c | `(unknown callee) 0x19808` | `[sp-40]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xcb858 | `(unknown callee) 0x200a0` | `0x1` | `0x12047c` | `0x3b1` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xcb438 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xcb4f8 | `? == 0x0` | `? != 0x0` |
| `ldrls` at 0xcb5ec | `[[sp-24]]:1 <= 0x4 (unsigned, 32-bit)` | `[[sp-24]]:1 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcb86c | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xcb5ec).

## `ioExpReadExp`

- `ioExpReadExp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcb894 to 0xcbab0.
  Established. The unnamed `0x48da8` call carries entry `r0`, `0x0`, `0x1204d0`, `0x1204d8` (`bl` at 0xcb9ec).
  A word store clears `sp -0x24` on an included path (`str` at 0xcba8c).
  Read dispatch target and expander transport roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcb8a4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcb8a8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcb8ac | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xcb8b0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcb8b4 | load | 1 | `r1 +0x0` | `-` |
| `ldrls` at 0xcb8bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcb9dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcb9e4 | load | 4 | `abs 0xcba9c` | `-` |
| `ldr` at 0xcb9e8 | load | 4 | `abs 0xcbaa0` | `-` |
| `str` at 0xcba24 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcba28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcba30 | load | 4 | `abs 0xcba9c` | `-` |
| `ldr` at 0xcba34 | load | 4 | `abs 0xcbaa0` | `-` |
| `ldr` at 0xcba40 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcba50 | load | 4 | `abs 0xcbaa4` | `-` |
| `str` at 0xcba54 | store | 4 | `(unknown)` | `0x1207d4` |
| `ldr` at 0xcba58 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcba5c | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0xcba60 | store | 4 | `(unknown)` | `[[sp-24]]:1` |
| `str` at 0xcba64 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0xcba68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcba6c | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xcba70 | store | 4 | `(unknown)` | `[[sp-24]+12]` |
| `ldr` at 0xcba78 | load | 4 | `abs 0xcbaa8` | `-` |
| `ldr` at 0xcba7c | load | 4 | `abs 0xcbaac` | `-` |
| `str` at 0xcba8c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xcba90 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xcba98 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcba98 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcba98 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcb9ec | `(unknown callee) 0x48da8` | `r0` | `0x0` | `0x1204d0` | `0x1204d8` |
| `bl` at 0xcb9f8 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xcba38 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0x1204d0` | `0x1204d8` |
| `bl` at 0xcba48 | `(unknown callee) 0x19808` | `[sp-32]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xcba84 | `(unknown callee) 0x200a0` | `0x1` | `0x12047c` | `0x3d5` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xcb8bc | `[r1]:1 <= 0x4 (unsigned, 32-bit)` | `[r1]:1 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcba98 | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xcb8bc).

## `ioExpIntHandler`

- `ioExpIntHandler`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcad04 to 0xcb034.
  Established. A word store assigns `[[sp-32]+8]+1` to `[sp-32] +0x8` (`str` at 0xcae54).
  The indirect call carries `[sp-24]`, `[sp-33]:1`, unknown, `[[sp-32]+2]:1` (`mov` at 0xcae8c).
  Zero `ret(0xcb894)` takes `0xcae44`, while nonzero falls through (`beq` at 0xcad84).
  Expander interrupt callback policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcad14 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xcad18 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xcad1c | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0xcad20 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xcad24 | store | 1 | `sp -0x21` | `(r3 & 0xff)` |
| `strb` at 0xcad2c | store | 1 | `sp -0x22` | `([sp] & 0xff)` |
| `ldr` at 0xcad30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcad34 | load | 4 | `r1 +0x8` | `-` |
| `ldr` at 0xcad40 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcad44 | load | 1 | `r2 +0x3` | `-` |
| `ldrb` at 0xcad48 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xcad5c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcad64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcad68 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcad78 | store | 4 | `sp -0x28` | `ret(0xcb894)` |
| `ldr` at 0xcad7c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcad88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcad90 | load | 4 | `abs 0xcb018` | `-` |
| `ldr` at 0xcad94 | load | 4 | `abs 0xcb01c` | `-` |
| `str` at 0xcadd0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xcadd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcaddc | load | 4 | `abs 0xcb018` | `-` |
| `ldr` at 0xcade0 | load | 4 | `abs 0xcb01c` | `-` |
| `ldr` at 0xcadec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcadfc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcae08 | load | 4 | `abs 0xcb020` | `-` |
| `str` at 0xcae0c | store | 4 | `(unknown)` | `0x12067c` |
| `str` at 0xcae10 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0xcae14 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcae18 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xcae1c | store | 4 | `(unknown)` | `[[sp-28]+12]` |
| `str` at 0xcae20 | store | 4 | `(unknown)` | `ret(0x1fc1c)` |
| `ldr` at 0xcae28 | load | 4 | `abs 0xcb024` | `-` |
| `str` at 0xcae3c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xcae44 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcae48 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcae4c | load | 4 | `[sp-32] +0x8` | `-` |
| `str` at 0xcae54 | store | 4 | `[sp-32] +0x8` | `[[sp-32]+8]+1` |
| `ldr` at 0xcae58 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcae5c | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xcae60 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcae64 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldrb` at 0xcae68 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xcae74 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcae78 | load | 1 | `[sp-32] +0x2` | `-` |
| `ldr` at 0xcae7c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xcae80 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcae94 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xcae9c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcaea0 | load | 1 | `r2 +0x4` | `-` |
| `ldrb` at 0xcaea4 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xcaeb8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcaebc | load | 1 | `r2 +0x5` | `-` |
| `ldr` at 0xcaec8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcaecc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcaed0 | load | 1 | `r2 +0x5` | `-` |
| `strb` at 0xcaed8 | store | 1 | `r2 +0x5` | `([r2+5]:1-1 & 0xff)` |
| `str` at 0xcaee0 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xcaee8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcaef0 | load | 4 | `abs 0xcb018` | `-` |
| `ldr` at 0xcaef4 | load | 4 | `abs 0xcb01c` | `-` |
| `str` at 0xcaf30 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xcaf34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcaf3c | load | 4 | `abs 0xcb018` | `-` |
| `ldr` at 0xcaf40 | load | 4 | `abs 0xcb01c` | `-` |
| `ldr` at 0xcaf4c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcaf5c | load | 4 | `abs 0xcb028` | `-` |
| `str` at 0xcaf60 | store | 4 | `(unknown)` | `0x1206a0` |
| `str` at 0xcaf64 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0xcaf68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcaf6c | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xcaf70 | store | 4 | `(unknown)` | `[[sp-28]+12]` |
| `ldrb` at 0xcaf74 | load | 1 | `sp -0x22` | `-` |
| `str` at 0xcaf78 | store | 4 | `(unknown)` | `[sp-34]:1` |
| `ldr` at 0xcaf7c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcaf80 | load | 1 | `[sp-32] +0x3` | `-` |
| `str` at 0xcaf84 | store | 4 | `(unknown)` | `[[sp-32]+3]:1` |
| `ldr` at 0xcaf88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcaf8c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xcaf98 | load | 4 | `abs 0xcb02c` | `-` |
| `str` at 0xcaf9c | store | 4 | `sp -0x34` | `0x1206e4` |
| `ldr` at 0xcafa4 | load | 4 | `abs 0xcb030` | `-` |
| `str` at 0xcafa8 | store | 4 | `sp -0x34` | `0x1206e8` |
| `ldr` at 0xcafac | load | 4 | `sp -0x34` | `-` |
| `str` at 0xcafb0 | store | 4 | `(unknown)` | `[sp-52]` |
| `ldr` at 0xcafb8 | load | 4 | `abs 0xcb024` | `-` |
| `ldrb` at 0xcafc8 | load | 1 | `sp -0x21` | `-` |
| `str` at 0xcafcc | store | 4 | `sp -0x38` | `[sp-33]:1` |
| `ldr` at 0xcafd0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcafd4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0xcafe0 | load | 1 | `sp -0x22` | `-` |
| `str` at 0xcafe4 | store | 4 | `sp -0x3c` | `[sp-34]:1` |
| `str` at 0xcaff0 | store | 4 | `sp -0x3c` | `0xff` |
| `ldr` at 0xcaff4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcaff8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xcaffc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xcb008 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xcb00c | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0xcb014 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xcb014 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcb014 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcb014 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcad70 | `(unknown callee) 0xcb894` | `r0` | `r1` | `r2+2` | `r2+2` |
| `bl` at 0xcad98 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x1204d0` | `0x1204d8` |
| `bl` at 0xcada4 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xcade4 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x1204d0` | `0x1204d8` |
| `bl` at 0xcadf4 | `(unknown callee) 0x19808` | `[sp-44]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xcae00 | `(unknown callee) 0x1fc1c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xcae34 | `(unknown callee) 0x200a0` | `0x3` | `0x12047c` | `0x308` | `0x0` |
| `mov` at 0xcae8c | `(unknown callee) (unknown target)` | `[sp-24]` | `[sp-33]:1` | `?` | `[[sp-32]+2]:1` |
| `bl` at 0xcaef8 | `(unknown callee) 0x48da8` | `r0` | `0x0` | `0x1204d0` | `0x1204d8` |
| `bl` at 0xcaf04 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xcaf44 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x1204d0` | `0x1204d8` |
| `bl` at 0xcaf54 | `(unknown callee) 0x19808` | `[sp-44]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xcafc4 | `(unknown callee) 0x200a0` | `0x1` | `0x12047c` | `0x324` | `0x0` |
| `bl` at 0xcb000 | `(unknown callee) 0xca878` | `[sp-24]` | `[sp-33]:1` | `[sp-60]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xcad3c | `[r1+8] == 0x0` | `[r1+8] != 0x0` |
| `beq` at 0xcad58 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xcad84 | `ret(0xcb894) == 0x0` | `ret(0xcb894) != 0x0` |
| `beq` at 0xcaeb4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xcaec4 | `[r2+5]:1 == 0x0` | `[r2+5]:1 != 0x0` |
| `beq` at 0xcaf94 | `[[sp-28]+8] == 0x0` | `[[sp-28]+8] != 0x0` |
| `beq` at 0xcafdc | `[[sp-28]+8] == 0x0` | `[[sp-28]+8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcb014 | `r0` | `?` |

## `ioExpWriteValue`

- `ioExpWriteValue`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca7a0 to 0xca7e8.
  Established. The unnamed `0xcb2c0` call carries entry `r0`, `(r1 & 0xff)`, entry `r2`, `(r1 & 0xff)` (`bl` at 0xca7c8).
  A byte read fetches `ret(0xcb2c0) +0x1` (`ldrb` at 0xca7d8).
  The integer return candidate is `[ret(0xcb2c0)+1]:1` (`ldm` at 0xca7e4).
  Expander value record meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca7b0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xca7b8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xca7bc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xca7c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xca7d0 | store | 4 | `sp -0x1c` | `ret(0xcb2c0)` |
| `ldr` at 0xca7d4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xca7d8 | load | 1 | `ret(0xcb2c0) +0x1` | `-` |
| `ldm` at 0xca7e4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xca7e4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xca7e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xca7c8 | `(unknown callee) 0xcb2c0` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xca7e4 | `r0` | `[ret(0xcb2c0)+1]:1` |

## `vrmInt`

- `vrmInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x25914 to 0x25e54.
  Established. The unnamed `0xca70c` call carries `0x0`, unknown, `sp-97`, unknown (`bl` at 0x259d4).
  Nonzero `ret(0x25248)` takes `0x25ba0`, while zero falls through (`bne` at 0x25a7c).
  An included unnamed `0x6f93c` call carries `0x0`, `0x3500`, `0x1a4`, `0xee860` (`bl` at 0x25c40).
  Regulator interrupt event interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x25924 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x25930 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x25938 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x25940 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldr` at 0x25944 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25958 | load | 4 | `abs 0x25e08` | `-` |
| `ldr` at 0x2595c | load | 4 | `abs 0x132068` | `-` |
| `str` at 0x25964 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x2596c | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0x25970 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x25974 | load | 1 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x25978 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x25984 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x25988 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x25990 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x25998 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0x2599c | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x259a0 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x259a4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x259b0 | load | 4 | `abs 0x25e0c` | `-` |
| `ldr` at 0x259b4 | load | 4 | `abs 0x132f14` | `-` |
| `ldrb` at 0x259c4 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x259dc | store | 1 | `sp -0x62` | `(ret(0xca70c) & 0xff)` |
| `ldrb` at 0x259e0 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x259e4 | load | 1 | `sp -0x61` | `-` |
| `ldrb` at 0x259f0 | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0x259f4 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x25a0c | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x25a10 | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0x25a14 | load | 1 | `sp -0x61` | `-` |
| `ldr` at 0x25a18 | load | 4 | `abs 0x25e10` | `-` |
| `ldrb` at 0x25a2c | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x25a30 | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0x25a34 | load | 1 | `sp -0x61` | `-` |
| `ldr` at 0x25a38 | load | 4 | `abs 0x25e14` | `-` |
| `ldr` at 0x25a48 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x25a4c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x25a50 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x25a64 | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x25a68 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x25a80 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x25a84 | load | 1 | `[sp-104] +0x5` | `-` |
| `ldr` at 0x25a94 | load | 4 | `abs 0x25e18` | `-` |
| `ldr` at 0x25a98 | load | 4 | `abs 0x13206c` | `-` |
| `ldr` at 0x25aa4 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x25aa8 | load | 1 | `[sp-104] +0x1` | `-` |
| `ldr` at 0x25aac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25ac0 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x25ac4 | load | 1 | `[sp-104] +0x2` | `-` |
| `str` at 0x25ac8 | store | 4 | `sp -0x70` | `[[sp-104]+2]:1` |
| `ldr` at 0x25ad0 | load | 4 | `abs 0x25e1c` | `-` |
| `ldr` at 0x25ad4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25ae4 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x25ae8 | load | 1 | `[sp-104] +0x1` | `-` |
| `ldr` at 0x25aec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25b00 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x25b04 | load | 1 | `[sp-104] +0x2` | `-` |
| `ldr` at 0x25b0c | load | 4 | `abs 0x25e20` | `-` |
| `str` at 0x25b20 | store | 4 | `sp -0x70` | `0x1` |
| `str` at 0x25b28 | store | 4 | `sp -0x6c` | `0x10` |
| `ldr` at 0x25b30 | load | 4 | `abs 0x25e24` | `-` |
| `ldr` at 0x25b34 | load | 4 | `abs 0x25e28` | `-` |
| `ldr` at 0x25b40 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x25b44 | load | 1 | `[sp-104] +0x1` | `-` |
| `ldr` at 0x25b48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25b5c | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x25b60 | load | 1 | `[sp-104] +0x2` | `-` |
| `ldr` at 0x25b68 | load | 4 | `abs 0x25e2c` | `-` |
| `str` at 0x25b7c | store | 4 | `sp -0x70` | `0x1` |
| `str` at 0x25b84 | store | 4 | `sp -0x6c` | `0x10` |
| `ldr` at 0x25b8c | load | 4 | `abs 0x25e30` | `-` |
| `ldr` at 0x25b90 | load | 4 | `abs 0x25e34` | `-` |
| `ldr` at 0x25ba0 | load | 4 | `abs 0x25e0c` | `-` |
| `ldr` at 0x25ba4 | load | 4 | `abs 0x132f14` | `-` |
| `ldr` at 0x25bb4 | load | 4 | `abs 0x25e38` | `-` |
| `ldr` at 0x25bc0 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x25bc4 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x25bc8 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x25bdc | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x25be0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x25be4 | load | 4 | `sp -0x14` | `-` |
| `ldc` at 0x25bf4 | load | unknown | `abs 0x25e04` | `-` |
| `ldrb` at 0x25c04 | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x25c0c | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x25c10 | load | 1 | `[sp-104] +0x4` | `-` |
| `str` at 0x25c24 | store | 4 | `sp -0x70` | `0x1` |
| `str` at 0x25c2c | store | 4 | `sp -0x6c` | `0x10` |
| `ldr` at 0x25c3c | load | 4 | `abs 0x25e3c` | `-` |
| `ldrb` at 0x25c48 | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x25c4c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x25c64 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x25c68 | load | 1 | `[sp-104] +0x5` | `-` |
| `ldr` at 0x25c78 | load | 4 | `abs 0x25e18` | `-` |
| `ldr` at 0x25c7c | load | 4 | `abs 0x13206c` | `-` |
| `ldr` at 0x25c88 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x25c8c | load | 1 | `[sp-104] +0x1` | `-` |
| `ldr` at 0x25c90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25ca4 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x25ca8 | load | 1 | `[sp-104] +0x2` | `-` |
| `str` at 0x25cac | store | 4 | `sp -0x70` | `[[sp-104]+2]:1` |
| `ldr` at 0x25cb4 | load | 4 | `abs 0x25e40` | `-` |
| `ldr` at 0x25cb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25cc8 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x25ccc | load | 1 | `[sp-104] +0x1` | `-` |
| `ldr` at 0x25cd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25ce4 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x25ce8 | load | 1 | `[sp-104] +0x2` | `-` |
| `ldr` at 0x25cf0 | load | 4 | `abs 0x25e44` | `-` |
| `str` at 0x25d04 | store | 4 | `sp -0x70` | `0x1` |
| `str` at 0x25d0c | store | 4 | `sp -0x6c` | `0x10` |
| `ldr` at 0x25d14 | load | 4 | `abs 0x25e24` | `-` |
| `ldr` at 0x25d18 | load | 4 | `abs 0x25e28` | `-` |
| `ldr` at 0x25d24 | load | 4 | `abs 0x25e18` | `-` |
| `ldr` at 0x25d28 | load | 4 | `abs 0x13206c` | `-` |
| `ldr` at 0x25d34 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x25d38 | load | 1 | `[sp-104] +0x1` | `-` |
| `ldr` at 0x25d3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25d50 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x25d54 | load | 1 | `[sp-104] +0x2` | `-` |
| `str` at 0x25d58 | store | 4 | `sp -0x70` | `[[sp-104]+2]:1` |
| `ldr` at 0x25d60 | load | 4 | `abs 0x25e48` | `-` |
| `ldr` at 0x25d64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25d74 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x25d78 | load | 1 | `[sp-104] +0x1` | `-` |
| `ldr` at 0x25d7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25d90 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x25d94 | load | 1 | `[sp-104] +0x2` | `-` |
| `ldr` at 0x25d9c | load | 4 | `abs 0x25e4c` | `-` |
| `str` at 0x25db0 | store | 4 | `sp -0x70` | `0x1` |
| `str` at 0x25db8 | store | 4 | `sp -0x6c` | `0x10` |
| `ldr` at 0x25dc0 | load | 4 | `abs 0x25e24` | `-` |
| `ldr` at 0x25dc4 | load | 4 | `abs 0x25e28` | `-` |
| `ldr` at 0x25dd0 | load | 4 | `abs 0x25e0c` | `-` |
| `ldr` at 0x25dd4 | load | 4 | `abs 0x132f14` | `-` |
| `ldr` at 0x25de4 | load | 4 | `abs 0x25e50` | `-` |
| `ldr` at 0x25dec | load | 4 | `sp -0x60` | `-` |
| `str` at 0x25df4 | store | 4 | `sp -0x60` | `?` |
| `ldm` at 0x25e00 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x25e00 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x25e00 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x259d4 | `(unknown callee) 0xca70c` | `0x0` | `?` | `sp-97` | `?` |
| `bl` at 0x25a24 | `(unknown callee) 0x1e958` | `0xee73c` | `[sp-22]:1` | `[sp-23]:1` | `[sp-97]:1` |
| `bl` at 0x25a44 | `(unknown callee) 0x1e958` | `0xee778` | `[sp-22]:1` | `[sp-23]:1` | `[sp-97]:1` |
| `bl` at 0x25a70 | `(unknown callee) 0x25248` | `?` | `?` | `?` | `?` |
| `bl` at 0x25ab4 | `(unknown callee) 0x39b0c` | `[sp-20]` | `[[sp-104]+1]:1` | `?` | `[[sp-104]+1]:1` |
| `bl` at 0x25adc | `(unknown callee) 0x197a8` | `sp-92` | `0xee7b4` | `[sp-20]` | `ret(0x39b0c)` |
| `bl` at 0x25af4 | `(unknown callee) 0x39b0c` | `[sp-20]` | `[[sp-104]+1]:1` | `?` | `[[sp-104]+1]:1` |
| `bl` at 0x25b14 | `(unknown callee) 0x197a8` | `sp-92` | `0xee7e4` | `ret(0x39b0c)` | `[[sp-104]+2]:1` |
| `bl` at 0x25b38 | `(unknown callee) 0x6f93c` | `0x0` | `0x3402` | `0x1a3` | `sp-92` |
| `bl` at 0x25b50 | `(unknown callee) 0x39b0c` | `[sp-20]` | `[[sp-104]+1]:1` | `?` | `[[sp-104]+1]:1` |
| `bl` at 0x25b70 | `(unknown callee) 0x197a8` | `sp-92` | `0xee80c` | `ret(0x39b0c)` | `[[sp-104]+2]:1` |
| `bl` at 0x25b98 | `(unknown callee) 0x6f93c` | `0x0` | `0x3401` | `0x1a2` | `sp-92` |
| `bl` at 0x25bb8 | `(unknown callee) 0x1e958` | `0xee830` | `?` | `?` | `?` |
| `bl` at 0x25bec | `(unknown callee) 0x39ddc` | `?` | `?` | `?` | `?` |
| `bl` at 0x25c40 | `(unknown callee) 0x6f93c` | `0x0` | `0x3500` | `0x1a4` | `0xee860` |
| `bl` at 0x25c54 | `(unknown callee) 0x251c0` | `[sp-104]` | `[sp-23]:1` | `?` | `[sp-23]:1` |
| `bl` at 0x25c98 | `(unknown callee) 0x39b0c` | `[sp-20]` | `[[sp-104]+1]:1` | `?` | `[[sp-104]+1]:1` |
| `bl` at 0x25cc0 | `(unknown callee) 0x197a8` | `sp-92` | `0xee878` | `[sp-20]` | `ret(0x39b0c)` |
| `bl` at 0x25cd8 | `(unknown callee) 0x39b0c` | `[sp-20]` | `[[sp-104]+1]:1` | `?` | `[[sp-104]+1]:1` |
| `bl` at 0x25cf8 | `(unknown callee) 0x197a8` | `sp-92` | `0xee8a8` | `ret(0x39b0c)` | `[[sp-104]+2]:1` |
| `bl` at 0x25d1c | `(unknown callee) 0x6f93c` | `0x0` | `0x3402` | `0x1a3` | `sp-92` |
| `bl` at 0x25d44 | `(unknown callee) 0x39b0c` | `[sp-20]` | `[[sp-104]+1]:1` | `?` | `[[sp-104]+1]:1` |
| `bl` at 0x25d6c | `(unknown callee) 0x197a8` | `sp-92` | `0xee8d0` | `[sp-20]` | `ret(0x39b0c)` |
| `bl` at 0x25d84 | `(unknown callee) 0x39b0c` | `[sp-20]` | `[[sp-104]+1]:1` | `?` | `[[sp-104]+1]:1` |
| `bl` at 0x25da4 | `(unknown callee) 0x197a8` | `sp-92` | `0xee8f8` | `ret(0x39b0c)` | `[[sp-104]+2]:1` |
| `bl` at 0x25dc8 | `(unknown callee) 0x6f93c` | `0x0` | `0x3402` | `0x1a3` | `sp-92` |
| `bl` at 0x25de8 | `(unknown callee) 0x1e958` | `0xee918` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x25980 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x259ac | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x259c0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x259ec | `[sp-22]:1 == [sp-97]:1` | `[sp-22]:1 != [sp-97]:1` |
| `bne` at 0x25a04 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x25a60 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x25a7c | `ret(0x25248) != 0x0` | `ret(0x25248) == 0x0` |
| `beq` at 0x25a90 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x25aa0 | `[0x13206c] <= 0x1 (signed, 32-bit)` | `[0x13206c] > 0x1 (signed, 32-bit)` |
| `beq` at 0x25bb0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x25bd8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x25bfc | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x25c1c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x25c60 | `ret(0x251c0) != 0x0` | `ret(0x251c0) == 0x0` |
| `beq` at 0x25c74 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x25c84 | `[0x13206c] <= 0x1 (signed, 32-bit)` | `[0x13206c] > 0x1 (signed, 32-bit)` |
| `ble` at 0x25d30 | `[0x13206c] <= 0x1 (signed, 32-bit)` | `[0x13206c] > 0x1 (signed, 32-bit)` |
| `beq` at 0x25de0 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x25e00 | `r0` | `?` |

## `smpcmd_ioexp`

- `smpcmd_ioexp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa505c to 0xa523c.
  Established. The unnamed `0xca70c` call carries `0x0`, unknown, `sp-45`, unknown (`bl` at 0xa5180);
  its zero result takes `0xa51b8`, while nonzero falls through (`beq` at 0xa5194).
  A word store assigns `ret(0x1943c)` to `[sp-28] +0x0` (`str` at 0xa50ac).
  Expander command result formatting remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa506c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xa5070 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xa5080 | store | 4 | `sp -0x2c` | `ret(0xca434)` |
| `ldr` at 0xa5084 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa5088 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xa50ac | store | 4 | `[sp-28] +0x0` | `ret(0x1943c)` |
| `ldr` at 0xa50b0 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xa50b4 | store | 4 | `sp -0x28` | `[[sp-28]]` |
| `ldr` at 0xa50b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa50bc | load | 4 | `abs 0xa5228` | `-` |
| `ldr` at 0xa50c4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa50d4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa50dc | store | 4 | `sp -0x28` | `([sp-40] + ret(0x19538))` |
| `str` at 0xa50e4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xa50e8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa50ec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa50f8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa5110 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa5114 | load | 4 | `abs 0xa522c` | `-` |
| `ldr` at 0xa5118 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa5120 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa5130 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa5138 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x19538))` |
| `ldr` at 0xa513c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa515c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa5160 | load | 4 | `abs 0xa5230` | `-` |
| `ldr` at 0xa516c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa5188 | store | 4 | `sp -0x20` | `ret(0xca70c)` |
| `ldr` at 0xa518c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa5198 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa51a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa51a8 | load | 4 | `abs 0xa5234` | `-` |
| `ldr` at 0xa51b8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xa51d4 | load | 1 | `sp -0x2d` | `-` |
| `ldr` at 0xa51d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa51dc | load | 4 | `abs 0xa5238` | `-` |
| `ldr` at 0xa51ec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa51fc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa5204 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x19538))` |
| `ldr` at 0xa5208 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa5210 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldm` at 0xa5224 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xa5224 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa5224 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa5224 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa5078 | `(unknown callee) 0xca434` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0xa50a4 | `(unknown callee) 0x1943c` | `?` | `?` | `ret(0xca434)` | `?` |
| `bl` at 0xa50c0 | `(unknown callee) 0x197a8` | `[[sp-28]]` | `0x113694` | `?` | `[[sp-28]]` |
| `bl` at 0xa50c8 | `(unknown callee) 0x19538` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xa5108 | `(unknown callee) 0xcb12c` | `0x0` | `?` | `?` | `?` |
| `bl` at 0xa511c | `(unknown callee) 0x197a8` | `[sp-40]` | `0x1136e0` | `[sp-36]` | `ret(0xcb12c)` |
| `bl` at 0xa5124 | `(unknown callee) 0x19538` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xa514c | `(unknown callee) 0xca484` | `0x0` | `?` | `ret(0x19538)` | `?` |
| `bl` at 0xa5164 | `(unknown callee) 0x197a8` | `[sp-40]` | `0x1136f0` | `?` | `ret(0xca484)` |
| `bl` at 0xa5180 | `(unknown callee) 0xca70c` | `0x0` | `?` | `sp-45` | `?` |
| `bl` at 0xa519c | `(unknown callee) 0x1fc1c` | `ret(0xca70c)` | `?` | `?` | `ret(0xca70c)` |
| `bl` at 0xa51b0 | `(unknown callee) 0x197a8` | `[sp-40]` | `0x113700` | `ret(0x1fc1c)` | `ret(0x1fc1c)` |
| `bl` at 0xa51c8 | `(unknown callee) 0xca830` | `0x0` | `?` | `?` | `?` |
| `bl` at 0xa51e8 | `(unknown callee) 0x197a8` | `[sp-40]` | `0x11371c` | `[sp-45]:1` | `?` |
| `bl` at 0xa51f0 | `(unknown callee) 0x19538` | `[sp-40]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xa50f4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xa5158 | `ret(0xca484) != 0x0` | `ret(0xca484) == 0x0` |
| `beq` at 0xa5194 | `ret(0xca70c) == 0x0` | `ret(0xca70c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa5224 | `r0` | `0x0` |

## `pcixASICInit`

- `pcixASICInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4f81c to 0x4f91c.
  Established. Included word stores write entry `r1`, `[sp-28]`, `[sp-32]`, `[sp]`, `[sp+4]` to unknown effective targets (`str` at 0x4f85c, `str` at 0x4f888, `str` at 0x4f8b4, `str` at 0x4f8e0, `str` at 0x4f90c).
  The integer return candidate is `0x10` (`ldm` at 0x4f914).
  PCI-X ASIC record roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4f82c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4f830 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x4f834 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x4f838 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x4f83c | load | 4 | `abs 0x4f918` | `-` |
| `ldr` at 0x4f840 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4f858 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4f85c | store | 4 | `(unknown)` | `r1` |
| `ldr` at 0x4f860 | load | 4 | `abs 0x4f918` | `-` |
| `ldr` at 0x4f864 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4f884 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4f888 | store | 4 | `(unknown)` | `[sp-28]` |
| `ldr` at 0x4f88c | load | 4 | `abs 0x4f918` | `-` |
| `ldr` at 0x4f890 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4f8b0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4f8b4 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0x4f8b8 | load | 4 | `abs 0x4f918` | `-` |
| `ldr` at 0x4f8bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4f8dc | load | 4 | `sp +0x0` | `-` |
| `str` at 0x4f8e0 | store | 4 | `(unknown)` | `[sp]` |
| `ldr` at 0x4f8e4 | load | 4 | `abs 0x4f918` | `-` |
| `ldr` at 0x4f8e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4f908 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x4f90c | store | 4 | `(unknown)` | `[sp+4]` |
| `ldm` at 0x4f914 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4f914 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4f914 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4f914 | `r0` | `0x10` |

## `pcixGetBusCfg`

- `pcixGetBusCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4f9b0 to 0x4fa44.
  Established. Nonzero `[0x1324a0]` takes `0x4f9e4`, while zero falls through (`bne` at 0x4f9d4).
  An included word assignment saves `[[sp-28]+4]` at `[sp-24] +0x0` (`str` at 0x4fa20).
  The word at `sp -0x20` receives `[[sp-28]+8]` on an included path (`str` at 0x4fa2c).
  Bus configuration fields remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4f9c0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4f9c4 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x4f9c8 | load | 4 | `abs 0x4fa3c` | `-` |
| `ldr` at 0x4f9cc | load | 4 | `abs 0x1324a0` | `-` |
| `str` at 0x4f9dc | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x4f9e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4f9f8 | load | 4 | `abs 0x4fa40` | `-` |
| `ldr` at 0x4f9fc | load | 4 | `abs 0x13249c` | `-` |
| `str` at 0x4fa04 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x4fa08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4fa14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4fa18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4fa1c | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0x4fa20 | store | 4 | `[sp-24] +0x0` | `[[sp-28]+4]` |
| `ldr` at 0x4fa24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4fa28 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0x4fa2c | store | 4 | `sp -0x20` | `[[sp-28]+8]` |
| `ldr` at 0x4fa30 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x4fa38 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4fa38 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4fa38 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x4f9d4 | `[0x1324a0] != 0x0` | `[0x1324a0] == 0x0` |
| `beq` at 0x4fa10 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4fa38 | `r0` | `?` |

## `pcixSetBusSpeed`

- `pcixSetBusSpeed`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x51440 to 0x51558.
  Established. The unnamed `0x53864` call carries `[sp-20]`, unknown, unknown, unknown (`bl` at 0x51500);
  its zero result takes `0x5152c`, while nonzero falls through (`beq` at 0x51514).
  An included word store preserves `ret(0x53864)` at `sp -0x1c` (`str` at 0x51528).
  PCI-X speed application policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x51450 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x51458 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x5145c | load | 4 | `abs 0x51550` | `-` |
| `ldr` at 0x51460 | load | 4 | `abs 0x1324a0` | `-` |
| `str` at 0x51470 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x51478 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5148c | load | 4 | `abs 0x51554` | `-` |
| `ldr` at 0x51490 | load | 4 | `abs 0x13249c` | `-` |
| `str` at 0x51498 | store | 4 | `sp -0x18` | `?` |
| `str` at 0x514a0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x514a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x514a8 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x514ac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x514b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x514bc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x514c4 | load | 4 | `[sp-24] +0x14` | `-` |
| `ldrb` at 0x514cc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x514d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x514dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x514f0 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x514f8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x51508 | store | 4 | `sp -0x20` | `ret(0x53864)` |
| `ldr` at 0x5150c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x51518 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x51524 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x51528 | store | 4 | `sp -0x1c` | `ret(0x53864)` |
| `ldr` at 0x5152c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x51534 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x5153c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x51540 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x51544 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x5154c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x5154c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x5154c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x51500 | `(unknown callee) 0x53864` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x51468 | `[0x1324a0] != 0x0` | `[0x1324a0] == 0x0` |
| `ble` at 0x514b4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x514d4 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x51514 | `ret(0x53864) == 0x0` | `ret(0x53864) != 0x0` |
| `bne` at 0x51520 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x5154c | `r0` | `?` |

## `pcixBusPopulation`

- `pcixBusPopulation`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x516e0 to 0x5183c.
  Established. The unnamed `0x53024` call carries `[sp-20]`, `[sp-21]:1`, unknown, `[sp-21]:1` (`bl` at 0x51750);
  its nonzero result takes `0x51774`, while zero falls through (`bne` at 0x51764).
  An included word store increments `sp -0x2c` with `[sp-44]+1` (`str` at 0x5180c).
  Bus population count interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x516f0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x516f8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `str` at 0x51700 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x51704 | load | 4 | `abs 0x51834` | `-` |
| `ldr` at 0x51708 | load | 4 | `abs 0x1324a0` | `-` |
| `str` at 0x51718 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x51720 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51734 | load | 4 | `abs 0x51838` | `-` |
| `ldr` at 0x51738 | load | 4 | `abs 0x13249c` | `-` |
| `str` at 0x51740 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x51744 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x51748 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x51758 | store | 4 | `sp -0x20` | `ret(0x53024)` |
| `ldr` at 0x5175c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5176c | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x51774 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x51778 | load | 1 | `ret(0x53024) +0x1` | `-` |
| `str` at 0x5177c | store | 4 | `sp -0x34` | `[ret(0x53024)+1]:1` |
| `ldr` at 0x51780 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x51784 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x51788 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x5178c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x51794 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x517a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x517a4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x517ac | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x517b8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x517c8 | store | 4 | `sp -0x30` | `ret(0x53658)` |
| `ldr` at 0x517cc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x517d8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x517dc | store | 4 | `sp -0x38` | `[sp-44]` |
| `ldrb` at 0x517f0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x51804 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x5180c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x51810 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x51818 | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `ldr` at 0x51820 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x51824 | store | 4 | `sp -0x38` | `[sp-44]` |
| `ldr` at 0x51828 | load | 4 | `sp -0x38` | `-` |
| `ldm` at 0x51830 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x51830 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x51830 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x51750 | `(unknown callee) 0x53024` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x517c0 | `(unknown callee) 0x53658` | `[sp-20]` | `?` | `sp-40` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x51710 | `[0x1324a0] != 0x0` | `[0x1324a0] == 0x0` |
| `bne` at 0x51764 | `ret(0x53024) != 0x0` | `ret(0x53024) == 0x0` |
| `ble` at 0x5179c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x517d4 | `ret(0x53658) == 0x0` | `ret(0x53658) != 0x0` |
| `beq` at 0x51800 | `? == 0x3` | `? != 0x3` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x51830 | `r0` | `?` |

## `pcixBusPresent`

- `pcixBusPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x519a8 to 0x51a28.
  Established. A nonzero `[0x1324a0]` takes `0x519d8`, while zero falls through (`bne` at 0x519d0).
  The included byte store writes `0x1` to an unknown effective target (`strb` at 0x51a14);
  its integer return candidate retains entry `r0` (`ldm` at 0x51a1c).
  Bus presence record indexing remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x519b8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x519c0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x519c4 | load | 4 | `abs 0x51a20` | `-` |
| `ldr` at 0x519c8 | load | 4 | `abs 0x1324a0` | `-` |
| `ldr` at 0x519d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x519ec | load | 4 | `abs 0x51a24` | `-` |
| `ldr` at 0x519f0 | load | 4 | `abs 0x13249c` | `-` |
| `str` at 0x519f8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x519fc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x51a00 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x51a08 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x51a14 | store | 1 | `(unknown)` | `0x1` |
| `ldm` at 0x51a1c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x51a1c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x51a1c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x519d0 | `[0x1324a0] != 0x0` | `[0x1324a0] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x51a1c | `r0` | `r0` |

## `pcixBusNotPresent`

- `pcixBusNotPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x51a28 to 0x51aa8.
  Established. The byte at `sp -0x15` receives `(r1 & 0xff)` (`strb` at 0x51a40).
  Nonzero `[0x1324a0]` takes `0x51a58`, while zero falls through (`bne` at 0x51a50).
  A byte store clears an unknown effective target (`strb` at 0x51a94).
  Bus absence field meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x51a38 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x51a40 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x51a44 | load | 4 | `abs 0x51aa0` | `-` |
| `ldr` at 0x51a48 | load | 4 | `abs 0x1324a0` | `-` |
| `ldr` at 0x51a58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51a6c | load | 4 | `abs 0x51aa4` | `-` |
| `ldr` at 0x51a70 | load | 4 | `abs 0x13249c` | `-` |
| `str` at 0x51a78 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x51a7c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x51a80 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x51a88 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x51a94 | store | 1 | `(unknown)` | `0x0` |
| `ldm` at 0x51a9c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x51a9c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x51a9c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x51a50 | `[0x1324a0] != 0x0` | `[0x1324a0] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x51a9c | `r0` | `r0` |

## `pcixBusIsPresent`

- `pcixBusIsPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x51aa8 to 0x51b64.
  Established. Included word reads fetch absolute `0x13249c` and `0x1324a0` (`ldr` at 0x51adc, `ldr` at 0x51aec).
  Nonzero `[0x1324a0]` takes `0x51b04`, while zero falls through (`bne` at 0x51af4).
  A subsequent byte read has an unknown effective target (`ldrb` at 0x51b3c).
  Bus presence result meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x51ab8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x51ac0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x51ac4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51ad8 | load | 4 | `abs 0x51b5c` | `-` |
| `ldr` at 0x51adc | load | 4 | `abs 0x13249c` | `-` |
| `str` at 0x51ae4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x51ae8 | load | 4 | `abs 0x51b60` | `-` |
| `ldr` at 0x51aec | load | 4 | `abs 0x1324a0` | `-` |
| `str` at 0x51afc | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x51b04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51b18 | load | 4 | `abs 0x51b5c` | `-` |
| `ldr` at 0x51b1c | load | 4 | `abs 0x13249c` | `-` |
| `str` at 0x51b24 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x51b28 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x51b2c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x51b34 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0x51b3c | load | 1 | `(unknown)` | `-` |
| `str` at 0x51b4c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x51b50 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x51b58 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x51b58 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x51b58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x51af4 | `[0x1324a0] != 0x0` | `[0x1324a0] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x51b58 | `r0` | `?` |

## `TIOActive`

- `TIOActive`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4ba54 to 0x4ba7c.
  Established. A byte load reads absolute `0x132494` (`ldrb` at 0x4ba6c).
  The integer return candidate is `[0x132494]:1` (`ldm` at 0x4ba74).
  TIO activity state interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4ba64 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x4ba68 | load | 4 | `abs 0x4ba78` | `-` |
| `ldrb` at 0x4ba6c | load | 1 | `abs 0x132494` | `-` |
| `ldm` at 0x4ba74 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x4ba74 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4ba74 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4ba74 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4ba74 | `r0` | `[0x132494]:1` |

## `TIOPresent`

- `TIOPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4be1c to 0x4be58.
  Established. The byte at `sp -0x11` receives `(r0 & 0xff)` (`strb` at 0x4be30).
  A word load fetches absolute `0x132498` (`ldr` at 0x4be40).
  The included byte store writes `0x1` to an unknown effective target (`strb` at 0x4be4c).
  TIO presence array semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4be30 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x4be34 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x4be3c | load | 4 | `abs 0x4be54` | `-` |
| `ldr` at 0x4be40 | load | 4 | `abs 0x132498` | `-` |
| `strb` at 0x4be4c | store | 1 | `(unknown)` | `0x1` |
| `ldm` at 0x4be50 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x4be50 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4be50 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4be50 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4be50 | `r0` | `r0` |
