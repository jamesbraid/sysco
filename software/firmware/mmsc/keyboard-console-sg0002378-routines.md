# MMSC keyboard and console paths in patch SG0002378

Read from `patches/SG0002378/usr/cpu/firmware/mmscfw.bin` (patch SG0002378).
Image SHA-256: `b1249e58cf4f00c252362da8ed0515e25bb94aaf096bb60a5c5082c9b6e4d342`.
This image has the x86 a.out link address `0x108000`. The [load map](load-map.md) explains segment mapping.
The analysed spans end at the next text symbol, with exclusive inferred ends.
They can include literals or padding after returns. Statements below concern sites
reached by known control-flow edges.
The computed transfer in `_pcConDrvIoctl` remains unresolved; its selector-arm effects are excluded.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records.
The full facts also include implicit push/pop accesses. Call and return instructions
remain in the listings without separate implicit stack access rows.
Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/keyboard-console-sg0002378-facts.txt) include all tool sections.

## `_pcConDrv`

- `_pcConDrv`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108858 to 0x1088b8.
  Established. The branch is taken when `[0x1d48a8] <= 0x0 (signed, 32-bit)` (`jle` at 0x108862),
  reaching the `_pcConDrvHrdInit` site (`call` at 0x108878). The `_iosDrvInstall` call presents
  leading candidates `0x108988`, `0x0`, `0x108988`, `0x0`, `0x16fb48`, and `0x16fa68` (`call` at
  0x10889a). A four-byte store places its result at absolute `0x1d48a8` (`mov` at 0x10889f).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10885b | load | 4 | `abs 0x1d48a8` | `-` |
| `mov` at 0x10889f | store | 4 | `abs 0x1d48a8` | `ret(_iosDrvInstall)` |
| `cmp` at 0x1088a6 | load | 4 | `abs 0x1d48a8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108878 | `_pcConDrvHrdInit 0x108958` | `ebp` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10889a | `_iosDrvInstall 0x1707a8` | `0x108988` | `0x0` | `0x108988` | `0x0` | `0x16fb48` | `0x16fa68` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x108862 | `[0x1d48a8] <= 0x0 (signed, 32-bit)` | `[0x1d48a8] > 0x0 (signed, 32-bit)` |
| `jne` at 0x1088ad | `[0x1d48a8] != 0xffffffff` | `[0x1d48a8] == 0xffffffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108869 | `eax` | `0x0` |
| `ret` at 0x1088b7 | `eax` | `?` |

## `_pcConDrvHrdInit`

- `_pcConDrvHrdInit`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108958 to 0x108988.
  Established. The `_kbdHrdInit` (`call` at 0x108963) and `_vgaHrdInit` (`call` at 0x108968) sites
  retain unknown candidate values. The later `_intUnlock` call has candidate zero `ret(_intLock)`
  (`call` at 0x10896e). A four-byte load at `sp -0x8` precedes the exit (`mov` at 0x108973).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x108973 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10895c | `_intLock 0x1712d0` | `ebx` | `ebp` | `?` | `?` | `?` | `?` |
| `call` at 0x108963 | `_kbdHrdInit 0x10ad08` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108968 | `_vgaHrdInit 0x108ee8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10896e | `_intUnlock 0x1712e0` | `ret(_intLock)` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108979 | `eax` | `ret(_intUnlock)` |

## `_pcConDrvIoctl`

- `_pcConDrvIoctl`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108998 to 0x108e08.
  Established. The selector-range branch is taken when `[sp+8]-1001 > 0x8 (unsigned, 32-bit)` (`ja`
  at 0x1089b1). That path reaches `_tyIoctl` with leading candidates `[sp+4]`, `[sp+8]`, and
  `[sp+12]` (`call` at 0x108aeb). The computed-transfer operand is a four-byte load at an unknown
  effective address (`jmp` at 0x1089b7). Selector-arm effects remain excluded.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10899d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1089a0 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1089a3 | load | 4 | `sp +0xc` | `-` |
| `jmp` at 0x1089b7 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108aeb | `_tyIoctl 0x16f748` | `[sp+4]` | `[sp+8]` | `[sp+12]` | `ebx` | `esi` | `ebp` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x1089b1 | `[sp+8]-1001 > 0x8 (unsigned, 32-bit)` | `[sp+8]-1001 <= 0x8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108afc | `eax` | `ret(_tyIoctl)` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x1089b7).

