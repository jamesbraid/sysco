# MMSC identity and conflict-handling routines in IRIX 6.5.2

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

[Fact sheets and full listings](disasm/identity-6.5.2-facts.txt) include all tool sections.

## `_identInit`

- `_identInit`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122cc8 to 0x122e48.
  Established. The later `_nvramRead` call has first candidates 2 and 0x1f0d0c (`call` at 0x122de4);
  its zero result takes `je` past the four-byte 0xffffffff store at 0x1f0d0c, while nonzero falls
  through to that store (`je` at 0x122dee, `mov` at 0x122df0). Earlier explicit four-byte stores
  include 0x64 at 0x1dd30c and 0xff000000 at 0x1dd310 (`mov` at 0x122da1, `mov` at 0x122dab).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x122cda | store | 4 | `abs 0x1ede74` | `0x0` |
| `mov` at 0x122ce4 | store | 4 | `abs 0x1f1244` | `0x0` |
| `mov` at 0x122cee | store | 1 | `abs 0x1f0d00` | `0x0` |
| `mov` at 0x122cf5 | store | 1 | `abs 0x1f0d01` | `0x0` |
| `mov` at 0x122d00 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x122d06 | store | 1 | `(unknown)` | `?` |
| `mov` at 0x122d20 | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x122d33 | store | 4 | `abs 0x1f0d10` | `0x1` |
| `test` at 0x122d50 | load | 1 | `abs 0x1dcdc9` | `-` |
| `movzx` at 0x122d66 | load | 1 | `abs 0x1f0d05` | `-` |
| `movzx` at 0x122d70 | load | 1 | `abs 0x1f0d06` | `-` |
| `movzx` at 0x122d7c | load | 1 | `abs 0x1f0d07` | `-` |
| `mov` at 0x122d85 | store | 4 | `abs 0x1dd308` | `?` |
| `mov` at 0x122d97 | store | 4 | `abs 0x1dd308` | `0xf` |
| `mov` at 0x122da1 | store | 4 | `abs 0x1dd30c` | `0x64` |
| `mov` at 0x122dab | store | 4 | `abs 0x1dd310` | `0xff000000` |
| `mov` at 0x122dc9 | load | 4 | `abs 0x1dd308` | `-` |
| `mov` at 0x122dcf | load | 4 | `abs 0x1dd30c` | `-` |
| `mov` at 0x122df0 | store | 4 | `abs 0x1f0d0c` | `0xffffffff` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122cd2 | `_bzero 0x172778` | `0x1f0d00` | `0x1c` | `ebp` | `?` | `?` | `?` |
| `call` at 0x122d44 | `_nvramRead 0x1270f8` | `0x1` | `0x1dd308` | `?` | `?` | `?` | `?` |
| `call` at 0x122d5e | `_ffscMsg 0x11f0d0` | `0x122c99` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122dbc | `_nvramWrite 0x127288` | `0x1` | `0x1dd308` | `?` | `?` | `?` | `?` |
| `call` at 0x122dd5 | `_inet_makeaddr_b 0x188ad0` | `[0x1dd30c]` | `[0x1dd308]` | `0x1f0d08` | `?` | `?` | `?` |
| `call` at 0x122de4 | `_nvramRead 0x1270f8` | `0x2` | `0x1f0d0c` | `?` | `?` | `?` | `?` |
| `call` at 0x122dff | `_lstInit 0x190278` | `0x1d4e68` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122e0c | `_identUpdate 0x123198` | `0x1f0d00` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x122d10 | `? <= 0x5 (signed, 32-bit)` | `? > 0x5 (signed, 32-bit)` |
| `jle` at 0x122d31 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `je` at 0x122d4e | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` |
| `je` at 0x122d57 | `(([0x1dcdc9]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dcdc9]:1 & 0xff) & 0x1) != 0x0` |
| `jbe` at 0x122d95 | `? <= 0xffffee (unsigned, 32-bit)` | `? > 0xffffee (unsigned, 32-bit)` |
| `je` at 0x122dee | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` |
| `jl` at 0x122e13 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122e1a | `eax` | `0x0` |
| `ret` at 0x122e30 | `eax` | `0xffffffff` |

