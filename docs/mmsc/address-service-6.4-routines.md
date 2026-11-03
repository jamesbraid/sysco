# MMSC address service in IRIX 6.4

Read from `6.4/usr/cpu/firmware/mmscfw.bin` (IRIX 6.4).
Image SHA-256: `58ac958cb0c3fa09f807340ec3f2f3864e3c601b3fc5f057ae9a8100db18a098`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred end and can include literals
after returns. Statements below concern sites reached by known control-flow edges.
Unresolved computed jumps make the effects incomplete for the affected routines.
I/O prefixes in the access table identify ports rather than memory addresses.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/address-service-6.4-facts.txt) include all tool sections.

## `_addrdInit`

- `_addrdInit`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10d950 to 0x10dae0.
  Established. The `_pipeDevCreate` call has leading candidates `0x10d882`, `0x5`, and `0x3e`
  (`call` at 0x10d95c). Four-byte stores save `ret(_open)` at absolute `0x1ab018` (`mov` at 0x10d9a1),
  `ret(_wdCreate)` at `0x1ebaf8` (`mov` at 0x10d9c5), and `ret(_semMCreate)` at `0x1ef52c` (`mov` at
  0x10da07). The `_etherInputHookAdd` call has leading candidate `0x10e3c0` (`call` at 0x10da59).
  The task and network setup semantics are unknown without the callees.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10d96d | load | 4 | `ret(___errno) +0x0` | `-` |
