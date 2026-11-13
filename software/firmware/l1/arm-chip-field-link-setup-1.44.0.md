# ARM chip fields and link setup in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package4/chip-field-link-setup-facts.txt) retain all tool sections.

## `nl4r_setup_llp`

- `nl4r_setup_llp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2a6d0 to 0x2ab3c.
  Established. A word assignment writes `0x190` to `sp -0x20` (`str` at 0x2a9b8).
  An included unnamed `0xdfc34` call carries `[sp-20]`, `[sp-24]+10`, `0x6`, `sp-32` (`bl` at 0x2a9d8).
  LLP field configuration effects beyond these arguments remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2a6e0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x2a6e4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x2a6e8 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x2a6ec | load | 4 | `abs 0x2ab34` | `-` |
| `ldr` at 0x2a6f0 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x2a6fc | load | 4 | `abs 0x2ab38` | `-` |
| `ldr` at 0x2a700 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a704 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a70c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2a71c | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x2a720 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a72c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a740 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a74c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a764 | store | 4 | `sp -0x20` | `0x18` |
| `ldr` at 0x2a768 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a774 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a788 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a794 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a7a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a7b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a7c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a7d4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a7f0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x2a7f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a800 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a814 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a820 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a838 | store | 4 | `sp -0x20` | `0xf` |
| `ldr` at 0x2a83c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a848 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a85c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a868 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a87c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a888 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a89c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a8a8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a8c0 | store | 4 | `sp -0x20` | `0x3f` |
| `ldr` at 0x2a8c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a8d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a8e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a8f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a904 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a910 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a924 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a930 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a948 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x2a94c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a958 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a96c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a978 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a990 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x2a994 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a9a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a9b8 | store | 4 | `sp -0x20` | `0x190` |
| `ldr` at 0x2a9bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a9c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a9dc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2a9ec | store | 4 | `sp -0x28` | `0x3` |
| `str` at 0x2a9f8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x2a9fc | load | 4 | `sp -0x28` | `-` |
| `str` at 0x2aa00 | store | 4 | `sp -0x20` | `[sp-40]` |
| `ldr` at 0x2aa04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2aa10 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2aa28 | store | 4 | `sp -0x20` | `0x2` |
| `ldr` at 0x2aa2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2aa38 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2aa50 | store | 4 | `sp -0x20` | `0x2` |
| `ldr` at 0x2aa54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2aa60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2aa74 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2aa84 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x2aa88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2aa94 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2aaac | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x2aab0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2aabc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2aad8 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x2aadc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2aae8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ab00 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x2ab04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2ab10 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x2ab30 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x2ab30 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x2ab30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2a708 | `(unknown callee) 0x221f0` | `0xf14cc` | `r0` | `r1` | `[0x29737c]` |
| `bl` at 0x2a73c | `(unknown callee) 0xdfc34` | `?` | `?` | `0x0` | `sp-32` |
| `bl` at 0x2a75c | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+10` | `0x3` | `sp-32` |
| `bl` at 0x2a784 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+10` | `0x1f` | `sp-32` |
| `bl` at 0x2a7a4 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+10` | `0x20` | `sp-32` |
| `bl` at 0x2a7c4 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+10` | `0x21` | `sp-32` |
| `bl` at 0x2a7e4 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+10` | `0x22` | `sp-32` |
| `bl` at 0x2a810 | `(unknown callee) 0xdfc34` | `?` | `?` | `0x0` | `sp-32` |
| `bl` at 0x2a830 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+10` | `0x3` | `sp-32` |
| `bl` at 0x2a858 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+10` | `0x1f` | `sp-32` |
| `bl` at 0x2a878 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+10` | `0x20` | `sp-32` |
| `bl` at 0x2a898 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+10` | `0x21` | `sp-32` |
| `bl` at 0x2a8b8 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+10` | `0x22` | `sp-32` |
| `bl` at 0x2a8e0 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+10` | `0x23` | `sp-32` |
| `bl` at 0x2a900 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+10` | `0x24` | `sp-32` |
| `bl` at 0x2a920 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+10` | `0x25` | `sp-32` |
| `bl` at 0x2a940 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+10` | `0x26` | `sp-32` |
| `bl` at 0x2a968 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+10` | `0x2` | `sp-32` |
| `bl` at 0x2a988 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+10` | `0x4` | `sp-32` |
| `bl` at 0x2a9b0 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+10` | `0x5` | `sp-32` |
| `bl` at 0x2a9d8 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+10` | `0x6` | `sp-32` |
| `bl` at 0x2aa20 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+18` | `0x2` | `sp-32` |
| `bl` at 0x2aa48 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+18` | `0x4` | `sp-32` |
| `bl` at 0x2aa70 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+18` | `0x5` | `sp-32` |
| `bl` at 0x2aaa4 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+26` | `0x0` | `sp-32` |
| `bl` at 0x2aacc | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+26` | `0x4` | `sp-32` |
| `bl` at 0x2aaf8 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+26` | `0x0` | `sp-32` |
| `bl` at 0x2ab20 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+26` | `0x4` | `sp-32` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x2a6f8 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `beq` at 0x2a714 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x2a9e4 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `beq` at 0x2aa7c | `[sp-28] == 0x0` | `[sp-28] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x2ab30 | `r0` | `0x0` |

## `JChipInit`

- `JChipInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdcfd0 to 0xdd46c.
  Established. An included word store places `ret(0x195d4)` at `[sp-36] +0x8` (`str` at 0xdd098).
  Byte stores clear `[sp-40] +0x20` and write an unknown value to `[sp-40] +0x21` (`strb` at 0xdd148, `strb` at 0xdd154).
  An indirect call with unknown target carries `[sp-40]`, unknown, unknown, `[[[sp-40]]+16]` (`mov` at 0xdd280).
  Chip initialization callback behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdcfe0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xdcfe4 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xdcfe8 | load | 4 | `abs 0xdd434` | `-` |
