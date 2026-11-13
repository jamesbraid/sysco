# MMSC VGA setup and keyboard-state routines in the 6.4-origin image

Read from `6.4-origin/usr/cpu/firmware/mmscfw.bin`.
Image SHA-256: `b766e559f86078526916d8bd98bbb8079294ad774cce9177928873153c6fb09b`.
The x86 a.out link address is `0x108000`. The [load map](load-map.md) explains segment mapping.
Analysed spans have exclusive ends inferred from the next text symbol.
Some include literals and padding decoded linearly after returns. Only sites reached
by known control-flow edges contribute effects to the tables.
The unresolved color selector excludes selector-only effects from caller tables.
A bounded aligned supplement covers `0x16deb0` to exclusive `0x16dee0`.
It starts with an independent entry state and covers both otherwise misaligned table targets;
its full fact sheet is retained separately; its effects are not merged into caller tables.
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

[Fact sheets and complete listings](disasm/vga-setup-keys-64-origin-facts.txt) include all tool sections.

## `_initialize`

- `_initialize`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16d2d0 to 0x16d4b0.
  Established. The four-byte store at absolute `0x1d3774` writes `0xa0000` (`mov` at 0x16d32f).
  Other four-byte stores write `0x3d4` to absolute `0x1d33e8` (`mov` at 0x16d3d3), `0x3d5` to
  `0x1d33ec` (`mov` at 0x16d3dd), and `0x3da` to `0x1d33f0` (`mov` at 0x16d3e7). The `_sysOutByte`
  sites include leading pairs `0x3ce`, `0x4` (`call` at 0x16d427, `call` at 0x16d458), `0x3cf`,
  `0x2` (`call` at 0x16d436), and `0x3cf`, `0x3` (`call` at 0x16d467). No aperture or register role
  is inferred from these values.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16d2e0 | store | 4 | `abs 0x1d06dc` | `ret(_et4000_test)` |
