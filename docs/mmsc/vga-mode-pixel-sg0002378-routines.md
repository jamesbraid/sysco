# MMSC VGA mode and pixel paths in patch SG0002378

Read from `patches/SG0002378/usr/cpu/firmware/mmscfw.bin` (patch SG0002378).
Image SHA-256: `b1249e58cf4f00c252362da8ed0515e25bb94aaf096bb60a5c5082c9b6e4d342`.
This image has the x86 a.out link address `0x108000`. The [load map](load-map.md) explains segment mapping.
The analysed spans end at the next text symbol, with exclusive inferred ends.
They can include literals or padding after returns. Statements below concern sites
reached by known control-flow edges.
Four selector transfers remain unresolved, and selector-only effects are excluded.
Aligned supplementary listings cover selector targets and the earlier shared exits.
The supplementary entry states are independent and do not resolve the selector dataflow.
Unsupported inc, dec and rotate effects leave corresponding values or flags unresolved.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records.
The full facts also include implicit push/pop accesses. Call and return instructions
remain in the listings without separate implicit stack access rows.
Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/vga-mode-pixel-sg0002378-facts.txt) include all tool sections.

## `_vga_setmode`

- `_vga_setmode`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16e848 to 0x16ee18.
  Established. Four-byte stores copy `[0x1d2ac0]` to absolute `0x1d2abc` (`mov` at 0x16e86f) and
  `[sp+4]` to absolute `0x1d2ac0` (`mov` at 0x16e874). The `_sysOutByte` sites include leading pairs
  `0x3c0`, `0x0` (`call` at 0x16e88f) and `0x3c0`, `0x20` (`call` at 0x16e9e6, `call` at 0x16edf0).
  The range branch is taken when `[sp+4]-1 > 0xb (unsigned, 32-bit)` (`ja` at 0x16ea5c); its other
  arm reaches an unresolved computed transfer (`jmp` at 0x16ea62), so selector-only effects remain
  excluded.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16e851 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x16e854 | load | 4 | `abs 0x1d2ac4` | `-` |
| `mov` at 0x16e867 | store | 4 | `sp -0x8` | `ret(_intLock)` |
| `mov` at 0x16e86a | load | 4 | `abs 0x1d2ac0` | `-` |
| `mov` at 0x16e86f | store | 4 | `abs 0x1d2abc` | `[0x1d2ac0]` |
| `mov` at 0x16e874 | store | 4 | `abs 0x1d2ac0` | `[sp+4]` |
| `mov` at 0x16e87a | load | 4 | `abs 0x1d5a70` | `-` |
| `cmp` at 0x16e897 | load | 4 | `abs 0x1d2acc` | `-` |
| `cmp` at 0x16e8a0 | load | 4 | `abs 0x1d2abc` | `-` |
| `mov` at 0x16e90b | load | 4 | `abs 0x1d5df4` | `-` |
| `rep movsd` at 0x16e91c | store | 4 | `(unknown)` | `?` |
| `rep movsd` at 0x16e91c | load | 4 | `abs 0x1d5df8` | `-` |
| `mov` at 0x16e93c | load | 4 | `abs 0x1d5df4` | `-` |
| `rep movsd` at 0x16e94d | store | 4 | `(unknown)` | `?` |
| `rep movsd` at 0x16e94d | load | 4 | `abs 0x1d7df8` | `-` |
| `cmp` at 0x16e94f | load | 4 | `abs 0x1d5a74` | `-` |
| `mov` at 0x16e958 | store | 4 | `abs 0x1d5a68` | `0x3b4` |
| `mov` at 0x16e962 | store | 4 | `abs 0x1d5a6c` | `0x3b5` |
| `mov` at 0x16e96c | store | 4 | `abs 0x1d5a70` | `0x3ba` |
| `movzx` at 0x16e998 | load | 1 | `(unknown)` | `-` |
| `movzx` at 0x16e9a0 | load | 1 | `(unknown)` | `-` |
| `movzx` at 0x16e9a8 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x16e9d1 | load | 4 | `abs 0x1d5a70` | `-` |
| `mov` at 0x16e9f8 | store | 1 | `(unknown)` | `0x20` |
| `mov` at 0x16e9fc | store | 1 | `(unknown)` | `0x1f` |
| `mov` at 0x16ea18 | store | 4 | `abs 0x1d5a68` | `0x3d4` |
| `mov` at 0x16ea22 | store | 4 | `abs 0x1d5a6c` | `0x3d5` |
| `mov` at 0x16ea2c | store | 4 | `abs 0x1d5a70` | `0x3da` |
| `jmp` at 0x16ea62 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x16ed9f | load | 4 | `abs 0x1d2ac8` | `-` |
| `movzx` at 0x16edac | load | 1 | `(unknown)` | `-` |
| `movzx` at 0x16edb4 | load | 1 | `(unknown)` | `-` |
| `movzx` at 0x16edbc | load | 1 | `(unknown)` | `-` |
| `mov` at 0x16eddb | load | 4 | `abs 0x1d5a70` | `-` |
| `mov` at 0x16edf8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16e85d | `_initialize 0x16e668` | `ebx` | `esi` | `edi` | `?` | `ebp` | `?` |
| `call` at 0x16e862 | `_intLock 0x1712d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16e880 | `_sysInByte 0x1081b0` | `[0x1d5a70]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16e88f | `_sysOutByte 0x1081d0` | `0x3c0` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x16e8b0 | `_set_regs 0x16e4c8` | `0x1d26b4` | `0xa` | `?` | `?` | `?` | `?` |
| `call` at 0x16e8c7 | `_set_regs 0x16e4c8` | `0x1d24ec` | `0x4` | `?` | `?` | `?` | `?` |
| `call` at 0x16e8d6 | `_sysOutByte 0x1081d0` | `0x3ce` | `0x1` | `?` | `?` | `?` | `?` |
| `call` at 0x16e8e5 | `_sysOutByte 0x1081d0` | `0x3cf` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x16e8f4 | `_sysOutByte 0x1081d0` | `0x3c4` | `0x2` | `?` | `?` | `?` | `?` |
| `call` at 0x16e903 | `_sysOutByte 0x1081d0` | `0x3c5` | `0x4` | `?` | `?` | `?` | `?` |
| `call` at 0x16e925 | `_sysOutByte 0x1081d0` | `0x3c4` | `0x2` | `?` | `?` | `?` | `?` |
| `call` at 0x16e934 | `_sysOutByte 0x1081d0` | `0x3c5` | `0x8` | `?` | `?` | `?` | `?` |
| `call` at 0x16e97b | `_sysInByte 0x1081b0` | `0x3cc` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16e98e | `_sysOutByte 0x1081d0` | `0x3c2` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16e9b1 | `_vga_setpalette 0x16efd8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16e9c9 | `_set_regs 0x16e4c8` | `0x1d5a78` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x16e9d7 | `_sysInByte 0x1081b0` | `[0x1d5a70]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16e9e6 | `_sysOutByte 0x1081d0` | `0x3c0` | `0x20` | `?` | `?` | `?` | `?` |
| `call` at 0x16ea3b | `_sysInByte 0x1081b0` | `0x3cc` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16ea4e | `_sysOutByte 0x1081d0` | `0x3c2` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16edc5 | `_vga_setpalette 0x16efd8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16edd6 | `_vga_clear 0x16ee68` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16ede1 | `_sysInByte 0x1081b0` | `[0x1d5a70]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16edf0 | `_sysOutByte 0x1081d0` | `0x3c0` | `0x20` | `?` | `?` | `?` | `?` |
| `call` at 0x16edfc | `_intUnlock 0x1712e0` | `[sp-8]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x16e85b | `[0x1d2ac4] != 0x0` | `[0x1d2ac4] == 0x0` |
| `je` at 0x16e89e | `[0x1d2acc] == 0x0` | `[0x1d2acc] != 0x0` |
| `jne` at 0x16e8a7 | `[0x1d2abc] != 0xc` | `[0x1d2abc] == 0xc` |
| `jne` at 0x16e8ba | `([sp+4] & [sp+4]) != 0x0` | `([sp+4] & [sp+4]) == 0x0` |
| `jne` at 0x16e956 | `[0x1d5a74] != 0x0` | `[0x1d5a74] == 0x0` |
| `jle` at 0x16e9c0 | `? <= 0xff (signed, 32-bit)` | `? > 0xff (signed, 32-bit)` |
| `jne` at 0x16ea03 | `Z == 0` | `not (Z == 0)` |
| `ja` at 0x16ea5c | `[sp+4]-1 > 0xb (unsigned, 32-bit)` | `[sp+4]-1 <= 0xb (unsigned, 32-bit)` |
| `jne` at 0x16eda6 | `[0x1d2ac8] != 0x0` | `[0x1d2ac8] == 0x0` |
| `jle` at 0x16edd4 | `? <= 0xff (signed, 32-bit)` | `? > 0xff (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16ee0c | `eax` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x16ea62).