| `mov` at 0x10d9a1 | store | 4 | `abs 0x1ab018` | `ret(_open)` |
| `mov` at 0x10d9c5 | store | 4 | `abs 0x1ebaf8` | `ret(_wdCreate)` |
| `mov` at 0x10da07 | store | 4 | `abs 0x1ef52c` | `ret(_semMCreate)` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10d95c | `_pipeDevCreate 0x18c2ac` | `0x10d882` | `0x5` | `0x3e` | `ebp` | `?` | `?` |
| `call` at 0x10d968 | `___errno 0x170518` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d97b | `_ffscMsgS 0x11eec0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d999 | `_open 0x1877d0` | `0x10d882` | `0x1` | `0x0` | `?` | `?` | `?` |
| `call` at 0x10d9c0 | `_wdCreate 0x174204` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d9d3 | `_ffscMsg 0x11edf0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d9f5 | `_lstInit 0x18e868` | `0x1db808` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d9ff | `_semMCreate 0x176058` | `0x1` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10da47 | `_taskSpawn 0x1706a0` | `0x10d91f` | `0x19` | `0x0` | `0x4e20` | `0x10df10` | `0x0` |
| `call` at 0x10da59 | `_etherInputHookAdd 0x18f2a8` | `0x10e3c0` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10da75 | `_ffscMsg 0x11edf0` | `0x10d926` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10d966 | `(ret(_pipeDevCreate) & ret(_pipeDevCreate)) == 0x0` | `(ret(_pipeDevCreate) & ret(_pipeDevCreate)) != 0x0` |
| `je` at 0x10d974 | `[ret(___errno)] == 0xd0006` | `[ret(___errno)] != 0xd0006` |
| `jge` at 0x10d9a8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x10d9cc | `(ret(_wdCreate) & ret(_wdCreate)) != 0x0` | `(ret(_wdCreate) & ret(_wdCreate)) == 0x0` |
| `jne` at 0x10da0e | `(ret(_semMCreate) & ret(_semMCreate)) != 0x0` | `(ret(_semMCreate) & ret(_semMCreate)) == 0x0` |
| `je` at 0x10da52 | `ret(_taskSpawn) == 0xffffffff` | `ret(_taskSpawn) != 0xffffffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10d988 | `eax` | `0xffffffff` |
| `ret` at 0x10d9e0 | `eax` | `0xffffffff` |
| `ret` at 0x10da63 | `eax` | `0x0` |
| `ret` at 0x10da82 | `eax` | `0xffffffff` |

## `_addrd`

- `_addrd`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10df10 to 0x10e210.
  Established. The `_open` call has leading candidates `0x10d882`, `0x0`, and `0x0` (`call` at
  0x10df25). `_select` presents leading candidates `0x100`, `sp-132`, `0x0`, and `0x0` (`call` at
  0x10dfb1). `_read` has candidates beginning `ret(_open)`, `sp-68`, and `0x3e` (`call` at
  0x10e007). The branch takes `[ret(___errno)] == 0x4` and falls through for inequality (`je` at
  0x10dfc7). The computed transfer is unresolved (`jmp` at 0x10e0a1). Decoder coverage stops inside
  the inline table, so the main fact sheet does not establish the message-dispatch role or later
  effects; its return list is empty, and known branches targeting code beyond the decoder stop
  also lack analysed destinations.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10df63 | store | 4 | `sp -0x8c` | `?` |
| `mov` at 0x10df79 | load | 4 | `sp -0x8c` | `-` |
| `mov` at 0x10df7f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10df85 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10df89 | store | 4 | `sp -0x88` | `0x1` |
| `rep movsd` at 0x10dfa0 | store | 4 | `(unknown)` | `?` |
| `rep movsd` at 0x10dfa0 | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x10dfc2 | load | 4 | `ret(___errno) +0x0` | `-` |
| `dec` at 0x10dfe3 | load | 4 | `sp -0x88` | `-` |
| `dec` at 0x10dfe3 | store | 4 | `sp -0x88` | `?` |
| `mov` at 0x10dff3 | load | 4 | `(unknown)` | `-` |
| `dec` at 0x10e056 | load | 4 | `sp -0x88` | `-` |
| `dec` at 0x10e056 | store | 4 | `sp -0x88` | `?` |
| `mov` at 0x10e05f | load | 4 | `sp -0x36` | `-` |
| `mov` at 0x10e06c | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x10e07a | store | 4 | `sp -0x36` | `0x2` |
| `mov` at 0x10e080 | load | 4 | `sp -0x36` | `-` |
| `mov` at 0x10e087 | store | 4 | `sp -0x1a` | `0x0` |
| `mov` at 0x10e08e | store | 4 | `sp -0x36` | `0x3` |
| `mov` at 0x10e094 | load | 4 | `sp -0x32` | `-` |
| `jmp` at 0x10e0a1 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10df25 | `_open 0x1877d0` | `0x10d882` | `0x0` | `0x0` | `ebx` | `esi` | `edi` |
| `call` at 0x10df38 | `_ffscMsgS 0x11eec0` | `0x10de48` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10df56 | `_bzero 0x1719c0` | `sp-100` | `0x20` | `?` | `?` | `?` | `?` |
| `call` at 0x10dfb1 | `_select 0x173a68` | `0x100` | `sp-132` | `0x0` | `0x0` | `0x0` | `?` |
| `call` at 0x10dfbd | `___errno 0x170518` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dfd2 | `_ffscMsgS 0x11eec0` | `0x10de64` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dfdb | `_close 0x187970` | `ret(_open)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e007 | `_read 0x1879f0` | `ret(_open)` | `sp-68` | `0x3e` | `?` | `?` | `?` |
| `call` at 0x10e018 | `_ffscMsgS 0x11eec0` | `0x10de7b` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e031 | `_ffscMsg 0x11edf0` | `0x10deb2` | `ret(_read)` | `0x3e` | `?` | `?` | `?` |
| `call` at 0x10e045 | `_ffscMsg 0x11edf0` | `0x10de9a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e04e | `_close 0x187970` | `ret(_open)` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x10df31 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jge` at 0x10dfbb | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x10dfc7 | `[ret(___errno)] == 0x4` | `[ret(___errno)] != 0x4` |
| `jae` at 0x10dffa | `C == 0` | `not (C == 0)` |
| `jge` at 0x10e011 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x10e022 | `(ret(_read) & ret(_read)) == 0x0` | `(ret(_read) & ret(_read)) != 0x0` |
| `je` at 0x10e027 | `ret(_read) == 0x3e` | `ret(_read) != 0x3e` |
| `ja` at 0x10e065 | `[sp-54] > 0x1 (unsigned, 32-bit)` | `[sp-54] <= 0x1 (unsigned, 32-bit)` |
| `jle` at 0x10e078 | `? <= 0x1 (signed, 32-bit)` | `? > 0x1 (signed, 32-bit)` |
| `ja` at 0x10e085 | `[sp-54] > 0x2 (unsigned, 32-bit)` | `[sp-54] <= 0x2 (unsigned, 32-bit)` |
| `ja` at 0x10e09b | `? > 0x5 (unsigned, 32-bit)` | `? <= 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x10e0a1) and
code after the decoder stop at 0x10e0b8. An empty return table does not establish that
the routine cannot return.

## `_addrdEtherInputHook`

- `_addrdEtherInputHook`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e3c0 to 0x10e410.
  Established. A two-byte read addresses `[sp+8] +0xc` (`mov` at 0x10e3ca). The branch takes
  `([[sp+8]+12]:2 & 0xffff) != 0x801` and falls through for equality (`jne` at 0x10e3d2). The
  fall-through includes `_write` with leading candidates `[0x1ab018]`, `[sp+8]`, and `[sp+12]`
  (`call` at 0x10e3dc); its result branch takes `ret(_write) == [sp+12]` and falls through for
  inequality (`je` at 0x10e3e6). The frame-field and hook-return semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e3c4 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10e3c7 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10e3ca | load | 2 | `[sp+8] +0xc` | `-` |
| `mov` at 0x10e3d6 | load | 4 | `abs 0x1ab018` | `-` |
| `mov` at 0x10e402 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e3dc | `_write 0x187a10` | `[0x1ab018]` | `[sp+8]` | `[sp+12]` | `ebx` | `ebp` | `?` |
| `call` at 0x10e3ed | `_ffscMsgS 0x11eec0` | `0x10e39e` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e3d2 | `([[sp+8]+12]:2 & 0xffff) != 0x801` | `([[sp+8]+12]:2 & 0xffff) == 0x801` |
| `je` at 0x10e3e6 | `ret(_write) == [sp+12]` | `ret(_write) != [sp+12]` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e408 | `eax` | `?` |

## `_addrdLocalMsg`

- `_addrdLocalMsg`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10dde0 to 0x10df10.
  Established. The `_bzero` call presents leading candidates `sp-68` and `0x3e` (`call` at
  0x10ddf1). A two-byte store writes `0x108` at `sp -0x38` (`mov` at 0x10ddf9). `_bcopy` has leading
  candidates `[sp+4]`, `sp-54`, and `0x30` (`call` at 0x10de06). `_write` presents `[0x1ab018]`,
  `sp-68`, and `0x3e` (`call` at 0x10de17). The branch takes `ret(_write) != 0x3e` and falls through
  for equality (`jne` at 0x10de22). The local transport mapping is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10dde8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10ddf9 | store | 2 | `sp -0x38` | `0x108` |
| `mov` at 0x10de11 | load | 4 | `abs 0x1ab018` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ddf1 | `_bzero 0x1719c0` | `sp-68` | `0x3e` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10de06 | `_bcopy 0x17529c` | `[sp+4]` | `sp-54` | `0x30` | `?` | `?` | `?` |
| `call` at 0x10de17 | `_write 0x187a10` | `[0x1ab018]` | `sp-68` | `0x3e` | `?` | `?` | `?` |
| `call` at 0x10de35 | `_ffscMsgS 0x11eec0` | `0x10ddb1` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10de22 | `ret(_write) != 0x3e` | `ret(_write) == 0x3e` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10de47 | `eax` | `?` |

## `_addrdBroadcastMsg`

- `_addrdBroadcastMsg`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10dc80 to 0x10dd00.
  Established. A two-byte store places `0x108` at `sp -0x8` (`mov` at 0x10dcb1). The `_etherOutput`
  call has leading candidates `[0x1ebae4]`, `sp-20`, `[sp+4]`, and `0x30` (`call` at 0x10dcc1). The
  branch takes `(ret(_etherOutput) & ret(_etherOutput)) != 0x0` and falls through for zero (`jne` at
  0x10dccb). The interface identity and packet destination remain unknown from these expressions.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10dc88 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10dcb1 | store | 2 | `sp -0x8` | `0x108` |
| `mov` at 0x10dcbb | load | 4 | `abs 0x1ebae4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10dc99 | `_bcopy 0x17529c` | `0x1ef610` | `sp-14` | `0x6` | `ebx` | `esi` | `?` |
| `call` at 0x10dca9 | `_bcopy 0x17529c` | `0x1d1bc8` | `sp-20` | `0x6` | `?` | `?` | `?` |
| `call` at 0x10dcc1 | `_etherOutput 0x18f168` | `[0x1ebae4]` | `sp-20` | `[sp+4]` | `0x30` | `?` | `?` |
| `call` at 0x10dce5 | `_ffscMsgS 0x11eec0` | `0x10dc5c` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10dccb | `(ret(_etherOutput) & ret(_etherOutput)) != 0x0` | `(ret(_etherOutput) & ret(_etherOutput)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10dcf7 | `eax` | `?` |

## `_addrdExtractIdentity`

- `_addrdExtractIdentity`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10dd00 to 0x10dd60.
  Established. The `_bcopy` calls present leading candidate triples `[sp+4]+32`, `[sp+8]`, `0x8`
  (`call` at 0x10dd25) and `[sp+4]+40`, `[sp+8]+8`, `0x4` (`call` at 0x10dd37). Four-byte stores write
  `[[sp+4]+44]` at `[sp+8] +0xc` (`mov` at 0x10dd3f) and `[[sp+4]+28]` at `[sp+8] +0x10` (`mov` at
  0x10dd45). The return carries `[[sp+4]+28]` in EAX (`ret` at 0x10dd50). The identities of the
  copied fields remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10dd05 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10dd08 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10dd10 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10dd14 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10dd3c | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10dd3f | store | 4 | `[sp+8] +0xc` | `[[sp+4]+44]` |
| `mov` at 0x10dd42 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x10dd45 | store | 4 | `[sp+8] +0x10` | `[[sp+4]+28]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10dd25 | `_bcopy 0x17529c` | `[sp+4]+32` | `[sp+8]` | `0x8` | `?` | `?` | `?` |
| `call` at 0x10dd37 | `_bcopy 0x17529c` | `[sp+4]+40` | `[sp+8]+8` | `0x4` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x10dd1c | `? <= 0x1 (signed, 32-bit)` | `? > 0x1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10dd50 | `eax` | `[[sp+4]+28]` |

## `_addrdInsertIdentity`

- `_addrdInsertIdentity`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10dd60 to 0x10dde0.
  Established. Calling `_bcopy` exposes leading candidates `[sp+8]`, `[sp+4]+32`, and `0x8` (`call`
  at 0x10dd85). Another `_bcopy` call presents `[sp+8]+8`, `[sp+4]+40`, and `0x4` (`call` at
  0x10dd97). Four-byte stores place `[[sp+8]+12]` at `[sp+4] +0x2c` (`mov` at 0x10dd9f) and `[[sp+8]+16]`
  at `[sp+4] +0x1c` (`mov` at 0x10dda5). The record-field semantics are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10dd65 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10dd68 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10dd70 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10dd74 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10dd9c | load | 4 | `[sp+8] +0xc` | `-` |
| `mov` at 0x10dd9f | store | 4 | `[sp+4] +0x2c` | `[[sp+8]+12]` |
| `mov` at 0x10dda2 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x10dda5 | store | 4 | `[sp+4] +0x1c` | `[[sp+8]+16]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10dd85 | `_bcopy 0x17529c` | `[sp+8]` | `[sp+4]+32` | `0x8` | `?` | `?` | `?` |
| `call` at 0x10dd97 | `_bcopy 0x17529c` | `[sp+8]+8` | `[sp+4]+40` | `0x4` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x10dd7c | `? <= 0x1 (signed, 32-bit)` | `? > 0x1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ddb0 | `eax` | `[[sp+8]+16]` |

## `_addrdAssignNewIdentity`

- `_addrdAssignNewIdentity`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e210 to 0x10e310.
  Established. The `_identFindByUniqueID` call has leading candidate `[sp+4]` (`call` at 0x10e21d).
  A branch takes `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` and falls
  through for zero (`jne` at 0x10e250). Four-byte stores on the taken route write `0x3` at `sp -0x4c`
  (`mov` at 0x10e280) and `sp -0x48` (`mov` at 0x10e287). The `_addrdBroadcastMsg` call has leading
  candidate `sp-76` (`call` at 0x10e29c). The assignment policy beyond these operations is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e219 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10e280 | store | 4 | `sp -0x4c` | `0x3` |
| `mov` at 0x10e287 | store | 4 | `sp -0x48` | `0x3` |
| `test` at 0x10e2d0 | load | 1 | `abs 0x1db370` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e21d | `_identFindByUniqueID 0x1226f8` | `[sp+4]` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x10e230 | `_ffscUniqueIDToString 0x125a98` | `[sp+4]` | `sp-28` | `?` | `?` | `?` | `?` |
| `call` at 0x10e246 | `_ffscCompareUniqueIDs 0x125268` | `[sp+4]` | `0x1ef238` | `?` | `?` | `?` | `?` |
| `call` at 0x10e253 | `_addrdUpdateLocalIdentity 0x10db80` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e268 | `_ffscMsg 0x11edf0` | `0x10e198` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e293 | `_addrdInsertIdentity 0x10dd60` | `sp-76` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x10e29c | `_addrdBroadcastMsg 0x10dc80` | `sp-76` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e2ad | `_ffscUniqueIDToString 0x125a98` | `ret(_identFindByUniqueID)` | `sp-100` | `?` | `?` | `?` | `?` |
| `call` at 0x10e2bb | `_ffscMsg 0x11edf0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e2de | `_ffscMsg 0x11edf0` | `0x10e1e9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e2e7 | `_identPrint 0x122e98` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e2f3 | `_memcpy 0x17e840` | `ret(_identFindByUniqueID)` | `[sp+4]` | `0x1c` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e229 | `(ret(_identFindByUniqueID) & ret(_identFindByUniqueID)) != 0x0` | `(ret(_identFindByUniqueID) & ret(_identFindByUniqueID)) == 0x0` |
| `jne` at 0x10e250 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `je` at 0x10e25d | `(ret(_addrdUpdateLocalIdentity) & ret(_addrdUpdateLocalIdentity)) == 0x0` | `(ret(_addrdUpdateLocalIdentity) & ret(_addrdUpdateLocalIdentity)) != 0x0` |
| `je` at 0x10e2a6 | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` |
| `je` at 0x10e2d7 | `(([0x1db370]:1 & 0xff) & 0x1) == 0x0` | `(([0x1db370]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e303 | `eax` | `?` |

## `_addrdProcessIdentityMsg`

- `_addrdProcessIdentityMsg`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e8b0 to 0x10e9e0.
  Established. The `_addrdExtractIdentity` call has leading candidates `[sp+4]` and `sp-32` (`call`
  at 0x10e8c0). The branch takes `[0x1db3bc] == 0x0` and falls through for nonzero (`je` at
  0x10e916). That fall-through includes `_addrdStartElectionTimer` with unknown candidates (`call`
  at 0x10e918). The `_identUpdate` call has leading candidate `sp-32` (`call` at 0x10e901). The
  identity-update and timer-restart semantics are unknown without those callees.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e8b8 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x10e90f | load | 4 | `abs 0x1db3bc` | `-` |
| `test` at 0x10e921 | load | 1 | `abs 0x1db370` | `-` |
| `cmp` at 0x10e952 | load | 4 | `abs 0x1ef76c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e8c0 | `_addrdExtractIdentity 0x10dd00` | `[sp+4]` | `sp-32` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10e8ce | `_ffscCompareUniqueIDs 0x125268` | `sp-32` | `0x1ef238` | `?` | `?` | `?` | `?` |
| `call` at 0x10e8df | `_ffscUniqueIDToString 0x125a98` | `sp-32` | `sp-56` | `?` | `?` | `?` | `?` |
| `call` at 0x10e8ed | `_ffscMsg 0x11edf0` | `0x10e83c` | `sp-56` | `?` | `?` | `?` | `?` |
| `call` at 0x10e901 | `_identUpdate 0x122d68` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e918 | `_addrdStartElectionTimer 0x10ee70` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e93e | `_ffscMsg 0x11edf0` | `0x10e893` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e94a | `_identPrint 0x122e98` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e95b | `_addrdResolveConflicts 0x10eca0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e969 | `_ffscMsg 0x11edf0` | `0x10e5e5` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e8d8 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `jl` at 0x10e90d | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10e916 | `[0x1db3bc] == 0x0` | `[0x1db3bc] != 0x0` |
| `jne` at 0x10e91f | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) != 0x0` | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) == 0x0` |
| `je` at 0x10e928 | `(([0x1db370]:1 & 0xff) & 0x1) == 0x0` | `(([0x1db370]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x10e931 | `(ret(_identUpdate) & ret(_identUpdate)) != 0x0` | `(ret(_identUpdate) & ret(_identUpdate)) == 0x0` |
| `je` at 0x10e959 | `[0x1ef76c] == 0x0` | `[0x1ef76c] != 0x0` |
| `je` at 0x10e962 | `(ret(_addrdResolveConflicts) & ret(_addrdResolveConflicts)) == 0x0` | `(ret(_addrdResolveConflicts) & ret(_addrdResolveConflicts)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e978 | `eax` | `?` |

## `_addrdProcessQueryMsg`

- `_addrdProcessQueryMsg`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e9e0 to 0x10eb50.
  Established. The `_clock_gettime` call presents leading candidates `0x0` and `0x1ebaf0` (`call` at
  0x10e9f2). `_taskDelay` has leading candidate `0x3` (`call` at 0x10ea9c). Four-byte stores include
  `0x3` at `sp -0x34` (`mov` at 0x10eaa4) and `0x1` at `sp -0x30` (`mov` at 0x10eaab). The
  `_addrdBroadcastMsg` call has leading candidate `sp-52` (`call` at 0x10eac4). A later four-byte store
  sets absolute `0x1db3bc` to `0x1` (`mov` at 0x10ead9). The query-response format, delay units and
  flag meaning remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e9e8 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x10ea53 | load | 4 | `abs 0x1db3bc` | `-` |
| `test` at 0x10ea69 | load | 1 | `abs 0x1db370` | `-` |
| `mov` at 0x10eaa4 | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10eaab | store | 4 | `sp -0x30` | `0x1` |
| `mov` at 0x10ead9 | store | 4 | `abs 0x1db3bc` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e9f2 | `_clock_gettime 0x17a2bc` | `0x0` | `0x1ebaf0` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10e9ff | `_addrdExtractIdentity 0x10dd00` | `[sp+4]` | `sp-80` | `?` | `?` | `?` | `?` |
| `call` at 0x10ea0d | `_ffscCompareUniqueIDs 0x125268` | `sp-80` | `0x1ef238` | `?` | `?` | `?` | `?` |
| `call` at 0x10ea1e | `_ffscUniqueIDToString 0x125a98` | `sp-80` | `sp-104` | `?` | `?` | `?` | `?` |
| `call` at 0x10ea2c | `_ffscMsg 0x11edf0` | `0x10e979` | `sp-104` | `?` | `?` | `?` | `?` |
| `call` at 0x10ea41 | `_identUpdate 0x122d68` | `sp-80` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ea5c | `_addrdStartElectionTimer 0x10ee70` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ea86 | `_ffscMsg 0x11edf0` | `0x10e9c2` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ea92 | `_identPrint 0x122e98` | `sp-80` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ea9c | `_taskDelay 0x171620` | `0x3` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10eabb | `_addrdInsertIdentity 0x10dd60` | `sp-52` | `0x1ef238` | `?` | `?` | `?` | `?` |
| `call` at 0x10eac4 | `_addrdBroadcastMsg 0x10dc80` | `sp-52` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ead0 | `_addrdStartElectionTimer 0x10ee70` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10ea17 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `jl` at 0x10ea4d | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10ea5a | `[0x1db3bc] == 0x0` | `[0x1db3bc] != 0x0` |
| `jne` at 0x10ea63 | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) != 0x0` | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) == 0x0` |
| `je` at 0x10ea70 | `(([0x1db370]:1 & 0xff) & 0x1) == 0x0` | `(([0x1db370]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x10ea79 | `(ret(_identUpdate) & ret(_identUpdate)) != 0x0` | `(ret(_identUpdate) & ret(_identUpdate)) == 0x0` |
| `jne` at 0x10eace | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` |
| `jne` at 0x10ead7 | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) != 0x0` | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10eafd | `eax` | `?` |

## `_addrdProcessAssignMsg`

- `_addrdProcessAssignMsg`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e500 to 0x10e610.
  Established. The `_ffscCompareUniqueIDs` call has leading candidates `[sp+4]+32` and `0x1ef238`
  (`call` at 0x10e521). The branch takes `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs))
  == 0x0` and falls through for nonzero (`je` at 0x10e52b); its taken route calls
  `_addrdUpdateLocalIdentity` with leading candidate `sp-32` (`call` at 0x10e541). The exit returns
  EAX zero (`ret` at 0x10e580). The message-assignment role remains unknown beyond those calls.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e508 | load | 4 | `sp +0x4` | `-` |
| `test` at 0x10e52d | load | 1 | `abs 0x1db370` | `-` |
| `test` at 0x10e55a | load | 1 | `abs 0x1db370` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e510 | `_addrdExtractIdentity 0x10dd00` | `[sp+4]` | `sp-32` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10e521 | `_ffscCompareUniqueIDs 0x125268` | `[sp+4]+32` | `0x1ef238` | `?` | `?` | `?` | `?` |
| `call` at 0x10e541 | `_addrdUpdateLocalIdentity 0x10db80` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e552 | `_ffscMsg 0x11edf0` | `0x10e4ba` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e568 | `_ffscMsg 0x11edf0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e571 | `_identPrint 0x122e98` | `sp-32` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10e52b | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` |
| `je` at 0x10e534 | `(([0x1db370]:1 & 0xff) & 0x1) == 0x0` | `(([0x1db370]:1 & 0xff) & 0x1) != 0x0` |
| `je` at 0x10e54b | `(ret(_addrdUpdateLocalIdentity) & ret(_addrdUpdateLocalIdentity)) == 0x0` | `(ret(_addrdUpdateLocalIdentity) & ret(_addrdUpdateLocalIdentity)) != 0x0` |
| `je` at 0x10e561 | `(([0x1db370]:1 & 0xff) & 0x1) == 0x0` | `(([0x1db370]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e580 | `eax` | `0x0` |

## `_addrdResolveConflicts`

- `_addrdResolveConflicts`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10eca0 to 0x10ed20.
  Established. The branch takes `[0x1ef76c] != 0x0` and falls through for zero (`jne` at 0x10ecaa).
  On the taken route, `_identScanForConflicts` has leading candidates `0x10ed20` and `0x10edc0`
  (`call` at 0x10ecca); its result branch takes `(ret(_identScanForConflicts) &
  ret(_identScanForConflicts)) != 0x0` and falls through for zero (`jne` at 0x10ecd4). The global
  authorization and conflict policy remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10eca3 | load | 4 | `abs 0x1ef76c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ecb1 | `_ffscMsg 0x11edf0` | `0x10ec3c` | `ebp` | `?` | `?` | `?` | `?` |
| `call` at 0x10ecca | `_identScanForConflicts 0x122ad8` | `0x10ed20` | `0x10edc0` | `ebp` | `?` | `?` | `?` |
| `call` at 0x10ece5 | `_ffscMsg 0x11edf0` | `0x10ec6c` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10ecaa | `[0x1ef76c] != 0x0` | `[0x1ef76c] == 0x0` |
| `jne` at 0x10ecd4 | `(ret(_identScanForConflicts) & ret(_identScanForConflicts)) != 0x0` | `(ret(_identScanForConflicts) & ret(_identScanForConflicts)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ecbe | `eax` | `0xffffffff` |
| `ret` at 0x10ecdb | `eax` | `0x0` |
| `ret` at 0x10ecf2 | `eax` | `0xffffffff` |

## `_addrdResolveIPAddrConflict`

- `_addrdResolveIPAddrConflict`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ed20 to 0x10edc0.
  Established. Calling `_ffscCompareUniqueIDs` presents leading candidates `[sp+4]` and `[sp+8]`
  (`call` at 0x10ed30). `_memcpy` has leading candidates `sp-32`, unknown, and `0x1c` (`call` at
  0x10ed57). `_addrdFindUnusedIPAddr` has leading candidate `sp-24` (`call` at 0x10ed66). The branch
  takes `(ret(_addrdFindUnusedIPAddr) & ret(_addrdFindUnusedIPAddr)) != 0x0` and falls through for
  zero (`jne` at 0x10ed70). The selected identity and address-allocation policy are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10ed28 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10ed2b | load | 4 | `sp +0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ed30 | `_ffscCompareUniqueIDs 0x125268` | `[sp+4]` | `[sp+8]` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10ed57 | `_memcpy 0x17e840` | `sp-32` | `?` | `0x1c` | `?` | `?` | `?` |
| `call` at 0x10ed66 | `_addrdFindUnusedIPAddr 0x10e410` | `sp-24` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ed73 | `_addrdAssignNewIdentity 0x10e210` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ed85 | `_ffscMsg 0x11edf0` | `0x10ecf3` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x10ed3a | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x10ed70 | `(ret(_addrdFindUnusedIPAddr) & ret(_addrdFindUnusedIPAddr)) != 0x0` | `(ret(_addrdFindUnusedIPAddr) & ret(_addrdFindUnusedIPAddr)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ed97 | `eax` | `?` |

## `_addrdFindUnusedIPAddr`

- `_addrdFindUnusedIPAddr`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e410 to 0x10e470.
  Established. The `_inet_makeaddr_b` call has leading candidates `[0x1db84c]`, unknown, and
  `[sp+4]` (`call` at 0x10e428). `_identFindByIPAddr` presents leading candidate `[sp+4]` (`call` at
  0x10e431). The branch takes `(ret(_identFindByIPAddr) & ret(_identFindByIPAddr)) != 0x0` and falls
  through for zero (`jne` at 0x10e43b). The loop branch takes `? <= 0xfffffe (signed, 32-bit)` and
  falls through for larger values, with its operand unknown (`jle` at 0x10e457). The concrete
  address range remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e415 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10e422 | load | 4 | `abs 0x1db84c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e428 | `_inet_makeaddr_b 0x1870c0` | `[0x1db84c]` | `?` | `[sp+4]` | `?` | `?` | `?` |
| `call` at 0x10e431 | `_identFindByIPAddr 0x1225e8` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e43b | `(ret(_identFindByIPAddr) & ret(_identFindByIPAddr)) != 0x0` | `(ret(_identFindByIPAddr) & ret(_identFindByIPAddr)) == 0x0` |
| `jle` at 0x10e457 | `? <= 0xfffffe (signed, 32-bit)` | `? > 0xfffffe (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e466 | `eax` | `?` |

## `_addrdUpdateLocalIdentity`

- `_addrdUpdateLocalIdentity`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10db80 to 0x10dc80.
  Established. The first branch takes `[0x1ef240] == [[sp+4]+8]` and falls through for inequality
  (`je` at 0x10db96). The fall-through calls `_identSetIPAddr` with leading candidate `[sp+4]+8`
  (`call` at 0x10db9c). Later four-byte stores include `0x3` at `sp -0x34` (`mov` at 0x10dc29) and `0x1`
  at `sp -0x30` (`mov` at 0x10dc30). `_addrdBroadcastMsg` has leading candidate `sp-52` (`call` at
  0x10dc45). The meanings of the compared globals and the resulting identity change are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10db88 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10db8d | load | 4 | `[sp+4] +0x8` | `-` |
| `cmp` at 0x10db90 | load | 4 | `abs 0x1ef240` | `-` |
| `mov` at 0x10dba9 | load | 4 | `[sp+4] +0xc` | `-` |
| `cmp` at 0x10dbac | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x10dbb4 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10dbc5 | load | 4 | `[sp+4] +0x10` | `-` |
| `cmp` at 0x10dbc8 | load | 4 | `abs 0x1ef248` | `-` |
| `mov` at 0x10dbd0 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10dbe4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10dbe8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10dc29 | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10dc30 | store | 4 | `sp -0x30` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10db9c | `_identSetIPAddr 0x123088` | `[sp+4]+8` | `ebx` | `esi` | `?` | `?` | `?` |
| `call` at 0x10dbb8 | `_identSetRackID 0x123258` | `[[sp+4]+12]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dbd4 | `_identSetCapabilities 0x122fa8` | `[[sp+4]+16]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dc14 | `_identSetModuleNums 0x123178` | `[sp+4]+20` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dc1d | `_identUpdate 0x122d68` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dc3c | `_addrdInsertIdentity 0x10dd60` | `sp-52` | `[sp+4]` | `?` | `?` | `0x3` | `0x1` |
| `call` at 0x10dc45 | `_addrdBroadcastMsg 0x10dc80` | `sp-52` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10db96 | `[0x1ef240] == [[sp+4]+8]` | `[0x1ef240] != [[sp+4]+8]` |
| `je` at 0x10dbb2 | `[0x1ef244] == [[sp+4]+12]` | `[0x1ef244] != [[sp+4]+12]` |
| `je` at 0x10dbce | `[0x1ef248] == [[sp+4]+16]` | `[0x1ef248] != [[sp+4]+16]` |
| `jne` at 0x10dbf1 | `Z == 0` | `not (Z == 0)` |
| `jbe` at 0x10dbf7 | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |
| `jne` at 0x10dbfb | `Z == 0` | `not (Z == 0)` |
| `jl` at 0x10dc27 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10dc4c | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10dc5b | `eax` | `?` |
