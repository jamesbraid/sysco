# ARM MMR, PIO and TAP driver paths in 1.38.4

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

[Fact sheets and full listings](disasm/arm-1.38.4-package4/mmr-pio-tap-drivers-facts.txt) retain all tool sections.

## `JChip_MMR_Read`

- `JChip_MMR_Read`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xea468 to 0xea56c.
  Established. The callback has unknown target and candidates `[[sp-36]+25]:1`, unknown, `0x5`,
  `[sp-24]` (`mov` at 0xea528); its result selects the taken arm on equality with `0x1` and falls
  through on inequality (`beq` at 0xea534). Earlier byte reads address `[sp-36] +0x19` and `[sp-36]
  +0x21` (`ldrb` at 0xea4dc, `ldrsb` at 0xea4e4). The merged r0 result is unknown (`ldmdb` at
  0xea550). Register-read semantics are not established by this wrapper.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xea478 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xea47c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xea480 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xea484 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xea488 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xea494 | store | 4 | `sp -0x24` | `ret(0xeb27c)` |
| `ldr` at 0xea498 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xea4a4 | load | 4 | `abs 0xea554` | `-` |
| `str` at 0xea4a8 | store | 4 | `sp -0x28` | `0xfffff7c6` |
| `ldr` at 0xea4b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xea4c4 | load | 4 | `abs 0xea558` | `-` |
| `str` at 0xea4c8 | store | 4 | `sp -0x28` | `0xfffff7c5` |
| `ldr` at 0xea4d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xea4d4 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xea4d8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xea4dc | load | 1 | `[sp-36] +0x19` | `-` |
| `ldr` at 0xea4e0 | load | 4 | `sp -0x24` | `-` |
| `ldrsb` at 0xea4e4 | load | 1 | `[sp-36] +0x21` | `-` |
| `ldr` at 0xea4e8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xea4ec | store | 4 | `sp -0x3c` | `[sp-28]` |
| `ldr` at 0xea4f0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xea4f4 | store | 4 | `sp -0x38` | `[sp-32]` |
| `ldr` at 0xea4f8 | load | 4 | `abs 0xea55c` | `-` |
| `str` at 0xea4fc | store | 4 | `sp -0x34` | `0x2ad28c` |
| `ldr` at 0xea500 | load | 4 | `abs 0xea560` | `-` |
| `str` at 0xea504 | store | 4 | `sp -0x30` | `0x2ad30c` |
| `ldr` at 0xea508 | load | 4 | `abs 0xea564` | `-` |
| `str` at 0xea50c | store | 4 | `sp -0x2c` | `0x2b3a78` |
| `ldr` at 0xea510 | load | 4 | `[[sp-36]] +0x14` | `-` |
| `ldr` at 0xea520 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xea538 | load | 4 | `abs 0xea568` | `-` |
| `str` at 0xea53c | store | 4 | `sp -0x28` | `0xfffff7c4` |
| `str` at 0xea548 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xea54c | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xea550 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xea550 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xea550 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xea48c | `(unknown callee) 0xeb27c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xea4b4 | `(unknown callee) 0xe9080` | `ret(0xeb27c)` | `?` | `?` | `ret(0xeb27c)` |
| `mov` at 0xea528 | `(unknown callee) (unknown target)` | `[[sp-36]+25]:1` | `?` | `0x5` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xea4a0 | `ret(0xeb27c) != 0x0` | `ret(0xeb27c) == 0x0` |
| `beq` at 0xea4c0 | `ret(0xe9080) == 0x0` | `ret(0xe9080) != 0x0` |
| `beq` at 0xea534 | `ret(unknown) == 0x1` | `ret(unknown) != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xea550 | `r0` | `?` |

## `JChip_MMR_Write`

- `JChip_MMR_Write`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xea56c to 0xea678.
  Established. The indirect call carries `[[sp-40]+25]:1`, unknown, `[0x142b64]`, `[sp-28]` to an
  unknown target (`mov` at 0xea630). A word read obtains its callback pointer at `[[sp-40]] +0x18`
  (`ldr` at 0xea618). The callback-result branch takes its equality arm for `ret(unknown) == 0x1`
  and falls through otherwise (`beq` at 0xea63c). The r0 return cannot be resolved (`ldmdb` at
  0xea658). Neither the callback name nor a physical register-write effect is proved here.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xea57c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xea580 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xea584 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xea588 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0xea58c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xea598 | store | 4 | `sp -0x28` | `ret(0xeb27c)` |
| `ldr` at 0xea59c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xea5a8 | load | 4 | `abs 0xea65c` | `-` |
| `str` at 0xea5ac | store | 4 | `sp -0x2c` | `0xfffff7c6` |
| `ldr` at 0xea5b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xea5c8 | load | 4 | `abs 0xea660` | `-` |
| `str` at 0xea5cc | store | 4 | `sp -0x2c` | `0xfffff7c5` |
| `ldr` at 0xea5d4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xea5d8 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0xea5dc | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xea5e0 | load | 1 | `[sp-40] +0x19` | `-` |
| `ldr` at 0xea5e4 | load | 4 | `sp -0x28` | `-` |
| `ldrsb` at 0xea5e8 | load | 1 | `[sp-40] +0x21` | `-` |
| `ldr` at 0xea5ec | load | 4 | `abs 0xea664` | `-` |
| `ldr` at 0xea5f0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xea5f4 | store | 4 | `sp -0x40` | `[sp-32]` |
| `ldr` at 0xea5f8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xea5fc | store | 4 | `sp -0x3c` | `[sp-36]` |
| `ldr` at 0xea600 | load | 4 | `abs 0xea668` | `-` |
| `str` at 0xea604 | store | 4 | `sp -0x38` | `0x2ad28c` |
| `ldr` at 0xea608 | load | 4 | `abs 0xea66c` | `-` |
| `str` at 0xea60c | store | 4 | `sp -0x34` | `0x2ad30c` |
| `ldr` at 0xea610 | load | 4 | `abs 0xea670` | `-` |
| `str` at 0xea614 | store | 4 | `sp -0x30` | `0x2b3a78` |
| `ldr` at 0xea618 | load | 4 | `[[sp-40]] +0x18` | `-` |
| `ldr` at 0xea624 | load | 4 | `abs 0x142b64` | `-` |
| `ldr` at 0xea628 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea640 | load | 4 | `abs 0xea674` | `-` |
| `str` at 0xea644 | store | 4 | `sp -0x2c` | `0xfffff7c4` |
| `str` at 0xea650 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xea654 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xea658 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xea658 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xea658 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xea658 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xea590 | `(unknown callee) 0xeb27c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xea5b8 | `(unknown callee) 0xe9080` | `ret(0xeb27c)` | `?` | `?` | `ret(0xeb27c)` |
| `mov` at 0xea630 | `(unknown callee) (unknown target)` | `[[sp-40]+25]:1` | `?` | `[0x142b64]` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xea5a4 | `ret(0xeb27c) != 0x0` | `ret(0xeb27c) == 0x0` |
| `beq` at 0xea5c4 | `ret(0xe9080) == 0x0` | `ret(0xe9080) != 0x0` |
| `beq` at 0xea63c | `ret(unknown) == 0x1` | `ret(unknown) != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xea658 | `r0` | `?` |

## `JChip_MMR_SC_f`

- `JChip_MMR_SC_f`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xea678 to 0xeac50.
  Established. Calls include `0xea56c` with `[sp-28]`, `([sp-72] + [sp-76])`, `[sp-64]`, `[sp-68]`
  (`bl` at 0xeaa34); its zero result takes the continuation and nonzero falls through (`beq` at
  0xeaa40). A separate call to `0xea468` carries `[sp-28]`, unknown, `sp-64`, `sp-68` (`bl` at
  0xeab44). Two word stores at `[sp-20] +0x0` have unknown values (`str` at 0xeaa58, `str` at
  0xeab68). The command-field meanings and final r0 remain unknown (`ldmdb` at 0xeac34).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xea688 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xea690 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xea698 | load | 4 | `r0 +0xc` | `-` |
| `ldr` at 0xea6b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xea6b8 | load | 4 | `r0 +0xc` | `-` |
| `str` at 0xea6c4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xea6c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xea6cc | load | 4 | `r0 +0x0` | `-` |
| `str` at 0xea6d8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xea6dc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xea6e4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xea6e8 | load | 4 | `abs 0xeac38` | `-` |
| `ldr` at 0xea6ec | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xea6fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xea704 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0xea714 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xea71c | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0xea728 | load | 4 | `abs 0xeac3c` | `-` |
| `ldr` at 0xea738 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xea740 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xea750 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xea758 | load | 4 | `(unknown)` | `-` |
| `str` at 0xea774 | store | 4 | `sp -0x1c` | `ret(0xeb47c)` |
| `ldr` at 0xea778 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xea788 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xea790 | load | 4 | `abs 0xeac38` | `-` |
| `ldr` at 0xea794 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xea7a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea7ac | load | 4 | `sp -0x20` | `-` |
| `str` at 0xea7b0 | store | 4 | `sp -0x70` | `[sp-32]` |
| `ldr` at 0xea7b4 | load | 4 | `abs 0xeac40` | `-` |
| `ldr` at 0xea7bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea7c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xea7c8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xea7d8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xea7dc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xea7e4 | store | 4 | `sp -0x30` | `?` |
| `str` at 0xea7ec | store | 4 | `sp -0x34` | `0x2` |
| `ldr` at 0xea7f0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xea804 | store | 4 | `sp -0x54` | `0x4` |
| `str` at 0xea810 | store | 4 | `sp -0x54` | `0x2` |
| `ldr` at 0xea814 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xea818 | store | 4 | `sp -0x38` | `[sp-84]` |
| `ldr` at 0xea81c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xea82c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xea830 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xea83c | store | 4 | `sp -0x58` | `ret(0xfa70c)` |
| `ldr` at 0xea844 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xea848 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xea850 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xea854 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xea858 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xea864 | store | 4 | `sp -0x58` | `ret(0xfa70c)` |
| `ldr` at 0xea868 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xea86c | store | 4 | `sp -0x3c` | `ret(0xfa70c)` |
| `str` at 0xea878 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xea87c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xea890 | store | 4 | `sp -0x5c` | `0x2` |
| `str` at 0xea89c | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xea8a0 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xea8a4 | store | 4 | `sp -0x30` | `[sp-92]` |
| `str` at 0xea8ac | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0xea8b4 | store | 4 | `sp -0x38` | `0x2` |
| `ldr` at 0xea8b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xea8c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xea8cc | load | 4 | `sp -0x38` | `-` |
| `str` at 0xea8d8 | store | 4 | `sp -0x60` | `ret(0xfa70c)` |
| `str` at 0xea8e4 | store | 4 | `sp -0x60` | `0x1` |
| `ldr` at 0xea8e8 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xea8ec | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xea8f0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xea900 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xea904 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xea908 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xea918 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xea91c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xea924 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xea934 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xea940 | load | 4 | `(unknown)` | `-` |
| `str` at 0xea944 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xea948 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xea958 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xea960 | store | 4 | `sp -0x64` | `?` |
| `str` at 0xea96c | store | 4 | `sp -0x64` | `0x0` |
| `ldr` at 0xea970 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xea974 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xea978 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xea97c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xea984 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xea994 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xea9a0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xea9a4 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xea9a8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xea9ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xea9b4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xea9c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xea9d0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xea9d4 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xea9d8 | load | 4 | `abs 0xeac38` | `-` |
| `ldr` at 0xea9dc | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xea9e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea9f4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xea9f8 | store | 4 | `sp -0x70` | `[sp-64]` |
| `ldr` at 0xea9fc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xeaa00 | store | 4 | `sp -0x6c` | `[sp-68]` |
| `ldr` at 0xeaa04 | load | 4 | `abs 0xeac44` | `-` |
| `ldr` at 0xeaa0c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xeaa10 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xeaa18 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xeaa1c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xeaa24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeaa2c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xeaa30 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xeaa44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeaa48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeaa4c | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xeaa58 | store | 4 | `[sp-20] +0x0` | `?` |
| `str` at 0xeaa60 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xeaa68 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xeaa70 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0xeaa7c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xeaa80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeaa84 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xeaa94 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xeaa98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeaaa0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xeaab0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeaabc | load | 4 | `(unknown)` | `-` |
| `str` at 0xeaac0 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xeaac4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeaad4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xeaadc | store | 4 | `sp -0x68` | `?` |
| `str` at 0xeaae8 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0xeaaec | load | 4 | `sp -0x68` | `-` |
| `str` at 0xeaaf0 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xeaaf4 | load | 4 | `abs 0xeac38` | `-` |
| `ldr` at 0xeaaf8 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xeab04 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeab10 | load | 4 | `abs 0xeac48` | `-` |
| `ldr` at 0xeab18 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xeab1c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xeab24 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xeab28 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xeab38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeab54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeab58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeab5c | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xeab68 | store | 4 | `[sp-20] +0x0` | `?` |
| `str` at 0xeab70 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xeab78 | load | 4 | `abs 0xeac38` | `-` |
| `ldr` at 0xeab7c | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xeab88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeab94 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xeab98 | store | 4 | `sp -0x70` | `[sp-64]` |
| `ldr` at 0xeab9c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xeaba0 | store | 4 | `sp -0x6c` | `[sp-68]` |
| `ldr` at 0xeaba4 | load | 4 | `abs 0xeac4c` | `-` |
| `ldr` at 0xeabac | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xeabb0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xeabb8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xeabbc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeabc4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xeabd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeabe0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xeabe4 | store | 4 | `(unknown)` | `[sp-64]` |
| `ldr` at 0xeabe8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xeabec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeabf4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xeac04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeac10 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xeac14 | store | 4 | `(unknown)` | `[sp-68]` |
| `ldr` at 0xeac18 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xeac20 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0xeac2c | store | 4 | `sp -0x50` | `0x1` |
| `ldr` at 0xeac30 | load | 4 | `sp -0x50` | `-` |
| `ldmdb` at 0xeac34 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xeac34 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xeac34 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xea730 | `(unknown callee) 0x23f2c` | `0x1364c4` | `?` | `?` | `?` |
| `bl` at 0xea76c | `(unknown callee) 0xeb47c` | `?` | `?` | `0x4` | `?` |
| `bl` at 0xea7a4 | `(unknown callee) 0xea118` | `[sp-28]` | `?` | `?` | `[0x2ad25c]` |
| `bl` at 0xea7c4 | `(unknown callee) 0x23f2c` | `0x1364e4` | `ret(0xea118)` | `[sp-28]` | `[sp-36]` |
| `bl` at 0xea834 | `(unknown callee) 0xfa70c` | `[sp-32]` | `[sp-84]` | `?` | `?` |
| `bl` at 0xea85c | `(unknown callee) 0xfa70c` | `[sp-88]` | `[sp-56]` | `0x2` | `?` |
| `bl` at 0xea8d0 | `(unknown callee) 0xfa70c` | `[sp-32]` | `0x2` | `?` | `?` |
| `bl` at 0xea9ec | `(unknown callee) 0xea118` | `[sp-28]` | `0x14` | `?` | `[0x2ad25c]` |
| `bl` at 0xeaa14 | `(unknown callee) 0x23f2c` | `0x13651c` | `ret(0xea118)` | `[sp-72]` | `[sp-76]` |
| `bl` at 0xeaa34 | `(unknown callee) 0xea56c` | `[sp-28]` | `([sp-72] + [sp-76])` | `[sp-64]` | `[sp-68]` |
| `bl` at 0xeab08 | `(unknown callee) 0xea118` | `[sp-28]` | `0x14` | `?` | `[0x2ad25c]` |
| `bl` at 0xeab20 | `(unknown callee) 0x23f2c` | `0x136554` | `ret(0xea118)` | `[sp-76]` | `[sp-72]` |
| `bl` at 0xeab44 | `(unknown callee) 0xea468` | `[sp-28]` | `?` | `sp-64` | `sp-68` |
| `bl` at 0xeab8c | `(unknown callee) 0xea118` | `[sp-28]` | `?` | `?` | `[0x2ad25c]` |
| `bl` at 0xeabb4 | `(unknown callee) 0x23f2c` | `0x136580` | `ret(0xea118)` | `[sp-76]` | `[sp-72]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xea6f4 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xea780 | `Z == 0` | `not (Z == 0)` |
| `bls` at 0xea79c | `[0x2ad25c] <= 0x1 (unsigned, 32-bit)` | `[0x2ad25c] > 0x1 (unsigned, 32-bit)` |
| `beq` at 0xea7d0 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `beq` at 0xea7fc | `? == 0x0` | `? != 0x0` |
| `beq` at 0xea828 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xea888 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xea8c4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xea8f8 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `blt` at 0xea910 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xea954 | `? == 0x0` | `? != 0x0` |
| `bls` at 0xea9e4 | `[0x2ad25c] <= 0x1 (unsigned, 32-bit)` | `[0x2ad25c] > 0x1 (unsigned, 32-bit)` |
| `beq` at 0xeaa40 | `ret(0xea56c) == 0x0` | `ret(0xea56c) != 0x0` |
| `blt` at 0xeaa8c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xeaad0 | `? == 0x0` | `? != 0x0` |
| `bls` at 0xeab00 | `[0x2ad25c] <= 0x1 (unsigned, 32-bit)` | `[0x2ad25c] > 0x1 (unsigned, 32-bit)` |
| `beq` at 0xeab50 | `ret(0xea468) == 0x0` | `ret(0xea468) != 0x0` |
| `bls` at 0xeab84 | `[0x2ad25c] <= 0x1 (unsigned, 32-bit)` | `[0x2ad25c] > 0x1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xeac34 | `r0` | `?` |

## `JChip_TDR_Raw`

- `JChip_TDR_Raw`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeac50 to 0xeb1d0.
  Established. The call to `0xeb3bc` carries `[sp-56]`, `[sp-48]`, `[sp-52]`, `sp-24` (`bl` at
  0xeaf00). Later, `0xeb644` receives `[sp-64]`, `[sp-60]`, `[sp]`, `[sp-28]` (`bl` at 0xeb09c). The
  following call to `0xdcf54` uses `0x1`, `0x1`, `0x1`, `[sp-44]` (`bl` at 0xeb0d0); its nonzero
  result takes the continuation, while zero falls through (`bne` at 0xeb0dc). Raw scan encoding and
  the joined r0 return are unresolved (`ldmdb` at 0xeb178).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeac60 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xeac64 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xeac68 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xeac6c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xeac70 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xeac7c | store | 4 | `sp -0x28` | `ret(0xeb27c)` |
