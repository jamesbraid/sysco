# MMSC buffers and serial I/O in patch SG0002378

Read from `patches/SG0002378/usr/cpu/firmware/mmscfw.bin` (patch SG0002378).
Image SHA-256: `b1249e58cf4f00c252362da8ed0515e25bb94aaf096bb60a5c5082c9b6e4d342`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred end and can include literals or padding
after returns. Statements below concern sites reached by known control-flow edges.
No unresolved jump transfers were reported in these spans.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/buffers-serial-io-sg0002378-facts.txt) include all tool sections.

## `_bufRead`

- `_bufRead`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10f9e8 to 0x10fae8.
  Established. The `_timeoutRead` call presents `[[sp+4]]`, `[[sp+4]+8]`, `[[sp+4]+20]`, and
  `[0x1dc738]` as its first four candidates (`call` at 0x10fa24). Two later four-byte stores place
  `([[sp+4]+8] + ret(_timeoutRead))` at `[sp+4] +0x8` (`mov` at 0x10fa8d) and `([[sp+4]+16] +
  ret(_timeoutRead))` at `[sp+4] +0x10` (`mov` at 0x10fa95). Their record meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10f9ec | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f9ef | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fa08 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10fa13 | load | 4 | `abs 0x1dc738` | `-` |
| `mov` at 0x10fa19 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x10fa1d | load | 4 | `[sp+4] +0x8` | `-` |
| `mov` at 0x10fa21 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fa39 | load | 4 | `ret(___errno) +0x0` | `-` |
| `mov` at 0x10fa40 | load | 4 | `abs 0x1dc738` | `-` |
| `mov` at 0x10fa46 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x10fa4a | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fa68 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fa88 | load | 4 | `[sp+4] +0x8` | `-` |
| `mov` at 0x10fa8d | store | 4 | `[sp+4] +0x8` | `([[sp+4]+8] + ret(_timeoutRead))` |
| `mov` at 0x10fa90 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10fa95 | store | 4 | `[sp+4] +0x10` | `([[sp+4]+16] + ret(_timeoutRead))` |
| `mov` at 0x10fa98 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x10fa9d | store | 4 | `[sp+4] +0x14` | `?` |
| `mov` at 0x10faa0 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x10faad | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10fab2 | store | 4 | `[sp+4] +0x18` | `?` |
| `mov` at 0x10fab7 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10fa24 | `_timeoutRead 0x12add0` | `[[sp+4]]` | `[[sp+4]+8]` | `[[sp+4]+20]` | `[0x1dc738]` | `ebx` | `ebp` |
| `call` at 0x10fa34 | `___errno 0x1711d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10fa52 | `_ffscMsg 0x11f040` | `0x10f9a4` | `[[sp+4]]` | `[[sp+4]+20]` | `[0x1dc738]` | `?` | `?` |
| `call` at 0x10fa70 | `_ffscMsgS 0x11f110` | `0x10f9c6` | `[[sp+4]]` | `?` | `?` | `?` | `?` |
| `call` at 0x10faa8 | `_bufHold 0x10f748` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x10f9f3 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x10fa0d | `(? & 0x18) != 0x0` | `(? & 0x18) == 0x0` |
| `jg` at 0x10fa30 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `jge` at 0x10fa32 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x10fa3e | `[ret(___errno)] != 0x4` | `[ret(___errno)] == 0x4` |
| `jg` at 0x10faa5 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10fabd | `eax` | `?` |

## `_bufWrite`

- `_bufWrite`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10fc28 to 0x10ff20.
  Established. The `_timeoutWrite` site has leading candidates `[[sp+4]]`, `[sp+8]`, `[sp+12]`, and
  `[0x1dc73c]` (`call` at 0x10fd36); its result is stored as four bytes at `sp -0x8` (`mov` at
  0x10fd3e). The error-value branch is taken when `[ret(___errno)] == 0x4` (`je` at 0x10fd4f),
  reaching a different four-byte store of `[sp+12]` to `sp -0x8` (`mov` at 0x10fd68).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10fc31 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10fc34 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10fc48 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x10fc50 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10fc54 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x10fc60 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10fc67 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10fc6e | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fc78 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x10fc7f | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x10fc83 | load | 4 | `[sp+4] +0x8` | `-` |
| `mov` at 0x10fc87 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10fc93 | load | 4 | `[sp+4] +0x8` | `-` |
| `mov` at 0x10fc98 | store | 4 | `[sp+4] +0x8` | `?` |
| `mov` at 0x10fc9b | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10fca0 | store | 4 | `[sp+4] +0x10` | `?` |
| `mov` at 0x10fca3 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x10fca8 | store | 4 | `[sp+4] +0x14` | `?` |
| `mov` at 0x10fcab | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10fcb2 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fcb8 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10fcbb | store | 4 | `sp -0x20` | `[[sp+4]+40]` |
| `mov` at 0x10fcbe | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fcc3 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fcc8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10fcd0 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fcd5 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x10fcd8 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fce8 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x10fceb | load | 4 | `sp -0x20` | `-` |
| `mov` at 0x10fcee | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10fcf3 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10fcf6 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10fcfd | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10fd00 | load | 4 | `[[sp+4]+44] +0x0` | `-` |
| `mov` at 0x10fd03 | store | 4 | `[[sp+4]+44] +0x0` | `?` |
| `mov` at 0x10fd05 | load | 4 | `[[sp+4]+44] +0x0` | `-` |
| `test` at 0x10fd07 | load | 1 | `abs 0x1dccc0` | `-` |
| `mov` at 0x10fd1f | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x10fd28 | load | 4 | `abs 0x1dc73c` | `-` |
| `mov` at 0x10fd2f | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10fd33 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fd3e | store | 4 | `sp -0x8` | `ret(_timeoutWrite)` |
| `mov` at 0x10fd4a | load | 4 | `ret(___errno) +0x0` | `-` |
| `mov` at 0x10fd51 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fd68 | store | 4 | `sp -0x8` | `[sp+12]` |
| `cmp` at 0x10fd6b | load | 4 | `sp -0x8` | `-` |
| `sub` at 0x10fd74 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10fd77 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10fd7e | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10fd82 | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x10fd86 | load | 4 | `sp +0x8` | `-` |
| `add` at 0x10fd89 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10fd95 | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x10fd9a | store | 4 | `[sp+4] +0x8` | `?` |
| `mov` at 0x10fd9d | store | 4 | `[sp+4] +0x10` | `?` |
| `mov` at 0x10fda0 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10fda5 | store | 4 | `[sp+4] +0x14` | `?` |
| `mov` at 0x10fda8 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10fdaf | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10fdb2 | store | 4 | `sp -0x20` | `[[sp+4]+40]` |
| `mov` at 0x10fdb5 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fdba | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fdbf | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10fdc7 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fdcc | store | 4 | `sp -0x14` | `?` |
| `mov` at 0x10fdcf | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fddf | load | 4 | `sp -0x14` | `-` |
| `mov` at 0x10fde2 | load | 4 | `sp -0x20` | `-` |
| `mov` at 0x10fde5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10fdea | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10fded | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10fdf4 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10fdf7 | load | 4 | `[[sp+4]+44] +0x0` | `-` |
| `mov` at 0x10fdfa | store | 4 | `[[sp+4]+44] +0x0` | `?` |
| `mov` at 0x10fdfc | load | 4 | `[[sp+4]+44] +0x0` | `-` |
| `test` at 0x10fdfe | load | 1 | `abs 0x1dccc0` | `-` |
| `add` at 0x10fe15 | load | 4 | `sp -0x8` | `-` |
| `add` at 0x10fe15 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x10fe18 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10fe23 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10fe28 | store | 4 | `[sp+4] +0x18` | `?` |
| `mov` at 0x10fe2b | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x10fe32 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10fe36 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x10fe3e | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10fe4e | load | 4 | `[sp+4] +0x20` | `-` |
| `cmp` at 0x10fe5b | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10fe60 | load | 4 | `[sp+4] +0x20` | `-` |
| `add` at 0x10fe63 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10fe66 | store | 4 | `[sp+4] +0x20` | `([[sp+4]+32] + [sp-8])` |
| `mov` at 0x10fe7b | store | 4 | `sp -0x20` | `[sp+4]+36` |
| `mov` at 0x10fe88 | load | 4 | `sp -0x20` | `-` |
| `mov` at 0x10fe95 | load | 4 | `sp +0x8` | `-` |
| `add` at 0x10fe98 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10feae | store | 4 | `[sp+4] +0x20` | `0x0` |
| `mov` at 0x10feb5 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10feba | store | 4 | `[sp+4] +0x18` | `?` |
| `mov` at 0x10fec8 | store | 4 | `[sp+4] +0x20` | `?` |
| `mov` at 0x10fed8 | store | 4 | `sp -0x1c` | `([sp+8] + [sp-8])` |
| `mov` at 0x10fedb | load | 4 | `sp -0x20` | `-` |
| `mov` at 0x10feee | store | 4 | `sp -0x20` | `[sp-32]` |
| `mov` at 0x10fef5 | load | 4 | `sp -0x20` | `-` |
| `mov` at 0x10fef9 | load | 4 | `sp -0x1c` | `-` |
| `mov` at 0x10ff10 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10fc58 | `_logWrite 0x124f9c` | `[[sp+4]+48]` | `[sp+8]` | `[sp+12]` | `ebx` | `esi` | `edi` |
| `call` at 0x10fc8b | `_bcopy 0x176bac` | `[sp+8]` | `[[sp+4]+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x10fd17 | `_ffscMsg 0x11f040` | `0x10fbc8` | `?` | `[sp+4]` | `?` | `?` | `?` |
| `call` at 0x10fd36 | `_timeoutWrite 0x12b020` | `[[sp+4]]` | `[sp+8]` | `[sp+12]` | `[0x1dc73c]` | `?` | `?` |
| `call` at 0x10fd45 | `___errno 0x1711d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10fd59 | `_ffscMsgS 0x11f110` | `0x10fbee` | `[[sp+4]]` | `?` | `?` | `?` | `?` |
| `call` at 0x10fd8d | `_bcopy 0x176bac` | `?` | `[[sp+4]+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x10fe0d | `_ffscMsg 0x11f040` | `0x10fc08` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10fe42 | `_strncmp 0x176d68` | `[sp+8]` | `([sp+4] + [[sp+4]+32])+36` | `[sp-8]` | `?` | `?` | `?` |
| `call` at 0x10fe56 | `_strlen 0x17466c` | `([sp+4] + [[sp+4]+32])+36` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10fe7f | `_strlen 0x17466c` | `[sp+4]+36` | `?` | `?` | `?` | `[sp+4]+36` | `?` |
| `call` at 0x10fe8d | `_strlen 0x17466c` | `[sp-32]` | `[sp-32]` | `ret(_strlen)` | `?` | `?` | `?` |
| `call` at 0x10fea2 | `_strncmp 0x176d68` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10fedf | `_strlen 0x17466c` | `[sp-32]` | `?` | `?` | `?` | `?` | `([sp+8] + [sp-8])` |
| `call` at 0x10feff | `_strncmp 0x176d68` | `?` | `[sp-32]` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x10fc39 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `je` at 0x10fc4d | `([[sp+4]+48] & [[sp+4]+48]) == 0x0` | `([[sp+4]+48] & [[sp+4]+48]) != 0x0` |
| `jg` at 0x10fc65 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `jne` at 0x10fc6c | `(? & 0x8) != 0x0` | `(? & 0x8) == 0x0` |
| `jge` at 0x10fc72 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jle` at 0x10fc7d | `[sp+12] <= [[sp+4]+20] (signed, 32-bit)` | `[sp+12] > [[sp+4]+20] (signed, 32-bit)` |
| `je` at 0x10fcb0 | `([[sp+4]+40] & [[sp+4]+40]) == 0x0` | `([[sp+4]+40] & [[sp+4]+40]) != 0x0` |
| `jl` at 0x10fcb6 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jb` at 0x10fcce | `C == 1` | `not (C == 1)` |
| `je` at 0x10fcfb | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |
| `je` at 0x10fd0e | `(([0x1dccc0]:1 & 0xff) & 0x8) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x8) != 0x0` |
| `jge` at 0x10fd43 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x10fd4f | `[ret(___errno)] == 0x4` | `[ret(___errno)] != 0x4` |
| `je` at 0x10fd6e | `Z == 1` | `not (Z == 1)` |
| `jle` at 0x10fd7c | `? <= [[sp+4]+12] (signed, 32-bit)` | `? > [[sp+4]+12] (signed, 32-bit)` |
| `je` at 0x10fdad | `([[sp+4]+40] & [[sp+4]+40]) == 0x0` | `([[sp+4]+40] & [[sp+4]+40]) != 0x0` |
| `jb` at 0x10fdc5 | `C == 1` | `not (C == 1)` |
| `je` at 0x10fdf2 | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |
| `je` at 0x10fe05 | `(([0x1dccc0]:1 & 0xff) & 0x8) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x8) != 0x0` |
| `je` at 0x10fe1d | `(? & 0x40) == 0x0` | `(? & 0x40) != 0x0` |
| `jle` at 0x10fe30 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jne` at 0x10fe4c | `(ret(_strncmp) & ret(_strncmp)) != 0x0` | `(ret(_strncmp) & ret(_strncmp)) == 0x0` |
| `je` at 0x10fe5e | `[sp-8] == ret(_strlen)` | `[sp-8] != ret(_strlen)` |
| `jne` at 0x10feac | `(ret(_strncmp) & ret(_strncmp)) != 0x0` | `(ret(_strncmp) & ret(_strncmp)) == 0x0` |
| `jle` at 0x10feec | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `je` at 0x10ff09 | `(ret(_strncmp) & ret(_strncmp)) == 0x0` | `(ret(_strncmp) & ret(_strncmp)) != 0x0` |
| `jg` at 0x10ff0e | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ff1c | `eax` | `?` |

## `_bufFlush`

- `_bufFlush`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10f608 to 0x10f748.
  Established. The `_timeoutWrite` call uses `[[sp+4]]`, `[[sp+4]+4]`, `[[sp+4]+16]`, and
  `[0x1dc73c]` as leading candidates (`call` at 0x10f629). A byte at `sp -0x8` is initially cleared
  (`mov` at 0x10f614). On the taken path when `[ret(___errno)] == 0x4` (`je` at 0x10f641), that byte
  is set to `0x1` (`mov` at 0x10f65b).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10f611 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f614 | store | 1 | `sp -0x8` | `0x0` |
| `mov` at 0x10f618 | load | 4 | `abs 0x1dc73c` | `-` |
| `mov` at 0x10f61e | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10f622 | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x10f626 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f63c | load | 4 | `ret(___errno) +0x0` | `-` |
| `mov` at 0x10f658 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10f65b | store | 1 | `sp -0x8` | `0x1` |
| `mov` at 0x10f65f | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10f66a | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10f670 | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x10f676 | load | 4 | `[sp+4] +0x4` | `-` |
| `test` at 0x10f682 | load | 1 | `abs 0x1dccc0` | `-` |
| `mov` at 0x10f68f | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f692 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10f6a8 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10f6af | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10f6b2 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f6b7 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x10f6ba | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f6ca | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x10f6cd | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10f6d2 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10f6d5 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10f6dc | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10f6df | load | 4 | `[[sp+4]+44] +0x0` | `-` |
| `mov` at 0x10f6e2 | store | 4 | `[[sp+4]+44] +0x0` | `?` |
| `mov` at 0x10f6e4 | load | 4 | `[[sp+4]+44] +0x0` | `-` |
| `test` at 0x10f6e6 | load | 1 | `abs 0x1dccc0` | `-` |
| `cmp` at 0x10f6ef | load | 1 | `sp -0x8` | `-` |
| `mov` at 0x10f6f5 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f708 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f715 | load | 4 | `[sp+4] +0x8` | `-` |
| `mov` at 0x10f71a | store | 4 | `[sp+4] +0x8` | `?` |
| `mov` at 0x10f71d | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10f722 | store | 4 | `[sp+4] +0x10` | `?` |
| `mov` at 0x10f725 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x10f72a | store | 4 | `[sp+4] +0x14` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10f629 | `_timeoutWrite 0x12b020` | `[[sp+4]]` | `[[sp+4]+4]` | `[[sp+4]+16]` | `[0x1dc73c]` | `ebx` | `esi` |
| `call` at 0x10f637 | `___errno 0x1711d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10f648 | `_ffscMsgS 0x11f110` | `0x10f579` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10f67a | `_memmove 0x180190` | `[[sp+4]+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10f69c | `_ffscMsg 0x11f040` | `0x10f59c` | `?` | `[[sp+4]+16]` | `[[sp+4]]` | `?` | `?` |
| `call` at 0x10f710 | `_ffscMsg 0x11f040` | `?` | `[[sp+4]]` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x10f635 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x10f641 | `[ret(___errno)] == 0x4` | `[ret(___errno)] != 0x4` |
| `je` at 0x10f664 | `Z == 1` | `not (Z == 1)` |
| `jle` at 0x10f668 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `je` at 0x10f689 | `(([0x1dccc0]:1 & 0xff) & 0x8) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x8) != 0x0` |
| `je` at 0x10f6ad | `([[sp+4]+40] & [[sp+4]+40]) == 0x0` | `([[sp+4]+40] & [[sp+4]+40]) != 0x0` |
| `je` at 0x10f6da | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |
| `je` at 0x10f6ed | `(([0x1dccc0]:1 & 0xff) & 0x8) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x8) != 0x0` |
| `jne` at 0x10f6f3 | `([sp-8]:1 & 0xff) != 0x0` | `([sp-8]:1 & 0xff) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10f738 | `eax` | `?` |

