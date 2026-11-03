# MMSC serial and interrupt helpers in patch SG0002378

Read from `patches/SG0002378/usr/cpu/firmware/mmscfw.bin` (patch SG0002378).
Image SHA-256: `b1249e58cf4f00c252362da8ed0515e25bb94aaf096bb60a5c5082c9b6e4d342`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred end and can include literals or padding
after returns. Statements below concern sites reached by known control-flow edges.
The known-edge CFGs have no unresolved jump entries.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/bsp-serial-pic-sg0002378-facts.txt) include all tool sections.

I/O prefixes identify ports rather than memory addresses.
The channel lookup span ends with two undecoded raw bytes:

| Span | Raw-byte start | Exclusive end | Bytes |
|---|---|---|---|
| `_sysSerialChanGet` | `0x10b806` | `0x10b808` | `36 00` |

## `_sysSerialChanGet`

- `_sysSerialChanGet`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b7c8 to 0x10b808.
  Established. The signed branch takes `[sp+4] <= 0x5 (signed, 32-bit)` and falls through for
  greater values (`jle` at 0x10b7d1). The fall-through exit has EAX `0xffffffff` (`ret` at
  0x10b7db), while the taken route reaches an unknown EAX result (`ret` at 0x10b800). The selected
  record address remains unresolved, including the behavior for negative indices.

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

- `_sysSerialHwInit`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b618 to 0x10b758.
  Established. A two-byte store writes `0x0` at an unknown effective target (`mov` at 0x10b640). The
  next four-byte store writes `0x1c2000`, also at an unknown target (`mov` at 0x10b647). The
  `_ns16550DevInit` candidates begin `[sp-12]` (`call` at 0x10b720); its loop branch takes `[sp-8]
  <= 0x5 (signed, 32-bit)` and falls through for greater values (`jle` at 0x10b73c). The indexed
  layout and the constant purpose remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b621 | store | 4 | `sp -0x8` | `0x0` |
| `mov` at 0x10b632 | store | 4 | `sp -0xc` | `0x1d4ae0` |
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
| `call` at 0x10b720 | `_ns16550DevInit 0x175294` | `[sp-12]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x10b73c | `[sp-8] <= 0x5 (signed, 32-bit)` | `[sp-8] > 0x5 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b74b | `eax` | `ret(_ns16550DevInit)` |

## `_sysSerialHwInit2`

- `_sysSerialHwInit2`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b758 to 0x10b7c8.
  Established. The first branch takes `Z == 1` and falls through for `not (Z == 1)` (`je` at
  0x10b778); its fall-through includes `_intConnect` with leading candidates unknown, `0x175fb4`,
  and unknown (`call` at 0x10b787), followed by `_sysIntEnablePIC` with unknown candidates (`call`
  at 0x10b797). A later loop decision takes `[sp-8] <= 0x5 (signed, 32-bit)` and falls through for
  greater values (`jle` at 0x10b7af). The selected interrupt and channel identities remain unknown.

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
| `call` at 0x10b787 | `_intConnect 0x176398` | `?` | `0x175fb4` | `?` | `?` | `?` | `?` |
| `call` at 0x10b797 | `_sysIntEnablePIC 0x108428` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10b778 | `Z == 1` | `not (Z == 1)` |
| `jle` at 0x10b7af | `[sp-8] <= 0x5 (signed, 32-bit)` | `[sp-8] > 0x5 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b7ba | `eax` | `?` |

## `_sysIntInitPIC`

- `_sysIntInitPIC`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1082f8 to 0x108398.
  Established. The `_sysOutByte` call candidates include leading pairs `0x20`, `0x11` (`call` at
  0x1082ff), `0x21`, `0x4` (`call` at 0x10831d), and `0xa0`, `0x11` (`call` at 0x108338). Two later
  `_sysOutByte` sites begin `0x21`, `0xfffffffb` (`call` at 0x10837b) and `0xa1`, `0xffffffff`
  (`call` at 0x10838a). EAX returns the last `ret(_sysOutByte)` (`ret` at 0x108392). The numeric
  command meanings are not established by this body.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movsx` at 0x108307 | load | 1 | `abs 0x1abfa8` | `-` |
