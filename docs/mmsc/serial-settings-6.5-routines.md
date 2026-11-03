# MMSC serial settings in IRIX 6.5

Read from `6.5/usr/cpu/firmware/mmscfw.bin` (IRIX 6.5).
Image SHA-256: `2017f743fb7360a160f4252b69569b7f9713d9efad82719cfb2fd82538e37ff8`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred end and can include literals
after returns. Statements below concern sites reached by known control-flow edges.
No unresolved computed transfers were observed in these routine spans.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/serial-settings-6.5-facts.txt) include all tool sections.

The decoder leaves raw literal bytes at three span ends:

| Span | Raw-byte start | Exclusive end | Bytes |
|---|---|---|---|
| `_comSetRxBuf` | `0x12cf91` | `0x12cf94` | `25 64 00` |
| `_comSetFlowCntl` | `0x12ce90` | `0x12ce94` | `00 90 90 90` |
| `_ttyGetTxBuf` | `0x12d1c2` | `0x12d1c4` | `00 90` |

## `_comGetInfo`

- `_comGetInfo`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12cbb4 to 0x12cbd4.
  Established. A four-byte load reads `sp +0x4` (`mov` at 0x12cbb7). EAX is unknown at the return
  (`ret` at 0x12cbc5). The calculated record address is not resolved by these facts.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12cbb7 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12cbc5 | `eax` | `?` |

## `_comGetTxBuf`

- `_comGetTxBuf`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12cbf4 to 0x12cc14.
  Established. The entry four-byte load reads `sp +0x4` (`mov` at 0x12cbf7). Calling `_ttyGetTxBuf`
  presents unknown candidate values (`call` at 0x12cbfc). The return carries `ret(_ttyGetTxBuf)` in
  EAX (`ret` at 0x12cc04). The adjusted selection value and buffer meaning remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12cbf7 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12cbfc | `_ttyGetTxBuf 0x12d0d4` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12cc04 | `eax` | `ret(_ttyGetTxBuf)` |

## `_comGetRxBuf`

- `_comGetRxBuf`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12cbd4 to 0x12cbf4.
  Established. Reads four bytes from `sp +0x4` (`mov` at 0x12cbd7). `_ttyGetRxBuf` is called with
  unknown candidate slots (`call` at 0x12cbdc). EAX returns `ret(_ttyGetRxBuf)` (`ret` at 0x12cbe4).
  The argument adjustment is not recovered in the tool facts.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12cbd7 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12cbdc | `_ttyGetRxBuf 0x12d084` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12cbe4 | `eax` | `ret(_ttyGetRxBuf)` |

## `_comSetFlag`

- `_comSetFlag`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12cc84 to 0x12cd44.
  Established. A two-byte OR site reads and writes an unknown effective target, with unknown stored
  value (`or` at 0x12cc95). `_nvramWrite` has leading candidates `0x4` and `0x1df928` (`call` at
  0x12cca3). The later `_cmCheckTTYFlags` call has unknown candidates (`call` at 0x12ccbc). The loop
  branch takes `le flags condition (operands unknown)` and falls through for its negation (`jle` at
  0x12ccd0). The affected flag record remains unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12cc8a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12cc91 | load | 2 | `sp +0x8` | `-` |
| `or` at 0x12cc95 | load | 2 | `(unknown)` | `-` |
| `or` at 0x12cc95 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x12ccb4 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12cca3 | `_nvramWrite 0x128b18` | `0x4` | `0x1df928` | `?` | `?` | `?` | `?` |
| `call` at 0x12ccbc | `_cmCheckTTYFlags 0x11b0e0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x12ccd0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12cce5 | `eax` | `?` |

## `_comResetFlag`

- `_comResetFlag`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12cc14 to 0x12cc84.
  Established. A two-byte AND site has unknown effective target and stored value (`and` at
  0x12cc28). The `_nvramWrite` candidates begin `0x4` and `0x1df928` (`call` at 0x12cc36). Calling
  `_cmCheckTTYFlags` presents unknown candidates (`call` at 0x12cc50); its later loop decision takes
  `le flags condition (operands unknown)` and falls through for the negation (`jle` at 0x12cc64).
  The cleared flag identity and cumulative result remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12cc1a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12cc21 | load | 2 | `sp +0x8` | `-` |
| `and` at 0x12cc28 | load | 2 | `(unknown)` | `-` |
| `and` at 0x12cc28 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x12cc48 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12cc36 | `_nvramWrite 0x128b18` | `0x4` | `0x1df928` | `?` | `?` | `?` | `?` |
| `call` at 0x12cc50 | `_cmCheckTTYFlags 0x11b0e0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x12cc64 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12cc79 | `eax` | `?` |

