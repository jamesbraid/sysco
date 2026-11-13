# MMSC 2.2 election and rack-identity routines

Read from `6.5.6/usr/cpu/firmware/mmscfw.bin` (IRIX 6.5.6, MMSC 2.2).
Image SHA-256: `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol as an inferred end and can include literals
after returns. Statements below concern sites reached by known control-flow edges.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Candidate slots can contain saved
registers, local values or stale stack words. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/election-2.2-facts.txt) include all tool sections.

## `_addrdElectMaster`

- `_addrdElectMaster`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10d9a0 to 0x10dba0.
  Established. Calls `_semBCreate` with first candidates zero and zero, and `_clock_gettime` with
  zero and `sp-12` (`call` at 0x10d9ab, `call` at 0x10d9bf). The zero result of
  `_addrdElectionStart` takes `je` to the `_semTake` call whose candidates begin `[sp-16]` and
  0xffffffff (`call` at 0x10da0c, `je` at 0x10da13, `call` at 0x10da46).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10d9b3 | store | 4 | `sp -0x10` | `ret(_semBCreate)` |
| `mov` at 0x10d9cd | load | 4 | `abs 0x1f4e64` | `-` |
| `mov` at 0x10d9fe | load | 4 | `abs 0x1f4e64` | `-` |
| `mov` at 0x10da28 | load | 4 | `abs 0x1f4e64` | `-` |
| `mov` at 0x10da42 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x10da65 | load | 4 | `abs 0x1f4e64` | `-` |
| `mov` at 0x10da9e | load | 4 | `abs 0x1f4e64` | `-` |
| `mov` at 0x10daac | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x10dac0 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x10dacb | load | 4 | `sp -0x1c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10d9ab | `_semBCreate 0x17b5f0` | `0x0` | `0x0` | `ebx` | `?` | `?` | `?` |
| `call` at 0x10d9bf | `_clock_gettime 0x17fa34` | `0x0` | `sp-12` | `?` | `?` | `?` | `ret(_semBCreate)` |
| `call` at 0x10d9c7 | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d9d3 | `_semTake 0x1768bc` | `[0x1f4e64]` | `ret(_sysClkRateGet)` | `?` | `?` | `?` | `?` |
| `call` at 0x10d9e4 | `_ffscMsg 0x12154c` | `0x10d8fa` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d9f6 | `_lstAdd 0x194000` | `0x1e1128` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x10da04 | `_semGive 0x17683c` | `[0x1f4e64]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10da0c | `_addrdElectionStart 0x10e5b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10da1a | `_ffscMsg 0x12154c` | `0x10d92a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10da22 | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10da2e | `_semTake 0x1768bc` | `[0x1f4e64]` | `ret(_sysClkRateGet)` | `?` | `?` | `?` | `?` |
| `call` at 0x10da46 | `_semTake 0x1768bc` | `[sp-16]` | `0xffffffff` | `?` | `?` | `?` | `?` |
| `call` at 0x10da57 | `_ffscMsg 0x12154c` | `0x10d96f` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10da5f | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10da6b | `_semTake 0x1768bc` | `[0x1f4e64]` | `ret(_sysClkRateGet)` | `?` | `?` | `?` | `?` |
| `call` at 0x10da7c | `_ffscMsg 0x12154c` | `0x10d943` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10da96 | `_lstDelete 0x194090` | `0x1e1128` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x10daa4 | `_semGive 0x17683c` | `[0x1f4e64]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dab0 | `_semDelete 0x194454` | `[sp-16]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dac4 | `_semDelete 0x194454` | `[sp-16]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x10d9dd | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x10da13 | `(ret(_addrdElectionStart) & ret(_addrdElectionStart)) == 0x0` | `(ret(_addrdElectionStart) & ret(_addrdElectionStart)) != 0x0` |
| `jl` at 0x10da38 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10da50 | `(ret(_semTake) & ret(_semTake)) == 0x0` | `(ret(_semTake) & ret(_semTake)) != 0x0` |
| `jge` at 0x10da75 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10dad1 | `eax` | `?` |

