# MMSC NVRAM and SRAM routines in patch SG0002378

Read from `patches/SG0002378/usr/cpu/firmware/mmscfw.bin` (patch SG0002378).
Image SHA-256: `b1249e58cf4f00c252362da8ed0515e25bb94aaf096bb60a5c5082c9b6e4d342`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred end and can include literals or padding
after returns. Statements below concern sites reached by known control-flow edges.
One computed jump remains unresolved in `_nvramInit`; its selector arms are excluded from the facts.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/nvram-sram-sg0002378-facts.txt) include all tool sections.

The following raw bytes remain after the decoder stops in two span tails:

| Span | Raw-byte start | Exclusive end | Bytes |
|---|---|---|---|
| `_nvramUpgrade1to2` | `0x127485` | `0x127488` | `00 90 90` |
| `_nvramUpgrade2to3` | `0x1275f6` | `0x1275f8` | `00 90` |

## `_nvramInit`

- `_nvramInit`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x126cb8 to 0x126fd8.
  Established. The `_sram_read` candidates begin `0x1dcf50`, `0x40`, and `0x0` (`call` at 0x126da1);
  its later decision takes `ret(_sram_read) == 0x1` and falls through for inequality (`je` at
  0x126dc0). A later branch takes `[0x1dcf54] == 0x5` and falls through for inequality (`je` at
  0x126e08). The computed transfer that follows on another route remains unresolved (`jmp` at
  0x126e19). Upgrade-selector arm effects are excluded from these facts.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x126ce0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x126ce6 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x126d0f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x126d39 | load | 4 | `ret(___stderr) +0x0` | `-` |
