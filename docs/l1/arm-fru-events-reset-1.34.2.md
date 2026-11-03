# ARM FRU events, deadlock and reset-report paths in 1.34.2

Read from `6.5.27/usr/cpu/firmware/sysco/l1.bin`, package `6.5.27:l1#3` (version 1.34.2),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x3b0c1c`.
Carrier SHA-256: `45c872a661d4a4ad20a4cb482fe2aee921ee9ae52784382be125b3d7b7836572`.
Package SHA-256: `d8561deedcf6bae5bef085e0420886534c72bf6c59c54b742d086b9998264128`.
Inner executable SHA-256: `65c32185ef6f284717fd705e751157059ffed1b028bd6195f92eac73749012d3`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1069932 bytes.
Segment SHA-256: `c92d999ce63930093f87780073854fbb74f634d99098b88b7830a55634c9de6c`. Analysed routine spans use the exact
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
The status selector and deadlock routine have unresolved computed transfers. Excluded destinations supply no known-edge arguments or effects. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/fru-events-reset-facts.txt) retain all tool sections.

## `FRUInit`

- `FRUInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa1130 to 0xa1140.
  Established. The body saves frame registers and immediately restores them through the return
  (`push` at 0xa1134, `ldmdb` at 0xa113c). The reported r0 value is the entry r0 (`ldmdb` at
  0xa113c). Calls and conditional branches are absent, and an initialization effect is not
  established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0xa113c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa113c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa113c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa113c | `r0` | `r0` |

## `FRUEventInit`

- `FRUEventInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa4510 to 0xa4588.
  Established. The first branch takes the bypass when `[r0] == 0x0` and falls through otherwise
  (`beq` at 0xa4530). The loop calls `0x18be4` with four unknown candidates (`bl` at 0xa4568). At
  the join, a word store writes `0x0` at an unknown target (`str` at 0xa4580). The r0 return and
  event-list cleanup semantics remain unknown (`ldmdb` at 0xa4584).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa4520 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa4524 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa4528 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0xa4534 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa4538 | load | 4 | `r0 +0x0` | `-` |
| `str` at 0xa453c | store | 4 | `sp -0x18` | `[r0]` |
| `str` at 0xa4544 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xa4548 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4558 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa455c | load | 4 | `(unknown)` | `-` |
| `str` at 0xa4560 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xa4564 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa456c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa4570 | store | 4 | `sp -0x18` | `[sp-28]` |
| `ldr` at 0xa4578 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa4580 | store | 4 | `(unknown)` | `0x0` |
| `ldmdb` at 0xa4584 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa4584 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa4584 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa4568 | `(unknown callee) 0x18be4` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa4530 | `[r0] == 0x0` | `[r0] != 0x0` |
| `bne` at 0xa4550 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa4584 | `r0` | `?` |

## `FRUEventMerge`

- `FRUEventMerge`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa467c to 0xa46d8.
  Established. The word store near the call writes `0x0` at an unknown target (`str` at 0xa46c0).
  The call to `0xa4588` carries `[sp-20]`, `[sp-28]`, unknown, `0x0` (`bl` at 0xa46cc); its loop
  branch retains the predicate `Z == 0`, following that edge when true and falling through otherwise
  (`bne` at 0xa469c). The event-merge role and final r0 value are unknown (`ldmdb` at 0xa46d4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa468c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa4690 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xa4694 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa46a4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa46a8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xa46ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa46b0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa46b4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xa46b8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa46c0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xa46c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa46c8 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xa46d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa46d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa46d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa46cc | `(unknown callee) 0xa4588` | `[sp-20]` | `[sp-28]` | `?` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa469c | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa46d4 | `r0` | `?` |

## `FRUEventAdd`

- `FRUEventAdd`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa4588 to 0xa467c.
  Established. The opening test takes the continuation for `[r0] != 0x0` and falls through on zero
  to store entry r1 as a word at entry `r0 +0x0` (`bne` at 0xa45ac, `str` at 0xa45b8). On
  continuation, the first call to `0x18a58` carries entry r1, `[r0]`, `0x8`, entry r0 (`bl` at
  0xa45d0); its signed result takes the later path when positive and falls through when
  less-or-equal to zero (`bgt` at 0xa45dc). That fall-through arm stores word `[[sp-20]]` at
  `[sp-24] +0xa8` and word `[sp-24]` at `[sp-20] +0x0` (`str` at 0xa45ec, `str` at 0xa45f8). The
  event ordering rule and merged return are unproven (`ldmdb` at 0xa4678).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa4598 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa459c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xa45a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa45a4 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0xa45b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa45b4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa45b8 | store | 4 | `r0 +0x0` | `r1` |
| `ldr` at 0xa45c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa45c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa45c8 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0xa45e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa45e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa45e8 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xa45ec | store | 4 | `[sp-24] +0xa8` | `[[sp-20]]` |
| `ldr` at 0xa45f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa45f4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa45f8 | store | 4 | `[sp-20] +0x0` | `[sp-24]` |
| `ldr` at 0xa4600 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa4604 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xa4608 | store | 4 | `sp -0x1c` | `[[sp-20]]` |
| `ldr` at 0xa460c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa4610 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xa4614 | load | 4 | `[[sp-20]] +0xa8` | `-` |
| `str` at 0xa4618 | store | 4 | `sp -0x20` | `[[[sp-20]]+168]` |
| `ldr` at 0xa461c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa4628 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa462c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa4648 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa464c | store | 4 | `sp -0x1c` | `[sp-32]` |
| `ldr` at 0xa4650 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa4654 | load | 4 | `[sp-32] +0xa8` | `-` |
| `str` at 0xa4658 | store | 4 | `sp -0x20` | `[[sp-32]+168]` |
| `ldr` at 0xa4660 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4664 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa4668 | store | 4 | `[sp-24] +0xa8` | `?` |
| `ldr` at 0xa466c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa4670 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa4674 | store | 4 | `[sp-28] +0xa8` | `[sp-24]` |
| `ldmdb` at 0xa4678 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa4678 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa4678 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa45d0 | `(unknown callee) 0x18a58` | `r1` | `[r0]` | `0x8` | `r0` |
| `bl` at 0xa4634 | `(unknown callee) 0x18a58` | `[sp-24]` | `?` | `0x8` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa45ac | `[r0] != 0x0` | `[r0] == 0x0` |
| `bgt` at 0xa45dc | `ret(0x18a58) > 0x0 (signed, 32-bit)` | `ret(0x18a58) <= 0x0 (signed, 32-bit)` |
| `beq` at 0xa4624 | `Z == 1` | `not (Z == 1)` |
| `bge` at 0xa4640 | `ret(0x18a58) >= 0x0 (signed, 32-bit)` | `ret(0x18a58) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa4678 | `r0` | `?` |

## `FRUEventCreate`

- `FRUEventCreate`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa46d8 to 0xa487c.
  Established. A call to `0x18920` carries `0xac`, entry r1, entry r2, entry r3 (`bl` at 0xa46fc);
  its nonzero result takes the continuation and zero falls through to the common exit (`bne` at
  0xa4710). Later halfword stores clear `[sp-36] +0x1c` and `[sp-36] +0x1e` (`strh` at 0xa479c,
  `strh` at 0xa47a8). Byte stores clear `[sp-36] +0x67` and `[sp-36] +0xa7` (`strb` at 0xa4838,
  `strb` at 0xa485c). A word store clears `[sp-36] +0xa8` (`str` at 0xa4868). The final call to
  `0xa4588` has `[sp-20]`, `[sp-36]`, `[sp-36]`, `0x0` (`bl` at 0xa4874). Record-field meanings and
  the joined r0 return remain unknown (`ldmdb` at 0xa4878).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa46e8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa46ec | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa46f0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xa46f4 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xa4704 | store | 4 | `sp -0x24` | `ret(0x18920)` |
