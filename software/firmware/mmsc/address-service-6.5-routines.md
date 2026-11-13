# MMSC address service in IRIX 6.5

Read from `6.5/usr/cpu/firmware/mmscfw.bin` (IRIX 6.5).
Image SHA-256: `2017f743fb7360a160f4252b69569b7f9713d9efad82719cfb2fd82538e37ff8`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred end and can include literals
after returns. Statements below concern sites reached by known control-flow edges.
One computed transfer remains unresolved in `_addrd`; its selector arms are excluded from the facts.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/address-service-6.5-facts.txt) include all tool sections.
The decoder stops within trailing literals in three spans. The remaining raw bytes,
read through each exclusive span end, are:

| Span | Raw-byte start | Exclusive end | Bytes |
|---|---|---|---|
| `_addrdInsertIdentity` | `0x10e019` | `0x10e020` | `69 6c 65 64 00 90 90` |
| `_addrdProcessAssignMsg` | `0x10e88c` | `0x10e890` | `65 64 00 90` |
| `_addrdResolveConflicts` | `0x10ef9b` | `0x10efa0` | `00 90 90 90 90` |

## `_addrdInit`

- `_addrdInit`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10db90 to 0x10dd20.
  Established. The `_pipeDevCreate` call presents leading candidates `0x10dac2`, `0xb`, and `0x3e`
  (`call` at 0x10db9c); its branch takes `(ret(_pipeDevCreate) & ret(_pipeDevCreate)) == 0x0` and
  falls through for nonzero (`je` at 0x10dba6). Later paths include four-byte stores of `ret(_open)`
  at absolute `0x1af0f0` (`mov` at 0x10dbe1), `ret(_wdCreate)` at absolute `0x1efd08` (`mov` at
  0x10dc05), and `ret(_semMCreate)` at absolute `0x1f373c` (`mov` at 0x10dc47). The service startup
  policy remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10dbad | load | 4 | `ret(___errno) +0x0` | `-` |
