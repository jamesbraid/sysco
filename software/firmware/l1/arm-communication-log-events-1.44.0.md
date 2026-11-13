# ARM communication logs and event paths in 1.44.0

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
No unresolved computed transfers were reported for these routines. Undecodable byte ranges
remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/communication-log-events-facts.txt) retain all tool sections.

## `SysComLogInit_2m`

- `SysComLogInit_2m`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa25a8 to 0xa26d8.
  Established. A nonzero `[0x29a5e4]` takes `0xa25f0`, while zero falls through (`bne` at 0xa25c4).
  The absolute word at `0x133b68` receives `ret(0x1943c)` on an included path (`str` at 0xa260c).
  The unnamed `0xacd00` call carries `0x1000`, `0x0`, `[0x29a5e4]`, unknown (`bl` at 0xa2670).
  Communication log allocation policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xa25b8 | load | 4 | `abs 0xa26bc` | `-` |
| `ldr` at 0xa25bc | load | 4 | `abs 0x29a5e4` | `-` |
| `ldr` at 0xa25c8 | load | 4 | `abs 0xa26c0` | `-` |
| `str` at 0xa25d0 | store | 4 | `abs 0x13383c` | `0x0` |
| `ldr` at 0xa25d4 | load | 4 | `abs 0xa26c4` | `-` |
| `str` at 0xa25dc | store | 4 | `abs 0x133840` | `0x0` |
| `ldr` at 0xa25e0 | load | 4 | `abs 0xa26c8` | `-` |
| `str` at 0xa25e8 | store | 4 | `abs 0x133844` | `0x0` |
| `ldr` at 0xa25f0 | load | 4 | `abs 0xa26cc` | `-` |
| `ldr` at 0xa25f4 | load | 4 | `abs 0xa26bc` | `-` |
| `ldr` at 0xa25f8 | load | 4 | `abs 0x29a5e4` | `-` |
| `str` at 0xa260c | store | 4 | `abs 0x133b68` | `ret(0x1943c)` |
| `ldr` at 0xa2610 | load | 4 | `abs 0x133b68` | `-` |
| `ldr` at 0xa261c | load | 4 | `abs 0xa26d0` | `-` |
| `ldr` at 0xa2624 | load | 4 | `abs 0xa26c0` | `-` |
| `str` at 0xa262c | store | 4 | `abs 0x13383c` | `0x0` |
| `ldr` at 0xa2630 | load | 4 | `abs 0xa26c4` | `-` |
| `str` at 0xa2638 | store | 4 | `abs 0x133840` | `0x0` |
| `ldr` at 0xa263c | load | 4 | `abs 0xa26c8` | `-` |
| `str` at 0xa2644 | store | 4 | `abs 0x133844` | `0x0` |
| `str` at 0xa2650 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xa2654 | load | 4 | `abs 0xa26bc` | `-` |
| `ldr` at 0xa2658 | load | 4 | `abs 0x29a5e4` | `-` |
| `ldr` at 0xa265c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa2678 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa2680 | load | 4 | `abs 0xa26cc` | `-` |
| `ldr` at 0xa2684 | load | 4 | `abs 0x133b68` | `-` |
| `str` at 0xa268c | store | 4 | `(unknown)` | `ret(0xacd00)` |
| `ldr` at 0xa2690 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa269c | load | 4 | `abs 0xa26d4` | `-` |
| `ldr` at 0xa26a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa26a8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa26b0 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldm` at 0xa26b8 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xa26b8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xa26b8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa26b8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa26b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa2604 | `(unknown callee) 0x1943c` | `?` | `r1` | `r2` | `?` |
| `bl` at 0xa2620 | `(unknown callee) 0x1e958` | `0x113238` | `?` | `?` | `[0x133b68]` |
| `bl` at 0xa2670 | `(unknown callee) 0xacd00` | `0x1000` | `0x0` | `[0x29a5e4]` | `?` |
| `bl` at 0xa26a4 | `(unknown callee) 0x1e958` | `0x113260` | `[sp-24]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa25c4 | `[0x29a5e4] != 0x0` | `[0x29a5e4] == 0x0` |
| `bne` at 0xa2618 | `[0x133b68] != 0x0` | `[0x133b68] == 0x0` |
| `ble` at 0xa2664 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xa2698 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa26b8 | `r0` | `?` |

## `SysComLogFetch_2m`

- `SysComLogFetch_2m`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa2738 to 0xa2984.
  Established. An included byte store clears `ret(0x1943c) +0x0` (`strb` at 0xa2864).
  The unnamed `0x5f954` call carries `(ret(0x19538) + [sp-36])`, `[[sp-24]+8]`, `[[sp-24]+16]`, `[[sp-24]+12]` (`bl` at 0xa28c0).
  Communication log fetch record formatting remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa2748 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa274c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa2764 | store | 4 | `sp -0x18` | `ret(0x6a0f8)` |