| `ldr` at 0xa4708 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa4718 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa471c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4728 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xa472c | load | 1 | `sp -0x1c` | `-` |
| `strb` at 0xa4730 | store | 1 | `[sp-36] +0x8` | `([sp-28]:1 & 0xff)` |
| `ldr` at 0xa4734 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xa4738 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0xa473c | store | 1 | `[sp-36] +0x9` | `([sp-32]:1 & 0xff)` |
| `ldr` at 0xa4740 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xa4744 | load | 1 | `sp +0x0` | `-` |
| `strb` at 0xa4748 | store | 1 | `[sp-36] +0xa` | `([sp]:1 & 0xff)` |
| `ldr` at 0xa474c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xa4750 | load | 1 | `sp +0x4` | `-` |
| `strb` at 0xa4754 | store | 1 | `[sp-36] +0xb` | `([sp+4]:1 & 0xff)` |
| `ldr` at 0xa4758 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xa475c | load | 1 | `sp +0x8` | `-` |
| `strb` at 0xa4760 | store | 1 | `[sp-36] +0xc` | `([sp+8]:1 & 0xff)` |
| `ldr` at 0xa4764 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa4768 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xa476c | store | 4 | `[sp-36] +0x10` | `[sp+12]` |
| `ldr` at 0xa4770 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xa4774 | load | 1 | `sp +0x10` | `-` |
| `strb` at 0xa4778 | store | 1 | `[sp-36] +0xd` | `([sp+16]:1 & 0xff)` |
| `ldr` at 0xa477c | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xa4780 | load | 2 | `sp +0x14` | `-` |
| `strh` at 0xa4784 | store | 2 | `[sp-36] +0x18` | `([sp+20]:2 & 0xffff)` |
| `ldr` at 0xa4788 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xa478c | load | 2 | `sp +0x18` | `-` |
| `strh` at 0xa4790 | store | 2 | `[sp-36] +0x1a` | `([sp+24]:2 & 0xffff)` |
| `ldr` at 0xa4794 | load | 4 | `sp -0x24` | `-` |
| `strh` at 0xa479c | store | 2 | `[sp-36] +0x1c` | `0x0` |
| `ldr` at 0xa47a0 | load | 4 | `sp -0x24` | `-` |
| `strh` at 0xa47a8 | store | 2 | `[sp-36] +0x1e` | `0x0` |
| `ldr` at 0xa47ac | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xa47b0 | load | 1 | `sp +0x1c` | `-` |
| `strb` at 0xa47b4 | store | 1 | `[sp-36] +0x14` | `([sp+28]:1 & 0xff)` |
| `ldr` at 0xa47b8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xa47bc | load | 1 | `sp +0x20` | `-` |
| `strb` at 0xa47c0 | store | 1 | `[sp-36] +0x20` | `([sp+32]:1 & 0xff)` |
| `ldr` at 0xa47c4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xa47c8 | load | 1 | `sp +0x24` | `-` |
| `strb` at 0xa47cc | store | 1 | `[sp-36] +0x15` | `([sp+36]:1 & 0xff)` |
| `ldr` at 0xa47d0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xa47d4 | load | 1 | `sp +0x28` | `-` |
| `strb` at 0xa47d8 | store | 1 | `[sp-36] +0x21` | `([sp+40]:1 & 0xff)` |
| `ldr` at 0xa47dc | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xa47e0 | load | 1 | `sp +0x2c` | `-` |
| `strb` at 0xa47e4 | store | 1 | `[sp-36] +0x22` | `([sp+44]:1 & 0xff)` |
| `ldr` at 0xa47e8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xa47ec | load | 1 | `sp +0x30` | `-` |
| `strb` at 0xa47f0 | store | 1 | `[sp-36] +0x23` | `([sp+48]:1 & 0xff)` |
| `ldr` at 0xa47f4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xa47f8 | load | 1 | `sp +0x34` | `-` |
| `strb` at 0xa47fc | store | 1 | `[sp-36] +0x24` | `([sp+52]:1 & 0xff)` |
| `ldr` at 0xa4800 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xa4804 | load | 1 | `sp +0x38` | `-` |
| `strb` at 0xa4808 | store | 1 | `[sp-36] +0x26` | `([sp+56]:1 & 0xff)` |
| `ldr` at 0xa480c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xa4810 | load | 1 | `sp +0x3c` | `-` |
| `strb` at 0xa4814 | store | 1 | `[sp-36] +0x25` | `([sp+60]:1 & 0xff)` |
| `ldr` at 0xa4818 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa4824 | load | 4 | `sp +0x40` | `-` |
| `ldr` at 0xa4830 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xa4838 | store | 1 | `[sp-36] +0x67` | `0x0` |
| `ldr` at 0xa483c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa4848 | load | 4 | `sp +0x44` | `-` |
| `ldr` at 0xa4854 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xa485c | store | 1 | `[sp-36] +0xa7` | `0x0` |
| `ldr` at 0xa4860 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa4868 | store | 4 | `[sp-36] +0xa8` | `0x0` |
| `ldr` at 0xa486c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa4870 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xa4878 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa4878 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa4878 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa46fc | `(unknown callee) 0x18920` | `0xac` | `r1` | `r2` | `r3` |
| `bl` at 0xa4724 | `(unknown callee) 0x18ae8` | `ret(0x18920)` | `[sp-24]` | `0x8` | `ret(0x18920)` |
| `bl` at 0xa482c | `(unknown callee) 0x18c38` | `[sp-36]+40` | `[sp+64]` | `0x40` | `[sp-36]+40` |
| `bl` at 0xa4850 | `(unknown callee) 0x18c38` | `[sp-36]+104` | `[sp+68]` | `0x40` | `[sp-36]+104` |
| `bl` at 0xa4874 | `(unknown callee) 0xa4588` | `[sp-20]` | `[sp-36]` | `[sp-36]` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa4710 | `ret(0x18920) != 0x0` | `ret(0x18920) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa4878 | `r0` | `?` |

## `FRUEventRank`

- `FRUEventRank`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa487c to 0xa4b74.
  Established. Three calls to `0xa1140` share candidates unknown, `0x3f`, `0x20`, unknown (`bl` at
  0xa48d4, `bl` at 0xa4960, `bl` at 0xa49f0). Halfword stores put `([sp-32]:2 & 0xffff)` at `[sp-20]
  +0x1c` and `([sp-36]:2 & 0xffff)` at `[sp-20] +0x1e` (`strh` at 0xa49b4, `strh` at 0xa4a94). A
  later halfword store writes `([sp-40]:2 & 0xffff)` at an unknown target (`strh` at 0xa4b5c).
  Ranking policy and the r0 return are not established (`ldmdb` at 0xa4b70).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa488c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa4894 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xa489c | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xa48a4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xa48a8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa48ac | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xa48b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa48b4 | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0xa48b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa48c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa48e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa4900 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa4904 | load | 1 | `[sp-20] +0x26` | `-` |
| `ldr` at 0xa4910 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa4914 | load | 1 | `[sp-20] +0x26` | `-` |
| `ldr` at 0xa4920 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa4928 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xa492c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa4930 | load | 4 | `[sp-20] +0xa8` | `-` |
| `str` at 0xa4934 | store | 4 | `sp -0x14` | `[[sp-20]+168]` |
| `ldr` at 0xa493c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa4940 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xa4944 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa4954 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa4970 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa498c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa4990 | load | 1 | `[sp-20] +0x26` | `-` |
| `ldr` at 0xa499c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa49a0 | load | 1 | `[sp-20] +0x26` | `-` |
| `ldr` at 0xa49ac | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xa49b0 | load | 2 | `sp -0x20` | `-` |
| `strh` at 0xa49b4 | store | 2 | `[sp-20] +0x1c` | `([sp-32]:2 & 0xffff)` |
| `ldr` at 0xa49b8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa49c0 | store | 4 | `sp -0x20` | `[sp-32]-1` |
| `ldr` at 0xa49c4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa49c8 | store | 4 | `sp -0x1c` | `[sp-24]` |
| `str` at 0xa49d0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xa49d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa49e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa4a00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa4a1c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa4a20 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0xa4a2c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa4a30 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0xa4a3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa4a40 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xa4a44 | load | 2 | `[sp-28] +0x18` | `-` |
| `ldrh` at 0xa4a48 | load | 2 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xa4a54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa4a58 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xa4a5c | load | 2 | `[sp-28] +0x1a` | `-` |
| `ldrh` at 0xa4a60 | load | 2 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xa4a70 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa4a78 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xa4a7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa4a80 | load | 4 | `[sp-28] +0xa8` | `-` |
| `str` at 0xa4a84 | store | 4 | `sp -0x1c` | `[[sp-28]+168]` |
| `ldr` at 0xa4a8c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xa4a90 | load | 2 | `sp -0x24` | `-` |
| `strh` at 0xa4a94 | store | 2 | `[sp-20] +0x1e` | `([sp-36]:2 & 0xffff)` |
| `ldr` at 0xa4a98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa4a9c | load | 4 | `[sp-20] +0xa8` | `-` |
| `str` at 0xa4aa0 | store | 4 | `sp -0x14` | `[[sp-20]+168]` |
| `ldr` at 0xa4aa8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa4aac | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xa4ab0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa4ac4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xa4ac8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa4acc | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xa4ad0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa4ae0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa4ae4 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xa4ae8 | load | 2 | `(unknown)` | `-` |
| `ldrh` at 0xa4aec | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xa4af8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa4afc | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xa4b00 | load | 2 | `(unknown)` | `-` |
| `ldrh` at 0xa4b04 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xa4b14 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xa4b18 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xa4b24 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xa4b28 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xa4b2c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa4b38 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xa4b3c | load | 2 | `(unknown)` | `-` |
| `str` at 0xa4b40 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xa4b44 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa4b48 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa4b4c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xa4b54 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xa4b58 | load | 2 | `sp -0x28` | `-` |
| `strh` at 0xa4b5c | store | 2 | `(unknown)` | `([sp-40]:2 & 0xffff)` |
| `ldr` at 0xa4b60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa4b64 | load | 4 | `[sp-20] +0xa8` | `-` |
| `str` at 0xa4b68 | store | 4 | `sp -0x14` | `[[sp-20]+168]` |
| `ldmdb` at 0xa4b70 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa4b70 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa4b70 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa48d4 | `(unknown callee) 0xa1140` | `?` | `0x3f` | `0x20` | `?` |
| `bl` at 0xa48f0 | `(unknown callee) 0xa1140` | `[sp-20]` | `0x1f` | `0x0` | `ret(0xa1140)` |
| `bl` at 0xa4960 | `(unknown callee) 0xa1140` | `?` | `0x3f` | `0x20` | `?` |
| `bl` at 0xa497c | `(unknown callee) 0xa1140` | `[sp-20]` | `0x1f` | `0x0` | `ret(0xa1140)` |
| `bl` at 0xa49f0 | `(unknown callee) 0xa1140` | `?` | `0x3f` | `0x20` | `?` |
| `bl` at 0xa4a0c | `(unknown callee) 0xa1140` | `[sp-28]` | `0x1f` | `0x0` | `ret(0xa1140)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa48c0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xa48e0 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xa48fc | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xa490c | `[[sp-20]+38]:1 == 0x4` | `[[sp-20]+38]:1 != 0x4` |
| `beq` at 0xa491c | `[[sp-20]+38]:1 == 0x8` | `[[sp-20]+38]:1 != 0x8` |
| `bne` at 0xa494c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xa496c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xa4988 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xa4998 | `[[sp-20]+38]:1 == 0x4` | `[[sp-20]+38]:1 != 0x4` |
| `beq` at 0xa49a8 | `[[sp-20]+38]:1 == 0x8` | `[[sp-20]+38]:1 != 0x8` |
| `bne` at 0xa49dc | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xa49fc | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xa4a18 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xa4a28 | `[[sp-28]+38]:1 == 0x4` | `[[sp-28]+38]:1 != 0x4` |
| `beq` at 0xa4a38 | `[[sp-28]+38]:1 == 0x8` | `[[sp-28]+38]:1 != 0x8` |
| `beq` at 0xa4a50 | `[[sp-28]+24]:2 == [[sp-20]+24]:2` | `[[sp-28]+24]:2 != [[sp-20]+24]:2` |
| `beq` at 0xa4a68 | `[[sp-28]+26]:2 == [[sp-20]+24]:2` | `[[sp-28]+26]:2 != [[sp-20]+24]:2` |
| `bne` at 0xa4ab8 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xa4ad8 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xa4af4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xa4b0c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xa4b20 | `Z == 1` | `not (Z == 1)` |
| `bls` at 0xa4b34 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa4b70 | `r0` | `?` |

