# ARM board reset callers in 1.40.5

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
resetSource has an unresolved computed transfer; its tables and return cover known edges
only. Indirect call targets and numeric callee effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/board-reset-facts.txt) retain all tool sections.

## `reset`

- `reset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4e54c to 0x4e5ec.
  Established. Entry `r0 == 0x5` takes the branch to `0x4e584` (`beq` at 0x4e568). The unnamed
  `0x4eb30` call has four unknown candidates, and its zero result takes the branch to `0x4e5a8`
  (`bl` at 0x4e584, `beq` at 0x4e598). That arm includes unnamed `0x4ebc8` with `0xa`, unknown,
  unknown, `ret(0x4eb30)`, followed by unnamed `0x4e63c` with `0x5` and three unknown candidates
  (`bl` at 0x4e5ac, `bl` at 0x4e5b4). Physical reset sequencing remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4e55c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x4e560 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4e56c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4e578 | load | 4 | `abs 0x4e5e8` | `-` |
| `str` at 0x4e58c | store | 4 | `sp -0x1c` | `ret(0x4eb30)` |
| `ldr` at 0x4e590 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4e59c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4e5a0 | store | 4 | `sp -0x20` | `ret(0x4eb30)` |
| `str` at 0x4e5bc | store | 4 | `sp -0x1c` | `ret(0x4e63c)` |
| `ldr` at 0x4e5c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4e5cc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4e5d0 | store | 4 | `sp -0x20` | `ret(0x4e63c)` |
| `str` at 0x4e5dc | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x4e5e0 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x4e5e4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4e5e4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4e5e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4e570 | `(unknown callee) 0x4ea90` | `r0` | `r1` | `r2` | `r0` |
| `bl` at 0x4e580 | `(unknown callee) 0x2079c` | `0x10b0ac` | `ret(0x4ea90)` | `?` | `ret(0x4ea90)` |
| `bl` at 0x4e584 | `(unknown callee) 0x4eb30` | `?` | `?` | `?` | `?` |
| `bl` at 0x4e5ac | `(unknown callee) 0x4ebc8` | `0xa` | `?` | `?` | `ret(0x4eb30)` |
| `bl` at 0x4e5b4 | `(unknown callee) 0x4e63c` | `0x5` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4e568 | `r0 == 0x5` | `r0 != 0x5` |
| `beq` at 0x4e598 | `ret(0x4eb30) == 0x0` | `ret(0x4eb30) != 0x0` |
| `beq` at 0x4e5c8 | `ret(0x4e63c) == 0x0` | `ret(0x4e63c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4e5e4 | `r0` | `?` |

## `resetHold`

- `resetHold`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4e5ec to 0x4e63c.
  Established. The branch takes entry `r0 == 0x5` to `0x4e624` (`beq` at 0x4e608). The unnamed calls
  there include `0xe5db4` with four unknown candidates, then `0x4eb30` with `ret(0xe5db4)` followed
  by three unknowns (`bl` at 0x4e624, `bl` at 0x4e628). The exit carries `ret(0x4eb30)` (`ldmdb` at
  0x4e634). Reset-hold signaling is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4e5fc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x4e600 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4e60c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4e618 | load | 4 | `abs 0x4e638` | `-` |
| `ldmdb` at 0x4e634 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4e634 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4e634 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4e610 | `(unknown callee) 0x4ea90` | `r0` | `r1` | `r2` | `r0` |
| `bl` at 0x4e620 | `(unknown callee) 0x2079c` | `0x10b0b8` | `ret(0x4ea90)` | `?` | `ret(0x4ea90)` |
| `bl` at 0x4e624 | `(unknown callee) 0xe5db4` | `?` | `?` | `?` | `?` |
| `bl` at 0x4e628 | `(unknown callee) 0x4eb30` | `ret(0xe5db4)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4e608 | `r0 == 0x5` | `r0 != 0x5` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4e634 | `r0` | `ret(0x4eb30)` |

## `resetRelease`

- `resetRelease`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4e63c to 0x4e88c.
  Established. Four-byte absolute reads include `0x2af348` and `0x2af34c` (`ldr` at 0x4e7c0, `ldr`
  at 0x4e82c). Their zero tests take branches past separate unknown-target calls (`beq` at 0x4e7c8,
  `beq` at 0x4e834, `mov` at 0x4e7dc, `mov` at 0x4e848). Those calls carry `0xff`, unknown, unknown,
  `[0x2af348]`, and `0xff`, unknown, unknown, `[0x2af34c]`, respectively (`mov` at 0x4e7dc, `mov` at
  0x4e848). The unnamed `0x42284` call has `0x0`, `0xff`, `0x5`, unknown (`bl` at 0x4e7fc). Callback
  and reset-release effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4e64c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4e65c | store | 4 | `sp -0x18` | `?` |
| `str` at 0x4e664 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x4e668 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4e674 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4e680 | load | 4 | `abs 0x4e884` | `-` |
| `ldr` at 0x4e690 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e6a8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4e6ac | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0x4e6bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e6c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e6d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e6e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e6ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e6f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e704 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e710 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e71c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e728 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e734 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e740 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e74c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e758 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e764 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e770 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e77c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e788 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e794 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e7bc | load | 4 | `abs 0x4e888` | `-` |
| `ldr` at 0x4e7c0 | load | 4 | `abs 0x2af348` | `-` |
| `ldr` at 0x4e7cc | load | 4 | `abs 0x4e888` | `-` |
| `ldr` at 0x4e7d0 | load | 4 | `abs 0x2af348` | `-` |
| `str` at 0x4e7e4 | store | 4 | `sp -0x1c` | `ret(unknown)` |
| `str` at 0x4e804 | store | 4 | `sp -0x20` | `ret(0x42284)` |
| `ldr` at 0x4e808 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4e814 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4e820 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4e824 | store | 4 | `sp -0x1c` | `ret(0x42284)` |
| `ldr` at 0x4e828 | load | 4 | `abs 0x4e888` | `-` |
| `ldr` at 0x4e82c | load | 4 | `abs 0x2af34c` | `-` |
| `ldr` at 0x4e838 | load | 4 | `abs 0x4e888` | `-` |
| `ldr` at 0x4e83c | load | 4 | `abs 0x2af34c` | `-` |
| `str` at 0x4e850 | store | 4 | `sp -0x20` | `ret(unknown)` |
| `ldr` at 0x4e854 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4e860 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4e86c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4e870 | store | 4 | `sp -0x1c` | `ret(unknown)` |
| `ldr` at 0x4e874 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4e878 | store | 4 | `sp -0x24` | `[sp-28]` |
| `ldr` at 0x4e87c | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x4e880 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4e880 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4e880 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4e650 | `(unknown callee) 0x4f4d0` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x4e678 | `(unknown callee) 0x4ea90` | `[sp-20]` | `?` | `?` | `[sp-20]` |
| `bl` at 0x4e688 | `(unknown callee) 0x2079c` | `0x10b0c8` | `ret(0x4ea90)` | `?` | `ret(0x4ea90)` |
| `bl` at 0x4e698 | `(unknown callee) 0xe5ccc` | `[sp-24]` | `sp-32` | `?` | `sp-32` |
| `bl` at 0x4e6b8 | `(unknown callee) 0x4ebc8` | `0xa` | `?` | `?` | `ret(0xe5ccc)` |
| `bl` at 0x4e7a4 | `(unknown callee) 0x82fac` | `ret(0x4ebc8)` | `?` | `?` | `[sp-24]` |
| `bl` at 0x4e7a8 | `(unknown callee) 0x862bc` | `ret(0x82fac)` | `?` | `?` | `?` |
| `bl` at 0x4e7ac | `(unknown callee) 0x688a4` | `?` | `?` | `?` | `?` |
| `bl` at 0x4e7b8 | `(unknown callee) 0x5e538` | `0xff` | `0x0` | `?` | `?` |
| `mov` at 0x4e7dc | `(unknown callee) (unknown target)` | `0xff` | `?` | `?` | `[0x2af348]` |
| `bl` at 0x4e7ec | `(unknown callee) 0x40be0` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x4e7fc | `(unknown callee) 0x42284` | `0x0` | `0xff` | `0x5` | `?` |
| `mov` at 0x4e848 | `(unknown callee) (unknown target)` | `0xff` | `?` | `?` | `[0x2af34c]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4e670 | `[sp-20] == 0x5` | `[sp-20] != 0x5` |
| `bne` at 0x4e6a4 | `ret(0xe5ccc) != 0x0` | `ret(0xe5ccc) == 0x0` |
| `beq` at 0x4e6c4 | `[sp-24] == 0x43` | `[sp-24] != 0x43` |
| `beq` at 0x4e6d0 | `[sp-24] == 0x33` | `[sp-24] != 0x33` |
| `beq` at 0x4e6dc | `[sp-24] == 0x35` | `[sp-24] != 0x35` |
| `beq` at 0x4e6e8 | `[sp-24] == 0x34` | `[sp-24] != 0x34` |
| `beq` at 0x4e6f4 | `[sp-24] == 0x41` | `[sp-24] != 0x41` |
| `beq` at 0x4e700 | `[sp-24] == 0x36` | `[sp-24] != 0x36` |
| `beq` at 0x4e70c | `[sp-24] == 0x39` | `[sp-24] != 0x39` |
| `beq` at 0x4e718 | `[sp-24] == 0x37` | `[sp-24] != 0x37` |
| `beq` at 0x4e724 | `[sp-24] == 0x38` | `[sp-24] != 0x38` |
| `beq` at 0x4e730 | `[sp-24] == 0x30` | `[sp-24] != 0x30` |
| `beq` at 0x4e73c | `[sp-24] == 0x51` | `[sp-24] != 0x51` |
| `beq` at 0x4e748 | `[sp-24] == 0x40` | `[sp-24] != 0x40` |
| `beq` at 0x4e754 | `[sp-24] == 0x4f` | `[sp-24] != 0x4f` |
| `beq` at 0x4e760 | `[sp-24] == 0x42` | `[sp-24] != 0x42` |
| `beq` at 0x4e76c | `[sp-24] == 0x46` | `[sp-24] != 0x46` |
| `beq` at 0x4e778 | `[sp-24] == 0x48` | `[sp-24] != 0x48` |
| `beq` at 0x4e784 | `[sp-24] == 0x4c` | `[sp-24] != 0x4c` |
| `beq` at 0x4e790 | `[sp-24] == 0x4d` | `[sp-24] != 0x4d` |
| `beq` at 0x4e79c | `[sp-24] == 0x7c` | `[sp-24] != 0x7c` |
| `beq` at 0x4e7c8 | `[0x2af348] == 0x0` | `[0x2af348] != 0x0` |
| `beq` at 0x4e810 | `ret(0x42284) == 0x0` | `ret(0x42284) != 0x0` |
| `bne` at 0x4e81c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x4e834 | `[0x2af34c] == 0x0` | `[0x2af34c] != 0x0` |
| `beq` at 0x4e85c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x4e868 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4e880 | `r0` | `?` |

## `softReset`

- `softReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4e88c to 0x4e918.
  Established. A zero `[0x2af350]` takes the branch to `0x4e900` (`beq` at 0x4e8d8); its
  fall-through includes an unknown-target call whose candidates are unknown, unknown, unknown,
  `[0x2af350]` (`mov` at 0x4e8f0). Separate four-byte assignments at `sp -0x1c` retain
  `ret(unknown)` and `0x0` (`str` at 0x4e8f8, `str` at 0x4e904). Soft-reset delivery is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4e89c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x4e8a4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x4e8a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4e8b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4e8c0 | load | 4 | `abs 0x4e910` | `-` |
| `ldr` at 0x4e8cc | load | 4 | `abs 0x4e914` | `-` |
| `ldr` at 0x4e8d0 | load | 4 | `abs 0x2af350` | `-` |
| `ldr` at 0x4e8dc | load | 4 | `abs 0x4e914` | `-` |
| `ldrb` at 0x4e8e0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x4e8e4 | load | 4 | `abs 0x2af350` | `-` |
| `str` at 0x4e8f8 | store | 4 | `sp -0x1c` | `ret(unknown)` |
| `str` at 0x4e904 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x4e908 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x4e90c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4e90c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4e90c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4e8b8 | `(unknown callee) 0x4ea90` | `r0` | `r1` | `r2` | `r0` |
| `bl` at 0x4e8c8 | `(unknown callee) 0x2079c` | `0x10b0dc` | `ret(0x4ea90)` | `?` | `ret(0x4ea90)` |
| `mov` at 0x4e8f0 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `[0x2af350]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4e8b0 | `r0 == 0x5` | `r0 != 0x5` |
| `beq` at 0x4e8d8 | `[0x2af350] == 0x0` | `[0x2af350] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4e90c | `r0` | `?` |

## `resetSource`

- `resetSource`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4ea90 to 0x4eb30.
  Established. The computed transfer takes an unresolved edge when `r0-1 <= 0x4` unsigned 32-bit and
  falls through when `r0-1 > 0x4` (`ldrls` at 0x4eab0); its known fall-through stores four-byte
  `0x10b128` at `sp -0x18`, then the exit carries that value (`str` at 0x4eb0c, `ldmdb` at 0x4eb14).
  Selector-specific results and reset-source labels remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4eaa0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x4eaa4 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0x4eab0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4eb08 | load | 4 | `abs 0x4eb2c` | `-` |
| `str` at 0x4eb0c | store | 4 | `sp -0x18` | `0x10b128` |
| `ldr` at 0x4eb10 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x4eb14 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4eb14 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4eb14 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x4eab0 | `r0-1 <= 0x4 (unsigned, 32-bit)` | `r0-1 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4eb14 | `r0` | `0x10b128` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x4eab0).

