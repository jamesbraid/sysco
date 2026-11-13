# MMSC address-service message paths in the 6.4-origin image

Read from `6.4-origin/usr/cpu/firmware/mmscfw.bin`.
Image SHA-256: `b766e559f86078526916d8bd98bbb8079294ad774cce9177928873153c6fb09b`.
The x86 a.out link address is `0x108000`. The [load map](load-map.md) explains segment mapping.
Analysed spans have exclusive ends inferred from the next text symbol.
Some include literals and padding decoded linearly after returns. Only sites reached
by known control-flow edges contribute effects to the tables.
Undecoded tails were read directly: `0x10ddc9` to exclusive `0x10ddd0` is
`69 6c 65 64 00 90 90`, `0x10e5fc` to exclusive `0x10e600` is `65 64 00 90`,
and `0x10ed0b` to exclusive `0x10ed10` is `00 90 90 90 90`.
No instruction effects are credited to those bytes.
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

[Fact sheets and complete listings](disasm/address-service-64-origin-facts.txt) include all tool sections.

## `_addrdInit`

- `_addrdInit`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10d940 to 0x10dad0.
  Established. The `_pipeDevCreate` site presents `0x10d872`, `0x5`, and `0x3e` as its first three
  candidates (`call` at 0x10d94c). Four-byte stores at absolute addresses retain `ret(_open)` at
  `0x1aa578` (`mov` at 0x10d991), `ret(_wdCreate)` at `0x1eae18` (`mov` at 0x10d9b5), and
  `ret(_semMCreate)` at `0x1ee84c` (`mov` at 0x10d9f7). The `_etherInputHookAdd` site has candidate
  zero `0x10e3b0` (`call` at 0x10da49). Callee effects remain unassigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10d95d | load | 4 | `ret(___errno) +0x0` | `-` |