| `mov` at 0x10dbe1 | store | 4 | `abs 0x1af0f0` | `ret(_open)` |
| `mov` at 0x10dc05 | store | 4 | `abs 0x1efd08` | `ret(_wdCreate)` |
| `mov` at 0x10dc47 | store | 4 | `abs 0x1f373c` | `ret(_semMCreate)` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10db9c | `_pipeDevCreate 0x190384` | `0x10dac2` | `0xb` | `0x3e` | `ebp` | `?` | `?` |
| `call` at 0x10dba8 | `___errno 0x173998` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dbbb | `_ffscMsgS 0x120634` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dbd9 | `_open 0x18b8a8` | `0x10dac2` | `0x1` | `0x0` | `?` | `?` | `?` |
| `call` at 0x10dc00 | `_wdCreate 0x177684` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dc13 | `_ffscMsg 0x120504` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dc35 | `_lstInit 0x192940` | `0x1dfa08` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dc3f | `_semMCreate 0x17a130` | `0x1` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dc87 | `_taskSpawn 0x173b20` | `0x10db5f` | `0x19` | `0x0` | `0x4e20` | `0x10e170` | `0x0` |
| `call` at 0x10dc99 | `_etherInputHookAdd 0x193380` | `0x10e640` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dcb5 | `_ffscMsg 0x120504` | `0x10db66` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10dba6 | `(ret(_pipeDevCreate) & ret(_pipeDevCreate)) == 0x0` | `(ret(_pipeDevCreate) & ret(_pipeDevCreate)) != 0x0` |
| `je` at 0x10dbb4 | `[ret(___errno)] == 0xd0006` | `[ret(___errno)] != 0xd0006` |
| `jge` at 0x10dbe8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x10dc0c | `(ret(_wdCreate) & ret(_wdCreate)) != 0x0` | `(ret(_wdCreate) & ret(_wdCreate)) == 0x0` |
| `jne` at 0x10dc4e | `(ret(_semMCreate) & ret(_semMCreate)) != 0x0` | `(ret(_semMCreate) & ret(_semMCreate)) == 0x0` |
| `je` at 0x10dc92 | `ret(_taskSpawn) == 0xffffffff` | `ret(_taskSpawn) != 0xffffffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10dbc8 | `eax` | `0xffffffff` |
| `ret` at 0x10dc20 | `eax` | `0xffffffff` |
| `ret` at 0x10dca3 | `eax` | `0x0` |
| `ret` at 0x10dcc2 | `eax` | `0xffffffff` |

## `_addrd`

- `_addrd`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e170 to 0x10e490.
  Established. Calling `_read` has leading candidates `ret(_open)`, `sp-68`, and `0x3e` (`call` at
  0x10e267). A later branch takes `ret(_read) == 0x3e` and falls through for inequality (`je` at
  0x10e287). The computed transfer has an unknown effective target (`jmp` at 0x10e301); its selector
  arms are not credited with effects here. The receive-loop protocol and dispatch outcomes remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e1c3 | store | 4 | `sp -0x8c` | `?` |
| `mov` at 0x10e1d9 | load | 4 | `sp -0x8c` | `-` |
| `mov` at 0x10e1df | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10e1e5 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10e1e9 | store | 4 | `sp -0x88` | `0x1` |
| `rep movsd` at 0x10e200 | store | 4 | `(unknown)` | `?` |
| `rep movsd` at 0x10e200 | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x10e222 | load | 4 | `ret(___errno) +0x0` | `-` |
| `dec` at 0x10e243 | load | 4 | `sp -0x88` | `-` |
| `dec` at 0x10e243 | store | 4 | `sp -0x88` | `?` |
| `mov` at 0x10e253 | load | 4 | `(unknown)` | `-` |
| `dec` at 0x10e2b6 | load | 4 | `sp -0x88` | `-` |
| `dec` at 0x10e2b6 | store | 4 | `sp -0x88` | `?` |
| `mov` at 0x10e2bf | load | 4 | `sp -0x36` | `-` |
| `mov` at 0x10e2cc | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x10e2da | store | 4 | `sp -0x36` | `0x2` |
| `mov` at 0x10e2e0 | load | 4 | `sp -0x36` | `-` |
| `mov` at 0x10e2e7 | store | 4 | `sp -0x1a` | `0x0` |
| `mov` at 0x10e2ee | store | 4 | `sp -0x36` | `0x3` |
| `mov` at 0x10e2f4 | load | 4 | `sp -0x32` | `-` |
| `jmp` at 0x10e301 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10e3b0 | load | 4 | `sp -0x32` | `-` |
| `cmp` at 0x10e3c1 | load | 4 | `sp -0x88` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e185 | `_open 0x18b8a8` | `0x10dac2` | `0x0` | `0x0` | `ebx` | `esi` | `edi` |
| `call` at 0x10e198 | `_ffscMsgS 0x120634` | `0x10e088` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e1b6 | `_bzero 0x174e40` | `sp-100` | `0x20` | `?` | `?` | `?` | `?` |
| `call` at 0x10e211 | `_select 0x176ee8` | `0x100` | `sp-132` | `0x0` | `0x0` | `0x0` | `?` |
| `call` at 0x10e21d | `___errno 0x173998` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e232 | `_ffscMsgS 0x120634` | `0x10e0a4` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e23b | `_close 0x18ba48` | `ret(_open)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e267 | `_read 0x18bac8` | `ret(_open)` | `sp-68` | `0x3e` | `?` | `?` | `?` |
| `call` at 0x10e278 | `_ffscMsgS 0x120634` | `0x10e0bb` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e291 | `_ffscMsg 0x120504` | `0x10e0f2` | `ret(_read)` | `0x3e` | `?` | `?` | `?` |
| `call` at 0x10e2a5 | `_ffscMsg 0x120504` | `0x10e0da` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e2ae | `_close 0x18ba48` | `ret(_open)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e3b9 | `_ffscMsg 0x120504` | `0x10e13f` | `[sp-50]` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x10e191 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jge` at 0x10e21b | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x10e227 | `[ret(___errno)] == 0x4` | `[ret(___errno)] != 0x4` |
| `jae` at 0x10e25a | `C == 0` | `not (C == 0)` |
| `jge` at 0x10e271 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x10e282 | `(ret(_read) & ret(_read)) == 0x0` | `(ret(_read) & ret(_read)) != 0x0` |
| `je` at 0x10e287 | `ret(_read) == 0x3e` | `ret(_read) != 0x3e` |
| `ja` at 0x10e2c5 | `[sp-54] > 0x1 (unsigned, 32-bit)` | `[sp-54] <= 0x1 (unsigned, 32-bit)` |
| `jle` at 0x10e2d8 | `? <= 0x1 (signed, 32-bit)` | `? > 0x1 (signed, 32-bit)` |
| `ja` at 0x10e2e5 | `[sp-54] > 0x2 (unsigned, 32-bit)` | `[sp-54] <= 0x2 (unsigned, 32-bit)` |
| `ja` at 0x10e2fb | `? > 0x6 (unsigned, 32-bit)` | `? <= 0x6 (unsigned, 32-bit)` |
| `jg` at 0x10e3c8 | `[sp-136] > 0x0 (signed, 32-bit)` | `[sp-136] <= 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e3dc | `eax` | `?` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x10e301).

## `_addrdEtherInputHook`

- `_addrdEtherInputHook`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e640 to 0x10e690.
  Established. A two-byte load reads `[sp+8] +0xc` (`mov` at 0x10e64a). The branch takes
  `([[sp+8]+12]:2 & 0xffff) != 0x801` and falls through for equality (`jne` at 0x10e652); its
  fall-through includes `_write` with leading candidates `[0x1af0f0]`, `[sp+8]`, and `[sp+12]`
  (`call` at 0x10e65c). The subsequent branch takes `ret(_write) == [sp+12]` and falls through for
  inequality (`je` at 0x10e666). The accepted payload and header-field meanings are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e644 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10e647 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10e64a | load | 2 | `[sp+8] +0xc` | `-` |
| `mov` at 0x10e656 | load | 4 | `abs 0x1af0f0` | `-` |
| `mov` at 0x10e682 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e65c | `_write 0x18bae8` | `[0x1af0f0]` | `[sp+8]` | `[sp+12]` | `ebx` | `ebp` | `?` |
| `call` at 0x10e66d | `_ffscMsgS 0x120634` | `0x10e61e` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e652 | `([[sp+8]+12]:2 & 0xffff) != 0x801` | `([[sp+8]+12]:2 & 0xffff) == 0x801` |
| `je` at 0x10e666 | `ret(_write) == [sp+12]` | `ret(_write) != [sp+12]` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e688 | `eax` | `?` |

## `_addrdLocalMsg`

- `_addrdLocalMsg`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e020 to 0x10e170.
  Established. The stack receives two-byte `0x108` at `sp -0x38` (`mov` at 0x10e039). `_bcopy` has
  leading candidates `[sp+4]`, `sp-54`, and `0x30` (`call` at 0x10e046). The `_write` candidates
  begin `[0x1af0f0]`, `sp-68`, and `0x3e` (`call` at 0x10e057); its result branch takes `ret(_write)
  != 0x3e` and falls through for equality (`jne` at 0x10e062). The record layout beyond these sites
  is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e028 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10e039 | store | 2 | `sp -0x38` | `0x108` |
| `mov` at 0x10e051 | load | 4 | `abs 0x1af0f0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e031 | `_bzero 0x174e40` | `sp-68` | `0x3e` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10e046 | `_bcopy 0x179374` | `[sp+4]` | `sp-54` | `0x30` | `?` | `?` | `?` |
| `call` at 0x10e057 | `_write 0x18bae8` | `[0x1af0f0]` | `sp-68` | `0x3e` | `?` | `?` | `?` |
| `call` at 0x10e075 | `_ffscMsgS 0x120634` | `0x10dff1` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e062 | `ret(_write) != 0x3e` | `ret(_write) == 0x3e` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e087 | `eax` | `?` |

