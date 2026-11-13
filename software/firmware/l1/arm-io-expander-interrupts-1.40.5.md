# ARM I/O-expander callers in 1.40.5

Read from `6.5.29/usr/cpu/firmware/sysco/l1.bin`, package `6.5.29:l1#4` (version 1.40.5),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53d448`.
Carrier SHA-256: `80f4a616601d187bc3839a2758f2e7f2d0db962ad7fd8f2d96d41bcd15bdf88c`.
Package SHA-256: `4ccd1a6e7054bc3cd56e962a60737d1c90339ffa93fcc8af0912b6c9c7d385dd`.
Inner executable SHA-256: `08ebc648bb298017c7e3d6049983f2a54a8fe56f1bb5738df88b82c38f75ac3a`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1257088 bytes.
Segment SHA-256: `dc7cc5d18fbdf816d328411825c9c01e26c3f8852f1012c7a642aea0767e2665`. Analysed routine spans use the exact
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
`ioExpInt`, `ioExpWriteExp` and `ioExpReadExp` have unresolved computed transfers
(`ldrls` at 0xd780c, `ldrls` at 0xd8360, `ldrls` at 0xd861c). Excluded destinations
supply no known-edge arguments or effects. Undecodable byte ranges remain visible as
fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/io-expander-interrupts-facts.txt) retain all tool sections.

An undecodable word lies at `0xd75bc` through exclusive `0xd75c0` inside the
`ioExpRead` ELF span; its raw bytes `11 fc ff ff` are retained after the fact sheets.
No error-code meaning is assigned to that word. Three conditional computed transfers
remain unresolved, so selector-only effects are excluded from the entry tables.

## `ioExpRead`

- `ioExpRead`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd7530 to 0xd75c0.
  Established. Two unnamed call sites target `0xd7f54` and `0xd8068`, with their results stored at
  `sp -0x20` and `sp -0x24` (`bl` at 0xd755c, `bl` at 0xd7574, `str` at 0xd7564, `str` at 0xd757c).
  A byte read at `ret(0xd8068) +0x0` controls a nonzero branch (`ldrb` at 0xd7584, `bne` at
  0xd758c); its fall-through stores four-byte `0xfffffc11` at `sp -0x28` (`str` at 0xd7594). The
  other arm calls `0xd85f4` with candidates `[sp-20]`, `[sp-32]`, `[sp-28]`, `[ret(0xd8068)]:1`
  (`bl` at 0xd75a8). Expander identity and error semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd7540 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd7548 | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0xd754c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xd7550 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd7554 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd7564 | store | 4 | `sp -0x20` | `ret(0xd7f54)` |
| `ldrb` at 0xd7568 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd756c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd757c | store | 4 | `sp -0x24` | `ret(0xd8068)` |
| `ldr` at 0xd7580 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xd7584 | load | 1 | `ret(0xd8068) +0x0` | `-` |
| `ldr` at 0xd7590 | load | 4 | `abs 0xd75bc` | `-` |
| `str` at 0xd7594 | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0xd759c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd75a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd75a4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd75b0 | store | 4 | `sp -0x28` | `ret(0xd85f4)` |
| `ldr` at 0xd75b4 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xd75b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd75b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd75b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd755c | `(unknown callee) 0xd7f54` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xd7574 | `(unknown callee) 0xd8068` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0xd75a8 | `(unknown callee) 0xd85f4` | `[sp-20]` | `[sp-32]` | `[sp-28]` | `[ret(0xd8068)]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd758c | `[ret(0xd8068)]:1 != 0x0` | `[ret(0xd8068)]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd75b8 | `r0` | `?` |

## `ioExpWriteValue`

- `ioExpWriteValue`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd75c0 to 0xd7604.
  Established. The unnamed call to `0xd8068` has candidates `r0`, `(r1 & 0xff)`, `r2`, `(r1 & 0xff)`
  (`bl` at 0xd75e8). A subsequent byte load reads `ret(0xd8068) +0x1` (`ldrb` at 0xd75f8). The exit
  retains `[ret(0xd8068)+1]:1` (`ldmdb` at 0xd7600). No conditional branch is reported. The
  byte-field role is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd75d0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd75d8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xd75dc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd75e0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd75f0 | store | 4 | `sp -0x1c` | `ret(0xd8068)` |
| `ldr` at 0xd75f4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd75f8 | load | 1 | `ret(0xd8068) +0x1` | `-` |
| `ldmdb` at 0xd7600 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd7600 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd7600 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd75e8 | `(unknown callee) 0xd8068` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd7600 | `r0` | `[ret(0xd8068)+1]:1` |

## `ioExpIntCount`

- `ioExpIntCount`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd7604 to 0xd7648.
  Established. The four-byte local store preserves `ret(0xd8068)` at `sp -0x1c` (`str` at 0xd7634).
  It follows an unnamed call with candidates `r0`, `(r1 & 0xff)`, `r2`, `(r1 & 0xff)` (`bl` at
  0xd762c). A four-byte load at `ret(0xd8068) +0x8` supplies the return expression
  `[ret(0xd8068)+8]` (`ldr` at 0xd763c, `ldmdb` at 0xd7644). A counter interpretation is not
  established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd7614 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd761c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xd7620 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd7624 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd7634 | store | 4 | `sp -0x1c` | `ret(0xd8068)` |
| `ldr` at 0xd7638 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd763c | load | 4 | `ret(0xd8068) +0x8` | `-` |
| `ldmdb` at 0xd7644 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd7644 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd7644 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd762c | `(unknown callee) 0xd8068` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd7644 | `r0` | `[ret(0xd8068)+8]` |

## `ioExpIntMask`

- `ioExpIntMask`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd7648 to 0xd768c.
  Established. The unnamed call to `0xd8068` carries `r0`, `(r1 & 0xff)`, `r2`, `(r1 & 0xff)` (`bl`
  at 0xd7670). The later byte read uses `ret(0xd8068) +0x3` (`ldrb` at 0xd7680), retained as
  `[ret(0xd8068)+3]:1` at the exit (`ldmdb` at 0xd7688). There is no condition. Interrupt-mask
  meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd7658 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd7660 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xd7664 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd7668 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd7678 | store | 4 | `sp -0x1c` | `ret(0xd8068)` |
| `ldr` at 0xd767c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd7680 | load | 1 | `ret(0xd8068) +0x3` | `-` |
| `ldmdb` at 0xd7688 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd7688 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd7688 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd7670 | `(unknown callee) 0xd8068` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd7688 | `r0` | `[ret(0xd8068)+3]:1` |

## `ioExpIntDisable`

- `ioExpIntDisable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd768c to 0xd76d4.
  Established. Byte stores preserve `(r1 & 0xff)` and `(r2 & 0xff)` at `sp -0x15` and `sp -0x16`
  (`strb` at 0xd76a4, `strb` at 0xd76ac). The single unnamed call to `0xd8800` has candidates `r0`,
  `(r1 & 0xff)`, `(r2 & 0xff)`, `0x0` (`bl` at 0xd76c4); its result reaches the return (`ldmdb` at
  0xd76d0). No interrupt-disable action is proved within this caller.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd769c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd76a4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xd76ac | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xd76b0 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xd76b4 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xd76b8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd76d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd76d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd76d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd76c4 | `(unknown callee) 0xd8800` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd76d0 | `r0` | `ret(0xd8800)` |

