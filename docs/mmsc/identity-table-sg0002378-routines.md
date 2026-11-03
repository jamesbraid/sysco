# MMSC identity table in patch SG0002378

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

[Fact sheets and full listings](disasm/identity-table-sg0002378-facts.txt) include all tool sections.

## `_identUpdate`

- `_identUpdate`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x123078 to 0x1231a8.
  Established. The `_identFindIdentityNodeByUniqueID` call has candidate zero `[sp+4]` (`call` at
  0x123081). The two `_memcpy` sites have different destination candidates,
  `ret(_identFindIdentityNodeByUniqueID)+16` (`call` at 0x1230af) and `ret(_malloc)+16` (`call` at
  0x12311f), with candidate two `0x1c` at each site. The allocation path includes `_malloc` with
  candidate zero `0x2c` (`call` at 0x1230ba).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12307d | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x123081 | `_identFindIdentityNodeByUniqueID 0x1235d8` | `[sp+4]` | `ebx` | `esi` | `ebp` | `?` | `?` |
| `call` at 0x123095 | `_clock_gettime 0x17bbcc` | `0x0` | `ret(_identFindIdentityNodeByUniqueID)+8` | `?` | `?` | `?` | `?` |
| `call` at 0x1230af | `_memcpy 0x180150` | `ret(_identFindIdentityNodeByUniqueID)+16` | `[sp+4]` | `0x1c` | `?` | `?` | `?` |
| `call` at 0x1230ba | `_malloc 0x17ec28` | `0x2c` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1230cd | `_ffscMsgS 0x11f110` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1230ee | `_clock_gettime 0x17bbcc` | `0x0` | `ret(_malloc)+8` | `?` | `?` | `?` | `?` |
| `call` at 0x1230ff | `_ffscMsgS 0x11f110` | `0x123038` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x123108 | `_free 0x17ec48` | `ret(_malloc)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12311f | `_memcpy 0x180150` | `ret(_malloc)+16` | `[sp+4]` | `0x1c` | `?` | `?` | `?` |
| `call` at 0x12312d | `_lstAdd 0x190198` | `0x1d4d68` | `ret(_malloc)` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12308d | `(ret(_identFindIdentityNodeByUniqueID) & ret(_identFindIdentityNodeByUniqueID)) == 0x0` | `(ret(_identFindIdentityNodeByUniqueID) & ret(_identFindIdentityNodeByUniqueID)) != 0x0` |
| `je` at 0x12309f | `(ret(_clock_gettime) & ret(_clock_gettime)) == 0x0` | `(ret(_clock_gettime) & ret(_clock_gettime)) != 0x0` |
| `jne` at 0x1230c6 | `(ret(_malloc) & ret(_malloc)) != 0x0` | `(ret(_malloc) & ret(_malloc)) == 0x0` |
| `je` at 0x1230f8 | `(ret(_clock_gettime) & ret(_clock_gettime)) == 0x0` | `(ret(_clock_gettime) & ret(_clock_gettime)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12313f | `eax` | `?` |

## `_identSetIPAddr`

- `_identSetIPAddr`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x123398 to 0x123488.
  Established. On the taken zero-result path from `_ifAddrSet` (`je` at 0x1233f3), four-byte stores
  write `[[sp+4]]` to absolute `0x1f0c00` (`mov` at 0x12340a), `ret(_inet_netof)` to absolute
  `0x1dd204` (`mov` at 0x12340f), and `[sp-28]` to absolute `0x1dd200` (`mov` at 0x123418). The
  `_nvramWrite` call then presents candidates `0x1` and `0x1dd200` (`call` at 0x123425).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1233a1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1233a4 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x1233af | store | 4 | `sp -0x1c` | `ret(_inet_lnaof)` |
| `mov` at 0x1233b2 | load | 4 | `[sp+4] +0x0` | `-` |
| `cmp` at 0x1233bf | load | 4 | `abs 0x1dd204` | `-` |
| `mov` at 0x1233d8 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x123408 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x12340a | store | 4 | `abs 0x1f0c00` | `[[sp+4]]` |
| `mov` at 0x12340f | store | 4 | `abs 0x1dd204` | `ret(_inet_netof)` |
| `mov` at 0x123415 | load | 4 | `sp -0x1c` | `-` |
| `mov` at 0x123418 | store | 4 | `abs 0x1dd200` | `[sp-28]` |
| `test` at 0x12343e | load | 1 | `abs 0x1dccc1` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1233a7 | `_inet_lnaof 0x188960` | `[[sp+4]]` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x1233b5 | `_inet_netof 0x188aa0` | `[[sp+4]]` | `?` | `?` | `?` | `ret(_inet_lnaof)` | `?` |
| `call` at 0x1233cc | `_ffscMsg 0x11f040` | `0x1232dd` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1233db | `_inet_ntoa_b 0x188c90` | `[[sp+4]]` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x1233e9 | `_ifAddrSet 0x1898e0` | `0x123303` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x1233fb | `_ffscMsgS 0x11f110` | `0x123308` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x123425 | `_nvramWrite 0x127298` | `0x1` | `0x1dd200` | `?` | `?` | `?` | `?` |
| `call` at 0x123436 | `_ffscMsg 0x11f040` | `0x123332` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12344d | `_ffscMsg 0x11f040` | `0x123370` | `sp-24` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1233c5 | `[0x1dd204] == ret(_inet_netof)` | `[0x1dd204] != ret(_inet_netof)` |
| `je` at 0x1233f3 | `(ret(_ifAddrSet) & ret(_ifAddrSet)) == 0x0` | `(ret(_ifAddrSet) & ret(_ifAddrSet)) != 0x0` |
| `je` at 0x12342f | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` |
| `je` at 0x123445 | `(([0x1dccc1]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dccc1]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12345d | `eax` | `?` |

## `_identSetCapabilities`

- `_identSetCapabilities`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1232b8 to 0x123398.
  Established. A four-byte store places `[sp+4]` at absolute `0x1f0c08` (`mov` at 0x1232be). The
  branch skips the `_ffscMsg` call when `(([0x1dccc1]:1 & 0xff) & 0x1) == 0x0` (`je` at 0x1232ca).
  That call has leading candidates `0x12328d` and `[sp+4]` (`call` at 0x1232d2). The stored field
  meaning remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1232bb | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1232be | store | 4 | `abs 0x1f0c08` | `[sp+4]` |
| `test` at 0x1232c3 | load | 1 | `abs 0x1dccc1` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1232d2 | `_ffscMsg 0x11f040` | `0x12328d` | `[sp+4]` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1232ca | `(([0x1dccc1]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dccc1]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1232dc | `eax` | `0x0` |

## `_identSetModuleNums`

- `_identSetModuleNums`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x123488 to 0x123568.
  Established. The indexed four-byte store has an unknown effective address and value (`mov` at
  0x1234a3). The `_ffscModuleNumToString` site has unknown candidate zero and candidate one `sp-16`
  (`call` at 0x1234dd). The subsequent `_ffscMsgN` site presents `0x123479` and `sp-16` (`call` at
  0x1234eb). No fixed destination range is established for the indexed store.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x123491 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1234a0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1234a3 | store | 4 | `(unknown)` | `?` |
| `test` at 0x1234b2 | load | 1 | `abs 0x1dccc1` | `-` |
| `mov` at 0x1234d5 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1234c0 | `_ffscMsgN 0x11f0a0` | `0x12345e` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1234dd | `_ffscModuleNumToString 0x125788` | `?` | `sp-16` | `?` | `?` | `?` | `?` |
| `call` at 0x1234eb | `_ffscMsgN 0x11f0a0` | `0x123479` | `sp-16` | `?` | `?` | `?` | `?` |
| `call` at 0x123504 | `_ffscMsg 0x11f040` | `0x12314a` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x1234b0 | `be flags condition (operands unknown)` | `not (be flags condition (operands unknown))` |
| `je` at 0x1234b9 | `(([0x1dccc1]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dccc1]:1 & 0xff) & 0x1) != 0x0` |
| `jbe` at 0x1234fd | `be flags condition (operands unknown)` | `not (be flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x123514 | `eax` | `0x0` |

## `_identFindByUniqueID`

- `_identFindByUniqueID`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122a08 to 0x122a28.
  Established. The `_identFindIdentityNodeByUniqueID` call forwards candidate zero `[sp+4]` (`call`
  at 0x122a0f); its zero-result branch is taken when `(ret(_identFindIdentityNodeByUniqueID) &
  ret(_identFindIdentityNodeByUniqueID)) == 0x0` (`je` at 0x122a18). The merged return value remains
  unknown (`ret` at 0x122a22).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x122a0b | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122a0f | `_identFindIdentityNodeByUniqueID 0x1235d8` | `[sp+4]` | `ebp` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x122a18 | `(ret(_identFindIdentityNodeByUniqueID) & ret(_identFindIdentityNodeByUniqueID)) == 0x0` | `(ret(_identFindIdentityNodeByUniqueID) & ret(_identFindIdentityNodeByUniqueID)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122a22 | `eax` | `?` |

## `_identPrint`

- `_identPrint`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1231a8 to 0x1232b8.
  Established. The `_inet_ntoa_b` call uses candidates `[[sp+4]+8]` and `sp-60` (`call` at
  0x1231f9). The separate `_ffscRackIDToString` call uses `[[sp+4]+12]` and `sp-72` (`call` at
  0x12320c). The local byte at `sp -0x1c` is cleared (`mov` at 0x123214), and the `_strcat` sites
  have destination candidate `sp-28` (`call` at 0x123237, `call` at 0x123241).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1231b1 | store | 4 | `sp -0x64` | `0x0` |
| `cmp` at 0x1231b8 | load | 4 | `abs 0x1edd6c` | `-` |
| `mov` at 0x1231c1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1231c5 | load | 4 | `abs 0x1edd6c` | `-` |
| `mov` at 0x1231d7 | store | 4 | `sp -0x64` | `0x1` |
| `mov` at 0x1231e2 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1231f2 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1231f5 | load | 4 | `[sp+4] +0x8` | `-` |
| `mov` at 0x123205 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x123208 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x123214 | store | 1 | `sp -0x1c` | `0x0` |
| `mov` at 0x123221 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x123224 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12324f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x123252 | load | 4 | `[sp+4] +0x10` | `-` |
| `cmp` at 0x123267 | load | 4 | `sp -0x64` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1231cb | `_ffscCompareUniqueIDs 0x125578` | `[0x1edd6c]` | `[sp+4]` | `ebx` | `esi` | `edi` | `0x0` |
| `call` at 0x1231e6 | `_ffscUniqueIDToString 0x125da8` | `[sp+4]` | `sp-96` | `?` | `?` | `?` | `?` |
| `call` at 0x1231f9 | `_inet_ntoa_b 0x188c90` | `[[sp+4]+8]` | `sp-60` | `?` | `?` | `?` | `?` |
| `call` at 0x12320c | `_ffscRackIDToString 0x125b08` | `[[sp+4]+12]` | `sp-72` | `?` | `?` | `?` | `?` |
| `call` at 0x123229 | `_ffscModuleNumToString 0x125788` | `?` | `sp-40` | `?` | `?` | `?` | `?` |
| `call` at 0x123237 | `_strcat 0x1801d8` | `sp-28` | `0x123140` | `?` | `?` | `?` | `?` |
| `call` at 0x123241 | `_strcat 0x1801d8` | `sp-28` | `sp-40` | `?` | `?` | `?` | `?` |
| `call` at 0x12327c | `_ffscMsg 0x11f040` | `0x12314b` | `sp-96` | `?` | `sp-72` | `sp-60` | `sp-28` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1231bf | `[0x1edd6c] == 0x0` | `[0x1edd6c] != 0x0` |
| `jne` at 0x1231d5 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `jle` at 0x12324d | `? <= 0x1 (signed, 32-bit)` | `? > 0x1 (signed, 32-bit)` |
| `je` at 0x12326b | `[sp-100] == 0x0` | `[sp-100] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12328c | `eax` | `0x0` |

## `_identFindByIPAddr`

- `_identFindByIPAddr`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1228f8 to 0x122948.
  Established. The `_lstFirst` site has candidate zero `0x1d4d68` (`call` at 0x122904). The
  four-byte load through `[sp+4] +0x0` has an unknown value (`mov` at 0x12291b). The comparison
  branch is taken when `Z == 0` (`jne` at 0x12291f). The tool does not resolve its compared values.
  The `_lstNext` candidate values are unknown (`call` at 0x122929).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1228fc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122918 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12291b | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x122939 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122904 | `_lstFirst 0x190308` | `0x1d4d68` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x122929 | `_lstNext 0x1903b8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x12291f | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x122935 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12293f | `eax` | `?` |

## `_identCleanOldEntries`

- `_identCleanOldEntries`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122698 to 0x122738.
  Established. The call to `_ffscCompareTimestamps` has unknown candidate zero and candidate one
  `[sp+4]` (`call` at 0x1226c7). After the later `_ffscCompareUniqueIDs` call (`call` at 0x1226ef),
  the result branch falls through when `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) ==
  0x0` (`jne` at 0x1226f9). That path clears absolute `0x1edd6c` (`mov` at 0x1226fb) and absolute
  `0x1f113c` (`mov` at 0x122705) with four-byte stores before joining the `_lstDelete` site (`call`
  at 0x122715).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1226bf | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1226e8 | load | 4 | `abs 0x1edd6c` | `-` |
| `mov` at 0x1226fb | store | 4 | `abs 0x1edd6c` | `0x0` |
| `mov` at 0x122705 | store | 4 | `abs 0x1f113c` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1226a3 | `_lstFirst 0x190308` | `0x1d4d68` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x1226b5 | `_lstNext 0x1903b8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1226c7 | `_ffscCompareTimestamps 0x125528` | `?` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x1226dc | `_ffscCompareUniqueIDs 0x125578` | `?` | `0x1f0bf8` | `?` | `?` | `?` | `?` |
| `call` at 0x1226ef | `_ffscCompareUniqueIDs 0x125578` | `?` | `[0x1edd6c]` | `?` | `?` | `?` | `?` |
| `call` at 0x122715 | `_lstDelete 0x190228` | `0x1d4d68` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12271e | `_free 0x17ec48` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1226af | `(ret(_lstFirst) & ret(_lstFirst)) == 0x0` | `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` |
| `jge` at 0x1226d1 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x1226e6 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` |
| `jne` at 0x1226f9 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `jne` at 0x12272a | `(ret(_lstNext) & ret(_lstNext)) != 0x0` | `(ret(_lstNext) & ret(_lstNext)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122737 | `eax` | `0x0` |

## `_identScanForConflicts`

- `_identScanForConflicts`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122de8 to 0x123078.
  Established. Four indirect call sites have unknown callees (`call` at 0x122e39, `call` at
  0x122e8e, `call` at 0x122f18, `call` at 0x122f5d). The second and fourth have candidate one
  `ret(_lstNext)+16` (`call` at 0x122e8e, `call` at 0x122f5d). The other candidate values remain
  unknown. The first `_inet_lnaof` result branch is taken when `ret(_inet_lnaof) > 0xf (signed,
  32-bit)` (`jg` at 0x122e2f). The conflict-resolution policy is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `test` at 0x122dee | load | 1 | `abs 0x1dccc0` | `-` |
| `mov` at 0x122e20 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122e36 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122e68 | load | 4 | `ret(_lstNext) +0x18` | `-` |
| `mov` at 0x122e79 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122e7c | load | 4 | `ret(_lstNext) +0x18` | `-` |
| `mov` at 0x122e8b | load | 4 | `sp +0x4` | `-` |
| `test` at 0x122ed4 | load | 1 | `abs 0x1dccc0` | `-` |
| `mov` at 0x122f08 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122f15 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x122f48 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122f4b | load | 4 | `ret(_lstNext) +0x1c` | `-` |
| `mov` at 0x122f5a | load | 4 | `sp +0x8` | `-` |
| `test` at 0x122fb3 | load | 1 | `abs 0x1dccc0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122dfc | `_ffscMsg 0x11f040` | `0x122d79` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x122e0b | `_lstFirst 0x190308` | `0x1d4d68` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122e24 | `_inet_lnaof 0x188960` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122e39 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122e6c | `_inet_lnaof 0x188960` | `[ret(_lstNext)+24]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122e8e | `(unknown callee) (unknown target)` | `?` | `ret(_lstNext)+16` | `?` | `?` | `?` | `?` |
| `call` at 0x122ea3 | `_lstNext 0x1903b8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122eb6 | `_lstNext 0x1903b8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122ee2 | `_ffscMsg 0x11f040` | `0x122da3` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122ef3 | `_lstFirst 0x190308` | `0x1d4d68` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122f18 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122f5d | `(unknown callee) (unknown target)` | `?` | `ret(_lstNext)+16` | `?` | `?` | `?` | `?` |
| `call` at 0x122f82 | `_lstNext 0x1903b8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122f95 | `_lstNext 0x1903b8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122fc1 | `_ffscMsg 0x11f040` | `0x122dc9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x122df5 | `(([0x1dccc0]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x1) != 0x0` |
| `je` at 0x122e17 | `(ret(_lstFirst) & ret(_lstFirst)) == 0x0` | `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` |
| `jg` at 0x122e2f | `ret(_inet_lnaof) > 0xf (signed, 32-bit)` | `ret(_inet_lnaof) <= 0xf (signed, 32-bit)` |
| `jl` at 0x122e40 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jne` at 0x122e46 | `(ret(unknown) & ret(unknown)) != 0x0` | `(ret(unknown) & ret(unknown)) == 0x0` |
| `jle` at 0x122e77 | `ret(_inet_lnaof) <= 0xf (signed, 32-bit)` | `ret(_inet_lnaof) > 0xf (signed, 32-bit)` |
| `jne` at 0x122e81 | `Z == 0` | `not (Z == 0)` |
| `jl` at 0x122e95 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jne` at 0x122e9b | `(ret(unknown) & ret(unknown)) != 0x0` | `(ret(unknown) & ret(unknown)) == 0x0` |
| `je` at 0x122eaf | `(ret(_lstNext) & ret(_lstNext)) == 0x0` | `(ret(_lstNext) & ret(_lstNext)) != 0x0` |
| `je` at 0x122eb3 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x122ec2 | `(ret(_lstNext) & ret(_lstNext)) == 0x0` | `(ret(_lstNext) & ret(_lstNext)) != 0x0` |
| `je` at 0x122ec6 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x122ece | `Z == 0` | `not (Z == 0)` |
| `je` at 0x122edb | `(([0x1dccc0]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x1) != 0x0` |
| `je` at 0x122eff | `(ret(_lstFirst) & ret(_lstFirst)) == 0x0` | `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` |
| `jne` at 0x122f0e | `Z == 0` | `not (Z == 0)` |
| `jl` at 0x122f1f | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jne` at 0x122f21 | `(ret(unknown) & ret(unknown)) != 0x0` | `(ret(unknown) & ret(unknown)) == 0x0` |
| `jne` at 0x122f50 | `Z == 0` | `not (Z == 0)` |
| `jge` at 0x122f64 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x122f7a | `(ret(unknown) & ret(unknown)) != 0x0` | `(ret(unknown) & ret(unknown)) == 0x0` |
| `je` at 0x122f8e | `(ret(_lstNext) & ret(_lstNext)) == 0x0` | `(ret(_lstNext) & ret(_lstNext)) != 0x0` |
| `je` at 0x122f92 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x122fa1 | `(ret(_lstNext) & ret(_lstNext)) == 0x0` | `(ret(_lstNext) & ret(_lstNext)) != 0x0` |
| `je` at 0x122fa5 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x122fad | `Z == 0` | `not (Z == 0)` |
| `je` at 0x122fba | `(([0x1dccc0]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122fd1 | `eax` | `?` |

## `_identEnumerateRacks`

- `_identEnumerateRacks`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122848 to 0x1228f8.
  Established. The `_lstFirst` call presents candidate zero `0x1d4d68` (`call` at 0x12285b). The
  indexed four-byte stores have unknown targets and values (`mov` at 0x12287a, `mov` at 0x1228cb,
  `mov` at 0x1228ce). A separate four-byte update writes `[sp-16]+4` to `sp -0x10` (`add` at
  0x1228d1); its later branch is taken when `[sp-16] < [sp-12] (signed, 32-bit)` (`jl` at 0x1228dc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x122851 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12286c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122874 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122877 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12287a | store | 4 | `(unknown)` | `?` |
| `mov` at 0x122896 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x1228a3 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1228a9 | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x1228ac | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1228b2 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x1228b8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1228bb | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1228be | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1228c5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1228c8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1228cb | store | 4 | `(unknown)` | `?` |
| `mov` at 0x1228ce | store | 4 | `(unknown)` | `?` |
| `add` at 0x1228d1 | load | 4 | `sp -0x10` | `-` |
| `add` at 0x1228d1 | store | 4 | `sp -0x10` | `[sp-16]+4` |
| `mov` at 0x1228d6 | load | 4 | `sp -0xc` | `-` |
| `cmp` at 0x1228d9 | load | 4 | `sp -0x10` | `-` |
| `cmp` at 0x1228df | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12285b | `_lstFirst 0x190308` | `0x1d4d68` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x12287f | `_lstNext 0x1903b8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12286a | `Z == 1` | `not (Z == 1)` |
| `je` at 0x122872 | `Z == 1` | `not (Z == 1)` |
| `jle` at 0x12288b | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jge` at 0x122894 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jge` at 0x1228a1 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jae` at 0x1228c3 | `C == 0` | `not (C == 0)` |
| `jl` at 0x1228dc | `[sp-16] < [sp-12] (signed, 32-bit)` | `[sp-16] >= [sp-12] (signed, 32-bit)` |
| `jg` at 0x1228e2 | `[sp-8] > ? (signed, 32-bit)` | `[sp-8] <= ? (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1228ef | `eax` | `?` |

## `_identEnumerateModules`

- `_identEnumerateModules`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122788 to 0x122848.
  Established. The `_lstNext` site has unknown candidate values (`call` at 0x1227cf). Three indexed
  four-byte stores remain unresolved (`mov` at 0x1227c4, `mov` at 0x12281b, `mov` at 0x12281e). The
  stack update writes `[sp-16]+4` to `sp -0x10` (`add` at 0x122821). The return register value is
  unknown at the merged exit (`ret` at 0x12283f).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x122791 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1227b4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1227bd | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1227c1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1227c4 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x1227e6 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x1227f3 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1227f9 | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x1227fc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122802 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x122808 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12280b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12280e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122815 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122818 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12281b | store | 4 | `(unknown)` | `?` |
| `mov` at 0x12281e | store | 4 | `(unknown)` | `?` |
| `add` at 0x122821 | load | 4 | `sp -0x10` | `-` |
| `add` at 0x122821 | store | 4 | `sp -0x10` | `[sp-16]+4` |
| `mov` at 0x122826 | load | 4 | `sp -0xc` | `-` |
| `cmp` at 0x122829 | load | 4 | `sp -0x10` | `-` |
| `cmp` at 0x12282f | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12279b | `_lstFirst 0x190308` | `0x1d4d68` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x1227cf | `_lstNext 0x1903b8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1227aa | `Z == 1` | `not (Z == 1)` |
| `jg` at 0x1227b2 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `je` at 0x1227bb | `Z == 1` | `not (Z == 1)` |
| `jle` at 0x1227cc | `? <= 0x1 (signed, 32-bit)` | `? > 0x1 (signed, 32-bit)` |
| `jle` at 0x1227db | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jge` at 0x1227e4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jge` at 0x1227f1 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jae` at 0x122813 | `C == 0` | `not (C == 0)` |
| `jl` at 0x12282c | `[sp-16] < [sp-12] (signed, 32-bit)` | `[sp-16] >= [sp-12] (signed, 32-bit)` |
| `jg` at 0x122832 | `[sp-8] > ? (signed, 32-bit)` | `[sp-8] <= ? (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12283f | `eax` | `?` |

## `_identFindByModuleNum`

- `_identFindByModuleNum`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122948 to 0x1229b8.
  Established. The initial branch is taken when `[sp+4] == 0xffffffff` (`je` at 0x122957). A
  four-byte store through `[sp+8] +0x0` has unknown value (`mov` at 0x12297c). It lies on the
  fall-through of the branch taken when `([sp+8] & [sp+8]) == 0x0` (`je` at 0x12297a). The indexed
  four-byte load also has an unknown effective address (`mov` at 0x122970).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12294e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122951 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x122970 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12297c | store | 4 | `[sp+8] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12295e | `_lstFirst 0x190308` | `0x1d4d68` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x12298f | `_lstNext 0x1903b8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x122957 | `[sp+4] == 0xffffffff` | `[sp+4] != 0xffffffff` |
| `jne` at 0x122976 | `Z == 0` | `not (Z == 0)` |
| `je` at 0x12297a | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |
| `jle` at 0x12298c | `? <= 0x1 (signed, 32-bit)` | `? > 0x1 (signed, 32-bit)` |
| `jne` at 0x12299b | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1229a8 | `eax` | `?` |

## `_identClearTable`

- `_identClearTable`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122738 to 0x122788.
  Established. The `_lstFirst` result branch returns to the deletion path when `(ret(_lstFirst) &
  ret(_lstFirst)) != 0x0` (`jne` at 0x122774). That path saves `ret(_lstFirst)` as four bytes at `sp
  -0x8` (`mov` at 0x12274e), calls `_lstDelete` with leading candidates `0x1d4d68` and
  `ret(_lstFirst)` (`call` at 0x122751), then calls `_free` with candidate zero `[sp-8]` (`call` at
  0x12275d).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12274e | store | 4 | `sp -0x8` | `ret(_lstFirst)` |
| `mov` at 0x122759 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122751 | `_lstDelete 0x190228` | `0x1d4d68` | `ret(_lstFirst)` | `ret(_lstFirst)` | `?` | `?` | `?` |
| `call` at 0x12275d | `_free 0x17ec48` | `[sp-8]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12276a | `_lstFirst 0x190308` | `0x1d4d68` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x122774 | `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` | `(ret(_lstFirst) & ret(_lstFirst)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12277b | `eax` | `0x0` |

## `_identFindIdentityNodeByUniqueID`

- `_identFindIdentityNodeByUniqueID`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1235d8 to 0x123634.
  Established. The `_ffscCompareUniqueIDs` site has unknown candidate zero and candidate one
  `[sp+4]` (`call` at 0x1235fd); its branch is taken when `(ret(_ffscCompareUniqueIDs) &
  ret(_ffscCompareUniqueIDs)) != 0x0` (`jne` at 0x123607), reaching `_lstNext` with unknown
  candidate values (`call` at 0x123619). The tool retains an unknown merged return value (`ret` at
  0x123631).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1235dd | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1235e5 | `_lstFirst 0x190308` | `0x1d4d68` | `ebx` | `esi` | `ebp` | `?` | `?` |
| `call` at 0x1235fd | `_ffscCompareUniqueIDs 0x125578` | `?` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x123619 | `_lstNext 0x1903b8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x123607 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `jne` at 0x123625 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x123631 | `eax` | `?` |

## `_identFindConflictingIPAddr`

- `_identFindConflictingIPAddr`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122a28 to 0x122a98.
  Established. The `_ffscCompareUniqueIDs` site presents unknown candidate zero and candidate one
  `[sp+4]` (`call` at 0x122a5d); its zero-result branch bypasses the following loads (`je` at
  0x122a67). On the fall-through path, the four-byte load through `[sp+4] +0x8` is resolved (`mov`
  at 0x122a6c), while the other four-byte load has an unknown effective address (`mov` at 0x122a69).
  The equality branch is expressed only as `Z == 1` (`je` at 0x122a71).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x122a2e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122a69 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122a6c | load | 4 | `[sp+4] +0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122a4d | `_lstFirst 0x190308` | `0x1d4d68` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x122a5d | `_ffscCompareUniqueIDs 0x125578` | `?` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x122a74 | `_lstNext 0x1903b8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x122a33 | `([sp+4] & [sp+4]) != 0x0` | `([sp+4] & [sp+4]) == 0x0` |
| `je` at 0x122a67 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` |
| `je` at 0x122a71 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x122a80 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122a8d | `eax` | `?` |