The undecoded tail is `0x108bc2` to exclusive end `0x108e08`, 582 bytes;
its SHA-256 is `c39232eb1c9aeb6f2037d0b13fc5ba30f45800ba3cd920bb5e29daafa4c4acbc`.
Those bytes have no credited instruction effects.

## `_pcConDevCreate`

- `_pcConDevCreate`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1088b8 to 0x108958.
  Established. The `_tyDevInit` site presents unknown candidate zero, `[sp+12]`, `[sp+16]`, and
  `0x109798` as its first four candidates (`call` at 0x108900); its zero-result fall-through (`jne`
  at 0x10890a) reaches `_sysIntEnablePIC` with candidate zero `0x1` (`call` at 0x10890e). The
  subsequent four-byte store writes `0x1` through an unknown effective address (`mov` at 0x108916).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1088be | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1088c1 | load | 4 | `sp +0xc` | `-` |
| `cmp` at 0x1088c4 | load | 4 | `abs 0x1d48a8` | `-` |
| `mov` at 0x1088eb | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1088fa | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x108916 | store | 4 | `(unknown)` | `0x1` |
| `mov` at 0x108920 | load | 4 | `abs 0x1d48a8` | `-` |
| `mov` at 0x108926 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108900 | `_tyDevInit 0x16f4f8` | `?` | `[sp+12]` | `[sp+16]` | `0x109798` | `ebx` | `esi` |
| `call` at 0x10890e | `_sysIntEnablePIC 0x108428` | `0x1` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10892b | `_iosDevAdd 0x170968` | `?` | `[sp+4]` | `[0x1d48a8]` | `?` | `?` | `?` |
| `call` at 0x10893d | `_errnoSet 0x171240` | `0xc0001` | `ebx` | `esi` | `edi` | `ebp` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x1088cb | `[0x1d48a8] <= 0x0 (signed, 32-bit)` | `[0x1d48a8] > 0x0 (signed, 32-bit)` |
| `ja` at 0x1088d0 | `[sp+8] > 0x1 (unsigned, 32-bit)` | `[sp+8] <= 0x1 (unsigned, 32-bit)` |
| `jne` at 0x1088f3 | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `jne` at 0x10890a | `(ret(_tyDevInit) & ret(_tyDevInit)) != 0x0` | `(ret(_tyDevInit) & ret(_tyDevInit)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108950 | `eax` | `?` |

## `_kbdHrdInit`

- `_kbdHrdInit`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ad08 to 0x10adc8.
  Established. The `_kbdCommand` sites include candidate zero `0x60` (`call` at 0x10ad3c, `call` at
  0x10ad76) and `0xffffffae` (`call` at 0x10ad94). The separate `_kbdWrite` sites have candidate
  zero `0x44` (`call` at 0x10ad4b) and `0x45` (`call` at 0x10ad85). The `_sysInByte` site presents
  candidate zero `0x64` (`call` at 0x10ad5a). No command meaning or reply is inferred from those
  values.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10ad14 | store | 4 | `abs 0x1d4ac0` | `ret(_wdCreate)` |
| `cmp` at 0x10ad19 | load | 4 | `abs 0x1ac088` | `-` |
| `cmp` at 0x10ad2c | load | 4 | `abs 0x1ac08c` | `-` |
| `cmp` at 0x10ad9c | load | 4 | `abs 0x1ac08c` | `-` |
| `cmp` at 0x10ada5 | load | 4 | `abs 0x1ac088` | `-` |
| `mov` at 0x10adb7 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ad0f | `_wdCreate 0x174ebc` | `ebx` | `?` | `ebp` | `?` | `?` | `?` |
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
| `jg` at 0x10ad20 | `[0x1ac088] > 0x3 (signed, 32-bit)` | `[0x1ac088] <= 0x3 (signed, 32-bit)` |
| `je` at 0x10ad33 | `[0x1ac08c] == 0x0` | `[0x1ac08c] != 0x0` |
| `je` at 0x10ad47 | `ret(_kbdCommand) == 0xffffffff` | `ret(_kbdCommand) != 0xffffffff` |
| `je` at 0x10ad56 | `ret(_kbdWrite) == 0xffffffff` | `ret(_kbdWrite) != 0xffffffff` |
| `je` at 0x10ad64 | `(? & 0x1) == 0x0` | `(? & 0x1) != 0x0` |
| `jne` at 0x10ad72 | `ret(_kbdRead) != 0xffffffff` | `ret(_kbdRead) == 0xffffffff` |
| `je` at 0x10ad81 | `ret(_kbdCommand) == 0xffffffff` | `ret(_kbdCommand) != 0xffffffff` |
| `je` at 0x10ad90 | `ret(_kbdWrite) == 0xffffffff` | `ret(_kbdWrite) != 0xffffffff` |
| `je` at 0x10ada3 | `[0x1ac08c] == 0x0` | `[0x1ac08c] != 0x0` |
| `jle` at 0x10adac | `[0x1ac088] <= 0x3 (signed, 32-bit)` | `[0x1ac088] > 0x3 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10adbd | `eax` | `ret(_kbdStatInit)` |

