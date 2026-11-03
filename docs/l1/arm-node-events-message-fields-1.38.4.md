# ARM node events and message fields in 1.38.4

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
Accesses and return values cover known control-flow edges. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/node-events-message-fields-facts.txt) retain all tool sections.

## `IP65_NodeRemove`

- `IP65_NodeRemove`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x63e38 to 0x63f4c.
  Established. The call to `0x64978` carries `(r0 & 0xff)`, `sp-24`, `sp-24` and `(r0 & 0xff)` (`bl`
  at 0x63e60); its zero result takes the branch and nonzero falls through (`beq` at 0x63e74). The
  later call to `0x64054` passes `[sp-17]:1`, `0x1`, `0x1` and `[sp-17]:1` (`bl` at 0x63e94). The
  removal mechanism remains unknown beyond the listed effects.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x63e4c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x63e50 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x63e68 | store | 4 | `sp -0x1c` | `ret(0x64978)` |
| `ldr` at 0x63e6c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x63e78 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x63e7c | store | 4 | `sp -0x24` | `ret(0x64978)` |
| `ldrb` at 0x63e84 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x63e98 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x63e9c | load | 1 | `[sp-24] +0x2` | `-` |
| `strb` at 0x63ea0 | store | 1 | `sp -0x1d` | `([[sp-24]+2]:1 & 0xff)` |
| `ldrb` at 0x63ea4 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x63ea8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x63eac | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0x63ec0 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x63ed0 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x63ed8 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldr` at 0x63ee0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x63ee4 | load | 1 | `[sp-24] +0x0` | `-` |
| `strb` at 0x63ee8 | store | 1 | `sp -0x1d` | `([[sp-24]]:1 & 0xff)` |
| `ldrb` at 0x63eec | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x63ef0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x63ef4 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldrb` at 0x63f08 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x63f18 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x63f20 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldr` at 0x63f28 | load | 4 | `abs 0x63f48` | `-` |
| `ldrb` at 0x63f2c | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x63f34 | store | 1 | `[sp-17]:1 +0x2b0948` | `0xf` |
| `str` at 0x63f3c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x63f40 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x63f44 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x63f44 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x63f44 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x63e60 | `(unknown callee) 0x64978` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x63e94 | `(unknown callee) 0x64054` | `[sp-17]:1` | `0x1` | `0x1` | `[sp-17]:1` |
| `bl` at 0x63ecc | `(unknown callee) 0xd1820` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x63f14 | `(unknown callee) 0xd47a8` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x63e74 | `ret(0x64978) == 0x0` | `ret(0x64978) != 0x0` |
| `blt` at 0x63eb8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x63f00 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x63f44 | `r0` | `?` |

## `IP65_NodeDisable`

- `IP65_NodeDisable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x63f4c to 0x63f84.
  Established. A byte store saves `(r0 & 0xff)` at `sp -0x11` (`strb` at 0x63f60). The only call
  targets `0x64054` with `(r0 & 0xff)`, `0x0`, `0x1` and `(r0 & 0xff)` (`bl` at 0x63f74). The exit
  carries `ret(0x64054)` (`ldmdb` at 0x63f80). What these constants disable remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x63f60 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x63f64 | load | 1 | `sp -0x11` | `-` |
| `ldmdb` at 0x63f80 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x63f80 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x63f80 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x63f74 | `(unknown callee) 0x64054` | `(r0 & 0xff)` | `0x0` | `0x1` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x63f80 | `r0` | `ret(0x64054)` |

## `IP65_NodeEnable`

- `IP65_NodeEnable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x63f84 to 0x63fb8.
  Established. The call to `0x64218` receives `(r0 & 0xff)`, `0x0`, entry r2 and `(r0 & 0xff)` (`bl`
  at 0x63fa8); its result reaches the exit as `ret(0x64218)` (`ldmdb` at 0x63fb4). This establishes
  forwarding of the result, while the external enable effect remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x63f98 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x63f9c | load | 1 | `sp -0x11` | `-` |
| `ldmdb` at 0x63fb4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x63fb4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x63fb4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x63fa8 | `(unknown callee) 0x64218` | `(r0 & 0xff)` | `0x0` | `r2` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x63fb4 | `r0` | `ret(0x64218)` |

## `IP65_NodePresent`

- `IP65_NodePresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x64438 to 0x64474.
  Established. A byte read uses `(r0 & 0xff) +0x2b0948` (`ldrb` at 0x64458). The two conditional
  moves are retained in the listing, but the recorded exit value is unknown (`moveq` at 0x64460,
  `movne` at 0x64464, `ldmdb` at 0x6446c). A physical presence interpretation remains unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x6444c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x64450 | load | 4 | `abs 0x64470` | `-` |
| `ldrb` at 0x64454 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x64458 | load | 1 | `(r0 & 0xff) +0x2b0948` | `-` |
| `ldmdb` at 0x6446c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6446c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6446c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6446c | `r0` | `?` |

## `L1CfgEventSystemConsole`