## `_addrdElectionStart`

- `_addrdElectionStart`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e5b0 to 0x10e5f0.
  Established. The two explicit four-byte stores place 3 at `sp -0x34` and 4 at `sp -0x30` (`mov` at
  0x10e5b6, `mov` at 0x10e5bd). `_addrdLocalMsg` receives first candidate `sp-52` (`call` at
  0x10e5c8); its nonzero result takes `jne` to the all-ones EAX return, while zero falls through to
  the zero-valued exit (`jne` at 0x10e5cf, `ret` at 0x10e5e8, `ret` at 0x10e5d6).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e5b6 | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10e5bd | store | 4 | `sp -0x30` | `0x4` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e5c8 | `_addrdLocalMsg 0x10e030` | `sp-52` | `0x3` | `0x4` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e5cf | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) != 0x0` | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e5d6 | `eax` | `0x0` |
| `ret` at 0x10e5e8 | `eax` | `0xffffffff` |

## `_addrdElectionEnd`

- `_addrdElectionEnd`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e570 to 0x10e5b0.
  Established. The stack record begins with four-byte values 3 at `sp -0x34` and 5 at `sp -0x30`
  (`mov` at 0x10e576, `mov` at 0x10e57d). The `_addrdLocalMsg` result selects the all-ones return on
  taken `jne` for nonzero, or the zero return on fall-through for zero (`call` at 0x10e588, `jne` at
  0x10e58f, `ret` at 0x10e5a8, `ret` at 0x10e596).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e576 | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10e57d | store | 4 | `sp -0x30` | `0x5` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e588 | `_addrdLocalMsg 0x10e030` | `sp-52` | `0x3` | `0x5` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e58f | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) != 0x0` | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e596 | `eax` | `0x0` |
| `ret` at 0x10e5a8 | `eax` | `0xffffffff` |

## `_addrdElectionTimerPop`

- `_addrdElectionTimerPop`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e5f0 to 0x10e620.
  Established. Calls `_addrdElectionEnd` and then returns EAX zero (`call` at 0x10e5f3, `ret` at
  0x10e5fd). The linked listing includes bytes after that return which are not reached by known CFG
  edges.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e5f3 | `_addrdElectionEnd 0x10e570` | `ebp` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e5fd | `eax` | `0x0` |

## `_addrdStartElectionTimer`

- `_addrdStartElectionTimer`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10f0d0 to 0x10f118.
  Established. The four-byte read at 0x1f1430 supplies the first `_wdStart` candidate, and the
  second is `(ret(_sysClkRateGet) + ret(_sysClkRateGet))` (`mov` at 0x10f0e2, `call` at 0x10f0e8).
  The deciding `jl` retains unknown condition operands (`jl` at 0x10f0f2).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10f0e2 | load | 4 | `abs 0x1f1430` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10f0da | `_sysClkRateGet 0x1085a8` | `0x10e5f0` | `0x0` | `ebp` | `?` | `?` | `?` |
| `call` at 0x10f0e8 | `_wdStart 0x178f64` | `[0x1f1430]` | `(ret(_sysClkRateGet) + ret(_sysClkRateGet))` | `?` | `?` | `?` | `?` |
| `call` at 0x10f105 | `_ffscMsg 0x12154c` | `0x10f0a9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jl` at 0x10f0f2 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10f0f9 | `eax` | `0x0` |
| `ret` at 0x10f112 | `eax` | `0xffffffff` |

## `_addrdRequestRackID`

- `_addrdRequestRackID`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10dd30 to 0x10ddd0.
  Established. `_identSetRackID` receives first candidate `[[sp+4]+12]` (`call` at 0x10dd3e). The
  later local record contains four-byte values 3 and 6 at `sp -0x34` and `sp -0x30` (`mov` at
  0x10dd7a, `mov` at 0x10dd81). `je` bypasses `_addrdLocalMsg` when `[0x1f50e4] == 0x0` (`je` at
  0x10dda0, `call` at 0x10dda3).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10dd37 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10dd3a | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10dd7a | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10dd81 | store | 4 | `sp -0x30` | `0x6` |
