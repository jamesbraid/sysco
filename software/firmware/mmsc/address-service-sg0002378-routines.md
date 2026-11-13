# MMSC address service in patch SG0002378

Read from `patches/SG0002378/usr/cpu/firmware/mmscfw.bin` (patch SG0002378).
Image SHA-256: `b1249e58cf4f00c252362da8ed0515e25bb94aaf096bb60a5c5082c9b6e4d342`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred end and can include literals or padding
after returns. Statements below concern sites reached by known control-flow edges.
One computed jump remains unresolved in `_addrd`; its selector-arm effects are excluded.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/address-service-sg0002378-facts.txt) include all tool sections.

Three span tails leave the following raw bytes after decoding stops:

| Span | Raw-byte start | Exclusive end | Bytes |
|---|---|---|---|
| `_addrdInsertIdentity` | `0x10de49` | `0x10de50` | `69 6c 65 64 00 90 90` |
| `_addrdProcessAssignMsg` | `0x10e67c` | `0x10e680` | `65 64 00 90` |
| `_addrdResolveConflicts` | `0x10ed8b` | `0x10ed90` | `00 90 90 90 90` |

## `_addrdInit`

- `_addrdInit`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10d9c0 to 0x10db50.
  Established. The `_pipeDevCreate` candidates begin `0x10d8f2`, `0x5`, and `0x3e` (`call` at
  0x10d9cc); its branch takes `(ret(_pipeDevCreate) & ret(_pipeDevCreate)) == 0x0` and falls through
  for nonzero (`je` at 0x10d9d6). Later paths save four-byte `ret(_open)` at absolute `0x1ac928`
  (`mov` at 0x10da11), `ret(_wdCreate)` at absolute `0x1ed4b8` (`mov` at 0x10da35), and
  `ret(_semMCreate)` at absolute `0x1f0eec` (`mov` at 0x10da77). The service-object and startup
  meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10d9dd | load | 4 | `ret(___errno) +0x0` | `-` |
| `mov` at 0x10da11 | store | 4 | `abs 0x1ac928` | `ret(_open)` |
| `mov` at 0x10da35 | store | 4 | `abs 0x1ed4b8` | `ret(_wdCreate)` |
| `mov` at 0x10da77 | store | 4 | `abs 0x1f0eec` | `ret(_semMCreate)` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10d9cc | `_pipeDevCreate 0x18dbbc` | `0x10d8f2` | `0x5` | `0x3e` | `ebp` | `?` | `?` |
| `call` at 0x10d9d8 | `___errno 0x1711d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d9eb | `_ffscMsgS 0x11f110` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10da09 | `_open 0x1890e0` | `0x10d8f2` | `0x1` | `0x0` | `?` | `?` | `?` |
| `call` at 0x10da30 | `_wdCreate 0x174ebc` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10da43 | `_ffscMsg 0x11f040` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10da65 | `_lstInit 0x190178` | `0x1dd1c0` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10da6f | `_semMCreate 0x177968` | `0x1` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dab7 | `_taskSpawn 0x171358` | `0x10d98f` | `0x19` | `0x0` | `0x4e20` | `0x10df80` | `0x0` |
| `call` at 0x10dac9 | `_etherInputHookAdd 0x190bb8` | `0x10e430` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dae5 | `_ffscMsg 0x11f040` | `0x10d996` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10d9d6 | `(ret(_pipeDevCreate) & ret(_pipeDevCreate)) == 0x0` | `(ret(_pipeDevCreate) & ret(_pipeDevCreate)) != 0x0` |
| `je` at 0x10d9e4 | `[ret(___errno)] == 0xd0006` | `[ret(___errno)] != 0xd0006` |
| `jge` at 0x10da18 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x10da3c | `(ret(_wdCreate) & ret(_wdCreate)) != 0x0` | `(ret(_wdCreate) & ret(_wdCreate)) == 0x0` |
| `jne` at 0x10da7e | `(ret(_semMCreate) & ret(_semMCreate)) != 0x0` | `(ret(_semMCreate) & ret(_semMCreate)) == 0x0` |
| `je` at 0x10dac2 | `ret(_taskSpawn) == 0xffffffff` | `ret(_taskSpawn) != 0xffffffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10d9f8 | `eax` | `0xffffffff` |
| `ret` at 0x10da50 | `eax` | `0xffffffff` |
| `ret` at 0x10dad3 | `eax` | `0x0` |
| `ret` at 0x10daf2 | `eax` | `0xffffffff` |

