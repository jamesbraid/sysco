# MMSC BSP serial and PIC helpers in IRIX 6.5

Read from `6.5/usr/cpu/firmware/mmscfw.bin` (IRIX 6.5).
Image SHA-256: `2017f743fb7360a160f4252b69569b7f9713d9efad82719cfb2fd82538e37ff8`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred end and can include bytes after returns.
Statements below concern sites reached by known control-flow edges.
The tool reports no unresolved transfers in these fifteen spans.
I/O prefixes in the access table identify ports rather than memory addresses.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/bsp-serial-pic-6.5-facts.txt) include all tool sections.

## `_sysSerialChanGet`

- `_sysSerialChanGet`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b7c8 to 0x10b808.
  Established. The signed branch takes `[sp+4] <= 0x5` and falls through for greater values (`jle`
  at 0x10b7d1). The fall-through exit carries EAX `0xffffffff` (`ret` at 0x10b7db). The taken route
  reaches another exit with unknown EAX (`ret` at 0x10b800). The returned address and the
  validity of negative indices remain unknown. The bytes at 0x10b801 through
  0x10b807 are `50 43 20 34 38 36 00` and are not reached by known edges.
  The linear listing decodes through 0x10b805 and leaves the bytes at
  0x10b806 and 0x10b807 (`36 00`) undecoded.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b7cb | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x10b7d1 | `[sp+4] <= 0x5 (signed, 32-bit)` | `[sp+4] > 0x5 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b7db | `eax` | `0xffffffff` |
| `ret` at 0x10b800 | `eax` | `?` |

## `_sysSerialHwInit`

- `_sysSerialHwInit`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b618 to 0x10b758.
  Established. A two-byte store writes zero at an unknown effective target (`mov` at 0x10b640). The
  following four-byte store has unknown target and value `0x1c2000` (`mov` at 0x10b647).
  `_ns16550DevInit` presents leading candidate `[sp-12]` (`call` at 0x10b720). The loop branch takes
  `[sp-8] <= 0x5 (signed, 32-bit)` and falls through for greater values (`jle` at 0x10b73c). The
  record locations and the meaning of `0x1c2000` remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b621 | store | 4 | `sp -0x8` | `0x0` |
| `mov` at 0x10b632 | store | 4 | `sp -0xc` | `0x1d7318` |
| `mov` at 0x10b640 | store | 2 | `(unknown)` | `0x0` |
| `mov` at 0x10b647 | store | 4 | `(unknown)` | `0x1c2000` |
| `mov` at 0x10b64f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b653 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10b657 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10b65b | store | 1 | `(unknown)` | `?` |
| `mov` at 0x10b65f | load | 2 | `(unknown)` | `-` |
| `mov` at 0x10b664 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x10b669 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b66d | store | 4 | `(unknown)` | `?` |
| `movzx` at 0x10b671 | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b676 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b67a | store | 4 | `(unknown)` | `?` |
| `movzx` at 0x10b67e | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b685 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b689 | store | 4 | `(unknown)` | `?` |
| `movzx` at 0x10b68d | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b694 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b698 | store | 4 | `(unknown)` | `?` |
| `movzx` at 0x10b69c | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b6a4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b6a8 | store | 4 | `(unknown)` | `?` |
| `movzx` at 0x10b6ac | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b6b4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b6b8 | store | 4 | `(unknown)` | `?` |
| `movzx` at 0x10b6bc | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b6c4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b6c8 | store | 4 | `(unknown)` | `?` |
| `movzx` at 0x10b6cc | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b6d6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b6da | store | 4 | `(unknown)` | `?` |
| `movzx` at 0x10b6de | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b6ec | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b6f0 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10b6f4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b6f8 | store | 4 | `(unknown)` | `?` |
| `movzx` at 0x10b6fc | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b701 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b705 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10b709 | store | 4 | `(unknown)` | `0x1081d0` |
| `mov` at 0x10b711 | store | 4 | `(unknown)` | `0x1081b0` |
| `mov` at 0x10b719 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x10b71d | store | 4 | `sp -0x10` | `?` |
| `add` at 0x10b728 | load | 4 | `sp -0xc` | `-` |
| `add` at 0x10b728 | store | 4 | `sp -0xc` | `[sp-12]+108` |
| `inc` at 0x10b732 | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x10b732 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x10b735 | load | 4 | `sp -0x10` | `-` |
| `cmp` at 0x10b738 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b720 | `_ns16550DevInit 0x177a5c` | `[sp-12]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x10b73c | `[sp-8] <= 0x5 (signed, 32-bit)` | `[sp-8] > 0x5 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b74b | `eax` | `ret(_ns16550DevInit)` |

## `_sysSerialHwInit2`

- `_sysSerialHwInit2`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b758 to 0x10b7c8.
  Established. The branch takes `Z == 1` and falls through for `not (Z == 1)` (`je` at 0x10b778).
  The fall-through calls `_intConnect` with candidates beginning unknown, `0x17877c`, and unknown
  (`call` at 0x10b787), then `_sysIntEnablePIC` with unknown candidates (`call` at 0x10b797). The
  loop branch takes `[sp-8] <= 0x5 (signed, 32-bit)` and falls through for larger values (`jle` at
  0x10b7af). The selected interrupts and controller channels remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b761 | store | 4 | `sp -0x8` | `0x0` |
| `cmp` at 0x10b774 | load | 2 | `(unknown)` | `-` |
| `movzx` at 0x10b780 | load | 2 | `(unknown)` | `-` |
| `movzx` at 0x10b78f | load | 2 | `(unknown)` | `-` |
| `inc` at 0x10b7a8 | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x10b7a8 | store | 4 | `sp -0x8` | `?` |
| `cmp` at 0x10b7ab | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b787 | `_intConnect 0x178b60` | `?` | `0x17877c` | `?` | `?` | `?` | `?` |
| `call` at 0x10b797 | `_sysIntEnablePIC 0x108428` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10b778 | `Z == 1` | `not (Z == 1)` |
| `jle` at 0x10b7af | `[sp-8] <= 0x5 (signed, 32-bit)` | `[sp-8] > 0x5 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b7ba | `eax` | `?` |

