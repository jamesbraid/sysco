# ARM Linux 2.6 request routing and remaining configuration events in 1.44.0

Read from `6.5.30/usr/cpu/firmware/sysco/l1.bin`, package `6.5.30:l1#6` (version 1.44.0),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0xa936a8`.
Carrier SHA-256: `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
Package SHA-256: `e57ab8ecd48858e39ecdd0050a23e444a491c35e87f9d8ebb48881fc25229a06`.
Inner executable SHA-256: `b33154e53b00531623db6ff365e80b1c1825bc9427961c59fdc132c24f0c819e`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1629656 bytes.
Segment SHA-256: `f6491062ef0a2afd254f8510f82b4814c563c9256691f9a2b393cf6e84ba7fd8`. Analysed routine spans use the exact
ELF function symbols and sizes. They can include trailing literal words after a return.

`sp` denotes the entry stack pointer. Register expressions such as `r0` denote entry
values, and brackets denote loaded values. Decimal numbers inside expressions retain the
exact tool notation. Call candidates use the assumed ARM AAPCS
register locations r0 through r3 with unknown arity. Stack stores appear in the access tables.
Unknown values and predicates include unsupported instruction effects and merged paths.
Numeric call targets have unknown callee names in the flat-image analysis. Device roles
and pointee meanings remain unknown unless the body establishes them. Tables include the
explicit memory accesses reported on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
Two computed transfers in L1ReqIRouter leave unresolved edges; its tables cover known edges only.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-request-routing-events-facts.txt) retain all tool sections.

## `L1CfgEventPartition`

