# MMSC VGA helpers and low-level I/O routines in the 6.4-origin image

Read from `6.4-origin/usr/cpu/firmware/mmscfw.bin`.
Image SHA-256: `b766e559f86078526916d8bd98bbb8079294ad774cce9177928873153c6fb09b`.
The x86 a.out link address is `0x108000`. The [load map](load-map.md) explains segment mapping.
Except for the five explicit bounds below, analysed spans have exclusive ends
inferred from the next text symbol.
Some include literals and padding decoded linearly after returns. Only sites reached
by known control-flow edges contribute effects to the tables.
Five explicit bounds extend across local continuation symbols to the next global text symbol:
`_sysA20on` uses 160 bytes, and each byte/word I/O wrapper uses 16 bytes.
Post-return bytes in the expanded spans do not contribute reached effects.
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

[Fact sheets and complete listings](disasm/vga-lowlevel-io-64-origin-facts.txt) include all tool sections.

## `_get_perm`

- `_get_perm`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16cd80 to 0x16cdf0.
  Established. The `_sysInByte` site presents candidate zero `0x3cc` (`call` at 0x16cd88). Four-byte
  stores at absolute `0x1d33e8` write `0x3d4` (`mov` at 0x16cd97) and `0x3b4` (`mov` at 0x16cdc0) on
  separate arms. Their deciding branch is taken when `Z == 1` (`je` at 0x16cd95). Port-selection or
  permission meanings are not inferred from these values.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16cd90 | store | 4 | `abs 0x1d33f4` | `?` |
| `mov` at 0x16cd97 | store | 4 | `abs 0x1d33e8` | `0x3d4` |
| `mov` at 0x16cda1 | store | 4 | `abs 0x1d33ec` | `0x3d5` |
| `mov` at 0x16cdab | store | 4 | `abs 0x1d33f0` | `0x3da` |
| `mov` at 0x16cdc0 | store | 4 | `abs 0x1d33e8` | `0x3b4` |
| `mov` at 0x16cdca | store | 4 | `abs 0x1d33ec` | `0x3b5` |
| `mov` at 0x16cdd4 | store | 4 | `abs 0x1d33f0` | `0x3ba` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16cd88 | `_sysInByte 0x1081b0` | `0x3cc` | `ebp` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x16cd95 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16cdb8 | `eax` | `?` |
| `ret` at 0x16cde1 | `eax` | `?` |

## `_et4000_test`

- `_et4000_test`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16cdf0 to 0x16ced0.
  Established. The `_sysOutByte` site presents leading candidates `0x3cd` and `0x55` (`call` at
  0x16ce0e). The following `_sysInByte` site has candidate zero `0x3cd` (`call` at 0x16ce1b). The
  later branch is taken when `([sp-8]:1 & 0xff) == 0x55` (`je` at 0x16ce3c). The exit value remains
  unknown (`ret` at 0x16cec3). No hardware-identification result is assigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16ce04 | store | 1 | `sp -0xc` | `?` |
