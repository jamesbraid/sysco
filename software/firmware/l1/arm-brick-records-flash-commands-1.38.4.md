# ARM brick records and flash commands in 1.38.4

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
Computed transfers in cmdFlashBoot and cmdFlashSetDefault remain unresolved. Tables and return values cover only known edges, with excluded sites listed under the affected entries. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/brick-records-flash-commands-facts.txt) retain all tool sections.

## `L1CfgFindBrick`

- `L1CfgFindBrick`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6f784 to 0x6f8a0.
  Established. The entry-r0 guard takes its bypass for `r0 == 0x0` and falls through for nonzero
  (`beq` at 0x6f7c8). The earlier branch retains only `Z == 0` for taken and its negation for
  fall-through (`bne` at 0x6f7bc). The recognized return has unknown r0 (`ldmdb` at 0x6f89c). There
  are no calls in the facts. Record matching and field identities remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6f794 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6f798 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x6f79c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x6f7a0 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0x6f7a8 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x6f7b0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x6f7b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6f7c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f7cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f7d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6f7e0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f7f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f7fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6f800 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6f80c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f820 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f828 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6f82c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x6f83c | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x6f840 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6f848 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x6f850 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6f85c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f870 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6f878 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x6f87c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x6f884 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0x6f890 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x6f894 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x6f89c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6f89c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6f89c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x6f7bc | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x6f7c8 | `r0 == 0x0` | `r0 != 0x0` |
| `blt` at 0x6f7d8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x6f808 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x6f834 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x6f858 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6f89c | `r0` | `?` |

## `L1CfgAddBrick`