## `_identSetRackID`

- `_identSetRackID`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x123688 to 0x1236f8.
  Established. Copies four bytes from `[sp+4]` to 0x1f0d0c before calling `_nvramWrite` with
  candidates beginning 2 and 0x1f0d0c (`mov` at 0x123693, `call` at 0x1236a0). A later four-byte
  store sets 0x1ed5a0 to 1 (`mov` at 0x1236b9). The return records EAX zero (`ret` at 0x1236ee).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x123690 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x123693 | store | 4 | `abs 0x1f0d0c` | `[sp+4]` |
| `mov` at 0x1236b9 | store | 4 | `abs 0x1ed5a0` | `0x1` |
| `test` at 0x1236c3 | load | 1 | `abs 0x1dcdc9` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1236a0 | `_nvramWrite 0x127288` | `0x2` | `0x1f0d0c` | `?` | `?` | `?` | `?` |
| `call` at 0x1236b1 | `_ffscMsg 0x11f0d0` | `0x123635` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1236d1 | `_ffscRackIDToString 0x125c28` | `[sp+4]` | `sp-16` | `?` | `?` | `?` | `?` |
| `call` at 0x1236df | `_ffscMsg 0x11f0d0` | `0x123667` | `sp-16` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1236aa | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` |
| `je` at 0x1236ca | `(([0x1dcdc9]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dcdc9]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1236ee | `eax` | `0x0` |

## `_identSetCapabilities`

- `_identSetCapabilities`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1233d8 to 0x1234b8.
  Established. The explicit four-byte store writes `[sp+4]` at 0x1f0d10 (`mov` at 0x1233de). `je`
  bypasses `_ffscMsg` when `(([0x1dcdc9]:1 & 0xff) & 0x1) == 0x0`, while inequality falls through to
  the call (`je` at 0x1233ea, `call` at 0x1233f2).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1233db | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1233de | store | 4 | `abs 0x1f0d10` | `[sp+4]` |
| `test` at 0x1233e3 | load | 1 | `abs 0x1dcdc9` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1233f2 | `_ffscMsg 0x11f0d0` | `0x1233ad` | `[sp+4]` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1233ea | `(([0x1dcdc9]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dcdc9]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1233fc | `eax` | `0x0` |

## `_identSetModuleNums`

- `_identSetModuleNums`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1235a8 to 0x123688.
  Established. The indexed load and store have unknown targets and values (`mov` at 0x1235c0, `mov`
  at 0x1235c3). The two `_ffscMsgN` sites have first candidates 0x12357e and 0x123599, with `sp-16`
  as the second candidate at the latter (`call` at 0x1235e0, `call` at 0x12360b).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1235b1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1235c0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1235c3 | store | 4 | `(unknown)` | `?` |
| `test` at 0x1235d2 | load | 1 | `abs 0x1dcdc9` | `-` |
| `mov` at 0x1235f5 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1235e0 | `_ffscMsgN 0x11f170` | `0x12357e` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1235fd | `_ffscModuleNumToString 0x1258a8` | `?` | `sp-16` | `?` | `?` | `?` | `?` |
| `call` at 0x12360b | `_ffscMsgN 0x11f170` | `0x123599` | `sp-16` | `?` | `?` | `?` | `?` |
| `call` at 0x123624 | `_ffscMsg 0x11f0d0` | `0x12326a` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x1235d0 | `be flags condition (operands unknown)` | `not (be flags condition (operands unknown))` |
| `je` at 0x1235d9 | `(([0x1dcdc9]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dcdc9]:1 & 0xff) & 0x1) != 0x0` |
| `jbe` at 0x12361d | `be flags condition (operands unknown)` | `not (be flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x123634 | `eax` | `0x0` |

## `_identUpdate`

- `_identUpdate`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x123198 to 0x1232c8.
  Established. The `_clock_gettime` sites have second candidates
  `ret(_identFindIdentityNodeByUniqueID)+8` and `ret(_malloc)+8`, each preceded by first candidate
  zero (`call` at 0x1231b5, `call` at 0x12320e). The `_malloc` first candidate is 0x2c (`call` at
  0x1231da).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12319d | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1231a1 | `_identFindIdentityNodeByUniqueID 0x1236f8` | `[sp+4]` | `ebx` | `esi` | `ebp` | `?` | `?` |
| `call` at 0x1231b5 | `_clock_gettime 0x17bccc` | `0x0` | `ret(_identFindIdentityNodeByUniqueID)+8` | `?` | `?` | `?` | `?` |
| `call` at 0x1231cf | `_memcpy 0x180250` | `ret(_identFindIdentityNodeByUniqueID)+16` | `[sp+4]` | `0x1c` | `?` | `?` | `?` |
| `call` at 0x1231da | `_malloc 0x17ed28` | `0x2c` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1231ed | `_ffscMsgS 0x11f200` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12320e | `_clock_gettime 0x17bccc` | `0x0` | `ret(_malloc)+8` | `?` | `?` | `?` | `?` |
| `call` at 0x12321f | `_ffscMsgS 0x11f200` | `0x123158` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x123228 | `_free 0x17ed48` | `ret(_malloc)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12323f | `_memcpy 0x180250` | `ret(_malloc)+16` | `[sp+4]` | `0x1c` | `?` | `?` | `?` |
| `call` at 0x12324d | `_lstAdd 0x190298` | `0x1d4e68` | `ret(_malloc)` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1231ad | `(ret(_identFindIdentityNodeByUniqueID) & ret(_identFindIdentityNodeByUniqueID)) == 0x0` | `(ret(_identFindIdentityNodeByUniqueID) & ret(_identFindIdentityNodeByUniqueID)) != 0x0` |
| `je` at 0x1231bf | `(ret(_clock_gettime) & ret(_clock_gettime)) == 0x0` | `(ret(_clock_gettime) & ret(_clock_gettime)) != 0x0` |
| `jne` at 0x1231e6 | `(ret(_malloc) & ret(_malloc)) != 0x0` | `(ret(_malloc) & ret(_malloc)) == 0x0` |
| `je` at 0x123218 | `(ret(_clock_gettime) & ret(_clock_gettime)) == 0x0` | `(ret(_clock_gettime) & ret(_clock_gettime)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12325f | `eax` | `?` |

## `_identScanForConflicts`

- `_identScanForConflicts`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122f08 to 0x123198.
  Established. Four indirect call targets remain unknown (`call` at 0x122f59, `call` at 0x122fae,
  `call` at 0x123038, `call` at 0x12307d). At the second and fourth sites, the second candidate
  renders as `ret(_lstNext)+16` (`call` at 0x122fae, `call` at 0x12307d). The three byte-test sites
  read 0x1dcdc8 (`test` at 0x122f0e, `test` at 0x122ff4, `test` at 0x1230d3).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `test` at 0x122f0e | load | 1 | `abs 0x1dcdc8` | `-` |
| `mov` at 0x122f40 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122f56 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122f88 | load | 4 | `ret(_lstNext) +0x18` | `-` |
| `mov` at 0x122f99 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122f9c | load | 4 | `ret(_lstNext) +0x18` | `-` |
| `mov` at 0x122fab | load | 4 | `sp +0x4` | `-` |
| `test` at 0x122ff4 | load | 1 | `abs 0x1dcdc8` | `-` |
| `mov` at 0x123028 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123035 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x123068 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12306b | load | 4 | `ret(_lstNext) +0x1c` | `-` |
| `mov` at 0x12307a | load | 4 | `sp +0x8` | `-` |
| `test` at 0x1230d3 | load | 1 | `abs 0x1dcdc8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122f1c | `_ffscMsg 0x11f0d0` | `0x122e99` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x122f2b | `_lstFirst 0x190408` | `0x1d4e68` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122f44 | `_inet_lnaof 0x188a60` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122f59 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122f8c | `_inet_lnaof 0x188a60` | `[ret(_lstNext)+24]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122fae | `(unknown callee) (unknown target)` | `?` | `ret(_lstNext)+16` | `?` | `?` | `?` | `?` |
| `call` at 0x122fc3 | `_lstNext 0x1904b8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122fd6 | `_lstNext 0x1904b8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x123002 | `_ffscMsg 0x11f0d0` | `0x122ec3` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x123013 | `_lstFirst 0x190408` | `0x1d4e68` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x123038 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12307d | `(unknown callee) (unknown target)` | `?` | `ret(_lstNext)+16` | `?` | `?` | `?` | `?` |
| `call` at 0x1230a2 | `_lstNext 0x1904b8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1230b5 | `_lstNext 0x1904b8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1230e1 | `_ffscMsg 0x11f0d0` | `0x122ee9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x122f15 | `(([0x1dcdc8]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dcdc8]:1 & 0xff) & 0x1) != 0x0` |
| `je` at 0x122f37 | `(ret(_lstFirst) & ret(_lstFirst)) == 0x0` | `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` |
| `jg` at 0x122f4f | `ret(_inet_lnaof) > 0xf (signed, 32-bit)` | `ret(_inet_lnaof) <= 0xf (signed, 32-bit)` |
| `jl` at 0x122f60 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jne` at 0x122f66 | `(ret(unknown) & ret(unknown)) != 0x0` | `(ret(unknown) & ret(unknown)) == 0x0` |
| `jle` at 0x122f97 | `ret(_inet_lnaof) <= 0xf (signed, 32-bit)` | `ret(_inet_lnaof) > 0xf (signed, 32-bit)` |
| `jne` at 0x122fa1 | `Z == 0` | `not (Z == 0)` |
| `jl` at 0x122fb5 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jne` at 0x122fbb | `(ret(unknown) & ret(unknown)) != 0x0` | `(ret(unknown) & ret(unknown)) == 0x0` |
| `je` at 0x122fcf | `(ret(_lstNext) & ret(_lstNext)) == 0x0` | `(ret(_lstNext) & ret(_lstNext)) != 0x0` |
| `je` at 0x122fd3 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x122fe2 | `(ret(_lstNext) & ret(_lstNext)) == 0x0` | `(ret(_lstNext) & ret(_lstNext)) != 0x0` |
| `je` at 0x122fe6 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x122fee | `Z == 0` | `not (Z == 0)` |
| `je` at 0x122ffb | `(([0x1dcdc8]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dcdc8]:1 & 0xff) & 0x1) != 0x0` |
| `je` at 0x12301f | `(ret(_lstFirst) & ret(_lstFirst)) == 0x0` | `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` |
| `jne` at 0x12302e | `Z == 0` | `not (Z == 0)` |
| `jl` at 0x12303f | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jne` at 0x123041 | `(ret(unknown) & ret(unknown)) != 0x0` | `(ret(unknown) & ret(unknown)) == 0x0` |
| `jne` at 0x123070 | `Z == 0` | `not (Z == 0)` |
| `jge` at 0x123084 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x12309a | `(ret(unknown) & ret(unknown)) != 0x0` | `(ret(unknown) & ret(unknown)) == 0x0` |
| `je` at 0x1230ae | `(ret(_lstNext) & ret(_lstNext)) == 0x0` | `(ret(_lstNext) & ret(_lstNext)) != 0x0` |
| `je` at 0x1230b2 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x1230c1 | `(ret(_lstNext) & ret(_lstNext)) == 0x0` | `(ret(_lstNext) & ret(_lstNext)) != 0x0` |
| `je` at 0x1230c5 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x1230cd | `Z == 0` | `not (Z == 0)` |
| `je` at 0x1230da | `(([0x1dcdc8]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dcdc8]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1230f1 | `eax` | `?` |

## `_identFindMaster`

- `_identFindMaster`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122c38 to 0x122cc8.
  Established. The `_lstFirst` candidates begin 0x1d4e68 (`call` at 0x122c42). `jne` takes its
  traversal route when `(ret(_lstFirst) & ret(_lstFirst)) != 0x0`, and zero falls through to the
  exit (`jne` at 0x122c4e). The later `_ffscCompareUniqueIDs` first candidate is `ret(_lstNext)+16`
  (`call` at 0x122c70).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122c42 | `_lstFirst 0x190408` | `0x1d4e68` | `ebx` | `esi` | `ebp` | `?` | `?` |
| `call` at 0x122c70 | `_ffscCompareUniqueIDs 0x125698` | `ret(_lstNext)+16` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122c7f | `_lstNext 0x1904b8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x122c4e | `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` | `(ret(_lstFirst) & ret(_lstFirst)) == 0x0` |
| `jge` at 0x122c7a | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x122c8b | `(ret(_lstNext) & ret(_lstNext)) != 0x0` | `(ret(_lstNext) & ret(_lstNext)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122c98 | `eax` | `?` |

## `_identFindConflictingRackID`

- `_identFindConflictingRackID`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122bb8 to 0x122c38.
  Established. Two separate four-byte reads render as `[sp+4] +0xc` (`mov` at 0x122bc5, `mov` at
  0x122c0c). The earlier `jne` selects the `_lstFirst` route when `[[sp+4]+12] != 0xffffffff`, with
  equality falling through to the exit (`jne` at 0x122bcb, `call` at 0x122bed).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x122bbe | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122bc5 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x122c09 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122c0c | load | 4 | `[sp+4] +0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122bed | `_lstFirst 0x190408` | `0x1d4e68` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x122bfd | `_ffscCompareUniqueIDs 0x125698` | `?` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x122c14 | `_lstNext 0x1904b8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x122bc3 | `([sp+4] & [sp+4]) == 0x0` | `([sp+4] & [sp+4]) != 0x0` |
| `jne` at 0x122bcb | `[[sp+4]+12] != 0xffffffff` | `[[sp+4]+12] == 0xffffffff` |
| `je` at 0x122c07 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` |
| `je` at 0x122c11 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x122c20 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122c2d | `eax` | `?` |

