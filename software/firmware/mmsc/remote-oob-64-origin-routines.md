# MMSC remote links and out-of-band message callers in the 6.4-origin image

Read from `6.4-origin/usr/cpu/firmware/mmscfw.bin`.
Image SHA-256: `b766e559f86078526916d8bd98bbb8079294ad774cce9177928873153c6fb09b`.
The x86 a.out link address is `0x108000`. The [load map](load-map.md) explains segment mapping.
Analysed spans have exclusive ends inferred from the next text symbol.
Some include literals and padding decoded linearly after returns. Only sites reached
by known control-flow edges contribute effects to the tables.
Undecoded tails were read directly: `0x128313` to exclusive `0x128318` is
`00 90 90 90 90`, and `0x127471` to exclusive `0x127474` is `00 90 90`.
No instruction effects are credited to these bytes.
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

[Fact sheets and complete listings](disasm/remote-oob-64-origin-facts.txt) include all tool sections.

## `_remInit`

- `_remInit`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x127a08 to 0x127bc8.
  Established. The `_taskSpawn` site presents leading candidates `0x1279c5`, `0x64`, `0x0`,
  `0x4e20`, `0x127bc8`, and `0x0` (`call` at 0x127a32); its branch is taken when `ret(_taskSpawn) ==
  0xffffffff` (`je` at 0x127a3d). The two exits carry `0x0` (`ret` at 0x127a44) and `0xffffffff`
  (`ret` at 0x127a5a). Candidate positions do not establish task parameters or a listener role.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x127a32 | `_taskSpawn 0x16fc00` | `0x1279c5` | `0x64` | `0x0` | `0x4e20` | `0x127bc8` | `0x0` |