- `L1CfgAddBrick`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x70e7c to 0x711f8.
  Established. A word store writes `[sp+8]` at `ret(0x6f784) +0xc` (`str` at 0x70ed4). The prior
  call to `0x6f784` carries `[r0]`, `[r1]`, entry r3, `[sp]` (`bl` at 0x70eb4); its zero result
  takes the continuation and nonzero falls through (`beq` at 0x70ec8). Another word store writes
  `[[sp-32]]+16` at `[sp-32] +0x0` (`str` at 0x70f08). Configuration-record ownership and the merged
  return remain unresolved (`ldmdb` at 0x711e4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x70e8c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x70e90 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x70e94 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x70e98 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x70e9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x70ea0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70ea4 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0x70ea8 | load | 4 | `r1 +0x0` | `-` |
| `ldr` at 0x70eac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x70eb0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x70ebc | store | 4 | `sp -0x30` | `ret(0x6f784)` |
| `ldr` at 0x70ec0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x70ecc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x70ed0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x70ed4 | store | 4 | `ret(0x6f784) +0xc` | `[sp+8]` |
| `ldr` at 0x70edc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70ee0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x70ee8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x70eec | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x70ef8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x70efc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x70f00 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0x70f08 | store | 4 | `[sp-32] +0x0` | `[[sp-32]]+16` |
| `ldr` at 0x70f0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x70f10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x70f14 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x70f18 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x70f2c | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x70f3c | store | 4 | `[sp-24] +0x0` | `ret(0x1b2c8)` |
| `ldr` at 0x70f40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x70f44 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x70f48 | store | 4 | `sp -0x2c` | `[[sp-24]]` |
| `str` at 0x70f50 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x70f54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70f58 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x70f5c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x70f6c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x70f74 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x70f7c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x70f90 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x70f98 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70fa0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x70fb4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x70fbc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70fcc | load | 4 | `sp -0x28` | `-` |
| `str` at 0x70fd4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x70fdc | load | 4 | `abs 0x711e8` | `-` |
| `ldr` at 0x70fe0 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x70ff0 | load | 4 | `abs 0x711ec` | `-` |
| `ldr` at 0x70ff4 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x70ff8 | store | 4 | `sp -0x44` | `[sp]` |
| `ldr` at 0x70ffc | load | 4 | `sp +0x10` | `-` |
| `str` at 0x71000 | store | 4 | `sp -0x40` | `[sp+16]` |
| `ldr` at 0x71004 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x71008 | store | 4 | `sp -0x3c` | `[sp+12]` |
| `ldr` at 0x7100c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x71010 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x71014 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x71018 | store | 4 | `sp -0x34` | `[sp+8]` |
| `ldr` at 0x7101c | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x71020 | load | 4 | `abs 0x711f0` | `-` |
| `ldr` at 0x71024 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x71028 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x71030 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7103c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x71044 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71048 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x71060 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x71070 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71074 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x71078 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x71084 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x71098 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x710a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x710b8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x710c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x710c4 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x710c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x710e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x710fc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x71104 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x71108 | store | 4 | `(unknown)` | `[sp-36]` |
| `ldr` at 0x7110c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x71120 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x71128 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x7112c | store | 4 | `(unknown)` | `[sp]` |
| `ldr` at 0x71130 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x71144 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7114c | load | 4 | `sp +0x4` | `-` |
| `str` at 0x71150 | store | 4 | `(unknown)` | `[sp+4]` |
| `ldr` at 0x71154 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x71168 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x71170 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x71174 | store | 4 | `(unknown)` | `[sp+8]` |
| `ldr` at 0x71178 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7118c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x71194 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x71198 | store | 4 | `(unknown)` | `[sp+12]` |
| `ldr` at 0x7119c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x711b0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x711b8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x711bc | store | 4 | `(unknown)` | `[sp+16]` |
| `ldr` at 0x711c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x711c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x711c8 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x711d0 | store | 4 | `[sp-28] +0x0` | `[[sp-28]]+1` |
| `ldr` at 0x711d4 | load | 4 | `abs 0x711f4` | `-` |
| `ldr` at 0x711d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x711dc | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x711e0 | store | 4 | `abs 0x14864c` | `[[sp-28]]` |
| `ldmdb` at 0x711e4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x711e4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x711e4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x711e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x70eb4 | `(unknown callee) 0x6f784` | `[r0]` | `[r1]` | `r3` | `[sp]` |
| `bl` at 0x70f34 | `(unknown callee) 0x1b2c8` | `[[sp-24]]` | `?` | `[[sp-32]]` | `?` |
| `bl` at 0x7102c | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f6ec` | `[[sp-24]]` | `[sp-36]` |
| `bl` at 0x71068 | `(unknown callee) 0x1b16c` | `?` | `?` | `?` | `?` |
| `bl` at 0x710e4 | `(unknown callee) 0x1b16c` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x70ec8 | `ret(0x6f784) == 0x0` | `ret(0x6f784) != 0x0` |
| `ble` at 0x70ef4 | `[[sp-28]]+1 <= [[sp-32]] (signed, 32-bit)` | `[[sp-28]]+1 > [[sp-32]] (signed, 32-bit)` |
| `blt` at 0x70f64 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blo` at 0x70fc8 | `? < ? (unsigned, 32-bit)` | `? >= ? (unsigned, 32-bit)` |
| `beq` at 0x70fec | `? == 0x0` | `? != 0x0` |
| `bne` at 0x71038 | `Z == 0` | `not (Z == 0)` |
| `bge` at 0x71080 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x711e4 | `r0` | `?` |

## `L1CfgGetBrickType`

- `L1CfgGetBrickType`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x74680 to 0x74708.
  Established. The call to `0x6f784` uses `[0x1489c0]`, `[0x1489b8]`, `[sp-20]`, `[sp-24]` (`bl` at
  0x746b8); its zero result takes the branch and nonzero falls through (`beq` at 0x746cc). The exit
  carries r0 `[sp-32]` (`ldmdb` at 0x746fc). Type-code interpretation is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x74690 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x74694 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x74698 | load | 4 | `abs 0x74700` | `-` |
| `ldr` at 0x746a0 | load | 4 | `abs 0x74704` | `-` |
| `ldr` at 0x746a4 | load | 4 | `abs 0x74704` | `-` |
| `ldr` at 0x746a8 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x746ac | load | 4 | `abs 0x1489b8` | `-` |
| `ldr` at 0x746b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x746b4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x746c0 | store | 4 | `sp -0x1c` | `ret(0x6f784)` |
| `ldr` at 0x746c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x746d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x746d4 | load | 4 | `ret(0x6f784) +0x8` | `-` |
| `str` at 0x746d8 | store | 4 | `sp -0x24` | `[ret(0x6f784)+8]` |
| `str` at 0x746e4 | store | 4 | `sp -0x24` | `0x3f` |
| `ldr` at 0x746e8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x746ec | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x746f4 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x746fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x746fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x746fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7469c | `(unknown callee) 0x6c0d0` | `0x10fc88` | `r1` | `r2` | `r3` |
| `bl` at 0x746b8 | `(unknown callee) 0x6f784` | `[0x1489c0]` | `[0x1489b8]` | `[sp-20]` | `[sp-24]` |
| `bl` at 0x746f0 | `(unknown callee) 0x6c168` | `ret(0x6f784)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x746cc | `ret(0x6f784) == 0x0` | `ret(0x6f784) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x746fc | `r0` | `[sp-32]` |

## `L1CfgGetBrickCfg`

- `L1CfgGetBrickCfg`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x74708 to 0x74758.
  Established. The single call reaches `0x6f8a0` with `[0x1489cc]`, `[0x1489c4]`, entry r0, entry r1
  (`bl` at 0x74740); its result reaches r0 at exit (`ldmdb` at 0x74750). No conditional branches are
  reported. Configuration fields have no established meaning here.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x74718 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x7471c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x74724 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x74728 | load | 4 | `abs 0x74754` | `-` |
| `ldr` at 0x7472c | load | 4 | `abs 0x74754` | `-` |
| `ldr` at 0x74730 | load | 4 | `abs 0x1489cc` | `-` |
| `ldr` at 0x74734 | load | 4 | `abs 0x1489c4` | `-` |
| `ldr` at 0x74738 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7473c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x74744 | store | 4 | `sp -0x1c` | `ret(0x6f8a0)` |
| `ldr` at 0x74748 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x74750 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x74750 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x74750 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x74740 | `(unknown callee) 0x6f8a0` | `[0x1489cc]` | `[0x1489c4]` | `r0` | `r1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x74750 | `r0` | `ret(0x6f8a0)` |

## `L1CfgGetBrickIndex`

- `L1CfgGetBrickIndex`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x74758 to 0x74850.
  Established. The first conditional branch takes `Z == 0` and falls through on its negation (`bne`
  at 0x74788). The next retains only `lt flags condition (operands unknown)` and its negation (`blt`
  at 0x7479c). No calls appear in the facts. The return is `[sp-36]` (`ldmdb` at 0x74848). Index
  bounds and matching fields remain unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x74768 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x7476c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x74774 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x7477c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x74780 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7478c | load | 4 | `abs 0x7484c` | `-` |
| `ldr` at 0x74790 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74794 | load | 4 | `abs 0x1489b8` | `-` |
| `ldr` at 0x747a4 | load | 4 | `abs 0x7484c` | `-` |
| `ldr` at 0x747a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x747bc | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x747c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x747c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x747d4 | load | 4 | `abs 0x7484c` | `-` |
| `ldr` at 0x747d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x747ec | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x747f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x747f8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x74808 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x7480c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x74814 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x7481c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x74828 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x74830 | store | 4 | `sp -0x24` | `?` |
| `str` at 0x7483c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x74840 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x74848 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x74848 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x74848 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x74788 | `Z == 0` | `not (Z == 0)` |
| `blt` at 0x7479c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x747d0 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x74800 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x74824 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x74848 | `r0` | `[sp-36]` |

## `L1CfgGetIndexBrick`

- `L1CfgGetIndexBrick`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x74850 to 0x748c4.
  Established. The signed lower-bound test takes its bypass for `r0 < 0x0` and falls through for
  greater or equal (`blt` at 0x7486c). The next signed test takes the same bypass for `r0 >=
  [0x1489b8]` and falls through for less (`bge` at 0x74880). R0 at the common exit remains
  unresolved (`ldmdb` at 0x748bc). Physical brick identity is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x74860 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x74864 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x74870 | load | 4 | `abs 0x748c0` | `-` |
| `ldr` at 0x74874 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x74878 | load | 4 | `abs 0x1489b8` | `-` |
| `str` at 0x7488c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x74894 | load | 4 | `abs 0x748c0` | `-` |
| `ldr` at 0x74898 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x748ac | load | 4 | `abs 0x1489c0` | `-` |
| `str` at 0x748b4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x748b8 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x748bc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x748bc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x748bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x7486c | `r0 < 0x0 (signed, 32-bit)` | `r0 >= 0x0 (signed, 32-bit)` |
| `bge` at 0x74880 | `r0 >= [0x1489b8] (signed, 32-bit)` | `r0 < [0x1489b8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x748bc | `r0` | `?` |

## `L1CfgGetBrick`

- `L1CfgGetBrick`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x748c4 to 0x74908.
  Established. The call to `0x6f784` has `[0x1489c0]`, `[0x1489b8]`, entry r0 and entry r1 (`bl` at
  0x748f4). The exit propagates `ret(0x6f784)` (`ldmdb` at 0x74900). Record contents and caller
  ownership are unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x748d4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x748d8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x748dc | load | 4 | `abs 0x74904` | `-` |
| `ldr` at 0x748e0 | load | 4 | `abs 0x74904` | `-` |
| `ldr` at 0x748e4 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x748e8 | load | 4 | `abs 0x1489b8` | `-` |
| `ldr` at 0x748ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x748f0 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x74900 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x74900 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x74900 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x748f4 | `(unknown callee) 0x6f784` | `[0x1489c0]` | `[0x1489b8]` | `r0` | `r1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x74900 | `r0` | `ret(0x6f784)` |

## `L1CfgBrickCount`

- `L1CfgBrickCount`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x74b78 to 0x74b98.
  Established. The word load reads absolute `0x1489b8` (`ldr` at 0x74b88). The tool reports
  `[0x1489b8]` as exit r0 (`ldmdb` at 0x74b90). No calls or conditional branches occur in the facts.
  The counted population remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x74b84 | load | 4 | `abs 0x74b94` | `-` |
| `ldr` at 0x74b88 | load | 4 | `abs 0x1489b8` | `-` |
| `ldmdb` at 0x74b90 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x74b90 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x74b90 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x74b90 | `r0` | `[0x1489b8]` |

## `FlashValidateImage`

- `FlashValidateImage`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa4bb8 to 0xa5048.
  Established. A byte store clears absolute `0x149914` (`strb` at 0xa4bf0). The call to `0x1b49c`
  uses `0x149914`, `0x126080`, `[sp-20]`, `[sp-20]` (`bl` at 0xa4c0c). A later unsigned comparison
  takes the branch for `ret(0x1af68) < [sp-24]` and falls through for greater or equal (`blo` at
  0xa4c34). A word store writes `0x149914` at `[sp] +0x0` (`str` at 0xa4fbc). Image-field format and
  r0 at the joined exit are unresolved (`ldmdb` at 0xa5010).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa4bc8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa4bcc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa4bd0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xa4bd4 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xa4bd8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa4bdc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa4be4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xa4be8 | load | 4 | `abs 0xa5014` | `-` |
| `strb` at 0xa4bf0 | store | 1 | `abs 0x149914` | `0x0` |
| `ldr` at 0xa4bf4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa4c00 | load | 4 | `abs 0xa5014` | `-` |
| `ldr` at 0xa4c04 | load | 4 | `abs 0xa5018` | `-` |
| `ldr` at 0xa4c08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa4c14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa4c18 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xa4c24 | store | 4 | `sp -0x2c` | `ret(0x1af68)` |
| `ldr` at 0xa4c28 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa4c2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4c38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4c3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa4c44 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa4c54 | load | 4 | `abs 0xa5014` | `-` |
| `ldr` at 0xa4c58 | load | 4 | `abs 0xa501c` | `-` |
| `ldr` at 0xa4c5c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa4c68 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa4c7c | load | 4 | `abs 0xa5014` | `-` |
| `ldr` at 0xa4c80 | load | 4 | `abs 0xa5020` | `-` |
| `ldr` at 0xa4c84 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa4c90 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa4c94 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa4c9c | store | 4 | `sp -0x3c` | `([sp-44] + [sp-36])` |
| `ldr` at 0xa4ca0 | load | 4 | `sp -0x3c` | `-` |
| `ldrh` at 0xa4ca4 | load | 2 | `([sp-44] + [sp-36]) +0x2` | `-` |
| `str` at 0xa4cbc | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xa4cc0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa4ccc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa4cd0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa4cd8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4cdc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa4cf0 | load | 4 | `abs 0xa5014` | `-` |
| `ldr` at 0xa4cf4 | load | 4 | `abs 0xa5024` | `-` |
| `ldr` at 0xa4cf8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa4d04 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa4d08 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa4d24 | load | 4 | `abs 0xa5014` | `-` |
| `ldr` at 0xa4d28 | load | 4 | `abs 0xa5028` | `-` |
| `ldr` at 0xa4d2c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xa4d3c | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0xa4d44 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xa4d48 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xa4d4c | store | 4 | `sp -0x30` | `[sp-60]` |
| `ldr` at 0xa4d50 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa4d54 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa4d64 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa4d68 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa4d78 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa4d80 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x1af68))` |
| `ldr` at 0xa4d84 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xa4d8c | store | 4 | `sp -0x38` | `[sp-56]+4` |
| `ldr` at 0xa4d90 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xa4d98 | store | 4 | `sp -0x30` | `[sp-48]+4` |
| `ldr` at 0xa4da0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa4dac | load | 4 | `abs 0xa5014` | `-` |
| `ldr` at 0xa4db0 | load | 4 | `abs 0xa502c` | `-` |
| `ldr` at 0xa4dbc | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xa4dc8 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xa4dcc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xa4dd0 | store | 4 | `[sp+4] +0x0` | `[sp-60]` |
| `ldr` at 0xa4dd4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa4dd8 | load | 4 | `[sp-60] +0x18` | `-` |
| `str` at 0xa4de4 | store | 4 | `sp -0x2c` | `ret(0x1af68)` |
| `ldr` at 0xa4de8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa4df4 | load | 4 | `abs 0xa5014` | `-` |
| `ldr` at 0xa4df8 | load | 4 | `abs 0xa5030` | `-` |
| `ldr` at 0xa4e04 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa4e08 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa4e10 | store | 4 | `sp -0x40` | `(ret(0x1af68) + [sp-36])` |
| `ldr` at 0xa4e14 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa4e18 | store | 4 | `sp -0x30` | `[sp-20]` |
| `str` at 0xa4e20 | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0xa4e28 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xa4e2c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa4e30 | load | 4 | `[sp-60] +0xc` | `-` |
| `ldr` at 0xa4e40 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa4e50 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa4e54 | load | 4 | `[sp-48] +0x0` | `-` |
| `ldr` at 0xa4e64 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa4e6c | store | 4 | `sp -0x28` | `([sp-40] + ret(0x1af68))` |
| `ldr` at 0xa4e70 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xa4e78 | store | 4 | `sp -0x38` | `[sp-56]+4` |
| `ldr` at 0xa4e7c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xa4e84 | store | 4 | `sp -0x30` | `[sp-48]+4` |
| `ldr` at 0xa4e8c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa4e90 | load | 4 | `[sp-60] +0x20` | `-` |
| `ldr` at 0xa4ea0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa4eac | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa4eb0 | load | 4 | `[sp-60] +0x20` | `-` |
| `ldr` at 0xa4ec4 | load | 4 | `abs 0xa5014` | `-` |
| `ldr` at 0xa4ec8 | load | 4 | `abs 0xa5034` | `-` |
| `str` at 0xa4ed4 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xa4edc | load | 4 | `abs 0xa5014` | `-` |
| `ldr` at 0xa4ee0 | load | 4 | `abs 0xa5038` | `-` |
| `ldr` at 0xa4eec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa4ef8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa4efc | load | 4 | `[sp-64] +0x4` | `-` |
| `str` at 0xa4f08 | store | 4 | `sp -0x34` | `ret(0x1af68)` |
| `str` at 0xa4f10 | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0xa4f18 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xa4f1c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa4f20 | store | 4 | `sp -0x30` | `[sp-64]` |
| `ldr` at 0xa4f24 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa4f28 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa4f38 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa4f3c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa4f4c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa4f54 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x1af68))` |
| `ldr` at 0xa4f58 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xa4f60 | store | 4 | `sp -0x38` | `[sp-56]+4` |
| `ldr` at 0xa4f64 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xa4f6c | store | 4 | `sp -0x30` | `[sp-48]+4` |
| `ldr` at 0xa4f74 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa4f80 | load | 4 | `abs 0xa5014` | `-` |
| `ldr` at 0xa4f84 | load | 4 | `abs 0xa503c` | `-` |
| `ldr` at 0xa4f90 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xa4f9c | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xa4fa0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa4fa4 | store | 4 | `[sp+8] +0x0` | `[sp-64]` |
| `ldr` at 0xa4fa8 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xa4fb4 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xa4fb8 | load | 4 | `abs 0xa5014` | `-` |
| `str` at 0xa4fbc | store | 4 | `[sp] +0x0` | `0x149914` |
| `str` at 0xa4fc4 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xa4fcc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa4fd8 | load | 4 | `abs 0xa5040` | `-` |
| `ldr` at 0xa4fdc | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0xa4fe0 | load | 4 | `abs 0xa5044` | `-` |
| `ldr` at 0xa4fe4 | load | 4 | `abs 0xa5014` | `-` |
| `ldr` at 0xa4fec | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xa4ff8 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xa4ffc | load | 4 | `abs 0xa5014` | `-` |
| `str` at 0xa5000 | store | 4 | `[sp] +0x0` | `0x149914` |
| `str` at 0xa5008 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xa500c | load | 4 | `sp -0x44` | `-` |
| `ldmdb` at 0xa5010 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa5010 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa5010 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa4c0c | `(unknown callee) 0x1b49c` | `0x149914` | `0x126080` | `[sp-20]` | `[sp-20]` |
| `bl` at 0xa4c1c | `(unknown callee) 0x1af68` | `[[sp-20]]` | `r1` | `0x149914` | `[sp-20]` |
| `bl` at 0xa4c60 | `(unknown callee) 0x1b49c` | `0x149914` | `0x1260a0` | `ret(0x1af68)` | `?` |
| `bl` at 0xa4c6c | `(unknown callee) 0xcb21c` | `ret(0x1af68)` | `?` | `([sp-24] + [sp-28])` | `ret(0x1af68)` |
| `bl` at 0xa4c88 | `(unknown callee) 0x1b49c` | `0x149914` | `0x1260c0` | `[sp-44]` | `ret(0xcb21c)` |
| `bl` at 0xa4cac | `(unknown callee) 0x1b208` | `[([sp-44] + [sp-36])+2]:2` | `?` | `[sp-44]` | `[([sp-44] + [sp-36])+2]:2` |
| `bl` at 0xa4cfc | `(unknown callee) 0x1b49c` | `0x149914` | `0x1260fc` | `[sp-52]` | `?` |
| `bl` at 0xa4d14 | `(unknown callee) 0xcb21c` | `([sp-44] + [sp-52])` | `([sp-44] + [sp-52])` | `[sp-44]` | `([sp-44] + [sp-52])` |
| `bl` at 0xa4d30 | `(unknown callee) 0x1b49c` | `0x149914` | `0x126120` | `[sp-52]` | `ret(0xcb21c)` |
| `bl` at 0xa4d6c | `(unknown callee) 0x1af68` | `?` | `?` | `?` | `?` |
| `bl` at 0xa4db4 | `(unknown callee) 0x1b508` | `0x149914` | `0x126168` | `?` | `?` |
| `bl` at 0xa4ddc | `(unknown callee) 0x1af68` | `[[sp-60]+24]` | `?` | `?` | `[sp-60]` |
| `bl` at 0xa4dfc | `(unknown callee) 0x1b508` | `0x149914` | `0x126184` | `?` | `ret(0x1af68)` |
| `bl` at 0xa4e34 | `(unknown callee) 0x1af68` | `[[sp-60]+12]` | `?` | `ret(0x1af68)` | `[sp-60]` |
| `bl` at 0xa4e58 | `(unknown callee) 0x1af68` | `[[sp-48]]` | `?` | `ret(0x1af68)` | `[sp-48]` |
| `bl` at 0xa4e94 | `(unknown callee) 0x1af68` | `[[sp-60]+32]` | `?` | `ret(0x1af68)` | `[sp-60]` |
| `bl` at 0xa4eb4 | `(unknown callee) 0x1af68` | `[[sp-60]+32]` | `?` | `ret(0x1af68)` | `[sp-60]` |
| `bl` at 0xa4ecc | `(unknown callee) 0x1b508` | `0x149914` | `0x1261a0` | `?` | `ret(0x1af68)` |
| `bl` at 0xa4ee4 | `(unknown callee) 0x1b508` | `0x149914` | `0x1261b4` | `?` | `ret(0x1af68)` |
| `bl` at 0xa4f00 | `(unknown callee) 0x1af68` | `[[sp-64]+4]` | `?` | `?` | `[sp-64]` |
| `bl` at 0xa4f40 | `(unknown callee) 0x1af68` | `?` | `?` | `?` | `?` |
| `bl` at 0xa4f88 | `(unknown callee) 0x1b508` | `0x149914` | `0x1261c8` | `?` | `?` |
| `bl` at 0xa4fe8 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x1261e8` | `0x149914` | `0x144dc8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa4bfc | `Z == 0` | `not (Z == 0)` |
| `blo` at 0xa4c34 | `ret(0x1af68) < [sp-24] (unsigned, 32-bit)` | `ret(0x1af68) >= [sp-24] (unsigned, 32-bit)` |
| `bhi` at 0xa4c4c | `ret(0x1af68) > ([sp-24] + [sp-28]) (unsigned, 32-bit)` | `ret(0x1af68) <= ([sp-24] + [sp-28]) (unsigned, 32-bit)` |
| `bne` at 0xa4c78 | `Z == 0` | `not (Z == 0)` |
| `bls` at 0xa4cc8 | `[sp-52] <= 0x1f (unsigned, 32-bit)` | `[sp-52] > 0x1f (unsigned, 32-bit)` |
| `bhi` at 0xa4ce8 | `([sp-44] + [sp-52]) > ([sp-24] + [sp-28]) (unsigned, 32-bit)` | `([sp-44] + [sp-52]) <= ([sp-24] + [sp-28]) (unsigned, 32-bit)` |
| `bne` at 0xa4d20 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0xa4d5c | `C == 0` | `not (C == 0)` |
| `beq` at 0xa4da8 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xa4dc4 | `[sp+4] == 0x0` | `[sp+4] != 0x0` |
| `bne` at 0xa4df0 | `ret(0x1af68) != 0x0` | `ret(0x1af68) == 0x0` |
| `blo` at 0xa4e48 | `[sp-56] < ret(0x1af68) (unsigned, 32-bit)` | `[sp-56] >= ret(0x1af68) (unsigned, 32-bit)` |
| `beq` at 0xa4ea8 | `[sp-40] == ret(0x1af68)` | `[sp-40] != ret(0x1af68)` |
| `bne` at 0xa4ec0 | `ret(0x1af68) != 0x0` | `ret(0x1af68) == 0x0` |
| `beq` at 0xa4ef4 | `[sp-64] == 0x0` | `[sp-64] != 0x0` |
| `blo` at 0xa4f30 | `C == 0` | `not (C == 0)` |
| `beq` at 0xa4f7c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xa4f98 | `[sp+8] == 0x0` | `[sp+8] != 0x0` |
| `beq` at 0xa4fb0 | `[sp] == 0x0` | `[sp] != 0x0` |
| `beq` at 0xa4fd4 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `beq` at 0xa4ff4 | `[sp] == 0x0` | `[sp] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa5010 | `r0` | `[sp-68]` |