| `ldr` at 0xa2768 | load | 4 | `abs 0xa2978` | `-` |
| `ldr` at 0xa276c | load | 4 | `abs 0x29a5e4` | `-` |
| `str` at 0xa2780 | store | 4 | `sp -0x28` | `ret(0x1943c)` |
| `str` at 0xa2788 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xa278c | load | 4 | `abs 0xa2978` | `-` |
| `ldr` at 0xa2790 | load | 4 | `abs 0x29a5e4` | `-` |
| `ldr` at 0xa2794 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa27a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa27a8 | load | 4 | `abs 0xa297c` | `-` |
| `ldr` at 0xa27ac | load | 4 | `abs 0x133b68` | `-` |
| `ldr` at 0xa27b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa27c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa27cc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa27d4 | store | 4 | `(unknown)` | `ret(0xad294)` |
| `ldr` at 0xa27d8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa27e0 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0xa27ec | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xa27f4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xa27f8 | load | 4 | `abs 0xa2978` | `-` |
| `ldr` at 0xa27fc | load | 4 | `abs 0x29a5e4` | `-` |
| `ldr` at 0xa2800 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa280c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa2814 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa281c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa282c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa2838 | store | 4 | `sp -0x20` | `(ret(0x19538) + [sp-32])+80` |
| `ldr` at 0xa283c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa2844 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xa284c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa2858 | store | 4 | `sp -0x24` | `ret(0x1943c)` |
| `ldr` at 0xa285c | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xa2864 | store | 1 | `ret(0x1943c) +0x0` | `0x0` |
| `str` at 0xa286c | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xa2874 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xa2878 | load | 4 | `abs 0xa2978` | `-` |
| `ldr` at 0xa287c | load | 4 | `abs 0x29a5e4` | `-` |
| `ldr` at 0xa2880 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa288c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa289c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa28a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa28a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa28ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa28b4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xa28b8 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xa28bc | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xa28c4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa28d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa28e0 | load | 4 | `abs 0xa2980` | `-` |
| `ldr` at 0xa28e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa28ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa28f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa28fc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa2900 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa2908 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa2910 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa2918 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa2920 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa2928 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xa2930 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa2938 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa293c | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0xa2944 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa294c | load | 4 | `abs 0xa297c` | `-` |
| `ldr` at 0xa2950 | load | 4 | `abs 0x133b68` | `-` |
| `ldr` at 0xa2958 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa2968 | store | 4 | `sp -0x2c` | `ret(0xad294)` |
| `ldr` at 0xa296c | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0xa2974 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa2974 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa2974 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa275c | `(unknown callee) 0x6a0f8` | `0x0` | `r1` | `r2` | `r0` |
| `bl` at 0xa2778 | `(unknown callee) 0x1943c` | `?` | `?` | `?` | `?` |
| `bl` at 0xa27bc | `(unknown callee) 0xad294` | `?` | `0x0` | `?` | `?` |
| `bl` at 0xa2820 | `(unknown callee) 0x19538` | `?` | `?` | `?` | `?` |
| `bl` at 0xa2850 | `(unknown callee) 0x1943c` | `?` | `?` | `[0x29a5e4]` | `?` |
| `bl` at 0xa2890 | `(unknown callee) 0x19538` | `[sp-36]` | `?` | `[0x29a5e4]` | `?` |
| `bl` at 0xa28c0 | `(unknown callee) 0x5f954` | `(ret(0x19538) + [sp-36])` | `[[sp-24]+8]` | `[[sp-24]+16]` | `[[sp-24]+12]` |
| `bl` at 0xa28c8 | `(unknown callee) 0x19538` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0xa28e8 | `(unknown callee) 0x197a8` | `(ret(0x19538) + [sp-36])` | `0x113288` | `[sp-28]` | `(ret(0x19538) + [sp-36])` |
| `bl` at 0xa2904 | `(unknown callee) 0x195e0` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0xa291c | `(unknown callee) 0x1970c` | `?` | `?` | `?` | `?` |
| `bl` at 0xa2934 | `(unknown callee) 0x1970c` | `[sp-40]` | `?` | `[0x29a5e4]` | `?` |
| `bl` at 0xa2960 | `(unknown callee) 0xad294` | `?` | `0x0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa2754 | `Z == 0` | `not (Z == 0)` |
| `ble` at 0xa279c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ble` at 0xa2808 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ble` at 0xa2888 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa2974 | `r0` | `?` |

## `SysComLogInsert_2m`

- `SysComLogInsert_2m`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa26d8 to 0xa2738.
  Established. The absolute word at `0x133b68` is read (`ldr` at 0xa26f8);
  its nonzero value takes `0xa2708`, while zero falls through (`bne` at 0xa2700).
  The unnamed `0xace2c` call carries unknown, entry `r1`, entry `r2`, unknown (`bl` at 0xa2728).
  Log insertion semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa26e8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa26ec | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa26f0 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xa26f4 | load | 4 | `abs 0xa2734` | `-` |
| `ldr` at 0xa26f8 | load | 4 | `abs 0x133b68` | `-` |
| `ldr` at 0xa2708 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa2710 | load | 4 | `abs 0xa2734` | `-` |
| `ldr` at 0xa2714 | load | 4 | `abs 0x133b68` | `-` |
| `ldr` at 0xa271c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa2720 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa2724 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xa2730 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa2730 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa2730 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa2728 | `(unknown callee) 0xace2c` | `?` | `r1` | `r2` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa2700 | `[0x133b68] != 0x0` | `[0x133b68] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa2730 | `r0` | `?` |

## `SysComLogReset_2m`

- `SysComLogReset_2m`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa2984 to 0xa2a18.
  Established. The absolute word at `0x29a5e4` is read (`ldr` at 0xa29b0).
  An included stack word at `sp -0x18` receives `[sp-24]+1` (`str` at 0xa29e4).
  Separate calls reach the unnamed `0xacde4` with an unknown first candidate (`bl` at 0xa29d8, `bl` at 0xa2a04).
  Log reset selection remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa2994 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa2998 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa29a8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xa29ac | load | 4 | `abs 0xa2a10` | `-` |
| `ldr` at 0xa29b0 | load | 4 | `abs 0x29a5e4` | `-` |
| `ldr` at 0xa29b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa29c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa29c8 | load | 4 | `abs 0xa2a14` | `-` |
| `ldr` at 0xa29cc | load | 4 | `abs 0x133b68` | `-` |
| `ldr` at 0xa29d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa29dc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa29e4 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0xa29ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa29f4 | load | 4 | `abs 0xa2a14` | `-` |
| `ldr` at 0xa29f8 | load | 4 | `abs 0x133b68` | `-` |
| `ldr` at 0xa2a00 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xa2a0c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa2a0c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa2a0c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa29d8 | `(unknown callee) 0xacde4` | `?` | `?` | `?` | `?` |
| `bl` at 0xa2a04 | `(unknown callee) 0xacde4` | `?` | `r1` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa29a0 | `Z == 0` | `not (Z == 0)` |
| `ble` at 0xa29bc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa2a0c | `r0` | `?` |

