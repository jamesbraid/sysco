# MMSC buffer and TTY I/O callers in the 6.4-origin image

Read from `6.4-origin/usr/cpu/firmware/mmscfw.bin`.
Image SHA-256: `b766e559f86078526916d8bd98bbb8079294ad774cce9177928873153c6fb09b`.
The x86 a.out link address is `0x108000`. The [load map](load-map.md) explains segment mapping.
Analysed spans have exclusive ends inferred from the next text symbol.
Some include literals and padding decoded linearly after returns. Only sites reached
by known control-flow edges contribute effects to the tables.
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

[Fact sheets and complete listings](disasm/buffers-tty-64-origin-facts.txt) include all tool sections.

## `_bufRead`

- `_bufRead`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10f918 to 0x10fa18.
  Established. The `_timeoutRead` call presents `[[sp+4]]`, `[[sp+4]+8]`, `[[sp+4]+20]`, and
  `[0x1da110]` as its first four candidates (`call` at 0x10f954). Four-byte stores through `[sp+4]
  +0x8` retain `([[sp+4]+8] + ret(_timeoutRead))` (`mov` at 0x10f9bd) and through `[sp+4] +0x10`
  retain `([[sp+4]+16] + ret(_timeoutRead))` (`mov` at 0x10f9c5). The four-byte store through
  `[sp+4] +0x14` remains unknown (`mov` at 0x10f9cd). Candidate values do not establish a read
  extent.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10f91c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f91f | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f938 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10f943 | load | 4 | `abs 0x1da110` | `-` |
| `mov` at 0x10f949 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x10f94d | load | 4 | `[sp+4] +0x8` | `-` |
| `mov` at 0x10f951 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f969 | load | 4 | `ret(___errno) +0x0` | `-` |
| `mov` at 0x10f970 | load | 4 | `abs 0x1da110` | `-` |
| `mov` at 0x10f976 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x10f97a | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f998 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f9b8 | load | 4 | `[sp+4] +0x8` | `-` |
| `mov` at 0x10f9bd | store | 4 | `[sp+4] +0x8` | `([[sp+4]+8] + ret(_timeoutRead))` |
| `mov` at 0x10f9c0 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10f9c5 | store | 4 | `[sp+4] +0x10` | `([[sp+4]+16] + ret(_timeoutRead))` |
| `mov` at 0x10f9c8 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x10f9cd | store | 4 | `[sp+4] +0x14` | `?` |
| `mov` at 0x10f9d0 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x10f9dd | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10f9e2 | store | 4 | `[sp+4] +0x18` | `?` |
| `mov` at 0x10f9e7 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10f954 | `_timeoutRead 0x12a538` | `[[sp+4]]` | `[[sp+4]+8]` | `[[sp+4]+20]` | `[0x1da110]` | `ebx` | `ebp` |
| `call` at 0x10f964 | `___errno 0x16fa78` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10f982 | `_ffscMsg 0x11ebd0` | `0x10f8d4` | `[[sp+4]]` | `[[sp+4]+20]` | `[0x1da110]` | `?` | `?` |
| `call` at 0x10f9a0 | `_ffscMsgS 0x11eca0` | `0x10f8f6` | `[[sp+4]]` | `?` | `?` | `?` | `?` |
| `call` at 0x10f9d8 | `_bufHold 0x10f678` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x10f923 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x10f93d | `(? & 0x18) != 0x0` | `(? & 0x18) == 0x0` |
| `jg` at 0x10f960 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `jge` at 0x10f962 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x10f96e | `[ret(___errno)] != 0x4` | `[ret(___errno)] == 0x4` |
| `jg` at 0x10f9d5 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10f9ed | `eax` | `?` |

## `_bufWrite`

- `_bufWrite`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10fb78 to 0x10feb0.
  Established. The `_timeoutWrite` site presents `[[sp+4]]`, `[sp+8]`, `[sp+12]`, and `[0x1da114]`
  as leading candidates (`call` at 0x10fc99). The following four-byte store at `sp -0x8` retains
  `ret(_timeoutWrite)` (`mov` at 0x10fca1). A later four-byte store through `[sp+4] +0x20` writes
  `0x0` (`mov` at 0x10fe3b). Stores through `[sp+4] +0x18` include unknown values (`mov` at
  0x10fdac, `mov` at 0x10fdbf, `mov` at 0x10fe47). No buffer-state meanings are assigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10fb81 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x10fb84 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10fb98 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x10fb9f | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10fba3 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10fba7 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x10fbb3 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10fbba | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10fbc1 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fbcb | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x10fbce | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10fbd5 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x10fbd9 | load | 4 | `[sp+4] +0x8` | `-` |