## `cmdFlashBoot`

- `cmdFlashBoot`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa5b68 to 0xa62b4.
  Established. The opening call to `0x1b124` carries `0x300`, entry r1, entry r2, `[[r0+112]]` (`bl`
  at 0xa5b98). A word store writes its return at `r1 +0x0`, followed by a byte store clearing
  `[[sp-28]] +0x0` (`str` at 0xa5ba0, `strb` at 0xa5bb0). The computed branch takes its unresolved
  edge for unsigned `[sp-36]+3 <= 0xa` and falls through for greater (`ldrls` at 0xa5bd4). The
  known-edge exit reports r0 `0x0` (`ldmdb` at 0xa625c). Boot selection and the excluded command
  arms remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa5b78 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xa5b7c | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xa5b80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa5b84 | load | 4 | `r0 +0x70` | `-` |
| `ldr` at 0xa5b88 | load | 4 | `[r0+112] +0x0` | `-` |
| `str` at 0xa5b8c | store | 4 | `sp -0x24` | `[[r0+112]]` |
| `ldr` at 0xa5b90 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa5ba0 | store | 4 | `r1 +0x0` | `ret(0x1b124)` |
| `ldr` at 0xa5ba4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa5ba8 | load | 4 | `[sp-28] +0x0` | `-` |
| `strb` at 0xa5bb0 | store | 1 | `[[sp-28]] +0x0` | `0x0` |
| `ldr` at 0xa5bb4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa5bc8 | load | 4 | `sp -0x24` | `-` |
| `ldrls` at 0xa5bd4 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0xa625c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xa625c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa625c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa625c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa5b98 | `(unknown callee) 0x1b124` | `0x300` | `r1` | `r2` | `[[r0+112]]` |
| `bl` at 0xa5bc4 | `(unknown callee) 0xa6718` | `0x64` | `?` | `[[sp-28]]` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xa5bbc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ldrls` at 0xa5bd4 | `[sp-36]+3 <= 0xa (unsigned, 32-bit)` | `[sp-36]+3 > 0xa (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa625c | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xa5bd4).