## `SysCom_2m_Init`

- `SysCom_2m_Init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa2a18 to 0xa2b68.
  Established. Equality `[sp-20] == 0x43` takes `0xa2b14`, while inequality falls through (`beq` at 0xa2a40).
  Included absolute word stores write `0xa25a8` to `0x133838` and `0xa2984` to `0x133844` (`str` at 0xa2b1c, `str` at 0xa2b40).
  The values are stored addresses.
  Communication callback use remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa2a34 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xa2a38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa2a44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa2a50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa2a5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa2a68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa2a74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa2a80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa2a8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa2a98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa2aa4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa2ab0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa2abc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa2ac8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa2ad4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa2ae0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa2aec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa2af8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa2b04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa2b14 | load | 4 | `abs 0xa2b48` | `-` |
| `ldr` at 0xa2b18 | load | 4 | `abs 0xa2b4c` | `-` |
| `str` at 0xa2b1c | store | 4 | `abs 0x133838` | `0xa25a8` |
| `ldr` at 0xa2b20 | load | 4 | `abs 0xa2b50` | `-` |
| `ldr` at 0xa2b24 | load | 4 | `abs 0xa2b54` | `-` |
| `str` at 0xa2b28 | store | 4 | `abs 0x13383c` | `0xa26d8` |
| `ldr` at 0xa2b2c | load | 4 | `abs 0xa2b58` | `-` |
| `ldr` at 0xa2b30 | load | 4 | `abs 0xa2b5c` | `-` |
| `str` at 0xa2b34 | store | 4 | `abs 0x133840` | `0xa2738` |
| `ldr` at 0xa2b38 | load | 4 | `abs 0xa2b60` | `-` |
| `ldr` at 0xa2b3c | load | 4 | `abs 0xa2b64` | `-` |
| `str` at 0xa2b40 | store | 4 | `abs 0x133844` | `0xa2984` |
| `ldm` at 0xa2b44 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xa2b44 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa2b44 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa2b44 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa2a28 | `(unknown callee) 0x488cc` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa2a40 | `[sp-20] == 0x43` | `[sp-20] != 0x43` |
| `beq` at 0xa2a4c | `[sp-20] == 0x33` | `[sp-20] != 0x33` |
| `beq` at 0xa2a58 | `[sp-20] == 0x35` | `[sp-20] != 0x35` |
| `beq` at 0xa2a64 | `[sp-20] == 0x34` | `[sp-20] != 0x34` |
| `beq` at 0xa2a70 | `[sp-20] == 0x41` | `[sp-20] != 0x41` |
| `beq` at 0xa2a7c | `[sp-20] == 0x36` | `[sp-20] != 0x36` |
| `beq` at 0xa2a88 | `[sp-20] == 0x39` | `[sp-20] != 0x39` |
| `beq` at 0xa2a94 | `[sp-20] == 0x37` | `[sp-20] != 0x37` |
| `beq` at 0xa2aa0 | `[sp-20] == 0x38` | `[sp-20] != 0x38` |
| `beq` at 0xa2aac | `[sp-20] == 0x30` | `[sp-20] != 0x30` |
| `beq` at 0xa2ab8 | `[sp-20] == 0x51` | `[sp-20] != 0x51` |
| `beq` at 0xa2ac4 | `[sp-20] == 0x40` | `[sp-20] != 0x40` |
| `beq` at 0xa2ad0 | `[sp-20] == 0x4f` | `[sp-20] != 0x4f` |
| `beq` at 0xa2adc | `[sp-20] == 0x42` | `[sp-20] != 0x42` |
| `beq` at 0xa2ae8 | `[sp-20] == 0x46` | `[sp-20] != 0x46` |
| `beq` at 0xa2af4 | `[sp-20] == 0x4c` | `[sp-20] != 0x4c` |
| `beq` at 0xa2b00 | `[sp-20] == 0x4d` | `[sp-20] != 0x4d` |
| `beq` at 0xa2b0c | `[sp-20] == 0x7c` | `[sp-20] != 0x7c` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa2b44 | `r0` | `ret(0x488cc)` |

## `TIOSysComInt`

- `TIOSysComInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4bbf0 to 0x4bcfc.
  Established. Entry candidates `r1`, `r2`, `r3` are saved as bytes at `sp -0x15`, `sp -0x16`, `sp -0x17` (`strb` at 0x4bc0c, `strb` at 0x4bc14, `strb` at 0x4bc1c).
  The absolute byte at `0x299842` is read (`ldrb` at 0x4bc2c).
  The unnamed `0x78534` call carries unknown, unknown, `(r3 & 0xff)`, unknown (`bl` at 0x4bcdc).
  Controller interrupt routing remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4bc00 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x4bc0c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x4bc14 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x4bc1c | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `str` at 0x4bc24 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x4bc28 | load | 4 | `abs 0x4bce8` | `-` |
| `ldrb` at 0x4bc2c | load | 1 | `abs 0x299842` | `-` |
| `ldr` at 0x4bc30 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4bc3c | load | 4 | `abs 0x4bcec` | `-` |
| `str` at 0x4bc40 | store | 4 | `sp -0x28` | `0xf83d0` |
| `ldrb` at 0x4bc44 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x4bc48 | store | 4 | `sp -0x24` | `(r1 & 0xff)` |
| `ldr` at 0x4bc50 | load | 4 | `abs 0x4bcf0` | `-` |
| `ldr` at 0x4bc54 | load | 4 | `abs 0x4bcf4` | `-` |
| `ldr` at 0x4bc68 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4bc84 | load | 4 | `abs 0x4bcf8` | `-` |
| `ldr` at 0x4bc88 | load | 4 | `abs 0x132490` | `-` |
| `str` at 0x4bc94 | store | 4 | `sp -0x20` | `?` |
| `str` at 0x4bc98 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x4bc9c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4bca0 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x4bca4 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x4bcb4 | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0x4bcb8 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x4bcd0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4bcd4 | load | 1 | `(unknown)` | `-` |
| `ldm` at 0x4bce4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4bce4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4bce4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4bc5c | `(unknown callee) 0x200a0` | `0x1` | `0xf8308` | `0x241` | `0x0` |
| `bl` at 0x4bcdc | `(unknown callee) 0x78534` | `?` | `?` | `(r3 & 0xff)` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0x4bc38 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x4bcac | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x4bccc | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4bce4 | `r0` | `?` |