| `mov` at 0x16d32f | store | 4 | `abs 0x1d3774` | `0xa0000` |
| `mov` at 0x16d339 | load | 4 | `abs 0x1d33f0` | `-` |
| `mov` at 0x16d38c | store | 1 | `(unknown)` | `?` |
| `mov` at 0x16d3a8 | store | 1 | `(unknown)` | `?` |
| `mov` at 0x16d3c4 | store | 1 | `(unknown)` | `?` |
| `mov` at 0x16d3d3 | store | 4 | `abs 0x1d33e8` | `0x3d4` |
| `mov` at 0x16d3dd | store | 4 | `abs 0x1d33ec` | `0x3d5` |
| `mov` at 0x16d3e7 | store | 4 | `abs 0x1d33f0` | `0x3da` |
| `mov` at 0x16d443 | load | 4 | `abs 0x1d3774` | `-` |
| `rep movsd` at 0x16d44f | store | 4 | `(unknown)` | `?` |
| `rep movsd` at 0x16d44f | load | 4 | `[0x1d3774] +0x0` | `-` |
| `mov` at 0x16d471 | load | 4 | `abs 0x1d3774` | `-` |
| `rep movsd` at 0x16d47d | store | 4 | `(unknown)` | `?` |
| `rep movsd` at 0x16d47d | load | 4 | `[0x1d3774] +0x0` | `-` |
| `mov` at 0x16d484 | store | 1 | `(unknown)` | `0x0` |
| `mov` at 0x16d48f | store | 4 | `abs 0x1d06d4` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16d2d6 | `_get_perm 0x16cd80` | `ebx` | `esi` | `edi` | `ebp` | `?` | `?` |
| `call` at 0x16d2db | `_et4000_test 0x16cdf0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d2f0 | `_sysOutByte 0x1081d0` | `0x3bf` | `0x3` | `?` | `?` | `?` | `?` |
| `call` at 0x16d2fd | `_sysInByte 0x1081b0` | `0x3cc` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d327 | `_sysOutByte 0x1081d0` | `?` | `0xffffffa0` | `?` | `?` | `?` | `?` |
| `call` at 0x16d33f | `_sysInByte 0x1081b0` | `[0x1d33f0]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d34e | `_sysOutByte 0x1081d0` | `0x3c0` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x16d35b | `_save_regs 0x16d040` | `0x1d33f8` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d36a | `_sysOutByte 0x1081d0` | `0x3c7` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x16d384 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d3a0 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d3bc | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d3f6 | `_sysInByte 0x1081b0` | `0x3cc` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d409 | `_sysOutByte 0x1081d0` | `0x3c2` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d418 | `_set_regs 0x16d130` | `0x1d00fc` | `0x4` | `?` | `?` | `?` | `?` |
| `call` at 0x16d427 | `_sysOutByte 0x1081d0` | `0x3ce` | `0x4` | `?` | `?` | `?` | `?` |
| `call` at 0x16d436 | `_sysOutByte 0x1081d0` | `0x3cf` | `0x2` | `?` | `?` | `?` | `?` |
| `call` at 0x16d458 | `_sysOutByte 0x1081d0` | `0x3ce` | `0x4` | `?` | `?` | `?` | `?` |
| `call` at 0x16d467 | `_sysOutByte 0x1081d0` | `0x3cf` | `0x3` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x16d2e7 | `(ret(_et4000_test) & ret(_et4000_test)) == 0x0` | `(ret(_et4000_test) & ret(_et4000_test)) != 0x0` |
| `je` at 0x16d307 | `(? & 0x1) == 0x0` | `(? & 0x1) != 0x0` |
| `jns` at 0x16d37d | `N == 0` | `not (N == 0)` |
| `jns` at 0x16d399 | `N == 0` | `not (N == 0)` |
| `jns` at 0x16d3b5 | `N == 0` | `not (N == 0)` |
| `jle` at 0x16d3d1 | `? <= 0xff (signed, 32-bit)` | `? > 0xff (signed, 32-bit)` |
| `jge` at 0x16d48d | `? >= 0x1d7778 (signed, 32-bit)` | `? < 0x1d7778 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16d4a2 | `eax` | `?` |

## `_set_regs`

- `_set_regs`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16d130 to 0x16d2d0.
  Established. The byte load reads through `[sp+4] +0x3b` (`movsx` at 0x16d159). The `_sysOutByte`
  sites include leading pairs `0x3c4`, `0x0` (`call` at 0x16d172, `call` at 0x16d1c1), `0x3c5`,
  `0x1` (`call` at 0x16d181), and `0x3c5`, `0x3` (`call` at 0x16d1d0). A later site presents
  `[0x1d33e8]` and `0x11` (`call` at 0x16d1e0). The `_et4000_set_regs` call has candidate zero
  `[sp+4]` (`call` at 0x16d2b7). Callee effects and register identities remain unassigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16d136 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x16d139 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x16d13c | load | 4 | `abs 0x1d33f0` | `-` |
| `movsx` at 0x16d159 | load | 1 | `[sp+4] +0x3b` | `-` |
| `movsx` at 0x16d1a1 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x16d1da | load | 4 | `abs 0x1d33e8` | `-` |
| `mov` at 0x16d1e8 | load | 4 | `abs 0x1d33ec` | `-` |
| `mov` at 0x16d1fa | load | 4 | `abs 0x1d33ec` | `-` |
| `mov` at 0x16d210 | load | 4 | `abs 0x1d33e8` | `-` |
| `movsx` at 0x16d21e | load | 1 | `(unknown)` | `-` |
| `mov` at 0x16d223 | load | 4 | `abs 0x1d33ec` | `-` |
| `movsx` at 0x16d24d | load | 1 | `(unknown)` | `-` |
| `mov` at 0x16d268 | load | 4 | `abs 0x1d33f0` | `-` |
| `movsx` at 0x16d287 | load | 1 | `(unknown)` | `-` |
| `cmp` at 0x16d2a0 | load | 4 | `abs 0x1d06dc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16d142 | `_sysInByte 0x1081b0` | `[0x1d33f0]` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x16d151 | `_sysOutByte 0x1081d0` | `0x3c0` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x16d163 | `_sysOutByte 0x1081d0` | `0x3c2` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d172 | `_sysOutByte 0x1081d0` | `0x3c4` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x16d181 | `_sysOutByte 0x1081d0` | `0x3c5` | `0x1` | `?` | `?` | `?` | `?` |
| `call` at 0x16d199 | `_sysOutByte 0x1081d0` | `0x3c4` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d1ac | `_sysOutByte 0x1081d0` | `0x3c5` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d1c1 | `_sysOutByte 0x1081d0` | `0x3c4` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x16d1d0 | `_sysOutByte 0x1081d0` | `0x3c5` | `0x3` | `?` | `?` | `?` | `?` |
| `call` at 0x16d1e0 | `_sysOutByte 0x1081d0` | `[0x1d33e8]` | `0x11` | `?` | `?` | `?` | `?` |
| `call` at 0x16d1ee | `_sysInByte 0x1081b0` | `[0x1d33ec]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d200 | `_sysOutByte 0x1081d0` | `[0x1d33ec]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d216 | `_sysOutByte 0x1081d0` | `[0x1d33e8]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d229 | `_sysOutByte 0x1081d0` | `[0x1d33ec]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d245 | `_sysOutByte 0x1081d0` | `0x3ce` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d258 | `_sysOutByte 0x1081d0` | `0x3cf` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d26e | `_sysInByte 0x1081b0` | `[0x1d33f0]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d27f | `_sysOutByte 0x1081d0` | `0x3c0` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d292 | `_sysOutByte 0x1081d0` | `0x3c0` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d2b7 | `_et4000_set_regs 0x16cf80` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x16d1b8 | `? <= 0x4 (signed, 32-bit)` | `? > 0x4 (signed, 32-bit)` |
| `jle` at 0x16d235 | `? <= 0x17 (signed, 32-bit)` | `? > 0x17 (signed, 32-bit)` |
| `jle` at 0x16d264 | `? <= 0x8 (signed, 32-bit)` | `? > 0x8 (signed, 32-bit)` |
| `jle` at 0x16d29e | `? <= 0x14 (signed, 32-bit)` | `? > 0x14 (signed, 32-bit)` |
| `je` at 0x16d2a7 | `[0x1d06dc] == 0x0` | `[0x1d06dc] != 0x0` |
| `jbe` at 0x16d2af | `[sp+8]-10 <= 0x1 (unsigned, 32-bit)` | `[sp+8]-10 > 0x1 (unsigned, 32-bit)` |
| `jne` at 0x16d2b4 | `[sp+8] != 0xc` | `[sp+8] == 0xc` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16d2c5 | `eax` | `?` |

## `_vga_hasmode`

- `_vga_hasmode`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16da80 to 0x16dad0.
  Established. The first signed comparison branch is taken when `[sp+4] > 0x9 (signed, 32-bit)`
  (`jg` at 0x16da8d). The next is taken when `[sp+4] > 0xc (signed, 32-bit)` (`jg` at 0x16daa3). The
  separate exits carry `0x1` (`ret` at 0x16da97), `ret(_et4000_test)` (`ret` at 0x16dab2), and `0x0`
  (`ret` at 0x16dac5). Those values do not independently establish a supported-mode catalogue.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16da83 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16daa5 | `_get_perm 0x16cd80` | `ebp` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16daaa | `_et4000_test 0x16cdf0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jl` at 0x16da88 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jg` at 0x16da8d | `[sp+4] > 0x9 (signed, 32-bit)` | `[sp+4] <= 0x9 (signed, 32-bit)` |
