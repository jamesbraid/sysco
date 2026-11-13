# MMSC VGA cursor and memory-access routines in the 6.4-origin image

Read from `6.4-origin/usr/cpu/firmware/mmscfw.bin`.
Image SHA-256: `b766e559f86078526916d8bd98bbb8079294ad774cce9177928873153c6fb09b`.
The x86 a.out link address is `0x108000`. The [load map](load-map.md) explains segment mapping.
Analysed spans have exclusive ends inferred from the next text symbol.
Some include literals and padding decoded linearly after returns. Only sites reached
by known control-flow edges contribute effects to the tables.
Repeated `ret(F)` labels can refer to distinct call results and do not prove equal addresses.

`sp` denotes the entry stack pointer and brackets denote loaded values. Calls use
the assumed cdecl32 convention with unknown arity. Printed candidates can be local values,
saved registers or stale stack words. Unknown candidates and effective addresses stay unknown.
The access tables show explicit memory operands, including stack locations, and omit
implicit push/pop accesses. Full facts retain those accesses. Call and return instructions
remain listed without separate implicit stack access rows.
Unsupported instructions and partial-register tracking can leave results unresolved.
An analysis unknown does not establish indeterminate firmware behavior.
Field meanings, hardware roles, command semantics and callee effects remain unknown
unless explicitly established by an entry.

[Fact sheets and complete listings](disasm/vga-cursor-64-origin-facts.txt) include all tool sections.

## `_vgaHrdInit`

- `_vgaHrdInit`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108ee8 to 0x108f58.
  Established. The two `_sysOutByte` sites present leading pairs `0x3d4`, `0xa` (`call` at 0x108ef2)
  and `0x3d4`, `0xb` (`call` at 0x108f14). Byte stores at absolute `0x1d2728` (`mov` at 0x108f07)
  and `0x1d272c` (`mov` at 0x108f29) retain unknown values. The `_vgaClear` call presents
  `[0x1a9ae0]`, `0x2`, and `0x20` as its first three candidates (`call` at 0x108f3e). No
  display-register identities or clear extent are inferred.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x108f07 | store | 1 | `abs 0x1d2728` | `?` |
