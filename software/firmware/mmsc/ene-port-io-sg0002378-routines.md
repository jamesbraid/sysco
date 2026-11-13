# MMSC ene port I/O in patch SG0002378

Read from `patches/SG0002378/usr/cpu/firmware/mmscfw.bin` (patch SG0002378).
Image SHA-256: `b1249e58cf4f00c252362da8ed0515e25bb94aaf096bb60a5c5082c9b6e4d342`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans end at the next text symbol, including object markers. These inferred
boundaries can include literals or padding after returns. Statements below concern sites
reached by known control-flow edges.
No unresolved jump transfers were reported in these spans.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/ene-port-io-sg0002378-facts.txt) include all tool sections.

## `_eneattach`

- `_eneattach`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x182850 to 0x1829f0.
  Established. The `_malloc` site has candidate zero `0x6d8` (`call` at 0x182893). The `_intConnect`
  call presents unknown candidate zero, candidate one `0x182a40`, and candidate two `[sp+4]` (`call`
  at 0x182986). A byte store writes `0x4` through an unknown effective address (`mov` at 0x182953).
  The referenced record identities remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x182854 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x182880 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x182883 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x18289d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1828a0 | store | 4 | `(unknown)` | `ret(_malloc)` |
| `mov` at 0x1828a7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1828aa | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1828c5 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1828c8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1828d8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1828db | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1828e2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182900 | load | 2 | `sp +0x8` | `-` |
| `mov` at 0x182904 | store | 2 | `(unknown)` | `[sp+8]:2` |
| `cmp` at 0x182908 | load | 4 | `sp +0x10` | `-` |
| `cmp` at 0x18290e | load | 4 | `sp +0x10` | `-` |
| `cmp` at 0x182914 | load | 4 | `sp +0x10` | `-` |
| `cmp` at 0x18291a | load | 4 | `sp +0x10` | `-` |
| `cmp` at 0x182920 | load | 4 | `sp +0x10` | `-` |
| `cmp` at 0x182926 | load | 4 | `sp +0x10` | `-` |
| `cmp` at 0x18292c | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x182940 | store | 4 | `sp +0x10` | `0x3` |
| `mov` at 0x182947 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x18294a | store | 4 | `(unknown)` | `?` |
| `mov` at 0x18294d | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x182950 | store | 4 | `(unknown)` | `[sp+12]` |
| `mov` at 0x182953 | store | 1 | `(unknown)` | `0x4` |
| `mov` at 0x18295a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x182966 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x182972 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x18297b | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x18298e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x18299a | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x1829bf | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1829cc | store | 4 | `(unknown)` | `0x1` |
| `mov` at 0x1829e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x18285f | `_errnoSet 0x171240` | `0xd0005` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x182893 | `_malloc 0x17ec28` | `0x6d8` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x1828d0 | `_bzero 0x172678` | `?` | `0x6d8` | `?` | `?` | `?` | `?` |
| `call` at 0x18295e | `_eneBoardReset 0x183410` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x18296a | `_eneInit 0x183660` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x182986 | `_intConnect 0x176398` | `?` | `0x182a40` | `[sp+4]` | `?` | `?` | `?` |
| `call` at 0x182992 | `_eneIntEnable 0x183dd0` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x18299e | `_sysIntEnablePIC 0x108428` | `[sp+16]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1829c4 | `_ether_attach 0x1980c8` | `?` | `[sp+4]` | `0x182840` | `0x182a10` | `0x183540` | `0x1831b0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x182858 | `[sp+4] <= 0x3 (unsigned, 32-bit)` | `[sp+4] > 0x3 (unsigned, 32-bit)` |
| `jne` at 0x18288c | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `jne` at 0x1828b3 | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `je` at 0x1828e7 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x18290c | `[sp+16] == 0x0` | `[sp+16] != 0x0` |
| `je` at 0x182912 | `[sp+16] == 0x1` | `[sp+16] != 0x1` |
| `je` at 0x182918 | `[sp+16] == 0x6` | `[sp+16] != 0x6` |
| `je` at 0x18291e | `[sp+16] == 0x8` | `[sp+16] != 0x8` |
| `je` at 0x182924 | `[sp+16] == 0xc` | `[sp+16] != 0xc` |
| `je` at 0x18292a | `[sp+16] == 0xd` | `[sp+16] != 0xd` |
| `je` at 0x182930 | `[sp+16] == 0xe` | `[sp+16] != 0xe` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1829e6 | `eax` | `?` |

## `_eneBoardReset`

- `_eneBoardReset`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x183410 to 0x1834d0.
  Established. The two `_sysOutByte` sites have unknown candidate zero and candidate one `0x0`
  (`call` at 0x183453) and `0x21` (`call` at 0x18347a), respectively. The `_taskDelay` calls retain
  unknown candidates (`call` at 0x18343e, `call` at 0x183468). The repeat-path branch is taken when
  `(? & ?) == 0x0` (`je` at 0x18349e). The tested values remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x183414 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x183417 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x18341e | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183448 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183472 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183482 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1834c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x183429 | `_sysInByte 0x1081b0` | `?` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x183431 | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x18343e | `_taskDelay 0x1722d8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x183453 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x18345b | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x183468 | `_taskDelay 0x1722d8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x18347a | `_sysOutByte 0x1081d0` | `?` | `0x21` | `?` | `?` | `?` | `?` |
