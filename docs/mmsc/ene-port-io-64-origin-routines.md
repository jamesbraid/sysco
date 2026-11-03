# MMSC Ethernet port-I/O callers in the 6.4-origin image

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

[Fact sheets and complete listings](disasm/ene-port-io-64-origin-facts.txt) include all tool sections.

## `_eneattach`

- `_eneattach`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1804a0 to 0x180640.
  Established. The opening branch is taken when `[sp+4] <= 0x3 (unsigned, 32-bit)` (`jbe` at
  0x1804a8). The `_malloc` site presents candidate zero `0x6d8` (`call` at 0x1804e3). A four-byte
  store at an unresolved effective address retains `ret(_malloc)` (`mov` at 0x1804f0). The
  `_intConnect` site has unknown candidate zero, candidate one `0x180690`, and candidate two
  `[sp+4]` (`call` at 0x1805d6). No allocation size or handler invocation follows from these
  candidate slots.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x1804a4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1804d0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1804d3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1804ed | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1804f0 | store | 4 | `(unknown)` | `ret(_malloc)` |
| `mov` at 0x1804f7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1804fa | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180515 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x180518 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180528 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x18052b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180532 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180550 | load | 2 | `sp +0x8` | `-` |
| `mov` at 0x180554 | store | 2 | `(unknown)` | `[sp+8]:2` |
| `cmp` at 0x180558 | load | 4 | `sp +0x10` | `-` |
| `cmp` at 0x18055e | load | 4 | `sp +0x10` | `-` |
| `cmp` at 0x180564 | load | 4 | `sp +0x10` | `-` |
| `cmp` at 0x18056a | load | 4 | `sp +0x10` | `-` |
| `cmp` at 0x180570 | load | 4 | `sp +0x10` | `-` |
| `cmp` at 0x180576 | load | 4 | `sp +0x10` | `-` |
| `cmp` at 0x18057c | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x180590 | store | 4 | `sp +0x10` | `0x3` |
| `mov` at 0x180597 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x18059a | store | 4 | `(unknown)` | `?` |
| `mov` at 0x18059d | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1805a0 | store | 4 | `(unknown)` | `[sp+12]` |
| `mov` at 0x1805a3 | store | 1 | `(unknown)` | `0x4` |
| `mov` at 0x1805aa | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1805b6 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1805c2 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1805cb | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1805de | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1805ea | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x18060f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x18061c | store | 4 | `(unknown)` | `0x1` |
| `mov` at 0x180630 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1804af | `_errnoSet 0x16fae8` | `0xd0005` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x1804e3 | `_malloc 0x17c878` | `0x6d8` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x180520 | `_bzero 0x170f20` | `?` | `0x6d8` | `?` | `?` | `?` | `?` |
| `call` at 0x1805ae | `_eneBoardReset 0x181060` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1805ba | `_eneInit 0x1812b0` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1805d6 | `_intConnect 0x173fe8` | `?` | `0x180690` | `[sp+4]` | `?` | `?` | `?` |
| `call` at 0x1805e2 | `_eneIntEnable 0x181a20` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1805ee | `_sysIntEnablePIC 0x108428` | `[sp+16]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x180614 | `_ether_attach 0x195d18` | `?` | `[sp+4]` | `0x180490` | `0x180660` | `0x181190` | `0x180e00` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x1804a8 | `[sp+4] <= 0x3 (unsigned, 32-bit)` | `[sp+4] > 0x3 (unsigned, 32-bit)` |
| `jne` at 0x1804dc | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `jne` at 0x180503 | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `je` at 0x180537 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x18055c | `[sp+16] == 0x0` | `[sp+16] != 0x0` |
| `je` at 0x180562 | `[sp+16] == 0x1` | `[sp+16] != 0x1` |
| `je` at 0x180568 | `[sp+16] == 0x6` | `[sp+16] != 0x6` |
| `je` at 0x18056e | `[sp+16] == 0x8` | `[sp+16] != 0x8` |
| `je` at 0x180574 | `[sp+16] == 0xc` | `[sp+16] != 0xc` |
| `je` at 0x18057a | `[sp+16] == 0xd` | `[sp+16] != 0xd` |
| `je` at 0x180580 | `[sp+16] == 0xe` | `[sp+16] != 0xe` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x180636 | `eax` | `?` |

## `_eneBoardReset`

- `_eneBoardReset`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x181060 to 0x181120.
  Established. The `_sysOutByte` sites retain unknown candidate zero with candidate one `0x0`
  (`call` at 0x1810a3) and `0x21` (`call` at 0x1810ca). Both `_taskDelay` sites have unknown
  candidates (`call` at 0x18108e, `call` at 0x1810b8). The deciding branch is taken when `(? & ?) ==
  0x0` (`je` at 0x1810ee). Port addresses, delay units and a reset outcome remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x181064 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x181067 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x18106e | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181098 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1810c2 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1810d2 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181110 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x181079 | `_sysInByte 0x1081b0` | `?` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x181081 | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x18108e | `_taskDelay 0x170b80` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1810a3 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x1810ab | `_sysClkRateGet 0x1085a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1810b8 | `_taskDelay 0x170b80` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1810ca | `_sysOutByte 0x1081d0` | `?` | `0x21` | `?` | `?` | `?` | `?` |
