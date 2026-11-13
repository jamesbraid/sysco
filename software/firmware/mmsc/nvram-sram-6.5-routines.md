# MMSC NVRAM and SRAM in IRIX 6.5

Read from `6.5/usr/cpu/firmware/mmscfw.bin` (IRIX 6.5).
Image SHA-256: `2017f743fb7360a160f4252b69569b7f9713d9efad82719cfb2fd82538e37ff8`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred end and can include literals
after returns. Statements below concern sites reached by known control-flow edges.
Unresolved computed jumps make the effects incomplete for the affected routines.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/nvram-sram-6.5-facts.txt) include all tool sections.

## `_nvramInit`

- `_nvramInit`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x128538 to 0x128858.
  Established. The `_bzero` calls present leading candidate pairs `0x1df798`, `0x40` (`call` at 0x128545) and
  `0x1dee88`, `0x68` (`call` at 0x128554). A four-byte store writes an unknown value at absolute
  `0x1de8c4` (`mov` at 0x128612). The computed transfer has an unresolved target (`jmp` at
  0x128699). The initialization and compatibility policy behind that selector remain unknown, and
  recorded effects cover known edges only.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x128560 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128566 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x12858f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1285b9 | load | 4 | `ret(___stderr) +0x0` | `-` |
| `mov` at 0x1285de | load | 4 | `ret(___stderr) +0x0` | `-` |
| `mov` at 0x1285f8 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x1285fe | store | 4 | `(unknown)` | `?` |
| `mov` at 0x128612 | store | 4 | `abs 0x1de8c4` | `?` |
| `mov` at 0x12864d | load | 4 | `ret(___stderr) +0x0` | `-` |
| `cmp` at 0x128668 | load | 4 | `abs 0x1df798` | `-` |
| `cmp` at 0x128681 | load | 4 | `abs 0x1df79c` | `-` |
| `mov` at 0x12868e | load | 4 | `abs 0x1df79c` | `-` |
| `jmp` at 0x128699 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1286f5 | store | 4 | `abs 0x1df798` | `0x0` |
| `mov` at 0x128708 | load | 4 | `abs 0x1df79c` | `-` |
| `mov` at 0x128718 | load | 4 | `ret(___stderr) +0x0` | `-` |
| `cmp` at 0x128728 | load | 4 | `abs 0x1df7a0` | `-` |
| `mov` at 0x128731 | load | 4 | `abs 0x1df7a4` | `-` |
| `cmp` at 0x128736 | load | 4 | `abs 0x1de8c4` | `-` |
| `mov` at 0x128748 | load | 4 | `ret(___stderr) +0x0` | `-` |
| `cmp` at 0x128758 | load | 4 | `abs 0x1df7a0` | `-` |
| `cmp` at 0x12876c | load | 4 | `abs 0x1df7a0` | `-` |
| `add` at 0x12877c | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x128781 | load | 4 | `abs 0x1df7a0` | `-` |
| `cmp` at 0x128789 | load | 4 | `abs 0x1df7a4` | `-` |
| `mov` at 0x128798 | load | 4 | `abs 0x1de8c4` | `-` |
| `mov` at 0x12879d | store | 4 | `abs 0x1df7a4` | `[0x1de8c4]` |
| `mov` at 0x1287a2 | store | 4 | `abs 0x1df7a0` | `0xd` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x128545 | `_bzero 0x174e40` | `0x1df798` | `0x40` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x128554 | `_bzero 0x174e40` | `0x1dee88` | `0x68` | `?` | `?` | `?` | `?` |
| `call` at 0x1285b4 | `___stderr 0x190944` | `0x128439` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x1285bc | `_fprintf 0x17ecb4` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1285d9 | `___stderr 0x190944` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1285e1 | `_fprintf 0x17ecb4` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128621 | `_sram_read 0x10c998` | `0x1df798` | `0x40` | `0x0` | `?` | `?` | `?` |
| `call` at 0x128630 | `_nvramReset 0x128a28` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128648 | `___stderr 0x190944` | `0x12848c` | `ret(_sram_read)` | `?` | `?` | `?` | `?` |
| `call` at 0x128650 | `_fprintf 0x17ecb4` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128674 | `_nvramReset 0x128a28` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128713 | `___stderr 0x190944` | `0x12849c` | `[0x1df79c]` | `?` | `?` | `?` | `?` |
| `call` at 0x12871b | `_fprintf 0x17ecb4` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128743 | `___stderr 0x190944` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12874b | `_fprintf 0x17ecb4` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1287ac | `_nvramWriteHeader 0x129088` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x12856f | `be flags condition (operands unknown)` | `not (be flags condition (operands unknown))` |
| `je` at 0x128596 | `? == 0x0` | `? != 0x0` |
| `jne` at 0x1285aa | `Z == 0` | `not (Z == 0)` |
| `jbe` at 0x1285d1 | `be flags condition (operands unknown)` | `not (be flags condition (operands unknown))` |
| `jbe` at 0x12860c | `be flags condition (operands unknown)` | `not (be flags condition (operands unknown))` |
| `jne` at 0x12862e | `ret(_sram_read) != 0xfffffffd` | `ret(_sram_read) == 0xfffffffd` |
| `jne` at 0x128637 | `(ret(_nvramReset) & ret(_nvramReset)) != 0x0` | `(ret(_nvramReset) & ret(_nvramReset)) == 0x0` |
| `je` at 0x128640 | `ret(_sram_read) == 0x1` | `ret(_sram_read) != 0x1` |
| `je` at 0x128672 | `[0x1df798] == 0xff5cff5c` | `[0x1df798] != 0xff5cff5c` |
| `jne` at 0x12867b | `(ret(_nvramReset) & ret(_nvramReset)) != 0x0` | `(ret(_nvramReset) & ret(_nvramReset)) == 0x0` |
| `je` at 0x128688 | `[0x1df79c] == 0x5` | `[0x1df79c] != 0x5` |
| `ja` at 0x128697 | `? > 0x4 (unsigned, 32-bit)` | `? <= 0x4 (unsigned, 32-bit)` |
| `jne` at 0x12872f | `[0x1df7a0] != 0xd` | `[0x1df7a0] == 0xd` |
| `je` at 0x12873c | `[0x1de8c4] == [0x1df7a4]` | `[0x1de8c4] != [0x1df7a4]` |
| `jbe` at 0x12875f | `[0x1df7a0] <= 0xd (unsigned, 32-bit)` | `[0x1df7a0] > 0xd (unsigned, 32-bit)` |
| `jbe` at 0x128772 | `[0x1df7a0] <= 0x0 (unsigned, 32-bit)` | `[0x1df7a0] > 0x0 (unsigned, 32-bit)` |
| `ja` at 0x128787 | `[0x1df7a0] > ? (unsigned, 32-bit)` | `[0x1df7a0] <= ? (unsigned, 32-bit)` |
| `je` at 0x12878f | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x1287b3 | `(ret(_nvramWriteHeader) & ret(_nvramWriteHeader)) != 0x0` | `(ret(_nvramWriteHeader) & ret(_nvramWriteHeader)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1287d6 | `eax` | `?` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x128699).

## `_nvramIsValid`

- `_nvramIsValid`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x128bf8 to 0x128c28.
  Established. The entry value is read as four bytes at `sp +0x4` (`mov` at 0x128bfb). A later
  four-byte read has an unknown effective target (`mov` at 0x128c06). The exit value in EAX is
  unknown (`ret` at 0x128c1b). The validity interpretation remains unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x128bfb | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x128c06 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x128c1b | `eax` | `?` |

## `_nvramPrintInfo`

- `_nvramPrintInfo`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x128858 to 0x128988.
  Established. The branch takes `[0x1df798] == 0xff5cff5c` and falls through for inequality (`je` at
  0x12886b). The taken route includes `_ffscMsg` with leading candidates `0x1287f8` and `[0x1df79c]`
  (`call` at 0x128883). Another `_ffscMsg` call presents `0x128815`, `[0x1df7a0]`, and `[0x1df7a4]`
  (`call` at 0x12889c). EAX is zero at the return (`ret` at 0x128905). The diagnostic-field meanings
  are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x128861 | load | 4 | `abs 0x1df798` | `-` |
| `mov` at 0x128878 | load | 4 | `abs 0x1df79c` | `-` |
| `mov` at 0x12888b | load | 4 | `abs 0x1df7a4` | `-` |
| `mov` at 0x128891 | load | 4 | `abs 0x1df7a0` | `-` |
| `mov` at 0x1288c0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1288d4 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x1288df | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x128883 | `_ffscMsg 0x120504` | `0x1287f8` | `[0x1df79c]` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x12889c | `_ffscMsg 0x120504` | `0x128815` | `[0x1df7a0]` | `[0x1df7a4]` | `?` | `?` | `?` |
| `call` at 0x1288a9 | `_ffscMsgN 0x1205a4` | `0x128837` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1288d7 | `_ffscMsgN 0x1205a4` | `0x12884c` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1288f5 | `_ffscMsg 0x120504` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12886b | `[0x1df798] == 0xff5cff5c` | `[0x1df798] != 0xff5cff5c` |
| `je` at 0x1288c9 | `Z == 1` | `not (Z == 1)` |
| `jle` at 0x1288e8 | `? <= 0x1f (signed, 32-bit)` | `? > 0x1f (signed, 32-bit)` |
| `jle` at 0x1288ee | `? <= 0x3 (signed, 32-bit)` | `? > 0x3 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x128905 | `eax` | `0x0` |

