# MMSC PIC and rack election in IRIX 6.4

Read from `6.4/usr/cpu/firmware/mmscfw.bin` (IRIX 6.4).
Image SHA-256: `58ac958cb0c3fa09f807340ec3f2f3864e3c601b3fc5f057ae9a8100db18a098`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred end and can include literals
after returns. Statements below concern sites reached by known control-flow edges.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/pic-rack-election-6.4-facts.txt) include all tool sections.

## `_sysIntInitPIC`

- `_sysIntInitPIC`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1082f8 to 0x108398.
  Established. Calls to `_sysOutByte` include candidates `0x20`, `0x11` (`call` at 0x1082ff),
  `0x21`, `0x4` (`call` at 0x10831d), and `0xa0`, `0x11` (`call` at 0x108338). Each pair is the
  first two candidate stack slots, with arity unknown. The final call has candidates `0xa1` and
  `0xffffffff` (`call` at 0x10838a). EAX retains `ret(_sysOutByte)` at the return (`ret` at
  0x108392). The precise peripheral configuration remains unknown without the callee effects.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movsx` at 0x108307 | load | 1 | `abs 0x1aa698` | `-` |
| `mov` at 0x108340 | load | 1 | `abs 0x1aa698` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1082ff | `_sysOutByte 0x1081d0` | `0x20` | `0x11` | `ebp` | `?` | `?` | `?` |
| `call` at 0x108311 | `_sysOutByte 0x1081d0` | `0x21` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10831d | `_sysOutByte 0x1081d0` | `0x21` | `0x4` | `?` | `?` | `?` | `?` |
| `call` at 0x108329 | `_sysOutByte 0x1081d0` | `0x21` | `0x1` | `?` | `?` | `?` | `?` |
| `call` at 0x108338 | `_sysOutByte 0x1081d0` | `0xa0` | `0x11` | `?` | `?` | `?` | `?` |
| `call` at 0x108351 | `_sysOutByte 0x1081d0` | `0xa1` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108360 | `_sysOutByte 0x1081d0` | `0xa1` | `0x2` | `?` | `?` | `?` | `?` |
| `call` at 0x10836f | `_sysOutByte 0x1081d0` | `0xa1` | `0x1` | `?` | `?` | `?` | `?` |
| `call` at 0x10837b | `_sysOutByte 0x1081d0` | `0x21` | `0xfffffffb` | `?` | `?` | `?` | `?` |
| `call` at 0x10838a | `_sysOutByte 0x1081d0` | `0xa1` | `0xffffffff` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108392 | `eax` | `ret(_sysOutByte)` |

## `_sysIntEOI`

- `_sysIntEOI`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108398 to 0x1083b8.
  Established. Two `_sysOutByte` calls appear in sequence. The first has leading candidates `0x20`
  and `0x20` (`call` at 0x10839f), while the second has `0xa0` and `0x20` (`call` at 0x1083ae). The
  exit carries `ret(_sysOutByte)` in EAX (`ret` at 0x1083b6). The interrupt-acknowledgement role
  remains unknown from this caller alone.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10839f | `_sysOutByte 0x1081d0` | `0x20` | `0x20` | `ebp` | `?` | `?` | `?` |
| `call` at 0x1083ae | `_sysOutByte 0x1081d0` | `0xa0` | `0x20` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1083b6 | `eax` | `ret(_sysOutByte)` |

## `_sysIntDisablePIC`

- `_sysIntDisablePIC`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1083b8 to 0x108428.
  Established. A signed branch takes `[sp+4] > 0x7` and falls through for `[sp+4] <= 0x7` (`jg` at
  0x1083c2). The alternative `_sysInByte` calls have leading candidates `0x21` (`call` at 0x1083c6)
  and `0xa1` (`call` at 0x1083ed). All candidates at the merged `_sysOutByte` call are unknown
  (`call` at 0x10840c). The routine returns EAX `0x0` (`ret` at 0x108419), but the resulting mask
  and device effect are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1083bc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x108413 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1083c6 | `_sysInByte 0x1081b0` | `0x21` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x1083ed | `_sysInByte 0x1081b0` | `0xa1` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x10840c | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x1083c2 | `[sp+4] > 0x7 (signed, 32-bit)` | `[sp+4] <= 0x7 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108419 | `eax` | `0x0` |