## `_kbdReset`

- `_kbdReset`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b528 to 0x10b5f8.
  Established. The `_sysOutByte` calls include leading pairs `0x60`, `0xffffffff` (`call` at
  0x10b548), `0x60`, `0x60` (`call` at 0x10b57c), `0x60`, `0x45` (`call` at 0x10b5a0), and `0x64`,
  `0xffffffae` (`call` at 0x10b5c4). The `_taskDelay` call has unknown candidate values (`call` at
  0x10b559). The first repeat branch is taken when `? <= 0x63 (signed, 32-bit)` (`jle` at 0x10b542),
  leaving its tested value unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b5e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b532 | `_sysInByte 0x1081b0` | `0x64` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b548 | `_sysOutByte 0x1081d0` | `0x60` | `0xffffffff` | `?` | `?` | `?` | `?` |
| `call` at 0x10b550 | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b559 | `_taskDelay 0x1722d8` | `?` | `?` | `?` | `?` | `?` | `?` |
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

- `_kbdCommand`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b488 to 0x10b528.
  Established. The four-byte store clears absolute `0x1ac08c` (`mov` at 0x10b48f). The `_sysOutByte`
  site has candidate zero `0x64` and unknown candidate one (`call` at 0x10b4d9). The repeat branch
  is taken when `[0x1ac08c] == 0x0` (`je` at 0x10b4d1). The `_wdStart` site retains candidate zero
  `[0x1d4ac0]` with unknown remaining candidates (`call` at 0x10b4b3).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b48c | load | 1 | `sp +0x4` | `-` |
