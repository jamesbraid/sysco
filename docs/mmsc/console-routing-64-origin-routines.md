# MMSC console routing callers in the 6.4-origin image

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

[Fact sheets and complete listings](disasm/console-routing-64-origin-facts.txt) include all tool sections.

The linear decoder stops before the ends of three spans. Raw literal tails are
`22 25 73 22 00` at `0x11a46b` to `0x11a470` in `_cmSendFFSCCmd` and
`00 90 90 90` at `0x11dfdc` to `0x11dfe0` in `_cuUserInConsole`.
For `_crProcessSTATECmd`, decoding stops at `0x11ca34` inside the table;
its remaining table bytes through `0x11ca4c` are
`f0 ca 11 00 20 cb 11 00 40 cb 11 00 50 cb 11 00 60 cb 11 00 70 cb 11 00`.
An independent aligned analysis from `0x11ca4c` to `0x11cc60` follows the entry
analyses in the retained facts. It decodes the remaining bytes with fresh entry
state; its accesses, calls and returns do not establish caller state.
Known targets beyond the main decoding stop are omitted from the entry effects,
even though its unresolved-transfer list is empty. Those effects are incomplete.

## `_ceEnterConsoleMode`

- `_ceEnterConsoleMode`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1183b8 to 0x1185f8.
  Established. A branch takes the main assignment arm when `[0x1d9b9c] == 0x0` (`je` at 0x11841f).
  That arm writes four-byte `0x1` values at `[sp+4] +0x18`, `+0x1c`, `+0x24` (`mov` at 0x118438,
  `mov` at 0x11843f, `mov` at 0x118446). A `_bufAttachLog` site carries candidates `[[sp+4]+116]`,
  `[[sp+4]+128]` (`call` at 0x1184b0). The global assignment stores `[sp+4]` at absolute `0x1d9b9c`
  (`mov` at 0x1184c1). Stores at `[sp+4] +0x3c` have unknown values (`mov` at 0x118426, `mov`
  at 0x118503, `mov` at 0x118540, `mov` at 0x118551). Console mode semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1183c0 | load | 4 | `sp +0x4` | `-` |
| `test` at 0x1183c3 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x1183da | load | 4 | `abs 0x1d9b9c` | `-` |
| `mov` at 0x1183e0 | load | 4 | `[0x1d9b9c] +0x18` | `-` |
| `mov` at 0x1183ec | load | 4 | `[0x1d9b9c] +0x84` | `-` |
| `cmp` at 0x118418 | load | 4 | `abs 0x1d9b9c` | `-` |
| `mov` at 0x118421 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x118426 | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x118438 | store | 4 | `[sp+4] +0x18` | `0x1` |
| `mov` at 0x11843f | store | 4 | `[sp+4] +0x1c` | `0x1` |
| `mov` at 0x118446 | store | 4 | `[sp+4] +0x24` | `0x1` |
| `mov` at 0x11844d | load | 4 | `[sp+4] +0x70` | `-` |
| `mov` at 0x118459 | load | 4 | `[sp+4] +0x74` | `-` |
| `mov` at 0x118465 | load | 4 | `[sp+4] +0x4c` | `-` |
| `mov` at 0x118469 | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x118475 | load | 4 | `[sp+4] +0x4c` | `-` |
| `mov` at 0x118479 | load | 4 | `[sp+4] +0x6c` | `-` |
| `mov` at 0x118485 | load | 4 | `[sp+4] +0x50` | `-` |
| `mov` at 0x118489 | load | 4 | `[sp+4] +0x70` | `-` |
| `mov` at 0x118495 | load | 4 | `[sp+4] +0x50` | `-` |
| `mov` at 0x118499 | load | 4 | `[sp+4] +0x74` | `-` |
| `mov` at 0x1184a5 | load | 4 | `[sp+4] +0x80` | `-` |
| `mov` at 0x1184ac | load | 4 | `[sp+4] +0x74` | `-` |
| `mov` at 0x1184c1 | store | 4 | `abs 0x1d9b9c` | `[sp+4]` |
| `mov` at 0x1184c7 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x1184cf | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1184f1 | load | 4 | `[sp+4] +0x74` | `-` |
| `mov` at 0x1184fd | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x118503 | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x118506 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11852c | load | 4 | `[sp+4] +0x74` | `-` |
| `mov` at 0x118538 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x118540 | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x118543 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11854b | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x118551 | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x118554 | load | 4 | `[sp+4] +0x68` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1183d2 | `_ffscMsg 0x11ebd0` | `0x11833c` | `[sp+4]` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x1183f3 | `_userChangeInputMode 0x12aa48` | `[[0x1d9b9c]+132]` | `0x8` | `0x0` | `?` | `?` | `?` |
| `call` at 0x118404 | `_ffscMsg 0x11ebd0` | `0x11835d` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11842a | `_ceEnterFFSCMode 0x1188a8` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x118451 | `_bufClear 0x10f438` | `[[sp+4]+112]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11845d | `_bufClear 0x10f438` | `[[sp+4]+116]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11846d | `_bufAttachFD 0x10f378` | `[[sp+4]+104]` | `[[sp+4]+76]` | `?` | `?` | `?` | `?` |
| `call` at 0x11847d | `_bufAttachFD 0x10f378` | `[[sp+4]+108]` | `[[sp+4]+76]` | `?` | `?` | `?` | `?` |
| `call` at 0x11848d | `_bufAttachFD 0x10f378` | `[[sp+4]+112]` | `[[sp+4]+80]` | `?` | `?` | `?` | `?` |
| `call` at 0x11849d | `_bufAttachFD 0x10f378` | `[[sp+4]+116]` | `[[sp+4]+80]` | `?` | `?` | `?` | `?` |
| `call` at 0x1184b0 | `_bufAttachLog 0x10f418` | `[[sp+4]+116]` | `[[sp+4]+128]` | `?` | `?` | `?` | `?` |
| `call` at 0x1184b9 | `_cmCheckTTYFlags 0x119c20` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1184de | `_sprintf 0x171680` | `sp-20` | `0x11837d` | `[0x1ee564]` | `?` | `?` | `?` |
| `call` at 0x1184e7 | `_strlen 0x172f14` | `sp-20` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1184f5 | `_bufWrite 0x10fb78` | `[[sp+4]+116]` | `sp-20` | `ret(_strlen)` | `?` | `?` | `?` |
| `call` at 0x118519 | `_sprintf 0x171680` | `sp-100` | `0x118389` | `?` | `?` | `?` | `?` |
| `call` at 0x118522 | `_strlen 0x172f14` | `sp-100` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x118530 | `_bufWrite 0x10fb78` | `[[sp+4]+116]` | `sp-100` | `ret(_strlen)` | `?` | `?` | `?` |
| `call` at 0x118558 | `_bufRelease 0x10fa18` | `[[sp+4]+104]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1183ca | `(([0x1da698]:1 & 0xff) & 0x4) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x4) != 0x0` |
| `jne` at 0x1183e6 | `[[0x1d9b9c]+24] != 0x9` | `[[0x1d9b9c]+24] == 0x9` |
| `je` at 0x1183fd | `(ret(_userChangeInputMode) & ret(_userChangeInputMode)) == 0x0` | `(ret(_userChangeInputMode) & ret(_userChangeInputMode)) != 0x0` |
| `je` at 0x11841f | `[0x1d9b9c] == 0x0` | `[0x1d9b9c] != 0x0` |
| `je` at 0x1184cd | `(? & 0x40) == 0x0` | `(? & 0x40) != 0x0` |
| `je` at 0x11850e | `([[sp+4]+60] & 0x100000) == 0x0` | `([[sp+4]+60] & 0x100000) != 0x0` |
| `jge` at 0x118549 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x118567 | `eax` | `?` |

## `_clLeaveConsoleMode`

- `_clLeaveConsoleMode`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x119478 to 0x119598.
  Established. The call sequence includes `_bufHold` with candidate zero `[[sp+4]+104]` (`call` at
  0x119483) and `_bufDetachLog` with `[[sp+4]+116]` (`call` at 0x11950c). Four-byte stores clear
  `[sp+4] +0x18`, `+0x1c`, `+0x20`, `+0x24`, `+0x28` (`mov` at 0x119514, `mov` at 0x11951b, `mov` at
  0x119522, `mov` at 0x119529, `mov` at 0x119530). The branch falls through when `[0x1d9b9c] == [sp+4]` (`jne` at 0x11953d),
  reaching a zero assignment at absolute `0x1d9b9c` (`mov` at 0x11953f). The exit reports zero
  (`ret` at 0x119565). The field meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x11947c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x11947f | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x11948b | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x119492 | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x11949e | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x1194a3 | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x1194a6 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x1194ad | load | 4 | `[sp+4] +0x70` | `-` |
| `mov` at 0x1194b9 | load | 4 | `[sp+4] +0x6c` | `-` |
| `mov` at 0x1194c5 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x1194ca | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x1194cd | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x1194d5 | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x1194d8 | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x1194e4 | load | 4 | `[sp+4] +0x6c` | `-` |
| `mov` at 0x1194f0 | load | 4 | `[sp+4] +0x70` | `-` |
| `mov` at 0x1194fc | load | 4 | `[sp+4] +0x74` | `-` |
| `mov` at 0x119508 | load | 4 | `[sp+4] +0x74` | `-` |
| `mov` at 0x119514 | store | 4 | `[sp+4] +0x18` | `0x0` |
| `mov` at 0x11951b | store | 4 | `[sp+4] +0x1c` | `0x0` |
| `mov` at 0x119522 | store | 4 | `[sp+4] +0x20` | `0x0` |
| `mov` at 0x119529 | store | 4 | `[sp+4] +0x24` | `0x0` |
| `mov` at 0x119530 | store | 4 | `[sp+4] +0x28` | `0x0` |
| `cmp` at 0x119537 | load | 4 | `abs 0x1d9b9c` | `-` |
| `mov` at 0x11953f | store | 4 | `abs 0x1d9b9c` | `0x0` |
| `test` at 0x119549 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x11955f | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x119483 | `_bufHold 0x10f678` | `[[sp+4]+104]` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x119496 | `_bufRelease 0x10fa18` | `[[sp+4]+104]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1194b1 | `_bufRelease 0x10fa18` | `[[sp+4]+112]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1194bd | `_bufRelease 0x10fa18` | `[[sp+4]+108]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1194dc | `_bufDetachFD 0x10f478` | `[[sp+4]+104]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1194e8 | `_bufDetachFD 0x10f478` | `[[sp+4]+108]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1194f4 | `_bufDetachFD 0x10f478` | `[[sp+4]+112]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x119500 | `_bufDetachFD 0x10f478` | `[[sp+4]+116]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11950c | `_bufDetachLog 0x10f4e8` | `[[sp+4]+116]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x119558 | `_ffscMsg 0x11ebd0` | `0x11944f` | `[sp+4]` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x119490 | `(? & 0x4) == 0x0` | `(? & 0x4) != 0x0` |
| `je` at 0x1194ab | `(? & 0x8) == 0x0` | `(? & 0x8) != 0x0` |
| `jne` at 0x11953d | `[0x1d9b9c] != [sp+4]` | `[0x1d9b9c] == [sp+4]` |
| `je` at 0x119550 | `(([0x1da698]:1 & 0xff) & 0x4) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x119565 | `eax` | `0x0` |

