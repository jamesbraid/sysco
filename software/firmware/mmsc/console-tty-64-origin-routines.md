# MMSC console and TTY callers in the 6.4-origin image

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

[Fact sheets and complete listings](disasm/console-tty-64-origin-facts.txt) include all tool sections.


The decoder stops before the ends of four spans. These raw bytes complete the three
literal tails and the remaining table bytes of `_csSysIn`. The independent tail analysis
at `0x11cfb4` to `0x11d010` follows the fifteen entry analyses in the retained facts.
It decodes the remaining bytes with a fresh entry state; its accesses, calls and returns
do not supply caller state for `_csSysIn`.

| Span | Raw start | Exclusive end | Bytes |
|---|---|---|---|
| `_csSysIn` table remainder | `0x11cfa4` | `0x11cfb4` | `f0 cf 11 00 d0 cf 11 00 f0 cf 11 00 c0 cf 11 00` |
| `_ceEnterCopySysMode` | `0x118cc1` | `0x118cc8` | `64 65 00 90 90 90 90` |
| `_ceEnterWatchSysMode` | `0x119102` | `0x119108` | `64 65 00 90 90 90` |
| `_cmCheckTTYFlags` | `0x119dc9` | `0x119dd0` | `69 74 20 2a 2a 2a 00` |

## `_initTTYs`

- `_initTTYs`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1245c4 to 0x124840.
  Established. The hardware role is Unknown. `_nvramRead` and `_nvramWrite` have the same leading
  candidates `0x4`, `0x1da030` (`call` at 0x1245f4, `call` at 0x1246a8). `_ttyDevCreate` receives
  `[sp-28]` and `ret(_sysSerialChanGet)` (`call` at 0x124706). The `_ioctl` sites retain candidate
  one `0x4` and `0x3`, with unknown candidate zero and two (`call` at 0x124775, `call` at 0x1247b0).
  Four-byte assignments copy `[0x1ebbec]` to absolute `0x1ee990` and `[0x1ebbf0]` to `0x1ee994`
  (`mov` at 0x124825, `mov` at 0x12482f). Device identities and command semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1245e2 | load | 4 | `ret(___stderr) +0x0` | `-` |