| `mov` at 0x10b48f | store | 4 | `abs 0x1ac08c` | `0x0` |
| `imul` at 0x10b4a5 | load | 4 | `abs 0x1ac090` | `-` |
| `mov` at 0x10b4ad | load | 4 | `abs 0x1d4ac0` | `-` |
| `cmp` at 0x10b4ca | load | 4 | `abs 0x1ac08c` | `-` |
| `cmp` at 0x10b4f2 | load | 4 | `abs 0x1ac08c` | `-` |
| `mov` at 0x10b4fb | load | 4 | `abs 0x1d4ac0` | `-` |
| `cmp` at 0x10b508 | load | 4 | `abs 0x1ac08c` | `-` |
| `mov` at 0x10b516 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b4a0 | `_sysClkRateGet 0x1085a8` | `0x10b5f8` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10b4b3 | `_wdStart 0x1750fc` | `[0x1d4ac0]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b4be | `_sysInByte 0x1081b0` | `0x64` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b4d9 | `_sysOutByte 0x1081d0` | `0x64` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b4e6 | `_sysInByte 0x1081b0` | `0x64` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b501 | `_wdCancel 0x1751dc` | `[0x1d4ac0]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10b4c8 | `(? & 0x2) == 0x0` | `(? & 0x2) != 0x0` |
| `je` at 0x10b4d1 | `[0x1ac08c] == 0x0` | `[0x1ac08c] != 0x0` |
| `je` at 0x10b4f0 | `(? & 0x2) == 0x0` | `(? & 0x2) != 0x0` |
| `je` at 0x10b4f9 | `[0x1ac08c] == 0x0` | `[0x1ac08c] != 0x0` |
| `je` at 0x10b50f | `[0x1ac08c] == 0x0` | `[0x1ac08c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b51c | `eax` | `?` |

## `_kbdWrite`

- `_kbdWrite`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b408 to 0x10b488.
  Established. The `_wdStart` site has candidate zero `[0x1d4ac0]` and unknown candidates thereafter
  (`call` at 0x10b433). The `_sysOutByte` call presents `0x60` and unknown candidate one (`call` at
  0x10b467). A prior four-byte store clears absolute `0x1ac08c` (`mov` at 0x10b40f). The
  return-value branch is taken when `[0x1ac08c] == 0x0` (`je` at 0x10b475).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b40c | load | 1 | `sp +0x4` | `-` |
| `mov` at 0x10b40f | store | 4 | `abs 0x1ac08c` | `0x0` |
| `imul` at 0x10b425 | load | 4 | `abs 0x1ac090` | `-` |
| `mov` at 0x10b42d | load | 4 | `abs 0x1d4ac0` | `-` |
| `cmp` at 0x10b44a | load | 4 | `abs 0x1ac08c` | `-` |
| `mov` at 0x10b453 | load | 4 | `abs 0x1d4ac0` | `-` |
| `cmp` at 0x10b46e | load | 4 | `abs 0x1ac08c` | `-` |
| `mov` at 0x10b47c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b420 | `_sysClkRateGet 0x1085a8` | `0x10b5f8` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10b433 | `_wdStart 0x1750fc` | `[0x1d4ac0]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b43e | `_sysInByte 0x1081b0` | `0x64` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b459 | `_wdCancel 0x1751dc` | `[0x1d4ac0]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b467 | `_sysOutByte 0x1081d0` | `0x60` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10b448 | `(? & 0x2) == 0x0` | `(? & 0x2) != 0x0` |
| `je` at 0x10b451 | `[0x1ac08c] == 0x0` | `[0x1ac08c] != 0x0` |
| `je` at 0x10b475 | `[0x1ac08c] == 0x0` | `[0x1ac08c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b482 | `eax` | `?` |

## `_kbdRead`

- `_kbdRead`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b378 to 0x10b408.
  Established. The `_sysInByte` sites have candidate zero `0x64` (`call` at 0x10b3ae) and `0x60`
  (`call` at 0x10b3e4). The byte store through `[sp+4] +0x0` has unknown value (`mov` at 0x10b3e9).
  The earlier loop-exit branch is taken when `(? & 0x1) != 0x0` (`jne` at 0x10b3b8), with unknown
  tested value.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b37c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10b37f | store | 4 | `abs 0x1ac08c` | `0x0` |
| `imul` at 0x10b395 | load | 4 | `abs 0x1ac090` | `-` |
| `mov` at 0x10b39d | load | 4 | `abs 0x1d4ac0` | `-` |
| `cmp` at 0x10b3ba | load | 4 | `abs 0x1ac08c` | `-` |
| `mov` at 0x10b3c3 | load | 4 | `abs 0x1d4ac0` | `-` |
| `mov` at 0x10b3e9 | store | 1 | `[sp+4] +0x0` | `?` |
| `cmp` at 0x10b3ed | load | 4 | `abs 0x1ac08c` | `-` |
| `mov` at 0x10b3fb | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b390 | `_sysClkRateGet 0x1085a8` | `0x10b5f8` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10b3a3 | `_wdStart 0x1750fc` | `[0x1d4ac0]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b3ae | `_sysInByte 0x1081b0` | `0x64` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b3c9 | `_wdCancel 0x1751dc` | `[0x1d4ac0]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b3d1 | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b3da | `_taskDelay 0x1722d8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b3e4 | `_sysInByte 0x1081b0` | `0x60` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10b3b8 | `(? & 0x1) != 0x0` | `(? & 0x1) == 0x0` |
| `je` at 0x10b3c1 | `[0x1ac08c] == 0x0` | `[0x1ac08c] != 0x0` |
| `je` at 0x10b3f4 | `[0x1ac08c] == 0x0` | `[0x1ac08c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b401 | `eax` | `?` |

## `_kbdStatInit`

- `_kbdStatInit`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10adc8 to 0x10ae88.
  Established. Four-byte stores write `0x1` to absolute `0x1d4ac8` (`mov` at 0x10adea) and
  `0x1d4acc` (`mov` at 0x10adf4). The two-byte stores place `0x2` at absolute `0x1d4ad0` (`mov` at
  0x10adfe) and `0x0` at absolute `0x1d4ad2` (`mov` at 0x10ae07). Another four-byte store writes
  `0x10b338` to absolute `0x1d4ad8` (`mov` at 0x10ae10), without establishing that pointer role.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10add8 | store | 4 | `(unknown)` | `0x1d4ac8` |
| `mov` at 0x10adea | store | 4 | `abs 0x1d4ac8` | `0x1` |
| `mov` at 0x10adf4 | store | 4 | `abs 0x1d4acc` | `0x1` |
| `mov` at 0x10adfe | store | 2 | `abs 0x1d4ad0` | `0x2` |
| `mov` at 0x10ae07 | store | 2 | `abs 0x1d4ad2` | `0x0` |
| `mov` at 0x10ae10 | store | 4 | `abs 0x1d4ad8` | `0x10b338` |
| `mov` at 0x10ae1a | store | 4 | `abs 0x1d4ad4` | `0x0` |
| `mov` at 0x10ae24 | load | 1 | `abs 0x1d4ad0` | `-` |
| `cmp` at 0x10ae2d | load | 1 | `abs 0x1ac094` | `-` |
| `mov` at 0x10ae35 | store | 1 | `abs 0x1ac094` | `?` |
| `cmp` at 0x10ae3b | load | 4 | `abs 0x1ac088` | `-` |
| `cmp` at 0x10ae44 | load | 4 | `abs 0x1ac08c` | `-` |
| `cmp` at 0x10ae6d | load | 4 | `abs 0x1ac08c` | `-` |
| `cmp` at 0x10ae76 | load | 4 | `abs 0x1ac088` | `-` |
| `mov` at 0x10ae7f | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ae4d | `_kbdReset 0x10b528` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ae54 | `_kbdWrite 0x10b408` | `0xffffffed` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ae65 | `_kbdWrite 0x10b408` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jns` at 0x10ade8 | `N == 0` | `not (N == 0)` |
| `je` at 0x10ae33 | `([0x1ac094]:1 & 0xff) == ?` | `([0x1ac094]:1 & 0xff) != ?` |
| `jg` at 0x10ae42 | `[0x1ac088] > 0x3 (signed, 32-bit)` | `[0x1ac088] <= 0x3 (signed, 32-bit)` |
| `je` at 0x10ae4b | `[0x1ac08c] == 0x0` | `[0x1ac08c] != 0x0` |
| `je` at 0x10ae5f | `ret(_kbdWrite) == 0xffffffff` | `ret(_kbdWrite) != 0xffffffff` |
| `je` at 0x10ae74 | `[0x1ac08c] == 0x0` | `[0x1ac08c] != 0x0` |
| `jle` at 0x10ae7d | `[0x1ac088] <= 0x3 (signed, 32-bit)` | `[0x1ac088] > 0x3 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ae85 | `eax` | `?` |

## `_kbdHook`

- `_kbdHook`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b338 to 0x10b378.
  Established. The first branch is taken when `([sp+4] & [sp+4]) == 0x0` (`je` at 0x10b340),
  reaching `_kbdStatInit` with candidate zero `ebp` (`call` at 0x10b358). The separate branch is
  taken when `[sp+4] == 0x1` (`je` at 0x10b345), reaching `_kbdLedSet` with candidate zero `ebp`
  (`call` at 0x10b368). These are candidate slots with unknown arity, not established semantic
  arguments.

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

- `_kbdIntr`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ae88 to 0x10aed8.
  Established. The first `_sysInByte` site has candidate zero `0x64` (`call` at 0x10ae93). A
  conditional later site presents `0x60` (`call` at 0x10aea1); its comparison branch falls through
  when `? == 0xfa` (`jne` at 0x10aeab), reaching a four-byte store of `0x1` to absolute `0x1ac098`
  (`mov` at 0x10aead). The `_kbdConvChar` site retains unknown candidate values (`call` at
  0x10aece).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `inc` at 0x10ae8b | load | 4 | `abs 0x1ac084` | `-` |
| `inc` at 0x10ae8b | store | 4 | `abs 0x1ac084` | `?` |
| `mov` at 0x10aead | store | 4 | `abs 0x1ac098` | `0x1` |

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

- `_kbdConvChar`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10aed8 to 0x10afa8.
  Established. The first branch falls through when `([sp+4]:1 & 0xff) == 0xe0` (`jne` at 0x10aee1),
  reaching a byte read-modify-write at absolute `0x1d4ad0` with unknown stored value (`or` at
  0x10aee3). A separate branch is taken when `([sp+4]:1 & 0xff) == 0xe1` (`je` at 0x10af14). The
  indirect call has unknown callee and candidate values (`call` at 0x10af9d).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10aedb | load | 1 | `sp +0x4` | `-` |
| `or` at 0x10aee3 | load | 1 | `abs 0x1d4ad0` | `-` |
| `or` at 0x10aee3 | store | 1 | `abs 0x1d4ad0` | `?` |
| `or` at 0x10aefd | load | 2 | `abs 0x1d4ad0` | `-` |
| `or` at 0x10aefd | store | 2 | `abs 0x1d4ad0` | `?` |
| `and` at 0x10af08 | load | 2 | `abs 0x1d4ad0` | `-` |
| `and` at 0x10af08 | store | 2 | `abs 0x1d4ad0` | `?` |
| `cmp` at 0x10af16 | load | 1 | `abs 0x1d4ad0` | `-` |
| `xor` at 0x10af28 | load | 2 | `abs 0x1d4ad0` | `-` |
| `xor` at 0x10af28 | store | 2 | `abs 0x1d4ad0` | `?` |
| `xor` at 0x10af31 | load | 1 | `abs 0x1d4ad0` | `-` |
| `xor` at 0x10af31 | store | 1 | `abs 0x1d4ad0` | `?` |
| `test` at 0x10af5b | load | 1 | `abs 0x1d4ad0` | `-` |
| `mov` at 0x10af68 | load | 1 | `(unknown)` | `-` |
| `xor` at 0x10af78 | load | 1 | `abs 0x1d4ad0` | `-` |
| `xor` at 0x10af78 | store | 1 | `abs 0x1d4ad0` | `?` |
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
| `jge` at 0x10af1d | `([0x1d4ad0]:1 & 0xff) >= 0x0 (signed, 8-bit)` | `([0x1d4ad0]:1 & 0xff) < 0x0 (signed, 8-bit)` |
| `je` at 0x10af62 | `(([0x1d4ad0]:1 & 0xff) & 0x20) == 0x0` | `(([0x1d4ad0]:1 & 0xff) & 0x20) != 0x0` |
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

- `_kbdLedSet`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b2c8 to 0x10b338.
  Established. The `_sysOutByte` sites have leading candidates `0x60`, `0xffffffed` (`call` at
  0x10b304) and `0x60`, unknown (`call` at 0x10b32b). The initial branch bypasses both sites when
  `([0x1ac094]:1 & 0xff) == ?` (`je` at 0x10b2e1). The byte store to absolute `0x1ac094` has unknown
  value (`mov` at 0x10b2e3).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b2cf | load | 1 | `abs 0x1d4ad0` | `-` |
| `mov` at 0x10b2d8 | store | 1 | `sp -0x8` | `?` |
| `cmp` at 0x10b2db | load | 1 | `abs 0x1ac094` | `-` |
| `mov` at 0x10b2e3 | store | 1 | `abs 0x1ac094` | `?` |
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
| `je` at 0x10b2e1 | `([0x1ac094]:1 & 0xff) == ?` | `([0x1ac094]:1 & 0xff) != ?` |
| `je` at 0x10b2f8 | `(? & 0x2) == 0x0` | `(? & 0x2) != 0x0` |
| `jle` at 0x10b2fe | `? <= 0x63 (signed, 32-bit)` | `? > 0x63 (signed, 32-bit)` |
| `je` at 0x10b31c | `(? & 0x2) == 0x0` | `(? & 0x2) != 0x0` |
| `jle` at 0x10b322 | `? <= 0x63 (signed, 32-bit)` | `? > 0x63 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b336 | `eax` | `?` |

## `_kbdWdog`

- `_kbdWdog`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b5f8 to 0x10b618.
  Established. The four-byte store writes `0x1` to absolute `0x1ac08c` (`mov` at 0x10b5fb). The
  following four-byte read-modify-write at absolute `0x1ac088` has unknown stored value (`inc` at
  0x10b605). The exit retains the entry expression `eax` (`ret` at 0x10b60e). The routine role
  remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b5fb | store | 4 | `abs 0x1ac08c` | `0x1` |
| `inc` at 0x10b605 | load | 4 | `abs 0x1ac088` | `-` |
| `inc` at 0x10b605 | store | 4 | `abs 0x1ac088` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b60e | `eax` | `eax` |
