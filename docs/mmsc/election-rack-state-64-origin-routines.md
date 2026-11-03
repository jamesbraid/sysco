# MMSC election and rack state in the 6.4-origin image

Read from `6.4-origin/usr/cpu/firmware/mmscfw.bin`.
Image SHA-256: `b766e559f86078526916d8bd98bbb8079294ad774cce9177928873153c6fb09b`.
The x86 a.out link address is `0x108000`. The [load map](load-map.md) explains segment mapping.
Analysed spans have exclusive ends inferred from the next text symbol.
Some include literals and padding decoded linearly after returns. Only sites reached
by known control-flow edges contribute effects to the tables.
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

[Fact sheets and complete listings](disasm/election-rack-state-64-origin-facts.txt) include all tool sections.

## `_addrdElectMaster`

- `_addrdElectMaster`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10d740 to 0x10d940.
  Established. The four-byte store at `sp -0x10` retains `ret(_semBCreate)` (`mov` at 0x10d753). The
  `_lstAdd` site presents `0x1dab30` and `sp-24` as its leading candidates (`call` at 0x10d796). The
  `_addrdElectionStart` result branch is taken when `(ret(_addrdElectionStart) &
  ret(_addrdElectionStart)) == 0x0` (`je` at 0x10d7b3). A later `_semTake` site presents `[sp-16]`
  and `0xffffffff` (`call` at 0x10d7e6). List and election roles are not derived from these call
  names.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10d753 | store | 4 | `sp -0x10` | `ret(_semBCreate)` |
| `mov` at 0x10d76d | load | 4 | `abs 0x1ee84c` | `-` |
| `mov` at 0x10d79e | load | 4 | `abs 0x1ee84c` | `-` |
| `mov` at 0x10d7c8 | load | 4 | `abs 0x1ee84c` | `-` |
| `mov` at 0x10d7e2 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x10d805 | load | 4 | `abs 0x1ee84c` | `-` |
| `mov` at 0x10d83e | load | 4 | `abs 0x1ee84c` | `-` |
| `mov` at 0x10d84c | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x10d860 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x10d86b | load | 4 | `sp -0x1c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10d74b | `_semBCreate 0x1753d8` | `0x0` | `0x0` | `ebx` | `?` | `?` | `?` |
| `call` at 0x10d75f | `_clock_gettime 0x17981c` | `0x0` | `sp-12` | `?` | `?` | `?` | `ret(_semBCreate)` |
| `call` at 0x10d767 | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d773 | `_semTake 0x1712fc` | `[0x1ee84c]` | `ret(_sysClkRateGet)` | `?` | `?` | `?` | `?` |
| `call` at 0x10d784 | `_ffscMsg 0x11ebd0` | `0x10d6a0` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d796 | `_lstAdd 0x18dde8` | `0x1dab30` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x10d7a4 | `_semGive 0x17127c` | `[0x1ee84c]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d7ac | `_addrdElectionStart 0x10e340` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d7ba | `_ffscMsg 0x11ebd0` | `0x10d6d0` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d7c2 | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d7ce | `_semTake 0x1712fc` | `[0x1ee84c]` | `ret(_sysClkRateGet)` | `?` | `?` | `?` | `?` |
| `call` at 0x10d7e6 | `_semTake 0x1712fc` | `[sp-16]` | `0xffffffff` | `?` | `?` | `?` | `?` |
| `call` at 0x10d7f7 | `_ffscMsg 0x11ebd0` | `0x10d715` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d7ff | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d80b | `_semTake 0x1712fc` | `[0x1ee84c]` | `ret(_sysClkRateGet)` | `?` | `?` | `?` | `?` |
| `call` at 0x10d81c | `_ffscMsg 0x11ebd0` | `0x10d6e9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d836 | `_lstDelete 0x18de78` | `0x1dab30` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x10d844 | `_semGive 0x17127c` | `[0x1ee84c]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d850 | `_semDelete 0x18e23c` | `[sp-16]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d864 | `_semDelete 0x18e23c` | `[sp-16]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x10d77d | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x10d7b3 | `(ret(_addrdElectionStart) & ret(_addrdElectionStart)) == 0x0` | `(ret(_addrdElectionStart) & ret(_addrdElectionStart)) != 0x0` |
| `jl` at 0x10d7d8 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10d7f0 | `(ret(_semTake) & ret(_semTake)) == 0x0` | `(ret(_semTake) & ret(_semTake)) != 0x0` |
| `jge` at 0x10d815 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10d871 | `eax` | `?` |

