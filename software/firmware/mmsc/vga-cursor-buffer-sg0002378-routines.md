# MMSC VGA cursor and buffer paths in patch SG0002378

Read from `patches/SG0002378/usr/cpu/firmware/mmscfw.bin` (patch SG0002378).
Image SHA-256: `b1249e58cf4f00c252362da8ed0515e25bb94aaf096bb60a5c5082c9b6e4d342`.
This x86 a.out image is linked at `0x108000`. The [load map](load-map.md) explains the layout.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred end and can include literals or padding
after returns. Span ends are exclusive. Statements below concern sites reached by known
control-flow edges.
No unresolved jump transfers were reported in these spans.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/vga-cursor-buffer-sg0002378-facts.txt) include all tool sections.

## `_vgaHrdInit`

- `_vgaHrdInit`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108ee8 to 0x108f58.
  Established. The `_sysOutByte` calls have leading candidates `0x3d4`, `0xa` (`call` at 0x108ef2)
  and `0x3d4`, `0xb` (`call` at 0x108f14). Both `_sysInByte` sites use candidate zero `0x3d5`
  (`call` at 0x108eff, `call` at 0x108f21). The byte stores to absolute `0x1d4ab8` (`mov` at
  0x108f07) and `0x1d4abc` (`mov` at 0x108f29) have unknown values.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x108f07 | store | 1 | `abs 0x1d4ab8` | `?` |
| `mov` at 0x108f29 | store | 1 | `abs 0x1d4abc` | `?` |
| `mov` at 0x108f38 | load | 4 | `abs 0x1abe90` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108ef2 | `_sysOutByte 0x1081d0` | `0x3d4` | `0xa` | `ebp` | `?` | `?` | `?` |
| `call` at 0x108eff | `_sysInByte 0x1081b0` | `0x3d5` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108f14 | `_sysOutByte 0x1081d0` | `0x3d4` | `0xb` | `?` | `?` | `?` | `?` |
| `call` at 0x108f21 | `_sysInByte 0x1081b0` | `0x3d5` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108f2f | `_vgaStatInit 0x108f58` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108f3e | `_vgaClear 0x109208` | `[0x1abe90]` | `0x2` | `0x20` | `?` | `?` | `?` |
| `call` at 0x108f46 | `_vgaCursorOn 0x109708` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108f4e | `eax` | `ret(_vgaCursorOn)` |

## `_vgaConBeep`

- `_vgaConBeep`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108e08 to 0x108ee8.
  Established. The `_sysOutByte` site presents candidates `0x43` and `0xffffffb6` (`call` at
  0x108e6b). Two later sites have candidate zero `0x42` and unknown candidate one (`call` at
  0x108e79, `call` at 0x108e89). The `_taskDelay` call retains unknown candidates (`call` at
  0x108eac). The initial branch is taken when `[sp+4] == 0x0` (`je` at 0x108e15), without
  establishing timing units.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x108e11 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x108e2f | store | 4 | `sp -0x8` | `?` |
| `sub` at 0x108e34 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108e1c | `_sysClkRateGet 0x1085a8` | `ebx` | `esi` | `edi` | `?` | `ebp` | `?` |
| `call` at 0x108e4d | `_sysClkRateGet 0x1085a8` | `ebx` | `esi` | `edi` | `?` | `ebp` | `?` |
| `call` at 0x108e60 | `_intLock 0x1712d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108e6b | `_sysOutByte 0x1081d0` | `0x43` | `0xffffffb6` | `?` | `?` | `?` | `?` |
| `call` at 0x108e79 | `_sysOutByte 0x1081d0` | `0x42` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108e89 | `_sysOutByte 0x1081d0` | `0x42` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108e93 | `_sysInByte 0x1081b0` | `0x61` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108ea3 | `_sysOutByte 0x1081d0` | `0x61` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108eac | `_taskDelay 0x1722d8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108eb6 | `_sysInByte 0x1081b0` | `0x61` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108ec6 | `_sysOutByte 0x1081d0` | `0x61` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108ecf | `_intUnlock 0x1712e0` | `ret(_intLock)` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x108e15 | `[sp+4] == 0x0` | `[sp+4] != 0x0` |
| `jge` at 0x108e56 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108edd | `eax` | `ret(_intUnlock)` |

