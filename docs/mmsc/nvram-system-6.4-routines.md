# MMSC NVRAM and system helpers in IRIX 6.4

Read from `6.4/usr/cpu/firmware/mmscfw.bin` (IRIX 6.4).
Image SHA-256: `58ac958cb0c3fa09f807340ec3f2f3864e3c601b3fc5f057ae9a8100db18a098`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred end and can include literals after returns.
Statements below concern sites reached by known control-flow edges.
I/O prefixes in the access table identify ports rather than memory addresses.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/nvram-system-6.4-facts.txt) include all tool sections.

## `_nvramInit`

- `_nvramInit`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x126988 to 0x126c78.
  Established. Two `_bzero` calls have leading candidate pairs `0x1db600`, `0x40` (`call` at
  0x126995) and `0x1dacf8`, `0x68` (`call` at 0x1269a4). `_sram_read` presents leading candidates
  `0x1db600`, `0x40`, and `0x0` (`call` at 0x126a71). The branch takes `ret(_sram_read) !=
  0xfffffffd` and falls through for equality (`jne` at 0x126a7e). Alternative paths include a four-byte store of zero at
  absolute `0x1db600` (`mov` at 0x126b30) or `0xd` at absolute `0x1db608` (`mov` at 0x126bc2). The
  layout and device initialization semantics are unknown beyond the recorded operations.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1269b0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1269b6 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x1269df | load | 4 | `(unknown)` | `-` |
