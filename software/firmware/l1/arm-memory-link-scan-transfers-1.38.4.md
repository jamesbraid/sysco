# ARM memory, link and scan transfers in 1.38.4

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
`jmstr_move_to_idle` has an unresolved computed selector (`ldrls` at 0xdcc6c).
Accesses and return values cover only known control-flow edges. Undecodable byte ranges
remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/memory-link-scan-transfers-facts.txt) retain all tool sections.

## `LBIInit`

- `LBIInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x38474 to 0x38650.
  Established. The first call targets `0xd4a3c` with entry r0 through r3 as candidates (`bl` at
  0x3848c). A later branch takes `ret(0xf5668) == 0x0` and falls through on nonzero (`beq` at
  0x384bc). The fall-through word store writes `0xfffff060` at `sp -0x9c` (`str` at 0x384c4). The
  memory-interface initialisation contract remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x38484 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x38488 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x38490 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x384a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x384a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x384ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x384c0 | load | 4 | `abs 0x38640` | `-` |
| `str` at 0x384c4 | store | 4 | `sp -0x9c` | `0xfffff060` |
| `ldr` at 0x384cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x384e8 | load | 4 | `abs 0x38640` | `-` |
| `str` at 0x384ec | store | 4 | `sp -0x9c` | `0xfffff060` |
| `ldr` at 0x384f4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x38500 | store | 4 | `sp -0x98` | `ret(0xe9028)` |
| `ldr` at 0x38504 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x38510 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x38514 | store | 4 | `sp -0x9c` | `ret(0xe9028)` |
| `ldr` at 0x38544 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x38550 | load | 4 | `abs 0x38644` | `-` |
| `ldr` at 0x38558 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x38564 | store | 4 | `sp -0x9c` | `0x0` |
| `ldr` at 0x38570 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3857c | load | 4 | `abs 0x38648` | `-` |
| `ldr` at 0x38588 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x385a4 | load | 4 | `abs 0x38640` | `-` |
| `str` at 0x385a8 | store | 4 | `sp -0x9c` | `0xfffff060` |
| `ldr` at 0x385b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x385bc | load | 4 | `abs 0x3864c` | `-` |
| `str` at 0x385dc | store | 4 | `sp -0x9c` | `?` |
| `ldr` at 0x385e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x38604 | load | 4 | `abs 0x38640` | `-` |
| `str` at 0x38608 | store | 4 | `sp -0x9c` | `0xfffff060` |
| `ldr` at 0x38624 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x38634 | store | 4 | `sp -0x9c` | `ret(0x38650)` |
| `ldr` at 0x38638 | load | 4 | `sp -0x9c` | `-` |
| `ldmdb` at 0x3863c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3863c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3863c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3848c | `(unknown callee) 0xd4a3c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x384a4 | `(unknown callee) 0xd621c` | `?` | `[sp-24]` | `?` | `?` |
| `bl` at 0x384b0 | `(unknown callee) 0xf5668` | `[sp-20]` | `[sp-24]` | `?` | `?` |
| `bl` at 0x384d8 | `(unknown callee) 0xf5790` | `[sp-20]` | `0x0` | `0x0` | `ret(0xf5668)` |
| `bl` at 0x384f8 | `(unknown callee) 0xe9028` | `[sp-20]` | `?` | `?` | `ret(0xf5790)` |
| `bl` at 0x3851c | `(unknown callee) 0xe335c` | `ret(0xe9028)` | `?` | `?` | `ret(0xe9028)` |
| `bl` at 0x38530 | `(unknown callee) 0xf5830` | `0x0` | `sp-148` | `sp-144` | `sp-148` |
| `bl` at 0x38540 | `(unknown callee) 0xe33c8` | `ret(0xf5830)` | `?` | `?` | `ret(0xf5830)` |
| `bl` at 0x38548 | `(unknown callee) 0xea118` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x3855c | `(unknown callee) 0x2070c` | `0x107674` | `ret(0xea118)` | `[sp-148]` | `ret(0xea118)` |
| `bl` at 0x3856c | `(unknown callee) 0xe33c8` | `ret(0xf5830)` | `?` | `?` | `ret(0xf5830)` |
| `bl` at 0x38574 | `(unknown callee) 0xea118` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x38584 | `(unknown callee) 0x2070c` | `0x107698` | `ret(0xea118)` | `?` | `ret(0xea118)` |
| `bl` at 0x38594 | `(unknown callee) 0xf5790` | `[sp-20]` | `0x1` | `0x1` | `?` |
| `bl` at 0x385b0 | `(unknown callee) 0xe335c` | `ret(0xf5790)` | `?` | `?` | `ret(0xf5790)` |
| `bl` at 0x385c4 | `(unknown callee) 0xf5bb4` | `[sp-20]` | `0x1076bc` | `0x100` | `sp-144` |
| `bl` at 0x385d4 | `(unknown callee) 0xe33c8` | `ret(0xf5bb4)` | `?` | `?` | `ret(0xf5bb4)` |
| `bl` at 0x385e4 | `(unknown callee) 0xe33c8` | `ret(0xf5bb4)` | `?` | `?` | `ret(0xf5bb4)` |
| `bl` at 0x385f4 | `(unknown callee) 0xf5790` | `[sp-20]` | `0x0` | `0x0` | `?` |
| `bl` at 0x38620 | `(unknown callee) 0xdc9a8` | `0x0` | `0x1` | `sp-144` | `sp-144` |
| `bl` at 0x3862c | `(unknown callee) 0x38650` | `[sp-20]` | `0x2` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x384bc | `ret(0xf5668) == 0x0` | `ret(0xf5668) != 0x0` |
| `beq` at 0x384e4 | `ret(0xf5790) == 0x0` | `ret(0xf5790) != 0x0` |
| `beq` at 0x3850c | `ret(0xe9028) == 0x0` | `ret(0xe9028) != 0x0` |
| `bne` at 0x3853c | `ret(0xf5830) != 0x0` | `ret(0xf5830) == 0x0` |
| `beq` at 0x385a0 | `ret(0xf5790) == 0x0` | `ret(0xf5790) != 0x0` |
| `beq` at 0x385d0 | `ret(0xf5bb4) == 0x0` | `ret(0xf5bb4) != 0x0` |
| `beq` at 0x38600 | `ret(0xf5790) == 0x0` | `ret(0xf5790) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3863c | `r0` | `?` |

## `LBIMemInit`

- `LBIMemInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x38650 to 0x38940.
  Established. The opening call to `0xe335c` receives entry r0 through r3 as candidates (`bl` at
  0x38668). A later word store clears an unknown effective target (`str` at 0x3867c). The first
  result branch takes `ret(0xeac50) == 0x0` and falls through on nonzero (`beq` at 0x386c4). The
  memory setup encoding remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x38660 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x38664 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x3867c | store | 4 | `(unknown)` | `0x0` |
