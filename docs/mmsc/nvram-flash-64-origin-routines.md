# MMSC NVRAM and flash paths in the 6.4-origin image

Read from `6.4-origin/usr/cpu/firmware/mmscfw.bin`.
Image SHA-256: `b766e559f86078526916d8bd98bbb8079294ad774cce9177928873153c6fb09b`.
The x86 a.out link address is `0x108000`. The [load map](load-map.md) explains segment mapping.
Analysed spans have exclusive ends inferred from the next text symbol.
Some include literals and padding decoded linearly after returns. Only sites reached
by known control-flow edges contribute effects to the tables.

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

[Fact sheets and complete listings](disasm/nvram-flash-64-origin-facts.txt) include all tool sections.

## `_nvramInit`

- `_nvramInit`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1268d0 to 0x126bc0.
  Established. The `_sram_read` site presents leading candidates `0x1da928`, `0x40`, and `0x0`
  (`call` at 0x1269b9); its result branch falls through when `ret(_sram_read) == 0xfffffffd` (`jne`
  at 0x1269c6), reaching `_nvramReset` with unknown candidates (`call` at 0x1269c8). Later four-byte
  stores include `0x0` at absolute `0x1da928` (`mov` at 0x126a78), `[0x1d9a1c]` at absolute
  `0x1da934` (`mov` at 0x126b05), and `0xd` at absolute `0x1da930` (`mov` at 0x126b0a). Field
  meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1268f8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1268fe | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x126927 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x126951 | load | 4 | `ret(___stderr) +0x0` | `-` |