| `call` at 0x1810dd | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1810ee | `(? & ?) == 0x0` | `(? & ?) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x181116 | `eax` | `?` |

## `_eneInit`

- `_eneInit`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1812b0 to 0x1815c0.
  Established. The `_sysOutByte` sites include unknown candidate zero paired with `0x21` (`call` at
  0x1812d1), `0x49` (`call` at 0x1812e6), `0x46` (`call` at 0x181363, `call` at 0x18138d), and
  `0xffffff80` (`call` at 0x181378). The `_eneGetAddr` call presents candidate zero `[sp+4]` (`call`
  at 0x1812f2). A byte store at an unknown effective address writes `0x47` (`mov` at 0x181545). No
  register or command-token meanings are assigned from these literals.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1812b7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1812ba | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1812c4 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x1812c9 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1812db | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1812ee | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1812fc | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181311 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181324 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x18132e | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181343 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181358 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18136d | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181382 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181397 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1813ac | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1813c1 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1813d1 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1813d4 | load | 1 | `[sp-8] +0x0` | `-` |
| `inc` at 0x1813da | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x1813da | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x1813dd | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1813f0 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1813f3 | load | 1 | `[sp-8] +0x0` | `-` |
| `inc` at 0x1813f9 | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x1813f9 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x1813fc | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18140f | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x181412 | load | 1 | `[sp-8] +0x0` | `-` |
| `inc` at 0x181418 | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x181418 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x18141b | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18142e | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x181431 | load | 1 | `[sp-8] +0x0` | `-` |
| `inc` at 0x181437 | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x181437 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x18143a | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18144d | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x181450 | load | 1 | `[sp-8] +0x0` | `-` |
| `inc` at 0x181456 | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x181456 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x181459 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18146c | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x18146f | load | 1 | `[sp-8] +0x0` | `-` |
| `mov` at 0x181475 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18148a | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18149f | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1814b4 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1814c9 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1814de | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1814f3 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181508 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18151d | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181532 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181545 | store | 1 | `(unknown)` | `0x47` |
| `mov` at 0x18154e | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181560 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181575 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18158a | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18159c | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1815af | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1812d1 | `_sysOutByte 0x1081d0` | `?` | `0x21` | `ebx` | `?` | `ebp` | `?` |
| `call` at 0x1812e6 | `_sysOutByte 0x1081d0` | `?` | `0x49` | `?` | `?` | `?` | `?` |
| `call` at 0x1812f2 | `_eneGetAddr 0x1815c0` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x181307 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x18131c | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x181339 | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x18134e | `_sysOutByte 0x1081d0` | `?` | `0x4` | `?` | `?` | `?` | `?` |
| `call` at 0x181363 | `_sysOutByte 0x1081d0` | `?` | `0x46` | `?` | `?` | `?` | `?` |
| `call` at 0x181378 | `_sysOutByte 0x1081d0` | `?` | `0xffffff80` | `?` | `?` | `?` | `?` |
| `call` at 0x18138d | `_sysOutByte 0x1081d0` | `?` | `0x46` | `?` | `?` | `?` | `?` |
| `call` at 0x1813a2 | `_sysOutByte 0x1081d0` | `?` | `0xffffffff` | `?` | `?` | `?` | `?` |
| `call` at 0x1813b7 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x1813c9 | `_sysOutByte 0x1081d0` | `?` | `0x41` | `?` | `?` | `?` | `?` |
| `call` at 0x1813e8 | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x181407 | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x181426 | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x181445 | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x181464 | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x181480 | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x181495 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x1814aa | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x1814bf | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x1814d4 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x1814e9 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x1814fe | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x181513 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x181528 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x18153d | `_sysOutByte 0x1081d0` | `?` | `0x47` | `?` | `?` | `?` | `?` |
| `call` at 0x181556 | `_sysOutByte 0x1081d0` | `?` | `0xffffffa1` | `?` | `?` | `?` | `?` |
| `call` at 0x18156b | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x181580 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x181592 | `_sysOutByte 0x1081d0` | `?` | `0x22` | `?` | `?` | `?` | `?` |
| `call` at 0x1815a7 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1815b5 | `eax` | `ret(_sysOutByte)` |

## `_eneintr`

- `_eneintr`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x180690 to 0x180a00.
  Established. The `_netJobAdd` sites present leading candidates `0x181a50`, `[sp+4]`, `[sp-8]:1`
  (`call` at 0x180828), `0x180a00`, `[sp+4]`, `0x0` (`call` at 0x1808f7), and `0x180e50`, `[sp+4]`,
  `0x0` (`call` at 0x1809ed). Those candidate addresses do not establish calls through them. Byte
  stores at `sp -0x5` (`mov` at 0x1806d7), `sp -0x6` (`mov` at 0x180707), and `sp -0x7` (`mov` at
  0x18071f) retain unknown values. Interrupt-status meanings remain unassigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x180697 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x18069a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1806a1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1806aa | store | 4 | `(unknown)` | `?` |
| `mov` at 0x1806b0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1806b6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1806c2 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1806d7 | store | 1 | `sp -0x5` | `?` |
| `movsx` at 0x1806da | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x1806df | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1806f2 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x180707 | store | 1 | `sp -0x6` | `?` |
| `mov` at 0x18070a | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18071f | store | 1 | `sp -0x7` | `?` |
| `mov` at 0x180722 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18073a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180742 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x180748 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x180760 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180768 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x18076e | load | 2 | `(unknown)` | `-` |
| `mov` at 0x180786 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x18078e | store | 4 | `(unknown)` | `?` |
| `mov` at 0x180794 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1807ac | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1807b4 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x1807ba | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x1807c3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1807cc | store | 4 | `(unknown)` | `?` |
| `mov` at 0x1807d2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1807d8 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1807ea | store | 1 | `sp -0x8` | `?` |
| `mov` at 0x1807ef | load | 2 | `(unknown)` | `-` |
| `mov` at 0x180804 | load | 2 | `(unknown)` | `-` |
| `movzx` at 0x18081a | load | 1 | `sp -0x8` | `-` |
| `mov` at 0x18081f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x180830 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x180839 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180842 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x180848 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x18084e | load | 1 | `sp -0x7` | `-` |
| `mov` at 0x180858 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180861 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x180867 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x18086d | load | 1 | `sp -0x7` | `-` |
| `mov` at 0x180876 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x18087f | store | 4 | `(unknown)` | `?` |
| `mov` at 0x180885 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x18088b | load | 1 | `sp -0x7` | `-` |
| `mov` at 0x180894 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x18089d | store | 4 | `(unknown)` | `?` |
| `mov` at 0x1808a3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1808a9 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x1808b2 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1808c0 | store | 1 | `(unknown)` | `?` |
| `mov` at 0x1808c6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1808d5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1808e0 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x1808ee | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1808ff | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x180908 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180911 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x180917 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x18091d | load | 1 | `sp -0x6` | `-` |
| `mov` at 0x180927 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180930 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x180936 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x18093c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180945 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x18094b | load | 1 | `sp -0x6` | `-` |
| `mov` at 0x180954 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x18095d | store | 4 | `(unknown)` | `?` |
| `mov` at 0x180963 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180969 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x180972 | load | 1 | `sp -0x6` | `-` |
| `mov` at 0x18097b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180984 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x18098a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180990 | load | 1 | `sp -0x6` | `-` |
| `mov` at 0x180999 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1809a2 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x1809a8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1809ae | load | 1 | `sp -0x6` | `-` |
| `mov` at 0x1809b7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1809c0 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x1809c6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1809cc | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x1809d5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1809e4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1809f5 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1806ba | `_sysBusIntAck 0x108808` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1806cd | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1806ea | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1806fd | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x180715 | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x18072d | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x180753 | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x180779 | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x18079f | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1807e0 | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1807fa | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x18080c | `_sysOutByte 0x1081d0` | `?` | `0x21` | `?` | `?` | `?` | `?` |
| `call` at 0x180828 | `_netJobAdd 0x1829f0` | `0x181a50` | `[sp+4]` | `[sp-8]:1` | `0x0` | `0x0` | `0x0` |
| `call` at 0x1808b6 | `_eneGetCurr 0x1819b0` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1808f7 | `_netJobAdd 0x1829f0` | `0x180a00` | `[sp+4]` | `0x0` | `0x0` | `0x0` | `0x0` |
| `call` at 0x1809ed | `_netJobAdd 0x1829f0` | `0x180e50` | `[sp+4]` | `0x0` | `0x0` | `0x0` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1807c1 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x180837 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x180856 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x180874 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x180892 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x1808b0 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `jne` at 0x1808d3 | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `je` at 0x180906 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x180925 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x180952 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x180970 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x180979 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x180997 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x1809b5 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x1809d3 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x1809da | `(? & ?) == 0x0` | `(? & ?) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1809fb | `eax` | `?` |

