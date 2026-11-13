# MMSC keyboard and console callers in the 6.4-origin image

Read from `6.4-origin/usr/cpu/firmware/mmscfw.bin`.
Image SHA-256: `b766e559f86078526916d8bd98bbb8079294ad774cce9177928873153c6fb09b`.
The x86 a.out link address is `0x108000`. The [load map](load-map.md) explains segment mapping.
Analysed spans have exclusive ends inferred from the next text symbol.
Some include literals and padding decoded linearly after returns. Only sites reached
by known control-flow edges contribute effects to the tables.
The undecoded tail from `0x108bc2` to exclusive `0x108e08` contains 582 bytes.
It was read directly; its SHA-256 is `c39232eb1c9aeb6f2037d0b13fc5ba30f45800ba3cd920bb5e29daafa4c4acbc`.
No instruction effects are credited to those bytes.
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

[Fact sheets and complete listings](disasm/keyboard-console-64-origin-facts.txt) include all tool sections.

## `_pcConDrv`

- `_pcConDrv`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108858 to 0x1088b8.
  Established. The `_pcConDrvHrdInit` call retains candidate zero `ebp` (`call` at 0x108878). The
  `_iosDrvInstall` site presents `0x108988`, `0x0`, `0x108988`, `0x0`, `0x16e720`, and `0x16e640` as
  its six printed candidates (`call` at 0x10889a); its result is stored as four bytes at absolute
  `0x1d2518` (`mov` at 0x10889f). The later branch is taken when `[0x1d2518] != 0xffffffff` (`jne`
  at 0x1088ad). Installation semantics and calls through candidate addresses remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10885b | load | 4 | `abs 0x1d2518` | `-` |
| `mov` at 0x10889f | store | 4 | `abs 0x1d2518` | `ret(_iosDrvInstall)` |
| `cmp` at 0x1088a6 | load | 4 | `abs 0x1d2518` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108878 | `_pcConDrvHrdInit 0x108958` | `ebp` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10889a | `_iosDrvInstall 0x16f050` | `0x108988` | `0x0` | `0x108988` | `0x0` | `0x16e720` | `0x16e640` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x108862 | `[0x1d2518] <= 0x0 (signed, 32-bit)` | `[0x1d2518] > 0x0 (signed, 32-bit)` |
| `jne` at 0x1088ad | `[0x1d2518] != 0xffffffff` | `[0x1d2518] == 0xffffffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108869 | `eax` | `0x0` |
| `ret` at 0x1088b7 | `eax` | `?` |

## `_pcConDrvHrdInit`

- `_pcConDrvHrdInit`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108958 to 0x108988.
  Established. The `_intLock` site retains candidate zero `ebx` (`call` at 0x10895c). Calls to
  `_kbdHrdInit` (`call` at 0x108963) and `_vgaHrdInit` (`call` at 0x108968) have unknown candidate
  slots. The `_intUnlock` site presents candidate zero `ret(_intLock)` (`call` at 0x10896e), and the
  exit reports `ret(_intUnlock)` in `eax` (`ret` at 0x108979). The body does not independently
  establish keyboard or display initialization effects.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x108973 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10895c | `_intLock 0x16fb78` | `ebx` | `ebp` | `?` | `?` | `?` | `?` |
| `call` at 0x108963 | `_kbdHrdInit 0x10ad08` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108968 | `_vgaHrdInit 0x108ee8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10896e | `_intUnlock 0x16fb88` | `ret(_intLock)` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108979 | `eax` | `ret(_intUnlock)` |

## `_pcConDrvIoctl`