| `ldr` at 0xdcfec | load | 4 | `abs 0x2974b0` | `-` |
| `ldr` at 0xdcff8 | load | 4 | `abs 0xdd438` | `-` |
| `str` at 0xdcffc | store | 4 | `sp -0x50` | `0x122060` |
| `ldr` at 0xdd004 | load | 4 | `abs 0xdd43c` | `-` |
| `ldr` at 0xdd014 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdd020 | load | 4 | `abs 0xdd434` | `-` |
| `ldr` at 0xdd024 | load | 4 | `abs 0x2974b0` | `-` |
| `ldr` at 0xdd030 | load | 4 | `abs 0xdd440` | `-` |
| `str` at 0xdd034 | store | 4 | `sp -0x30` | `0xfffff7c7` |
| `ldr` at 0xdd03c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdd04c | load | 4 | `abs 0xdd444` | `-` |
| `ldr` at 0xdd050 | load | 4 | `abs 0x2974ac` | `-` |
| `str` at 0xdd058 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xdd05c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd060 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd064 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd068 | load | 4 | `[sp-36] +0xc` | `-` |
| `ldr` at 0xdd088 | load | 4 | `[sp-36] +0x8` | `-` |
| `str` at 0xdd098 | store | 4 | `[sp-36] +0x8` | `ret(0x195d4)` |
| `ldr` at 0xdd09c | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0xdd0a8 | load | 4 | `abs 0xdd448` | `-` |
| `str` at 0xdd0ac | store | 4 | `sp -0x30` | `0xfffff7cc` |
| `ldr` at 0xdd0b4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd0b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd0bc | load | 4 | `[sp-36] +0xc` | `-` |
| `ldr` at 0xdd0d8 | load | 4 | `[sp-36] +0x8` | `-` |
| `str` at 0xdd0e0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xdd0e4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd0e8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd0ec | load | 4 | `[sp-36] +0xc` | `-` |
| `str` at 0xdd0f4 | store | 4 | `[sp-36] +0xc` | `[[sp-36]+12]+1` |
| `ldr` at 0xdd0f8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdd0fc | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xdd110 | store | 1 | `[sp-40] +0x18` | `?` |
| `ldr` at 0xdd114 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdd118 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xdd130 | store | 1 | `[sp-40] +0x19` | `?` |
| `ldr` at 0xdd134 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdd138 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdd13c | store | 4 | `[sp-40] +0x1c` | `[sp-28]` |
| `ldr` at 0xdd140 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0xdd148 | store | 1 | `[sp-40] +0x20` | `0x0` |
| `ldr` at 0xdd14c | load | 4 | `sp -0x28` | `-` |
| `strb` at 0xdd154 | store | 1 | `[sp-40] +0x21` | `?` |
| `ldr` at 0xdd158 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdd15c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdd170 | store | 4 | `[sp-40] +0x0` | `ret(0xdef00)` |
| `ldr` at 0xdd174 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0xdd180 | load | 4 | `abs 0xdd44c` | `-` |
| `str` at 0xdd184 | store | 4 | `sp -0x50` | `0x1220a4` |
| `ldr` at 0xdd188 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdd194 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xdd19c | load | 4 | `abs 0xdd43c` | `-` |
| `ldr` at 0xdd1a0 | load | 4 | `abs 0xdd450` | `-` |
| `ldr` at 0xdd1ac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd1b0 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xdd1bc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd1c0 | load | 4 | `[sp-36] +0x4` | `-` |
| `ldr` at 0xdd1d0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xdd1d8 | store | 4 | `sp -0x34` | `[sp-40]+4` |
| `ldr` at 0xdd1dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd1e0 | load | 4 | `[sp-36] +0x4` | `-` |
| `str` at 0xdd1e4 | store | 4 | `sp -0x38` | `[[sp-36]+4]` |
| `ldr` at 0xdd1e8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd1ec | load | 4 | `[sp-36] +0x4` | `-` |
| `ldr` at 0xdd1f8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd1fc | load | 4 | `[sp-36] +0x0` | `-` |
| `str` at 0xdd200 | store | 4 | `sp -0x38` | `[[sp-36]]` |
| `ldr` at 0xdd204 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdd208 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0xdd20c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xdd210 | load | 1 | `[sp-40] +0x18` | `-` |
| `str` at 0xdd214 | store | 4 | `sp -0x50` | `[[sp-40]+24]:1` |
| `ldr` at 0xdd218 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdd21c | load | 4 | `abs 0xdd454` | `-` |
| `ldr` at 0xdd220 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xdd224 | load | 4 | `[[sp-40]] +0x8` | `-` |
| `ldr` at 0xdd230 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdd238 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdd23c | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0xdd240 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xdd244 | load | 1 | `[sp-40] +0x18` | `-` |
| `ldr` at 0xdd24c | load | 4 | `abs 0xdd458` | `-` |
| `ldr` at 0xdd250 | load | 4 | `[[sp-40]] +0x8` | `-` |
| `ldr` at 0xdd258 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdd25c | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0xdd260 | load | 4 | `[[sp-40]] +0x10` | `-` |
| `ldr` at 0xdd26c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdd270 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0xdd274 | load | 4 | `[[sp-40]] +0x10` | `-` |
| `ldr` at 0xdd278 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdd290 | load | 4 | `abs 0xdd45c` | `-` |
| `str` at 0xdd294 | store | 4 | `sp -0x50` | `0x1220e0` |
| `ldr` at 0xdd298 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdd2a4 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xdd2ac | load | 4 | `abs 0xdd43c` | `-` |
| `ldr` at 0xdd2b0 | load | 4 | `abs 0xdd460` | `-` |
| `str` at 0xdd2c0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xdd2c4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd2c8 | load | 4 | `[sp-36] +0xc` | `-` |
| `ldr` at 0xdd2d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdd2dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd2e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdd2fc | load | 4 | `[sp-36] +0x8` | `-` |
| `str` at 0xdd304 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xdd308 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdd30c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xdd310 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xdd314 | load | 1 | `[sp-40] +0x19` | `-` |
| `ldr` at 0xdd320 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdd324 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdd328 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdd32c | load | 4 | `[sp-40] +0x1c` | `-` |
| `ldr` at 0xdd338 | load | 4 | `abs 0xdd464` | `-` |
| `str` at 0xdd33c | store | 4 | `sp -0x50` | `0x122118` |
| `ldr` at 0xdd340 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xdd344 | load | 1 | `[sp-40] +0x19` | `-` |
| `str` at 0xdd348 | store | 4 | `sp -0x4c` | `[[sp-40]+25]:1` |
| `ldr` at 0xdd34c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdd350 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xdd354 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdd358 | load | 4 | `[sp-40] +0x1c` | `-` |
| `str` at 0xdd35c | store | 4 | `sp -0x44` | `[[sp-40]+28]` |
| `ldr` at 0xdd360 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdd364 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdd368 | load | 4 | `(unknown)` | `-` |
| `str` at 0xdd36c | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xdd370 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdd374 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0xdd378 | load | 4 | `[[sp-40]] +0x0` | `-` |
| `str` at 0xdd37c | store | 4 | `sp -0x3c` | `[[[sp-40]]]` |
| `ldr` at 0xdd384 | load | 4 | `abs 0xdd43c` | `-` |
| `ldr` at 0xdd388 | load | 4 | `abs 0xdd468` | `-` |
| `ldr` at 0xdd394 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdd398 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xdd39c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xdd3a0 | load | 1 | `[sp-40] +0x19` | `-` |
| `ldr` at 0xdd3ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdd3b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdd3b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdd3b8 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0xdd3bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdd3c0 | load | 4 | `[[sp-40]] +0x0` | `-` |
| `ldr` at 0xdd3cc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdd3d0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdd3d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdd3d8 | load | 4 | `[sp-40] +0x1c` | `-` |
| `ldr` at 0xdd3e4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdd3e8 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xdd3ec | load | 1 | `[sp-40] +0x20` | `-` |
| `strb` at 0xdd3f4 | store | 1 | `[sp-40] +0x20` | `([[sp-40]+32]:1+1 & 0xff)` |
| `ldr` at 0xdd3fc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdd400 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xdd404 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xdd40c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xdd410 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdd418 | store | 4 | `sp -0x20` | `?` |
| `str` at 0xdd424 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xdd428 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0xdd430 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xdd430 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdd430 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdd430 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdd010 | `(unknown callee) 0x200a0` | `0x9` | `0x12208c` | `0xfc` | `0x0` |
| `bl` at 0xdd090 | `(unknown callee) 0x195d4` | `[[sp-36]+8]` | `?` | `[[sp-36]+12]` | `?` |
| `bl` at 0xdd16c | `(unknown callee) 0xdef00` | `?` | `?` | `[sp-40]` | `?` |
| `bl` at 0xdd1a8 | `(unknown callee) 0x200a0` | `0x9` | `0x12208c` | `0x113` | `0x0` |
| `bl` at 0xdd228 | `(unknown callee) 0x197a8` | `[sp-40]+4` | `0x1220d0` | `?` | `[[[sp-40]]+8]` |
| `bl` at 0xdd254 | `(unknown callee) 0x197a8` | `[sp-40]+4` | `0x1220d8` | `[[[sp-40]]+8]` | `[[sp-40]+24]:1` |
| `mov` at 0xdd280 | `(unknown callee) (unknown target)` | `[sp-40]` | `?` | `?` | `[[[sp-40]]+16]` |
| `bl` at 0xdd2b8 | `(unknown callee) 0x200a0` | `0x9` | `0x12208c` | `0x121` | `0x0` |
| `bl` at 0xdd390 | `(unknown callee) 0x200a0` | `0x9` | `0x12208c` | `0x132` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdcff4 | `[0x2974b0] != 0x0` | `[0x2974b0] == 0x0` |
| `blt` at 0xdd02c | `? < [0x2974b0] (signed, 32-bit)` | `? >= [0x2974b0] (signed, 32-bit)` |
| `bne` at 0xdd0a4 | `[[sp-36]+8] != 0x0` | `[[sp-36]+8] == 0x0` |
| `bne` at 0xdd17c | `[[sp-40]] != 0x0` | `[[sp-40]] == 0x0` |
| `bne` at 0xdd1b8 | `[[sp-36]] != 0x0` | `[[sp-36]] == 0x0` |
| `bne` at 0xdd1c8 | `[[sp-36]+4] != 0x0` | `[[sp-36]+4] == 0x0` |
| `bne` at 0xdd1f4 | `[[sp-36]+4] != 0x0` | `[[sp-36]+4] == 0x0` |
| `beq` at 0xdd268 | `[[[sp-40]]+16] == 0x0` | `[[[sp-40]]+16] != 0x0` |
| `bge` at 0xdd28c | `ret(unknown) >= 0x0 (signed, 32-bit)` | `ret(unknown) < 0x0 (signed, 32-bit)` |
| `ble` at 0xdd2d8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xdd31c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xdd334 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xdd3a8 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xdd3c8 | `Z == 0` | `not (Z == 0)` |
| `bhs` at 0xdd3e0 | `C == 1` | `not (C == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdd430 | `r0` | `?` |

## `JChipUpdatePort`

- `JChipUpdatePort`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdd46c to 0xdd828.
  Established. The signed comparison entry `r0 < [0x2974b0]` takes `0xdd4a0`, while greater or equal falls through (`blt` at 0xdd490).
  An unnamed `0xdab04` call carries `[[sp-52]+25]:1`, unknown, `[[sp-52]+32]:1`, `[[[sp-52]]]` (`bl` at 0xdd774);
  its truncated result `(ret(0xdab04) & 0xff)` is stored to `[sp-52] +0x21` (`strb` at 0xdd77c).
  Port selection encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdd47c | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xdd480 | load | 4 | `abs 0xdd80c` | `-` |
| `ldr` at 0xdd484 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdd488 | load | 4 | `abs 0x2974b0` | `-` |
| `ldr` at 0xdd494 | load | 4 | `abs 0xdd810` | `-` |
| `str` at 0xdd498 | store | 4 | `sp -0x44` | `0xfffff7c5` |
| `ldr` at 0xdd4a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdd4a8 | load | 4 | `abs 0xdd814` | `-` |
| `ldr` at 0xdd4ac | load | 4 | `abs 0x2974ac` | `-` |
| `str` at 0xdd4b4 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0xdd4bc | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xdd4c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdd4c4 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0xdd4c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdd4d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdd4d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdd4f4 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xdd4fc | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xdd500 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xdd504 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xdd518 | load | 4 | `abs 0xdd818` | `-` |
| `str` at 0xdd520 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xdd524 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdd528 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdd530 | load | 4 | `sp -0x38` | `-` |
| `ldrb` at 0xdd534 | load | 1 | `[sp-56] +0x10` | `-` |
| `ldr` at 0xdd540 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xdd548 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xdd550 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xdd554 | load | 4 | `[sp-56] +0x14` | `-` |
| `ldr` at 0xdd560 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdd564 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdd57c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xdd580 | load | 4 | `[sp-56] +0x14` | `-` |
| `ldr` at 0xdd590 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xdd598 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xdd5a0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xdd5a4 | load | 4 | `[sp-56] +0x14` | `-` |
| `ldr` at 0xdd5b0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdd5b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdd5c4 | load | 4 | `abs 0xdd81c` | `-` |
| `ldr` at 0xdd5d0 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xdd5d8 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xdd5e0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xdd5e4 | load | 4 | `[sp-56] +0x8` | `-` |
| `ldr` at 0xdd5f0 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xdd5f8 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xdd5fc | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xdd600 | load | 1 | `[sp-52] +0x20` | `-` |
| `str` at 0xdd604 | store | 4 | `sp -0x2c` | `[[sp-52]+32]:1` |
| `str` at 0xdd60c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xdd610 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdd614 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0xdd618 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd624 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdd628 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd644 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xdd64c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xdd650 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdd654 | load | 4 | `(unknown)` | `-` |
| `str` at 0xdd658 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xdd65c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdd660 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xdd664 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xdd668 | load | 1 | `[sp-52] +0x19` | `-` |
| `ldr` at 0xdd674 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdd678 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xdd67c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xdd680 | load | 1 | `[sp-52] +0x20` | `-` |
| `ldr` at 0xdd68c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdd690 | load | 4 | `[sp-52] +0x1c` | `-` |
| `ldr` at 0xdd698 | load | 4 | `sp -0x38` | `-` |
| `ldrb` at 0xdd69c | load | 1 | `[sp-56] +0x10` | `-` |
| `str` at 0xdd6ac | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xdd6b4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xdd6b8 | load | 4 | `[sp-56] +0x14` | `-` |
| `ldr` at 0xdd6c4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdd6c8 | load | 4 | `[sp-52] +0x1c` | `-` |
| `ldr` at 0xdd6e0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xdd6e4 | load | 4 | `[sp-56] +0x14` | `-` |
| `str` at 0xdd6f8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xdd700 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xdd704 | load | 4 | `[sp-56] +0x14` | `-` |
| `ldr` at 0xdd710 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdd714 | load | 4 | `[sp-52] +0x1c` | `-` |
| `ldr` at 0xdd724 | load | 4 | `abs 0xdd81c` | `-` |
| `str` at 0xdd734 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xdd738 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdd740 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xdd748 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdd74c | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xdd750 | load | 1 | `[sp-52] +0x19` | `-` |
| `ldr` at 0xdd754 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xdd758 | load | 1 | `[sp-52] +0x20` | `-` |
| `ldr` at 0xdd75c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdd760 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0xdd770 | load | 4 | `[[sp-52]] +0x0` | `-` |
| `strb` at 0xdd77c | store | 1 | `[sp-52] +0x21` | `(ret(0xdab04) & 0xff)` |
| `ldr` at 0xdd780 | load | 4 | `abs 0xdd820` | `-` |
| `ldr` at 0xdd784 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xdd790 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdd794 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0xdd798 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xdd79c | load | 1 | `[sp-52] +0x18` | `-` |
| `ldr` at 0xdd7a0 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xdd7a4 | load | 1 | `[sp-52] +0x19` | `-` |
| `str` at 0xdd7a8 | store | 4 | `sp -0x54` | `[[sp-52]+25]:1` |
| `ldr` at 0xdd7ac | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xdd7b0 | load | 1 | `[sp-52] +0x20` | `-` |
| `str` at 0xdd7b4 | store | 4 | `sp -0x50` | `[[sp-52]+32]:1` |
| `ldr` at 0xdd7b8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdd7bc | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0xdd7c0 | load | 4 | `[[sp-52]] +0x0` | `-` |
| `str` at 0xdd7c4 | store | 4 | `sp -0x4c` | `[[[sp-52]]]` |
| `ldr` at 0xdd7c8 | load | 4 | `sp -0x34` | `-` |
| `ldrsb` at 0xdd7cc | load | 1 | `[sp-52] +0x21` | `-` |
| `str` at 0xdd7d0 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xdd7d4 | load | 4 | `abs 0xdd824` | `-` |
| `ldr` at 0xdd7d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdd7dc | load | 4 | `[[sp-52]] +0x8` | `-` |
| `ldr` at 0xdd7e8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdd7f0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `str` at 0xdd7fc | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xdd800 | load | 4 | `sp -0x44` | `-` |
| `ldm` at 0xdd808 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xdd808 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdd808 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdd808 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdd774 | `(unknown callee) 0xdab04` | `[[sp-52]+25]:1` | `?` | `[[sp-52]+32]:1` | `[[[sp-52]]]` |
| `bl` at 0xdd7e4 | `(unknown callee) 0x221f0` | `0x122180` | `[sp-24]` | `[[[sp-52]]+8]` | `[[sp-52]+24]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xdd490 | `r0 < [0x2974b0] (signed, 32-bit)` | `r0 >= [0x2974b0] (signed, 32-bit)` |
| `ble` at 0xdd4d0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xdd53c | `? == [[sp-56]+16]:1` | `? != [[sp-56]+16]:1` |
| `beq` at 0xdd55c | `[[sp-56]+20] == 0x0` | `[[sp-56]+20] != 0x0` |
| `bne` at 0xdd58c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xdd5ac | `[[sp-56]+20] != 0x0` | `[[sp-56]+20] == 0x0` |
| `beq` at 0xdd5cc | `? == 0xffff` | `? != 0xffff` |
| `bne` at 0xdd5ec | `[[sp-56]+8] != 0x0` | `[[sp-56]+8] == 0x0` |
| `ble` at 0xdd620 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xdd670 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0xdd688 | `C == 0` | `not (C == 0)` |
| `beq` at 0xdd6a4 | `? == [[sp-56]+16]:1` | `? != [[sp-56]+16]:1` |
| `beq` at 0xdd6c0 | `[[sp-56]+20] == 0x0` | `[[sp-56]+20] != 0x0` |
| `bne` at 0xdd6f0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xdd70c | `[[sp-56]+20] != 0x0` | `[[sp-56]+20] == 0x0` |
| `beq` at 0xdd72c | `? == 0xffff` | `? != 0xffff` |
| `bls` at 0xdd78c | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdd808 | `r0` | `?` |