| `mov` at 0x126976 | load | 4 | `ret(___stderr) +0x0` | `-` |
| `mov` at 0x126990 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x126996 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x1269aa | store | 4 | `abs 0x1d9a1c` | `?` |
| `mov` at 0x1269e5 | load | 4 | `ret(___stderr) +0x0` | `-` |
| `cmp` at 0x126a00 | load | 4 | `abs 0x1da928` | `-` |
| `cmp` at 0x126a19 | load | 4 | `abs 0x1da92c` | `-` |
| `mov` at 0x126a22 | load | 4 | `abs 0x1da92c` | `-` |
| `mov` at 0x126a60 | load | 4 | `abs 0x1da92c` | `-` |
| `mov` at 0x126a70 | load | 4 | `ret(___stderr) +0x0` | `-` |
| `mov` at 0x126a78 | store | 4 | `abs 0x1da928` | `0x0` |
| `cmp` at 0x126a90 | load | 4 | `abs 0x1da930` | `-` |
| `mov` at 0x126a99 | load | 4 | `abs 0x1da934` | `-` |
| `cmp` at 0x126a9e | load | 4 | `abs 0x1d9a1c` | `-` |
| `mov` at 0x126ab0 | load | 4 | `ret(___stderr) +0x0` | `-` |
| `cmp` at 0x126ac0 | load | 4 | `abs 0x1da930` | `-` |
| `cmp` at 0x126ad4 | load | 4 | `abs 0x1da930` | `-` |
| `add` at 0x126ae4 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x126ae9 | load | 4 | `abs 0x1da930` | `-` |
| `cmp` at 0x126af1 | load | 4 | `abs 0x1da934` | `-` |
| `mov` at 0x126b00 | load | 4 | `abs 0x1d9a1c` | `-` |
| `mov` at 0x126b05 | store | 4 | `abs 0x1da934` | `[0x1d9a1c]` |
| `mov` at 0x126b0a | store | 4 | `abs 0x1da930` | `0xd` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1268dd | `_bzero 0x170f20` | `0x1da928` | `0x40` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x1268ec | `_bzero 0x170f20` | `0x1d9fc0` | `0x68` | `?` | `?` | `?` | `?` |
| `call` at 0x12694c | `___stderr 0x18bdcc` | `0x1267d1` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x126954 | `_fprintf 0x17a13c` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126971 | `___stderr 0x18bdcc` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126979 | `_fprintf 0x17a13c` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1269b9 | `_sram_read 0x10c928` | `0x1da928` | `0x40` | `0x0` | `?` | `?` | `?` |
| `call` at 0x1269c8 | `_nvramReset 0x126d90` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1269e0 | `___stderr 0x18bdcc` | `0x126824` | `ret(_sram_read)` | `?` | `?` | `?` | `?` |
| `call` at 0x1269e8 | `_fprintf 0x17a13c` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126a0c | `_nvramReset 0x126d90` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126a40 | `_nvramUpgrade1to2 0x126f90` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126a49 | `_nvramWriteHeader 0x127080` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126a6b | `___stderr 0x18bdcc` | `0x126834` | `[0x1da92c]` | `?` | `?` | `?` | `?` |
| `call` at 0x126a73 | `_fprintf 0x17a13c` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126aab | `___stderr 0x18bdcc` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126ab3 | `_fprintf 0x17a13c` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126b14 | `_nvramWriteHeader 0x127080` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x126907 | `be flags condition (operands unknown)` | `not (be flags condition (operands unknown))` |
| `je` at 0x12692e | `? == 0x0` | `? != 0x0` |
| `jne` at 0x126942 | `Z == 0` | `not (Z == 0)` |
| `jbe` at 0x126969 | `be flags condition (operands unknown)` | `not (be flags condition (operands unknown))` |
| `jbe` at 0x1269a4 | `be flags condition (operands unknown)` | `not (be flags condition (operands unknown))` |
| `jne` at 0x1269c6 | `ret(_sram_read) != 0xfffffffd` | `ret(_sram_read) == 0xfffffffd` |
| `jne` at 0x1269cf | `(ret(_nvramReset) & ret(_nvramReset)) != 0x0` | `(ret(_nvramReset) & ret(_nvramReset)) == 0x0` |
| `je` at 0x1269d8 | `ret(_sram_read) == 0x1` | `ret(_sram_read) != 0x1` |
| `je` at 0x126a0a | `[0x1da928] == 0xff5cff5c` | `[0x1da928] != 0xff5cff5c` |
| `jne` at 0x126a13 | `(ret(_nvramReset) & ret(_nvramReset)) != 0x0` | `(ret(_nvramReset) & ret(_nvramReset)) == 0x0` |
| `je` at 0x126a20 | `[0x1da92c] == 0x2` | `[0x1da92c] != 0x2` |
| `je` at 0x126a2a | `[0x1da92c] == 0x1` | `[0x1da92c] != 0x1` |
| `je` at 0x126a2f | `[0x1da92c] == 0x2` | `[0x1da92c] != 0x2` |
| `jne` at 0x126a47 | `(ret(_nvramUpgrade1to2) & ret(_nvramUpgrade1to2)) != 0x0` | `(ret(_nvramUpgrade1to2) & ret(_nvramUpgrade1to2)) == 0x0` |
| `je` at 0x126a50 | `(ret(_nvramWriteHeader) & ret(_nvramWriteHeader)) == 0x0` | `(ret(_nvramWriteHeader) & ret(_nvramWriteHeader)) != 0x0` |
| `jne` at 0x126a97 | `[0x1da930] != 0xd` | `[0x1da930] == 0xd` |
| `je` at 0x126aa4 | `[0x1d9a1c] == [0x1da934]` | `[0x1d9a1c] != [0x1da934]` |
| `jbe` at 0x126ac7 | `[0x1da930] <= 0xd (unsigned, 32-bit)` | `[0x1da930] > 0xd (unsigned, 32-bit)` |
| `jbe` at 0x126ada | `[0x1da930] <= 0x0 (unsigned, 32-bit)` | `[0x1da930] > 0x0 (unsigned, 32-bit)` |
| `ja` at 0x126aef | `[0x1da930] > ? (unsigned, 32-bit)` | `[0x1da930] <= ? (unsigned, 32-bit)` |
| `je` at 0x126af7 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x126b1b | `(ret(_nvramWriteHeader) & ret(_nvramWriteHeader)) != 0x0` | `(ret(_nvramWriteHeader) & ret(_nvramWriteHeader)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x126b3e | `eax` | `?` |

## `_nvramRead`

- `_nvramRead`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x126cf0 to 0x126d90.
  Established. The initial range branch is taken when `[sp+4] > 0xc (unsigned, 32-bit)` (`ja` at
  0x126cff). The later `_nvramIsValid` call presents candidate zero `[sp+4]` (`call` at 0x126d41).
  Falling through its result test when `(ret(_nvramIsValid) & ret(_nvramIsValid)) != 0x0` (`je` at
  0x126d4b) reaches `_sram_read` with candidate zero `[sp+8]` and unknown next candidates (`call` at
  0x126d5a). No transfer size is inferred.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x126cf6 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x126cf9 | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x126d0d | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x126d20 | load | 4 | `abs 0x1da928` | `-` |
| `mov` at 0x126d4d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x126d54 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x126d31 | `_ffscMsg 0x11ebd0` | `0x126c95` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x126d41 | `_nvramIsValid 0x126f60` | `[sp+4]` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x126d5a | `_sram_read 0x10c928` | `[sp+8]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126d76 | `_ffscMsg 0x11ebd0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x126cff | `[sp+4] > 0xc (unsigned, 32-bit)` | `[sp+4] <= 0xc (unsigned, 32-bit)` |
| `jne` at 0x126d12 | `? != 0x0` | `? == 0x0` |
| `je` at 0x126d2a | `[0x1da928] == 0xff5cff5c` | `[0x1da928] != 0xff5cff5c` |
| `je` at 0x126d4b | `(ret(_nvramIsValid) & ret(_nvramIsValid)) == 0x0` | `(ret(_nvramIsValid) & ret(_nvramIsValid)) != 0x0` |
| `jne` at 0x126d65 | `ret(_sram_read) != 0x1` | `ret(_sram_read) == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x126d89 | `eax` | `?` |