| `mov` at 0x108f29 | store | 1 | `abs 0x1d272c` | `?` |
| `mov` at 0x108f38 | load | 4 | `abs 0x1a9ae0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108ef2 | `_sysOutByte 0x1081d0` | `0x3d4` | `0xa` | `ebp` | `?` | `?` | `?` |
| `call` at 0x108eff | `_sysInByte 0x1081b0` | `0x3d5` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108f14 | `_sysOutByte 0x1081d0` | `0x3d4` | `0xb` | `?` | `?` | `?` | `?` |
| `call` at 0x108f21 | `_sysInByte 0x1081b0` | `0x3d5` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108f2f | `_vgaStatInit 0x108f58` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108f3e | `_vgaClear 0x109208` | `[0x1a9ae0]` | `0x2` | `0x20` | `?` | `?` | `?` |
| `call` at 0x108f46 | `_vgaCursorOn 0x109708` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108f4e | `eax` | `ret(_vgaCursorOn)` |

## `_vgaConBeep`

- `_vgaConBeep`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108e08 to 0x108ee8.
  Established. The initial branch is taken when `[sp+4] == 0x0` (`je` at 0x108e15). The
  `_sysOutByte` site presents `0x43` and `0xffffffb6` as its first two candidates (`call` at
  0x108e6b). Other sites present candidate zero `0x42` with unknown candidate one (`call` at
  0x108e79, `call` at 0x108e89), or `0x61` with unknown candidate one (`call` at 0x108ea3, `call` at
  0x108ec6). The `_taskDelay` site retains unknown candidates (`call` at 0x108eac). No tone
  frequency or duration is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x108e11 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x108e2f | store | 4 | `sp -0x8` | `?` |
| `sub` at 0x108e34 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108e1c | `_sysClkRateGet 0x1085a8` | `ebx` | `esi` | `edi` | `?` | `ebp` | `?` |
| `call` at 0x108e4d | `_sysClkRateGet 0x1085a8` | `ebx` | `esi` | `edi` | `?` | `ebp` | `?` |
| `call` at 0x108e60 | `_intLock 0x16fb78` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108e6b | `_sysOutByte 0x1081d0` | `0x43` | `0xffffffb6` | `?` | `?` | `?` | `?` |
| `call` at 0x108e79 | `_sysOutByte 0x1081d0` | `0x42` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108e89 | `_sysOutByte 0x1081d0` | `0x42` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108e93 | `_sysInByte 0x1081b0` | `0x61` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108ea3 | `_sysOutByte 0x1081d0` | `0x61` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108eac | `_taskDelay 0x170b80` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108eb6 | `_sysInByte 0x1081b0` | `0x61` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108ec6 | `_sysOutByte 0x1081d0` | `0x61` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108ecf | `_intUnlock 0x16fb88` | `ret(_intLock)` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x108e15 | `[sp+4] == 0x0` | `[sp+4] != 0x0` |
| `jge` at 0x108e56 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108edd | `eax` | `ret(_intUnlock)` |

## `_vgaStatInit`

- `_vgaStatInit`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108f58 to 0x109128.
  Established. Four-byte stores at unresolved effective addresses include values `0xb8000` (`mov` at
  0x108f86), `0x50` (`mov` at 0x109002), `0x19` (`mov` at 0x10900c), and `0x18` (`mov` at 0x109020).
  The `_bzero` sites present leading pairs `[sp-16]`, `0x40` (`call` at 0x109080) and `[sp-12]`,
  `0x50` (`call` at 0x10908e). Unknown targets prevent an established record layout or display
  geometry.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x108f61 | store | 4 | `sp -0x8` | `0x0` |
| `mov` at 0x108f68 | store | 4 | `sp -0xc` | `0x1d2578` |
| `mov` at 0x108f6f | store | 4 | `sp -0x10` | `0x1d25d0` |
| `mov` at 0x108f80 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x108f86 | store | 4 | `(unknown)` | `0xb8000` |
| `mov` at 0x108f90 | store | 4 | `(unknown)` | `0x1` |
| `mov` at 0x108f9a | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x108fa4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x108faa | store | 4 | `(unknown)` | `?` |
| `mov` at 0x108fb0 | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x108fba | load | 4 | `(unknown)` | `-` |
| `mov` at 0x108fc0 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x108fc6 | store | 1 | `(unknown)` | `0x1f` |
| `mov` at 0x108fcd | load | 1 | `(unknown)` | `-` |
| `mov` at 0x108fd3 | store | 1 | `(unknown)` | `?` |
| `mov` at 0x108fd9 | store | 1 | `(unknown)` | `0x1f` |
| `mov` at 0x108fe0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x108fe6 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x108fec | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x108ff6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x108ffc | store | 4 | `(unknown)` | `?` |
| `mov` at 0x109002 | store | 4 | `(unknown)` | `0x50` |
| `mov` at 0x10900c | store | 4 | `(unknown)` | `0x19` |
| `mov` at 0x109016 | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x109020 | store | 4 | `(unknown)` | `0x18` |
| `mov` at 0x10902a | store | 4 | `(unknown)` | `0x1` |
| `mov` at 0x109034 | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x10903e | load | 4 | `abs 0x1a9cc0` | `-` |
| `mov` at 0x109043 | store | 4 | `(unknown)` | `[0x1a9cc0]` |
| `mov` at 0x109049 | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x109053 | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x10905d | store | 2 | `(unknown)` | `0x1` |
| `mov` at 0x109066 | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x109070 | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x10907c | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x10908a | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x109096 | store | 1 | `(unknown)` | `0x1` |
| `mov` at 0x10909a | store | 1 | `(unknown)` | `0x1` |
| `mov` at 0x1090a1 | store | 1 | `(unknown)` | `0x1` |
| `mov` at 0x1090a8 | store | 1 | `(unknown)` | `0x1` |
| `mov` at 0x1090af | store | 1 | `(unknown)` | `0x1` |
| `mov` at 0x1090b6 | store | 1 | `(unknown)` | `0x1` |
| `mov` at 0x1090bd | store | 1 | `(unknown)` | `0x1` |
| `mov` at 0x1090c4 | store | 1 | `(unknown)` | `0x1` |
| `mov` at 0x1090cb | store | 1 | `(unknown)` | `0x1` |
| `mov` at 0x1090d2 | store | 1 | `(unknown)` | `0x1` |
| `mov` at 0x1090d9 | store | 4 | `(unknown)` | `0x109128` |
| `add` at 0x1090e3 | load | 4 | `sp -0xc` | `-` |
| `add` at 0x1090e3 | store | 4 | `sp -0xc` | `[sp-12]+260` |
| `add` at 0x1090ea | load | 4 | `sp -0x10` | `-` |
| `add` at 0x1090ea | store | 4 | `sp -0x10` | `[sp-16]+260` |
| `inc` at 0x109103 | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x109103 | store | 4 | `sp -0x8` | `?` |
| `cmp` at 0x109106 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x109080 | `_bzero 0x170f20` | `[sp-16]` | `0x40` | `?` | `?` | `?` | `?` |
| `call` at 0x10908e | `_bzero 0x170f20` | `[sp-12]` | `0x50` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x10910a | `[sp-8] <= 0x1 (signed, 32-bit)` | `[sp-8] > 0x1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x109119 | `eax` | `ret(_bzero)` |

