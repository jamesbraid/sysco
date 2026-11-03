# MMSC election and rack state in patch SG0002378

Read from `patches/SG0002378/usr/cpu/firmware/mmscfw.bin` (patch SG0002378).
Image SHA-256: `b1249e58cf4f00c252362da8ed0515e25bb94aaf096bb60a5c5082c9b6e4d342`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred end and can include literals or padding
after returns. Statements below concern sites reached by known control-flow edges.
No unresolved jump transfers were reported in these spans.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/election-rack-state-sg0002378-facts.txt) include all tool sections.

## `_addrdElectMaster`

- `_addrdElectMaster`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10d7c0 to 0x10d9c0.
  Established. The `_semBCreate` candidates begin `0x0` and `0x0` (`call` at 0x10d7cb). A four-byte
  store saves `ret(_semBCreate)` at `sp -0x10` (`mov` at 0x10d7d3). `_semTake` has leading
  candidates `[0x1f0eec]` and `ret(_sysClkRateGet)` (`call` at 0x10d7f3). The later branch takes
  `(ret(_addrdElectionStart) & ret(_addrdElectionStart)) == 0x0` and falls through for nonzero (`je`
  at 0x10d833). The election-wait policy is not established by the local calls.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10d7d3 | store | 4 | `sp -0x10` | `ret(_semBCreate)` |
| `mov` at 0x10d7ed | load | 4 | `abs 0x1f0eec` | `-` |
| `mov` at 0x10d81e | load | 4 | `abs 0x1f0eec` | `-` |
| `mov` at 0x10d848 | load | 4 | `abs 0x1f0eec` | `-` |
| `mov` at 0x10d862 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x10d885 | load | 4 | `abs 0x1f0eec` | `-` |
| `mov` at 0x10d8be | load | 4 | `abs 0x1f0eec` | `-` |
| `mov` at 0x10d8cc | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x10d8e0 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x10d8eb | load | 4 | `sp -0x1c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10d7cb | `_semBCreate 0x177788` | `0x0` | `0x0` | `ebx` | `?` | `?` | `?` |
| `call` at 0x10d7df | `_clock_gettime 0x17bbcc` | `0x0` | `sp-12` | `?` | `?` | `?` | `ret(_semBCreate)` |
| `call` at 0x10d7e7 | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d7f3 | `_semTake 0x172a54` | `[0x1f0eec]` | `ret(_sysClkRateGet)` | `?` | `?` | `?` | `?` |
| `call` at 0x10d804 | `_ffscMsg 0x11f040` | `0x10d720` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d816 | `_lstAdd 0x190198` | `0x1dd1c0` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x10d824 | `_semGive 0x1729d4` | `[0x1f0eec]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d82c | `_addrdElectionStart 0x10e3c0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d83a | `_ffscMsg 0x11f040` | `0x10d750` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d842 | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d84e | `_semTake 0x172a54` | `[0x1f0eec]` | `ret(_sysClkRateGet)` | `?` | `?` | `?` | `?` |
| `call` at 0x10d866 | `_semTake 0x172a54` | `[sp-16]` | `0xffffffff` | `?` | `?` | `?` | `?` |
| `call` at 0x10d877 | `_ffscMsg 0x11f040` | `0x10d795` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d87f | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d88b | `_semTake 0x172a54` | `[0x1f0eec]` | `ret(_sysClkRateGet)` | `?` | `?` | `?` | `?` |
| `call` at 0x10d89c | `_ffscMsg 0x11f040` | `0x10d769` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d8b6 | `_lstDelete 0x190228` | `0x1dd1c0` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x10d8c4 | `_semGive 0x1729d4` | `[0x1f0eec]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d8d0 | `_semDelete 0x1905ec` | `[sp-16]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d8e4 | `_semDelete 0x1905ec` | `[sp-16]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x10d7fd | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x10d833 | `(ret(_addrdElectionStart) & ret(_addrdElectionStart)) == 0x0` | `(ret(_addrdElectionStart) & ret(_addrdElectionStart)) != 0x0` |
| `jl` at 0x10d858 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10d870 | `(ret(_semTake) & ret(_semTake)) == 0x0` | `(ret(_semTake) & ret(_semTake)) != 0x0` |
| `jge` at 0x10d895 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10d8f1 | `eax` | `?` |