## `JChipTargetByName`

- `JChipTargetByName`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdd828 to 0xdde20.
  Established. A byte store writes `0x3a` to `[sp-36] +0x0` (`strb` at 0xdd900).
  The unnamed `0x196f4` call carries `[sp-28]`, `0x1221e0`, `sp-92`, `sp-72` (`bl` at 0xdd938).
  Equality `ret(0x196f4) == 0x3` takes `0xdd9a4`, while inequality falls through (`beq` at 0xdd94c).
  Target-name syntax beyond these accesses remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdd838 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xdd83c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdd848 | store | 4 | `sp -0x1c` | `ret(0x192bc)` |
| `ldr` at 0xdd84c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdd858 | store | 4 | `sp -0x20` | `ret(0x19538)` |
| `ldr` at 0xdd85c | load | 4 | `abs 0xdddfc` | `-` |
| `ldrb` at 0xdd860 | load | 1 | `abs 0x1221cc` | `-` |
| `strb` at 0xdd864 | store | 1 | `sp -0x5c` | `([0x1221cc]:1 & 0xff)` |
| `ldr` at 0xdd87c | load | 4 | `abs 0xdddfc` | `-` |
| `ldrb` at 0xdd880 | load | 1 | `abs 0x1221cc` | `-` |
| `strb` at 0xdd884 | store | 1 | `sp -0x70` | `([0x1221cc]:1 & 0xff)` |
| `str` at 0xdd8a0 | store | 4 | `sp -0x74` | `?` |
| `str` at 0xdd8a8 | store | 4 | `sp -0x78` | `0x0` |
| `str` at 0xdd8b0 | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0xdd8b4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdd8b8 | store | 4 | `sp -0x24` | `[sp-28]` |
| `ldr` at 0xdd8bc | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xdd8c0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xdd8cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd8d0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xdd8d4 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xdd8e4 | store | 1 | `(unknown)` | `(ret(0x195c8) & 0xff)` |
| `ldr` at 0xdd8e8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xdd8ec | load | 1 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xdd8f8 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xdd900 | store | 1 | `[sp-36] +0x0` | `0x3a` |
| `ldr` at 0xdd904 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdd90c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `str` at 0xdd920 | store | 4 | `sp -0xb0` | `sp-112` |
| `str` at 0xdd928 | store | 4 | `sp -0xac` | `sp-116` |
| `ldr` at 0xdd92c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdd930 | load | 4 | `abs 0xdde00` | `-` |
| `str` at 0xdd940 | store | 4 | `sp -0x3c` | `ret(0x196f4)` |
| `ldr` at 0xdd944 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdd950 | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0xdd960 | store | 1 | `sp -0x5c` | `0x0` |
| `ldr` at 0xdd96c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdd970 | load | 4 | `abs 0xdde04` | `-` |
| `str` at 0xdd984 | store | 4 | `sp -0x3c` | `ret(0x196f4)` |
| `ldr` at 0xdd988 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdd994 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xdd9a8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xdd9ac | load | 4 | `abs 0xdde08` | `-` |
| `ldr` at 0xdd9b0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdd9b4 | load | 4 | `abs 0x12da80` | `-` |
| `ldr` at 0xdd9c0 | load | 4 | `abs 0xdde0c` | `-` |
| `ldr` at 0xdd9c4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdd9e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdda00 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdda14 | load | 4 | `abs 0xdde0c` | `-` |
| `str` at 0xdda1c | store | 4 | `sp -0x78` | `?` |
| `ldr` at 0xdda24 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xdda2c | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xdda34 | load | 4 | `sp -0x78` | `-` |
| `ldrb` at 0xdda44 | load | 1 | `sp -0x5c` | `-` |
| `str` at 0xdda54 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xdda58 | load | 4 | `abs 0xdde10` | `-` |
| `ldr` at 0xdda5c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdda60 | load | 4 | `abs 0x2974b0` | `-` |
| `ldr` at 0xdda6c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdda74 | load | 4 | `abs 0xdde14` | `-` |
| `ldr` at 0xdda78 | load | 4 | `abs 0x2974ac` | `-` |
| `str` at 0xdda80 | store | 4 | `sp -0x80` | `?` |
| `ldr` at 0xdda84 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xdda88 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdda94 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xdda9c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xddab8 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xddac0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xddad8 | load | 4 | `sp -0x80` | `-` |
| `str` at 0xddadc | store | 4 | `sp -0x7c` | `[sp-128]` |
| `ldr` at 0xddae4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xddaec | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `str` at 0xddaf8 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xddafc | load | 4 | `abs 0xdde10` | `-` |
| `ldr` at 0xddb00 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xddb04 | load | 4 | `abs 0x2974b0` | `-` |
| `ldr` at 0xddb10 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xddb18 | load | 4 | `abs 0xdde14` | `-` |
| `ldr` at 0xddb1c | load | 4 | `abs 0x2974ac` | `-` |
| `str` at 0xddb24 | store | 4 | `sp -0x80` | `?` |
| `str` at 0xddb2c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xddb30 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xddb34 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xddb38 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xddb44 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xddb48 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xddb64 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xddb6c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xddb70 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xddb7c | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xddb88 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xddb8c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xddb9c | load | 4 | `sp -0x80` | `-` |
| `str` at 0xddba0 | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0xddba4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xddbac | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xddbb4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xddbbc | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xddbc4 | load | 4 | `sp -0x7c` | `-` |
| `str` at 0xddbd8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xddbdc | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xddbe0 | load | 4 | `[sp-124] +0xc` | `-` |
| `ldr` at 0xddbe4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xddbf0 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xddbf4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xddc10 | load | 4 | `[sp-124] +0x8` | `-` |
| `str` at 0xddc18 | store | 4 | `sp -0x80` | `?` |
| `ldr` at 0xddc1c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xddc20 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xddc24 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xddc30 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xddc3c | load | 4 | `sp -0x80` | `-` |
| `ldrb` at 0xddc40 | load | 1 | `[sp-128] +0x18` | `-` |
| `ldr` at 0xddc50 | load | 4 | `sp -0x80` | `-` |
| `ldrb` at 0xddc54 | load | 1 | `[sp-128] +0x18` | `-` |
| `ldr` at 0xddc60 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xddc6c | load | 4 | `sp -0x80` | `-` |
| `ldrb` at 0xddc70 | load | 1 | `[sp-128] +0x18` | `-` |
| `ldr` at 0xddc74 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xddc80 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xddc84 | load | 4 | `[sp-124] +0x0` | `-` |
| `ldr` at 0xddc90 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xddc94 | load | 4 | `[sp-124] +0x4` | `-` |
| `str` at 0xddca8 | store | 4 | `sp -0x9c` | `sp-148` |
| `ldr` at 0xddcac | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xddcb0 | load | 4 | `[sp-124] +0x4` | `-` |
| `str` at 0xddcb4 | store | 4 | `sp -0xa0` | `[[sp-124]+4]` |
| `ldr` at 0xddcb8 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xddcbc | load | 4 | `[sp-124] +0x4` | `-` |
| `ldr` at 0xddcc8 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xddccc | load | 4 | `[sp-124] +0x0` | `-` |
| `str` at 0xddcd0 | store | 4 | `sp -0xa0` | `[[sp-124]]` |
| `ldr` at 0xddcd4 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xddcd8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xddcdc | load | 4 | `sp -0x80` | `-` |
| `ldrb` at 0xddce0 | load | 1 | `(unknown)` | `-` |
| `str` at 0xddce4 | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0xddce8 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0xddcec | load | 4 | `abs 0xdde18` | `-` |
| `ldr` at 0xddcf0 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0xddcf4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xddd04 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xddd08 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xddd0c | load | 4 | `sp -0x80` | `-` |
| `ldrb` at 0xddd10 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xddd18 | load | 4 | `abs 0xdde1c` | `-` |
| `ldr` at 0xddd1c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xddd2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xddd40 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xddd44 | load | 4 | `[sp-128] +0x0` | `-` |
| `str` at 0xddd48 | store | 4 | `sp -0x78` | `[[sp-128]]` |
| `ldr` at 0xddd4c | load | 4 | `sp -0x80` | `-` |
| `ldrb` at 0xddd50 | load | 1 | `[sp-128] +0x18` | `-` |
| `str` at 0xddd54 | store | 4 | `sp -0x74` | `[[sp-128]+24]:1` |
| `ldr` at 0xddd5c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xddd64 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xddd6c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xddd74 | load | 4 | `abs 0xdde14` | `-` |
| `ldr` at 0xddd78 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xddd7c | load | 4 | `abs 0x2974ac` | `-` |
| `str` at 0xddd84 | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0xddd88 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0xddd94 | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0xddd98 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xddd9c | load | 4 | `[sp-120] +0x0` | `-` |
| `ldr` at 0xddda4 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0xdddac | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0xdddb0 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0xdddb4 | store | 4 | `sp -0xa4` | `[sp-168]` |
| `ldr` at 0xdddb8 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xdddc4 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xdddc8 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0xdddd0 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0xdddd4 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0xdddd8 | store | 4 | `sp -0x98` | `[sp-164]` |
| `ldr` at 0xddde0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdddec | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0xdddf0 | load | 4 | `sp -0x98` | `-` |
| `ldm` at 0xdddf8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xdddf8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdddf8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdddf8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdd840 | `(unknown callee) 0x192bc` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xdd850 | `(unknown callee) 0x19538` | `[sp-24]` | `?` | `?` | `ret(0x192bc)` |
| `bl` at 0xdd878 | `(unknown callee) 0x19748` | `sp-91` | `0x0` | `0x13` | `sp-91` |
| `bl` at 0xdd898 | `(unknown callee) 0x19748` | `sp-111` | `0x0` | `0x13` | `sp-111` |
| `bl` at 0xdd8dc | `(unknown callee) 0x195c8` | `?` | `?` | `?` | `?` |
| `bl` at 0xdd938 | `(unknown callee) 0x196f4` | `[sp-28]` | `0x1221e0` | `sp-92` | `sp-72` |
| `bl` at 0xdd97c | `(unknown callee) 0x196f4` | `[sp-28]` | `0x1221f4` | `sp-112` | `sp-116` |
| `bl` at 0xdd9f0 | `(unknown callee) 0x19310` | `?` | `sp-112` | `sp-112` | `?` |
| `bl` at 0xddaa4 | `(unknown callee) 0x19310` | `?` | `sp-92` | `sp-92` | `?` |
| `bl` at 0xddac8 | `(unknown callee) 0x19310` | `?` | `sp-92` | `sp-92` | `?` |
| `bl` at 0xddcf8 | `(unknown callee) 0x197a8` | `sp-148` | `0x1220d0` | `?` | `?` |
| `bl` at 0xddd20 | `(unknown callee) 0x197a8` | `sp-148` | `0x1220d8` | `?` | `?` |
| `bl` at 0xddd30 | `(unknown callee) 0x19310` | `sp-148` | `[sp-28]` | `?` | `sp-148` |
| `bl` at 0xddd70 | `(unknown callee) 0x1970c` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0xddde4 | `(unknown callee) 0x1970c` | `[sp-28]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xdd8c8 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xdd8f4 | `[[sp-36]]:1 != 0x2e` | `[[sp-36]]:1 == 0x2e` |
| `beq` at 0xdd94c | `ret(0x196f4) == 0x3` | `ret(0x196f4) != 0x3` |
| `beq` at 0xdd958 | `ret(0x196f4) == 0x4` | `ret(0x196f4) != 0x4` |
| `beq` at 0xdd990 | `ret(0x196f4) == 0x1` | `ret(0x196f4) != 0x1` |
| `beq` at 0xdd99c | `ret(0x196f4) == 0x2` | `ret(0x196f4) != 0x2` |
| `bge` at 0xdd9bc | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xdd9fc | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |
| `bne` at 0xdda3c | `[sp-120] != 0x0` | `[sp-120] == 0x0` |
| `beq` at 0xdda4c | `[sp-92]:1 == 0x0` | `[sp-92]:1 != 0x0` |
| `bge` at 0xdda68 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xdda90 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xddab0 | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |
| `bne` at 0xddad4 | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |
| `bge` at 0xddb0c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `ble` at 0xddb40 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xddb78 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xddb84 | `[sp-124] == 0x0` | `[sp-124] != 0x0` |
| `beq` at 0xddb94 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xddbcc | `[sp-124] != 0x0` | `[sp-124] == 0x0` |
| `ble` at 0xddbec | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xddc2c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xddc38 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xddc48 | `[[sp-128]+24]:1 == 0x0` | `[[sp-128]+24]:1 != 0x0` |
| `bne` at 0xddc5c | `[[sp-128]+24]:1 != 0x0` | `[[sp-128]+24]:1 == 0x0` |
| `beq` at 0xddc68 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xddc7c | `[[sp-128]+24]:1 == [sp-116]` | `[[sp-128]+24]:1 != [sp-116]` |
| `bne` at 0xddc8c | `[[sp-124]] != 0x0` | `[[sp-124]] == 0x0` |
| `bne` at 0xddc9c | `[[sp-124]+4] != 0x0` | `[[sp-124]+4] == 0x0` |
| `bne` at 0xddcc4 | `[[sp-124]+4] != 0x0` | `[[sp-124]+4] == 0x0` |
| `bne` at 0xddd3c | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |
| `beq` at 0xdddc0 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdddf8 | `r0` | `[sp-152]` |

## `JChipGetChipList`

- `JChipGetChipList`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdde74 to 0xde004.
  Established. A word assignment writes `[sp-32]+50` to `sp -0x20` (`str` at 0xddf54).
  The unnamed `0x195d4` call carries `[sp-36]`, `[sp-32]+50`, `[sp-32]`, `[sp-32]+50` (`bl` at 0xddf60).
  Another word store records `(ret(0x195d4) + [sp-28])` at `sp -0x28` (`str` at 0xddf78).
  Chip-list allocation ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdde88 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xdde90 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xdde98 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xddea0 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xddea8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xddeac | load | 4 | `abs 0xddff8` | `-` |
| `ldr` at 0xddeb0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xddeb4 | load | 4 | `abs 0x2974b0` | `-` |
| `ldr` at 0xddec0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xddec8 | load | 4 | `abs 0xddffc` | `-` |
| `ldr` at 0xddecc | load | 4 | `abs 0x2974ac` | `-` |
| `str` at 0xdded4 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xddedc | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0xddee0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xddee4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xddee8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xddef4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xddef8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xddf14 | load | 4 | `(unknown)` | `-` |
| `str` at 0xddf1c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xddf20 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xddf38 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xddf3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xddf4c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xddf54 | store | 4 | `sp -0x20` | `[sp-32]+50` |
| `ldr` at 0xddf58 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xddf5c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xddf68 | store | 4 | `sp -0x24` | `ret(0x195d4)` |
| `ldr` at 0xddf6c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xddf70 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xddf78 | store | 4 | `sp -0x28` | `(ret(0x195d4) + [sp-28])` |
| `ldr` at 0xddf7c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xddf84 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xddf90 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xddf94 | load | 4 | `abs 0xde000` | `-` |
| `ldr` at 0xddf9c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xddfac | load | 4 | `sp -0x28` | `-` |
| `str` at 0xddfb4 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x19538))` |
| `ldr` at 0xddfb8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xddfbc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xddfc4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xddfc8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xddfd0 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0xddfd8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xddfe0 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xddfe8 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xddff4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xddff4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xddff4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xddf2c | `(unknown callee) 0x19538` | `?` | `?` | `?` | `?` |
| `bl` at 0xddf60 | `(unknown callee) 0x195d4` | `[sp-36]` | `[sp-32]+50` | `[sp-32]` | `[sp-32]+50` |
| `bl` at 0xddf8c | `(unknown callee) 0x19808` | `[sp-40]` | `[sp-48]+4` | `?` | `[sp-48]+4` |
| `bl` at 0xddf98 | `(unknown callee) 0x195e0` | `[sp-40]` | `0x122200` | `?` | `?` |
| `bl` at 0xddfa0 | `(unknown callee) 0x19538` | `[sp-40]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xddebc | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `ble` at 0xddef0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bls` at 0xddf48 | `ret(0x19538)+2 <= ? (unsigned, 32-bit)` | `ret(0x19538)+2 > ? (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xddff4 | `r0` | `?` |

## `JChip_get_field`

- `JChip_get_field`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdf6d8 to 0xdf808.
  Established. The unnamed `0xe41a8` call carries unknown, `[sp-28]`, `0x0`, unknown (`bl` at 0xdf7e0).
  A word read addresses `ret(0xdefac) +0x28` (`ldr` at 0xdf760).
  A nonzero `ret(0xdefac)` takes `0xdf744`, while zero falls through (`bne` at 0xdf734).
  Field extraction result meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdf6e8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdf6ec | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xdf6f0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xdf6f4 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xdf6f8 | load | 4 | `abs 0xdf7f8` | `-` |
| `ldr` at 0xdf6fc | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xdf708 | load | 4 | `abs 0xdf7fc` | `-` |
| `ldr` at 0xdf70c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdf710 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdf714 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdf71c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdf728 | store | 4 | `sp -0x24` | `ret(0xdefac)` |
| `ldr` at 0xdf72c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdf738 | load | 4 | `abs 0xdf800` | `-` |
| `str` at 0xdf73c | store | 4 | `sp -0x28` | `0xfffff7c6` |
| `ldr` at 0xdf744 | load | 4 | `abs 0xdf7f8` | `-` |
| `ldr` at 0xdf748 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xdf754 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdf758 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdf760 | load | 4 | `ret(0xdefac) +0x28` | `-` |
| `ldr` at 0xdf768 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf76c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf770 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdf784 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf78c | load | 4 | `abs 0xdf804` | `-` |
| `ldr` at 0xdf790 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf798 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdf79c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdf7a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf7ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf7b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdf7b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdf7bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf7c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf7c8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdf7cc | store | 4 | `sp -0x2c` | `[sp-32]` |
| `ldr` at 0xdf7d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf7d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdf7dc | load | 4 | `(unknown)` | `-` |
| `str` at 0xdf7e8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xdf7ec | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xdf7f4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdf7f4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdf7f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdf718 | `(unknown callee) 0x221f0` | `0x1224e8` | `r0` | `r1` | `r2` |
| `bl` at 0xdf720 | `(unknown callee) 0xdefac` | `?` | `?` | `?` | `?` |
| `bl` at 0xdf794 | `(unknown callee) 0x221f0` | `0x12251c` | `?` | `?` | `?` |
| `bl` at 0xdf7e0 | `(unknown callee) 0xe41a8` | `?` | `[sp-28]` | `0x0` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xdf704 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xdf734 | `ret(0xdefac) != 0x0` | `ret(0xdefac) == 0x0` |
| `bls` at 0xdf750 | `[0x29737c] <= 0x3 (unsigned, 32-bit)` | `[0x29737c] > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdf7f4 | `r0` | `?` |