- `L1CfgEventSystemConsole`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x74f04 to 0x74f24.
  Established. The call to `0x74c78` has candidates `0x1`, `0x0`, `0x0` and entry r3 (`bl` at
  0x74f1c). The return records `ret(0x74c78)` (`ldmdb` at 0x74f20). The body does not establish the
  event encoding or a system-console hardware effect, so both remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x74f20 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x74f20 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x74f20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x74f1c | `(unknown callee) 0x74c78` | `0x1` | `0x0` | `0x0` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x74f20 | `r0` | `ret(0x74c78)` |

## `L1CfgEventConsoleSubchannel`

- `L1CfgEventConsoleSubchannel`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x74f24 to 0x74f44.
  Established. Candidate r0 is `0x2` at the call to `0x74c78`, followed by `0x0`, `0x0` and entry r3
  (`bl` at 0x74f3c); its returned expression is `ret(0x74c78)` (`ldmdb` at 0x74f40). The meaning of
  the selector and subchannel remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x74f40 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x74f40 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x74f40 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x74f3c | `(unknown callee) 0x74c78` | `0x2` | `0x0` | `0x0` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x74f40 | `r0` | `ret(0x74c78)` |

## `L1CfgEventConsoleEnable`

- `L1CfgEventConsoleEnable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x74f44 to 0x74f64.
  Established. The wrapper calls `0x74c78` with `0x3`, `0x0`, `0x0` and entry r3 (`bl` at 0x74f5c),
  then retains `ret(0x74c78)` at the exit (`ldmdb` at 0x74f60). Console-enable polarity is unknown
  because the callee's effect is outside this body.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x74f60 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x74f60 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x74f60 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x74f5c | `(unknown callee) 0x74c78` | `0x3` | `0x0` | `0x0` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x74f60 | `r0` | `ret(0x74c78)` |

## `IcmbCvtToMsg`

- `IcmbCvtToMsg`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xae6c4 to 0xae708.
  Established. A halfword store saves `(r1 & 0xffff)` at `sp -0x16` (`strh` at 0xae6dc). The first
  byte store writes `([sp-22]:1 & 0xff)` at `r0 +0x0`. The other writes an unknown byte at `[sp-20]
  +0x1` (`strb` at 0xae6e8, `strb` at 0xae700). Entry r0 is the recorded exit value (`ldmdb` at
  0xae704). The encoded message value remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xae6d4 | store | 4 | `sp -0x14` | `r0` |
| `strh` at 0xae6dc | store | 2 | `sp -0x16` | `(r1 & 0xffff)` |
| `ldr` at 0xae6e0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xae6e4 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0xae6e8 | store | 1 | `r0 +0x0` | `([sp-22]:1 & 0xff)` |
| `ldr` at 0xae6f0 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xae6f8 | load | 2 | `sp -0x16` | `-` |
| `strb` at 0xae700 | store | 1 | `[sp-20] +0x1` | `?` |
| `ldmdb` at 0xae704 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xae704 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xae704 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xae704 | `r0` | `r0` |

## `IcmbMsgGetMsgType`

- `IcmbMsgGetMsgType`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xae708 to 0xae740.
  Established. The byte read targets `r0 +0x0` (`ldrb` at 0xae720). A word store at `sp -0x18` has
  an unknown value, and the exit carries `[sp-24]` (`str` at 0xae730, `ldmdb` at 0xae73c). The
  message-type interpretation is unknown beyond this recorded data flow.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xae718 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xae71c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xae720 | load | 1 | `r0 +0x0` | `-` |
| `str` at 0xae730 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xae734 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xae73c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xae73c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xae73c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xae73c | `r0` | `[sp-24]` |

## `IcmbMsgGetSeqNbr`

- `IcmbMsgGetSeqNbr`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xae794 to 0xae7c4.
  Established. The input byte is read at `r0 +0x0` (`ldrb` at 0xae7ac). A subsequent byte store has
  an unknown value at `sp -0x15` (`strb` at 0xae7b4); its stored byte is the exit expression
  `[sp-21]:1` (`ldmdb` at 0xae7c0). The sequence-number encoding cannot be recovered from the
  supported facts and remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xae7a4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xae7a8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xae7ac | load | 1 | `r0 +0x0` | `-` |
| `strb` at 0xae7b4 | store | 1 | `sp -0x15` | `?` |
| `ldrb` at 0xae7b8 | load | 1 | `sp -0x15` | `-` |
| `ldmdb` at 0xae7c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xae7c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xae7c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xae7c0 | `r0` | `[sp-21]:1` |

## `IcmbMsgGetCmd`

- `IcmbMsgGetCmd`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xae810 to 0xae844.
  Established. The byte at `r0 +0x1` is loaded and saved as a word at `sp -0x18` (`ldrb` at 0xae830,
  `str` at 0xae834). The exit carries `[r0+1]:1` (`ldmdb` at 0xae840). Command semantics are
  unknown, although this byte extraction is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xae820 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xae828 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xae830 | load | 1 | `r0 +0x1` | `-` |
| `str` at 0xae834 | store | 4 | `sp -0x18` | `[r0+1]:1` |
| `ldr` at 0xae838 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xae840 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xae840 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xae840 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xae840 | `r0` | `[r0+1]:1` |

## `IcmbMsgGetCmpCode`

- `IcmbMsgGetCmpCode`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xae874 to 0xae8a8.
  Established. A byte load reads `r0 +0x2` and a word store records `[r0+2]:1` at `sp -0x18` (`ldrb`
  at 0xae894, `str` at 0xae898). The exit retains `[r0+2]:1` (`ldmdb` at 0xae8a4). The byte's
  completion-code meaning remains unknown in this isolated body.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xae884 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xae88c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xae894 | load | 1 | `r0 +0x2` | `-` |
| `str` at 0xae898 | store | 4 | `sp -0x18` | `[r0+2]:1` |
| `ldr` at 0xae89c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xae8a4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xae8a4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xae8a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xae8a4 | `r0` | `[r0+2]:1` |

## `IcmbMsgGetData`

- `IcmbMsgGetData`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xae8a8 to 0xae900.
  Established. The call to `0xae708` uses entry r0 through r3 (`bl` at 0xae8c0). A nonzero result
  takes the branch, while zero falls through (`bne` at 0xae8d4). The fall-through store writes
  `[sp-20]+2` at `sp -0x1c`, and the taken arm writes `[sp-20]+3` there (`str` at 0xae8e0, `str` at
  0xae8f0). The message layout represented by these selected values remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xae8b8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xae8bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xae8c8 | store | 4 | `sp -0x18` | `ret(0xae708)` |
| `ldr` at 0xae8cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae8d8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xae8e0 | store | 4 | `sp -0x1c` | `[sp-20]+2` |
| `ldr` at 0xae8e8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xae8f0 | store | 4 | `sp -0x1c` | `[sp-20]+3` |
| `ldr` at 0xae8f4 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xae8fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xae8fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xae8fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xae8c0 | `(unknown callee) 0xae708` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xae8d4 | `ret(0xae708) != 0x0` | `ret(0xae708) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xae8fc | `r0` | `[sp-28]` |