| `mov` at 0x126d5e | load | 4 | `ret(___stderr) +0x0` | `-` |
| `mov` at 0x126d78 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x126d7e | store | 4 | `(unknown)` | `?` |
| `mov` at 0x126d92 | store | 4 | `abs 0x1dc084` | `?` |
| `mov` at 0x126dcd | load | 4 | `ret(___stderr) +0x0` | `-` |
| `cmp` at 0x126de8 | load | 4 | `abs 0x1dcf50` | `-` |
| `cmp` at 0x126e01 | load | 4 | `abs 0x1dcf54` | `-` |
| `mov` at 0x126e0e | load | 4 | `abs 0x1dcf54` | `-` |
| `jmp` at 0x126e19 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x126e75 | store | 4 | `abs 0x1dcf50` | `0x0` |
| `mov` at 0x126e88 | load | 4 | `abs 0x1dcf54` | `-` |
| `mov` at 0x126e98 | load | 4 | `ret(___stderr) +0x0` | `-` |
| `cmp` at 0x126ea8 | load | 4 | `abs 0x1dcf58` | `-` |
| `mov` at 0x126eb1 | load | 4 | `abs 0x1dcf5c` | `-` |
| `cmp` at 0x126eb6 | load | 4 | `abs 0x1dc084` | `-` |
| `mov` at 0x126ec8 | load | 4 | `ret(___stderr) +0x0` | `-` |
| `cmp` at 0x126ed8 | load | 4 | `abs 0x1dcf58` | `-` |
| `cmp` at 0x126eec | load | 4 | `abs 0x1dcf58` | `-` |
| `add` at 0x126efc | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x126f01 | load | 4 | `abs 0x1dcf58` | `-` |
| `cmp` at 0x126f09 | load | 4 | `abs 0x1dcf5c` | `-` |
| `mov` at 0x126f18 | load | 4 | `abs 0x1dc084` | `-` |
| `mov` at 0x126f1d | store | 4 | `abs 0x1dcf5c` | `[0x1dc084]` |
| `mov` at 0x126f22 | store | 4 | `abs 0x1dcf58` | `0xd` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x126cc5 | `_bzero 0x172678` | `0x1dcf50` | `0x40` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x126cd4 | `_bzero 0x172678` | `0x1dc648` | `0x68` | `?` | `?` | `?` | `?` |
| `call` at 0x126d34 | `___stderr 0x18e17c` | `0x126bb9` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x126d3c | `_fprintf 0x17c4ec` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126d59 | `___stderr 0x18e17c` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126d61 | `_fprintf 0x17c4ec` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126da1 | `_sram_read 0x10c998` | `0x1dcf50` | `0x40` | `0x0` | `?` | `?` | `?` |
| `call` at 0x126db0 | `_nvramReset 0x1271a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126dc8 | `___stderr 0x18e17c` | `0x126c0c` | `ret(_sram_read)` | `?` | `?` | `?` | `?` |
| `call` at 0x126dd0 | `_fprintf 0x17c4ec` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126df4 | `_nvramReset 0x1271a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126e93 | `___stderr 0x18e17c` | `0x126c1c` | `[0x1dcf54]` | `?` | `?` | `?` | `?` |
| `call` at 0x126e9b | `_fprintf 0x17c4ec` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126ec3 | `___stderr 0x18e17c` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126ecb | `_fprintf 0x17c4ec` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126f2c | `_nvramWriteHeader 0x127808` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x126cef | `be flags condition (operands unknown)` | `not (be flags condition (operands unknown))` |
| `je` at 0x126d16 | `? == 0x0` | `? != 0x0` |
| `jne` at 0x126d2a | `Z == 0` | `not (Z == 0)` |
| `jbe` at 0x126d51 | `be flags condition (operands unknown)` | `not (be flags condition (operands unknown))` |
| `jbe` at 0x126d8c | `be flags condition (operands unknown)` | `not (be flags condition (operands unknown))` |
| `jne` at 0x126dae | `ret(_sram_read) != 0xfffffffd` | `ret(_sram_read) == 0xfffffffd` |
| `jne` at 0x126db7 | `(ret(_nvramReset) & ret(_nvramReset)) != 0x0` | `(ret(_nvramReset) & ret(_nvramReset)) == 0x0` |
| `je` at 0x126dc0 | `ret(_sram_read) == 0x1` | `ret(_sram_read) != 0x1` |
| `je` at 0x126df2 | `[0x1dcf50] == 0xff5cff5c` | `[0x1dcf50] != 0xff5cff5c` |
| `jne` at 0x126dfb | `(ret(_nvramReset) & ret(_nvramReset)) != 0x0` | `(ret(_nvramReset) & ret(_nvramReset)) == 0x0` |
| `je` at 0x126e08 | `[0x1dcf54] == 0x5` | `[0x1dcf54] != 0x5` |
| `ja` at 0x126e17 | `? > 0x4 (unsigned, 32-bit)` | `? <= 0x4 (unsigned, 32-bit)` |
| `jne` at 0x126eaf | `[0x1dcf58] != 0xd` | `[0x1dcf58] == 0xd` |
| `je` at 0x126ebc | `[0x1dc084] == [0x1dcf5c]` | `[0x1dc084] != [0x1dcf5c]` |
| `jbe` at 0x126edf | `[0x1dcf58] <= 0xd (unsigned, 32-bit)` | `[0x1dcf58] > 0xd (unsigned, 32-bit)` |
| `jbe` at 0x126ef2 | `[0x1dcf58] <= 0x0 (unsigned, 32-bit)` | `[0x1dcf58] > 0x0 (unsigned, 32-bit)` |
| `ja` at 0x126f07 | `[0x1dcf58] > ? (unsigned, 32-bit)` | `[0x1dcf58] <= ? (unsigned, 32-bit)` |
| `je` at 0x126f0f | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x126f33 | `(ret(_nvramWriteHeader) & ret(_nvramWriteHeader)) != 0x0` | `(ret(_nvramWriteHeader) & ret(_nvramWriteHeader)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x126f56 | `eax` | `?` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x126e19).

## `_nvramIsValid`

- `_nvramIsValid`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x127378 to 0x1273a8.
  Established. The indexed four-byte load has unknown effective target (`mov` at 0x127386). EAX
  remains unknown at the return (`ret` at 0x12739b). The selected validity bit and record are not
  resolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12737b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x127386 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12739b | `eax` | `?` |

## `_nvramPrintInfo`

- `_nvramPrintInfo`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x126fd8 to 0x127108.
  Established. The initial decision takes `[0x1dcf50] == 0xff5cff5c` and falls through for
  inequality (`je` at 0x126feb); its taken route calls `_ffscMsg` with leading candidates `0x126f78`
  and `[0x1dcf54]` (`call` at 0x127003). A later `_ffscMsg` call has leading candidates `0x126f95`,
  `[0x1dcf58]`, and `[0x1dcf5c]` (`call` at 0x12701c). EAX reaches the return as `0x0` (`ret` at
  0x127085). The displayed field meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x126fe1 | load | 4 | `abs 0x1dcf50` | `-` |
| `mov` at 0x126ff8 | load | 4 | `abs 0x1dcf54` | `-` |
| `mov` at 0x12700b | load | 4 | `abs 0x1dcf5c` | `-` |
| `mov` at 0x127011 | load | 4 | `abs 0x1dcf58` | `-` |
| `mov` at 0x127040 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x127054 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x12705f | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x127003 | `_ffscMsg 0x11f040` | `0x126f78` | `[0x1dcf54]` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x12701c | `_ffscMsg 0x11f040` | `0x126f95` | `[0x1dcf58]` | `[0x1dcf5c]` | `?` | `?` | `?` |
| `call` at 0x127029 | `_ffscMsgN 0x11f0a0` | `0x126fb7` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127057 | `_ffscMsgN 0x11f0a0` | `0x126fcc` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127075 | `_ffscMsg 0x11f040` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x126feb | `[0x1dcf50] == 0xff5cff5c` | `[0x1dcf50] != 0xff5cff5c` |
| `je` at 0x127049 | `Z == 1` | `not (Z == 1)` |
| `jle` at 0x127068 | `? <= 0x1f (signed, 32-bit)` | `? > 0x1f (signed, 32-bit)` |
| `jle` at 0x12706e | `? <= 0x3 (signed, 32-bit)` | `? > 0x3 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x127085 | `eax` | `0x0` |