## `_nvramRead`

- `_nvramRead`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x128988 to 0x128a28.
  Established. The unsigned branch takes `[sp+4] > 0xc` and falls through for `[sp+4] <= 0xc` (`ja`
  at 0x128997). The `_nvramIsValid` call presents leading candidate `[sp+4]` (`call` at 0x1289d9).
  The later `_sram_read` call has leading candidate `[sp+8]`, with its remaining candidates unknown
  (`call` at 0x1289f2). The storage record, transferred length and device offset remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12898e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x128991 | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x1289a5 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x1289b8 | load | 4 | `abs 0x1df798` | `-` |
| `mov` at 0x1289e5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1289ec | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1289c9 | `_ffscMsg 0x120504` | `0x12892d` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x1289d9 | `_nvramIsValid 0x128bf8` | `[sp+4]` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x1289f2 | `_sram_read 0x10c998` | `[sp+8]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128a0e | `_ffscMsg 0x120504` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x128997 | `[sp+4] > 0xc (unsigned, 32-bit)` | `[sp+4] <= 0xc (unsigned, 32-bit)` |
| `jne` at 0x1289aa | `? != 0x0` | `? == 0x0` |
| `je` at 0x1289c2 | `[0x1df798] == 0xff5cff5c` | `[0x1df798] != 0xff5cff5c` |
| `je` at 0x1289e3 | `(ret(_nvramIsValid) & ret(_nvramIsValid)) == 0x0` | `(ret(_nvramIsValid) & ret(_nvramIsValid)) != 0x0` |
| `jne` at 0x1289fd | `ret(_sram_read) != 0x1` | `ret(_sram_read) == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x128a21 | `eax` | `?` |