| `mov` at 0x10fbdd | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10fbe9 | load | 4 | `[sp+4] +0x8` | `-` |
| `mov` at 0x10fbee | store | 4 | `[sp+4] +0x8` | `?` |
| `mov` at 0x10fbf1 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10fbf6 | store | 4 | `[sp+4] +0x10` | `?` |
| `mov` at 0x10fbf9 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x10fbfe | store | 4 | `[sp+4] +0x14` | `?` |
| `mov` at 0x10fc01 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10fc08 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fc0e | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10fc11 | store | 4 | `sp -0x20` | `[[sp+4]+40]` |
| `mov` at 0x10fc14 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fc19 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fc1e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10fc26 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fc2b | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x10fc2e | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fc3e | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x10fc41 | load | 4 | `sp -0x20` | `-` |
| `mov` at 0x10fc44 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10fc49 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10fc4c | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10fc53 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10fc56 | load | 4 | `[[sp+4]+44] +0x0` | `-` |
| `mov` at 0x10fc59 | store | 4 | `[[sp+4]+44] +0x0` | `?` |
| `mov` at 0x10fc5b | load | 4 | `[[sp+4]+44] +0x0` | `-` |
| `test` at 0x10fc5d | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x10fc75 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x10fc88 | load | 4 | `abs 0x1da114` | `-` |
| `mov` at 0x10fc8e | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10fc92 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10fc96 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fca1 | store | 4 | `sp -0x8` | `ret(_timeoutWrite)` |
| `mov` at 0x10fcad | load | 4 | `ret(___errno) +0x0` | `-` |
| `mov` at 0x10fcb4 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fcd8 | store | 4 | `sp -0x8` | `0x0` |
| `mov` at 0x10fcdf | load | 4 | `sp +0xc` | `-` |
| `cmp` at 0x10fce2 | load | 4 | `sp -0x8` | `-` |
| `sub` at 0x10fced | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10fcf0 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10fcf7 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10fcfb | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x10fcff | load | 4 | `sp +0x8` | `-` |
| `add` at 0x10fd02 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10fd0e | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x10fd13 | store | 4 | `[sp+4] +0x8` | `?` |
| `mov` at 0x10fd16 | store | 4 | `[sp+4] +0x10` | `?` |
| `mov` at 0x10fd19 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10fd1e | store | 4 | `[sp+4] +0x14` | `?` |
| `mov` at 0x10fd21 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10fd28 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10fd2b | store | 4 | `sp -0x20` | `[[sp+4]+40]` |
| `mov` at 0x10fd2e | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fd33 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fd38 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10fd40 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fd45 | store | 4 | `sp -0x14` | `?` |
| `mov` at 0x10fd48 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fd58 | load | 4 | `sp -0x14` | `-` |
| `mov` at 0x10fd5b | load | 4 | `sp -0x20` | `-` |
| `mov` at 0x10fd5e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10fd63 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10fd66 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10fd6d | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10fd70 | load | 4 | `[[sp+4]+44] +0x0` | `-` |
| `mov` at 0x10fd73 | store | 4 | `[[sp+4]+44] +0x0` | `?` |
| `mov` at 0x10fd75 | load | 4 | `[[sp+4]+44] +0x0` | `-` |
| `test` at 0x10fd77 | load | 1 | `abs 0x1da698` | `-` |
| `add` at 0x10fd8e | load | 4 | `sp -0x8` | `-` |
| `add` at 0x10fd8e | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x10fd91 | load | 4 | `sp +0xc` | `-` |
| `cmp` at 0x10fd94 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10fda7 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10fdac | store | 4 | `[sp+4] +0x18` | `?` |
| `mov` at 0x10fdaf | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10fdba | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10fdbf | store | 4 | `[sp+4] +0x18` | `?` |
| `mov` at 0x10fdc2 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x10fdc9 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10fdcd | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x10fdd5 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10fde5 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x10fdf6 | load | 4 | `[sp+4] +0x20` | `-` |
| `add` at 0x10fdf9 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10fdfc | store | 4 | `[sp+4] +0x20` | `([[sp+4]+32] + [sp-8])` |
| `mov` at 0x10fe0b | store | 4 | `sp -0x20` | `[sp+4]+36` |
| `mov` at 0x10fe22 | load | 4 | `sp +0x8` | `-` |
| `add` at 0x10fe25 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10fe3b | store | 4 | `[sp+4] +0x20` | `0x0` |
| `mov` at 0x10fe42 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10fe47 | store | 4 | `[sp+4] +0x18` | `?` |
| `mov` at 0x10fe58 | store | 4 | `[sp+4] +0x20` | `?` |
| `mov` at 0x10fe68 | store | 4 | `sp -0x1c` | `([sp+8] + [sp-8])` |
| `mov` at 0x10fe6b | load | 4 | `sp -0x20` | `-` |
| `mov` at 0x10fe7e | store | 4 | `sp -0x20` | `[sp-32]` |
| `mov` at 0x10fe85 | load | 4 | `sp -0x20` | `-` |
| `mov` at 0x10fe89 | load | 4 | `sp -0x1c` | `-` |
| `mov` at 0x10fea0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10fbab | `_logWrite 0x124de0` | `[[sp+4]+48]` | `[sp+8]` | `[sp+12]` | `ebx` | `esi` | `edi` |
| `call` at 0x10fbe1 | `_bcopy 0x1747fc` | `[sp+8]` | `[[sp+4]+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x10fc6d | `_ffscMsg 0x11ebd0` | `0x10faf8` | `?` | `[sp+4]` | `?` | `?` | `?` |
| `call` at 0x10fc99 | `_timeoutWrite 0x12a788` | `[[sp+4]]` | `[sp+8]` | `[sp+12]` | `[0x1da114]` | `?` | `?` |
| `call` at 0x10fca8 | `___errno 0x16fa78` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10fcbc | `_ffscMsgS 0x11eca0` | `0x10fb1e` | `[[sp+4]]` | `?` | `?` | `?` | `?` |
| `call` at 0x10fd06 | `_bcopy 0x1747fc` | `?` | `[[sp+4]+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x10fd86 | `_ffscMsg 0x11ebd0` | `0x10fb38` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10fd9f | `_ffscMsg 0x11ebd0` | `0x10fb55` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x10fdd9 | `_strncmp 0x1749b8` | `[sp+8]` | `([sp+4] + [[sp+4]+32])+36` | `[sp-8]` | `?` | `?` | `?` |
| `call` at 0x10fded | `_strlen 0x172f14` | `([sp+4] + [[sp+4]+32])+36` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10fe0f | `_strlen 0x172f14` | `[sp+4]+36` | `?` | `?` | `?` | `[sp+4]+36` | `?` |
| `call` at 0x10fe1a | `_strlen 0x172f14` | `[sp+4]+36` | `[sp+4]+36` | `ret(_strlen)` | `?` | `?` | `?` |
| `call` at 0x10fe2f | `_strncmp 0x1749b8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10fe6f | `_strlen 0x172f14` | `[sp-32]` | `?` | `?` | `?` | `?` | `([sp+8] + [sp-8])` |
| `call` at 0x10fe8f | `_strncmp 0x1749b8` | `?` | `[sp-32]` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x10fb88 | `[sp+12] > 0x0 (signed, 32-bit)` | `[sp+12] <= 0x0 (signed, 32-bit)` |
| `je` at 0x10fb9d | `([[sp+4]+48] & [[sp+4]+48]) == 0x0` | `([[sp+4]+48] & [[sp+4]+48]) != 0x0` |
| `jg` at 0x10fbb8 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `jne` at 0x10fbbf | `(? & 0x8) != 0x0` | `(? & 0x8) == 0x0` |
| `jge` at 0x10fbc5 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jle` at 0x10fbd3 | `[sp+12] <= [[sp+4]+20] (signed, 32-bit)` | `[sp+12] > [[sp+4]+20] (signed, 32-bit)` |
| `je` at 0x10fc06 | `([[sp+4]+40] & [[sp+4]+40]) == 0x0` | `([[sp+4]+40] & [[sp+4]+40]) != 0x0` |
| `jl` at 0x10fc0c | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jb` at 0x10fc24 | `C == 1` | `not (C == 1)` |
| `je` at 0x10fc51 | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |
| `je` at 0x10fc64 | `(([0x1da698]:1 & 0xff) & 0x8) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x8) != 0x0` |
| `jge` at 0x10fca6 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x10fcb2 | `[ret(___errno)] == 0x4` | `[ret(___errno)] != 0x4` |
| `je` at 0x10fce5 | `Z == 1` | `not (Z == 1)` |
| `jle` at 0x10fcf5 | `? <= [[sp+4]+12] (signed, 32-bit)` | `? > [[sp+4]+12] (signed, 32-bit)` |
| `je` at 0x10fd26 | `([[sp+4]+40] & [[sp+4]+40]) == 0x0` | `([[sp+4]+40] & [[sp+4]+40]) != 0x0` |
| `jb` at 0x10fd3e | `C == 1` | `not (C == 1)` |
| `je` at 0x10fd6b | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |
| `je` at 0x10fd7e | `(([0x1da698]:1 & 0xff) & 0x8) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x8) != 0x0` |
| `jge` at 0x10fd97 | `[sp-8] >= [sp+12] (signed, 32-bit)` | `[sp-8] < [sp+12] (signed, 32-bit)` |
| `je` at 0x10fdb4 | `(? & 0x40) == 0x0` | `(? & 0x40) != 0x0` |
| `jle` at 0x10fdc7 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jne` at 0x10fde3 | `(ret(_strncmp) & ret(_strncmp)) != 0x0` | `(ret(_strncmp) & ret(_strncmp)) == 0x0` |
| `je` at 0x10fdf4 | `[sp-8] == ret(_strlen)` | `[sp-8] != ret(_strlen)` |
| `jne` at 0x10fe39 | `(ret(_strncmp) & ret(_strncmp)) != 0x0` | `(ret(_strncmp) & ret(_strncmp)) == 0x0` |
| `jle` at 0x10fe7c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `je` at 0x10fe99 | `(ret(_strncmp) & ret(_strncmp)) == 0x0` | `(ret(_strncmp) & ret(_strncmp)) != 0x0` |
| `jg` at 0x10fe9e | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10feac | `eax` | `?` |