| `mov` at 0x10d991 | store | 4 | `abs 0x1aa578` | `ret(_open)` |
| `mov` at 0x10d9b5 | store | 4 | `abs 0x1eae18` | `ret(_wdCreate)` |
| `mov` at 0x10d9f7 | store | 4 | `abs 0x1ee84c` | `ret(_semMCreate)` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10d94c | `_pipeDevCreate 0x18b80c` | `0x10d872` | `0x5` | `0x3e` | `ebp` | `?` | `?` |
| `call` at 0x10d958 | `___errno 0x16fa78` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d96b | `_ffscMsgS 0x11eca0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d989 | `_open 0x186d30` | `0x10d872` | `0x1` | `0x0` | `?` | `?` | `?` |
| `call` at 0x10d9b0 | `_wdCreate 0x173764` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d9c3 | `_ffscMsg 0x11ebd0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d9e5 | `_lstInit 0x18ddc8` | `0x1dab30` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d9ef | `_semMCreate 0x1755b8` | `0x1` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10da37 | `_taskSpawn 0x16fc00` | `0x10d90f` | `0x19` | `0x0` | `0x4e20` | `0x10df00` | `0x0` |
| `call` at 0x10da49 | `_etherInputHookAdd 0x18e808` | `0x10e3b0` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10da65 | `_ffscMsg 0x11ebd0` | `0x10d916` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10d956 | `(ret(_pipeDevCreate) & ret(_pipeDevCreate)) == 0x0` | `(ret(_pipeDevCreate) & ret(_pipeDevCreate)) != 0x0` |
| `je` at 0x10d964 | `[ret(___errno)] == 0xd0006` | `[ret(___errno)] != 0xd0006` |
| `jge` at 0x10d998 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x10d9bc | `(ret(_wdCreate) & ret(_wdCreate)) != 0x0` | `(ret(_wdCreate) & ret(_wdCreate)) == 0x0` |
| `jne` at 0x10d9fe | `(ret(_semMCreate) & ret(_semMCreate)) != 0x0` | `(ret(_semMCreate) & ret(_semMCreate)) == 0x0` |
| `je` at 0x10da42 | `ret(_taskSpawn) == 0xffffffff` | `ret(_taskSpawn) != 0xffffffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10d978 | `eax` | `0xffffffff` |
| `ret` at 0x10d9d0 | `eax` | `0xffffffff` |
| `ret` at 0x10da53 | `eax` | `0x0` |
| `ret` at 0x10da72 | `eax` | `0xffffffff` |

## `_addrd`

- `_addrd`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10df00 to 0x10e200.
  Established. The `_select` site presents leading candidates `0x100`, `sp-132`, `0x0`, `0x0`, and
  `0x0` (`call` at 0x10dfa1). The `_read` site presents `ret(_open)`, `sp-68`, and `0x3e` (`call` at
  0x10dff7); its comparison branch is taken when `ret(_read) == 0x3e` (`je` at 0x10e017). Four-byte
  stores to `sp -0x36` write `0x2` (`mov` at 0x10e06a) and `0x3` (`mov` at 0x10e07e). A computed
  transfer has unresolved edges (`jmp` at 0x10e091), so handler-path effects are excluded.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10df53 | store | 4 | `sp -0x8c` | `?` |
| `mov` at 0x10df69 | load | 4 | `sp -0x8c` | `-` |
| `mov` at 0x10df6f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10df75 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10df79 | store | 4 | `sp -0x88` | `0x1` |
| `rep movsd` at 0x10df90 | store | 4 | `(unknown)` | `?` |
| `rep movsd` at 0x10df90 | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x10dfb2 | load | 4 | `ret(___errno) +0x0` | `-` |
| `dec` at 0x10dfd3 | load | 4 | `sp -0x88` | `-` |
| `dec` at 0x10dfd3 | store | 4 | `sp -0x88` | `?` |
| `mov` at 0x10dfe3 | load | 4 | `(unknown)` | `-` |
| `dec` at 0x10e046 | load | 4 | `sp -0x88` | `-` |
| `dec` at 0x10e046 | store | 4 | `sp -0x88` | `?` |
| `mov` at 0x10e04f | load | 4 | `sp -0x36` | `-` |
| `mov` at 0x10e05c | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x10e06a | store | 4 | `sp -0x36` | `0x2` |
| `mov` at 0x10e070 | load | 4 | `sp -0x36` | `-` |
| `mov` at 0x10e077 | store | 4 | `sp -0x1a` | `0x0` |
| `mov` at 0x10e07e | store | 4 | `sp -0x36` | `0x3` |
| `mov` at 0x10e084 | load | 4 | `sp -0x32` | `-` |
| `jmp` at 0x10e091 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10e120 | load | 4 | `sp -0x32` | `-` |
| `cmp` at 0x10e131 | load | 4 | `sp -0x88` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10df15 | `_open 0x186d30` | `0x10d872` | `0x0` | `0x0` | `ebx` | `esi` | `edi` |
| `call` at 0x10df28 | `_ffscMsgS 0x11eca0` | `0x10de38` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10df46 | `_bzero 0x170f20` | `sp-100` | `0x20` | `?` | `?` | `?` | `?` |
| `call` at 0x10dfa1 | `_select 0x172fc8` | `0x100` | `sp-132` | `0x0` | `0x0` | `0x0` | `?` |
| `call` at 0x10dfad | `___errno 0x16fa78` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dfc2 | `_ffscMsgS 0x11eca0` | `0x10de54` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dfcb | `_close 0x186ed0` | `ret(_open)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dff7 | `_read 0x186f50` | `ret(_open)` | `sp-68` | `0x3e` | `?` | `?` | `?` |
| `call` at 0x10e008 | `_ffscMsgS 0x11eca0` | `0x10de6b` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e021 | `_ffscMsg 0x11ebd0` | `0x10dea2` | `ret(_read)` | `0x3e` | `?` | `?` | `?` |
| `call` at 0x10e035 | `_ffscMsg 0x11ebd0` | `0x10de8a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e03e | `_close 0x186ed0` | `ret(_open)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e129 | `_ffscMsg 0x11ebd0` | `0x10ded4` | `[sp-50]` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x10df21 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jge` at 0x10dfab | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x10dfb7 | `[ret(___errno)] == 0x4` | `[ret(___errno)] != 0x4` |
| `jae` at 0x10dfea | `C == 0` | `not (C == 0)` |
| `jge` at 0x10e001 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x10e012 | `(ret(_read) & ret(_read)) == 0x0` | `(ret(_read) & ret(_read)) != 0x0` |
| `je` at 0x10e017 | `ret(_read) == 0x3e` | `ret(_read) != 0x3e` |
| `ja` at 0x10e055 | `[sp-54] > 0x1 (unsigned, 32-bit)` | `[sp-54] <= 0x1 (unsigned, 32-bit)` |
| `jle` at 0x10e068 | `? <= 0x1 (signed, 32-bit)` | `? > 0x1 (signed, 32-bit)` |
| `ja` at 0x10e075 | `[sp-54] > 0x2 (unsigned, 32-bit)` | `[sp-54] <= 0x2 (unsigned, 32-bit)` |
| `ja` at 0x10e08b | `? > 0x5 (unsigned, 32-bit)` | `? <= 0x5 (unsigned, 32-bit)` |
| `jg` at 0x10e138 | `[sp-136] > 0x0 (signed, 32-bit)` | `[sp-136] <= 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e14c | `eax` | `?` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x10e091).

