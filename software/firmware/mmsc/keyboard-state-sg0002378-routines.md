# MMSC keyboard state and port-call paths in patch SG0002378

Read from `patches/SG0002378/usr/cpu/firmware/mmscfw.bin` (patch SG0002378).
Image SHA-256: `b1249e58cf4f00c252362da8ed0515e25bb94aaf096bb60a5c5082c9b6e4d342`.
This image has the x86 a.out link address `0x108000`. The [load map](load-map.md) explains segment mapping.
The analysed spans end at the next text symbol, with exclusive inferred ends.
They can include literals or padding after returns. Statements below concern sites
reached by known control-flow edges.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack locations.
The full facts also include implicit push/pop accesses. Call and return instructions
remain in the listings without separate implicit stack access rows.
Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/keyboard-state-sg0002378-facts.txt) include all tool sections.

## `_kbdNormal`

- `_kbdNormal`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10afa8 to 0x10b028.
  Established. The initial branch bypasses the call when `(([0x1d4ad1]:1 & 0xff) & 0x2) != 0x0`
  (`jne` at 0x10afbc). The indexed byte load has an unknown effective address (`mov` at 0x10afe3).
  The `_tyIRd` site has unknown leading candidates (`call` at 0x10b01b). The role of the
  indexed load and call remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10afac | load | 1 | `sp +0x4` | `-` |