| `mov` at 0x12461c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x124622 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x124628 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12462c | store | 4 | `(unknown)` | `?` |
| `mov` at 0x124630 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x124634 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x124638 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12463c | store | 4 | `(unknown)` | `?` |
| `cmp` at 0x124659 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x124663 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x124668 | store | 2 | `(unknown)` | `?` |
| `test` at 0x124674 | load | 1 | `(unknown)` | `-` |
| `or` at 0x12467d | load | 2 | `(unknown)` | `-` |
| `or` at 0x12467d | store | 2 | `(unknown)` | `?` |
| `mov` at 0x124686 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x12468b | store | 2 | `(unknown)` | `?` |
| `mov` at 0x1246bc | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x1246d4 | store | 4 | `sp -0x1c` | `sp-24` |
| `mov` at 0x1246e2 | load | 4 | `sp -0x1c` | `-` |
| `mov` at 0x124702 | load | 4 | `sp -0x1c` | `-` |
| `mov` at 0x124712 | load | 4 | `sp -0x1c` | `-` |
| `mov` at 0x124728 | load | 4 | `sp -0x1c` | `-` |
| `mov` at 0x124734 | store | 4 | `(unknown)` | `ret(_open)` |
| `mov` at 0x12473b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x124746 | load | 4 | `sp -0x1c` | `-` |
| `mov` at 0x124754 | load | 4 | `ret(___stderr) +0x0` | `-` |
| `mov` at 0x124764 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12476d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x124781 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x124788 | load | 4 | `sp -0x1c` | `-` |
| `cmp` at 0x124796 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1247a8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1247bd | load | 4 | `sp -0x1c` | `-` |
| `mov` at 0x1247cb | load | 4 | `ret(___stderr) +0x0` | `-` |
| `mov` at 0x1247d6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1247e6 | store | 4 | `(unknown)` | `0xffffffff` |
| `cmp` at 0x1247f4 | load | 2 | `(unknown)` | `-` |
| `movsx` at 0x1247fe | load | 2 | `(unknown)` | `-` |
| `mov` at 0x124805 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12480c | store | 4 | `(unknown)` | `?` |
| `mov` at 0x124820 | load | 4 | `abs 0x1ebbec` | `-` |
| `mov` at 0x124825 | store | 4 | `abs 0x1ee990` | `[0x1ebbec]` |
| `mov` at 0x12482a | load | 4 | `abs 0x1ebbf0` | `-` |
| `mov` at 0x12482f | store | 4 | `abs 0x1ee994` | `[0x1ebbf0]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1245cf | `_ttyDrv 0x18fca8` | `ebx` | `esi` | `edi` | `?` | `?` | `?` |
| `call` at 0x1245dd | `___stderr 0x18bdcc` | `0x124519` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1245e5 | `_fprintf 0x17a13c` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1245f4 | `_nvramRead 0x126cf0` | `0x4` | `0x1da030` | `?` | `?` | `?` | `?` |
| `call` at 0x124607 | `_bzero 0x170f20` | `0x1da030` | `0x60` | `?` | `?` | `?` | `?` |
| `call` at 0x1246a8 | `_nvramWrite 0x126e80` | `0x4` | `0x1da030` | `?` | `?` | `?` | `?` |
| `call` at 0x1246e6 | `_sprintf 0x171680` | `?` | `0x124537` | `?` | `?` | `?` | `?` |
| `call` at 0x1246f9 | `_sysSerialChanGet 0x10b788` | `?` | `0x1000` | `0x1000` | `?` | `?` | `?` |
| `call` at 0x124706 | `_ttyDevCreate 0x18fd08` | `[sp-28]` | `ret(_sysSerialChanGet)` | `?` | `?` | `?` | `?` |
| `call` at 0x12472c | `_open 0x186d30` | `[sp-28]` | `0x2` | `0x0` | `?` | `?` | `?` |
| `call` at 0x12474f | `___stderr 0x18bdcc` | `?` | `[sp-28]` | `?` | `?` | `?` | `?` |
| `call` at 0x124757 | `_fprintf 0x17a13c` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x124775 | `_ioctl 0x186f90` | `?` | `0x4` | `?` | `?` | `?` | `?` |
| `call` at 0x1247b0 | `_ioctl 0x186f90` | `?` | `0x3` | `?` | `?` | `?` | `?` |
| `call` at 0x1247c6 | `___stderr 0x18bdcc` | `?` | `[sp-28]` | `?` | `?` | `?` | `?` |
| `call` at 0x1247ce | `_fprintf 0x17a13c` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1247de | `_close 0x186ed0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1245d6 | `(ret(_ttyDrv) & ret(_ttyDrv)) == 0x0` | `(ret(_ttyDrv) & ret(_ttyDrv)) != 0x0` |
| `je` at 0x1245fe | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` |
| `jle` at 0x124646 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jl` at 0x124661 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jne` at 0x12467b | `Z == 0` | `not (Z == 0)` |
| `jle` at 0x12469b | `? <= 0x5 (signed, 32-bit)` | `? > 0x5 (signed, 32-bit)` |
| `je` at 0x12469f | `Z == 1` | `not (Z == 1)` |
| `jle` at 0x1246cd | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `je` at 0x124710 | `(ret(_ttyDevCreate) & ret(_ttyDevCreate)) == 0x0` | `(ret(_ttyDevCreate) & ret(_ttyDevCreate)) != 0x0` |
| `jge` at 0x124744 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jge` at 0x12477f | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x12479e | `Z == 0` | `not (Z == 0)` |
| `jge` at 0x1247ba | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jl` at 0x1247fc | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jle` at 0x12481a | `? <= 0x5 (signed, 32-bit)` | `? > 0x5 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12483f | `eax` | `0x0` |

## `_ffscSetDebugConsole`

- `_ffscSetDebugConsole`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x11ed80 to 0x11ee80.
  Established. The two-byte entry load reads `sp +0x4` (`mov` at 0x11ed83). The branch falls through
  when the printed masked self-test is zero (`jne` at 0x11ed8a). The next branch falls through
  when `[0x1d9b94] >= 0x0 (signed, 32-bit)` (`jl` at 0x11ed93), reaching the four-byte
  `0xffffffff` store at absolute `0x1d9b94` (`mov` at 0x11ed95). A later computed jump has unresolved edges (`jmp` at
  0x11edbd). The alternate diagnostic arm calls `_fprintf` with candidate zero `[ret(___stderr)]`
  (`call` at 0x11ee61). Selector-only stores are excluded, leaving the console-selection role
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x11ed83 | load | 2 | `sp +0x4` | `-` |
| `cmp` at 0x11ed8c | load | 4 | `abs 0x1d9b94` | `-` |
| `mov` at 0x11ed95 | store | 4 | `abs 0x1d9b94` | `0xffffffff` |
| `jmp` at 0x11edbd | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11ee5e | load | 4 | `ret(___stderr) +0x0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x11ee59 | `___stderr 0x18bdcc` | `0x11ed49` | `[sp+4]:2` | `?` | `?` | `?` | `?` |
| `call` at 0x11ee61 | `_fprintf 0x17a13c` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x11ed8a | `(([sp+4]:2 & 0xffff) & ([sp+4]:2 & 0xffff)) != 0x0` | `(([sp+4]:2 & 0xffff) & ([sp+4]:2 & 0xffff)) == 0x0` |
| `jl` at 0x11ed93 | `[0x1d9b94] < 0x0 (signed, 32-bit)` | `[0x1d9b94] >= 0x0 (signed, 32-bit)` |
| `ja` at 0x11edb7 | `? > 0xb (unsigned, 32-bit)` | `? <= 0xb (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x11eda4 | `eax` | `0x0` |
| `ret` at 0x11ee6e | `eax` | `0x1` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x11edbd).

## `_csSysIn`

- `_csSysIn`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x11cf70 to 0x11d010.
  Established. The four-byte entry load is from `[sp+4] +0x18` (`mov` at 0x11cf76). The unsigned
  branch takes its arm when `? > 0x8 (unsigned, 32-bit)` (`ja` at 0x11cf7d). The computed transfer
  remains unresolved (`jmp` at 0x11cf7f). Linear decoding stops within embedded table bytes. The known taken target
  `0x11cff0` is absent from this entry decode (`ja` at 0x11cf7d); its effects are also missing
  from the tables below. An independent aligned tail listing is retained separately, with fresh
  entry state rather than caller state. Callees and return values are not established by this
  entry analysis. Role: Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x11cf73 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x11cf76 | load | 4 | `[sp+4] +0x18` | `-` |
| `jmp` at 0x11cf7f | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x11cf7d | `? > 0x8 (unsigned, 32-bit)` | `? <= 0x8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x11cf7f).

## `_csSysInEcho`

- `_csSysInEcho`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x11d010 to 0x11d090.
  Established. The `_bufAppendAddMsgs` call carries the leading candidates `[[sp+4]+116]`,
  `[[sp+4]+104]`, `[[[sp+4]+104]+16]`, `[[sp+4]+120]` (`call` at 0x11d029); its result expression
  reaches the exit (`ret` at 0x11d031). No conditional branch or explicit memory store is present.
  The object fields and append behavior inside the callee are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x11d013 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x11d016 | load | 4 | `[sp+4] +0x78` | `-` |
| `mov` at 0x11d01a | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x11d01d | load | 4 | `[[sp+4]+104] +0x10` | `-` |
| `mov` at 0x11d021 | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x11d025 | load | 4 | `[sp+4] +0x74` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x11d029 | `_bufAppendAddMsgs 0x10eea8` | `[[sp+4]+116]` | `[[sp+4]+104]` | `[[[sp+4]+104]+16]` | `[[sp+4]+120]` | `ebp` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x11d031 | `eax` | `ret(_bufAppendAddMsgs)` |

## `_csSysInToUserOut`