| `jg` at 0x16daa3 | `[sp+4] > 0xc (signed, 32-bit)` | `[sp+4] <= 0xc (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16da97 | `eax` | `0x1` |
| `ret` at 0x16dab2 | `eax` | `ret(_et4000_test)` |
| `ret` at 0x16dac5 | `eax` | `0x0` |

## `_vga_setcolor`

- `_vga_setcolor`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16de60 to 0x16def0.
  Established. Four-byte loads read `sp +0x4` (`mov` at 0x16de64), absolute `0x1d06d0` (`mov` at
  0x16de67), and an unknown effective address at the computed transfer (`jmp` at 0x16de72). That
  transfer remains unresolved, so selector-only calls and stores are excluded. The known default
  path exits with `0x0` in `eax` (`ret` at 0x16deee). A color-setting effect is not inferred.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16de64 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x16de67 | load | 4 | `abs 0x1d06d0` | `-` |
| `jmp` at 0x16de72 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x16dee8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x16de70 | `? > 0xb (unsigned, 32-bit)` | `? <= 0xb (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16deee | `eax` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x16de72).

## `_vga_screenoff`

- `_vga_screenoff`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16e050 to 0x16e090.
  Established. The first `_sysOutByte` site presents leading candidates `0x3c4` and `0x1` (`call` at
  0x16e05a). The `_sysInByte` site presents candidate zero `0x3c5` (`call` at 0x16e067). The later
  `_sysOutByte` site also has candidate zero `0x3c5`, with unknown candidate one (`call` at
  0x16e07a). The exit reports `0x0` (`ret` at 0x16e084), without establishing display blanking.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16e05a | `_sysOutByte 0x1081d0` | `0x3c4` | `0x1` | `ebp` | `?` | `?` | `?` |