## `_sysIntInitPIC`

- `_sysIntInitPIC`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1082f8 to 0x108398.
  Established. The `_sysOutByte` call candidates begin `0x20`, `0x11` (`call` at 0x1082ff), followed
  by another call whose candidates begin `0x21`, unknown (`call` at 0x108311). A further call
  presents leading candidates `0x21` and `0x4` (`call` at 0x10831d). EAX retains `ret(_sysOutByte)`
  at the return (`ret` at 0x108392). The interrupt-controller configuration is unknown beyond the
  recorded call sequence.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movsx` at 0x108307 | load | 1 | `abs 0x1ae770` | `-` |
| `mov` at 0x108340 | load | 1 | `abs 0x1ae770` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1082ff | `_sysOutByte 0x1081d0` | `0x20` | `0x11` | `ebp` | `?` | `?` | `?` |
| `call` at 0x108311 | `_sysOutByte 0x1081d0` | `0x21` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10831d | `_sysOutByte 0x1081d0` | `0x21` | `0x4` | `?` | `?` | `?` | `?` |
| `call` at 0x108329 | `_sysOutByte 0x1081d0` | `0x21` | `0x1` | `?` | `?` | `?` | `?` |
| `call` at 0x108338 | `_sysOutByte 0x1081d0` | `0xa0` | `0x11` | `?` | `?` | `?` | `?` |
| `call` at 0x108351 | `_sysOutByte 0x1081d0` | `0xa1` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108360 | `_sysOutByte 0x1081d0` | `0xa1` | `0x2` | `?` | `?` | `?` | `?` |
| `call` at 0x10836f | `_sysOutByte 0x1081d0` | `0xa1` | `0x1` | `?` | `?` | `?` | `?` |
| `call` at 0x10837b | `_sysOutByte 0x1081d0` | `0x21` | `0xfffffffb` | `?` | `?` | `?` | `?` |
| `call` at 0x10838a | `_sysOutByte 0x1081d0` | `0xa1` | `0xffffffff` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108392 | `eax` | `ret(_sysOutByte)` |

## `_sysIntEOI`

- `_sysIntEOI`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108398 to 0x1083b8.
  Established. Calling `_sysOutByte` first presents leading candidates `0x20` and `0x20` (`call` at
  0x10839f). A second call presents `0xa0` and `0x20` (`call` at 0x1083ae). The resulting EAX return
  expression is `ret(_sysOutByte)` (`ret` at 0x1083b6). The physical acknowledgement semantics are
  unknown from the caller.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10839f | `_sysOutByte 0x1081d0` | `0x20` | `0x20` | `ebp` | `?` | `?` | `?` |
| `call` at 0x1083ae | `_sysOutByte 0x1081d0` | `0xa0` | `0x20` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1083b6 | `eax` | `ret(_sysOutByte)` |

## `_sysIntDisablePIC`

- `_sysIntDisablePIC`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1083b8 to 0x108428.
  Established. The branch takes `[sp+4] > 0x7 (signed, 32-bit)` and falls through for values at most
  `0x7` (`jg` at 0x1083c2); its fall-through calls `_sysInByte` with leading candidate `0x21`
  (`call` at 0x1083c6). The taken arm instead uses `0xa1` (`call` at 0x1083ed). The joined
  `_sysOutByte` call has unknown candidates (`call` at 0x10840c). EAX is zero at the return (`ret`
  at 0x108419). The final output mask and peripheral effect remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1083bc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x108413 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1083c6 | `_sysInByte 0x1081b0` | `0x21` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x1083ed | `_sysInByte 0x1081b0` | `0xa1` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x10840c | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x1083c2 | `[sp+4] > 0x7 (signed, 32-bit)` | `[sp+4] <= 0x7 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108419 | `eax` | `0x0` |