## `JChip_get_field_by_name`

- `JChip_get_field_by_name`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdf808 to 0xdf98c.
  Established. The unsigned comparison `ret(0xe4488) < [[sp-44]+2]:2` takes `0xdf8c8`, while greater or equal falls through (`blo` at 0xdf8b8).
  An unnamed `0xe4488` call carries `[sp-44]`, `[sp-28]` and two unknown candidates (`bl` at 0xdf89c).
  A halfword read uses `[sp-44] +0x2` (`ldrh` at 0xdf8ac).
  Named-field lookup failure meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdf818 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdf81c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xdf820 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xdf824 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xdf828 | load | 4 | `abs 0xdf97c` | `-` |
| `ldr` at 0xdf82c | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xdf838 | load | 4 | `abs 0xdf980` | `-` |
| `ldr` at 0xdf83c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdf840 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdf844 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdf84c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdf858 | store | 4 | `sp -0x24` | `ret(0xdefac)` |
| `ldr` at 0xdf85c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdf868 | load | 4 | `abs 0xdf984` | `-` |
| `str` at 0xdf86c | store | 4 | `sp -0x30` | `0xfffff7c6` |
| `ldr` at 0xdf874 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdf878 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdf880 | load | 4 | `ret(0xdefac) +0x28` | `-` |
| `ldr` at 0xdf888 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf88c | load | 4 | `(unknown)` | `-` |
| `str` at 0xdf890 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xdf894 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdf898 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdf8a4 | store | 4 | `sp -0x28` | `ret(0xe4488)` |
| `ldr` at 0xdf8a8 | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0xdf8ac | load | 2 | `[sp-44] +0x2` | `-` |
| `ldr` at 0xdf8b0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdf8bc | load | 4 | `abs 0xdf984` | `-` |
| `str` at 0xdf8c0 | store | 4 | `sp -0x30` | `0xfffff7c6` |
| `ldr` at 0xdf8c8 | load | 4 | `abs 0xdf97c` | `-` |
| `ldr` at 0xdf8cc | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xdf8d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdf8dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdf8e4 | load | 4 | `[sp-36] +0x28` | `-` |
| `ldr` at 0xdf8ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf8f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf8f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdf908 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf910 | load | 4 | `abs 0xdf988` | `-` |
| `ldr` at 0xdf914 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf91c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdf920 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdf928 | load | 4 | `[sp-36] +0x28` | `-` |
| `ldr` at 0xdf930 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf934 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdf938 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdf940 | load | 4 | `[sp-36] +0x28` | `-` |
| `ldr` at 0xdf948 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf94c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdf950 | store | 4 | `sp -0x34` | `[sp-32]` |
| `ldr` at 0xdf954 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf958 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdf960 | load | 4 | `(unknown)` | `-` |
| `str` at 0xdf96c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xdf970 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0xdf978 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdf978 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdf978 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdf848 | `(unknown callee) 0x221f0` | `0x12253c` | `r0` | `r1` | `r2` |
| `bl` at 0xdf850 | `(unknown callee) 0xdefac` | `?` | `?` | `?` | `?` |
| `bl` at 0xdf89c | `(unknown callee) 0xe4488` | `[sp-44]` | `[sp-28]` | `?` | `?` |
| `bl` at 0xdf918 | `(unknown callee) 0x221f0` | `0x12251c` | `?` | `?` | `?` |
| `bl` at 0xdf964 | `(unknown callee) 0xe41a8` | `?` | `?` | `0x0` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xdf834 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xdf864 | `ret(0xdefac) != 0x0` | `ret(0xdefac) == 0x0` |
| `blo` at 0xdf8b8 | `ret(0xe4488) < [[sp-44]+2]:2 (unsigned, 32-bit)` | `ret(0xe4488) >= [[sp-44]+2]:2 (unsigned, 32-bit)` |
| `bls` at 0xdf8d4 | `[0x29737c] <= 0x3 (unsigned, 32-bit)` | `[0x29737c] > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdf978 | `r0` | `?` |

## `JChip_get_field_str`

- `JChip_get_field_str`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdf98c to 0xdfb68.
  Established. A nonzero `ret(0x1943c)` takes `0xdfadc`, while zero falls through (`bne` at 0xdfacc).
  The unnamed `0xe4dc0` call carries `[sp-40]`, `[sp-44]`, `[sp-32]`, unknown (`bl` at 0xdfb34).
  A word store places `0xfffff7cc` at `sp -0x30` (`str` at 0xdfad4).
  Field-string representation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdf99c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdf9a0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xdf9a4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xdf9a8 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xdf9ac | load | 4 | `abs 0xdfb54` | `-` |
| `ldr` at 0xdf9b0 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xdf9bc | load | 4 | `abs 0xdfb58` | `-` |
| `ldr` at 0xdf9c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdf9c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdf9c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdf9d0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdf9dc | store | 4 | `sp -0x24` | `ret(0xdefac)` |
| `ldr` at 0xdf9e0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdf9ec | load | 4 | `abs 0xdfb5c` | `-` |
| `str` at 0xdf9f0 | store | 4 | `sp -0x30` | `0xfffff7c6` |
| `ldr` at 0xdf9f8 | load | 4 | `abs 0xdfb54` | `-` |
| `ldr` at 0xdf9fc | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xdfa08 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdfa0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdfa14 | load | 4 | `ret(0xdefac) +0x28` | `-` |
| `ldr` at 0xdfa1c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdfa20 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdfa24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdfa38 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdfa40 | load | 4 | `abs 0xdfb60` | `-` |
| `ldr` at 0xdfa44 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdfa4c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdfa50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdfa58 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdfa60 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdfa64 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdfa68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdfa7c | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xdfa84 | load | 2 | `(unknown)` | `-` |
| `str` at 0xdfa88 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xdfa8c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdfaa4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xdfac0 | store | 4 | `sp -0x2c` | `ret(0x1943c)` |
| `ldr` at 0xdfac4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdfad0 | load | 4 | `abs 0xdfb64` | `-` |
| `str` at 0xdfad4 | store | 4 | `sp -0x30` | `0xfffff7cc` |
| `ldr` at 0xdfadc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdfae0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdfae8 | load | 4 | `[sp-36] +0x28` | `-` |
| `ldr` at 0xdfaf0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdfaf4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdfaf8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdfb00 | load | 4 | `[sp-36] +0x28` | `-` |
| `ldr` at 0xdfb08 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdfb0c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xdfb10 | store | 4 | `sp -0x34` | `ret(0x1943c)` |
| `ldr` at 0xdfb14 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdfb18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdfb20 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdfb28 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdfb2c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdfb30 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdfb38 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xdfb44 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xdfb48 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0xdfb50 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdfb50 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdfb50 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdf9cc | `(unknown callee) 0x221f0` | `0x122578` | `r0` | `r1` | `r2` |
| `bl` at 0xdf9d4 | `(unknown callee) 0xdefac` | `?` | `?` | `?` | `?` |
| `bl` at 0xdfa48 | `(unknown callee) 0x221f0` | `0x1225b0` | `?` | `?` | `?` |
| `bl` at 0xdfab8 | `(unknown callee) 0x1943c` | `?` | `?` | `?` | `?` |
| `bl` at 0xdfb24 | `(unknown callee) 0xe41a8` | `?` | `[sp-28]` | `0x0` | `?` |
| `bl` at 0xdfb34 | `(unknown callee) 0xe4dc0` | `[sp-40]` | `[sp-44]` | `[sp-32]` | `?` |
| `bl` at 0xdfb3c | `(unknown callee) 0x1970c` | `[sp-44]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xdf9b8 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xdf9e8 | `ret(0xdefac) != 0x0` | `ret(0xdefac) == 0x0` |
| `bls` at 0xdfa04 | `[0x29737c] <= 0x3 (unsigned, 32-bit)` | `[0x29737c] > 0x3 (unsigned, 32-bit)` |
| `bne` at 0xdfacc | `ret(0x1943c) != 0x0` | `ret(0x1943c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdfb50 | `r0` | `?` |

## `JChip_can_override_field`

- `JChip_can_override_field`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdfb68 to 0xdfc34.
  Established. A byte read has an unknown effective target (`ldrb` at 0xdfbf4).
  Included word assignments write `0x0` and `0x1` to `sp -0x28` (`str` at 0xdfc08, `str` at 0xdfc14).
  A nonzero `ret(0xdefac)` takes `0xdfbd0`, while zero falls through (`bne` at 0xdfbc0).
  Field override policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdfb78 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdfb7c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xdfb80 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xdfb84 | load | 4 | `abs 0xdfc2c` | `-` |
| `ldr` at 0xdfb88 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xdfb94 | load | 4 | `abs 0xdfc30` | `-` |
| `ldr` at 0xdfb98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdfb9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdfba0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdfba8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdfbb4 | store | 4 | `sp -0x20` | `ret(0xdefac)` |
| `ldr` at 0xdfbb8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdfbc8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xdfbd0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdfbd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdfbdc | load | 4 | `ret(0xdefac) +0x28` | `-` |
| `ldr` at 0xdfbe4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdfbe8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdfbec | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xdfbf4 | load | 1 | `(unknown)` | `-` |
| `str` at 0xdfc08 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xdfc14 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0xdfc18 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xdfc1c | store | 4 | `sp -0x24` | `[sp-40]` |
| `ldr` at 0xdfc20 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xdfc28 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdfc28 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdfc28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdfba4 | `(unknown callee) 0x221f0` | `0x1225d4` | `r0` | `r1` | `r2` |
| `bl` at 0xdfbac | `(unknown callee) 0xdefac` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xdfb90 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xdfbc0 | `ret(0xdefac) != 0x0` | `ret(0xdefac) == 0x0` |
| `beq` at 0xdfc00 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdfc28 | `r0` | `?` |

## `JChip_protect_field`

- `JChip_protect_field`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdf544 to 0xdf6d8.
  Established. A zero `[sp-32]` takes `0xdf658`, while nonzero falls through (`beq` at 0xdf630).
  The unnamed `0xe53fc` call carries unknown, `[sp-28]`, `[sp-32]`, unknown (`bl` at 0xdf654).
  A byte store has unknown value and effective target (`strb` at 0xdf6a8).
  Protection flag placement remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdf554 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdf558 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xdf55c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xdf560 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xdf564 | load | 4 | `abs 0xdf6c0` | `-` |
| `ldr` at 0xdf568 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xdf574 | load | 4 | `abs 0xdf6c4` | `-` |
| `ldr` at 0xdf578 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdf57c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdf580 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdf588 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdf594 | store | 4 | `sp -0x24` | `ret(0xdefac)` |
| `ldr` at 0xdf598 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdf5a4 | load | 4 | `abs 0xdf6c8` | `-` |
| `str` at 0xdf5a8 | store | 4 | `sp -0x28` | `0xfffff7c6` |
| `ldr` at 0xdf5b0 | load | 4 | `abs 0xdf6c0` | `-` |
| `ldr` at 0xdf5b4 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xdf5c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdf5c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdf5cc | load | 4 | `ret(0xdefac) +0x28` | `-` |
| `ldr` at 0xdf5d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf5d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf5dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdf5f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf5f8 | load | 4 | `abs 0xdf6cc` | `-` |
| `ldr` at 0xdf5fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf604 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdf610 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdf614 | load | 4 | `abs 0xdf6d0` | `-` |
| `ldr` at 0xdf618 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xdf620 | load | 4 | `abs 0xdf6d4` | `-` |
| `ldr` at 0xdf628 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdf634 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdf638 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdf640 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf648 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf64c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdf650 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdf658 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdf65c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdf664 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf66c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf670 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf674 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdf67c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdf680 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdf688 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf690 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf694 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdf698 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xdf6a0 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xdf6a8 | store | 1 | `(unknown)` | `?` |
| `str` at 0xdf6b0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xdf6b4 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xdf6bc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdf6bc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdf6bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdf584 | `(unknown callee) 0x221f0` | `0x122478` | `r0` | `r1` | `r2` |
| `bl` at 0xdf58c | `(unknown callee) 0xdefac` | `?` | `?` | `?` | `?` |
| `bl` at 0xdf600 | `(unknown callee) 0x221f0` | `0x1224b0` | `?` | `?` | `?` |
| `bl` at 0xdf61c | `(unknown callee) 0x221f0` | `0x1224d0` | `[[sp-32]]` | `?` | `[sp-32]` |
| `bl` at 0xdf624 | `(unknown callee) 0x221f0` | `0x1224e4` | `?` | `?` | `?` |
| `bl` at 0xdf654 | `(unknown callee) 0xe53fc` | `?` | `[sp-28]` | `[sp-32]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xdf570 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xdf5a0 | `ret(0xdefac) != 0x0` | `ret(0xdefac) == 0x0` |
| `bls` at 0xdf5bc | `[0x29737c] <= 0x1 (unsigned, 32-bit)` | `[0x29737c] > 0x1 (unsigned, 32-bit)` |
| `beq` at 0xdf60c | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `beq` at 0xdf630 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdf6bc | `r0` | `?` |

