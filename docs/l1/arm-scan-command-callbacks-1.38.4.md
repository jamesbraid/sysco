# ARM scan commands and configuration callbacks in 1.38.4

Read from `6.5.28/usr/cpu/firmware/sysco/l1.bin`, package `6.5.28:l1#4` (version 1.38.4),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53cfa0`.
Carrier SHA-256: `e5b65e5f28d7fba6013a8618f55c34bed7f60f7c6f46d7e9c5b8b7b2eabbe7b3`.
Package SHA-256: `c6693d19c6eb8c3fcf00cbb84d6974d1b5b17b930fb5375c1df2df20894edec9`.
Inner executable SHA-256: `f652c04c77cd7c53ca7ae6d6d6f581f11200efa211ff089bfba0d8bd4f718eb8`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1254136 bytes.
Segment SHA-256: `a373bb774d119f083e52d394fbd2701ebfbf35210301a40dcdb4ebdc39bf6bdc`. Analysed routine spans use the exact
ELF function symbols and sizes. Span ends are exclusive. The spans can include trailing literal
words after a return.

`sp` denotes the entry stack pointer. Register expressions such as `r0` denote entry
values, and brackets denote loaded values. Decimal numbers inside expressions retain the
exact tool notation. Call candidates use the assumed ARM AAPCS
register locations r0 through r3 with unknown arity. Stack stores appear in the access tables.
Unknown values and predicates include unsupported instruction effects and merged paths.
Numeric call targets have unknown callee names in the flat-image analysis. Device roles
and pointee meanings remain unknown unless the body establishes them. Tables include all
explicit memory operands on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
No unresolved computed transfers were reported for these routines. Warnings about unsupported instruction effects remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/scan-command-callbacks-facts.txt) retain all tool sections.

## `JTAG_Cfg_pid`

- `JTAG_Cfg_pid`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe70a0 to 0xe70e0.
  Established. The word save at `sp -0x14` carries entry r0 (`str` at 0xe70b0). A later word load
  has unknown effective address, and the exit reports an unknown r0 (`ldr` at 0xe70d0, `ldmdb` at
  0xe70d8). No calls or conditional branches appear in the facts. Port-ID meaning and the indexed
  record layout are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe70b0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xe70b4 | load | 4 | `abs 0xe70dc` | `-` |
| `ldr` at 0xe70b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe70d0 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0xe70d8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe70d8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe70d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe70d8 | `r0` | `?` |

## `JTAG_Cfg_Setup`

- `JTAG_Cfg_Setup`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe77f4 to 0xe7848.
  Established. Three word stores write `0xe739c` at `0x142b4c`, `0xe6c10` at `0x142b50` and
  `0xe70e0` at `0x142b54`, respectively (`str` at 0xe7808, `str` at 0xe7814, `str` at 0xe7820). The
  return carries r0 `0x0` (`ldmdb` at 0xe782c). Neither calls nor conditional branches occur in the
  fact sections. The installed values alone do not prove a callback invocation or physical
  configuration change.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xe7800 | load | 4 | `abs 0xe7830` | `-` |
| `ldr` at 0xe7804 | load | 4 | `abs 0xe7834` | `-` |
| `str` at 0xe7808 | store | 4 | `abs 0x142b4c` | `0xe739c` |
| `ldr` at 0xe780c | load | 4 | `abs 0xe7838` | `-` |
| `ldr` at 0xe7810 | load | 4 | `abs 0xe783c` | `-` |
| `str` at 0xe7814 | store | 4 | `abs 0x142b50` | `0xe6c10` |
| `ldr` at 0xe7818 | load | 4 | `abs 0xe7840` | `-` |
| `ldr` at 0xe781c | load | 4 | `abs 0xe7844` | `-` |
| `str` at 0xe7820 | store | 4 | `abs 0x142b54` | `0xe70e0` |
| `ldmdb` at 0xe782c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe782c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe782c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe782c | `r0` | `0x0` |

## `JTAG_Comm_read`

- `JTAG_Comm_read`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe7848 to 0xe79e0.
  Established. The call to `0xeac50` has entry r0, `[sp-48]`, `sp-44`, `sp-44` (`bl` at 0xe78b0);
  its result takes the continuation on zero and falls through on nonzero (`beq` at 0xe78c4). Later
  byte-store sites have unknown values: one target is unknown and the other is `([sp-24] + [sp-32])
  +0x0` (`strb` at 0xe794c, `strb` at 0xe7990). Packet meaning and the common r0 result remain
  unresolved (`ldmdb` at 0xe79d4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe7858 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe785c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe7860 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xe7864 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe787c | store | 4 | `sp -0x30` | `0xc` |
| `str` at 0xe7888 | store | 4 | `sp -0x30` | `0x11` |
| `str` at 0xe7898 | store | 4 | `sp -0x3c` | `0x40` |
| `ldr` at 0xe789c | load | 4 | `abs 0xe79d8` | `-` |
| `str` at 0xe78a0 | store | 4 | `sp -0x38` | `0x135c68` |
| `ldr` at 0xe78a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe78a8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe78b8 | store | 4 | `sp -0x24` | `ret(0xeac50)` |
| `ldr` at 0xe78bc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe78c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe78d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe78e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe78e4 | load | 4 | `abs 0xe79dc` | `-` |
| `ldr` at 0xe78f0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe78f4 | store | 4 | `sp -0x34` | `?` |
| `str` at 0xe7914 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xe7918 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe792c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7930 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xe7948 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xe794c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe7968 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe7970 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe7974 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7978 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xe798c | load | 1 | `(unknown)` | `-` |
| `strb` at 0xe7990 | store | 1 | `([sp-24] + [sp-32]) +0x0` | `?` |
| `ldr` at 0xe79ac | load | 4 | `(unknown)` | `-` |
| `str` at 0xe79b4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe79b8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe79c0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `str` at 0xe79cc | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xe79d0 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xe79d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe79d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe79d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe78b0 | `(unknown callee) 0xeac50` | `r0` | `[sp-48]` | `sp-44` | `sp-44` |
| `bl` at 0xe78d8 | `(unknown callee) 0x21674` | `ret(0xeac50)` | `?` | `?` | `[sp-28]` |
| `bl` at 0xe78ec | `(unknown callee) 0x1b49c` | `[sp-28]` | `0x135c78` | `ret(0x21674)` | `ret(0x21674)` |
| `bl` at 0xe790c | `(unknown callee) 0xe235c` | `0x0` | `0x40` | `sp-44` | `sp-44` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe7874 | `? != 0x6` | `? == 0x6` |
| `beq` at 0xe78c4 | `ret(0xeac50) == 0x0` | `ret(0xeac50) != 0x0` |
| `beq` at 0xe78d0 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `ble` at 0xe7920 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe79d4 | `r0` | `?` |

## `JTAG_Comm_write`

- `JTAG_Comm_write`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe79e0 to 0xe7b74.
  Established. Two byte reads address `([sp-24] + [sp-32]) +0x4` and `([sp-24] + [sp-32]) +0x0`,
  respectively (`ldrb` at 0xe7a88, `ldrb` at 0xe7ac0). The later call to `0xeac50` carries
  `[sp-20]`, `[sp-48]`, `sp-44`, `0x0` (`bl` at 0xe7b10). A zero result takes the branch and nonzero
  falls through (`beq` at 0xe7b24). Wire format and the merged r0 return are unknown (`ldmdb` at
  0xe7b68).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe79f0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe79f4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe79f8 | store | 4 | `sp -0x1c` | `r2` |
| `stm` at 0xe7a10 | store | 4 | `(unknown)` | `0x0` |
| `stm` at 0xe7a10 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe7a14 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe7a2c | store | 4 | `sp -0x30` | `0xd` |
| `str` at 0xe7a38 | store | 4 | `sp -0x30` | `0x12` |
| `str` at 0xe7a40 | store | 4 | `sp -0x20` | `0x3` |
| `ldr` at 0xe7a44 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe7a6c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe7a78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7a7c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xe7a88 | load | 1 | `([sp-24] + [sp-32]) +0x4` | `-` |
| `str` at 0xe7a90 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe7aac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe7ab4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7ab8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xe7ac0 | load | 1 | `([sp-24] + [sp-32]) +0x0` | `-` |
| `str` at 0xe7ac8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe7acc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe7ad4 | store | 4 | `sp -0x20` | `[sp-32]-1` |
| `str` at 0xe7af8 | store | 4 | `sp -0x3c` | `0x40` |
| `ldr` at 0xe7afc | load | 4 | `abs 0xe7b6c` | `-` |
| `str` at 0xe7b00 | store | 4 | `sp -0x38` | `0x135ca4` |
| `ldr` at 0xe7b04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7b08 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe7b18 | store | 4 | `sp -0x24` | `ret(0xeac50)` |
| `ldr` at 0xe7b1c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe7b28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7b34 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe7b40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7b44 | load | 4 | `abs 0xe7b70` | `-` |
| `ldr` at 0xe7b50 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe7b54 | store | 4 | `sp -0x34` | `?` |
| `str` at 0xe7b60 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xe7b64 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xe7b68 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe7b68 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe7b68 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe7aec | `(unknown callee) 0xe235c` | `0x0` | `0x40` | `sp-44` | `sp-44` |
| `bl` at 0xe7b10 | `(unknown callee) 0xeac50` | `[sp-20]` | `[sp-48]` | `sp-44` | `0x0` |
| `bl` at 0xe7b38 | `(unknown callee) 0x21674` | `ret(0xeac50)` | `?` | `?` | `[sp-28]` |
| `bl` at 0xe7b4c | `(unknown callee) 0x1b49c` | `[sp-28]` | `0x135cb4` | `ret(0x21674)` | `ret(0x21674)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe7a24 | `? != 0x6` | `? == 0x6` |
| `bge` at 0xe7a4c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xe7b24 | `ret(0xeac50) == 0x0` | `ret(0xeac50) != 0x0` |
| `beq` at 0xe7b30 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe7b68 | `r0` | `?` |

