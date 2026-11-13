# ARM Linux 2.6 ICMB, checksum and FRU helpers in 1.44.0

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
register locations r0 through r3 with unknown arity. Custom calling conventions remain unknown.
Stack stores appear in the access tables.
Unknown values and predicates include unsupported instruction effects and merged paths.
Numeric call targets have unknown callee names in the flat-image analysis. Device roles
and pointee meanings remain unknown unless the body establishes them. Tables include the
explicit memory accesses reported on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
No unresolved computed transfers were reported for these routines.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-icmb-checksum-fru-facts.txt) retain all tool sections.

## `IcmbCvtToU16`

- `IcmbCvtToU16`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xede7c to 0xeded8.
  Established. Byte loads read entry r0 plus 1 and entry r0 (`ldrb` at 0xede9c, `ldrb` at 0xedeb4).
  A halfword store writes ([r0+1]:1 & 0xffff) at sp-0x16 (`strh` at 0xedea0).
  Halfword stores at sp-0x16 also include unknown values (`strh` at 0xedeac, `strh` at 0xedec4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xede8c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xede94 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xede9c | load | 1 | `r0 +0x1` | `-` |
| `strh` at 0xedea0 | store | 2 | `sp -0x16` | `([r0+1]:1 & 0xffff)` |
| `ldrh` at 0xedea4 | load | 2 | `sp -0x16` | `-` |
| `strh` at 0xedeac | store | 2 | `sp -0x16` | `?` |
| `ldr` at 0xedeb0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xedeb4 | load | 1 | `r0 +0x0` | `-` |
| `ldrh` at 0xedebc | load | 2 | `sp -0x16` | `-` |
| `strh` at 0xedec4 | store | 2 | `sp -0x16` | `?` |
| `ldrh` at 0xedec8 | load | 2 | `sp -0x16` | `-` |
| `ldm` at 0xeded4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xeded4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xeded4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xeded4 | `r0` | `[sp-22]:2` |

## `IcmbCvtToMsg`

- `IcmbCvtToMsg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeded8 to 0xedf20.
  Established. A byte store writes ((r1 & 0xffff) & 0xff) at entry r0 (`strb` at 0xedefc).
  A separate byte store at the address formed by [sp-20] plus 1 has an unknown value (`strb` at 0xedf14).
  The return record preserves entry r0 (`ldm` at 0xedf1c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xedee8 | store | 4 | `sp -0x14` | `r0` |
| `strh` at 0xedef0 | store | 2 | `sp -0x16` | `(r1 & 0xffff)` |
| `ldr` at 0xedef4 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xedef8 | load | 2 | `sp -0x16` | `-` |
| `strb` at 0xedefc | store | 1 | `r0 +0x0` | `((r1 & 0xffff) & 0xff)` |
| `ldr` at 0xedf04 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xedf0c | load | 2 | `sp -0x16` | `-` |
| `strb` at 0xedf14 | store | 1 | `[sp-20] +0x1` | `?` |
| `ldm` at 0xedf1c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xedf1c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xedf1c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xedf1c | `r0` | `r0` |

## `IcmbMsgGetMsgType`

- `IcmbMsgGetMsgType`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xedf20 to 0xedf5c.
  Established. A byte load reads entry r0 (`ldrb` at 0xedf38).
  The instruction sequence places 0x80 in r3, applies AND and shifts arithmetically right by seven (`mov` at 0xedf3c, `and` at 0xedf40, `asr` at 0xedf44).
  The word stored at sp-0x18 has an unknown value (`str` at 0xedf48).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xedf30 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xedf34 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xedf38 | load | 1 | `r0 +0x0` | `-` |
| `str` at 0xedf48 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xedf4c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xedf58 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xedf58 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xedf58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xedf58 | `r0` | `[sp-24]` |

## `IcmbMsgGetSeqNbr`

- `IcmbMsgGetSeqNbr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xedfb8 to 0xedfec.
  Established. A byte load reads entry r0 (`ldrb` at 0xedfd0).
  The AND instruction uses the mask 0x1f (`and` at 0xedfd4).
  A byte store at sp-0x15 has an unknown value and the recorded return is [sp-21]:1 (`strb` at 0xedfd8, `ldm` at 0xedfe8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xedfc8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xedfcc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xedfd0 | load | 1 | `r0 +0x0` | `-` |
| `strb` at 0xedfd8 | store | 1 | `sp -0x15` | `?` |
| `ldrb` at 0xedfdc | load | 1 | `sp -0x15` | `-` |
| `ldm` at 0xedfe8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xedfe8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xedfe8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xedfe8 | `r0` | `[sp-21]:1` |

## `IcmbMsgGetCmd`

- `IcmbMsgGetCmd`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xee03c to 0xee074.
  Established. Entry r0 is saved as a word at sp-0x14 (`str` at 0xee04c).
  The byte loaded from entry r0 plus 1 is stored as a word at sp-0x18 (`ldrb` at 0xee05c, `str` at 0xee060).
  The recorded r0 value retains that loaded byte (`ldrb` at 0xee05c, `ldm` at 0xee070).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xee04c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xee054 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xee05c | load | 1 | `r0 +0x1` | `-` |
| `str` at 0xee060 | store | 4 | `sp -0x18` | `[r0+1]:1` |
| `ldr` at 0xee064 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xee070 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xee070 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xee070 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xee070 | `r0` | `[r0+1]:1` |

## `IcmbMsgGetCmpCode`

- `IcmbMsgGetCmpCode`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xee0a8 to 0xee0e0.
  Established. Entry r0 is saved as a word at sp-0x14 (`str` at 0xee0b8).
  A byte loaded from entry r0 plus 2 is stored as a word at sp-0x18 (`ldrb` at 0xee0c8, `str` at 0xee0cc).
  The reported return in r0 is [r0+2]:1 (`ldrb` at 0xee0c8, `ldm` at 0xee0dc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xee0b8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xee0c0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xee0c8 | load | 1 | `r0 +0x2` | `-` |
| `str` at 0xee0cc | store | 4 | `sp -0x18` | `[r0+2]:1` |
| `ldr` at 0xee0d0 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xee0dc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xee0dc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xee0dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xee0dc | `r0` | `[r0+2]:1` |

## `IcmbMsgGetData`

- `IcmbMsgGetData`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xee0e0 to 0xee13c.
  Established. The result from unnamed 0xedf20 is stored as a word at sp-0x18 (`bl` at 0xee0f8, `str` at 0xee100).
  The branch takes 0xee120 when that call result is nonzero and falls through when it is zero (`bl` at 0xee0f8, `bne` at 0xee10c).
  Word stores at sp-0x1c include [sp-20]+2 and [sp-20]+3 (`str` at 0xee118, `str` at 0xee128).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xee0f0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xee0f4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xee100 | store | 4 | `sp -0x18` | `ret(0xedf20)` |
| `ldr` at 0xee104 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xee110 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xee118 | store | 4 | `sp -0x1c` | `[sp-20]+2` |
| `ldr` at 0xee120 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xee128 | store | 4 | `sp -0x1c` | `[sp-20]+3` |
| `ldr` at 0xee12c | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xee138 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xee138 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xee138 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xee0f8 | `(unknown callee) 0xedf20` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xee10c | `ret(0xedf20) != 0x0` | `ret(0xedf20) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xee138 | `r0` | `[sp-28]` |

## `IcmbMsgRqBuild`

- `IcmbMsgRqBuild`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xee13c to 0xee1bc.
  Established. A byte store preserves the low eight bits of entry r0 at sp-0x11 (`strb` at 0xee15c).
  A word store writes entry r3 plus 2 at sp-0x24 (`str` at 0xee168).
  Printed candidates for the call to unnamed 0x203f4 include r0=[sp]+2, r1=[sp-28], r2=[sp-32] and r3=[sp]+2 (`bl` at 0xee1a8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xee14c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xee150 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xee154 | store | 4 | `sp -0x20` | `r3` |
| `strb` at 0xee15c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xee160 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xee168 | store | 4 | `sp -0x24` | `r3+2` |
| `ldr` at 0xee16c | load | 4 | `sp +0x0` | `-` |
| `ldrb` at 0xee178 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xee17c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xee188 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xee18c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xee194 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xee1a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xee1a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xee1ac | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xee1b8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xee1b8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xee1b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xee174 | `(unknown callee) 0xedf5c` | `[sp]` | `0x0` | `r2` | `r3+2` |
| `bl` at 0xee184 | `(unknown callee) 0xedfec` | `[sp]` | `[sp-17]:1` | `?` | `[sp-17]:1` |
| `bl` at 0xee190 | `(unknown callee) 0xee074` | `[sp]` | `[sp-24]` | `?` | `?` |
| `bl` at 0xee1a8 | `(unknown callee) 0x203f4` | `[sp]+2` | `[sp-28]` | `[sp-32]` | `[sp]+2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xee1b8 | `r0` | `[sp-36]` |

## `ipmiInit`

- `ipmiInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52784 to 0x52794.
  Established. Included implicit word saves, with PC omitted, place entry r11, sp and lr at sp-0x10, sp-0xc and sp-8 (`push` at 0x52788).
  The ldm reads words at those three stack locations (`ldm` at 0x52790).
  The exit record preserves entry r0 (`ldm` at 0x52790).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldm` at 0x52790 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x52790 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x52790 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x52790 | `r0` | `r0` |

## `ipmiCheckCSum`

- `ipmiCheckCSum`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52794 to 0x52800.
  Established. A byte store writes zero at sp-0x19 (`strb` at 0x527b0).
  The branch takes 0x527f0 for Z == 1 and falls through for not (Z == 1) (`beq` at 0x527c4).
  A load, add and word store write [sp-20]+1 at sp-0x14 (`ldr` at 0x527cc, `add` at 0x527e0, `str` at 0x527e4).
  A byte store writes (([sp-25]:1 + [[sp-20]]:1) & 0xff) at sp-0x19 (`ldrb` at 0x527d4, `ldrb` at 0x527d8, `add` at 0x527dc, `strb` at 0x527e8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x527a4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x527a8 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x527b0 | store | 1 | `sp -0x19` | `0x0` |
| `ldr` at 0x527b4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x527bc | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x527cc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x527d4 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0x527d8 | load | 1 | `[sp-20] +0x0` | `-` |
| `str` at 0x527e4 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `strb` at 0x527e8 | store | 1 | `sp -0x19` | `(([sp-25]:1 + [[sp-20]]:1) & 0xff)` |
| `ldrb` at 0x527f0 | load | 1 | `sp -0x19` | `-` |
| `ldm` at 0x527fc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x527fc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x527fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x527c4 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x527fc | `r0` | `[sp-25]:1` |

## `ipmiCalcCSum`

- `ipmiCalcCSum`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52800 to 0x52878.
  Established. A byte store initializes sp-0x19 to zero (`strb` at 0x5281c).
  A computed byte load has an unknown effective address (`ldrb` at 0x52844).
  A word store at sp-0x14 and a byte store at sp-0x19 have unknown values (`str` at 0x52850, `strb` at 0x52854).
  The return record leaves r0 unknown after the unsupported mvn effect (`mvn` at 0x52860, `ldm` at 0x52874).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x52810 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x52814 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x5281c | store | 1 | `sp -0x19` | `0x0` |
| `ldr` at 0x52820 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x52828 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x52838 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x52840 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0x52844 | load | 1 | `(unknown)` | `-` |
| `str` at 0x52850 | store | 4 | `sp -0x14` | `?` |
| `strb` at 0x52854 | store | 1 | `sp -0x19` | `?` |
| `ldrb` at 0x5285c | load | 1 | `sp -0x19` | `-` |
| `ldm` at 0x52874 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x52874 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x52874 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x52830 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x52874 | `r0` | `?` |

## `FRUName`

- `FRUName`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x105fe4 to 0x106010.
  Established. Word stores save entry r0 and r1 at sp-0x14 and sp-0x18 (`str` at 0x105ff4, `str` at 0x105ff8).
  A word load from absolute address 0x10600c places 0x18bfa8 in r3 (`ldr` at 0x105ffc).
  The recorded return in r0 is 0x18bfa8 (`ldm` at 0x106008).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x105ff4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x105ff8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x105ffc | load | 4 | `abs 0x10600c` | `-` |
| `ldm` at 0x106008 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x106008 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x106008 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x106008 | `r0` | `0x18bfa8` |

## `_FRUDumpXML`

- `_FRUDumpXML`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x106f04 to 0x106f2c.
  Established. Word stores preserve entry r0 and r1 at sp-0x14 and sp-0x18 (`str` at 0x106f14, `str` at 0x106f18).
  Two other word stores preserve entry r2 and r3 at sp-0x1c and sp-0x20 (`str` at 0x106f1c, `str` at 0x106f20).
  The exit record retains entry r0 (`ldm` at 0x106f28).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x106f14 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x106f18 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x106f1c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x106f20 | store | 4 | `sp -0x20` | `r3` |
| `ldm` at 0x106f28 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x106f28 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x106f28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x106f28 | `r0` | `r0` |

## `FRUDump`

- `FRUDump`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x106f2c to 0x106ff8.
  Established. The result from unnamed 0x104a4c is stored as a word at sp-0xb0 (`bl` at 0x106f58, `str` at 0x106f60).
  The branch takes 0x106fa8 when that call result is zero and falls through when it is nonzero (`bl` at 0x106f58, `beq` at 0x106f6c).
  Printed candidates for the call to unnamed 0x205bc include r0=sp-172 and r1=0x18c270 (`bl` at 0x106f8c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x106f3c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x106f40 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x106f44 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x106f48 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x106f50 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x106f60 | store | 4 | `sp -0xb0` | `ret(0x104a4c)` |
| `ldr` at 0x106f64 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x106f70 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x106f84 | load | 4 | `abs 0x106ff4` | `-` |
| `ldr` at 0x106f94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x106f98 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x106fa8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x106fb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x106fbc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x106fc0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x106fd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x106fd4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x106fd8 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x106ff0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x106ff0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x106ff0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x106f58 | `(unknown callee) 0x104a4c` | `[sp]` | `sp-92` | `r2` | `sp-92` |
| `bl` at 0x106f74 | `(unknown callee) 0x1073c8` | `ret(0x104a4c)` | `?` | `?` | `ret(0x104a4c)` |
| `bl` at 0x106f8c | `(unknown callee) 0x205bc` | `sp-172` | `0x18c270` | `ret(0x1073c8)` | `ret(0x1073c8)` |
| `bl` at 0x106fa0 | `(unknown callee) 0x105fb0` | `[sp-28]` | `[sp-32]` | `sp-172` | `sp-172` |
| `bl` at 0x106fc4 | `(unknown callee) 0x106010` | `[sp-20]` | `[sp-28]` | `[sp-32]` | `sp-92` |
| `bl` at 0x106fdc | `(unknown callee) 0x106f04` | `[sp-20]` | `[sp-28]` | `[sp-32]` | `sp-92` |
| `bl` at 0x106fe8 | `(unknown callee) 0x1070e8` | `sp-92` | `?` | `?` | `sp-92` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x106f6c | `ret(0x104a4c) == 0x0` | `ret(0x104a4c) != 0x0` |
| `bne` at 0x106fb0 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x106ff0 | `r0` | `?` |

## `FRUDumpAll`

- `FRUDumpAll`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x106ff8 to 0x1070e8.
  Established. The result from unnamed 0x104a4c is stored as a word at sp-0xb0 (`bl` at 0x107024, `str` at 0x10702c).
  The branch takes 0x107074 when that call result is zero and falls through when it is nonzero (`bl` at 0x107024, `beq` at 0x107038).
  Word stores at sp-0xbc include 1 and an unknown value (`str` at 0x107078, `str` at 0x1070c8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x107008 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x10700c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x107010 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x107014 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x10701c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x10702c | store | 4 | `sp -0xb0` | `ret(0x104a4c)` |
| `ldr` at 0x107030 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x10703c | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x107050 | load | 4 | `abs 0x1070e4` | `-` |
| `ldr` at 0x107060 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x107064 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x107078 | store | 4 | `sp -0xbc` | `0x1` |
| `ldr` at 0x10707c | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x107088 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x107098 | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x10709c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1070a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1070b0 | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x1070b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1070b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1070c0 | load | 4 | `sp -0xbc` | `-` |
| `str` at 0x1070c8 | store | 4 | `sp -0xbc` | `?` |
| `ldm` at 0x1070e0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1070e0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1070e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x107024 | `(unknown callee) 0x104a4c` | `r3` | `sp-92` | `r2` | `sp-92` |
| `bl` at 0x107040 | `(unknown callee) 0x1073c8` | `ret(0x104a4c)` | `?` | `?` | `ret(0x104a4c)` |
| `bl` at 0x107058 | `(unknown callee) 0x205bc` | `sp-172` | `0x18c270` | `ret(0x1073c8)` | `ret(0x1073c8)` |
| `bl` at 0x10706c | `(unknown callee) 0x105fb0` | `[sp-24]` | `[sp-28]` | `sp-172` | `sp-172` |
| `bl` at 0x1070a4 | `(unknown callee) 0x106010` | `?` | `[sp-24]` | `[sp-28]` | `sp-92` |
| `bl` at 0x1070bc | `(unknown callee) 0x106f04` | `?` | `[sp-24]` | `[sp-28]` | `sp-92` |
| `bl` at 0x1070d8 | `(unknown callee) 0x1070e8` | `sp-92` | `?` | `?` | `sp-92` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x107038 | `ret(0x104a4c) == 0x0` | `ret(0x104a4c) != 0x0` |
| `beq` at 0x107084 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x107090 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1070e0 | `r0` | `?` |
