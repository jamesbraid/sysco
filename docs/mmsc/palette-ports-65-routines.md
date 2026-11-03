# MMSC palette and port paths in IRIX 6.5

Read from `6.5/usr/cpu/firmware/mmscfw.bin` (IRIX 6.5).
Image SHA-256: `2017f743fb7360a160f4252b69569b7f9713d9efad82719cfb2fd82538e37ff8`.
This image has the x86 a.out link address `0x108000`. The [load map](load-map.md) explains segment mapping.
Most analysed spans end at the next text symbol, with exclusive inferred ends.
The `_mwVGAHWPal` and `_mwVGAHRPal` spans extend through shared continuations.
Those two given extents overlap the following entry rather than ending at its symbol.
They can include literals or padding after returns. Statements below concern sites
reached by known control-flow edges.

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

[Fact sheets and full listings](disasm/palette-ports-65-facts.txt) include all tool sections.

## `_mwCGAPal`

- `_mwCGAPal`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14d160 to 0x14d1c8.
  Established. Three two-byte loads use `[[ebp+16]+12] +0x0` (`mov` at 0x14d171), `[[ebp+16]+12]
  +0x2` (`mov` at 0x14d184), and `[[ebp+16]+12] +0x4` (`mov` at 0x14d198). The first branch is taken
  when `Z == 0` (`jne` at 0x14d16a). Both software interrupts have immediate `0x10` (`int` at
  0x14d1ae, `int` at 0x14d1c1), without an established service meaning.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14d162 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14d165 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14d16c | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14d171 | load | 2 | `[[ebp+16]+12] +0x0` | `-` |
