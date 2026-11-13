# MMSC board and boot-support routines in the 6.4-origin image

Read from `6.4-origin/usr/cpu/firmware/mmscfw.bin`.
Image SHA-256: `b766e559f86078526916d8bd98bbb8079294ad774cce9177928873153c6fb09b`.
The a.out load address is `0x108000`. The [load map](load-map.md) explains mapping.
Ends are exclusive. Most bounds are inferred from the next text symbol.
The `_sysCpuProbe` and `_sysWait` bounds extend across local continuation symbols
to the next global text symbol. Their explicit sizes are 80 and 16 bytes.

`_sysInit` is retained as five independently analysed spans under explicit decoding
assumptions: x86-32 at `0x108000` to `0x10803e`, x86-16 at `0x10803e` to `0x108064`
and `0x108064` to `0x108090`, then x86-32 at `0x108096` to `0x1080d0` and
`0x1080d6` to `0x108100`. This accounts for its full 256-byte range together with
the directly read bytes at `0x108090` to `0x108096`: `96 80 10 00 08 00`, and
`0x1080d0` to `0x1080d6`: `27 00 d0 82 10 00`.
Runtime entry modes, segment bases and a combined CFG remain unknown.
Each span starts with an independent analysis state. The indirect jump remains unresolved
(`jmp` at 0x10807a). The raw records and alternative linear entry bytes receive no
effects beyond those reached by known edges within their stated independent spans.

`sp` denotes the entry stack pointer of the individual analysis and brackets denote loaded values.
x86-32 call candidates use assumed cdecl32 with unknown arity. The x86-16 spans use unknown ABI.
Candidates can include saved registers or stale stack values. Unknown values and addresses stay unknown.
Curated tables show explicit memory operands and omit implicit push/pop accesses retained in full facts.
Call and return implicit stack operations have no separate access rows.
Unsupported instructions and partial-register operations can leave expressions unresolved.
Field, hardware, protocol and callee roles remain unknown unless an entry establishes them.

[Complete fact sheets and listings](disasm/board-boot-64-origin-facts.txt) retain every tool section.

## `_sysInit`

- `_sysInit`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108000 to 0x108100.
  Established for the stated decoding assumptions. The first decoded x86-32 instruction is `cli`
  (`cli` at 0x108000), followed by a direct jump to `0x10803e` (`jmp` at 0x108001). The remainder is
  retained as separately decoded spans under the stated mode assumptions. Runtime entry modes,
  segment bases and combined control flow remain unknown. The tables below keep each span
  independent.

### Independent x86-32 span 0x108000

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|

### Independent x86-16 span 0x10803e

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `lidt` at 0x10803e | load | 6 | `(unknown)` | `-` |
| `lgdt` at 0x108048 | load | 6 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|

### Independent x86-16 span 0x108064

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `jmp` at 0x10807a | load | 6 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|

Unresolved transfer (`jmp` at 0x10807a) excludes effects beyond its unknown edges.

### Independent x86-32 span 0x108096

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `rep movsd` at 0x1080c0 | store | 4 | `(unknown)` | `?` |
| `rep movsd` at 0x1080c0 | load | 4 | `abs 0x108000` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1080a6 | `_sysA20on 0x108100` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|

### Independent x86-32 span 0x1080d6

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1080e8 | `_sysLoadGdt 0x108290` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `iretd` at 0x1080fe | `eax` | `0x10c768` |

## `_sysCpuProbe`

- `_sysCpuProbe`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108240 to 0x108290.
  Established. The two deciding branches are each taken when `Z == 1` (`je` at 0x108261, `je` at
  0x108273). The separate exits retain `0x2` (`ret` at 0x10827a), `0x1` (`ret` at 0x108280), and
  `0x0` (`ret` at 0x108286). These expressions do not establish a CPU-identification catalogue.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x108261 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x108273 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10827a | `eax` | `0x2` |
| `ret` at 0x108280 | `eax` | `0x1` |
| `ret` at 0x108286 | `eax` | `0x0` |

## `_sysLoadGdt`

- `_sysLoadGdt`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108290 to 0x1082b0.
  Established. The four-byte load reads `sp +0x4` (`mov` at 0x108290). The following six-byte load
  uses `[sp+4] +0x0` (`lgdt` at 0x108294). The exit value is unknown (`ret` at 0x1082aa). A
  descriptor-table role is not inferred from the routine name.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x108290 | load | 4 | `sp +0x4` | `-` |
