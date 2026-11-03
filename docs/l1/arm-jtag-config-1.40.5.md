# ARM JTAG configuration and scan-data routines in 1.40.5

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
These routines have no unresolved computed transfers. Indirect call targets and numeric callee names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/jtag-config-facts.txt) retain all tool sections.

## `verify_length_data`

- `verify_length_data`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe1604 to 0xe17d4.
  Established. The first four-byte pointee assignment writes `0x0` through `r3 +0x0` (`str` at
  0xe162c). A later four-byte write through `[sp-32] +0x0` has unknown value (`str` at 0xe17c0). The
  mask test takes `[sp-48] != 0x80000000` to `0xe16e0` (`bne` at 0xe16c4). Data-length verification
  semantics and the merged return value remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe1614 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe1618 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe161c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe1620 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xe1624 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe162c | store | 4 | `r3 +0x0` | `0x0` |
| `str` at 0xe1634 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe1638 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe1650 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe1654 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe1674 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe1678 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe1680 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe1688 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe168c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe169c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe16a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe16b0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe16b8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe16bc | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe16cc | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xe16d0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe16d8 | store | 4 | `sp -0x2c` | `[sp-44]-1` |
| `ldr` at 0xe16e0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe16e8 | store | 4 | `sp -0x30` | `?` |
| `str` at 0xe16f4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe16f8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe1700 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe1708 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe170c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe171c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe1720 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe1730 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe1738 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe173c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe174c | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xe1750 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe1758 | store | 4 | `sp -0x2c` | `[sp-44]-1` |
| `ldr` at 0xe1760 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe1768 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe1770 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe1774 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe1780 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe1788 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe1790 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe1794 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe17a8 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xe17b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe17b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe17b8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe17c0 | store | 4 | `[sp-32] +0x0` | `?` |
| `str` at 0xe17c8 | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0xe17cc | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xe17d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe17d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe17d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe1698 | `? == 0x0` | `? != 0x0` |
| `blo` at 0xe16a8 | `C == 0` | `not (C == 0)` |
| `bne` at 0xe16c4 | `[sp-48] != 0x80000000` | `[sp-48] == 0x80000000` |
| `bne` at 0xe1718 | `? != 0x0` | `? == 0x0` |
| `blo` at 0xe1728 | `C == 0` | `not (C == 0)` |
| `bne` at 0xe1744 | `[sp-48] != 0x80000000` | `[sp-48] == 0x80000000` |
| `bne` at 0xe177c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xe17a0 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe17d0 | `r0` | `?` |

## `build_sic_tdi`

- `build_sic_tdi`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe17d4 to 0xe1a30.
  Established. A four-byte assignment clears `r3 +0x0` (`str` at 0xe1858). Included four-byte writes
  at unknown targets have unknown values (`str` at 0xe1994, `str` at 0xe19f8). The branch takes
  `([sp-36] + [sp-24]) <= 0x1f`, unsigned 32-bit, to `0xe19fc` (`bls` at 0xe19b0). Bit packing
  layout and the exit value remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe17e4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe17e8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe17ec | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe17f0 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xe17f8 | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0xe1800 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xe1804 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe1808 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe181c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe1824 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe182c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe1830 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe1838 | store | 4 | `sp -0x34` | `?` |