## `_vgaClear`

- `_vgaClear`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x109208 to 0x1092a8.
  Established. The first branch falls through when `[sp+8] == 0x2` (`jne` at 0x109229). Four-byte
  stores through `[sp+4] +0xc` write `0x0` (`mov` at 0x109235), through `+0x10` retain `[[sp+4]+12]`
  (`mov` at 0x10923f), and through `+0x14` retain `[[sp+4]]` (`mov` at 0x109244). The two-byte store
  at an unknown effective address retains an unknown value (`mov` at 0x109288). A cleared screen
  interval is not inferred.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10920d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x109210 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x109213 | load | 1 | `[sp+4] +0x19` | `-` |
| `movzx` at 0x10921c | load | 1 | `sp +0xc` | `-` |
| `mov` at 0x10922b | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10922d | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x109235 | store | 4 | `[sp+4] +0xc` | `0x0` |
| `mov` at 0x10923c | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10923f | store | 4 | `[sp+4] +0x10` | `[[sp+4]+12]` |
| `mov` at 0x109242 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x109244 | store | 4 | `[sp+4] +0x14` | `[[sp+4]]` |
| `mov` at 0x10925c | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x10925f | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x109278 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10927a | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x109288 | store | 2 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x109229 | `[sp+8] != 0x2` | `[sp+8] == 0x2` |
| `jne` at 0x10925a | `([sp+8] & [sp+8]) != 0x0` | `([sp+8] & [sp+8]) == 0x0` |
| `jb` at 0x109290 | `? < ? (unsigned, 32-bit)` | `? >= ? (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10929a | `eax` | `?` |

## `_vgaCursorOn`

- `_vgaCursorOn`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x109708 to 0x109758.
  Established. The `_sysOutByte` sites present leading pairs `0x3d4`, `0xa` (`call` at 0x109712) and
  `0x3d4`, `0xb` (`call` at 0x10973a). The other two sites present candidate zero `0x3d5` with
  unknown candidate one (`call` at 0x10972b, `call` at 0x10974f). Byte loads read absolute
  `0x1d2728` (`mov` at 0x10971a) and `0x1d272c` (`movsx` at 0x109742). Cursor-enabling effects
  remain unassigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10971a | load | 1 | `abs 0x1d2728` | `-` |
| `movsx` at 0x109742 | load | 1 | `abs 0x1d272c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x109712 | `_sysOutByte 0x1081d0` | `0x3d4` | `0xa` | `ebp` | `?` | `?` | `?` |
| `call` at 0x10972b | `_sysOutByte 0x1081d0` | `0x3d5` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10973a | `_sysOutByte 0x1081d0` | `0x3d4` | `0xb` | `?` | `?` | `?` | `?` |
| `call` at 0x10974f | `_sysOutByte 0x1081d0` | `0x3d5` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x109757 | `eax` | `ret(_sysOutByte)` |

## `_vgaCursorOff`

