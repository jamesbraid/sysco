# MMSC 2.1 address-service message paths

These routines were read in `6.5.5/usr/cpu/firmware/mmscfw.bin`, MMSC 2.1.
Image SHA-256: `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
The x86 a.out load address is `0x108000`, as described in the [load map](load-map.md).
Each analysed span ends at the next text symbol. That inferred boundary can include literals.

Brackets denote loaded values and `sp` the entry stack pointer. Call candidates use
assumed cdecl32 locations with unknown arity. Entries establish the stated accesses
and call routes. Hardware roles, unresolved values and unexamined callee effects remain unknown.

[Fact sheets with decoded listings](disasm/address-service-2.1-facts.txt).

## `_addrdInit`

- `_addrdInit`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10db90 to 0x10dd20.
  Established. Calls `_pipeDevCreate` with candidates 0x10dac2, 0x15 and 0x3e (`call` at 0x10db9c);
  its zero result takes `je` into the open-call region; the nonzero route can also reach that region
  through `je` when `[ret(___errno)] == 0xd0006` (`je` at 0x10dba6, `je` at 0x10dbb4). Four-byte
  stores at 0x1b0780, 0x1f1418 and 0x1f4e4c record `ret(_open)`, `ret(_wdCreate)` and
  `ret(_semMCreate)` (`mov` at 0x10dbe1, `mov` at 0x10dc05, `mov` at 0x10dc47). `_taskSpawn` has
  candidates beginning 0x10db5f, 0x19, zero, 0x4e20, 0x10e150 and zero (`call` at 0x10dc87); its
  all-ones result takes `je` past `_etherInputHookAdd`; otherwise that call receives first candidate
  0x10e610 (`je` at 0x10dc92, `call` at 0x10dc99). The return after the hook call has EAX zero,
  while the other three recorded exits have 0xffffffff (`ret` at 0x10dca3, `ret` at 0x10dbc8, `ret`
  at 0x10dc20, `ret` at 0x10dcc2).

## `_addrd`

- `_addrd`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e150 to 0x10e460.
  Established. Known edges only. `_open` receives 0x10dac2, zero and zero (`call` at 0x10e165). The
  body includes `_select` candidates 0x100, `sp-132`, zero, zero and zero (`call` at 0x10e1f1); its
  following `jge` has unresolved flag operands (`jge` at 0x10e1fb). The later `_read` candidates are
  `ret(_open)`, `sp-68` and 0x3e (`call` at 0x10e247). `je` takes the later store-and-dispatch region for
  `ret(_read) == 0x3e`, otherwise falling through to `_ffscMsg` (`je` at 0x10e267, `call` at
  0x10e271). Four-byte store sites in that region include 2 at `sp -0x36`, zero at `sp -0x1a` and 3 at `sp
  -0x36` (`mov` at 0x10e2ba, `mov` at 0x10e2c7, `mov` at 0x10e2ce). Dispatch uses an unresolved
  indirect jump, so its case calls and argument values are not established here (`jmp` at 0x10e2e1).
  A later `jg` repeats from the selection region for signed `[sp-136] > 0` (`jg` at 0x10e398).

## `_addrdEtherInputHook`

- `_addrdEtherInputHook`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e610 to 0x10e660.
  Established. A two-byte load through `[sp+8] +0xc` controls `jne`, which selects the zero-load
  route for values unequal to 0x801 (`mov` at 0x10e61a, `jne` at 0x10e622, `xor` at 0x10e650).
  Equality falls through to `_write` with candidates `[0x1b0780]`, `[sp+8]` and `[sp+12]` (`jne` at
  0x10e622, `call` at 0x10e62c). The following `je` skips `_ffscMsgS` when `ret(_write) == [sp+12]`
  (`je` at 0x10e636, `call` at 0x10e63d). Both paths after the write call reach the EAX load of 1
  (`mov` at 0x10e642). The final merged EAX value is unresolved (`ret` at 0x10e658).

## `_addrdUpdatePartInfo`

- `_addrdUpdatePartInfo`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10d740 to 0x10d800.
  Established. Uses `_memset` candidates `sp-132`, zero and 0x80, then `_contains_module` candidates
  0x1b1030, `[[sp+4]+12]` and `[[sp+4]+8]` (`call` at 0x10d759, `call` at 0x10d76e). A nonzero
  contains-call result falls through `je` to a zero EAX load; zero takes the branch into the next
  test region (`je` at 0x10d778, `xor` at 0x10d77a). There, zero `[0x1b1030]` takes `je` to the
  all-ones EAX load (`je` at 0x10d787, `mov` at 0x10d7d0). The later `_sprintf` candidates begin
  `sp-132`, unknown, `[[sp+4]+12]`, `[[sp+4]+8]` (`call` at 0x10d7ae). `_update_partition_info`
  receives 0x1b1030 and `sp-132` (`call` at 0x10d7bf).

## `_addrdSendPartitionInfo`

- `_addrdSendPartitionInfo`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10d800 to 0x10d860.
  Established. Writes four-byte constants 3 and 7 at `sp -0x34` and `sp -0x30` (`mov` at 0x10d809,
  `mov` at 0x10d810). Three further four-byte stack stores use values `[[sp+4]+4]`, `[[sp+4]]` and
  `[[sp+4]+8]` at `sp -0x2c`, `sp -0x28` and `sp -0x24` (`mov` at 0x10d81a, `mov` at 0x10d81f, `mov`
  at 0x10d825). `_addrdBroadcastMsg` has first candidate `sp-52` (`call` at 0x10d82c). Nonzero
  result takes `jne` to `_ffscDebugWrite` candidates 0x10d7e1 and 0x13 (`jne` at 0x10d836, `call` at
  0x10d847). That route returns 0xffffffff; the zero-result fall-through returns zero (`ret` at
  0x10d854, `ret` at 0x10d83d).

## `_addrdBroadcastMsg`

- `_addrdBroadcastMsg`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10dec0 to 0x10df40.
  Established. The two `_bcopy` calls have candidates 0x1f4f68, `sp-14`, 6 and 0x1d73a8, `sp-20`, 6
  respectively (`call` at 0x10ded9, `call` at 0x10dee9). A two-byte store writes 0x108 at `sp -0x8`
  (`mov` at 0x10def1). `_etherOutput` receives `[0x1f1404]`, `sp-20`, `[sp+4]` and 0x30 (`call` at
  0x10df01); its nonzero result takes `jne` to the `_ffscMsgS` route and an all-ones EAX load (`jne`
  at 0x10df0b, `call` at 0x10df25, `mov` at 0x10df2a). Zero falls through to a zero EAX load (`jne`
  at 0x10df0b, `xor` at 0x10df0d).

## `_addrdInsertIdentity`

- `_addrdInsertIdentity`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10dfa0 to 0x10e020.
  Established. Includes a four-byte store with unknown effective address and value (`mov` at
  0x10dfb4). An unsupported increment precedes `jle`, taken backward for an unknown signed value at
  most 1 (`inc` at 0x10dfb8, `jle` at 0x10dfbc). The subsequent `_bcopy` calls receive `[sp+8]`,
  `[sp+4]+32`, 8 and `[sp+8]+8`, `[sp+4]+40`, 4 (`call` at 0x10dfc5, `call` at 0x10dfd7). Two
  four-byte stores write `[[sp+8]+12]` through `[sp+4] +0x2c` and `[[sp+8]+16]` through `[sp+4]
  +0x1c` (`mov` at 0x10dfdf, `mov` at 0x10dfe5). The return records `[[sp+8]+16]` in EAX (`ret` at
  0x10dff0).

## `_addrdLocalMsg`

- `_addrdLocalMsg`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e020 to 0x10e150.
  Established. Calls `_bzero` with `sp-68` and 0x3e, then stores two-byte 0x108 at `sp -0x38`
  (`call` at 0x10e031, `mov` at 0x10e039). `_bcopy` receives `[sp+4]`, `sp-54` and 0x30 (`call` at
  0x10e046). The `_write` candidates are `[0x1b0780]`, `sp-68` and 0x3e (`call` at 0x10e057). `jne`
  takes the diagnostic route for a return unequal to 0x3e; equality falls through to the zero EAX
  load (`jne` at 0x10e062, `call` at 0x10e075, `xor` at 0x10e064). The diagnostic route loads EAX
  with 0xffffffff (`mov` at 0x10e07a).

## `_addrdUpdateLocalIdentity`

- `_addrdUpdateLocalIdentity`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ddc0 to 0x10dec0.
  Established. Three equality branches skip individual calls when `[0x1f4b60] == [[sp+4]+8]`,
  `[0x1f4b64] == [[sp+4]+12]`, and `[0x1f4b68] == [[sp+4]+16]` (`je` at 0x10ddd6, `je` at 0x10ddf2,
  `je` at 0x10de0e). Their fall-through calls are `_identSetIPAddr` with `[sp+4]+8`,
  `_identSetRackID` with `[[sp+4]+12]`, and `_identSetCapabilities` with `[[sp+4]+16]` (`call` at
  0x10dddc, `call` at 0x10ddf8, `call` at 0x10de14). Call sites after the unresolved loop decisions include
  `_identSetModuleNums` with first candidate `[sp+4]+20` and `_identUpdate` with `[sp+4]` (`call` at
  0x10de54, `call` at 0x10de5d). A following `jl` has unknown flag operands (`jl` at 0x10de67). The
  other route writes four-byte 3 and 1 at `sp -0x34` and `sp -0x30`, calls `_addrdInsertIdentity`
  with `sp-52` and `[sp+4]`, then `_addrdBroadcastMsg` with `sp-52` (`mov` at 0x10de69, `mov` at
  0x10de70, `call` at 0x10de7c, `call` at 0x10de85). Zero broadcast result takes `je` to the zero
  EAX load; nonzero falls through to an all-ones load (`je` at 0x10de8c, `xor` at 0x10de3d, `mov` at
  0x10de8e).

## `_addrdExtractIdentity`

- `_addrdExtractIdentity`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10df40 to 0x10dfa0.
  Established. The indexed four-byte store has unresolved destination and value (`mov` at 0x10df54).
  The loop's unsupported increment is followed by backward `jle` for an unknown signed value at most
  1 (`inc` at 0x10df58, `jle` at 0x10df5c). `_bcopy` candidates are `[sp+4]+32`, `[sp+8]`, 8 and
  `[sp+4]+40`, `[sp+8]+8`, 4 (`call` at 0x10df65, `call` at 0x10df77). The final four-byte stores
  put `[[sp+4]+44]` through `[sp+8] +0xc` and `[[sp+4]+28]` through `[sp+8] +0x10` (`mov` at
  0x10df7f, `mov` at 0x10df85). EAX at the recorded return is `[[sp+4]+28]` (`ret` at 0x10df90).

## `_addrdProcessAssignMsg`

- `_addrdProcessAssignMsg`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e750 to 0x10e860.
  Established. Passes `[sp+4]` and `sp-32` to `_addrdExtractIdentity`, then `[sp+4]+32` and 0x1f4b58
  to `_ffscCompareUniqueIDs` (`call` at 0x10e760, `call` at 0x10e771). Zero comparison result takes
  `je` to `_addrdUpdateLocalIdentity` with first candidate `sp-32` (`je` at 0x10e77b, `call` at
  0x10e791). A zero update result takes `je` past `_ffscMsg`; nonzero falls through to it (`je` at
  0x10e79b, `call` at 0x10e7a2). The later byte test at 0x1e0be0 controls `je`, taken when bit 0 is
  clear to bypass `_ffscMsg` and `_identPrint` (`test` at 0x10e7aa, `je` at 0x10e7b1, `call` at
  0x10e7b8, `call` at 0x10e7c1). The recorded exit has EAX zero (`ret` at 0x10e7d0).

## `_addrdResolveConflicts`

- `_addrdResolveConflicts`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10eef0 to 0x10ef70.
  Established. Takes `jne` into the scan-call region for nonzero `[0x1f50c4]`; zero falls through to
  `_ffscMsg` and a return of 0xffffffff (`jne` at 0x10eefa, `call` at 0x10ef01, `ret` at 0x10ef0e).
  `_identScanForConflicts` receives candidates 0x10ef70 and 0x10f010 (`call` at 0x10ef1a); its zero
  result falls through the following `jne` to a return of zero (`jne` at 0x10ef24, `ret` at
  0x10ef2b). Nonzero takes the branch to another `_ffscMsg` and a return of 0xffffffff (`jne` at
  0x10ef24, `call` at 0x10ef35, `ret` at 0x10ef42).

## `_addrdResolveIPAddrConflict`

- `_addrdResolveIPAddrConflict`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ef70 to 0x10f010.
  Established. Calls `_ffscCompareUniqueIDs` with `[sp+4]` and `[sp+8]` (`call` at 0x10ef80). The
  following `jge` has unknown flag operands, and the merged `_memcpy` source candidate is unknown;
  its other candidates are `sp-32` and 0x1c (`jge` at 0x10ef8a, `call` at 0x10efa7).
  `_addrdFindUnusedIPAddr` receives `sp-24` (`call` at 0x10efb6). Nonzero result takes `jne` to
  `_ffscMsg` and EAX load 1 (`jne` at 0x10efc0, `call` at 0x10efd5, `mov` at 0x10efda). Zero falls
  through to `_addrdAssignNewIdentity` with first candidate `sp-32` (`jne` at 0x10efc0, `call` at
  0x10efc3). The shared exit value is unresolved (`ret` at 0x10efe7).

## `_identSetIPAddr`

- `_identSetIPAddr`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x125d60 to 0x125e50.
  Established. Both `_inet_lnaof` and `_inet_netof` receive `[[sp+4]]` as their first candidate
  (`call` at 0x125d6f, `call` at 0x125d7d). `_inet_ntoa_b` has candidates `[[sp+4]]` and `sp-24`,
  followed by `_ifAddrSet` candidates 0x125ccb and `sp-24` (`call` at 0x125da3, `call` at 0x125db1).
  Zero `_ifAddrSet` result takes `je` into the store region, while nonzero falls through to
  `_ffscMsgS` and an all-ones EAX load (`je` at 0x125dbb, `call` at 0x125dc3, `mov` at 0x125dc8).
  Three four-byte stores in the zero-result region write `[[sp+4]]` at 0x1f4b60, `ret(_inet_netof)`
  at 0x1e1154 and `[sp-28]` at 0x1e1150 (`mov` at 0x125dd2, `mov` at 0x125dd7, `mov` at 0x125de0).
  `_nvramWrite` then receives 1 and 0x1e1150 (`call` at 0x125ded); its zero result takes `je` past
  the next `_ffscMsg`, while nonzero falls through to it (`je` at 0x125df7, `call` at 0x125dfe).

## `_identFindByModuleNum`

- `_identFindByModuleNum`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x125300 to 0x125370.
  Established. `[sp+4]` equal to 0xffffffff takes `je` to the zero EAX load (`je` at 0x12530f, `xor`
  at 0x125355). Otherwise `_lstFirst` receives first candidate 0x1d8c78 (`je` at 0x12530f, `call` at
  0x125316). The indexed four-byte load and the optional four-byte store retain unknown effective
  address and unknown value respectively; the store target is `[sp+8] +0x0` (`mov` at 0x125328,
  `mov` at 0x125334). Zero `[sp+8]` takes `je` past that store (`je` at 0x125332). After an
  unsupported increment, `jle` loops for an unknown signed value at most 1 (`inc` at 0x125340, `jle`
  at 0x125344). The later `_lstNext` has unknown first candidate, and the backward `jne` retains
  only `Z == 0` with unresolved compared values (`call` at 0x125347, `jne` at 0x125353).
