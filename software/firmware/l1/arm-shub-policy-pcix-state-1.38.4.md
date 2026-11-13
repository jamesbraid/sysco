# ARM SHUB policy and PCI-X state in 1.38.4

Read from `6.5.28/usr/cpu/firmware/sysco/l1.bin`, package `6.5.28:l1#4` (version 1.38.4),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53cfa0`.
Carrier SHA-256: `e5b65e5f28d7fba6013a8618f55c34bed7f60f7c6f46d7e9c5b8b7b2eabbe7b3`.
Package SHA-256: `c6693d19c6eb8c3fcf00cbb84d6974d1b5b17b930fb5375c1df2df20894edec9`.
Inner executable SHA-256: `f652c04c77cd7c53ca7ae6d6d6f581f11200efa211ff089bfba0d8bd4f718eb8`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1254136 bytes.
Segment SHA-256: `a373bb774d119f083e52d394fbd2701ebfbf35210301a40dcdb4ebdc39bf6bdc`. Analysed routine spans use the exact
ELF function symbols and sizes. They can include trailing literal words after a return.

`sp` denotes the entry stack pointer. Register expressions such as `r0` denote entry
values, and brackets denote loaded values. Decimal numbers inside expressions retain the
exact tool notation. Call candidates use the assumed ARM AAPCS
register locations r0 through r3 with unknown arity. Stack stores appear in the access tables.
Unknown values and predicates include unsupported instruction effects and merged paths.
Numeric call targets have unknown callee names in the flat-image analysis. Device roles
and pointee meanings remain unknown unless the body establishes them. Tables include all
explicit memory operands on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
No unresolved computed transfers were reported for these routines. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/shub-policy-pcix-state-facts.txt) retain all tool sections.

## `SHUB_SetTP`

- `SHUB_SetTP`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4c574 to 0x4c5e4.
  Established. A byte store writes `([sp-20]:1 & 0xff)` at an unknown effective address (`strb` at
  0x4c5c8). The loop branch retains `le flags condition (operands unknown)` for taken and its
  negation for fall-through (`ble` at 0x4c598). No calls appear in the facts. TP meaning and r0 at
  exit remain unresolved (`ldmdb` at 0x4c5dc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4c584 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4c58c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x4c590 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4c5a0 | load | 4 | `abs 0x4c5e0` | `-` |
| `ldr` at 0x4c5a4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4c5c4 | load | 1 | `sp -0x14` | `-` |
| `strb` at 0x4c5c8 | store | 1 | `(unknown)` | `([sp-20]:1 & 0xff)` |
| `ldr` at 0x4c5cc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4c5d4 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldmdb` at 0x4c5dc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4c5dc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4c5dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x4c598 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4c5dc | `r0` | `?` |

## `SHUB_GetTP`

- `SHUB_GetTP`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4c5e4 to 0x4c604.
  Established. The byte read addresses absolute `0x2b0774` (`ldrb` at 0x4c5f4). The return reports
  `[0x2b0774]:1` (`ldmdb` at 0x4c5fc). Calls and conditional branches are absent. The field has
  unknown physical interpretation.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x4c5f0 | load | 4 | `abs 0x4c600` | `-` |
| `ldrb` at 0x4c5f4 | load | 1 | `abs 0x2b0774` | `-` |
| `ldmdb` at 0x4c5fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4c5fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4c5fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4c5fc | `r0` | `[0x2b0774]:1` |

## `SHUB_WarCheck`

- `SHUB_WarCheck`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4db2c to 0x4dbcc.
  Established. The deciding branch takes its bypass for an unknown zero operand and falls through
  for nonzero (`beq` at 0x4db7c). The sole call to `0x21e28` has `0x1`, `0x109480`, `0x819`, `0x0`
  (`bl` at 0x4dbb0). The common r0 remains unknown (`ldmdb` at 0x4dbb4). The checked physical
  condition is unestablished.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4db3c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4db40 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x4db44 | load | 4 | `abs 0x4dbb8` | `-` |