## `_addrdEtherInputHook`

- `_addrdEtherInputHook`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e3b0 to 0x10e400.
  Established. The two-byte load reads through `[sp+8] +0xc` (`mov` at 0x10e3ba). The deciding
  branch falls through when `([[sp+8]+12]:2 & 0xffff) == 0x801` (`jne` at 0x10e3c2). The `_write`
  call presents `[0x1aa578]`, `[sp+8]`, and `[sp+12]` as its first three candidates (`call` at
  0x10e3cc); its result branch is taken when `ret(_write) == [sp+12]` (`je` at 0x10e3d6). Neither a
  protocol number nor a transfer extent is inferred.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e3b4 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10e3b7 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10e3ba | load | 2 | `[sp+8] +0xc` | `-` |
| `mov` at 0x10e3c6 | load | 4 | `abs 0x1aa578` | `-` |
| `mov` at 0x10e3f2 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e3cc | `_write 0x186f70` | `[0x1aa578]` | `[sp+8]` | `[sp+12]` | `ebx` | `ebp` | `?` |
| `call` at 0x10e3dd | `_ffscMsgS 0x11eca0` | `0x10e38e` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e3c2 | `([[sp+8]+12]:2 & 0xffff) != 0x801` | `([[sp+8]+12]:2 & 0xffff) == 0x801` |
| `je` at 0x10e3d6 | `ret(_write) == [sp+12]` | `ret(_write) != [sp+12]` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e3f8 | `eax` | `?` |

## `_addrdLocalMsg`

- `_addrdLocalMsg`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ddd0 to 0x10df00.
  Established. A two-byte store writes `0x108` at `sp -0x38` (`mov` at 0x10dde9). The `_bcopy` site
  presents `[sp+4]`, `sp-54`, and `0x30` as leading candidates (`call` at 0x10ddf6). The `_write`
  site presents `[0x1aa578]`, `sp-68`, and `0x3e` (`call` at 0x10de07); its branch falls through
  when `ret(_write) == 0x3e` (`jne` at 0x10de12). Those candidates do not establish packet or
  payload sizes.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10ddd8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10dde9 | store | 2 | `sp -0x38` | `0x108` |
| `mov` at 0x10de01 | load | 4 | `abs 0x1aa578` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10dde1 | `_bzero 0x170f20` | `sp-68` | `0x3e` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10ddf6 | `_bcopy 0x1747fc` | `[sp+4]` | `sp-54` | `0x30` | `?` | `?` | `?` |
| `call` at 0x10de07 | `_write 0x186f70` | `[0x1aa578]` | `sp-68` | `0x3e` | `?` | `?` | `?` |
| `call` at 0x10de25 | `_ffscMsgS 0x11eca0` | `0x10dda1` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10de12 | `ret(_write) != 0x3e` | `ret(_write) == 0x3e` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10de37 | `eax` | `?` |

## `_addrdBroadcastMsg`

- `_addrdBroadcastMsg`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10dc70 to 0x10dcf0.
  Established. The two `_bcopy` sites present leading triples `0x1ee930`, `sp-14`, `0x6` (`call` at
  0x10dc89) and `0x1d1108`, `sp-20`, `0x6` (`call` at 0x10dc99). A two-byte store writes `0x108` at
  `sp -0x8` (`mov` at 0x10dca1). The `_etherOutput` site has leading candidates `[0x1eae00]`,
  `sp-20`, `[sp+4]`, and `0x30` (`call` at 0x10dcb1). No link-layer layout is inferred from these
  calls.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10dc78 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10dca1 | store | 2 | `sp -0x8` | `0x108` |