| `mov` at 0x10afaf | load | 4 | `abs 0x1d4ad4` | `-` |
| `test` at 0x10afb5 | load | 1 | `abs 0x1d4ad1` | `-` |
| `mov` at 0x10afbe | load | 4 | `abs 0x1d4acc` | `-` |
| `movzx` at 0x10afc9 | load | 2 | `abs 0x1d4ad2` | `-` |
| `mov` at 0x10afe3 | load | 1 | `(unknown)` | `-` |
| `test` at 0x10afef | load | 1 | `abs 0x1d4ad0` | `-` |
| `mov` at 0x10b020 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b01b | `_tyIRd 0x16fe88` | `?` | `?` | `ebx` | `ebp` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10afbc | `(([0x1d4ad1]:1 & 0xff) & 0x2) != 0x0` | `(([0x1d4ad1]:1 & 0xff) & 0x2) == 0x0` |
| `je` at 0x10afe9 | `? == 0xff` | `? != 0xff` |
| `je` at 0x10afed | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x10aff6 | `(([0x1d4ad0]:1 & 0xff) & 0x4) == 0x0` | `(([0x1d4ad0]:1 & 0xff) & 0x4) != 0x0` |
| `ja` at 0x10affe | `? > 0x19 (unsigned, 8-bit)` | `? <= 0x19 (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b026 | `eax` | `?` |

## `_kbdShift`

- `_kbdShift`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b028 to 0x10b068.
  Established. Taking the branch when `(([0x1d4ad1]:1 & 0xff) & 0x2) == 0x0` (`je` at 0x10b032)
  reaches the two-byte store of `0x1` at absolute `0x1d4ad2` (`mov` at 0x10b048); its fall-through
  reaches the separate two-byte store of `0x0` there (`mov` at 0x10b034). The two byte
  read-modify-writes at absolute `0x1d4ad0` retain unknown stored values (`and` at 0x10b03d, `or` at
  0x10b051).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `test` at 0x10b02b | load | 1 | `abs 0x1d4ad1` | `-` |
| `mov` at 0x10b034 | store | 2 | `abs 0x1d4ad2` | `0x0` |
| `and` at 0x10b03d | load | 1 | `abs 0x1d4ad0` | `-` |
| `and` at 0x10b03d | store | 1 | `abs 0x1d4ad0` | `?` |
| `mov` at 0x10b048 | store | 2 | `abs 0x1d4ad2` | `0x1` |
| `or` at 0x10b051 | load | 1 | `abs 0x1d4ad0` | `-` |
| `or` at 0x10b051 | store | 1 | `abs 0x1d4ad0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10b032 | `(([0x1d4ad1]:1 & 0xff) & 0x2) == 0x0` | `(([0x1d4ad1]:1 & 0xff) & 0x2) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b047 | `eax` | `eax` |
| `ret` at 0x10b05b | `eax` | `eax` |

## `_kbdCtrl`

- `_kbdCtrl`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b068 to 0x10b0a8.
  Established. The two-byte store to absolute `0x1d4ad2` writes `0x2` on the taken arm (`mov` at
  0x10b088), selected when `(([0x1d4ad1]:1 & 0xff) & 0x2) == 0x0` (`je` at 0x10b072). The
  fall-through instead stores `0x0` at that address (`mov` at 0x10b074). Both exits preserve the
  entry expression `eax` (`ret` at 0x10b087, `ret` at 0x10b09b).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `test` at 0x10b06b | load | 1 | `abs 0x1d4ad1` | `-` |
| `mov` at 0x10b074 | store | 2 | `abs 0x1d4ad2` | `0x0` |
| `and` at 0x10b07d | load | 1 | `abs 0x1d4ad0` | `-` |
| `and` at 0x10b07d | store | 1 | `abs 0x1d4ad0` | `?` |
| `mov` at 0x10b088 | store | 2 | `abs 0x1d4ad2` | `0x2` |
| `or` at 0x10b091 | load | 1 | `abs 0x1d4ad0` | `-` |
| `or` at 0x10b091 | store | 1 | `abs 0x1d4ad0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10b072 | `(([0x1d4ad1]:1 & 0xff) & 0x2) == 0x0` | `(([0x1d4ad1]:1 & 0xff) & 0x2) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b087 | `eax` | `eax` |
| `ret` at 0x10b09b | `eax` | `eax` |

## `_kbdNum`

- `_kbdNum`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b0c8 to 0x10b108.
  Established. The initial taken edge bypasses the mutations when `(([0x1d4ad1]:1 & 0xff) & 0x2) !=
  0x0` (`jne` at 0x10b0d2). The remaining path includes a byte read-modify-write at absolute
  `0x1d4ad0` (`xor` at 0x10b0d4) and a two-byte store at absolute `0x1d4ad2` (`mov` at 0x10b0eb),
  both with unknown stored values. The `_kbdLedSet` site has unknown candidates (`call` at
  0x10b0f1).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `test` at 0x10b0cb | load | 1 | `abs 0x1d4ad1` | `-` |
| `xor` at 0x10b0d4 | load | 1 | `abs 0x1d4ad0` | `-` |
| `xor` at 0x10b0d4 | store | 1 | `abs 0x1d4ad0` | `?` |
| `test` at 0x10b0dd | load | 1 | `abs 0x1d4ad0` | `-` |
| `mov` at 0x10b0eb | store | 2 | `abs 0x1d4ad2` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b0f1 | `_kbdLedSet 0x10b2c8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10b0d2 | `(([0x1d4ad1]:1 & 0xff) & 0x2) != 0x0` | `(([0x1d4ad1]:1 & 0xff) & 0x2) == 0x0` |
| `jne` at 0x10b0e4 | `(([0x1d4ad0]:1 & 0xff) & 0x2) != 0x0` | `(([0x1d4ad0]:1 & 0xff) & 0x2) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b0f9 | `eax` | `?` |

## `_kbdCaps`

- `_kbdCaps`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b0a8 to 0x10b0c8.
  Established. The branch falls through when `(([0x1d4ad1]:1 & 0xff) & 0x2) == 0x0` (`jne` at
  0x10b0b2), reaching the byte read-modify-write at absolute `0x1d4ad0` with unknown stored value
  (`xor` at 0x10b0b4). The following `_kbdLedSet` call retains unknown candidate slots (`call` at
  0x10b0bb). No indicator meaning is assigned to that state.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `test` at 0x10b0ab | load | 1 | `abs 0x1d4ad1` | `-` |
| `xor` at 0x10b0b4 | load | 1 | `abs 0x1d4ad0` | `-` |
| `xor` at 0x10b0b4 | store | 1 | `abs 0x1d4ad0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b0bb | `_kbdLedSet 0x10b2c8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10b0b2 | `(([0x1d4ad1]:1 & 0xff) & 0x2) != 0x0` | `(([0x1d4ad1]:1 & 0xff) & 0x2) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b0c3 | `eax` | `?` |

## `_kbdStp`

- `_kbdStp`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b108 to 0x10b168.
  Established. A four-byte load reads absolute `0x1d4ad4` (`mov` at 0x10b10b). The first branch
  bypasses both call sites when `(([0x1d4ad1]:1 & 0xff) & 0x2) != 0x0` (`jne` at 0x10b118). On the
  remaining path, `_tyIRd` (`call` at 0x10b14e) and `_kbdLedSet` (`call` at 0x10b156) retain unknown
  candidate values. The byte read-modify-write at absolute `0x1d4ad0` also has unknown stored value
  (`xor` at 0x10b11a).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b10b | load | 4 | `abs 0x1d4ad4` | `-` |