## `JTAG_Cfg_L3SetCfg_stub`

- `JTAG_Cfg_L3SetCfg_stub`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe7b74 to 0xe7b94.
  Established. The explicit word save puts entry r0 at `sp -0x14` (`str` at 0xe7b84). The return
  supplies r0 `0x1` (`ldmdb` at 0xe7b90); its fact sections contain no calls or conditional
  branches. L3 configuration changes are not established in this stub.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe7b84 | store | 4 | `sp -0x14` | `r0` |
| `ldmdb` at 0xe7b90 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe7b90 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe7b90 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe7b90 | `r0` | `0x1` |

## `JTAG_Cfg_set_ports_stub`

- `JTAG_Cfg_set_ports_stub`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe7b94 to 0xe7bb8.
  Established. The two explicit stack-word saves retain entry r0 at `sp -0x14` and entry r1 at `sp
  -0x18` (`str` at 0xe7ba4, `str` at 0xe7ba8); its unbranched return has r0 `0x0` (`ldmdb` at
  0xe7bb4). No call or conditional branch is reported. Physical port selection is unproved here.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe7ba4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe7ba8 | store | 4 | `sp -0x18` | `r1` |
| `ldmdb` at 0xe7bb4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe7bb4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe7bb4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe7bb4 | `r0` | `0x0` |