## `_addrd`

- `_addrd`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10df80 to 0x10e280.
  Established. Calling `_read` presents leading candidates `ret(_open)`, `sp-68`, and `0x3e` (`call`
  at 0x10e077). The later decision takes `ret(_read) == 0x3e` and falls through for inequality (`je`
  at 0x10e097). A later route reaches an unresolved computed jump (`jmp` at 0x10e111). The selector
  arms are excluded from credited effects, and the dispatch semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10dfd3 | store | 4 | `sp -0x8c` | `?` |
| `mov` at 0x10dfe9 | load | 4 | `sp -0x8c` | `-` |
| `mov` at 0x10dfef | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10dff5 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10dff9 | store | 4 | `sp -0x88` | `0x1` |
| `rep movsd` at 0x10e010 | store | 4 | `(unknown)` | `?` |
| `rep movsd` at 0x10e010 | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x10e032 | load | 4 | `ret(___errno) +0x0` | `-` |
| `dec` at 0x10e053 | load | 4 | `sp -0x88` | `-` |
| `dec` at 0x10e053 | store | 4 | `sp -0x88` | `?` |
| `mov` at 0x10e063 | load | 4 | `(unknown)` | `-` |
| `dec` at 0x10e0c6 | load | 4 | `sp -0x88` | `-` |
| `dec` at 0x10e0c6 | store | 4 | `sp -0x88` | `?` |
| `mov` at 0x10e0cf | load | 4 | `sp -0x36` | `-` |
| `mov` at 0x10e0dc | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x10e0ea | store | 4 | `sp -0x36` | `0x2` |
| `mov` at 0x10e0f0 | load | 4 | `sp -0x36` | `-` |
| `mov` at 0x10e0f7 | store | 4 | `sp -0x1a` | `0x0` |
| `mov` at 0x10e0fe | store | 4 | `sp -0x36` | `0x3` |
| `mov` at 0x10e104 | load | 4 | `sp -0x32` | `-` |
| `jmp` at 0x10e111 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10e1a0 | load | 4 | `sp -0x32` | `-` |
| `cmp` at 0x10e1b1 | load | 4 | `sp -0x88` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10df95 | `_open 0x1890e0` | `0x10d8f2` | `0x0` | `0x0` | `ebx` | `esi` | `edi` |
| `call` at 0x10dfa8 | `_ffscMsgS 0x11f110` | `0x10deb8` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dfc6 | `_bzero 0x172678` | `sp-100` | `0x20` | `?` | `?` | `?` | `?` |
| `call` at 0x10e021 | `_select 0x174720` | `0x100` | `sp-132` | `0x0` | `0x0` | `0x0` | `?` |
| `call` at 0x10e02d | `___errno 0x1711d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e042 | `_ffscMsgS 0x11f110` | `0x10ded4` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e04b | `_close 0x189280` | `ret(_open)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e077 | `_read 0x189300` | `ret(_open)` | `sp-68` | `0x3e` | `?` | `?` | `?` |
| `call` at 0x10e088 | `_ffscMsgS 0x11f110` | `0x10deeb` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e0a1 | `_ffscMsg 0x11f040` | `0x10df22` | `ret(_read)` | `0x3e` | `?` | `?` | `?` |
| `call` at 0x10e0b5 | `_ffscMsg 0x11f040` | `0x10df0a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e0be | `_close 0x189280` | `ret(_open)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e1a9 | `_ffscMsg 0x11f040` | `0x10df54` | `[sp-50]` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x10dfa1 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jge` at 0x10e02b | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x10e037 | `[ret(___errno)] == 0x4` | `[ret(___errno)] != 0x4` |
| `jae` at 0x10e06a | `C == 0` | `not (C == 0)` |
| `jge` at 0x10e081 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x10e092 | `(ret(_read) & ret(_read)) == 0x0` | `(ret(_read) & ret(_read)) != 0x0` |
| `je` at 0x10e097 | `ret(_read) == 0x3e` | `ret(_read) != 0x3e` |
| `ja` at 0x10e0d5 | `[sp-54] > 0x1 (unsigned, 32-bit)` | `[sp-54] <= 0x1 (unsigned, 32-bit)` |
| `jle` at 0x10e0e8 | `? <= 0x1 (signed, 32-bit)` | `? > 0x1 (signed, 32-bit)` |
| `ja` at 0x10e0f5 | `[sp-54] > 0x2 (unsigned, 32-bit)` | `[sp-54] <= 0x2 (unsigned, 32-bit)` |
| `ja` at 0x10e10b | `? > 0x5 (unsigned, 32-bit)` | `? <= 0x5 (unsigned, 32-bit)` |
| `jg` at 0x10e1b8 | `[sp-136] > 0x0 (signed, 32-bit)` | `[sp-136] <= 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e1cc | `eax` | `?` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x10e111).