| `str` at 0xeac84 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xeac88 | load | 4 | `abs 0xeb17c` | `-` |
| `str` at 0xeac8c | store | 4 | `sp -0x44` | `0x2ad28c` |
| `ldr` at 0xeac90 | load | 4 | `abs 0xeb180` | `-` |
| `str` at 0xeac94 | store | 4 | `sp -0x48` | `0x2ad30c` |
| `ldr` at 0xeac98 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeaca4 | load | 4 | `abs 0xeb184` | `-` |
| `str` at 0xeaca8 | store | 4 | `sp -0x4c` | `0xfffff7c6` |
| `ldr` at 0xeacb0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeacc4 | load | 4 | `abs 0xeb188` | `-` |
| `str` at 0xeacc8 | store | 4 | `sp -0x4c` | `0xfffff7c5` |
| `ldr` at 0xeacd0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeacd4 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldrh` at 0xeacd8 | load | 2 | `[[sp-40]] +0xe` | `-` |
| `str` at 0xeacec | store | 4 | `sp -0x2c` | `0x1` |
| `str` at 0xeacf4 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xeacf8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeacfc | load | 4 | `[sp-40] +0x0` | `-` |
| `ldrb` at 0xead00 | load | 1 | `[[sp-40]] +0xc` | `-` |
| `str` at 0xead04 | store | 4 | `sp -0x34` | `[[[sp-40]]+12]:1` |
| `ldr` at 0xead08 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xead0c | store | 4 | `sp -0x38` | `[[[sp-40]]+12]:1` |
| `str` at 0xead14 | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0xead1c | store | 4 | `sp -0x40` | `0x1` |
| `ldr` at 0xead24 | load | 4 | `sp -0x28` | `-` |
| `ldrsb` at 0xead28 | load | 1 | `[sp-40] +0x21` | `-` |
| `str` at 0xead2c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xead30 | load | 4 | `abs 0xeb18c` | `-` |
| `ldr` at 0xead34 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xead38 | load | 1 | `[sp-40] +0x19` | `-` |
| `ldr` at 0xead58 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xead60 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xead68 | load | 2 | `(unknown)` | `-` |
| `str` at 0xead6c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xead70 | load | 4 | `abs 0xeb18c` | `-` |
| `ldr` at 0xead74 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xead78 | load | 1 | `[sp-40] +0x19` | `-` |
| `ldr` at 0xead98 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xeada0 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xeada8 | load | 1 | `(unknown)` | `-` |
| `str` at 0xeadac | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xeadb0 | load | 4 | `abs 0xeb18c` | `-` |
| `ldr` at 0xeadb4 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xeadb8 | load | 1 | `[sp-40] +0x19` | `-` |
| `ldr` at 0xeadd8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xeaddc | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xeade0 | load | 4 | `abs 0xeb18c` | `-` |
| `ldr` at 0xeade4 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xeade8 | load | 1 | `[sp-40] +0x19` | `-` |
| `ldr` at 0xeae08 | load | 4 | `(unknown)` | `-` |
| `str` at 0xeae0c | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xeae10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeae1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeae20 | load | 4 | `abs 0xeb190` | `-` |
| `ldr` at 0xeae2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeae34 | load | 4 | `abs 0xeb190` | `-` |
| `str` at 0xeae40 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xeae44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeae48 | load | 4 | `abs 0xeb194` | `-` |
| `ldr` at 0xeae54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeae5c | load | 4 | `abs 0xeb194` | `-` |
| `str` at 0xeae68 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xeae6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeae70 | load | 4 | `abs 0xeb198` | `-` |
| `ldr` at 0xeae7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeae84 | load | 4 | `abs 0xeb198` | `-` |
| `str` at 0xeae90 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xeae94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeaea4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xeaeb8 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xeaebc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeaecc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xeaee0 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xeaee8 | load | 4 | `abs 0xeb17c` | `-` |
| `str` at 0xeaeec | store | 4 | `sp -0x5c` | `0x2ad28c` |
| `ldr` at 0xeaef0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xeaef4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xeaef8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xeaf08 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xeaf0c | load | 4 | `abs 0xeb19c` | `-` |
| `ldr` at 0xeaf14 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xeaf18 | store | 4 | `sp -0x5c` | `[sp-56]` |
| `ldr` at 0xeaf1c | load | 4 | `abs 0xeb17c` | `-` |
| `str` at 0xeaf20 | store | 4 | `sp -0x58` | `0x2ad28c` |
| `ldr` at 0xeaf24 | load | 4 | `abs 0xeb180` | `-` |
| `str` at 0xeaf28 | store | 4 | `sp -0x54` | `0x2ad30c` |
| `ldr` at 0xeaf2c | load | 4 | `abs 0xeb19c` | `-` |
| `str` at 0xeaf30 | store | 4 | `sp -0x50` | `0x2b3a78` |
| `ldr` at 0xeaf40 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xeaf54 | load | 4 | `abs 0xeb1a0` | `-` |
| `str` at 0xeaf58 | store | 4 | `sp -0x5c` | `0x1365b4` |
| `ldr` at 0xeaf60 | load | 4 | `abs 0xeb1a4` | `-` |
| `ldr` at 0xeaf64 | load | 4 | `abs 0xeb1a8` | `-` |
| `ldr` at 0xeaf70 | load | 4 | `abs 0xeb1ac` | `-` |
| `str` at 0xeaf74 | store | 4 | `sp -0x4c` | `0xfffff7ca` |
| `ldr` at 0xeaf7c | load | 4 | `abs 0xeb1b0` | `-` |
| `ldr` at 0xeaf80 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xeaf8c | load | 4 | `abs 0xeb180` | `-` |
| `str` at 0xeaf90 | store | 4 | `sp -0x5c` | `0x2ad30c` |
| `ldr` at 0xeaf94 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xeaf98 | load | 4 | `abs 0xeb1b4` | `-` |
| `ldr` at 0xeaf9c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xeafa0 | load | 4 | `abs 0xeb17c` | `-` |
| `ldr` at 0xeafa8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeafb4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeafc4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeafd0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xeafd4 | store | 4 | `sp -0x1c` | `[sp-32]` |
| `ldr` at 0xeafd8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xeafe4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xeafe8 | store | 4 | `sp -0x44` | `[sp-28]` |
| `ldr` at 0xeafec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeaff8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xeaffc | store | 4 | `sp -0x48` | `[sp-32]` |
| `ldr` at 0xeb004 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xeb010 | load | 4 | `abs 0xeb1b8` | `-` |
| `str` at 0xeb014 | store | 4 | `sp -0x5c` | `0x1365e0` |
| `ldr` at 0xeb018 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xeb01c | store | 4 | `sp -0x58` | `[sp]` |
| `ldr` at 0xeb024 | load | 4 | `abs 0xeb1a4` | `-` |
| `ldr` at 0xeb028 | load | 4 | `abs 0xeb1bc` | `-` |
| `ldr` at 0xeb034 | load | 4 | `abs 0xeb1ac` | `-` |
| `str` at 0xeb038 | store | 4 | `sp -0x4c` | `0xfffff7ca` |
| `ldr` at 0xeb040 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xeb04c | load | 4 | `abs 0xeb1c0` | `-` |
| `str` at 0xeb050 | store | 4 | `sp -0x5c` | `0x136630` |
| `ldr` at 0xeb054 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xeb058 | store | 4 | `sp -0x58` | `[sp]` |
| `ldr` at 0xeb060 | load | 4 | `abs 0xeb1a4` | `-` |
| `ldr` at 0xeb064 | load | 4 | `abs 0xeb1c4` | `-` |
| `ldr` at 0xeb070 | load | 4 | `abs 0xeb1ac` | `-` |
| `str` at 0xeb074 | store | 4 | `sp -0x4c` | `0xfffff7ca` |
| `str` at 0xeb080 | store | 4 | `sp -0x5c` | `sp-36` |
| `ldr` at 0xeb084 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xeb088 | store | 4 | `sp -0x58` | `[sp-68]` |
| `ldr` at 0xeb08c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xeb090 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xeb094 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xeb098 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeb0a0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xeb0a4 | store | 4 | `sp -0x5c` | `[sp-36]` |
| `ldr` at 0xeb0a8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xeb0ac | store | 4 | `sp -0x58` | `[sp-68]` |
| `ldr` at 0xeb0b0 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xeb0b4 | store | 4 | `sp -0x54` | `[sp-72]` |
| `ldr` at 0xeb0b8 | load | 4 | `abs 0xeb19c` | `-` |
| `str` at 0xeb0bc | store | 4 | `sp -0x50` | `0x2b3a78` |
| `ldr` at 0xeb0cc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xeb0e0 | load | 4 | `abs 0xeb1c8` | `-` |
| `str` at 0xeb0e4 | store | 4 | `sp -0x5c` | `0x136690` |
| `ldr` at 0xeb0ec | load | 4 | `abs 0xeb1a4` | `-` |
| `ldr` at 0xeb0fc | load | 4 | `abs 0xeb1ac` | `-` |
| `str` at 0xeb100 | store | 4 | `sp -0x4c` | `0xfffff7ca` |
| `ldr` at 0xeb108 | load | 4 | `abs 0xeb1b0` | `-` |
| `ldr` at 0xeb10c | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xeb118 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xeb11c | store | 4 | `sp -0x5c` | `[sp-72]` |
| `ldr` at 0xeb120 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xeb124 | load | 4 | `abs 0xeb1cc` | `-` |
| `ldr` at 0xeb128 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeb12c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xeb134 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeb140 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xeb144 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeb150 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xeb154 | store | 4 | `sp -0x5c` | `[sp]` |
| `ldr` at 0xeb158 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xeb15c | load | 4 | `abs 0xeb180` | `-` |
| `ldr` at 0xeb164 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xeb170 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0xeb174 | load | 4 | `sp -0x4c` | `-` |
| `ldmdb` at 0xeb178 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xeb178 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xeb178 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeac74 | `(unknown callee) 0xeb27c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xeacb4 | `(unknown callee) 0xe9080` | `ret(0xeb27c)` | `?` | `?` | `ret(0xeb27c)` |
| `bl` at 0xeaf00 | `(unknown callee) 0xeb3bc` | `[sp-56]` | `[sp-48]` | `[sp-52]` | `sp-24` |
| `bl` at 0xeaf10 | `(unknown callee) 0xdc9a8` | `0x1` | `[sp-44]` | `0x2b3a78` | `?` |
| `bl` at 0xeaf44 | `(unknown callee) 0xdcf54` | `0x0` | `0x0` | `0x1` | `[sp-44]` |
| `bl` at 0xeaf6c | `(unknown callee) 0x21e28` | `0x3` | `0x13634c` | `0x376` | `0x0` |
| `bl` at 0xeafa4 | `(unknown callee) 0xeb734` | `[sp+4]` | `0x1365d8` | `[sp-56]` | `0x2ad28c` |
| `bl` at 0xeb030 | `(unknown callee) 0x21e28` | `0x3` | `0x13634c` | `0x39f` | `0x0` |
| `bl` at 0xeb06c | `(unknown callee) 0x21e28` | `0x3` | `0x13634c` | `0x3a7` | `0x0` |
| `bl` at 0xeb09c | `(unknown callee) 0xeb644` | `[sp-64]` | `[sp-60]` | `[sp]` | `[sp-28]` |
| `bl` at 0xeb0d0 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `[sp-44]` |
| `bl` at 0xeb0f8 | `(unknown callee) 0x21e28` | `0x3` | `0x13634c` | `0x3b0` | `0x0` |
| `bl` at 0xeb130 | `(unknown callee) 0xeb734` | `[sp+4]` | `0x1366b4` | `[sp-36]` | `[sp-68]` |
| `bl` at 0xeb168 | `(unknown callee) 0xe2174` | `[sp-60]` | `0x2ad30c` | `0x0` | `[sp-32]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xeaca0 | `ret(0xeb27c) != 0x0` | `ret(0xeb27c) == 0x0` |
| `beq` at 0xeacc0 | `ret(0xe9080) == 0x0` | `ret(0xe9080) != 0x0` |
| `beq` at 0xeace4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xeae18 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xeaf50 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xeaf88 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xeafb0 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xeafbc | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0xeafcc | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xeafe0 | `[sp-64] != 0x1` | `[sp-64] == 0x1` |
| `beq` at 0xeaff4 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `ble` at 0xeb00c | `[sp] <= 0x400 (signed, 32-bit)` | `[sp] > 0x400 (signed, 32-bit)` |
| `ble` at 0xeb048 | `[sp] <= 0x400 (signed, 32-bit)` | `[sp] > 0x400 (signed, 32-bit)` |
| `bne` at 0xeb0dc | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xeb114 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xeb13c | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `beq` at 0xeb14c | `[sp-72] == [sp-32]` | `[sp-72] != [sp-32]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xeb178 | `r0` | `?` |

## `JChip_ir_load`

- `JChip_ir_load`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xec394 to 0xec7d4.
  Established. A word store clears `0x2ad288` to `0x0` (`str` at 0xec614). The unsigned loop test
  takes its body for `[0x2ad288] < [sp-32]` and falls through for greater or equal (`blo` at
  0xec628). A later word store writes `[0x2ad288]+1` at `0x2ad288` (`str` at 0xec65c). The call to
  `0xdcf54` has `0x0`, `0x1`, `0x1`, `[sp-64]` (`bl` at 0xec6d0). Instruction-register loading and
  the combined return value remain unproved (`ldmdb` at 0xec784).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xec3a4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xec3a8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xec3ac | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xec3b4 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xec3b8 | load | 4 | `abs 0xec788` | `-` |
| `ldr` at 0xec3bc | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xec3c8 | load | 4 | `abs 0xec78c` | `-` |
| `ldr` at 0xec3cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xec3d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec3d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xec3dc | load | 4 | `abs 0xec790` | `-` |
| `str` at 0xec3e0 | store | 4 | `sp -0x38` | `0x2ad28c` |
| `ldr` at 0xec3e4 | load | 4 | `abs 0xec794` | `-` |
| `str` at 0xec3e8 | store | 4 | `sp -0x3c` | `0x2ad30c` |
| `ldr` at 0xec3ec | load | 4 | `sp -0x14` | `-` |
| `str` at 0xec3f8 | store | 4 | `sp -0x30` | `ret(0xeb27c)` |
| `ldr` at 0xec3fc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xec408 | load | 4 | `abs 0xec798` | `-` |
| `ldr` at 0xec40c | load | 4 | `abs 0xec79c` | `-` |
| `ldr` at 0xec410 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xec418 | load | 4 | `abs 0xec798` | `-` |
| `str` at 0xec41c | store | 4 | `sp -0x58` | `0x2b3a78` |
| `ldr` at 0xec424 | load | 4 | `abs 0xec7a0` | `-` |
| `ldr` at 0xec428 | load | 4 | `abs 0xec7a4` | `-` |
| `ldr` at 0xec434 | load | 4 | `abs 0xec7a8` | `-` |
| `str` at 0xec438 | store | 4 | `sp -0x48` | `0xfffff7c6` |
| `ldr` at 0xec440 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xec454 | load | 4 | `abs 0xec7ac` | `-` |
| `str` at 0xec458 | store | 4 | `sp -0x48` | `0xfffff7c5` |
| `ldr` at 0xec460 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xec464 | load | 4 | `[sp-48] +0x0` | `-` |
| `ldrh` at 0xec468 | load | 2 | `[[sp-48]] +0xe` | `-` |
| `str` at 0xec47c | store | 4 | `sp -0x40` | `0x1` |
| `str` at 0xec484 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xec488 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xec48c | load | 4 | `[sp-48] +0x0` | `-` |
| `ldrb` at 0xec490 | load | 1 | `[[sp-48]] +0xc` | `-` |
| `str` at 0xec494 | store | 4 | `sp -0x28` | `[[[sp-48]]+12]:1` |
| `ldr` at 0xec498 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xec49c | store | 4 | `sp -0x2c` | `[[[sp-48]]+12]:1` |
| `ldr` at 0xec4a4 | load | 4 | `sp -0x30` | `-` |
| `ldrsb` at 0xec4a8 | load | 1 | `[sp-48] +0x21` | `-` |
| `str` at 0xec4ac | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xec4b0 | load | 4 | `abs 0xec7b0` | `-` |
| `ldr` at 0xec4b4 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xec4b8 | load | 1 | `[sp-48] +0x19` | `-` |
| `ldr` at 0xec4d8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xec4e0 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xec4e8 | load | 2 | `(unknown)` | `-` |
| `str` at 0xec4ec | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xec4f0 | load | 4 | `abs 0xec7b0` | `-` |
| `ldr` at 0xec4f4 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xec4f8 | load | 1 | `[sp-48] +0x19` | `-` |
| `ldr` at 0xec518 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xec520 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xec528 | load | 1 | `(unknown)` | `-` |
| `str` at 0xec52c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xec530 | load | 4 | `abs 0xec7b0` | `-` |
| `ldr` at 0xec534 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xec538 | load | 1 | `[sp-48] +0x19` | `-` |
| `ldr` at 0xec558 | load | 4 | `(unknown)` | `-` |
| `str` at 0xec55c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xec560 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xec564 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xec56c | load | 4 | `[sp-48] +0x28` | `-` |
| `ldr` at 0xec574 | load | 4 | `(unknown)` | `-` |
| `str` at 0xec578 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xec57c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec58c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xec590 | load | 4 | `abs 0xec798` | `-` |
| `ldr` at 0xec5a4 | load | 4 | `abs 0xec798` | `-` |
| `str` at 0xec5a8 | store | 4 | `sp -0x58` | `0x2b3a78` |
| `ldr` at 0xec5b0 | load | 4 | `abs 0xec7a0` | `-` |
| `ldr` at 0xec5b4 | load | 4 | `abs 0xec7b4` | `-` |
| `ldr` at 0xec5c0 | load | 4 | `abs 0xec7b8` | `-` |
| `str` at 0xec5c4 | store | 4 | `sp -0x48` | `0xfffff7ca` |
| `ldr` at 0xec5cc | load | 4 | `abs 0xec788` | `-` |
| `ldr` at 0xec5d0 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xec5dc | load | 4 | `abs 0xec7bc` | `-` |
| `ldr` at 0xec5e0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xec5e8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xec5fc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xec608 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xec60c | load | 4 | `abs 0xec7c0` | `-` |
| `str` at 0xec614 | store | 4 | `abs 0x2ad288` | `0x0` |
| `ldr` at 0xec618 | load | 4 | `abs 0xec7c0` | `-` |
| `ldr` at 0xec61c | load | 4 | `abs 0x2ad288` | `-` |
| `ldr` at 0xec620 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xec630 | load | 4 | `abs 0xec7c0` | `-` |
| `ldr` at 0xec634 | load | 4 | `abs 0x2ad288` | `-` |
| `ldr` at 0xec63c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xec648 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xec64c | load | 4 | `abs 0xec7c0` | `-` |
| `ldr` at 0xec650 | load | 4 | `abs 0xec7c0` | `-` |
| `ldr` at 0xec654 | load | 4 | `abs 0x2ad288` | `-` |
| `str` at 0xec65c | store | 4 | `abs 0x2ad288` | `[0x2ad288]+1` |
| `ldr` at 0xec664 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xec668 | load | 4 | `sp -0x34` | `-` |
| `ldrh` at 0xec66c | load | 2 | `[sp-52] +0x4` | `-` |
| `str` at 0xec670 | store | 4 | `[sp-60] +0x0` | `[[sp-52]+4]:2` |
| `ldr` at 0xec67c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xec684 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xec688 | store | 4 | `sp -0x58` | `[sp-40]` |
| `ldr` at 0xec690 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xec694 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec698 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xec6a0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xec6a4 | store | 4 | `sp -0x58` | `[sp-44]` |
| `ldr` at 0xec6a8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xec6ac | store | 4 | `sp -0x54` | `[sp-56]` |
| `ldr` at 0xec6b0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xec6b4 | store | 4 | `sp -0x50` | `[sp-60]` |
| `ldr` at 0xec6b8 | load | 4 | `abs 0xec798` | `-` |
| `str` at 0xec6bc | store | 4 | `sp -0x4c` | `0x2b3a78` |
| `ldr` at 0xec6cc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xec6e0 | load | 4 | `abs 0xec798` | `-` |
| `str` at 0xec6e4 | store | 4 | `sp -0x58` | `0x2b3a78` |
| `ldr` at 0xec6ec | load | 4 | `abs 0xec7a0` | `-` |
| `ldr` at 0xec6f0 | load | 4 | `abs 0xec7c4` | `-` |
| `ldr` at 0xec6fc | load | 4 | `abs 0xec7b8` | `-` |
| `str` at 0xec700 | store | 4 | `sp -0x48` | `0xfffff7ca` |
| `ldr` at 0xec708 | load | 4 | `abs 0xec788` | `-` |
| `ldr` at 0xec70c | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xec718 | load | 4 | `abs 0xec7b0` | `-` |
| `ldr` at 0xec71c | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xec720 | load | 1 | `[sp-48] +0x19` | `-` |
| `ldr` at 0xec740 | load | 4 | `abs 0xec7c8` | `-` |
| `ldr` at 0xec744 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xec748 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec754 | load | 4 | `abs 0xec7cc` | `-` |
| `ldr` at 0xec758 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xec75c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xec768 | load | 4 | `abs 0xec7d0` | `-` |
| `ldr` at 0xec76c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xec770 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xec77c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xec780 | load | 4 | `sp -0x48` | `-` |
| `ldmdb` at 0xec784 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xec784 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xec784 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xec3d8 | `(unknown callee) 0x23f2c` | `0x136a04` | `r0` | `r1` | `r2` |
| `bl` at 0xec3f0 | `(unknown callee) 0xeb27c` | `?` | `?` | `?` | `0x2ad30c` |
| `bl` at 0xec414 | `(unknown callee) 0x1b49c` | `0x2b3a78` | `0x136a38` | `[sp-20]` | `ret(0xeb27c)` |
| `bl` at 0xec430 | `(unknown callee) 0x21e28` | `0x3` | `0x13634c` | `0x568` | `0x0` |
| `bl` at 0xec444 | `(unknown callee) 0xe9080` | `ret(0xeb27c)` | `?` | `?` | `ret(0xeb27c)` |
| `bl` at 0xec594 | `(unknown callee) 0xdca64` | `0x1` | `?` | `0x2b3a78` | `[sp-24]` |
| `bl` at 0xec5bc | `(unknown callee) 0x21e28` | `0x3` | `0x13634c` | `0x589` | `0x0` |
| `bl` at 0xec5e4 | `(unknown callee) 0x23f2c` | `0x136a68` | `[sp-64]` | `?` | `[0x2ad25c]` |
| `bl` at 0xec680 | `(unknown callee) 0xe235c` | `0x0` | `0x20` | `[sp-60]` | `[[sp-52]+4]:2` |
| `bl` at 0xec69c | `(unknown callee) 0xe2174` | `0x0` | `[sp-60]` | `[sp-36]` | `[sp-56]` |
| `bl` at 0xec6d0 | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `[sp-64]` |
| `bl` at 0xec6f8 | `(unknown callee) 0x21e28` | `0x3` | `0x13634c` | `0x59d` | `0x0` |
| `bl` at 0xec74c | `(unknown callee) 0x23f2c` | `0x136aa0` | `[sp-64]` | `?` | `?` |
| `bl` at 0xec760 | `(unknown callee) 0xf7858` | `0x0` | `0x136afc` | `[sp-32]` | `[sp-56]` |
| `bl` at 0xec774 | `(unknown callee) 0xf7858` | `0x0` | `0x136b08` | `[sp-32]` | `[sp-60]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xec3c4 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xec404 | `ret(0xeb27c) != 0x0` | `ret(0xeb27c) == 0x0` |
| `beq` at 0xec450 | `ret(0xe9080) == 0x0` | `ret(0xe9080) != 0x0` |
| `beq` at 0xec474 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xec584 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `bne` at 0xec5a0 | `ret(0xdca64) != 0x0` | `ret(0xdca64) == 0x0` |
| `bls` at 0xec5d8 | `[0x2ad25c] <= 0x3 (unsigned, 32-bit)` | `[0x2ad25c] > 0x3 (unsigned, 32-bit)` |
| `blo` at 0xec628 | `[0x2ad288] < [sp-32] (unsigned, 32-bit)` | `[0x2ad288] >= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xec6dc | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xec714 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xec784 | `r0` | `?` |