| `call` at 0x16e067 | `_sysInByte 0x1081b0` | `0x3c5` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16e07a | `_sysOutByte 0x1081d0` | `0x3c5` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16e084 | `eax` | `0x0` |

## `_vga_screenon`

- `_vga_screenon`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16e090 to 0x16e0d0.
  Established. The `_sysOutByte` site presents `0x3c4` and `0x1` as its first two candidates (`call`
  at 0x16e09a). A `_sysInByte` call uses candidate zero `0x3c5` (`call` at 0x16e0a7). The final
  `_sysOutByte` site retains candidate zero `0x3c5` and unknown candidate one (`call` at 0x16e0ba).
  The literal-zero exit follows (`ret` at 0x16e0c4). Screen-enabling semantics remain unassigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16e09a | `_sysOutByte 0x1081d0` | `0x3c4` | `0x1` | `ebp` | `?` | `?` | `?` |
| `call` at 0x16e0a7 | `_sysInByte 0x1081b0` | `0x3c5` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16e0ba | `_sysOutByte 0x1081d0` | `0x3c5` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16e0c4 | `eax` | `0x0` |

## `_vga_getxdim`

- `_vga_getxdim`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16e0d0 to 0x16e0e0.
  Established. The sole explicit four-byte load reads absolute `0x1d3760` (`mov` at 0x16e0d3); its
  expression `[0x1d3760]` reaches `eax` at the exit (`ret` at 0x16e0db). A horizontal-dimension role
  is unknown from this body alone.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16e0d3 | load | 4 | `abs 0x1d3760` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16e0db | `eax` | `[0x1d3760]` |

## `_vga_getydim`

- `_vga_getydim`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16e0e0 to 0x16e0f0.
  Established. A four-byte load reads absolute `0x1d3764` (`mov` at 0x16e0e3). The exit retains
  `[0x1d3764]` in `eax` (`ret` at 0x16e0eb). Neither the value nor a vertical-dimension unit is
  established here.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16e0e3 | load | 4 | `abs 0x1d3764` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16e0eb | `eax` | `[0x1d3764]` |

## `_vga_getcolors`

- `_vga_getcolors`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16e0f0 to 0x16e100.
  Established. The four-byte load at absolute `0x1d3768` supplies `[0x1d3768]` (`mov` at 0x16e0f3),
  which the exit reports in `eax` (`ret` at 0x16e0fb). Post-return linear data decodes contribute no
  additional effects. A color-count role remains unassigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16e0f3 | load | 4 | `abs 0x1d3768` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16e0fb | `eax` | `[0x1d3768]` |

## `_kbdNormal`

- `_kbdNormal`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10afa8 to 0x10b028.
  Established. The first branch is taken when `(([0x1d2741]:1 & 0xff) & 0x2) != 0x0` (`jne` at
  0x10afbc). The two-byte load reads absolute `0x1d2742` (`movzx` at 0x10afc9). A later byte load
  has an unknown effective address (`mov` at 0x10afe3). The `_tyIRd` site retains unknown leading
  candidate slots (`call` at 0x10b01b). No keymap or character-delivery semantics is inferred.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10afac | load | 1 | `sp +0x4` | `-` |
