# MMSC election and rack-identity routines in IRIX 6.5.2

Read from `6.5.2/usr/cpu/firmware/mmscfw.bin` (IRIX 6.5.2 carrier).
Image SHA-256: `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
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

[Fact sheets and full listings](disasm/election-6.5.2-facts.txt) include all tool sections.

## `_addrdElectMaster`

- `_addrdElectMaster`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10d7c0 to 0x10d9c0.
  Established. The initial `_semBCreate` and `_clock_gettime` calls have first candidates zero, zero
  and zero, `sp-12` respectively (`call` at 0x10d7cb, `call` at 0x10d7df). Later `_lstAdd` and
  `_lstDelete` candidates begin 0x1dd2c8 and `sp-24` (`call` at 0x10d816, `call` at 0x10d8b6).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10d7d3 | store | 4 | `sp -0x10` | `ret(_semBCreate)` |
| `mov` at 0x10d7ed | load | 4 | `abs 0x1f0ff4` | `-` |
| `mov` at 0x10d81e | load | 4 | `abs 0x1f0ff4` | `-` |
| `mov` at 0x10d848 | load | 4 | `abs 0x1f0ff4` | `-` |
| `mov` at 0x10d862 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x10d885 | load | 4 | `abs 0x1f0ff4` | `-` |
| `mov` at 0x10d8be | load | 4 | `abs 0x1f0ff4` | `-` |
| `mov` at 0x10d8cc | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x10d8e0 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x10d8eb | load | 4 | `sp -0x1c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10d7cb | `_semBCreate 0x177888` | `0x0` | `0x0` | `ebx` | `?` | `?` | `?` |
| `call` at 0x10d7df | `_clock_gettime 0x17bccc` | `0x0` | `sp-12` | `?` | `?` | `?` | `ret(_semBCreate)` |
| `call` at 0x10d7e7 | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d7f3 | `_semTake 0x172b54` | `[0x1f0ff4]` | `ret(_sysClkRateGet)` | `?` | `?` | `?` | `?` |
| `call` at 0x10d804 | `_ffscMsg 0x11f0d0` | `0x10d720` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d816 | `_lstAdd 0x190298` | `0x1dd2c8` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x10d824 | `_semGive 0x172ad4` | `[0x1f0ff4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d82c | `_addrdElectionStart 0x10e3c0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d83a | `_ffscMsg 0x11f0d0` | `0x10d750` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d842 | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d84e | `_semTake 0x172b54` | `[0x1f0ff4]` | `ret(_sysClkRateGet)` | `?` | `?` | `?` | `?` |
| `call` at 0x10d866 | `_semTake 0x172b54` | `[sp-16]` | `0xffffffff` | `?` | `?` | `?` | `?` |
| `call` at 0x10d877 | `_ffscMsg 0x11f0d0` | `0x10d795` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d87f | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d88b | `_semTake 0x172b54` | `[0x1f0ff4]` | `ret(_sysClkRateGet)` | `?` | `?` | `?` | `?` |
| `call` at 0x10d89c | `_ffscMsg 0x11f0d0` | `0x10d769` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d8b6 | `_lstDelete 0x190328` | `0x1dd2c8` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x10d8c4 | `_semGive 0x172ad4` | `[0x1f0ff4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d8d0 | `_semDelete 0x1906ec` | `[sp-16]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d8e4 | `_semDelete 0x1906ec` | `[sp-16]` | `?` | `?` | `?` | `?` | `?` |

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

## `_addrdElectionStart`

- `_addrdElectionStart`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e3c0 to 0x10e400.
  Established. Two explicit four-byte stores place 3 at `sp -0x34` and 4 at `sp -0x30` (`mov` at
  0x10e3c6, `mov` at 0x10e3cd). `_addrdLocalMsg` receives first candidate `sp-52`, and its nonzero
  result takes `jne` to the all-ones exit, while zero falls through to the zero exit (`call`
  at 0x10e3d8, `jne` at 0x10e3df, `ret` at 0x10e3f8, `ret` at 0x10e3e6).

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

## `_addrdElectionEnd`

- `_addrdElectionEnd`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e380 to 0x10e3c0.
  Established. The local four-byte store sites carry 3 at `sp -0x34` and 5 at `sp -0x30` (`mov` at
  0x10e386, `mov` at 0x10e38d). The following call to `_addrdLocalMsg` has candidates beginning
  `sp-52`, 3 and 5 (`call` at 0x10e398).

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