| `ldrb` at 0x4db48 | load | 1 | `abs 0x148234` | `-` |
| `ldr` at 0x4db4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4db5c | load | 4 | `abs 0x4dbbc` | `-` |
| `ldrb` at 0x4db60 | load | 1 | `abs 0x148235` | `-` |
| `ldr` at 0x4db64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4db80 | load | 4 | `abs 0x4dbc0` | `-` |
| `str` at 0x4db84 | store | 4 | `sp -0x24` | `0x10a43c` |
| `ldr` at 0x4db88 | load | 4 | `abs 0x4dbb8` | `-` |
| `ldrb` at 0x4db8c | load | 1 | `abs 0x148234` | `-` |
| `str` at 0x4db90 | store | 4 | `sp -0x20` | `[0x148234]:1` |
| `ldr` at 0x4db94 | load | 4 | `abs 0x4dbbc` | `-` |
| `ldrb` at 0x4db98 | load | 1 | `abs 0x148235` | `-` |
| `str` at 0x4db9c | store | 4 | `sp -0x1c` | `[0x148235]:1` |
| `ldr` at 0x4dba4 | load | 4 | `abs 0x4dbc4` | `-` |
| `ldr` at 0x4dba8 | load | 4 | `abs 0x4dbc8` | `-` |
| `ldmdb` at 0x4dbb4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4dbb4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4dbb4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4dbb0 | `(unknown callee) 0x21e28` | `0x1` | `0x109480` | `0x819` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4db7c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4dbb4 | `r0` | `?` |

## `SHUB_WarStatus`

- `SHUB_WarStatus`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4dbcc to 0x4dc1c.
  Established. The entry-r0 guard takes its bypass on zero and falls through on nonzero (`beq` at
  0x4dbe8). The call to `0x1b49c` receives entry r0, `0x10a484`, entry r2, entry r0 (`bl` at
  0x4dbf4). The exit value is unresolved (`ldmdb` at 0x4dc10). Status-bit meaning remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4dbdc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x4dbe0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4dbec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4dbf0 | load | 4 | `abs 0x4dc14` | `-` |
| `ldr` at 0x4dbf8 | load | 4 | `abs 0x4dc18` | `-` |
| `ldrb` at 0x4dbfc | load | 1 | `abs 0x148235` | `-` |
| `ldmdb` at 0x4dc10 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4dc10 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4dc10 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4dbf4 | `(unknown callee) 0x1b49c` | `r0` | `0x10a484` | `r2` | `r0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4dbe8 | `r0 == 0x0` | `r0 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4dc10 | `r0` | `?` |

## `SHUB_WarAllow`

- `SHUB_WarAllow`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4dc1c to 0x4dc44.
  Established. The byte store at absolute `0x148234` carries `([sp-20]:1 & 0xff)` (`strb` at
  0x4dc38). The exit retains entry r0 (`ldmdb` at 0x4dc3c). No calls or conditional branches are
  reported. Allow-policy semantics remain unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4dc2c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x4dc30 | load | 4 | `abs 0x4dc40` | `-` |
| `ldrb` at 0x4dc34 | load | 1 | `sp -0x14` | `-` |
| `strb` at 0x4dc38 | store | 1 | `abs 0x148234` | `([sp-20]:1 & 0xff)` |
| `ldmdb` at 0x4dc3c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4dc3c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4dc3c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4dc3c | `r0` | `r0` |

## `SHUB_WarEnable`

