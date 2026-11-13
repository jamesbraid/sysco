# MMSC election and rack state in IRIX 6.5

Read from `6.5/usr/cpu/firmware/mmscfw.bin` (IRIX 6.5).
Image SHA-256: `2017f743fb7360a160f4252b69569b7f9713d9efad82719cfb2fd82538e37ff8`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred end and can include literals
after returns. Statements below concern sites reached by known control-flow edges.
No unresolved computed transfers were observed in these routine spans.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/election-rack-state-6.5-facts.txt) include all tool sections.

## `_addrdElectMaster`

- `_addrdElectMaster`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10d990 to 0x10db90.
  Established. The `_semBCreate` call has leading candidate values `0x0` and `0x0` (`call` at
  0x10d99b). A four-byte store saves `ret(_semBCreate)` at `sp -0x10` (`mov` at 0x10d9a3). Calling
  `_semTake` presents `[0x1f373c]` and `ret(_sysClkRateGet)` as leading candidates (`call` at
  0x10d9c3). The branch takes `(ret(_addrdElectionStart) & ret(_addrdElectionStart)) == 0x0` and
  falls through for nonzero (`je` at 0x10da03). The election-wait policy and global-state meanings
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10d9a3 | store | 4 | `sp -0x10` | `ret(_semBCreate)` |
| `mov` at 0x10d9bd | load | 4 | `abs 0x1f373c` | `-` |
| `mov` at 0x10d9ee | load | 4 | `abs 0x1f373c` | `-` |
| `mov` at 0x10da18 | load | 4 | `abs 0x1f373c` | `-` |
| `mov` at 0x10da32 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x10da55 | load | 4 | `abs 0x1f373c` | `-` |
| `mov` at 0x10da8e | load | 4 | `abs 0x1f373c` | `-` |
| `mov` at 0x10da9c | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x10dab0 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x10dabb | load | 4 | `sp -0x1c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10d99b | `_semBCreate 0x179f50` | `0x0` | `0x0` | `ebx` | `?` | `?` | `?` |
| `call` at 0x10d9af | `_clock_gettime 0x17e394` | `0x0` | `sp-12` | `?` | `?` | `?` | `ret(_semBCreate)` |
| `call` at 0x10d9b7 | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d9c3 | `_semTake 0x17521c` | `[0x1f373c]` | `ret(_sysClkRateGet)` | `?` | `?` | `?` | `?` |
| `call` at 0x10d9d4 | `_ffscMsg 0x120504` | `0x10d8ea` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d9e6 | `_lstAdd 0x192960` | `0x1dfa08` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x10d9f4 | `_semGive 0x17519c` | `[0x1f373c]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d9fc | `_addrdElectionStart 0x10e5d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10da0a | `_ffscMsg 0x120504` | `0x10d91a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10da12 | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10da1e | `_semTake 0x17521c` | `[0x1f373c]` | `ret(_sysClkRateGet)` | `?` | `?` | `?` | `?` |
| `call` at 0x10da36 | `_semTake 0x17521c` | `[sp-16]` | `0xffffffff` | `?` | `?` | `?` | `?` |
| `call` at 0x10da47 | `_ffscMsg 0x120504` | `0x10d95f` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10da4f | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10da5b | `_semTake 0x17521c` | `[0x1f373c]` | `ret(_sysClkRateGet)` | `?` | `?` | `?` | `?` |
| `call` at 0x10da6c | `_ffscMsg 0x120504` | `0x10d933` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10da86 | `_lstDelete 0x1929f0` | `0x1dfa08` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x10da94 | `_semGive 0x17519c` | `[0x1f373c]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10daa0 | `_semDelete 0x192db4` | `[sp-16]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dab4 | `_semDelete 0x192db4` | `[sp-16]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x10d9cd | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x10da03 | `(ret(_addrdElectionStart) & ret(_addrdElectionStart)) == 0x0` | `(ret(_addrdElectionStart) & ret(_addrdElectionStart)) != 0x0` |
| `jl` at 0x10da28 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10da40 | `(ret(_semTake) & ret(_semTake)) == 0x0` | `(ret(_semTake) & ret(_semTake)) != 0x0` |
| `jge` at 0x10da65 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10dac1 | `eax` | `?` |

## `_addrdElectionEnd`

- `_addrdElectionEnd`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e590 to 0x10e5d0.
  Established. Four-byte stores put `0x3` at `sp -0x34` (`mov` at 0x10e596) and `0x5` at `sp -0x30`
  (`mov` at 0x10e59d). `_addrdLocalMsg` has leading candidates `sp-52`, `0x3`, and `0x5` (`call` at
  0x10e5a8). The result branch takes `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) != 0x0` and falls
  through for zero (`jne` at 0x10e5af). The two routes return EAX `0xffffffff` (`ret` at 0x10e5c8)
  and `0x0` (`ret` at 0x10e5b6), respectively. The queued record semantics are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e596 | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10e59d | store | 4 | `sp -0x30` | `0x5` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e5a8 | `_addrdLocalMsg 0x10e020` | `sp-52` | `0x3` | `0x5` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e5af | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) != 0x0` | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e5b6 | `eax` | `0x0` |