- `_csSysInToUserOut`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x11d730 to 0x11d7a0.
  Established. `_logWrite` has candidates `[0x1dab5c]`, `[[[sp+4]+104]+4]`, `[sp+8]` (`call` at
  0x11d749). The branch takes the alternate arm when `[[sp+4]+24] == 0x9` (`je` at 0x11d757). That
  arm calls `_bufLeftAlign` with `[[sp+4]+104]` and `([[[sp+4]+104]+4] + [sp+8])` (`call` at
  0x11d78d). The other arm calls `_bufAppendAddMsgs` (`call` at 0x11d766). Buffer and log effects
  remain Unknown in this caller.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x11d735 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x11d738 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x11d73c | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x11d73f | load | 4 | `[[sp+4]+104] +0x4` | `-` |
| `mov` at 0x11d743 | load | 4 | `abs 0x1dab5c` | `-` |
| `mov` at 0x11d751 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x11d759 | load | 4 | `[sp+4] +0x78` | `-` |
| `mov` at 0x11d75e | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x11d762 | load | 4 | `[sp+4] +0x74` | `-` |
| `mov` at 0x11d780 | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x11d783 | load | 4 | `[[sp+4]+104] +0x4` | `-` |
| `mov` at 0x11d789 | load | 4 | `[sp+4] +0x68` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x11d749 | `_logWrite 0x124de0` | `[0x1dab5c]` | `[[[sp+4]+104]+4]` | `[sp+8]` | `ebx` | `esi` | `ebp` |
| `call` at 0x11d766 | `_bufAppendAddMsgs 0x10eea8` | `[[sp+4]+116]` | `[[sp+4]+104]` | `[sp+8]` | `[[sp+4]+120]` | `?` | `?` |
| `call` at 0x11d78d | `_bufLeftAlign 0x10f838` | `[[sp+4]+104]` | `([[[sp+4]+104]+4] + [sp+8])` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x11d757 | `[[sp+4]+24] == 0x9` | `[[sp+4]+24] != 0x9` |
| `je` at 0x11d76d | `(ret(_bufAppendAddMsgs) & ret(_bufAppendAddMsgs)) == 0x0` | `(ret(_bufAppendAddMsgs) & ret(_bufAppendAddMsgs)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x11d79c | `eax` | `?` |

## `_ceEnterCopySysMode`

- `_ceEnterCopySysMode`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x118bd8 to 0x118cc8.
  Established. Four-byte assignments put `0x5`, `0x1`, `0x1` at `[sp+4] +0x18`, `+0x1c`, `+0x24`
  respectively (`mov` at 0x118bf6, `mov` at 0x118bfd, `mov` at 0x118c04). The `_bufAttachFD` sites
  include pairs `[[sp+4]+104]`, `[[sp+4]+96]` and `[[sp+4]+108]`, `[[sp+4]+96]` (`call` at 0x118c2b,
  `call` at 0x118c3b). Two stores at `[sp+4] +0x3c` have unknown values (`mov` at 0x118c71, `mov` at
  0x118c82). The field meanings and mode outcome are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x118bdc | load | 4 | `sp +0x4` | `-` |
| `test` at 0x118bdf | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x118bf6 | store | 4 | `[sp+4] +0x18` | `0x5` |
| `mov` at 0x118bfd | store | 4 | `[sp+4] +0x1c` | `0x1` |
| `mov` at 0x118c04 | store | 4 | `[sp+4] +0x24` | `0x1` |
| `mov` at 0x118c0b | load | 4 | `[sp+4] +0x70` | `-` |
| `mov` at 0x118c17 | load | 4 | `[sp+4] +0x74` | `-` |
| `mov` at 0x118c23 | load | 4 | `[sp+4] +0x60` | `-` |
| `mov` at 0x118c27 | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x118c33 | load | 4 | `[sp+4] +0x60` | `-` |
| `mov` at 0x118c37 | load | 4 | `[sp+4] +0x6c` | `-` |
| `mov` at 0x118c43 | load | 4 | `[sp+4] +0x4c` | `-` |
| `mov` at 0x118c47 | load | 4 | `[sp+4] +0x70` | `-` |
| `mov` at 0x118c53 | load | 4 | `[sp+4] +0x4c` | `-` |
| `mov` at 0x118c57 | load | 4 | `[sp+4] +0x74` | `-` |
| `mov` at 0x118c63 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x118c6b | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x118c71 | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x118c74 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x118c7c | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x118c82 | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x118c85 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x118c8c | store | 4 | `[sp+4] +0x44` | `0x0` |
| `mov` at 0x118c93 | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x118c9e | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x118bee | `_ffscMsg 0x11ebd0` | `0x118bb7` | `[sp+4]` | `ebx` | `ebp` | `?` | `?` |
| `call` at 0x118c0f | `_bufClear 0x10f438` | `[[sp+4]+112]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x118c1b | `_bufClear 0x10f438` | `[[sp+4]+116]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x118c2b | `_bufAttachFD 0x10f378` | `[[sp+4]+104]` | `[[sp+4]+96]` | `?` | `?` | `?` | `?` |
| `call` at 0x118c3b | `_bufAttachFD 0x10f378` | `[[sp+4]+108]` | `[[sp+4]+96]` | `?` | `?` | `?` | `?` |
| `call` at 0x118c4b | `_bufAttachFD 0x10f378` | `[[sp+4]+112]` | `[[sp+4]+76]` | `?` | `?` | `?` | `?` |
| `call` at 0x118c5b | `_bufAttachFD 0x10f378` | `[[sp+4]+116]` | `[[sp+4]+76]` | `?` | `?` | `?` | `?` |
| `call` at 0x118c97 | `_bufRelease 0x10fa18` | `[[sp+4]+104]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x118be6 | `(([0x1da698]:1 & 0xff) & 0x4) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x4) != 0x0` |
| `je` at 0x118c69 | `(? & 0x40) == 0x0` | `(? & 0x40) != 0x0` |
| `jge` at 0x118c7a | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x118c8a | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x118ca4 | `eax` | `0x0` |

## `_clLeaveCopySysMode`

