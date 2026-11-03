# MMSC palette and port paths in IRIX 6.5.2

Read from `6.5.2/usr/cpu/firmware/mmscfw.bin` (IRIX 6.5.2).
Image SHA-256: `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
This image has the x86 a.out link address `0x108000`. The [load map](load-map.md) explains segment mapping.
Most analysed spans end at the next text symbol, with exclusive inferred ends.
The `_mwVGAHWPal` and `_mwVGAHRPal` spans extend through shared continuations.
Those two given extents overlap the following entry rather than ending at its symbol.
They can include literals or padding after returns. Statements below concern sites
reached by known control-flow edges. The unnamed targets `0x14ae05` and `0x14ae4e`
appear in the `_mw8514RPal` listing but are not reached from that entry.
Their instructions are not included in its effect tables.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack locations,
and omit implicit push/pop accesses. The full facts retain those accesses.
Call and return instructions
remain in the listings without separate implicit stack access rows.
Unsupported instructions and partial-register operations can leave values or flags unknown.
Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/palette-ports-652-facts.txt) include all tool sections.

## `_mwCGAPal`

- `_mwCGAPal`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14aa98 to 0x14ab00.
  Established. The first four-byte load at `ebp +0x10` (`mov` at 0x14aa9a) supplies the pointer used
  by the four-byte load at `[ebp+16] +0x4` (`mov` at 0x14aa9d). The branch after that load is taken
  when `Z == 0` (`jne` at 0x14aaa2). Software-interrupt instructions carry immediate `0x10` (`int`
  at 0x14aae6, `int` at 0x14aaf9). No service effect is established by those immediates.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14aa9a | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14aa9d | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14aaa4 | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14aaa9 | load | 2 | `[[ebp+16]+12] +0x0` | `-` |
| `mov` at 0x14aabc | load | 2 | `[[ebp+16]+12] +0x2` | `-` |
| `mov` at 0x14aad0 | load | 2 | `[[ebp+16]+12] +0x4` | `-` |
| `mov` at 0x14aae8 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14aaeb | load | 4 | `[ebp+16] +0x0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x14aaa2 | `Z == 0` | `not (Z == 0)` |
| `je` at 0x14aaaf | `(? & 0x80) == 0x0` | `(? & 0x80) != 0x0` |
| `je` at 0x14aab7 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x14aac3 | `(? & 0x80) == 0x0` | `(? & 0x80) != 0x0` |
| `je` at 0x14aacb | `Z == 1` | `not (Z == 1)` |
| `je` at 0x14aad7 | `(? & 0x80) == 0x0` | `(? & 0x80) != 0x0` |
| `je` at 0x14aadf | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x14aaf1 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14aaff | `eax` | `?` |

## `_mwEGAWPal`

- `_mwEGAWPal`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14ab00 to 0x14abb4.
  Established. The two range guards require taken conditions `[[ebp+16]+8] <= 0xf (unsigned,
  32-bit)` (`jbe` at 0x14ab11) and `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` (`jae` at
  0x14ab1a) to reach the body. The body includes a byte input at I/O absolute `0x3da` (`in` at
  0x14ab2c); its byte outputs retain unknown I/O addresses, with unknown values at two sites (`out`
  at 0x14ab7f, `out` at 0x14ab82) and value `0x20` at another (`out` at 0x14ab8f).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14ab05 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14ab08 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14ab0b | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14ab21 | load | 1 | `[ebp+16] +0x0` | `-` |