## `_comSetFunction`

- `_comSetFunction`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12ce94 to 0x12cf34.
  Established. The initial two-byte store writes `[sp+8]:2` to an unknown target (`mov` at 0x12ceaf).
  The two-byte store of `0xffff` at an unknown target (`mov` at 0x12cee9) is reached only
  through the fall-through arms of `jne` at 0x12cee2 and `je` at 0x12cee7.
  The loop branch takes `? <= 0x5 (signed, 32-bit)` and falls through for greater values (`jle` at
  0x12cf0e). `_nvramWrite` presents leading candidates `0x4` and `0x1df928` (`call` at 0x12cf17).
  EAX reaches the return as `0x1` (`ret` at 0x12cf2a). The function-assignment semantics remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12ce9d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12cea0 | load | 2 | `sp +0x8` | `-` |
| `mov` at 0x12cea7 | store | 4 | `sp -0x8` | `[sp+4]-1` |
| `mov` at 0x12ceaf | store | 2 | `(unknown)` | `[sp+8]:2` |
| `test` at 0x12ceb6 | load | 1 | `abs 0x1df500` | `-` |
| `cmp` at 0x12cedd | load | 2 | `(unknown)` | `-` |
| `cmp` at 0x12cee4 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x12cee9 | store | 2 | `(unknown)` | `0xffff` |
| `test` at 0x12cef0 | load | 1 | `abs 0x1df500` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12cec9 | `_ffscMsg 0x120504` | `0x12ce54` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x12cf02 | `_ffscMsg 0x120504` | `0x12ce70` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12cf17 | `_nvramWrite 0x128b18` | `0x4` | `0x1df928` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12cebd | `(([0x1df500]:1 & 0xff) & 0x4) == 0x0` | `(([0x1df500]:1 & 0xff) & 0x4) != 0x0` |
| `jne` at 0x12cee2 | `Z == 0` | `not (Z == 0)` |
| `je` at 0x12cee7 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x12cef7 | `(([0x1df500]:1 & 0xff) & 0x4) == 0x0` | `(([0x1df500]:1 & 0xff) & 0x4) != 0x0` |
| `jle` at 0x12cf0e | `? <= 0x5 (signed, 32-bit)` | `? > 0x5 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12cf2a | `eax` | `0x1` |

## `_comSetSpeed`

- `_comSetSpeed`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12cf94 to 0x12d004.
  Established. The `_ttySetSpeed` call has leading candidates unknown and `[sp+8]` (`call` at
  0x12cfac); its branch takes `(ret(_ttySetSpeed) & ret(_ttySetSpeed)) != 0x0` and falls through for
  zero (`jne` at 0x12cfb6). The fall-through includes a four-byte store of `[sp+8]` at an unknown
  effective target (`mov` at 0x12cfbd), followed by `_nvramWrite` with leading candidates `0x4` and
  `0x1df928` (`call` at 0x12cfca). The selected descriptor and rate units are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12cf9a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12cf9d | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12cfa4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12cfbd | store | 4 | `(unknown)` | `[sp+8]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12cfac | `_ttySetSpeed 0x12d4d4` | `?` | `[sp+8]` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x12cfca | `_nvramWrite 0x128b18` | `0x4` | `0x1df928` | `?` | `?` | `?` | `?` |
| `call` at 0x12cfeb | `_ffscMsgS 0x120634` | `0x12cf6d` | `[sp+8]` | `[sp+4]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x12cfb6 | `(ret(_ttySetSpeed) & ret(_ttySetSpeed)) != 0x0` | `(ret(_ttySetSpeed) & ret(_ttySetSpeed)) == 0x0` |
| `jne` at 0x12cfd1 | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12cffe | `eax` | `?` |

## `_comSetRxBuf`

- `_comSetRxBuf`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12cf34 to 0x12cf94.
  Established. The four-byte store writes `[sp+8]` to an unknown effective target (`mov` at
  0x12cf41). `_nvramWrite` presents leading candidates `0x4` and `0x1df928` (`call` at 0x12cf4e).
  The result branch takes `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` and falls through for zero
  (`jne` at 0x12cf55). The two routes return EAX `0xffffffff` (`ret` at 0x12cf6c) and `0x0` (`ret`
  at 0x12cf5c), respectively. The stored buffer-field interpretation is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12cf37 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12cf3a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12cf41 | store | 4 | `(unknown)` | `[sp+8]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12cf4e | `_nvramWrite 0x128b18` | `0x4` | `0x1df928` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x12cf55 | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12cf5c | `eax` | `0x0` |