## `_addrdEtherInputHook`

- `_addrdEtherInputHook`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e430 to 0x10e480.
  Established. The two-byte load reads `[sp+8] +0xc` (`mov` at 0x10e43a). The branch takes
  `([[sp+8]+12]:2 & 0xffff) != 0x801` and falls through for equality (`jne` at 0x10e442); its
  fall-through calls `_write` with leading candidates `[0x1ac928]`, `[sp+8]`, and `[sp+12]` (`call`
  at 0x10e44c). The following decision takes `ret(_write) == [sp+12]` and falls through for
  inequality (`je` at 0x10e456). The payload and header-field interpretations remain unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e434 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10e437 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10e43a | load | 2 | `[sp+8] +0xc` | `-` |
| `mov` at 0x10e446 | load | 4 | `abs 0x1ac928` | `-` |
| `mov` at 0x10e472 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e44c | `_write 0x189320` | `[0x1ac928]` | `[sp+8]` | `[sp+12]` | `ebx` | `ebp` | `?` |
| `call` at 0x10e45d | `_ffscMsgS 0x11f110` | `0x10e40e` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e442 | `([[sp+8]+12]:2 & 0xffff) != 0x801` | `([[sp+8]+12]:2 & 0xffff) == 0x801` |
| `je` at 0x10e456 | `ret(_write) == [sp+12]` | `ret(_write) != [sp+12]` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e478 | `eax` | `?` |

## `_addrdLocalMsg`

- `_addrdLocalMsg`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10de50 to 0x10df80.
  Established. A two-byte store puts `0x108` at `sp -0x38` (`mov` at 0x10de69). The `_bcopy`
  candidates begin `[sp+4]`, `sp-54`, and `0x30` (`call` at 0x10de76). The subsequent `_write` call
  has leading candidates `[0x1ac928]`, `sp-68`, and `0x3e` (`call` at 0x10de87); its branch takes
  `ret(_write) != 0x3e` and falls through for equality (`jne` at 0x10de92). The record protocol
  remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10de58 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10de69 | store | 2 | `sp -0x38` | `0x108` |
| `mov` at 0x10de81 | load | 4 | `abs 0x1ac928` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10de61 | `_bzero 0x172678` | `sp-68` | `0x3e` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10de76 | `_bcopy 0x176bac` | `[sp+4]` | `sp-54` | `0x30` | `?` | `?` | `?` |
| `call` at 0x10de87 | `_write 0x189320` | `[0x1ac928]` | `sp-68` | `0x3e` | `?` | `?` | `?` |
| `call` at 0x10dea5 | `_ffscMsgS 0x11f110` | `0x10de21` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10de92 | `ret(_write) != 0x3e` | `ret(_write) == 0x3e` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10deb7 | `eax` | `?` |

## `_addrdBroadcastMsg`

- `_addrdBroadcastMsg`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10dcf0 to 0x10dd70.
  Established. The `_bcopy` calls present leading triples `0x1f0fe0`, `sp-14`, `0x6` (`call` at
  0x10dd09) and `0x1d3498`, `sp-20`, `0x6` (`call` at 0x10dd19). A two-byte store writes `0x108` at
  `sp -0x8` (`mov` at 0x10dd21). `_etherOutput` then begins `[0x1ed4a4]`, `sp-20`, `[sp+4]`, and
  `0x30` (`call` at 0x10dd31). The address and payload semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10dcf8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10dd21 | store | 2 | `sp -0x8` | `0x108` |