| `ret` at 0x10e5c8 | `eax` | `0xffffffff` |

## `_addrdElectionStart`

- `_addrdElectionStart`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e5d0 to 0x10e610.
  Established. The stack receives four-byte values `0x3` at `sp -0x34` (`mov` at 0x10e5d6) and `0x4`
  at `sp -0x30` (`mov` at 0x10e5dd). The `_addrdLocalMsg` candidates begin `sp-52`, `0x3`, and `0x4`
  (`call` at 0x10e5e8). The branch takes `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) != 0x0` and
  falls through for zero (`jne` at 0x10e5ef); its taken exit carries EAX `0xffffffff` (`ret` at
  0x10e608), while the fall-through exit carries `0x0` (`ret` at 0x10e5f6). The request-type meaning
  remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e5d6 | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10e5dd | store | 4 | `sp -0x30` | `0x4` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e5e8 | `_addrdLocalMsg 0x10e020` | `sp-52` | `0x3` | `0x4` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e5ef | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) != 0x0` | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e5f6 | `eax` | `0x0` |
| `ret` at 0x10e608 | `eax` | `0xffffffff` |

## `_addrdElectionTimerPop`

- `_addrdElectionTimerPop`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e610 to 0x10e640.
  Established. Calls `_addrdElectionEnd` with leading candidate entry `ebp` and unknown remaining
  candidates (`call` at 0x10e613). The return carries EAX `0x0` (`ret` at 0x10e61d). The
  timer-callback purpose is unknown beyond that call and constant result.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e613 | `_addrdElectionEnd 0x10e590` | `ebp` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e61d | `eax` | `0x0` |

## `_addrdFindUnusedRackID`

- `_addrdFindUnusedRackID`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e6f0 to 0x10e780.
  Established. The `_identFindByRackID` call has unknown candidate slots after loop merging (`call`
  at 0x10e6fd). The branch takes `(ret(_identFindByRackID) & ret(_identFindByRackID)) != 0x0` and
  falls through for zero (`jne` at 0x10e707). A later unsigned branch takes `? <= 0x9` and falls
  through for larger values (`jbe` at 0x10e714). The return value is unknown (`ret` at 0x10e721), as
  is the concrete rack-selection rule.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e71b | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e6fd | `_identFindByRackID 0x124368` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e707 | `(ret(_identFindByRackID) & ret(_identFindByRackID)) != 0x0` | `(ret(_identFindByRackID) & ret(_identFindByRackID)) == 0x0` |
| `jbe` at 0x10e714 | `? <= 0x9 (unsigned, 32-bit)` | `? > 0x9 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e721 | `eax` | `?` |

## `_addrdProcessElectionEndMsg`

- `_addrdProcessElectionEndMsg`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e890 to 0x10ea30.
  Established. The `_identCleanOldEntries` call presents leading candidate `0x1efd00` (`call` at
  0x10e8b1); its branch takes `(ret(_identCleanOldEntries) & ret(_identCleanOldEntries)) == 0x0` and
  falls through for nonzero (`je` at 0x10e8bb). The taken route includes a four-byte store of
  `ret(_identFindMaster)` at absolute `0x1f05c0` (`mov` at 0x10e8d5). Later alternative stores put
  four-byte `0x1` (`mov` at 0x10e8f0) or `0x0` (`mov` at 0x10e90d) at absolute `0x1f398c`. The
  elected identity and global flag interpretation remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `test` at 0x10e896 | load | 1 | `abs 0x1df500` | `-` |
