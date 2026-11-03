# MMSC board support and boot spans in patch SG0002378

Image: `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`.
SHA-256: `b1249e58cf4f00c252362da8ed0515e25bb94aaf096bb60a5c5082c9b6e4d342`.
The [load map](load-map.md) describes its a.out mapping at `0x108000`.

Ends are exclusive inferred boundaries at the next text symbol, or the next global text
symbol across local labels. Spans include padding and can contain data. Effects below concern
known control-flow edges within each separately analysed span. The boot decoding choices are
inferred from coherent instruction boundaries. A connection between its 16-bit and 32-bit
spans is not established across the unresolved indirect transfer.

`sp` is the entry stack pointer of each analysis, not a shared value across boot spans.
Brackets denote loaded values. The 32-bit analyses assume cdecl32 with unknown arity.
The 16-bit calling convention is unknown. Candidate slots may be saved registers, locals or
stale stack words. Unknown values and field meanings remain unknown. The tables include
explicit memory operands. Implicit stack operations appear in the complete fact sheets.

[Full fact sheets](disasm/board-boot-sg0002378-facts.txt) retain all output sections.

## `_sysInit`

- `_sysInit`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108000 to 0x108100.
  Inferred. The decoded 16-bit span has six-byte loads at unknown effective addresses for `lidt`
  (`lidt` at 0x10803e) and `lgdt` (`lgdt` at 0x108048); its indirect jump also reads six bytes
  through an unknown effective address (`jmp` at 0x10807a), with an unresolved destination. In
  separately analysed 32-bit spans, the `_sysA20on` (`call` at 0x1080a6) and `_sysLoadGdt` (`call`
  at 0x1080e8) calls have unknown candidate values. The two `rep movsd` store sites retain unknown
  targets and values (`rep movsd` at 0x108036, `rep movsd` at 0x1080c0). The routine role and the
  connection across the unresolved transfer remain unknown.

### entry decode

Analysis range `0x108000` to `0x108010`, `x86-16`.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|

### alternate decode

Analysis range `0x108010` to `0x108020`, `x86-32`.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x108011 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|

### copy decode

Analysis range `0x108020` to `0x10803e`, `x86-32`.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `rep movsd` at 0x108036 | store | 4 | `(unknown)` | `?` |
| `rep movsd` at 0x108036 | load | 4 | `abs 0x108000` | `-` |
| `mov` at 0x108038 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|

### cold decode

Analysis range `0x10803e` to `0x108090`, `x86-16`.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `lidt` at 0x10803e | load | 6 | `(unknown)` | `-` |
| `lgdt` at 0x108048 | load | 6 | `(unknown)` | `-` |
| `jmp` at 0x10807a | load | 6 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|

The indirect transfer remains unresolved (`jmp` at 0x10807a).

### continuation decode

Analysis range `0x108096` to `0x1080d0`, `x86-32`.

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

### last decode

Analysis range `0x1080d6` to `0x108100`, `x86-32`.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1080e8 | `_sysLoadGdt 0x108290` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `iretd` at 0x1080fe | `eax` | `0x10c7c8` |

### Raw bytes between boot spans

These ranges are retained as data with unknown field meanings.

| Start | Exclusive end | Bytes |
|---|---|---|
| `0x108090` | `0x108096` | `96 80 10 00 08 00` |
| `0x1080d0` | `0x1080d6` | `27 00 d0 82 10 00` |

## `_sysCpuProbe`

- `_sysCpuProbe`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108240 to 0x108290.
  Established. Both conditional branches are taken when `Z == 1` (`je` at 0x108261, `je` at
  0x108273). The three exits report `eax` values `0x2` (`ret` at 0x10827a), `0x1` (`ret` at
  0x108280), and `0x0` (`ret` at 0x108286). These values have no established CPU-model meaning here.

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

- `_sysLoadGdt`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108290 to 0x1082b0.
  Established. The six-byte `lgdt` load uses `[sp+4] +0x0` (`lgdt` at 0x108294). The prior
  four-byte load reads `sp +0x4` (`mov` at 0x108290). No callee is visible, and the exit retains an
  unknown `eax` value (`ret` at 0x1082aa).

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