| `test` at 0x10b111 | load | 1 | `abs 0x1d4ad1` | `-` |
| `xor` at 0x10b11a | load | 1 | `abs 0x1d4ad0` | `-` |
| `xor` at 0x10b11a | store | 1 | `abs 0x1d4ad0` | `?` |
| `test` at 0x10b121 | load | 1 | `abs 0x1d4ad0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b14e | `_tyIRd 0x16fe88` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b156 | `_kbdLedSet 0x10b2c8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10b118 | `(([0x1d4ad1]:1 & 0xff) & 0x2) != 0x0` | `(([0x1d4ad1]:1 & 0xff) & 0x2) == 0x0` |
| `je` at 0x10b128 | `(([0x1d4ad0]:1 & 0xff) & 0x1) == 0x0` | `(([0x1d4ad0]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b15e | `eax` | `?` |

## `_kbdExt`

- `_kbdExt`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b168 to 0x10b228.
  Established. The first `_tyIRd` site presents unknown candidate zero and candidate one `0x1b`
  (`call` at 0x10b1c8). The next site keeps all candidates unknown (`call` at 0x10b1eb); its
  prior branch is taken when `[0x1d4ac8] == 0x0` (`je` at 0x10b1d7). The byte store at `sp -0x8`
  has unknown value (`mov` at 0x10b1af). No emitted sequence is inferred across these candidates.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b170 | load | 1 | `sp +0x4` | `-` |
| `mov` at 0x10b173 | load | 4 | `abs 0x1d4ad4` | `-` |
| `test` at 0x10b179 | load | 1 | `abs 0x1d4ad1` | `-` |
| `mov` at 0x10b186 | load | 4 | `abs 0x1d4acc` | `-` |
| `movzx` at 0x10b192 | load | 2 | `abs 0x1d4ad2` | `-` |
| `mov` at 0x10b1ac | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10b1af | store | 1 | `sp -0x8` | `?` |
| `cmp` at 0x10b1d0 | load | 4 | `abs 0x1d4ac8` | `-` |
| `movsx` at 0x10b1f3 | load | 1 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b1c8 | `_tyIRd 0x16fe88` | `?` | `0x1b` | `ebx` | `esi` | `?` | `ebp` |
| `call` at 0x10b1eb | `_tyIRd 0x16fe88` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b20c | `_tyIRd 0x16fe88` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10b180 | `(([0x1d4ad1]:1 & 0xff) & 0x2) != 0x0` | `(([0x1d4ad1]:1 & 0xff) & 0x2) == 0x0` |
| `je` at 0x10b1d7 | `[0x1d4ac8] == 0x0` | `[0x1d4ac8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b219 | `eax` | `?` |

## `_kbdEs`

- `_kbdEs`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b228 to 0x10b2c8.
  Established. Three `_tyIRd` sites include candidate one `0x1b` (`call` at 0x10b28c), candidate one
  `0x4f` (`call` at 0x10b297), and unknown candidate values (`call` at 0x10b2b8). The branch
  bypasses the first two when `(([0x1d4ad0]:1 & 0xff) & 0x2) != 0x0` (`jne` at 0x10b274). The
  earlier indexed byte load retains an unknown effective address (`mov` at 0x10b267).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b230 | load | 1 | `sp +0x4` | `-` |
| `mov` at 0x10b233 | load | 4 | `abs 0x1d4ad4` | `-` |
| `test` at 0x10b239 | load | 1 | `abs 0x1d4ad1` | `-` |
| `mov` at 0x10b242 | load | 4 | `abs 0x1d4acc` | `-` |
| `movzx` at 0x10b24d | load | 2 | `abs 0x1d4ad2` | `-` |
| `mov` at 0x10b267 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10b26a | store | 1 | `sp -0x8` | `?` |
| `test` at 0x10b26d | load | 1 | `abs 0x1d4ad0` | `-` |
| `movsx` at 0x10b29f | load | 1 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b28c | `_tyIRd 0x16fe88` | `?` | `0x1b` | `ebx` | `esi` | `?` | `ebp` |
| `call` at 0x10b297 | `_tyIRd 0x16fe88` | `?` | `0x4f` | `?` | `?` | `?` | `?` |
| `call` at 0x10b2b8 | `_tyIRd 0x16fe88` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10b240 | `(([0x1d4ad1]:1 & 0xff) & 0x2) != 0x0` | `(([0x1d4ad1]:1 & 0xff) & 0x2) == 0x0` |
| `jne` at 0x10b274 | `(([0x1d4ad0]:1 & 0xff) & 0x2) != 0x0` | `(([0x1d4ad0]:1 & 0xff) & 0x2) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b2c5 | `eax` | `?` |

## `_pcConDrvOpen`

- `_pcConDrvOpen`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108988 to 0x108998.
  Established. The four-byte load uses `sp +0x4` (`mov` at 0x10898b). The exit reports `[sp+4]` in
  `eax` (`ret` at 0x108991), preserving that entry stack value. A device-open effect is not
  established by the wrapper; its role remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10898b | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108991 | `eax` | `[sp+4]` |

## `_kbRead`

- `_kbRead`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16daa8 to 0x16dae8.
  Established. The `_sysInByte` calls present candidate zero `0x64` (`call` at 0x16dab2) and `0x60`
  (`call` at 0x16dac3). The branch falls through to the second site when `(? & 0x1) != 0x0` (`je` at
  0x16dabf). A byte store through `[sp+4] +0x0` has unknown value (`mov` at 0x16dac8). The tested
  value and hardware reply remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16daad | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x16dac8 | store | 1 | `[sp+4] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16dab2 | `_sysInByte 0x1081b0` | `0x64` | `ebx` | `esi` | `ebp` | `?` | `?` |
| `call` at 0x16dac3 | `_sysInByte 0x1081b0` | `0x60` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x16dabf | `(? & 0x1) == 0x0` | `(? & 0x1) != 0x0` |
| `jne` at 0x16dacd | `(? & 0xc0) != 0x0` | `(? & 0xc0) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16dae5 | `eax` | `?` |

## `_kbWrite`

- `_kbWrite`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16dae8 to 0x16db38.
  Established. A byte store copies `[sp+4]:1` to `sp -0x8` (`mov` at 0x16daf2). The `_sysInByte`
  site presents candidate zero `0x64` (`call` at 0x16dafe). Falling through when `(? & 0x2) == 0x0`
  (`jne` at 0x16db08) reaches `_sysOutByte` with candidate zero `0x60` and unknown candidate one
  (`call` at 0x16db11). The repeat edge is taken when `Z == 0` (`jne` at 0x16db29).
  The tool invalidates unsupported effects of the prior `dec` at 0x16db28, so the
  branch table retains the flag expression.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16daef | load | 1 | `sp +0x4` | `-` |
| `mov` at 0x16daf2 | store | 1 | `sp -0x8` | `[sp+4]:1` |
| `movsx` at 0x16db0a | load | 1 | `sp -0x8` | `-` |
| `mov` at 0x16db30 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16dafe | `_sysInByte 0x1081b0` | `0x64` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16db11 | `_sysOutByte 0x1081d0` | `0x60` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x16db08 | `(? & 0x2) != 0x0` | `(? & 0x2) == 0x0` |
| `jne` at 0x16db29 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16db36 | `eax` | `?` |

## `_kbLedUpdate`

- `_kbLedUpdate`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16db38 to 0x16db68.
  Established. The first `_kbWrite` call has candidate zero `0xed` (`call` at 0x16db40). A later
  four-byte load reads absolute `0x1d23fc` (`mov` at 0x16db48), but the second `_kbWrite` site keeps
  all candidate values unknown (`call` at 0x16db54). The exit reports `ret(_kbWrite)` in `eax`
  (`ret` at 0x16db5c). The command meaning remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16db48 | load | 4 | `abs 0x1d23fc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16db40 | `_kbWrite 0x16dae8` | `0xed` | `ebp` | `?` | `?` | `?` | `?` |
| `call` at 0x16db54 | `_kbWrite 0x16dae8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16db5c | `eax` | `ret(_kbWrite)` |

## `_kbISR`

- `_kbISR`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16db68 to 0x16dfe8.
  Established. The `_kbRead` site has candidate zero `sp-5` (`call` at 0x16db86); its result branch
  falls through when `(ret(_kbRead) & ret(_kbRead)) == 0x0` (`jne` at 0x16db90). A later branch
  falls through when `([sp-5]:1 & 0xff) == 0xe0` (`jne` at 0x16dbc8), reaching the four-byte store
  of `0x1` to absolute `0x1d23e4` (`mov` at 0x16dbca). The `_mwKbHookKeyDown` sites include
  candidate zero `0xffffffff` (`call` at 0x16df89) and `0xffff0000` (`call` at 0x16dfbe). Candidate
  one remains unknown at both. No key encoding is established. The tool invalidates
  unsupported effects of `inc` at 0x16dbc3 and `rol` at 0x16dd48.
  Unknown expressions in the tables remain unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16db73 | store | 2 | `sp -0xc` | `0xffff` |
| `mov` at 0x16db79 | load | 4 | `abs 0x1d23fc` | `-` |
| `mov` at 0x16db7f | store | 4 | `sp -0x10` | `[0x1d23fc]` |
| `test` at 0x16db96 | load | 1 | `abs 0x1d23fc` | `-` |
| `test` at 0x16dba8 | load | 1 | `abs 0x1d23fc` | `-` |
| `test` at 0x16dbba | load | 1 | `abs 0x1d23fc` | `-` |
| `cmp` at 0x16dbc4 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x16dbca | store | 4 | `abs 0x1d23e4` | `0x1` |
| `cmp` at 0x16dbe8 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x16dbee | store | 4 | `abs 0x1d23e8` | `0x1` |
| `cmp` at 0x16dc08 | load | 4 | `abs 0x1d23e8` | `-` |
| `mov` at 0x16dc11 | store | 4 | `abs 0x1d23e8` | `0x0` |
| `cmp` at 0x16dc1b | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x16dc21 | store | 4 | `abs 0x1d23ec` | `0x1` |
| `mov` at 0x16dc2b | store | 4 | `abs 0x1d23f0` | `0x1` |
| `cmp` at 0x16dc38 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x16dc3e | store | 4 | `abs 0x1d23ec` | `0x0` |
| `cmp` at 0x16dc58 | load | 4 | `abs 0x1d23f0` | `-` |
| `cmp` at 0x16dc61 | load | 1 | `sp -0x5` | `-` |
| `cmp` at 0x16dc78 | load | 1 | `sp -0x5` | `-` |
| `or` at 0x16dc82 | load | 4 | `abs 0x1d23fc` | `-` |
| `or` at 0x16dc82 | store | 4 | `abs 0x1d23fc` | `?` |
| `mov` at 0x16dc8c | store | 4 | `abs 0x1d23f0` | `0x0` |
| `cmp` at 0x16dca8 | load | 1 | `sp -0x5` | `-` |
| `and` at 0x16dcb2 | load | 1 | `sp -0x5` | `-` |
| `and` at 0x16dcb2 | store | 1 | `sp -0x5` | `?` |
| `cmp` at 0x16dcb6 | load | 4 | `abs 0x1d23e4` | `-` |
| `cmp` at 0x16dcbf | load | 1 | `sp -0x5` | `-` |
| `movzx` at 0x16dcc5 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x16dccb | load | 1 | `([sp-5]:1 + [sp-5]:1) +0x1d1f42` | `-` |
| `movzx` at 0x16dcd1 | load | 1 | `([sp-5]:1 + [sp-5]:1) +0x1d1f43` | `-` |
| `mov` at 0x16dcd9 | store | 4 | `abs 0x1d23e4` | `0x0` |
| `cmp` at 0x16dce8 | load | 1 | `sp -0x5` | `-` |
| `movzx` at 0x16dcee | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x16dcf4 | load | 1 | `([sp-5]:1 + [sp-5]:1) +0x1d1e90` | `-` |
| `movzx` at 0x16dcfa | load | 1 | `([sp-5]:1 + [sp-5]:1) +0x1d1e91` | `-` |
| `cmp` at 0x16dd16 | load | 1 | `sp -0x5` | `-` |
| `test` at 0x16dd1c | load | 1 | `abs 0x1d23fc` | `-` |
| `test` at 0x16dd2b | load | 1 | `abs 0x1d23fc` | `-` |
| `and` at 0x16dd4a | load | 4 | `abs 0x1d23fc` | `-` |
| `and` at 0x16dd4a | store | 4 | `abs 0x1d23fc` | `?` |
| `cmp` at 0x16dd50 | load | 4 | `abs 0x1d23f4` | `-` |
| `test` at 0x16dd5d | load | 4 | `abs 0x1d23fc` | `-` |
| `mov` at 0x16dd6d | store | 4 | `abs 0x1d23f4` | `0x0` |
| `movzx` at 0x16dd77 | load | 1 | `abs 0x1d23f8` | `-` |
| `mov` at 0x16dd7f | store | 2 | `sp -0xc` | `[0x1d23f8]:1` |
| `mov` at 0x16dd83 | store | 1 | `abs 0x1d23f8` | `0x0` |
| `cmp` at 0x16dd98 | load | 4 | `abs 0x1d23e4` | `-` |
| `cmp` at 0x16dda1 | load | 1 | `sp -0x5` | `-` |
| `movzx` at 0x16dda7 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x16ddad | load | 1 | `([sp-5]:1 + [sp-5]:1) +0x1d1f42` | `-` |
| `movzx` at 0x16ddb3 | load | 1 | `([sp-5]:1 + [sp-5]:1) +0x1d1f43` | `-` |
| `mov` at 0x16ddbb | store | 4 | `abs 0x1d23e4` | `0x0` |
| `cmp` at 0x16ddc8 | load | 1 | `sp -0x5` | `-` |
| `movzx` at 0x16ddce | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x16ddd4 | load | 1 | `([sp-5]:1 + [sp-5]:1) +0x1d1e90` | `-` |
| `movzx` at 0x16ddda | load | 1 | `([sp-5]:1 + [sp-5]:1) +0x1d1e91` | `-` |
| `test` at 0x16ddf1 | load | 1 | `abs 0x1d23fc` | `-` |
| `mov` at 0x16de04 | store | 4 | `abs 0x1d23f4` | `0x1` |
| `movzx` at 0x16de0e | load | 1 | `abs 0x1d23f8` | `-` |
| `mov` at 0x16de1c | store | 1 | `abs 0x1d23f8` | `?` |
| `mov` at 0x16de28 | store | 4 | `abs 0x1d23f4` | `0x0` |
| `mov` at 0x16de32 | store | 1 | `abs 0x1d23f8` | `0x0` |
| `mov` at 0x16de39 | load | 4 | `abs 0x1d23fc` | `-` |
| `mov` at 0x16de60 | load | 4 | `abs 0x1d23fc` | `-` |
| `cmp` at 0x16de74 | load | 1 | `sp -0x5` | `-` |
| `test` at 0x16de7a | load | 1 | `abs 0x1d23fc` | `-` |
| `or` at 0x16de83 | load | 4 | `abs 0x1d23fc` | `-` |
| `or` at 0x16de83 | store | 4 | `abs 0x1d23fc` | `?` |
| `test` at 0x16dea7 | load | 4 | `abs 0x1d23fc` | `-` |
| `xor` at 0x16deb8 | load | 4 | `abs 0x1d23fc` | `-` |
| `xor` at 0x16deb8 | store | 4 | `abs 0x1d23fc` | `?` |
| `or` at 0x16debe | load | 4 | `abs 0x1d23fc` | `-` |
| `or` at 0x16debe | store | 4 | `abs 0x1d23fc` | `?` |
| `test` at 0x16ded9 | load | 1 | `abs 0x1d23fc` | `-` |
| `or` at 0x16defa | load | 4 | `abs 0x1d23fc` | `-` |
| `or` at 0x16defa | store | 4 | `abs 0x1d23fc` | `?` |
| `and` at 0x16df00 | load | 4 | `abs 0x1d23fc` | `-` |
| `and` at 0x16df00 | store | 4 | `abs 0x1d23fc` | `?` |
| `test` at 0x16df07 | load | 4 | `abs 0x1d23fc` | `-` |
| `or` at 0x16df13 | load | 1 | `abs 0x1d23fc` | `-` |
| `or` at 0x16df13 | store | 1 | `abs 0x1d23fc` | `?` |
| `and` at 0x16df1a | load | 4 | `abs 0x1d23fc` | `-` |
| `and` at 0x16df1a | store | 4 | `abs 0x1d23fc` | `?` |
| `test` at 0x16df21 | load | 4 | `abs 0x1d23fc` | `-` |
| `or` at 0x16df2d | load | 1 | `abs 0x1d23fc` | `-` |
| `or` at 0x16df2d | store | 1 | `abs 0x1d23fc` | `?` |
| `cmp` at 0x16df34 | load | 2 | `sp -0xc` | `-` |
| `mov` at 0x16df42 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x16df4a | store | 2 | `sp -0xc` | `?` |
| `test` at 0x16df4e | load | 1 | `abs 0x1d23fd` | `-` |
| `cmp` at 0x16df57 | load | 2 | `sp -0xc` | `-` |
| `and` at 0x16df5f | load | 4 | `abs 0x1d23fc` | `-` |
| `and` at 0x16df5f | store | 4 | `abs 0x1d23fc` | `?` |
| `mov` at 0x16df69 | store | 2 | `sp -0xc` | `0xffff` |
| `cmp` at 0x16df6f | load | 2 | `sp -0xc` | `-` |
| `mov` at 0x16df77 | load | 2 | `abs 0x1d23fc` | `-` |
| `movzx` at 0x16df80 | load | 2 | `sp -0xc` | `-` |
| `mov` at 0x16df98 | load | 4 | `sp -0x10` | `-` |
| `xor` at 0x16df9b | load | 4 | `abs 0x1d23fc` | `-` |
| `test` at 0x16dfa7 | load | 4 | `abs 0x1d23fc` | `-` |
| `mov` at 0x16dfaf | load | 2 | `abs 0x1d23fc` | `-` |
| `mov` at 0x16dfc8 | load | 2 | `abs 0x1d23fc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16db86 | `_kbRead 0x16daa8` | `sp-5` | `ebx` | `esi` | `edi` | `[0x1d23fc]` | `?` |
| `call` at 0x16dec4 | `_kbLedUpdate 0x16db38` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16df89 | `_mwKbHookKeyDown 0x14b6b4` | `0xffffffff` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16dfbe | `_mwKbHookKeyDown 0x14b6b4` | `0xffff0000` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16dfd7 | `_mwKbHookKeyUp 0x14b6c3` | `0xffff0000` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x16db90 | `(ret(_kbRead) & ret(_kbRead)) != 0x0` | `(ret(_kbRead) & ret(_kbRead)) == 0x0` |
| `je` at 0x16db9d | `(([0x1d23fc]:1 & 0xff) & 0x8) == 0x0` | `(([0x1d23fc]:1 & 0xff) & 0x8) != 0x0` |
| `je` at 0x16dbaf | `(([0x1d23fc]:1 & 0xff) & 0x4) == 0x0` | `(([0x1d23fc]:1 & 0xff) & 0x4) != 0x0` |
| `je` at 0x16dbc1 | `(([0x1d23fc]:1 & 0xff) & 0x3) == 0x0` | `(([0x1d23fc]:1 & 0xff) & 0x3) != 0x0` |
| `jne` at 0x16dbc8 | `([sp-5]:1 & 0xff) != 0xe0` | `([sp-5]:1 & 0xff) == 0xe0` |
| `jne` at 0x16dbec | `([sp-5]:1 & 0xff) != 0xe1` | `([sp-5]:1 & 0xff) == 0xe1` |
| `je` at 0x16dc0f | `[0x1d23e8] == 0x0` | `[0x1d23e8] != 0x0` |
| `jne` at 0x16dc1f | `([sp-5]:1 & 0xff) != 0x1d` | `([sp-5]:1 & 0xff) == 0x1d` |
| `jne` at 0x16dc3c | `([sp-5]:1 & 0xff) != 0x9d` | `([sp-5]:1 & 0xff) == 0x9d` |
| `je` at 0x16dc5f | `[0x1d23f0] == 0x0` | `[0x1d23f0] != 0x0` |
| `jne` at 0x16dc65 | `([sp-5]:1 & 0xff) != 0x45` | `([sp-5]:1 & 0xff) == 0x45` |
| `jne` at 0x16dc7c | `([sp-5]:1 & 0xff) != 0xc5` | `([sp-5]:1 & 0xff) == 0xc5` |
| `jge` at 0x16dcac | `([sp-5]:1 & 0xff) >= 0x0 (signed, 8-bit)` | `([sp-5]:1 & 0xff) < 0x0 (signed, 8-bit)` |
| `je` at 0x16dcbd | `[0x1d23e4] == 0x0` | `[0x1d23e4] != 0x0` |
| `ja` at 0x16dcc3 | `([sp-5]:1 & 0xff) > 0x54 (unsigned, 8-bit)` | `([sp-5]:1 & 0xff) <= 0x54 (unsigned, 8-bit)` |
| `ja` at 0x16dcec | `([sp-5]:1 & 0xff) > 0x58 (unsigned, 8-bit)` | `([sp-5]:1 & 0xff) <= 0x58 (unsigned, 8-bit)` |
| `je` at 0x16dd0f | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x16dd1a | `([sp-5]:1 & 0xff) != 0x45` | `([sp-5]:1 & 0xff) == 0x45` |
| `jne` at 0x16dd23 | `(([0x1d23fc]:1 & 0xff) & 0x4) != 0x0` | `(([0x1d23fc]:1 & 0xff) & 0x4) == 0x0` |
| `jne` at 0x16dd29 | `Z == 0` | `not (Z == 0)` |
| `je` at 0x16dd32 | `(([0x1d23fc]:1 & 0xff) & 0x8) == 0x0` | `(([0x1d23fc]:1 & 0xff) & 0x8) != 0x0` |
| `ja` at 0x16dd3e | `? > 0x1 (unsigned, 32-bit)` | `? <= 0x1 (unsigned, 32-bit)` |
| `je` at 0x16dd57 | `[0x1d23f4] == 0x0` | `[0x1d23f4] != 0x0` |
| `jne` at 0x16dd67 | `([0x1d23fc] & 0x8000200) != 0x0` | `([0x1d23fc] & 0x8000200) == 0x0` |
| `je` at 0x16dd9f | `[0x1d23e4] == 0x0` | `[0x1d23e4] != 0x0` |
| `ja` at 0x16dda5 | `([sp-5]:1 & 0xff) > 0x54 (unsigned, 8-bit)` | `([sp-5]:1 & 0xff) <= 0x54 (unsigned, 8-bit)` |
| `ja` at 0x16ddcc | `([sp-5]:1 & 0xff) > 0x58 (unsigned, 8-bit)` | `([sp-5]:1 & 0xff) <= 0x58 (unsigned, 8-bit)` |
| `je` at 0x16ddef | `Z == 1` | `not (Z == 1)` |
| `je` at 0x16ddf8 | `(([0x1d23fc]:1 & 0xff) & 0x8) == 0x0` | `(([0x1d23fc]:1 & 0xff) & 0x8) != 0x0` |
| `ja` at 0x16de02 | `a flags condition (operands unknown)` | `not (a flags condition (operands unknown))` |
| `jne` at 0x16de44 | `? != 0x20` | `? == 0x20` |
| `jne` at 0x16de4b | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `jge` at 0x16de4f | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x16de5e | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x16de6b | `? != 0x40` | `? == 0x40` |
| `jge` at 0x16de72 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x16de78 | `([sp-5]:1 & 0xff) != 0x45` | `([sp-5]:1 & 0xff) == 0x45` |
| `je` at 0x16de81 | `(([0x1d23fc]:1 & 0xff) & 0x4) == 0x0` | `(([0x1d23fc]:1 & 0xff) & 0x4) != 0x0` |
| `jne` at 0x16dead | `([0x1d23fc] & ?) != 0x0` | `([0x1d23fc] & ?) == 0x0` |
| `je` at 0x16decc | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x16ded7 | `Z == 0` | `not (Z == 0)` |
| `je` at 0x16dee0 | `(([0x1d23fc]:1 & 0xff) & 0x8) == 0x0` | `(([0x1d23fc]:1 & 0xff) & 0x8) != 0x0` |
| `ja` at 0x16deee | `? > 0x1 (unsigned, 32-bit)` | `? <= 0x1 (unsigned, 32-bit)` |
| `je` at 0x16df11 | `([0x1d23fc] & 0x4000100) == 0x0` | `([0x1d23fc] & 0x4000100) != 0x0` |
| `je` at 0x16df2b | `([0x1d23fc] & 0x8000200) == 0x0` | `([0x1d23fc] & 0x8000200) != 0x0` |
| `jne` at 0x16df3a | `([sp-12]:2 & 0xffff) != 0xffff` | `([sp-12]:2 & 0xffff) == 0xffff` |
| `je` at 0x16df55 | `(([0x1d23fd]:1 & 0xff) & 0x8) == 0x0` | `(([0x1d23fd]:1 & 0xff) & 0x8) != 0x0` |
| `je` at 0x16df5d | `([sp-12]:2 & 0xffff) == 0xffff` | `([sp-12]:2 & 0xffff) != 0xffff` |
| `je` at 0x16df75 | `([sp-12]:2 & 0xffff) == 0xffff` | `([sp-12]:2 & 0xffff) != 0xffff` |
| `je` at 0x16dfa5 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x16dfad | `([0x1d23fc] & ?) == 0x0` | `([0x1d23fc] & ?) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16dfe5 | `eax` | `?` |

## `_vga_screenoff`

- `_vga_screenoff`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16f3e8 to 0x16f428.
  Established. The initial `_sysOutByte` site presents leading candidates `0x3c4` and `0x1` (`call`
  at 0x16f3f2). The `_sysInByte` site has candidate zero `0x3c5` (`call` at 0x16f3ff). The
  subsequent `_sysOutByte` call retains candidate zero `0x3c5` and unknown candidate one (`call` at
  0x16f412); its exit value is `0x0` (`ret` at 0x16f41c). No display effect is inferred from the
  name.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16f3f2 | `_sysOutByte 0x1081d0` | `0x3c4` | `0x1` | `ebp` | `?` | `?` | `?` |
| `call` at 0x16f3ff | `_sysInByte 0x1081b0` | `0x3c5` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16f412 | `_sysOutByte 0x1081d0` | `0x3c5` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16f41c | `eax` | `0x0` |

## `_vga_screenon`

- `_vga_screenon`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16f428 to 0x16f468.
  Established. The `_sysInByte` call presents candidate zero `0x3c5` (`call` at 0x16f43f); its
  surrounding `_sysOutByte` sites have leading pair `0x3c4`, `0x1` (`call` at 0x16f432) and leading
  pair `0x3c5`, unknown (`call` at 0x16f452). The sole exit reports `0x0` (`ret` at 0x16f45c). The
  output value and display role remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16f432 | `_sysOutByte 0x1081d0` | `0x3c4` | `0x1` | `ebp` | `?` | `?` | `?` |
| `call` at 0x16f43f | `_sysInByte 0x1081b0` | `0x3c5` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16f452 | `_sysOutByte 0x1081d0` | `0x3c5` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16f45c | `eax` | `0x0` |