| `mov` at 0x10e8d5 | store | 4 | `abs 0x1f05c0` | `ret(_identFindMaster)` |
| `mov` at 0x10e8f0 | store | 4 | `abs 0x1f398c` | `0x1` |
| `mov` at 0x10e90d | store | 4 | `abs 0x1f398c` | `0x0` |
| `mov` at 0x10e91d | load | 4 | `abs 0x1f373c` | `-` |
| `test` at 0x10e984 | load | 1 | `abs 0x1df500` | `-` |
| `mov` at 0x10e9b4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10e9c6 | load | 4 | `abs 0x1f373c` | `-` |
| `mov` at 0x10e9d4 | store | 4 | `abs 0x1df554` | `0x0` |
| `cmp` at 0x10e9de | load | 4 | `abs 0x1f398c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e8a4 | `_ffscMsg 0x120504` | `0x10e801` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x10e8b1 | `_identCleanOldEntries 0x124048` | `0x1efd00` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e8d0 | `_identFindMaster 0x1244c8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e8e4 | `_ffscCompareUniqueIDs 0x126f28` | `0x1f3448` | `ret(_identFindMaster)` | `?` | `?` | `?` | `?` |
| `call` at 0x10e905 | `_ffscMsg 0x120504` | `0x10e81d` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e917 | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e923 | `_semTake 0x17521c` | `[0x1f373c]` | `ret(_sysClkRateGet)` | `?` | `?` | `?` | `?` |
| `call` at 0x10e934 | `_ffscMsg 0x120504` | `0x10d8ea` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e955 | `_lstFirst 0x192ad0` | `0x1dfa08` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e96d | `_ffscCompareTimestamps 0x126ed8` | `?` | `0x1efd00` | `?` | `?` | `?` | `?` |
| `call` at 0x10e97a | `_lstNext 0x192b80` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e990 | `_ffscTimestampToString 0x127608` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10e99e | `_ffscMsg 0x120504` | `0x10e838` | `ret(_ffscTimestampToString)` | `?` | `?` | `?` | `?` |
| `call` at 0x10e9ac | `_lstDelete 0x1929f0` | `0x1dfa08` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e9b8 | `_semGive 0x17519c` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e9cc | `_semGive 0x17519c` | `[0x1f373c]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e9e7 | `_addrdResolveConflicts 0x10ef20` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e9f5 | `_ffscMsg 0x120504` | `0x10e865` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10e89d | `(([0x1df500]:1 & 0xff) & 0x1) == 0x0` | `(([0x1df500]:1 & 0xff) & 0x1) != 0x0` |
| `je` at 0x10e8bb | `(ret(_identCleanOldEntries) & ret(_identCleanOldEntries)) == 0x0` | `(ret(_identCleanOldEntries) & ret(_identCleanOldEntries)) != 0x0` |
| `je` at 0x10e8dc | `(ret(_identFindMaster) & ret(_identFindMaster)) == 0x0` | `(ret(_identFindMaster) & ret(_identFindMaster)) != 0x0` |
| `jne` at 0x10e8ee | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `jge` at 0x10e92d | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x10e961 | `(ret(_lstFirst) & ret(_lstFirst)) == 0x0` | `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` |
| `jg` at 0x10e977 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `je` at 0x10e98b | `(([0x1df500]:1 & 0xff) & 0x1) == 0x0` | `(([0x1df500]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x10e9c4 | `(ret(_lstNext) & ret(_lstNext)) != 0x0` | `(ret(_lstNext) & ret(_lstNext)) == 0x0` |
| `je` at 0x10e9e5 | `[0x1f398c] == 0x0` | `[0x1f398c] != 0x0` |
| `je` at 0x10e9ee | `(ret(_addrdResolveConflicts) & ret(_addrdResolveConflicts)) == 0x0` | `(ret(_addrdResolveConflicts) & ret(_addrdResolveConflicts)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ea05 | `eax` | `?` |

## `_addrdProcessElectionStartMsg`

- `_addrdProcessElectionStartMsg`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ea30 to 0x10eb30.
  Established. Calling `_clock_gettime` exposes leading candidates `0x0` and `0x1efd00` (`call` at
  0x10ea3e). Four-byte stores place `0x3` at `sp -0x34` (`mov` at 0x10ea6c) and `0x2` at `sp -0x30`
  (`mov` at 0x10ea73). The `_addrdInsertIdentity` call presents leading candidates `sp-52` and
  `0x1f3448` (`call` at 0x10ea83). The branch takes `(ret(_addrdBroadcastMsg) &
  ret(_addrdBroadcastMsg)) != 0x0` and falls through for zero (`jne` at 0x10ea96). The broadcast
  record and timer-start semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `test` at 0x10ea46 | load | 1 | `abs 0x1df500` | `-` |
| `mov` at 0x10ea6c | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10ea73 | store | 4 | `sp -0x30` | `0x2` |
| `mov` at 0x10eaa1 | store | 4 | `abs 0x1df554` | `0x1` |
| `mov` at 0x10eab5 | load | 4 | `sp -0x38` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ea3e | `_clock_gettime 0x17e394` | `0x0` | `0x1efd00` | `ebx` | `?` | `?` | `?` |
| `call` at 0x10ea56 | `_ffscTimestampToString 0x127608` | `0x1efd00` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10ea64 | `_ffscMsg 0x120504` | `0x10ea06` | `ret(_ffscTimestampToString)` | `?` | `?` | `?` | `?` |
| `call` at 0x10ea83 | `_addrdInsertIdentity 0x10dfa0` | `sp-52` | `0x1f3448` | `?` | `0x3` | `0x2` | `?` |
| `call` at 0x10ea8c | `_addrdBroadcastMsg 0x10dec0` | `sp-52` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ea98 | `_addrdStartElectionTimer 0x10f0f0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10ea4d | `(([0x1df500]:1 & 0xff) & 0x1) == 0x0` | `(([0x1df500]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x10ea96 | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` |
| `jne` at 0x10ea9f | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) != 0x0` | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10eabb | `eax` | `?` |

## `_addrdProcessRequestRackIDMsg`

- `_addrdProcessRequestRackIDMsg`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10edd0 to 0x10ef20.
  Established. The `_addrdExtractIdentity` call has leading candidates `[sp+4]` and `sp-32` (`call`
  at 0x10ede1). `_identUpdate` then presents leading candidate `sp-32` (`call` at 0x10edea). The
  branch takes `[0x1df554] == 0x0` and falls through for nonzero (`je` at 0x10ee03); its
  fall-through includes `_addrdStartElectionTimer` with unknown candidates (`call` at 0x10ee05). A
  later conditional four-byte store saves `ret(_addrdFindUnusedRackID)` at `sp -0x30` (`mov` at
  0x10ee72). The assignment policy and global flag meaning are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10edd9 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x10edfc | load | 4 | `abs 0x1df554` | `-` |
| `test` at 0x10ee12 | load | 1 | `abs 0x1df500` | `-` |
| `cmp` at 0x10ee43 | load | 4 | `abs 0x1f398c` | `-` |
| `mov` at 0x10ee72 | store | 4 | `sp -0x30` | `ret(_addrdFindUnusedRackID)` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ede1 | `_addrdExtractIdentity 0x10df40` | `[sp+4]` | `sp-32` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x10edea | `_identUpdate 0x124a28` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ee05 | `_addrdStartElectionTimer 0x10f0f0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ee2f | `_ffscMsg 0x120504` | `0x10ed7e` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ee3b | `_identPrint 0x124b58` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ee50 | `_identFindConflictingRackID 0x124448` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ee65 | `_memcpy 0x182918` | `sp-60` | `ret(_identFindConflictingRackID)` | `0x1c` | `?` | `?` | `?` |
| `call` at 0x10ee6d | `_addrdFindUnusedRackID 0x10e6f0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ee7f | `_ffscUniqueIDToString 0x127758` | `ret(_identFindConflictingRackID)` | `sp-84` | `?` | `?` | `?` | `?` |
| `call` at 0x10ee8d | `_ffscMsg 0x120504` | `0x10ed9b` | `sp-84` | `?` | `?` | `?` | `?` |
| `call` at 0x10ee96 | `_addrdAssignNewIdentity 0x10e490` | `sp-60` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jl` at 0x10edf6 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10ee03 | `[0x1df554] == 0x0` | `[0x1df554] != 0x0` |
| `jne` at 0x10ee0c | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) != 0x0` | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) == 0x0` |
| `je` at 0x10ee19 | `(([0x1df500]:1 & 0xff) & 0x1) == 0x0` | `(([0x1df500]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x10ee22 | `(ret(_identUpdate) & ret(_identUpdate)) != 0x0` | `(ret(_identUpdate) & ret(_identUpdate)) == 0x0` |
| `je` at 0x10ee4a | `[0x1f398c] == 0x0` | `[0x1f398c] != 0x0` |
| `je` at 0x10ee5c | `(ret(_identFindConflictingRackID) & ret(_identFindConflictingRackID)) == 0x0` | `(ret(_identFindConflictingRackID) & ret(_identFindConflictingRackID)) != 0x0` |
| `jne` at 0x10ee78 | `ret(_addrdFindUnusedRackID) != 0xffffffff` | `ret(_addrdFindUnusedRackID) == 0xffffffff` |
| `je` at 0x10ee9d | `(ret(_addrdAssignNewIdentity) & ret(_addrdAssignNewIdentity)) == 0x0` | `(ret(_addrdAssignNewIdentity) & ret(_addrdAssignNewIdentity)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10eebb | `eax` | `?` |

## `_addrdRequestRackID`

- `_addrdRequestRackID`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10dd20 to 0x10ddc0.
  Established. The `_identSetRackID` call presents leading candidate `[[sp+4]+12]` (`call` at
  0x10dd2e). The branch takes `(ret(_identSetRackID) & ret(_identSetRackID)) == 0x0` and falls
  through for nonzero (`je` at 0x10dd38). The taken route includes `_identUpdate` with leading
  candidate `[sp+4]` (`call` at 0x10dd51). Later paths include four-byte stores of `0x3` at `sp
  -0x34` (`mov` at 0x10dd6a) and `0x6` at `sp -0x30` (`mov` at 0x10dd71). The rack-request record
  and local-delivery policy remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10dd27 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10dd2a | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10dd6a | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10dd71 | store | 4 | `sp -0x30` | `0x6` |
| `cmp` at 0x10dd89 | load | 4 | `abs 0x1f398c` | `-` |
| `mov` at 0x10ddb5 | load | 4 | `sp -0x38` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10dd2e | `_identSetRackID 0x124f18` | `[[sp+4]+12]` | `ebx` | `?` | `?` | `?` | `?` |
| `call` at 0x10dd3f | `_ffscMsg 0x120504` | `0x10dcc3` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dd51 | `_identUpdate 0x124a28` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dd62 | `_ffscMsg 0x120504` | `0x10dcdf` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dd81 | `_addrdInsertIdentity 0x10dfa0` | `sp-52` | `0x1f3448` | `?` | `0x3` | `0x6` | `?` |
| `call` at 0x10dd93 | `_addrdLocalMsg 0x10e020` | `sp-52` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dda3 | `_addrdBroadcastMsg 0x10dec0` | `sp-52` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10dd38 | `(ret(_identSetRackID) & ret(_identSetRackID)) == 0x0` | `(ret(_identSetRackID) & ret(_identSetRackID)) != 0x0` |
| `je` at 0x10dd5b | `(ret(_identUpdate) & ret(_identUpdate)) == 0x0` | `(ret(_identUpdate) & ret(_identUpdate)) != 0x0` |
| `je` at 0x10dd90 | `[0x1f398c] == 0x0` | `[0x1f398c] != 0x0` |
| `jne` at 0x10dd9d | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) != 0x0` | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) == 0x0` |
| `jne` at 0x10ddaa | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ddbb | `eax` | `?` |