## `_identFindConflictingIPAddr`

- `_identFindConflictingIPAddr`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122b48 to 0x122bb8.
  Established. The `_ffscCompareUniqueIDs` call has unknown first candidate and `[sp+4]` as its
  second (`call` at 0x122b7d). Later four-byte reads have unknown target and `[sp+4] +0x8`
  respectively (`mov` at 0x122b89, `mov` at 0x122b8c). The deciding `je` takes its route for `Z ==
  1` (`je` at 0x122b91).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x122b4e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122b89 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122b8c | load | 4 | `[sp+4] +0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122b6d | `_lstFirst 0x190408` | `0x1d4e68` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x122b7d | `_ffscCompareUniqueIDs 0x125698` | `?` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x122b94 | `_lstNext 0x1904b8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x122b53 | `([sp+4] & [sp+4]) != 0x0` | `([sp+4] & [sp+4]) == 0x0` |
| `je` at 0x122b87 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` |
| `je` at 0x122b91 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x122ba0 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122bad | `eax` | `?` |

## `_identEnumerateRacks`

- `_identEnumerateRacks`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122968 to 0x122a18.
  Established. The three indexed four-byte stores retain unknown targets and values (`mov` at
  0x12299a, `mov` at 0x1229eb, `mov` at 0x1229ee). The later `add` reads and writes four bytes at
  `sp -0x10`, storing `[sp-16]+4` (`add` at 0x1229f1). The backward `jl` is taken for `[sp-16] <
  [sp-12] (signed, 32-bit)` (`jl` at 0x1229fc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x122971 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12298c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122994 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122997 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12299a | store | 4 | `(unknown)` | `?` |
| `mov` at 0x1229b6 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x1229c3 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1229c9 | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x1229cc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1229d2 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x1229d8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1229db | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1229de | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1229e5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1229e8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1229eb | store | 4 | `(unknown)` | `?` |
| `mov` at 0x1229ee | store | 4 | `(unknown)` | `?` |
| `add` at 0x1229f1 | load | 4 | `sp -0x10` | `-` |
| `add` at 0x1229f1 | store | 4 | `sp -0x10` | `[sp-16]+4` |
| `mov` at 0x1229f6 | load | 4 | `sp -0xc` | `-` |
| `cmp` at 0x1229f9 | load | 4 | `sp -0x10` | `-` |
| `cmp` at 0x1229ff | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12297b | `_lstFirst 0x190408` | `0x1d4e68` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x12299f | `_lstNext 0x1904b8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12298a | `Z == 1` | `not (Z == 1)` |
| `je` at 0x122992 | `Z == 1` | `not (Z == 1)` |
| `jle` at 0x1229ab | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jge` at 0x1229b4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jge` at 0x1229c1 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jae` at 0x1229e3 | `C == 0` | `not (C == 0)` |
| `jl` at 0x1229fc | `[sp-16] < [sp-12] (signed, 32-bit)` | `[sp-16] >= [sp-12] (signed, 32-bit)` |
| `jg` at 0x122a02 | `[sp-8] > ? (signed, 32-bit)` | `[sp-8] <= ? (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122a0f | `eax` | `?` |

## `_identEnumerateModules`

- `_identEnumerateModules`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1228a8 to 0x122968.
  Established. The indexed four-byte store sites have unknown targets and values (`mov` at 0x1228e4,
  `mov` at 0x12293b, `mov` at 0x12293e). `jae` bypasses the latter pair for `C == 0` and falls
  through for `not (C == 0)` (`jae` at 0x122933, `mov` at 0x12293b, `mov` at 0x12293e). The recorded
  EAX return value is unknown (`ret` at 0x12295f).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1228b1 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1228d4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1228dd | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1228e1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1228e4 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x122906 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x122913 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122919 | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x12291c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122922 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x122928 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12292b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12292e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122935 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122938 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12293b | store | 4 | `(unknown)` | `?` |
| `mov` at 0x12293e | store | 4 | `(unknown)` | `?` |
| `add` at 0x122941 | load | 4 | `sp -0x10` | `-` |
| `add` at 0x122941 | store | 4 | `sp -0x10` | `[sp-16]+4` |
| `mov` at 0x122946 | load | 4 | `sp -0xc` | `-` |
| `cmp` at 0x122949 | load | 4 | `sp -0x10` | `-` |
| `cmp` at 0x12294f | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1228bb | `_lstFirst 0x190408` | `0x1d4e68` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x1228ef | `_lstNext 0x1904b8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1228ca | `Z == 1` | `not (Z == 1)` |
| `jg` at 0x1228d2 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `je` at 0x1228db | `Z == 1` | `not (Z == 1)` |
| `jle` at 0x1228ec | `? <= 0x1 (signed, 32-bit)` | `? > 0x1 (signed, 32-bit)` |
| `jle` at 0x1228fb | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jge` at 0x122904 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jge` at 0x122911 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jae` at 0x122933 | `C == 0` | `not (C == 0)` |
| `jl` at 0x12294c | `[sp-16] < [sp-12] (signed, 32-bit)` | `[sp-16] >= [sp-12] (signed, 32-bit)` |
| `jg` at 0x122952 | `[sp-8] > ? (signed, 32-bit)` | `[sp-8] <= ? (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12295f | `eax` | `?` |

## `_identFindByRackID`

- `_identFindByRackID`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122ad8 to 0x122b28.
  Established. The four-byte input read renders as `sp +0x4` (`mov` at 0x122adc). `jne` takes the
  `_lstFirst` route for `[sp+4] != 0xffffffff`, with equality falling through to the exit (`jne`
  at 0x122ae2, `call` at 0x122afd). The later four-byte read target remains unresolved (`mov` at
  0x122b08).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x122adc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122b08 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122b20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122afd | `_lstFirst 0x190408` | `0x1d4e68` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x122b10 | `_lstNext 0x1904b8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x122ae2 | `[sp+4] != 0xffffffff` | `[sp+4] == 0xffffffff` |
| `je` at 0x122b0d | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x122b1c | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122b26 | `eax` | `?` |

## `_identFindByIPAddr`

- `_identFindByIPAddr`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122a18 to 0x122a68.
  Established. A four-byte read through `[sp+4] +0x0` follows a four-byte read with unknown target
  (`mov` at 0x122a3b, `mov` at 0x122a38). The following `jne` is taken for `Z == 0` and falls
  through for `not (Z == 0)` (`jne` at 0x122a3f).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x122a1c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122a38 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122a3b | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x122a59 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122a24 | `_lstFirst 0x190408` | `0x1d4e68` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x122a49 | `_lstNext 0x1904b8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x122a3f | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x122a55 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122a5f | `eax` | `?` |

## `_identFindByUniqueID`

- `_identFindByUniqueID`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122b28 to 0x122b48.
  Established. The `_identFindIdentityNodeByUniqueID` first candidate is `[sp+4]` (`call` at
  0x122b2f); its zero result takes `je` past the address calculation, and nonzero falls through
  (`je` at 0x122b38). The EAX result at the recorded return remains unknown (`ret` at 0x122b42).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x122b2b | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122b2f | `_identFindIdentityNodeByUniqueID 0x1236f8` | `[sp+4]` | `ebp` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x122b38 | `(ret(_identFindIdentityNodeByUniqueID) & ret(_identFindIdentityNodeByUniqueID)) == 0x0` | `(ret(_identFindIdentityNodeByUniqueID) & ret(_identFindIdentityNodeByUniqueID)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122b42 | `eax` | `?` |

## `_identCleanOldEntries`

- `_identCleanOldEntries`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1227b8 to 0x122858.
  Established. Separate four-byte stores clear 0x1ede74 and 0x1f1244 (`mov` at 0x12281b, `mov` at
  0x122825). The prior `jne` bypasses them for a nonzero result from the second
  `_ffscCompareUniqueIDs` call and falls through for zero (`call` at 0x12280f, `jne` at 0x122819).
  The later `_free` call has an unknown first candidate (`call` at 0x12283e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1227df | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122808 | load | 4 | `abs 0x1ede74` | `-` |
| `mov` at 0x12281b | store | 4 | `abs 0x1ede74` | `0x0` |
| `mov` at 0x122825 | store | 4 | `abs 0x1f1244` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1227c3 | `_lstFirst 0x190408` | `0x1d4e68` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x1227d5 | `_lstNext 0x1904b8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1227e7 | `_ffscCompareTimestamps 0x125648` | `?` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x1227fc | `_ffscCompareUniqueIDs 0x125698` | `?` | `0x1f0d00` | `?` | `?` | `?` | `?` |
| `call` at 0x12280f | `_ffscCompareUniqueIDs 0x125698` | `?` | `[0x1ede74]` | `?` | `?` | `?` | `?` |
| `call` at 0x122835 | `_lstDelete 0x190328` | `0x1d4e68` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12283e | `_free 0x17ed48` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1227cf | `(ret(_lstFirst) & ret(_lstFirst)) == 0x0` | `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` |
| `jge` at 0x1227f1 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x122806 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` |
| `jne` at 0x122819 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `jne` at 0x12284a | `(ret(_lstNext) & ret(_lstNext)) != 0x0` | `(ret(_lstNext) & ret(_lstNext)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122857 | `eax` | `0x0` |