## `JChip_tdr_load`

- `JChip_tdr_load`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xec7d4 to 0xed150.
  Established. The call to `0xec394` carries `[sp-20]`, `0x0`, `[sp-28]`, unknown (`bl` at 0xec9bc);
  its signed result takes the continuation when at least `0x0` and falls through when negative
  (`bge` at 0xec9d0). Later word stores place `0x0` and `0x1` at `[sp-32] +0x0` (`str` at 0xed060,
  `str` at 0xed0b0). The facts leave both the data-register format and r0 at exit unknown (`ldmdb`
  at 0xed0d0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xec7e4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xec7e8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xec7ec | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xec7f0 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xec7f8 | store | 4 | `sp -0xd0` | `0x0` |
| `ldr` at 0xec7fc | load | 4 | `abs 0xed0d4` | `-` |
| `ldr` at 0xec800 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xec80c | load | 4 | `abs 0xed0d8` | `-` |
| `ldr` at 0xec810 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xec814 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec818 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xec820 | load | 4 | `abs 0xed0dc` | `-` |
| `str` at 0xec824 | store | 4 | `sp -0x48` | `0x2ad28c` |
| `ldr` at 0xec828 | load | 4 | `abs 0xed0e0` | `-` |
| `str` at 0xec82c | store | 4 | `sp -0x4c` | `0x2ad30c` |
| `ldr` at 0xec830 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xec83c | store | 4 | `sp -0x3c` | `ret(0xeb27c)` |
| `ldr` at 0xec840 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xec84c | load | 4 | `abs 0xed0e4` | `-` |
| `ldr` at 0xec850 | load | 4 | `abs 0xed0e8` | `-` |
| `ldr` at 0xec854 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xec85c | load | 4 | `abs 0xed0e4` | `-` |
| `str` at 0xec860 | store | 4 | `sp -0xe8` | `0x2b3a78` |
| `ldr` at 0xec868 | load | 4 | `abs 0xed0ec` | `-` |
| `ldr` at 0xec86c | load | 4 | `abs 0xed0f0` | `-` |
| `ldr` at 0xec878 | load | 4 | `abs 0xed0f4` | `-` |
| `str` at 0xec87c | store | 4 | `sp -0xd4` | `0xfffff7c6` |
| `ldr` at 0xec884 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xec898 | load | 4 | `abs 0xed0f8` | `-` |
| `str` at 0xec89c | store | 4 | `sp -0xd4` | `0xfffff7c5` |
| `ldr` at 0xec8a4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xec8a8 | load | 4 | `[sp-60] +0x0` | `-` |
| `ldrh` at 0xec8ac | load | 2 | `[[sp-60]] +0xe` | `-` |
| `str` at 0xec8c0 | store | 4 | `sp -0xd0` | `0x1` |
| `str` at 0xec8c8 | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0xec8d0 | store | 4 | `sp -0x38` | `0x1` |
| `ldr` at 0xec8d8 | load | 4 | `sp -0x3c` | `-` |
| `ldrsb` at 0xec8dc | load | 1 | `[sp-60] +0x21` | `-` |
| `str` at 0xec8e0 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xec8e4 | load | 4 | `abs 0xed0fc` | `-` |
| `ldr` at 0xec8e8 | load | 4 | `sp -0x3c` | `-` |
| `ldrb` at 0xec8ec | load | 1 | `[sp-60] +0x19` | `-` |
| `ldr` at 0xec90c | load | 4 | `(unknown)` | `-` |
| `str` at 0xec910 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xec914 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xec918 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xec920 | load | 4 | `[sp-60] +0x28` | `-` |
| `ldr` at 0xec928 | load | 4 | `(unknown)` | `-` |
| `str` at 0xec92c | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xec930 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec944 | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0xec948 | load | 4 | `abs 0xed0e4` | `-` |
| `ldr` at 0xec95c | load | 4 | `abs 0xed0e4` | `-` |
| `str` at 0xec960 | store | 4 | `sp -0xe8` | `0x2b3a78` |
| `ldr` at 0xec968 | load | 4 | `abs 0xed0ec` | `-` |
| `ldr` at 0xec96c | load | 4 | `abs 0xed100` | `-` |
| `ldr` at 0xec978 | load | 4 | `abs 0xed104` | `-` |
| `str` at 0xec97c | store | 4 | `sp -0xd4` | `0xfffff7ca` |
| `ldr` at 0xec984 | load | 4 | `abs 0xed0d4` | `-` |
| `ldr` at 0xec988 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xec994 | load | 4 | `abs 0xed108` | `-` |
| `ldr` at 0xec998 | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0xec9a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec9b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xec9b8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xec9c4 | store | 4 | `sp -0x44` | `ret(0xec394)` |
| `ldr` at 0xec9c8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xec9d4 | load | 4 | `abs 0xed0e4` | `-` |
| `str` at 0xec9d8 | store | 4 | `sp -0xe8` | `0x2b3a78` |
| `ldr` at 0xec9e0 | load | 4 | `abs 0xed0ec` | `-` |
| `ldr` at 0xec9e4 | load | 4 | `abs 0xed10c` | `-` |
| `ldr` at 0xec9f0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xec9f4 | store | 4 | `sp -0xd4` | `[sp-68]` |
| `ldr` at 0xec9fc | load | 4 | `abs 0xed0d4` | `-` |
| `ldr` at 0xeca00 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xeca10 | load | 4 | `abs 0xed110` | `-` |
| `ldr` at 0xeca14 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xeca20 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xeca24 | load | 4 | `[sp-64] +0x8` | `-` |
| `ldrh` at 0xeca28 | load | 2 | `[[sp-64]+8] +0x0` | `-` |
| `ldr` at 0xeca2c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xeca38 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xeca3c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeca50 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xeca5c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xeca60 | load | 4 | `abs 0xed114` | `-` |
| `str` at 0xeca68 | store | 4 | `abs 0x2ad288` | `0x0` |
| `ldr` at 0xeca6c | load | 4 | `abs 0xed114` | `-` |
| `ldr` at 0xeca70 | load | 4 | `abs 0x2ad288` | `-` |
| `ldr` at 0xeca74 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xeca84 | load | 4 | `abs 0xed114` | `-` |
| `ldr` at 0xeca88 | load | 4 | `abs 0x2ad288` | `-` |
| `ldr` at 0xeca90 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xeca9c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xecaa0 | load | 4 | `abs 0xed114` | `-` |
| `ldr` at 0xecaa4 | load | 4 | `abs 0xed114` | `-` |
| `ldr` at 0xecaa8 | load | 4 | `abs 0x2ad288` | `-` |
| `str` at 0xecab0 | store | 4 | `abs 0x2ad288` | `[0x2ad288]+1` |
| `ldr` at 0xecab8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xecabc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xecac0 | load | 4 | `[sp-64] +0x8` | `-` |
| `ldrh` at 0xecac4 | load | 2 | `[[sp-64]+8] +0x0` | `-` |
| `str` at 0xecac8 | store | 4 | `sp -0xe8` | `[[[sp-64]+8]]:2` |
| `ldr` at 0xecad0 | load | 4 | `[sp-64] +0x10` | `-` |
| `ldr` at 0xecad4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xecad8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xecae0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xecaf0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xecaf4 | store | 4 | `sp -0xe8` | `[sp-36]` |
| `ldr` at 0xecaf8 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xecafc | store | 4 | `sp -0xe4` | `[sp-72]` |
| `ldr` at 0xecb00 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xecb04 | store | 4 | `sp -0xe0` | `[sp-76]` |
| `ldr` at 0xecb08 | load | 4 | `abs 0xed0e4` | `-` |
| `str` at 0xecb0c | store | 4 | `sp -0xdc` | `0x2b3a78` |
| `ldr` at 0xecb1c | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0xecb30 | load | 4 | `abs 0xed0e4` | `-` |
| `str` at 0xecb34 | store | 4 | `sp -0xe8` | `0x2b3a78` |
| `ldr` at 0xecb3c | load | 4 | `abs 0xed0ec` | `-` |
| `ldr` at 0xecb40 | load | 4 | `abs 0xed118` | `-` |
| `ldr` at 0xecb4c | load | 4 | `abs 0xed104` | `-` |
| `str` at 0xecb50 | store | 4 | `sp -0xd4` | `0xfffff7ca` |
| `ldr` at 0xecb58 | load | 4 | `abs 0xed0d4` | `-` |
| `ldr` at 0xecb5c | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xecb68 | load | 4 | `abs 0xed11c` | `-` |
| `ldr` at 0xecb6c | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0xecb70 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xecb7c | load | 4 | `abs 0xed120` | `-` |
| `ldr` at 0xecb80 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xecb84 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xecb90 | load | 4 | `abs 0xed124` | `-` |
| `ldr` at 0xecb94 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xecb98 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xecba0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xecba4 | load | 4 | `[sp-64] +0x8` | `-` |
| `ldrh` at 0xecba8 | load | 2 | `[[sp-64]+8] +0x0` | `-` |
| `str` at 0xecbac | store | 4 | `sp -0xe8` | `[[[sp-64]+8]]:2` |
| `ldr` at 0xecbb0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xecbb4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xecbb8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xecbbc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xecbc4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xecbc8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xecbcc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xecbd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xecbe4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xecbe8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xecbec | load | 4 | `[sp-64] +0x8` | `-` |
| `ldrh` at 0xecbf0 | load | 2 | `[[sp-64]+8] +0x0` | `-` |
| `str` at 0xecbf4 | store | 4 | `sp -0xe8` | `[[[sp-64]+8]]:2` |
| `ldr` at 0xecbf8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xecbfc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xecc04 | load | 4 | `[sp-64] +0x10` | `-` |
| `str` at 0xecc10 | store | 4 | `sp -0x28` | `0x6` |
| `str` at 0xecc1c | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0xecc20 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xecc24 | store | 4 | `sp -0xe8` | `[sp-36]` |
| `ldr` at 0xecc28 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xecc2c | store | 4 | `sp -0xe4` | `[sp-72]` |
| `ldr` at 0xecc30 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xecc34 | store | 4 | `sp -0xe0` | `[sp-76]` |
| `ldr` at 0xecc38 | load | 4 | `abs 0xed0e4` | `-` |
| `str` at 0xecc3c | store | 4 | `sp -0xdc` | `0x2b3a78` |
| `ldr` at 0xecc44 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xecc4c | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0xecc60 | load | 4 | `abs 0xed0e4` | `-` |
| `str` at 0xecc64 | store | 4 | `sp -0xe8` | `0x2b3a78` |
| `ldr` at 0xecc6c | load | 4 | `abs 0xed0ec` | `-` |
| `ldr` at 0xecc7c | load | 4 | `abs 0xed104` | `-` |
| `str` at 0xecc80 | store | 4 | `sp -0xd4` | `0xfffff7ca` |
| `ldr` at 0xecc88 | load | 4 | `abs 0xed0d4` | `-` |
| `ldr` at 0xecc8c | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xecc98 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xecca4 | load | 4 | `abs 0xed128` | `-` |
| `str` at 0xecca8 | store | 4 | `sp -0xd8` | `0x136bf0` |
| `ldr` at 0xeccb0 | load | 4 | `abs 0xed12c` | `-` |
| `str` at 0xeccb4 | store | 4 | `sp -0xd8` | `0x136c00` |
| `ldr` at 0xeccb8 | load | 4 | `abs 0xed130` | `-` |
| `ldr` at 0xeccbc | load | 4 | `sp -0xd8` | `-` |
| `ldr` at 0xeccc0 | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0xeccc4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeccd0 | load | 4 | `abs 0xed134` | `-` |
| `ldr` at 0xeccd4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xeccd8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xecce4 | load | 4 | `abs 0xed138` | `-` |
| `ldr` at 0xecce8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xeccec | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xeccf4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xecd08 | load | 4 | `abs 0xed13c` | `-` |
| `ldr` at 0xecd0c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xecd18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xecd28 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xecd2c | store | 4 | `sp -0xe8` | `[sp-36]` |
| `ldr` at 0xecd30 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xecd34 | store | 4 | `sp -0xe4` | `[sp-72]` |
| `ldr` at 0xecd38 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xecd3c | store | 4 | `sp -0xe0` | `[sp-76]` |
| `ldr` at 0xecd40 | load | 4 | `abs 0xed0e4` | `-` |
| `str` at 0xecd44 | store | 4 | `sp -0xdc` | `0x2b3a78` |
| `ldr` at 0xecd54 | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0xecd68 | load | 4 | `abs 0xed0e4` | `-` |
| `str` at 0xecd6c | store | 4 | `sp -0xe8` | `0x2b3a78` |
| `ldr` at 0xecd74 | load | 4 | `abs 0xed0ec` | `-` |
| `ldr` at 0xecd78 | load | 4 | `abs 0xed140` | `-` |
| `ldr` at 0xecd84 | load | 4 | `abs 0xed104` | `-` |
| `str` at 0xecd88 | store | 4 | `sp -0xd4` | `0xfffff7ca` |
| `ldr` at 0xecd90 | load | 4 | `abs 0xed0d4` | `-` |
| `ldr` at 0xecd94 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xecda0 | load | 4 | `abs 0xed144` | `-` |
| `ldr` at 0xecda4 | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0xecda8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xecdb4 | load | 4 | `abs 0xed134` | `-` |
| `ldr` at 0xecdb8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xecdbc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xecdc8 | load | 4 | `abs 0xed138` | `-` |
| `ldr` at 0xecdcc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xecdd0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xecdd8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xecde4 | load | 4 | `abs 0xed114` | `-` |
| `str` at 0xecdec | store | 4 | `abs 0x2ad288` | `0x0` |
| `ldr` at 0xecdf0 | load | 4 | `abs 0xed114` | `-` |
| `ldr` at 0xecdf4 | load | 4 | `abs 0x2ad288` | `-` |
| `ldr` at 0xecdf8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xece08 | load | 4 | `abs 0xed114` | `-` |
| `ldr` at 0xece0c | load | 4 | `abs 0x2ad288` | `-` |
| `str` at 0xece28 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xece2c | load | 4 | `abs 0xed114` | `-` |
| `ldr` at 0xece30 | load | 4 | `abs 0xed114` | `-` |
| `ldr` at 0xece34 | load | 4 | `abs 0x2ad288` | `-` |
| `str` at 0xece3c | store | 4 | `abs 0x2ad288` | `[0x2ad288]+1` |
| `ldr` at 0xece48 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xece50 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xece58 | load | 4 | `abs 0xed0d4` | `-` |
| `ldr` at 0xece5c | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xece70 | load | 4 | `abs 0xed148` | `-` |
| `ldr` at 0xece74 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xece7c | load | 4 | `abs 0xed114` | `-` |
| `str` at 0xece84 | store | 4 | `abs 0x2ad288` | `0x0` |
| `ldr` at 0xece88 | load | 4 | `abs 0xed114` | `-` |
| `ldr` at 0xece8c | load | 4 | `abs 0x2ad288` | `-` |
| `ldr` at 0xece90 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xecea0 | load | 4 | `abs 0xed114` | `-` |
| `ldr` at 0xecea4 | load | 4 | `abs 0x2ad288` | `-` |
| `ldr` at 0xeceac | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xeceb4 | load | 4 | `abs 0xed114` | `-` |
| `ldr` at 0xeceb8 | load | 4 | `abs 0x2ad288` | `-` |
| `ldr` at 0xecec0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xecec8 | load | 4 | `abs 0xed114` | `-` |
| `ldr` at 0xececc | load | 4 | `abs 0x2ad288` | `-` |
| `ldr` at 0xecee4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xecee8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xecef0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xecef4 | load | 4 | `abs 0xed114` | `-` |
| `ldr` at 0xecef8 | load | 4 | `abs 0xed114` | `-` |
| `ldr` at 0xecefc | load | 4 | `abs 0x2ad288` | `-` |
| `str` at 0xecf04 | store | 4 | `abs 0x2ad288` | `[0x2ad288]+1` |
| `ldr` at 0xecf0c | load | 4 | `abs 0xed114` | `-` |
| `str` at 0xecf14 | store | 4 | `abs 0x2ad288` | `0x0` |
| `ldr` at 0xecf18 | load | 4 | `abs 0xed114` | `-` |
| `ldr` at 0xecf1c | load | 4 | `abs 0x2ad288` | `-` |
| `ldr` at 0xecf20 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xecf30 | load | 4 | `abs 0xed114` | `-` |
| `ldr` at 0xecf34 | load | 4 | `abs 0x2ad288` | `-` |
| `ldr` at 0xecf3c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xecf44 | load | 4 | `abs 0xed114` | `-` |
| `ldr` at 0xecf48 | load | 4 | `abs 0x2ad288` | `-` |
| `ldr` at 0xecf50 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xecf58 | load | 4 | `abs 0xed114` | `-` |
| `ldr` at 0xecf5c | load | 4 | `abs 0x2ad288` | `-` |
| `ldr` at 0xecf74 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xecf78 | load | 4 | `(unknown)` | `-` |
| `str` at 0xecf80 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xecf84 | load | 4 | `abs 0xed114` | `-` |
| `ldr` at 0xecf88 | load | 4 | `abs 0xed114` | `-` |
| `ldr` at 0xecf8c | load | 4 | `abs 0x2ad288` | `-` |
| `str` at 0xecf94 | store | 4 | `abs 0x2ad288` | `[0x2ad288]+1` |
| `ldr` at 0xecf9c | load | 4 | `abs 0xed114` | `-` |
| `str` at 0xecfa4 | store | 4 | `abs 0x2ad288` | `0x0` |
| `ldr` at 0xecfa8 | load | 4 | `abs 0xed114` | `-` |
| `ldr` at 0xecfac | load | 4 | `abs 0x2ad288` | `-` |
| `ldr` at 0xecfb0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xecfc0 | load | 4 | `abs 0xed114` | `-` |
| `ldr` at 0xecfc4 | load | 4 | `abs 0x2ad288` | `-` |
| `ldr` at 0xecfdc | load | 4 | `abs 0xed114` | `-` |
| `ldr` at 0xecfe0 | load | 4 | `abs 0x2ad288` | `-` |
| `ldr` at 0xecfe8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xecff0 | load | 4 | `abs 0xed114` | `-` |
| `ldr` at 0xecff4 | load | 4 | `abs 0x2ad288` | `-` |
| `ldr` at 0xecffc | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xed004 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xed008 | load | 4 | `(unknown)` | `-` |
| `str` at 0xed010 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xed014 | load | 4 | `abs 0xed114` | `-` |
| `ldr` at 0xed018 | load | 4 | `abs 0xed114` | `-` |
| `ldr` at 0xed01c | load | 4 | `abs 0x2ad288` | `-` |
| `str` at 0xed024 | store | 4 | `abs 0x2ad288` | `[0x2ad288]+1` |
| `ldr` at 0xed02c | load | 4 | `abs 0xed0d4` | `-` |
| `ldr` at 0xed030 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xed044 | load | 4 | `abs 0xed14c` | `-` |
| `ldr` at 0xed048 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xed054 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xed058 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xed060 | store | 4 | `[sp-32] +0x0` | `0x0` |
| `ldr` at 0xed064 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xed068 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xed074 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xed078 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xed084 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xed09c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xed0a8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xed0b0 | store | 4 | `[sp-32] +0x0` | `0x1` |
| `ldr` at 0xed0b4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xed0bc | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `str` at 0xed0c8 | store | 4 | `sp -0xd4` | `0x0` |
| `ldr` at 0xed0cc | load | 4 | `sp -0xd4` | `-` |
| `ldmdb` at 0xed0d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xed0d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xed0d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xec81c | `(unknown callee) 0x23f2c` | `0x136b14` | `r0` | `r1` | `r2` |
| `bl` at 0xec834 | `(unknown callee) 0xeb27c` | `?` | `?` | `?` | `0x2ad30c` |
| `bl` at 0xec858 | `(unknown callee) 0x1b49c` | `0x2b3a78` | `0x136a38` | `[sp-20]` | `ret(0xeb27c)` |
| `bl` at 0xec874 | `(unknown callee) 0x21e28` | `0x3` | `0x13634c` | `0x5c9` | `0x0` |
| `bl` at 0xec888 | `(unknown callee) 0xe9080` | `ret(0xeb27c)` | `?` | `?` | `ret(0xeb27c)` |
| `bl` at 0xec94c | `(unknown callee) 0xdca64` | `0x1` | `?` | `0x2b3a78` | `?` |
| `bl` at 0xec974 | `(unknown callee) 0x21e28` | `0x3` | `0x13634c` | `0x5e5` | `0x0` |
| `bl` at 0xec99c | `(unknown callee) 0x23f2c` | `0x136a68` | `[sp-208]` | `?` | `[0x2ad25c]` |
| `bl` at 0xec9bc | `(unknown callee) 0xec394` | `[sp-20]` | `0x0` | `[sp-28]` | `?` |
| `bl` at 0xec9ec | `(unknown callee) 0x21e28` | `0x3` | `0x13634c` | `0x5f1` | `0x0` |
| `bl` at 0xeca1c | `(unknown callee) 0xf09f8` | `0x0` | `0x136b50` | `[sp-64]` | `0x4` |
| `bl` at 0xecadc | `(unknown callee) 0xe2174` | `0x0` | `[[sp-64]+16]` | `[sp-52]` | `[sp-72]` |
| `bl` at 0xecb20 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x6` | `[sp-208]` |
| `bl` at 0xecb48 | `(unknown callee) 0x21e28` | `0x3` | `0x13634c` | `0x604` | `0x0` |
| `bl` at 0xecb74 | `(unknown callee) 0x23f2c` | `0x136b6c` | `[sp-208]` | `[sp-36]` | `[0x2ad25c]` |
| `bl` at 0xecb88 | `(unknown callee) 0xf7858` | `0x0` | `0x136bc8` | `[sp-44]` | `[sp-72]` |
| `bl` at 0xecb9c | `(unknown callee) 0xf7858` | `0x0` | `0x136bdc` | `[sp-44]` | `[sp-76]` |
| `bl` at 0xecbc0 | `(unknown callee) 0xe2174` | `[sp-52]` | `[sp-76]` | `[sp-52]` | `[sp-72]` |
| `bl` at 0xecbd0 | `(unknown callee) 0xf1714` | `[sp-64]` | `[sp-52]` | `[sp-72]` | `?` |
| `bl` at 0xecc08 | `(unknown callee) 0xe2174` | `[sp-52]` | `[sp-72]` | `0x0` | `[[sp-64]+16]` |
| `bl` at 0xecc50 | `(unknown callee) 0xdcf54` | `0x1` | `?` | `0x1` | `[sp-208]` |
| `bl` at 0xecc78 | `(unknown callee) 0x21e28` | `0x3` | `0x13634c` | `0x620` | `0x0` |
| `bl` at 0xeccc8 | `(unknown callee) 0x23f2c` | `0x136c0c` | `[sp-216]` | `[sp-208]` | `[sp-36]` |
| `bl` at 0xeccdc | `(unknown callee) 0xf7858` | `0x0` | `0x136c60` | `[sp-44]` | `[sp-72]` |
| `bl` at 0xeccf0 | `(unknown callee) 0xf7858` | `0x0` | `0x136c6c` | `[sp-44]` | `[sp-76]` |
| `bl` at 0xecd14 | `(unknown callee) 0xf09f8` | `0x0` | `0x136c78` | `[sp-64]` | `0x4` |
| `bl` at 0xecd58 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `[sp-208]` |
| `bl` at 0xecd80 | `(unknown callee) 0x21e28` | `0x3` | `0x13634c` | `0x633` | `0x0` |
| `bl` at 0xecdac | `(unknown callee) 0x23f2c` | `0x136c94` | `[sp-208]` | `[sp-36]` | `[0x2ad25c]` |
| `bl` at 0xecdc0 | `(unknown callee) 0xf7858` | `0x0` | `0x136c60` | `[sp-44]` | `[sp-72]` |
| `bl` at 0xecdd4 | `(unknown callee) 0xf7858` | `0x0` | `0x136c6c` | `[sp-44]` | `[sp-76]` |
| `bl` at 0xece54 | `(unknown callee) 0xf1820` | `[sp-64]` | `0x4` | `[sp-52]` | `sp-204` |
| `bl` at 0xece78 | `(unknown callee) 0xf7858` | `0x0` | `0x136cf0` | `[sp-44]` | `sp-204` |
| `bl` at 0xed04c | `(unknown callee) 0xf7858` | `0x0` | `0x136d00` | `[sp-44]` | `sp-204` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xec808 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xec848 | `ret(0xeb27c) != 0x0` | `ret(0xeb27c) == 0x0` |
| `beq` at 0xec894 | `ret(0xe9080) == 0x0` | `ret(0xe9080) != 0x0` |
| `beq` at 0xec8b8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xec93c | `? == 0x0` | `? != 0x0` |
| `bne` at 0xec958 | `ret(0xdca64) != 0x0` | `ret(0xdca64) == 0x0` |
| `bls` at 0xec990 | `[0x2ad25c] <= 0x3 (unsigned, 32-bit)` | `[0x2ad25c] > 0x3 (unsigned, 32-bit)` |
| `beq` at 0xec9ac | `? == 0x0` | `? != 0x0` |
| `bge` at 0xec9d0 | `ret(0xec394) >= 0x0 (signed, 32-bit)` | `ret(0xec394) < 0x0 (signed, 32-bit)` |
| `bls` at 0xeca08 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `blo` at 0xeca7c | `[0x2ad288] < [sp-44] (unsigned, 32-bit)` | `[0x2ad288] >= [sp-44] (unsigned, 32-bit)` |
| `beq` at 0xecaec | `? == 0x0` | `? != 0x0` |
| `bne` at 0xecb2c | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xecb64 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xecbe0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xecc5c | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xecc94 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xecca0 | `[sp-40] != 0x6` | `[sp-40] == 0x6` |
| `beq` at 0xecd00 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xecd24 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xecd64 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xecd9c | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xecde0 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `blo` at 0xece00 | `[0x2ad288] < [sp-44] (unsigned, 32-bit)` | `[0x2ad288] >= [sp-44] (unsigned, 32-bit)` |
| `bls` at 0xece64 | `[0x2ad25c] <= 0x3 (unsigned, 32-bit)` | `[0x2ad25c] > 0x3 (unsigned, 32-bit)` |
| `blo` at 0xece98 | `[0x2ad288] < [sp-44] (unsigned, 32-bit)` | `[0x2ad288] >= [sp-44] (unsigned, 32-bit)` |
| `blo` at 0xecf28 | `[0x2ad288] < [sp-44] (unsigned, 32-bit)` | `[0x2ad288] >= [sp-44] (unsigned, 32-bit)` |
| `blo` at 0xecfb8 | `[0x2ad288] < [sp-44] (unsigned, 32-bit)` | `[0x2ad288] >= [sp-44] (unsigned, 32-bit)` |
| `bls` at 0xed038 | `[0x2ad25c] <= 0x3 (unsigned, 32-bit)` | `[0x2ad25c] > 0x3 (unsigned, 32-bit)` |
| `bhs` at 0xed070 | `[sp-48] >= [sp-44] (unsigned, 32-bit)` | `[sp-48] < [sp-44] (unsigned, 32-bit)` |
| `bne` at 0xed080 | `[[sp-32]] != 0x0` | `[[sp-32]] == 0x0` |
| `beq` at 0xed0a4 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xed0d0 | `r0` | `?` |

## `PIOWrite_Multi`

- `PIOWrite_Multi`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xed150 to 0xed8b8.
  Established. The two calls to `0xe1cec` use `[sp-32]+64`, `[sp+20]`, unknown, `[sp-32]+64` and
  `[sp-32]+65`, `[sp+20]`, unknown, `[sp-32]+65`, respectively (`bl` at 0xed7f8, `bl` at 0xed818).
  The first result selects its taken arm on zero and falls through on nonzero (`beq` at 0xed804).
  The word store at `0x2ad394` writes `0x0` (`str` at 0xed6e0). Physical PIO effects and the merged
  r0 return are unidentified (`ldmdb` at 0xed888).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xed160 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xed164 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xed168 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xed16c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xed170 | load | 4 | `abs 0xed88c` | `-` |
| `ldr` at 0xed174 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xed194 | load | 4 | `(unknown)` | `-` |
| `str` at 0xed198 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xed19c | load | 4 | `abs 0xed88c` | `-` |
| `ldr` at 0xed1a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xed1c0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xed1c4 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xed1c8 | load | 4 | `abs 0xed88c` | `-` |
| `ldr` at 0xed1cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xed1ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xed1f4 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xed1fc | load | 1 | `(unknown)` | `-` |
| `str` at 0xed200 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xed204 | load | 4 | `abs 0xed88c` | `-` |
| `ldr` at 0xed208 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xed228 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xed230 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xed238 | load | 2 | `(unknown)` | `-` |
| `str` at 0xed23c | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xed240 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xed258 | load | 4 | `sp +0x18` | `-` |
| `str` at 0xed264 | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0xed270 | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0xed274 | load | 4 | `abs 0xed88c` | `-` |
| `ldr` at 0xed278 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xed298 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xed2a0 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xed2a8 | load | 1 | `(unknown)` | `-` |
| `str` at 0xed2b8 | store | 4 | `sp -0x44` | `0xd0000000` |
| `str` at 0xed2c0 | store | 4 | `sp -0x48` | `0x50000000` |
| `str` at 0xed2cc | store | 4 | `sp -0x44` | `0x8000000` |
| `str` at 0xed2d4 | store | 4 | `sp -0x48` | `0xf0000000` |
| `ldr` at 0xed2d8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xed2dc | store | 4 | `sp -0x38` | `[sp-68]` |
| `ldr` at 0xed2e4 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xed2e8 | store | 4 | `sp -0x6c` | `[sp+16]` |
| `ldr` at 0xed2ec | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xed2f0 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xed2f4 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xed300 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xed304 | store | 4 | `sp -0x6c` | `[sp-80]` |
| `ldr` at 0xed308 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xed30c | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xed310 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xed314 | store | 4 | `sp -0x64` | `[sp+20]` |
| `ldr` at 0xed318 | load | 4 | `sp +0x18` | `-` |
| `str` at 0xed31c | store | 4 | `sp -0x60` | `[sp+24]` |
| `ldr` at 0xed324 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xed344 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xed34c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xed350 | store | 4 | `sp -0x34` | `[sp+12]` |
| `ldr` at 0xed354 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xed358 | store | 4 | `sp -0x38` | `[sp+8]` |
| `str` at 0xed378 | store | 4 | `sp -0x6c` | `sp-36` |
| `ldr` at 0xed37c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xed380 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xed384 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xed388 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xed398 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xed39c | store | 4 | `sp -0x6c` | `[sp-36]` |
| `ldr` at 0xed3a0 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xed3a4 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xed3a8 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xed3ac | store | 4 | `sp -0x64` | `[sp+20]` |
| `ldr` at 0xed3b0 | load | 4 | `sp +0x18` | `-` |
| `str` at 0xed3b4 | store | 4 | `sp -0x60` | `[sp+24]` |
| `str` at 0xed3dc | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xed3e4 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xed3e8 | store | 4 | `sp -0x38` | `[sp-72]` |
| `ldr` at 0xed3f0 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xed3f4 | store | 4 | `sp -0x6c` | `[sp+16]` |
| `ldr` at 0xed3f8 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xed3fc | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xed400 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xed40c | load | 4 | `sp -0x50` | `-` |
| `str` at 0xed410 | store | 4 | `sp -0x6c` | `[sp-80]` |
| `ldr` at 0xed414 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xed418 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xed41c | load | 4 | `sp +0x14` | `-` |
| `str` at 0xed420 | store | 4 | `sp -0x64` | `[sp+20]` |
| `ldr` at 0xed424 | load | 4 | `sp +0x18` | `-` |
| `str` at 0xed428 | store | 4 | `sp -0x60` | `[sp+24]` |
| `str` at 0xed450 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xed458 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xed464 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xed468 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xed470 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xed474 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xed47c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xed480 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xed484 | load | 4 | `abs 0xed890` | `-` |
| `ldr` at 0xed490 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xed498 | load | 4 | `abs 0xed890` | `-` |
| `str` at 0xed4a4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xed4a8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xed4ac | load | 4 | `abs 0xed894` | `-` |
| `ldr` at 0xed4b8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xed4c0 | load | 4 | `abs 0xed894` | `-` |
| `str` at 0xed4cc | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xed4d0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xed4d4 | load | 4 | `abs 0xed898` | `-` |
| `ldr` at 0xed4e0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xed4e8 | load | 4 | `abs 0xed898` | `-` |
| `str` at 0xed4f4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xed4f8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xed508 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xed51c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xed520 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xed530 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xed544 | store | 4 | `sp -0x38` | `?` |
| `str` at 0xed550 | store | 4 | `sp -0x6c` | `sp-36` |
| `ldr` at 0xed554 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xed558 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xed55c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xed560 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xed570 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xed574 | store | 4 | `sp -0x6c` | `[sp-36]` |
| `ldr` at 0xed578 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xed57c | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xed580 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xed584 | store | 4 | `sp -0x64` | `[sp+20]` |
| `ldr` at 0xed588 | load | 4 | `sp +0x18` | `-` |
| `str` at 0xed58c | store | 4 | `sp -0x60` | `[sp+24]` |
| `str` at 0xed5b4 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xed5bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xed5cc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xed5d0 | store | 4 | `sp -0x6c` | `[sp-36]` |
| `ldr` at 0xed5d4 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xed5d8 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xed5dc | load | 4 | `sp +0x14` | `-` |
| `str` at 0xed5e0 | store | 4 | `sp -0x64` | `[sp+20]` |
| `ldr` at 0xed5e4 | load | 4 | `sp +0x18` | `-` |
| `str` at 0xed5e8 | store | 4 | `sp -0x60` | `[sp+24]` |
| `str` at 0xed610 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xed618 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xed628 | store | 4 | `sp -0x5c` | `0x1` |
| `ldr` at 0xed630 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xed634 | store | 4 | `sp -0x38` | `[sp-68]` |
| `ldr` at 0xed63c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xed640 | store | 4 | `sp -0x6c` | `[sp+16]` |
| `ldr` at 0xed644 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xed648 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xed64c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xed658 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xed65c | store | 4 | `sp -0x6c` | `[sp-80]` |
| `ldr` at 0xed660 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xed664 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xed668 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xed66c | store | 4 | `sp -0x64` | `[sp+20]` |
| `ldr` at 0xed670 | load | 4 | `sp +0x18` | `-` |
| `str` at 0xed674 | store | 4 | `sp -0x60` | `[sp+24]` |
| `str` at 0xed69c | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xed6a4 | load | 4 | `abs 0xed89c` | `-` |
| `ldr` at 0xed6a8 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xed6b4 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xed6b8 | store | 4 | `sp -0x6c` | `[sp+20]` |
| `ldr` at 0xed6bc | load | 4 | `abs 0xed8a0` | `-` |
| `ldr` at 0xed6c0 | load | 4 | `abs 0xed8a4` | `-` |
| `ldr` at 0xed6c4 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xed6c8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xed6d4 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xed6d8 | load | 4 | `abs 0xed8a8` | `-` |
| `str` at 0xed6e0 | store | 4 | `abs 0x2ad394` | `0x0` |
| `ldr` at 0xed6e4 | load | 4 | `abs 0xed8a8` | `-` |
| `ldr` at 0xed6e8 | load | 4 | `abs 0x2ad394` | `-` |
| `ldr` at 0xed6f8 | load | 4 | `abs 0xed8a8` | `-` |
| `ldr` at 0xed6fc | load | 4 | `abs 0x2ad394` | `-` |
| `str` at 0xed718 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xed71c | load | 4 | `abs 0xed8a8` | `-` |
| `ldr` at 0xed720 | load | 4 | `abs 0xed8a8` | `-` |
| `ldr` at 0xed724 | load | 4 | `abs 0x2ad394` | `-` |
| `str` at 0xed72c | store | 4 | `abs 0x2ad394` | `[0x2ad394]+1` |
| `str` at 0xed73c | store | 4 | `sp -0x6c` | `sp-36` |
| `ldr` at 0xed740 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xed744 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xed748 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xed74c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xed758 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xed764 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xed770 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xed774 | store | 4 | `sp -0x6c` | `[sp-36]` |
| `ldr` at 0xed778 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xed77c | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xed780 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xed784 | store | 4 | `sp -0x64` | `[sp+20]` |
| `ldr` at 0xed788 | load | 4 | `sp +0x18` | `-` |
| `str` at 0xed78c | store | 4 | `sp -0x60` | `[sp+24]` |
| `str` at 0xed7b4 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xed7bc | load | 4 | `abs 0xed89c` | `-` |
| `ldr` at 0xed7c0 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xed7cc | load | 4 | `sp +0x14` | `-` |
| `str` at 0xed7d0 | store | 4 | `sp -0x6c` | `[sp+20]` |
| `ldr` at 0xed7d4 | load | 4 | `abs 0xed8ac` | `-` |
| `ldr` at 0xed7d8 | load | 4 | `abs 0xed8a4` | `-` |
| `ldr` at 0xed7dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xed7e0 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xed7e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xed7f4 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xed808 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xed814 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xed828 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xed82c | load | 4 | `abs 0xed8b0` | `-` |
| `str` at 0xed838 | store | 4 | `sp -0x5c` | `0x0` |
| `str` at 0xed844 | store | 4 | `sp -0x40` | `0x1` |
| `ldr` at 0xed848 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xed850 | store | 4 | `sp +0x0` | `[sp]-1` |
| `ldr` at 0xed858 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xed864 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xed868 | load | 4 | `abs 0xed8b4` | `-` |
| `str` at 0xed874 | store | 4 | `sp -0x5c` | `0x0` |
| `str` at 0xed880 | store | 4 | `sp -0x5c` | `0x1` |
| `ldr` at 0xed884 | load | 4 | `sp -0x5c` | `-` |
| `ldmdb` at 0xed888 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xed888 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xed888 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xed25c | `(unknown callee) 0xdc9a8` | `0x1` | `0x0` | `[sp+24]` | `?` |
| `bl` at 0xed2fc | `(unknown callee) 0xeb3bc` | `[sp-80]` | `[sp-88]` | `[sp-84]` | `sp-56` |
| `bl` at 0xed330 | `(unknown callee) 0xdcf54` | `0x0` | `[sp-60]` | `0x1` | `0x0` |
| `bl` at 0xed36c | `(unknown callee) 0xe235c` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0xed394 | `(unknown callee) 0xeb644` | `[sp-76]` | `[sp-32]` | `0x42` | `sp-56` |
| `bl` at 0xed3c8 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xed408 | `(unknown callee) 0xeb3bc` | `[sp-80]` | `[sp-88]` | `[sp-84]` | `sp-56` |
| `bl` at 0xed43c | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xed56c | `(unknown callee) 0xeb644` | `[sp-76]` | `[sp-32]` | `0x1e` | `sp-56` |
| `bl` at 0xed5a0 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xed5fc | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xed654 | `(unknown callee) 0xeb3bc` | `[sp-80]` | `[sp-88]` | `[sp-84]` | `sp-56` |
| `bl` at 0xed688 | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xed6cc | `(unknown callee) 0xeb734` | `0x136d10` | `0x136d20` | `[sp-80]` | `[sp+16]` |
| `bl` at 0xed754 | `(unknown callee) 0xeb644` | `[sp-76]` | `[sp-32]` | `0x42` | `sp-56` |
| `bl` at 0xed7a0 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xed7e4 | `(unknown callee) 0xeb734` | `0x136d24` | `0x136d20` | `[sp-36]` | `[sp+16]` |
| `bl` at 0xed7f8 | `(unknown callee) 0xe1cec` | `[sp-32]+64` | `[sp+20]` | `?` | `[sp-32]+64` |
| `bl` at 0xed818 | `(unknown callee) 0xe1cec` | `[sp-32]+65` | `[sp+20]` | `?` | `[sp-32]+65` |
| `bl` at 0xed830 | `(unknown callee) 0x1b508` | `[sp+24]` | `0x136d3c` | `?` | `ret(0xe1cec)` |
| `bl` at 0xed86c | `(unknown callee) 0x1b508` | `[sp+24]` | `0x136d70` | `?` | `[sp-64]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xed24c | `? == 0x0` | `? != 0x0` |
| `bne` at 0xed2b0 | `? != 0x6` | `? == 0x6` |
| `bne` at 0xed33c | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bne` at 0xed3d4 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bne` at 0xed448 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bne` at 0xed5ac | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `beq` at 0xed5c8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xed608 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bne` at 0xed620 | `[sp] != 0x0` | `[sp] == 0x0` |
| `bne` at 0xed694 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xed6b0 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bls` at 0xed6f0 | `[0x2ad394] <= 0x2 (unsigned, 32-bit)` | `[0x2ad394] > 0x2 (unsigned, 32-bit)` |
| `ble` at 0xed760 | `[sp] <= 0x0 (signed, 32-bit)` | `[sp] > 0x0 (signed, 32-bit)` |
| `bne` at 0xed76c | `[sp-64] != 0x0` | `[sp-64] == 0x0` |
| `bne` at 0xed7ac | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xed7c8 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xed804 | `ret(0xe1cec) == 0x0` | `ret(0xe1cec) != 0x0` |
| `beq` at 0xed824 | `ret(0xe1cec) == 0x0` | `ret(0xe1cec) != 0x0` |
| `bne` at 0xed860 | `[sp-64] != 0x0` | `[sp-64] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xed888 | `r0` | `?` |

## `PIOWrite_Special`

- `PIOWrite_Special`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xed8b8 to 0xedff4.
  Established. The initial branch takes its bypass for entry `r0 != 0x0` and falls through on zero
  (`bne` at 0xed9b0). The fall-through call to `0xdc9a8` carries `0x1`, `0x0`, `[sp+20]`, entry r0
  (`bl` at 0xed9c0). A later call to `0xeb644` has `[sp-72]`, unknown, `0x1e`, `sp-56` (`bl` at
  0xedcd4). The r0 result at the common return is unknown (`ldmdb` at 0xedfc0). The
  special-operation selector has no established hardware interpretation.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xed8c8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xed8cc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xed8d0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xed8d4 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xed8d8 | load | 4 | `abs 0xedfc4` | `-` |
| `ldr` at 0xed8dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xed8fc | load | 4 | `(unknown)` | `-` |
| `str` at 0xed900 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xed904 | load | 4 | `abs 0xedfc4` | `-` |
| `ldr` at 0xed908 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xed928 | load | 4 | `(unknown)` | `-` |
| `str` at 0xed92c | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xed930 | load | 4 | `abs 0xedfc4` | `-` |
| `ldr` at 0xed934 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xed954 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xed95c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xed964 | load | 1 | `(unknown)` | `-` |
| `str` at 0xed968 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xed96c | load | 4 | `abs 0xedfc4` | `-` |
| `ldr` at 0xed970 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xed990 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xed998 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xed9a0 | load | 2 | `(unknown)` | `-` |
| `str` at 0xed9a4 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xed9a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xed9bc | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xed9c4 | load | 4 | `abs 0xedfc4` | `-` |
| `ldr` at 0xed9c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xed9e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xed9f0 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xed9f8 | load | 1 | `(unknown)` | `-` |
| `str` at 0xeda08 | store | 4 | `sp -0x40` | `0xd0000000` |
| `str` at 0xeda10 | store | 4 | `sp -0x44` | `0x50000000` |
| `str` at 0xeda1c | store | 4 | `sp -0x40` | `0x8000000` |
| `str` at 0xeda24 | store | 4 | `sp -0x44` | `0xf0000000` |
| `ldr` at 0xeda28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeda34 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xeda38 | store | 4 | `sp -0x38` | `[sp-64]` |
| `ldr` at 0xeda40 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xeda44 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xeda48 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xeda4c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xeda50 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xeda5c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xeda60 | store | 4 | `sp -0x68` | `[sp-76]` |
| `ldr` at 0xeda64 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xeda68 | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0xeda6c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xeda70 | store | 4 | `sp -0x60` | `[sp+16]` |
| `ldr` at 0xeda74 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xeda78 | store | 4 | `sp -0x5c` | `[sp+20]` |
| `str` at 0xedaa0 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xedaa8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xedaac | store | 4 | `sp -0x34` | `[sp+8]` |
| `ldr` at 0xedab0 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xedab4 | store | 4 | `sp -0x38` | `[sp+4]` |
| `str` at 0xedad4 | store | 4 | `sp -0x68` | `sp-36` |
| `ldr` at 0xedad8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xedadc | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0xedae0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xedae4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xedaf4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xedaf8 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xedafc | load | 4 | `sp +0xc` | `-` |
| `str` at 0xedb00 | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0xedb04 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xedb08 | store | 4 | `sp -0x60` | `[sp+16]` |
| `ldr` at 0xedb0c | load | 4 | `sp +0x14` | `-` |
| `str` at 0xedb10 | store | 4 | `sp -0x5c` | `[sp+20]` |
| `str` at 0xedb38 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xedb40 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xedb44 | store | 4 | `sp -0x38` | `[sp-68]` |
| `ldr` at 0xedb4c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xedb50 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xedb54 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xedb58 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xedb5c | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xedb68 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xedb6c | store | 4 | `sp -0x68` | `[sp-76]` |
| `ldr` at 0xedb70 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xedb74 | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0xedb78 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xedb7c | store | 4 | `sp -0x60` | `[sp+16]` |
| `ldr` at 0xedb80 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xedb84 | store | 4 | `sp -0x5c` | `[sp+20]` |
| `str` at 0xedbac | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xedbb8 | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0xedbc0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xedbcc | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xedbd0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xedbd8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xedbdc | load | 4 | `sp -0x38` | `-` |
| `str` at 0xedbe4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xedbe8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xedbec | load | 4 | `abs 0xedfc8` | `-` |
| `ldr` at 0xedbf8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xedc00 | load | 4 | `abs 0xedfc8` | `-` |
| `str` at 0xedc0c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xedc10 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xedc14 | load | 4 | `abs 0xedfcc` | `-` |
| `ldr` at 0xedc20 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xedc28 | load | 4 | `abs 0xedfcc` | `-` |
| `str` at 0xedc34 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xedc38 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xedc3c | load | 4 | `abs 0xedfd0` | `-` |
| `ldr` at 0xedc48 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xedc50 | load | 4 | `abs 0xedfd0` | `-` |
| `str` at 0xedc5c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xedc60 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xedc70 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xedc84 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xedc88 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xedc98 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xedcac | store | 4 | `sp -0x38` | `?` |
| `str` at 0xedcb8 | store | 4 | `sp -0x68` | `sp-36` |
| `ldr` at 0xedcbc | load | 4 | `sp +0xc` | `-` |
| `str` at 0xedcc0 | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0xedcc4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xedcc8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xedcd8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xedcdc | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xedce0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xedce4 | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0xedce8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xedcec | store | 4 | `sp -0x60` | `[sp+16]` |
| `ldr` at 0xedcf0 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xedcf4 | store | 4 | `sp -0x5c` | `[sp+20]` |
| `str` at 0xedd1c | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xedd24 | load | 4 | `abs 0xedfd4` | `-` |
| `ldr` at 0xedd28 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xedd34 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xedd38 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xedd3c | load | 4 | `abs 0xedfd8` | `-` |
| `ldr` at 0xedd40 | load | 4 | `abs 0xedfdc` | `-` |
| `ldr` at 0xedd44 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xedd48 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xedd50 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xedd60 | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0xedd68 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xedd6c | store | 4 | `sp -0x38` | `[sp-64]` |
| `ldr` at 0xedd74 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xedd78 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xedd7c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xedd80 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xedd84 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xedd90 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xedd94 | store | 4 | `sp -0x68` | `[sp-76]` |
| `ldr` at 0xedd98 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xedd9c | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0xedda0 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xedda4 | store | 4 | `sp -0x60` | `[sp+16]` |
| `ldr` at 0xedda8 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xeddac | store | 4 | `sp -0x5c` | `[sp+20]` |
| `str` at 0xeddd4 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xedddc | load | 4 | `abs 0xedfd4` | `-` |
| `ldr` at 0xedde0 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xeddec | load | 4 | `sp +0x10` | `-` |
| `str` at 0xeddf0 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xeddf4 | load | 4 | `abs 0xedfe0` | `-` |
| `ldr` at 0xeddf8 | load | 4 | `abs 0xedfdc` | `-` |
| `ldr` at 0xeddfc | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xede00 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xede0c | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xede10 | load | 4 | `abs 0xedfe4` | `-` |
| `str` at 0xede18 | store | 4 | `abs 0x2ad394` | `0x0` |
| `ldr` at 0xede1c | load | 4 | `abs 0xedfe4` | `-` |
| `ldr` at 0xede20 | load | 4 | `abs 0x2ad394` | `-` |
| `ldr` at 0xede30 | load | 4 | `abs 0xedfe4` | `-` |
| `ldr` at 0xede34 | load | 4 | `abs 0x2ad394` | `-` |
| `str` at 0xede50 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xede54 | load | 4 | `abs 0xedfe4` | `-` |
| `ldr` at 0xede58 | load | 4 | `abs 0xedfe4` | `-` |
| `ldr` at 0xede5c | load | 4 | `abs 0x2ad394` | `-` |
| `str` at 0xede64 | store | 4 | `abs 0x2ad394` | `[0x2ad394]+1` |
| `str` at 0xede74 | store | 4 | `sp -0x68` | `sp-36` |
| `ldr` at 0xede78 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xede7c | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0xede80 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xede84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xede90 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xede9c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xedea8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xedeac | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xedeb0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xedeb4 | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0xedeb8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xedebc | store | 4 | `sp -0x60` | `[sp+16]` |
| `ldr` at 0xedec0 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xedec4 | store | 4 | `sp -0x5c` | `[sp+20]` |
| `str` at 0xedeec | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xedef4 | load | 4 | `abs 0xedfd4` | `-` |
| `ldr` at 0xedef8 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xedf04 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xedf08 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xedf0c | load | 4 | `abs 0xedfe8` | `-` |
| `ldr` at 0xedf10 | load | 4 | `abs 0xedfdc` | `-` |
| `ldr` at 0xedf14 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xedf18 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xedf20 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xedf2c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xedf40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xedf4c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xedf60 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xedf64 | load | 4 | `abs 0xedfec` | `-` |
| `str` at 0xedf70 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xedf7c | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0xedf80 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xedf88 | store | 4 | `sp -0x20` | `[sp-32]-1` |
| `ldr` at 0xedf90 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xedf9c | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xedfa0 | load | 4 | `abs 0xedff0` | `-` |
| `str` at 0xedfac | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xedfb8 | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0xedfbc | load | 4 | `sp -0x58` | `-` |
| `ldmdb` at 0xedfc0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xedfc0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xedfc0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xed9c0 | `(unknown callee) 0xdc9a8` | `0x1` | `0x0` | `[sp+20]` | `r0` |
| `bl` at 0xeda58 | `(unknown callee) 0xeb3bc` | `[sp-76]` | `[sp-84]` | `[sp-80]` | `sp-56` |
| `bl` at 0xeda8c | `(unknown callee) 0xdcf54` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0xedac8 | `(unknown callee) 0xe235c` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0xedaf0 | `(unknown callee) 0xeb644` | `[sp-72]` | `[sp-28]` | `0x42` | `sp-56` |
| `bl` at 0xedb24 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xedb64 | `(unknown callee) 0xeb3bc` | `[sp-76]` | `[sp-84]` | `[sp-80]` | `sp-56` |
| `bl` at 0xedb98 | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xedcd4 | `(unknown callee) 0xeb644` | `[sp-72]` | `?` | `0x1e` | `sp-56` |
| `bl` at 0xedd08 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xedd4c | `(unknown callee) 0xeb734` | `0x136da8` | `0x136d20` | `[sp-36]` | `[sp+12]` |
| `bl` at 0xedd8c | `(unknown callee) 0xeb3bc` | `[sp-76]` | `[sp-84]` | `[sp-80]` | `sp-56` |
| `bl` at 0xeddc0 | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xede04 | `(unknown callee) 0xeb734` | `0x136d10` | `0x136d20` | `[sp-76]` | `[sp+12]` |
| `bl` at 0xede8c | `(unknown callee) 0xeb644` | `[sp-72]` | `[sp-28]` | `0x42` | `sp-56` |
| `bl` at 0xeded8 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xedf1c | `(unknown callee) 0xeb734` | `0x136d24` | `0x136d20` | `[sp-36]` | `[sp+12]` |
| `bl` at 0xedf30 | `(unknown callee) 0xe1cec` | `[sp-28]+64` | `[sp+16]` | `?` | `[sp-28]+64` |
| `bl` at 0xedf50 | `(unknown callee) 0xe1cec` | `[sp-28]+65` | `[sp+16]` | `?` | `[sp-28]+65` |
| `bl` at 0xedf68 | `(unknown callee) 0x1b508` | `[sp+20]` | `0x136d3c` | `?` | `ret(0xe1cec)` |
| `bl` at 0xedfa4 | `(unknown callee) 0x1b508` | `[sp+20]` | `0x136d70` | `?` | `[sp-60]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xed9b0 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xeda00 | `? != 0x6` | `? == 0x6` |
| `bne` at 0xeda30 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xeda98 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bne` at 0xedb30 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bne` at 0xedba4 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bne` at 0xedd14 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xedd30 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xedd58 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0xeddcc | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xedde8 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bls` at 0xede28 | `[0x2ad394] <= 0x2 (unsigned, 32-bit)` | `[0x2ad394] > 0x2 (unsigned, 32-bit)` |
| `ble` at 0xede98 | `[sp-32] <= 0x0 (signed, 32-bit)` | `[sp-32] > 0x0 (signed, 32-bit)` |
| `bne` at 0xedea4 | `[sp-60] != 0x0` | `[sp-60] == 0x0` |
| `bne` at 0xedee4 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xedf00 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xedf3c | `ret(0xe1cec) == 0x0` | `ret(0xe1cec) != 0x0` |
| `beq` at 0xedf5c | `ret(0xe1cec) == 0x0` | `ret(0xe1cec) != 0x0` |
| `bne` at 0xedf98 | `[sp-60] != 0x0` | `[sp-60] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xedfc0 | `r0` | `?` |

## `PIOWrite_Common`

- `PIOWrite_Common`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xedff4 to 0xee798.
  Established. The call to `0xeb644` includes candidates `[sp-72]`, `[sp-24]`, `0x42`, `sp-56` (`bl`
  at 0xee240). Later it is called with `[sp-72]`, `[sp-24]`, `0x1e`, `sp-56` (`bl` at 0xee470). The
  signed branch takes its exit arm for `[sp-28] <= 0x0` and falls through for greater (`ble` at
  0xee634). Another word store writes `0x0` at `0x2ad394` (`str` at 0xee5b4). Transaction encoding
  and the final r0 value remain unknown (`ldmdb` at 0xee75c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xee004 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xee008 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xee00c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xee010 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xee014 | load | 4 | `abs 0xee760` | `-` |
| `ldr` at 0xee018 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xee038 | load | 4 | `(unknown)` | `-` |
| `str` at 0xee03c | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xee040 | load | 4 | `abs 0xee760` | `-` |
| `ldr` at 0xee044 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xee064 | load | 4 | `(unknown)` | `-` |
| `str` at 0xee068 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xee06c | load | 4 | `abs 0xee760` | `-` |
| `ldr` at 0xee070 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xee090 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xee098 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xee0a0 | load | 1 | `(unknown)` | `-` |
| `str` at 0xee0a4 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xee0a8 | load | 4 | `abs 0xee760` | `-` |
| `ldr` at 0xee0ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xee0cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xee0d4 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xee0dc | load | 2 | `(unknown)` | `-` |
| `str` at 0xee0e0 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xee0ec | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xee0f4 | load | 4 | `abs 0xee760` | `-` |
| `ldr` at 0xee0f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xee118 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xee120 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xee128 | load | 1 | `(unknown)` | `-` |
| `str` at 0xee138 | store | 4 | `sp -0x40` | `0xd0000000` |
| `str` at 0xee140 | store | 4 | `sp -0x44` | `0x50000000` |
| `str` at 0xee14c | store | 4 | `sp -0x40` | `0x8000000` |
| `str` at 0xee154 | store | 4 | `sp -0x44` | `0xf0000000` |
| `ldr` at 0xee158 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xee15c | store | 4 | `sp -0x38` | `[sp-64]` |
| `ldr` at 0xee164 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xee168 | store | 4 | `sp -0x68` | `[sp+8]` |
| `ldr` at 0xee16c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xee170 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xee174 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xee180 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xee184 | store | 4 | `sp -0x68` | `[sp-76]` |
| `ldr` at 0xee188 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xee18c | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xee190 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xee194 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xee198 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xee19c | store | 4 | `sp -0x5c` | `[sp+16]` |
| `str` at 0xee1c4 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xee1cc | load | 4 | `abs 0xee764` | `-` |
| `ldr` at 0xee1d0 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xee1dc | load | 4 | `sp +0xc` | `-` |
| `str` at 0xee1e0 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xee1e4 | load | 4 | `abs 0xee768` | `-` |
| `ldr` at 0xee1e8 | load | 4 | `abs 0xee76c` | `-` |
| `ldr` at 0xee1ec | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xee1f0 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xee1f8 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xee1fc | store | 4 | `sp -0x34` | `[sp+4]` |
| `ldr` at 0xee200 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xee204 | store | 4 | `sp -0x38` | `[sp]` |
| `str` at 0xee224 | store | 4 | `sp -0x68` | `sp-36` |
| `ldr` at 0xee228 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xee22c | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xee230 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xee234 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xee244 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xee248 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xee24c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xee250 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xee254 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xee258 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xee25c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xee260 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `str` at 0xee288 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xee290 | load | 4 | `abs 0xee764` | `-` |
| `ldr` at 0xee294 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xee2a0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xee2a4 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xee2a8 | load | 4 | `abs 0xee770` | `-` |
| `ldr` at 0xee2ac | load | 4 | `abs 0xee76c` | `-` |
| `ldr` at 0xee2b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xee2b4 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xee2bc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xee2c0 | store | 4 | `sp -0x38` | `[sp-68]` |
| `ldr` at 0xee2c8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xee2cc | store | 4 | `sp -0x68` | `[sp+8]` |
| `ldr` at 0xee2d0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xee2d4 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xee2d8 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xee2e4 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xee2e8 | store | 4 | `sp -0x68` | `[sp-76]` |
| `ldr` at 0xee2ec | load | 4 | `sp +0x8` | `-` |
| `str` at 0xee2f0 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xee2f4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xee2f8 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xee2fc | load | 4 | `sp +0x10` | `-` |
| `str` at 0xee300 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `str` at 0xee328 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xee330 | load | 4 | `abs 0xee764` | `-` |
| `ldr` at 0xee334 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xee340 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xee344 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xee348 | load | 4 | `abs 0xee774` | `-` |
| `ldr` at 0xee34c | load | 4 | `abs 0xee76c` | `-` |
| `ldr` at 0xee350 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xee354 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xee35c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xee368 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xee36c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xee374 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xee378 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xee380 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xee384 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xee388 | load | 4 | `abs 0xee778` | `-` |
| `ldr` at 0xee394 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xee39c | load | 4 | `abs 0xee778` | `-` |
| `str` at 0xee3a8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xee3ac | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xee3b0 | load | 4 | `abs 0xee77c` | `-` |
| `ldr` at 0xee3bc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xee3c4 | load | 4 | `abs 0xee77c` | `-` |
| `str` at 0xee3d0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xee3d4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xee3d8 | load | 4 | `abs 0xee780` | `-` |
| `ldr` at 0xee3e4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xee3ec | load | 4 | `abs 0xee780` | `-` |
| `str` at 0xee3f8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xee3fc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xee40c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xee420 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xee424 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xee434 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xee448 | store | 4 | `sp -0x38` | `?` |
| `str` at 0xee454 | store | 4 | `sp -0x68` | `sp-36` |
| `ldr` at 0xee458 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xee45c | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xee460 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xee464 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xee474 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xee478 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xee47c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xee480 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xee484 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xee488 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xee48c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xee490 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `str` at 0xee4b8 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xee4c0 | load | 4 | `abs 0xee764` | `-` |
| `ldr` at 0xee4c4 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xee4d0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xee4d4 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xee4d8 | load | 4 | `abs 0xee784` | `-` |
| `ldr` at 0xee4dc | load | 4 | `abs 0xee76c` | `-` |
| `ldr` at 0xee4e0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xee4e4 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xee4ec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xee4fc | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0xee504 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xee508 | store | 4 | `sp -0x38` | `[sp-64]` |
| `ldr` at 0xee510 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xee514 | store | 4 | `sp -0x68` | `[sp+8]` |
| `ldr` at 0xee518 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xee51c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xee520 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xee52c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xee530 | store | 4 | `sp -0x68` | `[sp-76]` |
| `ldr` at 0xee534 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xee538 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xee53c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xee540 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xee544 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xee548 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `str` at 0xee570 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xee578 | load | 4 | `abs 0xee764` | `-` |
| `ldr` at 0xee57c | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xee588 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xee58c | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xee590 | load | 4 | `abs 0xee768` | `-` |
| `ldr` at 0xee594 | load | 4 | `abs 0xee76c` | `-` |
| `ldr` at 0xee598 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xee59c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xee5a8 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xee5ac | load | 4 | `abs 0xee788` | `-` |
| `str` at 0xee5b4 | store | 4 | `abs 0x2ad394` | `0x0` |
| `ldr` at 0xee5b8 | load | 4 | `abs 0xee788` | `-` |
| `ldr` at 0xee5bc | load | 4 | `abs 0x2ad394` | `-` |
| `ldr` at 0xee5cc | load | 4 | `abs 0xee788` | `-` |
| `ldr` at 0xee5d0 | load | 4 | `abs 0x2ad394` | `-` |
| `str` at 0xee5ec | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xee5f0 | load | 4 | `abs 0xee788` | `-` |
| `ldr` at 0xee5f4 | load | 4 | `abs 0xee788` | `-` |
| `ldr` at 0xee5f8 | load | 4 | `abs 0x2ad394` | `-` |
| `str` at 0xee600 | store | 4 | `abs 0x2ad394` | `[0x2ad394]+1` |
| `str` at 0xee610 | store | 4 | `sp -0x68` | `sp-36` |
| `ldr` at 0xee614 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xee618 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xee61c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xee620 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xee62c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xee638 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xee644 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xee648 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xee64c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xee650 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xee654 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xee658 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xee65c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xee660 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `str` at 0xee688 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xee690 | load | 4 | `abs 0xee764` | `-` |
| `ldr` at 0xee694 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xee6a0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xee6a4 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xee6a8 | load | 4 | `abs 0xee78c` | `-` |
| `ldr` at 0xee6ac | load | 4 | `abs 0xee76c` | `-` |
| `ldr` at 0xee6b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xee6b4 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xee6bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xee6c8 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xee6dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xee6e8 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xee6fc | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xee700 | load | 4 | `abs 0xee790` | `-` |
| `str` at 0xee70c | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xee718 | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0xee71c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xee724 | store | 4 | `sp -0x1c` | `[sp-28]-1` |
| `ldr` at 0xee72c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xee738 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xee73c | load | 4 | `abs 0xee794` | `-` |
| `str` at 0xee748 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xee754 | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0xee758 | load | 4 | `sp -0x58` | `-` |
| `ldmdb` at 0xee75c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xee75c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xee75c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xee0f0 | `(unknown callee) 0xdc9a8` | `0x1` | `0x0` | `[sp+16]` | `?` |
| `bl` at 0xee17c | `(unknown callee) 0xeb3bc` | `[sp-76]` | `[sp-84]` | `[sp-80]` | `sp-56` |
| `bl` at 0xee1b0 | `(unknown callee) 0xdcf54` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0xee1f4 | `(unknown callee) 0xeb734` | `0x136d10` | `0x136d20` | `[sp-76]` | `[sp+8]` |
| `bl` at 0xee218 | `(unknown callee) 0xe235c` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0xee240 | `(unknown callee) 0xeb644` | `[sp-72]` | `[sp-24]` | `0x42` | `sp-56` |
| `bl` at 0xee274 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xee2b8 | `(unknown callee) 0xeb734` | `0x136db8` | `0x136d20` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xee2e0 | `(unknown callee) 0xeb3bc` | `[sp-76]` | `[sp-84]` | `[sp-80]` | `sp-56` |
| `bl` at 0xee314 | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xee358 | `(unknown callee) 0xeb734` | `0x136dc8` | `0x136d20` | `[sp-76]` | `[sp+8]` |
| `bl` at 0xee470 | `(unknown callee) 0xeb644` | `[sp-72]` | `[sp-24]` | `0x1e` | `sp-56` |
| `bl` at 0xee4a4 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xee4e8 | `(unknown callee) 0xeb734` | `0x136da8` | `0x136d20` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xee528 | `(unknown callee) 0xeb3bc` | `[sp-76]` | `[sp-84]` | `[sp-80]` | `sp-56` |
| `bl` at 0xee55c | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xee5a0 | `(unknown callee) 0xeb734` | `0x136d10` | `0x136d20` | `[sp-76]` | `[sp+8]` |
| `bl` at 0xee628 | `(unknown callee) 0xeb644` | `[sp-72]` | `[sp-24]` | `0x42` | `sp-56` |
| `bl` at 0xee674 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xee6b8 | `(unknown callee) 0xeb734` | `0x136d24` | `0x136d20` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xee6cc | `(unknown callee) 0xe1cec` | `[sp-24]+64` | `[sp+12]` | `?` | `[sp-24]+64` |
| `bl` at 0xee6ec | `(unknown callee) 0xe1cec` | `[sp-24]+65` | `[sp+12]` | `?` | `[sp-24]+65` |
| `bl` at 0xee704 | `(unknown callee) 0x1b508` | `[sp+16]` | `0x136d3c` | `?` | `ret(0xe1cec)` |
| `bl` at 0xee740 | `(unknown callee) 0x1b508` | `[sp+16]` | `0x136d70` | `?` | `[sp-60]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xee130 | `? != 0x6` | `? == 0x6` |
| `bne` at 0xee1bc | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xee1d8 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xee280 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xee29c | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xee320 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xee33c | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xee4b0 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xee4cc | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xee4f4 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xee568 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xee584 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bls` at 0xee5c4 | `[0x2ad394] <= 0x2 (unsigned, 32-bit)` | `[0x2ad394] > 0x2 (unsigned, 32-bit)` |
| `ble` at 0xee634 | `[sp-28] <= 0x0 (signed, 32-bit)` | `[sp-28] > 0x0 (signed, 32-bit)` |
| `bne` at 0xee640 | `[sp-60] != 0x0` | `[sp-60] == 0x0` |
| `bne` at 0xee680 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xee69c | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xee6d8 | `ret(0xe1cec) == 0x0` | `ret(0xe1cec) != 0x0` |
| `beq` at 0xee6f8 | `ret(0xe1cec) == 0x0` | `ret(0xe1cec) != 0x0` |
| `bne` at 0xee734 | `[sp-60] != 0x0` | `[sp-60] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xee75c | `r0` | `?` |

## `PIORead_Common`

- `PIORead_Common`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xee798 to 0xeee2c.
  Established. Two word stores clear `[sp+4] +0x0` and `[sp] +0x0`, respectively (`str` at 0xee908,
  `str` at 0xee910). Later stores at those same printed targets carry `[sp-68]` and `[sp-72]`,
  respectively (`str` at 0xeed9c, `str` at 0xeeda8). The call to `0xe2174` has `[sp-24]`, `[sp+12]`,
  `0x0`, `sp-72` (`bl` at 0xeed7c). Hardware reply layout and the r0 return are not resolved
  (`ldmdb` at 0xeedf4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xee7a8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xee7ac | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xee7b0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xee7b4 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xee7b8 | load | 4 | `abs 0xeedf8` | `-` |
| `ldr` at 0xee7bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xee7dc | load | 4 | `(unknown)` | `-` |
| `str` at 0xee7e0 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xee7e4 | load | 4 | `abs 0xeedf8` | `-` |
| `ldr` at 0xee7e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xee808 | load | 4 | `(unknown)` | `-` |
| `str` at 0xee80c | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0xee810 | load | 4 | `abs 0xeedf8` | `-` |
| `ldr` at 0xee814 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xee834 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xee83c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xee844 | load | 1 | `(unknown)` | `-` |
| `str` at 0xee848 | store | 4 | `sp -0x60` | `?` |
| `ldr` at 0xee84c | load | 4 | `abs 0xeedf8` | `-` |
| `ldr` at 0xee850 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xee870 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xee878 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xee880 | load | 2 | `(unknown)` | `-` |
| `str` at 0xee884 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0xee890 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xee898 | load | 4 | `abs 0xeedf8` | `-` |
| `ldr` at 0xee89c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xee8bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xee8c4 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xee8cc | load | 1 | `(unknown)` | `-` |
| `str` at 0xee8dc | store | 4 | `sp -0x50` | `0xd0000000` |
| `str` at 0xee8e4 | store | 4 | `sp -0x54` | `0x50000000` |
| `str` at 0xee8f0 | store | 4 | `sp -0x50` | `0x8000000` |
| `str` at 0xee8f8 | store | 4 | `sp -0x54` | `0xf0000000` |
| `ldr` at 0xee8fc | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xee900 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xee908 | store | 4 | `[sp+4] +0x0` | `0x0` |
| `str` at 0xee910 | store | 4 | `[sp] +0x0` | `0x0` |
| `ldr` at 0xee914 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xee918 | store | 4 | `sp -0x38` | `[sp-84]` |
| `ldr` at 0xee920 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xee924 | store | 4 | `sp -0x78` | `[sp+8]` |
| `ldr` at 0xee928 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xee92c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xee930 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xee93c | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xee940 | store | 4 | `sp -0x78` | `[sp-92]` |
| `ldr` at 0xee944 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xee948 | store | 4 | `sp -0x74` | `[sp+8]` |
| `ldr` at 0xee94c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xee950 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xee954 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xee958 | store | 4 | `sp -0x6c` | `[sp+16]` |
| `str` at 0xee980 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0xee988 | load | 4 | `abs 0xeedfc` | `-` |
| `ldr` at 0xee98c | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xee998 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xee99c | store | 4 | `sp -0x78` | `[sp+12]` |
| `ldr` at 0xee9a0 | load | 4 | `abs 0xeee00` | `-` |
| `ldr` at 0xee9a4 | load | 4 | `abs 0xeee04` | `-` |
| `ldr` at 0xee9a8 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xee9ac | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xee9b4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xee9c0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xee9c4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xee9cc | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xee9d0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xee9d4 | load | 4 | `abs 0xeee08` | `-` |
| `ldr` at 0xee9e0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xee9e8 | load | 4 | `abs 0xeee08` | `-` |
| `str` at 0xee9f4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xee9f8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xee9fc | load | 4 | `abs 0xeee0c` | `-` |
| `ldr` at 0xeea08 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xeea10 | load | 4 | `abs 0xeee0c` | `-` |
| `str` at 0xeea1c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xeea20 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xeea24 | load | 4 | `abs 0xeee10` | `-` |
| `ldr` at 0xeea30 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xeea38 | load | 4 | `abs 0xeee10` | `-` |
| `str` at 0xeea44 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xeea48 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xeea58 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xeea6c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xeea70 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xeea80 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xeea94 | store | 4 | `sp -0x38` | `?` |
| `str` at 0xeeaa0 | store | 4 | `sp -0x78` | `sp-36` |
| `ldr` at 0xeeaa4 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xeeaa8 | store | 4 | `sp -0x74` | `[sp+8]` |
| `ldr` at 0xeeaac | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xeeab0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeeac0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xeeac4 | store | 4 | `sp -0x78` | `[sp-36]` |
| `ldr` at 0xeeac8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xeeacc | store | 4 | `sp -0x74` | `[sp+8]` |
| `ldr` at 0xeead0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xeead4 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xeead8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xeeadc | store | 4 | `sp -0x6c` | `[sp+16]` |
| `str` at 0xeeb04 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0xeeb0c | load | 4 | `abs 0xeedfc` | `-` |
| `ldr` at 0xeeb10 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xeeb1c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xeeb20 | store | 4 | `sp -0x78` | `[sp+12]` |
| `ldr` at 0xeeb24 | load | 4 | `abs 0xeee14` | `-` |
| `ldr` at 0xeeb28 | load | 4 | `abs 0xeee04` | `-` |
| `ldr` at 0xeeb2c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeeb30 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xeeb38 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xeeb48 | store | 4 | `sp -0x68` | `0x1` |
| `ldr` at 0xeeb50 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xeeb54 | store | 4 | `sp -0x38` | `[sp-80]` |
| `ldr` at 0xeeb5c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xeeb60 | store | 4 | `sp -0x78` | `[sp+8]` |
| `ldr` at 0xeeb64 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xeeb68 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xeeb6c | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xeeb78 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xeeb7c | store | 4 | `sp -0x78` | `[sp-92]` |
| `ldr` at 0xeeb80 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xeeb84 | store | 4 | `sp -0x74` | `[sp+8]` |
| `ldr` at 0xeeb88 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xeeb8c | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xeeb90 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xeeb94 | store | 4 | `sp -0x6c` | `[sp+16]` |
| `str` at 0xeebbc | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0xeebc4 | load | 4 | `abs 0xeedfc` | `-` |
| `ldr` at 0xeebc8 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xeebd4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xeebd8 | store | 4 | `sp -0x78` | `[sp+12]` |
| `ldr` at 0xeebdc | load | 4 | `abs 0xeee18` | `-` |
| `ldr` at 0xeebe0 | load | 4 | `abs 0xeee04` | `-` |
| `ldr` at 0xeebe4 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xeebe8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xeebf4 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0xeebf8 | load | 4 | `abs 0xeee1c` | `-` |
| `str` at 0xeec00 | store | 4 | `abs 0x2ad394` | `0x0` |
| `ldr` at 0xeec04 | load | 4 | `abs 0xeee1c` | `-` |
| `ldr` at 0xeec08 | load | 4 | `abs 0x2ad394` | `-` |
| `ldr` at 0xeec18 | load | 4 | `abs 0xeee1c` | `-` |
| `ldr` at 0xeec1c | load | 4 | `abs 0x2ad394` | `-` |
| `str` at 0xeec38 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xeec3c | load | 4 | `abs 0xeee1c` | `-` |
| `ldr` at 0xeec40 | load | 4 | `abs 0xeee1c` | `-` |
| `ldr` at 0xeec44 | load | 4 | `abs 0x2ad394` | `-` |
| `str` at 0xeec4c | store | 4 | `abs 0x2ad394` | `[0x2ad394]+1` |
| `str` at 0xeec5c | store | 4 | `sp -0x78` | `sp-36` |
| `ldr` at 0xeec60 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xeec64 | store | 4 | `sp -0x74` | `[sp+8]` |
| `ldr` at 0xeec68 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xeec6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeec78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeec84 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xeec90 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xeec94 | store | 4 | `sp -0x78` | `[sp-36]` |
| `ldr` at 0xeec98 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xeec9c | store | 4 | `sp -0x74` | `[sp+8]` |
| `ldr` at 0xeeca0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xeeca4 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xeeca8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xeecac | store | 4 | `sp -0x6c` | `[sp+16]` |
| `str` at 0xeecd4 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0xeecdc | load | 4 | `abs 0xeedfc` | `-` |
| `ldr` at 0xeece0 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xeecec | load | 4 | `sp +0xc` | `-` |
| `str` at 0xeecf0 | store | 4 | `sp -0x78` | `[sp+12]` |
| `ldr` at 0xeecf4 | load | 4 | `abs 0xeee20` | `-` |
| `ldr` at 0xeecf8 | load | 4 | `abs 0xeee04` | `-` |
| `ldr` at 0xeecfc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeed00 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xeed08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeed14 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xeed28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeed34 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xeed48 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xeed4c | load | 4 | `abs 0xeee24` | `-` |
| `str` at 0xeed58 | store | 4 | `sp -0x68` | `0x0` |
| `str` at 0xeed68 | store | 4 | `sp -0x78` | `0x42` |
| `ldr` at 0xeed6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeed70 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xeed94 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xeed98 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xeed9c | store | 4 | `[sp+4] +0x0` | `[sp-68]` |
| `ldr` at 0xeeda0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xeeda4 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xeeda8 | store | 4 | `[sp] +0x0` | `[sp-72]` |
| `str` at 0xeedb0 | store | 4 | `sp -0x4c` | `0x1` |
| `ldr` at 0xeedb4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xeedbc | store | 4 | `sp -0x1c` | `[sp-28]-1` |
| `ldr` at 0xeedc4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xeedd0 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xeedd4 | load | 4 | `abs 0xeee28` | `-` |
| `str` at 0xeede0 | store | 4 | `sp -0x68` | `0x0` |
| `str` at 0xeedec | store | 4 | `sp -0x68` | `0x1` |
| `ldr` at 0xeedf0 | load | 4 | `sp -0x68` | `-` |
| `ldmdb` at 0xeedf4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xeedf4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xeedf4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xee894 | `(unknown callee) 0xdc9a8` | `0x1` | `0x0` | `[sp+16]` | `?` |
| `bl` at 0xee938 | `(unknown callee) 0xeb3bc` | `[sp-92]` | `[sp-100]` | `[sp-96]` | `sp-56` |
| `bl` at 0xee96c | `(unknown callee) 0xdcf54` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0xee9b0 | `(unknown callee) 0xeb734` | `0x136dd8` | `0x136d20` | `[sp-92]` | `[sp+8]` |
| `bl` at 0xeeabc | `(unknown callee) 0xeb644` | `[sp-88]` | `[sp-24]` | `0x1e` | `sp-56` |
| `bl` at 0xeeaf0 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xeeb34 | `(unknown callee) 0xeb734` | `0x136de8` | `0x136d20` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xeeb74 | `(unknown callee) 0xeb3bc` | `[sp-92]` | `[sp-100]` | `[sp-96]` | `sp-56` |
| `bl` at 0xeeba8 | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xeebec | `(unknown callee) 0xeb734` | `0x136df8` | `0x136d20` | `[sp-92]` | `[sp+8]` |
| `bl` at 0xeec74 | `(unknown callee) 0xeb644` | `[sp-88]` | `[sp-24]` | `0x42` | `sp-56` |
| `bl` at 0xeecc0 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xeed04 | `(unknown callee) 0xeb734` | `0x136e08` | `0x136d20` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xeed18 | `(unknown callee) 0xe1cec` | `[sp-24]+64` | `[sp+12]` | `?` | `[sp-24]+64` |
| `bl` at 0xeed38 | `(unknown callee) 0xe1cec` | `[sp-24]+65` | `[sp+12]` | `?` | `[sp-24]+65` |
| `bl` at 0xeed50 | `(unknown callee) 0x1b508` | `[sp+16]` | `0x136d3c` | `?` | `ret(0xe1cec)` |
| `bl` at 0xeed7c | `(unknown callee) 0xe2174` | `[sp-24]` | `[sp+12]` | `0x0` | `sp-72` |
| `bl` at 0xeed90 | `(unknown callee) 0xe235c` | `0x0` | `0x40` | `sp-72` | `sp-72` |
| `bl` at 0xeedd8 | `(unknown callee) 0x1b508` | `[sp+16]` | `0x136e20` | `?` | `[sp-76]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xee8d4 | `? != 0x6` | `? == 0x6` |
| `bne` at 0xee978 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xee994 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xeeafc | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xeeb18 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xeeb40 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xeebb4 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xeebd0 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bls` at 0xeec10 | `[0x2ad394] <= 0x2 (unsigned, 32-bit)` | `[0x2ad394] > 0x2 (unsigned, 32-bit)` |
| `ble` at 0xeec80 | `[sp-28] <= 0x0 (signed, 32-bit)` | `[sp-28] > 0x0 (signed, 32-bit)` |
| `bne` at 0xeec8c | `[sp-76] != 0x0` | `[sp-76] == 0x0` |
| `bne` at 0xeeccc | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xeece8 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xeed24 | `ret(0xe1cec) == 0x0` | `ret(0xe1cec) != 0x0` |
| `beq` at 0xeed44 | `ret(0xe1cec) == 0x0` | `ret(0xe1cec) != 0x0` |
| `bne` at 0xeedcc | `[sp-76] != 0x0` | `[sp-76] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xeedf4 | `r0` | `?` |

## `PIOWrite_PIC`

- `PIOWrite_PIC`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeee2c to 0xef544.
  Established. The word store at `sp -0x38` saves `0x48000000` (`str` at 0xeef30). Calls to
  `0xeb644` include `[sp-64]`, `[sp-24]`, `0x40`, `sp-56` (`bl` at 0xef014). Later `0xe1cec`
  receives `[sp-24]+57`, `[sp+12]`, unknown, `[sp-24]+57` (`bl` at 0xef480); its zero result takes
  the branch and nonzero falls through (`beq` at 0xef48c). PIC register meaning and r0 at the joined
  exit remain unknown (`ldmdb` at 0xef510).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeee3c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xeee40 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xeee44 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xeee48 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xeee4c | load | 4 | `abs 0xef514` | `-` |
| `ldr` at 0xeee50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeee70 | load | 4 | `(unknown)` | `-` |
| `str` at 0xeee74 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xeee78 | load | 4 | `abs 0xef514` | `-` |
| `ldr` at 0xeee7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeee9c | load | 4 | `(unknown)` | `-` |
| `str` at 0xeeea0 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xeeea4 | load | 4 | `abs 0xef514` | `-` |
| `ldr` at 0xeeea8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeeec8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeeed0 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xeeed8 | load | 1 | `(unknown)` | `-` |
| `str` at 0xeeedc | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xeeee0 | load | 4 | `abs 0xef514` | `-` |
| `ldr` at 0xeeee4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeef04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeef0c | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xeef14 | load | 2 | `(unknown)` | `-` |
| `str` at 0xeef18 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xeef24 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xeef30 | store | 4 | `sp -0x38` | `0x48000000` |
| `ldr` at 0xeef38 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xeef3c | store | 4 | `sp -0x60` | `[sp+8]` |
| `ldr` at 0xeef40 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xeef44 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xeef48 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xeef54 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xeef58 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xeef5c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xeef60 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xeef64 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xeef68 | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0xeef6c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xeef70 | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0xeef98 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xeefa0 | load | 4 | `abs 0xef518` | `-` |
| `ldr` at 0xeefa4 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xeefb0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xeefb4 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xeefb8 | load | 4 | `abs 0xef51c` | `-` |
| `ldr` at 0xeefbc | load | 4 | `abs 0xef520` | `-` |
| `ldr` at 0xeefc0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xeefc4 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xeefcc | load | 4 | `sp +0x4` | `-` |
| `str` at 0xeefd0 | store | 4 | `sp -0x34` | `[sp+4]` |
| `ldr` at 0xeefd4 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xeefd8 | store | 4 | `sp -0x38` | `[sp]` |
| `str` at 0xeeff8 | store | 4 | `sp -0x60` | `sp-36` |
| `ldr` at 0xeeffc | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef000 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xef004 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xef008 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef018 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xef01c | store | 4 | `sp -0x60` | `[sp-36]` |
| `ldr` at 0xef020 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef024 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xef028 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef02c | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0xef030 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xef034 | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0xef05c | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xef064 | load | 4 | `abs 0xef518` | `-` |
| `ldr` at 0xef068 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xef074 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef078 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xef07c | load | 4 | `abs 0xef524` | `-` |
| `ldr` at 0xef080 | load | 4 | `abs 0xef520` | `-` |
| `ldr` at 0xef084 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xef088 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef094 | store | 4 | `sp -0x38` | `0x28000000` |
| `ldr` at 0xef09c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef0a0 | store | 4 | `sp -0x60` | `[sp+8]` |
| `ldr` at 0xef0a4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xef0a8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xef0ac | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xef0b8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xef0bc | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xef0c0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef0c4 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xef0c8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef0cc | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0xef0d0 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xef0d4 | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0xef0fc | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xef104 | load | 4 | `abs 0xef518` | `-` |
| `ldr` at 0xef108 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xef114 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef118 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xef11c | load | 4 | `abs 0xef528` | `-` |
| `ldr` at 0xef120 | load | 4 | `abs 0xef520` | `-` |
| `ldr` at 0xef124 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xef128 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef134 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xef138 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xef140 | store | 4 | `sp -0x38` | `0x1000000` |
| `ldr` at 0xef144 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xef148 | store | 4 | `sp -0x34` | `[sp-32]` |
| `str` at 0xef168 | store | 4 | `sp -0x60` | `sp-36` |
| `ldr` at 0xef16c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef170 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xef174 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xef178 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef188 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xef18c | store | 4 | `sp -0x60` | `[sp-36]` |
| `ldr` at 0xef190 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef194 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xef198 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef19c | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0xef1a0 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xef1a4 | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0xef1cc | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xef1d4 | load | 4 | `abs 0xef518` | `-` |
| `ldr` at 0xef1d8 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xef1e4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef1e8 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xef1ec | load | 4 | `abs 0xef52c` | `-` |
| `ldr` at 0xef1f0 | load | 4 | `abs 0xef520` | `-` |
| `ldr` at 0xef1f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xef1f8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef204 | store | 4 | `sp -0x38` | `0x8000000` |
| `ldr` at 0xef20c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef210 | store | 4 | `sp -0x60` | `[sp+8]` |
| `ldr` at 0xef214 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xef218 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xef21c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xef228 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xef22c | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xef230 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef234 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xef238 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef23c | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0xef240 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xef244 | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0xef26c | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xef274 | load | 4 | `abs 0xef518` | `-` |
| `ldr` at 0xef278 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xef284 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef288 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xef28c | load | 4 | `abs 0xef530` | `-` |
| `ldr` at 0xef290 | load | 4 | `abs 0xef520` | `-` |
| `ldr` at 0xef294 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xef298 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xef2a0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xef2b0 | store | 4 | `sp -0x50` | `0x1` |
| `str` at 0xef2bc | store | 4 | `sp -0x38` | `0x28000000` |
| `ldr` at 0xef2c4 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef2c8 | store | 4 | `sp -0x60` | `[sp+8]` |
| `ldr` at 0xef2cc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xef2d0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xef2d4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xef2e0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xef2e4 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xef2e8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef2ec | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xef2f0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef2f4 | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0xef2f8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xef2fc | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0xef324 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xef32c | load | 4 | `abs 0xef518` | `-` |
| `ldr` at 0xef330 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xef33c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef340 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xef344 | load | 4 | `abs 0xef528` | `-` |
| `ldr` at 0xef348 | load | 4 | `abs 0xef520` | `-` |
| `ldr` at 0xef34c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xef350 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef35c | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xef360 | load | 4 | `abs 0xef534` | `-` |
| `str` at 0xef368 | store | 4 | `abs 0x2ad394` | `0x0` |
| `ldr` at 0xef36c | load | 4 | `abs 0xef534` | `-` |
| `ldr` at 0xef370 | load | 4 | `abs 0x2ad394` | `-` |
| `ldr` at 0xef380 | load | 4 | `abs 0xef534` | `-` |
| `ldr` at 0xef384 | load | 4 | `abs 0x2ad394` | `-` |
| `str` at 0xef3a0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xef3a4 | load | 4 | `abs 0xef534` | `-` |
| `ldr` at 0xef3a8 | load | 4 | `abs 0xef534` | `-` |
| `ldr` at 0xef3ac | load | 4 | `abs 0x2ad394` | `-` |
| `str` at 0xef3b4 | store | 4 | `abs 0x2ad394` | `[0x2ad394]+1` |
| `str` at 0xef3c4 | store | 4 | `sp -0x60` | `sp-36` |
| `ldr` at 0xef3c8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef3cc | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xef3d0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xef3d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef3e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xef3ec | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xef3f8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xef3fc | store | 4 | `sp -0x60` | `[sp-36]` |
| `ldr` at 0xef400 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef404 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xef408 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef40c | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0xef410 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xef414 | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0xef43c | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xef444 | load | 4 | `abs 0xef518` | `-` |
| `ldr` at 0xef448 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xef454 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef458 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xef45c | load | 4 | `abs 0xef538` | `-` |
| `ldr` at 0xef460 | load | 4 | `abs 0xef520` | `-` |
| `ldr` at 0xef464 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xef468 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xef470 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef47c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xef490 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef49c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xef4b0 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xef4b4 | load | 4 | `abs 0xef53c` | `-` |
| `str` at 0xef4c0 | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0xef4cc | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0xef4d0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xef4d8 | store | 4 | `sp -0x1c` | `[sp-28]-1` |
| `ldr` at 0xef4e0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xef4ec | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xef4f0 | load | 4 | `abs 0xef540` | `-` |
| `str` at 0xef4fc | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0xef508 | store | 4 | `sp -0x50` | `0x1` |
| `ldr` at 0xef50c | load | 4 | `sp -0x50` | `-` |
| `ldmdb` at 0xef510 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xef510 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xef510 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeef28 | `(unknown callee) 0xdc9a8` | `0x1` | `0x0` | `[sp+16]` | `?` |
| `bl` at 0xeef50 | `(unknown callee) 0xeb3bc` | `[sp-68]` | `[sp-76]` | `[sp-72]` | `sp-56` |
| `bl` at 0xeef84 | `(unknown callee) 0xdcf54` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0xeefc8 | `(unknown callee) 0xeb734` | `0x136e58` | `0x136d20` | `[sp-68]` | `[sp+8]` |
| `bl` at 0xeefec | `(unknown callee) 0xe235c` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0xef014 | `(unknown callee) 0xeb644` | `[sp-64]` | `[sp-24]` | `0x40` | `sp-56` |
| `bl` at 0xef048 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xef08c | `(unknown callee) 0xeb734` | `0x136e6c` | `0x136d20` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xef0b4 | `(unknown callee) 0xeb3bc` | `[sp-68]` | `[sp-76]` | `[sp-72]` | `sp-56` |
| `bl` at 0xef0e8 | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xef12c | `(unknown callee) 0xeb734` | `0x136e80` | `0x136d20` | `[sp-68]` | `[sp+8]` |
| `bl` at 0xef15c | `(unknown callee) 0xe235c` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0xef184 | `(unknown callee) 0xeb644` | `[sp-64]` | `[sp-24]` | `0x40` | `sp-56` |
| `bl` at 0xef1b8 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xef1fc | `(unknown callee) 0xeb734` | `0x136e94` | `0x136d20` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xef224 | `(unknown callee) 0xeb3bc` | `[sp-68]` | `[sp-76]` | `[sp-72]` | `sp-56` |
| `bl` at 0xef258 | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xef29c | `(unknown callee) 0xeb734` | `0x136ea8` | `0x136d20` | `[sp-68]` | `[sp+8]` |
| `bl` at 0xef2dc | `(unknown callee) 0xeb3bc` | `[sp-68]` | `[sp-76]` | `[sp-72]` | `sp-56` |
| `bl` at 0xef310 | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xef354 | `(unknown callee) 0xeb734` | `0x136e80` | `0x136d20` | `[sp-68]` | `[sp+8]` |
| `bl` at 0xef3dc | `(unknown callee) 0xeb644` | `[sp-64]` | `[sp-24]` | `0x40` | `sp-56` |
| `bl` at 0xef428 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xef46c | `(unknown callee) 0xeb734` | `0x136ebc` | `0x136d20` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xef480 | `(unknown callee) 0xe1cec` | `[sp-24]+57` | `[sp+12]` | `?` | `[sp-24]+57` |
| `bl` at 0xef4a0 | `(unknown callee) 0xe1cec` | `[sp-24]+58` | `[sp+12]` | `?` | `[sp-24]+58` |
| `bl` at 0xef4b8 | `(unknown callee) 0x1b508` | `[sp+16]` | `0x136ed8` | `?` | `ret(0xe1cec)` |
| `bl` at 0xef4f4 | `(unknown callee) 0x1b508` | `[sp+16]` | `0x136f10` | `?` | `[sp-60]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xeef90 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xeefac | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xef054 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xef070 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xef0f4 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xef110 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xef1c4 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xef1e0 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xef264 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xef280 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xef2a8 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xef31c | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xef338 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bls` at 0xef378 | `[0x2ad394] <= 0x1 (unsigned, 32-bit)` | `[0x2ad394] > 0x1 (unsigned, 32-bit)` |
| `ble` at 0xef3e8 | `[sp-28] <= 0x0 (signed, 32-bit)` | `[sp-28] > 0x0 (signed, 32-bit)` |
| `bne` at 0xef3f4 | `[sp-60] != 0x0` | `[sp-60] == 0x0` |
| `bne` at 0xef434 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xef450 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xef48c | `ret(0xe1cec) == 0x0` | `ret(0xe1cec) != 0x0` |
| `beq` at 0xef4ac | `ret(0xe1cec) == 0x0` | `ret(0xe1cec) != 0x0` |
| `bne` at 0xef4e8 | `[sp-60] != 0x0` | `[sp-60] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xef510 | `r0` | `?` |

## `PIORead_PIC`

- `PIORead_PIC`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xef544 to 0xefca4.
  Established. Word stores first write `0x0` at `[sp+4] +0x0` and `[sp] +0x0` (`str` at 0xef650,
  `str` at 0xef658). Near the end those printed targets receive `[sp-68]` and `[sp-72]`,
  respectively (`str` at 0xefc18, `str` at 0xefc24). The intervening call to `0xe2174` carries
  `[sp-24]`, `[sp+12]`, `0x0`, `sp-72` (`bl` at 0xefbf8). The branch for `ret(0xe1cec) == 0x0` is
  taken, with nonzero falling through (`beq` at 0xefa6c). Reply-field roles and the common r0 result
  are unestablished (`ldmdb` at 0xefc70).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xef554 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xef558 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xef55c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xef560 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xef564 | load | 4 | `abs 0xefc74` | `-` |
| `ldr` at 0xef568 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef588 | load | 4 | `(unknown)` | `-` |
| `str` at 0xef58c | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xef590 | load | 4 | `abs 0xefc74` | `-` |
| `ldr` at 0xef594 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef5b4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xef5b8 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xef5bc | load | 4 | `abs 0xefc74` | `-` |
| `ldr` at 0xef5c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef5e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef5e8 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xef5f0 | load | 1 | `(unknown)` | `-` |
| `str` at 0xef5f4 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xef5f8 | load | 4 | `abs 0xefc74` | `-` |
| `ldr` at 0xef5fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef61c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef624 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xef62c | load | 2 | `(unknown)` | `-` |
| `str` at 0xef630 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0xef63c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xef644 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xef648 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xef650 | store | 4 | `[sp+4] +0x0` | `0x0` |
| `str` at 0xef658 | store | 4 | `[sp] +0x0` | `0x0` |
| `str` at 0xef660 | store | 4 | `sp -0x38` | `0x28000000` |
| `ldr` at 0xef668 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef66c | store | 4 | `sp -0x70` | `[sp+8]` |
| `ldr` at 0xef670 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xef674 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xef678 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xef684 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xef688 | store | 4 | `sp -0x70` | `[sp-84]` |
| `ldr` at 0xef68c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef690 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xef694 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef698 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xef69c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xef6a0 | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0xef6c8 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xef6d0 | load | 4 | `abs 0xefc78` | `-` |
| `ldr` at 0xef6d4 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xef6e0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef6e4 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xef6e8 | load | 4 | `abs 0xefc7c` | `-` |
| `ldr` at 0xef6ec | load | 4 | `abs 0xefc80` | `-` |
| `ldr` at 0xef6f0 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xef6f4 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef700 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xef704 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xef708 | store | 4 | `sp -0x34` | `[sp-32]` |
| `str` at 0xef728 | store | 4 | `sp -0x70` | `sp-36` |
| `ldr` at 0xef72c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef730 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xef734 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xef738 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef748 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xef74c | store | 4 | `sp -0x70` | `[sp-36]` |
| `ldr` at 0xef750 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef754 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xef758 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef75c | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xef760 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xef764 | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0xef78c | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xef794 | load | 4 | `abs 0xefc78` | `-` |
| `ldr` at 0xef798 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xef7a4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef7a8 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xef7ac | load | 4 | `abs 0xefc84` | `-` |
| `ldr` at 0xef7b0 | load | 4 | `abs 0xefc80` | `-` |
| `ldr` at 0xef7b4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xef7b8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef7c4 | store | 4 | `sp -0x38` | `0x8000000` |
| `ldr` at 0xef7cc | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef7d0 | store | 4 | `sp -0x70` | `[sp+8]` |
| `ldr` at 0xef7d4 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xef7d8 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xef7dc | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xef7e8 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xef7ec | store | 4 | `sp -0x70` | `[sp-84]` |
| `ldr` at 0xef7f0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef7f4 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xef7f8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef7fc | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xef800 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xef804 | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0xef82c | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xef834 | load | 4 | `abs 0xefc78` | `-` |
| `ldr` at 0xef838 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xef844 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef848 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xef84c | load | 4 | `abs 0xefc88` | `-` |
| `ldr` at 0xef850 | load | 4 | `abs 0xefc80` | `-` |
| `ldr` at 0xef854 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xef858 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xef860 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xef870 | store | 4 | `sp -0x60` | `0x1` |
| `str` at 0xef87c | store | 4 | `sp -0x38` | `0x28000000` |
| `ldr` at 0xef884 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef888 | store | 4 | `sp -0x70` | `[sp+8]` |
| `ldr` at 0xef88c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xef890 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xef894 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xef8a0 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xef8a4 | store | 4 | `sp -0x70` | `[sp-84]` |
| `ldr` at 0xef8a8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef8ac | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xef8b0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef8b4 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xef8b8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xef8bc | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0xef8e4 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xef8ec | load | 4 | `abs 0xefc78` | `-` |
| `ldr` at 0xef8f0 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xef8fc | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef900 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xef904 | load | 4 | `abs 0xefc7c` | `-` |
| `ldr` at 0xef908 | load | 4 | `abs 0xefc80` | `-` |
| `ldr` at 0xef90c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xef910 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef91c | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0xef920 | load | 4 | `abs 0xefc8c` | `-` |
| `str` at 0xef928 | store | 4 | `abs 0x2ad394` | `0x0` |
| `ldr` at 0xef92c | load | 4 | `abs 0xefc8c` | `-` |
| `ldr` at 0xef930 | load | 4 | `abs 0x2ad394` | `-` |
| `ldr` at 0xef940 | load | 4 | `abs 0xefc8c` | `-` |
| `ldr` at 0xef944 | load | 4 | `abs 0x2ad394` | `-` |
| `str` at 0xef960 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xef964 | load | 4 | `abs 0xefc8c` | `-` |
| `ldr` at 0xef968 | load | 4 | `abs 0xefc8c` | `-` |
| `ldr` at 0xef96c | load | 4 | `abs 0x2ad394` | `-` |
| `str` at 0xef974 | store | 4 | `abs 0x2ad394` | `[0x2ad394]+1` |
| `str` at 0xef984 | store | 4 | `sp -0x70` | `sp-36` |
| `ldr` at 0xef988 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef98c | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xef990 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xef994 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef9a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xef9ac | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xef9b8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xef9bc | store | 4 | `sp -0x70` | `[sp-36]` |
| `ldr` at 0xef9c0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef9c4 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xef9c8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef9cc | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xef9d0 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xef9d4 | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0xef9fc | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xefa04 | load | 4 | `abs 0xefc78` | `-` |
| `ldr` at 0xefa08 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xefa14 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xefa18 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xefa1c | load | 4 | `abs 0xefc90` | `-` |
| `ldr` at 0xefa20 | load | 4 | `abs 0xefc80` | `-` |
| `ldr` at 0xefa24 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xefa28 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xefa30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xefa3c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xefa50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xefa5c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xefa70 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xefa74 | load | 4 | `abs 0xefc94` | `-` |
| `str` at 0xefa80 | store | 4 | `sp -0x60` | `0x0` |
| `str` at 0xefa8c | store | 4 | `sp -0x38` | `0x48000000` |
| `ldr` at 0xefa94 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xefa98 | store | 4 | `sp -0x70` | `[sp+8]` |
| `ldr` at 0xefa9c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xefaa0 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xefaa4 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xefab0 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xefab4 | store | 4 | `sp -0x70` | `[sp-84]` |
| `ldr` at 0xefab8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xefabc | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xefac0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xefac4 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xefac8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xefacc | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0xefaf4 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xefafc | load | 4 | `abs 0xefc78` | `-` |
| `ldr` at 0xefb00 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xefb0c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xefb10 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xefb14 | load | 4 | `abs 0xefc98` | `-` |
| `ldr` at 0xefb18 | load | 4 | `abs 0xefc80` | `-` |
| `ldr` at 0xefb1c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xefb20 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xefb44 | store | 4 | `sp -0x70` | `sp-36` |
| `ldr` at 0xefb48 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xefb4c | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xefb50 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xefb54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xefb64 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xefb68 | store | 4 | `sp -0x70` | `[sp-36]` |
| `ldr` at 0xefb6c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xefb70 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xefb74 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xefb78 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xefb7c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xefb80 | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0xefba8 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xefbb0 | load | 4 | `abs 0xefc78` | `-` |
| `ldr` at 0xefbb4 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xefbc0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xefbc4 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xefbc8 | load | 4 | `abs 0xefc9c` | `-` |
| `ldr` at 0xefbcc | load | 4 | `abs 0xefc80` | `-` |
| `ldr` at 0xefbd0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xefbd4 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xefbe4 | store | 4 | `sp -0x70` | `0x40` |
| `ldr` at 0xefbe8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xefbec | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xefc10 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xefc14 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xefc18 | store | 4 | `[sp+4] +0x0` | `[sp-68]` |
| `ldr` at 0xefc1c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xefc20 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xefc24 | store | 4 | `[sp] +0x0` | `[sp-72]` |
| `str` at 0xefc2c | store | 4 | `sp -0x4c` | `0x1` |
| `ldr` at 0xefc30 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xefc38 | store | 4 | `sp -0x1c` | `[sp-28]-1` |
| `ldr` at 0xefc40 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xefc4c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xefc50 | load | 4 | `abs 0xefca0` | `-` |
| `str` at 0xefc5c | store | 4 | `sp -0x60` | `0x0` |
| `str` at 0xefc68 | store | 4 | `sp -0x60` | `0x1` |
| `ldr` at 0xefc6c | load | 4 | `sp -0x60` | `-` |
| `ldmdb` at 0xefc70 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xefc70 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xefc70 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xef640 | `(unknown callee) 0xdc9a8` | `0x1` | `0x0` | `[sp+16]` | `?` |
| `bl` at 0xef680 | `(unknown callee) 0xeb3bc` | `[sp-84]` | `[sp-92]` | `[sp-88]` | `sp-56` |
| `bl` at 0xef6b4 | `(unknown callee) 0xdcf54` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0xef6f8 | `(unknown callee) 0xeb734` | `0x136f4c` | `0x136d20` | `[sp-84]` | `[sp+8]` |
| `bl` at 0xef71c | `(unknown callee) 0xe235c` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0xef744 | `(unknown callee) 0xeb644` | `[sp-80]` | `[sp-24]` | `0x40` | `sp-56` |
| `bl` at 0xef778 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xef7bc | `(unknown callee) 0xeb734` | `0x136f60` | `0x136d20` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xef7e4 | `(unknown callee) 0xeb3bc` | `[sp-84]` | `[sp-92]` | `[sp-88]` | `sp-56` |
| `bl` at 0xef818 | `(unknown callee) 0xdcf54` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0xef85c | `(unknown callee) 0xeb734` | `0x136f74` | `0x136d20` | `[sp-84]` | `[sp+8]` |
| `bl` at 0xef89c | `(unknown callee) 0xeb3bc` | `[sp-84]` | `[sp-92]` | `[sp-88]` | `sp-56` |
| `bl` at 0xef8d0 | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xef914 | `(unknown callee) 0xeb734` | `0x136f4c` | `0x136d20` | `[sp-84]` | `[sp+8]` |
| `bl` at 0xef99c | `(unknown callee) 0xeb644` | `[sp-80]` | `[sp-24]` | `0x40` | `sp-56` |
| `bl` at 0xef9e8 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xefa2c | `(unknown callee) 0xeb734` | `0x136f88` | `0x136d20` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xefa40 | `(unknown callee) 0xe1cec` | `[sp-24]+57` | `[sp+12]` | `?` | `[sp-24]+57` |
| `bl` at 0xefa60 | `(unknown callee) 0xe1cec` | `[sp-24]+58` | `[sp+12]` | `?` | `[sp-24]+58` |
| `bl` at 0xefa78 | `(unknown callee) 0x1b508` | `[sp+16]` | `0x136ed8` | `?` | `ret(0xe1cec)` |
| `bl` at 0xefaac | `(unknown callee) 0xeb3bc` | `[sp-84]` | `[sp-92]` | `[sp-88]` | `sp-56` |
| `bl` at 0xefae0 | `(unknown callee) 0xdcf54` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0xefb24 | `(unknown callee) 0xeb734` | `0x136fa4` | `0x136d20` | `[sp-84]` | `[sp+8]` |
| `bl` at 0xefb38 | `(unknown callee) 0xe235c` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0xefb60 | `(unknown callee) 0xeb644` | `[sp-80]` | `[sp-24]` | `0x40` | `sp-56` |
| `bl` at 0xefb94 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xefbd8 | `(unknown callee) 0xeb734` | `0x136fb8` | `0x136d20` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xefbf8 | `(unknown callee) 0xe2174` | `[sp-24]` | `[sp+12]` | `0x0` | `sp-72` |
| `bl` at 0xefc0c | `(unknown callee) 0xe235c` | `0x0` | `0x40` | `sp-72` | `sp-72` |
| `bl` at 0xefc54 | `(unknown callee) 0x1b508` | `[sp+16]` | `0x136fcc` | `?` | `[sp-76]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xef6c0 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xef6dc | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xef784 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xef7a0 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xef824 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xef840 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xef868 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xef8dc | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xef8f8 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bls` at 0xef938 | `[0x2ad394] <= 0x1 (unsigned, 32-bit)` | `[0x2ad394] > 0x1 (unsigned, 32-bit)` |
| `ble` at 0xef9a8 | `[sp-28] <= 0x0 (signed, 32-bit)` | `[sp-28] > 0x0 (signed, 32-bit)` |
| `bne` at 0xef9b4 | `[sp-76] != 0x0` | `[sp-76] == 0x0` |
| `bne` at 0xef9f4 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xefa10 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xefa4c | `ret(0xe1cec) == 0x0` | `ret(0xe1cec) != 0x0` |
| `beq` at 0xefa6c | `ret(0xe1cec) == 0x0` | `ret(0xe1cec) != 0x0` |
| `bne` at 0xefaec | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xefb08 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xefba0 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bls` at 0xefbbc | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xefc48 | `[sp-76] != 0x0` | `[sp-76] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xefc70 | `r0` | `?` |

## `sn1sc_asm_drive_tap_fixed_tms`

- `sn1sc_asm_drive_tap_fixed_tms`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf7328 to 0xf74c8.
  Established. Six halfword store sites address `[0x2b37a0] +0x0`, each with unknown value (`strh`
  at 0xf73d4, `strh` at 0xf73f0, `strh` at 0xf7408, `strh` at 0xf7474, `strh` at 0xf7490, `strh` at
  0xf74a8). Halfword reads at the same printed target occur in each part (`ldrh` at 0xf73e8, `ldrh`
  at 0xf7488). The later signed branch takes its bypass for `0x1f < unknown` and falls through for
  greater or equal (`blt` at 0xf7458). No calls appear in the facts. Signal encoding, physical
  timing and r0 at return remain unknown (`pop` at 0xf74c0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xf7334 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0xf7338 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0xf7340 | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xf7344 | load | 4 | `r0 +0xc` | `-` |
| `ldr` at 0xf739c | load | 4 | `abs 0xf74c4` | `-` |
| `ldr` at 0xf73a0 | load | 4 | `abs 0x2b37a0` | `-` |
| `ldr` at 0xf73ac | load | 4 | `(unknown)` | `-` |
| `str` at 0xf73b0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xf73c8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf73d0 | store | 4 | `sp -0x28` | `?` |
| `strh` at 0xf73d4 | store | 2 | `[0x2b37a0] +0x0` | `?` |
| `ldr` at 0xf73dc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf73e4 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldrh` at 0xf73e8 | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `strh` at 0xf73f0 | store | 2 | `[0x2b37a0] +0x0` | `?` |
| `ldr` at 0xf73fc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf7404 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `strh` at 0xf7408 | store | 2 | `[0x2b37a0] +0x0` | `?` |
| `str` at 0xf7418 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf7430 | load | 4 | `abs 0xf74c4` | `-` |
| `ldr` at 0xf7434 | load | 4 | `abs 0x2b37a0` | `-` |
| `ldr` at 0xf7438 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf7448 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xf7468 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf7470 | store | 4 | `sp -0x28` | `?` |
| `strh` at 0xf7474 | store | 2 | `[0x2b37a0] +0x0` | `?` |
| `ldr` at 0xf747c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf7484 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldrh` at 0xf7488 | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `strh` at 0xf7490 | store | 2 | `[0x2b37a0] +0x0` | `?` |
| `ldr` at 0xf749c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf74a4 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `strh` at 0xf74a8 | store | 2 | `[0x2b37a0] +0x0` | `?` |
| `str` at 0xf74b8 | store | 4 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhs` at 0xf7390 | `0x0 >= ? (unsigned, 32-bit)` | `0x0 < ? (unsigned, 32-bit)` |
| `bge` at 0xf7414 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `blo` at 0xf7424 | `C == 0` | `not (C == 0)` |
| `beq` at 0xf742c | `Z == 1` | `not (Z == 1)` |
| `blt` at 0xf7458 | `0x1f < ? (signed, 32-bit)` | `0x1f >= ? (signed, 32-bit)` |
| `bge` at 0xf74b4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `pop` at 0xf74c0 | `r0` | `?` |

## `sn1sc_asm_drive_tck`

- `sn1sc_asm_drive_tck`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf74c8 to 0xf7548.
  Established. Two halfword stores have unknown values at `[0x2b37a0] +0x0` (`strh` at 0xf7524,
  `strh` at 0xf7530). The first conditional return is taken for unsigned `0x0 >= [r0+4]` and falls
  through for less (`pophs` at 0xf7518). That exit reports r0 `0x0`, whereas the later conditional
  return reports an unknown r0 and takes its exit for `C == 1`, falling through otherwise (`pophs`
  at 0xf7518, `pophs` at 0xf753c). Clock polarity and the relation between the two stores remain
  unproved by the resolved expressions.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf74c8 | store | 4 | `sp -0x4` | `lr` |
| `ldm` at 0xf74cc | load | 4 | `r0 +0x0` | `-` |
| `ldm` at 0xf74cc | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0xf751c | load | 4 | `abs 0xf7544` | `-` |
| `ldr` at 0xf7520 | load | 4 | `abs 0x2b37a0` | `-` |
| `strh` at 0xf7524 | store | 2 | `[0x2b37a0] +0x0` | `?` |
| `ldr` at 0xf752c | load | 4 | `abs 0x2b37a0` | `-` |
| `strh` at 0xf7530 | store | 2 | `[0x2b37a0] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `pophs` at 0xf7518 | `0x0 >= [r0+4] (unsigned, 32-bit)` | `0x0 < [r0+4] (unsigned, 32-bit)` |
| `pophs` at 0xf753c | `C == 1` | `not (C == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `pophs` at 0xf7518 | `r0` | `0x0` |
| `pophs` at 0xf753c | `r0` | `?` |

## `sn1sc_asm_drive_tap`

- `sn1sc_asm_drive_tap`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf7548 to 0xf7704.
  Established. Halfword writes include six sites at `[0x2b37a0] +0x0`, all with unknown value
  (`strh` at 0xf7600, `strh` at 0xf761c, `strh` at 0xf7634, `strh` at 0xf76b0, `strh` at 0xf76cc,
  `strh` at 0xf76e4). Two halfword reads address that printed target as well (`ldrh` at 0xf7614,
  `ldrh` at 0xf76c4). The word store at `[sp-48] +0x0` has unknown value (`str` at 0xf7648). The
  final signed bypass is taken for `0x1f < unknown` and falls through for greater or equal (`blt` at
  0xf7694). Scan bit meanings and the resulting r0 are not established (`pop` at 0xf76fc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xf7554 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0xf7558 | load | 4 | `r0 +0x4` | `-` |
| `str` at 0xf755c | store | 4 | `sp -0x2c` | `[r0+4]` |
| `ldr` at 0xf7564 | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xf7568 | load | 4 | `r0 +0xc` | `-` |
| `ldr` at 0xf756c | load | 4 | `r0 +0x10` | `-` |
| `ldr` at 0xf75c4 | load | 4 | `abs 0xf7700` | `-` |
| `ldr` at 0xf75c8 | load | 4 | `abs 0x2b37a0` | `-` |
| `str` at 0xf75cc | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xf75d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf75d8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf75dc | store | 4 | `sp -0x28` | `0x0` |
| `strh` at 0xf7600 | store | 2 | `[0x2b37a0] +0x0` | `?` |
| `ldr` at 0xf7608 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf7610 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldrh` at 0xf7614 | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `strh` at 0xf761c | store | 2 | `[0x2b37a0] +0x0` | `?` |
| `ldr` at 0xf7628 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf7630 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `strh` at 0xf7634 | store | 2 | `[0x2b37a0] +0x0` | `?` |
| `ldr` at 0xf7644 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xf7648 | store | 4 | `[sp-48] +0x0` | `?` |
| `ldr` at 0xf7650 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf765c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf7668 | load | 4 | `abs 0xf7700` | `-` |
| `ldr` at 0xf766c | load | 4 | `abs 0x2b37a0` | `-` |
| `ldr` at 0xf7670 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf7674 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf7684 | store | 4 | `sp -0x28` | `0x0` |
| `strh` at 0xf76b0 | store | 2 | `[0x2b37a0] +0x0` | `?` |
| `ldr` at 0xf76b8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf76c0 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldrh` at 0xf76c4 | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `strh` at 0xf76cc | store | 2 | `[0x2b37a0] +0x0` | `?` |
| `ldr` at 0xf76d8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf76e0 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `strh` at 0xf76e4 | store | 2 | `[0x2b37a0] +0x0` | `?` |
| `str` at 0xf76f4 | store | 4 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhs` at 0xf75b8 | `0x0 >= ? (unsigned, 32-bit)` | `0x0 < ? (unsigned, 32-bit)` |
| `bge` at 0xf7640 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `blo` at 0xf7658 | `C == 0` | `not (C == 0)` |
| `beq` at 0xf7664 | `Z == 1` | `not (Z == 1)` |
| `blt` at 0xf7694 | `0x1f < ? (signed, 32-bit)` | `0x1f >= ? (signed, 32-bit)` |
| `bge` at 0xf76f0 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `pop` at 0xf76fc | `r0` | `?` |
