# MMSC keyboard and console callers in the 6.5.7 image

Read from `6.5.7/usr/cpu/firmware/mmscfw.bin`.
Image SHA-256: `6c3db7ffd7fb4cc43ed02146f0c5bea0e85338def2530689c9990f877f2ddedc`.
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

[Fact sheets and complete listings](disasm/keyboard-console-657-facts.txt) include all tool sections.

The `_pcConDrvIoctl` linear decoder stops at `0x108bc2`. The retained facts include
the raw remainder through its inferred end at `0x108e08`. Bytes after its known
return are not attributed code effects. The computed selector remains unresolved,
and effects reachable only through it are excluded from the entry tables.

## `_pcConDrv`

- `_pcConDrv`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108858 to 0x1088b8.
  Established. The entry branch takes its call arm when `[0x1d87d8] <= 0x0 (signed, 32-bit)` (`jle`
  at 0x108862). `_iosDrvInstall` carries the candidate prefix `0x108988`, `0x0`, `0x108988`, `0x0`,
  `0x1739c0`, `0x1738e0` (`call` at 0x10889a); its result is stored as four bytes at absolute
  `0x1d87d8` (`mov` at 0x10889f). Those numeric candidates do not establish calls through them.
  Driver registration effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10885b | load | 4 | `abs 0x1d87d8` | `-` |
| `mov` at 0x10889f | store | 4 | `abs 0x1d87d8` | `ret(_iosDrvInstall)` |
| `cmp` at 0x1088a6 | load | 4 | `abs 0x1d87d8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108878 | `_pcConDrvHrdInit 0x108958` | `ebp` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10889a | `_iosDrvInstall 0x174620` | `0x108988` | `0x0` | `0x108988` | `0x0` | `0x1739c0` | `0x1738e0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x108862 | `[0x1d87d8] <= 0x0 (signed, 32-bit)` | `[0x1d87d8] > 0x0 (signed, 32-bit)` |
| `jne` at 0x1088ad | `[0x1d87d8] != 0xffffffff` | `[0x1d87d8] == 0xffffffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108869 | `eax` | `0x0` |
| `ret` at 0x1088b7 | `eax` | `?` |

## `_pcConDrvHrdInit`

- `_pcConDrvHrdInit`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108958 to 0x108988.
  Established. The unbranched sequence includes `_intLock`, `_kbdHrdInit`, `_vgaHrdInit`, and
  `_intUnlock` (`call` at 0x10895c, `call` at 0x108963, `call` at 0x108968, `call` at 0x10896e). The
  final call has candidate zero `ret(_intLock)` (`call` at 0x10896e). The returned expression is
  `ret(_intUnlock)` (`ret` at 0x108979). No hardware initialization effect is proved inside this
  caller.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x108973 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10895c | `_intLock 0x175148` | `ebx` | `ebp` | `?` | `?` | `?` | `?` |
| `call` at 0x108963 | `_kbdHrdInit 0x10ad08` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108968 | `_vgaHrdInit 0x108ee8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10896e | `_intUnlock 0x175158` | `ret(_intLock)` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108979 | `eax` | `ret(_intUnlock)` |

## `_pcConDrvIoctl`

- `_pcConDrvIoctl`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108998 to 0x108e08.
  Established. The unsigned branch takes its fallback arm when `[sp+8]-1001 > 0x8 (unsigned,
  32-bit)` (`ja` at 0x1089b1). That arm calls `_tyIoctl` with candidates `[sp+4]`, `[sp+8]`,
  `[sp+12]` (`call` at 0x108aeb). The other arm reaches an unresolved computed jump (`jmp` at
  0x1089b7); its selector-only effects remain excluded, while the known return carries
  `ret(_tyIoctl)` (`ret` at 0x108afc). Command meanings and the full effect catalogue are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10899d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1089a0 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1089a3 | load | 4 | `sp +0xc` | `-` |