- `SHUB_WarEnable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4dc44 to 0x4dd44.
  Established. Byte stores in separate arms set absolute `0x148235` to `0x1` and `0x0` (`strb` at
  0x4dcd4, `strb` at 0x4dd0c). The entry-r0 test takes its bypass on zero and falls through on
  nonzero (`beq` at 0x4dc60). An indirect call has unknown target and candidates `0xff`, entry r1,
  entry r2, `[0x14806c]` (`mov` at 0x4dc94). Enable-policy ownership and the joined return remain
  unknown (`ldmdb` at 0x4dd18).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4dc54 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x4dc58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4dc64 | load | 4 | `abs 0x4dd1c` | `-` |
| `ldrb` at 0x4dc68 | load | 1 | `abs 0x148234` | `-` |
| `ldr` at 0x4dc74 | load | 4 | `abs 0x4dd20` | `-` |
| `ldr` at 0x4dc78 | load | 4 | `abs 0x14806c` | `-` |
| `ldr` at 0x4dc84 | load | 4 | `abs 0x4dd20` | `-` |
| `ldr` at 0x4dc88 | load | 4 | `abs 0x14806c` | `-` |
| `ldr` at 0x4dca4 | load | 4 | `abs 0x4dd24` | `-` |
| `str` at 0x4dca8 | store | 4 | `sp -0x18` | `0x10a494` |
| `ldr` at 0x4dcb0 | load | 4 | `abs 0x4dd28` | `-` |
| `ldr` at 0x4dcc4 | load | 4 | `abs 0x4dd2c` | `-` |
| `ldr` at 0x4dccc | load | 4 | `abs 0x4dd30` | `-` |
| `strb` at 0x4dcd4 | store | 1 | `abs 0x148235` | `0x1` |
| `ldr` at 0x4dcd8 | load | 4 | `abs 0x4dd34` | `-` |
| `ldr` at 0x4dce4 | load | 4 | `abs 0x4dd38` | `-` |
| `str` at 0x4dce8 | store | 4 | `sp -0x18` | `0x10a4c0` |
| `ldr` at 0x4dcf0 | load | 4 | `abs 0x4dd28` | `-` |
| `ldr` at 0x4dcf4 | load | 4 | `abs 0x4dd3c` | `-` |
| `ldr` at 0x4dd04 | load | 4 | `abs 0x4dd30` | `-` |
| `strb` at 0x4dd0c | store | 1 | `abs 0x148235` | `0x0` |
| `ldr` at 0x4dd10 | load | 4 | `abs 0x4dd40` | `-` |
| `ldmdb` at 0x4dd18 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4dd18 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4dd18 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x4dc94 | `(unknown callee) (unknown target)` | `0xff` | `r1` | `r2` | `[0x14806c]` |
| `bl` at 0x4dcbc | `(unknown callee) 0x21e28` | `0x1` | `0x109480` | `0x840` | `0x0` |
| `bl` at 0x4dcc8 | `(unknown callee) 0x20240` | `0x10a4b4` | `?` | `?` | `?` |
| `bl` at 0x4dcdc | `(unknown callee) 0x22dc0` | `0x1481e0` | `?` | `0x1` | `0x148235` |
| `bl` at 0x4dcfc | `(unknown callee) 0x21e28` | `0x3` | `0x109480` | `0x848` | `0x0` |
| `bl` at 0x4dd14 | `(unknown callee) 0x20240` | `0x10a4f4` | `r1` | `0x0` | `0x148235` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4dc60 | `r0 == 0x0` | `r0 != 0x0` |
| `beq` at 0x4dc70 | `[0x148234]:1 == 0x0` | `[0x148234]:1 != 0x0` |
| `beq` at 0x4dc80 | `[0x14806c] == 0x0` | `[0x14806c] != 0x0` |
| `beq` at 0x4dca0 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4dd18 | `r0` | `?` |

## `pcixASICInit`

- `pcixASICInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5796c to 0x57a68.
  Established. Word stores at unknown effective targets carry entry r1, `[sp-28]` and `[sp-32]`,
  respectively (`str` at 0x579ac, `str` at 0x579d8, `str` at 0x57a04). The exit reports r0
  `0x148364` (`ldmdb` at 0x57a60). No calls or deciding branches appear in the facts. ASIC record
  geometry and physical register meaning remain unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5797c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x57980 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x57984 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x57988 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x5798c | load | 4 | `abs 0x57a64` | `-` |
