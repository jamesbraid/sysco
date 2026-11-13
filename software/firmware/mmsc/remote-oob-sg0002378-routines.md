# MMSC remote links and out-of-band messages in patch SG0002378

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

[Fact sheets and full listings](disasm/remote-oob-sg0002378-facts.txt) include all tool sections.

## `_remInit`

- `_remInit`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x128190 to 0x128350.
  Established. The `_taskSpawn` call presents leading candidates `0x12814d`, `0x64`, `0x0`,
  `0x4e20`, `0x128350`, and `0x0` (`call` at 0x1281ba); its result branch is taken when
  `ret(_taskSpawn) == 0xffffffff` (`je` at 0x1281c5). The two exits expose `0x0` (`ret` at 0x1281cc)
  and `0xffffffff` (`ret` at 0x1281e2). The candidate slots retain unknown semantic roles.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1281ba | `_taskSpawn 0x171358` | `0x12814d` | `0x64` | `0x0` | `0x4e20` | `0x128350` | `0x0` |
| `call` at 0x1281d5 | `_ffscMsg 0x11f040` | `0x128155` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1281c5 | `ret(_taskSpawn) == 0xffffffff` | `ret(_taskSpawn) != 0xffffffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1281cc | `eax` | `0x0` |
| `ret` at 0x1281e2 | `eax` | `0xffffffff` |

## `_remAcceptConnection`

- `_remAcceptConnection`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x127f80 to 0x128190.
  Established. The first `_timeoutRead` call has candidates `[sp+4]`, `sp-44`, `0x4`, and
  `[0x1dc768]` (`call` at 0x127fbd). A separate later site has candidates `[sp-44]`, `[sp+12]`,
  `0x70`, and `[0x1dc75c]` (`call` at 0x12809d). The four-byte store to `[sp+12] +0x4` writes
  `0xffffffff` (`mov` at 0x128100).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x127f89 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x127f8c | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x127f8f | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x127fb0 | load | 4 | `abs 0x1dc768` | `-` |
| `mov` at 0x127fce | load | 4 | `ret(___errno) +0x0` | `-` |
| `mov` at 0x128010 | store | 4 | `sp -0x30` | `0x10` |
| `mov` at 0x12801f | load | 4 | `sp -0x2c` | `-` |
| `mov` at 0x128056 | load | 4 | `sp -0x24` | `-` |
| `mov` at 0x12806f | load | 4 | `sp -0x2c` | `-` |
| `mov` at 0x128084 | load | 4 | `ret(_identFindByIPAddr) +0xc` | `-` |
| `mov` at 0x128087 | store | 4 | `[sp+8] +0x0` | `[ret(_identFindByIPAddr)+12]` |
| `mov` at 0x128089 | load | 4 | `ret(_identFindByIPAddr) +0x10` | `-` |
| `mov` at 0x128090 | load | 4 | `abs 0x1dc75c` | `-` |
| `mov` at 0x128099 | load | 4 | `sp -0x2c` | `-` |
| `mov` at 0x1280ae | load | 4 | `ret(___errno) +0x0` | `-` |
| `mov` at 0x128100 | store | 4 | `[sp+12] +0x4` | `0xffffffff` |
| `cmp` at 0x128107 | load | 1 | `abs 0x1dccc0` | `-` |
| `mov` at 0x128114 | load | 4 | `sp -0x24` | `-` |
| `mov` at 0x128124 | load | 4 | `ret(_identFindByIPAddr) +0xc` | `-` |
| `mov` at 0x128130 | load | 4 | `sp -0x2c` | `-` |
| `mov` at 0x128140 | load | 4 | `sp -0x2c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x127fa9 | `_ffscMsg 0x11f040` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127fbd | `_timeoutRead 0x12add0` | `[sp+4]` | `sp-44` | `0x4` | `[0x1dc768]` | `ebx` | `esi` |
| `call` at 0x127fc9 | `___errno 0x1711d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127fda | `_ffscMsg 0x11f040` | `0x127e12` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127ff5 | `_ffscMsgS 0x11f110` | `0x127e3e` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128023 | `_getpeername 0x193208` | `[sp-44]` | `sp-40` | `sp-48` | `?` | `?` | `?` |
| `call` at 0x128034 | `_ffscMsgS 0x11f110` | `0x127e7e` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128044 | `_identFindByIPAddr 0x1228f8` | `sp-36` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12805a | `_inet_ntoa_b 0x188c90` | `[sp-36]` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x128067 | `_ffscMsg 0x11f040` | `0x127ea2` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128073 | `_close 0x189280` | `[sp-44]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12809d | `_timeoutRead 0x12add0` | `[sp-44]` | `[sp+12]` | `0x70` | `[0x1dc75c]` | `?` | `?` |
| `call` at 0x1280a9 | `___errno 0x1711d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1280c5 | `_ffscMsgS 0x11f110` | `0x127f02` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1280db | `_ffscMsg 0x11f040` | `?` | `ret(_timeoutRead)` | `?` | `?` | `?` | `?` |
| `call` at 0x1280f8 | `_bcopy 0x176bac` | `0x12bb24` | `[sp+12]` | `0x70` | `?` | `?` | `?` |
| `call` at 0x128118 | `_inet_ntoa_b 0x188c90` | `[sp-36]` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x128128 | `_ffscRackIDToString 0x125b08` | `[ret(_identFindByIPAddr)+12]` | `sp-60` | `?` | `?` | `?` | `?` |
| `call` at 0x12813b | `_ffscMsg 0x11f040` | `0x127f4e` | `sp-60` | `sp-24` | `[sp-44]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x127f94 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x127fa2 | `([sp+12] & [sp+12]) != 0x0` | `([sp+12] & [sp+12]) == 0x0` |
| `jge` at 0x127fc7 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x127fd3 | `[ret(___errno)] != 0x4` | `[ret(___errno)] == 0x4` |
| `je` at 0x128003 | `ret(_timeoutRead) == 0x4` | `ret(_timeoutRead) != 0x4` |
| `je` at 0x12802d | `(ret(_getpeername) & ret(_getpeername)) == 0x0` | `(ret(_getpeername) & ret(_getpeername)) != 0x0` |
| `jne` at 0x128050 | `(ret(_identFindByIPAddr) & ret(_identFindByIPAddr)) != 0x0` | `(ret(_identFindByIPAddr) & ret(_identFindByIPAddr)) == 0x0` |
| `je` at 0x128082 | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |
| `je` at 0x12808e | `(? & 0x1) == 0x0` | `(? & 0x1) != 0x0` |
| `jge` at 0x1280a7 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x1280b3 | `[ret(___errno)] != 0x4` | `[ret(___errno)] == 0x4` |
| `je` at 0x1280d3 | `ret(_timeoutRead) == 0x70` | `ret(_timeoutRead) != 0x70` |
| `jge` at 0x12810e | `([0x1dccc0]:1 & 0xff) >= 0x0 (signed, 8-bit)` | `([0x1dccc0]:1 & 0xff) < 0x0 (signed, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12814c | `eax` | `?` |

## `_remListener`

- `_remListener`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x128350 to 0x1286a0.
  Established. A two-byte store writes `0x2` to `sp -0x24` (`mov` at 0x128365). The `_socket` site
  has leading candidates `0x2`, `0x1`, and `0x0` (`call` at 0x128391). The `_bind` site presents
  `ret(_socket)`, `sp-36`, and `0x10` (`call` at 0x1283b7). The later `_write` call uses
  `[0x1dcf0c]`, `sp-44`, and `0x4` (`call` at 0x1284c8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x128365 | store | 2 | `sp -0x24` | `0x2` |
| `mov` at 0x12836b | load | 2 | `abs 0x1dc790` | `-` |
| `movzx` at 0x128376 | load | 1 | `abs 0x1dc791` | `-` |
| `mov` at 0x128380 | store | 2 | `sp -0x22` | `?` |
| `mov` at 0x128384 | store | 4 | `sp -0x20` | `0x0` |
| `mov` at 0x128399 | store | 4 | `sp -0x30` | `ret(_socket)` |
| `mov` at 0x1283b3 | load | 4 | `sp -0x30` | `-` |
| `mov` at 0x1283e0 | load | 4 | `abs 0x1dc794` | `-` |
| `mov` at 0x1283e6 | load | 4 | `sp -0x30` | `-` |
| `cmp` at 0x128410 | load | 1 | `abs 0x1dccc0` | `-` |
| `mov` at 0x128419 | load | 4 | `sp -0x30` | `-` |
| `mov` at 0x128440 | load | 4 | `abs 0x1f0f2c` | `-` |
| `mov` at 0x128470 | store | 4 | `sp -0x28` | `0x10` |
| `mov` at 0x12847f | load | 4 | `sp -0x30` | `-` |
| `mov` at 0x12848b | store | 4 | `sp -0x2c` | `ret(_accept)` |
| `mov` at 0x1284a5 | load | 4 | `abs 0x1dc758` | `-` |
| `mov` at 0x1284c2 | load | 4 | `abs 0x1dcf0c` | `-` |
| `cmp` at 0x1284ee | load | 4 | `sp -0x30` | `-` |
| `mov` at 0x128507 | load | 4 | `sp -0x34` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12835d | `_bzero 0x172678` | `sp-36` | `0x10` | `ebx` | `?` | `?` | `?` |
| `call` at 0x128391 | `_socket 0x192238` | `0x2` | `0x1` | `0x0` | `?` | `?` | `?` |
| `call` at 0x1283a5 | `_ffscMsgS 0x11f110` | `0x1281e3` | `?` | `ret(_socket)` | `?` | `?` | `?` |
| `call` at 0x1283b7 | `_bind 0x192308` | `ret(_socket)` | `sp-36` | `0x10` | `?` | `ret(_socket)` | `?` |
| `call` at 0x1283c8 | `_ffscMsgS 0x11f110` | `0x128202` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1283ea | `_listen 0x192398` | `[sp-48]` | `[0x1dc794]` | `?` | `?` | `?` | `?` |
| `call` at 0x1283fb | `_ffscMsgS 0x11f110` | `0x12822a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128422 | `_ffscMsg 0x11f040` | `0x128249` | `[sp-48]` | `?` | `?` | `?` | `?` |
| `call` at 0x12842a | `_timeoutInit 0x12aca0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128449 | `_setjmp 0x192088` | `[0x1f0f2c]+4` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12845e | `_ffscMsg 0x11f040` | `0x1282a0` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128466 | `_timeoutCancel 0x12aba0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128483 | `_accept 0x1923e8` | `[sp-48]` | `sp-20` | `sp-40` | `?` | `?` | `?` |
| `call` at 0x1284ab | `_timeoutSet 0x12aef0` | `[0x1dc758]` | `0x1282f5` | `?` | `?` | `ret(_accept)` | `?` |
| `call` at 0x1284b7 | `_timeoutEnable 0x12abe0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1284c8 | `_write 0x189320` | `[0x1dcf0c]` | `sp-44` | `0x4` | `?` | `?` | `?` |
| `call` at 0x1284d2 | `_timeoutDisable 0x12abc0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1284d7 | `_timeoutCancel 0x12aba0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1284e6 | `_ffscMsgS 0x11f110` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1284fd | `_ffscMsg 0x11f040` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x12839e | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jge` at 0x1283c1 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jge` at 0x1283f4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jge` at 0x128417 | `([0x1dccc0]:1 & 0xff) >= 0x0 (signed, 8-bit)` | `([0x1dccc0]:1 & 0xff) < 0x0 (signed, 8-bit)` |
| `je` at 0x128431 | `(ret(_timeoutInit) & ret(_timeoutInit)) == 0x0` | `(ret(_timeoutInit) & ret(_timeoutInit)) != 0x0` |
| `je` at 0x128453 | `(ret(_setjmp) & ret(_setjmp)) == 0x0` | `(ret(_setjmp) & ret(_setjmp)) != 0x0` |
| `jge` at 0x128490 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x1284b5 | `(ret(_timeoutSet) & ret(_timeoutSet)) != 0x0` | `(ret(_timeoutSet) & ret(_timeoutSet)) == 0x0` |
| `je` at 0x1284df | `ret(_write) == 0x4` | `ret(_write) != 0x4` |
| `jge` at 0x1284f2 | `[sp-48] >= 0x0 (signed, 32-bit)` | `[sp-48] < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12850d | `eax` | `0xffffffff` |

## `_remOpenConnection`

- `_remOpenConnection`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1286a0 to 0x128930.
  Established. The `_identFindByRackID` call forwards `[sp+4]` (`call` at 0x1286d4). The `_connect`
  site has leading candidates `[sp-48]`, `sp-20`, and `0x10` (`call` at 0x1287ca). A later
  `_timeoutWrite` site has candidates `[sp-48]`, `[sp+8]`, `0x70`, and `[0x1dc77c]` (`call` at
  0x12881a). The subsequent equality branch is taken when `ret(_timeoutWrite) == 0x70` (`je` at
  0x128853).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x1286a7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1286d0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1286dc | store | 4 | `sp -0x2c` | `ret(_identFindByRackID)` |
| `mov` at 0x1286e3 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12870e | store | 4 | `sp -0x30` | `ret(_socket)` |
| `mov` at 0x12873e | store | 2 | `sp -0x14` | `0x2` |
| `mov` at 0x128744 | load | 2 | `abs 0x1dc790` | `-` |
| `movzx` at 0x12874f | load | 1 | `abs 0x1dc791` | `-` |
| `mov` at 0x128759 | store | 2 | `sp -0x12` | `?` |
| `mov` at 0x12875d | load | 4 | `sp -0x2c` | `-` |
| `mov` at 0x128760 | load | 4 | `[sp-44] +0x8` | `-` |
| `mov` at 0x128763 | store | 4 | `sp -0x10` | `[[sp-44]+8]` |
| `mov` at 0x128766 | load | 4 | `abs 0x1f0f2c` | `-` |
| `mov` at 0x1287a5 | load | 4 | `abs 0x1dc78c` | `-` |
| `mov` at 0x1287c6 | load | 4 | `sp -0x30` | `-` |
| `mov` at 0x128800 | load | 4 | `sp -0x2c` | `-` |
| `mov` at 0x128803 | load | 4 | `[sp-44] +0x10` | `-` |
| `mov` at 0x12880a | load | 4 | `abs 0x1dc77c` | `-` |
| `mov` at 0x128812 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x128816 | load | 4 | `sp -0x30` | `-` |
| `mov` at 0x12882d | load | 4 | `ret(___errno) +0x0` | `-` |
| `mov` at 0x128863 | load | 4 | `sp -0x30` | `-` |
| `cmp` at 0x128880 | load | 1 | `abs 0x1dccc0` | `-` |
| `mov` at 0x12888d | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x128899 | load | 4 | `sp -0x30` | `-` |
| `mov` at 0x12889e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1288ac | load | 4 | `sp -0x30` | `-` |
| `mov` at 0x1288af | load | 4 | `sp -0x34` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1286b2 | `_ffscMsg 0x11f040` | `0x12850e` | `ebx` | `?` | `?` | `?` | `?` |
| `call` at 0x1286d4 | `_identFindByRackID 0x1229b8` | `[sp+4]` | `ebx` | `?` | `?` | `?` | `?` |
| `call` at 0x1286ec | `_ffscMsg 0x11f040` | `0x128540` | `[sp+4]` | `?` | `?` | `ret(_identFindByRackID)` | `?` |
| `call` at 0x128706 | `_socket 0x192238` | `0x2` | `0x1` | `0x0` | `?` | `?` | `ret(_identFindByRackID)` |
| `call` at 0x12871a | `_ffscMsgS 0x11f110` | `0x128568` | `?` | `ret(_socket)` | `?` | `?` | `?` |
| `call` at 0x128736 | `_bzero 0x172678` | `sp-20` | `0x10` | `?` | `ret(_socket)` | `?` | `?` |
| `call` at 0x12876f | `_setjmp 0x192088` | `[0x1f0f2c]+4` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128780 | `_ffscMsg 0x11f040` | `0x128593` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128788 | `_timeoutCancel 0x12aba0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1287ab | `_timeoutSet 0x12aef0` | `[0x1dc78c]` | `0x1285bf` | `?` | `?` | `?` | `?` |
| `call` at 0x1287bb | `_timeoutEnable 0x12abe0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1287ca | `_connect 0x192598` | `[sp-48]` | `sp-20` | `0x10` | `?` | `?` | `?` |
| `call` at 0x1287d4 | `_timeoutDisable 0x12abc0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1287d9 | `_timeoutCancel 0x12aba0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1287e7 | `_ffscMsgS 0x11f110` | `0x1285c7` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12881a | `_timeoutWrite 0x12b020` | `[sp-48]` | `[sp+8]` | `0x70` | `[0x1dc77c]` | `?` | `?` |
| `call` at 0x128828 | `___errno 0x1711d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128839 | `_ffscMsg 0x11f040` | `0x1285e8` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128845 | `_ffscMsgS 0x11f110` | `0x128613` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12885b | `_ffscMsg 0x11f040` | `0x12863a` | `ret(_timeoutWrite)` | `?` | `?` | `?` | `?` |
| `call` at 0x128867 | `_close 0x189280` | `[sp-48]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128891 | `_inet_ntoa_b 0x188c90` | `[sp-16]` | `sp-40` | `?` | `?` | `?` | `?` |
| `call` at 0x1288a7 | `_ffscMsg 0x11f040` | `0x12865e` | `[sp+4]` | `sp-40` | `[sp-48]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x1286ab | `[sp+4] != 0xffffffff` | `[sp+4] == 0xffffffff` |
| `jne` at 0x1286e1 | `(ret(_identFindByRackID) & ret(_identFindByRackID)) != 0x0` | `(ret(_identFindByRackID) & ret(_identFindByRackID)) == 0x0` |
| `jge` at 0x128713 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x128779 | `(ret(_setjmp) & ret(_setjmp)) == 0x0` | `(ret(_setjmp) & ret(_setjmp)) != 0x0` |
| `jne` at 0x1287b5 | `(ret(_timeoutSet) & ret(_timeoutSet)) != 0x0` | `(ret(_timeoutSet) & ret(_timeoutSet)) == 0x0` |
| `jge` at 0x1287e0 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x128808 | `(? & 0x1) == 0x0` | `(? & 0x1) != 0x0` |
| `jge` at 0x128826 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x128832 | `[ret(___errno)] != 0x4` | `[ret(___errno)] == 0x4` |
| `je` at 0x128853 | `ret(_timeoutWrite) == 0x70` | `ret(_timeoutWrite) != 0x70` |
| `jge` at 0x128887 | `([0x1dccc0]:1 & 0xff) >= 0x0 (signed, 8-bit)` | `([0x1dccc0]:1 & 0xff) < 0x0 (signed, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1288b5 | `eax` | `?` |

## `_remReceiveData`

- `_remReceiveData`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x128930 to 0x128ab0.
  Established. The `_timeoutRead` site has candidate zero `[sp+4]`, unknown candidates one and two,
  and candidate three `[0x1dc770]` (`call` at 0x128956). The error-value branch is taken when
  `[ret(___errno)] != 0x4` (`jne` at 0x12896c). Both of its paths join a `_close` call with
  candidate zero `[sp+4]` (`call` at 0x12898f).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x128936 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12893b | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x128948 | load | 4 | `abs 0x1dc770` | `-` |
| `mov` at 0x12894f | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x128967 | load | 4 | `ret(___errno) +0x0` | `-` |
| `mov` at 0x12899c | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1289a1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1289a8 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x1289c8 | load | 1 | `abs 0x1dccc0` | `-` |
| `mov` at 0x1289d2 | load | 4 | `sp +0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x128956 | `_timeoutRead 0x12add0` | `[sp+4]` | `?` | `?` | `[0x1dc770]` | `?` | `?` |
| `call` at 0x128962 | `___errno 0x1711d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128974 | `_ffscMsg 0x11f040` | `0x1288b6` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x128986 | `_ffscMsgS 0x11f110` | `0x1288d8` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x12898f | `_close 0x189280` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1289ac | `_sprintf 0x172dd8` | `?` | `0x1288f9` | `?` | `?` | `?` | `?` |
| `call` at 0x1289db | `_ffscMsg 0x11f040` | `0x128904` | `[sp+12]` | `[sp+4]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x128940 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jge` at 0x128960 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x12896c | `[ret(___errno)] != 0x4` | `[ret(___errno)] == 0x4` |
| `jg` at 0x1289c6 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `jge` at 0x1289cf | `([0x1dccc0]:1 & 0xff) >= 0x0 (signed, 8-bit)` | `([0x1dccc0]:1 & 0xff) < 0x0 (signed, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1289eb | `eax` | `?` |

## `_remSendData`

- `_remSendData`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x128ab0 to 0x128d00.
  Established. The four-byte store through `[[sp+4]+136] +0x1c` writes `0x1` (`mov` at 0x128afb).
  The `_msgWrite` site has leading candidates `[[sp+4]+136]` and `[0x1dc780]` (`call` at 0x128b0f).
  The `_timeoutWrite` site instead has candidate zero `[[[sp+4]+136]+20]` and candidate three
  `[0x1dc77c]` (`call` at 0x128b73). Candidates one and two remain unknown there.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x128ab6 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x128ab9 | load | 4 | `sp +0xc` | `-` |
| `cmp` at 0x128abc | load | 1 | `abs 0x1dccc0` | `-` |
| `mov` at 0x128ac5 | load | 4 | `[sp+4] +0x84` | `-` |
| `mov` at 0x128acb | load | 4 | `[[sp+4]+132] +0x14` | `-` |
| `mov` at 0x128ae3 | load | 4 | `[sp+4] +0x88` | `-` |
| `mov` at 0x128ae9 | load | 4 | `[[sp+4]+136] +0x8` | `-` |
| `mov` at 0x128af5 | load | 4 | `[sp+4] +0x88` | `-` |
| `mov` at 0x128afb | store | 4 | `[[sp+4]+136] +0x1c` | `0x1` |
| `mov` at 0x128b02 | load | 4 | `abs 0x1dc780` | `-` |
| `mov` at 0x128b08 | load | 4 | `[sp+4] +0x88` | `-` |
| `mov` at 0x128b1b | load | 4 | `[sp+4] +0x88` | `-` |
| `mov` at 0x128b21 | load | 4 | `[[sp+4]+136] +0x1c` | `-` |
| `mov` at 0x128b29 | load | 4 | `[sp+4] +0x88` | `-` |
| `mov` at 0x128b2f | load | 4 | `[[sp+4]+136] +0x1c` | `-` |
| `mov` at 0x128b33 | load | 4 | `[sp+4] +0x88` | `-` |
| `mov` at 0x128b5c | load | 4 | `abs 0x1dc77c` | `-` |
| `mov` at 0x128b63 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x128b69 | load | 4 | `[sp+4] +0x88` | `-` |
| `mov` at 0x128b6f | load | 4 | `[[sp+4]+136] +0x14` | `-` |
| `mov` at 0x128b84 | load | 4 | `ret(___errno) +0x0` | `-` |
| `mov` at 0x128b8b | load | 4 | `[sp+4] +0x88` | `-` |
| `mov` at 0x128b91 | load | 4 | `[[sp+4]+136] +0x14` | `-` |
| `mov` at 0x128bb0 | load | 4 | `[sp+4] +0x88` | `-` |
| `mov` at 0x128bb6 | load | 4 | `[[sp+4]+136] +0x14` | `-` |
| `mov` at 0x128bcc | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x128bd1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128bd8 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x128ad5 | `_ffscMsg 0x11f040` | `0x1289ec` | `[sp+12]` | `[[[sp+4]+132]+20]` | `ebx` | `esi` | `edi` |
| `call` at 0x128aed | `_sprintf 0x172dd8` | `[[[sp+4]+136]+8]` | `0x128a0e` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x128b0f | `_msgWrite 0x1269c8` | `[[sp+4]+136]` | `[0x1dc780]` | `?` | `?` | `?` | `?` |
| `call` at 0x128b3f | `_ffscMsg 0x11f040` | `0x128a18` | `[[sp+4]+136]` | `[[[sp+4]+136]+28]` | `?` | `?` | `?` |
| `call` at 0x128b73 | `_timeoutWrite 0x12b020` | `[[[sp+4]+136]+20]` | `?` | `?` | `[0x1dc77c]` | `?` | `?` |
| `call` at 0x128b7f | `___errno 0x1711d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128b9a | `_ffscMsg 0x11f040` | `0x128a51` | `[[[sp+4]+136]+20]` | `?` | `?` | `?` | `?` |
| `call` at 0x128bbf | `_ffscMsgS 0x11f110` | `0x128a71` | `[[[sp+4]+136]+20]` | `?` | `?` | `?` | `?` |
| `call` at 0x128bdc | `_sprintf 0x172dd8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x128ac3 | `([0x1dccc0]:1 & 0xff) >= 0x0 (signed, 8-bit)` | `([0x1dccc0]:1 & 0xff) < 0x0 (signed, 8-bit)` |
| `jne` at 0x128b19 | `(ret(_msgWrite) & ret(_msgWrite)) != 0x0` | `(ret(_msgWrite) & ret(_msgWrite)) == 0x0` |
| `je` at 0x128b27 | `[[[sp+4]+136]+28] == 0xb` | `[[[sp+4]+136]+28] != 0xb` |
| `jle` at 0x128b54 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jge` at 0x128b7d | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x128b89 | `[ret(___errno)] != 0x4` | `[ret(___errno)] == 0x4` |
| `jg` at 0x128bf6 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x128c07 | `eax` | `?` |

## `_remStartConnectCommand`

- `_remStartConnectCommand`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x128d00 to 0x128f40.
  Established. The `_remStartRemoteCommand` call presents `[sp+4]`, `[sp+8]`, `[sp+12]`, and
  `[sp+16]` as its first four candidates (`call` at 0x128d69). The `_msgRead` site has unknown
  candidate zero and candidate one `[0x1dc75c]` (`call` at 0x128dbd). Two alternative four-byte
  stores write `0xffffffff` through unknown effective addresses (`mov` at 0x128eae, `mov` at
  0x128ee3).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x128d09 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x128d0c | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x128d0f | load | 4 | `(unknown)` | `-` |
| `test` at 0x128d16 | load | 1 | `abs 0x1dccc0` | `-` |
| `mov` at 0x128d23 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x128d40 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128d4f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128d55 | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x128d5c | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x128d60 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x128d65 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x128d71 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128d77 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128d83 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128d8f | load | 4 | `(unknown)` | `-` |
| `test` at 0x128d9a | load | 1 | `abs 0x1dccc0` | `-` |
| `mov` at 0x128db0 | load | 4 | `abs 0x1dc75c` | `-` |
| `mov` at 0x128db6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128dc9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128dcf | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128dd7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128ddd | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128df0 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x128df9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128e09 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x128e16 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128e1c | load | 4 | `(unknown)` | `-` |
| `test` at 0x128e40 | load | 1 | `abs 0x1dccc0` | `-` |
| `mov` at 0x128e49 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128e4d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128e60 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128e66 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128e78 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128e7e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128ea5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128eab | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128eae | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x128ece | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128ed4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128edd | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128ee3 | store | 4 | `(unknown)` | `0xffffffff` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x128d29 | `_ffscRackIDToString 0x125b08` | `[0x1f0c04]` | `sp-16` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x128d38 | `_ffscMsg 0x11f040` | `0x128c08` | `[sp+8]` | `sp-16` | `?` | `?` | `?` |
| `call` at 0x128d47 | `_msgClear 0x126058` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128d69 | `_remStartRemoteCommand 0x1290b0` | `[sp+4]` | `[sp+8]` | `[sp+12]` | `[sp+16]` | `?` | `?` |
| `call` at 0x128da8 | `_ffscMsg 0x11f040` | `0x128c3a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128dbd | `_msgRead 0x126348` | `?` | `[0x1dc75c]` | `?` | `?` | `?` | `?` |
| `call` at 0x128dfd | `_sprintf 0x172dd8` | `?` | `0x128c7b` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x128e0d | `_ffscRackIDToString 0x125b08` | `[sp+12]` | `sp-28` | `?` | `?` | `?` | `?` |
| `call` at 0x128e26 | `_ffscMsg 0x11f040` | `0x128c83` | `sp-28` | `?` | `?` | `?` | `?` |
| `call` at 0x128e56 | `_ffscMsg 0x11f040` | `0x128cab` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128e6a | `_strchr 0x180208` | `?` | `0x3a` | `?` | `?` | `?` | `?` |
| `call` at 0x128e99 | `_strncmp 0x176d68` | `?` | `0x128ccc` | `0x5` | `?` | `?` | `?` |
| `call` at 0x128ec6 | `_ffscMsg 0x11f040` | `0x128cd2` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128ed8 | `_close 0x189280` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x128d1d | `(([0x1dccc0]:1 & 0xff) & 0x20) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x20) != 0x0` |
| `je` at 0x128d7d | `? == 0xb` | `? != 0xb` |
| `je` at 0x128d89 | `? == 0xa` | `? != 0xa` |
| `jl` at 0x128d94 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x128da1 | `(([0x1dccc0]:1 & 0xff) & 0x20) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x20) != 0x0` |
| `jne` at 0x128dc7 | `(ret(_msgRead) & ret(_msgRead)) != 0x0` | `(ret(_msgRead) & ret(_msgRead)) == 0x0` |
| `je` at 0x128dd5 | `? == 0xb` | `? != 0xb` |
| `jne` at 0x128de8 | `? != 0xa` | `? == 0xa` |
| `je` at 0x128e47 | `(([0x1dccc0]:1 & 0xff) & 0x20) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x20) != 0x0` |
| `jne` at 0x128e76 | `(ret(_strchr) & ret(_strchr)) != 0x0` | `(ret(_strchr) & ret(_strchr)) == 0x0` |
| `jne` at 0x128ea3 | `(ret(_strncmp) & ret(_strncmp)) != 0x0` | `(ret(_strncmp) & ret(_strncmp)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x128ef8 | `eax` | `?` |

## `_remStartDataRequest`

- `_remStartDataRequest`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x128f40 to 0x1290b0.
  Established. The `_remStartConnectCommand` site forwards candidates `[sp+4]`, `[sp+8]`, `[sp+12]`,
  and `[sp+16]` (`call` at 0x128f59). The `_sscanf` site has unknown candidate zero, candidate one
  `0x128f0a`, and candidate two `[sp+20]` (`call` at 0x128fbb); its result branch is taken when
  `ret(_sscanf) != 0x1` (`jne` at 0x128fc6), leading to the `_close` site with candidate zero
  `ret(_remStartConnectCommand)` (`call` at 0x128fef).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x128f46 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x128f49 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x128f4c | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x128f4f | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x128f52 | load | 4 | `sp +0x14` | `-` |
| `mov` at 0x128f67 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128f6e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128f74 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128f7d | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x128f82 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128f89 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128faa | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128fb1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128fb7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128fd0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128fd7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128fdd | load | 4 | `(unknown)` | `-` |
| `mov` at 0x128ffc | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x129001 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x129008 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x128f59 | `_remStartConnectCommand 0x128d00` | `[sp+4]` | `[sp+8]` | `[sp+12]` | `[sp+16]` | `ebx` | `esi` |
| `call` at 0x128f8d | `_sprintf 0x172dd8` | `?` | `0x128ef9` | `?` | `?` | `?` | `?` |
| `call` at 0x128fbb | `_sscanf 0x173b88` | `?` | `0x128f0a` | `[sp+20]` | `?` | `?` | `?` |
| `call` at 0x128fe6 | `_ffscMsg 0x11f040` | `0x128f13` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x128fef | `_close 0x189280` | `ret(_remStartConnectCommand)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12900c | `_sprintf 0x172dd8` | `?` | `0x128a43` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x128f65 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x128fa2 | `([sp+20] & [sp+20]) == 0x0` | `([sp+20] & [sp+20]) != 0x0` |
| `jne` at 0x128fc6 | `ret(_sscanf) != 0x1` | `ret(_sscanf) == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12901f | `eax` | `?` |

## `_remStartRemoteCommand`

- `_remStartRemoteCommand`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1290b0 to 0x12929c.
  Established. The `_userExtractUserInfo` call has leading candidates `[sp+4]` and `sp-120` (`call`
  at 0x1290cb). The following `_remOpenConnection` site presents `[sp+12]` and `sp-120` (`call` at
  0x1290d5). Two four-byte stores have unknown targets and value `ret(_remOpenConnection)` (`mov` at
  0x129116, `mov` at 0x12911f). The later `_msgWrite` call has candidate one `[0x1dc77c]` and
  unknown candidate zero (`call` at 0x1291d4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1290b9 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1290bc | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1290bf | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1290e9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1290ef | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1290f8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1290fe | store | 4 | `(unknown)` | `0xb` |
| `mov` at 0x129110 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x129116 | store | 4 | `(unknown)` | `ret(_remOpenConnection)` |
| `mov` at 0x129119 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12911f | store | 4 | `(unknown)` | `ret(_remOpenConnection)` |
| `mov` at 0x129122 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x129125 | load | 1 | `([sp+12] + [sp+16]) +0x4` | `-` |
| `mov` at 0x129144 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x129147 | load | 1 | `([sp+12] + [sp+16]) +0x4` | `-` |
| `mov` at 0x129155 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x12915b | store | 1 | `(unknown)` | `?` |
| `mov` at 0x129166 | store | 1 | `(unknown)` | `0x0` |
| `mov` at 0x12916b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12917a | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x129187 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12918d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x129199 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12919f | store | 4 | `(unknown)` | `0x1` |
| `cmp` at 0x1291a6 | load | 1 | `abs 0x1dccc0` | `-` |
| `mov` at 0x1291b0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1291b6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1291c7 | load | 4 | `abs 0x1dc77c` | `-` |
| `mov` at 0x1291cd | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1291e6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1291ec | load | 4 | `(unknown)` | `-` |
| `mov` at 0x129200 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x129206 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12920e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x129214 | store | 4 | `(unknown)` | `0x3` |
| `mov` at 0x129220 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x129238 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12923e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12924a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x129250 | store | 4 | `(unknown)` | `0xb` |
| `mov` at 0x129257 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12925d | store | 4 | `(unknown)` | `0xb` |
| `mov` at 0x129264 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12926a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x129273 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x129279 | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x129280 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x129286 | store | 4 | `(unknown)` | `0xffffffff` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1290cb | `_userExtractUserInfo 0x12bd84` | `[sp+4]` | `sp-120` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x1290d5 | `_remOpenConnection 0x1286a0` | `[sp+12]` | `sp-120` | `?` | `?` | `?` | `?` |
| `call` at 0x1290f3 | `_sprintf 0x172dd8` | `?` | `0x129020` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x129136 | `_strcpy 0x18038c` | `sp-8` | `0x129033` | `?` | `?` | `?` | `?` |
| `call` at 0x129172 | `_msgClear 0x126058` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x129191 | `_sprintf 0x172dd8` | `?` | `0x129035` | `sp-8` | `[sp+8]` | `?` | `?` |
| `call` at 0x1291bf | `_ffscMsg 0x11f040` | `0x129041` | `?` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x1291d4 | `_msgWrite 0x1269c8` | `?` | `[0x1dc77c]` | `?` | `?` | `?` | `?` |
| `call` at 0x1291f0 | `_sprintf 0x172dd8` | `?` | `0x129068` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x12922a | `_ffscMsg 0x11f040` | `0x129078` | `[sp+12]` | `?` | `?` | `?` | `?` |
| `call` at 0x129242 | `_sprintf 0x172dd8` | `?` | `0x129068` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x12926e | `_close 0x189280` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x1290e1 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jge` at 0x12912b | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jae` at 0x129153 | `C == 0` | `not (C == 0)` |
| `jbe` at 0x129164 | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |
| `jge` at 0x1291ad | `([0x1dccc0]:1 & 0xff) >= 0x0 (signed, 8-bit)` | `([0x1dccc0]:1 & 0xff) < 0x0 (signed, 8-bit)` |
| `je` at 0x1291de | `(ret(_msgWrite) & ret(_msgWrite)) == 0x0` | `(ret(_msgWrite) & ret(_msgWrite)) != 0x0` |
| `jne` at 0x12920c | `? != 0xb` | `? == 0xb` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x129299 | `eax` | `?` |

## `_oobChecksumMsg`

- `_oobChecksumMsg`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12783c to 0x12794c.
  Established. Byte loads read `[sp+4] +0x2` (`mov` at 0x127844) and `[sp+4] +0x3` (`mov` at
  0x12784c). The loop byte load has unknown effective address (`mov` at 0x127868); its repeat branch
  is taken when `? < ? (signed, 32-bit)` (`jl` at 0x127875), leaving both compared values unknown.
  The final return value is also unresolved (`ret` at 0x127882).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x127841 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x127844 | load | 1 | `[sp+4] +0x2` | `-` |
| `mov` at 0x12784c | load | 1 | `[sp+4] +0x3` | `-` |
| `mov` at 0x127868 | load | 1 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x127865 | `0x1 >= ? (signed, 32-bit)` | `0x1 < ? (signed, 32-bit)` |
| `jl` at 0x127875 | `? < ? (signed, 32-bit)` | `? >= ? (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x127882 | `eax` | `?` |

## `_oobProcessMsg`

- `_oobProcessMsg`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12794c to 0x127adc.
  Established. The prefix branch is taken when `([[sp+8]]:1 & 0xff) == 0xa0` (`je` at 0x127970). The
  `_oobSendGraphicsCmd` site has leading candidates `[sp+4]` and `[sp+8]` (`call` at 0x127a5e). A
  separate `_oobSendFFSCCmd` site uses `[sp+4]` and `[sp+8]+4` (`call` at 0x127a7b). The selector
  values remain unknown in the branch predicates below.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x127951 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x127954 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12796c | load | 1 | `[sp+8] +0x0` | `-` |
| `mov` at 0x127972 | load | 1 | `[sp+8] +0x0` | `-` |
| `mov` at 0x127997 | load | 1 | `[sp+8] +0x2` | `-` |
| `mov` at 0x12799f | load | 1 | `[sp+8] +0x3` | `-` |
| `mov` at 0x1279af | load | 1 | `(unknown)` | `-` |
| `mov` at 0x1279ba | load | 1 | `[sp+8] +0x2` | `-` |
| `mov` at 0x1279c2 | load | 1 | `[sp+8] +0x3` | `-` |
| `mov` at 0x1279d2 | load | 1 | `(unknown)` | `-` |
| `test` at 0x1279fc | load | 1 | `abs 0x1dccc1` | `-` |
| `mov` at 0x127a05 | load | 1 | `[sp+8] +0x1` | `-` |
| `mov` at 0x127a1c | load | 1 | `[sp+8] +0x1` | `-` |
| `mov` at 0x127a6c | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x127aa1 | load | 4 | `[sp+4] +0x6c` | `-` |
| `mov` at 0x127aa4 | load | 4 | `[[sp+4]+108] +0x0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x127961 | `_ffscMsg 0x11f040` | `0x127883` | `[sp+4]` | `ebx` | `esi` | `ebp` | `?` |
| `call` at 0x127980 | `_ffscMsg 0x11f040` | `0x1278ad` | `[sp+4]` | `?` | `ebx` | `esi` | `ebp` |
| `call` at 0x12798d | `_oobChecksumMsg 0x12783c` | `[sp+8]` | `ebx` | `esi` | `ebp` | `?` | `?` |
| `call` at 0x1279e1 | `_ffscMsg 0x11f040` | `0x1278e7` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x127a14 | `_ffscMsg 0x11f040` | `0x127926` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x127a5e | `_oobSendGraphicsCmd 0x127bfc` | `[sp+4]` | `[sp+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x127a7b | `_oobSendFFSCCmd 0x127adc` | `[sp+4]` | `[sp+8]+4` | `?` | `?` | `?` | `?` |
| `call` at 0x127aa7 | `_oobSendMsg 0x127d4c` | `[[[sp+4]+108]]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x127959 | `([sp+8] & [sp+8]) != 0x0` | `([sp+8] & [sp+8]) == 0x0` |
| `je` at 0x127970 | `([[sp+8]]:1 & 0xff) == 0xa0` | `([[sp+8]]:1 & 0xff) != 0xa0` |
| `je` at 0x1279b4 | `? == ?` | `? != ?` |
| `je` at 0x127a03 | `(([0x1dccc1]:1 & 0xff) & 0x8) == 0x0` | `(([0x1dccc1]:1 & 0xff) & 0x8) != 0x0` |
| `jg` at 0x127a27 | `? > 0xb (signed, 32-bit)` | `? <= 0xb (signed, 32-bit)` |
| `jge` at 0x127a2c | `? >= 0x1 (signed, 32-bit)` | `? < 0x1 (signed, 32-bit)` |
| `je` at 0x127a30 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x127a3f | `? == 0x3d` | `? != 0x3d` |
| `je` at 0x127a44 | `? == 0x3e` | `? != 0x3e` |
| `jne` at 0x127a74 | `([[sp+4]+60] & 0x40000) != 0x0` | `([[sp+4]+60] & 0x40000) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x127ab4 | `eax` | `?` |

## `_oobSendGraphicsCmd`

- `_oobSendGraphicsCmd`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x127bfc to 0x127d4c.
  Established. The branch falls through when `(ret(_strncmp) & ret(_strncmp)) != 0x0` (`je` at
  0x127c6f). That path writes byte `0xa0` to `sp -0x848` (`mov` at 0x127c85) and byte `0x5` to
  `sp -0x847` (`mov` at 0x127c8c). The
  `_oobChecksumMsg` site uses candidate zero `sp-2120` (`call` at 0x127ccf). The `_oobSendMsg` site
  has leading candidates `[[[sp+4]+108]]` and `sp-2120` (`call` at 0x127ceb).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x127c08 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x127c0b | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x127c3b | load | 1 | `[sp+4] +0x93` | `-` |
| `mov` at 0x127c85 | store | 1 | `sp -0x848` | `0xa0` |
| `mov` at 0x127c8c | store | 1 | `sp -0x847` | `0x5` |
| `mov` at 0x127ca2 | store | 1 | `sp -0x846` | `?` |
| `mov` at 0x127caa | store | 1 | `sp -0x845` | `?` |
| `mov` at 0x127cc6 | store | 1 | `(ret(_strlen)+1 + sp-4) -0x841` | `0x0` |
| `mov` at 0x127cd7 | store | 1 | `(ret(_strlen)+1 + sp-4) -0x840` | `?` |
| `mov` at 0x127ce5 | load | 4 | `[sp+4] +0x6c` | `-` |
| `mov` at 0x127ce8 | load | 4 | `[[sp+4]+108] +0x0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x127c22 | `_sprintf 0x172dd8` | `sp-248` | `0x127bd9` | `[sp+8]` | `sp-2120` | `ebx` | `esi` |
| `call` at 0x127c33 | `_cmSendFFSCCmd 0x11a6e0` | `[sp+4]` | `sp-248` | `sp-332` | `?` | `?` | `?` |
| `call` at 0x127c46 | `_strchr 0x180208` | `sp-332` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127c65 | `_strncmp 0x176d68` | `?` | `0x127bf7` | `0x2` | `?` | `?` | `?` |
| `call` at 0x127c78 | `_strlen 0x17466c` | `sp-332` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127cbe | `_strcpy 0x18038c` | `sp-2116` | `sp-332` | `?` | `?` | `?` | `?` |
| `call` at 0x127ccf | `_oobChecksumMsg 0x12783c` | `sp-2120` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127ceb | `_oobSendMsg 0x127d4c` | `[[[sp+4]+108]]` | `sp-2120` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x127c50 | `(ret(_strchr) & ret(_strchr)) != 0x0` | `(ret(_strchr) & ret(_strchr)) == 0x0` |
| `je` at 0x127c6f | `(ret(_strncmp) & ret(_strncmp)) == 0x0` | `(ret(_strncmp) & ret(_strncmp)) != 0x0` |
| `jge` at 0x127c97 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x127cfc | `eax` | `ret(_oobSendMsg)` |

## `_oobSendFFSCCmd`

- `_oobSendFFSCCmd`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x127adc to 0x127bfc.
  Established. The initial byte stores write `0xa0` to `sp -0x7f4` (`mov` at 0x127aee) and zero to
  `sp -0x7f3` (`mov` at 0x127af5). The `_cmSendFFSCCmdNoPB` call presents leading candidates `[sp+4]`, `[sp+8]`, and
  `sp-2032` (`call` at 0x127b05). The `_oobSendMsg` site has candidate one `sp-2036` (`call` at
  0x127b96). The raw suffix from 0x127bf9 to 0x127bfc contains `00 90 90`, beyond the retained decoder
  output. It follows the unreachable bytes decoded through 0x127bf8.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x127ae8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x127aeb | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x127aee | store | 1 | `sp -0x7f4` | `0xa0` |
| `mov` at 0x127af5 | store | 1 | `sp -0x7f3` | `0x0` |
| `mov` at 0x127b0d | load | 1 | `[sp+4] +0x92` | `-` |
| `mov` at 0x127b39 | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x127b41 | store | 1 | `ret(_strchr) +0x0` | `0x0` |
| `mov` at 0x127b61 | store | 1 | `sp -0x7f2` | `?` |
| `mov` at 0x127b67 | store | 1 | `sp -0x7f1` | `?` |
| `mov` at 0x127b74 | store | 4 | `sp -0x7f8` | `ret(_strlen)+1` |
| `mov` at 0x127b82 | load | 4 | `sp -0x7f8` | `-` |
| `mov` at 0x127b88 | store | 1 | `([sp-2040] + sp-4) -0x7ec` | `?` |
| `mov` at 0x127b90 | load | 4 | `[sp+4] +0x6c` | `-` |
| `mov` at 0x127b93 | load | 4 | `[[sp+4]+108] +0x0` | `-` |
| `test` at 0x127ba2 | load | 1 | `abs 0x1dccc0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x127b05 | `_cmSendFFSCCmdNoPB 0x11a8e0` | `[sp+4]` | `[sp+8]` | `sp-2032` | `ebx` | `esi` | `edi` |
| `call` at 0x127b18 | `_strchr 0x180208` | `sp-2032` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127b31 | `_strcpy 0x18038c` | `sp-248` | `ret(_strchr)+1` | `?` | `?` | `?` | `?` |
| `call` at 0x127b45 | `_strlen 0x17466c` | `sp-2032` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127b7a | `_oobChecksumMsg 0x12783c` | `sp-2036` | `?` | `?` | `?` | `ret(_strlen)+1` | `?` |
| `call` at 0x127b96 | `_oobSendMsg 0x127d4c` | `[[[sp+4]+108]]` | `sp-2036` | `?` | `?` | `?` | `?` |
| `call` at 0x127bb7 | `_ffscMsg 0x11f040` | `0x127ab5` | `sp-248` | `?` | `?` | `?` | `?` |
| `call` at 0x127bc7 | `_crProcessCmd 0x11c0f0` | `[sp+4]` | `sp-248` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x127b24 | `(ret(_strchr) & ret(_strchr)) == 0x0` | `(ret(_strchr) & ret(_strchr)) != 0x0` |
| `je` at 0x127b3f | `[[sp+4]+52] == 0x3` | `[[sp+4]+52] != 0x3` |
| `jge` at 0x127b56 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x127ba0 | `(ret(_strchr) & ret(_strchr)) == 0x0` | `(ret(_strchr) & ret(_strchr)) != 0x0` |
| `je` at 0x127ba9 | `(([0x1dccc0]:1 & 0xff) & 0x4) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x127bd8 | `eax` | `?` |

## `_oobSendMsg`

- `_oobSendMsg`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x127d4c to 0x127de0.
  Established. The byte loads through `[sp+8] +0x2` (`mov` at 0x127d55) and `[sp+8] +0x3` (`mov` at
  0x127d5d) precede the `_timeoutWrite` site. That call has candidate zero `[sp+4]`, unknown
  candidates one and two, and candidate three `[0x1dc7b0]` (`call` at 0x127d83). The error-value
  branch is taken when `[ret(___errno)] != 0x4` (`jne` at 0x127d99).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x127d52 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x127d55 | load | 1 | `[sp+8] +0x2` | `-` |
| `mov` at 0x127d5d | load | 1 | `[sp+8] +0x3` | `-` |
| `mov` at 0x127d74 | load | 4 | `abs 0x1dc7b0` | `-` |
| `mov` at 0x127d7f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x127d94 | load | 4 | `ret(___errno) +0x0` | `-` |
| `mov` at 0x127dac | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x127d83 | `_timeoutWrite 0x12b020` | `[sp+4]` | `?` | `?` | `[0x1dc7b0]` | `?` | `?` |
| `call` at 0x127d8f | `___errno 0x1711d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127da0 | `_ffscMsg 0x11f040` | `0x127cfd` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x127db5 | `_ffscMsgS 0x11f110` | `0x127d1b` | `[sp+4]` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x127d71 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jge` at 0x127d8d | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x127d99 | `[ret(___errno)] != 0x4` | `[ret(___errno)] == 0x4` |
| `jg` at 0x127dd2 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x127ddf | `eax` | `?` |

## `_identInit`

- `_identInit`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122ba8 to 0x122d28.
  Established. The four-byte stores clear absolute `0x1edd6c` (`mov` at 0x122bba) and absolute
  `0x1f113c` (`mov` at 0x122bc4). The two `_nvramRead` sites present leading candidates `0x1`,
  `0x1dd200` (`call` at 0x122c24) and `0x2`, `0x1f0c04` (`call` at 0x122cc4). The `_identUpdate`
  call has candidate zero `0x1f0bf8` (`call` at 0x122cec). These global record meanings remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x122bba | store | 4 | `abs 0x1edd6c` | `0x0` |
| `mov` at 0x122bc4 | store | 4 | `abs 0x1f113c` | `0x0` |
| `mov` at 0x122bce | store | 1 | `abs 0x1f0bf8` | `0x0` |
| `mov` at 0x122bd5 | store | 1 | `abs 0x1f0bf9` | `0x0` |
| `mov` at 0x122be0 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x122be6 | store | 1 | `(unknown)` | `?` |
| `mov` at 0x122c00 | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x122c13 | store | 4 | `abs 0x1f0c08` | `0x1` |
| `test` at 0x122c30 | load | 1 | `abs 0x1dccc1` | `-` |
| `movzx` at 0x122c46 | load | 1 | `abs 0x1f0bfd` | `-` |
| `movzx` at 0x122c50 | load | 1 | `abs 0x1f0bfe` | `-` |
| `movzx` at 0x122c5c | load | 1 | `abs 0x1f0bff` | `-` |
| `mov` at 0x122c65 | store | 4 | `abs 0x1dd200` | `?` |
| `mov` at 0x122c77 | store | 4 | `abs 0x1dd200` | `0xf` |
| `mov` at 0x122c81 | store | 4 | `abs 0x1dd204` | `0x64` |
| `mov` at 0x122c8b | store | 4 | `abs 0x1dd208` | `0xff000000` |
| `mov` at 0x122ca9 | load | 4 | `abs 0x1dd200` | `-` |
| `mov` at 0x122caf | load | 4 | `abs 0x1dd204` | `-` |
| `mov` at 0x122cd0 | store | 4 | `abs 0x1f0c04` | `0xffffffff` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122bb2 | `_bzero 0x172678` | `0x1f0bf8` | `0x1c` | `ebp` | `?` | `?` | `?` |
| `call` at 0x122c24 | `_nvramRead 0x127108` | `0x1` | `0x1dd200` | `?` | `?` | `?` | `?` |
| `call` at 0x122c3e | `_ffscMsg 0x11f040` | `0x122b79` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122c9c | `_nvramWrite 0x127298` | `0x1` | `0x1dd200` | `?` | `?` | `?` | `?` |
| `call` at 0x122cb5 | `_inet_makeaddr_b 0x1889d0` | `[0x1dd204]` | `[0x1dd200]` | `0x1f0c00` | `?` | `?` | `?` |
| `call` at 0x122cc4 | `_nvramRead 0x127108` | `0x2` | `0x1f0c04` | `?` | `?` | `?` | `?` |
| `call` at 0x122cdf | `_lstInit 0x190178` | `0x1d4d68` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122cec | `_identUpdate 0x123078` | `0x1f0bf8` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x122bf0 | `? <= 0x5 (signed, 32-bit)` | `? > 0x5 (signed, 32-bit)` |
| `jle` at 0x122c11 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `je` at 0x122c2e | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` |
| `je` at 0x122c37 | `(([0x1dccc1]:1 & 0xff) & 0x1) == 0x0` | `(([0x1dccc1]:1 & 0xff) & 0x1) != 0x0` |
| `jbe` at 0x122c75 | `? <= 0xffffee (unsigned, 32-bit)` | `? > 0xffffee (unsigned, 32-bit)` |
| `je` at 0x122cce | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` |
| `jl` at 0x122cf3 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122cfa | `eax` | `0x0` |
| `ret` at 0x122d10 | `eax` | `0xffffffff` |