## `_addrdElectionEnd`

- `_addrdElectionEnd`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e380 to 0x10e3c0.
  Established. Four-byte stores place `0x3` at `sp -0x34` (`mov` at 0x10e386) and `0x5` at `sp
  -0x30` (`mov` at 0x10e38d). The `_addrdLocalMsg` candidates begin `sp-52`, `0x3`, and `0x5`
  (`call` at 0x10e398); its decision takes `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) != 0x0` and
  falls through for zero (`jne` at 0x10e39f). Those routes reach EAX `0xffffffff` (`ret` at
  0x10e3b8) and `0x0` (`ret` at 0x10e3a6), respectively. The record meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e386 | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10e38d | store | 4 | `sp -0x30` | `0x5` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e398 | `_addrdLocalMsg 0x10de50` | `sp-52` | `0x3` | `0x5` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e39f | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) != 0x0` | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e3a6 | `eax` | `0x0` |
| `ret` at 0x10e3b8 | `eax` | `0xffffffff` |

## `_addrdElectionStart`

- `_addrdElectionStart`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e3c0 to 0x10e400.
  Established. The stack receives four-byte `0x3` at `sp -0x34` (`mov` at 0x10e3c6) and `0x4` at `sp
  -0x30` (`mov` at 0x10e3cd). `_addrdLocalMsg` presents leading candidates `sp-52`, `0x3`, and `0x4`
  (`call` at 0x10e3d8); its branch takes `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) != 0x0` and
  falls through for zero (`jne` at 0x10e3df). The request encoding is unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e3c6 | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10e3cd | store | 4 | `sp -0x30` | `0x4` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e3d8 | `_addrdLocalMsg 0x10de50` | `sp-52` | `0x3` | `0x4` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e3df | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) != 0x0` | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e3e6 | `eax` | `0x0` |
| `ret` at 0x10e3f8 | `eax` | `0xffffffff` |

## `_addrdElectionTimerPop`

- `_addrdElectionTimerPop`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e400 to 0x10e430.
  Established. Calls `_addrdElectionEnd` with leading candidate entry `ebp` and unknown remaining
  candidates (`call` at 0x10e403). EAX returns `0x0` (`ret` at 0x10e40d). A timer-callback contract
  is not established from the symbol name and call.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e403 | `_addrdElectionEnd 0x10e380` | `ebp` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e40d | `eax` | `0x0` |

## `_addrdFindUnusedRackID`

- `_addrdFindUnusedRackID`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e4e0 to 0x10e570.
  Established. Calling `_identFindByRackID` presents unknown candidates after loop merging (`call`
  at 0x10e4ed); its decision takes `(ret(_identFindByRackID) & ret(_identFindByRackID)) != 0x0` and
  falls through for zero (`jne` at 0x10e4f7). The later loop branch takes `? <= 0x9 (unsigned,
  32-bit)` and falls through for greater values (`jbe` at 0x10e504). The selected rack and merged
  EAX return remain unknown (`ret` at 0x10e511).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e50b | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e4ed | `_identFindByRackID 0x1229b8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e4f7 | `(ret(_identFindByRackID) & ret(_identFindByRackID)) != 0x0` | `(ret(_identFindByRackID) & ret(_identFindByRackID)) == 0x0` |
| `jbe` at 0x10e504 | `? <= 0x9 (unsigned, 32-bit)` | `? > 0x9 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e511 | `eax` | `?` |

## `_addrdProcessElectionEndMsg`