## `_addrdElectionEnd`

- `_addrdElectionEnd`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e300 to 0x10e340.
  Established. The two explicit four-byte stores place `0x3` at `sp -0x34` (`mov` at 0x10e306) and
  `0x5` at `sp -0x30` (`mov` at 0x10e30d). The `_addrdLocalMsg` site has leading candidates `sp-52`,
  `0x3`, and `0x5` (`call` at 0x10e318); its result branch is taken when `(ret(_addrdLocalMsg) &
  ret(_addrdLocalMsg)) != 0x0` (`jne` at 0x10e31f). The exits retain distinct literal values `0x0`
  (`ret` at 0x10e326) and `0xffffffff` (`ret` at 0x10e338), without establishing message meanings.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e306 | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10e30d | store | 4 | `sp -0x30` | `0x5` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e318 | `_addrdLocalMsg 0x10ddd0` | `sp-52` | `0x3` | `0x5` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e31f | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) != 0x0` | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e326 | `eax` | `0x0` |
| `ret` at 0x10e338 | `eax` | `0xffffffff` |

## `_addrdElectionStart`

- `_addrdElectionStart`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e340 to 0x10e380.
  Established. Stack stores write four-byte values `0x3` at `sp -0x34` (`mov` at 0x10e346) and `0x4`
  at `sp -0x30` (`mov` at 0x10e34d). The `_addrdLocalMsg` call presents `sp-52`, `0x3`, and `0x4` as
  its first three candidates (`call` at 0x10e358); its branch falls through when
  `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) == 0x0` (`jne` at 0x10e35f), reaching the
  literal-zero exit (`ret` at 0x10e366). The separate exit carries `0xffffffff` (`ret` at 0x10e378).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e346 | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10e34d | store | 4 | `sp -0x30` | `0x4` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e358 | `_addrdLocalMsg 0x10ddd0` | `sp-52` | `0x3` | `0x4` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e35f | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) != 0x0` | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e366 | `eax` | `0x0` |
| `ret` at 0x10e378 | `eax` | `0xffffffff` |

## `_addrdElectionTimerPop`

- `_addrdElectionTimerPop`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e380 to 0x10e3b0.
  Established. The sole reached call names `_addrdElectionEnd` and retains candidate zero `ebp`
  (`call` at 0x10e383). The subsequent exit reports `0x0` in `eax` (`ret` at 0x10e38d). The later
  linear decodes contribute no executable I/O effects. A timer role remains unknown from this body
  alone.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e383 | `_addrdElectionEnd 0x10e300` | `ebp` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e38d | `eax` | `0x0` |

## `_addrdFindUnusedRackID`

- `_addrdFindUnusedRackID`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e460 to 0x10e4f0.
  Established. The `_identFindByRackID` site retains unknown candidates (`call` at 0x10e46d); its
  result branch is taken when `(ret(_identFindByRackID) & ret(_identFindByRackID)) != 0x0` (`jne` at
  0x10e477). The backward branch is taken when `? <= 0x7 (unsigned, 32-bit)` (`jbe` at 0x10e484).
  The final four-byte load uses `sp -0x8` (`mov` at 0x10e48b). Unknown tracking does not establish
  which IDs are examined.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e48b | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e46d | `_identFindByRackID 0x1222f8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e477 | `(ret(_identFindByRackID) & ret(_identFindByRackID)) != 0x0` | `(ret(_identFindByRackID) & ret(_identFindByRackID)) == 0x0` |
| `jbe` at 0x10e484 | `? <= 0x7 (unsigned, 32-bit)` | `? > 0x7 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e491 | `eax` | `?` |

