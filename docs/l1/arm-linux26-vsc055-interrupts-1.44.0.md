# ARM Linux 2.6 VSC055 reset and interrupt paths in 1.44.0

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
and pointee meanings remain unknown unless the body establishes them. Tables include all
explicit memory operands on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
A computed transfer in l1ioInit leaves unresolved edges; its tables cover known edges only.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-vsc055-interrupts-facts.txt) retain all tool sections.

## `vsc055_Init`

- `vsc055_Init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x127538 to 0x127780.
  Established. Word assignments store `ret(0x6e140)` at `0x1a0838` and `ret(0x204d8)` at `0x1a0834` (`str` at 0x1275e4, `str` at 0x127618).
  Another word assignment writes `ret(0x204d8)` to `[sp-36] +0x8` (`str` at 0x1276a0).
  The unnamed `0x20550` call carries `[[sp-36]+8]`, `0x1`, `[[sp-36]+4]` and `[sp-36]` (`bl` at 0x127758).
  VSC055 initialization hardware responsibilities remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x127548 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x12754c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x127550 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x127554 | load | 4 | `abs 0x127764` | `-` |
| `ldr` at 0x127558 | load | 4 | `abs 0x1a0834` | `-` |
| `ldr` at 0x12755c | load | 4 | `abs 0x127768` | `-` |
| `ldr` at 0x127570 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x12757c | load | 4 | `abs 0x12776c` | `-` |
| `str` at 0x127580 | store | 4 | `sp -0x3c` | `0x18ffc4` |
| `ldr` at 0x127584 | load | 4 | `abs 0x127764` | `-` |
| `ldr` at 0x127588 | load | 4 | `abs 0x1a0834` | `-` |
| `str` at 0x12758c | store | 4 | `sp -0x38` | `[0x1a0834]` |
| `ldr` at 0x127590 | load | 4 | `abs 0x127770` | `-` |
| `ldr` at 0x127594 | load | 4 | `abs 0x1a0838` | `-` |
| `str` at 0x127598 | store | 4 | `sp -0x34` | `[0x1a0838]` |
| `ldr` at 0x1275a0 | load | 4 | `abs 0x127774` | `-` |
| `ldr` at 0x1275b0 | load | 4 | `abs 0x127770` | `-` |
| `ldr` at 0x1275b4 | load | 4 | `abs 0x1a0838` | `-` |
| `ldr` at 0x1275c0 | load | 4 | `abs 0x127764` | `-` |
| `ldr` at 0x1275c4 | load | 4 | `abs 0x1a0834` | `-` |
| `ldr` at 0x1275c8 | load | 4 | `abs 0x127768` | `-` |
| `ldr` at 0x1275e0 | load | 4 | `abs 0x127770` | `-` |
| `str` at 0x1275e4 | store | 4 | `abs 0x1a0838` | `ret(0x6e140)` |
| `ldr` at 0x1275e8 | load | 4 | `abs 0x127770` | `-` |
| `ldr` at 0x1275ec | load | 4 | `abs 0x1a0838` | `-` |
| `str` at 0x127600 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x127604 | load | 4 | `abs 0x127764` | `-` |
| `ldr` at 0x127608 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x127618 | store | 4 | `abs 0x1a0834` | `ret(0x204d8)` |
| `ldr` at 0x12761c | load | 4 | `abs 0x1a0834` | `-` |
| `ldr` at 0x127628 | load | 4 | `abs 0x127778` | `-` |
| `str` at 0x12762c | store | 4 | `sp -0x3c` | `0x190010` |
| `ldr` at 0x127630 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x127634 | store | 4 | `sp -0x38` | `[sp-40]` |
| `ldr` at 0x12763c | load | 4 | `abs 0x127774` | `-` |
| `ldr` at 0x12764c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x127660 | load | 4 | `abs 0x127764` | `-` |
| `ldr` at 0x127664 | load | 4 | `abs 0x1a0834` | `-` |
| `str` at 0x12766c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x127670 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x127674 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x127678 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0x12767c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x127680 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x127684 | store | 4 | `[sp-36] +0x4` | `[sp-32]` |
| `ldr` at 0x127688 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x12768c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x127690 | load | 4 | `[sp-36] +0x4` | `-` |
| `str` at 0x1276a0 | store | 4 | `[sp-36] +0x8` | `ret(0x204d8)` |
| `ldr` at 0x1276a4 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0x1276b0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x1276c4 | store | 4 | `sp -0x28` | `ret(0x6e1b0)` |
| `ldr` at 0x1276c8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x1276d8 | store | 4 | `sp -0x2c` | `ret(0x2031c)+1` |
| `ldr` at 0x1276dc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x127704 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x127708 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x12770c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x127710 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12771c | load | 4 | `abs 0x12777c` | `-` |
| `str` at 0x127720 | store | 4 | `(unknown)` | `0x190048` |
| `str` at 0x127724 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x127728 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x12772c | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0x127734 | load | 4 | `abs 0x127774` | `-` |
| `ldr` at 0x127744 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x127748 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x12774c | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0x127754 | load | 4 | `[sp-36] +0x4` | `-` |
| `ldm` at 0x127760 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x127760 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x127760 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x127760 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x127568 | `(unknown callee) 0x6e190` | `r0` | `r1` | `[0x1a0834]` | `0x314fe0` |
| `bl` at 0x1275ac | `(unknown callee) 0x22070` | `0x9` | `0x18fff4` | `0x53` | `0x0` |
| `bl` at 0x1275d8 | `(unknown callee) 0x6e140` | `?` | `?` | `?` | `?` |
| `bl` at 0x127610 | `(unknown callee) 0x204d8` | `[sp-40]` | `0x1` | `[0x1a0838]` | `?` |
| `bl` at 0x127648 | `(unknown callee) 0x22070` | `0x9` | `0x18fff4` | `0x5e` | `0x0` |
| `bl` at 0x127698 | `(unknown callee) 0x204d8` | `[[sp-36]+4]` | `0x1` | `[sp-36]` | `[sp-36]` |
| `bl` at 0x1276c0 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x1276cc | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x127714 | `(unknown callee) 0x203f4` | `[sp-48]` | `[sp-40]` | `[sp-44]` | `?` |
| `bl` at 0x127740 | `(unknown callee) 0x22070` | `0x9` | `0x18fff4` | `0x6a` | `0x0` |
| `bl` at 0x127758 | `(unknown callee) 0x20550` | `[[sp-36]+8]` | `0x1` | `[[sp-36]+4]` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x127564 | `[0x1a0834] == 0x314fe0` | `[0x1a0834] != 0x314fe0` |
| `bne` at 0x127578 | `ret(0x6e190) != [sp-24]` | `ret(0x6e190) == [sp-24]` |
| `beq` at 0x1275bc | `[0x1a0838] == 0x0` | `[0x1a0838] != 0x0` |
| `beq` at 0x1275d0 | `[0x1a0834] == 0x314fe0` | `[0x1a0834] != 0x314fe0` |
| `bne` at 0x127624 | `[0x1a0834] != 0x0` | `[0x1a0834] == 0x0` |
| `bne` at 0x1276ac | `[[sp-36]+8] != 0x0` | `[[sp-36]+8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x127760 | `r0` | `ret(0x20550)` |

## `vsc055_Remove`

- `vsc055_Remove`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x127780 to 0x12783c.
  Established. Separate word assignments clear `[sp-24] +0x4`, `[sp-24] +0x0` and `[sp-24] +0x8` (`str` at 0x127804, `str` at 0x127810, `str` at 0x127828).
  The unnamed `0x20514` call carries `[[sp-24]+8]`, entry `r1`, `[sp-24]` and `[sp-24]` (`bl` at 0x12781c).
  Device removal and resource ownership remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x127790 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x127794 | load | 4 | `abs 0x127834` | `-` |
| `ldr` at 0x127798 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12779c | load | 4 | `abs 0x1a0838` | `-` |
| `ldr` at 0x1277a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1277bc | load | 4 | `abs 0x127838` | `-` |
| `ldr` at 0x1277c0 | load | 4 | `abs 0x1a0834` | `-` |
| `ldr` at 0x1277c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1277d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1277ec | load | 4 | `abs 0x127838` | `-` |
| `ldr` at 0x1277f0 | load | 4 | `abs 0x1a0834` | `-` |
| `str` at 0x1277f8 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x1277fc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x127804 | store | 4 | `[sp-24] +0x4` | `0x0` |
| `ldr` at 0x127808 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x127810 | store | 4 | `[sp-24] +0x0` | `0x0` |
| `ldr` at 0x127814 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x127818 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x127820 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x127828 | store | 4 | `[sp-24] +0x8` | `0x0` |
| `ldm` at 0x127830 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x127830 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x127830 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x12781c | `(unknown callee) 0x20514` | `[[sp-24]+8]` | `r1` | `[sp-24]` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x1277a4 | `r0 >= [0x1a0838] (signed, 32-bit)` | `r0 < [0x1a0838] (signed, 32-bit)` |
| `bne` at 0x1277d0 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x127830 | `r0` | `?` |

## `vsc055_Enable`

- `vsc055_Enable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12783c to 0x127a40.
  Established. A byte assignment writes `0x1` to `([sp-21]:1 + [[sp-44]+8]) +0x0` (`strb` at 0x12798c).
  The signed condition `(r1 & 0xff) >= [[sp-44]+4]` takes `0x127994`, while lower falls through (`bge` at 0x1278d0).
  The unnamed `0x124b4c` call carries `[sp-20]`, unknown, `[sp-28]` and unknown (`bl` at 0x127958).
  Enable effects on the physical device remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x12784c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x127854 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x127858 | load | 4 | `abs 0x127a30` | `-` |
| `ldr` at 0x12785c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x127860 | load | 4 | `abs 0x1a0838` | `-` |
| `ldr` at 0x12786c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x127880 | load | 4 | `abs 0x127a34` | `-` |
| `ldr` at 0x127884 | load | 4 | `abs 0x1a0834` | `-` |
| `ldr` at 0x12788c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x12789c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1278b0 | load | 4 | `abs 0x127a34` | `-` |
| `ldr` at 0x1278b4 | load | 4 | `abs 0x1a0834` | `-` |
| `str` at 0x1278bc | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0x1278c0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x1278c4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1278c8 | load | 4 | `[sp-44] +0x4` | `-` |
| `ldr` at 0x1278d4 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x1278d8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x1278e0 | load | 4 | `[sp-44] +0x0` | `-` |
| `str` at 0x1278e8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x1278f0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x127900 | store | 4 | `sp -0x24` | `ret(0x12576c)` |
| `strb` at 0x127908 | store | 1 | `sp -0x25` | `0x0` |
| `ldrb` at 0x12790c | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0x127910 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x12791c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x127920 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x127924 | load | 1 | `[sp-32] +0x1` | `-` |
| `ldrb` at 0x127928 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x127934 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x127938 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x12793c | load | 1 | `[sp-32] +0x2` | `-` |
| `ldrb` at 0x127940 | load | 1 | `[sp-28] +0x1` | `-` |
| `ldrb` at 0x12794c | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0x127950 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12795c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x127964 | store | 4 | `sp -0x20` | `[sp-32]+16` |
| `ldrb` at 0x127968 | load | 1 | `sp -0x25` | `-` |
| `strb` at 0x127970 | store | 1 | `sp -0x25` | `?` |
| `ldr` at 0x127978 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x12797c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x127980 | load | 4 | `[sp-44] +0x8` | `-` |
| `strb` at 0x12798c | store | 1 | `([sp-21]:1 + [[sp-44]+8]) +0x0` | `0x1` |
| `ldr` at 0x127994 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1279a8 | store | 4 | `sp -0x30` | `ret(0x6e1b0)` |
| `ldr` at 0x1279ac | load | 4 | `sp -0x30` | `-` |
| `str` at 0x1279bc | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x1279c0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x1279e8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x1279ec | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x1279f0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x1279f4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x127a00 | load | 4 | `abs 0x127a38` | `-` |
| `str` at 0x127a04 | store | 4 | `(unknown)` | `0x190084` |
| `str` at 0x127a08 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldrb` at 0x127a0c | load | 1 | `sp -0x15` | `-` |
| `str` at 0x127a10 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x127a18 | load | 4 | `abs 0x127a3c` | `-` |
| `ldm` at 0x127a2c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x127a2c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x127a2c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1278f8 | `(unknown callee) 0x12576c` | `r0` | `sp-32` | `?` | `sp-32` |
| `bl` at 0x127958 | `(unknown callee) 0x124b4c` | `[sp-20]` | `?` | `[sp-28]` | `?` |
| `bl` at 0x1279a4 | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x1279b0 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x1279f8 | `(unknown callee) 0x203f4` | `[sp-56]` | `[sp-48]` | `[sp-52]` | `?` |
| `bl` at 0x127a24 | `(unknown callee) 0x22070` | `0x6` | `0x18fff4` | `0x99` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x127868 | `r0 >= [0x1a0838] (signed, 32-bit)` | `r0 < [0x1a0838] (signed, 32-bit)` |
| `bne` at 0x127894 | `? != 0x0` | `? == 0x0` |
| `bge` at 0x1278d0 | `(r1 & 0xff) >= [[sp-44]+4] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-44]+4] (signed, 32-bit)` |
| `bge` at 0x127918 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x127930 | `[[sp-32]+1]:1 != [[sp-28]]:1` | `[[sp-32]+1]:1 == [[sp-28]]:1` |
| `bne` at 0x127948 | `[[sp-32]+2]:1 != [[sp-28]+1]:1` | `[[sp-32]+2]:1 == [[sp-28]+1]:1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x127a2c | `r0` | `?` |

## `vsc055_Disable`

- `vsc055_Disable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x127a40 to 0x127c40.
  Established. A byte assignment clears `((r1 & 0xff) + [[sp-44]+8]) +0x0` (`strb` at 0x127aec).
  Inequality `[[sp-32]+1]:1 != [[sp-28]]:1` takes `0x127b78`, while equality falls through (`bne` at 0x127b4c).
  The unnamed `0x12576c` call carries `[sp-20]`, `sp-32`, unknown and `sp-32` (`bl` at 0x127b14).
  Disable effects on the physical device remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x127a50 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x127a58 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x127a5c | load | 4 | `abs 0x127c30` | `-` |
| `ldr` at 0x127a60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x127a64 | load | 4 | `abs 0x1a0838` | `-` |
| `ldr` at 0x127a70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x127a84 | load | 4 | `abs 0x127c34` | `-` |
| `ldr` at 0x127a88 | load | 4 | `abs 0x1a0834` | `-` |
| `ldr` at 0x127a90 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x127aa0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x127ab4 | load | 4 | `abs 0x127c34` | `-` |
| `ldr` at 0x127ab8 | load | 4 | `abs 0x1a0834` | `-` |
| `str` at 0x127ac0 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0x127ac4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x127ac8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x127acc | load | 4 | `[sp-44] +0x4` | `-` |
| `ldr` at 0x127ad8 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x127adc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x127ae0 | load | 4 | `[sp-44] +0x8` | `-` |
| `strb` at 0x127aec | store | 1 | `((r1 & 0xff) + [[sp-44]+8]) +0x0` | `0x0` |
| `ldr` at 0x127af0 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x127af4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x127afc | load | 4 | `[sp-44] +0x0` | `-` |
| `str` at 0x127b04 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x127b0c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x127b1c | store | 4 | `sp -0x24` | `ret(0x12576c)` |
| `strb` at 0x127b24 | store | 1 | `sp -0x25` | `0x0` |
| `ldrb` at 0x127b28 | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0x127b2c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x127b38 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x127b3c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x127b40 | load | 1 | `[sp-32] +0x1` | `-` |
| `ldrb` at 0x127b44 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x127b50 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x127b54 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x127b58 | load | 1 | `[sp-32] +0x2` | `-` |
| `ldrb` at 0x127b5c | load | 1 | `[sp-28] +0x1` | `-` |
| `ldrb` at 0x127b68 | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0x127b6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x127b78 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x127b80 | store | 4 | `sp -0x20` | `[sp-32]+16` |
| `ldrb` at 0x127b84 | load | 1 | `sp -0x25` | `-` |
| `strb` at 0x127b8c | store | 1 | `sp -0x25` | `?` |
| `ldr` at 0x127b94 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x127ba8 | store | 4 | `sp -0x30` | `ret(0x6e1b0)` |
| `ldr` at 0x127bac | load | 4 | `sp -0x30` | `-` |
| `str` at 0x127bbc | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x127bc0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x127be8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x127bec | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x127bf0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x127bf4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x127c00 | load | 4 | `abs 0x127c38` | `-` |
| `str` at 0x127c04 | store | 4 | `(unknown)` | `0x1900a4` |
| `str` at 0x127c08 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldrb` at 0x127c0c | load | 1 | `sp -0x15` | `-` |
| `str` at 0x127c10 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x127c18 | load | 4 | `abs 0x127c3c` | `-` |
| `ldm` at 0x127c2c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x127c2c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x127c2c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x127b14 | `(unknown callee) 0x12576c` | `[sp-20]` | `sp-32` | `?` | `sp-32` |
| `bl` at 0x127b74 | `(unknown callee) 0x124cb8` | `[sp-20]` | `?` | `[sp-28]` | `?` |
| `bl` at 0x127ba4 | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x127bb0 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x127bf8 | `(unknown callee) 0x203f4` | `[sp-56]` | `[sp-48]` | `[sp-52]` | `?` |
| `bl` at 0x127c24 | `(unknown callee) 0x22070` | `0x6` | `0x18fff4` | `0xba` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x127a6c | `r0 >= [0x1a0838] (signed, 32-bit)` | `r0 < [0x1a0838] (signed, 32-bit)` |
| `bne` at 0x127a98 | `? != 0x0` | `? == 0x0` |
| `bge` at 0x127ad4 | `(r1 & 0xff) >= [[sp-44]+4] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-44]+4] (signed, 32-bit)` |
| `bge` at 0x127b34 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x127b4c | `[[sp-32]+1]:1 != [[sp-28]]:1` | `[[sp-32]+1]:1 == [[sp-28]]:1` |
| `bne` at 0x127b64 | `[[sp-32]+2]:1 != [[sp-28]+1]:1` | `[[sp-32]+2]:1 == [[sp-28]+1]:1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x127c2c | `r0` | `?` |