## `NL4R_SysComEnable`

- `NL4R_SysComEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2958c to 0x29688.
  Established. Entry `r0 != 0xff` takes `0x29610`, while equality falls through (`bne` at 0x295a8).
  The signed condition `r0 < [0x132094]` takes `0x29630`, while greater or equal values fall through (`blt` at 0x29620).
  The unnamed `0xca8c4` call carries `0x0`, `[[sp-24]+2]:1`, `0x2`, `[[sp-24]+2]:1` at two sites (`bl` at 0x295fc, `bl` at 0x29664).
  Link enable selection remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2959c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x295a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x295b0 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x295b4 | load | 4 | `abs 0x2967c` | `-` |
| `ldr` at 0x295b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x295bc | load | 4 | `abs 0x132094` | `-` |
| `ldr` at 0x295c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x295d0 | load | 4 | `abs 0x29680` | `-` |
| `ldr` at 0x295d4 | load | 4 | `abs 0x132090` | `-` |
| `str` at 0x295dc | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x295e0 | load | 4 | `abs 0x29684` | `-` |
| `ldr` at 0x295e8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x295ec | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0x29600 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x29608 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0x29610 | load | 4 | `abs 0x2967c` | `-` |
| `ldr` at 0x29614 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29618 | load | 4 | `abs 0x132094` | `-` |
| `str` at 0x29628 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x29630 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29638 | load | 4 | `abs 0x29680` | `-` |
| `ldr` at 0x2963c | load | 4 | `abs 0x132090` | `-` |
| `str` at 0x29644 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x29648 | load | 4 | `abs 0x29684` | `-` |
| `ldr` at 0x29650 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x29654 | load | 1 | `[sp-24] +0x2` | `-` |
| `str` at 0x2966c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x29670 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x29678 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x29678 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x29678 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x295e4 | `(unknown callee) 0x1e958` | `0xf118c` | `?` | `?` | `?` |
| `bl` at 0x295fc | `(unknown callee) 0xca8c4` | `0x0` | `[[sp-24]+2]:1` | `0x2` | `[[sp-24]+2]:1` |
| `bl` at 0x2964c | `(unknown callee) 0x1e958` | `0xf118c` | `r1` | `?` | `?` |
| `bl` at 0x29664 | `(unknown callee) 0xca8c4` | `0x0` | `[[sp-24]+2]:1` | `0x2` | `[[sp-24]+2]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x295a8 | `r0 != 0xff` | `r0 == 0xff` |
| `bge` at 0x295c4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `blt` at 0x29620 | `r0 < [0x132094] (signed, 32-bit)` | `r0 >= [0x132094] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x29678 | `r0` | `[sp-28]` |

## `NL4R_SysComInt`

- `NL4R_SysComInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x29688 to 0x29784.
  Established. The absolute word at `0x132094` is read (`ldr` at 0x296c8).
  The byte through the loaded record at `0x29728` has an unknown effective target (`ldrb` at 0x29728).
  The unnamed `0x78534` call carries unknown, unknown, `(r3 & 0xff)`, unknown (`bl` at 0x29764).
  Link interrupt dispatch meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x29698 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x296a4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x296ac | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x296b4 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `str` at 0x296bc | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x296c0 | load | 4 | `abs 0x29770` | `-` |
| `ldr` at 0x296c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x296c8 | load | 4 | `abs 0x132094` | `-` |
| `ldr` at 0x296d4 | load | 4 | `abs 0x29774` | `-` |
| `str` at 0x296d8 | store | 4 | `sp -0x28` | `0xf119c` |
| `ldrb` at 0x296dc | load | 1 | `sp -0x15` | `-` |
| `str` at 0x296e0 | store | 4 | `sp -0x24` | `(r1 & 0xff)` |
| `ldr` at 0x296e8 | load | 4 | `abs 0x29778` | `-` |
| `ldr` at 0x296ec | load | 4 | `abs 0x2977c` | `-` |
| `ldr` at 0x29700 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2970c | load | 4 | `abs 0x29780` | `-` |
| `ldr` at 0x29710 | load | 4 | `abs 0x132090` | `-` |
| `str` at 0x2971c | store | 4 | `sp -0x20` | `?` |
| `str` at 0x29720 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x29724 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x29728 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x2972c | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x2973c | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0x29740 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x29758 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x2975c | load | 1 | `(unknown)` | `-` |
| `ldm` at 0x2976c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x2976c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x2976c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x296f4 | `(unknown callee) 0x200a0` | `0x1` | `0xf115c` | `0x129` | `0x0` |
| `bl` at 0x29764 | `(unknown callee) 0x78534` | `?` | `?` | `(r3 & 0xff)` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x296d0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x29734 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x29754 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x2976c | `r0` | `?` |

## `SysComEventEnabled`

- `SysComEventEnabled`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7a750 to 0x7a790.
  Established. The absolute word at `0x29a5e0` is read (`ldr` at 0x7a764).
  Separate stack word assignments at `sp -0x14` write zero and `0x1` (`str` at 0x7a774, `str` at 0x7a780).
  The included return record carries `[sp-20]` in `r0` (`ldm` at 0x7a788).
  Event enable-state interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x7a760 | load | 4 | `abs 0x7a78c` | `-` |