- `_addrdProcessElectionEndMsg`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e680 to 0x10e820.
  Established. The `_identCleanOldEntries` candidates begin `0x1ed4b0` (`call` at 0x10e6a1); its
  branch takes `(ret(_identCleanOldEntries) & ret(_identCleanOldEntries)) == 0x0` and falls through
  for nonzero (`je` at 0x10e6ab). The taken route stores four-byte `ret(_identFindMaster)` at
  absolute `0x1edd6c` (`mov` at 0x10e6c5). Later alternative four-byte stores write `0x1` (`mov` at
  0x10e6e0) or `0x0` (`mov` at 0x10e6fd) at absolute `0x1f113c`. The elected identity and global
  flag meaning remain unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `test` at 0x10e686 | load | 1 | `abs 0x1dccc0` | `-` |
| `mov` at 0x10e6c5 | store | 4 | `abs 0x1edd6c` | `ret(_identFindMaster)` |
| `mov` at 0x10e6e0 | store | 4 | `abs 0x1f113c` | `0x1` |
| `mov` at 0x10e6fd | store | 4 | `abs 0x1f113c` | `0x0` |
| `mov` at 0x10e70d | load | 4 | `abs 0x1f0eec` | `-` |
| `test` at 0x10e774 | load | 1 | `abs 0x1dccc0` | `-` |
| `mov` at 0x10e7a4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10e7b6 | load | 4 | `abs 0x1f0eec` | `-` |
| `mov` at 0x10e7c4 | store | 4 | `abs 0x1dcd0c` | `0x0` |
| `cmp` at 0x10e7ce | load | 4 | `abs 0x1f113c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e694 | `_ffscMsg 0x11f040` | `0x10e5f1` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x10e6a1 | `_identCleanOldEntries 0x122698` | `0x1ed4b0` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e6c0 | `_identFindMaster 0x122b18` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e6d4 | `_ffscCompareUniqueIDs 0x125578` | `0x1f0bf8` | `ret(_identFindMaster)` | `?` | `?` | `?` | `?` |
| `call` at 0x10e6f5 | `_ffscMsg 0x11f040` | `0x10e60d` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e707 | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e713 | `_semTake 0x172a54` | `[0x1f0eec]` | `ret(_sysClkRateGet)` | `?` | `?` | `?` | `?` |
| `call` at 0x10e724 | `_ffscMsg 0x11f040` | `0x10d720` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e745 | `_lstFirst 0x190308` | `0x1dd1c0` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e75d | `_ffscCompareTimestamps 0x125528` | `?` | `0x1ed4b0` | `?` | `?` | `?` | `?` |
| `call` at 0x10e76a | `_lstNext 0x1903b8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e780 | `_ffscTimestampToString 0x125c58` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10e78e | `_ffscMsg 0x11f040` | `0x10e628` | `ret(_ffscTimestampToString)` | `?` | `?` | `?` | `?` |
| `call` at 0x10e79c | `_lstDelete 0x190228` | `0x1dd1c0` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e7a8 | `_semGive 0x1729d4` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e7bc | `_semGive 0x1729d4` | `[0x1f0eec]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e7d7 | `_addrdResolveConflicts 0x10ed10` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e7e5 | `_ffscMsg 0x11f040` | `0x10e655` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10e68d | `(([0x1dccc0]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x1) != 0x0` |
| `je` at 0x10e6ab | `(ret(_identCleanOldEntries) & ret(_identCleanOldEntries)) == 0x0` | `(ret(_identCleanOldEntries) & ret(_identCleanOldEntries)) != 0x0` |
| `je` at 0x10e6cc | `(ret(_identFindMaster) & ret(_identFindMaster)) == 0x0` | `(ret(_identFindMaster) & ret(_identFindMaster)) != 0x0` |
| `jne` at 0x10e6de | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `jge` at 0x10e71d | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x10e751 | `(ret(_lstFirst) & ret(_lstFirst)) == 0x0` | `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` |
| `jg` at 0x10e767 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `je` at 0x10e77b | `(([0x1dccc0]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x10e7b4 | `(ret(_lstNext) & ret(_lstNext)) != 0x0` | `(ret(_lstNext) & ret(_lstNext)) == 0x0` |
| `je` at 0x10e7d5 | `[0x1f113c] == 0x0` | `[0x1f113c] != 0x0` |
| `je` at 0x10e7de | `(ret(_addrdResolveConflicts) & ret(_addrdResolveConflicts)) == 0x0` | `(ret(_addrdResolveConflicts) & ret(_addrdResolveConflicts)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e7f5 | `eax` | `?` |

## `_addrdProcessElectionStartMsg`

- `_addrdProcessElectionStartMsg`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e820 to 0x10e920.
  Established. The `_clock_gettime` call has leading candidates `0x0` and `0x1ed4b0` (`call` at
  0x10e82e). Four-byte stores put `0x3` at `sp -0x34` (`mov` at 0x10e85c) and `0x2` at `sp -0x30`
  (`mov` at 0x10e863). A later branch takes `(ret(_addrdStartElectionTimer) &
  ret(_addrdStartElectionTimer)) != 0x0` and falls through for zero (`jne` at 0x10e88f); its
  fall-through sets absolute `0x1dcd0c` to four-byte `0x1` (`mov` at 0x10e891). The broadcast and
  timer semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `test` at 0x10e836 | load | 1 | `abs 0x1dccc0` | `-` |
| `mov` at 0x10e85c | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10e863 | store | 4 | `sp -0x30` | `0x2` |
| `mov` at 0x10e891 | store | 4 | `abs 0x1dcd0c` | `0x1` |
| `mov` at 0x10e8a5 | load | 4 | `sp -0x38` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e82e | `_clock_gettime 0x17bbcc` | `0x0` | `0x1ed4b0` | `ebx` | `?` | `?` | `?` |
| `call` at 0x10e846 | `_ffscTimestampToString 0x125c58` | `0x1ed4b0` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10e854 | `_ffscMsg 0x11f040` | `0x10e7f6` | `ret(_ffscTimestampToString)` | `?` | `?` | `?` | `?` |
| `call` at 0x10e873 | `_addrdInsertIdentity 0x10ddd0` | `sp-52` | `0x1f0bf8` | `?` | `0x3` | `0x2` | `?` |
| `call` at 0x10e87c | `_addrdBroadcastMsg 0x10dcf0` | `sp-52` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e888 | `_addrdStartElectionTimer 0x10eee0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10e83d | `(([0x1dccc0]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x10e886 | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` |
| `jne` at 0x10e88f | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) != 0x0` | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e8ab | `eax` | `?` |

## `_addrdProcessRequestRackIDMsg`

- `_addrdProcessRequestRackIDMsg`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ebc0 to 0x10ed10.
  Established. The `_addrdExtractIdentity` candidates begin `[sp+4]` and `sp-32` (`call` at
  0x10ebd1). The later branch takes `[0x1dcd0c] == 0x0` and falls through for nonzero (`je` at
  0x10ebf3); its fall-through calls `_addrdStartElectionTimer` with unknown candidates (`call` at
  0x10ebf5). A later conditional four-byte store writes `ret(_addrdFindUnusedRackID)` at `sp -0x30`
  (`mov` at 0x10ec62). The assignment policy and flag meaning remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10ebc9 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x10ebec | load | 4 | `abs 0x1dcd0c` | `-` |
| `test` at 0x10ec02 | load | 1 | `abs 0x1dccc0` | `-` |
| `cmp` at 0x10ec33 | load | 4 | `abs 0x1f113c` | `-` |
| `mov` at 0x10ec62 | store | 4 | `sp -0x30` | `ret(_addrdFindUnusedRackID)` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ebd1 | `_addrdExtractIdentity 0x10dd70` | `[sp+4]` | `sp-32` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x10ebda | `_identUpdate 0x123078` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ebf5 | `_addrdStartElectionTimer 0x10eee0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ec1f | `_ffscMsg 0x11f040` | `0x10eb6e` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ec2b | `_identPrint 0x1231a8` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ec40 | `_identFindConflictingRackID 0x122a98` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ec55 | `_memcpy 0x180150` | `sp-60` | `ret(_identFindConflictingRackID)` | `0x1c` | `?` | `?` | `?` |
| `call` at 0x10ec5d | `_addrdFindUnusedRackID 0x10e4e0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ec6f | `_ffscUniqueIDToString 0x125da8` | `ret(_identFindConflictingRackID)` | `sp-84` | `?` | `?` | `?` | `?` |
| `call` at 0x10ec7d | `_ffscMsg 0x11f040` | `0x10eb8b` | `sp-84` | `?` | `?` | `?` | `?` |
| `call` at 0x10ec86 | `_addrdAssignNewIdentity 0x10e280` | `sp-60` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jl` at 0x10ebe6 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10ebf3 | `[0x1dcd0c] == 0x0` | `[0x1dcd0c] != 0x0` |
| `jne` at 0x10ebfc | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) != 0x0` | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) == 0x0` |
| `je` at 0x10ec09 | `(([0x1dccc0]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x10ec12 | `(ret(_identUpdate) & ret(_identUpdate)) != 0x0` | `(ret(_identUpdate) & ret(_identUpdate)) == 0x0` |
| `je` at 0x10ec3a | `[0x1f113c] == 0x0` | `[0x1f113c] != 0x0` |
| `je` at 0x10ec4c | `(ret(_identFindConflictingRackID) & ret(_identFindConflictingRackID)) == 0x0` | `(ret(_identFindConflictingRackID) & ret(_identFindConflictingRackID)) != 0x0` |
| `jne` at 0x10ec68 | `ret(_addrdFindUnusedRackID) != 0xffffffff` | `ret(_addrdFindUnusedRackID) == 0xffffffff` |
| `je` at 0x10ec8d | `(ret(_addrdAssignNewIdentity) & ret(_addrdAssignNewIdentity)) == 0x0` | `(ret(_addrdAssignNewIdentity) & ret(_addrdAssignNewIdentity)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ecab | `eax` | `?` |

## `_addrdRequestRackID`

- `_addrdRequestRackID`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10db50 to 0x10dbf0.
  Established. Calling `_identSetRackID` presents leading candidate `[[sp+4]+12]` (`call` at
  0x10db5e). The branch takes `(ret(_identSetRackID) & ret(_identSetRackID)) == 0x0` and falls
  through for nonzero (`je` at 0x10db68); its taken route includes `_identUpdate` with leading
  candidate `[sp+4]` (`call` at 0x10db81). Later routes include four-byte stores of `0x3` at `sp
  -0x34` (`mov` at 0x10db9a) and `0x6` at `sp -0x30` (`mov` at 0x10dba1). The rack-request record
  meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10db57 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10db5a | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10db9a | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10dba1 | store | 4 | `sp -0x30` | `0x6` |
| `cmp` at 0x10dbb9 | load | 4 | `abs 0x1f113c` | `-` |
| `mov` at 0x10dbe5 | load | 4 | `sp -0x38` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10db5e | `_identSetRackID 0x123568` | `[[sp+4]+12]` | `ebx` | `?` | `?` | `?` | `?` |
| `call` at 0x10db6f | `_ffscMsg 0x11f040` | `0x10daf3` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10db81 | `_identUpdate 0x123078` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10db92 | `_ffscMsg 0x11f040` | `0x10db0f` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dbb1 | `_addrdInsertIdentity 0x10ddd0` | `sp-52` | `0x1f0bf8` | `?` | `0x3` | `0x6` | `?` |
| `call` at 0x10dbc3 | `_addrdLocalMsg 0x10de50` | `sp-52` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dbd3 | `_addrdBroadcastMsg 0x10dcf0` | `sp-52` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10db68 | `(ret(_identSetRackID) & ret(_identSetRackID)) == 0x0` | `(ret(_identSetRackID) & ret(_identSetRackID)) != 0x0` |
| `je` at 0x10db8b | `(ret(_identUpdate) & ret(_identUpdate)) == 0x0` | `(ret(_identUpdate) & ret(_identUpdate)) != 0x0` |
| `je` at 0x10dbc0 | `[0x1f113c] == 0x0` | `[0x1f113c] != 0x0` |
| `jne` at 0x10dbcd | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) != 0x0` | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) == 0x0` |
| `jne` at 0x10dbda | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10dbeb | `eax` | `?` |

## `_addrdResolveRackIDConflict`

- `_addrdResolveRackIDConflict`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ee30 to 0x10eee0.
  Established. The `_ffscCompareUniqueIDs` candidates begin `[sp+4]` and `[sp+8]` (`call` at
  0x10ee40). The merged `_memcpy` call has leading candidates `sp-32`, unknown, and `0x1c` (`call`
  at 0x10ee67). A four-byte store saves `ret(_addrdFindUnusedRackID)` at `sp -0x14` (`mov` at
  0x10ee74). The next decision takes `ret(_addrdFindUnusedRackID) != 0xffffffff` and falls through
  for equality (`jne` at 0x10ee7a). The selected identity and conflict policy remain unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10ee38 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10ee3b | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10ee74 | store | 4 | `sp -0x14` | `ret(_addrdFindUnusedRackID)` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ee40 | `_ffscCompareUniqueIDs 0x125578` | `[sp+4]` | `[sp+8]` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10ee67 | `_memcpy 0x180150` | `sp-32` | `?` | `0x1c` | `?` | `?` | `?` |
| `call` at 0x10ee6f | `_addrdFindUnusedRackID 0x10e4e0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ee84 | `_ffscUniqueIDToString 0x125da8` | `sp-32` | `sp-56` | `?` | `?` | `?` | `?` |
| `call` at 0x10ee92 | `_ffscMsg 0x11f040` | `0x10ee08` | `sp-56` | `?` | `?` | `?` | `?` |
| `call` at 0x10ee9e | `_addrdAssignNewIdentity 0x10e280` | `sp-32` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x10ee4a | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x10ee7a | `ret(_addrdFindUnusedRackID) != 0xffffffff` | `ret(_addrdFindUnusedRackID) == 0xffffffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10eeab | `eax` | `ret(_addrdAssignNewIdentity)` |

## `_addrdStartElectionTimer`

- `_addrdStartElectionTimer`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10eee0 to 0x10ef28.
  Established. The `_wdStart` candidates begin `[0x1ed4b8]` and `(ret(_sysClkRateGet) +
  ret(_sysClkRateGet))`, followed by unknown slots (`call` at 0x10eef8); its decision takes `l flags
  condition (operands unknown)` and falls through for the negation (`jl` at 0x10ef02). The taken
  route reaches EAX `0xffffffff` (`ret` at 0x10ef22), while the fall-through reaches `0x0` (`ret` at
  0x10ef09). The delay units and call arity remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10eef2 | load | 4 | `abs 0x1ed4b8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10eeea | `_sysClkRateGet 0x1085a8` | `0x10e400` | `0x0` | `ebp` | `?` | `?` | `?` |
| `call` at 0x10eef8 | `_wdStart 0x1750fc` | `[0x1ed4b8]` | `(ret(_sysClkRateGet) + ret(_sysClkRateGet))` | `?` | `?` | `?` | `?` |
| `call` at 0x10ef15 | `_ffscMsg 0x11f040` | `0x10eeac` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jl` at 0x10ef02 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ef09 | `eax` | `0x0` |
| `ret` at 0x10ef22 | `eax` | `0xffffffff` |

## `_identFindMaster`

- `_identFindMaster`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122b18 to 0x122ba8.
  Established. The `_lstFirst` candidates begin `0x1d4d68` (`call` at 0x122b22); its branch takes
  `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` and falls through for zero (`jne` at 0x122b2e). A later
  conditional route calls `_ffscCompareUniqueIDs` with leading candidates `ret(_lstNext)+16` and
  unknown (`call` at 0x122b50); its decision takes `ge flags condition (operands unknown)` and falls
  through for the negation (`jge` at 0x122b5a). The master ordering and returned record remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122b22 | `_lstFirst 0x190308` | `0x1d4d68` | `ebx` | `esi` | `ebp` | `?` | `?` |
| `call` at 0x122b50 | `_ffscCompareUniqueIDs 0x125578` | `ret(_lstNext)+16` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122b5f | `_lstNext 0x1903b8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x122b2e | `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` | `(ret(_lstFirst) & ret(_lstFirst)) == 0x0` |
| `jge` at 0x122b5a | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x122b6b | `(ret(_lstNext) & ret(_lstNext)) != 0x0` | `(ret(_lstNext) & ret(_lstNext)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122b78 | `eax` | `?` |

## `_identSetRackID`

- `_identSetRackID`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x123568 to 0x1235d8.
  Established. A four-byte store saves `[sp+4]` at absolute `0x1f0c04` (`mov` at 0x123573).
  `_nvramWrite` presents leading candidates `0x2` and `0x1f0c04` (`call` at 0x123580); its decision
  takes `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` and falls through for nonzero (`je` at
  0x12358a). After the routes merge, a four-byte store writes `0x1` at absolute `0x1ed498` (`mov` at
  0x123599). The storage-record and notification meanings remain unestablished.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x123570 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x123573 | store | 4 | `abs 0x1f0c04` | `[sp+4]` |
| `mov` at 0x123599 | store | 4 | `abs 0x1ed498` | `0x1` |
| `test` at 0x1235a3 | load | 1 | `abs 0x1dccc1` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x123580 | `_nvramWrite 0x127298` | `0x2` | `0x1f0c04` | `?` | `?` | `?` | `?` |
| `call` at 0x123591 | `_ffscMsg 0x11f040` | `0x123515` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1235b1 | `_ffscRackIDToString 0x125b08` | `[sp+4]` | `sp-16` | `?` | `?` | `?` | `?` |
| `call` at 0x1235bf | `_ffscMsg 0x11f040` | `0x123547` | `sp-16` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12358a | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` |
| `je` at 0x1235aa | `(([0x1dccc1]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dccc1]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1235ce | `eax` | `0x0` |

## `_identFindByRackID`

- `_identFindByRackID`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1229b8 to 0x122a08.
  Established. The entry branch takes `[sp+4] != 0xffffffff` and falls through for equality (`jne`
  at 0x1229c2); its taken route calls `_lstFirst` with leading candidate `0x1d4d68` (`call` at
  0x1229dd). The later comparison decision takes `Z == 1` and falls through for `not (Z == 1)` (`je`
  at 0x1229ed). EAX remains unknown at the merged return (`ret` at 0x122a06). The matched record is
  unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1229bc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1229e8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122a00 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1229dd | `_lstFirst 0x190308` | `0x1d4d68` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x1229f0 | `_lstNext 0x1903b8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x1229c2 | `[sp+4] != 0xffffffff` | `[sp+4] == 0xffffffff` |
| `je` at 0x1229ed | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x1229fc | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122a06 | `eax` | `?` |

## `_identFindConflictingRackID`

- `_identFindConflictingRackID`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122a98 to 0x122b18.
  Established. The entry decision takes `([sp+4] & [sp+4]) == 0x0` and falls through for nonzero
  (`je` at 0x122aa3). The later branch takes `[[sp+4]+12] != 0xffffffff` and falls through for
  equality (`jne` at 0x122aab). A later conditional route calls `_ffscCompareUniqueIDs` with leading
  candidates unknown and `[sp+4]` (`call` at 0x122add). The comparison branch takes
  `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` and falls through for nonzero
  (`je` at 0x122ae7). The conflicting record and rack identity remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x122a9e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122aa5 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x122ae9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122aec | load | 4 | `[sp+4] +0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122acd | `_lstFirst 0x190308` | `0x1d4d68` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x122add | `_ffscCompareUniqueIDs 0x125578` | `?` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x122af4 | `_lstNext 0x1903b8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x122aa3 | `([sp+4] & [sp+4]) == 0x0` | `([sp+4] & [sp+4]) != 0x0` |
| `jne` at 0x122aab | `[[sp+4]+12] != 0xffffffff` | `[[sp+4]+12] == 0xffffffff` |
| `je` at 0x122ae7 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` |
| `je` at 0x122af1 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x122b00 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122b0d | `eax` | `?` |
