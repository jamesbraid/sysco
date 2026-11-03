# ARM FRU, chip and scan records in 1.38.4

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
Undecodable byte ranges remain visible as fact-sheet warnings. Calls and predicates retain unknown values where tracking cannot follow the body.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/fru-chip-scan-records-facts.txt) retain all tool sections.

## `FRUEventGuess`

- `FRUEventGuess`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc1138 to 0xc1cbc.
  Established. The first byte test uses `[sp-28] +0xd` (`ldrb` at 0xc1168). Nonzero takes its branch
  and zero falls through (`bne` at 0xc1170). The opening call targets `0x1b508` with candidates
  `sp-284`, `0x1327a8`, entry r2 and `sp-284` (`bl` at 0xc1160). A later indirect call has candidates
  `[sp-24]`, `sp-924`, `ret(0x1b22c)` and `sp-924`, but its target is unknown (`mov` at 0xc1980).
  The FRU classification and output destination are not established by these operands.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc1148 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc114c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc1150 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xc115c | load | 4 | `abs 0xc1c60` | `-` |
| `ldr` at 0xc1164 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1168 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xc117c | load | 4 | `abs 0xc1c64` | `-` |
| `ldr` at 0xc1184 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1188 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xc11a0 | load | 4 | `abs 0xc1c60` | `-` |
| `ldr` at 0xc11bc | load | 4 | `abs 0xc1c68` | `-` |
| `ldr` at 0xc11cc | load | 4 | `abs 0xc1c6c` | `-` |
| `ldr` at 0xc11d4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc11d8 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xc11f0 | load | 4 | `abs 0xc1c60` | `-` |
| `ldr` at 0xc120c | load | 4 | `abs 0xc1c68` | `-` |
| `ldr` at 0xc121c | load | 4 | `abs 0xc1c70` | `-` |
| `ldr` at 0xc1224 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1228 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xc1240 | load | 4 | `abs 0xc1c60` | `-` |
| `ldr` at 0xc125c | load | 4 | `abs 0xc1c68` | `-` |
| `ldr` at 0xc126c | load | 4 | `abs 0xc1c74` | `-` |
| `ldr` at 0xc1274 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1278 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xc1290 | load | 4 | `abs 0xc1c60` | `-` |
| `ldr` at 0xc12ac | load | 4 | `abs 0xc1c68` | `-` |
| `ldr` at 0xc12bc | load | 4 | `abs 0xc1c78` | `-` |
| `ldr` at 0xc12c4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc12c8 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xc12e0 | load | 4 | `abs 0xc1c60` | `-` |
| `ldr` at 0xc12fc | load | 4 | `abs 0xc1c68` | `-` |
| `ldr` at 0xc130c | load | 4 | `abs 0xc1c7c` | `-` |
| `ldr` at 0xc1314 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1318 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xc1330 | load | 4 | `abs 0xc1c60` | `-` |
| `ldr` at 0xc134c | load | 4 | `abs 0xc1c68` | `-` |
| `ldr` at 0xc135c | load | 4 | `abs 0xc1c80` | `-` |
| `ldr` at 0xc136c | load | 4 | `abs 0xc1c60` | `-` |
| `ldr` at 0xc1374 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xc1378 | load | 2 | `[sp-28] +0x1a` | `-` |
| `ldr` at 0xc137c | load | 4 | `abs 0xc1c84` | `-` |
| `ldr` at 0xc1388 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc138c | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xc139c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xc13a0 | load | 2 | `[sp-28] +0x1a` | `-` |
| `str` at 0xc13ac | store | 4 | `sp -0x3a4` | `?` |
| `ldr` at 0xc13b0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc13b4 | load | 1 | `[sp-28] +0x21` | `-` |
| `ldr` at 0xc13c0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc13c4 | load | 1 | `[sp-28] +0x22` | `-` |
| `ldr` at 0xc13d0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc13d4 | load | 1 | `[sp-28] +0x23` | `-` |
| `ldr` at 0xc13e0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc13e4 | load | 1 | `[sp-28] +0x24` | `-` |
| `ldr` at 0xc13f8 | load | 4 | `abs 0xc1c88` | `-` |
| `ldr` at 0xc1408 | load | 4 | `abs 0xc1c8c` | `-` |
| `ldr` at 0xc140c | load | 4 | `sp -0x3a4` | `-` |
| `ldr` at 0xc1418 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc141c | load | 1 | `[sp-28] +0x21` | `-` |
| `ldr` at 0xc1428 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc142c | load | 1 | `[sp-28] +0x22` | `-` |
| `ldr` at 0xc1438 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc143c | load | 1 | `[sp-28] +0x23` | `-` |
| `ldr` at 0xc1448 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc144c | load | 1 | `[sp-28] +0x24` | `-` |
| `ldr` at 0xc1460 | load | 4 | `abs 0xc1c90` | `-` |
| `ldr` at 0xc1470 | load | 4 | `abs 0xc1c8c` | `-` |
| `ldr` at 0xc1474 | load | 4 | `sp -0x3a4` | `-` |
| `ldr` at 0xc1488 | load | 4 | `abs 0xc1c8c` | `-` |
| `ldr` at 0xc148c | load | 4 | `sp -0x3a4` | `-` |
| `ldr` at 0xc1494 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1498 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xc14b0 | load | 4 | `abs 0xc1c94` | `-` |
| `ldr` at 0xc14c0 | load | 4 | `abs 0xc1c60` | `-` |
| `ldr` at 0xc14d0 | load | 4 | `abs 0xc1c60` | `-` |
| `ldr` at 0xc14e0 | load | 4 | `abs 0xc1c60` | `-` |
| `ldr` at 0xc14f0 | load | 4 | `abs 0xc1c60` | `-` |
| `ldr` at 0xc14f8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc14fc | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xc1514 | load | 4 | `abs 0xc1c98` | `-` |
| `ldr` at 0xc1524 | load | 4 | `abs 0xc1c98` | `-` |
| `ldr` at 0xc1534 | load | 4 | `abs 0xc1c98` | `-` |
| `ldr` at 0xc1544 | load | 4 | `abs 0xc1c98` | `-` |
| `str` at 0xc1550 | store | 4 | `sp -0x3a8` | `0x8` |
| `ldr` at 0xc1554 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1558 | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0xc1564 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1568 | load | 1 | `[sp-28] +0x14` | `-` |
| `ldr` at 0xc1574 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1578 | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0xc1580 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1584 | load | 1 | `[sp-28] +0x14` | `-` |
| `str` at 0xc158c | store | 4 | `sp -0x3a8` | `?` |
| `str` at 0xc1594 | store | 4 | `sp -0x3ac` | `0x3` |
| `str` at 0xc159c | store | 4 | `sp -0x3b0` | `0x3` |
| `str` at 0xc15a4 | store | 4 | `sp -0x3b4` | `0x3` |
| `str` at 0xc15ac | store | 4 | `sp -0x3b8` | `0x3` |
| `ldr` at 0xc15b0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc15b4 | load | 1 | `[sp-28] +0x15` | `-` |
| `ldr` at 0xc15c0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc15c4 | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0xc15d0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc15d4 | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0xc15e4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc15e8 | load | 1 | `[sp-28] +0x15` | `-` |
| `str` at 0xc15ec | store | 4 | `sp -0x3ac` | `[[sp-28]+21]:1` |
| `ldr` at 0xc15f0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc15f4 | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0xc1600 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1604 | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0xc1614 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1618 | load | 1 | `[sp-28] +0x21` | `-` |
| `ldr` at 0xc1624 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1628 | load | 1 | `[sp-28] +0x21` | `-` |
| `str` at 0xc1638 | store | 4 | `sp -0x3ac` | `ret(0xc057c)` |
| `ldr` at 0xc163c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1640 | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0xc164c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1650 | load | 1 | `[sp-28] +0x22` | `-` |
| `ldr` at 0xc165c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1660 | load | 1 | `[sp-28] +0x22` | `-` |
| `str` at 0xc1670 | store | 4 | `sp -0x3b0` | `ret(0xc057c)` |
| `ldr` at 0xc1674 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1678 | load | 1 | `[sp-28] +0x23` | `-` |
| `ldr` at 0xc1684 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1688 | load | 1 | `[sp-28] +0x23` | `-` |
| `str` at 0xc1698 | store | 4 | `sp -0x3b4` | `ret(0xc057c)` |
| `ldr` at 0xc169c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc16a0 | load | 1 | `[sp-28] +0x24` | `-` |
| `ldr` at 0xc16ac | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc16b0 | load | 1 | `[sp-28] +0x24` | `-` |
| `str` at 0xc16c0 | store | 4 | `sp -0x3b8` | `ret(0xc057c)` |
| `ldr` at 0xc16c4 | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xc16d0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc16d4 | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0xc16e0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc16e4 | load | 1 | `[sp-28] +0x21` | `-` |
| `ldr` at 0xc16f0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc16f4 | load | 1 | `[sp-28] +0x22` | `-` |
| `ldr` at 0xc1700 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1704 | load | 1 | `[sp-28] +0x23` | `-` |
| `ldr` at 0xc1710 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1714 | load | 1 | `[sp-28] +0x24` | `-` |
| `ldr` at 0xc1724 | load | 4 | `abs 0xc1c9c` | `-` |
| `ldr` at 0xc1728 | load | 4 | `sp -0x3ac` | `-` |
| `ldrb` at 0xc1730 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc1734 | load | 4 | `abs 0xc1ca0` | `-` |
| `ldr` at 0xc1738 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc173c | load | 1 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0xc1740 | load | 1 | `[[sp-28]+20]:1 +0x128cfc` | `-` |
| `str` at 0xc1744 | store | 4 | `sp -0x3c0` | `[[[sp-28]+20]:1+1215740]:1` |
| `ldr` at 0xc174c | load | 4 | `abs 0xc1ca4` | `-` |
| `ldr` at 0xc1750 | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xc1764 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1768 | load | 1 | `[sp-28] +0xc` | `-` |
| `ldr` at 0xc176c | load | 4 | `abs 0xc1c9c` | `-` |
| `ldr` at 0xc1770 | load | 4 | `sp -0x3ac` | `-` |
| `ldrb` at 0xc1778 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc177c | store | 4 | `sp -0x3c0` | `?` |
| `ldr` at 0xc1780 | load | 4 | `abs 0xc1ca0` | `-` |
| `ldr` at 0xc1784 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1788 | load | 1 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0xc178c | load | 1 | `[[sp-28]+20]:1 +0x128cfc` | `-` |
| `str` at 0xc1790 | store | 4 | `sp -0x3bc` | `[[[sp-28]+20]:1+1215740]:1` |
| `ldr` at 0xc1798 | load | 4 | `abs 0xc1ca8` | `-` |
| `ldr` at 0xc179c | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xc17ac | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xc17b8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc17bc | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0xc17c8 | load | 4 | `sp -0x3ac` | `-` |
| `ldr` at 0xc17d8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc17dc | load | 1 | `[sp-28] +0xc` | `-` |
| `ldr` at 0xc17e0 | load | 4 | `abs 0xc1c9c` | `-` |
| `ldr` at 0xc17e4 | load | 4 | `sp -0x3ac` | `-` |
| `ldrb` at 0xc17ec | load | 1 | `(unknown)` | `-` |
| `str` at 0xc17f0 | store | 4 | `sp -0x3c0` | `?` |
| `ldr` at 0xc17f4 | load | 4 | `abs 0xc1ca0` | `-` |
| `ldr` at 0xc17f8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc17fc | load | 1 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0xc1800 | load | 1 | `[[sp-28]+20]:1 +0x128cfc` | `-` |
| `str` at 0xc1804 | store | 4 | `sp -0x3bc` | `[[[sp-28]+20]:1+1215740]:1` |
| `ldr` at 0xc180c | load | 4 | `abs 0xc1ca8` | `-` |
| `ldr` at 0xc1810 | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xc181c | load | 4 | `sp -0x3b0` | `-` |
| `ldr` at 0xc182c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1830 | load | 1 | `[sp-28] +0xc` | `-` |
| `ldr` at 0xc1834 | load | 4 | `abs 0xc1c9c` | `-` |
| `ldr` at 0xc1838 | load | 4 | `sp -0x3b0` | `-` |
| `ldrb` at 0xc1840 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc1844 | store | 4 | `sp -0x3c0` | `?` |
| `ldr` at 0xc1848 | load | 4 | `abs 0xc1ca0` | `-` |
| `ldr` at 0xc184c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1850 | load | 1 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0xc1854 | load | 1 | `[[sp-28]+20]:1 +0x128cfc` | `-` |
| `str` at 0xc1858 | store | 4 | `sp -0x3bc` | `[[[sp-28]+20]:1+1215740]:1` |
| `ldr` at 0xc1860 | load | 4 | `abs 0xc1ca8` | `-` |
| `ldr` at 0xc1864 | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xc1870 | load | 4 | `sp -0x3b4` | `-` |
| `ldr` at 0xc1880 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1884 | load | 1 | `[sp-28] +0xc` | `-` |
| `ldr` at 0xc1888 | load | 4 | `abs 0xc1c9c` | `-` |
| `ldr` at 0xc188c | load | 4 | `sp -0x3b4` | `-` |
| `ldrb` at 0xc1894 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc1898 | store | 4 | `sp -0x3c0` | `?` |
| `ldr` at 0xc189c | load | 4 | `abs 0xc1ca0` | `-` |
| `ldr` at 0xc18a0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc18a4 | load | 1 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0xc18a8 | load | 1 | `[[sp-28]+20]:1 +0x128cfc` | `-` |
| `str` at 0xc18ac | store | 4 | `sp -0x3bc` | `[[[sp-28]+20]:1+1215740]:1` |
| `ldr` at 0xc18b4 | load | 4 | `abs 0xc1ca8` | `-` |
| `ldr` at 0xc18b8 | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xc18c4 | load | 4 | `sp -0x3b8` | `-` |
| `ldr` at 0xc18d4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc18d8 | load | 1 | `[sp-28] +0xc` | `-` |
| `ldr` at 0xc18dc | load | 4 | `abs 0xc1c9c` | `-` |
| `ldr` at 0xc18e0 | load | 4 | `sp -0x3b8` | `-` |
| `ldrb` at 0xc18e8 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc18ec | store | 4 | `sp -0x3c0` | `?` |
| `ldr` at 0xc18f0 | load | 4 | `abs 0xc1ca0` | `-` |
| `ldr` at 0xc18f4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc18f8 | load | 1 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0xc18fc | load | 1 | `[[sp-28]+20]:1 +0x128cfc` | `-` |
| `str` at 0xc1900 | store | 4 | `sp -0x3bc` | `[[[sp-28]+20]:1+1215740]:1` |
| `ldr` at 0xc1908 | load | 4 | `abs 0xc1ca8` | `-` |
| `ldr` at 0xc190c | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xc1918 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xc191c | load | 2 | `[sp-28] +0x1c` | `-` |
| `ldr` at 0xc1920 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xc1924 | load | 2 | `[sp-28] +0x1e` | `-` |
| `str` at 0xc192c | store | 4 | `sp -0x3a0` | `([[sp-28]+28]:2 + [[sp-28]+30]:2)` |
| `ldr` at 0xc1934 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xc1938 | load | 2 | `[sp-28] +0x18` | `-` |
| `str` at 0xc1940 | store | 4 | `sp -0x3c0` | `sp-284` |
| `ldr` at 0xc1948 | load | 4 | `abs 0xc1cac` | `-` |
| `ldr` at 0xc194c | load | 4 | `sp -0x3a0` | `-` |
| `ldr` at 0xc1970 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc1974 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc1988 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xc198c | load | 2 | `[sp-28] +0x18` | `-` |
| `ldr` at 0xc1990 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1994 | load | 1 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xc1998 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc199c | load | 1 | `[sp-28] +0x9` | `-` |
| `str` at 0xc19a0 | store | 4 | `sp -0x3c0` | `[[sp-28]+9]:1` |
| `ldr` at 0xc19a8 | load | 4 | `abs 0xc1cb0` | `-` |
| `ldr` at 0xc19d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc19d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc19ec | load | 4 | `abs 0xc1c60` | `-` |
| `ldr` at 0xc1a0c | load | 4 | `abs 0xc1cb4` | `-` |
| `ldr` at 0xc1a2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc1a30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc1a48 | load | 4 | `abs 0xc1c60` | `-` |
| `ldr` at 0xc1a64 | load | 4 | `abs 0xc1c98` | `-` |
| `ldr` at 0xc1a84 | load | 4 | `abs 0xc1cb4` | `-` |
| `ldr` at 0xc1aa4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc1aa8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc1ac0 | load | 4 | `abs 0xc1c60` | `-` |
| `ldr` at 0xc1adc | load | 4 | `abs 0xc1c98` | `-` |
| `ldr` at 0xc1afc | load | 4 | `abs 0xc1cb4` | `-` |
| `ldr` at 0xc1b1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc1b20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc1b38 | load | 4 | `abs 0xc1c60` | `-` |
| `ldr` at 0xc1b54 | load | 4 | `abs 0xc1c98` | `-` |
| `ldr` at 0xc1b74 | load | 4 | `abs 0xc1cb4` | `-` |
| `ldr` at 0xc1b94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc1b98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc1bb0 | load | 4 | `abs 0xc1c60` | `-` |
| `ldr` at 0xc1bcc | load | 4 | `abs 0xc1c98` | `-` |
| `ldr` at 0xc1bec | load | 4 | `abs 0xc1cb4` | `-` |
| `ldr` at 0xc1c0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc1c10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc1c28 | load | 4 | `abs 0xc1cb8` | `-` |
| `ldr` at 0xc1c48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc1c4c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xc1c5c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc1c5c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc1c5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc1160 | `(unknown callee) 0x1b508` | `sp-284` | `0x1327a8` | `r2` | `sp-284` |
| `bl` at 0xc1180 | `(unknown callee) 0x1b2d4` | `sp-284` | `0x1327ac` | `?` | `sp-284` |
| `bl` at 0xc11a4 | `(unknown callee) 0x1aff8` | `sp-284` | `0x1327a8` | `?` | `sp-284` |
| `bl` at 0xc11c0 | `(unknown callee) 0x1b2d4` | `sp-284` | `0x1327b4` | `?` | `sp-284` |
| `bl` at 0xc11d0 | `(unknown callee) 0x1b2d4` | `sp-284` | `0x1327b8` | `?` | `sp-284` |
| `bl` at 0xc11f4 | `(unknown callee) 0x1aff8` | `sp-284` | `0x1327a8` | `?` | `sp-284` |
| `bl` at 0xc1210 | `(unknown callee) 0x1b2d4` | `sp-284` | `0x1327b4` | `?` | `sp-284` |
| `bl` at 0xc1220 | `(unknown callee) 0x1b2d4` | `sp-284` | `0x1327cc` | `?` | `sp-284` |
| `bl` at 0xc1244 | `(unknown callee) 0x1aff8` | `sp-284` | `0x1327a8` | `?` | `sp-284` |
| `bl` at 0xc1260 | `(unknown callee) 0x1b2d4` | `sp-284` | `0x1327b4` | `?` | `sp-284` |
| `bl` at 0xc1270 | `(unknown callee) 0x1b2d4` | `sp-284` | `0x1327d8` | `?` | `sp-284` |
| `bl` at 0xc1294 | `(unknown callee) 0x1aff8` | `sp-284` | `0x1327a8` | `?` | `sp-284` |
| `bl` at 0xc12b0 | `(unknown callee) 0x1b2d4` | `sp-284` | `0x1327b4` | `?` | `sp-284` |
| `bl` at 0xc12c0 | `(unknown callee) 0x1b2d4` | `sp-284` | `0x1327ec` | `?` | `sp-284` |
| `bl` at 0xc12e4 | `(unknown callee) 0x1aff8` | `sp-284` | `0x1327a8` | `?` | `sp-284` |
| `bl` at 0xc1300 | `(unknown callee) 0x1b2d4` | `sp-284` | `0x1327b4` | `?` | `sp-284` |
| `bl` at 0xc1310 | `(unknown callee) 0x1b2d4` | `sp-284` | `0x1327f4` | `?` | `sp-284` |
| `bl` at 0xc1334 | `(unknown callee) 0x1aff8` | `sp-284` | `0x1327a8` | `?` | `sp-284` |
| `bl` at 0xc1350 | `(unknown callee) 0x1b2d4` | `sp-284` | `0x1327b4` | `?` | `sp-284` |
| `bl` at 0xc1360 | `(unknown callee) 0x1b2d4` | `sp-284` | `0x132818` | `?` | `sp-284` |
| `bl` at 0xc1370 | `(unknown callee) 0x1b508` | `sp-156` | `0x1327a8` | `?` | `sp-156` |
| `bl` at 0xc13fc | `(unknown callee) 0x1b49c` | `sp-284` | `0x132824` | `[[sp-28]+26]:2` | `sp-284` |
| `bl` at 0xc1410 | `(unknown callee) 0x1b49c` | `sp-156` | `0x132854` | `[sp-932]` | `sp-156` |
| `bl` at 0xc1464 | `(unknown callee) 0x1b49c` | `sp-284` | `0x132884` | `[[sp-28]+26]:2` | `sp-284` |
| `bl` at 0xc1478 | `(unknown callee) 0x1b49c` | `sp-156` | `0x132854` | `[sp-932]` | `sp-156` |
| `bl` at 0xc1490 | `(unknown callee) 0x1b49c` | `sp-156` | `0x132854` | `[sp-932]` | `sp-156` |
| `bl` at 0xc14b4 | `(unknown callee) 0x1b49c` | `sp-284` | `0x1328ac` | `?` | `sp-284` |
| `bl` at 0xc14c4 | `(unknown callee) 0x1b508` | `sp-412` | `0x1327a8` | `?` | `sp-412` |
| `bl` at 0xc14d4 | `(unknown callee) 0x1b508` | `sp-540` | `0x1327a8` | `?` | `sp-540` |
| `bl` at 0xc14e4 | `(unknown callee) 0x1b508` | `sp-668` | `0x1327a8` | `?` | `sp-668` |
| `bl` at 0xc14f4 | `(unknown callee) 0x1b508` | `sp-796` | `0x1327a8` | `?` | `sp-796` |
| `bl` at 0xc1518 | `(unknown callee) 0x1b2d4` | `sp-412` | `0x1328e4` | `?` | `sp-412` |
| `bl` at 0xc1528 | `(unknown callee) 0x1b2d4` | `sp-540` | `0x1328e4` | `?` | `sp-540` |
| `bl` at 0xc1538 | `(unknown callee) 0x1b2d4` | `sp-668` | `0x1328e4` | `?` | `sp-668` |
| `bl` at 0xc1548 | `(unknown callee) 0x1b2d4` | `sp-796` | `0x1328e4` | `?` | `sp-796` |
| `bl` at 0xc1630 | `(unknown callee) 0xc057c` | `[[sp-28]+33]:1` | `?` | `?` | `[[sp-28]+33]:1` |
| `bl` at 0xc1668 | `(unknown callee) 0xc057c` | `[[sp-28]+34]:1` | `?` | `?` | `[[sp-28]+34]:1` |
| `bl` at 0xc1690 | `(unknown callee) 0xc057c` | `[[sp-28]+35]:1` | `?` | `?` | `[[sp-28]+35]:1` |
| `bl` at 0xc16b8 | `(unknown callee) 0xc057c` | `[[sp-28]+36]:1` | `?` | `?` | `[[sp-28]+36]:1` |
| `bl` at 0xc1758 | `(unknown callee) 0x1b49c` | `sp-412` | `0x1328e8` | `?` | `?` |
| `bl` at 0xc17a4 | `(unknown callee) 0x1b49c` | `sp-412` | `0x132918` | `?` | `[[sp-28]+12]:1` |
| `bl` at 0xc1818 | `(unknown callee) 0x1b49c` | `sp-412` | `0x132918` | `?` | `[[sp-28]+12]:1` |
| `bl` at 0xc186c | `(unknown callee) 0x1b49c` | `sp-540` | `0x132918` | `?` | `[[sp-28]+12]:1` |
| `bl` at 0xc18c0 | `(unknown callee) 0x1b49c` | `sp-668` | `0x132918` | `?` | `[[sp-28]+12]:1` |
| `bl` at 0xc1914 | `(unknown callee) 0x1b49c` | `sp-796` | `0x132918` | `?` | `[[sp-28]+12]:1` |
| `bl` at 0xc1954 | `(unknown callee) 0x1b49c` | `sp-924` | `0x132948` | `([[sp-28]+28]:2 + [[sp-28]+30]:2)` | `[[sp-28]+24]:2` |
| `bl` at 0xc1960 | `(unknown callee) 0x1b22c` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xc1980 | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x1b22c)` | `sp-924` |
| `bl` at 0xc19b4 | `(unknown callee) 0x1b49c` | `sp-924` | `0x132960` | `[[sp-28]+24]:2` | `[[sp-28]+8]:1` |
| `bl` at 0xc19c0 | `(unknown callee) 0x1b22c` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xc19e0 | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x1b22c)` | `sp-924` |
| `bl` at 0xc19f0 | `(unknown callee) 0x1aff8` | `sp-156` | `0x1327a8` | `?` | `sp-156` |
| `bl` at 0xc1a10 | `(unknown callee) 0x1b49c` | `sp-924` | `0x132990` | `sp-156` | `sp-924` |
| `bl` at 0xc1a1c | `(unknown callee) 0x1b22c` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xc1a3c | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x1b22c)` | `sp-924` |
| `bl` at 0xc1a4c | `(unknown callee) 0x1aff8` | `sp-412` | `0x1327a8` | `?` | `sp-412` |
| `bl` at 0xc1a68 | `(unknown callee) 0x1aff8` | `sp-412` | `0x1328e4` | `?` | `sp-412` |
| `bl` at 0xc1a88 | `(unknown callee) 0x1b49c` | `sp-924` | `0x132990` | `sp-412` | `sp-924` |
| `bl` at 0xc1a94 | `(unknown callee) 0x1b22c` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xc1ab4 | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x1b22c)` | `sp-924` |
| `bl` at 0xc1ac4 | `(unknown callee) 0x1aff8` | `sp-540` | `0x1327a8` | `?` | `sp-540` |
| `bl` at 0xc1ae0 | `(unknown callee) 0x1aff8` | `sp-540` | `0x1328e4` | `?` | `sp-540` |
| `bl` at 0xc1b00 | `(unknown callee) 0x1b49c` | `sp-924` | `0x132990` | `sp-540` | `sp-924` |
| `bl` at 0xc1b0c | `(unknown callee) 0x1b22c` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xc1b2c | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x1b22c)` | `sp-924` |
| `bl` at 0xc1b3c | `(unknown callee) 0x1aff8` | `sp-668` | `0x1327a8` | `?` | `sp-668` |
| `bl` at 0xc1b58 | `(unknown callee) 0x1aff8` | `sp-668` | `0x1328e4` | `?` | `sp-668` |
| `bl` at 0xc1b78 | `(unknown callee) 0x1b49c` | `sp-924` | `0x132990` | `sp-668` | `sp-924` |
| `bl` at 0xc1b84 | `(unknown callee) 0x1b22c` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xc1ba4 | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x1b22c)` | `sp-924` |
| `bl` at 0xc1bb4 | `(unknown callee) 0x1aff8` | `sp-796` | `0x1327a8` | `?` | `sp-796` |
| `bl` at 0xc1bd0 | `(unknown callee) 0x1aff8` | `sp-796` | `0x1328e4` | `?` | `sp-796` |
| `bl` at 0xc1bf0 | `(unknown callee) 0x1b49c` | `sp-924` | `0x132990` | `sp-796` | `sp-924` |
| `bl` at 0xc1bfc | `(unknown callee) 0x1b22c` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xc1c1c | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x1b22c)` | `sp-924` |
| `bl` at 0xc1c2c | `(unknown callee) 0x1b49c` | `sp-924` | `0x132998` | `?` | `sp-924` |
| `bl` at 0xc1c38 | `(unknown callee) 0x1b22c` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xc1c58 | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x1b22c)` | `sp-924` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc1170 | `[[sp-28]+13]:1 != 0x0` | `[[sp-28]+13]:1 == 0x0` |
| `beq` at 0xc1194 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc11b0 | `ret(0x1aff8) == 0x0` | `ret(0x1aff8) != 0x0` |
| `beq` at 0xc11e4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc1200 | `ret(0x1aff8) == 0x0` | `ret(0x1aff8) != 0x0` |
| `beq` at 0xc1234 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc1250 | `ret(0x1aff8) == 0x0` | `ret(0x1aff8) != 0x0` |
| `beq` at 0xc1284 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc12a0 | `ret(0x1aff8) == 0x0` | `ret(0x1aff8) != 0x0` |
| `beq` at 0xc12d4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc12f0 | `ret(0x1aff8) == 0x0` | `ret(0x1aff8) != 0x0` |
| `beq` at 0xc1324 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc1340 | `ret(0x1aff8) == 0x0` | `ret(0x1aff8) != 0x0` |
| `beq` at 0xc1384 | `[[sp-28]+26]:2 == 0xffff` | `[[sp-28]+26]:2 != 0xffff` |
| `beq` at 0xc1398 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xc13bc | `[[sp-28]+33]:1 != 0x77` | `[[sp-28]+33]:1 == 0x77` |
| `bne` at 0xc13cc | `[[sp-28]+34]:1 != 0x77` | `[[sp-28]+34]:1 == 0x77` |
| `bne` at 0xc13dc | `[[sp-28]+35]:1 != 0x77` | `[[sp-28]+35]:1 == 0x77` |
| `bne` at 0xc13ec | `[[sp-28]+36]:1 != 0x77` | `[[sp-28]+36]:1 == 0x77` |
| `bne` at 0xc1424 | `[[sp-28]+33]:1 != 0x78` | `[[sp-28]+33]:1 == 0x78` |
| `bne` at 0xc1434 | `[[sp-28]+34]:1 != 0x78` | `[[sp-28]+34]:1 == 0x78` |
| `bne` at 0xc1444 | `[[sp-28]+35]:1 != 0x78` | `[[sp-28]+35]:1 == 0x78` |
| `bne` at 0xc1454 | `[[sp-28]+36]:1 != 0x78` | `[[sp-28]+36]:1 == 0x78` |
| `beq` at 0xc14a4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc1508 | `? == 0x0` | `? != 0x0` |
| `bhi` at 0xc1560 | `[[sp-28]+32]:1 > 0x6 (unsigned, 32-bit)` | `[[sp-28]+32]:1 <= 0x6 (unsigned, 32-bit)` |
| `bhi` at 0xc1570 | `[[sp-28]+20]:1 > 0x2 (unsigned, 32-bit)` | `[[sp-28]+20]:1 <= 0x2 (unsigned, 32-bit)` |
| `bhi` at 0xc15bc | `[[sp-28]+21]:1 > 0x2 (unsigned, 32-bit)` | `[[sp-28]+21]:1 <= 0x2 (unsigned, 32-bit)` |
| `beq` at 0xc15cc | `[[sp-28]+37]:1 == 0x6` | `[[sp-28]+37]:1 != 0x6` |
| `beq` at 0xc15dc | `[[sp-28]+37]:1 == 0x8` | `[[sp-28]+37]:1 != 0x8` |
| `beq` at 0xc15fc | `[[sp-28]+37]:1 == 0xa` | `[[sp-28]+37]:1 != 0xa` |
| `beq` at 0xc160c | `[[sp-28]+37]:1 == 0x16` | `[[sp-28]+37]:1 != 0x16` |
| `bhi` at 0xc1620 | `[[sp-28]+33]:1 > 0x7e (unsigned, 32-bit)` | `[[sp-28]+33]:1 <= 0x7e (unsigned, 32-bit)` |
| `bne` at 0xc1648 | `[[sp-28]+37]:1 != 0x16` | `[[sp-28]+37]:1 == 0x16` |
| `bhi` at 0xc1658 | `[[sp-28]+34]:1 > 0x7e (unsigned, 32-bit)` | `[[sp-28]+34]:1 <= 0x7e (unsigned, 32-bit)` |
| `bhi` at 0xc1680 | `[[sp-28]+35]:1 > 0x7e (unsigned, 32-bit)` | `[[sp-28]+35]:1 <= 0x7e (unsigned, 32-bit)` |
| `bhi` at 0xc16a8 | `[[sp-28]+36]:1 > 0x7e (unsigned, 32-bit)` | `[[sp-28]+36]:1 <= 0x7e (unsigned, 32-bit)` |
| `bhi` at 0xc16cc | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0xc16dc | `[[sp-28]+37]:1 == 0x16` | `[[sp-28]+37]:1 != 0x16` |
| `bne` at 0xc16ec | `[[sp-28]+33]:1 != 0x78` | `[[sp-28]+33]:1 == 0x78` |
| `bne` at 0xc16fc | `[[sp-28]+34]:1 != 0x78` | `[[sp-28]+34]:1 == 0x78` |
| `bne` at 0xc170c | `[[sp-28]+35]:1 != 0x78` | `[[sp-28]+35]:1 == 0x78` |
| `bne` at 0xc171c | `[[sp-28]+36]:1 != 0x78` | `[[sp-28]+36]:1 == 0x78` |
| `bhi` at 0xc17b4 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0xc17c4 | `[[sp-28]+37]:1 != 0x16` | `[[sp-28]+37]:1 == 0x16` |
| `bhi` at 0xc17d0 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bhi` at 0xc1824 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bhi` at 0xc1878 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bhi` at 0xc18cc | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0xc19fc | `ret(0x1aff8) == 0x0` | `ret(0x1aff8) != 0x0` |
| `beq` at 0xc1a58 | `ret(0x1aff8) == 0x0` | `ret(0x1aff8) != 0x0` |
| `beq` at 0xc1a74 | `ret(0x1aff8) == 0x0` | `ret(0x1aff8) != 0x0` |
| `beq` at 0xc1ad0 | `ret(0x1aff8) == 0x0` | `ret(0x1aff8) != 0x0` |
| `beq` at 0xc1aec | `ret(0x1aff8) == 0x0` | `ret(0x1aff8) != 0x0` |
| `beq` at 0xc1b48 | `ret(0x1aff8) == 0x0` | `ret(0x1aff8) != 0x0` |
| `beq` at 0xc1b64 | `ret(0x1aff8) == 0x0` | `ret(0x1aff8) != 0x0` |
| `beq` at 0xc1bc0 | `ret(0x1aff8) == 0x0` | `ret(0x1aff8) != 0x0` |
| `beq` at 0xc1bdc | `ret(0x1aff8) == 0x0` | `ret(0x1aff8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc1c5c | `r0` | `ret(unknown)` |

## `FRUPackData`

- `FRUPackData`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc1cec to 0xc2a14.
  Established. Four byte stores write values `0x6`, `0x8`, `0x0` and `0x0` at `[[sp+28]]
  +0x0`, `[[sp+28]] +0x1`, `[[sp+28]] +0x2` and `[[sp+28]] +0x3`, respectively (`strb` at 0xc2038,
  `strb` at 0xc2054, `strb` at 0xc2070, `strb` at 0xc208c). Earlier, a word store initializes
  `[sp+32] +0x0` to `0x1` (`str` at 0xc1d14). The call to `0x1b22c` has `[sp+16]`, unknown,
  `[sp+32]` and `[sp+16]` as candidates (`bl` at 0xc1ef0). Header meanings and buffer validity
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc1cfc | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xc1d00 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xc1d04 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xc1d08 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0xc1d0c | load | 4 | `sp +0x20` | `-` |
| `str` at 0xc1d14 | store | 4 | `[sp+32] +0x0` | `0x1` |
| `ldr` at 0xc1d18 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1d1c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1d20 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc1d28 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+1` |
| `ldr` at 0xc1d2c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1d30 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1d34 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc1d3c | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+1` |
| `ldr` at 0xc1d40 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1d44 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1d48 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc1d50 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+1` |
| `ldr` at 0xc1d54 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1d58 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1d5c | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc1d64 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xc1d68 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1d6c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1d70 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc1d78 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xc1d7c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1d80 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1d84 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc1d8c | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xc1d90 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1d94 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1d98 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc1da0 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xc1da4 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1da8 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1dac | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc1db4 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xc1db8 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1dbc | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1dc0 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc1dc8 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xc1dcc | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1dd0 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1dd4 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc1ddc | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `str` at 0xc1de4 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xc1de8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc1dec | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xc1dfc | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1e00 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1e04 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc1e0c | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xc1e10 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1e14 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1e18 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc1e20 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xc1e24 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1e28 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1e2c | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc1e34 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xc1e38 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1e3c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1e40 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc1e54 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc1e5c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc1e64 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc1e6c | store | 4 | `[sp+32] +0x0` | `?` |
| `ldr` at 0xc1e70 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1e74 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1e78 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc1e8c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc1e94 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc1e9c | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc1ea4 | store | 4 | `[sp+32] +0x0` | `?` |
| `ldr` at 0xc1ea8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xc1eb0 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xc1eb8 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1ebc | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1ec0 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc1ec8 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xc1ecc | load | 4 | `sp +0x20` | `-` |
| `str` at 0xc1ed0 | store | 4 | `sp -0x38` | `[sp+32]` |
| `ldr` at 0xc1ed4 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1ed8 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc1edc | store | 4 | `sp -0x3c` | `[[sp+32]]` |
| `ldr` at 0xc1ee0 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc1eec | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc1ef8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xc1f00 | store | 4 | `sp -0x40` | `([sp-60] + ret(0x1b22c))` |
| `ldr` at 0xc1f04 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xc1f0c | store | 4 | `sp -0x40` | `([sp-60] + ret(0x1b22c))+1` |
| `ldr` at 0xc1f14 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xc1f18 | store | 4 | `sp -0x40` | `[[sp+32]]` |
| `ldr` at 0xc1f1c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc1f20 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xc1f24 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xc1f28 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1f2c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1f30 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc1f38 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xc1f3c | load | 4 | `sp +0x20` | `-` |
| `str` at 0xc1f40 | store | 4 | `sp -0x44` | `[sp+32]` |
| `ldr` at 0xc1f44 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1f48 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc1f4c | store | 4 | `sp -0x48` | `[[sp+32]]` |
| `ldr` at 0xc1f50 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc1f5c | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc1f68 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xc1f70 | store | 4 | `sp -0x4c` | `([sp-72] + ret(0x1b22c))` |
| `ldr` at 0xc1f74 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xc1f7c | store | 4 | `sp -0x4c` | `([sp-72] + ret(0x1b22c))+1` |
| `ldr` at 0xc1f84 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xc1f88 | store | 4 | `sp -0x4c` | `[[sp+32]]` |
| `ldr` at 0xc1f8c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xc1f90 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xc1f94 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xc1f98 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1f9c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1fa0 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc1fa8 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xc1fac | load | 4 | `sp +0x20` | `-` |
| `str` at 0xc1fb0 | store | 4 | `sp -0x50` | `[sp+32]` |
| `ldr` at 0xc1fb4 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc1fb8 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc1fbc | store | 4 | `sp -0x54` | `[[sp+32]]` |
| `ldr` at 0xc1fc0 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xc1fcc | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xc1fd8 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xc1fe0 | store | 4 | `sp -0x58` | `([sp-84] + ret(0x1b22c))` |
| `ldr` at 0xc1fe4 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xc1fec | store | 4 | `sp -0x58` | `([sp-84] + ret(0x1b22c))+1` |
| `ldr` at 0xc1ff4 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xc1ff8 | store | 4 | `sp -0x58` | `[[sp+32]]` |
| `ldr` at 0xc1ffc | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xc2000 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xc2004 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xc2008 | load | 4 | `sp +0x1c` | `-` |
| `ldr` at 0xc200c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc2010 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc201c | store | 4 | `[sp+28] +0x0` | `ret(0x1b124)` |
| `ldr` at 0xc2020 | load | 4 | `[sp+28] +0x0` | `-` |
| `str` at 0xc2024 | store | 4 | `sp -0x28` | `[[sp+28]]` |
| `ldr` at 0xc202c | load | 4 | `sp -0x28` | `-` |
| `strb` at 0xc2038 | store | 1 | `[[sp+28]] +0x0` | `0x6` |
| `str` at 0xc2040 | store | 4 | `sp -0x28` | `[[sp+28]]+1` |
| `ldr` at 0xc2048 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0xc2054 | store | 1 | `[[sp+28]] +0x1` | `0x8` |
| `str` at 0xc205c | store | 4 | `sp -0x28` | `[[sp+28]]+2` |
| `ldr` at 0xc2064 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0xc2070 | store | 1 | `[[sp+28]] +0x2` | `0x0` |
| `str` at 0xc2078 | store | 4 | `sp -0x28` | `[[sp+28]]+3` |
| `ldr` at 0xc2080 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0xc208c | store | 1 | `[[sp+28]] +0x3` | `0x0` |
| `str` at 0xc2094 | store | 4 | `sp -0x28` | `[[sp+28]]+4` |
| `ldr` at 0xc2098 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc209c | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xc20a4 | store | 1 | `[[sp+28]] +0x4` | `?` |
| `ldr` at 0xc20ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc20b4 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xc20bc | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xc20c4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc20cc | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xc20d4 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xc20dc | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xc20e4 | load | 1 | `sp +0x0` | `-` |
| `strb` at 0xc20e8 | store | 1 | `[sp-40] +0x3` | `([sp]:1 & 0xff)` |
| `ldr` at 0xc20ec | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc20f4 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xc20f8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc20fc | load | 4 | `sp +0x4` | `-` |
| `strb` at 0xc2104 | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xc210c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2114 | load | 4 | `sp +0x4` | `-` |
| `strb` at 0xc211c | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xc2124 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc212c | load | 4 | `sp +0x4` | `-` |
| `strb` at 0xc2134 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xc213c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xc2144 | load | 1 | `sp +0x4` | `-` |
| `strb` at 0xc2148 | store | 1 | `[sp-40] +0x3` | `([sp+4]:1 & 0xff)` |
| `ldr` at 0xc214c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc2154 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xc2158 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc215c | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xc2164 | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xc216c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2174 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xc217c | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xc2184 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc218c | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xc2194 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xc219c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xc21a4 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xc21a8 | store | 1 | `[sp-40] +0x3` | `([sp-24]:1 & 0xff)` |
| `ldr` at 0xc21ac | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc21b4 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xc21b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc21bc | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xc21c4 | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xc21cc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc21d4 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xc21dc | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xc21e4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc21ec | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xc21f4 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xc21fc | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xc2204 | load | 1 | `sp -0x1c` | `-` |
| `strb` at 0xc2208 | store | 1 | `[sp-40] +0x3` | `([sp-28]:1 & 0xff)` |
| `ldr` at 0xc220c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc2214 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xc2218 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc221c | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xc2224 | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xc222c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2234 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xc223c | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xc2244 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc224c | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xc2254 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xc225c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xc2264 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0xc2268 | store | 1 | `[sp-40] +0x3` | `([sp-32]:1 & 0xff)` |
| `ldr` at 0xc226c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc2274 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xc2278 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc227c | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xc2284 | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xc228c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2294 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xc229c | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xc22a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc22ac | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xc22b4 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xc22bc | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xc22c4 | load | 1 | `sp -0x24` | `-` |
| `strb` at 0xc22c8 | store | 1 | `[sp-40] +0x3` | `([sp-36]:1 & 0xff)` |
| `ldr` at 0xc22cc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc22d4 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xc22d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc22dc | load | 4 | `sp +0x8` | `-` |
| `strb` at 0xc22e4 | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xc22ec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc22f4 | load | 4 | `sp +0x8` | `-` |
| `strb` at 0xc22fc | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xc2304 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc230c | load | 4 | `sp +0x8` | `-` |
| `strb` at 0xc2314 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xc231c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xc2324 | load | 1 | `sp +0x8` | `-` |
| `strb` at 0xc2328 | store | 1 | `[sp-40] +0x3` | `([sp+8]:1 & 0xff)` |
| `ldr` at 0xc232c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc2334 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `str` at 0xc233c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xc2340 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc2344 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xc2354 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2358 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc236c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc2374 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xc237c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xc2384 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc238c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc23a0 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc23a8 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xc23b0 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xc23b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc23c0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc23d4 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc23dc | load | 4 | `(unknown)` | `-` |
| `strb` at 0xc23e4 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xc23ec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc23f4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc2408 | load | 4 | `sp +0xc` | `-` |
| `ldrb` at 0xc2410 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xc2414 | store | 1 | `[sp-40] +0x3` | `?` |
| `ldr` at 0xc2418 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc2420 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xc2424 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2428 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc243c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc2444 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xc244c | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xc2454 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc245c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc2470 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc2478 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xc2480 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xc2488 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2490 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc24a4 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc24ac | load | 4 | `(unknown)` | `-` |
| `strb` at 0xc24b4 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xc24bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc24c4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc24d8 | load | 4 | `sp +0xc` | `-` |
| `ldrb` at 0xc24e0 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xc24e4 | store | 1 | `[sp-40] +0x3` | `?` |
| `ldr` at 0xc24e8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc24f0 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xc24f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc24f8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc250c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc2514 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xc251c | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xc2524 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc252c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc2540 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc2548 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xc2550 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xc2558 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2560 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc2574 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc257c | load | 4 | `(unknown)` | `-` |
| `strb` at 0xc2584 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xc258c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2594 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc25a8 | load | 4 | `sp +0xc` | `-` |
| `ldrb` at 0xc25b0 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xc25b4 | store | 1 | `[sp-40] +0x3` | `?` |
| `ldr` at 0xc25b8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc25c0 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `str` at 0xc25c8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xc25cc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc25e0 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc25e8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc25ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc25fc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2600 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc2614 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc261c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc2630 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc2638 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xc2640 | store | 1 | `[sp-40] +0x0` | `?` |
| `ldr` at 0xc2648 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2650 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc2664 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc266c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc2680 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc2688 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xc2690 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xc2698 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc26a0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc26b4 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc26bc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc26d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc26d8 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xc26e0 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xc26e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc26f0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc2704 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc270c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc2720 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xc2728 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xc272c | store | 1 | `[sp-40] +0x3` | `?` |
| `ldr` at 0xc2730 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc2738 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xc273c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc2744 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xc274c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc2760 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc2768 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc277c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc2784 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc278c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2790 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc279c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc27b0 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc27b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc27c0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc27c8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xc27cc | load | 4 | `sp -0x30` | `-` |
| `str` at 0xc27d4 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xc27dc | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc27e8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xc27f4 | store | 4 | `sp -0x5c` | `ret(0x1b22c)` |
| `ldr` at 0xc27f8 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xc2800 | store | 4 | `sp -0x5c` | `ret(0x1b22c)+1` |
| `str` at 0xc280c | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xc2810 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xc2814 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xc2818 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc281c | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xc2824 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xc282c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2834 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xc283c | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xc2844 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc284c | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xc2854 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xc285c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xc2864 | load | 1 | `sp -0x34` | `-` |
| `strb` at 0xc2868 | store | 1 | `[sp-40] +0x3` | `([sp-52]:1 & 0xff)` |
| `ldr` at 0xc286c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc2874 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xc2878 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc287c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc2880 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc2888 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc288c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xc2894 | store | 4 | `sp -0x28` | `([sp-40] + [sp-52])` |
| `ldr` at 0xc2898 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc28a4 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xc28b0 | store | 4 | `sp -0x60` | `ret(0x1b22c)` |
| `ldr` at 0xc28b4 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xc28bc | store | 4 | `sp -0x60` | `ret(0x1b22c)+1` |
| `str` at 0xc28c8 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xc28cc | load | 4 | `sp -0x60` | `-` |
| `str` at 0xc28d0 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xc28d4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc28d8 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xc28e0 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xc28e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc28f0 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xc28f8 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xc2900 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2908 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xc2910 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xc2918 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xc2920 | load | 1 | `sp -0x34` | `-` |
| `strb` at 0xc2924 | store | 1 | `[sp-40] +0x3` | `([sp-52]:1 & 0xff)` |
| `ldr` at 0xc2928 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc2930 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xc2934 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2938 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc293c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc2944 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2948 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xc2950 | store | 4 | `sp -0x28` | `([sp-40] + [sp-52])` |
| `ldr` at 0xc2954 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xc2960 | load | 4 | `sp +0x18` | `-` |
| `str` at 0xc296c | store | 4 | `sp -0x64` | `ret(0x1b22c)` |
| `ldr` at 0xc2970 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xc2978 | store | 4 | `sp -0x64` | `ret(0x1b22c)+1` |
| `str` at 0xc2984 | store | 4 | `sp -0x64` | `0x0` |
| `ldr` at 0xc2988 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xc298c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xc2990 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2994 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xc299c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xc29a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc29ac | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xc29b4 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xc29bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc29c4 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xc29cc | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xc29d4 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xc29dc | load | 1 | `sp -0x34` | `-` |
| `strb` at 0xc29e0 | store | 1 | `[sp-40] +0x3` | `([sp-52]:1 & 0xff)` |
| `ldr` at 0xc29e4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc29ec | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xc29f0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc29f4 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xc29f8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc2a00 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2a04 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xc2a0c | store | 4 | `sp -0x28` | `([sp-40] + [sp-52])` |
| `ldmdb` at 0xc2a10 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc2a10 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc2a10 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc2a10 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc1ef0 | `(unknown callee) 0x1b22c` | `[sp+16]` | `?` | `[sp+32]` | `[sp+16]` |
| `bl` at 0xc1f60 | `(unknown callee) 0x1b22c` | `[sp+20]` | `?` | `[sp+32]` | `[sp+20]` |
| `bl` at 0xc1fd0 | `(unknown callee) 0x1b22c` | `[sp+24]` | `?` | `[sp+32]` | `[sp+24]` |
| `bl` at 0xc2014 | `(unknown callee) 0x1b124` | `[[sp+32]]` | `?` | `?` | `[sp+32]` |
| `bl` at 0xc2798 | `(unknown callee) 0x1b304` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xc27ec | `(unknown callee) 0x1b22c` | `[sp+16]` | `?` | `?` | `[sp+16]` |
| `bl` at 0xc2884 | `(unknown callee) 0x1b304` | `[sp-40]+4` | `[sp+16]` | `[sp-52]` | `[sp-40]+4` |
| `bl` at 0xc28a8 | `(unknown callee) 0x1b22c` | `[sp+20]` | `?` | `[sp-40]` | `[sp+20]` |
| `bl` at 0xc2940 | `(unknown callee) 0x1b304` | `[sp-40]+4` | `[sp+20]` | `[sp-52]` | `[sp-40]+4` |
| `bl` at 0xc2964 | `(unknown callee) 0x1b22c` | `[sp+24]` | `?` | `[sp-40]` | `[sp+24]` |
| `bl` at 0xc29fc | `(unknown callee) 0x1b304` | `[sp-40]+4` | `[sp+24]` | `[sp-52]` | `[sp-40]+4` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xc1df4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xc1ee8 | `[sp+16] == 0x0` | `[sp+16] != 0x0` |
| `beq` at 0xc1f58 | `[sp+20] == 0x0` | `[sp+20] != 0x0` |
| `beq` at 0xc1fc8 | `[sp+24] == 0x0` | `[sp+24] != 0x0` |
| `blt` at 0xc234c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0xc25f4 | `[sp-44] < ? (signed, 32-bit)` | `[sp-44] >= ? (signed, 32-bit)` |
| `beq` at 0xc27e4 | `[sp+16] == 0x0` | `[sp+16] != 0x0` |
| `beq` at 0xc28a0 | `[sp+20] == 0x0` | `[sp+20] != 0x0` |
| `beq` at 0xc295c | `[sp+24] == 0x0` | `[sp+24] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc2a10 | `r0` | `ret(0x1b304)` |

## `FRUPackEvents`

- `FRUPackEvents`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc2a14 to 0xc2ee4.
  Established. A byte store places `0x6` at `[[sp+12]] +0x0` (`strb` at 0xc2b9c). The next byte is
  `([sp-52]:1 & 0xff)` at `[[sp+12]] +0x1` (`strb` at 0xc2bb8). Calls include `0x1b124` with
  candidates `[[sp+16]]`, `[sp+16]`, unknown and `[sp+16]` (`bl` at 0xc2b78), and `0x1b304` with
  candidate r2 `0xa8` while the other candidates are unknown (`bl` at 0xc2ec0). Event record content
  beyond these reads and writes remains unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc2a24 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xc2a28 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xc2a2c | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xc2a30 | store | 4 | `sp -0x24` | `r3` |
| `str` at 0xc2a38 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xc2a3c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xc2a40 | store | 4 | `sp -0x28` | `[sp+8]` |
| `ldr` at 0xc2a44 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2a54 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xc2a5c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xc2a60 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2a64 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc2a68 | store | 4 | `sp -0x28` | `?` |
| `str` at 0xc2a74 | store | 4 | `sp -0x34` | `0xa8` |
| `ldr` at 0xc2a78 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xc2a80 | store | 4 | `[sp+16] +0x0` | `0x1` |
| `ldr` at 0xc2a84 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc2a88 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc2a8c | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xc2a94 | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+1` |
| `ldr` at 0xc2a98 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc2a9c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc2aa0 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xc2aa8 | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+1` |
| `ldr` at 0xc2aac | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc2ab0 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc2ab4 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xc2abc | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+1` |
| `ldr` at 0xc2ac0 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc2ac4 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc2ac8 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xc2ad0 | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+4` |
| `ldr` at 0xc2ad4 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc2ad8 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc2adc | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xc2ae4 | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+4` |
| `ldr` at 0xc2ae8 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc2aec | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc2af0 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xc2af8 | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+4` |
| `ldr` at 0xc2afc | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc2b00 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc2b04 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xc2b0c | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+4` |
| `ldr` at 0xc2b10 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc2b14 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc2b18 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xc2b20 | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+4` |
| `ldr` at 0xc2b24 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc2b28 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc2b2c | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xc2b34 | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+4` |
| `ldr` at 0xc2b38 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc2b3c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc2b40 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xc2b48 | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+4` |
| `ldr` at 0xc2b4c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc2b50 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc2b54 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc2b58 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc2b60 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xc2b68 | store | 4 | `[sp+16] +0x0` | `?` |
| `ldr` at 0xc2b6c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc2b70 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc2b74 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xc2b80 | store | 4 | `[sp+12] +0x0` | `ret(0x1b124)` |
| `ldr` at 0xc2b84 | load | 4 | `[sp+12] +0x0` | `-` |
| `str` at 0xc2b88 | store | 4 | `sp -0x2c` | `[[sp+12]]` |
| `ldr` at 0xc2b90 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0xc2b9c | store | 1 | `[[sp+12]] +0x0` | `0x6` |
| `str` at 0xc2ba4 | store | 4 | `sp -0x2c` | `[[sp+12]]+1` |
| `ldr` at 0xc2bac | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xc2bb0 | load | 1 | `sp -0x34` | `-` |
| `strb` at 0xc2bb8 | store | 1 | `[[sp+12]] +0x1` | `([sp-52]:1 & 0xff)` |
| `str` at 0xc2bc0 | store | 4 | `sp -0x2c` | `[[sp+12]]+2` |
| `ldr` at 0xc2bc8 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0xc2bd4 | store | 1 | `[[sp+12]] +0x2` | `0x0` |
| `str` at 0xc2bdc | store | 4 | `sp -0x2c` | `[[sp+12]]+3` |
| `ldr` at 0xc2be4 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0xc2bf0 | store | 1 | `[[sp+12]] +0x3` | `0x0` |
| `str` at 0xc2bf8 | store | 4 | `sp -0x2c` | `[[sp+12]]+4` |
| `ldr` at 0xc2bfc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc2c00 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xc2c08 | store | 1 | `[[sp+12]] +0x4` | `?` |
| `ldr` at 0xc2c10 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc2c18 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xc2c20 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0xc2c28 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc2c30 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xc2c38 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0xc2c40 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xc2c48 | load | 1 | `sp +0x0` | `-` |
| `strb` at 0xc2c4c | store | 1 | `[sp-44] +0x3` | `([sp]:1 & 0xff)` |
| `ldr` at 0xc2c50 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc2c58 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0xc2c5c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc2c60 | load | 4 | `sp +0x4` | `-` |
| `strb` at 0xc2c68 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0xc2c70 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc2c78 | load | 4 | `sp +0x4` | `-` |
| `strb` at 0xc2c80 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0xc2c88 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc2c90 | load | 4 | `sp +0x4` | `-` |
| `strb` at 0xc2c98 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0xc2ca0 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xc2ca8 | load | 1 | `sp +0x4` | `-` |
| `strb` at 0xc2cac | store | 1 | `[sp-44] +0x3` | `([sp+4]:1 & 0xff)` |
| `ldr` at 0xc2cb0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc2cb8 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0xc2cbc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc2cc0 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xc2cc8 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0xc2cd0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc2cd8 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xc2ce0 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0xc2ce8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc2cf0 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xc2cf8 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0xc2d00 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xc2d08 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xc2d0c | store | 1 | `[sp-44] +0x3` | `([sp-24]:1 & 0xff)` |
| `ldr` at 0xc2d10 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc2d18 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0xc2d1c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc2d20 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xc2d28 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0xc2d30 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc2d38 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xc2d40 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0xc2d48 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc2d50 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xc2d58 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0xc2d60 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xc2d68 | load | 1 | `sp -0x1c` | `-` |
| `strb` at 0xc2d6c | store | 1 | `[sp-44] +0x3` | `([sp-28]:1 & 0xff)` |
| `ldr` at 0xc2d70 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc2d78 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0xc2d7c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc2d80 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xc2d88 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0xc2d90 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc2d98 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xc2da0 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0xc2da8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc2db0 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xc2db8 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0xc2dc0 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xc2dc8 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0xc2dcc | store | 1 | `[sp-44] +0x3` | `([sp-32]:1 & 0xff)` |
| `ldr` at 0xc2dd0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc2dd8 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0xc2ddc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc2de0 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xc2de8 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0xc2df0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc2df8 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xc2e00 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0xc2e08 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc2e10 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xc2e18 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0xc2e20 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xc2e28 | load | 1 | `sp -0x24` | `-` |
| `strb` at 0xc2e2c | store | 1 | `[sp-44] +0x3` | `([sp-36]:1 & 0xff)` |
| `ldr` at 0xc2e30 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc2e38 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0xc2e3c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc2e40 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0xc2e48 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0xc2e50 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc2e58 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0xc2e60 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0xc2e68 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc2e70 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0xc2e78 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0xc2e80 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xc2e88 | load | 1 | `sp -0x30` | `-` |
| `strb` at 0xc2e8c | store | 1 | `[sp-44] +0x3` | `([sp-48]:1 & 0xff)` |
| `ldr` at 0xc2e90 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc2e98 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0xc2e9c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xc2ea0 | store | 4 | `sp -0x28` | `[sp+8]` |
| `ldr` at 0xc2ea4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2eb4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc2eb8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2ec4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc2ecc | store | 4 | `sp -0x2c` | `[sp-44]+168` |
| `ldr` at 0xc2ed0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2ed4 | load | 4 | `[sp-40] +0xa8` | `-` |
| `str` at 0xc2ed8 | store | 4 | `sp -0x28` | `[[sp-40]+168]` |
| `ldmdb` at 0xc2ee0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc2ee0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc2ee0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc2ee0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc2b78 | `(unknown callee) 0x1b124` | `[[sp+16]]` | `[sp+16]` | `?` | `[sp+16]` |
| `bl` at 0xc2ec0 | `(unknown callee) 0x1b304` | `?` | `?` | `0xa8` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc2a4c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xc2eac | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc2ee0 | `r0` | `?` |

## `FRUUnpackData`

- `FRUUnpackData`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc2ee4 to 0xc3994.
  Established. Four word stores clear `[sp+16] +0x0`, `[sp+20] +0x0`, `[sp+24] +0x0` and `[sp+28]
  +0x0` (`str` at 0xc2f0c, `str` at 0xc2f18, `str` at 0xc2f24, `str` at 0xc2f30). A byte at `[sp-24]
  +0x0` is loaded and stored as the word `[[sp-24]]:1` at `[sp-28] +0x0` (`ldrb` at 0xc2f44, `str`
  at 0xc2f48). The branch takes the `[[sp-28]] == 0x6` arm and falls through on inequality (`beq` at
  0xc2f60). That fall-through path writes word `0x5` at `[sp+12] +0x0` (`str` at 0xc2f6c). The
  error-code interpretation is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc2ef4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xc2ef8 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xc2efc | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xc2f00 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0xc2f04 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xc2f0c | store | 4 | `[sp+16] +0x0` | `0x0` |
| `ldr` at 0xc2f10 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xc2f18 | store | 4 | `[sp+20] +0x0` | `0x0` |
| `ldr` at 0xc2f1c | load | 4 | `sp +0x18` | `-` |
| `str` at 0xc2f24 | store | 4 | `[sp+24] +0x0` | `0x0` |
| `ldr` at 0xc2f28 | load | 4 | `sp +0x1c` | `-` |
| `str` at 0xc2f30 | store | 4 | `[sp+28] +0x0` | `0x0` |
| `ldr` at 0xc2f34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc2f3c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc2f44 | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0xc2f48 | store | 4 | `[sp-28] +0x0` | `[[sp-24]]:1` |
| `str` at 0xc2f50 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0xc2f54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc2f58 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xc2f64 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xc2f6c | store | 4 | `[sp+12] +0x0` | `0x5` |
| `ldr` at 0xc2f78 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc2f80 | load | 1 | `[sp-24] +0x1` | `-` |
| `str` at 0xc2f84 | store | 4 | `sp -0x30` | `[[sp-24]+1]:1` |
| `str` at 0xc2f8c | store | 4 | `sp -0x18` | `[sp-24]+2` |
| `ldr` at 0xc2f94 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc2f9c | load | 1 | `[sp-24] +0x2` | `-` |
| `str` at 0xc2fa0 | store | 4 | `sp -0x34` | `[[sp-24]+2]:1` |
| `str` at 0xc2fa8 | store | 4 | `sp -0x18` | `[sp-24]+3` |
| `ldr` at 0xc2fb0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc2fb8 | load | 1 | `[sp-24] +0x3` | `-` |
| `str` at 0xc2fbc | store | 4 | `sp -0x38` | `[[sp-24]+3]:1` |
| `str` at 0xc2fc4 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xc2fc8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc2fcc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc2fd0 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xc2fdc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc2fe4 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xc2ff4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc2ffc | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xc300c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3014 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xc301c | store | 4 | `[sp-32] +0x0` | `?` |
| `ldr` at 0xc3020 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc3028 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xc302c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc3030 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3034 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xc3040 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3048 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xc3058 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3060 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xc3070 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3078 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xc3080 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0xc3084 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc308c | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xc3090 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xc3094 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3098 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xc30a4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc30ac | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xc30bc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc30c4 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xc30d4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc30dc | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xc30e4 | store | 4 | `[sp] +0x0` | `?` |
| `ldr` at 0xc30e8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc30f0 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xc30f4 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xc30f8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc30fc | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xc3108 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3110 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xc3120 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3128 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xc3138 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3140 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xc3148 | store | 4 | `[sp+4] +0x0` | `?` |
| `ldr` at 0xc314c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc3154 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xc3158 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xc315c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3160 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xc316c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3174 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xc3184 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc318c | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xc319c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc31a4 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xc31ac | store | 4 | `[sp+8] +0x0` | `?` |
| `ldr` at 0xc31b0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc31b8 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xc31bc | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc31c0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc31c4 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xc31d0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc31d8 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xc31e8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc31f0 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xc3200 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3208 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xc3210 | store | 4 | `[sp+12] +0x0` | `?` |
| `ldr` at 0xc3214 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc321c | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xc3220 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc3224 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3228 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xc3234 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc323c | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xc324c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3254 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xc3264 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc326c | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xc3274 | store | 4 | `[sp+16] +0x0` | `?` |
| `ldr` at 0xc3278 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc3280 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xc3284 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc3290 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xc3298 | store | 4 | `[sp+12] +0x0` | `0x6` |
| `ldr` at 0xc32a0 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc32a4 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc32a8 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xc32c8 | store | 4 | `[sp+20] +0x0` | `ret(0x1b124)` |
| `ldr` at 0xc32cc | load | 4 | `[sp+20] +0x0` | `-` |
| `str` at 0xc32d0 | store | 4 | `sp -0x28` | `[[sp+20]]` |
| `ldr` at 0xc32d4 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc32d8 | load | 4 | `[sp+20] +0x0` | `-` |
| `ldr` at 0xc32e4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xc32ec | store | 4 | `[sp+12] +0x0` | `0x8` |
| `str` at 0xc32f8 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xc32fc | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc3300 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc3304 | load | 4 | `[sp+16] +0x0` | `-` |
| `ldr` at 0xc3314 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc3328 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3330 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3334 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xc3340 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3348 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xc3358 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3360 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xc3370 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3378 | load | 1 | `[sp-24] +0x3` | `-` |
| `str` at 0xc3380 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xc3384 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc338c | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xc3390 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc33a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc33ac | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc33b0 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xc33bc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc33c4 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xc33d4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc33dc | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xc33ec | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc33f4 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xc33fc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xc3400 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc3408 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xc340c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc3420 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3428 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc342c | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xc3438 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3440 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xc3450 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3458 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xc3468 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3470 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xc3478 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xc347c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc3484 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xc3488 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc349c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc34a4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc34b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc34c0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc34d4 | store | 4 | `(unknown)` | `ret(0x1b124)` |
| `ldr` at 0xc34d8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc34ec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc34f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc3500 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xc3508 | store | 4 | `[sp+12] +0x0` | `0x8` |
| `ldr` at 0xc3510 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc3524 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc352c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc3540 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3548 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc3550 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc3564 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xc3568 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc357c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3584 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc3588 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc3598 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc359c | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xc35a8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc35b0 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xc35c0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc35c8 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xc35d8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc35e0 | load | 1 | `[sp-24] +0x3` | `-` |
| `str` at 0xc35e8 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xc35ec | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc35f4 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xc35f8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc360c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3614 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc361c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc3624 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xc3628 | store | 4 | `(unknown)` | `[sp-68]` |
| `ldr` at 0xc362c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc3634 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xc363c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc3650 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3658 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc366c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3674 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc3688 | store | 4 | `(unknown)` | `ret(0x1b124)` |
| `ldr` at 0xc368c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc36a0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc36a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc36b4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xc36bc | store | 4 | `[sp+12] +0x0` | `0x8` |
| `ldr` at 0xc36c4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc36d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc36e0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc36f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc36fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc3704 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc3714 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc3728 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3730 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc3744 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc374c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc3754 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc3758 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc3764 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc3778 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3780 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc3788 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc3790 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xc3794 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xc379c | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0xc37a4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc37a8 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xc37b4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc37bc | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xc37cc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc37d4 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xc37e4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc37ec | load | 1 | `[sp-24] +0x3` | `-` |
| `str` at 0xc37f4 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xc37f8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc3800 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xc3804 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc3810 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xc3814 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xc3820 | store | 4 | `[sp+24] +0x0` | `ret(0x1b124)` |
| `ldr` at 0xc3824 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xc3828 | load | 4 | `[sp+24] +0x0` | `-` |
| `ldr` at 0xc382c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc3830 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc3838 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc383c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xc3844 | store | 4 | `sp -0x18` | `([sp-24] + [sp-64])` |
| `ldr` at 0xc3848 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc384c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc3858 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3860 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc3870 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3878 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc3888 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3890 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc3898 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xc389c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc38a4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xc38a8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc38b4 | load | 4 | `sp +0x1c` | `-` |
| `ldr` at 0xc38b8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xc38c4 | store | 4 | `[sp+28] +0x0` | `ret(0x1b124)` |
| `ldr` at 0xc38c8 | load | 4 | `sp +0x1c` | `-` |
| `ldr` at 0xc38cc | load | 4 | `[sp+28] +0x0` | `-` |
| `ldr` at 0xc38d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc38d4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc38dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc38e0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xc38e8 | store | 4 | `sp -0x18` | `([sp-24] + [sp-64])` |
| `ldr` at 0xc38ec | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc38f0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc38fc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3904 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc3914 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc391c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc392c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3934 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc393c | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xc3940 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc3948 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xc394c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc3958 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc395c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xc3968 | store | 4 | `[sp+32] +0x0` | `ret(0x1b124)` |
| `ldr` at 0xc396c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc3970 | load | 4 | `[sp+32] +0x0` | `-` |
| `ldr` at 0xc3974 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc3978 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc3980 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc3984 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xc398c | store | 4 | `sp -0x18` | `([sp-24] + [sp-64])` |
| `ldmdb` at 0xc3990 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc3990 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc3990 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc3990 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc32c0 | `(unknown callee) 0x1b124` | `?` | `?` | `[[sp+16]]` | `?` |
| `bl` at 0xc34cc | `(unknown callee) 0x1b124` | `?` | `?` | `?` | `?` |
| `bl` at 0xc355c | `(unknown callee) 0x1b43c` | `?` | `0x0` | `?` | `?` |
| `bl` at 0xc3680 | `(unknown callee) 0x1b124` | `?` | `?` | `?` | `?` |
| `bl` at 0xc3710 | `(unknown callee) 0x1b43c` | `?` | `0x0` | `?` | `?` |
| `bl` at 0xc3760 | `(unknown callee) 0x1b304` | `?` | `[sp-24]` | `?` | `?` |
| `bl` at 0xc3818 | `(unknown callee) 0x1b124` | `[sp-64]` | `?` | `0x3` | `[sp-64]` |
| `bl` at 0xc3834 | `(unknown callee) 0x1b304` | `[[sp+24]]` | `[sp-24]` | `[sp-64]` | `[sp+24]` |
| `bl` at 0xc38bc | `(unknown callee) 0x1b124` | `[sp-64]` | `?` | `0x3` | `[sp-64]` |
| `bl` at 0xc38d8 | `(unknown callee) 0x1b304` | `[[sp+28]]` | `[sp-24]` | `[sp-64]` | `[sp+28]` |
| `bl` at 0xc3960 | `(unknown callee) 0x1b124` | `[sp-64]` | `?` | `0x3` | `[sp-64]` |
| `bl` at 0xc397c | `(unknown callee) 0x1b304` | `[[sp+32]]` | `[sp-24]` | `[sp-64]` | `[sp+32]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc2f60 | `[[sp-28]] == 0x6` | `[[sp-28]] != 0x6` |
| `beq` at 0xc328c | `[sp-48] == 0x8` | `[sp-48] != 0x8` |
| `bne` at 0xc32e0 | `[[sp+20]] != 0x0` | `[[sp+20]] == 0x0` |
| `blt` at 0xc330c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xc34fc | `? != 0x0` | `? == 0x0` |
| `blt` at 0xc3590 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xc36b0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xc380c | `[sp-64] == 0x0` | `[sp-64] != 0x0` |
| `beq` at 0xc38b0 | `[sp-64] == 0x0` | `[sp-64] != 0x0` |
| `beq` at 0xc3954 | `[sp-64] == 0x0` | `[sp-64] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc3990 | `r0` | `?` |

## `FRUUnpackEvents`

- `FRUUnpackEvents`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc3994 to 0xc3edc.
  Established. The first branch is taken for `[[sp-24]] == 0x6` and falls through
  otherwise (`beq` at 0xc39f8). The later branch takes `[sp-56] == 0xa8`, with inequality falling
  through (`beq` at 0xc3d24); its call to `0x1b124` has candidate r0 `0xac`, two unknown candidates
  and `[[sp+16]]` (`bl` at 0xc3d5c). Word stores include zero at `[sp-36] +0xa8` and `[sp-36]` at
  `[sp-40] +0xa8` (`str` at 0xc3d90, `str` at 0xc3ebc). Event-link ownership is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc39a4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc39a8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc39ac | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xc39b0 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xc39b4 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xc39bc | store | 4 | `[sp+16] +0x0` | `0x0` |
| `ldr` at 0xc39c0 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xc39c8 | store | 4 | `[sp+20] +0x0` | `0x0` |
| `ldr` at 0xc39cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc39d4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc39dc | load | 1 | `[sp-20] +0x0` | `-` |
| `str` at 0xc39e0 | store | 4 | `[sp-24] +0x0` | `[[sp-20]]:1` |
| `str` at 0xc39e8 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0xc39ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc39f0 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xc39fc | load | 4 | `sp +0xc` | `-` |
| `str` at 0xc3a04 | store | 4 | `[sp+12] +0x0` | `0x5` |
| `ldr` at 0xc3a10 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc3a18 | load | 1 | `[sp-20] +0x1` | `-` |
| `str` at 0xc3a1c | store | 4 | `sp -0x38` | `[[sp-20]+1]:1` |
| `str` at 0xc3a24 | store | 4 | `sp -0x14` | `[sp-20]+2` |
| `ldr` at 0xc3a2c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc3a34 | load | 1 | `[sp-20] +0x2` | `-` |
| `str` at 0xc3a38 | store | 4 | `sp -0x30` | `[[sp-20]+2]:1` |
| `str` at 0xc3a40 | store | 4 | `sp -0x14` | `[sp-20]+3` |
| `ldr` at 0xc3a48 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc3a50 | load | 1 | `[sp-20] +0x3` | `-` |
| `str` at 0xc3a54 | store | 4 | `sp -0x34` | `[[sp-20]+3]:1` |
| `str` at 0xc3a5c | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xc3a60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc3a64 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc3a68 | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xc3a74 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc3a7c | load | 1 | `[sp-20] +0x5` | `-` |
| `ldr` at 0xc3a8c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc3a94 | load | 1 | `[sp-20] +0x6` | `-` |
| `ldr` at 0xc3aa4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc3aac | load | 1 | `[sp-20] +0x7` | `-` |
| `str` at 0xc3ab4 | store | 4 | `[sp-28] +0x0` | `?` |
| `ldr` at 0xc3ab8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc3ac0 | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xc3ac4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc3ac8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc3acc | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xc3ad8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc3ae0 | load | 1 | `[sp-20] +0x5` | `-` |
| `ldr` at 0xc3af0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc3af8 | load | 1 | `[sp-20] +0x6` | `-` |
| `ldr` at 0xc3b08 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc3b10 | load | 1 | `[sp-20] +0x7` | `-` |
| `str` at 0xc3b18 | store | 4 | `[sp-32] +0x0` | `?` |
| `ldr` at 0xc3b1c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc3b24 | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xc3b28 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xc3b2c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc3b30 | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xc3b3c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc3b44 | load | 1 | `[sp-20] +0x5` | `-` |
| `ldr` at 0xc3b54 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc3b5c | load | 1 | `[sp-20] +0x6` | `-` |
| `ldr` at 0xc3b6c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc3b74 | load | 1 | `[sp-20] +0x7` | `-` |
| `str` at 0xc3b7c | store | 4 | `[sp] +0x0` | `?` |
| `ldr` at 0xc3b80 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc3b88 | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xc3b8c | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xc3b90 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc3b94 | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xc3ba0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc3ba8 | load | 1 | `[sp-20] +0x5` | `-` |
| `ldr` at 0xc3bb8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc3bc0 | load | 1 | `[sp-20] +0x6` | `-` |
| `ldr` at 0xc3bd0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc3bd8 | load | 1 | `[sp-20] +0x7` | `-` |
| `str` at 0xc3be0 | store | 4 | `[sp+4] +0x0` | `?` |
| `ldr` at 0xc3be4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc3bec | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xc3bf0 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xc3bf4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc3bf8 | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xc3c04 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc3c0c | load | 1 | `[sp-20] +0x5` | `-` |
| `ldr` at 0xc3c1c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc3c24 | load | 1 | `[sp-20] +0x6` | `-` |
| `ldr` at 0xc3c34 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc3c3c | load | 1 | `[sp-20] +0x7` | `-` |
| `str` at 0xc3c44 | store | 4 | `[sp+8] +0x0` | `?` |
| `ldr` at 0xc3c48 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc3c50 | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xc3c54 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc3c58 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc3c5c | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xc3c68 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc3c70 | load | 1 | `[sp-20] +0x5` | `-` |
| `ldr` at 0xc3c80 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc3c88 | load | 1 | `[sp-20] +0x6` | `-` |
| `ldr` at 0xc3c98 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc3ca0 | load | 1 | `[sp-20] +0x7` | `-` |
| `str` at 0xc3ca8 | store | 4 | `[sp+12] +0x0` | `?` |
| `ldr` at 0xc3cac | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc3cb4 | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xc3cb8 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc3cbc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc3cc0 | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xc3ccc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc3cd4 | load | 1 | `[sp-20] +0x5` | `-` |
| `ldr` at 0xc3ce4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc3cec | load | 1 | `[sp-20] +0x6` | `-` |
| `ldr` at 0xc3cfc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc3d04 | load | 1 | `[sp-20] +0x7` | `-` |
| `str` at 0xc3d0c | store | 4 | `[sp+16] +0x0` | `?` |
| `ldr` at 0xc3d10 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc3d18 | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xc3d1c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc3d28 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xc3d30 | store | 4 | `[sp+12] +0x0` | `0x7` |
| `str` at 0xc3d3c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xc3d40 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc3d44 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc3d48 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xc3d64 | store | 4 | `sp -0x24` | `ret(0x1b124)` |
| `ldr` at 0xc3d68 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc3d6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc3d70 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc3d78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc3d7c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xc3d84 | store | 4 | `sp -0x14` | `([sp-20] + [sp-56])` |
| `ldr` at 0xc3d88 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc3d90 | store | 4 | `[sp-36] +0xa8` | `0x0` |
| `ldr` at 0xc3d94 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc3d98 | load | 4 | `[sp-36] +0x10` | `-` |
| `str` at 0xc3d9c | store | 4 | `sp -0x3c` | `[[sp-36]+16]` |
| `ldr` at 0xc3da0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xc3dac | store | 4 | `sp -0x3c` | `ret(0x1af68)` |
| `ldr` at 0xc3db0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc3db4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xc3db8 | store | 4 | `[sp-36] +0x10` | `ret(0x1af68)` |
| `ldr` at 0xc3dbc | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xc3dc0 | load | 2 | `[sp-36] +0x18` | `-` |
| `str` at 0xc3dc4 | store | 4 | `sp -0x3c` | `[[sp-36]+24]:2` |
| `ldrh` at 0xc3dc8 | load | 2 | `sp -0x3c` | `-` |
| `str` at 0xc3de0 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xc3de4 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xc3de8 | load | 2 | `sp -0x3c` | `-` |
| `strh` at 0xc3dec | store | 2 | `[sp-36] +0x18` | `([sp-60]:2 & 0xffff)` |
| `ldr` at 0xc3df0 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xc3df4 | load | 2 | `[sp-36] +0x1a` | `-` |
| `str` at 0xc3df8 | store | 4 | `sp -0x3c` | `[[sp-36]+26]:2` |
| `ldrh` at 0xc3dfc | load | 2 | `sp -0x3c` | `-` |
| `str` at 0xc3e14 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xc3e18 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xc3e1c | load | 2 | `sp -0x3c` | `-` |
| `strh` at 0xc3e20 | store | 2 | `[sp-36] +0x1a` | `([sp-60]:2 & 0xffff)` |
| `ldr` at 0xc3e24 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xc3e28 | load | 2 | `[sp-36] +0x1c` | `-` |
| `str` at 0xc3e2c | store | 4 | `sp -0x3c` | `[[sp-36]+28]:2` |
| `ldrh` at 0xc3e30 | load | 2 | `sp -0x3c` | `-` |
| `str` at 0xc3e48 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xc3e4c | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xc3e50 | load | 2 | `sp -0x3c` | `-` |
| `strh` at 0xc3e54 | store | 2 | `[sp-36] +0x1c` | `([sp-60]:2 & 0xffff)` |
| `ldr` at 0xc3e58 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xc3e5c | load | 2 | `[sp-36] +0x1e` | `-` |
| `str` at 0xc3e60 | store | 4 | `sp -0x3c` | `[[sp-36]+30]:2` |
| `ldrh` at 0xc3e64 | load | 2 | `sp -0x3c` | `-` |
| `str` at 0xc3e7c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xc3e80 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xc3e84 | load | 2 | `sp -0x3c` | `-` |
| `strh` at 0xc3e88 | store | 2 | `[sp-36] +0x1e` | `([sp-60]:2 & 0xffff)` |
| `ldr` at 0xc3e8c | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc3e90 | load | 4 | `[sp+20] +0x0` | `-` |
| `ldr` at 0xc3e9c | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc3ea0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc3ea4 | store | 4 | `[sp+20] +0x0` | `[sp-36]` |
| `ldr` at 0xc3ea8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc3eac | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0xc3eb4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3eb8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc3ebc | store | 4 | `[sp-40] +0xa8` | `[sp-36]` |
| `ldr` at 0xc3ec0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc3ec4 | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0xc3ec8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc3ed0 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldmdb` at 0xc3ed8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc3ed8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc3ed8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc3d5c | `(unknown callee) 0x1b124` | `0xac` | `?` | `?` | `[[sp+16]]` |
| `bl` at 0xc3d74 | `(unknown callee) 0x1b304` | `ret(0x1b124)` | `[sp-20]` | `[sp-56]` | `ret(0x1b124)` |
| `bl` at 0xc3da4 | `(unknown callee) 0x1af68` | `[[sp-36]+16]` | `?` | `[sp-36]` | `[[sp-36]+16]` |
| `bl` at 0xc3dd0 | `(unknown callee) 0x1b208` | `[sp-60]:2` | `?` | `[sp-36]` | `[sp-60]:2` |
| `bl` at 0xc3e04 | `(unknown callee) 0x1b208` | `[sp-60]:2` | `?` | `[sp-36]` | `[sp-60]:2` |
| `bl` at 0xc3e38 | `(unknown callee) 0x1b208` | `[sp-60]:2` | `?` | `[sp-36]` | `[sp-60]:2` |
| `bl` at 0xc3e6c | `(unknown callee) 0x1b208` | `[sp-60]:2` | `?` | `[sp-36]` | `[sp-60]:2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc39f8 | `[[sp-24]] == 0x6` | `[[sp-24]] != 0x6` |
| `beq` at 0xc3d24 | `[sp-56] == 0xa8` | `[sp-56] != 0xa8` |
| `blt` at 0xc3d50 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xc3e98 | `[[sp+20]] != 0x0` | `[[sp+20]] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc3ed8 | `r0` | `?` |

## `JChipTargetByName`

- `JChipTargetByName`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe9b0c to 0xea118.
  Established. Two byte reads use absolute `0x13648c` (`ldrb` at 0xe9b44, `ldrb` at 0xe9b64). The
  opening call to `0x1afa4` carries entry r0 through r3 as candidates (`bl` at 0xe9b24). After a
  later byte comparison, inequality with `0x2e` takes the branch and equality falls through (`bne`
  at 0xe9bdc). The recorded return value is `[sp-152]` (`ldmdb` at 0xea0f0). Physical chip selection is not
  established by these reads, calls and return operands.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe9b1c | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xe9b20 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe9b2c | store | 4 | `sp -0x1c` | `ret(0x1afa4)` |
| `ldr` at 0xe9b30 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe9b3c | store | 4 | `sp -0x20` | `ret(0x1b22c)` |
| `ldr` at 0xe9b40 | load | 4 | `abs 0xea0f4` | `-` |
| `ldrb` at 0xe9b44 | load | 1 | `abs 0x13648c` | `-` |
| `strb` at 0xe9b48 | store | 1 | `sp -0x5c` | `([0x13648c]:1 & 0xff)` |
| `ldr` at 0xe9b60 | load | 4 | `abs 0xea0f4` | `-` |
| `ldrb` at 0xe9b64 | load | 1 | `abs 0x13648c` | `-` |
| `strb` at 0xe9b68 | store | 1 | `sp -0x70` | `([0x13648c]:1 & 0xff)` |
| `str` at 0xe9b84 | store | 4 | `sp -0x74` | `?` |
| `str` at 0xe9b8c | store | 4 | `sp -0x78` | `0x0` |
| `str` at 0xe9b94 | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0xe9b98 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe9b9c | store | 4 | `sp -0x24` | `[sp-28]` |
| `ldr` at 0xe9ba0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xe9ba4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xe9bb4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe9bb8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xe9bbc | load | 1 | `(unknown)` | `-` |
| `strb` at 0xe9bcc | store | 1 | `(unknown)` | `(ret(0x1b2bc) & 0xff)` |
| `ldr` at 0xe9bd0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xe9bd4 | load | 1 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xe9be0 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xe9be8 | store | 1 | `[sp-36] +0x0` | `0x3a` |
| `ldr` at 0xe9bec | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe9bf4 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `str` at 0xe9c08 | store | 4 | `sp -0xb0` | `sp-112` |
| `str` at 0xe9c10 | store | 4 | `sp -0xac` | `sp-116` |
| `ldr` at 0xe9c14 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe9c18 | load | 4 | `abs 0xea0f8` | `-` |
| `str` at 0xe9c28 | store | 4 | `sp -0x3c` | `ret(0x1b3e8)` |
| `ldr` at 0xe9c2c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe9c38 | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0xe9c48 | store | 1 | `sp -0x5c` | `0x0` |
| `ldr` at 0xe9c54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe9c58 | load | 4 | `abs 0xea0fc` | `-` |
| `str` at 0xe9c6c | store | 4 | `sp -0x3c` | `ret(0x1b3e8)` |
| `ldr` at 0xe9c70 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe9c7c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe9c90 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe9c94 | load | 4 | `abs 0xea100` | `-` |
| `ldr` at 0xe9c98 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe9c9c | load | 4 | `abs 0x142b68` | `-` |
| `ldr` at 0xe9cac | load | 4 | `abs 0xea104` | `-` |
| `ldr` at 0xe9cb0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe9cd4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe9cec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe9d00 | load | 4 | `abs 0xea104` | `-` |
| `str` at 0xe9d08 | store | 4 | `sp -0x78` | `?` |
| `ldr` at 0xe9d10 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe9d18 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xe9d20 | load | 4 | `sp -0x78` | `-` |
| `ldrb` at 0xe9d30 | load | 1 | `sp -0x5c` | `-` |
| `str` at 0xe9d40 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xe9d44 | load | 4 | `abs 0xea108` | `-` |
| `ldr` at 0xe9d48 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe9d4c | load | 4 | `abs 0x2ad390` | `-` |
| `ldr` at 0xe9d5c | load | 4 | `abs 0xea10c` | `-` |
| `ldr` at 0xe9d60 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe9d68 | load | 4 | `abs 0x2ad38c` | `-` |
| `str` at 0xe9d70 | store | 4 | `sp -0x80` | `?` |
| `ldr` at 0xe9d74 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xe9d78 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe9d84 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xe9d8c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe9da8 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xe9db0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe9dc8 | load | 4 | `sp -0x80` | `-` |
| `str` at 0xe9dcc | store | 4 | `sp -0x7c` | `[sp-128]` |
| `ldr` at 0xe9dd4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe9ddc | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `str` at 0xe9de8 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xe9dec | load | 4 | `abs 0xea108` | `-` |
| `ldr` at 0xe9df0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe9df4 | load | 4 | `abs 0x2ad390` | `-` |
| `ldr` at 0xe9e04 | load | 4 | `abs 0xea10c` | `-` |
| `ldr` at 0xe9e08 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe9e10 | load | 4 | `abs 0x2ad38c` | `-` |
| `str` at 0xe9e18 | store | 4 | `sp -0x80` | `?` |
| `str` at 0xe9e20 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe9e24 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xe9e28 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe9e2c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe9e3c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xe9e40 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe9e5c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe9e64 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe9e68 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xe9e74 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xe9e80 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xe9e84 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xe9e94 | load | 4 | `sp -0x80` | `-` |
| `str` at 0xe9e98 | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0xe9e9c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe9ea4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe9eac | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe9eb4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe9ebc | load | 4 | `sp -0x7c` | `-` |
| `str` at 0xe9ed0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe9ed4 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xe9ed8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe9edc | load | 4 | `[sp-124] +0xc` | `-` |
| `ldr` at 0xe9eec | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xe9ef0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe9f0c | load | 4 | `[sp-124] +0x8` | `-` |
| `str` at 0xe9f14 | store | 4 | `sp -0x80` | `?` |
| `ldr` at 0xe9f18 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xe9f1c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe9f20 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xe9f2c | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xe9f38 | load | 4 | `sp -0x80` | `-` |
| `ldrb` at 0xe9f3c | load | 1 | `[sp-128] +0x18` | `-` |
| `ldr` at 0xe9f4c | load | 4 | `sp -0x80` | `-` |
| `ldrb` at 0xe9f50 | load | 1 | `[sp-128] +0x18` | `-` |
| `ldr` at 0xe9f5c | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xe9f68 | load | 4 | `sp -0x80` | `-` |
| `ldrb` at 0xe9f6c | load | 1 | `[sp-128] +0x18` | `-` |
| `ldr` at 0xe9f70 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xe9f7c | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xe9f80 | load | 4 | `[sp-124] +0x0` | `-` |
| `ldr` at 0xe9f8c | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xe9f90 | load | 4 | `[sp-124] +0x4` | `-` |
| `str` at 0xe9fa4 | store | 4 | `sp -0x9c` | `sp-148` |
| `ldr` at 0xe9fa8 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xe9fac | load | 4 | `[sp-124] +0x4` | `-` |
| `str` at 0xe9fb0 | store | 4 | `sp -0xa0` | `[[sp-124]+4]` |
| `ldr` at 0xe9fb4 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xe9fb8 | load | 4 | `[sp-124] +0x4` | `-` |
| `ldr` at 0xe9fc4 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xe9fc8 | load | 4 | `[sp-124] +0x0` | `-` |
| `str` at 0xe9fcc | store | 4 | `sp -0xa0` | `[[sp-124]]` |
| `ldr` at 0xe9fd0 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xe9fd4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe9fd8 | load | 4 | `sp -0x80` | `-` |
| `ldrb` at 0xe9fdc | load | 1 | `(unknown)` | `-` |
| `str` at 0xe9fe0 | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0xe9fe4 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0xe9fe8 | load | 4 | `abs 0xea110` | `-` |
| `ldr` at 0xe9fec | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0xe9ff0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xea000 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xea004 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xea008 | load | 4 | `sp -0x80` | `-` |
| `ldrb` at 0xea00c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xea014 | load | 4 | `abs 0xea114` | `-` |
| `ldr` at 0xea018 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xea028 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea03c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xea040 | load | 4 | `[sp-128] +0x0` | `-` |
| `str` at 0xea044 | store | 4 | `sp -0x78` | `[[sp-128]]` |
| `ldr` at 0xea048 | load | 4 | `sp -0x80` | `-` |
| `ldrb` at 0xea04c | load | 1 | `[sp-128] +0x18` | `-` |
| `str` at 0xea050 | store | 4 | `sp -0x74` | `[[sp-128]+24]:1` |
| `ldr` at 0xea058 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xea060 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xea068 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea070 | load | 4 | `abs 0xea10c` | `-` |
| `ldr` at 0xea074 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xea078 | load | 4 | `abs 0x2ad38c` | `-` |
| `str` at 0xea080 | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0xea084 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0xea090 | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0xea094 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xea098 | load | 4 | `[sp-120] +0x0` | `-` |
| `ldr` at 0xea0a0 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0xea0a8 | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0xea0ac | load | 4 | `sp -0xa8` | `-` |
| `str` at 0xea0b0 | store | 4 | `sp -0xa4` | `[sp-168]` |
| `ldr` at 0xea0b4 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xea0c0 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xea0c4 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0xea0cc | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0xea0d0 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0xea0d4 | store | 4 | `sp -0x98` | `[sp-164]` |
| `ldr` at 0xea0dc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xea0e8 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0xea0ec | load | 4 | `sp -0x98` | `-` |
| `ldmdb` at 0xea0f0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xea0f0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xea0f0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xea0f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe9b24 | `(unknown callee) 0x1afa4` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xe9b34 | `(unknown callee) 0x1b22c` | `[sp-24]` | `?` | `?` | `ret(0x1afa4)` |
| `bl` at 0xe9b5c | `(unknown callee) 0x1b43c` | `sp-91` | `0x0` | `0x13` | `sp-91` |
| `bl` at 0xe9b7c | `(unknown callee) 0x1b43c` | `sp-111` | `0x0` | `0x13` | `sp-111` |
| `bl` at 0xe9bc4 | `(unknown callee) 0x1b2bc` | `?` | `?` | `?` | `?` |
| `bl` at 0xe9c20 | `(unknown callee) 0x1b3e8` | `[sp-28]` | `0x1364a0` | `sp-92` | `sp-72` |
| `bl` at 0xe9c64 | `(unknown callee) 0x1b3e8` | `[sp-28]` | `0x1364b4` | `sp-112` | `sp-116` |
| `bl` at 0xe9cdc | `(unknown callee) 0x1aff8` | `?` | `sp-112` | `sp-112` | `?` |
| `bl` at 0xe9d94 | `(unknown callee) 0x1aff8` | `?` | `sp-92` | `sp-92` | `?` |
| `bl` at 0xe9db8 | `(unknown callee) 0x1aff8` | `?` | `sp-92` | `sp-92` | `?` |
| `bl` at 0xe9ff4 | `(unknown callee) 0x1b49c` | `sp-148` | `0x136390` | `?` | `?` |
| `bl` at 0xea01c | `(unknown callee) 0x1b49c` | `sp-148` | `0x136398` | `?` | `?` |
| `bl` at 0xea02c | `(unknown callee) 0x1aff8` | `sp-148` | `[sp-28]` | `?` | `sp-148` |
| `bl` at 0xea06c | `(unknown callee) 0x1b400` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0xea0e0 | `(unknown callee) 0x1b400` | `[sp-28]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe9bac | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xe9bdc | `[[sp-36]]:1 != 0x2e` | `[[sp-36]]:1 == 0x2e` |
| `beq` at 0xe9c34 | `ret(0x1b3e8) == 0x3` | `ret(0x1b3e8) != 0x3` |
| `beq` at 0xe9c40 | `ret(0x1b3e8) == 0x4` | `ret(0x1b3e8) != 0x4` |
| `beq` at 0xe9c78 | `ret(0x1b3e8) == 0x1` | `ret(0x1b3e8) != 0x1` |
| `beq` at 0xe9c84 | `ret(0x1b3e8) == 0x2` | `ret(0x1b3e8) != 0x2` |
| `blt` at 0xe9ca4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xe9ce8 | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |
| `bne` at 0xe9d28 | `[sp-120] != 0x0` | `[sp-120] == 0x0` |
| `beq` at 0xe9d38 | `[sp-92]:1 == 0x0` | `[sp-92]:1 != 0x0` |
| `blt` at 0xe9d54 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xe9d80 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xe9da0 | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |
| `bne` at 0xe9dc4 | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |
| `blt` at 0xe9dfc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0xe9e34 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xe9e70 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xe9e7c | `[sp-124] == 0x0` | `[sp-124] != 0x0` |
| `beq` at 0xe9e8c | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xe9ec4 | `[sp-124] != 0x0` | `[sp-124] == 0x0` |
| `blt` at 0xe9ee4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xe9f28 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xe9f34 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xe9f44 | `[[sp-128]+24]:1 == 0x0` | `[[sp-128]+24]:1 != 0x0` |
| `bne` at 0xe9f58 | `[[sp-128]+24]:1 != 0x0` | `[[sp-128]+24]:1 == 0x0` |
| `beq` at 0xe9f64 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xe9f78 | `[[sp-128]+24]:1 == [sp-116]` | `[[sp-128]+24]:1 != [sp-116]` |
| `bne` at 0xe9f88 | `[[sp-124]] != 0x0` | `[[sp-124]] == 0x0` |
| `bne` at 0xe9f98 | `[[sp-124]+4] != 0x0` | `[[sp-124]+4] == 0x0` |
| `bne` at 0xe9fc0 | `[[sp-124]+4] != 0x0` | `[[sp-124]+4] == 0x0` |
| `bne` at 0xea038 | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |
| `beq` at 0xea0bc | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xea0f0 | `r0` | `[sp-152]` |

## `JChipName`

- `JChipName`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xea118 to 0xea168.
  Established. The call targets `0xeb27c` with entry r0 through r3 as candidates (`bl` at 0xea130).
  A zero result takes the branch and nonzero falls through (`beq` at 0xea144). The recorded r0 exit
  is unknown after the paths merge (`ldmdb` at 0xea164). The name returned to a caller cannot be
  identified from the flat-image result.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xea128 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xea12c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xea138 | store | 4 | `sp -0x18` | `ret(0xeb27c)` |
| `ldr` at 0xea13c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xea148 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xea150 | store | 4 | `sp -0x1c` | `ret(0xeb27c)+4` |
| `str` at 0xea15c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xea160 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xea164 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xea164 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xea164 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xea130 | `(unknown callee) 0xeb27c` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xea144 | `ret(0xeb27c) == 0x0` | `ret(0xeb27c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xea164 | `r0` | `?` |

## `JChipGetChipList`

- `JChipGetChipList`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xea168 to 0xea2fc.
  Established. Word reads include absolute `0x2ad390` and absolute `0x2ad38c` (`ldr` at 0xea1a8,
  `ldr` at 0xea1c4). A call to `0x1b2c8` has candidates `[sp-36]`, `[sp-32]+50`, `[sp-32]` and
  `[sp-32]+50` (`bl` at 0xea25c). The recorded exit has unknown r0 (`ldmdb` at 0xea2ec). The
  chip-list format and memory-allocation contract remain unsettled.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xea17c | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xea184 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xea18c | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xea194 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xea19c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xea1a0 | load | 4 | `abs 0xea2f0` | `-` |
| `ldr` at 0xea1a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xea1a8 | load | 4 | `abs 0x2ad390` | `-` |
| `ldr` at 0xea1b8 | load | 4 | `abs 0xea2f4` | `-` |
| `ldr` at 0xea1bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xea1c4 | load | 4 | `abs 0x2ad38c` | `-` |
| `str` at 0xea1cc | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xea1d4 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0xea1d8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xea1dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xea1e0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xea1f0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xea1f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xea210 | load | 4 | `(unknown)` | `-` |
| `str` at 0xea218 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xea21c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xea234 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xea238 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea248 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xea250 | store | 4 | `sp -0x20` | `[sp-32]+50` |
| `ldr` at 0xea254 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xea258 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xea264 | store | 4 | `sp -0x24` | `ret(0x1b2c8)` |
| `ldr` at 0xea268 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xea26c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xea274 | store | 4 | `sp -0x28` | `(ret(0x1b2c8) + [sp-28])` |
| `ldr` at 0xea278 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xea280 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xea28c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xea290 | load | 4 | `abs 0xea2f8` | `-` |
| `ldr` at 0xea298 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xea2a8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xea2b0 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x1b22c))` |
| `ldr` at 0xea2b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xea2b8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xea2c0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xea2c4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xea2cc | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0xea2d4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xea2dc | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xea2e4 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xea2ec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xea2ec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xea2ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xea228 | `(unknown callee) 0x1b22c` | `?` | `?` | `?` | `?` |
| `bl` at 0xea25c | `(unknown callee) 0x1b2c8` | `[sp-36]` | `[sp-32]+50` | `[sp-32]` | `[sp-32]+50` |
| `bl` at 0xea288 | `(unknown callee) 0x1b508` | `[sp-40]` | `[sp-48]+4` | `?` | `[sp-48]+4` |
| `bl` at 0xea294 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `0x1364c0` | `?` | `?` |
| `bl` at 0xea29c | `(unknown callee) 0x1b22c` | `[sp-40]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xea1b0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0xea1e8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bls` at 0xea244 | `ret(0x1b22c)+2 <= ? (unsigned, 32-bit)` | `ret(0x1b22c)+2 > ? (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xea2ec | `r0` | `?` |

## `JChipGetNextChip`

- `JChipGetNextChip`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xea2fc to 0xea468.
  Established. A negative signed entry r0 takes the first branch, while a nonnegative value falls
  through (`blt` at 0xea324). Word reads on known paths include absolute `0x2ad390` and absolute
  `0x2ad38c` (`ldr` at 0xea34c, `ldr` at 0xea37c), plus `[sp-48] +0xc` and `[sp-48] +0x8` (`ldr` at
  0xea398, `ldr` at 0xea3e4). No calls are recorded in this span. Which physical chip each returned
  record describes is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xea30c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xea310 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xea318 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xea31c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xea328 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xea32c | store | 4 | `sp -0x20` | `r0` |
| `ldr` at 0xea330 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xea338 | store | 4 | `sp -0x24` | `r0+1` |
| `str` at 0xea344 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xea348 | load | 4 | `abs 0xea460` | `-` |
| `ldr` at 0xea34c | load | 4 | `abs 0x2ad390` | `-` |
| `str` at 0xea350 | store | 4 | `sp -0x24` | `[0x2ad390]` |
| `ldr` at 0xea354 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xea358 | store | 4 | `sp -0x1c` | `[sp-32]` |
| `ldr` at 0xea35c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea360 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xea370 | load | 4 | `abs 0xea464` | `-` |
| `ldr` at 0xea374 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea37c | load | 4 | `abs 0x2ad38c` | `-` |
| `str` at 0xea384 | store | 4 | `sp -0x30` | `?` |
| `str` at 0xea38c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xea390 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xea394 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xea398 | load | 4 | `[sp-48] +0xc` | `-` |
| `ldr` at 0xea3a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xea3b4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xea3c4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xea3c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xea3e4 | load | 4 | `[sp-48] +0x8` | `-` |
| `str` at 0xea3ec | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xea3f4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xea3f8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xea414 | load | 4 | `[sp-48] +0x8` | `-` |
| `ldr` at 0xea41c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xea42c | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xea430 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xea438 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xea440 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xea448 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0xea454 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xea458 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xea45c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xea45c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xea45c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xea324 | `r0 < 0x0 (signed, 32-bit)` | `r0 >= 0x0 (signed, 32-bit)` |
| `blt` at 0xea368 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0xea3a0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xea3b0 | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0xea3bc | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xea424 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xea45c | `r0` | `?` |

## `tdrfld_dump`

- `tdrfld_dump`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xefdac to 0xefec0.
  Established. Entry r2 nonzero takes the opening branch and zero falls through (`bne` at 0xefdd0).
  Later reads include a word at `[sp-28] +0x0` and halfwords at `[sp-28] +0x4` and `[sp-28] +0x6`
  (`ldr` at 0xefe20, `ldrh` at 0xefe54, `ldrh` at 0xefe6c). One call targets `0x24028` with
  candidates unknown, `0x137008`, entry r2 and entry r1 (`bl` at 0xefe04). These operands do not
  establish hardware accesses or field units.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xefdbc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xefdc0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xefdc4 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xefdc8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xefdd8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xefde4 | load | 4 | `abs 0xefea0` | `-` |
| `ldr` at 0xefde8 | load | 4 | `abs 0x144dc4` | `-` |
| `str` at 0xefdec | store | 4 | `sp -0x14` | `[0x144dc4]` |
| `ldr` at 0xefdf0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xefdfc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xefe00 | load | 4 | `abs 0xefea4` | `-` |
| `ldr` at 0xefe0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xefe10 | load | 4 | `abs 0xefea8` | `-` |
| `ldr` at 0xefe14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xefe1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xefe20 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xefe24 | store | 4 | `sp -0x20` | `[[sp-28]]` |
| `ldr` at 0xefe28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xefe2c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xefe38 | load | 4 | `abs 0xefeac` | `-` |
| `str` at 0xefe3c | store | 4 | `sp -0x20` | `0x13702c` |
| `ldr` at 0xefe40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xefe44 | load | 4 | `abs 0xefeb0` | `-` |
| `ldr` at 0xefe48 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xefe50 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xefe54 | load | 2 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xefe58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xefe5c | load | 4 | `abs 0xefeb4` | `-` |
| `ldr` at 0xefe68 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xefe6c | load | 2 | `[sp-28] +0x6` | `-` |
| `ldr` at 0xefe70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xefe74 | load | 4 | `abs 0xefeb8` | `-` |
| `ldr` at 0xefe80 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xefe84 | load | 1 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xefe88 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xefe8c | load | 1 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xefe90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xefe94 | load | 4 | `abs 0xefebc` | `-` |
| `ldmdb` at 0xefe9c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xefe9c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xefe9c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xefe04 | `(unknown callee) 0x24028` | `?` | `0x137008` | `r2` | `r1` |
| `bl` at 0xefe18 | `(unknown callee) 0x24028` | `?` | `0x137028` | `r1` | `r1` |
| `bl` at 0xefe4c | `(unknown callee) 0x24028` | `[sp-20]` | `0x137034` | `?` | `?` |
| `bl` at 0xefe64 | `(unknown callee) 0x24028` | `[sp-20]` | `0x137040` | `[[sp-28]+4]:2` | `[[sp-28]+4]:2` |
| `bl` at 0xefe7c | `(unknown callee) 0x24028` | `[sp-20]` | `0x13704c` | `[[sp-28]+6]:2` | `[[sp-28]+6]:2` |
| `bl` at 0xefe98 | `(unknown callee) 0x24028` | `[sp-20]` | `0x137058` | `[[sp-28]+8]:1` | `[[sp-28]+8]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xefdd0 | `r2 != 0x0` | `r2 == 0x0` |
| `bne` at 0xefde0 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xefdf8 | `r1 != 0x0` | `r1 == 0x0` |
| `bne` at 0xefe34 | `[[sp-28]] != 0x0` | `[[sp-28]] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xefe9c | `r0` | `?` |

## `tdrdef_dump`

- `tdrdef_dump`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf0054 to 0xf0200.
  Established. The routine reads halfwords at `[sp-28] +0x0` and `[sp-28] +0x2` (`ldrh` at
  0xf00c8, `ldrh` at 0xf00e0); its first branch takes nonzero entry r2 and falls through for zero
  (`bne` at 0xf0078). Calls include `0x24028` with unknown, `0x137070`, entry r2 and entry r1 as
  candidates (`bl` at 0xf00ac). Definition flags and the referenced hardware layout remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf0064 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf0068 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf006c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xf0070 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf0080 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf008c | load | 4 | `abs 0xf01e4` | `-` |
| `ldr` at 0xf0090 | load | 4 | `abs 0x144dc4` | `-` |
| `str` at 0xf0094 | store | 4 | `sp -0x14` | `[0x144dc4]` |
| `ldr` at 0xf0098 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf00a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf00a8 | load | 4 | `abs 0xf01e8` | `-` |
| `ldr` at 0xf00b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf00b8 | load | 4 | `abs 0xf01ec` | `-` |
| `ldr` at 0xf00bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf00c4 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xf00c8 | load | 2 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xf00cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf00d0 | load | 4 | `abs 0xf01f0` | `-` |
| `ldr` at 0xf00dc | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xf00e0 | load | 2 | `[sp-28] +0x2` | `-` |
| `ldr` at 0xf00e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf00e8 | load | 4 | `abs 0xf01f4` | `-` |
| `str` at 0xf00f8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xf00fc | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xf0100 | load | 2 | `[sp-28] +0x2` | `-` |
| `ldr` at 0xf0104 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf011c | load | 4 | `abs 0xf01f8` | `-` |
| `ldr` at 0xf0120 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf012c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf0130 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf0144 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xf014c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf015c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf0164 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf016c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xf0170 | load | 2 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xf0188 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xf018c | load | 2 | `[sp-28] +0x0` | `-` |
| `str` at 0xf019c | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xf01a4 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xf01a8 | load | 2 | `[sp-28] +0x0` | `-` |
| `str` at 0xf01b8 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xf01bc | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xf01c4 | store | 4 | `sp -0x4c` | `[sp-76]+1` |
| `ldr` at 0xf01c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf01cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf01d0 | load | 4 | `abs 0xf01fc` | `-` |
| `ldr` at 0xf01d4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xf01d8 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldmdb` at 0xf01e0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf01e0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf01e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf00ac | `(unknown callee) 0x24028` | `?` | `0x137070` | `r2` | `r1` |
| `bl` at 0xf00c0 | `(unknown callee) 0x24028` | `?` | `0x137028` | `r1` | `r1` |
| `bl` at 0xf00d8 | `(unknown callee) 0x24028` | `[sp-20]` | `0x13704c` | `[[sp-28]]:2` | `[[sp-28]]:2` |
| `bl` at 0xf00f0 | `(unknown callee) 0x24028` | `[sp-20]` | `0x137098` | `[[sp-28]+2]:2` | `[[sp-28]+2]:2` |
| `bl` at 0xf0124 | `(unknown callee) 0x1b49c` | `sp-72` | `0x1370a4` | `?` | `sp-72` |
| `bl` at 0xf0158 | `(unknown callee) 0xefdac` | `[sp-20]` | `sp-72` | `?` | `?` |
| `bl` at 0xf01dc | `(unknown callee) 0xf7858` | `[sp-20]` | `0x1370b4` | `[sp-76]` | `[[sp-28]+8]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf0078 | `r2 != 0x0` | `r2 == 0x0` |
| `bne` at 0xf0088 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xf00a0 | `r1 != 0x0` | `r1 == 0x0` |
| `blo` at 0xf010c | `C == 0` | `not (C == 0)` |
| `bne` at 0xf0184 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf01e0 | `r0` | `?` |

## `tdrdef_set_field`

- `tdrdef_set_field`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf0200 to 0xf0448.
  Established. The routine reads a word at `r0 +0x4` (`ldr` at 0xf0238), followed by a halfword
  through an unknown effective address (`ldrh` at 0xf0240). The bit-test branch takes its zero arm
  and falls through for a nonzero unknown value (`beq` at 0xf0368). The two branch arms contain word stores with
  unknown values and unknown effective addresses (`str` at 0xf0398, `str` at 0xf03d0). No call is
  recorded, and the hardware meaning of the selected bits remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf0210 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf0214 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf0218 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xf021c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xf0220 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0224 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0238 | load | 4 | `r0 +0x4` | `-` |
| `ldrh` at 0xf0240 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf025c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0260 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0274 | load | 4 | `r0 +0x4` | `-` |
| `ldrh` at 0xf027c | load | 2 | `(unknown)` | `-` |
| `str` at 0xf0290 | store | 4 | `sp -0x28` | `?` |
| `str` at 0xf0298 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xf029c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf02a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf02b4 | load | 4 | `[sp-20] +0x4` | `-` |
| `ldrh` at 0xf02bc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf02c0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf02cc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xf02d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf02d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf02e8 | load | 4 | `[sp-20] +0x4` | `-` |
| `ldrh` at 0xf02f0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf02f4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf0308 | store | 4 | `sp -0x34` | `?` |
| `str` at 0xf0310 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xf0314 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0318 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf032c | load | 4 | `[sp-20] +0x4` | `-` |
| `ldrh` at 0xf0334 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf0338 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf0348 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xf0350 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf0358 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf035c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xf036c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf0374 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xf037c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf0384 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xf038c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf0390 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xf0398 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf03a0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf03a8 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xf03b0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf03b8 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xf03c0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf03c8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf03d0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf03d4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xf03e0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf03e8 | store | 4 | `sp -0x28` | `[sp-40]-1` |
| `str` at 0xf03f0 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xf03f8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xf0400 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xf0404 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf0410 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xf0418 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xf0420 | store | 4 | `sp -0x34` | `0x80000000` |
| `ldr` at 0xf0428 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xf0430 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xf0434 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf043c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldmdb` at 0xf0444 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf0444 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf0444 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xf0340 | `C == 0` | `not (C == 0)` |
| `beq` at 0xf0368 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xf03dc | `[sp-48] != 0x80000000` | `[sp-48] == 0x80000000` |
| `bne` at 0xf040c | `[sp-52] != 0x1` | `[sp-52] == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf0444 | `r0` | `?` |

## `tdrdef_get_field`

- `tdrdef_get_field`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf0448 to 0xf072c.
  Established. A word read at `r0 +0x4` precedes a halfword load with unknown effective address
  (`ldr` at 0xf0480, `ldrh` at 0xf0488). The first branch takes a nonzero unknown operand and falls
  through for zero (`bne` at 0xf049c). A later word store clears an unknown effective address (`str`
  at 0xf05dc). No calls are recorded, and the final r0 value is unknown (`ldmdb` at
  0xf0728). Source and destination bit numbering remain unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf0458 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf045c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf0460 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xf0464 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xf0468 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf046c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0480 | load | 4 | `r0 +0x4` | `-` |
| `ldrh` at 0xf0488 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf04a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf04a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf04b8 | load | 4 | `r0 +0x4` | `-` |
| `ldrh` at 0xf04c0 | load | 2 | `(unknown)` | `-` |
| `str` at 0xf04d0 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf04d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf04dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf04f0 | load | 4 | `r0 +0x4` | `-` |
| `ldrh` at 0xf04f8 | load | 2 | `(unknown)` | `-` |
| `str` at 0xf0508 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf050c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xf0514 | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0xf0518 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xf051c | store | 4 | `sp -0x28` | `[sp-60]` |
| `ldr` at 0xf0520 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf0528 | store | 4 | `sp -0x30` | `[sp-60]-1` |
| `str` at 0xf0530 | store | 4 | `sp -0x38` | `0x1` |
| `ldr` at 0xf0534 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0538 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf054c | load | 4 | `r0 +0x4` | `-` |
| `ldrh` at 0xf0554 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf0558 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf0564 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xf056c | store | 4 | `sp -0x34` | `0x80000000` |
| `ldr` at 0xf0570 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0574 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0588 | load | 4 | `r0 +0x4` | `-` |
| `ldrh` at 0xf0590 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf0594 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf05a0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xf05a8 | store | 4 | `sp -0x34` | `?` |
| `str` at 0xf05b0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xf05b4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf05b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xf05c8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf05d0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xf05dc | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xf05e0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf05e8 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `str` at 0xf05f4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xf05f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf05fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0610 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf0618 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf061c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf062c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf0634 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf063c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf0640 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf0650 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xf0658 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xf0660 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xf0668 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xf0670 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf0674 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf067c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf0684 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xf068c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xf0694 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xf069c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xf06a4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf06ac | load | 4 | `(unknown)` | `-` |
| `str` at 0xf06b4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf06b8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf06c4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xf06cc | store | 4 | `sp -0x30` | `[sp-48]-1` |
| `str` at 0xf06d4 | store | 4 | `sp -0x38` | `0x1` |
| `ldr` at 0xf06dc | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf06e4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xf06e8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf06f4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xf06fc | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xf0704 | store | 4 | `sp -0x34` | `0x80000000` |
| `ldr` at 0xf070c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xf0714 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xf0718 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf0720 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldmdb` at 0xf0728 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf0728 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf0728 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf049c | `? != 0x0` | `? == 0x0` |
| `blo` at 0xf05c0 | `C == 0` | `not (C == 0)` |
| `blo` at 0xf0624 | `C == 0` | `not (C == 0)` |
| `beq` at 0xf064c | `? == 0x0` | `? != 0x0` |
| `bne` at 0xf06c0 | `[sp-56] != 0x80000000` | `[sp-56] == 0x80000000` |
| `bne` at 0xf06f0 | `[sp-52] != 0x1` | `[sp-52] == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf0728 | `r0` | `?` |

## `tdrdata_dump_fields`

- `tdrdata_dump_fields`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf0d38 to 0xf1074.
  Established. The initial decision takes nonzero entry r0 and falls through for zero (`bne` at
  0xf0d5c). A word at `r1 +0x8` supplies the next decision: nonzero takes the branch and zero falls
  through (`ldr` at 0xf0d70, `bne` at 0xf0d78). A later word store clears absolute `0x2ad39c` (`str`
  at 0xf0e94). The call to `0xf0448` carries `[[sp-24]+8]`, `[sp-32]`, `0x0` and `[[sp-24]+16]` as
  candidates (`bl` at 0xf0fbc). These accesses do not identify physical registers.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf0d48 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf0d4c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf0d50 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xf0d54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0d60 | load | 4 | `abs 0xf1064` | `-` |
| `ldr` at 0xf0d64 | load | 4 | `abs 0x144dc4` | `-` |
| `str` at 0xf0d68 | store | 4 | `sp -0x14` | `[0x144dc4]` |
| `ldr` at 0xf0d6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0d70 | load | 4 | `r1 +0x8` | `-` |
| `ldr` at 0xf0d80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0d84 | load | 4 | `r1 +0x8` | `-` |
| `ldrh` at 0xf0d88 | load | 2 | `[r1+8] +0x0` | `-` |
| `ldr` at 0xf0da0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0da4 | load | 4 | `r1 +0x8` | `-` |
| `ldrh` at 0xf0da8 | load | 2 | `[r1+8] +0x0` | `-` |
| `str` at 0xf0db8 | store | 4 | `sp -0x78` | `?` |
| `ldr` at 0xf0dbc | load | 4 | `sp -0x78` | `-` |
| `str` at 0xf0dc4 | store | 4 | `sp -0x78` | `?` |
| `ldr` at 0xf0dcc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0dd0 | load | 4 | `r1 +0x8` | `-` |
| `ldrh` at 0xf0dd4 | load | 2 | `[r1+8] +0x0` | `-` |
| `str` at 0xf0de4 | store | 4 | `sp -0x78` | `?` |
| `ldr` at 0xf0de8 | load | 4 | `sp -0x78` | `-` |
| `str` at 0xf0df4 | store | 4 | `sp -0x78` | `?` |
| `ldr` at 0xf0df8 | load | 4 | `sp -0x78` | `-` |
| `str` at 0xf0e04 | store | 4 | `sp -0x24` | `ret(0x1b124)` |
| `ldr` at 0xf0e08 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf0e18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0e1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0e20 | load | 4 | `abs 0xf1068` | `-` |
| `ldr` at 0xf0e24 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xf0e30 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xf0e34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0e38 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrh` at 0xf0e3c | load | 2 | `[[sp-24]+8] +0x2` | `-` |
| `ldr` at 0xf0e40 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf0e50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf0e5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf0e68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0e6c | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xf0e70 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xf0e78 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xf0e8c | load | 4 | `abs 0xf106c` | `-` |
| `str` at 0xf0e94 | store | 4 | `abs 0x2ad39c` | `0x0` |
| `ldr` at 0xf0e98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0e9c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xf0ea0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf0eb4 | load | 4 | `[[sp-24]+8] +0x4` | `-` |
| `ldrh` at 0xf0ebc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf0ed4 | load | 4 | `abs 0xf106c` | `-` |
| `ldr` at 0xf0ed8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0edc | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xf0ee0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf0ef4 | load | 4 | `[[sp-24]+8] +0x4` | `-` |
| `ldrh` at 0xf0efc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf0f0c | load | 4 | `abs 0x2ad39c` | `-` |
| `ldr` at 0xf0f1c | load | 4 | `abs 0xf106c` | `-` |
| `ldr` at 0xf0f20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0f24 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xf0f28 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf0f3c | load | 4 | `[[sp-24]+8] +0x4` | `-` |
| `ldrh` at 0xf0f44 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf0f58 | load | 4 | `abs 0x2ad39c` | `-` |
| `ldr` at 0xf0f68 | load | 4 | `abs 0xf106c` | `-` |
| `ldr` at 0xf0f6c | load | 4 | `abs 0x2ad39c` | `-` |
| `ldr` at 0xf0f74 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf0f80 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xf0f84 | load | 4 | `abs 0xf106c` | `-` |
| `ldr` at 0xf0f88 | load | 4 | `abs 0xf106c` | `-` |
| `ldr` at 0xf0f8c | load | 4 | `abs 0x2ad39c` | `-` |
| `str` at 0xf0f94 | store | 4 | `abs 0x2ad39c` | `[0x2ad39c]+1` |
| `ldr` at 0xf0f9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0fa0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0fa4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf0fa8 | store | 4 | `sp -0x7c` | `[sp-36]` |
| `ldr` at 0xf0fac | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xf0fb0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf0fb8 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xf0fc0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0fc4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xf0fc8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf0fdc | load | 4 | `[[sp-24]+8] +0x4` | `-` |
| `ldrh` at 0xf0fe4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf0ff0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf0ff8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0ffc | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xf1000 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1014 | load | 4 | `[[sp-24]+8] +0x4` | `-` |
| `ldr` at 0xf101c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1020 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xf1024 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xf102c | load | 1 | `([[sp-24]+12] + [sp-32]) +0x0` | `-` |
| `ldr` at 0xf1034 | load | 4 | `abs 0xf1070` | `-` |
| `ldr` at 0xf1038 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf1048 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf1050 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xf1058 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xf1060 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf1060 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf1060 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf0dfc | `(unknown callee) 0x1b124` | `[sp-120]` | `r1` | `r2` | `?` |
| `bl` at 0xf0e28 | `(unknown callee) 0x24028` | `[sp-20]` | `0x137190` | `[[sp-24]]` | `[sp-24]` |
| `bl` at 0xf0fbc | `(unknown callee) 0xf0448` | `[[sp-24]+8]` | `[sp-32]` | `0x0` | `[[sp-24]+16]` |
| `bl` at 0xf0ff4 | `(unknown callee) 0xf1074` | `?` | `[sp-36]` | `sp-116` | `?` |
| `bl` at 0xf1044 | `(unknown callee) 0x23f2c` | `0x1371b0` | `?` | `[([[sp-24]+12] + [sp-32])]:1` | `sp-116` |
| `bl` at 0xf105c | `(unknown callee) 0x1b400` | `[sp-36]` | `?` | `[[[sp-24]+8]+2]:2` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf0d5c | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xf0d78 | `[r1+8] != 0x0` | `[r1+8] == 0x0` |
| `bne` at 0xf0d9c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf0e10 | `ret(0x1b124) != 0x0` | `ret(0x1b124) == 0x0` |
| `blo` at 0xf0e48 | `C == 0` | `not (C == 0)` |
| `beq` at 0xf0e58 | `[sp-28] == 0x1` | `[sp-28] != 0x1` |
| `bne` at 0xf0e64 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xf0e84 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf0ed0 | `? != 0x0` | `? == 0x0` |
| `blo` at 0xf0f14 | `[0x2ad39c] < ? (unsigned, 32-bit)` | `[0x2ad39c] >= ? (unsigned, 32-bit)` |
| `blo` at 0xf0f60 | `[0x2ad39c] < ? (unsigned, 32-bit)` | `[0x2ad39c] >= ? (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf1060 | `r0` | `?` |

## `tdrdata_create_field_str`

- `tdrdata_create_field_str`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf1074 to 0xf1188.
  Established. The routine stores `0x1371d0` at `sp -0x34` (`str` at 0xf1094); its byte writes
  include an unknown value through an unknown effective address (`strb` at 0xf1124), and
  a zero byte store also has an unknown effective address (`strb` at 0xf117c). One branch takes `[sp-44] !=
  0x0` and falls through for zero (`bne` at 0xf113c). There are no recorded calls, and the exit
  preserves entry r0 (`ldmdb` at 0xf1180). The character-table contents were not established in this
  routine reading.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf1084 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf1088 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf108c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xf1090 | load | 4 | `abs 0xf1184` | `-` |
| `str` at 0xf1094 | store | 4 | `sp -0x34` | `0x1371d0` |
| `str` at 0xf109c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xf10a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf10b4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xf10c0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xf10c4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf10d4 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xf10dc | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xf10e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf10e4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xf10f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf10f8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf1100 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1108 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf110c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf1118 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xf1120 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xf1124 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xf1128 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf1130 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xf1134 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xf1144 | store | 4 | `sp -0x2c` | `0x1c` |
| `ldr` at 0xf1148 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf1150 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xf1158 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xf1160 | store | 4 | `sp -0x2c` | `[sp-44]-4` |
| `ldr` at 0xf1164 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf116c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf1174 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xf117c | store | 1 | `(unknown)` | `0x0` |
| `ldmdb` at 0xf1180 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf1180 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf1180 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xf10ec | `C == 0` | `not (C == 0)` |
| `bne` at 0xf113c | `[sp-44] != 0x0` | `[sp-44] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf1180 | `r0` | `r0` |