| `jmp` at 0x1089b7 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108aeb | `_tyIoctl 0x1735c0` | `[sp+4]` | `[sp+8]` | `[sp+12]` | `ebx` | `esi` | `ebp` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x1089b1 | `[sp+8]-1001 > 0x8 (unsigned, 32-bit)` | `[sp+8]-1001 <= 0x8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108afc | `eax` | `ret(_tyIoctl)` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x1089b7).

## `_pcConDevCreate`

- `_pcConDevCreate`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1088b8 to 0x108958.
  Established. The initial signed branch is taken when `[0x1d87d8] <= 0x0` (`jle` at 0x1088cb). That
  arm calls `_errnoSet` with candidate zero `0xc0001` (`call` at 0x10893d). The fall-through arm's
  unsigned branch is taken when `[sp+8] > 0x1` (`ja` at 0x1088d0). `_tyDevInit` has unknown
  candidate zero followed by `[sp+12]`, `[sp+16]`, `0x109798` (`call` at 0x108900). A later
  four-byte store writes `0x1` at an unresolved target (`mov` at 0x108916). Device identities and
  record roles are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1088be | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1088c1 | load | 4 | `sp +0xc` | `-` |
| `cmp` at 0x1088c4 | load | 4 | `abs 0x1d87d8` | `-` |
| `mov` at 0x1088eb | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1088fa | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x108916 | store | 4 | `(unknown)` | `0x1` |
| `mov` at 0x108920 | load | 4 | `abs 0x1d87d8` | `-` |
| `mov` at 0x108926 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108900 | `_tyDevInit 0x173370` | `?` | `[sp+12]` | `[sp+16]` | `0x109798` | `ebx` | `esi` |
| `call` at 0x10890e | `_sysIntEnablePIC 0x108428` | `0x1` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10892b | `_iosDevAdd 0x1747e0` | `?` | `[sp+4]` | `[0x1d87d8]` | `?` | `?` | `?` |
| `call` at 0x10893d | `_errnoSet 0x1750b8` | `0xc0001` | `ebx` | `esi` | `edi` | `ebp` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x1088cb | `[0x1d87d8] <= 0x0 (signed, 32-bit)` | `[0x1d87d8] > 0x0 (signed, 32-bit)` |
| `ja` at 0x1088d0 | `[sp+8] > 0x1 (unsigned, 32-bit)` | `[sp+8] <= 0x1 (unsigned, 32-bit)` |
| `jne` at 0x1088f3 | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `jne` at 0x10890a | `(ret(_tyDevInit) & ret(_tyDevInit)) != 0x0` | `(ret(_tyDevInit) & ret(_tyDevInit)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108950 | `eax` | `?` |

## `_kbdHrdInit`

- `_kbdHrdInit`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ad08 to 0x10adc8.
  Established. The `_wdCreate` result is written as four bytes at absolute `0x1d89f0` (`mov` at
  0x10ad14). `_kbdCommand` sites carry candidate zero `0x60`, `0x60`, and `0xffffffae` (`call` at
  0x10ad3c, `call` at 0x10ad76, `call` at 0x10ad94). `_kbdWrite` sites carry `0x44` and `0x45`
  (`call` at 0x10ad4b, `call` at 0x10ad85). The loop branch is taken when `[0x1aff00] <= 0x3
  (signed, 32-bit)` (`jle` at 0x10adac). Command and keyboard-controller meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10ad14 | store | 4 | `abs 0x1d89f0` | `ret(_wdCreate)` |
| `cmp` at 0x10ad19 | load | 4 | `abs 0x1aff00` | `-` |
| `cmp` at 0x10ad2c | load | 4 | `abs 0x1aff04` | `-` |
| `cmp` at 0x10ad9c | load | 4 | `abs 0x1aff04` | `-` |
| `cmp` at 0x10ada5 | load | 4 | `abs 0x1aff00` | `-` |
| `mov` at 0x10adb7 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ad0f | `_wdCreate 0x178d34` | `ebx` | `?` | `ebp` | `?` | `?` | `?` |
| `call` at 0x10ad35 | `_kbdReset 0x10b528` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ad3c | `_kbdCommand 0x10b488` | `0x60` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ad4b | `_kbdWrite 0x10b408` | `0x44` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ad5a | `_sysInByte 0x1081b0` | `0x64` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ad67 | `_kbdRead 0x10b378` | `sp-5` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ad76 | `_kbdCommand 0x10b488` | `0x60` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ad85 | `_kbdWrite 0x10b408` | `0x45` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ad94 | `_kbdCommand 0x10b488` | `0xffffffae` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10adb2 | `_kbdStatInit 0x10adc8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x10ad20 | `[0x1aff00] > 0x3 (signed, 32-bit)` | `[0x1aff00] <= 0x3 (signed, 32-bit)` |
| `je` at 0x10ad33 | `[0x1aff04] == 0x0` | `[0x1aff04] != 0x0` |
| `je` at 0x10ad47 | `ret(_kbdCommand) == 0xffffffff` | `ret(_kbdCommand) != 0xffffffff` |
| `je` at 0x10ad56 | `ret(_kbdWrite) == 0xffffffff` | `ret(_kbdWrite) != 0xffffffff` |
| `je` at 0x10ad64 | `(? & 0x1) == 0x0` | `(? & 0x1) != 0x0` |
| `jne` at 0x10ad72 | `ret(_kbdRead) != 0xffffffff` | `ret(_kbdRead) == 0xffffffff` |
| `je` at 0x10ad81 | `ret(_kbdCommand) == 0xffffffff` | `ret(_kbdCommand) != 0xffffffff` |
| `je` at 0x10ad90 | `ret(_kbdWrite) == 0xffffffff` | `ret(_kbdWrite) != 0xffffffff` |
| `je` at 0x10ada3 | `[0x1aff04] == 0x0` | `[0x1aff04] != 0x0` |
| `jle` at 0x10adac | `[0x1aff00] <= 0x3 (signed, 32-bit)` | `[0x1aff00] > 0x3 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10adbd | `eax` | `ret(_kbdStatInit)` |

## `_kbdReset`

- `_kbdReset`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b528 to 0x10b5f8.
  Established. The `_sysOutByte` sites carry pairs `0x60`, `0xffffffff` (`call` at 0x10b548),
  `0x60`, `0x60` (`call` at 0x10b57c), `0x60`, `0x45` (`call` at 0x10b5a0), and `0x64`, `0xffffffae`
  (`call` at 0x10b5c4). Five `_sysInByte` calls carry candidate zero `0x64` (`call` at 0x10b532,
  `call` at 0x10b566, `call` at 0x10b58a, `call` at 0x10b5ae, `call` at 0x10b5d2). `_taskDelay`
  retains unknown candidates (`call` at 0x10b559). Timing units and a reset outcome are not
  established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b5e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b532 | `_sysInByte 0x1081b0` | `0x64` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b548 | `_sysOutByte 0x1081d0` | `0x60` | `0xffffffff` | `?` | `?` | `?` | `?` |
| `call` at 0x10b550 | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b559 | `_taskDelay 0x176150` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b566 | `_sysInByte 0x1081b0` | `0x64` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b57c | `_sysOutByte 0x1081d0` | `0x60` | `0x60` | `?` | `?` | `?` | `?` |
| `call` at 0x10b58a | `_sysInByte 0x1081b0` | `0x64` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b5a0 | `_sysOutByte 0x1081d0` | `0x60` | `0x45` | `?` | `?` | `?` | `?` |
| `call` at 0x10b5ae | `_sysInByte 0x1081b0` | `0x64` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b5c4 | `_sysOutByte 0x1081d0` | `0x64` | `0xffffffae` | `?` | `?` | `?` | `?` |
| `call` at 0x10b5d2 | `_sysInByte 0x1081b0` | `0x64` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10b53c | `(? & 0x2) == 0x0` | `(? & 0x2) != 0x0` |
| `jle` at 0x10b542 | `? <= 0x63 (signed, 32-bit)` | `? > 0x63 (signed, 32-bit)` |
| `je` at 0x10b570 | `(? & 0x2) == 0x0` | `(? & 0x2) != 0x0` |
| `jle` at 0x10b576 | `? <= 0x63 (signed, 32-bit)` | `? > 0x63 (signed, 32-bit)` |
| `je` at 0x10b594 | `(? & 0x2) == 0x0` | `(? & 0x2) != 0x0` |
| `jle` at 0x10b59a | `? <= 0x63 (signed, 32-bit)` | `? > 0x63 (signed, 32-bit)` |
| `je` at 0x10b5b8 | `(? & 0x2) == 0x0` | `(? & 0x2) != 0x0` |
| `jle` at 0x10b5be | `? <= 0x63 (signed, 32-bit)` | `? > 0x63 (signed, 32-bit)` |
| `je` at 0x10b5dc | `(? & 0x2) == 0x0` | `(? & 0x2) != 0x0` |
| `jle` at 0x10b5e2 | `? <= 0x63 (signed, 32-bit)` | `? > 0x63 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b5ea | `eax` | `ret(_sysInByte)` |

## `_kbdCommand`

- `_kbdCommand`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b488 to 0x10b528.
  Established. A four-byte zero is stored at absolute `0x1aff04` (`mov` at 0x10b48f). `_wdStart` has
  candidate zero `[0x1d89f0]` and unknown remaining slots (`call` at 0x10b4b3). The first polling
  branch takes its forward arm when `(? & 0x2) == 0x0` (`je` at 0x10b4c8). The backward branch is
  taken when `[0x1aff04] == 0x0` (`je` at 0x10b4d1). `_sysOutByte` carries `0x64` and an unknown
  second candidate (`call` at 0x10b4d9). The command value and wait duration are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b48c | load | 1 | `sp +0x4` | `-` |
| `mov` at 0x10b48f | store | 4 | `abs 0x1aff04` | `0x0` |
| `imul` at 0x10b4a5 | load | 4 | `abs 0x1aff08` | `-` |
| `mov` at 0x10b4ad | load | 4 | `abs 0x1d89f0` | `-` |
| `cmp` at 0x10b4ca | load | 4 | `abs 0x1aff04` | `-` |
| `cmp` at 0x10b4f2 | load | 4 | `abs 0x1aff04` | `-` |
| `mov` at 0x10b4fb | load | 4 | `abs 0x1d89f0` | `-` |
| `cmp` at 0x10b508 | load | 4 | `abs 0x1aff04` | `-` |
| `mov` at 0x10b516 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b4a0 | `_sysClkRateGet 0x1085a8` | `0x10b5f8` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10b4b3 | `_wdStart 0x178f74` | `[0x1d89f0]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b4be | `_sysInByte 0x1081b0` | `0x64` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b4d9 | `_sysOutByte 0x1081d0` | `0x64` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b4e6 | `_sysInByte 0x1081b0` | `0x64` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b501 | `_wdCancel 0x179054` | `[0x1d89f0]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10b4c8 | `(? & 0x2) == 0x0` | `(? & 0x2) != 0x0` |
| `je` at 0x10b4d1 | `[0x1aff04] == 0x0` | `[0x1aff04] != 0x0` |
| `je` at 0x10b4f0 | `(? & 0x2) == 0x0` | `(? & 0x2) != 0x0` |
| `je` at 0x10b4f9 | `[0x1aff04] == 0x0` | `[0x1aff04] != 0x0` |
| `je` at 0x10b50f | `[0x1aff04] == 0x0` | `[0x1aff04] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b51c | `eax` | `?` |

## `_kbdWrite`

- `_kbdWrite`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b408 to 0x10b488.
  Established. The entry byte load reads `sp +0x4` (`mov` at 0x10b40c), before a four-byte zero
  assignment at absolute `0x1aff04` (`mov` at 0x10b40f). `_wdCancel` carries candidate zero
  `[0x1d89f0]` (`call` at 0x10b459). The later `_sysOutByte` site has candidate zero `0x60` and
  unknown candidate one (`call` at 0x10b467). The final branch is taken for `[0x1aff04] == 0x0`
  (`je` at 0x10b475), while the merged return expression remains unknown (`ret` at 0x10b482). Write
  completion semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b40c | load | 1 | `sp +0x4` | `-` |
| `mov` at 0x10b40f | store | 4 | `abs 0x1aff04` | `0x0` |
| `imul` at 0x10b425 | load | 4 | `abs 0x1aff08` | `-` |
| `mov` at 0x10b42d | load | 4 | `abs 0x1d89f0` | `-` |
| `cmp` at 0x10b44a | load | 4 | `abs 0x1aff04` | `-` |
| `mov` at 0x10b453 | load | 4 | `abs 0x1d89f0` | `-` |
| `cmp` at 0x10b46e | load | 4 | `abs 0x1aff04` | `-` |
| `mov` at 0x10b47c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b420 | `_sysClkRateGet 0x1085a8` | `0x10b5f8` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10b433 | `_wdStart 0x178f74` | `[0x1d89f0]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b43e | `_sysInByte 0x1081b0` | `0x64` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b459 | `_wdCancel 0x179054` | `[0x1d89f0]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b467 | `_sysOutByte 0x1081d0` | `0x60` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10b448 | `(? & 0x2) == 0x0` | `(? & 0x2) != 0x0` |
| `je` at 0x10b451 | `[0x1aff04] == 0x0` | `[0x1aff04] != 0x0` |
| `je` at 0x10b475 | `[0x1aff04] == 0x0` | `[0x1aff04] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b482 | `eax` | `?` |

## `_kbdRead`

- `_kbdRead`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b378 to 0x10b408.
  Established. The polling branch takes its forward arm when `(? & 0x1) != 0x0` (`jne` at 0x10b3b8);
  its alternative back edge is taken when `[0x1aff04] == 0x0` (`je` at 0x10b3c1). Two `_sysInByte`
  sites have candidate zero `0x64` and `0x60` respectively (`call` at 0x10b3ae, `call` at 0x10b3e4).
  A one-byte unknown value is stored at `[sp+4] +0x0` (`mov` at 0x10b3e9). `_taskDelay` has unknown
  candidates (`call` at 0x10b3da). Port status meaning and timing are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b37c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10b37f | store | 4 | `abs 0x1aff04` | `0x0` |
| `imul` at 0x10b395 | load | 4 | `abs 0x1aff08` | `-` |
| `mov` at 0x10b39d | load | 4 | `abs 0x1d89f0` | `-` |
| `cmp` at 0x10b3ba | load | 4 | `abs 0x1aff04` | `-` |
| `mov` at 0x10b3c3 | load | 4 | `abs 0x1d89f0` | `-` |
| `mov` at 0x10b3e9 | store | 1 | `[sp+4] +0x0` | `?` |
| `cmp` at 0x10b3ed | load | 4 | `abs 0x1aff04` | `-` |
| `mov` at 0x10b3fb | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b390 | `_sysClkRateGet 0x1085a8` | `0x10b5f8` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10b3a3 | `_wdStart 0x178f74` | `[0x1d89f0]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b3ae | `_sysInByte 0x1081b0` | `0x64` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b3c9 | `_wdCancel 0x179054` | `[0x1d89f0]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b3d1 | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b3da | `_taskDelay 0x176150` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b3e4 | `_sysInByte 0x1081b0` | `0x60` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10b3b8 | `(? & 0x1) != 0x0` | `(? & 0x1) == 0x0` |
| `je` at 0x10b3c1 | `[0x1aff04] == 0x0` | `[0x1aff04] != 0x0` |
| `je` at 0x10b3f4 | `[0x1aff04] == 0x0` | `[0x1aff04] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b401 | `eax` | `?` |

## `_kbdStatInit`

- `_kbdStatInit`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10adc8 to 0x10ae88.
  Established. Four-byte assignments write `0x1` to absolute `0x1d89f8` and `0x1d89fc`, `0x10b338`
  to `0x1d8a08`, and zero to `0x1d8a04` (`mov` at 0x10adea, `mov` at 0x10adf4, `mov` at 0x10ae10,
  `mov` at 0x10ae1a). Two-byte stores put `0x2` at `0x1d8a00` and zero at `0x1d8a02` (`mov` at
  0x10adfe, `mov` at 0x10ae07). `_kbdWrite` has candidate zero `0xffffffed` at one site and unknown
  candidates at another (`call` at 0x10ae54, `call` at 0x10ae65). The numeric value does not prove
  a call through it. State meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10add8 | store | 4 | `(unknown)` | `0x1d89f8` |
| `mov` at 0x10adea | store | 4 | `abs 0x1d89f8` | `0x1` |
| `mov` at 0x10adf4 | store | 4 | `abs 0x1d89fc` | `0x1` |
| `mov` at 0x10adfe | store | 2 | `abs 0x1d8a00` | `0x2` |
| `mov` at 0x10ae07 | store | 2 | `abs 0x1d8a02` | `0x0` |
| `mov` at 0x10ae10 | store | 4 | `abs 0x1d8a08` | `0x10b338` |
| `mov` at 0x10ae1a | store | 4 | `abs 0x1d8a04` | `0x0` |
| `mov` at 0x10ae24 | load | 1 | `abs 0x1d8a00` | `-` |
| `cmp` at 0x10ae2d | load | 1 | `abs 0x1aff0c` | `-` |
| `mov` at 0x10ae35 | store | 1 | `abs 0x1aff0c` | `?` |
| `cmp` at 0x10ae3b | load | 4 | `abs 0x1aff00` | `-` |
| `cmp` at 0x10ae44 | load | 4 | `abs 0x1aff04` | `-` |
| `cmp` at 0x10ae6d | load | 4 | `abs 0x1aff04` | `-` |
| `cmp` at 0x10ae76 | load | 4 | `abs 0x1aff00` | `-` |
| `mov` at 0x10ae7f | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ae4d | `_kbdReset 0x10b528` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ae54 | `_kbdWrite 0x10b408` | `0xffffffed` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ae65 | `_kbdWrite 0x10b408` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jns` at 0x10ade8 | `N == 0` | `not (N == 0)` |
| `je` at 0x10ae33 | `([0x1aff0c]:1 & 0xff) == ?` | `([0x1aff0c]:1 & 0xff) != ?` |
| `jg` at 0x10ae42 | `[0x1aff00] > 0x3 (signed, 32-bit)` | `[0x1aff00] <= 0x3 (signed, 32-bit)` |
| `je` at 0x10ae4b | `[0x1aff04] == 0x0` | `[0x1aff04] != 0x0` |
| `je` at 0x10ae5f | `ret(_kbdWrite) == 0xffffffff` | `ret(_kbdWrite) != 0xffffffff` |
| `je` at 0x10ae74 | `[0x1aff04] == 0x0` | `[0x1aff04] != 0x0` |
| `jle` at 0x10ae7d | `[0x1aff00] <= 0x3 (signed, 32-bit)` | `[0x1aff00] > 0x3 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ae85 | `eax` | `?` |

## `_kbdHook`

- `_kbdHook`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b338 to 0x10b378.
  Established. The first branch takes the `_kbdStatInit` arm when `([sp+4] & [sp+4]) == 0x0` (`je`
  at 0x10b340, `call` at 0x10b358). The subsequent comparison takes the `_kbdLedSet` arm when
  `[sp+4] == 0x1` (`je` at 0x10b345, `call` at 0x10b368). Both sites have candidate zero `ebp`
  (`call` at 0x10b358, `call` at 0x10b368). The remaining path returns `[sp+4]` (`ret` at 0x10b34a).
  Hook event meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b33b | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b358 | `_kbdStatInit 0x10adc8` | `ebp` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b368 | `_kbdLedSet 0x10b2c8` | `ebp` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10b340 | `([sp+4] & [sp+4]) == 0x0` | `([sp+4] & [sp+4]) != 0x0` |
| `je` at 0x10b345 | `[sp+4] == 0x1` | `[sp+4] != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b34a | `eax` | `[sp+4]` |
| `ret` at 0x10b360 | `eax` | `ret(_kbdStatInit)` |
| `ret` at 0x10b370 | `eax` | `ret(_kbdLedSet)` |

## `_kbdIntr`

- `_kbdIntr`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ae88 to 0x10aed8.
  Established. Two `_sysInByte` sites carry candidate zero `0x64` and `0x60` (`call` at 0x10ae93,
  `call` at 0x10aea1). The comparison falls through when `? == 0xfa` (`jne` at 0x10aeab), reaching a
  four-byte `0x1` store at absolute `0x1aff10` (`mov` at 0x10aead); its other arm calls
  `_kbdConvChar` with unknown candidate slots (`call` at 0x10aece). The four-byte update at absolute
  `0x1afefc` has an unknown value (`inc` at 0x10ae8b). Interrupt acknowledgment and byte-code
  meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `inc` at 0x10ae8b | load | 4 | `abs 0x1afefc` | `-` |
| `inc` at 0x10ae8b | store | 4 | `abs 0x1afefc` | `?` |
| `mov` at 0x10aead | store | 4 | `abs 0x1aff10` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ae93 | `_sysInByte 0x1081b0` | `0x64` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10aea1 | `_sysInByte 0x1081b0` | `0x60` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10aece | `_kbdConvChar 0x10aed8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10ae9d | `(? & 0x1) == 0x0` | `(? & 0x1) != 0x0` |
| `jne` at 0x10aeab | `? != 0xfa` | `? == 0xfa` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10aeba | `eax` | `ret(_sysInByte)` |
| `ret` at 0x10aed6 | `eax` | `?` |

## `_kbdConvChar`

- `_kbdConvChar`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10aed8 to 0x10afa8.
  Established. The initial comparison falls through when `([sp+4]:1 & 0xff) == 0xe0` (`jne` at
  0x10aee1), reaching a one-byte update at absolute `0x1d8a00` with unknown stored value (`or` at
  0x10aee3). Another branch takes its arm for `([sp+4]:1 & 0xff) == 0xe1` (`je` at 0x10af14). The
  indexed byte and four-byte loads have unknown targets (`mov` at 0x10af8a, `mov` at 0x10af96). The
  final indirect site has unknown callee and candidate slots (`call` at 0x10af9d). Character and
  flag encodings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10aedb | load | 1 | `sp +0x4` | `-` |
| `or` at 0x10aee3 | load | 1 | `abs 0x1d8a00` | `-` |
| `or` at 0x10aee3 | store | 1 | `abs 0x1d8a00` | `?` |
| `or` at 0x10aefd | load | 2 | `abs 0x1d8a00` | `-` |
| `or` at 0x10aefd | store | 2 | `abs 0x1d8a00` | `?` |
| `and` at 0x10af08 | load | 2 | `abs 0x1d8a00` | `-` |
| `and` at 0x10af08 | store | 2 | `abs 0x1d8a00` | `?` |
| `cmp` at 0x10af16 | load | 1 | `abs 0x1d8a00` | `-` |
| `xor` at 0x10af28 | load | 2 | `abs 0x1d8a00` | `-` |
| `xor` at 0x10af28 | store | 2 | `abs 0x1d8a00` | `?` |
| `xor` at 0x10af31 | load | 1 | `abs 0x1d8a00` | `-` |
| `xor` at 0x10af31 | store | 1 | `abs 0x1d8a00` | `?` |
| `test` at 0x10af5b | load | 1 | `abs 0x1d8a00` | `-` |
| `mov` at 0x10af68 | load | 1 | `(unknown)` | `-` |
| `xor` at 0x10af78 | load | 1 | `abs 0x1d8a00` | `-` |
| `xor` at 0x10af78 | store | 1 | `abs 0x1d8a00` | `?` |
| `mov` at 0x10af8a | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10af96 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10af9d | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10aee1 | `([sp+4]:1 & 0xff) != 0xe0` | `([sp+4]:1 & 0xff) == 0xe0` |
| `je` at 0x10aefb | `(([sp+4]:1 & 0xff) & 0x80) == 0x0` | `(([sp+4]:1 & 0xff) & 0x80) != 0x0` |
| `je` at 0x10af14 | `([sp+4]:1 & 0xff) == 0xe1` | `([sp+4]:1 & 0xff) != 0xe1` |
| `jge` at 0x10af1d | `([0x1d8a00]:1 & 0xff) >= 0x0 (signed, 8-bit)` | `([0x1d8a00]:1 & 0xff) < 0x0 (signed, 8-bit)` |
| `je` at 0x10af62 | `(([0x1d8a00]:1 & 0xff) & 0x20) == 0x0` | `(([0x1d8a00]:1 & 0xff) & 0x20) != 0x0` |
| `je` at 0x10af70 | `Z == 1` | `not (Z == 1)` |
| `jle` at 0x10af76 | `? <= 0xf (signed, 32-bit)` | `? > 0xf (signed, 32-bit)` |
| `jne` at 0x10af82 | `? != 0xffffffff` | `? == 0xffffffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10aeed | `eax` | `eax` |
| `ret` at 0x10af22 | `eax` | `eax` |
| `ret` at 0x10af3b | `eax` | `eax` |
| `ret` at 0x10afa2 | `eax` | `?` |

## `_kbdLedSet`

- `_kbdLedSet`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b2c8 to 0x10b338.
  Established. The initial comparison takes its exit arm when `([0x1aff0c]:1 & 0xff) == ?` (`je` at
  0x10b2e1). On the fall-through a byte of unknown value is stored at absolute `0x1aff0c` (`mov` at
  0x10b2e3). The `_sysOutByte` sites carry `0x60`, `0xffffffed` and then `0x60`, unknown (`call` at
  0x10b304, `call` at 0x10b32b). Both polling call sites use `_sysInByte` candidate zero `0x64`
  (`call` at 0x10b2ee, `call` at 0x10b312). LED identities and command effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b2cf | load | 1 | `abs 0x1d8a00` | `-` |
| `mov` at 0x10b2d8 | store | 1 | `sp -0x8` | `?` |
| `cmp` at 0x10b2db | load | 1 | `abs 0x1aff0c` | `-` |
| `mov` at 0x10b2e3 | store | 1 | `abs 0x1aff0c` | `?` |
| `movsx` at 0x10b324 | load | 1 | `sp -0x8` | `-` |
| `mov` at 0x10b330 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b2ee | `_sysInByte 0x1081b0` | `0x64` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b304 | `_sysOutByte 0x1081d0` | `0x60` | `0xffffffed` | `?` | `?` | `?` | `?` |
| `call` at 0x10b312 | `_sysInByte 0x1081b0` | `0x64` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b32b | `_sysOutByte 0x1081d0` | `0x60` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10b2e1 | `([0x1aff0c]:1 & 0xff) == ?` | `([0x1aff0c]:1 & 0xff) != ?` |
| `je` at 0x10b2f8 | `(? & 0x2) == 0x0` | `(? & 0x2) != 0x0` |
| `jle` at 0x10b2fe | `? <= 0x63 (signed, 32-bit)` | `? > 0x63 (signed, 32-bit)` |
| `je` at 0x10b31c | `(? & 0x2) == 0x0` | `(? & 0x2) != 0x0` |
| `jle` at 0x10b322 | `? <= 0x63 (signed, 32-bit)` | `? > 0x63 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b336 | `eax` | `?` |

## `_kbdWdog`

- `_kbdWdog`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b5f8 to 0x10b618.
  Established. The four-byte literal store writes `0x1` at absolute `0x1aff04` (`mov` at 0x10b5fb).
  The following four-byte update at absolute `0x1aff00` has an unknown stored value (`inc` at
  0x10b605). No call or conditional branch is reported, and the exit retains the entry `eax`
  expression (`ret` at 0x10b60e). A watchdog-expiry role is not proved by the body.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b5fb | store | 4 | `abs 0x1aff04` | `0x1` |
| `inc` at 0x10b605 | load | 4 | `abs 0x1aff00` | `-` |
| `inc` at 0x10b605 | store | 4 | `abs 0x1aff00` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b60e | `eax` | `eax` |