## `vsc055_ResetAll`

- `vsc055_ResetAll`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x127c40 to 0x127d24.
  Established. A byte assignment clears `sp -0x15` (`strb` at 0x127cc0).
  An included word read addresses `[sp-28] +0x4` (`ldr` at 0x127ccc).
  The unnamed `0x127d24` call has four unknown printed argument candidates (`bl` at 0x127d00).
  Reset coverage and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x127c50 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x127c54 | load | 4 | `abs 0x127d1c` | `-` |
| `ldr` at 0x127c58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x127c5c | load | 4 | `abs 0x1a0838` | `-` |
| `ldr` at 0x127c68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x127c7c | load | 4 | `abs 0x127d20` | `-` |
| `ldr` at 0x127c80 | load | 4 | `abs 0x1a0834` | `-` |
| `ldr` at 0x127c88 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x127c98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x127cac | load | 4 | `abs 0x127d20` | `-` |
| `ldr` at 0x127cb0 | load | 4 | `abs 0x1a0834` | `-` |
| `str` at 0x127cb8 | store | 4 | `sp -0x1c` | `?` |
| `strb` at 0x127cc0 | store | 1 | `sp -0x15` | `0x0` |
| `ldrb` at 0x127cc4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x127cc8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x127ccc | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x127cd8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x127cdc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x127ce0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0x127ce8 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x127cf4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x127cf8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x127d04 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x127d0c | store | 1 | `sp -0x15` | `?` |
| `ldm` at 0x127d18 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x127d18 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x127d18 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x127d00 | `(unknown callee) 0x127d24` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x127c64 | `r0 >= [0x1a0838] (signed, 32-bit)` | `r0 < [0x1a0838] (signed, 32-bit)` |
| `bne` at 0x127c90 | `? != 0x0` | `? == 0x0` |
| `bge` at 0x127cd4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x127cf0 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x127d18 | `r0` | `?` |

