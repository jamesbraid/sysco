# MMSC serial configuration in patch SG0002378

Read from `patches/SG0002378/usr/cpu/firmware/mmscfw.bin` (patch SG0002378).
Image SHA-256: `b1249e58cf4f00c252362da8ed0515e25bb94aaf096bb60a5c5082c9b6e4d342`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred end and can include literals or padding
after returns. Statements below concern sites reached by known control-flow edges.
No unresolved jump transfers were reported in these spans.

Two spans end with bytes the decoder did not render as instructions.
These raw tails are outside the known control-flow paths. End addresses are exclusive.

| Span | Raw tail start | End | Bytes |
|---|---|---|---|
| `_comSetRxBuf` | `0x12b59d` | `0x12b5a0` | `25 64 00` |
| `_comSetFlowCntl` | `0x12b49c` | `0x12b4a0` | `00 90 90 90` |

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/serial-config-sg0002378-facts.txt) include all tool sections.

## `_comGetInfo`

- `_comGetInfo`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12b1c0 to 0x12b1e0.
  Established. The explicit four-byte load reads `sp +0x4` (`mov` at 0x12b1c3). The final `eax`
  value remains unknown in the fact sheet (`ret` at 0x12b1d1). No calls or conditional branches
  occur in the analysed span, so the name does not establish a record layout.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12b1c3 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12b1d1 | `eax` | `?` |

## `_comGetTxBuf`

- `_comGetTxBuf`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12b200 to 0x12b220.
  Established. The `_ttyGetTxBuf` site has unknown candidate values (`call` at 0x12b208); its result
  reaches the exit as `ret(_ttyGetTxBuf)` (`ret` at 0x12b210). The entry four-byte load is resolved
  to `sp +0x4` (`mov` at 0x12b203), but the call candidate is not resolved from it.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12b203 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12b208 | `_ttyGetTxBuf 0x12b6e0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12b210 | `eax` | `ret(_ttyGetTxBuf)` |

## `_comGetRxBuf`

- `_comGetRxBuf`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12b1e0 to 0x12b200.
  Established. The four-byte load reads `sp +0x4` (`mov` at 0x12b1e3). The subsequent `_ttyGetRxBuf`
  call retains unknown candidate zero and unknown remaining candidates (`call` at 0x12b1e8). The
  returned expression is `ret(_ttyGetRxBuf)` (`ret` at 0x12b1f0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12b1e3 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12b1e8 | `_ttyGetRxBuf 0x12b690` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12b1f0 | `eax` | `ret(_ttyGetRxBuf)` |

## `_comSetFlag`

- `_comSetFlag`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12b290 to 0x12b350.
  Established. The two-byte read-modify-write has unknown effective address and stored value (`or`
  at 0x12b2a1). The `_nvramWrite` site presents leading candidates `0x4` and `0x1dd0e0` (`call` at
  0x12b2af). The `_cmCheckTTYFlags` call retains unknown candidate values (`call` at 0x12b2c8); its
  loop branch exposes only `le flags condition (operands unknown)` as the taken predicate (`jle` at
  0x12b2dc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12b296 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12b29d | load | 2 | `sp +0x8` | `-` |
| `or` at 0x12b2a1 | load | 2 | `(unknown)` | `-` |
| `or` at 0x12b2a1 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x12b2c0 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12b2af | `_nvramWrite 0x127298` | `0x4` | `0x1dd0e0` | `?` | `?` | `?` | `?` |
| `call` at 0x12b2c8 | `_cmCheckTTYFlags 0x11a070` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x12b2dc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12b2f1 | `eax` | `?` |

## `_comResetFlag`

- `_comResetFlag`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12b220 to 0x12b290.
  Established. The two-byte `and` store remains unresolved in target and value (`and` at 0x12b234).
  The `_nvramWrite` call uses candidates `0x4` and `0x1dd0e0` (`call` at 0x12b242). A distinct
  `_cmCheckTTYFlags` site has unknown candidate values (`call` at 0x12b25c). No bit meaning is
  established by the routine name.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12b226 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12b22d | load | 2 | `sp +0x8` | `-` |
| `and` at 0x12b234 | load | 2 | `(unknown)` | `-` |
| `and` at 0x12b234 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x12b254 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12b242 | `_nvramWrite 0x127298` | `0x4` | `0x1dd0e0` | `?` | `?` | `?` | `?` |
| `call` at 0x12b25c | `_cmCheckTTYFlags 0x11a070` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x12b270 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12b285 | `eax` | `?` |

## `_comSetFunction`

- `_comSetFunction`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12b4a0 to 0x12b540.
  Established. A two-byte store has unknown target and value `[sp+8]:2` (`mov` at 0x12b4bb). The
  later two-byte store instead has value `0xffff`, again through an unknown effective address (`mov`
  at 0x12b4f5). The `_nvramWrite` site presents `0x4` and `0x1dd0e0` as leading candidates (`call`
  at 0x12b523).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12b4a9 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12b4ac | load | 2 | `sp +0x8` | `-` |
| `mov` at 0x12b4b3 | store | 4 | `sp -0x8` | `[sp+4]-1` |
| `mov` at 0x12b4bb | store | 2 | `(unknown)` | `[sp+8]:2` |
| `test` at 0x12b4c2 | load | 1 | `abs 0x1dccc0` | `-` |
| `cmp` at 0x12b4e9 | load | 2 | `(unknown)` | `-` |
| `cmp` at 0x12b4f0 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x12b4f5 | store | 2 | `(unknown)` | `0xffff` |
| `test` at 0x12b4fc | load | 1 | `abs 0x1dccc0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12b4d5 | `_ffscMsg 0x11f040` | `0x12b460` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x12b50e | `_ffscMsg 0x11f040` | `0x12b47c` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12b523 | `_nvramWrite 0x127298` | `0x4` | `0x1dd0e0` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12b4c9 | `(([0x1dccc0]:1 & 0xff) & 0x4) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x4) != 0x0` |
| `jne` at 0x12b4ee | `Z == 0` | `not (Z == 0)` |
| `je` at 0x12b4f3 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x12b503 | `(([0x1dccc0]:1 & 0xff) & 0x4) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x4) != 0x0` |
| `jle` at 0x12b51a | `? <= 0x5 (signed, 32-bit)` | `? > 0x5 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12b536 | `eax` | `0x1` |

## `_comSetSpeed`

- `_comSetSpeed`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12b5a0 to 0x12b610.
  Established. The `_ttySetSpeed` call has unknown candidate zero and candidate one `[sp+8]` (`call`
  at 0x12b5b8); its result branch falls through when `(ret(_ttySetSpeed) & ret(_ttySetSpeed)) ==
  0x0` (`jne` at 0x12b5c2). That path stores `[sp+8]` as four bytes through an unknown effective
  address (`mov` at 0x12b5c9), then reaches `_nvramWrite` with leading candidates `0x4` and
  `0x1dd0e0` (`call` at 0x12b5d6).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12b5a6 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12b5a9 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12b5b0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12b5c9 | store | 4 | `(unknown)` | `[sp+8]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12b5b8 | `_ttySetSpeed 0x12bb00` | `?` | `[sp+8]` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x12b5d6 | `_nvramWrite 0x127298` | `0x4` | `0x1dd0e0` | `?` | `?` | `?` | `?` |