## `_sysIntEnablePIC`

- `_sysIntEnablePIC`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108428 to 0x108498.
  Established. The `[sp+4] > 0x7` signed branch is taken, with values at most `0x7` falling through
  (`jg` at 0x108432); its arms call `_sysInByte` with leading candidates `0x21` (`call` at 0x108436)
  or `0xa1` (`call` at 0x10845d). The later `_sysOutByte` call has unknown candidates (`call` at
  0x10847c). The return carries EAX `0x0` (`ret` at 0x108489). The enabling role and final hardware
  value remain unknown in this analysis.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10842c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x108483 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108436 | `_sysInByte 0x1081b0` | `0x21` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x10845d | `_sysInByte 0x1081b0` | `0xa1` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x10847c | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x108432 | `[sp+4] > 0x7 (signed, 32-bit)` | `[sp+4] <= 0x7 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108489 | `eax` | `0x0` |

## `_addrdElectMaster`

- `_addrdElectMaster`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10d750 to 0x10d950.
  Established. The `_semBCreate` call has leading candidates `0x0` and `0x0` (`call` at 0x10d75b). A
  word store places its result at `sp -0x10` (`mov` at 0x10d763). Calling `_lstAdd` exposes leading
  candidates `0x1db808` and `sp-24` (`call` at 0x10d7a6). After the `_addrdElectionStart` call
  (`call` at 0x10d7bc), the branch takes `(ret(_addrdElectionStart) & ret(_addrdElectionStart)) ==
  0x0` and falls through on inequality (`je` at 0x10d7c3). That taken route contains `_semTake` with
  candidates `[sp-16]` and `0xffffffff` (`call` at 0x10d7f6). The precise waiting and election
  semantics remain unknown without the callees.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10d763 | store | 4 | `sp -0x10` | `ret(_semBCreate)` |
| `mov` at 0x10d77d | load | 4 | `abs 0x1ef52c` | `-` |
| `mov` at 0x10d7ae | load | 4 | `abs 0x1ef52c` | `-` |
| `mov` at 0x10d7d8 | load | 4 | `abs 0x1ef52c` | `-` |
| `mov` at 0x10d7f2 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x10d815 | load | 4 | `abs 0x1ef52c` | `-` |
| `mov` at 0x10d84e | load | 4 | `abs 0x1ef52c` | `-` |
| `mov` at 0x10d85c | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x10d870 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x10d87b | load | 4 | `sp -0x1c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10d75b | `_semBCreate 0x175e78` | `0x0` | `0x0` | `ebx` | `?` | `?` | `?` |
| `call` at 0x10d76f | `_clock_gettime 0x17a2bc` | `0x0` | `sp-12` | `?` | `?` | `?` | `ret(_semBCreate)` |
| `call` at 0x10d777 | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d783 | `_semTake 0x171d9c` | `[0x1ef52c]` | `ret(_sysClkRateGet)` | `?` | `?` | `?` | `?` |
| `call` at 0x10d794 | `_ffscMsg 0x11edf0` | `0x10d6b0` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d7a6 | `_lstAdd 0x18e888` | `0x1db808` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x10d7b4 | `_semGive 0x171d1c` | `[0x1ef52c]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d7bc | `_addrdElectionStart 0x10e350` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d7ca | `_ffscMsg 0x11edf0` | `0x10d6e0` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d7d2 | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d7de | `_semTake 0x171d9c` | `[0x1ef52c]` | `ret(_sysClkRateGet)` | `?` | `?` | `?` | `?` |
| `call` at 0x10d7f6 | `_semTake 0x171d9c` | `[sp-16]` | `0xffffffff` | `?` | `?` | `?` | `?` |
| `call` at 0x10d807 | `_ffscMsg 0x11edf0` | `0x10d725` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d80f | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d81b | `_semTake 0x171d9c` | `[0x1ef52c]` | `ret(_sysClkRateGet)` | `?` | `?` | `?` | `?` |
| `call` at 0x10d82c | `_ffscMsg 0x11edf0` | `0x10d6f9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d846 | `_lstDelete 0x18e918` | `0x1db808` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x10d854 | `_semGive 0x171d1c` | `[0x1ef52c]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d860 | `_semDelete 0x18ecdc` | `[sp-16]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d874 | `_semDelete 0x18ecdc` | `[sp-16]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x10d78d | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x10d7c3 | `(ret(_addrdElectionStart) & ret(_addrdElectionStart)) == 0x0` | `(ret(_addrdElectionStart) & ret(_addrdElectionStart)) != 0x0` |
| `jl` at 0x10d7e8 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10d800 | `(ret(_semTake) & ret(_semTake)) == 0x0` | `(ret(_semTake) & ret(_semTake)) != 0x0` |
| `jge` at 0x10d825 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10d881 | `eax` | `?` |

## `_addrdElectionEnd`

- `_addrdElectionEnd`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e310 to 0x10e350.
  Established. Word stores place `0x3` at `sp -0x34` (`mov` at 0x10e316) and `0x5` at `sp -0x30`
  (`mov` at 0x10e31d). The `_addrdLocalMsg` call has leading candidates `sp-52`, `0x3`, and `0x5`
  (`call` at 0x10e328). The branch takes `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) != 0x0` and
  falls through for zero (`jne` at 0x10e32f). The two exits carry EAX `0x0` (`ret` at 0x10e336) and
  `0xffffffff` (`ret` at 0x10e348). The external message meaning is unknown here.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e316 | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10e31d | store | 4 | `sp -0x30` | `0x5` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e328 | `_addrdLocalMsg 0x10dde0` | `sp-52` | `0x3` | `0x5` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e32f | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) != 0x0` | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e336 | `eax` | `0x0` |
