# ARM EEPROM and interrupt-access paths in 1.38.4

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
The EEPROM read-data, release and status selectors have unresolved computed transfers. Excluded destinations supply no known-edge arguments or effects. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/eeprom-interrupt-access-facts.txt) retain all tool sections.

## `eepromInit`

- `eepromInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd1584 to 0xd1820.
  Established. On the zero arm of the absolute-word test, a word store writes `0x1` at `0x2ad20c`
  (`bne` at 0xd15e4, `str` at 0xd15f0). The call to `0x1b3c4` carries `[sp-40]`, `0x1`, `0x2ad20c`,
  unknown, and its result is stored as a word at `0x2ad208` (`bl` at 0xd1610, `str` at 0xd1618).
  Later word stores include `[sp-32]` at `[sp-44] +0x4`, `ret(0x1b124)` at `[sp-44] +0x8`, and
  `ret(0x1b3c4)` at `[sp-44] +0xc` (`str` at 0xd1678, `str` at 0xd1690, `str` at 0xd175c). EEPROM
  record meanings and the final r0 value remain unknown (`ldmdb` at 0xd17f8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd1594 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xd1598 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xd159c | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0xd15a0 | load | 4 | `abs 0xd17fc` | `-` |
| `ldr` at 0xd15a4 | load | 4 | `abs 0x2ad208` | `-` |
| `ldr` at 0xd15b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd15bc | load | 4 | `abs 0xd1800` | `-` |
| `str` at 0xd15c0 | store | 4 | `sp -0x3c` | `0x134180` |
| `ldr` at 0xd15c8 | load | 4 | `abs 0xd1804` | `-` |
| `ldr` at 0xd15d8 | load | 4 | `abs 0xd1808` | `-` |
| `ldr` at 0xd15dc | load | 4 | `abs 0x2ad20c` | `-` |
| `ldr` at 0xd15e8 | load | 4 | `abs 0xd1808` | `-` |
| `str` at 0xd15f0 | store | 4 | `abs 0x2ad20c` | `0x1` |
| `ldr` at 0xd15f4 | load | 4 | `abs 0xd1808` | `-` |
| `ldr` at 0xd15f8 | load | 4 | `abs 0x2ad20c` | `-` |
| `str` at 0xd1600 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xd1604 | load | 4 | `abs 0xd17fc` | `-` |
| `ldr` at 0xd1608 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd1618 | store | 4 | `abs 0x2ad208` | `ret(0x1b3c4)` |
| `ldr` at 0xd161c | load | 4 | `abs 0x2ad208` | `-` |
| `ldr` at 0xd1628 | load | 4 | `abs 0xd180c` | `-` |
| `str` at 0xd162c | store | 4 | `sp -0x3c` | `0x1341c0` |
| `ldr` at 0xd1630 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd1634 | store | 4 | `sp -0x38` | `[sp-40]` |
| `ldr` at 0xd163c | load | 4 | `abs 0xd1804` | `-` |
| `ldr` at 0xd164c | load | 4 | `abs 0xd17fc` | `-` |
| `ldr` at 0xd1650 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1658 | load | 4 | `abs 0x2ad208` | `-` |
| `str` at 0xd1660 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd1664 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd1668 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd166c | store | 4 | `[sp-44] +0x0` | `?` |
| `ldr` at 0xd1670 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd1674 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd1678 | store | 4 | `[sp-44] +0x4` | `[sp-32]` |
| `ldr` at 0xd167c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd1680 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd1684 | load | 4 | `[sp-44] +0x4` | `-` |
| `str` at 0xd1690 | store | 4 | `[sp-44] +0x8` | `ret(0x1b124)` |
| `ldr` at 0xd1694 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldr` at 0xd16a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd16a8 | load | 4 | `abs 0xd1810` | `-` |
| `ldr` at 0xd16ac | load | 4 | `abs 0xd1814` | `-` |
| `str` at 0xd16d8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd16dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd16e4 | load | 4 | `abs 0xd1810` | `-` |
| `ldr` at 0xd16e8 | load | 4 | `abs 0xd1814` | `-` |
| `ldr` at 0xd16f4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd1704 | load | 4 | `abs 0xd1818` | `-` |
| `str` at 0xd1708 | store | 4 | `(unknown)` | `0x134204` |
| `str` at 0xd170c | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0xd1710 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd1714 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0xd171c | load | 4 | `abs 0xd1804` | `-` |
| `ldr` at 0xd172c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd1730 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd1734 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldr` at 0xd173c | load | 4 | `[sp-44] +0x4` | `-` |
| `ldr` at 0xd1744 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd1748 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd174c | load | 4 | `[sp-44] +0x4` | `-` |
| `str` at 0xd175c | store | 4 | `[sp-44] +0xc` | `ret(0x1b3c4)` |
| `ldr` at 0xd1760 | load | 4 | `[sp-44] +0xc` | `-` |
| `ldr` at 0xd176c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1774 | load | 4 | `abs 0xd1810` | `-` |
| `ldr` at 0xd1778 | load | 4 | `abs 0xd1814` | `-` |
| `str` at 0xd17a4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd17a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd17b0 | load | 4 | `abs 0xd1810` | `-` |
| `ldr` at 0xd17b4 | load | 4 | `abs 0xd1814` | `-` |
| `ldr` at 0xd17c0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd17d0 | load | 4 | `abs 0xd181c` | `-` |
| `str` at 0xd17d4 | store | 4 | `(unknown)` | `0x134240` |
| `str` at 0xd17d8 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0xd17dc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd17e0 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0xd17e8 | load | 4 | `abs 0xd1804` | `-` |
| `ldmdb` at 0xd17f8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd17f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd17f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd17f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd15d4 | `(unknown callee) 0x21e28` | `0x9` | `0x1341a4` | `0xe3` | `0x0` |
| `bl` at 0xd1610 | `(unknown callee) 0x1b3c4` | `[sp-40]` | `0x1` | `0x2ad20c` | `?` |
| `bl` at 0xd1648 | `(unknown callee) 0x21e28` | `0x9` | `0x1341a4` | `0xed` | `0x0` |
| `bl` at 0xd1688 | `(unknown callee) 0x1b124` | `[[sp-44]+4]` | `0x2ad208` | `[sp-44]` | `[sp-44]` |
| `bl` at 0xd16b0 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x1341f8` | `0x134200` |
| `bl` at 0xd16bc | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xd16ec | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x1341f8` | `0x134200` |
| `bl` at 0xd16fc | `(unknown callee) 0x1b508` | `[sp-48]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xd1728 | `(unknown callee) 0x21e28` | `0x9` | `0x1341a4` | `0xf9` | `0x0` |
| `bl` at 0xd1740 | `(unknown callee) 0x1b43c` | `[[sp-44]+8]` | `0x1` | `[[sp-44]+4]` | `[sp-44]` |
| `bl` at 0xd1754 | `(unknown callee) 0x1b3c4` | `[[sp-44]+4]` | `0x78` | `?` | `[sp-44]` |
| `bl` at 0xd177c | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x1341f8` | `0x134200` |
| `bl` at 0xd1788 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xd17b8 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x1341f8` | `0x134200` |
| `bl` at 0xd17c8 | `(unknown callee) 0x1b508` | `[sp-48]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xd17f4 | `(unknown callee) 0x21e28` | `0x9` | `0x1341a4` | `0xfe` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd15ac | `[0x2ad208] == 0x0` | `[0x2ad208] != 0x0` |
| `bne` at 0xd15b8 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xd15e4 | `[0x2ad20c] != 0x0` | `[0x2ad20c] == 0x0` |
| `bne` at 0xd1624 | `[0x2ad208] != 0x0` | `[0x2ad208] == 0x0` |
| `bne` at 0xd169c | `[[sp-44]+8] != 0x0` | `[[sp-44]+8] == 0x0` |
| `bne` at 0xd1768 | `[[sp-44]+12] != 0x0` | `[[sp-44]+12] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd17f8 | `r0` | `?` |

