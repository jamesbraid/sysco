# ARM fan reset and control paths in 1.38.4

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
The fan-reading selector has an unresolved computed transfer. Excluded destinations supply no known-edge arguments or effects. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/fan-reset-control-facts.txt) retain all tool sections.

## `fanTachInit`

- `fanTachInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x45214 to 0x454a0.
  Established. The zero arm of the absolute-word test writes word `0x1` at `0x148040` (`bne` at
  0x4528c, `str` at 0x45298). The call to `0x1b3c4` has `[sp-44]`, `0x1`, `[0x148040]`, unknown, and
  its result is stored as a word at `0x14803c` (`bl` at 0x452c4, `str` at 0x452cc). Later stores
  include byte `([sp-32]:1 & 0xff)` at `[sp-36] +0x4` and word `ret(0x1b3c4)` at `[sp-36] +0x8`
  (`strb` at 0x45338, `str` at 0x45358). The tachometer configuration and final r0 return are
  unknown (`ldmdb` at 0x4547c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x45224 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x45228 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x4522c | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x45230 | load | 4 | `abs 0x45480` | `-` |
| `ldr` at 0x45234 | load | 4 | `abs 0x14803c` | `-` |
| `ldr` at 0x45240 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4524c | load | 4 | `abs 0x45484` | `-` |
| `str` at 0x45250 | store | 4 | `sp -0x38` | `0x109184` |
| `ldr` at 0x45254 | load | 4 | `abs 0x45480` | `-` |
| `ldr` at 0x45258 | load | 4 | `abs 0x14803c` | `-` |
| `str` at 0x4525c | store | 4 | `sp -0x34` | `[0x14803c]` |
| `ldr` at 0x45260 | load | 4 | `abs 0x45488` | `-` |
| `ldr` at 0x45264 | load | 4 | `abs 0x148040` | `-` |
| `str` at 0x45268 | store | 4 | `sp -0x30` | `[0x148040]` |
| `ldr` at 0x45270 | load | 4 | `abs 0x4548c` | `-` |
| `ldr` at 0x45280 | load | 4 | `abs 0x45488` | `-` |
| `ldr` at 0x45284 | load | 4 | `abs 0x148040` | `-` |
| `ldr` at 0x45290 | load | 4 | `abs 0x45488` | `-` |
| `str` at 0x45298 | store | 4 | `abs 0x148040` | `0x1` |
| `ldr` at 0x4529c | load | 4 | `abs 0x45488` | `-` |
| `ldr` at 0x452a0 | load | 4 | `abs 0x148040` | `-` |
| `str` at 0x452b4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x452b8 | load | 4 | `abs 0x45480` | `-` |
| `ldr` at 0x452bc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x452cc | store | 4 | `abs 0x14803c` | `ret(0x1b3c4)` |
| `ldr` at 0x452d0 | load | 4 | `abs 0x14803c` | `-` |
| `ldr` at 0x452dc | load | 4 | `abs 0x45490` | `-` |
| `str` at 0x452e0 | store | 4 | `sp -0x38` | `0x1091cc` |
| `ldr` at 0x452e4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x452e8 | store | 4 | `sp -0x34` | `[sp-44]` |
| `ldr` at 0x452f0 | load | 4 | `abs 0x4548c` | `-` |
| `ldr` at 0x45300 | load | 4 | `abs 0x45480` | `-` |
| `ldr` at 0x45304 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45318 | load | 4 | `abs 0x14803c` | `-` |
| `str` at 0x45320 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x45324 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x45328 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4532c | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0x45330 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x45334 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0x45338 | store | 1 | `[sp-36] +0x4` | `([sp-32]:1 & 0xff)` |
| `ldr` at 0x4533c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x45340 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x45344 | load | 1 | `[sp-36] +0x4` | `-` |
| `str` at 0x45358 | store | 4 | `[sp-36] +0x8` | `ret(0x1b3c4)` |
| `ldr` at 0x4535c | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0x45368 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45370 | load | 4 | `abs 0x45494` | `-` |
| `ldr` at 0x45374 | load | 4 | `abs 0x45498` | `-` |
| `str` at 0x453a0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x453a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x453ac | load | 4 | `abs 0x45494` | `-` |
| `ldr` at 0x453b0 | load | 4 | `abs 0x45498` | `-` |
| `ldr` at 0x453bc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x453cc | load | 4 | `abs 0x4549c` | `-` |
| `str` at 0x453d0 | store | 4 | `(unknown)` | `0x109214` |
| `str` at 0x453d4 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0x453d8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x453dc | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0x453e4 | load | 4 | `abs 0x4548c` | `-` |
| `ldr` at 0x453f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x453f8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x453fc | load | 1 | `[sp-36] +0x4` | `-` |
| `ldr` at 0x45400 | load | 4 | `[sp-36] +0x8` | `-` |
| `str` at 0x45414 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x45418 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x4541c | load | 1 | `[sp-36] +0x4` | `-` |
| `ldr` at 0x45420 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x45430 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x45434 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x45448 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldrb` at 0x45450 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x4545c | load | 1 | `sp -0x28` | `-` |
| `ldr` at 0x45460 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4546c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x45474 | store | 4 | `sp -0x28` | `?` |
| `ldmdb` at 0x4547c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x4547c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4547c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4547c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4527c | `(unknown callee) 0x21e28` | `0x9` | `0x1091b8` | `0x4f` | `0x0` |
| `bl` at 0x452c4 | `(unknown callee) 0x1b3c4` | `[sp-44]` | `0x1` | `[0x148040]` | `?` |
| `bl` at 0x452fc | `(unknown callee) 0x21e28` | `0x9` | `0x1091b8` | `0x5b` | `0x0` |
| `bl` at 0x45350 | `(unknown callee) 0x1b3c4` | `[[sp-36]+4]:1` | `0x1` | `[sp-36]` | `[[sp-36]+4]:1` |
| `bl` at 0x45378 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x109208` | `0x109210` |
| `bl` at 0x45384 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x453b4 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x109208` | `0x109210` |
| `bl` at 0x453c4 | `(unknown callee) 0x1b508` | `[sp-44]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x453f0 | `(unknown callee) 0x21e28` | `0x9` | `0x1091b8` | `0x66` | `0x0` |
| `bl` at 0x4540c | `(unknown callee) 0x1b43c` | `[[sp-36]+8]` | `0x1` | `[[sp-36]+4]:1` | `[[sp-36]+4]:1` |
| `bl` at 0x45468 | `(unknown callee) 0x455c8` | `[sp-24]` | `[sp-40]:1` | `?` | `[sp-40]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4523c | `[0x14803c] == 0x0` | `[0x14803c] != 0x0` |
| `bne` at 0x45248 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0x4528c | `[0x148040] != 0x0` | `[0x148040] == 0x0` |
| `bne` at 0x452d8 | `[0x14803c] != 0x0` | `[0x14803c] == 0x0` |
| `bne` at 0x45364 | `[[sp-36]+8] != 0x0` | `[[sp-36]+8] == 0x0` |
| `blt` at 0x45428 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x45458 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4547c | `r0` | `?` |

## `fanReset`