- `_clLeaveCopySysMode`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1197f8 to 0x1198e8.
  Established. The `_close` candidate is `[[sp+4]+96]` (`call` at 0x11983f), followed by a four-byte
  `0xffffffff` assignment at `[sp+4] +0x60` (`mov` at 0x119847). The branch falls through when `(? & 0x20) != 0x0` (`je` at 0x119853), reaching
  `_timeoutWrite` with candidates `[[sp+4]+100]`, `0x1197cd`, `0x2`, `[0x1da124]` (`call` at 0x119866). Later
  assignments zero `[sp+4] +0x18`, `+0x1c`, `+0x20`, `+0x24`, `+0x28` (`mov` at 0x11987d, `mov` at
  0x119884, `mov` at 0x11988b, `mov` at 0x119892, `mov` at 0x119899). State semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1197fc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1197ff | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x11980b | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x119817 | load | 4 | `[sp+4] +0x6c` | `-` |
| `mov` at 0x119823 | load | 4 | `[sp+4] +0x70` | `-` |
| `mov` at 0x11982f | load | 4 | `[sp+4] +0x74` | `-` |
| `mov` at 0x11983b | load | 4 | `[sp+4] +0x60` | `-` |
| `mov` at 0x119847 | store | 4 | `[sp+4] +0x60` | `0xffffffff` |
| `mov` at 0x11984e | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x119855 | load | 4 | `abs 0x1da124` | `-` |
| `mov` at 0x119862 | load | 4 | `[sp+4] +0x64` | `-` |
| `mov` at 0x11986e | store | 4 | `[sp+4] +0x44` | `0x0` |
| `mov` at 0x119875 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11987a | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x11987d | store | 4 | `[sp+4] +0x18` | `0x0` |
| `mov` at 0x119884 | store | 4 | `[sp+4] +0x1c` | `0x0` |
| `mov` at 0x11988b | store | 4 | `[sp+4] +0x20` | `0x0` |
| `mov` at 0x119892 | store | 4 | `[sp+4] +0x24` | `0x0` |
| `mov` at 0x119899 | store | 4 | `[sp+4] +0x28` | `0x0` |
| `test` at 0x1198a0 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x1198b6 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x119803 | `_bufHold 0x10f678` | `[[sp+4]+104]` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x11980f | `_bufDetachFD 0x10f478` | `[[sp+4]+104]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11981b | `_bufDetachFD 0x10f478` | `[[sp+4]+108]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x119827 | `_bufDetachFD 0x10f478` | `[[sp+4]+112]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x119833 | `_bufDetachFD 0x10f478` | `[[sp+4]+116]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11983f | `_close 0x186ed0` | `[[sp+4]+96]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x119866 | `_timeoutWrite 0x12a788` | `[[sp+4]+100]` | `0x1197cd` | `0x2` | `[0x1da124]` | `?` | `?` |
| `call` at 0x1198af | `_ffscMsg 0x11ebd0` | `0x1197d0` | `[sp+4]` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x119853 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `je` at 0x1198a7 | `(([0x1da698]:1 & 0xff) & 0x4) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1198bc | `eax` | `0x0` |

## `_ceEnterWatchSysMode`

- `_ceEnterWatchSysMode`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x118ff8 to 0x119108.
  Established. The branch takes the main assignment arm when `[0x1d9b9c] == 0x0` (`je` at 0x11901d).
  Four-byte stores write `0x9` at `[sp+4] +0x18`, `0x1` at `[sp+4] +0x1c`, and `0x1` at absolute
  `0x1ebc4c` (`mov` at 0x119048, `mov` at 0x11904f, `mov` at 0x1190aa). The `_cmCheckTTYFlags`
  candidate is `[sp+4]` (`call` at 0x119077). A later store at `[[sp+4]+132] +0x14` has an unknown
  value (`mov` at 0x1190a7). Watch-mode behavior is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x118ffc | load | 4 | `sp +0x4` | `-` |