## `JChip_override_field`

- `JChip_override_field`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdfc34 to 0xdfd90.
  Established. An included word load addresses entry `r3 +0x0` (`ldr` at 0xdfc68).
  The unnamed `0xe53fc` call carries unknown, `[sp-28]`, `[sp-32]`, unknown (`bl` at 0xdfd68).
  A word assignment clears `sp -0x28` (`str` at 0xdfd70).
  Override acceptance beyond the printed predicates remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdfc44 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdfc48 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xdfc4c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xdfc50 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xdfc54 | load | 4 | `abs 0xdfd80` | `-` |
| `ldr` at 0xdfc58 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xdfc64 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdfc68 | load | 4 | `r3 +0x0` | `-` |
| `str` at 0xdfc6c | store | 4 | `sp -0x2c` | `[r3]` |
| `ldr` at 0xdfc70 | load | 4 | `abs 0xdfd84` | `-` |
| `ldr` at 0xdfc74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdfc78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdfc7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdfc84 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdfc90 | store | 4 | `sp -0x24` | `ret(0xdefac)` |
| `ldr` at 0xdfc94 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdfca0 | load | 4 | `abs 0xdfd88` | `-` |
| `str` at 0xdfca4 | store | 4 | `sp -0x28` | `0xfffff7c6` |
| `ldr` at 0xdfcac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdfcb0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdfcb8 | load | 4 | `ret(0xdefac) +0x28` | `-` |
| `ldr` at 0xdfcc0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdfcc4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdfcc8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xdfcd0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xdfce0 | load | 4 | `abs 0xdfd88` | `-` |
| `str` at 0xdfce4 | store | 4 | `sp -0x28` | `0xfffff7c6` |
| `ldr` at 0xdfcec | load | 4 | `abs 0xdfd80` | `-` |
| `ldr` at 0xdfcf0 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xdfcfc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdfd00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdfd08 | load | 4 | `ret(0xdefac) +0x28` | `-` |
| `ldr` at 0xdfd10 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdfd14 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdfd18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdfd2c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdfd34 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdfd38 | load | 4 | `abs 0xdfd8c` | `-` |
| `ldr` at 0xdfd3c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdfd40 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xdfd48 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdfd4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdfd54 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdfd5c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdfd60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdfd64 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdfd70 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xdfd74 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xdfd7c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdfd7c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdfd7c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdfc80 | `(unknown callee) 0x221f0` | `0x122610` | `r0` | `r1` | `r2` |
| `bl` at 0xdfc88 | `(unknown callee) 0xdefac` | `?` | `?` | `?` | `?` |
| `bl` at 0xdfd44 | `(unknown callee) 0x221f0` | `0x122658` | `?` | `[[sp-32]]` | `?` |
| `bl` at 0xdfd68 | `(unknown callee) 0xe53fc` | `?` | `[sp-28]` | `[sp-32]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xdfc60 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xdfc9c | `ret(0xdefac) != 0x0` | `ret(0xdefac) == 0x0` |
| `beq` at 0xdfcdc | `? == 0x0` | `? != 0x0` |
| `bls` at 0xdfcf8 | `[0x29737c] <= 0x3 (unsigned, 32-bit)` | `[0x29737c] > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdfd7c | `r0` | `?` |