| `call` at 0x12b5f7 | `_ffscMsgS 0x11f110` | `0x12b579` | `[sp+8]` | `[sp+4]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x12b5c2 | `(ret(_ttySetSpeed) & ret(_ttySetSpeed)) != 0x0` | `(ret(_ttySetSpeed) & ret(_ttySetSpeed)) == 0x0` |
| `jne` at 0x12b5dd | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12b60a | `eax` | `?` |

## `_comSetRxBuf`

- `_comSetRxBuf`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12b540 to 0x12b5a0.
  Established. The four-byte store has unknown effective address and value `[sp+8]` (`mov` at
  0x12b54d). The `_nvramWrite` site presents candidates `0x4` and `0x1dd0e0` (`call` at 0x12b55a);
  its nonzero-result branch is taken (`jne` at 0x12b561), reaching the exit with `0xffffffff` (`ret`
  at 0x12b578). The other exit reports `0x0` (`ret` at 0x12b568).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12b543 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12b546 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12b54d | store | 4 | `(unknown)` | `[sp+8]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12b55a | `_nvramWrite 0x127298` | `0x4` | `0x1dd0e0` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x12b561 | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12b568 | `eax` | `0x0` |
| `ret` at 0x12b578 | `eax` | `0xffffffff` |

## `_comSetTxBuf`