| `mov` at 0x10dd2b | load | 4 | `abs 0x1ed4a4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10dd09 | `_bcopy 0x176bac` | `0x1f0fe0` | `sp-14` | `0x6` | `ebx` | `esi` | `?` |
| `call` at 0x10dd19 | `_bcopy 0x176bac` | `0x1d3498` | `sp-20` | `0x6` | `?` | `?` | `?` |
| `call` at 0x10dd31 | `_etherOutput 0x190a78` | `[0x1ed4a4]` | `sp-20` | `[sp+4]` | `0x30` | `?` | `?` |
| `call` at 0x10dd55 | `_ffscMsgS 0x11f110` | `0x10dccc` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10dd3b | `(ret(_etherOutput) & ret(_etherOutput)) != 0x0` | `(ret(_etherOutput) & ret(_etherOutput)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10dd67 | `eax` | `?` |

## `_addrdExtractIdentity`

- `_addrdExtractIdentity`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10dd70 to 0x10ddd0.
  Established. The indexed four-byte store has unknown target and value (`mov` at 0x10dd84). Two
  `_bcopy` sites have leading triples `[sp+4]+32`, `[sp+8]`, `0x8` (`call` at 0x10dd95) and
  `[sp+4]+40`, `[sp+8]+8`, `0x4` (`call` at 0x10dda7). Later four-byte stores save `[[sp+4]+44]` at
  `[sp+8] +0xc` (`mov` at 0x10ddaf) and `[[sp+4]+28]` at `[sp+8] +0x10` (`mov` at 0x10ddb5). The
  record-field meanings remain unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10dd75 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10dd78 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10dd80 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10dd84 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10ddac | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10ddaf | store | 4 | `[sp+8] +0xc` | `[[sp+4]+44]` |
| `mov` at 0x10ddb2 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x10ddb5 | store | 4 | `[sp+8] +0x10` | `[[sp+4]+28]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10dd95 | `_bcopy 0x176bac` | `[sp+4]+32` | `[sp+8]` | `0x8` | `?` | `?` | `?` |
| `call` at 0x10dda7 | `_bcopy 0x176bac` | `[sp+4]+40` | `[sp+8]+8` | `0x4` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x10dd8c | `? <= 0x1 (signed, 32-bit)` | `? > 0x1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ddc0 | `eax` | `[[sp+4]+28]` |

## `_addrdInsertIdentity`

- `_addrdInsertIdentity`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ddd0 to 0x10de50.
  Established. An indexed four-byte store writes an unknown value at an unknown target (`mov` at
  0x10dde4). The `_bcopy` candidates begin `[sp+8]`, `[sp+4]+32`, `0x8` (`call` at 0x10ddf5) and
  `[sp+8]+8`, `[sp+4]+40`, `0x4` (`call` at 0x10de07). The final four-byte stores put `[[sp+8]+12]`
  at `[sp+4] +0x2c` (`mov` at 0x10de0f) and `[[sp+8]+16]` at `[sp+4] +0x1c` (`mov` at 0x10de15). The
  insertion-record contract is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10ddd5 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10ddd8 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10dde0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10dde4 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10de0c | load | 4 | `[sp+8] +0xc` | `-` |
| `mov` at 0x10de0f | store | 4 | `[sp+4] +0x2c` | `[[sp+8]+12]` |
| `mov` at 0x10de12 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x10de15 | store | 4 | `[sp+4] +0x1c` | `[[sp+8]+16]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ddf5 | `_bcopy 0x176bac` | `[sp+8]` | `[sp+4]+32` | `0x8` | `?` | `?` | `?` |
| `call` at 0x10de07 | `_bcopy 0x176bac` | `[sp+8]+8` | `[sp+4]+40` | `0x4` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x10ddec | `? <= 0x1 (signed, 32-bit)` | `? > 0x1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10de20 | `eax` | `[[sp+8]+16]` |

## `_addrdAssignNewIdentity`

- `_addrdAssignNewIdentity`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e280 to 0x10e380.
  Established. The `_identFindByUniqueID` candidates begin `[sp+4]` (`call` at 0x10e28d); its
  decision takes `(ret(_identFindByUniqueID) & ret(_identFindByUniqueID)) != 0x0` and falls through
  for zero (`jne` at 0x10e299). Later routes include four-byte stores of `0x3` at `sp -0x4c` (`mov`
  at 0x10e2f0) and `sp -0x48` (`mov` at 0x10e2f7). A later `_memcpy` site begins
  `ret(_identFindByUniqueID)`, `[sp+4]`, and `0x1c` (`call` at 0x10e363). The identity-selection
  rule is unestablished.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e289 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10e2f0 | store | 4 | `sp -0x4c` | `0x3` |
| `mov` at 0x10e2f7 | store | 4 | `sp -0x48` | `0x3` |
| `test` at 0x10e340 | load | 1 | `abs 0x1dccc0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e28d | `_identFindByUniqueID 0x122a08` | `[sp+4]` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x10e2a0 | `_ffscUniqueIDToString 0x125da8` | `[sp+4]` | `sp-28` | `?` | `?` | `?` | `?` |
| `call` at 0x10e2b6 | `_ffscCompareUniqueIDs 0x125578` | `[sp+4]` | `0x1f0bf8` | `?` | `?` | `?` | `?` |
| `call` at 0x10e2c3 | `_addrdUpdateLocalIdentity 0x10dbf0` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e2d8 | `_ffscMsg 0x11f040` | `0x10e208` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e303 | `_addrdInsertIdentity 0x10ddd0` | `sp-76` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x10e30c | `_addrdBroadcastMsg 0x10dcf0` | `sp-76` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e31d | `_ffscUniqueIDToString 0x125da8` | `ret(_identFindByUniqueID)` | `sp-100` | `?` | `?` | `?` | `?` |
| `call` at 0x10e32b | `_ffscMsg 0x11f040` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e34e | `_ffscMsg 0x11f040` | `0x10e259` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e357 | `_identPrint 0x1231a8` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e363 | `_memcpy 0x180150` | `ret(_identFindByUniqueID)` | `[sp+4]` | `0x1c` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e299 | `(ret(_identFindByUniqueID) & ret(_identFindByUniqueID)) != 0x0` | `(ret(_identFindByUniqueID) & ret(_identFindByUniqueID)) == 0x0` |
| `jne` at 0x10e2c0 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `je` at 0x10e2cd | `(ret(_addrdUpdateLocalIdentity) & ret(_addrdUpdateLocalIdentity)) == 0x0` | `(ret(_addrdUpdateLocalIdentity) & ret(_addrdUpdateLocalIdentity)) != 0x0` |
| `je` at 0x10e316 | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` |
| `je` at 0x10e347 | `(([0x1dccc0]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e373 | `eax` | `?` |

## `_addrdProcessIdentityMsg`

- `_addrdProcessIdentityMsg`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e920 to 0x10ea50.
  Established. The `_addrdExtractIdentity` candidates begin `[sp+4]` and `sp-32` (`call` at
  0x10e930). A later branch takes `[0x1dcd0c] == 0x0` and falls through for nonzero (`je` at
  0x10e986); its fall-through calls `_addrdStartElectionTimer` with unknown candidates (`call` at
  0x10e988). Another route includes `_addrdResolveConflicts` with unknown candidates (`call` at
  0x10e9cb). The state and timer policy remain unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e928 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x10e97f | load | 4 | `abs 0x1dcd0c` | `-` |
| `test` at 0x10e991 | load | 1 | `abs 0x1dccc0` | `-` |
| `cmp` at 0x10e9c2 | load | 4 | `abs 0x1f113c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e930 | `_addrdExtractIdentity 0x10dd70` | `[sp+4]` | `sp-32` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10e93e | `_ffscCompareUniqueIDs 0x125578` | `sp-32` | `0x1f0bf8` | `?` | `?` | `?` | `?` |
| `call` at 0x10e94f | `_ffscUniqueIDToString 0x125da8` | `sp-32` | `sp-56` | `?` | `?` | `?` | `?` |
| `call` at 0x10e95d | `_ffscMsg 0x11f040` | `0x10e8ac` | `sp-56` | `?` | `?` | `?` | `?` |
| `call` at 0x10e971 | `_identUpdate 0x123078` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e988 | `_addrdStartElectionTimer 0x10eee0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e9ae | `_ffscMsg 0x11f040` | `0x10e903` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e9ba | `_identPrint 0x1231a8` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e9cb | `_addrdResolveConflicts 0x10ed10` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e9d9 | `_ffscMsg 0x11f040` | `0x10e655` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e948 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `jl` at 0x10e97d | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10e986 | `[0x1dcd0c] == 0x0` | `[0x1dcd0c] != 0x0` |
| `jne` at 0x10e98f | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) != 0x0` | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) == 0x0` |
| `je` at 0x10e998 | `(([0x1dccc0]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x10e9a1 | `(ret(_identUpdate) & ret(_identUpdate)) != 0x0` | `(ret(_identUpdate) & ret(_identUpdate)) == 0x0` |
| `je` at 0x10e9c9 | `[0x1f113c] == 0x0` | `[0x1f113c] != 0x0` |
| `je` at 0x10e9d2 | `(ret(_addrdResolveConflicts) & ret(_addrdResolveConflicts)) == 0x0` | `(ret(_addrdResolveConflicts) & ret(_addrdResolveConflicts)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e9e8 | `eax` | `?` |

## `_addrdProcessQueryMsg`

- `_addrdProcessQueryMsg`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ea50 to 0x10ebc0.
  Established. Calling `_clock_gettime` presents leading candidates `0x0` and `0x1ed4b0` (`call` at
  0x10ea62). A later route calls `_taskDelay` with leading candidate `0x3` (`call` at 0x10eb0c),
  then stores four-byte `0x3` at `sp -0x34` (`mov` at 0x10eb14) and `0x1` at `sp -0x30` (`mov` at
  0x10eb1b). The final branch takes `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer))
  != 0x0` and falls through for zero (`jne` at 0x10eb47); its fall-through stores four-byte `0x1` at
  absolute `0x1dcd0c` (`mov` at 0x10eb49). The delay units and response meaning are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10ea58 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x10eac3 | load | 4 | `abs 0x1dcd0c` | `-` |
| `test` at 0x10ead9 | load | 1 | `abs 0x1dccc0` | `-` |
| `mov` at 0x10eb14 | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10eb1b | store | 4 | `sp -0x30` | `0x1` |
| `mov` at 0x10eb49 | store | 4 | `abs 0x1dcd0c` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ea62 | `_clock_gettime 0x17bbcc` | `0x0` | `0x1ed4b0` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10ea6f | `_addrdExtractIdentity 0x10dd70` | `[sp+4]` | `sp-80` | `?` | `?` | `?` | `?` |
| `call` at 0x10ea7d | `_ffscCompareUniqueIDs 0x125578` | `sp-80` | `0x1f0bf8` | `?` | `?` | `?` | `?` |
| `call` at 0x10ea8e | `_ffscUniqueIDToString 0x125da8` | `sp-80` | `sp-104` | `?` | `?` | `?` | `?` |
| `call` at 0x10ea9c | `_ffscMsg 0x11f040` | `0x10e9e9` | `sp-104` | `?` | `?` | `?` | `?` |
| `call` at 0x10eab1 | `_identUpdate 0x123078` | `sp-80` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10eacc | `_addrdStartElectionTimer 0x10eee0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10eaf6 | `_ffscMsg 0x11f040` | `0x10ea32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10eb02 | `_identPrint 0x1231a8` | `sp-80` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10eb0c | `_taskDelay 0x1722d8` | `0x3` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10eb2b | `_addrdInsertIdentity 0x10ddd0` | `sp-52` | `0x1f0bf8` | `?` | `?` | `?` | `?` |
| `call` at 0x10eb34 | `_addrdBroadcastMsg 0x10dcf0` | `sp-52` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10eb40 | `_addrdStartElectionTimer 0x10eee0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10ea87 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `jl` at 0x10eabd | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10eaca | `[0x1dcd0c] == 0x0` | `[0x1dcd0c] != 0x0` |
| `jne` at 0x10ead3 | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) != 0x0` | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) == 0x0` |
| `je` at 0x10eae0 | `(([0x1dccc0]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x1) != 0x0` |
| `jne` at 0x10eae9 | `(ret(_identUpdate) & ret(_identUpdate)) != 0x0` | `(ret(_identUpdate) & ret(_identUpdate)) == 0x0` |
| `jne` at 0x10eb3e | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` |
| `jne` at 0x10eb47 | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) != 0x0` | `(ret(_addrdStartElectionTimer) & ret(_addrdStartElectionTimer)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10eb6d | `eax` | `?` |

## `_addrdProcessAssignMsg`

- `_addrdProcessAssignMsg`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e570 to 0x10e680.
  Established. The `_ffscCompareUniqueIDs` candidates begin `[sp+4]+32` and `0x1f0bf8` (`call` at
  0x10e591). The decision takes `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0`
  and falls through for nonzero (`je` at 0x10e59b); its taken route calls
  `_addrdUpdateLocalIdentity` with leading candidate `sp-32` (`call` at 0x10e5b1). EAX returns `0x0`
  (`ret` at 0x10e5f0). The assignment-message interpretation remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e578 | load | 4 | `sp +0x4` | `-` |
| `test` at 0x10e59d | load | 1 | `abs 0x1dccc0` | `-` |
| `test` at 0x10e5ca | load | 1 | `abs 0x1dccc0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e580 | `_addrdExtractIdentity 0x10dd70` | `[sp+4]` | `sp-32` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10e591 | `_ffscCompareUniqueIDs 0x125578` | `[sp+4]+32` | `0x1f0bf8` | `?` | `?` | `?` | `?` |
| `call` at 0x10e5b1 | `_addrdUpdateLocalIdentity 0x10dbf0` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e5c2 | `_ffscMsg 0x11f040` | `0x10e52a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e5d8 | `_ffscMsg 0x11f040` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e5e1 | `_identPrint 0x1231a8` | `sp-32` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10e59b | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` |
| `je` at 0x10e5a4 | `(([0x1dccc0]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x1) != 0x0` |
| `je` at 0x10e5bb | `(ret(_addrdUpdateLocalIdentity) & ret(_addrdUpdateLocalIdentity)) == 0x0` | `(ret(_addrdUpdateLocalIdentity) & ret(_addrdUpdateLocalIdentity)) != 0x0` |
| `je` at 0x10e5d1 | `(([0x1dccc0]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e5f0 | `eax` | `0x0` |

## `_addrdResolveConflicts`

- `_addrdResolveConflicts`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ed10 to 0x10ed90.
  Established. The first branch takes `[0x1f113c] != 0x0` and falls through for zero (`jne` at
  0x10ed1a). The taken route calls `_identScanForConflicts` with leading candidates `0x10ed90` and
  `0x10ee30` (`call` at 0x10ed3a); its result decision takes `(ret(_identScanForConflicts) &
  ret(_identScanForConflicts)) != 0x0` and falls through for zero (`jne` at 0x10ed44). Those routes
  reach EAX `0xffffffff` (`ret` at 0x10ed62) and `0x0` (`ret` at 0x10ed4b), respectively. The
  conflict policy remains unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10ed13 | load | 4 | `abs 0x1f113c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ed21 | `_ffscMsg 0x11f040` | `0x10ecac` | `ebp` | `?` | `?` | `?` | `?` |
