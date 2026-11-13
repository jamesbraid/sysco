# Selected ARM Linux 2.6 routines in 1.44.0

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
No unresolved computed transfers were reported for these routines.
No undecodable byte ranges were reported in these spans.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-boot-console-scan-helpers-facts.txt) retain all tool sections.

## `bootGetHeaderPtr`

- `bootGetHeaderPtr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1085dc to 0x1086c8.
  Established. The result from unnamed 0x11053c is stored as a word at entry sp minus 0x24 (`bl` at 0x108614, `str` at 0x10861c).
  A distinct result from unnamed 0x108420 is stored as a word at entry sp minus 0x20 (`bl` at 0x108660, `str` at 0x108664).
  The later word store at entry sp minus 0x2c has an unknown value (`str` at 0x1086b8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1085ec | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x1085f0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x108608 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x108610 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x10861c | store | 4 | `sp -0x24` | `ret(0x11053c)` |
| `ldr` at 0x108620 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x108630 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x108638 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x10863c | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0x10864c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x108664 | store | 4 | `sp -0x20` | `ret(0x108420)` |
| `ldr` at 0x108668 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x108680 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x108688 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x10869c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x1086a4 | store | 4 | `sp -0x30` | `ret(0x11053c)` |
| `str` at 0x1086b0 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x1086b4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x1086b8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x1086bc | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x1086c4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x1086c4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1086c4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1086c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1085f4 | `(unknown callee) 0x108300` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x108614 | `(unknown callee) 0x11053c` | `[sp-24]` | `?` | `?` | `ret(0x108300)` |
| `bl` at 0x108640 | `(unknown callee) 0x20010` | `[[sp-36]]` | `?` | `?` | `[sp-36]` |
| `bl` at 0x108650 | `(unknown callee) 0x110410` | `[sp-24]` | `?` | `?` | `ret(0x20010)` |
| `bl` at 0x108660 | `(unknown callee) 0x108420` | `ret(0x20010)` | `ret(0x110410)` | `?` | `ret(0x110410)` |
| `bl` at 0x10866c | `(unknown callee) 0x108300` | `ret(0x108420)` | `?` | `?` | `?` |
| `bl` at 0x10868c | `(unknown callee) 0x11053c` | `[sp-32]` | `?` | `?` | `ret(0x108300)` |
| `bl` at 0x1086a0 | `(unknown callee) 0x11053c` | `[sp-32]` | `?` | `?` | `ret(0x11053c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x108600 | `ret(0x108300) != 0x0` | `ret(0x108300) == 0x0` |
| `bne` at 0x108628 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x108678 | `ret(0x108300) != 0x0` | `ret(0x108300) == 0x0` |
| `beq` at 0x108698 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1086c4 | `r0` | `?` |

## `bootGetHeader`

- `bootGetHeader`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1086c8 to 0x108878.
  Established. A result from unnamed 0x110410 is stored as a word at entry sp minus 0x28 (`bl` at 0x1086ec, `str` at 0x1086f4).
  The word store consuming the result from unnamed 0x20010 has an unknown effective target (`bl` at 0x108758, `str` at 0x108774).
  A later result from unnamed 0x11053c is stored as a word in memory at [sp-28] plus 0x10 (`bl` at 0x1087a4, `str` at 0x1087a8).
  One halfword store writes the low 16 bits of [[sp-28]]:2 to memory at [sp-28] plus 2 (`strh` at 0x108854).
  Another writes the low 16 bits of [sp-42]:2 to memory at [sp-28] (`strh` at 0x108860).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1086d8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x1086dc | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x1086e4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x1086e8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x1086f4 | store | 4 | `sp -0x28` | `ret(0x110410)` |
| `ldr` at 0x1086f8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x108704 | store | 4 | `sp -0x20` | `ret(0x1085dc)` |
| `ldr` at 0x108708 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x108718 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x108720 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x108724 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x108734 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x108738 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x108744 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x10874c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x108754 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x108764 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x10876c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x108774 | store | 4 | `(unknown)` | `ret(0x20010)` |
| `ldr` at 0x108778 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x108780 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x108788 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x10878c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x108790 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x108794 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x1087a8 | store | 4 | `[sp-28] +0x10` | `ret(0x11053c)` |
| `ldr` at 0x1087ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1087b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1087b4 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldr` at 0x1087b8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x1087cc | store | 4 | `[sp-28] +0x14` | `ret(0x11053c)` |
| `ldr` at 0x1087d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1087d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1087d8 | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0x1087dc | load | 4 | `sp -0x28` | `-` |
| `str` at 0x1087f0 | store | 4 | `[sp-28] +0x18` | `ret(0x11053c)` |
| `ldr` at 0x1087f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1087f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1087fc | load | 4 | `[sp-28] +0x24` | `-` |
| `ldr` at 0x108800 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x108814 | store | 4 | `[sp-28] +0x24` | `ret(0x11053c)` |
| `ldr` at 0x108818 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x10881c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x108820 | load | 4 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0x108824 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x108838 | store | 4 | `[sp-28] +0x2c` | `ret(0x11053c)` |
| `ldr` at 0x10883c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x108840 | load | 2 | `[sp-28] +0x2` | `-` |
| `strh` at 0x108844 | store | 2 | `sp -0x2a` | `([[sp-28]+2]:2 & 0xffff)` |
| `ldr` at 0x108848 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x10884c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x108850 | load | 2 | `[sp-28] +0x0` | `-` |
| `strh` at 0x108854 | store | 2 | `[sp-28] +0x2` | `([[sp-28]]:2 & 0xffff)` |
| `ldr` at 0x108858 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x10885c | load | 2 | `sp -0x2a` | `-` |
| `strh` at 0x108860 | store | 2 | `[sp-28] +0x0` | `([sp-42]:2 & 0xffff)` |
| `ldr` at 0x108864 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x108868 | store | 4 | `sp -0x30` | `[sp-28]` |
| `ldr` at 0x10886c | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x108874 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x108874 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x108874 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x108874 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1086ec | `(unknown callee) 0x110410` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x1086fc | `(unknown callee) 0x1085dc` | `[sp-24]` | `?` | `?` | `ret(0x110410)` |
| `bl` at 0x10872c | `(unknown callee) 0x203f4` | `[sp-28]` | `ret(0x1085dc)` | `0x30` | `ret(0x1085dc)` |
| `bl` at 0x108758 | `(unknown callee) 0x20010` | `?` | `?` | `?` | `?` |
| `bl` at 0x108798 | `(unknown callee) 0x108420` | `[[sp-28]+16]` | `[sp-40]` | `?` | `[sp-28]` |
| `bl` at 0x1087a4 | `(unknown callee) 0x11053c` | `ret(0x108420)` | `?` | `?` | `ret(0x108420)` |
| `bl` at 0x1087bc | `(unknown callee) 0x108420` | `[[sp-28]+20]` | `[sp-40]` | `?` | `[sp-28]` |
| `bl` at 0x1087c8 | `(unknown callee) 0x11053c` | `ret(0x108420)` | `?` | `?` | `ret(0x108420)` |
| `bl` at 0x1087e0 | `(unknown callee) 0x108420` | `[[sp-28]+24]` | `[sp-40]` | `?` | `[sp-28]` |
| `bl` at 0x1087ec | `(unknown callee) 0x11053c` | `ret(0x108420)` | `?` | `?` | `ret(0x108420)` |
| `bl` at 0x108804 | `(unknown callee) 0x108420` | `[[sp-28]+36]` | `[sp-40]` | `?` | `[sp-28]` |
| `bl` at 0x108810 | `(unknown callee) 0x11053c` | `ret(0x108420)` | `?` | `?` | `ret(0x108420)` |
| `bl` at 0x108828 | `(unknown callee) 0x108420` | `[[sp-28]+44]` | `[sp-40]` | `?` | `[sp-28]` |
| `bl` at 0x108834 | `(unknown callee) 0x11053c` | `ret(0x108420)` | `?` | `?` | `ret(0x108420)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x108710 | `ret(0x1085dc) != 0x0` | `ret(0x1085dc) == 0x0` |
| `bhi` at 0x108740 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x108874 | `r0` | `?` |

## `bootDefaultImage`

- `bootDefaultImage`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x108878 to 0x108aa8.
  Established. A word store writes 0 to memory at entry r0 (`str` at 0x1088b8).
  The word loaded from absolute address 0x31e434 is stored at entry sp minus 0x24 (`ldr` at 0x1088c0, `str` at 0x1088c4).
  A call to unnamed 0x108488 has printed r0-r3 candidates [sp-96], entry sp minus 88, [0x31e434] and entry sp minus 88 (`bl` at 0x108964).
  The later computed word store retains an unknown effective address and value (`str` at 0x108a8c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x108888 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x108890 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x108898 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x1088a0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x1088a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1088b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1088b8 | store | 4 | `r0 +0x0` | `0x0` |
| `ldr` at 0x1088bc | load | 4 | `abs 0x108aa4` | `-` |
| `ldr` at 0x1088c0 | load | 4 | `abs 0x31e434` | `-` |
| `str` at 0x1088c4 | store | 4 | `sp -0x24` | `[0x31e434]` |
| `ldr` at 0x1088c8 | load | 4 | `abs 0x108aa4` | `-` |
| `ldr` at 0x1088cc | load | 4 | `abs 0x31e41c` | `-` |
| `ldr` at 0x1088d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1088dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1088e0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1088ec | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1088f4 | store | 4 | `sp -0x24` | `[sp-36]+4` |
| `ldr` at 0x1088fc | load | 4 | `abs 0x108aa4` | `-` |
| `ldr` at 0x108900 | load | 4 | `abs 0x31e41c` | `-` |
| `ldr` at 0x108904 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x108914 | store | 4 | `sp -0x6c` | `0x0` |
| `ldr` at 0x10891c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x108924 | store | 4 | `sp -0x24` | `?` |
| `str` at 0x108928 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x10892c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x108930 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x108934 | load | 4 | `abs 0x108aa4` | `-` |
| `ldr` at 0x108938 | load | 4 | `abs 0x31e434` | `-` |
| `ldr` at 0x10893c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x108948 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x10894c | load | 4 | `(unknown)` | `-` |
| `str` at 0x108954 | store | 4 | `sp -0x60` | `?` |
| `ldr` at 0x10895c | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x108978 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x10897c | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0x10898c | load | 4 | `sp -0x60` | `-` |
| `str` at 0x108990 | store | 4 | `sp -0x18` | `[sp-96]` |
| `ldr` at 0x108994 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x108998 | load | 4 | `[sp-36] +0x0` | `-` |
| `str` at 0x1089a0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x1089a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1089b4 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x1089b8 | store | 4 | `sp -0x64` | `[sp-72]` |
| `ldr` at 0x1089bc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x1089c0 | store | 4 | `sp -0x68` | `[sp-32]` |
| `ldr` at 0x1089c4 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x1089d0 | load | 4 | `sp -0x64` | `-` |
| `ldrb` at 0x1089d4 | load | 1 | `[sp-100] +0x0` | `-` |
| `ldr` at 0x1089e0 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x1089e4 | load | 1 | `[sp-104] +0x0` | `-` |
| `ldr` at 0x1089f0 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x1089f4 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x1089f8 | load | 1 | `[sp-100] +0x0` | `-` |
| `ldrb` at 0x1089fc | load | 1 | `[sp-104] +0x0` | `-` |
| `ldr` at 0x108a0c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x108a10 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x108a14 | load | 1 | `[sp-100] +0x0` | `-` |
| `ldrb` at 0x108a18 | load | 1 | `[sp-104] +0x0` | `-` |
| `str` at 0x108a28 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x108a30 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x108a38 | store | 4 | `sp -0x64` | `[sp-100]+1` |
| `ldr` at 0x108a3c | load | 4 | `sp -0x68` | `-` |
| `str` at 0x108a44 | store | 4 | `sp -0x68` | `[sp-104]+1` |
| `ldr` at 0x108a4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x108a58 | load | 4 | `sp -0x60` | `-` |
| `str` at 0x108a5c | store | 4 | `sp -0x18` | `[sp-96]` |
| `ldr` at 0x108a60 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x108a64 | store | 4 | `sp -0x20` | `[sp-72]` |
| `ldr` at 0x108a68 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x108a70 | store | 4 | `sp -0x24` | `[sp-36]-4` |
| `ldr` at 0x108a78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x108a84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x108a88 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x108a8c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x108a90 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x108a94 | store | 4 | `sp -0x6c` | `?` |
| `ldr` at 0x108a98 | load | 4 | `sp -0x6c` | `-` |
| `ldm` at 0x108aa0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x108aa0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x108aa0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x108964 | `(unknown callee) 0x108488` | `[sp-96]` | `sp-88` | `[0x31e434]` | `sp-88` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x1088ac | `r0 == 0x0` | `r0 != 0x0` |
| `bls` at 0x1088d8 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x1088e8 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x10890c | `Z == 0` | `not (Z == 0)` |
| `bhi` at 0x108944 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0x108970 | `ret(0x108488) == 0x0` | `ret(0x108488) != 0x0` |
| `beq` at 0x108988 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x1089b0 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0x1089cc | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x1089dc | `[[sp-100]]:1 == 0x0` | `[[sp-100]]:1 != 0x0` |
| `beq` at 0x1089ec | `[[sp-104]]:1 == 0x0` | `[[sp-104]]:1 != 0x0` |
| `bhs` at 0x108a04 | `[[sp-100]]:1 >= [[sp-104]]:1 (unsigned, 32-bit)` | `[[sp-100]]:1 < [[sp-104]]:1 (unsigned, 32-bit)` |
| `bls` at 0x108a20 | `[[sp-100]]:1 <= [[sp-104]]:1 (unsigned, 32-bit)` | `[[sp-100]]:1 > [[sp-104]]:1 (unsigned, 32-bit)` |
| `bne` at 0x108a54 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x108a80 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x108aa0 | `r0` | `?` |

## `charioInit`

- `charioInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x23fe0 to 0x2405c.
  Established. Entry r0 is stored as a word at entry sp minus 0x14 (`str` at 0x23ff0).
  A call to unnamed 0x22b84 has printed r0-r3 candidates 0x1a2f64, 0x153238, 1 and 9 (`bl` at 0x24030).
  A later word store writes [sp-20] to absolute address 0x1a2fb8 (`str` at 0x2403c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x23ff0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x23ff4 | load | 4 | `abs 0x24048` | `-` |
| `ldr` at 0x23ff8 | load | 4 | `abs 0x1a2fb8` | `-` |
| `ldr` at 0x24004 | load | 4 | `abs 0x2404c` | `-` |
| `str` at 0x24008 | store | 4 | `sp -0x1c` | `0x1531f8` |
| `ldr` at 0x24010 | load | 4 | `abs 0x24050` | `-` |
| `ldr` at 0x24020 | load | 4 | `abs 0x24054` | `-` |
| `ldr` at 0x24024 | load | 4 | `abs 0x24058` | `-` |
| `ldr` at 0x24034 | load | 4 | `abs 0x24048` | `-` |
| `ldr` at 0x24038 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2403c | store | 4 | `abs 0x1a2fb8` | `[sp-20]` |
| `ldm` at 0x24044 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x24044 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x24044 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2401c | `(unknown callee) 0x22070` | `0x9` | `0x15321c` | `0x73` | `0x0` |
| `bl` at 0x24030 | `(unknown callee) 0x22b84` | `0x1a2f64` | `0x153238` | `0x1` | `0x9` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x24000 | `[0x1a2fb8] == 0x0` | `[0x1a2fb8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x24044 | `r0` | `ret(0x22b84)` |

## `charioOpen`

- `charioOpen`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2405c to 0x24214.
  Established. Distinct results from unnamed 0x201fc and 0x10edb4 are stored as words at absolute address 0x1a2fbc on separate paths (`bl` at 0x24114, `str` at 0x2411c, `bl` at 0x24150, `str` at 0x2415c).
  Two word stores write 1 to absolute address 0x1a2fc0 (`str` at 0x2413c, `str` at 0x24174).
  The computed word store at entry sp minus 0x1c has an unknown value (`str` at 0x240f8).
  The recorded return value in r0 is [0x1a2fbc] (`ldm` at 0x241f0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2406c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x24074 | store | 4 | `sp -0x1c` | `0x1` |
| `str` at 0x2407c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x24080 | load | 4 | `abs 0x241f4` | `-` |
| `ldr` at 0x24098 | load | 4 | `abs 0x241f8` | `-` |
| `ldr` at 0x2409c | load | 4 | `abs 0x1a2d00` | `-` |
| `str` at 0x240ac | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x240b0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x240b4 | store | 4 | `sp -0x20` | `[sp-36]` |
| `ldr` at 0x240c0 | load | 4 | `abs 0x241fc` | `-` |
| `str` at 0x240c4 | store | 4 | `sp -0x30` | `0x153244` |
| `ldr` at 0x240c8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x240cc | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0x240d4 | load | 4 | `abs 0x24200` | `-` |
| `ldr` at 0x240e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x240f0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x240f8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x240fc | load | 4 | `abs 0x24204` | `-` |
| `ldr` at 0x24100 | load | 4 | `abs 0x1a2fb8` | `-` |
| `ldr` at 0x2410c | load | 4 | `abs 0x24208` | `-` |
| `str` at 0x2411c | store | 4 | `abs 0x1a2fbc` | `ret(0x201fc)` |
| `ldr` at 0x24120 | load | 4 | `abs 0x24208` | `-` |
| `ldr` at 0x24124 | load | 4 | `abs 0x1a2fbc` | `-` |
| `ldr` at 0x24134 | load | 4 | `abs 0x2420c` | `-` |
| `str` at 0x2413c | store | 4 | `abs 0x1a2fc0` | `0x1` |
| `ldr` at 0x24144 | load | 4 | `abs 0x24204` | `-` |
| `ldr` at 0x24148 | load | 4 | `abs 0x1a2fb8` | `-` |
| `ldr` at 0x2414c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x24158 | load | 4 | `abs 0x24208` | `-` |
| `str` at 0x2415c | store | 4 | `abs 0x1a2fbc` | `ret(0x10edb4)` |
| `ldr` at 0x24160 | load | 4 | `abs 0x1a2fbc` | `-` |
| `ldr` at 0x2416c | load | 4 | `abs 0x2420c` | `-` |
| `str` at 0x24174 | store | 4 | `abs 0x1a2fc0` | `0x1` |
| `str` at 0x2417c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x24180 | load | 4 | `abs 0x241f4` | `-` |
| `ldr` at 0x24194 | load | 4 | `abs 0x241f8` | `-` |
| `ldr` at 0x24198 | load | 4 | `abs 0x1a2d00` | `-` |
| `str` at 0x241a8 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x241ac | load | 4 | `sp -0x28` | `-` |
| `str` at 0x241b0 | store | 4 | `sp -0x20` | `[sp-40]` |
| `ldr` at 0x241bc | load | 4 | `abs 0x24210` | `-` |
| `str` at 0x241c0 | store | 4 | `sp -0x30` | `0x153268` |
| `ldr` at 0x241c4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x241c8 | store | 4 | `sp -0x2c` | `[sp-40]` |
| `ldr` at 0x241d0 | load | 4 | `abs 0x24200` | `-` |
| `ldr` at 0x241e0 | load | 4 | `abs 0x24208` | `-` |
| `ldr` at 0x241e4 | load | 4 | `abs 0x1a2fbc` | `-` |
| `ldm` at 0x241f0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x241f0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x241f0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x241f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x24088 | `(unknown callee) 0x22ca0` | `0x1a2f64` | `?` | `r2` | `0x0` |
| `bl` at 0x240e0 | `(unknown callee) 0x22070` | `0x9` | `0x15321c` | `0x93` | `0x0` |
| `bl` at 0x24114 | `(unknown callee) 0x201fc` | `0x108` | `?` | `?` | `[0x1a2fb8]` |
| `bl` at 0x24130 | `(unknown callee) 0x20550` | `[0x1a2fbc]` | `0x0` | `0x108` | `0x1a2fbc` |
| `bl` at 0x24150 | `(unknown callee) 0x10edb4` | `[0x1a2fb8]` | `[sp-28]` | `?` | `0x1a2fb8` |
| `bl` at 0x24184 | `(unknown callee) 0x2302c` | `0x1a2f64` | `?` | `?` | `0x0` |
| `bl` at 0x241dc | `(unknown callee) 0x22070` | `0x9` | `0x15321c` | `0xab` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x24094 | `ret(0x22ca0) == 0x0` | `ret(0x22ca0) != 0x0` |
| `beq` at 0x240a4 | `[0x1a2d00] == 0x1` | `[0x1a2d00] != 0x1` |
| `beq` at 0x240bc | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0x240ec | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `bne` at 0x24108 | `[0x1a2fb8] != 0x0` | `[0x1a2fb8] == 0x0` |
| `beq` at 0x24168 | `[0x1a2fbc] == 0x0` | `[0x1a2fbc] != 0x0` |
| `beq` at 0x24190 | `ret(0x2302c) == 0x0` | `ret(0x2302c) != 0x0` |
| `beq` at 0x241a0 | `[0x1a2d00] == 0x1` | `[0x1a2d00] != 0x1` |
| `beq` at 0x241b8 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x241f0 | `r0` | `[0x1a2fbc]` |

## `charioPort`

- `charioPort`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24214 to 0x24234.
  Established. Included implicit word saves place entry r11, sp and lr at entry sp minus 0x10, 0xc and 8 respectively (`push` at 0x24218).
  A word load reads absolute address 0x1a2fb8 (`ldr` at 0x24224).
  The return record places that loaded value in r0 (`ldr` at 0x24224, `ldm` at 0x2422c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x24220 | load | 4 | `abs 0x24230` | `-` |
| `ldr` at 0x24224 | load | 4 | `abs 0x1a2fb8` | `-` |
| `ldm` at 0x2422c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x2422c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x2422c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x2422c | `r0` | `[0x1a2fb8]` |

## `charioClose`

- `charioClose`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24234 to 0x24368.
  Established. The result from unnamed 0x22ca0 is stored as a word at entry sp minus 0x18 (`bl` at 0x24254, `str` at 0x2425c).
  Word stores write 0 to absolute addresses 0x1a2fbc and 0x1a2fc0 (`str` at 0x242b4, `str` at 0x242c0).
  A result from unnamed 0x10f2bc is stored as a word at entry sp minus 0x14 (`bl` at 0x242e4, `str` at 0x242ec).
  The recorded return value is [sp-20] (`ldm` at 0x24348).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x24248 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x2424c | load | 4 | `abs 0x2434c` | `-` |
| `str` at 0x2425c | store | 4 | `sp -0x18` | `ret(0x22ca0)` |
| `ldr` at 0x24260 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2426c | load | 4 | `abs 0x24350` | `-` |
| `str` at 0x24270 | store | 4 | `sp -0x20` | `0x153244` |
| `ldr` at 0x24274 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x24278 | store | 4 | `sp -0x1c` | `ret(0x22ca0)` |
| `ldr` at 0x24280 | load | 4 | `abs 0x24354` | `-` |
| `ldr` at 0x24290 | load | 4 | `abs 0x24358` | `-` |
| `ldr` at 0x24294 | load | 4 | `abs 0x1a2fb8` | `-` |
| `ldr` at 0x242a0 | load | 4 | `abs 0x2435c` | `-` |
| `ldr` at 0x242a4 | load | 4 | `abs 0x1a2fbc` | `-` |
| `ldr` at 0x242ac | load | 4 | `abs 0x2435c` | `-` |
| `str` at 0x242b4 | store | 4 | `abs 0x1a2fbc` | `0x0` |
| `ldr` at 0x242b8 | load | 4 | `abs 0x24360` | `-` |
| `str` at 0x242c0 | store | 4 | `abs 0x1a2fc0` | `0x0` |
| `ldr` at 0x242c8 | load | 4 | `abs 0x24360` | `-` |
| `ldr` at 0x242cc | load | 4 | `abs 0x1a2fc0` | `-` |
| `ldr` at 0x242d8 | load | 4 | `abs 0x2435c` | `-` |
| `ldr` at 0x242dc | load | 4 | `abs 0x1a2fbc` | `-` |
| `str` at 0x242ec | store | 4 | `sp -0x14` | `ret(0x10f2bc)` |
| `ldr` at 0x242f0 | load | 4 | `abs 0x24360` | `-` |
| `str` at 0x242f8 | store | 4 | `abs 0x1a2fc0` | `0x0` |
| `ldr` at 0x242fc | load | 4 | `abs 0x2434c` | `-` |
| `str` at 0x24308 | store | 4 | `sp -0x18` | `ret(0x2302c)` |
| `ldr` at 0x2430c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x24318 | load | 4 | `abs 0x24364` | `-` |
| `str` at 0x2431c | store | 4 | `sp -0x20` | `0x153268` |
| `ldr` at 0x24320 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x24324 | store | 4 | `sp -0x1c` | `ret(0x2302c)` |
| `ldr` at 0x2432c | load | 4 | `abs 0x24354` | `-` |
| `ldr` at 0x2433c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x24348 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x24348 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x24348 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x24254 | `(unknown callee) 0x22ca0` | `0x1a2f64` | `?` | `r2` | `0x0` |
| `bl` at 0x2428c | `(unknown callee) 0x22070` | `0x9` | `0x15321c` | `0xce` | `0x0` |
| `bl` at 0x242a8 | `(unknown callee) 0x20514` | `[0x1a2fbc]` | `?` | `?` | `0x1a2fbc` |
| `bl` at 0x242e4 | `(unknown callee) 0x10f2bc` | `[0x1a2fbc]` | `0x1` | `?` | `0x1a2fbc` |
| `bl` at 0x24300 | `(unknown callee) 0x2302c` | `0x1a2f64` | `?` | `?` | `?` |
| `bl` at 0x24338 | `(unknown callee) 0x22070` | `0x9` | `0x15321c` | `0xe5` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x24268 | `ret(0x22ca0) == 0x0` | `ret(0x22ca0) != 0x0` |
| `bne` at 0x2429c | `[0x1a2fb8] != 0x0` | `[0x1a2fb8] == 0x0` |
| `beq` at 0x242d4 | `[0x1a2fc0] == 0x0` | `[0x1a2fc0] != 0x0` |
| `beq` at 0x24314 | `ret(0x2302c) == 0x0` | `ret(0x2302c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x24348 | `r0` | `[sp-20]` |

## `charioLock`

- `charioLock`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24368 to 0x243b8.
  Established. Entry r0 is stored as a word at entry sp minus 0x14 (`str` at 0x24378).
  The call to unnamed 0x24714 has printed r0-r3 candidates entry r0, entry r1, entry r2 and entry r3 (`bl` at 0x24380).
  Separate paths store 0 or the word loaded from absolute address 0x1a2fbc at entry sp minus 0x18 (`str` at 0x24394, `ldr` at 0x243a0, `str` at 0x243a4).
  The return record reads [sp-24] into r0 (`ldm` at 0x243b0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x24378 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2437c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x24394 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x2439c | load | 4 | `abs 0x243b4` | `-` |
| `ldr` at 0x243a0 | load | 4 | `abs 0x1a2fbc` | `-` |
| `str` at 0x243a4 | store | 4 | `sp -0x18` | `[0x1a2fbc]` |
| `ldr` at 0x243a8 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x243b0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x243b0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x243b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x24380 | `(unknown callee) 0x24714` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x2438c | `ret(0x24714) == 0x0` | `ret(0x24714) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x243b0 | `r0` | `[sp-24]` |

## `charioUnlock`

- `charioUnlock`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x243b8 to 0x243d4.
  Established. Included implicit word saves place entry r11, sp and lr at entry sp minus 0x10, 0xc and 8 respectively (`push` at 0x243bc).
  The call to unnamed 0x24838 has printed r0-r3 candidates entry r0, entry r1, entry r2 and entry r3 (`bl` at 0x243c4).
  The final ldm reads words at entry sp minus 0x10, 0xc and 8 (`ldm` at 0x243d0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldm` at 0x243d0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x243d0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x243d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x243c4 | `(unknown callee) 0x24838` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x243d0 | `r0` | `ret(0x24838)` |

## `chario_redirect`

- `chario_redirect`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2498c to 0x249d4.
  Established. Entry r0 is stored as a word at entry sp minus 0x14 (`str` at 0x2499c).
  The branch to 0x249bc is taken when entry r0 is 0 and falls through when it is nonzero (`beq` at 0x249a8).
  Separate paths write entry r0 or 0x10f690 as a word to absolute address 0x19e410 (`str` at 0x249b4, `str` at 0x249c4).
  The return record preserves entry r0 (`ldm` at 0x249c8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2499c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x249a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x249ac | load | 4 | `abs 0x249cc` | `-` |
| `ldr` at 0x249b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x249b4 | store | 4 | `abs 0x19e410` | `r0` |
| `ldr` at 0x249bc | load | 4 | `abs 0x249cc` | `-` |
| `ldr` at 0x249c0 | load | 4 | `abs 0x249d0` | `-` |
| `str` at 0x249c4 | store | 4 | `abs 0x19e410` | `0x10f690` |
| `ldm` at 0x249c8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x249c8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x249c8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x249c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x249a8 | `r0 == 0x0` | `r0 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x249c8 | `r0` | `r0` |

## `sn1sc_select_port`

- `sn1sc_select_port`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x134374 to 0x134634.
  Established. A halfword store preserves the low 16 bits of entry r0 at entry sp minus 0x12 (`strh` at 0x13438c).
  A byte store preserves the low eight bits of entry r1 at entry sp minus 0x13 (`strb` at 0x134394).
  A byte store writes the low eight bits of [sp-23]:1 to memory at [sp-28] plus 0x31e4e0 (`strb` at 0x1344f0).
  A halfword store writes 0 to memory at [0x31e56c] plus 0xe (`strh` at 0x134518).
  Later halfword stores to memory at [0x31e4dc] and [0x31e56c] plus 0xe have unknown values (`strh` at 0x1345ec, `strh` at 0x134614).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strh` at 0x13438c | store | 2 | `sp -0x12` | `(r0 & 0xffff)` |
| `strb` at 0x134394 | store | 1 | `sp -0x13` | `(r1 & 0xff)` |
| `ldr` at 0x13439c | load | 4 | `abs 0x134628` | `-` |
| `ldr` at 0x1343a0 | load | 4 | `abs 0x31e56c` | `-` |
| `ldrh` at 0x1343a8 | load | 2 | `[0x31e56c] +0xe` | `-` |
| `strh` at 0x1343b4 | store | 2 | `sp -0x16` | `?` |
| `ldr` at 0x1343b8 | load | 4 | `abs 0x13462c` | `-` |
| `ldr` at 0x1343bc | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x1343c0 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `ldr` at 0x1343d4 | load | 4 | `abs 0x13462c` | `-` |
| `ldr` at 0x1343d8 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x1343dc | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `ldr` at 0x134400 | load | 4 | `abs 0x13462c` | `-` |
| `ldr` at 0x134404 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x134408 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `ldr` at 0x13442c | load | 4 | `abs 0x13462c` | `-` |
| `ldr` at 0x134430 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x134434 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `ldr` at 0x134458 | load | 4 | `abs 0x13462c` | `-` |
| `ldr` at 0x13445c | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x134460 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `ldr` at 0x134484 | load | 4 | `abs 0x13462c` | `-` |
| `ldr` at 0x134488 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x13448c | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `strb` at 0x1344b0 | store | 1 | `sp -0x17` | `?` |
| `str` at 0x1344b8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x1344bc | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x1344c8 | load | 2 | `sp -0x16` | `-` |
| `ldr` at 0x1344cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1344e0 | load | 4 | `abs 0x134630` | `-` |
| `ldr` at 0x1344e4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x1344ec | load | 1 | `sp -0x17` | `-` |
| `strb` at 0x1344f0 | store | 1 | `[sp-28] +0x31e4e0` | `([sp-23]:1 & 0xff)` |
| `ldr` at 0x1344f4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x1344fc | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x134508 | load | 4 | `abs 0x134628` | `-` |
| `ldr` at 0x13450c | load | 4 | `abs 0x31e56c` | `-` |
| `strh` at 0x134518 | store | 2 | `[0x31e56c] +0xe` | `0x0` |
| `ldr` at 0x13451c | load | 4 | `abs 0x13462c` | `-` |
| `ldr` at 0x134520 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldr` at 0x134524 | load | 4 | `abs 0x134630` | `-` |
| `ldrb` at 0x134528 | load | 1 | `sp -0x13` | `-` |
| `ldrb` at 0x13452c | load | 1 | `[sp-19]:1 +0x31e4e0` | `-` |
| `ldr` at 0x134538 | load | 4 | `abs 0x134630` | `-` |
| `ldrb` at 0x13453c | load | 1 | `sp -0x13` | `-` |
| `ldrb` at 0x134540 | load | 1 | `[sp-19]:1 +0x31e4e0` | `-` |
| `ldr` at 0x13455c | load | 4 | `abs 0x134630` | `-` |
| `ldrb` at 0x134560 | load | 1 | `sp -0x13` | `-` |
| `ldrb` at 0x134564 | load | 1 | `[sp-19]:1 +0x31e4e0` | `-` |
| `ldr` at 0x134580 | load | 4 | `abs 0x134630` | `-` |
| `ldrb` at 0x134584 | load | 1 | `sp -0x13` | `-` |
| `ldrb` at 0x134588 | load | 1 | `[sp-19]:1 +0x31e4e0` | `-` |
| `ldr` at 0x1345a4 | load | 4 | `abs 0x134630` | `-` |
| `ldrb` at 0x1345a8 | load | 1 | `sp -0x13` | `-` |
| `ldrb` at 0x1345ac | load | 1 | `[sp-19]:1 +0x31e4e0` | `-` |
| `ldr` at 0x1345c8 | load | 4 | `abs 0x134630` | `-` |
| `ldrb` at 0x1345cc | load | 1 | `sp -0x13` | `-` |
| `ldrb` at 0x1345d0 | load | 1 | `[sp-19]:1 +0x31e4e0` | `-` |
| `strh` at 0x1345ec | store | 2 | `[0x31e4dc] +0x0` | `?` |
| `ldr` at 0x1345f4 | load | 4 | `abs 0x134628` | `-` |
| `ldr` at 0x1345f8 | load | 4 | `abs 0x31e56c` | `-` |
| `ldrb` at 0x134600 | load | 1 | `sp -0x13` | `-` |
| `ldrh` at 0x13460c | load | 2 | `sp -0x12` | `-` |
| `strh` at 0x134614 | store | 2 | `[0x31e56c] +0xe` | `?` |
| `ldm` at 0x134624 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x134624 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x134624 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0x1344c4 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x1344dc | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x134624 | `r0` | `0x1` |

## `JTAG_Lock`

- `JTAG_Lock`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x134634 to 0x1346fc.
  Established. A word store updates absolute address 0x315044 with its loaded value plus 1 (`ldr` at 0x134664, `str` at 0x13466c).
  The result from unnamed 0x22ca0 is stored as a word at entry sp minus 0x14 (`bl` at 0x13467c, `str` at 0x134684).
  The later result from unnamed 0x228ec is stored as a word at absolute address 0x315040 (`bl` at 0x1346c0, `str` at 0x1346cc).
  A further word store writes 1 to absolute address 0x315044 (`str` at 0x1346d8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x13464c | load | 4 | `abs 0x1346e4` | `-` |
| `ldr` at 0x134650 | load | 4 | `abs 0x315040` | `-` |
| `ldr` at 0x13465c | load | 4 | `abs 0x1346e8` | `-` |
| `ldr` at 0x134660 | load | 4 | `abs 0x1346e8` | `-` |
| `ldr` at 0x134664 | load | 4 | `abs 0x315044` | `-` |
| `str` at 0x13466c | store | 4 | `abs 0x315044` | `[0x315044]+1` |
| `ldr` at 0x134674 | load | 4 | `abs 0x1346ec` | `-` |
| `str` at 0x134684 | store | 4 | `sp -0x14` | `ret(0x22ca0)` |
| `ldr` at 0x134688 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x134694 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1346a0 | load | 4 | `abs 0x1346f0` | `-` |
| `str` at 0x1346a4 | store | 4 | `sp -0x1c` | `0x190cf4` |
| `str` at 0x1346a8 | store | 4 | `sp -0x18` | `ret(0x219a4)` |
| `ldr` at 0x1346b0 | load | 4 | `abs 0x1346f4` | `-` |
| `ldr` at 0x1346b4 | load | 4 | `abs 0x1346f8` | `-` |
| `ldr` at 0x1346c8 | load | 4 | `abs 0x1346e4` | `-` |
| `str` at 0x1346cc | store | 4 | `abs 0x315040` | `ret(0x228ec)` |
| `ldr` at 0x1346d0 | load | 4 | `abs 0x1346e8` | `-` |
| `str` at 0x1346d8 | store | 4 | `abs 0x315044` | `0x1` |
| `ldm` at 0x1346e0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1346e0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1346e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x134644 | `(unknown callee) 0x228ec` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x13467c | `(unknown callee) 0x22ca0` | `0x31e570` | `?` | `[0x315040]` | `ret(0x228ec)` |
| `bl` at 0x134698 | `(unknown callee) 0x219a4` | `ret(0x22ca0)` | `?` | `?` | `ret(0x22ca0)` |
| `bl` at 0x1346bc | `(unknown callee) 0x22070` | `0x9` | `0x190c44` | `0x1a1` | `0x0` |
| `bl` at 0x1346c0 | `(unknown callee) 0x228ec` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x134658 | `ret(0x228ec) != [0x315040]` | `ret(0x228ec) == [0x315040]` |
| `beq` at 0x134690 | `ret(0x22ca0) == 0x0` | `ret(0x22ca0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1346e0 | `r0` | `ret(0x228ec)` |

## `JTAG_Unlock`

- `JTAG_Unlock`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1346fc to 0x1347dc.
  Established. A word store updates absolute address 0x315044 with its loaded value minus 1 (`ldr` at 0x134748, `str` at 0x134750).
  A later word store writes 0 to absolute address 0x315040 (`str` at 0x134768).
  The result from unnamed 0x2302c is stored as a word at entry sp minus 0x14 (`bl` at 0x134770, `str` at 0x134778).
  A call to unnamed 0x22070 has printed r0-r3 candidates 9, 0x190c44, 0x1bf and 0 (`bl` at 0x1347b0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x134714 | load | 4 | `abs 0x1347bc` | `-` |
| `ldr` at 0x134718 | load | 4 | `abs 0x315040` | `-` |
| `ldr` at 0x134724 | load | 4 | `abs 0x1347c0` | `-` |
| `str` at 0x134728 | store | 4 | `sp -0x1c` | `0x190d14` |
| `ldr` at 0x134730 | load | 4 | `abs 0x1347c4` | `-` |
| `ldr` at 0x134734 | load | 4 | `abs 0x1347c8` | `-` |
| `ldr` at 0x134740 | load | 4 | `abs 0x1347cc` | `-` |
| `ldr` at 0x134744 | load | 4 | `abs 0x1347cc` | `-` |
| `ldr` at 0x134748 | load | 4 | `abs 0x315044` | `-` |
| `str` at 0x134750 | store | 4 | `abs 0x315044` | `[0x315044]-1` |
| `ldr` at 0x134760 | load | 4 | `abs 0x1347bc` | `-` |
| `str` at 0x134768 | store | 4 | `abs 0x315040` | `0x0` |
| `ldr` at 0x13476c | load | 4 | `abs 0x1347d0` | `-` |
| `str` at 0x134778 | store | 4 | `sp -0x14` | `ret(0x2302c)` |
| `ldr` at 0x13477c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x134788 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x134794 | load | 4 | `abs 0x1347d4` | `-` |
| `str` at 0x134798 | store | 4 | `sp -0x1c` | `0x190d4c` |
| `str` at 0x13479c | store | 4 | `sp -0x18` | `ret(0x219a4)` |
| `ldr` at 0x1347a4 | load | 4 | `abs 0x1347c4` | `-` |
| `ldr` at 0x1347a8 | load | 4 | `abs 0x1347d8` | `-` |
| `ldm` at 0x1347b8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1347b8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1347b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x13470c | `(unknown callee) 0x228ec` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x13473c | `(unknown callee) 0x22070` | `0x9` | `0x190c44` | `0x1b2` | `0x0` |
| `bl` at 0x134770 | `(unknown callee) 0x2302c` | `0x31e570` | `?` | `0x315040` | `0x0` |
| `bl` at 0x13478c | `(unknown callee) 0x219a4` | `ret(0x2302c)` | `?` | `?` | `ret(0x2302c)` |
| `bl` at 0x1347b0 | `(unknown callee) 0x22070` | `0x9` | `0x190c44` | `0x1bf` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x134720 | `ret(0x228ec) == [0x315040]` | `ret(0x228ec) != [0x315040]` |
| `ble` at 0x134758 | `[0x315044]-1 <= 0x0 (signed, 32-bit)` | `[0x315044]-1 > 0x0 (signed, 32-bit)` |
| `beq` at 0x134784 | `ret(0x2302c) == 0x0` | `ret(0x2302c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1347b8 | `r0` | `?` |

## `sn1sc_set_tap_val`

- `sn1sc_set_tap_val`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x137458 to 0x1374f0.
  Established. Entry r0 is stored as a word at entry sp minus 0x14 (`str` at 0x137468).
  A word load reads absolute address 0x31e4dc (`ldr` at 0x137470).
  The halfword store to memory at [0x31e4dc] has an unknown value (`strh` at 0x1374e4).
  The return record preserves entry r0 (`ldm` at 0x1374e8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x137468 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x13746c | load | 4 | `abs 0x1374ec` | `-` |
| `ldr` at 0x137470 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldr` at 0x137474 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x137480 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x137494 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1374a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1374bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1374d0 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0x1374e4 | store | 2 | `[0x31e4dc] +0x0` | `?` |
| `ldm` at 0x1374e8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x1374e8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1374e8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1374e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1374e8 | `r0` | `r0` |

## `sn1sc_get_tap_val`

- `sn1sc_get_tap_val`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x137100 to 0x137224.
  Established. Six separate halfword loads read memory at [0x31e4dc] (`ldrh` at 0x137118, `ldrh` at 0x13713c, `ldrh` at 0x137168, `ldrh` at 0x137194, `ldrh` at 0x1371c0, `ldrh` at 0x1371ec).
  The computed word stored at entry sp minus 0x14 has an unknown value (`str` at 0x137210).
  A subsequent word load reads that stack location, and the return record places [sp-20] in r0 (`ldr` at 0x137214, `ldm` at 0x13721c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x137110 | load | 4 | `abs 0x137220` | `-` |
| `ldr` at 0x137114 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x137118 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `ldr` at 0x137134 | load | 4 | `abs 0x137220` | `-` |
| `ldr` at 0x137138 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x13713c | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `ldr` at 0x137160 | load | 4 | `abs 0x137220` | `-` |
| `ldr` at 0x137164 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x137168 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `ldr` at 0x13718c | load | 4 | `abs 0x137220` | `-` |
| `ldr` at 0x137190 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x137194 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `ldr` at 0x1371b8 | load | 4 | `abs 0x137220` | `-` |
| `ldr` at 0x1371bc | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x1371c0 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `ldr` at 0x1371e4 | load | 4 | `abs 0x137220` | `-` |
| `ldr` at 0x1371e8 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x1371ec | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `str` at 0x137210 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0x137214 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x13721c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x13721c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x13721c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x13721c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x13721c | `r0` | `[sp-20]` |