## `_bufHold`

- `_bufHold`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10f748 to 0x10f818.
  Established. The four-byte stores to `[sp+4] +0x18` (`mov` at 0x10f759) and `[sp+4] +0x1c` (`mov`
  at 0x10f760) have unknown values. Another four-byte store has unknown effective address and value
  (`mov` at 0x10f7a0). The branch bypassing the last nested access group is taken when `([[sp+4]+44]
  & [[sp+4]+44]) == 0x0` (`je` at 0x10f7a8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10f751 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f754 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10f759 | store | 4 | `[sp+4] +0x18` | `?` |
| `mov` at 0x10f75c | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x10f760 | store | 4 | `[sp+4] +0x1c` | `?` |
| `mov` at 0x10f763 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x10f766 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f76c | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10f773 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10f776 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f77b | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f780 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10f788 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f78d | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f79b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10f7a0 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10f7a3 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10f7aa | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10f7ad | load | 4 | `[[sp+4]+44] +0x0` | `-` |
| `mov` at 0x10f7b0 | store | 4 | `[[sp+4]+44] +0x0` | `?` |
| `mov` at 0x10f7b2 | load | 4 | `[[sp+4]+44] +0x0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jl` at 0x10f76a | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10f771 | `([[sp+4]+40] & [[sp+4]+40]) == 0x0` | `([[sp+4]+40] & [[sp+4]+40]) != 0x0` |
| `jae` at 0x10f786 | `C == 0` | `not (C == 0)` |
| `je` at 0x10f7a8 | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10f7bf | `eax` | `0x0` |

## `_bufRelease`

- `_bufRelease`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10fae8 to 0x10fc28.
  Established. The `_bufFlush` call has candidate zero `[sp+4]` (`call` at 0x10fb48). It is bypassed
  by the branch taken when `(? & 0x2) == 0x0` (`je` at 0x10fb45). The call result is stored as four
  bytes at `sp -0x10` (`mov` at 0x10fb4d). The indexed store later in the body retains unknown
  target and value (`mov` at 0x10fba7).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10faf1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10faf4 | store | 4 | `sp -0x10` | `0x0` |
| `mov` at 0x10fafb | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x10fb18 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x10fb1c | store | 4 | `[sp+4] +0x1c` | `?` |
| `mov` at 0x10fb1f | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x10fb22 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x10fb38 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10fb3d | store | 4 | `[sp+4] +0x18` | `?` |
| `mov` at 0x10fb40 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10fb4d | store | 4 | `sp -0x10` | `ret(_bufFlush)` |
| `mov` at 0x10fb50 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fb56 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10fb5d | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10fb60 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fb65 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fb6a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10fb72 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10fb79 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10fb80 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10fb87 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fb8c | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x10fb8f | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fb9f | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10fba2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10fba7 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10fbaa | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10fbb1 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10fbb4 | load | 4 | `[[sp+4]+44] +0x0` | `-` |
| `mov` at 0x10fbb7 | store | 4 | `[[sp+4]+44] +0x0` | `?` |
| `mov` at 0x10fbb9 | load | 4 | `[[sp+4]+44] +0x0` | `-` |
| `mov` at 0x10fbbb | load | 4 | `sp -0x10` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10fb07 | `_ffscMsg 0x11f040` | `0x10fabe` | `ebx` | `esi` | `edi` | `0x0` | `?` |
| `call` at 0x10fb48 | `_bufFlush 0x10f608` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x10fb00 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `jle` at 0x10fb27 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `je` at 0x10fb45 | `(? & 0x2) == 0x0` | `(? & 0x2) != 0x0` |
| `jl` at 0x10fb54 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10fb5b | `([[sp+4]+40] & [[sp+4]+40]) == 0x0` | `([[sp+4]+40] & [[sp+4]+40]) != 0x0` |
| `jb` at 0x10fb70 | `C == 1` | `not (C == 1)` |
| `jne` at 0x10fb77 | `(? & 0x1) != 0x0` | `(? & 0x1) == 0x0` |
| `je` at 0x10fb7e | `(? & 0x2) == 0x0` | `(? & 0x2) != 0x0` |
| `jle` at 0x10fb85 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `je` at 0x10fbaf | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10fbc7 | `eax` | `?` |

## `_bufAttachFD`

- `_bufAttachFD`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10f3f8 to 0x10f498.
  Established. The four-byte store to `[sp+4] +0x0` writes `[sp+8]` (`mov` at 0x10f416). The
  `_bufDetachFD` call has candidate zero `[sp+4]` (`call` at 0x10f40e). The later indexed four-byte
  store is unresolved (`mov` at 0x10f476), while the nested four-byte store through `[[sp+4]+44]
  +0x0` also has unknown value (`mov` at 0x10f486).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10f401 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f404 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10f407 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f416 | store | 4 | `[sp+4] +0x0` | `[sp+8]` |
| `mov` at 0x10f438 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10f43f | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10f446 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10f44d | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10f454 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10f45b | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10f45e | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f463 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f471 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10f476 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10f479 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10f480 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10f483 | load | 4 | `[[sp+4]+44] +0x0` | `-` |
| `mov` at 0x10f486 | store | 4 | `[[sp+4]+44] +0x0` | `?` |
| `mov` at 0x10f488 | load | 4 | `[[sp+4]+44] +0x0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10f40e | `_bufDetachFD 0x10f4f8` | `[sp+4]` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x10f422 | `_ffscMsg 0x11f040` | `0x10f3d4` | `[sp+8]` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jl` at 0x10f40b | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jge` at 0x10f41a | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x10f43d | `(? & 0x8) != 0x0` | `(? & 0x8) == 0x0` |
| `jne` at 0x10f444 | `(? & 0x1) != 0x0` | `(? & 0x1) == 0x0` |
| `je` at 0x10f44b | `(? & 0x2) == 0x0` | `(? & 0x2) != 0x0` |
| `jle` at 0x10f452 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `je` at 0x10f459 | `([[sp+4]+40] & [[sp+4]+40]) == 0x0` | `([[sp+4]+40] & [[sp+4]+40]) != 0x0` |
| `je` at 0x10f47e | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10f493 | `eax` | `?` |

## `_bufDetachFD`

- `_bufDetachFD`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10f4f8 to 0x10f568.
  Established. The four-byte store through `[sp+4] +0x0` writes `0xffffffff` (`mov` at 0x10f552). An
  earlier indexed four-byte store remains unknown in target and value (`mov` at 0x10f53e). The
  branch bypassing that indexed path is taken when `C == 0` (`jae` at 0x10f524). No call sites occur
  in the analysed span.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10f501 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f504 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f50a | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10f511 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10f514 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f519 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f51e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10f526 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f52b | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f539 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10f53e | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10f541 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10f548 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10f54b | load | 4 | `[[sp+4]+44] +0x0` | `-` |
| `mov` at 0x10f54e | store | 4 | `[[sp+4]+44] +0x0` | `?` |
| `mov` at 0x10f550 | load | 4 | `[[sp+4]+44] +0x0` | `-` |
| `mov` at 0x10f552 | store | 4 | `[sp+4] +0x0` | `0xffffffff` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jl` at 0x10f508 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10f50f | `([[sp+4]+40] & [[sp+4]+40]) == 0x0` | `([[sp+4]+40] & [[sp+4]+40]) != 0x0` |
| `jae` at 0x10f524 | `C == 0` | `not (C == 0)` |
| `je` at 0x10f546 | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10f561 | `eax` | `?` |

## `_bufInit`

- `_bufInit`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10f818 to 0x10f908.
  Established. The `_malloc` sites have candidate zero `0x34` (`call` at 0x10f826) and `[sp+4]+1`
  (`call` at 0x10f84c). Stores through the symbolic `ret(_malloc)` origin include `0xffffffff` at
  `+0x0` (`mov` at 0x10f85e), `[sp+4]` at `+0xc` (`mov` at 0x10f86a), and `[sp+8]` at `+0x18` (`mov`
  at 0x10f877), each four bytes. These expressions do not distinguish the two call results.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10f81e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f821 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10f854 | store | 4 | `ret(_malloc) +0x4` | `ret(_malloc)` |
| `mov` at 0x10f857 | load | 4 | `ret(_malloc) +0x4` | `-` |
| `mov` at 0x10f85e | store | 4 | `ret(_malloc) +0x0` | `0xffffffff` |
| `mov` at 0x10f864 | load | 4 | `ret(_malloc) +0x4` | `-` |
| `mov` at 0x10f867 | store | 4 | `ret(_malloc) +0x8` | `[ret(_malloc)+4]` |
| `mov` at 0x10f86a | store | 4 | `ret(_malloc) +0xc` | `[sp+4]` |
| `mov` at 0x10f86d | store | 4 | `ret(_malloc) +0x10` | `0x0` |
| `mov` at 0x10f874 | store | 4 | `ret(_malloc) +0x14` | `[sp+4]` |
| `mov` at 0x10f877 | store | 4 | `ret(_malloc) +0x18` | `[sp+8]` |
| `mov` at 0x10f87a | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10f87d | store | 4 | `ret(_malloc) +0x28` | `[sp+12]` |
| `mov` at 0x10f880 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x10f883 | store | 4 | `ret(_malloc) +0x2c` | `[sp+16]` |
| `mov` at 0x10f88e | store | 4 | `ret(_malloc) +0x1c` | `?` |
| `mov` at 0x10f891 | store | 4 | `ret(_malloc) +0x20` | `0x0` |
| `mov` at 0x10f8a6 | store | 4 | `ret(_malloc) +0x30` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10f826 | `_malloc 0x17ec28` | `0x34` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x10f84c | `_malloc 0x17ec28` | `[sp+4]+1` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10f8a1 | `_strcpy 0x18038c` | `ret(_malloc)+36` | `0x10f211` | `?` | `?` | `?` | `?` |
| `call` at 0x10f8bd | `_ffscMsgS 0x11f110` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10f832 | `(ret(_malloc) & ret(_malloc)) != 0x0` | `(ret(_malloc) & ret(_malloc)) == 0x0` |
| `je` at 0x10f85c | `([ret(_malloc)+4] & [ret(_malloc)+4]) == 0x0` | `([ret(_malloc)+4] & [ret(_malloc)+4]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10f8cd | `eax` | `?` |

## `_bufClear`

- `_bufClear`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10f4b8 to 0x10f4f8.
  Established. Three four-byte stores place zero at `[sp+4] +0x10` (`mov` at 0x10f4be),
  `[[sp+4]+12]` at `[sp+4] +0x14` (`mov` at 0x10f4c8), and `[[sp+4]+4]` at `[sp+4] +0x8` (`mov` at
  0x10f4ce). The branch bypasses `_bufRelease` when `(? & 0x10) == 0x0` (`je` at 0x10f4d6). That
  call has candidate zero `[sp+4]` (`call` at 0x10f4e1).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10f4bb | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f4be | store | 4 | `[sp+4] +0x10` | `0x0` |
| `mov` at 0x10f4c5 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10f4c8 | store | 4 | `[sp+4] +0x14` | `[[sp+4]+12]` |
| `mov` at 0x10f4cb | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x10f4ce | store | 4 | `[sp+4] +0x8` | `[[sp+4]+4]` |
| `mov` at 0x10f4d1 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10f4d8 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10f4dd | store | 4 | `[sp+4] +0x18` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10f4e1 | `_bufRelease 0x10fae8` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10f4d6 | `(? & 0x10) == 0x0` | `(? & 0x10) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10f4e9 | `eax` | `?` |

## `_bufLeftAlign`

- `_bufLeftAlign`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10f908 to 0x10f9e8.
  Established. The lower-bound branch is taken when `[sp+8] < [[sp+4]+4] (unsigned, 32-bit)` (`jb`
  at 0x10f917). The separate upper comparison is taken when `[sp+8] <= ([[sp+4]+4] + [[sp+4]+16])
  (unsigned, 32-bit)` (`jbe` at 0x10f923). The `_memmove` site presents `[[sp+4]+4]`, `[sp+8]`, and
  `[[sp+4]+16]` as leading candidates (`call` at 0x10f979).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10f90c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f90f | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10f912 | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x10f919 | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x10f91c | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10f948 | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x10f951 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10f956 | store | 4 | `[sp+4] +0x10` | `?` |
| `mov` at 0x10f959 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x10f95e | store | 4 | `[sp+4] +0x14` | `?` |
| `mov` at 0x10f961 | load | 4 | `[sp+4] +0x8` | `-` |
| `mov` at 0x10f966 | store | 4 | `[sp+4] +0x8` | `?` |
| `mov` at 0x10f969 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10f970 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10f975 | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x10f981 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x10f988 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10f995 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10f99a | store | 4 | `[sp+4] +0x18` | `?` |
| `mov` at 0x10f99d | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10f92a | `_ffscMsg 0x11f040` | `0x10f8ce` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x10f933 | `_bufClear 0x10f4b8` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10f979 | `_memmove 0x180190` | `[[sp+4]+4]` | `[sp+8]` | `[[sp+4]+16]` | `?` | `?` | `?` |
| `call` at 0x10f990 | `_bufRelease 0x10fae8` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jb` at 0x10f917 | `[sp+8] < [[sp+4]+4] (unsigned, 32-bit)` | `[sp+8] >= [[sp+4]+4] (unsigned, 32-bit)` |
| `jbe` at 0x10f923 | `[sp+8] <= ([[sp+4]+4] + [[sp+4]+16]) (unsigned, 32-bit)` | `[sp+8] > ([[sp+4]+4] + [[sp+4]+16]) (unsigned, 32-bit)` |
| `je` at 0x10f94f | `Z == 1` | `not (Z == 1)` |
| `jle` at 0x10f96e | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jle` at 0x10f986 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `je` at 0x10f98d | `(? & 0x10) == 0x0` | `(? & 0x10) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10f9a3 | `eax` | `?` |

## `_bufAppendBuf`

- `_bufAppendBuf`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10f058 to 0x10f0a8.
  Established. The `_bufWrite` site has leading candidates `[sp+4]`, `[[sp+8]+4]`, and `[[sp+8]+16]`
  (`call` at 0x10f06b). The later `_bufLeftAlign` site uses `[sp+8]` and `([[sp+8]+4] +
  ret(_bufWrite))` (`call` at 0x10f080); its intervening branch is taken under `l flags condition
  (operands unknown)` (`jl` at 0x10f077).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10f05c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f05f | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10f062 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x10f066 | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10f079 | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10f09d | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10f06b | `_bufWrite 0x10fc28` | `[sp+4]` | `[[sp+8]+4]` | `[[sp+8]+16]` | `ebx` | `ebp` | `?` |
| `call` at 0x10f080 | `_bufLeftAlign 0x10f908` | `[sp+8]` | `([[sp+8]+4] + ret(_bufWrite))` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jl` at 0x10f077 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10f0a3 | `eax` | `?` |

## `_bufAppendBufToEOL`

- `_bufAppendBufToEOL`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10f0a8 to 0x10f248.
  Established. A byte store clears `([[sp+8]+16] + [[sp+8]+4]) +0x0` (`mov` at 0x10f0bc). The
  `_strstr` call presents `[[sp+8]+4]` and `[sp+4]+36` as leading candidates (`call` at 0x10f10c).
  The `_bufWrite` site has candidate zero `[sp+4]`, candidate one `[[sp+8]+4]`, and unknown
  candidate two (`call` at 0x10f148).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10f0b3 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10f0b6 | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10f0b9 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x10f0bc | store | 1 | `([[sp+8]+16] + [[sp+8]+4]) +0x0` | `0x0` |
| `mov` at 0x10f0c0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f0c3 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x10f0ca | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x10f0dc | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10f0df | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10f0fe | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f105 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10f108 | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10f125 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10f128 | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10f136 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10f139 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x10f13d | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10f140 | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10f144 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f156 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10f159 | load | 4 | `[sp+8] +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10f0d2 | `_strlen 0x17466c` | `([sp+4] + [[sp+4]+32])+36` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10f0e3 | `_strncmp 0x176d68` | `[[sp+8]+4]` | `([sp+4] + [[sp+4]+32])+36` | `ret(_strlen)` | `?` | `?` | `?` |
| `call` at 0x10f0f0 | `_strlen 0x17466c` | `([sp+4] + [[sp+4]+32])+36` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10f10c | `_strstr 0x180670` | `[[sp+8]+4]` | `[sp+4]+36` | `?` | `?` | `?` | `?` |
| `call` at 0x10f11b | `_strlen 0x17466c` | `[sp+4]+36` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10f148 | `_bufWrite 0x10fc28` | `[sp+4]` | `[[sp+8]+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x10f160 | `_bufLeftAlign 0x10f908` | `[sp+8]` | `([[sp+8]+4] + ret(_bufWrite))` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x10f0c8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jne` at 0x10f0ed | `(ret(_strncmp) & ret(_strncmp)) != 0x0` | `(ret(_strncmp) & ret(_strncmp)) == 0x0` |
| `jge` at 0x10f0fc | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x10f118 | `(ret(_strstr) & ret(_strstr)) == 0x0` | `(ret(_strstr) & ret(_strstr)) != 0x0` |
| `jge` at 0x10f134 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jl` at 0x10f154 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10f186 | `eax` | `?` |

## `_bufAppendMsgs`

- `_bufAppendMsgs`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10f248 to 0x10f3f8.
  Established. The first `_bufWrite` site presents `[sp+4]`, `sp-84`, and `ret(_strlen)` as leading
  candidates (`call` at 0x10f2d7). The later site instead uses `[sp+4]`, `[[sp+8]+4]`, and unknown
  candidate two (`call` at 0x10f392). A byte store writes zero through `([[sp+8]+16] + [[sp+8]+4])
  +0x0` (`mov` at 0x10f2fe). The `_bufLeftAlign` site has unknown candidate one (`call` at
  0x10f3b5).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10f251 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10f254 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f257 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10f268 | load | 4 | `[sp+8] +0x18` | `-` |
| `mov` at 0x10f288 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f28b | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10f296 | load | 4 | `[sp+8] +0x18` | `-` |
| `mov` at 0x10f2a1 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x10f2c3 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f2c6 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x10f2d3 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f2e3 | load | 4 | `[sp+8] +0x18` | `-` |
| `mov` at 0x10f2e8 | store | 4 | `[sp+8] +0x18` | `?` |
| `mov` at 0x10f2f8 | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10f2fb | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x10f2fe | store | 1 | `([[sp+8]+16] + [[sp+8]+4]) +0x0` | `0x0` |
| `mov` at 0x10f307 | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10f319 | load | 4 | `[sp+8] +0x14` | `-` |
| `mov` at 0x10f321 | load | 4 | `[sp+8] +0x14` | `-` |
| `mov` at 0x10f324 | load | 4 | `[sp+8] +0xc` | `-` |
| `mov` at 0x10f338 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x10f340 | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10f351 | store | 4 | `[sp+8] +0x10` | `?` |
| `mov` at 0x10f354 | load | 4 | `[sp+8] +0xc` | `-` |
| `mov` at 0x10f357 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x10f35c | store | 4 | `[sp+8] +0x14` | `?` |
| `mov` at 0x10f35f | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10f362 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x10f37b | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10f382 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f385 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x10f38d | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10f3bd | load | 4 | `[sp+8] +0x10` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10f274 | `_ffscMsg 0x11f040` | `?` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x10f2b0 | `_sprintf 0x172dd8` | `sp-84` | `0x10f1e0` | `[0x1f0c04]` | `ebx` | `esi` | `edi` |
| `call` at 0x10f2b9 | `_strlen 0x17466c` | `sp-84` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10f2d7 | `_bufWrite 0x10fc28` | `[sp+4]` | `sp-84` | `ret(_strlen)` | `?` | `?` | `?` |
| `call` at 0x10f30b | `_strstr 0x180670` | `[[sp+8]+4]` | `0x10f211` | `?` | `?` | `?` | `?` |
| `call` at 0x10f346 | `_strcpy 0x18038c` | `?` | `0x10f211` | `?` | `?` | `?` | `?` |
| `call` at 0x10f392 | `_bufWrite 0x10fc28` | `[sp+4]` | `[[sp+8]+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x10f3ab | `_ffscMsg 0x11f040` | `0x10f214` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10f3b5 | `_bufLeftAlign 0x10f908` | `[sp+8]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10f25c | `(? & 0x2) != 0x0` | `(? & 0x2) == 0x0` |
| `jne` at 0x10f26d | `(? & 0x4) != 0x0` | `(? & 0x4) == 0x0` |
| `jne` at 0x10f290 | `(? & 0x8) != 0x0` | `(? & 0x8) == 0x0` |
| `je` at 0x10f29b | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `jg` at 0x10f2cb | `ret(_strlen) > [[sp+4]+20] (signed, 32-bit)` | `ret(_strlen) <= [[sp+4]+20] (signed, 32-bit)` |
| `jl` at 0x10f2e1 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jne` at 0x10f317 | `(ret(_strstr) & ret(_strstr)) != 0x0` | `(ret(_strstr) & ret(_strstr)) == 0x0` |
| `jg` at 0x10f31f | `[[sp+8]+20] > 0x1 (signed, 32-bit)` | `[[sp+8]+20] <= 0x1 (signed, 32-bit)` |
| `jg` at 0x10f38a | `? > [[sp+4]+20] (signed, 32-bit)` | `? <= [[sp+4]+20] (signed, 32-bit)` |
| `jl` at 0x10f39c | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10f3a4 | `ret(_bufWrite) == ?` | `ret(_bufWrite) != ?` |
| `jg` at 0x10f3c2 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10f3d3 | `eax` | `?` |

## `_ttyGetTxBuf`

- `_ttyGetTxBuf`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12b6e0 to 0x12b7d0.
  Established. The range branch is taken when `[sp+4] <= 0x5 (unsigned, 32-bit)` (`jbe` at
  0x12b6e9); its three indexed four-byte loads retain unknown effective addresses (`cmp` at
  0x12b70a, `mov` at 0x12b711, `mov` at 0x12b725). The out-of-range return value is `0xffffffff`
  (`ret` at 0x12b6f3), while the two in-range exits remain unknown (`ret` at 0x12b718, `ret` at
  0x12b72c). Two raw bytes, `00 90`, follow the retained decode.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12b6e3 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x12b70a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12b711 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12b725 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x12b6e9 | `[sp+4] <= 0x5 (unsigned, 32-bit)` | `[sp+4] > 0x5 (unsigned, 32-bit)` |
| `je` at 0x12b70f | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12b6f3 | `eax` | `0xffffffff` |
| `ret` at 0x12b718 | `eax` | `?` |
| `ret` at 0x12b72c | `eax` | `?` |

## `_ttyIoctl`

- `_ttyIoctl`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x19348c to 0x1934dc.
  Established. The first branch falls through when `[sp+8] == 0x4` (`jne` at 0x19349e). The indirect
  call has unknown callee, candidate zero `[[sp+4]+224]`, unknown candidate one, and candidate two
  `[sp+12]` (`call` at 0x1934b2). The later branch falls through when `ret(unknown) == 0x47` (`jne`
  at 0x1934ba), reaching `_tyIoctl` with leading candidates `[sp+4]`, unknown, and `[sp+12]` (`call`
  at 0x1934bf).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x193492 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x193495 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x193498 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1934a5 | load | 4 | `[sp+4] +0xe0` | `-` |
| `mov` at 0x1934ab | load | 4 | `[[sp+4]+224] +0x0` | `-` |
| `mov` at 0x1934b0 | load | 4 | `[[[sp+4]+224]] +0x0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1934b2 | `(unknown callee) (unknown target)` | `[[sp+4]+224]` | `?` | `[sp+12]` | `ebx` | `esi` | `edi` |
| `call` at 0x1934bf | `_tyIoctl 0x16f748` | `[sp+4]` | `?` | `[sp+12]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x19349e | `[sp+8] != 0x4` | `[sp+8] == 0x4` |
| `jne` at 0x1934ba | `ret(unknown) != 0x47` | `ret(unknown) == 0x47` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1934cd | `eax` | `?` |