## `_nvramRead`

- `_nvramRead`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x127108 to 0x1271a8.
  Established. The range branch takes `[sp+4] > 0xc (unsigned, 32-bit)` and falls through for lesser
  or equal values (`ja` at 0x127117). A later route calls `_nvramIsValid` with leading candidate
  `[sp+4]` (`call` at 0x127159); its decision takes `(ret(_nvramIsValid) & ret(_nvramIsValid)) ==
  0x0` and falls through for nonzero (`je` at 0x127163). That fall-through calls `_sram_read` with
  leading candidates `[sp+8]`, unknown, and unknown (`call` at 0x127172). The object bounds and byte
  count remain unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12710e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x127111 | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x127125 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x127138 | load | 4 | `abs 0x1dcf50` | `-` |
| `mov` at 0x127165 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12716c | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x127149 | `_ffscMsg 0x11f040` | `0x1270ad` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x127159 | `_nvramIsValid 0x127378` | `[sp+4]` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x127172 | `_sram_read 0x10c998` | `[sp+8]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12718e | `_ffscMsg 0x11f040` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x127117 | `[sp+4] > 0xc (unsigned, 32-bit)` | `[sp+4] <= 0xc (unsigned, 32-bit)` |
| `jne` at 0x12712a | `? != 0x0` | `? == 0x0` |
| `je` at 0x127142 | `[0x1dcf50] == 0xff5cff5c` | `[0x1dcf50] != 0xff5cff5c` |
| `je` at 0x127163 | `(ret(_nvramIsValid) & ret(_nvramIsValid)) == 0x0` | `(ret(_nvramIsValid) & ret(_nvramIsValid)) != 0x0` |
| `jne` at 0x12717d | `ret(_sram_read) != 0x1` | `ret(_sram_read) == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1271a1 | `eax` | `?` |

## `_nvramReset`

- `_nvramReset`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1271a8 to 0x127298.
  Established. Four-byte stores put `0xff5cff5c` at absolute `0x1dcf50` (`mov` at 0x1271ba), `0x5`
  at absolute `0x1dcf54` (`mov` at 0x1271c4), `0xd` at absolute `0x1dcf58` (`mov` at 0x1271ce), and
  `[0x1dc084]` at absolute `0x1dcf5c` (`mov` at 0x1271dd). The `_nvramValidate` candidates begin
  `0x0` (`call` at 0x1271e4). The later branch takes `(ret(_nvramWriteHeader) &
  ret(_nvramWriteHeader)) != 0x0` and falls through for zero (`jne` at 0x1271f3). The header-field
  interpretations remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1271ba | store | 4 | `abs 0x1dcf50` | `0xff5cff5c` |
