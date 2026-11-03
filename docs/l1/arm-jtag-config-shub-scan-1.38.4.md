# ARM JTAG configuration and SHUB scan paths in 1.38.4

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
No unresolved computed transfers were reported for these routines. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/jtag-config-shub-scan-facts.txt) retain all tool sections.

## `JTAG_Cfg_create`

- `JTAG_Cfg_create`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe6118 to 0xe6310.
  Established. Two word stores write `0x0` at `0x2ad26c` and `0x1` at `0x2ad268` (`str` at 0xe61ec,
  `str` at 0xe61f8). Four later word stores clear unknown targets (`str` at 0xe6250, `str` at
  0xe627c, `str` at 0xe62a8, `str` at 0xe62d4). The call to `0x1b400` has unknown, `0xc`, unknown,
  unknown (`bl` at 0xe61d0). The return reports r0 `0x1` (`ldmdb` at 0xe62f4). Configuration-record
  ownership and physical port meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe6128 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xe612c | load | 4 | `abs 0xe62f8` | `-` |
| `ldr` at 0xe6130 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xe613c | load | 4 | `abs 0xe62fc` | `-` |
| `str` at 0xe6140 | store | 4 | `sp -0x24` | `0x135940` |
| `ldr` at 0xe6144 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe6148 | store | 4 | `sp -0x20` | `r0` |
| `ldr` at 0xe6150 | load | 4 | `abs 0xe6300` | `-` |
| `str` at 0xe6164 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xe6168 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6178 | load | 4 | `abs 0xe6304` | `-` |
| `ldr` at 0xe617c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe619c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe61a8 | load | 4 | `abs 0xe6304` | `-` |
| `ldr` at 0xe61ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe61cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe61d4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe61dc | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xe61e4 | load | 4 | `abs 0xe6308` | `-` |
| `str` at 0xe61ec | store | 4 | `abs 0x2ad26c` | `0x0` |
| `ldr` at 0xe61f0 | load | 4 | `abs 0xe630c` | `-` |
| `str` at 0xe61f8 | store | 4 | `abs 0x2ad268` | `0x1` |
| `ldr` at 0xe61fc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe620c | store | 4 | `sp -0x1c` | `0x1` |
| `str` at 0xe6218 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xe621c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6220 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6230 | load | 4 | `abs 0xe6304` | `-` |
| `ldr` at 0xe6234 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6250 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe6254 | load | 4 | `abs 0xe6304` | `-` |
| `ldr` at 0xe6258 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe627c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe6280 | load | 4 | `abs 0xe6304` | `-` |
| `ldr` at 0xe6284 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe62a8 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe62ac | load | 4 | `abs 0xe6304` | `-` |
| `ldr` at 0xe62b0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe62d4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe62d8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe62e0 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0xe62ec | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0xe62f0 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xe62f4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe62f4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe62f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe615c | `(unknown callee) 0x21e28` | `0x6` | `0x135958` | `0x7b` | `0x0` |
| `bl` at 0xe61d0 | `(unknown callee) 0x1b400` | `?` | `0xc` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe6138 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `bls` at 0xe6170 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0xe61a4 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xe6204 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `blo` at 0xe6228 | `C == 0` | `not (C == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe62f4 | `r0` | `0x1` |

## `JTAG_Cfg_info_init`

- `JTAG_Cfg_info_init`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe6310 to 0xe6684.
  Established. The word store of `ret(0x1b124)` has unknown effective target (`str` at 0xe64cc). A
  halfword store and two byte stores later write `0x0` at unknown targets (`strh` at 0xe6558, `strb`
  at 0xe6594, `strb` at 0xe65d0). The call to `0x1b124` carries unknown, `0xc`, unknown, unknown
  (`bl` at 0xe64c4). Record dimensions and r0 at the common exit are unresolved (`ldmdb` at
  0xe666c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe6320 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe6324 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xe6328 | load | 4 | `abs 0xe6670` | `-` |
| `ldr` at 0xe632c | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xe6338 | load | 4 | `abs 0xe6674` | `-` |
| `str` at 0xe633c | store | 4 | `sp -0x30` | `0x135974` |
| `ldr` at 0xe6340 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6344 | store | 4 | `sp -0x2c` | `r0` |
| `ldr` at 0xe6348 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe634c | store | 4 | `sp -0x28` | `r1` |
| `ldr` at 0xe6354 | load | 4 | `abs 0xe6678` | `-` |
| `ldr` at 0xe6364 | load | 4 | `abs 0xe667c` | `-` |
| `ldr` at 0xe6368 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6388 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe638c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe6398 | load | 4 | `abs 0xe667c` | `-` |
| `ldr` at 0xe639c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe63c0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe63c4 | load | 4 | `abs 0xe667c` | `-` |
| `ldr` at 0xe63c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe63e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe63f4 | load | 4 | `abs 0xe667c` | `-` |
| `ldr` at 0xe63f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6418 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe6420 | load | 4 | `abs 0xe667c` | `-` |
| `ldr` at 0xe6424 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6448 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe644c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe645c | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0xe6464 | load | 4 | `abs 0xe667c` | `-` |
| `ldr` at 0xe6468 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6488 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe6494 | load | 4 | `abs 0xe667c` | `-` |
| `ldr` at 0xe6498 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe64b8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe64cc | store | 4 | `(unknown)` | `ret(0x1b124)` |
| `ldr` at 0xe64d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe64dc | load | 4 | `abs 0xe6680` | `-` |
| `str` at 0xe64e0 | store | 4 | `sp -0x30` | `0x13599c` |
| `ldr` at 0xe64e8 | load | 4 | `abs 0xe6678` | `-` |
| `str` at 0xe64fc | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xe6508 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xe650c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe6510 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe6520 | load | 4 | `abs 0xe667c` | `-` |
| `ldr` at 0xe6524 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6544 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe654c | load | 4 | `(unknown)` | `-` |
| `strh` at 0xe6558 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0xe655c | load | 4 | `abs 0xe667c` | `-` |
| `ldr` at 0xe6560 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6580 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe6588 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xe6594 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xe6598 | load | 4 | `abs 0xe667c` | `-` |
| `ldr` at 0xe659c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe65bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe65c4 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xe65d0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xe65d4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe65dc | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe65e4 | load | 4 | `abs 0xe667c` | `-` |
| `ldr` at 0xe65e8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6604 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe6608 | load | 4 | `abs 0xe667c` | `-` |
| `ldr` at 0xe660c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6630 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe6634 | load | 4 | `abs 0xe667c` | `-` |
| `ldr` at 0xe6638 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6658 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe665c | store | 4 | `(unknown)` | `[sp-28]` |
| `str` at 0xe6664 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0xe6668 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xe666c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe666c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe666c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe666c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe6360 | `(unknown callee) 0x21e28` | `0x6` | `0x135958` | `0xa3` | `0x0` |
| `bl` at 0xe641c | `(unknown callee) 0x1b400` | `?` | `0xc` | `[sp-24]` | `?` |
| `bl` at 0xe64c4 | `(unknown callee) 0x1b124` | `?` | `0xc` | `?` | `?` |
| `bl` at 0xe64f4 | `(unknown callee) 0x21e28` | `0x3` | `0x135958` | `0xb7` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xe6334 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xe6394 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xe63f0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xe6454 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xe6490 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe64d8 | `? != 0x0` | `? == 0x0` |
| `blo` at 0xe6518 | `C == 0` | `not (C == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe666c | `r0` | `?` |

## `JTAG_Cfg_port_init`

- `JTAG_Cfg_port_init`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe6684 to 0xe6a88.
  Established. The entry test takes its continuation for `r2 != 0x0` and falls through on zero
  (`bne` at 0xe66ac). Two byte stores in the indexed body have unknown targets and values (`strb` at
  0xe6918, `strb` at 0xe6964). The final call to `0xe974c` uses `[sp-24]` and three unknown
  candidates (`bl` at 0xe6a58). Port-record encoding and the merged r0 remain unproved (`ldmdb` at
  0xe6a68).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe6694 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe6698 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xe669c | store | 4 | `sp -0x20` | `r2` |
| `strb` at 0xe66a0 | store | 1 | `sp -0x21` | `(r3 & 0xff)` |
| `ldr` at 0xe66a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe66b0 | load | 4 | `abs 0xe6a6c` | `-` |
| `str` at 0xe66b4 | store | 4 | `sp -0x38` | `0x1359cc` |
| `ldr` at 0xe66bc | load | 4 | `abs 0xe6a70` | `-` |
| `ldr` at 0xe66cc | load | 4 | `abs 0xe6a74` | `-` |
| `ldr` at 0xe66d0 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xe66dc | load | 4 | `abs 0xe6a78` | `-` |
| `str` at 0xe66e0 | store | 4 | `sp -0x38` | `0x1359f8` |
| `ldr` at 0xe66e4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe66e8 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe66ec | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe66f0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe66f8 | load | 4 | `abs 0xe6a70` | `-` |
| `ldr` at 0xe6708 | load | 4 | `abs 0xe6a7c` | `-` |
| `ldr` at 0xe670c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6728 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe672c | load | 4 | `abs 0xe6a7c` | `-` |
| `ldr` at 0xe6730 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6750 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe6754 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe6760 | load | 4 | `abs 0xe6a7c` | `-` |
| `ldr` at 0xe6764 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6788 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe678c | load | 4 | `abs 0xe6a7c` | `-` |
| `ldr` at 0xe6790 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe67b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe67bc | load | 4 | `abs 0xe6a7c` | `-` |
| `ldr` at 0xe67c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe67e0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe67e8 | load | 4 | `abs 0xe6a7c` | `-` |
| `ldr` at 0xe67ec | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6810 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe6814 | load | 4 | `abs 0xe6a7c` | `-` |
| `ldr` at 0xe6818 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6838 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe6844 | load | 4 | `abs 0xe6a7c` | `-` |
| `ldr` at 0xe6848 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6868 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe687c | store | 4 | `(unknown)` | `ret(0x1b124)` |
| `ldr` at 0xe6880 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe688c | load | 4 | `abs 0xe6a80` | `-` |
| `str` at 0xe6890 | store | 4 | `sp -0x38` | `0x13599c` |
| `ldr` at 0xe6898 | load | 4 | `abs 0xe6a70` | `-` |
| `ldr` at 0xe68a8 | load | 4 | `abs 0xe6a84` | `-` |
| `str` at 0xe68ac | store | 4 | `sp -0x2c` | `0xfffff7cc` |
| `str` at 0xe68b8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe68bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe68c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe68d0 | load | 4 | `abs 0xe6a7c` | `-` |
| `ldr` at 0xe68d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe68f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe68fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe6904 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe690c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xe6914 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xe6918 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe691c | load | 4 | `abs 0xe6a7c` | `-` |
| `ldr` at 0xe6920 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6940 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe6948 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe6950 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe6958 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xe6960 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xe6964 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe6968 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe6970 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xe6978 | load | 4 | `abs 0xe6a7c` | `-` |
| `ldr` at 0xe697c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6998 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0xe699c | load | 4 | `abs 0xe6a7c` | `-` |
| `ldr` at 0xe69a0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe69c4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe69c8 | load | 4 | `abs 0xe6a7c` | `-` |
| `ldr` at 0xe69cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe69ec | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe69f0 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0xe69f4 | load | 4 | `abs 0xe6a7c` | `-` |
| `ldr` at 0xe69f8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xe6a18 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0xe6a1c | store | 1 | `(unknown)` | `([sp-33]:1 & 0xff)` |
| `ldr` at 0xe6a20 | load | 4 | `abs 0xe6a7c` | `-` |
| `ldr` at 0xe6a24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6a44 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xe6a48 | store | 4 | `(unknown)` | `[sp]` |
| `ldr` at 0xe6a4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6a54 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6a60 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe6a64 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xe6a68 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe6a68 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe6a68 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe6a68 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe66c8 | `(unknown callee) 0x21e28` | `0x9` | `0x135958` | `0xd4` | `0x0` |
| `bl` at 0xe6704 | `(unknown callee) 0x21e28` | `0x6` | `0x135958` | `0xd7` | `0x0` |
| `bl` at 0xe67e4 | `(unknown callee) 0x1b400` | `?` | `0xc` | `[sp-24]` | `?` |
| `bl` at 0xe6874 | `(unknown callee) 0x1b124` | `?` | `0xc` | `[sp-24]` | `?` |
| `bl` at 0xe68a4 | `(unknown callee) 0x21e28` | `0x3` | `0x135958` | `0xec` | `0x0` |
| `bl` at 0xe6a50 | `(unknown callee) 0xe6a88` | `[sp-24]` | `0x14` | `?` | `[sp]` |
| `bl` at 0xe6a58 | `(unknown callee) 0xe974c` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe66ac | `r2 != 0x0` | `r2 == 0x0` |
| `bls` at 0xe66d8 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xe675c | `? == [sp-32]` | `? != [sp-32]` |
| `beq` at 0xe67b8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xe6840 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe6888 | `? != 0x0` | `? == 0x0` |
| `blo` at 0xe68c8 | `C == 0` | `not (C == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe6a68 | `r0` | `?` |

## `JTAG_Cfg_calc_ir_vals`

- `JTAG_Cfg_calc_ir_vals`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe6a88 to 0xe6c10.
  Established. The halfword store writes `([sp-28]:2 & 0xffff)` at an unknown address (`strh` at
  0xe6b50). The unsigned global comparison takes its bypass for `[0x2ad25c] <= 0x3` and falls
  through for greater (`bls` at 0xe6be0). Fall-through includes an unknown-target call with
  `0x2b38c0`, `0x4`, unknown, `[0x142b54]` (`mov` at 0xe6bf0). Instruction-register field meaning
  and r0 at exit are unknown (`ldmdb` at 0xe6bf4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe6a98 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xe6a9c | load | 4 | `abs 0xe6bf8` | `-` |
| `ldr` at 0xe6aa0 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xe6aac | load | 4 | `abs 0xe6bfc` | `-` |
| `str` at 0xe6ab0 | store | 4 | `sp -0x24` | `0x135a20` |
| `ldr` at 0xe6ab4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe6ab8 | store | 4 | `sp -0x20` | `r0` |
| `ldr` at 0xe6ac0 | load | 4 | `abs 0xe6c00` | `-` |
| `ldr` at 0xe6ac4 | load | 4 | `abs 0xe6c04` | `-` |
| `str` at 0xe6ad4 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0xe6adc | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xe6ae0 | load | 4 | `abs 0xe6c08` | `-` |
| `ldr` at 0xe6ae4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6b04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6b08 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe6b18 | load | 4 | `abs 0xe6c08` | `-` |
| `ldr` at 0xe6b1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6b3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6b44 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe6b4c | load | 2 | `sp -0x1c` | `-` |
| `strh` at 0xe6b50 | store | 2 | `(unknown)` | `([sp-28]:2 & 0xffff)` |
| `ldr` at 0xe6b54 | load | 4 | `abs 0xe6c08` | `-` |
| `ldr` at 0xe6b58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6b78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6b80 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xe6b88 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xe6b8c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe6b94 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xe6b98 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6ba0 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0xe6ba8 | load | 4 | `abs 0xe6c08` | `-` |
| `ldr` at 0xe6bac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6bcc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe6bd0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe6bd4 | load | 4 | `abs 0xe6bf8` | `-` |
| `ldr` at 0xe6bd8 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xe6be4 | load | 4 | `abs 0xe6c0c` | `-` |
| `ldr` at 0xe6be8 | load | 4 | `abs 0x142b54` | `-` |
| `ldmdb` at 0xe6bf4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe6bf4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe6bf4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe6acc | `(unknown callee) 0x21e28` | `0x6` | `0x135958` | `0x113` | `0x0` |
| `mov` at 0xe6bf0 | `(unknown callee) (unknown target)` | `0x2b38c0` | `0x4` | `?` | `[0x142b54]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xe6aa8 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `blo` at 0xe6b10 | `C == 0` | `not (C == 0)` |
| `bls` at 0xe6be0 | `[0x2ad25c] <= 0x3 (unsigned, 32-bit)` | `[0x2ad25c] > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe6bf4 | `r0` | `?` |

## `JTAG_Cfg_set_ports_f`

- `JTAG_Cfg_set_ports_f`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe6c10 to 0xe6e08.
  Established. Word stores write an unknown value at `0x2ad268` and `[sp-24]` at `0x2ad26c` (`str`
  at 0xe6d60, `str` at 0xe6d6c). The call to `0xe32fc` has `[0x2ad268]:1`, `[0x2ad26c]:1`,
  `[0x2ad268]:1`, `[0x2ad26c]:1` (`bl` at 0xe6d88). The prior comparison takes its branch for
  `[0x2ad26c] != [sp-24]` and falls through on equality (`bne` at 0xe6cd8). Physical mux selection
  and the joined return value remain unidentified (`ldmdb` at 0xe6ddc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe6c20 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe6c24 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xe6c28 | load | 4 | `abs 0xe6de0` | `-` |
| `ldr` at 0xe6c2c | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xe6c38 | load | 4 | `abs 0xe6de4` | `-` |
| `str` at 0xe6c3c | store | 4 | `sp -0x38` | `0x135a40` |
| `ldr` at 0xe6c40 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe6c44 | store | 4 | `sp -0x34` | `r0` |
| `ldr` at 0xe6c48 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6c4c | store | 4 | `sp -0x30` | `r1` |
| `ldr` at 0xe6c54 | load | 4 | `abs 0xe6de8` | `-` |
| `ldr` at 0xe6c58 | load | 4 | `abs 0xe6dec` | `-` |
| `ldr` at 0xe6c64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6c70 | load | 4 | `abs 0xe6df0` | `-` |
| `ldr` at 0xe6c74 | load | 4 | `abs 0x2ad26c` | `-` |
| `str` at 0xe6c78 | store | 4 | `sp -0x18` | `[0x2ad26c]` |
| `ldr` at 0xe6c7c | load | 4 | `abs 0xe6df4` | `-` |
| `ldr` at 0xe6c80 | load | 4 | `abs 0x2ad268` | `-` |
| `str` at 0xe6c84 | store | 4 | `sp -0x14` | `[0x2ad268]` |
| `ldr` at 0xe6c8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6c9c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6ca4 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xe6ca8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe6cb0 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xe6cb4 | load | 4 | `abs 0xe6df4` | `-` |
| `ldr` at 0xe6cb8 | load | 4 | `abs 0x2ad268` | `-` |
| `ldr` at 0xe6cbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6cc8 | load | 4 | `abs 0xe6df0` | `-` |
| `ldr` at 0xe6ccc | load | 4 | `abs 0x2ad26c` | `-` |
| `ldr` at 0xe6cd0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6ce0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe6ce8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6cf4 | load | 4 | `abs 0xe6df8` | `-` |
| `str` at 0xe6cf8 | store | 4 | `sp -0x38` | `0x135a74` |
| `ldr` at 0xe6cfc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6d00 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe6d04 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe6d08 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe6d10 | load | 4 | `abs 0xe6de8` | `-` |
| `ldr` at 0xe6d14 | load | 4 | `abs 0xe6dfc` | `-` |
| `ldr` at 0xe6d20 | load | 4 | `abs 0xe6e00` | `-` |
| `str` at 0xe6d24 | store | 4 | `sp -0x24` | `0xfffff7c3` |
| `ldr` at 0xe6d2c | load | 4 | `abs 0xe6df4` | `-` |
| `str` at 0xe6d30 | store | 4 | `sp -0x28` | `0x2ad268` |
| `ldr` at 0xe6d34 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe6d38 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe6d3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6d4c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6d54 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe6d58 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe6d5c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe6d60 | store | 4 | `abs 0x2ad268` | `?` |
| `ldr` at 0xe6d64 | load | 4 | `abs 0xe6df0` | `-` |
| `ldr` at 0xe6d68 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6d6c | store | 4 | `abs 0x2ad26c` | `[sp-24]` |
| `ldr` at 0xe6d70 | load | 4 | `abs 0xe6df4` | `-` |
| `ldrb` at 0xe6d74 | load | 1 | `abs 0x2ad268` | `-` |
| `ldr` at 0xe6d78 | load | 4 | `abs 0xe6df0` | `-` |
| `ldrb` at 0xe6d7c | load | 1 | `abs 0x2ad26c` | `-` |
| `ldr` at 0xe6d8c | load | 4 | `abs 0xe6de0` | `-` |
| `ldr` at 0xe6d90 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xe6d9c | load | 4 | `abs 0xe6e04` | `-` |
| `str` at 0xe6da0 | store | 4 | `sp -0x38` | `0x135ab4` |
| `ldr` at 0xe6da4 | load | 4 | `abs 0xe6df4` | `-` |
| `ldr` at 0xe6da8 | load | 4 | `abs 0x2ad268` | `-` |
| `str` at 0xe6dac | store | 4 | `sp -0x34` | `[0x2ad268]` |
| `ldr` at 0xe6db0 | load | 4 | `abs 0xe6df0` | `-` |
| `ldr` at 0xe6db4 | load | 4 | `abs 0x2ad26c` | `-` |
| `str` at 0xe6db8 | store | 4 | `sp -0x30` | `[0x2ad26c]` |
| `ldr` at 0xe6dc0 | load | 4 | `abs 0xe6de8` | `-` |
| `str` at 0xe6dd4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe6dd8 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xe6ddc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe6ddc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe6ddc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe6c60 | `(unknown callee) 0x21e28` | `0x6` | `0x135958` | `0x12a` | `0x0` |
| `bl` at 0xe6d1c | `(unknown callee) 0x21e28` | `0x3` | `0x135958` | `0x141` | `0x0` |
| `bl` at 0xe6d88 | `(unknown callee) 0xe32fc` | `[0x2ad268]:1` | `[0x2ad26c]:1` | `[0x2ad268]:1` | `[0x2ad26c]:1` |
| `bl` at 0xe6dcc | `(unknown callee) 0x21e28` | `0x6` | `0x135958` | `0x160` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xe6c34 | `[0x2ad25c] <= 0x3 (unsigned, 32-bit)` | `[0x2ad25c] > 0x3 (unsigned, 32-bit)` |
| `bne` at 0xe6c6c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xe6c94 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bne` at 0xe6cc4 | `[0x2ad268] != [sp-20]` | `[0x2ad268] == [sp-20]` |
| `bne` at 0xe6cd8 | `[0x2ad26c] != [sp-24]` | `[0x2ad26c] == [sp-24]` |
| `bls` at 0xe6cf0 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xe6d44 | `Z == 0` | `not (Z == 0)` |
| `bls` at 0xe6d98 | `[0x2ad25c] <= 0x3 (unsigned, 32-bit)` | `[0x2ad25c] > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe6ddc | `r0` | `?` |

## `JTAG_Cfg_find_chip`

- `JTAG_Cfg_find_chip`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe6e08 to 0xe70a0.
  Established. The two byte reads in the search bodies have unknown effective addresses (`ldrb` at
  0xe6f8c, `ldrb` at 0xe7034). The sole call reaches `0x21e28` with `0x6`, `0x135958`, `0x17b`,
  `0x0` (`bl` at 0xe6e70). The exit reports r0 `[sp-44]` (`ldmdb` at 0xe7088). Chip-matching rules
  and indexed record meanings remain unestablished.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe6e18 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe6e1c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe6e20 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe6e24 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xe6e28 | load | 4 | `abs 0xe708c` | `-` |
| `ldr` at 0xe6e2c | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xe6e38 | load | 4 | `abs 0xe7090` | `-` |
| `str` at 0xe6e3c | store | 4 | `sp -0x40` | `0x135ae8` |
| `ldr` at 0xe6e40 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe6e44 | store | 4 | `sp -0x3c` | `r0` |
| `ldr` at 0xe6e48 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6e4c | store | 4 | `sp -0x38` | `r1` |
| `ldr` at 0xe6e50 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe6e54 | store | 4 | `sp -0x34` | `r2` |
| `ldr` at 0xe6e58 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe6e5c | store | 4 | `sp -0x30` | `r3` |
| `ldr` at 0xe6e64 | load | 4 | `abs 0xe7094` | `-` |
| `ldr` at 0xe6e68 | load | 4 | `abs 0xe7098` | `-` |
| `ldr` at 0xe6e74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6e80 | load | 4 | `abs 0xe709c` | `-` |
| `ldr` at 0xe6e84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6ea4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe6eb4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe6ebc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe6ec8 | load | 4 | `abs 0xe709c` | `-` |
| `ldr` at 0xe6ecc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6eec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe6ef0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe6f04 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe6f0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6f18 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe6f1c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe6f20 | load | 4 | `abs 0xe709c` | `-` |
| `ldr` at 0xe6f24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6f44 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe6f48 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe6f58 | load | 4 | `abs 0xe709c` | `-` |
| `ldr` at 0xe6f5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6f7c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe6f84 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xe6f8c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xe6f90 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe6f9c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe6fa0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe6fa8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe6fb0 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xe6fbc | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xe6fc4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe6fc8 | load | 4 | `abs 0xe709c` | `-` |
| `ldr` at 0xe6fcc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6fec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe6ff0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe7000 | load | 4 | `abs 0xe709c` | `-` |
| `ldr` at 0xe7004 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7024 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe702c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xe7034 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xe7038 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe7044 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7048 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7054 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe7058 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe7060 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe7068 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe706c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe7074 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xe7080 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe7084 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xe7088 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe7088 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe7088 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe6e70 | `(unknown callee) 0x21e28` | `0x6` | `0x135958` | `0x17b` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xe6e34 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bhi` at 0xe6e7c | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0xe6eac | `? != 0x0` | `? == 0x0` |
| `blt` at 0xe6ec4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bhs` at 0xe6ef8 | `C == 1` | `not (C == 1)` |
| `bne` at 0xe6f14 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0xe6f50 | `C == 0` | `not (C == 0)` |
| `bne` at 0xe6f98 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0xe6ff8 | `C == 0` | `not (C == 0)` |
| `bne` at 0xe7040 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xe7050 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe7088 | `r0` | `[sp-44]` |

## `Daytona_JTAG_Sel_Query`

- `Daytona_JTAG_Sel_Query`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x35dbc to 0x35e88.
  Established. The first two calls to `0xd6c2c` use `0x0`, `0x8`, entry r2, entry r3 and `0x0`,
  `0xc`, unknown, `ret(0xd6c2c)`, respectively (`bl` at 0x35dd8, `bl` at 0x35dec). Separate byte
  stores save `(ret(0xd6c2c) & 0xff)` at `sp -0x15` and `sp -0x16` (`strb` at 0x35de0, `strb` at
  0x35df4). The exit propagates `ret(0x1b22c)` (`ldmdb` at 0x35e74). Query-result signal meanings
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x35dcc | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x35de0 | store | 1 | `sp -0x15` | `(ret(0xd6c2c) & 0xff)` |
| `strb` at 0x35df4 | store | 1 | `sp -0x16` | `(ret(0xd6c2c) & 0xff)` |
| `ldrsb` at 0x35df8 | load | 1 | `sp -0x15` | `-` |
| `ldrsb` at 0x35e04 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x35e10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x35e14 | load | 4 | `abs 0x35e78` | `-` |
| `ldr` at 0x35e20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x35e24 | load | 4 | `abs 0x35e7c` | `-` |
| `ldrsb` at 0x35e30 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x35e3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x35e40 | load | 4 | `abs 0x35e80` | `-` |
| `ldr` at 0x35e4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x35e50 | load | 4 | `abs 0x35e84` | `-` |
| `ldr` at 0x35e58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x35e68 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x35e70 | store | 4 | `sp -0x14` | `([sp-20] + ret(0x1b22c))` |
| `ldmdb` at 0x35e74 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x35e74 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x35e74 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x35dd8 | `(unknown callee) 0xd6c2c` | `0x0` | `0x8` | `r2` | `r3` |
| `bl` at 0x35dec | `(unknown callee) 0xd6c2c` | `0x0` | `0xc` | `?` | `ret(0xd6c2c)` |
| `bl` at 0x35e18 | `(unknown callee) 0x1b49c` | `[sp-20]` | `0x1069fc` | `?` | `?` |
| `bl` at 0x35e28 | `(unknown callee) 0x1b49c` | `[sp-20]` | `0x106a14` | `?` | `?` |
| `bl` at 0x35e44 | `(unknown callee) 0x1b49c` | `[sp-20]` | `0x106a28` | `?` | `?` |
| `bl` at 0x35e54 | `(unknown callee) 0x1b49c` | `[sp-20]` | `0x106a40` | `?` | `?` |
| `bl` at 0x35e5c | `(unknown callee) 0x1b22c` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x35e00 | `? >= 0x0 (signed, 32-bit)` | `? < 0x0 (signed, 32-bit)` |
| `bge` at 0x35e0c | `? >= 0x0 (signed, 32-bit)` | `? < 0x0 (signed, 32-bit)` |
| `bge` at 0x35e38 | `? >= 0x0 (signed, 32-bit)` | `? < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x35e74 | `r0` | `ret(0x1b22c)` |

## `Daytona_JTAG_Sel`

- `Daytona_JTAG_Sel`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x35e88 to 0x35f7c.
  Established. The two calls to `0xd6a1c` have `0x0`, `0x8`, `0x80`, `[sp-17]:1` and `0x0`, `0xc`,
  `0x80`, `[sp-18]:1`, respectively (`bl` at 0x35f0c, `bl` at 0x35f30). The second result takes the
  continuation on zero and falls through on nonzero (`beq` at 0x35f3c). The return reports r0
  `[sp-19]:1` (`ldmdb` at 0x35f78). Selection polarity and physical JTAG routing are not
  established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x35ea0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x35ea8 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldrb` at 0x35eac | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x35ebc | store | 1 | `sp -0x15` | `?` |
| `strb` at 0x35ec8 | store | 1 | `sp -0x15` | `0x0` |
| `ldrb` at 0x35ecc | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x35ed0 | store | 1 | `sp -0x11` | `([sp-21]:1 & 0xff)` |
| `ldrb` at 0x35ed4 | load | 1 | `sp -0x12` | `-` |
| `strb` at 0x35ee4 | store | 1 | `sp -0x16` | `?` |
| `strb` at 0x35ef0 | store | 1 | `sp -0x16` | `0x0` |
| `ldrb` at 0x35ef4 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x35ef8 | store | 1 | `sp -0x12` | `([sp-22]:1 & 0xff)` |
| `ldrb` at 0x35efc | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x35f14 | store | 1 | `sp -0x13` | `(ret(0xd6a1c) & 0xff)` |
| `strb` at 0x35f1c | store | 1 | `sp -0x17` | `0x0` |
| `ldrb` at 0x35f20 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0x35f40 | load | 1 | `sp -0x13` | `-` |
| `strb` at 0x35f50 | store | 1 | `sp -0x17` | `0x1` |
| `ldrb` at 0x35f54 | load | 1 | `sp -0x17` | `-` |
| `strb` at 0x35f58 | store | 1 | `sp -0x14` | `([sp-23]:1 & 0xff)` |
| `ldrb` at 0x35f68 | load | 1 | `sp -0x14` | `-` |
| `strb` at 0x35f6c | store | 1 | `sp -0x13` | `(([sp-23]:1 & 0xff) & 0xff)` |
| `ldrb` at 0x35f70 | load | 1 | `sp -0x13` | `-` |
| `ldmdb` at 0x35f78 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x35f78 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x35f78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x35f0c | `(unknown callee) 0xd6a1c` | `0x0` | `0x8` | `0x80` | `[sp-17]:1` |
| `bl` at 0x35f30 | `(unknown callee) 0xd6a1c` | `0x0` | `0xc` | `0x80` | `[sp-18]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x35eb4 | `(r0 & 0xff) == 0x0` | `(r0 & 0xff) != 0x0` |
| `beq` at 0x35edc | `[sp-18]:1 == 0x0` | `[sp-18]:1 != 0x0` |
| `beq` at 0x35f3c | `ret(0xd6a1c) == 0x0` | `ret(0xd6a1c) != 0x0` |
| `bne` at 0x35f48 | `[sp-19]:1 != 0x0` | `[sp-19]:1 == 0x0` |
| `beq` at 0x35f64 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x35f78 | `r0` | `[sp-19]:1` |

## `SHUB_Reset_PreRelease`

- `SHUB_Reset_PreRelease`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4d358 to 0x4d8a0.
  Established. Two calls to `0xeac50` carry `[sp-112]`, `0x60`, `sp-28`, `0x0` and `[sp-112]`,
  `0x62`, `sp-28`, `0x0`, respectively (`bl` at 0x4d590, `bl` at 0x4d5f0). Each zero result takes
  its continuation and nonzero falls through (`beq` at 0x4d59c, `beq` at 0x4d5fc). A later call to
  that target uses `[sp-112]`, `0x6`, `[0x148070]`, `0x0` (`bl` at 0x4d680). Reset preparation
  semantics and the common r0 result remain unknown (`ldmdb` at 0x4d828).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4d36c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x4d374 | store | 4 | `sp -0x18` | `0x0` |
| `ldrb` at 0x4d378 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x4d380 | load | 4 | `abs 0x4d82c` | `-` |
| `ldr` at 0x4d384 | load | 4 | `abs 0x148058` | `-` |
| `ldr` at 0x4d38c | load | 4 | `(unknown)` | `-` |
| `str` at 0x4d390 | store | 4 | `sp -0x70` | `?` |
| `ldrb` at 0x4d394 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4d3a4 | store | 4 | `sp -0x18` | `ret(0x4bd0c)` |
| `ldr` at 0x4d3a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4d3b4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4d3b8 | store | 4 | `sp -0x74` | `ret(0x4bd0c)` |
| `ldrb` at 0x4d3c0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4d3d0 | store | 4 | `sp -0x18` | `ret(0x4c124)` |
| `ldr` at 0x4d3d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4d3e0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4d3e4 | store | 4 | `sp -0x74` | `ret(0x4c124)` |
| `ldr` at 0x4d40c | load | 4 | `abs 0x4d830` | `-` |
| `str` at 0x4d410 | store | 4 | `sp -0x74` | `0xfffff883` |
| `ldrb` at 0x4d418 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4d42c | store | 4 | `sp -0x18` | `ret(0x4b288)` |
| `ldr` at 0x4d430 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4d43c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4d440 | store | 4 | `sp -0x74` | `ret(0x4b288)` |
| `ldrb` at 0x4d448 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4d458 | store | 4 | `sp -0x18` | `ret(0x492bc)` |
| `ldr` at 0x4d45c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4d468 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4d46c | store | 4 | `sp -0x74` | `ret(0x492bc)` |
| `ldr` at 0x4d474 | load | 4 | `abs 0x4d834` | `-` |
| `ldr` at 0x4d478 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x4d484 | load | 4 | `abs 0x4d838` | `-` |
| `str` at 0x4d488 | store | 4 | `sp -0x7c` | `0x10a254` |
| `ldr` at 0x4d490 | load | 4 | `abs 0x4d83c` | `-` |
| `ldr` at 0x4d494 | load | 4 | `abs 0x4d840` | `-` |
| `str` at 0x4d4a4 | store | 4 | `sp -0x1c` | `0x10` |
| `ldr` at 0x4d4a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d4ac | load | 4 | `abs 0x4d844` | `-` |
| `ldr` at 0x4d4b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d4c0 | load | 4 | `abs 0x4d844` | `-` |
| `str` at 0x4d4cc | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x4d4d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d4d4 | load | 4 | `abs 0x4d848` | `-` |
| `ldr` at 0x4d4e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d4e8 | load | 4 | `abs 0x4d848` | `-` |
| `str` at 0x4d4f4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x4d4f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d4fc | load | 4 | `abs 0x4d84c` | `-` |
| `ldr` at 0x4d508 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d510 | load | 4 | `abs 0x4d84c` | `-` |
| `str` at 0x4d51c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x4d520 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d530 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4d544 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x4d548 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d558 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4d56c | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x4d578 | store | 4 | `sp -0x7c` | `0xa` |
| `ldr` at 0x4d57c | load | 4 | `abs 0x4d850` | `-` |
| `str` at 0x4d580 | store | 4 | `sp -0x78` | `0x10a268` |
| `ldr` at 0x4d584 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x4d5a0 | load | 4 | `abs 0x4d854` | `-` |
| `str` at 0x4d5a4 | store | 4 | `sp -0x7c` | `0x10a278` |
| `ldrb` at 0x4d5a8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4d5ac | store | 4 | `sp -0x78` | `[sp-17]:1` |
| `ldr` at 0x4d5b4 | load | 4 | `abs 0x4d83c` | `-` |
| `ldr` at 0x4d5b8 | load | 4 | `abs 0x4d858` | `-` |
| `ldr` at 0x4d5c4 | load | 4 | `abs 0x4d85c` | `-` |
| `str` at 0x4d5c8 | store | 4 | `sp -0x74` | `0xfffff894` |
| `str` at 0x4d5d8 | store | 4 | `sp -0x7c` | `0xa` |
| `ldr` at 0x4d5dc | load | 4 | `abs 0x4d860` | `-` |
| `str` at 0x4d5e0 | store | 4 | `sp -0x78` | `0x10a2a8` |
| `ldr` at 0x4d5e4 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x4d600 | load | 4 | `abs 0x4d864` | `-` |
| `str` at 0x4d604 | store | 4 | `sp -0x7c` | `0x10a2b8` |
| `ldrb` at 0x4d608 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4d60c | store | 4 | `sp -0x78` | `[sp-17]:1` |
| `ldr` at 0x4d614 | load | 4 | `abs 0x4d83c` | `-` |
| `ldr` at 0x4d618 | load | 4 | `abs 0x4d868` | `-` |
| `ldr` at 0x4d624 | load | 4 | `abs 0x4d85c` | `-` |
| `str` at 0x4d628 | store | 4 | `sp -0x74` | `0xfffff894` |
| `ldr` at 0x4d630 | load | 4 | `abs 0x4d834` | `-` |
| `ldr` at 0x4d634 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x4d640 | load | 4 | `abs 0x4d86c` | `-` |
| `str` at 0x4d644 | store | 4 | `sp -0x7c` | `0x10a2e8` |
| `ldr` at 0x4d64c | load | 4 | `abs 0x4d83c` | `-` |
| `ldr` at 0x4d650 | load | 4 | `abs 0x4d870` | `-` |
| `ldr` at 0x4d65c | load | 4 | `abs 0x4d874` | `-` |
| `str` at 0x4d664 | store | 4 | `sp -0x7c` | `0x7e` |
| `ldr` at 0x4d668 | load | 4 | `abs 0x4d878` | `-` |
| `str` at 0x4d66c | store | 4 | `sp -0x78` | `0x10a2f4` |
| `ldr` at 0x4d670 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x4d678 | load | 4 | `abs 0x148070` | `-` |
| `ldr` at 0x4d690 | load | 4 | `abs 0x4d87c` | `-` |
| `str` at 0x4d694 | store | 4 | `sp -0x7c` | `0x10a2fc` |
| `ldrb` at 0x4d698 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4d69c | store | 4 | `sp -0x78` | `[sp-17]:1` |
| `ldr` at 0x4d6a4 | load | 4 | `abs 0x4d83c` | `-` |
| `ldr` at 0x4d6a8 | load | 4 | `abs 0x4d880` | `-` |
| `ldr` at 0x4d6b4 | load | 4 | `abs 0x4d884` | `-` |
| `str` at 0x4d6b8 | store | 4 | `sp -0x74` | `0xfffff884` |
| `ldr` at 0x4d6c0 | load | 4 | `abs 0x4d834` | `-` |
| `ldr` at 0x4d6c4 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x4d6d0 | load | 4 | `abs 0x4d888` | `-` |
| `str` at 0x4d6d4 | store | 4 | `sp -0x7c` | `0x10a324` |
| `ldr` at 0x4d6dc | load | 4 | `abs 0x4d83c` | `-` |
| `ldr` at 0x4d6e0 | load | 4 | `abs 0x4d88c` | `-` |
| `str` at 0x4d6f0 | store | 4 | `sp -0x1c` | `0xff` |
| `ldr` at 0x4d6f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d6f8 | load | 4 | `abs 0x4d844` | `-` |
| `ldr` at 0x4d704 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d70c | load | 4 | `abs 0x4d844` | `-` |
| `str` at 0x4d718 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x4d71c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d720 | load | 4 | `abs 0x4d848` | `-` |
| `ldr` at 0x4d72c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d734 | load | 4 | `abs 0x4d848` | `-` |
| `str` at 0x4d740 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x4d744 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d748 | load | 4 | `abs 0x4d84c` | `-` |
| `ldr` at 0x4d754 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d75c | load | 4 | `abs 0x4d84c` | `-` |
| `str` at 0x4d768 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x4d76c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d77c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4d790 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x4d794 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d7a4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4d7b8 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x4d7c4 | store | 4 | `sp -0x7c` | `0x8` |
| `ldr` at 0x4d7c8 | load | 4 | `abs 0x4d890` | `-` |
| `str` at 0x4d7cc | store | 4 | `sp -0x78` | `0x10a334` |
| `ldr` at 0x4d7d0 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x4d7ec | load | 4 | `abs 0x4d894` | `-` |
| `str` at 0x4d7f0 | store | 4 | `sp -0x7c` | `0x10a340` |
| `ldrb` at 0x4d7f4 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4d7f8 | store | 4 | `sp -0x78` | `[sp-17]:1` |
| `ldr` at 0x4d800 | load | 4 | `abs 0x4d83c` | `-` |
| `ldr` at 0x4d804 | load | 4 | `abs 0x4d898` | `-` |
| `ldr` at 0x4d810 | load | 4 | `abs 0x4d89c` | `-` |
| `str` at 0x4d814 | store | 4 | `sp -0x74` | `0xfffff892` |
| `str` at 0x4d820 | store | 4 | `sp -0x74` | `0x0` |
| `ldr` at 0x4d824 | load | 4 | `sp -0x74` | `-` |
| `ldmdb` at 0x4d828 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4d828 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4d828 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4d39c | `(unknown callee) 0x4bd0c` | `(r0 & 0xff)` | `r1` | `?` | `(r0 & 0xff)` |
| `bl` at 0x4d3c8 | `(unknown callee) 0x4c124` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x4d3fc | `(unknown callee) 0xdca64` | `0x1` | `0x0` | `sp-108` | `sp-108` |
| `bl` at 0x4d424 | `(unknown callee) 0x4b288` | `[sp-17]:1` | `0x1` | `?` | `[sp-17]:1` |
| `bl` at 0x4d450 | `(unknown callee) 0x492bc` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x4d49c | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x763` | `0x0` |
| `bl` at 0x4d590 | `(unknown callee) 0xeac50` | `[sp-112]` | `0x60` | `sp-28` | `0x0` |
| `bl` at 0x4d5c0 | `(unknown callee) 0x21e28` | `0x3` | `0x109480` | `0x769` | `0x0` |
| `bl` at 0x4d5f0 | `(unknown callee) 0xeac50` | `[sp-112]` | `0x62` | `sp-28` | `0x0` |
| `bl` at 0x4d620 | `(unknown callee) 0x21e28` | `0x3` | `0x109480` | `0x76e` | `0x0` |
| `bl` at 0x4d658 | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x776` | `0x0` |
| `bl` at 0x4d680 | `(unknown callee) 0xeac50` | `[sp-112]` | `0x6` | `[0x148070]` | `0x0` |
| `bl` at 0x4d6b0 | `(unknown callee) 0x21e28` | `0x3` | `0x109480` | `0x77a` | `0x0` |
| `bl` at 0x4d6e8 | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x782` | `0x0` |
| `bl` at 0x4d7dc | `(unknown callee) 0xeac50` | `[sp-112]` | `0x9` | `sp-28` | `0x0` |
| `bl` at 0x4d80c | `(unknown callee) 0x21e28` | `0x3` | `0x109480` | `0x788` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4d3b0 | `ret(0x4bd0c) == 0x0` | `ret(0x4bd0c) != 0x0` |
| `beq` at 0x4d3dc | `ret(0x4c124) == 0x0` | `ret(0x4c124) != 0x0` |
| `bne` at 0x4d408 | `ret(0xdca64) != 0x0` | `ret(0xdca64) == 0x0` |
| `beq` at 0x4d438 | `ret(0x4b288) == 0x0` | `ret(0x4b288) != 0x0` |
| `beq` at 0x4d464 | `ret(0x492bc) == 0x0` | `ret(0x492bc) != 0x0` |
| `beq` at 0x4d480 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `beq` at 0x4d59c | `ret(0xeac50) == 0x0` | `ret(0xeac50) != 0x0` |
| `beq` at 0x4d5fc | `ret(0xeac50) == 0x0` | `ret(0xeac50) != 0x0` |
| `beq` at 0x4d63c | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `beq` at 0x4d68c | `ret(0xeac50) == 0x0` | `ret(0xeac50) != 0x0` |
| `beq` at 0x4d6cc | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `beq` at 0x4d7e8 | `ret(0xeac50) == 0x0` | `ret(0xeac50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4d828 | `r0` | `?` |

## `SHUB_Scan_IDCODE`

- `SHUB_Scan_IDCODE`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4bd0c to 0x4c0f0.
  Established. The indirect callback has unknown target and candidates unknown, unknown,
  `[0x142b50]`, unknown (`mov` at 0x4bd84). The call to `0xdca64` uses `0x0`, `0x0`, `sp-104`,
  `sp-104` (`bl` at 0x4bd98); its nonzero result takes the continuation and zero falls through
  (`bne` at 0x4bda4). Device-ID bit interpretation and r0 at exit are unresolved (`ldmdb` at
  0x4c090).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4bd1c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4bd24 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x4bd28 | load | 4 | `abs 0x4c094` | `-` |
| `ldr` at 0x4bd2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4bd34 | load | 4 | `abs 0x4c098` | `-` |
| `ldr` at 0x4bd38 | load | 4 | `abs 0x148058` | `-` |
| `ldr` at 0x4bd40 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4bd54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4bd5c | load | 4 | `abs 0x4c098` | `-` |
| `ldr` at 0x4bd60 | load | 4 | `abs 0x148058` | `-` |
| `ldr` at 0x4bd68 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4bd74 | load | 4 | `abs 0x142b50` | `-` |
| `ldr` at 0x4bda8 | load | 4 | `abs 0x4c09c` | `-` |
| `str` at 0x4bdac | store | 4 | `sp -0x78` | `0x109ef0` |
| `ldr` at 0x4bdb4 | load | 4 | `abs 0x4c0a0` | `-` |
| `ldr` at 0x4bdb8 | load | 4 | `abs 0x4c0a4` | `-` |
| `ldr` at 0x4bdc4 | load | 4 | `abs 0x4c0a8` | `-` |
| `str` at 0x4bdc8 | store | 4 | `sp -0x6c` | `0xfffff886` |
| `ldr` at 0x4bdd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4bdd8 | load | 4 | `abs 0x4c098` | `-` |
| `ldr` at 0x4bddc | load | 4 | `abs 0x148058` | `-` |
| `ldr` at 0x4bde4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4bdec | load | 4 | `abs 0x4c0ac` | `-` |
| `str` at 0x4bdf8 | store | 4 | `sp -0x78` | `0x20` |
| `ldr` at 0x4bdfc | load | 4 | `abs 0x4c0b0` | `-` |
| `str` at 0x4be00 | store | 4 | `sp -0x74` | `0x109f08` |
| `ldr` at 0x4be04 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4be24 | load | 4 | `abs 0x4c0b4` | `-` |
| `str` at 0x4be28 | store | 4 | `sp -0x78` | `0x109f18` |
| `ldr` at 0x4be2c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4be30 | store | 4 | `sp -0x74` | `[sp-20]` |
| `ldr` at 0x4be38 | load | 4 | `abs 0x4c0a0` | `-` |
| `ldr` at 0x4be3c | load | 4 | `abs 0x4c0b8` | `-` |
| `ldr` at 0x4be48 | load | 4 | `abs 0x4c0a8` | `-` |
| `str` at 0x4be4c | store | 4 | `sp -0x6c` | `0xfffff886` |
| `ldr` at 0x4be54 | load | 4 | `abs 0x4c0ac` | `-` |
| `ldr` at 0x4be58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4be5c | load | 4 | `abs 0x4c0ac` | `-` |
| `ldr` at 0x4be60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4be64 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4be68 | load | 4 | `abs 0x4c0bc` | `-` |
| `ldr` at 0x4be74 | load | 4 | `abs 0x4c0ac` | `-` |
| `ldr` at 0x4be78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4be7c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4be84 | load | 4 | `abs 0x4c0bc` | `-` |
| `str` at 0x4be90 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x4be94 | load | 4 | `abs 0x4c0ac` | `-` |
| `ldr` at 0x4be98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4be9c | load | 4 | `abs 0x4c0ac` | `-` |
| `ldr` at 0x4bea0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4bea4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4bea8 | load | 4 | `abs 0x4c0c0` | `-` |
| `ldr` at 0x4beb4 | load | 4 | `abs 0x4c0ac` | `-` |
| `ldr` at 0x4beb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4bebc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4bec4 | load | 4 | `abs 0x4c0c0` | `-` |
| `str` at 0x4bed0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x4bed4 | load | 4 | `abs 0x4c0ac` | `-` |
| `ldr` at 0x4bed8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4bedc | load | 4 | `abs 0x4c0ac` | `-` |
| `ldr` at 0x4bee0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4bee4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4bee8 | load | 4 | `abs 0x4c0c4` | `-` |
| `ldr` at 0x4bef4 | load | 4 | `abs 0x4c0ac` | `-` |
| `ldr` at 0x4bef8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4befc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4bf04 | load | 4 | `abs 0x4c0c4` | `-` |
| `str` at 0x4bf10 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x4bf14 | load | 4 | `abs 0x4c0ac` | `-` |
| `ldr` at 0x4bf18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4bf1c | load | 4 | `abs 0x4c0ac` | `-` |
| `ldr` at 0x4bf20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4bf24 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4bf34 | load | 4 | `abs 0x4c0ac` | `-` |
| `ldr` at 0x4bf38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4bf3c | load | 4 | `(unknown)` | `-` |
| `str` at 0x4bf50 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x4bf54 | load | 4 | `abs 0x4c0ac` | `-` |
| `ldr` at 0x4bf58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4bf5c | load | 4 | `abs 0x4c0ac` | `-` |
| `ldr` at 0x4bf60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4bf64 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4bf74 | load | 4 | `abs 0x4c0ac` | `-` |
| `ldr` at 0x4bf78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4bf7c | load | 4 | `(unknown)` | `-` |
| `str` at 0x4bf90 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x4bf94 | load | 4 | `abs 0x4c0ac` | `-` |
| `ldr` at 0x4bf98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4bf9c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4bfa4 | load | 4 | `abs 0x4c0c8` | `-` |
| `ldr` at 0x4bfb0 | load | 4 | `abs 0x4c0cc` | `-` |
| `str` at 0x4bfb4 | store | 4 | `sp -0x78` | `0x109f48` |
| `ldr` at 0x4bfbc | load | 4 | `abs 0x4c0a0` | `-` |
| `ldr` at 0x4bfc0 | load | 4 | `abs 0x4c0d0` | `-` |
| `ldr` at 0x4bfcc | load | 4 | `abs 0x4c0d4` | `-` |
| `str` at 0x4bfd0 | store | 4 | `sp -0x78` | `0x109f84` |
| `ldr` at 0x4bfd4 | load | 4 | `abs 0x4c0ac` | `-` |
| `ldr` at 0x4bfd8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4bfdc | load | 4 | `(unknown)` | `-` |
| `str` at 0x4bfe4 | store | 4 | `sp -0x74` | `?` |
| `ldr` at 0x4bfec | load | 4 | `abs 0x4c0a0` | `-` |
| `ldr` at 0x4bff0 | load | 4 | `abs 0x4c0d8` | `-` |
| `ldr` at 0x4bffc | load | 4 | `abs 0x4c0a8` | `-` |
| `str` at 0x4c000 | store | 4 | `sp -0x18` | `0xfffff886` |
| `ldr` at 0x4c008 | load | 4 | `abs 0x4c0dc` | `-` |
| `ldr` at 0x4c00c | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x4c018 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c024 | load | 4 | `abs 0x4c0e0` | `-` |
| `str` at 0x4c028 | store | 4 | `sp -0x78` | `0x109fb4` |
| `ldr` at 0x4c02c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4c030 | store | 4 | `sp -0x74` | `[sp-20]` |
| `str` at 0x4c034 | store | 4 | `sp -0x70` | `ret(0x4c0f0)` |
| `ldr` at 0x4c03c | load | 4 | `abs 0x4c0a0` | `-` |
| `ldr` at 0x4c040 | load | 4 | `abs 0x4c0e4` | `-` |
| `ldr` at 0x4c04c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c060 | load | 4 | `abs 0x4c0e8` | `-` |
| `str` at 0x4c064 | store | 4 | `sp -0x78` | `0x109fc4` |
| `ldr` at 0x4c06c | load | 4 | `abs 0x4c0a0` | `-` |
| `ldr` at 0x4c070 | load | 4 | `abs 0x4c0ec` | `-` |
| `ldr` at 0x4c07c | load | 4 | `abs 0x4c0a8` | `-` |
| `str` at 0x4c080 | store | 4 | `sp -0x18` | `0xfffff886` |
| `ldr` at 0x4c084 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4c088 | store | 4 | `sp -0x6c` | `[sp-24]` |
| `ldr` at 0x4c08c | load | 4 | `sp -0x6c` | `-` |
| `ldmdb` at 0x4c090 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4c090 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4c090 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x4bd84 | `(unknown callee) (unknown target)` | `?` | `?` | `[0x142b50]` | `?` |
| `bl` at 0x4bd98 | `(unknown callee) 0xdca64` | `0x0` | `0x0` | `sp-104` | `sp-104` |
| `bl` at 0x4bdc0 | `(unknown callee) 0x21e28` | `0x3` | `0x109480` | `0x581` | `0x0` |
| `bl` at 0x4be14 | `(unknown callee) 0xeac50` | `?` | `0x3` | `0x0` | `?` |
| `bl` at 0x4be44 | `(unknown callee) 0x21e28` | `0x3` | `0x109480` | `0x587` | `0x0` |
| `bl` at 0x4bfc8 | `(unknown callee) 0x21e28` | `0x3` | `0x109480` | `0x591` | `0x0` |
| `bl` at 0x4bff8 | `(unknown callee) 0x21e28` | `0x3` | `0x109480` | `0x593` | `0x0` |
| `bl` at 0x4c01c | `(unknown callee) 0x4c0f0` | `[sp-20]` | `?` | `?` | `[0x2ad25c]` |
| `bl` at 0x4c048 | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x597` | `0x0` |
| `bl` at 0x4c050 | `(unknown callee) 0x4c0f0` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x4c078 | `(unknown callee) 0x21e28` | `0x3` | `0x109480` | `0x59c` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x4bda4 | `ret(0xdca64) != 0x0` | `ret(0xdca64) == 0x0` |
| `beq` at 0x4be20 | `ret(0xeac50) == 0x0` | `ret(0xeac50) != 0x0` |
| `beq` at 0x4bfac | `? == 0x4363049` | `? != 0x4363049` |
| `beq` at 0x4c014 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `bgt` at 0x4c05c | `ret(0x4c0f0) > 0x1 (signed, 32-bit)` | `ret(0x4c0f0) <= 0x1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4c090 | `r0` | `?` |

## `SHUB_ScanSetBusratio`

- `SHUB_ScanSetBusratio`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4c124 to 0x4c4e4.
  Established. The unsigned halfword comparison takes its continuation for `[sp-34]:2 <= 0x8fc` and
  falls through for greater (`bls` at 0x4c3dc). A later byte store writes `([sp-35]:1 & 0xff)` at an
  unknown target (`strb` at 0x4c48c). The opening call to `0xd1dfc` has `0x0`, `sp-32`, entry r2,
  `sp-32` (`bl` at 0x4c168). The exit carries r0 `0x0` (`ldmdb` at 0x4c49c). Bus-ratio encoding and
  numeric units remain unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4c134 | store | 4 | `sp -0x14` | `r0` |
| `strh` at 0x4c13c | store | 2 | `sp -0x22` | `0x0` |
| `ldr` at 0x4c140 | load | 4 | `abs 0x4c4a0` | `-` |
| `ldr` at 0x4c144 | load | 4 | `abs 0x14807c` | `-` |
| `str` at 0x4c154 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x4c170 | store | 4 | `sp -0x1c` | `ret(0xd1dfc)` |
| `ldr` at 0x4c174 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4c180 | load | 4 | `abs 0x4c4a4` | `-` |
| `str` at 0x4c184 | store | 4 | `sp -0x38` | `0x10a004` |
| `ldr` at 0x4c18c | load | 4 | `abs 0x4c4a8` | `-` |
| `ldr` at 0x4c190 | load | 4 | `abs 0x4c4ac` | `-` |
| `str` at 0x4c1a0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x4c1a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c1b0 | load | 4 | `abs 0x4c4b0` | `-` |
| `ldr` at 0x4c1b4 | load | 4 | `abs 0x148058` | `-` |
| `ldrb` at 0x4c1bc | load | 1 | `(unknown)` | `-` |
| `str` at 0x4c1cc | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x4c1d8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x4c1dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c1e4 | load | 4 | `abs 0x4c4b0` | `-` |
| `ldr` at 0x4c1e8 | load | 4 | `abs 0x148058` | `-` |
| `ldrb` at 0x4c1f0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x4c204 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4c20c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x4c210 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4c220 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x4c22c | load | 1 | `sp -0x18` | `-` |
| `ldr` at 0x4c24c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c254 | load | 4 | `abs 0x4c4b0` | `-` |
| `ldr` at 0x4c258 | load | 4 | `abs 0x148058` | `-` |
| `ldrb` at 0x4c260 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x4c264 | load | 1 | `sp -0x18` | `-` |
| `ldr` at 0x4c28c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4c2a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4c2a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4c2b4 | load | 4 | `abs 0x4c4b4` | `-` |
| `ldr` at 0x4c2b8 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x4c2c4 | load | 4 | `abs 0x4c4b8` | `-` |
| `str` at 0x4c2c8 | store | 4 | `sp -0x38` | `0x10a020` |
| `ldr` at 0x4c2cc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4c2d0 | store | 4 | `sp -0x34` | `[sp-20]` |
| `ldr` at 0x4c2d4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4c2d8 | store | 4 | `sp -0x30` | `[sp-24]` |
| `ldr` at 0x4c2dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4c2f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4c2f8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x4c2fc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x4c304 | load | 4 | `abs 0x4c4a8` | `-` |
| `ldr` at 0x4c308 | load | 4 | `abs 0x4c4bc` | `-` |
| `ldrh` at 0x4c314 | load | 2 | `sp -0x22` | `-` |
| `ldrh` at 0x4c320 | load | 2 | `sp -0x22` | `-` |
| `ldr` at 0x4c324 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4c338 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4c340 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4c350 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4c364 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0x4c36c | load | 2 | `(unknown)` | `-` |
| `strh` at 0x4c370 | store | 2 | `sp -0x22` | `?` |
| `ldr` at 0x4c374 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4c37c | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldrh` at 0x4c384 | load | 2 | `sp -0x22` | `-` |
| `ldr` at 0x4c390 | load | 4 | `abs 0x4c4b4` | `-` |
| `ldr` at 0x4c394 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x4c3a0 | load | 4 | `abs 0x4c4c0` | `-` |
| `str` at 0x4c3a4 | store | 4 | `sp -0x38` | `0x10a03c` |
| `ldr` at 0x4c3a8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4c3ac | store | 4 | `sp -0x34` | `[sp-20]` |
| `ldr` at 0x4c3b4 | load | 4 | `abs 0x4c4a8` | `-` |
| `ldr` at 0x4c3b8 | load | 4 | `abs 0x4c4c4` | `-` |
| `str` at 0x4c3c8 | store | 4 | `sp -0x28` | `0x0` |
| `ldrh` at 0x4c3d0 | load | 2 | `sp -0x22` | `-` |
| `ldr` at 0x4c3d4 | load | 4 | `abs 0x4c4c8` | `-` |
| `ldr` at 0x4c3e0 | load | 4 | `abs 0x4c4cc` | `-` |
| `str` at 0x4c3e4 | store | 4 | `sp -0x38` | `0x10a070` |
| `ldr` at 0x4c3e8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4c3ec | store | 4 | `sp -0x34` | `[sp-20]` |
| `ldr` at 0x4c3f4 | load | 4 | `abs 0x4c4a8` | `-` |
| `ldr` at 0x4c3f8 | load | 4 | `abs 0x4c4d0` | `-` |
| `ldrh` at 0x4c404 | load | 2 | `sp -0x22` | `-` |
| `ldr` at 0x4c40c | load | 4 | `abs 0x4c4d4` | `-` |
| `strb` at 0x4c424 | store | 1 | `sp -0x23` | `?` |
| `ldr` at 0x4c428 | load | 4 | `abs 0x4c4b4` | `-` |
| `ldr` at 0x4c42c | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x4c438 | load | 4 | `abs 0x4c4d8` | `-` |
| `str` at 0x4c43c | store | 4 | `sp -0x38` | `0x10a0b0` |
| `ldr` at 0x4c440 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4c444 | store | 4 | `sp -0x34` | `[sp-20]` |
| `ldrb` at 0x4c448 | load | 1 | `sp -0x23` | `-` |
| `str` at 0x4c44c | store | 4 | `sp -0x30` | `[sp-35]:1` |
| `ldr` at 0x4c454 | load | 4 | `abs 0x4c4a8` | `-` |
| `ldr` at 0x4c458 | load | 4 | `abs 0x4c4dc` | `-` |
| `ldr` at 0x4c464 | load | 4 | `abs 0x4c4e0` | `-` |
| `ldr` at 0x4c468 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x4c488 | load | 1 | `sp -0x23` | `-` |
| `strb` at 0x4c48c | store | 1 | `(unknown)` | `([sp-35]:1 & 0xff)` |
| `str` at 0x4c494 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x4c498 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x4c49c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4c49c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4c49c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4c168 | `(unknown callee) 0xd1dfc` | `0x0` | `sp-32` | `r2` | `sp-32` |
| `bl` at 0x4c198 | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x5c3` | `0x0` |
| `bl` at 0x4c23c | `(unknown callee) 0x5d8c0` | `?` | `?` | `?` | `?` |
| `bl` at 0x4c278 | `(unknown callee) 0xd1e50` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x4c310 | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x5d3` | `0x0` |
| `bl` at 0x4c3c0 | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x5dd` | `0x0` |
| `bl` at 0x4c400 | `(unknown callee) 0x21e28` | `0x4` | `0x109480` | `0x5e3` | `0x0` |
| `bl` at 0x4c460 | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x5e6` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4c14c | `[0x14807c] == 0x0` | `[0x14807c] != 0x0` |
| `bne` at 0x4c17c | `ret(0xd1dfc) != 0x0` | `ret(0xd1dfc) == 0x0` |
| `bne` at 0x4c1c4 | `? != 0xff` | `? == 0xff` |
| `ble` at 0x4c218 | `[sp-24] <= 0x1 (signed, 32-bit)` | `[sp-24] > 0x1 (signed, 32-bit)` |
| `beq` at 0x4c248 | `ret(0x5d8c0) == 0x0` | `ret(0x5d8c0) != 0x0` |
| `bne` at 0x4c284 | `ret(0xd1e50) != 0x0` | `ret(0xd1e50) == 0x0` |
| `bne` at 0x4c2b0 | `? != 0x3` | `? == 0x3` |
| `beq` at 0x4c2c0 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `beq` at 0x4c31c | `[sp-34]:2 == 0x0` | `[sp-34]:2 != 0x0` |
| `bge` at 0x4c348 | `[sp-34]:2 >= ? (signed, 32-bit)` | `[sp-34]:2 < ? (signed, 32-bit)` |
| `bne` at 0x4c38c | `[sp-34]:2 != 0x0` | `[sp-34]:2 == 0x0` |
| `beq` at 0x4c39c | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `bls` at 0x4c3dc | `[sp-34]:2 <= 0x8fc (unsigned, 32-bit)` | `[sp-34]:2 > 0x8fc (unsigned, 32-bit)` |
| `beq` at 0x4c434 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4c49c | `r0` | `0x0` |

## `SHUB_ScanReadMCILockStatus`

- `SHUB_ScanReadMCILockStatus`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4ceb4 to 0x4d334.
  Established. The call to `0xeac50` carries `[sp-72]`, `0x7`, `sp-48`, `sp-64` (`bl` at 0x4cf5c);
  its zero result takes the continuation and nonzero falls through (`beq` at 0x4cf68). The next call
  to that target has `[sp-72]`, unknown, `sp-48`, `sp-64` (`bl` at 0x4cfdc). MCI lock-bit meaning
  and r0 at the merged exit remain unidentified (`ldmdb` at 0x4d2e0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4cec8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x4ced0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldrb` at 0x4ced4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x4cedc | load | 4 | `abs 0x4d2e4` | `-` |
| `ldr` at 0x4cee0 | load | 4 | `abs 0x148058` | `-` |
| `ldr` at 0x4cee8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x4ceec | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x4cef0 | load | 4 | `abs 0x4d2e8` | `-` |
| `ldr` at 0x4cef4 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x4cf00 | load | 4 | `abs 0x4d2ec` | `-` |
| `str` at 0x4cf04 | store | 4 | `sp -0x5c` | `0x10a128` |
| `ldr` at 0x4cf0c | load | 4 | `abs 0x4d2f0` | `-` |
| `ldr` at 0x4cf10 | load | 4 | `abs 0x4d2f4` | `-` |
| `ldrb` at 0x4cf24 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4cf44 | store | 4 | `sp -0x5c` | `0x4c` |
| `ldr` at 0x4cf48 | load | 4 | `abs 0x4d2f8` | `-` |
| `str` at 0x4cf4c | store | 4 | `sp -0x58` | `0x10a148` |
| `ldr` at 0x4cf50 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x4cf6c | load | 4 | `abs 0x4d2fc` | `-` |
| `str` at 0x4cf70 | store | 4 | `sp -0x5c` | `0x10a150` |
| `ldrb` at 0x4cf74 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4cf78 | store | 4 | `sp -0x58` | `[sp-17]:1` |
| `ldr` at 0x4cf80 | load | 4 | `abs 0x4d2f0` | `-` |
| `ldr` at 0x4cf84 | load | 4 | `abs 0x4d300` | `-` |
| `ldr` at 0x4cf90 | load | 4 | `abs 0x4d304` | `-` |
| `str` at 0x4cf94 | store | 4 | `sp -0x4c` | `0xfffff890` |
| `ldrb` at 0x4cf9c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4cfc4 | store | 4 | `sp -0x5c` | `0x4c` |
| `ldr` at 0x4cfc8 | load | 4 | `abs 0x4d2f8` | `-` |
| `str` at 0x4cfcc | store | 4 | `sp -0x58` | `0x10a148` |
| `ldr` at 0x4cfd0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x4cfec | load | 4 | `abs 0x4d2fc` | `-` |
| `str` at 0x4cff0 | store | 4 | `sp -0x5c` | `0x10a150` |
| `ldrb` at 0x4cff4 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4cff8 | store | 4 | `sp -0x58` | `[sp-17]:1` |
| `ldr` at 0x4d000 | load | 4 | `abs 0x4d2f0` | `-` |
| `ldr` at 0x4d004 | load | 4 | `abs 0x4d308` | `-` |
| `ldr` at 0x4d010 | load | 4 | `abs 0x4d304` | `-` |
| `str` at 0x4d014 | store | 4 | `sp -0x4c` | `0xfffff890` |
| `ldrb` at 0x4d01c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x4d030 | load | 4 | `abs 0x4d30c` | `-` |
| `ldrb` at 0x4d034 | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x4d04c | load | 2 | `(unknown)` | `-` |
| `strh` at 0x4d050 | store | 2 | `sp -0x42` | `?` |
| `ldr` at 0x4d054 | load | 4 | `abs 0x4d310` | `-` |
| `ldrb` at 0x4d058 | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x4d070 | load | 2 | `(unknown)` | `-` |
| `strh` at 0x4d074 | store | 2 | `sp -0x44` | `?` |
| `ldrh` at 0x4d078 | load | 2 | `sp -0x42` | `-` |
| `ldrh` at 0x4d07c | load | 2 | `sp -0x44` | `-` |
| `strh` at 0x4d084 | store | 2 | `sp -0x42` | `?` |
| `ldrh` at 0x4d088 | load | 2 | `sp -0x42` | `-` |
| `strh` at 0x4d094 | store | 2 | `sp -0x42` | `?` |
| `ldrh` at 0x4d098 | load | 2 | `sp -0x42` | `-` |
| `ldr` at 0x4d0a4 | load | 4 | `abs 0x4d314` | `-` |
| `ldrb` at 0x4d0b0 | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x4d0b4 | load | 2 | `sp -0x42` | `-` |
| `ldr` at 0x4d0b8 | load | 4 | `abs 0x4d318` | `-` |
| `str` at 0x4d0c8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x4d0cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4d0dc | load | 4 | `abs 0x4d2e8` | `-` |
| `ldr` at 0x4d0e0 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x4d0ec | load | 4 | `abs 0x4d31c` | `-` |
| `str` at 0x4d0f0 | store | 4 | `sp -0x5c` | `0x10a1b8` |
| `ldrb` at 0x4d0f4 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4d0f8 | store | 4 | `sp -0x58` | `[sp-17]:1` |
| `ldr` at 0x4d0fc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4d100 | store | 4 | `sp -0x54` | `?` |
| `ldrh` at 0x4d104 | load | 2 | `sp -0x42` | `-` |
| `str` at 0x4d10c | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x4d114 | load | 4 | `abs 0x4d2f0` | `-` |
| `ldr` at 0x4d118 | load | 4 | `abs 0x4d320` | `-` |
| `ldrh` at 0x4d124 | load | 2 | `sp -0x42` | `-` |
| `strh` at 0x4d12c | store | 2 | `sp -0x42` | `?` |
| `ldr` at 0x4d130 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4d138 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x4d140 | load | 4 | `abs 0x4d30c` | `-` |
| `ldrb` at 0x4d144 | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x4d15c | load | 2 | `(unknown)` | `-` |
| `strh` at 0x4d160 | store | 2 | `sp -0x42` | `?` |
| `ldr` at 0x4d164 | load | 4 | `abs 0x4d310` | `-` |
| `ldrb` at 0x4d168 | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x4d180 | load | 2 | `(unknown)` | `-` |
| `strh` at 0x4d184 | store | 2 | `sp -0x44` | `?` |
| `ldrh` at 0x4d188 | load | 2 | `sp -0x42` | `-` |
| `ldrh` at 0x4d18c | load | 2 | `sp -0x44` | `-` |
| `strh` at 0x4d194 | store | 2 | `sp -0x42` | `?` |
| `ldrh` at 0x4d198 | load | 2 | `sp -0x42` | `-` |
| `ldr` at 0x4d1a4 | load | 4 | `abs 0x4d314` | `-` |
| `ldrb` at 0x4d1b0 | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x4d1b4 | load | 2 | `sp -0x42` | `-` |
| `ldr` at 0x4d1b8 | load | 4 | `abs 0x4d324` | `-` |
| `str` at 0x4d1c8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x4d1cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4d1dc | load | 4 | `abs 0x4d2e8` | `-` |
| `ldr` at 0x4d1e0 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x4d1ec | load | 4 | `abs 0x4d328` | `-` |
| `str` at 0x4d1f0 | store | 4 | `sp -0x5c` | `0x10a210` |
| `ldrb` at 0x4d1f4 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4d1f8 | store | 4 | `sp -0x58` | `[sp-17]:1` |
| `ldr` at 0x4d1fc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4d200 | store | 4 | `sp -0x54` | `?` |
| `ldrh` at 0x4d204 | load | 2 | `sp -0x42` | `-` |
| `str` at 0x4d20c | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x4d214 | load | 4 | `abs 0x4d2f0` | `-` |
| `ldr` at 0x4d218 | load | 4 | `abs 0x4d32c` | `-` |
| `ldrh` at 0x4d224 | load | 2 | `sp -0x42` | `-` |
| `strh` at 0x4d22c | store | 2 | `sp -0x42` | `?` |
| `ldr` at 0x4d230 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4d238 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x4d240 | load | 4 | `abs 0x4d30c` | `-` |
| `ldrb` at 0x4d244 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x4d264 | load | 1 | `(unknown)` | `-` |
| `strh` at 0x4d268 | store | 2 | `sp -0x42` | `?` |
| `ldr` at 0x4d26c | load | 4 | `abs 0x4d310` | `-` |
| `ldrb` at 0x4d270 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x4d290 | load | 1 | `(unknown)` | `-` |
| `strh` at 0x4d294 | store | 2 | `sp -0x44` | `?` |
| `ldrh` at 0x4d298 | load | 2 | `sp -0x42` | `-` |
| `ldrh` at 0x4d29c | load | 2 | `sp -0x44` | `-` |
| `strh` at 0x4d2a4 | store | 2 | `sp -0x42` | `?` |
| `ldrh` at 0x4d2a8 | load | 2 | `sp -0x42` | `-` |
| `strh` at 0x4d2b4 | store | 2 | `sp -0x42` | `?` |
| `ldrh` at 0x4d2b8 | load | 2 | `sp -0x42` | `-` |
| `ldrb` at 0x4d2c4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x4d2c8 | load | 4 | `abs 0x4d330` | `-` |
| `str` at 0x4d2d8 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0x4d2dc | load | 4 | `sp -0x4c` | `-` |
| `ldmdb` at 0x4d2e0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4d2e0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4d2e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4cf18 | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x6ba` | `0x0` |
| `bl` at 0x4cf20 | `(unknown callee) 0x4e0e4` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x4cf34 | `(unknown callee) 0x4c604` | `[sp-17]:1` | `sp-48` | `sp-48` | `[sp-17]:1` |
| `bl` at 0x4cf5c | `(unknown callee) 0xeac50` | `[sp-72]` | `0x7` | `sp-48` | `sp-64` |
| `bl` at 0x4cf8c | `(unknown callee) 0x21e28` | `0x3` | `0x109480` | `0x6c7` | `0x0` |
| `bl` at 0x4cfac | `(unknown callee) 0x4ca0c` | `[sp-17]:1` | `0x1` | `sp-64` | `[sp-17]:1` |
| `bl` at 0x4cfb4 | `(unknown callee) 0x4e0e4` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x4cfdc | `(unknown callee) 0xeac50` | `[sp-72]` | `?` | `sp-48` | `sp-64` |
| `bl` at 0x4d00c | `(unknown callee) 0x21e28` | `0x3` | `0x109480` | `0x6da` | `0x0` |
| `bl` at 0x4d02c | `(unknown callee) 0x4ca0c` | `[sp-17]:1` | `0x2` | `sp-64` | `[sp-17]:1` |
| `bl` at 0x4d0c0 | `(unknown callee) 0x2070c` | `0x10a17c` | `[sp-17]:1` | `[sp-66]:2` | `[sp-17]:1` |
| `bl` at 0x4d120 | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x6ee` | `0x0` |
| `bl` at 0x4d1c0 | `(unknown callee) 0x2070c` | `0x10a1d4` | `[sp-17]:1` | `[sp-66]:2` | `[sp-17]:1` |
| `bl` at 0x4d220 | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x6f8` | `0x0` |
| `bl` at 0x4d2d0 | `(unknown callee) 0x2070c` | `0x10a22c` | `[sp-17]:1` | `[sp-66]:2` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4cefc | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `beq` at 0x4cf68 | `ret(0xeac50) == 0x0` | `ret(0xeac50) != 0x0` |
| `beq` at 0x4cfe8 | `ret(0xeac50) == 0x0` | `ret(0xeac50) != 0x0` |
| `beq` at 0x4d0ac | `? == 0x1ff` | `? != 0x1ff` |
| `ble` at 0x4d0d4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x4d0e8 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `beq` at 0x4d1ac | `? == 0x1ff` | `? != 0x1ff` |
| `ble` at 0x4d1d4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x4d1e8 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `beq` at 0x4d2c0 | `[sp-66]:2 == 0x1` | `[sp-66]:2 != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4d2e0 | `r0` | `?` |

## `FlashInit`

- `FlashInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa5048 to 0xa51d8.
  Established. The word store at `0x149910` writes `[0x149910]+1` (`str` at 0xa5094). Calls include
  `0xb5654` with `0x2b1a2c`, `0x126230`, `0x5`, `0x14` (`bl` at 0xa50c4). The later call to
  `0x223f0` uses `0x2b1a28`, `0x126280`, `0xa51d8`, `0x0`, and its result reaches the exit (`bl` at
  0xa5198, `ldmdb` at 0xa519c). Task creation, queue roles and physical flash initialization remain
  unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xa5058 | load | 4 | `abs 0xa51a0` | `-` |
| `ldr` at 0xa505c | load | 4 | `abs 0x149910` | `-` |
| `ldr` at 0xa5068 | load | 4 | `abs 0xa51a4` | `-` |
| `str` at 0xa506c | store | 4 | `sp -0x30` | `0x1261fc` |
| `ldr` at 0xa5074 | load | 4 | `abs 0xa51a8` | `-` |
| `ldr` at 0xa5078 | load | 4 | `abs 0xa51ac` | `-` |
| `ldr` at 0xa5084 | load | 4 | `abs 0xa51a0` | `-` |
| `ldr` at 0xa5088 | load | 4 | `abs 0xa51a0` | `-` |
| `ldr` at 0xa508c | load | 4 | `abs 0x149910` | `-` |
| `str` at 0xa5094 | store | 4 | `abs 0x149910` | `[0x149910]+1` |
| `ldr` at 0xa5098 | load | 4 | `abs 0xa51b0` | `-` |
| `str` at 0xa50ac | store | 4 | `sp -0x18` | `0x19` |
| `str` at 0xa50b4 | store | 4 | `sp -0x1c` | `0x14` |
| `ldr` at 0xa50b8 | load | 4 | `abs 0xa51b4` | `-` |
| `ldr` at 0xa50bc | load | 4 | `abs 0xa51b8` | `-` |
| `str` at 0xa50cc | store | 4 | `sp -0x14` | `ret(0xb5654)` |
| `ldr` at 0xa50d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa50dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa50e8 | load | 4 | `abs 0xa51bc` | `-` |
| `str` at 0xa50ec | store | 4 | `sp -0x30` | `0x126238` |
| `str` at 0xa50f0 | store | 4 | `sp -0x2c` | `ret(0x21674)` |
| `ldr` at 0xa50f8 | load | 4 | `abs 0xa51a8` | `-` |
| `ldr` at 0xa50fc | load | 4 | `abs 0xa51c0` | `-` |
| `ldr` at 0xa5108 | load | 4 | `abs 0xa51b0` | `-` |
| `ldr` at 0xa510c | load | 4 | `abs 0xa51b0` | `-` |
| `ldr` at 0xa5110 | load | 4 | `abs 0x2b1a20` | `-` |
| `ldr` at 0xa5118 | load | 4 | `abs 0xa51b4` | `-` |
| `str` at 0xa5124 | store | 4 | `sp -0x14` | `ret(0xb4220)` |
| `ldr` at 0xa5128 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa5134 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa5140 | load | 4 | `abs 0xa51c4` | `-` |
| `str` at 0xa5144 | store | 4 | `sp -0x30` | `0x12625c` |
| `str` at 0xa5148 | store | 4 | `sp -0x2c` | `ret(0x21674)` |
| `ldr` at 0xa5150 | load | 4 | `abs 0xa51a8` | `-` |
| `ldr` at 0xa5154 | load | 4 | `abs 0xa51c8` | `-` |
| `str` at 0xa5164 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0xa516c | store | 4 | `sp -0x2c` | `0x400` |
| `ldr` at 0xa5170 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa5174 | store | 4 | `sp -0x28` | `[sp-24]` |
| `ldr` at 0xa5178 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa517c | store | 4 | `sp -0x24` | `[sp-28]` |
| `str` at 0xa5184 | store | 4 | `sp -0x20` | `0x9` |
| `ldr` at 0xa5188 | load | 4 | `abs 0xa51cc` | `-` |
| `ldr` at 0xa518c | load | 4 | `abs 0xa51d0` | `-` |
| `ldr` at 0xa5190 | load | 4 | `abs 0xa51d4` | `-` |
| `ldmdb` at 0xa519c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa519c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa519c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa5080 | `(unknown callee) 0x21e28` | `0x9` | `0x126214` | `0xbc1` | `0x0` |
| `bl` at 0xa50a4 | `(unknown callee) 0xb553c` | `0x2b1a20` | `0x10` | `0x0` | `[0x149910]+1` |
| `bl` at 0xa50c4 | `(unknown callee) 0xb5654` | `0x2b1a2c` | `0x126230` | `0x5` | `0x14` |
| `bl` at 0xa50e0 | `(unknown callee) 0x21674` | `ret(0xb5654)` | `?` | `?` | `ret(0xb5654)` |
| `bl` at 0xa5104 | `(unknown callee) 0x21e28` | `0x9` | `0x126214` | `0xbd9` | `0x0` |
| `bl` at 0xa511c | `(unknown callee) 0xb4220` | `0x2b1a20` | `[0x2b1a20]` | `0x1` | `0x2b1a2c` |
| `bl` at 0xa5138 | `(unknown callee) 0x21674` | `ret(0xb4220)` | `?` | `?` | `ret(0xb4220)` |
| `bl` at 0xa515c | `(unknown callee) 0x21e28` | `0x9` | `0x126214` | `0xbe2` | `0x0` |
| `bl` at 0xa5198 | `(unknown callee) 0x223f0` | `0x2b1a28` | `0x126280` | `0xa51d8` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa5064 | `[0x149910] == 0x0` | `[0x149910] != 0x0` |
| `beq` at 0xa50d8 | `ret(0xb5654) == 0x0` | `ret(0xb5654) != 0x0` |
| `beq` at 0xa5130 | `ret(0xb4220) == 0x0` | `ret(0xb4220) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa519c | `r0` | `ret(0x223f0)` |

## `FlashTask`

- `FlashTask`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa51d8 to 0xa52ac.
  Established. The word store at `0x149978` writes `[0x149978]-1` (`str` at 0xa523c). The byte test
  takes its bypass for `[[sp-32]+22]:1 != 0x0` and falls through on zero (`bne` at 0xa5250).
  Fall-through calls `0xa52d4` with `[sp-32]`, unknown, unknown, `[[sp-32]+22]:1` (`bl` at 0xa5258).
  No return is recognized in the selected span. Message ownership and physical flash effects remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa51e8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa51ec | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xa51f8 | load | 4 | `abs 0xa5298` | `-` |
| `str` at 0xa5208 | store | 4 | `sp -0x1c` | `ret(0xb5698)` |
| `ldr` at 0xa520c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa521c | load | 4 | `abs 0xa529c` | `-` |
| `ldr` at 0xa5220 | load | 4 | `abs 0x149978` | `-` |
| `ldr` at 0xa522c | load | 4 | `abs 0xa529c` | `-` |
| `ldr` at 0xa5230 | load | 4 | `abs 0xa529c` | `-` |
| `ldr` at 0xa5234 | load | 4 | `abs 0x149978` | `-` |
| `str` at 0xa523c | store | 4 | `abs 0x149978` | `[0x149978]-1` |
| `ldr` at 0xa5244 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xa5248 | load | 1 | `[sp-32] +0x16` | `-` |
| `ldr` at 0xa5254 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa5260 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa526c | load | 4 | `abs 0xa52a0` | `-` |
| `str` at 0xa5270 | store | 4 | `sp -0x28` | `0x126288` |
| `str` at 0xa5274 | store | 4 | `sp -0x24` | `ret(0x21674)` |
| `ldr` at 0xa527c | load | 4 | `abs 0xa52a4` | `-` |
| `ldr` at 0xa5280 | load | 4 | `abs 0xa52a8` | `-` |
| `ldr` at 0xa528c | load | 4 | `sp -0x20` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa5200 | `(unknown callee) 0xb5698` | `sp-32` | `0x2b1a2c` | `?` | `sp-32` |
| `bl` at 0xa5258 | `(unknown callee) 0xa52d4` | `[sp-32]` | `?` | `?` | `[[sp-32]+22]:1` |
| `bl` at 0xa5264 | `(unknown callee) 0x21674` | `ret(0xb5698)` | `?` | `?` | `ret(0xb5698)` |
| `bl` at 0xa5288 | `(unknown callee) 0x21e28` | `0x3` | `0x126214` | `0xc0e` | `0x0` |
| `bl` at 0xa5290 | `(unknown callee) 0xb56f4` | `[sp-32]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa5214 | `ret(0xb5698) == 0x0` | `ret(0xb5698) != 0x0` |
| `beq` at 0xa5228 | `[0x149978] == 0x0` | `[0x149978] != 0x0` |
| `bne` at 0xa5250 | `[[sp-32]+22]:1 != 0x0` | `[[sp-32]+22]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|

## `SysComJTagReadReady`

- `SysComJTagReadReady`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x837e4 to 0x838e8.
  Established. The signed guard takes its bypass for `r0 >= [0x2b1730]` and falls through for less
  (`bge` at 0x83808). The call to `0x22dc0` has unknown, `[sp-20]`, unknown, unknown (`bl` at
  0x83890); its zero result takes the bypass and nonzero falls through (`beq` at 0x838a4).
  Read-ready notification semantics and r0 at exit remain unestablished (`ldmdb` at 0x838d4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x837f4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x837f8 | load | 4 | `abs 0x838d8` | `-` |
| `ldr` at 0x837fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x83800 | load | 4 | `abs 0x2b1730` | `-` |
| `ldr` at 0x8380c | load | 4 | `abs 0x838d8` | `-` |
| `ldr` at 0x83810 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x8381c | load | 4 | `abs 0x838d8` | `-` |
| `ldr` at 0x83820 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x83844 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x8384c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x83858 | load | 4 | `abs 0x838d8` | `-` |
| `ldr` at 0x8385c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x83880 | load | 4 | `abs 0x2b1734` | `-` |
| `str` at 0x83898 | store | 4 | `sp -0x18` | `ret(0x22dc0)` |
| `ldr` at 0x8389c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x838a8 | load | 4 | `abs 0x838dc` | `-` |
| `str` at 0x838ac | store | 4 | `sp -0x24` | `0x111c98` |
| `ldr` at 0x838b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x838b4 | store | 4 | `sp -0x20` | `[sp-20]` |
| `ldr` at 0x838b8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x838bc | store | 4 | `sp -0x1c` | `ret(0x22dc0)` |
| `ldr` at 0x838c4 | load | 4 | `abs 0x838e0` | `-` |
| `ldr` at 0x838c8 | load | 4 | `abs 0x838e4` | `-` |
| `ldmdb` at 0x838d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x838d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x838d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x83890 | `(unknown callee) 0x22dc0` | `?` | `[sp-20]` | `?` | `?` |
| `bl` at 0x838d0 | `(unknown callee) 0x21e28` | `0x9` | `0x111830` | `0x283` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x83808 | `r0 >= [0x2b1730] (signed, 32-bit)` | `r0 < [0x2b1730] (signed, 32-bit)` |
| `beq` at 0x83818 | `[0x2b1734] == 0x0` | `[0x2b1734] != 0x0` |
| `beq` at 0x83854 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x838a4 | `ret(0x22dc0) == 0x0` | `ret(0x22dc0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x838d4 | `r0` | `?` |