## `_cmSendFFSCCmd`

- `_cmSendFFSCCmd`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x11a290 to 0x11a470.
  Established. The initial `_cmSendFFSCCmdNoPB` site carries `[sp+4]`, `[sp+8]`, `[sp+12]` (`call`
  at 0x11a2a8). A zero `_strchr` result takes the branch to the exit path (`je` at 0x11a2c7).
  Otherwise `_strcpy` has candidates `sp-248`, `ret(_strchr)+1` (`call` at 0x11a2d4), and
  `_crProcessCmd` has `[sp+4]`, `sp-248` (`call` at 0x11a2f5). The branch falls through to a
  byte-zero store at `ret(_strchr) +0x0` when `[[sp+4]+52] != 0x3` (`je` at 0x11a300, `mov` at
  0x11a302). Protocol meaning is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x11a29c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x11a29f | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x11a2a2 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x11a2b0 | load | 1 | `[sp+4] +0x92` | `-` |
| `test` at 0x11a2dc | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x11a2fa | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x11a302 | store | 1 | `ret(_strchr) +0x0` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x11a2a8 | `_cmSendFFSCCmdNoPB 0x11a470` | `[sp+4]` | `[sp+8]` | `[sp+12]` | `ebx` | `esi` | `edi` |
| `call` at 0x11a2bb | `_strchr 0x17de58` | `[sp+12]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11a2d4 | `_strcpy 0x17dfdc` | `sp-248` | `ret(_strchr)+1` | `?` | `?` | `?` | `?` |
| `call` at 0x11a2eb | `_ffscMsg 0x11ebd0` | `0x11a26c` | `sp-248` | `?` | `?` | `?` | `?` |
| `call` at 0x11a2f5 | `_crProcessCmd 0x11bc80` | `[sp+4]` | `sp-248` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x11a2c7 | `(ret(_strchr) & ret(_strchr)) == 0x0` | `(ret(_strchr) & ret(_strchr)) != 0x0` |
| `je` at 0x11a2e3 | `(([0x1da698]:1 & 0xff) & 0x4) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x4) != 0x0` |
| `je` at 0x11a300 | `[[sp+4]+52] == 0x3` | `[[sp+4]+52] != 0x3` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x11a311 | `eax` | `?` |

## `_cmSendFFSCCmdNoPB`

- `_cmSendFFSCCmdNoPB`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x11a470 to 0x11a690.
  Established. The `_ioctl` sites use leading triples `[[sp+4]+88]`, `0x11`, `sp-8` and
  `[[sp+4]+88]`, `0x2`, `0x0` (`call` at 0x11a4c4, `call` at 0x11a500). `_timeoutWrite` has
  candidates `[[sp+4]+84]`, `[sp+8]`, with its remaining slots unknown (`call` at 0x11a527).
  `_timeoutRead` has `[[sp+4]+88]`, `[sp+12]`, `0x5b2`, then unknown slots (`call` at 0x11a576). A
  one-byte zero store targets `(ret(_timeoutRead) + [sp+12]) +0x0` (`mov` at 0x11a5c3). Command
  identities, time units and transfer outcomes are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x11a479 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x11a47c | load | 4 | `sp +0x8` | `-` |
| `test` at 0x11a49b | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x11a4b3 | store | 4 | `sp -0x8` | `0x0` |
| `mov` at 0x11a4c0 | load | 4 | `[sp+4] +0x58` | `-` |
| `cmp` at 0x11a4e0 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x11a4e7 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x11a4fc | load | 4 | `[sp+4] +0x58` | `-` |
| `mov` at 0x11a51a | load | 4 | `abs 0x1da128` | `-` |
| `mov` at 0x11a523 | load | 4 | `[sp+4] +0x54` | `-` |
| `mov` at 0x11a533 | load | 4 | `[sp+4] +0x54` | `-` |
| `mov` at 0x11a560 | load | 4 | `abs 0x1da11c` | `-` |
| `mov` at 0x11a56e | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x11a572 | load | 4 | `[sp+4] +0x58` | `-` |
| `mov` at 0x11a584 | load | 4 | `[sp+4] +0x58` | `-` |
| `mov` at 0x11a59f | load | 4 | `ret(___errno) +0x0` | `-` |
| `mov` at 0x11a5ab | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x11a5c0 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x11a5c3 | store | 1 | `(ret(_timeoutRead) + [sp+12]) +0x0` | `0x0` |
| `mov` at 0x11a5db | store | 4 | `sp -0xc` | `0x0` |
| `mov` at 0x11a5eb | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x11a60f | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x11a620 | load | 4 | `abs 0x1da120` | `-` |
| `mov` at 0x11a63a | load | 1 | `(unknown)` | `-` |
| `mov` at 0x11a65c | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x11a66c | store | 1 | `ret(_strrchr) +0x0` | `0x0` |
| `test` at 0x11a66f | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x11a678 | load | 4 | `sp +0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x11a491 | `_strlen 0x172f14` | `[sp+8]` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x11a4ab | `_ffscMsg 0x11ebd0` | `0x11a31d` | `ret(_strlen)` | `[sp+8]` | `?` | `?` | `?` |
| `call` at 0x11a4c4 | `_ioctl 0x186f90` | `[[sp+4]+88]` | `0x11` | `sp-8` | `?` | `?` | `?` |
| `call` at 0x11a4f0 | `_ffscMsg 0x11ebd0` | `0x11a376` | `[sp-8]` | `[sp+4]` | `?` | `?` | `?` |
| `call` at 0x11a500 | `_ioctl 0x186f90` | `[[sp+4]+88]` | `0x2` | `0x0` | `?` | `?` | `?` |
| `call` at 0x11a512 | `_ffscMsgS 0x11eca0` | `?` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x11a527 | `_timeoutWrite 0x12a788` | `[[sp+4]+84]` | `[sp+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x11a53d | `_ffscMsgS 0x11eca0` | `0x11a3dd` | `[sp+4]` | `[[sp+4]+84]` | `?` | `?` | `?` |
| `call` at 0x11a576 | `_timeoutRead 0x12a538` | `[[sp+4]+88]` | `[sp+12]` | `0x5b2` | `?` | `?` | `?` |
| `call` at 0x11a58e | `_ffscMsgS 0x11eca0` | `0x11a415` | `[sp+4]` | `[[sp+4]+88]` | `?` | `?` | `?` |
| `call` at 0x11a59a | `___errno 0x16fa78` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11a5af | `_sprintf 0x171680` | `[sp+12]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11a5cf | `_strncmp 0x1749b8` | `[sp+12]` | `0x11a454` | `0x4` | `?` | `?` | `?` |
| `call` at 0x11a5f2 | `_strtok_r 0x17e358` | `[sp+12]+4` | `0x11a459` | `sp-12` | `?` | `?` | `?` |
| `call` at 0x11a605 | `_strtol 0x17d9f8` | `ret(_strtok_r)` | `0x0` | `0x0` | `?` | `?` | `?` |
| `call` at 0x11a630 | `_strspn 0x17e264` | `?` | `0x11a459` | `?` | `?` | `?` | `?` |
| `call` at 0x11a642 | `_cmPrintResponse 0x11a120` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11a660 | `_strrchr 0x17e240` | `[sp+12]` | `0xd` | `?` | `?` | `?` | `?` |
| `call` at 0x11a681 | `_ffscMsg 0x11ebd0` | `0x11a45c` | `[sp+12]` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x11a481 | `([sp+8] & [sp+8]) != 0x0` | `([sp+8] & [sp+8]) == 0x0` |
| `je` at 0x11a4a2 | `(([0x1da698]:1 & 0xff) & 0x4) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x4) != 0x0` |
| `je` at 0x11a4ce | `(ret(_ioctl) & ret(_ioctl)) == 0x0` | `(ret(_ioctl) & ret(_ioctl)) != 0x0` |
| `jle` at 0x11a4e4 | `[sp-8] <= 0x0 (signed, 32-bit)` | `[sp-8] > 0x0 (signed, 32-bit)` |
| `je` at 0x11a50a | `(ret(_ioctl) & ret(_ioctl)) == 0x0` | `(ret(_ioctl) & ret(_ioctl)) != 0x0` |
| `je` at 0x11a531 | `ret(_timeoutWrite) == ?` | `ret(_timeoutWrite) != ?` |
| `jg` at 0x11a582 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `jge` at 0x11a598 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x11a5a4 | `[ret(___errno)] == 0x4` | `[ret(___errno)] != 0x4` |
| `jne` at 0x11a5d9 | `(ret(_strncmp) & ret(_strncmp)) != 0x0` | `(ret(_strncmp) & ret(_strncmp)) == 0x0` |
| `je` at 0x11a5fe | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` |
| `je` at 0x11a628 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x11a63e | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x11a654 | `Z == 0` | `not (Z == 0)` |
| `je` at 0x11a66a | `(ret(_strrchr) & ret(_strrchr)) == 0x0` | `(ret(_strrchr) & ret(_strrchr)) != 0x0` |
| `je` at 0x11a676 | `(([0x1da698]:1 & 0xff) & 0x4) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x11a68f | `eax` | `?` |