- `L1CfgEventPartition`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa4678 to 0xa485c.
  Established. The call to unnamed 0xa3c7c has its result saved at entry sp minus 0x18 (`bl` at 0xa4690, `str` at 0xa4698).
  A word store writes the word at entry sp minus 20 into memory at that result plus 0x3c (`bl` at 0xa4690, `str` at 0xa46b8).
  A later word load reads memory at the value held at entry sp minus 24 plus 0x10 (`ldr` at 0xa46d4).
  The call to unnamed 0xa4298 has printed r0-r3 candidates six, zero, zero and unknown, with the software field meanings unknown (`bl` at 0xa46cc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa4688 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa4698 | store | 4 | `sp -0x18` | `ret(0xa3c7c)` |
| `ldr` at 0xa469c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa46a0 | load | 4 | `ret(0xa3c7c) +0x3c` | `-` |
| `ldr` at 0xa46a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa46b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa46b4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa46b8 | store | 4 | `ret(0xa3c7c) +0x3c` | `[sp-20]` |
| `ldr` at 0xa46d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa46d4 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xa46e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa46e4 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xa46f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa46f4 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xa4700 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4704 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xa4710 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4714 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xa4720 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4724 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xa4730 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4734 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xa4740 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4744 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xa4750 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4754 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xa4760 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4764 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xa4770 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4774 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xa4780 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4784 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xa4790 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4794 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xa47a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa47a4 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xa47b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa47b4 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xa47c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa47c4 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xa47d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa47d4 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xa47e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa47e4 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xa47f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa47f4 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xa4800 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4804 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xa4810 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4814 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xa4820 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4824 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldm` at 0xa4858 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa4858 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa4858 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa4690 | `(unknown callee) 0xa3c7c` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0xa46bc | `(unknown callee) 0x9662c` | `ret(0xa3c7c)` | `?` | `ret(0xa3c7c)` | `[sp-20]` |
| `bl` at 0xa46cc | `(unknown callee) 0xa4298` | `0x6` | `0x0` | `0x0` | `?` |
| `bl` at 0xa4838 | `(unknown callee) 0xa1144` | `0x1` | `?` | `?` | `[[sp-24]+16]` |
| `bl` at 0xa4840 | `(unknown callee) 0xa1144` | `0x3` | `?` | `?` | `?` |
| `bl` at 0xa4848 | `(unknown callee) 0xa1144` | `0xe` | `?` | `?` | `?` |
| `bl` at 0xa4850 | `(unknown callee) 0xa1144` | `0xf` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa46ac | `[ret(0xa3c7c)+60] == [sp-20]` | `[ret(0xa3c7c)+60] != [sp-20]` |
| `beq` at 0xa46dc | `[[sp-24]+16] == 0x43` | `[[sp-24]+16] != 0x43` |
| `beq` at 0xa46ec | `[[sp-24]+16] == 0x33` | `[[sp-24]+16] != 0x33` |
| `beq` at 0xa46fc | `[[sp-24]+16] == 0x35` | `[[sp-24]+16] != 0x35` |
| `beq` at 0xa470c | `[[sp-24]+16] == 0x34` | `[[sp-24]+16] != 0x34` |
| `beq` at 0xa471c | `[[sp-24]+16] == 0x41` | `[[sp-24]+16] != 0x41` |
| `beq` at 0xa472c | `[[sp-24]+16] == 0x36` | `[[sp-24]+16] != 0x36` |
| `beq` at 0xa473c | `[[sp-24]+16] == 0x39` | `[[sp-24]+16] != 0x39` |
| `beq` at 0xa474c | `[[sp-24]+16] == 0x37` | `[[sp-24]+16] != 0x37` |
| `beq` at 0xa475c | `[[sp-24]+16] == 0x38` | `[[sp-24]+16] != 0x38` |
| `beq` at 0xa476c | `[[sp-24]+16] == 0x30` | `[[sp-24]+16] != 0x30` |
| `beq` at 0xa477c | `[[sp-24]+16] == 0x51` | `[[sp-24]+16] != 0x51` |
| `beq` at 0xa478c | `[[sp-24]+16] == 0x40` | `[[sp-24]+16] != 0x40` |
| `beq` at 0xa479c | `[[sp-24]+16] == 0x4f` | `[[sp-24]+16] != 0x4f` |
| `beq` at 0xa47ac | `[[sp-24]+16] == 0x42` | `[[sp-24]+16] != 0x42` |
| `beq` at 0xa47bc | `[[sp-24]+16] == 0x46` | `[[sp-24]+16] != 0x46` |
| `beq` at 0xa47cc | `[[sp-24]+16] == 0x4c` | `[[sp-24]+16] != 0x4c` |
| `beq` at 0xa47dc | `[[sp-24]+16] == 0x4d` | `[[sp-24]+16] != 0x4d` |
| `beq` at 0xa47ec | `[[sp-24]+16] == 0x24` | `[[sp-24]+16] != 0x24` |
| `beq` at 0xa47fc | `[[sp-24]+16] == 0x2c` | `[[sp-24]+16] != 0x2c` |
| `beq` at 0xa480c | `[[sp-24]+16] == 0x7d` | `[[sp-24]+16] != 0x7d` |
| `beq` at 0xa481c | `[[sp-24]+16] == 0x2e` | `[[sp-24]+16] != 0x2e` |
| `beq` at 0xa482c | `[[sp-24]+16] == 0x7b` | `[[sp-24]+16] != 0x7b` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa4858 | `r0` | `?` |

## `L1CfgEventSystemSN`

- `L1CfgEventSystemSN`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa485c to 0xa4910.
  Established. The call to unnamed 0xa3c7c has its result saved at entry sp minus 0x18 (`bl` at 0xa4874, `str` at 0xa487c).
  A separate word store writes zero at entry sp minus 0x4c (`str` at 0xa4884).
  The call to unnamed 0xa4298 has printed r0-r3 candidates seven, zero, zero and unknown (`bl` at 0xa48fc).
  The contents represented by the saved value remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa486c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa487c | store | 4 | `sp -0x18` | `ret(0xa3c7c)` |
| `str` at 0xa4884 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0xa4898 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa48ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa48c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa48e4 | load | 4 | `abs 0xa490c` | `-` |
| `ldm` at 0xa4908 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa4908 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa4908 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa4874 | `(unknown callee) 0xa3c7c` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0xa4888 | `(unknown callee) 0xa451c` | `ret(0xa3c7c)` | `?` | `?` | `0x0` |
| `bl` at 0xa4890 | `(unknown callee) 0x28230` | `0xd` | `?` | `?` | `?` |
| `bl` at 0xa48a8 | `(unknown callee) 0x20634` | `[sp-24]+76` | `ret(0x28230)` | `ret(0x28230)` | `[sp-24]+76` |
| `bl` at 0xa48bc | `(unknown callee) 0x28230` | `0xd` | `?` | `?` | `[sp-20]` |
| `bl` at 0xa48d4 | `(unknown callee) 0x20634` | `[sp-24]+100` | `ret(0x28230)` | `[sp-24]+100` | `ret(0x28230)` |
| `bl` at 0xa48dc | `(unknown callee) 0x28230` | `0xd` | `?` | `?` | `?` |
| `bl` at 0xa48ec | `(unknown callee) 0x20634` | `0x31b96c` | `ret(0x28230)` | `?` | `ret(0x28230)` |
| `bl` at 0xa48fc | `(unknown callee) 0xa4298` | `0x7` | `0x0` | `0x0` | `?` |
| `bl` at 0xa4900 | `(unknown callee) 0x9a0d4` | `ret(0xa4298)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa48b4 | `[sp-20] == 0x0` | `[sp-20] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa4908 | `r0` | `ret(0x9a0d4)` |

## `L1CfgEventEnvironmental`

- `L1CfgEventEnvironmental`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa4910 to 0xa496c.
  Established. The call to unnamed 0xa3c7c has its result saved at entry sp minus 0x18 (`bl` at 0xa4928, `str` at 0xa4930).
  A word load reads memory at that result plus 0x5c (`bl` at 0xa4928, `str` at 0xa4930, `ldr` at 0xa493c).
  A later word store writes the word at entry sp minus 20 at the result plus 0x5c (`bl` at 0xa4928, `str` at 0xa4950).
  The call to unnamed 0xa4298 has printed candidates eight, zero, zero and the word at entry sp minus 20, whose physical meaning is unknown (`bl` at 0xa4960).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa4920 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa4930 | store | 4 | `sp -0x18` | `ret(0xa3c7c)` |
| `ldr` at 0xa4934 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4938 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa493c | load | 4 | `ret(0xa3c7c) +0x5c` | `-` |
| `ldr` at 0xa4948 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa494c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa4950 | store | 4 | `ret(0xa3c7c) +0x5c` | `[sp-20]` |
| `ldm` at 0xa4968 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa4968 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa4968 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa4928 | `(unknown callee) 0xa3c7c` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0xa4960 | `(unknown callee) 0xa4298` | `0x8` | `0x0` | `0x0` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa4944 | `[sp-20] == [ret(0xa3c7c)+92]` | `[sp-20] != [ret(0xa3c7c)+92]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa4968 | `r0` | `?` |

## `L1CfgEventNVRAM`

- `L1CfgEventNVRAM`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa496c to 0xa499c.
  Established. A word store saves entry r0 at entry sp minus 0x14 (`str` at 0xa497c).
  Another saves entry r1 at entry sp minus 0x18 (`str` at 0xa4980).
  The call to unnamed 0xa4298 has printed r0-r3 candidates 0x66, entry r0, entry r1 and entry r3, with argument meanings unknown (`bl` at 0xa4990).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa497c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa4980 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xa4988 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa498c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xa4998 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa4998 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa4998 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa4990 | `(unknown callee) 0xa4298` | `0x66` | `r0` | `r1` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa4998 | `r0` | `ret(0xa4298)` |

## `AutofanEnvInit`

- `AutofanEnvInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x38a40 to 0x38aac.
  Established. A word store writes the word at entry sp minus 24 at entry sp minus 0x30 (`str` at 0x38a60).
  Four other word stores write 0x800, ten, ten and nine at entry sp minus 0x2c, minus 0x28, minus 0x24 and minus 0x20 respectively (`str` at 0x38a68, `str` at 0x38a70, `str` at 0x38a78, `str` at 0x38a80).
  The call to unnamed 0x22540 has printed candidates 0x1a5a24, 0x159be8, 0x36f7c and the word at entry sp minus 20 (`bl` at 0x38a94).
  The use of those stored constants and addresses remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x38a50 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x38a54 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x38a5c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x38a60 | store | 4 | `sp -0x30` | `[sp-24]` |
| `str` at 0x38a68 | store | 4 | `sp -0x2c` | `0x800` |
| `str` at 0x38a70 | store | 4 | `sp -0x28` | `0xa` |
| `str` at 0x38a78 | store | 4 | `sp -0x24` | `0xa` |
| `str` at 0x38a80 | store | 4 | `sp -0x20` | `0x9` |
| `ldr` at 0x38a84 | load | 4 | `abs 0x38aa0` | `-` |
| `ldr` at 0x38a88 | load | 4 | `abs 0x38aa4` | `-` |
| `ldr` at 0x38a8c | load | 4 | `abs 0x38aa8` | `-` |
| `ldr` at 0x38a90 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x38a9c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x38a9c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x38a9c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x38a58 | `(unknown callee) 0xb5ba4` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x38a94 | `(unknown callee) 0x22540` | `0x1a5a24` | `0x159be8` | `0x36f7c` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x38a9c | `r0` | `ret(0x22540)` |

## `IRouterGetQueueSuspendTime`

- `IRouterGetQueueSuspendTime`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfbb98 to 0xfbbe0.
  Established. Word loads read absolute addresses 0x1a04a4 and 0x1a04a8 (`ldr` at 0xfbbb8, `ldr` at 0xfbbc8).
  Word stores write those loaded values into memory at entry r0 and into memory addressed by the word at entry sp minus 24 respectively (`str` at 0xfbbbc, `str` at 0xfbbcc).
  The return record preserves entry r0, while the meanings of both software fields remain unknown (`ldm` at 0xfbbd4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfbba8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xfbbac | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xfbbb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfbbb4 | load | 4 | `abs 0xfbbd8` | `-` |
| `ldr` at 0xfbbb8 | load | 4 | `abs 0x1a04a4` | `-` |
| `str` at 0xfbbbc | store | 4 | `r0 +0x0` | `[0x1a04a4]` |
| `ldr` at 0xfbbc0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfbbc4 | load | 4 | `abs 0xfbbdc` | `-` |
| `ldr` at 0xfbbc8 | load | 4 | `abs 0x1a04a8` | `-` |
| `str` at 0xfbbcc | store | 4 | `[sp-24] +0x0` | `[0x1a04a8]` |
| `ldm` at 0xfbbd4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xfbbd4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xfbbd4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xfbbd4 | `r0` | `r0` |

## `L1ReqInit`

- `L1ReqInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbedb4 to 0xbeea8.
  Established. The call to unnamed 0xfa1b8 has printed candidates 0x31b67c, 0xffff006, one and 0xbeea8 (`bl` at 0xbee24);
  its result is saved at entry sp minus 0x14 and later at entry sp minus 0x18 (`bl` at 0xbee24, `str` at 0xbee2c, `str` at 0xbee48).
  A word store increases the loaded value at absolute 0x1b0ce4 by one, whose software role is unknown (`str` at 0xbee70).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xbedc4 | load | 4 | `abs 0xbee7c` | `-` |
| `ldr` at 0xbedc8 | load | 4 | `abs 0x1b0ce4` | `-` |
| `ldr` at 0xbedd4 | load | 4 | `abs 0xbee80` | `-` |
| `str` at 0xbedd8 | store | 4 | `sp -0x1c` | `0x16d8cc` |
| `ldr` at 0xbede0 | load | 4 | `abs 0xbee84` | `-` |
| `ldr` at 0xbede4 | load | 4 | `abs 0xbee88` | `-` |
| `ldr` at 0xbedf0 | load | 4 | `abs 0xbee8c` | `-` |
| `ldr` at 0xbedf4 | load | 4 | `abs 0xbee90` | `-` |
| `ldr` at 0xbee04 | load | 4 | `abs 0xbee94` | `-` |
| `ldr` at 0xbee14 | load | 4 | `abs 0xbee94` | `-` |
| `ldr` at 0xbee18 | load | 4 | `abs 0xbee98` | `-` |
| `ldr` at 0xbee20 | load | 4 | `abs 0xbee9c` | `-` |
| `str` at 0xbee2c | store | 4 | `sp -0x14` | `ret(0xfa1b8)` |
| `ldr` at 0xbee30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbee3c | load | 4 | `abs 0xbeea0` | `-` |
| `str` at 0xbee40 | store | 4 | `sp -0x1c` | `0x16d90c` |
| `ldr` at 0xbee44 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbee48 | store | 4 | `sp -0x18` | `ret(0xfa1b8)` |
| `ldr` at 0xbee50 | load | 4 | `abs 0xbee84` | `-` |
| `ldr` at 0xbee54 | load | 4 | `abs 0xbeea4` | `-` |
| `ldr` at 0xbee60 | load | 4 | `abs 0xbee7c` | `-` |
| `ldr` at 0xbee64 | load | 4 | `abs 0xbee7c` | `-` |
| `ldr` at 0xbee68 | load | 4 | `abs 0x1b0ce4` | `-` |
| `str` at 0xbee70 | store | 4 | `abs 0x1b0ce4` | `[0x1b0ce4]+1` |
| `ldm` at 0xbee78 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbee78 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbee78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbedec | `(unknown callee) 0x22070` | `0x9` | `0x16d8e8` | `0x202` | `0x0` |
| `bl` at 0xbee00 | `(unknown callee) 0x22b84` | `0x31b628` | `0x16d900` | `0x1` | `0x9` |
| `bl` at 0xbee10 | `(unknown callee) 0xfb5cc` | `0x31b67c` | `0x6` | `0x0` | `?` |
| `bl` at 0xbee24 | `(unknown callee) 0xfa1b8` | `0x31b67c` | `0xffff006` | `0x1` | `0xbeea8` |
| `bl` at 0xbee5c | `(unknown callee) 0x22070` | `0x9` | `0x16d8e8` | `0x216` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xbedd0 | `[0x1b0ce4] == 0x0` | `[0x1b0ce4] != 0x0` |
| `beq` at 0xbee38 | `ret(0xfa1b8) == 0x0` | `ret(0xfa1b8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbee78 | `r0` | `?` |

## `L1ReqIRouter`

- `L1ReqIRouter`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbeea8 to 0xc4b5c.
  Established. A byte load reads memory at the value held at entry sp minus 28 plus 0x20 (`ldrb` at 0xbef28).
  The call to unnamed 0x22ca0 has its result stored at entry sp minus 0x2a4 (`bl` at 0xbeee8, `str` at 0xbeef0).
  The indirect transfer at 0xc0234 is unresolved when the word at entry sp minus 736 is at most four unsigned (`ldrls` at 0xc0234).
  The separate indirect transfer at 0xc08c0 is unresolved when subtracting 36 from the word at entry sp minus 680 gives a value at most 0x59 in unsigned 32-bit arithmetic (`ldrls` at 0xc08c0).
  The meaning of those software values and effects along omitted edges remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbeebc | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0xbeec4 | store | 4 | `sp -0x2a0` | `0x0` |
| `str` at 0xbeecc | store | 4 | `sp -0x2a4` | `0x0` |
| `str` at 0xbeedc | store | 4 | `sp -0x2a8` | `?` |
| `ldr` at 0xbeee0 | load | 4 | `abs 0xbfec8` | `-` |
| `str` at 0xbeef0 | store | 4 | `sp -0x2a4` | `ret(0x22ca0)` |
| `ldr` at 0xbeef4 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0xbef00 | load | 4 | `abs 0xbfecc` | `-` |
| `str` at 0xbef04 | store | 4 | `sp -0x6b0` | `0x16d930` |
| `ldr` at 0xbef08 | load | 4 | `sp -0x2a4` | `-` |
| `str` at 0xbef0c | store | 4 | `sp -0x6ac` | `ret(0x22ca0)` |
| `ldr` at 0xbef14 | load | 4 | `abs 0xbfed0` | `-` |
| `ldr` at 0xbef18 | load | 4 | `abs 0xbfed4` | `-` |
| `ldr` at 0xbef24 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbef28 | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0xbef50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbef54 | load | 4 | `[sp-28] +0x1c` | `-` |
| `str` at 0xbef58 | store | 4 | `sp -0x2ac` | `[[sp-28]+28]` |
| `ldr` at 0xbef5c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbef60 | load | 1 | `[sp-28] +0x24` | `-` |
| `ldr` at 0xbef70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbef74 | load | 4 | `[sp-28] +0x28` | `-` |
| `str` at 0xbef78 | store | 4 | `sp -0x540` | `[[sp-28]+40]` |
| `ldr` at 0xbef80 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbef8c | store | 4 | `sp -0x540` | `ret(0xfde7c)` |
| `ldr` at 0xbef90 | load | 4 | `sp -0x540` | `-` |
| `str` at 0xbef94 | store | 4 | `sp -0x2b0` | `[sp-1344]` |
| `str` at 0xbef9c | store | 4 | `sp -0x544` | `?` |
| `ldr` at 0xbefb0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbefb4 | load | 1 | `[sp-28] +0x17` | `-` |
| `strb` at 0xbefc4 | store | 1 | `sp -0x545` | `?` |
| `strb` at 0xbefd4 | store | 1 | `sp -0x545` | `0x0` |
| `ldrb` at 0xbefdc | load | 1 | `sp -0x545` | `-` |
| `ldr` at 0xbefe4 | load | 4 | `sp -0x544` | `-` |
| `strb` at 0xbefe8 | store | 1 | `(sp-24 + [sp-1348]) +0x0` | `?` |
| `ldr` at 0xbefec | load | 4 | `abs 0xbff5c` | `-` |
| `str` at 0xbeff0 | store | 4 | `sp -0x54c` | `0xfffffd66` |
| `ldr` at 0xbeff4 | load | 4 | `sp -0x2ac` | `-` |
| `ldr` at 0xbf000 | load | 4 | `sp -0x2ac` | `-` |
| `str` at 0xbf008 | store | 4 | `sp -0x550` | `?` |
| `ldr` at 0xbf00c | load | 4 | `sp -0x550` | `-` |
| `str` at 0xbf014 | store | 4 | `sp -0x550` | `?` |
| `str` at 0xbf020 | store | 4 | `sp -0x550` | `?` |
| `ldr` at 0xbf024 | load | 4 | `sp -0x550` | `-` |
| `ldr` at 0xbf034 | load | 4 | `sp -0x54c` | `-` |
| `strb` at 0xbf038 | store | 1 | `(sp-24 + [sp-1356]) +0x0` | `(ret(0xb7e54) & 0xff)` |
| `ldr` at 0xbf03c | load | 4 | `abs 0xbfed8` | `-` |
| `str` at 0xbf040 | store | 4 | `sp -0x554` | `0xfffffd65` |
| `ldr` at 0xbf044 | load | 4 | `sp -0x2ac` | `-` |
| `ldr` at 0xbf050 | load | 4 | `sp -0x2ac` | `-` |
| `str` at 0xbf058 | store | 4 | `sp -0x558` | `?` |
| `ldr` at 0xbf05c | load | 4 | `sp -0x558` | `-` |
| `str` at 0xbf064 | store | 4 | `sp -0x558` | `?` |
| `str` at 0xbf070 | store | 4 | `sp -0x558` | `?` |
| `ldr` at 0xbf074 | load | 4 | `sp -0x558` | `-` |
| `ldr` at 0xbf084 | load | 4 | `sp -0x554` | `-` |
| `strb` at 0xbf088 | store | 1 | `(sp-24 + [sp-1364]) +0x0` | `(ret(0xb7f6c) & 0xff)` |
| `ldrb` at 0xbf094 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbf098 | load | 4 | `abs 0xbff5c` | `-` |
| `ldrb` at 0xbf0a0 | load | 1 | `sp -0x2b2` | `-` |
| `str` at 0xbf0b8 | store | 4 | `sp -0x2b8` | `ret(0x81b18)` |
| `ldr` at 0xbf0bc | load | 4 | `abs 0xbff60` | `-` |
| `ldr` at 0xbf0c0 | load | 4 | `abs 0x1a6468` | `-` |
| `ldrb` at 0xbf0d4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbf0d8 | load | 4 | `abs 0xbff5c` | `-` |
| `ldrb` at 0xbf0e0 | load | 1 | `sp -0x2b2` | `-` |
| `ldr` at 0xbf0e4 | load | 4 | `abs 0xbfed8` | `-` |
| `ldrb` at 0xbf0ec | load | 1 | `sp -0x2b3` | `-` |
| `str` at 0xbf0f0 | store | 4 | `sp -0x6b0` | `[sp-691]:1` |
| `ldr` at 0xbf0f4 | load | 4 | `sp -0x2b0` | `-` |
| `str` at 0xbf0f8 | store | 4 | `sp -0x6ac` | `[sp-688]` |
| `ldr` at 0xbf0fc | load | 4 | `abs 0xbfedc` | `-` |
| `ldr` at 0xbf100 | load | 4 | `sp -0x2ac` | `-` |
| `ldr` at 0xbf124 | load | 4 | `sp -0x2b0` | `-` |
| `str` at 0xbf128 | store | 4 | `sp -0x6a0` | `[sp-688]` |
| `ldr` at 0xbf12c | load | 4 | `abs 0xbfee0` | `-` |
| `ldr` at 0xbf130 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf13c | load | 4 | `abs 0xbfee0` | `-` |
| `ldr` at 0xbf140 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf14c | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf158 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf164 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf170 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf17c | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf188 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf194 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf1a4 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf1b0 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf1c0 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf1cc | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf1d8 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf1e4 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf1f4 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf200 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf210 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf21c | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf228 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf234 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf240 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf24c | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf25c | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf268 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf278 | load | 4 | `abs 0xbfee4` | `-` |
| `ldr` at 0xbf27c | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf288 | load | 4 | `abs 0xbfee8` | `-` |
| `ldr` at 0xbf28c | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf298 | load | 4 | `abs 0xbfeec` | `-` |
| `ldr` at 0xbf29c | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf2ac | load | 4 | `abs 0xbfef0` | `-` |
| `ldr` at 0xbf2b0 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf2bc | load | 4 | `abs 0xbfef0` | `-` |
| `ldr` at 0xbf2c0 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf2d0 | load | 4 | `abs 0xbfef4` | `-` |
| `ldr` at 0xbf2d4 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf2e0 | load | 4 | `abs 0xbfef4` | `-` |
| `ldr` at 0xbf2e4 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf2f0 | load | 4 | `abs 0xbfef8` | `-` |
| `ldr` at 0xbf2f4 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf300 | load | 4 | `abs 0xbfef8` | `-` |
| `ldr` at 0xbf304 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf310 | load | 4 | `abs 0xbfefc` | `-` |
| `ldr` at 0xbf314 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf320 | load | 4 | `abs 0xbfefc` | `-` |
| `ldr` at 0xbf324 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf330 | load | 4 | `abs 0xbff00` | `-` |
| `ldr` at 0xbf334 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf340 | load | 4 | `abs 0xbff04` | `-` |
| `ldr` at 0xbf344 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf354 | load | 4 | `abs 0xbff08` | `-` |
| `ldr` at 0xbf358 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf364 | load | 4 | `abs 0xbff0c` | `-` |
| `ldr` at 0xbf368 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf378 | load | 4 | `abs 0xbff10` | `-` |
| `ldr` at 0xbf37c | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf388 | load | 4 | `abs 0xbff10` | `-` |
| `ldr` at 0xbf38c | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf398 | load | 4 | `abs 0xbff14` | `-` |
| `ldr` at 0xbf39c | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf3a8 | load | 4 | `abs 0xbff18` | `-` |
| `ldr` at 0xbf3ac | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf3bc | load | 4 | `abs 0xbff1c` | `-` |
| `ldr` at 0xbf3c0 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf3cc | load | 4 | `abs 0xbff1c` | `-` |
| `ldr` at 0xbf3d0 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf3dc | load | 4 | `abs 0xbff20` | `-` |
| `ldr` at 0xbf3e0 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf3f0 | load | 4 | `abs 0xbff24` | `-` |
| `ldr` at 0xbf3f4 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf400 | load | 4 | `abs 0xbff24` | `-` |
| `ldr` at 0xbf404 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf410 | load | 4 | `abs 0xbff28` | `-` |
| `ldr` at 0xbf414 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf420 | load | 4 | `abs 0xbff28` | `-` |
| `ldr` at 0xbf424 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf430 | load | 4 | `abs 0xbff2c` | `-` |
| `ldr` at 0xbf434 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf440 | load | 4 | `abs 0xbff30` | `-` |
| `ldr` at 0xbf444 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf454 | load | 4 | `abs 0xbff34` | `-` |
| `ldr` at 0xbf458 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf464 | load | 4 | `abs 0xbff34` | `-` |
| `ldr` at 0xbf468 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf474 | load | 4 | `abs 0xbff38` | `-` |
| `ldr` at 0xbf478 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf488 | load | 4 | `abs 0xbff3c` | `-` |
| `ldr` at 0xbf48c | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf498 | load | 4 | `abs 0xbff3c` | `-` |
| `ldr` at 0xbf49c | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf4a8 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf4b4 | load | 4 | `abs 0xbff40` | `-` |
| `ldr` at 0xbf4b8 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf4c8 | load | 4 | `abs 0xbff44` | `-` |
| `ldr` at 0xbf4cc | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf4d8 | load | 4 | `abs 0xbff44` | `-` |
| `ldr` at 0xbf4dc | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0xbf4e8 | load | 4 | `abs 0xbff48` | `-` |
| `ldr` at 0xbf4ec | load | 4 | `sp -0x6a0` | `-` |
| `str` at 0xbf508 | store | 4 | `sp -0x2c0` | `ret(0x28230)` |
| `ldr` at 0xbf534 | load | 4 | `sp -0x2c0` | `-` |
| `str` at 0xbf560 | store | 4 | `sp -0x2c0` | `?` |
| `ldr` at 0xbf58c | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0xbf594 | load | 4 | `sp -0x2b0` | `-` |
| `ldr` at 0xbf5a0 | load | 4 | `sp -0x2b8` | `-` |
| `str` at 0xbf5b4 | store | 4 | `sp -0x2c8` | `?` |
| `ldr` at 0xbf5b8 | load | 4 | `sp -0x2a8` | `-` |
| `str` at 0xbf5bc | store | 4 | `sp -0x2cc` | `[sp-680]` |
| `ldr` at 0xbf5c0 | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0xbf5d0 | load | 4 | `sp -0x2c8` | `-` |
| `str` at 0xbf5e0 | store | 4 | `sp -0x2cc` | `0x23` |
| `ldr` at 0xbf5f4 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0xbf618 | load | 4 | `sp -0x2b8` | `-` |
| `str` at 0xbf62c | store | 4 | `sp -0x2c8` | `?` |
| `str` at 0xbf63c | store | 4 | `sp -0x2bc` | `ret(0xa3c7c)` |
| `ldr` at 0xbf640 | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0xbf644 | load | 4 | `ret(0xa3c7c) +0x8` | `-` |
| `ldr` at 0xbf64c | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0xbf650 | load | 4 | `ret(0xa3c7c) +0xc` | `-` |
| `str` at 0xbf658 | store | 4 | `sp -0x2c0` | `?` |
| `ldr` at 0xbf65c | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0xbf660 | load | 4 | `ret(0xa3c7c) +0x10` | `-` |
| `ldr` at 0xbf66c | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0xbf67c | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0xbf680 | load | 4 | `ret(0xa3c7c) +0x10` | `-` |
| `str` at 0xbf684 | store | 4 | `sp -0x55c` | `[ret(0xa3c7c)+16]` |
| `str` at 0xbf690 | store | 4 | `sp -0x55c` | `0x23` |
| `ldr` at 0xbf694 | load | 4 | `sp -0x55c` | `-` |
| `str` at 0xbf698 | store | 4 | `sp -0x2cc` | `?` |
| `ldr` at 0xbf6ac | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0xbf6c4 | load | 4 | `sp -0x2cc` | `-` |
| `str` at 0xbf6d8 | store | 4 | `sp -0x2bc` | `ret(0xa3c7c)` |
| `ldr` at 0xbf6dc | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0xbf6e0 | load | 4 | `ret(0xa3c7c) +0x8` | `-` |
| `ldr` at 0xbf6e8 | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0xbf6ec | load | 4 | `ret(0xa3c7c) +0xc` | `-` |
| `str` at 0xbf6f4 | store | 4 | `sp -0x2c0` | `?` |
| `ldr` at 0xbf6f8 | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0xbf6fc | load | 4 | `ret(0xa3c7c) +0x10` | `-` |
| `ldr` at 0xbf708 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0xbf718 | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0xbf71c | load | 4 | `ret(0xa3c7c) +0x10` | `-` |
| `str` at 0xbf720 | store | 4 | `sp -0x560` | `[ret(0xa3c7c)+16]` |
| `str` at 0xbf72c | store | 4 | `sp -0x560` | `0x23` |
| `ldr` at 0xbf730 | load | 4 | `sp -0x560` | `-` |
| `str` at 0xbf734 | store | 4 | `sp -0x2cc` | `?` |
| `ldr` at 0xbf748 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0xbf760 | load | 4 | `sp -0x2cc` | `-` |
| `str` at 0xbf774 | store | 4 | `sp -0x2bc` | `ret(0xa3c7c)` |
| `ldr` at 0xbf778 | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0xbf77c | load | 4 | `ret(0xa3c7c) +0x8` | `-` |
| `ldr` at 0xbf784 | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0xbf788 | load | 4 | `ret(0xa3c7c) +0xc` | `-` |
| `str` at 0xbf790 | store | 4 | `sp -0x2c0` | `?` |
| `ldr` at 0xbf794 | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0xbf798 | load | 4 | `ret(0xa3c7c) +0x10` | `-` |
| `ldr` at 0xbf7a4 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0xbf7b4 | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0xbf7b8 | load | 4 | `ret(0xa3c7c) +0x10` | `-` |
| `str` at 0xbf7bc | store | 4 | `sp -0x564` | `[ret(0xa3c7c)+16]` |
| `str` at 0xbf7c8 | store | 4 | `sp -0x564` | `0x23` |
| `ldr` at 0xbf7cc | load | 4 | `sp -0x564` | `-` |
| `str` at 0xbf7d0 | store | 4 | `sp -0x2cc` | `?` |
| `ldr` at 0xbf7e4 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0xbf7fc | load | 4 | `sp -0x2cc` | `-` |
| `str` at 0xbf810 | store | 4 | `sp -0x2bc` | `ret(0xa3c7c)` |
| `ldr` at 0xbf814 | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0xbf818 | load | 4 | `ret(0xa3c7c) +0x8` | `-` |
| `ldr` at 0xbf820 | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0xbf824 | load | 4 | `ret(0xa3c7c) +0xc` | `-` |
| `str` at 0xbf82c | store | 4 | `sp -0x2c0` | `?` |
| `ldr` at 0xbf830 | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0xbf834 | load | 4 | `ret(0xa3c7c) +0x10` | `-` |
| `ldr` at 0xbf840 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0xbf850 | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0xbf854 | load | 4 | `ret(0xa3c7c) +0x10` | `-` |
| `str` at 0xbf858 | store | 4 | `sp -0x568` | `[ret(0xa3c7c)+16]` |
| `str` at 0xbf864 | store | 4 | `sp -0x568` | `0x23` |
| `ldr` at 0xbf868 | load | 4 | `sp -0x568` | `-` |
| `str` at 0xbf86c | store | 4 | `sp -0x2cc` | `?` |
| `ldr` at 0xbf880 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0xbf898 | load | 4 | `sp -0x2cc` | `-` |
| `str` at 0xbf8c8 | store | 4 | `sp -0x2bc` | `ret(0xa3c7c)` |
| `ldr` at 0xbf8cc | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0xbf8d0 | load | 4 | `ret(0xa3c7c) +0x8` | `-` |
| `ldr` at 0xbf8d8 | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0xbf8dc | load | 4 | `ret(0xa3c7c) +0xc` | `-` |
| `str` at 0xbf8e4 | store | 4 | `sp -0x2c0` | `?` |
| `ldr` at 0xbf8f8 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0xbf908 | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0xbf914 | load | 4 | `[sp-700] +0x10` | `-` |
| `str` at 0xbf928 | store | 4 | `sp -0x2bc` | `ret(0xa3c7c)` |
| `ldr` at 0xbf92c | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0xbf930 | load | 4 | `ret(0xa3c7c) +0x8` | `-` |
| `ldr` at 0xbf938 | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0xbf93c | load | 4 | `ret(0xa3c7c) +0xc` | `-` |
| `str` at 0xbf944 | store | 4 | `sp -0x2c0` | `?` |
| `ldr` at 0xbf958 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0xbf968 | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0xbf974 | load | 4 | `[sp-700] +0x10` | `-` |
| `ldr` at 0xbf980 | load | 4 | `sp -0x2a8` | `-` |
| `str` at 0xbf984 | store | 4 | `sp -0x56c` | `[sp-680]` |
| `ldr` at 0xbf988 | load | 4 | `sp -0x56c` | `-` |
| `ldr` at 0xbf994 | load | 4 | `sp -0x56c` | `-` |
| `ldr` at 0xbf9a0 | load | 4 | `sp -0x56c` | `-` |
| `ldr` at 0xbf9ac | load | 4 | `sp -0x56c` | `-` |
| `ldrb` at 0xbf9c4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbf9c8 | load | 4 | `abs 0xbff5c` | `-` |
| `ldrb` at 0xbf9d0 | load | 1 | `sp -0x2b2` | `-` |
| `str` at 0xbf9e0 | store | 4 | `sp -0x2c4` | `ret(0x6dcc4)` |
| `str` at 0xbf9ec | store | 4 | `sp -0x2c4` | `ret(0x6debc)` |
| `ldr` at 0xbf9f0 | load | 4 | `sp -0x2c4` | `-` |
| `ldr` at 0xbfa30 | load | 4 | `sp -0x2c4` | `-` |
| `str` at 0xbfa4c | store | 4 | `sp -0x6b0` | `0x1` |
| `ldr` at 0xbfa54 | load | 4 | `sp -0x2c4` | `-` |
| `ldr` at 0xbfa80 | load | 4 | `abs 0xbff4c` | `-` |
| `str` at 0xbfa90 | store | 4 | `sp -0x2c0` | `ret(0x20544)` |
| `ldr` at 0xbfaa4 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0xbfaac | load | 4 | `sp -0x2c4` | `-` |
| `str` at 0xbfac8 | store | 4 | `sp -0x2c0` | `ret(0x20544)` |
| `ldr` at 0xbfadc | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0xbfae4 | load | 4 | `sp -0x2c4` | `-` |
| `str` at 0xbfafc | store | 4 | `sp -0x2c0` | `ret(0x20544)` |
| `ldr` at 0xbfb10 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0xbfb1c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbfb20 | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0xbfb48 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbfb4c | load | 1 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0xbfb5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbfb60 | load | 4 | `[sp-28] +0x30` | `-` |
| `str` at 0xbfb64 | store | 4 | `sp -0x570` | `[[sp-28]+48]` |
| `ldr` at 0xbfb6c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbfb78 | store | 4 | `sp -0x570` | `ret(0xfde7c)` |
| `ldrb` at 0xbfb84 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbfb88 | load | 4 | `abs 0xbff5c` | `-` |
| `ldrb` at 0xbfb90 | load | 1 | `sp -0x2b2` | `-` |
| `ldr` at 0xbfb9c | load | 4 | `sp -0x570` | `-` |
| `str` at 0xbfba8 | store | 4 | `sp -0x2d8` | `ret(0xbe1f4)` |
| `str` at 0xbfbd4 | store | 4 | `sp -0x574` | `ret(0x6e190)` |
| `ldrb` at 0xbfbe4 | load | 1 | `(unknown)` | `-` |
| `str` at 0xbfbe8 | store | 4 | `sp -0x574` | `?` |
| `ldr` at 0xbfbf0 | load | 4 | `sp -0x574` | `-` |
| `str` at 0xbfc00 | store | 4 | `sp -0x2dc` | `ret(0x120340)` |
| `str` at 0xbfc2c | store | 4 | `sp -0x578` | `ret(0x6e190)` |
| `ldrb` at 0xbfc3c | load | 1 | `(unknown)` | `-` |
| `str` at 0xbfc40 | store | 4 | `sp -0x578` | `?` |
| `ldr` at 0xbfc44 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0xbfc4c | load | 4 | `sp -0x578` | `-` |
| `ldr` at 0xbfc64 | load | 4 | `abs 0xbff50` | `-` |
| `ldr` at 0xbfc88 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0xbfc8c | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0xbfcd4 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0xbfce8 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0xbfcf8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbfd00 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0xbfd14 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0xbfd1c | load | 4 | `(unknown)` | `-` |
| `str` at 0xbfd20 | store | 4 | `sp -0x57c` | `?` |
| `ldr` at 0xbfd24 | load | 4 | `sp -0x57c` | `-` |
| `ldr` at 0xbfd30 | load | 4 | `sp -0x57c` | `-` |
| `ldr` at 0xbfd48 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0xbfd5c | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0xbfd6c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbfd7c | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0xbfd90 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0xbfda0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbfdb0 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0xbfdc4 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0xbfdd4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbfdfc | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0xbfe10 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0xbfe20 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbfe30 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0xbfe44 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0xbfe54 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbfe64 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0xbfe78 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0xbfe88 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbfe98 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0xbfeac | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0xbfebc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbff74 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0xbff88 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0xbff98 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbffa8 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0xbffbc | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0xbffcc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbffd8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbffdc | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0xc0004 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc0008 | load | 1 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0xc0018 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc001c | load | 4 | `[sp-28] +0x30` | `-` |
| `str` at 0xc0020 | store | 4 | `sp -0x580` | `[[sp-28]+48]` |
| `ldr` at 0xc0028 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc0034 | store | 4 | `sp -0x580` | `ret(0xfde7c)` |
| `ldrb` at 0xc0040 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc0044 | load | 4 | `abs 0xbff5c` | `-` |
| `ldrb` at 0xc004c | load | 1 | `sp -0x2b2` | `-` |
| `ldr` at 0xc0058 | load | 4 | `sp -0x580` | `-` |
| `str` at 0xc0064 | store | 4 | `sp -0x2d8` | `ret(0xbe1f4)` |
| `ldr` at 0xc0068 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc006c | load | 1 | `[sp-28] +0x34` | `-` |
| `ldr` at 0xc007c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0080 | load | 4 | `[sp-28] +0x38` | `-` |
| `str` at 0xc0084 | store | 4 | `sp -0x584` | `[[sp-28]+56]` |
| `ldr` at 0xc008c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc0098 | store | 4 | `sp -0x584` | `ret(0xfde7c)` |
| `ldr` at 0xc009c | load | 4 | `sp -0x584` | `-` |
| `str` at 0xc00a0 | store | 4 | `sp -0x2e0` | `[sp-1412]` |
| `ldr` at 0xc00a4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc00a8 | load | 1 | `[sp-28] +0x3c` | `-` |
| `ldr` at 0xc00b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc00bc | load | 4 | `[sp-28] +0x40` | `-` |
| `str` at 0xc00c0 | store | 4 | `sp -0x588` | `[[sp-28]+64]` |
| `ldr` at 0xc00c8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc00d4 | store | 4 | `sp -0x588` | `ret(0xfde7c)` |
| `ldr` at 0xc00d8 | load | 4 | `sp -0x588` | `-` |
| `str` at 0xc00dc | store | 4 | `sp -0x2e4` | `[sp-1416]` |
| `ldr` at 0xc00e0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc00e4 | load | 1 | `[sp-28] +0x44` | `-` |
| `ldr` at 0xc00f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc00f8 | load | 4 | `[sp-28] +0x48` | `-` |
| `str` at 0xc00fc | store | 4 | `sp -0x58c` | `[[sp-28]+72]` |
| `ldr` at 0xc0104 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc0110 | store | 4 | `sp -0x58c` | `ret(0xfde7c)` |
| `ldr` at 0xc0114 | load | 4 | `sp -0x58c` | `-` |
| `str` at 0xc0118 | store | 4 | `sp -0x2e8` | `[sp-1420]` |
| `str` at 0xc0120 | store | 4 | `sp -0x2ec` | `0x0` |
| `str` at 0xc0128 | store | 4 | `sp -0x2f0` | `0x0` |
| `str` at 0xc0154 | store | 4 | `sp -0x590` | `ret(0x6e190)` |
| `ldrb` at 0xc0164 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc0168 | store | 4 | `sp -0x590` | `?` |
| `ldr` at 0xc0170 | load | 4 | `sp -0x590` | `-` |
| `str` at 0xc0180 | store | 4 | `sp -0x2f4` | `ret(0x120340)` |
| `str` at 0xc01ac | store | 4 | `sp -0x594` | `ret(0x6e190)` |
| `ldrb` at 0xc01bc | load | 1 | `(unknown)` | `-` |
| `str` at 0xc01c0 | store | 4 | `sp -0x594` | `?` |
| `ldr` at 0xc01c4 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0xc01cc | load | 4 | `sp -0x594` | `-` |
| `ldr` at 0xc0200 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0xc0204 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc022c | load | 4 | `sp -0x2e0` | `-` |
| `ldrls` at 0xc0234 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc0670 | load | 4 | `sp -0x2ec` | `-` |
| `ldr` at 0xc067c | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0xc0680 | load | 4 | `sp -0x2f0` | `-` |
| `ldr` at 0xc06a8 | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0xc06ac | load | 4 | `sp -0x2e8` | `-` |
| `ldr` at 0xc06b4 | load | 4 | `sp -0x2f0` | `-` |
| `ldr` at 0xc06c0 | load | 4 | `sp -0x2f0` | `-` |
| `ldr` at 0xc06c4 | load | 4 | `sp -0x2e4` | `-` |
| `str` at 0xc06cc | store | 4 | `sp -0x2e8` | `?` |
| `str` at 0xc06d4 | store | 4 | `sp -0x2f0` | `0x0` |
| `ldr` at 0xc06dc | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0xc06e0 | load | 4 | `sp -0x2e8` | `-` |
| `ldr` at 0xc06e8 | load | 4 | `sp -0x2f0` | `-` |
| `str` at 0xc06f0 | store | 4 | `sp -0x2f0` | `?` |
| `ldr` at 0xc071c | load | 4 | `sp -0x2f0` | `-` |
| `ldr` at 0xc0724 | load | 4 | `sp -0x2e8` | `-` |
| `ldr` at 0xc0738 | load | 4 | `sp -0x2ec` | `-` |
| `ldr` at 0xc073c | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0xc0750 | load | 4 | `sp -0x2e8` | `-` |
| `ldr` at 0xc075c | load | 4 | `abs 0xbff54` | `-` |
| `ldr` at 0xc0760 | load | 4 | `abs 0x318478` | `-` |
| `ldr` at 0xc076c | load | 4 | `abs 0xbff54` | `-` |
| `ldr` at 0xc0770 | load | 4 | `abs 0x318478` | `-` |
| `str` at 0xc0784 | store | 4 | `sp -0x2c4` | `ret(unknown)` |
| `str` at 0xc0794 | store | 4 | `sp -0x2c4` | `ret(0x28230)` |
| `ldr` at 0xc0798 | load | 4 | `sp -0x2c4` | `-` |
| `ldr` at 0xc07e0 | load | 4 | `sp -0x2c4` | `-` |
| `str` at 0xc07fc | store | 4 | `sp -0x6b0` | `0x1` |
| `ldr` at 0xc0804 | load | 4 | `sp -0x2c4` | `-` |
| `str` at 0xc0820 | store | 4 | `sp -0x2c0` | `ret(0x28230)` |
| `ldr` at 0xc084c | load | 4 | `sp -0x2c0` | `-` |
| `str` at 0xc0864 | store | 4 | `sp -0x2c0` | `ret(0x28230)` |
| `ldr` at 0xc0890 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0xc08b4 | load | 4 | `sp -0x2a8` | `-` |
| `ldrls` at 0xc08c0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc0b4c | store | 4 | `sp -0x2c0` | `ret(0x28230)` |
| `ldr` at 0xc0b60 | load | 4 | `sp -0x2c0` | `-` |
| `str` at 0xc0b74 | store | 4 | `sp -0x2c0` | `ret(0x28230)` |
| `ldr` at 0xc0b88 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0xc0b94 | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0xc0ba0 | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0xc0bac | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0xc0bb8 | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0xc0bd8 | load | 4 | `abs 0xbff5c` | `-` |
| `str` at 0xc0bdc | store | 4 | `sp -0x598` | `0xfffffd66` |
| `ldr` at 0xc0bf0 | load | 4 | `abs 0xbff5c` | `-` |
| `ldrb` at 0xc0bf8 | load | 1 | `sp -0x2b2` | `-` |
| `strb` at 0xc0c0c | store | 1 | `sp -0x599` | `(ret(0x3e498) & 0xff)` |
| `ldr` at 0xc0c14 | load | 4 | `abs 0xbff5c` | `-` |
| `ldrb` at 0xc0c1c | load | 1 | `sp -0x2b2` | `-` |
| `strb` at 0xc0c24 | store | 1 | `sp -0x599` | `([sp-690]:1 & 0xff)` |
| `ldrb` at 0xc0c2c | load | 1 | `sp -0x599` | `-` |
| `ldr` at 0xc0c34 | load | 4 | `sp -0x598` | `-` |
| `strb` at 0xc0c38 | store | 1 | `(sp-24 + [sp-1432]) +0x0` | `?` |
| `ldr` at 0xc0c5c | load | 4 | `abs 0xbff5c` | `-` |
| `ldrb` at 0xc0c64 | load | 1 | `sp -0x2b2` | `-` |
| `ldr` at 0xc0c78 | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0xc0c84 | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0xc0c90 | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0xc0c9c | load | 4 | `sp -0x2a8` | `-` |
| `str` at 0xc0cc0 | store | 4 | `sp -0x5a0` | `?` |
| `ldr` at 0xc0cd4 | load | 4 | `abs 0xbff5c` | `-` |
| `ldrb` at 0xc0cdc | load | 1 | `sp -0x2b2` | `-` |
| `strb` at 0xc0cf0 | store | 1 | `sp -0x5a1` | `(ret(0x3e458) & 0xff)` |
| `ldrb` at 0xc0d00 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xc0d08 | store | 1 | `sp -0x5a1` | `?` |
| `ldrb` at 0xc0d10 | load | 1 | `sp -0x5a1` | `-` |
| `ldr` at 0xc0d18 | load | 4 | `sp -0x5a0` | `-` |
| `strb` at 0xc0d1c | store | 1 | `(sp-24 + [sp-1440]) +0x0` | `([sp-1441]:1 & 0xff)` |
| `ldrb` at 0xc0d28 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc0d38 | store | 4 | `sp -0x2a4` | `ret(0x3e2d0)` |
| `ldr` at 0xc0d3c | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0xc0d70 | load | 4 | `sp -0x2a4` | `-` |
| `ldrb` at 0xc0da4 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xc0dbc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc0dc8 | load | 4 | `abs 0xbff64` | `-` |
| `ldr` at 0xc0dcc | load | 4 | `abs 0x31a520` | `-` |
| `ldr` at 0xc0dd4 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xc0dd8 | load | 2 | `(unknown)` | `-` |
| `ldrb` at 0xc0df4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc0e00 | load | 4 | `abs 0xbff64` | `-` |
| `ldr` at 0xc0e04 | load | 4 | `abs 0x31a520` | `-` |
| `ldr` at 0xc0e0c | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xc0e10 | load | 2 | `(unknown)` | `-` |
| `ldrb` at 0xc0e38 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc0e44 | load | 4 | `abs 0xbff64` | `-` |
| `ldr` at 0xc0e48 | load | 4 | `abs 0x31a520` | `-` |
| `ldr` at 0xc0e50 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xc0e54 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc0e68 | load | 4 | `abs 0xbff60` | `-` |
| `ldr` at 0xc0e6c | load | 4 | `abs 0x1a6468` | `-` |
| `ldrb` at 0xc0e80 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc0e84 | store | 4 | `sp -0x5a8` | `?` |
| `ldrb` at 0xc0e90 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc0e9c | load | 4 | `abs 0xbff64` | `-` |
| `ldr` at 0xc0ea0 | load | 4 | `abs 0x31a520` | `-` |
| `ldr` at 0xc0ea8 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xc0eac | load | 2 | `(unknown)` | `-` |
| `ldrb` at 0xc0ec0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc0ecc | load | 4 | `abs 0xbff64` | `-` |
| `ldr` at 0xc0ed0 | load | 4 | `abs 0x31a520` | `-` |
| `ldr` at 0xc0ed8 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xc0edc | load | 2 | `(unknown)` | `-` |
| `str` at 0xc0ee0 | store | 4 | `sp -0x5ac` | `?` |
| `ldrb` at 0xc0ef0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc0efc | load | 4 | `abs 0xbff64` | `-` |
| `ldr` at 0xc0f00 | load | 4 | `abs 0x31a520` | `-` |
| `ldr` at 0xc0f08 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xc0f0c | load | 2 | `(unknown)` | `-` |
| `str` at 0xc0f10 | store | 4 | `sp -0x5ac` | `?` |
| `ldr` at 0xc0f14 | load | 4 | `abs 0xbff68` | `-` |
| `ldr` at 0xc0f18 | load | 4 | `sp -0x5a8` | `-` |
| `ldr` at 0xc0f1c | load | 4 | `sp -0x5ac` | `-` |
| `ldr` at 0xc0f68 | load | 4 | `sp -0x2a8` | `-` |
| `ldrb` at 0xc0f7c | load | 1 | `(unknown)` | `-` |
| `str` at 0xc0f8c | store | 4 | `sp -0x2f0` | `ret(0x7fbc8)` |
| `ldr` at 0xc0f90 | load | 4 | `sp -0x2f0` | `-` |
| `ldr` at 0xc0fb8 | load | 4 | `abs 0xc1f44` | `-` |
| `ldr` at 0xc0fbc | load | 4 | `abs 0x31841c` | `-` |
| `str` at 0xc0fc0 | store | 4 | `sp -0x5b0` | `[0x31841c]` |
| `ldr` at 0xc0fc4 | load | 4 | `abs 0xc1f44` | `-` |
| `ldr` at 0xc0fc8 | load | 4 | `abs 0x31841c` | `-` |
| `str` at 0xc0fd8 | store | 4 | `sp -0x5b0` | `0x1` |
| `ldr` at 0xc0fdc | load | 4 | `sp -0x5b0` | `-` |
| `str` at 0xc0fe0 | store | 4 | `sp -0x2e8` | `?` |
| `ldr` at 0xc0fe4 | load | 4 | `sp -0x2e8` | `-` |
| `str` at 0xc0ff8 | store | 4 | `sp -0x2e4` | `ret(0x201fc)` |
| `ldr` at 0xc0ffc | load | 4 | `sp -0x2e8` | `-` |
| `ldr` at 0xc1004 | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0xc1014 | load | 4 | `sp -0x2e8` | `-` |
| `str` at 0xc1028 | store | 4 | `sp -0x2e0` | `ret(0x201fc)` |
| `ldr` at 0xc102c | load | 4 | `sp -0x2e8` | `-` |
| `ldr` at 0xc1034 | load | 4 | `sp -0x2e0` | `-` |
| `str` at 0xc1048 | store | 4 | `sp -0x2f4` | `0x0` |
| `ldr` at 0xc104c | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc1050 | load | 4 | `sp -0x2f0` | `-` |
| `ldr` at 0xc105c | load | 4 | `abs 0xc1f44` | `-` |
| `ldr` at 0xc1064 | load | 4 | `abs 0x3183f4` | `-` |
| `str` at 0xc106c | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0xc1070 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc1074 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0xc1080 | store | 4 | `sp -0x2d8` | `ret(0x14fe64)` |
| `ldr` at 0xc1084 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc108c | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0xc1098 | load | 4 | `abs 0xc1f44` | `-` |
| `ldr` at 0xc109c | load | 4 | `abs 0x3183f4` | `-` |
| `str` at 0xc10a4 | store | 4 | `sp -0x2fc` | `?` |
| `str` at 0xc10d0 | store | 4 | `sp -0x5b4` | `ret(0x6e190)` |
| `ldrb` at 0xc10e0 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc10e4 | store | 4 | `sp -0x5b4` | `?` |
| `ldr` at 0xc10e8 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc10f0 | load | 4 | `sp -0x5b4` | `-` |
| `ldr` at 0xc1108 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0xc1110 | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0xc1118 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0xc1120 | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0xc112c | load | 4 | `sp -0x2fc` | `-` |
| `ldr` at 0xc1134 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc113c | store | 4 | `(unknown)` | `?` |
| `str` at 0xc1168 | store | 4 | `sp -0x5b8` | `ret(0x6e190)` |
| `ldrb` at 0xc1178 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc117c | store | 4 | `sp -0x5b8` | `?` |
| `ldr` at 0xc1180 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc1188 | load | 4 | `sp -0x5b8` | `-` |
| `ldr` at 0xc11a0 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0xc11a8 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0xc11b0 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0xc11b8 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0xc11c4 | load | 4 | `sp -0x2fc` | `-` |
| `ldr` at 0xc11cc | load | 4 | `(unknown)` | `-` |
| `str` at 0xc11d4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xc11d8 | load | 4 | `sp -0x2f4` | `-` |
| `str` at 0xc11e0 | store | 4 | `sp -0x2f4` | `[sp-756]+1` |
| `ldr` at 0xc11e8 | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0xc11f4 | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0xc1200 | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0xc120c | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0xc1264 | load | 4 | `sp -0x2f0` | `-` |
| `ldr` at 0xc1268 | load | 4 | `sp -0x2e8` | `-` |
| `ldr` at 0xc1284 | load | 4 | `abs 0xc1f48` | `-` |
| `ldr` at 0xc1288 | load | 4 | `abs 0x1a6468` | `-` |
| `ldr` at 0xc1294 | load | 4 | `abs 0xc1f78` | `-` |
| `ldrb` at 0xc129c | load | 1 | `sp -0x2b2` | `-` |
| `ldr` at 0xc12a0 | load | 4 | `sp -0x2f0` | `-` |
| `ldr` at 0xc12a4 | load | 4 | `sp -0x2e8` | `-` |
| `ldr` at 0xc12b0 | load | 4 | `abs 0xc1f4c` | `-` |
| `ldr` at 0xc12c4 | load | 4 | `abs 0xc1f48` | `-` |
| `ldr` at 0xc12c8 | load | 4 | `abs 0x1a6468` | `-` |
| `ldr` at 0xc12d4 | load | 4 | `abs 0xc1f78` | `-` |
| `ldrb` at 0xc12dc | load | 1 | `sp -0x2b2` | `-` |
| `ldr` at 0xc12e0 | load | 4 | `abs 0xc1f78` | `-` |
| `ldrb` at 0xc12e8 | load | 1 | `sp -0x2b2` | `-` |
| `ldr` at 0xc12f4 | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0xc12fc | load | 4 | `abs 0xc1f78` | `-` |
| `ldrb` at 0xc1304 | load | 1 | `sp -0x2b2` | `-` |
| `ldr` at 0xc1310 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0xc1318 | load | 4 | `abs 0xc1f50` | `-` |
| `ldr` at 0xc131c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc1320 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc1330 | load | 4 | `abs 0xc1f78` | `-` |
| `ldrb` at 0xc1338 | load | 1 | `sp -0x2b2` | `-` |
| `ldr` at 0xc1344 | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0xc1354 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc1364 | load | 4 | `abs 0xc1f78` | `-` |
| `ldrb` at 0xc136c | load | 1 | `sp -0x2b2` | `-` |
| `ldr` at 0xc1378 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0xc1388 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc13bc | load | 4 | `sp -0x2e8` | `-` |
| `ldr` at 0xc13d4 | load | 4 | `sp -0x2f0` | `-` |
| `ldr` at 0xc13dc | load | 4 | `abs 0xc1f48` | `-` |
| `ldr` at 0xc13e0 | load | 4 | `abs 0x1a6468` | `-` |
| `ldr` at 0xc13ec | load | 4 | `abs 0xc1f78` | `-` |
| `ldrb` at 0xc13f4 | load | 1 | `sp -0x2b2` | `-` |
| `ldr` at 0xc13f8 | load | 4 | `abs 0xc1f54` | `-` |
| `ldr` at 0xc1400 | load | 4 | `sp -0x2f0` | `-` |
| `ldr` at 0xc1404 | load | 4 | `sp -0x2e8` | `-` |
| `str` at 0xc1410 | store | 4 | `sp -0x2ec` | `0x0` |
| `ldr` at 0xc1414 | load | 4 | `sp -0x2ec` | `-` |
| `ldr` at 0xc1418 | load | 4 | `sp -0x2e8` | `-` |
| `ldr` at 0xc1424 | load | 4 | `abs 0xc1f48` | `-` |
| `ldr` at 0xc1428 | load | 4 | `abs 0x1a6468` | `-` |
| `ldr` at 0xc1434 | load | 4 | `abs 0xc1f78` | `-` |
| `ldrb` at 0xc143c | load | 1 | `sp -0x2b2` | `-` |
| `ldr` at 0xc1440 | load | 4 | `sp -0x2ec` | `-` |
| `ldr` at 0xc1448 | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0xc1450 | load | 4 | `sp -0x2ec` | `-` |
| `ldr` at 0xc1458 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0xc1460 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc1464 | store | 4 | `sp -0x6b0` | `?` |
| `ldr` at 0xc1468 | load | 4 | `abs 0xc1f58` | `-` |
| `ldr` at 0xc146c | load | 4 | `sp -0x2ec` | `-` |
| `ldr` at 0xc1470 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc1480 | load | 4 | `sp -0x2ec` | `-` |
| `ldr` at 0xc1488 | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0xc1498 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc14a8 | load | 4 | `sp -0x2ec` | `-` |
| `ldr` at 0xc14b0 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0xc14c0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc14c8 | load | 4 | `sp -0x2ec` | `-` |
| `str` at 0xc14d0 | store | 4 | `sp -0x2ec` | `[sp-748]+1` |
| `ldr` at 0xc14d8 | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0xc14e0 | load | 4 | `sp -0x2e0` | `-` |
| `str` at 0xc14f0 | store | 4 | `sp -0x2f4` | `sp-872` |
| `str` at 0xc151c | store | 4 | `sp -0x2f0` | `0x0` |
| `ldr` at 0xc1520 | load | 4 | `sp -0x2f0` | `-` |
| `ldr` at 0xc152c | load | 4 | `sp -0x31c` | `-` |
| `str` at 0xc1538 | store | 4 | `sp -0x31c` | `?` |
| `ldr` at 0xc153c | load | 4 | `sp -0x314` | `-` |
| `str` at 0xc1544 | store | 4 | `sp -0x314` | `?` |
| `ldr` at 0xc1548 | load | 4 | `sp -0x314` | `-` |
| `str` at 0xc1550 | store | 4 | `sp -0x314` | `?` |
| `str` at 0xc1558 | store | 4 | `sp -0x310` | `0xc5` |
| `ldr` at 0xc155c | load | 4 | `sp -0x2f0` | `-` |
| `str` at 0xc1568 | store | 4 | `sp -0x2e8` | `?` |
| `ldr` at 0xc156c | load | 4 | `sp -0x2e8` | `-` |
| `ldr` at 0xc1570 | load | 4 | `abs 0xc1f5c` | `-` |
| `ldr` at 0xc157c | load | 4 | `sp -0x2e8` | `-` |
| `ldr` at 0xc1584 | load | 4 | `abs 0xc1f5c` | `-` |
| `str` at 0xc1590 | store | 4 | `sp -0x2e8` | `?` |
| `ldr` at 0xc1594 | load | 4 | `sp -0x2e8` | `-` |
| `ldr` at 0xc1598 | load | 4 | `abs 0xc1f60` | `-` |
| `ldr` at 0xc15a4 | load | 4 | `sp -0x2e8` | `-` |
| `ldr` at 0xc15ac | load | 4 | `abs 0xc1f60` | `-` |
| `str` at 0xc15b8 | store | 4 | `sp -0x2e8` | `?` |
| `ldr` at 0xc15bc | load | 4 | `sp -0x2e8` | `-` |
| `ldr` at 0xc15c0 | load | 4 | `abs 0xc1f64` | `-` |
| `ldr` at 0xc15cc | load | 4 | `sp -0x2e8` | `-` |
| `ldr` at 0xc15d4 | load | 4 | `abs 0xc1f64` | `-` |
| `str` at 0xc15e0 | store | 4 | `sp -0x2e8` | `?` |
| `ldr` at 0xc15e4 | load | 4 | `sp -0x2e8` | `-` |
| `ldr` at 0xc15f4 | load | 4 | `sp -0x2e8` | `-` |
| `str` at 0xc1608 | store | 4 | `sp -0x2e8` | `?` |
| `ldr` at 0xc160c | load | 4 | `sp -0x2e8` | `-` |
| `ldr` at 0xc161c | load | 4 | `sp -0x2e8` | `-` |
| `str` at 0xc1630 | store | 4 | `sp -0x2e8` | `?` |
| `ldr` at 0xc1634 | load | 4 | `sp -0x2e8` | `-` |
| `str` at 0xc1638 | store | 4 | `sp -0x30c` | `[sp-744]` |
| `str` at 0xc1648 | store | 4 | `sp -0x2a4` | `ret(0x1347dc)` |
| `ldr` at 0xc164c | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0xc1658 | load | 4 | `sp -0x304` | `-` |
| `str` at 0xc165c | store | 4 | `sp -0x328` | `[sp-772]` |
| `ldr` at 0xc1660 | load | 4 | `sp -0x300` | `-` |
| `str` at 0xc1664 | store | 4 | `sp -0x324` | `[sp-768]` |
| `ldr` at 0xc1668 | load | 4 | `sp -0x2fc` | `-` |
| `str` at 0xc166c | store | 4 | `sp -0x320` | `[sp-764]` |
| `str` at 0xc1674 | store | 4 | `sp -0x2ec` | `0x0` |
| `ldr` at 0xc1678 | load | 4 | `sp -0x2ec` | `-` |
| `ldr` at 0xc1684 | load | 4 | `sp -0x2ec` | `-` |
| `ldr` at 0xc1690 | load | 4 | `abs 0xc1f68` | `-` |
| `ldr` at 0xc16a4 | load | 4 | `sp -0x2ec` | `-` |
| `ldr` at 0xc16b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc16c8 | load | 4 | `sp -0x2ec` | `-` |
| `ldr` at 0xc16d0 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc16d8 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc16e0 | load | 4 | `sp -0x2ec` | `-` |
| `ldrb` at 0xc16f0 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xc16f8 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xc16fc | load | 4 | `sp -0x2ec` | `-` |
| `str` at 0xc1704 | store | 4 | `sp -0x2ec` | `[sp-748]+1` |
| `ldr` at 0xc1734 | load | 4 | `sp -0x2a4` | `-` |
| `str` at 0xc1740 | store | 4 | `sp -0x2a4` | `0x0` |
| `ldr` at 0xc1748 | load | 4 | `sp -0x2f4` | `-` |
| `str` at 0xc1750 | store | 4 | `sp -0x2f4` | `[sp-756]+8` |
| `ldr` at 0xc1754 | load | 4 | `sp -0x2f0` | `-` |
| `str` at 0xc175c | store | 4 | `sp -0x2f0` | `[sp-752]+1` |
| `ldr` at 0xc1764 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0xc17ac | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc17b0 | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0xc17d8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc17dc | load | 1 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0xc17ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc17f0 | load | 4 | `[sp-28] +0x30` | `-` |
| `str` at 0xc17f4 | store | 4 | `sp -0x5bc` | `[[sp-28]+48]` |
| `ldr` at 0xc17fc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc1808 | store | 4 | `sp -0x5bc` | `ret(0xfde7c)` |
| `ldr` at 0xc1810 | load | 4 | `sp -0x5bc` | `-` |
| `ldr` at 0xc1834 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1838 | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0xc1860 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1864 | load | 1 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0xc1890 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1894 | load | 1 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0xc18a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc18a8 | load | 4 | `[sp-28] +0x30` | `-` |
| `str` at 0xc18ac | store | 4 | `sp -0x5c0` | `[[sp-28]+48]` |
| `ldr` at 0xc18b4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc18c0 | store | 4 | `sp -0x5c0` | `ret(0xfde7c)` |
| `ldr` at 0xc18c8 | load | 4 | `sp -0x5c0` | `-` |
| `ldr` at 0xc18f0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc18f4 | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0xc191c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1920 | load | 1 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0xc1930 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc1934 | load | 4 | `[sp-28] +0x30` | `-` |
| `str` at 0xc1938 | store | 4 | `sp -0x5c4` | `[[sp-28]+48]` |
| `ldr` at 0xc1940 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc194c | store | 4 | `sp -0x5c4` | `ret(0xfde7c)` |
| `ldr` at 0xc1954 | load | 4 | `sp -0x5c4` | `-` |
| `ldr` at 0xc198c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1990 | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0xc19b8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc19bc | load | 1 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0xc19cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc19d0 | load | 4 | `[sp-28] +0x30` | `-` |
| `str` at 0xc19d4 | store | 4 | `sp -0x5c8` | `[[sp-28]+48]` |
| `ldr` at 0xc19dc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc19e8 | store | 4 | `sp -0x5c8` | `ret(0xfde7c)` |
| `ldr` at 0xc19f0 | load | 4 | `sp -0x5c8` | `-` |
| `ldr` at 0xc1a14 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1a18 | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0xc1a40 | load | 4 | `abs 0xc1f6c` | `-` |
| `str` at 0xc1a44 | store | 4 | `sp -0x5cc` | `0x31b5a0` |
| `ldrb` at 0xc1a50 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc1a58 | store | 4 | `sp -0x5d0` | `?` |
| `ldr` at 0xc1a5c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1a60 | load | 1 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0xc1a70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc1a74 | load | 4 | `[sp-28] +0x30` | `-` |
| `str` at 0xc1a78 | store | 4 | `sp -0x5d4` | `[[sp-28]+48]` |
| `ldr` at 0xc1a80 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc1a8c | store | 4 | `sp -0x5d4` | `ret(0xfde7c)` |
| `ldr` at 0xc1a90 | load | 4 | `abs 0xc1f78` | `-` |
| `ldrb` at 0xc1a98 | load | 1 | `sp -0x2b2` | `-` |
| `ldr` at 0xc1aa0 | load | 4 | `sp -0x5d4` | `-` |
| `ldr` at 0xc1aac | load | 4 | `sp -0x5d0` | `-` |
| `ldr` at 0xc1ab4 | load | 4 | `sp -0x5cc` | `-` |
| `str` at 0xc1ab8 | store | 4 | `[sp-1484] +0x8` | `?` |
| `ldr` at 0xc1abc | load | 4 | `abs 0xc1f48` | `-` |
| `ldr` at 0xc1ac0 | load | 4 | `abs 0x1a6468` | `-` |
| `ldr` at 0xc1acc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1ad0 | load | 1 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0xc1ae0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc1ae4 | load | 4 | `[sp-28] +0x30` | `-` |
| `str` at 0xc1ae8 | store | 4 | `sp -0x5d8` | `[[sp-28]+48]` |
| `ldr` at 0xc1af0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc1afc | store | 4 | `sp -0x5d8` | `ret(0xfde7c)` |
| `ldr` at 0xc1b00 | load | 4 | `abs 0xc1f6c` | `-` |
| `ldr` at 0xc1b04 | load | 4 | `abs 0xc1f6c` | `-` |
| `ldrb` at 0xc1b10 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc1b14 | load | 4 | `abs 0xc1f78` | `-` |
| `ldrb` at 0xc1b1c | load | 1 | `sp -0x2b2` | `-` |
| `str` at 0xc1b20 | store | 4 | `sp -0x6b0` | `[sp-690]:1` |
| `ldr` at 0xc1b24 | load | 4 | `sp -0x5d8` | `-` |
| `str` at 0xc1b28 | store | 4 | `sp -0x6ac` | `[sp-1496]` |
| `ldr` at 0xc1b2c | load | 4 | `abs 0xc1f70` | `-` |
| `ldr` at 0xc1b30 | load | 4 | `abs 0x31b5a8` | `-` |
| `ldr` at 0xc1b34 | load | 4 | `abs 0x31b5a8` | `-` |
| `ldr` at 0xc1b40 | load | 4 | `abs 0xc1f6c` | `-` |
| `ldr` at 0xc1b48 | load | 4 | `abs 0x31b5a8` | `-` |
| `ldr` at 0xc1b6c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1b70 | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0xc1b98 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1b9c | load | 1 | `[sp-28] +0x34` | `-` |
| `ldr` at 0xc1bac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc1bb0 | load | 4 | `[sp-28] +0x38` | `-` |
| `str` at 0xc1bb8 | store | 4 | `sp -0x5ec` | `?` |
| `ldr` at 0xc1bc0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc1bcc | store | 4 | `sp -0x5ec` | `ret(0xfde7c)` |
| `ldr` at 0xc1bd0 | load | 4 | `sp -0x5ec` | `-` |
| `str` at 0xc1bd8 | store | 4 | `sp -0x5ec` | `?` |
| `ldr` at 0xc1bdc | load | 4 | `sp -0x5ec` | `-` |
| `ldr` at 0xc1be8 | load | 4 | `sp -0x5ec` | `-` |
| `ldr` at 0xc1bf4 | load | 4 | `sp -0x5ec` | `-` |
| `ldr` at 0xc1c04 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1c08 | load | 1 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0xc1c18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc1c1c | load | 4 | `[sp-28] +0x30` | `-` |
| `str` at 0xc1c20 | store | 4 | `sp -0x5dc` | `[[sp-28]+48]` |
| `ldr` at 0xc1c28 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc1c34 | store | 4 | `sp -0x5dc` | `ret(0xfde7c)` |
| `ldr` at 0xc1c38 | load | 4 | `sp -0x5dc` | `-` |
| `ldr` at 0xc1c48 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1c4c | load | 1 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0xc1c5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc1c60 | load | 4 | `[sp-28] +0x30` | `-` |
| `str` at 0xc1c64 | store | 4 | `sp -0x5e0` | `[[sp-28]+48]` |
| `ldr` at 0xc1c6c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc1c78 | store | 4 | `sp -0x5e0` | `ret(0xfde7c)` |
| `ldr` at 0xc1c7c | load | 4 | `sp -0x5e0` | `-` |
| `ldr` at 0xc1c88 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1c8c | load | 1 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0xc1c9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc1ca0 | load | 4 | `[sp-28] +0x30` | `-` |
| `str` at 0xc1ca4 | store | 4 | `sp -0x5e4` | `[[sp-28]+48]` |
| `ldr` at 0xc1cac | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc1cb8 | store | 4 | `sp -0x5e4` | `ret(0xfde7c)` |
| `ldr` at 0xc1cbc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1cc0 | load | 1 | `[sp-28] +0x34` | `-` |
| `ldr` at 0xc1cd0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc1cd4 | load | 4 | `[sp-28] +0x38` | `-` |
| `str` at 0xc1cdc | store | 4 | `sp -0x5e8` | `?` |
| `ldr` at 0xc1ce4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc1cf0 | store | 4 | `sp -0x5e8` | `ret(0xfde7c)` |
| `ldr` at 0xc1cf4 | load | 4 | `sp -0x5e8` | `-` |
| `str` at 0xc1cfc | store | 4 | `sp -0x5e8` | `?` |
| `ldr` at 0xc1d00 | load | 4 | `sp -0x5e4` | `-` |
| `ldr` at 0xc1d04 | load | 4 | `sp -0x5e8` | `-` |
| `ldr` at 0xc1d50 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1d54 | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0xc1d7c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1d80 | load | 1 | `[sp-28] +0x34` | `-` |
| `ldr` at 0xc1d90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc1d94 | load | 4 | `[sp-28] +0x38` | `-` |
| `str` at 0xc1d9c | store | 4 | `sp -0x5f4` | `?` |
| `ldr` at 0xc1da0 | load | 4 | `sp -0x5f4` | `-` |
| `str` at 0xc1da8 | store | 4 | `sp -0x5f4` | `?` |
| `ldr` at 0xc1db0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc1dbc | store | 4 | `sp -0x5f4` | `ret(0xfde7c)` |
| `ldr` at 0xc1dc0 | load | 4 | `sp -0x5f4` | `-` |
| `str` at 0xc1dc8 | store | 4 | `sp -0x5f4` | `?` |
| `ldr` at 0xc1dcc | load | 4 | `sp -0x5f4` | `-` |
| `str` at 0xc1dd4 | store | 4 | `sp -0x5f4` | `?` |
| `ldr` at 0xc1dd8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1ddc | load | 1 | `[sp-28] +0x3c` | `-` |
| `ldr` at 0xc1dec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc1df0 | load | 4 | `[sp-28] +0x40` | `-` |
| `ldr` at 0xc1e00 | load | 4 | `sp -0x5f4` | `-` |
| `str` at 0xc1e08 | store | 4 | `sp -0x5f0` | `?` |
| `ldr` at 0xc1e10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc1e2c | load | 4 | `sp -0x5f4` | `-` |
| `str` at 0xc1e34 | store | 4 | `sp -0x5f0` | `?` |
| `ldr` at 0xc1e38 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1e3c | load | 1 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0xc1e4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc1e50 | load | 4 | `[sp-28] +0x30` | `-` |
| `ldr` at 0xc1e58 | load | 4 | `sp -0x5f0` | `-` |
| `str` at 0xc1e60 | store | 4 | `sp -0x5f8` | `?` |
| `ldr` at 0xc1e68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc1e7c | load | 4 | `sp -0x5f0` | `-` |
| `str` at 0xc1e84 | store | 4 | `sp -0x5f8` | `?` |
| `ldr` at 0xc1e88 | load | 4 | `sp -0x5f8` | `-` |
| `str` at 0xc1e8c | store | 4 | `sp -0x2b8` | `[sp-1528]` |
| `ldrb` at 0xc1e98 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc1e9c | load | 4 | `abs 0xc1f78` | `-` |
| `ldrb` at 0xc1ea4 | load | 1 | `sp -0x2b2` | `-` |
| `ldr` at 0xc1eb4 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0xc1ebc | load | 4 | `abs 0xc1f48` | `-` |
| `ldr` at 0xc1ec0 | load | 4 | `abs 0x1a6468` | `-` |
| `ldr` at 0xc1ecc | load | 4 | `abs 0xc1f78` | `-` |
| `ldrb` at 0xc1ed4 | load | 1 | `sp -0x2b2` | `-` |
| `ldr` at 0xc1ed8 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0xc1ee0 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0xc1ef0 | load | 4 | `sp -0x2b8` | `-` |
| `str` at 0xc1ef8 | store | 4 | `sp -0x6b0` | `?` |
| `ldr` at 0xc1efc | load | 4 | `abs 0xc1f74` | `-` |
| `ldr` at 0xc1f08 | load | 4 | `abs 0xc1f6c` | `-` |
| `ldr` at 0xc1f0c | load | 4 | `abs 0x31b5a8` | `-` |
| `ldr` at 0xc1f18 | load | 4 | `abs 0xc1f6c` | `-` |
| `ldr` at 0xc1f1c | load | 4 | `abs 0x31b5a8` | `-` |
| `ldr` at 0xc1f28 | load | 4 | `abs 0xc1f78` | `-` |
| `ldrb` at 0xc1f30 | load | 1 | `sp -0x2b2` | `-` |
| `ldr` at 0xc1fe8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1fec | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0xc2014 | load | 4 | `abs 0xc1f78` | `-` |
| `ldrb` at 0xc201c | load | 1 | `sp -0x2b2` | `-` |
| `str` at 0xc2020 | store | 4 | `sp -0x5fc` | `[sp-690]:1` |
| `ldr` at 0xc2024 | load | 4 | `sp -0x5fc` | `-` |
| `str` at 0xc202c | store | 4 | `sp -0x5fc` | `?` |
| `ldr` at 0xc2030 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc2034 | load | 1 | `[sp-28] +0x34` | `-` |
| `ldr` at 0xc2044 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc2048 | load | 4 | `[sp-28] +0x38` | `-` |
| `ldr` at 0xc204c | load | 4 | `sp -0x5fc` | `-` |
| `str` at 0xc2058 | store | 4 | `sp -0x600` | `?` |
| `ldr` at 0xc2060 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc2070 | load | 4 | `sp -0x5fc` | `-` |
| `str` at 0xc207c | store | 4 | `sp -0x600` | `?` |
| `ldr` at 0xc2080 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc2084 | load | 1 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0xc2094 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc2098 | load | 4 | `[sp-28] +0x30` | `-` |
| `str` at 0xc20a0 | store | 4 | `sp -0x604` | `?` |
| `ldr` at 0xc20a8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc20bc | store | 4 | `sp -0x604` | `?` |
| `ldrb` at 0xc20c8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc20d0 | load | 4 | `sp -0x600` | `-` |
| `ldr` at 0xc20d4 | load | 4 | `sp -0x604` | `-` |
| `ldr` at 0xc20e0 | load | 4 | `abs 0xc1f7c` | `-` |
| `ldr` at 0xc20e4 | load | 4 | `abs 0x1a025c` | `-` |
| `str` at 0xc20f4 | store | 4 | `sp -0x2c0` | `ret(unknown)` |
| `ldr` at 0xc20f8 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0xc212c | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0xc2138 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc213c | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0xc2148 | load | 4 | `abs 0xc1f80` | `-` |
| `ldr` at 0xc214c | load | 4 | `abs 0x1b0cec` | `-` |
| `ldr` at 0xc2174 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc2178 | load | 1 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0xc2188 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc218c | load | 4 | `[sp-28] +0x30` | `-` |
| `str` at 0xc2194 | store | 4 | `sp -0x608` | `?` |
| `ldr` at 0xc219c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc21b0 | store | 4 | `sp -0x608` | `?` |
| `ldr` at 0xc21b4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc21b8 | load | 1 | `[sp-28] +0x34` | `-` |
| `ldr` at 0xc21c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc21cc | load | 4 | `[sp-28] +0x38` | `-` |
| `str` at 0xc21d4 | store | 4 | `sp -0x60c` | `?` |
| `ldr` at 0xc21dc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc21f0 | store | 4 | `sp -0x60c` | `?` |
| `ldr` at 0xc21f4 | load | 4 | `abs 0xc1f80` | `-` |
| `ldrb` at 0xc2200 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc2204 | load | 4 | `abs 0x1b0cec` | `-` |
| `ldr` at 0xc220c | load | 4 | `sp -0x608` | `-` |
| `ldr` at 0xc2210 | load | 4 | `sp -0x60c` | `-` |
| `str` at 0xc221c | store | 4 | `sp -0x2c0` | `ret(unknown)` |
| `ldr` at 0xc2220 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0xc2254 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0xc2260 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc2264 | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0xc2270 | load | 4 | `abs 0xc1f84` | `-` |
| `ldr` at 0xc2274 | load | 4 | `abs 0x1b0cf0` | `-` |
| `ldr` at 0xc229c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc22a0 | load | 1 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0xc22b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc22b4 | load | 4 | `[sp-28] +0x30` | `-` |
| `str` at 0xc22bc | store | 4 | `sp -0x610` | `?` |
| `ldr` at 0xc22c4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc22d8 | store | 4 | `sp -0x610` | `?` |
| `ldr` at 0xc22dc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc22e0 | load | 1 | `[sp-28] +0x34` | `-` |
| `ldr` at 0xc22f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc22f4 | load | 4 | `[sp-28] +0x38` | `-` |
| `str` at 0xc22fc | store | 4 | `sp -0x614` | `?` |
| `ldr` at 0xc2304 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc2318 | store | 4 | `sp -0x614` | `?` |
| `ldr` at 0xc231c | load | 4 | `abs 0xc1f84` | `-` |
| `ldrb` at 0xc2328 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc232c | load | 4 | `abs 0x1b0cf0` | `-` |
| `ldr` at 0xc2334 | load | 4 | `sp -0x610` | `-` |
| `ldr` at 0xc2338 | load | 4 | `sp -0x614` | `-` |
| `str` at 0xc2344 | store | 4 | `sp -0x2c0` | `ret(unknown)` |
| `ldr` at 0xc2348 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0xc237c | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0xc2388 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc238c | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0xc2398 | load | 4 | `abs 0xc1f88` | `-` |
| `ldr` at 0xc239c | load | 4 | `abs 0x1b0cf4` | `-` |
| `ldr` at 0xc23c4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc23c8 | load | 1 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0xc23d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc23dc | load | 4 | `[sp-28] +0x30` | `-` |
| `str` at 0xc23e4 | store | 4 | `sp -0x618` | `?` |
| `ldr` at 0xc23ec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc2400 | store | 4 | `sp -0x618` | `?` |
| `ldr` at 0xc2404 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc2408 | load | 1 | `[sp-28] +0x34` | `-` |
| `ldr` at 0xc2418 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc241c | load | 4 | `[sp-28] +0x38` | `-` |
| `str` at 0xc2424 | store | 4 | `sp -0x61c` | `?` |
| `ldr` at 0xc242c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc2440 | store | 4 | `sp -0x61c` | `?` |
| `ldr` at 0xc2444 | load | 4 | `abs 0xc1f88` | `-` |
| `ldrb` at 0xc2450 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc2454 | load | 4 | `abs 0x1b0cf4` | `-` |
| `ldr` at 0xc245c | load | 4 | `sp -0x618` | `-` |
| `ldr` at 0xc2460 | load | 4 | `sp -0x61c` | `-` |
| `str` at 0xc246c | store | 4 | `sp -0x2c0` | `ret(unknown)` |
| `ldr` at 0xc2470 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0xc24a4 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0xc24b0 | load | 4 | `abs 0xc1f8c` | `-` |
| `ldr` at 0xc24b4 | load | 4 | `abs 0x31a5fc` | `-` |
| `str` at 0xc24c4 | store | 4 | `sp -0x2c0` | `ret(unknown)` |
| `ldr` at 0xc24c8 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0xc24fc | load | 4 | `sp -0x2c0` | `-` |
| `str` at 0xc2510 | store | 4 | `sp -0x2c0` | `ret(0x55654)` |
| `ldr` at 0xc2514 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0xc2548 | load | 4 | `sp -0x2c0` | `-` |
| `str` at 0xc2570 | store | 4 | `sp -0x2f0` | `0x0` |
| `str` at 0xc2578 | store | 4 | `sp -0x2ec` | `0x0` |
| `ldr` at 0xc2584 | load | 4 | `sp -0x2f0` | `-` |
| `ldr` at 0xc2594 | load | 4 | `sp -0x2f0` | `-` |
| `str` at 0xc25a4 | store | 4 | `sp -0x2e8` | `ret(0x120340)` |
| `str` at 0xc25ac | store | 4 | `sp -0x2e4` | `0x0` |
| `ldr` at 0xc25b0 | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0xc25b4 | load | 4 | `sp -0x2e8` | `-` |
| `str` at 0xc25c4 | store | 4 | `sp -0x2dc` | `0x1` |
| `ldr` at 0xc25c8 | load | 4 | `sp -0x2f0` | `-` |
| `str` at 0xc25dc | store | 4 | `sp -0x3ec` | `ret(0x6e1b0)` |
| `ldr` at 0xc25e0 | load | 4 | `sp -0x3ec` | `-` |
| `str` at 0xc25f0 | store | 4 | `sp -0x3f0` | `ret(0x2031c)+1` |
| `ldr` at 0xc25f4 | load | 4 | `sp -0x3f0` | `-` |
| `str` at 0xc261c | store | 4 | `sp -0x3f4` | `?` |
| `ldr` at 0xc2620 | load | 4 | `sp -0x3f4` | `-` |
| `ldr` at 0xc2624 | load | 4 | `sp -0x3ec` | `-` |
| `ldr` at 0xc2628 | load | 4 | `sp -0x3f0` | `-` |
| `ldr` at 0xc2634 | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0xc263c | load | 4 | `sp -0x2f0` | `-` |
| `str` at 0xc2650 | store | 4 | `(unknown)` | `ret(0x1208b8)` |
| `ldr` at 0xc2658 | load | 4 | `abs 0xc1f90` | `-` |
| `ldr` at 0xc265c | load | 4 | `sp -0x2ec` | `-` |
| `str` at 0xc2680 | store | 4 | `sp -0x2fc` | `(sp-1000 + ret(0x2031c))` |
| `ldr` at 0xc2684 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc2688 | load | 4 | `[sp-756] +0x0` | `-` |
| `ldr` at 0xc2694 | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0xc269c | load | 4 | `sp -0x2f0` | `-` |
| `ldr` at 0xc26bc | load | 4 | `abs 0xc1f94` | `-` |
| `ldr` at 0xc26c8 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc26cc | load | 4 | `[sp-756] +0x1c` | `-` |
| `ldr` at 0xc26d8 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc26dc | load | 4 | `[sp-756] +0x1c` | `-` |
| `ldr` at 0xc26e8 | load | 4 | `sp -0x2fc` | `-` |
| `ldr` at 0xc26ec | load | 4 | `abs 0xc1f98` | `-` |
| `ldr` at 0xc26fc | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0xc2704 | load | 4 | `sp -0x2f0` | `-` |
| `str` at 0xc2720 | store | 4 | `sp -0x2d8` | `ret(0xc4b5c)` |
| `ldr` at 0xc2724 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0xc2730 | load | 4 | `abs 0xc1f9c` | `-` |
| `str` at 0xc2734 | store | 4 | `sp -0x620` | `0x16dab8` |
| `ldr` at 0xc273c | load | 4 | `abs 0xc1fa0` | `-` |
| `str` at 0xc2740 | store | 4 | `sp -0x620` | `0x16dac0` |
| `ldr` at 0xc2744 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc2748 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc274c | load | 4 | `[sp-756] +0x50` | `-` |
| `str` at 0xc2750 | store | 4 | `(unknown)` | `[[sp-756]+80]` |
| `ldr` at 0xc2754 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc2758 | load | 4 | `[sp-756] +0x54` | `-` |
| `str` at 0xc275c | store | 4 | `(unknown)` | `[[sp-756]+84]` |
| `ldr` at 0xc2760 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc2764 | load | 4 | `[sp-756] +0x58` | `-` |
| `ldrb` at 0xc2768 | load | 1 | `[[sp-756]+88] +0x0` | `-` |
| `ldr` at 0xc2778 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc277c | load | 4 | `[sp-756] +0x58` | `-` |
| `str` at 0xc2784 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xc2788 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc278c | load | 4 | `[sp-756] +0x5c` | `-` |
| `str` at 0xc2790 | store | 4 | `(unknown)` | `[[sp-756]+92]` |
| `ldr` at 0xc2794 | load | 4 | `sp -0x2fc` | `-` |
| `ldr` at 0xc2798 | load | 4 | `abs 0xc1fa4` | `-` |
| `ldr` at 0xc279c | load | 4 | `sp -0x620` | `-` |
| `ldr` at 0xc27a0 | load | 4 | `[sp-756] +0x4c` | `-` |
| `ldr` at 0xc27ac | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc27b0 | load | 4 | `[sp-756] +0x0` | `-` |
| `ldr` at 0xc27bc | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0xc27c4 | load | 4 | `sp -0x2f0` | `-` |
| `ldr` at 0xc27e4 | load | 4 | `abs 0xc1fa8` | `-` |
| `ldr` at 0xc27f0 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc27f4 | load | 4 | `[sp-756] +0x4` | `-` |
| `ldr` at 0xc2800 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc2804 | load | 4 | `[sp-756] +0x4` | `-` |
| `ldr` at 0xc2810 | load | 4 | `sp -0x2fc` | `-` |
| `ldr` at 0xc2814 | load | 4 | `abs 0xc1fac` | `-` |
| `ldr` at 0xc2824 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc2828 | load | 4 | `[sp-756] +0x20` | `-` |
| `ldr` at 0xc2834 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc2838 | load | 4 | `[sp-756] +0x20` | `-` |
| `ldr` at 0xc2844 | load | 4 | `sp -0x2fc` | `-` |
| `ldr` at 0xc2848 | load | 4 | `abs 0xc1fb0` | `-` |
| `ldr` at 0xc2858 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc285c | load | 4 | `[sp-756] +0x34` | `-` |
| `ldr` at 0xc2868 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc286c | load | 4 | `[sp-756] +0x34` | `-` |
| `ldr` at 0xc2878 | load | 4 | `sp -0x2fc` | `-` |
| `ldr` at 0xc287c | load | 4 | `abs 0xc1fb4` | `-` |
| `ldr` at 0xc288c | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc2890 | load | 4 | `[sp-756] +0x48` | `-` |
| `ldr` at 0xc289c | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc28a0 | load | 4 | `[sp-756] +0x48` | `-` |
| `ldr` at 0xc28ac | load | 4 | `sp -0x2fc` | `-` |
| `ldr` at 0xc28b0 | load | 4 | `abs 0xc1fb8` | `-` |
| `ldr` at 0xc28c0 | load | 4 | `sp -0x2fc` | `-` |
| `ldr` at 0xc28c4 | load | 4 | `abs 0xc1fbc` | `-` |
| `ldr` at 0xc28cc | load | 4 | `sp -0x2fc` | `-` |
| `ldr` at 0xc28dc | load | 4 | `sp -0x2fc` | `-` |
| `str` at 0xc28e4 | store | 4 | `sp -0x2fc` | `([sp-764] + ret(0x2031c))` |
| `str` at 0xc28ec | store | 4 | `sp -0x2e0` | `0x0` |
| `ldr` at 0xc28f0 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0xc2900 | load | 4 | `sp -0x2fc` | `-` |
| `str` at 0xc2904 | store | 4 | `sp -0x628` | `?` |
| `str` at 0xc290c | store | 4 | `sp -0x2fc` | `?` |
| `ldr` at 0xc2910 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc2914 | load | 4 | `[sp-756] +0x4c` | `-` |
| `ldr` at 0xc2918 | load | 4 | `sp -0x2e0` | `-` |
| `ldrb` at 0xc2920 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc2938 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc293c | load | 4 | `[sp-756] +0x4c` | `-` |
| `ldr` at 0xc2940 | load | 4 | `sp -0x2e0` | `-` |
| `ldrb` at 0xc2948 | load | 1 | `([[sp-756]+76] + [sp-736]) +0x0` | `-` |
| `strb` at 0xc2950 | store | 1 | `sp -0x621` | `([([[sp-756]+76] + [sp-736])]:1 & 0xff)` |
| `strb` at 0xc2960 | store | 1 | `sp -0x621` | `0x3f` |
| `ldrb` at 0xc2968 | load | 1 | `sp -0x621` | `-` |
| `ldr` at 0xc296c | load | 4 | `sp -0x628` | `-` |
| `strb` at 0xc2970 | store | 1 | `[sp-1576] +0x0` | `?` |
| `ldr` at 0xc2974 | load | 4 | `sp -0x2e0` | `-` |
| `str` at 0xc297c | store | 4 | `sp -0x2e0` | `[sp-736]+1` |
| `ldr` at 0xc2988 | load | 4 | `sp -0x2fc` | `-` |
| `strb` at 0xc2994 | store | 1 | `(unknown)` | `0x2c` |
| `str` at 0xc299c | store | 4 | `sp -0x2fc` | `?` |
| `str` at 0xc29a4 | store | 4 | `sp -0x2e0` | `0x7` |
| `ldr` at 0xc29a8 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0xc29b4 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc29b8 | load | 4 | `[sp-756] +0x4c` | `-` |
| `ldr` at 0xc29c0 | load | 4 | `sp -0x2e0` | `-` |
| `ldrb` at 0xc29cc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc29d0 | load | 4 | `sp -0x2fc` | `-` |
| `ldr` at 0xc29d4 | load | 4 | `abs 0xc1fc0` | `-` |
| `ldr` at 0xc29e0 | load | 4 | `sp -0x2fc` | `-` |
| `str` at 0xc29e8 | store | 4 | `sp -0x2fc` | `[sp-764]+2` |
| `ldr` at 0xc29ec | load | 4 | `sp -0x2e0` | `-` |
| `str` at 0xc29f4 | store | 4 | `sp -0x2e0` | `[sp-736]-1` |
| `ldr` at 0xc2a00 | load | 4 | `sp -0x2fc` | `-` |
| `strb` at 0xc2a0c | store | 1 | `(unknown)` | `0x2c` |
| `str` at 0xc2a14 | store | 4 | `sp -0x2fc` | `?` |
| `ldr` at 0xc2a18 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc2a1c | load | 4 | `[sp-756] +0x24` | `-` |
| `ldrb` at 0xc2a20 | load | 1 | `[[sp-756]+36] +0x3` | `-` |
| `ldr` at 0xc2a24 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc2a28 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0xc2a2c | load | 4 | `[sp-756] +0x3c` | `-` |
| `str` at 0xc2a30 | store | 4 | `(unknown)` | `[[sp-756]+60]` |
| `ldr` at 0xc2a34 | load | 4 | `sp -0x2fc` | `-` |
| `ldr` at 0xc2a38 | load | 4 | `abs 0xc1fc4` | `-` |
| `ldr` at 0xc2a3c | load | 4 | `[sp-756] +0x28` | `-` |
| `ldr` at 0xc2a50 | load | 4 | `abs 0xc1fc8` | `-` |
| `str` at 0xc2a5c | store | 4 | `sp -0x2dc` | `0x0` |
| `ldr` at 0xc2a60 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0xc2a90 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0xc2aa4 | load | 4 | `sp -0x2ec` | `-` |
| `str` at 0xc2aac | store | 4 | `sp -0x2ec` | `[sp-748]+1` |
| `ldr` at 0xc2ab0 | load | 4 | `sp -0x2e4` | `-` |
| `str` at 0xc2ab8 | store | 4 | `sp -0x2e4` | `[sp-740]+1` |
| `ldr` at 0xc2abc | load | 4 | `sp -0x2f4` | `-` |
| `str` at 0xc2ac4 | store | 4 | `sp -0x2f4` | `[sp-756]+120` |
| `ldr` at 0xc2acc | load | 4 | `sp -0x2f0` | `-` |
| `str` at 0xc2ad4 | store | 4 | `sp -0x2f0` | `[sp-752]+1` |
| `str` at 0xc2af8 | store | 4 | `sp -0x3f0` | `0x0` |
| `str` at 0xc2b00 | store | 4 | `sp -0x3ec` | `0x0` |
| `ldr` at 0xc2b0c | load | 4 | `sp -0x3f0` | `-` |
| `ldr` at 0xc2b1c | load | 4 | `sp -0x3f0` | `-` |
| `str` at 0xc2b2c | store | 4 | `sp -0x2f0` | `ret(0x120340)` |
| `str` at 0xc2b34 | store | 4 | `sp -0x2ec` | `0x0` |
| `ldr` at 0xc2b38 | load | 4 | `sp -0x2ec` | `-` |
| `ldr` at 0xc2b3c | load | 4 | `sp -0x2f0` | `-` |
| `ldr` at 0xc2b48 | load | 4 | `sp -0x2ec` | `-` |
| `ldr` at 0xc2b50 | load | 4 | `sp -0x3f0` | `-` |
| `ldr` at 0xc2b70 | load | 4 | `abs 0xc1fcc` | `-` |
| `ldr` at 0xc2b74 | load | 4 | `sp -0x2ec` | `-` |
| `str` at 0xc2ba8 | store | 4 | `sp -0x2dc` | `(sp-1140 + ret(0x2031c))` |
| `str` at 0xc2bb0 | store | 4 | `sp -0x2e4` | `0x1` |
| `ldr` at 0xc2bb4 | load | 4 | `sp -0x3f4` | `-` |
| `ldr` at 0xc2bb8 | load | 4 | `[sp-1012] +0x0` | `-` |
| `ldr` at 0xc2bc4 | load | 4 | `sp -0x2ec` | `-` |
| `ldr` at 0xc2bcc | load | 4 | `sp -0x3f0` | `-` |
| `ldr` at 0xc2bf4 | load | 4 | `abs 0xc1fd0` | `-` |
| `ldr` at 0xc2c00 | load | 4 | `sp -0x3f4` | `-` |
| `ldr` at 0xc2c04 | load | 4 | `[sp-1012] +0x4` | `-` |
| `ldr` at 0xc2c10 | load | 4 | `sp -0x3f4` | `-` |
| `ldr` at 0xc2c14 | load | 4 | `[sp-1012] +0x4` | `-` |
| `ldr` at 0xc2c20 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0xc2c24 | load | 4 | `abs 0xc1fd4` | `-` |
| `ldr` at 0xc2c34 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0xc2c38 | load | 4 | `abs 0xc1fd8` | `-` |
| `ldr` at 0xc2c40 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0xc2c50 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0xc2c58 | store | 4 | `sp -0x2dc` | `([sp-732] + ret(0x2031c))` |
| `str` at 0xc2c60 | store | 4 | `sp -0x2e8` | `0x40` |
| `ldr` at 0xc2c64 | load | 4 | `sp -0x2e8` | `-` |
| `ldr` at 0xc2c70 | load | 4 | `sp -0x3f4` | `-` |
| `ldr` at 0xc2c74 | load | 4 | `[sp-1012] +0xc` | `-` |
| `ldr` at 0xc2c78 | load | 4 | `sp -0x2e8` | `-` |
| `ldrb` at 0xc2c80 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc2c84 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0xc2c88 | load | 4 | `abs 0xc1fdc` | `-` |
| `ldr` at 0xc2c94 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0xc2c9c | store | 4 | `sp -0x2dc` | `[sp-732]+2` |
| `ldr` at 0xc2ca0 | load | 4 | `sp -0x2e8` | `-` |
| `str` at 0xc2ca8 | store | 4 | `sp -0x2e8` | `[sp-744]+1` |
| `str` at 0xc2cb4 | store | 4 | `sp -0x2e8` | `0x5f` |
| `ldr` at 0xc2cb8 | load | 4 | `sp -0x2e8` | `-` |
| `ldr` at 0xc2cc4 | load | 4 | `sp -0x3f4` | `-` |
| `ldr` at 0xc2cc8 | load | 4 | `[sp-1012] +0xc` | `-` |
| `ldr` at 0xc2ccc | load | 4 | `sp -0x2e8` | `-` |
| `ldrb` at 0xc2cd4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc2cd8 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0xc2cdc | load | 4 | `abs 0xc1fdc` | `-` |
| `ldr` at 0xc2ce8 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0xc2cf0 | store | 4 | `sp -0x2dc` | `[sp-732]+2` |
| `ldr` at 0xc2cf4 | load | 4 | `sp -0x2e8` | `-` |
| `str` at 0xc2cfc | store | 4 | `sp -0x2e8` | `[sp-744]+1` |
| `ldr` at 0xc2d08 | load | 4 | `sp -0x2dc` | `-` |
| `strb` at 0xc2d14 | store | 1 | `(unknown)` | `0x2c` |
| `str` at 0xc2d1c | store | 4 | `sp -0x2dc` | `?` |
| `str` at 0xc2d24 | store | 4 | `sp -0x2e8` | `0x49` |
| `ldr` at 0xc2d28 | load | 4 | `sp -0x2e8` | `-` |
| `ldr` at 0xc2d34 | load | 4 | `sp -0x3f4` | `-` |
| `ldr` at 0xc2d38 | load | 4 | `[sp-1012] +0xc` | `-` |
| `ldr` at 0xc2d3c | load | 4 | `sp -0x2e8` | `-` |
| `ldrb` at 0xc2d44 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc2d60 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0xc2d64 | load | 4 | `sp -0x3f4` | `-` |
| `ldr` at 0xc2d68 | load | 4 | `[sp-1012] +0xc` | `-` |
| `ldr` at 0xc2d6c | load | 4 | `sp -0x2e8` | `-` |
| `ldrb` at 0xc2d78 | load | 1 | `([[sp-1012]+12] + [sp-744]) +0x0` | `-` |
| `strb` at 0xc2d7c | store | 1 | `[sp-732] +0x0` | `([([[sp-1012]+12] + [sp-744])]:1 & 0xff)` |
| `str` at 0xc2d84 | store | 4 | `sp -0x2dc` | `[sp-732]+1` |
| `ldr` at 0xc2d90 | load | 4 | `sp -0x2dc` | `-` |
| `strb` at 0xc2d9c | store | 1 | `[sp-732] +0x0` | `0x20` |
| `str` at 0xc2da4 | store | 4 | `sp -0x2dc` | `[sp-732]+1` |
| `ldr` at 0xc2da8 | load | 4 | `sp -0x2e8` | `-` |
| `str` at 0xc2db0 | store | 4 | `sp -0x2e8` | `[sp-744]+1` |
| `ldr` at 0xc2dbc | load | 4 | `sp -0x2dc` | `-` |
| `strb` at 0xc2dc8 | store | 1 | `(unknown)` | `0x2c` |
| `str` at 0xc2dd0 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0xc2dd4 | load | 4 | `sp -0x3f4` | `-` |
| `ldr` at 0xc2ddc | load | 4 | `[sp-1012] +0xc` | `-` |
| `ldrb` at 0xc2de4 | load | 1 | `[[sp-1012]+12] +0x40` | `-` |
| `ldr` at 0xc2df0 | load | 4 | `sp -0x3f4` | `-` |
| `ldr` at 0xc2df8 | load | 4 | `[sp-1012] +0xc` | `-` |
| `ldrb` at 0xc2e00 | load | 1 | `[[sp-1012]+12] +0x5b` | `-` |
| `ldr` at 0xc2e18 | load | 4 | `sp -0x3f4` | `-` |
| `ldr` at 0xc2e20 | load | 4 | `[sp-1012] +0xc` | `-` |
| `ldrb` at 0xc2e28 | load | 1 | `[[sp-1012]+12] +0x5c` | `-` |
| `ldr` at 0xc2e40 | load | 4 | `sp -0x3f4` | `-` |
| `ldr` at 0xc2e48 | load | 4 | `[sp-1012] +0xc` | `-` |
| `ldrb` at 0xc2e50 | load | 1 | `[[sp-1012]+12] +0x5b` | `-` |
| `ldr` at 0xc2e54 | load | 4 | `sp -0x3f4` | `-` |
| `ldr` at 0xc2e5c | load | 4 | `[sp-1012] +0xc` | `-` |
| `ldrb` at 0xc2e64 | load | 1 | `[[sp-1012]+12] +0x5c` | `-` |
| `ldr` at 0xc2e68 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0xc2e6c | load | 4 | `abs 0xc1fe0` | `-` |
| `ldr` at 0xc2e78 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0xc2e80 | store | 4 | `sp -0x2dc` | `[sp-732]+4` |
| `ldr` at 0xc2e8c | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0xc2e90 | load | 4 | `sp -0x3f4` | `-` |
| `ldr` at 0xc2e98 | load | 4 | `[sp-1012] +0xc` | `-` |
| `ldrb` at 0xc2ea4 | load | 1 | `[[sp-1012]+12] +0x5b` | `-` |
| `strb` at 0xc2ea8 | store | 1 | `[sp-732] +0x0` | `([[[sp-1012]+12]+91]:1 & 0xff)` |
| `str` at 0xc2eb0 | store | 4 | `sp -0x2dc` | `[sp-732]+1` |
| `ldr` at 0xc2eb8 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0xc2ebc | load | 4 | `sp -0x3f4` | `-` |
| `ldr` at 0xc2ec4 | load | 4 | `[sp-1012] +0xc` | `-` |
| `ldrb` at 0xc2ed0 | load | 1 | `[[sp-1012]+12] +0x5c` | `-` |
| `strb` at 0xc2ed4 | store | 1 | `[sp-732] +0x1` | `([[[sp-1012]+12]+92]:1 & 0xff)` |
| `str` at 0xc2edc | store | 4 | `sp -0x2dc` | `[sp-732]+2` |
| `ldr` at 0xc2ee4 | load | 4 | `sp -0x2dc` | `-` |
| `strb` at 0xc2ef0 | store | 1 | `(unknown)` | `0x2c` |
| `str` at 0xc2ef8 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0xc2efc | load | 4 | `sp -0x3f4` | `-` |
| `ldr` at 0xc2f04 | load | 4 | `[sp-1012] +0xc` | `-` |
| `ldrb` at 0xc2f0c | load | 1 | `[[sp-1012]+12] +0x9` | `-` |
| `ldr` at 0xc2f18 | load | 4 | `sp -0x3f4` | `-` |
| `ldr` at 0xc2f20 | load | 4 | `[sp-1012] +0xc` | `-` |
| `ldrb` at 0xc2f28 | load | 1 | `[[sp-1012]+12] +0x9` | `-` |
| `ldr` at 0xc2f30 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0xc2f34 | load | 4 | `abs 0xc1fe4` | `-` |
| `ldr` at 0xc2f40 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0xc2f50 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0xc2f58 | store | 4 | `sp -0x2dc` | `([sp-732] + ret(0x2031c))` |
| `ldr` at 0xc2f5c | load | 4 | `sp -0x3f4` | `-` |
| `ldr` at 0xc2f68 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0xc2f84 | load | 4 | `abs 0xc3ef0` | `-` |
| `str` at 0xc2f90 | store | 4 | `sp -0x2e4` | `0x0` |
| `ldr` at 0xc2f94 | load | 4 | `sp -0x2e4` | `-` |
| `str` at 0xc2fd4 | store | 4 | `sp -0x6b0` | `0x1` |
| `ldr` at 0xc2fe8 | load | 4 | `sp -0x2ec` | `-` |
| `str` at 0xc2ff0 | store | 4 | `sp -0x2ec` | `[sp-748]+1` |
| `ldr` at 0xc2ff4 | load | 4 | `sp -0x3f4` | `-` |
| `str` at 0xc2ffc | store | 4 | `sp -0x3f4` | `[sp-1012]+120` |
| `ldr` at 0xc3004 | load | 4 | `sp -0x3f0` | `-` |
| `str` at 0xc300c | store | 4 | `sp -0x3f0` | `[sp-1008]+1` |
| `str` at 0xc3024 | store | 4 | `sp -0x3ec` | `ret(0xa4014)` |
| `ldr` at 0xc3028 | load | 4 | `sp -0x3ec` | `-` |
| `str` at 0xc306c | store | 4 | `sp -0x3f0` | `0x0` |
| `ldr` at 0xc3070 | load | 4 | `sp -0x3f0` | `-` |
| `ldr` at 0xc3074 | load | 4 | `sp -0x3ec` | `-` |
| `ldr` at 0xc3088 | load | 4 | `sp -0x3f0` | `-` |
| `ldr` at 0xc3090 | load | 4 | `sp -0x2f0` | `-` |
| `ldr` at 0xc30a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc30a8 | load | 4 | `sp -0x3f0` | `-` |
| `str` at 0xc30b0 | store | 4 | `sp -0x3f0` | `[sp-1008]+1` |
| `ldr` at 0xc30b8 | load | 4 | `sp -0x2f0` | `-` |
| `ldr` at 0xc30c4 | load | 4 | `abs 0xc3ef4` | `-` |
| `ldr` at 0xc30cc | load | 4 | `abs 0xc3ef8` | `-` |
| `ldr` at 0xc30d4 | load | 4 | `abs 0xc3efc` | `-` |
| `ldr` at 0xc30dc | load | 4 | `abs 0xc3f00` | `-` |
| `ldr` at 0xc30e4 | load | 4 | `abs 0x31e428` | `-` |
| `ldr` at 0xc30f0 | load | 4 | `abs 0xc3f04` | `-` |
| `ldr` at 0xc30f4 | load | 4 | `abs 0xc3f08` | `-` |
| `ldrb` at 0xc30f8 | load | 1 | `abs 0x16dc30` | `-` |
| `strb` at 0xc3100 | store | 1 | `sp -0x53c` | `([0x16dc30]:1 & 0xff)` |
| `ldr` at 0xc3120 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc3124 | load | 1 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0xc3134 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc3138 | load | 4 | `[sp-28] +0x30` | `-` |
| `str` at 0xc313c | store | 4 | `sp -0x62c` | `[[sp-28]+48]` |
| `ldr` at 0xc3144 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc3150 | store | 4 | `sp -0x62c` | `ret(0xfde7c)` |
| `ldr` at 0xc3154 | load | 4 | `sp -0x62c` | `-` |
| `str` at 0xc3158 | store | 4 | `sp -0x3f0` | `[sp-1580]` |
| `ldr` at 0xc315c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc3160 | load | 1 | `[sp-28] +0x34` | `-` |
| `ldr` at 0xc3170 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc3174 | load | 4 | `[sp-28] +0x38` | `-` |
| `str` at 0xc3178 | store | 4 | `sp -0x630` | `[[sp-28]+56]` |
| `ldr` at 0xc3180 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc318c | store | 4 | `sp -0x630` | `ret(0xfde7c)` |
| `ldr` at 0xc3190 | load | 4 | `sp -0x630` | `-` |
| `str` at 0xc3194 | store | 4 | `sp -0x2ec` | `[sp-1584]` |
| `ldr` at 0xc3198 | load | 4 | `sp -0x3f0` | `-` |
| `str` at 0xc319c | store | 4 | `sp -0x644` | `[sp-1008]` |
| `ldr` at 0xc31a0 | load | 4 | `sp -0x644` | `-` |
| `ldr` at 0xc31ac | load | 4 | `sp -0x644` | `-` |
| `ldr` at 0xc31bc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc31c0 | load | 1 | `[sp-28] +0x3c` | `-` |
| `ldr` at 0xc31d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc31d4 | load | 4 | `[sp-28] +0x40` | `-` |
| `str` at 0xc31d8 | store | 4 | `sp -0x634` | `[[sp-28]+64]` |
| `ldr` at 0xc31e0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc31ec | store | 4 | `sp -0x634` | `ret(0xfde7c)` |
| `ldr` at 0xc31f0 | load | 4 | `sp -0x634` | `-` |
| `str` at 0xc31f4 | store | 4 | `sp -0x2e8` | `[sp-1588]` |
| `ldr` at 0xc31f8 | load | 4 | `sp -0x2ec` | `-` |
| `ldr` at 0xc3214 | load | 4 | `sp -0x2e8` | `-` |
| `str` at 0xc3228 | store | 4 | `sp -0x3ec` | `ret(0x89e18)` |
| `ldr` at 0xc322c | load | 4 | `sp -0x3ec` | `-` |
| `ldr` at 0xc3250 | load | 4 | `abs 0xc3f08` | `-` |
| `ldrb` at 0xc3258 | load | 1 | `sp -0x53c` | `-` |
| `str` at 0xc3298 | store | 4 | `sp -0x6b0` | `0x1` |
| `ldr` at 0xc32d8 | load | 4 | `sp -0x3ec` | `-` |
| `ldr` at 0xc32e0 | load | 4 | `abs 0xc3f08` | `-` |
| `ldrb` at 0xc32e8 | load | 1 | `sp -0x53c` | `-` |
| `str` at 0xc3328 | store | 4 | `sp -0x6b0` | `0x1` |
| `ldr` at 0xc3340 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc3344 | load | 1 | `[sp-28] +0x3c` | `-` |
| `ldr` at 0xc3354 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc3358 | load | 4 | `[sp-28] +0x40` | `-` |
| `str` at 0xc335c | store | 4 | `sp -0x638` | `[[sp-28]+64]` |
| `ldr` at 0xc3364 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc3370 | store | 4 | `sp -0x638` | `ret(0xfde7c)` |
| `ldr` at 0xc3374 | load | 4 | `sp -0x638` | `-` |
| `str` at 0xc3378 | store | 4 | `sp -0x2e4` | `[sp-1592]` |
| `ldr` at 0xc337c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc3380 | load | 1 | `[sp-28] +0x44` | `-` |
| `ldr` at 0xc3390 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc3394 | load | 4 | `[sp-28] +0x48` | `-` |
| `str` at 0xc3398 | store | 4 | `sp -0x63c` | `[[sp-28]+72]` |
| `ldr` at 0xc33a0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc33ac | store | 4 | `sp -0x63c` | `ret(0xfde7c)` |
| `ldr` at 0xc33b0 | load | 4 | `sp -0x63c` | `-` |
| `str` at 0xc33b4 | store | 4 | `sp -0x2e0` | `[sp-1596]` |
| `ldr` at 0xc33b8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc33bc | load | 1 | `[sp-28] +0x4c` | `-` |
| `ldr` at 0xc33cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc33d0 | load | 4 | `[sp-28] +0x50` | `-` |
| `str` at 0xc33d4 | store | 4 | `sp -0x640` | `[[sp-28]+80]` |
| `ldr` at 0xc33dc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc33e8 | store | 4 | `sp -0x640` | `ret(0xfde7c)` |
| `ldr` at 0xc33ec | load | 4 | `sp -0x640` | `-` |
| `str` at 0xc33f0 | store | 4 | `sp -0x2dc` | `[sp-1600]` |
| `ldr` at 0xc33f4 | load | 4 | `sp -0x2ec` | `-` |
| `str` at 0xc3404 | store | 4 | `sp -0x6b0` | `0x0` |
| `str` at 0xc3414 | store | 4 | `sp -0x6ac` | `sp-1340` |
| `ldr` at 0xc341c | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0xc3420 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0xc3424 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0xc3430 | store | 4 | `sp -0x3ec` | `ret(0x8a1e4)` |
| `ldr` at 0xc3434 | load | 4 | `sp -0x3ec` | `-` |
| `ldr` at 0xc3458 | load | 4 | `abs 0xc3f08` | `-` |
| `ldrb` at 0xc3460 | load | 1 | `sp -0x53c` | `-` |
| `str` at 0xc34a0 | store | 4 | `sp -0x6b0` | `0x1` |
| `ldr` at 0xc34e0 | load | 4 | `sp -0x3ec` | `-` |
| `ldr` at 0xc34e8 | load | 4 | `abs 0xc3f08` | `-` |
| `ldrb` at 0xc34f0 | load | 1 | `sp -0x53c` | `-` |
| `str` at 0xc3530 | store | 4 | `sp -0x6b0` | `0x1` |
| `str` at 0xc3554 | store | 4 | `sp -0x2c0` | `ret(0x28230)` |
| `ldr` at 0xc3594 | load | 4 | `abs 0xc3f0c` | `-` |
| `ldr` at 0xc35c4 | load | 4 | `abs 0xc3f0c` | `-` |
| `ldr` at 0xc35e0 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0xc3630 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc3634 | load | 1 | `[sp-28] +0x20` | `-` |
| `str` at 0xc3660 | store | 4 | `sp -0x648` | `ret(0x12a4e8)` |
| `ldr` at 0xc3664 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc3668 | load | 1 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0xc3678 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc367c | load | 4 | `[sp-28] +0x30` | `-` |
| `ldr` at 0xc3680 | load | 4 | `sp -0x648` | `-` |
| `ldr` at 0xc3690 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc36a0 | load | 4 | `sp -0x648` | `-` |
| `ldr` at 0xc36cc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc36d0 | load | 1 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0xc36e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc36e4 | load | 4 | `[sp-28] +0x30` | `-` |
| `str` at 0xc36e8 | store | 4 | `sp -0x64c` | `[[sp-28]+48]` |
| `ldr` at 0xc36f0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc36fc | store | 4 | `sp -0x64c` | `ret(0xfde7c)` |
| `ldr` at 0xc3700 | load | 4 | `sp -0x64c` | `-` |
| `str` at 0xc370c | store | 4 | `sp -0x2a4` | `ret(0x12a6b8)` |
| `ldr` at 0xc3710 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0xc3744 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0xc376c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc3770 | load | 1 | `[sp-28] +0x20` | `-` |
| `str` at 0xc379c | store | 4 | `sp -0x650` | `ret(0x12a4e8)` |
| `ldr` at 0xc37a0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc37a4 | load | 1 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0xc37b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc37b8 | load | 4 | `[sp-28] +0x30` | `-` |
| `ldr` at 0xc37bc | load | 4 | `sp -0x650` | `-` |
| `ldr` at 0xc37cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc37dc | load | 4 | `sp -0x650` | `-` |
| `ldr` at 0xc3808 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc380c | load | 1 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0xc381c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc3820 | load | 4 | `[sp-28] +0x30` | `-` |
| `str` at 0xc3824 | store | 4 | `sp -0x654` | `[[sp-28]+48]` |
| `ldr` at 0xc382c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc3838 | store | 4 | `sp -0x654` | `ret(0xfde7c)` |
| `ldr` at 0xc3844 | load | 4 | `sp -0x654` | `-` |
| `str` at 0xc3854 | store | 4 | `sp -0x2a4` | `ret(0x12a7a4)` |
| `ldr` at 0xc3858 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0xc388c | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0xc38cc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc38d0 | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0xc38dc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc38e0 | load | 1 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0xc38f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc38f4 | load | 4 | `[sp-28] +0x30` | `-` |
| `str` at 0xc38f8 | store | 4 | `sp -0x658` | `[[sp-28]+48]` |
| `ldr` at 0xc3900 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc390c | store | 4 | `sp -0x658` | `ret(0xfde7c)` |
| `ldr` at 0xc3910 | load | 4 | `sp -0x658` | `-` |
| `str` at 0xc3914 | store | 4 | `sp -0x3ec` | `[sp-1624]` |
| `ldr` at 0xc3918 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc391c | load | 1 | `[sp-28] +0x20` | `-` |
| `str` at 0xc3968 | store | 4 | `sp -0x3f0` | `0x0` |
| `ldr` at 0xc396c | load | 4 | `sp -0x3f0` | `-` |
| `ldr` at 0xc3970 | load | 4 | `sp -0x3ec` | `-` |
| `ldr` at 0xc397c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc3980 | load | 4 | `sp -0x3f0` | `-` |
| `ldrb` at 0xc3994 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc39a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc39a8 | load | 4 | `sp -0x3f0` | `-` |
| `ldr` at 0xc39bc | load | 4 | `(unknown)` | `-` |
| `str` at 0xc39c4 | store | 4 | `sp -0x65c` | `?` |
| `ldr` at 0xc39cc | load | 4 | `sp -0x3f0` | `-` |
| `ldr` at 0xc39d4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc39e8 | store | 4 | `sp -0x65c` | `?` |
| `ldr` at 0xc39f8 | load | 4 | `sp -0x65c` | `-` |
| `str` at 0xc3a08 | store | 4 | `sp -0x2a4` | `ret(0x125b8c)` |
| `ldr` at 0xc3a0c | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0xc3a40 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0xc3a58 | load | 4 | `sp -0x3f0` | `-` |
| `ldrb` at 0xc3a74 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc3a84 | load | 4 | `sp -0x3f0` | `-` |
| `str` at 0xc3a8c | store | 4 | `sp -0x3f0` | `[sp-1008]+1` |
| `ldr` at 0xc3a94 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc3a98 | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0xc3ac0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc3ac4 | load | 1 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0xc3ad4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc3ad8 | load | 4 | `[sp-28] +0x30` | `-` |
| `str` at 0xc3adc | store | 4 | `sp -0x660` | `[[sp-28]+48]` |
| `ldr` at 0xc3ae4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc3af0 | store | 4 | `sp -0x660` | `ret(0xfde7c)` |
| `ldr` at 0xc3af4 | load | 4 | `sp -0x660` | `-` |
| `str` at 0xc3af8 | store | 4 | `sp -0x3ec` | `[sp-1632]` |
| `ldr` at 0xc3afc | load | 4 | `sp -0x3ec` | `-` |
| `ldr` at 0xc3b10 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc3b14 | load | 1 | `[sp-28] +0x20` | `-` |
| `str` at 0xc3b58 | store | 4 | `sp -0x3f0` | `0x0` |
| `ldr` at 0xc3b5c | load | 4 | `sp -0x3f0` | `-` |
| `ldr` at 0xc3b60 | load | 4 | `sp -0x3ec` | `-` |
| `ldr` at 0xc3b6c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc3b70 | load | 4 | `sp -0x3f0` | `-` |
| `ldrb` at 0xc3b90 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc3ba0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc3ba4 | load | 4 | `sp -0x3f0` | `-` |
| `ldr` at 0xc3bc4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc3bcc | store | 4 | `sp -0x664` | `?` |
| `ldr` at 0xc3bd4 | load | 4 | `sp -0x3f0` | `-` |
| `ldr` at 0xc3be8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc3bfc | store | 4 | `sp -0x664` | `?` |
| `ldr` at 0xc3c00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc3c04 | load | 4 | `sp -0x3f0` | `-` |
| `ldrb` at 0xc3c24 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc3c34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc3c38 | load | 4 | `sp -0x3f0` | `-` |
| `ldr` at 0xc3c58 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc3c60 | store | 4 | `sp -0x668` | `?` |
| `ldr` at 0xc3c68 | load | 4 | `sp -0x3f0` | `-` |
| `ldr` at 0xc3c7c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc3c90 | store | 4 | `sp -0x668` | `?` |
| `ldr` at 0xc3c94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc3c98 | load | 4 | `sp -0x3f0` | `-` |
| `ldrb` at 0xc3cb8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc3cc8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc3ccc | load | 4 | `sp -0x3f0` | `-` |
| `ldr` at 0xc3cec | load | 4 | `(unknown)` | `-` |
| `str` at 0xc3cf4 | store | 4 | `sp -0x66c` | `?` |
| `ldr` at 0xc3cfc | load | 4 | `sp -0x3f0` | `-` |
| `ldr` at 0xc3d10 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc3d24 | store | 4 | `sp -0x66c` | `?` |
| `ldr` at 0xc3d2c | load | 4 | `sp -0x664` | `-` |
| `ldr` at 0xc3d30 | load | 4 | `sp -0x668` | `-` |
| `ldr` at 0xc3d34 | load | 4 | `sp -0x66c` | `-` |
| `str` at 0xc3d40 | store | 4 | `sp -0x2a4` | `ret(0x125a58)` |
| `ldr` at 0xc3d44 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0xc3d78 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0xc3d90 | load | 4 | `sp -0x3f0` | `-` |
| `ldr` at 0xc3d9c | load | 4 | `sp -0x3f0` | `-` |
| `str` at 0xc3da4 | store | 4 | `sp -0x3f0` | `[sp-1008]+1` |
| `ldr` at 0xc3dac | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc3db0 | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0xc3dd8 | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0xc3de4 | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0xc3df0 | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0xc3dfc | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0xc3e08 | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0xc3e14 | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0xc3e20 | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0xc3e2c | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0xc3e38 | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0xc3e44 | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0xc3e50 | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0xc3e5c | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0xc3e6c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc3e70 | load | 1 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0xc3e80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc3e84 | load | 4 | `[sp-28] +0x30` | `-` |
| `str` at 0xc3e88 | store | 4 | `sp -0x670` | `[[sp-28]+48]` |
| `ldr` at 0xc3e90 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc3e9c | store | 4 | `sp -0x670` | `ret(0xfde7c)` |
| `ldr` at 0xc3ea0 | load | 4 | `sp -0x670` | `-` |
| `ldr` at 0xc3eac | load | 4 | `sp -0x670` | `-` |
| `ldr` at 0xc3eb8 | load | 4 | `sp -0x670` | `-` |
| `ldr` at 0xc3ec8 | load | 4 | `sp -0x670` | `-` |
| `str` at 0xc3edc | store | 4 | `sp -0x3ec` | `0x1` |
| `str` at 0xc3ee8 | store | 4 | `sp -0x3ec` | `0x2` |
| `str` at 0xc3f68 | store | 4 | `sp -0x3ec` | `0x3` |
| `str` at 0xc3f74 | store | 4 | `sp -0x3ec` | `0x0` |
| `ldr` at 0xc3f78 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc3f7c | load | 1 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0xc3f8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc3f90 | load | 4 | `[sp-28] +0x30` | `-` |
| `str` at 0xc3f94 | store | 4 | `sp -0x674` | `[[sp-28]+48]` |
| `ldr` at 0xc3f9c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc3fa8 | store | 4 | `sp -0x674` | `ret(0xfde7c)` |
| `ldr` at 0xc3fac | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc3fb0 | load | 1 | `[sp-28] +0x34` | `-` |
| `ldr` at 0xc3fc0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc3fc4 | load | 4 | `[sp-28] +0x38` | `-` |
| `str` at 0xc3fc8 | store | 4 | `sp -0x678` | `[[sp-28]+56]` |
| `ldr` at 0xc3fd0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc3fdc | store | 4 | `sp -0x678` | `ret(0xfde7c)` |
| `ldr` at 0xc3fe4 | load | 4 | `sp -0x674` | `-` |
| `ldr` at 0xc3fe8 | load | 4 | `sp -0x678` | `-` |
| `str` at 0xc3ff8 | store | 4 | `sp -0x3f0` | `ret(0x4ed88)` |
| `ldr` at 0xc3ffc | load | 4 | `sp -0x3f0` | `-` |
| `ldr` at 0xc4030 | load | 4 | `sp -0x3f0` | `-` |
| `ldr` at 0xc4064 | load | 4 | `sp -0x3f0` | `-` |
| `ldr` at 0xc408c | load | 4 | `abs 0xc3f10` | `-` |
| `ldr` at 0xc4090 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc4094 | load | 4 | `abs 0x1b0eb4` | `-` |
| `ldr` at 0xc4098 | load | 4 | `[sp-28] +0x1c` | `-` |
| `str` at 0xc40a8 | store | 4 | `sp -0x2a4` | `ret(unknown)` |
| `ldr` at 0xc40bc | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0xc40e0 | load | 4 | `abs 0xc3f14` | `-` |
| `ldr` at 0xc40e4 | load | 4 | `abs 0x1b0eb0` | `-` |
| `ldr` at 0xc410c | load | 4 | `abs 0xc3f18` | `-` |
| `ldr` at 0xc4114 | load | 4 | `abs 0x1b0ec4` | `-` |
| `ldr` at 0xc4118 | load | 4 | `sp -0x2a0` | `-` |
| `str` at 0xc412c | store | 4 | `sp -0x2a0` | `ret(unknown)` |
| `str` at 0xc4158 | store | 4 | `sp -0x2a4` | `ret(0x6aaa0)` |
| `ldr` at 0xc416c | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0xc4178 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc417c | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0xc41a4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc41a8 | load | 1 | `[sp-28] +0x34` | `-` |
| `ldr` at 0xc41b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc41bc | load | 4 | `[sp-28] +0x38` | `-` |
| `ldr` at 0xc41c0 | load | 4 | `abs 0xc3f1c` | `-` |
| `str` at 0xc41cc | store | 4 | `sp -0x67c` | `?` |
| `ldr` at 0xc41d0 | load | 4 | `sp -0x67c` | `-` |
| `ldr` at 0xc41d8 | load | 4 | `sp -0x67c` | `-` |
| `ldr` at 0xc41e4 | load | 4 | `sp -0x67c` | `-` |
| `str` at 0xc41f0 | store | 4 | `sp -0x67c` | `?` |
| `ldr` at 0xc41f8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc4204 | store | 4 | `sp -0x67c` | `ret(0xfde7c)` |
| `ldr` at 0xc4208 | load | 4 | `abs 0xc3f1c` | `-` |
| `ldr` at 0xc420c | load | 4 | `sp -0x67c` | `-` |
| `ldr` at 0xc422c | load | 4 | `sp -0x67c` | `-` |
| `str` at 0xc4234 | store | 4 | `sp -0x67c` | `?` |
| `ldr` at 0xc4238 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc423c | load | 1 | `[sp-28] +0x3c` | `-` |
| `ldr` at 0xc424c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc4250 | load | 4 | `[sp-28] +0x40` | `-` |
| `str` at 0xc4258 | store | 4 | `sp -0x680` | `?` |
| `ldr` at 0xc4260 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc426c | store | 4 | `sp -0x680` | `ret(0xfde7c)` |
| `ldr` at 0xc4270 | load | 4 | `sp -0x680` | `-` |
| `str` at 0xc4278 | store | 4 | `sp -0x680` | `?` |
| `ldr` at 0xc427c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc4280 | load | 1 | `[sp-28] +0x44` | `-` |
| `ldr` at 0xc4290 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc4294 | load | 4 | `[sp-28] +0x48` | `-` |
| `ldr` at 0xc4298 | load | 4 | `abs 0xc3f20` | `-` |
| `str` at 0xc42a4 | store | 4 | `sp -0x684` | `?` |
| `ldr` at 0xc42a8 | load | 4 | `sp -0x684` | `-` |
| `ldr` at 0xc42b0 | load | 4 | `sp -0x684` | `-` |
| `str` at 0xc42c0 | store | 4 | `sp -0x684` | `?` |
| `ldr` at 0xc42c8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc42d4 | store | 4 | `sp -0x684` | `ret(0xfde7c)` |
| `ldr` at 0xc42d8 | load | 4 | `abs 0xc3f20` | `-` |
| `ldr` at 0xc42dc | load | 4 | `sp -0x684` | `-` |
| `ldr` at 0xc42f8 | load | 4 | `sp -0x684` | `-` |
| `str` at 0xc4300 | store | 4 | `sp -0x684` | `?` |
| `ldr` at 0xc4304 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc4308 | load | 1 | `[sp-28] +0x4c` | `-` |
| `ldr` at 0xc4318 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc431c | load | 4 | `[sp-28] +0x50` | `-` |
| `ldr` at 0xc4320 | load | 4 | `abs 0xc3f28` | `-` |
| `str` at 0xc432c | store | 4 | `sp -0x688` | `?` |
| `ldr` at 0xc4330 | load | 4 | `sp -0x688` | `-` |
| `ldr` at 0xc4338 | load | 4 | `sp -0x688` | `-` |
| `str` at 0xc4348 | store | 4 | `sp -0x688` | `?` |
| `ldr` at 0xc4350 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc435c | store | 4 | `sp -0x688` | `ret(0xfde7c)` |
| `ldr` at 0xc4360 | load | 4 | `abs 0xc3f28` | `-` |
| `ldr` at 0xc4364 | load | 4 | `sp -0x688` | `-` |
| `ldr` at 0xc4380 | load | 4 | `sp -0x688` | `-` |
| `str` at 0xc4388 | store | 4 | `sp -0x688` | `?` |
| `ldr` at 0xc438c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc4390 | load | 1 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0xc43a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc43a4 | load | 4 | `[sp-28] +0x30` | `-` |
| `ldr` at 0xc43a8 | load | 4 | `abs 0xc3f24` | `-` |
| `str` at 0xc43b4 | store | 4 | `sp -0x68c` | `?` |
| `ldr` at 0xc43b8 | load | 4 | `sp -0x68c` | `-` |
| `ldr` at 0xc43c0 | load | 4 | `sp -0x68c` | `-` |
| `ldr` at 0xc43cc | load | 4 | `sp -0x68c` | `-` |
| `str` at 0xc43e4 | store | 4 | `sp -0x68c` | `?` |
| `ldr` at 0xc43ec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc43f8 | store | 4 | `sp -0x68c` | `ret(0xfde7c)` |
| `ldr` at 0xc43fc | load | 4 | `abs 0xc3f24` | `-` |
| `ldr` at 0xc4400 | load | 4 | `sp -0x68c` | `-` |
| `ldr` at 0xc442c | load | 4 | `sp -0x68c` | `-` |
| `str` at 0xc4434 | store | 4 | `sp -0x68c` | `?` |
| `ldr` at 0xc4438 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc443c | load | 1 | `[sp-28] +0x54` | `-` |
| `ldr` at 0xc444c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc4450 | load | 4 | `[sp-28] +0x58` | `-` |
| `ldr` at 0xc4454 | load | 4 | `abs 0xc3f28` | `-` |
| `str` at 0xc4460 | store | 4 | `sp -0x690` | `?` |
| `ldr` at 0xc4464 | load | 4 | `sp -0x690` | `-` |
| `ldr` at 0xc446c | load | 4 | `sp -0x690` | `-` |
| `str` at 0xc447c | store | 4 | `sp -0x690` | `?` |
| `ldr` at 0xc4484 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc4490 | store | 4 | `sp -0x690` | `ret(0xfde7c)` |
| `ldr` at 0xc4494 | load | 4 | `abs 0xc3f28` | `-` |
| `ldr` at 0xc4498 | load | 4 | `sp -0x690` | `-` |
| `ldr` at 0xc44b4 | load | 4 | `sp -0x690` | `-` |
| `str` at 0xc44bc | store | 4 | `sp -0x690` | `?` |
| `ldr` at 0xc44cc | load | 4 | `sp -0x684` | `-` |
| `str` at 0xc44d0 | store | 4 | `sp -0x6b0` | `[sp-1668]` |
| `ldr` at 0xc44d4 | load | 4 | `sp -0x688` | `-` |
| `str` at 0xc44d8 | store | 4 | `sp -0x6ac` | `[sp-1672]` |
| `ldr` at 0xc44dc | load | 4 | `sp -0x68c` | `-` |
| `str` at 0xc44e0 | store | 4 | `sp -0x6a8` | `[sp-1676]` |
| `ldr` at 0xc44e4 | load | 4 | `sp -0x690` | `-` |
| `str` at 0xc44e8 | store | 4 | `sp -0x6a4` | `[sp-1680]` |
| `ldr` at 0xc44f0 | load | 4 | `abs 0xc3f2c` | `-` |
| `ldr` at 0xc44f4 | load | 4 | `sp -0x67c` | `-` |
| `ldr` at 0xc44f8 | load | 4 | `sp -0x680` | `-` |
| `ldr` at 0xc452c | load | 4 | `abs 0xc3f30` | `-` |
| `str` at 0xc4578 | store | 4 | `sp -0x3f0` | `ret(0x202c8)` |
| `str` at 0xc458c | store | 4 | `sp -0x3ec` | `ret(0x204f0)` |
| `ldr` at 0xc4598 | load | 4 | `sp -0x3ec` | `-` |
| `ldr` at 0xc459c | load | 4 | `ret(0x204f0) +0x14` | `-` |
| `ldr` at 0xc45bc | load | 4 | `sp -0x3ec` | `-` |
| `ldr` at 0xc45c0 | load | 4 | `[sp-1004] +0x10` | `-` |
| `ldr` at 0xc45dc | load | 4 | `sp -0x3ec` | `-` |
| `ldr` at 0xc45e8 | load | 4 | `[sp-1004] +0xc` | `-` |
| `ldr` at 0xc45f8 | load | 4 | `sp -0x3ec` | `-` |
| `ldr` at 0xc4604 | load | 4 | `[sp-1004] +0x8` | `-` |
| `ldr` at 0xc4614 | load | 4 | `sp -0x3ec` | `-` |
| `ldr` at 0xc4620 | load | 4 | `[sp-1004] +0x4` | `-` |
| `ldr` at 0xc4630 | load | 4 | `sp -0x3ec` | `-` |
| `ldr` at 0xc463c | load | 4 | `[sp-1004] +0x0` | `-` |
| `ldr` at 0xc4648 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc464c | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0xc4674 | load | 4 | `abs 0xc3f3c` | `-` |
| `ldrb` at 0xc467c | load | 1 | `sp -0x2b2` | `-` |
| `ldr` at 0xc4680 | load | 4 | `abs 0xc3f38` | `-` |
| `ldrb` at 0xc468c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc4698 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xc46a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc46ac | load | 4 | `abs 0xc3f38` | `-` |
| `ldrb` at 0xc46b8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc46c4 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xc46cc | load | 4 | `abs 0xc3f3c` | `-` |
| `ldrb` at 0xc46d4 | load | 1 | `sp -0x2b2` | `-` |
| `ldr` at 0xc46fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc4704 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc4710 | load | 4 | `abs 0xc3f38` | `-` |
| `ldrb` at 0xc471c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc4728 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xc4730 | load | 4 | `abs 0xc3f3c` | `-` |
| `ldrb` at 0xc4738 | load | 1 | `sp -0x2b2` | `-` |
| `ldr` at 0xc4760 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc4768 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc47a8 | store | 4 | `sp -0x698` | `?` |
| `ldr` at 0xc47ac | load | 4 | `sp -0x698` | `-` |
| `ldr` at 0xc47b8 | load | 4 | `sp -0x698` | `-` |
| `ldr` at 0xc47c4 | load | 4 | `sp -0x698` | `-` |
| `ldr` at 0xc47d0 | load | 4 | `sp -0x698` | `-` |
| `ldr` at 0xc47e0 | load | 4 | `sp -0x698` | `-` |
| `ldr` at 0xc47ec | load | 4 | `sp -0x698` | `-` |
| `ldr` at 0xc47fc | load | 4 | `abs 0xc3f40` | `-` |
| `strh` at 0xc4808 | store | 2 | `sp -0x2ce` | `0x0` |
| `ldrb` at 0xc4818 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc4824 | store | 4 | `sp -0x694` | `ret(0x3deec)` |
| `ldr` at 0xc4828 | load | 4 | `sp -0x694` | `-` |
| `ldr` at 0xc4834 | load | 4 | `sp -0x694` | `-` |
| `ldr` at 0xc4844 | load | 4 | `abs 0xc3f40` | `-` |
| `ldr` at 0xc4848 | load | 4 | `abs 0xc3f40` | `-` |
| `ldrh` at 0xc4850 | load | 2 | `sp -0x2ce` | `-` |
| `strh` at 0xc485c | store | 2 | `sp -0x2ce` | `?` |
| `ldr` at 0xc4864 | load | 4 | `abs 0xc3f40` | `-` |
| `strh` at 0xc4870 | store | 2 | `sp -0x2ce` | `0x0` |
| `ldr` at 0xc4884 | load | 4 | `abs 0xc3f34` | `-` |
| `ldr` at 0xc4890 | load | 4 | `abs 0xc3f40` | `-` |
| `strh` at 0xc489c | store | 2 | `sp -0x2ce` | `0x0` |
| `ldrb` at 0xc48b8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc48cc | load | 4 | `abs 0xc3f38` | `-` |
| `ldrb` at 0xc48d8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc48e4 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xc48ec | load | 4 | `abs 0xc3f3c` | `-` |
| `ldrb` at 0xc48f4 | load | 1 | `sp -0x2b2` | `-` |
| `ldr` at 0xc491c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc4924 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc492c | store | 4 | `sp -0x69c` | `?` |
| `ldr` at 0xc4930 | load | 4 | `sp -0x69c` | `-` |
| `str` at 0xc4938 | store | 4 | `sp -0x69c` | `?` |
| `ldrb` at 0xc4944 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc494c | load | 4 | `sp -0x69c` | `-` |
| `str` at 0xc4954 | store | 4 | `sp -0x69c` | `?` |
| `ldr` at 0xc495c | load | 4 | `abs 0xc3f38` | `-` |
| `ldrb` at 0xc4968 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc4974 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xc497c | load | 4 | `abs 0xc3f3c` | `-` |
| `ldrb` at 0xc4984 | load | 1 | `sp -0x2b2` | `-` |
| `ldr` at 0xc49ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc49b4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc49b8 | store | 4 | `sp -0x69c` | `?` |
| `ldr` at 0xc49bc | load | 4 | `abs 0xc3f40` | `-` |
| `ldrh` at 0xc49c4 | load | 2 | `sp -0x2ce` | `-` |
| `ldr` at 0xc49c8 | load | 4 | `sp -0x69c` | `-` |
| `str` at 0xc49dc | store | 4 | `sp -0x2a4` | `ret(0x75a8c)` |
| `ldr` at 0xc49e0 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0xc4a14 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0xc4a48 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0xc4a54 | load | 4 | `abs 0xc3f44` | `-` |
| `ldr` at 0xc4a58 | load | 4 | `abs 0x1b0ce8` | `-` |
| `ldr` at 0xc4a64 | load | 4 | `abs 0xc3f44` | `-` |
| `ldr` at 0xc4a70 | load | 4 | `abs 0x1b0ce8` | `-` |
| `ldr` at 0xc4a74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc4aa4 | load | 4 | `sp -0x2a0` | `-` |
| `str` at 0xc4ab4 | store | 4 | `(unknown)` | `sp-668` |
| `ldr` at 0xc4ab8 | load | 4 | `abs 0xc3f48` | `-` |
| `ldr` at 0xc4ac0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc4ac4 | load | 4 | `sp -0x2a0` | `-` |
| `str` at 0xc4ad0 | store | 4 | `sp -0x2a4` | `ret(0xfd0dc)` |
| `ldr` at 0xc4ad4 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0xc4ae0 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0xc4aec | load | 4 | `abs 0xc3f4c` | `-` |
| `str` at 0xc4af0 | store | 4 | `(unknown)` | `0x16dd9c` |
| `str` at 0xc4af4 | store | 4 | `(unknown)` | `ret(0x219a4)` |
| `ldr` at 0xc4afc | load | 4 | `abs 0xc3f5c` | `-` |
| `ldr` at 0xc4b00 | load | 4 | `abs 0xc3f50` | `-` |
| `ldr` at 0xc4b0c | load | 4 | `abs 0xc3f54` | `-` |
| `str` at 0xc4b18 | store | 4 | `sp -0x2a4` | `ret(0x2302c)` |
| `ldr` at 0xc4b1c | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0xc4b28 | load | 4 | `abs 0xc3f58` | `-` |
| `str` at 0xc4b2c | store | 4 | `(unknown)` | `0x16ddb8` |
| `ldr` at 0xc4b30 | load | 4 | `sp -0x2a4` | `-` |
| `str` at 0xc4b34 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xc4b3c | load | 4 | `abs 0xc3f5c` | `-` |
| `ldr` at 0xc4b40 | load | 4 | `abs 0xc3f60` | `-` |
| `ldr` at 0xc4b4c | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xc4b58 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xc4b58 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xc4b58 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc4b58 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc4b58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbeed0 | `(unknown callee) 0x6dc0c` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0xbeee8 | `(unknown callee) 0x22ca0` | `0x31b628` | `?` | `?` | `?` |
| `bl` at 0xbef20 | `(unknown callee) 0x22070` | `0x9` | `0x16d8e8` | `0x22d` | `0x0` |
| `bl` at 0xbef48 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xbef88 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x0` | `?` | `?` |
| `bl` at 0xbefa0 | `(unknown callee) 0x6e140` | `[sp-1344]` | `?` | `?` | `?` |
| `bl` at 0xbf028 | `(unknown callee) 0xb7e54` | `[sp-1360]` | `?` | `[sp-1348]` | `?` |
| `bl` at 0xbf078 | `(unknown callee) 0xb7f6c` | `[sp-1368]` | `0xfffffd65` | `?` | `?` |
| `bl` at 0xbf0b0 | `(unknown callee) 0x81b18` | `?` | `[sp-690]:1+27` | `?` | `[sp-690]:1+27` |
| `bl` at 0xbf108 | `(unknown callee) 0x29908` | `0x16d954` | `[sp-684]` | `?` | `[sp-690]:1` |
| `bl` at 0xbf120 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xbf500 | `(unknown callee) 0x28230` | `0x3` | `[sp-688]` | `[sp-688]` | `[sp-688]` |
| `bl` at 0xbf520 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xbf538 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-704]` | `sp-672` |
| `bl` at 0xbf544 | `(unknown callee) 0x28230` | `0x1` | `[sp-688]` | `[sp-688]` | `[sp-688]` |
| `bl` at 0xbf554 | `(unknown callee) 0x28230` | `0x2` | `?` | `?` | `ret(0x28230)` |
| `bl` at 0xbf578 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xbf590 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-704]` | `sp-672` |
| `bl` at 0xbf5f8 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xbf614 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xbf634 | `(unknown callee) 0xa3c7c` | `0x1` | `?` | `?` | `?` |
| `bl` at 0xbf6b0 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-704]` | `sp-672` |
| `bl` at 0xbf6c8 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-716]` | `sp-672` |
| `bl` at 0xbf6d0 | `(unknown callee) 0xa3c7c` | `0x2` | `?` | `?` | `?` |
| `bl` at 0xbf74c | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-704]` | `sp-672` |
| `bl` at 0xbf764 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-716]` | `sp-672` |
| `bl` at 0xbf76c | `(unknown callee) 0xa3c7c` | `0x3` | `?` | `?` | `?` |
| `bl` at 0xbf7e8 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-704]` | `sp-672` |
| `bl` at 0xbf800 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-716]` | `sp-672` |
| `bl` at 0xbf808 | `(unknown callee) 0xa3c7c` | `0x4` | `?` | `?` | `?` |
| `bl` at 0xbf884 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-704]` | `sp-672` |
| `bl` at 0xbf89c | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-716]` | `sp-672` |
| `bl` at 0xbf8b8 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xbf8c0 | `(unknown callee) 0xa3c7c` | `0x5` | `?` | `?` | `?` |
| `bl` at 0xbf8fc | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-704]` | `sp-672` |
| `bl` at 0xbf918 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[[sp-700]+16]` | `sp-672` |
| `bl` at 0xbf920 | `(unknown callee) 0xa3c7c` | `0xa` | `?` | `?` | `?` |
| `bl` at 0xbf95c | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-704]` | `sp-672` |
| `bl` at 0xbf978 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[[sp-700]+16]` | `sp-672` |
| `bl` at 0xbf9dc | `(unknown callee) 0x6dcc4` | `?` | `[sp-690]:1` | `?` | `[sp-690]:1` |
| `bl` at 0xbf9e8 | `(unknown callee) 0x6debc` | `?` | `[sp-680]` | `[sp-680]` | `?` |
| `bl` at 0xbfa10 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xbfa2c | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xbfa34 | `(unknown callee) 0x2031c` | `[sp-708]` | `?` | `?` | `?` |
| `bl` at 0xbfa5c | `(unknown callee) 0x95b5c` | `sp-672` | `sp-668` | `[sp-708]` | `ret(0x2031c)` |
| `bl` at 0xbfa78 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xbfa8c | `(unknown callee) 0x20544` | `0x16d974` | `sp-708` | `0xa` | `sp-708` |
| `bl` at 0xbfaa8 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `ret(0x20544)` | `sp-672` |
| `bl` at 0xbfac4 | `(unknown callee) 0x20544` | `[sp-708]+1` | `sp-708` | `0xa` | `[sp-708]+1` |
| `bl` at 0xbfae0 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `ret(0x20544)` | `sp-672` |
| `bl` at 0xbfaf8 | `(unknown callee) 0x20544` | `[sp-708]+1` | `0x0` | `0xa` | `[sp-708]+1` |
| `bl` at 0xbfb14 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `ret(0x20544)` | `sp-672` |
| `bl` at 0xbfb40 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xbfb74 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x1` | `[sp-688]` | `?` |
| `bl` at 0xbfba0 | `(unknown callee) 0xbe1f4` | `?` | `[sp-690]:1` | `[sp-1392]` | `[sp-690]:1` |
| `bl` at 0xbfbac | `(unknown callee) 0x6e140` | `ret(0xbe1f4)` | `?` | `?` | `ret(0xbe1f4)` |
| `bl` at 0xbfbbc | `(unknown callee) 0x3e38c` | `ret(0x6e140)` | `?` | `?` | `ret(0x6e140)` |
| `bl` at 0xbfbd0 | `(unknown callee) 0x6e190` | `?` | `?` | `?` | `?` |
| `bl` at 0xbfbf8 | `(unknown callee) 0x120340` | `?` | `sp-724` | `?` | `sp-724` |
| `bl` at 0xbfc04 | `(unknown callee) 0x6e140` | `ret(0x120340)` | `?` | `?` | `ret(0x120340)` |
| `bl` at 0xbfc14 | `(unknown callee) 0x3e38c` | `ret(0x6e140)` | `?` | `?` | `ret(0x6e140)` |
| `bl` at 0xbfc28 | `(unknown callee) 0x6e190` | `?` | `?` | `?` | `?` |
| `bl` at 0xbfc54 | `(unknown callee) 0x120398` | `?` | `?` | `?` | `?` |
| `bl` at 0xbfc68 | `(unknown callee) 0x293f8` | `0x16d97c` | `?` | `?` | `ret(0x120398)` |
| `bl` at 0xbfc80 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xbfcac | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xbfcc8 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xbfcfc | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `?` | `?` |
| `bl` at 0xbfd70 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `?` | `?` |
| `bl` at 0xbfda4 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `?` | `?` |
| `bl` at 0xbfdd8 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `?` | `?` |
| `bl` at 0xbfdf0 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `0x8` | `sp-672` |
| `bl` at 0xbfe24 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `?` | `?` |
| `bl` at 0xbfe58 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `?` | `?` |
| `bl` at 0xbfe8c | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `?` | `?` |
| `bl` at 0xbfec0 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `?` | `?` |
| `bl` at 0xbff9c | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `?` | `?` |
| `bl` at 0xbffd0 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `?` | `?` |
| `bl` at 0xbfffc | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc0030 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x1` | `[sp-688]` | `?` |
| `bl` at 0xc005c | `(unknown callee) 0xbe1f4` | `?` | `[sp-690]:1` | `[sp-1408]` | `[sp-690]:1` |
| `bl` at 0xc0094 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x2` | `?` | `?` |
| `bl` at 0xc00d0 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x3` | `[sp-1412]` | `?` |
| `bl` at 0xc010c | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x4` | `?` | `?` |
| `bl` at 0xc012c | `(unknown callee) 0x6e140` | `[sp-1420]` | `?` | `?` | `0x0` |
| `bl` at 0xc013c | `(unknown callee) 0x3e38c` | `ret(0x6e140)` | `?` | `?` | `ret(0x6e140)` |
| `bl` at 0xc0150 | `(unknown callee) 0x6e190` | `?` | `?` | `?` | `?` |
| `bl` at 0xc0178 | `(unknown callee) 0x120340` | `?` | `sp-732` | `?` | `sp-732` |
| `bl` at 0xc0184 | `(unknown callee) 0x6e140` | `ret(0x120340)` | `?` | `?` | `ret(0x120340)` |
| `bl` at 0xc0194 | `(unknown callee) 0x3e38c` | `ret(0x6e140)` | `?` | `?` | `ret(0x6e140)` |
| `bl` at 0xc01a8 | `(unknown callee) 0x6e190` | `?` | `?` | `?` | `?` |
| `bl` at 0xc01d4 | `(unknown callee) 0x120398` | `?` | `?` | `?` | `?` |
| `bl` at 0xc01f8 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc0224 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc066c | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc06a0 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc0708 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc0720 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-752]` | `sp-672` |
| `bl` at 0xc0754 | `(unknown callee) 0x959dc` | `sp-672` | `sp-668` | `([sp-748] + [sp-740])` | `[sp-744]` |
| `mov` at 0xc077c | `(unknown callee) (unknown target)` | `0x0` | `[sp-688]` | `[sp-688]` | `[0x318478]` |
| `bl` at 0xc0790 | `(unknown callee) 0x28230` | `0xd` | `[sp-688]` | `[sp-688]` | `[0x318478]` |
| `bl` at 0xc079c | `(unknown callee) 0x2031c` | `?` | `?` | `?` | `?` |
| `bl` at 0xc07c0 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc07dc | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc07e4 | `(unknown callee) 0x2031c` | `[sp-708]` | `?` | `?` | `?` |
| `bl` at 0xc080c | `(unknown callee) 0x95b5c` | `sp-672` | `sp-668` | `[sp-708]` | `ret(0x2031c)` |
| `bl` at 0xc0818 | `(unknown callee) 0x28230` | `0xe` | `[sp-688]` | `[sp-688]` | `[sp-688]` |
| `bl` at 0xc0838 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc0850 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-704]` | `sp-672` |
| `bl` at 0xc085c | `(unknown callee) 0x28230` | `0x12` | `[sp-688]` | `[sp-688]` | `[sp-688]` |
| `bl` at 0xc087c | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc0894 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-704]` | `sp-672` |
| `bl` at 0xc08b0 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc0b44 | `(unknown callee) 0x28230` | `0x2a` | `?` | `?` | `[sp-680]-36` |
| `bl` at 0xc0b64 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `ret(0x28230)` | `sp-672` |
| `bl` at 0xc0b6c | `(unknown callee) 0x28230` | `0x2b` | `?` | `?` | `?` |
| `bl` at 0xc0b8c | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `ret(0x28230)` | `sp-672` |
| `bl` at 0xc0bc4 | `(unknown callee) 0x6e140` | `[sp-688]` | `[sp-688]` | `[sp-688]` | `[sp-680]` |
| `bl` at 0xc0be0 | `(unknown callee) 0x3e38c` | `?` | `?` | `0xfffffd66` | `?` |
| `bl` at 0xc0c00 | `(unknown callee) 0x3e498` | `[sp-690]:1` | `?` | `?` | `[sp-690]:1` |
| `bl` at 0xc0c50 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc0c70 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-690]:1` | `[sp-690]:1` |
| `bl` at 0xc0ca8 | `(unknown callee) 0x6e140` | `[sp-688]` | `[sp-688]` | `[sp-688]` | `[sp-680]` |
| `bl` at 0xc0cc4 | `(unknown callee) 0x3e38c` | `?` | `?` | `?` | `?` |
| `bl` at 0xc0ce4 | `(unknown callee) 0x3e458` | `[sp-690]:1` | `?` | `sp-24` | `[sp-690]:1` |
| `bl` at 0xc0d30 | `(unknown callee) 0x3e2d0` | `?` | `sp-24` | `sp-24` | `?` |
| `bl` at 0xc0d5c | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc0d74 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-676]` | `sp-672` |
| `bl` at 0xc0d90 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc0db0 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `?` | `?` |
| `bl` at 0xc0e20 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `?` | `?` |
| `bl` at 0xc0e64 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `?` | `?` |
| `bl` at 0xc0f20 | `(unknown callee) 0x29908` | `0x16d990` | `[sp-1448]` | `[sp-1452]` | `?` |
| `bl` at 0xc0f3c | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc0f54 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc0f6c | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-680]` | `sp-672` |
| `bl` at 0xc0f84 | `(unknown callee) 0x7fbc8` | `?` | `[sp-688]` | `sp-24` | `?` |
| `bl` at 0xc0fb0 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc0ff0 | `(unknown callee) 0x201fc` | `?` | `?` | `?` | `?` |
| `bl` at 0xc1010 | `(unknown callee) 0x20550` | `ret(0x201fc)` | `0x0` | `?` | `?` |
| `bl` at 0xc1020 | `(unknown callee) 0x201fc` | `?` | `?` | `?` | `?` |
| `bl` at 0xc1040 | `(unknown callee) 0x20550` | `ret(0x201fc)` | `0x0` | `?` | `?` |
| `bl` at 0xc1078 | `(unknown callee) 0x14fe64` | `[sp-756]` | `[sp-732]` | `0x4` | `?` |
| `bl` at 0xc1090 | `(unknown callee) 0x150058` | `[sp-756]` | `[sp-732]` | `?` | `[sp-756]` |
| `bl` at 0xc10a8 | `(unknown callee) 0x6e140` | `ret(0x150058)` | `?` | `ret(0x150058)` | `?` |
| `bl` at 0xc10b8 | `(unknown callee) 0x3e38c` | `ret(0x6e140)` | `?` | `?` | `ret(0x6e140)` |
| `bl` at 0xc10cc | `(unknown callee) 0x6e190` | `?` | `?` | `?` | `?` |
| `bl` at 0xc10f8 | `(unknown callee) 0x7fcdc` | `?` | `?` | `?` | `?` |
| `bl` at 0xc1140 | `(unknown callee) 0x6e140` | `?` | `?` | `?` | `?` |
| `bl` at 0xc1150 | `(unknown callee) 0x3e38c` | `ret(0x6e140)` | `?` | `?` | `ret(0x6e140)` |
| `bl` at 0xc1164 | `(unknown callee) 0x6e190` | `?` | `?` | `?` | `?` |
| `bl` at 0xc1190 | `(unknown callee) 0x7fe94` | `?` | `?` | `?` | `?` |
| `bl` at 0xc1218 | `(unknown callee) 0x6e140` | `?` | `?` | `?` | `[sp-680]` |
| `bl` at 0xc1240 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc1258 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `0x1` | `sp-672` |
| `bl` at 0xc126c | `(unknown callee) 0x14fe64` | `[sp-752]` | `[sp-744]` | `?` | `?` |
| `bl` at 0xc1280 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `ret(0x14fe64)` | `ret(0x14fe64)` |
| `bl` at 0xc12a8 | `(unknown callee) 0x14fe64` | `[sp-752]` | `[sp-744]` | `?` | `0xfffffd66` |
| `bl` at 0xc12c0 | `(unknown callee) 0x29908` | `0x16d9ac` | `[sp-690]:1` | `ret(0x14fe64)` | `0x1` |
| `bl` at 0xc1324 | `(unknown callee) 0x29908` | `0x16d9d0` | `[sp-690]:1` | `?` | `?` |
| `bl` at 0xc1358 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `?` | `?` |
| `bl` at 0xc138c | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `?` | `?` |
| `bl` at 0xc13a8 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc13c0 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-744]` | `sp-672` |
| `bl` at 0xc13d8 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-752]` | `sp-672` |
| `bl` at 0xc1408 | `(unknown callee) 0x29908` | `0x16d9fc` | `[sp-690]:1` | `[sp-752]` | `[sp-744]` |
| `bl` at 0xc1474 | `(unknown callee) 0x29908` | `0x16da1c` | `[sp-690]:1` | `?` | `?` |
| `bl` at 0xc149c | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `?` | `?` |
| `bl` at 0xc14c4 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `?` | `?` |
| `bl` at 0xc14dc | `(unknown callee) 0x20514` | `[sp-740]` | `?` | `?` | `?` |
| `bl` at 0xc14e4 | `(unknown callee) 0x20514` | `[sp-736]` | `?` | `?` | `?` |
| `bl` at 0xc1504 | `(unknown callee) 0x20550` | `sp-872` | `0x0` | `0x40` | `sp-872` |
| `bl` at 0xc1514 | `(unknown callee) 0x12c5b4` | `0x0` | `sp-796` | `?` | `sp-796` |
| `bl` at 0xc1644 | `(unknown callee) 0x1347dc` | `sp-796` | `?` | `?` | `sp-796` |
| `bl` at 0xc1720 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc1738 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-676]` | `sp-672` |
| `bl` at 0xc1784 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc17a4 | `(unknown callee) 0x959dc` | `sp-672` | `sp-668` | `sp-872` | `0x40` |
| `bl` at 0xc17d0 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc1804 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x1` | `[sp-688]` | `?` |
| `bl` at 0xc1814 | `(unknown callee) 0x2624c` | `0x3` | `[sp-1468]` | `?` | `?` |
| `bl` at 0xc182c | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc1858 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc1888 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc18bc | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x1` | `[sp-688]` | `?` |
| `bl` at 0xc18d0 | `(unknown callee) 0x706a0` | `0x32` | `[sp-1472]` | `0x0` | `?` |
| `bl` at 0xc18e8 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc1914 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc1948 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x1` | `[sp-688]` | `?` |
| `bl` at 0xc1958 | `(unknown callee) 0x2624c` | `0xe` | `[sp-1476]` | `?` | `?` |
| `bl` at 0xc1960 | `(unknown callee) 0x28230` | `0xe` | `?` | `?` | `?` |
| `bl` at 0xc196c | `(unknown callee) 0xa4678` | `ret(0x28230)` | `?` | `?` | `ret(0x28230)` |
| `bl` at 0xc1984 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc19b0 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc19e4 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x1` | `[sp-688]` | `?` |
| `bl` at 0xc19f4 | `(unknown callee) 0x2624c` | `0x12` | `[sp-1480]` | `?` | `?` |
| `bl` at 0xc1a0c | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc1a38 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc1a88 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x1` | `sp-24` | `?` |
| `bl` at 0xc1aa4 | `(unknown callee) 0xb80fc` | `[sp-690]:1` | `[sp-1492]` | `?` | `[sp-690]:1` |
| `bl` at 0xc1af8 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x1` | `[sp-1484]` | `?` |
| `bl` at 0xc1b3c | `(unknown callee) 0x29908` | `0x16da48` | `[0x31b5a8]` | `[0x31b5a8]` | `?` |
| `bl` at 0xc1b4c | `(unknown callee) 0x2624c` | `0x13` | `[0x31b5a8]` | `?` | `0x31b5a0` |
| `bl` at 0xc1b64 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc1b90 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc1bc8 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x2` | `[sp-688]` | `?` |
| `bl` at 0xc1c30 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x1` | `[sp-1516]` | `?` |
| `bl` at 0xc1c40 | `(unknown callee) 0x71c20` | `[sp-1500]` | `0x0` | `?` | `?` |
| `bl` at 0xc1c74 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x1` | `?` | `?` |
| `bl` at 0xc1c80 | `(unknown callee) 0x71c90` | `[sp-1504]` | `?` | `?` | `?` |
| `bl` at 0xc1cb4 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x1` | `[sp-1516]` | `?` |
| `bl` at 0xc1cec | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x2` | `?` | `?` |
| `bl` at 0xc1d0c | `(unknown callee) 0x71ce8` | `[sp-1508]` | `[sp-1512]` | `0x0` | `?` |
| `bl` at 0xc1d28 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc1d40 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc1d48 | `(unknown callee) 0xbb818` | `[sp-688]` | `[sp-688]` | `[sp-688]` | `0x100a` |
| `bl` at 0xc1d74 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc1db8 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x2` | `[sp-688]` | `?` |
| `bl` at 0xc1e18 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x3` | `?` | `?` |
| `bl` at 0xc1e70 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x1` | `?` | `?` |
| `bl` at 0xc1eb8 | `(unknown callee) 0x818a0` | `?` | `[sp-690]:1+27` | `[sp-696]` | `[sp-690]:1+27` |
| `bl` at 0xc1f04 | `(unknown callee) 0x29908` | `0x16da6c` | `[sp-690]:1` | `?` | `?` |
| `bl` at 0xc1f20 | `(unknown callee) 0xb7e54` | `[0x31b5a8]` | `?` | `?` | `0x31b5a0` |
| `bl` at 0xc1f3c | `(unknown callee) 0xbb818` | `sp-24` | `?` | `ret(0xb7e54)` | `[sp-690]:1` |
| `bl` at 0xc200c | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc2068 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x2` | `[sp-688]` | `?` |
| `bl` at 0xc20b0 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x1` | `?` | `?` |
| `bl` at 0xc20d8 | `(unknown callee) 0x80a78` | `?` | `[sp-1536]` | `[sp-1540]` | `?` |
| `mov` at 0xc20f0 | `(unknown callee) (unknown target)` | `0x6` | `[sp-688]` | `[sp-688]` | `[0x1a025c]` |
| `bl` at 0xc2118 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc2130 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-704]` | `sp-672` |
| `bl` at 0xc216c | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc21a4 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x1` | `[sp-688]` | `?` |
| `bl` at 0xc21e4 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x2` | `?` | `?` |
| `mov` at 0xc2218 | `(unknown callee) (unknown target)` | `?` | `[sp-1544]` | `[sp-1548]` | `?` |
| `bl` at 0xc2240 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc2258 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-704]` | `sp-672` |
| `bl` at 0xc2294 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc22cc | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x1` | `[sp-688]` | `?` |
| `bl` at 0xc230c | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x2` | `?` | `?` |
| `mov` at 0xc2340 | `(unknown callee) (unknown target)` | `?` | `[sp-1552]` | `[sp-1556]` | `?` |
| `bl` at 0xc2368 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc2380 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-704]` | `sp-672` |
| `bl` at 0xc23bc | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc23f4 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x1` | `[sp-688]` | `?` |
| `bl` at 0xc2434 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x2` | `?` | `?` |
| `mov` at 0xc2468 | `(unknown callee) (unknown target)` | `?` | `[sp-1560]` | `[sp-1564]` | `?` |
| `bl` at 0xc2490 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc24a8 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-704]` | `sp-672` |
| `mov` at 0xc24c0 | `(unknown callee) (unknown target)` | `0x6` | `[sp-688]` | `[sp-688]` | `[0x31a5fc]` |
| `bl` at 0xc24e8 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc2500 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-704]` | `sp-672` |
| `bl` at 0xc250c | `(unknown callee) 0x55654` | `0x6` | `[sp-688]` | `[sp-688]` | `0x2006` |
| `bl` at 0xc2534 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc254c | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-704]` | `sp-672` |
| `bl` at 0xc2568 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc257c | `(unknown callee) 0x6e140` | `?` | `?` | `?` | `?` |
| `bl` at 0xc259c | `(unknown callee) 0x120340` | `[sp-752]` | `sp-756` | `ret(0x6e140)` | `sp-756` |
| `bl` at 0xc25d8 | `(unknown callee) 0x6e1b0` | `[sp-752]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0xc25e4 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0xc262c | `(unknown callee) 0x203f4` | `[sp-1012]` | `[sp-1004]` | `[sp-1008]` | `?` |
| `bl` at 0xc2644 | `(unknown callee) 0x1208b8` | `[sp-752]` | `?` | `?` | `?` |
| `bl` at 0xc2664 | `(unknown callee) 0x205bc` | `sp-1000` | `0x16da88` | `[sp-748]` | `ret(0x203f4)` |
| `bl` at 0xc2674 | `(unknown callee) 0x2031c` | `sp-1000` | `?` | `?` | `sp-1000` |
| `bl` at 0xc26a4 | `(unknown callee) 0x120398` | `[sp-752]` | `?` | `?` | `?` |
| `bl` at 0xc26c0 | `(unknown callee) 0x203c4` | `sp-1000` | `0x16da94` | `?` | `sp-1000` |
| `bl` at 0xc26e0 | `(unknown callee) 0x120944` | `[[sp-756]+28]` | `?` | `?` | `[sp-756]` |
| `bl` at 0xc26f4 | `(unknown callee) 0x205bc` | `[sp-764]` | `0x16daa8` | `ret(0x120944)` | `ret(0x120944)` |
| `bl` at 0xc270c | `(unknown callee) 0x1208b8` | `[sp-752]` | `?` | `?` | `?` |
| `bl` at 0xc2718 | `(unknown callee) 0xc4b5c` | `ret(0x1208b8)` | `?` | `?` | `ret(0x1208b8)` |
| `bl` at 0xc27a4 | `(unknown callee) 0x205bc` | `[sp-764]` | `0x16dacc` | `[sp-1568]` | `[[sp-756]+76]` |
| `bl` at 0xc27cc | `(unknown callee) 0x120398` | `[sp-752]` | `?` | `?` | `?` |
| `bl` at 0xc27e8 | `(unknown callee) 0x203c4` | `sp-1000` | `0x16dae4` | `?` | `sp-1000` |
| `bl` at 0xc2808 | `(unknown callee) 0x120944` | `[[sp-756]+4]` | `?` | `?` | `[sp-756]` |
| `bl` at 0xc281c | `(unknown callee) 0x205bc` | `[sp-764]` | `0x16daf8` | `ret(0x120944)` | `ret(0x120944)` |
| `bl` at 0xc283c | `(unknown callee) 0x120944` | `[[sp-756]+32]` | `?` | `?` | `[sp-756]` |
| `bl` at 0xc2850 | `(unknown callee) 0x205bc` | `[sp-764]` | `0x16db08` | `ret(0x120944)` | `ret(0x120944)` |
| `bl` at 0xc2870 | `(unknown callee) 0x120944` | `[[sp-756]+52]` | `?` | `?` | `[sp-756]` |
| `bl` at 0xc2884 | `(unknown callee) 0x205bc` | `[sp-764]` | `0x16db2c` | `ret(0x120944)` | `ret(0x120944)` |
| `bl` at 0xc28a4 | `(unknown callee) 0x120944` | `[[sp-756]+72]` | `?` | `?` | `[sp-756]` |
| `bl` at 0xc28b8 | `(unknown callee) 0x205bc` | `[sp-764]` | `0x16db50` | `ret(0x120944)` | `ret(0x120944)` |
| `bl` at 0xc28c8 | `(unknown callee) 0x205bc` | `[sp-764]` | `0x16db78` | `?` | `[[sp-756]+72]` |
| `bl` at 0xc28d0 | `(unknown callee) 0x2031c` | `[sp-764]` | `?` | `?` | `?` |
| `bl` at 0xc2928 | `(unknown callee) 0x20070` | `?` | `?` | `[[sp-756]+76]` | `?` |
| `bl` at 0xc29dc | `(unknown callee) 0x205bc` | `?` | `0x16db84` | `?` | `?` |
| `bl` at 0xc2a40 | `(unknown callee) 0x205bc` | `[sp-764]` | `0x16db8c` | `[[[sp-756]+36]+3]:1` | `[[sp-756]+40]` |
| `bl` at 0xc2a54 | `(unknown callee) 0x203c4` | `sp-1000` | `0x16dba0` | `?` | `sp-1000` |
| `bl` at 0xc2a74 | `(unknown callee) 0x2031c` | `sp-1000` | `?` | `?` | `sp-1000` |
| `bl` at 0xc2aa0 | `(unknown callee) 0x95b5c` | `sp-672` | `sp-668` | `sp-1000` | `ret(0x2031c)` |
| `bl` at 0xc2af0 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc2b04 | `(unknown callee) 0x6e140` | `?` | `?` | `?` | `?` |
| `bl` at 0xc2b24 | `(unknown callee) 0x120340` | `[sp-1008]` | `sp-1012` | `ret(0x6e140)` | `sp-1012` |
| `bl` at 0xc2b58 | `(unknown callee) 0x1208b8` | `[sp-1008]` | `?` | `?` | `?` |
| `bl` at 0xc2b7c | `(unknown callee) 0x205bc` | `sp-1140` | `0x16dba8` | `[sp-748]` | `ret(0x1208b8)` |
| `bl` at 0xc2b9c | `(unknown callee) 0x2031c` | `sp-1140` | `?` | `?` | `sp-1140` |
| `bl` at 0xc2bd4 | `(unknown callee) 0x120398` | `[sp-1008]` | `?` | `?` | `?` |
| `bl` at 0xc2bf8 | `(unknown callee) 0x203c4` | `sp-1140` | `0x16dbb0` | `?` | `sp-1140` |
| `bl` at 0xc2c18 | `(unknown callee) 0x120944` | `[[sp-1012]+4]` | `?` | `?` | `[sp-1012]` |
| `bl` at 0xc2c2c | `(unknown callee) 0x205bc` | `[sp-732]` | `0x16dbc4` | `ret(0x120944)` | `ret(0x120944)` |
| `bl` at 0xc2c3c | `(unknown callee) 0x205bc` | `[sp-732]` | `0x16dbd4` | `?` | `[[sp-1012]+4]` |
| `bl` at 0xc2c44 | `(unknown callee) 0x2031c` | `[sp-732]` | `?` | `?` | `?` |
| `bl` at 0xc2c90 | `(unknown callee) 0x205bc` | `?` | `0x16dbe0` | `?` | `?` |
| `bl` at 0xc2ce4 | `(unknown callee) 0x205bc` | `?` | `0x16dbe0` | `?` | `?` |
| `bl` at 0xc2d4c | `(unknown callee) 0x20070` | `?` | `?` | `[[sp-1012]+12]` | `?` |
| `bl` at 0xc2e08 | `(unknown callee) 0x20070` | `[[[sp-1012]+12]+91]:1` | `?` | `0x5b` | `[[[sp-1012]+12]+91]:1` |
| `bl` at 0xc2e30 | `(unknown callee) 0x20070` | `[[[sp-1012]+12]+92]:1` | `?` | `0x5c` | `[[[sp-1012]+12]+92]:1` |
| `bl` at 0xc2e74 | `(unknown callee) 0x205bc` | `?` | `0x16dbe8` | `[[[sp-1012]+12]+91]:1` | `[[[sp-1012]+12]+92]:1` |
| `bl` at 0xc2f3c | `(unknown callee) 0x205bc` | `?` | `0x16dbf4` | `?` | `?` |
| `bl` at 0xc2f44 | `(unknown callee) 0x2031c` | `[sp-732]` | `?` | `?` | `?` |
| `bl` at 0xc2f6c | `(unknown callee) 0x4ed88` | `[sp-1012]` | `0x0` | `0x0` | `([sp-732] + ret(0x2031c))` |
| `bl` at 0xc2f88 | `(unknown callee) 0x203c4` | `sp-1140` | `0x16dba0` | `?` | `sp-1140` |
| `bl` at 0xc2fb0 | `(unknown callee) 0x2031c` | `sp-1140` | `?` | `?` | `sp-1140` |
| `bl` at 0xc2fe4 | `(unknown callee) 0x95b5c` | `sp-672` | `sp-668` | `sp-1140` | `ret(0x2031c)` |
| `bl` at 0xc301c | `(unknown callee) 0xa4014` | `sp-752` | `[sp-688]` | `[sp-688]` | `sp-752` |
| `bl` at 0xc3048 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc3064 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc30a4 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `?` | `?` |
| `bl` at 0xc30bc | `(unknown callee) 0x20514` | `[sp-752]` | `?` | `?` | `[sp-1004]` |
| `bl` at 0xc30c8 | `(unknown callee) 0x293f8` | `0x16dc00` | `[sp-688]` | `[sp-688]` | `0x4002` |
| `bl` at 0xc30d0 | `(unknown callee) 0x201a8` | `0x16dc1c` | `?` | `?` | `?` |
| `bl` at 0xc30d8 | `(unknown callee) 0x1fff8` | `0x2dc6c0` | `?` | `?` | `?` |
| `bl` at 0xc30e8 | `(unknown callee) 0x21058` | `0x1` | `[0x31e428]` | `?` | `0x31e418` |
| `bl` at 0xc311c | `(unknown callee) 0x20550` | `sp-1339` | `0x0` | `0xc7` | `sp-1339` |
| `bl` at 0xc314c | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x1` | `?` | `?` |
| `bl` at 0xc3188 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x2` | `?` | `?` |
| `bl` at 0xc31e8 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x3` | `[sp-1584]` | `?` |
| `bl` at 0xc3220 | `(unknown callee) 0x89e18` | `?` | `[sp-1588]` | `sp-1340` | `0x0` |
| `bl` at 0xc324c | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc3274 | `(unknown callee) 0x2031c` | `sp-1340` | `?` | `?` | `sp-1340` |
| `bl` at 0xc32a8 | `(unknown callee) 0x95b5c` | `sp-672` | `sp-668` | `sp-1340` | `ret(0x2031c)` |
| `bl` at 0xc32c4 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc32dc | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-1004]` | `sp-672` |
| `bl` at 0xc3304 | `(unknown callee) 0x2031c` | `sp-1340` | `sp-24` | `?` | `sp-1340` |
| `bl` at 0xc3338 | `(unknown callee) 0x95b5c` | `sp-672` | `sp-668` | `sp-1340` | `ret(0x2031c)` |
| `bl` at 0xc336c | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x3` | `[sp-1584]` | `?` |
| `bl` at 0xc33a8 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x4` | `[sp-1592]` | `?` |
| `bl` at 0xc33e4 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x5` | `?` | `?` |
| `bl` at 0xc3428 | `(unknown callee) 0x8a1e4` | `?` | `[sp-740]` | `[sp-736]` | `[sp-1600]` |
| `bl` at 0xc3454 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc347c | `(unknown callee) 0x2031c` | `sp-1340` | `sp-24` | `?` | `sp-1340` |
| `bl` at 0xc34b0 | `(unknown callee) 0x95b5c` | `sp-672` | `sp-668` | `sp-1340` | `ret(0x2031c)` |
| `bl` at 0xc34cc | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc34e4 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-1004]` | `sp-672` |
| `bl` at 0xc350c | `(unknown callee) 0x2031c` | `sp-1340` | `?` | `sp-24` | `sp-1340` |
| `bl` at 0xc3540 | `(unknown callee) 0x95b5c` | `sp-672` | `sp-668` | `sp-1340` | `ret(0x2031c)` |
| `bl` at 0xc354c | `(unknown callee) 0x28230` | `0x7e` | `[sp-688]` | `[sp-688]` | `[sp-688]` |
| `bl` at 0xc356c | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc3574 | `(unknown callee) 0x28230` | `0x50` | `?` | `?` | `?` |
| `bl` at 0xc3598 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `0xffff` | `sp-672` |
| `bl` at 0xc35a4 | `(unknown callee) 0x28230` | `0x80` | `?` | `?` | `ret(0x28230)` |
| `bl` at 0xc35c8 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `0xffff` | `sp-672` |
| `bl` at 0xc35e4 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-704]` | `sp-672` |
| `bl` at 0xc3600 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc3608 | `(unknown callee) 0x28230` | `0xcc` | `?` | `?` | `?` |
| `bl` at 0xc3628 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc3654 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc365c | `(unknown callee) 0x12a4e8` | `[sp-688]` | `[sp-688]` | `[sp-688]` | `[[sp-28]+32]:1` |
| `bl` at 0xc3698 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x1` | `?` | `?` |
| `bl` at 0xc36c4 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc36f8 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x1` | `?` | `?` |
| `bl` at 0xc3704 | `(unknown callee) 0x12a6b8` | `[sp-1612]` | `?` | `?` | `?` |
| `bl` at 0xc3730 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc3748 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-676]` | `sp-672` |
| `bl` at 0xc3764 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc3790 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc3798 | `(unknown callee) 0x12a4e8` | `[sp-688]` | `[sp-688]` | `[sp-688]` | `[[sp-28]+32]:1` |
| `bl` at 0xc37d4 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x1` | `?` | `?` |
| `bl` at 0xc3800 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc3834 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x1` | `?` | `?` |
| `bl` at 0xc384c | `(unknown callee) 0x12a7a4` | `[sp-1620]` | `sp-757` | `?` | `sp-757` |
| `bl` at 0xc3878 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc3890 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-676]` | `sp-672` |
| `bl` at 0xc38ac | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc38c4 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc3908 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x1` | `[sp-688]` | `?` |
| `bl` at 0xc3944 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc3960 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc39dc | `(unknown callee) 0xfde7c` | `[sp-28]` | `?` | `[sp-28]` | `?` |
| `bl` at 0xc3a00 | `(unknown callee) 0x125b8c` | `0x0` | `[sp-1628]` | `sp-757` | `sp-757` |
| `bl` at 0xc3a2c | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc3a44 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-676]` | `sp-672` |
| `bl` at 0xc3a5c | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-1008]` | `sp-672` |
| `bl` at 0xc3a80 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `?` | `?` |
| `bl` at 0xc3ab8 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc3aec | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x1` | `[sp-688]` | `?` |
| `bl` at 0xc3b34 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc3b50 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc3bf0 | `(unknown callee) 0xfde7c` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0xc3c84 | `(unknown callee) 0xfde7c` | `[sp-28]` | `?` | `[sp-1008]` | `?` |
| `bl` at 0xc3d18 | `(unknown callee) 0xfde7c` | `[sp-28]` | `?` | `[sp-1008]` | `?` |
| `bl` at 0xc3d38 | `(unknown callee) 0x125a58` | `0x0` | `[sp-1636]` | `[sp-1640]` | `[sp-1644]` |
| `bl` at 0xc3d64 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc3d7c | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-676]` | `sp-672` |
| `bl` at 0xc3d94 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-1008]` | `sp-672` |
| `bl` at 0xc3dd0 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc3e98 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x1` | `[sp-688]` | `?` |
| `bl` at 0xc3fa4 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x1` | `[sp-1648]` | `?` |
| `bl` at 0xc3fd8 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x2` | `?` | `?` |
| `bl` at 0xc3ff0 | `(unknown callee) 0x4ed88` | `0x0` | `[sp-1652]` | `[sp-1656]` | `0x0` |
| `bl` at 0xc401c | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc4034 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-1008]` | `sp-672` |
| `bl` at 0xc4050 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc4068 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-1008]` | `sp-672` |
| `bl` at 0xc4084 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `mov` at 0xc40a0 | `(unknown callee) (unknown target)` | `[[sp-28]+28]` | `[sp-688]` | `[sp-28]` | `[0x1b0eb4]` |
| `bl` at 0xc40c0 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `ret(unknown)` | `sp-672` |
| `bl` at 0xc40dc | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `mov` at 0xc40ec | `(unknown callee) (unknown target)` | `ret(0x958d4)` | `?` | `?` | `[0x1b0eb0]` |
| `bl` at 0xc4108 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `mov` at 0xc4124 | `(unknown callee) (unknown target)` | `[sp-672]` | `sp-668` | `sp-668` | `[0x1b0ec4]` |
| `bl` at 0xc4148 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc4150 | `(unknown callee) 0x6aaa0` | `0x0` | `?` | `?` | `?` |
| `bl` at 0xc4170 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `ret(0x6aaa0)` | `sp-672` |
| `bl` at 0xc419c | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc4200 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x2` | `[sp-688]` | `?` |
| `bl` at 0xc4268 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x3` | `?` | `?` |
| `bl` at 0xc42d0 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x4` | `?` | `?` |
| `bl` at 0xc4358 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x5` | `?` | `?` |
| `bl` at 0xc43f4 | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x1` | `?` | `?` |
| `bl` at 0xc448c | `(unknown callee) 0xfde7c` | `[sp-28]` | `0x6` | `?` | `?` |
| `bl` at 0xc44fc | `(unknown callee) 0x205bc` | `sp-1340` | `0x16dcf8` | `[sp-1660]` | `[sp-1664]` |
| `bl` at 0xc4510 | `(unknown callee) 0x201a8` | `sp-1340` | `?` | `?` | `sp-1340` |
| `bl` at 0xc4534 | `(unknown callee) 0x29908` | `0x16dd4c` | `sp-1340` | `?` | `sp-1340` |
| `bl` at 0xc454c | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc4568 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc4570 | `(unknown callee) 0x202c8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0xc4584 | `(unknown callee) 0x204f0` | `sp-1008` | `?` | `?` | `sp-1008` |
| `bl` at 0xc45b0 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[ret(0x204f0)+20]+1900` | `[ret(0x204f0)+20]+1900` |
| `bl` at 0xc45d0 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[[sp-1004]+16]+1` | `[[sp-1004]+16]+1` |
| `bl` at 0xc45ec | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[[sp-1004]+12]` | `sp-672` |
| `bl` at 0xc4608 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[[sp-1004]+8]` | `sp-672` |
| `bl` at 0xc4624 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[[sp-1004]+4]` | `sp-672` |
| `bl` at 0xc4640 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[[sp-1004]]` | `sp-672` |
| `bl` at 0xc466c | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc4794 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc479c | `(unknown callee) 0x6dc0c` | `?` | `?` | `?` | `?` |
| `bl` at 0xc4820 | `(unknown callee) 0x3deec` | `?` | `sp-24` | `[sp-1688]` | `?` |
| `bl` at 0xc4878 | `(unknown callee) 0x6dc0c` | `?` | `?` | `[sp-1688]` | `[sp-1688]` |
| `bl` at 0xc488c | `(unknown callee) 0x243d4` | `0x16dd70` | `?` | `?` | `?` |
| `bl` at 0xc48a0 | `(unknown callee) 0x6e140` | `?` | `?` | `?` | `?` |
| `bl` at 0xc48bc | `(unknown callee) 0x6e190` | `ret(0x6e140)` | `?` | `sp-24` | `?` |
| `bl` at 0xc49d4 | `(unknown callee) 0x75a8c` | `[sp-1692]` | `0x0` | `[sp-718]:2` | `[sp-718]:2` |
| `bl` at 0xc4a00 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc4a18 | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-676]` | `sp-672` |
| `bl` at 0xc4a34 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `0x0` | `sp-672` |
| `bl` at 0xc4a4c | `(unknown callee) 0x95934` | `sp-672` | `sp-668` | `[sp-676]` | `sp-672` |
| `mov` at 0xc4a84 | `(unknown callee) (unknown target)` | `[sp-28]` | `sp-672` | `sp-668` | `[0x1b0ce8]` |
| `bl` at 0xc4aa0 | `(unknown callee) 0x958d4` | `sp-672` | `sp-668` | `?` | `sp-672` |
| `bl` at 0xc4ac8 | `(unknown callee) 0xfd0dc` | `0x31b67c` | `0x0` | `[sp-28]` | `[sp-672]` |
| `bl` at 0xc4ae4 | `(unknown callee) 0x219a4` | `ret(0xfd0dc)` | `?` | `?` | `ret(0xfd0dc)` |
| `bl` at 0xc4b08 | `(unknown callee) 0x22070` | `0x6` | `0x16d8e8` | `0x811` | `0x0` |
| `bl` at 0xc4b10 | `(unknown callee) 0x2302c` | `0x31b628` | `?` | `?` | `?` |
| `bl` at 0xc4b48 | `(unknown callee) 0x22070` | `0x9` | `0x16d8e8` | `0x81a` | `0x0` |
| `bl` at 0xc4b50 | `(unknown callee) 0xfb794` | `[sp-28]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xbeefc | `ret(0x22ca0) == 0x0` | `ret(0x22ca0) != 0x0` |
| `bne` at 0xbef30 | `[[sp-28]+32]:1 != 0x0` | `[[sp-28]+32]:1 == 0x0` |
| `bne` at 0xbef6c | `? != 0x0` | `? == 0x0` |
| `ble` at 0xbefac | `ret(0x6e140) <= 0x1 (signed, 32-bit)` | `ret(0x6e140) > 0x1 (signed, 32-bit)` |
| `blt` at 0xbeffc | `[sp-684] < 0x0 (signed, 32-bit)` | `[sp-684] >= 0x0 (signed, 32-bit)` |
| `blt` at 0xbf04c | `[sp-684] < 0x0 (signed, 32-bit)` | `[sp-684] >= 0x0 (signed, 32-bit)` |
| `beq` at 0xbf0c8 | `[0x1a6468] == 0x0` | `[0x1a6468] != 0x0` |
| `beq` at 0xbf138 | `[sp-688] == 0x1009` | `[sp-688] != 0x1009` |
| `bhi` at 0xbf148 | `[sp-688] > 0x1009 (unsigned, 32-bit)` | `[sp-688] <= 0x1009 (unsigned, 32-bit)` |
| `beq` at 0xbf154 | `[sp-688] == 0xd` | `[sp-688] != 0xd` |
| `bhi` at 0xbf160 | `[sp-688] > 0xd (unsigned, 32-bit)` | `[sp-688] <= 0xd (unsigned, 32-bit)` |
| `beq` at 0xbf16c | `[sp-688] == 0x6` | `[sp-688] != 0x6` |
| `bhi` at 0xbf178 | `[sp-688] > 0x6 (unsigned, 32-bit)` | `[sp-688] <= 0x6 (unsigned, 32-bit)` |
| `bhi` at 0xbf184 | `[sp-688] > 0x3 (unsigned, 32-bit)` | `[sp-688] <= 0x3 (unsigned, 32-bit)` |
| `bhs` at 0xbf190 | `[sp-688] >= 0x2 (unsigned, 32-bit)` | `[sp-688] < 0x2 (unsigned, 32-bit)` |
| `beq` at 0xbf19c | `[sp-688] == 0x1` | `[sp-688] != 0x1` |
| `beq` at 0xbf1ac | `[sp-688] == 0x4` | `[sp-688] != 0x4` |
| `beq` at 0xbf1b8 | `[sp-688] == 0x5` | `[sp-688] != 0x5` |
| `beq` at 0xbf1c8 | `[sp-688] == 0x9` | `[sp-688] != 0x9` |
| `bhi` at 0xbf1d4 | `[sp-688] > 0x9 (unsigned, 32-bit)` | `[sp-688] <= 0x9 (unsigned, 32-bit)` |
| `beq` at 0xbf1e0 | `[sp-688] == 0x7` | `[sp-688] != 0x7` |
| `beq` at 0xbf1ec | `[sp-688] == 0x8` | `[sp-688] != 0x8` |
| `beq` at 0xbf1fc | `[sp-688] == 0xb` | `[sp-688] != 0xb` |
| `bhi` at 0xbf208 | `[sp-688] > 0xb (unsigned, 32-bit)` | `[sp-688] <= 0xb (unsigned, 32-bit)` |
| `beq` at 0xbf218 | `[sp-688] == 0x15` | `[sp-688] != 0x15` |
| `bhi` at 0xbf224 | `[sp-688] > 0x15 (unsigned, 32-bit)` | `[sp-688] <= 0x15 (unsigned, 32-bit)` |
| `beq` at 0xbf230 | `[sp-688] == 0x11` | `[sp-688] != 0x11` |
| `bhi` at 0xbf23c | `[sp-688] > 0x11 (unsigned, 32-bit)` | `[sp-688] <= 0x11 (unsigned, 32-bit)` |
| `beq` at 0xbf248 | `[sp-688] == 0xe` | `[sp-688] != 0xe` |
| `beq` at 0xbf254 | `[sp-688] == 0x10` | `[sp-688] != 0x10` |
| `beq` at 0xbf264 | `[sp-688] == 0x12` | `[sp-688] != 0x12` |
| `beq` at 0xbf270 | `[sp-688] == 0x13` | `[sp-688] != 0x13` |
| `bhi` at 0xbf284 | `[sp-688] > 0x1005 (unsigned, 32-bit)` | `[sp-688] <= 0x1005 (unsigned, 32-bit)` |
| `bhs` at 0xbf294 | `[sp-688] >= 0x1004 (unsigned, 32-bit)` | `[sp-688] < 0x1004 (unsigned, 32-bit)` |
| `beq` at 0xbf2a4 | `[sp-688] == 0x1001` | `[sp-688] != 0x1001` |
| `beq` at 0xbf2b8 | `[sp-688] == 0x1007` | `[sp-688] != 0x1007` |
| `bhi` at 0xbf2c8 | `[sp-688] > 0x1007 (unsigned, 32-bit)` | `[sp-688] <= 0x1007 (unsigned, 32-bit)` |
| `beq` at 0xbf2dc | `[sp-688] == 0x200c` | `[sp-688] != 0x200c` |
| `bhi` at 0xbf2ec | `[sp-688] > 0x200c (unsigned, 32-bit)` | `[sp-688] <= 0x200c (unsigned, 32-bit)` |
| `beq` at 0xbf2fc | `[sp-688] == 0x2003` | `[sp-688] != 0x2003` |
| `bhi` at 0xbf30c | `[sp-688] > 0x2003 (unsigned, 32-bit)` | `[sp-688] <= 0x2003 (unsigned, 32-bit)` |
| `beq` at 0xbf31c | `[sp-688] == 0x100c` | `[sp-688] != 0x100c` |
| `bhi` at 0xbf32c | `[sp-688] > 0x100c (unsigned, 32-bit)` | `[sp-688] <= 0x100c (unsigned, 32-bit)` |
| `beq` at 0xbf33c | `[sp-688] == 0x100a` | `[sp-688] != 0x100a` |
| `beq` at 0xbf34c | `[sp-688] == 0x100b` | `[sp-688] != 0x100b` |
| `beq` at 0xbf360 | `[sp-688] == 0x2001` | `[sp-688] != 0x2001` |
| `beq` at 0xbf370 | `[sp-688] == 0x2002` | `[sp-688] != 0x2002` |
| `beq` at 0xbf384 | `[sp-688] == 0x2006` | `[sp-688] != 0x2006` |
| `bhi` at 0xbf394 | `[sp-688] > 0x2006 (unsigned, 32-bit)` | `[sp-688] <= 0x2006 (unsigned, 32-bit)` |
| `beq` at 0xbf3a4 | `[sp-688] == 0x2004` | `[sp-688] != 0x2004` |
| `beq` at 0xbf3b4 | `[sp-688] == 0x2005` | `[sp-688] != 0x2005` |
| `beq` at 0xbf3c8 | `[sp-688] == 0x200a` | `[sp-688] != 0x200a` |
| `bhi` at 0xbf3d8 | `[sp-688] > 0x200a (unsigned, 32-bit)` | `[sp-688] <= 0x200a (unsigned, 32-bit)` |
| `beq` at 0xbf3e8 | `[sp-688] == 0x2009` | `[sp-688] != 0x2009` |
| `beq` at 0xbf3fc | `[sp-688] == 0x3004` | `[sp-688] != 0x3004` |
| `bhi` at 0xbf40c | `[sp-688] > 0x3004 (unsigned, 32-bit)` | `[sp-688] <= 0x3004 (unsigned, 32-bit)` |
| `beq` at 0xbf41c | `[sp-688] == 0x200f` | `[sp-688] != 0x200f` |
| `bhi` at 0xbf42c | `[sp-688] > 0x200f (unsigned, 32-bit)` | `[sp-688] <= 0x200f (unsigned, 32-bit)` |
| `beq` at 0xbf43c | `[sp-688] == 0x200d` | `[sp-688] != 0x200d` |
| `beq` at 0xbf44c | `[sp-688] == 0x200e` | `[sp-688] != 0x200e` |
| `beq` at 0xbf460 | `[sp-688] == 0x3002` | `[sp-688] != 0x3002` |
| `bhi` at 0xbf470 | `[sp-688] > 0x3002 (unsigned, 32-bit)` | `[sp-688] <= 0x3002 (unsigned, 32-bit)` |
| `beq` at 0xbf480 | `[sp-688] == 0x3001` | `[sp-688] != 0x3001` |
| `beq` at 0xbf494 | `[sp-688] == 0x4002` | `[sp-688] != 0x4002` |
| `bhi` at 0xbf4a4 | `[sp-688] > 0x4002 (unsigned, 32-bit)` | `[sp-688] <= 0x4002 (unsigned, 32-bit)` |
| `beq` at 0xbf4b0 | `[sp-688] == 0x4000` | `[sp-688] != 0x4000` |
| `beq` at 0xbf4c0 | `[sp-688] == 0x4001` | `[sp-688] != 0x4001` |
| `beq` at 0xbf4d4 | `[sp-688] == 0x4004` | `[sp-688] != 0x4004` |
| `blo` at 0xbf4e4 | `[sp-688] < 0x4004 (unsigned, 32-bit)` | `[sp-688] >= 0x4004 (unsigned, 32-bit)` |
| `beq` at 0xbf4f4 | `[sp-688] == 0x4005` | `[sp-688] != 0x4005` |
| `bne` at 0xbf59c | `[sp-688] != 0x3` | `[sp-688] == 0x3` |
| `beq` at 0xbf5c8 | `[sp-680] == 0x3d` | `[sp-680] != 0x3d` |
| `bne` at 0xbf5d8 | `[sp-712] != 0x0` | `[sp-712] == 0x0` |
| `bne` at 0xbf668 | `[ret(0xa3c7c)+16] != 0x3d` | `[ret(0xa3c7c)+16] == 0x3d` |
| `bne` at 0xbf674 | `[sp-712] != 0x0` | `[sp-712] == 0x0` |
| `bne` at 0xbf704 | `[ret(0xa3c7c)+16] != 0x3d` | `[ret(0xa3c7c)+16] == 0x3d` |
| `bne` at 0xbf710 | `[sp-712] != 0x0` | `[sp-712] == 0x0` |
| `bne` at 0xbf7a0 | `[ret(0xa3c7c)+16] != 0x3d` | `[ret(0xa3c7c)+16] == 0x3d` |
| `bne` at 0xbf7ac | `[sp-712] != 0x0` | `[sp-712] == 0x0` |
| `bne` at 0xbf83c | `[ret(0xa3c7c)+16] != 0x3d` | `[ret(0xa3c7c)+16] == 0x3d` |
| `bne` at 0xbf848 | `[sp-712] != 0x0` | `[sp-712] == 0x0` |
| `blt` at 0xbf990 | `[sp-680] < 0x2c (signed, 32-bit)` | `[sp-680] >= 0x2c (signed, 32-bit)` |
| `ble` at 0xbf99c | `[sp-680] <= 0x2e (signed, 32-bit)` | `[sp-680] > 0x2e (signed, 32-bit)` |
| `beq` at 0xbf9a8 | `[sp-680] == 0x40` | `[sp-680] != 0x40` |
| `beq` at 0xbf9b4 | `[sp-680] == 0x7d` | `[sp-680] != 0x7d` |
| `bne` at 0xbf9f8 | `Z == 0` | `not (Z == 0)` |
| `bhi` at 0xbfb28 | `[[sp-28]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-28]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0xbfb58 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xbfbb8 | `ret(0x6e140) == 0x1` | `ret(0x6e140) != 0x1` |
| `bne` at 0xbfbc8 | `ret(0x3e38c) != 0x0` | `ret(0x3e38c) == 0x0` |
| `beq` at 0xbfc10 | `ret(0x6e140) == 0x1` | `ret(0x6e140) != 0x1` |
| `bne` at 0xbfc20 | `ret(0x3e38c) != 0x0` | `ret(0x3e38c) == 0x0` |
| `bne` at 0xbfc60 | `ret(0x120398) != 0x0` | `ret(0x120398) == 0x0` |
| `blt` at 0xbfc94 | `[sp-728] < [sp-732] (signed, 32-bit)` | `[sp-728] >= [sp-732] (signed, 32-bit)` |
| `beq` at 0xbfd2c | `[sp-1404] == 0x1` | `[sp-1404] != 0x1` |
| `beq` at 0xbfd38 | `[sp-1404] == 0x2` | `[sp-1404] != 0x2` |
| `bhi` at 0xbffe4 | `[[sp-28]+32]:1 > 0x4 (unsigned, 32-bit)` | `[[sp-28]+32]:1 <= 0x4 (unsigned, 32-bit)` |
| `bne` at 0xc0014 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc0078 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc00b4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc00f0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xc0138 | `ret(0x6e140) == 0x1` | `ret(0x6e140) != 0x1` |
| `bne` at 0xc0148 | `ret(0x3e38c) != 0x0` | `ret(0x3e38c) == 0x0` |
| `beq` at 0xc0190 | `ret(0x6e140) == 0x1` | `ret(0x6e140) != 0x1` |
| `bne` at 0xc01a0 | `ret(0x3e38c) != 0x0` | `ret(0x3e38c) == 0x0` |
| `bne` at 0xc01e0 | `ret(0x120398) != 0x0` | `ret(0x120398) == 0x0` |
| `blt` at 0xc020c | `[sp-728] < [sp-756] (signed, 32-bit)` | `[sp-728] >= [sp-756] (signed, 32-bit)` |
| `ldrls` at 0xc0234 | `[sp-736] <= 0x4 (unsigned, 32-bit)` | `[sp-736] > 0x4 (unsigned, 32-bit)` |
| `beq` at 0xc0678 | `[sp-748] == 0x0` | `[sp-748] != 0x0` |
| `blo` at 0xc0688 | `[sp-740] < [sp-752] (unsigned, 32-bit)` | `[sp-740] >= [sp-752] (unsigned, 32-bit)` |
| `bls` at 0xc06bc | `([sp-740] + [sp-744]) <= [sp-752] (unsigned, 32-bit)` | `([sp-740] + [sp-744]) > [sp-752] (unsigned, 32-bit)` |
| `beq` at 0xc072c | `[sp-744] == 0x0` | `[sp-744] != 0x0` |
| `beq` at 0xc0768 | `[0x318478] == 0x0` | `[0x318478] != 0x0` |
| `bne` at 0xc07a8 | `ret(0x2031c) != 0x0` | `ret(0x2031c) == 0x0` |
| `ldrls` at 0xc08c0 | `[sp-680]-36 <= 0x59 (unsigned, 32-bit)` | `[sp-680]-36 > 0x59 (unsigned, 32-bit)` |
| `beq` at 0xc0b9c | `[sp-680] == 0x2c` | `[sp-680] != 0x2c` |
| `beq` at 0xc0ba8 | `[sp-680] == 0x7d` | `[sp-680] != 0x7d` |
| `beq` at 0xc0bb4 | `[sp-680] == 0x2e` | `[sp-680] != 0x2e` |
| `beq` at 0xc0bc0 | `[sp-680] == 0x24` | `[sp-680] != 0x24` |
| `bgt` at 0xc0bd0 | `ret(0x6e140) > 0x1 (signed, 32-bit)` | `ret(0x6e140) <= 0x1 (signed, 32-bit)` |
| `beq` at 0xc0bec | `ret(0x3e38c) == 0x0` | `ret(0x3e38c) != 0x0` |
| `beq` at 0xc0c80 | `[sp-680] == 0x2c` | `[sp-680] != 0x2c` |
| `beq` at 0xc0c8c | `[sp-680] == 0x7d` | `[sp-680] != 0x7d` |
| `beq` at 0xc0c98 | `[sp-680] == 0x2e` | `[sp-680] != 0x2e` |
| `beq` at 0xc0ca4 | `[sp-680] == 0x24` | `[sp-680] != 0x24` |
| `bgt` at 0xc0cb4 | `ret(0x6e140) > 0x1 (signed, 32-bit)` | `ret(0x6e140) <= 0x1 (signed, 32-bit)` |
| `beq` at 0xc0cd0 | `ret(0x3e38c) == 0x0` | `ret(0x3e38c) != 0x0` |
| `beq` at 0xc0d44 | `ret(0x3e2d0) == 0x0` | `ret(0x3e2d0) != 0x0` |
| `beq` at 0xc0de0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc0e74 | `[0x1a6468] == 0x0` | `[0x1a6468] != 0x0` |
| `beq` at 0xc0eb4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xc0f98 | `ret(0x7fbc8) != 0x0` | `ret(0x7fbc8) == 0x0` |
| `bne` at 0xc0fd0 | `[0x31841c] != 0x0` | `[0x31841c] == 0x0` |
| `bge` at 0xc1058 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xc10b4 | `ret(0x6e140) == 0x1` | `ret(0x6e140) != 0x1` |
| `bne` at 0xc10c4 | `ret(0x3e38c) != 0x0` | `ret(0x3e38c) == 0x0` |
| `beq` at 0xc1104 | `ret(0x7fcdc) == 0x0` | `ret(0x7fcdc) != 0x0` |
| `beq` at 0xc114c | `ret(0x6e140) == 0x1` | `ret(0x6e140) != 0x1` |
| `bne` at 0xc115c | `ret(0x3e38c) != 0x0` | `ret(0x3e38c) == 0x0` |
| `beq` at 0xc119c | `ret(0x7fe94) == 0x0` | `ret(0x7fe94) != 0x0` |
| `beq` at 0xc11f0 | `[sp-680] == 0x2c` | `[sp-680] != 0x2c` |
| `beq` at 0xc11fc | `[sp-680] == 0x7d` | `[sp-680] != 0x7d` |
| `beq` at 0xc1208 | `[sp-680] == 0x2e` | `[sp-680] != 0x2e` |
| `beq` at 0xc1214 | `[sp-680] == 0x24` | `[sp-680] != 0x24` |
| `bgt` at 0xc1224 | `ret(0x6e140) > 0x1 (signed, 32-bit)` | `ret(0x6e140) <= 0x1 (signed, 32-bit)` |
| `beq` at 0xc1290 | `[0x1a6468] == 0x0` | `[0x1a6468] != 0x0` |
| `beq` at 0xc12d0 | `[0x1a6468] == 0x0` | `[0x1a6468] != 0x0` |
| `beq` at 0xc13e8 | `[0x1a6468] == 0x0` | `[0x1a6468] != 0x0` |
| `bge` at 0xc1420 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xc1430 | `[0x1a6468] == 0x0` | `[0x1a6468] != 0x0` |
| `bgt` at 0xc1528 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0xc1654 | `ret(0x1347dc) == 0x0` | `ret(0x1347dc) != 0x0` |
| `bhi` at 0xc1680 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0xc16c4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc176c | `[sp-676] == 0x0` | `[sp-676] != 0x0` |
| `bhi` at 0xc17b8 | `[[sp-28]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-28]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0xc17e8 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0xc1840 | `[[sp-28]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-28]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `beq` at 0xc1870 | `? == 0x2` | `? != 0x2` |
| `bne` at 0xc18a0 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0xc18fc | `[[sp-28]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-28]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0xc192c | `? != 0x0` | `? == 0x0` |
| `bhi` at 0xc1998 | `[[sp-28]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-28]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0xc19c8 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0xc1a20 | `[[sp-28]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-28]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0xc1a6c | `? != 0x0` | `? == 0x0` |
| `beq` at 0xc1ac8 | `[0x1a6468] == 0x0` | `[0x1a6468] != 0x0` |
| `bne` at 0xc1adc | `? != 0x0` | `? == 0x0` |
| `bhi` at 0xc1b78 | `[[sp-28]+32]:1 > 0x2 (unsigned, 32-bit)` | `[[sp-28]+32]:1 <= 0x2 (unsigned, 32-bit)` |
| `bne` at 0xc1ba8 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xc1be4 | `[sp-1516] == 0x1` | `[sp-1516] != 0x1` |
| `blo` at 0xc1bf0 | `[sp-1516] < 0x1 (unsigned, 32-bit)` | `[sp-1516] >= 0x1 (unsigned, 32-bit)` |
| `beq` at 0xc1bfc | `[sp-1516] == 0x2` | `[sp-1516] != 0x2` |
| `bne` at 0xc1c14 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc1c58 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc1c98 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc1ccc | `? != 0x0` | `? == 0x0` |
| `beq` at 0xc1d5c | `[[sp-28]+32]:1 == 0x4` | `[[sp-28]+32]:1 != 0x4` |
| `bne` at 0xc1d8c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc1de8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc1e48 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xc1ec8 | `[0x1a6468] == 0x0` | `[0x1a6468] != 0x0` |
| `beq` at 0xc1f14 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xc1f38 | `ret(0xb7e54) != [sp-690]:1` | `ret(0xb7e54) == [sp-690]:1` |
| `beq` at 0xc1ff4 | `[[sp-28]+32]:1 == 0x3` | `[[sp-28]+32]:1 != 0x3` |
| `bne` at 0xc2040 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc2090 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xc2100 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bls` at 0xc2144 | `[[sp-28]+32]:1 <= 0x2 (unsigned, 32-bit)` | `[[sp-28]+32]:1 > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xc2154 | `[0x1b0cec] != 0x0` | `[0x1b0cec] == 0x0` |
| `bne` at 0xc2184 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc21c4 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xc2228 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bls` at 0xc226c | `[[sp-28]+32]:1 <= 0x2 (unsigned, 32-bit)` | `[[sp-28]+32]:1 > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xc227c | `[0x1b0cf0] != 0x0` | `[0x1b0cf0] == 0x0` |
| `bne` at 0xc22ac | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc22ec | `? != 0x0` | `? == 0x0` |
| `beq` at 0xc2350 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bls` at 0xc2394 | `[[sp-28]+32]:1 <= 0x2 (unsigned, 32-bit)` | `[[sp-28]+32]:1 > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xc23a4 | `[0x1b0cf4] != 0x0` | `[0x1b0cf4] == 0x0` |
| `bne` at 0xc23d4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc2414 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xc2478 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0xc24d0 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0xc251c | `ret(0x55654) == 0x0` | `ret(0x55654) != 0x0` |
| `ble` at 0xc258c | `ret(0x6e140) <= [sp-752] (signed, 32-bit)` | `ret(0x6e140) > [sp-752] (signed, 32-bit)` |
| `bge` at 0xc25bc | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xc2690 | `[[sp-756]] != 0x1` | `[[sp-756]] == 0x1` |
| `bne` at 0xc26b0 | `ret(0x120398) != 0x0` | `ret(0x120398) == 0x0` |
| `beq` at 0xc26d4 | `[[sp-756]+28] == 0x0` | `[[sp-756]+28] != 0x0` |
| `beq` at 0xc272c | `ret(0xc4b5c) == 0x0` | `ret(0xc4b5c) != 0x0` |
| `bne` at 0xc27b8 | `[[sp-756]] != 0x3` | `[[sp-756]] == 0x3` |
| `bne` at 0xc27d8 | `ret(0x120398) != 0x0` | `ret(0x120398) == 0x0` |
| `beq` at 0xc27fc | `[[sp-756]+4] == 0x0` | `[[sp-756]+4] != 0x0` |
| `beq` at 0xc2830 | `[[sp-756]+32] == 0x0` | `[[sp-756]+32] != 0x0` |
| `beq` at 0xc2864 | `[[sp-756]+52] == 0x0` | `[[sp-756]+52] != 0x0` |
| `beq` at 0xc2898 | `[[sp-756]+72] == 0x0` | `[[sp-756]+72] != 0x0` |
| `bgt` at 0xc28f8 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0xc2934 | `ret(0x20070) == 0x0` | `ret(0x20070) != 0x0` |
| `blt` at 0xc29b0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xc2a68 | `Z == 1` | `not (Z == 1)` |
| `ble` at 0xc2b14 | `ret(0x6e140) <= [sp-1008] (signed, 32-bit)` | `ret(0x6e140) > [sp-1008] (signed, 32-bit)` |
| `bge` at 0xc2b44 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xc2bc0 | `[[sp-1012]] != 0x2` | `[[sp-1012]] == 0x2` |
| `bne` at 0xc2be0 | `ret(0x120398) != 0x0` | `ret(0x120398) == 0x0` |
| `beq` at 0xc2c0c | `[[sp-1012]+4] == 0x0` | `[[sp-1012]+4] != 0x0` |
| `bgt` at 0xc2c6c | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bgt` at 0xc2cc0 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bgt` at 0xc2d30 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0xc2d58 | `ret(0x20070) == 0x0` | `ret(0x20070) != 0x0` |
| `beq` at 0xc2dec | `[[[sp-1012]+12]+64]:1 == 0x2c` | `[[[sp-1012]+12]+64]:1 != 0x2c` |
| `beq` at 0xc2e14 | `ret(0x20070) == 0x0` | `ret(0x20070) != 0x0` |
| `bne` at 0xc2e3c | `ret(0x20070) != 0x0` | `ret(0x20070) == 0x0` |
| `beq` at 0xc2f9c | `Z == 1` | `not (Z == 1)` |
| `bge` at 0xc3030 | `ret(0xa4014) >= 0x0 (signed, 32-bit)` | `ret(0xa4014) < 0x0 (signed, 32-bit)` |
| `bge` at 0xc307c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xc3130 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc316c | `? != 0x0` | `? == 0x0` |
| `beq` at 0xc31a8 | `[sp-1008] == 0x1` | `[sp-1008] != 0x1` |
| `beq` at 0xc31b4 | `[sp-1008] == 0x2` | `[sp-1008] != 0x2` |
| `bne` at 0xc31cc | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc3234 | `ret(0x89e18) != 0x0` | `ret(0x89e18) == 0x0` |
| `beq` at 0xc3260 | `[sp-1340]:1 == 0x0` | `[sp-1340]:1 != 0x0` |
| `beq` at 0xc32f0 | `[sp-1340]:1 == 0x0` | `[sp-1340]:1 != 0x0` |
| `bne` at 0xc3350 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc338c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc33c8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc343c | `ret(0x8a1e4) != 0x0` | `ret(0x8a1e4) == 0x0` |
| `beq` at 0xc3468 | `[sp-1340]:1 == 0x0` | `[sp-1340]:1 != 0x0` |
| `beq` at 0xc34f8 | `[sp-1340]:1 == 0x0` | `[sp-1340]:1 != 0x0` |
| `beq` at 0xc3580 | `ret(0x28230) == 0x0` | `ret(0x28230) != 0x0` |
| `beq` at 0xc35b0 | `ret(0x28230) == 0x0` | `ret(0x28230) != 0x0` |
| `bhi` at 0xc363c | `[[sp-28]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-28]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0xc3674 | `? != 0x0` | `? == 0x0` |
| `bhs` at 0xc3688 | `[[sp-28]+48] >= ret(0x12a4e8) (unsigned, 32-bit)` | `[[sp-28]+48] < ret(0x12a4e8) (unsigned, 32-bit)` |
| `bhs` at 0xc36a8 | `ret(0xfde7c) >= [sp-1608] (unsigned, 32-bit)` | `ret(0xfde7c) < [sp-1608] (unsigned, 32-bit)` |
| `bne` at 0xc36dc | `? != 0x0` | `? == 0x0` |
| `beq` at 0xc3718 | `ret(0x12a6b8) == 0x0` | `ret(0x12a6b8) != 0x0` |
| `bne` at 0xc3778 | `[[sp-28]+32]:1 != 0x0` | `[[sp-28]+32]:1 == 0x0` |
| `bne` at 0xc37b0 | `? != 0x0` | `? == 0x0` |
| `bhs` at 0xc37c4 | `[[sp-28]+48] >= ret(0x12a4e8) (unsigned, 32-bit)` | `[[sp-28]+48] < ret(0x12a4e8) (unsigned, 32-bit)` |
| `bhs` at 0xc37e4 | `ret(0xfde7c) >= [sp-1616] (unsigned, 32-bit)` | `ret(0xfde7c) < [sp-1616] (unsigned, 32-bit)` |
| `bne` at 0xc3818 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xc3860 | `ret(0x12a7a4) == 0x0` | `ret(0x12a7a4) != 0x0` |
| `bls` at 0xc38d8 | `[[sp-28]+32]:1 <= 0x1 (unsigned, 32-bit)` | `[[sp-28]+32]:1 > 0x1 (unsigned, 32-bit)` |
| `bne` at 0xc38ec | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc3928 | `[sp-1624] != [[sp-28]+32]:1-2` | `[sp-1624] == [[sp-28]+32]:1-2` |
| `bge` at 0xc3978 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xc39a0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xc3a14 | `ret(0x125b8c) == 0x0` | `ret(0x125b8c) != 0x0` |
| `bhi` at 0xc3aa0 | `[[sp-28]+32]:1 > 0x4 (unsigned, 32-bit)` | `[[sp-28]+32]:1 <= 0x4 (unsigned, 32-bit)` |
| `bne` at 0xc3ad0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xc3b1c | `? == [[sp-28]+32]:1` | `? != [[sp-28]+32]:1` |
| `bge` at 0xc3b68 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xc3b9c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc3c30 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc3cc4 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xc3d4c | `ret(0x125a58) == 0x0` | `ret(0x125a58) != 0x0` |
| `bhi` at 0xc3db8 | `[[sp-28]+32]:1 > 0x2 (unsigned, 32-bit)` | `[[sp-28]+32]:1 <= 0x2 (unsigned, 32-bit)` |
| `beq` at 0xc3de0 | `[sp-680] == 0x35` | `[sp-680] != 0x35` |
| `beq` at 0xc3dec | `[sp-680] == 0x40` | `[sp-680] != 0x40` |
| `beq` at 0xc3df8 | `[sp-680] == 0x4f` | `[sp-680] != 0x4f` |
| `beq` at 0xc3e04 | `[sp-680] == 0x42` | `[sp-680] != 0x42` |
| `beq` at 0xc3e10 | `[sp-680] == 0x46` | `[sp-680] != 0x46` |
| `beq` at 0xc3e1c | `[sp-680] == 0x4c` | `[sp-680] != 0x4c` |
| `beq` at 0xc3e28 | `[sp-680] == 0x4d` | `[sp-680] != 0x4d` |
| `beq` at 0xc3e34 | `[sp-680] == 0x24` | `[sp-680] != 0x24` |
| `beq` at 0xc3e40 | `[sp-680] == 0x2c` | `[sp-680] != 0x2c` |
| `beq` at 0xc3e4c | `[sp-680] == 0x7d` | `[sp-680] != 0x7d` |
| `beq` at 0xc3e58 | `[sp-680] == 0x2e` | `[sp-680] != 0x2e` |
| `beq` at 0xc3e64 | `[sp-680] == 0x7b` | `[sp-680] != 0x7b` |
| `bne` at 0xc3e7c | `? != 0x0` | `? == 0x0` |
| `beq` at 0xc3ea8 | `[sp-1648] == 0x2` | `[sp-1648] != 0x2` |
| `bhi` at 0xc3eb4 | `[sp-1648] > 0x2 (unsigned, 32-bit)` | `[sp-1648] <= 0x2 (unsigned, 32-bit)` |
| `beq` at 0xc3ec0 | `[sp-1648] == 0x1` | `[sp-1648] != 0x1` |
| `beq` at 0xc3ed0 | `[sp-1648] == 0x4` | `[sp-1648] != 0x4` |
| `bne` at 0xc3f88 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc3fbc | `? != 0x0` | `? == 0x0` |
| `bge` at 0xc4004 | `ret(0x4ed88) >= 0x0 (signed, 32-bit)` | `ret(0x4ed88) < 0x0 (signed, 32-bit)` |
| `bhi` at 0xc4184 | `[[sp-28]+32]:1 > 0x6 (unsigned, 32-bit)` | `[[sp-28]+32]:1 <= 0x6 (unsigned, 32-bit)` |
| `bne` at 0xc41b4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc4248 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc428c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc4314 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc439c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc4448 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xc451c | `ret(0x201a8) == 0x0` | `ret(0x201a8) != 0x0` |
| `beq` at 0xc4654 | `[[sp-28]+32]:1 == 0x1` | `[[sp-28]+32]:1 != 0x1` |
| `bge` at 0xc46a8 | `[sp-690]:1 >= ? (signed, 32-bit)` | `[sp-690]:1 < ? (signed, 32-bit)` |
| `bne` at 0xc470c | `? != 0x1` | `? == 0x1` |
| `bne` at 0xc4778 | `? != 0x4` | `? == 0x4` |
| `beq` at 0xc47b4 | `[sp-1688] == 0x2e` | `[sp-1688] != 0x2e` |
| `bgt` at 0xc47c0 | `[sp-1688] > 0x2e (signed, 32-bit)` | `[sp-1688] <= 0x2e (signed, 32-bit)` |
| `beq` at 0xc47cc | `[sp-1688] == 0x2b` | `[sp-1688] != 0x2b` |
| `beq` at 0xc47d8 | `[sp-1688] == 0x2c` | `[sp-1688] != 0x2c` |
| `beq` at 0xc47e8 | `[sp-1688] == 0x7b` | `[sp-1688] != 0x7b` |
| `beq` at 0xc47f4 | `[sp-1688] == 0x7d` | `[sp-1688] != 0x7d` |
| `beq` at 0xc4830 | `ret(0x3deec) == 0x400` | `ret(0x3deec) != 0x400` |
| `beq` at 0xc483c | `ret(0x3deec) == 0x600` | `ret(0x3deec) != 0x600` |
| `ble` at 0xc48ac | `ret(0x6e140) <= 0x1 (signed, 32-bit)` | `ret(0x6e140) > 0x1 (signed, 32-bit)` |
| `beq` at 0xc48c8 | `? == ret(0x6e190)` | `? != ret(0x6e190)` |
| `bge` at 0xc49e8 | `ret(0x75a8c) >= 0x0 (signed, 32-bit)` | `ret(0x75a8c) < 0x0 (signed, 32-bit)` |
| `beq` at 0xc4a60 | `[0x1b0ce8] == 0x0` | `[0x1b0ce8] != 0x0` |
| `beq` at 0xc4aac | `[sp-672] == 0x0` | `[sp-672] != 0x0` |
| `beq` at 0xc4adc | `ret(0xfd0dc) == 0x0` | `ret(0xfd0dc) != 0x0` |
| `beq` at 0xc4b24 | `ret(0x2302c) == 0x0` | `ret(0x2302c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc4b58 | `r0` | `ret(0xfb794)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xc0234).

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xc08c0).