| `test` at 0x118fff | load | 1 | `abs 0x1da698` | `-` |
| `cmp` at 0x119016 | load | 4 | `abs 0x1d9b9c` | `-` |
| `cmp` at 0x11901f | load | 4 | `abs 0x1ebc00` | `-` |
| `mov` at 0x119028 | load | 4 | `abs 0x1d9b9c` | `-` |
| `mov` at 0x119048 | store | 4 | `[sp+4] +0x18` | `0x9` |
| `mov` at 0x11904f | store | 4 | `[sp+4] +0x1c` | `0x1` |
| `mov` at 0x119056 | load | 4 | `[sp+4] +0x4c` | `-` |
| `mov` at 0x11905a | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x119066 | load | 4 | `[sp+4] +0x4c` | `-` |
| `mov` at 0x11906a | load | 4 | `[sp+4] +0x6c` | `-` |
| `cmp` at 0x11907f | load | 4 | `abs 0x1ebc00` | `-` |
| `mov` at 0x119088 | store | 4 | `abs 0x1d9b9c` | `[sp+4]` |
| `mov` at 0x11908e | load | 4 | `[sp+4] +0x84` | `-` |
| `cmp` at 0x119094 | load | 4 | `abs 0x1da968` | `-` |
| `mov` at 0x11909c | load | 4 | `[sp+4] +0x84` | `-` |
| `mov` at 0x1190a2 | load | 4 | `[[sp+4]+132] +0x14` | `-` |
| `mov` at 0x1190a7 | store | 4 | `[[sp+4]+132] +0x14` | `?` |
| `mov` at 0x1190aa | store | 4 | `abs 0x1ebc4c` | `0x1` |
| `mov` at 0x1190b4 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x1190bc | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x1190c2 | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x1190c5 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x1190cd | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x1190d3 | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x1190d6 | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x1190e1 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x11900e | `_ffscMsg 0x11ebd0` | `0x118f95` | `[sp+4]` | `ebx` | `ebp` | `?` | `?` |
| `call` at 0x119033 | `_ffscMsg 0x11ebd0` | `0x118fb7` | `[0x1d9b9c]` | `?` | `?` | `?` | `?` |
| `call` at 0x11903c | `_ceEnterIdleMode 0x118ee8` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11905e | `_bufAttachFD 0x10f378` | `[[sp+4]+104]` | `[[sp+4]+76]` | `?` | `?` | `?` | `?` |
| `call` at 0x11906e | `_bufAttachFD 0x10f378` | `[[sp+4]+108]` | `[[sp+4]+76]` | `?` | `?` | `?` | `?` |
| `call` at 0x119077 | `_cmCheckTTYFlags 0x119c20` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1190da | `_bufRelease 0x10fa18` | `[[sp+4]+104]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x119006 | `(([0x1da698]:1 & 0xff) & 0x4) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x4) != 0x0` |
| `je` at 0x11901d | `[0x1d9b9c] == 0x0` | `[0x1d9b9c] != 0x0` |
| `jge` at 0x119026 | `[0x1ebc00] >= 0x0 (signed, 32-bit)` | `[0x1ebc00] < 0x0 (signed, 32-bit)` |
| `jge` at 0x119086 | `[0x1ebc00] >= 0x0 (signed, 32-bit)` | `[0x1ebc00] < 0x0 (signed, 32-bit)` |
| `jne` at 0x11909a | `[0x1da968] != [[sp+4]+132]` | `[0x1da968] == [[sp+4]+132]` |
| `je` at 0x1190ba | `(? & 0x40) == 0x0` | `(? & 0x40) != 0x0` |
| `jge` at 0x1190cb | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1190e7 | `eax` | `?` |

## `_clLeaveWatchSysMode`

- `_clLeaveWatchSysMode`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x119a28 to 0x119b08.
  Established. The visible call sequence includes `_bufHold` with candidate zero `[[sp+4]+104]`
  (`call` at 0x119a33), and `_bufDetachFD` with `[[sp+4]+104]` and `[[sp+4]+108]` at distinct sites
  (`call` at 0x119a71, `call` at 0x119a7d). Four-byte stores write `0x1` to absolute `0x1ebc4c` and
  zero to `0x1d9b9c` (`mov` at 0x119ab6, `mov` at 0x119ac0). The return is zero (`ret` at 0x119ae6).
  These caller effects do not settle the mode role.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x119a2c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x119a2f | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x119a3b | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x119a42 | load | 4 | `[sp+4] +0x70` | `-` |
| `mov` at 0x119a4e | load | 4 | `[sp+4] +0x6c` | `-` |
| `mov` at 0x119a5a | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x119a5f | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x119a62 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x119a6a | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x119a6d | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x119a79 | load | 4 | `[sp+4] +0x6c` | `-` |
| `mov` at 0x119a85 | store | 4 | `[sp+4] +0x18` | `0x0` |
| `mov` at 0x119a8c | store | 4 | `[sp+4] +0x1c` | `0x0` |
| `mov` at 0x119a93 | store | 4 | `[sp+4] +0x20` | `0x0` |
| `mov` at 0x119a9a | load | 4 | `[sp+4] +0x84` | `-` |
| `cmp` at 0x119aa0 | load | 4 | `abs 0x1da968` | `-` |
| `mov` at 0x119aa8 | load | 4 | `[sp+4] +0x84` | `-` |
| `mov` at 0x119aae | load | 4 | `[[sp+4]+132] +0x14` | `-` |
| `mov` at 0x119ab3 | store | 4 | `[[sp+4]+132] +0x14` | `?` |
| `mov` at 0x119ab6 | store | 4 | `abs 0x1ebc4c` | `0x1` |
| `mov` at 0x119ac0 | store | 4 | `abs 0x1d9b9c` | `0x0` |
| `test` at 0x119aca | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x119ae0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x119a33 | `_bufHold 0x10f678` | `[[sp+4]+104]` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x119a46 | `_bufRelease 0x10fa18` | `[[sp+4]+112]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x119a52 | `_bufRelease 0x10fa18` | `[[sp+4]+108]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x119a71 | `_bufDetachFD 0x10f478` | `[[sp+4]+104]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x119a7d | `_bufDetachFD 0x10f478` | `[[sp+4]+108]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x119ad9 | `_ffscMsg 0x11ebd0` | `0x119a07` | `[sp+4]` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x119a40 | `(? & 0x8) == 0x0` | `(? & 0x8) != 0x0` |
| `jne` at 0x119aa6 | `[0x1da968] != [[sp+4]+132]` | `[0x1da968] == [[sp+4]+132]` |
| `je` at 0x119ad1 | `(([0x1da698]:1 & 0xff) & 0x4) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x119ae6 | `eax` | `0x0` |

## `_crRouterIn`

- `_crRouterIn`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x11bb70 to 0x11bc80.
  Established. `_timeoutRead` receives candidates `[[sp+4]+92]`, `sp-248`, `0xf0`, `[0x1da118]`
  (`call` at 0x11bb95). A zero `_strrchr` result takes the branch past its one-byte zero store (`je`
  at 0x11bbdd, `mov` at 0x11bbdf). Another byte zero is stored at `(ret(_timeoutRead) + sp-4) -0xf4`
  (`mov` at 0x11bbe2). `_crProcessCmd` has candidates `[sp+4]`, `sp-248` (`call` at 0x11bc04).
  Message extent and routing semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x11bb7c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x11bb7f | load | 4 | `abs 0x1da118` | `-` |