| `mov` at 0x16ce23 | store | 1 | `sp -0x8` | `?` |
| `movsx` at 0x16ce26 | load | 1 | `sp -0xc` | `-` |
| `cmp` at 0x16ce38 | load | 1 | `sp -0x8` | `-` |
| `mov` at 0x16ce80 | store | 1 | `sp -0xc` | `?` |
| `mov` at 0x16ce88 | store | 1 | `sp -0x8` | `?` |
| `mov` at 0x16cea1 | store | 1 | `sp -0x10` | `?` |
| `movsx` at 0x16cea4 | load | 1 | `sp -0xc` | `-` |
| `mov` at 0x16ceaf | load | 1 | `sp -0x10` | `-` |
| `cmp` at 0x16ceb2 | load | 1 | `sp -0x8` | `-` |
| `mov` at 0x16cebd | load | 4 | `sp -0x14` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16cdfc | `_sysInByte 0x1081b0` | `0x3cd` | `ebx` | `?` | `?` | `?` | `ebp` |
| `call` at 0x16ce0e | `_sysOutByte 0x1081d0` | `0x3cd` | `0x55` | `?` | `?` | `?` | `?` |
| `call` at 0x16ce1b | `_sysInByte 0x1081b0` | `0x3cd` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16ce30 | `_sysOutByte 0x1081d0` | `0x3cd` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16ce55 | `_sysInByte 0x1081b0` | `0x3cc` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16ce6e | `_sysOutByte 0x1081d0` | `?` | `0x33` | `?` | `?` | `?` | `?` |
| `call` at 0x16ce78 | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16ce90 | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16ce99 | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16ceaa | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x16ce3c | `([sp-8]:1 & 0xff) == 0x55` | `([sp-8]:1 & 0xff) != 0x55` |
| `je` at 0x16ce64 | `(? & 0x1) == 0x0` | `(? & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16cec3 | `eax` | `?` |

## `_et4000_save_regs`

- `_et4000_save_regs`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16ced0 to 0x16cf80.
  Established. Byte stores through `[sp+4] +0x42` (`mov` at 0x16cf26), `+0x43` (`mov` at 0x16cf36),
  `+0x44` (`mov` at 0x16cf46), and `+0x45` (`mov` at 0x16cf70) retain unknown values. The
  `_sysOutByte` sites present leading pairs `0x3c4`, `0x7` (`call` at 0x16cf11) and `0x3c0`, `0x16`
  (`call` at 0x16cf5e). The stored bytes do not establish a saved-register record layout.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16ced5 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x16cee4 | load | 4 | `abs 0x1d33e8` | `-` |
| `mov` at 0x16cef2 | load | 4 | `abs 0x1d33ec` | `-` |
| `mov` at 0x16cf00 | store | 1 | `(unknown)` | `?` |
| `mov` at 0x16cf26 | store | 1 | `[sp+4] +0x42` | `?` |
| `mov` at 0x16cf36 | store | 1 | `[sp+4] +0x43` | `?` |
| `mov` at 0x16cf46 | store | 1 | `[sp+4] +0x44` | `?` |
| `mov` at 0x16cf49 | load | 4 | `abs 0x1d33f0` | `-` |
| `mov` at 0x16cf70 | store | 1 | `[sp+4] +0x45` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16ceea | `_sysOutByte 0x1081d0` | `[0x1d33e8]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16cef8 | `_sysInByte 0x1081b0` | `[0x1d33ec]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16cf11 | `_sysOutByte 0x1081d0` | `0x3c4` | `0x7` | `?` | `?` | `?` | `?` |
| `call` at 0x16cf1e | `_sysInByte 0x1081b0` | `0x3c5` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16cf2e | `_sysInByte 0x1081b0` | `0x3c3` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16cf3e | `_sysInByte 0x1081b0` | `0x3cd` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16cf4f | `_sysInByte 0x1081b0` | `[0x1d33f0]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16cf5e | `_sysOutByte 0x1081d0` | `0x3c0` | `0x16` | `?` | `?` | `?` | `?` |
| `call` at 0x16cf6b | `_sysInByte 0x1081b0` | `0x3c1` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x16cf08 | `? <= 0x5 (signed, 32-bit)` | `? > 0x5 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16cf7b | `eax` | `ret(_sysInByte)` |

## `_et4000_set_regs`

- `_et4000_set_regs`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16cf80 to 0x16d040.
  Established. The byte loads read through `[sp+4] +0x43` (`movsx` at 0x16cf88) and `[sp+4] +0x44`
  (`movsx` at 0x16cf9a). The `_sysOutByte` sites present candidate zero `0x3c3` (`call` at 0x16cf92)
  and `0x3cd` (`call` at 0x16cfa4), each with unknown candidate one. A later site presents `0x3c4`
  and `0x7` as its leading candidates (`call` at 0x16cfb3). No register-restoration effect is
  inferred.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16cf85 | load | 4 | `sp +0x4` | `-` |
| `movsx` at 0x16cf88 | load | 1 | `[sp+4] +0x43` | `-` |
| `movsx` at 0x16cf9a | load | 1 | `[sp+4] +0x44` | `-` |
| `movsx` at 0x16cfbb | load | 1 | `[sp+4] +0x42` | `-` |
| `mov` at 0x16cfd8 | load | 4 | `abs 0x1d33e8` | `-` |
| `movsx` at 0x16cfe6 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x16cfec | load | 4 | `abs 0x1d33ec` | `-` |
| `mov` at 0x16d000 | load | 4 | `abs 0x1d33f0` | `-` |
| `movsx` at 0x16d01d | load | 1 | `[sp+4] +0x45` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16cf92 | `_sysOutByte 0x1081d0` | `0x3c3` | `?` | `ebx` | `esi` | `ebp` | `?` |
| `call` at 0x16cfa4 | `_sysOutByte 0x1081d0` | `0x3cd` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16cfb3 | `_sysOutByte 0x1081d0` | `0x3c4` | `0x7` | `?` | `?` | `?` | `?` |
| `call` at 0x16cfc5 | `_sysOutByte 0x1081d0` | `0x3c5` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16cfde | `_sysOutByte 0x1081d0` | `[0x1d33e8]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16cff2 | `_sysOutByte 0x1081d0` | `[0x1d33ec]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d006 | `_sysInByte 0x1081b0` | `[0x1d33f0]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d015 | `_sysOutByte 0x1081d0` | `0x3c0` | `0x16` | `?` | `?` | `?` | `?` |
| `call` at 0x16d027 | `_sysOutByte 0x1081d0` | `0x3c0` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x16cffe | `? <= 0x5 (signed, 32-bit)` | `? > 0x5 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16d034 | `eax` | `ret(_sysOutByte)` |

## `_save_regs`

- `_save_regs`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16d040 to 0x16d130.
  Established. The byte store through `[sp+4] +0x3b` retains an unknown value (`mov` at 0x16d111).
  Earlier indexed byte stores have unknown effective addresses and values (`mov` at 0x16d06c, `mov`
  at 0x16d0a4, `mov` at 0x16d0ce, `mov` at 0x16d0fa). The `_et4000_save_regs` site presents
  candidate zero `[sp+4]` (`call` at 0x16d11e). Record size and hardware-register roles remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16d045 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x16d050 | load | 4 | `abs 0x1d33e8` | `-` |
| `mov` at 0x16d05e | load | 4 | `abs 0x1d33ec` | `-` |
| `mov` at 0x16d06c | store | 1 | `(unknown)` | `?` |
| `mov` at 0x16d078 | load | 4 | `abs 0x1d33f0` | `-` |
| `mov` at 0x16d0a4 | store | 1 | `(unknown)` | `?` |
| `mov` at 0x16d0ce | store | 1 | `(unknown)` | `?` |
| `mov` at 0x16d0fa | store | 1 | `(unknown)` | `?` |
| `mov` at 0x16d111 | store | 1 | `[sp+4] +0x3b` | `?` |
| `cmp` at 0x16d114 | load | 4 | `abs 0x1d06dc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16d056 | `_sysOutByte 0x1081d0` | `[0x1d33e8]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d064 | `_sysInByte 0x1081b0` | `[0x1d33ec]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d07e | `_sysInByte 0x1081b0` | `[0x1d33f0]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d08f | `_sysOutByte 0x1081d0` | `0x3c0` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d09c | `_sysInByte 0x1081b0` | `0x3c1` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d0b9 | `_sysOutByte 0x1081d0` | `0x3ce` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d0c6 | `_sysInByte 0x1081b0` | `0x3cf` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d0e5 | `_sysOutByte 0x1081d0` | `0x3c4` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d0f2 | `_sysInByte 0x1081b0` | `0x3c5` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d109 | `_sysInByte 0x1081b0` | `0x3cc` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16d11e | `_et4000_save_regs 0x16ced0` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x16d073 | `? <= 0x17 (signed, 32-bit)` | `? > 0x17 (signed, 32-bit)` |
| `jle` at 0x16d0ac | `? <= 0x14 (signed, 32-bit)` | `? > 0x14 (signed, 32-bit)` |
| `jle` at 0x16d0d6 | `? <= 0x8 (signed, 32-bit)` | `? > 0x8 (signed, 32-bit)` |
| `jle` at 0x16d102 | `? <= 0x4 (signed, 32-bit)` | `? > 0x4 (signed, 32-bit)` |
| `je` at 0x16d11b | `[0x1d06dc] == 0x0` | `[0x1d06dc] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16d12b | `eax` | `?` |