| `ldr` at 0x57990 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x579a8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x579ac | store | 4 | `(unknown)` | `r1` |
| `ldr` at 0x579b0 | load | 4 | `abs 0x57a64` | `-` |
| `ldr` at 0x579b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x579d4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x579d8 | store | 4 | `(unknown)` | `[sp-28]` |
| `ldr` at 0x579dc | load | 4 | `abs 0x57a64` | `-` |
| `ldr` at 0x579e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x57a00 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x57a04 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0x57a08 | load | 4 | `abs 0x57a64` | `-` |
| `ldr` at 0x57a0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x57a2c | load | 4 | `sp +0x0` | `-` |
| `str` at 0x57a30 | store | 4 | `(unknown)` | `[sp]` |
| `ldr` at 0x57a34 | load | 4 | `abs 0x57a64` | `-` |
| `ldr` at 0x57a38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x57a58 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x57a5c | store | 4 | `(unknown)` | `[sp+4]` |
| `ldmdb` at 0x57a60 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x57a60 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x57a60 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x57a60 | `r0` | `0x148364` |

## `pcixGetBusCfg`

- `pcixGetBusCfg`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x57af8 to 0x57b88.
  Established. The initial test takes its continuation for `[0x148360] != 0x0` and falls through on
  zero (`bne` at 0x57b1c). A word store writes `[[sp-28]+4]` at `[sp-24] +0x0` (`str` at 0x57b68);
  its prior zero test takes the bypass and nonzero falls through (`beq` at 0x57b58). Bus-record
  fields and the exit r0 remain unknown (`ldmdb` at 0x57b7c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x57b08 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x57b0c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x57b10 | load | 4 | `abs 0x57b80` | `-` |
| `ldr` at 0x57b14 | load | 4 | `abs 0x148360` | `-` |
| `str` at 0x57b24 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x57b2c | load | 4 | `abs 0x57b84` | `-` |
| `ldr` at 0x57b30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x57b44 | load | 4 | `abs 0x14835c` | `-` |
| `str` at 0x57b4c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x57b50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57b5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57b60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57b64 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0x57b68 | store | 4 | `[sp-24] +0x0` | `[[sp-28]+4]` |
| `ldr` at 0x57b6c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57b70 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0x57b74 | store | 4 | `sp -0x20` | `[[sp-28]+8]` |
| `ldr` at 0x57b78 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x57b7c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x57b7c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x57b7c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x57b1c | `[0x148360] != 0x0` | `[0x148360] == 0x0` |
| `beq` at 0x57b58 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x57b7c | `r0` | `?` |

## `pcixLedOn`

- `pcixLedOn`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x581d4 to 0x5843c.
  Established. The first call to `0x5c008` carries `[sp-20]`, unknown, `0x1`, unknown (`bl` at
  0x582ac). The global guard takes the continuation for `[0x148360] != 0x0` and falls through on
  zero (`bne` at 0x5820c). The following byte comparison takes its branch for `(r1 & 0xff) == 0xff`
  and falls through otherwise (`beq` at 0x58218). LED mapping and final r0 are unresolved (`ldmdb`
  at 0x5842c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x581e4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x581ec | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x581f4 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x581fc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x58200 | load | 4 | `abs 0x58430` | `-` |
| `ldr` at 0x58204 | load | 4 | `abs 0x148360` | `-` |
| `ldrb` at 0x58210 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5821c | load | 4 | `abs 0x58434` | `-` |
| `str` at 0x58220 | store | 4 | `sp -0x38` | `0xfffff703` |
| `str` at 0x5822c | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x58230 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x58234 | store | 4 | `sp -0x34` | `[sp-56]` |
| `ldr` at 0x5823c | load | 4 | `abs 0x58438` | `-` |
| `ldr` at 0x58240 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58254 | load | 4 | `abs 0x14835c` | `-` |
| `str` at 0x5825c | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x58260 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x58270 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x58274 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58278 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5827c | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x5828c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58290 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x58298 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x582a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x582b4 | store | 4 | `sp -0x2c` | `ret(0x5c008)` |
| `ldr` at 0x582b8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x582c4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x582d0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x582d4 | store | 4 | `sp -0x28` | `ret(0x5c008)` |
| `ldr` at 0x582d8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x582e0 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x582e8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x582ec | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x582f4 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x58300 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x58304 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58314 | store | 4 | `sp -0x20` | `ret(0x5b130)` |
| `ldr` at 0x58318 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x58324 | load | 4 | `abs 0x58434` | `-` |
| `str` at 0x58328 | store | 4 | `sp -0x34` | `0xfffff703` |
| `ldr` at 0x58330 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x58334 | load | 1 | `ret(0x5b130) +0x1` | `-` |
| `str` at 0x58338 | store | 4 | `sp -0x30` | `[ret(0x5b130)+1]:1` |
| `ldr` at 0x5833c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x58340 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x58344 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x58348 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x58350 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x58360 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58364 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5836c | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x58374 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58388 | store | 4 | `sp -0x2c` | `ret(0x5c008)` |
| `ldr` at 0x5838c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x58398 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x583a4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x583a8 | store | 4 | `sp -0x28` | `ret(0x5c008)` |
| `ldr` at 0x583ac | load | 4 | `sp -0x30` | `-` |
| `str` at 0x583b4 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x583bc | load | 4 | `sp -0x28` | `-` |
| `str` at 0x583c0 | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x583c8 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x583cc | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x583d0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x583e0 | store | 4 | `sp -0x24` | `ret(0x5b20c)` |
| `ldr` at 0x583e4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x583f0 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x583f4 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x583f8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58408 | store | 4 | `sp -0x34` | `ret(0x5b3ec)` |
| `ldr` at 0x58410 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58414 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x58424 | store | 4 | `sp -0x34` | `ret(0x5c008)` |
| `ldr` at 0x58428 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x5842c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5842c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5842c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x582ac | `(unknown callee) 0x5c008` | `[sp-20]` | `?` | `0x1` | `?` |
| `bl` at 0x5830c | `(unknown callee) 0x5b130` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x58380 | `(unknown callee) 0x5c008` | `[sp-20]` | `?` | `0x1` | `?` |
| `bl` at 0x583d8 | `(unknown callee) 0x5b20c` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x58400 | `(unknown callee) 0x5b3ec` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x5841c | `(unknown callee) 0x5c008` | `[sp-20]` | `ret(0x5b20c)` | `0x1` | `ret(0x5b20c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5820c | `[0x148360] != 0x0` | `[0x148360] == 0x0` |
| `beq` at 0x58218 | `(r1 & 0xff) == 0xff` | `(r1 & 0xff) != 0xff` |
| `bne` at 0x58268 | `[sp-21]:1 != 0xff` | `[sp-21]:1 == 0xff` |
| `blt` at 0x58284 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x582c0 | `ret(0x5c008) == 0x0` | `ret(0x5c008) != 0x0` |
| `bne` at 0x582cc | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0x582fc | `[sp-22]:1 != 0xff` | `[sp-22]:1 == 0xff` |
| `bne` at 0x58320 | `ret(0x5b130) != 0x0` | `ret(0x5b130) == 0x0` |
| `blt` at 0x58358 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x58394 | `ret(0x5c008) == 0x0` | `ret(0x5c008) != 0x0` |
| `bne` at 0x583a0 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0x583ec | `ret(0x5b20c) != 0x0` | `ret(0x5b20c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5842c | `r0` | `?` |

## `pcixLedOff`

- `pcixLedOff`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5843c to 0x586a4.
  Established. The initial global test takes its continuation for `[0x148360] != 0x0` and falls
  through on zero (`bne` at 0x58474). The first call to `0x5c008` has `[sp-20]`, unknown, `0x0`,
  unknown (`bl` at 0x58514). The later calls are retained separately with their own candidates. LED
  polarity and r0 at the common exit remain unknown (`ldmdb` at 0x58694).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5844c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x58454 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x5845c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x58464 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x58468 | load | 4 | `abs 0x58698` | `-` |
| `ldr` at 0x5846c | load | 4 | `abs 0x148360` | `-` |
| `ldrb` at 0x58478 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x58484 | load | 4 | `abs 0x5869c` | `-` |
| `str` at 0x58488 | store | 4 | `sp -0x38` | `0xfffff703` |
| `str` at 0x58494 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x58498 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x5849c | store | 4 | `sp -0x34` | `[sp-56]` |
| `ldr` at 0x584a4 | load | 4 | `abs 0x586a0` | `-` |
| `ldr` at 0x584a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x584bc | load | 4 | `abs 0x14835c` | `-` |
| `str` at 0x584c4 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x584c8 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x584d8 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x584dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x584e0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x584e4 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x584f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x584f8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x58500 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x58508 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5851c | store | 4 | `sp -0x2c` | `ret(0x5c008)` |
| `ldr` at 0x58520 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5852c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x58538 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x5853c | store | 4 | `sp -0x28` | `ret(0x5c008)` |
| `ldr` at 0x58540 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x58548 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x58550 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x58554 | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x5855c | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x58568 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5856c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5857c | store | 4 | `sp -0x20` | `ret(0x5b130)` |
| `ldr` at 0x58580 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5858c | load | 4 | `abs 0x5869c` | `-` |
| `str` at 0x58590 | store | 4 | `sp -0x34` | `0xfffff703` |
| `ldr` at 0x58598 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x5859c | load | 1 | `ret(0x5b130) +0x1` | `-` |
| `str` at 0x585a0 | store | 4 | `sp -0x30` | `[ret(0x5b130)+1]:1` |
| `ldr` at 0x585a4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x585a8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x585ac | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x585b0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x585b8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x585c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x585cc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x585d4 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x585dc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x585f0 | store | 4 | `sp -0x2c` | `ret(0x5c008)` |
| `ldr` at 0x585f4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x58600 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5860c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x58610 | store | 4 | `sp -0x28` | `ret(0x5c008)` |
| `ldr` at 0x58614 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x5861c | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x58624 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x58628 | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x58630 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x58634 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x58638 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58648 | store | 4 | `sp -0x24` | `ret(0x5b20c)` |
| `ldr` at 0x5864c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x58658 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x5865c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x58660 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58670 | store | 4 | `sp -0x34` | `ret(0x5b3ec)` |
| `ldr` at 0x58678 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5867c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5868c | store | 4 | `sp -0x34` | `ret(0x5c008)` |
| `ldr` at 0x58690 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x58694 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x58694 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x58694 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x58514 | `(unknown callee) 0x5c008` | `[sp-20]` | `?` | `0x0` | `?` |
| `bl` at 0x58574 | `(unknown callee) 0x5b130` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x585e8 | `(unknown callee) 0x5c008` | `[sp-20]` | `?` | `0x1` | `?` |
| `bl` at 0x58640 | `(unknown callee) 0x5b20c` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x58668 | `(unknown callee) 0x5b3ec` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x58684 | `(unknown callee) 0x5c008` | `[sp-20]` | `ret(0x5b20c)` | `0x0` | `ret(0x5b20c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x58474 | `[0x148360] != 0x0` | `[0x148360] == 0x0` |
| `beq` at 0x58480 | `(r1 & 0xff) == 0xff` | `(r1 & 0xff) != 0xff` |
| `bne` at 0x584d0 | `[sp-21]:1 != 0xff` | `[sp-21]:1 == 0xff` |
| `blt` at 0x584ec | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x58528 | `ret(0x5c008) == 0x0` | `ret(0x5c008) != 0x0` |
| `bne` at 0x58534 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0x58564 | `[sp-22]:1 != 0xff` | `[sp-22]:1 == 0xff` |
| `bne` at 0x58588 | `ret(0x5b130) != 0x0` | `ret(0x5b130) == 0x0` |
| `blt` at 0x585c0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x585fc | `ret(0x5c008) == 0x0` | `ret(0x5c008) != 0x0` |
| `bne` at 0x58608 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0x58654 | `ret(0x5b20c) != 0x0` | `ret(0x5b20c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x58694 | `r0` | `?` |

## `pcixSetBusSpeed`

- `pcixSetBusSpeed`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x59534 to 0x5964c.
  Established. The call to `0x5b95c` receives `[sp-20]` and three unknown candidates (`bl` at
  0x595f8). The first guard takes the continuation for `[0x148360] != 0x0` and falls through on zero
  (`bne` at 0x5955c). The loop predicate retains only `lt flags condition (operands unknown)` and
  its negation (`blt` at 0x595a8). Speed units, selection rules and the joined r0 are unestablished
  (`ldmdb` at 0x59640).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x59544 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x5954c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x59550 | load | 4 | `abs 0x59644` | `-` |
| `ldr` at 0x59554 | load | 4 | `abs 0x148360` | `-` |
| `str` at 0x59564 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x5956c | load | 4 | `abs 0x59648` | `-` |
| `ldr` at 0x59570 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59584 | load | 4 | `abs 0x14835c` | `-` |
| `str` at 0x5958c | store | 4 | `sp -0x18` | `?` |
| `str` at 0x59594 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x59598 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5959c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x595a0 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x595b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x595b4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x595bc | load | 4 | `[sp-24] +0x14` | `-` |
| `ldrb` at 0x595c4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x595d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x595d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x595e8 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x595f0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x59600 | store | 4 | `sp -0x20` | `ret(0x5b95c)` |
| `ldr` at 0x59604 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x59610 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5961c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x59620 | store | 4 | `sp -0x1c` | `ret(0x5b95c)` |
| `ldr` at 0x59624 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5962c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x59634 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x59638 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x5963c | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x59640 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x59640 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x59640 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x595f8 | `(unknown callee) 0x5b95c` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5955c | `[0x148360] != 0x0` | `[0x148360] == 0x0` |
| `blt` at 0x595a8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x595cc | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x5960c | `ret(0x5b95c) == 0x0` | `ret(0x5b95c) != 0x0` |
| `bne` at 0x59618 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x59640 | `r0` | `?` |

## `pcixBusPopulation`

- `pcixBusPopulation`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x597d4 to 0x59930.
  Established. The first call to `0x5b130` carries `[sp-20]`, `[sp-21]:1`, unknown, `[sp-21]:1`
  (`bl` at 0x59844); its nonzero result takes the continuation and zero falls through (`bne` at
  0x59858). The earlier global test takes its branch for `[0x148360] != 0x0` and falls through on
  zero (`bne` at 0x59804). Population units and the final result remain unidentified (`ldmdb` at
  0x59924).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x597e4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x597ec | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `str` at 0x597f4 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x597f8 | load | 4 | `abs 0x59928` | `-` |
| `ldr` at 0x597fc | load | 4 | `abs 0x148360` | `-` |
| `str` at 0x5980c | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x59814 | load | 4 | `abs 0x5992c` | `-` |
| `ldr` at 0x59818 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5982c | load | 4 | `abs 0x14835c` | `-` |
| `str` at 0x59834 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x59838 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5983c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5984c | store | 4 | `sp -0x20` | `ret(0x5b130)` |
| `ldr` at 0x59850 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x59860 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x59868 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x5986c | load | 1 | `ret(0x5b130) +0x1` | `-` |
| `str` at 0x59870 | store | 4 | `sp -0x34` | `[ret(0x5b130)+1]:1` |
| `ldr` at 0x59874 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x59878 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5987c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x59880 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x59888 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x59898 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5989c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x598a4 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x598b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x598c0 | store | 4 | `sp -0x30` | `ret(0x5b754)` |
| `ldr` at 0x598c4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x598d0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x598d4 | store | 4 | `sp -0x38` | `[sp-44]` |
| `ldrb` at 0x598e8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x598fc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x59904 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x59908 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x59910 | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `ldr` at 0x59918 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x5991c | store | 4 | `sp -0x38` | `[sp-44]` |
| `ldr` at 0x59920 | load | 4 | `sp -0x38` | `-` |
| `ldmdb` at 0x59924 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x59924 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x59924 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x59844 | `(unknown callee) 0x5b130` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x598b8 | `(unknown callee) 0x5b754` | `[sp-20]` | `?` | `sp-40` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x59804 | `[0x148360] != 0x0` | `[0x148360] == 0x0` |
| `bne` at 0x59858 | `ret(0x5b130) != 0x0` | `ret(0x5b130) == 0x0` |
| `blt` at 0x59890 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x598cc | `ret(0x5b754) == 0x0` | `ret(0x5b754) != 0x0` |
| `beq` at 0x598f8 | `? == 0x3` | `? != 0x3` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x59924 | `r0` | `?` |

## `pcixBusPresent`

- `pcixBusPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x59a9c to 0x59b18.
  Established. The byte store at an unknown effective address writes `0x1` (`strb` at 0x59b08). The
  global guard takes the continuation for `[0x148360] != 0x0` and falls through for zero (`bne` at
  0x59ac4). Entry r0 remains the exit expression (`ldmdb` at 0x59b0c). Bus-presence propagation is
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x59aac | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x59ab4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x59ab8 | load | 4 | `abs 0x59b10` | `-` |
| `ldr` at 0x59abc | load | 4 | `abs 0x148360` | `-` |
| `ldr` at 0x59acc | load | 4 | `abs 0x59b14` | `-` |
| `ldr` at 0x59ad0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59ae4 | load | 4 | `abs 0x14835c` | `-` |
| `str` at 0x59aec | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x59af0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x59af4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x59afc | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x59b08 | store | 1 | `(unknown)` | `0x1` |
| `ldmdb` at 0x59b0c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x59b0c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x59b0c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x59ac4 | `[0x148360] != 0x0` | `[0x148360] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x59b0c | `r0` | `r0` |

## `pcixBusNotPresent`

- `pcixBusNotPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x59b18 to 0x59b94.
  Established. A byte store clears an unknown effective address (`strb` at 0x59b84). The initial
  test takes its continuation for `[0x148360] != 0x0` and falls through on zero (`bne` at 0x59b40).
  R0 exits with its entry value (`ldmdb` at 0x59b88). The hardware absence indication remains
  unestablished.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x59b28 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x59b30 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x59b34 | load | 4 | `abs 0x59b8c` | `-` |
| `ldr` at 0x59b38 | load | 4 | `abs 0x148360` | `-` |
| `ldr` at 0x59b48 | load | 4 | `abs 0x59b90` | `-` |
| `ldr` at 0x59b4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59b60 | load | 4 | `abs 0x14835c` | `-` |
| `str` at 0x59b68 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x59b6c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x59b70 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x59b78 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x59b84 | store | 1 | `(unknown)` | `0x0` |
| `ldmdb` at 0x59b88 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x59b88 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x59b88 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x59b40 | `[0x148360] != 0x0` | `[0x148360] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x59b88 | `r0` | `r0` |

## `pcixBusIsPresent`

- `pcixBusIsPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x59b94 to 0x59c4c.
  Established. The global test takes its continuation for `[0x148360] != 0x0` and falls through on
  zero (`bne` at 0x59be0). No calls are reported, and the common exit has unknown r0 (`ldmdb` at
  0x59c40). Physical bus-presence meaning remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x59ba4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x59bac | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x59bb0 | load | 4 | `abs 0x59c44` | `-` |
| `ldr` at 0x59bb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59bc8 | load | 4 | `abs 0x14835c` | `-` |
| `str` at 0x59bd0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x59bd4 | load | 4 | `abs 0x59c48` | `-` |
| `ldr` at 0x59bd8 | load | 4 | `abs 0x148360` | `-` |
| `str` at 0x59be8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x59bf0 | load | 4 | `abs 0x59c44` | `-` |
| `ldr` at 0x59bf4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59c08 | load | 4 | `abs 0x14835c` | `-` |
| `str` at 0x59c10 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x59c14 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x59c18 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x59c20 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0x59c28 | load | 1 | `(unknown)` | `-` |
| `str` at 0x59c38 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x59c3c | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x59c40 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x59c40 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x59c40 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x59be0 | `[0x148360] != 0x0` | `[0x148360] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x59c40 | `r0` | `?` |