- `_sysInWordString`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081f0 to 0x108210.
  Established. The three four-byte stack loads read `sp +0x4` (`mov` at 0x1081f1), `sp +0x8` (`mov`
  at 0x1081f5), and `sp +0xc` (`mov` at 0x1081f9). The `rep insw` access is classified unknown with
  width two bytes and an unknown effective address (`rep insw` at 0x1081fe). A total transfer size
  is not established by these facts.

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

- `_sysOutWordString`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108210 to 0x108230.
  Established. The `rep outsw` access has width two bytes and resolved target `[sp+8] +0x0`, but its
  access kind is unknown (`rep outsw` at 0x10821e). The three prior four-byte loads are
  stack-relative (`mov` at 0x108211, `mov` at 0x108215, `mov` at 0x108219). The exit value remains
  unresolved (`ret` at 0x108224).

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

- `_sysWait`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081a0 to 0x1081b0.
  Established. The byte I/O load reads port `0x64` with an unknown value (`in` at 0x1081a7). The
  loop branch reports an unknown flags predicate (`loopne` at 0x1081aa). The tool does not establish
  an iteration count or a device reply from this span.

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

- `_sysMemTop`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b8b8 to 0x10ba38.
  Established. The indexed four-byte store writes `0x12345678` through an unknown effective address
  (`mov` at 0x10b923). The `_bcopy` site has candidates `0x1082d0`, `[0x1abfb4]`, and `0x28` (`call`
  at 0x10b98c). The `_sysLoadGdt` call has candidate zero `sp-24` (`call` at 0x10b9e2). The final
  return expression is `[0x1ac624]` (`ret` at 0x10ba29).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b8c6 | load | 4 | `abs 0x1abfb4` | `-` |