## `_addrdProcessElectionEndMsg`

- `_addrdProcessElectionEndMsg`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e600 to 0x10e7a0.
  Established. The `_identFindMaster` result is stored as four bytes at absolute `0x1eb6cc` (`mov`
  at 0x10e645). Separate four-byte stores write `0x1` (`mov` at 0x10e660) and `0x0` (`mov` at
  0x10e67d) to absolute `0x1eea94`. Another writes `0x0` to absolute `0x1da6e4` (`mov` at 0x10e744).
  The later branch is taken when `[0x1eea94] == 0x0` (`je` at 0x10e755). These addresses do not by
  themselves identify master or election fields.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `test` at 0x10e606 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x10e645 | store | 4 | `abs 0x1eb6cc` | `ret(_identFindMaster)` |
| `mov` at 0x10e660 | store | 4 | `abs 0x1eea94` | `0x1` |
| `mov` at 0x10e67d | store | 4 | `abs 0x1eea94` | `0x0` |
| `mov` at 0x10e68d | load | 4 | `abs 0x1ee84c` | `-` |
| `test` at 0x10e6f4 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x10e724 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10e736 | load | 4 | `abs 0x1ee84c` | `-` |
| `mov` at 0x10e744 | store | 4 | `abs 0x1da6e4` | `0x0` |
| `cmp` at 0x10e74e | load | 4 | `abs 0x1eea94` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e614 | `_ffscMsg 0x11ebd0` | `0x10e571` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x10e621 | `_identCleanOldEntries 0x121fd8` | `0x1eae10` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e640 | `_identFindMaster 0x122458` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e654 | `_ffscCompareUniqueIDs 0x1253bc` | `0x1ee558` | `ret(_identFindMaster)` | `?` | `?` | `?` | `?` |
| `call` at 0x10e675 | `_ffscMsg 0x11ebd0` | `0x10e58d` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e687 | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e693 | `_semTake 0x1712fc` | `[0x1ee84c]` | `ret(_sysClkRateGet)` | `?` | `?` | `?` | `?` |
| `call` at 0x10e6a4 | `_ffscMsg 0x11ebd0` | `0x10d6a0` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e6c5 | `_lstFirst 0x18df58` | `0x1dab30` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e6dd | `_ffscCompareTimestamps 0x12536c` | `?` | `0x1eae10` | `?` | `?` | `?` | `?` |
| `call` at 0x10e6ea | `_lstNext 0x18e008` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e700 | `_ffscTimestampToString 0x125a9c` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10e70e | `_ffscMsg 0x11ebd0` | `0x10e5a8` | `ret(_ffscTimestampToString)` | `?` | `?` | `?` | `?` |
| `call` at 0x10e71c | `_lstDelete 0x18de78` | `0x1dab30` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e728 | `_semGive 0x17127c` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e73c | `_semGive 0x17127c` | `[0x1ee84c]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e757 | `_addrdResolveConflicts 0x10ec90` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e765 | `_ffscMsg 0x11ebd0` | `0x10e5d5` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10e60d | `(([0x1da698]:1 & 0xff) & 0x1) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x1) != 0x0` |
| `je` at 0x10e62b | `(ret(_identCleanOldEntries) & ret(_identCleanOldEntries)) == 0x0` | `(ret(_identCleanOldEntries) & ret(_identCleanOldEntries)) != 0x0` |
| `je` at 0x10e64c | `(ret(_identFindMaster) & ret(_identFindMaster)) == 0x0` | `(ret(_identFindMaster) & ret(_identFindMaster)) != 0x0` |
| `jne` at 0x10e65e | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `jge` at 0x10e69d | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x10e6d1 | `(ret(_lstFirst) & ret(_lstFirst)) == 0x0` | `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` |
| `jg` at 0x10e6e7 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `je` at 0x10e6fb | `(([0x1da698]:1 & 0xff) & 0x1) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x10e734 | `(ret(_lstNext) & ret(_lstNext)) != 0x0` | `(ret(_lstNext) & ret(_lstNext)) == 0x0` |
| `je` at 0x10e755 | `[0x1eea94] == 0x0` | `[0x1eea94] != 0x0` |
| `je` at 0x10e75e | `(ret(_addrdResolveConflicts) & ret(_addrdResolveConflicts)) == 0x0` | `(ret(_addrdResolveConflicts) & ret(_addrdResolveConflicts)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e775 | `eax` | `?` |

## `_addrdProcessElectionStartMsg`

- `_addrdProcessElectionStartMsg`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e7a0 to 0x10e8a0.
  Established. The `_clock_gettime` site presents leading candidates `0x0` and `0x1eae10` (`call` at
  0x10e7ae). Four-byte stack stores write `0x3` at `sp -0x34` (`mov` at 0x10e7dc) and `0x2` at `sp
  -0x30` (`mov` at 0x10e7e3). The `_addrdBroadcastMsg` result branch falls through when
  `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` (`jne` at 0x10e806), reaching
  `_addrdStartElectionTimer` (`call` at 0x10e808). A four-byte store at absolute `0x1da6e4` writes
  `0x1` (`mov` at 0x10e811).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `test` at 0x10e7b6 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x10e7dc | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10e7e3 | store | 4 | `sp -0x30` | `0x2` |
| `mov` at 0x10e811 | store | 4 | `abs 0x1da6e4` | `0x1` |
| `mov` at 0x10e825 | load | 4 | `sp -0x38` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e7ae | `_clock_gettime 0x17981c` | `0x0` | `0x1eae10` | `ebx` | `?` | `?` | `?` |
| `call` at 0x10e7c6 | `_ffscTimestampToString 0x125a9c` | `0x1eae10` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10e7d4 | `_ffscMsg 0x11ebd0` | `0x10e776` | `ret(_ffscTimestampToString)` | `?` | `?` | `?` | `?` |
| `call` at 0x10e7f3 | `_addrdInsertIdentity 0x10dd50` | `sp-52` | `0x1ee558` | `?` | `0x3` | `0x2` | `?` |
| `call` at 0x10e7fc | `_addrdBroadcastMsg 0x10dc70` | `sp-52` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e808 | `_addrdStartElectionTimer 0x10ee60` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10e7bd | `(([0x1da698]:1 & 0xff) & 0x1) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x10e806 | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` |
| `jne` at 0x10e80f | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) != 0x0` | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e82b | `eax` | `?` |