## `vsc055_Reset`

- `vsc055_Reset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x127d24 to 0x128054.
  Established. The unnamed `0x10adc4` call carries unknown, `[[sp-28]+1]:1`, `0xfc` and `0x80` (`bl` at 0x127ed0).
  The unnamed `0x10aef0` call carries unknown, `[[sp-28]+1]:1`, `0xff` and `sp-29` (`bl` at 0x127f2c).
  Inequality `[sp-29]:1 != 0x21` takes `0x127f58`, while equality falls through (`bne` at 0x127f50).
  Reset request semantics and hardware completion remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x127d34 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x127d3c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x127d40 | load | 4 | `abs 0x128038` | `-` |
| `ldr` at 0x127d44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x127d48 | load | 4 | `abs 0x1a0838` | `-` |
| `ldr` at 0x127d54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x127d68 | load | 4 | `abs 0x12803c` | `-` |
| `ldr` at 0x127d6c | load | 4 | `abs 0x1a0834` | `-` |
| `ldr` at 0x127d74 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x127d84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x127d98 | load | 4 | `abs 0x12803c` | `-` |
| `ldr` at 0x127d9c | load | 4 | `abs 0x1a0834` | `-` |
| `str` at 0x127da4 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0x127da8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x127dac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x127db0 | load | 4 | `[sp-44] +0x4` | `-` |
| `ldr` at 0x127dbc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x127dd0 | store | 4 | `sp -0x30` | `ret(0x6e1b0)` |
| `ldr` at 0x127dd4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x127de4 | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x127de8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x127e10 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x127e14 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x127e18 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x127e1c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x127e28 | load | 4 | `abs 0x128040` | `-` |
| `str` at 0x127e2c | store | 4 | `(unknown)` | `0x1900c8` |
| `str` at 0x127e30 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldrb` at 0x127e34 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x127e38 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x127e40 | load | 4 | `abs 0x128044` | `-` |
| `ldr` at 0x127e50 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x127e54 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x127e58 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldrb` at 0x127e60 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x127e70 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x127e74 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x127e7c | load | 4 | `[sp-44] +0x0` | `-` |
| `str` at 0x127e84 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x127e8c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x127e90 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x127e9c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x127ea0 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x127ea4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x127eb8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x127ebc | load | 1 | `[sp-28] +0x1` | `-` |
| `str` at 0x127ed8 | store | 4 | `sp -0x24` | `ret(0x10adc4)` |
| `ldr` at 0x127edc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x127ef4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x127ef8 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x127efc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x127f10 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x127f14 | load | 1 | `[sp-28] +0x1` | `-` |
| `str` at 0x127f34 | store | 4 | `sp -0x24` | `ret(0x10aef0)` |
| `ldr` at 0x127f38 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x127f48 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x127f58 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x127f60 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x127f68 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x127f7c | store | 4 | `sp -0x38` | `ret(0x6e1b0)` |
| `ldr` at 0x127f80 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x127f90 | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x127f94 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x127fbc | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x127fc0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x127fc4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x127fc8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x127fd0 | store | 4 | `sp -0x3c` | `ret(0x203f4)` |
| `ldr` at 0x127fd4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x127fe0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x127fe8 | store | 4 | `sp -0x40` | `ret(0x21ce0)` |
| `ldr` at 0x127ff0 | load | 4 | `abs 0x128048` | `-` |
| `str` at 0x127ff4 | store | 4 | `sp -0x40` | `0x1900e0` |
| `ldr` at 0x127ff8 | load | 4 | `abs 0x12804c` | `-` |
| `str` at 0x127ffc | store | 4 | `(unknown)` | `0x1900f8` |
| `ldr` at 0x128000 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x128004 | store | 4 | `(unknown)` | `[sp-60]` |
| `ldr` at 0x128008 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x12800c | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0x128010 | store | 4 | `(unknown)` | `[[sp-28]+4]` |
| `ldr` at 0x128014 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x128018 | store | 4 | `(unknown)` | `[sp-64]` |
| `ldr` at 0x128020 | load | 4 | `abs 0x128044` | `-` |
| `ldr` at 0x128024 | load | 4 | `abs 0x128050` | `-` |
| `ldm` at 0x128034 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x128034 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x128034 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x127dcc | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x127dd8 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x127e20 | `(unknown callee) 0x203f4` | `[sp-56]` | `[sp-48]` | `[sp-52]` | `?` |
| `bl` at 0x127e4c | `(unknown callee) 0x22070` | `0x9` | `0x18fff4` | `0xe1` | `0x0` |
| `bl` at 0x127eac | `(unknown callee) 0x10a4ec` | `?` | `[[sp-28]]:1` | `?` | `[[sp-28]]:1` |
| `bl` at 0x127ed0 | `(unknown callee) 0x10adc4` | `?` | `[[sp-28]+1]:1` | `0xfc` | `0x80` |
| `bl` at 0x127ef0 | `(unknown callee) 0x128054` | `0x2` | `?` | `?` | `ret(0x10adc4)` |
| `bl` at 0x127f04 | `(unknown callee) 0x10a4ec` | `[sp-20]` | `[[sp-28]]:1` | `?` | `[[sp-28]]:1` |
| `bl` at 0x127f2c | `(unknown callee) 0x10aef0` | `?` | `[[sp-28]+1]:1` | `0xff` | `sp-29` |
| `bl` at 0x127f78 | `(unknown callee) 0x6e1b0` | `?` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x127f84 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x127fcc | `(unknown callee) 0x203f4` | `[sp-48]` | `[sp-56]` | `[sp-52]` | `?` |
| `bl` at 0x127fe4 | `(unknown callee) 0x21ce0` | `[sp-36]` | `?` | `?` | `[sp-36]` |
| `bl` at 0x12802c | `(unknown callee) 0x22070` | `0x3` | `0x18fff4` | `0x101` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x127d50 | `r0 >= [0x1a0838] (signed, 32-bit)` | `r0 < [0x1a0838] (signed, 32-bit)` |
| `bne` at 0x127d7c | `? != 0x0` | `? == 0x0` |
| `ble` at 0x127db8 | `(r1 & 0xff) <= [[sp-44]+4] (signed, 32-bit)` | `(r1 & 0xff) > [[sp-44]+4] (signed, 32-bit)` |
| `bne` at 0x127e68 | `Z == 0` | `not (Z == 0)` |
| `bgt` at 0x127e98 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x127ee4 | `ret(0x10adc4) == 0x0` | `ret(0x10adc4) != 0x0` |
| `beq` at 0x127f40 | `ret(0x10aef0) == 0x0` | `ret(0x10aef0) != 0x0` |
| `bne` at 0x127f50 | `[sp-29]:1 != 0x21` | `[sp-29]:1 == 0x21` |
| `beq` at 0x127fdc | `[sp-36] == 0x0` | `[sp-36] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x128034 | `r0` | `?` |

## `vsc055_PortReset`