## `FRUEventPrint`

- `FRUEventPrint`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa4b74 to 0xa4f38.
  Established. Five indirect calls have unknown callees and targets, with candidates `[sp-28]`,
  `sp-164`, `ret(0x18a1c)`, `sp-164` (`mov` at 0xa4bf4, `mov` at 0xa4c58, `mov` at 0xa4cb0, `mov` at
  0xa4dd0, `mov` at 0xa4e7c). The entry selector takes its alternate path for entry `r0 != 0x0` and
  falls through on zero (`bne` at 0xa4bac). Stack word stores include `0x107368` at `sp -0xb8` and
  `0x107380` at `sp -0xac` (`str` at 0xa4c00, `str` at 0xa4c18). Output content, callback meaning
  and the returned r0 value remain unknown (`ldmdb` at 0xa4efc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa4b84 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa4b88 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa4b8c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xa4b90 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xa4b98 | store | 4 | `sp -0xa8` | `0x0` |
| `ldr` at 0xa4b9c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa4ba0 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0xa4ba4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa4bb0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa4bc4 | load | 4 | `abs 0xa4f00` | `-` |
| `ldr` at 0xa4be4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4be8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa4bfc | load | 4 | `abs 0xa4f04` | `-` |
| `str` at 0xa4c00 | store | 4 | `sp -0xb8` | `0x107368` |
| `ldr` at 0xa4c04 | load | 4 | `abs 0xa4f08` | `-` |
| `str` at 0xa4c08 | store | 4 | `sp -0xb4` | `0x107370` |
| `ldr` at 0xa4c0c | load | 4 | `abs 0xa4f0c` | `-` |
| `str` at 0xa4c10 | store | 4 | `sp -0xb0` | `0x107378` |
| `ldr` at 0xa4c14 | load | 4 | `abs 0xa4f10` | `-` |
| `str` at 0xa4c18 | store | 4 | `sp -0xac` | `0x107380` |
| `ldr` at 0xa4c20 | load | 4 | `abs 0xa4f14` | `-` |
| `ldr` at 0xa4c24 | load | 4 | `abs 0xa4f18` | `-` |
| `ldr` at 0xa4c28 | load | 4 | `abs 0xa4f1c` | `-` |
| `ldr` at 0xa4c48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4c4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa4c60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa4c6c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa4c80 | load | 4 | `abs 0xa4f20` | `-` |
| `ldr` at 0xa4ca0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4ca4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa4cb8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa4cc8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa4cd4 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xa4cd8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xa4ce4 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xa4ce8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xa4cec | load | 4 | `abs 0xa4f24` | `-` |
| `ldr` at 0xa4cfc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa4d04 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa4d0c | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xa4d10 | load | 2 | `(unknown)` | `-` |
| `str` at 0xa4d14 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0xa4d18 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xa4d1c | load | 2 | `(unknown)` | `-` |
| `str` at 0xa4d20 | store | 4 | `sp -0xb4` | `?` |
| `ldr` at 0xa4d24 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xa4d28 | load | 2 | `(unknown)` | `-` |
| `str` at 0xa4d2c | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0xa4d30 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xa4d34 | load | 2 | `(unknown)` | `-` |
| `str` at 0xa4d38 | store | 4 | `sp -0xac` | `?` |
| `ldr` at 0xa4d40 | load | 4 | `abs 0xa4f28` | `-` |
| `ldr` at 0xa4d58 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa4d60 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa4d68 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xa4d6c | load | 2 | `(unknown)` | `-` |
| `str` at 0xa4d70 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0xa4d74 | load | 4 | `abs 0xa4f2c` | `-` |
| `str` at 0xa4d78 | store | 4 | `sp -0xb4` | `0x10741c` |
| `ldr` at 0xa4d7c | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xa4d80 | load | 2 | `(unknown)` | `-` |
| `str` at 0xa4d84 | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0xa4d88 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xa4d8c | load | 2 | `(unknown)` | `-` |
| `str` at 0xa4d90 | store | 4 | `sp -0xac` | `?` |
| `ldr` at 0xa4d98 | load | 4 | `abs 0xa4f30` | `-` |
| `ldr` at 0xa4dc0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4dc4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa4dd8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa4de4 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xa4de8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xa4dec | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xa4df0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xa4df8 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0xa4e04 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xa4e08 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xa4e0c | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xa4e10 | load | 2 | `(unknown)` | `-` |
| `str` at 0xa4e18 | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0xa4e1c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa4e20 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa4e24 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xa4e2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa4e38 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0xa4e4c | load | 4 | `abs 0xa4f34` | `-` |
| `ldr` at 0xa4e6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4e70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa4e84 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0xa4e94 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa4e98 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xa4e9c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa4eac | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xa4eb0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xa4eb4 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xa4eb8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xa4ec0 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0xa4ecc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4ed0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa4ed4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa4edc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa4ee0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa4ee4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xa4eec | load | 4 | `sp -0xa8` | `-` |
| `str` at 0xa4ef4 | store | 4 | `sp -0xa8` | `?` |
| `ldmdb` at 0xa4efc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa4efc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa4efc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa4bc8 | `(unknown callee) 0x18c74` | `sp-164` | `0x10733c` | `r2` | `sp-164` |
| `bl` at 0xa4bd4 | `(unknown callee) 0x18a1c` | `sp-164` | `?` | `?` | `sp-164` |
| `mov` at 0xa4bf4 | `(unknown callee) (unknown target)` | `[sp-28]` | `sp-164` | `ret(0x18a1c)` | `sp-164` |
| `bl` at 0xa4c2c | `(unknown callee) 0x18c74` | `sp-164` | `0x107384` | `0x1073a8` | `0x1073ac` |
| `bl` at 0xa4c38 | `(unknown callee) 0x18a1c` | `sp-164` | `?` | `?` | `sp-164` |
| `mov` at 0xa4c58 | `(unknown callee) (unknown target)` | `[sp-28]` | `sp-164` | `ret(0x18a1c)` | `sp-164` |
| `bl` at 0xa4c84 | `(unknown callee) 0x18c74` | `sp-164` | `0x1073b4` | `r2` | `sp-164` |
| `bl` at 0xa4c90 | `(unknown callee) 0x18a1c` | `sp-164` | `?` | `?` | `sp-164` |
| `mov` at 0xa4cb0 | `(unknown callee) (unknown target)` | `[sp-28]` | `sp-164` | `ret(0x18a1c)` | `sp-164` |
| `bl` at 0xa4d4c | `(unknown callee) 0x18c74` | `sp-164` | `0x1073f8` | `?` | `?` |
| `bl` at 0xa4da4 | `(unknown callee) 0x18c74` | `sp-164` | `0x107424` | `?` | `?` |
| `bl` at 0xa4db0 | `(unknown callee) 0x18a1c` | `sp-164` | `?` | `?` | `sp-164` |
| `mov` at 0xa4dd0 | `(unknown callee) (unknown target)` | `[sp-28]` | `sp-164` | `ret(0x18a1c)` | `sp-164` |
| `bl` at 0xa4e50 | `(unknown callee) 0x18c74` | `sp-164` | `0x107448` | `?` | `sp-164` |
| `bl` at 0xa4e5c | `(unknown callee) 0x18a1c` | `sp-164` | `?` | `?` | `sp-164` |
| `mov` at 0xa4e7c | `(unknown callee) (unknown target)` | `[sp-28]` | `sp-164` | `ret(0x18a1c)` | `sp-164` |
| `bl` at 0xa4ed8 | `(unknown callee) 0xa4f38` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa4bac | `r0 != 0x0` | `r0 == 0x0` |
| `beq` at 0xa4bb8 | `r3 == 0x0` | `r3 != 0x0` |
| `bne` at 0xa4c68 | `r0 != 0x2` | `r0 == 0x2` |
| `beq` at 0xa4c74 | `r3 == 0x0` | `r3 != 0x0` |
| `bne` at 0xa4cc0 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xa4cd0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xa4ce0 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xa4cf4 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xa4de0 | `Z == 0` | `not (Z == 0)` |
| `bls` at 0xa4e00 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xa4e34 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xa4e40 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xa4e8c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xa4ea4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xa4ec8 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa4efc | `r0` | `?` |

## `FRUStatus`

- `FRUStatus`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa7c14 to 0xa7cf0.
  Established. The computed transfer takes an unresolved edge for entry `r0 <= 0x8` unsigned and
  falls through for greater values (`ldrls` at 0xa7c30). The known fall-through route stores word
  `0x10772c` at `sp -0x18` and returns that same value in r0 (`str` at 0xa7cc0, `ldmdb` at 0xa7cc8).
  No calls are reported on known edges. The excluded selector cases and status encoding remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa7c24 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa7c28 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0xa7c30 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa7cbc | load | 4 | `abs 0xa7cec` | `-` |
| `str` at 0xa7cc0 | store | 4 | `sp -0x18` | `0x10772c` |
| `ldr` at 0xa7cc4 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xa7cc8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa7cc8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa7cc8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xa7c30 | `r0 <= 0x8 (unsigned, 32-bit)` | `r0 > 0x8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa7cc8 | `r0` | `0x10772c` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xa7c30).

## `FRUDumpBuff`

- `FRUDumpBuff`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa841c to 0xa8468.
  Established. The opening call reaches `0xa8db8` with entry r0 through r3 (`bl` at 0xa8430). The
  next call targets `0xa8200` with `0xa8e74`, `ret(0xa8db8)`, `[sp-20]`, `ret(0xa8db8)` (`bl` at
  0xa8448). The last call carries `[sp-24]`, `0x0`, unknown, unknown to `0xa8e08` and supplies the
  returned r0 value (`bl` at 0xa8454, `ldmdb` at 0xa8460). Buffer lifetime and the dump interface
  remain unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa842c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa8438 | store | 4 | `sp -0x18` | `ret(0xa8db8)` |
| `ldr` at 0xa843c | load | 4 | `abs 0xa8464` | `-` |
| `ldr` at 0xa8440 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8444 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa844c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xa8460 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa8460 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa8460 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa8430 | `(unknown callee) 0xa8db8` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xa8448 | `(unknown callee) 0xa8200` | `0xa8e74` | `ret(0xa8db8)` | `[sp-20]` | `ret(0xa8db8)` |
| `bl` at 0xa8454 | `(unknown callee) 0xa8e08` | `[sp-24]` | `0x0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa8460 | `r0` | `ret(0xa8e08)` |

## `FRUDump`

- `FRUDump`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa8200 to 0xa841c.
  Established. The first call targets `0xa6ce4` with entry r2, `sp-32`, `sp-36`, `sp-40` (`bl` at
  0xa8298). A subsequent call reaches `0xa7cf0` with `[sp-20]`, `[sp-24]`, `[sp-32]`, `[sp-36]`
  (`bl` at 0xa82f4). The later branch takes its bypass for `[sp-64] == 0x0` and falls through to
  `0x18be4` on nonzero (`beq` at 0xa83d0, `bl` at 0xa83d8). The final r0 result and ownership of the
  referenced buffers remain unknown (`ldmdb` at 0xa8418).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa8210 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa8214 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa8218 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xa8220 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xa8228 | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0xa8230 | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0xa8244 | store | 4 | `sp -0xc4` | `sp-44` |
| `str` at 0xa824c | store | 4 | `sp -0xc0` | `sp-48` |
| `str` at 0xa8254 | store | 4 | `sp -0xbc` | `sp-52` |
| `str` at 0xa825c | store | 4 | `sp -0xb8` | `sp-56` |
| `str` at 0xa8264 | store | 4 | `sp -0xb4` | `sp-60` |
| `str` at 0xa826c | store | 4 | `sp -0xb0` | `sp-64` |
| `str` at 0xa8274 | store | 4 | `sp -0xac` | `sp-72` |
| `str` at 0xa827c | store | 4 | `sp -0xa8` | `sp-76` |
| `str` at 0xa8284 | store | 4 | `sp -0xa4` | `sp-80` |
| `ldr` at 0xa8288 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa829c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xa82a0 | store | 4 | `sp -0xc4` | `[sp-44]` |
| `ldr` at 0xa82a4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xa82a8 | store | 4 | `sp -0xc0` | `[sp-48]` |
| `ldr` at 0xa82ac | load | 4 | `sp -0x34` | `-` |
| `str` at 0xa82b0 | store | 4 | `sp -0xbc` | `[sp-52]` |
| `ldr` at 0xa82b4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xa82b8 | store | 4 | `sp -0xb8` | `[sp-56]` |
| `ldr` at 0xa82bc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xa82c0 | store | 4 | `sp -0xb4` | `[sp-60]` |
| `ldr` at 0xa82c4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa82c8 | store | 4 | `sp -0xb0` | `[sp-64]` |
| `ldr` at 0xa82cc | load | 4 | `sp -0x48` | `-` |
| `str` at 0xa82d0 | store | 4 | `sp -0xac` | `[sp-72]` |
| `ldr` at 0xa82d4 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xa82d8 | store | 4 | `sp -0xa8` | `[sp-76]` |
| `ldr` at 0xa82dc | load | 4 | `sp -0x50` | `-` |
| `str` at 0xa82e0 | store | 4 | `sp -0xa4` | `[sp-80]` |
| `ldr` at 0xa82e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa82e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa82ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa82f0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa82fc | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xa8300 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa830c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xa8310 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa8320 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xa8334 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa833c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa8348 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xa835c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa8364 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa836c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xa8380 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa8388 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa8394 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xa83a8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa83b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa83b8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xa83c0 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xa83c8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa83d4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa83dc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xa83e8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xa83f0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa83fc | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa8404 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xa8410 | load | 4 | `sp -0x50` | `-` |
| `ldmdb` at 0xa8418 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa8418 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa8418 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa8298 | `(unknown callee) 0xa6ce4` | `r2` | `sp-32` | `sp-36` | `sp-40` |
| `bl` at 0xa82f4 | `(unknown callee) 0xa7cf0` | `[sp-20]` | `[sp-24]` | `[sp-32]` | `[sp-36]` |
| `bl` at 0xa8368 | `(unknown callee) 0x18be4` | `?` | `?` | `?` | `?` |
| `bl` at 0xa83b4 | `(unknown callee) 0x18be4` | `?` | `?` | `?` | `?` |
| `bl` at 0xa83d8 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `[sp-64]` |
| `bl` at 0xa83ec | `(unknown callee) 0x18be4` | `[sp-72]` | `?` | `?` | `[sp-72]` |
| `bl` at 0xa8400 | `(unknown callee) 0x18be4` | `[sp-76]` | `?` | `?` | `[sp-76]` |
| `bl` at 0xa8414 | `(unknown callee) 0x18be4` | `[sp-80]` | `?` | `?` | `[sp-80]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa8308 | `[sp-64] == 0x0` | `[sp-64] != 0x0` |
| `blt` at 0xa8318 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xa8344 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xa8390 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xa83d0 | `[sp-64] == 0x0` | `[sp-64] != 0x0` |
| `beq` at 0xa83e4 | `[sp-72] == 0x0` | `[sp-72] != 0x0` |
| `beq` at 0xa83f8 | `[sp-76] == 0x0` | `[sp-76] != 0x0` |
| `beq` at 0xa840c | `[sp-80] == 0x0` | `[sp-80] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa8418 | `r0` | `?` |

## `_FRUDump`

- `_FRUDump`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa7cf0 to 0xa8200.
  Established. The initial call reaches `0xa7c14` with `[sp+12]`, entry r1, entry r2, entry r3 (`bl`
  at 0xa7d14). A later branch takes the continuing path for `[sp+12] == 0x0` and falls through
  to the exit on nonzero (`beq` at 0xa7d70). Calls to `0xa7be4` include candidates `[sp-20]`,
  `[sp-24]`, `[sp+24]`, `[sp+24]` and later `[sp-20]`, `[sp-24]`, `[sp+32]`, `[sp+32]` (`bl` at
  0xa816c, `bl` at 0xa81c4). The reported r0 return is `ret(0xa7be4)` (`ldmdb` at 0xa81d8).
  Formatting and pointee meanings are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa7d00 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa7d04 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa7d08 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xa7d0c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xa7d10 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xa7d24 | load | 4 | `abs 0xa81dc` | `-` |
| `ldr` at 0xa7d30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7d34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7d48 | load | 4 | `abs 0xa81e0` | `-` |
| `ldr` at 0xa7d4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa7d58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7d5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7d68 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xa7d90 | load | 4 | `abs 0xa81e4` | `-` |
| `ldr` at 0xa7d9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7da0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa7db0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xa7db4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa7db8 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa7dc8 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xa7dd8 | store | 4 | `sp -0x9c` | `ret(0xa87c0)` |
| `ldr` at 0xa7de0 | load | 4 | `abs 0xa81e8` | `-` |
| `ldr` at 0xa7de4 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xa7de8 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xa7df4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7df8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7e04 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa7e18 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xa7e20 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa7e24 | store | 4 | `sp -0x80` | `?` |
| `ldr` at 0xa7e28 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xa7e34 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xa7e40 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xa7e50 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xa7e64 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa7e78 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xa7e84 | load | 4 | `abs 0xa81ec` | `-` |
| `ldr` at 0xa7e88 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa7e98 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa7eac | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xa7eb8 | load | 4 | `abs 0xa81f0` | `-` |
| `ldr` at 0xa7ebc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa7ecc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa7ee0 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xa7eec | load | 4 | `abs 0xa81f4` | `-` |
| `ldr` at 0xa7ef0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa7efc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7f00 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa7f10 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xa7f14 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa7f28 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xa7f30 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa7f34 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa7f48 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa7f5c | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xa7f64 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa7f6c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa7f74 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa7f88 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xa7f90 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa7f98 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xa7fa0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa7fa4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa7fb8 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xa7fc0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa7fc8 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xa7fd0 | load | 1 | `(unknown)` | `-` |
| `str` at 0xa7fd4 | store | 4 | `sp -0x9c` | `?` |
| `ldr` at 0xa7fd8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa7fec | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xa7ff4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa7ffc | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xa8004 | load | 1 | `(unknown)` | `-` |
| `str` at 0xa8008 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0xa800c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa8020 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xa8028 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa8030 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xa8038 | load | 1 | `(unknown)` | `-` |
| `str` at 0xa803c | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0xa8040 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa8054 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xa805c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa8064 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xa806c | load | 1 | `(unknown)` | `-` |
| `str` at 0xa8070 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xa8074 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa8088 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xa8090 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa8098 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xa80a0 | load | 1 | `(unknown)` | `-` |
| `str` at 0xa80a4 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0xa80a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa80bc | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xa80c4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa80cc | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xa80d4 | load | 1 | `(unknown)` | `-` |
| `str` at 0xa80d8 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xa80dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa80f0 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xa80f8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa8100 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xa8108 | load | 1 | `(unknown)` | `-` |
| `str` at 0xa810c | store | 4 | `sp -0x84` | `?` |
| `ldr` at 0xa8110 | load | 4 | `abs 0xa81f8` | `-` |
| `ldr` at 0xa8114 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa8124 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8128 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8134 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa813c | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xa8144 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa814c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xa8154 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xa8160 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8164 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8168 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xa8170 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8174 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8178 | load | 4 | `abs 0xa81fc` | `-` |
| `ldr` at 0xa8180 | load | 4 | `sp +0x1c` | `-` |
| `ldr` at 0xa818c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8190 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8194 | load | 4 | `sp +0x1c` | `-` |
| `ldr` at 0xa819c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa81a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa81a4 | load | 4 | `abs 0xa81fc` | `-` |
| `ldr` at 0xa81ac | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa81b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa81bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa81c0 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa81c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa81cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa81d0 | load | 4 | `abs 0xa81fc` | `-` |
| `ldmdb` at 0xa81d8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa81d8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa81d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa7d14 | `(unknown callee) 0xa7c14` | `[sp+12]` | `r1` | `r2` | `r3` |
| `bl` at 0xa7d28 | `(unknown callee) 0x18c74` | `sp-124` | `0x107740` | `ret(0xa7c14)` | `sp-124` |
| `bl` at 0xa7d3c | `(unknown callee) 0xa7be4` | `[sp-20]` | `[sp-24]` | `sp-124` | `sp-124` |
| `bl` at 0xa7d50 | `(unknown callee) 0x18c74` | `sp-124` | `0x107750` | `[sp-28]` | `sp-124` |
| `bl` at 0xa7d64 | `(unknown callee) 0xa7be4` | `[sp-20]` | `[sp-24]` | `sp-124` | `sp-124` |
| `bl` at 0xa7d80 | `(unknown callee) 0x18b18` | `sp-32` | `?` | `?` | `sp-32` |
| `bl` at 0xa7d94 | `(unknown callee) 0x18c74` | `sp-124` | `0x107764` | `ret(0x18b18)` | `sp-124` |
| `bl` at 0xa7da8 | `(unknown callee) 0xa7be4` | `[sp-20]` | `[sp-24]` | `sp-124` | `sp-124` |
| `bl` at 0xa7dcc | `(unknown callee) 0xa87c0` | `[sp]` | `?` | `?` | `[sp+16]` |
| `bl` at 0xa7dec | `(unknown callee) 0x18c74` | `sp-124` | `0x107774` | `[sp+4]` | `[sp+8]` |
| `bl` at 0xa7e00 | `(unknown callee) 0xa7be4` | `[sp-20]` | `[sp-24]` | `sp-124` | `sp-124` |
| `bl` at 0xa7e8c | `(unknown callee) 0x18c74` | `sp-124` | `0x107784` | `?` | `?` |
| `bl` at 0xa7ec0 | `(unknown callee) 0x18c74` | `sp-124` | `0x107790` | `?` | `?` |
| `bl` at 0xa7ef4 | `(unknown callee) 0x18c74` | `sp-124` | `0x10779c` | `?` | `?` |
| `bl` at 0xa7f08 | `(unknown callee) 0xa7be4` | `[sp-20]` | `[sp-24]` | `sp-124` | `sp-124` |
| `bl` at 0xa811c | `(unknown callee) 0x18c74` | `sp-124` | `0x1077a4` | `?` | `?` |
| `bl` at 0xa8130 | `(unknown callee) 0xa7be4` | `[sp-20]` | `[sp-24]` | `sp-124` | `sp-124` |
| `bl` at 0xa816c | `(unknown callee) 0xa7be4` | `[sp-20]` | `[sp-24]` | `[sp+24]` | `[sp+24]` |
| `bl` at 0xa817c | `(unknown callee) 0xa7be4` | `[sp-20]` | `[sp-24]` | `0x107660` | `?` |
| `bl` at 0xa8198 | `(unknown callee) 0xa7be4` | `[sp-20]` | `[sp-24]` | `[sp+28]` | `[sp+28]` |
| `bl` at 0xa81a8 | `(unknown callee) 0xa7be4` | `[sp-20]` | `[sp-24]` | `0x107660` | `?` |
| `bl` at 0xa81c4 | `(unknown callee) 0xa7be4` | `[sp-20]` | `[sp-24]` | `[sp+32]` | `[sp+32]` |
| `bl` at 0xa81d4 | `(unknown callee) 0xa7be4` | `[sp-20]` | `[sp-24]` | `0x107660` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa7d70 | `[sp+12] == 0x0` | `[sp+12] != 0x0` |
| `blt` at 0xa7dc0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xa7e30 | `[sp-128] == 0x2` | `[sp-128] != 0x2` |
| `bgt` at 0xa7e3c | `[sp-128] > 0x2 (signed, 32-bit)` | `[sp-128] <= 0x2 (signed, 32-bit)` |
| `beq` at 0xa7e48 | `[sp-128] == 0x1` | `[sp-128] != 0x1` |
| `beq` at 0xa7e58 | `[sp-128] == 0x3` | `[sp-128] != 0x3` |
| `blt` at 0xa7f3c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xa815c | `[sp+24] == 0x0` | `[sp+24] != 0x0` |
| `beq` at 0xa8188 | `[sp+28] == 0x0` | `[sp+28] != 0x0` |
| `beq` at 0xa81b4 | `[sp+32] == 0x0` | `[sp+32] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa81d8 | `r0` | `ret(0xa7be4)` |

## `L1FRUSHUBDeadlock`

- `L1FRUSHUBDeadlock`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8e7d0 to 0x8f12c.
  Established. The first branch takes its bypass for `[0x11b310] != 0x0` and falls through on zero
  (`bne` at 0x8e820). The zero arm writes word `0x1e` at `0x282320`, an unknown word at `0x282340`,
  word `ret(0x18920)` at `0x11b314`, and word `0x1` at `0x11b310` (`str` at 0x8e82c, `str` at
  0x8e84c, `str` at 0x8e864, `str` at 0x8e870). The call to `0xc97a8` has unknown, unknown,
  `sp-116`, `sp-120` (`bl` at 0x8ea64); its later computed transfer takes an unresolved edge for `?
  <= 0x8` unsigned and falls through above that bound (`ldrls` at 0x8eaf8). The reported r0 return
  on known edges is `0x0` (`ldmdb` at 0x8f0fc). Excluded dispatch effects and physical deadlock
  recovery remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8e7e0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x8e7e4 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x8e7e8 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x8e7f0 | store | 4 | `sp -0x48` | `0x1e` |
| `str` at 0x8e7f8 | store | 4 | `sp -0x4c` | `0x5` |
| `str` at 0x8e800 | store | 4 | `sp -0x64` | `0x0` |
| `str` at 0x8e808 | store | 4 | `sp -0x68` | `0x0` |
| `str` at 0x8e810 | store | 4 | `sp -0x6c` | `0x0` |
| `ldr` at 0x8e814 | load | 4 | `abs 0x8f100` | `-` |
| `ldr` at 0x8e818 | load | 4 | `abs 0x11b310` | `-` |
| `ldr` at 0x8e824 | load | 4 | `abs 0x8f104` | `-` |
| `str` at 0x8e82c | store | 4 | `abs 0x282320` | `0x1e` |
| `ldr` at 0x8e830 | load | 4 | `abs 0x8f108` | `-` |
| `ldr` at 0x8e834 | load | 4 | `abs 0x8f104` | `-` |
| `ldr` at 0x8e838 | load | 4 | `abs 0x282320` | `-` |
| `str` at 0x8e84c | store | 4 | `abs 0x282340` | `?` |
| `ldr` at 0x8e850 | load | 4 | `abs 0x8f10c` | `-` |
| `ldr` at 0x8e854 | load | 4 | `abs 0x8f108` | `-` |
| `ldr` at 0x8e858 | load | 4 | `abs 0x282340` | `-` |
| `str` at 0x8e864 | store | 4 | `abs 0x11b314` | `ret(0x18920)` |
| `ldr` at 0x8e868 | load | 4 | `abs 0x8f100` | `-` |
| `str` at 0x8e870 | store | 4 | `abs 0x11b310` | `0x1` |
| `ldr` at 0x8e888 | load | 4 | `abs 0x8f10c` | `-` |
| `ldr` at 0x8e88c | load | 4 | `abs 0x8f108` | `-` |
| `ldr` at 0x8e890 | load | 4 | `abs 0x11b314` | `-` |
| `ldr` at 0x8e898 | load | 4 | `abs 0x282340` | `-` |
| `str` at 0x8e8a4 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0x8e8ac | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0x8e8b0 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8e8b4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x8e8c4 | load | 4 | `abs 0x8f110` | `-` |
| `ldr` at 0x8e8c8 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8e8dc | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x8e8e0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8e8e8 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x8e8ec | load | 4 | `sp -0x58` | `-` |
| `str` at 0x8e8f4 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x8e8fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8e900 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x8e90c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8e910 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x8e930 | store | 4 | `[sp-28] +0x0` | `ret(0x18920)` |
| `ldr` at 0x8e934 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x8e938 | store | 4 | `sp -0x60` | `[[sp-28]]` |
| `ldr` at 0x8e93c | load | 4 | `sp -0x60` | `-` |
| `strb` at 0x8e944 | store | 1 | `[[sp-28]] +0x0` | `0x0` |
| `ldr` at 0x8e94c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8e950 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x8e95c | store | 4 | `sp -0x70` | `ret(0x18a1c)` |
| `ldr` at 0x8e960 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8e964 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8e968 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x8e97c | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x8e984 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x8e994 | store | 4 | `[sp-28] +0x0` | `ret(0x18aac)` |
| `ldr` at 0x8e998 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x8e99c | store | 4 | `sp -0x60` | `[[sp-28]]` |
| `ldr` at 0x8e9a0 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x8e9a4 | load | 4 | `sp -0x70` | `-` |
| `str` at 0x8e9ac | store | 4 | `sp -0x60` | `([[sp-28]] + [sp-112])` |
| `str` at 0x8e9b8 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x8e9c0 | store | 4 | `sp -0x54` | `0x0` |
| `ldr` at 0x8e9c4 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x8e9c8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x8e9d8 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x8e9e8 | store | 4 | `sp -0x70` | `0x0` |
| `ldr` at 0x8e9f0 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x8e9f8 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x8ea08 | store | 4 | `sp -0x70` | `0x2` |
| `str` at 0x8ea14 | store | 4 | `sp -0x70` | `0x1` |
| `str` at 0x8ea1c | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0x8ea20 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8ea2c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8ea30 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x8ea40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8ea48 | load | 4 | `abs 0x8f110` | `-` |
| `ldr` at 0x8ea4c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8ea5c | load | 4 | `(unknown)` | `-` |
| `str` at 0x8ea6c | store | 4 | `sp -0x24` | `ret(0xc97a8)` |
| `ldr` at 0x8ea70 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8ea80 | load | 4 | `sp -0x74` | `-` |
| `strb` at 0x8ea88 | store | 1 | `sp -0x80` | `?` |
| `ldr` at 0x8ea8c | load | 4 | `sp -0x74` | `-` |
| `strb` at 0x8ea94 | store | 1 | `sp -0x7f` | `?` |
| `ldr` at 0x8ea98 | load | 4 | `sp -0x74` | `-` |
| `strb` at 0x8eaa0 | store | 1 | `sp -0x7e` | `?` |
| `ldrb` at 0x8eaa4 | load | 1 | `sp -0x74` | `-` |
| `strb` at 0x8eaa8 | store | 1 | `sp -0x7d` | `([sp-116]:1 & 0xff)` |
| `ldr` at 0x8eaac | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x8eab4 | store | 1 | `sp -0x7c` | `?` |
| `ldr` at 0x8eab8 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x8eac0 | store | 1 | `sp -0x7b` | `?` |
| `ldr` at 0x8eac4 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x8eacc | store | 1 | `sp -0x7a` | `?` |
| `ldrb` at 0x8ead0 | load | 1 | `sp -0x78` | `-` |
| `strb` at 0x8ead4 | store | 1 | `sp -0x79` | `([sp-120]:1 & 0xff)` |
| `ldr` at 0x8ead8 | load | 4 | `abs 0x8f110` | `-` |
| `ldr` at 0x8eadc | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8eaf0 | load | 4 | `(unknown)` | `-` |
| `ldrls` at 0x8eaf8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8ed48 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x8ed50 | store | 4 | `sp -0x58` | `[sp-88]+1` |
| `ldr` at 0x8ed58 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x8ed60 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x8ed7c | load | 4 | `sp -0x54` | `-` |
| `str` at 0x8ed84 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x8ed90 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8ed9c | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x8eda0 | load | 4 | `abs 0x8f114` | `-` |
| `ldr` at 0x8eda4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8edac | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x8edbc | load | 4 | `sp -0x60` | `-` |
| `str` at 0x8edc4 | store | 4 | `sp -0x60` | `([sp-96] + ret(0x18a1c))` |
| `str` at 0x8edcc | store | 4 | `sp -0xc4` | `0x0` |
| `str` at 0x8edd8 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0x8eddc | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8ede0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x8edf0 | load | 4 | `abs 0x8f110` | `-` |
| `ldr` at 0x8edf4 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8ee08 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x8ee0c | load | 4 | `(unknown)` | `-` |
| `str` at 0x8ee14 | store | 4 | `sp -0x6c` | `?` |
| `str` at 0x8ee1c | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0x8ee20 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x8ee30 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8ee40 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x8ee58 | load | 4 | `abs 0x8f10c` | `-` |
| `ldr` at 0x8ee5c | load | 4 | `abs 0x11b314` | `-` |
| `ldr` at 0x8ee64 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8ee74 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x8ee7c | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0x8ee80 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8ee90 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x8eea8 | load | 4 | `abs 0x8f10c` | `-` |
| `ldr` at 0x8eeac | load | 4 | `abs 0x11b314` | `-` |
| `ldr` at 0x8eeb4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8eec4 | load | 4 | `sp -0x68` | `-` |
| `str` at 0x8eecc | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0x8eed0 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x8eed8 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x8eee0 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x8eee8 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x8eef0 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x8eefc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8ef08 | load | 4 | `sp -0x6c` | `-` |
| `str` at 0x8ef0c | store | 4 | `sp -0xcc` | `[sp-108]` |
| `ldr` at 0x8ef10 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x8ef14 | load | 4 | `abs 0x8f118` | `-` |
| `ldr` at 0x8ef18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8ef1c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x8ef24 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x8ef34 | load | 4 | `sp -0x60` | `-` |
| `str` at 0x8ef3c | store | 4 | `sp -0x60` | `([sp-96] + ret(0x18a1c))` |
| `ldr` at 0x8ef44 | load | 4 | `sp -0x68` | `-` |
| `str` at 0x8ef48 | store | 4 | `sp -0xcc` | `[sp-104]` |
| `ldr` at 0x8ef4c | load | 4 | `sp -0x6c` | `-` |
| `str` at 0x8ef50 | store | 4 | `sp -0xc8` | `[sp-108]` |
| `ldr` at 0x8ef54 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x8ef58 | load | 4 | `abs 0x8f11c` | `-` |
| `ldr` at 0x8ef5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8ef60 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x8ef68 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x8ef78 | load | 4 | `sp -0x60` | `-` |
| `str` at 0x8ef80 | store | 4 | `sp -0x60` | `([sp-96] + ret(0x18a1c))` |
| `str` at 0x8ef88 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0x8ef8c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8ef98 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8ef9c | load | 4 | `sp -0x48` | `-` |
| `str` at 0x8efb0 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0x8efb4 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x8efc4 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8efd4 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x8efec | load | 4 | `abs 0x8f10c` | `-` |
| `ldr` at 0x8eff0 | load | 4 | `abs 0x11b314` | `-` |
| `ldr` at 0x8eff8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8f00c | load | 4 | `abs 0x8f110` | `-` |
| `ldr` at 0x8f010 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8f024 | load | 4 | `abs 0x8f120` | `-` |
| `ldr` at 0x8f028 | load | 4 | `abs 0x8f110` | `-` |
| `ldr` at 0x8f02c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8f040 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8f050 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x8f058 | load | 4 | `abs 0x8f124` | `-` |
| `ldr` at 0x8f05c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8f060 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8f06c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8f07c | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x8f094 | load | 4 | `abs 0x8f10c` | `-` |
| `ldr` at 0x8f098 | load | 4 | `abs 0x11b314` | `-` |
| `ldr` at 0x8f0a0 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x8f0a4 | load | 4 | `abs 0x8f128` | `-` |
| `ldr` at 0x8f0ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8f0b4 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x8f0c4 | load | 4 | `sp -0x60` | `-` |
| `str` at 0x8f0cc | store | 4 | `sp -0x60` | `([sp-96] + ret(0x18a1c))` |
| `ldr` at 0x8f0d0 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x8f0d8 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x8f0e0 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x8f0e8 | store | 4 | `sp -0x58` | `?` |
| `str` at 0x8f0f4 | store | 4 | `sp -0xc4` | `0x0` |
| `ldr` at 0x8f0f8 | load | 4 | `sp -0xc4` | `-` |
| `ldmdb` at 0x8f0fc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x8f0fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8f0fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8f0fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8e85c | `(unknown callee) 0x18920` | `[0x282340]` | `0x282340` | `[0x282320]` | `0x282340` |
| `bl` at 0x8e884 | `(unknown callee) 0x18c20` | `sp-68` | `0x0` | `0x20` | `sp-68` |
| `bl` at 0x8e89c | `(unknown callee) 0x18c20` | `[0x11b314]` | `0x0` | `[0x282340]` | `0x11b314` |
| `bl` at 0x8e928 | `(unknown callee) 0x18920` | `?` | `?` | `?` | `?` |
| `bl` at 0x8e954 | `(unknown callee) 0x18a1c` | `[[sp-28]]` | `?` | `?` | `[sp-28]` |
| `bl` at 0x8e98c | `(unknown callee) 0x18aac` | `[[sp-28]]` | `?` | `?` | `?` |
| `bl` at 0x8e9b0 | `(unknown callee) 0xc2c9c` | `?` | `?` | `[[sp-28]]` | `?` |
| `bl` at 0x8ea64 | `(unknown callee) 0xc97a8` | `?` | `?` | `sp-116` | `sp-120` |
| `bl` at 0x8ed6c | `(unknown callee) 0xc2d08` | `?` | `?` | `?` | `[sp-76]` |
| `bl` at 0x8ed74 | `(unknown callee) 0x8f32c` | `0x64` | `?` | `?` | `?` |
| `bl` at 0x8ed78 | `(unknown callee) 0xc2c9c` | `ret(0x8f32c)` | `?` | `?` | `?` |
| `bl` at 0x8ed8c | `(unknown callee) 0xc2d08` | `?` | `?` | `?` | `[sp-76]` |
| `bl` at 0x8eda8 | `(unknown callee) 0x18c74` | `[sp-96]` | `0xfbc48` | `[sp-24]` | `[sp-36]` |
| `bl` at 0x8edb0 | `(unknown callee) 0x18a1c` | `[sp-96]` | `?` | `?` | `?` |
| `bl` at 0x8ef20 | `(unknown callee) 0x18c74` | `[sp-96]` | `0xfbc6c` | `[sp-24]` | `[sp-104]` |
| `bl` at 0x8ef28 | `(unknown callee) 0x18a1c` | `[sp-96]` | `?` | `?` | `?` |
| `bl` at 0x8ef64 | `(unknown callee) 0x18c74` | `[sp-96]` | `0xfbc94` | `[sp-24]` | `[sp-100]` |
| `bl` at 0x8ef6c | `(unknown callee) 0x18a1c` | `[sp-96]` | `?` | `?` | `?` |
| `bl` at 0x8f064 | `(unknown callee) 0x18c74` | `sp-192` | `0xfbcc0` | `?` | `?` |
| `bl` at 0x8f0b0 | `(unknown callee) 0x18c74` | `[sp-96]` | `0xfbcc8` | `sp-192` | `?` |
| `bl` at 0x8f0b8 | `(unknown callee) 0x18a1c` | `[sp-96]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x8e820 | `[0x11b310] != 0x0` | `[0x11b310] == 0x0` |
| `blt` at 0x8e8bc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x8e908 | `[[sp-28]] != 0x0` | `[[sp-28]] == 0x0` |
| `blt` at 0x8e9d0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x8e9e0 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x8ea00 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x8ea28 | `Z == 0` | `not (Z == 0)` |
| `blt` at 0x8ea38 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x8ea78 | `ret(0xc97a8) == 0x0` | `ret(0xc97a8) != 0x0` |
| `ldrls` at 0x8eaf8 | `? <= 0x8 (unsigned, 32-bit)` | `? > 0x8 (unsigned, 32-bit)` |
| `bge` at 0x8ed68 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x8ed98 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `blt` at 0x8ede8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `ble` at 0x8ee28 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x8eef8 | `[sp-100] != 0x0` | `[sp-100] == 0x0` |
| `beq` at 0x8ef04 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `bne` at 0x8ef94 | `[sp-32] != 0x2` | `[sp-32] == 0x2` |
| `blt` at 0x8efa4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `ble` at 0x8efbc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x8f000 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8f0fc | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x8eaf8).