| `mov` at 0x126a09 | load | 4 | `ret(___stderr) +0x0` | `-` |
| `mov` at 0x126a2e | load | 4 | `ret(___stderr) +0x0` | `-` |
| `mov` at 0x126a48 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x126a4e | store | 4 | `(unknown)` | `?` |
| `mov` at 0x126a62 | store | 4 | `abs 0x1da734` | `?` |
| `mov` at 0x126a9d | load | 4 | `ret(___stderr) +0x0` | `-` |
| `cmp` at 0x126ab8 | load | 4 | `abs 0x1db600` | `-` |
| `cmp` at 0x126ad1 | load | 4 | `abs 0x1db604` | `-` |
| `mov` at 0x126ada | load | 4 | `abs 0x1db604` | `-` |
| `mov` at 0x126b18 | load | 4 | `abs 0x1db604` | `-` |
| `mov` at 0x126b28 | load | 4 | `ret(___stderr) +0x0` | `-` |
| `mov` at 0x126b30 | store | 4 | `abs 0x1db600` | `0x0` |
| `cmp` at 0x126b48 | load | 4 | `abs 0x1db608` | `-` |
| `mov` at 0x126b51 | load | 4 | `abs 0x1db60c` | `-` |
| `cmp` at 0x126b56 | load | 4 | `abs 0x1da734` | `-` |
| `mov` at 0x126b68 | load | 4 | `ret(___stderr) +0x0` | `-` |
| `cmp` at 0x126b78 | load | 4 | `abs 0x1db608` | `-` |
| `cmp` at 0x126b8c | load | 4 | `abs 0x1db608` | `-` |
| `add` at 0x126b9c | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x126ba1 | load | 4 | `abs 0x1db608` | `-` |
| `cmp` at 0x126ba9 | load | 4 | `abs 0x1db60c` | `-` |
| `mov` at 0x126bb8 | load | 4 | `abs 0x1da734` | `-` |
| `mov` at 0x126bbd | store | 4 | `abs 0x1db60c` | `[0x1da734]` |
| `mov` at 0x126bc2 | store | 4 | `abs 0x1db608` | `0xd` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x126995 | `_bzero 0x1719c0` | `0x1db600` | `0x40` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x1269a4 | `_bzero 0x1719c0` | `0x1dacf8` | `0x68` | `?` | `?` | `?` | `?` |
| `call` at 0x126a04 | `___stderr 0x18c86c` | `0x126889` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x126a0c | `_fprintf 0x17abdc` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126a29 | `___stderr 0x18c86c` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126a31 | `_fprintf 0x17abdc` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126a71 | `_sram_read 0x10c928` | `0x1db600` | `0x40` | `0x0` | `?` | `?` | `?` |
| `call` at 0x126a80 | `_nvramReset 0x126e48` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126a98 | `___stderr 0x18c86c` | `0x1268dc` | `ret(_sram_read)` | `?` | `?` | `?` | `?` |
| `call` at 0x126aa0 | `_fprintf 0x17abdc` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126ac4 | `_nvramReset 0x126e48` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126af8 | `_nvramUpgrade1to2 0x127048` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126b01 | `_nvramWriteHeader 0x127138` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126b23 | `___stderr 0x18c86c` | `0x1268ec` | `[0x1db604]` | `?` | `?` | `?` | `?` |
| `call` at 0x126b2b | `_fprintf 0x17abdc` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126b63 | `___stderr 0x18c86c` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126b6b | `_fprintf 0x17abdc` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126bcc | `_nvramWriteHeader 0x127138` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x1269bf | `be flags condition (operands unknown)` | `not (be flags condition (operands unknown))` |
| `je` at 0x1269e6 | `? == 0x0` | `? != 0x0` |
| `jne` at 0x1269fa | `Z == 0` | `not (Z == 0)` |
| `jbe` at 0x126a21 | `be flags condition (operands unknown)` | `not (be flags condition (operands unknown))` |
| `jbe` at 0x126a5c | `be flags condition (operands unknown)` | `not (be flags condition (operands unknown))` |
| `jne` at 0x126a7e | `ret(_sram_read) != 0xfffffffd` | `ret(_sram_read) == 0xfffffffd` |
| `jne` at 0x126a87 | `(ret(_nvramReset) & ret(_nvramReset)) != 0x0` | `(ret(_nvramReset) & ret(_nvramReset)) == 0x0` |
| `je` at 0x126a90 | `ret(_sram_read) == 0x1` | `ret(_sram_read) != 0x1` |
| `je` at 0x126ac2 | `[0x1db600] == 0xff5cff5c` | `[0x1db600] != 0xff5cff5c` |
| `jne` at 0x126acb | `(ret(_nvramReset) & ret(_nvramReset)) != 0x0` | `(ret(_nvramReset) & ret(_nvramReset)) == 0x0` |
| `je` at 0x126ad8 | `[0x1db604] == 0x2` | `[0x1db604] != 0x2` |
| `je` at 0x126ae2 | `[0x1db604] == 0x1` | `[0x1db604] != 0x1` |
| `je` at 0x126ae7 | `[0x1db604] == 0x2` | `[0x1db604] != 0x2` |
| `jne` at 0x126aff | `(ret(_nvramUpgrade1to2) & ret(_nvramUpgrade1to2)) != 0x0` | `(ret(_nvramUpgrade1to2) & ret(_nvramUpgrade1to2)) == 0x0` |
| `je` at 0x126b08 | `(ret(_nvramWriteHeader) & ret(_nvramWriteHeader)) == 0x0` | `(ret(_nvramWriteHeader) & ret(_nvramWriteHeader)) != 0x0` |
| `jne` at 0x126b4f | `[0x1db608] != 0xd` | `[0x1db608] == 0xd` |
| `je` at 0x126b5c | `[0x1da734] == [0x1db60c]` | `[0x1da734] != [0x1db60c]` |
| `jbe` at 0x126b7f | `[0x1db608] <= 0xd (unsigned, 32-bit)` | `[0x1db608] > 0xd (unsigned, 32-bit)` |
| `jbe` at 0x126b92 | `[0x1db608] <= 0x0 (unsigned, 32-bit)` | `[0x1db608] > 0x0 (unsigned, 32-bit)` |
| `ja` at 0x126ba7 | `[0x1db608] > ? (unsigned, 32-bit)` | `[0x1db608] <= ? (unsigned, 32-bit)` |
| `je` at 0x126baf | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x126bd3 | `(ret(_nvramWriteHeader) & ret(_nvramWriteHeader)) != 0x0` | `(ret(_nvramWriteHeader) & ret(_nvramWriteHeader)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x126bf6 | `eax` | `?` |

## `_nvramIsValid`

- `_nvramIsValid`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x127018 to 0x127048.
  Established. A four-byte read uses `sp +0x4` (`mov` at 0x12701b), and another has an unknown effective
  address (`mov` at 0x127026). The return value in EAX is unknown (`ret` at 0x12703b). The
  validity-check role remains unknown because the result is unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12701b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x127026 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12703b | `eax` | `?` |

## `_nvramPrintInfo`

- `_nvramPrintInfo`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x126c78 to 0x126da8.
  Established. The first branch takes `[0x1db600] == 0xff5cff5c` and falls through on inequality
  (`je` at 0x126c8b). The taken route includes `_ffscMsg` with leading candidates `0x126c18` and
  `[0x1db604]` (`call` at 0x126ca3). Another `_ffscMsg` call presents `0x126c35`, `[0x1db608]`, and
  `[0x1db60c]` (`call` at 0x126cbc). The routine returns EAX zero (`ret` at 0x126d25). The printed
  fields and their NVRAM meaning are unknown here.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x126c81 | load | 4 | `abs 0x1db600` | `-` |
| `mov` at 0x126c98 | load | 4 | `abs 0x1db604` | `-` |
| `mov` at 0x126cab | load | 4 | `abs 0x1db60c` | `-` |
| `mov` at 0x126cb1 | load | 4 | `abs 0x1db608` | `-` |
| `mov` at 0x126ce0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x126cf4 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x126cff | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x126ca3 | `_ffscMsg 0x11edf0` | `0x126c18` | `[0x1db604]` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x126cbc | `_ffscMsg 0x11edf0` | `0x126c35` | `[0x1db608]` | `[0x1db60c]` | `?` | `?` | `?` |
| `call` at 0x126cc9 | `_ffscMsgN 0x11ee50` | `0x126c57` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126cf7 | `_ffscMsgN 0x11ee50` | `0x126c6c` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126d15 | `_ffscMsg 0x11edf0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x126c8b | `[0x1db600] == 0xff5cff5c` | `[0x1db600] != 0xff5cff5c` |
| `je` at 0x126ce9 | `Z == 1` | `not (Z == 1)` |
| `jle` at 0x126d08 | `? <= 0x1f (signed, 32-bit)` | `? > 0x1f (signed, 32-bit)` |
| `jle` at 0x126d0e | `? <= 0x3 (signed, 32-bit)` | `? > 0x3 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x126d25 | `eax` | `0x0` |

## `_nvramRead`

- `_nvramRead`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x126da8 to 0x126e48.
  Established. An unsigned branch takes `[sp+4] > 0xc` and falls through for values at most `0xc`
  (`ja` at 0x126db7). The `_nvramIsValid` call has leading candidate `[sp+4]` (`call` at 0x126df9).
  `_sram_read` exposes leading candidate `[sp+8]`, with all other candidates unknown (`call` at
  0x126e12). The following branch takes `ret(_sram_read) != 0x1` and falls through for equality
  (`jne` at 0x126e1d). The record bounds and physical read mapping are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x126dae | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x126db1 | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x126dc5 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x126dd8 | load | 4 | `abs 0x1db600` | `-` |
| `mov` at 0x126e05 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x126e0c | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x126de9 | `_ffscMsg 0x11edf0` | `0x126d4d` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x126df9 | `_nvramIsValid 0x127018` | `[sp+4]` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x126e12 | `_sram_read 0x10c928` | `[sp+8]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126e2e | `_ffscMsg 0x11edf0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x126db7 | `[sp+4] > 0xc (unsigned, 32-bit)` | `[sp+4] <= 0xc (unsigned, 32-bit)` |
| `jne` at 0x126dca | `? != 0x0` | `? == 0x0` |
| `je` at 0x126de2 | `[0x1db600] == 0xff5cff5c` | `[0x1db600] != 0xff5cff5c` |
| `je` at 0x126e03 | `(ret(_nvramIsValid) & ret(_nvramIsValid)) == 0x0` | `(ret(_nvramIsValid) & ret(_nvramIsValid)) != 0x0` |
| `jne` at 0x126e1d | `ret(_sram_read) != 0x1` | `ret(_sram_read) == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x126e41 | `eax` | `?` |

