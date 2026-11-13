# ARM flash image and peripheral reset paths in 1.38.4

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

[Fact sheets and full listings](disasm/arm-1.38.4-package4/flash-image-reset-paths-facts.txt) retain all tool sections.

## `flash_init`

- `flash_init`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcae70 to 0xcaf90.
  Established. The word stores write `ret(0x1b3d0)` at `0x142a84` and `0x1` at `0x2ad1d4` (`str` at
  0xcaef8, `str` at 0xcaf64). The call to `0x1b3d0` has `0x124f0e5`, `0x1`, `0x3b6`, `[0x142a84]`
  (`bl` at 0xcaeec). The signed test takes its bypass for `[0x142a84] >= 0x0` and falls through when
  negative (`bge` at 0xcaf04). Flash initialization semantics and r0 at exit remain unknown (`ldmdb`
  at 0xcaf74).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcae80 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xcae84 | load | 4 | `abs 0xcaf78` | `-` |
| `ldr` at 0xcae88 | load | 4 | `abs 0x2ad1d4` | `-` |
| `str` at 0xcae98 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xcaea4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xcaea8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcaeb8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcaec0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcaec8 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xcaed0 | load | 4 | `abs 0xcaf7c` | `-` |
| `ldr` at 0xcaed4 | load | 4 | `abs 0x142a84` | `-` |
| `ldr` at 0xcaee0 | load | 4 | `abs 0xcaf80` | `-` |
| `ldr` at 0xcaee8 | load | 4 | `abs 0xcaf84` | `-` |
| `ldr` at 0xcaef4 | load | 4 | `abs 0xcaf7c` | `-` |
| `str` at 0xcaef8 | store | 4 | `abs 0x142a84` | `ret(0x1b3d0)` |
| `ldr` at 0xcaefc | load | 4 | `abs 0x142a84` | `-` |
| `ldr` at 0xcaf1c | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xcaf28 | load | 4 | `abs 0xcaf88` | `-` |
| `str` at 0xcaf2c | store | 4 | `sp -0x2c` | `0x133970` |
| `ldr` at 0xcaf30 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `str` at 0xcaf34 | store | 4 | `sp -0x28` | `[ret(0x1b0b8)]` |
| `str` at 0xcaf38 | store | 4 | `sp -0x24` | `ret(0x1b094)` |
| `ldr` at 0xcaf40 | load | 4 | `abs 0xcaf8c` | `-` |
| `str` at 0xcaf54 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcaf5c | load | 4 | `abs 0xcaf78` | `-` |
| `str` at 0xcaf64 | store | 4 | `abs 0x2ad1d4` | `0x1` |
| `str` at 0xcaf6c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xcaf70 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xcaf74 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xcaf74 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcaf74 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcaf74 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcaebc | `(unknown callee) 0xca9d8` | `?` | `?` | `?` | `?` |
| `bl` at 0xcaeec | `(unknown callee) 0x1b3d0` | `0x124f0e5` | `0x1` | `0x3b6` | `[0x142a84]` |
| `bl` at 0xcaf08 | `(unknown callee) 0x1b0b8` | `ret(0x1b3d0)` | `?` | `ret(0x1b3d0)` | `[0x142a84]` |
| `bl` at 0xcaf14 | `(unknown callee) 0x1b0b8` | `ret(0x1b0b8)` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0xcaf20 | `(unknown callee) 0x1b094` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0xcaf4c | `(unknown callee) 0x21e28` | `0x3` | `0x133850` | `0xd9` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xcae90 | `[0x2ad1d4] == 0x0` | `[0x2ad1d4] != 0x0` |
| `ble` at 0xcaeb0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bge` at 0xcaedc | `[0x142a84] >= 0x0 (signed, 32-bit)` | `[0x142a84] < 0x0 (signed, 32-bit)` |
| `bge` at 0xcaf04 | `[0x142a84] >= 0x0 (signed, 32-bit)` | `[0x142a84] < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcaf74 | `r0` | `?` |

## `flash_addr_to_index`

- `flash_addr_to_index`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcb0f8 to 0xcb190.
  Established. The call to `0xcae70` carries `0x0`, entry r1, entry r2, `[0x2ad1d4]` (`bl` at
  0xcb120). A later signed branch takes its alternate arm for `[sp-24] > 0x2` and falls through for
  less or equal (`bgt` at 0xcb168). The return reports r0 `[sp-32]` (`ldmdb` at 0xcb188).
  Flash-index encoding is not established by the resolved expressions.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcb108 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xcb10c | load | 4 | `abs 0xcb18c` | `-` |
| `ldr` at 0xcb110 | load | 4 | `abs 0x2ad1d4` | `-` |
| `str` at 0xcb128 | store | 4 | `sp -0x1c` | `ret(0xcae70)` |
| `ldr` at 0xcb12c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcb138 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcb13c | store | 4 | `sp -0x20` | `ret(0xcae70)` |
| `ldr` at 0xcb144 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcb150 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xcb154 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcb160 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcb174 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcb17c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcb180 | store | 4 | `sp -0x20` | `[sp-24]` |
| `ldr` at 0xcb184 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xcb188 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcb188 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcb188 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcb120 | `(unknown callee) 0xcae70` | `0x0` | `r1` | `r2` | `[0x2ad1d4]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcb118 | `[0x2ad1d4] != 0x0` | `[0x2ad1d4] == 0x0` |
| `bge` at 0xcb134 | `ret(0xcae70) >= 0x0 (signed, 32-bit)` | `ret(0xcae70) < 0x0 (signed, 32-bit)` |
| `blt` at 0xcb15c | `[sp-24] < 0x0 (signed, 32-bit)` | `[sp-24] >= 0x0 (signed, 32-bit)` |
| `bgt` at 0xcb168 | `[sp-24] > 0x2 (signed, 32-bit)` | `[sp-24] <= 0x2 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcb188 | `r0` | `[sp-32]` |

## `flash_program`