## `_vga_clear`

- `_vga_clear`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16ee68 to 0x16efd8.
  Established. The `_vga_screenoff` call presents saved-register candidates (`call` at 0x16ee6e).
  The range branch is taken when `? > 0xb (unsigned, 32-bit)` (`ja` at 0x16ee7c), reaching
  `_vga_setcolor` with candidate zero `0xf` (`call` at 0x16efb8) and then `_vga_screenon` with
  unknown candidates (`call` at 0x16efc0). The computed-transfer operand is a four-byte load at an
  unknown effective address (`jmp` at 0x16ee82). No clearing extent or selector-arm effect is
  established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16ee73 | load | 4 | `abs 0x1d2ac0` | `-` |
| `jmp` at 0x16ee82 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16ee6e | `_vga_screenoff 0x16f3e8` | `ebx` | `esi` | `edi` | `ebp` | `?` | `?` |
| `call` at 0x16efb8 | `_vga_setcolor 0x16f1f8` | `0xf` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16efc0 | `_vga_screenon 0x16f428` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x16ee7c | `? > 0xb (unsigned, 32-bit)` | `? <= 0xb (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16efd0 | `eax` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x16ee82).

## `_vga_hasmode`

- `_vga_hasmode`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16ee18 to 0x16ee68.
  Established. The four-byte load reads `sp +0x4` (`mov` at 0x16ee1b). The range branch is taken
  when `[sp+4] > 0x9 (signed, 32-bit)` (`jg` at 0x16ee25), reaching a second comparison whose taken
  condition is `[sp+4] > 0xc (signed, 32-bit)` (`jg` at 0x16ee3b); its fall-through reaches
  `_get_perm` (`call` at 0x16ee3d) and `_et4000_test` (`call` at 0x16ee42). The corresponding exit
  retains `ret(_et4000_test)` (`ret` at 0x16ee4a).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16ee1b | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16ee3d | `_get_perm 0x16e118` | `ebp` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16ee42 | `_et4000_test 0x16e188` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jl` at 0x16ee20 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jg` at 0x16ee25 | `[sp+4] > 0x9 (signed, 32-bit)` | `[sp+4] <= 0x9 (signed, 32-bit)` |
| `jg` at 0x16ee3b | `[sp+4] > 0xc (signed, 32-bit)` | `[sp+4] <= 0xc (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16ee2f | `eax` | `0x1` |
| `ret` at 0x16ee4a | `eax` | `ret(_et4000_test)` |
| `ret` at 0x16ee5d | `eax` | `0x0` |

## `_vga_setcolor`

- `_vga_setcolor`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16f1f8 to 0x16f288.
  Established. The entry loads use `sp +0x4` (`mov` at 0x16f1fc) and absolute `0x1d2ac0` (`mov` at
  0x16f1ff). The range branch is taken when `? > 0xb (unsigned, 32-bit)` (`ja` at 0x16f208); its
  tested value remains unknown. The computed-transfer operand is a four-byte load at an unknown
  effective address (`jmp` at 0x16f20a). Selector-only output calls and state stores receive no
  effect credit.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16f1fc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x16f1ff | load | 4 | `abs 0x1d2ac0` | `-` |
| `jmp` at 0x16f20a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x16f280 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x16f208 | `? > 0xb (unsigned, 32-bit)` | `? <= 0xb (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16f286 | `eax` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x16f20a).

## `_vga_drawpixel`

- `_vga_drawpixel`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16f288 to 0x16f3e8.
  Established. Two four-byte loads read `sp +0x4` (`mov` at 0x16f28d) and `sp +0x8` (`mov` at
  0x16f290). The branch is taken when `? > 0xb (unsigned, 32-bit)` (`ja` at 0x16f29c), reaching the
  zero-valued exit (`ret` at 0x16f3e3). The alternative computed transfer reads a four-byte operand
  at an unknown effective address (`jmp` at 0x16f2a2). Pixel addresses and selector-arm stores
  remain unestablished.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16f28d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x16f290 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x16f293 | load | 4 | `abs 0x1d2ac0` | `-` |
| `jmp` at 0x16f2a2 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x16f29c | `? > 0xb (unsigned, 32-bit)` | `? <= 0xb (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16f3e3 | `eax` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x16f2a2).

## `_vga_getxdim`

- `_vga_getxdim`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16f468 to 0x16f478.
  Established. A four-byte load reads absolute `0x1d5de0` (`mov` at 0x16f46b). The exit reports
  `[0x1d5de0]` in `eax` (`ret` at 0x16f473). The returned global value is established, but its
  dimension role and units remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16f46b | load | 4 | `abs 0x1d5de0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16f473 | `eax` | `[0x1d5de0]` |

## `_vga_getydim`

- `_vga_getydim`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16f478 to 0x16f488.
  Established. The entry reads absolute `0x1d5de4` with a four-byte load (`mov` at 0x16f47b), and
  the exit carries `[0x1d5de4]` in `eax` (`ret` at 0x16f483). Neither the global field meaning nor
  its units follows from this accessor alone.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16f47b | load | 4 | `abs 0x1d5de4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16f483 | `eax` | `[0x1d5de4]` |

## `_vga_getcolors`

- `_vga_getcolors`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16f488 to 0x16f498.
  Established. The loaded expression `[0x1d5de8]` comes from a four-byte access at that absolute
  address (`mov` at 0x16f48b). The exit preserves it in `eax` (`ret` at 0x16f493). A colour count or
  format is not established by the routine name.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16f48b | load | 4 | `abs 0x1d5de8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16f493 | `eax` | `[0x1d5de8]` |

## `_mwSPX`

- `_mwSPX`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1664e8 to 0x1667d0.
  Established. The two-byte I/O outputs at absolute `0x3ce` include unknown values (`out` at
  0x166646, `out` at 0x166657), value `0xf01` (`out` at 0x16664c), value `0x1` (`out` at 0x166670),
  and value `0xff08` (`out` at 0x166676). Another two-byte output has unknown I/O address and value
  (`out` at 0x166660). The indirect call through `[[sp+4]+28] +0xc0` presents candidate zero
  `0x166668` with unknown remaining candidates (`call` at 0x166621). The later `ret` site (`ret` at
  0x166664) must not be treated as proof of a top-level return or of callback execution.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1664f2 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1664f5 | load | 1 | `[sp+4] +0x8` | `-` |
| `mov` at 0x1664fd | store | 1 | `sp -0x26` | `?` |
| `mov` at 0x166502 | store | 1 | `sp -0x27` | `?` |
| `mov` at 0x166507 | load | 4 | `[sp+4] +0x24` | `-` |
| `mov` at 0x16650a | load | 4 | `[[sp+4]+36] +0x0` | `-` |
| `mov` at 0x16650c | store | 4 | `sp -0x38` | `[[[sp+4]+36]]` |
| `mov` at 0x16650f | load | 4 | `[[sp+4]+36] +0x4` | `-` |
| `mov` at 0x166512 | store | 4 | `sp -0x34` | `[[[sp+4]+36]+4]` |
| `mov` at 0x166515 | load | 1 | `[sp+4] +0x8` | `-` |
| `mov` at 0x16651a | store | 1 | `sp -0x28` | `?` |
| `mov` at 0x166523 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x166526 | load | 4 | `[[sp+4]+32] +0x14` | `-` |
| `cmp` at 0x166529 | load | 2 | `[[[sp+4]+32]+20] +0x2` | `-` |
| `mov` at 0x166535 | store | 4 | `sp -0x24` | `[[[sp+4]+32]+20]` |
| `cmp` at 0x166538 | load | 1 | `sp -0x27` | `-` |
| `mov` at 0x166542 | load | 2 | `sp -0x38` | `-` |
| `cmp` at 0x166546 | load | 2 | `[[sp+4]+32] +0xc` | `-` |
| `mov` at 0x16654c | load | 2 | `[[sp+4]+32] +0xc` | `-` |
| `mov` at 0x166550 | store | 2 | `sp -0x38` | `[[[sp+4]+32]+12]:2` |
| `mov` at 0x166554 | load | 2 | `[[sp+4]+32] +0xe` | `-` |
| `cmp` at 0x166558 | load | 2 | `sp -0x36` | `-` |
| `mov` at 0x16655e | load | 2 | `sp -0x36` | `-` |
| `cmp` at 0x16656f | load | 2 | `(unknown)` | `-` |
| `mov` at 0x166575 | store | 4 | `sp -0x24` | `?` |
| `mov` at 0x166578 | store | 2 | `sp -0x36` | `?` |
| `mov` at 0x16657c | load | 2 | `sp -0x34` | `-` |
| `cmp` at 0x166580 | load | 2 | `[[sp+4]+32] +0x10` | `-` |
| `mov` at 0x166586 | load | 2 | `[[sp+4]+32] +0x10` | `-` |
| `mov` at 0x16658a | store | 2 | `sp -0x34` | `[[[sp+4]+32]+16]:2` |
| `mov` at 0x16658e | load | 2 | `[[sp+4]+32] +0x12` | `-` |
| `cmp` at 0x166592 | load | 2 | `sp -0x32` | `-` |
| `mov` at 0x166598 | load | 2 | `sp -0x32` | `-` |
| `mov` at 0x16659c | store | 2 | `sp -0x32` | `?` |
| `mov` at 0x1665bc | load | 4 | `[[sp+4]+32] +0xc` | `-` |
| `mov` at 0x1665bf | store | 4 | `sp -0x38` | `[[[sp+4]+32]+12]` |
| `mov` at 0x1665c2 | load | 4 | `[[sp+4]+32] +0x10` | `-` |
| `mov` at 0x1665c5 | store | 4 | `sp -0x34` | `[[[sp+4]+32]+16]` |
| `mov` at 0x1665c8 | store | 1 | `sp -0x27` | `0x1` |
| `mov` at 0x1665cc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1665cf | load | 4 | `[sp+4] +0x30` | `-` |
| `movsx` at 0x1665d2 | load | 2 | `[[sp+4]+48] +0x0` | `-` |
| `movsx` at 0x1665d5 | load | 2 | `[[sp+4]+48] +0x2` | `-` |
| `mov` at 0x1665d9 | load | 4 | `[sp+4] +0x1c` | `-` |
| `dec` at 0x1665dc | load | 1 | `[[sp+4]+28] +0x98` | `-` |
| `dec` at 0x1665dc | store | 1 | `[[sp+4]+28] +0x98` | `?` |
| `cmp` at 0x1665e2 | load | 1 | `sp -0x27` | `-` |
| `cmp` at 0x1665e8 | load | 2 | `sp -0x38` | `-` |
| `cmp` at 0x1665ee | load | 2 | `sp -0x36` | `-` |
| `cmp` at 0x1665f4 | load | 2 | `sp -0x34` | `-` |
| `cmp` at 0x1665fa | load | 2 | `sp -0x32` | `-` |
| `cmp` at 0x166600 | load | 1 | `sp -0x28` | `-` |
| `cmp` at 0x16660f | load | 4 | `[[sp+4]+28] +0xa4` | `-` |
| `cmp` at 0x166617 | load | 4 | `[[sp+4]+28] +0xac` | `-` |
| `call` at 0x166621 | load | 4 | `[[sp+4]+28] +0xc0` | `-` |
| `mov` at 0x166627 | load | 4 | `[[sp+4]+28] +0x18` | `-` |
| `mov` at 0x16662a | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x166637 | load | 1 | `[sp+4] +0xc` | `-` |
| `mov` at 0x16663d | load | 1 | `[sp+4] +0x2c` | `-` |
| `out` at 0x166646 | store | 2 | `I/O abs 0x3ce` | `?` |
| `out` at 0x16664c | store | 2 | `I/O abs 0x3ce` | `0xf01` |
| `mov` at 0x166650 | load | 1 | `(unknown)` | `-` |
| `out` at 0x166657 | store | 2 | `I/O abs 0x3ce` | `?` |
| `mov` at 0x16665d | load | 1 | `[sp+4] +0x10` | `-` |
| `out` at 0x166660 | store | 2 | `I/O (unknown)` | `?` |
| `xchg` at 0x166662 | load | 1 | `(unknown)` | `-` |
| `xchg` at 0x166662 | store | 1 | `(unknown)` | `?` |
| `out` at 0x166670 | store | 2 | `I/O abs 0x3ce` | `0x1` |
| `out` at 0x166676 | store | 2 | `I/O abs 0x3ce` | `0xff08` |
| `mov` at 0x166678 | load | 4 | `[sp+4] +0x1c` | `-` |
| `inc` at 0x16667b | load | 1 | `[[sp+4]+28] +0x98` | `-` |
| `inc` at 0x16667b | store | 1 | `[[sp+4]+28] +0x98` | `?` |
| `test` at 0x166685 | load | 2 | `[[sp+4]+28] +0x16` | `-` |
| `call` at 0x16668d | load | 4 | `[[sp+4]+28] +0xf8` | `-` |
| `test` at 0x16669c | load | 1 | `[sp+4] +0xa` | `-` |
| `mov` at 0x1666a2 | load | 1 | `[sp+4] +0x28` | `-` |
| `mov` at 0x1666ae | load | 4 | `sp -0x24` | `-` |
| `cmp` at 0x1666b7 | load | 2 | `(unknown)` | `-` |
| `cmp` at 0x1666bd | load | 1 | `sp -0x26` | `-` |
| `mov` at 0x1666c3 | store | 4 | `sp -0x24` | `?` |
| `cmp` at 0x1666c6 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1666d0 | store | 2 | `sp -0x20` | `?` |
| `mov` at 0x1666d4 | store | 2 | `sp -0x1e` | `?` |
| `mov` at 0x1666d8 | store | 2 | `sp -0x1c` | `?` |
| `mov` at 0x1666dc | store | 2 | `sp -0x1a` | `?` |
| `movsx` at 0x1666e0 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1666e4 | store | 4 | `sp -0x14` | `?` |
| `mov` at 0x1666e7 | load | 2 | `sp -0x1e` | `-` |
| `cmp` at 0x1666eb | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1666f1 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1666f5 | store | 2 | `sp -0x18` | `?` |
| `mov` at 0x1666f9 | load | 2 | `sp -0x1a` | `-` |
| `cmp` at 0x1666fd | load | 2 | `(unknown)` | `-` |
| `mov` at 0x166703 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x166707 | store | 2 | `sp -0x16` | `?` |
| `mov` at 0x16670b | load | 2 | `sp -0x20` | `-` |
| `cmp` at 0x16670f | load | 2 | `(unknown)` | `-` |
| `cmp` at 0x166718 | load | 2 | `(unknown)` | `-` |
| `cmp` at 0x166722 | load | 2 | `(unknown)` | `-` |
| `movsx` at 0x166728 | load | 2 | `sp -0x1c` | `-` |
| `cmp` at 0x16672c | load | 2 | `(unknown)` | `-` |
| `movsx` at 0x166735 | load | 2 | `sp -0x18` | `-` |
| `mov` at 0x166739 | store | 2 | `sp -0x2e` | `?` |
| `movsx` at 0x16673d | load | 2 | `sp -0x16` | `-` |
| `mov` at 0x166741 | store | 2 | `sp -0x2a` | `?` |
| `cmp` at 0x166745 | load | 2 | `(unknown)` | `-` |
| `movsx` at 0x16674b | load | 2 | `(unknown)` | `-` |
| `mov` at 0x16674f | store | 2 | `sp -0x2c` | `?` |
| `movsx` at 0x166753 | load | 2 | `sp -0x20` | `-` |
| `cmp` at 0x166757 | load | 2 | `(unknown)` | `-` |
| `movsx` at 0x16675c | load | 2 | `(unknown)` | `-` |
| `mov` at 0x16675f | store | 2 | `sp -0x30` | `?` |
| `mov` at 0x16676d | load | 4 | `sp -0x14` | `-` |
| `cmp` at 0x166770 | load | 2 | `[sp-60] +0xa` | `-` |
| `movsx` at 0x166776 | load | 2 | `sp -0x1c` | `-` |
| `cmp` at 0x16677a | load | 2 | `[sp-60] +0x8` | `-` |
| `cmp` at 0x166782 | load | 2 | `(unknown)` | `-` |
| `movsx` at 0x166788 | load | 2 | `(unknown)` | `-` |
| `cmp` at 0x16678c | load | 2 | `sp -0x1a` | `-` |
| `cmp` at 0x16679c | load | 2 | `(unknown)` | `-` |
| `cmp` at 0x1667a8 | load | 1 | `sp -0x26` | `-` |
| `mov` at 0x1667c0 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x166621 | `(unknown callee) (unknown target)` | `0x166668` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16668d | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x166764 | `(unknown callee) 0x1667b8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x166505 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x16651d | `Z == 1` | `not (Z == 1)` |
| `je` at 0x16652f | `([[[[sp+4]+32]+20]+2]:2 & 0xffff) == 0x7fff` | `([[[[sp+4]+32]+20]+2]:2 & 0xffff) != 0x7fff` |
| `je` at 0x16653c | `([sp-39]:1 & 0xff) == 0x0` | `([sp-39]:1 & 0xff) != 0x0` |
| `jge` at 0x16654a | `([sp-56]:2 & 0xffff) >= ([[[sp+4]+32]+12]:2 & 0xffff) (signed, 16-bit)` | `([sp-56]:2 & 0xffff) < ([[[sp+4]+32]+12]:2 & 0xffff) (signed, 16-bit)` |
| `jge` at 0x16655c | `([[[sp+4]+32]+14]:2 & 0xffff) >= ([sp-54]:2 & 0xffff) (signed, 16-bit)` | `([[[sp+4]+32]+14]:2 & 0xffff) < ([sp-54]:2 & 0xffff) (signed, 16-bit)` |
| `je` at 0x166567 | `([sp-54]:2 & 0xffff) == 0x7fff` | `([sp-54]:2 & 0xffff) != 0x7fff` |
| `jle` at 0x166573 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jle` at 0x166584 | `([sp-52]:2 & 0xffff) <= ([[[sp+4]+32]+16]:2 & 0xffff) (signed, 16-bit)` | `([sp-52]:2 & 0xffff) > ([[[sp+4]+32]+16]:2 & 0xffff) (signed, 16-bit)` |
| `jle` at 0x166596 | `([[[sp+4]+32]+18]:2 & 0xffff) <= ([sp-50]:2 & 0xffff) (signed, 16-bit)` | `([[[sp+4]+32]+18]:2 & 0xffff) > ([sp-50]:2 & 0xffff) (signed, 16-bit)` |
| `jge` at 0x1665a3 | `? >= ? (signed, 16-bit)` | `? < ? (signed, 16-bit)` |
| `jge` at 0x1665ac | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x1665e6 | `([sp-39]:1 & 0xff) == 0x0` | `([sp-39]:1 & 0xff) != 0x0` |
| `jl` at 0x1665ec | `? < ([sp-56]:2 & 0xffff) (signed, 16-bit)` | `? >= ([sp-56]:2 & 0xffff) (signed, 16-bit)` |
| `jl` at 0x1665f2 | `? < ([sp-54]:2 & 0xffff) (signed, 16-bit)` | `? >= ([sp-54]:2 & 0xffff) (signed, 16-bit)` |
| `jge` at 0x1665f8 | `? >= ([sp-52]:2 & 0xffff) (signed, 16-bit)` | `? < ([sp-52]:2 & 0xffff) (signed, 16-bit)` |
| `jge` at 0x1665fe | `? >= ([sp-50]:2 & 0xffff) (signed, 16-bit)` | `? < ([sp-50]:2 & 0xffff) (signed, 16-bit)` |
| `jne` at 0x166604 | `([sp-40]:1 & 0xff) != 0x0` | `([sp-40]:1 & 0xff) == 0x0` |
| `jb` at 0x166615 | `? < [[[sp+4]+28]+164] (unsigned, 32-bit)` | `? >= [[[sp+4]+28]+164] (unsigned, 32-bit)` |
| `jbe` at 0x16661d | `? <= [[[sp+4]+28]+172] (unsigned, 32-bit)` | `? > [[[sp+4]+28]+172] (unsigned, 32-bit)` |
| `je` at 0x16663b | `([[sp+4]+12]:1 & 0xff) == 0x0` | `([[sp+4]+12]:1 & 0xff) != 0x0` |
| `js` at 0x166681 | `N == 1` | `not (N == 1)` |
| `jg` at 0x166683 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `je` at 0x16668b | `(([[[sp+4]+28]+22]:2 & 0xffff) & 0x8) == 0x0` | `(([[[sp+4]+28]+22]:2 & 0xffff) & 0x8) != 0x0` |
| `jne` at 0x1666a0 | `(([[sp+4]+10]:1 & 0xff) & 0x10) != 0x0` | `(([[sp+4]+10]:1 & 0xff) & 0x10) == 0x0` |
| `jge` at 0x1666bb | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x1666c1 | `([sp-38]:1 & 0xff) == 0x0` | `([sp-38]:1 & 0xff) != 0x0` |
| `jle` at 0x1666ca | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jge` at 0x1666ef | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jle` at 0x166701 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jl` at 0x166713 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jne` at 0x16671c | `Z == 0` | `not (Z == 0)` |
| `jge` at 0x166726 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jle` at 0x16672f | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jle` at 0x166749 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jge` at 0x16675a | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x166774 | `([[sp-60]+10]:2 & 0xffff) != ?` | `([[sp-60]+10]:2 & 0xffff) == ?` |
| `jg` at 0x16677d | `? > ([[sp-60]+8]:2 & 0xffff) (signed, 16-bit)` | `? <= ([[sp-60]+8]:2 & 0xffff) (signed, 16-bit)` |
| `je` at 0x166786 | `Z == 1` | `not (Z == 1)` |
| `jg` at 0x166790 | `([sp-26]:2 & 0xffff) > ? (signed, 16-bit)` | `([sp-26]:2 & 0xffff) <= ? (signed, 16-bit)` |
| `jne` at 0x1667a2 | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x1667ac | `([sp-38]:1 & 0xff) != 0x0` | `([sp-38]:1 & 0xff) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x166664 | `eax` | `?` |
| `ret` at 0x16669a | `eax` | `?` |

## `_mwGPX`

- `_mwGPX`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x153578 to 0x15361c.
  Established. The two-byte I/O store writes `0x304` to absolute `0x3ce` (`out` at 0x1535c7). Three
  byte outputs use unknown I/O addresses, with value `0x2` (`out` at 0x1535db) followed by unknown
  values (`out` at 0x1535e3, `out` at 0x1535eb). The first indirect call operand is a four-byte load
  through `[sp+12] +0xc4` (`call` at 0x1535a7); its callee and candidate values remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x15357e | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x153581 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x153584 | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x15358f | load | 2 | `[sp+12] +0xa` | `-` |
| `cmp` at 0x153595 | load | 2 | `[sp+12] +0xc` | `-` |
| `dec` at 0x15359b | load | 1 | `[sp+12] +0x98` | `-` |
| `dec` at 0x15359b | store | 1 | `[sp+12] +0x98` | `?` |
| `call` at 0x1535a7 | load | 4 | `[sp+12] +0xc4` | `-` |
| `mov` at 0x1535af | load | 4 | `[sp+12] +0x18` | `-` |
| `mov` at 0x1535b2 | load | 4 | `(unknown)` | `-` |
| `out` at 0x1535c7 | store | 2 | `I/O abs 0x3ce` | `0x304` |
| `mov` at 0x1535ca | load | 1 | `(unknown)` | `-` |
| `mov` at 0x1535d3 | load | 1 | `(unknown)` | `-` |
| `out` at 0x1535db | store | 1 | `I/O (unknown)` | `0x2` |
| `mov` at 0x1535dc | load | 1 | `(unknown)` | `-` |
| `out` at 0x1535e3 | store | 1 | `I/O (unknown)` | `?` |
| `mov` at 0x1535e4 | load | 1 | `(unknown)` | `-` |
| `out` at 0x1535eb | store | 1 | `I/O (unknown)` | `?` |
| `mov` at 0x1535ec | load | 1 | `(unknown)` | `-` |
| `inc` at 0x1535f3 | load | 1 | `[sp+12] +0x98` | `-` |
| `inc` at 0x1535f3 | store | 1 | `[sp+12] +0x98` | `?` |
| `test` at 0x1535fd | load | 2 | `[sp+12] +0x16` | `-` |
| `call` at 0x153605 | load | 4 | `[sp+12] +0xf8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1535a7 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x153605 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `js` at 0x153589 | `N == 1` | `not (N == 1)` |
| `js` at 0x15358d | `N == 1` | `not (N == 1)` |
| `jae` at 0x153593 | `? >= ([[sp+12]+10]:2 & 0xffff) (unsigned, 16-bit)` | `? < ([[sp+12]+10]:2 & 0xffff) (unsigned, 16-bit)` |
| `jae` at 0x153599 | `? >= ([[sp+12]+12]:2 & 0xffff) (unsigned, 16-bit)` | `? < ([[sp+12]+12]:2 & 0xffff) (unsigned, 16-bit)` |
| `js` at 0x1535f9 | `N == 1` | `not (N == 1)` |
| `jg` at 0x1535fb | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `je` at 0x153603 | `(([[sp+12]+22]:2 & 0xffff) & 0x8) == 0x0` | `(([[sp+12]+22]:2 & 0xffff) & 0x8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x153610 | `eax` | `[sp-20]` |

## `_ResetGUIPalette`

- `_ResetGUIPalette`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12ecf0 to 0x12ed20.
  Established. The first unnamed direct call to `0x157fd0` presents leading candidates `0x0`, `0x0`,
  `0xf`, and `0x1ad128` (`call` at 0x12ecfe). The `_QueryColors` site retains unknown candidates
  (`call` at 0x12ed06). The second call to `0x157fd0` has leading candidates `0x0`,
  `ret(_QueryColors)`, `ret(_QueryColors)`, and `0x1ad182` (`call` at 0x12ed14). The pointer roles
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12ecfe | `(unknown callee) 0x157fd0` | `0x0` | `0x0` | `0xf` | `0x1ad128` | `ebp` | `?` |
| `call` at 0x12ed06 | `_QueryColors 0x1621e8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12ed14 | `(unknown callee) 0x157fd0` | `0x0` | `ret(_QueryColors)` | `ret(_QueryColors)` | `0x1ad182` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12ed1c | `eax` | `ret(0x157fd0)` |

## `_mwGPA`

- `_mwGPA`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1533a4 to 0x153430.
  Established. The first indirect call operand is a four-byte load through `[sp+12] +0xc0` (`call`
  at 0x1533df). The following branch is taken when `C == 1` (`jb` at 0x1533e5), without a resolved
  semantic predicate. A byte load later uses an unknown effective address (`mov` at 0x1533f1). The
  byte read-modify-writes through `[sp+12] +0x98` retain unknown stored values (`dec` at 0x1533c7,
  `inc` at 0x1533f4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1533aa | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1533ad | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1533b0 | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x1533bb | load | 2 | `[sp+12] +0xa` | `-` |
| `cmp` at 0x1533c1 | load | 2 | `[sp+12] +0xc` | `-` |
| `dec` at 0x1533c7 | load | 1 | `[sp+12] +0x98` | `-` |
| `dec` at 0x1533c7 | store | 1 | `[sp+12] +0x98` | `?` |
| `cmp` at 0x1533cd | load | 4 | `[sp+12] +0xa4` | `-` |
| `cmp` at 0x1533d5 | load | 4 | `[sp+12] +0xac` | `-` |
| `call` at 0x1533df | load | 4 | `[sp+12] +0xc0` | `-` |
| `mov` at 0x1533e7 | load | 4 | `[sp+12] +0x18` | `-` |
| `mov` at 0x1533ea | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1533f1 | load | 1 | `(unknown)` | `-` |
| `inc` at 0x1533f4 | load | 1 | `[sp+12] +0x98` | `-` |
| `inc` at 0x1533f4 | store | 1 | `[sp+12] +0x98` | `?` |
| `test` at 0x1533fe | load | 2 | `[sp+12] +0x16` | `-` |
| `call` at 0x153406 | load | 4 | `[sp+12] +0xf8` | `-` |
| `call` at 0x153421 | load | 4 | `[sp+12] +0xc0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1533df | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x153406 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x153421 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `js` at 0x1533b5 | `N == 1` | `not (N == 1)` |
| `js` at 0x1533b9 | `N == 1` | `not (N == 1)` |
| `jae` at 0x1533bf | `? >= ([[sp+12]+10]:2 & 0xffff) (unsigned, 16-bit)` | `? < ([[sp+12]+10]:2 & 0xffff) (unsigned, 16-bit)` |
| `jae` at 0x1533c5 | `? >= ([[sp+12]+12]:2 & 0xffff) (unsigned, 16-bit)` | `? < ([[sp+12]+12]:2 & 0xffff) (unsigned, 16-bit)` |
| `jb` at 0x1533d3 | `? < [[sp+12]+164] (unsigned, 32-bit)` | `? >= [[sp+12]+164] (unsigned, 32-bit)` |
| `jbe` at 0x1533db | `? <= [[sp+12]+172] (unsigned, 32-bit)` | `? > [[sp+12]+172] (unsigned, 32-bit)` |
| `jb` at 0x1533e5 | `C == 1` | `not (C == 1)` |
| `js` at 0x1533fa | `N == 1` | `not (N == 1)` |
| `jg` at 0x1533fc | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `je` at 0x153404 | `(([[sp+12]+22]:2 & 0xffff) & 0x8) == 0x0` | `(([[sp+12]+22]:2 & 0xffff) & 0x8) != 0x0` |
| `jae` at 0x15341a | `? >= ret(unknown) (unsigned, 32-bit)` | `? < ret(unknown) (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x153411 | `eax` | `?` |

## `_mwGPM`

- `_mwGPM`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1534e4 to 0x153570.
  Established. A two-byte load reads `[sp+12] +0xa` (`cmp` at 0x1534fb), and its branch is taken
  when `? >= ([[sp+12]+10]:2 & 0xffff) (unsigned, 16-bit)` (`jae` at 0x1534ff). The later indexed
  byte load has an unknown effective address (`mov` at 0x153535). The two indirect call operands are
  four-byte loads through `[sp+12] +0xc0` (`call` at 0x15351f) and `[sp+12] +0xf8` (`call` at
  0x153557). Their targets and candidate slots remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1534ea | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1534ed | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1534f0 | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x1534fb | load | 2 | `[sp+12] +0xa` | `-` |
| `cmp` at 0x153501 | load | 2 | `[sp+12] +0xc` | `-` |
| `dec` at 0x153507 | load | 1 | `[sp+12] +0x98` | `-` |
| `dec` at 0x153507 | store | 1 | `[sp+12] +0x98` | `?` |
| `cmp` at 0x15350d | load | 4 | `[sp+12] +0xa4` | `-` |
| `cmp` at 0x153515 | load | 4 | `[sp+12] +0xac` | `-` |
| `call` at 0x15351f | load | 4 | `[sp+12] +0xc0` | `-` |
| `mov` at 0x153525 | load | 4 | `[sp+12] +0x18` | `-` |
| `mov` at 0x153528 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x153535 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x15353e | load | 1 | `(unknown)` | `-` |
| `inc` at 0x153545 | load | 1 | `[sp+12] +0x98` | `-` |
| `inc` at 0x153545 | store | 1 | `[sp+12] +0x98` | `?` |
| `test` at 0x15354f | load | 2 | `[sp+12] +0x16` | `-` |
| `call` at 0x153557 | load | 4 | `[sp+12] +0xf8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x15351f | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x153557 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `js` at 0x1534f5 | `N == 1` | `not (N == 1)` |
| `js` at 0x1534f9 | `N == 1` | `not (N == 1)` |
| `jae` at 0x1534ff | `? >= ([[sp+12]+10]:2 & 0xffff) (unsigned, 16-bit)` | `? < ([[sp+12]+10]:2 & 0xffff) (unsigned, 16-bit)` |
| `jae` at 0x153505 | `? >= ([[sp+12]+12]:2 & 0xffff) (unsigned, 16-bit)` | `? < ([[sp+12]+12]:2 & 0xffff) (unsigned, 16-bit)` |
| `jb` at 0x153513 | `? < [[sp+12]+164] (unsigned, 32-bit)` | `? >= [[sp+12]+164] (unsigned, 32-bit)` |
| `jbe` at 0x15351b | `? <= [[sp+12]+172] (unsigned, 32-bit)` | `? > [[sp+12]+172] (unsigned, 32-bit)` |
| `js` at 0x15354b | `N == 1` | `not (N == 1)` |
| `jg` at 0x15354d | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `je` at 0x153555 | `(([[sp+12]+22]:2 & 0xffff) & 0x8) == 0x0` | `(([[sp+12]+22]:2 & 0xffff) & 0x8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x153562 | `eax` | `?` |

## `_mwGPP`

- `_mwGPP`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x153624 to 0x1536c8.
  Established. The four-byte read-modify-write at `sp -0x14` stores `[sp-20]-4` (`sub` at 0x153684).
  A subsequent four-byte load uses `[sp-20] +0x0` (`mov` at 0x153688). The repeat edge is taken when
  `N == 0` (`jns` at 0x15369a), with no established iteration count. The indirect call through
  `[sp+12] +0xf8` retains unknown callee and candidate values (`call` at 0x1536af).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x15362d | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x153630 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x153633 | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x15363e | load | 2 | `[sp+12] +0xa` | `-` |
| `cmp` at 0x153644 | load | 2 | `[sp+12] +0xc` | `-` |
| `dec` at 0x15364a | load | 1 | `[sp+12] +0x98` | `-` |
| `dec` at 0x15364a | store | 1 | `[sp+12] +0x98` | `?` |
| `mov` at 0x153655 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x15365c | store | 1 | `sp -0x1c` | `?` |
| `mov` at 0x153662 | store | 4 | `sp -0x18` | `?` |
| `movzx` at 0x153665 | load | 2 | `[sp+12] +0x14` | `-` |
| `mov` at 0x153674 | store | 4 | `sp -0x14` | `?` |
| `call` at 0x15367b | load | 4 | `[sp+12] +0xc4` | `-` |
| `mov` at 0x153681 | load | 4 | `sp -0x14` | `-` |
| `sub` at 0x153684 | load | 4 | `sp -0x14` | `-` |
| `sub` at 0x153684 | store | 4 | `sp -0x14` | `[sp-20]-4` |
| `mov` at 0x153688 | load | 4 | `[sp-20] +0x0` | `-` |
| `mov` at 0x15368a | load | 4 | `(unknown)` | `-` |
| `add` at 0x15368d | load | 4 | `sp -0x18` | `-` |
| `mov` at 0x153690 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x153692 | load | 1 | `sp -0x1c` | `-` |
| `inc` at 0x15369d | load | 1 | `[sp+12] +0x98` | `-` |
| `inc` at 0x15369d | store | 1 | `[sp+12] +0x98` | `?` |
| `test` at 0x1536a7 | load | 2 | `[sp+12] +0x16` | `-` |
| `call` at 0x1536af | load | 4 | `[sp+12] +0xf8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x15367b | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1536af | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `js` at 0x153638 | `N == 1` | `not (N == 1)` |
| `js` at 0x15363c | `N == 1` | `not (N == 1)` |
| `jae` at 0x153642 | `? >= ([[sp+12]+10]:2 & 0xffff) (unsigned, 16-bit)` | `? < ([[sp+12]+10]:2 & 0xffff) (unsigned, 16-bit)` |
| `jae` at 0x153648 | `? >= ([[sp+12]+12]:2 & 0xffff) (unsigned, 16-bit)` | `? < ([[sp+12]+12]:2 & 0xffff) (unsigned, 16-bit)` |
| `jns` at 0x15369a | `N == 0` | `not (N == 0)` |
| `js` at 0x1536a3 | `N == 1` | `not (N == 1)` |
| `jg` at 0x1536a5 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `je` at 0x1536ad | `(([[sp+12]+22]:2 & 0xffff) & 0x8) == 0x0` | `(([[sp+12]+22]:2 & 0xffff) & 0x8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1536bd | `eax` | `[sp-32]` |

## `_mwGP6`

- `_mwGP6`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x153430 to 0x1534e4.
  Established. Two-byte loads at unknown effective addresses occur on separate paths (`movzx` at
  0x15347d, `movzx` at 0x1534c4). The first path includes a byte read-modify-write through `[sp+12]
  +0x98` with unknown stored value (`inc` at 0x153481); the other includes another there (`inc` at
  0x1534c7). The indirect call sites include the four-byte operands through `[sp+12] +0xc0` (`call`
  at 0x15346b, `call` at 0x1534ad) and `[sp+12] +0xf8` (`call` at 0x153493, `call` at 0x1534d9).
  Routine purpose remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x153436 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x153439 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x15343c | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x153447 | load | 2 | `[sp+12] +0xa` | `-` |
| `cmp` at 0x15344d | load | 2 | `[sp+12] +0xc` | `-` |
| `dec` at 0x153453 | load | 1 | `[sp+12] +0x98` | `-` |
| `dec` at 0x153453 | store | 1 | `[sp+12] +0x98` | `?` |
| `cmp` at 0x153459 | load | 4 | `[sp+12] +0xa4` | `-` |
| `cmp` at 0x153461 | load | 4 | `[sp+12] +0xac` | `-` |
| `call` at 0x15346b | load | 4 | `[sp+12] +0xc0` | `-` |
| `mov` at 0x153473 | load | 4 | `[sp+12] +0x18` | `-` |
| `mov` at 0x153476 | load | 4 | `(unknown)` | `-` |
| `movzx` at 0x15347d | load | 2 | `(unknown)` | `-` |
| `inc` at 0x153481 | load | 1 | `[sp+12] +0x98` | `-` |
| `inc` at 0x153481 | store | 1 | `[sp+12] +0x98` | `?` |
| `test` at 0x15348b | load | 2 | `[sp+12] +0x16` | `-` |
| `call` at 0x153493 | load | 4 | `[sp+12] +0xf8` | `-` |
| `call` at 0x1534ad | load | 4 | `[sp+12] +0xc0` | `-` |
| `mov` at 0x1534b8 | load | 4 | `[sp+12] +0x18` | `-` |
| `mov` at 0x1534bb | load | 4 | `(unknown)` | `-` |
| `movzx` at 0x1534c4 | load | 2 | `(unknown)` | `-` |
| `inc` at 0x1534c7 | load | 1 | `[sp+12] +0x98` | `-` |
| `inc` at 0x1534c7 | store | 1 | `[sp+12] +0x98` | `?` |
| `test` at 0x1534d1 | load | 2 | `[sp+12] +0x16` | `-` |
| `call` at 0x1534d9 | load | 4 | `[sp+12] +0xf8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x15346b | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x153493 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1534ad | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1534d9 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `js` at 0x153441 | `N == 1` | `not (N == 1)` |
| `js` at 0x153445 | `N == 1` | `not (N == 1)` |
| `jae` at 0x15344b | `? >= ([[sp+12]+10]:2 & 0xffff) (unsigned, 16-bit)` | `? < ([[sp+12]+10]:2 & 0xffff) (unsigned, 16-bit)` |
| `jae` at 0x153451 | `? >= ([[sp+12]+12]:2 & 0xffff) (unsigned, 16-bit)` | `? < ([[sp+12]+12]:2 & 0xffff) (unsigned, 16-bit)` |
| `jb` at 0x15345f | `? < [[sp+12]+164] (unsigned, 32-bit)` | `? >= [[sp+12]+164] (unsigned, 32-bit)` |
| `jbe` at 0x153467 | `? <= [[sp+12]+172] (unsigned, 32-bit)` | `? > [[sp+12]+172] (unsigned, 32-bit)` |
| `jb` at 0x153471 | `C == 1` | `not (C == 1)` |
| `js` at 0x153487 | `N == 1` | `not (N == 1)` |
| `jg` at 0x153489 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `je` at 0x153491 | `(([[sp+12]+22]:2 & 0xffff) & 0x8) == 0x0` | `(([[sp+12]+22]:2 & 0xffff) & 0x8) != 0x0` |
| `jae` at 0x1534a6 | `? >= ret(unknown) (unsigned, 32-bit)` | `? < ret(unknown) (unsigned, 32-bit)` |
| `js` at 0x1534cd | `N == 1` | `not (N == 1)` |
| `jg` at 0x1534cf | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `je` at 0x1534d7 | `(([[sp+12]+22]:2 & 0xffff) & 0x8) == 0x0` | `(([[sp+12]+22]:2 & 0xffff) & 0x8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x15349e | `eax` | `?` |
| `ret` at 0x1534e3 | `eax` | `?` |