| `call` at 0x10ed3a | `_identScanForConflicts 0x122de8` | `0x10ed90` | `0x10ee30` | `ebp` | `?` | `?` | `?` |
| `call` at 0x10ed55 | `_ffscMsg 0x11f040` | `0x10ecdc` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10ed1a | `[0x1f113c] != 0x0` | `[0x1f113c] == 0x0` |
| `jne` at 0x10ed44 | `(ret(_identScanForConflicts) & ret(_identScanForConflicts)) != 0x0` | `(ret(_identScanForConflicts) & ret(_identScanForConflicts)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ed2e | `eax` | `0xffffffff` |
| `ret` at 0x10ed4b | `eax` | `0x0` |
| `ret` at 0x10ed62 | `eax` | `0xffffffff` |

## `_addrdResolveIPAddrConflict`

- `_addrdResolveIPAddrConflict`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ed90 to 0x10ee30.
  Established. The merged `_memcpy` candidates begin `sp-32`, unknown, and `0x1c` (`call` at
  0x10edc7). `_addrdFindUnusedIPAddr` follows with leading candidate `sp-24` (`call` at 0x10edd6);
  its branch takes `(ret(_addrdFindUnusedIPAddr) & ret(_addrdFindUnusedIPAddr)) != 0x0` and falls
  through for zero (`jne` at 0x10ede0). The fall-through calls `_addrdAssignNewIdentity` with
  leading candidate `sp-32` (`call` at 0x10ede3). The source-identity selection is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10ed98 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10ed9b | load | 4 | `sp +0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10eda0 | `_ffscCompareUniqueIDs 0x125578` | `[sp+4]` | `[sp+8]` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10edc7 | `_memcpy 0x180150` | `sp-32` | `?` | `0x1c` | `?` | `?` | `?` |