## `cmdFlashVersion`

- `cmdFlashVersion`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa62b4 to 0xa64d0.
  Established. The word store at `r1 +0x0` writes `ret(0x1b124)` (`str` at 0xa63cc). That call has
  `0x80`, `[0x2b36b0]`, `0x2b36a0`, `[sp-108]` (`bl` at 0xa63c4). The earlier global comparison
  takes its branch for `[0x2b36b0] != [0x2b36a8]` and falls through on equality (`bne` at 0xa6394).
  The common return reports r0 `0x0` (`ldmdb` at 0xa64b8). Version-record fields remain
  unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa62c4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xa62c8 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xa62cc | load | 4 | `abs 0xa64bc` | `-` |
| `ldrb` at 0xa62d0 | load | 1 | `abs 0x1263dc` | `-` |
| `strb` at 0xa62d4 | store | 1 | `sp -0x2c` | `([0x1263dc]:1 & 0xff)` |
| `strb` at 0xa62e0 | store | 1 | `sp -0x2b` | `0x0` |
| `strb` at 0xa62ec | store | 1 | `sp -0x2a` | `0x0` |
| `strb` at 0xa62f8 | store | 1 | `sp -0x29` | `0x0` |
| `strb` at 0xa6304 | store | 1 | `sp -0x28` | `0x0` |
| `strb` at 0xa6310 | store | 1 | `sp -0x27` | `0x0` |
| `strb` at 0xa631c | store | 1 | `sp -0x26` | `0x0` |
| `strb` at 0xa6328 | store | 1 | `sp -0x25` | `0x0` |
| `strb` at 0xa6334 | store | 1 | `sp -0x24` | `0x0` |
| `strb` at 0xa6340 | store | 1 | `sp -0x23` | `0x0` |
| `strb` at 0xa634c | store | 1 | `sp -0x22` | `0x0` |
| `strb` at 0xa6358 | store | 1 | `sp -0x21` | `0x0` |
| `strb` at 0xa6364 | store | 1 | `sp -0x20` | `0x0` |
| `strb` at 0xa6370 | store | 1 | `sp -0x1f` | `0x0` |
| `strb` at 0xa637c | store | 1 | `sp -0x1e` | `0x0` |
| `ldr` at 0xa6380 | load | 4 | `abs 0xa64c0` | `-` |
| `ldr` at 0xa6384 | load | 4 | `abs 0xa64c0` | `-` |
| `ldr` at 0xa6388 | load | 4 | `abs 0x2b36b0` | `-` |
| `ldr` at 0xa638c | load | 4 | `abs 0x2b36a8` | `-` |
| `ldr` at 0xa6398 | load | 4 | `abs 0xa64c0` | `-` |
| `ldr` at 0xa639c | load | 4 | `abs 0x2b36a8` | `-` |
| `str` at 0xa63a0 | store | 4 | `sp -0x6c` | `[0x2b36a8]` |
| `ldr` at 0xa63a8 | load | 4 | `abs 0xa64c0` | `-` |
| `ldr` at 0xa63ac | load | 4 | `abs 0x2b36ac` | `-` |
| `str` at 0xa63b0 | store | 4 | `sp -0x6c` | `[0x2b36ac]` |
| `ldr` at 0xa63b4 | load | 4 | `sp -0x6c` | `-` |
| `str` at 0xa63b8 | store | 4 | `sp -0x30` | `[sp-108]` |
| `ldr` at 0xa63bc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa63cc | store | 4 | `r1 +0x0` | `ret(0x1b124)` |
| `ldr` at 0xa63d4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa63ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa63f0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xa63f4 | load | 4 | `abs 0xa64c4` | `-` |
| `ldr` at 0xa63f8 | load | 4 | `abs 0xa64c8` | `-` |
| `str` at 0xa6404 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0xa640c | load | 4 | `sp -0x50` | `-` |
| `str` at 0xa6410 | store | 4 | `sp -0x64` | `[sp-80]` |
| `ldr` at 0xa6414 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa6418 | store | 4 | `sp -0x70` | `[sp-28]` |
| `ldr` at 0xa641c | load | 4 | `abs 0xa64c0` | `-` |
| `ldr` at 0xa6420 | load | 4 | `abs 0xa64c0` | `-` |
| `ldr` at 0xa6424 | load | 4 | `abs 0x2b36b0` | `-` |
| `ldr` at 0xa6428 | load | 4 | `abs 0x2b36a8` | `-` |
| `str` at 0xa6438 | store | 4 | `sp -0x74` | `0x41` |
| `str` at 0xa6444 | store | 4 | `sp -0x74` | `0x42` |
| `ldr` at 0xa6448 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xa6450 | store | 4 | `sp -0x90` | `[sp-80]+4` |
| `ldr` at 0xa6454 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xa645c | store | 4 | `sp -0x8c` | `[sp-80]+6` |
| `ldr` at 0xa6460 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xa6464 | store | 4 | `sp -0x88` | `[sp-80]` |
| `ldr` at 0xa6468 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xa6470 | store | 4 | `sp -0x84` | `[sp-80]+8` |
| `ldr` at 0xa6474 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xa647c | store | 4 | `sp -0x80` | `[sp-80]+10` |
| `ldr` at 0xa6480 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xa6488 | store | 4 | `sp -0x7c` | `[sp-80]+12` |
| `ldr` at 0xa648c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xa6490 | store | 4 | `sp -0x78` | `[sp-52]` |
| `ldr` at 0xa6494 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0xa6498 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xa649c | load | 4 | `abs 0xa64cc` | `-` |
| `ldr` at 0xa64a0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa64a4 | load | 4 | `sp -0x74` | `-` |
| `str` at 0xa64b0 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0xa64b4 | load | 4 | `sp -0x68` | `-` |
| `ldmdb` at 0xa64b8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xa64b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa64b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa64b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa63c4 | `(unknown callee) 0x1b124` | `0x80` | `[0x2b36b0]` | `0x2b36a0` | `[sp-108]` |
| `bl` at 0xa63dc | `(unknown callee) 0xc5764` | `[sp-48]` | `sp-96` | `?` | `sp-96` |
| `bl` at 0xa63fc | `(unknown callee) 0x1b49c` | `[[sp-28]]` | `0x12653c` | `0x126570` | `[sp-28]` |
| `bl` at 0xa64a8 | `(unknown callee) 0x1b49c` | `[[sp-28]]` | `0x126578` | `[sp-76]` | `[sp-116]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa6394 | `[0x2b36b0] != [0x2b36a8]` | `[0x2b36b0] == [0x2b36a8]` |
| `beq` at 0xa63e8 | `ret(0xc5764) == 0x0` | `ret(0xc5764) != 0x0` |
| `bne` at 0xa6430 | `[0x2b36b0] != [0x2b36a8]` | `[0x2b36b0] == [0x2b36a8]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa64b8 | `r0` | `0x0` |