| `lgdt` at 0x108294 | load | 6 | `[sp+4] +0x0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1082aa | `eax` | `?` |

## `_sysInWordString`

- `_sysInWordString`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081f0 to 0x108210.
  Established. The three four-byte loads read `sp +0x4` (`mov` at 0x1081f1), `sp +0x8` (`mov` at
  0x1081f5), and `sp +0xc` (`mov` at 0x1081f9). The repeated I/O instruction has an access row of
  unknown kind and target, with width two bytes (`rep insw` at 0x1081fe). The exit remains unknown
  (`ret` at 0x108204). No transfer count or resolved I/O address is supplied.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1081f1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1081f5 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1081f9 | load | 4 | `sp +0xc` | `-` |
| `rep insw` at 0x1081fe | unknown | 2 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108204 | `eax` | `?` |

## `_sysOutWordString`

- `_sysOutWordString`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108210 to 0x108230.
  Established. The four-byte loads read `sp +0x4` (`mov` at 0x108211), `sp +0x8` (`mov` at
  0x108215), and `sp +0xc` (`mov` at 0x108219). The repeated I/O instruction has an access row of
  unknown kind through `[sp+8] +0x0`, with width two bytes (`rep outsw` at 0x10821e). The exit value
  remains unknown (`ret` at 0x108224). No port or transfer extent is inferred.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x108211 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x108215 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x108219 | load | 4 | `sp +0xc` | `-` |
| `rep outsw` at 0x10821e | unknown | 2 | `[sp+8] +0x0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108224 | `eax` | `?` |

## `_sysWait`