| `call` at 0x10edd6 | `_addrdFindUnusedIPAddr 0x10e480` | `sp-24` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ede3 | `_addrdAssignNewIdentity 0x10e280` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10edf5 | `_ffscMsg 0x11f040` | `0x10ed63` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x10edaa | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x10ede0 | `(ret(_addrdFindUnusedIPAddr) & ret(_addrdFindUnusedIPAddr)) != 0x0` | `(ret(_addrdFindUnusedIPAddr) & ret(_addrdFindUnusedIPAddr)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ee07 | `eax` | `?` |

## `_addrdFindUnusedIPAddr`

- `_addrdFindUnusedIPAddr`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e480 to 0x10e4e0.
  Established. A four-byte load reads absolute `0x1dd204` (`mov` at 0x10e492). The
  `_inet_makeaddr_b` candidates begin `[0x1dd204]`, unknown, and `[sp+4]` (`call` at 0x10e498). The
  next decision takes `(ret(_identFindByIPAddr) & ret(_identFindByIPAddr)) != 0x0` and falls through
  for zero (`jne` at 0x10e4ab). The later loop decision takes `? <= 0xfffffe (signed, 32-bit)` and
  falls through for greater values (`jle` at 0x10e4c7). The concrete address sequence remains
  unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e485 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10e492 | load | 4 | `abs 0x1dd204` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e498 | `_inet_makeaddr_b 0x1889d0` | `[0x1dd204]` | `?` | `[sp+4]` | `?` | `?` | `?` |
| `call` at 0x10e4a1 | `_identFindByIPAddr 0x1228f8` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e4ab | `(ret(_identFindByIPAddr) & ret(_identFindByIPAddr)) != 0x0` | `(ret(_identFindByIPAddr) & ret(_identFindByIPAddr)) == 0x0` |
| `jle` at 0x10e4c7 | `? <= 0xfffffe (signed, 32-bit)` | `? > 0xfffffe (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e4d6 | `eax` | `?` |

## `_addrdUpdateLocalIdentity`

- `_addrdUpdateLocalIdentity`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10dbf0 to 0x10dcf0.
  Established. The entry comparison takes `[0x1f0c00] == [[sp+4]+8]` and falls through for
  inequality (`je` at 0x10dc06). The fall-through calls `_identSetIPAddr` with leading candidate
  `[sp+4]+8` (`call` at 0x10dc0c). On a later route, four-byte stores put `0x3` at `sp -0x34` (`mov`
  at 0x10dc99) and `0x1` at `sp -0x30` (`mov` at 0x10dca0). `_addrdInsertIdentity` then presents
  leading candidates `sp-52` and `[sp+4]` on that route (`call` at 0x10dcac). The update-record
  semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10dbf8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10dbfd | load | 4 | `[sp+4] +0x8` | `-` |
| `cmp` at 0x10dc00 | load | 4 | `abs 0x1f0c00` | `-` |
| `mov` at 0x10dc19 | load | 4 | `[sp+4] +0xc` | `-` |
| `cmp` at 0x10dc1c | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x10dc24 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10dc35 | load | 4 | `[sp+4] +0x10` | `-` |
| `cmp` at 0x10dc38 | load | 4 | `abs 0x1f0c08` | `-` |
| `mov` at 0x10dc40 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10dc54 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10dc58 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10dc99 | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10dca0 | store | 4 | `sp -0x30` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10dc0c | `_identSetIPAddr 0x123398` | `[sp+4]+8` | `ebx` | `esi` | `?` | `?` | `?` |
| `call` at 0x10dc28 | `_identSetRackID 0x123568` | `[[sp+4]+12]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dc44 | `_identSetCapabilities 0x1232b8` | `[[sp+4]+16]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dc84 | `_identSetModuleNums 0x123488` | `[sp+4]+20` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dc8d | `_identUpdate 0x123078` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dcac | `_addrdInsertIdentity 0x10ddd0` | `sp-52` | `[sp+4]` | `?` | `?` | `0x3` | `0x1` |
| `call` at 0x10dcb5 | `_addrdBroadcastMsg 0x10dcf0` | `sp-52` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10dc06 | `[0x1f0c00] == [[sp+4]+8]` | `[0x1f0c00] != [[sp+4]+8]` |
| `je` at 0x10dc22 | `[0x1f0c04] == [[sp+4]+12]` | `[0x1f0c04] != [[sp+4]+12]` |
| `je` at 0x10dc3e | `[0x1f0c08] == [[sp+4]+16]` | `[0x1f0c08] != [[sp+4]+16]` |
| `jne` at 0x10dc61 | `Z == 0` | `not (Z == 0)` |
| `jbe` at 0x10dc67 | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |
| `jne` at 0x10dc6b | `Z == 0` | `not (Z == 0)` |
| `jl` at 0x10dc97 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10dcbc | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10dccb | `eax` | `?` |