| `ret` at 0x10e348 | `eax` | `0xffffffff` |

## `_addrdElectionStart`

- `_addrdElectionStart`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e350 to 0x10e390.
  Established. The local record receives word values `0x3` at `sp -0x34` (`mov` at 0x10e356) and
  `0x4` at `sp -0x30` (`mov` at 0x10e35d). `_addrdLocalMsg` sees leading candidates `sp-52`, `0x3`,
  and `0x4` (`call` at 0x10e368). The nonzero `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg))` edge is
  taken and the zero edge falls through (`jne` at 0x10e36f). EAX is zero at the fall-through exit
  (`ret` at 0x10e376), or `0xffffffff` at the taken exit (`ret` at 0x10e388). The request semantics
  beyond those stores and call are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e356 | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10e35d | store | 4 | `sp -0x30` | `0x4` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e368 | `_addrdLocalMsg 0x10dde0` | `sp-52` | `0x3` | `0x4` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e36f | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) != 0x0` | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e376 | `eax` | `0x0` |
| `ret` at 0x10e388 | `eax` | `0xffffffff` |

## `_addrdElectionTimerPop`

- `_addrdElectionTimerPop`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e390 to 0x10e3c0.
  Established. Calls `_addrdElectionEnd` (`call` at 0x10e393) and returns EAX `0x0` (`ret` at
  0x10e39d). The call candidates begin with entry `ebp`, followed by unknown slots (`call` at
  0x10e393). The timer-callback role is unknown from this body alone.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e393 | `_addrdElectionEnd 0x10e310` | `ebp` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e39d | `eax` | `0x0` |

## `_addrdFindUnusedRackID`

- `_addrdFindUnusedRackID`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e470 to 0x10e500.
  Established. The `_identFindByRackID` call has unknown candidates after loop merging (`call` at
  0x10e47d). The branch takes `(ret(_identFindByRackID) & ret(_identFindByRackID)) != 0x0` and falls
  through for zero (`jne` at 0x10e487). A later unsigned branch takes `? <= 0x7` and falls through
  for `? > 0x7`, with the compared value unknown (`jbe` at 0x10e494). The merged return value is
  unknown (`ret` at 0x10e4a1). The rack-selection policy is not established by these partial
  expressions.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e49b | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e47d | `_identFindByRackID 0x1226a8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e487 | `(ret(_identFindByRackID) & ret(_identFindByRackID)) != 0x0` | `(ret(_identFindByRackID) & ret(_identFindByRackID)) == 0x0` |