| `mov` at 0x11bb91 | load | 4 | `[sp+4] +0x5c` | `-` |
| `mov` at 0x11bbdf | store | 1 | `ret(_strrchr) +0x0` | `0x0` |
| `mov` at 0x11bbe2 | store | 1 | `(ret(_timeoutRead) + sp-4) -0xf4` | `0x0` |
| `test` at 0x11bbea | load | 1 | `abs 0x1da698` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x11bb95 | `_timeoutRead 0x12a538` | `[[sp+4]+92]` | `sp-248` | `0xf0` | `[0x1da118]` | `ebx` | `esi` |
| `call` at 0x11bba9 | `_ffscMsgS 0x11eca0` | `0x11bb10` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x11bbd3 | `_strrchr 0x17e240` | `sp-248` | `0xd` | `?` | `?` | `?` | `?` |
| `call` at 0x11bbfa | `_ffscMsg 0x11ebd0` | `0x11bb39` | `[sp+4]` | `sp-248` | `?` | `?` | `?` |
| `call` at 0x11bc04 | `_crProcessCmd 0x11bc80` | `[sp+4]` | `sp-248` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x11bba1 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x11bbc2 | `(ret(_timeoutRead) & ret(_timeoutRead)) != 0x0` | `(ret(_timeoutRead) & ret(_timeoutRead)) == 0x0` |
| `je` at 0x11bbdd | `(ret(_strrchr) & ret(_strrchr)) == 0x0` | `(ret(_strrchr) & ret(_strrchr)) != 0x0` |
| `je` at 0x11bbf1 | `(([0x1da698]:1 & 0xff) & 0x4) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x11bc15 | `eax` | `?` |

## `_crProcessCmd`

- `_crProcessCmd`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x11bc80 to 0x11bd90.
  Established. `_strtok_r` has leading candidates `[sp+8]`, `0x11bc16`, `sp-8` (`call` at 0x11bc9f).
  The dispatcher includes calls to `_crProcessMSGCmd`, `_crProcessSHUTDOWNCmd`,
  `_crProcessSTATECmd`, `_crProcessSYSCmd`, each with leading candidates `[sp+4]`, `[sp-8]` (`call`
  at 0x11bcd7, `call` at 0x11bcf7, `call` at 0x11bd17, `call` at 0x11bd37). Each intervening
  comparison takes the next-test arm on a nonzero `_strcmp` self-test (`jne` at 0x11bcd0, `jne` at
  0x11bcf0, `jne` at 0x11bd10, `jne` at 0x11bd30). The common exit reports zero (`ret` at 0x11bd56).
  Command effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x11bc88 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x11bc8b | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x11bc8e | store | 4 | `sp -0x8` | `0x0` |
| `mov` at 0x11bcd2 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x11bcf2 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x11bd12 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x11bd32 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x11bc9f | `_strtok_r 0x17e358` | `[sp+8]` | `0x11bc16` | `sp-8` | `ebx` | `esi` | `0x0` |
| `call` at 0x11bcb3 | `_ffscMsg 0x11ebd0` | `0x11bc18` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x11bcc6 | `_strcmp 0x17de80` | `ret(_strtok_r)` | `0x11bc38` | `?` | `?` | `?` | `?` |
| `call` at 0x11bcd7 | `_crProcessMSGCmd 0x11bd90` | `[sp+4]` | `[sp-8]` | `?` | `?` | `?` | `?` |
| `call` at 0x11bce6 | `_strcmp 0x17de80` | `ret(_strtok_r)` | `0x11bc3c` | `?` | `?` | `?` | `?` |
| `call` at 0x11bcf7 | `_crProcessSHUTDOWNCmd 0x11bfa0` | `[sp+4]` | `[sp-8]` | `?` | `?` | `?` | `?` |
| `call` at 0x11bd06 | `_strcmp 0x17de80` | `ret(_strtok_r)` | `0x11bc45` | `?` | `?` | `?` | `?` |
| `call` at 0x11bd17 | `_crProcessSTATECmd 0x11c2c0` | `[sp+4]` | `[sp-8]` | `?` | `?` | `?` | `?` |
| `call` at 0x11bd26 | `_strcmp 0x17de80` | `ret(_strtok_r)` | `0x11bc4b` | `?` | `?` | `?` | `?` |
| `call` at 0x11bd37 | `_crProcessSYSCmd 0x11cc60` | `[sp+4]` | `[sp-8]` | `?` | `?` | `?` | `?` |
| `call` at 0x11bd47 | `_ffscMsg 0x11ebd0` | `0x11bc4f` | `[sp+4]` | `ret(_strtok_r)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x11bcab | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `jne` at 0x11bcd0 | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |
| `jne` at 0x11bcf0 | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |
| `jne` at 0x11bd10 | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |
| `jne` at 0x11bd30 | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x11bd56 | `eax` | `0x0` |

## `_crProcessSYSCmd`

- `_crProcessSYSCmd`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x11cc60 to 0x11cde0.
  Established. The byte comparison branch takes the default destination arm when the printed byte
  value is not `0x40` (`jne` at 0x11cc8d). That arm stores `[sp+4]` at `sp -0x100` (`mov` at
  0x11cd33). The final `_bufWrite` candidates are `[[sp-256]+108]`, `sp-248`, `ret(_strlen)` (`call`
  at 0x11cd81). The indexed destination loads remain unresolved (`mov` at 0x11ccd0, `mov` at
  0x11ccdb, `mov` at 0x11cd20). The destination catalogue and command semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x11cc6c | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x11cc6f | store | 4 | `sp -0x100` | `0x0` |