## `_eneIntEnable`

- `_eneIntEnable`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x181a20 to 0x181a50.
  Established. The `_sysOutByte` site has unknown candidate zero and candidate one `0x1f` (`call` at
  0x181a3b). The exit reports `ret(_sysOutByte)` in `eax` (`ret` at 0x181a49). The prior
  two-byte load has an unknown effective address (`mov` at 0x181a30). No port address or
  interrupt-mask role is inferred.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x181a24 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x181a27 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x181a30 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181a43 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x181a3b | `_sysOutByte 0x1081d0` | `?` | `0x1f` | `ebx` | `ebp` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x181a49 | `eax` | `ret(_sysOutByte)` |

## `_eneioctl`

- `_eneioctl`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x181190 to 0x1812b0.
  Established. The two equality branches are taken when `[sp+8] == 0x8020690c` (`je` at 0x1811cb)
  and `[sp+8] == 0x80206910` (`je` at 0x1811d2). The `_sysOutByte` site presents candidate zero
  `[[sp-12]+104]:2+12` and unknown candidate one (`call` at 0x181253). A four-byte store at `sp
  -0x10` writes `0x16` (`mov` at 0x181280). Ioctl-code meanings and a field role for that stored
  literal remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x181198 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x18119b | store | 4 | `sp -0x10` | `0x0` |
| `mov` at 0x1811a2 | load | 2 | `[sp+4] +0x4` | `-` |
| `mov` at 0x1811a9 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x1811ac | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1811af | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1811b6 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x1811c0 | store | 4 | `sp -0x14` | `ret(_splimp)` |
| `mov` at 0x1811c3 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1811e0 | load | 2 | `[sp+4] +0x4` | `-` |
| `mov` at 0x1811e7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1811f2 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x181201 | store | 4 | `sp -0x10` | `ret(_set_if_addr)` |
| `mov` at 0x181210 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x181213 | load | 2 | `[sp+4] +0x8` | `-` |
| `mov` at 0x181232 | store | 1 | `[sp-12] +0xe2` | `?` |
| `mov` at 0x181238 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x18123b | load | 1 | `[sp-12] +0xe2` | `-` |
| `mov` at 0x181245 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x181248 | load | 2 | `[sp-12] +0x68` | `-` |
| `mov` at 0x18125b | load | 2 | `[sp+4] +0x8` | `-` |
| `mov` at 0x18126a | load | 2 | `[sp+4] +0x4` | `-` |
| `mov` at 0x181280 | store | 4 | `sp -0x10` | `0x16` |
| `mov` at 0x181287 | load | 4 | `sp -0x14` | `-` |
| `mov` at 0x181293 | load | 4 | `sp -0x10` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1811b9 | `_splimp 0x195e8c` | `ebx` | `esi` | `?` | `0x0` | `?` | `?` |
| `call` at 0x1811f7 | `_set_if_addr 0x195cb8` | `[sp+4]` | `[sp+12]` | `?` | `?` | `?` | `ret(_splimp)` |
| `call` at 0x181253 | `_sysOutByte 0x1081d0` | `[[sp-12]+104]:2+12` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x181272 | `_eneinit 0x180660` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x18128b | `_splx 0x195ecc` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1811cb | `[sp+8] == 0x8020690c` | `[sp+8] != 0x8020690c` |
| `je` at 0x1811d2 | `[sp+8] == 0x80206910` | `[sp+8] != 0x80206910` |
| `je` at 0x181221 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x181268 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1812a8 | `eax` | `[sp-16]` |

## `_eneReset`

- `_eneReset`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x181120 to 0x181190.
  Established. The `_sysOutByte` site retains unknown candidate zero and candidate one `0x0` (`call`
  at 0x181148). The `_eneInit` (`call` at 0x181154) and `_eneIntEnable` (`call` at 0x181160) calls
  each present candidate zero `[sp+4]`. A four-byte store at `sp -0x8` retains `ret(_splnet)` (`mov`
  at 0x181138). The `_splx` site later presents `[sp-8]` (`call` at 0x181181). Reset and interrupt
  effects are not inferred from the names.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x181127 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x18112a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x181138 | store | 4 | `sp -0x8` | `ret(_splnet)` |