| `ret` at 0x12cf6c | `eax` | `0xffffffff` |

## `_comSetTxBuf`

- `_comSetTxBuf`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12d004 to 0x12d044.
  Established. Stores four-byte `[sp+8]` at an unknown effective target (`mov` at 0x12d011). The
  `_nvramWrite` candidates begin `0x4` and `0x1df928` (`call` at 0x12d01e); its branch takes
  `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` and falls through for zero (`jne` at 0x12d025). The
  taken exit carries EAX `0xffffffff` (`ret` at 0x12d03c), while the fall-through exit carries `0x0`
  (`ret` at 0x12d02c). The transmit-setting role remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12d007 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12d00a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12d011 | store | 4 | `(unknown)` | `[sp+8]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12d01e | `_nvramWrite 0x128b18` | `0x4` | `0x1df928` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x12d025 | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12d02c | `eax` | `0x0` |
| `ret` at 0x12d03c | `eax` | `0xffffffff` |

## `_comSetFlowCntl`

- `_comSetFlowCntl`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12cd44 to 0x12ce94.
  Established. The first `_ioctl` candidates begin unknown, `0x1006`, and `sp-8` (`call` at
  0x12cd63). A later branch takes `([sp+8] & [sp+8]) == 0x0` and falls through for nonzero (`je` at
  0x12cd86). The taken route calls `_comResetFlag` with leading candidates `[sp+4]+1` and `0x800`
  (`call` at 0x12cdb5), while the fall-through calls `_comSetFlag` with the same leading candidates
  (`call` at 0x12cd99). The merged `_ioctl` candidates then begin unknown, `0x1005`, and `[sp-8]`
  (`call` at 0x12cdce). The flow-control register and option meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12cd4c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12cd4f | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12cd5b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12cd6f | load | 4 | `(unknown)` | `-` |
| `and` at 0x12cd88 | load | 4 | `sp -0x8` | `-` |
| `and` at 0x12cd88 | store | 4 | `sp -0x8` | `?` |
| `or` at 0x12cd8c | load | 1 | `sp -0x8` | `-` |
| `or` at 0x12cd8c | store | 1 | `sp -0x8` | `?` |
| `or` at 0x12cda4 | load | 1 | `sp -0x8` | `-` |
| `or` at 0x12cda4 | store | 1 | `sp -0x8` | `?` |
| `and` at 0x12cda8 | load | 4 | `sp -0x8` | `-` |
| `and` at 0x12cda8 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x12cdbd | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x12cdc6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12cdde | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12ce0e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12ce34 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12cd63 | `_ioctl 0x18bb08` | `?` | `0x1006` | `sp-8` | `ebx` | `esi` | `?` |
| `call` at 0x12cd99 | `_comSetFlag 0x12cc84` | `[sp+4]+1` | `0x800` | `?` | `?` | `?` | `?` |
| `call` at 0x12cdb5 | `_comResetFlag 0x12cc14` | `[sp+4]+1` | `0x800` | `?` | `?` | `?` | `?` |
| `call` at 0x12cdce | `_ioctl 0x18bb08` | `?` | `0x1005` | `[sp-8]` | `?` | `?` | `?` |
| `call` at 0x12cde6 | `_ioctl 0x18bb08` | `?` | `0x13` | `0x0` | `?` | `?` | `?` |
| `call` at 0x12ce16 | `_ioctl 0x18bb08` | `?` | `0x3` | `?` | `?` | `?` | `?` |
| `call` at 0x12ce41 | `_ffscMsg 0x120504` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x12cd6d | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x12cd86 | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |
| `jl` at 0x12cdd8 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x12cdf0 | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |
| `jl` at 0x12ce20 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12ce53 | `eax` | `?` |

## `_ttyReInit`

- `_ttyReInit`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12d474 to 0x12d4d4.
  Established. Calls `_sysSerialHwInit` with leading candidates entry `ebx`, `esi`, `edi`, and `ebp`
  (`call` at 0x12d47a). The loop `_ioctl` site has leading candidates unknown, `0x1007`, and `0x2`
  (`call` at 0x12d49b); its branch takes `(ret(_ioctl) & ret(_ioctl)) == 0x0` and falls through for
  nonzero (`je` at 0x12d4a5). EAX returns `0x0` after the loop (`ret` at 0x12d4cf). The descriptor
  sequence and reinitialization effects are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12d493 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12d4a7 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12d47a | `_sysSerialHwInit 0x10b618` | `ebx` | `esi` | `edi` | `ebp` | `?` | `?` |
| `call` at 0x12d49b | `_ioctl 0x18bb08` | `?` | `0x1007` | `0x2` | `?` | `?` | `?` |
| `call` at 0x12d4b4 | `_ffscMsg 0x120504` | `0x12d452` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12d4a5 | `(ret(_ioctl) & ret(_ioctl)) == 0x0` | `(ret(_ioctl) & ret(_ioctl)) != 0x0` |
| `jle` at 0x12d4c2 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12d4cf | `eax` | `0x0` |

## `_ttyGetInfo`

- `_ttyGetInfo`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12d044 to 0x12d084.
  Established. The unsigned branch takes `[sp+4] <= 0x5 (unsigned, 32-bit)` and falls through for
  larger values (`jbe` at 0x12d050). The fall-through returns EAX `0x0` (`ret` at 0x12d057). After the taken range branch, `je` at 0x12d066 takes
  `([sp+8] & [sp+8]) == 0x0` and falls through for nonzero; its fall-through reaches a four-byte
  store of an unknown value at `[sp+8] +0x0` (`mov` at 0x12d06f). The
  other return carries an unknown EAX value (`ret` at 0x12d07e). The selected information record
  remains unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12d047 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12d04a | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12d068 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12d06f | store | 4 | `[sp+8] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x12d050 | `[sp+4] <= 0x5 (unsigned, 32-bit)` | `[sp+4] > 0x5 (unsigned, 32-bit)` |