## `eepromNotPresent`

- `eepromNotPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd1820 to 0xd18b8.
  Established. The signed bound takes its alternate path for `(r1 & 0xff) >= [[sp-28]+4]` and falls
  through for less (`bge` at 0xd1864). The fall-through byte store clears `((r1 & 0xff) +
  [[sp-28]+8]) +0x0` (`strb` at 0xd187c). The other path calls `0x21e28` with `0x6`, `0x1341a4`,
  `0x10c`, `0x0` (`bl` at 0xd18a4). Physical presence semantics and the joined r0 return remain
  unproven (`ldmdb` at 0xd18a8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd1830 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd1838 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xd183c | load | 4 | `abs 0xd18ac` | `-` |
| `ldr` at 0xd1840 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1848 | load | 4 | `abs 0x2ad208` | `-` |
| `str` at 0xd1850 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xd1854 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd1858 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd185c | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xd1868 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd186c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd1870 | load | 4 | `[sp-28] +0x8` | `-` |
| `strb` at 0xd187c | store | 1 | `((r1 & 0xff) + [[sp-28]+8]) +0x0` | `0x0` |
| `ldr` at 0xd1884 | load | 4 | `abs 0xd18b0` | `-` |
| `str` at 0xd1888 | store | 4 | `sp -0x24` | `0x13427c` |
| `ldrb` at 0xd188c | load | 1 | `sp -0x15` | `-` |
| `str` at 0xd1890 | store | 4 | `sp -0x20` | `(r1 & 0xff)` |
| `ldr` at 0xd1898 | load | 4 | `abs 0xd18b4` | `-` |
| `ldmdb` at 0xd18a8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd18a8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd18a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd18a4 | `(unknown callee) 0x21e28` | `0x6` | `0x1341a4` | `0x10c` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xd1864 | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd18a8 | `r0` | `?` |

## `eepromProbe`

- `eepromProbe`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd18b8 to 0xd1a50.
  Established. The call to `0xd1e50` has entry r0, `(r1 & 0xff)`, unknown, `(r1 & 0xff)` (`bl` at
  0xd1900); its nonzero result takes the continuation and zero falls through to save word
  `0xfffffbb4` at `sp -0x38` (`bne` at 0xd190c, `str` at 0xd1914). On continuation, `0xd1ec8`
  carries `[sp-28]`, `[sp-29]:1`, `0x0`, `sp-48` (`bl` at 0xd193c). The later byte clear addresses
  `([sp-29]:1 + [[sp-36]+8]) +0x0` (`strb` at 0xd1a20). Probe replies, physical presence and the
  merged return remain unknown (`ldmdb` at 0xd1a30).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd18c8 | store | 4 | `sp -0x1c` | `r0` |
| `strb` at 0xd18d0 | store | 1 | `sp -0x1d` | `(r1 & 0xff)` |
| `ldr` at 0xd18d4 | load | 4 | `abs 0xd1a34` | `-` |
| `ldr` at 0xd18d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd18e0 | load | 4 | `abs 0x2ad208` | `-` |
| `str` at 0xd18e8 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xd18f0 | store | 4 | `sp -0x28` | `0x0` |
| `ldrb` at 0xd18f4 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd18f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1910 | load | 4 | `abs 0xd1a38` | `-` |
| `str` at 0xd1914 | store | 4 | `sp -0x38` | `0xfffffbb4` |
| `ldrb` at 0xd191c | load | 1 | `sp -0x1d` | `-` |
| `str` at 0xd1928 | store | 4 | `sp -0x48` | `0x8` |
| `ldr` at 0xd192c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd1944 | store | 4 | `sp -0x28` | `ret(0xd1ec8)` |
| `ldr` at 0xd1948 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd1954 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd1960 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1968 | load | 4 | `abs 0xd1a3c` | `-` |
| `ldr` at 0xd196c | load | 4 | `abs 0xd1a40` | `-` |
| `str` at 0xd1998 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xd199c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd19a4 | load | 4 | `abs 0xd1a3c` | `-` |
| `ldr` at 0xd19a8 | load | 4 | `abs 0xd1a40` | `-` |
| `ldr` at 0xd19b4 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xd19c4 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd19c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd19d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd19e4 | load | 4 | `abs 0xd1a44` | `-` |
| `str` at 0xd19e8 | store | 4 | `(unknown)` | `0x134298` |
| `str` at 0xd19ec | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `str` at 0xd19f0 | store | 4 | `(unknown)` | `ret(0xd2280)` |
| `str` at 0xd19f4 | store | 4 | `(unknown)` | `ret(0x21674)` |
| `ldr` at 0xd19fc | load | 4 | `abs 0xd1a48` | `-` |
| `ldr` at 0xd1a00 | load | 4 | `abs 0xd1a4c` | `-` |
| `ldr` at 0xd1a0c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xd1a10 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd1a14 | load | 4 | `[sp-36] +0x8` | `-` |
| `strb` at 0xd1a20 | store | 1 | `([sp-29]:1 + [[sp-36]+8]) +0x0` | `0x0` |
| `ldr` at 0xd1a24 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd1a28 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xd1a2c | load | 4 | `sp -0x38` | `-` |
| `ldmdb` at 0xd1a30 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xd1a30 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd1a30 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd1a30 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd1a30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd1900 | `(unknown callee) 0xd1e50` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0xd193c | `(unknown callee) 0xd1ec8` | `[sp-28]` | `[sp-29]:1` | `0x0` | `sp-48` |
| `bl` at 0xd1970 | `(unknown callee) 0x4f564` | `[sp-28]` | `0x0` | `0x1341f8` | `0x134200` |
| `bl` at 0xd197c | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xd19ac | `(unknown callee) 0x4f564` | `[sp-28]` | `0x0` | `0x1341f8` | `0x134200` |
| `bl` at 0xd19bc | `(unknown callee) 0x1b508` | `[sp-52]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xd19d0 | `(unknown callee) 0xd2280` | `[sp-28]` | `[sp-29]:1` | `?` | `[sp-29]:1` |
| `bl` at 0xd19dc | `(unknown callee) 0x21674` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xd1a08 | `(unknown callee) 0x21e28` | `0x1` | `0x1341a4` | `0x11d` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd190c | `ret(0xd1e50) != 0x0` | `ret(0xd1e50) == 0x0` |
| `beq` at 0xd1950 | `ret(0xd1ec8) == 0x0` | `ret(0xd1ec8) != 0x0` |
| `beq` at 0xd195c | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd1a30 | `r0` | `?` |