| `call` at 0x127a4d | `_ffscMsg 0x11ebd0` | `0x1279cd` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x127a3d | `ret(_taskSpawn) == 0xffffffff` | `ret(_taskSpawn) != 0xffffffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x127a44 | `eax` | `0x0` |
| `ret` at 0x127a5a | `eax` | `0xffffffff` |

## `_remAcceptConnection`

- `_remAcceptConnection`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1277f8 to 0x127a08.
  Established. The first `_timeoutRead` site presents `[sp+4]`, `sp-44`, `0x4`, and `[0x1da140]` as
  leading candidates (`call` at 0x127835). A later four-byte store through `[sp+8] +0x0` writes
  `[ret(_identFindByIPAddr)+12]` (`mov` at 0x1278ff). Another four-byte store through `[sp+12] +0x4`
  writes `0xffffffff` (`mov` at 0x127978). Peer-record and transfer-size meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x127801 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x127804 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x127807 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x127828 | load | 4 | `abs 0x1da140` | `-` |
| `mov` at 0x127846 | load | 4 | `ret(___errno) +0x0` | `-` |
| `mov` at 0x127888 | store | 4 | `sp -0x30` | `0x10` |
| `mov` at 0x127897 | load | 4 | `sp -0x2c` | `-` |
| `mov` at 0x1278ce | load | 4 | `sp -0x24` | `-` |
| `mov` at 0x1278e7 | load | 4 | `sp -0x2c` | `-` |
| `mov` at 0x1278fc | load | 4 | `ret(_identFindByIPAddr) +0xc` | `-` |
| `mov` at 0x1278ff | store | 4 | `[sp+8] +0x0` | `[ret(_identFindByIPAddr)+12]` |
| `mov` at 0x127901 | load | 4 | `ret(_identFindByIPAddr) +0x10` | `-` |
| `mov` at 0x127908 | load | 4 | `abs 0x1da134` | `-` |
| `mov` at 0x127911 | load | 4 | `sp -0x2c` | `-` |
| `mov` at 0x127926 | load | 4 | `ret(___errno) +0x0` | `-` |
| `mov` at 0x127978 | store | 4 | `[sp+12] +0x4` | `0xffffffff` |
| `cmp` at 0x12797f | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x12798c | load | 4 | `sp -0x24` | `-` |
| `mov` at 0x12799c | load | 4 | `ret(_identFindByIPAddr) +0xc` | `-` |
| `mov` at 0x1279a8 | load | 4 | `sp -0x2c` | `-` |
| `mov` at 0x1279b8 | load | 4 | `sp -0x2c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x127821 | `_ffscMsg 0x11ebd0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127835 | `_timeoutRead 0x12a538` | `[sp+4]` | `sp-44` | `0x4` | `[0x1da140]` | `ebx` | `esi` |
| `call` at 0x127841 | `___errno 0x16fa78` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127852 | `_ffscMsg 0x11ebd0` | `0x12768a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12786d | `_ffscMsgS 0x11eca0` | `0x1276b6` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12789b | `_getpeername 0x191000` | `[sp-44]` | `sp-40` | `sp-48` | `?` | `?` | `?` |
| `call` at 0x1278ac | `_ffscMsgS 0x11eca0` | `0x1276f6` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1278bc | `_identFindByIPAddr 0x122238` | `sp-36` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1278d2 | `_inet_ntoa_b 0x1868e0` | `[sp-36]` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x1278df | `_ffscMsg 0x11ebd0` | `0x12771a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1278eb | `_close 0x186ed0` | `[sp-44]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127915 | `_timeoutRead 0x12a538` | `[sp-44]` | `[sp+12]` | `0x70` | `[0x1da134]` | `?` | `?` |
| `call` at 0x127921 | `___errno 0x16fa78` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12793d | `_ffscMsgS 0x11eca0` | `0x12777a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127953 | `_ffscMsg 0x11ebd0` | `?` | `ret(_timeoutRead)` | `?` | `?` | `?` | `?` |
| `call` at 0x127970 | `_bcopy 0x1747fc` | `0x12a8c8` | `[sp+12]` | `0x70` | `?` | `?` | `?` |
| `call` at 0x127990 | `_inet_ntoa_b 0x1868e0` | `[sp-36]` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x1279a0 | `_ffscRackIDToString 0x12594c` | `[ret(_identFindByIPAddr)+12]` | `sp-60` | `?` | `?` | `?` | `?` |
| `call` at 0x1279b3 | `_ffscMsg 0x11ebd0` | `0x1277c6` | `sp-60` | `sp-24` | `[sp-44]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x12780c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x12781a | `([sp+12] & [sp+12]) != 0x0` | `([sp+12] & [sp+12]) == 0x0` |
| `jge` at 0x12783f | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x12784b | `[ret(___errno)] != 0x4` | `[ret(___errno)] == 0x4` |
| `je` at 0x12787b | `ret(_timeoutRead) == 0x4` | `ret(_timeoutRead) != 0x4` |
| `je` at 0x1278a5 | `(ret(_getpeername) & ret(_getpeername)) == 0x0` | `(ret(_getpeername) & ret(_getpeername)) != 0x0` |
| `jne` at 0x1278c8 | `(ret(_identFindByIPAddr) & ret(_identFindByIPAddr)) != 0x0` | `(ret(_identFindByIPAddr) & ret(_identFindByIPAddr)) == 0x0` |
| `je` at 0x1278fa | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |
| `je` at 0x127906 | `(? & 0x1) == 0x0` | `(? & 0x1) != 0x0` |
| `jge` at 0x12791f | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x12792b | `[ret(___errno)] != 0x4` | `[ret(___errno)] == 0x4` |
| `je` at 0x12794b | `ret(_timeoutRead) == 0x70` | `ret(_timeoutRead) != 0x70` |
| `jge` at 0x127986 | `([0x1da698]:1 & 0xff) >= 0x0 (signed, 8-bit)` | `([0x1da698]:1 & 0xff) < 0x0 (signed, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1279c4 | `eax` | `?` |

## `_remListener`

- `_remListener`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x127bc8 to 0x127f18.
  Established. The two-byte store at `sp -0x24` writes `0x2` (`mov` at 0x127bdd), while the two-byte
  store at `sp -0x22` retains an unknown value (`mov` at 0x127bf8). The `_socket` site presents
  `0x2`, `0x1`, and `0x0` as its first three candidates (`call` at 0x127c09). The `_write` site
  presents `[0x1da8e4]`, `sp-44`, and `0x4` (`call` at 0x127d40). No socket-family or structure-size
  meaning is assigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x127bdd | store | 2 | `sp -0x24` | `0x2` |
| `mov` at 0x127be3 | load | 2 | `abs 0x1da168` | `-` |
| `movzx` at 0x127bee | load | 1 | `abs 0x1da169` | `-` |
| `mov` at 0x127bf8 | store | 2 | `sp -0x22` | `?` |
| `mov` at 0x127bfc | store | 4 | `sp -0x20` | `0x0` |
| `mov` at 0x127c11 | store | 4 | `sp -0x30` | `ret(_socket)` |
| `mov` at 0x127c2b | load | 4 | `sp -0x30` | `-` |
| `mov` at 0x127c58 | load | 4 | `abs 0x1da16c` | `-` |
| `mov` at 0x127c5e | load | 4 | `sp -0x30` | `-` |
| `cmp` at 0x127c88 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x127c91 | load | 4 | `sp -0x30` | `-` |
| `mov` at 0x127cb8 | load | 4 | `abs 0x1ee88c` | `-` |
| `mov` at 0x127ce8 | store | 4 | `sp -0x28` | `0x10` |
| `mov` at 0x127cf7 | load | 4 | `sp -0x30` | `-` |
| `mov` at 0x127d03 | store | 4 | `sp -0x2c` | `ret(_accept)` |
| `mov` at 0x127d1d | load | 4 | `abs 0x1da130` | `-` |
| `mov` at 0x127d3a | load | 4 | `abs 0x1da8e4` | `-` |
| `cmp` at 0x127d66 | load | 4 | `sp -0x30` | `-` |
| `mov` at 0x127d7f | load | 4 | `sp -0x34` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x127bd5 | `_bzero 0x170f20` | `sp-36` | `0x10` | `ebx` | `?` | `?` | `?` |
| `call` at 0x127c09 | `_socket 0x190030` | `0x2` | `0x1` | `0x0` | `?` | `?` | `?` |
| `call` at 0x127c1d | `_ffscMsgS 0x11eca0` | `0x127a5b` | `?` | `ret(_socket)` | `?` | `?` | `?` |
| `call` at 0x127c2f | `_bind 0x190100` | `ret(_socket)` | `sp-36` | `0x10` | `?` | `ret(_socket)` | `?` |
| `call` at 0x127c40 | `_ffscMsgS 0x11eca0` | `0x127a7a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127c62 | `_listen 0x190190` | `[sp-48]` | `[0x1da16c]` | `?` | `?` | `?` | `?` |
| `call` at 0x127c73 | `_ffscMsgS 0x11eca0` | `0x127aa2` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127c9a | `_ffscMsg 0x11ebd0` | `0x127ac1` | `[sp-48]` | `?` | `?` | `?` | `?` |
| `call` at 0x127ca2 | `_timeoutInit 0x12a408` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127cc1 | `_setjmp 0x18fe80` | `[0x1ee88c]+4` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127cd6 | `_ffscMsg 0x11ebd0` | `0x127b18` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127cde | `_timeoutCancel 0x12a308` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127cfb | `_accept 0x1901e0` | `[sp-48]` | `sp-20` | `sp-40` | `?` | `?` | `?` |
| `call` at 0x127d23 | `_timeoutSet 0x12a658` | `[0x1da130]` | `0x127b6d` | `?` | `?` | `ret(_accept)` | `?` |
| `call` at 0x127d2f | `_timeoutEnable 0x12a348` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127d40 | `_write 0x186f70` | `[0x1da8e4]` | `sp-44` | `0x4` | `?` | `?` | `?` |
| `call` at 0x127d4a | `_timeoutDisable 0x12a328` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127d4f | `_timeoutCancel 0x12a308` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127d5e | `_ffscMsgS 0x11eca0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127d75 | `_ffscMsg 0x11ebd0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x127c16 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jge` at 0x127c39 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jge` at 0x127c6c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jge` at 0x127c8f | `([0x1da698]:1 & 0xff) >= 0x0 (signed, 8-bit)` | `([0x1da698]:1 & 0xff) < 0x0 (signed, 8-bit)` |
| `je` at 0x127ca9 | `(ret(_timeoutInit) & ret(_timeoutInit)) == 0x0` | `(ret(_timeoutInit) & ret(_timeoutInit)) != 0x0` |
| `je` at 0x127ccb | `(ret(_setjmp) & ret(_setjmp)) == 0x0` | `(ret(_setjmp) & ret(_setjmp)) != 0x0` |
| `jge` at 0x127d08 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x127d2d | `(ret(_timeoutSet) & ret(_timeoutSet)) != 0x0` | `(ret(_timeoutSet) & ret(_timeoutSet)) == 0x0` |
| `je` at 0x127d57 | `ret(_write) == 0x4` | `ret(_write) != 0x4` |
| `jge` at 0x127d6a | `[sp-48] >= 0x0 (signed, 32-bit)` | `[sp-48] < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x127d85 | `eax` | `0xffffffff` |

## `_remOpenConnection`

- `_remOpenConnection`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x127f18 to 0x1281a8.
  Established. The opening branch is taken when `[sp+4] != 0xffffffff` (`jne` at 0x127f23).
  Four-byte stores retain `ret(_identFindByRackID)` at `sp -0x2c` (`mov` at 0x127f54) and
  `ret(_socket)` at `sp -0x30` (`mov` at 0x127f86). The `_timeoutWrite` call presents `[sp-48]`,
  `[sp+8]`, `0x70`, and `[0x1da154]` as leading candidates (`call` at 0x128092). These candidates do
  not establish a transferred record size or timeout unit.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x127f1f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x127f48 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x127f54 | store | 4 | `sp -0x2c` | `ret(_identFindByRackID)` |