- `_sysWait`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081a0 to 0x1081b0.
  Established. The byte I/O load reads absolute `0x64` (`in` at 0x1081a7). The backward branch uses
  the printed condition `oopne flags condition (operands unknown)` (`loopne` at 0x1081aa); its
  operands stay unknown, and the exit value is unresolved (`ret` at 0x1081ac). The body does not
  establish a wait duration.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `in` at 0x1081a7 | load | 1 | `I/O abs 0x64` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `loopne` at 0x1081aa | `oopne flags condition (operands unknown)` | `not (oopne flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1081ac | `eax` | `?` |

## `_sysMemTop`

- `_sysMemTop`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b858 to 0x10b9d8.
  Established. A four-byte store at absolute `0x1aa274` writes `0x400000` (`mov` at 0x10b915).
  Another site writes `0xa0000` to that address (`mov` at 0x10b9b1). A four-byte store at an unknown
  effective address writes `0x12345678` (`mov` at 0x10b8c3). The `_sysLoadGdt` call presents
  candidate zero `sp-24` (`call` at 0x10b982). No discovered memory size is inferred from these
  literals.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b866 | load | 4 | `abs 0x1a9c04` | `-` |
| `cmp` at 0x10b86c | load | 4 | `abs 0x1aa274` | `-` |
| `mov` at 0x10b888 | store | 4 | `abs 0x1aa274` | `?` |
| `mov` at 0x10b8bc | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b8bf | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10b8c3 | store | 4 | `(unknown)` | `0x12345678` |
| `mov` at 0x10b8dd | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b8f8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b8fc | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10b915 | store | 4 | `abs 0x1aa274` | `0x400000` |
| `mov` at 0x10b921 | load | 4 | `abs 0x1a9c04` | `-` |
| `mov` at 0x10b934 | store | 2 | `sp -0x18` | `0x27` |
| `mov` at 0x10b93a | load | 4 | `abs 0x1a9c04` | `-` |
| `mov` at 0x10b93f | store | 4 | `sp -0x16` | `[0x1a9c04]` |
| `mov` at 0x10b942 | load | 4 | `abs 0x1aa274` | `-` |
| `mov` at 0x10b96b | store | 2 | `(unknown)` | `?` |
| `mov` at 0x10b96e | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10b975 | store | 1 | `(unknown)` | `?` |
| `mov` at 0x10b987 | load | 4 | `abs 0x1a99e4` | `-` |
| `mov` at 0x10b98c | load | 4 | `abs 0x1aa274` | `-` |
| `mov` at 0x10b996 | store | 4 | `abs 0x1a99e8` | `?` |
| `mov` at 0x10b99b | load | 4 | `abs 0x1aa274` | `-` |
| `mov` at 0x10b9a0 | store | 4 | `abs 0x1a9c00` | `[0x1aa274]` |
| `mov` at 0x10b9b1 | store | 4 | `abs 0x1aa274` | `0xa0000` |
| `mov` at 0x10b9bb | load | 4 | `abs 0x1aa274` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b8d5 | `_cacheFlush 0x174498` | `0x1` | `?` | `0x10` | `?` | `?` | `?` |
| `call` at 0x10b92c | `_bcopy 0x1747fc` | `0x1082d0` | `[0x1a9c04]` | `0x28` | `?` | `?` | `?` |
| `call` at 0x10b982 | `_sysLoadGdt 0x108290` | `sp-24` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10b873 | `[0x1aa274] == 0x0` | `[0x1aa274] != 0x0` |
| `jle` at 0x10b8a7 | `0x1eebd0 <= 0x100000 (signed, 32-bit)` | `0x1eebd0 > 0x100000 (signed, 32-bit)` |
| `jle` at 0x10b8ce | `? <= 0x2 (signed, 32-bit)` | `? > 0x2 (signed, 32-bit)` |
| `je` at 0x10b8e4 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x10b8ee | `Z == 1` | `not (Z == 1)` |
| `jle` at 0x10b903 | `? <= 0x2 (signed, 32-bit)` | `? > 0x2 (signed, 32-bit)` |
| `jle` at 0x10b90d | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jne` at 0x10b913 | `(0x0 & 0x0) != 0x0` | `(0x0 & 0x0) == 0x0` |
| `jge` at 0x10b949 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jle` at 0x10b97c | `? <= 0x4 (signed, 32-bit)` | `? > 0x4 (signed, 32-bit)` |
| `jg` at 0x10b9af | `0x1eebd0 > 0xfffff (signed, 32-bit)` | `0x1eebd0 <= 0xfffff (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b9c9 | `eax` | `[0x1aa274]` |

## `_sysToMonitor`

- `_sysToMonitor`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b9d8 to 0x10bb08.
  Established. The `_sysOutByte` sites present leading pairs `0x64`, `0xfffffffe` (`call` at
  0x10bad9) and `0x64`, `0xffffffff` (`call` at 0x10baea). Two indirect calls retain unknown callees
  (`call` at 0x10b9f1, `call` at 0x10baad). Their leading candidates are `0x0` (`call` at 0x10b9f1)
  and `[sp+4]` (`call` at 0x10baad). The exit reports `0x0` (`ret` at 0x10bb02), without
  establishing a reboot or monitor transfer.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10b9e1 | load | 4 | `abs 0x1d086c` | `-` |
| `mov` at 0x10b9ec | load | 4 | `abs 0x1d086c` | `-` |
| `mov` at 0x10ba07 | store | 4 | `sp -0x18` | `0x108004` |
| `mov` at 0x10ba1d | store | 4 | `sp -0x18` | `0x108014` |
| `mov` at 0x10ba24 | store | 4 | `sp -0x24` | `0x0` |
| `mov` at 0x10ba2e | store | 4 | `sp -0x1c` | `sp-20` |
| `mov` at 0x10ba31 | store | 4 | `sp -0x20` | `0x1a9c0c` |
| `mov` at 0x10ba3a | load | 4 | `sp -0x1c` | `-` |
| `mov` at 0x10ba3e | load | 4 | `sp -0x24` | `-` |
| `mov` at 0x10ba41 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10ba53 | load | 4 | `sp -0x1c` | `-` |
| `mov` at 0x10ba6c | load | 4 | `sp -0x24` | `-` |
| `mov` at 0x10ba6f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10ba76 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x10ba79 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10ba88 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x10ba8b | store | 2 | `(unknown)` | `?` |
| `mov` at 0x10baa6 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10baaa | load | 4 | `sp -0x18` | `-` |
| `add` at 0x10bab2 | load | 4 | `sp -0x20` | `-` |
| `add` at 0x10bab2 | store | 4 | `sp -0x20` | `[sp-32]+4` |
| `inc` at 0x10bab6 | load | 4 | `sp -0x24` | `-` |
| `inc` at 0x10bab6 | store | 4 | `sp -0x24` | `?` |
| `cmp` at 0x10bab9 | load | 4 | `sp -0x20` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b9f1 | `(unknown callee) (unknown target)` | `0x0` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x10ba49 | `_bcopyBytes 0x17489c` | `?` | `?` | `0x10` | `?` | `?` | `?` |
| `call` at 0x10ba5c | `_strncmp 0x1749b8` | `0x1a9c1c` | `[sp-28]` | `0x10` | `?` | `?` | `?` |
| `call` at 0x10baad | `(unknown callee) (unknown target)` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10bac6 | `_intLock 0x16fb78` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10bacb | `_sysClkDisable 0x1084f8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10bad0 | `_sysWait 0x1081a0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10bad9 | `_sysOutByte 0x1081d0` | `0x64` | `0xfffffffe` | `?` | `?` | `?` | `?` |
| `call` at 0x10bae1 | `_sysWait 0x1081a0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10baea | `_sysOutByte 0x1081d0` | `0x64` | `0xffffffff` | `?` | `?` | `?` | `?` |
| `call` at 0x10baf2 | `_sysReboot 0x108230` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10b9e8 | `[0x1d086c] == 0x0` | `[0x1d086c] != 0x0` |
| `jle` at 0x10ba00 | `0x1eebd0 <= 0x100000 (signed, 32-bit)` | `0x1eebd0 > 0x100000 (signed, 32-bit)` |
| `jne` at 0x10ba66 | `(ret(_strncmp) & ret(_strncmp)) != 0x0` | `(ret(_strncmp) & ret(_strncmp)) == 0x0` |
| `jle` at 0x10ba9b | `? <= 0xff (signed, 32-bit)` | `? > 0xff (signed, 32-bit)` |
| `jle` at 0x10baa4 | `? <= 0x3ff (signed, 32-bit)` | `? > 0x3ff (signed, 32-bit)` |
| `jbe` at 0x10bac0 | `[sp-32] <= 0x1a9c18 (unsigned, 32-bit)` | `[sp-32] > 0x1a9c18 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10bb02 | `eax` | `0x0` |

## `_sysStrayInt`

- `_sysStrayInt`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10bb68 to 0x10bb78.
  Established. The four-byte read-modify-write at absolute `0x1a9bfc` retains an unknown stored
  value (`inc` at 0x10bb6b). The exit reports the entry expression `eax` (`ret` at 0x10bb74). No
  interrupt-count meaning is assigned to that address.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `inc` at 0x10bb6b | load | 4 | `abs 0x1a9bfc` | `-` |
| `inc` at 0x10bb6b | store | 4 | `abs 0x1a9bfc` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10bb74 | `eax` | `eax` |

## `_sysProcNumGet`

- `_sysProcNumGet`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10bb28 to 0x10bb38.
  Established. The four-byte load reads absolute `0x1eae24` (`mov` at 0x10bb2b), and its expression
  `[0x1eae24]` reaches `eax` at the exit (`ret` at 0x10bb33). A processor-number role remains
  unknown from this body alone.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10bb2b | load | 4 | `abs 0x1eae24` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10bb33 | `eax` | `[0x1eae24]` |

## `_sysProcNumSet`

- `_sysProcNumSet`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10bb38 to 0x10bb48.
  Established. The four-byte load reads `sp +0x4` (`mov` at 0x10bb3b). The four-byte store at
  absolute `0x1eae24` writes `[sp+4]` (`mov` at 0x10bb3e). The exit retains `[sp+4]` (`ret` at
  0x10bb46). No processor-number update semantics is inferred.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10bb3b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10bb3e | store | 4 | `abs 0x1eae24` | `[sp+4]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10bb46 | `eax` | `[sp+4]` |

## `_sysDelay`

- `_sysDelay`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10bb48 to 0x10bb68.
  Established. The `_sysInByte` call presents candidate zero `0x84` (`call` at 0x10bb50). The exit
  reports `ret(_sysInByte)` in `eax` (`ret` at 0x10bb58). The call candidates do not establish a
  delay duration or port role.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10bb50 | `_sysInByte 0x1081b0` | `0x84` | `ebp` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10bb58 | `eax` | `ret(_sysInByte)` |

## `_sysLocalToBusAdrs`

- `_sysLocalToBusAdrs`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1087e8 to 0x1087f8.
  Established. The literal assignment writes `0xffffffff` to `eax` (`mov` at 0x1087eb), which the
  exit retains (`ret` at 0x1087f3). No explicit access or call rows appear in this body. A
  local-to-bus translation effect remains unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1087f3 | `eax` | `0xffffffff` |

## `_sysBusToLocalAdrs`

- `_sysBusToLocalAdrs`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1087f8 to 0x108808.
  Established. The assignment places `0xffffffff` in `eax` (`mov` at 0x1087fb), and that expression
  reaches the exit (`ret` at 0x108803). The tables report no explicit accesses or calls. A
  bus-to-local conversion is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108803 | `eax` | `0xffffffff` |

## `_sysBusIntAck`

- `_sysBusIntAck`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108808 to 0x108818.
  Established. The `eax` assignment retains the printed zero expression (`xor` at 0x10880b), and the
  exit carries `0x0` (`ret` at 0x108810). There are no explicit accesses or calls in this body. An
  interrupt-acknowledgement effect remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108810 | `eax` | `0x0` |
