# MMSC message, timeout and identity callers in the 6.4-origin image

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

[Fact sheets and complete listings](disasm/messages-timeouts-64-origin-facts.txt) include all tool sections.

## `_msgClear`

- `_msgClear`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x125ea0 to 0x125ed0.
  Established. A zero first stack value takes the branch past the explicit updates (`je` at
  0x125ea8). The other arm copies `[sp+4] +0x8` into `[sp+4] +0xc` (`mov` at 0x125eaa, `mov` at
  0x125ead) and writes zero at `[sp+4] +0x1c` (`mov` at 0x125eb0); its `_bzero` call has candidate
  zero `[[sp+4]+8]`, while the extent candidate is unknown (`call` at 0x125ec0). The object layout
  role is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x125ea3 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x125eaa | load | 4 | `[sp+4] +0x8` | `-` |
| `mov` at 0x125ead | store | 4 | `[sp+4] +0xc` | `[[sp+4]+8]` |
| `mov` at 0x125eb0 | store | 4 | `[sp+4] +0x1c` | `0x0` |
| `mov` at 0x125eb7 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x125ebc | load | 4 | `[sp+4] +0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x125ec0 | `_bzero 0x170f20` | `[[sp+4]+8]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x125ea8 | `([sp+4] & [sp+4]) == 0x0` | `([sp+4] & [sp+4]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x125ec8 | `eax` | `?` |

## `_msgFree`

- `_msgFree`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x125ed0 to 0x125f50.
  Established. The zero-value arm skips both calls (`je` at 0x125ed9). On the other arm, `_free`
  receives candidate zero `[[sp+4]+8]` (`call` at 0x125edf), then `[sp+4]` (`call` at 0x125ee8). No
  explicit memory stores appear in the access section. Ownership beyond those candidate values is
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x125ed4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x125edb | load | 4 | `[sp+4] +0x8` | `-` |
| `mov` at 0x125eed | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x125edf | `_free 0x17c898` | `[[sp+4]+8]` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x125ee8 | `_free 0x17c898` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x125ed9 | `([sp+4] & [sp+4]) == 0x0` | `([sp+4] & [sp+4]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x125ef3 | `eax` | `?` |

## `_msgNew`

- `_msgNew`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x125f50 to 0x126160.
  Established. Two `_malloc` calls carry candidate zero values `0x20` and `[sp+4]+1` (`call` at
  0x125f5e, `call` at 0x125f84). Stores include `[sp+4]` at `ret(_malloc) +0x10`, `[sp+8]` at
  `ret(_malloc) +0x14`, and `[sp+12]` at `ret(_malloc) +0x18` (`mov` at 0x125f96, `mov` at 0x125f99,
  `mov` at 0x125f9f). `_strncpy` has candidates `ret(_malloc)`, `[sp+16]`, `0x8` (`call` at
  0x125fa9). Distinct allocation identities are not resolved by the repeated return label. Role:
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x125f56 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x125f59 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x125f8c | store | 4 | `ret(_malloc) +0x8` | `ret(_malloc)` |
| `mov` at 0x125f8f | load | 4 | `ret(_malloc) +0x8` | `-` |
| `mov` at 0x125f96 | store | 4 | `ret(_malloc) +0x10` | `[sp+4]` |
| `mov` at 0x125f99 | store | 4 | `ret(_malloc) +0x14` | `[sp+8]` |
| `mov` at 0x125f9c | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x125f9f | store | 4 | `ret(_malloc) +0x18` | `[sp+12]` |
| `mov` at 0x125fa4 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x125fb1 | store | 1 | `ret(_malloc) +0x7` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x125f5e | `_malloc 0x17c878` | `0x20` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x125f71 | `_ffscMsg 0x11ebd0` | `0x125ef4` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x125f84 | `_malloc 0x17c878` | `[sp+4]+1` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x125fa9 | `_strncpy 0x17e1b0` | `ret(_malloc)` | `[sp+16]` | `0x8` | `?` | `?` | `?` |
| `call` at 0x125fb6 | `_msgClear 0x125ea0` | `ret(_malloc)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x125fc5 | `_ffscMsg 0x11ebd0` | `0x125f19` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x125fce | `_free 0x17c898` | `ret(_malloc)` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x125f6a | `(ret(_malloc) & ret(_malloc)) != 0x0` | `(ret(_malloc) & ret(_malloc)) == 0x0` |
| `je` at 0x125f94 | `([ret(_malloc)+8] & [ret(_malloc)+8]) == 0x0` | `([ret(_malloc)+8] & [ret(_malloc)+8]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x125fde | `eax` | `?` |

## `_msgRead`

- `_msgRead`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x126160 to 0x126680.
  Established. Role: Unknown. The visible `_read` call carries candidates
  `[[sp+4]+20]`, `[[sp+4]+12]`, `[sp-96]` (`call` at 0x126293). A one-byte zero is stored at
  `([sp-104] + [[sp+4]+12]) +0x0` (`mov` at 0x12633a). The unsigned branch takes the diagnostic arm
  when `[[sp+4]+28]-2 > 0x16` (`ja` at 0x12634a). Otherwise a computed jump remains unresolved
  (`jmp` at 0x126350). Effects reached only through that selector are excluded, so the full state
  transition behavior is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x126169 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12616c | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x126173 | load | 4 | `abs 0x1ee88c` | `-` |
| `mov` at 0x126188 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12618b | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x12618f | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x126193 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x1261a4 | store | 4 | `sp -0x64` | `0x0` |
| `mov` at 0x1261b0 | store | 4 | `sp -0x64` | `0x0` |
| `mov` at 0x1261bc | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1261e0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1261e3 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x1261e6 | load | 4 | `[sp+4] +0x8` | `-` |
| `mov` at 0x1261eb | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x1261f4 | store | 4 | `sp -0x5c` | `0x0` |
| `mov` at 0x1261fb | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x126203 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x12620b | store | 4 | `sp -0x60` | `?` |
| `mov` at 0x12622d | store | 4 | `sp -0x64` | `0xffffffff` |
| `mov` at 0x126240 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x126243 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x126254 | store | 4 | `sp -0x64` | `0xffffffff` |
| `mov` at 0x126260 | store | 4 | `sp -0x60` | `0x1` |
| `mov` at 0x126270 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x126273 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x126284 | load | 4 | `sp -0x60` | `-` |
| `mov` at 0x126288 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12628b | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x12628f | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x12629b | store | 4 | `sp -0x68` | `ret(_read)` |
| `cmp` at 0x1262a3 | load | 4 | `sp -0x68` | `-` |
| `movzx` at 0x1262af | load | 2 | `abs 0x1da69e` | `-` |
| `mov` at 0x1262b6 | load | 4 | `[sp+4] +0x14` | `-` |
| `cmp` at 0x1262bd | load | 4 | `sp -0x60` | `-` |
| `mov` at 0x1262c3 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x1262c6 | load | 1 | `[[sp+4]+12] +0x0` | `-` |
| `mov` at 0x1262cc | load | 1 | `[[sp+4]+12] +0x0` | `-` |
| `mov` at 0x1262d1 | load | 4 | `abs 0x1d1130` | `-` |
| `mov` at 0x1262d6 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x1262dd | load | 1 | `[[sp+4]+12] +0x0` | `-` |
| `mov` at 0x1262f6 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1262f9 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x126310 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x126313 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x126317 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x126331 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x126334 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x126337 | load | 4 | `sp -0x68` | `-` |
| `mov` at 0x12633a | store | 1 | `([sp-104] + [[sp+4]+12]) +0x0` | `0x0` |
| `mov` at 0x12633e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x126341 | load | 4 | `[sp+4] +0x1c` | `-` |
| `jmp` at 0x126350 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x126590 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x126593 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x126607 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12660a | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x12661b | load | 4 | `sp -0x64` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12617c | `_setjmp 0x18fe80` | `[0x1ee88c]+4` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x12619c | `_ffscMsg 0x11ebd0` | `0x125fdf` | `[[sp+4]+20]` | `[[sp+4]+24]` | `[[sp+4]+28]` | `?` | `?` |
| `call` at 0x1261c0 | `_timeoutSet 0x12a658` | `[sp+8]` | `0x12600f` | `?` | `?` | `?` | `?` |
| `call` at 0x126225 | `_ffscMsgS 0x11eca0` | `0x12601f` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12624c | `_ffscMsg 0x11ebd0` | `0x126035` | `[[sp+4]+20]` | `?` | `?` | `?` | `?` |
| `call` at 0x12627f | `_timeoutEnable 0x12a348` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126293 | `_read 0x186f50` | `[[sp+4]+20]` | `[[sp+4]+12]` | `[sp-96]` | `?` | `?` | `?` |
| `call` at 0x12629e | `_timeoutDisable 0x12a328` | `?` | `?` | `?` | `ret(_read)` | `?` | `?` |
| `call` at 0x126302 | `_ffscMsg 0x11ebd0` | `0x126042` | `[[sp+4]+20]` | `?` | `?` | `?` | `?` |
| `call` at 0x12631b | `_strlen 0x172f14` | `[[sp+4]+12]` | `[[sp+4]+12]` | `?` | `?` | `?` | `?` |
| `call` at 0x126329 | `_ffscMsg 0x11ebd0` | `0x126064` | `ret(_strlen)` | `?` | `?` | `?` | `?` |
| `call` at 0x12659c | `_ffscMsg 0x11ebd0` | `0x126103` | `[[sp+4]+28]` | `?` | `?` | `?` | `?` |
| `call` at 0x126616 | `_timeoutCancel 0x12a308` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jl` at 0x126171 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x126186 | `(ret(_setjmp) & ret(_setjmp)) == 0x0` | `(ret(_setjmp) & ret(_setjmp)) != 0x0` |
| `je` at 0x1261ca | `(ret(_timeoutSet) & ret(_timeoutSet)) == 0x0` | `(ret(_timeoutSet) & ret(_timeoutSet)) != 0x0` |
| `je` at 0x126201 | `[[sp+4]+28] == 0x2` | `[[sp+4]+28] != 0x2` |
| `jne` at 0x126209 | `[[sp+4]+28] != 0x6` | `[[sp+4]+28] == 0x6` |
| `je` at 0x126279 | `[[sp+4]+28] == 0xa` | `[[sp+4]+28] != 0xa` |
| `jl` at 0x1262a7 | `[sp-104] < 0x0 (signed, 32-bit)` | `[sp-104] >= 0x0 (signed, 32-bit)` |
| `je` at 0x1262ad | `[sp-104] == 0x0` | `[sp-104] != 0x0` |
| `jne` at 0x1262bb | `[[sp+4]+20] != [0x1da69e]:2` | `[[sp+4]+20] == [0x1da69e]:2` |
| `jne` at 0x1262c1 | `[sp-96] != 0x1` | `[sp-96] == 0x1` |
| `je` at 0x1262db | `(? & 0x1f) == 0x0` | `(? & 0x1f) != 0x0` |
| `ja` at 0x12634a | `[[sp+4]+28]-2 > 0x16 (unsigned, 32-bit)` | `[[sp+4]+28]-2 <= 0x16 (unsigned, 32-bit)` |
| `jne` at 0x126610 | `[[sp+4]+28] != 0xb` | `[[sp+4]+28] == 0xb` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x126627 | `eax` | `?` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x126350).

## `_msgWrite`

- `_msgWrite`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x126680 to 0x126790.
  Established. The `_strlen` candidate is `[[sp+4]+12]` (`call` at 0x126727). A later `_write` has
  candidates `[[sp+4]+20]`, `[[sp+4]+12]`, and an unknown third slot (`call` at 0x126749). Explicit
  stores include `0x14` at `[sp+4] +0x1c` (`mov` at 0x12675f) and `([[sp+4]+12] + ret(_write))` at
  `[sp+4] +0xc` (`mov` at 0x12676b). The loop condition operands remain unknown (`jg` at 0x126772).
  Transfer extent and state meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x126686 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x126689 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x126690 | store | 4 | `[sp+4] +0x1c` | `0xb` |
| `mov` at 0x1266a0 | load | 4 | `abs 0x1ee88c` | `-` |
| `mov` at 0x1266b5 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1266b8 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x1266bc | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x1266c0 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x1266e7 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x126720 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x126723 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x12673e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x126741 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x126745 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x12675c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12675f | store | 4 | `[sp+4] +0x1c` | `0x14` |
| `mov` at 0x126766 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x12676b | store | 4 | `[sp+4] +0xc` | `([[sp+4]+12] + ret(_write))` |
| `mov` at 0x126774 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x126777 | store | 4 | `[sp+4] +0x1c` | `0xb` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1266a9 | `_setjmp 0x18fe80` | `[0x1ee88c]+4` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x1266c9 | `_ffscMsg 0x11ebd0` | `0x126628` | `[[sp+4]+20]` | `[[sp+4]+24]` | `[[sp+4]+28]` | `?` | `?` |
| `call` at 0x1266eb | `_timeoutSet 0x12a658` | `[sp+8]` | `0x126656` | `?` | `?` | `?` | `?` |
| `call` at 0x126705 | `_ffscMsgS 0x11eca0` | `0x126666` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126727 | `_strlen 0x172f14` | `[[sp+4]+12]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126738 | `_timeoutEnable 0x12a348` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126749 | `_write 0x186f70` | `[[sp+4]+20]` | `[[sp+4]+12]` | `?` | `?` | `?` | `?` |
| `call` at 0x126753 | `_timeoutDisable 0x12a328` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12677e | `_timeoutCancel 0x12a308` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x12668e | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x1266b3 | `(ret(_setjmp) & ret(_setjmp)) == 0x0` | `(ret(_setjmp) & ret(_setjmp)) != 0x0` |
| `je` at 0x1266f5 | `(ret(_timeoutSet) & ret(_timeoutSet)) == 0x0` | `(ret(_timeoutSet) & ret(_timeoutSet)) != 0x0` |
| `jle` at 0x126733 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jle` at 0x12675a | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jg` at 0x126772 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12678e | `eax` | `?` |

## `_timeoutCancel`

- `_timeoutCancel`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12a308 to 0x12a328.
  Established. The `_timer_cancel` call has candidate zero `[[0x1ee88c]+60]` (`call` at 0x12a314).
  After reloading the pointer (`mov` at 0x12a319), a four-byte zero is stored at `[0x1ee88c] +0x0`
  (`mov` at 0x12a31e). The returned register contains `[0x1ee88c]` (`ret` at 0x12a327). The meaning
  of the cleared value is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12a30b | load | 4 | `abs 0x1ee88c` | `-` |
| `mov` at 0x12a310 | load | 4 | `[0x1ee88c] +0x3c` | `-` |
| `mov` at 0x12a319 | load | 4 | `abs 0x1ee88c` | `-` |
| `mov` at 0x12a31e | store | 4 | `[0x1ee88c] +0x0` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12a314 | `_timer_cancel 0x178d70` | `[[0x1ee88c]+60]` | `ebp` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12a327 | `eax` | `[0x1ee88c]` |

## `_timeoutDisable`

- `_timeoutDisable`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12a328 to 0x12a348.
  Established. `_sigprocmask` receives the candidate prefix `0x1`, `[0x1ee88c]+56`, `0x0` (`call` at
  0x12a338). The return value is retained as `ret(_sigprocmask)` (`ret` at 0x12a340). No conditional
  branch or explicit memory store is reported. Signal-mask semantics are Unknown in this caller.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12a32d | load | 4 | `abs 0x1ee88c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12a338 | `_sigprocmask 0x18a620` | `0x1` | `[0x1ee88c]+56` | `0x0` | `ebp` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12a340 | `eax` | `ret(_sigprocmask)` |

## `_timeoutEnable`

- `_timeoutEnable`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12a348 to 0x12a408.
  Established. This unbranched wrapper calls `_sigprocmask` with candidates `0x2`,
  `[0x1ee88c]+56`, `0x0` (`call` at 0x12a358); its return register is `ret(_sigprocmask)` (`ret` at
  0x12a360). Bytes decoded after that return contribute no reachable effects. The command-value
  interpretation is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12a34d | load | 4 | `abs 0x1ee88c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12a358 | `_sigprocmask 0x18a620` | `0x2` | `[0x1ee88c]+56` | `0x0` | `ebp` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12a360 | `eax` | `ret(_sigprocmask)` |

## `_timeoutInit`

- `_timeoutInit`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12a408 to 0x12a538.
  Established. A `_malloc` call carries candidate zero `0x40` (`call` at 0x12a43a), and its result
  is stored at absolute `0x1ee88c` (`mov` at 0x12a442). `_timer_create` has the candidate prefix
  `0x0`, `0x0`, `[0x1ee88c]+60` (`call` at 0x12a475). The `_sigaction` call uses `0xe`, `sp-16`,
  `0x0` (`call` at 0x12a4ba); its nonzero result takes the arm containing `_timer_delete` (`jne` at
  0x12a4c4, `call` at 0x12a51e). Allocation and registration effects inside the callees remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12a442 | store | 4 | `abs 0x1ee88c` | `ret(_malloc)` |
| `mov` at 0x12a468 | load | 4 | `abs 0x1ee88c` | `-` |
| `mov` at 0x12a498 | store | 4 | `sp -0x8` | `0x8` |
| `mov` at 0x12a49f | store | 4 | `sp -0x10` | `0x12a868` |
| `mov` at 0x12a4c6 | load | 4 | `abs 0x1ee88c` | `-` |
| `mov` at 0x12a4d9 | load | 4 | `abs 0x1ee88c` | `-` |
| `mov` at 0x12a4ef | load | 4 | `abs 0x1ee88c` | `-` |
| `mov` at 0x12a4f4 | store | 4 | `[0x1ee88c] +0x0` | `0x0` |
| `mov` at 0x12a515 | load | 4 | `abs 0x1ee88c` | `-` |
| `mov` at 0x12a51a | load | 4 | `[0x1ee88c] +0x3c` | `-` |
| `mov` at 0x12a528 | load | 4 | `sp -0x14` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12a416 | `_taskVarAdd 0x178b44` | `0x0` | `0x1ee88c` | `ebx` | `?` | `?` | `?` |
| `call` at 0x12a427 | `_ffscMsgS 0x11eca0` | `0x12a361` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12a43a | `_malloc 0x17c878` | `0x40` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12a450 | `_ffscMsg 0x11ebd0` | `0x12a390` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12a475 | `_timer_create 0x178f00` | `0x0` | `0x0` | `[0x1ee88c]+60` | `?` | `?` | `?` |
| `call` at 0x12a486 | `_ffscMsgS 0x11eca0` | `0x12a3bd` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12a4ad | `_sigemptyset 0x18a390` | `sp-12` | `?` | `0x12a868` | `?` | `0x8` | `?` |
| `call` at 0x12a4ba | `_sigaction 0x18a4e0` | `0xe` | `sp-16` | `0x0` | `?` | `?` | `?` |
| `call` at 0x12a4cf | `_sigemptyset 0x18a390` | `[0x1ee88c]+56` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12a4e2 | `_sigaddset 0x18a3d0` | `[0x1ee88c]+56` | `0xe` | `?` | `?` | `?` | `?` |
| `call` at 0x12a4ea | `_timeoutDisable 0x12a328` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12a50d | `_ffscMsgS 0x11eca0` | `0x12a3e2` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12a51e | `_timer_delete 0x179010` | `[[0x1ee88c]+60]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12a420 | `(ret(_taskVarAdd) & ret(_taskVarAdd)) == 0x0` | `(ret(_taskVarAdd) & ret(_taskVarAdd)) != 0x0` |
| `jne` at 0x12a449 | `(ret(_malloc) & ret(_malloc)) != 0x0` | `(ret(_malloc) & ret(_malloc)) == 0x0` |
| `je` at 0x12a47f | `(ret(_timer_create) & ret(_timer_create)) == 0x0` | `(ret(_timer_create) & ret(_timer_create)) != 0x0` |
| `jne` at 0x12a4c4 | `(ret(_sigaction) & ret(_sigaction)) != 0x0` | `(ret(_sigaction) & ret(_sigaction)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12a52e | `eax` | `?` |

## `_timeoutRead`

- `_timeoutRead`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12a538 to 0x12a658.
  Established. The zero `_setjmp` result takes the setup arm (`je` at 0x12a54f). `_timeoutSet` has
  candidates `[sp+16]` and `0x12a52f` (`call` at 0x12a571). When its result is zero, the
  fall-through path reaches `_read` with candidates `[sp+4]`, `[sp+8]`, `[sp+12]` (`jne` at
  0x12a57b, `call` at 0x12a58e). Separate stores to `ret(___errno) +0x0` write four-byte values `0x4`
  and `0x16` (`mov` at 0x12a55b, `mov` at 0x12a5ad). The final merged return remains unknown (`ret`
  at 0x12a5be).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12a53c | load | 4 | `abs 0x1ee88c` | `-` |
| `mov` at 0x12a55b | store | 4 | `ret(___errno) +0x0` | `0x4` |
| `mov` at 0x12a56d | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x12a582 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x12a586 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12a58a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12a5ad | store | 4 | `ret(___errno) +0x0` | `0x16` |
| `mov` at 0x12a5b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12a545 | `_setjmp 0x18fe80` | `[0x1ee88c]+4` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x12a551 | `_timeoutCancel 0x12a308` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12a556 | `___errno 0x16fa78` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12a571 | `_timeoutSet 0x12a658` | `[sp+16]` | `0x12a52f` | `?` | `?` | `?` | `?` |
| `call` at 0x12a57d | `_timeoutEnable 0x12a348` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12a58e | `_read 0x186f50` | `[sp+4]` | `[sp+8]` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x12a598 | `_timeoutDisable 0x12a328` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12a59d | `_timeoutCancel 0x12a308` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12a5a8 | `___errno 0x16fa78` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12a54f | `(ret(_setjmp) & ret(_setjmp)) == 0x0` | `(ret(_setjmp) & ret(_setjmp)) != 0x0` |
| `jne` at 0x12a57b | `(ret(_timeoutSet) & ret(_timeoutSet)) != 0x0` | `(ret(_timeoutSet) & ret(_timeoutSet)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12a5be | `eax` | `?` |

## `_timeoutSet`

- `_timeoutSet`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12a658 to 0x12a788.
  Established. The initial zero test takes the arm containing `_sigpending` (`je` at 0x12a66c,
  `call` at 0x12a68c). The `_timer_settime` candidate prefix is `[[0x1ee88c]+60]`, `0x0`, `sp-20`,
  `0x0` (`call` at 0x12a739). The local stores at `sp -0xc` and `sp -0x8` have unknown values (`mov`
  at 0x12a6f9, `mov` at 0x12a725). A final four-byte store at `[0x1ee88c] +0x0` also has an unknown
  merged value (`mov` at 0x12a76a). Time units and timer configuration meaning are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12a660 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12a663 | load | 4 | `abs 0x1ee88c` | `-` |
| `mov` at 0x12a668 | load | 4 | `[0x1ee88c] +0x0` | `-` |
| `mov` at 0x12a6bc | load | 4 | `abs 0x1ee88c` | `-` |
| `mov` at 0x12a6d8 | store | 4 | `sp -0x14` | `0x0` |
| `mov` at 0x12a6df | store | 4 | `sp -0x10` | `0x0` |
| `mov` at 0x12a6f9 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x12a725 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x12a730 | load | 4 | `abs 0x1ee88c` | `-` |
| `mov` at 0x12a735 | load | 4 | `[0x1ee88c] +0x3c` | `-` |
| `mov` at 0x12a758 | load | 4 | `abs 0x1ee88c` | `-` |
| `mov` at 0x12a75e | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12a76a | store | 4 | `[0x1ee88c] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12a673 | `_ffscMsg 0x11ebd0` | `0x12a5bf` | `ebx` | `esi` | `?` | `?` | `?` |
| `call` at 0x12a68c | `_sigpending 0x18a700` | `sp-36` | `ebx` | `esi` | `?` | `?` | `?` |
| `call` at 0x12a6ab | `_sigismember 0x18a450` | `sp-36` | `0xe` | `?` | `?` | `?` | `?` |
| `call` at 0x12a6c5 | `_sigwaitinfo 0x18aac0` | `[0x1ee88c]+56` | `sp-32` | `?` | `?` | `?` | `?` |
| `call` at 0x12a739 | `_timer_settime 0x179330` | `[[0x1ee88c]+60]` | `0x0` | `sp-20` | `0x0` | `?` | `?` |
| `call` at 0x12a74c | `_ffscMsgS 0x11eca0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12a66c | `([[0x1ee88c]] & [[0x1ee88c]]) == 0x0` | `([[0x1ee88c]] & [[0x1ee88c]]) != 0x0` |
| `je` at 0x12a696 | `(ret(_sigpending) & ret(_sigpending)) == 0x0` | `(ret(_sigpending) & ret(_sigpending)) != 0x0` |
| `jne` at 0x12a6b6 | `ret(_sigismember) != 0x1` | `ret(_sigismember) == 0x1` |
| `jge` at 0x12a6cf | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x12a745 | `(ret(_timer_settime) & ret(_timer_settime)) == 0x0` | `(ret(_timer_settime) & ret(_timer_settime)) != 0x0` |
| `jne` at 0x12a763 | `([sp+8] & [sp+8]) != 0x0` | `([sp+8] & [sp+8]) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12a776 | `eax` | `?` |

## `_timeoutWrite`

- `_timeoutWrite`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12a788 to 0x12a868.
  Established. The `_timeoutSet` call carries `[sp+16]` and `0x12a777` (`call` at 0x12a7c1). A
  nonzero result takes the arm that stores `0x16` at `ret(___errno) +0x0` (`jne` at 0x12a7cb, `mov`
  at 0x12a7fd). The fall-through arm calls `_write` with candidates `[sp+4]`, `[sp+8]`, `[sp+12]`
  (`call` at 0x12a7de), followed by `_timeoutDisable` and `_timeoutCancel` (`call` at 0x12a7e8,
  `call` at 0x12a7ed). This establishes caller sequencing, while timeout behavior is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12a78c | load | 4 | `abs 0x1ee88c` | `-` |
| `mov` at 0x12a7ab | store | 4 | `ret(___errno) +0x0` | `0x4` |
| `mov` at 0x12a7bd | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x12a7d2 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x12a7d6 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12a7da | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12a7fd | store | 4 | `ret(___errno) +0x0` | `0x16` |
| `mov` at 0x12a808 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12a795 | `_setjmp 0x18fe80` | `[0x1ee88c]+4` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x12a7a1 | `_timeoutCancel 0x12a308` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12a7a6 | `___errno 0x16fa78` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12a7c1 | `_timeoutSet 0x12a658` | `[sp+16]` | `0x12a777` | `?` | `?` | `?` | `?` |
| `call` at 0x12a7cd | `_timeoutEnable 0x12a348` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12a7de | `_write 0x186f70` | `[sp+4]` | `[sp+8]` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x12a7e8 | `_timeoutDisable 0x12a328` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12a7ed | `_timeoutCancel 0x12a308` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12a7f8 | `___errno 0x16fa78` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12a79f | `(ret(_setjmp) & ret(_setjmp)) == 0x0` | `(ret(_setjmp) & ret(_setjmp)) != 0x0` |
| `jne` at 0x12a7cb | `(ret(_timeoutSet) & ret(_timeoutSet)) != 0x0` | `(ret(_timeoutSet) & ret(_timeoutSet)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12a80e | `eax` | `?` |

## `_identPrintTable`

- `_identPrintTable`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122668 to 0x122728.
  Established. The first `_lstFirst` candidate is `0x1d28e8` (`call` at 0x122671). The loop includes
  calls to `_ffscMsg`, `_identPrint`, and `_lstNext` (`call` at 0x122685, `call` at 0x122691, `call`
  at 0x12269a); its deciding branch is taken when `Z == 0` (`jne` at 0x1226a6). `_lstCount` receives
  candidate zero `0x1d28e8` (`call` at 0x1226ad), and its result reaches the return (`ret` at
  0x1226b8). Element fields and list meaning are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x122678 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12267c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1226b2 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122671 | `_lstFirst 0x18df58` | `0x1d28e8` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x122685 | `_ffscMsg 0x11ebd0` | `0x122651` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122691 | `_identPrint 0x122ae8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12269a | `_lstNext 0x18e008` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1226ad | `_lstCount 0x18de68` | `0x1d28e8` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x1226a6 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1226b8 | `eax` | `ret(_lstCount)` |

## `_identPrint`

- `_identPrint`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122ae8 to 0x122bf8.
  Established. The formatting role is Unknown.
  `_ffscUniqueIDToString` carries `[sp+4]`, `sp-96` (`call` at 0x122b26), while
  `_ffscRackIDToString` carries `[[sp+4]+12]`, `sp-72` (`call` at 0x122b4c). The first `_ffscModuleNumToString`
  candidate remains unknown (`call` at 0x122b69). Two `_strcat` calls use `sp-28` with `0x122a80`
  and `sp-40` respectively (`call` at 0x122b77, `call` at 0x122b81). The final `_ffscMsg` candidate
  list is retained below (`call` at 0x122bbc). The return value is zero (`ret` at 0x122bcc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x122af1 | store | 4 | `sp -0x64` | `0x0` |
| `cmp` at 0x122af8 | load | 4 | `abs 0x1eb6cc` | `-` |
| `mov` at 0x122b01 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122b05 | load | 4 | `abs 0x1eb6cc` | `-` |
| `mov` at 0x122b17 | store | 4 | `sp -0x64` | `0x1` |
| `mov` at 0x122b22 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122b32 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122b35 | load | 4 | `[sp+4] +0x8` | `-` |
| `mov` at 0x122b45 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122b48 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x122b54 | store | 1 | `sp -0x1c` | `0x0` |
| `mov` at 0x122b61 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122b64 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122b8f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122b92 | load | 4 | `[sp+4] +0x10` | `-` |
| `cmp` at 0x122ba7 | load | 4 | `sp -0x64` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122b0b | `_ffscCompareUniqueIDs 0x1253bc` | `[0x1eb6cc]` | `[sp+4]` | `ebx` | `esi` | `edi` | `0x0` |
| `call` at 0x122b26 | `_ffscUniqueIDToString 0x125bec` | `[sp+4]` | `sp-96` | `?` | `?` | `?` | `?` |
| `call` at 0x122b39 | `_inet_ntoa_b 0x1868e0` | `[[sp+4]+8]` | `sp-60` | `?` | `?` | `?` | `?` |
| `call` at 0x122b4c | `_ffscRackIDToString 0x12594c` | `[[sp+4]+12]` | `sp-72` | `?` | `?` | `?` | `?` |
| `call` at 0x122b69 | `_ffscModuleNumToString 0x1255cc` | `?` | `sp-40` | `?` | `?` | `?` | `?` |
| `call` at 0x122b77 | `_strcat 0x17de28` | `sp-28` | `0x122a80` | `?` | `?` | `?` | `?` |
| `call` at 0x122b81 | `_strcat 0x17de28` | `sp-28` | `sp-40` | `?` | `?` | `?` | `?` |
| `call` at 0x122bbc | `_ffscMsg 0x11ebd0` | `0x122a8b` | `sp-96` | `?` | `sp-72` | `sp-60` | `sp-28` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x122aff | `[0x1eb6cc] == 0x0` | `[0x1eb6cc] != 0x0` |
| `jne` at 0x122b15 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `jle` at 0x122b8d | `? <= 0x1 (signed, 32-bit)` | `? > 0x1 (signed, 32-bit)` |
| `je` at 0x122bab | `[sp-100] == 0x0` | `[sp-100] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122bcc | `eax` | `0x0` |

## `_cuUserInELSC`

- `_cuUserInELSC`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x11dfe0 to 0x11e120.
  Established. The branch takes the main arm when `[[sp+4]+36] == 0x1` (`je` at 0x11dff8). A
  one-byte zero goes to `([[[sp+4]+112]+16] + [[[sp+4]+112]+4]) +0x0` (`mov` at 0x11e026).
  `_bufWrite` carries candidates `[[sp+4]+108]`, `[[[sp+4]+112]+4]`, and an unknown third slot
  (`call` at 0x11e065). The later command call carries `[sp+4]`, `0x11dfd4`, `sp-1464` (`call` at
  0x11e0a4). The command semantics and ELSC interaction remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x11dfec | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x11dfef | load | 4 | `[sp+4] +0x70` | `-` |
| `mov` at 0x11dff2 | load | 4 | `[sp+4] +0x24` | `-` |
| `mov` at 0x11dffa | load | 4 | `[sp+4] +0x24` | `-` |
| `mov` at 0x11e020 | load | 4 | `[[sp+4]+112] +0x4` | `-` |
| `mov` at 0x11e023 | load | 4 | `[[sp+4]+112] +0x10` | `-` |
| `mov` at 0x11e026 | store | 1 | `([[[sp+4]+112]+16] + [[[sp+4]+112]+4]) +0x0` | `0x0` |
| `mov` at 0x11e02a | load | 1 | `[sp+4] +0x90` | `-` |
| `mov` at 0x11e034 | load | 4 | `[[sp+4]+112] +0x4` | `-` |
| `mov` at 0x11e046 | load | 4 | `[[sp+4]+112] +0x4` | `-` |
| `mov` at 0x11e049 | load | 4 | `[[sp+4]+112] +0x10` | `-` |
| `mov` at 0x11e04f | load | 4 | `[[sp+4]+112] +0x4` | `-` |
| `mov` at 0x11e05d | load | 4 | `[[sp+4]+112] +0x4` | `-` |
| `mov` at 0x11e061 | load | 4 | `[sp+4] +0x6c` | `-` |
| `mov` at 0x11e06d | load | 1 | `(unknown)` | `-` |
| `test` at 0x11e080 | load | 1 | `abs 0x1da698` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x11e004 | `_ffscMsg 0x11ebd0` | `0x11df80` | `[sp+4]` | `[[sp+4]+36]` | `ebx` | `esi` | `edi` |
| `call` at 0x11e038 | `_strchr 0x17de58` | `[[[sp+4]+112]+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11e065 | `_bufWrite 0x10fb78` | `[[sp+4]+108]` | `[[[sp+4]+112]+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x11e074 | `_bufClear 0x10f438` | `[[sp+4]+112]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11e08f | `_ffscMsg 0x11ebd0` | `0x11dfad` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x11e0a4 | `_cmSendFFSCCmd 0x11a290` | `[sp+4]` | `0x11dfd4` | `sp-1464` | `?` | `?` | `?` |
| `call` at 0x11e0ae | `_cmPrintResponse 0x11a120` | `[sp+4]` | `sp-1464` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x11dff8 | `[[sp+4]+36] == 0x1` | `[[sp+4]+36] != 0x1` |
| `jne` at 0x11e044 | `(ret(_strchr) & ret(_strchr)) != 0x0` | `(ret(_strchr) & ret(_strchr)) == 0x0` |
| `jle` at 0x11e05a | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jne` at 0x11e071 | `Z == 0` | `not (Z == 0)` |
| `je` at 0x11e087 | `(([0x1da698]:1 & 0xff) & 0x4) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x11e0c1 | `eax` | `?` |