| `mov` at 0x14d184 | load | 2 | `[[ebp+16]+12] +0x2` | `-` |
| `mov` at 0x14d198 | load | 2 | `[[ebp+16]+12] +0x4` | `-` |
| `mov` at 0x14d1b0 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14d1b3 | load | 4 | `[ebp+16] +0x0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x14d16a | `Z == 0` | `not (Z == 0)` |
| `je` at 0x14d177 | `(? & 0x80) == 0x0` | `(? & 0x80) != 0x0` |
| `je` at 0x14d17f | `Z == 1` | `not (Z == 1)` |
| `je` at 0x14d18b | `(? & 0x80) == 0x0` | `(? & 0x80) != 0x0` |
| `je` at 0x14d193 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x14d19f | `(? & 0x80) == 0x0` | `(? & 0x80) != 0x0` |
| `je` at 0x14d1a7 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x14d1b9 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14d1c7 | `eax` | `?` |

## `_mwEGAWPal`

- `_mwEGAWPal`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14d1c8 to 0x14d27c.
  Established. The upper-range guard falls through when `[[ebp+16]+8] > 0xf (unsigned, 32-bit)`
  (`jbe` at 0x14d1d9), reaching the jump (`jmp` at 0x14d1db) to the `eax` assignment of `0xffffffff`
  (`mov` at 0x14d274). Taking both range guards (`jbe` at 0x14d1d9, `jae` at 0x14d1e2) instead
  reaches the body; its byte I/O includes input at absolute `0x3da` (`in` at 0x14d1f4), unknown
  output values (`out` at 0x14d247, `out` at 0x14d24a), and output value `0x20` (`out` at 0x14d257),
  with all output addresses unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14d1cd | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14d1d0 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14d1d3 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14d1e9 | load | 1 | `[ebp+16] +0x0` | `-` |
| `in` at 0x14d1f4 | load | 1 | `I/O abs 0x3da` | `-` |
| `mov` at 0x14d1f9 | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14d205 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14d218 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14d22b | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14d23e | store | 1 | `(unknown)` | `?` |
| `out` at 0x14d247 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14d24a | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14d257 | store | 1 | `I/O (unknown)` | `0x20` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x14d1d9 | `[[ebp+16]+8] <= 0xf (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xf (unsigned, 32-bit)` |
| `jae` at 0x14d1e2 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14d1ee | `([[ebp+16]]:1 & 0xff) != 0x0` | `([[ebp+16]]:1 & 0xff) == 0x0` |
| `jne` at 0x14d243 | `([[ebp+16]]:1 & 0xff) != 0x0` | `([[ebp+16]]:1 & 0xff) == 0x0` |
| `jle` at 0x14d253 | `? <= ? (signed, 8-bit)` | `? > ? (signed, 8-bit)` |
| `jle` at 0x14d26e | `? <= ? (signed, 8-bit)` | `? > ? (signed, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14d25f | `eax` | `?` |

## `_mwEGARPal`

- `_mwEGARPal`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14d27c to 0x14d30c.
  Established. The four-byte load uses `[ebp+16] +0xc` (`mov` at 0x14d29d). The loop stores two
  bytes at each of three unknown effective addresses, retaining unknown values (`mov` at 0x14d2b9,
  `mov` at 0x14d2d3, `mov` at 0x14d2ee); its repeat branch is taken when `? <= ? (signed, 8-bit)`
  (`jle` at 0x14d2fa). No destination extent or data encoding is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14d280 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14d283 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14d286 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14d29d | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14d2a0 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14d2b9 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14d2d3 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14d2ee | store | 2 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x14d28c | `[[ebp+16]+8] <= 0xf (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xf (unsigned, 32-bit)` |
| `jae` at 0x14d292 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14d2aa | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14d2b3 | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14d2c4 | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14d2cd | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14d2df | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14d2e8 | `Z == 0` | `not (Z == 0)` |
| `jle` at 0x14d2fa | `? <= ? (signed, 8-bit)` | `? > ? (signed, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14d302 | `eax` | `?` |

## `_mwEGAOscan`

- `_mwEGAOscan`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14d30c to 0x14d334.
  Established. The byte input at I/O absolute `0x3da` (`in` at 0x14d312) is followed by byte outputs
  to I/O absolute `0x3c0`, carrying `0x11` (`out` at 0x14d319), unknown (`out` at 0x14d328), and
  `0x20` (`out` at 0x14d32b). The intervening byte load remains at an unknown effective address
  (`mov` at 0x14d326). The port roles stay unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `in` at 0x14d312 | load | 1 | `I/O abs 0x3da` | `-` |
| `out` at 0x14d319 | store | 1 | `I/O abs 0x3c0` | `0x11` |
| `mov` at 0x14d31a | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14d326 | load | 1 | `(unknown)` | `-` |
| `out` at 0x14d328 | store | 1 | `I/O abs 0x3c0` | `?` |
| `out` at 0x14d32b | store | 1 | `I/O abs 0x3c0` | `0x20` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14d330 | `eax` | `?` |

## `_mwVGAHWPal`

- `_mwVGAHWPal`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14d334 to 0x14d3e8.
  Established. Taking the first branch when `[[ebp+16]] == 0x0` (`je` at 0x14d347) reaches the
  shared four-byte load at `ebp +0x8` (`mov` at 0x14d35d). Falling through calls unnamed target
  `0x14d4cd` (`call` at 0x14d349) and then jumps to restoration (`jmp` at 0x14d34e). The shared byte
  outputs retain unknown I/O targets and values (`out` at 0x14d39a, `out` at 0x14d39f, `out` at
  0x14d3a2, `out` at 0x14d3a5).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14d341 | load | 4 | `ebp +0x10` | `-` |
| `cmp` at 0x14d344 | load | 4 | `[ebp+16] +0x0` | `-` |
| `mov` at 0x14d35d | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x14d360 | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x14d364 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14d367 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14d36a | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14d37d | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14d386 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14d389 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14d38c | load | 1 | `(unknown)` | `-` |
| `out` at 0x14d39a | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14d39f | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14d3a2 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14d3a5 | store | 1 | `I/O (unknown)` | `?` |
| `mov` at 0x14d3c2 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14d3c5 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14d3c8 | load | 1 | `(unknown)` | `-` |
| `out` at 0x14d3cd | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14d3d2 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14d3d5 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14d3d8 | store | 1 | `I/O (unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x14d349 | `(unknown callee) 0x14d4cd` | `edi` | `esi` | `edx` | `ecx` | `ebx` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x14d347 | `[[ebp+16]] == 0x0` | `[[ebp+16]] != 0x0` |
| `jbe` at 0x14d373 | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x14d379 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14d384 | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jae` at 0x14d3aa | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x14d3dd | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14d3ba | `eax` | `?` |

## `_mwVGAHRPal`

- `_mwVGAHRPal`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14d3e8 to 0x14d4cd.
  Established. The first branch falls through when `[[ebp+16]] != 0x0` (`je` at 0x14d3fb), reaching
  an unnamed direct call to `0x14d516` (`call` at 0x14d3fd) and restoration jump (`jmp` at
  0x14d402). Taking that branch reaches the shared two-byte load at `[ebp+8] +0x16` (`mov` at
  0x14d417). Shared byte inputs keep unknown I/O addresses (`in` at 0x14d442, `in` at 0x14d445, `in`
  at 0x14d448).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14d3f5 | load | 4 | `ebp +0x10` | `-` |
| `cmp` at 0x14d3f8 | load | 4 | `[ebp+16] +0x0` | `-` |
| `mov` at 0x14d414 | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x14d417 | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x14d41b | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14d41e | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14d421 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14d434 | load | 4 | `[ebp+16] +0xc` | `-` |
| `out` at 0x14d43f | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x14d442 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14d445 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14d448 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x14d45a | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14d46c | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14d47f | store | 2 | `(unknown)` | `?` |
| `out` at 0x14d49f | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x14d4a2 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14d4a5 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14d4a8 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x14d4af | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14d4b6 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14d4be | store | 2 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x14d3fd | `(unknown callee) 0x14d516` | `edi` | `esi` | `edx` | `ecx` | `ebx` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x14d3fb | `[[ebp+16]] == 0x0` | `[[ebp+16]] != 0x0` |
| `jbe` at 0x14d42a | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x14d430 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14d43b | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jne` at 0x14d455 | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x14d467 | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x14d47a | `? != 0xfc` | `? == 0xfc` |
| `jae` at 0x14d485 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x14d4c4 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14d495 | `eax` | `?` |

## `_mwVGAOscan`

- `_mwVGAOscan`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14d588 to 0x14d5a0.
  Established. Two byte outputs at I/O absolute `0x3c0` carry `0x31` (`out` at 0x14d594) and unknown
  (`out` at 0x14d598). A four-byte load from `ebp +0x10` separates them (`mov` at 0x14d595). The
  prior byte input uses I/O absolute `0x3da` (`in` at 0x14d58d). No hardware field
  interpretation follows from those values.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `in` at 0x14d58d | load | 1 | `I/O abs 0x3da` | `-` |
| `out` at 0x14d594 | store | 1 | `I/O abs 0x3c0` | `0x31` |
| `mov` at 0x14d595 | load | 4 | `ebp +0x10` | `-` |
| `out` at 0x14d598 | store | 1 | `I/O abs 0x3c0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14d59c | `eax` | `?` |

## `_mw8514WPal`

- `_mw8514WPal`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14d350 to 0x14d3e8.
  Established. The failed upper-range guard falls through when `[[ebp+16]+8] > 0xff (unsigned,
  32-bit)` (`jbe` at 0x14d373), then jumps (`jmp` at 0x14d375) to the assignment of `0xffffffff` to
  `eax` (`mov` at 0x14d3bb). Arm selection follows following both range guards (`jbe` at 0x14d373,
  `jae` at 0x14d379), with taken predicate `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` (`jne` at
  0x14d384). Eight byte outputs retain unknown I/O addresses and values (`out` at 0x14d39a, `out` at
  0x14d39f, `out` at 0x14d3a2, `out` at 0x14d3a5, `out` at 0x14d3cd, `out` at 0x14d3d2, `out` at
  0x14d3d5, `out` at 0x14d3d8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14d35d | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x14d360 | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x14d364 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14d367 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14d36a | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14d37d | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14d386 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14d389 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14d38c | load | 1 | `(unknown)` | `-` |
| `out` at 0x14d39a | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14d39f | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14d3a2 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14d3a5 | store | 1 | `I/O (unknown)` | `?` |
| `mov` at 0x14d3c2 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14d3c5 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14d3c8 | load | 1 | `(unknown)` | `-` |
| `out` at 0x14d3cd | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14d3d2 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14d3d5 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14d3d8 | store | 1 | `I/O (unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x14d373 | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x14d379 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14d384 | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jae` at 0x14d3aa | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x14d3dd | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14d3ba | `eax` | `?` |

## `_mw8514RPal`

- `_mw8514RPal`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14d407 to 0x14d588.
  Established. The arm-selection branch falls through when `(([[ebp+8]+22]:2 & 0xffff) & 0x100) ==
  0x0` (`jne` at 0x14d43b). That arm includes a byte output (`out` at 0x14d43f), three byte inputs
  (`in` at 0x14d442, `in` at 0x14d445, `in` at 0x14d448), and two-byte stores (`mov` at 0x14d45a,
  `mov` at 0x14d46c, `mov` at 0x14d47f). Effective addresses remain unknown, as do output and stored
  values. The other arm stays separate in the access table.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14d414 | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x14d417 | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x14d41b | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14d41e | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14d421 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14d434 | load | 4 | `[ebp+16] +0xc` | `-` |
| `out` at 0x14d43f | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x14d442 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14d445 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14d448 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x14d45a | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14d46c | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14d47f | store | 2 | `(unknown)` | `?` |
| `out` at 0x14d49f | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x14d4a2 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14d4a5 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14d4a8 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x14d4af | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14d4b6 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14d4be | store | 2 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x14d42a | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x14d430 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14d43b | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jne` at 0x14d455 | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x14d467 | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x14d47a | `? != 0xfc` | `? == 0xfc` |
| `jae` at 0x14d485 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x14d4c4 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14d495 | `eax` | `?` |

## `_vga_setpalette`

- `_vga_setpalette`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1717a0 to 0x171810.
  Established. The `_sysOutByte` sites include candidate zero `0x3c8` at the first call (`call` at
  0x1717ad) and `0x3c9` at the other three (`call` at 0x1717c9, `call` at 0x1717e5, `call` at
  0x171801). Candidate one remains unknown at each. The repeat branches all take their edges when `N
  == 0` (`jns` at 0x1717bd, `jns` at 0x1717d9, `jns` at 0x1717f5). The exit carries `0x0` in `eax`
  (`ret` at 0x17180b).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movsx` at 0x1717a3 | load | 1 | `sp +0x4` | `-` |
| `movsx` at 0x1717bf | load | 1 | `sp +0x8` | `-` |
| `movsx` at 0x1717db | load | 1 | `sp +0xc` | `-` |
| `movsx` at 0x1717f7 | load | 1 | `sp +0x10` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1717ad | `_sysOutByte 0x1081d0` | `0x3c8` | `?` | `ebp` | `?` | `?` | `?` |
| `call` at 0x1717c9 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1717e5 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x171801 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jns` at 0x1717bd | `N == 0` | `not (N == 0)` |
| `jns` at 0x1717d9 | `N == 0` | `not (N == 0)` |
| `jns` at 0x1717f5 | `N == 0` | `not (N == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x17180b | `eax` | `0x0` |

## `_vga_getpalette`

- `_vga_getpalette`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x171810 to 0x171890.
  Established. The initial `_sysOutByte` call presents candidate zero `0x3c7` with unknown candidate
  one (`call` at 0x171829). Three `_sysInByte` calls present candidate zero `0x3c9` (`call` at
  0x171840, `call` at 0x17185c, `call` at 0x171878). Four-byte stores through `[sp+8] +0x0` (`mov`
  at 0x17184d), `[sp+12] +0x0` (`mov` at 0x171869), and `[sp+16] +0x0` (`mov` at 0x171882) retain
  unknown values.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x171816 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x171819 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x17181c | load | 4 | `sp +0x10` | `-` |
| `movsx` at 0x17181f | load | 1 | `sp +0x4` | `-` |
| `mov` at 0x17184d | store | 4 | `[sp+8] +0x0` | `?` |
| `mov` at 0x171869 | store | 4 | `[sp+12] +0x0` | `?` |
| `mov` at 0x171882 | store | 4 | `[sp+16] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x171829 | `_sysOutByte 0x1081d0` | `0x3c7` | `?` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x171840 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x17185c | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x171878 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jns` at 0x171839 | `N == 0` | `not (N == 0)` |
| `jns` at 0x171855 | `N == 0` | `not (N == 0)` |
| `jns` at 0x171871 | `N == 0` | `not (N == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x17188f | `eax` | `0x0` |

## `_vga_setpalvec`

- `_vga_setpalvec`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x171890 to 0x171920.
  Established. The initial guard takes its bypass edge when `0x0 >= [sp+8] (signed, 32-bit)` (`jge`
  at 0x1718b2). The body includes byte loads at three unknown effective addresses (`movsx` at
  0x1718bf, `movsx` at 0x1718db, `movsx` at 0x1718f7); its later `_sysOutByte` sites have candidate
  zero `0x3c9` and unknown candidate one (`call` at 0x1718cb, `call` at 0x1718e7, `call` at
  0x171903). The repeat comparison retains an unknown left operand (`jl` at 0x17190e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x171896 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x171899 | load | 4 | `sp +0xc` | `-` |
| `movsx` at 0x17189c | load | 1 | `sp +0x4` | `-` |
| `movsx` at 0x1718bf | load | 1 | `(unknown)` | `-` |
| `movsx` at 0x1718db | load | 1 | `(unknown)` | `-` |
| `movsx` at 0x1718f7 | load | 1 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1718a6 | `_sysOutByte 0x1081d0` | `0x3c8` | `?` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x1718cb | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1718e7 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x171903 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x1718b2 | `0x0 >= [sp+8] (signed, 32-bit)` | `0x0 < [sp+8] (signed, 32-bit)` |
| `jns` at 0x1718bd | `N == 0` | `not (N == 0)` |
| `jns` at 0x1718d9 | `N == 0` | `not (N == 0)` |
| `jns` at 0x1718f5 | `N == 0` | `not (N == 0)` |
| `jl` at 0x17190e | `? < [sp+8] (signed, 32-bit)` | `? >= [sp+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x17191b | `eax` | `?` |

## `_vga_getpalvec`

- `_vga_getpalvec`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x171920 to 0x1719c0.
  Established. The signed guard enters the body on fall-through when `0x0 < [sp+8] (signed, 32-bit)`
  (`jge` at 0x171942). Three `_sysInByte` sites have candidate zero `0x3c9` (`call` at 0x171954,
  `call` at 0x171974, `call` at 0x171994). Three four-byte stores retain unknown addresses and
  values (`mov` at 0x171961, `mov` at 0x171981, `mov` at 0x1719a1). The final return expression
  remains unknown (`ret` at 0x1719b6).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x171926 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x171929 | load | 4 | `sp +0xc` | `-` |
| `movsx` at 0x17192c | load | 1 | `sp +0x4` | `-` |
| `mov` at 0x171961 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x171981 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x1719a1 | store | 4 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x171936 | `_sysOutByte 0x1081d0` | `0x3c7` | `?` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x171954 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x171974 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x171994 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x171942 | `0x0 >= [sp+8] (signed, 32-bit)` | `0x0 < [sp+8] (signed, 32-bit)` |
| `jns` at 0x17194d | `N == 0` | `not (N == 0)` |
| `jns` at 0x17196d | `N == 0` | `not (N == 0)` |
| `jns` at 0x17198d | `N == 0` | `not (N == 0)` |
| `jl` at 0x1719a9 | `? < [sp+8] (signed, 32-bit)` | `? >= [sp+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1719b6 | `eax` | `?` |

## `_ReadPalette`

- `_ReadPalette`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x15a7ec to 0x15a840.
  Established. Two four-byte loads use absolute `0x1d1b44` (`mov` at 0x15a80d) and `[0x1d1b44] +0x0`
  (`mov` at 0x15a813). The indirect call operand reads four bytes from `[[0x1d1b44]] +0xcc` and
  presents leading candidates `[[0x1d1b44]]`, `0x4`, and `sp-32` (`call` at 0x15a820). The later
  unnamed direct call targets `0x1649e0` with unknown candidates (`call` at 0x15a832). No callee
  effect is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x15a7f5 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x15a7f8 | store | 4 | `sp -0x20` | `[sp+4]` |
| `mov` at 0x15a7fb | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x15a7fe | store | 4 | `sp -0x1c` | `[sp+8]` |
| `mov` at 0x15a801 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x15a804 | store | 4 | `sp -0x18` | `[sp+12]` |
| `mov` at 0x15a807 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x15a80a | store | 4 | `sp -0x14` | `[sp+16]` |
| `mov` at 0x15a80d | load | 4 | `abs 0x1d1b44` | `-` |
| `mov` at 0x15a813 | load | 4 | `[0x1d1b44] +0x0` | `-` |
| `call` at 0x15a820 | load | 4 | `[[0x1d1b44]] +0xcc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x15a820 | `(unknown callee) (unknown target)` | `[[0x1d1b44]]` | `0x4` | `sp-32` | `[sp+4]` | `[sp+8]` | `[sp+12]` |
| `call` at 0x15a832 | `(unknown callee) 0x1649e0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x15a82b | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x15a83e | `eax` | `?` |

## `_WritePalette`

- `_WritePalette`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x15a798 to 0x15a7ec.
  Established. Four-byte stores copy `[sp+4]` to `sp -0x20` (`mov` at 0x15a7a4), `[sp+8]` to `sp
  -0x1c` (`mov` at 0x15a7aa), `[sp+12]` to `sp -0x18` (`mov` at 0x15a7b0), and `[sp+16]` to `sp
  -0x14` (`mov` at 0x15a7b6). The indirect call presents leading candidates `[[0x1d1b44]]`, `0x3`,
  and `sp-32` (`call` at 0x15a7cc). Taking the later branch when `Z == 1` (`je` at 0x15a7d7)
  bypasses the unnamed direct call (`call` at 0x15a7de).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x15a7a1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x15a7a4 | store | 4 | `sp -0x20` | `[sp+4]` |
| `mov` at 0x15a7a7 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x15a7aa | store | 4 | `sp -0x1c` | `[sp+8]` |
| `mov` at 0x15a7ad | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x15a7b0 | store | 4 | `sp -0x18` | `[sp+12]` |
| `mov` at 0x15a7b3 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x15a7b6 | store | 4 | `sp -0x14` | `[sp+16]` |
| `mov` at 0x15a7b9 | load | 4 | `abs 0x1d1b44` | `-` |
| `mov` at 0x15a7bf | load | 4 | `[0x1d1b44] +0x0` | `-` |
| `call` at 0x15a7cc | load | 4 | `[[0x1d1b44]] +0xcc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x15a7cc | `(unknown callee) (unknown target)` | `[[0x1d1b44]]` | `0x3` | `sp-32` | `[sp+4]` | `[sp+8]` | `[sp+12]` |
| `call` at 0x15a7de | `(unknown callee) 0x1649e0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x15a7d7 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x15a7ea | `eax` | `?` |