## `exceptionResetReport`

- `exceptionResetReport`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1e23c to 0x1e24c.
  Established. This symbol contains only frame setup and restoration (`mov` at 0x1e23c, `push` at
  0x1e240, `sub` at 0x1e244, `ldmdb` at 0x1e248). Entry r0 survives as the reported return value
  (`ldmdb` at 0x1e248). No calls, conditional branches or reset-reporting operation are established
  by the body.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x1e248 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x1e248 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x1e248 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x1e248 | `r0` | `r0` |

## `_parse_cpu`

- `_parse_cpu`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x76cb4 to 0x76e50.
  Established. The call to `0x188e4` carries `[[sp-32]]:1`, unknown, unknown, `[[sp-32]]:1` (`bl` at
  0x76dd4); its result takes the lower fallback for `ret(0x188e4)-97 < 0x0` signed and falls through
  for greater-or-equal (`blt` at 0x76dec). The next bound takes the same fallback for
  `ret(0x188e4)-97 > 0x3` signed and falls through for less-or-equal (`bgt` at 0x76df8). The
  fallback calls `0x18c74` with `[sp-24]`, `0xf18a0`, unknown, unknown (`bl` at 0x76e34). The r0
  return is `[sp-44]`, while the accepted CPU notation and numeric encoding remain unknown (`ldmdb`
  at 0x76e44).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x76cc4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x76cc8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x76cd0 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x76d18 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0x76d1c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x76d20 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x76d24 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x76d28 | store | 4 | `sp -0x20` | `[sp-20]` |