| `call` at 0x18348d | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x18349e | `(? & ?) == 0x0` | `(? & ?) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1834c6 | `eax` | `?` |

## `_eneInit`

- `_eneInit`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x183660 to 0x183970.
  Established. The `_sysOutByte` sites include candidate one `0x21` (`call` at 0x183681), `0x49`
  (`call` at 0x183696), `0x41` (`call` at 0x183779), and `0x47` (`call` at 0x1838ed), with unknown
  candidate zero at each site. The byte store has unknown target and known value `0x47` (`mov` at
  0x1838f5). The `_eneGetAddr` call has candidate zero `[sp+4]` (`call` at 0x1836a2).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x183667 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x18366a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x183674 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x183679 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18368b | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18369e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1836ac | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1836c1 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1836d4 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x1836de | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1836f3 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183708 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18371d | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183732 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183747 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18375c | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183771 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183781 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x183784 | load | 1 | `[sp-8] +0x0` | `-` |
| `inc` at 0x18378a | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x18378a | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x18378d | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1837a0 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1837a3 | load | 1 | `[sp-8] +0x0` | `-` |
| `inc` at 0x1837a9 | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x1837a9 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x1837ac | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1837bf | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1837c2 | load | 1 | `[sp-8] +0x0` | `-` |
| `inc` at 0x1837c8 | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x1837c8 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x1837cb | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1837de | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1837e1 | load | 1 | `[sp-8] +0x0` | `-` |
| `inc` at 0x1837e7 | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x1837e7 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x1837ea | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1837fd | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x183800 | load | 1 | `[sp-8] +0x0` | `-` |
| `inc` at 0x183806 | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x183806 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x183809 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18381c | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x18381f | load | 1 | `[sp-8] +0x0` | `-` |
| `mov` at 0x183825 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18383a | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18384f | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183864 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183879 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18388e | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1838a3 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1838b8 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1838cd | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1838e2 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1838f5 | store | 1 | `(unknown)` | `0x47` |
| `mov` at 0x1838fe | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183910 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183925 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18393a | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18394c | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18395f | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x183681 | `_sysOutByte 0x1081d0` | `?` | `0x21` | `ebx` | `?` | `ebp` | `?` |
| `call` at 0x183696 | `_sysOutByte 0x1081d0` | `?` | `0x49` | `?` | `?` | `?` | `?` |
| `call` at 0x1836a2 | `_eneGetAddr 0x183970` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1836b7 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x1836cc | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x1836e9 | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1836fe | `_sysOutByte 0x1081d0` | `?` | `0x4` | `?` | `?` | `?` | `?` |
| `call` at 0x183713 | `_sysOutByte 0x1081d0` | `?` | `0x46` | `?` | `?` | `?` | `?` |
| `call` at 0x183728 | `_sysOutByte 0x1081d0` | `?` | `0xffffff80` | `?` | `?` | `?` | `?` |
| `call` at 0x18373d | `_sysOutByte 0x1081d0` | `?` | `0x46` | `?` | `?` | `?` | `?` |
| `call` at 0x183752 | `_sysOutByte 0x1081d0` | `?` | `0xffffffff` | `?` | `?` | `?` | `?` |
| `call` at 0x183767 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x183779 | `_sysOutByte 0x1081d0` | `?` | `0x41` | `?` | `?` | `?` | `?` |
| `call` at 0x183798 | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1837b7 | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1837d6 | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1837f5 | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x183814 | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x183830 | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x183845 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x18385a | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x18386f | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x183884 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x183899 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x1838ae | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x1838c3 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x1838d8 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x1838ed | `_sysOutByte 0x1081d0` | `?` | `0x47` | `?` | `?` | `?` | `?` |
| `call` at 0x183906 | `_sysOutByte 0x1081d0` | `?` | `0xffffffa1` | `?` | `?` | `?` | `?` |
| `call` at 0x18391b | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x183930 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x183942 | `_sysOutByte 0x1081d0` | `?` | `0x22` | `?` | `?` | `?` | `?` |
| `call` at 0x183957 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x183965 | `eax` | `ret(_sysOutByte)` |

## `_eneintr`

- `_eneintr`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x182a40 to 0x182db0.
  Established. The three `_netJobAdd` sites have candidate zero `0x183e00` (`call` at 0x182bd8),
  `0x182db0` (`call` at 0x182ca7), and `0x183200` (`call` at 0x182d9d). Each has candidate one
  `[sp+4]`. The first has candidate two `[sp-8]:1` (`call` at 0x182bd8), while the other two have
  candidate two `0x0` (`call` at 0x182ca7, `call` at 0x182d9d).
  The first site follows the fall-through `(? & ?) != 0x0` (`je` at 0x182b71).
  The second requires the fall-through `(? & ?) != 0x0` (`je` at 0x182c60), then the fall-through
  `(? & ?) == 0x0` (`jne` at 0x182c83). The third follows two fall-through edges, each
  `(? & ?) != 0x0` (`je` at 0x182d83, `je` at 0x182d8a). The tested values remain unknown,
  and these call sites do not establish callback execution.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x182a47 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x182a4a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182a51 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182a5a | store | 4 | `(unknown)` | `?` |
| `mov` at 0x182a60 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182a66 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182a72 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x182a87 | store | 1 | `sp -0x5` | `?` |
| `movsx` at 0x182a8a | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x182a8f | load | 2 | `(unknown)` | `-` |
| `mov` at 0x182aa2 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x182ab7 | store | 1 | `sp -0x6` | `?` |
| `mov` at 0x182aba | load | 2 | `(unknown)` | `-` |
| `mov` at 0x182acf | store | 1 | `sp -0x7` | `?` |
| `mov` at 0x182ad2 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x182aea | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182af2 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x182af8 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x182b10 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182b18 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x182b1e | load | 2 | `(unknown)` | `-` |
| `mov` at 0x182b36 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182b3e | store | 4 | `(unknown)` | `?` |
| `mov` at 0x182b44 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x182b5c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182b64 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x182b6a | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x182b73 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182b7c | store | 4 | `(unknown)` | `?` |
| `mov` at 0x182b82 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182b88 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x182b9a | store | 1 | `sp -0x8` | `?` |
| `mov` at 0x182b9f | load | 2 | `(unknown)` | `-` |
| `mov` at 0x182bb4 | load | 2 | `(unknown)` | `-` |
| `movzx` at 0x182bca | load | 1 | `sp -0x8` | `-` |
| `mov` at 0x182bcf | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x182be0 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x182be9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182bf2 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x182bf8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182bfe | load | 1 | `sp -0x7` | `-` |
| `mov` at 0x182c08 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182c11 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x182c17 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182c1d | load | 1 | `sp -0x7` | `-` |
| `mov` at 0x182c26 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182c2f | store | 4 | `(unknown)` | `?` |
| `mov` at 0x182c35 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182c3b | load | 1 | `sp -0x7` | `-` |
| `mov` at 0x182c44 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182c4d | store | 4 | `(unknown)` | `?` |
| `mov` at 0x182c53 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182c59 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x182c62 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x182c70 | store | 1 | `(unknown)` | `?` |
| `mov` at 0x182c76 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182c85 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182c90 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x182c9e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x182caf | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x182cb8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182cc1 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x182cc7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182ccd | load | 1 | `sp -0x6` | `-` |
| `mov` at 0x182cd7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182ce0 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x182ce6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182cec | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182cf5 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x182cfb | load | 1 | `sp -0x6` | `-` |
| `mov` at 0x182d04 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182d0d | store | 4 | `(unknown)` | `?` |
| `mov` at 0x182d13 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182d19 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x182d22 | load | 1 | `sp -0x6` | `-` |
| `mov` at 0x182d2b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182d34 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x182d3a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182d40 | load | 1 | `sp -0x6` | `-` |
| `mov` at 0x182d49 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182d52 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x182d58 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182d5e | load | 1 | `sp -0x6` | `-` |
| `mov` at 0x182d67 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182d70 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x182d76 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182d7c | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x182d85 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182d94 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x182da5 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x182a6a | `_sysBusIntAck 0x108808` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x182a7d | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x182a9a | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x182aad | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x182ac5 | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x182add | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x182b03 | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x182b29 | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x182b4f | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x182b90 | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x182baa | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x182bbc | `_sysOutByte 0x1081d0` | `?` | `0x21` | `?` | `?` | `?` | `?` |
| `call` at 0x182bd8 | `_netJobAdd 0x184da0` | `0x183e00` | `[sp+4]` | `[sp-8]:1` | `0x0` | `0x0` | `0x0` |
| `call` at 0x182c66 | `_eneGetCurr 0x183d60` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x182ca7 | `_netJobAdd 0x184da0` | `0x182db0` | `[sp+4]` | `0x0` | `0x0` | `0x0` | `0x0` |
| `call` at 0x182d9d | `_netJobAdd 0x184da0` | `0x183200` | `[sp+4]` | `0x0` | `0x0` | `0x0` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x182b71 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x182be7 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x182c06 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x182c24 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x182c42 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x182c60 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `jne` at 0x182c83 | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `je` at 0x182cb6 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x182cd5 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x182d02 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x182d20 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x182d29 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x182d47 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x182d65 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x182d83 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x182d8a | `(? & ?) == 0x0` | `(? & ?) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x182dab | `eax` | `?` |