## `RouteInit`

- `RouteInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf0e48 to 0xf0f20.
  Established. The call to unnamed 0xfa1b8 has its result saved at entry sp minus 0x14 (`bl` at 0xf0ea4, `str` at 0xf0eac).
  A later call to unnamed 0x219a4 has its result stored at entry sp minus 0x18 (`bl` at 0xf0ec0, `str` at 0xf0ed0).
  A word store increases the loaded value at absolute 0x1b0fc0 by one, whose software role is unknown (`str` at 0xf0ef8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xf0e58 | load | 4 | `abs 0xf0f04` | `-` |
| `ldr` at 0xf0e5c | load | 4 | `abs 0x1b0fc0` | `-` |
| `ldr` at 0xf0e68 | load | 4 | `abs 0xf0f08` | `-` |
| `str` at 0xf0e6c | store | 4 | `sp -0x1c` | `0x1850d4` |
| `ldr` at 0xf0e74 | load | 4 | `abs 0xf0f0c` | `-` |
| `ldr` at 0xf0e84 | load | 4 | `abs 0xf0f10` | `-` |
| `ldr` at 0xf0e94 | load | 4 | `abs 0xf0f10` | `-` |
| `ldr` at 0xf0e98 | load | 4 | `abs 0xf0f14` | `-` |
| `ldr` at 0xf0ea0 | load | 4 | `abs 0xf0f18` | `-` |
| `str` at 0xf0eac | store | 4 | `sp -0x14` | `ret(0xfa1b8)` |
| `ldr` at 0xf0eb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0ebc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0ec8 | load | 4 | `abs 0xf0f1c` | `-` |
| `str` at 0xf0ecc | store | 4 | `sp -0x1c` | `0x185110` |
| `str` at 0xf0ed0 | store | 4 | `sp -0x18` | `ret(0x219a4)` |
| `ldr` at 0xf0ed8 | load | 4 | `abs 0xf0f0c` | `-` |
| `ldr` at 0xf0ee8 | load | 4 | `abs 0xf0f04` | `-` |
| `ldr` at 0xf0eec | load | 4 | `abs 0xf0f04` | `-` |
| `ldr` at 0xf0ef0 | load | 4 | `abs 0x1b0fc0` | `-` |
| `str` at 0xf0ef8 | store | 4 | `abs 0x1b0fc0` | `[0x1b0fc0]+1` |
| `ldm` at 0xf0f00 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xf0f00 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xf0f00 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf0e80 | `(unknown callee) 0x22070` | `0x9` | `0x1850f8` | `0x2a` | `0x0` |
| `bl` at 0xf0e90 | `(unknown callee) 0xfb5cc` | `0x31b978` | `0x17` | `0x0` | `?` |
| `bl` at 0xf0ea4 | `(unknown callee) 0xfa1b8` | `0x31b978` | `0xffff017` | `0x1` | `0xf0f20` |
| `bl` at 0xf0ec0 | `(unknown callee) 0x219a4` | `ret(0xfa1b8)` | `?` | `?` | `ret(0xfa1b8)` |
| `bl` at 0xf0ee4 | `(unknown callee) 0x22070` | `0x9` | `0x1850f8` | `0x38` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xf0e64 | `[0x1b0fc0] == 0x0` | `[0x1b0fc0] != 0x0` |
| `beq` at 0xf0eb8 | `ret(0xfa1b8) == 0x0` | `ret(0xfa1b8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xf0f00 | `r0` | `?` |

## `RouteIRouter`

- `RouteIRouter`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf0f20 to 0xf10f0.
  Established. A word loaded from memory at entry r0 plus 0x1c is saved at entry sp minus 0x18 (`ldr` at 0xf0f38, `str` at 0xf0f3c).
  Two byte stores and a halfword store write zero to unknown effective addresses (`strb` at 0xf0f84, `strb` at 0xf0fa4, `strh` at 0xf1048).
  The call to unnamed 0xfd4a8 has its result saved at entry sp minus 0x1c (`bl` at 0xf1090, `str` at 0xf1098).
  A later word store writes the word loaded from the value at entry sp minus 20 plus 0xc into memory at that result plus 0xc (`bl` at 0xf1090, `ldr` at 0xf10bc, `str` at 0xf10c0).
  The stored word meaning remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf0f30 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xf0f34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0f38 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0xf0f3c | store | 4 | `sp -0x18` | `[r0+28]` |
| `str` at 0xf0f44 | store | 4 | `sp -0x220` | `0x0` |
| `ldr` at 0xf0f48 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xf0f4c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xf0f50 | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0xf0f5c | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0xf0f68 | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0xf0f6c | load | 4 | `abs 0xf10e0` | `-` |
| `strb` at 0xf0f84 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xf0f88 | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0xf0f8c | load | 4 | `abs 0xf10e0` | `-` |
| `strb` at 0xf0fa4 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xf0fa8 | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0xf0fac | load | 4 | `abs 0xf10e4` | `-` |
| `str` at 0xf0fc0 | store | 4 | `sp -0x224` | `?` |
| `ldr` at 0xf0fc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0fc8 | load | 4 | `sp -0x220` | `-` |
| `ldrb` at 0xf0fdc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xf0fec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0ff0 | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0xf1004 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf1008 | store | 4 | `sp -0x228` | `?` |
| `ldr` at 0xf1010 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1014 | load | 4 | `sp -0x220` | `-` |
| `str` at 0xf101c | store | 4 | `sp -0x228` | `ret(0xfde7c)` |
| `ldr` at 0xf1020 | load | 4 | `sp -0x228` | `-` |
| `ldr` at 0xf1024 | load | 4 | `sp -0x224` | `-` |
| `str` at 0xf1028 | store | 4 | `[sp-548] +0x0` | `?` |
| `ldr` at 0xf102c | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0xf1030 | load | 4 | `abs 0xf10e0` | `-` |
| `strh` at 0xf1048 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0xf104c | load | 4 | `sp -0x220` | `-` |
| `str` at 0xf1054 | store | 4 | `sp -0x220` | `[sp-544]+1` |
| `str` at 0xf1064 | store | 4 | `sp -0x238` | `0x1` |
| `str` at 0xf106c | store | 4 | `sp -0x234` | `0x2` |
| `ldr` at 0xf1070 | load | 4 | `sp -0x220` | `-` |
| `str` at 0xf1074 | store | 4 | `sp -0x230` | `?` |
| `str` at 0xf107c | store | 4 | `sp -0x22c` | `sp-540` |
| `ldr` at 0xf1080 | load | 4 | `abs 0xf10e8` | `-` |
| `str` at 0xf1098 | store | 4 | `sp -0x1c` | `ret(0xfd4a8)` |
| `ldr` at 0xf109c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf10a8 | load | 4 | `abs 0xf10ec` | `-` |
| `ldr` at 0xf10b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf10b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf10bc | load | 4 | `[sp-20] +0xc` | `-` |
| `str` at 0xf10c0 | store | 4 | `ret(0xfd4a8) +0xc` | `[[sp-20]+12]` |
| `ldr` at 0xf10c4 | load | 4 | `abs 0xf10e8` | `-` |
| `ldr` at 0xf10c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf10d0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xf10dc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xf10dc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xf10dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf1018 | `(unknown callee) 0xfde7c` | `?` | `?` | `?` | `?` |
| `bl` at 0xf1090 | `(unknown callee) 0xfd4a8` | `0x31b978` | `sp-24` | `0x0` | `0x0` |
| `bl` at 0xf10ac | `(unknown callee) 0x293f8` | `0x185140` | `?` | `?` | `ret(0xfd4a8)` |
| `bl` at 0xf10cc | `(unknown callee) 0xfca84` | `0x31b978` | `[sp-28]` | `[[sp-20]+12]` | `ret(0xfd4a8)` |
| `bl` at 0xf10d4 | `(unknown callee) 0xfb794` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xf0f58 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bgt` at 0xf0f64 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0xf0fe8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf10a4 | `ret(0xfd4a8) != 0x0` | `ret(0xfd4a8) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xf10dc | `r0` | `ret(0xfb794)` |

## `RouteProcessIn`

- `RouteProcessIn`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf10f0 to 0xf12bc.
  Established. A byte load reads memory at entry r0 plus 0x16, and a later store at entry sp minus 0x28 has an unknown value (`ldrb` at 0xf110c, `str` at 0xf111c).
  The call to unnamed 0xa3c7c has its result stored at absolute 0x1b0fc4 (`bl` at 0xf1134, `str` at 0xf1140).
  Two later word stores have unknown effective addresses and write the words at entry sp minus 44 and minus 48 respectively (`str` at 0xf1274, `str` at 0xf1294).
  The meanings of those software values and returned r0 value remain unknown (`ldm` at 0xf12b4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf1100 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xf1104 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xf1108 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xf110c | load | 1 | `r0 +0x16` | `-` |
| `str` at 0xf111c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xf1120 | load | 4 | `abs 0xf12b8` | `-` |
| `ldr` at 0xf1124 | load | 4 | `abs 0x1b0fc4` | `-` |
| `ldr` at 0xf113c | load | 4 | `abs 0xf12b8` | `-` |
| `str` at 0xf1140 | store | 4 | `abs 0x1b0fc4` | `ret(0xa3c7c)` |
| `str` at 0xf1148 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xf1150 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xf1154 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf1160 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf116c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1170 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xf1184 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xf1194 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1198 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf11ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf11bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf11c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf11d4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xf11dc | load | 4 | `abs 0xf12b8` | `-` |
| `ldr` at 0xf11e0 | load | 4 | `abs 0x1b0fc4` | `-` |
| `ldr` at 0xf11e4 | load | 4 | `[0x1b0fc4] +0xc` | `-` |
| `ldr` at 0xf11f8 | load | 4 | `abs 0xf12b8` | `-` |
| `ldr` at 0xf11fc | load | 4 | `abs 0x1b0fc4` | `-` |
| `ldr` at 0xf1200 | load | 4 | `[0x1b0fc4] +0x8` | `-` |
| `ldr` at 0xf1214 | load | 4 | `abs 0xf12b8` | `-` |
| `ldr` at 0xf1218 | load | 4 | `abs 0x1b0fc4` | `-` |
| `ldr` at 0xf121c | load | 4 | `[0x1b0fc4] +0x10` | `-` |
| `str` at 0xf122c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xf1230 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf1254 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xf1258 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf125c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1270 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xf1274 | store | 4 | `(unknown)` | `[sp-44]` |
| `ldr` at 0xf1278 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf127c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1290 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xf1294 | store | 4 | `(unknown)` | `[sp-48]` |
| `str` at 0xf129c | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0xf12a0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf12a8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldm` at 0xf12b4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xf12b4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xf12b4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xf12b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf1134 | `(unknown callee) 0xa3c7c` | `0x0` | `r1` | `r2` | `[0x1b0fc4]` |
| `bl` at 0xf11c4 | `(unknown callee) 0xfde7c` | `?` | `?` | `?` | `?` |
| `bl` at 0xf1240 | `(unknown callee) 0x202c8` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf112c | `[0x1b0fc4] != 0x0` | `[0x1b0fc4] == 0x0` |
| `bne` at 0xf115c | `Z == 0` | `not (Z == 0)` |
| `bgt` at 0xf1168 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0xf1190 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf11b4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xf11d0 | `ret(0xfde7c) != 0x0` | `ret(0xfde7c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xf12b4 | `r0` | `?` |

## `RouteProcessOut`

- `RouteProcessOut`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf12bc to 0xf1424.
  Established. A byte load reads memory at entry r0 plus 0x16, and the store at entry sp minus 0x24 has an unknown value (`ldrb` at 0xf12d8, `str` at 0xf12e8).
  The call to unnamed 0xa3c7c has its result stored at absolute 0x1b0fc8 (`bl` at 0xf1300, `str` at 0xf130c).
  A later word store writes the word at entry sp minus 40 to an unknown effective address (`str` at 0xf13fc).
  The store at entry sp minus 0x1c subtracts one from the loaded word there, whose software role remains unknown (`str` at 0xf1410).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf12cc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf12d0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xf12d4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xf12d8 | load | 1 | `r0 +0x16` | `-` |
| `str` at 0xf12e8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf12ec | load | 4 | `abs 0xf1420` | `-` |
| `ldr` at 0xf12f0 | load | 4 | `abs 0x1b0fc8` | `-` |
| `ldr` at 0xf1308 | load | 4 | `abs 0xf1420` | `-` |
| `str` at 0xf130c | store | 4 | `abs 0x1b0fc8` | `ret(0xa3c7c)` |
| `str` at 0xf1314 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xf131c | store | 4 | `sp -0x1c` | `0x3f` |
| `ldr` at 0xf1320 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf132c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf1338 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf133c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xf1350 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xf1360 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1364 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf1378 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf1388 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf138c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf13a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf13a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf13bc | load | 4 | `(unknown)` | `-` |
| `str` at 0xf13c0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xf13c4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xf13cc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf13dc | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xf13e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf13e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf13f8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf13fc | store | 4 | `(unknown)` | `[sp-40]` |
| `str` at 0xf1404 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xf1408 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf1410 | store | 4 | `sp -0x1c` | `[sp-28]-1` |
| `ldm` at 0xf141c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xf141c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xf141c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf1300 | `(unknown callee) 0xa3c7c` | `0x0` | `r1` | `r2` | `[0x1b0fc8]` |
| `bl` at 0xf1390 | `(unknown callee) 0xfde7c` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf12f8 | `[0x1b0fc8] != 0x0` | `[0x1b0fc8] == 0x0` |
| `bne` at 0xf1328 | `Z == 0` | `not (Z == 0)` |
| `blt` at 0xf1334 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xf135c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf1380 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xf139c | `ret(0xfde7c) != 0x0` | `ret(0xfde7c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xf141c | `r0` | `?` |

## `LpbkInit`

- `LpbkInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe05d4 to 0xe06ac.
  Established. The call to unnamed 0xfa1b8 has printed candidates 0x31b82c, 0xffff012, one and 0xe06ac (`bl` at 0xe0630);
  its result is saved at entry sp minus 0x14 (`bl` at 0xe0630, `str` at 0xe0638).
  A later call to unnamed 0x219a4 has its result saved at entry sp minus 0x18 (`bl` at 0xe064c, `str` at 0xe065c).
  The word store at absolute 0x1b0e18 increases the loaded value there by one, whose physical meaning is unknown (`str` at 0xe0684).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xe05e4 | load | 4 | `abs 0xe0690` | `-` |
| `ldr` at 0xe05e8 | load | 4 | `abs 0x1b0e18` | `-` |
| `ldr` at 0xe05f4 | load | 4 | `abs 0xe0694` | `-` |
| `str` at 0xe05f8 | store | 4 | `sp -0x1c` | `0x1828f4` |
| `ldr` at 0xe0600 | load | 4 | `abs 0xe0698` | `-` |
| `ldr` at 0xe0610 | load | 4 | `abs 0xe069c` | `-` |
| `ldr` at 0xe0620 | load | 4 | `abs 0xe069c` | `-` |
| `ldr` at 0xe0624 | load | 4 | `abs 0xe06a0` | `-` |
| `ldr` at 0xe062c | load | 4 | `abs 0xe06a4` | `-` |
| `str` at 0xe0638 | store | 4 | `sp -0x14` | `ret(0xfa1b8)` |
| `ldr` at 0xe063c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe0648 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe0654 | load | 4 | `abs 0xe06a8` | `-` |
| `str` at 0xe0658 | store | 4 | `sp -0x1c` | `0x182930` |
| `str` at 0xe065c | store | 4 | `sp -0x18` | `ret(0x219a4)` |
| `ldr` at 0xe0664 | load | 4 | `abs 0xe0698` | `-` |
| `ldr` at 0xe0674 | load | 4 | `abs 0xe0690` | `-` |
| `ldr` at 0xe0678 | load | 4 | `abs 0xe0690` | `-` |
| `ldr` at 0xe067c | load | 4 | `abs 0x1b0e18` | `-` |
| `str` at 0xe0684 | store | 4 | `abs 0x1b0e18` | `[0x1b0e18]+1` |
| `ldm` at 0xe068c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe068c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe068c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe060c | `(unknown callee) 0x22070` | `0x9` | `0x182918` | `0x2a` | `0x0` |
| `bl` at 0xe061c | `(unknown callee) 0xfb5cc` | `0x31b82c` | `0x12` | `0x0` | `?` |
| `bl` at 0xe0630 | `(unknown callee) 0xfa1b8` | `0x31b82c` | `0xffff012` | `0x1` | `0xe06ac` |
| `bl` at 0xe064c | `(unknown callee) 0x219a4` | `ret(0xfa1b8)` | `?` | `?` | `ret(0xfa1b8)` |
| `bl` at 0xe0670 | `(unknown callee) 0x22070` | `0x9` | `0x182918` | `0x38` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe05f0 | `[0x1b0e18] == 0x0` | `[0x1b0e18] != 0x0` |
| `beq` at 0xe0644 | `ret(0xfa1b8) == 0x0` | `ret(0xfa1b8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe068c | `r0` | `?` |

## `LpbkIRouter`

- `LpbkIRouter`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe06ac to 0xe0b18.
  Established. A word loaded from memory at entry r0 plus 0x1c is saved at entry sp minus 0x18 (`ldr` at 0xe06c4, `str` at 0xe06c8).
  The call to unnamed 0xfde7c at 0xe0800 has printed candidates the words at entry sp minus 20 and minus 64, the word at entry sp minus 20, and unknown (`bl` at 0xe0800).
  The call to unnamed 0x2031c has a halfword consumer that stores its result plus one masked to 16 bits (`bl` at 0xe0a78, `strh` at 0xe0a88).
  That halfword store targets memory at the word held at entry sp minus 92 plus two (`strh` at 0xe0a88).
  The use of that halfword and the computed storage remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe06bc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xe06c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe06c4 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0xe06c8 | store | 4 | `sp -0x18` | `[r0+28]` |
| `str` at 0xe06d0 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xe06d4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xe06d8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xe06dc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe06e8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe06f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe06f8 | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0xe070c | load | 1 | `(unknown)` | `-` |
| `str` at 0xe0714 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0xe0718 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe0724 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe0730 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe0740 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe0750 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0xe076c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xe0770 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0xe078c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xe0790 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe07a8 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xe07ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe07b0 | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0xe07c4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xe07d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe07d8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe07ec | load | 4 | `(unknown)` | `-` |
| `str` at 0xe07f0 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xe07f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe07fc | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe0804 | store | 4 | `sp -0x48` | `ret(0xfde7c)` |
| `ldr` at 0xe0808 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xe080c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe0810 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xe0814 | load | 4 | `sp -0x40` | `-` |
| `strh` at 0xe0830 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0xe0838 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0xe0854 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0xe0858 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0xe0874 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xe0878 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe0890 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xe0894 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe0898 | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0xe08ac | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xe08bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe08c0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe08d4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe08d8 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xe08e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe08e4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe08ec | store | 4 | `sp -0x50` | `ret(0xfde7c)` |
| `ldr` at 0xe08f0 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xe08f4 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xe08f8 | store | 4 | `[sp-76] +0x0` | `?` |
| `ldr` at 0xe08fc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0914 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe0918 | load | 4 | `sp -0x40` | `-` |
| `ldrh` at 0xe092c | load | 2 | `(unknown)` | `-` |
| `strh` at 0xe0930 | store | 2 | `(unknown)` | `?` |
| `ldr` at 0xe0938 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0xe0954 | store | 1 | `(unknown)` | `0x2` |
| `ldr` at 0xe0958 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0xe0974 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xe0978 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe0990 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xe0994 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe0998 | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0xe09ac | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xe09bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe09c0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe09d4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe09d8 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xe09e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe09e4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe09ec | store | 4 | `sp -0x58` | `ret(0xfde7c)` |
| `ldr` at 0xe09f0 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xe09f4 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xe09f8 | store | 4 | `[sp-84] +0x0` | `?` |
| `ldr` at 0xe09fc | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe0a14 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0xe0a18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe0a1c | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0xe0a30 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xe0a40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe0a44 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0a58 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe0a5c | store | 4 | `sp -0x60` | `?` |
| `ldr` at 0xe0a64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe0a68 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe0a70 | store | 4 | `sp -0x60` | `ret(0xfde7c)` |
| `ldr` at 0xe0a74 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xe0a84 | load | 4 | `sp -0x5c` | `-` |
| `strh` at 0xe0a88 | store | 2 | `[sp-92] +0x2` | `(ret(0x2031c)+1 & 0xffff)` |
| `ldr` at 0xe0a8c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe0a94 | store | 4 | `sp -0x40` | `?` |
| `str` at 0xe0aa4 | store | 4 | `sp -0x74` | `0x1` |
| `str` at 0xe0aac | store | 4 | `sp -0x70` | `0x2` |
| `ldr` at 0xe0ab0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe0ab4 | store | 4 | `sp -0x6c` | `?` |
| `str` at 0xe0abc | store | 4 | `sp -0x68` | `sp-60` |
| `ldr` at 0xe0ac0 | load | 4 | `abs 0xe0b10` | `-` |
| `str` at 0xe0ad8 | store | 4 | `sp -0x1c` | `ret(0xfd4a8)` |
| `ldr` at 0xe0adc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe0ae8 | load | 4 | `abs 0xe0b14` | `-` |
| `ldr` at 0xe0af4 | load | 4 | `abs 0xe0b10` | `-` |
| `ldr` at 0xe0af8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe0b00 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xe0b0c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe0b0c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe0b0c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe0800 | `(unknown callee) 0xfde7c` | `[sp-20]` | `[sp-64]` | `[sp-20]` | `?` |
| `bl` at 0xe08e8 | `(unknown callee) 0xfde7c` | `[sp-20]` | `[sp-64]` | `[sp-20]` | `?` |
| `bl` at 0xe09e8 | `(unknown callee) 0xfde7c` | `[sp-20]` | `[sp-64]` | `[sp-20]` | `?` |
| `bl` at 0xe0a6c | `(unknown callee) 0xfde7c` | `[sp-20]` | `[sp-64]` | `[sp-20]` | `?` |
| `bl` at 0xe0a78 | `(unknown callee) 0x2031c` | `?` | `?` | `?` | `?` |
| `bl` at 0xe0ad0 | `(unknown callee) 0xfd4a8` | `0x31b82c` | `sp-24` | `0x0` | `0x0` |
| `bl` at 0xe0aec | `(unknown callee) 0x293f8` | `0x182960` | `?` | `?` | `ret(0xfd4a8)` |
| `bl` at 0xe0afc | `(unknown callee) 0xfca84` | `0x31b82c` | `ret(0xfd4a8)` | `?` | `ret(0xfd4a8)` |
| `bl` at 0xe0b04 | `(unknown callee) 0xfb794` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xe06e4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bgt` at 0xe06f0 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0xe0720 | `[sp-100] == 0x1` | `[sp-100] != 0x1` |
| `bgt` at 0xe072c | `[sp-100] > 0x1 (signed, 32-bit)` | `[sp-100] <= 0x1 (signed, 32-bit)` |
| `beq` at 0xe0738 | `[sp-100] == 0x0` | `[sp-100] != 0x0` |
| `beq` at 0xe0748 | `[sp-100] == 0x2` | `[sp-100] != 0x2` |
| `bne` at 0xe07d0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe08b8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe09b8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe0a3c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe0ae4 | `ret(0xfd4a8) != 0x0` | `ret(0xfd4a8) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe0b0c | `r0` | `ret(0xfb794)` |

## `EvtInit`

- `EvtInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x955f0 to 0x956cc.
  Established. A word store writes 0x95800 at entry sp minus 0x18, while an alternative store there writes zero (`str` at 0x95624, `str` at 0x95630).
  The call to unnamed 0xfa1b8 has its result saved at entry sp minus 0x14 and later at entry sp minus 0x1c (`bl` at 0x9565c, `str` at 0x95664, `str` at 0x95680).
  A word store writes one at absolute 0x1a654c (`str` at 0x956a0).
  The meaning of that software field and the final r0 value remain unknown (`ldm` at 0x956a8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x95600 | load | 4 | `abs 0x956ac` | `-` |
| `ldr` at 0x95610 | load | 4 | `abs 0x956b0` | `-` |
| `ldr` at 0x95614 | load | 4 | `abs 0x1a0348` | `-` |
| `ldr` at 0x95620 | load | 4 | `abs 0x956b4` | `-` |
| `str` at 0x95624 | store | 4 | `sp -0x18` | `0x95800` |
| `str` at 0x95630 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x95634 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9563c | load | 4 | `abs 0x956b8` | `-` |
| `ldr` at 0x95640 | load | 4 | `abs 0x1a654c` | `-` |
| `ldr` at 0x9564c | load | 4 | `abs 0x956ac` | `-` |
| `ldr` at 0x95650 | load | 4 | `abs 0x956bc` | `-` |
| `ldr` at 0x95658 | load | 4 | `abs 0x956c0` | `-` |
| `str` at 0x95664 | store | 4 | `sp -0x14` | `ret(0xfa1b8)` |
| `ldr` at 0x95668 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x95674 | load | 4 | `abs 0x956c4` | `-` |
| `str` at 0x95678 | store | 4 | `sp -0x20` | `0x16a3b4` |
| `ldr` at 0x9567c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x95680 | store | 4 | `sp -0x1c` | `ret(0xfa1b8)` |
| `ldr` at 0x95688 | load | 4 | `abs 0x956c8` | `-` |
| `ldr` at 0x95698 | load | 4 | `abs 0x956b8` | `-` |
| `str` at 0x956a0 | store | 4 | `abs 0x1a654c` | `0x1` |
| `ldm` at 0x956a8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x956a8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x956a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9560c | `(unknown callee) 0xfb5cc` | `0x1a6550` | `0x1` | `0x0` | `r3` |
| `bl` at 0x95638 | `(unknown callee) 0x2498c` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x9565c | `(unknown callee) 0xfa1b8` | `0x1a6550` | `0xffff001` | `0x1` | `0x956cc` |
| `bl` at 0x95694 | `(unknown callee) 0x22070` | `0x9` | `0x16a3e0` | `0x43` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x9561c | `[0x1a0348] == 0x0` | `[0x1a0348] != 0x0` |
| `bne` at 0x95648 | `[0x1a654c] != 0x0` | `[0x1a654c] == 0x0` |
| `beq` at 0x95670 | `ret(0xfa1b8) == 0x0` | `ret(0xfa1b8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x956a8 | `r0` | `?` |