## `_kbdEs`

- `_kbdEs`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b228 to 0x10b2c8.
  Established. The byte load at an unknown effective address retains an unknown value (`mov` at
  0x10b267), as does the byte store at `sp -0x8` (`mov` at 0x10b26a). The `_tyIRd` sites have
  unknown candidate zero paired with `0x1b` (`call` at 0x10b28c) and `0x4f` (`call` at 0x10b297).
  The final site retains unknown candidates (`call` at 0x10b2b8). No key-encoding grammar is
  assigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b230 | load | 1 | `sp +0x4` | `-` |
| `mov` at 0x10b233 | load | 4 | `abs 0x1d2744` | `-` |
| `test` at 0x10b239 | load | 1 | `abs 0x1d2741` | `-` |
| `mov` at 0x10b242 | load | 4 | `abs 0x1d273c` | `-` |
| `movzx` at 0x10b24d | load | 2 | `abs 0x1d2742` | `-` |
| `mov` at 0x10b267 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10b26a | store | 1 | `sp -0x8` | `?` |
| `test` at 0x10b26d | load | 1 | `abs 0x1d2740` | `-` |
| `movsx` at 0x10b29f | load | 1 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b28c | `_tyIRd 0x16e970` | `?` | `0x1b` | `ebx` | `esi` | `?` | `ebp` |
| `call` at 0x10b297 | `_tyIRd 0x16e970` | `?` | `0x4f` | `?` | `?` | `?` | `?` |
| `call` at 0x10b2b8 | `_tyIRd 0x16e970` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10b240 | `(([0x1d2741]:1 & 0xff) & 0x2) != 0x0` | `(([0x1d2741]:1 & 0xff) & 0x2) == 0x0` |
| `jne` at 0x10b274 | `(([0x1d2740]:1 & 0xff) & 0x2) != 0x0` | `(([0x1d2740]:1 & 0xff) & 0x2) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b2c5 | `eax` | `?` |

## `_sysA20on`

- `_sysA20on`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108100 to 0x1081a0.
  Established. Byte I/O stores write `0xd1` to absolute `0x64` (`out` at 0x10810a), `0xdf` to `0x60`
  (`out` at 0x108116), `0xff` to `0x64` (`out` at 0x108122), and `0x2` to `0x92` (`out` at
  0x108152). Four-byte stores write `0x1234567` at absolute `0x0` (`mov` at 0x10813f, `mov` at
  0x108172). The later comparison branch is taken when `[0x100000] != 0x1234567` (`jne` at
  0x10817e). The body does not establish an address-line or reset outcome.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `out` at 0x10810a | store | 1 | `I/O abs 0x64` | `0xd1` |
| `out` at 0x108116 | store | 1 | `I/O abs 0x60` | `0xdf` |
| `out` at 0x108122 | store | 1 | `I/O abs 0x64` | `0xff` |
| `mov` at 0x108133 | store | 4 | `abs 0x0` | `0x0` |
| `mov` at 0x108139 | store | 4 | `abs 0x100000` | `0x0` |
| `mov` at 0x10813f | store | 4 | `abs 0x0` | `0x1234567` |
| `cmp` at 0x108145 | load | 4 | `abs 0x100000` | `-` |
| `out` at 0x108152 | store | 1 | `I/O abs 0x92` | `0x2` |
| `in` at 0x108156 | load | 1 | `I/O abs 0x92` | `-` |
| `mov` at 0x108166 | store | 4 | `abs 0x0` | `0x0` |
| `mov` at 0x10816c | store | 4 | `abs 0x100000` | `0x0` |
| `mov` at 0x108172 | store | 4 | `abs 0x0` | `0x1234567` |
| `cmp` at 0x108178 | load | 4 | `abs 0x100000` | `-` |
| `mov` at 0x108180 | store | 4 | `abs 0x108190` | `0xffffffff` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108100 | `_sysWait 0x1081a0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10810c | `_sysWait 0x1081a0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108118 | `_sysWait 0x1081a0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108124 | `_sysWait 0x1081a0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10814b | `[0x100000] != 0x1234567` | `[0x100000] == 0x1234567` |
| `loope` at 0x10815a | `oope flags condition (operands unknown)` | `not (oope flags condition (operands unknown))` |
| `jne` at 0x10817e | `[0x100000] != 0x1234567` | `[0x100000] == 0x1234567` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10818a | `eax` | `0x0` |

## `_sysReboot`

- `_sysReboot`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108230 to 0x108240.
  Established. The six-byte load reads absolute `0x0` (`lgdt` at 0x108235). The exit carries `0x0`
  in `eax` (`ret` at 0x108238). The load and literal return do not independently establish a reboot.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `lgdt` at 0x108235 | load | 6 | `abs 0x0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108238 | `eax` | `0x0` |