## `_crProcessMSGCmd`

- `_crProcessMSGCmd`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x11bd90 to 0x11bfa0.
  Established. The first byte-comparison branch takes the default arm when the printed byte is not
  `0x40` (`jne` at 0x11bdbf). A later `_strchr` call has all candidate slots unknown (`call` at
  0x11be70); its nonzero-result fall-through arm stores one-byte zero at `ret(_strchr) +0x0` (`je`
  at 0x11be7c, `mov` at 0x11be84). `_sprintf` sites use destination candidate `sp-252` with second
  candidates `0x11bd7e` and `0x11bd83` (`call` at 0x11bed8, `call` at 0x11bf04). `_cmPrintMsg` then
  has unknown candidate zero and `sp-252` in slot one (`call` at 0x11bf14). Destination identities
  and message effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x11bd9c | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x11bd9f | store | 4 | `sp -0x104` | `0x0` |
| `mov` at 0x11bdbb | load | 1 | `([sp+8] + ret(_strspn)) +0x0` | `-` |
| `mov` at 0x11bdc5 | store | 4 | `sp -0x100` | `([sp+8] + ret(_strspn))` |
| `mov` at 0x11bddb | load | 1 | `(([sp+8] + ret(_strspn)) + ret(_strcspn)) +0x0` | `-` |
| `mov` at 0x11bde1 | store | 1 | `(([sp+8] + ret(_strspn)) + ret(_strcspn)) +0x0` | `0x0` |
| `mov` at 0x11bdfb | load | 4 | `sp -0x100` | `-` |
| `mov` at 0x11be02 | store | 4 | `sp -0x108` | `?` |
| `mov` at 0x11be08 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11be13 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11be1b | load | 4 | `sp -0x108` | `-` |
| `mov` at 0x11be38 | load | 4 | `sp -0x100` | `-` |
| `mov` at 0x11be50 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11be60 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x11be65 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x11be7e | store | 4 | `sp -0x104` | `?` |
| `mov` at 0x11be84 | store | 1 | `ret(_strchr) +0x0` | `0x0` |
| `mov` at 0x11be99 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11bead | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11beb3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11beba | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x11bec2 | load | 4 | `sp -0x104` | `-` |
| `mov` at 0x11bef1 | load | 4 | `sp -0x104` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x11bdb1 | `_strspn 0x17e264` | `[sp+8]` | `0x11bc16` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x11bdd1 | `_strcspn 0x17e008` | `([sp+8] + ret(_strspn))` | `0x11bc16` | `?` | `?` | `?` | `?` |
| `call` at 0x11bded | `_strspn 0x17e264` | `(([sp+8] + ret(_strspn)) + ret(_strcspn))+1` | `0x11bc16` | `?` | `?` | `?` | `?` |
| `call` at 0x11be22 | `_strcmp 0x17de80` | `[sp-264]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11be44 | `_ffscMsg 0x11ebd0` | `0x11bd57` | `[sp-256]` | `?` | `?` | `?` | `?` |
| `call` at 0x11be70 | `_strchr 0x17de58` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11be8e | `_strspn 0x17e264` | `?` | `0x11bc16` | `?` | `?` | `?` | `?` |
| `call` at 0x11bea1 | `_strlen 0x172f14` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11bed8 | `_sprintf 0x171680` | `sp-252` | `0x11bd7e` | `?` | `?` | `?` | `?` |
| `call` at 0x11bf04 | `_sprintf 0x171680` | `sp-252` | `0x11bd83` | `[sp-260]` | `?` | `?` | `?` |
| `call` at 0x11bf14 | `_cmPrintMsg 0x11a0c0` | `?` | `sp-252` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x11bdbf | `([([sp+8] + ret(_strspn))]:1 & 0xff) != 0x40` | `([([sp+8] + ret(_strspn))]:1 & 0xff) == 0x40` |
| `je` at 0x11bddf | `(([(([sp+8] + ret(_strspn)) + ret(_strcspn))]:1 & 0xff) & ([(([sp+8] + ret(_strspn)) + ret(_strcspn))]:1 & 0xff)) == 0x0` | `(([(([sp+8] + ret(_strspn)) + ret(_strcspn))]:1 & 0xff) & ([(([sp+8] + ret(_strspn)) + ret(_strcspn))]:1 & 0xff)) != 0x0` |
| `je` at 0x11be11 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x11be2c | `(ret(_strcmp) & ret(_strcmp)) == 0x0` | `(ret(_strcmp) & ret(_strcmp)) != 0x0` |
| `jle` at 0x11be32 | `? <= 0x3 (signed, 32-bit)` | `? > 0x3 (signed, 32-bit)` |
| `jne` at 0x11be36 | `Z == 0` | `not (Z == 0)` |
| `je` at 0x11be7c | `(ret(_strchr) & ret(_strchr)) == 0x0` | `(ret(_strchr) & ret(_strchr)) != 0x0` |
| `je` at 0x11be9e | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x11beab | `(ret(_strlen) & ret(_strlen)) != 0x0` | `(ret(_strlen) & ret(_strlen)) == 0x0` |
| `je` at 0x11beb8 | `(? & 0x8) == 0x0` | `(? & 0x8) != 0x0` |
| `je` at 0x11bec0 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x11bec9 | `[sp-260] != 0x0` | `[sp-260] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x11bf25 | `eax` | `?` |

## `_crProcessSHUTDOWNCmd`

- `_crProcessSHUTDOWNCmd`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x11bfa0 to 0x11c2c0.
  Established. The sole conditional branch takes its alternate arm when `[[sp+4]+12] == 0x4` (`je`
  at 0x11bfb1). That arm includes `_taskDelay` with candidate zero `0x1e` and `_close` with
  candidate zero `[[[sp+4]+116]]` (`call` at 0x11c030, `call` at 0x11c03e). Both arms join before
  `_bufHold` sites carrying `[[sp+4]+104]` and `[[sp+4]+112]` (`call` at 0x11c04a, `call` at
  0x11c056). Stores at `[sp+4] +0x3c` have unknown values (`mov` at 0x11bfe7, `mov` at 0x11c060). A
  shutdown or power effect is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x11bfa8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x11bfab | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x11bfb3 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x11bfd5 | load | 4 | `[sp+4] +0x74` | `-` |
| `mov` at 0x11bfe1 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11bfe7 | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x11c000 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x11c022 | load | 4 | `[sp+4] +0x74` | `-` |
| `mov` at 0x11c038 | load | 4 | `[sp+4] +0x74` | `-` |
| `mov` at 0x11c03b | load | 4 | `[[sp+4]+116] +0x0` | `-` |
| `mov` at 0x11c046 | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x11c052 | load | 4 | `[sp+4] +0x70` | `-` |
| `mov` at 0x11c05b | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11c060 | store | 4 | `[sp+4] +0x3c` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x11bfc2 | `_sprintf 0x171680` | `sp-84` | `0x11bf26` | `[0x1ee564]` | `ebx` | `esi` | `?` |
| `call` at 0x11bfcb | `_strlen 0x172f14` | `sp-84` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11bfd9 | `_bufWrite 0x10fb78` | `[[sp+4]+116]` | `sp-84` | `ret(_strlen)` | `?` | `?` | `?` |
| `call` at 0x11bfeb | `_consUpdateConsInfo 0x11b770` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11c00f | `_sprintf 0x171680` | `sp-84` | `0x11bf72` | `[0x1ee564]` | `ebx` | `esi` | `?` |
| `call` at 0x11c018 | `_strlen 0x172f14` | `sp-84` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11c026 | `_bufWrite 0x10fb78` | `[[sp+4]+116]` | `sp-84` | `ret(_strlen)` | `?` | `?` | `?` |
| `call` at 0x11c030 | `_taskDelay 0x170b80` | `0x1e` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11c03e | `_close 0x186ed0` | `[[[sp+4]+116]]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11c04a | `_bufHold 0x10f678` | `[[sp+4]+104]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11c056 | `_bufHold 0x10f678` | `[[sp+4]+112]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x11bfb1 | `[[sp+4]+12] == 0x4` | `[[sp+4]+12] != 0x4` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x11c06b | `eax` | `?` |

## `_crProcessSTATECmd`

- `_crProcessSTATECmd`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x11c2c0 to 0x11cc60.
  Established. Entry assignments include `0xffffffff` at `sp -0xfc`, `sp -0x100`, `sp -0x104`, `sp
  -0x108`, and zero at `sp -0x10c` (`mov` at 0x11c2cf, `mov` at 0x11c2de, `mov` at 0x11c2e8, `mov`
  at 0x11c2f4, `mov` at 0x11c2fe). `_strtok_r` carries `0x0`, `0x11bc16`, `sp-8` (`call` at
  0x11c313). The next byte comparison takes the alternate arm when the printed value is not `0x40`
  (`jne` at 0x11c329). Linear decoding stops within an embedded table and omits known downstream
  targets. The complete state transition behavior and callees beyond this boundary are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x11c2cc | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x11c2cf | store | 4 | `sp -0xfc` | `0xffffffff` |
| `mov` at 0x11c2d9 | store | 4 | `sp -0x8` | `[sp+8]` |
| `mov` at 0x11c2de | store | 4 | `sp -0x100` | `0xffffffff` |
| `mov` at 0x11c2e8 | store | 4 | `sp -0x104` | `0xffffffff` |
| `mov` at 0x11c2f4 | store | 4 | `sp -0x108` | `0xffffffff` |
| `mov` at 0x11c2fe | store | 4 | `sp -0x10c` | `0x0` |
| `mov` at 0x11c325 | load | 1 | `ret(_strtok_r) +0x0` | `-` |
| `mov` at 0x11c330 | store | 4 | `sp -0x110` | `ret(_strtok_r)+1` |
| `mov` at 0x11c338 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11c343 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11c34b | load | 4 | `sp -0x110` | `-` |
| `mov` at 0x11c352 | store | 4 | `sp -0x118` | `?` |
| `mov` at 0x11c362 | load | 4 | `sp -0x118` | `-` |
| `mov` at 0x11c390 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11c3a0 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x11c313 | `_strtok_r 0x17e358` | `0x0` | `0x11bc16` | `sp-8` | `ebx` | `esi` | `edi` |
| `call` at 0x11c358 | `_strcmp 0x17de80` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11c380 | `_ffscMsg 0x11ebd0` | `0x11c06c` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x11c31f | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` |
| `jne` at 0x11c329 | `([ret(_strtok_r)]:1 & 0xff) != 0x40` | `([ret(_strtok_r)]:1 & 0xff) == 0x40` |
| `je` at 0x11c341 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x11c36a | `(ret(_strcmp) & ret(_strcmp)) == 0x0` | `(ret(_strcmp) & ret(_strcmp)) != 0x0` |
| `jle` at 0x11c370 | `? <= 0x3 (signed, 32-bit)` | `? > 0x3 (signed, 32-bit)` |
| `jne` at 0x11c374 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|