## `cmdFlashSetDefault`

- `cmdFlashSetDefault`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa64d0 to 0xa6718.
  Established. A word store saves `ret(0x1b124)` at `r1 +0x0` (`str` at 0xa6510). A byte store clears
  `[[sp-28]] +0x0` before the computed selector (`strb` at 0xa6520). The later byte clear at that
  target (`strb` at 0xa66f8) follows the taken branch for `ret(0xcbb10) == 0x0`. Nonzero falls
  through to the other arm (`beq` at 0xa66c0).
  The computed selector takes the unresolved edge for unsigned `[sp-36]+1 <= 0x5` and falls through for
  greater (`ldrls` at 0xa6530). The known-edge call to `0xcbb10` uses `[sp-40]`, unknown,
  `[[sp-28]]`, `[sp-36]+1` (`bl` at 0xa66ac). Default-bank semantics and r0 at exit remain unknown
  (`ldmdb` at 0xa6708).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa64e0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xa64e4 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xa64e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa64ec | load | 4 | `r0 +0x70` | `-` |
| `ldr` at 0xa64f0 | load | 4 | `[r0+112] +0x0` | `-` |
| `str` at 0xa64f4 | store | 4 | `sp -0x24` | `[[r0+112]]` |
| `str` at 0xa64fc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xa6500 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa6510 | store | 4 | `r1 +0x0` | `ret(0x1b124)` |
| `ldr` at 0xa6514 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa6518 | load | 4 | `[sp-28] +0x0` | `-` |
| `strb` at 0xa6520 | store | 1 | `[[sp-28]] +0x0` | `0x0` |
| `ldr` at 0xa6524 | load | 4 | `sp -0x24` | `-` |
| `ldrls` at 0xa6530 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa66a8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa66b4 | store | 4 | `sp -0x20` | `ret(0xcbb10)` |
| `ldr` at 0xa66b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa66c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa66d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa66d4 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xa66d8 | load | 4 | `abs 0xa6714` | `-` |
| `str` at 0xa66e4 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xa66ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa66f0 | load | 4 | `[sp-28] +0x0` | `-` |
| `strb` at 0xa66f8 | store | 1 | `[[sp-28]] +0x0` | `0x0` |
| `str` at 0xa6700 | store | 4 | `sp -0x90` | `0x0` |
| `ldr` at 0xa6704 | load | 4 | `sp -0x90` | `-` |
| `ldmdb` at 0xa6708 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xa6708 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa6708 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa6708 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa6508 | `(unknown callee) 0x1b124` | `0x80` | `r1` | `r2` | `0x0` |
| `bl` at 0xa66ac | `(unknown callee) 0xcbb10` | `[sp-40]` | `?` | `[[sp-28]]` | `[sp-36]+1` |
| `bl` at 0xa66c8 | `(unknown callee) 0x21674` | `ret(0xcbb10)` | `?` | `?` | `ret(0xcbb10)` |
| `bl` at 0xa66dc | `(unknown callee) 0x1b49c` | `[[sp-28]]` | `0x1265e8` | `ret(0x21674)` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xa6530 | `[sp-36]+1 <= 0x5 (unsigned, 32-bit)` | `[sp-36]+1 > 0x5 (unsigned, 32-bit)` |
| `beq` at 0xa66c0 | `ret(0xcbb10) == 0x0` | `ret(0xcbb10) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa6708 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xa6530).

## `PBaySdrFlashInfo`

- `PBaySdrFlashInfo`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xadef0 to 0xae090.
  Established. The opening call to `0xaa7dc` receives entry r0, `sp-428`, `0xfffffe64`, `sp-428`
  (`bl` at 0xadf4c); its zero result takes the bypass and nonzero falls through (`beq` at 0xadf60).
  A later call to `0x1b3dc` carries `sp-436` and two unknown candidates followed by `sp-436` (`bl`
  at 0xadfb0). The return reports `[sp-524]` (`ldmdb` at 0xae080). Flash-information record fields
  and units remain unestablished.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xadf00 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xadf04 | load | 4 | `abs 0xae084` | `-` |
| `ldr` at 0xadf08 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xadf0c | load | 2 | `r0 +0x6` | `-` |
| `strh` at 0xadf14 | store | 2 | `sp -0x1ac` | `([r0+6]:2 & 0xffff)` |
| `str` at 0xadf1c | store | 4 | `sp -0x1a8` | `0xc0` |
| `str` at 0xadf24 | store | 4 | `sp -0x20` | `0x3` |
| `str` at 0xadf2c | store | 4 | `sp -0x1c` | `0x3` |
| `str` at 0xadf34 | store | 4 | `sp -0x114` | `0x0` |
| `str` at 0xadf3c | store | 4 | `sp -0x110` | `0xff` |
| `ldr` at 0xadf44 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xadf54 | store | 4 | `sp -0x1b0` | `ret(0xaa7dc)` |
| `ldr` at 0xadf58 | load | 4 | `sp -0x1b0` | `-` |
| `ldr` at 0xadf64 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xadf68 | store | 4 | `sp -0x214` | `[sp-36]` |
| `ldr` at 0xadf6c | load | 4 | `sp -0x214` | `-` |
| `ldrb` at 0xadf70 | load | 1 | `[sp-36] +0x5` | `-` |
| `ldr` at 0xadf78 | load | 4 | `sp -0x214` | `-` |
| `ldrb` at 0xadf7c | load | 1 | `[sp-36] +0x4` | `-` |
| `ldr` at 0xadf88 | load | 4 | `sp -0x214` | `-` |
| `ldrb` at 0xadf8c | load | 1 | `[sp-36] +0x3` | `-` |
| `ldr` at 0xadf98 | load | 4 | `sp -0x214` | `-` |
| `ldrb` at 0xadf9c | load | 1 | `[sp-36] +0x2` | `-` |
| `str` at 0xadfa4 | store | 4 | `sp -0x1b4` | `?` |
| `str` at 0xadfb8 | store | 4 | `sp -0x210` | `ret(0x1b3dc)` |
| `ldr` at 0xadfc0 | load | 4 | `sp -0x214` | `-` |
| `ldrb` at 0xadfc4 | load | 1 | `[sp-532] +0x1` | `-` |
| `ldr` at 0xadfc8 | load | 4 | `sp -0x214` | `-` |
| `ldrb` at 0xadfcc | load | 1 | `[sp-532] +0x0` | `-` |
| `ldr` at 0xadfd0 | load | 4 | `sp -0x210` | `-` |
| `ldr` at 0xadfd4 | load | 4 | `ret(0x1b3dc) +0x10` | `-` |
| `str` at 0xadfdc | store | 4 | `sp -0x22c` | `[ret(0x1b3dc)+16]+1` |
| `ldr` at 0xadfe0 | load | 4 | `sp -0x210` | `-` |
| `ldr` at 0xadfe4 | load | 4 | `ret(0x1b3dc) +0xc` | `-` |
| `str` at 0xadfe8 | store | 4 | `sp -0x228` | `[ret(0x1b3dc)+12]` |
| `ldr` at 0xadfec | load | 4 | `sp -0x210` | `-` |
| `ldr` at 0xadff0 | load | 4 | `ret(0x1b3dc) +0x14` | `-` |
| `str` at 0xadffc | store | 4 | `sp -0x224` | `[ret(0x1b3dc)+20]+1900` |
| `ldr` at 0xae000 | load | 4 | `sp -0x210` | `-` |
| `ldr` at 0xae004 | load | 4 | `ret(0x1b3dc) +0x8` | `-` |
| `str` at 0xae008 | store | 4 | `sp -0x220` | `[ret(0x1b3dc)+8]` |
| `ldr` at 0xae00c | load | 4 | `sp -0x210` | `-` |
| `ldr` at 0xae010 | load | 4 | `ret(0x1b3dc) +0x4` | `-` |
| `str` at 0xae014 | store | 4 | `sp -0x21c` | `[ret(0x1b3dc)+4]` |
| `ldr` at 0xae018 | load | 4 | `sp -0x210` | `-` |
| `ldr` at 0xae01c | load | 4 | `ret(0x1b3dc) +0x0` | `-` |
| `str` at 0xae020 | store | 4 | `sp -0x218` | `[ret(0x1b3dc)]` |
| `ldr` at 0xae028 | load | 4 | `abs 0xae088` | `-` |
| `str` at 0xae03c | store | 4 | `sp -0x208` | `sp-516` |
| `ldr` at 0xae044 | load | 4 | `abs 0xae08c` | `-` |
| `str` at 0xae048 | store | 4 | `sp -0x208` | `0x127354` |
| `ldr` at 0xae04c | load | 4 | `sp -0x208` | `-` |
| `str` at 0xae068 | store | 4 | `sp -0x20c` | `ret(0x1b124)` |
| `ldr` at 0xae06c | load | 4 | `sp -0x20c` | `-` |
| `ldr` at 0xae070 | load | 4 | `sp -0x208` | `-` |
| `ldr` at 0xae078 | load | 4 | `sp -0x20c` | `-` |
| `ldmdb` at 0xae080 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xae080 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xae080 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xadf4c | `(unknown callee) 0xaa7dc` | `r0` | `sp-428` | `0xfffffe64` | `sp-428` |
| `bl` at 0xadfb0 | `(unknown callee) 0x1b3dc` | `sp-436` | `?` | `?` | `sp-436` |
| `bl` at 0xae034 | `(unknown callee) 0x1b49c` | `sp-516` | `0x127314` | `[[sp-532]+1]:1` | `[[sp-532]]:1` |
| `bl` at 0xae050 | `(unknown callee) 0x1b22c` | `?` | `?` | `?` | `?` |
| `bl` at 0xae060 | `(unknown callee) 0x1b124` | `ret(0x1b22c)+1` | `?` | `?` | `ret(0x1b22c)+1` |
| `bl` at 0xae074 | `(unknown callee) 0x1b508` | `ret(0x1b124)` | `[sp-520]` | `?` | `ret(0x1b124)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xadf60 | `ret(0xaa7dc) == 0x0` | `ret(0xaa7dc) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xae080 | `r0` | `[sp-524]` |

## `SysComLogReset_2m`

- `SysComLogReset_2m`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaf518 to 0xaf5ac.
  Established. Two calls reach `0xb988c` with all four candidates unknown at the first site (`bl` at
  0xaf570). The second has unknown, entry r1, unknown, unknown (`bl` at 0xaf59c). The entry guard
  retains `Z == 0` for taken and its negation for fall-through (`bne` at 0xaf534). Log-channel
  selection and final r0 are not established (`ldmdb` at 0xaf5a0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaf528 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xaf52c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaf53c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xaf540 | load | 4 | `abs 0xaf5a4` | `-` |
| `ldr` at 0xaf544 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaf548 | load | 4 | `abs 0x2b1730` | `-` |
| `ldr` at 0xaf558 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaf560 | load | 4 | `abs 0xaf5a8` | `-` |
| `ldr` at 0xaf564 | load | 4 | `abs 0x149a48` | `-` |
| `ldr` at 0xaf56c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xaf574 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xaf57c | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0xaf584 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf58c | load | 4 | `abs 0xaf5a8` | `-` |
| `ldr` at 0xaf590 | load | 4 | `abs 0x149a48` | `-` |
| `ldr` at 0xaf598 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0xaf5a0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaf5a0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaf5a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaf570 | `(unknown callee) 0xb988c` | `?` | `?` | `?` | `?` |
| `bl` at 0xaf59c | `(unknown callee) 0xb988c` | `?` | `r1` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xaf534 | `Z == 0` | `not (Z == 0)` |
| `blt` at 0xaf550 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaf5a0 | `r0` | `?` |

## `SysComCaptureReset`

- `SysComCaptureReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb04ec to 0xb0870.
  Established. The entry guard takes its bypass for `[0x149a50] != 0x0` and falls through on zero
  (`bne` at 0xb0508). A word store writes `0x1` at absolute `0x149a50` (`str` at 0xb0514). Two calls
  to `0x1b124` have unknown, `[sp-24]`, `[sp-28]`, unknown (`bl` at 0xb0604, `bl` at 0xb0670). A
  later word store writes `0xaf9d8` at absolute `0x149710` (`str` at 0xb06d8). Capture-buffer
  ownership and r0 at the merged exit remain unknown (`ldmdb` at 0xb0850).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xb04fc | load | 4 | `abs 0xb0854` | `-` |