## `_addrdElectionTimerPop`

- `_addrdElectionTimerPop`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e400 to 0x10e430.
  Established. Calls `_addrdElectionEnd` before returning EAX zero (`call` at 0x10e403, `ret` at
  0x10e40d). The zero value is assigned after that call (`xor` at 0x10e408).

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

## `_addrdStartElectionTimer`

- `_addrdStartElectionTimer`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10eee0 to 0x10ef28.
  Established. The four-byte read at 0x1ed5c0 supplies the first `_wdStart` candidate, whose second
  candidate is `(ret(_sysClkRateGet) + ret(_sysClkRateGet))` (`mov` at 0x10eef2, `call` at
  0x10eef8). The following `jl` has unresolved condition operands (`jl` at 0x10ef02).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10eef2 | load | 4 | `abs 0x1ed5c0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10eeea | `_sysClkRateGet 0x1085a8` | `0x10e400` | `0x0` | `ebp` | `?` | `?` | `?` |
| `call` at 0x10eef8 | `_wdStart 0x1751fc` | `[0x1ed5c0]` | `(ret(_sysClkRateGet) + ret(_sysClkRateGet))` | `?` | `?` | `?` | `?` |
| `call` at 0x10ef15 | `_ffscMsg 0x11f0d0` | `0x10eeac` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jl` at 0x10ef02 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ef09 | `eax` | `0x0` |
| `ret` at 0x10ef22 | `eax` | `0xffffffff` |

## `_addrdRequestRackID`

- `_addrdRequestRackID`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10db50 to 0x10dbf0.
  Established. The `_identSetRackID` call receives first candidate `[[sp+4]+12]` (`call` at
  0x10db5e). The explicit local stores later place four-byte values 3 and 6 at `sp -0x34` and `sp
  -0x30` (`mov` at 0x10db9a, `mov` at 0x10dba1). `je` bypasses `_addrdLocalMsg` when `[0x1f1244] ==
  0x0` (`je` at 0x10dbc0, `call` at 0x10dbc3).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10db57 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10db5a | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10db9a | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10dba1 | store | 4 | `sp -0x30` | `0x6` |
| `cmp` at 0x10dbb9 | load | 4 | `abs 0x1f1244` | `-` |
| `mov` at 0x10dbe5 | load | 4 | `sp -0x38` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10db5e | `_identSetRackID 0x123688` | `[[sp+4]+12]` | `ebx` | `?` | `?` | `?` | `?` |
| `call` at 0x10db6f | `_ffscMsg 0x11f0d0` | `0x10daf3` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10db81 | `_identUpdate 0x123198` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10db92 | `_ffscMsg 0x11f0d0` | `0x10db0f` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dbb1 | `_addrdInsertIdentity 0x10ddd0` | `sp-52` | `0x1f0d00` | `?` | `0x3` | `0x6` | `?` |
| `call` at 0x10dbc3 | `_addrdLocalMsg 0x10de50` | `sp-52` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dbd3 | `_addrdBroadcastMsg 0x10dcf0` | `sp-52` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10db68 | `(ret(_identSetRackID) & ret(_identSetRackID)) == 0x0` | `(ret(_identSetRackID) & ret(_identSetRackID)) != 0x0` |
| `je` at 0x10db8b | `(ret(_identUpdate) & ret(_identUpdate)) == 0x0` | `(ret(_identUpdate) & ret(_identUpdate)) != 0x0` |
| `je` at 0x10dbc0 | `[0x1f1244] == 0x0` | `[0x1f1244] != 0x0` |
| `jne` at 0x10dbcd | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) != 0x0` | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) == 0x0` |
| `jne` at 0x10dbda | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10dbeb | `eax` | `?` |

## `_addrdProcessElectionStartMsg`

- `_addrdProcessElectionStartMsg`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e820 to 0x10e920.
  Established. The `_clock_gettime` candidates begin zero and 0x1ed5b8 (`call` at 0x10e82e). Later
  four-byte stores place 3 at `sp -0x34`, 2 at `sp -0x30` and 1 at absolute 0x1dce14 at distinct
  sites (`mov` at 0x10e85c, `mov` at 0x10e863, `mov` at 0x10e891).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `test` at 0x10e836 | load | 1 | `abs 0x1dcdc8` | `-` |