| `mov` at 0x127f5b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x127f86 | store | 4 | `sp -0x30` | `ret(_socket)` |
| `mov` at 0x127fb6 | store | 2 | `sp -0x14` | `0x2` |
| `mov` at 0x127fbc | load | 2 | `abs 0x1da168` | `-` |
| `movzx` at 0x127fc7 | load | 1 | `abs 0x1da169` | `-` |
| `mov` at 0x127fd1 | store | 2 | `sp -0x12` | `?` |
| `mov` at 0x127fd5 | load | 4 | `sp -0x2c` | `-` |
| `mov` at 0x127fd8 | load | 4 | `[sp-44] +0x8` | `-` |
| `mov` at 0x127fdb | store | 4 | `sp -0x10` | `[[sp-44]+8]` |
| `mov` at 0x127fde | load | 4 | `abs 0x1ee88c` | `-` |
| `mov` at 0x12801d | load | 4 | `abs 0x1da164` | `-` |
| `mov` at 0x12803e | load | 4 | `sp -0x30` | `-` |
| `mov` at 0x128078 | load | 4 | `sp -0x2c` | `-` |
| `mov` at 0x12807b | load | 4 | `[sp-44] +0x10` | `-` |
| `mov` at 0x128082 | load | 4 | `abs 0x1da154` | `-` |
| `mov` at 0x12808a | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12808e | load | 4 | `sp -0x30` | `-` |
| `mov` at 0x1280a5 | load | 4 | `ret(___errno) +0x0` | `-` |
| `mov` at 0x1280db | load | 4 | `sp -0x30` | `-` |
| `cmp` at 0x1280f8 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x128105 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x128111 | load | 4 | `sp -0x30` | `-` |
| `mov` at 0x128116 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x128124 | load | 4 | `sp -0x30` | `-` |
| `mov` at 0x128127 | load | 4 | `sp -0x34` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x127f2a | `_ffscMsg 0x11ebd0` | `0x127d86` | `ebx` | `?` | `?` | `?` | `?` |
| `call` at 0x127f4c | `_identFindByRackID 0x1222f8` | `[sp+4]` | `ebx` | `?` | `?` | `?` | `?` |
| `call` at 0x127f64 | `_ffscMsg 0x11ebd0` | `0x127db8` | `[sp+4]` | `?` | `?` | `ret(_identFindByRackID)` | `?` |
| `call` at 0x127f7e | `_socket 0x190030` | `0x2` | `0x1` | `0x0` | `?` | `?` | `ret(_identFindByRackID)` |
| `call` at 0x127f92 | `_ffscMsgS 0x11eca0` | `0x127de0` | `?` | `ret(_socket)` | `?` | `?` | `?` |
| `call` at 0x127fae | `_bzero 0x170f20` | `sp-20` | `0x10` | `?` | `ret(_socket)` | `?` | `?` |
| `call` at 0x127fe7 | `_setjmp 0x18fe80` | `[0x1ee88c]+4` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127ff8 | `_ffscMsg 0x11ebd0` | `0x127e0b` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128000 | `_timeoutCancel 0x12a308` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128023 | `_timeoutSet 0x12a658` | `[0x1da164]` | `0x127e37` | `?` | `?` | `?` | `?` |
| `call` at 0x128033 | `_timeoutEnable 0x12a348` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128042 | `_connect 0x190390` | `[sp-48]` | `sp-20` | `0x10` | `?` | `?` | `?` |
| `call` at 0x12804c | `_timeoutDisable 0x12a328` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128051 | `_timeoutCancel 0x12a308` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12805f | `_ffscMsgS 0x11eca0` | `0x127e3f` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128092 | `_timeoutWrite 0x12a788` | `[sp-48]` | `[sp+8]` | `0x70` | `[0x1da154]` | `?` | `?` |
| `call` at 0x1280a0 | `___errno 0x16fa78` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1280b1 | `_ffscMsg 0x11ebd0` | `0x127e60` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1280bd | `_ffscMsgS 0x11eca0` | `0x127e8b` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1280d3 | `_ffscMsg 0x11ebd0` | `0x127eb2` | `ret(_timeoutWrite)` | `?` | `?` | `?` | `?` |
| `call` at 0x1280df | `_close 0x186ed0` | `[sp-48]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128109 | `_inet_ntoa_b 0x1868e0` | `[sp-16]` | `sp-40` | `?` | `?` | `?` | `?` |
| `call` at 0x12811f | `_ffscMsg 0x11ebd0` | `0x127ed6` | `[sp+4]` | `sp-40` | `[sp-48]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x127f23 | `[sp+4] != 0xffffffff` | `[sp+4] == 0xffffffff` |
| `jne` at 0x127f59 | `(ret(_identFindByRackID) & ret(_identFindByRackID)) != 0x0` | `(ret(_identFindByRackID) & ret(_identFindByRackID)) == 0x0` |
| `jge` at 0x127f8b | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x127ff1 | `(ret(_setjmp) & ret(_setjmp)) == 0x0` | `(ret(_setjmp) & ret(_setjmp)) != 0x0` |
| `jne` at 0x12802d | `(ret(_timeoutSet) & ret(_timeoutSet)) != 0x0` | `(ret(_timeoutSet) & ret(_timeoutSet)) == 0x0` |
| `jge` at 0x128058 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x128080 | `(? & 0x1) == 0x0` | `(? & 0x1) != 0x0` |
| `jge` at 0x12809e | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x1280aa | `[ret(___errno)] != 0x4` | `[ret(___errno)] == 0x4` |
| `je` at 0x1280cb | `ret(_timeoutWrite) == 0x70` | `ret(_timeoutWrite) != 0x70` |
| `jge` at 0x1280ff | `([0x1da698]:1 & 0xff) >= 0x0 (signed, 8-bit)` | `([0x1da698]:1 & 0xff) < 0x0 (signed, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12812d | `eax` | `?` |

## `_remReceiveData`

- `_remReceiveData`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1281a8 to 0x128318.
  Established. The `_timeoutRead` site presents candidate zero `[sp+4]`, unknown candidates one and
  two, and candidate three `[0x1da148]` (`call` at 0x1281ce). The later branch is taken when
  `[ret(___errno)] != 0x4` (`jne` at 0x1281e4). The `_close` site has candidate zero `[sp+4]`
  (`call` at 0x128207). Neither a received byte count nor connection closure is established by these
  names.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1281ae | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1281b3 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1281c0 | load | 4 | `abs 0x1da148` | `-` |
| `mov` at 0x1281c7 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1281df | load | 4 | `ret(___errno) +0x0` | `-` |
| `mov` at 0x128214 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x128219 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128220 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x128240 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x12824a | load | 4 | `sp +0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1281ce | `_timeoutRead 0x12a538` | `[sp+4]` | `?` | `?` | `[0x1da148]` | `?` | `?` |
| `call` at 0x1281da | `___errno 0x16fa78` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1281ec | `_ffscMsg 0x11ebd0` | `0x12812e` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x1281fe | `_ffscMsgS 0x11eca0` | `0x128150` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x128207 | `_close 0x186ed0` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128224 | `_sprintf 0x171680` | `?` | `0x128171` | `?` | `?` | `?` | `?` |
| `call` at 0x128253 | `_ffscMsg 0x11ebd0` | `0x12817c` | `[sp+12]` | `[sp+4]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x1281b8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jge` at 0x1281d8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x1281e4 | `[ret(___errno)] != 0x4` | `[ret(___errno)] == 0x4` |
| `jg` at 0x12823e | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `jge` at 0x128247 | `([0x1da698]:1 & 0xff) >= 0x0 (signed, 8-bit)` | `([0x1da698]:1 & 0xff) < 0x0 (signed, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x128263 | `eax` | `?` |

## `_remSendData`

- `_remSendData`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x128318 to 0x128568.
  Established. The four-byte store through `[[sp+4]+136] +0x1c` writes `0x1` (`mov` at 0x128363).
  The `_msgWrite` site presents leading candidates `[[sp+4]+136]` and `[0x1da158]` (`call` at
  0x128377). The later `_timeoutWrite` site has candidate zero `[[[sp+4]+136]+20]`, unknown
  candidates one and two, and candidate three `[0x1da154]` (`call` at 0x1283db). The unresolved
  candidates do not establish a transfer extent.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12831e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x128321 | load | 4 | `sp +0xc` | `-` |
| `cmp` at 0x128324 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x12832d | load | 4 | `[sp+4] +0x84` | `-` |
| `mov` at 0x128333 | load | 4 | `[[sp+4]+132] +0x14` | `-` |
| `mov` at 0x12834b | load | 4 | `[sp+4] +0x88` | `-` |
| `mov` at 0x128351 | load | 4 | `[[sp+4]+136] +0x8` | `-` |
| `mov` at 0x12835d | load | 4 | `[sp+4] +0x88` | `-` |
| `mov` at 0x128363 | store | 4 | `[[sp+4]+136] +0x1c` | `0x1` |
| `mov` at 0x12836a | load | 4 | `abs 0x1da158` | `-` |
| `mov` at 0x128370 | load | 4 | `[sp+4] +0x88` | `-` |
| `mov` at 0x128383 | load | 4 | `[sp+4] +0x88` | `-` |
| `mov` at 0x128389 | load | 4 | `[[sp+4]+136] +0x1c` | `-` |
| `mov` at 0x128391 | load | 4 | `[sp+4] +0x88` | `-` |
| `mov` at 0x128397 | load | 4 | `[[sp+4]+136] +0x1c` | `-` |
| `mov` at 0x12839b | load | 4 | `[sp+4] +0x88` | `-` |
| `mov` at 0x1283c4 | load | 4 | `abs 0x1da154` | `-` |
| `mov` at 0x1283cb | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1283d1 | load | 4 | `[sp+4] +0x88` | `-` |
| `mov` at 0x1283d7 | load | 4 | `[[sp+4]+136] +0x14` | `-` |
| `mov` at 0x1283ec | load | 4 | `ret(___errno) +0x0` | `-` |
| `mov` at 0x1283f3 | load | 4 | `[sp+4] +0x88` | `-` |
| `mov` at 0x1283f9 | load | 4 | `[[sp+4]+136] +0x14` | `-` |
| `mov` at 0x128418 | load | 4 | `[sp+4] +0x88` | `-` |
| `mov` at 0x12841e | load | 4 | `[[sp+4]+136] +0x14` | `-` |
| `mov` at 0x128434 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x128439 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128440 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12833d | `_ffscMsg 0x11ebd0` | `0x128264` | `[sp+12]` | `[[[sp+4]+132]+20]` | `ebx` | `esi` | `edi` |
| `call` at 0x128355 | `_sprintf 0x171680` | `[[[sp+4]+136]+8]` | `0x128286` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x128377 | `_msgWrite 0x126680` | `[[sp+4]+136]` | `[0x1da158]` | `?` | `?` | `?` | `?` |
| `call` at 0x1283a7 | `_ffscMsg 0x11ebd0` | `0x128290` | `[[sp+4]+136]` | `[[[sp+4]+136]+28]` | `?` | `?` | `?` |
| `call` at 0x1283db | `_timeoutWrite 0x12a788` | `[[[sp+4]+136]+20]` | `?` | `?` | `[0x1da154]` | `?` | `?` |
| `call` at 0x1283e7 | `___errno 0x16fa78` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128402 | `_ffscMsg 0x11ebd0` | `0x1282c9` | `[[[sp+4]+136]+20]` | `?` | `?` | `?` | `?` |
| `call` at 0x128427 | `_ffscMsgS 0x11eca0` | `0x1282e9` | `[[[sp+4]+136]+20]` | `?` | `?` | `?` | `?` |
| `call` at 0x128444 | `_sprintf 0x171680` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x12832b | `([0x1da698]:1 & 0xff) >= 0x0 (signed, 8-bit)` | `([0x1da698]:1 & 0xff) < 0x0 (signed, 8-bit)` |
| `jne` at 0x128381 | `(ret(_msgWrite) & ret(_msgWrite)) != 0x0` | `(ret(_msgWrite) & ret(_msgWrite)) == 0x0` |
| `je` at 0x12838f | `[[[sp+4]+136]+28] == 0xb` | `[[[sp+4]+136]+28] != 0xb` |
| `jle` at 0x1283bc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jge` at 0x1283e5 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x1283f1 | `[ret(___errno)] != 0x4` | `[ret(___errno)] == 0x4` |
| `jg` at 0x12845e | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12846f | `eax` | `?` |