## `_addrdProcessRequestRackIDMsg`

- `_addrdProcessRequestRackIDMsg`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10eb40 to 0x10ec90.
  Established. The `_addrdExtractIdentity` call has leading candidates `[sp+4]` and `sp-32` (`call`
  at 0x10eb51). The `_memcpy` site presents `sp-60`, `ret(_identFindConflictingRackID)`, and `0x1c`
  (`call` at 0x10ebd5). The `_addrdFindUnusedRackID` result is stored as four bytes at `sp -0x30`
  (`mov` at 0x10ebe2); its branch falls through when `ret(_addrdFindUnusedRackID) == 0xffffffff`
  (`jne` at 0x10ebe8). The candidate `0x1c` does not establish a record size.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10eb49 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x10eb6c | load | 4 | `abs 0x1da6e4` | `-` |
| `test` at 0x10eb82 | load | 1 | `abs 0x1da698` | `-` |
| `cmp` at 0x10ebb3 | load | 4 | `abs 0x1eea94` | `-` |
| `mov` at 0x10ebe2 | store | 4 | `sp -0x30` | `ret(_addrdFindUnusedRackID)` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10eb51 | `_addrdExtractIdentity 0x10dcf0` | `[sp+4]` | `sp-32` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x10eb5a | `_identUpdate 0x1229b8` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10eb75 | `_addrdStartElectionTimer 0x10ee60` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10eb9f | `_ffscMsg 0x11ebd0` | `0x10eaee` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ebab | `_identPrint 0x122ae8` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ebc0 | `_identFindConflictingRackID 0x1223d8` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ebd5 | `_memcpy 0x17dda0` | `sp-60` | `ret(_identFindConflictingRackID)` | `0x1c` | `?` | `?` | `?` |
| `call` at 0x10ebdd | `_addrdFindUnusedRackID 0x10e460` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ebef | `_ffscUniqueIDToString 0x125bec` | `ret(_identFindConflictingRackID)` | `sp-84` | `?` | `?` | `?` | `?` |
| `call` at 0x10ebfd | `_ffscMsg 0x11ebd0` | `0x10eb0b` | `sp-84` | `?` | `?` | `?` | `?` |
| `call` at 0x10ec06 | `_addrdAssignNewIdentity 0x10e200` | `sp-60` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jl` at 0x10eb66 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10eb73 | `[0x1da6e4] == 0x0` | `[0x1da6e4] != 0x0` |
| `jne` at 0x10eb7c | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) != 0x0` | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) == 0x0` |
| `je` at 0x10eb89 | `(([0x1da698]:1 & 0xff) & 0x1) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x10eb92 | `(ret(_identUpdate) & ret(_identUpdate)) != 0x0` | `(ret(_identUpdate) & ret(_identUpdate)) == 0x0` |
| `je` at 0x10ebba | `[0x1eea94] == 0x0` | `[0x1eea94] != 0x0` |
| `je` at 0x10ebcc | `(ret(_identFindConflictingRackID) & ret(_identFindConflictingRackID)) == 0x0` | `(ret(_identFindConflictingRackID) & ret(_identFindConflictingRackID)) != 0x0` |
| `jne` at 0x10ebe8 | `ret(_addrdFindUnusedRackID) != 0xffffffff` | `ret(_addrdFindUnusedRackID) == 0xffffffff` |
| `je` at 0x10ec0d | `(ret(_addrdAssignNewIdentity) & ret(_addrdAssignNewIdentity)) == 0x0` | `(ret(_addrdAssignNewIdentity) & ret(_addrdAssignNewIdentity)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ec2b | `eax` | `?` |

## `_addrdRequestRackID`

- `_addrdRequestRackID`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10dad0 to 0x10db70.
  Established. The `_identSetRackID` site has candidate zero `[[sp+4]+12]` (`call` at 0x10dade).
  Later four-byte stores write `0x3` at `sp -0x34` (`mov` at 0x10db1a) and `0x6` at `sp -0x30`
  (`mov` at 0x10db21). The branch is taken when `[0x1eea94] == 0x0` (`je` at 0x10db40), reaching
  `_addrdBroadcastMsg` with candidate zero `sp-52` (`call` at 0x10db53). No request-token meaning is
  assigned from these literals.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10dad7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10dada | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10db1a | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10db21 | store | 4 | `sp -0x30` | `0x6` |
| `cmp` at 0x10db39 | load | 4 | `abs 0x1eea94` | `-` |
| `mov` at 0x10db65 | load | 4 | `sp -0x38` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10dade | `_identSetRackID 0x122ea8` | `[[sp+4]+12]` | `ebx` | `?` | `?` | `?` | `?` |
| `call` at 0x10daef | `_ffscMsg 0x11ebd0` | `0x10da73` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10db01 | `_identUpdate 0x1229b8` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10db12 | `_ffscMsg 0x11ebd0` | `0x10da8f` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10db31 | `_addrdInsertIdentity 0x10dd50` | `sp-52` | `0x1ee558` | `?` | `0x3` | `0x6` | `?` |
| `call` at 0x10db43 | `_addrdLocalMsg 0x10ddd0` | `sp-52` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10db53 | `_addrdBroadcastMsg 0x10dc70` | `sp-52` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10dae8 | `(ret(_identSetRackID) & ret(_identSetRackID)) == 0x0` | `(ret(_identSetRackID) & ret(_identSetRackID)) != 0x0` |
| `je` at 0x10db0b | `(ret(_identUpdate) & ret(_identUpdate)) == 0x0` | `(ret(_identUpdate) & ret(_identUpdate)) != 0x0` |
| `je` at 0x10db40 | `[0x1eea94] == 0x0` | `[0x1eea94] != 0x0` |
| `jne` at 0x10db4d | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) != 0x0` | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) == 0x0` |
| `jne` at 0x10db5a | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10db6b | `eax` | `?` |

## `_addrdResolveRackIDConflict`

- `_addrdResolveRackIDConflict`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10edb0 to 0x10ee60.
  Established. The `_memcpy` site presents `sp-32`, unknown, and `0x1c` as its first three
  candidates (`call` at 0x10ede7). A four-byte store retains `ret(_addrdFindUnusedRackID)` at `sp
  -0x14` (`mov` at 0x10edf4); its comparison branch is taken when `ret(_addrdFindUnusedRackID) !=
  0xffffffff` (`jne` at 0x10edfa). The exit reports `ret(_addrdAssignNewIdentity)` in `eax` (`ret`
  at 0x10ee2b). The assignment outcome remains unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10edb8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10edbb | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10edf4 | store | 4 | `sp -0x14` | `ret(_addrdFindUnusedRackID)` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10edc0 | `_ffscCompareUniqueIDs 0x1253bc` | `[sp+4]` | `[sp+8]` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10ede7 | `_memcpy 0x17dda0` | `sp-32` | `?` | `0x1c` | `?` | `?` | `?` |
| `call` at 0x10edef | `_addrdFindUnusedRackID 0x10e460` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ee04 | `_ffscUniqueIDToString 0x125bec` | `sp-32` | `sp-56` | `?` | `?` | `?` | `?` |
| `call` at 0x10ee12 | `_ffscMsg 0x11ebd0` | `0x10ed88` | `sp-56` | `?` | `?` | `?` | `?` |
| `call` at 0x10ee1e | `_addrdAssignNewIdentity 0x10e200` | `sp-32` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x10edca | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x10edfa | `ret(_addrdFindUnusedRackID) != 0xffffffff` | `ret(_addrdFindUnusedRackID) == 0xffffffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ee2b | `eax` | `ret(_addrdAssignNewIdentity)` |