## `_nvramReset`

- `_nvramReset`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x126e48 to 0x126f38.
  Established. Four-byte stores write `0xff5cff5c` at absolute `0x1db600` (`mov` at 0x126e5a), `0x2` at
  `0x1db604` (`mov` at 0x126e64), and `0xd` at `0x1db608` (`mov` at 0x126e6e). Another four-byte store
  writes `[0x1da734]` at absolute `0x1db60c` (`mov` at 0x126e7d). `_nvramValidate` has leading
  candidate `0x0` (`call` at 0x126e84). The branch takes `(ret(_nvramWriteHeader) &
  ret(_nvramWriteHeader)) != 0x0` and falls through for zero (`jne` at 0x126e93). The header-field
  meanings and reset persistence remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x126e5a | store | 4 | `abs 0x1db600` | `0xff5cff5c` |
| `mov` at 0x126e64 | store | 4 | `abs 0x1db604` | `0x2` |
| `mov` at 0x126e6e | store | 4 | `abs 0x1db608` | `0xd` |
| `mov` at 0x126e78 | load | 4 | `abs 0x1da734` | `-` |
| `mov` at 0x126e7d | store | 4 | `abs 0x1db60c` | `[0x1da734]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x126e52 | `_bzero 0x1719c0` | `0x1db600` | `0x40` | `ebp` | `?` | `?` | `?` |
| `call` at 0x126e84 | `_nvramValidate 0x1270d8` | `0x0` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126e8c | `_nvramWriteHeader 0x127138` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x126e93 | `(ret(_nvramWriteHeader) & ret(_nvramWriteHeader)) != 0x0` | `(ret(_nvramWriteHeader) & ret(_nvramWriteHeader)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x126e9a | `eax` | `0x0` |
| `ret` at 0x126eb0 | `eax` | `0xffffffff` |

## `_nvramUpgrade1to2`

- `_nvramUpgrade1to2`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x127048 to 0x1270d8.
  Established. The first `_nvramRead` call presents leading candidates `0x6` and `sp-52` (`call` at
  0x127055). A branch takes `(([sp-32]:1 & 0xff) & 0x1) == 0x0` and falls through when the
  expression is nonzero (`je` at 0x127065). That fall-through writes byte `0xa0` at `sp -0x28`
  (`mov` at 0x127067), then calls `_nvramWrite` with leading candidates `0x6` and `sp-52` (`call` at
  0x12706e). Another path writes four bytes containing `0x2` at absolute `0x1db604` (`mov` at 0x1270b8).
  The first write-result branch takes `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0`
  past that store (`jne` at 0x127078). The second takes equality to zero to the store and
  falls through on inequality (`je` at 0x1270a3).
  The compatibility transformation represented by these values is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `test` at 0x127061 | load | 1 | `sp -0x20` | `-` |
| `mov` at 0x127067 | store | 1 | `sp -0x28` | `0xa0` |
| `test` at 0x12708c | load | 1 | `sp -0x20` | `-` |
| `mov` at 0x127092 | store | 1 | `sp -0x28` | `0xa0` |
| `mov` at 0x1270b8 | store | 4 | `abs 0x1db604` | `0x2` |
| `mov` at 0x1270c7 | load | 4 | `sp -0x38` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x127055 | `_nvramRead 0x126da8` | `0x6` | `sp-52` | `ebx` | `?` | `?` | `?` |
| `call` at 0x12706e | `_nvramWrite 0x126f38` | `0x6` | `sp-52` | `?` | `?` | `?` | `?` |
| `call` at 0x127080 | `_nvramRead 0x126da8` | `0x7` | `sp-52` | `?` | `?` | `?` | `?` |
| `call` at 0x127099 | `_nvramWrite 0x126f38` | `0x7` | `sp-52` | `?` | `?` | `?` | `?` |
| `call` at 0x1270c2 | `_nvramWriteHeader 0x127138` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x12705f | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` |
| `je` at 0x127065 | `(([sp-32]:1 & 0xff) & 0x1) == 0x0` | `(([sp-32]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x127078 | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` |
| `jne` at 0x12708a | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` |
| `je` at 0x127090 | `(([sp-32]:1 & 0xff) & 0x1) == 0x0` | `(([sp-32]:1 & 0xff) & 0x1) != 0x0` |
| `je` at 0x1270a3 | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1270cd | `eax` | `?` |

## `_nvramValidate`

- `_nvramValidate`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1270d8 to 0x127138.
  Established. A four-byte read has an unresolved effective target (`mov` at 0x1270f3). The subsequent
  four-byte store has both unknown effective target and unknown value (`mov` at 0x1270fc). EAX is unknown
  at the return (`ret` at 0x127109). The validity-bit update role cannot be established from those
  unresolved accesses.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1270df | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1270f3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1270fc | store | 4 | `(unknown)` | `?` |
| `mov` at 0x127103 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x127109 | `eax` | `?` |

## `_nvramWriteHeader`

- `_nvramWriteHeader`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x127138 to 0x12716c.
  Established. The `_sram_write` call has leading candidates `0x1db600`, `0x40`, and `0x0` (`call`
  at 0x127144). The branch takes `ret(_sram_write) != 0x1` and falls through for equality (`jne` at
  0x12714f); its fall-through exit carries EAX zero (`ret` at 0x127156), while the taken route
  returns `0xffffffff` (`ret` at 0x12716b). The underlying persistent-memory mapping is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x127144 | `_sram_write 0x10ca08` | `0x1db600` | `0x40` | `0x0` | `ebp` | `?` | `?` |
| `call` at 0x12715e | `_ffscMsg 0x11edf0` | `0x12710a` | `ret(_sram_write)` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x12714f | `ret(_sram_write) != 0x1` | `ret(_sram_write) == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x127156 | `eax` | `0x0` |
| `ret` at 0x12716b | `eax` | `0xffffffff` |

## `_nvramZap`

- `_nvramZap`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x126ff8 to 0x127018.
  Established. Calls `_bzero` with leading candidates `0x1db600` and `0x40` (`call` at 0x127002),
  followed by `_nvramWriteHeader` with unknown candidates (`call` at 0x12700a). The exit carries
  `ret(_nvramWriteHeader)` in EAX (`ret` at 0x127012). Whether this clears persistent storage is
  unknown without those callees.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x127002 | `_bzero 0x1719c0` | `0x1db600` | `0x40` | `ebp` | `?` | `?` | `?` |
| `call` at 0x12700a | `_nvramWriteHeader 0x127138` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x127012 | `eax` | `ret(_nvramWriteHeader)` |

## `_nvramWrite`

- `_nvramWrite`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x126f38 to 0x126ff8.
  Established. The unsigned branch takes `[sp+4] > 0xc` and falls through for `[sp+4] <= 0xc` (`ja`
  at 0x126f46). `_sram_write` has leading candidate `[sp+8]`, followed by two unknown candidates
  (`call` at 0x126f95). The next branch takes `ret(_sram_write) == 0x1` and falls through on
  inequality (`je` at 0x126fa0). Calls on the taken route include `_nvramIsValid` (`call` at
  0x126fb9); its result branch takes `(ret(_nvramIsValid) & ret(_nvramIsValid)) != 0x0`
  past the subsequent calls and falls through for zero (`jne` at 0x126fc3). That fall-through
  calls `_nvramValidate` (`call` at 0x126fc6) and `_nvramWriteHeader` (`call` at 0x126fce).
  The write length, storage offset and persistence rules remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x126f3d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x126f40 | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x126f54 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x126f68 | load | 4 | `abs 0x1db600` | `-` |
| `mov` at 0x126f88 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x126f8f | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x126f79 | `_ffscMsg 0x11edf0` | `0x126ed9` | `ebx` | `esi` | `ebp` | `?` | `?` |
| `call` at 0x126f95 | `_sram_write 0x10ca08` | `[sp+8]` | `?` | `?` | `ebx` | `esi` | `ebp` |
| `call` at 0x126fa8 | `_ffscMsg 0x11edf0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126fb9 | `_nvramIsValid 0x127018` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126fc6 | `_nvramValidate 0x1270d8` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126fce | `_nvramWriteHeader 0x127138` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x126f46 | `[sp+4] > 0xc (unsigned, 32-bit)` | `[sp+4] <= 0xc (unsigned, 32-bit)` |
| `jne` at 0x126f59 | `? != 0x0` | `? == 0x0` |
| `je` at 0x126f72 | `[0x1db600] == 0xff5cff5c` | `[0x1db600] != 0xff5cff5c` |
| `je` at 0x126fa0 | `ret(_sram_write) == 0x1` | `ret(_sram_write) != 0x1` |
| `jne` at 0x126fc3 | `(ret(_nvramIsValid) & ret(_nvramIsValid)) != 0x0` | `(ret(_nvramIsValid) & ret(_nvramIsValid)) == 0x0` |
| `je` at 0x126fd5 | `(ret(_nvramWriteHeader) & ret(_nvramWriteHeader)) == 0x0` | `(ret(_nvramWriteHeader) & ret(_nvramWriteHeader)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x126ff2 | `eax` | `?` |

## `_sysNvRamGet`

- `_sysNvRamGet`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1087c8 to 0x1087d8.
  Established. Loads EAX with `0xffffffff` (`mov` at 0x1087cb) and returns that value (`ret` at
  0x1087d3). The exposed body provides a constant result rather than an established NVRAM read; its
  service role is unknown.

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

- `_sysNvRamSet`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1087d8 to 0x1087e8.
  Established. The EAX assignment is `0xffffffff` (`mov` at 0x1087db), which reaches the exit
  unchanged (`ret` at 0x1087e3). The NVRAM-set operation implied by the symbol is not established.
  The service semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1087e3 | `eax` | `0xffffffff` |

## `_sysA20on`

- `_sysA20on`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108100 to 0x1081a0.
  Established. I/O byte stores send `0xd1` to port `0x64` (`out` at 0x10810a), `0xdf` to port `0x60`
  (`out` at 0x108116), and `0xff` to port `0x64` (`out` at 0x108122). Four-byte stores clear absolute
  `0x0` (`mov` at 0x108133) and `0x100000` (`mov` at 0x108139), then put `0x1234567` at absolute
  `0x0` (`mov` at 0x10813f). The branch takes `[0x100000] != 0x1234567` and falls through for
  equality (`jne` at 0x10814b); its fall-through includes a byte store of `0x2` to I/O port `0x92`
  (`out` at 0x108152) and a byte read from port `0x92` (`in` at 0x108156).
  The second comparison branch takes `[0x100000] != 0x1234567` to the return and falls
  through for equality (`jne` at 0x10817e). That fall-through writes four bytes containing `0xffffffff`
  at absolute `0x108190` (`mov` at 0x108180). Their A20 hardware interpretation remains unknown.

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

## `_sysIntDisable`

- `_sysIntDisable`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10bb08 to 0x10bb18.
  Established. The body assigns EAX `0xffffffff` (`mov` at 0x10bb0b) and returns it (`ret` at
  0x10bb13). The interrupt-disable effect is unknown. The established behavior is the constant
  result.

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

- `_sysIntEnable`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10bb18 to 0x10bb28.
  Established. At the exit, EAX is `0xffffffff` (`ret` at 0x10bb23), supplied by its immediate
  assignment (`mov` at 0x10bb1b). No interrupt-enabling role is established by that result, so the
  device effect stays unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10bb23 | `eax` | `0xffffffff` |