| `mov` at 0x10dcab | load | 4 | `abs 0x1eae00` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10dc89 | `_bcopy 0x1747fc` | `0x1ee930` | `sp-14` | `0x6` | `ebx` | `esi` | `?` |
| `call` at 0x10dc99 | `_bcopy 0x1747fc` | `0x1d1108` | `sp-20` | `0x6` | `?` | `?` | `?` |
| `call` at 0x10dcb1 | `_etherOutput 0x18e6c8` | `[0x1eae00]` | `sp-20` | `[sp+4]` | `0x30` | `?` | `?` |
| `call` at 0x10dcd5 | `_ffscMsgS 0x11eca0` | `0x10dc4c` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10dcbb | `(ret(_etherOutput) & ret(_etherOutput)) != 0x0` | `(ret(_etherOutput) & ret(_etherOutput)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10dce7 | `eax` | `?` |

## `_addrdExtractIdentity`

- `_addrdExtractIdentity`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10dcf0 to 0x10dd50.
  Established. The first `_bcopy` site presents `[sp+4]+32`, `[sp+8]`, and `0x8` (`call` at
  0x10dd15). The second presents `[sp+4]+40`, `[sp+8]+8`, and `0x4` (`call` at 0x10dd27). Four-byte
  stores write `[[sp+4]+44]` through `[sp+8] +0xc` (`mov` at 0x10dd2f) and `[[sp+4]+28]` through
  `[sp+8] +0x10` (`mov` at 0x10dd35). These offsets alone do not identify fields.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10dcf5 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10dcf8 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10dd00 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10dd04 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10dd2c | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10dd2f | store | 4 | `[sp+8] +0xc` | `[[sp+4]+44]` |
| `mov` at 0x10dd32 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x10dd35 | store | 4 | `[sp+8] +0x10` | `[[sp+4]+28]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10dd15 | `_bcopy 0x1747fc` | `[sp+4]+32` | `[sp+8]` | `0x8` | `?` | `?` | `?` |
| `call` at 0x10dd27 | `_bcopy 0x1747fc` | `[sp+4]+40` | `[sp+8]+8` | `0x4` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x10dd0c | `? <= 0x1 (signed, 32-bit)` | `? > 0x1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10dd40 | `eax` | `[[sp+4]+28]` |

## `_addrdInsertIdentity`

- `_addrdInsertIdentity`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10dd50 to 0x10ddd0.
  Established. The `_bcopy` sites have leading triples `[sp+8]`, `[sp+4]+32`, `0x8` (`call` at
  0x10dd75) and `[sp+8]+8`, `[sp+4]+40`, `0x4` (`call` at 0x10dd87). Four-byte stores write
  `[[sp+8]+12]` through `[sp+4] +0x2c` (`mov` at 0x10dd8f) and `[[sp+8]+16]` through `[sp+4] +0x1c`
  (`mov` at 0x10dd95). The earlier indexed four-byte store has an unknown target and value (`mov` at
  0x10dd64). Field meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10dd55 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10dd58 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10dd60 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10dd64 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10dd8c | load | 4 | `[sp+8] +0xc` | `-` |
| `mov` at 0x10dd8f | store | 4 | `[sp+4] +0x2c` | `[[sp+8]+12]` |
| `mov` at 0x10dd92 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x10dd95 | store | 4 | `[sp+4] +0x1c` | `[[sp+8]+16]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10dd75 | `_bcopy 0x1747fc` | `[sp+8]` | `[sp+4]+32` | `0x8` | `?` | `?` | `?` |
| `call` at 0x10dd87 | `_bcopy 0x1747fc` | `[sp+8]+8` | `[sp+4]+40` | `0x4` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x10dd6c | `? <= 0x1 (signed, 32-bit)` | `? > 0x1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10dda0 | `eax` | `[[sp+8]+16]` |

## `_addrdAssignNewIdentity`

- `_addrdAssignNewIdentity`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e200 to 0x10e300.
  Established. The `_identFindByUniqueID` site presents candidate zero `[sp+4]` (`call` at
  0x10e20d). Two four-byte stores put `0x3` at `sp -0x4c` (`mov` at 0x10e270) and `sp -0x48` (`mov`
  at 0x10e277). The `_addrdBroadcastMsg` result branch is taken when `(ret(_addrdBroadcastMsg) &
  ret(_addrdBroadcastMsg)) == 0x0` (`je` at 0x10e296). The later `_memcpy` call presents
  `ret(_identFindByUniqueID)`, `[sp+4]`, and `0x1c` (`call` at 0x10e2e3), without proving a record
  extent.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e209 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10e270 | store | 4 | `sp -0x4c` | `0x3` |
| `mov` at 0x10e277 | store | 4 | `sp -0x48` | `0x3` |
| `test` at 0x10e2c0 | load | 1 | `abs 0x1da698` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e20d | `_identFindByUniqueID 0x122348` | `[sp+4]` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x10e220 | `_ffscUniqueIDToString 0x125bec` | `[sp+4]` | `sp-28` | `?` | `?` | `?` | `?` |
| `call` at 0x10e236 | `_ffscCompareUniqueIDs 0x1253bc` | `[sp+4]` | `0x1ee558` | `?` | `?` | `?` | `?` |
| `call` at 0x10e243 | `_addrdUpdateLocalIdentity 0x10db70` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e258 | `_ffscMsg 0x11ebd0` | `0x10e188` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e283 | `_addrdInsertIdentity 0x10dd50` | `sp-76` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x10e28c | `_addrdBroadcastMsg 0x10dc70` | `sp-76` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e29d | `_ffscUniqueIDToString 0x125bec` | `ret(_identFindByUniqueID)` | `sp-100` | `?` | `?` | `?` | `?` |
| `call` at 0x10e2ab | `_ffscMsg 0x11ebd0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e2ce | `_ffscMsg 0x11ebd0` | `0x10e1d9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e2d7 | `_identPrint 0x122ae8` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e2e3 | `_memcpy 0x17dda0` | `ret(_identFindByUniqueID)` | `[sp+4]` | `0x1c` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e219 | `(ret(_identFindByUniqueID) & ret(_identFindByUniqueID)) != 0x0` | `(ret(_identFindByUniqueID) & ret(_identFindByUniqueID)) == 0x0` |
| `jne` at 0x10e240 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `je` at 0x10e24d | `(ret(_addrdUpdateLocalIdentity) & ret(_addrdUpdateLocalIdentity)) == 0x0` | `(ret(_addrdUpdateLocalIdentity) & ret(_addrdUpdateLocalIdentity)) != 0x0` |
| `je` at 0x10e296 | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` |
| `je` at 0x10e2c7 | `(([0x1da698]:1 & 0xff) & 0x1) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e2f3 | `eax` | `?` |

## `_addrdProcessIdentityMsg`

- `_addrdProcessIdentityMsg`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e8a0 to 0x10e9d0.
  Established. The `_addrdExtractIdentity` site has leading candidates `[sp+4]` and `sp-32` (`call`
  at 0x10e8b0). The comparison-result branch is taken when `(ret(_ffscCompareUniqueIDs) &
  ret(_ffscCompareUniqueIDs)) != 0x0` (`jne` at 0x10e8c8). A later branch falls through when
  `[0x1da6e4] != 0x0` (`je` at 0x10e906), reaching `_addrdStartElectionTimer` with unknown
  candidates (`call` at 0x10e908); its name does not establish a timer restart effect.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e8a8 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x10e8ff | load | 4 | `abs 0x1da6e4` | `-` |
| `test` at 0x10e911 | load | 1 | `abs 0x1da698` | `-` |
| `cmp` at 0x10e942 | load | 4 | `abs 0x1eea94` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e8b0 | `_addrdExtractIdentity 0x10dcf0` | `[sp+4]` | `sp-32` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10e8be | `_ffscCompareUniqueIDs 0x1253bc` | `sp-32` | `0x1ee558` | `?` | `?` | `?` | `?` |
| `call` at 0x10e8cf | `_ffscUniqueIDToString 0x125bec` | `sp-32` | `sp-56` | `?` | `?` | `?` | `?` |
| `call` at 0x10e8dd | `_ffscMsg 0x11ebd0` | `0x10e82c` | `sp-56` | `?` | `?` | `?` | `?` |
| `call` at 0x10e8f1 | `_identUpdate 0x1229b8` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e908 | `_addrdStartElectionTimer 0x10ee60` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e92e | `_ffscMsg 0x11ebd0` | `0x10e883` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e93a | `_identPrint 0x122ae8` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e94b | `_addrdResolveConflicts 0x10ec90` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e959 | `_ffscMsg 0x11ebd0` | `0x10e5d5` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e8c8 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `jl` at 0x10e8fd | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10e906 | `[0x1da6e4] == 0x0` | `[0x1da6e4] != 0x0` |
| `jne` at 0x10e90f | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) != 0x0` | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) == 0x0` |
| `je` at 0x10e918 | `(([0x1da698]:1 & 0xff) & 0x1) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x10e921 | `(ret(_identUpdate) & ret(_identUpdate)) != 0x0` | `(ret(_identUpdate) & ret(_identUpdate)) == 0x0` |
| `je` at 0x10e949 | `[0x1eea94] == 0x0` | `[0x1eea94] != 0x0` |
| `je` at 0x10e952 | `(ret(_addrdResolveConflicts) & ret(_addrdResolveConflicts)) == 0x0` | `(ret(_addrdResolveConflicts) & ret(_addrdResolveConflicts)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e968 | `eax` | `?` |

## `_addrdProcessQueryMsg`

- `_addrdProcessQueryMsg`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e9d0 to 0x10eb40.
  Established. The `_clock_gettime` call presents leading candidates `0x0` and `0x1eae10` (`call` at
  0x10e9e2). The `_taskDelay` site presents candidate zero `0x3` (`call` at 0x10ea8c). Four-byte
  stores write `0x3` to `sp -0x34` (`mov` at 0x10ea94), `0x1` to `sp -0x30` (`mov` at 0x10ea9b), and
  `0x1` to absolute `0x1da6e4` (`mov` at 0x10eac9). No time unit or message-token identity is
  assigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e9d8 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x10ea43 | load | 4 | `abs 0x1da6e4` | `-` |
| `test` at 0x10ea59 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x10ea94 | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10ea9b | store | 4 | `sp -0x30` | `0x1` |
| `mov` at 0x10eac9 | store | 4 | `abs 0x1da6e4` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e9e2 | `_clock_gettime 0x17981c` | `0x0` | `0x1eae10` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10e9ef | `_addrdExtractIdentity 0x10dcf0` | `[sp+4]` | `sp-80` | `?` | `?` | `?` | `?` |
| `call` at 0x10e9fd | `_ffscCompareUniqueIDs 0x1253bc` | `sp-80` | `0x1ee558` | `?` | `?` | `?` | `?` |
| `call` at 0x10ea0e | `_ffscUniqueIDToString 0x125bec` | `sp-80` | `sp-104` | `?` | `?` | `?` | `?` |
| `call` at 0x10ea1c | `_ffscMsg 0x11ebd0` | `0x10e969` | `sp-104` | `?` | `?` | `?` | `?` |
| `call` at 0x10ea31 | `_identUpdate 0x1229b8` | `sp-80` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ea4c | `_addrdStartElectionTimer 0x10ee60` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ea76 | `_ffscMsg 0x11ebd0` | `0x10e9b2` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ea82 | `_identPrint 0x122ae8` | `sp-80` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ea8c | `_taskDelay 0x170b80` | `0x3` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10eaab | `_addrdInsertIdentity 0x10dd50` | `sp-52` | `0x1ee558` | `?` | `?` | `?` | `?` |
| `call` at 0x10eab4 | `_addrdBroadcastMsg 0x10dc70` | `sp-52` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10eac0 | `_addrdStartElectionTimer 0x10ee60` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10ea07 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `jl` at 0x10ea3d | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10ea4a | `[0x1da6e4] == 0x0` | `[0x1da6e4] != 0x0` |
| `jne` at 0x10ea53 | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) != 0x0` | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) == 0x0` |
| `je` at 0x10ea60 | `(([0x1da698]:1 & 0xff) & 0x1) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x10ea69 | `(ret(_identUpdate) & ret(_identUpdate)) != 0x0` | `(ret(_identUpdate) & ret(_identUpdate)) == 0x0` |
| `jne` at 0x10eabe | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` |
| `jne` at 0x10eac7 | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) != 0x0` | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10eaed | `eax` | `?` |

