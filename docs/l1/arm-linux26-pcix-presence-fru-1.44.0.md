# ARM Linux 2.6 PCI-X presence and FRU routines in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-pcix-presence-fru-facts.txt) retain all tool sections.

## `pcixOverrideSpeed`

- `pcixOverrideSpeed`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7b3a8 to 0x7b5a4.
  Established. Byte stores save the low bytes of entry r1, r2 and r3 at sp-0x15, sp-0x16 and sp-0x17 (`strb` at 0x7b3c4, `strb` at 0x7b3cc, `strb` at 0x7b3d4).
  A word store writes 0xfffff703 at sp-0x28 (`str` at 0x7b444).
  The byte increment loads sp-0x1d, adds one and stores ([sp-29]:1+1 & 0xff) there (`ldrb` at 0x7b4e8, `add` at 0x7b4ec, `strb` at 0x7b4f0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7b3b8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x7b3c4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x7b3cc | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x7b3d4 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldrb` at 0x7b3d8 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x7b3dc | load | 1 | `sp -0x17` | `-` |
| `strb` at 0x7b3e4 | store | 1 | `sp -0x1e` | `?` |
| `ldr` at 0x7b3e8 | load | 4 | `abs 0x7b598` | `-` |
| `ldr` at 0x7b3ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7b3f0 | load | 4 | `abs 0x1a6220` | `-` |
| `ldr` at 0x7b3fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7b410 | load | 4 | `abs 0x7b59c` | `-` |
| `ldr` at 0x7b414 | load | 4 | `abs 0x1a621c` | `-` |
| `ldr` at 0x7b41c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x7b428 | load | 1 | `sp -0x16` | `-` |
| `str` at 0x7b438 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x7b440 | load | 4 | `abs 0x7b5a0` | `-` |
| `str` at 0x7b444 | store | 4 | `sp -0x28` | `0xfffff703` |
| `ldr` at 0x7b448 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x7b44c | store | 4 | `sp -0x24` | `[sp-40]` |
| `ldr` at 0x7b454 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7b468 | load | 4 | `abs 0x7b59c` | `-` |
| `ldr` at 0x7b46c | load | 4 | `abs 0x1a621c` | `-` |
| `str` at 0x7b474 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x7b478 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x7b488 | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0x7b48c | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x7b490 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7b494 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0x7b4b0 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x7b4bc | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x7b4c0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x7b4d0 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x7b4d8 | load | 1 | `sp -0x1e` | `-` |
| `ldrb` at 0x7b4e8 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x7b4f0 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldrb` at 0x7b4f8 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x7b504 | load | 4 | `abs 0x7b5a0` | `-` |
| `str` at 0x7b508 | store | 4 | `sp -0x24` | `0xfffff703` |
| `ldrb` at 0x7b510 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x7b518 | store | 1 | `sp -0x16` | `([sp-22]:1-1 & 0xff)` |
| `ldrb` at 0x7b51c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x7b520 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7b524 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x7b530 | load | 4 | `abs 0x7b5a0` | `-` |
| `str` at 0x7b534 | store | 4 | `sp -0x24` | `0xfffff703` |
| `ldrb` at 0x7b54c | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x7b558 | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x7b55c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x7b56c | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x7b574 | load | 1 | `sp -0x1e` | `-` |
| `str` at 0x7b588 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x7b58c | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x7b594 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7b594 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7b594 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7b4a0 | `(unknown callee) 0x3e3ac` | `?` | `?` | `[[sp-28]+8]` | `?` |
| `bl` at 0x7b4c8 | `(unknown callee) 0x41d74` | `[sp-20]` | `[sp-29]:1+550` | `[sp-30]:1` | `[sp-29]:1+550` |
| `bl` at 0x7b4e4 | `(unknown callee) 0x2624c` | `[sp-29]:1+170` | `[sp-30]:1` | `[sp-30]:1` | `[sp-29]:1+170` |
| `bl` at 0x7b53c | `(unknown callee) 0x3e3ac` | `r3` | `r1` | `([sp-22]:1-1 & 0xff)` | `[[sp-28]+8]` |
| `bl` at 0x7b564 | `(unknown callee) 0x41d74` | `[sp-20]` | `[sp-22]:1+550` | `[sp-30]:1` | `[sp-22]:1+550` |
| `bl` at 0x7b580 | `(unknown callee) 0x2624c` | `[sp-22]:1+170` | `[sp-30]:1` | `[sp-30]:1` | `[sp-22]:1+170` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x7b3f8 | `r0 >= [0x1a6220] (signed, 32-bit)` | `r0 < [0x1a6220] (signed, 32-bit)` |
| `bne` at 0x7b424 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x7b430 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x7b480 | `[sp-22]:1 != 0xff` | `[sp-22]:1 == 0xff` |
| `bge` at 0x7b49c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `ble` at 0x7b4ac | `ret(0x3e3ac) <= 0x0 (signed, 32-bit)` | `ret(0x3e3ac) > 0x0 (signed, 32-bit)` |
| `bne` at 0x7b500 | `[sp-22]:1 != 0x0` | `[sp-22]:1 == 0x0` |
| `ble` at 0x7b52c | `([sp-22]:1-1 & 0xff) <= [[sp-28]+8] (signed, 32-bit)` | `([sp-22]:1-1 & 0xff) > [[sp-28]+8] (signed, 32-bit)` |
| `ble` at 0x7b548 | `ret(0x3e3ac) <= 0x0 (signed, 32-bit)` | `ret(0x3e3ac) > 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7b594 | `r0` | `?` |

## `pcixBusPresent`

- `pcixBusPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7b5a4 to 0x7b654.
  Established. A word load reads absolute address 0x1a6220 (`ldr` at 0x7b5c8).
  The branch takes 0x7b644 when entry r0 is greater than or equal to that loaded word using signed 32-bit operands, and falls through when it is less (`ldr` at 0x7b5c8, `bge` at 0x7b5d0).
  A byte store writes one to an unknown effective address (`strb` at 0x7b640).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7b5b4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x7b5bc | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x7b5c0 | load | 4 | `abs 0x7b64c` | `-` |
| `ldr` at 0x7b5c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7b5c8 | load | 4 | `abs 0x1a6220` | `-` |
| `ldr` at 0x7b5d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7b5e8 | load | 4 | `abs 0x7b650` | `-` |
| `ldr` at 0x7b5ec | load | 4 | `abs 0x1a621c` | `-` |
| `ldr` at 0x7b5f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7b604 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7b618 | load | 4 | `abs 0x7b650` | `-` |
| `ldr` at 0x7b61c | load | 4 | `abs 0x1a621c` | `-` |
| `str` at 0x7b624 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x7b628 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7b62c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7b634 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x7b640 | store | 1 | `(unknown)` | `0x1` |
| `ldm` at 0x7b648 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7b648 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7b648 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x7b5d0 | `r0 >= [0x1a6220] (signed, 32-bit)` | `r0 < [0x1a6220] (signed, 32-bit)` |
| `bne` at 0x7b5fc | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7b648 | `r0` | `r0` |

## `pcixBusNotPresent`

- `pcixBusNotPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7b654 to 0x7b704.
  Established. The low byte of entry r1 is saved at sp-0x15 (`strb` at 0x7b66c).
  A word load reads [sp-28]+0x14, followed by a byte store of zero to an unknown effective address (`ldr` at 0x7b6e4, `strb` at 0x7b6f0).
  The return record retains entry r0 (`ldm` at 0x7b6f8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7b664 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x7b66c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x7b670 | load | 4 | `abs 0x7b6fc` | `-` |
| `ldr` at 0x7b674 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7b678 | load | 4 | `abs 0x1a6220` | `-` |
| `ldr` at 0x7b684 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7b698 | load | 4 | `abs 0x7b700` | `-` |
| `ldr` at 0x7b69c | load | 4 | `abs 0x1a621c` | `-` |
| `ldr` at 0x7b6a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7b6b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7b6c8 | load | 4 | `abs 0x7b700` | `-` |
| `ldr` at 0x7b6cc | load | 4 | `abs 0x1a621c` | `-` |
| `str` at 0x7b6d4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x7b6d8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7b6dc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7b6e4 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x7b6f0 | store | 1 | `(unknown)` | `0x0` |
| `ldm` at 0x7b6f8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7b6f8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7b6f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x7b680 | `r0 >= [0x1a6220] (signed, 32-bit)` | `r0 < [0x1a6220] (signed, 32-bit)` |
| `bne` at 0x7b6ac | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7b6f8 | `r0` | `r0` |

## `pcixBusIsPresent`

- `pcixBusIsPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7b704 to 0x7b7f0.
  Established. The branch compares [sp-20] with the word loaded from absolute address 0x1a6220 as signed 32-bit values (`ldr` at 0x7b74c, `bge` at 0x7b754).
  It takes 0x7b784 for greater than or equal, and falls through for less (`bge` at 0x7b754).
  A word store writes zero at sp-0x20 (`str` at 0x7b788).
  A byte load has an unknown effective address, and a word store at sp-0x20 has an unknown value (`ldrb` at 0x7b7c8, `str` at 0x7b7d8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7b714 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x7b71c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x7b720 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7b734 | load | 4 | `abs 0x7b7e8` | `-` |
| `ldr` at 0x7b738 | load | 4 | `abs 0x1a621c` | `-` |
| `str` at 0x7b740 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x7b744 | load | 4 | `abs 0x7b7ec` | `-` |
| `ldr` at 0x7b748 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7b74c | load | 4 | `abs 0x1a6220` | `-` |
| `ldr` at 0x7b758 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7b76c | load | 4 | `abs 0x7b7e8` | `-` |
| `ldr` at 0x7b770 | load | 4 | `abs 0x1a621c` | `-` |
| `ldr` at 0x7b778 | load | 4 | `(unknown)` | `-` |
| `str` at 0x7b788 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x7b790 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7b7a4 | load | 4 | `abs 0x7b7e8` | `-` |
| `ldr` at 0x7b7a8 | load | 4 | `abs 0x1a621c` | `-` |
| `str` at 0x7b7b0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x7b7b4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7b7b8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7b7c0 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0x7b7c8 | load | 1 | `(unknown)` | `-` |
| `str` at 0x7b7d8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x7b7dc | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x7b7e4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7b7e4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7b7e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x7b754 | `[sp-20] >= [0x1a6220] (signed, 32-bit)` | `[sp-20] < [0x1a6220] (signed, 32-bit)` |
| `bne` at 0x7b780 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7b7e4 | `r0` | `?` |

## `pcixSlotIsPresent`

- `pcixSlotIsPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7b7f0 to 0x7b8f0.
  Established. Entry r0 is saved as a word at sp-0x14, and the low byte of entry r1 at sp-0x15 (`str` at 0x7b800, `strb` at 0x7b808).
  Word loads read [sp-28]+0xc and [sp-28]+0x14 (`ldr` at 0x7b8b0, `ldr` at 0x7b8c0).
  The intervening and subsequent byte loads both have unknown effective addresses (`ldrb` at 0x7b8b8, `ldrb` at 0x7b8c8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7b800 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x7b808 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x7b80c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7b820 | load | 4 | `abs 0x7b8e8` | `-` |
| `ldr` at 0x7b824 | load | 4 | `abs 0x1a621c` | `-` |
| `str` at 0x7b82c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x7b830 | load | 4 | `abs 0x7b8ec` | `-` |
| `ldr` at 0x7b834 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7b838 | load | 4 | `abs 0x1a6220` | `-` |
| `ldr` at 0x7b844 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7b858 | load | 4 | `abs 0x7b8e8` | `-` |
| `ldr` at 0x7b85c | load | 4 | `abs 0x1a621c` | `-` |
| `ldr` at 0x7b864 | load | 4 | `(unknown)` | `-` |
| `str` at 0x7b874 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x7b87c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7b890 | load | 4 | `abs 0x7b8e8` | `-` |
| `ldr` at 0x7b894 | load | 4 | `abs 0x1a621c` | `-` |
| `str` at 0x7b89c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x7b8a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7b8a4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7b8a8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7b8b0 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldrb` at 0x7b8b8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7b8c0 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0x7b8c8 | load | 1 | `(unknown)` | `-` |
| `str` at 0x7b8d8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x7b8dc | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x7b8e4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7b8e4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7b8e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x7b840 | `[sp-20] >= [0x1a6220] (signed, 32-bit)` | `[sp-20] < [0x1a6220] (signed, 32-bit)` |
| `bne` at 0x7b86c | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7b8e4 | `r0` | `?` |

## `L1FRU_2m_Init`

- `L1FRU_2m_Init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe56fc to 0xe5c14.
  Established. Word stores write 0xe56fc, 0xe6044 and 0xe63e0 at absolute 0x1b0eac, 0x1b0eb0 and 0x1b0eb4 (`str` at 0xe574c, `str` at 0xe5758, `str` at 0xe5764).
  The call to unnamed 0x20310 has its result stored at sp-0x78, then a byte store writes zero at that result (`bl` at 0xe5b40, `str` at 0xe5b48, `strb` at 0xe5b54).
  A later call to unnamed 0x20058 has its result stored at absolute 0x1b0ea0 (`bl` at 0xe5b68, `str` at 0xe5b78).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xe570c | load | 4 | `abs 0xe5b84` | `-` |
| `ldr` at 0xe5710 | load | 4 | `abs 0x1b0eac` | `-` |
| `ldr` at 0xe571c | load | 4 | `abs 0xe5b88` | `-` |
| `str` at 0xe5724 | store | 4 | `abs 0x1b0ea8` | `0x1` |
| `ldr` at 0xe5728 | load | 4 | `abs 0xe5b8c` | `-` |
| `ldr` at 0xe5738 | load | 4 | `abs 0xe5b8c` | `-` |
| `str` at 0xe5740 | store | 4 | `abs 0x31b940` | `0x1` |
| `ldr` at 0xe5744 | load | 4 | `abs 0xe5b84` | `-` |
| `ldr` at 0xe5748 | load | 4 | `abs 0xe5b90` | `-` |
| `str` at 0xe574c | store | 4 | `abs 0x1b0eac` | `0xe56fc` |
| `ldr` at 0xe5750 | load | 4 | `abs 0xe5b94` | `-` |
| `ldr` at 0xe5754 | load | 4 | `abs 0xe5b98` | `-` |
| `str` at 0xe5758 | store | 4 | `abs 0x1b0eb0` | `0xe6044` |
| `ldr` at 0xe575c | load | 4 | `abs 0xe5b9c` | `-` |
| `ldr` at 0xe5760 | load | 4 | `abs 0xe5ba0` | `-` |
| `str` at 0xe5764 | store | 4 | `abs 0x1b0eb4` | `0xe63e0` |
| `ldr` at 0xe5768 | load | 4 | `abs 0xe5ba4` | `-` |
| `ldr` at 0xe576c | load | 4 | `abs 0xe5ba8` | `-` |
| `str` at 0xe5770 | store | 4 | `abs 0x1b0eb8` | `0xe704c` |
| `ldr` at 0xe5774 | load | 4 | `abs 0xe5bac` | `-` |
| `ldr` at 0xe5778 | load | 4 | `abs 0xe5bb0` | `-` |
| `str` at 0xe577c | store | 4 | `abs 0x1b0ec4` | `0xe7084` |
| `ldr` at 0xe5780 | load | 4 | `abs 0xe5bb4` | `-` |
| `ldr` at 0xe5784 | load | 4 | `abs 0xe5bb8` | `-` |
| `str` at 0xe5788 | store | 4 | `abs 0x1b0ec8` | `0xe70e8` |
| `ldr` at 0xe578c | load | 4 | `abs 0xe5bbc` | `-` |
| `ldr` at 0xe5790 | load | 4 | `abs 0xe5bc0` | `-` |
| `str` at 0xe5794 | store | 4 | `abs 0x1b0ebc` | `0xe5f68` |
| `ldr` at 0xe5798 | load | 4 | `abs 0xe5bc4` | `-` |
| `ldr` at 0xe579c | load | 4 | `abs 0xe5bc8` | `-` |
| `str` at 0xe57a0 | store | 4 | `abs 0x1b0ec0` | `0xe6018` |
| `ldr` at 0xe57a8 | load | 4 | `abs 0xe5bcc` | `-` |
| `ldr` at 0xe57ac | load | 4 | `abs 0x1b0ea4` | `-` |
| `ldr` at 0xe57b8 | load | 4 | `abs 0xe5bd0` | `-` |
| `str` at 0xe57bc | store | 4 | `sp -0x80` | `0x184508` |
| `ldr` at 0xe57c4 | load | 4 | `abs 0xe5bd4` | `-` |
| `ldr` at 0xe57d4 | load | 4 | `abs 0xe5bcc` | `-` |
| `ldr` at 0xe57d8 | load | 4 | `abs 0xe5bcc` | `-` |
| `ldr` at 0xe57dc | load | 4 | `abs 0x1b0ea4` | `-` |
| `str` at 0xe57e4 | store | 4 | `abs 0x1b0ea4` | `[0x1b0ea4]+1` |
| `ldr` at 0xe57ec | load | 4 | `abs 0xe5bd8` | `-` |
| `ldr` at 0xe5804 | load | 4 | `abs 0xe5bdc` | `-` |
| `ldr` at 0xe5814 | load | 4 | `abs 0xe5bd8` | `-` |
| `str` at 0xe5824 | store | 4 | `sp -0x74` | `ret(0x150220)` |
| `ldr` at 0xe5828 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xe583c | load | 4 | `abs 0xe5be0` | `-` |
| `ldr` at 0xe5840 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0xe5848 | load | 4 | `abs 0xe5bd8` | `-` |
| `ldr` at 0xe5850 | load | 4 | `abs 0xe5bdc` | `-` |
| `ldr` at 0xe585c | load | 4 | `abs 0xe5bd8` | `-` |
| `str` at 0xe586c | store | 4 | `sp -0x74` | `ret(0x2046c)` |
| `ldr` at 0xe5870 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xe5884 | load | 4 | `abs 0xe5be4` | `-` |
| `ldr` at 0xe5888 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0xe5890 | load | 4 | `abs 0xe5bd8` | `-` |
| `ldr` at 0xe5898 | load | 4 | `abs 0xe5bdc` | `-` |
| `ldr` at 0xe58a4 | load | 4 | `abs 0xe5be8` | `-` |
| `ldr` at 0xe58a8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe58ac | store | 4 | `abs 0x1b0e98` | `[sp-68]` |
| `ldr` at 0xe58b0 | load | 4 | `abs 0xe5bec` | `-` |
| `ldr` at 0xe58b4 | load | 4 | `abs 0xe5be8` | `-` |
| `ldr` at 0xe58b8 | load | 4 | `abs 0x1b0e98` | `-` |
| `str` at 0xe58c4 | store | 4 | `abs 0x1b0e94` | `ret(0x201fc)` |
| `ldr` at 0xe58c8 | load | 4 | `abs 0xe5bec` | `-` |
| `ldr` at 0xe58cc | load | 4 | `abs 0x1b0e94` | `-` |
| `ldr` at 0xe58d8 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xe58e8 | load | 4 | `abs 0xe5bf0` | `-` |
| `ldr` at 0xe58ec | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0xe58f4 | load | 4 | `abs 0xe5bd8` | `-` |
| `ldr` at 0xe58fc | load | 4 | `abs 0xe5bdc` | `-` |
| `ldr` at 0xe5908 | load | 4 | `abs 0xe5bec` | `-` |
| `ldr` at 0xe590c | load | 4 | `abs 0xe5be8` | `-` |
| `ldr` at 0xe5910 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xe5914 | load | 4 | `abs 0x1b0e94` | `-` |
| `ldr` at 0xe5918 | load | 4 | `abs 0x1b0e98` | `-` |
| `ldr` at 0xe5924 | load | 4 | `abs 0xe5be8` | `-` |
| `ldr` at 0xe5928 | load | 4 | `abs 0x1b0e98` | `-` |
| `ldr` at 0xe5934 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xe5944 | load | 4 | `abs 0xe5bf4` | `-` |
| `ldr` at 0xe5948 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0xe5950 | load | 4 | `abs 0xe5bd8` | `-` |
| `ldr` at 0xe5958 | load | 4 | `abs 0xe5bdc` | `-` |
| `ldr` at 0xe5960 | load | 4 | `abs 0xe5bec` | `-` |
| `str` at 0xe5968 | store | 4 | `abs 0x1b0e94` | `0x0` |
| `ldr` at 0xe5970 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xe597c | load | 4 | `abs 0xe5bdc` | `-` |
| `str` at 0xe598c | store | 4 | `sp -0x74` | `ret(0x150220)` |
| `ldr` at 0xe5990 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xe599c | load | 4 | `abs 0xe5bec` | `-` |
| `ldr` at 0xe59a0 | load | 4 | `abs 0x1b0e94` | `-` |
| `ldr` at 0xe59a8 | load | 4 | `abs 0xe5bec` | `-` |
| `str` at 0xe59b0 | store | 4 | `abs 0x1b0e94` | `0x0` |
| `ldr` at 0xe59bc | load | 4 | `abs 0xe5bf8` | `-` |
| `ldr` at 0xe59c0 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0xe59c8 | load | 4 | `abs 0xe5bd8` | `-` |
| `ldr` at 0xe59d0 | load | 4 | `abs 0xe5bdc` | `-` |
| `ldr` at 0xe59dc | load | 4 | `abs 0xe5bdc` | `-` |
| `str` at 0xe59ec | store | 4 | `sp -0x74` | `ret(0x2046c)` |
| `ldr` at 0xe59f0 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xe59fc | load | 4 | `abs 0xe5bec` | `-` |
| `ldr` at 0xe5a00 | load | 4 | `abs 0x1b0e94` | `-` |
| `ldr` at 0xe5a08 | load | 4 | `abs 0xe5bec` | `-` |
| `str` at 0xe5a10 | store | 4 | `abs 0x1b0e94` | `0x0` |
| `ldr` at 0xe5a1c | load | 4 | `abs 0xe5bfc` | `-` |
| `ldr` at 0xe5a20 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0xe5a28 | load | 4 | `abs 0xe5bd8` | `-` |
| `ldr` at 0xe5a30 | load | 4 | `abs 0xe5bdc` | `-` |
| `ldr` at 0xe5a3c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe5a40 | store | 4 | `sp -0x7c` | `[sp-68]` |
| `ldr` at 0xe5a44 | load | 4 | `sp -0x7c` | `-` |
| `str` at 0xe5a50 | store | 4 | `sp -0x78` | `ret(0x201fc)` |
| `ldr` at 0xe5a54 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xe5a60 | load | 4 | `abs 0xe5bec` | `-` |
| `ldr` at 0xe5a64 | load | 4 | `abs 0x1b0e94` | `-` |
| `ldr` at 0xe5a6c | load | 4 | `abs 0xe5bec` | `-` |
| `str` at 0xe5a74 | store | 4 | `abs 0x1b0e94` | `0x0` |
| `ldr` at 0xe5a78 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xe5a88 | load | 4 | `abs 0xe5c00` | `-` |
| `ldr` at 0xe5a8c | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0xe5a94 | load | 4 | `abs 0xe5bd8` | `-` |
| `ldr` at 0xe5a9c | load | 4 | `abs 0xe5bdc` | `-` |
| `ldr` at 0xe5aa8 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xe5aac | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xe5ab0 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xe5abc | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xe5ac8 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xe5ad0 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xe5ad8 | load | 4 | `abs 0xe5bec` | `-` |
| `ldr` at 0xe5adc | load | 4 | `abs 0x1b0e94` | `-` |
| `ldr` at 0xe5ae4 | load | 4 | `abs 0xe5bec` | `-` |
| `str` at 0xe5aec | store | 4 | `abs 0x1b0e94` | `0x0` |
| `ldr` at 0xe5af8 | load | 4 | `abs 0xe5c04` | `-` |
| `ldr` at 0xe5afc | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0xe5b04 | load | 4 | `abs 0xe5bd8` | `-` |
| `ldr` at 0xe5b0c | load | 4 | `abs 0xe5bdc` | `-` |
| `ldr` at 0xe5b14 | load | 4 | `abs 0xe5bec` | `-` |
| `str` at 0xe5b1c | store | 4 | `abs 0x1b0e94` | `0x0` |
| `ldr` at 0xe5b24 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xe5b2c | load | 4 | `abs 0xe5c08` | `-` |
| `ldr` at 0xe5b30 | load | 4 | `sp -0x78` | `-` |
| `str` at 0xe5b34 | store | 4 | `abs 0x1b0e9c` | `[sp-120]` |
| `ldr` at 0xe5b38 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xe5b3c | load | 4 | `abs 0xe5c0c` | `-` |
| `str` at 0xe5b48 | store | 4 | `sp -0x78` | `ret(0x20310)` |
| `ldr` at 0xe5b4c | load | 4 | `sp -0x78` | `-` |
| `strb` at 0xe5b54 | store | 1 | `ret(0x20310) +0x0` | `0x0` |
| `ldr` at 0xe5b58 | load | 4 | `sp -0x78` | `-` |
| `str` at 0xe5b60 | store | 4 | `sp -0x78` | `[sp-120]+2` |
| `ldr` at 0xe5b64 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xe5b74 | load | 4 | `abs 0xe5c10` | `-` |
| `str` at 0xe5b78 | store | 4 | `abs 0x1b0ea0` | `ret(0x20058)` |
| `ldm` at 0xe5b80 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xe5b80 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe5b80 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe5b80 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe5734 | `(unknown callee) 0x20550` | `0x31b938` | `0x0` | `0x34` | `0x1` |
| `bl` at 0xe57d0 | `(unknown callee) 0x22070` | `0x9` | `0x184528` | `0x5e` | `0x0` |
| `bl` at 0xe57e8 | `(unknown callee) 0x10325c` | `?` | `?` | `0x1b0ea4` | `[0x1b0ea4]+1` |
| `bl` at 0xe57f4 | `(unknown callee) 0x201c0` | `0x184544` | `0x4` | `?` | `?` |
| `bl` at 0xe5808 | `(unknown callee) 0x20100` | `0x184554` | `?` | `?` | `ret(0x201c0)` |
| `bl` at 0xe581c | `(unknown callee) 0x150220` | `0x184544` | `sp-112` | `?` | `sp-112` |
| `bl` at 0xe5834 | `(unknown callee) 0x20184` | `ret(0x150220)` | `?` | `?` | `ret(0x150220)` |
| `bl` at 0xe5844 | `(unknown callee) 0x29908` | `0x184564` | `[ret(0x20184)]` | `?` | `ret(0x20184)` |
| `bl` at 0xe584c | `(unknown callee) 0x20100` | `0x184544` | `?` | `?` | `?` |
| `bl` at 0xe5854 | `(unknown callee) 0x20100` | `0x184554` | `?` | `?` | `?` |
| `bl` at 0xe5864 | `(unknown callee) 0x2046c` | `0x184544` | `0x0` | `?` | `ret(0x150220)` |
| `bl` at 0xe587c | `(unknown callee) 0x20184` | `ret(0x2046c)` | `?` | `?` | `ret(0x2046c)` |
| `bl` at 0xe588c | `(unknown callee) 0x29908` | `0x184598` | `[ret(0x20184)]` | `?` | `ret(0x20184)` |
| `bl` at 0xe5894 | `(unknown callee) 0x20100` | `0x184544` | `?` | `?` | `?` |
| `bl` at 0xe589c | `(unknown callee) 0x20100` | `0x184554` | `?` | `?` | `?` |
| `bl` at 0xe58bc | `(unknown callee) 0x201fc` | `[0x1b0e98]` | `?` | `0x1b0e98` | `0x1b0e98` |
| `bl` at 0xe58dc | `(unknown callee) 0x200b8` | `[sp-116]` | `?` | `?` | `[0x1b0e94]` |
| `bl` at 0xe58e0 | `(unknown callee) 0x20184` | `ret(0x200b8)` | `?` | `?` | `?` |
| `bl` at 0xe58f0 | `(unknown callee) 0x29908` | `0x1845cc` | `[ret(0x20184)]` | `?` | `ret(0x20184)` |
| `bl` at 0xe58f8 | `(unknown callee) 0x20100` | `0x184544` | `?` | `?` | `?` |
| `bl` at 0xe5900 | `(unknown callee) 0x20100` | `0x184554` | `?` | `?` | `?` |
| `bl` at 0xe591c | `(unknown callee) 0x20628` | `[sp-116]` | `[0x1b0e94]` | `[0x1b0e98]` | `0x1b0e94` |
| `bl` at 0xe5938 | `(unknown callee) 0x200b8` | `[sp-116]` | `?` | `ret(0x20628)` | `[0x1b0e98]` |
| `bl` at 0xe593c | `(unknown callee) 0x20184` | `ret(0x200b8)` | `?` | `?` | `?` |
| `bl` at 0xe594c | `(unknown callee) 0x29908` | `0x184608` | `[ret(0x20184)]` | `?` | `ret(0x20184)` |
| `bl` at 0xe5954 | `(unknown callee) 0x20100` | `0x184544` | `?` | `?` | `?` |
| `bl` at 0xe595c | `(unknown callee) 0x20100` | `0x184554` | `?` | `?` | `?` |
| `bl` at 0xe5974 | `(unknown callee) 0x200b8` | `[sp-116]` | `?` | `ret(0x20628)` | `[0x1b0e98]` |
| `bl` at 0xe5984 | `(unknown callee) 0x150220` | `0x184554` | `sp-112` | `?` | `sp-112` |
| `bl` at 0xe59a4 | `(unknown callee) 0x20514` | `[0x1b0e94]` | `?` | `?` | `0x1b0e94` |
| `bl` at 0xe59b4 | `(unknown callee) 0x20184` | `ret(0x20514)` | `?` | `0x1b0e94` | `0x0` |
| `bl` at 0xe59c4 | `(unknown callee) 0x29908` | `0x184638` | `[ret(0x20184)]` | `?` | `ret(0x20184)` |
| `bl` at 0xe59cc | `(unknown callee) 0x20100` | `0x184544` | `?` | `?` | `?` |
| `bl` at 0xe59d4 | `(unknown callee) 0x20100` | `0x184554` | `?` | `?` | `?` |
| `bl` at 0xe59e4 | `(unknown callee) 0x2046c` | `0x184554` | `0x0` | `?` | `ret(0x150220)` |
| `bl` at 0xe5a04 | `(unknown callee) 0x20514` | `[0x1b0e94]` | `?` | `?` | `0x1b0e94` |
| `bl` at 0xe5a14 | `(unknown callee) 0x20184` | `ret(0x20514)` | `?` | `0x1b0e94` | `0x0` |
| `bl` at 0xe5a24 | `(unknown callee) 0x29908` | `0x184670` | `[ret(0x20184)]` | `?` | `ret(0x20184)` |
| `bl` at 0xe5a2c | `(unknown callee) 0x20100` | `0x184544` | `?` | `?` | `?` |
| `bl` at 0xe5a34 | `(unknown callee) 0x20100` | `0x184554` | `?` | `?` | `?` |
| `bl` at 0xe5a48 | `(unknown callee) 0x201fc` | `[sp-68]` | `?` | `?` | `[sp-68]` |
| `bl` at 0xe5a68 | `(unknown callee) 0x20514` | `[0x1b0e94]` | `?` | `?` | `0x1b0e94` |
| `bl` at 0xe5a7c | `(unknown callee) 0x200b8` | `[sp-116]` | `?` | `0x1b0e94` | `0x0` |
| `bl` at 0xe5a80 | `(unknown callee) 0x20184` | `ret(0x200b8)` | `?` | `?` | `?` |
| `bl` at 0xe5a90 | `(unknown callee) 0x29908` | `0x1846a8` | `[ret(0x20184)]` | `?` | `ret(0x20184)` |
| `bl` at 0xe5a98 | `(unknown callee) 0x20100` | `0x184544` | `?` | `?` | `?` |
| `bl` at 0xe5aa0 | `(unknown callee) 0x20100` | `0x184554` | `?` | `?` | `?` |
| `bl` at 0xe5ab4 | `(unknown callee) 0x20628` | `[sp-116]` | `ret(0x201fc)` | `[sp-124]` | `ret(0x201fc)` |
| `bl` at 0xe5acc | `(unknown callee) 0x200b8` | `[sp-116]` | `?` | `ret(0x20628)` | `[sp-124]` |
| `bl` at 0xe5ad4 | `(unknown callee) 0x20514` | `[sp-120]` | `?` | `?` | `?` |
| `bl` at 0xe5ae0 | `(unknown callee) 0x20514` | `[0x1b0e94]` | `?` | `?` | `0x1b0e94` |
| `bl` at 0xe5af0 | `(unknown callee) 0x20184` | `ret(0x20514)` | `?` | `0x1b0e94` | `0x0` |
| `bl` at 0xe5b00 | `(unknown callee) 0x29908` | `0x1846e8` | `[ret(0x20184)]` | `?` | `ret(0x20184)` |
| `bl` at 0xe5b08 | `(unknown callee) 0x20100` | `0x184544` | `?` | `?` | `?` |
| `bl` at 0xe5b10 | `(unknown callee) 0x20100` | `0x184554` | `?` | `?` | `?` |
| `bl` at 0xe5b28 | `(unknown callee) 0x200b8` | `[sp-116]` | `?` | `ret(0x20628)` | `[sp-124]` |
| `bl` at 0xe5b40 | `(unknown callee) 0x20310` | `[sp-120]` | `0x18471c` | `0x1b0e9c` | `[sp-120]` |
| `bl` at 0xe5b68 | `(unknown callee) 0x20058` | `[sp-120]+2` | `?` | `ret(0x20310)` | `[sp-120]+2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe5718 | `[0x1b0eac] != 0x0` | `[0x1b0eac] == 0x0` |
| `beq` at 0xe57b4 | `[0x1b0ea4] == 0x0` | `[0x1b0ea4] != 0x0` |
| `beq` at 0xe5800 | `ret(0x201c0) == 0x0` | `ret(0x201c0) != 0x0` |
| `bge` at 0xe5830 | `ret(0x150220) >= 0x0 (signed, 32-bit)` | `ret(0x150220) < 0x0 (signed, 32-bit)` |
| `bge` at 0xe5878 | `ret(0x2046c) >= 0x0 (signed, 32-bit)` | `ret(0x2046c) < 0x0 (signed, 32-bit)` |
| `bne` at 0xe58d4 | `[0x1b0e94] != 0x0` | `[0x1b0e94] == 0x0` |
| `beq` at 0xe5930 | `ret(0x20628) == [0x1b0e98]` | `ret(0x20628) != [0x1b0e98]` |
| `bge` at 0xe5998 | `ret(0x150220) >= 0x0 (signed, 32-bit)` | `ret(0x150220) < 0x0 (signed, 32-bit)` |
| `bge` at 0xe59f8 | `ret(0x2046c) >= 0x0 (signed, 32-bit)` | `ret(0x2046c) < 0x0 (signed, 32-bit)` |
| `bne` at 0xe5a5c | `ret(0x201fc) != 0x0` | `ret(0x201fc) == 0x0` |
| `beq` at 0xe5ac4 | `ret(0x20628) == [sp-124]` | `ret(0x20628) != [sp-124]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe5b80 | `r0` | `?` |

## `l1fru_status`

- `l1fru_status`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe5c14 to 0xe5f68.
  Established. Zero word stores initialize sp-0x28 and sp-0x2c (`str` at 0xe5c30, `str` at 0xe5c38).
  The call to unnamed 0x201fc has its result stored at the address held in entry r0 (`bl` at 0xe5cd8, `str` at 0xe5ce0).
  The load, add and store sequence near the tail writes [sp-44]+1 at sp-0x2c (`ldr` at 0xe5f2c, `add` at 0xe5f30, `str` at 0xe5f34).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe5c24 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0xe5c28 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0xe5c30 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xe5c38 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe5c3c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5c48 | load | 4 | `abs 0xe5f54` | `-` |
| `ldr` at 0xe5c4c | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe5c58 | load | 4 | `abs 0xe5f54` | `-` |
| `ldr` at 0xe5c5c | load | 4 | `abs 0x31b94c` | `-` |
| `ldr` at 0xe5c60 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe5c6c | load | 4 | `abs 0xe5f54` | `-` |
| `ldr` at 0xe5c70 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe5c84 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe5c8c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe5c90 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe5c98 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe5c9c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe5ca4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe5cac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe5cb0 | load | 4 | `abs 0xe5f54` | `-` |
| `ldr` at 0xe5cb4 | load | 4 | `abs 0x31b94c` | `-` |
| `ldr` at 0xe5cb8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe5ce0 | store | 4 | `r0 +0x0` | `ret(0x201fc)` |
| `ldr` at 0xe5ce4 | load | 4 | `r0 +0x0` | `-` |
| `str` at 0xe5ce8 | store | 4 | `sp -0x24` | `[r0]` |
| `ldr` at 0xe5cec | load | 4 | `abs 0xe5f54` | `-` |
| `ldr` at 0xe5cf0 | load | 4 | `abs 0x31b940` | `-` |
| `ldr` at 0xe5cfc | load | 4 | `abs 0xe5f54` | `-` |
| `ldr` at 0xe5d00 | load | 4 | `abs 0x31b940` | `-` |
| `ldr` at 0xe5d0c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe5d10 | load | 4 | `abs 0xe5f58` | `-` |
| `ldr` at 0xe5d20 | load | 4 | `abs 0xe5f54` | `-` |
| `ldr` at 0xe5d24 | load | 4 | `abs 0x31b940` | `-` |
| `ldr` at 0xe5d30 | load | 4 | `abs 0xe5f54` | `-` |
| `ldr` at 0xe5d34 | load | 4 | `abs 0x31b948` | `-` |
| `ldr` at 0xe5d40 | load | 4 | `abs 0xe5f5c` | `-` |
| `str` at 0xe5d4c | store | 4 | `sp -0x34` | `ret(0x20424)` |
| `ldr` at 0xe5d50 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe5d54 | load | 4 | `abs 0xe5f60` | `-` |
| `ldr` at 0xe5d64 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5d74 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe5d84 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe5d8c | store | 4 | `sp -0x24` | `([sp-36] + ret(0x2031c))` |
| `str` at 0xe5d94 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe5d98 | load | 4 | `abs 0xe5f54` | `-` |
| `ldr` at 0xe5d9c | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe5da8 | load | 4 | `abs 0xe5f54` | `-` |
| `ldr` at 0xe5dac | load | 4 | `abs 0x31b94c` | `-` |
| `ldr` at 0xe5db0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe5dc0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe5dc4 | load | 4 | `abs 0xe5f54` | `-` |
| `ldr` at 0xe5dc8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe5ddc | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe5de4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe5df0 | load | 4 | `abs 0xe5f54` | `-` |
| `ldr` at 0xe5df4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe5e08 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe5e10 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe5e14 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe5e20 | load | 4 | `abs 0xe5f54` | `-` |
| `ldr` at 0xe5e24 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe5e38 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe5e40 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe5e5c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe5e64 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe5e6c | load | 4 | `abs 0xe5f54` | `-` |
| `ldr` at 0xe5e70 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe5e84 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe5e8c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe5ea8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe5eb0 | load | 4 | `abs 0xe5f54` | `-` |
| `ldr` at 0xe5eb4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe5ec8 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe5ed0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe5eec | load | 4 | `(unknown)` | `-` |
| `str` at 0xe5ef4 | store | 4 | `sp -0x34` | `?` |
| `str` at 0xe5ef8 | store | 4 | `sp -0x30` | `ret(0x1073c8)` |
| `ldr` at 0xe5efc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe5f00 | load | 4 | `abs 0xe5f64` | `-` |
| `ldr` at 0xe5f04 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe5f08 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe5f10 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe5f20 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe5f28 | store | 4 | `sp -0x24` | `([sp-36] + ret(0x2031c))` |
| `ldr` at 0xe5f2c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe5f34 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe5f3c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe5f44 | store | 4 | `sp -0x28` | `?` |
| `ldm` at 0xe5f50 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xe5f50 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xe5f50 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe5f50 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe5f50 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe5cd8 | `(unknown callee) 0x201fc` | `?` | `?` | `?` | `?` |
| `bl` at 0xe5d04 | `(unknown callee) 0x1073c8` | `[0x31b940]` | `?` | `?` | `0x31b938` |
| `bl` at 0xe5d18 | `(unknown callee) 0x205bc` | `[sp-36]` | `0x184720` | `ret(0x1073c8)` | `ret(0x1073c8)` |
| `bl` at 0xe5d28 | `(unknown callee) 0x1073c8` | `[0x31b940]` | `?` | `?` | `0x31b938` |
| `bl` at 0xe5d38 | `(unknown callee) 0x1074b4` | `[0x31b948]` | `?` | `?` | `0x31b938` |
| `bl` at 0xe5d44 | `(unknown callee) 0x20424` | `0x31b944` | `?` | `?` | `?` |
| `bl` at 0xe5d60 | `(unknown callee) 0x205bc` | `[sp-36]` | `0x184728` | `ret(0x1073c8)` | `ret(0x1074b4)` |
| `bl` at 0xe5d78 | `(unknown callee) 0x2031c` | `[sp-36]` | `?` | `?` | `[sp-32]` |
| `bl` at 0xe5e68 | `(unknown callee) 0x1073c8` | `?` | `?` | `?` | `?` |
| `bl` at 0xe5f0c | `(unknown callee) 0x205bc` | `[sp-36]` | `0x184740` | `[sp-40]` | `?` |
| `bl` at 0xe5f14 | `(unknown callee) 0x2031c` | `[sp-36]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe5c44 | `r1 == 0x0` | `r1 != 0x0` |
| `beq` at 0xe5c54 | `[0x31b950] == 0x0` | `[0x31b950] != 0x0` |
| `ble` at 0xe5c68 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xe5cf8 | `[0x31b940] != 0x1` | `[0x31b940] == 0x1` |
| `bne` at 0xe5d6c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xe5da4 | `[0x31b950] == 0x0` | `[0x31b950] != 0x0` |
| `ble` at 0xe5db8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xe5dec | `Z == 1` | `not (Z == 1)` |
| `ble` at 0xe5e1c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe5f50 | `r0` | `?` |

## `L1FRU_2m_Status`

- `L1FRU_2m_Status`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe5f68 to 0xe6018.
  Established. The calls to unnamed 0x20058 have printed r0 candidates [0x1b0ea0] at 0xe5fa4 and [0x1b0e9c] at 0xe5fe4 (`bl` at 0xe5fa4, `bl` at 0xe5fe4).
  Word stores record their results at the address held in [sp-20] (`bl` at 0xe5fa4, `str` at 0xe5fb4, `bl` at 0xe5fe4, `str` at 0xe5ff4).
  The printed r1 candidates for the two calls to unnamed 0xe5c14 are 0 at 0xe5fc4 and 1 at 0xe6004 (`bl` at 0xe5fc4, `bl` at 0xe6004).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe5f78 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe5f7c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xe5f80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5f8c | load | 4 | `abs 0xe6010` | `-` |
| `ldr` at 0xe5f90 | load | 4 | `abs 0x1b0ea0` | `-` |
| `ldr` at 0xe5f9c | load | 4 | `abs 0xe6010` | `-` |
| `ldr` at 0xe5fa0 | load | 4 | `abs 0x1b0ea0` | `-` |
| `ldr` at 0xe5fb0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe5fb4 | store | 4 | `[sp-20] +0x0` | `ret(0x20058)` |
| `ldr` at 0xe5fbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5fcc | load | 4 | `abs 0xe6014` | `-` |
| `ldr` at 0xe5fd0 | load | 4 | `abs 0x1b0e9c` | `-` |
| `ldr` at 0xe5fdc | load | 4 | `abs 0xe6014` | `-` |
| `ldr` at 0xe5fe0 | load | 4 | `abs 0x1b0e9c` | `-` |
| `ldr` at 0xe5ff0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe5ff4 | store | 4 | `[sp-20] +0x0` | `ret(0x20058)` |
| `ldr` at 0xe5ffc | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xe600c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe600c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe600c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe5fa4 | `(unknown callee) 0x20058` | `[0x1b0ea0]` | `r1` | `r2` | `0x1b0ea0` |
| `bl` at 0xe5fc4 | `(unknown callee) 0xe5c14` | `r0` | `0x0` | `r2` | `[0x1b0ea0]` |
| `bl` at 0xe5fe4 | `(unknown callee) 0x20058` | `[0x1b0e9c]` | `r1` | `r2` | `0x1b0e9c` |
| `bl` at 0xe6004 | `(unknown callee) 0xe5c14` | `r0` | `0x1` | `r2` | `[0x1b0e9c]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe5f88 | `r1 == 0x0` | `r1 != 0x0` |
| `beq` at 0xe5f98 | `[0x1b0ea0] == 0x0` | `[0x1b0ea0] != 0x0` |
| `beq` at 0xe5fd8 | `[0x1b0e9c] == 0x0` | `[0x1b0e9c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe600c | `r0` | `?` |

## `L1FRU_2m_Ready`

- `L1FRU_2m_Ready`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe6018 to 0xe6044.
  Established. A word load reads absolute address 0x1b0e94 (`ldr` at 0xe6028).
  The following comparison tests r3 against zero, followed by movne and moveq instructions (`cmp` at 0xe602c, `movne` at 0xe6030, `moveq` at 0xe6034).
  The reported return in r0 is unknown after those predicated effects are merged (`movne` at 0xe6030, `moveq` at 0xe6034, `ldm` at 0xe603c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xe6024 | load | 4 | `abs 0xe6040` | `-` |
| `ldr` at 0xe6028 | load | 4 | `abs 0x1b0e94` | `-` |
| `ldm` at 0xe603c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe603c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe603c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe603c | `r0` | `?` |

## `L1FRU_2m_Reset`

- `L1FRU_2m_Reset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe6044 to 0xe63e0.
  Established. Absolute address 0x31b944 receives a zero word (`str` at 0xe605c).
  The printed r0 candidate for the call to unnamed 0x20514 at 0xe6078 is [0x31b954] (`bl` at 0xe6078).
  Later word stores write one at absolute address 0x31b940 and zero at absolute addresses 0x1b0e94 and 0x1b0e98 (`str` at 0xe6334, `str` at 0xe635c, `str` at 0xe6368).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xe6054 | load | 4 | `abs 0xe63c4` | `-` |
| `str` at 0xe605c | store | 4 | `abs 0x31b944` | `0x0` |
| `ldr` at 0xe6060 | load | 4 | `abs 0xe63c4` | `-` |
| `ldr` at 0xe6064 | load | 4 | `abs 0x31b954` | `-` |
| `ldr` at 0xe6070 | load | 4 | `abs 0xe63c4` | `-` |
| `ldr` at 0xe6074 | load | 4 | `abs 0x31b954` | `-` |
| `ldr` at 0xe607c | load | 4 | `abs 0xe63c4` | `-` |
| `str` at 0xe6084 | store | 4 | `abs 0x31b954` | `0x0` |
| `ldr` at 0xe6088 | load | 4 | `abs 0xe63c4` | `-` |
| `ldr` at 0xe608c | load | 4 | `abs 0x31b958` | `-` |
| `ldr` at 0xe6098 | load | 4 | `abs 0xe63c4` | `-` |
| `ldr` at 0xe609c | load | 4 | `abs 0x31b958` | `-` |
| `ldr` at 0xe60a4 | load | 4 | `abs 0xe63c4` | `-` |
| `str` at 0xe60ac | store | 4 | `abs 0x31b958` | `0x0` |
| `ldr` at 0xe60b0 | load | 4 | `abs 0xe63c4` | `-` |
| `ldr` at 0xe60b4 | load | 4 | `abs 0x31b95c` | `-` |
| `ldr` at 0xe60c0 | load | 4 | `abs 0xe63c4` | `-` |
| `ldr` at 0xe60c4 | load | 4 | `abs 0x31b95c` | `-` |
| `ldr` at 0xe60cc | load | 4 | `abs 0xe63c4` | `-` |
| `str` at 0xe60d4 | store | 4 | `abs 0x31b95c` | `0x0` |
| `ldr` at 0xe60d8 | load | 4 | `abs 0xe63c4` | `-` |
| `ldr` at 0xe60dc | load | 4 | `abs 0x31b960` | `-` |
| `ldr` at 0xe60e8 | load | 4 | `abs 0xe63c4` | `-` |
| `ldr` at 0xe60ec | load | 4 | `abs 0x31b960` | `-` |
| `ldr` at 0xe60f4 | load | 4 | `abs 0xe63c4` | `-` |
| `str` at 0xe60fc | store | 4 | `abs 0x31b960` | `0x0` |
| `ldr` at 0xe6100 | load | 4 | `abs 0xe63c4` | `-` |
| `ldr` at 0xe6104 | load | 4 | `abs 0x31b964` | `-` |
| `ldr` at 0xe6110 | load | 4 | `abs 0xe63c4` | `-` |
| `ldr` at 0xe6114 | load | 4 | `abs 0x31b964` | `-` |
| `ldr` at 0xe611c | load | 4 | `abs 0xe63c4` | `-` |
| `str` at 0xe6124 | store | 4 | `abs 0x31b964` | `0x0` |
| `ldr` at 0xe6128 | load | 4 | `abs 0xe63c4` | `-` |
| `ldr` at 0xe612c | load | 4 | `abs 0x31b968` | `-` |
| `ldr` at 0xe6138 | load | 4 | `abs 0xe63c4` | `-` |
| `ldr` at 0xe613c | load | 4 | `abs 0x31b968` | `-` |
| `ldr` at 0xe6144 | load | 4 | `abs 0xe63c4` | `-` |
| `str` at 0xe614c | store | 4 | `abs 0x31b968` | `0x0` |
| `str` at 0xe6154 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0xe6158 | load | 4 | `abs 0xe63c4` | `-` |
| `ldr` at 0xe615c | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe6168 | load | 4 | `abs 0xe63c4` | `-` |
| `ldr` at 0xe616c | load | 4 | `abs 0x31b94c` | `-` |
| `ldr` at 0xe6170 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe6180 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xe6184 | load | 4 | `abs 0xe63c4` | `-` |
| `ldr` at 0xe6188 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe619c | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe61a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe61b0 | load | 4 | `abs 0xe63c4` | `-` |
| `ldr` at 0xe61b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe61c8 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe61d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe61d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe61e0 | load | 4 | `abs 0xe63c4` | `-` |
| `ldr` at 0xe61e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe61f8 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe6200 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe621c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe6224 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe6230 | load | 4 | `abs 0xe63c4` | `-` |
| `ldr` at 0xe6234 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6248 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe6250 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe626c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe6274 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe627c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6284 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xe628c | load | 4 | `abs 0xe63c4` | `-` |
| `ldr` at 0xe6290 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe62a4 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe62ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe62b8 | load | 4 | `abs 0xe63c4` | `-` |
| `ldr` at 0xe62bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe62d0 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe62d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe62e0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe62e8 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xe62f0 | load | 4 | `abs 0xe63c4` | `-` |
| `ldr` at 0xe62f4 | load | 4 | `abs 0xe63c4` | `-` |
| `ldr` at 0xe62f8 | load | 4 | `abs 0x31b94c` | `-` |
| `ldr` at 0xe630c | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe631c | load | 4 | `abs 0xe63c8` | `-` |
| `ldr` at 0xe6324 | load | 4 | `abs 0xe63cc` | `-` |
| `ldr` at 0xe632c | load | 4 | `abs 0xe63c4` | `-` |
| `str` at 0xe6334 | store | 4 | `abs 0x31b940` | `0x1` |
| `ldr` at 0xe6338 | load | 4 | `abs 0xe63d0` | `-` |
| `ldr` at 0xe633c | load | 4 | `abs 0x1b0e94` | `-` |
| `ldr` at 0xe6348 | load | 4 | `abs 0xe63d0` | `-` |
| `ldr` at 0xe634c | load | 4 | `abs 0x1b0e94` | `-` |
| `ldr` at 0xe6354 | load | 4 | `abs 0xe63d0` | `-` |
| `str` at 0xe635c | store | 4 | `abs 0x1b0e94` | `0x0` |
| `ldr` at 0xe6360 | load | 4 | `abs 0xe63d4` | `-` |
| `str` at 0xe6368 | store | 4 | `abs 0x1b0e98` | `0x0` |
| `ldr` at 0xe636c | load | 4 | `abs 0xe63d8` | `-` |
| `ldr` at 0xe6370 | load | 4 | `abs 0x1b0e9c` | `-` |
| `ldr` at 0xe637c | load | 4 | `abs 0xe63d8` | `-` |
| `ldr` at 0xe6380 | load | 4 | `abs 0x1b0e9c` | `-` |
| `ldr` at 0xe6388 | load | 4 | `abs 0xe63d8` | `-` |
| `str` at 0xe6390 | store | 4 | `abs 0x1b0e9c` | `0x0` |
| `ldr` at 0xe6394 | load | 4 | `abs 0xe63dc` | `-` |
| `ldr` at 0xe6398 | load | 4 | `abs 0x1b0ea0` | `-` |
| `ldr` at 0xe63a4 | load | 4 | `abs 0xe63dc` | `-` |
| `ldr` at 0xe63a8 | load | 4 | `abs 0x1b0ea0` | `-` |
| `ldr` at 0xe63b0 | load | 4 | `abs 0xe63dc` | `-` |
| `str` at 0xe63b8 | store | 4 | `abs 0x1b0ea0` | `0x0` |
| `ldm` at 0xe63c0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe63c0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe63c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe6078 | `(unknown callee) 0x20514` | `[0x31b954]` | `r1` | `0x31b938` | `0x31b938` |
| `bl` at 0xe60a0 | `(unknown callee) 0x20514` | `[0x31b958]` | `?` | `0x31b938` | `0x31b938` |
| `bl` at 0xe60c8 | `(unknown callee) 0x20514` | `[0x31b95c]` | `?` | `0x31b938` | `0x31b938` |
| `bl` at 0xe60f0 | `(unknown callee) 0x20514` | `[0x31b960]` | `?` | `0x31b938` | `0x31b938` |
| `bl` at 0xe6118 | `(unknown callee) 0x20514` | `[0x31b964]` | `?` | `0x31b938` | `0x31b938` |
| `bl` at 0xe6140 | `(unknown callee) 0x20514` | `[0x31b968]` | `?` | `0x31b938` | `0x31b938` |
| `bl` at 0xe6278 | `(unknown callee) 0x20514` | `?` | `?` | `?` | `?` |
| `bl` at 0xe62dc | `(unknown callee) 0x20514` | `?` | `0x31b938` | `?` | `?` |
| `bl` at 0xe6318 | `(unknown callee) 0x20550` | `[0x31b950]` | `0x0` | `?` | `?` |
| `bl` at 0xe6320 | `(unknown callee) 0x20100` | `0x184544` | `?` | `?` | `?` |
| `bl` at 0xe6328 | `(unknown callee) 0x20100` | `0x184554` | `?` | `?` | `?` |
| `bl` at 0xe6350 | `(unknown callee) 0x20514` | `[0x1b0e94]` | `?` | `0x31b938` | `0x1b0e94` |
| `bl` at 0xe6384 | `(unknown callee) 0x20514` | `[0x1b0e9c]` | `?` | `0x1b0e98` | `0x1b0e9c` |
| `bl` at 0xe63ac | `(unknown callee) 0x20514` | `[0x1b0ea0]` | `?` | `0x1b0e9c` | `0x1b0ea0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe606c | `[0x31b954] == 0x0` | `[0x31b954] != 0x0` |
| `beq` at 0xe6094 | `[0x31b958] == 0x0` | `[0x31b958] != 0x0` |
| `beq` at 0xe60bc | `[0x31b95c] == 0x0` | `[0x31b95c] != 0x0` |
| `beq` at 0xe60e4 | `[0x31b960] == 0x0` | `[0x31b960] != 0x0` |
| `beq` at 0xe610c | `[0x31b964] == 0x0` | `[0x31b964] != 0x0` |
| `beq` at 0xe6134 | `[0x31b968] == 0x0` | `[0x31b968] != 0x0` |
| `beq` at 0xe6164 | `[0x31b950] == 0x0` | `[0x31b950] != 0x0` |
| `ble` at 0xe6178 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xe61ac | `Z == 1` | `not (Z == 1)` |
| `ble` at 0xe61dc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xe622c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xe62b4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xe6344 | `[0x1b0e94] == 0x0` | `[0x1b0e94] != 0x0` |
| `beq` at 0xe6378 | `[0x1b0e9c] == 0x0` | `[0x1b0e9c] != 0x0` |
| `beq` at 0xe63a0 | `[0x1b0ea0] == 0x0` | `[0x1b0ea0] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe63c0 | `r0` | `?` |

## `L1FRU_2m_Capture`

- `L1FRU_2m_Capture`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe63e0 to 0xe704c.
  Established. The call to unnamed 0x6e140 has its result stored at absolute 0x31b94c (`bl` at 0xe6410, `str` at 0xe6414).
  A call to unnamed 0x201fc records its result at absolute 0x31b950 (`bl` at 0xe6438, `str` at 0xe6440).
  An indirect call uses an unknown target (`mov` at 0xe6c50).
  A following load, add and store writes ([sp-28] + ret(unknown)) at sp-0x1c (`mov` at 0xe6c50, `ldr` at 0xe6c58, `add` at 0xe6c5c, `str` at 0xe6c60).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe63f0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe63f8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xe63fc | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6400 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe640c | load | 4 | `abs 0xe7000` | `-` |
| `str` at 0xe6414 | store | 4 | `abs 0x31b94c` | `ret(0x6e140)` |
| `ldr` at 0xe6418 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe641c | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6420 | load | 4 | `abs 0x31b94c` | `-` |
| `str` at 0xe6440 | store | 4 | `abs 0x31b950` | `ret(0x201fc)` |
| `ldr` at 0xe6444 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6448 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe6454 | load | 4 | `abs 0xe7000` | `-` |
| `str` at 0xe645c | store | 4 | `abs 0x31b940` | `0x7` |
| `str` at 0xe6464 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xe646c | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6470 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6474 | load | 4 | `abs 0x31b94c` | `-` |
| `ldr` at 0xe6488 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe6498 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe649c | load | 4 | `abs 0x31b944` | `-` |
| `str` at 0xe64ac | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xe64b4 | load | 4 | `abs 0xe7000` | `-` |
| `str` at 0xe64bc | store | 4 | `abs 0x31b940` | `0x0` |
| `str` at 0xe64cc | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xe64d0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe64dc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe64e8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe64f4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe6500 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe650c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe651c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe652c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe6538 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe6544 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe6554 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe6564 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6568 | load | 4 | `abs 0x31b950` | `-` |
| `str` at 0xe6578 | store | 4 | `[0x31b950] +0x14` | `ret(0x201fc)` |
| `ldr` at 0xe6594 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe659c | store | 4 | `sp -0x1c` | `([sp-28] + ret(0xe7110))` |
| `ldr` at 0xe65a4 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe65a8 | load | 4 | `abs 0x31b950` | `-` |
| `str` at 0xe65b8 | store | 4 | `[0x31b950] +0x14` | `ret(0x201fc)` |
| `ldr` at 0xe65d4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe65dc | store | 4 | `sp -0x1c` | `([sp-28] + ret(0xe7110))` |
| `ldr` at 0xe65e4 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe65e8 | load | 4 | `abs 0x31b950` | `-` |
| `str` at 0xe65f8 | store | 4 | `[0x31b950] +0x14` | `ret(0x201fc)` |
| `ldr` at 0xe6614 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe661c | store | 4 | `sp -0x1c` | `([sp-28] + ret(0xe7110))` |
| `ldr` at 0xe6638 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe6640 | store | 4 | `sp -0x1c` | `([sp-28] + ret(0xe784c))` |
| `ldr` at 0xe6648 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe664c | load | 4 | `abs 0x31b950` | `-` |
| `str` at 0xe665c | store | 4 | `[0x31b950] +0x14` | `ret(0x201fc)` |
| `ldr` at 0xe6678 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe6680 | store | 4 | `sp -0x1c` | `([sp-28] + ret(0xe784c))` |
| `str` at 0xe668c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe6690 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe669c | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe66a0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe66b4 | load | 4 | `abs 0x31b950` | `-` |
| `str` at 0xe66c8 | store | 4 | `(unknown)` | `ret(0x201fc)` |
| `ldr` at 0xe66cc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe66dc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe66e4 | store | 4 | `sp -0x1c` | `([sp-28] + ret(0xe784c))` |
| `ldr` at 0xe66e8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe66f0 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `str` at 0xe66fc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe6700 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6704 | load | 4 | `abs 0x31b94c` | `-` |
| `ldr` at 0xe6708 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe6714 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe672c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe6734 | store | 4 | `sp -0x40` | `ret(0x3deec)` |
| `ldr` at 0xe6738 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe6744 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe6750 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe675c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe6768 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe6774 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe6784 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe6790 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe67a0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe67ac | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe67b8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe67c4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe67d4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe67e0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe67f0 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe67f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe6808 | load | 4 | `abs 0x31b950` | `-` |
| `str` at 0xe681c | store | 4 | `(unknown)` | `ret(0x201fc)` |
| `ldr` at 0xe6820 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe6830 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe6838 | store | 4 | `sp -0x1c` | `([sp-28] + ret(0xe7fc8))` |
| `ldr` at 0xe6840 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6844 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe6858 | load | 4 | `abs 0x31b950` | `-` |
| `str` at 0xe686c | store | 4 | `(unknown)` | `ret(0x201fc)` |
| `ldr` at 0xe6870 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe6880 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe6888 | store | 4 | `sp -0x1c` | `([sp-28] + ret(0xe7fc8))` |
| `ldr` at 0xe6890 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6894 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe68a8 | load | 4 | `abs 0x31b950` | `-` |
| `str` at 0xe68bc | store | 4 | `(unknown)` | `ret(0x201fc)` |
| `ldr` at 0xe68c0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe68d0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe68d8 | store | 4 | `sp -0x1c` | `([sp-28] + ret(0xe784c))` |
| `ldr` at 0xe68e0 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe68e4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe68f8 | load | 4 | `abs 0x31b950` | `-` |
| `str` at 0xe690c | store | 4 | `(unknown)` | `ret(0x201fc)` |
| `ldr` at 0xe6910 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe6920 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe6928 | store | 4 | `sp -0x1c` | `([sp-28] + ret(0xe784c))` |
| `ldr` at 0xe6930 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6934 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe6948 | load | 4 | `abs 0x31b950` | `-` |
| `str` at 0xe695c | store | 4 | `(unknown)` | `ret(0x201fc)` |
| `ldr` at 0xe6960 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe6970 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe6978 | store | 4 | `sp -0x1c` | `([sp-28] + ret(0xe8744))` |
| `ldr` at 0xe6980 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6984 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe6998 | load | 4 | `abs 0x31b950` | `-` |
| `str` at 0xe69ac | store | 4 | `(unknown)` | `ret(0x201fc)` |
| `ldr` at 0xe69b0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe69c0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe69c8 | store | 4 | `sp -0x1c` | `([sp-28] + ret(0xe8744))` |
| `ldr` at 0xe69d0 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe69d4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe69e8 | load | 4 | `abs 0x31b950` | `-` |
| `str` at 0xe69fc | store | 4 | `(unknown)` | `ret(0x201fc)` |
| `ldr` at 0xe6a00 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe6a10 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe6a18 | store | 4 | `sp -0x1c` | `([sp-28] + ret(0xe8744))` |
| `ldr` at 0xe6a20 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6a24 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe6a38 | load | 4 | `abs 0x31b950` | `-` |
| `str` at 0xe6a4c | store | 4 | `(unknown)` | `ret(0x201fc)` |
| `ldr` at 0xe6a50 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe6a60 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe6a68 | store | 4 | `sp -0x1c` | `([sp-28] + ret(0xe8744))` |
| `ldr` at 0xe6a6c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe6a74 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xe6a7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe6a88 | load | 4 | `abs 0xe7000` | `-` |
| `str` at 0xe6a90 | store | 4 | `abs 0x31b940` | `0x2` |
| `str` at 0xe6a98 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xe6aa0 | load | 4 | `abs 0xe7000` | `-` |
| `str` at 0xe6aac | store | 4 | `abs 0x31b944` | `ret(0x202c8)` |
| `ldr` at 0xe6ab0 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6ab4 | load | 4 | `abs 0xe7004` | `-` |
| `ldr` at 0xe6ab8 | load | 4 | `abs 0x31b944` | `-` |
| `ldr` at 0xe6abc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6ac4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6ad0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6ae4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6afc | load | 4 | `abs 0xe7000` | `-` |
| `str` at 0xe6b04 | store | 4 | `abs 0x31b948` | `0x1` |
| `ldr` at 0xe6b0c | load | 4 | `abs 0xe7000` | `-` |
| `str` at 0xe6b14 | store | 4 | `abs 0x31b948` | `0x2` |
| `ldr` at 0xe6b1c | load | 4 | `abs 0xe7000` | `-` |
| `str` at 0xe6b24 | store | 4 | `abs 0x31b948` | `0x3` |
| `str` at 0xe6b2c | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xe6b34 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe6b38 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6b3c | load | 4 | `abs 0x31b94c` | `-` |
| `ldr` at 0xe6b40 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe6b5c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe6b78 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xe6b7c | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6b80 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe6b94 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe6b9c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe6ba0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe6bac | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6bb0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe6bc4 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe6bcc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe6be8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe6bf0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe6bfc | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6c00 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe6c14 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe6c1c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe6c38 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe6c40 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe6c44 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe6c48 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe6c58 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe6c60 | store | 4 | `sp -0x1c` | `([sp-28] + ret(unknown))` |
| `ldr` at 0xe6c64 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe6c6c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe6c74 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe6c7c | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xe6c84 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6c88 | load | 4 | `abs 0x31b954` | `-` |
| `ldr` at 0xe6c94 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6c98 | load | 4 | `abs 0x31b954` | `-` |
| `ldr` at 0xe6ca0 | load | 4 | `abs 0xe7000` | `-` |
| `str` at 0xe6ca8 | store | 4 | `abs 0x31b954` | `0x0` |
| `ldr` at 0xe6cac | load | 4 | `abs 0xe7008` | `-` |
| `ldr` at 0xe6cb8 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6cbc | load | 4 | `abs 0x31b958` | `-` |
| `ldr` at 0xe6cc8 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6ccc | load | 4 | `abs 0x31b958` | `-` |
| `ldr` at 0xe6cd4 | load | 4 | `abs 0xe7000` | `-` |
| `str` at 0xe6cdc | store | 4 | `abs 0x31b958` | `0x0` |
| `ldr` at 0xe6ce4 | load | 4 | `abs 0xe700c` | `-` |
| `ldr` at 0xe6cf4 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6cf8 | load | 4 | `abs 0x31b95c` | `-` |
| `ldr` at 0xe6d04 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6d08 | load | 4 | `abs 0x31b95c` | `-` |
| `ldr` at 0xe6d10 | load | 4 | `abs 0xe7000` | `-` |
| `str` at 0xe6d18 | store | 4 | `abs 0x31b95c` | `0x0` |
| `ldr` at 0xe6d1c | load | 4 | `abs 0xe7010` | `-` |
| `ldr` at 0xe6d28 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6d2c | load | 4 | `abs 0x31b960` | `-` |
| `ldr` at 0xe6d38 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6d3c | load | 4 | `abs 0x31b960` | `-` |
| `ldr` at 0xe6d44 | load | 4 | `abs 0xe7000` | `-` |
| `str` at 0xe6d4c | store | 4 | `abs 0x31b960` | `0x0` |
| `ldr` at 0xe6d54 | load | 4 | `abs 0xe7014` | `-` |
| `ldr` at 0xe6d60 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6d64 | load | 4 | `abs 0x31b964` | `-` |
| `ldr` at 0xe6d70 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6d74 | load | 4 | `abs 0x31b964` | `-` |
| `ldr` at 0xe6d7c | load | 4 | `abs 0xe7000` | `-` |
| `str` at 0xe6d84 | store | 4 | `abs 0x31b964` | `0x0` |
| `ldr` at 0xe6d88 | load | 4 | `abs 0xe7018` | `-` |
| `ldr` at 0xe6d94 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6d98 | load | 4 | `abs 0x31b968` | `-` |
| `ldr` at 0xe6da4 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6da8 | load | 4 | `abs 0x31b968` | `-` |
| `ldr` at 0xe6db0 | load | 4 | `abs 0xe7000` | `-` |
| `str` at 0xe6db8 | store | 4 | `abs 0x31b968` | `0x0` |
| `ldr` at 0xe6dbc | load | 4 | `abs 0xe701c` | `-` |
| `ldr` at 0xe6dc4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe6dd0 | load | 4 | `abs 0xe7000` | `-` |
| `str` at 0xe6dd8 | store | 4 | `abs 0x31b940` | `0x3` |
| `str` at 0xe6de4 | store | 4 | `sp -0x2c` | `ret(0x28230)` |
| `str` at 0xe6df0 | store | 4 | `sp -0x30` | `ret(0x28230)` |
| `str` at 0xe6e00 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe6e04 | load | 4 | `abs 0xe7020` | `-` |
| `ldr` at 0xe6e10 | load | 4 | `abs 0xe7024` | `-` |
| `ldr` at 0xe6e1c | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6e20 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6e24 | load | 4 | `abs 0x31b944` | `-` |
| `str` at 0xe6e28 | store | 4 | `sp -0x78` | `[0x31b944]` |
| `ldr` at 0xe6e2c | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6e30 | load | 4 | `abs 0x31b948` | `-` |
| `str` at 0xe6e34 | store | 4 | `sp -0x74` | `[0x31b948]` |
| `ldr` at 0xe6e38 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6e3c | load | 4 | `abs 0x31b94c` | `-` |
| `str` at 0xe6e40 | store | 4 | `sp -0x70` | `[0x31b94c]` |
| `ldr` at 0xe6e44 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6e48 | load | 4 | `abs 0x31b950` | `-` |
| `str` at 0xe6e4c | store | 4 | `sp -0x6c` | `[0x31b950]` |
| `ldr` at 0xe6e50 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6e54 | load | 4 | `abs 0x31b954` | `-` |
| `str` at 0xe6e58 | store | 4 | `sp -0x68` | `[0x31b954]` |
| `ldr` at 0xe6e5c | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6e60 | load | 4 | `abs 0x31b958` | `-` |
| `str` at 0xe6e64 | store | 4 | `sp -0x64` | `[0x31b958]` |
| `ldr` at 0xe6e68 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6e6c | load | 4 | `abs 0x31b95c` | `-` |
| `str` at 0xe6e70 | store | 4 | `sp -0x60` | `[0x31b95c]` |
| `ldr` at 0xe6e74 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6e78 | load | 4 | `abs 0x31b960` | `-` |
| `str` at 0xe6e7c | store | 4 | `sp -0x5c` | `[0x31b960]` |
| `ldr` at 0xe6e80 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6e84 | load | 4 | `abs 0x31b964` | `-` |
| `str` at 0xe6e88 | store | 4 | `sp -0x58` | `[0x31b964]` |
| `ldr` at 0xe6e8c | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6e90 | load | 4 | `abs 0x31b968` | `-` |
| `str` at 0xe6e94 | store | 4 | `sp -0x54` | `[0x31b968]` |
| `ldr` at 0xe6e98 | load | 4 | `abs 0xe7028` | `-` |
| `str` at 0xe6e9c | store | 4 | `sp -0x50` | `0x1b0e94` |
| `ldr` at 0xe6ea0 | load | 4 | `abs 0xe702c` | `-` |
| `str` at 0xe6ea4 | store | 4 | `sp -0x4c` | `0x1b0e98` |
| `ldr` at 0xe6ea8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe6eac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe6eb0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe6eb4 | load | 4 | `abs 0x31b940` | `-` |
| `ldr` at 0xe6ebc | load | 4 | `abs 0xe7030` | `-` |
| `ldr` at 0xe6ec0 | load | 4 | `abs 0xe7034` | `-` |
| `ldr` at 0xe6ec4 | load | 4 | `abs 0xe7038` | `-` |
| `str` at 0xe6ed0 | store | 4 | `sp -0x38` | `ret(0x2046c)` |
| `ldr` at 0xe6ed4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe6ee0 | load | 4 | `abs 0xe7028` | `-` |
| `ldr` at 0xe6ee4 | load | 4 | `abs 0xe702c` | `-` |
| `ldr` at 0xe6ee8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe6eec | load | 4 | `abs 0x1b0e94` | `-` |
| `ldr` at 0xe6ef0 | load | 4 | `abs 0x1b0e98` | `-` |
| `ldr` at 0xe6ef8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe6f10 | load | 4 | `abs 0xe702c` | `-` |
| `ldr` at 0xe6f14 | load | 4 | `abs 0xe703c` | `-` |
| `ldr` at 0xe6f18 | load | 4 | `abs 0xe7030` | `-` |
| `ldr` at 0xe6f1c | load | 4 | `abs 0x1b0e98` | `-` |
| `ldr` at 0xe6f20 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0xe6f28 | load | 4 | `abs 0xe7040` | `-` |
| `ldr` at 0xe6f2c | load | 4 | `abs 0xe7034` | `-` |
| `ldr` at 0xe6f30 | load | 4 | `abs 0xe7038` | `-` |
| `str` at 0xe6f3c | store | 4 | `sp -0x38` | `ret(0x2046c)` |
| `ldr` at 0xe6f40 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe6f4c | load | 4 | `abs 0xe7020` | `-` |
| `ldr` at 0xe6f50 | load | 4 | `abs 0x1b0e9c` | `-` |
| `ldr` at 0xe6f60 | load | 4 | `abs 0xe7020` | `-` |
| `ldr` at 0xe6f64 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe6f68 | load | 4 | `abs 0x1b0e9c` | `-` |
| `ldr` at 0xe6f70 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe6f74 | load | 4 | `abs 0xe7044` | `-` |
| `ldr` at 0xe6f80 | load | 4 | `abs 0xe7024` | `-` |
| `ldr` at 0xe6f84 | load | 4 | `abs 0x1b0ea0` | `-` |
| `ldr` at 0xe6f94 | load | 4 | `abs 0xe7024` | `-` |
| `ldr` at 0xe6f98 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe6f9c | load | 4 | `abs 0x1b0ea0` | `-` |
| `ldr` at 0xe6fa4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe6fb8 | load | 4 | `abs 0xe7048` | `-` |
| `ldr` at 0xe6fbc | load | 4 | `abs 0xe7040` | `-` |
| `ldr` at 0xe6fc0 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0xe6fc8 | load | 4 | `abs 0xe7000` | `-` |
| `ldr` at 0xe6fcc | load | 4 | `abs 0x31b940` | `-` |
| `str` at 0xe6fdc | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xe6fe8 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xe6fec | load | 4 | `sp -0x48` | `-` |
| `str` at 0xe6ff0 | store | 4 | `sp -0x3c` | `[sp-72]` |
| `ldr` at 0xe6ff4 | load | 4 | `sp -0x3c` | `-` |
| `ldm` at 0xe6ffc | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xe6ffc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe6ffc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe6ffc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe6410 | `(unknown callee) 0x6e140` | `r0` | `r1` | `r2` | `[0x31b950]` |
| `bl` at 0xe6438 | `(unknown callee) 0x201fc` | `?` | `?` | `[0x31b94c]` | `?` |
| `bl` at 0xe6494 | `(unknown callee) 0x20550` | `[0x31b950]` | `0x0` | `?` | `?` |
| `bl` at 0xe64c0 | `(unknown callee) 0x6dc0c` | `?` | `?` | `0x0` | `0x31b938` |
| `bl` at 0xe6570 | `(unknown callee) 0x201fc` | `0x6c` | `?` | `?` | `0x31b938` |
| `bl` at 0xe657c | `(unknown callee) 0x6e190` | `ret(0x201fc)` | `?` | `?` | `ret(0x201fc)` |
| `bl` at 0xe658c | `(unknown callee) 0xe7110` | `ret(0x6e190)` | `0x1` | `?` | `ret(0x6e190)` |
| `bl` at 0xe65b0 | `(unknown callee) 0x201fc` | `0xd8` | `?` | `?` | `0x31b938` |
| `bl` at 0xe65bc | `(unknown callee) 0x6e190` | `ret(0x201fc)` | `?` | `?` | `ret(0x201fc)` |
| `bl` at 0xe65cc | `(unknown callee) 0xe7110` | `ret(0x6e190)` | `0x2` | `?` | `ret(0x6e190)` |
| `bl` at 0xe65f0 | `(unknown callee) 0x201fc` | `0x288` | `?` | `?` | `0x31b938` |
| `bl` at 0xe65fc | `(unknown callee) 0x6e190` | `ret(0x201fc)` | `?` | `?` | `ret(0x201fc)` |
| `bl` at 0xe660c | `(unknown callee) 0xe7110` | `ret(0x6e190)` | `0x4` | `?` | `ret(0x6e190)` |
| `bl` at 0xe6620 | `(unknown callee) 0x6e190` | `ret(0xe7110)` | `?` | `ret(0xe7110)` | `([sp-28] + ret(0xe7110))` |
| `bl` at 0xe6630 | `(unknown callee) 0xe784c` | `ret(0x6e190)` | `0x2` | `?` | `ret(0x6e190)` |
| `bl` at 0xe6654 | `(unknown callee) 0x201fc` | `0x6c` | `?` | `?` | `0x31b938` |
| `bl` at 0xe6660 | `(unknown callee) 0x6e190` | `ret(0x201fc)` | `?` | `?` | `ret(0x201fc)` |
| `bl` at 0xe6670 | `(unknown callee) 0xe784c` | `ret(0x6e190)` | `0x1` | `?` | `ret(0x6e190)` |
| `bl` at 0xe66c0 | `(unknown callee) 0x201fc` | `0x6c` | `0x31b938` | `?` | `[0x31b950]` |
| `bl` at 0xe66d4 | `(unknown callee) 0xe784c` | `[sp-40]` | `0x1` | `?` | `ret(0x201fc)` |
| `bl` at 0xe6718 | `(unknown callee) 0x3ec6c` | `?` | `?` | `[0x31b94c]` | `?` |
| `bl` at 0xe6730 | `(unknown callee) 0x3deec` | `[sp-40]` | `?` | `?` | `ret(0x3ec6c)` |
| `bl` at 0xe6814 | `(unknown callee) 0x201fc` | `0x6c` | `0x31b938` | `?` | `[0x31b950]` |
| `bl` at 0xe6828 | `(unknown callee) 0xe7fc8` | `[sp-40]` | `0x1` | `?` | `ret(0x201fc)` |
| `bl` at 0xe6864 | `(unknown callee) 0x201fc` | `0x6c` | `0x31b938` | `?` | `[0x31b950]` |
| `bl` at 0xe6878 | `(unknown callee) 0xe7fc8` | `[sp-40]` | `0x1` | `?` | `ret(0x201fc)` |
| `bl` at 0xe68b4 | `(unknown callee) 0x201fc` | `0xd8` | `0x31b938` | `?` | `[0x31b950]` |
| `bl` at 0xe68c8 | `(unknown callee) 0xe784c` | `[sp-40]` | `0x2` | `?` | `ret(0x201fc)` |
| `bl` at 0xe6904 | `(unknown callee) 0x201fc` | `0x6c` | `0x31b938` | `?` | `[0x31b950]` |
| `bl` at 0xe6918 | `(unknown callee) 0xe784c` | `[sp-40]` | `0x1` | `?` | `ret(0x201fc)` |
| `bl` at 0xe6954 | `(unknown callee) 0x201fc` | `0x6c` | `0x31b938` | `?` | `[0x31b950]` |
| `bl` at 0xe6968 | `(unknown callee) 0xe8744` | `[sp-40]` | `0x1` | `?` | `ret(0x201fc)` |
| `bl` at 0xe69a4 | `(unknown callee) 0x201fc` | `0x6c` | `0x31b938` | `?` | `[0x31b950]` |
| `bl` at 0xe69b8 | `(unknown callee) 0xe8744` | `[sp-40]` | `0x1` | `?` | `ret(0x201fc)` |
| `bl` at 0xe69f4 | `(unknown callee) 0x201fc` | `0x6c` | `0x31b938` | `?` | `[0x31b950]` |
| `bl` at 0xe6a08 | `(unknown callee) 0xe8744` | `[sp-40]` | `0x1` | `?` | `ret(0x201fc)` |
| `bl` at 0xe6a44 | `(unknown callee) 0x201fc` | `0xd8` | `0x31b938` | `?` | `[0x31b950]` |
| `bl` at 0xe6a58 | `(unknown callee) 0xe8744` | `[sp-40]` | `0x2` | `?` | `ret(0x201fc)` |
| `bl` at 0xe6aa8 | `(unknown callee) 0x202c8` | `0x0` | `?` | `?` | `[sp-28]` |
| `bl` at 0xe6ac0 | `(unknown callee) 0x29908` | `0x184754` | `[0x31b944]` | `[sp-24]` | `0x31b938` |
| `bl` at 0xe6b4c | `(unknown callee) 0x6e140` | `?` | `?` | `[0x31b94c]` | `?` |
| `bl` at 0xe6b60 | `(unknown callee) 0x3ec6c` | `[sp-40]` | `?` | `?` | `ret(0x6e140)` |
| `mov` at 0xe6c50 | `(unknown callee) (unknown target)` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xe6c9c | `(unknown callee) 0x20514` | `[0x31b954]` | `?` | `[0x31b94c]` | `0x31b938` |
| `bl` at 0xe6cb4 | `(unknown callee) 0xa1dac` | `0x31b954` | `0x4` | `0x31b938` | `0x0` |
| `bl` at 0xe6cd0 | `(unknown callee) 0x20514` | `[0x31b958]` | `?` | `?` | `0x31b938` |
| `bl` at 0xe6cf0 | `(unknown callee) 0x5de7c` | `?` | `0x31b958` | `0x0` | `0x0` |
| `bl` at 0xe6d0c | `(unknown callee) 0x20514` | `[0x31b95c]` | `?` | `?` | `0x31b938` |
| `bl` at 0xe6d24 | `(unknown callee) 0xb22f8` | `0x31b95c` | `0x1` | `0x31b938` | `0x0` |
| `bl` at 0xe6d40 | `(unknown callee) 0x20514` | `[0x31b960]` | `?` | `?` | `0x31b938` |
| `bl` at 0xe6d5c | `(unknown callee) 0xcc97c` | `?` | `0x31b960` | `0x1` | `0x0` |
| `bl` at 0xe6d78 | `(unknown callee) 0x20514` | `[0x31b964]` | `?` | `?` | `0x31b938` |
| `bl` at 0xe6d90 | `(unknown callee) 0xf54a8` | `0x31b964` | `0x0` | `0x31b938` | `0x0` |
| `bl` at 0xe6dac | `(unknown callee) 0x20514` | `[0x31b968]` | `?` | `?` | `0x31b938` |
| `bl` at 0xe6dc0 | `(unknown callee) 0xf6770` | `0x31b968` | `?` | `0x0` | `0x31b938` |
| `bl` at 0xe6de0 | `(unknown callee) 0x28230` | `0x1` | `?` | `?` | `?` |
| `bl` at 0xe6dec | `(unknown callee) 0x28230` | `0x2` | `?` | `?` | `?` |
| `bl` at 0xe6df4 | `(unknown callee) 0x6dc0c` | `ret(0x28230)` | `?` | `?` | `?` |
| `bl` at 0xe6e0c | `(unknown callee) 0xe5c14` | `0x1b0e9c` | `0x0` | `?` | `?` |
| `bl` at 0xe6e18 | `(unknown callee) 0xe5c14` | `0x1b0ea0` | `0x1` | `?` | `?` |
| `bl` at 0xe6eb8 | `(unknown callee) 0x1032a0` | `[sp-52]` | `[sp-44]` | `[sp-48]` | `[0x31b940]` |
| `bl` at 0xe6ec8 | `(unknown callee) 0x2046c` | `0x184544` | `0x242` | `0x1b6` | `?` |
| `bl` at 0xe6ef4 | `(unknown callee) 0x2007c` | `[sp-56]` | `[0x1b0e94]` | `[0x1b0e98]` | `0x1b0e94` |
| `bl` at 0xe6efc | `(unknown callee) 0x200b8` | `[sp-56]` | `?` | `?` | `?` |
| `bl` at 0xe6f04 | `(unknown callee) 0x20184` | `ret(0x2046c)` | `?` | `?` | `ret(0x2046c)` |
| `bl` at 0xe6f24 | `(unknown callee) 0x29908` | `0x184760` | `0x184544` | `[0x1b0e98]` | `[ret(0x20184)]` |
| `bl` at 0xe6f34 | `(unknown callee) 0x2046c` | `0x184554` | `0x242` | `0x1b6` | `?` |
| `bl` at 0xe6f54 | `(unknown callee) 0x2031c` | `[0x1b0e9c]` | `?` | `?` | `0x1b0e9c` |
| `bl` at 0xe6f6c | `(unknown callee) 0x2007c` | `[sp-56]` | `[0x1b0e9c]` | `ret(0x2031c)` | `0x1b0e9c` |
| `bl` at 0xe6f7c | `(unknown callee) 0x2007c` | `[sp-56]` | `0x18471c` | `0x2` | `?` |
| `bl` at 0xe6f88 | `(unknown callee) 0x2031c` | `[0x1b0ea0]` | `?` | `?` | `0x1b0ea0` |
| `bl` at 0xe6fa0 | `(unknown callee) 0x2007c` | `[sp-56]` | `[0x1b0ea0]` | `ret(0x2031c)` | `0x1b0ea0` |
| `bl` at 0xe6fa8 | `(unknown callee) 0x200b8` | `[sp-56]` | `?` | `?` | `?` |
| `bl` at 0xe6fb0 | `(unknown callee) 0x20184` | `ret(0x2046c)` | `?` | `?` | `ret(0x2046c)` |
| `bl` at 0xe6fc4 | `(unknown callee) 0x29908` | `0x184798` | `0x184554` | `[ret(0x20184)]` | `ret(0x20184)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe6408 | `[0x31b950] != 0x0` | `[0x31b950] == 0x0` |
| `bne` at 0xe6450 | `[0x31b950] != 0x0` | `[0x31b950] == 0x0` |
| `beq` at 0xe64a4 | `[0x31b944] == 0x0` | `[0x31b944] != 0x0` |
| `beq` at 0xe64d8 | `[sp-68] == 0x40` | `[sp-68] != 0x40` |
| `bgt` at 0xe64e4 | `[sp-68] > 0x40 (signed, 32-bit)` | `[sp-68] <= 0x40 (signed, 32-bit)` |
| `beq` at 0xe64f0 | `[sp-68] == 0x2e` | `[sp-68] != 0x2e` |
| `bgt` at 0xe64fc | `[sp-68] > 0x2e (signed, 32-bit)` | `[sp-68] <= 0x2e (signed, 32-bit)` |
| `beq` at 0xe6508 | `[sp-68] == 0x2c` | `[sp-68] != 0x2c` |
| `beq` at 0xe6514 | `[sp-68] == 0x2d` | `[sp-68] != 0x2d` |
| `beq` at 0xe6524 | `[sp-68] == 0x35` | `[sp-68] != 0x35` |
| `beq` at 0xe6534 | `[sp-68] == 0x4f` | `[sp-68] != 0x4f` |
| `bgt` at 0xe6540 | `[sp-68] > 0x4f (signed, 32-bit)` | `[sp-68] <= 0x4f (signed, 32-bit)` |
| `beq` at 0xe654c | `[sp-68] == 0x44` | `[sp-68] != 0x44` |
| `beq` at 0xe655c | `[sp-68] == 0x7d` | `[sp-68] != 0x7d` |
| `bgt` at 0xe6698 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `ble` at 0xe6710 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xe6724 | `ret(0x3ec6c) != 0x0` | `ret(0x3ec6c) == 0x0` |
| `beq` at 0xe6740 | `ret(0x3deec) == 0x700` | `ret(0x3deec) != 0x700` |
| `bgt` at 0xe674c | `ret(0x3deec) > 0x700 (signed, 32-bit)` | `ret(0x3deec) <= 0x700 (signed, 32-bit)` |
| `beq` at 0xe6758 | `ret(0x3deec) == 0x300` | `ret(0x3deec) != 0x300` |
| `bgt` at 0xe6764 | `ret(0x3deec) > 0x300 (signed, 32-bit)` | `ret(0x3deec) <= 0x300 (signed, 32-bit)` |
| `beq` at 0xe6770 | `ret(0x3deec) == 0x100` | `ret(0x3deec) != 0x100` |
| `beq` at 0xe677c | `ret(0x3deec) == 0x200` | `ret(0x3deec) != 0x200` |
| `beq` at 0xe678c | `ret(0x3deec) == 0x400` | `ret(0x3deec) != 0x400` |
| `beq` at 0xe6798 | `ret(0x3deec) == 0x500` | `ret(0x3deec) != 0x500` |
| `beq` at 0xe67a8 | `ret(0x3deec) == 0xa00` | `ret(0x3deec) != 0xa00` |
| `bgt` at 0xe67b4 | `ret(0x3deec) > 0xa00 (signed, 32-bit)` | `ret(0x3deec) <= 0xa00 (signed, 32-bit)` |
| `beq` at 0xe67c0 | `ret(0x3deec) == 0x800` | `ret(0x3deec) != 0x800` |
| `beq` at 0xe67cc | `ret(0x3deec) == 0x900` | `ret(0x3deec) != 0x900` |
| `beq` at 0xe67dc | `ret(0x3deec) == 0xb00` | `ret(0x3deec) != 0xb00` |
| `beq` at 0xe67e8 | `ret(0x3deec) == 0xc00` | `ret(0x3deec) != 0xc00` |
| `beq` at 0xe6a84 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `blt` at 0xe6acc | `[sp-24] < 0x0 (signed, 32-bit)` | `[sp-24] >= 0x0 (signed, 32-bit)` |
| `bne` at 0xe6ae0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe6af8 | `? != 0x5` | `? == 0x5` |
| `ble` at 0xe6b48 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ble` at 0xe6b58 | `ret(0x6e140) <= 0x1 (signed, 32-bit)` | `ret(0x6e140) > 0x1 (signed, 32-bit)` |
| `bne` at 0xe6b6c | `ret(0x3ec6c) != 0x0` | `ret(0x3ec6c) == 0x0` |
| `ble` at 0xe6ba8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xe6bf8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xe6c90 | `[0x31b954] == 0x0` | `[0x31b954] != 0x0` |
| `beq` at 0xe6cc4 | `[0x31b958] == 0x0` | `[0x31b958] != 0x0` |
| `beq` at 0xe6d00 | `[0x31b95c] == 0x0` | `[0x31b95c] != 0x0` |
| `beq` at 0xe6d34 | `[0x31b960] == 0x0` | `[0x31b960] != 0x0` |
| `beq` at 0xe6d6c | `[0x31b964] == 0x0` | `[0x31b964] != 0x0` |
| `beq` at 0xe6da0 | `[0x31b968] == 0x0` | `[0x31b968] != 0x0` |
| `beq` at 0xe6dcc | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `beq` at 0xe6edc | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xe6f48 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xe6fd4 | `[0x31b940] != 0x0` | `[0x31b940] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe6ffc | `r0` | `[sp-60]` |

## `L1FRU_2m_Dump`

- `L1FRU_2m_Dump`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe704c to 0xe7084.
  Established. Three word stores save entry r0, r1 and r2 at sp-0x14, sp-0x18 and sp-0x1c (`str` at 0xe705c, `str` at 0xe7060, `str` at 0xe7064).
  Printed r0-r3 candidates for the call to unnamed 0x106ff8 are entry r0, entry r1, 0 and entry r2 (`bl` at 0xe7078);
  its result remains the recorded return in r0 (`bl` at 0xe7078, `ldm` at 0xe7080).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe705c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe7060 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe7064 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xe7068 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe706c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7074 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xe7080 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe7080 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe7080 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe7078 | `(unknown callee) 0x106ff8` | `r0` | `r1` | `0x0` | `r2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe7080 | `r0` | `ret(0x106ff8)` |

## `L1FRU_2m_Attach`

- `L1FRU_2m_Attach`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe7084 to 0xe70e8.
  Established. The branch takes 0xe70d0 when the word loaded from absolute address 0x1b0e94 is zero and falls through when it is nonzero (`ldr` at 0xe70a0, `beq` at 0xe70a8).
  Separate word loads supply printed candidates r2=[0x1b0e94] and r3=[0x1b0e98] for the call to unnamed 0x105ca8 (`ldr` at 0xe70bc, `ldr` at 0xe70c0, `bl` at 0xe70c4).
  That call result is stored as a word at sp-0x14 (`bl` at 0xe70c4, `str` at 0xe70cc).
  The return record leaves r0 unknown (`ldm` at 0xe70dc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe7094 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe7098 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xe709c | load | 4 | `abs 0xe70e0` | `-` |
| `ldr` at 0xe70a0 | load | 4 | `abs 0x1b0e94` | `-` |
| `ldr` at 0xe70ac | load | 4 | `abs 0xe70e0` | `-` |
| `ldr` at 0xe70b0 | load | 4 | `abs 0xe70e4` | `-` |
| `ldr` at 0xe70b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe70b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe70bc | load | 4 | `abs 0x1b0e94` | `-` |
| `ldr` at 0xe70c0 | load | 4 | `abs 0x1b0e98` | `-` |
| `str` at 0xe70cc | store | 4 | `sp -0x14` | `ret(0x105ca8)` |
| `ldr` at 0xe70d0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xe70dc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe70dc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe70dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe70c4 | `(unknown callee) 0x105ca8` | `r0` | `r1` | `[0x1b0e94]` | `[0x1b0e98]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe70a8 | `[0x1b0e94] == 0x0` | `[0x1b0e94] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe70dc | `r0` | `?` |

## `L1FRU_2m_Detach`

- `L1FRU_2m_Detach`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe70e8 to 0xe7110.
  Established. A word store saves entry r0 at sp-0x14 (`str` at 0xe70f8).
  Printed r0-r3 candidates for the call to unnamed 0x105d8c are entry r0, entry r1, entry r2 and entry r3 (`bl` at 0xe7100).
  The return record identifies r0 as that call's result (`bl` at 0xe7100, `ldm` at 0xe710c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe70f8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xe70fc | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xe710c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xe710c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe710c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe710c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe7100 | `(unknown callee) 0x105d8c` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe710c | `r0` | `ret(0x105d8c)` |

## `L1FRU_SHUBInit`

- `L1FRU_SHUBInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe7110 to 0xe74c4.
  Established. Word stores with unknown effective addresses write 0x188b08 and 0xe74c4 (`str` at 0xe7324, `str` at 0xe7370).
  Printed candidates for the call to unnamed 0x201fc include r0=[sp-32] (`bl` at 0xe73cc);
  its result is stored as a word at an unknown effective address (`bl` at 0xe73cc, `str` at 0xe73d4).
  Load, add and store sequences write [sp-40]+1 at sp-0x28 and [sp-36]+1 at sp-0x24 (`ldr` at 0xe7480, `add` at 0xe7484, `str` at 0xe7488, `ldr` at 0xe748c, `add` at 0xe7490, `str` at 0xe7494).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe7120 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe7124 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xe7128 | load | 4 | `abs 0xe74b0` | `-` |
| `ldr` at 0xe712c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7140 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe7148 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe714c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe7150 | load | 4 | `abs 0xe74b0` | `-` |
| `ldr` at 0xe7154 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7168 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe7170 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe7180 | store | 4 | `sp -0x2c` | `0x1` |
| `str` at 0xe718c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe7190 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe7194 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe71a0 | load | 4 | `abs 0xe74b0` | `-` |
| `ldr` at 0xe71a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe71b8 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe71c0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe71dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe71e8 | load | 4 | `abs 0xe74b4` | `-` |
| `ldr` at 0xe71ec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe71f4 | load | 4 | `abs 0xe74b0` | `-` |
| `ldr` at 0xe71f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe720c | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe7214 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7230 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe723c | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0xe7240 | load | 4 | `abs 0xe74b0` | `-` |
| `ldr` at 0xe7244 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7258 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe7260 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe727c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe7284 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe7288 | store | 4 | `(unknown)` | `[sp-36]` |
| `ldr` at 0xe728c | load | 4 | `abs 0xe74b0` | `-` |
| `ldr` at 0xe7290 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe72a4 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe72ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe72c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe72d0 | load | 4 | `abs 0xe74b8` | `-` |
| `ldr` at 0xe72d4 | load | 4 | `abs 0x1a04ac` | `-` |
| `str` at 0xe72d8 | store | 4 | `(unknown)` | `[0x1a04ac]` |
| `ldr` at 0xe72dc | load | 4 | `abs 0xe74b0` | `-` |
| `ldr` at 0xe72e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe72f4 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe72fc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7318 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe7320 | load | 4 | `abs 0xe74bc` | `-` |
| `str` at 0xe7324 | store | 4 | `(unknown)` | `0x188b08` |
| `ldr` at 0xe7328 | load | 4 | `abs 0xe74b0` | `-` |
| `ldr` at 0xe732c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7340 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe7348 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7364 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe736c | load | 4 | `abs 0xe74c0` | `-` |
| `str` at 0xe7370 | store | 4 | `(unknown)` | `0xe74c4` |
| `ldr` at 0xe7374 | load | 4 | `abs 0xe74b8` | `-` |
| `ldr` at 0xe7378 | load | 4 | `abs 0x1a04ac` | `-` |
| `str` at 0xe7380 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe7384 | load | 4 | `abs 0xe74b0` | `-` |
| `ldr` at 0xe7388 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe739c | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe73a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe73c0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe73c8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe73d4 | store | 4 | `(unknown)` | `ret(0x201fc)` |
| `ldr` at 0xe73d8 | load | 4 | `abs 0xe74b0` | `-` |
| `ldr` at 0xe73dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe73f0 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe73f8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7414 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe741c | load | 4 | `(unknown)` | `-` |
| `str` at 0xe742c | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xe7434 | load | 4 | `abs 0xe74b0` | `-` |
| `ldr` at 0xe7438 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe744c | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe7454 | load | 4 | `abs 0xe74b0` | `-` |
| `ldr` at 0xe7458 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe746c | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe7474 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe747c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe7480 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe7488 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xe748c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe7494 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `str` at 0xe74a0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe74a4 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0xe74ac | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xe74ac | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe74ac | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe74ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe71f0 | `(unknown callee) 0x205bc` | `?` | `0x1847c4` | `?` | `?` |
| `bl` at 0xe73cc | `(unknown callee) 0x201fc` | `[sp-32]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe7178 | `? != 0x0` | `? == 0x0` |
| `bge` at 0xe719c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xe7424 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe74ac | `r0` | `?` |