| `cmp` at 0x10b8cc | load | 4 | `abs 0x1ac624` | `-` |
| `mov` at 0x10b8e8 | store | 4 | `abs 0x1ac624` | `?` |
| `mov` at 0x10b91c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b91f | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10b923 | store | 4 | `(unknown)` | `0x12345678` |
| `mov` at 0x10b93d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b958 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b95c | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10b975 | store | 4 | `abs 0x1ac624` | `0x400000` |
| `mov` at 0x10b981 | load | 4 | `abs 0x1abfb4` | `-` |
| `mov` at 0x10b994 | store | 2 | `sp -0x18` | `0x27` |
| `mov` at 0x10b99a | load | 4 | `abs 0x1abfb4` | `-` |
| `mov` at 0x10b99f | store | 4 | `sp -0x16` | `[0x1abfb4]` |
| `mov` at 0x10b9a2 | load | 4 | `abs 0x1ac624` | `-` |
| `mov` at 0x10b9cb | store | 2 | `(unknown)` | `?` |
| `mov` at 0x10b9ce | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10b9d5 | store | 1 | `(unknown)` | `?` |
| `mov` at 0x10b9e7 | load | 4 | `abs 0x1abd94` | `-` |
| `mov` at 0x10b9ec | load | 4 | `abs 0x1ac624` | `-` |
| `mov` at 0x10b9f6 | store | 4 | `abs 0x1abd98` | `?` |
| `mov` at 0x10b9fb | load | 4 | `abs 0x1ac624` | `-` |
| `mov` at 0x10ba00 | store | 4 | `abs 0x1abfb0` | `[0x1ac624]` |
| `mov` at 0x10ba11 | store | 4 | `abs 0x1ac624` | `0xa0000` |
| `mov` at 0x10ba1b | load | 4 | `abs 0x1ac624` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b935 | `_cacheFlush 0x176848` | `0x1` | `?` | `0x10` | `?` | `?` | `?` |
| `call` at 0x10b98c | `_bcopy 0x176bac` | `0x1082d0` | `[0x1abfb4]` | `0x28` | `?` | `?` | `?` |
| `call` at 0x10b9e2 | `_sysLoadGdt 0x108290` | `sp-24` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10b8d3 | `[0x1ac624] == 0x0` | `[0x1ac624] != 0x0` |
| `jle` at 0x10b907 | `0x1f1278 <= 0x100000 (signed, 32-bit)` | `0x1f1278 > 0x100000 (signed, 32-bit)` |
| `jle` at 0x10b92e | `? <= 0x2 (signed, 32-bit)` | `? > 0x2 (signed, 32-bit)` |
| `je` at 0x10b944 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x10b94e | `Z == 1` | `not (Z == 1)` |
| `jle` at 0x10b963 | `? <= 0x2 (signed, 32-bit)` | `? > 0x2 (signed, 32-bit)` |
| `jle` at 0x10b96d | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jne` at 0x10b973 | `(0x0 & 0x0) != 0x0` | `(0x0 & 0x0) == 0x0` |
| `jge` at 0x10b9a9 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jle` at 0x10b9dc | `? <= 0x4 (signed, 32-bit)` | `? > 0x4 (signed, 32-bit)` |
| `jg` at 0x10ba0f | `0x1f1278 > 0xfffff (signed, 32-bit)` | `0x1f1278 <= 0xfffff (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ba29 | `eax` | `[0x1ac624]` |

## `_sysToMonitor`

- `_sysToMonitor`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ba38 to 0x10bb68.
  Established. Two `_sysOutByte` sites present candidates `0x64`, `0xfffffffe` (`call` at 0x10bb39)
  and `0x64`, `0xffffffff` (`call` at 0x10bb4a). Each follows a visible `_sysWait` call (`call` at
  0x10bb30, `call` at 0x10bb41). The `_sysReboot` site has unknown candidates (`call` at 0x10bb52).
  The earlier indirect call sites have unknown callees (`call` at 0x10ba51, `call` at 0x10bb0d).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10ba41 | load | 4 | `abs 0x1d2c74` | `-` |
| `mov` at 0x10ba4c | load | 4 | `abs 0x1d2c74` | `-` |
| `mov` at 0x10ba67 | store | 4 | `sp -0x18` | `0x108004` |
| `mov` at 0x10ba7d | store | 4 | `sp -0x18` | `0x108014` |
| `mov` at 0x10ba84 | store | 4 | `sp -0x24` | `0x0` |
| `mov` at 0x10ba8e | store | 4 | `sp -0x1c` | `sp-20` |
| `mov` at 0x10ba91 | store | 4 | `sp -0x20` | `0x1abfbc` |
| `mov` at 0x10ba9a | load | 4 | `sp -0x1c` | `-` |
| `mov` at 0x10ba9e | load | 4 | `sp -0x24` | `-` |
| `mov` at 0x10baa1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10bab3 | load | 4 | `sp -0x1c` | `-` |
| `mov` at 0x10bacc | load | 4 | `sp -0x24` | `-` |
| `mov` at 0x10bacf | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10bad6 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x10bad9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10bae8 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x10baeb | store | 2 | `(unknown)` | `?` |
| `mov` at 0x10bb06 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10bb0a | load | 4 | `sp -0x18` | `-` |
| `add` at 0x10bb12 | load | 4 | `sp -0x20` | `-` |
| `add` at 0x10bb12 | store | 4 | `sp -0x20` | `[sp-32]+4` |
| `inc` at 0x10bb16 | load | 4 | `sp -0x24` | `-` |
| `inc` at 0x10bb16 | store | 4 | `sp -0x24` | `?` |
| `cmp` at 0x10bb19 | load | 4 | `sp -0x20` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ba51 | `(unknown callee) (unknown target)` | `0x0` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x10baa9 | `_bcopyBytes 0x176c4c` | `?` | `?` | `0x10` | `?` | `?` | `?` |
| `call` at 0x10babc | `_strncmp 0x176d68` | `0x1abfcc` | `[sp-28]` | `0x10` | `?` | `?` | `?` |
| `call` at 0x10bb0d | `(unknown callee) (unknown target)` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10bb26 | `_intLock 0x1712d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10bb2b | `_sysClkDisable 0x1084f8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10bb30 | `_sysWait 0x1081a0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10bb39 | `_sysOutByte 0x1081d0` | `0x64` | `0xfffffffe` | `?` | `?` | `?` | `?` |
| `call` at 0x10bb41 | `_sysWait 0x1081a0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10bb4a | `_sysOutByte 0x1081d0` | `0x64` | `0xffffffff` | `?` | `?` | `?` | `?` |
| `call` at 0x10bb52 | `_sysReboot 0x108230` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10ba48 | `[0x1d2c74] == 0x0` | `[0x1d2c74] != 0x0` |
| `jle` at 0x10ba60 | `0x1f1278 <= 0x100000 (signed, 32-bit)` | `0x1f1278 > 0x100000 (signed, 32-bit)` |
| `jne` at 0x10bac6 | `(ret(_strncmp) & ret(_strncmp)) != 0x0` | `(ret(_strncmp) & ret(_strncmp)) == 0x0` |
| `jle` at 0x10bafb | `? <= 0xff (signed, 32-bit)` | `? > 0xff (signed, 32-bit)` |
| `jle` at 0x10bb04 | `? <= 0x3ff (signed, 32-bit)` | `? > 0x3ff (signed, 32-bit)` |
| `jbe` at 0x10bb20 | `[sp-32] <= 0x1abfc8 (unsigned, 32-bit)` | `[sp-32] > 0x1abfc8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10bb62 | `eax` | `0x0` |