| `ldr` at 0xb0500 | load | 4 | `abs 0x149a50` | `-` |
| `ldr` at 0xb050c | load | 4 | `abs 0xb0854` | `-` |
| `str` at 0xb0514 | store | 4 | `abs 0x149a50` | `0x1` |
| `ldr` at 0xb0518 | load | 4 | `abs 0xb0858` | `-` |
| `ldr` at 0xb051c | load | 4 | `abs 0x2b1728` | `-` |
| `ldr` at 0xb0520 | load | 4 | `[0x2b1728] +0x10` | `-` |
| `str` at 0xb0534 | store | 4 | `sp -0x1c` | `0x14` |
| `str` at 0xb0540 | store | 4 | `sp -0x1c` | `0x20` |
| `str` at 0xb0548 | store | 4 | `sp -0x1c` | `0x8` |
| `str` at 0xb0550 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xb0554 | load | 4 | `abs 0xb0858` | `-` |
| `ldr` at 0xb0558 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb055c | load | 4 | `abs 0x2b1730` | `-` |
| `ldr` at 0xb056c | load | 4 | `abs 0xb0858` | `-` |
| `ldr` at 0xb0570 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0594 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xb059c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb05a0 | store | 4 | `(unknown)` | `[sp-28]` |
| `ldr` at 0xb05a4 | load | 4 | `abs 0xb0858` | `-` |
| `ldr` at 0xb05a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb05cc | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xb05d4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb060c | store | 4 | `(unknown)` | `ret(0x1b124)` |
| `ldr` at 0xb0610 | load | 4 | `abs 0xb0858` | `-` |
| `ldr` at 0xb0614 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0638 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xb0640 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb0678 | store | 4 | `(unknown)` | `ret(0x1b124)` |
| `ldr` at 0xb067c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb0684 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0xb068c | load | 4 | `abs 0xb0858` | `-` |
| `ldr` at 0xb0690 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb0694 | load | 4 | `abs 0x2b1730` | `-` |
| `ldr` at 0xb06c4 | load | 4 | `abs 0xb085c` | `-` |
| `ldr` at 0xb06d0 | load | 4 | `abs 0xb0860` | `-` |
| `ldr` at 0xb06d4 | load | 4 | `abs 0xb0864` | `-` |
| `str` at 0xb06d8 | store | 4 | `abs 0x149710` | `0xaf9d8` |
| `ldr` at 0xb06dc | load | 4 | `abs 0xb0868` | `-` |
| `ldr` at 0xb06e0 | load | 4 | `abs 0xb086c` | `-` |
| `str` at 0xb06e4 | store | 4 | `abs 0x149714` | `0xb0044` |
| `str` at 0xb06ec | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xb06f0 | load | 4 | `abs 0xb0858` | `-` |
| `ldr` at 0xb06f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb06f8 | load | 4 | `abs 0x2b1730` | `-` |
| `ldr` at 0xb0708 | load | 4 | `abs 0xb0858` | `-` |
| `ldr` at 0xb070c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0730 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xb0738 | load | 4 | `abs 0xb0858` | `-` |
| `ldr` at 0xb073c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0760 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xb0768 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb0794 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb07a4 | load | 4 | `abs 0xb0858` | `-` |
| `ldr` at 0xb07a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb07cc | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xb07d4 | load | 4 | `abs 0xb0858` | `-` |
| `ldr` at 0xb07d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb07fc | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xb0804 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb0830 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb0840 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb0848 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldmdb` at 0xb0850 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb0850 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb0850 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb0850 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb0604 | `(unknown callee) 0x1b124` | `?` | `[sp-24]` | `[sp-28]` | `?` |
| `bl` at 0xb0670 | `(unknown callee) 0x1b124` | `?` | `[sp-24]` | `[sp-28]` | `?` |
| `bl` at 0xb06cc | `(unknown callee) 0x2070c` | `0x1275d4` | `?` | `?` | `?` |
| `bl` at 0xb07a0 | `(unknown callee) 0x1b43c` | `?` | `0x0` | `?` | `?` |
| `bl` at 0xb083c | `(unknown callee) 0x1b43c` | `?` | `0x0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb0508 | `[0x149a50] != 0x0` | `[0x149a50] == 0x0` |
| `beq` at 0xb0528 | `[[0x2b1728]+16] == 0x40` | `[[0x2b1728]+16] != 0x40` |
| `blt` at 0xb0564 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0xb0700 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb0850 | `r0` | `?` |