## `JChip_override_field_by_name`

- `JChip_override_field_by_name`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdfd90 to 0xdff40.
  Established. A halfword read uses `[sp-44] +0x2` (`ldrh` at 0xdfe40).
  The unsigned comparison `ret(0xe4488) < [[sp-44]+2]:2` takes `0xdfe5c`, while greater or equal falls through (`blo` at 0xdfe4c).
  The unnamed `0xe53fc` call carries two unknown candidates, `[sp-32]`, unknown (`bl` at 0xdff18).
  Named override result interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdfda0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdfda4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xdfda8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xdfdac | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xdfdb0 | load | 4 | `abs 0xdff30` | `-` |
| `ldr` at 0xdfdb4 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xdfdc0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdfdc4 | load | 4 | `r3 +0x0` | `-` |
| `str` at 0xdfdc8 | store | 4 | `sp -0x34` | `[r3]` |
| `ldr` at 0xdfdcc | load | 4 | `abs 0xdff34` | `-` |
| `ldr` at 0xdfdd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdfdd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdfdd8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdfde0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdfdec | store | 4 | `sp -0x24` | `ret(0xdefac)` |
| `ldr` at 0xdfdf0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdfdfc | load | 4 | `abs 0xdff38` | `-` |
| `str` at 0xdfe00 | store | 4 | `sp -0x30` | `0xfffff7c6` |
| `ldr` at 0xdfe08 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdfe0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdfe14 | load | 4 | `ret(0xdefac) +0x28` | `-` |
| `ldr` at 0xdfe1c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdfe20 | load | 4 | `(unknown)` | `-` |
| `str` at 0xdfe24 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xdfe28 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdfe2c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdfe38 | store | 4 | `sp -0x28` | `ret(0xe4488)` |
| `ldr` at 0xdfe3c | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0xdfe40 | load | 2 | `[sp-44] +0x2` | `-` |
| `ldr` at 0xdfe44 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdfe50 | load | 4 | `abs 0xdff38` | `-` |
| `str` at 0xdfe54 | store | 4 | `sp -0x30` | `0xfffff7c6` |
| `ldr` at 0xdfe5c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdfe60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdfe68 | load | 4 | `[sp-36] +0x28` | `-` |
| `ldr` at 0xdfe70 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdfe74 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdfe78 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xdfe80 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xdfe90 | load | 4 | `abs 0xdff38` | `-` |
| `str` at 0xdfe94 | store | 4 | `sp -0x30` | `0xfffff7c6` |
| `ldr` at 0xdfe9c | load | 4 | `abs 0xdff30` | `-` |
| `ldr` at 0xdfea0 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xdfeac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdfeb0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdfeb8 | load | 4 | `[sp-36] +0x28` | `-` |
| `ldr` at 0xdfec0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdfec4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdfec8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdfedc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdfee4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdfee8 | load | 4 | `abs 0xdff3c` | `-` |
| `ldr` at 0xdfeec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdfef0 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xdfef8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdfefc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdff04 | load | 4 | `[sp-36] +0x28` | `-` |
| `ldr` at 0xdff0c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdff10 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdff14 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdff20 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xdff24 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0xdff2c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdff2c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdff2c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdfddc | `(unknown callee) 0x221f0` | `0x12268c` | `r0` | `r1` | `r2` |
| `bl` at 0xdfde4 | `(unknown callee) 0xdefac` | `?` | `?` | `?` | `?` |
| `bl` at 0xdfe30 | `(unknown callee) 0xe4488` | `[sp-44]` | `[sp-28]` | `?` | `?` |
| `bl` at 0xdfef4 | `(unknown callee) 0x221f0` | `0x122658` | `?` | `[[sp-32]]` | `?` |
| `bl` at 0xdff18 | `(unknown callee) 0xe53fc` | `?` | `?` | `[sp-32]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xdfdbc | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xdfdf8 | `ret(0xdefac) != 0x0` | `ret(0xdefac) == 0x0` |
| `blo` at 0xdfe4c | `ret(0xe4488) < [[sp-44]+2]:2 (unsigned, 32-bit)` | `ret(0xe4488) >= [[sp-44]+2]:2 (unsigned, 32-bit)` |
| `beq` at 0xdfe8c | `? == 0x0` | `? != 0x0` |
| `bls` at 0xdfea8 | `[0x29737c] <= 0x3 (unsigned, 32-bit)` | `[0x29737c] > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdff2c | `r0` | `?` |