| `in` at 0x14ab2c | load | 1 | `I/O abs 0x3da` | `-` |
| `mov` at 0x14ab31 | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14ab3d | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14ab50 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14ab63 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14ab76 | store | 1 | `(unknown)` | `?` |
| `out` at 0x14ab7f | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ab82 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ab8f | store | 1 | `I/O (unknown)` | `0x20` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x14ab11 | `[[ebp+16]+8] <= 0xf (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xf (unsigned, 32-bit)` |
| `jae` at 0x14ab1a | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14ab26 | `([[ebp+16]]:1 & 0xff) != 0x0` | `([[ebp+16]]:1 & 0xff) == 0x0` |
| `jne` at 0x14ab7b | `([[ebp+16]]:1 & 0xff) != 0x0` | `([[ebp+16]]:1 & 0xff) == 0x0` |
| `jle` at 0x14ab8b | `? <= ? (signed, 8-bit)` | `? > ? (signed, 8-bit)` |
| `jle` at 0x14aba6 | `? <= ? (signed, 8-bit)` | `? > ? (signed, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14ab97 | `eax` | `?` |

## `_mwEGARPal`

- `_mwEGARPal`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14abb4 to 0x14ac44.
  Established. A four-byte access loads `[ebp+16] +0xc` (`mov` at 0x14abd5). Three two-byte stores
  in the loop keep their effective addresses and values unknown (`mov` at 0x14abf1, `mov` at
  0x14ac0b, `mov` at 0x14ac26). The repeat branch is taken when `? <= ? (signed, 8-bit)` (`jle` at
  0x14ac32). The tool establishes neither the destination extent nor a component representation.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14abb8 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14abbb | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14abbe | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14abd5 | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14abd8 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14abf1 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14ac0b | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14ac26 | store | 2 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x14abc4 | `[[ebp+16]+8] <= 0xf (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xf (unsigned, 32-bit)` |
| `jae` at 0x14abca | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14abe2 | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14abeb | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14abfc | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14ac05 | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14ac17 | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14ac20 | `Z == 0` | `not (Z == 0)` |
| `jle` at 0x14ac32 | `? <= ? (signed, 8-bit)` | `? > ? (signed, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14ac3a | `eax` | `?` |

## `_mwEGAOscan`

- `_mwEGAOscan`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14ac44 to 0x14ac6c.
  Established. The I/O sequence includes a byte read at absolute `0x3da` (`in` at 0x14ac4a), then
  byte writes at absolute `0x3c0` with values `0x11` (`out` at 0x14ac51), unknown (`out` at
  0x14ac60), and `0x20` (`out` at 0x14ac63). The byte load feeding the middle output has an unknown
  effective address (`mov` at 0x14ac5e). The port roles remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `in` at 0x14ac4a | load | 1 | `I/O abs 0x3da` | `-` |
| `out` at 0x14ac51 | store | 1 | `I/O abs 0x3c0` | `0x11` |
| `mov` at 0x14ac52 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14ac5e | load | 1 | `(unknown)` | `-` |
| `out` at 0x14ac60 | store | 1 | `I/O abs 0x3c0` | `?` |
| `out` at 0x14ac63 | store | 1 | `I/O abs 0x3c0` | `0x20` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14ac68 | `eax` | `?` |

## `_mwVGAHWPal`

- `_mwVGAHWPal`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14ac6c to 0x14ad20.
  Established. The initial branch is taken when `[[ebp+16]] == 0x0` (`je` at 0x14ac7f), reaching the
  shared four-byte load at `ebp +0x8` (`mov` at 0x14ac95); its fall-through instead calls unnamed
  target `0x14ae05` (`call` at 0x14ac81), then jumps to restoration (`jmp` at 0x14ac86). Shared byte
  output sites retain unknown I/O addresses and values (`out` at 0x14acd2, `out` at 0x14acd7, `out`
  at 0x14acda, `out` at 0x14acdd).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14ac79 | load | 4 | `ebp +0x10` | `-` |
| `cmp` at 0x14ac7c | load | 4 | `[ebp+16] +0x0` | `-` |
| `mov` at 0x14ac95 | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x14ac98 | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x14ac9c | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14ac9f | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14aca2 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14acb5 | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14acbe | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14acc1 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14acc4 | load | 1 | `(unknown)` | `-` |
| `out` at 0x14acd2 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14acd7 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14acda | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14acdd | store | 1 | `I/O (unknown)` | `?` |
| `mov` at 0x14acfa | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14acfd | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14ad00 | load | 1 | `(unknown)` | `-` |
| `out` at 0x14ad05 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ad0a | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ad0d | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ad10 | store | 1 | `I/O (unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x14ac81 | `(unknown callee) 0x14ae05` | `edi` | `esi` | `edx` | `ecx` | `ebx` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x14ac7f | `[[ebp+16]] == 0x0` | `[[ebp+16]] != 0x0` |
| `jbe` at 0x14acab | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x14acb1 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14acbc | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jae` at 0x14ace2 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x14ad15 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14acf2 | `eax` | `?` |

## `_mwVGAHRPal`

- `_mwVGAHRPal`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14ad20 to 0x14ae05.
  Established. The unnamed call to `0x14ae4e` (`call` at 0x14ad35) follows the branch fall-through
  condition `[[ebp+16]] != 0x0` (`je` at 0x14ad33), then jumps to restoration (`jmp` at 0x14ad3a).
  Taking that branch reaches the shared two-byte load at `[ebp+8] +0x16` (`mov` at 0x14ad4f). The
  subsequent shared byte inputs have unknown I/O addresses (`in` at 0x14ad7a, `in` at 0x14ad7d, `in`
  at 0x14ad80).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14ad2d | load | 4 | `ebp +0x10` | `-` |
| `cmp` at 0x14ad30 | load | 4 | `[ebp+16] +0x0` | `-` |
| `mov` at 0x14ad4c | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x14ad4f | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x14ad53 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14ad56 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14ad59 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14ad6c | load | 4 | `[ebp+16] +0xc` | `-` |
| `out` at 0x14ad77 | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x14ad7a | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14ad7d | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14ad80 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x14ad92 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14ada4 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14adb7 | store | 2 | `(unknown)` | `?` |
| `out` at 0x14add7 | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x14adda | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14addd | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14ade0 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x14ade7 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14adee | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14adf6 | store | 2 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x14ad35 | `(unknown callee) 0x14ae4e` | `edi` | `esi` | `edx` | `ecx` | `ebx` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x14ad33 | `[[ebp+16]] == 0x0` | `[[ebp+16]] != 0x0` |
| `jbe` at 0x14ad62 | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x14ad68 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14ad73 | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jne` at 0x14ad8d | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x14ad9f | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x14adb2 | `? != 0xfc` | `? == 0xfc` |
| `jae` at 0x14adbd | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x14adfc | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14adcd | `eax` | `?` |

## `_mwVGAOscan`

- `_mwVGAOscan`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14aec0 to 0x14aed8.
  Established. The first byte output writes `0x31` at I/O absolute `0x3c0` (`out` at 0x14aecc), and
  the next writes an unknown byte there (`out` at 0x14aed0). Between them, a four-byte load reads
  `ebp +0x10` (`mov` at 0x14aecd). An earlier byte input reads I/O absolute `0x3da` (`in` at
  0x14aec5). No output register meaning follows from the name.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `in` at 0x14aec5 | load | 1 | `I/O abs 0x3da` | `-` |
| `out` at 0x14aecc | store | 1 | `I/O abs 0x3c0` | `0x31` |
| `mov` at 0x14aecd | load | 4 | `ebp +0x10` | `-` |
| `out` at 0x14aed0 | store | 1 | `I/O abs 0x3c0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14aed4 | `eax` | `?` |

## `_mw8514WPal`

- `_mw8514WPal`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14ac88 to 0x14ad20.
  Established. Falling through the upper-range guard means `[[ebp+16]+8] > 0xff (unsigned, 32-bit)`
  (`jbe` at 0x14acab) and reaches a jump (`jmp` at 0x14acad) to the assignment
  `eax = 0xffffffff` (`mov` at 0x14acf3). Arm selection
  lies after following both range guards (`jbe` at 0x14acab, `jae` at 0x14acb1); its branch is taken
  when `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` (`jne` at 0x14acbc). Both arms include byte
  outputs with unknown I/O addresses and values (`out` at 0x14acd2, `out` at 0x14acd7, `out` at
  0x14acda, `out` at 0x14acdd, `out` at 0x14ad05, `out` at 0x14ad0a, `out` at 0x14ad0d, `out` at
  0x14ad10).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14ac95 | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x14ac98 | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x14ac9c | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14ac9f | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14aca2 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14acb5 | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14acbe | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14acc1 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14acc4 | load | 1 | `(unknown)` | `-` |
| `out` at 0x14acd2 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14acd7 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14acda | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14acdd | store | 1 | `I/O (unknown)` | `?` |
| `mov` at 0x14acfa | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14acfd | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14ad00 | load | 1 | `(unknown)` | `-` |
| `out` at 0x14ad05 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ad0a | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ad0d | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ad10 | store | 1 | `I/O (unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x14acab | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x14acb1 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14acbc | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jae` at 0x14ace2 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x14ad15 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14acf2 | `eax` | `?` |

## `_mw8514RPal`

- `_mw8514RPal`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14ad3f to 0x14aec0.
  Established. The arm-selection fall-through is `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` (`jne`
  at 0x14ad73). That arm includes a byte output (`out` at 0x14ad77), byte inputs (`in` at 0x14ad7a,
  `in` at 0x14ad7d, `in` at 0x14ad80), and two-byte stores (`mov` at 0x14ad92, `mov` at 0x14ada4,
  `mov` at 0x14adb7). All those effective targets remain unknown, as do the output and stored
  values. The other arm stays separate in the access table.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14ad4c | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x14ad4f | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x14ad53 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14ad56 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14ad59 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14ad6c | load | 4 | `[ebp+16] +0xc` | `-` |
| `out` at 0x14ad77 | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x14ad7a | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14ad7d | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14ad80 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x14ad92 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14ada4 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14adb7 | store | 2 | `(unknown)` | `?` |
| `out` at 0x14add7 | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x14adda | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14addd | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14ade0 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x14ade7 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14adee | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14adf6 | store | 2 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x14ad62 | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x14ad68 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14ad73 | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jne` at 0x14ad8d | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x14ad9f | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x14adb2 | `? != 0xfc` | `? == 0xfc` |
| `jae` at 0x14adbd | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x14adfc | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14adcd | `eax` | `?` |

## `_vga_setpalette`

- `_vga_setpalette`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16f0d8 to 0x16f148.
  Established. Candidate zero at the first `_sysOutByte` site is `0x3c8` (`call` at 0x16f0e5). The
  three later sites each present `0x3c9` in that slot (`call` at 0x16f101, `call` at 0x16f11d,
  `call` at 0x16f139). Candidate one remains unknown at all four. Each repeat branch is taken when
  `N == 0` (`jns` at 0x16f0f5, `jns` at 0x16f111, `jns` at 0x16f12d), with no established iteration
  count.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movsx` at 0x16f0db | load | 1 | `sp +0x4` | `-` |
| `movsx` at 0x16f0f7 | load | 1 | `sp +0x8` | `-` |
| `movsx` at 0x16f113 | load | 1 | `sp +0xc` | `-` |
| `movsx` at 0x16f12f | load | 1 | `sp +0x10` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16f0e5 | `_sysOutByte 0x1081d0` | `0x3c8` | `?` | `ebp` | `?` | `?` | `?` |
| `call` at 0x16f101 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16f11d | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16f139 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jns` at 0x16f0f5 | `N == 0` | `not (N == 0)` |
| `jns` at 0x16f111 | `N == 0` | `not (N == 0)` |
| `jns` at 0x16f12d | `N == 0` | `not (N == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16f143 | `eax` | `0x0` |

## `_vga_getpalette`

- `_vga_getpalette`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16f148 to 0x16f1c8.
  Established. The `_sysOutByte` call has candidate zero `0x3c7` and unknown candidate one (`call`
  at 0x16f161). Three `_sysInByte` calls present candidate zero `0x3c9` (`call` at 0x16f178, `call`
  at 0x16f194, `call` at 0x16f1b0). Four-byte stores target `[sp+8] +0x0` (`mov` at 0x16f185),
  `[sp+12] +0x0` (`mov` at 0x16f1a1), and `[sp+16] +0x0` (`mov` at 0x16f1ba). Their values remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16f14e | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x16f151 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x16f154 | load | 4 | `sp +0x10` | `-` |
| `movsx` at 0x16f157 | load | 1 | `sp +0x4` | `-` |
| `mov` at 0x16f185 | store | 4 | `[sp+8] +0x0` | `?` |
| `mov` at 0x16f1a1 | store | 4 | `[sp+12] +0x0` | `?` |
| `mov` at 0x16f1ba | store | 4 | `[sp+16] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16f161 | `_sysOutByte 0x1081d0` | `0x3c7` | `?` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x16f178 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16f194 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16f1b0 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jns` at 0x16f171 | `N == 0` | `not (N == 0)` |
| `jns` at 0x16f18d | `N == 0` | `not (N == 0)` |
| `jns` at 0x16f1a9 | `N == 0` | `not (N == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16f1c7 | `eax` | `0x0` |

## `_vga_setpalvec`

- `_vga_setpalvec`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16f1c8 to 0x16f258.
  Established. The body is bypassed by the taken guard condition `0x0 >= [sp+8] (signed, 32-bit)`
  (`jge` at 0x16f1ea). Inside it, three byte loads retain unknown effective addresses (`movsx` at
  0x16f1f7, `movsx` at 0x16f213, `movsx` at 0x16f22f). The corresponding `_sysOutByte` sites present
  candidate zero `0x3c9` and unknown candidate one (`call` at 0x16f203, `call` at 0x16f21f, `call`
  at 0x16f23b). The repeat comparison keeps its left operand unknown (`jl` at 0x16f246).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16f1ce | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x16f1d1 | load | 4 | `sp +0xc` | `-` |
| `movsx` at 0x16f1d4 | load | 1 | `sp +0x4` | `-` |
| `movsx` at 0x16f1f7 | load | 1 | `(unknown)` | `-` |
| `movsx` at 0x16f213 | load | 1 | `(unknown)` | `-` |
| `movsx` at 0x16f22f | load | 1 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16f1de | `_sysOutByte 0x1081d0` | `0x3c8` | `?` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x16f203 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16f21f | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16f23b | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x16f1ea | `0x0 >= [sp+8] (signed, 32-bit)` | `0x0 < [sp+8] (signed, 32-bit)` |
| `jns` at 0x16f1f5 | `N == 0` | `not (N == 0)` |
| `jns` at 0x16f211 | `N == 0` | `not (N == 0)` |
| `jns` at 0x16f22d | `N == 0` | `not (N == 0)` |
| `jl` at 0x16f246 | `? < [sp+8] (signed, 32-bit)` | `? >= [sp+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16f253 | `eax` | `?` |

## `_vga_getpalvec`

- `_vga_getpalvec`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16f258 to 0x16f2f8.
  Established. The signed guard falls through when `0x0 < [sp+8] (signed, 32-bit)` (`jge` at
  0x16f27a). That body includes three `_sysInByte` calls with candidate zero `0x3c9` (`call` at
  0x16f28c, `call` at 0x16f2ac, `call` at 0x16f2cc). Three four-byte stores retain unknown effective
  addresses and values (`mov` at 0x16f299, `mov` at 0x16f2b9, `mov` at 0x16f2d9). The exit value
  stays unknown (`ret` at 0x16f2ee).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16f25e | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x16f261 | load | 4 | `sp +0xc` | `-` |
| `movsx` at 0x16f264 | load | 1 | `sp +0x4` | `-` |
| `mov` at 0x16f299 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x16f2b9 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x16f2d9 | store | 4 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16f26e | `_sysOutByte 0x1081d0` | `0x3c7` | `?` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x16f28c | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16f2ac | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16f2cc | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x16f27a | `0x0 >= [sp+8] (signed, 32-bit)` | `0x0 < [sp+8] (signed, 32-bit)` |
| `jns` at 0x16f285 | `N == 0` | `not (N == 0)` |
| `jns` at 0x16f2a5 | `N == 0` | `not (N == 0)` |
| `jns` at 0x16f2c5 | `N == 0` | `not (N == 0)` |
| `jl` at 0x16f2e1 | `? < [sp+8] (signed, 32-bit)` | `? >= [sp+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16f2ee | `eax` | `?` |

## `_ReadPalette`

- `_ReadPalette`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x158124 to 0x158178.
  Established. The four-byte call operand loads `[[0x1cf40c]] +0xcc` (`call` at 0x158158); its
  leading candidate slots contain `[[0x1cf40c]]`, `0x4`, and `sp-32` (`call` at 0x158158). The
  following branch takes its edge when `Z == 1` (`je` at 0x158163), bypassing the unnamed call to
  `0x162318` (`call` at 0x15816a). Neither callee operation is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x15812d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x158130 | store | 4 | `sp -0x20` | `[sp+4]` |
| `mov` at 0x158133 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x158136 | store | 4 | `sp -0x1c` | `[sp+8]` |
| `mov` at 0x158139 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x15813c | store | 4 | `sp -0x18` | `[sp+12]` |
| `mov` at 0x15813f | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x158142 | store | 4 | `sp -0x14` | `[sp+16]` |
| `mov` at 0x158145 | load | 4 | `abs 0x1cf40c` | `-` |
| `mov` at 0x15814b | load | 4 | `[0x1cf40c] +0x0` | `-` |
| `call` at 0x158158 | load | 4 | `[[0x1cf40c]] +0xcc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x158158 | `(unknown callee) (unknown target)` | `[[0x1cf40c]]` | `0x4` | `sp-32` | `[sp+4]` | `[sp+8]` | `[sp+12]` |
| `call` at 0x15816a | `(unknown callee) 0x162318` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x158163 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x158176 | `eax` | `?` |

## `_WritePalette`

- `_WritePalette`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1580d0 to 0x158124.
  Established. Four-byte local stores copy `[sp+4]` to `sp -0x20` (`mov` at 0x1580dc), `[sp+8]` to
  `sp -0x1c` (`mov` at 0x1580e2), `[sp+12]` to `sp -0x18` (`mov` at 0x1580e8), and `[sp+16]` to `sp
  -0x14` (`mov` at 0x1580ee). The indirect call presents leading candidates `[[0x1cf40c]]`, `0x3`,
  and `sp-32` (`call` at 0x158104). The operation reached through that target remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1580d9 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1580dc | store | 4 | `sp -0x20` | `[sp+4]` |
| `mov` at 0x1580df | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1580e2 | store | 4 | `sp -0x1c` | `[sp+8]` |
| `mov` at 0x1580e5 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1580e8 | store | 4 | `sp -0x18` | `[sp+12]` |
| `mov` at 0x1580eb | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x1580ee | store | 4 | `sp -0x14` | `[sp+16]` |
| `mov` at 0x1580f1 | load | 4 | `abs 0x1cf40c` | `-` |
| `mov` at 0x1580f7 | load | 4 | `[0x1cf40c] +0x0` | `-` |
| `call` at 0x158104 | load | 4 | `[[0x1cf40c]] +0xcc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x158104 | `(unknown callee) (unknown target)` | `[[0x1cf40c]]` | `0x3` | `sp-32` | `[sp+4]` | `[sp+8]` | `[sp+12]` |
| `call` at 0x158116 | `(unknown callee) 0x162318` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x15810f | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x158122 | `eax` | `?` |
