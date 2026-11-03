# MMSC SRAM and flash command paths in the 6.4-origin image

Read from `6.4-origin/usr/cpu/firmware/mmscfw.bin`.
Image SHA-256: `b766e559f86078526916d8bd98bbb8079294ad774cce9177928873153c6fb09b`.
The x86 a.out link address is `0x108000`. The [load map](load-map.md) explains segment mapping.
Analysed spans have exclusive ends inferred from the next text symbol.
Some include literals and padding decoded linearly after returns. Only sites reached
by known control-flow edges contribute effects to the tables.
Four undecoded bytes from `0x1180fc` to exclusive `0x118100` were read directly:
`20 25 73 00`. No instruction effects are credited to them.
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

[Fact sheets and complete listings](disasm/sram-download-64-origin-facts.txt) include all tool sections.

## `_sram_read`

- `_sram_read`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10c928 to 0x10ca08.
  Established. The two-byte load reads absolute `0xa2000` (`mov` at 0x10c942); its branch is taken
  when `([0xa2000]:2 & 0xffff) == 0xaa55` (`je` at 0x10c94c). A later four-byte load uses absolute
  `0xa2004` (`mov` at 0x10c981). The byte stores include value `0x0` at an unknown effective address
  (`mov` at 0x10c98c) and an unknown value at another unknown effective address (`mov` at 0x10c9ca).
  The `_sram_off` site retains unknown candidates (`call` at 0x10c9ef).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10c93a | store | 4 | `sp -0x8` | `ret(_sram_on)` |
| `mov` at 0x10c942 | load | 2 | `abs 0xa2000` | `-` |
| `cmp` at 0x10c958 | load | 4 | `abs 0x1aa4e0` | `-` |
| `mov` at 0x10c968 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10c981 | load | 4 | `abs 0xa2004` | `-` |
| `mov` at 0x10c98c | store | 1 | `(unknown)` | `0x0` |
| `mov` at 0x10c9a8 | store | 4 | `abs 0x1aa4e0` | `0x0` |
| `mov` at 0x10c9b8 | load | 4 | `sp +0xc` | `-` |
| `cmp` at 0x10c9be | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10c9c4 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10c9c7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10c9ca | store | 1 | `(unknown)` | `?` |
| `cmp` at 0x10c9cf | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10c9d4 | load | 4 | `sp +0xc` | `-` |
| `add` at 0x10c9d7 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10c9eb | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10c935 | `_sram_on 0x10cac8` | `ebx` | `esi` | `edi` | `?` | `ebp` | `?` |
| `call` at 0x10c9ef | `_sram_off 0x10caf8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10c94c | `([0xa2000]:2 & 0xffff) == 0xaa55` | `([0xa2000]:2 & 0xffff) != 0xaa55` |
| `je` at 0x10c95f | `[0x1aa4e0] == 0x0` | `[0x1aa4e0] != 0x0` |
| `jbe` at 0x10c979 | `? <= 0x1ff7 (unsigned, 32-bit)` | `? > 0x1ff7 (unsigned, 32-bit)` |
| `je` at 0x10c986 | `Z == 1` | `not (Z == 1)` |
| `jbe` at 0x10c997 | `? <= 0x1ff7 (unsigned, 32-bit)` | `? > 0x1ff7 (unsigned, 32-bit)` |
| `jne` at 0x10c9b4 | `(0x0 & 0x0) != 0x0` | `(0x0 & 0x0) == 0x0` |
| `jle` at 0x10c9c1 | `[sp+8] <= 0x0 (signed, 32-bit)` | `[sp+8] > 0x0 (signed, 32-bit)` |
| `jg` at 0x10c9d2 | `[sp+8] > ? (signed, 32-bit)` | `[sp+8] <= ? (signed, 32-bit)` |
| `jbe` at 0x10c9e4 | `([sp+12] + [sp+8]) <= 0x1ff8 (unsigned, 32-bit)` | `([sp+12] + [sp+8]) > 0x1ff8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10c9ff | `eax` | `?` |

## `_sram_write`

- `_sram_write`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ca08 to 0x10cac8.
  Established. The comparison falls through when `([0xa2000]:2 & 0xffff) != 0xaa55` (`je` at
  0x10ca31), reaching a two-byte store of `0xaa55` at absolute `0xa2000` (`mov` at 0x10ca33) and
  byte stores of `0x0` at absolute `0xa2002` (`mov` at 0x10ca3c) and `0xa2003` (`mov` at 0x10ca43).
  Four-byte stores at absolute `0xa2004` write `0x0` (`mov` at 0x10ca6c) and later unknown (`mov` at
  0x10ca90). The final bound branch is taken when `([sp+12] + [sp+8]) <= 0x1ff8 (unsigned, 32-bit)`
  (`jbe` at 0x10caa5).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10ca13 | store | 4 | `sp -0xc` | `0x0` |
| `mov` at 0x10ca1f | store | 4 | `sp -0x8` | `ret(_sram_on)` |
| `mov` at 0x10ca27 | load | 2 | `abs 0xa2000` | `-` |
| `mov` at 0x10ca33 | store | 2 | `abs 0xa2000` | `0xaa55` |
| `mov` at 0x10ca3c | store | 1 | `abs 0xa2002` | `0x0` |
| `mov` at 0x10ca43 | store | 1 | `abs 0xa2003` | `0x0` |
| `mov` at 0x10ca4c | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10ca52 | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x10ca55 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x10ca5c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10ca5f | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10ca62 | store | 1 | `(unknown)` | `?` |
| `cmp` at 0x10ca67 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10ca6c | store | 4 | `abs 0xa2004` | `0x0` |
| `mov` at 0x10ca78 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10ca90 | store | 4 | `abs 0xa2004` | `?` |
| `mov` at 0x10ca93 | load | 4 | `sp +0xc` | `-` |
| `add` at 0x10ca96 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10ca99 | store | 4 | `sp -0xc` | `0x1` |
| `mov` at 0x10caa7 | store | 4 | `sp -0xc` | `0xffffffff` |
| `mov` at 0x10caae | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10cab7 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ca1a | `_sram_on 0x10cac8` | `ebx` | `esi` | `edi` | `0x0` | `?` | `ebp` |
| `call` at 0x10cab2 | `_sram_off 0x10caf8` | `[sp-8]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10ca31 | `([0xa2000]:2 & 0xffff) == 0xaa55` | `([0xa2000]:2 & 0xffff) != 0xaa55` |
| `jge` at 0x10ca58 | `[sp-12] >= [sp+8] (signed, 32-bit)` | `[sp-12] < [sp+8] (signed, 32-bit)` |
| `jg` at 0x10ca6a | `[sp+8] > ? (signed, 32-bit)` | `[sp+8] <= ? (signed, 32-bit)` |
| `jbe` at 0x10ca89 | `? <= 0x1ff7 (unsigned, 32-bit)` | `? > 0x1ff7 (unsigned, 32-bit)` |
| `jbe` at 0x10caa5 | `([sp+12] + [sp+8]) <= 0x1ff8 (unsigned, 32-bit)` | `([sp+12] + [sp+8]) > 0x1ff8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10cac3 | `eax` | `[sp-12]` |

## `_sram_on`

- `_sram_on`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10cac8 to 0x10caf8.
  Established. Two `_sysOutByte` sites have leading pairs `0x548`, `0x0` (`call` at 0x10cada) and
  `0x760`, `0xffffff81` (`call` at 0x10cae9). The entry also calls `_intLock` (`call` at 0x10cacc).
  The exit reports `ret(_intLock)` in `eax` (`ret` at 0x10caf6). Port meanings and callee effects
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10caf0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10cacc | `_intLock 0x16fb78` | `ebx` | `ebp` | `?` | `?` | `?` | `?` |
| `call` at 0x10cada | `_sysOutByte 0x1081d0` | `0x548` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10cae9 | `_sysOutByte 0x1081d0` | `0x760` | `0xffffff81` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10caf6 | `eax` | `ret(_intLock)` |

## `_sram_off`

- `_sram_off`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10caf8 to 0x10cb28.
  Established. The `_sysOutByte` call presents leading candidates `0x540` and `0x0` (`call` at
  0x10cb06). The later `_intUnlock` site has candidate zero `[sp+4]` (`call` at 0x10cb0f); its
  result expression `ret(_intUnlock)` reaches `eax` at the exit (`ret` at 0x10cb1a). No hardware
  mapping effect is inferred from the name.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10cafc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10cb14 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10cb06 | `_sysOutByte 0x1081d0` | `0x540` | `0x0` | `ebx` | `ebp` | `?` | `?` |