| `mov` at 0x11cc89 | load | 1 | `([sp+8] + ret(_strspn)) +0x0` | `-` |
| `mov` at 0x11cc93 | store | 4 | `sp -0xfc` | `([sp+8] + ret(_strspn))` |
| `mov` at 0x11cca9 | load | 1 | `(([sp+8] + ret(_strspn)) + ret(_strcspn)) +0x0` | `-` |
| `mov` at 0x11ccaf | store | 1 | `(([sp+8] + ret(_strspn)) + ret(_strcspn)) +0x0` | `0x0` |
| `mov` at 0x11ccc9 | load | 4 | `sp -0xfc` | `-` |
| `mov` at 0x11ccd0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11ccdb | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x11ccf6 | load | 4 | `sp -0x100` | `-` |
| `mov` at 0x11ccff | load | 4 | `sp -0xfc` | `-` |
| `mov` at 0x11cd20 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11cd27 | store | 4 | `sp -0x100` | `?` |
| `mov` at 0x11cd30 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x11cd33 | store | 4 | `sp -0x100` | `[sp+4]` |
| `test` at 0x11cd4e | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x11cd58 | load | 4 | `sp -0x100` | `-` |
| `mov` at 0x11cd77 | load | 4 | `sp -0x100` | `-` |
| `mov` at 0x11cd7d | load | 4 | `[sp-256] +0x6c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x11cc7f | `_strspn 0x17e264` | `[sp+8]` | `0x11bc16` | `ebx` | `esi` | `edi` | `0x0` |
| `call` at 0x11cc9f | `_strcspn 0x17e008` | `([sp+8] + ret(_strspn))` | `0x11bc16` | `?` | `?` | `?` | `?` |
| `call` at 0x11ccbb | `_strspn 0x17e264` | `(([sp+8] + ret(_strspn)) + ret(_strcspn))+1` | `0x11bc16` | `?` | `?` | `?` | `?` |
| `call` at 0x11cce4 | `_strcmp 0x17de80` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11cd0b | `_ffscMsg 0x11ebd0` | `0x11bd57` | `[sp-252]` | `?` | `?` | `?` | `?` |
| `call` at 0x11cd46 | `_sprintf 0x171680` | `sp-248` | `0x11cc34` | `?` | `?` | `?` | `?` |
| `call` at 0x11cd64 | `_ffscMsg 0x11ebd0` | `0x11cc38` | `[sp-256]` | `?` | `?` | `?` | `?` |
| `call` at 0x11cd6d | `_strlen 0x172f14` | `sp-248` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11cd81 | `_bufWrite 0x10fb78` | `[[sp-256]+108]` | `sp-248` | `ret(_strlen)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x11cc8d | `([([sp+8] + ret(_strspn))]:1 & 0xff) != 0x40` | `([([sp+8] + ret(_strspn))]:1 & 0xff) == 0x40` |
| `je` at 0x11ccad | `(([(([sp+8] + ret(_strspn)) + ret(_strcspn))]:1 & 0xff) & ([(([sp+8] + ret(_strspn)) + ret(_strcspn))]:1 & 0xff)) == 0x0` | `(([(([sp+8] + ret(_strspn)) + ret(_strcspn))]:1 & 0xff) & ([(([sp+8] + ret(_strspn)) + ret(_strcspn))]:1 & 0xff)) != 0x0` |
| `je` at 0x11ccd9 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x11ccee | `(ret(_strcmp) & ret(_strcmp)) == 0x0` | `(ret(_strcmp) & ret(_strcmp)) != 0x0` |
| `jle` at 0x11ccf4 | `? <= 0x3 (signed, 32-bit)` | `? > 0x3 (signed, 32-bit)` |
| `jne` at 0x11ccfd | `[sp-256] != 0x0` | `[sp-256] == 0x0` |
| `je` at 0x11cd55 | `(([0x1da698]:1 & 0xff) & 0x4) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x11cd92 | `eax` | `?` |

## `_cmCheckTTYFlags`

- `_cmCheckTTYFlags`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x119c20 to 0x119dd0.
  Established. The entry saves `[[sp+4]+60]` at `sp -0x8` and `0x1da0e0` at `sp -0xc` (`mov` at
  0x119c2f, `mov` at 0x119c36). Four stores at `[sp+4] +0x3c` retain unknown merged values (`mov` at
  0x119c78, `mov` at 0x119ca8, `mov` at 0x119cd8, `mov` at 0x119d08). The loop branch is taken when
  `[sp-12] <= 0x1da0f4 (signed, 32-bit)` (`jle` at 0x119d1a). The returned value remains unknown
  (`ret` at 0x119d37). Flag meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x119c29 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x119c2c | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x119c2f | store | 4 | `sp -0x8` | `[[sp+4]+60]` |
| `mov` at 0x119c36 | store | 4 | `sp -0xc` | `0x1da0e0` |
| `mov` at 0x119c40 | load | 4 | `[sp+4] +0x70` | `-` |
| `mov` at 0x119c43 | load | 4 | `[[sp+4]+112] +0x0` | `-` |
| `mov` at 0x119c45 | load | 4 | `(unknown)` | `-` |
| `test` at 0x119c50 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x119c59 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x119c70 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x119c78 | store | 4 | `[sp+4] +0x3c` | `?` |
| `test` at 0x119c7b | load | 1 | `(unknown)` | `-` |
| `test` at 0x119c84 | load | 1 | `abs 0x1da69b` | `-` |
| `mov` at 0x119c8d | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x119ca0 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x119ca8 | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x119cab | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x119cae | load | 4 | `[[sp+4]+104] +0x0` | `-` |
| `mov` at 0x119cb0 | load | 4 | `(unknown)` | `-` |
| `test` at 0x119cbb | load | 1 | `(unknown)` | `-` |
| `mov` at 0x119cc4 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x119cd0 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x119cd8 | store | 4 | `[sp+4] +0x3c` | `?` |
| `test` at 0x119cdb | load | 1 | `(unknown)` | `-` |
| `test` at 0x119ce4 | load | 1 | `abs 0x1da69b` | `-` |
| `mov` at 0x119ced | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x119d00 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x119d08 | store | 4 | `[sp+4] +0x3c` | `?` |
| `add` at 0x119d0e | load | 4 | `sp -0xc` | `-` |
| `add` at 0x119d0e | store | 4 | `sp -0xc` | `[sp-12]+4` |
| `cmp` at 0x119d13 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x119d20 | load | 4 | `[sp+4] +0x3c` | `-` |
| `cmp` at 0x119d23 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x119c4e | `Z == 0` | `not (Z == 0)` |
| `je` at 0x119c57 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x119c82 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x119c8b | `(([0x1da69b]:1 & 0xff) & 0x10) != 0x0` | `(([0x1da69b]:1 & 0xff) & 0x10) == 0x0` |
| `jne` at 0x119cb9 | `Z == 0` | `not (Z == 0)` |
| `je` at 0x119cc2 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x119ce2 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x119ceb | `(([0x1da69b]:1 & 0xff) & 0x20) != 0x0` | `(([0x1da69b]:1 & 0xff) & 0x20) == 0x0` |
| `jle` at 0x119d1a | `[sp-12] <= 0x1da0f4 (signed, 32-bit)` | `[sp-12] > 0x1da0f4 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x119d37 | `eax` | `?` |

## `_cmdChangeCtrlChar`