## `_nvramReset`

- `_nvramReset`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x128a28 to 0x128b18.
  Established. Four-byte stores put `0xff5cff5c` at absolute `0x1df798` (`mov` at 0x128a3a), `0x5`
  at `0x1df79c` (`mov` at 0x128a44), and `0xd` at `0x1df7a0` (`mov` at 0x128a4e). A fourth writes
  `[0x1de8c4]` at absolute `0x1df7a4` (`mov` at 0x128a5d). The branch takes `(ret(_nvramWriteHeader)
  & ret(_nvramWriteHeader)) != 0x0` and falls through for zero (`jne` at 0x128a73). The header
  layout semantics and persistent reset effect remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x128a3a | store | 4 | `abs 0x1df798` | `0xff5cff5c` |
| `mov` at 0x128a44 | store | 4 | `abs 0x1df79c` | `0x5` |
| `mov` at 0x128a4e | store | 4 | `abs 0x1df7a0` | `0xd` |
| `mov` at 0x128a58 | load | 4 | `abs 0x1de8c4` | `-` |
| `mov` at 0x128a5d | store | 4 | `abs 0x1df7a4` | `[0x1de8c4]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x128a32 | `_bzero 0x174e40` | `0x1df798` | `0x40` | `ebp` | `?` | `?` | `?` |
| `call` at 0x128a64 | `_nvramValidate 0x129028` | `0x0` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128a6c | `_nvramWriteHeader 0x129088` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x128a73 | `(ret(_nvramWriteHeader) & ret(_nvramWriteHeader)) != 0x0` | `(ret(_nvramWriteHeader) & ret(_nvramWriteHeader)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x128a7a | `eax` | `0x0` |
| `ret` at 0x128a90 | `eax` | `0xffffffff` |

## `_nvramUpgrade1to2`

- `_nvramUpgrade1to2`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x128c28 to 0x128d08.
  Established. The `_nvramRead` call has leading candidates `0x6` and `sp-52` (`call` at 0x128c35);
  its branch takes `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` and falls through for zero (`jne` at
  0x128c3f). The next branch takes `(([sp-32]:1 & 0xff) & 0x1) == 0x0` and falls through for a
  nonzero expression (`je` at 0x128c45). That latter fall-through writes byte `0xa0` at `sp -0x28`
  (`mov` at 0x128c47). The compatibility transformation and record identity remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `test` at 0x128c41 | load | 1 | `sp -0x20` | `-` |
| `mov` at 0x128c47 | store | 1 | `sp -0x28` | `0xa0` |
| `test` at 0x128c6c | load | 1 | `sp -0x20` | `-` |
| `mov` at 0x128c72 | store | 1 | `sp -0x28` | `0xa0` |
| `mov` at 0x128c98 | store | 4 | `abs 0x1df79c` | `0x2` |
| `mov` at 0x128ca7 | load | 4 | `sp -0x38` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x128c35 | `_nvramRead 0x128988` | `0x6` | `sp-52` | `ebx` | `?` | `?` | `?` |
| `call` at 0x128c4e | `_nvramWrite 0x128b18` | `0x6` | `sp-52` | `?` | `?` | `?` | `?` |
| `call` at 0x128c60 | `_nvramRead 0x128988` | `0x7` | `sp-52` | `?` | `?` | `?` | `?` |
| `call` at 0x128c79 | `_nvramWrite 0x128b18` | `0x7` | `sp-52` | `?` | `?` | `?` | `?` |
| `call` at 0x128ca2 | `_nvramWriteHeader 0x129088` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x128c3f | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` |
| `je` at 0x128c45 | `(([sp-32]:1 & 0xff) & 0x1) == 0x0` | `(([sp-32]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x128c58 | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` |
| `jne` at 0x128c6a | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` |
| `je` at 0x128c70 | `(([sp-32]:1 & 0xff) & 0x1) == 0x0` | `(([sp-32]:1 & 0xff) & 0x1) != 0x0` |
| `je` at 0x128c83 | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x128cad | `eax` | `?` |

## `_nvramUpgrade2to3`

- `_nvramUpgrade2to3`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x128d08 to 0x128e78.
  Established. The entry branch takes `[0x124fe4] == 0x0` and falls through for nonzero (`je` at
  0x128d19). A later `_nvramRead` call has leading candidates `0x8` and `sp-388` (`call` at
  0x128d48). `_ffscTuneVarToIndex` presents leading candidate `0x128cae` (`call` at 0x128d5d).
  Four-byte stores on subsequent paths include an unknown value at an unknown effective target
  (`mov` at 0x128d6e). The tuning-field conversion and its storage mapping remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x128d12 | load | 4 | `abs 0x124fe4` | `-` |
| `mov` at 0x128d24 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128d28 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128d2c | store | 4 | `(unknown)` | `?` |
| `cmp` at 0x128d36 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128d67 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128d6e | store | 4 | `(unknown)` | `?` |
| `mov` at 0x128d84 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128d8b | store | 4 | `(unknown)` | `?` |
| `mov` at 0x128da1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128da8 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x128dbe | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128dc5 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x128ddb | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128de2 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x128df8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128dff | store | 4 | `(unknown)` | `?` |
| `mov` at 0x128e2a | load | 2 | `abs 0x12cb88` | `-` |
| `mov` at 0x128e30 | store | 2 | `sp -0x1b0` | `[0x12cb88]:2` |
| `mov` at 0x128e46 | store | 4 | `abs 0x1df79c` | `0x3` |
| `mov` at 0x128e5d | load | 4 | `sp -0x1e8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x128d48 | `_nvramRead 0x128988` | `0x8` | `sp-388` | `?` | `?` | `?` | `?` |
| `call` at 0x128d5d | `_ffscTuneVarToIndex 0x1276c8` | `0x128cae` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128d7a | `_ffscTuneVarToIndex 0x1276c8` | `0x128cbd` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128d97 | `_ffscTuneVarToIndex 0x1276c8` | `0x128cca` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128db4 | `_ffscTuneVarToIndex 0x1276c8` | `0x128cd8` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128dd1 | `_ffscTuneVarToIndex 0x1276c8` | `0x128ce7` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128dee | `_ffscTuneVarToIndex 0x1276c8` | `0x128cf7` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128e09 | `_nvramWrite 0x128b18` | `0x8` | `sp-388` | `?` | `?` | `?` | `?` |
| `call` at 0x128e1e | `_nvramRead 0x128988` | `0x4` | `sp-484` | `?` | `?` | `?` | `?` |
| `call` at 0x128e3a | `_nvramWrite 0x128b18` | `0x4` | `sp-484` | `?` | `?` | `?` | `?` |
| `call` at 0x128e50 | `_nvramWriteHeader 0x129088` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x128d19 | `[0x124fe4] == 0x0` | `[0x124fe4] != 0x0` |
| `jne` at 0x128d3d | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x128d52 | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` |
| `jne` at 0x128e13 | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` |
| `jne` at 0x128e28 | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` |
| `jne` at 0x128e44 | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x128e66 | `eax` | `?` |

## `_nvramUpgrade3to4`

- `_nvramUpgrade3to4`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x128e78 to 0x128f28.
  Established. The first branch takes `[0x124fe4] == 0x0` and falls through for nonzero (`je` at
  0x128e89). `_nvramRead` has leading candidates `0x8` and `sp-388` (`call` at 0x128eb8). Later
  calls include `_ffscTuneVarToIndex` with leading candidate `0x128e67` (`call` at 0x128ec9) and
  `_nvramWrite` with leading candidates `0x8` and `sp-388` (`call` at 0x128ee4). A four-byte store
  on a later conditional path writes `0x4` at absolute `0x1df79c` (`mov` at 0x128ef0). The
  conversion policy and field meaning remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x128e82 | load | 4 | `abs 0x124fe4` | `-` |
| `mov` at 0x128e94 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128e98 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128e9c | store | 4 | `(unknown)` | `?` |
| `cmp` at 0x128ea6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128ed3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128eda | store | 4 | `(unknown)` | `?` |
| `mov` at 0x128ef0 | store | 4 | `abs 0x1df79c` | `0x4` |
| `mov` at 0x128f0d | load | 4 | `sp -0x188` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x128eb8 | `_nvramRead 0x128988` | `0x8` | `sp-388` | `?` | `?` | `?` | `?` |
| `call` at 0x128ec9 | `_ffscTuneVarToIndex 0x1276c8` | `0x128e67` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128ee4 | `_nvramWrite 0x128b18` | `0x8` | `sp-388` | `?` | `?` | `?` | `?` |
| `call` at 0x128efa | `_nvramWriteHeader 0x129088` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x128e89 | `[0x124fe4] == 0x0` | `[0x124fe4] != 0x0` |
| `jne` at 0x128ead | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x128ec2 | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` |
| `jne` at 0x128eee | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x128f16 | `eax` | `?` |

## `_nvramUpgrade4to5`

- `_nvramUpgrade4to5`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x128f28 to 0x129028.
  Established. Calls `_nvramRead` with leading candidates `0x8` and `sp-388` (`call` at 0x128f68).
  `_ffscTuneVarToIndex` has leading candidate `0x128f17` (`call` at 0x128f7d). A later `_nvramWrite`
  call presents leading candidates `0x8` and `sp-388` (`call` at 0x128f98). Stores on subsequent
  paths include two bytes of unknown value at an unknown effective target (`mov` at 0x128fce). The
  tuning and peripheral-configuration changes remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x128f33 | load | 4 | `abs 0x124fe4` | `-` |
| `mov` at 0x128f44 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128f48 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128f4c | store | 4 | `(unknown)` | `?` |
| `cmp` at 0x128f56 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128f87 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128f8e | store | 4 | `(unknown)` | `?` |
| `mov` at 0x128fc9 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x128fce | store | 2 | `(unknown)` | `?` |
| `mov` at 0x128fee | store | 4 | `abs 0x1df79c` | `0x5` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x128f68 | `_nvramRead 0x128988` | `0x8` | `sp-388` | `?` | `?` | `?` | `?` |
| `call` at 0x128f7d | `_ffscTuneVarToIndex 0x1276c8` | `0x128f17` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128f98 | `_nvramWrite 0x128b18` | `0x8` | `sp-388` | `?` | `?` | `?` | `?` |
| `call` at 0x128fad | `_nvramRead 0x128988` | `0x4` | `sp-484` | `?` | `?` | `?` | `?` |
| `call` at 0x128fe2 | `_nvramWrite 0x128b18` | `0x4` | `sp-484` | `?` | `?` | `?` | `?` |
| `call` at 0x128ff8 | `_nvramWriteHeader 0x129088` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x128f3a | `[0x124fe4] == 0x0` | `[0x124fe4] != 0x0` |
| `jne` at 0x128f5d | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x128f72 | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` |
| `jne` at 0x128fa2 | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` |
| `jne` at 0x128fb7 | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` |
| `jle` at 0x128fd7 | `? <= 0x2 (signed, 32-bit)` | `? > 0x2 (signed, 32-bit)` |
| `jne` at 0x128fec | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x129018 | `eax` | `?` |

## `_nvramValidate`

- `_nvramValidate`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x129028 to 0x129088.
  Established. A four-byte load has an unresolved effective target (`mov` at 0x129043). The
  following four-byte store has both unknown target and value (`mov` at 0x12904c). EAX remains
  unknown at the return (`ret` at 0x129059). The validity update and affected record are not
  established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12902f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x129043 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12904c | store | 4 | `(unknown)` | `?` |
| `mov` at 0x129053 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x129059 | `eax` | `?` |

## `_nvramWrite`

- `_nvramWrite`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x128b18 to 0x128bd8.
  Established. The unsigned branch takes `[sp+4] > 0xc` and falls through for values at most `0xc`
  (`ja` at 0x128b26). The `_sram_write` call presents leading candidate `[sp+8]`, followed by
  unknown candidates (`call` at 0x128b75). The merged EAX result is unknown (`ret` at 0x128bd2). The
  write range, storage mapping and validation policy remain unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x128b1d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x128b20 | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x128b34 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x128b48 | load | 4 | `abs 0x1df798` | `-` |
| `mov` at 0x128b68 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128b6f | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x128b59 | `_ffscMsg 0x120504` | `0x128ab9` | `ebx` | `esi` | `ebp` | `?` | `?` |
| `call` at 0x128b75 | `_sram_write 0x10ca78` | `[sp+8]` | `?` | `?` | `ebx` | `esi` | `ebp` |
| `call` at 0x128b88 | `_ffscMsg 0x120504` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128b99 | `_nvramIsValid 0x128bf8` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128ba6 | `_nvramValidate 0x129028` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128bae | `_nvramWriteHeader 0x129088` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x128b26 | `[sp+4] > 0xc (unsigned, 32-bit)` | `[sp+4] <= 0xc (unsigned, 32-bit)` |
| `jne` at 0x128b39 | `? != 0x0` | `? == 0x0` |
| `je` at 0x128b52 | `[0x1df798] == 0xff5cff5c` | `[0x1df798] != 0xff5cff5c` |
| `je` at 0x128b80 | `ret(_sram_write) == 0x1` | `ret(_sram_write) != 0x1` |
| `jne` at 0x128ba3 | `(ret(_nvramIsValid) & ret(_nvramIsValid)) != 0x0` | `(ret(_nvramIsValid) & ret(_nvramIsValid)) == 0x0` |
| `je` at 0x128bb5 | `(ret(_nvramWriteHeader) & ret(_nvramWriteHeader)) == 0x0` | `(ret(_nvramWriteHeader) & ret(_nvramWriteHeader)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x128bd2 | `eax` | `?` |

## `_nvramWriteHeader`

- `_nvramWriteHeader`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x129088 to 0x1290bc.
  Established. The `_sram_write` call has leading candidates `0x1df798`, `0x40`, and `0x0` (`call`
  at 0x129094). The result branch takes `ret(_sram_write) != 0x1` and falls through for equality
  (`jne` at 0x12909f). The fall-through exit carries EAX zero (`ret` at 0x1290a6), and the taken
  route returns `0xffffffff` (`ret` at 0x1290bb). The underlying persistent-memory mapping is
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x129094 | `_sram_write 0x10ca78` | `0x1df798` | `0x40` | `0x0` | `ebp` | `?` | `?` |
| `call` at 0x1290ae | `_ffscMsg 0x120504` | `0x12905a` | `ret(_sram_write)` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x12909f | `ret(_sram_write) != 0x1` | `ret(_sram_write) == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1290a6 | `eax` | `0x0` |
| `ret` at 0x1290bb | `eax` | `0xffffffff` |

## `_nvramZap`

- `_nvramZap`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x128bd8 to 0x128bf8.
  Established. Calling `_bzero` exposes leading candidates `0x1df798` and `0x40` (`call` at
  0x128be2). `_nvramWriteHeader` then receives unknown candidates (`call` at 0x128bea). The return
  retains `ret(_nvramWriteHeader)` in EAX (`ret` at 0x128bf2). The persistent erasure effect implied
  by the name remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x128be2 | `_bzero 0x174e40` | `0x1df798` | `0x40` | `ebp` | `?` | `?` | `?` |
| `call` at 0x128bea | `_nvramWriteHeader 0x129088` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x128bf2 | `eax` | `ret(_nvramWriteHeader)` |

## `_sram_read`

- `_sram_read`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10c998 to 0x10ca78.
  Established. A two-byte read at absolute `0xa2000` (`mov` at 0x10c9b2) is tested by a branch taken
  when `([0xa2000]:2 & 0xffff) == 0xaa55`, falling through for inequality (`je` at 0x10c9bc). A
  later branch takes `[0x1af058] == 0x0` and falls through for nonzero (`je` at 0x10c9cf). Stores on
  later conditional paths include byte zero at an unknown effective target (`mov` at 0x10c9fc) and
  an unknown byte at an unknown target (`mov` at 0x10ca3a). The integrity rule and memory-window
  interpretation remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10c9aa | store | 4 | `sp -0x8` | `ret(_sram_on)` |
| `mov` at 0x10c9b2 | load | 2 | `abs 0xa2000` | `-` |
| `cmp` at 0x10c9c8 | load | 4 | `abs 0x1af058` | `-` |
| `mov` at 0x10c9d8 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10c9f1 | load | 4 | `abs 0xa2004` | `-` |
| `mov` at 0x10c9fc | store | 1 | `(unknown)` | `0x0` |
| `mov` at 0x10ca18 | store | 4 | `abs 0x1af058` | `0x0` |
| `mov` at 0x10ca28 | load | 4 | `sp +0xc` | `-` |
| `cmp` at 0x10ca2e | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10ca34 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10ca37 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10ca3a | store | 1 | `(unknown)` | `?` |
| `cmp` at 0x10ca3f | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10ca44 | load | 4 | `sp +0xc` | `-` |
| `add` at 0x10ca47 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10ca5b | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10c9a5 | `_sram_on 0x10cb38` | `ebx` | `esi` | `edi` | `?` | `ebp` | `?` |
| `call` at 0x10ca5f | `_sram_off 0x10cb68` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10c9bc | `([0xa2000]:2 & 0xffff) == 0xaa55` | `([0xa2000]:2 & 0xffff) != 0xaa55` |
| `je` at 0x10c9cf | `[0x1af058] == 0x0` | `[0x1af058] != 0x0` |
| `jbe` at 0x10c9e9 | `? <= 0x1ff7 (unsigned, 32-bit)` | `? > 0x1ff7 (unsigned, 32-bit)` |
| `je` at 0x10c9f6 | `Z == 1` | `not (Z == 1)` |
| `jbe` at 0x10ca07 | `? <= 0x1ff7 (unsigned, 32-bit)` | `? > 0x1ff7 (unsigned, 32-bit)` |
| `jne` at 0x10ca24 | `(0x0 & 0x0) != 0x0` | `(0x0 & 0x0) == 0x0` |
| `jle` at 0x10ca31 | `[sp+8] <= 0x0 (signed, 32-bit)` | `[sp+8] > 0x0 (signed, 32-bit)` |
| `jg` at 0x10ca42 | `[sp+8] > ? (signed, 32-bit)` | `[sp+8] <= ? (signed, 32-bit)` |
| `jbe` at 0x10ca54 | `([sp+12] + [sp+8]) <= 0x1ff8 (unsigned, 32-bit)` | `([sp+12] + [sp+8]) > 0x1ff8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ca6f | `eax` | `?` |

## `_sram_write`

- `_sram_write`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ca78 to 0x10cb38.
  Established. The branch takes `([0xa2000]:2 & 0xffff) == 0xaa55` and falls through for inequality
  (`je` at 0x10caa1). The fall-through writes two bytes containing `0xaa55` at absolute `0xa2000`
  (`mov` at 0x10caa3), then byte zero at `0xa2002` (`mov` at 0x10caac) and `0xa2003` (`mov` at
  0x10cab3). `_sram_off` has leading candidate `[sp-8]` (`call` at 0x10cb22). The exit returns
  `[sp-12]` in EAX (`ret` at 0x10cb33). The storage format and physical-window behavior remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10ca83 | store | 4 | `sp -0xc` | `0x0` |
| `mov` at 0x10ca8f | store | 4 | `sp -0x8` | `ret(_sram_on)` |
| `mov` at 0x10ca97 | load | 2 | `abs 0xa2000` | `-` |
| `mov` at 0x10caa3 | store | 2 | `abs 0xa2000` | `0xaa55` |
| `mov` at 0x10caac | store | 1 | `abs 0xa2002` | `0x0` |
| `mov` at 0x10cab3 | store | 1 | `abs 0xa2003` | `0x0` |
| `mov` at 0x10cabc | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10cac2 | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x10cac5 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x10cacc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10cacf | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10cad2 | store | 1 | `(unknown)` | `?` |
| `cmp` at 0x10cad7 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10cadc | store | 4 | `abs 0xa2004` | `0x0` |
| `mov` at 0x10cae8 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10cb00 | store | 4 | `abs 0xa2004` | `?` |
| `mov` at 0x10cb03 | load | 4 | `sp +0xc` | `-` |
| `add` at 0x10cb06 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10cb09 | store | 4 | `sp -0xc` | `0x1` |
| `mov` at 0x10cb17 | store | 4 | `sp -0xc` | `0xffffffff` |
| `mov` at 0x10cb1e | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10cb27 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ca8a | `_sram_on 0x10cb38` | `ebx` | `esi` | `edi` | `0x0` | `?` | `ebp` |
| `call` at 0x10cb22 | `_sram_off 0x10cb68` | `[sp-8]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10caa1 | `([0xa2000]:2 & 0xffff) == 0xaa55` | `([0xa2000]:2 & 0xffff) != 0xaa55` |
| `jge` at 0x10cac8 | `[sp-12] >= [sp+8] (signed, 32-bit)` | `[sp-12] < [sp+8] (signed, 32-bit)` |
| `jg` at 0x10cada | `[sp+8] > ? (signed, 32-bit)` | `[sp+8] <= ? (signed, 32-bit)` |
| `jbe` at 0x10caf9 | `? <= 0x1ff7 (unsigned, 32-bit)` | `? > 0x1ff7 (unsigned, 32-bit)` |
| `jbe` at 0x10cb15 | `([sp+12] + [sp+8]) <= 0x1ff8 (unsigned, 32-bit)` | `([sp+12] + [sp+8]) > 0x1ff8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10cb33 | `eax` | `[sp-12]` |