| `cmp` at 0x10dd99 | load | 4 | `abs 0x1f50e4` | `-` |
| `mov` at 0x10ddc5 | load | 4 | `sp -0x38` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10dd3e | `_identSetRackID 0x125f40` | `[[sp+4]+12]` | `ebx` | `?` | `?` | `?` | `?` |
| `call` at 0x10dd4f | `_ffscMsg 0x12154c` | `0x10dcd3` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dd61 | `_identUpdate 0x125a50` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dd72 | `_ffscMsg 0x12154c` | `0x10dcef` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dd91 | `_addrdInsertIdentity 0x10dfb0` | `sp-52` | `0x1f4b70` | `?` | `0x3` | `0x6` | `?` |
| `call` at 0x10dda3 | `_addrdLocalMsg 0x10e030` | `sp-52` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ddb3 | `_addrdBroadcastMsg 0x10ded0` | `sp-52` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10dd48 | `(ret(_identSetRackID) & ret(_identSetRackID)) == 0x0` | `(ret(_identSetRackID) & ret(_identSetRackID)) != 0x0` |
| `je` at 0x10dd6b | `(ret(_identUpdate) & ret(_identUpdate)) == 0x0` | `(ret(_identUpdate) & ret(_identUpdate)) != 0x0` |
| `je` at 0x10dda0 | `[0x1f50e4] == 0x0` | `[0x1f50e4] != 0x0` |
| `jne` at 0x10ddad | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) != 0x0` | `(ret(_addrdLocalMsg) & ret(_addrdLocalMsg)) == 0x0` |
| `jne` at 0x10ddba | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ddcb | `eax` | `?` |

## `_addrdProcessElectionStartMsg`

- `_addrdProcessElectionStartMsg`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ea10 to 0x10eb10.
  Established. The explicit local stores place 3 at `sp -0x34` and 2 at `sp -0x30` before
  `_addrdInsertIdentity` receives `sp-52` and 0x1f4b70 as its first candidates (`mov` at 0x10ea4c,
  `mov` at 0x10ea53, `call` at 0x10ea63). A later four-byte store writes 1 to 0x1e0c44 (`mov` at
  0x10ea81).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `test` at 0x10ea26 | load | 1 | `abs 0x1e0bf0` | `-` |
| `mov` at 0x10ea4c | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10ea53 | store | 4 | `sp -0x30` | `0x2` |
| `mov` at 0x10ea81 | store | 4 | `abs 0x1e0c44` | `0x1` |
| `mov` at 0x10ea95 | load | 4 | `sp -0x38` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ea1e | `_clock_gettime 0x17fa34` | `0x0` | `0x1f1428` | `ebx` | `?` | `?` | `?` |
| `call` at 0x10ea36 | `_ffscTimestampToString 0x128630` | `0x1f1428` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10ea44 | `_ffscMsg 0x12154c` | `0x10e9e6` | `ret(_ffscTimestampToString)` | `?` | `?` | `?` | `?` |
| `call` at 0x10ea63 | `_addrdInsertIdentity 0x10dfb0` | `sp-52` | `0x1f4b70` | `?` | `0x3` | `0x2` | `?` |
| `call` at 0x10ea6c | `_addrdBroadcastMsg 0x10ded0` | `sp-52` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ea78 | `_addrdStartElectionTimer 0x10f0d0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10ea2d | `(([0x1e0bf0]:1 & 0xff) & 0x1) == 0x0` | `(([0x1e0bf0]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x10ea76 | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` |
| `jne` at 0x10ea7f | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) != 0x0` | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ea9b | `eax` | `?` |

## `_addrdProcessElectionEndMsg`

- `_addrdProcessElectionEndMsg`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e870 to 0x10ea10.
  Established. Stores the `_identFindMaster` result as four bytes at 0x1f1ce8 (`call` at 0x10e8b0,
  `mov` at 0x10e8b5). Later distinct store sites write four-byte values 1 and zero to 0x1f50e4, and
  a further store clears four bytes at 0x1e0c44 (`mov` at 0x10e8d0, `mov` at 0x10e8ed, `mov` at
  0x10e9b4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `test` at 0x10e876 | load | 1 | `abs 0x1e0bf0` | `-` |
| `mov` at 0x10e8b5 | store | 4 | `abs 0x1f1ce8` | `ret(_identFindMaster)` |
| `mov` at 0x10e8d0 | store | 4 | `abs 0x1f50e4` | `0x1` |
| `mov` at 0x10e8ed | store | 4 | `abs 0x1f50e4` | `0x0` |
| `mov` at 0x10e8fd | load | 4 | `abs 0x1f4e64` | `-` |
| `test` at 0x10e964 | load | 1 | `abs 0x1e0bf0` | `-` |
| `mov` at 0x10e994 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10e9a6 | load | 4 | `abs 0x1f4e64` | `-` |
| `mov` at 0x10e9b4 | store | 4 | `abs 0x1e0c44` | `0x0` |
| `cmp` at 0x10e9be | load | 4 | `abs 0x1f50e4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e884 | `_ffscMsg 0x12154c` | `0x10e7e1` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x10e891 | `_identCleanOldEntries 0x125060` | `0x1f1428` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e8b0 | `_identFindMaster 0x1254e0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e8c4 | `_ffscCompareUniqueIDs 0x127f50` | `0x1f4b70` | `ret(_identFindMaster)` | `?` | `?` | `?` | `?` |
| `call` at 0x10e8e5 | `_ffscMsg 0x12154c` | `0x10e7fd` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e8f7 | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e903 | `_semTake 0x1768bc` | `[0x1f4e64]` | `ret(_sysClkRateGet)` | `?` | `?` | `?` | `?` |
| `call` at 0x10e914 | `_ffscMsg 0x12154c` | `0x10d8fa` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e935 | `_lstFirst 0x194170` | `0x1e1128` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e94d | `_ffscCompareTimestamps 0x127f00` | `?` | `0x1f1428` | `?` | `?` | `?` | `?` |
| `call` at 0x10e95a | `_lstNext 0x194220` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e970 | `_ffscTimestampToString 0x128630` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10e97e | `_ffscMsg 0x12154c` | `0x10e818` | `ret(_ffscTimestampToString)` | `?` | `?` | `?` | `?` |
| `call` at 0x10e98c | `_lstDelete 0x194090` | `0x1e1128` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e998 | `_semGive 0x17683c` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e9ac | `_semGive 0x17683c` | `[0x1f4e64]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e9c7 | `_addrdResolveConflicts 0x10ef00` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e9d5 | `_ffscMsg 0x12154c` | `0x10e845` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10e87d | `(([0x1e0bf0]:1 & 0xff) & 0x1) == 0x0` | `(([0x1e0bf0]:1 & 0xff) & 0x1) != 0x0` |
| `je` at 0x10e89b | `(ret(_identCleanOldEntries) & ret(_identCleanOldEntries)) == 0x0` | `(ret(_identCleanOldEntries) & ret(_identCleanOldEntries)) != 0x0` |
| `je` at 0x10e8bc | `(ret(_identFindMaster) & ret(_identFindMaster)) == 0x0` | `(ret(_identFindMaster) & ret(_identFindMaster)) != 0x0` |
| `jne` at 0x10e8ce | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `jge` at 0x10e90d | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x10e941 | `(ret(_lstFirst) & ret(_lstFirst)) == 0x0` | `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` |
| `jg` at 0x10e957 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `je` at 0x10e96b | `(([0x1e0bf0]:1 & 0xff) & 0x1) == 0x0` | `(([0x1e0bf0]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x10e9a4 | `(ret(_lstNext) & ret(_lstNext)) != 0x0` | `(ret(_lstNext) & ret(_lstNext)) == 0x0` |
| `je` at 0x10e9c5 | `[0x1f50e4] == 0x0` | `[0x1f50e4] != 0x0` |
| `je` at 0x10e9ce | `(ret(_addrdResolveConflicts) & ret(_addrdResolveConflicts)) == 0x0` | `(ret(_addrdResolveConflicts) & ret(_addrdResolveConflicts)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e9e5 | `eax` | `?` |

## `_addrdProcessIdentityMsg`

- `_addrdProcessIdentityMsg`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10eb10 to 0x10ec40.
  Established. The initial `_addrdExtractIdentity` call has first candidates `[sp+4]` and `sp-32`
  (`call` at 0x10eb20). `je` bypasses `_addrdStartElectionTimer` when `[0x1e0c44] == 0x0`, while
  nonzero falls through to that call (`je` at 0x10eb76, `call` at 0x10eb78).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10eb18 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x10eb6f | load | 4 | `abs 0x1e0c44` | `-` |
| `test` at 0x10eb81 | load | 1 | `abs 0x1e0bf0` | `-` |
| `cmp` at 0x10ebb2 | load | 4 | `abs 0x1f50e4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10eb20 | `_addrdExtractIdentity 0x10df50` | `[sp+4]` | `sp-32` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10eb2e | `_ffscCompareUniqueIDs 0x127f50` | `sp-32` | `0x1f4b70` | `?` | `?` | `?` | `?` |
| `call` at 0x10eb3f | `_ffscUniqueIDToString 0x128780` | `sp-32` | `sp-56` | `?` | `?` | `?` | `?` |
| `call` at 0x10eb4d | `_ffscMsg 0x12154c` | `0x10ea9c` | `sp-56` | `?` | `?` | `?` | `?` |
| `call` at 0x10eb61 | `_identUpdate 0x125a50` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10eb78 | `_addrdStartElectionTimer 0x10f0d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10eb9e | `_ffscMsg 0x12154c` | `0x10eaf3` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ebaa | `_identPrint 0x125b80` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ebbb | `_addrdResolveConflicts 0x10ef00` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ebc9 | `_ffscMsg 0x12154c` | `0x10e845` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10eb38 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `jl` at 0x10eb6d | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10eb76 | `[0x1e0c44] == 0x0` | `[0x1e0c44] != 0x0` |
| `jne` at 0x10eb7f | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) != 0x0` | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) == 0x0` |
| `je` at 0x10eb88 | `(([0x1e0bf0]:1 & 0xff) & 0x1) == 0x0` | `(([0x1e0bf0]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x10eb91 | `(ret(_identUpdate) & ret(_identUpdate)) != 0x0` | `(ret(_identUpdate) & ret(_identUpdate)) == 0x0` |
| `je` at 0x10ebb9 | `[0x1f50e4] == 0x0` | `[0x1f50e4] != 0x0` |
| `je` at 0x10ebc2 | `(ret(_addrdResolveConflicts) & ret(_addrdResolveConflicts)) == 0x0` | `(ret(_addrdResolveConflicts) & ret(_addrdResolveConflicts)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ebd8 | `eax` | `?` |

## `_addrdProcessQueryMsg`

- `_addrdProcessQueryMsg`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ec40 to 0x10edb0.
  Established. Calls `_taskDelay` with first candidate 3 before storing four-byte values 3 and 1 at
  `sp -0x34` and `sp -0x30` (`call` at 0x10ecfc, `mov` at 0x10ed04, `mov` at 0x10ed0b). The later
  `_addrdBroadcastMsg` call has first candidate `sp-52` (`call` at 0x10ed24).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10ec48 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x10ecb3 | load | 4 | `abs 0x1e0c44` | `-` |
| `test` at 0x10ecc9 | load | 1 | `abs 0x1e0bf0` | `-` |
| `mov` at 0x10ed04 | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10ed0b | store | 4 | `sp -0x30` | `0x1` |
| `mov` at 0x10ed39 | store | 4 | `abs 0x1e0c44` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ec52 | `_clock_gettime 0x17fa34` | `0x0` | `0x1f1428` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10ec5f | `_addrdExtractIdentity 0x10df50` | `[sp+4]` | `sp-80` | `?` | `?` | `?` | `?` |
| `call` at 0x10ec6d | `_ffscCompareUniqueIDs 0x127f50` | `sp-80` | `0x1f4b70` | `?` | `?` | `?` | `?` |
| `call` at 0x10ec7e | `_ffscUniqueIDToString 0x128780` | `sp-80` | `sp-104` | `?` | `?` | `?` | `?` |
| `call` at 0x10ec8c | `_ffscMsg 0x12154c` | `0x10ebd9` | `sp-104` | `?` | `?` | `?` | `?` |
| `call` at 0x10eca1 | `_identUpdate 0x125a50` | `sp-80` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ecbc | `_addrdStartElectionTimer 0x10f0d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ece6 | `_ffscMsg 0x12154c` | `0x10ec22` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ecf2 | `_identPrint 0x125b80` | `sp-80` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ecfc | `_taskDelay 0x176140` | `0x3` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ed1b | `_addrdInsertIdentity 0x10dfb0` | `sp-52` | `0x1f4b70` | `?` | `?` | `?` | `?` |
| `call` at 0x10ed24 | `_addrdBroadcastMsg 0x10ded0` | `sp-52` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ed30 | `_addrdStartElectionTimer 0x10f0d0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10ec77 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `jl` at 0x10ecad | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10ecba | `[0x1e0c44] == 0x0` | `[0x1e0c44] != 0x0` |
| `jne` at 0x10ecc3 | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) != 0x0` | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) == 0x0` |
| `je` at 0x10ecd0 | `(([0x1e0bf0]:1 & 0xff) & 0x1) == 0x0` | `(([0x1e0bf0]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x10ecd9 | `(ret(_identUpdate) & ret(_identUpdate)) != 0x0` | `(ret(_identUpdate) & ret(_identUpdate)) == 0x0` |
| `jne` at 0x10ed2e | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` |
| `jne` at 0x10ed37 | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) != 0x0` | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ed5d | `eax` | `?` |

## `_addrdProcessRequestRackIDMsg`

- `_addrdProcessRequestRackIDMsg`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10edb0 to 0x10ef00.
  Established. The `_memcpy` candidates begin `sp-60`, `ret(_identFindConflictingRackID)` and 0x1c
  (`call` at 0x10ee45). A later four-byte store records `ret(_addrdFindUnusedRackID)` at `sp -0x30`
  (`mov` at 0x10ee52). `jne` skips the following `_ffscUniqueIDToString` and `_ffscMsg` calls when
  that result is not equal to 0xffffffff (`jne` at 0x10ee58, `call` at 0x10ee5f, `call` at 0x10ee6d).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10edb9 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x10eddc | load | 4 | `abs 0x1e0c44` | `-` |
| `test` at 0x10edf2 | load | 1 | `abs 0x1e0bf0` | `-` |
| `cmp` at 0x10ee23 | load | 4 | `abs 0x1f50e4` | `-` |
| `mov` at 0x10ee52 | store | 4 | `sp -0x30` | `ret(_addrdFindUnusedRackID)` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10edc1 | `_addrdExtractIdentity 0x10df50` | `[sp+4]` | `sp-32` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x10edca | `_identUpdate 0x125a50` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ede5 | `_addrdStartElectionTimer 0x10f0d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ee0f | `_ffscMsg 0x12154c` | `0x10ed5e` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ee1b | `_identPrint 0x125b80` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ee30 | `_identFindConflictingRackID 0x125460` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ee45 | `_memcpy 0x183fb8` | `sp-60` | `ret(_identFindConflictingRackID)` | `0x1c` | `?` | `?` | `?` |
| `call` at 0x10ee4d | `_addrdFindUnusedRackID 0x10e6d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ee5f | `_ffscUniqueIDToString 0x128780` | `ret(_identFindConflictingRackID)` | `sp-84` | `?` | `?` | `?` | `?` |
| `call` at 0x10ee6d | `_ffscMsg 0x12154c` | `0x10ed7b` | `sp-84` | `?` | `?` | `?` | `?` |
| `call` at 0x10ee76 | `_addrdAssignNewIdentity 0x10e470` | `sp-60` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jl` at 0x10edd6 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10ede3 | `[0x1e0c44] == 0x0` | `[0x1e0c44] != 0x0` |
| `jne` at 0x10edec | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) != 0x0` | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) == 0x0` |
| `je` at 0x10edf9 | `(([0x1e0bf0]:1 & 0xff) & 0x1) == 0x0` | `(([0x1e0bf0]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x10ee02 | `(ret(_identUpdate) & ret(_identUpdate)) != 0x0` | `(ret(_identUpdate) & ret(_identUpdate)) == 0x0` |
| `je` at 0x10ee2a | `[0x1f50e4] == 0x0` | `[0x1f50e4] != 0x0` |
| `je` at 0x10ee3c | `(ret(_identFindConflictingRackID) & ret(_identFindConflictingRackID)) == 0x0` | `(ret(_identFindConflictingRackID) & ret(_identFindConflictingRackID)) != 0x0` |
| `jne` at 0x10ee58 | `ret(_addrdFindUnusedRackID) != 0xffffffff` | `ret(_addrdFindUnusedRackID) == 0xffffffff` |
| `je` at 0x10ee7d | `(ret(_addrdAssignNewIdentity) & ret(_addrdAssignNewIdentity)) == 0x0` | `(ret(_addrdAssignNewIdentity) & ret(_addrdAssignNewIdentity)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ee9b | `eax` | `?` |

## `_addrdAssignNewIdentity`

- `_addrdAssignNewIdentity`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e470 to 0x10e570.
  Established. The explicit local stores write four-byte value 3 at both `sp -0x4c` and `sp -0x48`
  (`mov` at 0x10e4e0, `mov` at 0x10e4e7). `_addrdInsertIdentity` then receives first candidates
  `sp-76` and `[sp+4]` (`call` at 0x10e4f3).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e479 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10e4e0 | store | 4 | `sp -0x4c` | `0x3` |
| `mov` at 0x10e4e7 | store | 4 | `sp -0x48` | `0x3` |
| `test` at 0x10e530 | load | 1 | `abs 0x1e0bf0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e47d | `_identFindByUniqueID 0x1253d0` | `[sp+4]` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x10e490 | `_ffscUniqueIDToString 0x128780` | `[sp+4]` | `sp-28` | `?` | `?` | `?` | `?` |
| `call` at 0x10e4a6 | `_ffscCompareUniqueIDs 0x127f50` | `[sp+4]` | `0x1f4b70` | `?` | `?` | `?` | `?` |
| `call` at 0x10e4b3 | `_addrdUpdateLocalIdentity 0x10ddd0` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e4c8 | `_ffscMsg 0x12154c` | `0x10e3f8` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e4f3 | `_addrdInsertIdentity 0x10dfb0` | `sp-76` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x10e4fc | `_addrdBroadcastMsg 0x10ded0` | `sp-76` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e50d | `_ffscUniqueIDToString 0x128780` | `ret(_identFindByUniqueID)` | `sp-100` | `?` | `?` | `?` | `?` |
| `call` at 0x10e51b | `_ffscMsg 0x12154c` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e53e | `_ffscMsg 0x12154c` | `0x10e449` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e547 | `_identPrint 0x125b80` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e553 | `_memcpy 0x183fb8` | `ret(_identFindByUniqueID)` | `[sp+4]` | `0x1c` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e489 | `(ret(_identFindByUniqueID) & ret(_identFindByUniqueID)) != 0x0` | `(ret(_identFindByUniqueID) & ret(_identFindByUniqueID)) == 0x0` |
| `jne` at 0x10e4b0 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `je` at 0x10e4bd | `(ret(_addrdUpdateLocalIdentity) & ret(_addrdUpdateLocalIdentity)) == 0x0` | `(ret(_addrdUpdateLocalIdentity) & ret(_addrdUpdateLocalIdentity)) != 0x0` |
| `je` at 0x10e506 | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` |
| `je` at 0x10e537 | `(([0x1e0bf0]:1 & 0xff) & 0x1) == 0x0` | `(([0x1e0bf0]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e563 | `eax` | `?` |

## `_addrdFindUnusedRackID`

- `_addrdFindUnusedRackID`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e6d0 to 0x10e760.
  Established. The `_identFindByRackID` call has an unknown first candidate (`call` at 0x10e6dd).
  The backward `jbe` is taken for `? <= 0x15 (unsigned, 32-bit)` and falls through for `? > 0x15
  (unsigned, 32-bit)`, so the tool does not establish the changing value (`jbe` at 0x10e6f4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e6fb | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e6dd | `_identFindByRackID 0x125380` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e6e7 | `(ret(_identFindByRackID) & ret(_identFindByRackID)) != 0x0` | `(ret(_identFindByRackID) & ret(_identFindByRackID)) == 0x0` |
| `jbe` at 0x10e6f4 | `? <= 0x15 (unsigned, 32-bit)` | `? > 0x15 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e701 | `eax` | `?` |

## `_addrdResolveRackIDConflict`

- `_addrdResolveRackIDConflict`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10f020 to 0x10f0d0.
  Established. `_ffscCompareUniqueIDs` has first candidates `[sp+4]` and `[sp+8]` (`call` at
  0x10f030). The later `_memcpy` has candidates beginning `sp-32`, unknown and 0x1c (`call` at
  0x10f057). `je` selects the region calling `_ffscUniqueIDToString` and `_ffscMsg` when
  `ret(_addrdFindUnusedRackID) == 0xffffffff` (`je` at 0x10f06a, `call` at 0x10f088, `call` at
  0x10f096).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10f028 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f02b | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10f064 | store | 4 | `sp -0x14` | `ret(_addrdFindUnusedRackID)` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10f030 | `_ffscCompareUniqueIDs 0x127f50` | `[sp+4]` | `[sp+8]` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10f057 | `_memcpy 0x183fb8` | `sp-32` | `?` | `0x1c` | `?` | `?` | `?` |
| `call` at 0x10f05f | `_addrdFindUnusedRackID 0x10e6d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10f070 | `_addrdAssignNewIdentity 0x10e470` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10f088 | `_ffscUniqueIDToString 0x128780` | `sp-32` | `sp-56` | `?` | `?` | `?` | `?` |
| `call` at 0x10f096 | `_ffscMsg 0x12154c` | `0x10eff8` | `sp-56` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x10f03a | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x10f06a | `ret(_addrdFindUnusedRackID) == 0xffffffff` | `ret(_addrdFindUnusedRackID) != 0xffffffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10f0a8 | `eax` | `?` |

## `_addrdFindUnusedIPAddr`

- `_addrdFindUnusedIPAddr`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e670 to 0x10e6d0.
  Established. The `_inet_makeaddr_b` candidates begin `[0x1e116c]`, unknown and `[sp+4]` (`call` at
  0x10e688). The backward `jle` retains an unknown compared value with taken predicate `? <=
  0xfffffe (signed, 32-bit)` (`jle` at 0x10e6b7).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e675 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10e682 | load | 4 | `abs 0x1e116c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e688 | `_inet_makeaddr_b 0x18c838` | `[0x1e116c]` | `?` | `[sp+4]` | `?` | `?` | `?` |
| `call` at 0x10e691 | `_identFindByIPAddr 0x1252c0` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e69b | `(ret(_identFindByIPAddr) & ret(_identFindByIPAddr)) != 0x0` | `(ret(_identFindByIPAddr) & ret(_identFindByIPAddr)) == 0x0` |
| `jle` at 0x10e6b7 | `? <= 0xfffffe (signed, 32-bit)` | `? > 0xfffffe (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e6c6 | `eax` | `?` |