## `_bufFlush`

- `_bufFlush`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10f558 to 0x10f678.
  Established. The `_timeoutWrite` call has leading candidates `[[sp+4]]`, `[[sp+4]+4]`,
  `[[sp+4]+16]`, and `[0x1da114]` (`call` at 0x10f575). A four-byte store through `[[sp+4]+44] +0x0`
  retains an unknown value (`mov` at 0x10f632). Later four-byte stores through `[sp+4] +0x8` (`mov`
  at 0x10f651), `+0x10` (`mov` at 0x10f659), and `+0x14` (`mov` at 0x10f661) also remain unknown. No
  flushed byte count follows from the name.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10f561 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f564 | load | 4 | `abs 0x1da114` | `-` |
| `mov` at 0x10f56a | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10f56e | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x10f572 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f588 | load | 4 | `ret(___errno) +0x0` | `-` |
| `mov` at 0x10f5aa | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10f5b5 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10f5bb | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x10f5c1 | load | 4 | `[sp+4] +0x4` | `-` |
| `test` at 0x10f5cd | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x10f5d6 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f5d9 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10f5f8 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10f5ff | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10f602 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f607 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x10f60a | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f61a | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10f61d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10f622 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10f625 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10f62c | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10f62f | load | 4 | `[[sp+4]+44] +0x0` | `-` |
| `mov` at 0x10f632 | store | 4 | `[[sp+4]+44] +0x0` | `?` |
| `mov` at 0x10f634 | load | 4 | `[[sp+4]+44] +0x0` | `-` |
| `test` at 0x10f636 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x10f63f | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f64c | load | 4 | `[sp+4] +0x8` | `-` |
| `mov` at 0x10f651 | store | 4 | `[sp+4] +0x8` | `?` |
| `mov` at 0x10f654 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10f659 | store | 4 | `[sp+4] +0x10` | `?` |
| `mov` at 0x10f65c | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x10f661 | store | 4 | `[sp+4] +0x14` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10f575 | `_timeoutWrite 0x12a788` | `[[sp+4]]` | `[[sp+4]+4]` | `[[sp+4]+16]` | `[0x1da114]` | `ebx` | `esi` |
| `call` at 0x10f583 | `___errno 0x16fa78` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10f594 | `_ffscMsgS 0x11eca0` | `0x10f4f9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10f5c5 | `_memmove 0x17dde0` | `[[sp+4]+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10f5e3 | `_ffscMsg 0x11ebd0` | `0x10f51c` | `?` | `[[sp+4]+16]` | `[[sp+4]]` | `?` | `?` |
| `call` at 0x10f647 | `_ffscMsg 0x11ebd0` | `0x10f539` | `[[sp+4]]` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x10f581 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x10f58d | `[ret(___errno)] == 0x4` | `[ret(___errno)] != 0x4` |
| `je` at 0x10f5af | `Z == 1` | `not (Z == 1)` |
| `jle` at 0x10f5b3 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `je` at 0x10f5d4 | `(([0x1da698]:1 & 0xff) & 0x8) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x8) != 0x0` |
| `je` at 0x10f5fd | `([[sp+4]+40] & [[sp+4]+40]) == 0x0` | `([[sp+4]+40] & [[sp+4]+40]) != 0x0` |
| `je` at 0x10f62a | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |
| `je` at 0x10f63d | `(([0x1da698]:1 & 0xff) & 0x8) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10f66f | `eax` | `?` |

## `_bufHold`

- `_bufHold`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10f678 to 0x10f748.
  Established. Four-byte stores through `[sp+4] +0x18` (`mov` at 0x10f689) and `[sp+4] +0x1c` (`mov`
  at 0x10f690) retain unknown values. The branch is taken when `([[sp+4]+40] & [[sp+4]+40]) == 0x0`
  (`je` at 0x10f6a1). Another four-byte store through `[[sp+4]+44] +0x0` has an unknown value (`mov`
  at 0x10f6e0). The common exit reports `0x0` (`ret` at 0x10f6ef), without proving a hold count or
  flow-control effect.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10f681 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f684 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10f689 | store | 4 | `[sp+4] +0x18` | `?` |
| `mov` at 0x10f68c | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x10f690 | store | 4 | `[sp+4] +0x1c` | `?` |
| `mov` at 0x10f693 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x10f696 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f69c | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10f6a3 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10f6a6 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f6ab | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f6b0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10f6b8 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f6bd | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f6cb | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10f6d0 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10f6d3 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10f6da | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10f6dd | load | 4 | `[[sp+4]+44] +0x0` | `-` |
| `mov` at 0x10f6e0 | store | 4 | `[[sp+4]+44] +0x0` | `?` |
| `mov` at 0x10f6e2 | load | 4 | `[[sp+4]+44] +0x0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jl` at 0x10f69a | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10f6a1 | `([[sp+4]+40] & [[sp+4]+40]) == 0x0` | `([[sp+4]+40] & [[sp+4]+40]) != 0x0` |
| `jae` at 0x10f6b6 | `C == 0` | `not (C == 0)` |
| `je` at 0x10f6d8 | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10f6ef | `eax` | `0x0` |

## `_bufRelease`

- `_bufRelease`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10fa18 to 0x10fb78.
  Established. The `_bufFlush` call has candidate zero `[sp+4]` (`call` at 0x10fa78); its result is
  stored as four bytes at `sp -0x10` (`mov` at 0x10fa7d). The stores through `[sp+4] +0x1c` (`mov`
  at 0x10fa4c) and `[sp+4] +0x18` (`mov` at 0x10fa6d) remain unknown. A later four-byte store
  through `[[sp+4]+44] +0x0` also retains an unknown value (`mov` at 0x10fae7). Release and counter
  roles remain unassigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10fa21 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10fa24 | store | 4 | `sp -0x10` | `0x0` |
| `mov` at 0x10fa2b | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x10fa48 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x10fa4c | store | 4 | `[sp+4] +0x1c` | `?` |
| `mov` at 0x10fa4f | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x10fa52 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x10fa68 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10fa6d | store | 4 | `[sp+4] +0x18` | `?` |
| `mov` at 0x10fa70 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10fa7d | store | 4 | `sp -0x10` | `ret(_bufFlush)` |
| `mov` at 0x10fa80 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fa86 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10fa8d | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10fa90 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fa95 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fa9a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10faa2 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10faa9 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10fab0 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10fab7 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10fabc | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x10fabf | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10facf | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10fad2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10fad7 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10fada | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10fae1 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10fae4 | load | 4 | `[[sp+4]+44] +0x0` | `-` |
| `mov` at 0x10fae7 | store | 4 | `[[sp+4]+44] +0x0` | `?` |
| `mov` at 0x10fae9 | load | 4 | `[[sp+4]+44] +0x0` | `-` |
| `mov` at 0x10faeb | load | 4 | `sp -0x10` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10fa37 | `_ffscMsg 0x11ebd0` | `0x10f9ee` | `ebx` | `esi` | `edi` | `0x0` | `?` |
| `call` at 0x10fa78 | `_bufFlush 0x10f558` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x10fa30 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `jle` at 0x10fa57 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `je` at 0x10fa75 | `(? & 0x2) == 0x0` | `(? & 0x2) != 0x0` |
| `jl` at 0x10fa84 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10fa8b | `([[sp+4]+40] & [[sp+4]+40]) == 0x0` | `([[sp+4]+40] & [[sp+4]+40]) != 0x0` |
| `jb` at 0x10faa0 | `C == 1` | `not (C == 1)` |
| `jne` at 0x10faa7 | `(? & 0x1) != 0x0` | `(? & 0x1) == 0x0` |
| `je` at 0x10faae | `(? & 0x2) == 0x0` | `(? & 0x2) != 0x0` |
| `jle` at 0x10fab5 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `je` at 0x10fadf | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10faf7 | `eax` | `?` |

## `_bufAttachFD`

- `_bufAttachFD`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10f378 to 0x10f418.
  Established. The four-byte store through `[sp+4] +0x0` writes `[sp+8]` (`mov` at 0x10f396). An
  earlier `_bufDetachFD` site presents candidate zero `[sp+4]` (`call` at 0x10f38e). The later
  four-byte indexed store has an unresolved effective address and value (`mov` at 0x10f3f6). The
  four-byte store through `[[sp+4]+44] +0x0` is also unknown (`mov` at 0x10f406). No
  descriptor-registration effect is inferred from the call name.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10f381 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f384 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10f387 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f396 | store | 4 | `[sp+4] +0x0` | `[sp+8]` |
| `mov` at 0x10f3b8 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10f3bf | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10f3c6 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10f3cd | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10f3d4 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10f3db | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10f3de | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f3e3 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f3f1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10f3f6 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10f3f9 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10f400 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10f403 | load | 4 | `[[sp+4]+44] +0x0` | `-` |
| `mov` at 0x10f406 | store | 4 | `[[sp+4]+44] +0x0` | `?` |
| `mov` at 0x10f408 | load | 4 | `[[sp+4]+44] +0x0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10f38e | `_bufDetachFD 0x10f478` | `[sp+4]` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x10f3a2 | `_ffscMsg 0x11ebd0` | `0x10f354` | `[sp+8]` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jl` at 0x10f38b | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jge` at 0x10f39a | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x10f3bd | `(? & 0x8) != 0x0` | `(? & 0x8) == 0x0` |
| `jne` at 0x10f3c4 | `(? & 0x1) != 0x0` | `(? & 0x1) == 0x0` |
| `je` at 0x10f3cb | `(? & 0x2) == 0x0` | `(? & 0x2) != 0x0` |
| `jle` at 0x10f3d2 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `je` at 0x10f3d9 | `([[sp+4]+40] & [[sp+4]+40]) == 0x0` | `([[sp+4]+40] & [[sp+4]+40]) != 0x0` |
| `je` at 0x10f3fe | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10f413 | `eax` | `?` |

## `_bufDetachFD`

- `_bufDetachFD`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10f478 to 0x10f4e8.
  Established. The four-byte store through `[sp+4] +0x0` writes `0xffffffff` (`mov` at 0x10f4d2).
  The prior indexed four-byte store has an unknown target and value (`mov` at 0x10f4be). Another
  four-byte store through `[[sp+4]+44] +0x0` retains unknown (`mov` at 0x10f4ce). The deciding
  branch for that last access is taken when `([[sp+4]+44] & [[sp+4]+44]) == 0x0` (`je` at 0x10f4c6),
  skipping it. Field meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10f481 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f484 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f48a | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10f491 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10f494 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f499 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f49e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10f4a6 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f4ab | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10f4b9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10f4be | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10f4c1 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10f4c8 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10f4cb | load | 4 | `[[sp+4]+44] +0x0` | `-` |
| `mov` at 0x10f4ce | store | 4 | `[[sp+4]+44] +0x0` | `?` |
| `mov` at 0x10f4d0 | load | 4 | `[[sp+4]+44] +0x0` | `-` |
| `mov` at 0x10f4d2 | store | 4 | `[sp+4] +0x0` | `0xffffffff` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jl` at 0x10f488 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10f48f | `([[sp+4]+40] & [[sp+4]+40]) == 0x0` | `([[sp+4]+40] & [[sp+4]+40]) != 0x0` |
| `jae` at 0x10f4a4 | `C == 0` | `not (C == 0)` |
| `je` at 0x10f4c6 | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10f4e1 | `eax` | `?` |

## `_bufInit`

- `_bufInit`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10f748 to 0x10f838.
  Established. The `_malloc` sites present candidate zero `0x34` (`call` at 0x10f756) and `[sp+4]+1`
  (`call` at 0x10f77c). A four-byte store through `ret(_malloc) +0x0` writes `0xffffffff` (`mov` at
  0x10f78e). Other four-byte stores through that printed origin write `0x0` at `+0x10` (`mov` at
  0x10f79d), `+0x20` (`mov` at 0x10f7c1), and `+0x30` (`mov` at 0x10f7d6). The repeated result label
  does not prove that the two allocations have equal addresses.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10f74e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f751 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10f784 | store | 4 | `ret(_malloc) +0x4` | `ret(_malloc)` |
| `mov` at 0x10f787 | load | 4 | `ret(_malloc) +0x4` | `-` |
| `mov` at 0x10f78e | store | 4 | `ret(_malloc) +0x0` | `0xffffffff` |
| `mov` at 0x10f794 | load | 4 | `ret(_malloc) +0x4` | `-` |
| `mov` at 0x10f797 | store | 4 | `ret(_malloc) +0x8` | `[ret(_malloc)+4]` |
| `mov` at 0x10f79a | store | 4 | `ret(_malloc) +0xc` | `[sp+4]` |
| `mov` at 0x10f79d | store | 4 | `ret(_malloc) +0x10` | `0x0` |
| `mov` at 0x10f7a4 | store | 4 | `ret(_malloc) +0x14` | `[sp+4]` |
| `mov` at 0x10f7a7 | store | 4 | `ret(_malloc) +0x18` | `[sp+8]` |
| `mov` at 0x10f7aa | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10f7ad | store | 4 | `ret(_malloc) +0x28` | `[sp+12]` |
| `mov` at 0x10f7b0 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x10f7b3 | store | 4 | `ret(_malloc) +0x2c` | `[sp+16]` |
| `mov` at 0x10f7be | store | 4 | `ret(_malloc) +0x1c` | `?` |
| `mov` at 0x10f7c1 | store | 4 | `ret(_malloc) +0x20` | `0x0` |
| `mov` at 0x10f7d6 | store | 4 | `ret(_malloc) +0x30` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10f756 | `_malloc 0x17c878` | `0x34` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x10f77c | `_malloc 0x17c878` | `[sp+4]+1` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10f7d1 | `_strcpy 0x17dfdc` | `ret(_malloc)+36` | `0x10f191` | `?` | `?` | `?` | `?` |
| `call` at 0x10f7ed | `_ffscMsgS 0x11eca0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10f762 | `(ret(_malloc) & ret(_malloc)) != 0x0` | `(ret(_malloc) & ret(_malloc)) == 0x0` |
| `je` at 0x10f78c | `([ret(_malloc)+4] & [ret(_malloc)+4]) == 0x0` | `([ret(_malloc)+4] & [ret(_malloc)+4]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10f7fd | `eax` | `?` |

## `_bufClear`

- `_bufClear`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10f438 to 0x10f478.
  Established. A four-byte store through `[sp+4] +0x10` writes `0x0` (`mov` at 0x10f43e). The
  four-byte store through `[sp+4] +0x14` retains `[[sp+4]+12]` (`mov` at 0x10f448), and the store
  through `[sp+4] +0x8` retains `[[sp+4]+4]` (`mov` at 0x10f44e). The `_bufRelease` site presents
  candidate zero `[sp+4]` (`call` at 0x10f461). These offsets do not establish field roles.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10f43b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f43e | store | 4 | `[sp+4] +0x10` | `0x0` |
| `mov` at 0x10f445 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10f448 | store | 4 | `[sp+4] +0x14` | `[[sp+4]+12]` |
| `mov` at 0x10f44b | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x10f44e | store | 4 | `[sp+4] +0x8` | `[[sp+4]+4]` |
| `mov` at 0x10f451 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10f458 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10f45d | store | 4 | `[sp+4] +0x18` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10f461 | `_bufRelease 0x10fa18` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10f456 | `(? & 0x10) == 0x0` | `(? & 0x10) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10f469 | `eax` | `?` |

## `_bufLeftAlign`

- `_bufLeftAlign`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10f838 to 0x10f918.
  Established. The first bounds branch is taken when `[sp+8] < [[sp+4]+4] (unsigned, 32-bit)` (`jb`
  at 0x10f847). The next is taken when `[sp+8] <= ([[sp+4]+4] + [[sp+4]+16]) (unsigned, 32-bit)`
  (`jbe` at 0x10f853). The `_memmove` site presents `[[sp+4]+4]`, `[sp+8]`, and `[[sp+4]+16]` as its
  first three candidates (`call` at 0x10f8a9). The candidates do not prove a move extent or a valid
  buffer interval.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10f83c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f83f | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10f842 | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x10f849 | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x10f84c | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10f878 | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x10f881 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10f886 | store | 4 | `[sp+4] +0x10` | `?` |
| `mov` at 0x10f889 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x10f88e | store | 4 | `[sp+4] +0x14` | `?` |
| `mov` at 0x10f891 | load | 4 | `[sp+4] +0x8` | `-` |
| `mov` at 0x10f896 | store | 4 | `[sp+4] +0x8` | `?` |
| `mov` at 0x10f899 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10f8a0 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10f8a5 | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x10f8b1 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x10f8b8 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10f8c5 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10f8ca | store | 4 | `[sp+4] +0x18` | `?` |
| `mov` at 0x10f8cd | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10f85a | `_ffscMsg 0x11ebd0` | `0x10f7fe` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x10f863 | `_bufClear 0x10f438` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10f8a9 | `_memmove 0x17dde0` | `[[sp+4]+4]` | `[sp+8]` | `[[sp+4]+16]` | `?` | `?` | `?` |
| `call` at 0x10f8c0 | `_bufRelease 0x10fa18` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jb` at 0x10f847 | `[sp+8] < [[sp+4]+4] (unsigned, 32-bit)` | `[sp+8] >= [[sp+4]+4] (unsigned, 32-bit)` |
| `jbe` at 0x10f853 | `[sp+8] <= ([[sp+4]+4] + [[sp+4]+16]) (unsigned, 32-bit)` | `[sp+8] > ([[sp+4]+4] + [[sp+4]+16]) (unsigned, 32-bit)` |
| `je` at 0x10f87f | `Z == 1` | `not (Z == 1)` |
| `jle` at 0x10f89e | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jle` at 0x10f8b6 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `je` at 0x10f8bd | `(? & 0x10) == 0x0` | `(? & 0x10) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10f8d3 | `eax` | `?` |

## `_bufAppendBuf`

- `_bufAppendBuf`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10efd8 to 0x10f028.
  Established. The `_bufWrite` call presents `[sp+4]`, `[[sp+8]+4]`, and `[[sp+8]+16]` as leading
  candidates (`call` at 0x10efeb). The `_bufLeftAlign` site has leading candidates `[sp+8]` and
  `([[sp+8]+4] + ret(_bufWrite))` (`call` at 0x10f000). The exit value remains unknown (`ret` at
  0x10f023). An append effect or transferred length is not supplied from these names.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10efdc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10efdf | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10efe2 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x10efe6 | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10eff9 | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10f01d | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10efeb | `_bufWrite 0x10fb78` | `[sp+4]` | `[[sp+8]+4]` | `[[sp+8]+16]` | `ebx` | `ebp` | `?` |
| `call` at 0x10f000 | `_bufLeftAlign 0x10f838` | `[sp+8]` | `([[sp+8]+4] + ret(_bufWrite))` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jl` at 0x10eff7 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10f023 | `eax` | `?` |

## `_bufAppendBufToEOL`

- `_bufAppendBufToEOL`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10f028 to 0x10f1c8.
  Established. The byte store at `([[sp+8]+16] + [[sp+8]+4]) +0x0` writes `0x0` (`mov` at 0x10f03c).
  The `_strstr` site presents `[[sp+8]+4]` and `[sp+4]+36` as its first two candidates (`call` at
  0x10f08c). The later `_bufWrite` call has leading candidates `[sp+4]`, `[[sp+8]+4]`, and unknown
  (`call` at 0x10f0c8). No end-of-line grammar or transfer size is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10f033 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10f036 | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10f039 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x10f03c | store | 1 | `([[sp+8]+16] + [[sp+8]+4]) +0x0` | `0x0` |
| `mov` at 0x10f040 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f043 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x10f04a | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x10f05c | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10f05f | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10f07e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f085 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10f088 | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10f0a5 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10f0a8 | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10f0b6 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10f0b9 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x10f0bd | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10f0c0 | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10f0c4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f0d6 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10f0d9 | load | 4 | `[sp+8] +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10f052 | `_strlen 0x172f14` | `([sp+4] + [[sp+4]+32])+36` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10f063 | `_strncmp 0x1749b8` | `[[sp+8]+4]` | `([sp+4] + [[sp+4]+32])+36` | `ret(_strlen)` | `?` | `?` | `?` |
| `call` at 0x10f070 | `_strlen 0x172f14` | `([sp+4] + [[sp+4]+32])+36` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10f08c | `_strstr 0x17e2c0` | `[[sp+8]+4]` | `[sp+4]+36` | `?` | `?` | `?` | `?` |
| `call` at 0x10f09b | `_strlen 0x172f14` | `[sp+4]+36` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10f0c8 | `_bufWrite 0x10fb78` | `[sp+4]` | `[[sp+8]+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x10f0e0 | `_bufLeftAlign 0x10f838` | `[sp+8]` | `([[sp+8]+4] + ret(_bufWrite))` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x10f048 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jne` at 0x10f06d | `(ret(_strncmp) & ret(_strncmp)) != 0x0` | `(ret(_strncmp) & ret(_strncmp)) == 0x0` |
| `jge` at 0x10f07c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x10f098 | `(ret(_strstr) & ret(_strstr)) == 0x0` | `(ret(_strstr) & ret(_strstr)) != 0x0` |
| `jge` at 0x10f0b4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jl` at 0x10f0d4 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10f106 | `eax` | `?` |

## `_bufAppendMsgs`

- `_bufAppendMsgs`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10f1c8 to 0x10f378.
  Established. The byte store at `([[sp+8]+16] + [[sp+8]+4]) +0x0` writes `0x0` (`mov` at 0x10f27e).
  The first `_bufWrite` site presents `[sp+4]`, `sp-84`, and `ret(_strlen)` as leading candidates
  (`call` at 0x10f257). The second presents `[sp+4]`, `[[sp+8]+4]`, and unknown (`call` at
  0x10f312). The four-byte store through `[sp+8] +0x18` retains an unknown value (`mov` at
  0x10f268). Message boundaries remain unassigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10f1d1 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10f1d4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f1d7 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10f1e8 | load | 4 | `[sp+8] +0x18` | `-` |
| `mov` at 0x10f208 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f20b | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10f216 | load | 4 | `[sp+8] +0x18` | `-` |
| `mov` at 0x10f221 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x10f243 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f246 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x10f253 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f263 | load | 4 | `[sp+8] +0x18` | `-` |
| `mov` at 0x10f268 | store | 4 | `[sp+8] +0x18` | `?` |
| `mov` at 0x10f278 | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10f27b | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x10f27e | store | 1 | `([[sp+8]+16] + [[sp+8]+4]) +0x0` | `0x0` |
| `mov` at 0x10f287 | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10f299 | load | 4 | `[sp+8] +0x14` | `-` |
| `mov` at 0x10f2a1 | load | 4 | `[sp+8] +0x14` | `-` |
| `mov` at 0x10f2a4 | load | 4 | `[sp+8] +0xc` | `-` |
| `mov` at 0x10f2b8 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x10f2c0 | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10f2d1 | store | 4 | `[sp+8] +0x10` | `?` |
| `mov` at 0x10f2d4 | load | 4 | `[sp+8] +0xc` | `-` |
| `mov` at 0x10f2d7 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x10f2dc | store | 4 | `[sp+8] +0x14` | `?` |
| `mov` at 0x10f2df | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10f2e2 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x10f2fb | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10f302 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f305 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x10f30d | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10f33d | load | 4 | `[sp+8] +0x10` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10f1f4 | `_ffscMsg 0x11ebd0` | `?` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x10f230 | `_sprintf 0x171680` | `sp-84` | `0x10f160` | `[0x1ee564]` | `ebx` | `esi` | `edi` |
| `call` at 0x10f239 | `_strlen 0x172f14` | `sp-84` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10f257 | `_bufWrite 0x10fb78` | `[sp+4]` | `sp-84` | `ret(_strlen)` | `?` | `?` | `?` |
| `call` at 0x10f28b | `_strstr 0x17e2c0` | `[[sp+8]+4]` | `0x10f191` | `?` | `?` | `?` | `?` |
| `call` at 0x10f2c6 | `_strcpy 0x17dfdc` | `?` | `0x10f191` | `?` | `?` | `?` | `?` |
| `call` at 0x10f312 | `_bufWrite 0x10fb78` | `[sp+4]` | `[[sp+8]+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x10f32b | `_ffscMsg 0x11ebd0` | `0x10f194` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10f335 | `_bufLeftAlign 0x10f838` | `[sp+8]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10f1dc | `(? & 0x2) != 0x0` | `(? & 0x2) == 0x0` |
| `jne` at 0x10f1ed | `(? & 0x4) != 0x0` | `(? & 0x4) == 0x0` |
| `jne` at 0x10f210 | `(? & 0x8) != 0x0` | `(? & 0x8) == 0x0` |
| `je` at 0x10f21b | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `jg` at 0x10f24b | `ret(_strlen) > [[sp+4]+20] (signed, 32-bit)` | `ret(_strlen) <= [[sp+4]+20] (signed, 32-bit)` |
| `jl` at 0x10f261 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jne` at 0x10f297 | `(ret(_strstr) & ret(_strstr)) != 0x0` | `(ret(_strstr) & ret(_strstr)) == 0x0` |
| `jg` at 0x10f29f | `[[sp+8]+20] > 0x1 (signed, 32-bit)` | `[[sp+8]+20] <= 0x1 (signed, 32-bit)` |
| `jg` at 0x10f30a | `? > [[sp+4]+20] (signed, 32-bit)` | `? <= [[sp+4]+20] (signed, 32-bit)` |
| `jl` at 0x10f31c | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10f324 | `ret(_bufWrite) == ?` | `ret(_bufWrite) != ?` |
| `jg` at 0x10f342 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10f353 | `eax` | `?` |

## `_ttyDrv`

- `_ttyDrv`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x18fca8 to 0x18fd08.
  Established. The `_iosDrvInstall` site presents `0x18fdd8`, `0x0`, `0x18fdd8`, `0x0`, `0x16e720`,
  and `0x16e640` as its six printed candidates (`call` at 0x18fce5); its result is stored as four
  bytes at absolute `0x1d95f8` (`mov` at 0x18fcea). The following branch is taken when `[0x1d95f8]
  != 0xffffffff` (`jne` at 0x18fcf8). The candidate addresses do not establish calls through them or
  a successful driver installation.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x18fcab | load | 4 | `abs 0x1d95f8` | `-` |
| `mov` at 0x18fcea | store | 4 | `abs 0x1d95f8` | `ret(_iosDrvInstall)` |
| `cmp` at 0x18fcf1 | load | 4 | `abs 0x1d95f8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x18fce5 | `_iosDrvInstall 0x16f050` | `0x18fdd8` | `0x0` | `0x18fdd8` | `0x0` | `0x16e720` | `0x16e640` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x18fcb2 | `[0x1d95f8] <= 0x0 (signed, 32-bit)` | `[0x1d95f8] > 0x0 (signed, 32-bit)` |
| `jne` at 0x18fcf8 | `[0x1d95f8] != 0xffffffff` | `[0x1d95f8] == 0xffffffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x18fcb9 | `eax` | `0x0` |
| `ret` at 0x18fd02 | `eax` | `?` |

## `_ttyIoctl`

- `_ttyIoctl`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x18fde8 to 0x18fe38.
  Established. The initial branch is taken when `[sp+8] != 0x4` (`jne` at 0x18fdfa). The indirect
  call has an unknown callee, candidate zero `[[sp+4]+224]`, unknown candidate one, and candidate
  two `[sp+12]` (`call` at 0x18fe0e). The next branch falls through when `ret(unknown) == 0x47`
  (`jne` at 0x18fe16), reaching `_tyIoctl` with candidates `[sp+4]`, unknown, and `[sp+12]` (`call`
  at 0x18fe1b). No ioctl-code translation meaning is inferred.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x18fdee | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x18fdf1 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x18fdf4 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x18fe01 | load | 4 | `[sp+4] +0xe0` | `-` |
| `mov` at 0x18fe07 | load | 4 | `[[sp+4]+224] +0x0` | `-` |
| `mov` at 0x18fe0c | load | 4 | `[[[sp+4]+224]] +0x0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x18fe0e | `(unknown callee) (unknown target)` | `[[sp+4]+224]` | `?` | `[sp+12]` | `ebx` | `esi` | `edi` |
| `call` at 0x18fe1b | `_tyIoctl 0x16e370` | `[sp+4]` | `?` | `[sp+12]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x18fdfa | `[sp+8] != 0x4` | `[sp+8] == 0x4` |
| `jne` at 0x18fe16 | `ret(unknown) != 0x47` | `ret(unknown) == 0x47` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x18fe29 | `eax` | `?` |