| `str` at 0x38690 | store | 4 | `(unknown)` | `0x0` |
| `str` at 0x386a0 | store | 4 | `sp -0x2c` | `0x39` |
| `ldr` at 0x386a4 | load | 4 | `abs 0x3891c` | `-` |
| `str` at 0x386a8 | store | 4 | `sp -0x28` | `0x1076d8` |
| `ldr` at 0x386ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x386cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x386d8 | load | 4 | `abs 0x38920` | `-` |
| `str` at 0x386dc | store | 4 | `sp -0x2c` | `0x1076e8` |
| `str` at 0x386e0 | store | 4 | `sp -0x28` | `ret(0xea118)` |
| `ldr` at 0x386e8 | load | 4 | `abs 0x38924` | `-` |
| `ldr` at 0x386f8 | load | 4 | `abs 0x38928` | `-` |
| `str` at 0x386fc | store | 4 | `sp -0x24` | `0xfffff05e` |
| `ldr` at 0x38710 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x38714 | store | 4 | `(unknown)` | `[sp-24]` |
| `ldr` at 0x38730 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x38734 | load | 4 | `abs 0x3892c` | `-` |
| `ldr` at 0x3874c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x38754 | load | 4 | `abs 0x3892c` | `-` |
| `str` at 0x38760 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x3877c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x38780 | load | 4 | `abs 0x38930` | `-` |
| `ldr` at 0x38798 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x387a0 | load | 4 | `abs 0x38930` | `-` |
| `str` at 0x387ac | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x387c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x387cc | load | 4 | `abs 0x38934` | `-` |
| `ldr` at 0x387e4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x387ec | load | 4 | `abs 0x38934` | `-` |
| `str` at 0x387f8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x38814 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x38830 | load | 4 | `(unknown)` | `-` |
| `str` at 0x38844 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x38860 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3887c | load | 4 | `(unknown)` | `-` |
| `str` at 0x38890 | store | 4 | `(unknown)` | `?` |
| `str` at 0x3889c | store | 4 | `sp -0x2c` | `0x2` |
| `ldr` at 0x388a0 | load | 4 | `abs 0x38938` | `-` |
| `str` at 0x388a4 | store | 4 | `sp -0x28` | `0x107728` |
| `ldr` at 0x388a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x388c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x388d4 | load | 4 | `abs 0x3893c` | `-` |
| `str` at 0x388d8 | store | 4 | `sp -0x2c` | `0x107738` |
| `str` at 0x388dc | store | 4 | `sp -0x28` | `ret(0xea118)` |
| `ldr` at 0x388e4 | load | 4 | `abs 0x38924` | `-` |
| `ldr` at 0x388f4 | load | 4 | `abs 0x38928` | `-` |
| `str` at 0x388f8 | store | 4 | `sp -0x24` | `0xfffff05e` |
| `str` at 0x38910 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x38914 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x38918 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x38918 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x38918 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x38668 | `(unknown callee) 0xe335c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x386b8 | `(unknown callee) 0xeac50` | `[sp-20]` | `0x16` | `sp-32` | `sp-32` |
| `bl` at 0x386c8 | `(unknown callee) 0xe33c8` | `ret(0xeac50)` | `?` | `?` | `ret(0xeac50)` |
| `bl` at 0x386d0 | `(unknown callee) 0xea118` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x386f4 | `(unknown callee) 0x21e28` | `0x3` | `0x107714` | `0xa3` | `0x0` |
| `bl` at 0x388b4 | `(unknown callee) 0xeac50` | `[sp-20]` | `0x10` | `sp-32` | `0x0` |
| `bl` at 0x388c4 | `(unknown callee) 0xe33c8` | `ret(0xeac50)` | `?` | `?` | `ret(0xeac50)` |
| `bl` at 0x388cc | `(unknown callee) 0xea118` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x388f0 | `(unknown callee) 0x21e28` | `0x3` | `0x107714` | `0xac` | `0x0` |
| `bl` at 0x38904 | `(unknown callee) 0x3a37c` | `0x12c` | `?` | `?` | `ret(0xeac50)` |
| `bl` at 0x38908 | `(unknown callee) 0xe33c8` | `ret(0x3a37c)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x386c4 | `ret(0xeac50) == 0x0` | `ret(0xeac50) != 0x0` |
| `beq` at 0x388c0 | `ret(0xeac50) == 0x0` | `ret(0xeac50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x38918 | `r0` | `?` |

## `LBIMemRead`

- `LBIMemRead`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x38940 to 0x38dc4.
  Established. The opening decision has an unknown operand (`beq` at 0x3896c). The call to `0xe335c`
  carries entry r0, entry r1, entry r2 and unknown (`bl` at 0x3897c). The common exit has an unknown
  r0 expression (`ldmdb` at 0x38d94). Read geometry and the external memory operation remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x38950 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x38954 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x38958 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x3895c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x38960 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x38970 | load | 4 | `abs 0x38d98` | `-` |
| `str` at 0x38974 | store | 4 | `sp -0x30` | `0xfffff05d` |
| `ldr` at 0x38980 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x38988 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x3898c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38990 | load | 4 | `abs 0x38d9c` | `-` |
| `ldr` at 0x3899c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x389a4 | load | 4 | `abs 0x38d9c` | `-` |
| `str` at 0x389b0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x389b4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x389b8 | load | 4 | `abs 0x38da0` | `-` |
| `ldr` at 0x389c4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x389cc | load | 4 | `abs 0x38da0` | `-` |
| `str` at 0x389d8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x389dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x389e0 | load | 4 | `abs 0x38da4` | `-` |
| `ldr` at 0x389ec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x389f4 | load | 4 | `abs 0x38da4` | `-` |
| `str` at 0x38a00 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x38a04 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38a14 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x38a28 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x38a2c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38a3c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x38a50 | store | 4 | `sp -0x24` | `?` |
| `str` at 0x38a5c | store | 4 | `sp -0x3c` | `0x18` |
| `ldr` at 0x38a60 | load | 4 | `abs 0x38da8` | `-` |
| `str` at 0x38a64 | store | 4 | `sp -0x38` | `0x107764` |
| `ldr` at 0x38a68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x38a88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x38a94 | load | 4 | `abs 0x38dac` | `-` |
| `str` at 0x38a98 | store | 4 | `sp -0x3c` | `0x107774` |
| `str` at 0x38a9c | store | 4 | `sp -0x38` | `ret(0xea118)` |
| `ldr` at 0x38aa4 | load | 4 | `abs 0x38db0` | `-` |
| `ldr` at 0x38ab4 | load | 4 | `abs 0x38db4` | `-` |
| `str` at 0x38ab8 | store | 4 | `sp -0x30` | `0xfffff05c` |
| `ldr` at 0x38ac0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x38ad0 | store | 4 | `sp -0x34` | `sp-36` |
| `ldr` at 0x38ad8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x38adc | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0x38ae0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x38ae4 | store | 4 | `sp -0x28` | `[sp-52]` |
| `ldr` at 0x38ae8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x38af0 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x38af4 | load | 4 | `abs 0x38db8` | `-` |
| `str` at 0x38af8 | store | 4 | `sp -0x38` | `0x1077a0` |
| `ldr` at 0x38afc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x38b08 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x38b20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x38b2c | load | 4 | `abs 0x38dbc` | `-` |
| `str` at 0x38b30 | store | 4 | `sp -0x3c` | `0x1077b0` |
| `str` at 0x38b34 | store | 4 | `sp -0x38` | `ret(0xea118)` |
| `ldr` at 0x38b3c | load | 4 | `abs 0x38db0` | `-` |
| `ldr` at 0x38b4c | load | 4 | `abs 0x38dc0` | `-` |
| `str` at 0x38b50 | store | 4 | `sp -0x30` | `0xfffff05b` |
| `str` at 0x38b60 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x38b64 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x38b78 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x38b88 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x38b8c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x38b90 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0x38b94 | load | 4 | `abs 0x38d9c` | `-` |
| `ldr` at 0x38ba0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x38ba4 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0x38bac | load | 4 | `abs 0x38d9c` | `-` |
| `str` at 0x38bb8 | store | 4 | `[sp-40] +0x0` | `?` |
| `ldr` at 0x38bbc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x38bc0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x38bc4 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0x38bc8 | load | 4 | `abs 0x38da0` | `-` |
| `ldr` at 0x38bd4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x38bd8 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0x38be0 | load | 4 | `abs 0x38da0` | `-` |
| `str` at 0x38bec | store | 4 | `[sp-40] +0x0` | `?` |
| `ldr` at 0x38bf0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x38bf4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x38bf8 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0x38bfc | load | 4 | `abs 0x38da4` | `-` |
| `ldr` at 0x38c08 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x38c0c | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0x38c14 | load | 4 | `abs 0x38da4` | `-` |
| `str` at 0x38c20 | store | 4 | `[sp-40] +0x0` | `?` |
| `ldr` at 0x38c24 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x38c28 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x38c2c | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0x38c3c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x38c40 | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0x38c54 | store | 4 | `[sp-40] +0x0` | `?` |
| `ldr` at 0x38c58 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x38c5c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x38c60 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0x38c70 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x38c74 | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0x38c88 | store | 4 | `[sp-40] +0x0` | `?` |
| `ldr` at 0x38c8c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x38c94 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x38c98 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x38ca0 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0x38ca8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x38cb4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38cb8 | load | 4 | `abs 0x38d9c` | `-` |
| `ldr` at 0x38cc4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38ccc | load | 4 | `abs 0x38d9c` | `-` |
| `str` at 0x38cd8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x38cdc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38ce0 | load | 4 | `abs 0x38da0` | `-` |
| `ldr` at 0x38cec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38cf4 | load | 4 | `abs 0x38da0` | `-` |
| `str` at 0x38d00 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x38d04 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38d08 | load | 4 | `abs 0x38da4` | `-` |
| `ldr` at 0x38d14 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38d1c | load | 4 | `abs 0x38da4` | `-` |
| `str` at 0x38d28 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x38d2c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38d3c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x38d50 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x38d54 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38d64 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x38d78 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x38d7c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x38d80 | load | 2 | `sp -0x24` | `-` |
| `strh` at 0x38d84 | store | 2 | `[sp-28] +0x0` | `([sp-36]:2 & 0xffff)` |
| `str` at 0x38d8c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x38d90 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0x38d94 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x38d94 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x38d94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3897c | `(unknown callee) 0xe335c` | `r0` | `r1` | `r2` | `?` |
| `bl` at 0x38a74 | `(unknown callee) 0xeac50` | `[sp-20]` | `0x11` | `sp-36` | `0x0` |
| `bl` at 0x38a84 | `(unknown callee) 0xe33c8` | `ret(0xeac50)` | `?` | `?` | `ret(0xeac50)` |
| `bl` at 0x38a8c | `(unknown callee) 0xea118` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x38ab0 | `(unknown callee) 0x21e28` | `0x3` | `0x107714` | `0xc9` | `0x0` |
| `bl` at 0x38b0c | `(unknown callee) 0xeac50` | `[sp-20]` | `0x14` | `0x0` | `[sp-52]` |
| `bl` at 0x38b1c | `(unknown callee) 0xe33c8` | `ret(0xeac50)` | `?` | `?` | `ret(0xeac50)` |
| `bl` at 0x38b24 | `(unknown callee) 0xea118` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x38b48 | `(unknown callee) 0x21e28` | `0x3` | `0x107714` | `0xd4` | `0x0` |
| `bl` at 0x38b58 | `(unknown callee) 0xe33c8` | `ret(0xeac50)` | `?` | `?` | `ret(0xeac50)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3896c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x38a80 | `ret(0xeac50) == 0x0` | `ret(0xeac50) != 0x0` |
| `bgt` at 0x38ac8 | `[sp-32] > 0x3 (signed, 32-bit)` | `[sp-32] <= 0x3 (signed, 32-bit)` |
| `beq` at 0x38b18 | `ret(0xeac50) == 0x0` | `ret(0xeac50) != 0x0` |
| `blt` at 0x38b80 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bgt` at 0x38cb0 | `[sp-32] > 0x3 (signed, 32-bit)` | `[sp-32] <= 0x3 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x38d94 | `r0` | `?` |

## `LBIMemWrite`

- `LBIMemWrite`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x38dc4 to 0x392f0.
  Established. The first branch retains an unknown operand (`beq` at 0x38df0). The call to `0xe335c`
  has four unknown candidates (`bl` at 0x390fc). The recorded final r0 expression is unknown
  (`ldmdb` at 0x392c0). The memory write protocol remains unproved beyond the explicit calls and
  accesses.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x38dd4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x38dd8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x38ddc | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x38de0 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x38de4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x38df4 | load | 4 | `abs 0x392c4` | `-` |