- `flash_program`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcb48c to 0xcb8ec.
  Established. The call to `0x1b40c` has unknown, `0xc00c4d04`, `sp-92`, `[sp-40]` (`bl` at
  0xcb604); its zero result takes the continuation and nonzero falls through (`beq` at 0xcb610). A
  later word store writes `[sp-20]` at `[sp-32] +0x0` (`str` at 0xcb88c). Programming command
  meaning and r0 at the merged exit are unresolved (`ldmdb` at 0xcb8a0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcb49c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcb4a0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcb4a4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xcb4a8 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xcb4b0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xcb4b4 | load | 4 | `abs 0xcb8a4` | `-` |
| `ldr` at 0xcb4b8 | load | 4 | `abs 0x2ad1d4` | `-` |
| `str` at 0xcb4d0 | store | 4 | `sp -0x40` | `ret(0xcae70)` |
| `ldr` at 0xcb4d4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xcb4e0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xcb4e4 | store | 4 | `sp -0x60` | `ret(0xcae70)` |
| `ldr` at 0xcb4f0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcb4fc | store | 4 | `sp -0x28` | `ret(0xcb0f8)` |
| `ldr` at 0xcb500 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcb50c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcb518 | store | 4 | `sp -0x30` | `ret(0xcb330)` |
| `ldr` at 0xcb51c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcb52c | load | 4 | `abs 0xcb8a8` | `-` |
| `str` at 0xcb530 | store | 4 | `sp -0x2c` | `0xfffffc7c` |
| `ldr` at 0xcb538 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcb544 | store | 4 | `sp -0x34` | `ret(0xcb190)` |
| `ldr` at 0xcb548 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcb554 | load | 4 | `abs 0xcb8ac` | `-` |
| `ldr` at 0xcb558 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xcb560 | load | 4 | `abs 0x2b36c0` | `-` |
| `ldr` at 0xcb574 | load | 4 | `abs 0xcb8b0` | `-` |
| `str` at 0xcb578 | store | 4 | `sp -0x70` | `0x133a68` |
| `ldr` at 0xcb57c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcb580 | store | 4 | `sp -0x6c` | `[sp-20]` |
| `ldr` at 0xcb588 | load | 4 | `abs 0xcb8b4` | `-` |
| `ldr` at 0xcb58c | load | 4 | `abs 0xcb8b8` | `-` |
| `str` at 0xcb59c | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xcb5a8 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xcb5ac | load | 4 | `abs 0xcb8ac` | `-` |
| `ldr` at 0xcb5b0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xcb5b4 | load | 4 | `abs 0x2b36c0` | `-` |
| `ldr` at 0xcb5bc | load | 4 | `sp -0x40` | `-` |
| `str` at 0xcb5d0 | store | 4 | `sp -0x5c` | `0x0` |
| `str` at 0xcb5d8 | store | 4 | `sp -0x58` | `0x10` |
| `str` at 0xcb5e0 | store | 4 | `sp -0x54` | `sp-80` |
| `ldr` at 0xcb5e4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xcb5e8 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0xcb5ec | load | 4 | `abs 0xcb8bc` | `-` |
| `ldr` at 0xcb5f0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcb5f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcb5fc | load | 4 | `abs 0xcb8c0` | `-` |
| `ldr` at 0xcb61c | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xcb628 | load | 4 | `abs 0xcb8c4` | `-` |
| `str` at 0xcb62c | store | 4 | `sp -0x70` | `0x133aa4` |
| `ldr` at 0xcb630 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xcb634 | store | 4 | `sp -0x6c` | `[sp-64]` |
| `str` at 0xcb638 | store | 4 | `sp -0x68` | `ret(0x21674)` |
| `ldr` at 0xcb640 | load | 4 | `abs 0xcb8b4` | `-` |
| `ldr` at 0xcb644 | load | 4 | `abs 0xcb8c8` | `-` |
| `str` at 0xcb654 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0xcb65c | load | 1 | `sp -0x4b` | `-` |
| `ldr` at 0xcb668 | load | 4 | `abs 0xcb8ac` | `-` |
| `ldr` at 0xcb66c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xcb670 | load | 4 | `abs 0x2b36c0` | `-` |
| `str` at 0xcb678 | store | 4 | `sp -0x34` | `([sp-52] + [0x2b36c0])` |
| `ldr` at 0xcb67c | load | 4 | `abs 0xcb8ac` | `-` |
| `ldr` at 0xcb680 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcb684 | load | 4 | `abs 0x2b36c0` | `-` |
| `str` at 0xcb68c | store | 4 | `sp -0x30` | `([sp-48] + [0x2b36c0])` |
| `ldr` at 0xcb690 | load | 4 | `abs 0xcb8ac` | `-` |
| `ldr` at 0xcb694 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xcb698 | load | 4 | `abs 0x2b36c0` | `-` |
| `str` at 0xcb6a0 | store | 4 | `sp -0x40` | `([sp-64] + [0x2b36c0])` |
| `ldr` at 0xcb6a8 | load | 4 | `abs 0xcb8cc` | `-` |
| `ldr` at 0xcb6ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcb6b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcb6b4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xcb6d4 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xcb6e0 | load | 4 | `abs 0xcb8d0` | `-` |
| `str` at 0xcb6e4 | store | 4 | `sp -0x70` | `0x133ad8` |
| `ldr` at 0xcb6e8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcb6ec | store | 4 | `sp -0x6c` | `[sp-20]` |
| `str` at 0xcb6f0 | store | 4 | `sp -0x68` | `ret(0x21674)` |
| `ldr` at 0xcb6f8 | load | 4 | `abs 0xcb8b4` | `-` |
| `ldr` at 0xcb6fc | load | 4 | `abs 0xcb8d4` | `-` |
| `str` at 0xcb70c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xcb714 | load | 4 | `abs 0xcb8cc` | `-` |
| `ldr` at 0xcb718 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcb71c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcb720 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcb724 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcb730 | store | 4 | `sp -0x2c` | `ret(0x1afc8)` |
| `ldr` at 0xcb734 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcb738 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcb74c | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xcb758 | load | 4 | `abs 0xcb8d8` | `-` |
| `str` at 0xcb75c | store | 4 | `sp -0x70` | `0x133b04` |
| `ldr` at 0xcb760 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcb764 | store | 4 | `sp -0x6c` | `[sp-20]` |
| `ldr` at 0xcb768 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xcb76c | store | 4 | `sp -0x68` | `[sp-44]` |
| `str` at 0xcb770 | store | 4 | `sp -0x64` | `ret(0x21674)` |
| `ldr` at 0xcb778 | load | 4 | `abs 0xcb8b4` | `-` |
| `ldr` at 0xcb77c | load | 4 | `abs 0xcb8dc` | `-` |
| `str` at 0xcb78c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xcb794 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcb798 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcb7b8 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xcb7c4 | load | 4 | `abs 0xcb8e0` | `-` |
| `str` at 0xcb7c8 | store | 4 | `sp -0x70` | `0x133b34` |
| `ldr` at 0xcb7cc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcb7d0 | store | 4 | `sp -0x6c` | `[sp-20]` |
| `str` at 0xcb7d4 | store | 4 | `sp -0x68` | `ret(0x21674)` |
| `ldr` at 0xcb7dc | load | 4 | `abs 0xcb8b4` | `-` |
| `ldr` at 0xcb7e0 | load | 4 | `abs 0xcb8e4` | `-` |
| `str` at 0xcb7f0 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xcb7fc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xcb800 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcb804 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcb814 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcb818 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcb820 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcb824 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xcb82c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xcb830 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xcb83c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcb840 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcb848 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xcb84c | load | 4 | `abs 0xcb8e8` | `-` |
| `str` at 0xcb850 | store | 4 | `sp -0x2c` | `0xfffffc7a` |
| `ldr` at 0xcb858 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcb860 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xcb870 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xcb878 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcb884 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcb888 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcb88c | store | 4 | `[sp-32] +0x0` | `[sp-20]` |
| `ldr` at 0xcb894 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xcb898 | store | 4 | `sp -0x60` | `[sp-44]` |
| `ldr` at 0xcb89c | load | 4 | `sp -0x60` | `-` |
| `ldmdb` at 0xcb8a0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcb8a0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcb8a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcb4c8 | `(unknown callee) 0xcae70` | `0x0` | `r1` | `r2` | `[0x2ad1d4]` |
| `bl` at 0xcb4ec | `(unknown callee) 0xcaf90` | `?` | `?` | `?` | `?` |
| `bl` at 0xcb4f4 | `(unknown callee) 0xcb0f8` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0xcb510 | `(unknown callee) 0xcb330` | `[sp-20]` | `?` | `?` | `ret(0xcb0f8)` |
| `bl` at 0xcb53c | `(unknown callee) 0xcb190` | `[sp-20]` | `?` | `?` | `ret(0xcb330)` |
| `bl` at 0xcb564 | `(unknown callee) 0xfa7bc` | `ret(0xcb190)` | `[0x2b36c0]` | `ret(0xcb190)` | `0x2b36a0` |
| `bl` at 0xcb594 | `(unknown callee) 0x21e28` | `0x3` | `0x133850` | `0x193` | `0x0` |
| `bl` at 0xcb604 | `(unknown callee) 0x1b40c` | `?` | `0xc00c4d04` | `sp-92` | `[sp-40]` |
| `bl` at 0xcb614 | `(unknown callee) 0x1b0b8` | `ret(0x1b40c)` | `?` | `?` | `ret(0x1b40c)` |
| `bl` at 0xcb620 | `(unknown callee) 0x21674` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0xcb64c | `(unknown callee) 0x21e28` | `0x3` | `0x133850` | `0x19e` | `0x0` |
| `bl` at 0xcb6bc | `(unknown callee) 0x1b2ec` | `?` | `?` | `0x0` | `0x142a94` |
| `bl` at 0xcb6cc | `(unknown callee) 0x1b0b8` | `ret(0x1b2ec)` | `?` | `?` | `ret(0x1b2ec)` |
| `bl` at 0xcb6d8 | `(unknown callee) 0x21674` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0xcb704 | `(unknown callee) 0x21e28` | `0x3` | `0x133850` | `0x1af` | `0x0` |
| `bl` at 0xcb728 | `(unknown callee) 0x1afc8` | `?` | `[sp-24]` | `[sp-28]` | `[sp-40]` |
| `bl` at 0xcb744 | `(unknown callee) 0x1b0b8` | `ret(0x1afc8)` | `?` | `ret(0x1afc8)` | `[sp-28]` |
| `bl` at 0xcb750 | `(unknown callee) 0x21674` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0xcb784 | `(unknown callee) 0x21e28` | `0x3` | `0x133850` | `0x1b5` | `0x0` |
| `bl` at 0xcb7a0 | `(unknown callee) 0x1b1b4` | `[sp-48]` | `[sp-28]` | `0x6` | `[sp-28]` |
| `bl` at 0xcb7b0 | `(unknown callee) 0x1b0b8` | `ret(0x1b1b4)` | `?` | `?` | `ret(0x1b1b4)` |
| `bl` at 0xcb7bc | `(unknown callee) 0x21674` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0xcb7e8 | `(unknown callee) 0x21e28` | `0x3` | `0x133850` | `0x1bd` | `0x0` |
| `bl` at 0xcb868 | `(unknown callee) 0xcb044` | `ret(0x1b1b4)` | `?` | `?` | `[sp-28]` |
| `bl` at 0xcb890 | `(unknown callee) 0xcb044` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcb4c0 | `[0x2ad1d4] != 0x0` | `[0x2ad1d4] == 0x0` |
| `bge` at 0xcb4dc | `ret(0xcae70) >= 0x0 (signed, 32-bit)` | `ret(0xcae70) < 0x0 (signed, 32-bit)` |
| `beq` at 0xcb508 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xcb524 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xcb550 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `beq` at 0xcb570 | `ret(0xfa7bc) == 0x0` | `ret(0xfa7bc) != 0x0` |
| `blo` at 0xcb5c4 | `C == 0` | `not (C == 0)` |
| `beq` at 0xcb610 | `ret(0x1b40c) == 0x0` | `ret(0x1b40c) != 0x0` |
| `beq` at 0xcb664 | `[sp-75]:1 == 0xff` | `[sp-75]:1 != 0xff` |
| `bge` at 0xcb6c8 | `ret(0x1b2ec) >= 0x0 (signed, 32-bit)` | `ret(0x1b2ec) < 0x0 (signed, 32-bit)` |
| `beq` at 0xcb740 | `ret(0x1afc8) == [sp-28]` | `ret(0x1afc8) != [sp-28]` |
| `bge` at 0xcb7ac | `ret(0x1b1b4) >= 0x0 (signed, 32-bit)` | `ret(0x1b1b4) < 0x0 (signed, 32-bit)` |
| `blo` at 0xcb80c | `C == 0` | `not (C == 0)` |
| `beq` at 0xcb838 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xcb880 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcb8a0 | `r0` | `?` |

## `flash_set_boot_image`

- `flash_set_boot_image`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcb8ec to 0xcbb10.
  Established. The two byte stores at `sp -0x15` write `0x41` and `0x42`, respectively (`strb` at
  0xcb9a0, `strb` at 0xcb9c0). The call to `0x1afc8` uses `ret(0x1b370)`, `sp-21`, `0x1`, `sp-21`
  (`bl` at 0xcba70); its result takes the continuation on equality with `0x1` and falls through
  otherwise (`beq` at 0xcba7c). Boot selection persistence and the common r0 result are unproved
  (`ldmdb` at 0xcbadc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcb8fc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xcb900 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcb90c | load | 4 | `abs 0xcbae0` | `-` |
| `ldr` at 0xcb928 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xcb93c | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xcb948 | load | 4 | `abs 0xcbae4` | `-` |
| `str` at 0xcb94c | store | 4 | `sp -0x2c` | `0x133b74` |
| `ldr` at 0xcb950 | load | 4 | `abs 0xcbae0` | `-` |
| `str` at 0xcb954 | store | 4 | `sp -0x28` | `0x133b60` |
| `str` at 0xcb958 | store | 4 | `sp -0x24` | `ret(0x21674)` |
| `ldr` at 0xcb960 | load | 4 | `abs 0xcbae8` | `-` |
| `str` at 0xcb974 | store | 4 | `sp -0x20` | `?` |
| `str` at 0xcb980 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xcb988 | load | 4 | `abs 0xcbaec` | `-` |
| `ldr` at 0xcb98c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcb990 | load | 4 | `abs 0x2b36a8` | `-` |
| `strb` at 0xcb9a0 | store | 1 | `sp -0x15` | `0x41` |
| `ldr` at 0xcb9a8 | load | 4 | `abs 0xcbaec` | `-` |
| `ldr` at 0xcb9ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcb9b0 | load | 4 | `abs 0x2b36ac` | `-` |
| `strb` at 0xcb9c0 | store | 1 | `sp -0x15` | `0x42` |
| `ldr` at 0xcb9c8 | load | 4 | `abs 0xcbaf0` | `-` |
| `str` at 0xcb9cc | store | 4 | `sp -0x2c` | `0x133ba0` |
| `ldr` at 0xcb9d0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcb9d4 | store | 4 | `sp -0x28` | `r0` |
| `ldr` at 0xcb9dc | load | 4 | `abs 0xcbae8` | `-` |
| `ldr` at 0xcb9e0 | load | 4 | `abs 0xcbaf4` | `-` |
| `str` at 0xcb9f0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcb9f8 | load | 4 | `abs 0xcbae0` | `-` |
| `ldr` at 0xcb9fc | load | 4 | `abs 0xcbaf8` | `-` |
| `str` at 0xcba08 | store | 4 | `sp -0x1c` | `ret(0x1b370)` |
| `ldr` at 0xcba0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcba20 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xcba2c | load | 4 | `abs 0xcbafc` | `-` |
| `str` at 0xcba30 | store | 4 | `sp -0x2c` | `0x133bc8` |
| `ldr` at 0xcba34 | load | 4 | `abs 0xcbb00` | `-` |
| `str` at 0xcba38 | store | 4 | `sp -0x28` | `0x133bf4` |
| `str` at 0xcba3c | store | 4 | `sp -0x24` | `ret(0x21674)` |
| `ldr` at 0xcba44 | load | 4 | `abs 0xcbae8` | `-` |
| `ldr` at 0xcba48 | load | 4 | `abs 0xcbb04` | `-` |
| `str` at 0xcba58 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcba64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcba88 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xcba94 | load | 4 | `abs 0xcbb08` | `-` |
| `str` at 0xcba98 | store | 4 | `sp -0x2c` | `0x133c0c` |
| `ldr` at 0xcba9c | load | 4 | `abs 0xcbae0` | `-` |
| `str` at 0xcbaa0 | store | 4 | `sp -0x28` | `0x133b60` |
| `str` at 0xcbaa4 | store | 4 | `sp -0x24` | `ret(0x21674)` |
| `ldr` at 0xcbaac | load | 4 | `abs 0xcbae8` | `-` |
| `ldr` at 0xcbab0 | load | 4 | `abs 0xcbb0c` | `-` |
| `str` at 0xcbac0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcbac8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcbad4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xcbad8 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xcbadc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcbadc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcbadc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcb910 | `(unknown callee) 0x1b040` | `0x133b60` | `r1` | `r2` | `r0` |
| `bl` at 0xcb920 | `(unknown callee) 0x1b0b8` | `ret(0x1b040)` | `?` | `?` | `ret(0x1b040)` |
| `bl` at 0xcb934 | `(unknown callee) 0x1b0b8` | `ret(0x1b0b8)` | `?` | `?` | `[ret(0x1b0b8)]` |
| `bl` at 0xcb940 | `(unknown callee) 0x21674` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0xcb96c | `(unknown callee) 0x21e28` | `0x3` | `0x133850` | `0x1e8` | `0x0` |
| `bl` at 0xcb9e8 | `(unknown callee) 0x21e28` | `0x3` | `0x133850` | `0x1f2` | `0x0` |
| `bl` at 0xcba00 | `(unknown callee) 0x1b370` | `0x133b60` | `0x241` | `r0` | `?` |
| `bl` at 0xcba18 | `(unknown callee) 0x1b0b8` | `ret(0x1b370)` | `?` | `?` | `ret(0x1b370)` |
| `bl` at 0xcba24 | `(unknown callee) 0x21674` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0xcba50 | `(unknown callee) 0x21e28` | `0x3` | `0x133850` | `0x1f7` | `0x0` |
| `bl` at 0xcba70 | `(unknown callee) 0x1afc8` | `ret(0x1b370)` | `sp-21` | `0x1` | `sp-21` |
| `bl` at 0xcba80 | `(unknown callee) 0x1b0b8` | `ret(0x1afc8)` | `?` | `?` | `ret(0x1afc8)` |
| `bl` at 0xcba8c | `(unknown callee) 0x21674` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0xcbab8 | `(unknown callee) 0x21e28` | `0x3` | `0x133850` | `0x1fd` | `0x0` |
| `bl` at 0xcbacc | `(unknown callee) 0x1b004` | `[sp-28]` | `?` | `?` | `ret(0x1afc8)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcb908 | `r0 != 0x0` | `r0 == 0x0` |
| `bge` at 0xcb91c | `ret(0x1b040) >= 0x0 (signed, 32-bit)` | `ret(0x1b040) < 0x0 (signed, 32-bit)` |
| `beq` at 0xcb930 | `[ret(0x1b0b8)] == 0x2` | `[ret(0x1b0b8)] != 0x2` |
| `bne` at 0xcb998 | `r0 != [0x2b36a8]` | `r0 == [0x2b36a8]` |
| `bne` at 0xcb9b8 | `r0 != [0x2b36ac]` | `r0 == [0x2b36ac]` |
| `bge` at 0xcba14 | `ret(0x1b370) >= 0x0 (signed, 32-bit)` | `ret(0x1b370) < 0x0 (signed, 32-bit)` |
| `beq` at 0xcba7c | `ret(0x1afc8) == 0x1` | `ret(0x1afc8) != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcbadc | `r0` | `?` |

## `flash_set_default_image`

- `flash_set_default_image`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcbb10 to 0xcbdc0.
  Established. The opening word stores at `[0x2b36bc] +0x0` and `[0x2b36bc] +0x4` have unknown
  values (`str` at 0xcbb3c, `str` at 0xcbb68). Later byte stores write `0x41` and `0x42` at `sp
  -0x19` (`strb` at 0xcbc0c, `strb` at 0xcbc2c). The call to `0x1afc8` has `ret(0x1b370)`, `sp-25`,
  `0x1`, `sp-25` (`bl` at 0xcbcdc). Default-image flag meanings and r0 at exit remain unidentified
  (`ldmdb` at 0xcbd8c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcbb20 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xcbb24 | load | 4 | `abs 0xcbd90` | `-` |
| `ldr` at 0xcbb28 | load | 4 | `abs 0x2b36bc` | `-` |
| `ldr` at 0xcbb2c | load | 4 | `abs 0xcbd90` | `-` |
| `ldr` at 0xcbb30 | load | 4 | `abs 0x2b36bc` | `-` |
| `ldr` at 0xcbb34 | load | 4 | `[0x2b36bc] +0x0` | `-` |
| `str` at 0xcbb3c | store | 4 | `[0x2b36bc] +0x0` | `?` |
| `ldr` at 0xcbb40 | load | 4 | `abs 0xcbd90` | `-` |
| `ldr` at 0xcbb48 | load | 4 | `abs 0x2b36bc` | `-` |
| `ldr` at 0xcbb50 | load | 4 | `abs 0xcbd90` | `-` |
| `ldr` at 0xcbb58 | load | 4 | `abs 0x2b36bc` | `-` |
| `ldr` at 0xcbb60 | load | 4 | `[0x2b36bc] +0x4` | `-` |
| `str` at 0xcbb68 | store | 4 | `[0x2b36bc] +0x4` | `?` |
| `ldr` at 0xcbb6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcbb78 | load | 4 | `abs 0xcbd94` | `-` |
| `ldr` at 0xcbb94 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xcbba8 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xcbbb4 | load | 4 | `abs 0xcbd98` | `-` |
| `str` at 0xcbbb8 | store | 4 | `sp -0x2c` | `0x133c38` |
| `ldr` at 0xcbbbc | load | 4 | `abs 0xcbd94` | `-` |
| `str` at 0xcbbc0 | store | 4 | `sp -0x28` | `0x133bf4` |
| `str` at 0xcbbc4 | store | 4 | `sp -0x24` | `ret(0x21674)` |
| `ldr` at 0xcbbcc | load | 4 | `abs 0xcbd9c` | `-` |
| `ldr` at 0xcbbd0 | load | 4 | `abs 0xcbda0` | `-` |
| `str` at 0xcbbe0 | store | 4 | `sp -0x20` | `?` |
| `str` at 0xcbbec | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xcbbf4 | load | 4 | `abs 0xcbd90` | `-` |
| `ldr` at 0xcbbf8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcbbfc | load | 4 | `abs 0x2b36a8` | `-` |
| `strb` at 0xcbc0c | store | 1 | `sp -0x19` | `0x41` |
| `ldr` at 0xcbc14 | load | 4 | `abs 0xcbd90` | `-` |
| `ldr` at 0xcbc18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcbc1c | load | 4 | `abs 0x2b36ac` | `-` |
| `strb` at 0xcbc2c | store | 1 | `sp -0x19` | `0x42` |
| `ldr` at 0xcbc34 | load | 4 | `abs 0xcbda4` | `-` |
| `str` at 0xcbc38 | store | 4 | `sp -0x2c` | `0x133c68` |
| `ldr` at 0xcbc3c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcbc40 | store | 4 | `sp -0x28` | `[sp-20]` |
| `ldr` at 0xcbc48 | load | 4 | `abs 0xcbd9c` | `-` |
| `ldr` at 0xcbc4c | load | 4 | `abs 0xcbda8` | `-` |
| `str` at 0xcbc5c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcbc64 | load | 4 | `abs 0xcbd94` | `-` |
| `ldr` at 0xcbc68 | load | 4 | `abs 0xcbdac` | `-` |
| `str` at 0xcbc74 | store | 4 | `sp -0x18` | `ret(0x1b370)` |
| `ldr` at 0xcbc78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcbc8c | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xcbc98 | load | 4 | `abs 0xcbdb0` | `-` |
| `str` at 0xcbc9c | store | 4 | `sp -0x2c` | `0x133bc8` |
| `ldr` at 0xcbca0 | load | 4 | `abs 0xcbd94` | `-` |
| `str` at 0xcbca4 | store | 4 | `sp -0x28` | `0x133bf4` |
| `str` at 0xcbca8 | store | 4 | `sp -0x24` | `ret(0x21674)` |
| `ldr` at 0xcbcb0 | load | 4 | `abs 0xcbd9c` | `-` |
| `str` at 0xcbcc4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcbcd0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcbcf4 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xcbd00 | load | 4 | `abs 0xcbdb4` | `-` |
| `str` at 0xcbd04 | store | 4 | `sp -0x2c` | `0x133c94` |
| `ldr` at 0xcbd08 | load | 4 | `abs 0xcbd94` | `-` |
| `str` at 0xcbd0c | store | 4 | `sp -0x28` | `0x133bf4` |
| `str` at 0xcbd10 | store | 4 | `sp -0x24` | `ret(0x21674)` |
| `ldr` at 0xcbd18 | load | 4 | `abs 0xcbd9c` | `-` |
| `ldr` at 0xcbd1c | load | 4 | `abs 0xcbdb8` | `-` |
| `str` at 0xcbd2c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcbd34 | load | 4 | `abs 0xcbd90` | `-` |
| `ldrb` at 0xcbd38 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xcbd3c | load | 4 | `abs 0xcbdbc` | `-` |
| `ldr` at 0xcbd44 | load | 4 | `abs 0x2b36bc` | `-` |
| `ldr` at 0xcbd50 | load | 4 | `abs 0xcbd90` | `-` |
| `ldrb` at 0xcbd54 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xcbd58 | load | 4 | `abs 0xcbdbc` | `-` |
| `ldr` at 0xcbd60 | load | 4 | `abs 0x2b36bc` | `-` |
| `ldr` at 0xcbd6c | load | 4 | `(unknown)` | `-` |
| `str` at 0xcbd74 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xcbd78 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcbd84 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xcbd88 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xcbd8c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcbd8c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcbd8c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcbb7c | `(unknown callee) 0x1b040` | `0x133bf4` | `[0x2b36bc]+4` | `0x4` | `[sp-20]` |
| `bl` at 0xcbb8c | `(unknown callee) 0x1b0b8` | `ret(0x1b040)` | `?` | `?` | `ret(0x1b040)` |
| `bl` at 0xcbba0 | `(unknown callee) 0x1b0b8` | `ret(0x1b0b8)` | `?` | `?` | `[ret(0x1b0b8)]` |
| `bl` at 0xcbbac | `(unknown callee) 0x21674` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0xcbbd8 | `(unknown callee) 0x21e28` | `0x3` | `0x133850` | `0x215` | `0x0` |
| `bl` at 0xcbc54 | `(unknown callee) 0x21e28` | `0x3` | `0x133850` | `0x21f` | `0x0` |
| `bl` at 0xcbc6c | `(unknown callee) 0x1b370` | `0x133bf4` | `0x241` | `[sp-20]` | `?` |
| `bl` at 0xcbc84 | `(unknown callee) 0x1b0b8` | `ret(0x1b370)` | `?` | `?` | `ret(0x1b370)` |
| `bl` at 0xcbc90 | `(unknown callee) 0x21674` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0xcbcbc | `(unknown callee) 0x21e28` | `0x3` | `0x133850` | `0x224` | `0x0` |
| `bl` at 0xcbcdc | `(unknown callee) 0x1afc8` | `ret(0x1b370)` | `sp-25` | `0x1` | `sp-25` |
| `bl` at 0xcbcec | `(unknown callee) 0x1b0b8` | `ret(0x1afc8)` | `?` | `?` | `ret(0x1afc8)` |
| `bl` at 0xcbcf8 | `(unknown callee) 0x21674` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0xcbd24 | `(unknown callee) 0x21e28` | `0x3` | `0x133850` | `0x22a` | `0x0` |
| `bl` at 0xcbd7c | `(unknown callee) 0x1b004` | `[sp-24]` | `0x2b36a0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcbb74 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bge` at 0xcbb88 | `ret(0x1b040) >= 0x0 (signed, 32-bit)` | `ret(0x1b040) < 0x0 (signed, 32-bit)` |
| `beq` at 0xcbb9c | `[ret(0x1b0b8)] == 0x2` | `[ret(0x1b0b8)] != 0x2` |
| `bne` at 0xcbc04 | `[sp-20] != [0x2b36a8]` | `[sp-20] == [0x2b36a8]` |
| `bne` at 0xcbc24 | `[sp-20] != [0x2b36ac]` | `[sp-20] == [0x2b36ac]` |
| `bge` at 0xcbc80 | `ret(0x1b370) >= 0x0 (signed, 32-bit)` | `ret(0x1b370) < 0x0 (signed, 32-bit)` |
| `beq` at 0xcbce8 | `ret(0x1afc8) == 0x1` | `ret(0x1afc8) != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcbd8c | `r0` | `?` |

## `flash_get_default_image`

- `flash_get_default_image`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcbdc0 to 0xcbfec.
  Established. Two initial word stores carry unknown values at `[0x2b36bc] +0x0` and `[0x2b36bc]
  +0x4` (`str` at 0xcbde8, `str` at 0xcbe14). The call to `0x1b4fc` has `ret(0x1b370)`, `sp-25`,
  `0x1`, `sp-25` (`bl` at 0xcbeb0). The later byte test takes its arm for `[sp-25]:1 != 0x41` and
  falls through on equality (`bne` at 0xcbf10). Stored-image interpretation and the common r0 return
  remain unknown (`ldmdb` at 0xcbfc8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xcbdd0 | load | 4 | `abs 0xcbfcc` | `-` |
| `ldr` at 0xcbdd4 | load | 4 | `abs 0x2b36bc` | `-` |
| `ldr` at 0xcbdd8 | load | 4 | `abs 0xcbfcc` | `-` |
| `ldr` at 0xcbddc | load | 4 | `abs 0x2b36bc` | `-` |
| `ldr` at 0xcbde0 | load | 4 | `[0x2b36bc] +0x0` | `-` |
| `str` at 0xcbde8 | store | 4 | `[0x2b36bc] +0x0` | `?` |
| `ldr` at 0xcbdec | load | 4 | `abs 0xcbfcc` | `-` |
| `ldr` at 0xcbdf4 | load | 4 | `abs 0x2b36bc` | `-` |
| `ldr` at 0xcbdfc | load | 4 | `abs 0xcbfcc` | `-` |
| `ldr` at 0xcbe04 | load | 4 | `abs 0x2b36bc` | `-` |
| `ldr` at 0xcbe0c | load | 4 | `[0x2b36bc] +0x4` | `-` |
| `str` at 0xcbe14 | store | 4 | `[0x2b36bc] +0x4` | `?` |
| `ldr` at 0xcbe18 | load | 4 | `abs 0xcbfd0` | `-` |
| `str` at 0xcbe28 | store | 4 | `sp -0x14` | `ret(0x1b370)` |
| `ldr` at 0xcbe2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcbe40 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `str` at 0xcbe50 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xcbe60 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xcbe6c | load | 4 | `abs 0xcbfd4` | `-` |
| `str` at 0xcbe70 | store | 4 | `sp -0x2c` | `0x133cc0` |
| `ldr` at 0xcbe74 | load | 4 | `abs 0xcbfd0` | `-` |
| `str` at 0xcbe78 | store | 4 | `sp -0x28` | `0x133bf4` |
| `str` at 0xcbe7c | store | 4 | `sp -0x24` | `ret(0x21674)` |
| `ldr` at 0xcbe84 | load | 4 | `abs 0xcbfd8` | `-` |
| `str` at 0xcbe98 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcbea4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcbec8 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xcbed4 | load | 4 | `abs 0xcbfdc` | `-` |
| `str` at 0xcbed8 | store | 4 | `sp -0x2c` | `0x133cf0` |
| `ldr` at 0xcbedc | load | 4 | `abs 0xcbfd0` | `-` |
| `str` at 0xcbee0 | store | 4 | `sp -0x28` | `0x133bf4` |
| `str` at 0xcbee4 | store | 4 | `sp -0x24` | `ret(0x21674)` |
| `ldr` at 0xcbeec | load | 4 | `abs 0xcbfd8` | `-` |
| `ldr` at 0xcbef0 | load | 4 | `abs 0xcbfe0` | `-` |
| `str` at 0xcbf00 | store | 4 | `sp -0x20` | `?` |
| `ldrb` at 0xcbf08 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xcbf14 | load | 4 | `abs 0xcbfcc` | `-` |
| `ldr` at 0xcbf18 | load | 4 | `abs 0x2b36a8` | `-` |
| `str` at 0xcbf1c | store | 4 | `sp -0x18` | `[0x2b36a8]` |
| `ldrb` at 0xcbf24 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xcbf30 | load | 4 | `abs 0xcbfcc` | `-` |
| `ldr` at 0xcbf34 | load | 4 | `abs 0x2b36ac` | `-` |
| `str` at 0xcbf38 | store | 4 | `sp -0x18` | `[0x2b36ac]` |
| `ldr` at 0xcbf40 | load | 4 | `abs 0xcbfe4` | `-` |
| `str` at 0xcbf44 | store | 4 | `sp -0x2c` | `0x133d24` |
| `ldrb` at 0xcbf48 | load | 1 | `sp -0x19` | `-` |
| `str` at 0xcbf4c | store | 4 | `sp -0x28` | `[sp-25]:1` |
| `ldr` at 0xcbf54 | load | 4 | `abs 0xcbfd8` | `-` |
| `str` at 0xcbf68 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcbf70 | load | 4 | `abs 0xcbfcc` | `-` |
| `ldrb` at 0xcbf74 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xcbf78 | load | 4 | `abs 0xcbfe8` | `-` |
| `ldr` at 0xcbf80 | load | 4 | `abs 0x2b36bc` | `-` |
| `ldr` at 0xcbf8c | load | 4 | `abs 0xcbfcc` | `-` |
| `ldrb` at 0xcbf90 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xcbf94 | load | 4 | `abs 0xcbfe8` | `-` |
| `ldr` at 0xcbf9c | load | 4 | `abs 0x2b36bc` | `-` |
| `ldr` at 0xcbfa8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcbfb0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xcbfb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcbfbc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcbfc0 | store | 4 | `sp -0x20` | `[sp-24]` |
| `ldr` at 0xcbfc4 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xcbfc8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcbfc8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcbfc8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcbe20 | `(unknown callee) 0x1b370` | `0x133bf4` | `0x0` | `0x4` | `?` |
| `bl` at 0xcbe38 | `(unknown callee) 0x1b0b8` | `ret(0x1b370)` | `?` | `?` | `ret(0x1b370)` |
| `bl` at 0xcbe58 | `(unknown callee) 0x1b0b8` | `ret(0x1b0b8)` | `?` | `?` | `[ret(0x1b0b8)]` |
| `bl` at 0xcbe64 | `(unknown callee) 0x21674` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0xcbe90 | `(unknown callee) 0x21e28` | `0x3` | `0x133850` | `0x248` | `0x0` |
| `bl` at 0xcbeb0 | `(unknown callee) 0x1b4fc` | `ret(0x1b370)` | `sp-25` | `0x1` | `sp-25` |
| `bl` at 0xcbec0 | `(unknown callee) 0x1b0b8` | `ret(0x1b4fc)` | `?` | `?` | `ret(0x1b4fc)` |
| `bl` at 0xcbecc | `(unknown callee) 0x21674` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0xcbef8 | `(unknown callee) 0x21e28` | `0x3` | `0x133850` | `0x24e` | `0x0` |
| `bl` at 0xcbf60 | `(unknown callee) 0x21e28` | `0x3` | `0x133850` | `0x258` | `0x0` |
| `bl` at 0xcbfb8 | `(unknown callee) 0x1b004` | `[sp-20]` | `0x2b36a0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xcbe34 | `ret(0x1b370) >= 0x0 (signed, 32-bit)` | `ret(0x1b370) < 0x0 (signed, 32-bit)` |
| `bne` at 0xcbe48 | `[ret(0x1b0b8)] != 0x2` | `[ret(0x1b0b8)] == 0x2` |
| `beq` at 0xcbebc | `ret(0x1b4fc) == 0x1` | `ret(0x1b4fc) != 0x1` |
| `bne` at 0xcbf10 | `[sp-25]:1 != 0x41` | `[sp-25]:1 == 0x41` |
| `bne` at 0xcbf2c | `[sp-25]:1 != 0x42` | `[sp-25]:1 == 0x42` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcbfc8 | `r0` | `?` |

## `portReset`

- `portReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x41c24 to 0x41e20.
  Established. Two byte-store sites place `0x3` at `[sp-44] +0x0` (`strb` at 0x41d08, `strb` at
  0x41d34). The call to `0x43254` has `[sp-24]`, `[sp-40]`, unknown, unknown (`bl` at 0x41d40); its
  zero result takes the continuation and nonzero falls through (`beq` at 0x41d54). Port-reset
  hardware effects and r0 at the exit remain unresolved (`ldmdb` at 0x41e04).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x41c34 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x41c38 | load | 4 | `abs 0x41e08` | `-` |
| `ldr` at 0x41c3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x41c50 | load | 4 | `abs 0x147fdc` | `-` |
| `str` at 0x41c58 | store | 4 | `sp -0x24` | `?` |
| `strb` at 0x41c60 | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0x41c64 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x41c68 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x41c6c | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0x41c7c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x41c80 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x41c88 | load | 4 | `[sp-36] +0x4` | `-` |
| `ldrb` at 0x41c90 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x41c9c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x41ca0 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x41cb4 | load | 4 | `[sp-36] +0x0` | `-` |
| `str` at 0x41cbc | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x41cc0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x41cc4 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x41ccc | load | 4 | `[sp-36] +0x4` | `-` |
| `str` at 0x41cd4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x41cd8 | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0x41cdc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x41cec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x41d00 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0x41d08 | store | 1 | `[sp-44] +0x0` | `0x3` |
| `ldr` at 0x41d10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x41d14 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x41d2c | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0x41d34 | store | 1 | `[sp-44] +0x0` | `0x3` |
| `ldr` at 0x41d38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x41d3c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x41d48 | store | 4 | `sp -0x1c` | `ret(0x43254)` |
| `ldr` at 0x41d4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x41d58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x41d60 | load | 4 | `abs 0x41e0c` | `-` |
| `ldr` at 0x41d64 | load | 4 | `abs 0x41e10` | `-` |
| `str` at 0x41d90 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x41d94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x41d9c | load | 4 | `abs 0x41e0c` | `-` |
| `ldr` at 0x41da0 | load | 4 | `abs 0x41e10` | `-` |
| `ldr` at 0x41dac | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x41dbc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x41dc8 | load | 4 | `abs 0x41e14` | `-` |
| `str` at 0x41dcc | store | 4 | `(unknown)` | `0x108c7c` |
| `str` at 0x41dd0 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldrb` at 0x41dd4 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x41dd8 | store | 4 | `(unknown)` | `[sp-29]:1` |
| `str` at 0x41ddc | store | 4 | `(unknown)` | `ret(0x219a8)` |
| `ldr` at 0x41de4 | load | 4 | `abs 0x41e18` | `-` |
| `ldr` at 0x41de8 | load | 4 | `abs 0x41e1c` | `-` |
| `ldrb` at 0x41df4 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x41dfc | store | 1 | `sp -0x1d` | `?` |
| `ldmdb` at 0x41e04 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x41e04 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x41e04 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x41e04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x41cf0 | `(unknown callee) 0x3d970` | `?` | `[sp-36]` | `?` | `?` |
| `bl` at 0x41d1c | `(unknown callee) 0x447f0` | `?` | `?` | `0x2` | `?` |
| `bl` at 0x41d40 | `(unknown callee) 0x43254` | `[sp-24]` | `[sp-40]` | `?` | `?` |
| `bl` at 0x41d68 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x108b40` | `0x108b48` |
| `bl` at 0x41d74 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x41da4 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x108b40` | `0x108b48` |
| `bl` at 0x41db4 | `(unknown callee) 0x1b508` | `[sp-48]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x41dc0 | `(unknown callee) 0x219a8` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x41df0 | `(unknown callee) 0x21e28` | `0x3` | `0x108af4` | `0x10b` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x41c74 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bls` at 0x41c98 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x41ce8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x41cfc | `ret(0x3d970) == 0x0` | `ret(0x3d970) != 0x0` |
| `bne` at 0x41d28 | `ret(0x447f0) != 0x1` | `ret(0x447f0) == 0x1` |
| `beq` at 0x41d54 | `ret(0x43254) == 0x0` | `ret(0x43254) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x41e04 | `r0` | `?` |

## `promIceReset`

- `promIceReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x44b78 to 0x44ca8.
  Established. The call to `0x1f1dc` receives `[sp-25]:1+108`, `0x2b0720`, unknown, `[sp-25]:1+108`
  (`bl` at 0x44be4); its nonzero result takes the branch and zero falls through (`bne` at 0x44bf0).
  The later call to `0xd6a1c` has `0x0`, `[[sp-20]]:1`, `[[sp-20]+1]:1`, unknown (`bl` at 0x44c38).
  PROM reset signaling and the merged r0 value are unestablished (`ldmdb` at 0x44c94).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x44b8c | store | 1 | `sp -0x19` | `0x0` |
| `ldrb` at 0x44b90 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x44b94 | load | 4 | `abs 0x44c98` | `-` |
| `ldr` at 0x44b98 | load | 4 | `abs 0x2b0700` | `-` |
| `ldrb` at 0x44ba8 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x44bc0 | load | 4 | `abs 0x44c9c` | `-` |
| `ldrb` at 0x44bc4 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x44bcc | load | 4 | `abs 0x2b0720` | `-` |
| `str` at 0x44bd4 | store | 4 | `sp -0x14` | `?` |
| `ldrb` at 0x44bd8 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x44bf4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x44bf8 | load | 1 | `[sp-20] +0x1` | `-` |
| `strb` at 0x44bfc | store | 1 | `sp -0x1b` | `([[sp-20]+1]:1 & 0xff)` |
| `strb` at 0x44c08 | store | 1 | `sp -0x1b` | `0x0` |
| `ldrb` at 0x44c0c | load | 1 | `sp -0x1b` | `-` |
| `strb` at 0x44c10 | store | 1 | `sp -0x1a` | `?` |
| `ldr` at 0x44c14 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x44c18 | load | 1 | `[sp-20] +0x0` | `-` |
| `ldr` at 0x44c1c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x44c20 | load | 1 | `[sp-20] +0x1` | `-` |
| `ldrb` at 0x44c24 | load | 1 | `sp -0x1a` | `-` |
| `str` at 0x44c40 | store | 4 | `sp -0x18` | `ret(0xd6a1c)` |
| `ldr` at 0x44c44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x44c50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x44c5c | load | 4 | `abs 0x44ca0` | `-` |
| `str` at 0x44c60 | store | 4 | `sp -0x28` | `0x109144` |
| `ldrb` at 0x44c64 | load | 1 | `sp -0x19` | `-` |
| `str` at 0x44c68 | store | 4 | `sp -0x24` | `[sp-25]:1` |
| `str` at 0x44c6c | store | 4 | `sp -0x20` | `ret(0x219a8)` |
| `ldr` at 0x44c74 | load | 4 | `abs 0x44ca4` | `-` |
| `ldrb` at 0x44c84 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0x44c8c | store | 1 | `sp -0x19` | `([sp-25]:1+1 & 0xff)` |
| `ldmdb` at 0x44c94 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x44c94 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x44c94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x44bb0 | `(unknown callee) 0x44d04` | `?` | `?` | `[0x2b0700]` | `?` |
| `bl` at 0x44be4 | `(unknown callee) 0x1f1dc` | `[sp-25]:1+108` | `0x2b0720` | `?` | `[sp-25]:1+108` |
| `bl` at 0x44c38 | `(unknown callee) 0xd6a1c` | `0x0` | `[[sp-20]]:1` | `[[sp-20]+1]:1` | `?` |
| `bl` at 0x44c54 | `(unknown callee) 0x219a8` | `ret(0xd6a1c)` | `?` | `?` | `ret(0xd6a1c)` |
| `bl` at 0x44c80 | `(unknown callee) 0x21e28` | `0x1` | `0x109110` | `0x68` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x44ba0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x44bbc | `ret(0x44d04) == 0x0` | `ret(0x44d04) != 0x0` |
| `bne` at 0x44bf0 | `ret(0x1f1dc) != 0x0` | `ret(0x1f1dc) == 0x0` |
| `beq` at 0x44c4c | `ret(0xd6a1c) == 0x0` | `ret(0xd6a1c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x44c94 | `r0` | `?` |

## `tio_test_pci_reset_int`

- `tio_test_pci_reset_int`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2c1d0 to 0x2c23c.
  Established. The entry byte comparison takes its bypass for `(r1 & 0xff) != 0xc` and falls through
  on equality (`bne` at 0x2c208). The sole call to `0x53e88` has `0x0`, `(r1 & 0xff)`, `0x2`, `(r3 &
  0xff)` (`bl` at 0x2c234). Earlier byte saves retain `(r2 & 0xff)` at `sp -0x16` and `(r3 & 0xff)`
  at `sp -0x17` (`strb` at 0x2c1f4, `strb` at 0x2c1fc). PCI reset-test effects and r0 at exit remain
  unknown (`ldmdb` at 0x2c238).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2c1e0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x2c1ec | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x2c1f4 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x2c1fc | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldrb` at 0x2c200 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x2c20c | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x2c21c | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x2c220 | load | 1 | `sp -0x17` | `-` |
| `ldmdb` at 0x2c238 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2c238 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2c238 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2c234 | `(unknown callee) 0x53e88` | `0x0` | `(r1 & 0xff)` | `0x2` | `(r3 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x2c208 | `(r1 & 0xff) != 0xc` | `(r1 & 0xff) == 0xc` |
| `beq` at 0x2c218 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2c238 | `r0` | `?` |

## `imageSyncResetHold`

- `imageSyncResetHold`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdadd8 to 0xdae50.
  Established. The call to `0xdb07c` uses `(r0 & 0xff)`, entry r1, entry r2, `(r0 & 0xff)` (`bl` at
  0xdadf8); its nonzero result takes the continuation and zero falls through (`bne` at 0xdae0c). The
  later call to `0xd69d4` has `0x0`, `[ret(0xdb07c)+2]:1`, `[ret(0xdb07c)+3]:1`,
  `[ret(0xdb07c)+3]:1` (`bl` at 0xdae38). Image-sync assertion and the common r0 result remain
  unproved (`ldmdb` at 0xdae48).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xdadec | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xdadf0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xdae00 | store | 4 | `sp -0x18` | `ret(0xdb07c)` |
| `ldr` at 0xdae04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdae10 | load | 4 | `abs 0xdae4c` | `-` |
| `str` at 0xdae14 | store | 4 | `sp -0x1c` | `0xfffffc10` |
| `ldr` at 0xdae1c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdae20 | load | 1 | `ret(0xdb07c) +0x2` | `-` |
| `ldr` at 0xdae24 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdae28 | load | 1 | `ret(0xdb07c) +0x3` | `-` |
| `str` at 0xdae40 | store | 4 | `sp -0x1c` | `ret(0xd69d4)` |
| `ldr` at 0xdae44 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xdae48 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdae48 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdae48 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdadf8 | `(unknown callee) 0xdb07c` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xdae38 | `(unknown callee) 0xd69d4` | `0x0` | `[ret(0xdb07c)+2]:1` | `[ret(0xdb07c)+3]:1` | `[ret(0xdb07c)+3]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdae0c | `ret(0xdb07c) != 0x0` | `ret(0xdb07c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdae48 | `r0` | `?` |

## `imageSyncResetRelease`

- `imageSyncResetRelease`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdae50 to 0xdaec8.
  Established. The byte reads address `ret(0xdb07c) +0x2` and `ret(0xdb07c) +0x3` (`ldrb` at
  0xdae98, `ldrb` at 0xdaea0). The subsequent call to `0xd6988` carries `0x0`, `[ret(0xdb07c)+2]:1`,
  `[ret(0xdb07c)+3]:1`, `[ret(0xdb07c)+3]:1` (`bl` at 0xdaeb0). The earlier branch takes its
  continuation for `ret(0xdb07c) != 0x0` and falls through on zero (`bne` at 0xdae84). Release
  polarity and r0 at exit remain unknown (`ldmdb` at 0xdaec0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xdae64 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xdae68 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xdae78 | store | 4 | `sp -0x18` | `ret(0xdb07c)` |
| `ldr` at 0xdae7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdae88 | load | 4 | `abs 0xdaec4` | `-` |
| `str` at 0xdae8c | store | 4 | `sp -0x1c` | `0xfffffc10` |
| `ldr` at 0xdae94 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdae98 | load | 1 | `ret(0xdb07c) +0x2` | `-` |
| `ldr` at 0xdae9c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdaea0 | load | 1 | `ret(0xdb07c) +0x3` | `-` |
| `str` at 0xdaeb8 | store | 4 | `sp -0x1c` | `ret(0xd6988)` |
| `ldr` at 0xdaebc | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xdaec0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdaec0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdaec0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdae70 | `(unknown callee) 0xdb07c` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xdaeb0 | `(unknown callee) 0xd6988` | `0x0` | `[ret(0xdb07c)+2]:1` | `[ret(0xdb07c)+3]:1` | `[ret(0xdb07c)+3]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdae84 | `ret(0xdb07c) != 0x0` | `ret(0xdb07c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdaec0 | `r0` | `?` |

## `psicReset`

- `psicReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdb8c4 to 0xdb9c4.
  Established. The two calls to `0xdbed8` have `[[sp-32]]`, `[[sp-24]+3]:1`, `0xa`, `[[sp-24]+6]:1`
  and `[[sp-32]]`, `[[sp-24]+3]:1`, `0xc`, `0x0`, respectively (`bl` at 0xdb94c, `bl` at 0xdb988).
  The second result test takes its continuation on zero and falls through on nonzero (`beq` at
  0xdb99c). The return reports r0 `[sp-40]` (`ldmdb` at 0xdb9bc). PSIC command and reset semantics
  remain unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdb8d4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdb8d8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xdb8dc | load | 4 | `abs 0xdb9c0` | `-` |
| `ldr` at 0xdb8e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdb8f4 | load | 4 | `abs 0x142b1c` | `-` |
| `str` at 0xdb8fc | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xdb900 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdb904 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdb908 | load | 1 | `r1 +0x2` | `-` |
| `ldr` at 0xdb910 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xdb918 | store | 4 | `sp -0x20` | `?` |
| `str` at 0xdb920 | store | 4 | `sp -0x28` | `0x0` |
| `strb` at 0xdb928 | store | 1 | `sp -0x29` | `?` |
| `ldr` at 0xdb92c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdb930 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdb934 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0xdb938 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdb93c | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xdb940 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0xdb954 | store | 4 | `sp -0x24` | `ret(0xdbed8)` |
| `ldr` at 0xdb958 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdb964 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdb968 | store | 4 | `sp -0x28` | `ret(0xdbed8)` |
| `ldr` at 0xdb96c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdb970 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdb974 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0xdb978 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0xdb990 | store | 4 | `sp -0x24` | `ret(0xdbed8)` |
| `ldr` at 0xdb994 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdb9a0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdb9ac | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdb9b0 | store | 4 | `sp -0x28` | `ret(0xdbed8)` |
| `ldr` at 0xdb9b4 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xdb9bc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdb9bc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdb9bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdb94c | `(unknown callee) 0xdbed8` | `[[sp-32]]` | `[[sp-24]+3]:1` | `0xa` | `[[sp-24]+6]:1` |
| `bl` at 0xdb988 | `(unknown callee) 0xdbed8` | `[[sp-32]]` | `[[sp-24]+3]:1` | `0xc` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xdb960 | `ret(0xdbed8) == 0x0` | `ret(0xdbed8) != 0x0` |
| `beq` at 0xdb99c | `ret(0xdbed8) == 0x0` | `ret(0xdbed8) != 0x0` |
| `bne` at 0xdb9a8 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdb9bc | `r0` | `[sp-40]` |

## `psicResetAll`

- `psicResetAll`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdb9c4 to 0xdba90.
  Established. The opening call to `0xd687c` carries entry r0, `sp-24`, entry r2, `sp-24` (`bl` at
  0xdb9ec). A later call to `0xdb8c4` has `[sp-20]` and three unknown candidates (`bl` at 0xdba48);
  its zero result takes the continuation and nonzero falls through (`beq` at 0xdba5c). The exit
  reports r0 `[sp-36]` (`ldmdb` at 0xdba8c). Enumeration completeness and physical reset coverage
  remain unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdb9d4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdb9dc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xdb9e4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdb9f4 | store | 4 | `sp -0x20` | `ret(0xd687c)` |
| `str` at 0xdb9fc | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xdba00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdba04 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdba14 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdba1c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdba24 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xdba30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdba38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdba40 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdba50 | store | 4 | `sp -0x28` | `ret(0xdb8c4)` |
| `ldr` at 0xdba54 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdba60 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdba6c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xdba70 | store | 4 | `sp -0x24` | `ret(0xdb8c4)` |
| `ldr` at 0xdba74 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdba7c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xdba84 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xdba8c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdba8c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdba8c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdb9ec | `(unknown callee) 0xd687c` | `r0` | `sp-24` | `r2` | `sp-24` |
| `bl` at 0xdba48 | `(unknown callee) 0xdb8c4` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xdba0c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xdba2c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xdba5c | `ret(0xdb8c4) == 0x0` | `ret(0xdb8c4) != 0x0` |
| `bne` at 0xdba68 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdba8c | `r0` | `[sp-36]` |

## `pcixReset`

- `pcixReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x58150 to 0x581d4.
  Established. The call to `0x58bb8` uses entry r0, `(r1 & 0xff)`, `(r2 & 0xff)`, `(r1 & 0xff)`
  (`bl` at 0x58184); its zero result takes the arm containing the call to `0x5c29c`. Nonzero falls
  through (`beq` at 0x58198). That call carries `0x2`, unknown, unknown, `ret(0x58bb8)` (`bl` at
  0x581ac). A later call to `0x58e8c` uses `[sp-20]`, `[sp-21]:1`, `[sp-22]:1`, `[sp-21]:1` (`bl` at
  0x581c0). Reset timing units and the joined r0 result are unresolved (`ldmdb` at 0x581d0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x58160 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x58168 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x58170 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0x58174 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x58178 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x5817c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5818c | store | 4 | `sp -0x1c` | `ret(0x58bb8)` |
| `ldr` at 0x58190 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5819c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x581a0 | store | 4 | `sp -0x20` | `ret(0x58bb8)` |
| `ldrb` at 0x581b0 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x581b4 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x581b8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x581c8 | store | 4 | `sp -0x20` | `ret(0x58e8c)` |
| `ldr` at 0x581cc | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x581d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x581d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x581d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x58184 | `(unknown callee) 0x58bb8` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `(r1 & 0xff)` |
| `bl` at 0x581ac | `(unknown callee) 0x5c29c` | `0x2` | `?` | `?` | `ret(0x58bb8)` |
| `bl` at 0x581c0 | `(unknown callee) 0x58e8c` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x58198 | `ret(0x58bb8) == 0x0` | `ret(0x58bb8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x581d0 | `r0` | `?` |

## `vrmReset`

- `vrmReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5c708 to 0x5c748.
  Established. The result of `0x3d970` selects the taken arm on inequality with `0x1` and falls
  through on equality (`bl` at 0x5c720, `bne` at 0x5c72c). The fall-through call reaches `0x5c7a8`
  with `[sp-20]`, unknown, unknown, `ret(0x3d970)` (`bl` at 0x5c734). The taken arm instead calls
  `0x5c95c` with those same candidates (`bl` at 0x5c740). VRM reset mechanism and r0 at the common
  exit remain unknown (`ldmdb` at 0x5c744).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5c718 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x5c71c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5c730 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5c73c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x5c744 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5c744 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5c744 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5c720 | `(unknown callee) 0x3d970` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x5c734 | `(unknown callee) 0x5c7a8` | `[sp-20]` | `?` | `?` | `ret(0x3d970)` |
| `bl` at 0x5c740 | `(unknown callee) 0x5c95c` | `[sp-20]` | `?` | `?` | `ret(0x3d970)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5c72c | `ret(0x3d970) != 0x1` | `ret(0x3d970) == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5c744 | `r0` | `?` |