- `vsc055_PortReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12807c to 0x128224.
  Established. Byte reads address entry `r1 +0x5` and entry `r1 +0x6` (`ldrb` at 0x12810c, `ldrb` at 0x128110).
  The unnamed `0x10adc4` call carries unknown, `[[sp-24]+2]:1`, `[[sp-24]+3]:1` and `[sp-26]:1` (`bl` at 0x128188).
  A word assignment places `0xfffffc11` at `sp -0x2c` (`str` at 0x1280d8).
  Port reset fields and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x12808c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x128090 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x128094 | load | 4 | `abs 0x128218` | `-` |
| `ldr` at 0x128098 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12809c | load | 4 | `abs 0x1a0838` | `-` |
| `ldr` at 0x1280a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1280bc | load | 4 | `abs 0x12821c` | `-` |
| `ldr` at 0x1280c0 | load | 4 | `abs 0x1a0834` | `-` |
| `ldr` at 0x1280c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1280d4 | load | 4 | `abs 0x128220` | `-` |
| `str` at 0x1280d8 | store | 4 | `sp -0x2c` | `0xfffffc11` |
| `ldr` at 0x1280e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1280f4 | load | 4 | `abs 0x12821c` | `-` |
| `ldr` at 0x1280f8 | load | 4 | `abs 0x1a0834` | `-` |
| `str` at 0x128100 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x128104 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x128108 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x12810c | load | 1 | `r1 +0x5` | `-` |
| `ldrb` at 0x128110 | load | 1 | `r1 +0x6` | `-` |
| `strb` at 0x128118 | store | 1 | `sp -0x1a` | `?` |
| `ldr` at 0x12811c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x128120 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x128124 | load | 1 | `r1 +0x5` | `-` |
| `ldr` at 0x12812c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x128134 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldrb` at 0x12813c | load | 1 | `r1 +0x4` | `-` |
| `strb` at 0x128144 | store | 1 | `sp -0x19` | `?` |
| `ldr` at 0x128148 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x12814c | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0x128150 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x128164 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x128168 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0x12816c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x128170 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldrb` at 0x128174 | load | 1 | `sp -0x1a` | `-` |
| `str` at 0x128190 | store | 4 | `sp -0x20` | `ret(0x10adc4)` |
| `ldr` at 0x128194 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x128198 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0x12819c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1281b0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x1281b4 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0x1281b8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x1281bc | load | 1 | `[sp-24] +0x3` | `-` |
| `ldrb` at 0x1281c8 | load | 1 | `sp -0x19` | `-` |
| `str` at 0x1281e0 | store | 4 | `sp -0x24` | `ret(0x10adc4)` |
| `ldr` at 0x1281e4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1281f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x1281fc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x128200 | store | 4 | `sp -0x20` | `ret(0x10adc4)` |
| `ldr` at 0x128204 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x128208 | store | 4 | `sp -0x2c` | `[sp-32]` |
| `ldr` at 0x12820c | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x128214 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x128214 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x128214 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x128158 | `(unknown callee) 0x10a4ec` | `[sp-20]` | `[[sp-24]+1]:1` | `[r1+4]:1` | `[[sp-24]+1]:1` |
| `bl` at 0x128188 | `(unknown callee) 0x10adc4` | `?` | `[[sp-24]+2]:1` | `[[sp-24]+3]:1` | `[sp-26]:1` |
| `bl` at 0x1281a4 | `(unknown callee) 0x10a4ec` | `[sp-20]` | `[[sp-24]+1]:1` | `?` | `[[sp-24]+1]:1` |
| `bl` at 0x1281d8 | `(unknown callee) 0x10adc4` | `?` | `[[sp-24]+2]:1` | `?` | `[sp-25]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x1280a4 | `r0 >= [0x1a0838] (signed, 32-bit)` | `r0 < [0x1a0838] (signed, 32-bit)` |
| `bne` at 0x1280d0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x1281ec | `ret(0x10adc4) == 0x0` | `ret(0x10adc4) != 0x0` |
| `bne` at 0x1281f8 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x128214 | `r0` | `?` |

## `vsc055_PortSoftReset`