## `ioExpIntEnable`

- `ioExpIntEnable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd76d4 to 0xd771c.
  Established. The entry stores four-byte `r0` at `sp -0x14` and the low bytes of `r1` and `r2` at
  `sp -0x15`, `sp -0x16` (`str` at 0xd76e4, `strb` at 0xd76ec, `strb` at 0xd76f4); its unnamed call
  to `0xd8800` carries `r0`, `(r1 & 0xff)`, `(r2 & 0xff)`, `0x1` (`bl` at 0xd770c). The exit
  expression is `ret(0xd8800)` (`ldmdb` at 0xd7718). Enable-command semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd76e4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd76ec | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xd76f4 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xd76f8 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xd76fc | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xd7700 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd7718 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd7718 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd7718 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd770c | `(unknown callee) 0xd8800` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd7718 | `r0` | `ret(0xd8800)` |

## `ioExpIntSet`

- `ioExpIntSet`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd771c to 0xd7768.
  Established. The entry preserves four-byte `r3` at `sp -0x1c` (`str` at 0xd7730). The unnamed call
  to `0xd8800` has candidates `r0`, `(r1 & 0xff)`, `(r2 & 0xff)`, `r3` (`bl` at 0xd7758). The return
  retains its result expression (`ldmdb` at 0xd7764). This body establishes caller values, while the
  callee's command effect remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd772c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd7730 | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0xd7738 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xd7740 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xd7744 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xd7748 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xd774c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd7754 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xd7764 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd7764 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd7764 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd7758 | `(unknown callee) 0xd8800` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd7764 | `r0` | `ret(0xd8800)` |

## `ioExpInt`

- `ioExpInt`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd7768 to 0xd7918.
  Established. The entry stores four-byte zero at `sp -0x28` and byte `((r1 & 0xff) & 0xff)` at `sp
  -0x21` (`str` at 0xd7790, `strb` at 0xd7798). The computed transfer is taken when `[[sp-28]]:1 <=
  0x4 (unsigned, 32-bit)` and falls through when it is greater (`ldrls` at 0xd780c); its
  destinations remain unresolved. The visible default arm includes an unnamed call to `0x21eb8` with
  candidates `0x1`, `0x13525c`, `0x2d3`, `0x0` (`bl` at 0xd78ec). Interrupt-type meanings and
  selector-only effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd7778 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd7780 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xd7788 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0xd7790 | store | 4 | `sp -0x28` | `0x0` |
| `ldrb` at 0xd7794 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0xd7798 | store | 1 | `sp -0x21` | `((r1 & 0xff) & 0xff)` |
| `ldrb` at 0xd779c | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0xd77a0 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0xd77b0 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xd77b4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd77c4 | store | 4 | `sp -0x1c` | `ret(0xd7f54)` |
| `ldrb` at 0xd77c8 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xd77cc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd77dc | store | 4 | `sp -0x20` | `ret(0xd8068)` |
| `ldr` at 0xd77e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd77e4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xd77f0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd77f4 | load | 1 | `ret(0xd8068) +0x0` | `-` |
| `ldr` at 0xd7800 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd7804 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0xd780c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd78c8 | load | 4 | `abs 0xd790c` | `-` |
| `str` at 0xd78cc | store | 4 | `sp -0x30` | `0x1353f8` |
| `ldr` at 0xd78d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd78d4 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xd78d8 | store | 4 | `sp -0x2c` | `[[sp-28]+12]` |
| `ldr` at 0xd78e0 | load | 4 | `abs 0xd7910` | `-` |
| `ldr` at 0xd78e4 | load | 4 | `abs 0xd7914` | `-` |
| `ldrb` at 0xd78f0 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0xd78f8 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldr` at 0xd7900 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xd7908 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd7908 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd7908 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd77bc | `(unknown callee) 0xd7f54` | `?` | `?` | `?` | `?` |
| `bl` at 0xd77d4 | `(unknown callee) 0xd8068` | `[sp-20]` | `[sp-33]:1` | `?` | `[sp-33]:1` |
| `bl` at 0xd78ec | `(unknown callee) 0x21eb8` | `0x1` | `0x13525c` | `0x2d3` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xd77a8 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0xd77ec | `[[sp-28]+8] == 0x0` | `[[sp-28]+8] != 0x0` |
| `beq` at 0xd77fc | `[ret(0xd8068)]:1 == 0x0` | `[ret(0xd8068)]:1 != 0x0` |
| `ldrls` at 0xd780c | `[[sp-28]]:1 <= 0x4 (unsigned, 32-bit)` | `[[sp-28]]:1 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd7908 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xd780c).

## `ioExpPortInt`

- `ioExpPortInt`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd7918 to 0xd7afc.
  Established. Four byte stores preserve the low bytes of `r1`, `r2`, `r3` and `[sp]` at `sp -0x15`,
  `-0x16`, `-0x17`, `-0x18` (`strb` at 0xd7938, `strb` at 0xd7940, `strb` at 0xd7948, `strb` at
  0xd7950). Three branches take their target for unequal byte values (`bne` at 0xd79d8,
  `bne` at 0xd79ec, `bne` at 0xd7a00). The unnamed call to `0xd7afc` has candidates `[sp-20]`,
  `[sp-28]`, `ret(0xd8068)`, `[sp-33]:1` (`bl` at 0xd7a1c). Port identities and callback effects
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd7928 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xd7930 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xd7938 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xd7940 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0xd7948 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `strb` at 0xd7950 | store | 1 | `sp -0x18` | `([sp] & 0xff)` |
| `ldr` at 0xd7954 | load | 4 | `abs 0xd7ae8` | `-` |
| `ldr` at 0xd7958 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd796c | load | 4 | `abs 0x2adf98` | `-` |
| `str` at 0xd7974 | store | 4 | `sp -0x28` | `?` |
| `strb` at 0xd797c | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0xd7980 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xd7984 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd7988 | load | 4 | `[sp-40] +0x4` | `-` |
| `ldrb` at 0xd7998 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xd799c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd79ac | store | 4 | `sp -0x1c` | `ret(0xd7f54)` |
| `ldrb` at 0xd79b0 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xd79b4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd79c4 | store | 4 | `sp -0x20` | `ret(0xd8068)` |
| `ldr` at 0xd79c8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd79cc | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xd79d0 | load | 1 | `[sp-28] +0x1` | `-` |
| `ldr` at 0xd79dc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd79e0 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0xd79e4 | load | 1 | `[sp-28] +0x2` | `-` |
| `ldr` at 0xd79f0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd79f4 | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0xd79f8 | load | 1 | `[sp-28] +0x3` | `-` |
| `ldrb` at 0xd7a04 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0xd7a08 | load | 1 | `sp -0x18` | `-` |
| `str` at 0xd7a0c | store | 4 | `sp -0x44` | `[sp-24]:1` |
| `ldr` at 0xd7a10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd7a14 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd7a18 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd7a24 | store | 4 | `sp -0x30` | `ret(0xd7afc)` |
| `ldrb` at 0xd7a2c | load | 1 | `sp -0x21` | `-` |
| `strb` at 0xd7a34 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldr` at 0xd7a3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd7a44 | load | 4 | `abs 0xd7aec` | `-` |
| `ldr` at 0xd7a48 | load | 4 | `abs 0xd7af0` | `-` |
| `str` at 0xd7a74 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd7a78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd7a80 | load | 4 | `abs 0xd7aec` | `-` |
| `ldr` at 0xd7a84 | load | 4 | `abs 0xd7af0` | `-` |
| `ldr` at 0xd7a90 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd7aa0 | load | 4 | `abs 0xd7af4` | `-` |
| `str` at 0xd7aa4 | store | 4 | `(unknown)` | `0x135424` |
| `str` at 0xd7aa8 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldrb` at 0xd7aac | load | 1 | `sp -0x15` | `-` |
| `str` at 0xd7ab0 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldrb` at 0xd7ab4 | load | 1 | `sp -0x16` | `-` |
| `str` at 0xd7ab8 | store | 4 | `(unknown)` | `[sp-22]:1` |
| `ldrb` at 0xd7abc | load | 1 | `sp -0x17` | `-` |
| `str` at 0xd7ac0 | store | 4 | `(unknown)` | `[sp-23]:1` |
| `ldr` at 0xd7ac8 | load | 4 | `abs 0xd7af8` | `-` |
| `str` at 0xd7adc | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xd7ae0 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xd7ae4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd7ae4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd7ae4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd79a4 | `(unknown callee) 0xd7f54` | `?` | `?` | `[[sp-40]+4]` | `?` |
| `bl` at 0xd79bc | `(unknown callee) 0xd8068` | `[sp-20]` | `[sp-33]:1` | `?` | `[sp-33]:1` |
| `bl` at 0xd7a1c | `(unknown callee) 0xd7afc` | `[sp-20]` | `[sp-28]` | `ret(0xd8068)` | `[sp-33]:1` |
| `bl` at 0xd7a4c | `(unknown callee) 0x4f9a4` | `?` | `0x0` | `0x1352b0` | `0x1352b8` |
| `bl` at 0xd7a58 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd7a88 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x1352b0` | `0x1352b8` |
| `bl` at 0xd7a98 | `(unknown callee) 0x1b598` | `[sp-44]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd7ad4 | `(unknown callee) 0x21eb8` | `0x1` | `0x13525c` | `0x2f4` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xd7990 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xd79d8 | `[sp-21]:1 != [[sp-28]+1]:1` | `[sp-21]:1 == [[sp-28]+1]:1` |
| `bne` at 0xd79ec | `[sp-22]:1 != [[sp-28]+2]:1` | `[sp-22]:1 == [[sp-28]+2]:1` |
| `bne` at 0xd7a00 | `[sp-23]:1 != [[sp-28]+3]:1` | `[sp-23]:1 == [[sp-28]+3]:1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd7ae4 | `r0` | `?` |

## `ioExpIntHandler`

- `ioExpIntHandler`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd7afc to 0xd7e08.
  Established. A zero `[r1+8]` takes the branch to `0xd7cd0` (`beq` at 0xd7b34). The
  fall-through arm includes a call to unnamed target `0xd85f4` with `r0`, `r1`, `r2+2`, `r2+2` (`bl` at 0xd7b68); its
  zero result takes the branch to a four-byte update at `[sp-32] +0x8` storing `[[sp-32]+8]+1`
  (`beq` at 0xd7b7c, `str` at 0xd7c3c). An indirect site then carries `[sp-24]`, `[sp-33]:1`,
  unknown, `[[sp-32]+2]:1` with an unknown callee (`mov` at 0xd7c74). Interrupt servicing and field
  roles are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd7b0c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xd7b10 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xd7b14 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0xd7b18 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xd7b1c | store | 1 | `sp -0x21` | `(r3 & 0xff)` |
| `strb` at 0xd7b24 | store | 1 | `sp -0x22` | `([sp] & 0xff)` |
| `ldr` at 0xd7b28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd7b2c | load | 4 | `r1 +0x8` | `-` |
| `ldr` at 0xd7b38 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd7b3c | load | 1 | `sp -0x22` | `-` |
| `ldrb` at 0xd7b40 | load | 1 | `r2 +0x3` | `-` |
| `ldr` at 0xd7b54 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd7b5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd7b60 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd7b70 | store | 4 | `sp -0x28` | `ret(0xd85f4)` |
| `ldr` at 0xd7b74 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd7b80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd7b88 | load | 4 | `abs 0xd7dec` | `-` |
| `ldr` at 0xd7b8c | load | 4 | `abs 0xd7df0` | `-` |
| `str` at 0xd7bb8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd7bbc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd7bc4 | load | 4 | `abs 0xd7dec` | `-` |
| `ldr` at 0xd7bc8 | load | 4 | `abs 0xd7df0` | `-` |
| `ldr` at 0xd7bd4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd7be4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd7bf0 | load | 4 | `abs 0xd7df4` | `-` |
| `str` at 0xd7bf4 | store | 4 | `(unknown)` | `0x13545c` |
| `str` at 0xd7bf8 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0xd7bfc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd7c00 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xd7c04 | store | 4 | `(unknown)` | `[[sp-28]+12]` |
| `str` at 0xd7c08 | store | 4 | `(unknown)` | `ret(0x21a38)` |
| `ldr` at 0xd7c10 | load | 4 | `abs 0xd7df8` | `-` |
| `str` at 0xd7c24 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xd7c2c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd7c30 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd7c34 | load | 4 | `[sp-32] +0x8` | `-` |
| `str` at 0xd7c3c | store | 4 | `[sp-32] +0x8` | `[[sp-32]+8]+1` |
| `ldr` at 0xd7c40 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd7c44 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xd7c48 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd7c4c | load | 1 | `sp -0x22` | `-` |
| `ldrb` at 0xd7c50 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0xd7c5c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd7c60 | load | 1 | `[sp-32] +0x2` | `-` |
| `ldr` at 0xd7c64 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xd7c68 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd7c7c | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xd7c84 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd7c88 | load | 1 | `sp -0x22` | `-` |
| `ldrb` at 0xd7c8c | load | 1 | `r2 +0x4` | `-` |
| `ldr` at 0xd7ca0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd7ca4 | load | 1 | `r2 +0x5` | `-` |
| `ldr` at 0xd7cb0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd7cb4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd7cb8 | load | 1 | `r2 +0x5` | `-` |
| `strb` at 0xd7cc0 | store | 1 | `r2 +0x5` | `([r2+5]:1-1 & 0xff)` |
| `str` at 0xd7cc8 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xd7cd0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd7cd8 | load | 4 | `abs 0xd7dec` | `-` |
| `ldr` at 0xd7cdc | load | 4 | `abs 0xd7df0` | `-` |
| `str` at 0xd7d08 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd7d0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd7d14 | load | 4 | `abs 0xd7dec` | `-` |
| `ldr` at 0xd7d18 | load | 4 | `abs 0xd7df0` | `-` |
| `ldr` at 0xd7d24 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd7d34 | load | 4 | `abs 0xd7dfc` | `-` |
| `str` at 0xd7d38 | store | 4 | `(unknown)` | `0x135480` |
| `str` at 0xd7d3c | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0xd7d40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd7d44 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xd7d48 | store | 4 | `(unknown)` | `[[sp-28]+12]` |
| `ldrb` at 0xd7d4c | load | 1 | `sp -0x22` | `-` |
| `str` at 0xd7d50 | store | 4 | `(unknown)` | `[sp-34]:1` |
| `ldr` at 0xd7d54 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd7d58 | load | 1 | `[sp-32] +0x3` | `-` |
| `str` at 0xd7d5c | store | 4 | `(unknown)` | `[[sp-32]+3]:1` |
| `ldr` at 0xd7d60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd7d64 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xd7d70 | load | 4 | `abs 0xd7e00` | `-` |
| `str` at 0xd7d74 | store | 4 | `sp -0x34` | `0x1354c4` |
| `ldr` at 0xd7d7c | load | 4 | `abs 0xd7e04` | `-` |
| `str` at 0xd7d80 | store | 4 | `sp -0x34` | `0x1354c8` |
| `ldr` at 0xd7d84 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xd7d88 | store | 4 | `(unknown)` | `[sp-52]` |
| `ldr` at 0xd7d90 | load | 4 | `abs 0xd7df8` | `-` |
| `ldrb` at 0xd7da0 | load | 1 | `sp -0x21` | `-` |
| `str` at 0xd7da4 | store | 4 | `sp -0x38` | `[sp-33]:1` |
| `ldr` at 0xd7da8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd7dac | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0xd7db8 | load | 1 | `sp -0x22` | `-` |
| `str` at 0xd7dbc | store | 4 | `sp -0x3c` | `[sp-34]:1` |
| `str` at 0xd7dc8 | store | 4 | `sp -0x3c` | `0xff` |
| `ldr` at 0xd7dcc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd7dd0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd7dd4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd7de0 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xd7de4 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xd7de8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd7de8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd7de8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd7de8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd7b68 | `(unknown callee) 0xd85f4` | `r0` | `r1` | `r2+2` | `r2+2` |
| `bl` at 0xd7b90 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x1352b0` | `0x1352b8` |
| `bl` at 0xd7b9c | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd7bcc | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x1352b0` | `0x1352b8` |
| `bl` at 0xd7bdc | `(unknown callee) 0x1b598` | `[sp-44]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd7be8 | `(unknown callee) 0x21a38` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xd7c1c | `(unknown callee) 0x21eb8` | `0x3` | `0x13525c` | `0x308` | `0x0` |
| `mov` at 0xd7c74 | `(unknown callee) (unknown target)` | `[sp-24]` | `[sp-33]:1` | `?` | `[[sp-32]+2]:1` |
| `bl` at 0xd7ce0 | `(unknown callee) 0x4f9a4` | `r0` | `0x0` | `0x1352b0` | `0x1352b8` |
| `bl` at 0xd7cec | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd7d1c | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x1352b0` | `0x1352b8` |
| `bl` at 0xd7d2c | `(unknown callee) 0x1b598` | `[sp-44]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd7d9c | `(unknown callee) 0x21eb8` | `0x1` | `0x13525c` | `0x324` | `0x0` |
| `bl` at 0xd7dd8 | `(unknown callee) 0xd768c` | `[sp-24]` | `[sp-33]:1` | `[sp-60]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd7b34 | `[r1+8] == 0x0` | `[r1+8] != 0x0` |
| `beq` at 0xd7b50 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd7b7c | `ret(0xd85f4) == 0x0` | `ret(0xd85f4) != 0x0` |
| `beq` at 0xd7c9c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd7cac | `[r2+5]:1 == 0x0` | `[r2+5]:1 != 0x0` |
| `beq` at 0xd7d6c | `[[sp-28]+8] == 0x0` | `[[sp-28]+8] != 0x0` |
| `beq` at 0xd7db4 | `[[sp-28]+8] == 0x0` | `[[sp-28]+8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd7de8 | `r0` | `?` |

## `ioExpIgnoreInt`

- `ioExpIgnoreInt`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd7e08 to 0xd7eec.
  Established. Three byte stores preserve low bytes of entry `r1`, `r2`, `r3` at `sp -0x15`,
  `-0x16`, `-0x17` (`strb` at 0xd7e24, `strb` at 0xd7e2c, `strb` at 0xd7e34). The final unnamed call
  to `0x21eb8` has candidates `0x6`, `0x13525c`, `0x337`, `0x0` (`bl` at 0xd7ed0); its result
  expression reaches the exit (`ldmdb` at 0xd7ed4). No conditional branch is reported. An
  interrupt-ignore action is not proved within this caller.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd7e18 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd7e24 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xd7e2c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0xd7e34 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldr` at 0xd7e38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd7e40 | load | 4 | `abs 0xd7ed8` | `-` |
| `ldr` at 0xd7e44 | load | 4 | `abs 0xd7edc` | `-` |
| `str` at 0xd7e70 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xd7e74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd7e7c | load | 4 | `abs 0xd7ed8` | `-` |
| `ldr` at 0xd7e80 | load | 4 | `abs 0xd7edc` | `-` |
| `ldr` at 0xd7e8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd7e9c | load | 4 | `abs 0xd7ee0` | `-` |
| `str` at 0xd7ea0 | store | 4 | `(unknown)` | `0x1354d8` |
| `str` at 0xd7ea4 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldrb` at 0xd7ea8 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xd7eac | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldrb` at 0xd7eb0 | load | 1 | `sp -0x16` | `-` |
| `str` at 0xd7eb4 | store | 4 | `(unknown)` | `[sp-22]:1` |
| `ldrb` at 0xd7eb8 | load | 1 | `sp -0x17` | `-` |
| `str` at 0xd7ebc | store | 4 | `(unknown)` | `[sp-23]:1` |
| `ldr` at 0xd7ec4 | load | 4 | `abs 0xd7ee4` | `-` |
| `ldr` at 0xd7ec8 | load | 4 | `abs 0xd7ee8` | `-` |
| `ldmdb` at 0xd7ed4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd7ed4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd7ed4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd7e48 | `(unknown callee) 0x4f9a4` | `r0` | `0x0` | `0x1352b0` | `0x1352b8` |
| `bl` at 0xd7e54 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd7e84 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x1352b0` | `0x1352b8` |
| `bl` at 0xd7e94 | `(unknown callee) 0x1b598` | `[sp-28]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd7ed0 | `(unknown callee) 0x21eb8` | `0x6` | `0x13525c` | `0x337` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd7ed4 | `r0` | `ret(0x21eb8)` |

## `ioExpGetCfgIndex`

- `ioExpGetCfgIndex`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd7f54 to 0xd8068.
  Established. The four-byte load reads absolute `0x2adf98` (`ldr` at 0xd7f88). The signed
  comparison takes its later arm when `(r1 & 0xff) < [[sp-28]+4]` (`blt` at 0xd7fa4); its other arm
  includes an unnamed call to `0x4f9a4` with `r0`, `0x0`, `0x1352b0`, `0x1352b8` (`bl` at 0xd7fb8).
  A later four-byte read uses `[sp-28] +0x0` (`ldr` at 0xd8040), while the final expression is
  unknown (`ldmdb` at 0xd804c). Configuration-record indexing is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd7f64 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd7f6c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xd7f70 | load | 4 | `abs 0xd8050` | `-` |
| `ldr` at 0xd7f74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd7f88 | load | 4 | `abs 0x2adf98` | `-` |
| `str` at 0xd7f90 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xd7f94 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd7f98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd7f9c | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xd7fa8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd7fb0 | load | 4 | `abs 0xd8054` | `-` |
| `ldr` at 0xd7fb4 | load | 4 | `abs 0xd8058` | `-` |
| `str` at 0xd7fe0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xd7fe4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd7fec | load | 4 | `abs 0xd8054` | `-` |
| `ldr` at 0xd7ff0 | load | 4 | `abs 0xd8058` | `-` |
| `ldr` at 0xd7ffc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd800c | load | 4 | `abs 0xd805c` | `-` |
| `str` at 0xd8010 | store | 4 | `(unknown)` | `0x135520` |
| `str` at 0xd8014 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldrb` at 0xd8018 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xd801c | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0xd8024 | load | 4 | `abs 0xd8060` | `-` |
| `ldr` at 0xd8028 | load | 4 | `abs 0xd8064` | `-` |
| `ldr` at 0xd8034 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd8038 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd8040 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldmdb` at 0xd804c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd804c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd804c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd7fb8 | `(unknown callee) 0x4f9a4` | `r0` | `0x0` | `0x1352b0` | `0x1352b8` |
| `bl` at 0xd7fc4 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd7ff4 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x1352b0` | `0x1352b8` |
| `bl` at 0xd8004 | `(unknown callee) 0x1b598` | `[sp-32]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd8030 | `(unknown callee) 0x21eb8` | `0x9` | `0x13525c` | `0x357` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xd7fa4 | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd804c | `r0` | `?` |

## `ioExpGetValIndex`

- `ioExpGetValIndex`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd8068 to 0xd8188.
  Established. The entry saves four-byte `r0` at `sp -0x14` and byte `(r1 & 0xff)` at `sp -0x15`
  (`str` at 0xd8078, `strb` at 0xd8080). A four-byte read uses absolute `0x2adf98` (`ldr` at
  0xd809c). The signed branch takes its later arm when `(r1 & 0xff) < [[sp-28]+4]` (`blt` at
  0xd80b8). A subsequent read uses `[sp-28] +0x8` (`ldr` at 0xd8160). The tracked return is unknown
  (`ldmdb` at 0xd816c), so a value-record address formula is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd8078 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd8080 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xd8084 | load | 4 | `abs 0xd8170` | `-` |
| `ldr` at 0xd8088 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd809c | load | 4 | `abs 0x2adf98` | `-` |
| `str` at 0xd80a4 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xd80a8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd80ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd80b0 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xd80bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd80c4 | load | 4 | `abs 0xd8174` | `-` |
| `ldr` at 0xd80c8 | load | 4 | `abs 0xd8178` | `-` |
| `str` at 0xd80f4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xd80f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8100 | load | 4 | `abs 0xd8174` | `-` |
| `ldr` at 0xd8104 | load | 4 | `abs 0xd8178` | `-` |
| `ldr` at 0xd8110 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd8120 | load | 4 | `abs 0xd817c` | `-` |
| `str` at 0xd8124 | store | 4 | `(unknown)` | `0x135520` |
| `str` at 0xd8128 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldrb` at 0xd812c | load | 1 | `sp -0x15` | `-` |
| `str` at 0xd8130 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0xd8138 | load | 4 | `abs 0xd8180` | `-` |
| `ldr` at 0xd813c | load | 4 | `abs 0xd8184` | `-` |
| `ldr` at 0xd8148 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd814c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd8160 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldmdb` at 0xd816c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd816c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd816c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd80cc | `(unknown callee) 0x4f9a4` | `r0` | `0x0` | `0x1352b0` | `0x1352b8` |
| `bl` at 0xd80d8 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd8108 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x1352b0` | `0x1352b8` |
| `bl` at 0xd8118 | `(unknown callee) 0x1b598` | `[sp-32]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd8144 | `(unknown callee) 0x21eb8` | `0x9` | `0x13525c` | `0x365` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xd80b8 | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd816c | `r0` | `?` |

## `ioExpWriteExp`

- `ioExpWriteExp`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd8188 to 0xd85f4.
  Established. The entry preserves `(r3 & 0xff)` at `sp -0x1d` and `([sp] & 0xff)` at `sp -0x1e`
  (`strb` at 0xd81a8, `strb` at 0xd81b0). Two byte stores at `[sp-28] +0x1` have unknown values
  (`strb` at 0xd8338, `strb` at 0xd8350). The computed transfer is taken when `[[sp-24]]:1 <= 0x4
  (unsigned, 32-bit)` (`ldrls` at 0xd8360); its targets remain unresolved. The known-edge return
  reports zero (`ldmdb` at 0xd85cc). Selector-only writes and command meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd8198 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd819c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xd81a0 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xd81a4 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xd81a8 | store | 1 | `sp -0x1d` | `(r3 & 0xff)` |
| `strb` at 0xd81b0 | store | 1 | `sp -0x1e` | `([sp] & 0xff)` |
| `ldr` at 0xd81b4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd81b8 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0xd81bc | load | 1 | `r1 +0x4` | `-` |
| `ldr` at 0xd81d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd81d8 | load | 4 | `abs 0xd85d0` | `-` |
| `ldr` at 0xd81dc | load | 4 | `abs 0xd85d4` | `-` |
| `str` at 0xd8208 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xd820c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8214 | load | 4 | `abs 0xd85d0` | `-` |
| `ldr` at 0xd8218 | load | 4 | `abs 0xd85d4` | `-` |
| `ldr` at 0xd8224 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd8234 | load | 4 | `abs 0xd85d8` | `-` |
| `str` at 0xd8238 | store | 4 | `(unknown)` | `0x135544` |
| `str` at 0xd823c | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0xd8240 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd8244 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xd8248 | store | 4 | `(unknown)` | `[[sp-24]+12]` |
| `ldrb` at 0xd824c | load | 1 | `sp -0x1d` | `-` |
| `str` at 0xd8250 | store | 4 | `(unknown)` | `[sp-29]:1` |
| `ldr` at 0xd8258 | load | 4 | `abs 0xd85dc` | `-` |
| `ldr` at 0xd825c | load | 4 | `abs 0xd85e0` | `-` |
| `ldrb` at 0xd8268 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0xd8270 | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0xd8280 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8288 | load | 4 | `abs 0xd85d0` | `-` |
| `ldr` at 0xd828c | load | 4 | `abs 0xd85d4` | `-` |
| `str` at 0xd82b8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xd82bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd82c4 | load | 4 | `abs 0xd85d0` | `-` |
| `ldr` at 0xd82c8 | load | 4 | `abs 0xd85d4` | `-` |
| `ldr` at 0xd82d4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd82e4 | load | 4 | `abs 0xd85e4` | `-` |
| `str` at 0xd82e8 | store | 4 | `(unknown)` | `0x13557c` |
| `str` at 0xd82ec | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0xd82f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd82f4 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xd82f8 | store | 4 | `(unknown)` | `[[sp-24]+12]` |
| `ldrb` at 0xd82fc | load | 1 | `sp -0x1d` | `-` |
| `str` at 0xd8300 | store | 4 | `(unknown)` | `[sp-29]:1` |
| `ldrb` at 0xd8304 | load | 1 | `sp -0x1e` | `-` |
| `str` at 0xd8308 | store | 4 | `(unknown)` | `[sp-30]:1` |
| `ldr` at 0xd8310 | load | 4 | `abs 0xd85dc` | `-` |
| `ldr` at 0xd8314 | load | 4 | `abs 0xd85e8` | `-` |
| `ldr` at 0xd8320 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd8324 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd8328 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0xd8330 | load | 1 | `[sp-28] +0x1` | `-` |
| `strb` at 0xd8338 | store | 1 | `[sp-28] +0x1` | `?` |
| `ldr` at 0xd833c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd8340 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd8344 | load | 1 | `[sp-28] +0x1` | `-` |
| `ldrb` at 0xd8348 | load | 1 | `sp -0x1e` | `-` |
| `strb` at 0xd8350 | store | 1 | `[sp-28] +0x1` | `?` |
| `ldr` at 0xd8354 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd8358 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldrls` at 0xd8360 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd8524 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd852c | load | 4 | `abs 0xd85d0` | `-` |
| `ldr` at 0xd8530 | load | 4 | `abs 0xd85d4` | `-` |
| `str` at 0xd855c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xd8560 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8568 | load | 4 | `abs 0xd85d0` | `-` |
| `ldr` at 0xd856c | load | 4 | `abs 0xd85d4` | `-` |
| `ldr` at 0xd8578 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd8588 | load | 4 | `abs 0xd85ec` | `-` |
| `str` at 0xd858c | store | 4 | `(unknown)` | `0x1355b4` |
| `ldr` at 0xd8590 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd8594 | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0xd8598 | store | 4 | `(unknown)` | `[[sp-24]]:1` |
| `str` at 0xd859c | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0xd85a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd85a4 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xd85a8 | store | 4 | `(unknown)` | `[[sp-24]+12]` |
| `ldr` at 0xd85b0 | load | 4 | `abs 0xd85dc` | `-` |
| `ldr` at 0xd85b4 | load | 4 | `abs 0xd85f0` | `-` |
| `str` at 0xd85c4 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xd85c8 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xd85cc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd85cc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd85cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd81e0 | `(unknown callee) 0x4f9a4` | `r0` | `0x0` | `0x1352b0` | `0x1352b8` |
| `bl` at 0xd81ec | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd821c | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x1352b0` | `0x1352b8` |
| `bl` at 0xd822c | `(unknown callee) 0x1b598` | `[sp-40]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd8264 | `(unknown callee) 0x21eb8` | `0x9` | `0x13525c` | `0x37d` | `0x0` |
| `bl` at 0xd8290 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x1352b0` | `0x1352b8` |
| `bl` at 0xd829c | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd82cc | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x1352b0` | `0x1352b8` |
| `bl` at 0xd82dc | `(unknown callee) 0x1b598` | `[sp-40]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd831c | `(unknown callee) 0x21eb8` | `0x9` | `0x13525c` | `0x381` | `0x0` |
| `bl` at 0xd8534 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x1352b0` | `0x1352b8` |
| `bl` at 0xd8540 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd8570 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x1352b0` | `0x1352b8` |
| `bl` at 0xd8580 | `(unknown callee) 0x1b598` | `[sp-40]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd85bc | `(unknown callee) 0x21eb8` | `0x1` | `0x13525c` | `0x3b1` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd81cc | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd827c | `? == 0x0` | `? != 0x0` |
| `ldrls` at 0xd8360 | `[[sp-24]]:1 <= 0x4 (unsigned, 32-bit)` | `[[sp-24]]:1 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd85cc | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xd8360).

## `ioExpReadExp`

- `ioExpReadExp`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd85f4 to 0xd8800.
  Established. A byte load reads `r1 +0x0` (`ldrb` at 0xd8614). The subsequent computed transfer is
  taken when `[r1]:1 <= 0x4 (unsigned, 32-bit)` and falls through when it is greater (`ldrls` at
  0xd861c); its destinations remain unresolved. The fall-through path includes an unnamed call to
  `0x21eb8` with candidates `0x1`, `0x13525c`, `0x3d5`, `0x0` (`bl` at 0xd87d8). A four-byte zero is
  stored at `sp -0x24` before the known-edge return (`str` at 0xd87e0, `ldmdb` at 0xd87e8). The read
  operation and selector-only effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd8604 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd8608 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xd860c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xd8610 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd8614 | load | 1 | `r1 +0x0` | `-` |
| `ldrls` at 0xd861c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd8740 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8748 | load | 4 | `abs 0xd87ec` | `-` |
| `ldr` at 0xd874c | load | 4 | `abs 0xd87f0` | `-` |
| `str` at 0xd8778 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xd877c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8784 | load | 4 | `abs 0xd87ec` | `-` |
| `ldr` at 0xd8788 | load | 4 | `abs 0xd87f0` | `-` |
| `ldr` at 0xd8794 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd87a4 | load | 4 | `abs 0xd87f4` | `-` |
| `str` at 0xd87a8 | store | 4 | `(unknown)` | `0x1355b4` |
| `ldr` at 0xd87ac | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd87b0 | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0xd87b4 | store | 4 | `(unknown)` | `[[sp-24]]:1` |
| `str` at 0xd87b8 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0xd87bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd87c0 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xd87c4 | store | 4 | `(unknown)` | `[[sp-24]+12]` |
| `ldr` at 0xd87cc | load | 4 | `abs 0xd87f8` | `-` |
| `ldr` at 0xd87d0 | load | 4 | `abs 0xd87fc` | `-` |
| `str` at 0xd87e0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xd87e4 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xd87e8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd87e8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd87e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd8750 | `(unknown callee) 0x4f9a4` | `r0` | `0x0` | `0x1352b0` | `0x1352b8` |
| `bl` at 0xd875c | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd878c | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x1352b0` | `0x1352b8` |
| `bl` at 0xd879c | `(unknown callee) 0x1b598` | `[sp-32]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd87d8 | `(unknown callee) 0x21eb8` | `0x1` | `0x13525c` | `0x3d5` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xd861c | `[r1]:1 <= 0x4 (unsigned, 32-bit)` | `[r1]:1 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd87e8 | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xd861c).