| `mov` at 0x10afaf | load | 4 | `abs 0x1d2744` | `-` |
| `test` at 0x10afb5 | load | 1 | `abs 0x1d2741` | `-` |
| `mov` at 0x10afbe | load | 4 | `abs 0x1d273c` | `-` |
| `movzx` at 0x10afc9 | load | 2 | `abs 0x1d2742` | `-` |
| `mov` at 0x10afe3 | load | 1 | `(unknown)` | `-` |
| `test` at 0x10afef | load | 1 | `abs 0x1d2740` | `-` |
| `mov` at 0x10b020 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b01b | `_tyIRd 0x16e970` | `?` | `?` | `ebx` | `ebp` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10afbc | `(([0x1d2741]:1 & 0xff) & 0x2) != 0x0` | `(([0x1d2741]:1 & 0xff) & 0x2) == 0x0` |
| `je` at 0x10afe9 | `? == 0xff` | `? != 0xff` |
| `je` at 0x10afed | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x10aff6 | `(([0x1d2740]:1 & 0xff) & 0x4) == 0x0` | `(([0x1d2740]:1 & 0xff) & 0x4) != 0x0` |
| `ja` at 0x10affe | `? > 0x19 (unsigned, 8-bit)` | `? <= 0x19 (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b026 | `eax` | `?` |

## `_kbdShift`

- `_kbdShift`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b028 to 0x10b068.
  Established. The branch is taken when `(([0x1d2741]:1 & 0xff) & 0x2) == 0x0` (`je` at 0x10b032),
  reaching a two-byte store of `0x1` at absolute `0x1d2742` (`mov` at 0x10b048). The other arm
  stores two-byte `0x0` at the same address (`mov` at 0x10b034). Byte read-modify-write stores at
  absolute `0x1d2740` retain unknown values (`and` at 0x10b03d, `or` at 0x10b051). Shift-state
  meanings remain unassigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `test` at 0x10b02b | load | 1 | `abs 0x1d2741` | `-` |
| `mov` at 0x10b034 | store | 2 | `abs 0x1d2742` | `0x0` |
| `and` at 0x10b03d | load | 1 | `abs 0x1d2740` | `-` |
| `and` at 0x10b03d | store | 1 | `abs 0x1d2740` | `?` |
| `mov` at 0x10b048 | store | 2 | `abs 0x1d2742` | `0x1` |
| `or` at 0x10b051 | load | 1 | `abs 0x1d2740` | `-` |
| `or` at 0x10b051 | store | 1 | `abs 0x1d2740` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10b032 | `(([0x1d2741]:1 & 0xff) & 0x2) == 0x0` | `(([0x1d2741]:1 & 0xff) & 0x2) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b047 | `eax` | `eax` |
| `ret` at 0x10b05b | `eax` | `eax` |

## `_kbdCtrl`

- `_kbdCtrl`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b068 to 0x10b0a8.
  Established. The deciding branch falls through when `(([0x1d2741]:1 & 0xff) & 0x2) != 0x0` (`je`
  at 0x10b072), reaching a two-byte store of `0x0` at absolute `0x1d2742` (`mov` at 0x10b074); its
  taken arm stores two-byte `0x2` at that address (`mov` at 0x10b088). The byte read-modify-write
  stores at absolute `0x1d2740` retain unknown values (`and` at 0x10b07d, `or` at 0x10b091). No
  control-key interpretation is assigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `test` at 0x10b06b | load | 1 | `abs 0x1d2741` | `-` |