- `_vgaCursorOff`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x109758 to 0x109798.
  Established. Four `_sysOutByte` sites present leading pairs `0x3d4`, `0xa` (`call` at 0x109762),
  `0x3d5`, `0x20` (`call` at 0x109771), `0x3d4`, `0xb` (`call` at 0x109780), and `0x3d5`, `0x0`
  (`call` at 0x10978f). The exit carries `ret(_sysOutByte)` in `eax` (`ret` at 0x109797). These
  candidates alone do not establish cursor-disabling semantics.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x109762 | `_sysOutByte 0x1081d0` | `0x3d4` | `0xa` | `ebp` | `?` | `?` | `?` |
| `call` at 0x109771 | `_sysOutByte 0x1081d0` | `0x3d5` | `0x20` | `?` | `?` | `?` | `?` |
| `call` at 0x109780 | `_sysOutByte 0x1081d0` | `0x3d4` | `0xb` | `?` | `?` | `?` | `?` |
| `call` at 0x10978f | `_sysOutByte 0x1081d0` | `0x3d5` | `0x0` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x109797 | `eax` | `ret(_sysOutByte)` |

## `_vgaCursorPos`

- `_vgaCursorPos`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1096a8 to 0x109708.
  Established. The two-byte load reads `sp +0x4` (`mov` at 0x1096ac). The `_sysOutByte` sites
  present leading pairs `0x3d4`, `0xe` (`call` at 0x1096b7) and `0x3d4`, `0xf` (`call` at 0x1096dd).
  Sites with candidate zero `0x3d5` retain unknown candidate one (`call` at 0x1096ce, `call` at
  0x1096ee). No cursor-address value is filled in from the raw instructions.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1096ac | load | 2 | `sp +0x4` | `-` |
| `mov` at 0x1096f3 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1096b7 | `_sysOutByte 0x1081d0` | `0x3d4` | `0xe` | `ebx` | `ebp` | `?` | `?` |
| `call` at 0x1096ce | `_sysOutByte 0x1081d0` | `0x3d5` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1096dd | `_sysOutByte 0x1081d0` | `0x3d4` | `0xf` | `?` | `?` | `?` | `?` |
| `call` at 0x1096ee | `_sysOutByte 0x1081d0` | `0x3d5` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1096f9 | `eax` | `ret(_sysOutByte)` |

## `_vgaScreenRev`

- `_vgaScreenRev`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1092a8 to 0x1092f8.
  Established. The byte stores at unknown effective addresses retain unknown values (`mov` at
  0x1092bf, `mov` at 0x1092ce, `mov` at 0x1092dc). The backward branch is taken when `C == 1` (`jb`
  at 0x1092eb). The exit reports `[[sp+4]]+4000` in `eax` (`ret` at 0x1092f5). The expression does
  not establish a screen size or colour transformation.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1092ad | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1092b0 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x1092b8 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x1092bf | store | 1 | `(unknown)` | `?` |