## `_eneIntEnable`

- `_eneIntEnable`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x183dd0 to 0x183e00.
  Established. The `_sysOutByte` call has unknown candidate zero and candidate one `0x1f` (`call` at
  0x183deb); its prior two-byte load also has an unknown effective address (`mov` at 0x183de0).
  The exit retains `ret(_sysOutByte)` (`ret` at 0x183df9). No register meaning is assigned to the
  candidate value.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x183dd4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x183dd7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x183de0 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183df3 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x183deb | `_sysOutByte 0x1081d0` | `?` | `0x1f` | `ebx` | `ebp` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x183df9 | `eax` | `ret(_sysOutByte)` |

## `_eneioctl`

- `_eneioctl`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x183540 to 0x183660.
  Established. The selector branches are taken when `[sp+8] == 0x8020690c` (`je` at 0x18357b) and
  `[sp+8] == 0x80206910` (`je` at 0x183582). A conditional `_sysOutByte` site has candidate zero
  `[[sp-12]+104]:2+12` and unknown candidate one (`call` at 0x183603). The byte store to `[sp-12]
  +0xe2` has unknown value (`mov` at 0x1835e2).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x183548 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x18354b | store | 4 | `sp -0x10` | `0x0` |
| `mov` at 0x183552 | load | 2 | `[sp+4] +0x4` | `-` |
| `mov` at 0x183559 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x18355c | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x18355f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x183566 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x183570 | store | 4 | `sp -0x14` | `ret(_splimp)` |
| `mov` at 0x183573 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x183590 | load | 2 | `[sp+4] +0x4` | `-` |
| `mov` at 0x183597 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1835a2 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1835b1 | store | 4 | `sp -0x10` | `ret(_set_if_addr)` |
| `mov` at 0x1835c0 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1835c3 | load | 2 | `[sp+4] +0x8` | `-` |
| `mov` at 0x1835e2 | store | 1 | `[sp-12] +0xe2` | `?` |
| `mov` at 0x1835e8 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1835eb | load | 1 | `[sp-12] +0xe2` | `-` |
| `mov` at 0x1835f5 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1835f8 | load | 2 | `[sp-12] +0x68` | `-` |
| `mov` at 0x18360b | load | 2 | `[sp+4] +0x8` | `-` |
| `mov` at 0x18361a | load | 2 | `[sp+4] +0x4` | `-` |
| `mov` at 0x183630 | store | 4 | `sp -0x10` | `0x16` |
| `mov` at 0x183637 | load | 4 | `sp -0x14` | `-` |
| `mov` at 0x183643 | load | 4 | `sp -0x10` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x183569 | `_splimp 0x19823c` | `ebx` | `esi` | `?` | `0x0` | `?` | `?` |
| `call` at 0x1835a7 | `_set_if_addr 0x198068` | `[sp+4]` | `[sp+12]` | `?` | `?` | `?` | `ret(_splimp)` |
| `call` at 0x183603 | `_sysOutByte 0x1081d0` | `[[sp-12]+104]:2+12` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x183622 | `_eneinit 0x182a10` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x18363b | `_splx 0x19827c` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x18357b | `[sp+8] == 0x8020690c` | `[sp+8] != 0x8020690c` |
| `je` at 0x183582 | `[sp+8] == 0x80206910` | `[sp+8] != 0x80206910` |
| `je` at 0x1835d1 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x183618 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x183658 | `eax` | `[sp-16]` |

## `_eneReset`