## `_vgaStatInit`

- `_vgaStatInit`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108f58 to 0x109128.
  Established. A four-byte store writes `0xb8000` to an unknown target (`mov` at 0x108f86). Other
  indexed four-byte stores include values `0x50` (`mov` at 0x109002) and `0x19` (`mov` at 0x10900c),
  again at unknown effective addresses. The two `_bzero` calls present candidates `[sp-16]`, `0x40`
  (`call` at 0x109080) and `[sp-12]`, `0x50` (`call` at 0x10908e). The record fields remain
  unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x108f61 | store | 4 | `sp -0x8` | `0x0` |
| `mov` at 0x108f68 | store | 4 | `sp -0xc` | `0x1d4908` |
| `mov` at 0x108f6f | store | 4 | `sp -0x10` | `0x1d4960` |
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
| `mov` at 0x10903e | load | 4 | `abs 0x1ac070` | `-` |
| `mov` at 0x109043 | store | 4 | `(unknown)` | `[0x1ac070]` |
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
| `call` at 0x109080 | `_bzero 0x172678` | `[sp-16]` | `0x40` | `?` | `?` | `?` | `?` |
| `call` at 0x10908e | `_bzero 0x172678` | `[sp-12]` | `0x50` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x10910a | `[sp-8] <= 0x1 (signed, 32-bit)` | `[sp-8] > 0x1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x109119 | `eax` | `ret(_bzero)` |

## `_vgaClear`

- `_vgaClear`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x109208 to 0x1092a8.
  Established. The first branch falls through when `[sp+8] == 0x2` (`jne` at 0x109229). That path
  clears `[sp+4] +0xc` with a four-byte store (`mov` at 0x109235), then stores `[[sp+4]+12]` at
  `[sp+4] +0x10` (`mov` at 0x10923f). The two-byte store in the shared loop has unknown effective
  address and value (`mov` at 0x109288).

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

- `_vgaCursorOn`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x109708 to 0x109758.
  Established. The `_sysOutByte` sites include leading candidates `0x3d4`, `0xa` (`call` at
  0x109712) and `0x3d4`, `0xb` (`call` at 0x10973a). The two other sites have candidate zero `0x3d5`
  and unknown candidate one (`call` at 0x10972b, `call` at 0x10974f). Byte loads read absolute
  `0x1d4ab8` (`mov` at 0x10971a) and `0x1d4abc` (`movsx` at 0x109742).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10971a | load | 1 | `abs 0x1d4ab8` | `-` |
| `movsx` at 0x109742 | load | 1 | `abs 0x1d4abc` | `-` |

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

- `_vgaCursorOff`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x109758 to 0x109798.
  Established. The `_sysOutByte` call sites present candidates `0x3d4`, `0xa` (`call` at 0x109762),
  `0x3d5`, `0x20` (`call` at 0x109771), `0x3d4`, `0xb` (`call` at 0x109780), and `0x3d5`, `0x0`
  (`call` at 0x10978f). No conditional branch is reported in the analysed span. The exit retains
  `ret(_sysOutByte)` (`ret` at 0x109797).

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

- `_vgaCursorPos`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1096a8 to 0x109708.
  Established. The entry load reads two bytes at `sp +0x4` (`mov` at 0x1096ac). The `_sysOutByte`
  calls include candidates `0x3d4`, `0xe` (`call` at 0x1096b7) and `0x3d4`, `0xf` (`call` at
  0x1096dd). Their intervening and final sites have candidate zero `0x3d5` with unknown candidate
  one (`call` at 0x1096ce, `call` at 0x1096ee).

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

- `_vgaScreenRev`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1092a8 to 0x1092f8.
  Established. The three byte stores retain unknown effective addresses and values (`mov` at
  0x1092bf, `mov` at 0x1092ce, `mov` at 0x1092dc). The loop branch is taken when `C == 1` (`jb` at
  0x1092eb). The four-byte load through `[sp+4] +0x0` is resolved (`mov` at 0x1092e2), but the
  byte-store destinations are not resolved from it.

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

- `_vgaScroll`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1092f8 to 0x1094c8.
  Established. After both earlier range branches fall through, the path-selection branch is
  taken when `[sp+16] == 0x0` (`je` at 0x109326). The nonzero fall-through path includes a byte store of `0x20` (`mov` at 0x1093cc), while the zero
  taken path includes a separate byte store of `0x20` (`mov` at 0x10949d). Both effective addresses
  are unknown. The first byte-transfer store writes `[[sp-12]]:1` to an unknown target (`mov` at
  0x109389).

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

- `_vgaInsertChar`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1094c8 to 0x109548.
  Established. The range branch is taken when `[sp+8] <= [[sp+4]+32] (signed, 32-bit)` (`jle` at
  0x1094dc). A two-byte local store at `sp -0x8` has unknown value (`mov` at 0x10950c). The loop
  two-byte store writes `[sp-8]:2` to an unknown target (`mov` at 0x10951f). The field and index
  meanings remain unknown.

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

- `_vgaDelLeftChar`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x109548 to 0x109588.
  Established. The final two-byte store through `[[sp+4]+20] +0x0` has unknown value (`mov` at
  0x10957f). The conditional four-byte store to `[sp+4] +0x14` has value `[[sp+4]+20]-2` (`mov` at
  0x109579). The earlier branch is taken when `([[sp+4]+48] & [[sp+4]+48]) != 0x0` (`jne` at
  0x109560), reaching that update path. The same update also follows the fall-through predicate
  `not (le flags condition (operands unknown))` (`jle` at 0x109567); its taken predicate
  `le flags condition (operands unknown)` skips the update and reaches the shared final store
  (`mov` at 0x10957f).

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

- `_vgaCarriageReturn`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x109588 to 0x1095a8.
  Established. The four-byte store to `[sp+4] +0x14` has unknown value (`mov` at 0x109598). It is
  followed by a four-byte zero store to `[sp+4] +0x10` (`mov` at 0x10959b). The four-byte loads
  supplying the calculation read `[sp+4] +0x10` (`mov` at 0x10958e) and `[sp+4] +0x14` (`mov` at
  0x109593).

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

- `_vgaBackSpace`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1095a8 to 0x1095f8.
  Established. The last group of stores is reached on the fall-through predicate `not (ge flags
  condition (operands unknown))` (`jge` at 0x1095d4); its final four-byte store writes `0x1` to
  `[sp+4] +0x44` (`mov` at 0x1095e7). A separate earlier four-byte store to `[sp+4] +0x14` has value
  `[[sp+4]+20]-2` (`mov` at 0x1095cc). That earlier store is reached either on the taken predicate
  `([[sp+4]+48] & [[sp+4]+48]) != 0x0` (`jne` at 0x1095b3), or after that branch falls through
  and the next branch also falls through on `not (le flags condition (operands unknown))`
  (`jle` at 0x1095ba). The update and skip paths converge before the final conditional branch
  (`jge` at 0x1095d4).

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

- `_vgaTab`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1095f8 to 0x109678.
  Established. The first branch is taken when `[[sp+4]+16]+1 > 0x4f (signed, 32-bit)` (`jg` at
  0x109607). The byte load in the loop has an unknown effective address (`mov` at 0x10960c). The
  conditional four-byte store to `[sp+4] +0x10` has unknown value (`mov` at 0x109614). No tab-stop
  record format is established by these resolved accesses.

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

- `_vgaLineFeed`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x109678 to 0x1096a8.
  Established. Two four-byte stores write unknown values to `[sp+4] +0x14` (`mov` at 0x109687) and
  `[sp+4] +0xc` (`mov` at 0x10968e). The next four-byte store writes `0x1` to `[sp+4] +0x44` (`mov`
  at 0x109694). The return expression is `[[sp+4]+12]` (`ret` at 0x10969e).

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