## `_nvramWrite`

- `_nvramWrite`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x126e80 to 0x126f40.
  Established. The `_sram_write` site presents candidate zero `[sp+8]` with unknown candidates one
  and two (`call` at 0x126edd); its result branch is taken when `ret(_sram_write) == 0x1` (`je` at
  0x126ee8), reaching `_nvramIsValid` with candidate zero `[sp+4]` (`call` at 0x126f01). The
  subsequent result-test fall-through is `(ret(_nvramIsValid) & ret(_nvramIsValid)) == 0x0` (`jne`
  at 0x126f0b), reaching `_nvramValidate` with candidate zero `[sp+4]` (`call` at 0x126f0e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x126e85 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x126e88 | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x126e9c | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x126eb0 | load | 4 | `abs 0x1da928` | `-` |
| `mov` at 0x126ed0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x126ed7 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x126ec1 | `_ffscMsg 0x11ebd0` | `0x126e21` | `ebx` | `esi` | `ebp` | `?` | `?` |
| `call` at 0x126edd | `_sram_write 0x10ca08` | `[sp+8]` | `?` | `?` | `ebx` | `esi` | `ebp` |
| `call` at 0x126ef0 | `_ffscMsg 0x11ebd0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126f01 | `_nvramIsValid 0x126f60` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126f0e | `_nvramValidate 0x127020` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126f16 | `_nvramWriteHeader 0x127080` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x126e8e | `[sp+4] > 0xc (unsigned, 32-bit)` | `[sp+4] <= 0xc (unsigned, 32-bit)` |
| `jne` at 0x126ea1 | `? != 0x0` | `? == 0x0` |
| `je` at 0x126eba | `[0x1da928] == 0xff5cff5c` | `[0x1da928] != 0xff5cff5c` |
| `je` at 0x126ee8 | `ret(_sram_write) == 0x1` | `ret(_sram_write) != 0x1` |
| `jne` at 0x126f0b | `(ret(_nvramIsValid) & ret(_nvramIsValid)) != 0x0` | `(ret(_nvramIsValid) & ret(_nvramIsValid)) == 0x0` |
| `je` at 0x126f1d | `(ret(_nvramWriteHeader) & ret(_nvramWriteHeader)) == 0x0` | `(ret(_nvramWriteHeader) & ret(_nvramWriteHeader)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x126f3a | `eax` | `?` |

## `_nvramReset`

- `_nvramReset`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x126d90 to 0x126e80.
  Established. Four-byte stores write `0xff5cff5c` to absolute `0x1da928` (`mov` at 0x126da2), `0x2`
  to absolute `0x1da92c` (`mov` at 0x126dac), `0xd` to absolute `0x1da930` (`mov` at 0x126db6), and
  `[0x1d9a1c]` to absolute `0x1da934` (`mov` at 0x126dc5). The `_nvramValidate` site presents
  candidate zero `0x0` (`call` at 0x126dcc). The later branch is taken when `(ret(_nvramWriteHeader)
  & ret(_nvramWriteHeader)) != 0x0` (`jne` at 0x126ddb), reaching the `0xffffffff` exit (`ret` at
  0x126df8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x126da2 | store | 4 | `abs 0x1da928` | `0xff5cff5c` |
| `mov` at 0x126dac | store | 4 | `abs 0x1da92c` | `0x2` |
| `mov` at 0x126db6 | store | 4 | `abs 0x1da930` | `0xd` |
| `mov` at 0x126dc0 | load | 4 | `abs 0x1d9a1c` | `-` |
| `mov` at 0x126dc5 | store | 4 | `abs 0x1da934` | `[0x1d9a1c]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x126d9a | `_bzero 0x170f20` | `0x1da928` | `0x40` | `ebp` | `?` | `?` | `?` |
| `call` at 0x126dcc | `_nvramValidate 0x127020` | `0x0` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126dd4 | `_nvramWriteHeader 0x127080` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x126ddb | `(ret(_nvramWriteHeader) & ret(_nvramWriteHeader)) != 0x0` | `(ret(_nvramWriteHeader) & ret(_nvramWriteHeader)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x126de2 | `eax` | `0x0` |
| `ret` at 0x126df8 | `eax` | `0xffffffff` |

## `_nvramZap`

- `_nvramZap`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x126f40 to 0x126f60.
  Established. The `_bzero` site presents leading candidates `0x1da928` and `0x40` (`call` at
  0x126f4a). The following `_nvramWriteHeader` site retains unknown candidates (`call` at 0x126f52);
  its call-result expression `ret(_nvramWriteHeader)` reaches `eax` at the exit (`ret` at 0x126f5a).
  The routine role is not inferred from the name.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x126f4a | `_bzero 0x170f20` | `0x1da928` | `0x40` | `ebp` | `?` | `?` | `?` |
| `call` at 0x126f52 | `_nvramWriteHeader 0x127080` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x126f5a | `eax` | `ret(_nvramWriteHeader)` |

## `_nvramIsValid`

- `_nvramIsValid`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x126f60 to 0x126f90.
  Established. A four-byte load reads `sp +0x4` (`mov` at 0x126f63). The later indexed four-byte
  load retains an unknown effective address (`mov` at 0x126f6e); its exit value in `eax` remains
  unknown (`ret` at 0x126f83). No bitmap location or tested-bit meaning is established by this
  entry.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x126f63 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x126f6e | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x126f83 | `eax` | `?` |

## `_nvramUpgrade1to2`

- `_nvramUpgrade1to2`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x126f90 to 0x127020.
  Established. The first result branch falls through when `(ret(_nvramRead) & ret(_nvramRead)) ==
  0x0` (`jne` at 0x126fa7). The next branch also falls through when `(([sp-32]:1 & 0xff) & 0x1) !=
  0x0` (`je` at 0x126fad), reaching the byte store of `0xa0` at `sp -0x28` (`mov` at 0x126faf); its
  following `_nvramWrite` site presents leading candidates `0x6` and `sp-52` (`call` at 0x126fb6). A
  later four-byte store writes `0x2` to absolute `0x1da92c` (`mov` at 0x127000).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `test` at 0x126fa9 | load | 1 | `sp -0x20` | `-` |
| `mov` at 0x126faf | store | 1 | `sp -0x28` | `0xa0` |
| `test` at 0x126fd4 | load | 1 | `sp -0x20` | `-` |
| `mov` at 0x126fda | store | 1 | `sp -0x28` | `0xa0` |
| `mov` at 0x127000 | store | 4 | `abs 0x1da92c` | `0x2` |
| `mov` at 0x12700f | load | 4 | `sp -0x38` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x126f9d | `_nvramRead 0x126cf0` | `0x6` | `sp-52` | `ebx` | `?` | `?` | `?` |
| `call` at 0x126fb6 | `_nvramWrite 0x126e80` | `0x6` | `sp-52` | `?` | `?` | `?` | `?` |
| `call` at 0x126fc8 | `_nvramRead 0x126cf0` | `0x7` | `sp-52` | `?` | `?` | `?` | `?` |
| `call` at 0x126fe1 | `_nvramWrite 0x126e80` | `0x7` | `sp-52` | `?` | `?` | `?` | `?` |
| `call` at 0x12700a | `_nvramWriteHeader 0x127080` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x126fa7 | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` |
| `je` at 0x126fad | `(([sp-32]:1 & 0xff) & 0x1) == 0x0` | `(([sp-32]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x126fc0 | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` |
| `jne` at 0x126fd2 | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` |
| `je` at 0x126fd8 | `(([sp-32]:1 & 0xff) & 0x1) == 0x0` | `(([sp-32]:1 & 0xff) & 0x1) != 0x0` |
| `je` at 0x126feb | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x127015 | `eax` | `?` |

## `_nvramValidate`

- `_nvramValidate`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x127020 to 0x127080.
  Established. The initial four-byte load uses `sp +0x4` (`mov` at 0x127027). The indexed four-byte
  load (`mov` at 0x12703b) and store (`mov` at 0x127044) retain unknown effective addresses, with
  unknown stored value. A later four-byte load reads `sp -0xc` (`mov` at 0x12704b). The return
  expression remains unknown (`ret` at 0x127051). No validation effect is assigned from the name.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x127027 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12703b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x127044 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x12704b | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x127051 | `eax` | `?` |

## `_nvramWriteHeader`

- `_nvramWriteHeader`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x127080 to 0x1270b4.
  Established. The `_sram_write` call presents leading candidates `0x1da928`, `0x40`, and `0x0`
  (`call` at 0x12708c). The comparison branch falls through when `ret(_sram_write) == 0x1` (`jne` at
  0x127097), reaching the `0x0` exit (`ret` at 0x12709e). Taking that branch instead reaches
  `_ffscMsg` with leading candidates `0x127052` and `ret(_sram_write)` (`call` at 0x1270a6), then
  the `0xffffffff` exit (`ret` at 0x1270b3). No hardware reply meaning is inferred.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12708c | `_sram_write 0x10ca08` | `0x1da928` | `0x40` | `0x0` | `ebp` | `?` | `?` |
| `call` at 0x1270a6 | `_ffscMsg 0x11ebd0` | `0x127052` | `ret(_sram_write)` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x127097 | `ret(_sram_write) != 0x1` | `ret(_sram_write) == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12709e | `eax` | `0x0` |
| `ret` at 0x1270b3 | `eax` | `0xffffffff` |

## `_initFlash`

- `_initFlash`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10cba4 to 0x10cc14.
  Established. Four-byte stores include `0x760` through `[sp+4] +0x18` (`mov` at 0x10cbd3), `0x540`
  through `[sp+4] +0x1c` (`mov` at 0x10cbda), `0x548` through `[sp+4] +0x20` (`mov` at 0x10cbe1),
  and `0xa0000` through `[sp+4] +0x24` (`mov` at 0x10cbe8). The other stores remain listed below.
  The exit reports `[sp+4]` in `eax` (`ret` at 0x10cc07). No field roles or allocation size are
  assigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10cba7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10cbaa | store | 4 | `[sp+4] +0x0` | `0x550` |
| `mov` at 0x10cbb0 | store | 4 | `[sp+4] +0x4` | `0x0` |
| `mov` at 0x10cbb7 | store | 4 | `[sp+4] +0x8` | `0x2` |
| `mov` at 0x10cbbe | store | 4 | `[sp+4] +0xc` | `0x0` |
| `mov` at 0x10cbc5 | store | 4 | `[sp+4] +0x10` | `0x8000` |
| `mov` at 0x10cbcc | store | 4 | `[sp+4] +0x14` | `0x1` |
| `mov` at 0x10cbd3 | store | 4 | `[sp+4] +0x18` | `0x760` |
| `mov` at 0x10cbda | store | 4 | `[sp+4] +0x1c` | `0x540` |
| `mov` at 0x10cbe1 | store | 4 | `[sp+4] +0x20` | `0x548` |
| `mov` at 0x10cbe8 | store | 4 | `[sp+4] +0x24` | `0xa0000` |
| `mov` at 0x10cbef | store | 4 | `[sp+4] +0x28` | `0x800` |
| `mov` at 0x10cbf6 | store | 4 | `[sp+4] +0x2c` | `0x1aa4f4` |
| `mov` at 0x10cbfd | store | 4 | `[sp+4] +0x30` | `0x1aa534` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10cc07 | `eax` | `[sp+4]` |

## `_openFlashWindow`

- `_openFlashWindow`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10cc14 to 0x10cc94.
  Established. The first `_sysOutByte` site presents leading candidates `[[sp+4]+32]` and
  `0xffffffff` (`call` at 0x10cc4b). The second presents `[[sp+4]+24]` and unknown candidate one
  (`call` at 0x10cc6f). The intervening branch falls through when `[sp+12] == 0x40` (`jne` at
  0x10cc5d), reaching a four-byte load at `[sp+4] +0x30` (`mov` at 0x10cc5f). The `_nanosleep` site
  presents leading candidates `0x1aa4e4` and `0x0` (`call` at 0x10cc7e), without establishing a
  duration.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10cc1a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10cc1d | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10cc20 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10cc26 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10cc3f | load | 4 | `[sp+4] +0x24` | `-` |
| `mov` at 0x10cc47 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x10cc53 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10cc56 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x10cc59 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10cc5f | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x10cc62 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10cc6b | load | 4 | `[sp+4] +0x18` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10cc4b | `_sysOutByte 0x1081d0` | `[[sp+4]+32]` | `0xffffffff` | `?` | `?` | `?` | `?` |
| `call` at 0x10cc6f | `_sysOutByte 0x1081d0` | `[[sp+4]+24]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10cc7e | `_nanosleep 0x179570` | `0x1aa4e4` | `0x0` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x10cc2e | `? <= ? (unsigned, 32-bit)` | `? > ? (unsigned, 32-bit)` |
| `jne` at 0x10cc5d | `[sp+12] != 0x40` | `[sp+12] == 0x40` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10cc8e | `eax` | `?` |

## `_closeFlashWindow`

- `_closeFlashWindow`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10cc94 to 0x10cd84.
  Established. The first two `_sysOutByte` sites present candidate zero `[[sp+4]+24]` and unknown
  candidate one (`call` at 0x10cca8, `call` at 0x10ccbe). The third presents leading candidates
  `[[sp+4]+28]` and `0xffffffff` (`call` at 0x10cccc). The `_nanosleep` call presents `0x1aa4e4` and
  `0x0` as its leading candidates (`call` at 0x10ccdb). The exit retains `ret(_nanosleep)` (`ret` at
  0x10cce6), without a proven restoration role.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10cc98 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10cc9b | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10cc9e | load | 4 | `[[sp+4]+44] +0x0` | `-` |
| `mov` at 0x10cca4 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10ccb0 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10ccb3 | load | 4 | `[[sp+4]+44] +0x20` | `-` |
| `mov` at 0x10ccba | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10ccc8 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x10cce0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10cca8 | `_sysOutByte 0x1081d0` | `[[sp+4]+24]` | `?` | `ebx` | `ebp` | `?` | `?` |
| `call` at 0x10ccbe | `_sysOutByte 0x1081d0` | `[[sp+4]+24]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10cccc | `_sysOutByte 0x1081d0` | `[[sp+4]+28]` | `0xffffffff` | `?` | `?` | `?` | `?` |
| `call` at 0x10ccdb | `_nanosleep 0x179570` | `0x1aa4e4` | `0x0` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10cce6 | `eax` | `ret(_nanosleep)` |

## `_readFlash`

- `_readFlash`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10d0e4 to 0x10d224.
  Established. Three `_openFlashWindow` sites include leading candidates `[sp+4]`, unknown, and
  `0xff` (`call` at 0x10d12b, `call` at 0x10d188, `call` at 0x10d1d3). Taking the first result
  branch when `(ret(_openFlashWindow) & ret(_openFlashWindow)) == 0x0` (`je` at 0x10d137) reaches a
  four-byte store of `0xfffffff7` at `sp -0xc` (`mov` at 0x10d204). The final `_closeFlashWindow`
  site presents candidate zero `[sp+4]` (`call` at 0x10d20f). The exit reports `[sp-12]` in `eax`
  (`ret` at 0x10d220).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10d0ed | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10d0f0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d0f3 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10d0f6 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10d0f9 | store | 4 | `sp -0x8` | `[sp+8]` |
| `mov` at 0x10d0fc | store | 4 | `sp -0xc` | `0x0` |
| `mov` at 0x10d103 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d10e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d111 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d11a | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x10d127 | load | 4 | `sp +0x4` | `-` |
| `add` at 0x10d13d | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x10d149 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10d156 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d159 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d15e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d161 | store | 4 | `[sp+4] +0xc` | `?` |
| `add` at 0x10d166 | load | 4 | `sp -0x8` | `-` |
| `add` at 0x10d166 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x10d170 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d173 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d184 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d193 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10d1a0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d1a3 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d1a8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d1ab | store | 4 | `[sp+4] +0xc` | `([[sp+4]+12] + [[sp+4]+16])` |
| `add` at 0x10d1b0 | load | 4 | `sp -0x8` | `-` |
| `add` at 0x10d1b0 | store | 4 | `sp -0x8` | `([sp-8] + [[sp+4]+16])` |
| `mov` at 0x10d1bb | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d1be | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d1cf | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d1de | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10d1eb | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d1ee | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d1f3 | store | 4 | `sp -0x14` | `?` |
| `mov` at 0x10d1f6 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d1f9 | store | 4 | `[sp+4] +0xc` | `?` |
| `mov` at 0x10d204 | store | 4 | `sp -0xc` | `0xfffffff7` |
| `mov` at 0x10d20b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d214 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10d12b | `_openFlashWindow 0x10cc14` | `[sp+4]` | `?` | `0xff` | `?` | `?` | `?` |
| `call` at 0x10d14e | `_bcopy 0x1747fc` | `(ret(_openFlashWindow) + [sp-16])` | `[sp-8]` | `?` | `?` | `?` | `?` |
| `call` at 0x10d188 | `_openFlashWindow 0x10cc14` | `[sp+4]` | `?` | `0xff` | `?` | `?` | `?` |
| `call` at 0x10d198 | `_bcopy 0x1747fc` | `ret(_openFlashWindow)` | `[sp-8]` | `[[sp+4]+16]` | `?` | `?` | `?` |
| `call` at 0x10d1d3 | `_openFlashWindow 0x10cc14` | `[sp+4]` | `?` | `0xff` | `?` | `?` | `?` |
| `call` at 0x10d1e3 | `_bcopy 0x1747fc` | `?` | `[sp-8]` | `?` | `?` | `?` | `?` |
| `call` at 0x10d20f | `_closeFlashWindow 0x10cc94` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10d137 | `(ret(_openFlashWindow) & ret(_openFlashWindow)) == 0x0` | `(ret(_openFlashWindow) & ret(_openFlashWindow)) != 0x0` |
| `jbe` at 0x10d142 | `[sp+12] <= ? (unsigned, 32-bit)` | `[sp+12] > ? (unsigned, 32-bit)` |
| `jb` at 0x10d16b | `? < [[sp+4]+16] (unsigned, 32-bit)` | `? >= [[sp+4]+16] (unsigned, 32-bit)` |
| `jae` at 0x10d1b5 | `? >= [[sp+4]+16] (unsigned, 32-bit)` | `? < [[sp+4]+16] (unsigned, 32-bit)` |
| `je` at 0x10d1b9 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10d220 | `eax` | `[sp-12]` |

## `_writeFlashBytes`

- `_writeFlashBytes`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10d224 to 0x10d314.
  Established. Byte stores through `[sp+4] +0x0` include `0x50` (`mov` at 0x10d24c, `mov` at
  0x10d254, `mov` at 0x10d274, `mov` at 0x10d288), `0x70` (`mov` at 0x10d257, `mov` at 0x10d28b),
  and `0xff` (`mov` at 0x10d2c4). A separate byte store writes `0x40` at an unknown effective
  address (`mov` at 0x10d277). Another stores `[[sp+8]]:1` at an unknown effective address (`mov` at
  0x10d27f). The first inner repeat branch is taken when `? != 0xffffffff` (`jne` at 0x10d264), with
  its tested operand unknown. Command meanings and loop counts remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10d22d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d232 | store | 4 | `sp -0xc` | `0x64` |
| `mov` at 0x10d23b | store | 4 | `sp -0x8` | `0x0` |
| `cmp` at 0x10d242 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10d24c | store | 1 | `[sp+4] +0x0` | `0x50` |
| `mov` at 0x10d254 | store | 1 | `[sp+4] +0x0` | `0x50` |
| `mov` at 0x10d257 | store | 1 | `[sp+4] +0x0` | `0x70` |
| `mov` at 0x10d25a | load | 1 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10d274 | store | 1 | `[sp+4] +0x0` | `0x50` |
| `mov` at 0x10d277 | store | 1 | `(unknown)` | `0x40` |
| `mov` at 0x10d27a | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10d27d | load | 1 | `[sp+8] +0x0` | `-` |
| `mov` at 0x10d27f | store | 1 | `(unknown)` | `[[sp+8]]:1` |
| `mov` at 0x10d288 | store | 1 | `[sp+4] +0x0` | `0x50` |
| `mov` at 0x10d28b | store | 1 | `[sp+4] +0x0` | `0x70` |
| `mov` at 0x10d28e | load | 1 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10d2c4 | store | 1 | `[sp+4] +0x0` | `0xff` |
| `mov` at 0x10d2c7 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10d2c9 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10d2cc | load | 1 | `[sp+8] +0x0` | `-` |
| `dec` at 0x10d2d7 | load | 4 | `sp -0xc` | `-` |
| `dec` at 0x10d2d7 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x10d2e4 | store | 4 | `sp -0xc` | `0x2710` |
| `inc` at 0x10d2eb | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x10d2eb | store | 4 | `sp -0x8` | `?` |
| `inc` at 0x10d2ef | load | 4 | `sp +0x8` | `-` |
| `inc` at 0x10d2ef | store | 4 | `sp +0x8` | `?` |
| `mov` at 0x10d2f2 | load | 4 | `sp +0xc` | `-` |
| `cmp` at 0x10d2f5 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x10d245 | `[sp+12] <= 0x0 (unsigned, 32-bit)` | `[sp+12] > 0x0 (unsigned, 32-bit)` |
| `jl` at 0x10d25e | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jne` at 0x10d264 | `? != 0xffffffff` | `? == 0xffffffff` |
| `jne` at 0x10d268 | `Z == 0` | `not (Z == 0)` |
| `jl` at 0x10d292 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jne` at 0x10d298 | `? != 0xffffffff` | `? == 0xffffffff` |
| `jne` at 0x10d29c | `Z == 0` | `not (Z == 0)` |
| `je` at 0x10d2b7 | `(([[sp+4]]:1 & 0xff) & 0x10) == 0x0` | `(([[sp+4]]:1 & 0xff) & 0x10) != 0x0` |
| `je` at 0x10d2d0 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x10d2da | `Z == 1` | `not (Z == 1)` |
| `jb` at 0x10d2f8 | `[sp-8] < [sp+12] (unsigned, 32-bit)` | `[sp-8] >= [sp+12] (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10d309 | `eax` | `?` |

## `_writeFlash`

- `_writeFlash`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10d314 to 0x10d504.
  Established. The `_openFlashWindow` sites present leading candidates `[sp+4]`, unknown, and `0x40`
  (`call` at 0x10d352, `call` at 0x10d3e9, `call` at 0x10d433). The first two `_writeFlashBytes`
  sites have leading candidates `(ret(_openFlashWindow) + [sp-12])`, `[sp+8]`, and `[sp+12]` (`call`
  at 0x10d36f, `call` at 0x10d3a7). The four-byte store through `[sp+4] +0xc` writes `([sp+12] +
  [[sp+4]+12])` (`mov` at 0x10d38f). The final `_closeFlashWindow` site has candidate zero `[sp+4]`
  (`call` at 0x10d46d). The return value remains unknown (`ret` at 0x10d483).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10d31d | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10d320 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d323 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10d326 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10d329 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d334 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d337 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d33a | store | 4 | `sp -0x18` | `[[sp+4]+12]` |
| `mov` at 0x10d341 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x10d348 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x10d34e | load | 4 | `sp +0x4` | `-` |
| `add` at 0x10d364 | load | 4 | `sp -0xc` | `-` |
| `cmp` at 0x10d367 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10d381 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d384 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d389 | store | 4 | `sp -0x14` | `([sp+12] + [[sp+4]+12])` |
| `mov` at 0x10d38c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d38f | store | 4 | `[sp+4] +0xc` | `([sp+12] + [[sp+4]+12])` |
| `mov` at 0x10d3b9 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d3bc | load | 4 | `[sp+4] +0xc` | `-` |
| `add` at 0x10d3bf | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10d3c2 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d3c5 | store | 4 | `[sp+4] +0xc` | `([[sp+4]+12] + [sp-8])` |
| `sub` at 0x10d3c8 | load | 4 | `sp -0x8` | `-` |
| `add` at 0x10d3cb | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10d3d4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d3d7 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d3e5 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d404 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d407 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d40c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d40f | store | 4 | `[sp+4] +0xc` | `([[sp+4]+12] + [[sp+4]+16])` |
| `mov` at 0x10d41e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d421 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d42f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d44e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d451 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d456 | store | 4 | `sp -0x14` | `?` |
| `mov` at 0x10d459 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d45c | store | 4 | `[sp+4] +0xc` | `?` |
| `mov` at 0x10d466 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d46a | store | 4 | `sp -0x1c` | `?` |
| `mov` at 0x10d472 | load | 4 | `sp -0x1c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10d352 | `_openFlashWindow 0x10cc14` | `[sp+4]` | `?` | `0x40` | `?` | `?` | `?` |
| `call` at 0x10d36f | `_writeFlashBytes 0x10d224` | `(ret(_openFlashWindow) + [sp-12])` | `[sp+8]` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x10d3a7 | `_writeFlashBytes 0x10d224` | `(ret(_openFlashWindow) + [sp-12])` | `[sp+8]` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x10d3e9 | `_openFlashWindow 0x10cc14` | `[sp+4]` | `?` | `0x40` | `?` | `?` | `?` |
| `call` at 0x10d3f6 | `_writeFlashBytes 0x10d224` | `ret(_openFlashWindow)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d433 | `_openFlashWindow 0x10cc14` | `[sp+4]` | `?` | `0x40` | `?` | `?` | `?` |
| `call` at 0x10d440 | `_writeFlashBytes 0x10d224` | `ret(_openFlashWindow)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d46d | `_closeFlashWindow 0x10cc94` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10d35e | `(ret(_openFlashWindow) & ret(_openFlashWindow)) == 0x0` | `(ret(_openFlashWindow) & ret(_openFlashWindow)) != 0x0` |
| `jb` at 0x10d36a | `[sp-8] < [sp+12] (unsigned, 32-bit)` | `[sp-8] >= [sp+12] (unsigned, 32-bit)` |
| `jne` at 0x10d37b | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `jne` at 0x10d3b3 | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `jb` at 0x10d3d0 | `? < [[sp+4]+16] (unsigned, 32-bit)` | `? >= [[sp+4]+16] (unsigned, 32-bit)` |
| `jne` at 0x10d402 | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `jae` at 0x10d418 | `? >= [[sp+4]+16] (unsigned, 32-bit)` | `? < [[sp+4]+16] (unsigned, 32-bit)` |
| `je` at 0x10d41c | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `jne` at 0x10d44c | `(? & ?) != 0x0` | `(? & ?) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10d483 | `eax` | `?` |
