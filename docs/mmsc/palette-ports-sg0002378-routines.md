# MMSC palette and port paths in patch SG0002378

Read from `patches/SG0002378/usr/cpu/firmware/mmscfw.bin` (patch SG0002378).
Image SHA-256: `b1249e58cf4f00c252362da8ed0515e25bb94aaf096bb60a5c5082c9b6e4d342`.
This image has the x86 a.out link address `0x108000`. The [load map](load-map.md) explains segment mapping.
Most analysed spans end at the next text symbol, with exclusive inferred ends.
The `_mwVGAHWPal` and `_mwVGAHRPal` spans extend through shared continuations.
Those two given extents overlap the following entry rather than ending at its symbol.
They can include literals or padding after returns. The raw zero bytes at `0x14add7`
and `0x158077` remain outside the decoded instructions. Statements below concern sites
reached by known control-flow edges. The unnamed helpers at `0x14ad05` and `0x14ad4e`
are included in the `_mw8514RPal` listing but are not reached from that entry.
Their effects are not credited to the callers below.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The tables retain analysis limits for partial registers, self-subtraction and unsupported
instructions. An unknown result does not establish that the firmware produces an indeterminate value.
The access tables include explicit memory operands, including stack records.
The full facts also include implicit push/pop accesses. Call and return instructions
remain in the listings without separate implicit stack access rows.
Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/palette-ports-sg0002378-facts.txt) include all tool sections.

## `_mwCGAPal`

- `_mwCGAPal`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14a998 to 0x14aa00.
  Established. The four-byte load at `ebp +0x10` (`mov` at 0x14a99a) supplies the base for the
  four-byte load at `[ebp+16] +0x4` (`mov` at 0x14a99d). The first branch is taken when `Z == 0`
  (`jne` at 0x14a9a2). Two software-interrupt sites use immediate `0x10` (`int` at 0x14a9e6, `int`
  at 0x14a9f9). Their service effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14a99a | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14a99d | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14a9a4 | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14a9a9 | load | 2 | `[[ebp+16]+12] +0x0` | `-` |