| `mov` at 0x18113d | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181150 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x18115c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x181168 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x181171 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x181177 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x18117d | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x181189 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x181131 | `_splnet 0x195e4c` | `ebx` | `?` | `ebp` | `?` | `?` | `?` |
| `call` at 0x181148 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `ret(_splnet)` | `?` | `?` |
| `call` at 0x181154 | `_eneInit 0x1812b0` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x181160 | `_eneIntEnable 0x181a20` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x181181 | `_splx 0x195ecc` | `[sp-8]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x18118f | `eax` | `ret(_splx)` |

## `_eneOverwriteRecover`

- `_eneOverwriteRecover`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x181a50 to 0x181d90.
  Established. The `_sysOutByte` sites include leading pairs `[sp-16]+10`, `0x0` (`call` at
  0x181ab2), `[sp-16]+11`, `0x0` (`call` at 0x181acd), and `[sp-16]+13`, `0x4` (`call` at 0x181b3a).
  The `_eneGet` call has candidate zero `[sp+4]` (`call` at 0x181b82). A byte store at an unknown
  effective address writes `[sp-16]:1` (`mov` at 0x181b72). These expressions do not identify a
  register base or overwrite-recovery action.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x181a58 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x181a5b | store | 1 | `sp -0x5` | `?` |
| `mov` at 0x181a5e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x181a61 | store | 4 | `sp -0x10` | `[sp+4]` |
| `mov` at 0x181a64 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x181a67 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x181a73 | store | 4 | `sp -0x10` | `ret(_sysClkRateGet)` |
| `mov` at 0x181a7b | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x181a85 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x181a8f | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x181a92 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x181aa1 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181aa8 | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x181aab | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x181abc | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181ac3 | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x181ac6 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x181ad5 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x181ada | store | 1 | `sp -0x10` | `?` |
| `cmp` at 0x181add | load | 1 | `sp -0x10` | `-` |
| `mov` at 0x181ae3 | store | 4 | `sp -0xc` | `0x0` |
| `mov` at 0x181af0 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181af7 | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x181afa | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x181b09 | store | 1 | `sp -0x10` | `?` |
| `mov` at 0x181b0c | load | 1 | `sp -0x10` | `-` |
| `mov` at 0x181b16 | store | 4 | `sp -0xc` | `0x1` |
| `mov` at 0x181b20 | store | 4 | `sp -0xc` | `0x0` |
| `mov` at 0x181b29 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181b30 | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x181b33 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x181b44 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181b4b | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x181b4e | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x181b5a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x181b5d | store | 4 | `sp -0x10` | `[sp+4]` |
| `mov` at 0x181b60 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x181b6c | store | 1 | `sp -0x10` | `?` |
| `mov` at 0x181b6f | load | 1 | `sp -0x10` | `-` |
| `mov` at 0x181b72 | store | 1 | `(unknown)` | `[sp-16]:1` |
| `mov` at 0x181b78 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x181b7b | store | 4 | `sp -0x10` | `[sp+4]` |
| `mov` at 0x181b7e | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x181b8a | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181b91 | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x181b94 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x181ba3 | store | 1 | `sp -0x10` | `?` |
| `movsx` at 0x181ba6 | load | 1 | `sp -0x10` | `-` |
| `mov` at 0x181bab | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181bb2 | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x181bb5 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x181bc6 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181bcd | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x181bd0 | load | 4 | `sp -0x10` | `-` |
| `cmp` at 0x181bdf | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x181be7 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181bee | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x181bf1 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x181c00 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x181c03 | store | 4 | `sp -0x10` | `?` |
| `cmp` at 0x181c06 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x181c0c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x181c0f | store | 4 | `sp -0x10` | `[sp+4]` |
| `mov` at 0x181c12 | load | 4 | `sp -0x10` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x181a6e | `_sysClkRateGet 0x1085a8` | `ebx` | `esi` | `[sp+4]` | `?` | `?` | `ebp` |
| `call` at 0x181a97 | `_taskDelay 0x170b80` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x181ab2 | `_sysOutByte 0x1081d0` | `[sp-16]+10` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x181acd | `_sysOutByte 0x1081d0` | `[sp-16]+11` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x181b01 | `_sysInByte 0x1081b0` | `[sp-16]+7` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x181b3a | `_sysOutByte 0x1081d0` | `[sp-16]+13` | `0x4` | `?` | `?` | `?` | `?` |
| `call` at 0x181b52 | `_sysOutByte 0x1081d0` | `[sp-16]` | `0x22` | `?` | `?` | `?` | `?` |
| `call` at 0x181b64 | `_eneGetCurr 0x1819b0` | `[sp+4]` | `?` | `?` | `[sp+4]` | `?` | `?` |
| `call` at 0x181b82 | `_eneGet 0x180a00` | `[sp+4]` | `?` | `?` | `[sp+4]` | `?` | `?` |
| `call` at 0x181b9b | `_sysInByte 0x1081b0` | `[sp-16]+7` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x181bbc | `_sysOutByte 0x1081d0` | `[sp-16]+7` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x181bd7 | `_sysOutByte 0x1081d0` | `[sp-16]+13` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x181bf5 | `_sysOutByte 0x1081d0` | `[sp-16]` | `0x4` | `?` | `?` | `?` | `?` |
| `call` at 0x181c16 | `_enePut 0x180e50` | `[sp+4]` | `?` | `?` | `[sp+4]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x181ae1 | `([sp-16]:1 & 0xff) != 0x0` | `([sp-16]:1 & 0xff) == 0x0` |
| `jne` at 0x181b14 | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `je` at 0x181be3 | `[sp-12] == 0x0` | `[sp-12] != 0x0` |
| `je` at 0x181c0a | `[sp-16] == 0x0` | `[sp-16] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x181c26 | `eax` | `?` |

## `_eneGetCurr`

- `_eneGetCurr`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1819b0 to 0x181a20.
  Established. The first `_sysOutByte` site has unknown candidate zero and candidate one `0x40`
  (`call` at 0x1819cb). A byte store at `sp -0x5` retains an unknown value (`mov` at 0x1819e8). The
  later `_sysOutByte` site pairs unknown candidate zero with `0x0` (`call` at 0x1819f5). The exit
  reports `[sp-5]:1` (`ret` at 0x181a16). The returned byte is not assigned a ring-pointer meaning.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1819b7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1819ba | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1819c3 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1819d3 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1819e8 | store | 1 | `sp -0x5` | `?` |
| `mov` at 0x1819ed | load | 2 | `(unknown)` | `-` |
| `movzx` at 0x1819fd | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x181a10 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1819cb | `_sysOutByte 0x1081d0` | `?` | `0x40` | `ebx` | `?` | `ebp` | `?` |
| `call` at 0x1819de | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1819f5 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x181a16 | `eax` | `[sp-5]:1` |

## `_eneGet`

- `_eneGet`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x180a00 to 0x180e00.
  Established. The first `_eneDataIn` site presents `[sp+4]`, unknown, `0x4`, and `sp-24` as its
  leading candidates (`call` at 0x180a6e). Byte stores at absolute `0x1d9318` have unknown values
  (`mov` at 0x180a8c, `mov` at 0x180aaf). Another byte store at that address writes `[0x1d9318]:1-2`
  (`add` at 0x180abb). The indirect call has an unknown callee, unknown first two candidates, and
  candidate two `[sp-12]` (`call` at 0x180cd3). No packet length or callback role is inferred.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x180a09 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x180a0c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180a30 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x180a36 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x180a56 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x180a6a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x180a76 | load | 1 | `(unknown)` | `-` |
| `cmp` at 0x180a7c | load | 1 | `sp -0x17` | `-` |
| `mov` at 0x180a81 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x180a87 | load | 1 | `sp -0x17` | `-` |
| `mov` at 0x180a8c | store | 1 | `abs 0x1d9318` | `?` |
| `mov` at 0x180aa0 | load | 1 | `sp -0x17` | `-` |
| `mov` at 0x180aa5 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x180aaf | store | 1 | `abs 0x1d9318` | `?` |
| `cmp` at 0x180ab5 | load | 1 | `sp -0x16` | `-` |
| `add` at 0x180abb | load | 1 | `abs 0x1d9318` | `-` |
| `add` at 0x180abb | store | 1 | `abs 0x1d9318` | `[0x1d9318]:1-2` |
| `dec` at 0x180ad0 | load | 1 | `abs 0x1d9318` | `-` |
| `dec` at 0x180ad0 | store | 1 | `abs 0x1d9318` | `?` |
| `mov` at 0x180ad6 | load | 1 | `abs 0x1d9318` | `-` |
| `mov` at 0x180adc | store | 1 | `sp -0x15` | `[0x1d9318]:1` |
| `movzx` at 0x180adf | load | 1 | `sp -0x15` | `-` |
| `movzx` at 0x180ae8 | load | 1 | `sp -0x16` | `-` |
| `mov` at 0x180af1 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x180af4 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x180b01 | store | 1 | `sp -0x12` | `?` |
| `mov` at 0x180b04 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x180b0a | load | 1 | `sp -0x12` | `-` |
| `mov` at 0x180b0f | store | 1 | `sp -0x11` | `?` |
| `cmp` at 0x180b12 | load | 1 | `sp -0x11` | `-` |
| `add` at 0x180b18 | load | 1 | `sp -0x11` | `-` |
| `add` at 0x180b18 | store | 1 | `sp -0x11` | `[sp-17]:1-58` |
| `mov` at 0x180b1c | load | 1 | `sp -0x17` | `-` |
| `cmp` at 0x180b1f | load | 1 | `sp -0x11` | `-` |
| `cmp` at 0x180b24 | load | 1 | `sp -0x17` | `-` |
| `cmp` at 0x180b2a | load | 1 | `sp -0x17` | `-` |
| `mov` at 0x180b40 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180b49 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x180b4f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180b55 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180b5e | store | 4 | `(unknown)` | `?` |
| `mov` at 0x180b64 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180b6a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x180b76 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180b81 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x180b87 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x180ba0 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x180ba6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180baf | store | 4 | `(unknown)` | `?` |
| `mov` at 0x180bb5 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x180bc0 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x180bd0 | load | 1 | `sp -0x18` | `-` |
| `mov` at 0x180bd9 | load | 1 | `sp -0x18` | `-` |
| `mov` at 0x180be2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180beb | store | 4 | `(unknown)` | `?` |
| `mov` at 0x180bf1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180bf7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180c00 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x180c06 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180c20 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180c29 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x180c2f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180c40 | load | 1 | `sp -0x18` | `-` |
| `mov` at 0x180c49 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180c52 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x180c58 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180c5e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180c67 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x180c6d | load | 4 | `(unknown)` | `-` |
| `movzx` at 0x180c87 | load | 2 | `sp -0xc` | `-` |
| `mov` at 0x180c8c | load | 1 | `(unknown)` | `-` |
| `mov` at 0x180ca6 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x180cb8 | load | 4 | `abs 0x1d10fc` | `-` |
| `mov` at 0x180cc1 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x180cc7 | load | 4 | `abs 0x1d10fc` | `-` |
| `mov` at 0x180ccd | store | 4 | `sp -0x1c` | `[0x1d10fc]` |
| `mov` at 0x180cd0 | load | 4 | `sp -0x1c` | `-` |
| `cmp` at 0x180cf0 | load | 4 | `sp -0xc` | `-` |
| `add` at 0x180cf6 | load | 4 | `sp -0xc` | `-` |
| `add` at 0x180cf6 | store | 4 | `sp -0xc` | `[sp-12]-14` |
| `mov` at 0x180d13 | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x180d1f | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x180d2f | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x180d33 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x180d37 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x180d50 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x180d56 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x180d66 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180d6c | store | 4 | `(unknown)` | `?` |
| `mov` at 0x180d6f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180d72 | load | 1 | `sp -0x17` | `-` |
| `mov` at 0x180d75 | store | 1 | `(unknown)` | `[sp-23]:1` |
| `mov` at 0x180d7b | load | 1 | `(unknown)` | `-` |
| `mov` at 0x180d85 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x180da6 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x180dc0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180dcb | store | 4 | `(unknown)` | `?` |
| `mov` at 0x180dd1 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x180dd7 | load | 1 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x180a6e | `_eneDataIn 0x181870` | `[sp+4]` | `?` | `0x4` | `sp-24` | `?` | `?` |
| `call` at 0x180b6e | `_eneInit 0x1812b0` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x180b8b | `_eneIntEnable 0x181a20` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x180caa | `_eneDataIn 0x181870` | `[sp+4]` | `?` | `[sp-12]:2` | `?` | `?` | `?` |
| `call` at 0x180cd3 | `(unknown callee) (unknown target)` | `?` | `?` | `[sp-12]` | `?` | `?` | `?` |
| `call` at 0x180d24 | `_check_trailer 0x1953f8` | `?` | `[sp-16]` | `sp-12` | `sp-8` | `?` | `?` |
| `call` at 0x180d3b | `_bcopy_to_mbufs 0x1954c8` | `[sp-16]` | `[sp-12]` | `[sp-8]` | `?` | `0x2` | `?` |
| `call` at 0x180d5e | `_do_protocol_with_type 0x195928` | `?` | `ret(_bcopy_to_mbufs)` | `?` | `[sp-12]` | `?` | `?` |
| `call` at 0x180db1 | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x180a3e | `? != ?` | `? == ?` |
| `jbe` at 0x180a7f | `([sp-23]:1 & 0xff) <= ? (unsigned, 8-bit)` | `([sp-23]:1 & 0xff) > ? (unsigned, 8-bit)` |
| `jbe` at 0x180ab9 | `([sp-22]:1 & 0xff) <= 0xfc (unsigned, 8-bit)` | `([sp-22]:1 & 0xff) > 0xfc (unsigned, 8-bit)` |
| `jbe` at 0x180b16 | `([sp-17]:1 & 0xff) <= 0x7f (unsigned, 8-bit)` | `([sp-17]:1 & 0xff) > 0x7f (unsigned, 8-bit)` |
| `jne` at 0x180b22 | `([sp-17]:1 & 0xff) != ([sp-23]:1 & 0xff)` | `([sp-17]:1 & 0xff) == ([sp-23]:1 & 0xff)` |
| `jbe` at 0x180b28 | `([sp-23]:1 & 0xff) <= 0x45 (unsigned, 8-bit)` | `([sp-23]:1 & 0xff) > 0x45 (unsigned, 8-bit)` |
| `ja` at 0x180b2e | `([sp-23]:1 & 0xff) > 0x7f (unsigned, 8-bit)` | `([sp-23]:1 & 0xff) <= 0x7f (unsigned, 8-bit)` |
| `ja` at 0x180ba4 | `[sp-12] > 0x3b (unsigned, 32-bit)` | `[sp-12] <= 0x3b (unsigned, 32-bit)` |
| `jbe` at 0x180bc7 | `[sp-12] <= 0x5ea (unsigned, 32-bit)` | `[sp-12] > 0x5ea (unsigned, 32-bit)` |
| `je` at 0x180bd7 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x180be0 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x180c47 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x180cbf | `[0x1d10fc] == 0x0` | `[0x1d10fc] != 0x0` |
| `je` at 0x180cdc | `(ret(unknown) & ret(unknown)) == 0x0` | `(ret(unknown) & ret(unknown)) != 0x0` |
| `jbe` at 0x180cf4 | `[sp-12] <= 0xd (unsigned, 32-bit)` | `[sp-12] > 0xd (unsigned, 32-bit)` |
| `jne` at 0x180d47 | `(ret(_bcopy_to_mbufs) & ret(_bcopy_to_mbufs)) != 0x0` | `(ret(_bcopy_to_mbufs) & ret(_bcopy_to_mbufs)) == 0x0` |
| `je` at 0x180d83 | `? == 0x46` | `? != 0x46` |
| `je` at 0x180ddf | `? == ?` | `? != ?` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x180df9 | `eax` | `?` |

## `_enePut`

- `_enePut`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x180e50 to 0x181060.
  Established. A four-byte store at `sp -0x8` writes `0x2710` (`mov` at 0x180e78). The branch is
  taken when `[sp-8] != 0x0` (`jne` at 0x180ed4). The `_eneDataOut` site presents `[sp+4]`, unknown,
  unknown, and `0x4000` as its first four candidates (`call` at 0x180fc4). The indirect call retains
  unknown callee and candidate slots (`call` at 0x180f9f). No transmit extent or polling duration is
  assigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x180e59 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x180e5c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180e6a | store | 4 | `sp -0xc` | `ret(_splnet)` |
| `mov` at 0x180e6d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180e78 | store | 4 | `sp -0x8` | `0x2710` |
| `mov` at 0x180e7f | load | 2 | `(unknown)` | `-` |
| `dec` at 0x180e9a | load | 4 | `sp -0x8` | `-` |
| `dec` at 0x180e9a | store | 4 | `sp -0x8` | `?` |
| `cmp` at 0x180e9d | load | 4 | `sp -0x8` | `-` |
| `cmp` at 0x180ed0 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x180ed6 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x180ee2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180eec | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180eef | store | 4 | `(unknown)` | `?` |
| `mov` at 0x180ef1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180ef7 | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x180efe | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x180f08 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180f0e | store | 4 | `sp -0x1c` | `?` |
| `mov` at 0x180f11 | load | 4 | `sp -0x1c` | `-` |
| `mov` at 0x180f14 | store | 4 | `(unknown)` | `[sp-28]` |
| `mov` at 0x180f17 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180f20 | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x180f23 | store | 4 | `sp -0x14` | `?` |
| `mov` at 0x180f30 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x180f38 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x180f3c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180f4b | load | 2 | `(unknown)` | `-` |
| `add` at 0x180f52 | load | 4 | `sp -0x10` | `-` |
| `add` at 0x180f52 | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x180f55 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x180f60 | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x180f6b | load | 4 | `sp -0x14` | `-` |
| `cmp` at 0x180f81 | load | 4 | `abs 0x1d1100` | `-` |
| `mov` at 0x180f93 | load | 4 | `abs 0x1d1100` | `-` |
| `mov` at 0x180f99 | store | 4 | `sp -0x18` | `[0x1d1100]` |
| `mov` at 0x180f9c | load | 4 | `sp -0x18` | `-` |
| `mov` at 0x180fc0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x180fce | load | 2 | `(unknown)` | `-` |
| `mov` at 0x180fe3 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x180fff | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181018 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18102d | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18103d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x181049 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x180e63 | `_splnet 0x195e4c` | `ebx` | `esi` | `edi` | `?` | `?` | `?` |
| `call` at 0x180e87 | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x180eda | `_eneInit 0x1812b0` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x180f43 | `_bcopy 0x1747fc` | `?` | `[sp-16]` | `?` | `?` | `?` | `?` |
| `call` at 0x180f6f | `_m_freem 0x196890` | `[sp-20]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x180f9f | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x180fc4 | `_eneDataOut 0x181710` | `[sp+4]` | `?` | `?` | `0x4000` | `?` | `?` |
| `call` at 0x180fd9 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x180fee | `_sysOutByte 0x1081d0` | `?` | `0x40` | `?` | `?` | `?` | `?` |
| `call` at 0x18100a | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x181023 | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x181035 | `_sysOutByte 0x1081d0` | `?` | `0x4` | `?` | `?` | `?` | `?` |
| `call` at 0x181041 | `_eneIntEnable 0x181a20` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x18104d | `_splx 0x195ecc` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x180e72 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x180e98 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `jg` at 0x180ea1 | `[sp-8] > 0x0 (signed, 32-bit)` | `[sp-8] <= 0x0 (signed, 32-bit)` |
| `jne` at 0x180ed4 | `[sp-8] != 0x0` | `[sp-8] == 0x0` |
| `je` at 0x180ee7 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `jne` at 0x180ef5 | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `jne` at 0x180f28 | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `jge` at 0x180f7a | `? >= 0x3c (signed, 32-bit)` | `? < 0x3c (signed, 32-bit)` |
| `je` at 0x180f88 | `[0x1d1100] == 0x0` | `[0x1d1100] != 0x0` |
| `je` at 0x180fa8 | `(ret(unknown) & ret(unknown)) == 0x0` | `(ret(unknown) & ret(unknown)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x18105e | `eax` | `ret(_splx)` |

## `_eneDataIn`

- `_eneDataIn`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x181870 to 0x1819b0.
  Established. The `_sysOutByte` sites include unknown candidate zero paired with `0x0` (`call` at
  0x18189e), `0x22` (`call` at 0x1818b0), and `0xa` (`call` at 0x181930). The `_sysInWordString`
  site presents unknown candidate zero, candidate one `[sp+16]`, and unknown candidate two (`call`
  at 0x181955). The byte store at `([sp-8]:2 + [sp+16]) -0x1` retains an unknown value (`mov` at
  0x18198d). No transfer width or count is inferred from the callee name.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x181879 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x18187c | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x18187f | store | 2 | `sp -0x6` | `?` |
| `mov` at 0x181883 | store | 2 | `sp -0x8` | `?` |
| `mov` at 0x181887 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x18188a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x181893 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1818a8 | load | 2 | `(unknown)` | `-` |
| `movsx` at 0x1818b8 | load | 1 | `sp -0x6` | `-` |
| `mov` at 0x1818bd | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1818d0 | load | 2 | `sp -0x6` | `-` |
| `mov` at 0x1818dc | load | 2 | `(unknown)` | `-` |
| `movsx` at 0x1818ef | load | 1 | `sp -0x8` | `-` |
| `mov` at 0x1818f4 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181907 | load | 2 | `sp -0x8` | `-` |
| `mov` at 0x181913 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181928 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181938 | load | 2 | `sp -0x8` | `-` |
| `mov` at 0x181946 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x18194a | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18195d | load | 2 | `sp -0x8` | `-` |
| `mov` at 0x18196a | load | 2 | `(unknown)` | `-` |
| `movzx` at 0x18197f | load | 2 | `sp -0x8` | `-` |
| `add` at 0x181985 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x18198d | store | 1 | `([sp-8]:2 + [sp+16]) -0x1` | `?` |
| `mov` at 0x18198f | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x18189e | `_sysOutByte 0x1081d0` | `?` | `0x0` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x1818b0 | `_sysOutByte 0x1081d0` | `?` | `0x22` | `?` | `?` | `?` | `?` |
| `call` at 0x1818c8 | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1818e7 | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1818ff | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x18191e | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x181930 | `_sysOutByte 0x1081d0` | `?` | `0xa` | `?` | `?` | `?` | `?` |
| `call` at 0x181955 | `_sysInWordString 0x1081f0` | `?` | `[sp+16]` | `?` | `?` | `?` | `?` |
| `call` at 0x181975 | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x181993 | `_eneIntEnable 0x181a20` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x181968 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1819a4 | `eax` | `ret(_eneIntEnable)` |

## `_eneDataOut`

- `_eneDataOut`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x181710 to 0x181870.
  Established. The initial branch is taken when `([sp-6]:2 & 0xffff) != 0x0` (`jne` at 0x181736).
  The `_sysOutByte` sites include unknown candidate zero paired with `0xffffffff` (`call` at
  0x181762), `0x22` (`call` at 0x181774), and `0x12` (`call` at 0x1817f4). The `_sysOutWordString`
  call has unknown candidate zero, candidate one `[sp+8]`, and unknown candidate two (`call` at
  0x181819). Transfer size and hardware-command semantics remain unassigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x181719 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x18171c | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x18171f | store | 2 | `sp -0x6` | `?` |
| `mov` at 0x181723 | store | 2 | `sp -0x8` | `?` |
| `mov` at 0x181727 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x18172a | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x181731 | load | 2 | `sp -0x6` | `-` |
| `mov` at 0x181742 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181757 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18176c | load | 2 | `(unknown)` | `-` |
| `movsx` at 0x18177c | load | 1 | `sp -0x8` | `-` |
| `mov` at 0x181781 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181794 | load | 2 | `sp -0x8` | `-` |
| `mov` at 0x1817a0 | load | 2 | `(unknown)` | `-` |
| `movsx` at 0x1817b3 | load | 1 | `sp -0x6` | `-` |
| `mov` at 0x1817b8 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1817cb | load | 2 | `sp -0x6` | `-` |
| `mov` at 0x1817d7 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1817ec | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1817fc | load | 2 | `sp -0x6` | `-` |
| `mov` at 0x18180a | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x18180e | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181821 | load | 2 | `sp -0x6` | `-` |
| `movzx` at 0x18182e | load | 2 | `sp -0x6` | `-` |
| `add` at 0x181834 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x18183c | load | 1 | `([sp-6]:2 + [sp+8]) -0x1` | `-` |
| `mov` at 0x181842 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181855 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x18174d | `_sysOutByte 0x1081d0` | `?` | `0x0` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x181762 | `_sysOutByte 0x1081d0` | `?` | `0xffffffff` | `?` | `?` | `?` | `?` |
| `call` at 0x181774 | `_sysOutByte 0x1081d0` | `?` | `0x22` | `?` | `?` | `?` | `?` |
| `call` at 0x18178c | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1817ab | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1817c3 | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1817e2 | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1817f4 | `_sysOutByte 0x1081d0` | `?` | `0x12` | `?` | `?` | `?` | `?` |
| `call` at 0x181819 | `_sysOutWordString 0x108210` | `?` | `[sp+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x18184d | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x181859 | `_eneIntEnable 0x181a20` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x181736 | `([sp-6]:2 & 0xffff) != 0x0` | `([sp-6]:2 & 0xffff) == 0x0` |
| `je` at 0x18182c | `(? & ?) == 0x0` | `(? & ?) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x18186a | `eax` | `?` |

## `_eneGetAddr`

- `_eneGetAddr`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1815c0 to 0x181710.
  Established. The `_sysOutByte` sites include unknown candidate zero paired with `0x22` (`call` at
  0x1815e1), `0xc` (`call` at 0x181620), and `0xa` (`call` at 0x181647). Byte stores through `[sp-8]
  +0x0` retain unknown values (`mov` at 0x181667, `mov` at 0x181684, `mov` at 0x1816a1, `mov` at
  0x1816be, `mov` at 0x1816db, `mov` at 0x1816f8). The six stores do not establish an address-record
  format or port location.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1815c7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1815ca | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1815d4 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x1815d9 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1815eb | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181600 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181615 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18162a | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18163f | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18164f | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181664 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x181667 | store | 1 | `[sp-8] +0x0` | `?` |
| `inc` at 0x181669 | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x181669 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x18166c | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181681 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x181684 | store | 1 | `[sp-8] +0x0` | `?` |
| `inc` at 0x181686 | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x181686 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x181689 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x18169e | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1816a1 | store | 1 | `[sp-8] +0x0` | `?` |
| `inc` at 0x1816a3 | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x1816a3 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x1816a6 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1816bb | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1816be | store | 1 | `[sp-8] +0x0` | `?` |
| `inc` at 0x1816c0 | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x1816c0 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x1816c3 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1816d8 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1816db | store | 1 | `[sp-8] +0x0` | `?` |
| `inc` at 0x1816dd | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x1816dd | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x1816e0 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x1816f5 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1816f8 | store | 1 | `[sp-8] +0x0` | `?` |
| `mov` at 0x1816fa | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1815e1 | `_sysOutByte 0x1081d0` | `?` | `0x22` | `ebx` | `?` | `ebp` | `?` |
| `call` at 0x1815f6 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x18160b | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x181620 | `_sysOutByte 0x1081d0` | `?` | `0xc` | `?` | `?` | `?` | `?` |
| `call` at 0x181635 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x181647 | `_sysOutByte 0x1081d0` | `?` | `0xa` | `?` | `?` | `?` | `?` |
| `call` at 0x18165a | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x181677 | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x181694 | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1816b1 | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1816ce | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1816eb | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x181700 | `eax` | `?` |

## `_enedetach`

- `_enedetach`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x181e80 to 0x181f40.
  Established. The `_sysOutByte` sites pair unknown candidate zero with `0x0` (`call` at 0x181ea6,
  `call` at 0x181efd), `0x21` (`call` at 0x181eb8), and `0x4` (`call` at 0x181f12). The
  `_sysIntDisablePIC` site retains unknown candidates (`call` at 0x181f1e), while `_eneBoardReset`
  presents candidate zero `[sp+4]` (`call` at 0x181f2a). A four-byte store at an unknown effective
  address writes `0x0` (`mov` at 0x181f32). A detached or disabled device outcome remains unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x181e84 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x181e87 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x181e8e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x181e9b | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181eb0 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181ec0 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181ef2 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181f07 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x181f1a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x181f26 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x181f32 | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x181f39 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x181ea6 | `_sysOutByte 0x1081d0` | `?` | `0x0` | `ebx` | `ebp` | `?` | `?` |
| `call` at 0x181eb8 | `_sysOutByte 0x1081d0` | `?` | `0x21` | `?` | `?` | `?` | `?` |
| `call` at 0x181ecb | `_sysInByte 0x1081b0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x181efd | `_sysOutByte 0x1081d0` | `?` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x181f12 | `_sysOutByte 0x1081d0` | `?` | `0x4` | `?` | `?` | `?` | `?` |
| `call` at 0x181f1e | `_sysIntDisablePIC 0x1083b8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x181f2a | `_eneBoardReset 0x181060` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x181e93 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x181edc | `(? & ?) == 0x0` | `(? & ?) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x181f3f | `eax` | `?` |