## `_remStartConnectCommand`

- `_remStartConnectCommand`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x128568 to 0x1287a8.
  Established. The `_remStartRemoteCommand` call presents `[sp+4]`, `[sp+8]`, `[sp+12]`, and
  `[sp+16]` as its first four candidates (`call` at 0x1285d1). A four-byte store at an unknown
  effective address writes `0x0` (`mov` at 0x1285bd). Later four-byte stores at unknown targets
  write `0xffffffff` (`mov` at 0x128716, `mov` at 0x12874b). A connection-state role for these
  stores is unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x128571 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x128574 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x128577 | load | 4 | `(unknown)` | `-` |
| `test` at 0x12857e | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x12858b | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1285a8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1285b7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1285bd | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x1285c4 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x1285c8 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1285cd | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1285d9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1285df | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1285eb | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1285f7 | load | 4 | `(unknown)` | `-` |
| `test` at 0x128602 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x128618 | load | 4 | `abs 0x1da134` | `-` |
| `mov` at 0x12861e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128631 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128637 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12863f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128645 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128658 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x128661 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128671 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x12867e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128684 | load | 4 | `(unknown)` | `-` |
| `test` at 0x1286a8 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x1286b1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1286b5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1286c8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1286ce | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1286e0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1286e6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12870d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128713 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128716 | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x128736 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12873c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128745 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12874b | store | 4 | `(unknown)` | `0xffffffff` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x128591 | `_ffscRackIDToString 0x12594c` | `[0x1ee564]` | `sp-16` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x1285a0 | `_ffscMsg 0x11ebd0` | `0x128470` | `[sp+8]` | `sp-16` | `?` | `?` | `?` |
| `call` at 0x1285af | `_msgClear 0x125ea0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1285d1 | `_remStartRemoteCommand 0x128918` | `[sp+4]` | `[sp+8]` | `[sp+12]` | `[sp+16]` | `?` | `?` |
| `call` at 0x128610 | `_ffscMsg 0x11ebd0` | `0x1284a2` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128625 | `_msgRead 0x126160` | `?` | `[0x1da134]` | `?` | `?` | `?` | `?` |
| `call` at 0x128665 | `_sprintf 0x171680` | `?` | `0x1284e3` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x128675 | `_ffscRackIDToString 0x12594c` | `[sp+12]` | `sp-28` | `?` | `?` | `?` | `?` |
| `call` at 0x12868e | `_ffscMsg 0x11ebd0` | `0x1284eb` | `sp-28` | `?` | `?` | `?` | `?` |
| `call` at 0x1286be | `_ffscMsg 0x11ebd0` | `0x128513` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1286d2 | `_strchr 0x17de58` | `?` | `0x3a` | `?` | `?` | `?` | `?` |
| `call` at 0x128701 | `_strncmp 0x1749b8` | `?` | `0x128534` | `0x5` | `?` | `?` | `?` |
| `call` at 0x12872e | `_ffscMsg 0x11ebd0` | `0x12853a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128740 | `_close 0x186ed0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x128585 | `(([0x1da698]:1 & 0xff) & 0x20) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x20) != 0x0` |
| `je` at 0x1285e5 | `? == 0xb` | `? != 0xb` |
| `je` at 0x1285f1 | `? == 0xa` | `? != 0xa` |
| `jl` at 0x1285fc | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x128609 | `(([0x1da698]:1 & 0xff) & 0x20) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x20) != 0x0` |
| `jne` at 0x12862f | `(ret(_msgRead) & ret(_msgRead)) != 0x0` | `(ret(_msgRead) & ret(_msgRead)) == 0x0` |
| `je` at 0x12863d | `? == 0xb` | `? != 0xb` |
| `jne` at 0x128650 | `? != 0xa` | `? == 0xa` |
| `je` at 0x1286af | `(([0x1da698]:1 & 0xff) & 0x20) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x20) != 0x0` |
| `jne` at 0x1286de | `(ret(_strchr) & ret(_strchr)) != 0x0` | `(ret(_strchr) & ret(_strchr)) == 0x0` |
| `jne` at 0x12870b | `(ret(_strncmp) & ret(_strncmp)) != 0x0` | `(ret(_strncmp) & ret(_strncmp)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x128760 | `eax` | `?` |

## `_remStartDataRequest`

- `_remStartDataRequest`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1287a8 to 0x128918.
  Established. The `_remStartConnectCommand` site presents `[sp+4]`, `[sp+8]`, `[sp+12]`, and
  `[sp+16]` as leading candidates (`call` at 0x1287c1). The `_sscanf` site retains unknown candidate
  zero, candidate one `0x128772`, and candidate two `[sp+20]` (`call` at 0x128823); its result
  branch is taken when `ret(_sscanf) != 0x1` (`jne` at 0x12882e). The subsequent `_close` site
  presents candidate zero `ret(_remStartConnectCommand)` (`call` at 0x128857).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1287ae | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1287b1 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1287b4 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1287b7 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x1287ba | load | 4 | `sp +0x14` | `-` |
| `mov` at 0x1287cf | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1287d6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1287dc | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1287e5 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1287ea | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1287f1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128812 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128819 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12881f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128838 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12883f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128845 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128864 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x128869 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128870 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1287c1 | `_remStartConnectCommand 0x128568` | `[sp+4]` | `[sp+8]` | `[sp+12]` | `[sp+16]` | `ebx` | `esi` |
| `call` at 0x1287f5 | `_sprintf 0x171680` | `?` | `0x128761` | `?` | `?` | `?` | `?` |
| `call` at 0x128823 | `_sscanf 0x172430` | `?` | `0x128772` | `[sp+20]` | `?` | `?` | `?` |
| `call` at 0x12884e | `_ffscMsg 0x11ebd0` | `0x12877b` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128857 | `_close 0x186ed0` | `ret(_remStartConnectCommand)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128874 | `_sprintf 0x171680` | `?` | `0x1282bb` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x1287cd | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x12880a | `([sp+20] & [sp+20]) == 0x0` | `([sp+20] & [sp+20]) != 0x0` |
| `jne` at 0x12882e | `ret(_sscanf) != 0x1` | `ret(_sscanf) == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x128887 | `eax` | `?` |

## `_remStartRemoteCommand`

- `_remStartRemoteCommand`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x128918 to 0x128b04.
  Established. The `_remOpenConnection` call presents `[sp+12]` and `sp-120` as its first two
  candidates (`call` at 0x12893d). Four-byte stores at unknown effective addresses retain
  `ret(_remOpenConnection)` (`mov` at 0x12897e, `mov` at 0x128987). Other four-byte stores at
  unresolved targets include `0xb` (`mov` at 0x128966), `0x1` (`mov` at 0x128a07), and `0x3` (`mov`
  at 0x128a7c). The literal values do not identify connection states.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x128921 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x128924 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x128927 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128951 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128957 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128960 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128966 | store | 4 | `(unknown)` | `0xb` |
| `mov` at 0x128978 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12897e | store | 4 | `(unknown)` | `ret(_remOpenConnection)` |
| `mov` at 0x128981 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128987 | store | 4 | `(unknown)` | `ret(_remOpenConnection)` |
| `mov` at 0x12898a | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x12898d | load | 1 | `([sp+12] + [sp+16]) +0x4` | `-` |
| `mov` at 0x1289ac | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x1289af | load | 1 | `([sp+12] + [sp+16]) +0x4` | `-` |
| `mov` at 0x1289bd | load | 1 | `(unknown)` | `-` |
| `mov` at 0x1289c3 | store | 1 | `(unknown)` | `?` |
| `mov` at 0x1289ce | store | 1 | `(unknown)` | `0x0` |
| `mov` at 0x1289d3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1289e2 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1289ef | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1289f5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128a01 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128a07 | store | 4 | `(unknown)` | `0x1` |
| `cmp` at 0x128a0e | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x128a18 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128a1e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128a2f | load | 4 | `abs 0x1da154` | `-` |
| `mov` at 0x128a35 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128a4e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128a54 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128a68 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128a6e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128a76 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128a7c | store | 4 | `(unknown)` | `0x3` |
| `mov` at 0x128a88 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128aa0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128aa6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128ab2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128ab8 | store | 4 | `(unknown)` | `0xb` |
| `mov` at 0x128abf | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128ac5 | store | 4 | `(unknown)` | `0xb` |
| `mov` at 0x128acc | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128ad2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128adb | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128ae1 | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x128ae8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128aee | store | 4 | `(unknown)` | `0xffffffff` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x128933 | `_userExtractUserInfo 0x12ab28` | `[sp+4]` | `sp-120` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x12893d | `_remOpenConnection 0x127f18` | `[sp+12]` | `sp-120` | `?` | `?` | `?` | `?` |
| `call` at 0x12895b | `_sprintf 0x171680` | `?` | `0x128888` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x12899e | `_strcpy 0x17dfdc` | `sp-8` | `0x12889b` | `?` | `?` | `?` | `?` |
| `call` at 0x1289da | `_msgClear 0x125ea0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1289f9 | `_sprintf 0x171680` | `?` | `0x12889d` | `sp-8` | `[sp+8]` | `?` | `?` |
| `call` at 0x128a27 | `_ffscMsg 0x11ebd0` | `0x1288a9` | `?` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x128a3c | `_msgWrite 0x126680` | `?` | `[0x1da154]` | `?` | `?` | `?` | `?` |
| `call` at 0x128a58 | `_sprintf 0x171680` | `?` | `0x1288d0` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x128a92 | `_ffscMsg 0x11ebd0` | `0x1288e0` | `[sp+12]` | `?` | `?` | `?` | `?` |
| `call` at 0x128aaa | `_sprintf 0x171680` | `?` | `0x1288d0` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x128ad6 | `_close 0x186ed0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x128949 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jge` at 0x128993 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jae` at 0x1289bb | `C == 0` | `not (C == 0)` |
| `jbe` at 0x1289cc | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |
| `jge` at 0x128a15 | `([0x1da698]:1 & 0xff) >= 0x0 (signed, 8-bit)` | `([0x1da698]:1 & 0xff) < 0x0 (signed, 8-bit)` |
| `je` at 0x128a46 | `(ret(_msgWrite) & ret(_msgWrite)) == 0x0` | `(ret(_msgWrite) & ret(_msgWrite)) != 0x0` |
| `jne` at 0x128a74 | `? != 0xb` | `? == 0xb` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x128b01 | `eax` | `?` |

## `_oobChecksumMsg`

- `_oobChecksumMsg`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1270b4 to 0x1271c4.
  Established. The byte loads through `[sp+4] +0x2` (`mov` at 0x1270bc) and `[sp+4] +0x3` (`mov` at
  0x1270c4) precede a branch taken when `0x1 >= ? (signed, 32-bit)` (`jge` at 0x1270dd). The later
  byte load has an unknown effective address (`mov` at 0x1270e0). The exit value remains unknown
  (`ret` at 0x1270fa). A checksum formula cannot be filled in from the routine name.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1270b9 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1270bc | load | 1 | `[sp+4] +0x2` | `-` |
| `mov` at 0x1270c4 | load | 1 | `[sp+4] +0x3` | `-` |
| `mov` at 0x1270e0 | load | 1 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x1270dd | `0x1 >= ? (signed, 32-bit)` | `0x1 < ? (signed, 32-bit)` |
| `jl` at 0x1270ed | `? < ? (signed, 32-bit)` | `? >= ? (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1270fa | `eax` | `?` |

## `_oobProcessMsg`

- `_oobProcessMsg`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1271c4 to 0x127354.
  Established. The first byte-value branch is taken when `([[sp+8]]:1 & 0xff) == 0xa0` (`je` at
  0x1271e8). The `_oobSendGraphicsCmd` site presents `[sp+4]` and `[sp+8]` as its first two
  candidates (`call` at 0x1272d6). The `_oobSendFFSCCmd` site instead presents `[sp+4]` and
  `[sp+8]+4` (`call` at 0x1272f3). The later branch is taken when `([[sp+4]+60] & 0x40000) != 0x0`
  (`jne` at 0x1272ec). No request-code meaning is inferred.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1271c9 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1271cc | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1271e4 | load | 1 | `[sp+8] +0x0` | `-` |
| `mov` at 0x1271ea | load | 1 | `[sp+8] +0x0` | `-` |
| `mov` at 0x12720f | load | 1 | `[sp+8] +0x2` | `-` |
| `mov` at 0x127217 | load | 1 | `[sp+8] +0x3` | `-` |
| `mov` at 0x127227 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x127232 | load | 1 | `[sp+8] +0x2` | `-` |
| `mov` at 0x12723a | load | 1 | `[sp+8] +0x3` | `-` |
| `mov` at 0x12724a | load | 1 | `(unknown)` | `-` |
| `test` at 0x127274 | load | 1 | `abs 0x1da699` | `-` |
| `mov` at 0x12727d | load | 1 | `[sp+8] +0x1` | `-` |
| `mov` at 0x127294 | load | 1 | `[sp+8] +0x1` | `-` |
| `mov` at 0x1272e4 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x127319 | load | 4 | `[sp+4] +0x6c` | `-` |
| `mov` at 0x12731c | load | 4 | `[[sp+4]+108] +0x0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1271d9 | `_ffscMsg 0x11ebd0` | `0x1270fb` | `[sp+4]` | `ebx` | `esi` | `ebp` | `?` |
| `call` at 0x1271f8 | `_ffscMsg 0x11ebd0` | `0x127125` | `[sp+4]` | `?` | `ebx` | `esi` | `ebp` |
| `call` at 0x127205 | `_oobChecksumMsg 0x1270b4` | `[sp+8]` | `ebx` | `esi` | `ebp` | `?` | `?` |
| `call` at 0x127259 | `_ffscMsg 0x11ebd0` | `0x12715f` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x12728c | `_ffscMsg 0x11ebd0` | `0x12719e` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x1272d6 | `_oobSendGraphicsCmd 0x127474` | `[sp+4]` | `[sp+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x1272f3 | `_oobSendFFSCCmd 0x127354` | `[sp+4]` | `[sp+8]+4` | `?` | `?` | `?` | `?` |
| `call` at 0x12731f | `_oobSendMsg 0x1275c4` | `[[[sp+4]+108]]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x1271d1 | `([sp+8] & [sp+8]) != 0x0` | `([sp+8] & [sp+8]) == 0x0` |
| `je` at 0x1271e8 | `([[sp+8]]:1 & 0xff) == 0xa0` | `([[sp+8]]:1 & 0xff) != 0xa0` |
| `je` at 0x12722c | `? == ?` | `? != ?` |
| `je` at 0x12727b | `(([0x1da699]:1 & 0xff) & 0x8) == 0x0` | `(([0x1da699]:1 & 0xff) & 0x8) != 0x0` |
| `jg` at 0x12729f | `? > 0xb (signed, 32-bit)` | `? <= 0xb (signed, 32-bit)` |
| `jge` at 0x1272a4 | `? >= 0x1 (signed, 32-bit)` | `? < 0x1 (signed, 32-bit)` |
| `je` at 0x1272a8 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x1272b7 | `? == 0x3d` | `? != 0x3d` |
| `je` at 0x1272bc | `? == 0x3e` | `? != 0x3e` |
| `jne` at 0x1272ec | `([[sp+4]+60] & 0x40000) != 0x0` | `([[sp+4]+60] & 0x40000) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12732c | `eax` | `?` |

## `_oobSendGraphicsCmd`

- `_oobSendGraphicsCmd`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x127474 to 0x1275c4.
  Established. Byte stores write `0xa0` at `sp -0x704` (`mov` at 0x1274fd) and `0x5` at `sp -0x703`
  (`mov` at 0x127504). The byte stores at `sp -0x702` (`mov` at 0x12751a) and `sp -0x701` (`mov` at
  0x127522) retain unknown values. The `_oobSendMsg` site presents `[[[sp+4]+108]]` and `sp-1796` as
  its first two candidates (`call` at 0x127563). Those bytes do not establish a message format or
  graphics-command identity.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x127480 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x127483 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1274b3 | load | 1 | `[sp+4] +0x93` | `-` |
| `mov` at 0x1274fd | store | 1 | `sp -0x704` | `0xa0` |
| `mov` at 0x127504 | store | 1 | `sp -0x703` | `0x5` |
| `mov` at 0x12751a | store | 1 | `sp -0x702` | `?` |
| `mov` at 0x127522 | store | 1 | `sp -0x701` | `?` |
| `mov` at 0x12753e | store | 1 | `(ret(_strlen)+1 + sp-4) -0x6fd` | `0x0` |
| `mov` at 0x12754f | store | 1 | `(ret(_strlen)+1 + sp-4) -0x6fc` | `?` |
| `mov` at 0x12755d | load | 4 | `[sp+4] +0x6c` | `-` |
| `mov` at 0x127560 | load | 4 | `[[sp+4]+108] +0x0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12749a | `_sprintf 0x171680` | `sp-248` | `0x127451` | `[sp+8]` | `sp-1796` | `ebx` | `esi` |
| `call` at 0x1274ab | `_cmSendFFSCCmd 0x11a290` | `[sp+4]` | `sp-248` | `sp-332` | `?` | `?` | `?` |
| `call` at 0x1274be | `_strchr 0x17de58` | `sp-332` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1274dd | `_strncmp 0x1749b8` | `?` | `0x12746f` | `0x2` | `?` | `?` | `?` |
| `call` at 0x1274f0 | `_strlen 0x172f14` | `sp-332` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127536 | `_strcpy 0x17dfdc` | `sp-1792` | `sp-332` | `?` | `?` | `?` | `?` |
| `call` at 0x127547 | `_oobChecksumMsg 0x1270b4` | `sp-1796` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127563 | `_oobSendMsg 0x1275c4` | `[[[sp+4]+108]]` | `sp-1796` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x1274c8 | `(ret(_strchr) & ret(_strchr)) != 0x0` | `(ret(_strchr) & ret(_strchr)) == 0x0` |
| `je` at 0x1274e7 | `(ret(_strncmp) & ret(_strncmp)) == 0x0` | `(ret(_strncmp) & ret(_strncmp)) != 0x0` |
| `jge` at 0x12750f | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x127574 | `eax` | `ret(_oobSendMsg)` |

## `_oobSendFFSCCmd`

- `_oobSendFFSCCmd`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x127354 to 0x127474.
  Established. The byte stores at `sp -0x6b0` and `sp -0x6af` write `0xa0` (`mov` at 0x127366) and
  `0x0` (`mov` at 0x12736d). A byte store through `ret(_strchr) +0x0` writes `0x0` (`mov` at
  0x1273b9). The four-byte store at `sp -0x6b4` retains `ret(_strlen)+1` (`mov` at 0x1273ec). The
  `_oobSendMsg` site has leading candidates `[[[sp+4]+108]]` and `sp-1712` (`call` at 0x12740e).
  Message roles remain unassigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x127360 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x127363 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x127366 | store | 1 | `sp -0x6b0` | `0xa0` |
| `mov` at 0x12736d | store | 1 | `sp -0x6af` | `0x0` |
| `mov` at 0x127385 | load | 1 | `[sp+4] +0x92` | `-` |
| `mov` at 0x1273b1 | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x1273b9 | store | 1 | `ret(_strchr) +0x0` | `0x0` |
| `mov` at 0x1273d9 | store | 1 | `sp -0x6ae` | `?` |
| `mov` at 0x1273df | store | 1 | `sp -0x6ad` | `?` |
| `mov` at 0x1273ec | store | 4 | `sp -0x6b4` | `ret(_strlen)+1` |
| `mov` at 0x1273fa | load | 4 | `sp -0x6b4` | `-` |
| `mov` at 0x127400 | store | 1 | `([sp-1716] + sp-4) -0x6a8` | `?` |
| `mov` at 0x127408 | load | 4 | `[sp+4] +0x6c` | `-` |
| `mov` at 0x12740b | load | 4 | `[[sp+4]+108] +0x0` | `-` |
| `test` at 0x12741a | load | 1 | `abs 0x1da698` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12737d | `_cmSendFFSCCmdNoPB 0x11a470` | `[sp+4]` | `[sp+8]` | `sp-1708` | `ebx` | `esi` | `edi` |
| `call` at 0x127390 | `_strchr 0x17de58` | `sp-1708` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1273a9 | `_strcpy 0x17dfdc` | `sp-248` | `ret(_strchr)+1` | `?` | `?` | `?` | `?` |
| `call` at 0x1273bd | `_strlen 0x172f14` | `sp-1708` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1273f2 | `_oobChecksumMsg 0x1270b4` | `sp-1712` | `?` | `?` | `?` | `ret(_strlen)+1` | `?` |
| `call` at 0x12740e | `_oobSendMsg 0x1275c4` | `[[[sp+4]+108]]` | `sp-1712` | `?` | `?` | `?` | `?` |
| `call` at 0x12742f | `_ffscMsg 0x11ebd0` | `0x12732d` | `sp-248` | `?` | `?` | `?` | `?` |
| `call` at 0x12743f | `_crProcessCmd 0x11bc80` | `[sp+4]` | `sp-248` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12739c | `(ret(_strchr) & ret(_strchr)) == 0x0` | `(ret(_strchr) & ret(_strchr)) != 0x0` |
| `je` at 0x1273b7 | `[[sp+4]+52] == 0x3` | `[[sp+4]+52] != 0x3` |
| `jge` at 0x1273ce | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x127418 | `(ret(_strchr) & ret(_strchr)) == 0x0` | `(ret(_strchr) & ret(_strchr)) != 0x0` |
| `je` at 0x127421 | `(([0x1da698]:1 & 0xff) & 0x4) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x127450 | `eax` | `?` |

## `_oobSendMsg`

- `_oobSendMsg`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1275c4 to 0x127658.
  Established. The byte loads read through `[sp+8] +0x2` (`mov` at 0x1275cd) and `[sp+8] +0x3`
  (`mov` at 0x1275d5). The `_timeoutWrite` site has candidate zero `[sp+4]`, unknown candidates one
  and two, and candidate three `[0x1da188]` (`call` at 0x1275fb). The later branch is taken when
  `[ret(___errno)] != 0x4` (`jne` at 0x127611). No message length or timing unit is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1275ca | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1275cd | load | 1 | `[sp+8] +0x2` | `-` |
| `mov` at 0x1275d5 | load | 1 | `[sp+8] +0x3` | `-` |
| `mov` at 0x1275ec | load | 4 | `abs 0x1da188` | `-` |
| `mov` at 0x1275f7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12760c | load | 4 | `ret(___errno) +0x0` | `-` |
| `mov` at 0x127624 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1275fb | `_timeoutWrite 0x12a788` | `[sp+4]` | `?` | `?` | `[0x1da188]` | `?` | `?` |
| `call` at 0x127607 | `___errno 0x16fa78` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127618 | `_ffscMsg 0x11ebd0` | `0x127575` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12762d | `_ffscMsgS 0x11eca0` | `0x127593` | `[sp+4]` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x1275e9 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jge` at 0x127605 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x127611 | `[ret(___errno)] != 0x4` | `[ret(___errno)] == 0x4` |
| `jg` at 0x12764a | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x127657 | `eax` | `?` |

## `_cmdRACKID`

- `_cmdRACKID`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x113528 to 0x1136e8.
  Established. The signed branch is taken when `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` (`jg` at
  0x1135b0). The `_strtoul` site presents `ret(_strtok_r)`, `sp-48`, and `0x0` as its first three
  candidates (`call` at 0x11362f). The later comparison branch is taken when `ret(_strtoul) <= 0x9
  (unsigned, 32-bit)` (`jbe` at 0x113647). The `_addrdRequestRackID` call has candidate zero `sp-32`
  (`call` at 0x11366c). No authority level or valid rack-ID range is assigned from these
  comparisons.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x113531 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x113534 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x113537 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x113550 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x113556 | load | 1 | `([0x1ee564] + [sp+12]) +0x4` | `-` |
| `mov` at 0x113570 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x11357b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113582 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11358b | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x113590 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113597 | store | 4 | `(unknown)` | `0xb` |
| `mov` at 0x1135a8 | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x1135f8 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1135fd | load | 1 | `([0x1ee564] + [sp+12]) +0x4` | `-` |
| `rep movsd` at 0x113626 | store | 4 | `(unknown)` | `?` |
| `rep movsd` at 0x113626 | load | 4 | `abs 0x1ee558` | `-` |
| `mov` at 0x113637 | store | 4 | `sp -0x14` | `ret(_strtoul)` |
| `mov` at 0x11363a | load | 4 | `sp -0x30` | `-` |
| `mov` at 0x11363d | load | 1 | `[sp-48] +0x0` | `-` |
| `cmp` at 0x113643 | load | 4 | `sp -0x14` | `-` |
| `mov` at 0x11368a | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x11368f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113696 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1136a8 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1136b3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1136ba | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x113542 | `_strtok_r 0x17e358` | `0x0` | `0x1128b7` | `[sp+4]` | `ebx` | `esi` | `edi` |
| `call` at 0x113567 | `_ffscRackIDToString 0x12594c` | `[0x1ee564]` | `sp-44` | `?` | `?` | `?` | `?` |
| `call` at 0x113586 | `_sprintf 0x171680` | `?` | `0x11346d` | `[0x1ee564]` | `sp-44` | `?` | `?` |
| `call` at 0x1135cb | `_destNumRacks 0x11ef3c` | `[sp+12]` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x1135de | `_ffscMsg 0x11ebd0` | `0x113478` | `ret(_destNumRacks)` | `?` | `?` | `?` | `?` |
| `call` at 0x11362f | `_strtoul 0x17db9c` | `ret(_strtok_r)` | `sp-48` | `0x0` | `?` | `?` | `?` |
| `call` at 0x11364f | `_ffscMsg 0x11ebd0` | `0x1134b4` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x11366c | `_addrdRequestRackID 0x10dad0` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11367d | `_ffscMsg 0x11ebd0` | `0x1134c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11369a | `_sprintf 0x171680` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1136be | `_sprintf 0x171680` | `?` | `0x113512` | `[0x1ee564]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x11354e | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `je` at 0x11355c | `(([([0x1ee564] + [sp+12])+4]:1 & 0xff) & ([([0x1ee564] + [sp+12])+4]:1 & 0xff)) == 0x0` | `(([([0x1ee564] + [sp+12])+4]:1 & 0xff) & ([([0x1ee564] + [sp+12])+4]:1 & 0xff)) != 0x0` |
| `jg` at 0x1135b0 | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `jle` at 0x1135d6 | `ret(_destNumRacks) <= 0x1 (signed, 32-bit)` | `ret(_destNumRacks) > 0x1 (signed, 32-bit)` |
| `jne` at 0x113603 | `(([([0x1ee564] + [sp+12])+4]:1 & 0xff) & ([([0x1ee564] + [sp+12])+4]:1 & 0xff)) != 0x0` | `(([([0x1ee564] + [sp+12])+4]:1 & 0xff) & ([([0x1ee564] + [sp+12])+4]:1 & 0xff)) == 0x0` |
| `jne` at 0x113641 | `(([[sp-48]]:1 & 0xff) & ([[sp-48]]:1 & 0xff)) != 0x0` | `(([[sp-48]]:1 & 0xff) & ([[sp-48]]:1 & 0xff)) == 0x0` |
| `jbe` at 0x113647 | `ret(_strtoul) <= 0x9 (unsigned, 32-bit)` | `ret(_strtoul) > 0x9 (unsigned, 32-bit)` |
| `je` at 0x113676 | `(ret(_addrdRequestRackID) & ret(_addrdRequestRackID)) == 0x0` | `(ret(_addrdRequestRackID) & ret(_addrdRequestRackID)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1136ce | `eax` | `?` |