## `_sysIntEnablePIC`

- `_sysIntEnablePIC`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108428 to 0x108498.
  Established. The signed `[sp+4] > 0x7` edge is taken and the complementary edge falls through
  (`jg` at 0x108432). Alternative calls to `_sysInByte` have leading candidates `0x21` (`call` at
  0x108436) or `0xa1` (`call` at 0x10845d). A later `_sysOutByte` call has unknown candidate slots
  (`call` at 0x10847c). The routine returns EAX `0x0` (`ret` at 0x108489), without establishing the
  hardware enable mask.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10842c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x108483 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108436 | `_sysInByte 0x1081b0` | `0x21` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x10845d | `_sysInByte 0x1081b0` | `0xa1` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x10847c | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x108432 | `[sp+4] > 0x7 (signed, 32-bit)` | `[sp+4] <= 0x7 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108489 | `eax` | `0x0` |

## `_sysInByte`

- `_sysInByte`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081b0 to 0x1081c0.
  Established. An I/O access reads one byte from an unknown effective port (`in` at 0x1081b9). The
  return value in EAX is unknown (`ret` at 0x1081bc). This establishes byte port input, with device
  selection unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1081b0 | load | 4 | `sp +0x4` | `-` |
| `in` at 0x1081b9 | load | 1 | `I/O (unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1081bc | `eax` | `?` |

## `_sysInWord`

- `_sysInWord`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081c0 to 0x1081d0.
  Established. The I/O read transfers two bytes from an unresolved port address (`in` at 0x1081c9).
  EAX is unknown at the exit (`ret` at 0x1081cd). The word input primitive is established, but the
  device mapping remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1081c0 | load | 4 | `sp +0x4` | `-` |
| `in` at 0x1081c9 | load | 2 | `I/O (unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1081cd | `eax` | `?` |

## `_sysOutByte`

- `_sysOutByte`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081d0 to 0x1081e0.
  Established. The I/O store writes a byte of unknown value at an unknown effective port (`out` at
  0x1081d8). At return, EAX contains `[sp+8]` (`ret` at 0x1081db). Byte output is established
  without identifying its selected peripheral.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1081d0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1081d4 | load | 4 | `sp +0x8` | `-` |
| `out` at 0x1081d8 | store | 1 | `I/O (unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1081db | `eax` | `[sp+8]` |

## `_sysOutWord`

- `_sysOutWord`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081e0 to 0x1081f0.
  Established. An I/O store sends two bytes, with both the effective port and value unknown (`out`
  at 0x1081e8). EAX retains `[sp+8]` at the exit (`ret` at 0x1081ec). The word output operation is
  established; its hardware destination remains unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1081e0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1081e4 | load | 4 | `sp +0x8` | `-` |
| `out` at 0x1081e8 | store | 2 | `I/O (unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1081ec | `eax` | `[sp+8]` |

## `_sysNvRamGet`

- `_sysNvRamGet`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1087c8 to 0x1087d8.
  Established. The immediate assignment sets EAX to `0xffffffff` (`mov` at 0x1087cb), which is the
  exit value (`ret` at 0x1087d3). The NVRAM-read service implied by the name remains unknown beyond
  that constant-result stub.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1087d3 | `eax` | `0xffffffff` |

## `_sysNvRamSet`

- `_sysNvRamSet`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1087d8 to 0x1087e8.
  Established. EAX receives `0xffffffff` (`mov` at 0x1087db) and reaches the return with that value
  (`ret` at 0x1087e3). This body establishes a constant result. The NVRAM-write service semantics
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1087e3 | `eax` | `0xffffffff` |

## `_sysIntDisable`

- `_sysIntDisable`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10bb68 to 0x10bb78.
  Established. The body places `0xffffffff` in EAX (`mov` at 0x10bb6b), then returns it (`ret` at
  0x10bb73). No interrupt-disable effect is established by that result, so the hardware role remains
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10bb73 | `eax` | `0xffffffff` |

## `_sysIntEnable`

- `_sysIntEnable`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10bb78 to 0x10bb88.
  Established. At return EAX is `0xffffffff` (`ret` at 0x10bb83), supplied by the immediate
  assignment (`mov` at 0x10bb7b). The interrupt-enable role is unknown from this short
  constant-result body.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10bb83 | `eax` | `0xffffffff` |