| `jbe` at 0x10e494 | `? <= 0x7 (unsigned, 32-bit)` | `? > 0x7 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e4a1 | `eax` | `?` |

## `_addrdProcessElectionEndMsg`

- `_addrdProcessElectionEndMsg`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e610 to 0x10e7b0.
  Established. The `_identCleanOldEntries` call has leading candidate `0x1ebaf0` (`call` at
  0x10e631). A word store saves `ret(_identFindMaster)` at absolute `0x1ec3ac` (`mov` at 0x10e655).
  Alternative word stores write `0x1` (`mov` at 0x10e670) or `0x0` (`mov` at 0x10e68d) at absolute
  `0x1ef76c`. The controlling branch takes `(ret(_ffscCompareUniqueIDs) &
  ret(_ffscCompareUniqueIDs)) != 0x0` and falls through for zero (`jne` at 0x10e66e). Another word
  store clears absolute `0x1db3bc` (`mov` at 0x10e754). The meanings of these global flags and the
  election outcome remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `test` at 0x10e616 | load | 1 | `abs 0x1db370` | `-` |
| `mov` at 0x10e655 | store | 4 | `abs 0x1ec3ac` | `ret(_identFindMaster)` |
| `mov` at 0x10e670 | store | 4 | `abs 0x1ef76c` | `0x1` |
| `mov` at 0x10e68d | store | 4 | `abs 0x1ef76c` | `0x0` |
| `mov` at 0x10e69d | load | 4 | `abs 0x1ef52c` | `-` |
| `test` at 0x10e704 | load | 1 | `abs 0x1db370` | `-` |
| `mov` at 0x10e734 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10e746 | load | 4 | `abs 0x1ef52c` | `-` |
| `mov` at 0x10e754 | store | 4 | `abs 0x1db3bc` | `0x0` |
| `cmp` at 0x10e75e | load | 4 | `abs 0x1ef76c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e624 | `_ffscMsg 0x11edf0` | `0x10e581` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x10e631 | `_identCleanOldEntries 0x122388` | `0x1ebaf0` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e650 | `_identFindMaster 0x122808` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e664 | `_ffscCompareUniqueIDs 0x125268` | `0x1ef238` | `ret(_identFindMaster)` | `?` | `?` | `?` | `?` |
| `call` at 0x10e685 | `_ffscMsg 0x11edf0` | `0x10e59d` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e697 | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e6a3 | `_semTake 0x171d9c` | `[0x1ef52c]` | `ret(_sysClkRateGet)` | `?` | `?` | `?` | `?` |
| `call` at 0x10e6b4 | `_ffscMsg 0x11edf0` | `0x10d6b0` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e6d5 | `_lstFirst 0x18e9f8` | `0x1db808` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e6ed | `_ffscCompareTimestamps 0x125218` | `?` | `0x1ebaf0` | `?` | `?` | `?` | `?` |
| `call` at 0x10e6fa | `_lstNext 0x18eaa8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e710 | `_ffscTimestampToString 0x125948` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10e71e | `_ffscMsg 0x11edf0` | `0x10e5b8` | `ret(_ffscTimestampToString)` | `?` | `?` | `?` | `?` |
| `call` at 0x10e72c | `_lstDelete 0x18e918` | `0x1db808` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e738 | `_semGive 0x171d1c` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e74c | `_semGive 0x171d1c` | `[0x1ef52c]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e767 | `_addrdResolveConflicts 0x10eca0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e775 | `_ffscMsg 0x11edf0` | `0x10e5e5` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10e61d | `(([0x1db370]:1 & 0xff) & 0x1) == 0x0` | `(([0x1db370]:1 & 0xff) & 0x1) != 0x0` |
| `je` at 0x10e63b | `(ret(_identCleanOldEntries) & ret(_identCleanOldEntries)) == 0x0` | `(ret(_identCleanOldEntries) & ret(_identCleanOldEntries)) != 0x0` |
| `je` at 0x10e65c | `(ret(_identFindMaster) & ret(_identFindMaster)) == 0x0` | `(ret(_identFindMaster) & ret(_identFindMaster)) != 0x0` |
| `jne` at 0x10e66e | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `jge` at 0x10e6ad | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x10e6e1 | `(ret(_lstFirst) & ret(_lstFirst)) == 0x0` | `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` |
| `jg` at 0x10e6f7 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `je` at 0x10e70b | `(([0x1db370]:1 & 0xff) & 0x1) == 0x0` | `(([0x1db370]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x10e744 | `(ret(_lstNext) & ret(_lstNext)) != 0x0` | `(ret(_lstNext) & ret(_lstNext)) == 0x0` |
| `je` at 0x10e765 | `[0x1ef76c] == 0x0` | `[0x1ef76c] != 0x0` |
| `je` at 0x10e76e | `(ret(_addrdResolveConflicts) & ret(_addrdResolveConflicts)) == 0x0` | `(ret(_addrdResolveConflicts) & ret(_addrdResolveConflicts)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e785 | `eax` | `?` |

## `_addrdProcessElectionStartMsg`

- `_addrdProcessElectionStartMsg`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e7b0 to 0x10e8b0.
  Established. Calling `_clock_gettime` presents leading candidates `0x0` and `0x1ebaf0` (`call` at
  0x10e7be). Word stores write `0x3` at `sp -0x34` (`mov` at 0x10e7ec) and `0x2` at `sp -0x30`
  (`mov` at 0x10e7f3). `_addrdBroadcastMsg` has leading candidate `sp-52` (`call` at 0x10e80c). The
  branch takes `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` and falls through for
  zero (`jne` at 0x10e816). A word store sets absolute `0x1db3bc` to `0x1` (`mov` at 0x10e821). The
  transmitted record semantics are unknown without the message callees.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `test` at 0x10e7c6 | load | 1 | `abs 0x1db370` | `-` |
| `mov` at 0x10e7ec | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10e7f3 | store | 4 | `sp -0x30` | `0x2` |
| `mov` at 0x10e821 | store | 4 | `abs 0x1db3bc` | `0x1` |
| `mov` at 0x10e835 | load | 4 | `sp -0x38` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e7be | `_clock_gettime 0x17a2bc` | `0x0` | `0x1ebaf0` | `ebx` | `?` | `?` | `?` |
| `call` at 0x10e7d6 | `_ffscTimestampToString 0x125948` | `0x1ebaf0` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10e7e4 | `_ffscMsg 0x11edf0` | `0x10e786` | `ret(_ffscTimestampToString)` | `?` | `?` | `?` | `?` |
| `call` at 0x10e803 | `_addrdInsertIdentity 0x10dd60` | `sp-52` | `0x1ef238` | `?` | `0x3` | `0x2` | `?` |
| `call` at 0x10e80c | `_addrdBroadcastMsg 0x10dc80` | `sp-52` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e818 | `_addrdStartElectionTimer 0x10ee70` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10e7cd | `(([0x1db370]:1 & 0xff) & 0x1) == 0x0` | `(([0x1db370]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x10e816 | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` |
| `jne` at 0x10e81f | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) != 0x0` | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e83b | `eax` | `?` |

## `_addrdProcessRequestRackIDMsg`

- `_addrdProcessRequestRackIDMsg`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10eb50 to 0x10eca0.
  Established. The `_addrdExtractIdentity` call has leading candidates `[sp+4]` and `sp-32` (`call`
  at 0x10eb61). A `_memcpy` call presents candidates `sp-60`, `ret(_identFindConflictingRackID)`,
  and `0x1c` (`call` at 0x10ebe5). A word store saves `ret(_addrdFindUnusedRackID)` at `sp -0x30`
  (`mov` at 0x10ebf2). The branch takes `ret(_addrdFindUnusedRackID) != 0xffffffff` and falls
  through for equality (`jne` at 0x10ebf8). `_addrdAssignNewIdentity` receives leading candidate
  `sp-60` (`call` at 0x10ec16). The assignment policy and external effects are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10eb59 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x10eb7c | load | 4 | `abs 0x1db3bc` | `-` |
| `test` at 0x10eb92 | load | 1 | `abs 0x1db370` | `-` |
| `cmp` at 0x10ebc3 | load | 4 | `abs 0x1ef76c` | `-` |
| `mov` at 0x10ebf2 | store | 4 | `sp -0x30` | `ret(_addrdFindUnusedRackID)` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10eb61 | `_addrdExtractIdentity 0x10dd00` | `[sp+4]` | `sp-32` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x10eb6a | `_identUpdate 0x122d68` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10eb85 | `_addrdStartElectionTimer 0x10ee70` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ebaf | `_ffscMsg 0x11edf0` | `0x10eafe` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ebbb | `_identPrint 0x122e98` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ebd0 | `_identFindConflictingRackID 0x122788` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ebe5 | `_memcpy 0x17e840` | `sp-60` | `ret(_identFindConflictingRackID)` | `0x1c` | `?` | `?` | `?` |
| `call` at 0x10ebed | `_addrdFindUnusedRackID 0x10e470` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ebff | `_ffscUniqueIDToString 0x125a98` | `ret(_identFindConflictingRackID)` | `sp-84` | `?` | `?` | `?` | `?` |
| `call` at 0x10ec0d | `_ffscMsg 0x11edf0` | `0x10eb1b` | `sp-84` | `?` | `?` | `?` | `?` |
| `call` at 0x10ec16 | `_addrdAssignNewIdentity 0x10e210` | `sp-60` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jl` at 0x10eb76 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10eb83 | `[0x1db3bc] == 0x0` | `[0x1db3bc] != 0x0` |
| `jne` at 0x10eb8c | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) != 0x0` | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) == 0x0` |
| `je` at 0x10eb99 | `(([0x1db370]:1 & 0xff) & 0x1) == 0x0` | `(([0x1db370]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x10eba2 | `(ret(_identUpdate) & ret(_identUpdate)) != 0x0` | `(ret(_identUpdate) & ret(_identUpdate)) == 0x0` |
| `je` at 0x10ebca | `[0x1ef76c] == 0x0` | `[0x1ef76c] != 0x0` |
| `je` at 0x10ebdc | `(ret(_identFindConflictingRackID) & ret(_identFindConflictingRackID)) == 0x0` | `(ret(_identFindConflictingRackID) & ret(_identFindConflictingRackID)) != 0x0` |
| `jne` at 0x10ebf8 | `ret(_addrdFindUnusedRackID) != 0xffffffff` | `ret(_addrdFindUnusedRackID) == 0xffffffff` |
| `je` at 0x10ec1d | `(ret(_addrdAssignNewIdentity) & ret(_addrdAssignNewIdentity)) == 0x0` | `(ret(_addrdAssignNewIdentity) & ret(_addrdAssignNewIdentity)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ec3b | `eax` | `?` |

## `_addrdRequestRackID`

- `_addrdRequestRackID`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10dae0 to 0x10db80.
  Established. The `_identSetRackID` call exposes leading candidate `[[sp+4]+12]` (`call` at
  0x10daee). The body writes word `0x3` at `sp -0x34` (`mov` at 0x10db2a) and word `0x6` at `sp
  -0x30` (`mov` at 0x10db31). The branch takes `[0x1ef76c] == 0x0` and falls through for nonzero
  (`je` at 0x10db50). The local-message call presents leading candidate `sp-52` (`call` at
  0x10db53), as does the broadcast call (`call` at 0x10db63). The global flag and request policy
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10dae7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10daea | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10db2a | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10db31 | store | 4 | `sp -0x30` | `0x6` |
| `cmp` at 0x10db49 | load | 4 | `abs 0x1ef76c` | `-` |
| `mov` at 0x10db75 | load | 4 | `sp -0x38` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10daee | `_identSetRackID 0x123258` | `[[sp+4]+12]` | `ebx` | `?` | `?` | `?` | `?` |
| `call` at 0x10daff | `_ffscMsg 0x11edf0` | `0x10da83` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10db11 | `_identUpdate 0x122d68` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10db22 | `_ffscMsg 0x11edf0` | `0x10da9f` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10db41 | `_addrdInsertIdentity 0x10dd60` | `sp-52` | `0x1ef238` | `?` | `0x3` | `0x6` | `?` |
| `call` at 0x10db53 | `_addrdLocalMsg 0x10dde0` | `sp-52` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10db63 | `_addrdBroadcastMsg 0x10dc80` | `sp-52` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10daf8 | `(ret(_identSetRackID) & ret(_identSetRackID)) == 0x0` | `(ret(_identSetRackID) & ret(_identSetRackID)) != 0x0` |
| `je` at 0x10db1b | `(ret(_identUpdate) & ret(_identUpdate)) == 0x0` | `(ret(_identUpdate) & ret(_identUpdate)) != 0x0` |
| `je` at 0x10db50 | `[0x1ef76c] == 0x0` | `[0x1ef76c] != 0x0` |
| `jne` at 0x10db5d | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) != 0x0` | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) == 0x0` |
| `jne` at 0x10db6a | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10db7b | `eax` | `?` |

## `_addrdResolveRackIDConflict`

- `_addrdResolveRackIDConflict`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10edc0 to 0x10ee70.
  Established. Calling `_ffscCompareUniqueIDs` presents leading candidates `[sp+4]` and `[sp+8]`
  (`call` at 0x10edd0). The merged `_memcpy` call has candidates `sp-32`, unknown, and `0x1c`
  (`call` at 0x10edf7). A word store saves `ret(_addrdFindUnusedRackID)` at `sp -0x14` (`mov` at
  0x10ee04). The branch takes `ret(_addrdFindUnusedRackID) != 0xffffffff` and falls through for
  equality (`jne` at 0x10ee0a). The exit returns `ret(_addrdAssignNewIdentity)` in EAX (`ret` at
  0x10ee3b). Which identity should change remains unknown from the merged copy source.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10edc8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10edcb | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10ee04 | store | 4 | `sp -0x14` | `ret(_addrdFindUnusedRackID)` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10edd0 | `_ffscCompareUniqueIDs 0x125268` | `[sp+4]` | `[sp+8]` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10edf7 | `_memcpy 0x17e840` | `sp-32` | `?` | `0x1c` | `?` | `?` | `?` |
| `call` at 0x10edff | `_addrdFindUnusedRackID 0x10e470` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ee14 | `_ffscUniqueIDToString 0x125a98` | `sp-32` | `sp-56` | `?` | `?` | `?` | `?` |
| `call` at 0x10ee22 | `_ffscMsg 0x11edf0` | `0x10ed98` | `sp-56` | `?` | `?` | `?` | `?` |
| `call` at 0x10ee2e | `_addrdAssignNewIdentity 0x10e210` | `sp-32` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x10edda | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x10ee0a | `ret(_addrdFindUnusedRackID) != 0xffffffff` | `ret(_addrdFindUnusedRackID) == 0xffffffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ee3b | `eax` | `ret(_addrdAssignNewIdentity)` |

## `_addrdStartElectionTimer`

- `_addrdStartElectionTimer`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ee70 to 0x10eeb8.
  Established. The `_wdStart` call has leading candidates `[0x1ebaf8]` and `(ret(_sysClkRateGet) +
  ret(_sysClkRateGet))` (`call` at 0x10ee88); its remaining candidates are unknown (`call` at
  0x10ee88). The branch is taken for `l flags condition (operands unknown)` and falls through for
  its negation (`jl` at 0x10ee92). The fall-through return carries EAX `0x0` (`ret` at 0x10ee99),
  and the taken route returns `0xffffffff` (`ret` at 0x10eeb2). Timer units, callback arguments and
  the full scheduling role are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10ee82 | load | 4 | `abs 0x1ebaf8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ee7a | `_sysClkRateGet 0x1085a8` | `0x10e390` | `0x0` | `ebp` | `?` | `?` | `?` |
| `call` at 0x10ee88 | `_wdStart 0x174444` | `[0x1ebaf8]` | `(ret(_sysClkRateGet) + ret(_sysClkRateGet))` | `?` | `?` | `?` | `?` |
| `call` at 0x10eea5 | `_ffscMsg 0x11edf0` | `0x10ee3c` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jl` at 0x10ee92 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ee99 | `eax` | `0x0` |
| `ret` at 0x10eeb2 | `eax` | `0xffffffff` |
