# MMSC 2.1 election and rack-identity routines

Read from IRIX 6.5.5 MMSC 2.1, `6.5.5/usr/cpu/firmware/mmscfw.bin`.
Image SHA-256: `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans end at the next text symbol and can include literals
after the return. These bounds are inferred.

In the expressions below, `sp` denotes the stack pointer at entry and
brackets denote a loaded value. Candidate arguments are locations under
the assumed cdecl32 convention, with unknown arity. Unknown values and
unsupported instruction effects remain explicit.

[Fact sheets and full listings](disasm/election-2.1-facts.txt) preserve
the accesses, calls, conditions and exits behind these entries.

## `_addrdElectMaster`

- `_addrdElectMaster`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10d990 to 0x10db90.
  Established. Begins with `_semBCreate` candidates zero and zero, then calls `_clock_gettime`
  with zero and `sp-12` (`call` at 0x10d99b, `call` at 0x10d9af). `_lstAdd` and `_lstDelete`
  both have candidates 0x1e1110 and `sp-24` (`call` at 0x10d9e6, `call` at 0x10da86). A call to
  `_addrdElectionStart` precedes `je`, which takes the path containing `_semTake` when that call
  returns zero (`call` at 0x10d9fc, `je` at 0x10da03). On that route `_semTake` has `[sp-16]`
  and 0xffffffff as its first two candidates (`call` at 0x10da36). Both later `_semDelete` sites
  receive `[sp-16]` as their first candidate (`call` at 0x10daa0, `call` at 0x10dab4). The
  signed-flag decisions remain unresolved (`jge` at 0x10d9cd, `jl` at 0x10da28, `jge` at
  0x10da65). The merged EAX result is unknown (`ret` at 0x10dac1).

## `_addrdElectionStart`

- `_addrdElectionStart`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e5a0 to 0x10e5e0.
  Established. Stores four-byte values 3 and 4 at entry stack -0x34 and -0x30, then calls
  `_addrdLocalMsg` with first candidate `sp-52` (`mov` at 0x10e5a6, `mov` at 0x10e5ad, `call` at
  0x10e5b8). `jne` takes the nonzero-result path to EAX 0xffffffff, while a zero result falls
  through to EAX zero (`jne` at 0x10e5bf, `mov` at 0x10e5d0, `ret` at 0x10e5d8, `xor` at
  0x10e5c1, `ret` at 0x10e5c6).

## `_addrdElectionEnd`

- `_addrdElectionEnd`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e560 to 0x10e5a0.
  Established. Prepares the stack values 3 and 5 before handing `sp-52` to `_addrdLocalMsg` as
  its first candidate (`mov` at 0x10e566, `mov` at 0x10e56d, `call` at 0x10e578). The two stores
  are four bytes at entry stack -0x34 and -0x30 (`mov` at 0x10e566, `mov` at 0x10e56d). At
  `jne`, a nonzero local-call result takes the 0xffffffff path and zero falls through to the
  zero path (`jne` at 0x10e57f, `mov` at 0x10e590, `ret` at 0x10e598, `xor` at 0x10e581, `ret`
  at 0x10e586).

## `_addrdElectionTimerPop`

- `_addrdElectionTimerPop`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e5e0 to 0x10e610.
  Established. Calls `_addrdElectionEnd`, then replaces EAX with zero before returning (`call` at
  0x10e5e3, `xor` at 0x10e5e8, `ret` at 0x10e5ed). The call result is not forwarded through EAX (`xor` at 0x10e5e8);
  its only recorded memory accesses save and reload EBP on the entry stack (`push` at 0x10e5e0,
  `pop` at 0x10e5ec).

## `_addrdStartElectionTimer`

- `_addrdStartElectionTimer`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10f0c0 to 0x10f108.
  Established. Calls `_sysClkRateGet` and uses `ret(_sysClkRateGet) + ret(_sysClkRateGet)` as the
  second candidate argument of `_wdStart` (`call` at 0x10f0ca, `call` at 0x10f0d8). The first
  candidate is `[0x1f1418]` (`call` at 0x10f0d8). The remaining candidates at `_wdStart` are
  unknown. The `jl` condition is an unresolved flag condition (`jl` at 0x10f0e2); its taken path
  calls `_ffscMsg` and loads 0xffffffff, while fall-through loads zero (`jl` at 0x10f0e2, `call`
  at 0x10f0f5, `mov` at 0x10f0fa, `ret` at 0x10f102, `xor` at 0x10f0e4, `ret` at 0x10f0e9).

## `_addrdRequestRackID`

- `_addrdRequestRackID`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10dd20 to 0x10ddc0.
  Established. Calls `_identSetRackID` with first candidate `[[sp+4]+12]`, then reaches
  `_identUpdate` on the zero-result branch (`call` at 0x10dd2e, `je` at 0x10dd38, `call` at
  0x10dd51). The later stack stores put four-byte values 3 and 6 at entry stack -0x34 and -0x30
  (`mov` at 0x10dd6a, `mov` at 0x10dd71). `_addrdInsertIdentity` receives `sp-52` and 0x1f4b58
  as its first candidates (`call` at 0x10dd81). `je` skips the local-message call when
  `[0x1f50c4] == 0x0` (`je` at 0x10dd90). A nonzero value falls through to `_addrdLocalMsg` (`je` at 0x10dd90,
  `call` at 0x10dd93). A nonzero local-call result takes `jne` past `_addrdBroadcastMsg` (`jne`
  at 0x10dd9d). When reached, `_addrdBroadcastMsg` has `sp-52` in its first candidate location
  (`call` at 0x10dda3).

## `_addrdProcessElectionStartMsg`

- `_addrdProcessElectionStartMsg`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ea00 to 0x10eb00.
  Established. Calls `_clock_gettime` with zero and 0x1f1410 (`call` at 0x10ea0e). It later
  stores four-byte values 3 and 2 at entry stack -0x34 and -0x30 (`mov` at 0x10ea3c, `mov` at
  0x10ea43). `_addrdInsertIdentity` has candidates `sp-52` and 0x1f4b58, followed by
  `_addrdBroadcastMsg` with first candidate `sp-52` (`call` at 0x10ea53, `call` at 0x10ea5c). A
  nonzero broadcast result takes `jne` to the 0xffffffff load (`jne` at 0x10ea66, `mov` at
  0x10ea80). Zero falls through to `_addrdStartElectionTimer`, whose nonzero result takes the
  next `jne` to that load (`call` at 0x10ea68, `jne` at 0x10ea6f, `mov` at 0x10ea80). The
  remaining fall-through path writes four-byte value 1 at 0x1e0c34 (`mov` at 0x10ea71).

## `_addrdProcessElectionEndMsg`

- `_addrdProcessElectionEndMsg`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e860 to 0x10ea00.
  Established. Passes 0x1f1410 to `_identCleanOldEntries`, and takes `je` to the next stage when
  that result is zero (`call` at 0x10e881, `je` at 0x10e88b). `_identFindMaster` is called and
  its result stored as four bytes at 0x1f1cd0 (`call` at 0x10e8a0, `mov` at 0x10e8a5). A zero
  result takes `je` to a path that stores zero at 0x1f50c4 (`je` at 0x10e8ac, `mov` at
  0x10e8dd). Otherwise `_ffscCompareUniqueIDs` has candidates 0x1f4b58 and
  `ret(_identFindMaster)` (`call` at 0x10e8b4); its nonzero result takes `jne` to the same zero
  store, while zero falls through to storing 1 there (`jne` at 0x10e8be, `mov` at 0x10e8dd,
  `mov` at 0x10e8c0). Later effects include `_semGive` with first candidate `[0x1f4e4c]` and a
  four-byte clear at 0x1e0c34 (`call` at 0x10e99c, `mov` at 0x10e9a4).

## `_addrdProcessIdentityMsg`

- `_addrdProcessIdentityMsg`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10eb00 to 0x10ec30.
  Established. Starts with a call to `_addrdExtractIdentity` with candidates `[sp+4]` and
  `sp-32`, then calls `_ffscCompareUniqueIDs` with `sp-32` and 0x1f4b58 (`call` at 0x10eb10,
  `call` at 0x10eb1e). `jne` takes the path containing `_identUpdate` for a nonzero call result
  (`jne` at 0x10eb28, `call` at 0x10eb51). A zero value at 0x1e0c34 takes `je` past
  `_addrdStartElectionTimer` (`je` at 0x10eb66). Nonzero falls through to that call (`je` at
  0x10eb66, `call` at 0x10eb68). Another `je` skips `_addrdResolveConflicts` when `[0x1f50c4] ==
  0x0` (`je` at 0x10eba9, `call` at 0x10ebab). The signed-flag decision after `_identUpdate`
  remains unresolved (`jl` at 0x10eb5d).

## `_addrdProcessQueryMsg`

- `_addrdProcessQueryMsg`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ec30 to 0x10eda0.
  Established. Starts with `_clock_gettime` candidates zero and 0x1f1410, then
  `_addrdExtractIdentity` candidates `[sp+4]` and `sp-80` (`call` at 0x10ec42, `call` at
  0x10ec4f). Later calls include `_taskDelay` with first candidate 3 and `_addrdInsertIdentity`
  with `sp-52` and 0x1f4b58 (`call` at 0x10ecec, `call` at 0x10ed0b). Four-byte stack stores set
  values 3 and 1 at entry stack -0x34 and -0x30 (`mov` at 0x10ecf4, `mov` at 0x10ecfb).
  `_addrdBroadcastMsg` is followed by `jne`, which takes the 0xffffffff path for a nonzero
  result (`call` at 0x10ed14, `jne` at 0x10ed1e, `mov` at 0x10ed40); its zero-result fall-through calls `_addrdStartElectionTimer` (`call` at 0x10ed20). A nonzero timer result takes
  the following `jne` to the same path (`jne` at 0x10ed27, `mov` at 0x10ed40). The final fall-through stores 1 as four bytes at 0x1e0c34 (`mov` at 0x10ed29).

## `_addrdProcessRequestRackIDMsg`

- `_addrdProcessRequestRackIDMsg`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10eda0 to 0x10eef0.
  Established. Calls `_addrdExtractIdentity` with `[sp+4]` and `sp-32`, then `_identUpdate` with
  `sp-32` (`call` at 0x10edb1, `call` at 0x10edba). The `je` at 0x10ee1a skips the path
  containing `_identFindConflictingRackID` for `[0x1f50c4] == 0x0` (`je` at 0x10ee1a). That call
  has first candidate `sp-32` (`call` at 0x10ee20); its zero result takes `je` to the common
  exit (`je` at 0x10ee2c). `_memcpy` then has candidates `sp-60`,
  `ret(_identFindConflictingRackID)` and 0x1c (`call` at 0x10ee35). `_addrdFindUnusedRackID` is
  called, and `jne` skips the path containing `_ffscUniqueIDToString` and `_ffscMsg` for a
  result unequal to 0xffffffff (`call` at 0x10ee3d, `jne` at 0x10ee48). The following
  `_addrdAssignNewIdentity` call has first candidate `sp-60` (`call` at 0x10ee66).

## `_addrdAssignNewIdentity`

- `_addrdAssignNewIdentity`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e460 to 0x10e560.
  Established. Calls `_identFindByUniqueID` with first candidate `[sp+4]` (`call` at 0x10e46d). A
  nonzero result takes `jne` to the path containing `_ffscCompareUniqueIDs` (`jne` at 0x10e479).
  `_ffscCompareUniqueIDs` receives `[sp+4]` and 0x1f4b58 (`call` at 0x10e496); its nonzero
  result takes the next `jne` to the stack-store path (`jne` at 0x10e4a0). That path stores
  four-byte values 3 and 3 at entry stack -0x4c and -0x48, calls `_addrdInsertIdentity` with
  `sp-76` and `[sp+4]`, then `_addrdBroadcastMsg` with `sp-76` (`mov` at 0x10e4d0, `mov` at
  0x10e4d7, `call` at 0x10e4e3, `call` at 0x10e4ec). A zero broadcast result takes `je` to the
  later path containing `_memcpy`, whose candidates are `ret(_identFindByUniqueID)`, `[sp+4]`
  and 0x1c (`je` at 0x10e4f6, `call` at 0x10e543).

## `_addrdFindUnusedRackID`

- `_addrdFindUnusedRackID`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e6c0 to 0x10e750.
  Established. Loads EBX with 1 before a loop containing `_identFindByRackID` (`mov` at 0x10e6c4,
  `call` at 0x10e6cd). That call's first candidate value is unknown. `jne` enters the increment
  path for a nonzero call result, while zero falls through (`jne` at 0x10e6d7). The analysis
  does not resolve the increment's effects (`inc` at 0x10e6e0). `jbe` returns to the call when
  an unknown unsigned 32-bit value is at most 0x13 (`jbe` at 0x10e6e4). A larger value falls
  through to a 0xffffffff load (`jbe` at 0x10e6e4, `mov` at 0x10e6e6). The merged return value
  is unknown (`ret` at 0x10e6f1).

## `_addrdResolveRackIDConflict`

- `_addrdResolveRackIDConflict`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10f010 to 0x10f0c0.
  Established. Supplies `[sp+4]` and `[sp+8]` as candidates to `_ffscCompareUniqueIDs` (`call` at
  0x10f020). The following `jge` uses unresolved flags to choose a source route, so the merged
  second candidate at `_memcpy` is unknown (`jge` at 0x10f02a, `call` at 0x10f047); its first
  and third candidates are `sp-32` and 0x1c (`call` at 0x10f047). `_addrdFindUnusedRackID` is
  called and its result compared with 0xffffffff (`call` at 0x10f04f, `cmp` at 0x10f057).
  Equality takes `je` to the path containing `_ffscUniqueIDToString` and `_ffscMsg` (`je` at
  0x10f05a, `call` at 0x10f078, `call` at 0x10f086). Inequality falls through to
  `_addrdAssignNewIdentity` with first candidate `sp-32` (`je` at 0x10f05a, `call` at 0x10f060).

## `_addrdFindUnusedIPAddr`

- `_addrdFindUnusedIPAddr`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e660 to 0x10e6c0.
  Established. Initialises EBX to 0x10 and enters a region containing `_inet_makeaddr_b` and
  `_identFindByIPAddr` (`mov` at 0x10e668, `call` at 0x10e678, `call` at 0x10e681). The former
  has candidates `[0x1e1154]`, unknown and `[sp+4]` (`call` at 0x10e678). The latter's first
  candidate is `[sp+4]` (`call` at 0x10e681). A nonzero call result takes `jne` to an increment
  whose effects the analysis does not resolve (`jne` at 0x10e68b, `inc` at 0x10e6a0). The
  subsequent `jle` loops for an unknown signed 32-bit value at most 0xfffffe, and falls through
  for a greater value (`jle` at 0x10e6a7). The listed constant loads on the two exit routes are
  zero and 0xffffffff, but the merged EAX result is unknown (`xor` at 0x10e68d, `mov` at
  0x10e6a9, `ret` at 0x10e6b6).