| `ldr` at 0x7a764 | load | 4 | `abs 0x29a5e0` | `-` |
| `str` at 0x7a774 | store | 4 | `sp -0x14` | `0x0` |
| `str` at 0x7a780 | store | 4 | `sp -0x14` | `0x1` |
| `ldr` at 0x7a784 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x7a788 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x7a788 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7a788 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7a788 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x7a76c | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7a788 | `r0` | `[sp-20]` |

## `SysComEventDisable`

- `SysComEventDisable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7a790 to 0x7a7bc.
  Established. A word store at `0x29a5e0` has an unknown value (`str` at 0x7a7a4).
  The unnamed `0x1b5f0` call carries `0x13`, `0x0`, `0x29a5d4`, unknown (`bl` at 0x7a7b0);
  its result appears in the included return record (`ldm` at 0x7a7b4).
  Event disable effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x7a79c | load | 4 | `abs 0x7a7b8` | `-` |
| `str` at 0x7a7a4 | store | 4 | `abs 0x29a5e0` | `?` |
| `ldm` at 0x7a7b4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7a7b4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7a7b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7a7b0 | `(unknown callee) 0x1b5f0` | `0x13` | `0x0` | `0x29a5d4` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7a7b4 | `r0` | `ret(0x1b5f0)` |

## `IRouterEvent`

- `IRouterEvent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaa20c to 0xaa2f4.
  Established. The absolute word at `0x133c20` receives `[0x133c20]+1` (`str` at 0xaa25c).
  The unnamed `0xa80e0` call carries `sp-36`, `0x2`, `0x0`, `[r0+4]:1` (`bl` at 0xaa29c);
  its zero result takes `0xaa2c0`, while nonzero falls through (`beq` at 0xaa2b0).
  Event packet submission remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaa21c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xaa220 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xaa224 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xaa228 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xaa230 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaa234 | load | 1 | `r0 +0x4` | `-` |