- `_pcConDrvIoctl`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108998 to 0x108e08.
  Established. The range branch is taken when `[sp+8]-1001 > 0x8 (unsigned, 32-bit)` (`ja` at
  0x1089b1), reaching `_tyIoctl` with leading candidates `[sp+4]`, `[sp+8]`, and `[sp+12]` (`call`
  at 0x108aeb). The other path reaches an unresolved computed transfer (`jmp` at 0x1089b7).
  Selector-only arms contribute no access or call effects to this entry. The known-path exit reports
  `ret(_tyIoctl)` in `eax` (`ret` at 0x108afc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10899d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1089a0 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1089a3 | load | 4 | `sp +0xc` | `-` |
| `jmp` at 0x1089b7 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108aeb | `_tyIoctl 0x16e370` | `[sp+4]` | `[sp+8]` | `[sp+12]` | `ebx` | `esi` | `ebp` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x1089b1 | `[sp+8]-1001 > 0x8 (unsigned, 32-bit)` | `[sp+8]-1001 <= 0x8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108afc | `eax` | `ret(_tyIoctl)` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x1089b7).

## `_pcConDevCreate`

- `_pcConDevCreate`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1088b8 to 0x108958.
  Established. The opening branch is taken when `[0x1d2518] <= 0x0 (signed, 32-bit)` (`jle` at
  0x1088cb). The `_tyDevInit` site has unknown candidate zero, followed by `[sp+12]`, `[sp+16]`, and
  `0x109798` (`call` at 0x108900). The `_sysIntEnablePIC` call presents candidate zero `0x1` (`call`
  at 0x10890e). A four-byte store at an unresolved target writes `0x1` (`mov` at 0x108916). Neither
  a device record nor interrupt-enabling semantics is assigned from these names.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1088be | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1088c1 | load | 4 | `sp +0xc` | `-` |
| `cmp` at 0x1088c4 | load | 4 | `abs 0x1d2518` | `-` |
| `mov` at 0x1088eb | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1088fa | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x108916 | store | 4 | `(unknown)` | `0x1` |
| `mov` at 0x108920 | load | 4 | `abs 0x1d2518` | `-` |
| `mov` at 0x108926 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108900 | `_tyDevInit 0x16e160` | `?` | `[sp+12]` | `[sp+16]` | `0x109798` | `ebx` | `esi` |
| `call` at 0x10890e | `_sysIntEnablePIC 0x108428` | `0x1` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10892b | `_iosDevAdd 0x16f210` | `?` | `[sp+4]` | `[0x1d2518]` | `?` | `?` | `?` |
| `call` at 0x10893d | `_errnoSet 0x16fae8` | `0xc0001` | `ebx` | `esi` | `edi` | `ebp` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x1088cb | `[0x1d2518] <= 0x0 (signed, 32-bit)` | `[0x1d2518] > 0x0 (signed, 32-bit)` |
| `ja` at 0x1088d0 | `[sp+8] > 0x1 (unsigned, 32-bit)` | `[sp+8] <= 0x1 (unsigned, 32-bit)` |
| `jne` at 0x1088f3 | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `jne` at 0x10890a | `(ret(_tyDevInit) & ret(_tyDevInit)) != 0x0` | `(ret(_tyDevInit) & ret(_tyDevInit)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108950 | `eax` | `?` |

## `_kbdHrdInit`

- `_kbdHrdInit`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ad08 to 0x10adc8.
  Established. The four-byte store at absolute `0x1d2730` retains `ret(_wdCreate)` (`mov` at
  0x10ad14). The two `_kbdCommand` sites present candidate zero `0x60` (`call` at 0x10ad3c, `call`
  at 0x10ad76), while a later site presents `0xffffffae` (`call` at 0x10ad94). The `_kbdWrite` sites
  present `0x44` (`call` at 0x10ad4b) and `0x45` (`call` at 0x10ad85). These candidates do not
  establish command-token meanings.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10ad14 | store | 4 | `abs 0x1d2730` | `ret(_wdCreate)` |
| `cmp` at 0x10ad19 | load | 4 | `abs 0x1a9cd8` | `-` |
| `cmp` at 0x10ad2c | load | 4 | `abs 0x1a9cdc` | `-` |
| `cmp` at 0x10ad9c | load | 4 | `abs 0x1a9cdc` | `-` |
| `cmp` at 0x10ada5 | load | 4 | `abs 0x1a9cd8` | `-` |
| `mov` at 0x10adb7 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ad0f | `_wdCreate 0x173764` | `ebx` | `?` | `ebp` | `?` | `?` | `?` |
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
| `jg` at 0x10ad20 | `[0x1a9cd8] > 0x3 (signed, 32-bit)` | `[0x1a9cd8] <= 0x3 (signed, 32-bit)` |
| `je` at 0x10ad33 | `[0x1a9cdc] == 0x0` | `[0x1a9cdc] != 0x0` |
| `je` at 0x10ad47 | `ret(_kbdCommand) == 0xffffffff` | `ret(_kbdCommand) != 0xffffffff` |
| `je` at 0x10ad56 | `ret(_kbdWrite) == 0xffffffff` | `ret(_kbdWrite) != 0xffffffff` |
| `je` at 0x10ad64 | `(? & 0x1) == 0x0` | `(? & 0x1) != 0x0` |
| `jne` at 0x10ad72 | `ret(_kbdRead) != 0xffffffff` | `ret(_kbdRead) == 0xffffffff` |
| `je` at 0x10ad81 | `ret(_kbdCommand) == 0xffffffff` | `ret(_kbdCommand) != 0xffffffff` |
| `je` at 0x10ad90 | `ret(_kbdWrite) == 0xffffffff` | `ret(_kbdWrite) != 0xffffffff` |
| `je` at 0x10ada3 | `[0x1a9cdc] == 0x0` | `[0x1a9cdc] != 0x0` |
| `jle` at 0x10adac | `[0x1a9cd8] <= 0x3 (signed, 32-bit)` | `[0x1a9cd8] > 0x3 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10adbd | `eax` | `ret(_kbdStatInit)` |

## `_kbdReset`

- `_kbdReset`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b528 to 0x10b5f8.
  Established. The `_sysOutByte` sites present leading pairs `0x60`, `0xffffffff` (`call` at
  0x10b548), `0x60`, `0x60` (`call` at 0x10b57c), `0x60`, `0x45` (`call` at 0x10b5a0), and `0x64`,
  `0xffffffae` (`call` at 0x10b5c4). The `_taskDelay` site retains unknown candidates (`call` at
  0x10b559). No keyboard-reset outcome, port role or delay unit is inferred from these calls.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b5e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b532 | `_sysInByte 0x1081b0` | `0x64` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b548 | `_sysOutByte 0x1081d0` | `0x60` | `0xffffffff` | `?` | `?` | `?` | `?` |
| `call` at 0x10b550 | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b559 | `_taskDelay 0x170b80` | `?` | `?` | `?` | `?` | `?` | `?` |
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

- `_kbdCommand`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b488 to 0x10b528.
  Established. A four-byte store writes `0x0` at absolute `0x1a9cdc` (`mov` at 0x10b48f). The
  `_wdStart` site presents candidate zero `[0x1d2730]` and unknown remaining slots (`call` at
  0x10b4b3). The `_sysOutByte` site has candidate zero `0x64` and unknown candidate one (`call` at
  0x10b4d9). The final branch is taken when `[0x1a9cdc] == 0x0` (`je` at 0x10b50f). Command and
  watchdog roles remain unassigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b48c | load | 1 | `sp +0x4` | `-` |
| `mov` at 0x10b48f | store | 4 | `abs 0x1a9cdc` | `0x0` |
| `imul` at 0x10b4a5 | load | 4 | `abs 0x1a9ce0` | `-` |
| `mov` at 0x10b4ad | load | 4 | `abs 0x1d2730` | `-` |
| `cmp` at 0x10b4ca | load | 4 | `abs 0x1a9cdc` | `-` |
| `cmp` at 0x10b4f2 | load | 4 | `abs 0x1a9cdc` | `-` |
| `mov` at 0x10b4fb | load | 4 | `abs 0x1d2730` | `-` |
| `cmp` at 0x10b508 | load | 4 | `abs 0x1a9cdc` | `-` |
| `mov` at 0x10b516 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b4a0 | `_sysClkRateGet 0x1085a8` | `0x10b5f8` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10b4b3 | `_wdStart 0x1739a4` | `[0x1d2730]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b4be | `_sysInByte 0x1081b0` | `0x64` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b4d9 | `_sysOutByte 0x1081d0` | `0x64` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b4e6 | `_sysInByte 0x1081b0` | `0x64` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b501 | `_wdCancel 0x173a84` | `[0x1d2730]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10b4c8 | `(? & 0x2) == 0x0` | `(? & 0x2) != 0x0` |
| `je` at 0x10b4d1 | `[0x1a9cdc] == 0x0` | `[0x1a9cdc] != 0x0` |
| `je` at 0x10b4f0 | `(? & 0x2) == 0x0` | `(? & 0x2) != 0x0` |
| `je` at 0x10b4f9 | `[0x1a9cdc] == 0x0` | `[0x1a9cdc] != 0x0` |
| `je` at 0x10b50f | `[0x1a9cdc] == 0x0` | `[0x1a9cdc] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b51c | `eax` | `?` |

## `_kbdWrite`

- `_kbdWrite`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b408 to 0x10b488.
  Established. The initial four-byte store writes `0x0` at absolute `0x1a9cdc` (`mov` at 0x10b40f).
  The `_sysInByte` site presents candidate zero `0x64` (`call` at 0x10b43e). The following backward
  branch is taken when `[0x1a9cdc] == 0x0` (`je` at 0x10b451). The `_sysOutByte` call presents
  candidate zero `0x60` and unknown candidate one (`call` at 0x10b467). The emitted value remains
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b40c | load | 1 | `sp +0x4` | `-` |
| `mov` at 0x10b40f | store | 4 | `abs 0x1a9cdc` | `0x0` |
| `imul` at 0x10b425 | load | 4 | `abs 0x1a9ce0` | `-` |
| `mov` at 0x10b42d | load | 4 | `abs 0x1d2730` | `-` |
| `cmp` at 0x10b44a | load | 4 | `abs 0x1a9cdc` | `-` |
| `mov` at 0x10b453 | load | 4 | `abs 0x1d2730` | `-` |
| `cmp` at 0x10b46e | load | 4 | `abs 0x1a9cdc` | `-` |
| `mov` at 0x10b47c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b420 | `_sysClkRateGet 0x1085a8` | `0x10b5f8` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10b433 | `_wdStart 0x1739a4` | `[0x1d2730]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b43e | `_sysInByte 0x1081b0` | `0x64` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b459 | `_wdCancel 0x173a84` | `[0x1d2730]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b467 | `_sysOutByte 0x1081d0` | `0x60` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10b448 | `(? & 0x2) == 0x0` | `(? & 0x2) != 0x0` |
| `je` at 0x10b451 | `[0x1a9cdc] == 0x0` | `[0x1a9cdc] != 0x0` |
| `je` at 0x10b475 | `[0x1a9cdc] == 0x0` | `[0x1a9cdc] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b482 | `eax` | `?` |

## `_kbdRead`

- `_kbdRead`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b378 to 0x10b408.
  Established. The `_sysInByte` sites present candidate zero `0x64` (`call` at 0x10b3ae) and later
  `0x60` (`call` at 0x10b3e4). A byte store through `[sp+4] +0x0` retains an unknown value (`mov` at
  0x10b3e9). The final branch is taken when `[0x1a9cdc] == 0x0` (`je` at 0x10b3f4). Neither a
  received character value nor a timeout duration is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b37c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10b37f | store | 4 | `abs 0x1a9cdc` | `0x0` |
| `imul` at 0x10b395 | load | 4 | `abs 0x1a9ce0` | `-` |
| `mov` at 0x10b39d | load | 4 | `abs 0x1d2730` | `-` |
| `cmp` at 0x10b3ba | load | 4 | `abs 0x1a9cdc` | `-` |
| `mov` at 0x10b3c3 | load | 4 | `abs 0x1d2730` | `-` |
| `mov` at 0x10b3e9 | store | 1 | `[sp+4] +0x0` | `?` |
| `cmp` at 0x10b3ed | load | 4 | `abs 0x1a9cdc` | `-` |
| `mov` at 0x10b3fb | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b390 | `_sysClkRateGet 0x1085a8` | `0x10b5f8` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10b3a3 | `_wdStart 0x1739a4` | `[0x1d2730]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b3ae | `_sysInByte 0x1081b0` | `0x64` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b3c9 | `_wdCancel 0x173a84` | `[0x1d2730]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b3d1 | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b3da | `_taskDelay 0x170b80` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b3e4 | `_sysInByte 0x1081b0` | `0x60` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10b3b8 | `(? & 0x1) != 0x0` | `(? & 0x1) == 0x0` |
| `je` at 0x10b3c1 | `[0x1a9cdc] == 0x0` | `[0x1a9cdc] != 0x0` |
| `je` at 0x10b3f4 | `[0x1a9cdc] == 0x0` | `[0x1a9cdc] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b401 | `eax` | `?` |

## `_kbdStatInit`

- `_kbdStatInit`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10adc8 to 0x10ae88.
  Established. Four-byte stores write `0x1` at absolute `0x1d2738` (`mov` at 0x10adea) and
  `0x1d273c` (`mov` at 0x10adf4). Two-byte stores write `0x2` at absolute `0x1d2740` (`mov` at
  0x10adfe) and `0x0` at `0x1d2742` (`mov` at 0x10ae07). Another four-byte store writes `0x10b338`
  at absolute `0x1d2748` (`mov` at 0x10ae10). The `_kbdWrite` site presents candidate zero
  `0xffffffed` (`call` at 0x10ae54). No record layout or callback invocation is inferred.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10add8 | store | 4 | `(unknown)` | `0x1d2738` |
| `mov` at 0x10adea | store | 4 | `abs 0x1d2738` | `0x1` |
| `mov` at 0x10adf4 | store | 4 | `abs 0x1d273c` | `0x1` |
| `mov` at 0x10adfe | store | 2 | `abs 0x1d2740` | `0x2` |
| `mov` at 0x10ae07 | store | 2 | `abs 0x1d2742` | `0x0` |
| `mov` at 0x10ae10 | store | 4 | `abs 0x1d2748` | `0x10b338` |
| `mov` at 0x10ae1a | store | 4 | `abs 0x1d2744` | `0x0` |
| `mov` at 0x10ae24 | load | 1 | `abs 0x1d2740` | `-` |
| `cmp` at 0x10ae2d | load | 1 | `abs 0x1a9ce4` | `-` |
| `mov` at 0x10ae35 | store | 1 | `abs 0x1a9ce4` | `?` |
| `cmp` at 0x10ae3b | load | 4 | `abs 0x1a9cd8` | `-` |
| `cmp` at 0x10ae44 | load | 4 | `abs 0x1a9cdc` | `-` |
| `cmp` at 0x10ae6d | load | 4 | `abs 0x1a9cdc` | `-` |
| `cmp` at 0x10ae76 | load | 4 | `abs 0x1a9cd8` | `-` |
| `mov` at 0x10ae7f | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ae4d | `_kbdReset 0x10b528` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ae54 | `_kbdWrite 0x10b408` | `0xffffffed` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ae65 | `_kbdWrite 0x10b408` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jns` at 0x10ade8 | `N == 0` | `not (N == 0)` |
| `je` at 0x10ae33 | `([0x1a9ce4]:1 & 0xff) == ?` | `([0x1a9ce4]:1 & 0xff) != ?` |
| `jg` at 0x10ae42 | `[0x1a9cd8] > 0x3 (signed, 32-bit)` | `[0x1a9cd8] <= 0x3 (signed, 32-bit)` |
| `je` at 0x10ae4b | `[0x1a9cdc] == 0x0` | `[0x1a9cdc] != 0x0` |
| `je` at 0x10ae5f | `ret(_kbdWrite) == 0xffffffff` | `ret(_kbdWrite) != 0xffffffff` |
| `je` at 0x10ae74 | `[0x1a9cdc] == 0x0` | `[0x1a9cdc] != 0x0` |
| `jle` at 0x10ae7d | `[0x1a9cd8] <= 0x3 (signed, 32-bit)` | `[0x1a9cd8] > 0x3 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ae85 | `eax` | `?` |

## `_kbdHook`

- `_kbdHook`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b338 to 0x10b378.
  Established. The first branch is taken when `([sp+4] & [sp+4]) == 0x0` (`je` at 0x10b340),
  reaching `_kbdStatInit` with candidate zero `ebp` (`call` at 0x10b358). The next branch is taken
  when `[sp+4] == 0x1` (`je` at 0x10b345), reaching `_kbdLedSet` with candidate zero `ebp` (`call`
  at 0x10b368). The separate exits retain `[sp+4]` (`ret` at 0x10b34a), `ret(_kbdStatInit)` (`ret`
  at 0x10b360), and `ret(_kbdLedSet)` (`ret` at 0x10b370).

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

- `_kbdIntr`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ae88 to 0x10aed8.
  Established. The read-modify-write at absolute `0x1a9cd4` has an unknown stored four-byte value
  (`inc` at 0x10ae8b). The two `_sysInByte` sites present candidate zero `0x64` (`call` at 0x10ae93)
  and `0x60` (`call` at 0x10aea1). A four-byte store writes `0x1` at absolute `0x1a9ce8` (`mov` at
  0x10aead). The `_kbdConvChar` site retains unknown candidates (`call` at 0x10aece). Interrupt and
  acknowledgement meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `inc` at 0x10ae8b | load | 4 | `abs 0x1a9cd4` | `-` |
| `inc` at 0x10ae8b | store | 4 | `abs 0x1a9cd4` | `?` |
| `mov` at 0x10aead | store | 4 | `abs 0x1a9ce8` | `0x1` |

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

- `_kbdConvChar`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10aed8 to 0x10afa8.
  Established. The opening branch falls through when `([sp+4]:1 & 0xff) == 0xe0` (`jne` at
  0x10aee1), reaching a byte read-modify-write at absolute `0x1d2740` with an unknown stored value
  (`or` at 0x10aee3). Two-byte read-modify-write stores at that address also retain unknown values
  (`or` at 0x10aefd, `and` at 0x10af08, `xor` at 0x10af28). The indirect call has an unknown callee
  and unknown candidates (`call` at 0x10af9d). No scan-code interpretation is assigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10aedb | load | 1 | `sp +0x4` | `-` |
| `or` at 0x10aee3 | load | 1 | `abs 0x1d2740` | `-` |
| `or` at 0x10aee3 | store | 1 | `abs 0x1d2740` | `?` |
| `or` at 0x10aefd | load | 2 | `abs 0x1d2740` | `-` |
| `or` at 0x10aefd | store | 2 | `abs 0x1d2740` | `?` |
| `and` at 0x10af08 | load | 2 | `abs 0x1d2740` | `-` |
| `and` at 0x10af08 | store | 2 | `abs 0x1d2740` | `?` |
| `cmp` at 0x10af16 | load | 1 | `abs 0x1d2740` | `-` |
| `xor` at 0x10af28 | load | 2 | `abs 0x1d2740` | `-` |
| `xor` at 0x10af28 | store | 2 | `abs 0x1d2740` | `?` |
| `xor` at 0x10af31 | load | 1 | `abs 0x1d2740` | `-` |
| `xor` at 0x10af31 | store | 1 | `abs 0x1d2740` | `?` |
| `test` at 0x10af5b | load | 1 | `abs 0x1d2740` | `-` |
| `mov` at 0x10af68 | load | 1 | `(unknown)` | `-` |
| `xor` at 0x10af78 | load | 1 | `abs 0x1d2740` | `-` |
| `xor` at 0x10af78 | store | 1 | `abs 0x1d2740` | `?` |
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
| `jge` at 0x10af1d | `([0x1d2740]:1 & 0xff) >= 0x0 (signed, 8-bit)` | `([0x1d2740]:1 & 0xff) < 0x0 (signed, 8-bit)` |
| `je` at 0x10af62 | `(([0x1d2740]:1 & 0xff) & 0x20) == 0x0` | `(([0x1d2740]:1 & 0xff) & 0x20) != 0x0` |
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

- `_kbdLedSet`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b2c8 to 0x10b338.
  Established. Byte stores at `sp -0x8` (`mov` at 0x10b2d8) and absolute `0x1a9ce4` (`mov` at
  0x10b2e3) retain unknown values. The first `_sysOutByte` site presents leading candidates `0x60`
  and `0xffffffed` (`call` at 0x10b304). The second presents `0x60` and unknown (`call` at
  0x10b32b). The intervening backward branch is taken when `? <= 0x63 (signed, 32-bit)` (`jle` at
  0x10b322). Neither an LED mask nor a retry count is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b2cf | load | 1 | `abs 0x1d2740` | `-` |
| `mov` at 0x10b2d8 | store | 1 | `sp -0x8` | `?` |
| `cmp` at 0x10b2db | load | 1 | `abs 0x1a9ce4` | `-` |
| `mov` at 0x10b2e3 | store | 1 | `abs 0x1a9ce4` | `?` |
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
| `je` at 0x10b2e1 | `([0x1a9ce4]:1 & 0xff) == ?` | `([0x1a9ce4]:1 & 0xff) != ?` |
| `je` at 0x10b2f8 | `(? & 0x2) == 0x0` | `(? & 0x2) != 0x0` |
| `jle` at 0x10b2fe | `? <= 0x63 (signed, 32-bit)` | `? > 0x63 (signed, 32-bit)` |
| `je` at 0x10b31c | `(? & 0x2) == 0x0` | `(? & 0x2) != 0x0` |
| `jle` at 0x10b322 | `? <= 0x63 (signed, 32-bit)` | `? > 0x63 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b336 | `eax` | `?` |

## `_kbdWdog`

- `_kbdWdog`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b5f8 to 0x10b618.
  Established. The four-byte store at absolute `0x1a9cdc` writes `0x1` (`mov` at 0x10b5fb). The
  read-modify-write at absolute `0x1a9cd8` retains an unknown stored four-byte value (`inc` at
  0x10b605). The exit carries the entry expression `eax` (`ret` at 0x10b60e). Those assignments do
  not by themselves establish a watchdog-expiry role.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b5fb | store | 4 | `abs 0x1a9cdc` | `0x1` |
| `inc` at 0x10b605 | load | 4 | `abs 0x1a9cd8` | `-` |
| `inc` at 0x10b605 | store | 4 | `abs 0x1a9cd8` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b60e | `eax` | `eax` |