## `_csSysInWatchOOB`

- `_csSysInWatchOOB`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x11d400 to 0x11d730.
  Established. The branch takes its arm when `[[sp+4]+28] == 0x7` (`je` at 0x11d44c).
  Within that arm, `jl` is taken when `[[[sp+4]+104]+16] < [[sp+4]+32] (signed, 32-bit)`
  (`jl` at 0x11d628). The `_bcopy` call carries
  `[[[sp+4]+104]+4]`, `0x1dad90`, `[[sp+4]+32]` (`call` at 0x11d672). `_oobProcessMsg` then carries
  `[sp+4]`, `0x1dad90` (`call` at 0x11d695). Stores include `0x1` at `[sp+4] +0x1c` and zero at
  `[sp+4] +0x20` (`mov` at 0x11d6a4, `mov` at 0x11d6ab). Packet layout and state meanings are
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x11d406 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x11d409 | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x11d420 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x11d460 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11d46a | load | 4 | `[[sp+4]+104] +0x10` | `-` |
| `mov` at 0x11d46e | load | 1 | `[sp+4] +0x96` | `-` |
| `mov` at 0x11d478 | load | 4 | `[[sp+4]+104] +0x4` | `-` |
| `mov` at 0x11d48a | load | 4 | `[[sp+4]+104] +0x10` | `-` |
| `mov` at 0x11d4a0 | load | 4 | `[[sp+4]+104] +0x4` | `-` |
| `test` at 0x11d4a7 | load | 1 | `abs 0x1da699` | `-` |
| `mov` at 0x11d4cd | store | 4 | `[sp+4] +0x1c` | `0x5` |
| `mov` at 0x11d4d4 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11d4da | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x11d4dd | load | 4 | `[sp+4] +0x70` | `-` |
| `mov` at 0x11d4e9 | load | 4 | `[sp+4] +0x6c` | `-` |
| `mov` at 0x11d4f5 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11d4fa | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x11d510 | load | 4 | `[[sp+4]+104] +0x10` | `-` |
| `mov` at 0x11d51c | load | 4 | `[[sp+4]+104] +0x4` | `-` |
| `mov` at 0x11d51f | load | 1 | `[[[sp+4]+104]+4] +0x1` | `-` |
| `mov` at 0x11d522 | load | 1 | `[sp+4] +0x96` | `-` |
| `mov` at 0x11d530 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11d537 | load | 4 | `[sp+4] +0x70` | `-` |
| `mov` at 0x11d543 | load | 4 | `[sp+4] +0x6c` | `-` |
| `mov` at 0x11d54f | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11d554 | store | 4 | `[sp+4] +0x3c` | `?` |
| `test` at 0x11d557 | load | 1 | `abs 0x1da699` | `-` |
| `mov` at 0x11d56e | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x11d57f | load | 4 | `[sp+4] +0x74` | `-` |
| `mov` at 0x11d58b | load | 4 | `[[sp+4]+104] +0x4` | `-` |
| `mov` at 0x11d59b | store | 4 | `[sp+4] +0x1c` | `0x1` |
| `mov` at 0x11d5a2 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11d5a8 | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x11d5b0 | store | 4 | `[sp+4] +0x1c` | `0x6` |
| `test` at 0x11d5b7 | load | 1 | `abs 0x1da699` | `-` |
| `mov` at 0x11d5e0 | load | 4 | `[[sp+4]+104] +0x10` | `-` |
| `mov` at 0x11d5ec | store | 4 | `[sp+4] +0x1c` | `0x7` |
| `mov` at 0x11d5f3 | load | 4 | `[[sp+4]+104] +0x4` | `-` |
| `mov` at 0x11d5f6 | load | 1 | `[[[sp+4]+104]+4] +0x2` | `-` |
| `mov` at 0x11d5fc | load | 1 | `[[[sp+4]+104]+4] +0x3` | `-` |
| `mov` at 0x11d60b | store | 4 | `[sp+4] +0x20` | `?` |
| `mov` at 0x11d620 | load | 4 | `[[sp+4]+104] +0x10` | `-` |
| `mov` at 0x11d623 | load | 4 | `[sp+4] +0x20` | `-` |
| `test` at 0x11d62e | load | 1 | `abs 0x1da699` | `-` |
| `mov` at 0x11d637 | load | 4 | `[[sp+4]+104] +0x4` | `-` |
| `mov` at 0x11d63a | load | 1 | `[[[sp+4]+104]+4] +0x2` | `-` |
| `mov` at 0x11d643 | load | 1 | `[[[sp+4]+104]+4] +0x3` | `-` |
| `mov` at 0x11d64e | load | 1 | `[[[sp+4]+104]+4] +0x1` | `-` |
| `mov` at 0x11d665 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x11d66e | load | 4 | `[[sp+4]+104] +0x4` | `-` |
| `mov` at 0x11d67a | load | 4 | `[[sp+4]+104] +0x4` | `-` |
| `mov` at 0x11d67d | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x11d68c | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x11d69d | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x11d6a4 | store | 4 | `[sp+4] +0x1c` | `0x1` |
| `mov` at 0x11d6ab | store | 4 | `[sp+4] +0x20` | `0x0` |
| `mov` at 0x11d6b2 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11d6b8 | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x11d6bb | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11d6c2 | load | 4 | `[sp+4] +0x70` | `-` |
| `mov` at 0x11d6ce | load | 4 | `[sp+4] +0x6c` | `-` |
| `mov` at 0x11d6da | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11d6df | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x11d6f0 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x11d710 | load | 4 | `[[sp+4]+104] +0x10` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x11d47c | `_memchr 0x17dd2c` | `[[[sp+4]+104]+4]` | `?` | `[[[sp+4]+104]+16]` | `?` | `?` | `?` |
| `call` at 0x11d48f | `_csSysInToUserOut 0x11d730` | `[sp+4]` | `[[[sp+4]+104]+16]` | `?` | `?` | `?` | `?` |
| `call` at 0x11d4b7 | `_ffscMsg 0x11ebd0` | `0x11d34b` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x11d4c5 | `_csSysInToUserOut 0x11d730` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11d4e1 | `_bufHold 0x10f678` | `[[sp+4]+112]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11d4ed | `_bufHold 0x10f678` | `[[sp+4]+108]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11d53b | `_bufRelease 0x10fa18` | `[[sp+4]+112]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11d547 | `_bufRelease 0x10fa18` | `[[sp+4]+108]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11d566 | `_ffscMsg 0x11ebd0` | `0x11d36a` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x11d583 | `_bufWrite 0x10fb78` | `[[sp+4]+116]` | `[sp+4]+150` | `0x1` | `?` | `?` | `?` |
| `call` at 0x11d593 | `_bufLeftAlign 0x10f838` | `[[sp+4]+104]` | `[[[sp+4]+104]+4]+2` | `?` | `?` | `?` | `?` |
| `call` at 0x11d5ca | `_ffscMsg 0x11ebd0` | `0x11d37e` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x11d65d | `_ffscMsg 0x11ebd0` | `0x11d3a3` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x11d672 | `_bcopy 0x1747fc` | `[[[sp+4]+104]+4]` | `0x1dad90` | `[[sp+4]+32]` | `?` | `?` | `?` |
| `call` at 0x11d684 | `_bufLeftAlign 0x10f838` | `[[sp+4]+104]` | `([[[sp+4]+104]+4] + [[sp+4]+32])` | `?` | `?` | `?` | `?` |
| `call` at 0x11d695 | `_oobProcessMsg 0x1271c4` | `[sp+4]` | `0x1dad90` | `?` | `?` | `?` | `?` |
| `call` at 0x11d6c6 | `_bufRelease 0x10fa18` | `[[sp+4]+112]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11d6d2 | `_bufRelease 0x10fa18` | `[[sp+4]+108]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11d6fa | `_ffscMsg 0x11ebd0` | `0x11d3c9` | `[sp+4]` | `[[sp+4]+28]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x11d426 | `[[sp+4]+28] == 0x5` | `[[sp+4]+28] != 0x5` |
| `jg` at 0x11d42c | `[[sp+4]+28] > 0x5 (signed, 32-bit)` | `[[sp+4]+28] <= 0x5 (signed, 32-bit)` |
| `je` at 0x11d431 | `[[sp+4]+28] == 0x1` | `[[sp+4]+28] != 0x1` |
| `je` at 0x11d443 | `[[sp+4]+28] == 0x6` | `[[sp+4]+28] != 0x6` |
| `je` at 0x11d44c | `[[sp+4]+28] == 0x7` | `[[sp+4]+28] != 0x7` |
| `je` at 0x11d468 | `([[sp+4]+60] & 0x10000) == 0x0` | `([[sp+4]+60] & 0x10000) != 0x0` |
| `jne` at 0x11d488 | `(ret(_memchr) & ret(_memchr)) != 0x0` | `(ret(_memchr) & ret(_memchr)) == 0x0` |
| `je` at 0x11d4ae | `(([0x1da699]:1 & 0xff) & 0x8) == 0x0` | `(([0x1da699]:1 & 0xff) & 0x8) != 0x0` |
| `jle` at 0x11d4c1 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jle` at 0x11d516 | `[[[sp+4]+104]+16] <= 0x1 (signed, 32-bit)` | `[[[sp+4]+104]+16] > 0x1 (signed, 32-bit)` |
| `jne` at 0x11d52a | `([[[[sp+4]+104]+4]+1]:1 & 0xff) != ([[sp+4]+150]:1 & 0xff)` | `([[[[sp+4]+104]+4]+1]:1 & 0xff) == ([[sp+4]+150]:1 & 0xff)` |
| `je` at 0x11d535 | `(? & 0x8) == 0x0` | `(? & 0x8) != 0x0` |
| `je` at 0x11d55e | `(([0x1da699]:1 & 0xff) & 0x8) == 0x0` | `(([0x1da699]:1 & 0xff) & 0x8) != 0x0` |
| `je` at 0x11d574 | `[[sp+4]+24] == 0x9` | `[[sp+4]+24] != 0x9` |
| `je` at 0x11d5be | `(([0x1da699]:1 & 0xff) & 0x8) == 0x0` | `(([0x1da699]:1 & 0xff) & 0x8) != 0x0` |
| `jle` at 0x11d5e6 | `[[[sp+4]+104]+16] <= 0x3 (signed, 32-bit)` | `[[[sp+4]+104]+16] > 0x3 (signed, 32-bit)` |
| `jl` at 0x11d628 | `[[[sp+4]+104]+16] < [[sp+4]+32] (signed, 32-bit)` | `[[[sp+4]+104]+16] >= [[sp+4]+32] (signed, 32-bit)` |
| `je` at 0x11d635 | `(([0x1da699]:1 & 0xff) & 0x8) == 0x0` | `(([0x1da699]:1 & 0xff) & 0x8) != 0x0` |
| `jne` at 0x11d6a2 | `[[sp+4]+24] != [[sp+4]+24]` | `[[sp+4]+24] == [[sp+4]+24]` |
| `je` at 0x11d6c0 | `(? & 0x8) == 0x0` | `(? & 0x8) != 0x0` |
| `jg` at 0x11d715 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x11d726 | `eax` | `?` |

