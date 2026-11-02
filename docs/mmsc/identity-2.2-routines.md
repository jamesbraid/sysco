# MMSC 2.2 identity and conflict-handling routines

Read from `6.5.6/usr/cpu/firmware/mmscfw.bin` (IRIX 6.5.6, MMSC 2.2).
Image SHA-256: `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol as an inferred end and can include literals
after returns. Statements below concern sites reached by known control-flow edges.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/identity-2.2-facts.txt) include all tool sections.

## `_identInit`

- `_identInit`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x125570 to 0x125700.
  Established. Calls `_nvramRead` with first candidates 1 and 0x1e1168, and later with 2 and
  0x1f4b7c (`call` at 0x1255ec, `call` at 0x12568c). The four-byte store sites include 0xf at
  0x1e1168, 0x64 at 0x1e116c and 0xff000000 at 0x1e1170 (`mov` at 0x12563f, `mov` at 0x125649, `mov`
  at 0x125653).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x125582 | store | 4 | `abs 0x1f1ce8` | `0x0` |
| `mov` at 0x12558c | store | 4 | `abs 0x1f50e4` | `0x0` |
| `mov` at 0x125596 | store | 1 | `abs 0x1f4b70` | `0x0` |
| `mov` at 0x12559d | store | 1 | `abs 0x1f4b71` | `0x0` |
| `mov` at 0x1255a8 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x1255ae | store | 1 | `(unknown)` | `?` |
| `mov` at 0x1255c8 | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x1255db | store | 4 | `abs 0x1f4b80` | `0x1` |
| `test` at 0x1255f8 | load | 1 | `abs 0x1e0bf1` | `-` |
| `movzx` at 0x12560e | load | 1 | `abs 0x1f4b75` | `-` |
| `movzx` at 0x125618 | load | 1 | `abs 0x1f4b76` | `-` |
| `movzx` at 0x125624 | load | 1 | `abs 0x1f4b77` | `-` |
| `mov` at 0x12562d | store | 4 | `abs 0x1e1168` | `?` |
| `mov` at 0x12563f | store | 4 | `abs 0x1e1168` | `0xf` |
| `mov` at 0x125649 | store | 4 | `abs 0x1e116c` | `0x64` |
| `mov` at 0x125653 | store | 4 | `abs 0x1e1170` | `0xff000000` |
| `mov` at 0x125671 | load | 4 | `abs 0x1e1168` | `-` |
| `mov` at 0x125677 | load | 4 | `abs 0x1e116c` | `-` |
| `mov` at 0x125698 | store | 4 | `abs 0x1f4b7c` | `0xffffffff` |
| `cmp` at 0x1256a2 | load | 4 | `abs 0x1f4b7c` | `-` |
| `mov` at 0x1256ab | store | 4 | `abs 0x1f4b7c` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12557a | `_bzero 0x1764e0` | `0x1f4b70` | `0x1c` | `ebp` | `?` | `?` | `?` |
| `call` at 0x1255ec | `_nvramRead 0x1299c0` | `0x1` | `0x1e1168` | `?` | `?` | `?` | `?` |
| `call` at 0x125606 | `_ffscMsg 0x12154c` | `0x125541` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x125664 | `_nvramWrite 0x129b50` | `0x1` | `0x1e1168` | `?` | `?` | `?` | `?` |
| `call` at 0x12567d | `_inet_makeaddr_b 0x18c838` | `[0x1e116c]` | `[0x1e1168]` | `0x1f4b78` | `?` | `?` | `?` |
| `call` at 0x12568c | `_nvramRead 0x1299c0` | `0x2` | `0x1f4b7c` | `?` | `?` | `?` | `?` |
| `call` at 0x1256ba | `_lstInit 0x193fe0` | `0x1d8c88` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1256c7 | `_identUpdate 0x125a50` | `0x1f4b70` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x1255b8 | `? <= 0x5 (signed, 32-bit)` | `? > 0x5 (signed, 32-bit)` |
| `jle` at 0x1255d9 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `je` at 0x1255f6 | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` |
| `je` at 0x1255ff | `(([0x1e0bf1]:1 & 0xff) & 0x1) == 0x0` | `(([0x1e0bf1]:1 & 0xff) & 0x1) != 0x0` |
| `jbe` at 0x12563d | `? <= 0xffffee (unsigned, 32-bit)` | `? > 0xffffee (unsigned, 32-bit)` |
| `je` at 0x125696 | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` |
| `jbe` at 0x1256a9 | `[0x1f4b7c] <= 0x16 (unsigned, 32-bit)` | `[0x1f4b7c] > 0x16 (unsigned, 32-bit)` |
| `jl` at 0x1256ce | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1256d5 | `eax` | `0x0` |
| `ret` at 0x1256e8 | `eax` | `0xffffffff` |

## `_identSetRackID`

- `_identSetRackID`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x125f40 to 0x125fb0.
  Established. A four-byte store copies `[sp+4]` to 0x1f4b7c before `_nvramWrite` receives
  candidates beginning 2 and 0x1f4b7c (`mov` at 0x125f4b, `call` at 0x125f58). The zero result of
  that call takes `je` past the first `_ffscMsg` call (`je` at 0x125f62, `call` at 0x125f69). A
  later four-byte store writes 1 to 0x1f1410 (`mov` at 0x125f71).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x125f48 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x125f4b | store | 4 | `abs 0x1f4b7c` | `[sp+4]` |
| `mov` at 0x125f71 | store | 4 | `abs 0x1f1410` | `0x1` |
| `test` at 0x125f7b | load | 1 | `abs 0x1e0bf1` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x125f58 | `_nvramWrite 0x129b50` | `0x2` | `0x1f4b7c` | `?` | `?` | `?` | `?` |
| `call` at 0x125f69 | `_ffscMsg 0x12154c` | `0x125eed` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x125f89 | `_ffscRackIDToString 0x1284e0` | `[sp+4]` | `sp-16` | `?` | `?` | `?` | `?` |
| `call` at 0x125f97 | `_ffscMsg 0x12154c` | `0x125f1f` | `sp-16` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x125f62 | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` |
| `je` at 0x125f82 | `(([0x1e0bf1]:1 & 0xff) & 0x1) == 0x0` | `(([0x1e0bf1]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x125fa6 | `eax` | `0x0` |

## `_identSetCapabilities`

- `_identSetCapabilities`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x125c90 to 0x125d70.
  Established. Stores four bytes from `[sp+4]` at 0x1f4b80 (`mov` at 0x125c96). The byte test at
  0x1e0bf1 precedes `je`, which bypasses `_ffscMsg` for `(([0x1e0bf1]:1 & 0xff) & 0x1) == 0x0`
  (`test` at 0x125c9b, `je` at 0x125ca2, `call` at 0x125caa).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x125c93 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x125c96 | store | 4 | `abs 0x1f4b80` | `[sp+4]` |
| `test` at 0x125c9b | load | 1 | `abs 0x1e0bf1` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x125caa | `_ffscMsg 0x12154c` | `0x125c65` | `[sp+4]` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x125ca2 | `(([0x1e0bf1]:1 & 0xff) & 0x1) == 0x0` | `(([0x1e0bf1]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x125cb4 | `eax` | `0x0` |

## `_identSetModuleNums`

- `_identSetModuleNums`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x125e60 to 0x125f40.
  Established. The indexed four-byte load and store sites have unresolved effective targets and
  values (`mov` at 0x125e78, `mov` at 0x125e7b). The later `_ffscModuleNumToString` call has an
  unknown first candidate and `sp-16` as its second (`call` at 0x125eb5). Both backward `jbe`
  predicates retain unknown operands (`jbe` at 0x125e88, `jbe` at 0x125ed5).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x125e69 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x125e78 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x125e7b | store | 4 | `(unknown)` | `?` |
| `test` at 0x125e8a | load | 1 | `abs 0x1e0bf1` | `-` |
| `mov` at 0x125ead | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x125e98 | `_ffscMsgN 0x1215ec` | `0x125e36` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x125eb5 | `_ffscModuleNumToString 0x128160` | `?` | `sp-16` | `?` | `?` | `?` | `?` |
| `call` at 0x125ec3 | `_ffscMsgN 0x1215ec` | `0x125e51` | `sp-16` | `?` | `?` | `?` | `?` |
| `call` at 0x125edc | `_ffscMsg 0x12154c` | `0x125b22` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x125e88 | `be flags condition (operands unknown)` | `not (be flags condition (operands unknown))` |
| `je` at 0x125e91 | `(([0x1e0bf1]:1 & 0xff) & 0x1) == 0x0` | `(([0x1e0bf1]:1 & 0xff) & 0x1) != 0x0` |
| `jbe` at 0x125ed5 | `be flags condition (operands unknown)` | `not (be flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x125eec | `eax` | `0x0` |

## `_identUpdate`

- `_identUpdate`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x125a50 to 0x125b80.
  Established. The two `_memcpy` calls have first candidates
  `ret(_identFindIdentityNodeByUniqueID)+16` and `ret(_malloc)+16`, each followed by `[sp+4]` and
  0x1c (`call` at 0x125a87, `call` at 0x125af7). The `_malloc` call has first candidate 0x2c (`call`
  at 0x125a92).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x125a55 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x125a59 | `_identFindIdentityNodeByUniqueID 0x125fb0` | `[sp+4]` | `ebx` | `esi` | `ebp` | `?` | `?` |
| `call` at 0x125a6d | `_clock_gettime 0x17fa34` | `0x0` | `ret(_identFindIdentityNodeByUniqueID)+8` | `?` | `?` | `?` | `?` |
| `call` at 0x125a87 | `_memcpy 0x183fb8` | `ret(_identFindIdentityNodeByUniqueID)+16` | `[sp+4]` | `0x1c` | `?` | `?` | `?` |
| `call` at 0x125a92 | `_malloc 0x182a90` | `0x2c` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x125aa5 | `_ffscMsgS 0x12167c` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x125ac6 | `_clock_gettime 0x17fa34` | `0x0` | `ret(_malloc)+8` | `?` | `?` | `?` | `?` |
| `call` at 0x125ad7 | `_ffscMsgS 0x12167c` | `0x125a10` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x125ae0 | `_free 0x182ab0` | `ret(_malloc)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x125af7 | `_memcpy 0x183fb8` | `ret(_malloc)+16` | `[sp+4]` | `0x1c` | `?` | `?` | `?` |
| `call` at 0x125b05 | `_lstAdd 0x194000` | `0x1d8c88` | `ret(_malloc)` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x125a65 | `(ret(_identFindIdentityNodeByUniqueID) & ret(_identFindIdentityNodeByUniqueID)) == 0x0` | `(ret(_identFindIdentityNodeByUniqueID) & ret(_identFindIdentityNodeByUniqueID)) != 0x0` |
| `je` at 0x125a77 | `(ret(_clock_gettime) & ret(_clock_gettime)) == 0x0` | `(ret(_clock_gettime) & ret(_clock_gettime)) != 0x0` |
| `jne` at 0x125a9e | `(ret(_malloc) & ret(_malloc)) != 0x0` | `(ret(_malloc) & ret(_malloc)) == 0x0` |
| `je` at 0x125ad0 | `(ret(_clock_gettime) & ret(_clock_gettime)) == 0x0` | `(ret(_clock_gettime) & ret(_clock_gettime)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x125b17 | `eax` | `?` |

## `_identScanForConflicts`

- `_identScanForConflicts`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1257c0 to 0x125a50.
  Established. Four indirect call sites have unknown targets, with the second candidate rendered as
  `ret(_lstNext)+16` at two of them (`call` at 0x125811, `call` at 0x125866, `call` at 0x1258f0,
  `call` at 0x125935). The first two `_inet_lnaof` calls have first candidates unknown and
  `[ret(_lstNext)+24]` respectively (`call` at 0x1257fc, `call` at 0x125844).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `test` at 0x1257c6 | load | 1 | `abs 0x1e0bf0` | `-` |
| `mov` at 0x1257f8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12580e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x125840 | load | 4 | `ret(_lstNext) +0x18` | `-` |
| `mov` at 0x125851 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x125854 | load | 4 | `ret(_lstNext) +0x18` | `-` |
| `mov` at 0x125863 | load | 4 | `sp +0x4` | `-` |
| `test` at 0x1258ac | load | 1 | `abs 0x1e0bf0` | `-` |
| `mov` at 0x1258e0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1258ed | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x125920 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x125923 | load | 4 | `ret(_lstNext) +0x1c` | `-` |
| `mov` at 0x125932 | load | 4 | `sp +0x8` | `-` |
| `test` at 0x12598b | load | 1 | `abs 0x1e0bf0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1257d4 | `_ffscMsg 0x12154c` | `0x125751` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x1257e3 | `_lstFirst 0x194170` | `0x1d8c88` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1257fc | `_inet_lnaof 0x18c7c8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x125811 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x125844 | `_inet_lnaof 0x18c7c8` | `[ret(_lstNext)+24]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x125866 | `(unknown callee) (unknown target)` | `?` | `ret(_lstNext)+16` | `?` | `?` | `?` | `?` |
| `call` at 0x12587b | `_lstNext 0x194220` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12588e | `_lstNext 0x194220` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1258ba | `_ffscMsg 0x12154c` | `0x12577b` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1258cb | `_lstFirst 0x194170` | `0x1d8c88` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1258f0 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x125935 | `(unknown callee) (unknown target)` | `?` | `ret(_lstNext)+16` | `?` | `?` | `?` | `?` |
| `call` at 0x12595a | `_lstNext 0x194220` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12596d | `_lstNext 0x194220` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x125999 | `_ffscMsg 0x12154c` | `0x1257a1` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1257cd | `(([0x1e0bf0]:1 & 0xff) & 0x1) == 0x0` | `(([0x1e0bf0]:1 & 0xff) & 0x1) != 0x0` |
| `je` at 0x1257ef | `(ret(_lstFirst) & ret(_lstFirst)) == 0x0` | `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` |
| `jg` at 0x125807 | `ret(_inet_lnaof) > 0xf (signed, 32-bit)` | `ret(_inet_lnaof) <= 0xf (signed, 32-bit)` |
| `jl` at 0x125818 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jne` at 0x12581e | `(ret(unknown) & ret(unknown)) != 0x0` | `(ret(unknown) & ret(unknown)) == 0x0` |
| `jle` at 0x12584f | `ret(_inet_lnaof) <= 0xf (signed, 32-bit)` | `ret(_inet_lnaof) > 0xf (signed, 32-bit)` |
| `jne` at 0x125859 | `Z == 0` | `not (Z == 0)` |
| `jl` at 0x12586d | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jne` at 0x125873 | `(ret(unknown) & ret(unknown)) != 0x0` | `(ret(unknown) & ret(unknown)) == 0x0` |
| `je` at 0x125887 | `(ret(_lstNext) & ret(_lstNext)) == 0x0` | `(ret(_lstNext) & ret(_lstNext)) != 0x0` |
| `je` at 0x12588b | `Z == 1` | `not (Z == 1)` |
| `je` at 0x12589a | `(ret(_lstNext) & ret(_lstNext)) == 0x0` | `(ret(_lstNext) & ret(_lstNext)) != 0x0` |
| `je` at 0x12589e | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x1258a6 | `Z == 0` | `not (Z == 0)` |
| `je` at 0x1258b3 | `(([0x1e0bf0]:1 & 0xff) & 0x1) == 0x0` | `(([0x1e0bf0]:1 & 0xff) & 0x1) != 0x0` |
| `je` at 0x1258d7 | `(ret(_lstFirst) & ret(_lstFirst)) == 0x0` | `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` |
| `jne` at 0x1258e6 | `Z == 0` | `not (Z == 0)` |
| `jl` at 0x1258f7 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jne` at 0x1258f9 | `(ret(unknown) & ret(unknown)) != 0x0` | `(ret(unknown) & ret(unknown)) == 0x0` |
| `jne` at 0x125928 | `Z == 0` | `not (Z == 0)` |
| `jge` at 0x12593c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x125952 | `(ret(unknown) & ret(unknown)) != 0x0` | `(ret(unknown) & ret(unknown)) == 0x0` |
| `je` at 0x125966 | `(ret(_lstNext) & ret(_lstNext)) == 0x0` | `(ret(_lstNext) & ret(_lstNext)) != 0x0` |
| `je` at 0x12596a | `Z == 1` | `not (Z == 1)` |
| `je` at 0x125979 | `(ret(_lstNext) & ret(_lstNext)) == 0x0` | `(ret(_lstNext) & ret(_lstNext)) != 0x0` |
| `je` at 0x12597d | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x125985 | `Z == 0` | `not (Z == 0)` |
| `je` at 0x125992 | `(([0x1e0bf0]:1 & 0xff) & 0x1) == 0x0` | `(([0x1e0bf0]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1259a9 | `eax` | `?` |

## `_identFindMaster`

- `_identFindMaster`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1254e0 to 0x125570.
  Established. Calls `_lstFirst` with first candidate 0x1d8c88 (`call` at 0x1254ea). The later
  `_ffscCompareUniqueIDs` candidates begin `ret(_lstNext)+16` and unknown (`call` at 0x125518). The
  deciding `jge` keeps its condition operands unresolved (`jge` at 0x125522).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1254ea | `_lstFirst 0x194170` | `0x1d8c88` | `ebx` | `esi` | `ebp` | `?` | `?` |
| `call` at 0x125518 | `_ffscCompareUniqueIDs 0x127f50` | `ret(_lstNext)+16` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x125527 | `_lstNext 0x194220` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x1254f6 | `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` | `(ret(_lstFirst) & ret(_lstFirst)) == 0x0` |
| `jge` at 0x125522 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x125533 | `(ret(_lstNext) & ret(_lstNext)) != 0x0` | `(ret(_lstNext) & ret(_lstNext)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x125540 | `eax` | `?` |

## `_identFindConflictingRackID`

- `_identFindConflictingRackID`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x125460 to 0x1254e0.
  Established. A four-byte read is rendered as `[sp+4] +0xc` (`mov` at 0x12546d). `jne` enters the
  `_lstFirst` route when `[[sp+4]+12] != 0xffffffff`, while equality falls through to the exit
  (`jne` at 0x125473, `call` at 0x125495). The later read through `[sp+4] +0xc` is a separate access
  (`mov` at 0x1254b4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x125466 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12546d | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x1254b1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1254b4 | load | 4 | `[sp+4] +0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x125495 | `_lstFirst 0x194170` | `0x1d8c88` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x1254a5 | `_ffscCompareUniqueIDs 0x127f50` | `?` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x1254bc | `_lstNext 0x194220` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12546b | `([sp+4] & [sp+4]) == 0x0` | `([sp+4] & [sp+4]) != 0x0` |
| `jne` at 0x125473 | `[[sp+4]+12] != 0xffffffff` | `[[sp+4]+12] == 0xffffffff` |
| `je` at 0x1254af | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` |
| `je` at 0x1254b9 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x1254c8 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1254d5 | `eax` | `?` |

## `_identFindConflictingIPAddr`

- `_identFindConflictingIPAddr`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1253f0 to 0x125460.
  Established. The initial `jne` enters the `_lstFirst` route for `([sp+4] & [sp+4]) != 0x0` (`jne`
  at 0x1253fb, `call` at 0x125415). A later four-byte read renders as `[sp+4] +0x8`, while the
  adjacent four-byte read has an unknown effective target (`mov` at 0x125434, `mov` at 0x125431).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1253f6 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x125431 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x125434 | load | 4 | `[sp+4] +0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x125415 | `_lstFirst 0x194170` | `0x1d8c88` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x125425 | `_ffscCompareUniqueIDs 0x127f50` | `?` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x12543c | `_lstNext 0x194220` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x1253fb | `([sp+4] & [sp+4]) != 0x0` | `([sp+4] & [sp+4]) == 0x0` |
| `je` at 0x12542f | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` |
| `je` at 0x125439 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x125448 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x125455 | `eax` | `?` |

## `_identEnumerateRacks`

- `_identEnumerateRacks`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x125210 to 0x1252c0.
  Established. Three indexed four-byte store sites retain unknown targets and values (`mov` at
  0x125242, `mov` at 0x125293, `mov` at 0x125296). `jae` skips the latter two for `C == 0` and falls
  through for `not (C == 0)` (`jae` at 0x12528b, `mov` at 0x125293, `mov` at 0x125296). The backward
  `jl` is taken for `[sp-16] < [sp-12] (signed, 32-bit)` (`jl` at 0x1252a4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x125219 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x125234 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12523c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12523f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x125242 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x12525e | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x12526b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x125271 | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x125274 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12527a | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x125280 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x125283 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x125286 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12528d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x125290 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x125293 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x125296 | store | 4 | `(unknown)` | `?` |
| `add` at 0x125299 | load | 4 | `sp -0x10` | `-` |
| `add` at 0x125299 | store | 4 | `sp -0x10` | `[sp-16]+4` |
| `mov` at 0x12529e | load | 4 | `sp -0xc` | `-` |
| `cmp` at 0x1252a1 | load | 4 | `sp -0x10` | `-` |
| `cmp` at 0x1252a7 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x125223 | `_lstFirst 0x194170` | `0x1d8c88` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x125247 | `_lstNext 0x194220` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x125232 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x12523a | `Z == 1` | `not (Z == 1)` |
| `jle` at 0x125253 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jge` at 0x12525c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jge` at 0x125269 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jae` at 0x12528b | `C == 0` | `not (C == 0)` |
| `jl` at 0x1252a4 | `[sp-16] < [sp-12] (signed, 32-bit)` | `[sp-16] >= [sp-12] (signed, 32-bit)` |
| `jg` at 0x1252aa | `[sp-8] > ? (signed, 32-bit)` | `[sp-8] <= ? (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1252b7 | `eax` | `?` |

## `_identEnumerateModules`

- `_identEnumerateModules`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x125150 to 0x125210.
  Established. The indexed store sites all write four bytes to unresolved targets with unknown
  values (`mov` at 0x12518c, `mov` at 0x1251e3, `mov` at 0x1251e6). The `add` site reads and writes
  four bytes at `sp -0x10`, storing `[sp-16]+4` (`add` at 0x1251e9). The final EAX result is unknown
  (`ret` at 0x125207).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x125159 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12517c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x125185 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x125189 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12518c | store | 4 | `(unknown)` | `?` |
| `mov` at 0x1251ae | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x1251bb | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1251c1 | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x1251c4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1251ca | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x1251d0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1251d3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1251d6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1251dd | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1251e0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1251e3 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x1251e6 | store | 4 | `(unknown)` | `?` |
| `add` at 0x1251e9 | load | 4 | `sp -0x10` | `-` |
| `add` at 0x1251e9 | store | 4 | `sp -0x10` | `[sp-16]+4` |
| `mov` at 0x1251ee | load | 4 | `sp -0xc` | `-` |
| `cmp` at 0x1251f1 | load | 4 | `sp -0x10` | `-` |
| `cmp` at 0x1251f7 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x125163 | `_lstFirst 0x194170` | `0x1d8c88` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x125197 | `_lstNext 0x194220` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x125172 | `Z == 1` | `not (Z == 1)` |
| `jg` at 0x12517a | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `je` at 0x125183 | `Z == 1` | `not (Z == 1)` |
| `jle` at 0x125194 | `? <= 0x1 (signed, 32-bit)` | `? > 0x1 (signed, 32-bit)` |
| `jle` at 0x1251a3 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jge` at 0x1251ac | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jge` at 0x1251b9 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jae` at 0x1251db | `C == 0` | `not (C == 0)` |
| `jl` at 0x1251f4 | `[sp-16] < [sp-12] (signed, 32-bit)` | `[sp-16] >= [sp-12] (signed, 32-bit)` |
| `jg` at 0x1251fa | `[sp-8] > ? (signed, 32-bit)` | `[sp-8] <= ? (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x125207 | `eax` | `?` |

## `_identFindByRackID`

- `_identFindByRackID`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x125380 to 0x1253d0.
  Established. `jne` selects the `_lstFirst` route for `[sp+4] != 0xffffffff`, and equality falls
  through to the exit (`jne` at 0x12538a, `call` at 0x1253a5). The later four-byte read has an
  unresolved target (`mov` at 0x1253b0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x125384 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1253b0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1253c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1253a5 | `_lstFirst 0x194170` | `0x1d8c88` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x1253b8 | `_lstNext 0x194220` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x12538a | `[sp+4] != 0xffffffff` | `[sp+4] == 0xffffffff` |
| `je` at 0x1253b5 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x1253c4 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1253ce | `eax` | `?` |

## `_identFindByIPAddr`

- `_identFindByIPAddr`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1252c0 to 0x125310.
  Established. The four-byte read through `[sp+4] +0x0` follows another four-byte read whose
  effective target is unknown (`mov` at 0x1252e3, `mov` at 0x1252e0). The following `jne` is taken
  for `Z == 0`, with fall-through for `not (Z == 0)` (`jne` at 0x1252e7).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1252c4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1252e0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1252e3 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x125301 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1252cc | `_lstFirst 0x194170` | `0x1d8c88` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x1252f1 | `_lstNext 0x194220` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x1252e7 | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x1252fd | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x125307 | `eax` | `?` |

## `_identFindByUniqueID`

- `_identFindByUniqueID`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1253d0 to 0x1253f0.
  Established. The `_identFindIdentityNodeByUniqueID` call has first candidate `[sp+4]` (`call` at
  0x1253d7); its zero result takes `je` past the address calculation, while nonzero falls through
  (`je` at 0x1253e0). The recorded EAX exit remains unknown (`ret` at 0x1253ea).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1253d3 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1253d7 | `_identFindIdentityNodeByUniqueID 0x125fb0` | `[sp+4]` | `ebp` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1253e0 | `(ret(_identFindIdentityNodeByUniqueID) & ret(_identFindIdentityNodeByUniqueID)) == 0x0` | `(ret(_identFindIdentityNodeByUniqueID) & ret(_identFindIdentityNodeByUniqueID)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1253ea | `eax` | `?` |

## `_identCleanOldEntries`

- `_identCleanOldEntries`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x125060 to 0x125100.
  Established. Two four-byte store sites clear 0x1f1ce8 and 0x1f50e4 (`mov` at 0x1250c3, `mov` at
  0x1250cd). The prior `jne` bypasses those stores when the second `_ffscCompareUniqueIDs`
  result is nonzero, and falls through when it is zero (`call` at 0x1250b7, `jne` at 0x1250c1). The
  later `_lstDelete` candidates begin 0x1d8c88 and unknown (`call` at 0x1250dd).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x125087 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1250b0 | load | 4 | `abs 0x1f1ce8` | `-` |
| `mov` at 0x1250c3 | store | 4 | `abs 0x1f1ce8` | `0x0` |
| `mov` at 0x1250cd | store | 4 | `abs 0x1f50e4` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12506b | `_lstFirst 0x194170` | `0x1d8c88` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x12507d | `_lstNext 0x194220` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12508f | `_ffscCompareTimestamps 0x127f00` | `?` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x1250a4 | `_ffscCompareUniqueIDs 0x127f50` | `?` | `0x1f4b70` | `?` | `?` | `?` | `?` |
| `call` at 0x1250b7 | `_ffscCompareUniqueIDs 0x127f50` | `?` | `[0x1f1ce8]` | `?` | `?` | `?` | `?` |
| `call` at 0x1250dd | `_lstDelete 0x194090` | `0x1d8c88` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1250e6 | `_free 0x182ab0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x125077 | `(ret(_lstFirst) & ret(_lstFirst)) == 0x0` | `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` |
| `jge` at 0x125099 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x1250ae | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` |
| `jne` at 0x1250c1 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `jne` at 0x1250f2 | `(ret(_lstNext) & ret(_lstNext)) != 0x0` | `(ret(_lstNext) & ret(_lstNext)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1250ff | `eax` | `0x0` |