| `ldr` at 0xaa238 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xaa23c | store | 4 | `sp -0x50` | `r1` |
| `str` at 0xaa244 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0xaa248 | load | 4 | `abs 0xaa2f0` | `-` |
| `ldr` at 0xaa24c | load | 4 | `abs 0x133c20` | `-` |
| `str` at 0xaa254 | store | 4 | `sp -0x48` | `[0x133c20]` |
| `str` at 0xaa25c | store | 4 | `abs 0x133c20` | `[0x133c20]+1` |
| `str` at 0xaa264 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xaa268 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaa26c | store | 4 | `sp -0x40` | `[sp-20]` |
| `ldr` at 0xaa270 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xaa274 | store | 4 | `sp -0x3c` | `[sp-28]` |
| `ldr` at 0xaa278 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xaa27c | store | 4 | `sp -0x38` | `[sp-32]` |
| `str` at 0xaa284 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xaa288 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xaa28c | store | 4 | `sp -0x30` | `[sp]` |
| `str` at 0xaa2a4 | store | 4 | `sp -0x28` | `ret(0xa80e0)` |
| `ldr` at 0xaa2a8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaa2b4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xaa2b8 | store | 4 | `sp -0x2c` | `ret(0xa80e0)` |
| `ldr` at 0xaa2c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaa2c4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xaa2d0 | store | 4 | `sp -0x28` | `ret(0xa98b0)` |
| `ldr` at 0xaa2d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xaa2dc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xaa2e0 | store | 4 | `sp -0x2c` | `[sp-40]` |
| `ldr` at 0xaa2e4 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0xaa2ec | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xaa2ec | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xaa2ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaa29c | `(unknown callee) 0xa80e0` | `sp-36` | `0x2` | `0x0` | `[r0+4]:1` |
| `bl` at 0xaa2c8 | `(unknown callee) 0xa98b0` | `[sp-20]` | `[sp-36]` | `?` | `ret(0xa80e0)` |
| `bl` at 0xaa2d8 | `(unknown callee) 0xa8b98` | `[sp-36]` | `?` | `?` | `ret(0xa98b0)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xaa2b0 | `ret(0xa80e0) == 0x0` | `ret(0xa80e0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xaa2ec | `r0` | `?` |

## `IRouterCommand`

- `IRouterCommand`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaa2f4 to 0xaa424.
  Established. The unnamed `0xa9f4c` call retains entry `r0`, entry `r1`, entry `r2`, entry `r3` (`bl` at 0xaa344);
  its zero result takes `0xaa368`, while nonzero falls through (`beq` at 0xaa358).
  An included word assignment at `sp -0x30` writes `0xfffffd96` (`str` at 0xaa3e8).
  Command response matching remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaa304 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xaa308 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xaa30c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xaa310 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xaa314 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xaa318 | store | 4 | `sp -0x40` | `[sp]` |
| `ldr` at 0xaa31c | load | 4 | `sp +0x4` | `-` |
| `str` at 0xaa320 | store | 4 | `sp -0x3c` | `[sp+4]` |
| `str` at 0xaa328 | store | 4 | `sp -0x38` | `sp-40` |
| `str` at 0xaa330 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xaa334 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaa338 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaa33c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaa340 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xaa34c | store | 4 | `sp -0x24` | `ret(0xa9f4c)` |
| `ldr` at 0xaa350 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xaa35c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xaa360 | store | 4 | `sp -0x30` | `ret(0xa9f4c)` |
| `ldr` at 0xaa368 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xaa36c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xaa370 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xaa37c | store | 4 | `sp -0x24` | `ret(0xa8b38)` |
| `ldr` at 0xaa380 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xaa38c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xaa390 | store | 4 | `sp -0x30` | `ret(0xa8b38)` |
| `ldr` at 0xaa398 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xaa39c | load | 4 | `[sp+8] +0x0` | `-` |
| `ldrb` at 0xaa3a0 | load | 1 | `[[sp+8]] +0x14` | `-` |
| `ldr` at 0xaa3a4 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xaa3a8 | load | 4 | `[sp+8] +0x0` | `-` |
| `ldrb` at 0xaa3ac | load | 1 | `[[sp+8]] +0x21` | `-` |
| `str` at 0xaa3b8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xaa3bc | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xaa3c0 | load | 4 | `[sp+8] +0x0` | `-` |
| `ldrb` at 0xaa3c4 | load | 1 | `[[sp+8]] +0x12` | `-` |
| `ldr` at 0xaa3d0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaa3d8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xaa3e4 | load | 4 | `abs 0xaa420` | `-` |
| `str` at 0xaa3e8 | store | 4 | `sp -0x30` | `0xfffffd96` |
| `ldr` at 0xaa3f0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaa3f4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xaa400 | load | 4 | `abs 0xaa420` | `-` |
| `str` at 0xaa404 | store | 4 | `sp -0x30` | `0xfffffd96` |
| `str` at 0xaa410 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xaa414 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0xaa41c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xaa41c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xaa41c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaa344 | `(unknown callee) 0xa9f4c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xaa374 | `(unknown callee) 0xa8b38` | `[sp+8]` | `[sp+12]` | `[sp+16]` | `ret(0xa9f4c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xaa358 | `ret(0xa9f4c) == 0x0` | `ret(0xa9f4c) != 0x0` |
| `beq` at 0xaa388 | `ret(0xa8b38) == 0x0` | `ret(0xa8b38) != 0x0` |
| `bne` at 0xaa3cc | `[[[sp+8]]+18]:1 != 0x0` | `[[[sp+8]]+18]:1 == 0x0` |
| `beq` at 0xaa3e0 | `? == [sp-44]` | `? != [sp-44]` |
| `beq` at 0xaa3fc | `[sp-40] == [sp-44]` | `[sp-40] != [sp-44]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xaa41c | `r0` | `?` |

## `IRouterPacketUseCount`

- `IRouterPacketUseCount`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa6eb0 to 0xa7020.
  Established. The halfword through `[sp-20] +0x4` receives `([sp-32]:2 & 0xffff)` (`strh` at 0xa6f60).
  A zero `ret(0x2105c)` takes `0xa6ff8`, while nonzero falls through (`beq` at 0xa6fc8).
  Packet reference-count encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa6ec0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa6ec4 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xa6ec8 | load | 4 | `abs 0xa7000` | `-` |
| `str` at 0xa6ed8 | store | 4 | `sp -0x1c` | `ret(0x20cd0)` |
| `ldr` at 0xa6edc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa6ee8 | load | 4 | `abs 0xa7004` | `-` |
| `str` at 0xa6eec | store | 4 | `sp -0x2c` | `0x113e78` |
| `ldr` at 0xa6ef0 | load | 4 | `abs 0xa7000` | `-` |
| `str` at 0xa6ef4 | store | 4 | `sp -0x28` | `0x133c78` |
| `ldr` at 0xa6ef8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa6efc | store | 4 | `sp -0x24` | `ret(0x20cd0)` |
| `ldr` at 0xa6f04 | load | 4 | `abs 0xa7008` | `-` |
| `ldr` at 0xa6f14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa6f18 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xa6f1c | load | 2 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xa6f24 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xa6f34 | load | 2 | `[sp-20] +0x4` | `-` |
| `strh` at 0xa6f38 | store | 2 | `sp -0x20` | `([[sp-20]+4]:2 & 0xffff)` |
| `ldr` at 0xa6f3c | load | 4 | `abs 0xa700c` | `-` |
| `ldrh` at 0xa6f48 | load | 2 | `sp -0x20` | `-` |
| `strh` at 0xa6f58 | store | 2 | `sp -0x20` | `?` |
| `ldrh` at 0xa6f5c | load | 2 | `sp -0x20` | `-` |
| `strh` at 0xa6f60 | store | 2 | `[sp-20] +0x4` | `([sp-32]:2 & 0xffff)` |
| `ldr` at 0xa6f64 | load | 4 | `abs 0xa7010` | `-` |
| `ldr` at 0xa6f68 | load | 4 | `abs 0x133c1c` | `-` |
| `ldr` at 0xa6f74 | load | 4 | `abs 0xa7010` | `-` |
| `ldr` at 0xa6f78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa6f7c | load | 4 | `abs 0x133c1c` | `-` |
| `ldr` at 0xa6f88 | load | 4 | `abs 0xa7014` | `-` |
| `ldr` at 0xa6f8c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xa6f90 | load | 2 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xa6fa0 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0xa6fa4 | load | 4 | `abs 0xa7018` | `-` |
| `ldr` at 0xa6fa8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa6fb0 | load | 4 | `abs 0xa7000` | `-` |
| `str` at 0xa6fbc | store | 4 | `sp -0x1c` | `ret(0x2105c)` |
| `ldr` at 0xa6fc0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa6fcc | load | 4 | `abs 0xa701c` | `-` |
| `str` at 0xa6fd0 | store | 4 | `sp -0x2c` | `0x113ed0` |
| `ldr` at 0xa6fd4 | load | 4 | `abs 0xa7000` | `-` |
| `str` at 0xa6fd8 | store | 4 | `sp -0x28` | `0x133c78` |
| `ldr` at 0xa6fdc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa6fe0 | store | 4 | `sp -0x24` | `ret(0x2105c)` |
| `ldr` at 0xa6fe8 | load | 4 | `abs 0xa7008` | `-` |
| `ldm` at 0xa6ffc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa6ffc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa6ffc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa6ed0 | `(unknown callee) 0x20cd0` | `0x133c78` | `?` | `r2` | `r3` |
| `bl` at 0xa6f10 | `(unknown callee) 0x200a0` | `0x9` | `0x113ea4` | `0x52` | `0x0` |
| `bl` at 0xa6fac | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0x113ec4` | `[sp-20]` | `?` |
| `bl` at 0xa6fb4 | `(unknown callee) 0x2105c` | `0x133c78` | `?` | `?` | `?` |
| `bl` at 0xa6ff4 | `(unknown callee) 0x200a0` | `0x9` | `0x113ea4` | `0x5a` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa6ee4 | `ret(0x20cd0) == 0x0` | `ret(0x20cd0) != 0x0` |
| `beq` at 0xa6f70 | `[0x133c1c] == 0x0` | `[0x133c1c] != 0x0` |
| `bne` at 0xa6f84 | `[sp-20] != [0x133c1c]` | `[sp-20] == [0x133c1c]` |
| `beq` at 0xa6fc8 | `ret(0x2105c) == 0x0` | `ret(0x2105c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa6ffc | `r0` | `?` |

## `IRouterPacketCount`

- `IRouterPacketCount`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa7e88 to 0xa7fb4.
  Established. The absolute words at `0x133c10` and `0x133c14` receive `([0x133c10] + [sp-20])` and `([0x133c14] + [sp-20])` (`str` at 0xa7efc, `str` at 0xa7f14).
  The signed condition `[0x133c14] <= [0x133c18]` takes `0xa7f40`, while greater values fall through (`ble` at 0xa7f2c).
  An included word store writes `[0x133c14]` to `0x133c18` (`str` at 0xa7f3c).
  Packet counter roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa7e98 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa7e9c | load | 4 | `abs 0xa7f90` | `-` |
| `str` at 0xa7eac | store | 4 | `sp -0x18` | `ret(0x20cd0)` |
| `ldr` at 0xa7eb0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7ebc | load | 4 | `abs 0xa7f94` | `-` |
| `str` at 0xa7ec0 | store | 4 | `sp -0x24` | `0x113e78` |
| `ldr` at 0xa7ec4 | load | 4 | `abs 0xa7f90` | `-` |
| `str` at 0xa7ec8 | store | 4 | `sp -0x20` | `0x133c78` |
| `ldr` at 0xa7ecc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa7ed0 | store | 4 | `sp -0x1c` | `ret(0x20cd0)` |
| `ldr` at 0xa7ed8 | load | 4 | `abs 0xa7f98` | `-` |
| `ldr` at 0xa7edc | load | 4 | `abs 0xa7f9c` | `-` |
| `ldr` at 0xa7ee8 | load | 4 | `abs 0xa7fa0` | `-` |
| `ldr` at 0xa7eec | load | 4 | `abs 0xa7fa0` | `-` |
| `ldr` at 0xa7ef0 | load | 4 | `abs 0x133c10` | `-` |
| `ldr` at 0xa7ef4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa7efc | store | 4 | `abs 0x133c10` | `([0x133c10] + [sp-20])` |
| `ldr` at 0xa7f00 | load | 4 | `abs 0xa7fa4` | `-` |
| `ldr` at 0xa7f04 | load | 4 | `abs 0xa7fa4` | `-` |
| `ldr` at 0xa7f08 | load | 4 | `abs 0x133c14` | `-` |
| `ldr` at 0xa7f0c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa7f14 | store | 4 | `abs 0x133c14` | `([0x133c14] + [sp-20])` |
| `ldr` at 0xa7f18 | load | 4 | `abs 0xa7fa4` | `-` |
| `ldr` at 0xa7f1c | load | 4 | `abs 0xa7fa8` | `-` |
| `ldr` at 0xa7f20 | load | 4 | `abs 0x133c14` | `-` |
| `ldr` at 0xa7f24 | load | 4 | `abs 0x133c18` | `-` |
| `ldr` at 0xa7f30 | load | 4 | `abs 0xa7fa8` | `-` |
| `ldr` at 0xa7f34 | load | 4 | `abs 0xa7fa4` | `-` |
| `ldr` at 0xa7f38 | load | 4 | `abs 0x133c14` | `-` |
| `str` at 0xa7f3c | store | 4 | `abs 0x133c18` | `[0x133c14]` |
| `ldr` at 0xa7f40 | load | 4 | `abs 0xa7f90` | `-` |
| `str` at 0xa7f4c | store | 4 | `sp -0x18` | `ret(0x2105c)` |
| `ldr` at 0xa7f50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7f5c | load | 4 | `abs 0xa7fac` | `-` |
| `str` at 0xa7f60 | store | 4 | `sp -0x24` | `0x113ed0` |
| `ldr` at 0xa7f64 | load | 4 | `abs 0xa7f90` | `-` |
| `str` at 0xa7f68 | store | 4 | `sp -0x20` | `0x133c78` |
| `ldr` at 0xa7f6c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa7f70 | store | 4 | `sp -0x1c` | `ret(0x2105c)` |
| `ldr` at 0xa7f78 | load | 4 | `abs 0xa7f98` | `-` |
| `ldr` at 0xa7f7c | load | 4 | `abs 0xa7fb0` | `-` |
| `ldm` at 0xa7f8c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa7f8c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa7f8c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa7ea4 | `(unknown callee) 0x20cd0` | `0x133c78` | `?` | `r2` | `r3` |
| `bl` at 0xa7ee4 | `(unknown callee) 0x200a0` | `0x9` | `0x113ea4` | `0x1b5` | `0x0` |
| `bl` at 0xa7f44 | `(unknown callee) 0x2105c` | `0x133c78` | `[0x133c14]` | `?` | `?` |
| `bl` at 0xa7f84 | `(unknown callee) 0x200a0` | `0x9` | `0x113ea4` | `0x1bd` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa7eb8 | `ret(0x20cd0) == 0x0` | `ret(0x20cd0) != 0x0` |
| `ble` at 0xa7f2c | `[0x133c14] <= [0x133c18] (signed, 32-bit)` | `[0x133c14] > [0x133c18] (signed, 32-bit)` |
| `beq` at 0xa7f58 | `ret(0x2105c) == 0x0` | `ret(0x2105c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa7f8c | `r0` | `?` |

## `IRouterQPacket`

- `IRouterQPacket`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa9d18 to 0xa9f4c.
  Established. The condition `[[sp-24]+22]:1 != 0x2` takes `0xa9dd8`, while equality falls through (`bne` at 0xa9d6c).
  The unnamed `0x21964` call carries `[sp-20]`, `sp-24`, `[0x12d8c4]`, `0xa8fe8` (`bl` at 0xa9e08).
  Included stack word stores at `sp -0x28` write `0xfffffd9d` and `0xfffffd9e` (`str` at 0xa9e64, `str` at 0xa9ee4).
  Queue packet error policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa9d28 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa9d2c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa9d34 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xa9d48 | store | 4 | `(unknown)` | `0x0` |
| `str` at 0xa9d5c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xa9d60 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa9d64 | load | 1 | `[sp-24] +0x16` | `-` |
| `str` at 0xa9d74 | store | 4 | `sp -0x54` | `sp-24` |
| `ldr` at 0xa9d78 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xa9d7c | load | 2 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xa9d94 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xa9d98 | load | 2 | `[sp-24] +0x6` | `-` |
| `str` at 0xa9da8 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xa9db0 | load | 4 | `abs 0xa9f24` | `-` |
| `ldr` at 0xa9db4 | load | 4 | `abs 0x12d8c8` | `-` |
| `str` at 0xa9db8 | store | 4 | `sp -0x58` | `[0x12d8c8]` |
| `ldr` at 0xa9dbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa9dc0 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xa9dc4 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xa9dd0 | store | 4 | `sp -0x28` | `ret(0x21d50)` |
| `ldr` at 0xa9ddc | load | 4 | `abs 0xa9f28` | `-` |
| `ldr` at 0xa9de0 | load | 4 | `abs 0xa9f2c` | `-` |
| `str` at 0xa9de4 | store | 4 | `sp -0x74` | `0xa9130` |
| `str` at 0xa9dec | store | 4 | `sp -0x70` | `sp-32` |
| `str` at 0xa9df4 | store | 4 | `sp -0x6c` | `0x1` |
| `ldr` at 0xa9df8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa9e00 | load | 4 | `abs 0x12d8c4` | `-` |
| `ldr` at 0xa9e04 | load | 4 | `abs 0xa9f30` | `-` |
| `str` at 0xa9e10 | store | 4 | `sp -0x28` | `ret(0x21964)` |
| `ldr` at 0xa9e14 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa9e20 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa9e24 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0xa9e28 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xa9e34 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xa9e40 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xa9e50 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xa9e60 | load | 4 | `abs 0xa9f34` | `-` |
| `str` at 0xa9e64 | store | 4 | `sp -0x28` | `0xfffffd9d` |
| `str` at 0xa9e6c | store | 4 | `sp -0x5c` | `sp-80` |
| `ldr` at 0xa9e70 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa9e78 | store | 4 | `sp -0x60` | `[sp-20]+120` |
| `ldr` at 0xa9e7c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa9e80 | load | 1 | `[sp-24] +0x16` | `-` |
| `ldr` at 0xa9e8c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa9e90 | load | 1 | `[sp-24] +0x16` | `-` |
| `ldr` at 0xa9e9c | load | 4 | `abs 0xa9f38` | `-` |
| `str` at 0xa9ea0 | store | 4 | `sp -0x64` | `0x113fac` |
| `ldr` at 0xa9ea8 | load | 4 | `abs 0xa9f3c` | `-` |
| `str` at 0xa9eac | store | 4 | `sp -0x64` | `0x114054` |
| `ldr` at 0xa9eb4 | load | 4 | `abs 0xa9f40` | `-` |
| `str` at 0xa9eb8 | store | 4 | `sp -0x64` | `0x113fa8` |
| `ldr` at 0xa9ebc | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xa9ec0 | load | 4 | `abs 0xa9f44` | `-` |
| `ldr` at 0xa9ec4 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xa9ec8 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xa9ee0 | load | 4 | `abs 0xa9f48` | `-` |
| `str` at 0xa9ee4 | store | 4 | `sp -0x28` | `0xfffffd9e` |
| `ldr` at 0xa9ef4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa9f0c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa9f14 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xa9f20 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa9f20 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa9f20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa9dc8 | `(unknown callee) 0x21d50` | `[sp-20]` | `sp-24` | `?` | `?` |
| `bl` at 0xa9e08 | `(unknown callee) 0x21964` | `[sp-20]` | `sp-24` | `[0x12d8c4]` | `0xa8fe8` |
| `bl` at 0xa9ecc | `(unknown callee) 0x197a8` | `sp-80` | `0x114058` | `[sp-20]+120` | `?` |
| `bl` at 0xa9ed8 | `(unknown callee) 0x1e4ac` | `sp-80` | `?` | `?` | `sp-80` |
| `bl` at 0xa9f10 | `(unknown callee) 0xa8b98` | `?` | `sp-16` | `sp-16` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa9d6c | `[[sp-24]+22]:1 != 0x2` | `[[sp-24]+22]:1 == 0x2` |
| `beq` at 0xa9d90 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa9e1c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xa9e30 | `Z == 1` | `not (Z == 1)` |
| `bgt` at 0xa9e3c | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0xa9e48 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xa9e58 | `[sp-104] == 0x6e` | `[sp-104] != 0x6e` |
| `beq` at 0xa9e88 | `[[sp-24]+22]:1 == 0x0` | `[[sp-24]+22]:1 != 0x0` |
| `bne` at 0xa9e98 | `[[sp-24]+22]:1 != 0x1` | `[[sp-24]+22]:1 == 0x1` |
| `beq` at 0xa9efc | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa9f20 | `r0` | `[sp-40]` |