## `JTAG_Cfg_display_stub`

- `JTAG_Cfg_display_stub`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe7bb8 to 0xe7bc8.
  Established. The body saves and restores its frame in the retained
  facts (`push` at 0xe7bbc, `ldmdb` at 0xe7bc4). At the exit, r0 retains its entry value (`ldmdb` at
  0xe7bc4). The call and conditional-branch sections are empty. Display output is not established by
  this body.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0xe7bc4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe7bc4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe7bc4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe7bc4 | `r0` | `r0` |

## `JChip_MMR_SC_stub`

- `JChip_MMR_SC_stub`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe7bc8 to 0xe7be8.
  Established. A word save places entry r0 at `sp -0x14` (`str` at 0xe7bd8); its exit sets r0 to
  `0x0` (`ldmdb` at 0xe7be4). The listing contains no call or conditional-branch facts. No MMR
  transaction is proved by this stub.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe7bd8 | store | 4 | `sp -0x14` | `r0` |
| `ldmdb` at 0xe7be4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe7be4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe7be4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe7be4 | `r0` | `0x0` |

## `JTAG_LLP_SC_stub`

- `JTAG_LLP_SC_stub`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe7be8 to 0xe7c08.
  Established. The explicit word store saves entry r0 at `sp -0x14` (`str` at 0xe7bf8). A
  unbranched return reports r0 `0x0` (`ldmdb` at 0xe7c04). There are no calls or conditional
  branches in the facts. Link-protocol activity remains unestablished in this body.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe7bf8 | store | 4 | `sp -0x14` | `r0` |