- `_comSetTxBuf`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12b610 to 0x12b650.
  Established. The `_nvramWrite` call has leading candidates `0x4` and `0x1dd0e0` (`call` at
  0x12b62a). It follows an unresolved four-byte store with value `[sp+8]` (`mov` at 0x12b61d). The
  result branch is taken when `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` (`jne` at 0x12b631).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12b613 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12b616 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12b61d | store | 4 | `(unknown)` | `[sp+8]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12b62a | `_nvramWrite 0x127298` | `0x4` | `0x1dd0e0` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x12b631 | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12b638 | `eax` | `0x0` |
| `ret` at 0x12b648 | `eax` | `0xffffffff` |

## `_comSetFlowCntl`

- `_comSetFlowCntl`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12b350 to 0x12b4a0.
  Established. The `_ioctl` sites include candidate one `0x1006` with candidate two `sp-8` (`call`
  at 0x12b36f), `0x1005` with `[sp-8]` (`call` at 0x12b3da), `0x13` with `0x0` (`call` at 0x12b3f2),
  and `0x3` with unknown candidate two (`call` at 0x12b422). Candidate zero is unknown at all four
  sites. The branch is taken when `([sp+8] & [sp+8]) == 0x0` (`je` at 0x12b392), reaching
  `_comResetFlag` (`call` at 0x12b3c1); its nonzero fall-through reaches `_comSetFlag` (`call` at
  0x12b3a5).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12b358 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12b35b | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12b367 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12b37b | load | 4 | `(unknown)` | `-` |
| `and` at 0x12b394 | load | 4 | `sp -0x8` | `-` |
| `and` at 0x12b394 | store | 4 | `sp -0x8` | `?` |
| `or` at 0x12b398 | load | 1 | `sp -0x8` | `-` |
| `or` at 0x12b398 | store | 1 | `sp -0x8` | `?` |
| `or` at 0x12b3b0 | load | 1 | `sp -0x8` | `-` |
| `or` at 0x12b3b0 | store | 1 | `sp -0x8` | `?` |
| `and` at 0x12b3b4 | load | 4 | `sp -0x8` | `-` |
| `and` at 0x12b3b4 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x12b3c9 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x12b3d2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12b3ea | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12b41a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12b440 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12b36f | `_ioctl 0x189340` | `?` | `0x1006` | `sp-8` | `ebx` | `esi` | `?` |
| `call` at 0x12b3a5 | `_comSetFlag 0x12b290` | `[sp+4]+1` | `0x800` | `?` | `?` | `?` | `?` |
| `call` at 0x12b3c1 | `_comResetFlag 0x12b220` | `[sp+4]+1` | `0x800` | `?` | `?` | `?` | `?` |
| `call` at 0x12b3da | `_ioctl 0x189340` | `?` | `0x1005` | `[sp-8]` | `?` | `?` | `?` |
| `call` at 0x12b3f2 | `_ioctl 0x189340` | `?` | `0x13` | `0x0` | `?` | `?` | `?` |
| `call` at 0x12b422 | `_ioctl 0x189340` | `?` | `0x3` | `?` | `?` | `?` | `?` |
| `call` at 0x12b44d | `_ffscMsg 0x11f040` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x12b379 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x12b392 | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |
| `jl` at 0x12b3e4 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x12b3fc | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |
| `jl` at 0x12b42c | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12b45f | `eax` | `?` |

## `_ttyGetRxBuf`

- `_ttyGetRxBuf`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12b690 to 0x12b6e0.
  Established. The range branch is taken when `[sp+4] <= 0x5 (unsigned, 32-bit)` (`jbe` at
  0x12b699). The three indexed four-byte accesses retain unknown effective addresses (`cmp` at
  0x12b6ba, `mov` at 0x12b6c1, `mov` at 0x12b6d5). Only the out-of-range exit has a resolved return
  value, `0xffffffff` (`ret` at 0x12b6a3).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12b693 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x12b6ba | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12b6c1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12b6d5 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x12b699 | `[sp+4] <= 0x5 (unsigned, 32-bit)` | `[sp+4] > 0x5 (unsigned, 32-bit)` |
| `je` at 0x12b6bf | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12b6a3 | `eax` | `0xffffffff` |
| `ret` at 0x12b6c8 | `eax` | `?` |
| `ret` at 0x12b6dc | `eax` | `?` |

## `_ttyReInit`

- `_ttyReInit`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12baa0 to 0x12bb00.
  Established. The `_sysSerialHwInit` call is visible (`call` at 0x12baa6), followed later by
  `_ioctl` with unknown candidate zero, candidate one `0x1007`, and candidate two `0x2` (`call` at
  0x12bac7). The branch bypasses `_ffscMsg` when `(ret(_ioctl) & ret(_ioctl)) == 0x0` (`je` at
  0x12bad1). Both indexed four-byte loads have unknown targets (`mov` at 0x12babf, `mov` at
  0x12bad3).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12babf | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12bad3 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12baa6 | `_sysSerialHwInit 0x10b618` | `ebx` | `esi` | `edi` | `ebp` | `?` | `?` |
| `call` at 0x12bac7 | `_ioctl 0x189340` | `?` | `0x1007` | `0x2` | `?` | `?` | `?` |
| `call` at 0x12bae0 | `_ffscMsg 0x11f040` | `0x12ba7e` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12bad1 | `(ret(_ioctl) & ret(_ioctl)) == 0x0` | `(ret(_ioctl) & ret(_ioctl)) != 0x0` |
| `jle` at 0x12baee | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12bafb | `eax` | `0x0` |

## `_ttyGetInfo`

- `_ttyGetInfo`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12b650 to 0x12b690.
  Established. The first branch is taken when `[sp+4] <= 0x5 (unsigned, 32-bit)` (`jbe` at
  0x12b65c). A later four-byte store through `[sp+8] +0x0` has unknown value (`mov` at 0x12b67b).
  The intervening zero-input branch bypasses that store when `([sp+8] & [sp+8]) == 0x0` (`je` at
  0x12b672).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12b653 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12b656 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12b674 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12b67b | store | 4 | `[sp+8] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x12b65c | `[sp+4] <= 0x5 (unsigned, 32-bit)` | `[sp+4] > 0x5 (unsigned, 32-bit)` |