## `eepromReadAllData`

- `eepromReadAllData`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd1a50 to 0xd1ae8.
  Established. The byte at `sp -0x19` is initialized to `0x0` and later receives `([sp-25]:1+1 &
  0xff)` (`strb` at 0xd1a80, `strb` at 0xd1ad8). The deciding loop branch retains a less-than flags
  predicate with unknown operands, following its edge when true and falling through otherwise (`blt` at
  0xd1a94). The only call targets `0xd1ae8` with `[sp-20]`, unknown, unknown, unknown (`bl` at
  0xd1acc). The read catalogue and returned r0 value are not established (`ldmdb` at 0xd1ae0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd1a60 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xd1a64 | load | 4 | `abs 0xd1ae4` | `-` |
| `ldr` at 0xd1a68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1a70 | load | 4 | `abs 0x2ad208` | `-` |
| `str` at 0xd1a78 | store | 4 | `sp -0x18` | `?` |
| `strb` at 0xd1a80 | store | 1 | `sp -0x19` | `0x0` |
| `ldrb` at 0xd1a84 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xd1a88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1a8c | load | 4 | `[sp-24] +0x4` | `-` |
| `ldrb` at 0xd1a9c | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xd1aa0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd1aa4 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xd1ab8 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xd1ac0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xd1ad0 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xd1ad8 | store | 1 | `sp -0x19` | `([sp-25]:1+1 & 0xff)` |
| `ldmdb` at 0xd1ae0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd1ae0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd1ae0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd1acc | `(unknown callee) 0xd1ae8` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xd1a94 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd1ae0 | `r0` | `?` |

## `eepromReadData`

- `eepromReadData`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd1ae8 to 0xd1c14.
  Established. The call to `0x1b43c` carries `[sp-28]`, `0x0`, `0x78`, `ret(0xd2384)` (`bl` at
  0xd1b2c). A word store then writes `[[sp-32]+4]:1` at `[sp-28] +0x0` (`str` at 0xd1b3c). The
  computed transfer takes an unresolved edge for `[[sp-28]]-1 <= 0x3` unsigned and falls through for
  greater (`ldrls` at 0xd1b84). Excluded format-specific operations, data interpretation and the
  joined r0 return remain unknown (`ldmdb` at 0xd1c08).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd1af8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd1b00 | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0xd1b04 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xd1b08 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd1b0c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd1b1c | store | 4 | `sp -0x20` | `ret(0xd2384)` |
| `ldr` at 0xd1b20 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1b30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1b34 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd1b38 | load | 1 | `[sp-32] +0x4` | `-` |
| `str` at 0xd1b3c | store | 4 | `[sp-28] +0x0` | `[[sp-32]+4]:1` |
| `ldrb` at 0xd1b40 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xd1b4c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd1b50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1b68 | load | 4 | `abs 0xd1c0c` | `-` |
| `str` at 0xd1b6c | store | 4 | `sp -0x24` | `0xfffffbb4` |
| `ldr` at 0xd1b74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1b78 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0xd1b84 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd1bfc | load | 4 | `abs 0xd1c10` | `-` |
| `str` at 0xd1c00 | store | 4 | `sp -0x24` | `0xfffffbb1` |
| `ldr` at 0xd1c04 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xd1c08 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd1c08 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd1c08 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd1b14 | `(unknown callee) 0xd2384` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xd1b2c | `(unknown callee) 0x1b43c` | `[sp-28]` | `0x0` | `0x78` | `ret(0xd2384)` |
| `bl` at 0xd1b58 | `(unknown callee) 0xd1e50` | `[sp-20]` | `[sp-21]:1` | `[sp-28]` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhi` at 0xd1b48 | `[sp-21]:1 > 0xf4 (unsigned, 32-bit)` | `[sp-21]:1 <= 0xf4 (unsigned, 32-bit)` |
| `bne` at 0xd1b64 | `ret(0xd1e50) != 0x0` | `ret(0xd1e50) == 0x0` |
| `ldrls` at 0xd1b84 | `[[sp-28]]-1 <= 0x3 (unsigned, 32-bit)` | `[[sp-28]]-1 > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd1c08 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xd1b84).