## `_addrdBroadcastMsg`

- `_addrdBroadcastMsg`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10dec0 to 0x10df40.
  Established. The first `_bcopy` call presents leading candidates `0x1f3830`, `sp-14`, and `0x6`
  (`call` at 0x10ded9). The next `_bcopy` call instead begins `0x1d5cd0`, `sp-20`, and `0x6` (`call`
  at 0x10dee9). A two-byte store sets `sp -0x8` to `0x108` (`mov` at 0x10def1). Calling
  `_etherOutput` has leading candidates `[0x1efcf4]`, `sp-20`, `[sp+4]`, and `0x30` (`call` at
  0x10df01). The destination addressing and payload semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10dec8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10def1 | store | 2 | `sp -0x8` | `0x108` |
| `mov` at 0x10defb | load | 4 | `abs 0x1efcf4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ded9 | `_bcopy 0x179374` | `0x1f3830` | `sp-14` | `0x6` | `ebx` | `esi` | `?` |
| `call` at 0x10dee9 | `_bcopy 0x179374` | `0x1d5cd0` | `sp-20` | `0x6` | `?` | `?` | `?` |
| `call` at 0x10df01 | `_etherOutput 0x193240` | `[0x1efcf4]` | `sp-20` | `[sp+4]` | `0x30` | `?` | `?` |
| `call` at 0x10df25 | `_ffscMsgS 0x120634` | `0x10de9c` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10df0b | `(ret(_etherOutput) & ret(_etherOutput)) != 0x0` | `(ret(_etherOutput) & ret(_etherOutput)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10df37 | `eax` | `?` |

## `_addrdExtractIdentity`

- `_addrdExtractIdentity`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10df40 to 0x10dfa0.
  Established. The indexed four-byte store has unknown effective target and value (`mov` at
  0x10df54). Two `_bcopy` calls present leading candidates `[sp+4]+32`, `[sp+8]`, `0x8` (`call` at
  0x10df65) and `[sp+4]+40`, `[sp+8]+8`, `0x4` (`call` at 0x10df77). Later four-byte stores put
  `[[sp+4]+44]` at `[sp+8] +0xc` (`mov` at 0x10df7f) and `[[sp+4]+28]` at `[sp+8] +0x10` (`mov` at
  0x10df85). The translated record fields are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10df45 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10df48 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10df50 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10df54 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10df7c | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10df7f | store | 4 | `[sp+8] +0xc` | `[[sp+4]+44]` |
| `mov` at 0x10df82 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x10df85 | store | 4 | `[sp+8] +0x10` | `[[sp+4]+28]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10df65 | `_bcopy 0x179374` | `[sp+4]+32` | `[sp+8]` | `0x8` | `?` | `?` | `?` |
| `call` at 0x10df77 | `_bcopy 0x179374` | `[sp+4]+40` | `[sp+8]+8` | `0x4` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x10df5c | `? <= 0x1 (signed, 32-bit)` | `? > 0x1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10df90 | `eax` | `[[sp+4]+28]` |

## `_addrdInsertIdentity`

- `_addrdInsertIdentity`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10dfa0 to 0x10e020.
  Established. The indexed four-byte store leaves both target and value unknown (`mov` at 0x10dfb4).
  The `_bcopy` candidates begin `[sp+8]`, `[sp+4]+32`, `0x8` (`call` at 0x10dfc5), followed by
  `[sp+8]+8`, `[sp+4]+40`, `0x4` at the second call (`call` at 0x10dfd7). Four-byte stores place
  `[[sp+8]+12]` at `[sp+4] +0x2c` (`mov` at 0x10dfdf) and `[[sp+8]+16]` at `[sp+4] +0x1c` (`mov` at
  0x10dfe5). The inverse record mapping remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10dfa5 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10dfa8 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10dfb0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10dfb4 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10dfdc | load | 4 | `[sp+8] +0xc` | `-` |
| `mov` at 0x10dfdf | store | 4 | `[sp+4] +0x2c` | `[[sp+8]+12]` |
| `mov` at 0x10dfe2 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x10dfe5 | store | 4 | `[sp+4] +0x1c` | `[[sp+8]+16]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10dfc5 | `_bcopy 0x179374` | `[sp+8]` | `[sp+4]+32` | `0x8` | `?` | `?` | `?` |
| `call` at 0x10dfd7 | `_bcopy 0x179374` | `[sp+8]+8` | `[sp+4]+40` | `0x4` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x10dfbc | `? <= 0x1 (signed, 32-bit)` | `? > 0x1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10dff0 | `eax` | `[[sp+8]+16]` |

## `_addrdAssignNewIdentity`

- `_addrdAssignNewIdentity`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e490 to 0x10e590.
  Established. The `_identFindByUniqueID` call presents leading candidate `[sp+4]` (`call` at
  0x10e49d); its branch takes `(ret(_identFindByUniqueID) & ret(_identFindByUniqueID)) != 0x0` and
  falls through for zero (`jne` at 0x10e4a9). Later paths include four-byte stores of `0x3` at `sp
  -0x4c` (`mov` at 0x10e500) and `sp -0x48` (`mov` at 0x10e507). `_memcpy` has leading candidates
  `ret(_identFindByUniqueID)`, `[sp+4]`, and `0x1c` on a later route (`call` at 0x10e573). The
  identity-assignment policy remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e499 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10e500 | store | 4 | `sp -0x4c` | `0x3` |
| `mov` at 0x10e507 | store | 4 | `sp -0x48` | `0x3` |
| `test` at 0x10e550 | load | 1 | `abs 0x1df500` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e49d | `_identFindByUniqueID 0x1243b8` | `[sp+4]` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x10e4b0 | `_ffscUniqueIDToString 0x127758` | `[sp+4]` | `sp-28` | `?` | `?` | `?` | `?` |
| `call` at 0x10e4c6 | `_ffscCompareUniqueIDs 0x126f28` | `[sp+4]` | `0x1f3448` | `?` | `?` | `?` | `?` |
| `call` at 0x10e4d3 | `_addrdUpdateLocalIdentity 0x10ddc0` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e4e8 | `_ffscMsg 0x120504` | `0x10e418` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e513 | `_addrdInsertIdentity 0x10dfa0` | `sp-76` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x10e51c | `_addrdBroadcastMsg 0x10dec0` | `sp-76` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e52d | `_ffscUniqueIDToString 0x127758` | `ret(_identFindByUniqueID)` | `sp-100` | `?` | `?` | `?` | `?` |
| `call` at 0x10e53b | `_ffscMsg 0x120504` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e55e | `_ffscMsg 0x120504` | `0x10e469` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e567 | `_identPrint 0x124b58` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e573 | `_memcpy 0x182918` | `ret(_identFindByUniqueID)` | `[sp+4]` | `0x1c` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e4a9 | `(ret(_identFindByUniqueID) & ret(_identFindByUniqueID)) != 0x0` | `(ret(_identFindByUniqueID) & ret(_identFindByUniqueID)) == 0x0` |
| `jne` at 0x10e4d0 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `je` at 0x10e4dd | `(ret(_addrdUpdateLocalIdentity) & ret(_addrdUpdateLocalIdentity)) == 0x0` | `(ret(_addrdUpdateLocalIdentity) & ret(_addrdUpdateLocalIdentity)) != 0x0` |
| `je` at 0x10e526 | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` |
| `je` at 0x10e557 | `(([0x1df500]:1 & 0xff) & 0x1) == 0x0` | `(([0x1df500]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e583 | `eax` | `?` |

## `_addrdProcessIdentityMsg`

- `_addrdProcessIdentityMsg`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10eb30 to 0x10ec60.
  Established. The `_addrdExtractIdentity` candidates begin `[sp+4]` and `sp-32` (`call` at
  0x10eb40). After a four-byte read of absolute `0x1df554` (`cmp` at 0x10eb8f), the branch takes
  `[0x1df554] == 0x0` and falls through for nonzero (`je` at 0x10eb96); its fall-through calls
  `_addrdStartElectionTimer` with unknown candidate values (`call` at 0x10eb98). The later
  conflict-resolution call also has unknown candidates (`call` at 0x10ebdb). The global-state
  meanings and timer policy remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10eb38 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x10eb8f | load | 4 | `abs 0x1df554` | `-` |
| `test` at 0x10eba1 | load | 1 | `abs 0x1df500` | `-` |
| `cmp` at 0x10ebd2 | load | 4 | `abs 0x1f398c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10eb40 | `_addrdExtractIdentity 0x10df40` | `[sp+4]` | `sp-32` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10eb4e | `_ffscCompareUniqueIDs 0x126f28` | `sp-32` | `0x1f3448` | `?` | `?` | `?` | `?` |
| `call` at 0x10eb5f | `_ffscUniqueIDToString 0x127758` | `sp-32` | `sp-56` | `?` | `?` | `?` | `?` |
| `call` at 0x10eb6d | `_ffscMsg 0x120504` | `0x10eabc` | `sp-56` | `?` | `?` | `?` | `?` |
| `call` at 0x10eb81 | `_identUpdate 0x124a28` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10eb98 | `_addrdStartElectionTimer 0x10f0f0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ebbe | `_ffscMsg 0x120504` | `0x10eb13` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ebca | `_identPrint 0x124b58` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ebdb | `_addrdResolveConflicts 0x10ef20` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ebe9 | `_ffscMsg 0x120504` | `0x10e865` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10eb58 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `jl` at 0x10eb8d | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10eb96 | `[0x1df554] == 0x0` | `[0x1df554] != 0x0` |
| `jne` at 0x10eb9f | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) != 0x0` | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) == 0x0` |
| `je` at 0x10eba8 | `(([0x1df500]:1 & 0xff) & 0x1) == 0x0` | `(([0x1df500]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x10ebb1 | `(ret(_identUpdate) & ret(_identUpdate)) != 0x0` | `(ret(_identUpdate) & ret(_identUpdate)) == 0x0` |
| `je` at 0x10ebd9 | `[0x1f398c] == 0x0` | `[0x1f398c] != 0x0` |
| `je` at 0x10ebe2 | `(ret(_addrdResolveConflicts) & ret(_addrdResolveConflicts)) == 0x0` | `(ret(_addrdResolveConflicts) & ret(_addrdResolveConflicts)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ebf8 | `eax` | `?` |

## `_addrdProcessQueryMsg`

- `_addrdProcessQueryMsg`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ec60 to 0x10edd0.
  Established. Calling `_clock_gettime` presents leading candidates `0x0` and `0x1efd00` (`call` at
  0x10ec72). A later route calls `_taskDelay` with leading candidate `0x3` (`call` at 0x10ed1c).
  Four-byte stores on that route set `sp -0x34` to `0x3` (`mov` at 0x10ed24) and `sp -0x30` to `0x1`
  (`mov` at 0x10ed2b). The final result branch takes `(ret(_addrdStartElectionTimer) &
  ret(_addrdStartElectionTimer)) != 0x0` and falls through for zero (`jne` at 0x10ed57); its
  fall-through stores four-byte `0x1` at absolute `0x1df554` (`mov` at 0x10ed59). The delay units
  and response-record meaning remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10ec68 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x10ecd3 | load | 4 | `abs 0x1df554` | `-` |
| `test` at 0x10ece9 | load | 1 | `abs 0x1df500` | `-` |
| `mov` at 0x10ed24 | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10ed2b | store | 4 | `sp -0x30` | `0x1` |
| `mov` at 0x10ed59 | store | 4 | `abs 0x1df554` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ec72 | `_clock_gettime 0x17e394` | `0x0` | `0x1efd00` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10ec7f | `_addrdExtractIdentity 0x10df40` | `[sp+4]` | `sp-80` | `?` | `?` | `?` | `?` |
| `call` at 0x10ec8d | `_ffscCompareUniqueIDs 0x126f28` | `sp-80` | `0x1f3448` | `?` | `?` | `?` | `?` |
| `call` at 0x10ec9e | `_ffscUniqueIDToString 0x127758` | `sp-80` | `sp-104` | `?` | `?` | `?` | `?` |
| `call` at 0x10ecac | `_ffscMsg 0x120504` | `0x10ebf9` | `sp-104` | `?` | `?` | `?` | `?` |
| `call` at 0x10ecc1 | `_identUpdate 0x124a28` | `sp-80` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ecdc | `_addrdStartElectionTimer 0x10f0f0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ed06 | `_ffscMsg 0x120504` | `0x10ec42` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ed12 | `_identPrint 0x124b58` | `sp-80` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ed1c | `_taskDelay 0x174aa0` | `0x3` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ed3b | `_addrdInsertIdentity 0x10dfa0` | `sp-52` | `0x1f3448` | `?` | `?` | `?` | `?` |
| `call` at 0x10ed44 | `_addrdBroadcastMsg 0x10dec0` | `sp-52` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ed50 | `_addrdStartElectionTimer 0x10f0f0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10ec97 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `jl` at 0x10eccd | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10ecda | `[0x1df554] == 0x0` | `[0x1df554] != 0x0` |
| `jne` at 0x10ece3 | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) != 0x0` | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) == 0x0` |
| `je` at 0x10ecf0 | `(([0x1df500]:1 & 0xff) & 0x1) == 0x0` | `(([0x1df500]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x10ecf9 | `(ret(_identUpdate) & ret(_identUpdate)) != 0x0` | `(ret(_identUpdate) & ret(_identUpdate)) == 0x0` |
| `jne` at 0x10ed4e | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` |
| `jne` at 0x10ed57 | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) != 0x0` | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ed7d | `eax` | `?` |

## `_addrdProcessAssignMsg`

- `_addrdProcessAssignMsg`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e780 to 0x10e890.
  Established. The `_ffscCompareUniqueIDs` call has leading candidates `[sp+4]+32` and `0x1f3448`
  (`call` at 0x10e7a1). The branch takes `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs))
  == 0x0` and falls through for nonzero (`je` at 0x10e7ab). The taken route calls
  `_addrdUpdateLocalIdentity` with leading candidate `sp-32` (`call` at 0x10e7c1). EAX reaches the
  return as `0x0` (`ret` at 0x10e800). The assignment-message interpretation remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e788 | load | 4 | `sp +0x4` | `-` |
| `test` at 0x10e7ad | load | 1 | `abs 0x1df500` | `-` |
| `test` at 0x10e7da | load | 1 | `abs 0x1df500` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e790 | `_addrdExtractIdentity 0x10df40` | `[sp+4]` | `sp-32` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10e7a1 | `_ffscCompareUniqueIDs 0x126f28` | `[sp+4]+32` | `0x1f3448` | `?` | `?` | `?` | `?` |
| `call` at 0x10e7c1 | `_addrdUpdateLocalIdentity 0x10ddc0` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e7d2 | `_ffscMsg 0x120504` | `0x10e73a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e7e8 | `_ffscMsg 0x120504` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e7f1 | `_identPrint 0x124b58` | `sp-32` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10e7ab | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` |
| `je` at 0x10e7b4 | `(([0x1df500]:1 & 0xff) & 0x1) == 0x0` | `(([0x1df500]:1 & 0xff) & 0x1) != 0x0` |
| `je` at 0x10e7cb | `(ret(_addrdUpdateLocalIdentity) & ret(_addrdUpdateLocalIdentity)) == 0x0` | `(ret(_addrdUpdateLocalIdentity) & ret(_addrdUpdateLocalIdentity)) != 0x0` |
| `je` at 0x10e7e1 | `(([0x1df500]:1 & 0xff) & 0x1) == 0x0` | `(([0x1df500]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e800 | `eax` | `0x0` |

## `_addrdResolveConflicts`

- `_addrdResolveConflicts`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ef20 to 0x10efa0.
  Established. A four-byte comparison reads absolute `0x1f398c` (`cmp` at 0x10ef23). The branch
  takes `[0x1f398c] != 0x0` and falls through for zero (`jne` at 0x10ef2a); its taken route calls
  `_identScanForConflicts` with leading candidates `0x10efa0` and `0x10f040` (`call` at 0x10ef4a).
  The result branch takes `(ret(_identScanForConflicts) & ret(_identScanForConflicts)) != 0x0` and
  falls through for zero (`jne` at 0x10ef54). Those routes reach EAX `0xffffffff` (`ret` at
  0x10ef72) and `0x0` (`ret` at 0x10ef5b), respectively. The conflict-selection policy is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10ef23 | load | 4 | `abs 0x1f398c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ef31 | `_ffscMsg 0x120504` | `0x10eebc` | `ebp` | `?` | `?` | `?` | `?` |
| `call` at 0x10ef4a | `_identScanForConflicts 0x124798` | `0x10efa0` | `0x10f040` | `ebp` | `?` | `?` | `?` |
| `call` at 0x10ef65 | `_ffscMsg 0x120504` | `0x10eeec` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10ef2a | `[0x1f398c] != 0x0` | `[0x1f398c] == 0x0` |
| `jne` at 0x10ef54 | `(ret(_identScanForConflicts) & ret(_identScanForConflicts)) != 0x0` | `(ret(_identScanForConflicts) & ret(_identScanForConflicts)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ef3e | `eax` | `0xffffffff` |
| `ret` at 0x10ef5b | `eax` | `0x0` |
| `ret` at 0x10ef72 | `eax` | `0xffffffff` |

## `_addrdResolveIPAddrConflict`

- `_addrdResolveIPAddrConflict`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10efa0 to 0x10f040.
  Established. The merged `_memcpy` candidates begin `sp-32`, unknown, and `0x1c` (`call` at
  0x10efd7). `_addrdFindUnusedIPAddr` then presents leading candidate `sp-24` (`call` at 0x10efe6);
  its branch takes `(ret(_addrdFindUnusedIPAddr) & ret(_addrdFindUnusedIPAddr)) != 0x0` and falls
  through for zero (`jne` at 0x10eff0). The fall-through calls `_addrdAssignNewIdentity` with
  leading candidate `sp-32` (`call` at 0x10eff3). The selected source identity remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10efa8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10efab | load | 4 | `sp +0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10efb0 | `_ffscCompareUniqueIDs 0x126f28` | `[sp+4]` | `[sp+8]` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10efd7 | `_memcpy 0x182918` | `sp-32` | `?` | `0x1c` | `?` | `?` | `?` |
| `call` at 0x10efe6 | `_addrdFindUnusedIPAddr 0x10e690` | `sp-24` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10eff3 | `_addrdAssignNewIdentity 0x10e490` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10f005 | `_ffscMsg 0x120504` | `0x10ef73` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x10efba | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x10eff0 | `(ret(_addrdFindUnusedIPAddr) & ret(_addrdFindUnusedIPAddr)) != 0x0` | `(ret(_addrdFindUnusedIPAddr) & ret(_addrdFindUnusedIPAddr)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10f017 | `eax` | `?` |

## `_addrdFindUnusedIPAddr`

- `_addrdFindUnusedIPAddr`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e690 to 0x10e6f0.
  Established. A four-byte load reads absolute `0x1dfa4c` (`mov` at 0x10e6a2). The
  `_inet_makeaddr_b` candidates begin `[0x1dfa4c]`, unknown, and `[sp+4]` (`call` at 0x10e6a8). The
  branch takes `(ret(_identFindByIPAddr) & ret(_identFindByIPAddr)) != 0x0` and falls through for
  zero (`jne` at 0x10e6bb). The later loop branch takes `? <= 0xfffffe (signed, 32-bit)` and falls
  through for greater values (`jle` at 0x10e6d7). The concrete candidate-address sequence cannot be
  recovered from these merged facts.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e695 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10e6a2 | load | 4 | `abs 0x1dfa4c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e6a8 | `_inet_makeaddr_b 0x18b198` | `[0x1dfa4c]` | `?` | `[sp+4]` | `?` | `?` | `?` |
| `call` at 0x10e6b1 | `_identFindByIPAddr 0x1242a8` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e6bb | `(ret(_identFindByIPAddr) & ret(_identFindByIPAddr)) != 0x0` | `(ret(_identFindByIPAddr) & ret(_identFindByIPAddr)) == 0x0` |
| `jle` at 0x10e6d7 | `? <= 0xfffffe (signed, 32-bit)` | `? > 0xfffffe (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e6e6 | `eax` | `?` |

## `_addrdUpdateLocalIdentity`

- `_addrdUpdateLocalIdentity`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ddc0 to 0x10dec0.
  Established. The first comparison branch takes `[0x1f3450] == [[sp+4]+8]` and falls through for
  inequality (`je` at 0x10ddd6). That fall-through calls `_identSetIPAddr` with leading candidate
  `[sp+4]+8` (`call` at 0x10dddc). A later route sets four-byte `0x3` at `sp -0x34` (`mov` at
  0x10de69) and `0x1` at `sp -0x30` (`mov` at 0x10de70). Calling `_addrdInsertIdentity` on that
  route presents leading candidates `sp-52` and `[sp+4]` (`call` at 0x10de7c). The local-record
  change policy remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10ddc8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10ddcd | load | 4 | `[sp+4] +0x8` | `-` |
| `cmp` at 0x10ddd0 | load | 4 | `abs 0x1f3450` | `-` |
| `mov` at 0x10dde9 | load | 4 | `[sp+4] +0xc` | `-` |
| `cmp` at 0x10ddec | load | 4 | `abs 0x1f3454` | `-` |
| `mov` at 0x10ddf4 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10de05 | load | 4 | `[sp+4] +0x10` | `-` |
| `cmp` at 0x10de08 | load | 4 | `abs 0x1f3458` | `-` |
| `mov` at 0x10de10 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10de24 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10de28 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10de69 | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10de70 | store | 4 | `sp -0x30` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10dddc | `_identSetIPAddr 0x124d48` | `[sp+4]+8` | `ebx` | `esi` | `?` | `?` | `?` |
| `call` at 0x10ddf8 | `_identSetRackID 0x124f18` | `[[sp+4]+12]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10de14 | `_identSetCapabilities 0x124c68` | `[[sp+4]+16]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10de54 | `_identSetModuleNums 0x124e38` | `[sp+4]+20` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10de5d | `_identUpdate 0x124a28` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10de7c | `_addrdInsertIdentity 0x10dfa0` | `sp-52` | `[sp+4]` | `?` | `?` | `0x3` | `0x1` |
| `call` at 0x10de85 | `_addrdBroadcastMsg 0x10dec0` | `sp-52` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10ddd6 | `[0x1f3450] == [[sp+4]+8]` | `[0x1f3450] != [[sp+4]+8]` |
| `je` at 0x10ddf2 | `[0x1f3454] == [[sp+4]+12]` | `[0x1f3454] != [[sp+4]+12]` |
| `je` at 0x10de0e | `[0x1f3458] == [[sp+4]+16]` | `[0x1f3458] != [[sp+4]+16]` |
| `jne` at 0x10de31 | `Z == 0` | `not (Z == 0)` |
| `jbe` at 0x10de37 | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |
| `jne` at 0x10de3b | `Z == 0` | `not (Z == 0)` |
| `jl` at 0x10de67 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10de8c | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10de9b | `eax` | `?` |