| `str` at 0x76d30 | store | 4 | `sp -0x24` | `?` |
| `str` at 0x76d38 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x76d3c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x76d48 | store | 4 | `sp -0x34` | `ret(0x18a1c)` |
| `ldr` at 0x76d4c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x76d58 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x76d68 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x76d6c | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x76d74 | store | 4 | `sp -0x24` | `[[sp-32]]:1-48` |
| `ldr` at 0x76d78 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x76d84 | load | 4 | `abs 0x76e48` | `-` |
| `ldr` at 0x76d88 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x76d8c | load | 4 | `abs 0x27ee70` | `-` |
| `ldr` at 0x76d98 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x76da0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0x76da4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x76db0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x76dc4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x76dc8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x76dcc | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x76de0 | store | 4 | `sp -0x28` | `ret(0x188e4)-97` |
| `ldr` at 0x76de4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76df0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76dfc | load | 4 | `abs 0x76e48` | `-` |
| `ldr` at 0x76e00 | load | 4 | `abs 0x27ee60` | `-` |
| `ldr` at 0x76e08 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x76e10 | load | 4 | `abs 0x76e48` | `-` |
| `ldr` at 0x76e14 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76e18 | load | 4 | `abs 0x27ee60` | `-` |
| `str` at 0x76e24 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x76e2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x76e30 | load | 4 | `abs 0x76e4c` | `-` |
| `str` at 0x76e3c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x76e40 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x76e44 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x76e44 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x76e44 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x76cd4 | `(unknown callee) 0x2b31c` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x76ce8 | `(unknown callee) 0x2b31c` | `ret(0x2b31c)` | `?` | `?` | `?` |
| `bl` at 0x76cfc | `(unknown callee) 0x2b31c` | `ret(0x2b31c)` | `?` | `?` | `?` |
| `bl` at 0x76d40 | `(unknown callee) 0x18a1c` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x76dd4 | `(unknown callee) 0x188e4` | `[[sp-32]]:1` | `?` | `?` | `[[sp-32]]:1` |
| `bl` at 0x76e34 | `(unknown callee) 0x18c74` | `[sp-24]` | `0xf18a0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x76ce4 | `? == 0x35` | `? != 0x35` |
| `beq` at 0x76cf8 | `? == 0x40` | `? != 0x40` |
| `beq` at 0x76d0c | `? == 0x36` | `? != 0x36` |
| `beq` at 0x76d54 | `ret(0x18a1c) == 0x1` | `ret(0x18a1c) != 0x1` |
| `beq` at 0x76d60 | `ret(0x18a1c) == 0x2` | `ret(0x18a1c) != 0x2` |
| `blt` at 0x76d80 | `[[sp-32]]:1-48 < 0x0 (signed, 32-bit)` | `[[sp-32]]:1-48 >= 0x0 (signed, 32-bit)` |
| `bge` at 0x76d94 | `[[sp-32]]:1-48 >= [0x27ee70] (signed, 32-bit)` | `[[sp-32]]:1-48 < [0x27ee70] (signed, 32-bit)` |
| `bge` at 0x76dac | `[sp-36] >= 0x0 (signed, 32-bit)` | `[sp-36] < 0x0 (signed, 32-bit)` |
| `beq` at 0x76db8 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `blt` at 0x76dec | `ret(0x188e4)-97 < 0x0 (signed, 32-bit)` | `ret(0x188e4)-97 >= 0x0 (signed, 32-bit)` |
| `bgt` at 0x76df8 | `ret(0x188e4)-97 > 0x3 (signed, 32-bit)` | `ret(0x188e4)-97 <= 0x3 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x76e44 | `r0` | `[sp-44]` |