| `mov` at 0x10e85c | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10e863 | store | 4 | `sp -0x30` | `0x2` |
| `mov` at 0x10e891 | store | 4 | `abs 0x1dce14` | `0x1` |
| `mov` at 0x10e8a5 | load | 4 | `sp -0x38` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e82e | `_clock_gettime 0x17bccc` | `0x0` | `0x1ed5b8` | `ebx` | `?` | `?` | `?` |
| `call` at 0x10e846 | `_ffscTimestampToString 0x125d78` | `0x1ed5b8` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10e854 | `_ffscMsg 0x11f0d0` | `0x10e7f6` | `ret(_ffscTimestampToString)` | `?` | `?` | `?` | `?` |
| `call` at 0x10e873 | `_addrdInsertIdentity 0x10ddd0` | `sp-52` | `0x1f0d00` | `?` | `0x3` | `0x2` | `?` |
| `call` at 0x10e87c | `_addrdBroadcastMsg 0x10dcf0` | `sp-52` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e888 | `_addrdStartElectionTimer 0x10eee0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10e83d | `(([0x1dcdc8]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dcdc8]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x10e886 | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` |
| `jne` at 0x10e88f | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) != 0x0` | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e8ab | `eax` | `?` |

## `_addrdProcessElectionEndMsg`

- `_addrdProcessElectionEndMsg`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e680 to 0x10e820.
  Established. The `_identFindMaster` result is stored as four bytes at 0x1ede74 (`call` at
  0x10e6c0, `mov` at 0x10e6c5). Distinct later sites write four-byte values 1 and zero at 0x1f1244,
  and another clears four bytes at 0x1dce14 (`mov` at 0x10e6e0, `mov` at 0x10e6fd, `mov` at
  0x10e7c4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `test` at 0x10e686 | load | 1 | `abs 0x1dcdc8` | `-` |
| `mov` at 0x10e6c5 | store | 4 | `abs 0x1ede74` | `ret(_identFindMaster)` |
| `mov` at 0x10e6e0 | store | 4 | `abs 0x1f1244` | `0x1` |
| `mov` at 0x10e6fd | store | 4 | `abs 0x1f1244` | `0x0` |
| `mov` at 0x10e70d | load | 4 | `abs 0x1f0ff4` | `-` |
| `test` at 0x10e774 | load | 1 | `abs 0x1dcdc8` | `-` |
| `mov` at 0x10e7a4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10e7b6 | load | 4 | `abs 0x1f0ff4` | `-` |
| `mov` at 0x10e7c4 | store | 4 | `abs 0x1dce14` | `0x0` |
| `cmp` at 0x10e7ce | load | 4 | `abs 0x1f1244` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e694 | `_ffscMsg 0x11f0d0` | `0x10e5f1` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x10e6a1 | `_identCleanOldEntries 0x1227b8` | `0x1ed5b8` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e6c0 | `_identFindMaster 0x122c38` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e6d4 | `_ffscCompareUniqueIDs 0x125698` | `0x1f0d00` | `ret(_identFindMaster)` | `?` | `?` | `?` | `?` |
| `call` at 0x10e6f5 | `_ffscMsg 0x11f0d0` | `0x10e60d` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e707 | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e713 | `_semTake 0x172b54` | `[0x1f0ff4]` | `ret(_sysClkRateGet)` | `?` | `?` | `?` | `?` |
| `call` at 0x10e724 | `_ffscMsg 0x11f0d0` | `0x10d720` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e745 | `_lstFirst 0x190408` | `0x1dd2c8` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e75d | `_ffscCompareTimestamps 0x125648` | `?` | `0x1ed5b8` | `?` | `?` | `?` | `?` |
| `call` at 0x10e76a | `_lstNext 0x1904b8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e780 | `_ffscTimestampToString 0x125d78` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10e78e | `_ffscMsg 0x11f0d0` | `0x10e628` | `ret(_ffscTimestampToString)` | `?` | `?` | `?` | `?` |
| `call` at 0x10e79c | `_lstDelete 0x190328` | `0x1dd2c8` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e7a8 | `_semGive 0x172ad4` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e7bc | `_semGive 0x172ad4` | `[0x1f0ff4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e7d7 | `_addrdResolveConflicts 0x10ed10` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e7e5 | `_ffscMsg 0x11f0d0` | `0x10e655` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10e68d | `(([0x1dcdc8]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dcdc8]:1 & 0xff) & 0x1) != 0x0` |
| `je` at 0x10e6ab | `(ret(_identCleanOldEntries) & ret(_identCleanOldEntries)) == 0x0` | `(ret(_identCleanOldEntries) & ret(_identCleanOldEntries)) != 0x0` |
| `je` at 0x10e6cc | `(ret(_identFindMaster) & ret(_identFindMaster)) == 0x0` | `(ret(_identFindMaster) & ret(_identFindMaster)) != 0x0` |
| `jne` at 0x10e6de | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `jge` at 0x10e71d | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x10e751 | `(ret(_lstFirst) & ret(_lstFirst)) == 0x0` | `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` |
| `jg` at 0x10e767 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `je` at 0x10e77b | `(([0x1dcdc8]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dcdc8]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x10e7b4 | `(ret(_lstNext) & ret(_lstNext)) != 0x0` | `(ret(_lstNext) & ret(_lstNext)) == 0x0` |
| `je` at 0x10e7d5 | `[0x1f1244] == 0x0` | `[0x1f1244] != 0x0` |
| `je` at 0x10e7de | `(ret(_addrdResolveConflicts) & ret(_addrdResolveConflicts)) == 0x0` | `(ret(_addrdResolveConflicts) & ret(_addrdResolveConflicts)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e7f5 | `eax` | `?` |

## `_addrdProcessIdentityMsg`

- `_addrdProcessIdentityMsg`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e920 to 0x10ea50.
  Established. The initial `_addrdExtractIdentity` call begins with candidates `[sp+4]` and `sp-32`
  (`call` at 0x10e930). `je` bypasses `_addrdStartElectionTimer` when `[0x1dce14] == 0x0`, while
  nonzero falls through to that call (`je` at 0x10e986, `call` at 0x10e988).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e928 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x10e97f | load | 4 | `abs 0x1dce14` | `-` |
| `test` at 0x10e991 | load | 1 | `abs 0x1dcdc8` | `-` |
| `cmp` at 0x10e9c2 | load | 4 | `abs 0x1f1244` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e930 | `_addrdExtractIdentity 0x10dd70` | `[sp+4]` | `sp-32` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10e93e | `_ffscCompareUniqueIDs 0x125698` | `sp-32` | `0x1f0d00` | `?` | `?` | `?` | `?` |
| `call` at 0x10e94f | `_ffscUniqueIDToString 0x125ec8` | `sp-32` | `sp-56` | `?` | `?` | `?` | `?` |
| `call` at 0x10e95d | `_ffscMsg 0x11f0d0` | `0x10e8ac` | `sp-56` | `?` | `?` | `?` | `?` |
| `call` at 0x10e971 | `_identUpdate 0x123198` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e988 | `_addrdStartElectionTimer 0x10eee0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e9ae | `_ffscMsg 0x11f0d0` | `0x10e903` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e9ba | `_identPrint 0x1232c8` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e9cb | `_addrdResolveConflicts 0x10ed10` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e9d9 | `_ffscMsg 0x11f0d0` | `0x10e655` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e948 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `jl` at 0x10e97d | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10e986 | `[0x1dce14] == 0x0` | `[0x1dce14] != 0x0` |
| `jne` at 0x10e98f | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) != 0x0` | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) == 0x0` |
| `je` at 0x10e998 | `(([0x1dcdc8]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dcdc8]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x10e9a1 | `(ret(_identUpdate) & ret(_identUpdate)) != 0x0` | `(ret(_identUpdate) & ret(_identUpdate)) == 0x0` |
| `je` at 0x10e9c9 | `[0x1f1244] == 0x0` | `[0x1f1244] != 0x0` |
| `je` at 0x10e9d2 | `(ret(_addrdResolveConflicts) & ret(_addrdResolveConflicts)) == 0x0` | `(ret(_addrdResolveConflicts) & ret(_addrdResolveConflicts)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e9e8 | `eax` | `?` |

## `_addrdProcessQueryMsg`

- `_addrdProcessQueryMsg`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ea50 to 0x10ebc0.
  Established. Calls `_taskDelay` with first candidate 3 before writing four-byte values 3 and 1 at
  `sp -0x34` and `sp -0x30` (`call` at 0x10eb0c, `mov` at 0x10eb14, `mov` at 0x10eb1b). A later
  absolute four-byte store writes 1 at 0x1dce14 (`mov` at 0x10eb49).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10ea58 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x10eac3 | load | 4 | `abs 0x1dce14` | `-` |
| `test` at 0x10ead9 | load | 1 | `abs 0x1dcdc8` | `-` |
| `mov` at 0x10eb14 | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10eb1b | store | 4 | `sp -0x30` | `0x1` |
| `mov` at 0x10eb49 | store | 4 | `abs 0x1dce14` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ea62 | `_clock_gettime 0x17bccc` | `0x0` | `0x1ed5b8` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10ea6f | `_addrdExtractIdentity 0x10dd70` | `[sp+4]` | `sp-80` | `?` | `?` | `?` | `?` |
| `call` at 0x10ea7d | `_ffscCompareUniqueIDs 0x125698` | `sp-80` | `0x1f0d00` | `?` | `?` | `?` | `?` |
| `call` at 0x10ea8e | `_ffscUniqueIDToString 0x125ec8` | `sp-80` | `sp-104` | `?` | `?` | `?` | `?` |
| `call` at 0x10ea9c | `_ffscMsg 0x11f0d0` | `0x10e9e9` | `sp-104` | `?` | `?` | `?` | `?` |
| `call` at 0x10eab1 | `_identUpdate 0x123198` | `sp-80` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10eacc | `_addrdStartElectionTimer 0x10eee0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10eaf6 | `_ffscMsg 0x11f0d0` | `0x10ea32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10eb02 | `_identPrint 0x1232c8` | `sp-80` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10eb0c | `_taskDelay 0x1723d8` | `0x3` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10eb2b | `_addrdInsertIdentity 0x10ddd0` | `sp-52` | `0x1f0d00` | `?` | `?` | `?` | `?` |
| `call` at 0x10eb34 | `_addrdBroadcastMsg 0x10dcf0` | `sp-52` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10eb40 | `_addrdStartElectionTimer 0x10eee0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10ea87 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `jl` at 0x10eabd | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10eaca | `[0x1dce14] == 0x0` | `[0x1dce14] != 0x0` |
| `jne` at 0x10ead3 | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) != 0x0` | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) == 0x0` |
| `je` at 0x10eae0 | `(([0x1dcdc8]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dcdc8]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x10eae9 | `(ret(_identUpdate) & ret(_identUpdate)) != 0x0` | `(ret(_identUpdate) & ret(_identUpdate)) == 0x0` |
| `jne` at 0x10eb3e | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` |
| `jne` at 0x10eb47 | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) != 0x0` | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10eb6d | `eax` | `?` |

## `_addrdProcessRequestRackIDMsg`

- `_addrdProcessRequestRackIDMsg`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ebc0 to 0x10ed10.
  Established. The `_memcpy` candidates begin `sp-60`, `ret(_identFindConflictingRackID)` and 0x1c
  (`call` at 0x10ec55). The later explicit four-byte store records `ret(_addrdFindUnusedRackID)` at
  `sp -0x30` (`mov` at 0x10ec62). The final `_addrdAssignNewIdentity` call has first candidate
  `sp-60` (`call` at 0x10ec86).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10ebc9 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x10ebec | load | 4 | `abs 0x1dce14` | `-` |
| `test` at 0x10ec02 | load | 1 | `abs 0x1dcdc8` | `-` |
| `cmp` at 0x10ec33 | load | 4 | `abs 0x1f1244` | `-` |
| `mov` at 0x10ec62 | store | 4 | `sp -0x30` | `ret(_addrdFindUnusedRackID)` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ebd1 | `_addrdExtractIdentity 0x10dd70` | `[sp+4]` | `sp-32` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x10ebda | `_identUpdate 0x123198` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ebf5 | `_addrdStartElectionTimer 0x10eee0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ec1f | `_ffscMsg 0x11f0d0` | `0x10eb6e` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ec2b | `_identPrint 0x1232c8` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ec40 | `_identFindConflictingRackID 0x122bb8` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ec55 | `_memcpy 0x180250` | `sp-60` | `ret(_identFindConflictingRackID)` | `0x1c` | `?` | `?` | `?` |
| `call` at 0x10ec5d | `_addrdFindUnusedRackID 0x10e4e0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ec6f | `_ffscUniqueIDToString 0x125ec8` | `ret(_identFindConflictingRackID)` | `sp-84` | `?` | `?` | `?` | `?` |
| `call` at 0x10ec7d | `_ffscMsg 0x11f0d0` | `0x10eb8b` | `sp-84` | `?` | `?` | `?` | `?` |
| `call` at 0x10ec86 | `_addrdAssignNewIdentity 0x10e280` | `sp-60` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jl` at 0x10ebe6 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10ebf3 | `[0x1dce14] == 0x0` | `[0x1dce14] != 0x0` |
| `jne` at 0x10ebfc | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) != 0x0` | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) == 0x0` |
| `je` at 0x10ec09 | `(([0x1dcdc8]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dcdc8]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x10ec12 | `(ret(_identUpdate) & ret(_identUpdate)) != 0x0` | `(ret(_identUpdate) & ret(_identUpdate)) == 0x0` |
| `je` at 0x10ec3a | `[0x1f1244] == 0x0` | `[0x1f1244] != 0x0` |
| `je` at 0x10ec4c | `(ret(_identFindConflictingRackID) & ret(_identFindConflictingRackID)) == 0x0` | `(ret(_identFindConflictingRackID) & ret(_identFindConflictingRackID)) != 0x0` |
| `jne` at 0x10ec68 | `ret(_addrdFindUnusedRackID) != 0xffffffff` | `ret(_addrdFindUnusedRackID) == 0xffffffff` |
| `je` at 0x10ec8d | `(ret(_addrdAssignNewIdentity) & ret(_addrdAssignNewIdentity)) == 0x0` | `(ret(_addrdAssignNewIdentity) & ret(_addrdAssignNewIdentity)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ecab | `eax` | `?` |

## `_addrdAssignNewIdentity`

- `_addrdAssignNewIdentity`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e280 to 0x10e380.
  Established. Two four-byte stores place value 3 at `sp -0x4c` and `sp -0x48` (`mov` at 0x10e2f0,
  `mov` at 0x10e2f7). `_addrdInsertIdentity` then receives candidates beginning `sp-76` and `[sp+4]`
  (`call` at 0x10e303). The later `_memcpy` candidates begin `ret(_identFindByUniqueID)`, `[sp+4]`
  and 0x1c (`call` at 0x10e363).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e289 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10e2f0 | store | 4 | `sp -0x4c` | `0x3` |
| `mov` at 0x10e2f7 | store | 4 | `sp -0x48` | `0x3` |
| `test` at 0x10e340 | load | 1 | `abs 0x1dcdc8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e28d | `_identFindByUniqueID 0x122b28` | `[sp+4]` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x10e2a0 | `_ffscUniqueIDToString 0x125ec8` | `[sp+4]` | `sp-28` | `?` | `?` | `?` | `?` |
| `call` at 0x10e2b6 | `_ffscCompareUniqueIDs 0x125698` | `[sp+4]` | `0x1f0d00` | `?` | `?` | `?` | `?` |
| `call` at 0x10e2c3 | `_addrdUpdateLocalIdentity 0x10dbf0` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e2d8 | `_ffscMsg 0x11f0d0` | `0x10e208` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e303 | `_addrdInsertIdentity 0x10ddd0` | `sp-76` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x10e30c | `_addrdBroadcastMsg 0x10dcf0` | `sp-76` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e31d | `_ffscUniqueIDToString 0x125ec8` | `ret(_identFindByUniqueID)` | `sp-100` | `?` | `?` | `?` | `?` |
| `call` at 0x10e32b | `_ffscMsg 0x11f0d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e34e | `_ffscMsg 0x11f0d0` | `0x10e259` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e357 | `_identPrint 0x1232c8` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e363 | `_memcpy 0x180250` | `ret(_identFindByUniqueID)` | `[sp+4]` | `0x1c` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e299 | `(ret(_identFindByUniqueID) & ret(_identFindByUniqueID)) != 0x0` | `(ret(_identFindByUniqueID) & ret(_identFindByUniqueID)) == 0x0` |
| `jne` at 0x10e2c0 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `je` at 0x10e2cd | `(ret(_addrdUpdateLocalIdentity) & ret(_addrdUpdateLocalIdentity)) == 0x0` | `(ret(_addrdUpdateLocalIdentity) & ret(_addrdUpdateLocalIdentity)) != 0x0` |
| `je` at 0x10e316 | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` |
| `je` at 0x10e347 | `(([0x1dcdc8]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dcdc8]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e373 | `eax` | `?` |

## `_addrdFindUnusedRackID`

- `_addrdFindUnusedRackID`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e4e0 to 0x10e570.
  Established. The backward `jbe` is taken for `? <= 0x9 (unsigned, 32-bit)` and falls through for
  `? > 0x9 (unsigned, 32-bit)` (`jbe` at 0x10e504). The `_identFindByRackID` call has an unknown
  first candidate, and the final EAX value remains unknown (`call` at 0x10e4ed, `ret` at 0x10e511).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e50b | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e4ed | `_identFindByRackID 0x122ad8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e4f7 | `(ret(_identFindByRackID) & ret(_identFindByRackID)) != 0x0` | `(ret(_identFindByRackID) & ret(_identFindByRackID)) == 0x0` |
| `jbe` at 0x10e504 | `? <= 0x9 (unsigned, 32-bit)` | `? > 0x9 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e511 | `eax` | `?` |

## `_addrdResolveRackIDConflict`

- `_addrdResolveRackIDConflict`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ee30 to 0x10eee0.
  Established. The `_memcpy` candidates begin `sp-32`, unknown and 0x1c (`call` at 0x10ee67). After
  `_addrdFindUnusedRackID`, `jne` takes the route directly to `_addrdAssignNewIdentity` for
  `ret(_addrdFindUnusedRackID) != 0xffffffff` and equality falls through to `_ffscUniqueIDToString`
  and `_ffscMsg` before reaching that assignment call (`call` at 0x10ee6f, `jne` at 0x10ee7a, `call`
  at 0x10ee84, `call` at 0x10ee92, `call` at 0x10ee9e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10ee38 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10ee3b | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10ee74 | store | 4 | `sp -0x14` | `ret(_addrdFindUnusedRackID)` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ee40 | `_ffscCompareUniqueIDs 0x125698` | `[sp+4]` | `[sp+8]` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10ee67 | `_memcpy 0x180250` | `sp-32` | `?` | `0x1c` | `?` | `?` | `?` |
| `call` at 0x10ee6f | `_addrdFindUnusedRackID 0x10e4e0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ee84 | `_ffscUniqueIDToString 0x125ec8` | `sp-32` | `sp-56` | `?` | `?` | `?` | `?` |
| `call` at 0x10ee92 | `_ffscMsg 0x11f0d0` | `0x10ee08` | `sp-56` | `?` | `?` | `?` | `?` |
| `call` at 0x10ee9e | `_addrdAssignNewIdentity 0x10e280` | `sp-32` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x10ee4a | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x10ee7a | `ret(_addrdFindUnusedRackID) != 0xffffffff` | `ret(_addrdFindUnusedRackID) == 0xffffffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10eeab | `eax` | `ret(_addrdAssignNewIdentity)` |

## `_addrdFindUnusedIPAddr`

- `_addrdFindUnusedIPAddr`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e480 to 0x10e4e0.
  Established. The `_inet_makeaddr_b` candidates begin `[0x1dd30c]`, unknown and `[sp+4]` (`call` at
  0x10e498). The backward `jle` is taken for `? <= 0xfffffe (signed, 32-bit)` with the changing
  value unresolved (`jle` at 0x10e4c7).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e485 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10e492 | load | 4 | `abs 0x1dd30c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e498 | `_inet_makeaddr_b 0x188ad0` | `[0x1dd30c]` | `?` | `[sp+4]` | `?` | `?` | `?` |
| `call` at 0x10e4a1 | `_identFindByIPAddr 0x122a18` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e4ab | `(ret(_identFindByIPAddr) & ret(_identFindByIPAddr)) != 0x0` | `(ret(_identFindByIPAddr) & ret(_identFindByIPAddr)) == 0x0` |
| `jle` at 0x10e4c7 | `? <= 0xfffffe (signed, 32-bit)` | `? > 0xfffffe (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e4d6 | `eax` | `?` |