- `fanReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x456f4 to 0x45d70.
  Established. Calls include `0xd6a1c` with `[sp-28]`, `[[sp-32]+3]:1`, `[[sp-32]+5]:1`, `[sp-37]:1`
  and `0xcd47c` with `[sp-28]`, `[[sp-32]+3]:1`, `[[sp-32]+4]:1`, `[[sp-32]+7]:1` (`bl` at 0x45878,
  `bl` at 0x45a2c). Each zero result takes the respective continuation and nonzero falls through
  to further calls (`beq` at 0x4588c, `beq` at 0x45a40). A later call to `0xd9d3c` carries
  `[sp-28]`, `[sp-80]:1`, `[sp-84]:1`, `[sp-88]` (`bl` at 0x45d28). The reported r0 return is `0x0`
  (`ldmdb` at 0x45d44). Physical fan reset effects and timing remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x45704 | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0x45708 | load | 4 | `abs 0x45d48` | `-` |
| `ldr` at 0x4570c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45720 | load | 4 | `abs 0x14803c` | `-` |
| `str` at 0x45728 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x4572c | load | 4 | `abs 0x45d4c` | `-` |
| `ldr` at 0x45730 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45738 | load | 4 | `abs 0x148044` | `-` |
| `str` at 0x45740 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x45748 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x45758 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x4575c | load | 4 | `sp -0x38` | `-` |
| `ldrb` at 0x45760 | load | 1 | `[sp-56] +0x4` | `-` |
| `ldr` at 0x45764 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x45774 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x45778 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x4578c | load | 4 | `[sp-56] +0x0` | `-` |
| `str` at 0x45794 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x45798 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x4579c | load | 1 | `(unknown)` | `-` |
| `str` at 0x457a0 | store | 4 | `sp -0x60` | `?` |
| `ldr` at 0x457a4 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x457b0 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x457bc | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x457cc | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x457dc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x457e0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x457f0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x457f4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x457f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45834 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x45838 | load | 1 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x45840 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x45848 | load | 1 | `[sp-32] +0x5` | `-` |
| `strb` at 0x45850 | store | 1 | `sp -0x25` | `?` |
| `ldr` at 0x45854 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x45858 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x4585c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x45860 | load | 1 | `[sp-32] +0x5` | `-` |
| `ldrb` at 0x45864 | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0x45868 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x45880 | store | 4 | `sp -0x2c` | `ret(0xd6a1c)` |
| `ldr` at 0x45884 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x45890 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45898 | load | 4 | `abs 0x45d50` | `-` |
| `ldr` at 0x4589c | load | 4 | `abs 0x45d54` | `-` |
| `str` at 0x458c8 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x458cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x458d4 | load | 4 | `abs 0x45d50` | `-` |
| `ldr` at 0x458d8 | load | 4 | `abs 0x45d54` | `-` |
| `ldr` at 0x458e4 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x458f4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x458f8 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x458fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4590c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x45918 | load | 4 | `abs 0x45d58` | `-` |
| `str` at 0x4591c | store | 4 | `(unknown)` | `0x1092c4` |
| `str` at 0x45920 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `str` at 0x45924 | store | 4 | `(unknown)` | `ret(0xd7558)` |
| `str` at 0x45928 | store | 4 | `(unknown)` | `ret(0x219a8)` |
| `ldr` at 0x45930 | load | 4 | `abs 0x45d5c` | `-` |
| `ldr` at 0x45944 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x45948 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x4594c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45968 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x4596c | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x45980 | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x45988 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x45994 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x45998 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x459ac | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x459b4 | load | 1 | `(unknown)` | `-` |
| `str` at 0x459b8 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x459bc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x459c0 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x459d4 | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x459dc | load | 1 | `(unknown)` | `-` |
| `str` at 0x459e0 | store | 4 | `sp -0x40` | `?` |
| `ldrb` at 0x459e4 | load | 1 | `sp -0x3c` | `-` |
| `ldrb` at 0x459e8 | load | 1 | `sp -0x40` | `-` |
| `ldr` at 0x459ec | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x459f0 | load | 1 | `[sp-32] +0x4` | `-` |
| `str` at 0x459f8 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x459fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45a10 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x45a14 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x45a18 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x45a1c | load | 1 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x45a20 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x45a24 | load | 1 | `[sp-32] +0x7` | `-` |
| `ldr` at 0x45a28 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x45a34 | store | 4 | `sp -0x2c` | `ret(0xcd47c)` |
| `ldr` at 0x45a38 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x45a44 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45a4c | load | 4 | `abs 0x45d50` | `-` |
| `ldr` at 0x45a50 | load | 4 | `abs 0x45d54` | `-` |
| `str` at 0x45a7c | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x45a80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45a88 | load | 4 | `abs 0x45d50` | `-` |
| `ldr` at 0x45a8c | load | 4 | `abs 0x45d54` | `-` |
| `ldr` at 0x45a98 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x45aa8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x45aac | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x45ab0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45ac0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x45acc | load | 4 | `abs 0x45d60` | `-` |
| `str` at 0x45ad0 | store | 4 | `(unknown)` | `0x1092f8` |
| `str` at 0x45ad4 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `str` at 0x45ad8 | store | 4 | `(unknown)` | `ret(0xcdf64)` |
| `str` at 0x45adc | store | 4 | `(unknown)` | `ret(0x219a8)` |
| `ldr` at 0x45ae4 | load | 4 | `abs 0x45d5c` | `-` |
| `ldr` at 0x45af8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45b00 | load | 4 | `abs 0x45d64` | `-` |
| `ldr` at 0x45b04 | load | 4 | `abs 0x45d68` | `-` |
| `str` at 0x45b30 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x45b34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45b3c | load | 4 | `abs 0x45d64` | `-` |
| `ldr` at 0x45b40 | load | 4 | `abs 0x45d68` | `-` |
| `ldr` at 0x45b4c | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x45b5c | load | 4 | `abs 0x45d6c` | `-` |
| `str` at 0x45b60 | store | 4 | `(unknown)` | `0x109330` |
| `str` at 0x45b64 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0x45b68 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x45b6c | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x45b70 | store | 4 | `(unknown)` | `[[sp-32]]:1` |
| `ldr` at 0x45b78 | load | 4 | `abs 0x45d5c` | `-` |
| `ldr` at 0x45b88 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x45b90 | store | 4 | `sp -0x30` | `?` |
| `str` at 0x45b9c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x45ba0 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x45ba4 | load | 1 | `[sp-52] +0x4` | `-` |
| `ldr` at 0x45ba8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x45bb8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x45bbc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x45bd0 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldrb` at 0x45bd8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x45bec | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x45bf4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x45c04 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x45c08 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x45c1c | load | 4 | `[sp-52] +0x0` | `-` |
| `ldrb` at 0x45c24 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x45c38 | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x45c40 | load | 1 | `(unknown)` | `-` |
| `str` at 0x45c44 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x45c48 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x45c4c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x45c60 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldrb` at 0x45c68 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x45c7c | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x45c84 | load | 1 | `(unknown)` | `-` |
| `str` at 0x45c88 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x45c8c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x45c90 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x45ca4 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldrb` at 0x45cac | load | 1 | `(unknown)` | `-` |
| `str` at 0x45cb0 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x45cb4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x45cb8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x45ccc | load | 4 | `[sp-52] +0x0` | `-` |
| `ldrb` at 0x45cd4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x45ce8 | load | 4 | `sp -0x4c` | `-` |
| `ldrh` at 0x45cf0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x45cf4 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x45d04 | store | 4 | `sp -0x5c` | `?` |
| `ldrb` at 0x45d08 | load | 1 | `sp -0x50` | `-` |
| `ldrb` at 0x45d0c | load | 1 | `sp -0x54` | `-` |
| `ldr` at 0x45d10 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x45d14 | store | 4 | `(unknown)` | `[sp-92]` |
| `ldr` at 0x45d18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45d24 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x45d2c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x45d34 | store | 4 | `sp -0x30` | `?` |
| `ldmdb` at 0x45d44 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x45d44 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x45d44 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x45d44 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x45d44 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x45750 | `(unknown callee) 0xcc7c8` | `r0` | `sp-76` | `?` | `sp-76` |
| `bl` at 0x45800 | `(unknown callee) 0xd6928` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x45818 | `(unknown callee) 0x1f1dc` | `0x4` | `?` | `?` | `ret(0xd6928)` |
| `bl` at 0x4582c | `(unknown callee) 0x1f1dc` | `0x5` | `?` | `?` | `ret(0x1f1dc)` |
| `bl` at 0x45878 | `(unknown callee) 0xd6a1c` | `[sp-28]` | `[[sp-32]+3]:1` | `[[sp-32]+5]:1` | `[sp-37]:1` |
| `bl` at 0x458a0 | `(unknown callee) 0x4f564` | `[sp-28]` | `0x0` | `0x109208` | `0x109210` |
| `bl` at 0x458ac | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x458dc | `(unknown callee) 0x4f564` | `[sp-28]` | `0x0` | `0x109208` | `0x109210` |
| `bl` at 0x458ec | `(unknown callee) 0x1b508` | `[sp-80]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x45904 | `(unknown callee) 0xd7558` | `[sp-28]` | `[[sp-32]+3]:1` | `?` | `[[sp-32]+3]:1` |
| `bl` at 0x45910 | `(unknown callee) 0x219a8` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0x4593c | `(unknown callee) 0x21e28` | `0x1` | `0x1091b8` | `0xec` | `0x0` |
| `bl` at 0x45954 | `(unknown callee) 0xcc494` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x45a0c | `(unknown callee) 0xd9c84` | `[sp-28]` | `[sp-60]:1` | `[sp-64]:1` | `[[sp-32]+4]:1` |
| `bl` at 0x45a2c | `(unknown callee) 0xcd47c` | `[sp-28]` | `[[sp-32]+3]:1` | `[[sp-32]+4]:1` | `[[sp-32]+7]:1` |
| `bl` at 0x45a54 | `(unknown callee) 0x4f564` | `[sp-28]` | `0x0` | `0x109208` | `0x109210` |
| `bl` at 0x45a60 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x45a90 | `(unknown callee) 0x4f564` | `[sp-28]` | `0x0` | `0x109208` | `0x109210` |
| `bl` at 0x45aa0 | `(unknown callee) 0x1b508` | `[sp-80]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x45ab8 | `(unknown callee) 0xcdf64` | `[sp-28]` | `[[sp-32]+3]:1` | `?` | `[[sp-32]+3]:1` |
| `bl` at 0x45ac4 | `(unknown callee) 0x219a8` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0x45af0 | `(unknown callee) 0x21e28` | `0x1` | `0x1091b8` | `0x104` | `0x0` |
| `bl` at 0x45b08 | `(unknown callee) 0x4f564` | `[sp-28]` | `0x0` | `0x109324` | `0x10932c` |
| `bl` at 0x45b14 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x45b44 | `(unknown callee) 0x4f564` | `[sp-28]` | `0x0` | `0x109324` | `0x10932c` |
| `bl` at 0x45b54 | `(unknown callee) 0x1b508` | `[sp-80]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x45b84 | `(unknown callee) 0x21e28` | `0x1` | `0x1091b8` | `0x110` | `0x0` |
| `bl` at 0x45d28 | `(unknown callee) 0xd9d3c` | `[sp-28]` | `[sp-80]:1` | `[sp-84]:1` | `[sp-88]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x4576c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x457ac | `Z == 1` | `not (Z == 1)` |
| `bgt` at 0x457b8 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x457c4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x457d4 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x457e8 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x4580c | `ret(0xd6928) != 0x0` | `ret(0xd6928) == 0x0` |
| `beq` at 0x45824 | `ret(0x1f1dc) == 0x0` | `ret(0x1f1dc) != 0x0` |
| `beq` at 0x4588c | `ret(0xd6a1c) == 0x0` | `ret(0xd6a1c) != 0x0` |
| `bne` at 0x45960 | `ret(0xcc494) != 0x0` | `ret(0xcc494) == 0x0` |
| `bne` at 0x45990 | `? != 0x6` | `? == 0x6` |
| `beq` at 0x45a40 | `ret(0xcd47c) == 0x0` | `ret(0xcd47c) != 0x0` |
| `blt` at 0x45bb0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x45bfc | `? == 0x6` | `? != 0x6` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x45d44 | `r0` | `0x0` |

## `fanOn`

- `fanOn`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x45ddc to 0x461b8.
  Established. The first call reaches `0x47494` with entry r0, `(r1 & 0xff)`, `sp-29`, `sp-30` (`bl`
  at 0x45e34); its zero result takes the continuing edge and nonzero falls through to store word
  `ret(0x47494)` at `sp -0x34` (`beq` at 0x45e48, `str` at 0x45e50). Later calls include `0xd69d4`
  with `[sp-20]`, `[ret(0x47130)+1]:1`, `[ret(0x47130)+2]:1`, `[ret(0x47130)+2]:1` and `0xcd47c`
  with `[sp-20]`, `[ret(0x47130)+3]:1`, `[ret(0x47130)+4]:1`, `[ret(0x47130)+7]:1` (`bl` at 0x45f34,
  `bl` at 0x46010). Physical fan-on behavior and the merged r0 value remain unproven (`ldmdb` at
  0x46184).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x45dec | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x45df4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x45df8 | load | 4 | `abs 0x46188` | `-` |