| `mov` at 0x1271c4 | store | 4 | `abs 0x1dcf54` | `0x5` |
| `mov` at 0x1271ce | store | 4 | `abs 0x1dcf58` | `0xd` |
| `mov` at 0x1271d8 | load | 4 | `abs 0x1dc084` | `-` |
| `mov` at 0x1271dd | store | 4 | `abs 0x1dcf5c` | `[0x1dc084]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1271b2 | `_bzero 0x172678` | `0x1dcf50` | `0x40` | `ebp` | `?` | `?` | `?` |
| `call` at 0x1271e4 | `_nvramValidate 0x1277a8` | `0x0` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1271ec | `_nvramWriteHeader 0x127808` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x1271f3 | `(ret(_nvramWriteHeader) & ret(_nvramWriteHeader)) != 0x0` | `(ret(_nvramWriteHeader) & ret(_nvramWriteHeader)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1271fa | `eax` | `0x0` |
| `ret` at 0x127210 | `eax` | `0xffffffff` |

## `_nvramUpgrade1to2`

- `_nvramUpgrade1to2`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1273a8 to 0x127488.
  Established. The first `_nvramRead` site has leading candidates `0x6` and `sp-52` (`call` at
  0x1273b5). A later route includes a byte store of `0xa0` at `sp -0x28` (`mov` at 0x1273c7) and
  `_nvramWrite` with leading candidates `0x6` and `sp-52` (`call` at 0x1273ce). Another route stores
  four-byte `0x2` at absolute `0x1dcf54` (`mov` at 0x127418). The record translation is not
  established from these effects alone.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `test` at 0x1273c1 | load | 1 | `sp -0x20` | `-` |
| `mov` at 0x1273c7 | store | 1 | `sp -0x28` | `0xa0` |
| `test` at 0x1273ec | load | 1 | `sp -0x20` | `-` |
| `mov` at 0x1273f2 | store | 1 | `sp -0x28` | `0xa0` |
| `mov` at 0x127418 | store | 4 | `abs 0x1dcf54` | `0x2` |
| `mov` at 0x127427 | load | 4 | `sp -0x38` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1273b5 | `_nvramRead 0x127108` | `0x6` | `sp-52` | `ebx` | `?` | `?` | `?` |
| `call` at 0x1273ce | `_nvramWrite 0x127298` | `0x6` | `sp-52` | `?` | `?` | `?` | `?` |
| `call` at 0x1273e0 | `_nvramRead 0x127108` | `0x7` | `sp-52` | `?` | `?` | `?` | `?` |
| `call` at 0x1273f9 | `_nvramWrite 0x127298` | `0x7` | `sp-52` | `?` | `?` | `?` | `?` |
| `call` at 0x127422 | `_nvramWriteHeader 0x127808` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x1273bf | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` |
| `je` at 0x1273c5 | `(([sp-32]:1 & 0xff) & 0x1) == 0x0` | `(([sp-32]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x1273d8 | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` |
| `jne` at 0x1273ea | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` |
| `je` at 0x1273f0 | `(([sp-32]:1 & 0xff) & 0x1) == 0x0` | `(([sp-32]:1 & 0xff) & 0x1) != 0x0` |
| `je` at 0x127403 | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12742d | `eax` | `?` |

## `_nvramUpgrade2to3`

- `_nvramUpgrade2to3`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x127488 to 0x1275f8.
  Established. Calling `_nvramRead` first presents leading candidates `0x8` and `sp-388` (`call` at
  0x1274c8). A later `_nvramWrite` site begins `0x4` and `sp-484` (`call` at 0x1275ba); its decision
  takes `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` and falls through for zero (`jne` at
  0x1275c4). That fall-through writes four-byte `0x3` at absolute `0x1dcf54` (`mov` at 0x1275c6).
  The indexed field substitutions remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x127492 | load | 4 | `abs 0x123634` | `-` |
| `mov` at 0x1274a4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1274a8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1274ac | store | 4 | `(unknown)` | `?` |
| `cmp` at 0x1274b6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1274e7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1274ee | store | 4 | `(unknown)` | `?` |
| `mov` at 0x127504 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12750b | store | 4 | `(unknown)` | `?` |
| `mov` at 0x127521 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x127528 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x12753e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x127545 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x12755b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x127562 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x127578 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12757f | store | 4 | `(unknown)` | `?` |
| `mov` at 0x1275aa | load | 2 | `abs 0x12b194` | `-` |
| `mov` at 0x1275b0 | store | 2 | `sp -0x1b0` | `[0x12b194]:2` |
| `mov` at 0x1275c6 | store | 4 | `abs 0x1dcf54` | `0x3` |
| `mov` at 0x1275dd | load | 4 | `sp -0x1e8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1274c8 | `_nvramRead 0x127108` | `0x8` | `sp-388` | `?` | `?` | `?` | `?` |
| `call` at 0x1274dd | `_ffscTuneVarToIndex 0x125d18` | `0x12742e` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1274fa | `_ffscTuneVarToIndex 0x125d18` | `0x12743d` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127517 | `_ffscTuneVarToIndex 0x125d18` | `0x12744a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127534 | `_ffscTuneVarToIndex 0x125d18` | `0x127458` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127551 | `_ffscTuneVarToIndex 0x125d18` | `0x127467` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12756e | `_ffscTuneVarToIndex 0x125d18` | `0x127477` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127589 | `_nvramWrite 0x127298` | `0x8` | `sp-388` | `?` | `?` | `?` | `?` |
| `call` at 0x12759e | `_nvramRead 0x127108` | `0x4` | `sp-484` | `?` | `?` | `?` | `?` |
| `call` at 0x1275ba | `_nvramWrite 0x127298` | `0x4` | `sp-484` | `?` | `?` | `?` | `?` |
| `call` at 0x1275d0 | `_nvramWriteHeader 0x127808` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x127499 | `[0x123634] == 0x0` | `[0x123634] != 0x0` |
| `jne` at 0x1274bd | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x1274d2 | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` |
| `jne` at 0x127593 | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` |
| `jne` at 0x1275a8 | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` |
| `jne` at 0x1275c4 | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1275e6 | `eax` | `?` |

## `_nvramUpgrade3to4`

- `_nvramUpgrade3to4`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1275f8 to 0x1276a8.
  Established. The `_nvramRead` candidates begin `0x8` and `sp-388` (`call` at 0x127638). A later
  `_nvramWrite` call presents the same leading candidates (`call` at 0x127664); its branch takes
  `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` and falls through for zero (`jne` at 0x12766e). The
  fall-through stores four-byte `0x4` at absolute `0x1dcf54` (`mov` at 0x127670). The tuning-field
  meaning remains unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x127602 | load | 4 | `abs 0x123634` | `-` |
| `mov` at 0x127614 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x127618 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12761c | store | 4 | `(unknown)` | `?` |
| `cmp` at 0x127626 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x127653 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12765a | store | 4 | `(unknown)` | `?` |
| `mov` at 0x127670 | store | 4 | `abs 0x1dcf54` | `0x4` |
| `mov` at 0x12768d | load | 4 | `sp -0x188` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x127638 | `_nvramRead 0x127108` | `0x8` | `sp-388` | `?` | `?` | `?` | `?` |
| `call` at 0x127649 | `_ffscTuneVarToIndex 0x125d18` | `0x1275e7` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127664 | `_nvramWrite 0x127298` | `0x8` | `sp-388` | `?` | `?` | `?` | `?` |
| `call` at 0x12767a | `_nvramWriteHeader 0x127808` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x127609 | `[0x123634] == 0x0` | `[0x123634] != 0x0` |
| `jne` at 0x12762d | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x127642 | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` |
| `jne` at 0x12766e | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x127696 | `eax` | `?` |

## `_nvramUpgrade4to5`

- `_nvramUpgrade4to5`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1276a8 to 0x1277a8.
  Established. The second `_nvramRead` site has leading candidates `0x4` and `sp-484` (`call` at
  0x12772d). A later indexed two-byte store has unknown effective target and value (`mov` at
  0x12774e). The final `_nvramWrite` decision takes `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0`
  and falls through for zero (`jne` at 0x12776c). That fall-through writes four-byte `0x5` at
  absolute `0x1dcf54` (`mov` at 0x12776e). The updated option records are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x1276b3 | load | 4 | `abs 0x123634` | `-` |
| `mov` at 0x1276c4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1276c8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1276cc | store | 4 | `(unknown)` | `?` |
| `cmp` at 0x1276d6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x127707 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12770e | store | 4 | `(unknown)` | `?` |
| `mov` at 0x127749 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x12774e | store | 2 | `(unknown)` | `?` |
| `mov` at 0x12776e | store | 4 | `abs 0x1dcf54` | `0x5` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1276e8 | `_nvramRead 0x127108` | `0x8` | `sp-388` | `?` | `?` | `?` | `?` |
| `call` at 0x1276fd | `_ffscTuneVarToIndex 0x125d18` | `0x127697` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127718 | `_nvramWrite 0x127298` | `0x8` | `sp-388` | `?` | `?` | `?` | `?` |
| `call` at 0x12772d | `_nvramRead 0x127108` | `0x4` | `sp-484` | `?` | `?` | `?` | `?` |
| `call` at 0x127762 | `_nvramWrite 0x127298` | `0x4` | `sp-484` | `?` | `?` | `?` | `?` |
| `call` at 0x127778 | `_nvramWriteHeader 0x127808` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1276ba | `[0x123634] == 0x0` | `[0x123634] != 0x0` |
| `jne` at 0x1276dd | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x1276f2 | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` |
| `jne` at 0x127722 | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` |
| `jne` at 0x127737 | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` |
| `jle` at 0x127757 | `? <= 0x2 (signed, 32-bit)` | `? > 0x2 (signed, 32-bit)` |
| `jne` at 0x12776c | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x127798 | `eax` | `?` |

## `_nvramValidate`

- `_nvramValidate`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1277a8 to 0x127808.
  Established. The indexed four-byte read (`mov` at 0x1277c3) and four-byte store (`mov` at
  0x1277cc) have unknown effective targets. The store value is also unknown (`mov` at 0x1277cc). EAX
  is unresolved at the return (`ret` at 0x1277d9). The selected validation-state bit is not
  established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1277af | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1277c3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1277cc | store | 4 | `(unknown)` | `?` |
| `mov` at 0x1277d3 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1277d9 | `eax` | `?` |

## `_nvramWrite`

- `_nvramWrite`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x127298 to 0x127358.
  Established. The `_sram_write` candidates begin `[sp+8]`, unknown, and unknown (`call` at
  0x1272f5); its branch takes `ret(_sram_write) == 0x1` and falls through for inequality (`je` at
  0x127300). The taken route later calls `_nvramIsValid` with leading candidate `[sp+4]` (`call` at
  0x127319); its branch takes `(ret(_nvramIsValid) & ret(_nvramIsValid)) != 0x0` and falls through
  for zero (`jne` at 0x127323). The fall-through calls `_nvramValidate` with leading candidate
  `[sp+4]` (`call` at 0x127326). The physical persistence outcome remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12729d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1272a0 | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x1272b4 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x1272c8 | load | 4 | `abs 0x1dcf50` | `-` |
| `mov` at 0x1272e8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1272ef | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1272d9 | `_ffscMsg 0x11f040` | `0x127239` | `ebx` | `esi` | `ebp` | `?` | `?` |
| `call` at 0x1272f5 | `_sram_write 0x10ca78` | `[sp+8]` | `?` | `?` | `ebx` | `esi` | `ebp` |
| `call` at 0x127308 | `_ffscMsg 0x11f040` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127319 | `_nvramIsValid 0x127378` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127326 | `_nvramValidate 0x1277a8` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12732e | `_nvramWriteHeader 0x127808` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x1272a6 | `[sp+4] > 0xc (unsigned, 32-bit)` | `[sp+4] <= 0xc (unsigned, 32-bit)` |
| `jne` at 0x1272b9 | `? != 0x0` | `? == 0x0` |
| `je` at 0x1272d2 | `[0x1dcf50] == 0xff5cff5c` | `[0x1dcf50] != 0xff5cff5c` |
| `je` at 0x127300 | `ret(_sram_write) == 0x1` | `ret(_sram_write) != 0x1` |
| `jne` at 0x127323 | `(ret(_nvramIsValid) & ret(_nvramIsValid)) != 0x0` | `(ret(_nvramIsValid) & ret(_nvramIsValid)) == 0x0` |
| `je` at 0x127335 | `(ret(_nvramWriteHeader) & ret(_nvramWriteHeader)) == 0x0` | `(ret(_nvramWriteHeader) & ret(_nvramWriteHeader)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x127352 | `eax` | `?` |

## `_nvramWriteHeader`

- `_nvramWriteHeader`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x127808 to 0x12783c.
  Established. Calls `_sram_write` with leading candidates `0x1dcf50`, `0x40`, and `0x0` (`call` at
  0x127814); its decision takes `ret(_sram_write) != 0x1` and falls through for equality (`jne` at
  0x12781f). The two routes return EAX `0xffffffff` (`ret` at 0x12783b) and `0x0` (`ret` at
  0x127826), respectively. The stored-header contract is unknown beyond those candidates.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x127814 | `_sram_write 0x10ca78` | `0x1dcf50` | `0x40` | `0x0` | `ebp` | `?` | `?` |
| `call` at 0x12782e | `_ffscMsg 0x11f040` | `0x1277da` | `ret(_sram_write)` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x12781f | `ret(_sram_write) != 0x1` | `ret(_sram_write) == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x127826 | `eax` | `0x0` |
| `ret` at 0x12783b | `eax` | `0xffffffff` |

## `_nvramZap`

- `_nvramZap`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x127358 to 0x127378.
  Established. The `_bzero` candidates begin `0x1dcf50` and `0x40` (`call` at 0x127362).
  `_nvramWriteHeader` follows with unknown candidates (`call` at 0x12736a). The return carries EAX
  `ret(_nvramWriteHeader)` (`ret` at 0x127372). The physical erase effect is not established by the
  local calls.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x127362 | `_bzero 0x172678` | `0x1dcf50` | `0x40` | `ebp` | `?` | `?` | `?` |
| `call` at 0x12736a | `_nvramWriteHeader 0x127808` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x127372 | `eax` | `ret(_nvramWriteHeader)` |

## `_sram_read`

- `_sram_read`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10c998 to 0x10ca78.
  Established. A two-byte load reads absolute `0xa2000` (`mov` at 0x10c9b2). The branch takes
  `([0xa2000]:2 & 0xffff) == 0xaa55` and falls through for inequality (`je` at 0x10c9bc). A later
  route stores byte `0x0` at an unknown target (`mov` at 0x10c9fc). Another route stores four-byte
  `0x0` at absolute `0x1ac890` (`mov` at 0x10ca18). The mapped-region identity and checksum
  interpretation remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10c9aa | store | 4 | `sp -0x8` | `ret(_sram_on)` |
| `mov` at 0x10c9b2 | load | 2 | `abs 0xa2000` | `-` |
| `cmp` at 0x10c9c8 | load | 4 | `abs 0x1ac890` | `-` |
| `mov` at 0x10c9d8 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10c9f1 | load | 4 | `abs 0xa2004` | `-` |
| `mov` at 0x10c9fc | store | 1 | `(unknown)` | `0x0` |
| `mov` at 0x10ca18 | store | 4 | `abs 0x1ac890` | `0x0` |
| `mov` at 0x10ca28 | load | 4 | `sp +0xc` | `-` |
| `cmp` at 0x10ca2e | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10ca34 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10ca37 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10ca3a | store | 1 | `(unknown)` | `?` |
| `cmp` at 0x10ca3f | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10ca44 | load | 4 | `sp +0xc` | `-` |
| `add` at 0x10ca47 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10ca5b | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10c9a5 | `_sram_on 0x10cb38` | `ebx` | `esi` | `edi` | `?` | `ebp` | `?` |
| `call` at 0x10ca5f | `_sram_off 0x10cb68` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10c9bc | `([0xa2000]:2 & 0xffff) == 0xaa55` | `([0xa2000]:2 & 0xffff) != 0xaa55` |
| `je` at 0x10c9cf | `[0x1ac890] == 0x0` | `[0x1ac890] != 0x0` |
| `jbe` at 0x10c9e9 | `? <= 0x1ff7 (unsigned, 32-bit)` | `? > 0x1ff7 (unsigned, 32-bit)` |
| `je` at 0x10c9f6 | `Z == 1` | `not (Z == 1)` |
| `jbe` at 0x10ca07 | `? <= 0x1ff7 (unsigned, 32-bit)` | `? > 0x1ff7 (unsigned, 32-bit)` |
| `jne` at 0x10ca24 | `(0x0 & 0x0) != 0x0` | `(0x0 & 0x0) == 0x0` |
| `jle` at 0x10ca31 | `[sp+8] <= 0x0 (signed, 32-bit)` | `[sp+8] > 0x0 (signed, 32-bit)` |
| `jg` at 0x10ca42 | `[sp+8] > ? (signed, 32-bit)` | `[sp+8] <= ? (signed, 32-bit)` |
| `jbe` at 0x10ca54 | `([sp+12] + [sp+8]) <= 0x1ff8 (unsigned, 32-bit)` | `([sp+12] + [sp+8]) > 0x1ff8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ca6f | `eax` | `?` |

## `_sram_write`

- `_sram_write`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ca78 to 0x10cb38.
  Established. The branch takes `([0xa2000]:2 & 0xffff) == 0xaa55` and falls through for inequality
  (`je` at 0x10caa1); its fall-through writes two-byte `0xaa55` at absolute `0xa2000` (`mov` at
  0x10caa3), then byte `0x0` at absolute `0xa2002` (`mov` at 0x10caac) and absolute `0xa2003` (`mov`
  at 0x10cab3). Later routes include four-byte stores of `0x0` (`mov` at 0x10cadc) and unknown
  (`mov` at 0x10cb00) at absolute `0xa2004`. The region semantics and computed stored value remain
  unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10ca83 | store | 4 | `sp -0xc` | `0x0` |
| `mov` at 0x10ca8f | store | 4 | `sp -0x8` | `ret(_sram_on)` |
| `mov` at 0x10ca97 | load | 2 | `abs 0xa2000` | `-` |
| `mov` at 0x10caa3 | store | 2 | `abs 0xa2000` | `0xaa55` |
| `mov` at 0x10caac | store | 1 | `abs 0xa2002` | `0x0` |
| `mov` at 0x10cab3 | store | 1 | `abs 0xa2003` | `0x0` |
| `mov` at 0x10cabc | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10cac2 | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x10cac5 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x10cacc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10cacf | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10cad2 | store | 1 | `(unknown)` | `?` |
| `cmp` at 0x10cad7 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10cadc | store | 4 | `abs 0xa2004` | `0x0` |
| `mov` at 0x10cae8 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10cb00 | store | 4 | `abs 0xa2004` | `?` |
| `mov` at 0x10cb03 | load | 4 | `sp +0xc` | `-` |
| `add` at 0x10cb06 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10cb09 | store | 4 | `sp -0xc` | `0x1` |
| `mov` at 0x10cb17 | store | 4 | `sp -0xc` | `0xffffffff` |
| `mov` at 0x10cb1e | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10cb27 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ca8a | `_sram_on 0x10cb38` | `ebx` | `esi` | `edi` | `0x0` | `?` | `ebp` |
| `call` at 0x10cb22 | `_sram_off 0x10cb68` | `[sp-8]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10caa1 | `([0xa2000]:2 & 0xffff) == 0xaa55` | `([0xa2000]:2 & 0xffff) != 0xaa55` |
| `jge` at 0x10cac8 | `[sp-12] >= [sp+8] (signed, 32-bit)` | `[sp-12] < [sp+8] (signed, 32-bit)` |
| `jg` at 0x10cada | `[sp+8] > ? (signed, 32-bit)` | `[sp+8] <= ? (signed, 32-bit)` |
| `jbe` at 0x10caf9 | `? <= 0x1ff7 (unsigned, 32-bit)` | `? > 0x1ff7 (unsigned, 32-bit)` |
| `jbe` at 0x10cb15 | `([sp+12] + [sp+8]) <= 0x1ff8 (unsigned, 32-bit)` | `([sp+12] + [sp+8]) > 0x1ff8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10cb33 | `eax` | `[sp-12]` |