## `_sysStrayInt`

- `_sysStrayInt`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10bbc8 to 0x10bbd8.
  Established. The read-modify-write at absolute `0x1abfac` has width four bytes and an unknown
  stored value (`inc` at 0x10bbcb). There are no call or conditional-branch sites in this span. The
  exit retains the entry expression `eax` (`ret` at 0x10bbd4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `inc` at 0x10bbcb | load | 4 | `abs 0x1abfac` | `-` |
| `inc` at 0x10bbcb | store | 4 | `abs 0x1abfac` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10bbd4 | `eax` | `eax` |

## `_sysProcNumGet`

- `_sysProcNumGet`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10bb88 to 0x10bb98.
  Established. A four-byte load reads absolute `0x1ed4c4` (`mov` at 0x10bb8b). The exit reports that
  loaded expression as `[0x1ed4c4]` (`ret` at 0x10bb93). The global field meaning remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10bb8b | load | 4 | `abs 0x1ed4c4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10bb93 | `eax` | `[0x1ed4c4]` |

## `_sysProcNumSet`

- `_sysProcNumSet`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10bb98 to 0x10bba8.
  Established. The four-byte store writes `[sp+4]` to absolute `0x1ed4c4` (`mov` at 0x10bb9e); its
  value comes from the four-byte load at `sp +0x4` (`mov` at 0x10bb9b). The exit retains `[sp+4]` in
  `eax` (`ret` at 0x10bba6).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10bb9b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10bb9e | store | 4 | `abs 0x1ed4c4` | `[sp+4]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10bba6 | `eax` | `[sp+4]` |

## `_sysDelay`

- `_sysDelay`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10bba8 to 0x10bbc8.
  Established. The `_sysInByte` call has candidate zero `0x84` (`call` at 0x10bbb0); its result
  reaches the exit as `ret(_sysInByte)` (`ret` at 0x10bbb8). The body does not establish timing
  units.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10bbb0 | `_sysInByte 0x1081b0` | `0x84` | `ebp` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10bbb8 | `eax` | `ret(_sysInByte)` |

## `_sysLocalToBusAdrs`

- `_sysLocalToBusAdrs`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1087e8 to 0x1087f8.
  Established. The exit exposes `0xffffffff` in `eax` (`ret` at 0x1087f3). The body has no explicit
  memory operand, call or conditional branch. An address-translation effect is not established.

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

- `_sysBusToLocalAdrs`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1087f8 to 0x108808.
  Established. The return expression is `0xffffffff` (`ret` at 0x108803). The explicit-access table
  and call list are empty. No bus-to-local mapping is proved by the routine name.

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

- `_sysBusIntAck`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108808 to 0x108818.
  Established. The sole exit reports `0x0` in `eax` (`ret` at 0x108810). The body has no explicit
  memory operand, call or conditional branch. An interrupt-acknowledge effect remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108810 | `eax` | `0x0` |