| `call` at 0x10cb0f | `_intUnlock 0x16fb88` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10cb1a | `eax` | `ret(_intUnlock)` |

## `_openFlash`

- `_openFlash`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10cd84 to 0x10d0c4.
  Established. Byte stores using the printed target `ret(_openFlashWindow) +0x0` include `0x90`
  (`mov` at 0x10cdc2, `mov` at 0x10ce09), `0xff` (`mov` at 0x10cdca, `mov` at 0x10ce11), `0x50`
  (`mov` at 0x10ce71, `mov` at 0x10cf34), `0x20` (`mov` at 0x10cea7, `mov` at 0x10cf6a), and `0xd0`
  (`mov` at 0x10cedd, `mov` at 0x10cfa0). The initial `_openFlashWindow` sites present leading
  candidates `[sp+4]`, `0x0`, `0xff` (`call` at 0x10cdb8) and `[sp+4]`, `0x20`, `0xff` (`call` at
  0x10cdff). These sites do not establish command semantics or equal pointers across different call
  results.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10cd8d | store | 1 | `sp -0x8` | `0x0` |
| `mov` at 0x10cd91 | store | 4 | `sp -0x10` | `0x1` |
| `mov` at 0x10cd98 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10cd9b | store | 4 | `[sp+4] +0x4` | `0x1` |
| `mov` at 0x10cda2 | store | 4 | `sp -0x14` | `0x0` |
| `mov` at 0x10cda9 | store | 4 | `[sp+4] +0xc` | `0x0` |
| `mov` at 0x10cdc2 | store | 1 | `ret(_openFlashWindow) +0x0` | `0x90` |
| `mov` at 0x10cdc5 | load | 1 | `ret(_openFlashWindow) +0x0` | `-` |
| `mov` at 0x10cdc7 | load | 1 | `ret(_openFlashWindow) +0x1` | `-` |
| `mov` at 0x10cdca | store | 1 | `ret(_openFlashWindow) +0x0` | `0xff` |
| `mov` at 0x10cdd6 | store | 4 | `sp -0x14` | `0xfffffffa` |
| `mov` at 0x10cde4 | store | 4 | `sp -0x14` | `0xfffffff9` |
| `mov` at 0x10cdfb | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10ce09 | store | 1 | `ret(_openFlashWindow) +0x0` | `0x90` |
| `mov` at 0x10ce0c | load | 1 | `ret(_openFlashWindow) +0x0` | `-` |
| `mov` at 0x10ce0e | load | 1 | `ret(_openFlashWindow) +0x1` | `-` |
| `mov` at 0x10ce11 | store | 1 | `ret(_openFlashWindow) +0x0` | `0xff` |
| `mov` at 0x10ce1d | store | 4 | `sp -0x10` | `0x0` |
| `cmp` at 0x10ce24 | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x10ce37 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10ce42 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10ce4e | store | 4 | `sp -0xc` | `0x0` |
| `mov` at 0x10ce58 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x10ce5d | store | 4 | `sp -0x18` | `?` |
| `mov` at 0x10ce63 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10ce71 | store | 1 | `ret(_openFlashWindow) +0x0` | `0x50` |
| `cmp` at 0x10ce83 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10ce8e | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10cea7 | store | 1 | `ret(_openFlashWindow) +0x0` | `0x20` |
| `cmp` at 0x10ceb9 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10cec4 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10cedd | store | 1 | `ret(_openFlashWindow) +0x0` | `0xd0` |
| `cmp` at 0x10ceef | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10cefa | load | 4 | `sp +0xc` | `-` |
| `cmp` at 0x10cf13 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x10cf1f | load | 4 | `sp -0x18` | `-` |
| `mov` at 0x10cf26 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10cf34 | store | 1 | `ret(_openFlashWindow) +0x0` | `0x50` |
| `cmp` at 0x10cf46 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10cf51 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10cf6a | store | 1 | `ret(_openFlashWindow) +0x0` | `0x20` |
| `cmp` at 0x10cf7c | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10cf87 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10cfa0 | store | 1 | `ret(_openFlashWindow) +0x0` | `0xd0` |
| `cmp` at 0x10cfb2 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10cfbd | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10cfdb | load | 4 | `sp -0x18` | `-` |
| `mov` at 0x10cfdf | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10cff4 | load | 1 | `ret(_openFlashWindow) +0x0` | `-` |
| `cmp` at 0x10cffa | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x10d005 | load | 4 | `sp -0x18` | `-` |
| `mov` at 0x10d00c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d010 | store | 4 | `sp -0x1c` | `?` |
| `mov` at 0x10d01d | load | 1 | `ret(_openFlashWindow) +0x0` | `-` |
| `mov` at 0x10d01f | store | 1 | `sp -0x8` | `[ret(_openFlashWindow)]:1` |
| `mov` at 0x10d022 | load | 4 | `sp -0x1c` | `-` |
| `cmp` at 0x10d03c | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x10d042 | load | 1 | `sp -0x8` | `-` |
| `mov` at 0x10d052 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x10d063 | store | 4 | `sp -0x14` | `0xfffffff8` |
| `cmp` at 0x10d074 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10d07f | load | 4 | `sp +0xc` | `-` |
| `inc` at 0x10d098 | load | 4 | `sp -0xc` | `-` |
| `inc` at 0x10d098 | store | 4 | `sp -0xc` | `?` |
| `cmp` at 0x10d09b | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x10d0a5 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d0ae | load | 4 | `sp -0x14` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10cdb8 | `_openFlashWindow 0x10cc14` | `[sp+4]` | `0x0` | `0xff` | `?` | `?` | `?` |
| `call` at 0x10cdff | `_openFlashWindow 0x10cc14` | `[sp+4]` | `0x20` | `0xff` | `?` | `?` | `?` |
| `call` at 0x10ce2f | `_ffscMsg 0x11ebd0` | `0x10cce7` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ce46 | `_fdprintf 0x171660` | `[sp+12]` | `0x10ccfe` | `?` | `?` | `?` | `?` |
| `call` at 0x10ce67 | `_openFlashWindow 0x10cc14` | `[sp+4]` | `?` | `0x40` | `?` | `?` | `?` |
| `call` at 0x10ce7b | `_nanosleep 0x179570` | `0x1aa4ec` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10ce92 | `_fdprintf 0x171660` | `[sp+12]` | `0x10cd4a` | `?` | `?` | `?` | `?` |
| `call` at 0x10ce9f | `_ffscMsgN 0x11ec30` | `0x10cd4a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ceb1 | `_nanosleep 0x179570` | `0x1aa4ec` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10cec8 | `_fdprintf 0x171660` | `[sp+12]` | `0x10cd4a` | `?` | `?` | `?` | `?` |
| `call` at 0x10ced5 | `_ffscMsgN 0x11ec30` | `0x10cd4a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10cee7 | `_nanosleep 0x179570` | `0x1aa4ec` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10cefe | `_fdprintf 0x171660` | `[sp+12]` | `0x10cd4a` | `?` | `?` | `?` | `?` |
| `call` at 0x10cf0b | `_ffscMsgN 0x11ec30` | `0x10cd4a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10cf2a | `_openFlashWindow 0x10cc14` | `[sp+4]` | `[sp-24]+32` | `0x40` | `?` | `?` | `?` |
| `call` at 0x10cf3e | `_nanosleep 0x179570` | `0x1aa4ec` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10cf55 | `_fdprintf 0x171660` | `[sp+12]` | `0x10cd4a` | `?` | `?` | `?` | `?` |
| `call` at 0x10cf62 | `_ffscMsgN 0x11ec30` | `0x10cd4a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10cf74 | `_nanosleep 0x179570` | `0x1aa4ec` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10cf8b | `_fdprintf 0x171660` | `[sp+12]` | `0x10cd4a` | `?` | `?` | `?` | `?` |
| `call` at 0x10cf98 | `_ffscMsgN 0x11ec30` | `0x10cd4a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10cfaa | `_nanosleep 0x179570` | `0x1aa4ec` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10cfc1 | `_fdprintf 0x171660` | `[sp+12]` | `0x10cd4a` | `?` | `?` | `?` | `?` |
| `call` at 0x10cfce | `_ffscMsgN 0x11ec30` | `0x10cd4a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10cfe3 | `_openFlashWindow 0x10cc14` | `[sp+4]` | `[sp-24]` | `0xff` | `?` | `?` | `?` |
| `call` at 0x10d013 | `_openFlashWindow 0x10cc14` | `[sp+4]` | `[sp-24]+32` | `0xff` | `?` | `?` | `?` |
| `call` at 0x10d05b | `_ffscMsg 0x11ebd0` | `0x10cd4c` | `[sp-12]` | `?` | `?` | `?` | `?` |
| `call` at 0x10d083 | `_fdprintf 0x171660` | `[sp+12]` | `0x10cd76` | `?` | `?` | `?` | `?` |
| `call` at 0x10d090 | `_ffscMsgN 0x11ec30` | `0x10cd76` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d0a9 | `_closeFlashWindow 0x10cc94` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10cdcf | `([ret(_openFlashWindow)]:1 & 0xff) != 0x89` | `([ret(_openFlashWindow)]:1 & 0xff) == 0x89` |
| `je` at 0x10cdd4 | `([ret(_openFlashWindow)+1]:1 & 0xff) == 0xa2` | `([ret(_openFlashWindow)+1]:1 & 0xff) != 0xa2` |
| `jne` at 0x10ce16 | `([ret(_openFlashWindow)]:1 & 0xff) != 0x89` | `([ret(_openFlashWindow)]:1 & 0xff) == 0x89` |
| `je` at 0x10ce1b | `([ret(_openFlashWindow)+1]:1 & 0xff) == 0xa2` | `([ret(_openFlashWindow)+1]:1 & 0xff) != 0xa2` |
| `jne` at 0x10ce28 | `[sp+8] != 0x2` | `[sp+8] == 0x2` |
| `jl` at 0x10ce3b | `[sp+12] < 0x0 (signed, 32-bit)` | `[sp+12] >= 0x0 (signed, 32-bit)` |
| `jl` at 0x10ce87 | `[sp+12] < 0x0 (signed, 32-bit)` | `[sp+12] >= 0x0 (signed, 32-bit)` |
| `jl` at 0x10cebd | `[sp+12] < 0x0 (signed, 32-bit)` | `[sp+12] >= 0x0 (signed, 32-bit)` |
| `jl` at 0x10cef3 | `[sp+12] < 0x0 (signed, 32-bit)` | `[sp+12] >= 0x0 (signed, 32-bit)` |
| `je` at 0x10cf17 | `[sp-16] == 0x0` | `[sp-16] != 0x0` |
| `jl` at 0x10cf4a | `[sp+12] < 0x0 (signed, 32-bit)` | `[sp+12] >= 0x0 (signed, 32-bit)` |
| `jl` at 0x10cf80 | `[sp+12] < 0x0 (signed, 32-bit)` | `[sp+12] >= 0x0 (signed, 32-bit)` |
| `jl` at 0x10cfb6 | `[sp+12] < 0x0 (signed, 32-bit)` | `[sp+12] >= 0x0 (signed, 32-bit)` |
| `jge` at 0x10cff8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x10cffe | `[sp-16] == 0x0` | `[sp-16] != 0x0` |
| `jl` at 0x10d027 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jne` at 0x10d02d | `? != 0xffffffff` | `? == 0xffffffff` |
| `je` at 0x10d031 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x10d03a | `Z == 0` | `not (Z == 0)` |
| `je` at 0x10d040 | `[sp-16] == 0x0` | `[sp-16] != 0x0` |
| `je` at 0x10d048 | `(([sp-8]:1 & 0xff) & 0x20) == 0x0` | `(([sp-8]:1 & 0xff) & 0x20) != 0x0` |
| `jl` at 0x10d078 | `[sp+12] < 0x0 (signed, 32-bit)` | `[sp+12] >= 0x0 (signed, 32-bit)` |
| `jle` at 0x10d09f | `[sp-12] <= 0xf (signed, 32-bit)` | `[sp-12] > 0xf (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10d0ba | `eax` | `[sp-20]` |

## `_closeFlash`

- `_closeFlash`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10d0c4 to 0x10d0e4.
  Established. The `_closeFlashWindow` call has candidate zero `[sp+4]` (`call` at 0x10d0cc). A
  following four-byte store writes `0x0` through `[sp+4] +0x4` (`mov` at 0x10d0d1). The exit carries
  `0x0` in `eax` (`ret` at 0x10d0e0), without an established close effect.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10d0c8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d0d1 | store | 4 | `[sp+4] +0x4` | `0x0` |
| `mov` at 0x10d0da | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10d0cc | `_closeFlashWindow 0x10cc94` | `[sp+4]` | `ebx` | `ebp` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10d0e0 | `eax` | `0x0` |

## `_flashtest`

- `_flashtest`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10d504 to 0x10d6a0.
  Established. The `_openFlash` site has leading candidates `sp-56`, `0x2`, and `0xffffffff` (`call`
  at 0x10d57c). The later `_writeFlash` (`call` at 0x10d5b3) and `_readFlash` (`call` at 0x10d60f)
  sites each present `sp-56`, unknown, and `0x80` as their first three candidates. A byte store at
  an unknown effective address writes `[sp-64]:1` (`mov` at 0x10d563). The candidate values do not
  establish transfer sizes or a test outcome.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10d50d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d52d | store | 4 | `sp -0x3c` | `ret(_malloc)` |
| `mov` at 0x10d553 | store | 4 | `sp -0x40` | `?` |
| `mov` at 0x10d55a | store | 4 | `sp -0x40` | `?` |
| `mov` at 0x10d55d | load | 1 | `sp -0x40` | `-` |
| `mov` at 0x10d560 | load | 4 | `sp -0x3c` | `-` |
| `mov` at 0x10d563 | store | 1 | `(unknown)` | `[sp-64]:1` |
| `mov` at 0x10d5a9 | load | 4 | `sp -0x3c` | `-` |
| `mov` at 0x10d5be | load | 4 | `sp -0x2c` | `-` |
| `mov` at 0x10d5ea | store | 4 | `sp -0x2c` | `0x0` |
| `mov` at 0x10d605 | load | 4 | `sp -0x3c` | `-` |
| `mov` at 0x10d61a | load | 4 | `sp -0x2c` | `-` |
| `mov` at 0x10d640 | load | 4 | `sp -0x3c` | `-` |
| `mov` at 0x10d643 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10d649 | store | 4 | `sp -0x44` | `?` |
| `mov` at 0x10d666 | store | 4 | `sp -0x40` | `?` |
| `cmp` at 0x10d66d | load | 4 | `sp -0x44` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10d51c | `_ffscMsg 0x11ebd0` | `0x10d484` | `[sp+4]` | `?` | `ebx` | `esi` | `edi` |
| `call` at 0x10d525 | `_malloc 0x17c878` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d56f | `_initFlash 0x10cba4` | `sp-56` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d57c | `_openFlash 0x10cd84` | `sp-56` | `0x2` | `0xffffffff` | `?` | `?` | `?` |
| `call` at 0x10d58c | `_ffscMsg 0x11ebd0` | `0x10d49e` | `ret(_openFlash)` | `?` | `?` | `?` | `?` |
| `call` at 0x10d5b3 | `_writeFlash 0x10d314` | `sp-56` | `?` | `0x80` | `?` | `?` | `?` |
| `call` at 0x10d5c8 | `_ffscMsg 0x11ebd0` | `0x10d4b4` | `?` | `[sp-44]` | `ret(_writeFlash)` | `?` | `?` |
| `call` at 0x10d60f | `_readFlash 0x10d0e4` | `sp-56` | `?` | `0x80` | `?` | `?` | `?` |
| `call` at 0x10d624 | `_ffscMsg 0x11ebd0` | `0x10d4b4` | `?` | `[sp-44]` | `ret(_readFlash)` | `?` | `?` |
| `call` at 0x10d678 | `_ffscMsg 0x11ebd0` | `0x10d4d7` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d68e | `_ffscMsg 0x11ebd0` | `0x10d4e8` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x10d534 | `0x0 >= ? (signed, 32-bit)` | `0x0 < ? (signed, 32-bit)` |
| `jl` at 0x10d569 | `? < ? (signed, 32-bit)` | `? >= ? (signed, 32-bit)` |
| `jl` at 0x10d596 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jge` at 0x10d5a0 | `0x0 >= ? (signed, 32-bit)` | `0x0 < ? (signed, 32-bit)` |
| `jl` at 0x10d5d2 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jl` at 0x10d5e0 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jl` at 0x10d5e4 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jge` at 0x10d5f5 | `0x0 >= ? (signed, 32-bit)` | `0x0 < ? (signed, 32-bit)` |
| `jl` at 0x10d5fa | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jl` at 0x10d634 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jl` at 0x10d638 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jge` at 0x10d63e | `0x0 >= ? (signed, 32-bit)` | `0x0 < ? (signed, 32-bit)` |
| `je` at 0x10d670 | `[sp-68] == ?` | `[sp-68] != ?` |
| `jl` at 0x10d687 | `? < ? (signed, 32-bit)` | `? >= ? (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10d69c | `eax` | `ret(_ffscMsg)` |

## `_cmdFLASH`

- `_cmdFLASH`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1170e0 to 0x117740.
  Established. The initial signed branch is taken when `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` (`jg`
  at 0x117101). A later `_userSendCommand` site has leading candidates `ret(_userAcquireRemote)` and
  `sp-252` (`call` at 0x1172f2); its result branch is taken when `(ret(_userSendCommand) &
  ret(_userSendCommand)) == 0x0` (`je` at 0x1172fc), reaching a four-byte store of `0xffffffff`
  through `[[sp+8]+132] +0x14` (`mov` at 0x117316). Separate four-byte stores write `0x1` to
  absolute `0x1eebc0` (`mov` at 0x117414, `mov` at 0x117473).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1170ec | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1170ef | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1170f2 | store | 4 | `sp -0x8` | `0xffffffff` |
| `mov` at 0x1170f9 | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x117108 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x11710d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117114 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117134 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x117252 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x11725e | load | 4 | `sp -0x8` | `-` |
| `cmp` at 0x117261 | load | 4 | `abs 0x1ee564` | `-` |
| `test` at 0x1172a0 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x1172a9 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x1172af | load | 4 | `[[sp+8]+132] +0x14` | `-` |
| `mov` at 0x1172c2 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x1172c8 | load | 4 | `[[sp+8]+132] +0x14` | `-` |
| `mov` at 0x1172e0 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1172e5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1172ec | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117310 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x117316 | store | 4 | `[[sp+8]+132] +0x14` | `0xffffffff` |
| `mov` at 0x11731d | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x117322 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117329 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11732c | store | 1 | `(unknown)` | `0x0` |
| `mov` at 0x117340 | load | 4 | `sp -0x8` | `-` |
| `cmp` at 0x117343 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x11734f | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x117370 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x117389 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x11738c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117393 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117399 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1173a2 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1173a7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1173ae | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1173c6 | load | 4 | `[sp+8] +0x94` | `-` |
| `mov` at 0x1173cc | load | 4 | `[[sp+8]+148] +0x50` | `-` |
| `mov` at 0x1173d8 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1173dd | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1173fd | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x117402 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117409 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117414 | store | 4 | `abs 0x1eebc0` | `0x1` |
| `mov` at 0x117426 | load | 4 | `[sp+8] +0x94` | `-` |
| `mov` at 0x11742c | load | 4 | `[[sp+8]+148] +0x50` | `-` |
| `mov` at 0x117430 | load | 4 | `[[sp+8]+148] +0x4c` | `-` |
| `mov` at 0x11743b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117442 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117452 | load | 4 | `[sp+8] +0x94` | `-` |
| `mov` at 0x117458 | load | 4 | `[[sp+8]+148] +0x50` | `-` |
| `mov` at 0x117463 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11746a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117473 | store | 4 | `abs 0x1eebc0` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x117118 | `_sprintf 0x171680` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x117138 | `_destNumRacks 0x11ef3c` | `[sp+12]` | `sp-8` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x11714b | `_ffscMsg 0x11ebd0` | `0x116f31` | `ret(_destNumRacks)` | `?` | `?` | `?` | `?` |
| `call` at 0x117168 | `_strtok_r 0x17e358` | `0x0` | `0x116c80` | `[sp+4]` | `?` | `?` | `?` |
| `call` at 0x117181 | `_ffscConvertToUpperCase 0x12543c` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11718f | `_strcmp 0x17de80` | `ret(_strtok_r)` | `0x116f58` | `?` | `?` | `?` | `?` |
| `call` at 0x1171a1 | `_ffscMsg 0x11ebd0` | `0x116f64` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x1171c8 | `_strtok_r 0x17e358` | `0x0` | `0x116c80` | `[sp+4]` | `?` | `?` | `?` |
| `call` at 0x1171db | `_ffscMsg 0x11ebd0` | `0x116f82` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1171f1 | `_ffscConvertToUpperCase 0x12543c` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1171ff | `_strcmp 0x17de80` | `?` | `0x116f5d` | `?` | `?` | `?` | `?` |
| `call` at 0x117226 | `_strcmp 0x17de80` | `?` | `0x116fa3` | `?` | `?` | `?` | `?` |
| `call` at 0x117238 | `_ffscMsg 0x11ebd0` | `0x116fab` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11726e | `_ffscMsg 0x11ebd0` | `0x116fd7` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x117281 | `_userAcquireRemote 0x12a978` | `[sp+8]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1172b8 | `_ffscMsg 0x11ebd0` | `0x11700b` | `[[[sp+8]+132]+20]` | `?` | `?` | `?` | `?` |
| `call` at 0x1172d8 | `_sprintf 0x171680` | `sp-252` | `0x117037` | `[[[sp+8]+132]+20]` | `0x6` | `?` | `?` |
| `call` at 0x1172f2 | `_userSendCommand 0x12b7d8` | `ret(_userAcquireRemote)` | `sp-252` | `?` | `?` | `?` | `?` |
| `call` at 0x1172ff | `_userReleaseRemote 0x12b738` | `ret(_userAcquireRemote)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11735c | `_ffscMsg 0x11ebd0` | `0x117046` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11737b | `_remStartConnectCommand 0x128568` | `[sp+8]` | `0x11708d` | `[sp-8]` | `[sp+12]` | `?` | `?` |
| `call` at 0x1173b2 | `_sprintf 0x171680` | `?` | `0x117093` | `?` | `?` | `?` | `?` |
| `call` at 0x11740d | `_sprintf 0x171680` | `?` | `0x1170c0` | `0x10c0` | `ret(_remStartConnectCommand)` | `0x4` | `?` |
| `call` at 0x117446 | `_sprintf 0x171680` | `?` | `0x1170a4` | `?` | `?` | `[[[sp+8]+148]+80]` | `?` |
| `call` at 0x11746e | `_sprintf 0x171680` | `?` | `0x1170c0` | `0x40` | `[[[sp+8]+148]+80]` | `0x6` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x117101 | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `je` at 0x117143 | `ret(_destNumRacks) == 0x1` | `ret(_destNumRacks) != 0x1` |
| `jne` at 0x117174 | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `je` at 0x117199 | `(ret(_strcmp) & ret(_strcmp)) == 0x0` | `(ret(_strcmp) & ret(_strcmp)) != 0x0` |
| `jne` at 0x1171d4 | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `jne` at 0x117209 | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |
| `je` at 0x117230 | `(ret(_strcmp) & ret(_strcmp)) == 0x0` | `(ret(_strcmp) & ret(_strcmp)) != 0x0` |
| `jne` at 0x117258 | `[[sp+8]+16] != 0x6` | `[[sp+8]+16] == 0x6` |
| `je` at 0x117267 | `[0x1ee564] == [sp-8]` | `[0x1ee564] != [sp-8]` |
| `jne` at 0x11728d | `(ret(_userAcquireRemote) & ret(_userAcquireRemote)) != 0x0` | `(ret(_userAcquireRemote) & ret(_userAcquireRemote)) == 0x0` |
| `je` at 0x1172a7 | `(([0x1da698]:1 & 0xff) & 0x20) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x20) != 0x0` |
| `je` at 0x1172fc | `(ret(_userSendCommand) & ret(_userSendCommand)) == 0x0` | `(ret(_userSendCommand) & ret(_userSendCommand)) != 0x0` |
| `je` at 0x117349 | `[0x1ee564] == [sp-8]` | `[0x1ee564] != [sp-8]` |
| `jne` at 0x117355 | `[[sp+8]+16] != 0x4` | `[[sp+8]+16] == 0x4` |
| `jge` at 0x117387 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x1173c2 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x117422 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x11748b | `eax` | `0x0` |

## `_ceEnterFlashMode`

- `_ceEnterFlashMode`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x118cc8 to 0x118da8.
  Established. The `_bufClear` sites present candidate zero `[[sp+4]+112]` (`call` at 0x118cea) and
  `[[sp+4]+116]` (`call` at 0x118cf6). The four-byte store through `[sp+4] +0x18` writes `0x6`
  (`mov` at 0x118cfe). Later four-byte stores through `[sp+4] +0x3c` retain unknown values (`mov` at
  0x118d27, `mov` at 0x118d38). No mode or buffer-clearing effect is inferred from these call names.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x118ccc | load | 4 | `sp +0x4` | `-` |
| `test` at 0x118ccf | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x118ce6 | load | 4 | `[sp+4] +0x70` | `-` |
| `mov` at 0x118cf2 | load | 4 | `[sp+4] +0x74` | `-` |
| `mov` at 0x118cfe | store | 4 | `[sp+4] +0x18` | `0x6` |
| `mov` at 0x118d05 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x118d0d | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x118d18 | load | 4 | `[sp+4] +0x60` | `-` |
| `mov` at 0x118d21 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x118d27 | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x118d2a | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x118d32 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x118d38 | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x118d3d | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x118cde | `_ffscMsg 0x11ebd0` | `0x118ca5` | `[sp+4]` | `ebx` | `ebp` | `?` | `?` |
| `call` at 0x118cea | `_bufClear 0x10f438` | `[[sp+4]+112]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x118cf6 | `_bufClear 0x10f438` | `[[sp+4]+116]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x118d1c | `_fdprintf 0x171660` | `[[sp+4]+96]` | `0x11837d` | `[0x1ee564]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x118cd6 | `(([0x1da698]:1 & 0xff) & 0x4) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x4) != 0x0` |
| `je` at 0x118d0b | `(? & 0x40) == 0x0` | `(? & 0x40) != 0x0` |
| `jge` at 0x118d30 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x118d43 | `eax` | `0x0` |

## `_clLeaveFlashMode`

- `_clLeaveFlashMode`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1198e8 to 0x119978.
  Established. Four-byte stores write `0x0` through `[sp+4] +0x18` (`mov` at 0x1198ef), `+0x1c`
  (`mov` at 0x1198f6), `+0x20` (`mov` at 0x1198fd), `+0x24` (`mov` at 0x119904), and `+0x28` (`mov`
  at 0x11990b). The branch falls through when `[[sp+4]+12] == 0x4` (`jne` at 0x119920), reaching
  `_close` with candidate zero `[[sp+4]+96]` (`call` at 0x119926). A common four-byte store writes
  `0xffffffff` through `[sp+4] +0x60` (`mov` at 0x11992e). Field roles remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1198ec | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1198ef | store | 4 | `[sp+4] +0x18` | `0x0` |
| `mov` at 0x1198f6 | store | 4 | `[sp+4] +0x1c` | `0x0` |
| `mov` at 0x1198fd | store | 4 | `[sp+4] +0x20` | `0x0` |
| `mov` at 0x119904 | store | 4 | `[sp+4] +0x24` | `0x0` |
| `mov` at 0x11990b | store | 4 | `[sp+4] +0x28` | `0x0` |
| `mov` at 0x119912 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x119917 | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x11991a | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x119922 | load | 4 | `[sp+4] +0x60` | `-` |
| `mov` at 0x11992e | store | 4 | `[sp+4] +0x60` | `0xffffffff` |
| `test` at 0x119935 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x11994b | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x119926 | `_close 0x186ed0` | `[[sp+4]+96]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x119944 | `_ffscMsg 0x11ebd0` | `0x1198bd` | `[sp+4]` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x119920 | `[[sp+4]+12] != 0x4` | `[[sp+4]+12] == 0x4` |
| `je` at 0x11993c | `(([0x1da698]:1 & 0xff) & 0x4) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x119951 | `eax` | `0x0` |

## `_getflash`

- `_getflash`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x121a90 to 0x121fd8.
  Established. Byte stores at `sp -0x40d` include `0x43` (`mov` at 0x121b10), `0x6` (`mov` at
  0x121c10, `mov` at 0x121ebd), and `0x15` (`mov` at 0x121f50). The first `_timeoutRead` site has
  leading candidates `[sp+8]`, `sp-1036`, `0x405`, and `0x989680` (`call` at 0x121b55). The
  `_writeFlash` site presents `[sp+4]`, `sp-1033`, and `[sp-1052]` as its leading candidates (`call`
  at 0x121f02). Neither protocol-token meanings nor timing units are established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x121a9c | store | 4 | `sp -0x418` | `0x0` |
| `mov` at 0x121aa6 | store | 4 | `sp -0x420` | `0x0` |
| `mov` at 0x121ab0 | store | 4 | `sp -0x424` | `0xffffffff` |
| `mov` at 0x121abe | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x121ad7 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x121ae2 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x121aee | store | 4 | `sp -0x414` | `0x0` |
| `mov` at 0x121af8 | load | 4 | `sp -0x414` | `-` |
| `cmp` at 0x121afe | load | 4 | `abs 0x1da178` | `-` |
| `mov` at 0x121b10 | store | 1 | `sp -0x40d` | `0x43` |
| `mov` at 0x121b20 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x121b51 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x121b85 | load | 4 | `ret(___errno) +0x0` | `-` |
| `inc` at 0x121b90 | load | 4 | `sp -0x414` | `-` |
| `inc` at 0x121b90 | store | 4 | `sp -0x414` | `?` |
| `mov` at 0x121ba5 | store | 4 | `sp -0x428` | `sp-1036` |
| `test` at 0x121bac | load | 1 | `abs 0x1da699` | `-` |
| `cmp` at 0x121bc3 | load | 1 | `sp -0x40c` | `-` |
| `mov` at 0x121bcc | store | 4 | `sp -0x41c` | `0x400` |
| `mov` at 0x121be0 | store | 4 | `sp -0x41c` | `0x80` |
| `movzx` at 0x121bf7 | load | 1 | `sp -0x40c` | `-` |
| `mov` at 0x121c10 | store | 1 | `sp -0x40d` | `0x6` |
| `mov` at 0x121c20 | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x121c4d | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x121c5a | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x121c63 | store | 4 | `sp -0x424` | `0x0` |
| `mov` at 0x121c90 | store | 4 | `sp -0x414` | `0x0` |
| `test` at 0x121c9e | load | 1 | `abs 0x1da699` | `-` |
| `add` at 0x121cb7 | load | 4 | `sp -0x428` | `-` |
| `mov` at 0x121cbd | store | 4 | `sp -0x42c` | `?` |
| `mov` at 0x121cc4 | load | 4 | `sp -0x414` | `-` |
| `cmp` at 0x121cca | load | 4 | `abs 0x1da178` | `-` |
| `mov` at 0x121ce6 | load | 4 | `sp -0x42c` | `-` |
| `mov` at 0x121ced | load | 4 | `sp +0x8` | `-` |
| `test` at 0x121cfb | load | 1 | `abs 0x1da699` | `-` |
| `mov` at 0x121d29 | load | 4 | `ret(___errno) +0x0` | `-` |
| `inc` at 0x121d34 | load | 4 | `sp -0x414` | `-` |
| `inc` at 0x121d34 | store | 4 | `sp -0x414` | `?` |
| `add` at 0x121d42 | load | 4 | `sp -0x42c` | `-` |
| `add` at 0x121d42 | store | 4 | `sp -0x42c` | `([sp-1068] + ret(_timeoutRead))` |
| `mov` at 0x121d50 | load | 1 | `sp -0x40c` | `-` |
| `movzx` at 0x121d5c | load | 1 | `sp -0x40c` | `-` |
| `inc` at 0x121d71 | load | 4 | `sp -0x420` | `-` |
| `inc` at 0x121d71 | store | 4 | `sp -0x420` | `?` |
| `movzx` at 0x121d80 | load | 1 | `sp -0x40b` | `-` |
| `movzx` at 0x121d87 | load | 1 | `sp -0x40a` | `-` |
| `inc` at 0x121da7 | load | 4 | `sp -0x420` | `-` |
| `inc` at 0x121da7 | store | 4 | `sp -0x420` | `?` |
| `mov` at 0x121dc0 | load | 4 | `sp -0x418` | `-` |
| `inc` at 0x121ddd | load | 4 | `sp -0x420` | `-` |
| `inc` at 0x121ddd | store | 4 | `sp -0x420` | `?` |
| `cmp` at 0x121df4 | load | 4 | `sp -0x41c` | `-` |
| `mov` at 0x121e01 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x121e12 | load | 2 | `(unknown)` | `-` |
| `cmp` at 0x121e28 | load | 4 | `sp -0x41c` | `-` |
| `cmp` at 0x121e35 | load | 1 | `sp -0x40c` | `-` |
| `cmp` at 0x121e3e | load | 1 | `sp -0x9` | `-` |
| `cmp` at 0x121e43 | load | 1 | `sp -0x8` | `-` |
| `movzx` at 0x121e50 | load | 1 | `sp -0x8` | `-` |
| `movzx` at 0x121e55 | load | 1 | `sp -0x9` | `-` |
| `mov` at 0x121e60 | load | 1 | `sp -0x389` | `-` |
| `mov` at 0x121e6a | load | 1 | `sp -0x388` | `-` |
| `mov` at 0x121e7c | load | 1 | `sp -0x388` | `-` |
| `mov` at 0x121e88 | load | 1 | `sp -0x389` | `-` |
| `inc` at 0x121ea1 | load | 4 | `sp -0x420` | `-` |
| `inc` at 0x121ea1 | store | 4 | `sp -0x420` | `?` |
| `movzx` at 0x121eb0 | load | 1 | `sp -0x40b` | `-` |
| `mov` at 0x121eb7 | store | 4 | `sp -0x418` | `[sp-1035]:1` |
| `mov` at 0x121ebd | store | 1 | `sp -0x40d` | `0x6` |
| `mov` at 0x121ecd | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x121ef0 | load | 4 | `sp -0x41c` | `-` |
| `mov` at 0x121efe | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x121f0a | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x121f17 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x121f30 | load | 4 | `sp -0x420` | `-` |
| `cmp` at 0x121f36 | load | 4 | `abs 0x1da174` | `-` |
| `mov` at 0x121f50 | store | 1 | `sp -0x40d` | `0x15` |
| `mov` at 0x121f60 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x121f8a | load | 4 | `sp -0x428` | `-` |
| `mov` at 0x121f91 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x121fad | load | 4 | `ret(___errno) +0x0` | `-` |
| `mov` at 0x121fc2 | load | 4 | `sp -0x424` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x121ac2 | `_ioctl 0x186f90` | `[sp+8]` | `0x1a` | `0x0` | `ebx` | `esi` | `edi` |
| `call` at 0x121acf | `_ffscMsg 0x11ebd0` | `0x121870` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x121ae6 | `_fdprintf 0x171660` | `[sp+12]` | `0x12188e` | `?` | `?` | `?` | `?` |
| `call` at 0x121b24 | `_write 0x186f70` | `[sp+8]` | `sp-1037` | `0x1` | `?` | `?` | `?` |
| `call` at 0x121b55 | `_timeoutRead 0x12a538` | `[sp+8]` | `sp-1036` | `0x405` | `0x989680` | `?` | `?` |
| `call` at 0x121b80 | `___errno 0x16fa78` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x121bbb | `_ffscMsg 0x11ebd0` | `0x121919` | `ret(_timeoutRead)` | `?` | `?` | `?` | `?` |
| `call` at 0x121c24 | `_write 0x186f70` | `[sp+8]` | `sp-1037` | `0x1` | `?` | `?` | `?` |
| `call` at 0x121c45 | `_ffscMsg 0x11ebd0` | `0x121931` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x121c5e | `_write 0x186f70` | `[sp+12]` | `0x12194b` | `0x4` | `?` | `?` | `?` |
| `call` at 0x121caf | `_ffscMsg 0x11ebd0` | `0x121963` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x121cf1 | `_timeoutRead 0x12a538` | `[sp+8]` | `?` | `?` | `0xf4240` | `?` | `?` |
| `call` at 0x121d0a | `_ffscMsg 0x11ebd0` | `0x1219b6` | `ret(_timeoutRead)` | `?` | `?` | `?` | `?` |
| `call` at 0x121d24 | `___errno 0x16fa78` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x121d69 | `_ffscMsg 0x11ebd0` | `0x121a01` | `[sp-1036]:1` | `?` | `?` | `?` | `?` |
| `call` at 0x121d9f | `_ffscMsg 0x11ebd0` | `0x121a16` | `[sp-1035]:1` | `[sp-1034]:1` | `?` | `?` | `?` |
| `call` at 0x121dd5 | `_ffscMsg 0x11ebd0` | `0x121a38` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x121e99 | `_ffscMsg 0x11ebd0` | `0x121a4a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x121ed1 | `_write 0x186f70` | `[sp+8]` | `sp-1037` | `0x1` | `?` | `?` | `?` |
| `call` at 0x121f02 | `_writeFlash 0x10d314` | `[sp+4]` | `sp-1033` | `[sp-1052]` | `?` | `?` | `?` |
| `call` at 0x121f1b | `_write 0x186f70` | `[sp+12]` | `0x121a74` | `0x1` | `?` | `?` | `?` |
| `call` at 0x121f64 | `_write 0x186f70` | `[sp+8]` | `sp-1037` | `0x1` | `?` | `?` | `?` |
| `call` at 0x121f95 | `_timeoutRead 0x12a538` | `[sp+8]` | `[sp-1064]` | `0x405` | `0x989680` | `?` | `?` |
| `call` at 0x121fa8 | `___errno 0x16fa78` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x121fbd | `_ffscMsg 0x11ebd0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jl` at 0x121adb | `[sp+12] < 0x0 (signed, 32-bit)` | `[sp+12] >= 0x0 (signed, 32-bit)` |
| `jne` at 0x121b04 | `Z == 0` | `not (Z == 0)` |
| `je` at 0x121b2f | `ret(_write) == 0x1` | `ret(_write) != 0x1` |
| `je` at 0x121b62 | `ret(_timeoutRead) == 0xffffffff` | `ret(_timeoutRead) != 0xffffffff` |
| `jne` at 0x121b66 | `(ret(_timeoutRead) & ret(_timeoutRead)) != 0x0` | `(ret(_timeoutRead) & ret(_timeoutRead)) == 0x0` |
| `jne` at 0x121b8a | `[ret(___errno)] != 0x4` | `[ret(___errno)] == 0x4` |
| `je` at 0x121b99 | `ret(_timeoutRead) == 0xffffffff` | `ret(_timeoutRead) != 0xffffffff` |
| `je` at 0x121bb3 | `(([0x1da699]:1 & 0xff) & 0x4) == 0x0` | `(([0x1da699]:1 & 0xff) & 0x4) != 0x0` |
| `jne` at 0x121bca | `([sp-1036]:1 & 0xff) != 0x2` | `([sp-1036]:1 & 0xff) == 0x2` |
| `je` at 0x121bf1 | `ret(_timeoutRead) == ?` | `ret(_timeoutRead) != ?` |
| `je` at 0x121c01 | `[sp-1036]:1 == 0x4` | `[sp-1036]:1 != 0x4` |
| `je` at 0x121c06 | `[sp-1036]:1 == 0x18` | `[sp-1036]:1 != 0x18` |
| `je` at 0x121c2f | `ret(_write) == 0x1` | `ret(_write) != 0x1` |
| `jl` at 0x121c51 | `[sp+12] < 0x0 (signed, 32-bit)` | `[sp+12] >= 0x0 (signed, 32-bit)` |
| `je` at 0x121ca5 | `(([0x1da699]:1 & 0xff) & 0x4) == 0x0` | `(([0x1da699]:1 & 0xff) & 0x4) != 0x0` |
| `jne` at 0x121cd0 | `Z == 0` | `not (Z == 0)` |
| `je` at 0x121d02 | `(([0x1da699]:1 & 0xff) & 0x4) == 0x0` | `(([0x1da699]:1 & 0xff) & 0x4) != 0x0` |
| `jne` at 0x121d14 | `(ret(_timeoutRead) & ret(_timeoutRead)) != 0x0` | `(ret(_timeoutRead) & ret(_timeoutRead)) == 0x0` |
| `jge` at 0x121d22 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x121d2e | `[ret(___errno)] != 0x4` | `[ret(___errno)] == 0x4` |
| `jg` at 0x121d4a | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `jbe` at 0x121d5a | `? <= 0x1 (unsigned, 8-bit)` | `? > 0x1 (unsigned, 8-bit)` |
| `je` at 0x121d96 | `([sp-1034]:1 + [sp-1035]:1) == 0xff` | `([sp-1034]:1 + [sp-1035]:1) != 0xff` |
| `je` at 0x121dce | `[sp-1035]:1 == ?` | `[sp-1035]:1 != ?` |
| `jle` at 0x121dfa | `[sp-1052] <= 0x0 (signed, 32-bit)` | `[sp-1052] > 0x0 (signed, 32-bit)` |
| `jg` at 0x121e2e | `[sp-1052] > ? (signed, 32-bit)` | `[sp-1052] <= ? (signed, 32-bit)` |
| `jne` at 0x121e3c | `([sp-1036]:1 & 0xff) != 0x2` | `([sp-1036]:1 & 0xff) == 0x2` |
| `jne` at 0x121e41 | `Z == 0` | `not (Z == 0)` |
| `je` at 0x121e46 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x121e68 | `Z == 0` | `not (Z == 0)` |
| `je` at 0x121e72 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x121edc | `ret(_write) == 0x1` | `ret(_write) != 0x1` |
| `jl` at 0x121f0e | `[sp+12] < 0x0 (signed, 32-bit)` | `[sp+12] >= 0x0 (signed, 32-bit)` |
| `jne` at 0x121f3c | `[0x1da174] != [sp-1056]` | `[0x1da174] == [sp-1056]` |
| `je` at 0x121f6f | `ret(_write) == 0x1` | `ret(_write) != 0x1` |
| `jne` at 0x121fa2 | `ret(_timeoutRead) != 0xffffffff` | `ret(_timeoutRead) == 0xffffffff` |
| `je` at 0x121fb2 | `[ret(___errno)] == 0x4` | `[ret(___errno)] != 0x4` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x121fd4 | `eax` | `[sp-1060]` |

## `_diffbuf`

- `_diffbuf`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x121660 to 0x1217c0.
  Established. The `_malloc` site has candidate zero `0x8000` (`call` at 0x12166e). The `_readFlash`
  site presents leading candidates `sp-56`, `ret(_malloc)`, and `0x8000` (`call` at 0x1216ff). The
  `_memcmp` result branch falls through when `(ret(_memcmp) & ret(_memcmp)) != 0x0` (`je` at
  0x12172d), reaching `_taskSuspend` with candidate zero `0x0` (`call` at 0x12173f). The comparison
  extent and callee effects remain unassigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x1216f2 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12170b | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12171c | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x121763 | load | 4 | `sp +0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12166e | `_malloc 0x17c878` | `0x8000` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x121681 | `_ffscMsg 0x11ebd0` | `0x1215f0` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x121694 | `_initFlash 0x10cba4` | `sp-56` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1216a1 | `_openFlash 0x10cd84` | `sp-56` | `0x1` | `0xffffffff` | `?` | `?` | `?` |
| `call` at 0x1216b2 | `_ffscMsg 0x11ebd0` | `0x1215ff` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1216c5 | `_ffscMsg 0x11ebd0` | `0x121610` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1216ce | `_free 0x17c898` | `ret(_malloc)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1216d7 | `_closeFlash 0x10d0c4` | `sp-56` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1216ff | `_readFlash 0x10d0e4` | `sp-56` | `ret(_malloc)` | `0x8000` | `?` | `?` | `?` |
| `call` at 0x121723 | `_memcmp 0x17dd58` | `ret(_malloc)` | `?` | `0x8000` | `?` | `?` | `?` |
| `call` at 0x121735 | `_ffscMsg 0x11ebd0` | `0x121621` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12173f | `_taskSuspend 0x170560` | `0x0` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x121755 | `_ffscMsg 0x11ebd0` | `0x121648` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12176c | `_closeFlash 0x10d0c4` | `sp-56` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x121775 | `_free 0x17c898` | `ret(_malloc)` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x12167a | `(ret(_malloc) & ret(_malloc)) != 0x0` | `(ret(_malloc) & ret(_malloc)) == 0x0` |
| `je` at 0x1216ab | `(ret(_openFlash) & ret(_openFlash)) == 0x0` | `(ret(_openFlash) & ret(_openFlash)) != 0x0` |
| `jle` at 0x1216f5 | `[sp+8] <= 0x0 (signed, 32-bit)` | `[sp+8] > 0x0 (signed, 32-bit)` |
| `jne` at 0x121709 | `(ret(_readFlash) & ret(_readFlash)) != 0x0` | `(ret(_readFlash) & ret(_readFlash)) == 0x0` |
| `jle` at 0x121715 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `je` at 0x12172d | `(ret(_memcmp) & ret(_memcmp)) == 0x0` | `(ret(_memcmp) & ret(_memcmp)) != 0x0` |
| `jg` at 0x121766 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x121783 | `eax` | `?` |

## `_writebuf`

- `_writebuf`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1217c0 to 0x121a90.
  Established. The `_openFlash` site presents leading candidates `sp-56`, `0x2`, and `0xffffffff`
  (`call` at 0x1217da). Taking its result branch when `(ret(_openFlash) & ret(_openFlash)) == 0x0`
  (`je` at 0x1217e4) reaches the signed loop guard (`jle` at 0x121815). The `_writeFlash` site
  presents candidate zero `sp-56` with unknown candidates one and two (`call` at 0x121837); its
  result branch is taken when `(ret(_writeFlash) & ret(_writeFlash)) != 0x0` (`jne` at 0x121841),
  reaching `_ffscMsg` with candidate zero `0x121784` (`call` at 0x121805).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x121812 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12181c | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x121830 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x121858 | load | 4 | `sp +0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1217cd | `_initFlash 0x10cba4` | `sp-56` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x1217da | `_openFlash 0x10cd84` | `sp-56` | `0x2` | `0xffffffff` | `?` | `?` | `?` |
| `call` at 0x1217eb | `_ffscMsg 0x11ebd0` | `0x1215ff` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x121805 | `_ffscMsg 0x11ebd0` | `0x121784` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x121837 | `_writeFlash 0x10d314` | `sp-56` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12184a | `_ffscMsg 0x11ebd0` | `0x121796` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x121861 | `_closeFlash 0x10d0c4` | `sp-56` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1217e4 | `(ret(_openFlash) & ret(_openFlash)) == 0x0` | `(ret(_openFlash) & ret(_openFlash)) != 0x0` |
| `jle` at 0x121815 | `[sp+8] <= 0x0 (signed, 32-bit)` | `[sp+8] > 0x0 (signed, 32-bit)` |
| `jg` at 0x12182b | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `jne` at 0x121841 | `(ret(_writeFlash) & ret(_writeFlash)) != 0x0` | `(ret(_writeFlash) & ret(_writeFlash)) == 0x0` |
| `jg` at 0x12185b | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12186f | `eax` | `?` |

## `_cmdRESET_NVRAM`

- `_cmdRESET_NVRAM`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x118060 to 0x118100.
  Established. The signed branch is taken when `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` (`jg` at
  0x118071). The `_nvramReset` site presents candidate zero `ebp` with unknown remaining slots
  (`call` at 0x1180ad). A four-byte store at an unknown effective address writes `0xb` (`mov` at
  0x1180db). The separate exits report `0x0` (`ret` at 0x118092) and `0x1` (`ret` at 0x1180ea). No
  reset outcome is inferred.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x118063 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x118066 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x118069 | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x118078 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x11807d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x118084 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1180a0 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1180a5 | load | 1 | `([0x1ee564] + [sp+12]) +0x4` | `-` |
| `mov` at 0x1180b4 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1180bf | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1180c6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1180cf | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1180d4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1180db | store | 4 | `(unknown)` | `0xb` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x118088 | `_sprintf 0x171680` | `?` | `0x116c70` | `ebp` | `?` | `?` | `?` |
| `call` at 0x1180ad | `_nvramReset 0x126d90` | `ebp` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1180ca | `_sprintf 0x171680` | `?` | `0x11804a` | `[0x1ee564]` | `0x2` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x118071 | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `je` at 0x1180ab | `(([([0x1ee564] + [sp+12])+4]:1 & 0xff) & ([([0x1ee564] + [sp+12])+4]:1 & 0xff)) == 0x0` | `(([([0x1ee564] + [sp+12])+4]:1 & 0xff) & ([([0x1ee564] + [sp+12])+4]:1 & 0xff)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x118092 | `eax` | `0x0` |
| `ret` at 0x1180ea | `eax` | `0x1` |

## `_ffscEarlyInit`

- `_ffscEarlyInit`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x123504 to 0x123b14.
  Established. The first `_nvramInit` result branch falls through when `(ret(_nvramInit) &
  ret(_nvramInit)) != 0x0` (`je` at 0x1235b4), reaching `_nvramReset` with unknown candidates
  (`call` at 0x1235cb). The `_nvramRead` (`call` at 0x123659) and `_nvramWrite` (`call` at 0x12369d)
  sites present leading pairs `0x8`, `0x1da110`. A four-byte store places `ret(_open)` at absolute
  `0x1d9a60` (`mov` at 0x12355c). The common exit reports `0x0` (`ret` at 0x12379f), without
  establishing an initialization outcome.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12355c | store | 4 | `abs 0x1d9a60` | `ret(_open)` |
| `cmp` at 0x123574 | load | 4 | `abs 0x1d9a60` | `-` |
| `mov` at 0x12357d | load | 4 | `abs 0x1d9a60` | `-` |
| `mov` at 0x12358d | load | 4 | `abs 0x1d9a60` | `-` |
| `mov` at 0x12359d | load | 4 | `abs 0x1d9a60` | `-` |
| `mov` at 0x1235c0 | load | 4 | `ret(___stderr) +0x0` | `-` |
| `test` at 0x1235e4 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x1235f7 | load | 4 | `ret(___stderr) +0x0` | `-` |
| `cmp` at 0x123602 | load | 4 | `abs 0x122fd4` | `-` |
| `mov` at 0x12361c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123623 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123629 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x12362f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123633 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x123637 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12363b | store | 4 | `(unknown)` | `?` |
| `mov` at 0x12363f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123643 | store | 4 | `(unknown)` | `?` |
| `cmp` at 0x12364d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123685 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123689 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x1236bc | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1236cc | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1236d0 | store | 4 | `(unknown)` | `?` |
| `test` at 0x1236f4 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x123707 | load | 4 | `ret(___stderr) +0x0` | `-` |
| `mov` at 0x123725 | load | 4 | `ret(___stderr) +0x0` | `-` |
| `mov` at 0x123743 | load | 4 | `ret(___stderr) +0x0` | `-` |
| `cmp` at 0x12374e | load | 4 | `abs 0x1ebbfc` | `-` |
| `mov` at 0x123757 | load | 4 | `abs 0x1ebbfc` | `-` |
| `mov` at 0x123767 | load | 4 | `abs 0x1ebbfc` | `-` |
| `mov` at 0x123777 | load | 4 | `abs 0x1ebbfc` | `-` |
| `mov` at 0x123789 | load | 4 | `abs 0x1ebbfc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x123516 | `_iosInit 0x16ef40` | `0x14` | `0x32` | `0x12340a` | `ebx` | `esi` | `edi` |
| `call` at 0x12351e | `_pcConDrv 0x108858` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12352f | `_sprintf 0x171680` | `sp-24` | `0x123410` | `?` | `?` | `?` | `?` |
| `call` at 0x123543 | `_pcConDevCreate 0x1088b8` | `sp-24` | `?` | `0x200` | `0x200` | `?` | `?` |
| `call` at 0x123554 | `_open 0x186d30` | `sp-24` | `0x2` | `0x0` | `?` | `?` | `?` |
| `call` at 0x123566 | `_ioctl 0x186f90` | `ret(_open)` | `0x3` | `0x7f` | `?` | `?` | `?` |
| `call` at 0x123585 | `_ioGlobalStdSet 0x187450` | `0x0` | `[0x1d9a60]` | `?` | `?` | `?` | `?` |
| `call` at 0x123595 | `_ioGlobalStdSet 0x187450` | `0x1` | `[0x1d9a60]` | `?` | `?` | `?` | `?` |
| `call` at 0x1235a5 | `_ioGlobalStdSet 0x187450` | `0x2` | `[0x1d9a60]` | `?` | `?` | `?` | `?` |
| `call` at 0x1235ad | `_nvramInit 0x1268d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1235bb | `___stderr 0x18bdcc` | `0x12341e` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1235c3 | `_fprintf 0x17a13c` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1235cb | `_nvramReset 0x126d90` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1235d4 | `_nvramInit 0x1268d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1235f2 | `___stderr 0x18bdcc` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1235fa | `_fprintf 0x17a13c` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x123659 | `_nvramRead 0x126cf0` | `0x8` | `0x1da110` | `?` | `?` | `?` | `?` |
| `call` at 0x12366f | `_bzero 0x170f20` | `0x1da110` | `0xc0` | `?` | `?` | `?` | `?` |
| `call` at 0x12369d | `_nvramWrite 0x126e80` | `0x8` | `0x1da110` | `?` | `?` | `?` | `?` |
| `call` at 0x1236dd | `_initTTYs 0x1245c4` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x123702 | `___stderr 0x18bdcc` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12370a | `_fprintf 0x17a13c` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x123712 | `_logInitSGI 0x124b90` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x123720 | `___stderr 0x18bdcc` | `0x1234b7` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x123728 | `_fprintf 0x17a13c` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x123730 | `_ffscDebugInit 0x11ead0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12373e | `___stderr 0x18bdcc` | `0x1234d7` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x123746 | `_fprintf 0x17a13c` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12375f | `_ioGlobalStdSet 0x187450` | `0x0` | `[0x1ebbfc]` | `?` | `?` | `?` | `?` |
| `call` at 0x12376f | `_ioGlobalStdSet 0x187450` | `0x1` | `[0x1ebbfc]` | `?` | `?` | `?` | `?` |
| `call` at 0x12377f | `_ioGlobalStdSet 0x187450` | `0x2` | `[0x1ebbfc]` | `?` | `?` | `?` | `?` |
| `call` at 0x12378f | `_logInit 0x18f734` | `[0x1ebbfc]` | `0x32` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x12354d | `Z == 0` | `not (Z == 0)` |
| `jle` at 0x123572 | `? <= 0x1 (signed, 32-bit)` | `? > 0x1 (signed, 32-bit)` |
| `jl` at 0x12357b | `[0x1d9a60] < 0x0 (signed, 32-bit)` | `[0x1d9a60] >= 0x0 (signed, 32-bit)` |
| `je` at 0x1235b4 | `(ret(_nvramInit) & ret(_nvramInit)) == 0x0` | `(ret(_nvramInit) & ret(_nvramInit)) != 0x0` |
| `jne` at 0x1235d2 | `(ret(_nvramReset) & ret(_nvramReset)) != 0x0` | `(ret(_nvramReset) & ret(_nvramReset)) == 0x0` |
| `je` at 0x1235db | `(ret(_nvramInit) & ret(_nvramInit)) == 0x0` | `(ret(_nvramInit) & ret(_nvramInit)) != 0x0` |
| `je` at 0x1235eb | `(([0x1da698]:1 & 0xff) & 0x2) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x2) != 0x0` |
| `je` at 0x123609 | `[0x122fd4] == 0x0` | `[0x122fd4] != 0x0` |
| `jne` at 0x123650 | `Z == 0` | `not (Z == 0)` |
| `je` at 0x123663 | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` |
| `jle` at 0x123694 | `? <= 0x24 (signed, 32-bit)` | `? > 0x24 (signed, 32-bit)` |
| `jne` at 0x1236c5 | `Z == 0` | `not (Z == 0)` |
| `jle` at 0x1236db | `? <= 0x24 (signed, 32-bit)` | `? > 0x24 (signed, 32-bit)` |
| `je` at 0x1236e4 | `(ret(_initTTYs) & ret(_initTTYs)) == 0x0` | `(ret(_initTTYs) & ret(_initTTYs)) != 0x0` |
| `je` at 0x1236fb | `(([0x1da698]:1 & 0xff) & 0x2) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x2) != 0x0` |
| `je` at 0x123719 | `(ret(_logInitSGI) & ret(_logInitSGI)) == 0x0` | `(ret(_logInitSGI) & ret(_logInitSGI)) != 0x0` |
| `je` at 0x123737 | `(ret(_ffscDebugInit) & ret(_ffscDebugInit)) == 0x0` | `(ret(_ffscDebugInit) & ret(_ffscDebugInit)) != 0x0` |
| `jl` at 0x123755 | `[0x1ebbfc] < 0x0 (signed, 32-bit)` | `[0x1ebbfc] >= 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12379f | `eax` | `0x0` |