| `str` at 0xe1844 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xe184c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe1850 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe1858 | store | 4 | `r3 +0x0` | `0x0` |
| `str` at 0xe1860 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xe1864 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe1868 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe1878 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe1880 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe1888 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe188c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe1894 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe1898 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe189c | load | 4 | `abs 0xe1a24` | `-` |
| `ldr` at 0xe18a8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe18b0 | load | 4 | `abs 0xe1a24` | `-` |
| `str` at 0xe18bc | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe18c0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe18c4 | load | 4 | `abs 0xe1a28` | `-` |
| `ldr` at 0xe18d0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe18d8 | load | 4 | `abs 0xe1a28` | `-` |
| `str` at 0xe18e4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe18e8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe18ec | load | 4 | `abs 0xe1a2c` | `-` |
| `ldr` at 0xe18f8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe1900 | load | 4 | `abs 0xe1a2c` | `-` |
| `str` at 0xe190c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe1910 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe1920 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe1934 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe1938 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe1948 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe195c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe1960 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe1968 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe1970 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe1978 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe1980 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe1984 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe198c | load | 4 | `(unknown)` | `-` |
| `str` at 0xe1994 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe1998 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe199c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe19a4 | store | 4 | `sp -0x24` | `([sp-36] + [sp-24])` |
| `ldr` at 0xe19a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe19b4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe19bc | store | 4 | `sp -0x24` | `([sp-36] + [sp-24])-32` |
| `ldr` at 0xe19c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe19c4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe19cc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe19d0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe19d8 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xe19dc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe19e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe19ec | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe19f0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe19f8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe19fc | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe1a04 | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `ldr` at 0xe1a0c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe1a14 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xe1a20 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe1a20 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe1a20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xe1810 | `C == 0` | `not (C == 0)` |
| `blo` at 0xe1870 | `C == 0` | `not (C == 0)` |
| `bls` at 0xe19b0 | `([sp-36] + [sp-24]) <= 0x1f (unsigned, 32-bit)` | `([sp-36] + [sp-24]) > 0x1f (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe1a20 | `r0` | `?` |

## `add_sic_tdi`

- `add_sic_tdi`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe1a30 to 0xe1c84.
  Established. A four-byte load reads `r3 +0x0`, with its value retained at `sp -0x28` (`ldr` at
  0xe1b78, `str` at 0xe1b7c). A later four-byte store through `[sp-32] +0x0` writes `([[sp-32]] + [sp-24])` (`str` at 0xe1c64). The unsigned-lower-or-same branch takes `([sp-40] + [sp-24]) <= 0x20` to `0xe1c50` (`bls` at 0xe1c04). The exit carries `[[sp-32]]` (`ldmdb` at 0xe1c74).
  Scan-data representation is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe1a40 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe1a44 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe1a48 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe1a4c | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xe1a54 | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0xe1a5c | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xe1a60 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe1a64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe1a78 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe1a80 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe1a88 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe1a8c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe1a94 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe1a9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe1aa0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe1aa8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe1aac | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe1ab0 | load | 4 | `abs 0xe1c78` | `-` |
| `ldr` at 0xe1abc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe1ac4 | load | 4 | `abs 0xe1c78` | `-` |
| `str` at 0xe1ad0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe1ad4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe1ad8 | load | 4 | `abs 0xe1c7c` | `-` |
| `ldr` at 0xe1ae4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe1aec | load | 4 | `abs 0xe1c7c` | `-` |
| `str` at 0xe1af8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe1afc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe1b00 | load | 4 | `abs 0xe1c80` | `-` |
| `ldr` at 0xe1b0c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe1b14 | load | 4 | `abs 0xe1c80` | `-` |
| `str` at 0xe1b20 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe1b24 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe1b34 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe1b48 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe1b4c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe1b5c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe1b70 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe1b74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe1b78 | load | 4 | `r3 +0x0` | `-` |
| `str` at 0xe1b7c | store | 4 | `sp -0x28` | `[r3]` |
| `str` at 0xe1b84 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe1b88 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe1b98 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe1ba0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe1ba4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe1bac | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe1bb4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe1bbc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe1bc4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe1bcc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe1bd4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe1bd8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe1be0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe1be8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe1bec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe1bf0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe1bf8 | store | 4 | `sp -0x28` | `([sp-40] + [sp-24])` |
| `ldr` at 0xe1bfc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe1c08 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe1c10 | store | 4 | `sp -0x28` | `([sp-40] + [sp-24])-32` |
| `ldr` at 0xe1c14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe1c18 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe1c20 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe1c24 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe1c2c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xe1c30 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe1c38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe1c40 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe1c44 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe1c4c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe1c50 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe1c54 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe1c58 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xe1c5c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe1c64 | store | 4 | `[sp-32] +0x0` | `([[sp-32]] + [sp-24])` |
| `ldr` at 0xe1c68 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe1c6c | load | 4 | `[sp-32] +0x0` | `-` |
| `ldmdb` at 0xe1c74 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe1c74 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe1c74 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xe1a6c | `C == 0` | `not (C == 0)` |
| `bhi` at 0xe1b90 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bls` at 0xe1c04 | `([sp-40] + [sp-24]) <= 0x20 (unsigned, 32-bit)` | `([sp-40] + [sp-24]) > 0x20 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe1c74 | `r0` | `[[sp-32]]` |

## `scan_multi_sic_ir`

- `scan_multi_sic_ir`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe1c84 to 0xe2060.
  Established. The unnamed `0xe17d4` call has `sp-328`, unknown, unknown, `sp-136` (`bl` at
  0xe1d48). The subsequent unnamed `0xdd8e8` call carries `0x0`, `[sp-32]`, `[sp]`, `[sp+4]` (`bl`
  at 0xe1d84); its nonzero result takes the branch to `0xe1da0` (`bne` at 0xe1d90). A separate
  branch takes zero `[sp+8]` to `0xe2038` (`beq` at 0xe1da8). Multi-SIC hardware effects remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe1c94 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe1c98 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe1c9c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe1ca0 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xe1ca8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe1cac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe1cb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe1cc0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe1cc4 | load | 4 | `abs 0xe2048` | `-` |
| `ldr` at 0xe1cd8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe1cdc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe1ce0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe1cfc | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe1d00 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe1d1c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe1d20 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe1d28 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xe1d3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe1d40 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe1d50 | store | 4 | `sp -0x28` | `ret(0xe17d4)` |
| `ldr` at 0xe1d54 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe1d58 | store | 4 | `sp -0x168` | `ret(0xe17d4)` |
| `str` at 0xe1d60 | store | 4 | `sp -0x164` | `sp-136` |
| `str` at 0xe1d68 | store | 4 | `sp -0x160` | `sp-232` |
| `ldr` at 0xe1d6c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe1d70 | store | 4 | `sp -0x15c` | `[sp+12]` |
| `ldr` at 0xe1d78 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe1d7c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe1d80 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xe1d98 | store | 4 | `sp -0x158` | `0x0` |
| `ldr` at 0xe1da0 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xe1dac | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe1db4 | store | 4 | `sp -0x14c` | `?` |
| `ldr` at 0xe1db8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe1dc0 | store | 4 | `sp -0x150` | `?` |
| `str` at 0xe1dc8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe1dcc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe1dd0 | load | 4 | `sp -0x14c` | `-` |
| `ldr` at 0xe1de0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe1df8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe1e00 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xe1e08 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe1e0c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe1e18 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe1e20 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xe1e28 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe1e40 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe1e44 | store | 4 | `sp -0x168` | `?` |
| `ldr` at 0xe1e48 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xe1e4c | load | 4 | `abs 0xe204c` | `-` |
| `ldr` at 0xe1e50 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe1e54 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe1e60 | store | 4 | `sp -0x158` | `0x0` |
| `ldr` at 0xe1e68 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe1e70 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `str` at 0xe1e7c | store | 4 | `sp -0x154` | `0x0` |
| `str` at 0xe1e84 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe1e88 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe1e8c | load | 4 | `sp -0x150` | `-` |
| `ldr` at 0xe1ea0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe1ea8 | load | 4 | `sp -0x154` | `-` |
| `str` at 0xe1eb0 | store | 4 | `sp -0x154` | `?` |
| `ldr` at 0xe1eb4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe1ebc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe1ec4 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xe1ec8 | load | 4 | `abs 0xe2050` | `-` |
| `ldr` at 0xe1ed4 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xe1edc | load | 4 | `abs 0xe2050` | `-` |
| `str` at 0xe1ee8 | store | 4 | `sp -0x154` | `?` |
| `ldr` at 0xe1eec | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xe1ef0 | load | 4 | `abs 0xe2054` | `-` |
| `ldr` at 0xe1efc | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xe1f04 | load | 4 | `abs 0xe2054` | `-` |
| `str` at 0xe1f10 | store | 4 | `sp -0x154` | `?` |
| `ldr` at 0xe1f14 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xe1f18 | load | 4 | `abs 0xe2058` | `-` |
| `ldr` at 0xe1f24 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xe1f2c | load | 4 | `abs 0xe2058` | `-` |
| `str` at 0xe1f38 | store | 4 | `sp -0x154` | `?` |
| `ldr` at 0xe1f3c | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xe1f4c | load | 4 | `sp -0x154` | `-` |
| `str` at 0xe1f60 | store | 4 | `sp -0x154` | `?` |
| `ldr` at 0xe1f64 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xe1f74 | load | 4 | `sp -0x154` | `-` |
| `str` at 0xe1f88 | store | 4 | `sp -0x154` | `?` |
| `ldr` at 0xe1f8c | load | 4 | `sp -0x14c` | `-` |
| `ldr` at 0xe1fa4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe1fa8 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xe1fb0 | load | 4 | `sp -0x14c` | `-` |
| `ldr` at 0xe1fb8 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xe1fc0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe1fc4 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xe1fd4 | load | 4 | `sp -0x14c` | `-` |
| `ldr` at 0xe1fdc | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xe1fe4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe1fe8 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xe1ff0 | load | 4 | `sp -0x14c` | `-` |
| `ldr` at 0xe2008 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe200c | load | 4 | `sp -0x154` | `-` |
| `str` at 0xe2014 | store | 4 | `sp -0x168` | `?` |
| `ldr` at 0xe2018 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xe201c | load | 4 | `abs 0xe205c` | `-` |
| `ldr` at 0xe2020 | load | 4 | `sp -0x14c` | `-` |
| `str` at 0xe2030 | store | 4 | `sp -0x158` | `0x0` |
| `str` at 0xe203c | store | 4 | `sp -0x158` | `0x1` |
| `ldr` at 0xe2040 | load | 4 | `sp -0x158` | `-` |
| `ldmdb` at 0xe2044 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe2044 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe2044 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe1d48 | `(unknown callee) 0xe17d4` | `sp-328` | `?` | `?` | `sp-136` |
| `bl` at 0xe1d84 | `(unknown callee) 0xdd8e8` | `0x0` | `[sp-32]` | `[sp]` | `[sp+4]` |
| `bl` at 0xe1e58 | `(unknown callee) 0x1b52c` | `[sp+12]` | `0x13606c` | `[sp-36]` | `?` |
| `bl` at 0xe2028 | `(unknown callee) 0x1b52c` | `[sp+12]` | `0x1360bc` | `[sp-332]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xe1cb8 | `C == 0` | `not (C == 0)` |
| `bne` at 0xe1d90 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `beq` at 0xe1da8 | `[sp+8] == 0x0` | `[sp+8] != 0x0` |
| `blo` at 0xe1dd8 | `C == 0` | `not (C == 0)` |
| `beq` at 0xe1e14 | `? == ?` | `? != ?` |
| `blo` at 0xe1e94 | `C == 0` | `not (C == 0)` |
| `beq` at 0xe1fd0 | `? == ?` | `? != ?` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe2044 | `r0` | `?` |