- `_eneReset`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1834d0 to 0x183540.
  Established. The `_sysOutByte` site has unknown candidate zero and candidate one `0x0` (`call` at
  0x1834f8). The subsequent `_eneInit` (`call` at 0x183504) and `_eneIntEnable` (`call` at 0x183510)
  sites each have candidate zero `[sp+4]`. A later four-byte store has unknown effective address and
  value (`mov` at 0x183521).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1834d7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1834da | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1834e8 | store | 4 | `sp -0x8` | `ret(_splnet)` |
| `mov` at 0x1834ed | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183500 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x18350c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x183518 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x183521 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x183527 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x18352d | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x183539 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1834e1 | `_splnet 0x1981fc` | `ebx` | `?` | `ebp` | `?` | `?` | `?` |
| `call` at 0x1834f8 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `ret(_splnet)` | `?` | `?` |
| `call` at 0x183504 | `_eneInit 0x183660` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x183510 | `_eneIntEnable 0x183dd0` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x183531 | `_splx 0x19827c` | `[sp-8]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x18353f | `eax` | `ret(_splx)` |

## `_eneOverwriteRecover`

- `_eneOverwriteRecover`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x183e00 to 0x184140.
  Established. The `_sysOutByte` sites with leading candidates `[sp-16]+10`, `0x0` (`call` at
  0x183e62) and `[sp-16]+11`, `0x0` (`call` at 0x183e7d) retain stack-loaded origins. A later site
  presents `[sp-16]` and `0x22` (`call` at 0x183f02). The `_eneGet` call has candidate zero `[sp+4]`
  (`call` at 0x183f32). The stored byte through an unknown effective address has value `[sp-16]:1`
  (`mov` at 0x183f22).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x183e08 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x183e0b | store | 1 | `sp -0x5` | `?` |
| `mov` at 0x183e0e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x183e11 | store | 4 | `sp -0x10` | `[sp+4]` |
| `mov` at 0x183e14 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x183e17 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x183e23 | store | 4 | `sp -0x10` | `ret(_sysClkRateGet)` |
| `mov` at 0x183e2b | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x183e35 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x183e3f | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x183e42 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x183e51 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183e58 | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x183e5b | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x183e6c | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183e73 | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x183e76 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x183e85 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x183e8a | store | 1 | `sp -0x10` | `?` |
| `cmp` at 0x183e8d | load | 1 | `sp -0x10` | `-` |
| `mov` at 0x183e93 | store | 4 | `sp -0xc` | `0x0` |
| `mov` at 0x183ea0 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183ea7 | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x183eaa | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x183eb9 | store | 1 | `sp -0x10` | `?` |
| `mov` at 0x183ebc | load | 1 | `sp -0x10` | `-` |
| `mov` at 0x183ec6 | store | 4 | `sp -0xc` | `0x1` |
| `mov` at 0x183ed0 | store | 4 | `sp -0xc` | `0x0` |
| `mov` at 0x183ed9 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183ee0 | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x183ee3 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x183ef4 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183efb | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x183efe | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x183f0a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x183f0d | store | 4 | `sp -0x10` | `[sp+4]` |
| `mov` at 0x183f10 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x183f1c | store | 1 | `sp -0x10` | `?` |
| `mov` at 0x183f1f | load | 1 | `sp -0x10` | `-` |
| `mov` at 0x183f22 | store | 1 | `(unknown)` | `[sp-16]:1` |
| `mov` at 0x183f28 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x183f2b | store | 4 | `sp -0x10` | `[sp+4]` |
| `mov` at 0x183f2e | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x183f3a | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183f41 | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x183f44 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x183f53 | store | 1 | `sp -0x10` | `?` |
| `movsx` at 0x183f56 | load | 1 | `sp -0x10` | `-` |
| `mov` at 0x183f5b | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183f62 | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x183f65 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x183f76 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183f7d | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x183f80 | load | 4 | `sp -0x10` | `-` |
| `cmp` at 0x183f8f | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x183f97 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183f9e | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x183fa1 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x183fb0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x183fb3 | store | 4 | `sp -0x10` | `?` |
| `cmp` at 0x183fb6 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x183fbc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x183fbf | store | 4 | `sp -0x10` | `[sp+4]` |
| `mov` at 0x183fc2 | load | 4 | `sp -0x10` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x183e1e | `_sysClkRateGet 0x1085a8` | `ebx` | `esi` | `[sp+4]` | `?` | `?` | `ebp` |
| `call` at 0x183e47 | `_taskDelay 0x1722d8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x183e62 | `_sysOutByte 0x1081d0` | `[sp-16]+10` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x183e7d | `_sysOutByte 0x1081d0` | `[sp-16]+11` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x183eb1 | `_sysInByte 0x1081b0` | `[sp-16]+7` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x183eea | `_sysOutByte 0x1081d0` | `[sp-16]+13` | `0x4` | `?` | `?` | `?` | `?` |
| `call` at 0x183f02 | `_sysOutByte 0x1081d0` | `[sp-16]` | `0x22` | `?` | `?` | `?` | `?` |
| `call` at 0x183f14 | `_eneGetCurr 0x183d60` | `[sp+4]` | `?` | `?` | `[sp+4]` | `?` | `?` |
| `call` at 0x183f32 | `_eneGet 0x182db0` | `[sp+4]` | `?` | `?` | `[sp+4]` | `?` | `?` |
| `call` at 0x183f4b | `_sysInByte 0x1081b0` | `[sp-16]+7` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x183f6c | `_sysOutByte 0x1081d0` | `[sp-16]+7` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x183f87 | `_sysOutByte 0x1081d0` | `[sp-16]+13` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x183fa5 | `_sysOutByte 0x1081d0` | `[sp-16]` | `0x4` | `?` | `?` | `?` | `?` |
| `call` at 0x183fc6 | `_enePut 0x183200` | `[sp+4]` | `?` | `?` | `[sp+4]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x183e91 | `([sp-16]:1 & 0xff) != 0x0` | `([sp-16]:1 & 0xff) == 0x0` |
| `jne` at 0x183ec4 | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `je` at 0x183f93 | `[sp-12] == 0x0` | `[sp-12] != 0x0` |
| `je` at 0x183fba | `[sp-16] == 0x0` | `[sp-16] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x183fd6 | `eax` | `?` |

