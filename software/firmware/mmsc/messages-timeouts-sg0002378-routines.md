# MMSC messages and timeouts in patch SG0002378

Read from `patches/SG0002378/usr/cpu/firmware/mmscfw.bin` (patch SG0002378).
Image SHA-256: `b1249e58cf4f00c252362da8ed0515e25bb94aaf096bb60a5c5082c9b6e4d342`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred end and can include literals or padding
after returns. Statements below concern sites reached by known control-flow edges.
The computed jump in `_msgRead` remains unresolved; its selector-arm effects are excluded.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/messages-timeouts-sg0002378-facts.txt) include all tool sections.

## `_msgClear`

- `_msgClear`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x126058 to 0x126088.
  Established. On the nonzero-input fall-through path (`je` at 0x126060), four-byte stores place
  `[[sp+4]+8]` at `[sp+4] +0xc` (`mov` at 0x126065) and zero at `[sp+4] +0x1c` (`mov` at 0x126068).
  The `_bzero` call has candidate zero `[[sp+4]+8]` and unknown candidate one (`call` at 0x126078).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12605b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x126062 | load | 4 | `[sp+4] +0x8` | `-` |
| `mov` at 0x126065 | store | 4 | `[sp+4] +0xc` | `[[sp+4]+8]` |
| `mov` at 0x126068 | store | 4 | `[sp+4] +0x1c` | `0x0` |
| `mov` at 0x12606f | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x126074 | load | 4 | `[sp+4] +0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x126078 | `_bzero 0x172678` | `[[sp+4]+8]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x126060 | `([sp+4] & [sp+4]) == 0x0` | `([sp+4] & [sp+4]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x126080 | `eax` | `?` |

## `_msgFree`

- `_msgFree`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x126088 to 0x126108.
  Established. The zero-input branch bypasses both calls when `([sp+4] & [sp+4]) == 0x0` (`je` at
  0x126091). On fall-through, the `_free` sites present candidate zero `[[sp+4]+8]` (`call` at
  0x126097), then `[sp+4]` (`call` at 0x1260a0). The loaded record field has no established meaning
  here.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12608c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x126093 | load | 4 | `[sp+4] +0x8` | `-` |
| `mov` at 0x1260a5 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x126097 | `_free 0x17ec48` | `[[sp+4]+8]` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x1260a0 | `_free 0x17ec48` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x126091 | `([sp+4] & [sp+4]) == 0x0` | `([sp+4] & [sp+4]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1260ab | `eax` | `?` |

## `_msgNew`

- `_msgNew`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x126108 to 0x126348.
  Established. The two `_malloc` sites have candidate zero `0x20` (`call` at 0x126116) and
  `[sp+4]+1` (`call` at 0x12613c). The four-byte stores through the symbolic origin `ret(_malloc)`
  include `[sp+4]` at `+0x10` (`mov` at 0x12614e), `[sp+8]` at `+0x14` (`mov` at 0x126151), and
  `[sp+12]` at `+0x18` (`mov` at 0x126157). A byte store clears `ret(_malloc) +0x7` (`mov` at
  0x126169).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12610e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x126111 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x126144 | store | 4 | `ret(_malloc) +0x8` | `ret(_malloc)` |
| `mov` at 0x126147 | load | 4 | `ret(_malloc) +0x8` | `-` |
| `mov` at 0x12614e | store | 4 | `ret(_malloc) +0x10` | `[sp+4]` |
| `mov` at 0x126151 | store | 4 | `ret(_malloc) +0x14` | `[sp+8]` |
| `mov` at 0x126154 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x126157 | store | 4 | `ret(_malloc) +0x18` | `[sp+12]` |
| `mov` at 0x12615c | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x126169 | store | 1 | `ret(_malloc) +0x7` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x126116 | `_malloc 0x17ec28` | `0x20` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x126129 | `_ffscMsg 0x11f040` | `0x1260ac` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12613c | `_malloc 0x17ec28` | `[sp+4]+1` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126161 | `_strncpy 0x180560` | `ret(_malloc)` | `[sp+16]` | `0x8` | `?` | `?` | `?` |
| `call` at 0x12616e | `_msgClear 0x126058` | `ret(_malloc)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12617d | `_ffscMsg 0x11f040` | `0x1260d1` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126186 | `_free 0x17ec48` | `ret(_malloc)` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x126122 | `(ret(_malloc) & ret(_malloc)) != 0x0` | `(ret(_malloc) & ret(_malloc)) == 0x0` |
| `je` at 0x12614c | `([ret(_malloc)+8] & [ret(_malloc)+8]) == 0x0` | `([ret(_malloc)+8] & [ret(_malloc)+8]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x126196 | `eax` | `?` |

## `_msgRead`

- `_msgRead`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x126348 to 0x1269c8.
  Established. The `_read` site has leading candidates `[[sp+4]+20]`, `[[sp+4]+12]`, and `[sp-96]`
  (`call` at 0x12647b). A byte store writes zero to `([sp-104] + [[sp+4]+12]) +0x0` (`mov` at
  0x126522). The selector-range branch is taken when `[[sp+4]+28]-2 > 0x18 (unsigned, 32-bit)` (`ja`
  at 0x126532). Effects behind the unresolved computed jump are excluded from this entry.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x126351 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x126354 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x12635b | load | 4 | `abs 0x1f0f2c` | `-` |
| `mov` at 0x126370 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x126373 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x126377 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x12637b | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x12638c | store | 4 | `sp -0x64` | `0x0` |
| `mov` at 0x126398 | store | 4 | `sp -0x64` | `0x0` |
| `mov` at 0x1263a4 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1263c8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1263cb | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x1263ce | load | 4 | `[sp+4] +0x8` | `-` |
| `mov` at 0x1263d3 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x1263dc | store | 4 | `sp -0x5c` | `0x0` |
| `mov` at 0x1263e3 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x1263eb | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x1263f3 | store | 4 | `sp -0x60` | `?` |
| `mov` at 0x126415 | store | 4 | `sp -0x64` | `0xffffffff` |
| `mov` at 0x126428 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12642b | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x12643c | store | 4 | `sp -0x64` | `0xffffffff` |
| `mov` at 0x126448 | store | 4 | `sp -0x60` | `0x1` |
| `mov` at 0x126458 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12645b | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x12646c | load | 4 | `sp -0x60` | `-` |
| `mov` at 0x126470 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x126473 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x126477 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x126483 | store | 4 | `sp -0x68` | `ret(_read)` |
| `cmp` at 0x12648b | load | 4 | `sp -0x68` | `-` |
| `movzx` at 0x126497 | load | 2 | `abs 0x1dccc6` | `-` |
| `mov` at 0x12649e | load | 4 | `[sp+4] +0x14` | `-` |
| `cmp` at 0x1264a5 | load | 4 | `sp -0x60` | `-` |
| `mov` at 0x1264ab | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x1264ae | load | 1 | `[[sp+4]+12] +0x0` | `-` |
| `mov` at 0x1264b4 | load | 1 | `[[sp+4]+12] +0x0` | `-` |
| `mov` at 0x1264b9 | load | 4 | `abs 0x1d34c0` | `-` |
| `mov` at 0x1264be | load | 1 | `(unknown)` | `-` |
| `mov` at 0x1264c5 | load | 1 | `[[sp+4]+12] +0x0` | `-` |
| `mov` at 0x1264de | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1264e1 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x1264f8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1264fb | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x1264ff | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x126519 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12651c | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x12651f | load | 4 | `sp -0x68` | `-` |
| `mov` at 0x126522 | store | 1 | `([sp-104] + [[sp+4]+12]) +0x0` | `0x0` |
| `mov` at 0x126526 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x126529 | load | 4 | `[sp+4] +0x1c` | `-` |
| `jmp` at 0x126538 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x126868 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12686b | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x1268ec | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1268ef | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x126900 | load | 4 | `sp -0x64` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x126364 | `_setjmp 0x192088` | `[0x1f0f2c]+4` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x126384 | `_ffscMsg 0x11f040` | `0x126197` | `[[sp+4]+20]` | `[[sp+4]+24]` | `[[sp+4]+28]` | `?` | `?` |
| `call` at 0x1263a8 | `_timeoutSet 0x12aef0` | `[sp+8]` | `0x1261c7` | `?` | `?` | `?` | `?` |
| `call` at 0x12640d | `_ffscMsgS 0x11f110` | `0x1261d7` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126434 | `_ffscMsg 0x11f040` | `0x1261ed` | `[[sp+4]+20]` | `?` | `?` | `?` | `?` |
| `call` at 0x126467 | `_timeoutEnable 0x12abe0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12647b | `_read 0x189300` | `[[sp+4]+20]` | `[[sp+4]+12]` | `[sp-96]` | `?` | `?` | `?` |
| `call` at 0x126486 | `_timeoutDisable 0x12abc0` | `?` | `?` | `?` | `ret(_read)` | `?` | `?` |
| `call` at 0x1264ea | `_ffscMsg 0x11f040` | `0x1261fa` | `[[sp+4]+20]` | `?` | `?` | `?` | `?` |
| `call` at 0x126503 | `_strlen 0x17466c` | `[[sp+4]+12]` | `[[sp+4]+12]` | `?` | `?` | `?` | `?` |
| `call` at 0x126511 | `_ffscMsg 0x11f040` | `0x12621c` | `ret(_strlen)` | `?` | `?` | `?` | `?` |
| `call` at 0x126874 | `_ffscMsg 0x11f040` | `0x1262fe` | `[[sp+4]+28]` | `?` | `?` | `?` | `?` |
| `call` at 0x1268fb | `_timeoutCancel 0x12aba0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jl` at 0x126359 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x12636e | `(ret(_setjmp) & ret(_setjmp)) == 0x0` | `(ret(_setjmp) & ret(_setjmp)) != 0x0` |
| `je` at 0x1263b2 | `(ret(_timeoutSet) & ret(_timeoutSet)) == 0x0` | `(ret(_timeoutSet) & ret(_timeoutSet)) != 0x0` |
| `je` at 0x1263e9 | `[[sp+4]+28] == 0x2` | `[[sp+4]+28] != 0x2` |
| `jne` at 0x1263f1 | `[[sp+4]+28] != 0x6` | `[[sp+4]+28] == 0x6` |
| `je` at 0x126461 | `[[sp+4]+28] == 0xa` | `[[sp+4]+28] != 0xa` |
| `jl` at 0x12648f | `[sp-104] < 0x0 (signed, 32-bit)` | `[sp-104] >= 0x0 (signed, 32-bit)` |
| `je` at 0x126495 | `[sp-104] == 0x0` | `[sp-104] != 0x0` |
| `jne` at 0x1264a3 | `[[sp+4]+20] != [0x1dccc6]:2` | `[[sp+4]+20] == [0x1dccc6]:2` |
| `jne` at 0x1264a9 | `[sp-96] != 0x1` | `[sp-96] == 0x1` |
| `je` at 0x1264c3 | `(? & 0x1f) == 0x0` | `(? & 0x1f) != 0x0` |
| `ja` at 0x126532 | `[[sp+4]+28]-2 > 0x18 (unsigned, 32-bit)` | `[[sp+4]+28]-2 <= 0x18 (unsigned, 32-bit)` |
| `jne` at 0x1268f5 | `[[sp+4]+28] != 0xb` | `[[sp+4]+28] == 0xb` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12690c | `eax` | `?` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x126538).

## `_msgWrite`

- `_msgWrite`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1269c8 to 0x126b78.
  Established. The `_write` call presents leading candidates `[[sp+4]+20]` and `[[sp+4]+12]`, with
  unknown candidate two (`call` at 0x126ab5). A four-byte store writes `0x14` to `[sp+4] +0x1c`
  (`mov` at 0x126b41), followed by a four-byte store of `([[sp+4]+12] + ret(_write))` to `[sp+4]
  +0xc` (`mov` at 0x126b4d). The record status meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1269d1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1269d4 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x1269db | store | 4 | `[sp+4] +0x1c` | `0xb` |
| `mov` at 0x1269f8 | load | 4 | `abs 0x1f0f2c` | `-` |
| `mov` at 0x126a0d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x126a10 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x126a14 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x126a18 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x126a29 | store | 4 | `sp -0x8` | `0x0` |
| `mov` at 0x126a38 | store | 4 | `sp -0x8` | `0x0` |
| `mov` at 0x126a44 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x126a77 | store | 4 | `sp -0x8` | `0xffffffff` |
| `mov` at 0x126a88 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x126a8b | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x126aaa | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x126aad | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x126ab1 | load | 4 | `[sp+4] +0x14` | `-` |
| `movzx` at 0x126ac8 | load | 2 | `abs 0x1dccc6` | `-` |
| `mov` at 0x126acf | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x126adb | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x126ade | load | 1 | `[[sp+4]+12] +0x0` | `-` |
| `mov` at 0x126ae4 | load | 1 | `[[sp+4]+12] +0x0` | `-` |
| `mov` at 0x126ae9 | load | 4 | `abs 0x1d34c0` | `-` |
| `mov` at 0x126aee | load | 1 | `(unknown)` | `-` |
| `mov` at 0x126af5 | load | 1 | `[[sp+4]+12] +0x0` | `-` |
| `mov` at 0x126b0e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x126b11 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x126b28 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x126b2b | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x126b3e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x126b41 | store | 4 | `[sp+4] +0x1c` | `0x14` |
| `mov` at 0x126b48 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x126b4d | store | 4 | `[sp+4] +0xc` | `([[sp+4]+12] + ret(_write))` |
| `mov` at 0x126b5a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x126b5d | store | 4 | `[sp+4] +0x1c` | `0xb` |
| `mov` at 0x126b69 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x126a01 | `_setjmp 0x192088` | `[0x1f0f2c]+4` | `ebx` | `esi` | `edi` | `?` | `ebp` |
| `call` at 0x126a21 | `_ffscMsg 0x11f040` | `0x12690d` | `[[sp+4]+20]` | `[[sp+4]+24]` | `[[sp+4]+28]` | `?` | `?` |
| `call` at 0x126a48 | `_timeoutSet 0x12aef0` | `[sp+8]` | `0x12693b` | `?` | `?` | `?` | `0x0` |
| `call` at 0x126a6f | `_ffscMsgS 0x11f110` | `0x12694b` | `ret(_write)` | `?` | `?` | `?` | `?` |
| `call` at 0x126a8f | `_strlen 0x17466c` | `[[sp+4]+12]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126aa4 | `_timeoutEnable 0x12abe0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126ab5 | `_write 0x189320` | `[[sp+4]+20]` | `[[sp+4]+12]` | `?` | `?` | `?` | `?` |
| `call` at 0x126abf | `_timeoutDisable 0x12abc0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126b36 | `_ffscMsg 0x11f040` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126b64 | `_timeoutCancel 0x12aba0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x1269d9 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x126a0b | `(ret(_setjmp) & ret(_setjmp)) == 0x0` | `(ret(_setjmp) & ret(_setjmp)) != 0x0` |
| `je` at 0x126a52 | `(ret(_timeoutSet) & ret(_timeoutSet)) == 0x0` | `(ret(_timeoutSet) & ret(_timeoutSet)) != 0x0` |
| `jle` at 0x126a9b | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jle` at 0x126ac6 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jne` at 0x126ad4 | `[[sp+4]+20] != [0x1dccc6]:2` | `[[sp+4]+20] == [0x1dccc6]:2` |
| `jne` at 0x126ad9 | `ret(_write) != 0x1` | `ret(_write) == 0x1` |
| `je` at 0x126af3 | `(? & 0x1f) == 0x0` | `(? & 0x1f) != 0x0` |
| `jg` at 0x126b54 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x126b75 | `eax` | `?` |

## `_timeoutCancel`

- `_timeoutCancel`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12aba0 to 0x12abc0.
  Established. The `_timer_cancel` site has candidate zero `[[0x1f0f2c]+60]` (`call` at 0x12abac).
  The following four-byte store clears `[0x1f0f2c] +0x0` (`mov` at 0x12abb6). The return exposes
  `[0x1f0f2c]` in `eax` (`ret` at 0x12abbf).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12aba3 | load | 4 | `abs 0x1f0f2c` | `-` |
| `mov` at 0x12aba8 | load | 4 | `[0x1f0f2c] +0x3c` | `-` |
| `mov` at 0x12abb1 | load | 4 | `abs 0x1f0f2c` | `-` |
| `mov` at 0x12abb6 | store | 4 | `[0x1f0f2c] +0x0` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12abac | `_timer_cancel 0x17b120` | `[[0x1f0f2c]+60]` | `ebp` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12abbf | `eax` | `[0x1f0f2c]` |

## `_timeoutDisable`

- `_timeoutDisable`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12abc0 to 0x12abe0.
  Established. The `_sigprocmask` call has leading candidates `0x1`, `[0x1f0f2c]+56`, and `0x0`
  (`call` at 0x12abd0); its return expression reaches the routine exit as `ret(_sigprocmask)` (`ret`
  at 0x12abd8). This span establishes no signal-set contents.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12abc5 | load | 4 | `abs 0x1f0f2c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12abd0 | `_sigprocmask 0x18c9d0` | `0x1` | `[0x1f0f2c]+56` | `0x0` | `ebp` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12abd8 | `eax` | `ret(_sigprocmask)` |

## `_timeoutEnable`

- `_timeoutEnable`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12abe0 to 0x12aca0.
  Established. The single `_sigprocmask` site uses leading candidates `0x2`, `[0x1f0f2c]+56`, and
  `0x0` (`call` at 0x12abf0); its call result is retained at the exit (`ret` at 0x12abf8). The mask
  operation meaning is unknown from this body alone.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12abe5 | load | 4 | `abs 0x1f0f2c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12abf0 | `_sigprocmask 0x18c9d0` | `0x2` | `[0x1f0f2c]+56` | `0x0` | `ebp` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12abf8 | `eax` | `ret(_sigprocmask)` |

## `_timeoutInit`

- `_timeoutInit`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12aca0 to 0x12add0.
  Established. The `_malloc` call has candidate zero `0x40` (`call` at 0x12acd2), and a four-byte
  store places its result at absolute `0x1f0f2c` (`mov` at 0x12acda). The `_timer_create` site
  presents `0x0`, `0x0`, and `[0x1f0f2c]+60` (`call` at 0x12ad0d). The `_sigaction` site has
  candidates `0xe`, `sp-16`, and `0x0` (`call` at 0x12ad52).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12acda | store | 4 | `abs 0x1f0f2c` | `ret(_malloc)` |
| `mov` at 0x12ad00 | load | 4 | `abs 0x1f0f2c` | `-` |
| `mov` at 0x12ad30 | store | 4 | `sp -0x8` | `0x8` |
| `mov` at 0x12ad37 | store | 4 | `sp -0x10` | `0x12b100` |
| `mov` at 0x12ad5e | load | 4 | `abs 0x1f0f2c` | `-` |
| `mov` at 0x12ad71 | load | 4 | `abs 0x1f0f2c` | `-` |
| `mov` at 0x12ad87 | load | 4 | `abs 0x1f0f2c` | `-` |
| `mov` at 0x12ad8c | store | 4 | `[0x1f0f2c] +0x0` | `0x0` |
| `mov` at 0x12adad | load | 4 | `abs 0x1f0f2c` | `-` |
| `mov` at 0x12adb2 | load | 4 | `[0x1f0f2c] +0x3c` | `-` |
| `mov` at 0x12adc0 | load | 4 | `sp -0x14` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12acae | `_taskVarAdd 0x17aef4` | `0x0` | `0x1f0f2c` | `ebx` | `?` | `?` | `?` |
| `call` at 0x12acbf | `_ffscMsgS 0x11f110` | `0x12abf9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12acd2 | `_malloc 0x17ec28` | `0x40` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12ace8 | `_ffscMsg 0x11f040` | `0x12ac28` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12ad0d | `_timer_create 0x17b2b0` | `0x0` | `0x0` | `[0x1f0f2c]+60` | `?` | `?` | `?` |
| `call` at 0x12ad1e | `_ffscMsgS 0x11f110` | `0x12ac55` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12ad45 | `_sigemptyset 0x18c740` | `sp-12` | `?` | `0x12b100` | `?` | `0x8` | `?` |
| `call` at 0x12ad52 | `_sigaction 0x18c890` | `0xe` | `sp-16` | `0x0` | `?` | `?` | `?` |
| `call` at 0x12ad67 | `_sigemptyset 0x18c740` | `[0x1f0f2c]+56` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12ad7a | `_sigaddset 0x18c780` | `[0x1f0f2c]+56` | `0xe` | `?` | `?` | `?` | `?` |
| `call` at 0x12ad82 | `_timeoutDisable 0x12abc0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12ada5 | `_ffscMsgS 0x11f110` | `0x12ac7a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12adb6 | `_timer_delete 0x17b3c0` | `[[0x1f0f2c]+60]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12acb8 | `(ret(_taskVarAdd) & ret(_taskVarAdd)) == 0x0` | `(ret(_taskVarAdd) & ret(_taskVarAdd)) != 0x0` |
| `jne` at 0x12ace1 | `(ret(_malloc) & ret(_malloc)) != 0x0` | `(ret(_malloc) & ret(_malloc)) == 0x0` |
| `je` at 0x12ad17 | `(ret(_timer_create) & ret(_timer_create)) == 0x0` | `(ret(_timer_create) & ret(_timer_create)) != 0x0` |
| `jne` at 0x12ad5c | `(ret(_sigaction) & ret(_sigaction)) != 0x0` | `(ret(_sigaction) & ret(_sigaction)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12adc6 | `eax` | `?` |

## `_timeoutRead`

- `_timeoutRead`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12add0 to 0x12aef0.
  Established. The `_read` site forwards candidates `[sp+4]`, `[sp+8]`, and `[sp+12]` (`call` at
  0x12ae26). The fall-through path when `(ret(_setjmp) & ret(_setjmp)) != 0x0` (`je` at 0x12ade7)
  writes `0x4` through `ret(___errno) +0x0` as four bytes (`mov` at 0x12adf3). The taken path when
  `(ret(_timeoutSet) & ret(_timeoutSet)) != 0x0` (`jne` at 0x12ae13) instead stores `0x16` through
  that symbolic target (`mov` at 0x12ae45).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12add4 | load | 4 | `abs 0x1f0f2c` | `-` |
| `mov` at 0x12adf3 | store | 4 | `ret(___errno) +0x0` | `0x4` |
| `mov` at 0x12ae05 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x12ae1a | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x12ae1e | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12ae22 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12ae45 | store | 4 | `ret(___errno) +0x0` | `0x16` |
| `mov` at 0x12ae50 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12addd | `_setjmp 0x192088` | `[0x1f0f2c]+4` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x12ade9 | `_timeoutCancel 0x12aba0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12adee | `___errno 0x1711d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12ae09 | `_timeoutSet 0x12aef0` | `[sp+16]` | `0x12adc7` | `?` | `?` | `?` | `?` |
| `call` at 0x12ae15 | `_timeoutEnable 0x12abe0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12ae26 | `_read 0x189300` | `[sp+4]` | `[sp+8]` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x12ae30 | `_timeoutDisable 0x12abc0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12ae35 | `_timeoutCancel 0x12aba0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12ae40 | `___errno 0x1711d0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12ade7 | `(ret(_setjmp) & ret(_setjmp)) == 0x0` | `(ret(_setjmp) & ret(_setjmp)) != 0x0` |
| `jne` at 0x12ae13 | `(ret(_timeoutSet) & ret(_timeoutSet)) != 0x0` | `(ret(_timeoutSet) & ret(_timeoutSet)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12ae56 | `eax` | `?` |

## `_timeoutSet`

- `_timeoutSet`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12aef0 to 0x12b020.
  Established. The `_sigismember` call has leading candidates `sp-36` and `0xe` (`call` at
  0x12af43). The `_timer_settime` site presents `[[0x1f0f2c]+60]`, `0x0`, `sp-20`, and `0x0` (`call`
  at 0x12afd1). When `(ret(_timer_settime) & ret(_timer_settime)) == 0x0`, the branch is taken (`je`
  at 0x12afdd). The later four-byte store to `[0x1f0f2c] +0x0` has unknown value (`mov` at
  0x12b002).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12aef8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12aefb | load | 4 | `abs 0x1f0f2c` | `-` |
| `mov` at 0x12af00 | load | 4 | `[0x1f0f2c] +0x0` | `-` |
| `mov` at 0x12af54 | load | 4 | `abs 0x1f0f2c` | `-` |
| `mov` at 0x12af70 | store | 4 | `sp -0x14` | `0x0` |
| `mov` at 0x12af77 | store | 4 | `sp -0x10` | `0x0` |
| `mov` at 0x12af91 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x12afbd | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x12afc8 | load | 4 | `abs 0x1f0f2c` | `-` |
| `mov` at 0x12afcd | load | 4 | `[0x1f0f2c] +0x3c` | `-` |
| `mov` at 0x12aff0 | load | 4 | `abs 0x1f0f2c` | `-` |
| `mov` at 0x12aff6 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12b002 | store | 4 | `[0x1f0f2c] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12af0b | `_ffscMsg 0x11f040` | `0x12ae57` | `ebx` | `esi` | `?` | `?` | `?` |
| `call` at 0x12af24 | `_sigpending 0x18cab0` | `sp-36` | `ebx` | `esi` | `?` | `?` | `?` |
| `call` at 0x12af43 | `_sigismember 0x18c800` | `sp-36` | `0xe` | `?` | `?` | `?` | `?` |
| `call` at 0x12af5d | `_sigwaitinfo 0x18ce70` | `[0x1f0f2c]+56` | `sp-32` | `?` | `?` | `?` | `?` |
| `call` at 0x12afd1 | `_timer_settime 0x17b6e0` | `[[0x1f0f2c]+60]` | `0x0` | `sp-20` | `0x0` | `?` | `?` |
| `call` at 0x12afe4 | `_ffscMsgS 0x11f110` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12af04 | `([[0x1f0f2c]] & [[0x1f0f2c]]) == 0x0` | `([[0x1f0f2c]] & [[0x1f0f2c]]) != 0x0` |
| `je` at 0x12af2e | `(ret(_sigpending) & ret(_sigpending)) == 0x0` | `(ret(_sigpending) & ret(_sigpending)) != 0x0` |
| `jne` at 0x12af4e | `ret(_sigismember) != 0x1` | `ret(_sigismember) == 0x1` |
| `jge` at 0x12af67 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x12afdd | `(ret(_timer_settime) & ret(_timer_settime)) == 0x0` | `(ret(_timer_settime) & ret(_timer_settime)) != 0x0` |
| `jne` at 0x12affb | `([sp+8] & [sp+8]) != 0x0` | `([sp+8] & [sp+8]) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12b00e | `eax` | `?` |

## `_timeoutWrite`

- `_timeoutWrite`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12b020 to 0x12b100.
  Established. The `_write` call has leading candidates `[sp+4]`, `[sp+8]`, and `[sp+12]` (`call` at
  0x12b076). The branch is taken when `(ret(_timeoutSet) & ret(_timeoutSet)) != 0x0` (`jne` at
  0x12b063), reaching a four-byte store of `0x16` through `ret(___errno) +0x0` (`mov` at 0x12b095).
  A separate earlier path stores `0x4` through the same symbolic origin (`mov` at 0x12b043).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12b024 | load | 4 | `abs 0x1f0f2c` | `-` |
| `mov` at 0x12b043 | store | 4 | `ret(___errno) +0x0` | `0x4` |
| `mov` at 0x12b055 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x12b06a | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x12b06e | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12b072 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12b095 | store | 4 | `ret(___errno) +0x0` | `0x16` |
| `mov` at 0x12b0a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12b02d | `_setjmp 0x192088` | `[0x1f0f2c]+4` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x12b039 | `_timeoutCancel 0x12aba0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12b03e | `___errno 0x1711d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12b059 | `_timeoutSet 0x12aef0` | `[sp+16]` | `0x12b00f` | `?` | `?` | `?` | `?` |
| `call` at 0x12b065 | `_timeoutEnable 0x12abe0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12b076 | `_write 0x189320` | `[sp+4]` | `[sp+8]` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x12b080 | `_timeoutDisable 0x12abc0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12b085 | `_timeoutCancel 0x12aba0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12b090 | `___errno 0x1711d0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12b037 | `(ret(_setjmp) & ret(_setjmp)) == 0x0` | `(ret(_setjmp) & ret(_setjmp)) != 0x0` |
| `jne` at 0x12b063 | `(ret(_timeoutSet) & ret(_timeoutSet)) != 0x0` | `(ret(_timeoutSet) & ret(_timeoutSet)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12b0a6 | `eax` | `?` |

## `_identPrintTable`

- `_identPrintTable`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122d28 to 0x122de8.
  Established. The `_lstFirst` site has candidate zero `0x1d4d68` (`call` at 0x122d31). The
  `_identPrint` call has unknown candidate values (`call` at 0x122d51). The final `_lstCount` call
  again uses candidate zero `0x1d4d68` (`call` at 0x122d6d), and its result is the return expression
  (`ret` at 0x122d78).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x122d38 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122d3c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122d72 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122d31 | `_lstFirst 0x190308` | `0x1d4d68` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x122d45 | `_ffscMsg 0x11f040` | `0x122d11` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122d51 | `_identPrint 0x1231a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122d5a | `_lstNext 0x1903b8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122d6d | `_lstCount 0x190218` | `0x1d4d68` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x122d66 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122d78 | `eax` | `ret(_lstCount)` |

## `_cmdREBOOT_FFSC`

- `_cmdREBOOT_FFSC`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x118430 to 0x1184d0.
  Established. The initial branch is taken when `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` (`jg` at
  0x118441). A later conditional four-byte store writes `0x1` to absolute `0x1ee21c` (`mov` at
  0x11847e). A further four-byte store writes `0xb` through an unknown effective address (`mov` at
  0x1184aa). No reboot call is visible in this span.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x118433 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x118436 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x118439 | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x118448 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x11844d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x118454 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x118470 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x118476 | load | 1 | `([0x1f0c04] + [sp+12]) +0x4` | `-` |
| `mov` at 0x11847e | store | 4 | `abs 0x1ee21c` | `0x1` |
| `mov` at 0x11848e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x118495 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11849e | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1184a3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1184aa | store | 4 | `(unknown)` | `0xb` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x118458 | `_sprintf 0x172dd8` | `?` | `0x116e80` | `ebp` | `?` | `?` | `?` |
| `call` at 0x118499 | `_sprintf 0x172dd8` | `?` | `0x11840c` | `[0x1f0c04]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x118441 | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `je` at 0x11847c | `(([([0x1f0c04] + [sp+12])+4]:1 & 0xff) & ([([0x1f0c04] + [sp+12])+4]:1 & 0xff)) == 0x0` | `(([([0x1f0c04] + [sp+12])+4]:1 & 0xff) & ([([0x1f0c04] + [sp+12])+4]:1 & 0xff)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x118462 | `eax` | `0x0` |
| `ret` at 0x1184b9 | `eax` | `0x1` |

## `_cmdRESET_NVRAM`

- `_cmdRESET_NVRAM`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1184d0 to 0x118590.
  Established. The first branch uses `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` as its taken predicate
  (`jg` at 0x1184e1). The later `_nvramReset` site has candidate zero `ebp`, with unknown remaining
  candidates (`call` at 0x11851d). A subsequent four-byte store has unknown effective address and
  value `0xb` (`mov` at 0x11854b). The caller record roles remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1184d3 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1184d6 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1184d9 | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x1184e8 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1184ed | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1184f4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x118510 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x118515 | load | 1 | `([0x1f0c04] + [sp+12]) +0x4` | `-` |
| `mov` at 0x118524 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x11852f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x118536 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11853f | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x118544 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11854b | store | 4 | `(unknown)` | `0xb` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1184f8 | `_sprintf 0x172dd8` | `?` | `0x116e80` | `ebp` | `?` | `?` | `?` |
| `call` at 0x11851d | `_nvramReset 0x1271a8` | `ebp` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11853a | `_sprintf 0x172dd8` | `?` | `0x1184ba` | `[0x1f0c04]` | `0x5` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x1184e1 | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `je` at 0x11851b | `(([([0x1f0c04] + [sp+12])+4]:1 & 0xff) & ([([0x1f0c04] + [sp+12])+4]:1 & 0xff)) == 0x0` | `(([([0x1f0c04] + [sp+12])+4]:1 & 0xff) & ([([0x1f0c04] + [sp+12])+4]:1 & 0xff)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x118502 | `eax` | `0x0` |
| `ret` at 0x11855a | `eax` | `0x1` |