## `_cuUserInConsole`

- `_cuUserInConsole`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x11da70 to 0x11dfe0.
  Established. The dispatch takes one arm when `[[sp+4]+36] == 0x2` and another when it equals `0x3`
  (`je` at 0x11daa2, `je` at 0x11dac3). Stores include `0x2`, `0x1`, `0x3` at `[sp+4] +0x24` at
  distinct sites (`mov` at 0x11dc00, `mov` at 0x11dcf6, `mov` at 0x11dd10). A `_cmSendFFSCCmd` site
  has candidates `[sp+4]`, `[[sp-1476]+4]`, `sp-1472` (`call` at 0x11dee0).
  The subsequent branch falls through when `[[sp+4]+24] == 0x1` (`jne` at 0x11deee),
  reaching `_cmPrintResponse` with candidates `[sp+4]`, `sp-1472` (`call` at 0x11def2). Character
  and console protocol meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x11da7c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x11da7f | load | 4 | `[sp+4] +0x70` | `-` |
| `mov` at 0x11da82 | store | 4 | `sp -0x5c4` | `[[sp+4]+112]` |
| `mov` at 0x11da88 | load | 4 | `[[sp+4]+112] +0x10` | `-` |
| `mov` at 0x11da96 | store | 4 | `sp -0x5c8` | `sp-12` |
| `mov` at 0x11da9c | load | 4 | `[sp+4] +0x24` | `-` |
| `mov` at 0x11dad2 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11dadc | load | 1 | `[sp+4] +0x8d` | `-` |
| `mov` at 0x11dae2 | store | 1 | `sp -0xc` | `[[sp+4]+141]:1` |
| `mov` at 0x11daea | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11daf4 | load | 1 | `[sp+4] +0x96` | `-` |
| `mov` at 0x11dafa | store | 1 | `(unknown)` | `[[sp+4]+150]:1` |
| `mov` at 0x11daff | store | 1 | `(unknown)` | `0x0` |
| `mov` at 0x11db04 | load | 4 | `sp -0x5c4` | `-` |
| `mov` at 0x11db0a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11db0d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11db10 | store | 1 | `(unknown)` | `0x0` |
| `mov` at 0x11db14 | load | 4 | `sp -0x5c8` | `-` |
| `mov` at 0x11db1b | load | 4 | `sp -0x5c4` | `-` |
| `mov` at 0x11db21 | load | 4 | `[sp-1476] +0x4` | `-` |
| `mov` at 0x11db33 | load | 4 | `sp -0x5c4` | `-` |
| `mov` at 0x11db3a | load | 4 | `[sp+4] +0x6c` | `-` |
| `mov` at 0x11db60 | load | 1 | `ret(_strpbrk) +0x0` | `-` |
| `mov` at 0x11db62 | store | 1 | `sp -0x5cc` | `[ret(_strpbrk)]:1` |
| `mov` at 0x11db68 | load | 4 | `sp -0x5c4` | `-` |
| `mov` at 0x11db6e | load | 4 | `[sp-1476] +0x4` | `-` |
| `test` at 0x11db75 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x11db92 | load | 4 | `sp -0x5c4` | `-` |
| `mov` at 0x11db98 | load | 4 | `[sp-1476] +0x4` | `-` |
| `mov` at 0x11db9c | load | 4 | `[sp+4] +0x6c` | `-` |
| `mov` at 0x11dbaa | load | 4 | `sp -0x5c4` | `-` |
| `mov` at 0x11dbb0 | load | 4 | `[sp-1476] +0x4` | `-` |
| `mov` at 0x11dbc1 | load | 1 | `[sp+4] +0x96` | `-` |
| `cmp` at 0x11dbc7 | load | 1 | `sp -0x5cc` | `-` |
| `test` at 0x11dbd3 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x11dbf1 | load | 4 | `[sp+4] +0x6c` | `-` |
| `mov` at 0x11dc00 | store | 4 | `[sp+4] +0x24` | `0x2` |
| `mov` at 0x11dc07 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x11dc0f | load | 4 | `[sp+4] +0x84` | `-` |
| `mov` at 0x11dc15 | load | 4 | `[[sp+4]+132] +0x1c` | `-` |
| `mov` at 0x11dc1c | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x11dc28 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11dc2d | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x11dc30 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11dc35 | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x11dc38 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11dc3d | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x11dc50 | load | 4 | `sp -0x5c4` | `-` |
| `mov` at 0x11dc56 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11dc59 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x11dc5b | load | 1 | `[sp+4] +0x8d` | `-` |
| `mov` at 0x11dc69 | load | 4 | `[sp+4] +0x84` | `-` |
| `mov` at 0x11dc6f | load | 4 | `[[sp+4]+132] +0x1c` | `-` |
| `mov` at 0x11dc76 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x11dc85 | load | 4 | `[sp+4] +0x74` | `-` |
| `mov` at 0x11dc91 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11dc96 | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x11dc99 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11dca0 | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x11dcac | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11dcb1 | store | 4 | `[sp+4] +0x3c` | `?` |
| `test` at 0x11dcb4 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x11dcd4 | load | 4 | `[sp+4] +0x6c` | `-` |
| `mov` at 0x11dce2 | load | 4 | `sp -0x5c4` | `-` |
| `mov` at 0x11dce8 | load | 4 | `[sp-1476] +0x4` | `-` |
| `mov` at 0x11dcf6 | store | 4 | `[sp+4] +0x24` | `0x1` |
| `mov` at 0x11dd10 | store | 4 | `[sp+4] +0x24` | `0x3` |
| `mov` at 0x11dd17 | store | 4 | `[sp+4] +0x28` | `0x0` |
| `test` at 0x11dd1e | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x11dd35 | load | 4 | `[sp+4] +0x84` | `-` |
| `mov` at 0x11dd3b | load | 4 | `[[sp+4]+132] +0x1c` | `-` |
| `mov` at 0x11dd46 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x11dd52 | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x11dd5e | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11dd63 | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x11dd66 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11dd6b | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x11dd75 | load | 4 | `[sp+4] +0x74` | `-` |
| `mov` at 0x11dd97 | load | 4 | `sp -0x5c8` | `-` |
| `mov` at 0x11dda6 | load | 1 | `sp -0xb` | `-` |
| `mov` at 0x11dda9 | store | 1 | `sp -0x8` | `[sp-11]:1` |
| `mov` at 0x11ddac | load | 4 | `sp -0x5c4` | `-` |
| `mov` at 0x11ddb2 | load | 4 | `[sp-1476] +0x4` | `-` |
| `mov` at 0x11ddb5 | load | 4 | `[sp-1476] +0x10` | `-` |
| `mov` at 0x11ddb8 | store | 1 | `([[sp-1476]+16] + [[sp-1476]+4]) +0x0` | `0x0` |
| `mov` at 0x11ddbc | load | 4 | `[sp-1476] +0x4` | `-` |
| `mov` at 0x11ddbf | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x11ddc5 | load | 4 | `sp -0x5c8` | `-` |
| `mov` at 0x11dddb | load | 4 | `sp -0x5c4` | `-` |
| `mov` at 0x11dde1 | load | 4 | `[sp-1476] +0x4` | `-` |
| `mov` at 0x11dde4 | load | 4 | `[sp-1476] +0x10` | `-` |
| `mov` at 0x11ddee | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x11ddf3 | store | 4 | `[sp+4] +0x28` | `?` |
| `mov` at 0x11ddf6 | load | 4 | `[sp+4] +0x70` | `-` |
| `mov` at 0x11ddf9 | load | 4 | `[[sp+4]+112] +0x14` | `-` |
| `mov` at 0x11de00 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x11de07 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x11de0b | store | 4 | `[sp+4] +0x28` | `?` |
| `mov` at 0x11de0e | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x11de12 | load | 1 | `[sp+4] +0x8e` | `-` |
| `mov` at 0x11de18 | store | 1 | `(unknown)` | `[[sp+4]+142]:1` |
| `mov` at 0x11de1a | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11de27 | load | 4 | `[sp+4] +0x74` | `-` |
| `mov` at 0x11de3b | load | 1 | `(unknown)` | `-` |
| `mov` at 0x11de45 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x11de47 | load | 1 | `[sp+4] +0x8d` | `-` |
| `mov` at 0x11de51 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x11de53 | load | 1 | `[sp+4] +0x8f` | `-` |
| `mov` at 0x11de5d | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11de70 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x11de72 | load | 1 | `[sp+4] +0x8c` | `-` |
| `mov` at 0x11de7c | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11dea0 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x11dea2 | load | 1 | `[sp+4] +0x8e` | `-` |
| `mov` at 0x11deb0 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x11debf | load | 4 | `[sp+4] +0x74` | `-` |
| `mov` at 0x11decb | store | 1 | `(unknown)` | `0x0` |
| `mov` at 0x11ded5 | load | 4 | `sp -0x5c4` | `-` |
| `mov` at 0x11dedb | load | 4 | `[sp-1476] +0x4` | `-` |
| `mov` at 0x11dee8 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x11defe | load | 4 | `sp -0x5c4` | `-` |
| `mov` at 0x11df0d | store | 4 | `[sp+4] +0x24` | `0x1` |
| `mov` at 0x11df14 | store | 4 | `[sp+4] +0x28` | `0x0` |
| `mov` at 0x11df1b | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11df20 | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x11df23 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11df2a | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x11df36 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11df3b | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x11df40 | load | 4 | `[sp+4] +0x24` | `-` |
| `mov` at 0x11df60 | load | 4 | `sp -0x5c4` | `-` |
| `mov` at 0x11df66 | load | 4 | `[sp-1476] +0x10` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x11db25 | `_strpbrk 0x17e1f4` | `[[sp-1476]+4]` | `[sp-1480]` | `?` | `?` | `?` | `?` |
| `call` at 0x11db3e | `_bufAppendBuf 0x10efd8` | `[[sp+4]+108]` | `[sp-1476]` | `?` | `?` | `?` | `?` |
| `call` at 0x11db85 | `_ffscMsg 0x11ebd0` | `0x11d9ab` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x11dba0 | `_bufWrite 0x10fb78` | `[[sp+4]+108]` | `[[sp-1476]+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x11dbb9 | `_bufLeftAlign 0x10f838` | `[sp-1476]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11dbe2 | `_ffscMsg 0x11ebd0` | `0x11d9ce` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x11dbf5 | `_bufWrite 0x10fb78` | `[[sp+4]+108]` | `0x11d9e3` | `0x2` | `?` | `?` | `?` |
| `call` at 0x11dc20 | `_bufHold 0x10f678` | `[[sp+4]+104]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11dc89 | `_bufWrite 0x10fb78` | `[[sp+4]+116]` | `0x11d9e6` | `0x15` | `?` | `?` | `?` |
| `call` at 0x11dca4 | `_bufRelease 0x10fa18` | `[[sp+4]+104]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11dcc3 | `_ffscMsg 0x11ebd0` | `0x11d9fc` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x11dcd8 | `_bufWrite 0x10fb78` | `[[sp+4]+108]` | `[sp+4]+141` | `0x1` | `?` | `?` | `?` |
| `call` at 0x11dcee | `_bufLeftAlign 0x10f838` | `[sp-1476]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11dd2d | `_ffscMsg 0x11ebd0` | `0x11da10` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x11dd56 | `_bufHold 0x10f678` | `[[sp+4]+104]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11dd79 | `_bufWrite 0x10fb78` | `[[sp+4]+116]` | `0x11da2e` | `0x7` | `?` | `?` | `?` |
| `call` at 0x11dd9e | `_strcpy 0x17dfdc` | `?` | `[sp+4]+140` | `?` | `?` | `?` | `?` |
| `call` at 0x11ddcd | `_strpbrk 0x17e1f4` | `([[sp+4]+40] + [[sp-1476]+4])` | `[sp-1480]` | `?` | `?` | `?` | `?` |
| `call` at 0x11de2b | `_bufWrite 0x10fb78` | `[[sp+4]+116]` | `([[sp+4]+40] + [[sp-1476]+4])` | `?` | `?` | `?` | `?` |
| `call` at 0x11de66 | `_cuProcessKILL 0x11d960` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11de85 | `_cuProcessBS 0x11d8b0` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11dec3 | `_bufWrite 0x10fb78` | `[[sp+4]+116]` | `0x11da36` | `0x2` | `?` | `?` | `?` |
| `call` at 0x11dee0 | `_cmSendFFSCCmd 0x11a290` | `[sp+4]` | `[[sp-1476]+4]` | `sp-1472` | `?` | `?` | `?` |
| `call` at 0x11def2 | `_cmPrintResponse 0x11a120` | `[sp+4]` | `sp-1472` | `?` | `?` | `?` | `?` |
| `call` at 0x11df05 | `_bufLeftAlign 0x10f838` | `[sp-1476]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11df2e | `_bufRelease 0x10fa18` | `[[sp+4]+104]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11df4a | `_ffscMsg 0x11ebd0` | `0x11da39` | `[sp+4]` | `[[sp+4]+36]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x11da8d | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `je` at 0x11daa2 | `[[sp+4]+36] == 0x2` | `[[sp+4]+36] != 0x2` |
| `jg` at 0x11daa8 | `[[sp+4]+36] > 0x2 (signed, 32-bit)` | `[[sp+4]+36] <= 0x2 (signed, 32-bit)` |
| `je` at 0x11daad | `[[sp+4]+36] == 0x1` | `[[sp+4]+36] != 0x1` |
| `je` at 0x11dac3 | `[[sp+4]+36] == 0x3` | `[[sp+4]+36] != 0x3` |
| `jne` at 0x11dada | `([[sp+4]+60] & 0x80000) != 0x0` | `([[sp+4]+60] & 0x80000) == 0x0` |
| `je` at 0x11daf2 | `([[sp+4]+60] & 0x20000) == 0x0` | `([[sp+4]+60] & 0x20000) != 0x0` |
| `jne` at 0x11db31 | `(ret(_strpbrk) & ret(_strpbrk)) != 0x0` | `(ret(_strpbrk) & ret(_strpbrk)) == 0x0` |
| `jge` at 0x11db48 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x11db7c | `(([0x1da698]:1 & 0xff) & 0x4) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x4) != 0x0` |
| `jle` at 0x11db8f | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jne` at 0x11dbcd | `([sp-1484]:1 & 0xff) != ([[sp+4]+150]:1 & 0xff)` | `([sp-1484]:1 & 0xff) == ([[sp+4]+150]:1 & 0xff)` |
| `jl` at 0x11dbd1 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x11dbda | `(([0x1da698]:1 & 0xff) & 0x4) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x4) != 0x0` |
| `jne` at 0x11dc0d | `[[sp+4]+44] != 0x1` | `[[sp+4]+44] == 0x1` |
| `jne` at 0x11dc1a | `(? & 0x1) != 0x0` | `(? & 0x1) == 0x0` |
| `jge` at 0x11dc42 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x11dc63 | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x11dc74 | `(? & 0x1) != 0x0` | `(? & 0x1) == 0x0` |
| `jne` at 0x11dc7c | `[[sp+4]+44] != 0x1` | `[[sp+4]+44] == 0x1` |
| `je` at 0x11dc9e | `(? & 0x4) == 0x0` | `(? & 0x4) != 0x0` |
| `je` at 0x11dcbb | `(([0x1da698]:1 & 0xff) & 0x4) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x4) != 0x0` |
| `jge` at 0x11dcff | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x11dd25 | `(([0x1da698]:1 & 0xff) & 0x4) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x4) != 0x0` |
| `je` at 0x11dd40 | `(? & 0x1) == 0x0` | `(? & 0x1) != 0x0` |
| `jne` at 0x11dd4c | `[[sp+4]+44] != 0x1` | `[[sp+4]+44] == 0x1` |
| `jne` at 0x11ddd9 | `(ret(_strpbrk) & ret(_strpbrk)) != 0x0` | `(ret(_strpbrk) & ret(_strpbrk)) == 0x0` |
| `jne` at 0x11ddfe | `([[[sp+4]+112]+20] & [[[sp+4]+112]+20]) != 0x0` | `([[[sp+4]+112]+20] & [[[sp+4]+112]+20]) == 0x0` |
| `jne` at 0x11de04 | `Z == 0` | `not (Z == 0)` |
| `je` at 0x11de1f | `(? & 0x1) == 0x0` | `(? & 0x1) != 0x0` |
| `jle` at 0x11de23 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jl` at 0x11de35 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x11de3f | `Z == 1` | `not (Z == 1)` |
| `je` at 0x11de4f | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x11de5b | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x11de7a | `Z == 0` | `not (Z == 0)` |
| `je` at 0x11de8f | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x11deaa | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x11deb6 | `[[sp+4]+44] != 0x1` | `[[sp+4]+44] == 0x1` |
| `jne` at 0x11deee | `[[sp+4]+24] != 0x1` | `[[sp+4]+24] == 0x1` |
| `je` at 0x11df28 | `(? & 0x4) == 0x0` | `(? & 0x4) != 0x0` |
| `jg` at 0x11df6b | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x11df7f | `eax` | `?` |

## `_cuUserInCopySys`

- `_cuUserInCopySys`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x11e520 to 0x11e610.
  Established. The main arm is taken when `[[sp+4]+36] == 0x1` (`je` at 0x11e52d). One four-byte
  store at `[sp+4] +0x44` writes `([[sp+4]+68] + [[[sp+4]+112]+16])` (`mov` at 0x11e559). The
  subsequent branch falls through when `[0x1da170] < [[sp+4]+68] (signed, 32-bit)` (`jge` at
  0x11e565), reaching `_timeoutWrite` with candidates `[[sp+4]+100]`, `0x11e512`, `0x1`,
  `[0x1da124]` (`call` at 0x11e578). `_bufAppendBuf` uses `[[sp+4]+108]`, `[[sp+4]+112]` (`call` at
  0x11e594). Counter meaning and transfer effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x11e524 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x11e527 | load | 4 | `[sp+4] +0x24` | `-` |
| `mov` at 0x11e52f | load | 4 | `[sp+4] +0x24` | `-` |
| `mov` at 0x11e540 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11e547 | load | 4 | `[sp+4] +0x64` | `-` |
| `mov` at 0x11e54e | load | 4 | `[sp+4] +0x70` | `-` |
| `mov` at 0x11e551 | load | 4 | `[sp+4] +0x44` | `-` |
| `mov` at 0x11e554 | load | 4 | `[[sp+4]+112] +0x10` | `-` |
| `mov` at 0x11e559 | store | 4 | `[sp+4] +0x44` | `([[sp+4]+68] + [[[sp+4]+112]+16])` |
| `mov` at 0x11e55c | load | 4 | `[sp+4] +0x44` | `-` |
| `cmp` at 0x11e55f | load | 4 | `abs 0x1da170` | `-` |
| `mov` at 0x11e567 | load | 4 | `abs 0x1da124` | `-` |
| `mov` at 0x11e574 | load | 4 | `[sp+4] +0x64` | `-` |
| `mov` at 0x11e580 | load | 4 | `[sp+4] +0x44` | `-` |
| `sub` at 0x11e583 | load | 4 | `abs 0x1da170` | `-` |
| `mov` at 0x11e589 | store | 4 | `[sp+4] +0x44` | `?` |
| `mov` at 0x11e58c | load | 4 | `[sp+4] +0x70` | `-` |
| `mov` at 0x11e590 | load | 4 | `[sp+4] +0x6c` | `-` |
| `mov` at 0x11e5a0 | load | 4 | `[sp+4] +0x70` | `-` |
| `mov` at 0x11e5b5 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x11e539 | `_ffscMsg 0x11ebd0` | `0x11e4e2` | `[sp+4]` | `[[sp+4]+36]` | `ebx` | `ebp` | `?` |
| `call` at 0x11e578 | `_timeoutWrite 0x12a788` | `[[sp+4]+100]` | `0x11e512` | `0x1` | `[0x1da124]` | `?` | `?` |
| `call` at 0x11e594 | `_bufAppendBuf 0x10efd8` | `[[sp+4]+108]` | `[[sp+4]+112]` | `?` | `?` | `?` | `?` |
| `call` at 0x11e5a4 | `_bufClear 0x10f438` | `[[sp+4]+112]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x11e52d | `[[sp+4]+36] == 0x1` | `[[sp+4]+36] != 0x1` |
| `je` at 0x11e545 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `jl` at 0x11e54c | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jge` at 0x11e565 | `[0x1da170] >= [[sp+4]+68] (signed, 32-bit)` | `[0x1da170] < [[sp+4]+68] (signed, 32-bit)` |
| `jne` at 0x11e59e | `(ret(_bufAppendBuf) & ret(_bufAppendBuf)) != 0x0` | `(ret(_bufAppendBuf) & ret(_bufAppendBuf)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x11e5bb | `eax` | `?` |

## `_elscPrintMsg`

- `_elscPrintMsg`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x120790 to 0x120970.
  Established. `_logWriteLine` has candidates `[[sp+4]+44]`, `[sp+12]` (`call` at 0x1207ae). A
  branch falls through when `[0x1ee564] == [[sp+4]+20]` (`jne` at 0x1207bf), reaching
  `_userSendCommand` with `[0x1da96c]`, `sp-248`, `0x0` (`call` at 0x1207e3). A second such site
  carries `[0x1da970]`, `sp-736`, `0x0` (`call` at 0x12085b). Other call sites include
  `_routeDoAdminCommand` with `sp-492` and `sp-980` respectively (`call` at 0x120826, `call` at
  0x120896). The routing roles and command semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12079c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12079f | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1207a2 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x1207aa | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x1207b6 | load | 4 | `[sp+4] +0x14` | `-` |
| `cmp` at 0x1207b9 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1207c2 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1207dd | load | 4 | `abs 0x1da96c` | `-` |
| `mov` at 0x120800 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x120809 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12080d | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x12082e | load | 4 | `[sp+4] +0x18` | `-` |
| `cmp` at 0x120831 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x12083a | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x120855 | load | 4 | `abs 0x1da970` | `-` |
| `mov` at 0x120870 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x120879 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12087d | load | 4 | `[sp+4] +0x18` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1207ae | `_logWriteLine 0x124ff0` | `[[sp+4]+44]` | `[sp+12]` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x1207d2 | `_sprintf 0x171680` | `sp-248` | `0x12074c` | `[sp+8]` | `[sp+12]` | `?` | `?` |
| `call` at 0x1207e3 | `_userSendCommand 0x12b7d8` | `[0x1da96c]` | `sp-248` | `0x0` | `?` | `?` | `?` |
| `call` at 0x12081d | `_sprintf 0x171680` | `sp-492` | `0x120756` | `[[sp+4]+20]` | `[sp+8]` | `[sp+12]` | `?` |
| `call` at 0x120826 | `_routeDoAdminCommand 0x129274` | `sp-492` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12084a | `_sprintf 0x171680` | `sp-736` | `0x12074c` | `[sp+8]` | `[sp+12]` | `?` | `?` |
| `call` at 0x12085b | `_userSendCommand 0x12b7d8` | `[0x1da970]` | `sp-736` | `0x0` | `?` | `?` | `?` |
| `call` at 0x12088d | `_sprintf 0x171680` | `sp-980` | `0x12076c` | `[[sp+4]+24]` | `[sp+8]` | `[sp+12]` | `?` |
| `call` at 0x120896 | `_routeDoAdminCommand 0x129274` | `sp-980` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1207a7 | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |
| `jne` at 0x1207bf | `[0x1ee564] != [[sp+4]+20]` | `[0x1ee564] == [[sp+4]+20]` |
| `je` at 0x1207ed | `(ret(_userSendCommand) & ret(_userSendCommand)) == 0x0` | `(ret(_userSendCommand) & ret(_userSendCommand)) != 0x0` |
| `je` at 0x120806 | `[[sp+4]+20] == 0xffffffff` | `[[sp+4]+20] != 0xffffffff` |
| `jne` at 0x120837 | `[0x1ee564] != [[sp+4]+24]` | `[0x1ee564] == [[sp+4]+24]` |
| `je` at 0x120862 | `(ret(_userSendCommand) & ret(_userSendCommand)) == 0x0` | `(ret(_userSendCommand) & ret(_userSendCommand)) != 0x0` |
| `je` at 0x120876 | `[[sp+4]+24] == 0xffffffff` | `[[sp+4]+24] != 0xffffffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1208a9 | `eax` | `?` |

## `_nvramPrintInfo`

- `_nvramPrintInfo`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x126bc0 to 0x126cf0.
  Established. The branch takes its longer reporting arm when `[0x1da928] == 0xff5cff5c` (`je` at
  0x126bd3). `_ffscMsg` calls carry `[0x1da92c]` in candidate one and `[0x1da930]`, `[0x1da934]` in
  candidates one and two at another site (`call` at 0x126beb, `call` at 0x126c04). The indexed
  four-byte load remains unresolved (`mov` at 0x126c28). Both loop tests retain unknown compared
  values (`jle` at 0x126c50, `jle` at 0x126c56). The exit reports zero (`ret` at 0x126c6d). NVRAM
  layout meaning is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x126bc9 | load | 4 | `abs 0x1da928` | `-` |
| `mov` at 0x126be0 | load | 4 | `abs 0x1da92c` | `-` |
| `mov` at 0x126bf3 | load | 4 | `abs 0x1da934` | `-` |
| `mov` at 0x126bf9 | load | 4 | `abs 0x1da930` | `-` |
| `mov` at 0x126c28 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x126c3c | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x126c47 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x126beb | `_ffscMsg 0x11ebd0` | `0x126b60` | `[0x1da92c]` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x126c04 | `_ffscMsg 0x11ebd0` | `0x126b7d` | `[0x1da930]` | `[0x1da934]` | `?` | `?` | `?` |
| `call` at 0x126c11 | `_ffscMsgN 0x11ec30` | `0x126b9f` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126c3f | `_ffscMsgN 0x11ec30` | `0x126bb4` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x126c5d | `_ffscMsg 0x11ebd0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x126bd3 | `[0x1da928] == 0xff5cff5c` | `[0x1da928] != 0xff5cff5c` |
| `je` at 0x126c31 | `Z == 1` | `not (Z == 1)` |
| `jle` at 0x126c50 | `? <= 0x1f (signed, 32-bit)` | `? > 0x1f (signed, 32-bit)` |
| `jle` at 0x126c56 | `? <= 0x3 (signed, 32-bit)` | `? > 0x3 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x126c6d | `eax` | `0x0` |

## `_cmdEMSG_rack`

- `_cmdEMSG_rack`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x114330 to 0x114520.
  Established. A nonzero `_strtok_r` result takes the parsing arm (`jne` at 0x114364).
  `_ffscParseRackID` carries `ret(_strtok_r)`, `sp-20` (`call` at 0x114440). Four-byte stores at
  unresolved targets write `[sp-20]` at two sites and `0xb` at two others (`mov` at 0x1144b9, `mov`
  at 0x1144c3, `mov` at 0x1143fe, `mov` at 0x1144e0). A later `_elscUpdateSaveInfo` call has unknown
  candidate slots (`call` at 0x1144f0). Rack destination fields and persistence effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x114339 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x11433c | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x114341 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x114344 | load | 1 | `([0x1ee564] + [sp+8]) +0x4` | `-` |
| `mov` at 0x114374 | store | 4 | `sp -0x18` | `sp-16` |
| `mov` at 0x11437c | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x114382 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x114385 | load | 1 | `([0x1ee564] + [sp+8]) +0x4` | `-` |
| `mov` at 0x114393 | load | 1 | `([0x1ee564] + [sp+8]) +0x4` | `-` |
| `mov` at 0x1143a0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1143a4 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x1143c0 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1143c6 | load | 4 | `sp -0x18` | `-` |
| `mov` at 0x1143ca | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1143d0 | load | 4 | `sp -0x18` | `-` |
| `mov` at 0x1143d4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1143e1 | load | 4 | `sp -0x18` | `-` |
| `mov` at 0x1143ea | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1143ee | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1143fa | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1143fe | store | 4 | `(unknown)` | `0xb` |
| `mov` at 0x114451 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x114456 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11445d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x114470 | store | 4 | `sp -0x14` | `0xffffffff` |
| `mov` at 0x114480 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x114486 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x114489 | load | 1 | `([0x1ee564] + [sp+8]) +0x4` | `-` |
| `mov` at 0x114497 | load | 1 | `([0x1ee564] + [sp+8]) +0x4` | `-` |
| `cmp` at 0x1144b0 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1144b6 | load | 4 | `sp -0x14` | `-` |
| `mov` at 0x1144b9 | store | 4 | `(unknown)` | `[sp-20]` |
| `mov` at 0x1144c0 | load | 4 | `sp -0x14` | `-` |
| `mov` at 0x1144c3 | store | 4 | `(unknown)` | `[sp-20]` |
| `mov` at 0x1144cc | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1144d0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1144dc | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1144e0 | store | 4 | `(unknown)` | `0xb` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x114358 | `_strtok_r 0x17e358` | `0x0` | `0x113beb` | `[sp+4]` | `ebx` | `esi` | `edi` |
| `call` at 0x1143a8 | `_sprintf 0x171680` | `?` | `0x11430c` | `?` | `?` | `?` | `?` |
| `call` at 0x1143d9 | `_ffscRackIDToString 0x12594c` | `?` | `[sp-24]` | `?` | `?` | `?` | `?` |
| `call` at 0x1143f2 | `_sprintf 0x171680` | `?` | `0x114314` | `[sp-24]` | `?` | `?` | `?` |
| `call` at 0x114421 | `_ffscConvertToUpperCase 0x12543c` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11442f | `_strcmp 0x17de80` | `ret(_strtok_r)` | `0x11431a` | `?` | `?` | `?` | `?` |
| `call` at 0x114440 | `_ffscParseRackID 0x1257bc` | `ret(_strtok_r)` | `sp-20` | `?` | `?` | `?` | `?` |
| `call` at 0x114461 | `_sprintf 0x171680` | `?` | `0x114029` | `?` | `?` | `?` | `?` |
| `call` at 0x1144d4 | `_sprintf 0x171680` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1144f0 | `_elscUpdateSaveInfo 0x121540` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x11434a | `(([([0x1ee564] + [sp+8])+4]:1 & 0xff) & ([([0x1ee564] + [sp+8])+4]:1 & 0xff)) == 0x0` | `(([([0x1ee564] + [sp+8])+4]:1 & 0xff) & ([([0x1ee564] + [sp+8])+4]:1 & 0xff)) != 0x0` |
| `jne` at 0x114364 | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `jb` at 0x114391 | `C == 1` | `not (C == 1)` |
| `jge` at 0x114399 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x1143c4 | `[sp+12] == 0x0` | `[sp+12] != 0x0` |
| `jbe` at 0x11440c | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |
| `je` at 0x114439 | `(ret(_strcmp) & ret(_strcmp)) == 0x0` | `(ret(_strcmp) & ret(_strcmp)) != 0x0` |
| `je` at 0x11444a | `(ret(_ffscParseRackID) & ret(_ffscParseRackID)) == 0x0` | `(ret(_ffscParseRackID) & ret(_ffscParseRackID)) != 0x0` |
| `jb` at 0x114495 | `C == 1` | `not (C == 1)` |
| `jge` at 0x11449d | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x1144b4 | `[sp+12] == 0x0` | `[sp+12] != 0x0` |
| `jbe` at 0x1144ee | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x114503 | `eax` | `?` |

## `_cmdEMSG_text`

- `_cmdEMSG_text`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x114520 to 0x114730.
  Established. `_userSendCommand` carries `[sp+8]`, `sp-248`, then unknown candidate slots (`call`
  at 0x1145a7); its nonzero result takes the alternate arm (`jne` at 0x1145b1). Each result arm
  writes four-byte `0xb` at an unresolved target (`mov` at 0x1145da, `mov` at 0x1145fc). The
  formatting site before this call carries `sp-248`, `0x114504`, then unknown slots (`call` at
  0x11458d). Message semantics and destination identities are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x11452b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x11452e | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x114531 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x114534 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x11453a | load | 1 | `([0x1ee564] + [sp+12]) +0x4` | `-` |
| `mov` at 0x114595 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x11459a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1145a1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1145b3 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1145be | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1145c5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1145ce | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1145d3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1145da | store | 4 | `(unknown)` | `0xb` |
| `mov` at 0x1145f0 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1145f5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1145fc | store | 4 | `(unknown)` | `0xb` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x114558 | `_strtok_r 0x17e358` | `0x0` | `0x113c5d` | `[sp+4]` | `ebx` | `esi` | `?` |
| `call` at 0x114576 | `_strspn 0x17e264` | `ret(_strtok_r)` | `0x113beb` | `?` | `?` | `?` | `?` |
| `call` at 0x11458d | `_sprintf 0x171680` | `sp-248` | `0x114504` | `?` | `?` | `?` | `?` |
| `call` at 0x1145a7 | `_userSendCommand 0x12b7d8` | `[sp+8]` | `sp-248` | `?` | `?` | `?` | `?` |
| `call` at 0x1145c9 | `_sprintf 0x171680` | `?` | `0x11450b` | `[0x1ee564]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x114540 | `(([([0x1ee564] + [sp+12])+4]:1 & 0xff) & ([([0x1ee564] + [sp+12])+4]:1 & 0xff)) != 0x0` | `(([([0x1ee564] + [sp+12])+4]:1 & 0xff) & ([([0x1ee564] + [sp+12])+4]:1 & 0xff)) == 0x0` |
| `jne` at 0x114564 | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `jne` at 0x1145b1 | `(ret(_userSendCommand) & ret(_userSendCommand)) != 0x0` | `(ret(_userSendCommand) & ret(_userSendCommand)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x114610 | `eax` | `?` |

## `_cmdUNSTEAL`

- `_cmdUNSTEAL`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x116b10 to 0x116c70.
  Established. The signed branch takes its later arm when `[[sp+8]+32] > 0x1f3` (`jg` at 0x116b7e).
  The later branch falls through when `[0x1d9b9c] == [[sp+8]+148]`
  (`jne` at 0x116bec), reaching a four-byte zero store at absolute `0x1d9b9c` (`mov` at 0x116bee). `_userChangeInputMode` then has unknown candidate zero followed by `0x1`, `0x100000`
  (`call` at 0x116c00). The final formatting site retains unknown candidate zero followed by
  `0x11622f`, `0x8000`, and unknown later slots (`call` at 0x116c61). The exit value is zero (`ret`
  at 0x116c6e). Ownership and mode-switch behavior are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x116b14 | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x116b17 | load | 4 | `abs 0x1da96c` | `-` |
| `mov` at 0x116b1f | load | 4 | `abs 0x1da970` | `-` |
| `cmp` at 0x116b30 | load | 4 | `abs 0x1da970` | `-` |
| `mov` at 0x116b4b | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x116b50 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116b57 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116b70 | load | 4 | `abs 0x1da96c` | `-` |
| `mov` at 0x116b76 | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x116b85 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x116b8a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116b91 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116ba0 | load | 4 | `[sp+8] +0x94` | `-` |
| `cmp` at 0x116ba6 | load | 4 | `abs 0x1d9b9c` | `-` |
| `mov` at 0x116bc1 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x116bc6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116bcd | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116be0 | load | 4 | `[sp+8] +0x94` | `-` |
| `cmp` at 0x116be6 | load | 4 | `abs 0x1d9b9c` | `-` |
| `mov` at 0x116bee | store | 4 | `abs 0x1d9b9c` | `0x0` |
| `mov` at 0x116c11 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x116c16 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116c1d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116c30 | load | 4 | `[sp+8] +0x94` | `-` |
| `mov` at 0x116c36 | load | 4 | `[[sp+8]+148] +0x10` | `-` |
| `mov` at 0x116c43 | load | 4 | `[[sp+8]+148] +0x10` | `-` |
| `mov` at 0x116c51 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x116c56 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116c5d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116c68 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x116b3e | `_ffscMsg 0x11ebd0` | `0x116aa2` | `[sp+8]` | `ebx` | `ebp` | `?` | `?` |
| `call` at 0x116b5b | `_sprintf 0x171680` | `?` | `0x115982` | `?` | `?` | `?` | `?` |
| `call` at 0x116b95 | `_sprintf 0x171680` | `?` | `0x115b30` | `ebx` | `ebp` | `?` | `?` |
| `call` at 0x116bb4 | `_ffscMsg 0x11ebd0` | `0x116acb` | `[sp+8]` | `ebx` | `ebp` | `?` | `?` |
| `call` at 0x116bd1 | `_sprintf 0x171680` | `?` | `0x116af3` | `?` | `?` | `?` | `?` |
| `call` at 0x116c00 | `_userChangeInputMode 0x12aa48` | `?` | `0x1` | `0x100000` | `?` | `?` | `?` |
| `call` at 0x116c21 | `_sprintf 0x171680` | `?` | `0x115b40` | `?` | `?` | `?` | `?` |
| `call` at 0x116c61 | `_sprintf 0x171680` | `?` | `0x11622f` | `0x8000` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x116b1d | `[0x1da96c] != [sp+8]` | `[0x1da96c] == [sp+8]` |
| `je` at 0x116b36 | `[0x1da970] == [sp+8]` | `[0x1da970] != [sp+8]` |
| `jg` at 0x116b7e | `[[sp+8]+32] > 0x1f3 (signed, 32-bit)` | `[[sp+8]+32] <= 0x1f3 (signed, 32-bit)` |
| `je` at 0x116bac | `[0x1d9b9c] == [[sp+8]+148]` | `[0x1d9b9c] != [[sp+8]+148]` |
| `jne` at 0x116bec | `[0x1d9b9c] != [[sp+8]+148]` | `[0x1d9b9c] == [[sp+8]+148]` |
| `je` at 0x116c0a | `(ret(_userChangeInputMode) & ret(_userChangeInputMode)) == 0x0` | `(ret(_userChangeInputMode) & ret(_userChangeInputMode)) != 0x0` |
| `je` at 0x116c41 | `[[[sp+8]+148]+16] == 0x1` | `[[[sp+8]+148]+16] != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x116c6e | `eax` | `0x0` |