## `_eneGetCurr`

- `_eneGetCurr`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x183d60 to 0x183dd0.
  Established. The `_sysOutByte` sites present unknown candidate zero with candidate one `0x40`
  (`call` at 0x183d7b) and `0x0` (`call` at 0x183da5). The intervening `_sysInByte` site has unknown
  candidate values (`call` at 0x183d8e). The byte store at `sp -0x5` has unknown value (`mov` at
  0x183d98), and the exit reports `[sp-5]:1` (`ret` at 0x183dc6).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x183d67 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x183d6a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x183d73 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183d83 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183d98 | store | 1 | `sp -0x5` | `?` |
| `mov` at 0x183d9d | load | 2 | `(unknown)` | `-` |
| `movzx` at 0x183dad | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x183dc0 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x183d7b | `_sysOutByte 0x1081d0` | `?` | `0x40` | `ebx` | `?` | `ebp` | `?` |
| `call` at 0x183d8e | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x183da5 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x183dc6 | `eax` | `[sp-5]:1` |

## `_eneGet`

- `_eneGet`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x182db0 to 0x1831b0.
  Established. A byte store to absolute `0x1db988` has unknown value (`mov` at 0x182e3c). The two
  `_eneDataIn` sites have candidate zero `[sp+4]`, unknown candidate one, and candidate two `0x4`
  (`call` at 0x182e1e) or `[sp-12]:2` (`call` at 0x18305a). The indirect call has unknown callee and
  candidate two `[sp-12]` (`call` at 0x183083).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x182db9 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x182dbc | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182de0 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x182de6 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x182e06 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x182e1a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x182e26 | load | 1 | `(unknown)` | `-` |
| `cmp` at 0x182e2c | load | 1 | `sp -0x17` | `-` |
| `mov` at 0x182e31 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x182e37 | load | 1 | `sp -0x17` | `-` |
| `mov` at 0x182e3c | store | 1 | `abs 0x1db988` | `?` |
| `mov` at 0x182e50 | load | 1 | `sp -0x17` | `-` |
| `mov` at 0x182e55 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x182e5f | store | 1 | `abs 0x1db988` | `?` |
| `cmp` at 0x182e65 | load | 1 | `sp -0x16` | `-` |
| `add` at 0x182e6b | load | 1 | `abs 0x1db988` | `-` |
| `add` at 0x182e6b | store | 1 | `abs 0x1db988` | `[0x1db988]:1-2` |
| `dec` at 0x182e80 | load | 1 | `abs 0x1db988` | `-` |
| `dec` at 0x182e80 | store | 1 | `abs 0x1db988` | `?` |
| `mov` at 0x182e86 | load | 1 | `abs 0x1db988` | `-` |
| `mov` at 0x182e8c | store | 1 | `sp -0x15` | `[0x1db988]:1` |
| `movzx` at 0x182e8f | load | 1 | `sp -0x15` | `-` |
| `movzx` at 0x182e98 | load | 1 | `sp -0x16` | `-` |
| `mov` at 0x182ea1 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x182ea4 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x182eb1 | store | 1 | `sp -0x12` | `?` |
| `mov` at 0x182eb4 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x182eba | load | 1 | `sp -0x12` | `-` |
| `mov` at 0x182ebf | store | 1 | `sp -0x11` | `?` |
| `cmp` at 0x182ec2 | load | 1 | `sp -0x11` | `-` |
| `add` at 0x182ec8 | load | 1 | `sp -0x11` | `-` |
| `add` at 0x182ec8 | store | 1 | `sp -0x11` | `[sp-17]:1-58` |
| `mov` at 0x182ecc | load | 1 | `sp -0x17` | `-` |
| `cmp` at 0x182ecf | load | 1 | `sp -0x11` | `-` |
| `cmp` at 0x182ed4 | load | 1 | `sp -0x17` | `-` |
| `cmp` at 0x182eda | load | 1 | `sp -0x17` | `-` |
| `mov` at 0x182ef0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182ef9 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x182eff | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182f05 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182f0e | store | 4 | `(unknown)` | `?` |
| `mov` at 0x182f14 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182f1a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x182f26 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182f31 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x182f37 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x182f50 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x182f56 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182f5f | store | 4 | `(unknown)` | `?` |
| `mov` at 0x182f65 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x182f70 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x182f80 | load | 1 | `sp -0x18` | `-` |
| `mov` at 0x182f89 | load | 1 | `sp -0x18` | `-` |
| `mov` at 0x182f92 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182f9b | store | 4 | `(unknown)` | `?` |
| `mov` at 0x182fa1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182fa7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182fb0 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x182fb6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182fd0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182fd9 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x182fdf | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182ff0 | load | 1 | `sp -0x18` | `-` |
| `mov` at 0x182ff9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x183002 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x183008 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x18300e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x183017 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x18301d | load | 4 | `(unknown)` | `-` |
| `movzx` at 0x183037 | load | 2 | `sp -0xc` | `-` |
| `mov` at 0x18303c | load | 1 | `(unknown)` | `-` |
| `mov` at 0x183056 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x183068 | load | 4 | `abs 0x1d348c` | `-` |
| `mov` at 0x183071 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x183077 | load | 4 | `abs 0x1d348c` | `-` |
| `mov` at 0x18307d | store | 4 | `sp -0x1c` | `[0x1d348c]` |
| `mov` at 0x183080 | load | 4 | `sp -0x1c` | `-` |
| `cmp` at 0x1830a0 | load | 4 | `sp -0xc` | `-` |
| `add` at 0x1830a6 | load | 4 | `sp -0xc` | `-` |
| `add` at 0x1830a6 | store | 4 | `sp -0xc` | `[sp-12]-14` |
| `mov` at 0x1830c3 | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x1830cf | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x1830df | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1830e3 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1830e7 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x183100 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x183106 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183116 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x18311c | store | 4 | `(unknown)` | `?` |
| `mov` at 0x18311f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x183122 | load | 1 | `sp -0x17` | `-` |
| `mov` at 0x183125 | store | 1 | `(unknown)` | `[sp-23]:1` |
| `mov` at 0x18312b | load | 1 | `(unknown)` | `-` |
| `mov` at 0x183135 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x183156 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183170 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x18317b | store | 4 | `(unknown)` | `?` |
| `mov` at 0x183181 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x183187 | load | 1 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x182e1e | `_eneDataIn 0x183c20` | `[sp+4]` | `?` | `0x4` | `sp-24` | `?` | `?` |
| `call` at 0x182f1e | `_eneInit 0x183660` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x182f3b | `_eneIntEnable 0x183dd0` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x18305a | `_eneDataIn 0x183c20` | `[sp+4]` | `?` | `[sp-12]:2` | `?` | `?` | `?` |
| `call` at 0x183083 | `(unknown callee) (unknown target)` | `?` | `?` | `[sp-12]` | `?` | `?` | `?` |
| `call` at 0x1830d4 | `_check_trailer 0x1977a8` | `?` | `[sp-16]` | `sp-12` | `sp-8` | `?` | `?` |
| `call` at 0x1830eb | `_bcopy_to_mbufs 0x197878` | `[sp-16]` | `[sp-12]` | `[sp-8]` | `?` | `0x2` | `?` |
| `call` at 0x18310e | `_do_protocol_with_type 0x197cd8` | `?` | `ret(_bcopy_to_mbufs)` | `?` | `[sp-12]` | `?` | `?` |
| `call` at 0x183161 | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x182dee | `? != ?` | `? == ?` |
| `jbe` at 0x182e2f | `([sp-23]:1 & 0xff) <= ? (unsigned, 8-bit)` | `([sp-23]:1 & 0xff) > ? (unsigned, 8-bit)` |
| `jbe` at 0x182e69 | `([sp-22]:1 & 0xff) <= 0xfc (unsigned, 8-bit)` | `([sp-22]:1 & 0xff) > 0xfc (unsigned, 8-bit)` |
| `jbe` at 0x182ec6 | `([sp-17]:1 & 0xff) <= 0x7f (unsigned, 8-bit)` | `([sp-17]:1 & 0xff) > 0x7f (unsigned, 8-bit)` |
| `jne` at 0x182ed2 | `([sp-17]:1 & 0xff) != ([sp-23]:1 & 0xff)` | `([sp-17]:1 & 0xff) == ([sp-23]:1 & 0xff)` |
| `jbe` at 0x182ed8 | `([sp-23]:1 & 0xff) <= 0x45 (unsigned, 8-bit)` | `([sp-23]:1 & 0xff) > 0x45 (unsigned, 8-bit)` |
| `ja` at 0x182ede | `([sp-23]:1 & 0xff) > 0x7f (unsigned, 8-bit)` | `([sp-23]:1 & 0xff) <= 0x7f (unsigned, 8-bit)` |
| `ja` at 0x182f54 | `[sp-12] > 0x3b (unsigned, 32-bit)` | `[sp-12] <= 0x3b (unsigned, 32-bit)` |
| `jbe` at 0x182f77 | `[sp-12] <= 0x5ea (unsigned, 32-bit)` | `[sp-12] > 0x5ea (unsigned, 32-bit)` |
| `je` at 0x182f87 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x182f90 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x182ff7 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x18306f | `[0x1d348c] == 0x0` | `[0x1d348c] != 0x0` |
| `je` at 0x18308c | `(ret(unknown) & ret(unknown)) == 0x0` | `(ret(unknown) & ret(unknown)) != 0x0` |
| `jbe` at 0x1830a4 | `[sp-12] <= 0xd (unsigned, 32-bit)` | `[sp-12] > 0xd (unsigned, 32-bit)` |
| `jne` at 0x1830f7 | `(ret(_bcopy_to_mbufs) & ret(_bcopy_to_mbufs)) != 0x0` | `(ret(_bcopy_to_mbufs) & ret(_bcopy_to_mbufs)) == 0x0` |
| `je` at 0x183133 | `? == 0x46` | `? != 0x46` |
| `je` at 0x18318f | `? == ?` | `? != ?` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1831a9 | `eax` | `?` |

