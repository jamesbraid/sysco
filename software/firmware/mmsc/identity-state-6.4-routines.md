# MMSC identity state in IRIX 6.4

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

[Fact sheets and full listings](disasm/identity-state-6.4-facts.txt) include all tool sections.

## `_identInit`

- `_identInit`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122898 to 0x122a18.
  Established. Four-byte stores clear absolute `0x1ec3ac` (`mov` at 0x1228aa) and `0x1ef76c` (`mov`
  at 0x1228b4). `_nvramRead` has leading candidates `0x1` and `0x1db848` (`call` at 0x122914). The
  branch takes `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` and falls through for nonzero (`je` at
  0x12291e). Later calls include `_inet_makeaddr_b` with candidates `[0x1db84c]`, `[0x1db848]`, and
  `0x1ef240` (`call` at 0x1229a5). The initialized identity and network policy remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1228aa | store | 4 | `abs 0x1ec3ac` | `0x0` |
| `mov` at 0x1228b4 | store | 4 | `abs 0x1ef76c` | `0x0` |
| `mov` at 0x1228be | store | 1 | `abs 0x1ef238` | `0x0` |
| `mov` at 0x1228c5 | store | 1 | `abs 0x1ef239` | `0x0` |
| `mov` at 0x1228d0 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x1228d6 | store | 1 | `(unknown)` | `?` |
| `mov` at 0x1228f0 | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x122903 | store | 4 | `abs 0x1ef248` | `0x1` |
| `test` at 0x122920 | load | 1 | `abs 0x1db371` | `-` |
| `movzx` at 0x122936 | load | 1 | `abs 0x1ef23d` | `-` |
| `movzx` at 0x122940 | load | 1 | `abs 0x1ef23e` | `-` |
| `movzx` at 0x12294c | load | 1 | `abs 0x1ef23f` | `-` |
| `mov` at 0x122955 | store | 4 | `abs 0x1db848` | `?` |
| `mov` at 0x122967 | store | 4 | `abs 0x1db848` | `0xf` |
| `mov` at 0x122971 | store | 4 | `abs 0x1db84c` | `0x64` |
| `mov` at 0x12297b | store | 4 | `abs 0x1db850` | `0xff000000` |
| `mov` at 0x122999 | load | 4 | `abs 0x1db848` | `-` |
| `mov` at 0x12299f | load | 4 | `abs 0x1db84c` | `-` |
| `mov` at 0x1229c0 | store | 4 | `abs 0x1ef244` | `0xffffffff` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1228a2 | `_bzero 0x1719c0` | `0x1ef238` | `0x1c` | `ebp` | `?` | `?` | `?` |
| `call` at 0x122914 | `_nvramRead 0x126da8` | `0x1` | `0x1db848` | `?` | `?` | `?` | `?` |
| `call` at 0x12292e | `_ffscMsg 0x11edf0` | `0x122869` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12298c | `_nvramWrite 0x126f38` | `0x1` | `0x1db848` | `?` | `?` | `?` | `?` |
| `call` at 0x1229a5 | `_inet_makeaddr_b 0x1870c0` | `[0x1db84c]` | `[0x1db848]` | `0x1ef240` | `?` | `?` | `?` |
| `call` at 0x1229b4 | `_nvramRead 0x126da8` | `0x2` | `0x1ef244` | `?` | `?` | `?` | `?` |
| `call` at 0x1229cf | `_lstInit 0x18e868` | `0x1d33a8` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1229dc | `_identUpdate 0x122d68` | `0x1ef238` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x1228e0 | `? <= 0x5 (signed, 32-bit)` | `? > 0x5 (signed, 32-bit)` |
| `jle` at 0x122901 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `je` at 0x12291e | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` |
| `je` at 0x122927 | `(([0x1db371]:1 & 0xff) & 0x1) == 0x0` | `(([0x1db371]:1 & 0xff) & 0x1) != 0x0` |
| `jbe` at 0x122965 | `? <= 0xffffee (unsigned, 32-bit)` | `? > 0xffffee (unsigned, 32-bit)` |
| `je` at 0x1229be | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` |
| `jl` at 0x1229e3 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1229ea | `eax` | `0x0` |
| `ret` at 0x122a00 | `eax` | `0xffffffff` |