| `ldr` at 0x45dfc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x45e10 | load | 4 | `abs 0x14803c` | `-` |
| `str` at 0x45e18 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0x45e1c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x45e28 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x45e3c | store | 4 | `sp -0x24` | `ret(0x47494)` |
| `ldr` at 0x45e40 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x45e4c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x45e50 | store | 4 | `sp -0x34` | `ret(0x47494)` |
| `ldrb` at 0x45e58 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x45e5c | store | 1 | `sp -0x1f` | `([sp-29]:1 & 0xff)` |
| `ldrb` at 0x45e60 | load | 1 | `sp -0x1f` | `-` |
| `ldrb` at 0x45e64 | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x45e74 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x45e78 | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x45e7c | load | 4 | `[sp-44] +0x8` | `-` |
| `ldrb` at 0x45e84 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x45e94 | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x45e98 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x45ea8 | load | 1 | `ret(0x47004) +0x0` | `-` |
| `ldr` at 0x45eac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x45ebc | store | 4 | `sp -0x1c` | `ret(0x47130)` |
| `ldr` at 0x45ec0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x45ec4 | load | 1 | `ret(0x47130) +0x0` | `-` |
| `str` at 0x45ec8 | store | 4 | `sp -0x38` | `[ret(0x47130)]:1` |
| `ldr` at 0x45ecc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x45ed8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x45ee4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x45ef4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x45f04 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x45f08 | load | 1 | `ret(0x47130) +0x1` | `-` |
| `ldr` at 0x45f18 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x45f1c | load | 1 | `ret(0x47130) +0x1` | `-` |
| `ldr` at 0x45f20 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x45f24 | load | 1 | `ret(0x47130) +0x2` | `-` |
| `ldr` at 0x45f28 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x45f3c | store | 4 | `sp -0x28` | `ret(0xd69d4)` |
| `ldr` at 0x45f40 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x45f4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x45f54 | load | 4 | `abs 0x4618c` | `-` |
| `ldr` at 0x45f58 | load | 4 | `abs 0x46190` | `-` |
| `str` at 0x45f84 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x45f88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x45f90 | load | 4 | `abs 0x4618c` | `-` |
| `ldr` at 0x45f94 | load | 4 | `abs 0x46190` | `-` |
| `ldr` at 0x45fa0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x45fb0 | load | 4 | `abs 0x46194` | `-` |
| `str` at 0x45fb4 | store | 4 | `(unknown)` | `0x109354` |
| `str` at 0x45fb8 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0x45fbc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x45fc0 | load | 1 | `[sp-28] +0x1` | `-` |
| `str` at 0x45fc4 | store | 4 | `(unknown)` | `[[sp-28]+1]:1` |
| `ldr` at 0x45fcc | load | 4 | `abs 0x46198` | `-` |
| `ldr` at 0x45fd0 | load | 4 | `abs 0x4619c` | `-` |
| `ldr` at 0x45fdc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x45fe8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x45fec | store | 4 | `sp -0x24` | `[sp-40]` |
| `ldr` at 0x45ff4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x45ff8 | load | 1 | `ret(0x47130) +0x3` | `-` |
| `ldr` at 0x45ffc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46000 | load | 1 | `ret(0x47130) +0x4` | `-` |
| `ldr` at 0x46004 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46008 | load | 1 | `ret(0x47130) +0x7` | `-` |
| `ldr` at 0x4600c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x46018 | store | 4 | `sp -0x28` | `ret(0xcd47c)` |
| `ldr` at 0x4601c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x46028 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x46030 | load | 4 | `abs 0x4618c` | `-` |
| `ldr` at 0x46034 | load | 4 | `abs 0x46190` | `-` |
| `str` at 0x46060 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x46064 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4606c | load | 4 | `abs 0x4618c` | `-` |
| `ldr` at 0x46070 | load | 4 | `abs 0x46190` | `-` |
| `ldr` at 0x4607c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x4608c | load | 4 | `abs 0x46194` | `-` |
| `str` at 0x46090 | store | 4 | `(unknown)` | `0x109354` |
| `str` at 0x46094 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0x46098 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4609c | load | 1 | `[sp-28] +0x4` | `-` |
| `str` at 0x460a0 | store | 4 | `(unknown)` | `[[sp-28]+4]:1` |
| `ldr` at 0x460a8 | load | 4 | `abs 0x46198` | `-` |
| `ldr` at 0x460ac | load | 4 | `abs 0x461a0` | `-` |
| `ldr` at 0x460b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x460c4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x460c8 | store | 4 | `sp -0x24` | `[sp-40]` |
| `ldr` at 0x460d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x460d8 | load | 4 | `abs 0x461a4` | `-` |
| `ldr` at 0x460dc | load | 4 | `abs 0x461a8` | `-` |
| `str` at 0x46108 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x4610c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x46114 | load | 4 | `abs 0x461a4` | `-` |
| `ldr` at 0x46118 | load | 4 | `abs 0x461a8` | `-` |
| `ldr` at 0x46124 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x46134 | load | 4 | `abs 0x461ac` | `-` |
| `str` at 0x46138 | store | 4 | `(unknown)` | `0x109378` |
| `str` at 0x4613c | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0x46140 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46144 | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0x46148 | store | 4 | `(unknown)` | `[[sp-28]]:1` |
| `ldr` at 0x46150 | load | 4 | `abs 0x46198` | `-` |
| `ldr` at 0x46154 | load | 4 | `abs 0x461b0` | `-` |
| `ldr` at 0x46160 | load | 4 | `abs 0x461b4` | `-` |
| `str` at 0x46164 | store | 4 | `sp -0x24` | `0xfffffa86` |
| `ldrb` at 0x46168 | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0x46170 | store | 1 | `sp -0x1f` | `?` |
| `ldr` at 0x46178 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x4617c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x46180 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x46184 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x46184 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x46184 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x45e34 | `(unknown callee) 0x47494` | `r0` | `(r1 & 0xff)` | `sp-29` | `sp-30` |
| `bl` at 0x45ea0 | `(unknown callee) 0x47004` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x45eb4 | `(unknown callee) 0x47130` | `[sp-20]` | `[ret(0x47004)]:1` | `?` | `[ret(0x47004)]:1` |
| `bl` at 0x45f34 | `(unknown callee) 0xd69d4` | `[sp-20]` | `[ret(0x47130)+1]:1` | `[ret(0x47130)+2]:1` | `[ret(0x47130)+2]:1` |
| `bl` at 0x45f5c | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x109208` | `0x109210` |
| `bl` at 0x45f68 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x45f98 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x109208` | `0x109210` |
| `bl` at 0x45fa8 | `(unknown callee) 0x1b508` | `[sp-48]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x45fd8 | `(unknown callee) 0x21e28` | `0x1` | `0x1091b8` | `0x16d` | `0x0` |
| `bl` at 0x46010 | `(unknown callee) 0xcd47c` | `[sp-20]` | `[ret(0x47130)+3]:1` | `[ret(0x47130)+4]:1` | `[ret(0x47130)+7]:1` |
| `bl` at 0x46038 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x109208` | `0x109210` |
| `bl` at 0x46044 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x46074 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x109208` | `0x109210` |
| `bl` at 0x46084 | `(unknown callee) 0x1b508` | `[sp-48]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x460b4 | `(unknown callee) 0x21e28` | `0x1` | `0x1091b8` | `0x179` | `0x0` |
| `bl` at 0x460e0 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x109324` | `0x10932c` |
| `bl` at 0x460ec | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x4611c | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x109324` | `0x10932c` |
| `bl` at 0x4612c | `(unknown callee) 0x1b508` | `[sp-48]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x4615c | `(unknown callee) 0x21e28` | `0x1` | `0x1091b8` | `0x186` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x45e48 | `ret(0x47494) == 0x0` | `ret(0x47494) != 0x0` |
| `bls` at 0x45e6c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x45e8c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x45ed4 | `[ret(0x47130)]:1 == 0x2` | `[ret(0x47130)]:1 != 0x2` |
| `bgt` at 0x45ee0 | `[ret(0x47130)]:1 > 0x2 (signed, 32-bit)` | `[ret(0x47130)]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0x45eec | `[ret(0x47130)]:1 == 0x1` | `[ret(0x47130)]:1 != 0x1` |
| `beq` at 0x45efc | `[ret(0x47130)]:1 == 0x63` | `[ret(0x47130)]:1 != 0x63` |
| `bne` at 0x45f10 | `[ret(0x47130)+1]:1 != 0xff` | `[ret(0x47130)+1]:1 == 0xff` |
| `beq` at 0x45f48 | `ret(0xd69d4) == 0x0` | `ret(0xd69d4) != 0x0` |
| `bne` at 0x45fe4 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `beq` at 0x46024 | `ret(0xcd47c) == 0x0` | `ret(0xcd47c) != 0x0` |
| `bne` at 0x460c0 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x46184 | `r0` | `?` |

## `fanOff`

- `fanOff`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x461b8 to 0x4658c.
  Established. The call to `0x47494` has entry r0, `(r1 & 0xff)`, `sp-29`, `sp-30` (`bl` at
  0x46210). Zero takes the continuation and nonzero falls through to an early saved result
  (`beq` at 0x46224, `str` at 0x4622c). The selector takes one arm for `[ret(0x47130)]:1 == 0x2` and
  falls through on inequality (`beq` at 0x462b0). That arm calls `0xcd47c` with `[sp-20]`,
  `[ret(0x47130)+3]:1`, `[ret(0x47130)+4]:1`, `0x0` (`bl` at 0x463f0). The joined r0 return and
  physical fan-off action are unknown (`ldmdb` at 0x46564).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x461c8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x461d0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x461d4 | load | 4 | `abs 0x46568` | `-` |