## `_sysInByte`

- `_sysInByte`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081b0 to 0x1081c0.
  Established. The four-byte load reads `sp +0x4` (`mov` at 0x1081b0). The byte I/O load has an
  unknown effective address (`in` at 0x1081b9). The exit value also remains unknown (`ret` at
  0x1081bc). The port is not filled in from the raw register operands.

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

## `_sysOutByte`

- `_sysOutByte`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081d0 to 0x1081e0.
  Established. The four-byte loads read `sp +0x4` (`mov` at 0x1081d0) and `sp +0x8` (`mov` at
  0x1081d4). The byte I/O store has an unknown effective address and value (`out` at 0x1081d8). The
  exit reports `[sp+8]` (`ret` at 0x1081db). The return expression does not resolve the emitted
  byte.

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

## `_sysInWord`

- `_sysInWord`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081c0 to 0x1081d0.
  Established. The four-byte load reads `sp +0x4` (`mov` at 0x1081c0). The two-byte I/O load has an
  unknown effective address (`in` at 0x1081c9). The exit retains an unknown value (`ret` at
  0x1081cd). No port location or reply value is assigned.

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

## `_sysOutWord`

- `_sysOutWord`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081e0 to 0x1081f0.
  Established. The four-byte loads read `sp +0x4` (`mov` at 0x1081e0) and `sp +0x8` (`mov` at
  0x1081e4). The two-byte I/O store retains unknown target and value (`out` at 0x1081e8). The exit
  carries `[sp+8]` (`ret` at 0x1081ec). No hardware-window role is inferred from the name.

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

## `_sysIntDisable`

- `_sysIntDisable`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10bb08 to 0x10bb18.
  Established. The immediate assignment places `0xffffffff` in `eax` (`mov` at 0x10bb0b), and the
  exit retains that literal (`ret` at 0x10bb13). The explicit access and call tables are empty. No
  interrupt-disabling behavior is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10bb13 | `eax` | `0xffffffff` |

## `_sysIntEnable`

- `_sysIntEnable`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10bb18 to 0x10bb28.
  Established. The assignment writes literal `0xffffffff` to `eax` (`mov` at 0x10bb1b). The exit
  reports that value (`ret` at 0x10bb23), with no explicit access or call rows in this body.
  Interrupt-enabling semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10bb23 | `eax` | `0xffffffff` |

## `_sysClkRateGet`

- `_sysClkRateGet`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1085a8 to 0x1085b8.
  Established. The four-byte load reads absolute `0x1a9c3c` (`mov` at 0x1085ab), and `[0x1a9c3c]`
  reaches `eax` at the exit (`ret` at 0x1085b3). A clock frequency or timing unit is not established
  by this load.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1085ab | load | 4 | `abs 0x1a9c3c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1085b3 | `eax` | `[0x1a9c3c]` |