- `_cmdChangeCtrlChar`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x111440 to 0x111520.
  Established. `_cmdParseCharSpecification` receives candidate zero `[sp+4]` (`call` at 0x11144f). A
  later branch takes the alternate formatting arm when `[[sp+8]+16] == 0x2` (`je` at 0x1114a6). That
  arm calls `_sprintf` with candidate one `0x111426`, candidate two `[sp+12]`, and unknown
  candidate zero and three (`call` at 0x1114ea). The other `_sprintf` sites also have unknown
  candidate zero (`call` at 0x11148a, `call` at 0x1114bd). Character-setting behavior is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x111445 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x111448 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x11144b | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x11147a | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x11147f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x111486 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1114a0 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x1114ad | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1114b2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1114b9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1114da | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1114df | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1114e6 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x11144f | `_cmdParseCharSpecification 0x111640` | `[sp+4]` | `ebx` | `esi` | `ebp` | `?` | `?` |
| `call` at 0x11148a | `_sprintf 0x171680` | `?` | `0x111411` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x1114bd | `_sprintf 0x171680` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1114ea | `_sprintf 0x171680` | `?` | `0x111426` | `[sp+12]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x11145b | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x111472 | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `je` at 0x1114a6 | `[[sp+8]+16] == 0x2` | `[[sp+8]+16] != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1114f7 | `eax` | `ret(_sprintf)` |

## `_cmdParseCharSpecification`

- `_cmdParseCharSpecification`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x111640 to 0x111770.
  Established. The `_strtok_r` candidate prefix is `0x0`, `0x1115c8`, `[sp+4]` (`call` at 0x111652).
  The branch takes the numeric-conversion arm when the printed first byte is not `0x5e` (`jne` at
  0x1116b4). `_strtol` then receives `ret(_strtok_r)`, `sp-8`, `0x0` (`call` at 0x111727). A later
  unsigned branch is taken when `ret(_strtol)-1 <= 0xfe` (`jbe` at 0x111742). The merged return
  remains unknown (`ret` at 0x111768). The accepted character catalogue is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x111647 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x111664 | load | 1 | `ret(_strtok_r) +0x0` | `-` |
| `mov` at 0x111690 | load | 1 | `ret(_strtok_r) +0x1` | `-` |
| `mov` at 0x111697 | load | 1 | `ret(_strtok_r) +0x0` | `-` |
| `mov` at 0x1116b0 | load | 1 | `ret(_strtok_r) +0x0` | `-` |
| `mov` at 0x1116b6 | load | 1 | `ret(_strtok_r) +0x1` | `-` |
| `mov` at 0x1116bd | load | 1 | `ret(_strtok_r) +0x2` | `-` |
| `mov` at 0x1116d0 | load | 1 | `ret(_strtok_r) +0x1` | `-` |
| `mov` at 0x1116d7 | load | 1 | `ret(_strtok_r) +0x1` | `-` |
| `mov` at 0x1116de | load | 1 | `ret(_strtok_r) +0x1` | `-` |
| `mov` at 0x1116f0 | load | 1 | `ret(_strtok_r) +0x1` | `-` |
| `mov` at 0x111731 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x111734 | load | 1 | `[sp-8] +0x0` | `-` |
| `mov` at 0x111762 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x111652 | `_strtok_r 0x17e358` | `0x0` | `0x1115c8` | `[sp+4]` | `ebx` | `?` | `ebp` |
| `call` at 0x111674 | `_strcmp 0x17de80` | `ret(_strtok_r)` | `0x10fed6` | `?` | `?` | `?` | `?` |
| `call` at 0x111727 | `_strtol 0x17d9f8` | `ret(_strtok_r)` | `sp-8` | `0x0` | `?` | `?` | `?` |
| `call` at 0x111749 | `_ffscMsg 0x11ebd0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x11165e | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` |
| `je` at 0x111668 | `(([ret(_strtok_r)]:1 & 0xff) & ([ret(_strtok_r)]:1 & 0xff)) == 0x0` | `(([ret(_strtok_r)]:1 & 0xff) & ([ret(_strtok_r)]:1 & 0xff)) != 0x0` |
| `jne` at 0x11167e | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |
| `jne` at 0x111695 | `(([ret(_strtok_r)+1]:1 & 0xff) & ([ret(_strtok_r)+1]:1 & 0xff)) != 0x0` | `(([ret(_strtok_r)+1]:1 & 0xff) & ([ret(_strtok_r)+1]:1 & 0xff)) == 0x0` |
| `jne` at 0x1116b4 | `([ret(_strtok_r)]:1 & 0xff) != 0x5e` | `([ret(_strtok_r)]:1 & 0xff) == 0x5e` |
| `je` at 0x1116bb | `(([ret(_strtok_r)+1]:1 & 0xff) & ([ret(_strtok_r)+1]:1 & 0xff)) == 0x0` | `(([ret(_strtok_r)+1]:1 & 0xff) & ([ret(_strtok_r)+1]:1 & 0xff)) != 0x0` |
| `je` at 0x1116c2 | `(([ret(_strtok_r)+2]:1 & 0xff) & ([ret(_strtok_r)+2]:1 & 0xff)) == 0x0` | `(([ret(_strtok_r)+2]:1 & 0xff) & ([ret(_strtok_r)+2]:1 & 0xff)) != 0x0` |
| `jle` at 0x1116d5 | `([ret(_strtok_r)+1]:1 & 0xff) <= 0x60 (signed, 8-bit)` | `([ret(_strtok_r)+1]:1 & 0xff) > 0x60 (signed, 8-bit)` |
| `jg` at 0x1116dc | `([ret(_strtok_r)+1]:1 & 0xff) > 0x7a (signed, 8-bit)` | `([ret(_strtok_r)+1]:1 & 0xff) <= 0x7a (signed, 8-bit)` |
| `jbe` at 0x1116fc | `? <= 0x1f (unsigned, 32-bit)` | `? > 0x1f (unsigned, 32-bit)` |
| `jne` at 0x111738 | `(([[sp-8]]:1 & 0xff) & ([[sp-8]]:1 & 0xff)) != 0x0` | `(([[sp-8]]:1 & 0xff) & ([[sp-8]]:1 & 0xff)) == 0x0` |
| `jbe` at 0x111742 | `ret(_strtol)-1 <= 0xfe (unsigned, 32-bit)` | `ret(_strtol)-1 > 0xfe (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x111768 | `eax` | `?` |