## `_addrdResolveRackIDConflict`

- `_addrdResolveRackIDConflict`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10f040 to 0x10f0f0.
  Established. Calling `_ffscCompareUniqueIDs` presents leading candidates `[sp+4]` and `[sp+8]`
  (`call` at 0x10f050). The merged `_memcpy` call has leading candidates `sp-32`, unknown, and
  `0x1c` (`call` at 0x10f077). A four-byte store saves `ret(_addrdFindUnusedRackID)` at `sp -0x14`
  (`mov` at 0x10f084). The branch takes `ret(_addrdFindUnusedRackID) != 0xffffffff` and falls
  through for equality (`jne` at 0x10f08a). EAX returns `ret(_addrdAssignNewIdentity)` (`ret` at
  0x10f0bb). The selected identity and conflict rule remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10f048 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f04b | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10f084 | store | 4 | `sp -0x14` | `ret(_addrdFindUnusedRackID)` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10f050 | `_ffscCompareUniqueIDs 0x126f28` | `[sp+4]` | `[sp+8]` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10f077 | `_memcpy 0x182918` | `sp-32` | `?` | `0x1c` | `?` | `?` | `?` |
| `call` at 0x10f07f | `_addrdFindUnusedRackID 0x10e6f0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10f094 | `_ffscUniqueIDToString 0x127758` | `sp-32` | `sp-56` | `?` | `?` | `?` | `?` |
| `call` at 0x10f0a2 | `_ffscMsg 0x120504` | `0x10f018` | `sp-56` | `?` | `?` | `?` | `?` |
| `call` at 0x10f0ae | `_addrdAssignNewIdentity 0x10e490` | `sp-32` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x10f05a | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x10f08a | `ret(_addrdFindUnusedRackID) != 0xffffffff` | `ret(_addrdFindUnusedRackID) == 0xffffffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10f0bb | `eax` | `ret(_addrdAssignNewIdentity)` |