## `SHUB2_ResetHold`

- `SHUB2_ResetHold`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3af68 to 0x3b128.
  Established. Zero `[0x148cf4]` takes the branch to `0x3afbc`, while nonzero `[[0x148cf0]+8]` takes
  a later branch to `0x3afc8` (`beq` at 0x3af98, `bne` at 0x3afb8). On the latter taken edge, a four-byte store
  writes `0x0` at `[0x148cf0] +0xc` (`str` at 0x3afe8). Two unnamed `0x3b5e0` calls have four unknown
  candidates (`bl` at 0x3b05c, `bl` at 0x3b0fc). Included stack assignments retain `0xffffeffc` and
  `0xffffeffa` in different paths (`str` at 0x3afc0, `str` at 0x3b0b0). Hub reset-hold effects are
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3af78 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3af80 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x3af88 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x3af8c | load | 4 | `abs 0x3b118` | `-` |
| `ldr` at 0x3af90 | load | 4 | `abs 0x148cf4` | `-` |
| `ldr` at 0x3af9c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3afa4 | load | 4 | `abs 0x3b11c` | `-` |
| `ldr` at 0x3afa8 | load | 4 | `abs 0x148cf0` | `-` |
| `ldr` at 0x3afb0 | load | 4 | `[0x148cf0] +0x8` | `-` |
| `ldr` at 0x3afbc | load | 4 | `abs 0x3b120` | `-` |
| `str` at 0x3afc0 | store | 4 | `sp -0x2c` | `0xffffeffc` |
| `ldr` at 0x3afc8 | load | 4 | `abs 0x3b11c` | `-` |
| `ldr` at 0x3afcc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3afd4 | load | 4 | `abs 0x148cf0` | `-` |
| `str` at 0x3afdc | store | 4 | `sp -0x28` | `[0x148cf0]` |
| `ldr` at 0x3afe0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3afe8 | store | 4 | `[0x148cf0] +0xc` | `0x0` |
| `ldr` at 0x3afec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3affc | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x3b000 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3b004 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3b008 | load | 4 | `[sp-40] +0x8` | `-` |
| `ldr` at 0x3b018 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3b01c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3b030 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0x3b038 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3b03c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3b050 | load | 4 | `[sp-40] +0x4` | `-` |
| `str` at 0x3b064 | store | 4 | `sp -0x1c` | `ret(0x3b5e0)` |
| `ldr` at 0x3b068 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3b074 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3b080 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3b084 | store | 4 | `sp -0x20` | `ret(0x3b5e0)` |
| `ldr` at 0x3b088 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3b090 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x3b098 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3b09c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b0a0 | load | 4 | `[sp-40] +0x8` | `-` |
| `ldr` at 0x3b0ac | load | 4 | `abs 0x3b124` | `-` |
| `str` at 0x3b0b0 | store | 4 | `sp -0x20` | `0xffffeffa` |
| `ldr` at 0x3b0b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3b0bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b0d0 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0x3b0d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3b0dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b0f0 | load | 4 | `[sp-40] +0x4` | `-` |
| `str` at 0x3b104 | store | 4 | `sp -0x20` | `ret(0x3b5e0)` |
| `ldr` at 0x3b108 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3b10c | store | 4 | `sp -0x2c` | `[sp-32]` |
| `ldr` at 0x3b110 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x3b114 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3b114 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3b114 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3b05c | `(unknown callee) 0x3b5e0` | `?` | `?` | `?` | `?` |
| `bl` at 0x3b0fc | `(unknown callee) 0x3b5e0` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3af98 | `[0x148cf4] == 0x0` | `[0x148cf4] != 0x0` |
| `bne` at 0x3afb8 | `[[0x148cf0]+8] != 0x0` | `[[0x148cf0]+8] == 0x0` |
| `bne` at 0x3aff4 | `[sp-20] != 0xff` | `[sp-20] == 0xff` |
| `blt` at 0x3b010 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x3b070 | `ret(0x3b5e0) == 0x0` | `ret(0x3b5e0) != 0x0` |
| `bne` at 0x3b07c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `blt` at 0x3b0a8 | `[sp-20] < [[sp-40]+8] (signed, 32-bit)` | `[sp-20] >= [[sp-40]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3b114 | `r0` | `?` |

## `SHUB2_ResetRelease`

- `SHUB2_ResetRelease`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3b128 to 0x3b30c.
  Established. A four-byte assignment writes `0x0` at `[0x148cf0] +0xc` (`str` at 0x3b1a8). Two
  unnamed calls to `0x3b830` have four unknown candidates (`bl` at 0x3b220, `bl` at 0x3b2c0). A
  later four-byte store assigns `0x1` at `[sp-40] +0xc`, followed by unnamed `0x3b30c` with
  `[sp-36]`, `[sp-20]:1`, `[sp-40]`, `[sp-20]:1` (`str` at 0x3b2d8, `bl` at 0x3b2e8). Hub
  reset-release signaling remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3b138 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3b140 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x3b148 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x3b14c | load | 4 | `abs 0x3b2fc` | `-` |
| `ldr` at 0x3b150 | load | 4 | `abs 0x148cf4` | `-` |
| `ldr` at 0x3b15c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3b164 | load | 4 | `abs 0x3b300` | `-` |
| `ldr` at 0x3b168 | load | 4 | `abs 0x148cf0` | `-` |
| `ldr` at 0x3b170 | load | 4 | `[0x148cf0] +0x8` | `-` |
| `ldr` at 0x3b17c | load | 4 | `abs 0x3b304` | `-` |
| `str` at 0x3b180 | store | 4 | `sp -0x2c` | `0xffffeffc` |
| `ldr` at 0x3b188 | load | 4 | `abs 0x3b300` | `-` |
| `ldr` at 0x3b18c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3b194 | load | 4 | `abs 0x148cf0` | `-` |
| `str` at 0x3b19c | store | 4 | `sp -0x28` | `[0x148cf0]` |
| `ldr` at 0x3b1a0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3b1a8 | store | 4 | `[0x148cf0] +0xc` | `0x0` |
| `ldr` at 0x3b1b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3b1c0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x3b1c4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3b1c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3b1cc | load | 4 | `[sp-40] +0x8` | `-` |
| `ldr` at 0x3b1dc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3b1e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3b1f4 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0x3b1fc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3b200 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3b214 | load | 4 | `[sp-40] +0x4` | `-` |
| `str` at 0x3b228 | store | 4 | `sp -0x1c` | `ret(0x3b830)` |
| `ldr` at 0x3b22c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3b238 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3b244 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3b248 | store | 4 | `sp -0x20` | `ret(0x3b830)` |
| `ldr` at 0x3b24c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3b254 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x3b25c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3b260 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b264 | load | 4 | `[sp-40] +0x8` | `-` |
| `ldr` at 0x3b270 | load | 4 | `abs 0x3b308` | `-` |
| `str` at 0x3b274 | store | 4 | `sp -0x20` | `0xffffeffa` |
| `ldr` at 0x3b27c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3b280 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b294 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0x3b29c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3b2a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b2b4 | load | 4 | `[sp-40] +0x4` | `-` |
| `str` at 0x3b2c8 | store | 4 | `sp -0x20` | `ret(0x3b830)` |
| `ldr` at 0x3b2d0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3b2d8 | store | 4 | `[sp-40] +0xc` | `0x1` |
| `ldrb` at 0x3b2dc | load | 1 | `sp -0x14` | `-` |
| `ldr` at 0x3b2e0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3b2ec | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3b2f0 | store | 4 | `sp -0x2c` | `[sp-32]` |
| `ldr` at 0x3b2f4 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x3b2f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3b2f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3b2f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3b1ac | `(unknown callee) 0xe3cf8` | `r0` | `0x148cf0` | `[0x148cf0]` | `0x0` |
| `bl` at 0x3b220 | `(unknown callee) 0x3b830` | `?` | `?` | `?` | `?` |
| `bl` at 0x3b2c0 | `(unknown callee) 0x3b830` | `?` | `?` | `?` | `?` |
| `bl` at 0x3b2cc | `(unknown callee) 0xe3d64` | `?` | `?` | `?` | `?` |
| `bl` at 0x3b2e8 | `(unknown callee) 0x3b30c` | `[sp-36]` | `[sp-20]:1` | `[sp-40]` | `[sp-20]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3b158 | `[0x148cf4] == 0x0` | `[0x148cf4] != 0x0` |
| `bne` at 0x3b178 | `[[0x148cf0]+8] != 0x0` | `[[0x148cf0]+8] == 0x0` |
| `bne` at 0x3b1b8 | `[sp-20] != 0xff` | `[sp-20] == 0xff` |
| `blt` at 0x3b1d4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x3b234 | `ret(0x3b830) == 0x0` | `ret(0x3b830) != 0x0` |
| `bne` at 0x3b240 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `blt` at 0x3b26c | `[sp-20] < [[sp-40]+8] (signed, 32-bit)` | `[sp-20] >= [[sp-40]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3b2f8 | `r0` | `?` |

## `TIOResetHold`

- `TIOResetHold`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x53f1c to 0x54128.
  Established. Nonzero `[0x1490d0]` takes the branch to `0x53f54` (`bne` at 0x53f44). A byte
  assignment on that arm writes `0x0` at absolute `0x1490d4` (`strb` at 0x53f5c). The unnamed
  `0xd73b0` call carries `0x0`, `[[sp-24]]:1`, `0x60`, `0x0` (`bl` at 0x54050). Another unnamed
  `0x593cc` call has `0x0`, `0xff`, `0xff`, unknown (`bl` at 0x540d8). I/O reset-hold effects remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x53f2c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x53f34 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x53f38 | load | 4 | `abs 0x54114` | `-` |
| `ldr` at 0x53f3c | load | 4 | `abs 0x1490d0` | `-` |
| `ldr` at 0x53f48 | load | 4 | `abs 0x54118` | `-` |
| `str` at 0x53f4c | store | 4 | `sp -0x2c` | `0xfffff768` |
| `ldr` at 0x53f54 | load | 4 | `abs 0x5411c` | `-` |
| `strb` at 0x53f5c | store | 1 | `abs 0x1490d4` | `0x0` |
| `str` at 0x53f64 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x53f68 | load | 4 | `abs 0x54120` | `-` |
| `ldrb` at 0x53f6c | load | 1 | `abs 0x2b1620` | `-` |
| `ldr` at 0x53f70 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x53f80 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x53f88 | load | 4 | `abs 0x54124` | `-` |
| `ldr` at 0x53f8c | load | 4 | `abs 0x1490d8` | `-` |
| `ldrb` at 0x53f94 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x53fa0 | load | 4 | `abs 0x54114` | `-` |
| `ldr` at 0x53fa4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x53fb8 | load | 4 | `abs 0x1490d0` | `-` |
| `str` at 0x53fc0 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x53fc4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x53fc8 | load | 1 | `(unknown)` | `-` |
| `str` at 0x53fe0 | store | 4 | `sp -0x20` | `ret(0xd768c)` |
| `ldr` at 0x53fe4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x53ff0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x53ffc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x54000 | store | 4 | `sp -0x1c` | `ret(0xd768c)` |
| `ldr` at 0x54004 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x54014 | store | 4 | `sp -0x20` | `ret(0x569e0)` |
| `ldr` at 0x54018 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x54024 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x54030 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x54034 | store | 4 | `sp -0x1c` | `ret(0x569e0)` |
| `ldr` at 0x54038 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5403c | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0x54058 | store | 4 | `sp -0x20` | `ret(0xd73b0)` |
| `ldr` at 0x5405c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x54068 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x54074 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x54078 | store | 4 | `sp -0x1c` | `ret(0xd73b0)` |
| `ldr` at 0x5407c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x54080 | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0x54098 | store | 4 | `sp -0x20` | `ret(0xd7368)` |
| `ldr` at 0x5409c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x540a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x540b4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x540b8 | store | 4 | `sp -0x1c` | `ret(0xd7368)` |
| `ldr` at 0x540bc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x540c4 | store | 4 | `sp -0x24` | `?` |
| `str` at 0x540e0 | store | 4 | `sp -0x20` | `ret(0x593cc)` |
| `ldr` at 0x540e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x540f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x540fc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x54100 | store | 4 | `sp -0x1c` | `ret(0x593cc)` |
| `ldr` at 0x54104 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x54108 | store | 4 | `sp -0x2c` | `[sp-28]` |
| `ldr` at 0x5410c | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x54110 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x54110 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x54110 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x53fd8 | `(unknown callee) 0xd768c` | `0x0` | `?` | `0x3` | `?` |
| `bl` at 0x5400c | `(unknown callee) 0x569e0` | `[sp-24]` | `0x0` | `?` | `?` |
| `bl` at 0x54050 | `(unknown callee) 0xd73b0` | `0x0` | `[[sp-24]]:1` | `0x60` | `0x0` |
| `bl` at 0x54090 | `(unknown callee) 0xd7368` | `0x0` | `[[sp-24]]:1` | `0x4` | `[[sp-24]]:1` |
| `bl` at 0x540d8 | `(unknown callee) 0x593cc` | `0x0` | `0xff` | `0xff` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x53f44 | `[0x1490d0] != 0x0` | `[0x1490d0] == 0x0` |
| `blt` at 0x53f78 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x53f9c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x53fec | `ret(0xd768c) == 0x0` | `ret(0xd768c) != 0x0` |
| `bne` at 0x53ff8 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x54020 | `ret(0x569e0) == 0x0` | `ret(0x569e0) != 0x0` |
| `bne` at 0x5402c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x54064 | `ret(0xd73b0) == 0x0` | `ret(0xd73b0) != 0x0` |
| `bne` at 0x54070 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x540a4 | `ret(0xd7368) == 0x0` | `ret(0xd7368) != 0x0` |
| `bne` at 0x540b0 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x540ec | `ret(0x593cc) == 0x0` | `ret(0x593cc) != 0x0` |
| `bne` at 0x540f8 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x54110 | `r0` | `?` |

## `TIOResetRelease`

- `TIOResetRelease`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x54128 to 0x54404.
  Established. Included unnamed `0x59e60` calls carry `0x0`, `0x0`, unknown, `ret(0x59d48)`, and
  `0x0`, `0x1`, `[0x2b1620]:1`, unknown (`bl` at 0x541a4, `bl` at 0x542d8). A later unnamed
  `0x596a0` call has `0x0`, `0xff`, `0xff`, `ret(0x59e60)` (`bl` at 0x54308); its zero result takes
  the branch to `0x5432c`, followed by byte `0x1` at absolute `0x1490d4` (`beq` at 0x5431c, `strb`
  at 0x54334). Reset-release state and electrical behavior are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x54138 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x5413c | load | 4 | `abs 0x543f0` | `-` |
| `ldr` at 0x54140 | load | 4 | `abs 0x1490d0` | `-` |
| `ldr` at 0x5414c | load | 4 | `abs 0x543f4` | `-` |
| `str` at 0x54150 | store | 4 | `sp -0x20` | `0xfffff768` |
| `str` at 0x5416c | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x54180 | store | 4 | `sp -0x18` | `ret(0x59d48)` |
| `ldr` at 0x54184 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x54190 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x54194 | store | 4 | `sp -0x20` | `ret(0x59d48)` |
| `str` at 0x541ac | store | 4 | `sp -0x18` | `ret(0x59e60)` |
| `ldr` at 0x541b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x541bc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x541c0 | store | 4 | `sp -0x20` | `ret(0x59e60)` |
| `str` at 0x541cc | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x541d0 | load | 4 | `abs 0x543f8` | `-` |
| `ldrb` at 0x541d4 | load | 1 | `abs 0x2b1620` | `-` |
| `ldr` at 0x541d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x541e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x541f0 | load | 4 | `abs 0x543fc` | `-` |
| `ldr` at 0x541f4 | load | 4 | `abs 0x1490d8` | `-` |
| `ldrb` at 0x541fc | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x5420c | load | 1 | `sp -0x1c` | `-` |
| `str` at 0x5421c | store | 4 | `sp -0x18` | `ret(0x550e4)` |
| `ldr` at 0x54224 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x54230 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x54234 | store | 4 | `sp -0x20` | `[sp-24]` |
| `ldr` at 0x5423c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x54244 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x54250 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x54254 | load | 4 | `abs 0x543f8` | `-` |
| `ldrb` at 0x54258 | load | 1 | `abs 0x2b1620` | `-` |
| `ldr` at 0x5425c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5426c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x54274 | load | 4 | `abs 0x543fc` | `-` |
| `ldr` at 0x54278 | load | 4 | `abs 0x1490d8` | `-` |
| `ldrb` at 0x54280 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x54290 | load | 1 | `sp -0x1c` | `-` |
| `str` at 0x542a0 | store | 4 | `sp -0x18` | `ret(0x55d7c)` |
| `ldr` at 0x542a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x542b4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x542b8 | store | 4 | `sp -0x20` | `[sp-24]` |
| `ldr` at 0x542c0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x542c8 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x542e0 | store | 4 | `sp -0x18` | `ret(0x59e60)` |
| `ldr` at 0x542e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x542f0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x542f4 | store | 4 | `sp -0x20` | `ret(0x59e60)` |
| `str` at 0x54310 | store | 4 | `sp -0x18` | `ret(0x596a0)` |
| `ldr` at 0x54314 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x54320 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x54324 | store | 4 | `sp -0x20` | `ret(0x596a0)` |
| `ldr` at 0x5432c | load | 4 | `abs 0x54400` | `-` |
| `strb` at 0x54334 | store | 1 | `abs 0x1490d4` | `0x1` |
| `str` at 0x5433c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x54340 | load | 4 | `abs 0x543f8` | `-` |
| `ldrb` at 0x54344 | load | 1 | `abs 0x2b1620` | `-` |
| `ldr` at 0x54348 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x54358 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x54360 | load | 4 | `abs 0x543fc` | `-` |
| `ldr` at 0x54364 | load | 4 | `abs 0x1490d8` | `-` |
| `ldrb` at 0x5436c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x54378 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5438c | load | 4 | `abs 0x543f0` | `-` |
| `ldr` at 0x54390 | load | 4 | `abs 0x1490d0` | `-` |
| `ldrb` at 0x54398 | load | 1 | `(unknown)` | `-` |
| `str` at 0x543b4 | store | 4 | `sp -0x18` | `ret(0xd771c)` |
| `ldr` at 0x543b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x543c4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x543c8 | store | 4 | `sp -0x20` | `ret(0xd771c)` |
| `ldr` at 0x543d0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x543d8 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x543e4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x543e8 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x543ec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x543ec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x543ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x54158 | `(unknown callee) 0x3dce4` | `r0` | `r1` | `r2` | `[0x1490d0]` |
| `bl` at 0x54178 | `(unknown callee) 0x59d48` | `0x0` | `?` | `?` | `ret(0x3dce4)` |
| `bl` at 0x541a4 | `(unknown callee) 0x59e60` | `0x0` | `0x0` | `?` | `ret(0x59d48)` |
| `bl` at 0x54208 | `(unknown callee) 0xe3cf8` | `?` | `?` | `?` | `?` |
| `bl` at 0x54214 | `(unknown callee) 0x550e4` | `[sp-28]:1` | `?` | `?` | `[sp-28]:1` |
| `bl` at 0x54220 | `(unknown callee) 0xe3d64` | `ret(0x550e4)` | `?` | `?` | `ret(0x550e4)` |
| `bl` at 0x5428c | `(unknown callee) 0xe3cf8` | `?` | `?` | `?` | `?` |
| `bl` at 0x54298 | `(unknown callee) 0x55d7c` | `[sp-28]:1` | `?` | `?` | `[sp-28]:1` |
| `bl` at 0x542a4 | `(unknown callee) 0xe3d64` | `ret(0x55d7c)` | `?` | `?` | `ret(0x55d7c)` |
| `bl` at 0x542d8 | `(unknown callee) 0x59e60` | `0x0` | `0x1` | `[0x2b1620]:1` | `?` |
| `bl` at 0x54308 | `(unknown callee) 0x596a0` | `0x0` | `0xff` | `0xff` | `ret(0x59e60)` |
| `bl` at 0x543ac | `(unknown callee) 0xd771c` | `0x0` | `?` | `0x2` | `0x2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x54148 | `[0x1490d0] != 0x0` | `[0x1490d0] == 0x0` |
| `bne` at 0x54164 | `ret(0x3dce4) != 0x0` | `ret(0x3dce4) == 0x0` |
| `beq` at 0x5418c | `ret(0x59d48) == 0x0` | `ret(0x59d48) != 0x0` |
| `beq` at 0x541b8 | `ret(0x59e60) == 0x0` | `ret(0x59e60) != 0x0` |
| `blt` at 0x541e0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x54204 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x5422c | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `blt` at 0x54264 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x54288 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x542b0 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0x542ec | `ret(0x59e60) == 0x0` | `ret(0x59e60) != 0x0` |
| `beq` at 0x5431c | `ret(0x596a0) == 0x0` | `ret(0x596a0) != 0x0` |
| `blt` at 0x54350 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x54374 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x543c0 | `ret(0xd771c) == 0x0` | `ret(0xd771c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x543ec | `r0` | `?` |

## `PICResetHold`

- `PICResetHold`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x34698 to 0x34858.
  Established. A nonzero `[0x148c80]` takes the branch to `0x346c8` (`bne` at 0x346b8). Included
  unnamed `0xd7368` calls use `0x0`, `[[sp-24]+2]:1`, `[[sp-24]+3]:1`, `[[sp-24]+3]:1`, and `0x0`,
  `[[sp-24]+4]:1`, `[[sp-24]+5]:1`, `[[sp-24]+5]:1` (`bl` at 0x34778, `bl` at 0x347c8). A byte zero
  is later assigned at absolute `0x148c85` (`strb` at 0x34804). PCI reset-hold effects and record
  meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x346a8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x346ac | load | 4 | `abs 0x34848` | `-` |
| `ldr` at 0x346b0 | load | 4 | `abs 0x148c80` | `-` |
| `ldr` at 0x346bc | load | 4 | `abs 0x3484c` | `-` |
| `str` at 0x346c0 | store | 4 | `sp -0x24` | `0xfffff128` |
| `str` at 0x346cc | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x346d0 | load | 4 | `abs 0x34850` | `-` |
| `ldrb` at 0x346d4 | load | 1 | `abs 0x148c84` | `-` |
| `ldr` at 0x346d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x346e8 | load | 4 | `abs 0x34848` | `-` |
| `ldr` at 0x346ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x34700 | load | 4 | `abs 0x148c80` | `-` |
| `str` at 0x34708 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x3470c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x34710 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x34714 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x34718 | load | 1 | `(unknown)` | `-` |
| `str` at 0x34730 | store | 4 | `sp -0x1c` | `ret(0xd7368)` |
| `ldr` at 0x34734 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x34740 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x34744 | store | 4 | `sp -0x24` | `ret(0xd7368)` |
| `ldr` at 0x3474c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x34750 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0x3475c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x34760 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0x34764 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x34768 | load | 1 | `[sp-24] +0x3` | `-` |
| `str` at 0x34780 | store | 4 | `sp -0x1c` | `ret(0xd7368)` |
| `ldr` at 0x34784 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x34790 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x34794 | store | 4 | `sp -0x24` | `ret(0xd7368)` |
| `ldr` at 0x3479c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x347a0 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x347ac | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x347b0 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x347b4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x347b8 | load | 1 | `[sp-24] +0x5` | `-` |
| `str` at 0x347d0 | store | 4 | `sp -0x1c` | `ret(0xd7368)` |
| `ldr` at 0x347d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x347e0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x347e4 | store | 4 | `sp -0x24` | `ret(0xd7368)` |
| `ldr` at 0x347ec | load | 4 | `sp -0x20` | `-` |
| `str` at 0x347f4 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0x347fc | load | 4 | `abs 0x34854` | `-` |
| `strb` at 0x34804 | store | 1 | `abs 0x148c85` | `0x0` |
| `str` at 0x3481c | store | 4 | `sp -0x1c` | `ret(0x593cc)` |
| `ldr` at 0x34820 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3482c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x34830 | store | 4 | `sp -0x24` | `ret(0x593cc)` |
| `str` at 0x3483c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x34840 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x34844 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x34844 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x34844 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x34728 | `(unknown callee) 0xd7368` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x34778 | `(unknown callee) 0xd7368` | `0x0` | `[[sp-24]+2]:1` | `[[sp-24]+3]:1` | `[[sp-24]+3]:1` |
| `bl` at 0x347c8 | `(unknown callee) 0xd7368` | `0x0` | `[[sp-24]+4]:1` | `[[sp-24]+5]:1` | `[[sp-24]+5]:1` |
| `bl` at 0x34814 | `(unknown callee) 0x593cc` | `0x0` | `0xff` | `0xff` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x346b8 | `[0x148c80] != 0x0` | `[0x148c80] == 0x0` |
| `blt` at 0x346e0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x3473c | `ret(0xd7368) == 0x0` | `ret(0xd7368) != 0x0` |
| `beq` at 0x34758 | `[[sp-24]+2]:1 == 0xff` | `[[sp-24]+2]:1 != 0xff` |
| `beq` at 0x3478c | `ret(0xd7368) == 0x0` | `ret(0xd7368) != 0x0` |
| `beq` at 0x347a8 | `[[sp-24]+4]:1 == 0xff` | `[[sp-24]+4]:1 != 0xff` |
| `beq` at 0x347dc | `ret(0xd7368) == 0x0` | `ret(0xd7368) != 0x0` |
| `beq` at 0x34828 | `ret(0x593cc) == 0x0` | `ret(0x593cc) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x34844 | `r0` | `?` |