| `mov` at 0x10b074 | store | 2 | `abs 0x1d2742` | `0x0` |
| `and` at 0x10b07d | load | 1 | `abs 0x1d2740` | `-` |
| `and` at 0x10b07d | store | 1 | `abs 0x1d2740` | `?` |
| `mov` at 0x10b088 | store | 2 | `abs 0x1d2742` | `0x2` |
| `or` at 0x10b091 | load | 1 | `abs 0x1d2740` | `-` |
| `or` at 0x10b091 | store | 1 | `abs 0x1d2740` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10b072 | `(([0x1d2741]:1 & 0xff) & 0x2) == 0x0` | `(([0x1d2741]:1 & 0xff) & 0x2) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b087 | `eax` | `eax` |
| `ret` at 0x10b09b | `eax` | `eax` |

## `_kbdCaps`

- `_kbdCaps`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b0a8 to 0x10b0c8.
  Established. The branch falls through when `(([0x1d2741]:1 & 0xff) & 0x2) == 0x0` (`jne` at
  0x10b0b2), reaching a byte read-modify-write at absolute `0x1d2740` whose stored value is unknown
  (`xor` at 0x10b0b4). The `_kbdLedSet` site retains unknown candidate slots (`call` at 0x10b0bb).
  The merged exit also remains unknown (`ret` at 0x10b0c3). No caps-state or LED-mask meaning is
  inferred.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `test` at 0x10b0ab | load | 1 | `abs 0x1d2741` | `-` |
| `xor` at 0x10b0b4 | load | 1 | `abs 0x1d2740` | `-` |
| `xor` at 0x10b0b4 | store | 1 | `abs 0x1d2740` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b0bb | `_kbdLedSet 0x10b2c8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10b0b2 | `(([0x1d2741]:1 & 0xff) & 0x2) != 0x0` | `(([0x1d2741]:1 & 0xff) & 0x2) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b0c3 | `eax` | `?` |

## `_kbdNum`

- `_kbdNum`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b0c8 to 0x10b108.
  Established. The byte read-modify-write at absolute `0x1d2740` retains an unknown stored value
  (`xor` at 0x10b0d4). A later branch is taken when `(([0x1d2740]:1 & 0xff) & 0x2) != 0x0` (`jne` at
  0x10b0e4). The two-byte store at absolute `0x1d2742` has an unknown value (`mov` at 0x10b0eb). The
  `_kbdLedSet` call has unknown candidates (`call` at 0x10b0f1). Numeric-keypad semantics remain
  unassigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `test` at 0x10b0cb | load | 1 | `abs 0x1d2741` | `-` |
| `xor` at 0x10b0d4 | load | 1 | `abs 0x1d2740` | `-` |
| `xor` at 0x10b0d4 | store | 1 | `abs 0x1d2740` | `?` |
| `test` at 0x10b0dd | load | 1 | `abs 0x1d2740` | `-` |
| `mov` at 0x10b0eb | store | 2 | `abs 0x1d2742` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b0f1 | `_kbdLedSet 0x10b2c8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10b0d2 | `(([0x1d2741]:1 & 0xff) & 0x2) != 0x0` | `(([0x1d2741]:1 & 0xff) & 0x2) == 0x0` |
| `jne` at 0x10b0e4 | `(([0x1d2740]:1 & 0xff) & 0x2) != 0x0` | `(([0x1d2740]:1 & 0xff) & 0x2) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b0f9 | `eax` | `?` |

## `_kbdStp`

- `_kbdStp`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b108 to 0x10b168.
  Established. A four-byte load reads absolute `0x1d2744` (`mov` at 0x10b10b). The byte
  read-modify-write at absolute `0x1d2740` retains an unknown stored value (`xor` at 0x10b11a). The
  later branch is taken when `(([0x1d2740]:1 & 0xff) & 0x1) == 0x0` (`je` at 0x10b128). Both
  `_tyIRd` (`call` at 0x10b14e) and `_kbdLedSet` (`call` at 0x10b156) retain unknown candidates. No
  flow-control character is assigned from the raw pushes.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b10b | load | 4 | `abs 0x1d2744` | `-` |
| `test` at 0x10b111 | load | 1 | `abs 0x1d2741` | `-` |
| `xor` at 0x10b11a | load | 1 | `abs 0x1d2740` | `-` |
| `xor` at 0x10b11a | store | 1 | `abs 0x1d2740` | `?` |
| `test` at 0x10b121 | load | 1 | `abs 0x1d2740` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b14e | `_tyIRd 0x16e970` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b156 | `_kbdLedSet 0x10b2c8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10b118 | `(([0x1d2741]:1 & 0xff) & 0x2) != 0x0` | `(([0x1d2741]:1 & 0xff) & 0x2) == 0x0` |
| `je` at 0x10b128 | `(([0x1d2740]:1 & 0xff) & 0x1) == 0x0` | `(([0x1d2740]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b15e | `eax` | `?` |