## `JChip_find_tdr`

- `JChip_find_tdr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdff40 to 0xe0028.
  Established. A nonzero `ret(0x19310)` takes `0xdfff8`, while zero falls through (`bne` at 0xdffe8).
  The unnamed `0x19310` call carries `[sp-24]` followed by three unknown candidates (`bl` at 0xdffdc).
  A word store increments `sp -0x20` with `[sp-32]+1` (`str` at 0xe0000).
  TDR name comparison encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdff50 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdff54 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xdff58 | load | 4 | `abs 0xe001c` | `-` |
| `ldr` at 0xdff5c | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xdff68 | load | 4 | `abs 0xe0020` | `-` |
| `ldr` at 0xdff6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdff70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdff78 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdff84 | store | 4 | `sp -0x1c` | `ret(0xdefac)` |
| `ldr` at 0xdff88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdff94 | load | 4 | `abs 0xe0024` | `-` |
| `str` at 0xdff98 | store | 4 | `sp -0x24` | `0xfffff7c6` |
| `str` at 0xdffa4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xdffa8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdffac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdffb0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdffbc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdffc0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdffc8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdffd0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdffd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdffd8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdffec | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdfff0 | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0xdfff8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe0000 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe0008 | load | 4 | `abs 0xe0024` | `-` |
| `str` at 0xe000c | store | 4 | `sp -0x24` | `0xfffff7c6` |
| `ldr` at 0xe0010 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xe0018 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe0018 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe0018 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdff74 | `(unknown callee) 0x221f0` | `0x1226e0` | `r0` | `r1` | `[0x29737c]` |
| `bl` at 0xdff7c | `(unknown callee) 0xdefac` | `?` | `?` | `?` | `?` |
| `bl` at 0xdffdc | `(unknown callee) 0x19310` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xdff64 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xdff90 | `ret(0xdefac) != 0x0` | `ret(0xdefac) == 0x0` |
| `bhs` at 0xdffb8 | `C == 1` | `not (C == 1)` |
| `bne` at 0xdffe8 | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe0018 | `r0` | `?` |

## `JChip_find_tdr_field`

- `JChip_find_tdr_field`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe0028 to 0xe0104.
  Established. The unnamed `0xe4488` call carries `[sp-40]`, `[sp-28]` and two unknown candidates (`bl` at 0xe00b8).
  A halfword read addresses `[sp-40] +0x2` (`ldrh` at 0xe00c8).
  The unsigned comparison `ret(0xe4488) < [[sp-40]+2]:2` takes `0xe00e4`, while greater or equal falls through (`blo` at 0xe00d4).
  TDR field lookup result domain remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe0038 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe003c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe0040 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xe0044 | load | 4 | `abs 0xe00f8` | `-` |
| `ldr` at 0xe0048 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe0054 | load | 4 | `abs 0xe00fc` | `-` |
| `ldr` at 0xe0058 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe005c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe0060 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe0068 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe0074 | store | 4 | `sp -0x20` | `ret(0xdefac)` |
| `ldr` at 0xe0078 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe0084 | load | 4 | `abs 0xe0100` | `-` |
| `str` at 0xe0088 | store | 4 | `sp -0x2c` | `0xfffff7c6` |
| `ldr` at 0xe0090 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe0094 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe009c | load | 4 | `ret(0xdefac) +0x28` | `-` |
| `ldr` at 0xe00a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe00a8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe00ac | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe00b0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe00b4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe00c0 | store | 4 | `sp -0x24` | `ret(0xe4488)` |
| `ldr` at 0xe00c4 | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0xe00c8 | load | 2 | `[sp-40] +0x2` | `-` |
| `ldr` at 0xe00cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe00d8 | load | 4 | `abs 0xe0100` | `-` |
| `str` at 0xe00dc | store | 4 | `sp -0x2c` | `0xfffff7c6` |
| `ldr` at 0xe00e4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe00e8 | store | 4 | `sp -0x2c` | `ret(0xe4488)` |
| `ldr` at 0xe00ec | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0xe00f4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe00f4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe00f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe0064 | `(unknown callee) 0x221f0` | `0x12270c` | `r0` | `r1` | `r2` |
| `bl` at 0xe006c | `(unknown callee) 0xdefac` | `?` | `?` | `?` | `?` |
| `bl` at 0xe00b8 | `(unknown callee) 0xe4488` | `[sp-40]` | `[sp-28]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xe0050 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xe0080 | `ret(0xdefac) != 0x0` | `ret(0xdefac) == 0x0` |
| `blo` at 0xe00d4 | `ret(0xe4488) < [[sp-40]+2]:2 (unsigned, 32-bit)` | `ret(0xe4488) >= [[sp-40]+2]:2 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe00f4 | `r0` | `?` |

## `jchip_dump_io`

- `jchip_dump_io`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdf464 to 0xdf544.
  Established. Included unnamed `0xe8e78` calls carry `0x0`, `sp-72`, unknown, `[sp-32]` and `0x0`, `sp-72`, unknown, `[sp]` (`bl` at 0xdf4d8, `bl` at 0xdf530).
  A word read addresses `sp +0x0` (`ldr` at 0xdf52c).
  The integer return candidate is `ret(0xe8e78)` (`ldm` at 0xdf538).
  Dump destination interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdf474 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdf478 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xdf47c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xdf480 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xdf48c | load | 4 | `abs 0xdf53c` | `-` |
| `ldr` at 0xdf490 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdf494 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdf4a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdf4b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdf4d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdf4e4 | load | 4 | `abs 0xdf540` | `-` |
| `ldr` at 0xdf4e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdf4ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdf4f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdf50c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdf52c | load | 4 | `sp +0x0` | `-` |
| `ldm` at 0xdf538 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdf538 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdf538 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdf498 | `(unknown callee) 0x197a8` | `sp-72` | `0x122460` | `r0` | `r1` |
| `bl` at 0xdf4d8 | `(unknown callee) 0xe8e78` | `0x0` | `sp-72` | `?` | `[sp-32]` |
| `bl` at 0xdf4f0 | `(unknown callee) 0x197a8` | `sp-72` | `0x12246c` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xdf530 | `(unknown callee) 0xe8e78` | `0x0` | `sp-72` | `?` | `[sp]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdf538 | `r0` | `ret(0xe8e78)` |
