# MMSC identity state and NVRAM callers in the 6.4-origin image

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

[Fact sheets and complete listings](disasm/identity-state-64-origin-facts.txt) include all tool sections.

## `_identInit`

- `_identInit`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1224e8 to 0x122668.
  Established. Four-byte stores write `0x0` at absolute `0x1eb6cc` (`mov` at 0x1224fa) and
  `0x1eea94` (`mov` at 0x122504). Byte stores write `0x0` at absolute `0x1ee558` (`mov` at 0x12250e)
  and `0x1ee559` (`mov` at 0x122515). The `_nvramRead` sites present leading pairs `0x1`, `0x1dab70`
  (`call` at 0x122564) and `0x2`, `0x1ee564` (`call` at 0x122604). These sites do not establish
  record layouts or initialization success.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1224fa | store | 4 | `abs 0x1eb6cc` | `0x0` |
| `mov` at 0x122504 | store | 4 | `abs 0x1eea94` | `0x0` |
| `mov` at 0x12250e | store | 1 | `abs 0x1ee558` | `0x0` |
| `mov` at 0x122515 | store | 1 | `abs 0x1ee559` | `0x0` |
| `mov` at 0x122520 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x122526 | store | 1 | `(unknown)` | `?` |
| `mov` at 0x122540 | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x122553 | store | 4 | `abs 0x1ee568` | `0x1` |
| `test` at 0x122570 | load | 1 | `abs 0x1da699` | `-` |
| `movzx` at 0x122586 | load | 1 | `abs 0x1ee55d` | `-` |
| `movzx` at 0x122590 | load | 1 | `abs 0x1ee55e` | `-` |
| `movzx` at 0x12259c | load | 1 | `abs 0x1ee55f` | `-` |
| `mov` at 0x1225a5 | store | 4 | `abs 0x1dab70` | `?` |
| `mov` at 0x1225b7 | store | 4 | `abs 0x1dab70` | `0xf` |
| `mov` at 0x1225c1 | store | 4 | `abs 0x1dab74` | `0x64` |
| `mov` at 0x1225cb | store | 4 | `abs 0x1dab78` | `0xff000000` |
| `mov` at 0x1225e9 | load | 4 | `abs 0x1dab70` | `-` |
| `mov` at 0x1225ef | load | 4 | `abs 0x1dab74` | `-` |
| `mov` at 0x122610 | store | 4 | `abs 0x1ee564` | `0xffffffff` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1224f2 | `_bzero 0x170f20` | `0x1ee558` | `0x1c` | `ebp` | `?` | `?` | `?` |
| `call` at 0x122564 | `_nvramRead 0x126cf0` | `0x1` | `0x1dab70` | `?` | `?` | `?` | `?` |
| `call` at 0x12257e | `_ffscMsg 0x11ebd0` | `0x1224b9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1225dc | `_nvramWrite 0x126e80` | `0x1` | `0x1dab70` | `?` | `?` | `?` | `?` |
| `call` at 0x1225f5 | `_inet_makeaddr_b 0x186620` | `[0x1dab74]` | `[0x1dab70]` | `0x1ee560` | `?` | `?` | `?` |
| `call` at 0x122604 | `_nvramRead 0x126cf0` | `0x2` | `0x1ee564` | `?` | `?` | `?` | `?` |
| `call` at 0x12261f | `_lstInit 0x18ddc8` | `0x1d28e8` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12262c | `_identUpdate 0x1229b8` | `0x1ee558` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x122530 | `? <= 0x5 (signed, 32-bit)` | `? > 0x5 (signed, 32-bit)` |
| `jle` at 0x122551 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `je` at 0x12256e | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` |
| `je` at 0x122577 | `(([0x1da699]:1 & 0xff) & 0x1) == 0x0` | `(([0x1da699]:1 & 0xff) & 0x1) != 0x0` |
| `jbe` at 0x1225b5 | `? <= 0xffffee (unsigned, 32-bit)` | `? > 0xffffee (unsigned, 32-bit)` |
| `je` at 0x12260e | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` |
| `jl` at 0x122633 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12263a | `eax` | `0x0` |
| `ret` at 0x122650 | `eax` | `0xffffffff` |

## `_identUpdate`

- `_identUpdate`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1229b8 to 0x122ae8.
  Established. The `_identFindIdentityNodeByUniqueID` result branch is taken when
  `(ret(_identFindIdentityNodeByUniqueID) & ret(_identFindIdentityNodeByUniqueID)) == 0x0` (`je` at
  0x1229cd). The `_malloc` site presents candidate zero `0x2c` (`call` at 0x1229fa). The later
  `_memcpy` site has leading candidates `ret(_malloc)+16`, `[sp+4]`, and `0x1c` (`call` at
  0x122a5f). The `_lstAdd` site presents `0x1d28e8` and `ret(_malloc)` (`call` at 0x122a6d).
  Allocation and record sizes remain unproved by these candidates.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1229bd | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1229c1 | `_identFindIdentityNodeByUniqueID 0x122f18` | `[sp+4]` | `ebx` | `esi` | `ebp` | `?` | `?` |
| `call` at 0x1229d5 | `_clock_gettime 0x17981c` | `0x0` | `ret(_identFindIdentityNodeByUniqueID)+8` | `?` | `?` | `?` | `?` |
| `call` at 0x1229ef | `_memcpy 0x17dda0` | `ret(_identFindIdentityNodeByUniqueID)+16` | `[sp+4]` | `0x1c` | `?` | `?` | `?` |
| `call` at 0x1229fa | `_malloc 0x17c878` | `0x2c` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122a0d | `_ffscMsgS 0x11eca0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122a2e | `_clock_gettime 0x17981c` | `0x0` | `ret(_malloc)+8` | `?` | `?` | `?` | `?` |
| `call` at 0x122a3f | `_ffscMsgS 0x11eca0` | `0x122978` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122a48 | `_free 0x17c898` | `ret(_malloc)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122a5f | `_memcpy 0x17dda0` | `ret(_malloc)+16` | `[sp+4]` | `0x1c` | `?` | `?` | `?` |
| `call` at 0x122a6d | `_lstAdd 0x18dde8` | `0x1d28e8` | `ret(_malloc)` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1229cd | `(ret(_identFindIdentityNodeByUniqueID) & ret(_identFindIdentityNodeByUniqueID)) == 0x0` | `(ret(_identFindIdentityNodeByUniqueID) & ret(_identFindIdentityNodeByUniqueID)) != 0x0` |
| `je` at 0x1229df | `(ret(_clock_gettime) & ret(_clock_gettime)) == 0x0` | `(ret(_clock_gettime) & ret(_clock_gettime)) != 0x0` |
| `jne` at 0x122a06 | `(ret(_malloc) & ret(_malloc)) != 0x0` | `(ret(_malloc) & ret(_malloc)) == 0x0` |
| `je` at 0x122a38 | `(ret(_clock_gettime) & ret(_clock_gettime)) == 0x0` | `(ret(_clock_gettime) & ret(_clock_gettime)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122a7f | `eax` | `?` |

## `_identCleanOldEntries`

- `_identCleanOldEntries`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x121fd8 to 0x122078.
  Established. The first `_lstFirst` site has candidate zero `0x1d28e8` (`call` at 0x121fe3). The
  `_ffscCompareTimestamps` site has unknown candidate zero and candidate one `[sp+4]` (`call` at
  0x122007). Four-byte stores write `0x0` at absolute `0x1eb6cc` (`mov` at 0x12203b) and `0x1eea94`
  (`mov` at 0x122045). The common exit reports `0x0` in `eax` (`ret` at 0x122077). No age or
  identity role is assigned from the names.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x121fff | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122028 | load | 4 | `abs 0x1eb6cc` | `-` |
| `mov` at 0x12203b | store | 4 | `abs 0x1eb6cc` | `0x0` |
| `mov` at 0x122045 | store | 4 | `abs 0x1eea94` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x121fe3 | `_lstFirst 0x18df58` | `0x1d28e8` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x121ff5 | `_lstNext 0x18e008` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122007 | `_ffscCompareTimestamps 0x12536c` | `?` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x12201c | `_ffscCompareUniqueIDs 0x1253bc` | `?` | `0x1ee558` | `?` | `?` | `?` | `?` |
| `call` at 0x12202f | `_ffscCompareUniqueIDs 0x1253bc` | `?` | `[0x1eb6cc]` | `?` | `?` | `?` | `?` |
| `call` at 0x122055 | `_lstDelete 0x18de78` | `0x1d28e8` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12205e | `_free 0x17c898` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x121fef | `(ret(_lstFirst) & ret(_lstFirst)) == 0x0` | `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` |
| `jge` at 0x122011 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x122026 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` |
| `jne` at 0x122039 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `jne` at 0x12206a | `(ret(_lstNext) & ret(_lstNext)) != 0x0` | `(ret(_lstNext) & ret(_lstNext)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122077 | `eax` | `0x0` |

## `_identSetCapabilities`

- `_identSetCapabilities`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122bf8 to 0x122cd8.
  Established. The four-byte store at absolute `0x1ee568` writes `[sp+4]` (`mov` at 0x122bfe). The
  byte load reads absolute `0x1da699` (`test` at 0x122c03); its branch is taken when `(([0x1da699]:1
  & 0xff) & 0x1) == 0x0` (`je` at 0x122c0a). The `_ffscMsg` call presents leading candidates
  `0x122bcd` and `[sp+4]` (`call` at 0x122c12). A capability meaning for the stored value remains
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x122bfb | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122bfe | store | 4 | `abs 0x1ee568` | `[sp+4]` |
| `test` at 0x122c03 | load | 1 | `abs 0x1da699` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122c12 | `_ffscMsg 0x11ebd0` | `0x122bcd` | `[sp+4]` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x122c0a | `(([0x1da699]:1 & 0xff) & 0x1) == 0x0` | `(([0x1da699]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122c1c | `eax` | `0x0` |

## `_identSetIPAddr`

- `_identSetIPAddr`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122cd8 to 0x122dc8.
  Established. The `_ifAddrSet` site presents `0x122c43` and `sp-24` as its leading candidates
  (`call` at 0x122d29); its result branch is taken when `(ret(_ifAddrSet) & ret(_ifAddrSet)) == 0x0`
  (`je` at 0x122d33), reaching a four-byte store of `[[sp+4]]` at absolute `0x1ee560` (`mov` at
  0x122d4a). Other four-byte stores write `ret(_inet_netof)` at absolute `0x1dab74` (`mov` at
  0x122d4f) and `[sp-28]` at absolute `0x1dab70` (`mov` at 0x122d58). No interface-setting effect is
  inferred.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x122ce1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122ce4 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x122cef | store | 4 | `sp -0x1c` | `ret(_inet_lnaof)` |
| `mov` at 0x122cf2 | load | 4 | `[sp+4] +0x0` | `-` |
| `cmp` at 0x122cff | load | 4 | `abs 0x1dab74` | `-` |
| `mov` at 0x122d18 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x122d48 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x122d4a | store | 4 | `abs 0x1ee560` | `[[sp+4]]` |
| `mov` at 0x122d4f | store | 4 | `abs 0x1dab74` | `ret(_inet_netof)` |
| `mov` at 0x122d55 | load | 4 | `sp -0x1c` | `-` |
| `mov` at 0x122d58 | store | 4 | `abs 0x1dab70` | `[sp-28]` |
| `test` at 0x122d7e | load | 1 | `abs 0x1da699` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122ce7 | `_inet_lnaof 0x1865b0` | `[[sp+4]]` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x122cf5 | `_inet_netof 0x1866f0` | `[[sp+4]]` | `?` | `?` | `?` | `ret(_inet_lnaof)` | `?` |
| `call` at 0x122d0c | `_ffscMsg 0x11ebd0` | `0x122c1d` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122d1b | `_inet_ntoa_b 0x1868e0` | `[[sp+4]]` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x122d29 | `_ifAddrSet 0x187530` | `0x122c43` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x122d3b | `_ffscMsgS 0x11eca0` | `0x122c48` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x122d65 | `_nvramWrite 0x126e80` | `0x1` | `0x1dab70` | `?` | `?` | `?` | `?` |
| `call` at 0x122d76 | `_ffscMsg 0x11ebd0` | `0x122c72` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122d8d | `_ffscMsg 0x11ebd0` | `0x122cb0` | `sp-24` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x122d05 | `[0x1dab74] == ret(_inet_netof)` | `[0x1dab74] != ret(_inet_netof)` |
| `je` at 0x122d33 | `(ret(_ifAddrSet) & ret(_ifAddrSet)) == 0x0` | `(ret(_ifAddrSet) & ret(_ifAddrSet)) != 0x0` |
| `je` at 0x122d6f | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` |
| `je` at 0x122d85 | `(([0x1da699]:1 & 0xff) & 0x1) == 0x0` | `(([0x1da699]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122d9d | `eax` | `?` |

## `_identSetModuleNums`

- `_identSetModuleNums`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122dc8 to 0x122ea8.
  Established. The indexed four-byte store retains an unknown effective address and value (`mov` at
  0x122de3). The byte load reads absolute `0x1da699` (`test` at 0x122df2); its branch is taken when
  `(([0x1da699]:1 & 0xff) & 0x1) == 0x0` (`je` at 0x122df9). The `_ffscModuleNumToString` call has
  unknown candidate zero and candidate one `sp-16` (`call` at 0x122e1d). No module-number array
  extent is assigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x122dd1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122de0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122de3 | store | 4 | `(unknown)` | `?` |
| `test` at 0x122df2 | load | 1 | `abs 0x1da699` | `-` |
| `mov` at 0x122e15 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122e00 | `_ffscMsgN 0x11ec30` | `0x122d9e` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122e1d | `_ffscModuleNumToString 0x1255cc` | `?` | `sp-16` | `?` | `?` | `?` | `?` |
| `call` at 0x122e2b | `_ffscMsgN 0x11ec30` | `0x122db9` | `sp-16` | `?` | `?` | `?` | `?` |
| `call` at 0x122e44 | `_ffscMsg 0x11ebd0` | `0x122a8a` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x122df0 | `be flags condition (operands unknown)` | `not (be flags condition (operands unknown))` |
| `je` at 0x122df9 | `(([0x1da699]:1 & 0xff) & 0x1) == 0x0` | `(([0x1da699]:1 & 0xff) & 0x1) != 0x0` |
| `jbe` at 0x122e3d | `be flags condition (operands unknown)` | `not (be flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122e54 | `eax` | `0x0` |

## `_identFindByIPAddr`

- `_identFindByIPAddr`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122238 to 0x122288.
  Established. The `_lstFirst` site has candidate zero `0x1d28e8` (`call` at 0x122244). Four-byte
  loads include an unknown effective address (`mov` at 0x122258) and `[sp+4] +0x0` (`mov` at
  0x12225b). The `_lstNext` site retains unknown candidates (`call` at 0x122269). The exit value is
  unknown (`ret` at 0x12227f), so the body does not establish a successful address lookup.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12223c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122258 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12225b | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x122279 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122244 | `_lstFirst 0x18df58` | `0x1d28e8` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x122269 | `_lstNext 0x18e008` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x12225f | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x122275 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12227f | `eax` | `?` |

## `_identFindByModuleNum`

- `_identFindByModuleNum`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122288 to 0x1222f8.
  Established. The initial branch is taken when `[sp+4] == 0xffffffff` (`je` at 0x122297). A
  four-byte store through `[sp+8] +0x0` retains an unknown value (`mov` at 0x1222bc). The prior
  branch falls through when `([sp+8] & [sp+8]) != 0x0` (`je` at 0x1222ba). The `_lstNext` site has
  unknown candidates (`call` at 0x1222cf). The stored value is not assigned an index or module
  meaning.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12228e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122291 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1222b0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1222bc | store | 4 | `[sp+8] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12229e | `_lstFirst 0x18df58` | `0x1d28e8` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x1222cf | `_lstNext 0x18e008` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x122297 | `[sp+4] == 0xffffffff` | `[sp+4] != 0xffffffff` |
| `jne` at 0x1222b6 | `Z == 0` | `not (Z == 0)` |
| `je` at 0x1222ba | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |
| `jle` at 0x1222cc | `? <= 0x1 (signed, 32-bit)` | `? > 0x1 (signed, 32-bit)` |
| `jne` at 0x1222db | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1222e8 | `eax` | `?` |

## `_identFindByUniqueID`

- `_identFindByUniqueID`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122348 to 0x122368.
  Established. The four-byte load reads `sp +0x4` (`mov` at 0x12234b). The
  `_identFindIdentityNodeByUniqueID` site presents candidate zero `[sp+4]` (`call` at 0x12234f); its
  result branch is taken when `(ret(_identFindIdentityNodeByUniqueID) &
  ret(_identFindIdentityNodeByUniqueID)) == 0x0` (`je` at 0x122358). The merged exit remains unknown
  (`ret` at 0x122362). No nonzero return relation is filled in from the raw listing.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12234b | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12234f | `_identFindIdentityNodeByUniqueID 0x122f18` | `[sp+4]` | `ebp` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x122358 | `(ret(_identFindIdentityNodeByUniqueID) & ret(_identFindIdentityNodeByUniqueID)) == 0x0` | `(ret(_identFindIdentityNodeByUniqueID) & ret(_identFindIdentityNodeByUniqueID)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122362 | `eax` | `?` |

## `_identFindConflictingIPAddr`

- `_identFindConflictingIPAddr`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122368 to 0x1223d8.
  Established. The `_ffscCompareUniqueIDs` call has unknown candidate zero and candidate one
  `[sp+4]` (`call` at 0x12239d); its result branch is taken when `(ret(_ffscCompareUniqueIDs) &
  ret(_ffscCompareUniqueIDs)) == 0x0` (`je` at 0x1223a7). The following four-byte loads include an
  unknown effective address (`mov` at 0x1223a9) and `[sp+4] +0x8` (`mov` at 0x1223ac).
  Conflict-search semantics remain unassigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12236e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1223a9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1223ac | load | 4 | `[sp+4] +0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12238d | `_lstFirst 0x18df58` | `0x1d28e8` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x12239d | `_ffscCompareUniqueIDs 0x1253bc` | `?` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x1223b4 | `_lstNext 0x18e008` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x122373 | `([sp+4] & [sp+4]) != 0x0` | `([sp+4] & [sp+4]) == 0x0` |
| `je` at 0x1223a7 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` |
| `je` at 0x1223b1 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x1223c0 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1223cd | `eax` | `?` |

## `_identScanForConflicts`

- `_identScanForConflicts`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122728 to 0x1229b8.
  Established. Four indirect call sites have no established callee (`call` at 0x122779, `call` at
  0x1227ce, `call` at 0x122858, `call` at 0x12289d). The first and third retain unknown candidate
  slots (`call` at 0x122779, `call` at 0x122858). The second and fourth have unknown candidate zero
  and candidate one `ret(_lstNext)+16` (`call` at 0x1227ce, `call` at 0x12289d). A four-byte load
  through `ret(_lstNext) +0x1c` appears later (`mov` at 0x12288b). The supplied callbacks and
  conflict-resolution effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `test` at 0x12272e | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x122760 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122776 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1227a8 | load | 4 | `ret(_lstNext) +0x18` | `-` |
| `mov` at 0x1227b9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1227bc | load | 4 | `ret(_lstNext) +0x18` | `-` |
| `mov` at 0x1227cb | load | 4 | `sp +0x4` | `-` |
| `test` at 0x122814 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x122848 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122855 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x122888 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12288b | load | 4 | `ret(_lstNext) +0x1c` | `-` |
| `mov` at 0x12289a | load | 4 | `sp +0x8` | `-` |
| `test` at 0x1228f3 | load | 1 | `abs 0x1da698` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12273c | `_ffscMsg 0x11ebd0` | `0x1226b9` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x12274b | `_lstFirst 0x18df58` | `0x1d28e8` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122764 | `_inet_lnaof 0x1865b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122779 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1227ac | `_inet_lnaof 0x1865b0` | `[ret(_lstNext)+24]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1227ce | `(unknown callee) (unknown target)` | `?` | `ret(_lstNext)+16` | `?` | `?` | `?` | `?` |
| `call` at 0x1227e3 | `_lstNext 0x18e008` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1227f6 | `_lstNext 0x18e008` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122822 | `_ffscMsg 0x11ebd0` | `0x1226e3` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122833 | `_lstFirst 0x18df58` | `0x1d28e8` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122858 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12289d | `(unknown callee) (unknown target)` | `?` | `ret(_lstNext)+16` | `?` | `?` | `?` | `?` |
| `call` at 0x1228c2 | `_lstNext 0x18e008` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1228d5 | `_lstNext 0x18e008` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122901 | `_ffscMsg 0x11ebd0` | `0x122709` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x122735 | `(([0x1da698]:1 & 0xff) & 0x1) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x1) != 0x0` |
| `je` at 0x122757 | `(ret(_lstFirst) & ret(_lstFirst)) == 0x0` | `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` |
| `jg` at 0x12276f | `ret(_inet_lnaof) > 0xf (signed, 32-bit)` | `ret(_inet_lnaof) <= 0xf (signed, 32-bit)` |
| `jl` at 0x122780 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jne` at 0x122786 | `(ret(unknown) & ret(unknown)) != 0x0` | `(ret(unknown) & ret(unknown)) == 0x0` |
| `jle` at 0x1227b7 | `ret(_inet_lnaof) <= 0xf (signed, 32-bit)` | `ret(_inet_lnaof) > 0xf (signed, 32-bit)` |
| `jne` at 0x1227c1 | `Z == 0` | `not (Z == 0)` |
| `jl` at 0x1227d5 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jne` at 0x1227db | `(ret(unknown) & ret(unknown)) != 0x0` | `(ret(unknown) & ret(unknown)) == 0x0` |
| `je` at 0x1227ef | `(ret(_lstNext) & ret(_lstNext)) == 0x0` | `(ret(_lstNext) & ret(_lstNext)) != 0x0` |
| `je` at 0x1227f3 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x122802 | `(ret(_lstNext) & ret(_lstNext)) == 0x0` | `(ret(_lstNext) & ret(_lstNext)) != 0x0` |
| `je` at 0x122806 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x12280e | `Z == 0` | `not (Z == 0)` |
| `je` at 0x12281b | `(([0x1da698]:1 & 0xff) & 0x1) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x1) != 0x0` |
| `je` at 0x12283f | `(ret(_lstFirst) & ret(_lstFirst)) == 0x0` | `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` |
| `jne` at 0x12284e | `Z == 0` | `not (Z == 0)` |
| `jl` at 0x12285f | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jne` at 0x122861 | `(ret(unknown) & ret(unknown)) != 0x0` | `(ret(unknown) & ret(unknown)) == 0x0` |
| `jne` at 0x122890 | `Z == 0` | `not (Z == 0)` |
| `jge` at 0x1228a4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x1228ba | `(ret(unknown) & ret(unknown)) != 0x0` | `(ret(unknown) & ret(unknown)) == 0x0` |
| `je` at 0x1228ce | `(ret(_lstNext) & ret(_lstNext)) == 0x0` | `(ret(_lstNext) & ret(_lstNext)) != 0x0` |
| `je` at 0x1228d2 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x1228e1 | `(ret(_lstNext) & ret(_lstNext)) == 0x0` | `(ret(_lstNext) & ret(_lstNext)) != 0x0` |
| `je` at 0x1228e5 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x1228ed | `Z == 0` | `not (Z == 0)` |
| `je` at 0x1228fa | `(([0x1da698]:1 & 0xff) & 0x1) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122911 | `eax` | `?` |

## `_identEnumerateRacks`

- `_identEnumerateRacks`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122188 to 0x122238.
  Established. The `_lstFirst` site presents candidate zero `0x1d28e8` (`call` at 0x12219b). Indexed
  four-byte stores have unknown targets and values (`mov` at 0x1221ba, `mov` at 0x12220b, `mov` at
  0x12220e). A separate four-byte store at `sp -0x10` writes `[sp-16]+4` (`add` at 0x122211). The
  backward branch is taken when `[sp-16] < [sp-12] (signed, 32-bit)` (`jl` at 0x12221c). No
  enumeration count or ordering guarantee is inferred.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x122191 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1221ac | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1221b4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1221b7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1221ba | store | 4 | `(unknown)` | `?` |
| `mov` at 0x1221d6 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x1221e3 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1221e9 | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x1221ec | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1221f2 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x1221f8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1221fb | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1221fe | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122205 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122208 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12220b | store | 4 | `(unknown)` | `?` |
| `mov` at 0x12220e | store | 4 | `(unknown)` | `?` |
| `add` at 0x122211 | load | 4 | `sp -0x10` | `-` |
| `add` at 0x122211 | store | 4 | `sp -0x10` | `[sp-16]+4` |
| `mov` at 0x122216 | load | 4 | `sp -0xc` | `-` |
| `cmp` at 0x122219 | load | 4 | `sp -0x10` | `-` |
| `cmp` at 0x12221f | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12219b | `_lstFirst 0x18df58` | `0x1d28e8` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x1221bf | `_lstNext 0x18e008` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1221aa | `Z == 1` | `not (Z == 1)` |
| `je` at 0x1221b2 | `Z == 1` | `not (Z == 1)` |
| `jle` at 0x1221cb | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jge` at 0x1221d4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jge` at 0x1221e1 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jae` at 0x122203 | `C == 0` | `not (C == 0)` |
| `jl` at 0x12221c | `[sp-16] < [sp-12] (signed, 32-bit)` | `[sp-16] >= [sp-12] (signed, 32-bit)` |
| `jg` at 0x122222 | `[sp-8] > ? (signed, 32-bit)` | `[sp-8] <= ? (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12222f | `eax` | `?` |

## `_identEnumerateModules`

- `_identEnumerateModules`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1220c8 to 0x122188.
  Established. A four-byte store at an unknown target has an unknown value (`mov` at 0x122104). The
  later indexed four-byte stores also retain unknown targets and values (`mov` at 0x12215b, `mov` at
  0x12215e). The four-byte store at `sp -0x10` writes `[sp-16]+4` (`add` at 0x122161); its loop
  branch is taken when `[sp-16] < [sp-12] (signed, 32-bit)` (`jl` at 0x12216c). These effects do not
  establish an output-array extent.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1220d1 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1220f4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1220fd | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122101 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122104 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x122126 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x122133 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122139 | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x12213c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122142 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x122148 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12214b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12214e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122155 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122158 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12215b | store | 4 | `(unknown)` | `?` |
| `mov` at 0x12215e | store | 4 | `(unknown)` | `?` |
| `add` at 0x122161 | load | 4 | `sp -0x10` | `-` |
| `add` at 0x122161 | store | 4 | `sp -0x10` | `[sp-16]+4` |
| `mov` at 0x122166 | load | 4 | `sp -0xc` | `-` |
| `cmp` at 0x122169 | load | 4 | `sp -0x10` | `-` |
| `cmp` at 0x12216f | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1220db | `_lstFirst 0x18df58` | `0x1d28e8` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x12210f | `_lstNext 0x18e008` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1220ea | `Z == 1` | `not (Z == 1)` |
| `jg` at 0x1220f2 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `je` at 0x1220fb | `Z == 1` | `not (Z == 1)` |
| `jle` at 0x12210c | `? <= 0x1 (signed, 32-bit)` | `? > 0x1 (signed, 32-bit)` |
| `jle` at 0x12211b | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jge` at 0x122124 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jge` at 0x122131 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jae` at 0x122153 | `C == 0` | `not (C == 0)` |
| `jl` at 0x12216c | `[sp-16] < [sp-12] (signed, 32-bit)` | `[sp-16] >= [sp-12] (signed, 32-bit)` |
| `jg` at 0x122172 | `[sp-8] > ? (signed, 32-bit)` | `[sp-8] <= ? (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12217f | `eax` | `?` |

## `_identClearTable`

- `_identClearTable`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122078 to 0x1220c8.
  Established. The `_lstDelete` site presents leading candidates `0x1d28e8` and `ret(_lstFirst)`
  (`call` at 0x122091). The four-byte store at `sp -0x8` retains `ret(_lstFirst)` (`mov` at
  0x12208e). The `_free` site has candidate zero `[sp-8]` (`call` at 0x12209d). The loop branch is
  taken when `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` (`jne` at 0x1220b4). Deallocation and
  table-clearing effects remain unproved by the call names.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12208e | store | 4 | `sp -0x8` | `ret(_lstFirst)` |
| `mov` at 0x122099 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122091 | `_lstDelete 0x18de78` | `0x1d28e8` | `ret(_lstFirst)` | `ret(_lstFirst)` | `?` | `?` | `?` |
| `call` at 0x12209d | `_free 0x17c898` | `[sp-8]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1220aa | `_lstFirst 0x18df58` | `0x1d28e8` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x1220b4 | `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` | `(ret(_lstFirst) & ret(_lstFirst)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1220bb | `eax` | `0x0` |

## `_identFindIdentityNodeByUniqueID`

- `_identFindIdentityNodeByUniqueID`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122f18 to 0x122f74.
  Established. The `_lstFirst` site presents candidate zero `0x1d28e8` (`call` at 0x122f25). The
  `_ffscCompareUniqueIDs` call has unknown candidate zero and candidate one `[sp+4]` (`call` at
  0x122f3d); its result branch is taken when `(ret(_ffscCompareUniqueIDs) &
  ret(_ffscCompareUniqueIDs)) != 0x0` (`jne` at 0x122f47), reaching `_lstNext` with unknown
  candidates (`call` at 0x122f59). The exit value remains unknown (`ret` at 0x122f71).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x122f1d | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122f25 | `_lstFirst 0x18df58` | `0x1d28e8` | `ebx` | `esi` | `ebp` | `?` | `?` |
| `call` at 0x122f3d | `_ffscCompareUniqueIDs 0x1253bc` | `?` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x122f59 | `_lstNext 0x18e008` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x122f47 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `jne` at 0x122f65 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122f71 | `eax` | `?` |