| `ldmdb` at 0xe7c04 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe7c04 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe7c04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe7c04 | `r0` | `0x0` |

## `JChip_link_summary_stub`

- `JChip_link_summary_stub`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe7c08 to 0xe7c58.
  Established. Between two calls to `0x20240`, a byte store writes `0x0` at `[sp-32] +0x0` (`bl` at
  0xe7c2c, `strb` at 0xe7c38, `bl` at 0xe7c40). The calls carry `0x135ce0`, entry r1, entry r2,
  entry r3 and `0x135ce4`, unknown, `[sp-32]`, `0x0`, respectively (`bl` at 0xe7c2c, `bl` at
  0xe7c40). The return reports r0 `0x0` (`ldmdb` at 0xe7c4c). Summary content and the numeric callee
  role are unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe7c18 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe7c1c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe7c20 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe7c24 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xe7c28 | load | 4 | `abs 0xe7c50` | `-` |
| `ldr` at 0xe7c30 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xe7c38 | store | 1 | `[sp-32] +0x0` | `0x0` |
| `ldr` at 0xe7c3c | load | 4 | `abs 0xe7c54` | `-` |
| `ldmdb` at 0xe7c4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe7c4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe7c4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe7c2c | `(unknown callee) 0x20240` | `0x135ce0` | `r1` | `r2` | `r3` |
| `bl` at 0xe7c40 | `(unknown callee) 0x20240` | `0x135ce4` | `?` | `[sp-32]` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe7c4c | `r0` | `0x0` |

## `smpcmd_scan_config`

- `smpcmd_scan_config`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe7f7c to 0xe8010.
  Established. The test takes the call arm for `[0x2ad258] != 0x0` and falls through on
  zero (`bne` at 0xe7fb4). That arm includes an unknown-target call with `ret(0x1b124)`, unknown,
  unknown, `[0x142b54]` (`mov` at 0xe7fe0). The common exit supplies r0 `0x0` (`ldmdb` at 0xe8000).
  Configuration display and hardware effects of the callback remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe7f8c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe7f90 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xe7f94 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe7fa4 | store | 4 | `r1 +0x0` | `ret(0x1b124)` |