## `_addrdProcessAssignMsg`

- `_addrdProcessAssignMsg`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e4f0 to 0x10e600.
  Established. The `_ffscCompareUniqueIDs` site presents `[sp+4]+32` and `0x1ee558` as its leading
  candidates (`call` at 0x10e511); its result branch is taken when `(ret(_ffscCompareUniqueIDs) &
  ret(_ffscCompareUniqueIDs)) == 0x0` (`je` at 0x10e51b), reaching `_addrdUpdateLocalIdentity` with
  candidate zero `sp-32` (`call` at 0x10e531). The common exit reports `0x0` in `eax` (`ret` at
  0x10e570). This literal does not establish that an update occurred.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e4f8 | load | 4 | `sp +0x4` | `-` |
| `test` at 0x10e51d | load | 1 | `abs 0x1da698` | `-` |
| `test` at 0x10e54a | load | 1 | `abs 0x1da698` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e500 | `_addrdExtractIdentity 0x10dcf0` | `[sp+4]` | `sp-32` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10e511 | `_ffscCompareUniqueIDs 0x1253bc` | `[sp+4]+32` | `0x1ee558` | `?` | `?` | `?` | `?` |
| `call` at 0x10e531 | `_addrdUpdateLocalIdentity 0x10db70` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e542 | `_ffscMsg 0x11ebd0` | `0x10e4aa` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e558 | `_ffscMsg 0x11ebd0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e561 | `_identPrint 0x122ae8` | `sp-32` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10e51b | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` |
| `je` at 0x10e524 | `(([0x1da698]:1 & 0xff) & 0x1) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x1) != 0x0` |
| `je` at 0x10e53b | `(ret(_addrdUpdateLocalIdentity) & ret(_addrdUpdateLocalIdentity)) == 0x0` | `(ret(_addrdUpdateLocalIdentity) & ret(_addrdUpdateLocalIdentity)) != 0x0` |
| `je` at 0x10e551 | `(([0x1da698]:1 & 0xff) & 0x1) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e570 | `eax` | `0x0` |

## `_addrdResolveConflicts`

- `_addrdResolveConflicts`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ec90 to 0x10ed10.
  Established. The opening branch is taken when `[0x1eea94] != 0x0` (`jne` at 0x10ec9a). The
  `_identScanForConflicts` site has leading candidates `0x10ed10` and `0x10edb0` (`call` at
  0x10ecba); its result branch falls through when `(ret(_identScanForConflicts) &
  ret(_identScanForConflicts)) == 0x0` (`jne` at 0x10ecc4), reaching the literal-zero exit (`ret` at
  0x10eccb). The candidate addresses do not prove calls through them.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10ec93 | load | 4 | `abs 0x1eea94` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10eca1 | `_ffscMsg 0x11ebd0` | `0x10ec2c` | `ebp` | `?` | `?` | `?` | `?` |
| `call` at 0x10ecba | `_identScanForConflicts 0x122728` | `0x10ed10` | `0x10edb0` | `ebp` | `?` | `?` | `?` |
| `call` at 0x10ecd5 | `_ffscMsg 0x11ebd0` | `0x10ec5c` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10ec9a | `[0x1eea94] != 0x0` | `[0x1eea94] == 0x0` |
| `jne` at 0x10ecc4 | `(ret(_identScanForConflicts) & ret(_identScanForConflicts)) != 0x0` | `(ret(_identScanForConflicts) & ret(_identScanForConflicts)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ecae | `eax` | `0xffffffff` |
| `ret` at 0x10eccb | `eax` | `0x0` |
| `ret` at 0x10ece2 | `eax` | `0xffffffff` |

## `_addrdResolveIPAddrConflict`

- `_addrdResolveIPAddrConflict`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ed10 to 0x10edb0.
  Established. The `_memcpy` call presents `sp-32`, unknown, and `0x1c` as its first three
  candidates (`call` at 0x10ed47). The `_addrdFindUnusedIPAddr` site has candidate zero `sp-24`
  (`call` at 0x10ed56); its result branch falls through when `(ret(_addrdFindUnusedIPAddr) &
  ret(_addrdFindUnusedIPAddr)) == 0x0` (`jne` at 0x10ed60), reaching `_addrdAssignNewIdentity` with
  candidate zero `sp-32` (`call` at 0x10ed63). Conflict resolution semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10ed18 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10ed1b | load | 4 | `sp +0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ed20 | `_ffscCompareUniqueIDs 0x1253bc` | `[sp+4]` | `[sp+8]` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10ed47 | `_memcpy 0x17dda0` | `sp-32` | `?` | `0x1c` | `?` | `?` | `?` |
| `call` at 0x10ed56 | `_addrdFindUnusedIPAddr 0x10e400` | `sp-24` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ed63 | `_addrdAssignNewIdentity 0x10e200` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ed75 | `_ffscMsg 0x11ebd0` | `0x10ece3` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x10ed2a | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x10ed60 | `(ret(_addrdFindUnusedIPAddr) & ret(_addrdFindUnusedIPAddr)) != 0x0` | `(ret(_addrdFindUnusedIPAddr) & ret(_addrdFindUnusedIPAddr)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ed87 | `eax` | `?` |

## `_addrdFindUnusedIPAddr`

- `_addrdFindUnusedIPAddr`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e400 to 0x10e460.
  Established. The four-byte load reads absolute `0x1dab74` (`mov` at 0x10e412). The
  `_inet_makeaddr_b` call has leading candidates `[0x1dab74]`, unknown, and `[sp+4]` (`call` at
  0x10e418). The `_identFindByIPAddr` site presents candidate zero `[sp+4]` (`call` at 0x10e421).
  The backward branch is taken when `? <= 0xfffffe (signed, 32-bit)` (`jle` at 0x10e447). The
  unknown selector prevents an established address-search range.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e405 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10e412 | load | 4 | `abs 0x1dab74` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e418 | `_inet_makeaddr_b 0x186620` | `[0x1dab74]` | `?` | `[sp+4]` | `?` | `?` | `?` |
| `call` at 0x10e421 | `_identFindByIPAddr 0x122238` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e42b | `(ret(_identFindByIPAddr) & ret(_identFindByIPAddr)) != 0x0` | `(ret(_identFindByIPAddr) & ret(_identFindByIPAddr)) == 0x0` |
| `jle` at 0x10e447 | `? <= 0xfffffe (signed, 32-bit)` | `? > 0xfffffe (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e456 | `eax` | `?` |

## `_addrdUpdateLocalIdentity`

- `_addrdUpdateLocalIdentity`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10db70 to 0x10dc70.
  Established. The branch falls through when `[0x1ee560] != [[sp+4]+8]` (`je` at 0x10db86), reaching
  `_identSetIPAddr` with candidate zero `[sp+4]+8` (`call` at 0x10db8c). Later four-byte stores
  write `0x3` at `sp -0x34` (`mov` at 0x10dc19) and `0x1` at `sp -0x30` (`mov` at 0x10dc20). The
  `_addrdBroadcastMsg` site presents candidate zero `sp-52` (`call` at 0x10dc35). Setter names do
  not establish network or identity changes.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10db78 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10db7d | load | 4 | `[sp+4] +0x8` | `-` |
| `cmp` at 0x10db80 | load | 4 | `abs 0x1ee560` | `-` |
| `mov` at 0x10db99 | load | 4 | `[sp+4] +0xc` | `-` |
| `cmp` at 0x10db9c | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x10dba4 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10dbb5 | load | 4 | `[sp+4] +0x10` | `-` |
| `cmp` at 0x10dbb8 | load | 4 | `abs 0x1ee568` | `-` |
| `mov` at 0x10dbc0 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10dbd4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10dbd8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10dc19 | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10dc20 | store | 4 | `sp -0x30` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10db8c | `_identSetIPAddr 0x122cd8` | `[sp+4]+8` | `ebx` | `esi` | `?` | `?` | `?` |
| `call` at 0x10dba8 | `_identSetRackID 0x122ea8` | `[[sp+4]+12]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dbc4 | `_identSetCapabilities 0x122bf8` | `[[sp+4]+16]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dc04 | `_identSetModuleNums 0x122dc8` | `[sp+4]+20` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dc0d | `_identUpdate 0x1229b8` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dc2c | `_addrdInsertIdentity 0x10dd50` | `sp-52` | `[sp+4]` | `?` | `?` | `0x3` | `0x1` |
| `call` at 0x10dc35 | `_addrdBroadcastMsg 0x10dc70` | `sp-52` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10db86 | `[0x1ee560] == [[sp+4]+8]` | `[0x1ee560] != [[sp+4]+8]` |
| `je` at 0x10dba2 | `[0x1ee564] == [[sp+4]+12]` | `[0x1ee564] != [[sp+4]+12]` |
| `je` at 0x10dbbe | `[0x1ee568] == [[sp+4]+16]` | `[0x1ee568] != [[sp+4]+16]` |
| `jne` at 0x10dbe1 | `Z == 0` | `not (Z == 0)` |
| `jbe` at 0x10dbe7 | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |
| `jne` at 0x10dbeb | `Z == 0` | `not (Z == 0)` |
| `jl` at 0x10dc17 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10dc3c | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10dc4b | `eax` | `?` |