## `eepromFreeData`

- `eepromFreeData`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd1c14 to 0xd1da8.
  Established. The computed transfer takes an unresolved edge for `[r0]-1 <= 0x3` unsigned and falls
  through above that bound (`ldrls` at 0xd1c38). The known fall-through route calls `0x1b43c` with
  entry r0, `0x0`, `0x78`, `[r0]-1` and returns its result in r0 (`bl` at 0xd1da0, `ldmdb` at
  0xd1da4). No other calls are reported on known edges. Excluded ownership operations and
  payload-specific release behavior are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd1c24 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xd1c28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1c2c | load | 4 | `r0 +0x0` | `-` |
| `ldrls` at 0xd1c38 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd1d94 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd1da4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd1da4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd1da4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd1da0 | `(unknown callee) 0x1b43c` | `r0` | `0x0` | `0x78` | `[r0]-1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xd1c38 | `[r0]-1 <= 0x3 (unsigned, 32-bit)` | `[r0]-1 > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd1da4 | `r0` | `ret(0x1b43c)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xd1c38).

## `eepromGetCfg`

- `eepromGetCfg`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd1da8 to 0xd1dfc.
  Established. A word store writes `[[sp-28]]` at entry `r1 +0x0` (`str` at 0xd1de4). The next word
  read addresses `[sp-28] +0x4`, and its value supplies the r0 return `[[sp-28]+4]` (`ldr` at
  0xd1dec, `ldmdb` at 0xd1df4). The facts contain no calls or conditional branches. The
  configuration object and units remain unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd1db8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd1dbc | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xd1dc0 | load | 4 | `abs 0xd1df8` | `-` |
| `ldr` at 0xd1dc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1dcc | load | 4 | `abs 0x2ad208` | `-` |
| `str` at 0xd1dd4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xd1dd8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1ddc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1de0 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xd1de4 | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0xd1de8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1dec | load | 4 | `[sp-28] +0x4` | `-` |
| `ldmdb` at 0xd1df4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd1df4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd1df4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd1df4 | `r0` | `[[sp-28]+4]` |

## `eepromGetData`

- `eepromGetData`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd1dfc to 0xd1e50.
  Established. The output word store addresses entry `r1 +0x0` and writes `[[sp-28]+12]` (`str` at
  0xd1e38). A later word read addresses `[sp-28] +0x4` and reaches the return as `[[sp-28]+4]` in r0
  (`ldr` at 0xd1e40, `ldmdb` at 0xd1e48). Calls and conditional branches are absent. Data ownership,
  layout and count interpretation are unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd1e0c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd1e10 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xd1e14 | load | 4 | `abs 0xd1e4c` | `-` |
| `ldr` at 0xd1e18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1e20 | load | 4 | `abs 0x2ad208` | `-` |
| `str` at 0xd1e28 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xd1e2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1e30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1e34 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xd1e38 | store | 4 | `r1 +0x0` | `[[sp-28]+12]` |
| `ldr` at 0xd1e3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1e40 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldmdb` at 0xd1e48 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd1e48 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd1e48 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd1e48 | `r0` | `[[sp-28]+4]` |

## `eepromIsPresent`