| `mov` at 0x1092c9 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x1092ce | store | 1 | `(unknown)` | `?` |
| `mov` at 0x1092d7 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x1092dc | store | 1 | `(unknown)` | `?` |
| `mov` at 0x1092e2 | load | 4 | `[sp+4] +0x0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jb` at 0x1092eb | `C == 1` | `not (C == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1092f5 | `eax` | `[[sp+4]]+4000` |

## `_vgaScroll`

- `_vgaScroll`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1092f8 to 0x1094c8.
  Established. The initial bounds branches are taken when `[sp+8] < [[sp+4]+36] (signed, 32-bit)`
  (`jl` at 0x109310) or `[sp+8] > [[sp+4]+40] (signed, 32-bit)` (`jg` at 0x10931c). Byte stores at
  unresolved targets include `0x20` (`mov` at 0x1093cc, `mov` at 0x10949d), `[sp-8]:1` (`mov` at
  0x1093d2, `mov` at 0x10949b), and `[[sp-12]]:1` (`mov` at 0x109389, `mov` at 0x10945d). No
  scrolling area or transfer count is assigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x109301 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x109304 | load | 1 | `sp +0x14` | `-` |
| `mov` at 0x109307 | store | 1 | `sp -0x8` | `[sp+20]:1` |
| `mov` at 0x10930a | load | 4 | `[sp+4] +0x24` | `-` |
| `cmp` at 0x10930d | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x109316 | load | 4 | `[sp+4] +0x28` | `-` |
| `cmp` at 0x109319 | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x109322 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x10932c | load | 4 | `[sp+4] +0x24` | `-` |
| `add` at 0x10932f | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x109332 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10933a | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10933d | load | 4 | `[sp+4] +0x24` | `-` |
| `mov` at 0x109343 | store | 4 | `sp +0xc` | `?` |
| `mov` at 0x109346 | load | 4 | `[sp+4] +0x24` | `-` |
| `mov` at 0x10934b | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x109351 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x109356 | load | 4 | `[sp+4] +0x24` | `-` |
| `add` at 0x109359 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10935e | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x109364 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x109368 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x10936b | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x109375 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x10937b | load | 4 | `[sp+4] +0x0` | `-` |
| `cmp` at 0x10937f | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x109384 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x109387 | load | 1 | `[sp-12] +0x0` | `-` |
| `mov` at 0x109389 | store | 1 | `(unknown)` | `[[sp-12]]:1` |
| `mov` at 0x10938c | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x109390 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x109396 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10939e | load | 4 | `sp +0x8` | `-` |
| `sub` at 0x1093a4 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1093a9 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x1093af | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x1093b8 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x1093be | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x1093cc | store | 1 | `(unknown)` | `0x20` |
| `mov` at 0x1093cf | load | 1 | `sp -0x8` | `-` |
| `mov` at 0x1093d2 | store | 1 | `(unknown)` | `[sp-8]:1` |
| `mov` at 0x1093d8 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x1093de | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x1093f8 | load | 4 | `[sp+4] +0x24` | `-` |
| `add` at 0x1093fb | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1093fe | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x109406 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x109409 | load | 4 | `[sp+4] +0x24` | `-` |
| `mov` at 0x10940f | store | 4 | `sp +0xc` | `?` |
| `mov` at 0x109412 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10941c | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x109422 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x109428 | load | 4 | `[sp+4] +0x28` | `-` |
| `sub` at 0x10942c | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x109431 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x109437 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10943d | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x109440 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x109447 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x10944d | load | 4 | `[sp+4] +0x0` | `-` |
| `cmp` at 0x109453 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x109458 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x10945b | load | 1 | `[sp-12] +0x0` | `-` |
| `mov` at 0x10945d | store | 1 | `(unknown)` | `[[sp-12]]:1` |
| `mov` at 0x109460 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x109464 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x10946a | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x109474 | load | 4 | `sp +0x8` | `-` |
| `add` at 0x109477 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10947c | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x109482 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x109488 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x109498 | load | 1 | `sp -0x8` | `-` |
| `mov` at 0x10949b | store | 1 | `(unknown)` | `[sp-8]:1` |
| `mov` at 0x10949d | store | 1 | `(unknown)` | `0x20` |
| `mov` at 0x1094a4 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x1094aa | load | 4 | `[sp+4] +0x0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jl` at 0x109310 | `[sp+8] < [[sp+4]+36] (signed, 32-bit)` | `[sp+8] >= [[sp+4]+36] (signed, 32-bit)` |
| `jg` at 0x10931c | `[sp+8] > [[sp+4]+40] (signed, 32-bit)` | `[sp+8] <= [[sp+4]+40] (signed, 32-bit)` |
| `je` at 0x109326 | `[sp+16] == 0x0` | `[sp+16] != 0x0` |
| `jle` at 0x109338 | `([[sp+4]+36] + [sp+12]) <= ? (signed, 32-bit)` | `([[sp+4]+36] + [sp+12]) > ? (signed, 32-bit)` |
| `jae` at 0x109382 | `[sp-12] >= ? (unsigned, 32-bit)` | `[sp-12] < ? (unsigned, 32-bit)` |
| `jb` at 0x10939c | `? < ? (unsigned, 32-bit)` | `? >= ? (unsigned, 32-bit)` |
| `jae` at 0x1093c4 | `? >= ? (unsigned, 32-bit)` | `? < ? (unsigned, 32-bit)` |
| `jb` at 0x1093e4 | `? < ? (unsigned, 32-bit)` | `? >= ? (unsigned, 32-bit)` |
| `jle` at 0x109404 | `([[sp+4]+36] + [sp+12]) <= ? (signed, 32-bit)` | `([[sp+4]+36] + [sp+12]) > ? (signed, 32-bit)` |
| `jbe` at 0x109456 | `[sp-12] <= ? (unsigned, 32-bit)` | `[sp-12] > ? (unsigned, 32-bit)` |
| `ja` at 0x109472 | `? > ? (unsigned, 32-bit)` | `? <= ? (unsigned, 32-bit)` |
| `ja` at 0x1094b2 | `? > ? (unsigned, 32-bit)` | `? <= ? (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1094bd | `eax` | `?` |

## `_vgaInsertChar`

- `_vgaInsertChar`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1094c8 to 0x109548.
  Established. The initial branch is taken when `[sp+8] <= [[sp+4]+32] (signed, 32-bit)` (`jle` at
  0x1094dc). A two-byte store at `sp -0x8` retains an unknown value (`mov` at 0x10950c). The
  two-byte store at an unresolved effective address writes `[sp-8]:2` (`mov` at 0x10951f). The exit
  carries `[[sp+4]+32]` (`ret` at 0x109545). Character-insertion semantics remain unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1094d1 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1094d4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1094d7 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x1094de | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x1094f8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1094fb | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x1094fe | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x109501 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x10950c | store | 2 | `sp -0x8` | `?` |
| `mov` at 0x109518 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x10951b | load | 2 | `sp -0x8` | `-` |
| `mov` at 0x10951f | store | 2 | `(unknown)` | `[sp-8]:2` |
| `mov` at 0x109522 | store | 2 | `sp -0x8` | `?` |
| `mov` at 0x10952c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10952f | load | 4 | `[sp+4] +0x20` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x1094dc | `[sp+8] <= [[sp+4]+32] (signed, 32-bit)` | `[sp+8] > [[sp+4]+32] (signed, 32-bit)` |
| `jne` at 0x1094ea | `([sp+8] & [sp+8]) != 0x0` | `([sp+8] & [sp+8]) == 0x0` |
| `jl` at 0x109534 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jne` at 0x10953a | `? != 0xffffffff` | `? == 0xffffffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x109545 | `eax` | `[[sp+4]+32]` |

## `_vgaDelLeftChar`

- `_vgaDelLeftChar`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x109548 to 0x109588.
  Established. The four-byte store through `[sp+4] +0x14` writes `[[sp+4]+20]-2` (`mov` at
  0x109579). The later two-byte store through `[[sp+4]+20] +0x0` retains an unknown value (`mov` at
  0x10957f). The exit reports `[[sp+4]+20]` in `eax` (`ret` at 0x109585). A deleted-character or
  display-cell interpretation is not assigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10954b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10954e | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x10955b | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x109562 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x109569 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10956d | store | 4 | `[sp+4] +0x10` | `?` |
| `mov` at 0x109570 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x109573 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x109579 | store | 4 | `[sp+4] +0x14` | `[[sp+4]+20]-2` |
| `mov` at 0x10957c | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x10957f | store | 2 | `[[sp+4]+20] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x109560 | `([[sp+4]+48] & [[sp+4]+48]) != 0x0` | `([[sp+4]+48] & [[sp+4]+48]) == 0x0` |
| `jle` at 0x109567 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x109585 | `eax` | `[[sp+4]+20]` |

## `_vgaCarriageReturn`

- `_vgaCarriageReturn`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x109588 to 0x1095a8.
  Established. The four-byte store through `[sp+4] +0x14` has an unknown value (`mov` at 0x109598).
  The following four-byte store through `[sp+4] +0x10` writes `0x0` (`mov` at 0x10959b). The exit
  remains unknown (`ret` at 0x1095a5). The two assignments do not independently identify a
  carriage-return operation.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10958b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10958e | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x109593 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x109598 | store | 4 | `[sp+4] +0x14` | `?` |
| `mov` at 0x10959b | store | 4 | `[sp+4] +0x10` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1095a5 | `eax` | `?` |

## `_vgaBackSpace`

- `_vgaBackSpace`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1095a8 to 0x1095f8.
  Established. The four-byte store through `[sp+4] +0x14` retains `[[sp+4]+20]-2` (`mov` at
  0x1095cc). Other four-byte stores through `[sp+4] +0x10` (`mov` at 0x1095c0, `mov` at 0x1095da)
  and `[sp+4] +0xc` (`mov` at 0x1095e1) have unknown values. A store through `[sp+4] +0x44` writes
  `0x1` (`mov` at 0x1095e7). Field and backspace roles remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1095ab | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1095ae | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x1095b5 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x1095bc | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x1095c0 | store | 4 | `[sp+4] +0x10` | `?` |
| `mov` at 0x1095c3 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x1095c6 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x1095cc | store | 4 | `[sp+4] +0x14` | `[[sp+4]+20]-2` |
| `mov` at 0x1095cf | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x1095d6 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x1095da | store | 4 | `[sp+4] +0x10` | `?` |
| `mov` at 0x1095dd | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x1095e1 | store | 4 | `[sp+4] +0xc` | `?` |
| `mov` at 0x1095e4 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x1095e7 | store | 4 | `[sp+4] +0x44` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x1095b3 | `([[sp+4]+48] & [[sp+4]+48]) != 0x0` | `([[sp+4]+48] & [[sp+4]+48]) == 0x0` |
| `jle` at 0x1095ba | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jge` at 0x1095d4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1095f1 | `eax` | `?` |

## `_vgaTab`

- `_vgaTab`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1095f8 to 0x109678.
  Established. The first branch is taken when `[[sp+4]+16]+1 > 0x4f (signed, 32-bit)` (`jg` at
  0x109607). Four-byte stores write `0x0` through `[sp+4] +0x10` (`mov` at 0x10963a) and `0x1`
  through `[sp+4] +0x44` (`mov` at 0x10964b). The later four-byte store through `[sp+4] +0x14`
  remains unknown (`mov` at 0x109666). No tab-stop spacing or display width is inferred.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1095fb | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1095fe | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10960c | load | 1 | `(unknown)` | `-` |
| `mov` at 0x109614 | store | 4 | `[sp+4] +0x10` | `?` |
| `mov` at 0x10962e | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x10963a | store | 4 | `[sp+4] +0x10` | `0x0` |
| `mov` at 0x109641 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x109645 | store | 4 | `[sp+4] +0xc` | `?` |
| `mov` at 0x109648 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10964b | store | 4 | `[sp+4] +0x44` | `0x1` |
| `mov` at 0x109652 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x109655 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x10965b | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x109660 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x109666 | store | 4 | `[sp+4] +0x14` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x109607 | `[[sp+4]+16]+1 > 0x4f (signed, 32-bit)` | `[[sp+4]+16]+1 <= 0x4f (signed, 32-bit)` |
| `je` at 0x109612 | `Z == 1` | `not (Z == 1)` |
| `jle` at 0x10962c | `? <= 0x4f (signed, 32-bit)` | `? > 0x4f (signed, 32-bit)` |
| `je` at 0x109633 | `([[sp+4]+48] & [[sp+4]+48]) == 0x0` | `([[sp+4]+48] & [[sp+4]+48]) != 0x0` |
| `jle` at 0x109638 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10966c | `eax` | `?` |

## `_vgaLineFeed`

- `_vgaLineFeed`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x109678 to 0x1096a8.
  Established. The four-byte stores through `[sp+4] +0x14` (`mov` at 0x109687) and `[sp+4] +0xc`
  (`mov` at 0x10968e) retain unknown values. Another four-byte store through `[sp+4] +0x44` writes
  `0x1` (`mov` at 0x109694). The exit reports `[[sp+4]+12]` (`ret` at 0x10969e). These effects do
  not establish a line-feed role.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10967b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10967e | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x109681 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x109687 | store | 4 | `[sp+4] +0x14` | `?` |
| `mov` at 0x10968a | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10968e | store | 4 | `[sp+4] +0xc` | `?` |
| `mov` at 0x109691 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x109694 | store | 4 | `[sp+4] +0x44` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10969e | `eax` | `[[sp+4]+12]` |