| `ldr` at 0x461d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x461ec | load | 4 | `abs 0x14803c` | `-` |
| `str` at 0x461f4 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0x461f8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x46204 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x46218 | store | 4 | `sp -0x24` | `ret(0x47494)` |
| `ldr` at 0x4621c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x46228 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x4622c | store | 4 | `sp -0x34` | `ret(0x47494)` |
| `ldrb` at 0x46234 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x46238 | store | 1 | `sp -0x1f` | `([sp-29]:1 & 0xff)` |
| `ldrb` at 0x4623c | load | 1 | `sp -0x1f` | `-` |
| `ldrb` at 0x46240 | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x46250 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x46254 | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x46258 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldrb` at 0x46260 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x46270 | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x46274 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x46284 | load | 1 | `ret(0x47004) +0x0` | `-` |
| `ldr` at 0x46288 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x46298 | store | 4 | `sp -0x1c` | `ret(0x47130)` |
| `ldr` at 0x4629c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x462a0 | load | 1 | `ret(0x47130) +0x0` | `-` |
| `str` at 0x462a4 | store | 4 | `sp -0x38` | `[ret(0x47130)]:1` |
| `ldr` at 0x462a8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x462b4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x462c0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x462d0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x462e0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x462e4 | load | 1 | `ret(0x47130) +0x1` | `-` |
| `ldr` at 0x462f4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x462f8 | load | 1 | `ret(0x47130) +0x1` | `-` |
| `ldr` at 0x462fc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46300 | load | 1 | `ret(0x47130) +0x2` | `-` |
| `ldr` at 0x46304 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x46318 | store | 4 | `sp -0x28` | `ret(0xd6988)` |
| `ldr` at 0x4631c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x46328 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x46330 | load | 4 | `abs 0x4656c` | `-` |
| `ldr` at 0x46334 | load | 4 | `abs 0x46570` | `-` |
| `str` at 0x46360 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x46364 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4636c | load | 4 | `abs 0x4656c` | `-` |
| `ldr` at 0x46370 | load | 4 | `abs 0x46570` | `-` |
| `ldr` at 0x4637c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x4638c | load | 4 | `abs 0x46574` | `-` |
| `str` at 0x46390 | store | 4 | `(unknown)` | `0x10939c` |
| `str` at 0x46394 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0x46398 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4639c | load | 1 | `[sp-28] +0x1` | `-` |
| `str` at 0x463a0 | store | 4 | `(unknown)` | `[[sp-28]+1]:1` |
| `ldr` at 0x463a8 | load | 4 | `abs 0x46578` | `-` |
| `ldr` at 0x463ac | load | 4 | `abs 0x4657c` | `-` |
| `ldr` at 0x463b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x463c4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x463c8 | store | 4 | `sp -0x24` | `[sp-40]` |
| `ldr` at 0x463d0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x463d4 | load | 1 | `ret(0x47130) +0x3` | `-` |
| `ldr` at 0x463d8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x463dc | load | 1 | `ret(0x47130) +0x4` | `-` |
| `ldr` at 0x463e0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x463f8 | store | 4 | `sp -0x28` | `ret(0xcd47c)` |
| `ldr` at 0x463fc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x46408 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x46410 | load | 4 | `abs 0x4656c` | `-` |
| `ldr` at 0x46414 | load | 4 | `abs 0x46570` | `-` |
| `str` at 0x46440 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x46444 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4644c | load | 4 | `abs 0x4656c` | `-` |
| `ldr` at 0x46450 | load | 4 | `abs 0x46570` | `-` |
| `ldr` at 0x4645c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x4646c | load | 4 | `abs 0x46574` | `-` |
| `str` at 0x46470 | store | 4 | `(unknown)` | `0x10939c` |
| `str` at 0x46474 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0x46478 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4647c | load | 1 | `[sp-28] +0x3` | `-` |
| `str` at 0x46480 | store | 4 | `(unknown)` | `[[sp-28]+3]:1` |
| `ldr` at 0x46488 | load | 4 | `abs 0x46578` | `-` |
| `ldr` at 0x4648c | load | 4 | `abs 0x46580` | `-` |
| `ldr` at 0x46498 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x464a4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x464a8 | store | 4 | `sp -0x24` | `[sp-40]` |
| `ldr` at 0x464b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x464b8 | load | 4 | `abs 0x4656c` | `-` |
| `ldr` at 0x464bc | load | 4 | `abs 0x46570` | `-` |
| `str` at 0x464e8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x464ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x464f4 | load | 4 | `abs 0x4656c` | `-` |
| `ldr` at 0x464f8 | load | 4 | `abs 0x46570` | `-` |
| `ldr` at 0x46504 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x46514 | load | 4 | `abs 0x46584` | `-` |
| `str` at 0x46518 | store | 4 | `(unknown)` | `0x1093c0` |
| `str` at 0x4651c | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0x46520 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46524 | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0x46528 | store | 4 | `(unknown)` | `[[sp-28]]:1` |
| `ldr` at 0x46530 | load | 4 | `abs 0x46578` | `-` |
| `ldr` at 0x46540 | load | 4 | `abs 0x46588` | `-` |
| `str` at 0x46544 | store | 4 | `sp -0x24` | `0xfffffa86` |
| `ldrb` at 0x46548 | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0x46550 | store | 1 | `sp -0x1f` | `?` |
| `ldr` at 0x46558 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x4655c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x46560 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x46564 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x46564 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x46564 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x46210 | `(unknown callee) 0x47494` | `r0` | `(r1 & 0xff)` | `sp-29` | `sp-30` |
| `bl` at 0x4627c | `(unknown callee) 0x47004` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x46290 | `(unknown callee) 0x47130` | `[sp-20]` | `[ret(0x47004)]:1` | `?` | `[ret(0x47004)]:1` |
| `bl` at 0x46310 | `(unknown callee) 0xd6988` | `[sp-20]` | `[ret(0x47130)+1]:1` | `[ret(0x47130)+2]:1` | `[ret(0x47130)+2]:1` |
| `bl` at 0x46338 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x109208` | `0x109210` |
| `bl` at 0x46344 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x46374 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x109208` | `0x109210` |
| `bl` at 0x46384 | `(unknown callee) 0x1b508` | `[sp-48]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x463b4 | `(unknown callee) 0x21e28` | `0x1` | `0x1091b8` | `0x1af` | `0x0` |
| `bl` at 0x463f0 | `(unknown callee) 0xcd47c` | `[sp-20]` | `[ret(0x47130)+3]:1` | `[ret(0x47130)+4]:1` | `0x0` |
| `bl` at 0x46418 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x109208` | `0x109210` |
| `bl` at 0x46424 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x46454 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x109208` | `0x109210` |
| `bl` at 0x46464 | `(unknown callee) 0x1b508` | `[sp-48]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x46494 | `(unknown callee) 0x21e28` | `0x1` | `0x1091b8` | `0x1bb` | `0x0` |
| `bl` at 0x464c0 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x109208` | `0x109210` |
| `bl` at 0x464cc | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x464fc | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x109208` | `0x109210` |
| `bl` at 0x4650c | `(unknown callee) 0x1b508` | `[sp-48]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x4653c | `(unknown callee) 0x21e28` | `0x1` | `0x1091b8` | `0x1c8` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x46224 | `ret(0x47494) == 0x0` | `ret(0x47494) != 0x0` |
| `bls` at 0x46248 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x46268 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x462b0 | `[ret(0x47130)]:1 == 0x2` | `[ret(0x47130)]:1 != 0x2` |
| `bgt` at 0x462bc | `[ret(0x47130)]:1 > 0x2 (signed, 32-bit)` | `[ret(0x47130)]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0x462c8 | `[ret(0x47130)]:1 == 0x1` | `[ret(0x47130)]:1 != 0x1` |
| `beq` at 0x462d8 | `[ret(0x47130)]:1 == 0x63` | `[ret(0x47130)]:1 != 0x63` |
| `bne` at 0x462ec | `[ret(0x47130)+1]:1 != 0xff` | `[ret(0x47130)+1]:1 == 0xff` |
| `beq` at 0x46324 | `ret(0xd6988) == 0x0` | `ret(0xd6988) != 0x0` |
| `bne` at 0x463c0 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `beq` at 0x46404 | `ret(0xcd47c) == 0x0` | `ret(0xcd47c) != 0x0` |
| `bne` at 0x464a0 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x46564 | `r0` | `?` |

## `fanStatus`

- `fanStatus`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4658c to 0x467bc.
  Established. The first call targets `0x3d970` with entry r0, `0x14803c`, unknown, unknown (`bl` at
  0x465d0); its nonzero result takes the continuation and zero falls through to store word `0x0` at
  `sp -0x30` (`bne` at 0x465dc, `str` at 0x465e4). A later call to `0xcd71c` carries `[sp-20]`,
  `[ret(0x47130)+3]:1`, `[ret(0x47130)+4]:1`, `sp-37` (`bl` at 0x46750). The r0 return and physical
  status polarity remain unknown after the joins (`ldmdb` at 0x467b4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4659c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x465a4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x465a8 | load | 4 | `abs 0x467b8` | `-` |
| `ldr` at 0x465ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x465c0 | load | 4 | `abs 0x14803c` | `-` |
| `str` at 0x465c8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x465cc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x465e4 | store | 4 | `sp -0x30` | `0x0` |
| `ldrb` at 0x465ec | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x465f8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4660c | store | 4 | `sp -0x24` | `ret(0x47494)` |
| `ldr` at 0x46610 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4661c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x46620 | store | 4 | `sp -0x30` | `ret(0x47494)` |
| `ldrb` at 0x46628 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x4662c | store | 1 | `sp -0x1f` | `([sp-29]:1 & 0xff)` |
| `ldrb` at 0x46630 | load | 1 | `sp -0x1f` | `-` |
| `ldrb` at 0x46634 | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x46644 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x46648 | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x4664c | load | 4 | `[sp-44] +0x8` | `-` |
| `ldrb` at 0x46654 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x46664 | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x46668 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x46678 | load | 1 | `ret(0x47004) +0x0` | `-` |
| `ldr` at 0x4667c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4668c | store | 4 | `sp -0x1c` | `ret(0x47130)` |
| `ldr` at 0x46690 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46694 | load | 1 | `ret(0x47130) +0x0` | `-` |
| `str` at 0x46698 | store | 4 | `sp -0x34` | `[ret(0x47130)]:1` |
| `ldr` at 0x4669c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x466a8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x466b4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x466c4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x466d4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x466d8 | load | 1 | `ret(0x47130) +0x1` | `-` |
| `str` at 0x466e8 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x466f0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x466f4 | load | 1 | `ret(0x47130) +0x1` | `-` |
| `ldr` at 0x466f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x46708 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4670c | load | 1 | `[sp-28] +0x2` | `-` |
| `str` at 0x46724 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x4672c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46730 | load | 1 | `ret(0x47130) +0x3` | `-` |
| `ldr` at 0x46734 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46738 | load | 1 | `ret(0x47130) +0x4` | `-` |
| `ldr` at 0x46740 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x46754 | load | 1 | `sp -0x25` | `-` |
| `str` at 0x46764 | store | 4 | `sp -0x24` | `?` |
| `str` at 0x46770 | store | 4 | `sp -0x24` | `0x1` |
| `str` at 0x4677c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x46780 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4678c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x46790 | store | 4 | `sp -0x30` | `[sp-36]` |
| `ldrb` at 0x46798 | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0x467a0 | store | 1 | `sp -0x1f` | `?` |
| `ldr` at 0x467a8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x467ac | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x467b0 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0x467b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x467b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x467b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x465d0 | `(unknown callee) 0x3d970` | `r0` | `0x14803c` | `?` | `?` |
| `bl` at 0x46604 | `(unknown callee) 0x47494` | `[sp-20]` | `[sp-21]:1` | `sp-29` | `sp-30` |
| `bl` at 0x46670 | `(unknown callee) 0x47004` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x46684 | `(unknown callee) 0x47130` | `[sp-20]` | `[ret(0x47004)]:1` | `?` | `[ret(0x47004)]:1` |
| `bl` at 0x46700 | `(unknown callee) 0xd6c2c` | `[sp-20]` | `[ret(0x47130)+1]:1` | `?` | `[ret(0x47130)+1]:1` |
| `bl` at 0x46750 | `(unknown callee) 0xcd71c` | `[sp-20]` | `[ret(0x47130)+3]:1` | `[ret(0x47130)+4]:1` | `sp-37` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x465dc | `ret(0x3d970) != 0x0` | `ret(0x3d970) == 0x0` |
| `beq` at 0x46618 | `ret(0x47494) == 0x0` | `ret(0x47494) != 0x0` |
| `bls` at 0x4663c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x4665c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x466a4 | `[ret(0x47130)]:1 == 0x2` | `[ret(0x47130)]:1 != 0x2` |
| `bgt` at 0x466b0 | `[ret(0x47130)]:1 > 0x2 (signed, 32-bit)` | `[ret(0x47130)]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0x466bc | `[ret(0x47130)]:1 == 0x1` | `[ret(0x47130)]:1 != 0x1` |
| `beq` at 0x466cc | `[ret(0x47130)]:1 == 0x63` | `[ret(0x47130)]:1 != 0x63` |
| `bne` at 0x466e0 | `[ret(0x47130)+1]:1 != 0xff` | `[ret(0x47130)+1]:1 == 0xff` |
| `beq` at 0x46788 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x467b4 | `r0` | `?` |

## `fanSetSpeedRaw`