## `_addrdStartElectionTimer`

- `_addrdStartElectionTimer`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10f0f0 to 0x10f138.
  Established. The `_wdStart` call has leading candidates `[0x1efd08]` and `(ret(_sysClkRateGet) +
  ret(_sysClkRateGet))`, followed by unknown candidates (`call` at 0x10f108). The result branch
  takes `l flags condition (operands unknown)` and falls through for its negation (`jl` at
  0x10f112). The fall-through exit carries EAX `0x0` (`ret` at 0x10f119), while the taken route
  returns `0xffffffff` (`ret` at 0x10f132). The callback arguments and delay units remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10f102 | load | 4 | `abs 0x1efd08` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10f0fa | `_sysClkRateGet 0x1085a8` | `0x10e610` | `0x0` | `ebp` | `?` | `?` | `?` |
| `call` at 0x10f108 | `_wdStart 0x1778c4` | `[0x1efd08]` | `(ret(_sysClkRateGet) + ret(_sysClkRateGet))` | `?` | `?` | `?` | `?` |
| `call` at 0x10f125 | `_ffscMsg 0x120504` | `0x10f0bc` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jl` at 0x10f112 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10f119 | `eax` | `0x0` |
| `ret` at 0x10f132 | `eax` | `0xffffffff` |

## `_identFindMaster`

- `_identFindMaster`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1244c8 to 0x124558.
  Established. The `_lstFirst` call has leading candidate `0x1d75a0` (`call` at 0x1244d2); its
  branch takes `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` and falls through for zero (`jne` at
  0x1244de). A later `_ffscCompareUniqueIDs` call presents leading candidates `ret(_lstNext)+16` and
  unknown (`call` at 0x124500). The following branch takes `ge flags condition (operands unknown)`
  and falls through for its negation (`jge` at 0x12450a). The master ordering and returned identity
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1244d2 | `_lstFirst 0x192ad0` | `0x1d75a0` | `ebx` | `esi` | `ebp` | `?` | `?` |
| `call` at 0x124500 | `_ffscCompareUniqueIDs 0x126f28` | `ret(_lstNext)+16` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12450f | `_lstNext 0x192b80` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x1244de | `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` | `(ret(_lstFirst) & ret(_lstFirst)) == 0x0` |
| `jge` at 0x12450a | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x12451b | `(ret(_lstNext) & ret(_lstNext)) != 0x0` | `(ret(_lstNext) & ret(_lstNext)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x124528 | `eax` | `?` |

## `_identSetRackID`

- `_identSetRackID`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x124f18 to 0x124f88.
  Established. A four-byte store writes `[sp+4]` at absolute `0x1f3454` (`mov` at 0x124f23). The
  `_nvramWrite` candidates begin `0x2` and `0x1f3454` (`call` at 0x124f30); its branch takes
  `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` and falls through for nonzero (`je` at 0x124f3a).
  After the routes join, a four-byte store sets absolute `0x1efce8` to `0x1` (`mov` at 0x124f49).
  The storage-record and global notification meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x124f20 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x124f23 | store | 4 | `abs 0x1f3454` | `[sp+4]` |
| `mov` at 0x124f49 | store | 4 | `abs 0x1efce8` | `0x1` |
| `test` at 0x124f53 | load | 1 | `abs 0x1df501` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x124f30 | `_nvramWrite 0x128b18` | `0x2` | `0x1f3454` | `?` | `?` | `?` | `?` |
| `call` at 0x124f41 | `_ffscMsg 0x120504` | `0x124ec5` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x124f61 | `_ffscRackIDToString 0x1274b8` | `[sp+4]` | `sp-16` | `?` | `?` | `?` | `?` |
| `call` at 0x124f6f | `_ffscMsg 0x120504` | `0x124ef7` | `sp-16` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x124f3a | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` |
| `je` at 0x124f5a | `(([0x1df501]:1 & 0xff) & 0x1) == 0x0` | `(([0x1df501]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x124f7e | `eax` | `0x0` |

## `_identFindByRackID`

- `_identFindByRackID`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x124368 to 0x1243b8.
  Established. The entry branch takes `[sp+4] != 0xffffffff` and falls through for equality (`jne`
  at 0x124372). The taken route calls `_lstFirst` with leading candidate `0x1d75a0` (`call` at
  0x12438d). The comparison branch takes `Z == 1` and falls through for `not (Z == 1)` (`je` at
  0x12439d). The merged EAX return value is unknown (`ret` at 0x1243b6). The matched rack and record
  address remain unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12436c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x124398 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1243b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12438d | `_lstFirst 0x192ad0` | `0x1d75a0` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x1243a0 | `_lstNext 0x192b80` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x124372 | `[sp+4] != 0xffffffff` | `[sp+4] == 0xffffffff` |
| `je` at 0x12439d | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x1243ac | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1243b6 | `eax` | `?` |

## `_identFindConflictingRackID`

- `_identFindConflictingRackID`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x124448 to 0x1244c8.
  Established. The first branch takes `([sp+4] & [sp+4]) == 0x0` and falls through for nonzero (`je`
  at 0x124453). A later branch takes `[[sp+4]+12] != 0xffffffff` and falls through for equality
  (`jne` at 0x12445b). The taken route includes `_ffscCompareUniqueIDs` with leading candidates
  unknown and `[sp+4]` (`call` at 0x12448d); its branch takes `(ret(_ffscCompareUniqueIDs) &
  ret(_ffscCompareUniqueIDs)) == 0x0` and falls through for nonzero (`je` at 0x124497). The conflict
  match and selected record remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12444e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x124455 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x124499 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12449c | load | 4 | `[sp+4] +0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12447d | `_lstFirst 0x192ad0` | `0x1d75a0` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x12448d | `_ffscCompareUniqueIDs 0x126f28` | `?` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x1244a4 | `_lstNext 0x192b80` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x124453 | `([sp+4] & [sp+4]) == 0x0` | `([sp+4] & [sp+4]) != 0x0` |
| `jne` at 0x12445b | `[[sp+4]+12] != 0xffffffff` | `[[sp+4]+12] == 0xffffffff` |
| `je` at 0x124497 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` |
| `je` at 0x1244a1 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x1244b0 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1244bd | `eax` | `?` |