- `eepromIsPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd1e50 to 0xd1ec8.
  Established. The signed bound takes its continuing edge for `(r1 & 0xff) < [[sp-28]+4]` and falls
  through for greater-or-equal (`blt` at 0xd1e94). The fall-through arm stores word `0x0` at `sp
  -0x20` (`str` at 0xd1e9c). The other arm stores word `[((r1 & 0xff) + [[sp-28]+8])]:1` there
  (`str` at 0xd1eb8). Physical EEPROM presence and the merged r0 value are unknown (`ldmdb` at
  0xd1ec0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd1e60 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd1e68 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xd1e6c | load | 4 | `abs 0xd1ec4` | `-` |
| `ldr` at 0xd1e70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1e78 | load | 4 | `abs 0x2ad208` | `-` |
| `str` at 0xd1e80 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xd1e84 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd1e88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1e8c | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xd1e9c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xd1ea4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd1ea8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd1eac | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0xd1eb4 | load | 1 | `((r1 & 0xff) + [[sp-28]+8]) +0x0` | `-` |
| `str` at 0xd1eb8 | store | 4 | `sp -0x20` | `[((r1 & 0xff) + [[sp-28]+8])]:1` |
| `ldr` at 0xd1ebc | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xd1ec0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd1ec0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd1ec0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xd1e94 | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd1ec0 | `r0` | `?` |

## `eepromRead`

- `eepromRead`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd1ec8 to 0xd20c0.
  Established. The unsigned range branch takes the continuation for `([sp-24]:2 + [sp-30]:2) <=
  [ret(0xd2384)+2]:2` and falls through for greater (`bls` at 0xd1f30). A later unsigned bound takes
  its bypass for `[sp-44]:2 <= 0x20` and falls through above it to store halfword `0x20` at `sp
  -0x2c` (`bls` at 0xd1fe8, `strh` at 0xd1ff0). The call to `0xc7e94` has unknown, `[sp-42]:1`,
  `[sp-28]`, `[sp-44]:2` (`bl` at 0xd2064). Bank/address encoding, transaction extent and the joined
  r0 return remain unknown (`ldmdb` at 0xd20b4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd1ed8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd1ee0 | store | 4 | `sp -0x1c` | `r3` |
| `ldr` at 0xd1ee4 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xd1eec | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strh` at 0xd1ef0 | store | 2 | `sp -0x18` | `(r2 & 0xffff)` |
| `strh` at 0xd1ef4 | store | 2 | `sp -0x1e` | `([sp] & 0xffff)` |
| `str` at 0xd1efc | store | 4 | `sp -0x28` | `0x0` |
| `ldrb` at 0xd1f00 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd1f04 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd1f14 | store | 4 | `sp -0x24` | `ret(0xd2384)` |
| `ldrh` at 0xd1f18 | load | 2 | `sp -0x18` | `-` |
| `ldrh` at 0xd1f1c | load | 2 | `sp -0x1e` | `-` |
| `ldr` at 0xd1f24 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xd1f28 | load | 2 | `ret(0xd2384) +0x2` | `-` |
| `ldr` at 0xd1f34 | load | 4 | `abs 0xd20b8` | `-` |
| `str` at 0xd1f38 | store | 4 | `sp -0x44` | `0x1342b8` |
| `ldr` at 0xd1f3c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd1f40 | load | 4 | `ret(0xd2384) +0x8` | `-` |
| `str` at 0xd1f44 | store | 4 | `sp -0x40` | `[ret(0xd2384)+8]` |
| `ldrh` at 0xd1f48 | load | 2 | `sp -0x18` | `-` |
| `str` at 0xd1f4c | store | 4 | `sp -0x3c` | `[sp-24]:2` |
| `ldrh` at 0xd1f50 | load | 2 | `sp -0x1e` | `-` |
| `str` at 0xd1f54 | store | 4 | `sp -0x38` | `[sp-30]:2` |
| `ldr` at 0xd1f58 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xd1f5c | load | 2 | `ret(0xd2384) +0x2` | `-` |
| `str` at 0xd1f60 | store | 4 | `sp -0x34` | `[ret(0xd2384)+2]:2` |
| `ldr` at 0xd1f68 | load | 4 | `abs 0xd20bc` | `-` |
| `str` at 0xd1f7c | store | 4 | `sp -0x30` | `0x0` |
| `ldrh` at 0xd1f88 | load | 2 | `sp -0x1e` | `-` |
| `ldr` at 0xd1f94 | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0xd1fa0 | load | 2 | `sp -0x18` | `-` |
| `strb` at 0xd1fac | store | 1 | `sp -0x29` | `?` |
| `ldrb` at 0xd1fb0 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xd1fb4 | store | 1 | `sp -0x2a` | `([sp-24]:1 & 0xff)` |
| `ldrb` at 0xd1fb8 | load | 1 | `sp -0x2a` | `-` |
| `strh` at 0xd1fc0 | store | 2 | `sp -0x2c` | `?` |
| `ldrh` at 0xd1fcc | load | 2 | `sp -0x1e` | `-` |
| `ldrh` at 0xd1fd8 | load | 2 | `sp -0x1e` | `-` |
| `strh` at 0xd1fdc | store | 2 | `sp -0x2c` | `([sp-30]:2 & 0xffff)` |
| `ldrh` at 0xd1fe0 | load | 2 | `sp -0x2c` | `-` |
| `strh` at 0xd1ff0 | store | 2 | `sp -0x2c` | `0x20` |
| `ldr` at 0xd1ff4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xd1ff8 | load | 1 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xd1ffc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd201c | store | 4 | `sp -0x28` | `ret(0xc7a50)` |
| `ldr` at 0xd2020 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd202c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd2030 | store | 4 | `sp -0x30` | `ret(0xc7a50)` |
| `ldr` at 0xd2038 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xd203c | load | 1 | `[sp-36] +0x1` | `-` |
| `ldrb` at 0xd2040 | load | 1 | `sp -0x29` | `-` |
| `ldrb` at 0xd204c | load | 1 | `sp -0x2a` | `-` |
| `ldrh` at 0xd2050 | load | 2 | `sp -0x2c` | `-` |
| `ldr` at 0xd205c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd206c | store | 4 | `sp -0x28` | `ret(0xc7e94)` |
| `ldrh` at 0xd2074 | load | 2 | `sp -0x18` | `-` |
| `ldrh` at 0xd2078 | load | 2 | `sp -0x2c` | `-` |
| `strh` at 0xd2080 | store | 2 | `sp -0x18` | `(([sp-24]:2 + [sp-44]:2) & 0xffff)` |
| `ldrh` at 0xd2084 | load | 2 | `sp -0x2c` | `-` |
| `ldr` at 0xd2088 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd2090 | store | 4 | `sp -0x1c` | `([sp-28] + [sp-44]:2)` |
| `ldrh` at 0xd2094 | load | 2 | `sp -0x1e` | `-` |
| `ldrh` at 0xd2098 | load | 2 | `sp -0x2c` | `-` |
| `strh` at 0xd20a0 | store | 2 | `sp -0x1e` | `?` |
| `ldr` at 0xd20a8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd20ac | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xd20b0 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xd20b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd20b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd20b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd1f0c | `(unknown callee) 0xd2384` | `r0` | `(r1 & 0xff)` | `[sp]` | `(r1 & 0xff)` |
| `bl` at 0xd1f74 | `(unknown callee) 0x21e28` | `0x6` | `0x1341a4` | `0x1e4` | `0x0` |
| `bl` at 0xd2004 | `(unknown callee) 0xc7784` | `[sp-20]` | `[[sp-36]]:1` | `?` | `[[sp-36]]:1` |
| `bl` at 0xd2014 | `(unknown callee) 0xc7a50` | `?` | `?` | `?` | `?` |
| `bl` at 0xd2064 | `(unknown callee) 0xc7e94` | `?` | `[sp-42]:1` | `[sp-28]` | `[sp-44]:2` |
| `bl` at 0xd2070 | `(unknown callee) 0xc7b90` | `ret(0xc7e94)` | `?` | `?` | `ret(0xc7e94)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xd1f30 | `([sp-24]:2 + [sp-30]:2) <= [ret(0xd2384)+2]:2 (unsigned, 32-bit)` | `([sp-24]:2 + [sp-30]:2) > [ret(0xd2384)+2]:2 (unsigned, 32-bit)` |
| `beq` at 0xd1f90 | `[sp-30]:2 == 0x0` | `[sp-30]:2 != 0x0` |
| `bne` at 0xd1f9c | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bls` at 0xd1fd4 | `? <= [sp-30]:2 (unsigned, 32-bit)` | `? > [sp-30]:2 (unsigned, 32-bit)` |
| `bls` at 0xd1fe8 | `[sp-44]:2 <= 0x20 (unsigned, 32-bit)` | `[sp-44]:2 > 0x20 (unsigned, 32-bit)` |
| `beq` at 0xd2028 | `ret(0xc7a50) == 0x0` | `ret(0xc7a50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd20b4 | `r0` | `?` |

## `eepromWrite`

- `eepromWrite`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd20c0 to 0xd2280.
  Established. The unsigned bound takes its continuation for `([sp-24]:2 + [sp-30]:2) <=
  [ret(0xd2384)+2]:2` and falls through for greater (`bls` at 0xd2128). The later call to `0xd43d0`
  carries unknown, unknown, `[sp-42]:1`, `[sp-28]` (`bl` at 0xd2224). Before that call, a word store
  saves `[sp-44]:2` at `sp -0x44` (`str` at 0xd2210). Persistence guarantees, physical addressing
  and the merged return are unproven (`ldmdb` at 0xd2270).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd20d0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd20d8 | store | 4 | `sp -0x1c` | `r3` |
| `ldr` at 0xd20dc | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xd20e4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strh` at 0xd20e8 | store | 2 | `sp -0x18` | `(r2 & 0xffff)` |
| `strh` at 0xd20ec | store | 2 | `sp -0x1e` | `([sp] & 0xffff)` |
| `str` at 0xd20f4 | store | 4 | `sp -0x28` | `0x0` |
| `ldrb` at 0xd20f8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd20fc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd210c | store | 4 | `sp -0x24` | `ret(0xd2384)` |
| `ldrh` at 0xd2110 | load | 2 | `sp -0x18` | `-` |
| `ldrh` at 0xd2114 | load | 2 | `sp -0x1e` | `-` |
| `ldr` at 0xd211c | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xd2120 | load | 2 | `ret(0xd2384) +0x2` | `-` |
| `ldr` at 0xd212c | load | 4 | `abs 0xd2274` | `-` |
| `str` at 0xd2130 | store | 4 | `sp -0x44` | `0x1342b8` |
| `ldr` at 0xd2134 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd2138 | load | 4 | `ret(0xd2384) +0x8` | `-` |
| `str` at 0xd213c | store | 4 | `sp -0x40` | `[ret(0xd2384)+8]` |
| `ldrh` at 0xd2140 | load | 2 | `sp -0x18` | `-` |
| `str` at 0xd2144 | store | 4 | `sp -0x3c` | `[sp-24]:2` |
| `ldrh` at 0xd2148 | load | 2 | `sp -0x1e` | `-` |
| `str` at 0xd214c | store | 4 | `sp -0x38` | `[sp-30]:2` |
| `ldr` at 0xd2150 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xd2154 | load | 2 | `ret(0xd2384) +0x2` | `-` |
| `str` at 0xd2158 | store | 4 | `sp -0x34` | `[ret(0xd2384)+2]:2` |
| `ldr` at 0xd2160 | load | 4 | `abs 0xd2278` | `-` |
| `ldr` at 0xd2164 | load | 4 | `abs 0xd227c` | `-` |
| `str` at 0xd2174 | store | 4 | `sp -0x30` | `0x0` |
| `ldrh` at 0xd2180 | load | 2 | `sp -0x1e` | `-` |
| `ldr` at 0xd218c | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0xd2198 | load | 2 | `sp -0x18` | `-` |
| `strb` at 0xd21a4 | store | 1 | `sp -0x29` | `?` |
| `ldrb` at 0xd21a8 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xd21ac | store | 1 | `sp -0x2a` | `([sp-24]:1 & 0xff)` |
| `ldrb` at 0xd21b0 | load | 1 | `sp -0x2a` | `-` |
| `strh` at 0xd21b8 | store | 2 | `sp -0x2c` | `?` |
| `ldrh` at 0xd21c4 | load | 2 | `sp -0x1e` | `-` |
| `ldrh` at 0xd21d0 | load | 2 | `sp -0x1e` | `-` |
| `strh` at 0xd21d4 | store | 2 | `sp -0x2c` | `([sp-30]:2 & 0xffff)` |
| `ldr` at 0xd21d8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xd21dc | load | 1 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xd21e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd21f4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xd21f8 | load | 1 | `[sp-36] +0x1` | `-` |
| `ldrb` at 0xd21fc | load | 1 | `sp -0x29` | `-` |
| `ldrb` at 0xd2208 | load | 1 | `sp -0x2a` | `-` |
| `ldrh` at 0xd220c | load | 2 | `sp -0x2c` | `-` |
| `str` at 0xd2210 | store | 4 | `sp -0x44` | `[sp-44]:2` |
| `ldr` at 0xd2220 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd222c | store | 4 | `sp -0x28` | `ret(0xd43d0)` |
| `ldrh` at 0xd2230 | load | 2 | `sp -0x18` | `-` |
| `ldrh` at 0xd2234 | load | 2 | `sp -0x2c` | `-` |
| `strh` at 0xd223c | store | 2 | `sp -0x18` | `(([sp-24]:2 + [sp-44]:2) & 0xffff)` |
| `ldrh` at 0xd2240 | load | 2 | `sp -0x2c` | `-` |
| `ldr` at 0xd2244 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd224c | store | 4 | `sp -0x1c` | `([sp-28] + [sp-44]:2)` |
| `ldrh` at 0xd2250 | load | 2 | `sp -0x1e` | `-` |
| `ldrh` at 0xd2254 | load | 2 | `sp -0x2c` | `-` |
| `strh` at 0xd225c | store | 2 | `sp -0x1e` | `?` |
| `ldr` at 0xd2264 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd2268 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xd226c | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xd2270 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd2270 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd2270 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd2104 | `(unknown callee) 0xd2384` | `r0` | `(r1 & 0xff)` | `[sp]` | `(r1 & 0xff)` |
| `bl` at 0xd216c | `(unknown callee) 0x21e28` | `0x6` | `0x1341a4` | `0x22d` | `0x0` |
| `bl` at 0xd21e8 | `(unknown callee) 0xc7784` | `[sp-20]` | `[[sp-36]]:1` | `?` | `[[sp-36]]:1` |
| `bl` at 0xd2224 | `(unknown callee) 0xd43d0` | `?` | `?` | `[sp-42]:1` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xd2128 | `([sp-24]:2 + [sp-30]:2) <= [ret(0xd2384)+2]:2 (unsigned, 32-bit)` | `([sp-24]:2 + [sp-30]:2) > [ret(0xd2384)+2]:2 (unsigned, 32-bit)` |
| `beq` at 0xd2188 | `[sp-30]:2 == 0x0` | `[sp-30]:2 != 0x0` |
| `bne` at 0xd2194 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bls` at 0xd21cc | `? <= [sp-30]:2 (unsigned, 32-bit)` | `? > [sp-30]:2 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd2270 | `r0` | `?` |