## `JTAG_Cfg_create`

- `JTAG_Cfg_create`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe6ab4 to 0xe6cac.
  Established. Four-byte global assignments write `0x0` at `0x2adfe8` and `0x1` at `0x2adfe4` (`str`
  at 0xe6b88, `str` at 0xe6b94). Four additional explicit zero stores have unknown targets (`str` at
  0xe6bec, `str` at 0xe6c18, `str` at 0xe6c44, `str` at 0xe6c70). The exit carries `0x1` (`ldmdb` at
  0xe6c90). Configuration-record shape and ownership remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe6ac4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xe6ac8 | load | 4 | `abs 0xe6c94` | `-` |
| `ldr` at 0xe6acc | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xe6ad8 | load | 4 | `abs 0xe6c98` | `-` |
| `str` at 0xe6adc | store | 4 | `sp -0x24` | `0x1364c8` |
| `ldr` at 0xe6ae0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe6ae4 | store | 4 | `sp -0x20` | `r0` |
| `ldr` at 0xe6aec | load | 4 | `abs 0xe6c9c` | `-` |
| `str` at 0xe6b00 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xe6b04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6b14 | load | 4 | `abs 0xe6ca0` | `-` |
| `ldr` at 0xe6b18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6b38 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe6b44 | load | 4 | `abs 0xe6ca0` | `-` |
| `ldr` at 0xe6b48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6b68 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe6b70 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6b78 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xe6b80 | load | 4 | `abs 0xe6ca4` | `-` |
| `str` at 0xe6b88 | store | 4 | `abs 0x2adfe8` | `0x0` |
| `ldr` at 0xe6b8c | load | 4 | `abs 0xe6ca8` | `-` |
| `str` at 0xe6b94 | store | 4 | `abs 0x2adfe4` | `0x1` |
| `ldr` at 0xe6b98 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe6ba8 | store | 4 | `sp -0x1c` | `0x1` |
| `str` at 0xe6bb4 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xe6bb8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6bbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6bcc | load | 4 | `abs 0xe6ca0` | `-` |
| `ldr` at 0xe6bd0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6bec | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe6bf0 | load | 4 | `abs 0xe6ca0` | `-` |
| `ldr` at 0xe6bf4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6c18 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe6c1c | load | 4 | `abs 0xe6ca0` | `-` |
| `ldr` at 0xe6c20 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6c44 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe6c48 | load | 4 | `abs 0xe6ca0` | `-` |
| `ldr` at 0xe6c4c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6c70 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe6c74 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6c7c | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0xe6c88 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0xe6c8c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xe6c90 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe6c90 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe6c90 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe6af8 | `(unknown callee) 0x21eb8` | `0x6` | `0x1364e0` | `0x7b` | `0x0` |
| `bl` at 0xe6b6c | `(unknown callee) 0x1b490` | `?` | `0xc` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe6ad4 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `bls` at 0xe6b0c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0xe6b40 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xe6ba0 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `blo` at 0xe6bc4 | `C == 0` | `not (C == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe6c90 | `r0` | `0x1` |

## `JTAG_Cfg_info_init`

- `JTAG_Cfg_info_init`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe6cac to 0xe7020.
  Established. The entry branch takes `[0x2adfd8] <= 0x2`, unsigned 32-bit, to `0xe6d00` (`bls` at
  0xe6cd0). A four-byte assignment at an unknown target retains `ret(0x1b1b4)` (`str` at 0xe6e68).
  Included halfword and byte stores at unknown targets each write `0x0` (`strh` at 0xe6ef4, `strb`
  at 0xe6f30, `strb` at 0xe6f6c). Information-record layout and allocation effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe6cbc | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe6cc0 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xe6cc4 | load | 4 | `abs 0xe700c` | `-` |
| `ldr` at 0xe6cc8 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xe6cd4 | load | 4 | `abs 0xe7010` | `-` |
| `str` at 0xe6cd8 | store | 4 | `sp -0x30` | `0x1364fc` |
| `ldr` at 0xe6cdc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6ce0 | store | 4 | `sp -0x2c` | `r0` |
| `ldr` at 0xe6ce4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe6ce8 | store | 4 | `sp -0x28` | `r1` |
| `ldr` at 0xe6cf0 | load | 4 | `abs 0xe7014` | `-` |
| `ldr` at 0xe6d00 | load | 4 | `abs 0xe7018` | `-` |
| `ldr` at 0xe6d04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6d24 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe6d28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe6d34 | load | 4 | `abs 0xe7018` | `-` |
| `ldr` at 0xe6d38 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6d5c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe6d60 | load | 4 | `abs 0xe7018` | `-` |
| `ldr` at 0xe6d64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6d84 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe6d90 | load | 4 | `abs 0xe7018` | `-` |
| `ldr` at 0xe6d94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6db4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe6dbc | load | 4 | `abs 0xe7018` | `-` |
| `ldr` at 0xe6dc0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6de4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe6de8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe6df8 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0xe6e00 | load | 4 | `abs 0xe7018` | `-` |
| `ldr` at 0xe6e04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6e24 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe6e30 | load | 4 | `abs 0xe7018` | `-` |
| `ldr` at 0xe6e34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6e54 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe6e68 | store | 4 | `(unknown)` | `ret(0x1b1b4)` |
| `ldr` at 0xe6e6c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe6e78 | load | 4 | `abs 0xe701c` | `-` |
| `str` at 0xe6e7c | store | 4 | `sp -0x30` | `0x136524` |
| `ldr` at 0xe6e84 | load | 4 | `abs 0xe7014` | `-` |
| `str` at 0xe6e98 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xe6ea4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xe6ea8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe6eac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe6ebc | load | 4 | `abs 0xe7018` | `-` |
| `ldr` at 0xe6ec0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6ee0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe6ee8 | load | 4 | `(unknown)` | `-` |
| `strh` at 0xe6ef4 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0xe6ef8 | load | 4 | `abs 0xe7018` | `-` |
| `ldr` at 0xe6efc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6f1c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe6f24 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xe6f30 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xe6f34 | load | 4 | `abs 0xe7018` | `-` |
| `ldr` at 0xe6f38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6f58 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe6f60 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xe6f6c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xe6f70 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe6f78 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe6f80 | load | 4 | `abs 0xe7018` | `-` |
| `ldr` at 0xe6f84 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6fa0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe6fa4 | load | 4 | `abs 0xe7018` | `-` |
| `ldr` at 0xe6fa8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6fcc | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe6fd0 | load | 4 | `abs 0xe7018` | `-` |
| `ldr` at 0xe6fd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6ff4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe6ff8 | store | 4 | `(unknown)` | `[sp-28]` |
| `str` at 0xe7000 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0xe7004 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xe7008 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe7008 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe7008 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe7008 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe6cfc | `(unknown callee) 0x21eb8` | `0x6` | `0x1364e0` | `0xa3` | `0x0` |
| `bl` at 0xe6db8 | `(unknown callee) 0x1b490` | `?` | `0xc` | `[sp-24]` | `?` |
| `bl` at 0xe6e60 | `(unknown callee) 0x1b1b4` | `?` | `0xc` | `?` | `?` |
| `bl` at 0xe6e90 | `(unknown callee) 0x21eb8` | `0x3` | `0x1364e0` | `0xb7` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xe6cd0 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xe6d30 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xe6d8c | `? == 0x0` | `? != 0x0` |
| `bne` at 0xe6df0 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xe6e2c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe6e74 | `? != 0x0` | `? == 0x0` |
| `blo` at 0xe6eb4 | `C == 0` | `not (C == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe7008 | `r0` | `?` |

## `JTAG_Cfg_port_init`

- `JTAG_Cfg_port_init`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe7020 to 0xe7424.
  Established. Entry `r2 != 0x0` takes the branch to `0xe7068` (`bne` at 0xe7048). The low byte of
  entry `r3` is retained at `sp -0x21` (`strb` at 0xe703c). A later four-byte stack assignment at
  `sp -0x2c` writes `0xfffff7cc` (`str` at 0xe7248). Included unnamed calls to `0xe7424` and
  `0xea0e8` both have first candidate `[sp-24]` (`bl` at 0xe73ec, `bl` at 0xe73f4). Port-record
  fields and hardware effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe7030 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe7034 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xe7038 | store | 4 | `sp -0x20` | `r2` |
| `strb` at 0xe703c | store | 1 | `sp -0x21` | `(r3 & 0xff)` |
| `ldr` at 0xe7040 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe704c | load | 4 | `abs 0xe7408` | `-` |
| `str` at 0xe7050 | store | 4 | `sp -0x38` | `0x136554` |
| `ldr` at 0xe7058 | load | 4 | `abs 0xe740c` | `-` |
| `ldr` at 0xe7068 | load | 4 | `abs 0xe7410` | `-` |
| `ldr` at 0xe706c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xe7078 | load | 4 | `abs 0xe7414` | `-` |
| `str` at 0xe707c | store | 4 | `sp -0x38` | `0x136580` |
| `ldr` at 0xe7080 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe7084 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe7088 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe708c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe7094 | load | 4 | `abs 0xe740c` | `-` |
| `ldr` at 0xe70a4 | load | 4 | `abs 0xe7418` | `-` |
| `ldr` at 0xe70a8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe70c4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe70c8 | load | 4 | `abs 0xe7418` | `-` |
| `ldr` at 0xe70cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe70ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe70f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe70fc | load | 4 | `abs 0xe7418` | `-` |
| `ldr` at 0xe7100 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe7124 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe7128 | load | 4 | `abs 0xe7418` | `-` |
| `ldr` at 0xe712c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe714c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe7158 | load | 4 | `abs 0xe7418` | `-` |
| `ldr` at 0xe715c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe717c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe7184 | load | 4 | `abs 0xe7418` | `-` |
| `ldr` at 0xe7188 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe71ac | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe71b0 | load | 4 | `abs 0xe7418` | `-` |
| `ldr` at 0xe71b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe71d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe71e0 | load | 4 | `abs 0xe7418` | `-` |
| `ldr` at 0xe71e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7204 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe7218 | store | 4 | `(unknown)` | `ret(0x1b1b4)` |
| `ldr` at 0xe721c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe7228 | load | 4 | `abs 0xe741c` | `-` |
| `str` at 0xe722c | store | 4 | `sp -0x38` | `0x136524` |
| `ldr` at 0xe7234 | load | 4 | `abs 0xe740c` | `-` |
| `ldr` at 0xe7244 | load | 4 | `abs 0xe7420` | `-` |
| `str` at 0xe7248 | store | 4 | `sp -0x2c` | `0xfffff7cc` |
| `str` at 0xe7254 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe7258 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe725c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe726c | load | 4 | `abs 0xe7418` | `-` |
| `ldr` at 0xe7270 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7290 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7298 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe72a0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe72a8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xe72b0 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xe72b4 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe72b8 | load | 4 | `abs 0xe7418` | `-` |
| `ldr` at 0xe72bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe72dc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe72e4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe72ec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe72f4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xe72fc | load | 1 | `(unknown)` | `-` |
| `strb` at 0xe7300 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe7304 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe730c | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xe7314 | load | 4 | `abs 0xe7418` | `-` |
| `ldr` at 0xe7318 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe7334 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0xe7338 | load | 4 | `abs 0xe7418` | `-` |
| `ldr` at 0xe733c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe7360 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe7364 | load | 4 | `abs 0xe7418` | `-` |
| `ldr` at 0xe7368 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7388 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe738c | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0xe7390 | load | 4 | `abs 0xe7418` | `-` |
| `ldr` at 0xe7394 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xe73b4 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0xe73b8 | store | 1 | `(unknown)` | `([sp-33]:1 & 0xff)` |
| `ldr` at 0xe73bc | load | 4 | `abs 0xe7418` | `-` |
| `ldr` at 0xe73c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe73e0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xe73e4 | store | 4 | `(unknown)` | `[sp]` |
| `ldr` at 0xe73e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe73f0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe73fc | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe7400 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xe7404 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe7404 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe7404 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe7404 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe7064 | `(unknown callee) 0x21eb8` | `0x9` | `0x1364e0` | `0xd4` | `0x0` |
| `bl` at 0xe70a0 | `(unknown callee) 0x21eb8` | `0x6` | `0x1364e0` | `0xd7` | `0x0` |
| `bl` at 0xe7180 | `(unknown callee) 0x1b490` | `?` | `0xc` | `[sp-24]` | `?` |
| `bl` at 0xe7210 | `(unknown callee) 0x1b1b4` | `?` | `0xc` | `[sp-24]` | `?` |
| `bl` at 0xe7240 | `(unknown callee) 0x21eb8` | `0x3` | `0x1364e0` | `0xec` | `0x0` |
| `bl` at 0xe73ec | `(unknown callee) 0xe7424` | `[sp-24]` | `0x14` | `?` | `[sp]` |
| `bl` at 0xe73f4 | `(unknown callee) 0xea0e8` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe7048 | `r2 != 0x0` | `r2 == 0x0` |
| `bls` at 0xe7074 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xe70f8 | `? == [sp-32]` | `? != [sp-32]` |
| `beq` at 0xe7154 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xe71dc | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe7224 | `? != 0x0` | `? == 0x0` |
| `blo` at 0xe7264 | `C == 0` | `not (C == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe7404 | `r0` | `?` |

## `JTAG_Cfg_calc_ir_vals`

- `JTAG_Cfg_calc_ir_vals`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe7424 to 0xe75ac.
  Established. The halfword read at `sp -0x1c` supplies an included halfword store at an unknown
  target (`ldrh` at 0xe74e8, `strh` at 0xe74ec). A later four-byte store has unknown target and
  value (`str` at 0xe756c). The branch takes `[0x2adfd8] <= 0x3`, unsigned 32-bit, to the exit at
  `0xe7590` (`bls` at 0xe757c); its fall-through includes an unknown-target call with `0x2b4640`,
  `0x4`, unknown, `[0x143860]` (`mov` at 0xe758c). Instruction-register lengths and callback effects
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe7434 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xe7438 | load | 4 | `abs 0xe7594` | `-` |
| `ldr` at 0xe743c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xe7448 | load | 4 | `abs 0xe7598` | `-` |
| `str` at 0xe744c | store | 4 | `sp -0x24` | `0x1365a8` |
| `ldr` at 0xe7450 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe7454 | store | 4 | `sp -0x20` | `r0` |
| `ldr` at 0xe745c | load | 4 | `abs 0xe759c` | `-` |
| `ldr` at 0xe7460 | load | 4 | `abs 0xe75a0` | `-` |
| `str` at 0xe7470 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0xe7478 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xe747c | load | 4 | `abs 0xe75a4` | `-` |
| `ldr` at 0xe7480 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe74a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe74a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe74b4 | load | 4 | `abs 0xe75a4` | `-` |
| `ldr` at 0xe74b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe74d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe74e0 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe74e8 | load | 2 | `sp -0x1c` | `-` |
| `strh` at 0xe74ec | store | 2 | `(unknown)` | `([sp-28]:2 & 0xffff)` |
| `ldr` at 0xe74f0 | load | 4 | `abs 0xe75a4` | `-` |
| `ldr` at 0xe74f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7514 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe751c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xe7524 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xe7528 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe7530 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xe7534 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe753c | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0xe7544 | load | 4 | `abs 0xe75a4` | `-` |
| `ldr` at 0xe7548 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7568 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe756c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe7570 | load | 4 | `abs 0xe7594` | `-` |
| `ldr` at 0xe7574 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xe7580 | load | 4 | `abs 0xe75a8` | `-` |
| `ldr` at 0xe7584 | load | 4 | `abs 0x143860` | `-` |
| `ldmdb` at 0xe7590 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe7590 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe7590 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe7468 | `(unknown callee) 0x21eb8` | `0x6` | `0x1364e0` | `0x113` | `0x0` |
| `mov` at 0xe758c | `(unknown callee) (unknown target)` | `0x2b4640` | `0x4` | `?` | `[0x143860]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xe7444 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `blo` at 0xe74ac | `C == 0` | `not (C == 0)` |
| `bls` at 0xe757c | `[0x2adfd8] <= 0x3 (unsigned, 32-bit)` | `[0x2adfd8] > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe7590 | `r0` | `?` |

## `JTAG_Cfg_set_ports_f`

- `JTAG_Cfg_set_ports_f`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe75ac to 0xe77a4.
  Established. Two four-byte global assignments target `0x2adfe4` with unknown value and `0x2adfe8`
  with `[sp-24]` (`str` at 0xe76fc, `str` at 0xe7708). Separate byte loads read these globals
  (`ldrb` at 0xe7710, `ldrb` at 0xe7718). The unnamed `0xe3c98` call has `[0x2adfe4]:1`,
  `[0x2adfe8]:1`, `[0x2adfe4]:1`, `[0x2adfe8]:1` (`bl` at 0xe7724). Port-selection meaning and
  external effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe75bc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe75c0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xe75c4 | load | 4 | `abs 0xe777c` | `-` |
| `ldr` at 0xe75c8 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xe75d4 | load | 4 | `abs 0xe7780` | `-` |
| `str` at 0xe75d8 | store | 4 | `sp -0x38` | `0x1365c8` |
| `ldr` at 0xe75dc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe75e0 | store | 4 | `sp -0x34` | `r0` |
| `ldr` at 0xe75e4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe75e8 | store | 4 | `sp -0x30` | `r1` |
| `ldr` at 0xe75f0 | load | 4 | `abs 0xe7784` | `-` |
| `ldr` at 0xe75f4 | load | 4 | `abs 0xe7788` | `-` |
| `ldr` at 0xe7600 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe760c | load | 4 | `abs 0xe778c` | `-` |
| `ldr` at 0xe7610 | load | 4 | `abs 0x2adfe8` | `-` |
| `str` at 0xe7614 | store | 4 | `sp -0x18` | `[0x2adfe8]` |
| `ldr` at 0xe7618 | load | 4 | `abs 0xe7790` | `-` |
| `ldr` at 0xe761c | load | 4 | `abs 0x2adfe4` | `-` |
| `str` at 0xe7620 | store | 4 | `sp -0x14` | `[0x2adfe4]` |
| `ldr` at 0xe7628 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7638 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe7640 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xe7644 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe764c | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xe7650 | load | 4 | `abs 0xe7790` | `-` |
| `ldr` at 0xe7654 | load | 4 | `abs 0x2adfe4` | `-` |
| `ldr` at 0xe7658 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7664 | load | 4 | `abs 0xe778c` | `-` |
| `ldr` at 0xe7668 | load | 4 | `abs 0x2adfe8` | `-` |
| `ldr` at 0xe766c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe767c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe7684 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7690 | load | 4 | `abs 0xe7794` | `-` |
| `str` at 0xe7694 | store | 4 | `sp -0x38` | `0x1365fc` |
| `ldr` at 0xe7698 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe769c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe76a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe76a4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe76ac | load | 4 | `abs 0xe7784` | `-` |
| `ldr` at 0xe76b0 | load | 4 | `abs 0xe7798` | `-` |
| `ldr` at 0xe76bc | load | 4 | `abs 0xe779c` | `-` |
| `str` at 0xe76c0 | store | 4 | `sp -0x24` | `0xfffff7c3` |
| `ldr` at 0xe76c8 | load | 4 | `abs 0xe7790` | `-` |
| `str` at 0xe76cc | store | 4 | `sp -0x28` | `0x2adfe4` |
| `ldr` at 0xe76d0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe76d4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe76d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe76e8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe76f0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe76f4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe76f8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe76fc | store | 4 | `abs 0x2adfe4` | `?` |
| `ldr` at 0xe7700 | load | 4 | `abs 0xe778c` | `-` |
| `ldr` at 0xe7704 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe7708 | store | 4 | `abs 0x2adfe8` | `[sp-24]` |
| `ldr` at 0xe770c | load | 4 | `abs 0xe7790` | `-` |
| `ldrb` at 0xe7710 | load | 1 | `abs 0x2adfe4` | `-` |
| `ldr` at 0xe7714 | load | 4 | `abs 0xe778c` | `-` |
| `ldrb` at 0xe7718 | load | 1 | `abs 0x2adfe8` | `-` |
| `ldr` at 0xe7728 | load | 4 | `abs 0xe777c` | `-` |
| `ldr` at 0xe772c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xe7738 | load | 4 | `abs 0xe77a0` | `-` |
| `str` at 0xe773c | store | 4 | `sp -0x38` | `0x13663c` |
| `ldr` at 0xe7740 | load | 4 | `abs 0xe7790` | `-` |
| `ldr` at 0xe7744 | load | 4 | `abs 0x2adfe4` | `-` |
| `str` at 0xe7748 | store | 4 | `sp -0x34` | `[0x2adfe4]` |
| `ldr` at 0xe774c | load | 4 | `abs 0xe778c` | `-` |
| `ldr` at 0xe7750 | load | 4 | `abs 0x2adfe8` | `-` |
| `str` at 0xe7754 | store | 4 | `sp -0x30` | `[0x2adfe8]` |
| `ldr` at 0xe775c | load | 4 | `abs 0xe7784` | `-` |
| `str` at 0xe7770 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe7774 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xe7778 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe7778 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe7778 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe75fc | `(unknown callee) 0x21eb8` | `0x6` | `0x1364e0` | `0x12a` | `0x0` |
| `bl` at 0xe76b8 | `(unknown callee) 0x21eb8` | `0x3` | `0x1364e0` | `0x141` | `0x0` |
| `bl` at 0xe7724 | `(unknown callee) 0xe3c98` | `[0x2adfe4]:1` | `[0x2adfe8]:1` | `[0x2adfe4]:1` | `[0x2adfe8]:1` |
| `bl` at 0xe7768 | `(unknown callee) 0x21eb8` | `0x6` | `0x1364e0` | `0x160` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xe75d0 | `[0x2adfd8] <= 0x3 (unsigned, 32-bit)` | `[0x2adfd8] > 0x3 (unsigned, 32-bit)` |
| `bne` at 0xe7608 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xe7630 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bne` at 0xe7660 | `[0x2adfe4] != [sp-20]` | `[0x2adfe4] == [sp-20]` |
| `bne` at 0xe7674 | `[0x2adfe8] != [sp-24]` | `[0x2adfe8] == [sp-24]` |
| `bls` at 0xe768c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xe76e0 | `Z == 0` | `not (Z == 0)` |
| `bls` at 0xe7734 | `[0x2adfd8] <= 0x3 (unsigned, 32-bit)` | `[0x2adfd8] > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe7778 | `r0` | `?` |

## `JTAG_Cfg_find_chip`

- `JTAG_Cfg_find_chip`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe77a4 to 0xe7a3c.
  Established. The unnamed `0x21eb8` call has `0x6`, `0x1364e0`, `0x17b`, `0x0` (`bl` at 0xe780c).
  Included byte reads have unknown effective targets (`ldrb` at 0xe7928, `ldrb` at 0xe79d0). The
  unsigned-higher-or-same branch takes `C == 1` to `0xe789c` (`bhs` at 0xe7894). The exit carries
  `[sp-44]` (`ldmdb` at 0xe7a24). Chip matching criteria remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe77b4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe77b8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe77bc | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe77c0 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xe77c4 | load | 4 | `abs 0xe7a28` | `-` |
| `ldr` at 0xe77c8 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xe77d4 | load | 4 | `abs 0xe7a2c` | `-` |
| `str` at 0xe77d8 | store | 4 | `sp -0x40` | `0x136670` |
| `ldr` at 0xe77dc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe77e0 | store | 4 | `sp -0x3c` | `r0` |
| `ldr` at 0xe77e4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe77e8 | store | 4 | `sp -0x38` | `r1` |
| `ldr` at 0xe77ec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe77f0 | store | 4 | `sp -0x34` | `r2` |
| `ldr` at 0xe77f4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe77f8 | store | 4 | `sp -0x30` | `r3` |
| `ldr` at 0xe7800 | load | 4 | `abs 0xe7a30` | `-` |
| `ldr` at 0xe7804 | load | 4 | `abs 0xe7a34` | `-` |
| `ldr` at 0xe7810 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe781c | load | 4 | `abs 0xe7a38` | `-` |
| `ldr` at 0xe7820 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7840 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe7850 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe7858 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7864 | load | 4 | `abs 0xe7a38` | `-` |
| `ldr` at 0xe7868 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7888 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe788c | load | 4 | `(unknown)` | `-` |
| `str` at 0xe78a0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe78a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe78b4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe78b8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe78bc | load | 4 | `abs 0xe7a38` | `-` |
| `ldr` at 0xe78c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe78e0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe78e4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe78f4 | load | 4 | `abs 0xe7a38` | `-` |
| `ldr` at 0xe78f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7918 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe7920 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xe7928 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xe792c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe7938 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe793c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe7944 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe794c | store | 4 | `sp -0x24` | `?` |
| `str` at 0xe7958 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xe7960 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe7964 | load | 4 | `abs 0xe7a38` | `-` |
| `ldr` at 0xe7968 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7988 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe798c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe799c | load | 4 | `abs 0xe7a38` | `-` |
| `ldr` at 0xe79a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe79c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe79c8 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xe79d0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xe79d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe79e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe79e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe79f0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe79f4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe79fc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe7a04 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe7a08 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe7a10 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xe7a1c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe7a20 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xe7a24 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe7a24 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe7a24 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe780c | `(unknown callee) 0x21eb8` | `0x6` | `0x1364e0` | `0x17b` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xe77d0 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bhi` at 0xe7818 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0xe7848 | `? != 0x0` | `? == 0x0` |
| `blt` at 0xe7860 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bhs` at 0xe7894 | `C == 1` | `not (C == 1)` |
| `bne` at 0xe78b0 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0xe78ec | `C == 0` | `not (C == 0)` |
| `bne` at 0xe7934 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0xe7994 | `C == 0` | `not (C == 0)` |
| `bne` at 0xe79dc | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xe79ec | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe7a24 | `r0` | `[sp-44]` |

## `JTAG_Cfg_pid`

- `JTAG_Cfg_pid`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe7a3c to 0xe7a7c.
  Established. Entry `r0` is assigned as four bytes at `sp -0x14` (`str` at 0xe7a4c). A later
  four-byte load has unknown effective target, followed by an unknown exit value (`ldr` at 0xe7a6c,
  `ldmdb` at 0xe7a74). There are no calls or conditional branches in this span. The returned value's
  meaning is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe7a4c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xe7a50 | load | 4 | `abs 0xe7a78` | `-` |
| `ldr` at 0xe7a54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7a6c | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0xe7a74 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe7a74 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe7a74 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe7a74 | `r0` | `?` |

## `JTAG_Cfg_display_f`

- `JTAG_Cfg_display_f`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe7a7c to 0xe7d38.
  Established. Included unnamed `0x23fbc` calls have `0x1366d4`, `[0x2adfe4]`, unknown, `0x2adfe4`,
  and `0x1366e4`, `[0x2adfe8]`, unknown, `0x2adfe8` (`bl` at 0xe7acc, `bl` at 0xe7adc). Later
  halfword and byte reads have unknown targets (`ldrh` at 0xe7c4c, `ldrb` at 0xe7c84, `ldrb` at
  0xe7cbc). The exit carries `ret(0x23fbc)` (`ldmdb` at 0xe7d00). Display formatting and record
  meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xe7a8c | load | 4 | `abs 0xe7d04` | `-` |
| `ldr` at 0xe7a90 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xe7a9c | load | 4 | `abs 0xe7d08` | `-` |
| `str` at 0xe7aa0 | store | 4 | `sp -0x20` | `0x1366a8` |
| `ldr` at 0xe7aa8 | load | 4 | `abs 0xe7d0c` | `-` |
| `ldr` at 0xe7aac | load | 4 | `abs 0xe7d10` | `-` |
| `ldr` at 0xe7ab8 | load | 4 | `abs 0xe7d14` | `-` |
| `ldr` at 0xe7ac0 | load | 4 | `abs 0xe7d18` | `-` |
| `ldr` at 0xe7ac4 | load | 4 | `abs 0xe7d1c` | `-` |
| `ldr` at 0xe7ac8 | load | 4 | `abs 0x2adfe4` | `-` |
| `ldr` at 0xe7ad0 | load | 4 | `abs 0xe7d20` | `-` |
| `ldr` at 0xe7ad4 | load | 4 | `abs 0xe7d24` | `-` |
| `ldr` at 0xe7ad8 | load | 4 | `abs 0x2adfe8` | `-` |
| `str` at 0xe7ae4 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0xe7ae8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7af8 | load | 4 | `abs 0xe7d28` | `-` |
| `ldr` at 0xe7afc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7b1c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe7b2c | load | 4 | `abs 0xe7d28` | `-` |
| `ldr` at 0xe7b30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7b48 | load | 4 | `abs 0xe7d28` | `-` |
| `ldr` at 0xe7b4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7b6c | load | 4 | `abs 0xe7d28` | `-` |
| `ldr` at 0xe7b70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7b90 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe7b94 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe7b98 | load | 4 | `abs 0xe7d28` | `-` |
| `ldr` at 0xe7b9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7bbc | load | 4 | `(unknown)` | `-` |
| `str` at 0xe7bc0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xe7bc4 | load | 4 | `abs 0xe7d2c` | `-` |
| `ldr` at 0xe7bc8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7bcc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe7bd0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe7bdc | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xe7be0 | load | 4 | `abs 0xe7d28` | `-` |
| `ldr` at 0xe7be4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7c04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7c08 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe7c18 | load | 4 | `abs 0xe7d28` | `-` |
| `ldr` at 0xe7c1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7c3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7c44 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe7c4c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe7c50 | load | 4 | `abs 0xe7d28` | `-` |
| `ldr` at 0xe7c54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7c74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7c7c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xe7c84 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xe7c88 | load | 4 | `abs 0xe7d28` | `-` |
| `ldr` at 0xe7c8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7cac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7cb4 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xe7cbc | load | 1 | `(unknown)` | `-` |
| `str` at 0xe7cc0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe7cc4 | load | 4 | `abs 0xe7d30` | `-` |
| `ldr` at 0xe7cc8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7cd8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe7ce0 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0xe7ce8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe7cf0 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xe7cf8 | load | 4 | `abs 0xe7d34` | `-` |
| `ldmdb` at 0xe7d00 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe7d00 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe7d00 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe7ab4 | `(unknown callee) 0x21eb8` | `0x6` | `0x1364e0` | `0x1b1` | `0x0` |
| `bl` at 0xe7abc | `(unknown callee) 0x23fbc` | `0x1366bc` | `?` | `?` | `?` |
| `bl` at 0xe7acc | `(unknown callee) 0x23fbc` | `0x1366d4` | `[0x2adfe4]` | `?` | `0x2adfe4` |
| `bl` at 0xe7adc | `(unknown callee) 0x23fbc` | `0x1366e4` | `[0x2adfe8]` | `?` | `0x2adfe8` |
| `bl` at 0xe7bd4 | `(unknown callee) 0x23fbc` | `0x1366f4` | `?` | `?` | `?` |
| `bl` at 0xe7cd4 | `(unknown callee) 0x23fbc` | `0x136730` | `?` | `?` | `?` |
| `bl` at 0xe7cfc | `(unknown callee) 0x23fbc` | `0x136764` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xe7a98 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bls` at 0xe7af0 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xe7b24 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0xe7c10 | `C == 0` | `not (C == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe7d00 | `r0` | `ret(0x23fbc)` |

## `JTAG_Cfg_L3SetCfg_f`

- `JTAG_Cfg_L3SetCfg_f`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe7d38 to 0xe8190.
  Established. The unnamed `0xe6cac` call has `[0x2adfe8]`, unknown, unknown, `0x2adfe8` (`bl` at
  0xe7ee0); its nonzero result takes the branch to `0xe7f10` (`bne` at 0xe7eec). Included byte
  stores at unknown targets retain `0x3` and unknown, while a four-byte store at another unknown
  target writes `0x0` (`strb` at 0xe80d0, `strb` at 0xe811c, `str` at 0xe814c). Configuration-field
  meanings and L3 effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe7d48 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xe7d4c | load | 4 | `abs 0xe816c` | `-` |
| `ldr` at 0xe7d50 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xe7d5c | load | 4 | `abs 0xe8170` | `-` |
| `str` at 0xe7d60 | store | 4 | `sp -0x3c` | `0x136768` |
| `ldr` at 0xe7d68 | load | 4 | `abs 0xe8174` | `-` |
| `ldr` at 0xe7d7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7d84 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe7d88 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xe7d90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7d98 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe7d9c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xe7da0 | load | 4 | `abs 0xe816c` | `-` |
| `ldr` at 0xe7da4 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xe7db0 | load | 4 | `abs 0xe8178` | `-` |
| `str` at 0xe7db4 | store | 4 | `sp -0x3c` | `0x13677c` |
| `ldr` at 0xe7db8 | load | 4 | `abs 0xe817c` | `-` |
| `ldr` at 0xe7dbc | load | 4 | `abs 0x2adfe8` | `-` |
| `str` at 0xe7dc0 | store | 4 | `sp -0x38` | `[0x2adfe8]` |
| `ldr` at 0xe7dc4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe7dc8 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe7dcc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe7dd0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe7dd8 | load | 4 | `abs 0xe8174` | `-` |
| `ldr` at 0xe7ddc | load | 4 | `abs 0xe8180` | `-` |
| `ldr` at 0xe7de8 | load | 4 | `abs 0xe816c` | `-` |
| `ldr` at 0xe7dec | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xe7df8 | load | 4 | `abs 0xe8184` | `-` |
| `ldr` at 0xe7dfc | load | 4 | `abs 0xe817c` | `-` |
| `ldr` at 0xe7e00 | load | 4 | `abs 0x2adfe8` | `-` |
| `ldr` at 0xe7e18 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe7e1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7e28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7e34 | load | 4 | `abs 0xe8184` | `-` |
| `ldr` at 0xe7e38 | load | 4 | `abs 0xe817c` | `-` |
| `ldr` at 0xe7e3c | load | 4 | `abs 0x2adfe8` | `-` |
| `ldr` at 0xe7e5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7e60 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe7e6c | load | 4 | `abs 0xe8188` | `-` |
| `str` at 0xe7e70 | store | 4 | `sp -0x3c` | `0x1367ac` |
| `ldr` at 0xe7e74 | load | 4 | `abs 0xe817c` | `-` |
| `ldr` at 0xe7e78 | load | 4 | `abs 0x2adfe8` | `-` |
| `str` at 0xe7e7c | store | 4 | `sp -0x38` | `[0x2adfe8]` |
| `ldr` at 0xe7e80 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe7e84 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe7e88 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe7e8c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe7e90 | load | 4 | `abs 0xe8184` | `-` |
| `ldr` at 0xe7e94 | load | 4 | `abs 0xe817c` | `-` |
| `ldr` at 0xe7e98 | load | 4 | `abs 0x2adfe8` | `-` |
| `ldr` at 0xe7eb8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe7ebc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe7ec4 | load | 4 | `abs 0xe8174` | `-` |
| `ldr` at 0xe7ec8 | load | 4 | `abs 0xe818c` | `-` |
| `ldr` at 0xe7ed4 | load | 4 | `abs 0xe817c` | `-` |
| `ldr` at 0xe7ed8 | load | 4 | `abs 0x2adfe8` | `-` |
| `ldr` at 0xe7edc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7ef0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7ef4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7ef8 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xe7f00 | store | 4 | `[sp-20] +0x0` | `?` |
| `str` at 0xe7f08 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe7f10 | load | 4 | `abs 0xe8184` | `-` |
| `ldr` at 0xe7f14 | load | 4 | `abs 0xe817c` | `-` |
| `ldr` at 0xe7f18 | load | 4 | `abs 0x2adfe8` | `-` |
| `ldr` at 0xe7f30 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe7f34 | store | 4 | `(unknown)` | `[sp-24]` |
| `strb` at 0xe7f3c | store | 1 | `sp -0x22` | `0x0` |
| `ldrb` at 0xe7f40 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xe7f44 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xe7f54 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xe7f60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7f6c | load | 4 | `(unknown)` | `-` |
| `str` at 0xe7f70 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe7f74 | load | 4 | `abs 0xe8184` | `-` |
| `ldr` at 0xe7f78 | load | 4 | `abs 0xe817c` | `-` |
| `ldr` at 0xe7f7c | load | 4 | `abs 0x2adfe8` | `-` |
| `ldrb` at 0xe7f9c | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xe7fa4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe7fac | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xe7fb4 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe7fb8 | load | 4 | `abs 0xe8184` | `-` |
| `ldr` at 0xe7fbc | load | 4 | `abs 0xe817c` | `-` |
| `ldr` at 0xe7fc0 | load | 4 | `abs 0x2adfe8` | `-` |
| `ldrb` at 0xe7fe0 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xe7fe8 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xe7ff0 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0xe7ff4 | store | 1 | `(unknown)` | `([sp-32]:1 & 0xff)` |
| `ldr` at 0xe7ff8 | load | 4 | `abs 0xe8184` | `-` |
| `ldr` at 0xe7ffc | load | 4 | `abs 0xe817c` | `-` |
| `ldr` at 0xe8000 | load | 4 | `abs 0x2adfe8` | `-` |
| `ldrb` at 0xe8020 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xe8028 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xe8030 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xe803c | load | 4 | `abs 0xe8184` | `-` |
| `ldr` at 0xe8040 | load | 4 | `abs 0xe817c` | `-` |
| `ldr` at 0xe8044 | load | 4 | `abs 0x2adfe8` | `-` |
| `ldrb` at 0xe8064 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xe806c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xe8074 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xe8094 | load | 4 | `abs 0xe8184` | `-` |
| `ldr` at 0xe8098 | load | 4 | `abs 0xe817c` | `-` |
| `ldr` at 0xe809c | load | 4 | `abs 0x2adfe8` | `-` |
| `ldrb` at 0xe80bc | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xe80c4 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xe80d0 | store | 1 | `(unknown)` | `0x3` |
| `ldrb` at 0xe80d4 | load | 1 | `sp -0x22` | `-` |
| `strb` at 0xe80dc | store | 1 | `sp -0x22` | `([sp-34]:1+1 & 0xff)` |
| `ldr` at 0xe80e4 | load | 4 | `abs 0xe817c` | `-` |
| `ldr` at 0xe80e8 | load | 4 | `abs 0x2adfe8` | `-` |
| `ldr` at 0xe80f0 | load | 4 | `abs 0xe8184` | `-` |
| `ldr` at 0xe80f4 | load | 4 | `abs 0xe817c` | `-` |
| `ldr` at 0xe80f8 | load | 4 | `abs 0x2adfe8` | `-` |
| `strb` at 0xe811c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe8120 | load | 4 | `abs 0xe8184` | `-` |
| `ldr` at 0xe8124 | load | 4 | `abs 0xe817c` | `-` |
| `ldr` at 0xe8128 | load | 4 | `abs 0x2adfe8` | `-` |
| `str` at 0xe814c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe8150 | load | 4 | `abs 0xe817c` | `-` |
| `ldr` at 0xe8154 | load | 4 | `abs 0x2adfe8` | `-` |
| `str` at 0xe8160 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0xe8164 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xe8168 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe8168 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe8168 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe7d74 | `(unknown callee) 0x21eb8` | `0x6` | `0x1364e0` | `0x1d4` | `0x0` |
| `bl` at 0xe7de4 | `(unknown callee) 0x21eb8` | `0x6` | `0x1364e0` | `0x1dd` | `0x0` |
| `bl` at 0xe7ed0 | `(unknown callee) 0x21eb8` | `0x6` | `0x1364e0` | `0x1e2` | `0x0` |
| `bl` at 0xe7ee0 | `(unknown callee) 0xe6cac` | `[0x2adfe8]` | `?` | `?` | `0x2adfe8` |
| `bl` at 0xe8084 | `(unknown callee) 0xebb6c` | `0x3` | `?` | `?` | `?` |
| `bl` at 0xe80ec | `(unknown callee) 0xe7424` | `[0x2adfe8]` | `?` | `?` | `0x2adfe8` |
| `bl` at 0xe8158 | `(unknown callee) 0xea0e8` | `[0x2adfe8]` | `0x14` | `?` | `0x2adfe8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe7d58 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `beq` at 0xe7dac | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `beq` at 0xe7df4 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `beq` at 0xe7e24 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xe7e30 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xe7e68 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xe7eec | `ret(0xe6cac) != 0x0` | `ret(0xe6cac) == 0x0` |
| `blo` at 0xe7f4c | `C == 0` | `not (C == 0)` |
| `bne` at 0xe8038 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe807c | `? != 0x8` | `? == 0x8` |
| `beq` at 0xe8090 | `ret(0xebb6c) == 0x0` | `ret(0xebb6c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe8168 | `r0` | `?` |

## `JTAG_Cfg_Setup`

- `JTAG_Cfg_Setup`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe8190 to 0xe81e4.
  Established. Three four-byte global assignments write `0xe7d38` at `0x143858`, `0xe75ac` at
  `0x14385c`, and `0xe7a7c` at `0x143860` (`str` at 0xe81a4, `str` at 0xe81b0, `str` at 0xe81bc).
  The exit carries `0x0` (`ldmdb` at 0xe81c8). This span contains no calls or conditional branches.
  The stored pointer roles are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xe819c | load | 4 | `abs 0xe81cc` | `-` |
| `ldr` at 0xe81a0 | load | 4 | `abs 0xe81d0` | `-` |
| `str` at 0xe81a4 | store | 4 | `abs 0x143858` | `0xe7d38` |
| `ldr` at 0xe81a8 | load | 4 | `abs 0xe81d4` | `-` |
| `ldr` at 0xe81ac | load | 4 | `abs 0xe81d8` | `-` |
| `str` at 0xe81b0 | store | 4 | `abs 0x14385c` | `0xe75ac` |
| `ldr` at 0xe81b4 | load | 4 | `abs 0xe81dc` | `-` |
| `ldr` at 0xe81b8 | load | 4 | `abs 0xe81e0` | `-` |
| `str` at 0xe81bc | store | 4 | `abs 0x143860` | `0xe7a7c` |
| `ldmdb` at 0xe81c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe81c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe81c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe81c8 | `r0` | `0x0` |

## `JTAG_LLP_Setup`

- `JTAG_LLP_Setup`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf407c to 0xf40bc.
  Established. Two four-byte assignments write `0xf4bec` at absolute `0x143868` and `0xf46fc` at
  absolute `0x14386c` (`str` at 0xf4090, `str` at 0xf409c). The exit retains `0x0` (`ldmdb` at
  0xf40a8). No calls or conditional branches appear in this span. Link callback semantics remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xf4088 | load | 4 | `abs 0xf40ac` | `-` |
| `ldr` at 0xf408c | load | 4 | `abs 0xf40b0` | `-` |
| `str` at 0xf4090 | store | 4 | `abs 0x143868` | `0xf4bec` |
| `ldr` at 0xf4094 | load | 4 | `abs 0xf40b4` | `-` |
| `ldr` at 0xf4098 | load | 4 | `abs 0xf40b8` | `-` |
| `str` at 0xf409c | store | 4 | `abs 0x14386c` | `0xf46fc` |
| `ldmdb` at 0xf40a8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf40a8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf40a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf40a8 | `r0` | `0x0` |