## `PICResetRelease`

- `PICResetRelease`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x34858 to 0x34b44.
  Established. The unnamed `0x34d24` call carries `0xa`, unknown, `[0x148c84]:1`, `[sp-28]` (`bl` at
  0x349fc). The prior zero `[sp-28]` branch skips that call (`beq` at 0x349f4, `bl` at 0x349fc).
  A later unnamed `0x596a0` call has `0x0`, `0xff`, `0xff`, `ret(0x59d48)` (`bl` at 0x34af4); its
  zero result takes the branch to `0x34b18`, followed by byte `0x1` at absolute `0x148c85` (`beq` at
  0x34b08, `strb` at 0x34b20). Reset-release signaling remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x34868 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x34870 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x34874 | load | 4 | `abs 0x34b34` | `-` |
| `ldr` at 0x34878 | load | 4 | `abs 0x148c80` | `-` |
| `ldr` at 0x34884 | load | 4 | `abs 0x34b38` | `-` |
| `str` at 0x34888 | store | 4 | `sp -0x2c` | `0xfffff128` |
| `str` at 0x348a4 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0x348bc | store | 4 | `sp -0x20` | `ret(0x59e60)` |
| `ldr` at 0x348c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x348cc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x348d0 | store | 4 | `sp -0x2c` | `ret(0x59e60)` |
| `str` at 0x348dc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x348e0 | load | 4 | `abs 0x34b3c` | `-` |
| `ldrb` at 0x348e4 | load | 1 | `abs 0x148c84` | `-` |
| `ldr` at 0x348e8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x348fc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x34900 | load | 4 | `abs 0x34b34` | `-` |
| `ldr` at 0x34904 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x34918 | load | 4 | `abs 0x148c80` | `-` |
| `str` at 0x34920 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x34924 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x34928 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x34934 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3493c | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x34940 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x34944 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x34948 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3494c | load | 1 | `(unknown)` | `-` |
| `str` at 0x34964 | store | 4 | `sp -0x20` | `ret(0xd731c)` |
| `ldr` at 0x34968 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x34974 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x34978 | store | 4 | `sp -0x2c` | `ret(0xd731c)` |
| `ldr` at 0x34980 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x34984 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x34990 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x34998 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x3499c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x349a0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x349a4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x349a8 | load | 1 | `(unknown)` | `-` |
| `str` at 0x349c0 | store | 4 | `sp -0x20` | `ret(0xd731c)` |
| `ldr` at 0x349c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x349d0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x349d4 | store | 4 | `sp -0x2c` | `ret(0xd731c)` |
| `ldr` at 0x349dc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x349e4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x349ec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x34a04 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x34a08 | load | 4 | `abs 0x34b3c` | `-` |
| `ldrb` at 0x34a0c | load | 1 | `abs 0x148c84` | `-` |
| `ldr` at 0x34a10 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x34a20 | load | 4 | `abs 0x34b34` | `-` |
| `ldr` at 0x34a24 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x34a38 | load | 4 | `abs 0x148c80` | `-` |
| `str` at 0x34a40 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x34a44 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x34a48 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x34a4c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x34a50 | load | 1 | `(unknown)` | `-` |
| `str` at 0x34a68 | store | 4 | `sp -0x20` | `ret(0xd731c)` |
| `ldr` at 0x34a6c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x34a78 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x34a7c | store | 4 | `sp -0x2c` | `ret(0xd731c)` |
| `ldr` at 0x34a84 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x34a8c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `str` at 0x34aa4 | store | 4 | `sp -0x20` | `ret(0x59e60)` |
| `ldr` at 0x34aa8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x34ab4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x34ab8 | store | 4 | `sp -0x2c` | `ret(0x59e60)` |
| `str` at 0x34acc | store | 4 | `sp -0x20` | `ret(0x59d48)` |
| `ldr` at 0x34ad0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x34adc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x34ae0 | store | 4 | `sp -0x2c` | `ret(0x59d48)` |
| `str` at 0x34afc | store | 4 | `sp -0x20` | `ret(0x596a0)` |
| `ldr` at 0x34b00 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x34b0c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x34b10 | store | 4 | `sp -0x2c` | `ret(0x596a0)` |
| `ldr` at 0x34b18 | load | 4 | `abs 0x34b40` | `-` |
| `strb` at 0x34b20 | store | 1 | `abs 0x148c85` | `0x1` |
| `str` at 0x34b28 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x34b2c | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x34b30 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x34b30 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x34b30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x34890 | `(unknown callee) 0x3dce4` | `r0` | `r1` | `r2` | `[0x148c80]` |
| `bl` at 0x348b4 | `(unknown callee) 0x59e60` | `0x0` | `0x0` | `?` | `ret(0x3dce4)` |
| `bl` at 0x3495c | `(unknown callee) 0xd731c` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x349b8 | `(unknown callee) 0xd731c` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x349fc | `(unknown callee) 0x34d24` | `0xa` | `?` | `[0x148c84]:1` | `[sp-28]` |
| `bl` at 0x34a60 | `(unknown callee) 0xd731c` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x34a9c | `(unknown callee) 0x59e60` | `0x0` | `0x1` | `[0x148c84]:1` | `?` |
| `bl` at 0x34ac4 | `(unknown callee) 0x59d48` | `0x0` | `?` | `?` | `ret(0x59e60)` |
| `bl` at 0x34af4 | `(unknown callee) 0x596a0` | `0x0` | `0xff` | `0xff` | `ret(0x59d48)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x34880 | `[0x148c80] != 0x0` | `[0x148c80] == 0x0` |
| `bne` at 0x3489c | `ret(0x3dce4) != 0x0` | `ret(0x3dce4) == 0x0` |
| `beq` at 0x348c8 | `ret(0x59e60) == 0x0` | `ret(0x59e60) != 0x0` |
| `blt` at 0x348f0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x34930 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x34970 | `ret(0xd731c) == 0x0` | `ret(0xd731c) != 0x0` |
| `beq` at 0x3498c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x349cc | `ret(0xd731c) == 0x0` | `ret(0xd731c) != 0x0` |
| `beq` at 0x349f4 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `blt` at 0x34a18 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x34a74 | `ret(0xd731c) == 0x0` | `ret(0xd731c) != 0x0` |
| `beq` at 0x34ab0 | `ret(0x59e60) == 0x0` | `ret(0x59e60) != 0x0` |
| `beq` at 0x34ad8 | `ret(0x59d48) == 0x0` | `ret(0x59d48) != 0x0` |
| `beq` at 0x34b08 | `ret(0x596a0) == 0x0` | `ret(0x596a0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x34b30 | `r0` | `?` |

## `IP57_ResetHold`

- `IP57_ResetHold`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x60c50 to 0x60ef4.
  Established. An unknown-target call has candidates unknown, unknown, `[0x14385c]`, unknown (`mov`
  at 0x60ce8). Included unnamed `0xd73b0` calls carry `0x0`, `[sp-29]:1`, `0xc`, `0x8`, and `0x0`,
  `[sp-29]:1`, `0xc0`, `0x0` (`bl` at 0x60d80, `bl` at 0x60e88). The latter zero result takes the
  branch to `0x60eac` (`beq` at 0x60e9c). Reset-hold register and callback meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x60c64 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x60c68 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x60c80 | store | 4 | `sp -0x1c` | `ret(0x617b4)` |
| `ldr` at 0x60c84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x60c90 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x60c94 | store | 4 | `sp -0x24` | `ret(0x617b4)` |
| `ldr` at 0x60c9c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x60ca0 | load | 1 | `[sp-24] +0x2` | `-` |
| `strb` at 0x60ca8 | store | 1 | `sp -0x1d` | `([[sp-24]+2]:1+2 & 0xff)` |
| `ldr` at 0x60cac | load | 4 | `abs 0x60ef0` | `-` |
| `ldr` at 0x60cb0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x60cb4 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x60cc8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x60ccc | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x60cd8 | load | 4 | `abs 0x14385c` | `-` |
| `ldr` at 0x60d34 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x60d38 | store | 4 | `sp -0x24` | `[sp-28]` |
| `ldrb` at 0x60d40 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x60d50 | store | 4 | `sp -0x1c` | `ret(0x4d770)` |
| `ldr` at 0x60d54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x60d60 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x60d64 | store | 4 | `sp -0x24` | `ret(0x4d770)` |
| `ldrb` at 0x60d6c | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x60d88 | store | 4 | `sp -0x1c` | `ret(0xd73b0)` |
| `ldr` at 0x60d8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x60d98 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x60d9c | store | 4 | `sp -0x24` | `ret(0xd73b0)` |
| `ldrb` at 0x60da4 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x60dbc | store | 4 | `sp -0x1c` | `ret(0xd731c)` |
| `ldr` at 0x60dc0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x60dcc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x60dd0 | store | 4 | `sp -0x24` | `ret(0xd731c)` |
| `strb` at 0x60ddc | store | 1 | `sp -0x1e` | `0x0` |
| `ldrb` at 0x60de0 | load | 1 | `sp -0x1e` | `-` |
| `ldrb` at 0x60df0 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x60dfc | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x60e1c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x60e20 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0x60e24 | load | 1 | `sp -0x1e` | `-` |
| `str` at 0x60e48 | store | 4 | `sp -0x1c` | `ret(0xd7368)` |
| `ldr` at 0x60e4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x60e58 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x60e5c | store | 4 | `sp -0x24` | `ret(0xd7368)` |
| `ldrb` at 0x60e64 | load | 1 | `sp -0x1e` | `-` |
| `strb` at 0x60e6c | store | 1 | `sp -0x1e` | `([sp-30]:1+1 & 0xff)` |
| `ldrb` at 0x60e74 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x60e90 | store | 4 | `sp -0x1c` | `ret(0xd73b0)` |
| `ldr` at 0x60e94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x60ea0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x60ea4 | store | 4 | `sp -0x24` | `ret(0xd73b0)` |
| `ldrb` at 0x60eac | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x60ec4 | store | 4 | `sp -0x1c` | `ret(0xd7368)` |
| `ldr` at 0x60ec8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x60ed4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x60ed8 | store | 4 | `sp -0x24` | `ret(0xd7368)` |
| `str` at 0x60ee4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x60ee8 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x60eec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x60eec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x60eec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x60c78 | `(unknown callee) 0x617b4` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `mov` at 0x60ce8 | `(unknown callee) (unknown target)` | `?` | `?` | `[0x14385c]` | `?` |
| `bl` at 0x60cf0 | `(unknown callee) 0x4e054` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x60cf8 | `(unknown callee) 0x61854` | `0xa` | `?` | `?` | `?` |
| `bl` at 0x60cfc | `(unknown callee) 0x3dce4` | `ret(0x61854)` | `?` | `?` | `?` |
| `bl` at 0x60d0c | `(unknown callee) 0x4f4d0` | `ret(0x3dce4)` | `?` | `?` | `ret(0x3dce4)` |
| `bl` at 0x60d24 | `(unknown callee) 0xe5ccc` | `?` | `sp-28` | `sp-28` | `?` |
| `bl` at 0x60d48 | `(unknown callee) 0x4d770` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x60d80 | `(unknown callee) 0xd73b0` | `0x0` | `[sp-29]:1` | `0xc` | `0x8` |
| `bl` at 0x60db4 | `(unknown callee) 0xd731c` | `0x0` | `[sp-29]:1` | `0x10` | `[sp-29]:1` |
| `bl` at 0x60e0c | `(unknown callee) 0x5e14c` | `?` | `?` | `?` | `?` |
| `bl` at 0x60e40 | `(unknown callee) 0xd7368` | `0x0` | `?` | `0x4` | `?` |
| `bl` at 0x60e88 | `(unknown callee) 0xd73b0` | `0x0` | `[sp-29]:1` | `0xc0` | `0x0` |
| `bl` at 0x60ebc | `(unknown callee) 0xd7368` | `0x0` | `[sp-29]:1` | `0x2` | `[sp-29]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x60c8c | `ret(0x617b4) == 0x0` | `ret(0x617b4) != 0x0` |
| `beq` at 0x60d08 | `ret(0x3dce4) == 0x0` | `ret(0x3dce4) != 0x0` |
| `bne` at 0x60d30 | `ret(0xe5ccc) != 0x0` | `ret(0xe5ccc) == 0x0` |
| `beq` at 0x60d5c | `ret(0x4d770) == 0x0` | `ret(0x4d770) != 0x0` |
| `beq` at 0x60d94 | `ret(0xd73b0) == 0x0` | `ret(0xd73b0) != 0x0` |
| `beq` at 0x60dc8 | `ret(0xd731c) == 0x0` | `ret(0xd731c) != 0x0` |
| `bls` at 0x60de8 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x60e18 | `ret(0x5e14c) == 0x0` | `ret(0x5e14c) != 0x0` |
| `beq` at 0x60e54 | `ret(0xd7368) == 0x0` | `ret(0xd7368) != 0x0` |
| `beq` at 0x60e9c | `ret(0xd73b0) == 0x0` | `ret(0xd73b0) != 0x0` |
| `beq` at 0x60ed0 | `ret(0xd7368) == 0x0` | `ret(0xd7368) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x60eec | `r0` | `?` |

## `IP57_ResetRelease`

- `IP57_ResetRelease`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x60ef4 to 0x612f8.
  Established. The unnamed `0xdd33c` calls have `0x1`, `0x0`, `sp-112`, `sp-112` at two locations
  (`bl` at 0x610ac, `bl` at 0x612bc). Another unnamed `0x61854` call carries `[0x1434a0]`, unknown,
  unknown, `0x1434a0` (`bl` at 0x611cc). The later nonzero `ret(0x4dcdc)` branch reaches `0x6120c`
  (`bl` at 0x611f0, `bne` at 0x61204). Included four-byte stack assignments at `sp -0x78` retain `ret(0x617b4)` and
  `0x0` in different paths (`str` at 0x60f38, `str` at 0x612cc). Release sequencing, delay units and
  callback effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x60f08 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x60f0c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x60f24 | store | 4 | `sp -0x1c` | `ret(0x617b4)` |
| `ldr` at 0x60f28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x60f34 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x60f38 | store | 4 | `sp -0x78` | `ret(0x617b4)` |
| `ldr` at 0x60f40 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x60f44 | load | 1 | `[sp-24] +0x2` | `-` |
| `strb` at 0x60f4c | store | 1 | `sp -0x1d` | `([[sp-24]+2]:1+2 & 0xff)` |
| `ldr` at 0x60f50 | load | 4 | `abs 0x612d8` | `-` |
| `ldr` at 0x60f54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x60f58 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x60f6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x60f70 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x60f7c | load | 4 | `abs 0x14385c` | `-` |
| `ldrb` at 0x60f90 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x60fa0 | store | 4 | `sp -0x1c` | `ret(0x4d794)` |
| `ldr` at 0x60fa4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x60fb0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x60fb4 | store | 4 | `sp -0x78` | `ret(0x4d794)` |
| `ldrb` at 0x60fbc | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x60fd4 | store | 4 | `sp -0x1c` | `ret(0xd731c)` |
| `ldr` at 0x60fd8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x60fe4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x60fe8 | store | 4 | `sp -0x78` | `ret(0xd731c)` |
| `ldrb` at 0x60ff8 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x61010 | store | 4 | `sp -0x1c` | `ret(0xd7368)` |
| `ldr` at 0x61014 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61020 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x61024 | store | 4 | `sp -0x78` | `ret(0xd7368)` |
| `ldrb` at 0x61034 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x6104c | store | 4 | `sp -0x1c` | `ret(0xd731c)` |
| `ldr` at 0x61050 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6105c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x61060 | store | 4 | `sp -0x78` | `ret(0xd731c)` |
| `ldrb` at 0x61070 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x61080 | store | 4 | `sp -0x1c` | `ret(0x4d2f0)` |
| `ldr` at 0x61084 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61090 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x61094 | store | 4 | `sp -0x78` | `ret(0x4d2f0)` |
| `ldrb` at 0x610b0 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x610c8 | store | 4 | `sp -0x1c` | `ret(0xd731c)` |
| `ldr` at 0x610cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x610d8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x610dc | store | 4 | `sp -0x78` | `ret(0xd731c)` |
| `strb` at 0x610f0 | store | 1 | `sp -0x1e` | `0x0` |
| `ldrb` at 0x610f4 | load | 1 | `sp -0x1e` | `-` |
| `ldrb` at 0x61104 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x61110 | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x61130 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x61134 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0x61138 | load | 1 | `sp -0x1e` | `-` |
| `str` at 0x6115c | store | 4 | `sp -0x1c` | `ret(0xd731c)` |
| `ldr` at 0x61160 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6116c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x61170 | store | 4 | `sp -0x78` | `ret(0xd731c)` |
| `ldrb` at 0x61178 | load | 1 | `sp -0x1e` | `-` |
| `strb` at 0x61180 | store | 1 | `sp -0x1e` | `([sp-30]:1+1 & 0xff)` |
| `ldrb` at 0x61190 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x611a8 | store | 4 | `sp -0x1c` | `ret(0xd731c)` |
| `ldr` at 0x611ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x611b8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x611bc | store | 4 | `sp -0x78` | `ret(0xd731c)` |
| `ldr` at 0x611c4 | load | 4 | `abs 0x612dc` | `-` |
| `ldr` at 0x611c8 | load | 4 | `abs 0x1434a0` | `-` |
| `str` at 0x611d4 | store | 4 | `sp -0x74` | `0x0` |
| `ldr` at 0x611d8 | load | 4 | `sp -0x74` | `-` |
| `ldrb` at 0x611e8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x611f8 | store | 4 | `sp -0x1c` | `ret(0x4dcdc)` |
| `ldr` at 0x611fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61214 | load | 4 | `sp -0x74` | `-` |
| `str` at 0x6121c | store | 4 | `sp -0x74` | `[sp-116]+1` |
| `ldr` at 0x61224 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61230 | load | 4 | `abs 0x612e0` | `-` |
| `ldr` at 0x61234 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x61238 | load | 4 | `abs 0x148d80` | `-` |
| `ldr` at 0x61248 | load | 4 | `abs 0x612e4` | `-` |
| `str` at 0x6124c | store | 4 | `sp -0x88` | `0x10d154` |
| `ldr` at 0x61250 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6125c | load | 4 | `abs 0x612e8` | `-` |
| `str` at 0x61260 | store | 4 | `sp -0x7c` | `0x10d184` |
| `ldr` at 0x61268 | load | 4 | `abs 0x612ec` | `-` |
| `str` at 0x6126c | store | 4 | `sp -0x7c` | `0x10d18c` |
| `ldr` at 0x61270 | load | 4 | `sp -0x7c` | `-` |
| `str` at 0x61274 | store | 4 | `sp -0x84` | `[sp-124]` |
| `ldr` at 0x61278 | load | 4 | `sp -0x74` | `-` |
| `str` at 0x6127c | store | 4 | `sp -0x80` | `?` |
| `ldr` at 0x61284 | load | 4 | `abs 0x612f0` | `-` |
| `ldr` at 0x61288 | load | 4 | `abs 0x612f4` | `-` |
| `ldr` at 0x61294 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x612a0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x612a4 | store | 4 | `sp -0x78` | `[sp-28]` |
| `str` at 0x612cc | store | 4 | `sp -0x78` | `0x0` |
| `ldr` at 0x612d0 | load | 4 | `sp -0x78` | `-` |
| `ldmdb` at 0x612d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x612d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x612d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x60f1c | `(unknown callee) 0x617b4` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `mov` at 0x60f8c | `(unknown callee) (unknown target)` | `?` | `?` | `[0x14385c]` | `?` |
| `bl` at 0x60f98 | `(unknown callee) 0x4d794` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x60fcc | `(unknown callee) 0xd731c` | `0x0` | `[sp-29]:1` | `0x2` | `[sp-29]:1` |
| `bl` at 0x60ff4 | `(unknown callee) 0x61854` | `0xa` | `?` | `?` | `ret(0xd731c)` |
| `bl` at 0x61008 | `(unknown callee) 0xd7368` | `0x0` | `[sp-29]:1` | `0x2` | `[sp-29]:1` |
| `bl` at 0x61030 | `(unknown callee) 0x61854` | `0xa` | `?` | `?` | `ret(0xd7368)` |
| `bl` at 0x61044 | `(unknown callee) 0xd731c` | `0x0` | `[sp-29]:1` | `0x2` | `[sp-29]:1` |
| `bl` at 0x6106c | `(unknown callee) 0x61854` | `0x15` | `?` | `?` | `ret(0xd731c)` |
| `bl` at 0x61078 | `(unknown callee) 0x4d2f0` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x610ac | `(unknown callee) 0xdd33c` | `0x1` | `0x0` | `sp-112` | `sp-112` |
| `bl` at 0x610c0 | `(unknown callee) 0xd731c` | `0x0` | `[sp-29]:1` | `0xc0` | `[sp-29]:1` |
| `bl` at 0x610e8 | `(unknown callee) 0x61854` | `0x3` | `?` | `?` | `ret(0xd731c)` |
| `bl` at 0x61120 | `(unknown callee) 0x5e14c` | `?` | `?` | `?` | `?` |
| `bl` at 0x61154 | `(unknown callee) 0xd731c` | `0x0` | `?` | `0x4` | `?` |
| `bl` at 0x6118c | `(unknown callee) 0x61854` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x611a0 | `(unknown callee) 0xd731c` | `0x0` | `[sp-29]:1` | `0x4` | `[sp-29]:1` |
| `bl` at 0x611cc | `(unknown callee) 0x61854` | `[0x1434a0]` | `?` | `?` | `0x1434a0` |
| `bl` at 0x611f0 | `(unknown callee) 0x4dcdc` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x61210 | `(unknown callee) 0x61854` | `0xa` | `?` | `?` | `ret(0x4dcdc)` |
| `bl` at 0x61290 | `(unknown callee) 0x21eb8` | `0x4` | `0x10d08c` | `0x3fb` | `0x0` |
| `bl` at 0x612bc | `(unknown callee) 0xdd33c` | `0x1` | `0x0` | `sp-112` | `sp-112` |
| `bl` at 0x612c4 | `(unknown callee) 0x4e054` | `0x1` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x60f30 | `ret(0x617b4) == 0x0` | `ret(0x617b4) != 0x0` |
| `beq` at 0x60fac | `ret(0x4d794) == 0x0` | `ret(0x4d794) != 0x0` |
| `beq` at 0x60fe0 | `ret(0xd731c) == 0x0` | `ret(0xd731c) != 0x0` |
| `beq` at 0x6101c | `ret(0xd7368) == 0x0` | `ret(0xd7368) != 0x0` |
| `beq` at 0x61058 | `ret(0xd731c) == 0x0` | `ret(0xd731c) != 0x0` |
| `beq` at 0x6108c | `ret(0x4d2f0) == 0x0` | `ret(0x4d2f0) != 0x0` |
| `beq` at 0x610d4 | `ret(0xd731c) == 0x0` | `ret(0xd731c) != 0x0` |
| `bls` at 0x610fc | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x6112c | `ret(0x5e14c) == 0x0` | `ret(0x5e14c) != 0x0` |
| `beq` at 0x61168 | `ret(0xd731c) == 0x0` | `ret(0xd731c) != 0x0` |
| `beq` at 0x611b4 | `ret(0xd731c) == 0x0` | `ret(0xd731c) != 0x0` |
| `ble` at 0x611e0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x61204 | `ret(0x4dcdc) != 0x0` | `ret(0x4dcdc) == 0x0` |
| `bne` at 0x6122c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bgt` at 0x61240 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x61258 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `beq` at 0x6129c | `[sp-28] == 0x0` | `[sp-28] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x612d4 | `r0` | `?` |

## `IP57_SoftReset`

- `IP57_SoftReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x612f8 to 0x613d4.
  Established. The unnamed `0xd7368` call carries `0x0`, `([[sp-24]+2]:1+2 & 0xff)`, `0x8`,
  `([[sp-24]+2]:1+2 & 0xff)` (`bl` at 0x61364); its zero result takes the branch to `0x61388` (`beq`
  at 0x61378). The following unnamed `0x61854` call uses `0xa`, unknown, unknown, `ret(0xd7368)`
  (`bl` at 0x6138c). A later unnamed `0xd731c` call has `0x0`, `[sp-25]:1`, `0x8`, `[sp-25]:1` (`bl`
  at 0x613a0). Soft-reset line and timing meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x6130c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x61310 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x61328 | store | 4 | `sp -0x20` | `ret(0x617b4)` |
| `ldr` at 0x6132c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x61338 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x6133c | store | 4 | `sp -0x24` | `ret(0x617b4)` |
| `ldr` at 0x61344 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x61348 | load | 1 | `[sp-24] +0x2` | `-` |
| `strb` at 0x61350 | store | 1 | `sp -0x19` | `([[sp-24]+2]:1+2 & 0xff)` |
| `ldrb` at 0x61354 | load | 1 | `sp -0x19` | `-` |
| `str` at 0x6136c | store | 4 | `sp -0x20` | `ret(0xd7368)` |
| `ldr` at 0x61370 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6137c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x61380 | store | 4 | `sp -0x24` | `ret(0xd7368)` |
| `ldrb` at 0x61390 | load | 1 | `sp -0x19` | `-` |
| `str` at 0x613a8 | store | 4 | `sp -0x20` | `ret(0xd731c)` |
| `ldr` at 0x613ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x613b8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x613bc | store | 4 | `sp -0x24` | `ret(0xd731c)` |
| `str` at 0x613c8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x613cc | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x613d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x613d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x613d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x61320 | `(unknown callee) 0x617b4` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x61364 | `(unknown callee) 0xd7368` | `0x0` | `([[sp-24]+2]:1+2 & 0xff)` | `0x8` | `([[sp-24]+2]:1+2 & 0xff)` |
| `bl` at 0x6138c | `(unknown callee) 0x61854` | `0xa` | `?` | `?` | `ret(0xd7368)` |
| `bl` at 0x613a0 | `(unknown callee) 0xd731c` | `0x0` | `[sp-25]:1` | `0x8` | `[sp-25]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x61334 | `ret(0x617b4) == 0x0` | `ret(0x617b4) != 0x0` |
| `beq` at 0x61378 | `ret(0xd7368) == 0x0` | `ret(0xd7368) != 0x0` |
| `beq` at 0x613b4 | `ret(0xd731c) == 0x0` | `ret(0xd731c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x613d0 | `r0` | `?` |

## `NL4R_ResetHold`

- `NL4R_ResetHold`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x27a0c to 0x27b3c.
  Established. A nonzero byte `[0x148bd8]:1` takes the branch to `0x27a44`, followed by byte `0x0`
  at absolute `0x148bd9` (`bne` at 0x27a34, `strb` at 0x27a4c). The signed-less branch takes
  `[sp-20] < [0x148be0]` to `0x27afc` (`blt` at 0x27aec). Both unnamed `0x2802c` calls carry
  unknown, `0x148bdc`, unknown, unknown (`bl` at 0x27a94, `bl` at 0x27b14). Reset-hold state and
  device effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x27a1c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x27a24 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x27a28 | load | 4 | `abs 0x27b28` | `-` |
| `ldrb` at 0x27a2c | load | 1 | `abs 0x148bd8` | `-` |
| `ldr` at 0x27a38 | load | 4 | `abs 0x27b2c` | `-` |
| `str` at 0x27a3c | store | 4 | `sp -0x20` | `0xfffffed4` |
| `ldr` at 0x27a44 | load | 4 | `abs 0x27b30` | `-` |
| `strb` at 0x27a4c | store | 1 | `abs 0x148bd9` | `0x0` |
| `ldr` at 0x27a50 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27a60 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x27a64 | load | 4 | `abs 0x27b34` | `-` |
| `ldr` at 0x27a68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27a6c | load | 4 | `abs 0x148be0` | `-` |
| `ldr` at 0x27a7c | load | 4 | `abs 0x27b38` | `-` |
| `ldr` at 0x27a80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27a88 | load | 4 | `abs 0x148bdc` | `-` |
| `str` at 0x27a9c | store | 4 | `sp -0x1c` | `ret(0x2802c)` |
| `ldr` at 0x27aa0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x27aac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x27ab8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x27abc | store | 4 | `sp -0x18` | `ret(0x2802c)` |
| `ldr` at 0x27ac0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27ac8 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0x27ad0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x27ad4 | store | 4 | `sp -0x20` | `[sp-24]` |
| `ldr` at 0x27adc | load | 4 | `abs 0x27b34` | `-` |
| `ldr` at 0x27ae0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27ae4 | load | 4 | `abs 0x148be0` | `-` |
| `str` at 0x27af4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x27afc | load | 4 | `abs 0x27b38` | `-` |
| `ldr` at 0x27b00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27b08 | load | 4 | `abs 0x148bdc` | `-` |
| `str` at 0x27b1c | store | 4 | `sp -0x20` | `ret(0x2802c)` |
| `ldr` at 0x27b20 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x27b24 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x27b24 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x27b24 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x27a94 | `(unknown callee) 0x2802c` | `?` | `0x148bdc` | `?` | `?` |
| `bl` at 0x27b14 | `(unknown callee) 0x2802c` | `?` | `0x148bdc` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x27a34 | `[0x148bd8]:1 != 0x0` | `[0x148bd8]:1 == 0x0` |
| `bne` at 0x27a58 | `[sp-20] != 0xff` | `[sp-20] == 0xff` |
| `blt` at 0x27a74 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x27aa8 | `ret(0x2802c) == 0x0` | `ret(0x2802c) != 0x0` |
| `bne` at 0x27ab4 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `blt` at 0x27aec | `[sp-20] < [0x148be0] (signed, 32-bit)` | `[sp-20] >= [0x148be0] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x27b24 | `r0` | `?` |