## `_addrdStartElectionTimer`

- `_addrdStartElectionTimer`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ee60 to 0x10eea8.
  Established. A four-byte load reads absolute `0x1eae18` (`mov` at 0x10ee72). The `_wdStart` site
  presents leading candidates `[0x1eae18]` and `(ret(_sysClkRateGet) + ret(_sysClkRateGet))`,
  followed by unknown slots (`call` at 0x10ee78). The two exits carry `0x0` (`ret` at 0x10ee89) and
  `0xffffffff` (`ret` at 0x10eea2). Neither a callback argument nor a delay unit is established by
  these candidates.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10ee72 | load | 4 | `abs 0x1eae18` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ee6a | `_sysClkRateGet 0x1085a8` | `0x10e380` | `0x0` | `ebp` | `?` | `?` | `?` |
| `call` at 0x10ee78 | `_wdStart 0x1739a4` | `[0x1eae18]` | `(ret(_sysClkRateGet) + ret(_sysClkRateGet))` | `?` | `?` | `?` | `?` |
| `call` at 0x10ee95 | `_ffscMsg 0x11ebd0` | `0x10ee2c` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jl` at 0x10ee82 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ee89 | `eax` | `0x0` |
| `ret` at 0x10eea2 | `eax` | `0xffffffff` |

## `_identFindMaster`

- `_identFindMaster`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122458 to 0x1224e8.
  Established. The `_lstFirst` call has candidate zero `0x1d28e8` (`call` at 0x122462); its result
  branch is taken when `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` (`jne` at 0x12246e). The
  `_ffscCompareUniqueIDs` site has candidate zero `ret(_lstNext)+16` and unknown remaining slots
  (`call` at 0x122490). The backward branch is taken when `(ret(_lstNext) & ret(_lstNext)) != 0x0`
  (`jne` at 0x1224ab). A master-selection rule is not inferred from the names.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122462 | `_lstFirst 0x18df58` | `0x1d28e8` | `ebx` | `esi` | `ebp` | `?` | `?` |
| `call` at 0x122490 | `_ffscCompareUniqueIDs 0x1253bc` | `ret(_lstNext)+16` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12249f | `_lstNext 0x18e008` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x12246e | `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` | `(ret(_lstFirst) & ret(_lstFirst)) == 0x0` |
| `jge` at 0x12249a | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x1224ab | `(ret(_lstNext) & ret(_lstNext)) != 0x0` | `(ret(_lstNext) & ret(_lstNext)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1224b8 | `eax` | `?` |

## `_identSetRackID`

- `_identSetRackID`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122ea8 to 0x122f18.
  Established. The four-byte store at absolute `0x1ee564` writes `[sp+4]` (`mov` at 0x122eb3). The
  `_nvramWrite` site presents `0x2` and `0x1ee564` as its first two candidates (`call` at 0x122ec0).
  Another four-byte store writes `0x1` at absolute `0x1eadf8` (`mov` at 0x122ed9). The common exit
  reports `0x0` in `eax` (`ret` at 0x122f0e). That literal does not prove persistence or a
  successful rack-ID update.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x122eb0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122eb3 | store | 4 | `abs 0x1ee564` | `[sp+4]` |
| `mov` at 0x122ed9 | store | 4 | `abs 0x1eadf8` | `0x1` |
| `test` at 0x122ee3 | load | 1 | `abs 0x1da699` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122ec0 | `_nvramWrite 0x126e80` | `0x2` | `0x1ee564` | `?` | `?` | `?` | `?` |
| `call` at 0x122ed1 | `_ffscMsg 0x11ebd0` | `0x122e55` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122ef1 | `_ffscRackIDToString 0x12594c` | `[sp+4]` | `sp-16` | `?` | `?` | `?` | `?` |
| `call` at 0x122eff | `_ffscMsg 0x11ebd0` | `0x122e87` | `sp-16` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x122eca | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` |
| `je` at 0x122eea | `(([0x1da699]:1 & 0xff) & 0x1) == 0x0` | `(([0x1da699]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122f0e | `eax` | `0x0` |

## `_identFindByRackID`

- `_identFindByRackID`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1222f8 to 0x122348.
  Established. The opening comparison branch is taken when `[sp+4] != 0xffffffff` (`jne` at
  0x122302). The `_lstFirst` site presents candidate zero `0x1d28e8` (`call` at 0x12231d), while the
  `_lstNext` site retains unknown candidates (`call` at 0x122330). A four-byte load has an
  unresolved target (`mov` at 0x122328), and the exit value is unknown (`ret` at 0x122346). A
  successful lookup is not established by the call names.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1222fc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122328 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122340 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12231d | `_lstFirst 0x18df58` | `0x1d28e8` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x122330 | `_lstNext 0x18e008` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x122302 | `[sp+4] != 0xffffffff` | `[sp+4] == 0xffffffff` |
| `je` at 0x12232d | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x12233c | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122346 | `eax` | `?` |

## `_identFindConflictingRackID`

- `_identFindConflictingRackID`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1223d8 to 0x122458.
  Established. The four-byte load through `[sp+4] +0xc` feeds a branch taken when `[[sp+4]+12] !=
  0xffffffff` (`mov` at 0x1223e5, `jne` at 0x1223eb). The `_ffscCompareUniqueIDs` site has unknown
  candidate zero and candidate one `[sp+4]` (`call` at 0x12241d); its result branch is taken when
  `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` (`je` at 0x122427). The later
  four-byte load at an unresolved target remains unknown (`mov` at 0x122429).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1223de | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1223e5 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x122429 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12242c | load | 4 | `[sp+4] +0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12240d | `_lstFirst 0x18df58` | `0x1d28e8` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x12241d | `_ffscCompareUniqueIDs 0x1253bc` | `?` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x122434 | `_lstNext 0x18e008` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1223e3 | `([sp+4] & [sp+4]) == 0x0` | `([sp+4] & [sp+4]) != 0x0` |
| `jne` at 0x1223eb | `[[sp+4]+12] != 0xffffffff` | `[[sp+4]+12] == 0xffffffff` |
| `je` at 0x122427 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` |
| `je` at 0x122431 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x122440 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12244d | `eax` | `?` |