| `str` at 0x38df8 | store | 4 | `sp -0x34` | `0xfffff05d` |
| `ldr` at 0x38e00 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x38e10 | store | 4 | `sp -0x38` | `sp-44` |
| `ldr` at 0x38e18 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x38e1c | store | 4 | `sp -0x38` | `r2` |
| `ldr` at 0x38e20 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x38e24 | store | 4 | `sp -0x24` | `[sp-56]` |
| `str` at 0x38e2c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x38e30 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x38e44 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x38e54 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x38e5c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38e64 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x38e6c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38e74 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x38e78 | load | 4 | `abs 0x392c8` | `-` |
| `ldr` at 0x38e84 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x38e8c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38e94 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x38e9c | load | 4 | `abs 0x392c8` | `-` |
| `str` at 0x38ea8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x38eac | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x38eb4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38ebc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x38ec4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38ecc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x38ed0 | load | 4 | `abs 0x392cc` | `-` |
| `ldr` at 0x38edc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x38ee4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38eec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x38ef4 | load | 4 | `abs 0x392cc` | `-` |
| `str` at 0x38f00 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x38f04 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x38f0c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38f14 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x38f1c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38f24 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x38f28 | load | 4 | `abs 0x392d0` | `-` |
| `ldr` at 0x38f34 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x38f3c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38f44 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x38f4c | load | 4 | `abs 0x392d0` | `-` |
| `str` at 0x38f58 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x38f5c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x38f64 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38f6c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x38f74 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38f7c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x38f8c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x38f94 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38f9c | load | 4 | `(unknown)` | `-` |
| `str` at 0x38fb0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x38fb4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x38fbc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38fc4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x38fcc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38fd4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x38fe4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x38fec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38ff4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x39008 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x3900c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x39014 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x3901c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x39028 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x3902c | load | 2 | `(unknown)` | `-` |
| `strh` at 0x39030 | store | 2 | `sp -0x2c` | `?` |
| `ldr` at 0x39034 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x39038 | load | 4 | `abs 0x392c8` | `-` |
| `ldr` at 0x39044 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3904c | load | 4 | `abs 0x392c8` | `-` |
| `str` at 0x39058 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x3905c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x39060 | load | 4 | `abs 0x392cc` | `-` |
| `ldr` at 0x3906c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x39074 | load | 4 | `abs 0x392cc` | `-` |
| `str` at 0x39080 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x39084 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x39088 | load | 4 | `abs 0x392d0` | `-` |
| `ldr` at 0x39094 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3909c | load | 4 | `abs 0x392d0` | `-` |
| `str` at 0x390a8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x390ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x390bc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x390d0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x390d4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x390e4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x390f8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x39100 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x39108 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x3910c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39110 | load | 4 | `abs 0x392c8` | `-` |
| `ldr` at 0x3911c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39124 | load | 4 | `abs 0x392c8` | `-` |
| `str` at 0x39130 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x39134 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39138 | load | 4 | `abs 0x392cc` | `-` |
| `ldr` at 0x39144 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3914c | load | 4 | `abs 0x392cc` | `-` |
| `str` at 0x39158 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x3915c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39160 | load | 4 | `abs 0x392d0` | `-` |
| `ldr` at 0x3916c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39174 | load | 4 | `abs 0x392d0` | `-` |
| `str` at 0x39180 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x39184 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39194 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x391a8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x391ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x391bc | load | 4 | `sp -0x28` | `-` |
| `str` at 0x391d0 | store | 4 | `sp -0x28` | `?` |
| `str` at 0x391dc | store | 4 | `sp -0x40` | `0x18` |
| `ldr` at 0x391e0 | load | 4 | `abs 0x392d4` | `-` |
| `str` at 0x391e4 | store | 4 | `sp -0x3c` | `0x107764` |
| `ldr` at 0x391e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x39208 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x39214 | load | 4 | `abs 0x392d8` | `-` |
| `str` at 0x39218 | store | 4 | `sp -0x40` | `0x107774` |
| `str` at 0x3921c | store | 4 | `sp -0x3c` | `ret(0xea118)` |
| `ldr` at 0x39224 | load | 4 | `abs 0x392dc` | `-` |
| `ldr` at 0x39234 | load | 4 | `abs 0x392e0` | `-` |
| `str` at 0x39238 | store | 4 | `sp -0x34` | `0xfffff05c` |
| `ldr` at 0x39240 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x39248 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x3924c | load | 4 | `abs 0x392e4` | `-` |
| `str` at 0x39250 | store | 4 | `sp -0x3c` | `0x1077e0` |
| `ldr` at 0x39254 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3925c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39260 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39278 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x39284 | load | 4 | `abs 0x392e8` | `-` |
| `str` at 0x39288 | store | 4 | `sp -0x40` | `0x1077f0` |
| `str` at 0x3928c | store | 4 | `sp -0x3c` | `ret(0xea118)` |
| `ldr` at 0x39294 | load | 4 | `abs 0x392dc` | `-` |
| `ldr` at 0x392a4 | load | 4 | `abs 0x392ec` | `-` |
| `str` at 0x392a8 | store | 4 | `sp -0x34` | `0xfffff05a` |
| `str` at 0x392b8 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x392bc | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x392c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x392c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x392c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x390fc | `(unknown callee) 0xe335c` | `?` | `?` | `?` | `?` |
| `bl` at 0x391f4 | `(unknown callee) 0xeac50` | `[sp-20]` | `0x11` | `sp-40` | `0x0` |
| `bl` at 0x39204 | `(unknown callee) 0xe33c8` | `ret(0xeac50)` | `?` | `?` | `ret(0xeac50)` |
| `bl` at 0x3920c | `(unknown callee) 0xea118` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x39230 | `(unknown callee) 0x21e28` | `0x3` | `0x107714` | `0x10c` | `0x0` |
| `bl` at 0x39264 | `(unknown callee) 0xeac50` | `[sp-20]` | `0x13` | `[sp-36]` | `[sp-36]` |
| `bl` at 0x39274 | `(unknown callee) 0xe33c8` | `ret(0xeac50)` | `?` | `?` | `ret(0xeac50)` |
| `bl` at 0x3927c | `(unknown callee) 0xea118` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x392a0 | `(unknown callee) 0x21e28` | `0x3` | `0x107714` | `0x118` | `0x0` |
| `bl` at 0x392b0 | `(unknown callee) 0xe33c8` | `ret(0xeac50)` | `?` | `?` | `ret(0xeac50)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x38df0 | `? == 0x0` | `? != 0x0` |
| `bgt` at 0x38e08 | `r3 > 0x3 (signed, 32-bit)` | `r3 <= 0x3 (signed, 32-bit)` |
| `blt` at 0x38e4c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bgt` at 0x39024 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x39200 | `ret(0xeac50) == 0x0` | `ret(0xeac50) != 0x0` |
| `beq` at 0x39270 | `ret(0xeac50) == 0x0` | `ret(0xeac50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x392c0 | `r0` | `?` |

## `LBIMemReadToFile`

- `LBIMemReadToFile`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x392f0 to 0x39494.
  Established. The opening call to `0x1b370` carries entry r3, `0x242`, `0x1a4` and `0x1000` as
  candidates (`bl` at 0x3932c); its signed nonnegative result takes the branch and negative values
  fall through (`bge` at 0x39340). The final r0 expression is unknown (`ldmdb` at 0x39488).
  File-transfer error meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x39300 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x39304 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x39308 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x3930c | store | 4 | `sp -0x20` | `r3` |
| `str` at 0x39314 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0x3931c | store | 4 | `sp -0x34` | `0x1000` |
| `ldr` at 0x39320 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x39324 | load | 4 | `abs 0x3948c` | `-` |
| `str` at 0x39334 | store | 4 | `sp -0x28` | `ret(0x1b370)` |
| `ldr` at 0x39338 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39344 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x39348 | store | 4 | `sp -0x3c` | `ret(0x1b370)` |
| `ldr` at 0x39350 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x3935c | store | 4 | `sp -0x38` | `ret(0x1b124)` |
| `ldr` at 0x39360 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3936c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x39378 | store | 4 | `sp -0x3c` | `0xc` |
| `str` at 0x39384 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x39388 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3938c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3939c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x393a0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x393a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x393b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x393b8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x393c0 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x393c8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x393cc | store | 4 | `sp -0x40` | `[sp-52]` |
| `ldr` at 0x393d0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x393d4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x393d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x393dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x393e0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x393e4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x393f0 | store | 4 | `sp -0x2c` | `ret(0x38940)` |
| `ldr` at 0x393f4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x39404 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39408 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3940c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x39418 | store | 4 | `sp -0x2c` | `ret(0x1afc8)` |
| `ldr` at 0x3941c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x39420 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3942c | load | 4 | `abs 0x39490` | `-` |
| `ldr` at 0x39430 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x39434 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x39444 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x39448 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3944c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x39454 | store | 4 | `sp -0x24` | `([sp-36] + [sp-52])` |
| `ldr` at 0x39458 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3945c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x39464 | store | 4 | `sp -0x18` | `([sp-24] + [sp-52])` |
| `ldr` at 0x3946c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x39474 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3947c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x39480 | store | 4 | `sp -0x3c` | `[sp-44]` |
| `ldr` at 0x39484 | load | 4 | `sp -0x3c` | `-` |
| `ldmdb` at 0x39488 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x39488 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x39488 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3932c | `(unknown callee) 0x1b370` | `r3` | `0x242` | `0x1a4` | `0x1000` |
| `bl` at 0x39354 | `(unknown callee) 0x1b124` | `[sp-52]` | `?` | `?` | `ret(0x1b370)` |
| `bl` at 0x39370 | `(unknown callee) 0x1b004` | `[sp-40]` | `?` | `?` | `ret(0x1b124)` |
| `bl` at 0x393e8 | `(unknown callee) 0x38940` | `[sp-20]` | `[sp-24]` | `?` | `?` |
| `bl` at 0x39410 | `(unknown callee) 0x1afc8` | `[sp-40]` | `[sp-56]` | `[sp-48]` | `ret(0x38940)` |
| `bl` at 0x39438 | `(unknown callee) 0x23f2c` | `0x107820` | `[sp-48]` | `ret(0x1afc8)` | `[sp-48]` |
| `bl` at 0x39470 | `(unknown callee) 0x1b400` | `?` | `?` | `?` | `?` |
| `bl` at 0x39478 | `(unknown callee) 0x1b004` | `[sp-40]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x39340 | `ret(0x1b370) >= 0x0 (signed, 32-bit)` | `ret(0x1b370) < 0x0 (signed, 32-bit)` |
| `bne` at 0x39368 | `ret(0x1b124) != 0x0` | `ret(0x1b124) == 0x0` |
| `blt` at 0x39394 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x393b0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x393fc | `ret(0x38940) == 0x0` | `ret(0x38940) != 0x0` |
| `beq` at 0x39428 | `ret(0x1afc8) == [sp-48]` | `ret(0x1afc8) != [sp-48]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x39488 | `r0` | `?` |

## `LBIMemWriteFromFile`

- `LBIMemWriteFromFile`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x39494 to 0x39684.
  Established. The call to `0x1b370` receives entry r3, `0x0`, entry r2 and `0x1000` as candidates
  (`bl` at 0x394cc). Nonnegative results take the signed branch, while negative results fall through
  (`bge` at 0x394e0). The recorded exit value and memory-transfer contract remain unknown (`ldmdb`
  at 0x3967c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x394a4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x394a8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x394ac | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x394b0 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0x394b8 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0x394c0 | store | 4 | `sp -0x34` | `0x1000` |
| `ldr` at 0x394c4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x394d4 | store | 4 | `sp -0x28` | `ret(0x1b370)` |
| `ldr` at 0x394d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x394e4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x394e8 | store | 4 | `sp -0x94` | `ret(0x1b370)` |
| `ldr` at 0x394f0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x394fc | store | 4 | `sp -0x38` | `ret(0x1b124)` |
| `ldr` at 0x39500 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3950c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x39518 | store | 4 | `sp -0x94` | `0xc` |
| `ldr` at 0x39520 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x39530 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x39540 | store | 4 | `sp -0x2c` | `ret(0xfbbd4)` |
| `ldr` at 0x39544 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x39558 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `str` at 0x3955c | store | 4 | `sp -0x2c` | `[ret(0x1b0b8)]` |
| `ldr` at 0x39564 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x39568 | store | 4 | `sp -0x1c` | `[sp-100]` |
| `str` at 0x39570 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x39574 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39578 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x39588 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3958c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x39594 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x395a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x395a4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x395ac | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x395b4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x395b8 | store | 4 | `sp -0x98` | `[sp-52]` |
| `ldr` at 0x395bc | load | 4 | `sp -0x98` | `-` |
| `str` at 0x395c0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x395c4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x395c8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x395cc | load | 4 | `sp -0x30` | `-` |
| `str` at 0x395d8 | store | 4 | `sp -0x2c` | `ret(0x1b4fc)` |
| `ldr` at 0x395dc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x395e0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x395ec | load | 4 | `abs 0x39680` | `-` |
| `ldr` at 0x395f0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x395f4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x39604 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x39608 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x39614 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x39618 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3961c | load | 4 | `sp -0x38` | `-` |
| `str` at 0x39628 | store | 4 | `sp -0x2c` | `ret(0x38dc4)` |
| `ldr` at 0x3962c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3963c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39640 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x39648 | store | 4 | `sp -0x24` | `([sp-36] + [sp-52])` |
| `ldr` at 0x3964c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39650 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x39658 | store | 4 | `sp -0x18` | `([sp-24] + [sp-52])` |
| `ldr` at 0x39660 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x39668 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39670 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x39674 | store | 4 | `sp -0x94` | `[sp-44]` |
| `ldr` at 0x39678 | load | 4 | `sp -0x94` | `-` |
| `ldmdb` at 0x3967c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3967c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3967c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x394cc | `(unknown callee) 0x1b370` | `r3` | `0x0` | `r2` | `0x1000` |
| `bl` at 0x394f4 | `(unknown callee) 0x1b124` | `[sp-52]` | `?` | `?` | `ret(0x1b370)` |
| `bl` at 0x39510 | `(unknown callee) 0x1b004` | `[sp-40]` | `?` | `?` | `ret(0x1b124)` |
| `bl` at 0x39538 | `(unknown callee) 0xfbbd4` | `[sp-40]` | `sp-144` | `?` | `sp-144` |
| `bl` at 0x39550 | `(unknown callee) 0x1b0b8` | `ret(0xfbbd4)` | `?` | `?` | `ret(0xfbbd4)` |
| `bl` at 0x395d0 | `(unknown callee) 0x1b4fc` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x395f8 | `(unknown callee) 0x23f2c` | `0x10783c` | `[sp-48]` | `ret(0x1b4fc)` | `[sp-48]` |
| `bl` at 0x39620 | `(unknown callee) 0x38dc4` | `[sp-20]` | `[sp-24]` | `[sp-56]` | `?` |
| `bl` at 0x39664 | `(unknown callee) 0x1b400` | `?` | `?` | `?` | `?` |
| `bl` at 0x3966c | `(unknown callee) 0x1b004` | `[sp-40]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x394e0 | `ret(0x1b370) >= 0x0 (signed, 32-bit)` | `ret(0x1b370) < 0x0 (signed, 32-bit)` |
| `bne` at 0x39508 | `ret(0x1b124) != 0x0` | `ret(0x1b124) == 0x0` |
| `bne` at 0x39528 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bge` at 0x3954c | `ret(0xfbbd4) >= 0x0 (signed, 32-bit)` | `ret(0xfbbd4) < 0x0 (signed, 32-bit)` |
| `blt` at 0x39580 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x3959c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x395e8 | `ret(0x1b4fc) == [sp-48]` | `ret(0x1b4fc) != [sp-48]` |
| `beq` at 0x39634 | `ret(0x38dc4) == 0x0` | `ret(0x38dc4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3967c | `r0` | `?` |

## `LBILoadPROM`

- `LBILoadPROM`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x39684 to 0x3987c.
  Established. The opening call targets `0xfbbb8` with entry r1, `sp-120`, entry r2 and `sp-120` as
  candidates (`bl` at 0x396b8); its signed nonnegative result takes the branch and negative results
  fall through (`bge` at 0x396c4). The final r0 expression is unknown (`ldmdb` at 0x39868).
  PROM-loading address semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x39694 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x39698 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x3969c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x396a0 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0x396a8 | store | 4 | `sp -0x80` | `0xff000000` |
| `ldr` at 0x396b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x396c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x396dc | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0x396e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x396ec | load | 4 | `abs 0x3986c` | `-` |
| `ldr` at 0x39700 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `str` at 0x39704 | store | 4 | `sp -0xfc` | `[ret(0x1b0b8)]` |
| `str` at 0x39710 | store | 4 | `sp -0x88` | `0x2000000` |
| `ldr` at 0x39714 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x39718 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x39720 | store | 4 | `sp -0x84` | `?` |
| `ldr` at 0x39724 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x39728 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x39734 | store | 4 | `sp -0x8c` | `([sp-132] + [sp-128])+0xffffffffff000000` |
| `ldr` at 0x39738 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x3973c | load | 4 | `sp -0x80` | `-` |
| `str` at 0x39748 | store | 4 | `sp -0x90` | `([sp-136] + [sp-128])+0xffffffffff000000` |
| `ldr` at 0x3974c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x39758 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x3977c | store | 4 | `sp -0xf8` | `?` |
| `ldr` at 0x39784 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x3978c | store | 4 | `sp -0x104` | `([sp-136] + [sp-128])+0xfffffffffeffffff` |
| `ldr` at 0x39790 | load | 4 | `sp -0xf8` | `-` |
| `str` at 0x39794 | store | 4 | `sp -0x100` | `[sp-248]` |
| `ldr` at 0x3979c | load | 4 | `abs 0x39870` | `-` |
| `ldr` at 0x397a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x397a4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x397ac | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x397d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x397e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x397e4 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x397e8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x397ec | load | 4 | `sp -0x18` | `-` |
| `str` at 0x397f8 | store | 4 | `sp -0x7c` | `ret(0x39494)` |
| `ldr` at 0x397fc | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x39808 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x39814 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x3981c | store | 4 | `sp -0x104` | `[sp-144]-1` |
| `str` at 0x39820 | store | 4 | `sp -0x100` | `ret(0x21674)` |
| `ldr` at 0x39824 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x39828 | load | 4 | `abs 0x39874` | `-` |
| `ldr` at 0x3982c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39830 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x3983c | load | 4 | `sp -0x90` | `-` |
| `str` at 0x39844 | store | 4 | `sp -0x104` | `[sp-144]-1` |
| `ldr` at 0x39848 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3984c | load | 4 | `abs 0x39878` | `-` |
| `ldr` at 0x39850 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39854 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x3985c | load | 4 | `sp -0x7c` | `-` |
| `str` at 0x39860 | store | 4 | `sp -0xfc` | `[sp-124]` |
| `ldr` at 0x39864 | load | 4 | `sp -0xfc` | `-` |
| `ldmdb` at 0x39868 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x39868 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x39868 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x396b8 | `(unknown callee) 0xfbbb8` | `r1` | `sp-120` | `r2` | `sp-120` |
| `bl` at 0x396d4 | `(unknown callee) 0x1b0b8` | `ret(0xfbbb8)` | `?` | `?` | `[sp-28]` |
| `bl` at 0x396e0 | `(unknown callee) 0x21674` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0x396f4 | `(unknown callee) 0x1b49c` | `[sp-28]` | `0x107858` | `ret(0x21674)` | `ret(0x21674)` |
| `bl` at 0x396f8 | `(unknown callee) 0x1b0b8` | `?` | `?` | `?` | `?` |
| `bl` at 0x397a8 | `(unknown callee) 0x1b49c` | `sp-244` | `0x107878` | `[sp-24]` | `([sp-132] + [sp-128])+0xffffffffff000000` |
| `bl` at 0x397dc | `(unknown callee) 0x8d5dc` | `?` | `sp-244` | `[sp-32]` | `?` |
| `bl` at 0x397f0 | `(unknown callee) 0x39494` | `[sp-20]` | `[sp-132]` | `[sp-76]` | `[sp-24]` |
| `bl` at 0x3980c | `(unknown callee) 0x21674` | `ret(0x39494)` | `?` | `?` | `ret(0x39494)` |
| `bl` at 0x39834 | `(unknown callee) 0x1b49c` | `[sp-28]` | `0x1078bc` | `[sp-24]` | `[sp-140]` |
| `bl` at 0x39858 | `(unknown callee) 0x1b49c` | `[sp-28]` | `0x1078ec` | `[sp-24]` | `[sp-140]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x396c4 | `ret(0xfbbb8) >= 0x0 (signed, 32-bit)` | `ret(0xfbbb8) < 0x0 (signed, 32-bit)` |
| `beq` at 0x396d0 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `beq` at 0x39754 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `beq` at 0x39804 | `ret(0x39494) == 0x0` | `ret(0x39494) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x39868 | `r0` | `?` |

## `LBIMemPattern`

- `LBIMemPattern`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3987c to 0x399dc.
  Established. The first branch takes `C == 0` and falls through on `not (C == 0)` (`blo` at
  0x398c4). The call to `0x39d08` receives `[sp-32]`, `sp-4128`, `[sp-8236]` and `0x0` as candidates
  (`bl` at 0x39920). The merged return and generated memory pattern remain unknown (`ldmdb` at
  0x399cc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x39890 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x39894 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x39898 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x3989c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x398a0 | load | 4 | `abs 0x399d0` | `-` |
| `str` at 0x398ac | store | 4 | `sp -0x2024` | `0x0` |
| `ldr` at 0x398b0 | load | 4 | `abs 0x399d0` | `-` |
| `ldr` at 0x398b8 | load | 4 | `sp -0x2024` | `-` |
| `ldr` at 0x398bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x398cc | load | 4 | `abs 0x399d4` | `-` |
| `ldr` at 0x398d0 | load | 4 | `abs 0x399d0` | `-` |
| `ldr` at 0x398d8 | load | 4 | `sp -0x2024` | `-` |
| `ldr` at 0x398dc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x398f0 | store | 4 | `sp -0x202c` | `?` |
| `ldr` at 0x39900 | load | 4 | `abs 0x399d4` | `-` |
| `str` at 0x39908 | store | 4 | `sp -0x2034` | `0x0` |
| `ldr` at 0x3990c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x39918 | load | 4 | `sp -0x202c` | `-` |
| `ldr` at 0x39924 | load | 4 | `abs 0x399d0` | `-` |
| `ldr` at 0x39928 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39930 | load | 4 | `sp -0x2024` | `-` |
| `ldr` at 0x39944 | load | 4 | `abs 0x399d4` | `-` |
| `ldr` at 0x39948 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x39958 | load | 4 | `sp -0x202c` | `-` |
| `ldr` at 0x39964 | load | 4 | `abs 0x399d8` | `-` |
| `str` at 0x3996c | store | 4 | `sp -0x2028` | `ret(0x38dc4)` |
| `ldr` at 0x39974 | load | 4 | `sp -0x2028` | `-` |
| `ldr` at 0x39980 | load | 4 | `abs 0x399d8` | `-` |
| `ldr` at 0x39988 | load | 4 | `sp -0x2028` | `-` |
| `str` at 0x39990 | store | 4 | `sp -0x2030` | `ret(0x38dc4)` |
| `ldr` at 0x39998 | load | 4 | `abs 0x399d0` | `-` |
| `ldr` at 0x3999c | load | 4 | `abs 0x399d0` | `-` |
| `ldr` at 0x399a4 | load | 4 | `sp -0x2024` | `-` |
| `str` at 0x399b0 | store | 4 | `sp -0x2024` | `[sp-8228]+4096` |
| `str` at 0x399c0 | store | 4 | `sp -0x2030` | `0x0` |
| `ldr` at 0x399c8 | load | 4 | `sp -0x2030` | `-` |
| `ldmdb` at 0x399cc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x399cc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x399cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x39920 | `(unknown callee) 0x39d08` | `[sp-32]` | `sp-4128` | `[sp-8236]` | `0x0` |
| `bl` at 0x3995c | `(unknown callee) 0x38dc4` | `[sp-20]` | `([sp-24] + [sp-8228])` | `sp-4128` | `[sp-8236]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0x398c4 | `C == 0` | `not (C == 0)` |
| `beq` at 0x3997c | `ret(0x38dc4) == 0x0` | `ret(0x38dc4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x399cc | `r0` | `?` |

## `nl4r_init_icntlr`

- `nl4r_init_icntlr`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x284e4 to 0x2898c.
  Established. The opening guard takes `[0x2ad25c] == 0x0` and falls through on nonzero (`beq` at
  0x28508). The call to `0x23f2c` has `0x101488`, entry r0, entry r1 and `[0x2ad25c]` as candidates
  (`bl` at 0x28518). The final r0 value is unknown (`ldmdb` at 0x28974). Link-interrupt
  configuration fields remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x284f4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x284f8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x284fc | load | 4 | `abs 0x28978` | `-` |
| `ldr` at 0x28500 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x2850c | load | 4 | `abs 0x2897c` | `-` |
| `ldr` at 0x28510 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28514 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2851c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x28530 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x28538 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2854c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28560 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28574 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28588 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2859c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x285b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x285c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x285d4 | load | 4 | `abs 0x28978` | `-` |
| `ldr` at 0x285d8 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x285e4 | load | 4 | `abs 0x28980` | `-` |
| `ldr` at 0x285ec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28604 | store | 4 | `sp -0x20` | `ret(0xec7d4)` |
| `ldr` at 0x28608 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x28614 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x28618 | store | 4 | `sp -0x2c` | `ret(0xec7d4)` |
| `str` at 0x28624 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x2862c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28640 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28654 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28668 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2867c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28690 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x286a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x286b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x286c8 | load | 4 | `abs 0x28978` | `-` |
| `ldr` at 0x286cc | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x286d8 | load | 4 | `abs 0x28984` | `-` |
| `ldr` at 0x286e0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x286f8 | store | 4 | `sp -0x20` | `ret(0xec7d4)` |
| `ldr` at 0x286fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x28708 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x2870c | store | 4 | `sp -0x2c` | `ret(0xec7d4)` |
| `ldr` at 0x28714 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x28728 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x28730 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28744 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28758 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2876c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28780 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28794 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x287a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x287bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x287d0 | store | 4 | `sp -0x1c` | `0xf` |
| `ldr` at 0x287d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x287ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28800 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28814 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28828 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2883c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28850 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28864 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28878 | store | 4 | `sp -0x1c` | `0xf` |
| `ldr` at 0x28880 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28894 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x288a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x288bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x288d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x288e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x288f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2890c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2891c | load | 4 | `abs 0x28978` | `-` |
| `ldr` at 0x28920 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x2892c | load | 4 | `abs 0x28988` | `-` |
| `ldr` at 0x28934 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2894c | store | 4 | `sp -0x20` | `ret(0xec7d4)` |
| `ldr` at 0x28950 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2895c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x28960 | store | 4 | `sp -0x2c` | `ret(0xec7d4)` |
| `str` at 0x2896c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x28970 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x28974 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x28974 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x28974 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x28518 | `(unknown callee) 0x23f2c` | `0x101488` | `r0` | `r1` | `[0x2ad25c]` |
| `bl` at 0x28544 | `(unknown callee) 0xebed0` | `?` | `0x6` | `0x59` | `sp-28` |
| `bl` at 0x28558 | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x4f` | `sp-28` |
| `bl` at 0x2856c | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x45` | `sp-28` |
| `bl` at 0x28580 | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x3b` | `sp-28` |
| `bl` at 0x28594 | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x31` | `sp-28` |
| `bl` at 0x285a8 | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x27` | `sp-28` |
| `bl` at 0x285bc | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x1d` | `sp-28` |
| `bl` at 0x285d0 | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x13` | `sp-28` |
| `bl` at 0x285e8 | `(unknown callee) 0x23f2c` | `0x1014b4` | `?` | `?` | `[0x2ad25c]` |
| `bl` at 0x285fc | `(unknown callee) 0xec7d4` | `[sp-20]` | `0x2` | `0x6` | `0x0` |
| `bl` at 0x28638 | `(unknown callee) 0xebed0` | `?` | `0x6` | `0x59` | `sp-28` |
| `bl` at 0x2864c | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x4f` | `sp-28` |
| `bl` at 0x28660 | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x45` | `sp-28` |
| `bl` at 0x28674 | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x3b` | `sp-28` |
| `bl` at 0x28688 | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x31` | `sp-28` |
| `bl` at 0x2869c | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x27` | `sp-28` |
| `bl` at 0x286b0 | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x1d` | `sp-28` |
| `bl` at 0x286c4 | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x13` | `sp-28` |
| `bl` at 0x286dc | `(unknown callee) 0x23f2c` | `0x1014ec` | `?` | `?` | `[0x2ad25c]` |
| `bl` at 0x286f0 | `(unknown callee) 0xec7d4` | `[sp-20]` | `0x2` | `0x6` | `0x0` |
| `bl` at 0x2873c | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x5c` | `sp-28` |
| `bl` at 0x28750 | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x52` | `sp-28` |
| `bl` at 0x28764 | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x48` | `sp-28` |
| `bl` at 0x28778 | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x3e` | `sp-28` |
| `bl` at 0x2878c | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x34` | `sp-28` |
| `bl` at 0x287a0 | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x2a` | `sp-28` |
| `bl` at 0x287b4 | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x20` | `sp-28` |
| `bl` at 0x287c8 | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x16` | `sp-28` |
| `bl` at 0x287e4 | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x5b` | `sp-28` |
| `bl` at 0x287f8 | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x51` | `sp-28` |
| `bl` at 0x2880c | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x47` | `sp-28` |
| `bl` at 0x28820 | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x3d` | `sp-28` |
| `bl` at 0x28834 | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x33` | `sp-28` |
| `bl` at 0x28848 | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x29` | `sp-28` |
| `bl` at 0x2885c | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x1f` | `sp-28` |
| `bl` at 0x28870 | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x15` | `sp-28` |
| `bl` at 0x2888c | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x5a` | `sp-28` |
| `bl` at 0x288a0 | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x50` | `sp-28` |
| `bl` at 0x288b4 | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x46` | `sp-28` |
| `bl` at 0x288c8 | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x3c` | `sp-28` |
| `bl` at 0x288dc | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x32` | `sp-28` |
| `bl` at 0x288f0 | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x28` | `sp-28` |
| `bl` at 0x28904 | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x1e` | `sp-28` |
| `bl` at 0x28918 | `(unknown callee) 0xebed0` | `[sp-20]` | `0x6` | `0x14` | `sp-28` |
| `bl` at 0x28930 | `(unknown callee) 0x23f2c` | `0x101524` | `?` | `?` | `[0x2ad25c]` |
| `bl` at 0x28944 | `(unknown callee) 0xec7d4` | `[sp-20]` | `0x2` | `0x6` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x28508 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `beq` at 0x28528 | `? == 0x0` | `? != 0x0` |
| `bls` at 0x285e0 | `[0x2ad25c] <= 0x1 (unsigned, 32-bit)` | `[0x2ad25c] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x28610 | `ret(0xec7d4) >= 0x0 (signed, 32-bit)` | `ret(0xec7d4) < 0x0 (signed, 32-bit)` |
| `bls` at 0x286d4 | `[0x2ad25c] <= 0x1 (unsigned, 32-bit)` | `[0x2ad25c] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x28704 | `ret(0xec7d4) >= 0x0 (signed, 32-bit)` | `ret(0xec7d4) < 0x0 (signed, 32-bit)` |
| `beq` at 0x28720 | `? == 0x0` | `? != 0x0` |
| `bls` at 0x28928 | `[0x2ad25c] <= 0x1 (unsigned, 32-bit)` | `[0x2ad25c] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x28958 | `ret(0xec7d4) >= 0x0 (signed, 32-bit)` | `ret(0xec7d4) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x28974 | `r0` | `?` |

## `nl4r_setup_llp`

- `nl4r_setup_llp`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x28d94 to 0x291fc.
  Established. The first branch takes `[0x2ad25c] == 0x0` and falls through on nonzero (`beq` at
  0x28dbc). The call to `0x23f2c` carries `0x101654`, entry r0, entry r1 and `[0x2ad25c]` as
  candidates (`bl` at 0x28dcc). The recorded exit is `0x0` (`ldmdb` at 0x291f0). Link-setup timing
  and field meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x28da4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x28da8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x28dac | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x28db0 | load | 4 | `abs 0x291f4` | `-` |
| `ldr` at 0x28db4 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x28dc0 | load | 4 | `abs 0x291f8` | `-` |
| `ldr` at 0x28dc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28dc8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28dd0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x28de0 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x28de4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28df0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28e04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28e10 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28e28 | store | 4 | `sp -0x20` | `0x18` |
| `ldr` at 0x28e2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28e38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28e4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28e58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28e6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28e78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28e8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28e98 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28eb4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x28eb8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28ec4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28ed8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28ee4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28efc | store | 4 | `sp -0x20` | `0xf` |
| `ldr` at 0x28f00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28f0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28f20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28f2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28f40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28f4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28f60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28f6c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28f84 | store | 4 | `sp -0x20` | `0x3f` |
| `ldr` at 0x28f88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28f94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28fa8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28fb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28fc8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28fd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28fe8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28ff4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2900c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x29010 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2901c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29030 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2903c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x29054 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x29058 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29064 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2907c | store | 4 | `sp -0x20` | `0x190` |
| `ldr` at 0x29080 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2908c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x290a0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x290b0 | store | 4 | `sp -0x28` | `0x3` |
| `str` at 0x290bc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x290c0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x290c4 | store | 4 | `sp -0x20` | `[sp-40]` |
| `ldr` at 0x290c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x290d4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x290ec | store | 4 | `sp -0x20` | `0x2` |
| `ldr` at 0x290f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x290fc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x29114 | store | 4 | `sp -0x20` | `0x2` |
| `ldr` at 0x29118 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29124 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29138 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x29148 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x2914c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29158 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x29170 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x29174 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29180 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2919c | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x291a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x291ac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x291c4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x291c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x291d4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x291f0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x291f0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x291f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x28dcc | `(unknown callee) 0x23f2c` | `0x101654` | `r0` | `r1` | `[0x2ad25c]` |
| `bl` at 0x28e00 | `(unknown callee) 0xebed0` | `?` | `?` | `0x0` | `sp-32` |
| `bl` at 0x28e20 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+10` | `0x3` | `sp-32` |
| `bl` at 0x28e48 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+10` | `0x1f` | `sp-32` |
| `bl` at 0x28e68 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+10` | `0x20` | `sp-32` |
| `bl` at 0x28e88 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+10` | `0x21` | `sp-32` |
| `bl` at 0x28ea8 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+10` | `0x22` | `sp-32` |
| `bl` at 0x28ed4 | `(unknown callee) 0xebed0` | `?` | `?` | `0x0` | `sp-32` |
| `bl` at 0x28ef4 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+10` | `0x3` | `sp-32` |
| `bl` at 0x28f1c | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+10` | `0x1f` | `sp-32` |
| `bl` at 0x28f3c | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+10` | `0x20` | `sp-32` |
| `bl` at 0x28f5c | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+10` | `0x21` | `sp-32` |
| `bl` at 0x28f7c | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+10` | `0x22` | `sp-32` |
| `bl` at 0x28fa4 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+10` | `0x23` | `sp-32` |
| `bl` at 0x28fc4 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+10` | `0x24` | `sp-32` |
| `bl` at 0x28fe4 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+10` | `0x25` | `sp-32` |
| `bl` at 0x29004 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+10` | `0x26` | `sp-32` |
| `bl` at 0x2902c | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+10` | `0x2` | `sp-32` |
| `bl` at 0x2904c | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+10` | `0x4` | `sp-32` |
| `bl` at 0x29074 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+10` | `0x5` | `sp-32` |
| `bl` at 0x2909c | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+10` | `0x6` | `sp-32` |
| `bl` at 0x290e4 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+18` | `0x2` | `sp-32` |
| `bl` at 0x2910c | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+18` | `0x4` | `sp-32` |
| `bl` at 0x29134 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+18` | `0x5` | `sp-32` |
| `bl` at 0x29168 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+26` | `0x0` | `sp-32` |
| `bl` at 0x29190 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+26` | `0x4` | `sp-32` |
| `bl` at 0x291bc | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+26` | `0x0` | `sp-32` |
| `bl` at 0x291e4 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+26` | `0x4` | `sp-32` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x28dbc | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `beq` at 0x28dd8 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x290a8 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `beq` at 0x29140 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x291f0 | `r0` | `0x0` |

## `nl4r_is_scramble_needed`

- `nl4r_is_scramble_needed`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x29480 to 0x29548.
  Established. The first call to `0x4f090` has entry r0 through r3 as candidates (`bl` at 0x29498);
  its later comparison retains an unknown operand against `0x44` (`beq` at 0x294a8). The final r0
  value is unknown (`ldmdb` at 0x29544). The scrambling decision cannot be established from these
  supported predicates.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x29490 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x29494 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x294b0 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x294c0 | store | 4 | `sp -0x20` | `ret(0x1f1dc)` |
| `ldr` at 0x294c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x294c8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x294e0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x294e8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x294fc | store | 4 | `sp -0x24` | `ret(0x74b30)` |
| `ldr` at 0x29500 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x29504 | load | 4 | `ret(0x74b30) +0x10` | `-` |
| `str` at 0x29514 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x2951c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x29520 | load | 4 | `ret(0x74b30) +0x18` | `-` |
| `str` at 0x29530 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x2953c | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x29540 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x29544 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x29544 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x29544 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x29498 | `(unknown callee) 0x4f090` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x294bc | `(unknown callee) 0x1f1dc` | `0x98` | `?` | `?` | `?` |
| `bl` at 0x294f4 | `(unknown callee) 0x74b30` | `[sp-24]+1` | `?` | `ret(0x1f1dc)` | `[sp-24]+1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x294a8 | `? == 0x44` | `? != 0x44` |
| `bne` at 0x294d8 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x2950c | `[ret(0x74b30)+16] == 0x44` | `[ret(0x74b30)+16] != 0x44` |
| `bne` at 0x29528 | `[ret(0x74b30)+24] != 0x0` | `[ret(0x74b30)+24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x29544 | `r0` | `?` |

## `jmstr_move_to_idle`

- `jmstr_move_to_idle`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdcb88 to 0xdce0c.
  Established. The entry-r2 guard takes zero and falls through on nonzero (`beq` at 0xdcbc0). The
  call to `0x1b124` receives unknown, entry r1, unknown and unknown as candidates (`bl` at 0xdcbe8).
  The later computed selector remains unresolved (`ldrls` at 0xdcc6c). Idle-state transitions on
  selected edges and the merged return remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdcb98 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xdcb9c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xdcba0 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xdcba4 | store | 4 | `sp -0x24` | `r3` |
| `str` at 0xdcbac | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0xdcbb4 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xdcbb8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdcbc4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdcbd8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdcbf0 | store | 4 | `sp -0x30` | `ret(0x1b124)` |
| `ldr` at 0xdcbf4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdcc00 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdcc04 | load | 4 | `abs 0xdcdfc` | `-` |
| `str` at 0xdcc10 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xdcc18 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdcc2c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdcc38 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xdcc48 | store | 4 | `sp -0x28` | `0x10` |
| `ldr` at 0xdcc4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdcc58 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xdcc60 | store | 4 | `sp -0x28` | `0x18` |
| `ldr` at 0xdcc64 | load | 4 | `sp -0x18` | `-` |
| `ldrls` at 0xdcc6c | load | 4 | `(unknown)` | `-` |
| `str` at 0xdcd00 | store | 4 | `sp -0x2c` | `0xff000000` |
| `str` at 0xdcd08 | store | 4 | `sp -0x38` | `0x5` |
| `ldr` at 0xdcd0c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xdcd18 | load | 4 | `abs 0xdce00` | `-` |
| `str` at 0xdcd28 | store | 4 | `sp -0x44` | `sp-60` |
| `ldr` at 0xdcd2c | load | 4 | `abs 0x2ad248` | `-` |
| `ldr` at 0xdcd30 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdcd34 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xdcd54 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdcd58 | load | 4 | `abs 0xdce04` | `-` |
| `ldr` at 0xdcd60 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdcd6c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xdcd78 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xdcd80 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdcd8c | load | 4 | `abs 0xdce08` | `-` |
| `ldr` at 0xdcd90 | load | 4 | `abs 0x2ad24c` | `-` |
| `ldr` at 0xdcd94 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdcd98 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdcd9c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdcda0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdcdb8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdcdbc | load | 4 | `abs 0xdce04` | `-` |
| `ldr` at 0xdcdc4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xdcdd0 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xdcdd8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdcde4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xdcdf0 | store | 4 | `sp -0x40` | `0x1` |
| `ldr` at 0xdcdf4 | load | 4 | `sp -0x40` | `-` |
| `ldmdb` at 0xdcdf8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xdcdf8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdcdf8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdcdf8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdcbe8 | `(unknown callee) 0x1b124` | `?` | `r1` | `?` | `?` |
| `bl` at 0xdcc08 | `(unknown callee) 0x1b49c` | `[sp-36]` | `0x135350` | `?` | `ret(0x1b124)` |
| `bl` at 0xdcc40 | `(unknown callee) 0x1b43c` | `ret(0x1b124)` | `0x0` | `?` | `?` |
| `mov` at 0xdcd44 | `(unknown callee) (unknown target)` | `?` | `0x5` | `sp-60` | `sp-44` |
| `bl` at 0xdcd5c | `(unknown callee) 0x1b49c` | `[sp-36]` | `0x13531c` | `?` | `ret(unknown)` |
| `bl` at 0xdcd70 | `(unknown callee) 0x1b400` | `[sp-48]` | `?` | `?` | `[sp-48]` |
| `mov` at 0xdcda8 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` |
| `bl` at 0xdcdc0 | `(unknown callee) 0x1b49c` | `[sp-36]` | `0x13531c` | `?` | `ret(unknown)` |
| `bl` at 0xdcdc8 | `(unknown callee) 0x1b400` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0xdcde8 | `(unknown callee) 0x1b400` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xdcbc0 | `r2 == 0x0` | `r2 != 0x0` |
| `bne` at 0xdcbfc | `ret(0x1b124) != 0x0` | `ret(0x1b124) == 0x0` |
| `beq` at 0xdcc54 | `Z == 1` | `not (Z == 1)` |
| `ldrls` at 0xdcc6c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0xdcd14 | `0x5 == 0x0` | `0x5 != 0x0` |
| `bne` at 0xdcd50 | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |
| `beq` at 0xdcd68 | `[sp-48] == 0x0` | `[sp-48] != 0x0` |
| `beq` at 0xdcd88 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xdcdb4 | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |
| `beq` at 0xdcde0 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdcdf8 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xdcc6c).

## `jmstr_run_tck`

- `jmstr_run_tck`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdce0c to 0xdceb0.
  Established. The initial guard takes `r0 == 0x0` and falls through on nonzero (`beq` at 0xdce38).
  The indirect call has unknown, entry r1, `0x0` and `0x0` as candidates (`mov` at 0xdce70); its
  target is unknown (`mov` at 0xdce70). The final r0 expression is unknown (`ldmdb` at 0xdcea4). The
  clock-run effect remains unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdce1c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdce20 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xdce24 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xdce2c | store | 4 | `sp -0x20` | `0x10` |
| `ldr` at 0xdce30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdce3c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdce44 | store | 4 | `sp -0x20` | `0x18` |
| `ldr` at 0xdce48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdce54 | load | 4 | `abs 0xdcea8` | `-` |
| `ldr` at 0xdce58 | load | 4 | `abs 0x2ad24c` | `-` |
| `ldr` at 0xdce5c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdce60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdce80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdce84 | load | 4 | `abs 0xdceac` | `-` |
| `str` at 0xdce90 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xdce9c | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0xdcea0 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xdcea4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdcea4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdcea4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0xdce70 | `(unknown callee) (unknown target)` | `?` | `r1` | `0x0` | `0x0` |
| `bl` at 0xdce88 | `(unknown callee) 0x1b49c` | `[sp-28]` | `0x13531c` | `?` | `ret(unknown)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xdce38 | `r0 == 0x0` | `r0 != 0x0` |
| `beq` at 0xdce50 | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0xdce7c | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdcea4 | `r0` | `?` |

## `jmstr_custom_seq`

- `jmstr_custom_seq`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdceb0 to 0xdcf54.
  Established. Zero entry r0 takes the first branch and nonzero falls through (`beq` at 0xdcee0).
  The indirect call carries unknown, entry r1, entry r2 and entry r3 as candidates (`mov` at
  0xdcf14); its target is unknown (`mov` at 0xdcf14). The final r0 expression is unknown (`ldmdb` at
  0xdcf48). The custom scan-sequence encoding remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdcec0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdcec4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xdcec8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xdcecc | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xdced4 | store | 4 | `sp -0x24` | `0x10` |
| `ldr` at 0xdced8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdcee4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdceec | store | 4 | `sp -0x24` | `0x18` |
| `ldr` at 0xdcef0 | load | 4 | `abs 0xdcf4c` | `-` |
| `ldr` at 0xdcef4 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xdcef8 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xdcefc | load | 4 | `abs 0x2ad248` | `-` |
| `ldr` at 0xdcf00 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdcf04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdcf08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdcf0c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdcf24 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xdcf28 | load | 4 | `abs 0xdcf50` | `-` |
| `str` at 0xdcf34 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xdcf40 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0xdcf44 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xdcf48 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdcf48 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdcf48 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0xdcf14 | `(unknown callee) (unknown target)` | `?` | `r1` | `r2` | `r3` |
| `bl` at 0xdcf2c | `(unknown callee) 0x1b49c` | `[sp+4]` | `0x13531c` | `?` | `ret(unknown)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xdcee0 | `r0 == 0x0` | `r0 != 0x0` |
| `bne` at 0xdcf20 | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdcf48 | `r0` | `?` |

## `IcmbMsgRqRs`

- `IcmbMsgRqRs`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaa7dc to 0xaac6c.
  Established. The first branch takes `[r1+156] == 0xff` and falls through on inequality (`beq` at
  0xaa808). The call to `0xae1bc` has `[r1+160]:1`, `[r1+168]:1`, `[r1+164]` and `[r1+156]` as
  candidates (`bl` at 0xaa858). The return carries `[sp-32]` (`ldmdb` at 0xaac5c). Request/reply
  success semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaa7ec | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xaa7f0 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xaa7f8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xaa7fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaa800 | load | 4 | `r1 +0x9c` | `-` |
| `ldr` at 0xaa80c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaa810 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xaa814 | load | 1 | `r1 +0xa0` | `-` |
| `ldr` at 0xaa818 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xaa81c | load | 1 | `r1 +0xa8` | `-` |
| `ldr` at 0xaa820 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaa824 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaa828 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xaa830 | store | 4 | `sp -0x1b4` | `r1+169` |
| `ldr` at 0xaa834 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaa838 | load | 4 | `r1 +0xec` | `-` |
| `str` at 0xaa83c | store | 4 | `sp -0x1b0` | `[r1+236]` |
| `ldr` at 0xaa840 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xaa848 | store | 4 | `sp -0x1ac` | `r1+8` |
| `ldr` at 0xaa850 | load | 4 | `r1 +0xa4` | `-` |
| `ldr` at 0xaa854 | load | 4 | `r1 +0x9c` | `-` |
| `str` at 0xaa85c | store | 4 | `r1 +0x98` | `ret(0xae1bc)` |
| `strb` at 0xaa874 | store | 1 | `(unknown)` | `(ret(0xab0ac) & 0xff)` |
| `ldrb` at 0xaa880 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xaa884 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaa888 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaa890 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xaa898 | store | 4 | `sp -0x1b4` | `sp-184` |
| `ldr` at 0xaa8a0 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xaa8a4 | load | 4 | `[sp-28] +0x98` | `-` |
| `str` at 0xaa8b0 | store | 4 | `sp -0xbc` | `ret(0xae900)` |
| `ldr` at 0xaa8b4 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xaa8b8 | load | 2 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xaa8bc | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xaa8c0 | load | 2 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0xaa8c8 | load | 1 | `sp -0xbc` | `-` |
| `str` at 0xaa8d0 | store | 4 | `sp -0x1b4` | `sp-340` |
| `str` at 0xaa8ec | store | 4 | `sp -0x158` | `ret(0xae4c8)` |
| `str` at 0xaa8f4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xaa8f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaa8fc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xaa900 | load | 4 | `[sp-28] +0x18c` | `-` |
| `ldr` at 0xaa90c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xaa918 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xaa92c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaa930 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xaa93c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaa944 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xaa954 | store | 4 | `sp -0x19c` | `0x3` |
| `ldr` at 0xaa958 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaa960 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xaa970 | store | 4 | `sp -0x170` | `0x0` |
| `ldr` at 0xaa974 | load | 4 | `sp -0x170` | `-` |
| `ldr` at 0xaa984 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaa98c | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xaa994 | load | 4 | `sp -0x158` | `-` |
| `str` at 0xaa9a0 | store | 4 | `sp -0x16c` | `ret(0xca3c0)` |
| `ldr` at 0xaa9a4 | load | 4 | `sp -0x16c` | `-` |
| `ldr` at 0xaa9a8 | load | 4 | `abs 0xaac60` | `-` |
| `ldr` at 0xaa9bc | load | 4 | `sp -0x170` | `-` |
| `str` at 0xaa9c4 | store | 4 | `sp -0x170` | `[sp-368]+1` |
| `ldr` at 0xaa9cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaa9d0 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xaa9e0 | store | 4 | `sp -0x19c` | `0x0` |
| `ldr` at 0xaa9e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaa9ec | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xaa9f8 | load | 4 | `sp -0x170` | `-` |
| `str` at 0xaaa0c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xaaa10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaaa14 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaaa18 | load | 4 | `[sp-28] +0x190` | `-` |
| `ldr` at 0xaaa24 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xaaa30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaaa34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaaa3c | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xaaa50 | store | 4 | `sp -0x168` | `ret(0xca200)` |
| `ldr` at 0xaaa54 | load | 4 | `sp -0x168` | `-` |
| `ldr` at 0xaaa58 | load | 4 | `abs 0xaac64` | `-` |
| `ldr` at 0xaaa68 | load | 4 | `sp -0x168` | `-` |
| `ldr` at 0xaaa78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaaa80 | load | 4 | `abs 0xaac68` | `-` |
| `ldrb` at 0xaaa88 | load | 1 | `sp -0x168` | `-` |
| `str` at 0xaaab0 | store | 4 | `sp -0x164` | `ret(0xae5a0)` |
| `ldr` at 0xaaab4 | load | 4 | `sp -0x164` | `-` |
| `ldr` at 0xaaac4 | load | 4 | `sp -0x160` | `-` |
| `str` at 0xaaad0 | store | 4 | `sp -0x178` | `ret(0xae708)` |
| `ldr` at 0xaaad4 | load | 4 | `sp -0x178` | `-` |
| `ldr` at 0xaaae8 | load | 4 | `sp -0x160` | `-` |
| `strb` at 0xaaaf8 | store | 1 | `(unknown)` | `(ret(0xae794) & 0xff)` |
| `ldr` at 0xaaafc | load | 4 | `sp -0x160` | `-` |
| `str` at 0xaab08 | store | 4 | `sp -0x180` | `ret(0xae810)` |
| `ldr` at 0xaab0c | load | 4 | `sp -0x160` | `-` |
| `str` at 0xaab18 | store | 4 | `sp -0x184` | `ret(0xae874)` |
| `ldr` at 0xaab1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaab20 | load | 4 | `sp -0x184` | `-` |
| `str` at 0xaab24 | store | 4 | `[sp-28] +0x194` | `ret(0xae874)` |
| `ldr` at 0xaab28 | load | 4 | `sp -0x184` | `-` |
| `ldr` at 0xaab34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaab38 | load | 4 | `sp -0x180` | `-` |
| `ldr` at 0xaab3c | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0xaab54 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xaab5c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xaab68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaab6c | load | 4 | `sp -0x160` | `-` |
| `str` at 0xaab74 | store | 4 | `[sp-28] +0x188` | `ret(0xae8a8)` |
| `ldr` at 0xaab78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaab7c | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xaab88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaab8c | load | 4 | `[sp-28] +0x188` | `-` |
| `str` at 0xaab98 | store | 4 | `sp -0x184` | `ret(0xae3a8)` |
| `ldr` at 0xaab9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaaba0 | load | 4 | `[sp-28] +0x188` | `-` |
| `str` at 0xaabac | store | 4 | `sp -0x180` | `ret(0xae374)` |
| `ldr` at 0xaabb0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaabb4 | load | 4 | `[sp-28] +0x188` | `-` |
| `str` at 0xaabc0 | store | 4 | `sp -0x188` | `ret(0xae334)` |
| `ldr` at 0xaabc4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaabc8 | load | 4 | `[sp-28] +0x188` | `-` |
| `str` at 0xaabd4 | store | 4 | `sp -0x18c` | `ret(0xae2f4)` |
| `ldr` at 0xaabd8 | load | 4 | `sp -0x184` | `-` |
| `ldr` at 0xaabe4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaabe8 | load | 4 | `sp -0x180` | `-` |
| `ldr` at 0xaabec | load | 4 | `[sp-28] +0x9c` | `-` |
| `ldr` at 0xaabf8 | load | 4 | `sp -0x188` | `-` |
| `ldr` at 0xaac04 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaac08 | load | 4 | `sp -0x18c` | `-` |
| `ldr` at 0xaac0c | load | 4 | `[sp-28] +0xa4` | `-` |
| `str` at 0xaac1c | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xaac20 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xaac28 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xaac30 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xaac38 | store | 4 | `sp -0x24` | `?` |
| `ldrb` at 0xaac48 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xaac54 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xaac5c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xaac5c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaac5c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaac5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaa858 | `(unknown callee) 0xae1bc` | `[r1+160]:1` | `[r1+168]:1` | `[r1+164]` | `[r1+156]` |
| `bl` at 0xaa864 | `(unknown callee) 0xab0ac` | `0x20` | `?` | `?` | `?` |
| `bl` at 0xaa8a8 | `(unknown callee) 0xae900` | `?` | `[[sp-28]+4]` | `[sp-28]+8` | `[[sp-28]+152]` |
| `bl` at 0xaa8e4 | `(unknown callee) 0xae4c8` | `[[sp-24]+4]:2` | `[[sp-28]]:2` | `sp-184` | `[sp-188]:1` |
| `bl` at 0xaa928 | `(unknown callee) 0xab848` | `0x14` | `?` | `?` | `?` |
| `bl` at 0xaa94c | `(unknown callee) 0xca450` | `[[sp-24]+12]` | `sp-424` | `sp-424` | `[sp-24]` |
| `bl` at 0xaa968 | `(unknown callee) 0xca48c` | `[[sp-24]+12]` | `sp-424` | `sp-424` | `[sp-24]` |
| `bl` at 0xaa998 | `(unknown callee) 0xca3c0` | `[[sp-24]+12]` | `sp-340` | `?` | `[sp-24]` |
| `bl` at 0xaa9b8 | `(unknown callee) 0xab848` | `0x1` | `?` | `ret(0xca3c0)` | `0xfffff698` |
| `bl` at 0xaa9f4 | `(unknown callee) 0xca48c` | `[[sp-24]+12]` | `sp-424` | `sp-424` | `[sp-24]` |
| `bl` at 0xaaa48 | `(unknown callee) 0xca200` | `[[sp-24]+12]` | `[sp-28]+240` | `0x98` | `[sp-28]+240` |
| `bl` at 0xaaaa8 | `(unknown callee) 0xae5a0` | `[sp-28]+240` | `[sp-360]:1` | `sp-370` | `sp-352` |
| `bl` at 0xaaac8 | `(unknown callee) 0xae708` | `[sp-352]` | `?` | `?` | `ret(0xae5a0)` |
| `bl` at 0xaaaec | `(unknown callee) 0xae794` | `[sp-352]` | `?` | `?` | `ret(0xae708)` |
| `bl` at 0xaab00 | `(unknown callee) 0xae810` | `[sp-352]` | `sp-20` | `?` | `ret(0xae794)` |
| `bl` at 0xaab10 | `(unknown callee) 0xae874` | `[sp-352]` | `?` | `?` | `ret(0xae810)` |
| `bl` at 0xaab70 | `(unknown callee) 0xae8a8` | `[sp-352]` | `?` | `?` | `?` |
| `bl` at 0xaab90 | `(unknown callee) 0xae3a8` | `[[sp-28]+392]` | `?` | `?` | `[sp-28]` |
| `bl` at 0xaaba4 | `(unknown callee) 0xae374` | `[[sp-28]+392]` | `?` | `?` | `[sp-28]` |
| `bl` at 0xaabb8 | `(unknown callee) 0xae334` | `[[sp-28]+392]` | `?` | `?` | `[sp-28]` |
| `bl` at 0xaabcc | `(unknown callee) 0xae2f4` | `[[sp-28]+392]` | `?` | `?` | `[sp-28]` |
| `bl` at 0xaac50 | `(unknown callee) 0xab0ac` | `?` | `sp-20` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xaa808 | `[r1+156] == 0xff` | `[r1+156] != 0xff` |
| `bge` at 0xaa908 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xaa914 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xaa920 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xaa938 | `[[sp-28]+4] != 0xb` | `[[sp-28]+4] == 0xb` |
| `ble` at 0xaa97c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xaa9b0 | `ret(0xca3c0) != 0xfffff698` | `ret(0xca3c0) == 0xfffff698` |
| `bne` at 0xaa9d8 | `[[sp-28]+4] != 0xb` | `[[sp-28]+4] == 0xb` |
| `bne` at 0xaaa00 | `Z == 0` | `not (Z == 0)` |
| `bge` at 0xaaa20 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xaaa2c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0xaaa60 | `ret(0xca200) != 0xfffff69e` | `ret(0xca200) == 0xfffff69e` |
| `bgt` at 0xaaa70 | `ret(0xca200) > 0x0 (signed, 32-bit)` | `ret(0xca200) <= 0x0 (signed, 32-bit)` |
| `bne` at 0xaaabc | `ret(0xae5a0) != 0x0` | `ret(0xae5a0) == 0x0` |
| `beq` at 0xaaadc | `ret(0xae708) == 0x1` | `ret(0xae708) != 0x1` |
| `bne` at 0xaab30 | `[sp-388] != 0x0` | `[sp-388] == 0x0` |
| `bne` at 0xaab44 | `[sp-384] != [[sp-28]+4]` | `[sp-384] == [[sp-28]+4]` |
| `bne` at 0xaab64 | `? != ?` | `? == ?` |
| `bne` at 0xaab84 | `[[sp-28]+4] != 0x20` | `[[sp-28]+4] == 0x20` |
| `bne` at 0xaabe0 | `[sp-388] != 0x0` | `[sp-388] == 0x0` |
| `bne` at 0xaabf4 | `[sp-384] != [[sp-28]+156]` | `[sp-384] == [[sp-28]+156]` |
| `bne` at 0xaac00 | `[sp-392] != 0x1` | `[sp-392] == 0x1` |
| `bne` at 0xaac14 | `ret(0xae2f4) != [[sp-28]+164]` | `ret(0xae2f4) == [[sp-28]+164]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaac5c | `r0` | `[sp-32]` |