| `mov` at 0x108340 | load | 1 | `abs 0x1abfa8` | `-` |

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

- `_sysIntEOI`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108398 to 0x1083b8.
  Established. Calls `_sysOutByte` with leading candidates `0x20`, `0x20` (`call` at 0x10839f),
  followed by `0xa0`, `0x20` (`call` at 0x1083ae). The return carries EAX `ret(_sysOutByte)` (`ret`
  at 0x1083b6). An interrupt acknowledgment effect is unknown without the callee and port contract.

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

- `_sysIntDisablePIC`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1083b8 to 0x108428.
  Established. The decision takes `[sp+4] > 0x7 (signed, 32-bit)` and falls through for lesser or
  equal values (`jg` at 0x1083c2). The taken route calls `_sysInByte` with leading candidate `0xa1`
  (`call` at 0x1083ed), while the fall-through uses `0x21` (`call` at 0x1083c6). The merged
  `_sysOutByte` candidates are all unknown (`call` at 0x10840c). The resulting bit mask and
  interrupt effect remain unknown.

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

- `_sysIntEnablePIC`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108428 to 0x108498.
  Established. The comparison takes `[sp+4] > 0x7 (signed, 32-bit)` and falls through for lesser or
  equal values (`jg` at 0x108432). The taken `_sysInByte` site begins `0xa1` (`call` at 0x10845d),
  whereas the fall-through site begins `0x21` (`call` at 0x108436). After merging, `_sysOutByte` has
  unknown candidates (`call` at 0x10847c). The rotation-derived mask is unresolved
  (`rol` at 0x108447, 0x10846f).

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

- `_sysInByte`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081b0 to 0x1081c0.
  Established. The byte I/O load uses an unknown effective port (`in` at 0x1081b9). The return
  carries unknown EAX (`ret` at 0x1081bc). No concrete port or peripheral can be assigned from these
  facts.

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

- `_sysInWord`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081c0 to 0x1081d0.
  Established. A two-byte I/O read has unknown effective port (`in` at 0x1081c9). The following
  known return has unknown EAX (`ret` at 0x1081cd). The dynamic port identity remains unresolved.

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

- `_sysOutByte`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081d0 to 0x1081e0.
  Established. The byte I/O store writes an unknown value to an unknown effective port (`out` at
  0x1081d8). EAX returns `[sp+8]` (`ret` at 0x1081db). That returned four-byte expression does not
  resolve the byte written.

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

- `_sysOutWord`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081e0 to 0x1081f0.
  Established. The two-byte I/O store has unknown effective port and unknown stored value (`out` at
  0x1081e8). The static return carries EAX `[sp+8]` (`ret` at 0x1081ec). The emitted half of that
  expression remains unknown in the tool output.

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

- `_sysNvRamGet`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1087c8 to 0x1087d8.
  Established. The return carries EAX `0xffffffff` (`ret` at 0x1087d3). Only stack accesses appear
  in the routine facts (`push` at 0x1087c8, `pop` at 0x1087d2). A hardware NVRAM read is not established by this body.

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

- `_sysNvRamSet`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1087d8 to 0x1087e8.
  Established. EAX reaches the return as `0xffffffff` (`ret` at 0x1087e3). The access list contains
  the prologue and epilogue stack operations (`push` at 0x1087d8, `pop` at 0x1087e2). The name alone does not establish a storage update.

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

- `_sysIntDisable`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10bb68 to 0x10bb78.
  Established. The static EAX result is `0xffffffff` at the return (`ret` at 0x10bb73). Apart from
  the stack frame, the body has no recorded access or call. A physical interrupt-disable effect
  remains unknown.

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

- `_sysIntEnable`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10bb78 to 0x10bb88.
  Established. Returns EAX `0xffffffff` (`ret` at 0x10bb83) after restoration of the frame (`mov` at 0x10bb80, `pop` at 0x10bb82). The body
  contains no recorded I/O or call. An interrupt-enable role is not proven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10bb83 | `eax` | `0xffffffff` |