| `mov` at 0x14a9bc | load | 2 | `[[ebp+16]+12] +0x2` | `-` |
| `mov` at 0x14a9d0 | load | 2 | `[[ebp+16]+12] +0x4` | `-` |
| `mov` at 0x14a9e8 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14a9eb | load | 4 | `[ebp+16] +0x0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x14a9a2 | `Z == 0` | `not (Z == 0)` |
| `je` at 0x14a9af | `(? & 0x80) == 0x0` | `(? & 0x80) != 0x0` |
| `je` at 0x14a9b7 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x14a9c3 | `(? & 0x80) == 0x0` | `(? & 0x80) != 0x0` |
| `je` at 0x14a9cb | `Z == 1` | `not (Z == 1)` |
| `je` at 0x14a9d7 | `(? & 0x80) == 0x0` | `(? & 0x80) != 0x0` |
| `je` at 0x14a9df | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x14a9f1 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14a9ff | `eax` | `?` |

## `_mwEGAWPal`

- `_mwEGAWPal`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14aa00 to 0x14aab4.
  Established. The first range branch is taken when `[[ebp+16]+8] <= 0xf (unsigned, 32-bit)` (`jbe`
  at 0x14aa11). The byte input uses I/O absolute `0x3da` (`in` at 0x14aa2c). The byte output sites
  include unknown values and unknown I/O addresses (`out` at 0x14aa7f, `out` at 0x14aa82), followed
  by value `0x20` at an unknown I/O address (`out` at 0x14aa8f). The byte store has unknown
  address and value (`mov` at 0x14aa76).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14aa05 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14aa08 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14aa0b | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14aa21 | load | 1 | `[ebp+16] +0x0` | `-` |
| `in` at 0x14aa2c | load | 1 | `I/O abs 0x3da` | `-` |
| `mov` at 0x14aa31 | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14aa3d | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14aa50 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14aa63 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14aa76 | store | 1 | `(unknown)` | `?` |
| `out` at 0x14aa7f | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14aa82 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14aa8f | store | 1 | `I/O (unknown)` | `0x20` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x14aa11 | `[[ebp+16]+8] <= 0xf (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xf (unsigned, 32-bit)` |
| `jae` at 0x14aa1a | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14aa26 | `([[ebp+16]]:1 & 0xff) != 0x0` | `([[ebp+16]]:1 & 0xff) == 0x0` |
| `jne` at 0x14aa7b | `([[ebp+16]]:1 & 0xff) != 0x0` | `([[ebp+16]]:1 & 0xff) == 0x0` |
| `jle` at 0x14aa8b | `? <= ? (signed, 8-bit)` | `? > ? (signed, 8-bit)` |
| `jle` at 0x14aaa6 | `? <= ? (signed, 8-bit)` | `? > ? (signed, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14aa97 | `eax` | `?` |

## `_mwEGARPal`

- `_mwEGARPal`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14aab4 to 0x14ab44.
  Established. A four-byte load reads `[ebp+16] +0xc` (`mov` at 0x14aad5). The two-byte stores have
  unknown effective addresses and values (`mov` at 0x14aaf1, `mov` at 0x14ab0b, `mov` at 0x14ab26).
  The repeat branch is taken when `? <= ? (signed, 8-bit)` (`jle` at 0x14ab32). Neither a
  destination extent nor a component encoding is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14aab8 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14aabb | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14aabe | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14aad5 | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14aad8 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14aaf1 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14ab0b | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14ab26 | store | 2 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x14aac4 | `[[ebp+16]+8] <= 0xf (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xf (unsigned, 32-bit)` |
| `jae` at 0x14aaca | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14aae2 | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14aaeb | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14aafc | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14ab05 | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14ab17 | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14ab20 | `Z == 0` | `not (Z == 0)` |
| `jle` at 0x14ab32 | `? <= ? (signed, 8-bit)` | `? > ? (signed, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14ab3a | `eax` | `?` |

## `_mwEGAOscan`

- `_mwEGAOscan`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14ab44 to 0x14ab6c.
  Established. The byte input reads I/O absolute `0x3da` (`in` at 0x14ab4a). Three byte outputs to
  I/O absolute `0x3c0` carry `0x11` (`out` at 0x14ab51), an unknown value (`out` at 0x14ab60), and
  `0x20` (`out` at 0x14ab63). The intervening byte load has an unknown effective address (`mov` at
  0x14ab5e). Register meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `in` at 0x14ab4a | load | 1 | `I/O abs 0x3da` | `-` |
| `out` at 0x14ab51 | store | 1 | `I/O abs 0x3c0` | `0x11` |
| `mov` at 0x14ab52 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14ab5e | load | 1 | `(unknown)` | `-` |
| `out` at 0x14ab60 | store | 1 | `I/O abs 0x3c0` | `?` |
| `out` at 0x14ab63 | store | 1 | `I/O abs 0x3c0` | `0x20` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14ab68 | `eax` | `?` |

## `_mwVGAHWPal`

- `_mwVGAHWPal`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14ab6c to 0x14ac20, using an explicit 180-byte extent.
  Established. The zero-test branch is taken when `[[ebp+16]] == 0x0` (`je` at 0x14ab7f), reaching
  the shared four-byte load at `ebp +0x8` (`mov` at 0x14ab95); its fall-through instead reaches a
  direct call to `0x14ad05` with unknown callee name (`call` at 0x14ab81), then jumps to
  the restore sequence (`jmp` at 0x14ab86). The shared byte outputs retain unknown addresses and
  values (`out` at 0x14abd2, `out` at 0x14abd7, `out` at 0x14abda, `out` at 0x14abdd).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14ab79 | load | 4 | `ebp +0x10` | `-` |
| `cmp` at 0x14ab7c | load | 4 | `[ebp+16] +0x0` | `-` |
| `mov` at 0x14ab95 | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x14ab98 | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x14ab9c | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14ab9f | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14aba2 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14abb5 | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14abbe | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14abc1 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14abc4 | load | 1 | `(unknown)` | `-` |
| `out` at 0x14abd2 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14abd7 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14abda | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14abdd | store | 1 | `I/O (unknown)` | `?` |
| `mov` at 0x14abfa | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14abfd | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14ac00 | load | 1 | `(unknown)` | `-` |
| `out` at 0x14ac05 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ac0a | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ac0d | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ac10 | store | 1 | `I/O (unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x14ab81 | `(unknown callee) 0x14ad05` | `edi` | `esi` | `edx` | `ecx` | `ebx` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x14ab7f | `[[ebp+16]] == 0x0` | `[[ebp+16]] != 0x0` |
| `jbe` at 0x14abab | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x14abb1 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14abbc | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jae` at 0x14abe2 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x14ac15 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14abf2 | `eax` | `?` |

## `_mwVGAHRPal`

- `_mwVGAHRPal`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14ac20 to 0x14ad05, using an explicit 229-byte extent.
  Established. The first branch falls through when `[[ebp+16]] != 0x0` (`je` at 0x14ac33), reaching
  the direct call to `0x14ad4e` with unknown callee name (`call` at 0x14ac35), then jumps to the
  restore sequence (`jmp` at 0x14ac3a). Taking the first branch reaches the shared two-byte load
  at `[ebp+8] +0x16` (`mov` at 0x14ac4f). The shared byte input
  sites retain unknown I/O addresses (`in` at 0x14ac7a, `in` at 0x14ac7d, `in` at 0x14ac80).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14ac2d | load | 4 | `ebp +0x10` | `-` |
| `cmp` at 0x14ac30 | load | 4 | `[ebp+16] +0x0` | `-` |
| `mov` at 0x14ac4c | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x14ac4f | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x14ac53 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14ac56 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14ac59 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14ac6c | load | 4 | `[ebp+16] +0xc` | `-` |
| `out` at 0x14ac77 | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x14ac7a | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14ac7d | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14ac80 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x14ac92 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14aca4 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14acb7 | store | 2 | `(unknown)` | `?` |
| `out` at 0x14acd7 | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x14acda | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14acdd | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14ace0 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x14ace7 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14acee | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14acf6 | store | 2 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x14ac35 | `(unknown callee) 0x14ad4e` | `edi` | `esi` | `edx` | `ecx` | `ebx` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x14ac33 | `[[ebp+16]] == 0x0` | `[[ebp+16]] != 0x0` |
| `jbe` at 0x14ac62 | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x14ac68 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14ac73 | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jne` at 0x14ac8d | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x14ac9f | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x14acb2 | `? != 0xfc` | `? == 0xfc` |
| `jae` at 0x14acbd | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x14acfc | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14accd | `eax` | `?` |

## `_mwVGAOscan`

- `_mwVGAOscan`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14adc0 to 0x14add8.
  Established. One byte input reads I/O absolute `0x3da` (`in` at 0x14adc5). The two byte outputs to
  I/O absolute `0x3c0` write `0x31` (`out` at 0x14adcc) and an unknown value (`out` at 0x14add0). A
  four-byte load at `ebp +0x10` lies between them (`mov` at 0x14adcd). The role of these port values
  remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `in` at 0x14adc5 | load | 1 | `I/O abs 0x3da` | `-` |
| `out` at 0x14adcc | store | 1 | `I/O abs 0x3c0` | `0x31` |
| `mov` at 0x14adcd | load | 4 | `ebp +0x10` | `-` |
| `out` at 0x14add0 | store | 1 | `I/O abs 0x3c0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14add4 | `eax` | `?` |

## `_mw8514WPal`

- `_mw8514WPal`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14ab88 to 0x14ac20.
  Established. The range branch falls through when `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` (`jbe`
  at 0x14abab), reaching the assignment `eax = 0xffffffff` (`mov` at 0x14abf3) through a jump
  (`jmp` at 0x14abad).
  When both range guards take their branches (`jbe` at 0x14abab, `jae` at 0x14abb1), the selection
  branch is reached. It is taken when `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` (`jne` at
  0x14abbc). Each output arm includes four byte outputs with unknown I/O addresses
  and values (`out` at 0x14abd2, `out` at 0x14abd7, `out` at 0x14abda, `out` at 0x14abdd, `out` at
  0x14ac05, `out` at 0x14ac0a, `out` at 0x14ac0d, `out` at 0x14ac10). No output encoding is
  assigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14ab95 | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x14ab98 | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x14ab9c | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14ab9f | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14aba2 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14abb5 | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14abbe | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14abc1 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14abc4 | load | 1 | `(unknown)` | `-` |
| `out` at 0x14abd2 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14abd7 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14abda | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14abdd | store | 1 | `I/O (unknown)` | `?` |
| `mov` at 0x14abfa | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14abfd | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14ac00 | load | 1 | `(unknown)` | `-` |
| `out` at 0x14ac05 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ac0a | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ac0d | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ac10 | store | 1 | `I/O (unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x14abab | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x14abb1 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14abbc | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jae` at 0x14abe2 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x14ac15 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14abf2 | `eax` | `?` |

## `_mw8514RPal`

- `_mw8514RPal`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14ac3f to 0x14adc0.
  Established. The initial two-byte load uses `[ebp+8] +0x16` (`mov` at 0x14ac4f). The fall-through arm includes
  a byte output (`out` at 0x14ac77), three byte inputs (`in` at 0x14ac7a, `in` at 0x14ac7d, `in` at
  0x14ac80), and two-byte stores (`mov` at 0x14ac92, `mov` at 0x14aca4, `mov` at 0x14acb7), all at
  unknown effective addresses. The arm-selection branch is taken when `(([[ebp+8]+22]:2 & 0xffff) &
  0x100) != 0x0` (`jne` at 0x14ac73). The output value and stored values remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14ac4c | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x14ac4f | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x14ac53 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14ac56 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14ac59 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14ac6c | load | 4 | `[ebp+16] +0xc` | `-` |
| `out` at 0x14ac77 | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x14ac7a | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14ac7d | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14ac80 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x14ac92 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14aca4 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14acb7 | store | 2 | `(unknown)` | `?` |
| `out` at 0x14acd7 | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x14acda | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14acdd | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14ace0 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x14ace7 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14acee | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14acf6 | store | 2 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x14ac62 | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x14ac68 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14ac73 | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jne` at 0x14ac8d | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x14ac9f | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x14acb2 | `? != 0xfc` | `? == 0xfc` |
| `jae` at 0x14acbd | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x14acfc | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14accd | `eax` | `?` |

## `_vga_setpalette`

- `_vga_setpalette`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16efd8 to 0x16f048.
  Established. The `_sysOutByte` sites present candidate zero `0x3c8` (`call` at 0x16efe5) followed
  by `0x3c9` (`call` at 0x16f001, `call` at 0x16f01d, `call` at 0x16f039). Candidate one remains
  unknown at each site. The first repeat edge is taken when `N == 0` (`jns` at 0x16eff5). The exit
  reports `0x0` in `eax` (`ret` at 0x16f043).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movsx` at 0x16efdb | load | 1 | `sp +0x4` | `-` |
| `movsx` at 0x16eff7 | load | 1 | `sp +0x8` | `-` |
| `movsx` at 0x16f013 | load | 1 | `sp +0xc` | `-` |
| `movsx` at 0x16f02f | load | 1 | `sp +0x10` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16efe5 | `_sysOutByte 0x1081d0` | `0x3c8` | `?` | `ebp` | `?` | `?` | `?` |
| `call` at 0x16f001 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16f01d | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16f039 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jns` at 0x16eff5 | `N == 0` | `not (N == 0)` |
| `jns` at 0x16f011 | `N == 0` | `not (N == 0)` |
| `jns` at 0x16f02d | `N == 0` | `not (N == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16f043 | `eax` | `0x0` |

## `_vga_getpalette`

- `_vga_getpalette`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16f048 to 0x16f0c8.
  Established. The `_sysOutByte` site has candidate zero `0x3c7` and unknown candidate one (`call`
  at 0x16f061). The `_sysInByte` sites each present candidate zero `0x3c9` (`call` at 0x16f078,
  `call` at 0x16f094, `call` at 0x16f0b0). Four-byte stores through `[sp+8] +0x0` (`mov` at
  0x16f085), `[sp+12] +0x0` (`mov` at 0x16f0a1), and `[sp+16] +0x0` (`mov` at 0x16f0ba) retain
  unknown values. No hardware reply is inferred.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16f04e | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x16f051 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x16f054 | load | 4 | `sp +0x10` | `-` |
| `movsx` at 0x16f057 | load | 1 | `sp +0x4` | `-` |
| `mov` at 0x16f085 | store | 4 | `[sp+8] +0x0` | `?` |
| `mov` at 0x16f0a1 | store | 4 | `[sp+12] +0x0` | `?` |
| `mov` at 0x16f0ba | store | 4 | `[sp+16] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16f061 | `_sysOutByte 0x1081d0` | `0x3c7` | `?` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x16f078 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16f094 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16f0b0 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jns` at 0x16f071 | `N == 0` | `not (N == 0)` |
| `jns` at 0x16f08d | `N == 0` | `not (N == 0)` |
| `jns` at 0x16f0a9 | `N == 0` | `not (N == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16f0c7 | `eax` | `0x0` |

## `_vga_setpalvec`

- `_vga_setpalvec`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16f0c8 to 0x16f158.
  Established. The first `_sysOutByte` call presents candidate zero `0x3c8` (`call` at 0x16f0de).
  The entry loop guard is taken when `0x0 >= [sp+8] (signed, 32-bit)` (`jge` at 0x16f0ea). The three
  byte loads in the loop have unknown effective addresses (`movsx` at 0x16f0f7, `movsx` at 0x16f113,
  `movsx` at 0x16f12f); its three later `_sysOutByte` sites present candidate zero `0x3c9` and
  unknown candidate one (`call` at 0x16f103, `call` at 0x16f11f, `call` at 0x16f13b).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16f0ce | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x16f0d1 | load | 4 | `sp +0xc` | `-` |
| `movsx` at 0x16f0d4 | load | 1 | `sp +0x4` | `-` |
| `movsx` at 0x16f0f7 | load | 1 | `(unknown)` | `-` |
| `movsx` at 0x16f113 | load | 1 | `(unknown)` | `-` |
| `movsx` at 0x16f12f | load | 1 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16f0de | `_sysOutByte 0x1081d0` | `0x3c8` | `?` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x16f103 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16f11f | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16f13b | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x16f0ea | `0x0 >= [sp+8] (signed, 32-bit)` | `0x0 < [sp+8] (signed, 32-bit)` |
| `jns` at 0x16f0f5 | `N == 0` | `not (N == 0)` |
| `jns` at 0x16f111 | `N == 0` | `not (N == 0)` |
| `jns` at 0x16f12d | `N == 0` | `not (N == 0)` |
| `jl` at 0x16f146 | `? < [sp+8] (signed, 32-bit)` | `? >= [sp+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16f153 | `eax` | `?` |

## `_vga_getpalvec`

- `_vga_getpalvec`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16f158 to 0x16f1f8.
  Established. The loop is entered through the guard fall-through when `0x0 < [sp+8] (signed,
  32-bit)` (`jge` at 0x16f17a). The three `_sysInByte` sites have candidate zero `0x3c9` (`call` at
  0x16f18c, `call` at 0x16f1ac, `call` at 0x16f1cc). Four-byte stores in that loop have unknown
  addresses and values (`mov` at 0x16f199, `mov` at 0x16f1b9, `mov` at 0x16f1d9). The repeat
  condition retains an unknown left operand: `? < [sp+8] (signed, 32-bit)` (`jl` at 0x16f1e1).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16f15e | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x16f161 | load | 4 | `sp +0xc` | `-` |
| `movsx` at 0x16f164 | load | 1 | `sp +0x4` | `-` |
| `mov` at 0x16f199 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x16f1b9 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x16f1d9 | store | 4 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16f16e | `_sysOutByte 0x1081d0` | `0x3c7` | `?` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x16f18c | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16f1ac | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16f1cc | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x16f17a | `0x0 >= [sp+8] (signed, 32-bit)` | `0x0 < [sp+8] (signed, 32-bit)` |
| `jns` at 0x16f185 | `N == 0` | `not (N == 0)` |
| `jns` at 0x16f1a5 | `N == 0` | `not (N == 0)` |
| `jns` at 0x16f1c5 | `N == 0` | `not (N == 0)` |
| `jl` at 0x16f1e1 | `? < [sp+8] (signed, 32-bit)` | `? >= [sp+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16f1ee | `eax` | `?` |

## `_ReadPalette`

- `_ReadPalette`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x158024 to 0x158078.
  Established. The indirect call operand is a four-byte load from `[[0x1cf30c]] +0xcc` (`call` at
  0x158058). Leading candidates are `[[0x1cf30c]]`, `0x4`, and `sp-32` at that site (`call` at
  0x158058). A later branch is taken when `Z == 1` (`je` at 0x158063), bypassing the unnamed direct
  call to `0x162218` (`call` at 0x15806a). The indirect callee and its effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x15802d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x158030 | store | 4 | `sp -0x20` | `[sp+4]` |
| `mov` at 0x158033 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x158036 | store | 4 | `sp -0x1c` | `[sp+8]` |
| `mov` at 0x158039 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x15803c | store | 4 | `sp -0x18` | `[sp+12]` |
| `mov` at 0x15803f | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x158042 | store | 4 | `sp -0x14` | `[sp+16]` |
| `mov` at 0x158045 | load | 4 | `abs 0x1cf30c` | `-` |
| `mov` at 0x15804b | load | 4 | `[0x1cf30c] +0x0` | `-` |
| `call` at 0x158058 | load | 4 | `[[0x1cf30c]] +0xcc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x158058 | `(unknown callee) (unknown target)` | `[[0x1cf30c]]` | `0x4` | `sp-32` | `[sp+4]` | `[sp+8]` | `[sp+12]` |
| `call` at 0x15806a | `(unknown callee) 0x162218` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x158063 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x158076 | `eax` | `?` |

## `_WritePalette`

- `_WritePalette`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x157fd0 to 0x158024.
  Established. Four-byte stores copy `[sp+4]` to `sp -0x20` (`mov` at 0x157fdc), `[sp+8]` to `sp
  -0x1c` (`mov` at 0x157fe2), `[sp+12]` to `sp -0x18` (`mov` at 0x157fe8), and `[sp+16]` to `sp
  -0x14` (`mov` at 0x157fee). The indirect call presents leading candidates `[[0x1cf30c]]`, `0x3`,
  and `sp-32` (`call` at 0x158004); its target and downstream operation remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x157fd9 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x157fdc | store | 4 | `sp -0x20` | `[sp+4]` |
| `mov` at 0x157fdf | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x157fe2 | store | 4 | `sp -0x1c` | `[sp+8]` |
| `mov` at 0x157fe5 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x157fe8 | store | 4 | `sp -0x18` | `[sp+12]` |
| `mov` at 0x157feb | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x157fee | store | 4 | `sp -0x14` | `[sp+16]` |
| `mov` at 0x157ff1 | load | 4 | `abs 0x1cf30c` | `-` |
| `mov` at 0x157ff7 | load | 4 | `[0x1cf30c] +0x0` | `-` |
| `call` at 0x158004 | load | 4 | `[[0x1cf30c]] +0xcc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x158004 | `(unknown callee) (unknown target)` | `[[0x1cf30c]]` | `0x3` | `sp-32` | `[sp+4]` | `[sp+8]` | `[sp+12]` |
| `call` at 0x158016 | `(unknown callee) 0x162218` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x15800f | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x158022 | `eax` | `?` |