- `fanSetSpeedRaw`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x467bc to 0x46a5c.
  Established. Two calls to `0x1d38c` have `0x5`, `[sp-32]:1`, unknown, `[sp-32]:1` and `0x4`,
  `0x1`, unknown, unknown, respectively (`bl` at 0x46930, `bl` at 0x4693c). Later calls to the same
  target instead use `0x5`, `[sp-22]:1`, unknown, `[sp-22]:1`, followed by `0x4`, `0x1`, unknown,
  unknown (`bl` at 0x46a14, `bl` at 0x46a20). The call to `0xcd47c` has `[sp-20]`,
  `[ret(0x47130)+3]:1`, `[ret(0x47130)+4]:1`, `[sp-22]:1` (`bl` at 0x469dc). Speed encoding,
  physical output and the returned r0 value remain unproven (`ldmdb` at 0x46a4c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x467cc | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x467d4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x467dc | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0x467e0 | load | 4 | `abs 0x46a50` | `-` |
| `ldr` at 0x467e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x467f8 | load | 4 | `abs 0x14803c` | `-` |
| `str` at 0x46800 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0x46804 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x46810 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x46824 | store | 4 | `sp -0x24` | `ret(0x47494)` |
| `ldr` at 0x46828 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x46834 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x46838 | store | 4 | `sp -0x30` | `ret(0x47494)` |
| `ldrb` at 0x46840 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x46844 | store | 1 | `sp -0x1f` | `([sp-29]:1 & 0xff)` |
| `ldrb` at 0x46848 | load | 1 | `sp -0x1f` | `-` |
| `ldrb` at 0x4684c | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x4685c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x46860 | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x46864 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldrb` at 0x4686c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x46878 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x4687c | load | 1 | `sp -0x1f` | `-` |
| `ldrb` at 0x46880 | load | 1 | `[sp-44] +0x4` | `-` |
| `ldrb` at 0x4688c | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x46890 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x468a0 | load | 1 | `ret(0x47004) +0x0` | `-` |
| `ldr` at 0x468a4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x468b4 | store | 4 | `sp -0x1c` | `ret(0x47130)` |
| `ldr` at 0x468b8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x468bc | load | 1 | `ret(0x47130) +0x0` | `-` |
| `str` at 0x468c0 | store | 4 | `sp -0x34` | `[ret(0x47130)]:1` |
| `ldr` at 0x468c4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x468d0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x468dc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x468ec | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x468fc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46900 | load | 1 | `ret(0x47130) +0x3` | `-` |
| `ldr` at 0x4690c | load | 4 | `abs 0x46a54` | `-` |
| `str` at 0x46910 | store | 4 | `sp -0x30` | `0xfffffa87` |
| `ldrb` at 0x46918 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x46920 | store | 1 | `sp -0x20` | `?` |
| `ldrb` at 0x46924 | load | 1 | `sp -0x20` | `-` |
| `ldrb` at 0x46940 | load | 1 | `sp -0x20` | `-` |
| `ldr` at 0x46944 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46948 | load | 1 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x46950 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46958 | load | 1 | `[sp-28] +0x5` | `-` |
| `strb` at 0x46960 | store | 1 | `sp -0x20` | `?` |
| `ldr` at 0x46964 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46968 | load | 1 | `[sp-28] +0x3` | `-` |
| `ldr` at 0x4696c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46970 | load | 1 | `[sp-28] +0x5` | `-` |
| `ldrb` at 0x46974 | load | 1 | `sp -0x20` | `-` |
| `ldr` at 0x46978 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x46990 | store | 4 | `sp -0x28` | `ret(0xd6a1c)` |
| `ldr` at 0x46994 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x469a0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x469ac | load | 4 | `sp -0x28` | `-` |
| `str` at 0x469b0 | store | 4 | `sp -0x24` | `ret(0xd6a1c)` |
| `ldr` at 0x469b8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x469bc | load | 1 | `ret(0x47130) +0x3` | `-` |
| `ldr` at 0x469c0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x469c4 | load | 1 | `ret(0x47130) +0x4` | `-` |
| `ldrb` at 0x469c8 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x469cc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x469e4 | store | 4 | `sp -0x28` | `ret(0xcd47c)` |
| `ldr` at 0x469e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x469f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x46a00 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x46a04 | store | 4 | `sp -0x24` | `ret(0xcd47c)` |
| `ldrb` at 0x46a08 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x46a28 | load | 4 | `abs 0x46a58` | `-` |
| `str` at 0x46a2c | store | 4 | `sp -0x24` | `0xfffffa86` |
| `ldrb` at 0x46a30 | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0x46a38 | store | 1 | `sp -0x1f` | `?` |
| `ldr` at 0x46a40 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x46a44 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x46a48 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0x46a4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x46a4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x46a4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4681c | `(unknown callee) 0x47494` | `r0` | `(r1 & 0xff)` | `sp-29` | `sp-30` |
| `bl` at 0x46898 | `(unknown callee) 0x47004` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x468ac | `(unknown callee) 0x47130` | `[sp-20]` | `[ret(0x47004)]:1` | `?` | `[ret(0x47004)]:1` |
| `bl` at 0x46930 | `(unknown callee) 0x1d38c` | `0x5` | `[sp-32]:1` | `?` | `[sp-32]:1` |
| `bl` at 0x4693c | `(unknown callee) 0x1d38c` | `0x4` | `0x1` | `?` | `?` |
| `bl` at 0x46988 | `(unknown callee) 0xd6a1c` | `[sp-20]` | `[[sp-28]+3]:1` | `[[sp-28]+5]:1` | `[sp-32]:1` |
| `bl` at 0x469dc | `(unknown callee) 0xcd47c` | `[sp-20]` | `[ret(0x47130)+3]:1` | `[ret(0x47130)+4]:1` | `[sp-22]:1` |
| `bl` at 0x46a14 | `(unknown callee) 0x1d38c` | `0x5` | `[sp-22]:1` | `?` | `[sp-22]:1` |
| `bl` at 0x46a20 | `(unknown callee) 0x1d38c` | `0x4` | `0x1` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x46830 | `ret(0x47494) == 0x0` | `ret(0x47494) != 0x0` |
| `bls` at 0x46854 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x46874 | `Z == 1` | `not (Z == 1)` |
| `bhs` at 0x46888 | `C == 1` | `not (C == 1)` |
| `beq` at 0x468cc | `[ret(0x47130)]:1 == 0x2` | `[ret(0x47130)]:1 != 0x2` |
| `bgt` at 0x468d8 | `[ret(0x47130)]:1 > 0x2 (signed, 32-bit)` | `[ret(0x47130)]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0x468e4 | `[ret(0x47130)]:1 == 0x1` | `[ret(0x47130)]:1 != 0x1` |
| `beq` at 0x468f4 | `[ret(0x47130)]:1 == 0x63` | `[ret(0x47130)]:1 != 0x63` |
| `bne` at 0x46908 | `[ret(0x47130)+3]:1 != 0xff` | `[ret(0x47130)+3]:1 == 0xff` |
| `beq` at 0x4699c | `ret(0xd6a1c) == 0x0` | `ret(0xd6a1c) != 0x0` |
| `bne` at 0x469a8 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `beq` at 0x469f0 | `ret(0xcd47c) == 0x0` | `ret(0xcd47c) != 0x0` |
| `bne` at 0x469fc | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x46a4c | `r0` | `?` |

## `fanSetSpeed`

- `fanSetSpeed`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x46a5c to 0x46c64.
  Established. Three selector equality edges take their arms for `[sp-22]:1 == 0x0`, `0x1`, and
  `0x2`, respectively, falling through on inequality (`beq` at 0x46bb0, `beq` at 0x46b98, `beq` at
  0x46bc0). Their byte stores write `([ret(0x47130)+7]:1 & 0xff)`, `([ret(0x47130)+8]:1 & 0xff)`,
  and `([ret(0x47130)+9]:1 & 0xff)` at `sp -0x1d`, respectively (`strb` at 0x46bd0, `strb` at
  0x46be0, `strb` at 0x46bf0). The later call to `0x467bc` carries `[sp-20]`, `[sp-32]:1`, unknown,
  `[sp-32]:1` (`bl` at 0x46c10). Physical speed settings and the merged r0 return are unknown
  (`ldmdb` at 0x46c58).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x46a6c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x46a74 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x46a7c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0x46a80 | load | 4 | `abs 0x46c5c` | `-` |
| `ldr` at 0x46a84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x46a98 | load | 4 | `abs 0x14803c` | `-` |
| `str` at 0x46aa0 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0x46aa4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x46ab0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x46ac4 | store | 4 | `sp -0x24` | `ret(0x47494)` |
| `ldr` at 0x46ac8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x46ad4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x46ad8 | store | 4 | `sp -0x30` | `ret(0x47494)` |
| `ldrb` at 0x46ae0 | load | 1 | `sp -0x1e` | `-` |
| `strb` at 0x46ae4 | store | 1 | `sp -0x20` | `([sp-30]:1 & 0xff)` |
| `ldrb` at 0x46ae8 | load | 1 | `sp -0x20` | `-` |
| `ldrb` at 0x46aec | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x46afc | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x46b00 | load | 1 | `sp -0x20` | `-` |
| `ldrb` at 0x46b04 | load | 1 | `[sp-44] +0x4` | `-` |
| `ldr` at 0x46b10 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x46b14 | load | 1 | `sp -0x20` | `-` |
| `ldr` at 0x46b18 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldrb` at 0x46b20 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x46b30 | load | 1 | `sp -0x20` | `-` |
| `ldr` at 0x46b34 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x46b44 | load | 1 | `ret(0x47004) +0x0` | `-` |
| `ldr` at 0x46b48 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x46b58 | store | 4 | `sp -0x1c` | `ret(0x47130)` |
| `ldr` at 0x46b5c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46b60 | load | 1 | `ret(0x47130) +0x3` | `-` |
| `ldrb` at 0x46b6c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x46b7c | load | 4 | `abs 0x46c60` | `-` |
| `str` at 0x46b80 | store | 4 | `sp -0x30` | `0xfffffa87` |
| `ldrb` at 0x46b88 | load | 1 | `sp -0x16` | `-` |
| `str` at 0x46b8c | store | 4 | `sp -0x34` | `[sp-22]:1` |
| `ldr` at 0x46b90 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x46b9c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x46ba8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x46bb8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x46bc8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46bcc | load | 1 | `ret(0x47130) +0x7` | `-` |
| `strb` at 0x46bd0 | store | 1 | `sp -0x1d` | `([ret(0x47130)+7]:1 & 0xff)` |
| `ldr` at 0x46bd8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46bdc | load | 1 | `ret(0x47130) +0x8` | `-` |
| `strb` at 0x46be0 | store | 1 | `sp -0x1d` | `([ret(0x47130)+8]:1 & 0xff)` |
| `ldr` at 0x46be8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46bec | load | 1 | `ret(0x47130) +0x9` | `-` |
| `strb` at 0x46bf0 | store | 1 | `sp -0x1d` | `([ret(0x47130)+9]:1 & 0xff)` |
| `ldrb` at 0x46bf8 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x46bfc | store | 1 | `sp -0x1d` | `([sp-22]:1 & 0xff)` |
| `ldrb` at 0x46c00 | load | 1 | `sp -0x20` | `-` |
| `ldrb` at 0x46c04 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x46c08 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x46c18 | store | 4 | `sp -0x28` | `ret(0x467bc)` |
| `ldr` at 0x46c1c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x46c28 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x46c34 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x46c38 | store | 4 | `sp -0x24` | `ret(0x467bc)` |
| `ldrb` at 0x46c3c | load | 1 | `sp -0x20` | `-` |
| `strb` at 0x46c44 | store | 1 | `sp -0x20` | `?` |
| `ldr` at 0x46c4c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x46c50 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x46c54 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0x46c58 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x46c58 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x46c58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x46abc | `(unknown callee) 0x47494` | `r0` | `(r1 & 0xff)` | `sp-30` | `sp-31` |
| `bl` at 0x46b3c | `(unknown callee) 0x47004` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x46b50 | `(unknown callee) 0x47130` | `[sp-20]` | `[ret(0x47004)]:1` | `?` | `[ret(0x47004)]:1` |
| `bl` at 0x46c10 | `(unknown callee) 0x467bc` | `[sp-20]` | `[sp-32]:1` | `?` | `[sp-32]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x46ad0 | `ret(0x47494) == 0x0` | `ret(0x47494) != 0x0` |
| `bls` at 0x46af4 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bhs` at 0x46b0c | `C == 1` | `not (C == 1)` |
| `bne` at 0x46b28 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x46b68 | `[ret(0x47130)+3]:1 != 0xff` | `[ret(0x47130)+3]:1 == 0xff` |
| `bne` at 0x46b74 | `[sp-21]:1 != 0xff` | `[sp-21]:1 == 0xff` |
| `beq` at 0x46b98 | `[sp-22]:1 == 0x1` | `[sp-22]:1 != 0x1` |
| `bgt` at 0x46ba4 | `[sp-22]:1 > 0x1 (signed, 32-bit)` | `[sp-22]:1 <= 0x1 (signed, 32-bit)` |
| `beq` at 0x46bb0 | `[sp-22]:1 == 0x0` | `[sp-22]:1 != 0x0` |
| `beq` at 0x46bc0 | `[sp-22]:1 == 0x2` | `[sp-22]:1 != 0x2` |
| `beq` at 0x46c24 | `ret(0x467bc) == 0x0` | `ret(0x467bc) != 0x0` |
| `bne` at 0x46c30 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x46c58 | `r0` | `?` |

## `fanGetReading`

- `fanGetReading`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x46df0 to 0x46f9c.
  Established. The computed transfer takes an unresolved edge for `? <= 0x7` unsigned and falls
  through above that bound (`ldrls` at 0x46e64). The known fall-through route stores word
  `0x3f800000` at `sp -0x24` (`str` at 0x46e9c). A later call to `0xcd334` carries `[sp-20]`,
  `[[sp-40]+1]:1`, `[[sp-40]+2]:1`, `[sp-28]` (`bl` at 0x46ed8). The final indirect word store
  writes `ret(0xfba94)` at `[sp-48] +0x0` (`str` at 0x46f74). Excluded selector cases, reading units
  and the joined return remain unknown (`ldmdb` at 0x46f84).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x46e00 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x46e08 | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0x46e0c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x46e10 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x46e14 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x46e24 | store | 4 | `sp -0x28` | `ret(0x47004)` |
| `ldrb` at 0x46e28 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x46e2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x46e44 | load | 4 | `abs 0x46f88` | `-` |
| `str` at 0x46e48 | store | 4 | `sp -0x2c` | `0xfffffa85` |
| `ldr` at 0x46e50 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x46e54 | load | 1 | `[sp-40] +0x8` | `-` |
| `ldrls` at 0x46e64 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x46e98 | load | 4 | `abs 0x46f90` | `-` |
| `str` at 0x46e9c | store | 4 | `sp -0x24` | `0x3f800000` |
| `ldr` at 0x46eb8 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x46ebc | load | 1 | `[sp-40] +0x1` | `-` |
| `ldr` at 0x46ec0 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x46ec4 | load | 1 | `[sp-40] +0x2` | `-` |
| `ldr` at 0x46ec8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x46ed4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x46ee0 | store | 4 | `sp -0x20` | `ret(0xcd334)` |
| `ldr` at 0x46ee4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x46ee8 | store | 4 | `sp -0x30` | `[sp-28]` |
| `ldr` at 0x46eec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x46ef0 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0x46ef4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x46ef8 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x46f04 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x46f08 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x46f10 | store | 4 | `sp -0x38` | `ret(0xfb614)` |
| `ldr` at 0x46f18 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x46f1c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x46f24 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x46f28 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x46f3c | store | 4 | `sp -0x38` | `ret(0xfb614)` |
| `ldr` at 0x46f40 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x46f44 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x46f50 | store | 4 | `sp -0x38` | `ret(0xfb44c)` |
| `ldr` at 0x46f54 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x46f58 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x46f70 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x46f74 | store | 4 | `[sp-48] +0x0` | `ret(0xfba94)` |
| `ldr` at 0x46f78 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x46f7c | store | 4 | `sp -0x2c` | `[sp-32]` |
| `ldr` at 0x46f80 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x46f84 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x46f84 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x46f84 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x46e1c | `(unknown callee) 0x47004` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x46e34 | `(unknown callee) 0x45688` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x46ed8 | `(unknown callee) 0xcd334` | `[sp-20]` | `[[sp-40]+1]:1` | `[[sp-40]+2]:1` | `[sp-28]` |
| `bl` at 0x46f0c | `(unknown callee) 0xfb614` | `[[sp-28]]` | `[sp-28]` | `[sp-28]` | `[[sp-28]]` |
| `bl` at 0x46f38 | `(unknown callee) 0xfb614` | `?` | `[sp-28]` | `?` | `?` |
| `bl` at 0x46f48 | `(unknown callee) 0xfb44c` | `ret(0xfb614)` | `ret(0xfb614)` | `?` | `?` |
| `bl` at 0x46f5c | `(unknown callee) 0xfb7f0` | `?` | `[sp-36]` | `?` | `?` |
| `bl` at 0x46f68 | `(unknown callee) 0xfba94` | `ret(0xfb7f0)` | `?` | `?` | `ret(0xfb7f0)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x46e40 | `ret(0x45688) != 0x0` | `ret(0x45688) == 0x0` |
| `ldrls` at 0x46e64 | `? <= 0x7 (unsigned, 32-bit)` | `? > 0x7 (unsigned, 32-bit)` |
| `blt` at 0x46f00 | `[[sp-28]] < 0x0 (signed, 32-bit)` | `[[sp-28]] >= 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x46f84 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x46e64).

## `autofanInit`

- `autofanInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x308f8 to 0x30a68.
  Established. Absolute word stores write unknown at `0x147f24`, `[sp-28]` at `0x147f28`, and later
  `ret(0x1b124)` at `0x147f2c` (`str` at 0x3095c, `str` at 0x30968, `str` at 0x30a10). The test
  takes its alternate arm for `ret(0x1f1dc) != 0x0` and falls through on equality (`bne` at
  0x3097c). Alternative byte stores put `0x1`, `0x2`, or `0x0` at `sp -0x29` (`strb` at 0x30984,
  `strb` at 0x309a0, `strb` at 0x309ac). The final call to `0x1b43c` has `[0x147f2c]`, `0x0`,
  unknown, unknown and supplies the r0 return (`bl` at 0x30a44, `ldmdb` at 0x30a48). Automatic
  control policy and physical initialization are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x30908 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x3090c | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x30910 | load | 4 | `abs 0x30a4c` | `-` |
| `ldr` at 0x30914 | load | 4 | `abs 0x147f24` | `-` |
| `ldr` at 0x30920 | load | 4 | `abs 0x30a50` | `-` |
| `str` at 0x30924 | store | 4 | `sp -0x3c` | `0x10514c` |
| `ldr` at 0x30928 | load | 4 | `abs 0x30a4c` | `-` |
| `ldr` at 0x3092c | load | 4 | `abs 0x147f24` | `-` |
| `str` at 0x30930 | store | 4 | `sp -0x38` | `[0x147f24]` |
| `ldr` at 0x30934 | load | 4 | `abs 0x30a54` | `-` |
| `ldr` at 0x30938 | load | 4 | `abs 0x147f28` | `-` |
| `str` at 0x3093c | store | 4 | `sp -0x34` | `[0x147f28]` |
| `ldr` at 0x30944 | load | 4 | `abs 0x30a58` | `-` |
| `ldr` at 0x30954 | load | 4 | `abs 0x30a4c` | `-` |
| `ldr` at 0x30958 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3095c | store | 4 | `abs 0x147f24` | `?` |
| `ldr` at 0x30960 | load | 4 | `abs 0x30a54` | `-` |
| `ldr` at 0x30964 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x30968 | store | 4 | `abs 0x147f28` | `[sp-28]` |
| `strb` at 0x30984 | store | 1 | `sp -0x29` | `0x1` |
| `strb` at 0x309a0 | store | 1 | `sp -0x29` | `0x2` |
| `strb` at 0x309ac | store | 1 | `sp -0x29` | `0x0` |
| `str` at 0x309b4 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x309b8 | load | 4 | `abs 0x30a54` | `-` |
| `ldr` at 0x309bc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x309c0 | load | 4 | `abs 0x147f28` | `-` |
| `ldr` at 0x309d0 | load | 4 | `abs 0x30a5c` | `-` |
| `ldr` at 0x309d4 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0x309e0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x309e4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x309ec | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x309f4 | load | 4 | `abs 0x30a60` | `-` |
| `ldr` at 0x309f8 | load | 4 | `abs 0x30a54` | `-` |
| `ldr` at 0x309fc | load | 4 | `abs 0x147f28` | `-` |
| `str` at 0x30a10 | store | 4 | `abs 0x147f2c` | `ret(0x1b124)` |
| `ldr` at 0x30a14 | load | 4 | `abs 0x147f2c` | `-` |
| `ldr` at 0x30a20 | load | 4 | `abs 0x30a64` | `-` |
| `ldr` at 0x30a28 | load | 4 | `abs 0x30a60` | `-` |
| `ldr` at 0x30a2c | load | 4 | `abs 0x30a54` | `-` |
| `ldr` at 0x30a30 | load | 4 | `abs 0x147f28` | `-` |
| `ldr` at 0x30a38 | load | 4 | `abs 0x147f2c` | `-` |
| `ldmdb` at 0x30a48 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x30a48 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x30a48 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x30a48 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x30950 | `(unknown callee) 0x21e28` | `0x9` | `0x10517c` | `0x43` | `0x0` |
| `bl` at 0x30970 | `(unknown callee) 0x1f1dc` | `0x6` | `?` | `0x147f28` | `[sp-28]` |
| `bl` at 0x3098c | `(unknown callee) 0x3d8e0` | `ret(0x1f1dc)` | `?` | `?` | `ret(0x1f1dc)` |
| `bl` at 0x30a08 | `(unknown callee) 0x1b124` | `?` | `?` | `?` | `?` |
| `bl` at 0x30a24 | `(unknown callee) 0x20240` | `0x105194` | `?` | `?` | `[0x147f2c]` |
| `bl` at 0x30a44 | `(unknown callee) 0x1b43c` | `[0x147f2c]` | `0x0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3091c | `[0x147f24] == 0x0` | `[0x147f24] != 0x0` |
| `bne` at 0x3097c | `ret(0x1f1dc) != 0x0` | `ret(0x1f1dc) == 0x0` |
| `beq` at 0x30998 | `ret(0x3d8e0) == 0x0` | `ret(0x3d8e0) != 0x0` |
| `blt` at 0x309c8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x30a1c | `[0x147f2c] != 0x0` | `[0x147f2c] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x30a48 | `r0` | `ret(0x1b43c)` |

## `AutofanEnvInit`

- `AutofanEnvInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x32a6c to 0x32ad4.
  Established. The opening call to `0x824e4` carries entry r0 through r3 (`bl` at 0x32a84).
  Subsequent stack word stores include `0x800` at `sp -0x2c`, `0xa` at `sp -0x28` and `sp -0x24`,
  and `0x9` at `sp -0x20` (`str` at 0x32a94, `str` at 0x32a9c, `str` at 0x32aa4, `str` at 0x32aac).
  The final call targets `0x223f0` with `0x147f3c`, `0x105438`, `0x31e18`, `[sp-20]` and supplies r0
  at return (`bl` at 0x32ac0, `ldmdb` at 0x32ac4). The control task and stored parameters have
  unknown roles.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x32a7c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x32a80 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x32a88 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x32a8c | store | 4 | `sp -0x30` | `[sp-24]` |
| `str` at 0x32a94 | store | 4 | `sp -0x2c` | `0x800` |
| `str` at 0x32a9c | store | 4 | `sp -0x28` | `0xa` |
| `str` at 0x32aa4 | store | 4 | `sp -0x24` | `0xa` |
| `str` at 0x32aac | store | 4 | `sp -0x20` | `0x9` |
| `ldr` at 0x32ab0 | load | 4 | `abs 0x32ac8` | `-` |
| `ldr` at 0x32ab4 | load | 4 | `abs 0x32acc` | `-` |
| `ldr` at 0x32ab8 | load | 4 | `abs 0x32ad0` | `-` |
| `ldr` at 0x32abc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x32ac4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x32ac4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x32ac4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x32a84 | `(unknown callee) 0x824e4` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x32ac0 | `(unknown callee) 0x223f0` | `0x147f3c` | `0x105438` | `0x31e18` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x32ac4 | `r0` | `ret(0x223f0)` |

## `vsc055_Init`

- `vsc055_Init`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd8344 to 0xd8564.
  Established. The initial equality test takes its bypass for `[0x142b10] == 0x2ad224` and falls
  through on inequality (`beq` at 0xd8370). A later zero arm writes word `0x1` at `0x142b14` (`bne`
  at 0xd83c0, `str` at 0xd83cc). The call to `0x1b3c4` carries `[sp-40]`, `0x1`, `[0x142b14]`,
  unknown, with its result stored as a word at `0x142b10` (`bl` at 0xd83f8, `str` at 0xd8400). The
  final call to `0x1b43c` has `[[sp-36]+8]`, `0x1`, `[[sp-36]+4]`, `[sp-36]` and supplies the r0
  return (`bl` at 0xd8538, `ldmdb` at 0xd853c). Physical device initialization and record-field
  meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd8354 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xd8358 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xd835c | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0xd8360 | load | 4 | `abs 0xd8540` | `-` |
| `ldr` at 0xd8364 | load | 4 | `abs 0x142b10` | `-` |
| `ldr` at 0xd8368 | load | 4 | `abs 0xd8544` | `-` |
| `ldr` at 0xd8374 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd8380 | load | 4 | `abs 0xd8548` | `-` |
| `str` at 0xd8384 | store | 4 | `sp -0x34` | `0x134b48` |
| `ldr` at 0xd8388 | load | 4 | `abs 0xd8540` | `-` |
| `ldr` at 0xd838c | load | 4 | `abs 0x142b10` | `-` |
| `str` at 0xd8390 | store | 4 | `sp -0x30` | `[0x142b10]` |
| `ldr` at 0xd8394 | load | 4 | `abs 0xd854c` | `-` |
| `ldr` at 0xd8398 | load | 4 | `abs 0x142b14` | `-` |
| `str` at 0xd839c | store | 4 | `sp -0x2c` | `[0x142b14]` |
| `ldr` at 0xd83a4 | load | 4 | `abs 0xd8550` | `-` |
| `ldr` at 0xd83b4 | load | 4 | `abs 0xd854c` | `-` |
| `ldr` at 0xd83b8 | load | 4 | `abs 0x142b14` | `-` |
| `ldr` at 0xd83c4 | load | 4 | `abs 0xd854c` | `-` |
| `str` at 0xd83cc | store | 4 | `abs 0x142b14` | `0x1` |
| `ldr` at 0xd83d0 | load | 4 | `abs 0xd854c` | `-` |
| `ldr` at 0xd83d4 | load | 4 | `abs 0x142b14` | `-` |
| `str` at 0xd83e8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xd83ec | load | 4 | `abs 0xd8540` | `-` |
| `ldr` at 0xd83f0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd8400 | store | 4 | `abs 0x142b10` | `ret(0x1b3c4)` |
| `ldr` at 0xd8404 | load | 4 | `abs 0x142b10` | `-` |
| `ldr` at 0xd8410 | load | 4 | `abs 0xd8554` | `-` |
| `str` at 0xd8414 | store | 4 | `sp -0x34` | `0x134b94` |
| `ldr` at 0xd8418 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd841c | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xd8424 | load | 4 | `abs 0xd8550` | `-` |
| `ldr` at 0xd8434 | load | 4 | `abs 0xd8540` | `-` |
| `ldr` at 0xd8438 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd844c | load | 4 | `abs 0x142b10` | `-` |
| `str` at 0xd8454 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd8458 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd845c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd8460 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0xd8464 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd8468 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd846c | store | 4 | `[sp-36] +0x4` | `[sp-32]` |
| `ldr` at 0xd8470 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd8474 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd8478 | load | 4 | `[sp-36] +0x4` | `-` |
| `str` at 0xd8488 | store | 4 | `[sp-36] +0x8` | `ret(0x1b3c4)` |
| `ldr` at 0xd848c | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0xd8498 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd84a0 | load | 4 | `abs 0xd8558` | `-` |
| `ldr` at 0xd84a4 | load | 4 | `abs 0xd855c` | `-` |
| `str` at 0xd84d0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xd84d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd84dc | load | 4 | `abs 0xd8558` | `-` |
| `ldr` at 0xd84e0 | load | 4 | `abs 0xd855c` | `-` |
| `ldr` at 0xd84ec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd84fc | load | 4 | `abs 0xd8560` | `-` |
| `str` at 0xd8500 | store | 4 | `(unknown)` | `0x134bd8` |
| `str` at 0xd8504 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0xd8508 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd850c | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0xd8514 | load | 4 | `abs 0xd8550` | `-` |
| `ldr` at 0xd8524 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd8528 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd852c | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0xd8534 | load | 4 | `[sp-36] +0x4` | `-` |
| `ldmdb` at 0xd853c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd853c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd853c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd853c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd83b0 | `(unknown callee) 0x21e28` | `0x9` | `0x134b78` | `0x47` | `0x0` |
| `bl` at 0xd83f8 | `(unknown callee) 0x1b3c4` | `[sp-40]` | `0x1` | `[0x142b14]` | `?` |
| `bl` at 0xd8430 | `(unknown callee) 0x21e28` | `0x9` | `0x134b78` | `0x53` | `0x0` |
| `bl` at 0xd8480 | `(unknown callee) 0x1b3c4` | `[[sp-36]+4]` | `0x1` | `[sp-36]` | `[sp-36]` |
| `bl` at 0xd84a8 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x134bcc` | `0x134bd4` |
| `bl` at 0xd84b4 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xd84e4 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x134bcc` | `0x134bd4` |
| `bl` at 0xd84f4 | `(unknown callee) 0x1b508` | `[sp-40]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xd8520 | `(unknown callee) 0x21e28` | `0x9` | `0x134b78` | `0x5f` | `0x0` |
| `bl` at 0xd8538 | `(unknown callee) 0x1b43c` | `[[sp-36]+8]` | `0x1` | `[[sp-36]+4]` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd8370 | `[0x142b10] == 0x2ad224` | `[0x142b10] != 0x2ad224` |
| `bne` at 0xd837c | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xd83c0 | `[0x142b14] != 0x0` | `[0x142b14] == 0x0` |
| `bne` at 0xd840c | `[0x142b10] != 0x0` | `[0x142b10] == 0x0` |
| `bne` at 0xd8494 | `[[sp-36]+8] != 0x0` | `[[sp-36]+8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd853c | `r0` | `ret(0x1b43c)` |

## `vsc055_Enable`

- `vsc055_Enable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd8564 to 0xd8720.
  Established. The signed bound takes its alternate path for `(r1 & 0xff) >= [[sp-28]+4]` and falls
  through for less (`bge` at 0xd85b4). The latter path calls `0xd687c` with entry r0, `sp-36`,
  unknown, `sp-36` (`bl` at 0xd85dc). A later call to `0xd5e04` carries `[sp-20]`, unknown,
  `[sp-32]`, unknown (`bl` at 0xd8640). After the loop, a byte store writes `0x1` at `([sp-21]:1 +
  [[sp-28]+8]) +0x0` (`strb` at 0xd8674). Physical enabling and the joined r0 value remain unproven
  (`ldmdb` at 0xd8708).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd8574 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd857c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xd8580 | load | 4 | `abs 0xd870c` | `-` |
| `ldr` at 0xd8584 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8598 | load | 4 | `abs 0x142b10` | `-` |
| `str` at 0xd85a0 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xd85a4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd85a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd85ac | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xd85b8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd85bc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd85c4 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xd85cc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xd85d4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd85e4 | store | 4 | `sp -0x28` | `ret(0xd687c)` |
| `strb` at 0xd85ec | store | 1 | `sp -0x29` | `0x0` |
| `ldrb` at 0xd85f0 | load | 1 | `sp -0x29` | `-` |
| `ldr` at 0xd85f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd8604 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd8608 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd860c | load | 1 | `[sp-36] +0x1` | `-` |
| `ldrb` at 0xd8610 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xd861c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd8620 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd8624 | load | 1 | `[sp-36] +0x2` | `-` |
| `ldrb` at 0xd8628 | load | 1 | `[sp-32] +0x1` | `-` |
| `ldrb` at 0xd8634 | load | 1 | `sp -0x29` | `-` |
| `ldr` at 0xd8638 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8644 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd864c | store | 4 | `sp -0x24` | `[sp-36]+16` |
| `ldrb` at 0xd8650 | load | 1 | `sp -0x29` | `-` |
| `strb` at 0xd8658 | store | 1 | `sp -0x29` | `?` |
| `ldr` at 0xd8660 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd8664 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd8668 | load | 4 | `[sp-28] +0x8` | `-` |
| `strb` at 0xd8674 | store | 1 | `([sp-21]:1 + [[sp-28]+8]) +0x0` | `0x1` |
| `ldr` at 0xd867c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8684 | load | 4 | `abs 0xd8710` | `-` |
| `ldr` at 0xd8688 | load | 4 | `abs 0xd8714` | `-` |
| `str` at 0xd86b4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd86b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd86c0 | load | 4 | `abs 0xd8710` | `-` |
| `ldr` at 0xd86c4 | load | 4 | `abs 0xd8714` | `-` |
| `ldr` at 0xd86d0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd86e0 | load | 4 | `abs 0xd8718` | `-` |
| `str` at 0xd86e4 | store | 4 | `(unknown)` | `0x134c14` |
| `str` at 0xd86e8 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldrb` at 0xd86ec | load | 1 | `sp -0x15` | `-` |
| `str` at 0xd86f0 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0xd86f8 | load | 4 | `abs 0xd871c` | `-` |
| `ldmdb` at 0xd8708 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd8708 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd8708 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd85dc | `(unknown callee) 0xd687c` | `r0` | `sp-36` | `?` | `sp-36` |
| `bl` at 0xd8640 | `(unknown callee) 0xd5e04` | `[sp-20]` | `?` | `[sp-32]` | `?` |
| `bl` at 0xd868c | `(unknown callee) 0x4f564` | `r0` | `0x0` | `0x134bcc` | `0x134bd4` |
| `bl` at 0xd8698 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xd86c8 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x134bcc` | `0x134bd4` |
| `bl` at 0xd86d8 | `(unknown callee) 0x1b508` | `[sp-48]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xd8704 | `(unknown callee) 0x21e28` | `0x6` | `0x134b78` | `0x7e` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xd85b4 | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` |
| `blt` at 0xd85fc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xd8618 | `[[sp-36]+1]:1 != [[sp-32]]:1` | `[[sp-36]+1]:1 == [[sp-32]]:1` |
| `bne` at 0xd8630 | `[[sp-36]+2]:1 != [[sp-32]+1]:1` | `[[sp-36]+2]:1 == [[sp-32]+1]:1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd8708 | `r0` | `?` |

## `vsc055_Disable`

- `vsc055_Disable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd8720 to 0xd88d8.
  Established. The first signed test takes the alternate path for `(r1 & 0xff) >= [[sp-28]+4]` and
  falls through for less (`bge` at 0xd8770). The fall-through byte store clears `((r1 & 0xff) +
  [[sp-28]+8]) +0x0` (`strb` at 0xd8788); its later call to `0xd5f28` uses `[sp-20]`, unknown,
  `[sp-32]`, unknown (`bl` at 0xd8814). Device-disable effects and the final r0 return remain
  unknown (`ldmdb` at 0xd88c0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd8730 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd8738 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xd873c | load | 4 | `abs 0xd88c4` | `-` |
| `ldr` at 0xd8740 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8754 | load | 4 | `abs 0x142b10` | `-` |
| `str` at 0xd875c | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xd8760 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd8764 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd8768 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xd8774 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd8778 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd877c | load | 4 | `[sp-28] +0x8` | `-` |
| `strb` at 0xd8788 | store | 1 | `((r1 & 0xff) + [[sp-28]+8]) +0x0` | `0x0` |
| `ldr` at 0xd878c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd8790 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd8798 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xd87a0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xd87a8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd87b8 | store | 4 | `sp -0x28` | `ret(0xd687c)` |
| `strb` at 0xd87c0 | store | 1 | `sp -0x29` | `0x0` |
| `ldrb` at 0xd87c4 | load | 1 | `sp -0x29` | `-` |
| `ldr` at 0xd87c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd87d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd87dc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd87e0 | load | 1 | `[sp-36] +0x1` | `-` |
| `ldrb` at 0xd87e4 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xd87f0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd87f4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd87f8 | load | 1 | `[sp-36] +0x2` | `-` |
| `ldrb` at 0xd87fc | load | 1 | `[sp-32] +0x1` | `-` |
| `ldrb` at 0xd8808 | load | 1 | `sp -0x29` | `-` |
| `ldr` at 0xd880c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8818 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd8820 | store | 4 | `sp -0x24` | `[sp-36]+16` |
| `ldrb` at 0xd8824 | load | 1 | `sp -0x29` | `-` |
| `strb` at 0xd882c | store | 1 | `sp -0x29` | `?` |
| `ldr` at 0xd8834 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd883c | load | 4 | `abs 0xd88c8` | `-` |
| `ldr` at 0xd8840 | load | 4 | `abs 0xd88cc` | `-` |
| `str` at 0xd886c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd8870 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8878 | load | 4 | `abs 0xd88c8` | `-` |
| `ldr` at 0xd887c | load | 4 | `abs 0xd88cc` | `-` |
| `ldr` at 0xd8888 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd8898 | load | 4 | `abs 0xd88d0` | `-` |
| `str` at 0xd889c | store | 4 | `(unknown)` | `0x134c34` |
| `str` at 0xd88a0 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldrb` at 0xd88a4 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xd88a8 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0xd88b0 | load | 4 | `abs 0xd88d4` | `-` |
| `ldmdb` at 0xd88c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd88c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd88c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd87b0 | `(unknown callee) 0xd687c` | `[sp-20]` | `sp-36` | `?` | `sp-36` |
| `bl` at 0xd8814 | `(unknown callee) 0xd5f28` | `[sp-20]` | `?` | `[sp-32]` | `?` |
| `bl` at 0xd8844 | `(unknown callee) 0x4f564` | `r0` | `0x0` | `0x134bcc` | `0x134bd4` |
| `bl` at 0xd8850 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xd8880 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x134bcc` | `0x134bd4` |
| `bl` at 0xd8890 | `(unknown callee) 0x1b508` | `[sp-48]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xd88bc | `(unknown callee) 0x21e28` | `0x6` | `0x134b78` | `0x9e` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xd8770 | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` |
| `blt` at 0xd87d0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xd87ec | `[[sp-36]+1]:1 != [[sp-32]]:1` | `[[sp-36]+1]:1 == [[sp-32]]:1` |
| `bne` at 0xd8804 | `[[sp-36]+2]:1 != [[sp-32]+1]:1` | `[[sp-36]+2]:1 == [[sp-32]+1]:1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd88c0 | `r0` | `?` |

## `vsc055_ResetAll`

- `vsc055_ResetAll`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd88d8 to 0xd8974.
  Established. A byte store initializes `sp -0x19` to `0x0` (`strb` at 0xd8914). The loop test
  retains a less-than flags predicate with unknown operands, following its edge when true and falling
  through otherwise (`blt` at 0xd8928). The only call targets `0xd8974` with four unknown candidates
  (`bl` at 0xd8958). The aggregate reset effect and returned r0 value are unproven (`ldmdb` at
  0xd896c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd88e8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xd88ec | load | 4 | `abs 0xd8970` | `-` |
| `ldr` at 0xd88f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8904 | load | 4 | `abs 0x142b10` | `-` |
| `str` at 0xd890c | store | 4 | `sp -0x18` | `?` |
| `strb` at 0xd8914 | store | 1 | `sp -0x19` | `0x0` |
| `ldrb` at 0xd8918 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xd891c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd8920 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xd8930 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd8934 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xd8938 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0xd8940 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xd894c | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xd8950 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xd895c | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xd8964 | store | 1 | `sp -0x19` | `?` |
| `ldmdb` at 0xd896c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd896c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd896c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd8958 | `(unknown callee) 0xd8974` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xd8928 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xd8948 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd896c | `r0` | `?` |

## `vsc055_Reset`

- `vsc055_Reset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd8974 to 0xd8c50.
  Established. The signed bound takes its continuation for `(r1 & 0xff) <= [[sp-28]+4]` and falls
  through for greater (`ble` at 0xd89c4). The call to `0xc7f7c` carries unknown, `[[sp-32]+1]:1`,
  `0xfc`, `0x80` (`bl` at 0xd8ad8); its zero result takes the route through `0xd9e4c`, which has
  `0x2`, unknown, unknown, `ret(0xc7f7c)` (`beq` at 0xd8aec, `bl` at 0xd8af8). The subsequent call
  to `0xc80a0` uses unknown, `[[sp-32]+1]:1`, `0xff`, `sp-33` (`bl` at 0xd8b34). Physical reset
  timing, reply meaning and the joined return remain unknown (`ldmdb` at 0xd8c30).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd8984 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd898c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xd8990 | load | 4 | `abs 0xd8c34` | `-` |
| `ldr` at 0xd8994 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd89a8 | load | 4 | `abs 0x142b10` | `-` |
| `str` at 0xd89b0 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xd89b4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd89b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd89bc | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xd89c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd89d0 | load | 4 | `abs 0xd8c38` | `-` |
| `ldr` at 0xd89d4 | load | 4 | `abs 0xd8c3c` | `-` |
| `str` at 0xd8a00 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd8a04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8a0c | load | 4 | `abs 0xd8c38` | `-` |
| `ldr` at 0xd8a10 | load | 4 | `abs 0xd8c3c` | `-` |
| `ldr` at 0xd8a1c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd8a2c | load | 4 | `abs 0xd8c40` | `-` |
| `str` at 0xd8a30 | store | 4 | `(unknown)` | `0x134c58` |
| `str` at 0xd8a34 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldrb` at 0xd8a38 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xd8a3c | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0xd8a44 | load | 4 | `abs 0xd8c44` | `-` |
| `ldr` at 0xd8a54 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd8a58 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd8a5c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0xd8a64 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xd8a74 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd8a78 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd8a80 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xd8a88 | store | 4 | `sp -0x20` | `?` |
| `str` at 0xd8a90 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xd8a94 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd8aa4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd8aa8 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xd8aac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8ac0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd8ac4 | load | 1 | `[sp-32] +0x1` | `-` |
| `str` at 0xd8ae0 | store | 4 | `sp -0x28` | `ret(0xc7f7c)` |
| `ldr` at 0xd8ae4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd8afc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd8b00 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xd8b04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8b18 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd8b1c | load | 1 | `[sp-32] +0x1` | `-` |
| `str` at 0xd8b3c | store | 4 | `sp -0x28` | `ret(0xc80a0)` |
| `ldr` at 0xd8b40 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xd8b50 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xd8b60 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd8b68 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xd8b70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8b78 | load | 4 | `abs 0xd8c38` | `-` |
| `ldr` at 0xd8b7c | load | 4 | `abs 0xd8c3c` | `-` |
| `str` at 0xd8ba8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd8bac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8bb4 | load | 4 | `abs 0xd8c38` | `-` |
| `ldr` at 0xd8bb8 | load | 4 | `abs 0xd8c3c` | `-` |
| `ldr` at 0xd8bc4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd8bd0 | store | 4 | `sp -0x34` | `ret(0x1b508)` |
| `ldr` at 0xd8bd4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd8be0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd8be8 | store | 4 | `sp -0x38` | `ret(0x219a8)` |
| `ldr` at 0xd8bf0 | load | 4 | `abs 0xd8c48` | `-` |
| `str` at 0xd8bf4 | store | 4 | `sp -0x38` | `0x134c70` |
| `ldr` at 0xd8bf8 | load | 4 | `abs 0xd8c4c` | `-` |
| `str` at 0xd8bfc | store | 4 | `(unknown)` | `0x134c88` |
| `ldr` at 0xd8c00 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xd8c04 | store | 4 | `(unknown)` | `[sp-52]` |
| `ldr` at 0xd8c08 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd8c0c | load | 4 | `[sp-32] +0x4` | `-` |
| `str` at 0xd8c10 | store | 4 | `(unknown)` | `[[sp-32]+4]` |
| `ldr` at 0xd8c14 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xd8c18 | store | 4 | `(unknown)` | `[sp-56]` |
| `ldr` at 0xd8c20 | load | 4 | `abs 0xd8c44` | `-` |
| `ldmdb` at 0xd8c30 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd8c30 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd8c30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd89d8 | `(unknown callee) 0x4f564` | `r0` | `0x0` | `0x134bcc` | `0x134bd4` |
| `bl` at 0xd89e4 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xd8a14 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x134bcc` | `0x134bd4` |
| `bl` at 0xd8a24 | `(unknown callee) 0x1b508` | `[sp-48]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xd8a50 | `(unknown callee) 0x21e28` | `0x9` | `0x134b78` | `0xc1` | `0x0` |
| `bl` at 0xd8ab4 | `(unknown callee) 0xc7784` | `?` | `[[sp-32]]:1` | `?` | `[[sp-32]]:1` |
| `bl` at 0xd8ad8 | `(unknown callee) 0xc7f7c` | `?` | `[[sp-32]+1]:1` | `0xfc` | `0x80` |
| `bl` at 0xd8af8 | `(unknown callee) 0xd9e4c` | `0x2` | `?` | `?` | `ret(0xc7f7c)` |
| `bl` at 0xd8b0c | `(unknown callee) 0xc7784` | `[sp-20]` | `[[sp-32]]:1` | `?` | `[[sp-32]]:1` |
| `bl` at 0xd8b34 | `(unknown callee) 0xc80a0` | `?` | `[[sp-32]+1]:1` | `0xff` | `sp-33` |
| `bl` at 0xd8b80 | `(unknown callee) 0x4f564` | `?` | `0x0` | `0x134bcc` | `0x134bd4` |
| `bl` at 0xd8b8c | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xd8bbc | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x134bcc` | `0x134bd4` |
| `bl` at 0xd8bcc | `(unknown callee) 0x1b508` | `[sp-48]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xd8be4 | `(unknown callee) 0x219a8` | `[sp-40]` | `?` | `?` | `[sp-40]` |
| `bl` at 0xd8c2c | `(unknown callee) 0x21e28` | `0x3` | `0x134b78` | `0xe1` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xd89c4 | `(r1 & 0xff) <= [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) > [[sp-28]+4] (signed, 32-bit)` |
| `bne` at 0xd8a6c | `Z == 0` | `not (Z == 0)` |
| `ble` at 0xd8a9c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xd8aec | `ret(0xc7f7c) == 0x0` | `ret(0xc7f7c) != 0x0` |
| `beq` at 0xd8b48 | `ret(0xc80a0) == 0x0` | `ret(0xc80a0) != 0x0` |
| `bne` at 0xd8b58 | `[sp-33]:1 != 0x21` | `[sp-33]:1 == 0x21` |
| `beq` at 0xd8bdc | `[sp-40] == 0x0` | `[sp-40] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd8c30 | `r0` | `?` |