## `_identUpdate`

- `_identUpdate`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122d68 to 0x122e98.
  Established. The `_identFindIdentityNodeByUniqueID` call has leading candidate `[sp+4]` (`call` at
  0x122d71). The result branch takes `(ret(_identFindIdentityNodeByUniqueID) &
  ret(_identFindIdentityNodeByUniqueID)) == 0x0` and falls through for nonzero (`je` at 0x122d7d).
  On the fall-through route, a second branch takes `(ret(_clock_gettime) &
  ret(_clock_gettime)) == 0x0` and falls through for nonzero (`je` at 0x122d8f). This taken route
  calls `_memcpy` with candidates `ret(_identFindIdentityNodeByUniqueID)+16`, `[sp+4]`, and `0x1c`
  (`call` at 0x122d9f). The taken route from the first branch calls `_malloc` with leading
  candidate `0x2c` (`call` at 0x122daa). The subsequent branch takes `(ret(_malloc) &
  ret(_malloc)) != 0x0` and falls through for zero (`jne` at 0x122db6). On that taken route,
  another branch takes `(ret(_clock_gettime) & ret(_clock_gettime)) == 0x0` and falls through for
  nonzero (`je` at 0x122de8). This taken route reaches `_lstAdd` with leading candidates
  `0x1d33a8` and `ret(_malloc)` (`call` at 0x122e1d). The identity-record and timestamp semantics
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x122d6d | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122d71 | `_identFindIdentityNodeByUniqueID 0x1232c8` | `[sp+4]` | `ebx` | `esi` | `ebp` | `?` | `?` |
| `call` at 0x122d85 | `_clock_gettime 0x17a2bc` | `0x0` | `ret(_identFindIdentityNodeByUniqueID)+8` | `?` | `?` | `?` | `?` |
| `call` at 0x122d9f | `_memcpy 0x17e840` | `ret(_identFindIdentityNodeByUniqueID)+16` | `[sp+4]` | `0x1c` | `?` | `?` | `?` |
| `call` at 0x122daa | `_malloc 0x17d318` | `0x2c` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122dbd | `_ffscMsgS 0x11eec0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122dde | `_clock_gettime 0x17a2bc` | `0x0` | `ret(_malloc)+8` | `?` | `?` | `?` | `?` |
| `call` at 0x122def | `_ffscMsgS 0x11eec0` | `0x122d28` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122df8 | `_free 0x17d338` | `ret(_malloc)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122e0f | `_memcpy 0x17e840` | `ret(_malloc)+16` | `[sp+4]` | `0x1c` | `?` | `?` | `?` |
| `call` at 0x122e1d | `_lstAdd 0x18e888` | `0x1d33a8` | `ret(_malloc)` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x122d7d | `(ret(_identFindIdentityNodeByUniqueID) & ret(_identFindIdentityNodeByUniqueID)) == 0x0` | `(ret(_identFindIdentityNodeByUniqueID) & ret(_identFindIdentityNodeByUniqueID)) != 0x0` |
| `je` at 0x122d8f | `(ret(_clock_gettime) & ret(_clock_gettime)) == 0x0` | `(ret(_clock_gettime) & ret(_clock_gettime)) != 0x0` |
| `jne` at 0x122db6 | `(ret(_malloc) & ret(_malloc)) != 0x0` | `(ret(_malloc) & ret(_malloc)) == 0x0` |
| `je` at 0x122de8 | `(ret(_clock_gettime) & ret(_clock_gettime)) == 0x0` | `(ret(_clock_gettime) & ret(_clock_gettime)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122e2f | `eax` | `?` |

## `_identCleanOldEntries`

- `_identCleanOldEntries`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122388 to 0x122428.
  Established. The `_ffscCompareTimestamps` call has leading candidates unknown and `[sp+4]` (`call`
  at 0x1223b7). A later branch takes `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) !=
  0x0` and falls through for zero (`jne` at 0x1223e9). That fall-through clears four bytes at
  absolute `0x1ec3ac` (`mov` at 0x1223eb) and `0x1ef76c` (`mov` at 0x1223f5). Calls include
  `_lstDelete` with leading candidates `0x1d33a8` and unknown (`call` at 0x122405). The age rule and
  global-state meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1223af | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1223d8 | load | 4 | `abs 0x1ec3ac` | `-` |
| `mov` at 0x1223eb | store | 4 | `abs 0x1ec3ac` | `0x0` |
| `mov` at 0x1223f5 | store | 4 | `abs 0x1ef76c` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122393 | `_lstFirst 0x18e9f8` | `0x1d33a8` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x1223a5 | `_lstNext 0x18eaa8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1223b7 | `_ffscCompareTimestamps 0x125218` | `?` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x1223cc | `_ffscCompareUniqueIDs 0x125268` | `?` | `0x1ef238` | `?` | `?` | `?` | `?` |
| `call` at 0x1223df | `_ffscCompareUniqueIDs 0x125268` | `?` | `[0x1ec3ac]` | `?` | `?` | `?` | `?` |
| `call` at 0x122405 | `_lstDelete 0x18e918` | `0x1d33a8` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12240e | `_free 0x17d338` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12239f | `(ret(_lstFirst) & ret(_lstFirst)) == 0x0` | `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` |
| `jge` at 0x1223c1 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x1223d6 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` |
| `jne` at 0x1223e9 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `jne` at 0x12241a | `(ret(_lstNext) & ret(_lstNext)) != 0x0` | `(ret(_lstNext) & ret(_lstNext)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122427 | `eax` | `0x0` |

## `_identFindMaster`

- `_identFindMaster`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122808 to 0x122898.
  Established. The `_lstFirst` call has leading candidate `0x1d33a8` (`call` at 0x122812). The
  branch takes `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` and falls through for zero (`jne` at
  0x12281e). A later `_ffscCompareUniqueIDs` call presents leading candidates `ret(_lstNext)+16` and
  unknown (`call` at 0x122840); its following branch takes `ge flags condition (operands unknown)`
  and falls through for the negation (`jge` at 0x12284a). The master-selection order and merged
  result remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122812 | `_lstFirst 0x18e9f8` | `0x1d33a8` | `ebx` | `esi` | `ebp` | `?` | `?` |
| `call` at 0x122840 | `_ffscCompareUniqueIDs 0x125268` | `ret(_lstNext)+16` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12284f | `_lstNext 0x18eaa8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x12281e | `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` | `(ret(_lstFirst) & ret(_lstFirst)) == 0x0` |
| `jge` at 0x12284a | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x12285b | `(ret(_lstNext) & ret(_lstNext)) != 0x0` | `(ret(_lstNext) & ret(_lstNext)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122868 | `eax` | `?` |

## `_identSetCapabilities`

- `_identSetCapabilities`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122fa8 to 0x123088.
  Established. A four-byte store writes `[sp+4]` at absolute `0x1ef248` (`mov` at 0x122fae). The
  branch takes `(([0x1db371]:1 & 0xff) & 0x1) == 0x0` and falls through for a nonzero expression
  (`je` at 0x122fba). The fall-through includes `_ffscMsg` with leading candidates `0x122f7d` and
  `[sp+4]` (`call` at 0x122fc2). EAX is zero at the exit (`ret` at 0x122fcc). The capability
  meanings are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x122fab | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122fae | store | 4 | `abs 0x1ef248` | `[sp+4]` |
| `test` at 0x122fb3 | load | 1 | `abs 0x1db371` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122fc2 | `_ffscMsg 0x11edf0` | `0x122f7d` | `[sp+4]` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x122fba | `(([0x1db371]:1 & 0xff) & 0x1) == 0x0` | `(([0x1db371]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122fcc | `eax` | `0x0` |

## `_identSetIPAddr`

- `_identSetIPAddr`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x123088 to 0x123178.
  Established. The `_ifAddrSet` call presents leading candidates `0x122ff3` and `sp-24` (`call` at
  0x1230d9); its branch takes `(ret(_ifAddrSet) & ret(_ifAddrSet)) == 0x0` and falls through for
  nonzero (`je` at 0x1230e3). The taken route includes four-byte stores of `[[sp+4]]` at absolute
  `0x1ef240` (`mov` at 0x1230fa), `ret(_inet_netof)` at `0x1db84c` (`mov` at 0x1230ff), and
  `[sp-28]` at `0x1db848` (`mov` at 0x123108). `_nvramWrite` has leading candidates `0x1` and
  `0x1db848` (`call` at 0x123115). The interface string and persistent configuration semantics
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x123091 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x123094 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x12309f | store | 4 | `sp -0x1c` | `ret(_inet_lnaof)` |
| `mov` at 0x1230a2 | load | 4 | `[sp+4] +0x0` | `-` |
| `cmp` at 0x1230af | load | 4 | `abs 0x1db84c` | `-` |
| `mov` at 0x1230c8 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x1230f8 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x1230fa | store | 4 | `abs 0x1ef240` | `[[sp+4]]` |
| `mov` at 0x1230ff | store | 4 | `abs 0x1db84c` | `ret(_inet_netof)` |
| `mov` at 0x123105 | load | 4 | `sp -0x1c` | `-` |
| `mov` at 0x123108 | store | 4 | `abs 0x1db848` | `[sp-28]` |
| `test` at 0x12312e | load | 1 | `abs 0x1db371` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x123097 | `_inet_lnaof 0x187050` | `[[sp+4]]` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x1230a5 | `_inet_netof 0x187190` | `[[sp+4]]` | `?` | `?` | `?` | `ret(_inet_lnaof)` | `?` |
| `call` at 0x1230bc | `_ffscMsg 0x11edf0` | `0x122fcd` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1230cb | `_inet_ntoa_b 0x187380` | `[[sp+4]]` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x1230d9 | `_ifAddrSet 0x187fd0` | `0x122ff3` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x1230eb | `_ffscMsgS 0x11eec0` | `0x122ff8` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x123115 | `_nvramWrite 0x126f38` | `0x1` | `0x1db848` | `?` | `?` | `?` | `?` |
| `call` at 0x123126 | `_ffscMsg 0x11edf0` | `0x123022` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12313d | `_ffscMsg 0x11edf0` | `0x123060` | `sp-24` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1230b5 | `[0x1db84c] == ret(_inet_netof)` | `[0x1db84c] != ret(_inet_netof)` |
| `je` at 0x1230e3 | `(ret(_ifAddrSet) & ret(_ifAddrSet)) == 0x0` | `(ret(_ifAddrSet) & ret(_ifAddrSet)) != 0x0` |
| `je` at 0x12311f | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` |
| `je` at 0x123135 | `(([0x1db371]:1 & 0xff) & 0x1) == 0x0` | `(([0x1db371]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12314d | `eax` | `?` |

## `_identSetModuleNums`

- `_identSetModuleNums`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x123178 to 0x123258.
  Established. The four-byte store has unknown effective target and value after loop merging (`mov`
  at 0x123193). The loop branch takes `be flags condition (operands unknown)` and falls through for
  its negation (`jbe` at 0x1231a0). The `_ffscModuleNumToString` call has leading candidates unknown
  and `sp-16` (`call` at 0x1231cd). The exit carries EAX zero (`ret` at 0x123204). The affected
  module-number records remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x123181 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x123190 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123193 | store | 4 | `(unknown)` | `?` |
| `test` at 0x1231a2 | load | 1 | `abs 0x1db371` | `-` |
| `mov` at 0x1231c5 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1231b0 | `_ffscMsgN 0x11ee50` | `0x12314e` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1231cd | `_ffscModuleNumToString 0x125478` | `?` | `sp-16` | `?` | `?` | `?` | `?` |
| `call` at 0x1231db | `_ffscMsgN 0x11ee50` | `0x123169` | `sp-16` | `?` | `?` | `?` | `?` |
| `call` at 0x1231f4 | `_ffscMsg 0x11edf0` | `0x122e3a` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x1231a0 | `be flags condition (operands unknown)` | `not (be flags condition (operands unknown))` |
| `je` at 0x1231a9 | `(([0x1db371]:1 & 0xff) & 0x1) == 0x0` | `(([0x1db371]:1 & 0xff) & 0x1) != 0x0` |
| `jbe` at 0x1231ed | `be flags condition (operands unknown)` | `not (be flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x123204 | `eax` | `0x0` |

## `_identSetRackID`

- `_identSetRackID`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x123258 to 0x1232c8.
  Established. A four-byte store places `[sp+4]` at absolute `0x1ef244` (`mov` at 0x123263).
  `_nvramWrite` receives leading candidates `0x2` and `0x1ef244` (`call` at 0x123270). The branch
  takes `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` and falls through for nonzero (`je` at
  0x12327a). After those paths join, a four-byte store sets absolute `0x1ebad8` to `0x1` (`mov` at
  0x123289). The rack-ID persistence and global flag interpretation remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x123260 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x123263 | store | 4 | `abs 0x1ef244` | `[sp+4]` |
| `mov` at 0x123289 | store | 4 | `abs 0x1ebad8` | `0x1` |
| `test` at 0x123293 | load | 1 | `abs 0x1db371` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x123270 | `_nvramWrite 0x126f38` | `0x2` | `0x1ef244` | `?` | `?` | `?` | `?` |
| `call` at 0x123281 | `_ffscMsg 0x11edf0` | `0x123205` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1232a1 | `_ffscRackIDToString 0x1257f8` | `[sp+4]` | `sp-16` | `?` | `?` | `?` | `?` |
| `call` at 0x1232af | `_ffscMsg 0x11edf0` | `0x123237` | `sp-16` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12327a | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` |
| `je` at 0x12329a | `(([0x1db371]:1 & 0xff) & 0x1) == 0x0` | `(([0x1db371]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1232be | `eax` | `0x0` |

## `_identFindByIPAddr`

- `_identFindByIPAddr`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1225e8 to 0x122638.
  Established. The `_lstFirst` call has leading candidate `0x1d33a8` (`call` at 0x1225f4), while
  `_lstNext` has unknown candidates (`call` at 0x122619). The comparison branch takes `Z == 0` and
  falls through for `not (Z == 0)` (`jne` at 0x12260f). EAX is unknown at the merged exit (`ret` at
  0x12262f). The IP-address match and returned-record role cannot be established from these
  unresolved expressions.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1225ec | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122608 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12260b | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x122629 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1225f4 | `_lstFirst 0x18e9f8` | `0x1d33a8` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x122619 | `_lstNext 0x18eaa8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x12260f | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x122625 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12262f | `eax` | `?` |

## `_identFindByModuleNum`

- `_identFindByModuleNum`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122638 to 0x1226a8.
  Established. The first branch takes `[sp+4] == 0xffffffff` and falls through on inequality (`je`
  at 0x122647). A later branch takes `([sp+8] & [sp+8]) == 0x0` and falls through for a nonzero
  expression (`je` at 0x12266a). That fall-through writes four bytes of unknown value at `[sp+8]
  +0x0` (`mov` at 0x12266c). The loop decision takes `? <= 0x1 (signed, 32-bit)` and falls through
  for larger values (`jle` at 0x12267c). The matched module and output index remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12263e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122641 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x122660 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12266c | store | 4 | `[sp+8] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12264e | `_lstFirst 0x18e9f8` | `0x1d33a8` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x12267f | `_lstNext 0x18eaa8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x122647 | `[sp+4] == 0xffffffff` | `[sp+4] != 0xffffffff` |
| `jne` at 0x122666 | `Z == 0` | `not (Z == 0)` |
| `je` at 0x12266a | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |
| `jle` at 0x12267c | `? <= 0x1 (signed, 32-bit)` | `? > 0x1 (signed, 32-bit)` |
| `jne` at 0x12268b | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122698 | `eax` | `?` |

## `_identFindByRackID`

- `_identFindByRackID`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1226a8 to 0x1226f8.
  Established. The branch takes `[sp+4] != 0xffffffff` and falls through for equality (`jne` at
  0x1226b2). The taken route includes `_lstFirst` with leading candidate `0x1d33a8` (`call` at
  0x1226cd). The match branch takes `Z == 1` and falls through for `not (Z == 1)` (`je` at
  0x1226dd). The result in EAX is unknown at the exit (`ret` at 0x1226f6). The rack-lookup semantics
  remain unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1226ac | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1226d8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1226f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1226cd | `_lstFirst 0x18e9f8` | `0x1d33a8` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x1226e0 | `_lstNext 0x18eaa8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x1226b2 | `[sp+4] != 0xffffffff` | `[sp+4] == 0xffffffff` |
| `je` at 0x1226dd | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x1226ec | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1226f6 | `eax` | `?` |

## `_identFindByUniqueID`

- `_identFindByUniqueID`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1226f8 to 0x122718.
  Established. Calls `_identFindIdentityNodeByUniqueID` with leading candidate `[sp+4]` (`call` at
  0x1226ff). The branch takes `(ret(_identFindIdentityNodeByUniqueID) &
  ret(_identFindIdentityNodeByUniqueID)) == 0x0` and falls through for nonzero (`je` at 0x122708);
  its merged exit value is unknown (`ret` at 0x122712). The returned-record mapping is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1226fb | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1226ff | `_identFindIdentityNodeByUniqueID 0x1232c8` | `[sp+4]` | `ebp` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x122708 | `(ret(_identFindIdentityNodeByUniqueID) & ret(_identFindIdentityNodeByUniqueID)) == 0x0` | `(ret(_identFindIdentityNodeByUniqueID) & ret(_identFindIdentityNodeByUniqueID)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122712 | `eax` | `?` |

## `_identFindConflictingIPAddr`

- `_identFindConflictingIPAddr`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122718 to 0x122788.
  Established. The entry branch takes `([sp+4] & [sp+4]) != 0x0` and falls through for zero (`jne`
  at 0x122723). The taken route includes `_ffscCompareUniqueIDs` with leading candidates unknown and
  `[sp+4]` (`call` at 0x12274d). A later comparison branch takes `Z == 1` and falls through for its
  negation (`je` at 0x122761). The returned conflict and the compared IP-address values remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12271e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122759 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12275c | load | 4 | `[sp+4] +0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12273d | `_lstFirst 0x18e9f8` | `0x1d33a8` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x12274d | `_ffscCompareUniqueIDs 0x125268` | `?` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x122764 | `_lstNext 0x18eaa8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x122723 | `([sp+4] & [sp+4]) != 0x0` | `([sp+4] & [sp+4]) == 0x0` |
| `je` at 0x122757 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` |
| `je` at 0x122761 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x122770 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12277d | `eax` | `?` |

## `_identFindConflictingRackID`

- `_identFindConflictingRackID`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122788 to 0x122808.
  Established. The first branch takes `([sp+4] & [sp+4]) == 0x0` and falls through for nonzero (`je`
  at 0x122793). Another branch takes `[[sp+4]+12] != 0xffffffff` and falls through for equality
  (`jne` at 0x12279b). The taken route includes `_ffscCompareUniqueIDs` with leading candidates
  unknown and `[sp+4]` (`call` at 0x1227cd). The later match branch takes `Z == 1` and falls through
  for its negation (`je` at 0x1227e1). The concrete conflicting record remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12278e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122795 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x1227d9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1227dc | load | 4 | `[sp+4] +0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1227bd | `_lstFirst 0x18e9f8` | `0x1d33a8` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x1227cd | `_ffscCompareUniqueIDs 0x125268` | `?` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x1227e4 | `_lstNext 0x18eaa8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x122793 | `([sp+4] & [sp+4]) == 0x0` | `([sp+4] & [sp+4]) != 0x0` |
| `jne` at 0x12279b | `[[sp+4]+12] != 0xffffffff` | `[[sp+4]+12] == 0xffffffff` |
| `je` at 0x1227d7 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` |
| `je` at 0x1227e1 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x1227f0 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1227fd | `eax` | `?` |

## `_identFindIdentityNodeByUniqueID`

- `_identFindIdentityNodeByUniqueID`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1232c8 to 0x123324.
  Established. The `_lstFirst` call has leading candidate `0x1d33a8` (`call` at 0x1232d5).
  `_ffscCompareUniqueIDs` presents leading candidates unknown and `[sp+4]` (`call` at 0x1232ed). The
  branch takes `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` and falls through
  for zero (`jne` at 0x1232f7). The fall-through reaches the merged exit, whose EAX value is unknown
  (`ret` at 0x123321). The identity-node mapping is not established by the unresolved operands.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1232cd | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1232d5 | `_lstFirst 0x18e9f8` | `0x1d33a8` | `ebx` | `esi` | `ebp` | `?` | `?` |
| `call` at 0x1232ed | `_ffscCompareUniqueIDs 0x125268` | `?` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x123309 | `_lstNext 0x18eaa8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x1232f7 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `jne` at 0x123315 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x123321 | `eax` | `?` |