| `ldr` at 0xe7fa8 | load | 4 | `abs 0xe8004` | `-` |
| `ldr` at 0xe7fac | load | 4 | `abs 0x2ad258` | `-` |
| `ldr` at 0xe7fb8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7fbc | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe7fc0 | load | 4 | `abs 0xe8008` | `-` |
| `str` at 0xe7fcc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe7fd4 | load | 4 | `abs 0xe800c` | `-` |
| `ldr` at 0xe7fd8 | load | 4 | `abs 0x142b54` | `-` |
| `ldr` at 0xe7fe4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7fe8 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe7fec | load | 4 | `abs 0xe8008` | `-` |
| `str` at 0xe7ff8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe7ffc | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xe8000 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe8000 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe8000 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe8000 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe7f9c | `(unknown callee) 0x1b124` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xe7fc4 | `(unknown callee) 0x1b508` | `[[sp-28]]` | `0x135ce8` | `?` | `[sp-28]` |
| `mov` at 0xe7fe0 | `(unknown callee) (unknown target)` | `ret(0x1b124)` | `?` | `?` | `[0x142b54]` |
| `bl` at 0xe7ff0 | `(unknown callee) 0x1b508` | `[[sp-28]]` | `0x135ce8` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe7fb4 | `[0x2ad258] != 0x0` | `[0x2ad258] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe8000 | `r0` | `0x0` |

## `smpcmd_scan_debug`

- `smpcmd_scan_debug`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe8628 to 0xe86cc.
  Established. The unsigned comparison takes its store arm for `[[sp-24]+40] <= 0x4` and falls
  through for greater (`bls` at 0xe867c). The word store writes `[[sp-24]+40]` at `0x2ad25c` (`str`
  at 0xe8698). The later call to `0x1b49c` has `[[sp-28]]`, `0x135f7c`, `[0x2ad25c]`, `[sp-28]`
  (`bl` at 0xe86b0); its exit reports r0 `0x0` (`ldmdb` at 0xe86bc). Debug-level policy and callee
  formatting semantics are unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe8638 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe863c | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xe8640 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe8650 | store | 4 | `r1 +0x0` | `ret(0x1b124)` |
| `ldr` at 0xe8654 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8658 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xe865c | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0xe8668 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe866c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe8670 | store | 4 | `sp -0x20` | `[[sp-24]+40]` |
| `ldr` at 0xe8674 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe8680 | load | 4 | `abs 0xe86c0` | `-` |
| `ldr` at 0xe8690 | load | 4 | `abs 0xe86c4` | `-` |
| `ldr` at 0xe8694 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe8698 | store | 4 | `abs 0x2ad25c` | `[[sp-24]+40]` |
| `ldr` at 0xe869c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe86a0 | load | 4 | `abs 0xe86c4` | `-` |
| `ldr` at 0xe86a4 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe86a8 | load | 4 | `abs 0xe86c8` | `-` |
| `ldr` at 0xe86ac | load | 4 | `abs 0x2ad25c` | `-` |
| `ldmdb` at 0xe86bc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe86bc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe86bc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe86bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe8648 | `(unknown callee) 0x1b124` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xe8688 | `(unknown callee) 0x23f2c` | `0x135f40` | `0x4` | `?` | `[[sp-24]+40]` |
| `bl` at 0xe86b0 | `(unknown callee) 0x1b49c` | `[[sp-28]]` | `0x135f7c` | `[0x2ad25c]` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe8664 | `[[[sp-24]+112]] != 0x1` | `[[[sp-24]+112]] == 0x1` |
| `bls` at 0xe867c | `[[sp-24]+40] <= 0x4 (unsigned, 32-bit)` | `[[sp-24]+40] > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe86bc | `r0` | `0x0` |

## `smpcmd_scan_set`

- `smpcmd_scan_set`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe86cc to 0xe87c4.
  Established. The selector branch takes its alternate arm for `[[[sp-24]+112]+4] == 0x0` and falls
  through on nonzero (`beq` at 0xe875c). Separate word stores at `sp -0x24` carry unknown values
  (`str` at 0xe8774, `str` at 0xe8794). The call to `0xe5bf4` receives `[sp-36]`, unknown, unknown,
  unknown (`bl` at 0xe879c). The return carries r0 `0x0` (`ldmdb` at 0xe87b8). Signal masks and
  physical pin effects are not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe86dc | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe86e0 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xe86e4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe86f4 | store | 4 | `r1 +0x0` | `ret(0x1b124)` |
