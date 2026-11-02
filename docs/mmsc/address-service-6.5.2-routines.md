# MMSC address-service message paths in IRIX 6.5.2

Read from `6.5.2/usr/cpu/firmware/mmscfw.bin` (IRIX 6.5.2 carrier).
Image SHA-256: `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred end and can include literals
after returns. Statements below concern sites reached by known control-flow edges.
Unresolved computed jumps make the effects incomplete for the affected routines.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/address-service-6.5.2-facts.txt) include all tool sections.

## `_addrdInit`

- `_addrdInit`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10d9c0 to 0x10db50.
  Established. The `_pipeDevCreate` candidates begin 0x10d8f2, 0xb and 0x3e (`call` at 0x10d9cc).
  Separate four-byte stores record `ret(_open)` at 0x1aca28, `ret(_wdCreate)` at 0x1ed5c0 and
  `ret(_semMCreate)` at 0x1f0ff4 (`mov` at 0x10da11, `mov` at 0x10da35, `mov` at 0x10da77).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10d9dd | load | 4 | `ret(___errno) +0x0` | `-` |
| `mov` at 0x10da11 | store | 4 | `abs 0x1aca28` | `ret(_open)` |
| `mov` at 0x10da35 | store | 4 | `abs 0x1ed5c0` | `ret(_wdCreate)` |
| `mov` at 0x10da77 | store | 4 | `abs 0x1f0ff4` | `ret(_semMCreate)` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10d9cc | `_pipeDevCreate 0x18dcbc` | `0x10d8f2` | `0xb` | `0x3e` | `ebp` | `?` | `?` |
| `call` at 0x10d9d8 | `___errno 0x1712d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d9eb | `_ffscMsgS 0x11f200` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10da09 | `_open 0x1891e0` | `0x10d8f2` | `0x1` | `0x0` | `?` | `?` | `?` |
| `call` at 0x10da30 | `_wdCreate 0x174fbc` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10da43 | `_ffscMsg 0x11f0d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10da65 | `_lstInit 0x190278` | `0x1dd2c8` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10da6f | `_semMCreate 0x177a68` | `0x1` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dab7 | `_taskSpawn 0x171458` | `0x10d98f` | `0x19` | `0x0` | `0x4e20` | `0x10df80` | `0x0` |
| `call` at 0x10dac9 | `_etherInputHookAdd 0x190cb8` | `0x10e430` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dae5 | `_ffscMsg 0x11f0d0` | `0x10d996` | `?` | `?` | `?` | `?` | `?` |

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

- `_addrd`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10df80 to 0x10e280.
  Established. The `_read` call has candidates beginning `ret(_open)`, `sp-68` and 0x3e (`call` at
  0x10e077). The later `ja` is taken for `? > 0x5 (unsigned, 32-bit)` (`ja` at 0x10e10b); its
  fall-through reaches the unresolved computed jump whose dispatch edges are excluded from the known
  effects (`ja` at 0x10e10b, `jmp` at 0x10e111).

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
| `call` at 0x10df95 | `_open 0x1891e0` | `0x10d8f2` | `0x0` | `0x0` | `ebx` | `esi` | `edi` |
| `call` at 0x10dfa8 | `_ffscMsgS 0x11f200` | `0x10deb8` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dfc6 | `_bzero 0x172778` | `sp-100` | `0x20` | `?` | `?` | `?` | `?` |
| `call` at 0x10e021 | `_select 0x174820` | `0x100` | `sp-132` | `0x0` | `0x0` | `0x0` | `?` |
| `call` at 0x10e02d | `___errno 0x1712d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e042 | `_ffscMsgS 0x11f200` | `0x10ded4` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e04b | `_close 0x189380` | `ret(_open)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e077 | `_read 0x189400` | `ret(_open)` | `sp-68` | `0x3e` | `?` | `?` | `?` |
| `call` at 0x10e088 | `_ffscMsgS 0x11f200` | `0x10deeb` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e0a1 | `_ffscMsg 0x11f0d0` | `0x10df22` | `ret(_read)` | `0x3e` | `?` | `?` | `?` |
| `call` at 0x10e0b5 | `_ffscMsg 0x11f0d0` | `0x10df0a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e0be | `_close 0x189380` | `ret(_open)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e1a9 | `_ffscMsg 0x11f0d0` | `0x10df54` | `[sp-50]` | `?` | `?` | `?` | `?` |

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

- `_addrdEtherInputHook`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e430 to 0x10e480.
  Established. The two-byte read renders as `[sp+8] +0xc` (`mov` at 0x10e43a). `jne` bypasses
  `_write` for `([[sp+8]+12]:2 & 0xffff) != 0x801`, while equality falls through to the call (`jne`
  at 0x10e442, `call` at 0x10e44c). That call has first candidates `[0x1aca28]`, `[sp+8]` and
  `[sp+12]` (`call` at 0x10e44c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e434 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10e437 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10e43a | load | 2 | `[sp+8] +0xc` | `-` |
| `mov` at 0x10e446 | load | 4 | `abs 0x1aca28` | `-` |
| `mov` at 0x10e472 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e44c | `_write 0x189420` | `[0x1aca28]` | `[sp+8]` | `[sp+12]` | `ebx` | `ebp` | `?` |
| `call` at 0x10e45d | `_ffscMsgS 0x11f200` | `0x10e40e` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e442 | `([[sp+8]+12]:2 & 0xffff) != 0x801` | `([[sp+8]+12]:2 & 0xffff) == 0x801` |
| `je` at 0x10e456 | `ret(_write) == [sp+12]` | `ret(_write) != [sp+12]` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e478 | `eax` | `?` |

## `_addrdBroadcastMsg`

- `_addrdBroadcastMsg`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10dcf0 to 0x10dd70.
  Established. The two-byte local store carries 0x108 at `sp -0x8` (`mov` at 0x10dd21).
  `_etherOutput` has first candidates `[0x1ed5ac]`, `sp-20`, `[sp+4]` and 0x30 (`call` at 0x10dd31);
  its nonzero result takes `jne` to `_ffscMsgS`, and zero falls through (`jne` at 0x10dd3b,
  `call` at 0x10dd55).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10dcf8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10dd21 | store | 2 | `sp -0x8` | `0x108` |
| `mov` at 0x10dd2b | load | 4 | `abs 0x1ed5ac` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10dd09 | `_bcopy 0x176cac` | `0x1f10e8` | `sp-14` | `0x6` | `ebx` | `esi` | `?` |
| `call` at 0x10dd19 | `_bcopy 0x176cac` | `0x1d3598` | `sp-20` | `0x6` | `?` | `?` | `?` |
| `call` at 0x10dd31 | `_etherOutput 0x190b78` | `[0x1ed5ac]` | `sp-20` | `[sp+4]` | `0x30` | `?` | `?` |
| `call` at 0x10dd55 | `_ffscMsgS 0x11f200` | `0x10dccc` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10dd3b | `(ret(_etherOutput) & ret(_etherOutput)) != 0x0` | `(ret(_etherOutput) & ret(_etherOutput)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10dd67 | `eax` | `?` |

## `_addrdInsertIdentity`

- `_addrdInsertIdentity`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ddd0 to 0x10de50.
  Established. Two four-byte stores copy `[[sp+8]+12]` through `[sp+4] +0x2c` and `[[sp+8]+16]`
  through `[sp+4] +0x1c` (`mov` at 0x10de0f, `mov` at 0x10de15). The prior indexed four-byte
  store has unknown target and value (`mov` at 0x10dde4).

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
| `call` at 0x10ddf5 | `_bcopy 0x176cac` | `[sp+8]` | `[sp+4]+32` | `0x8` | `?` | `?` | `?` |
| `call` at 0x10de07 | `_bcopy 0x176cac` | `[sp+8]+8` | `[sp+4]+40` | `0x4` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x10ddec | `? <= 0x1 (signed, 32-bit)` | `? > 0x1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10de20 | `eax` | `[[sp+8]+16]` |

## `_addrdLocalMsg`

- `_addrdLocalMsg`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10de50 to 0x10df80.
  Established. Calls `_bzero` with first candidates `sp-68` and 0x3e, then `_bcopy` with `[sp+4]`,
  `sp-54` and 0x30 (`call` at 0x10de61, `call` at 0x10de76). The later `_write` candidates begin
  `[0x1aca28]`, `sp-68` and 0x3e (`call` at 0x10de87).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10de58 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10de69 | store | 2 | `sp -0x38` | `0x108` |
| `mov` at 0x10de81 | load | 4 | `abs 0x1aca28` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10de61 | `_bzero 0x172778` | `sp-68` | `0x3e` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10de76 | `_bcopy 0x176cac` | `[sp+4]` | `sp-54` | `0x30` | `?` | `?` | `?` |
| `call` at 0x10de87 | `_write 0x189420` | `[0x1aca28]` | `sp-68` | `0x3e` | `?` | `?` | `?` |
| `call` at 0x10dea5 | `_ffscMsgS 0x11f200` | `0x10de21` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10de92 | `ret(_write) != 0x3e` | `ret(_write) == 0x3e` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10deb7 | `eax` | `?` |

## `_addrdUpdateLocalIdentity`

- `_addrdUpdateLocalIdentity`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10dbf0 to 0x10dcf0.
  Established. The three early `je` sites bypass calls for `[0x1f0d08] == [[sp+4]+8]`, `[0x1f0d0c]
  == [[sp+4]+12]` and `[0x1f0d10] == [[sp+4]+16]` respectively (`je` at 0x10dc06, `je` at 0x10dc22,
  `je` at 0x10dc3e). Explicit later four-byte stores place values 3 and 1 at `sp -0x34` and `sp
  -0x30` (`mov` at 0x10dc99, `mov` at 0x10dca0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10dbf8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10dbfd | load | 4 | `[sp+4] +0x8` | `-` |
| `cmp` at 0x10dc00 | load | 4 | `abs 0x1f0d08` | `-` |
| `mov` at 0x10dc19 | load | 4 | `[sp+4] +0xc` | `-` |
| `cmp` at 0x10dc1c | load | 4 | `abs 0x1f0d0c` | `-` |
| `mov` at 0x10dc24 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10dc35 | load | 4 | `[sp+4] +0x10` | `-` |
| `cmp` at 0x10dc38 | load | 4 | `abs 0x1f0d10` | `-` |
| `mov` at 0x10dc40 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10dc54 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10dc58 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10dc99 | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10dca0 | store | 4 | `sp -0x30` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10dc0c | `_identSetIPAddr 0x1234b8` | `[sp+4]+8` | `ebx` | `esi` | `?` | `?` | `?` |
| `call` at 0x10dc28 | `_identSetRackID 0x123688` | `[[sp+4]+12]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dc44 | `_identSetCapabilities 0x1233d8` | `[[sp+4]+16]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dc84 | `_identSetModuleNums 0x1235a8` | `[sp+4]+20` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dc8d | `_identUpdate 0x123198` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dcac | `_addrdInsertIdentity 0x10ddd0` | `sp-52` | `[sp+4]` | `?` | `?` | `0x3` | `0x1` |
| `call` at 0x10dcb5 | `_addrdBroadcastMsg 0x10dcf0` | `sp-52` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10dc06 | `[0x1f0d08] == [[sp+4]+8]` | `[0x1f0d08] != [[sp+4]+8]` |
| `je` at 0x10dc22 | `[0x1f0d0c] == [[sp+4]+12]` | `[0x1f0d0c] != [[sp+4]+12]` |
| `je` at 0x10dc3e | `[0x1f0d10] == [[sp+4]+16]` | `[0x1f0d10] != [[sp+4]+16]` |
| `jne` at 0x10dc61 | `Z == 0` | `not (Z == 0)` |
| `jbe` at 0x10dc67 | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |
| `jne` at 0x10dc6b | `Z == 0` | `not (Z == 0)` |
| `jl` at 0x10dc97 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10dcbc | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10dccb | `eax` | `?` |

## `_addrdExtractIdentity`

- `_addrdExtractIdentity`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10dd70 to 0x10ddd0.
  Established. The `_bcopy` candidates begin `[sp+4]+32`, `[sp+8]`, 8 at one site and `[sp+4]+40`,
  `[sp+8]+8`, 4 at the other (`call` at 0x10dd95, `call` at 0x10dda7). Later four-byte stores copy
  `[[sp+4]+44]` through `[sp+8] +0xc` and `[[sp+4]+28]` through `[sp+8] +0x10` (`mov` at 0x10ddaf,
  `mov` at 0x10ddb5).

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
| `call` at 0x10dd95 | `_bcopy 0x176cac` | `[sp+4]+32` | `[sp+8]` | `0x8` | `?` | `?` | `?` |
| `call` at 0x10dda7 | `_bcopy 0x176cac` | `[sp+4]+40` | `[sp+8]+8` | `0x4` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x10dd8c | `? <= 0x1 (signed, 32-bit)` | `? > 0x1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ddc0 | `eax` | `[[sp+4]+28]` |

## `_addrdProcessAssignMsg`

- `_addrdProcessAssignMsg`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e570 to 0x10e680.
  Established. The `_ffscCompareUniqueIDs` candidates begin `[sp+4]+32` and 0x1f0d00 (`call` at
  0x10e591); its zero result takes `je` to `_addrdUpdateLocalIdentity`, whose first candidate is
  `sp-32`, and nonzero falls through to the byte-test region (`je` at 0x10e59b, `call` at
  0x10e5b1).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e578 | load | 4 | `sp +0x4` | `-` |
| `test` at 0x10e59d | load | 1 | `abs 0x1dcdc8` | `-` |
| `test` at 0x10e5ca | load | 1 | `abs 0x1dcdc8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e580 | `_addrdExtractIdentity 0x10dd70` | `[sp+4]` | `sp-32` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10e591 | `_ffscCompareUniqueIDs 0x125698` | `[sp+4]+32` | `0x1f0d00` | `?` | `?` | `?` | `?` |
| `call` at 0x10e5b1 | `_addrdUpdateLocalIdentity 0x10dbf0` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e5c2 | `_ffscMsg 0x11f0d0` | `0x10e52a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e5d8 | `_ffscMsg 0x11f0d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e5e1 | `_identPrint 0x1232c8` | `sp-32` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10e59b | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` |
| `je` at 0x10e5a4 | `(([0x1dcdc8]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dcdc8]:1 & 0xff) & 0x1) != 0x0` |
| `je` at 0x10e5bb | `(ret(_addrdUpdateLocalIdentity) & ret(_addrdUpdateLocalIdentity)) == 0x0` | `(ret(_addrdUpdateLocalIdentity) & ret(_addrdUpdateLocalIdentity)) != 0x0` |
| `je` at 0x10e5d1 | `(([0x1dcdc8]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dcdc8]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e5f0 | `eax` | `0x0` |

## `_addrdResolveConflicts`

- `_addrdResolveConflicts`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ed10 to 0x10ed90.
  Established. The `_identScanForConflicts` call receives candidates beginning 0x10ed90 and 0x10ee30
  (`call` at 0x10ed3a). The earlier `jne` selects that route for `[0x1f1244] != 0x0`, with zero
  falling through to `_ffscMsg` and the first all-ones return (`jne` at 0x10ed1a, `call` at
  0x10ed21, `ret` at 0x10ed2e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10ed13 | load | 4 | `abs 0x1f1244` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ed21 | `_ffscMsg 0x11f0d0` | `0x10ecac` | `ebp` | `?` | `?` | `?` | `?` |
| `call` at 0x10ed3a | `_identScanForConflicts 0x122f08` | `0x10ed90` | `0x10ee30` | `ebp` | `?` | `?` | `?` |
| `call` at 0x10ed55 | `_ffscMsg 0x11f0d0` | `0x10ecdc` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10ed1a | `[0x1f1244] != 0x0` | `[0x1f1244] == 0x0` |
| `jne` at 0x10ed44 | `(ret(_identScanForConflicts) & ret(_identScanForConflicts)) != 0x0` | `(ret(_identScanForConflicts) & ret(_identScanForConflicts)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ed2e | `eax` | `0xffffffff` |
| `ret` at 0x10ed4b | `eax` | `0x0` |
| `ret` at 0x10ed62 | `eax` | `0xffffffff` |

## `_addrdResolveIPAddrConflict`

- `_addrdResolveIPAddrConflict`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ed90 to 0x10ee30.
  Established. The `_memcpy` first candidates are `sp-32`, unknown and 0x1c (`call` at 0x10edc7).
  `_addrdFindUnusedIPAddr` then receives `sp-24` as its first candidate (`call` at 0x10edd6); its
  nonzero result takes `jne` to `_ffscMsg`, while zero falls through to
  `_addrdAssignNewIdentity` (`jne` at 0x10ede0, `call` at 0x10edf5, `call` at 0x10ede3).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10ed98 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10ed9b | load | 4 | `sp +0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10eda0 | `_ffscCompareUniqueIDs 0x125698` | `[sp+4]` | `[sp+8]` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10edc7 | `_memcpy 0x180250` | `sp-32` | `?` | `0x1c` | `?` | `?` | `?` |
| `call` at 0x10edd6 | `_addrdFindUnusedIPAddr 0x10e480` | `sp-24` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ede3 | `_addrdAssignNewIdentity 0x10e280` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10edf5 | `_ffscMsg 0x11f0d0` | `0x10ed63` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x10edaa | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x10ede0 | `(ret(_addrdFindUnusedIPAddr) & ret(_addrdFindUnusedIPAddr)) != 0x0` | `(ret(_addrdFindUnusedIPAddr) & ret(_addrdFindUnusedIPAddr)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ee07 | `eax` | `?` |

## `_identSetIPAddr`

- `_identSetIPAddr`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1234b8 to 0x1235a8.
  Established. Four-byte stores place `[[sp+4]]` at 0x1f0d08, `ret(_inet_netof)` at 0x1dd30c and
  `[sp-28]` at 0x1dd308 (`mov` at 0x12352a, `mov` at 0x12352f, `mov` at 0x123538). The following
  `_nvramWrite` first candidates are 1 and 0x1dd308 (`call` at 0x123545).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1234c1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1234c4 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x1234cf | store | 4 | `sp -0x1c` | `ret(_inet_lnaof)` |
| `mov` at 0x1234d2 | load | 4 | `[sp+4] +0x0` | `-` |
| `cmp` at 0x1234df | load | 4 | `abs 0x1dd30c` | `-` |
| `mov` at 0x1234f8 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x123528 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x12352a | store | 4 | `abs 0x1f0d08` | `[[sp+4]]` |
| `mov` at 0x12352f | store | 4 | `abs 0x1dd30c` | `ret(_inet_netof)` |
| `mov` at 0x123535 | load | 4 | `sp -0x1c` | `-` |
| `mov` at 0x123538 | store | 4 | `abs 0x1dd308` | `[sp-28]` |
| `test` at 0x12355e | load | 1 | `abs 0x1dcdc9` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1234c7 | `_inet_lnaof 0x188a60` | `[[sp+4]]` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x1234d5 | `_inet_netof 0x188ba0` | `[[sp+4]]` | `?` | `?` | `?` | `ret(_inet_lnaof)` | `?` |
| `call` at 0x1234ec | `_ffscMsg 0x11f0d0` | `0x1233fd` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1234fb | `_inet_ntoa_b 0x188d90` | `[[sp+4]]` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x123509 | `_ifAddrSet 0x1899e0` | `0x123423` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x12351b | `_ffscMsgS 0x11f200` | `0x123428` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x123545 | `_nvramWrite 0x127288` | `0x1` | `0x1dd308` | `?` | `?` | `?` | `?` |
| `call` at 0x123556 | `_ffscMsg 0x11f0d0` | `0x123452` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12356d | `_ffscMsg 0x11f0d0` | `0x123490` | `sp-24` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1234e5 | `[0x1dd30c] == ret(_inet_netof)` | `[0x1dd30c] != ret(_inet_netof)` |
| `je` at 0x123513 | `(ret(_ifAddrSet) & ret(_ifAddrSet)) == 0x0` | `(ret(_ifAddrSet) & ret(_ifAddrSet)) != 0x0` |
| `je` at 0x12354f | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` |
| `je` at 0x123565 | `(([0x1dcdc9]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dcdc9]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12357d | `eax` | `?` |

## `_identFindByModuleNum`

- `_identFindByModuleNum`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122a68 to 0x122ad8.
  Established. The four-byte store through `[sp+8] +0x0` has unknown value (`mov` at 0x122a9c). The
  prior `je` bypasses it for `([sp+8] & [sp+8]) == 0x0`, with nonzero falling through (`je` at
  0x122a9a). The backward `jle` uses `? <= 0x1 (signed, 32-bit)`, leaving the changing value
  unresolved (`jle` at 0x122aac).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x122a6e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122a71 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x122a90 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122a9c | store | 4 | `[sp+8] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122a7e | `_lstFirst 0x190408` | `0x1d4e68` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x122aaf | `_lstNext 0x1904b8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x122a77 | `[sp+4] == 0xffffffff` | `[sp+4] != 0xffffffff` |
| `jne` at 0x122a96 | `Z == 0` | `not (Z == 0)` |
| `je` at 0x122a9a | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |
| `jle` at 0x122aac | `? <= 0x1 (signed, 32-bit)` | `? > 0x1 (signed, 32-bit)` |
| `jne` at 0x122abb | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122ac8 | `eax` | `?` |

## `_identClearTable`

- `_identClearTable`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122858 to 0x1228a8.
  Established. The `_lstDelete` call has first candidates 0x1d4e68 and `ret(_lstFirst)` (`call` at
  0x122871). `_free` receives `[sp-8]` as its first candidate (`call` at 0x12287d). The backward
  `jne` repeats for a nonzero `_lstFirst` result and falls through for zero (`jne` at 0x122894).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12286e | store | 4 | `sp -0x8` | `ret(_lstFirst)` |
| `mov` at 0x122879 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122871 | `_lstDelete 0x190328` | `0x1d4e68` | `ret(_lstFirst)` | `ret(_lstFirst)` | `?` | `?` | `?` |
| `call` at 0x12287d | `_free 0x17ed48` | `[sp-8]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12288a | `_lstFirst 0x190408` | `0x1d4e68` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x122894 | `(ret(_lstFirst) & ret(_lstFirst)) != 0x0` | `(ret(_lstFirst) & ret(_lstFirst)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12289b | `eax` | `0x0` |

## `_identFindIdentityNodeByUniqueID`

- `_identFindIdentityNodeByUniqueID`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1236f8 to 0x123754.
  Established. The `_ffscCompareUniqueIDs` call has unknown first candidate and `[sp+4]` as its
  second (`call` at 0x12371d); its nonzero result takes `jne` to `_lstNext`, while zero falls
  through to the exit (`jne` at 0x123727, `call` at 0x123739). The recorded EAX return value
  remains unknown (`ret` at 0x123751).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1236fd | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x123705 | `_lstFirst 0x190408` | `0x1d4e68` | `ebx` | `esi` | `ebp` | `?` | `?` |
| `call` at 0x12371d | `_ffscCompareUniqueIDs 0x125698` | `?` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x123739 | `_lstNext 0x1904b8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x123727 | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` |
| `jne` at 0x123745 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x123751 | `eax` | `?` |