| `je` at 0x12b672 | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12b663 | `eax` | `0x0` |
| `ret` at 0x12b68a | `eax` | `?` |

## `_ttySetSpeed`

- `_ttySetSpeed`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12bb00 to 0x12bb24.
  Established. The `_ioctl` call has leading candidates `[sp+4]`, `0x4`, and `[sp+8]` (`call` at
  0x12bb0d); its following branch is taken under `ge flags condition (operands unknown)` (`jge` at
  0x12bb16). The merged return expression stays unknown (`ret` at 0x12bb22).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12bb03 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12bb06 | load | 4 | `sp +0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12bb0d | `_ioctl 0x189340` | `[sp+4]` | `0x4` | `[sp+8]` | `ebp` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x12bb16 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12bb22 | `eax` | `?` |

## `_ttyInit`

- `_ttyInit`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12b7d0 to 0x12baa0.
  Established. The `_nvramRead` site uses candidates `0x4` and `0x1dd0e0` (`call` at 0x12b805); its
  nonzero-result fall-through (`je` at 0x12b80f) reaches `_bzero` with candidates `0x1dd0e0` and
  `0x60` (`call` at 0x12b818). The later `_open` site presents `[sp-28]`, `0x2`, and `0x0` (`call`
  at 0x12b958). Indexed stores include unknown-target writes of `0xffffffff` (`mov` at 0x12b8e8,
  `mov` at 0x12ba18).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12b7d9 | store | 4 | `sp -0x20` | `0x0` |
| `mov` at 0x12b7f3 | load | 4 | `ret(___stderr) +0x0` | `-` |
| `mov` at 0x12b82c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12b832 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x12b838 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12b83c | store | 4 | `(unknown)` | `?` |
| `mov` at 0x12b840 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12b844 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x12b848 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12b84c | store | 4 | `(unknown)` | `?` |
| `mov` at 0x12b850 | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x12b858 | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x12b868 | store | 4 | `sp -0x20` | `0x1` |
| `cmp` at 0x12b881 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x12b889 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x12b88e | store | 2 | `(unknown)` | `?` |
| `test` at 0x12b8a0 | load | 1 | `(unknown)` | `-` |
| `or` at 0x12b8a7 | load | 2 | `(unknown)` | `-` |
| `or` at 0x12b8a7 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x12b8ae | load | 2 | `(unknown)` | `-` |
| `mov` at 0x12b8b3 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x12b8b8 | store | 4 | `sp -0x20` | `0x1` |
| `cmp` at 0x12b8c5 | load | 4 | `sp -0x20` | `-` |
| `mov` at 0x12b8e8 | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x12b900 | store | 4 | `sp -0x1c` | `sp-24` |
| `mov` at 0x12b90e | load | 4 | `sp -0x1c` | `-` |
| `mov` at 0x12b954 | load | 4 | `sp -0x1c` | `-` |
| `mov` at 0x12b960 | store | 4 | `(unknown)` | `ret(_open)` |
| `mov` at 0x12b967 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12b97d | load | 4 | `ret(___stderr) +0x0` | `-` |
| `mov` at 0x12b990 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12b997 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12b9ab | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12b9b2 | load | 4 | `sp -0x1c` | `-` |
| `cmp` at 0x12b9c2 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x12b9d4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12b9dc | store | 4 | `sp -0x24` | `?` |
| `mov` at 0x12b9e7 | load | 4 | `sp -0x24` | `-` |
| `mov` at 0x12b9ef | load | 4 | `sp -0x1c` | `-` |
| `mov` at 0x12b9fd | load | 4 | `ret(___stderr) +0x0` | `-` |
| `mov` at 0x12ba08 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12ba18 | store | 4 | `(unknown)` | `0xffffffff` |
| `cmp` at 0x12ba30 | load | 2 | `(unknown)` | `-` |
| `movsx` at 0x12ba3a | load | 2 | `(unknown)` | `-` |
| `mov` at 0x12ba41 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12ba48 | store | 4 | `(unknown)` | `?` |
| `test` at 0x12ba51 | load | 1 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12b7e0 | `_ttyDrv 0x19334c` | `ebx` | `esi` | `edi` | `?` | `0x0` | `?` |
| `call` at 0x12b7ee | `___stderr 0x18e17c` | `0x12b72d` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12b7f6 | `_fprintf 0x17c4ec` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12b805 | `_nvramRead 0x127108` | `0x4` | `0x1dd0e0` | `?` | `?` | `?` | `?` |
| `call` at 0x12b818 | `_bzero 0x172678` | `0x1dd0e0` | `0x60` | `?` | `?` | `?` | `?` |
| `call` at 0x12b8d2 | `_nvramWrite 0x127298` | `0x4` | `0x1dd0e0` | `?` | `?` | `?` | `?` |
| `call` at 0x12b912 | `_sprintf 0x172dd8` | `?` | `0x12b74b` | `?` | `?` | `?` | `?` |
| `call` at 0x12b91b | `_ttyGetTxBuf 0x12b6e0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12b925 | `_ttyGetRxBuf 0x12b690` | `?` | `ret(_ttyGetTxBuf)` | `?` | `?` | `?` | `?` |
| `call` at 0x12b92f | `_sysSerialChanGet 0x10b7c8` | `?` | `ret(_ttyGetRxBuf)` | `?` | `?` | `?` | `?` |
| `call` at 0x12b939 | `_ttyDevCreate 0x1933ac` | `?` | `ret(_sysSerialChanGet)` | `?` | `?` | `?` | `?` |
| `call` at 0x12b958 | `_open 0x1890e0` | `[sp-28]` | `0x2` | `0x0` | `?` | `?` | `?` |
| `call` at 0x12b978 | `___stderr 0x18e17c` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12b980 | `_fprintf 0x17c4ec` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12b99f | `_ttySetSpeed 0x12bb00` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12b9df | `_ioctl 0x189340` | `?` | `0x3` | `?` | `?` | `?` | `?` |
| `call` at 0x12b9f8 | `___stderr 0x18e17c` | `?` | `[sp-28]` | `?` | `?` | `?` | `?` |
| `call` at 0x12ba00 | `_fprintf 0x17c4ec` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12ba10 | `_close 0x189280` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12ba5d | `_comSetFlowCntl 0x12b350` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12b7e7 | `(ret(_ttyDrv) & ret(_ttyDrv)) == 0x0` | `(ret(_ttyDrv) & ret(_ttyDrv)) != 0x0` |
| `je` at 0x12b80f | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` |
| `jle` at 0x12b866 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jl` at 0x12b887 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jne` at 0x12b8a5 | `Z == 0` | `not (Z == 0)` |
| `jle` at 0x12b8c3 | `? <= 0x5 (signed, 32-bit)` | `? > 0x5 (signed, 32-bit)` |
| `je` at 0x12b8c9 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `jle` at 0x12b8f9 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `je` at 0x12b943 | `(ret(_ttyDevCreate) & ret(_ttyDevCreate)) == 0x0` | `(ret(_ttyDevCreate) & ret(_ttyDevCreate)) != 0x0` |
| `jge` at 0x12b970 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x12b9a9 | `(ret(_ttySetSpeed) & ret(_ttySetSpeed)) == 0x0` | `(ret(_ttySetSpeed) & ret(_ttySetSpeed)) != 0x0` |
| `jne` at 0x12b9ca | `Z == 0` | `not (Z == 0)` |
| `jge` at 0x12b9ec | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jl` at 0x12ba38 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x12ba58 | `Z == 1` | `not (Z == 1)` |
| `jle` at 0x12ba6c | `? <= 0x5 (signed, 32-bit)` | `? > 0x5 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12ba7d | `eax` | `0x0` |