## `_enePut`

- `_enePut`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x183200 to 0x183410.
  Established. The four-byte local store writes `0x2710` to `sp -0x8` (`mov` at 0x183228), without
  establishing timing units. The `_eneDataOut` site has candidate zero `[sp+4]`, unknown candidates
  one and two, and candidate three `0x4000` (`call` at 0x183374). Later `_sysOutByte` sites include
  candidate one `0x0` (`call` at 0x183389), `0x40` (`call` at 0x18339e), and `0x4` (`call` at
  0x1833e5), each with unknown candidate zero.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x183209 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x18320c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x18321a | store | 4 | `sp -0xc` | `ret(_splnet)` |
| `mov` at 0x18321d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x183228 | store | 4 | `sp -0x8` | `0x2710` |
| `mov` at 0x18322f | load | 2 | `(unknown)` | `-` |
| `dec` at 0x18324a | load | 4 | `sp -0x8` | `-` |
| `dec` at 0x18324a | store | 4 | `sp -0x8` | `?` |
| `cmp` at 0x18324d | load | 4 | `sp -0x8` | `-` |
| `cmp` at 0x183280 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x183286 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x183292 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x18329c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x18329f | store | 4 | `(unknown)` | `?` |
| `mov` at 0x1832a1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1832a7 | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x1832ae | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x1832b8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1832be | store | 4 | `sp -0x1c` | `?` |
| `mov` at 0x1832c1 | load | 4 | `sp -0x1c` | `-` |
| `mov` at 0x1832c4 | store | 4 | `(unknown)` | `[sp-28]` |
| `mov` at 0x1832c7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1832d0 | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x1832d3 | store | 4 | `sp -0x14` | `?` |
| `mov` at 0x1832e0 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1832e8 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x1832ec | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1832fb | load | 2 | `(unknown)` | `-` |
| `add` at 0x183302 | load | 4 | `sp -0x10` | `-` |
| `add` at 0x183302 | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x183305 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x183310 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x18331b | load | 4 | `sp -0x14` | `-` |
| `cmp` at 0x183331 | load | 4 | `abs 0x1d3490` | `-` |
| `mov` at 0x183343 | load | 4 | `abs 0x1d3490` | `-` |
| `mov` at 0x183349 | store | 4 | `sp -0x18` | `[0x1d3490]` |
| `mov` at 0x18334c | load | 4 | `sp -0x18` | `-` |
| `mov` at 0x183370 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x18337e | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183393 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1833af | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1833c8 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1833dd | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1833ed | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1833f9 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x183213 | `_splnet 0x1981fc` | `ebx` | `esi` | `edi` | `?` | `?` | `?` |
| `call` at 0x183237 | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x18328a | `_eneInit 0x183660` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1832f3 | `_bcopy 0x176bac` | `?` | `[sp-16]` | `?` | `?` | `?` | `?` |
| `call` at 0x18331f | `_m_freem 0x198c40` | `[sp-20]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x18334f | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x183374 | `_eneDataOut 0x183ac0` | `[sp+4]` | `?` | `?` | `0x4000` | `?` | `?` |
| `call` at 0x183389 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x18339e | `_sysOutByte 0x1081d0` | `?` | `0x40` | `?` | `?` | `?` | `?` |
| `call` at 0x1833ba | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1833d3 | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1833e5 | `_sysOutByte 0x1081d0` | `?` | `0x4` | `?` | `?` | `?` | `?` |
| `call` at 0x1833f1 | `_eneIntEnable 0x183dd0` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1833fd | `_splx 0x19827c` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x183222 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x183248 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `jg` at 0x183251 | `[sp-8] > 0x0 (signed, 32-bit)` | `[sp-8] <= 0x0 (signed, 32-bit)` |
| `jne` at 0x183284 | `[sp-8] != 0x0` | `[sp-8] == 0x0` |
| `je` at 0x183297 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `jne` at 0x1832a5 | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `jne` at 0x1832d8 | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `jge` at 0x18332a | `? >= 0x3c (signed, 32-bit)` | `? < 0x3c (signed, 32-bit)` |
| `je` at 0x183338 | `[0x1d3490] == 0x0` | `[0x1d3490] != 0x0` |
| `je` at 0x183358 | `(ret(unknown) & ret(unknown)) == 0x0` | `(ret(unknown) & ret(unknown)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x18340e | `eax` | `ret(_splx)` |

## `_eneDataIn`

- `_eneDataIn`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x183c20 to 0x183d60.
  Established. The `_sysInWordString` call has unknown candidate zero, candidate one `[sp+16]`, and
  unknown candidate two (`call` at 0x183d05). The branch falls through when `(? & ?) != 0x0` (`je`
  at 0x183d18), reaching a byte store to `([sp-8]:2 + [sp+16]) -0x1` with unknown value (`mov` at
  0x183d3d). Both branch paths reach `_eneIntEnable` with candidate zero `[sp+4]` (`call` at
  0x183d43).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x183c29 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x183c2c | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x183c2f | store | 2 | `sp -0x6` | `?` |
| `mov` at 0x183c33 | store | 2 | `sp -0x8` | `?` |
| `mov` at 0x183c37 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x183c3a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x183c43 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183c58 | load | 2 | `(unknown)` | `-` |
| `movsx` at 0x183c68 | load | 1 | `sp -0x6` | `-` |
| `mov` at 0x183c6d | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183c80 | load | 2 | `sp -0x6` | `-` |
| `mov` at 0x183c8c | load | 2 | `(unknown)` | `-` |
| `movsx` at 0x183c9f | load | 1 | `sp -0x8` | `-` |
| `mov` at 0x183ca4 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183cb7 | load | 2 | `sp -0x8` | `-` |
| `mov` at 0x183cc3 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183cd8 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183ce8 | load | 2 | `sp -0x8` | `-` |
| `mov` at 0x183cf6 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x183cfa | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183d0d | load | 2 | `sp -0x8` | `-` |
| `mov` at 0x183d1a | load | 2 | `(unknown)` | `-` |
| `movzx` at 0x183d2f | load | 2 | `sp -0x8` | `-` |
| `add` at 0x183d35 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x183d3d | store | 1 | `([sp-8]:2 + [sp+16]) -0x1` | `?` |
| `mov` at 0x183d3f | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x183c4e | `_sysOutByte 0x1081d0` | `?` | `0x0` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x183c60 | `_sysOutByte 0x1081d0` | `?` | `0x22` | `?` | `?` | `?` | `?` |
| `call` at 0x183c78 | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x183c97 | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x183caf | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x183cce | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x183ce0 | `_sysOutByte 0x1081d0` | `?` | `0xa` | `?` | `?` | `?` | `?` |
| `call` at 0x183d05 | `_sysInWordString 0x1081f0` | `?` | `[sp+16]` | `?` | `?` | `?` | `?` |
| `call` at 0x183d25 | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x183d43 | `_eneIntEnable 0x183dd0` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x183d18 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x183d54 | `eax` | `ret(_eneIntEnable)` |

## `_eneDataOut`

- `_eneDataOut`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x183ac0 to 0x183c20.
  Established. The initial branch is taken when `([sp-6]:2 & 0xffff) != 0x0` (`jne` at 0x183ae6). On
  that path, the `_sysOutWordString` site has unknown candidate zero, candidate one `[sp+8]`, and
  unknown candidate two (`call` at 0x183bc9). The earlier `_sysOutByte` sites include candidate one
  `0x22` (`call` at 0x183b24) and `0x12` (`call` at 0x183ba4), with unknown candidate zero at both
  sites. After the string-call site, the branch falls through when `(? & ?) != 0x0` (`je` at
  0x183bdc), reaching a byte load from `([sp-6]:2 + [sp+8]) -0x1` (`mov` at 0x183bec) and a
  `_sysOutByte` call whose candidates remain unknown (`call` at 0x183bfd). Both arms of that
  branch reach `_eneIntEnable` with candidate zero `[sp+4]` (`call` at 0x183c09). The initial
  branch's fall-through bypasses all those calls (`jne` at 0x183ae6, `jmp` at 0x183ae8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x183ac9 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x183acc | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x183acf | store | 2 | `sp -0x6` | `?` |
| `mov` at 0x183ad3 | store | 2 | `sp -0x8` | `?` |
| `mov` at 0x183ad7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x183ada | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x183ae1 | load | 2 | `sp -0x6` | `-` |
| `mov` at 0x183af2 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183b07 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183b1c | load | 2 | `(unknown)` | `-` |
| `movsx` at 0x183b2c | load | 1 | `sp -0x8` | `-` |
| `mov` at 0x183b31 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183b44 | load | 2 | `sp -0x8` | `-` |
| `mov` at 0x183b50 | load | 2 | `(unknown)` | `-` |
| `movsx` at 0x183b63 | load | 1 | `sp -0x6` | `-` |
| `mov` at 0x183b68 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183b7b | load | 2 | `sp -0x6` | `-` |
| `mov` at 0x183b87 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183b9c | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183bac | load | 2 | `sp -0x6` | `-` |
| `mov` at 0x183bba | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x183bbe | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183bd1 | load | 2 | `sp -0x6` | `-` |
| `movzx` at 0x183bde | load | 2 | `sp -0x6` | `-` |
| `add` at 0x183be4 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x183bec | load | 1 | `([sp-6]:2 + [sp+8]) -0x1` | `-` |
| `mov` at 0x183bf2 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183c05 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x183afd | `_sysOutByte 0x1081d0` | `?` | `0x0` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x183b12 | `_sysOutByte 0x1081d0` | `?` | `0xffffffff` | `?` | `?` | `?` | `?` |
| `call` at 0x183b24 | `_sysOutByte 0x1081d0` | `?` | `0x22` | `?` | `?` | `?` | `?` |
| `call` at 0x183b3c | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x183b5b | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x183b73 | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x183b92 | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x183ba4 | `_sysOutByte 0x1081d0` | `?` | `0x12` | `?` | `?` | `?` | `?` |
| `call` at 0x183bc9 | `_sysOutWordString 0x108210` | `?` | `[sp+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x183bfd | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x183c09 | `_eneIntEnable 0x183dd0` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x183ae6 | `([sp-6]:2 & 0xffff) != 0x0` | `([sp-6]:2 & 0xffff) == 0x0` |
| `je` at 0x183bdc | `(? & ?) == 0x0` | `(? & ?) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x183c1a | `eax` | `?` |

## `_eneGetAddr`

- `_eneGetAddr`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x183970 to 0x183ac0.
  Established. The `_sysOutByte` sites include candidate one `0x22` (`call` at 0x183991), `0xc`
  (`call` at 0x1839d0), and `0xa` (`call` at 0x1839f7), each with unknown candidate zero. Byte
  stores through `[sp-8] +0x0` have unknown values (`mov` at 0x183a17, `mov` at 0x183aa8). The
  `_sysInByte` sites also retain unknown candidate values (`call` at 0x183a0a, `call` at 0x183a9b).
  No address-record meaning is inferred from the function name.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x183977 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x18397a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x183984 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x183989 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18399b | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1839b0 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1839c5 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1839da | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1839ef | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1839ff | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183a14 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x183a17 | store | 1 | `[sp-8] +0x0` | `?` |
| `inc` at 0x183a19 | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x183a19 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x183a1c | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183a31 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x183a34 | store | 1 | `[sp-8] +0x0` | `?` |
| `inc` at 0x183a36 | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x183a36 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x183a39 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183a4e | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x183a51 | store | 1 | `[sp-8] +0x0` | `?` |
| `inc` at 0x183a53 | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x183a53 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x183a56 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183a6b | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x183a6e | store | 1 | `[sp-8] +0x0` | `?` |
| `inc` at 0x183a70 | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x183a70 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x183a73 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183a88 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x183a8b | store | 1 | `[sp-8] +0x0` | `?` |
| `inc` at 0x183a8d | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x183a8d | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x183a90 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x183aa5 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x183aa8 | store | 1 | `[sp-8] +0x0` | `?` |
| `mov` at 0x183aaa | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x183991 | `_sysOutByte 0x1081d0` | `?` | `0x22` | `ebx` | `?` | `ebp` | `?` |
| `call` at 0x1839a6 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x1839bb | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x1839d0 | `_sysOutByte 0x1081d0` | `?` | `0xc` | `?` | `?` | `?` | `?` |
| `call` at 0x1839e5 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x1839f7 | `_sysOutByte 0x1081d0` | `?` | `0xa` | `?` | `?` | `?` | `?` |
| `call` at 0x183a0a | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x183a27 | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x183a44 | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x183a61 | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x183a7e | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x183a9b | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x183ab0 | `eax` | `?` |

## `_enedetach`

- `_enedetach`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x184230 to 0x1842f0.
  Established. The initial branch is taken when `(? & ?) == 0x0` (`je` at 0x184243), reaching the
  four-byte store of `0x0` through an unknown effective address (`mov` at 0x1842e2). The other path
  includes `_sysOutByte` sites with unknown candidate zero and candidate one `0x0` (`call` at
  0x184256) and `0x21` (`call` at 0x184268). The `_eneBoardReset` call has candidate zero `[sp+4]`
  (`call` at 0x1842da).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x184234 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x184237 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x18423e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x18424b | load | 2 | `(unknown)` | `-` |
| `mov` at 0x184260 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x184270 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1842a2 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1842b7 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1842ca | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1842d6 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1842e2 | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x1842e9 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x184256 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `ebx` | `ebp` | `?` | `?` |
| `call` at 0x184268 | `_sysOutByte 0x1081d0` | `?` | `0x21` | `?` | `?` | `?` | `?` |
| `call` at 0x18427b | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1842ad | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x1842c2 | `_sysOutByte 0x1081d0` | `?` | `0x4` | `?` | `?` | `?` | `?` |
| `call` at 0x1842ce | `_sysIntDisablePIC 0x1083b8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1842da | `_eneBoardReset 0x183410` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x184243 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x18428c | `(? & ?) == 0x0` | `(? & ?) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1842ef | `eax` | `?` |