| `ldr` at 0xe86f8 | load | 4 | `abs 0xe87bc` | `-` |
| `ldr` at 0xe86fc | load | 4 | `abs 0x2ad258` | `-` |
| `ldr` at 0xe8708 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe870c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe8710 | load | 4 | `abs 0xe87c0` | `-` |
| `str` at 0xe871c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe8724 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8728 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xe872c | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `str` at 0xe8740 | store | 4 | `sp -0x24` | `ret(0xe58a0)` |
| `ldr` at 0xe8744 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe874c | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xe8754 | load | 4 | `[[sp-24]+112] +0x4` | `-` |
| `ldr` at 0xe8760 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8764 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xe8768 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe876c | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `str` at 0xe8774 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe877c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8780 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xe8784 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0xe878c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe8794 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe8798 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe87a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe87a4 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xe87b0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe87b4 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xe87b8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe87b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe87b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe87b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe86ec | `(unknown callee) 0x1b124` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xe8714 | `(unknown callee) 0x1b2d4` | `[[sp-28]]` | `0x135ce8` | `?` | `[sp-28]` |
| `bl` at 0xe8738 | `(unknown callee) 0xe58a0` | `ret(0x1b124)` | `?` | `?` | `[[[sp-24]+112]]` |
| `bl` at 0xe879c | `(unknown callee) 0xe5bf4` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0xe87a8 | `(unknown callee) 0xe59c4` | `[[sp-28]]` | `?` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe8704 | `[0x2ad258] != 0x0` | `[0x2ad258] == 0x0` |
| `beq` at 0xe8734 | `[[[sp-24]+112]] == 0x0` | `[[[sp-24]+112]] != 0x0` |
| `beq` at 0xe875c | `[[[sp-24]+112]+4] == 0x0` | `[[[sp-24]+112]+4] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe87b8 | `r0` | `0x0` |

## `smpcmd_scan_imp`

- `smpcmd_scan_imp`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe87c4 to 0xe8818.
  Established. The opening call to `0x1b124` carries `0xa0`, entry r1, entry r2, entry r3 (`bl` at
  0xe87e4). The following call reaches `0x1b49c` with `[[sp-28]]`, `0x135f94`, unknown, `[sp-28]`
  (`bl` at 0xe8804). The exit reports r0 `0x0` (`ldmdb` at 0xe8810). No conditional branch appears
  in the facts. Impedance-test behavior is not demonstrated by these calls.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe87d4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe87d8 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xe87dc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe87ec | store | 4 | `r1 +0x0` | `ret(0x1b124)` |
| `str` at 0xe87f4 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe87f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe87fc | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe8800 | load | 4 | `abs 0xe8814` | `-` |
| `ldmdb` at 0xe8810 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe8810 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe8810 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe8810 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe87e4 | `(unknown callee) 0x1b124` | `0xa0` | `r1` | `r2` | `r3` |
| `bl` at 0xe8804 | `(unknown callee) 0x1b49c` | `[[sp-28]]` | `0x135f94` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe8810 | `r0` | `0x0` |

## `smpcmd_scan_test`

- `smpcmd_scan_test`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe8818 to 0xe886c.
  Established. The call to `0x1b124` has `0xa0`, entry r1, entry r2, entry r3 (`bl` at 0xe8838). A
  later call to `0x1b49c` uses `[[sp-28]]`, `0x135fcc`, unknown, `[sp-28]` (`bl` at 0xe8858); its
  unbranched exit supplies r0 `0x0` (`ldmdb` at 0xe8864). The body has no conditional-branch
  facts and establishes no physical scan-test effect.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe8828 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe882c | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xe8830 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe8840 | store | 4 | `r1 +0x0` | `ret(0x1b124)` |
| `str` at 0xe8848 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xe884c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8850 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe8854 | load | 4 | `abs 0xe8868` | `-` |
| `ldmdb` at 0xe8864 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe8864 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe8864 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe8864 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe8838 | `(unknown callee) 0x1b124` | `0xa0` | `r1` | `r2` | `r3` |
| `bl` at 0xe8858 | `(unknown callee) 0x1b49c` | `[[sp-28]]` | `0x135fcc` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe8864 | `r0` | `0x0` |