## `eepromName`

- `eepromName`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd2280 to 0xd22e8.
  Established. The only call reaches `0xd2384` with entry r0, `(r1 & 0xff)`, entry r2, `(r1 & 0xff)`
  (`bl` at 0xd22a8). A later word read addresses `ret(0xd2384) +0x8` (`ldr` at 0xd22c4); its nonzero
  value takes the bypass and zero falls through to save word `0x1342f8` at `sp -0x20` (`bne` at
  0xd22cc, `str` at 0xd22d4). The name representation and joined r0 value remain unknown (`ldmdb` at
  0xd22e0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd2290 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd2298 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xd229c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd22a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd22b0 | store | 4 | `sp -0x1c` | `ret(0xd2384)` |
| `ldr` at 0xd22b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd22b8 | load | 4 | `ret(0xd2384) +0x8` | `-` |
| `str` at 0xd22bc | store | 4 | `sp -0x20` | `[ret(0xd2384)+8]` |
| `ldr` at 0xd22c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd22c4 | load | 4 | `ret(0xd2384) +0x8` | `-` |
| `ldr` at 0xd22d0 | load | 4 | `abs 0xd22e4` | `-` |
| `str` at 0xd22d4 | store | 4 | `sp -0x20` | `0x1342f8` |
| `ldr` at 0xd22d8 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xd22e0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd22e0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd22e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd22a8 | `(unknown callee) 0xd2384` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd22cc | `[ret(0xd2384)+8] != 0x0` | `[ret(0xd2384)+8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd22e0 | `r0` | `?` |

## `eepromStatus`

- `eepromStatus`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd22e8 to 0xd2384.
  Established. The computed transfer takes an unresolved edge for entry `r0 <= 0x4` unsigned and
  falls through for greater (`ldrls` at 0xd2304). On the known fall-through route, a word store puts
  `0x13434c` at `sp -0x18`, and that value is returned in r0 (`str` at 0xd2360, `ldmdb` at 0xd2368).
  No calls are reported on known edges. The excluded status cases and their names remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd22f8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xd22fc | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0xd2304 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd235c | load | 4 | `abs 0xd2380` | `-` |
| `str` at 0xd2360 | store | 4 | `sp -0x18` | `0x13434c` |
| `ldr` at 0xd2364 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xd2368 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd2368 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd2368 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xd2304 | `r0 <= 0x4 (unsigned, 32-bit)` | `r0 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd2368 | `r0` | `0x13434c` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xd2304).