- `vsc055_PortSoftReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x128224 to 0x128594.
  Established. The unnamed `0x10aef0` call carries unknown, `[[sp-24]+2]:1`, unknown and `sp-27` (`bl` at 0x128288);
  its zero result takes `0x1282ac`, while nonzero falls through (`beq` at 0x12829c).
  A later unnamed `0x10adc4` call carries unknown, `[[sp-24]+2]:1`, `[[sp-24]+3]:1` and unknown (`bl` at 0x128478).
  Soft reset sequencing and physical effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x128234 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x128238 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x128240 | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `ldr` at 0x128244 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x128248 | load | 1 | `r1 +0x1` | `-` |
| `ldr` at 0x12824c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x128260 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x128264 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0x128268 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x12826c | load | 1 | `[sp-24] +0x3` | `-` |
| `str` at 0x128290 | store | 4 | `sp -0x20` | `ret(0x10aef0)` |
| `ldr` at 0x128294 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x1282a0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x1282a4 | store | 4 | `sp -0x30` | `ret(0x10aef0)` |
| `ldr` at 0x1282ac | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x1282b0 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldrb` at 0x1282b4 | load | 1 | `sp -0x1b` | `-` |
| `strb` at 0x1282cc | store | 1 | `sp -0x1a` | `?` |
| `ldr` at 0x1282d0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x1282d4 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x1282dc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x1282e0 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldrb` at 0x1282e8 | load | 1 | `sp -0x1b` | `-` |
| `ldrb` at 0x1282f8 | load | 1 | `sp -0x1a` | `-` |
| `strb` at 0x128300 | store | 1 | `sp -0x1a` | `?` |
| `ldr` at 0x128304 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x128308 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x12830c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x128310 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldrb` at 0x128318 | load | 1 | `sp -0x1b` | `-` |
| `ldrb` at 0x128324 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0x128334 | load | 1 | `sp -0x1a` | `-` |
| `strb` at 0x12833c | store | 1 | `sp -0x1a` | `?` |
| `ldrb` at 0x128340 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0x12834c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x128350 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x128354 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x128358 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldrb` at 0x12835c | load | 1 | `[sp-24] +0x5` | `-` |
| `ldrb` at 0x12836c | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0x128374 | load | 1 | `[sp-24] +0x4` | `-` |
| `strb` at 0x12837c | store | 1 | `sp -0x1c` | `?` |
| `ldr` at 0x128380 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x128394 | store | 4 | `sp -0x24` | `ret(0x6e1b0)` |
| `ldr` at 0x128398 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1283a8 | store | 4 | `sp -0x28` | `ret(0x2031c)+1` |
| `ldr` at 0x1283ac | load | 4 | `sp -0x28` | `-` |
| `str` at 0x1283d4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x1283d8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1283dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1283e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x1283ec | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x1283f0 | load | 1 | `sp -0x1a` | `-` |
| `ldrb` at 0x1283f4 | load | 1 | `sp -0x1b` | `-` |
| `str` at 0x1283f8 | store | 4 | `(unknown)` | `[sp-27]:1` |
| `ldrb` at 0x1283fc | load | 1 | `sp -0x1c` | `-` |
| `str` at 0x128400 | store | 4 | `(unknown)` | `[sp-28]:1` |
| `ldrb` at 0x128404 | load | 1 | `sp -0x19` | `-` |
| `str` at 0x128408 | store | 4 | `(unknown)` | `[sp-25]:1` |
| `ldr` at 0x12840c | load | 4 | `abs 0x128590` | `-` |
| `ldr` at 0x128414 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x128420 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x128424 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0x128428 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12843c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x128440 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0x128444 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x128448 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0x12844c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x128450 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldrb` at 0x128454 | load | 1 | `sp -0x1b` | `-` |
| `ldrb` at 0x128460 | load | 1 | `sp -0x19` | `-` |
| `str` at 0x128480 | store | 4 | `sp -0x20` | `ret(0x10adc4)` |
| `ldr` at 0x128484 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x128490 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x128494 | store | 4 | `sp -0x30` | `ret(0x10adc4)` |
| `ldr` at 0x12849c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x1284a0 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0x1284a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1284b8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x1284bc | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0x1284c0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x1284c4 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldrb` at 0x1284d0 | load | 1 | `sp -0x1c` | `-` |
| `str` at 0x1284e8 | store | 4 | `sp -0x20` | `ret(0x10adc4)` |
| `ldr` at 0x1284ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x1284f8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x1284fc | store | 4 | `sp -0x30` | `ret(0x10adc4)` |
| `ldr` at 0x128504 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x128508 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0x12850c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x128520 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x128524 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0x128528 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x12852c | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0x128530 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x128534 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x128538 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldrb` at 0x12853c | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0x128544 | load | 1 | `[sp-24] +0x4` | `-` |
| `str` at 0x128560 | store | 4 | `sp -0x20` | `ret(0x10adc4)` |
| `ldr` at 0x128564 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x128570 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x128574 | store | 4 | `sp -0x30` | `ret(0x10adc4)` |
| `str` at 0x128580 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x128584 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x12858c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x12858c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x12858c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x128254 | `(unknown callee) 0x10a4ec` | `r0` | `[r1+1]:1` | `r2` | `[r1+1]:1` |
| `bl` at 0x128288 | `(unknown callee) 0x10aef0` | `?` | `[[sp-24]+2]:1` | `?` | `sp-27` |
| `bl` at 0x128390 | `(unknown callee) 0x6e1b0` | `[sp-20]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x12839c | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x1283e4 | `(unknown callee) 0x203f4` | `[sp-44]` | `[sp-36]` | `[sp-40]` | `?` |
| `bl` at 0x12841c | `(unknown callee) 0x29908` | `0x19011c` | `ret(0x203f4)` | `[[sp-24]+12]` | `[sp-26]:1` |
| `bl` at 0x128430 | `(unknown callee) 0x10a4ec` | `[sp-20]` | `[[sp-24]+1]:1` | `?` | `[[sp-24]+1]:1` |
| `bl` at 0x128478 | `(unknown callee) 0x10adc4` | `?` | `[[sp-24]+2]:1` | `[[sp-24]+3]:1` | `?` |
| `bl` at 0x1284ac | `(unknown callee) 0x10a4ec` | `[sp-20]` | `[[sp-24]+1]:1` | `?` | `[[sp-24]+1]:1` |
| `bl` at 0x1284e0 | `(unknown callee) 0x10adc4` | `?` | `[[sp-24]+2]:1` | `?` | `[sp-28]:1` |
| `bl` at 0x128514 | `(unknown callee) 0x10a4ec` | `[sp-20]` | `[[sp-24]+1]:1` | `?` | `[[sp-24]+1]:1` |
| `bl` at 0x128558 | `(unknown callee) 0x10adc4` | `?` | `[[sp-24]+2]:1` | `[[sp-24]+3]:1` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x12829c | `ret(0x10aef0) == 0x0` | `ret(0x10aef0) != 0x0` |
| `beq` at 0x1282f4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x128330 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x128348 | `[sp-26]:1 == 0x0` | `[sp-26]:1 != 0x0` |
| `beq` at 0x12848c | `ret(0x10adc4) == 0x0` | `ret(0x10adc4) != 0x0` |
| `beq` at 0x1284f4 | `ret(0x10adc4) == 0x0` | `ret(0x10adc4) != 0x0` |
| `beq` at 0x12856c | `ret(0x10adc4) == 0x0` | `ret(0x10adc4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x12858c | `r0` | `?` |

## `vsc055_GetCfg`

- `vsc055_GetCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x128594 to 0x128638.
  Established. The signed condition entry `r0 < [0x1a0838]` takes `0x1285d8`, while greater than or equal falls through (`blt` at 0x1285bc).
  The fall-through path clears the word at entry `r1 +0x0` (`str` at 0x1285c8).
  Another word assignment writes `[[sp-28]]` to entry `r1 +0x0` (`str` at 0x128614).
  Configuration record ownership and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1285a4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x1285a8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x1285ac | load | 4 | `abs 0x128630` | `-` |
| `ldr` at 0x1285b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1285b4 | load | 4 | `abs 0x1a0838` | `-` |
| `ldr` at 0x1285c0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x1285c8 | store | 4 | `r1 +0x0` | `0x0` |
| `str` at 0x1285d0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x1285d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1285ec | load | 4 | `abs 0x128634` | `-` |
| `ldr` at 0x1285f0 | load | 4 | `abs 0x1a0834` | `-` |
| `str` at 0x1285f8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x1285fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x128608 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x12860c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x128610 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x128614 | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0x128618 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x12861c | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0x128620 | store | 4 | `sp -0x20` | `[[sp-28]+4]` |
| `ldr` at 0x128624 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x12862c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x12862c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x12862c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x1285bc | `r0 < [0x1a0838] (signed, 32-bit)` | `r0 >= [0x1a0838] (signed, 32-bit)` |
| `beq` at 0x128604 | `r1 == 0x0` | `r1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x12862c | `r0` | `?` |

## `vsc055_Count`

- `vsc055_Count`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x128638 to 0x1286a8.
  Established. The signed condition entry `r0 < [0x1a0838]` takes `0x12866c`, while greater than or equal falls through (`blt` at 0x12865c).
  The fall-through path clears the word at `sp -0x18` (`str` at 0x128664).
  A word read on the taken path addresses `0x1a0834` (`ldr` at 0x128684).
  Count-field meaning and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x128648 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x12864c | load | 4 | `abs 0x1286a0` | `-` |
| `ldr` at 0x128650 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x128654 | load | 4 | `abs 0x1a0838` | `-` |
| `str` at 0x128664 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x12866c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x128680 | load | 4 | `abs 0x1286a4` | `-` |
| `ldr` at 0x128684 | load | 4 | `abs 0x1a0834` | `-` |
| `ldr` at 0x12868c | load | 4 | `(unknown)` | `-` |
| `str` at 0x128690 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x128694 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x12869c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x12869c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x12869c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x12865c | `r0 < [0x1a0838] (signed, 32-bit)` | `r0 >= [0x1a0838] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x12869c | `r0` | `?` |

## `vsc055_Present`

- `vsc055_Present`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1286a8 to 0x128784.
  Established. The signed condition entry `r0 >= [0x1a0838]` takes `0x128730`, while lower falls through (`bge` at 0x1286f8).
  The taken path clears the word at `sp -0x20` (`str` at 0x128734).
  A separate byte read has unknown effective address in the emitted facts (`ldrb` at 0x128768).
  Presence encoding and physical device detection remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1286b8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x1286c0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x1286c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1286d8 | load | 4 | `abs 0x12877c` | `-` |
| `ldr` at 0x1286dc | load | 4 | `abs 0x1a0834` | `-` |
| `str` at 0x1286e4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x1286e8 | load | 4 | `abs 0x128780` | `-` |
| `ldr` at 0x1286ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1286f0 | load | 4 | `abs 0x1a0838` | `-` |
| `ldr` at 0x1286fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x128710 | load | 4 | `abs 0x12877c` | `-` |
| `ldr` at 0x128714 | load | 4 | `abs 0x1a0834` | `-` |
| `ldrb` at 0x12871c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x128720 | load | 4 | `(unknown)` | `-` |
| `str` at 0x128734 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x12873c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x128750 | load | 4 | `abs 0x12877c` | `-` |
| `ldr` at 0x128754 | load | 4 | `abs 0x1a0834` | `-` |
| `ldrb` at 0x12875c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x128760 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x128768 | load | 1 | `(unknown)` | `-` |
| `str` at 0x12876c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x128770 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x128778 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x128778 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x128778 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x1286f8 | `r0 >= [0x1a0838] (signed, 32-bit)` | `r0 < [0x1a0838] (signed, 32-bit)` |
| `blt` at 0x128728 | `? < (r1 & 0xff) (signed, 32-bit)` | `? >= (r1 & 0xff) (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x128778 | `r0` | `?` |

## `vsc055_IntSet`

- `vsc055_IntSet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x128784 to 0x1288a0.
  Established. A byte read addresses entry `r3 +0x1a083c` (`ldrb` at 0x1287b4).
  The unnamed `0x10adc4` call carries unknown, `[[sp-24]+2]:1`, `[sp-34]:1` and `[sp-35]:1` (`bl` at 0x128848);
  its zero result takes `0x12886c`, while nonzero falls through (`beq` at 0x12885c).
  Interrupt-setting register identity and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x128794 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x128798 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x12879c | store | 4 | `sp -0x20` | `r3` |
| `strb` at 0x1287a4 | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `ldr` at 0x1287a8 | load | 4 | `abs 0x12889c` | `-` |
| `ldr` at 0x1287ac | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x1287b4 | load | 1 | `r3 +0x1a083c` | `-` |
| `strb` at 0x1287b8 | store | 1 | `sp -0x23` | `([r3+1706044]:1 & 0xff)` |
| `strb` at 0x1287c0 | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0x1287c4 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0x1287d0 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x1287e0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x1287e4 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x1287f0 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x128808 | store | 1 | `sp -0x22` | `?` |
| `ldr` at 0x12880c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x128810 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0x128814 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x128828 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x12882c | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0x128830 | load | 1 | `sp -0x22` | `-` |
| `ldrb` at 0x128834 | load | 1 | `sp -0x23` | `-` |
| `str` at 0x128850 | store | 4 | `sp -0x28` | `ret(0x10adc4)` |
| `ldr` at 0x128854 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x128860 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x128864 | store | 4 | `sp -0x2c` | `ret(0x10adc4)` |
| `ldrb` at 0x12886c | load | 1 | `sp -0x19` | `-` |
| `strb` at 0x128874 | store | 1 | `sp -0x19` | `?` |
| `ldrb` at 0x128878 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x128880 | store | 1 | `sp -0x21` | `?` |
| `str` at 0x12888c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x128890 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x128898 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x128898 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x128898 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x12881c | `(unknown callee) 0x10a4ec` | `[sp-20]` | `[[sp-24]+1]:1` | `?` | `[[sp-24]+1]:1` |
| `bl` at 0x128848 | `(unknown callee) 0x10adc4` | `?` | `[[sp-24]+2]:1` | `[sp-34]:1` | `[sp-35]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhi` at 0x1287cc | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0x1287dc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x12885c | `ret(0x10adc4) == 0x0` | `ret(0x10adc4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x128898 | `r0` | `?` |

## `vsc055_IntGet`

- `vsc055_IntGet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1288a0 to 0x1289e8.
  Established. A byte assignment clears entry `r2 +0x0` (`strb` at 0x1288c4).
  The unnamed `0x10aef0` call carries unknown, `[[sp-24]+2]:1`, `[sp-30]:1` and `sp-31` (`bl` at 0x128944);
  its zero result takes `0x128968`, while nonzero falls through (`beq` at 0x128958).
  A later byte assignment to `[sp-28] +0x0` has unknown value (`strb` at 0x1289bc).
  Interrupt status encoding and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1288b0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x1288b4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x1288b8 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x1288bc | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x1288c4 | store | 1 | `r2 +0x0` | `0x0` |
| `strb` at 0x1288cc | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0x1288d0 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x1288dc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x1288e0 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldrb` at 0x1288ec | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x128904 | store | 1 | `sp -0x1e` | `?` |
| `ldr` at 0x128908 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x12890c | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0x128910 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x128924 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x128928 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0x12892c | load | 1 | `sp -0x1e` | `-` |
| `str` at 0x12894c | store | 4 | `sp -0x24` | `ret(0x10aef0)` |
| `ldr` at 0x128950 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x12895c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x128960 | store | 4 | `sp -0x28` | `ret(0x10aef0)` |
| `ldrb` at 0x128968 | load | 1 | `sp -0x1f` | `-` |
| `str` at 0x128970 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x128974 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x128984 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12898c | load | 4 | `abs 0x1289e4` | `-` |
| `ldr` at 0x1289a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1289a4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x1289a8 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x1289b4 | load | 1 | `[sp-28] +0x0` | `-` |
| `strb` at 0x1289bc | store | 1 | `[sp-28] +0x0` | `?` |
| `ldrb` at 0x1289c0 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x1289c8 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `str` at 0x1289d4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x1289d8 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x1289e0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1289e0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1289e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x128918 | `(unknown callee) 0x10a4ec` | `[sp-20]` | `[[sp-24]+1]:1` | `?` | `[[sp-24]+1]:1` |
| `bl` at 0x128944 | `(unknown callee) 0x10aef0` | `?` | `[[sp-24]+2]:1` | `[sp-30]:1` | `sp-31` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhi` at 0x1288d8 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0x128958 | `ret(0x10aef0) == 0x0` | `ret(0x10aef0) != 0x0` |
| `bhi` at 0x12897c | `[sp-44] > 0xe (unsigned, 32-bit)` | `[sp-44] <= 0xe (unsigned, 32-bit)` |
| `bne` at 0x128998 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1289e0 | `r0` | `?` |

## `vsc055_Int`

- `vsc055_Int`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1289e8 to 0x128ac0.
  Established. A word read addresses `[sp-28] +0x8` (`ldr` at 0x128a54).
  The unnamed `0x128ac0` call has four unknown printed argument candidates (`bl` at 0x128a88).
  A word assignment writes `([sp-36] + ret(0x128ac0))` to `sp -0x24` (`str` at 0x128a98).
  Interrupt aggregation and physical interrupt behavior remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1289f8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x128a00 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x128a08 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0x128a0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x128a20 | load | 4 | `abs 0x128abc` | `-` |
| `ldr` at 0x128a24 | load | 4 | `abs 0x1a0834` | `-` |
| `str` at 0x128a2c | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x128a34 | store | 4 | `sp -0x24` | `0x0` |
| `ldrb` at 0x128a38 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x128a3c | store | 4 | `sp -0x20` | `(r1 & 0xff)` |
| `ldrb` at 0x128a40 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x128a44 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x128a50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x128a54 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x128a58 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x128a60 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x128a6c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x128a70 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x128a78 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x128a80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x128a90 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x128a98 | store | 4 | `sp -0x24` | `([sp-36] + ret(0x128ac0))` |
| `ldr` at 0x128a9c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x128aa4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x128aac | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x128ab8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x128ab8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x128ab8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x128a88 | `(unknown callee) 0x128ac0` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x128a4c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x128a68 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x128ab8 | `r0` | `?` |

## `l1ioInit`

- `l1ioInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x90b74 to 0x91698.
  Established. Word assignments store `ret(0x2046c)` at `0x1a0344` and `ret(0x2016c)` at `0x1a6450` (`str` at 0x90bfc, `str` at 0x90c88).
  Separate word assignments write `0x18`, `0x3` and `0x0` to `[sp-88] +0x0`, `[sp-88] +0x8` and `[sp-88] +0xc` (`str` at 0x913c8, `str` at 0x913dc, `str` at 0x913f0).
  An unnamed `0x2016c` call carries `0x0`, `0x100`, `0x3` and `0x1` (`bl` at 0x912f0).
  The computed transfer remains unresolved, so omitted initialization choices and hardware register roles remain Unknown (`ldrls` at 0x90d74).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x90b84 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x90b8c | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x90b94 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x90b9c | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x90ba4 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0x90bac | store | 4 | `sp -0x54` | `0x0` |
| `str` at 0x90bb4 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0x90bbc | store | 4 | `sp -0x5c` | `0x0` |
| `str` at 0x90bc4 | store | 4 | `sp -0x60` | `0x0` |
| `str` at 0x90bcc | store | 4 | `sp -0x64` | `0x0` |
| `str` at 0x90bd4 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0x90bd8 | load | 4 | `abs 0x915f0` | `-` |
| `ldr` at 0x90bdc | load | 4 | `abs 0x1a6450` | `-` |
| `ldr` at 0x90be8 | load | 4 | `abs 0x915f4` | `-` |
| `ldr` at 0x90bec | load | 4 | `abs 0x915f8` | `-` |
| `ldr` at 0x90bf8 | load | 4 | `abs 0x915fc` | `-` |
| `str` at 0x90bfc | store | 4 | `abs 0x1a0344` | `ret(0x2046c)` |
| `ldr` at 0x90c00 | load | 4 | `abs 0x1a0344` | `-` |
| `ldr` at 0x90c14 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x90c2c | load | 4 | `abs 0x91600` | `-` |
| `str` at 0x90c30 | store | 4 | `sp -0x88` | `0x168254` |
| `ldr` at 0x90c34 | load | 4 | `abs 0x91604` | `-` |
| `str` at 0x90c38 | store | 4 | `sp -0x84` | `0x16823c` |
| `str` at 0x90c3c | store | 4 | `sp -0x80` | `ret(0x219a4)` |
| `ldr` at 0x90c40 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `str` at 0x90c44 | store | 4 | `sp -0x7c` | `[ret(0x20184)]` |
| `ldr` at 0x90c4c | load | 4 | `abs 0x91608` | `-` |
| `ldr` at 0x90c5c | load | 4 | `abs 0x915f0` | `-` |
| `ldr` at 0x90c60 | load | 4 | `abs 0x915fc` | `-` |
| `ldr` at 0x90c64 | load | 4 | `abs 0x1a0344` | `-` |
| `str` at 0x90c68 | store | 4 | `sp -0x88` | `[0x1a0344]` |
| `str` at 0x90c70 | store | 4 | `sp -0x84` | `0x10000000` |
| `str` at 0x90c88 | store | 4 | `abs 0x1a6450` | `ret(0x2016c)` |
| `ldr` at 0x90c8c | load | 4 | `abs 0x1a6450` | `-` |
| `ldr` at 0x90ca0 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x90cb8 | load | 4 | `abs 0x9160c` | `-` |
| `str` at 0x90cbc | store | 4 | `sp -0x88` | `0x168280` |
| `ldr` at 0x90cc0 | load | 4 | `abs 0x91604` | `-` |
| `str` at 0x90cc4 | store | 4 | `sp -0x84` | `0x16823c` |
| `str` at 0x90cc8 | store | 4 | `sp -0x80` | `ret(0x219a4)` |
| `ldr` at 0x90ccc | load | 4 | `ret(0x20184) +0x0` | `-` |
| `str` at 0x90cd0 | store | 4 | `sp -0x7c` | `[ret(0x20184)]` |
| `ldr` at 0x90cd8 | load | 4 | `abs 0x91608` | `-` |
| `ldr` at 0x90ce8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x90d04 | load | 4 | `abs 0x91610` | `-` |
| `ldr` at 0x90d08 | load | 4 | `abs 0x91614` | `-` |
| `str` at 0x90d14 | store | 4 | `sp -0x6c` | `ret(0x2046c)` |
| `ldr` at 0x90d18 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x90d2c | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x90d38 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x90d40 | load | 4 | `abs 0x91618` | `-` |
| `ldr` at 0x90d4c | load | 4 | `sp -0x6c` | `-` |
| `str` at 0x90d58 | store | 4 | `sp -0x70` | `0x0` |
| `ldrls` at 0x90d74 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x91024 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x91030 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x91034 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x91040 | load | 4 | `abs 0x91650` | `-` |
| `str` at 0x91044 | store | 4 | `sp -0x88` | `0x1684b4` |
| `ldr` at 0x91048 | load | 4 | `abs 0x91604` | `-` |
| `str` at 0x9104c | store | 4 | `sp -0x84` | `0x16823c` |
| `ldr` at 0x91054 | load | 4 | `abs 0x91608` | `-` |
| `str` at 0x91068 | store | 4 | `sp -0x70` | `0x0` |
| `ldr` at 0x91070 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x91074 | load | 4 | `abs 0x91654` | `-` |
| `str` at 0x91080 | store | 4 | `sp -0x28` | `ret(0x20580)` |
| `ldr` at 0x91084 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x91098 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x910b0 | load | 4 | `abs 0x91658` | `-` |
| `str` at 0x910b4 | store | 4 | `sp -0x88` | `0x1684f0` |
| `ldr` at 0x910b8 | load | 4 | `abs 0x91604` | `-` |
| `str` at 0x910bc | store | 4 | `sp -0x84` | `0x16823c` |
| `ldr` at 0x910c0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x910c4 | store | 4 | `sp -0x80` | `[sp-32]` |
| `str` at 0x910c8 | store | 4 | `sp -0x7c` | `ret(0x219a4)` |
| `ldr` at 0x910cc | load | 4 | `ret(0x20184) +0x0` | `-` |
| `str` at 0x910d0 | store | 4 | `sp -0x78` | `[ret(0x20184)]` |
| `ldr` at 0x910d8 | load | 4 | `abs 0x91608` | `-` |
| `str` at 0x910ec | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0x910f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x91108 | load | 4 | `abs 0x9165c` | `-` |
| `str` at 0x9110c | store | 4 | `sp -0x88` | `0x168520` |
| `ldr` at 0x91110 | load | 4 | `abs 0x91604` | `-` |
| `str` at 0x91114 | store | 4 | `sp -0x84` | `0x16823c` |
| `ldr` at 0x91118 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9111c | store | 4 | `sp -0x80` | `[sp-32]` |
| `ldr` at 0x91124 | load | 4 | `abs 0x91608` | `-` |
| `str` at 0x91138 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x91140 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x91150 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9115c | store | 4 | `sp -0x24` | `(ret(0x2031c) + [sp-32])-3` |
| `ldr` at 0x91160 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x91164 | load | 4 | `abs 0x91660` | `-` |
| `ldr` at 0x9118c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x911a4 | load | 4 | `abs 0x91664` | `-` |
| `str` at 0x911a8 | store | 4 | `sp -0x88` | `0x168554` |
| `ldr` at 0x911ac | load | 4 | `abs 0x91604` | `-` |
| `str` at 0x911b0 | store | 4 | `sp -0x84` | `0x16823c` |
| `ldr` at 0x911b4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x911b8 | store | 4 | `sp -0x80` | `[sp-32]` |
| `ldr` at 0x911c0 | load | 4 | `abs 0x91608` | `-` |
| `str` at 0x911d4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x911dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x911e0 | load | 4 | `abs 0x91668` | `-` |
| `str` at 0x911f8 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0x91200 | load | 4 | `abs 0x9166c` | `-` |
| `str` at 0x91204 | store | 4 | `sp -0x88` | `0x168588` |
| `ldr` at 0x91208 | load | 4 | `abs 0x91604` | `-` |
| `str` at 0x9120c | store | 4 | `sp -0x84` | `0x16823c` |
| `ldr` at 0x91210 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x91214 | store | 4 | `sp -0x80` | `[sp-32]` |
| `ldr` at 0x9121c | load | 4 | `abs 0x91608` | `-` |
| `ldr` at 0x91220 | load | 4 | `abs 0x91670` | `-` |
| `str` at 0x91230 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x91238 | load | 4 | `abs 0x915fc` | `-` |
| `ldr` at 0x9123c | load | 4 | `abs 0x1a0344` | `-` |
| `str` at 0x91240 | store | 4 | `sp -0x88` | `[0x1a0344]` |
| `str` at 0x91248 | store | 4 | `sp -0x84` | `0x59000000` |
| `str` at 0x91260 | store | 4 | `sp -0x58` | `ret(0x2016c)` |
| `ldr` at 0x91264 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x91278 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x91290 | load | 4 | `abs 0x91674` | `-` |
| `str` at 0x91294 | store | 4 | `sp -0x88` | `0x1685bc` |
| `ldr` at 0x91298 | load | 4 | `abs 0x91604` | `-` |
| `str` at 0x9129c | store | 4 | `sp -0x84` | `0x16823c` |
| `str` at 0x912a0 | store | 4 | `sp -0x80` | `ret(0x219a4)` |
| `ldr` at 0x912a4 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `str` at 0x912a8 | store | 4 | `sp -0x7c` | `[ret(0x20184)]` |
| `ldr` at 0x912b0 | load | 4 | `abs 0x91608` | `-` |
| `ldr` at 0x912b4 | load | 4 | `abs 0x91678` | `-` |
| `str` at 0x912c4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x912cc | load | 4 | `abs 0x915fc` | `-` |
| `ldr` at 0x912d0 | load | 4 | `abs 0x1a0344` | `-` |
| `str` at 0x912d4 | store | 4 | `sp -0x88` | `[0x1a0344]` |
| `str` at 0x912dc | store | 4 | `sp -0x84` | `0x56000000` |
| `str` at 0x912f4 | store | 4 | `sp -0x5c` | `ret(0x2016c)` |
| `ldr` at 0x912f8 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x9130c | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x91324 | load | 4 | `abs 0x9167c` | `-` |
| `str` at 0x91328 | store | 4 | `sp -0x88` | `0x1685e4` |
| `ldr` at 0x9132c | load | 4 | `abs 0x91604` | `-` |
| `str` at 0x91330 | store | 4 | `sp -0x84` | `0x16823c` |
| `str` at 0x91334 | store | 4 | `sp -0x80` | `ret(0x219a4)` |
| `ldr` at 0x91338 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `str` at 0x9133c | store | 4 | `sp -0x7c` | `[ret(0x20184)]` |
| `ldr` at 0x91344 | load | 4 | `abs 0x91608` | `-` |
| `str` at 0x91358 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x91360 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x91368 | store | 4 | `sp -0x64` | `[sp-92]+128` |
| `ldr` at 0x9136c | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x91374 | store | 4 | `sp -0x68` | `[sp-92]+176` |
| `ldr` at 0x91378 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x91380 | store | 4 | `sp -0x60` | `[sp-92]+68` |
| `ldr` at 0x91384 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x91388 | load | 4 | `[sp-92] +0x44` | `-` |
| `str` at 0x9138c | store | 4 | `sp -0x30` | `[[sp-92]+68]` |
| `ldr` at 0x91390 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x91394 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x9139c | store | 4 | `[sp-92] +0x44` | `?` |
| `ldr` at 0x913a8 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x913ac | load | 4 | `sp -0x30` | `-` |
| `str` at 0x913b4 | store | 4 | `[sp-96] +0x0` | `?` |
| `ldr` at 0x913c0 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x913c8 | store | 4 | `[sp-88] +0x0` | `0x18` |
| `ldr` at 0x913d0 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x913dc | store | 4 | `[sp-88] +0x8` | `0x3` |
| `ldr` at 0x913e4 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x913f0 | store | 4 | `[sp-88] +0xc` | `0x0` |
| `str` at 0x913f8 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0x913fc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x91410 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9141c | store | 4 | `sp -0x48` | `ret(0x20004)` |
| `ldr` at 0x91420 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9142c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x91438 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x91450 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x91458 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x91460 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x91464 | store | 4 | `[sp-88] +0x10` | `[sp-72]` |
| `ldr` at 0x91468 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x91470 | store | 4 | `sp -0x4c` | `[sp-76]+1` |
| `ldr` at 0x91478 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x91480 | load | 4 | `[sp-88] +0x4` | `-` |
| `ldr` at 0x9149c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x914a4 | load | 4 | `[sp-88] +0x4` | `-` |
| `ldr` at 0x914b4 | load | 4 | `abs 0x91680` | `-` |
| `str` at 0x914b8 | store | 4 | `sp -0x88` | `0x16860c` |
| `ldr` at 0x914bc | load | 4 | `abs 0x91604` | `-` |
| `str` at 0x914c0 | store | 4 | `sp -0x84` | `0x16823c` |
| `ldr` at 0x914c8 | load | 4 | `abs 0x91608` | `-` |
| `ldr` at 0x914cc | load | 4 | `abs 0x91684` | `-` |
| `str` at 0x914dc | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x914e4 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x914f0 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x914f4 | load | 4 | `[sp-96] +0x0` | `-` |
| `ldr` at 0x91504 | load | 4 | `abs 0x91688` | `-` |
| `str` at 0x91508 | store | 4 | `sp -0x88` | `0x168634` |
| `ldr` at 0x9150c | load | 4 | `abs 0x91604` | `-` |
| `str` at 0x91510 | store | 4 | `sp -0x84` | `0x16823c` |
| `ldr` at 0x91518 | load | 4 | `abs 0x91608` | `-` |
| `str` at 0x9152c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x9153c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x91540 | load | 4 | `[sp-104] +0x0` | `-` |
| `ldr` at 0x91544 | load | 4 | `abs 0x9168c` | `-` |
| `ldr` at 0x91550 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x91554 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x91558 | load | 4 | `[sp-100] +0x0` | `-` |
| `str` at 0x91560 | store | 4 | `[sp-100] +0x0` | `?` |
| `ldr` at 0x91564 | load | 4 | `abs 0x91690` | `-` |
| `ldr` at 0x9156c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x91570 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x91574 | load | 4 | `[sp-100] +0x0` | `-` |
| `str` at 0x9157c | store | 4 | `[sp-100] +0x0` | `?` |
| `ldr` at 0x91580 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x9158c | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x91598 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x915a4 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x915b0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x915bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x915c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x915d0 | load | 4 | `abs 0x91694` | `-` |
| `ldr` at 0x915d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x915dc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x915e0 | store | 4 | `sp -0x70` | `[sp-28]` |
| `ldr` at 0x915e4 | load | 4 | `sp -0x70` | `-` |
| `ldm` at 0x915ec | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x915ec | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x915ec | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x915ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x90bf0 | `(unknown callee) 0x2046c` | `0x168248` | `0x1002` | `r2` | `[0x1a6450]` |
| `bl` at 0x90c0c | `(unknown callee) 0x20184` | `ret(0x2046c)` | `?` | `ret(0x2046c)` | `[0x1a0344]` |
| `bl` at 0x90c18 | `(unknown callee) 0x219a4` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x90c20 | `(unknown callee) 0x20184` | `ret(0x219a4)` | `?` | `?` | `?` |
| `bl` at 0x90c58 | `(unknown callee) 0x22070` | `0x9` | `0x168278` | `0x62` | `0x0` |
| `bl` at 0x90c84 | `(unknown callee) 0x2016c` | `0x0` | `0x100` | `0x3` | `0x1` |
| `bl` at 0x90c98 | `(unknown callee) 0x20184` | `ret(0x2016c)` | `?` | `?` | `[0x1a6450]` |
| `bl` at 0x90ca4 | `(unknown callee) 0x219a4` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x90cac | `(unknown callee) 0x20184` | `ret(0x219a4)` | `?` | `?` | `?` |
| `bl` at 0x90ce4 | `(unknown callee) 0x22070` | `0x9` | `0x168278` | `0x68` | `0x0` |
| `bl` at 0x90cf4 | `(unknown callee) 0x20fd8` | `?` | `?` | `?` | `?` |
| `bl` at 0x90d0c | `(unknown callee) 0x2046c` | `0x1682ac` | `0x802` | `?` | `ret(0x20fd8)` |
| `bl` at 0x90d24 | `(unknown callee) 0x20184` | `ret(0x2046c)` | `?` | `?` | `ret(0x2046c)` |
| `bl` at 0x90d3c | `(unknown callee) 0x200b8` | `[sp-108]` | `?` | `?` | `[ret(0x20184)]` |
| `bl` at 0x90d44 | `(unknown callee) 0x29908` | `0x1682bc` | `?` | `?` | `?` |
| `bl` at 0x90d50 | `(unknown callee) 0x200b8` | `?` | `?` | `?` | `?` |
| `bl` at 0x90d60 | `(unknown callee) 0x6dc0c` | `?` | `?` | `?` | `?` |
| `bl` at 0x91060 | `(unknown callee) 0x22070` | `0x4` | `0x168278` | `0xd5` | `0x0` |
| `bl` at 0x91078 | `(unknown callee) 0x20580` | `[sp-32]` | `0x1684ec` | `?` | `[[sp-32]]:1` |
| `bl` at 0x91090 | `(unknown callee) 0x20184` | `ret(0x20580)` | `?` | `?` | `ret(0x20580)` |
| `bl` at 0x9109c | `(unknown callee) 0x219a4` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x910a4 | `(unknown callee) 0x20184` | `ret(0x219a4)` | `?` | `?` | `?` |
| `bl` at 0x910e4 | `(unknown callee) 0x22070` | `0x3` | `0x168278` | `0xde` | `0x0` |
| `bl` at 0x910f8 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `ret(0x20580)` |
| `bl` at 0x91130 | `(unknown callee) 0x22070` | `0x3` | `0x168278` | `0xea` | `0x0` |
| `bl` at 0x91144 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `ret(0x2031c)` |
| `bl` at 0x91168 | `(unknown callee) 0x200ac` | `(ret(0x2031c) + [sp-32])-3` | `0x168550` | `ret(0x2031c)` | `(ret(0x2031c) + [sp-32])-3` |
| `bl` at 0x91180 | `(unknown callee) 0x13196c` | `sp-68` | `?` | `?` | `sp-68` |
| `bl` at 0x91190 | `(unknown callee) 0x131a10` | `sp-68` | `[sp-40]` | `?` | `sp-68` |
| `bl` at 0x911cc | `(unknown callee) 0x22070` | `0x3` | `0x168278` | `0xfa` | `0x0` |
| `bl` at 0x911e4 | `(unknown callee) 0x200ac` | `[sp-36]` | `0x168584` | `?` | `ret(0x200ac)` |
| `bl` at 0x91228 | `(unknown callee) 0x22070` | `0x3` | `0x168278` | `0x106` | `0x0` |
| `bl` at 0x9125c | `(unknown callee) 0x2016c` | `0x0` | `0x100` | `0x3` | `0x1` |
| `bl` at 0x91270 | `(unknown callee) 0x20184` | `ret(0x2016c)` | `?` | `?` | `ret(0x2016c)` |
| `bl` at 0x9127c | `(unknown callee) 0x219a4` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x91284 | `(unknown callee) 0x20184` | `ret(0x219a4)` | `?` | `?` | `?` |
| `bl` at 0x912bc | `(unknown callee) 0x22070` | `0x3` | `0x168278` | `0x113` | `0x0` |
| `bl` at 0x912f0 | `(unknown callee) 0x2016c` | `0x0` | `0x100` | `0x3` | `0x1` |
| `bl` at 0x91304 | `(unknown callee) 0x20184` | `ret(0x2016c)` | `?` | `?` | `ret(0x2016c)` |
| `bl` at 0x91310 | `(unknown callee) 0x219a4` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x91318 | `(unknown callee) 0x20184` | `ret(0x219a4)` | `?` | `?` | `?` |
| `bl` at 0x91350 | `(unknown callee) 0x22070` | `0x3` | `0x168278` | `0x120` | `0x0` |
| `bl` at 0x913a4 | `(unknown callee) 0x1fff8` | `0xa` | `?` | `[sp-92]+68` | `?` |
| `bl` at 0x913bc | `(unknown callee) 0x1fff8` | `0xa` | `?` | `[sp-96]` | `?` |
| `bl` at 0x91400 | `(unknown callee) 0x2004c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x91414 | `(unknown callee) 0x20004` | `[sp-40]` | `?` | `?` | `ret(0x2004c)` |
| `bl` at 0x91444 | `(unknown callee) 0x91698` | `?` | `?` | `?` | `?` |
| `bl` at 0x914d4 | `(unknown callee) 0x22070` | `0x3` | `0x168278` | `0x15a` | `0x0` |
| `bl` at 0x91524 | `(unknown callee) 0x22070` | `0x3` | `0x168278` | `0x160` | `0x0` |
| `bl` at 0x91538 | `(unknown callee) 0x1fff8` | `0xa` | `?` | `?` | `?` |
| `bl` at 0x91568 | `(unknown callee) 0x1fff8` | `0x2710` | `?` | `[sp-100]` | `?` |
| `bl` at 0x91594 | `(unknown callee) 0x2022c` | `[sp-92]` | `0x100` | `?` | `[sp-92]` |
| `bl` at 0x915ac | `(unknown callee) 0x2022c` | `[sp-88]` | `0x100` | `?` | `[sp-88]` |
| `bl` at 0x915c0 | `(unknown callee) 0x20400` | `[sp-40]` | `?` | `?` | `[sp-40]` |
| `bl` at 0x915d8 | `(unknown callee) 0x29908` | `0x168664` | `[sp-32]` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x90be4 | `[0x1a6450] != 0x0` | `[0x1a6450] == 0x0` |
| `bge` at 0x90c08 | `[0x1a0344] >= 0x0 (signed, 32-bit)` | `[0x1a0344] < 0x0 (signed, 32-bit)` |
| `bne` at 0x90c94 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x90cf0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x90d00 | `ret(0x20fd8) == 0x0` | `ret(0x20fd8) != 0x0` |
| `bge` at 0x90d20 | `ret(0x2046c) >= 0x0 (signed, 32-bit)` | `ret(0x2046c) < 0x0 (signed, 32-bit)` |
| `bne` at 0x90d34 | `[ret(0x20184)] != 0x13` | `[ret(0x20184)] == 0x13` |
| `ldrls` at 0x90d74 | `? <= 0x59 (unsigned, 32-bit)` | `? > 0x59 (unsigned, 32-bit)` |
| `beq` at 0x9102c | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `bne` at 0x9103c | `[[sp-32]]:1 != 0x0` | `[[sp-32]]:1 == 0x0` |
| `bne` at 0x9108c | `ret(0x20580) != 0x0` | `ret(0x20580) == 0x0` |
| `bhi` at 0x91104 | `ret(0x2031c) > 0x2 (unsigned, 32-bit)` | `ret(0x2031c) <= 0x2 (unsigned, 32-bit)` |
| `bne` at 0x91174 | `ret(0x200ac) != 0x0` | `ret(0x200ac) == 0x0` |
| `bne` at 0x9119c | `ret(0x131a10) != 0x0` | `ret(0x131a10) == 0x0` |
| `bne` at 0x911f0 | `ret(0x200ac) != 0x0` | `ret(0x200ac) == 0x0` |
| `bne` at 0x9126c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x91300 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x9140c | `ret(0x2004c) != 0x0` | `ret(0x2004c) == 0x0` |
| `beq` at 0x91428 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x91434 | `[sp-44] == 0x0` | `[sp-44] != 0x0` |
| `beq` at 0x91490 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x914b0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x914ec | `[sp-84] != 0x0` | `[sp-84] == 0x0` |
| `bne` at 0x91500 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x9154c | `[[sp-104]] != 0x32410000` | `[[sp-104]] == 0x32410000` |
| `beq` at 0x91588 | `[sp-92] == 0x0` | `[sp-92] != 0x0` |
| `beq` at 0x915a0 | `[sp-88] == 0x0` | `[sp-88] != 0x0` |
| `beq` at 0x915b8 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `bne` at 0x915cc | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x915ec | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x90d74).