| `je` at 0x12d066 | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12d057 | `eax` | `0x0` |
| `ret` at 0x12d07e | `eax` | `?` |

## `_ttyGetRxBuf`

- `_ttyGetRxBuf`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12d084 to 0x12d0d4.
  Established. The range decision takes `[sp+4] <= 0x5 (unsigned, 32-bit)` and falls through for
  greater values (`jbe` at 0x12d08d). The fall-through returns EAX `0xffffffff` (`ret` at 0x12d097).
  A later branch takes `? == 0x0` and falls through for nonzero (`je` at 0x12d0b3); its taken and fall-through routes reach four-byte loads with unknown effective targets
  (`mov` at 0x12d0c9, `mov` at 0x12d0b5), respectively. The receive-buffer choice cannot be established from these merged facts.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12d087 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x12d0ae | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12d0b5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12d0c9 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x12d08d | `[sp+4] <= 0x5 (unsigned, 32-bit)` | `[sp+4] > 0x5 (unsigned, 32-bit)` |
| `je` at 0x12d0b3 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12d097 | `eax` | `0xffffffff` |
| `ret` at 0x12d0bc | `eax` | `?` |
| `ret` at 0x12d0d0 | `eax` | `?` |

## `_ttyGetTxBuf`

- `_ttyGetTxBuf`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12d0d4 to 0x12d1c4.
  Established. The first branch takes `[sp+4] <= 0x5 (unsigned, 32-bit)` and falls through for
  larger values (`jbe` at 0x12d0dd); its fall-through exit carries EAX `0xffffffff` (`ret` at
  0x12d0e7). The later branch takes `? == 0x0` and falls through for nonzero (`je` at 0x12d103); its
  taken and fall-through routes include four-byte loads with unknown effective targets (`mov` at
  0x12d119, `mov` at 0x12d105), respectively. The default transmit-buffer selection remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12d0d7 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x12d0fe | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12d105 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12d119 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x12d0dd | `[sp+4] <= 0x5 (unsigned, 32-bit)` | `[sp+4] > 0x5 (unsigned, 32-bit)` |
| `je` at 0x12d103 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12d0e7 | `eax` | `0xffffffff` |
| `ret` at 0x12d10c | `eax` | `?` |
| `ret` at 0x12d120 | `eax` | `?` |

## `_ttySetSpeed`

- `_ttySetSpeed`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12d4d4 to 0x12d4f8.
  Established. The `_ioctl` candidates begin `[sp+4]`, `0x4`, and `[sp+8]` (`call` at 0x12d4e1). The
  result branch takes `ge flags condition (operands unknown)` and falls through for its negation
  (`jge` at 0x12d4ea). EAX is unknown at the merged return (`ret` at 0x12d4f6). The rate encoding
  and runtime descriptor behavior remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12d4d7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12d4da | load | 4 | `sp +0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12d4e1 | `_ioctl 0x18bb08` | `[sp+4]` | `0x4` | `[sp+8]` | `ebp` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x12d4ea | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12d4f6 | `eax` | `?` |