## `vsc055_IntSet`

- `vsc055_IntSet`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd9244 to 0xd935c.
  Established. The call to `0xc7f7c` carries unknown, `[[sp-24]+2]:1`, `[sp-34]:1`, `[sp-35]:1`
  (`bl` at 0xd9308); its zero result takes the continuing edge and nonzero falls through to save
  word `ret(0xc7f7c)` at `sp -0x2c` (`beq` at 0xd931c, `str` at 0xd9324). The loop branch retains a
  lower-or-same flags predicate with unknown operands, following its edge when true and falling through
  otherwise (`bls` at 0xd928c). Interrupt setting, register interpretation and the merged r0 return
  remain unknown (`ldmdb` at 0xd9354).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd9254 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd9258 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xd925c | store | 4 | `sp -0x20` | `r3` |
| `strb` at 0xd9264 | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `ldr` at 0xd9268 | load | 4 | `abs 0xd9358` | `-` |
| `ldr` at 0xd926c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd9274 | load | 1 | `r3 +0x142b18` | `-` |
| `strb` at 0xd9278 | store | 1 | `sp -0x23` | `([r3+1321752]:1 & 0xff)` |
| `strb` at 0xd9280 | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0xd9284 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0xd9294 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xd92a4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd92a8 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xd92b4 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0xd92cc | store | 1 | `sp -0x22` | `?` |
| `ldr` at 0xd92d0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd92d4 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xd92d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd92ec | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd92f0 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0xd92f4 | load | 1 | `sp -0x22` | `-` |
| `ldrb` at 0xd92f8 | load | 1 | `sp -0x23` | `-` |
| `str` at 0xd9310 | store | 4 | `sp -0x28` | `ret(0xc7f7c)` |
| `ldr` at 0xd9314 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd9320 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd9324 | store | 4 | `sp -0x2c` | `ret(0xc7f7c)` |
| `ldrb` at 0xd932c | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xd9334 | store | 1 | `sp -0x19` | `?` |
| `ldrb` at 0xd9338 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0xd9340 | store | 1 | `sp -0x21` | `?` |
| `str` at 0xd934c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xd9350 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xd9354 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd9354 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd9354 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd92e0 | `(unknown callee) 0xc7784` | `[sp-20]` | `[[sp-24]+1]:1` | `?` | `[[sp-24]+1]:1` |
| `bl` at 0xd9308 | `(unknown callee) 0xc7f7c` | `?` | `[[sp-24]+2]:1` | `[sp-34]:1` | `[sp-35]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xd928c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0xd92a0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd931c | `ret(0xc7f7c) == 0x0` | `ret(0xc7f7c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd9354 | `r0` | `?` |

## `vsc055_IntGet`

- `vsc055_IntGet`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd935c to 0xd94a8.
  Established. The first byte store clears entry `r2 +0x0` (`strb` at 0xd9380). A call to `0xc80a0`
  has unknown, `[[sp-24]+2]:1`, `[sp-30]:1`, `sp-31` (`bl` at 0xd9400); its zero result takes the
  continuation and nonzero falls through to an early saved result (`beq` at 0xd9414, `str` at
  0xd941c). A later byte store writes an unknown value at `[sp-28] +0x0` (`strb` at 0xd9484). The
  reported r0 return and physical interrupt-status interpretation remain unproven (`ldmdb` at
  0xd94a4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd936c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd9370 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xd9374 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xd9378 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xd9380 | store | 1 | `r2 +0x0` | `0x0` |
| `strb` at 0xd9388 | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0xd938c | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd939c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd93a0 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldrb` at 0xd93ac | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0xd93c4 | store | 1 | `sp -0x1e` | `?` |
| `ldr` at 0xd93c8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd93cc | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xd93d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd93e4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd93e8 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0xd93ec | load | 1 | `sp -0x1e` | `-` |
| `str` at 0xd9408 | store | 4 | `sp -0x24` | `ret(0xc80a0)` |
| `ldr` at 0xd940c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd9418 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd941c | store | 4 | `sp -0x28` | `ret(0xc80a0)` |
| `ldrb` at 0xd9424 | load | 1 | `sp -0x1f` | `-` |
| `str` at 0xd942c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd9430 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd943c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd9448 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd9458 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd9468 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd946c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd9470 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0xd947c | load | 1 | `[sp-28] +0x0` | `-` |
| `strb` at 0xd9484 | store | 1 | `[sp-28] +0x0` | `?` |
| `ldrb` at 0xd9488 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0xd9490 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `str` at 0xd949c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xd94a0 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xd94a4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd94a4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd94a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd93d8 | `(unknown callee) 0xc7784` | `[sp-20]` | `[[sp-24]+1]:1` | `?` | `[[sp-24]+1]:1` |
| `bl` at 0xd9400 | `(unknown callee) 0xc80a0` | `?` | `[[sp-24]+2]:1` | `[sp-30]:1` | `sp-31` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xd9394 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0xd9414 | `ret(0xc80a0) == 0x0` | `ret(0xc80a0) != 0x0` |
| `beq` at 0xd9438 | `[sp-44] == 0xa` | `[sp-44] != 0xa` |
| `bgt` at 0xd9444 | `[sp-44] > 0xa (signed, 32-bit)` | `[sp-44] <= 0xa (signed, 32-bit)` |
| `beq` at 0xd9450 | `[sp-44] == 0x6` | `[sp-44] != 0x6` |
| `beq` at 0xd9460 | `[sp-44] == 0xe` | `[sp-44] != 0xe` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd94a4 | `r0` | `?` |