## `SocketNodelayMode`

- `SocketNodelayMode`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9da18 to 0x9daa4.
  Established. The initial branch takes its bypass for entry `r1 == 0x0` and falls through otherwise
  (`beq` at 0x9da38). The reached call to `0x18adc` carries entry r0, `0x4`, `0x800`, entry r1 (`bl`
  at 0x9da48); its zero result takes the same bypass and nonzero falls through to `0x188b4`
  (`beq` at 0x9da54, `bl` at 0x9da58). A later call to `0x18800` has `[sp-20]`, unknown, unknown,
  unknown (`bl` at 0x9da78). The socket-option effect and joined r0 return remain unknown (`ldmdb`
  at 0x9da94).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9da28 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9da2c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x9da30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9da3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9da60 | load | 4 | `abs 0x9da98` | `-` |
| `ldr` at 0x9da64 | load | 4 | `abs 0x9da9c` | `-` |
| `ldr` at 0x9da68 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0x9da6c | load | 4 | `abs 0x9daa0` | `-` |
| `ldr` at 0x9da74 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9da80 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x9da8c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x9da90 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x9da94 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9da94 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9da94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9da48 | `(unknown callee) 0x18adc` | `r0` | `0x4` | `0x800` | `r1` |
| `bl` at 0x9da58 | `(unknown callee) 0x188b4` | `ret(0x18adc)` | `?` | `?` | `ret(0x18adc)` |
| `bl` at 0x9da70 | `(unknown callee) 0x1e384` | `0xfffffe6c` | `0xfd30c` | `[ret(0x188b4)]` | `0xfd314` |
| `bl` at 0x9da78 | `(unknown callee) 0x18800` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x9da38 | `r1 == 0x0` | `r1 != 0x0` |
| `beq` at 0x9da54 | `ret(0x18adc) == 0x0` | `ret(0x18adc) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9da94 | `r0` | `?` |