## `IcmbMsgRqBuild`

- `IcmbMsgRqBuild`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xae900 to 0xae97c.
  Established. A word store saves `r3+2` at `sp -0x24` (`str` at 0xae92c). The first call to
  `0xae740` receives `[sp]`, `0x0`, entry r2 and `r3+2` (`bl` at 0xae938). The last call targets
  `0x1b304` with `[sp]+2`, `[sp-28]`, `[sp-32]` and `[sp]+2` (`bl` at 0xae96c). The request format
  and callee names remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xae910 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xae914 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xae918 | store | 4 | `sp -0x20` | `r3` |
| `strb` at 0xae920 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xae924 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xae92c | store | 4 | `sp -0x24` | `r3+2` |
| `ldr` at 0xae930 | load | 4 | `sp +0x0` | `-` |
| `ldrb` at 0xae93c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xae940 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xae94c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xae950 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae958 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xae964 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xae968 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xae970 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xae978 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xae978 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xae978 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xae938 | `(unknown callee) 0xae740` | `[sp]` | `0x0` | `r2` | `r3+2` |
| `bl` at 0xae948 | `(unknown callee) 0xae7c4` | `[sp]` | `[sp-17]:1` | `?` | `[sp-17]:1` |
| `bl` at 0xae954 | `(unknown callee) 0xae844` | `[sp]` | `[sp-24]` | `?` | `?` |
| `bl` at 0xae96c | `(unknown callee) 0x1b304` | `[sp]+2` | `[sp-28]` | `[sp-32]` | `[sp]+2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xae978 | `r0` | `[sp-36]` |

## `ioExpName`

- `ioExpName`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd7558 to 0xd75c0.
  Established. The call to `0xd75c0` has candidates entry r0, `(r1 & 0xff)`, entry r2 and `(r1 &
  0xff)` (`bl` at 0xd7580). The word at `ret(0xd75c0) +0xc` is loaded twice (`ldr` at 0xd7590, `ldr`
  at 0xd759c); its nonzero test takes the branch, while zero falls through to the word store of
  `0x1349c8` at `sp -0x20` (`bne` at 0xd75a4, `str` at 0xd75ac). The pointed-to name and merged
  return value are unknown (`ldmdb` at 0xd75b8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd7568 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd7570 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xd7574 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd7578 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd7588 | store | 4 | `sp -0x1c` | `ret(0xd75c0)` |
| `ldr` at 0xd758c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd7590 | load | 4 | `ret(0xd75c0) +0xc` | `-` |
| `str` at 0xd7594 | store | 4 | `sp -0x20` | `[ret(0xd75c0)+12]` |
| `ldr` at 0xd7598 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd759c | load | 4 | `ret(0xd75c0) +0xc` | `-` |
| `ldr` at 0xd75a8 | load | 4 | `abs 0xd75bc` | `-` |
| `str` at 0xd75ac | store | 4 | `sp -0x20` | `0x1349c8` |
| `ldr` at 0xd75b0 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xd75b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd75b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd75b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd7580 | `(unknown callee) 0xd75c0` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd75a4 | `[ret(0xd75c0)+12] != 0x0` | `[ret(0xd75c0)+12] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd75b8 | `r0` | `?` |
