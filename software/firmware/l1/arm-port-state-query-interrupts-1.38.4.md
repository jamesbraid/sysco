# ARM port state, query and interrupt paths in 1.38.4

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

[Fact sheets and full listings](disasm/arm-1.38.4-package4/port-state-query-interrupts-facts.txt) retain all tool sections.

## `portInit`

- `portInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x41208 to 0x41554.
  Established. Word stores write `0x1` at absolute `0x147fe0` and `ret(0x1b3c4)` at `0x147fdc`
  (`str` at 0x4128c, `str` at 0x412c0). The initial zero test takes its bypass and nonzero falls
  through (`beq` at 0x41230). The first call to `0x21e28` has `0x9`, `0x108af4`, `0x6c`, `0x0` (`bl`
  at 0x41270). The exit carries `ret(0x22920)` (`ldmdb` at 0x41528). Port-record ownership and
  physical initialization effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x41218 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x4121c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x41220 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x41224 | load | 4 | `abs 0x4152c` | `-` |
| `ldr` at 0x41228 | load | 4 | `abs 0x147fdc` | `-` |
| `ldr` at 0x41234 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x41240 | load | 4 | `abs 0x41530` | `-` |
| `str` at 0x41244 | store | 4 | `sp -0x38` | `0x108ac4` |
| `ldr` at 0x41248 | load | 4 | `abs 0x4152c` | `-` |
| `ldr` at 0x4124c | load | 4 | `abs 0x147fdc` | `-` |
| `str` at 0x41250 | store | 4 | `sp -0x34` | `[0x147fdc]` |
| `ldr` at 0x41254 | load | 4 | `abs 0x41534` | `-` |
| `ldr` at 0x41258 | load | 4 | `abs 0x147fe0` | `-` |
| `str` at 0x4125c | store | 4 | `sp -0x30` | `[0x147fe0]` |
| `ldr` at 0x41264 | load | 4 | `abs 0x41538` | `-` |
| `ldr` at 0x41274 | load | 4 | `abs 0x41534` | `-` |
| `ldr` at 0x41278 | load | 4 | `abs 0x147fe0` | `-` |
| `ldr` at 0x41284 | load | 4 | `abs 0x41534` | `-` |
| `str` at 0x4128c | store | 4 | `abs 0x147fe0` | `0x1` |
| `ldr` at 0x41290 | load | 4 | `abs 0x41534` | `-` |
| `ldr` at 0x41294 | load | 4 | `abs 0x147fe0` | `-` |
| `str` at 0x412a8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x412ac | load | 4 | `abs 0x4152c` | `-` |
| `ldr` at 0x412b0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x412c0 | store | 4 | `abs 0x147fdc` | `ret(0x1b3c4)` |
| `ldr` at 0x412c4 | load | 4 | `abs 0x147fdc` | `-` |
| `ldr` at 0x412d0 | load | 4 | `abs 0x4153c` | `-` |
| `str` at 0x412d4 | store | 4 | `sp -0x38` | `0x108b08` |
| `ldr` at 0x412d8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x412dc | store | 4 | `sp -0x34` | `[sp-44]` |
| `ldr` at 0x412e4 | load | 4 | `abs 0x41538` | `-` |
| `ldr` at 0x412f4 | load | 4 | `abs 0x4152c` | `-` |
| `ldr` at 0x412f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4130c | load | 4 | `abs 0x147fdc` | `-` |
| `str` at 0x41314 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x41318 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4131c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x41320 | store | 4 | `[sp-40] +0x0` | `?` |
| `ldr` at 0x41324 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x41328 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4132c | store | 4 | `[sp-40] +0x8` | `[sp-32]` |
| `ldr` at 0x41330 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x41334 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x41338 | load | 4 | `[sp-40] +0x8` | `-` |
| `str` at 0x41348 | store | 4 | `[sp-40] +0x4` | `ret(0x1b3c4)` |
| `ldr` at 0x4134c | load | 4 | `[sp-40] +0x4` | `-` |
| `ldr` at 0x41358 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x41360 | load | 4 | `abs 0x41540` | `-` |
| `ldr` at 0x41364 | load | 4 | `abs 0x41544` | `-` |
| `str` at 0x41390 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x41394 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4139c | load | 4 | `abs 0x41540` | `-` |
| `ldr` at 0x413a0 | load | 4 | `abs 0x41544` | `-` |
| `ldr` at 0x413ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x413bc | load | 4 | `abs 0x41548` | `-` |
| `str` at 0x413c0 | store | 4 | `(unknown)` | `0x108b4c` |
| `str` at 0x413c4 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0x413c8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x413cc | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0x413d4 | load | 4 | `abs 0x41538` | `-` |
| `strb` at 0x413e8 | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0x413ec | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x413f0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x413f4 | load | 4 | `[sp-40] +0x8` | `-` |
| `ldr` at 0x41404 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x41408 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x4141c | load | 4 | `[sp-40] +0x0` | `-` |
| `ldrb` at 0x41424 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x41428 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x41440 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x41444 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x41458 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldrb` at 0x41460 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x4146c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x41470 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x41484 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldrb` at 0x4148c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x41490 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x414ac | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x414b0 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x414b8 | load | 4 | `[sp-40] +0x4` | `-` |
| `strb` at 0x414c4 | store | 1 | `(unknown)` | `0x2` |
| `ldr` at 0x414cc | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x414d0 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x414d8 | load | 4 | `[sp-40] +0x4` | `-` |
| `strb` at 0x414e4 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x414e8 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x414ec | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x414f4 | load | 4 | `[sp-40] +0x4` | `-` |
| `strb` at 0x41500 | store | 1 | `(unknown)` | `0x0` |
| `ldrb` at 0x41504 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x4150c | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldr` at 0x41514 | load | 4 | `abs 0x4154c` | `-` |
| `ldr` at 0x41518 | load | 4 | `abs 0x41550` | `-` |
| `ldmdb` at 0x41528 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x41528 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x41528 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x41528 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x41270 | `(unknown callee) 0x21e28` | `0x9` | `0x108af4` | `0x6c` | `0x0` |
| `bl` at 0x412b8 | `(unknown callee) 0x1b3c4` | `[sp-44]` | `0x1` | `[0x147fe0]` | `?` |
| `bl` at 0x412f0 | `(unknown callee) 0x21e28` | `0x9` | `0x108af4` | `0x78` | `0x0` |
| `bl` at 0x41340 | `(unknown callee) 0x1b3c4` | `[[sp-40]+8]` | `0x4` | `[sp-40]` | `[sp-40]` |
| `bl` at 0x41368 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x108b40` | `0x108b48` |
| `bl` at 0x41374 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x413a4 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x108b40` | `0x108b48` |
| `bl` at 0x413b4 | `(unknown callee) 0x1b508` | `[sp-44]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x413e0 | `(unknown callee) 0x21e28` | `0x9` | `0x108af4` | `0x84` | `0x0` |
| `bl` at 0x41430 | `(unknown callee) 0xd6928` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x41498 | `(unknown callee) 0xd6928` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x41524 | `(unknown callee) 0x22920` | `0x147fe4` | `0x108b8c` | `0x1` | `0x9` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x41230 | `[0x147fdc] == 0x0` | `[0x147fdc] != 0x0` |
| `bne` at 0x4123c | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0x41280 | `[0x147fe0] != 0x0` | `[0x147fe0] == 0x0` |
| `bne` at 0x412cc | `[0x147fdc] != 0x0` | `[0x147fdc] == 0x0` |
| `bne` at 0x41354 | `[[sp-40]+4] != 0x0` | `[[sp-40]+4] == 0x0` |
| `blt` at 0x413fc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x4143c | `ret(0xd6928) == 0x0` | `ret(0xd6928) != 0x0` |
| `beq` at 0x41468 | `? == 0xff` | `? != 0xff` |
| `bne` at 0x414a4 | `ret(0xd6928) != 0x0` | `ret(0xd6928) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x41528 | `r0` | `ret(0x22920)` |

## `portEnable`

- `portEnable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x41554 to 0x41688.
  Established. The signed guard takes its branch for `(r1 & 0xff) < [[sp-28]+8]` and falls through
  for greater or equal (`blt` at 0x415a4). The later byte store writes `0x2` at an unknown effective
  target (`strb` at 0x4166c); its prior test has unknown operand, takes the bypass for zero and
  falls through for nonzero (`beq` at 0x41650). Enable encoding and the joined r0 value remain
  unidentified (`ldmdb` at 0x41670).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x41564 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x4156c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x41570 | load | 4 | `abs 0x41674` | `-` |
| `ldr` at 0x41574 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41588 | load | 4 | `abs 0x147fdc` | `-` |
| `str` at 0x41590 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x41594 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x41598 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4159c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x415a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x415b0 | load | 4 | `abs 0x41678` | `-` |
| `ldr` at 0x415b4 | load | 4 | `abs 0x4167c` | `-` |
| `str` at 0x415e0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x415e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x415ec | load | 4 | `abs 0x41678` | `-` |
| `ldr` at 0x415f0 | load | 4 | `abs 0x4167c` | `-` |
| `ldr` at 0x415fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4160c | load | 4 | `abs 0x41680` | `-` |
| `str` at 0x41610 | store | 4 | `(unknown)` | `0x108b98` |
| `str` at 0x41614 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldrb` at 0x41618 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x4161c | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x41624 | load | 4 | `abs 0x41684` | `-` |
| `ldr` at 0x41634 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x41638 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x41640 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x41648 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x41654 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x41658 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x41660 | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0x4166c | store | 1 | `(unknown)` | `0x2` |
| `ldmdb` at 0x41670 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x41670 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x41670 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x415b8 | `(unknown callee) 0x4f564` | `r0` | `0x0` | `0x108b40` | `0x108b48` |
| `bl` at 0x415c4 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x415f4 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x108b40` | `0x108b48` |
| `bl` at 0x41604 | `(unknown callee) 0x1b508` | `[sp-32]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x41630 | `(unknown callee) 0x21e28` | `0x9` | `0x108af4` | `0xa1` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x415a4 | `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` |
| `beq` at 0x41650 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x41670 | `r0` | `?` |

## `portDisable`

- `portDisable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x41688 to 0x417bc.
  Established. A byte store at an unknown address writes `0x1` (`strb` at 0x417a0). The immediately
  prior zero test takes the bypass and nonzero falls through (`beq` at 0x41784). The opening
  call to `0x4f564` has entry r0, `0x0`, `0x108b40`, `0x108b48` (`bl` at 0x416ec). Disable semantics
  and exit r0 remain unproved (`ldmdb` at 0x417a4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x41698 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x416a0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x416a4 | load | 4 | `abs 0x417a8` | `-` |
| `ldr` at 0x416a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x416bc | load | 4 | `abs 0x147fdc` | `-` |
| `str` at 0x416c4 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x416c8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x416cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x416d0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x416dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x416e4 | load | 4 | `abs 0x417ac` | `-` |
| `ldr` at 0x416e8 | load | 4 | `abs 0x417b0` | `-` |
| `str` at 0x41714 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x41718 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41720 | load | 4 | `abs 0x417ac` | `-` |
| `ldr` at 0x41724 | load | 4 | `abs 0x417b0` | `-` |
| `ldr` at 0x41730 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x41740 | load | 4 | `abs 0x417b4` | `-` |
| `str` at 0x41744 | store | 4 | `(unknown)` | `0x108bbc` |
| `str` at 0x41748 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldrb` at 0x4174c | load | 1 | `sp -0x15` | `-` |
| `str` at 0x41750 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x41758 | load | 4 | `abs 0x417b8` | `-` |
| `ldr` at 0x41768 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4176c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x41774 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x4177c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x41788 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4178c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x41794 | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0x417a0 | store | 1 | `(unknown)` | `0x1` |
| `ldmdb` at 0x417a4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x417a4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x417a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x416ec | `(unknown callee) 0x4f564` | `r0` | `0x0` | `0x108b40` | `0x108b48` |
| `bl` at 0x416f8 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x41728 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x108b40` | `0x108b48` |
| `bl` at 0x41738 | `(unknown callee) 0x1b508` | `[sp-32]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x41764 | `(unknown callee) 0x21e28` | `0x9` | `0x108af4` | `0xad` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x416d8 | `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` |
| `beq` at 0x41784 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x417a4 | `r0` | `?` |

## `portIsEnabled`

- `portIsEnabled`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x417bc to 0x418dc.
  Established. The signed comparison takes its continuation for `(r1 & 0xff) < [[sp-28]+8]` and
  falls through for greater or equal (`blt` at 0x4180c). The first call to `0x4f564` carries entry
  r0, `0x0`, `0x108b40`, `0x108b48` (`bl` at 0x41820). No non-stack byte store is reported. The
  state-byte interpretation and r0 at exit are unknown (`ldmdb` at 0x418c4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x417cc | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x417d4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x417d8 | load | 4 | `abs 0x418c8` | `-` |
| `ldr` at 0x417dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x417f0 | load | 4 | `abs 0x147fdc` | `-` |
| `str` at 0x417f8 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x417fc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x41800 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x41804 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x41810 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41818 | load | 4 | `abs 0x418cc` | `-` |
| `ldr` at 0x4181c | load | 4 | `abs 0x418d0` | `-` |
| `str` at 0x41848 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x4184c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41854 | load | 4 | `abs 0x418cc` | `-` |
| `ldr` at 0x41858 | load | 4 | `abs 0x418d0` | `-` |
| `ldr` at 0x41864 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x41874 | load | 4 | `abs 0x418d4` | `-` |
| `str` at 0x41878 | store | 4 | `(unknown)` | `0x108be0` |
| `str` at 0x4187c | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldrb` at 0x41880 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x41884 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x4188c | load | 4 | `abs 0x418d8` | `-` |
| `ldr` at 0x4189c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x418a0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x418a8 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x418b0 | load | 1 | `(unknown)` | `-` |
| `ldmdb` at 0x418c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x418c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x418c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x41820 | `(unknown callee) 0x4f564` | `r0` | `0x0` | `0x108b40` | `0x108b48` |
| `bl` at 0x4182c | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x4185c | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x108b40` | `0x108b48` |
| `bl` at 0x4186c | `(unknown callee) 0x1b508` | `[sp-32]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x41898 | `(unknown callee) 0x21e28` | `0x9` | `0x108af4` | `0xb9` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x4180c | `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x418c4 | `r0` | `?` |

## `portPresent`

- `portPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x418dc to 0x419f0.
  Established. The byte store writes `0x2` at an unknown effective address (`strb` at 0x419d4). The
  entry comparison takes its continuation for signed `(r1 & 0xff) < [[sp-28]+8]` and falls through
  for greater or equal (`blt` at 0x4192c). The call to `0x4f564` has entry r0, `0x0`, `0x108b40`,
  `0x108b48` at its first site (`bl` at 0x41940). Presence encoding and final r0 are unresolved
  (`ldmdb` at 0x419d8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x418ec | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x418f4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x418f8 | load | 4 | `abs 0x419dc` | `-` |
| `ldr` at 0x418fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41910 | load | 4 | `abs 0x147fdc` | `-` |
| `str` at 0x41918 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x4191c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x41920 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x41924 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x41930 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41938 | load | 4 | `abs 0x419e0` | `-` |
| `ldr` at 0x4193c | load | 4 | `abs 0x419e4` | `-` |
| `str` at 0x41968 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x4196c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41974 | load | 4 | `abs 0x419e0` | `-` |
| `ldr` at 0x41978 | load | 4 | `abs 0x419e4` | `-` |
| `ldr` at 0x41984 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x41994 | load | 4 | `abs 0x419e8` | `-` |
| `str` at 0x41998 | store | 4 | `(unknown)` | `0x108c08` |
| `str` at 0x4199c | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldrb` at 0x419a0 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x419a4 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x419ac | load | 4 | `abs 0x419ec` | `-` |
| `ldr` at 0x419bc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x419c0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x419c8 | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0x419d4 | store | 1 | `(unknown)` | `0x2` |
| `ldmdb` at 0x419d8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x419d8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x419d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x41940 | `(unknown callee) 0x4f564` | `r0` | `0x0` | `0x108b40` | `0x108b48` |
| `bl` at 0x4194c | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x4197c | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x108b40` | `0x108b48` |
| `bl` at 0x4198c | `(unknown callee) 0x1b508` | `[sp-32]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x419b8 | `(unknown callee) 0x21e28` | `0x9` | `0x108af4` | `0xc8` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x4192c | `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x419d8 | `r0` | `?` |

## `portNotPresent`

- `portNotPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x419f0 to 0x41b04.
  Established. The final byte store clears an unknown effective target (`strb` at 0x41ae8). The
  signed comparison takes its branch for `(r1 & 0xff) < [[sp-28]+8]` and falls through for greater
  or equal (`blt` at 0x41a40). The first call to `0x4f564` uses entry r0, `0x0`, `0x108b40`,
  `0x108b48` (`bl` at 0x41a54). Absence handling and the return register value remain unknown
  (`ldmdb` at 0x41aec).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x41a00 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x41a08 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x41a0c | load | 4 | `abs 0x41af0` | `-` |
| `ldr` at 0x41a10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41a24 | load | 4 | `abs 0x147fdc` | `-` |
| `str` at 0x41a2c | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x41a30 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x41a34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x41a38 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x41a44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41a4c | load | 4 | `abs 0x41af4` | `-` |
| `ldr` at 0x41a50 | load | 4 | `abs 0x41af8` | `-` |
| `str` at 0x41a7c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x41a80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41a88 | load | 4 | `abs 0x41af4` | `-` |
| `ldr` at 0x41a8c | load | 4 | `abs 0x41af8` | `-` |
| `ldr` at 0x41a98 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x41aa8 | load | 4 | `abs 0x41afc` | `-` |
| `str` at 0x41aac | store | 4 | `(unknown)` | `0x108c2c` |
| `str` at 0x41ab0 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldrb` at 0x41ab4 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x41ab8 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x41ac0 | load | 4 | `abs 0x41b00` | `-` |
| `ldr` at 0x41ad0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x41ad4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x41adc | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0x41ae8 | store | 1 | `(unknown)` | `0x0` |
| `ldmdb` at 0x41aec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x41aec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x41aec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x41a54 | `(unknown callee) 0x4f564` | `r0` | `0x0` | `0x108b40` | `0x108b48` |
| `bl` at 0x41a60 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x41a90 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x108b40` | `0x108b48` |
| `bl` at 0x41aa0 | `(unknown callee) 0x1b508` | `[sp-32]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x41acc | `(unknown callee) 0x21e28` | `0x9` | `0x108af4` | `0xd3` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x41a40 | `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x41aec | `r0` | `?` |

## `portIsPresent`

- `portIsPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x41b04 to 0x41c24.
  Established. The first call to `0x4f564` carries entry r0, `0x0`, `0x108b40`, `0x108b48` (`bl` at
  0x41b68); its prior signed guard takes its branch for `(r1 & 0xff) < [[sp-28]+8]` and falls
  through for greater or equal (`blt` at 0x41b54). The recognized exit has unresolved r0 (`ldmdb` at
  0x41c0c). The physical presence test is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x41b14 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x41b1c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x41b20 | load | 4 | `abs 0x41c10` | `-` |
| `ldr` at 0x41b24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41b38 | load | 4 | `abs 0x147fdc` | `-` |
| `str` at 0x41b40 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x41b44 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x41b48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x41b4c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x41b58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41b60 | load | 4 | `abs 0x41c14` | `-` |
| `ldr` at 0x41b64 | load | 4 | `abs 0x41c18` | `-` |
| `str` at 0x41b90 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x41b94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41b9c | load | 4 | `abs 0x41c14` | `-` |
| `ldr` at 0x41ba0 | load | 4 | `abs 0x41c18` | `-` |
| `ldr` at 0x41bac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x41bbc | load | 4 | `abs 0x41c1c` | `-` |
| `str` at 0x41bc0 | store | 4 | `(unknown)` | `0x108c54` |
| `str` at 0x41bc4 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldrb` at 0x41bc8 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x41bcc | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x41bd4 | load | 4 | `abs 0x41c20` | `-` |
| `ldr` at 0x41be4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x41be8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x41bf0 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x41bf8 | load | 1 | `(unknown)` | `-` |
| `ldmdb` at 0x41c0c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x41c0c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x41c0c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x41b68 | `(unknown callee) 0x4f564` | `r0` | `0x0` | `0x108b40` | `0x108b48` |
| `bl` at 0x41b74 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x41ba4 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x108b40` | `0x108b48` |
| `bl` at 0x41bb4 | `(unknown callee) 0x1b508` | `[sp-32]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x41be0 | `(unknown callee) 0x21e28` | `0x9` | `0x108af4` | `0xde` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x41b54 | `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x41c0c | `r0` | `?` |

## `portGetCfg`

- `portGetCfg`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x41e20 to 0x41e80.
  Established. The word store at entry `r1 +0x0` carries `[[sp-28]]` (`str` at 0x41e68). The exit r0
  expression is `[[sp-28]+8]` (`ldmdb` at 0x41e78). No calls or conditional branches appear in the
  facts. Configuration fields and record count meaning remain unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x41e30 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x41e34 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x41e38 | load | 4 | `abs 0x41e7c` | `-` |
| `ldr` at 0x41e3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41e50 | load | 4 | `abs 0x147fdc` | `-` |
| `str` at 0x41e58 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x41e5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x41e60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x41e64 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x41e68 | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0x41e6c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x41e70 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldmdb` at 0x41e78 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x41e78 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x41e78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x41e78 | `r0` | `[[sp-28]+8]` |

## `portUpInternal`

- `portUpInternal`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x41e80 to 0x42178.
  Established. The byte comparison takes its branch for `(r1 & 0xff) != 0xff` and falls through on
  equality (`bne` at 0x41ed8). The call to `0x43c44` has entry r0, `(r1 & 0xff)`, unknown, `(r1 &
  0xff)` (`bl` at 0x41f00); its nonzero result takes the continuation and zero falls through (`bne`
  at 0x41f0c). Port transition semantics and merged r0 remain unknown (`ldmdb` at 0x42168).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x41e90 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x41e98 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x41ea0 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x41ea8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x41eac | load | 4 | `abs 0x4216c` | `-` |
| `ldr` at 0x41eb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41ec4 | load | 4 | `abs 0x147fdc` | `-` |
| `str` at 0x41ecc | store | 4 | `sp -0x34` | `?` |
| `ldrb` at 0x41ed0 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x41ee0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x41ee4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x41ee8 | load | 4 | `[sp-52] +0x8` | `-` |
| `str` at 0x41eec | store | 4 | `sp -0x30` | `[[sp-52]+8]` |
| `ldrb` at 0x41ef4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x41ef8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41f10 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x41f1c | store | 4 | `sp -0x38` | `ret(0x43cd8)` |
| `ldr` at 0x41f24 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x41f28 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x41f30 | load | 4 | `[sp-52] +0x4` | `-` |
| `ldrb` at 0x41f38 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x41f44 | load | 4 | `abs 0x42170` | `-` |
| `str` at 0x41f48 | store | 4 | `sp -0x38` | `0xfffffb4e` |
| `ldr` at 0x41f50 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x41f54 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x41f5c | load | 4 | `[sp-52] +0x4` | `-` |
| `ldrb` at 0x41f64 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x41f70 | load | 4 | `abs 0x42174` | `-` |
| `str` at 0x41f74 | store | 4 | `sp -0x38` | `0xfffffb4d` |
| `ldrb` at 0x41f7c | load | 1 | `sp -0x15` | `-` |
| `str` at 0x41f80 | store | 4 | `sp -0x2c` | `[sp-21]:1` |
| `ldrb` at 0x41f84 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x41f8c | store | 4 | `sp -0x30` | `[sp-21]:1+1` |
| `ldr` at 0x41f90 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x41f94 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x41f98 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x41f9c | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x41fac | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x41fb8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x41fbc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x41fd0 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldrh` at 0x41fd8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x41fe8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x41fec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x42000 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x42008 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x42018 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x42024 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x42028 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4203c | load | 4 | `[sp-52] +0x0` | `-` |
| `ldrh` at 0x42044 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x42054 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x42058 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4206c | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x42074 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x42084 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x42088 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x42090 | load | 4 | `[sp-52] +0x4` | `-` |
| `ldrb` at 0x42098 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x420a4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x420a8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x420bc | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x420c4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x420d4 | store | 4 | `sp -0x24` | `ret(0x43f20)` |
| `ldr` at 0x420d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x420e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x420f0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x420f4 | store | 4 | `sp -0x20` | `ret(0x43f20)` |
| `ldr` at 0x420f8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x420fc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x42110 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x42118 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x42128 | store | 4 | `sp -0x24` | `ret(0x43254)` |
| `ldr` at 0x4212c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42138 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x42144 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x42148 | store | 4 | `sp -0x20` | `ret(0x43254)` |
| `ldr` at 0x4214c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x42154 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x4215c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x42160 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x42164 | load | 4 | `sp -0x38` | `-` |
| `ldmdb` at 0x42168 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x42168 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x42168 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x41f00 | `(unknown callee) 0x43c44` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0x41f14 | `(unknown callee) 0x43cd8` | `[sp-20]` | `?` | `?` | `ret(0x43c44)` |
| `bl` at 0x420cc | `(unknown callee) 0x43f20` | `?` | `?` | `?` | `?` |
| `bl` at 0x42120 | `(unknown callee) 0x43254` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x41ed8 | `(r1 & 0xff) != 0xff` | `(r1 & 0xff) == 0xff` |
| `bne` at 0x41f0c | `ret(0x43c44) != 0x0` | `ret(0x43c44) == 0x0` |
| `bne` at 0x41f40 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x41f6c | `? > 0x1 (unsigned, 32-bit)` | `? <= 0x1 (unsigned, 32-bit)` |
| `blt` at 0x41fa4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x41fb4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x41fe4 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x42010 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x42020 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x42050 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x4207c | `Z == 0` | `not (Z == 0)` |
| `bls` at 0x420a0 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x420e0 | `ret(0x43f20) == 0x0` | `ret(0x43f20) != 0x0` |
| `bne` at 0x420ec | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x42134 | `ret(0x43254) == 0x0` | `ret(0x43254) != 0x0` |
| `bne` at 0x42140 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x42168 | `r0` | `?` |

## `portDownInternal`

- `portDownInternal`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x42178 to 0x4241c.
  Established. The first call to `0x43c44` uses entry r0, `(r1 & 0xff)`, unknown, `(r1 & 0xff)`
  (`bl` at 0x421f8). That result takes the continuation for nonzero and falls through for zero
  (`bne` at 0x42204). The earlier byte guard takes its branch for `(r1 & 0xff) != 0xff` and falls
  through on equality (`bne` at 0x421d0). Physical link shutdown and final r0 remain unresolved
  (`ldmdb` at 0x4240c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x42188 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x42190 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x42198 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x421a0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x421a4 | load | 4 | `abs 0x42410` | `-` |
| `ldr` at 0x421a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x421bc | load | 4 | `abs 0x147fdc` | `-` |
| `str` at 0x421c4 | store | 4 | `sp -0x34` | `?` |
| `ldrb` at 0x421c8 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x421d8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x421dc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x421e0 | load | 4 | `[sp-52] +0x8` | `-` |
| `str` at 0x421e4 | store | 4 | `sp -0x30` | `[[sp-52]+8]` |
| `ldrb` at 0x421ec | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x421f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x42208 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x42214 | store | 4 | `sp -0x38` | `ret(0x43cd8)` |
| `ldr` at 0x4221c | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x42220 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x42228 | load | 4 | `[sp-52] +0x4` | `-` |
| `ldrb` at 0x42230 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x4223c | load | 4 | `abs 0x42414` | `-` |
| `str` at 0x42240 | store | 4 | `sp -0x38` | `0xfffffb4e` |
| `ldr` at 0x42248 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x4224c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x42254 | load | 4 | `[sp-52] +0x4` | `-` |
| `ldrb` at 0x4225c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x42268 | load | 4 | `abs 0x42418` | `-` |
| `str` at 0x4226c | store | 4 | `sp -0x38` | `0xfffffb4d` |
| `ldrb` at 0x42274 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x42278 | store | 4 | `sp -0x2c` | `[sp-21]:1` |
| `ldrb` at 0x4227c | load | 1 | `sp -0x15` | `-` |
| `str` at 0x42284 | store | 4 | `sp -0x30` | `[sp-21]:1+1` |
| `ldr` at 0x42288 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x4228c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x42290 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x42294 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x422a4 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x422b0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x422b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x422c8 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldrh` at 0x422d0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x422e0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x422e4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x422f8 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x42300 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x42310 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x4231c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x42320 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x42334 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldrh` at 0x4233c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x4234c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x42350 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x42364 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x4236c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4237c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x42380 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x42388 | load | 4 | `[sp-52] +0x4` | `-` |
| `ldrb` at 0x42390 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x4239c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x423a0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x423b4 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x423bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x423cc | store | 4 | `sp -0x24` | `ret(0x441ec)` |
| `ldr` at 0x423d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x423dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x423e8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x423ec | store | 4 | `sp -0x20` | `ret(0x441ec)` |
| `ldr` at 0x423f0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x423f8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x42400 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x42404 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x42408 | load | 4 | `sp -0x38` | `-` |
| `ldmdb` at 0x4240c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4240c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4240c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x421f8 | `(unknown callee) 0x43c44` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0x4220c | `(unknown callee) 0x43cd8` | `[sp-20]` | `?` | `?` | `ret(0x43c44)` |
| `bl` at 0x423c4 | `(unknown callee) 0x441ec` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x421d0 | `(r1 & 0xff) != 0xff` | `(r1 & 0xff) == 0xff` |
| `bne` at 0x42204 | `ret(0x43c44) != 0x0` | `ret(0x43c44) == 0x0` |
| `bne` at 0x42238 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x42264 | `? > 0x1 (unsigned, 32-bit)` | `? <= 0x1 (unsigned, 32-bit)` |
| `blt` at 0x4229c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x422ac | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x422dc | `? != 0x0` | `? == 0x0` |
| `beq` at 0x42308 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x42318 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x42348 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x42374 | `Z == 0` | `not (Z == 0)` |
| `bls` at 0x42398 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x423d8 | `ret(0x441ec) == 0x0` | `ret(0x441ec) != 0x0` |
| `bne` at 0x423e4 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4240c | `r0` | `?` |

## `portStatus`

- `portStatus`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4241c to 0x428c4.
  Established. A byte store writes `([sp-61]:1 & 0xff)` at `[sp-32] +0x0` (`strb` at 0x42534). Two
  later byte stores at that expression have unknown values (`strb` at 0x4255c, `strb` at 0x42570).
  The initial call to `0x43c44` carries entry r0, `(r1 & 0xff)`, entry r2, `(r1 & 0xff)` (`bl` at
  0x42450); its nonzero result takes the continuation and zero falls through (`bne` at 0x42464).
  Status-bit layout and r0 at exit remain unknown (`ldmdb` at 0x428b0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4242c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x42434 | store | 4 | `sp -0x20` | `r2` |
| `strb` at 0x42438 | store | 1 | `sp -0x19` | `(r1 & 0xff)` |
| `strb` at 0x42440 | store | 1 | `sp -0x29` | `0x0` |
| `ldrb` at 0x42444 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x42448 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x42458 | store | 4 | `sp -0x24` | `ret(0x43c44)` |
| `ldr` at 0x4245c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42468 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x42474 | store | 4 | `sp -0x38` | `ret(0x43cd8)` |
| `ldrb` at 0x4247c | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x42480 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x42498 | load | 4 | `abs 0x428bc` | `-` |
| `str` at 0x4249c | store | 4 | `sp -0x38` | `0xfffffb4e` |
| `ldr` at 0x424a4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x424a8 | load | 1 | `[sp-36] +0x0` | `-` |
| `ldr` at 0x424ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x424b4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x424c0 | store | 4 | `sp -0x28` | `ret(0xd6b9c)` |
| `ldr` at 0x424c4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x424d0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x424d4 | store | 4 | `sp -0x38` | `ret(0xd6b9c)` |
| `ldr` at 0x424dc | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x424e0 | load | 2 | `[sp-36] +0x4` | `-` |
| `ldr` at 0x424f0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x424f4 | store | 4 | `sp -0x3c` | `[sp-32]` |
| `ldr` at 0x424f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x424fc | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x42500 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldrb` at 0x42504 | load | 1 | `[sp-36] +0x1` | `-` |
| `strb` at 0x4251c | store | 1 | `sp -0x3d` | `0x4` |
| `strb` at 0x42528 | store | 1 | `sp -0x3d` | `0x0` |
| `ldrb` at 0x4252c | load | 1 | `sp -0x3d` | `-` |
| `ldr` at 0x42530 | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0x42534 | store | 1 | `[sp-32] +0x0` | `([sp-61]:1 & 0xff)` |
| `ldr` at 0x4253c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x42540 | load | 1 | `[sp-36] +0x1` | `-` |
| `ldr` at 0x4254c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x42550 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x42554 | load | 1 | `[sp-32] +0x0` | `-` |
| `strb` at 0x4255c | store | 1 | `[sp-32] +0x0` | `?` |
| `ldr` at 0x42560 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x42564 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x42568 | load | 1 | `[sp-32] +0x0` | `-` |
| `strb` at 0x42570 | store | 1 | `[sp-32] +0x0` | `?` |
| `ldr` at 0x42574 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x42578 | load | 2 | `[sp-36] +0x4` | `-` |
| `ldr` at 0x42588 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x4258c | load | 1 | `[sp-32] +0x0` | `-` |
| `ldrb` at 0x42598 | load | 1 | `sp -0x29` | `-` |
| `strb` at 0x425a0 | store | 1 | `sp -0x29` | `?` |
| `ldr` at 0x425a4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x425a8 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldrb` at 0x425b4 | load | 1 | `sp -0x29` | `-` |
| `strb` at 0x425bc | store | 1 | `sp -0x29` | `?` |
| `ldr` at 0x425c0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x425c4 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldrb` at 0x425d0 | load | 1 | `sp -0x29` | `-` |
| `strb` at 0x425d8 | store | 1 | `sp -0x29` | `?` |
| `ldr` at 0x425dc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x425e0 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldrb` at 0x425ec | load | 1 | `sp -0x29` | `-` |
| `strb` at 0x425f4 | store | 1 | `sp -0x29` | `?` |
| `ldr` at 0x425f8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x425fc | load | 1 | `sp -0x29` | `-` |
| `strb` at 0x42600 | store | 1 | `[sp-32] +0x0` | `([sp-41]:1 & 0xff)` |
| `ldr` at 0x42604 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x42608 | load | 2 | `[sp-36] +0x4` | `-` |
| `ldr` at 0x42618 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4261c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x42620 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x42630 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0x42634 | store | 4 | `sp -0x44` | `[[sp-32]]` |
| `ldr` at 0x4263c | load | 4 | `sp -0x44` | `-` |
| `str` at 0x4264c | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x42650 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x42654 | store | 4 | `[sp-32] +0x0` | `[sp-68]` |
| `ldr` at 0x42658 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x4265c | load | 2 | `[sp-36] +0x4` | `-` |
| `ldr` at 0x4266c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x42670 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x42674 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x42684 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0x42688 | store | 4 | `sp -0x48` | `[[sp-32]]` |
| `ldr` at 0x42690 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x426a0 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x426a4 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x426a8 | store | 4 | `[sp-32] +0x0` | `[sp-72]` |
| `ldr` at 0x426ac | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x426b0 | load | 1 | `[sp-36] +0x2` | `-` |
| `ldr` at 0x426b8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x426c8 | store | 4 | `sp -0x28` | `ret(0xd6b9c)` |
| `ldr` at 0x426cc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x426d8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x426dc | store | 4 | `sp -0x38` | `ret(0xd6b9c)` |
| `ldr` at 0x426e4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x426e8 | load | 1 | `sp -0x29` | `-` |
| `ldrb` at 0x426ec | load | 1 | `[sp-36] +0x3` | `-` |
| `strb` at 0x426f4 | store | 1 | `sp -0x29` | `?` |
| `ldr` at 0x426f8 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x426fc | load | 2 | `[sp-36] +0x4` | `-` |
| `ldr` at 0x4270c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x42710 | load | 1 | `sp -0x29` | `-` |
| `ldr` at 0x42720 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0x42724 | store | 4 | `sp -0x4c` | `[[sp-32]]` |
| `ldr` at 0x4272c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x4273c | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x42740 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x42744 | store | 4 | `[sp-32] +0x0` | `[sp-76]` |
| `ldr` at 0x4274c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x42750 | load | 1 | `sp -0x29` | `-` |
| `ldr` at 0x42760 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0x42764 | store | 4 | `sp -0x50` | `[[sp-32]]` |
| `ldr` at 0x4276c | load | 4 | `sp -0x50` | `-` |
| `str` at 0x4277c | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x42780 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x42784 | store | 4 | `[sp-32] +0x0` | `[sp-80]` |
| `ldr` at 0x42788 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x4278c | load | 2 | `[sp-36] +0x4` | `-` |
| `ldr` at 0x4279c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x427a0 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x427b4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x427b8 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x427cc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x427d0 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x427f0 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x427f4 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x42800 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x4280c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x4281c | load | 4 | `sp -0x54` | `-` |
| `str` at 0x42830 | store | 4 | `sp -0x34` | `0x2` |
| `str` at 0x4283c | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x42844 | load | 4 | `abs 0x428c0` | `-` |
| `ldr` at 0x42854 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x42860 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x4287c | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x4287c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x42894 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x42898 | load | 1 | `[sp-32] +0x0` | `-` |
| `strb` at 0x428a0 | store | 1 | `[sp-32] +0x0` | `?` |
| `str` at 0x428a8 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x428ac | load | 4 | `sp -0x38` | `-` |
| `ldmdb` at 0x428b0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x428b0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x428b0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x428b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x42450 | `(unknown callee) 0x43c44` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x4246c | `(unknown callee) 0x43cd8` | `[sp-24]` | `?` | `?` | `ret(0x43c44)` |
| `bl` at 0x42488 | `(unknown callee) 0x41b04` | `[sp-24]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0x424b8 | `(unknown callee) 0xd6b9c` | `[sp-24]` | `[[sp-36]]:1` | `[sp-32]` | `[[sp-36]]:1` |
| `bl` at 0x426c0 | `(unknown callee) 0xd6b9c` | `[sp-24]` | `[[sp-36]+2]:1` | `sp-41` | `[[sp-36]+2]:1` |
| `bl` at 0x427e4 | `(unknown callee) 0x4f090` | `[[sp-32]]` | `[sp-32]` | `?` | `?` |
| `bl` at 0x42848 | `(unknown callee) 0x20240` | `0x108ca0` | `?` | `?` | `[sp-84]` |
| `bl` at 0x4285c | `(unknown callee) 0x3fcf8` | `0x0` | `?` | `0x0` | `sp-48` |
| `bl` at 0x42864 | `(unknown callee) 0xfaca8` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0x42880 | `(unknown callee) 0xfb1c0` | `ret(0xfaca8)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x42464 | `ret(0x43c44) != 0x0` | `ret(0x43c44) == 0x0` |
| `bne` at 0x42494 | `ret(0x41b04) != 0x0` | `ret(0x41b04) == 0x0` |
| `beq` at 0x424cc | `ret(0xd6b9c) == 0x0` | `ret(0xd6b9c) != 0x0` |
| `beq` at 0x424ec | `? == 0x0` | `? != 0x0` |
| `beq` at 0x42514 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x42548 | `[[sp-36]+1]:1 == 0x0` | `[[sp-36]+1]:1 != 0x0` |
| `beq` at 0x42584 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x42614 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x42668 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x426d4 | `ret(0xd6b9c) == 0x0` | `ret(0xd6b9c) != 0x0` |
| `beq` at 0x42708 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x42798 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x427b0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x427c8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x427e0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x427fc | `[sp-84] == 0x3d` | `[sp-84] != 0x3d` |
| `bgt` at 0x42808 | `[sp-84] > 0x3d (signed, 32-bit)` | `[sp-84] <= 0x3d (signed, 32-bit)` |
| `beq` at 0x42814 | `[sp-84] == 0x23` | `[sp-84] != 0x23` |
| `beq` at 0x42824 | `[sp-84] == 0x4f` | `[sp-84] != 0x4f` |
| `blt` at 0x4288c | `ret(0xfb1c0) < 0x0 (signed, 32-bit)` | `ret(0xfb1c0) >= 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x428b0 | `r0` | `?` |

## `portName`

- `portName`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x428c4 to 0x4293c.
  Established. The call to `0x43c44` receives entry r0, `(r1 & 0xff)`, entry r2, `(r1 & 0xff)` (`bl`
  at 0x428ec). A zero result takes the bypass and nonzero falls through (`beq` at 0x42900). The next
  test takes its bypass for `[ret(0x43c44)+8] == 0x0` and falls through for nonzero (`beq` at
  0x42910). Name-record ownership and the common r0 result remain unestablished (`ldmdb` at
  0x42934).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x428d4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x428dc | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x428e0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x428e4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x428f4 | store | 4 | `sp -0x1c` | `ret(0x43c44)` |
| `ldr` at 0x428f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x42904 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x42908 | load | 4 | `ret(0x43c44) +0x8` | `-` |
| `ldr` at 0x42914 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x42918 | load | 4 | `ret(0x43c44) +0x8` | `-` |
| `str` at 0x4291c | store | 4 | `sp -0x20` | `[ret(0x43c44)+8]` |
| `ldr` at 0x42924 | load | 4 | `abs 0x42938` | `-` |
| `str` at 0x42928 | store | 4 | `sp -0x20` | `0x108cbc` |
| `ldr` at 0x4292c | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x42934 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x42934 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x42934 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x428ec | `(unknown callee) 0x43c44` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x42900 | `ret(0x43c44) == 0x0` | `ret(0x43c44) != 0x0` |
| `beq` at 0x42910 | `[ret(0x43c44)+8] == 0x0` | `[ret(0x43c44)+8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x42934 | `r0` | `?` |

## `portState`

- `portState`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4293c to 0x42a24.
  Established. The signed guard takes its bypass for `(r1 & 0xff) >= [[sp-28]+8]` and falls through
  for less (`bge` at 0x429a8). The next unsigned comparison has unknown operand, takes its branch
  for greater than `0x5` and falls through for less or equal (`bhi` at 0x429c8). No calls occur in
  the facts. State-table entries and final r0 remain unidentified (`ldmdb` at 0x42a14).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4294c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x42954 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x42958 | load | 4 | `abs 0x42a18` | `-` |
| `ldr` at 0x4295c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x42970 | load | 4 | `abs 0x147fdc` | `-` |
| `str` at 0x42978 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x4297c | load | 4 | `abs 0x42a1c` | `-` |
| `ldm` at 0x42988 | load | 4 | `abs 0x108d08` | `-` |
| `ldm` at 0x42988 | load | 4 | `abs 0x108d0c` | `-` |
| `ldm` at 0x42988 | load | 4 | `abs 0x108d10` | `-` |
| `ldm` at 0x42988 | load | 4 | `abs 0x108d14` | `-` |
| `stm` at 0x4298c | store | 4 | `sp -0x34` | `[0x108d08]` |
| `stm` at 0x4298c | store | 4 | `sp -0x30` | `[0x108d0c]` |
| `stm` at 0x4298c | store | 4 | `sp -0x2c` | `[0x108d10]` |
| `stm` at 0x4298c | store | 4 | `sp -0x28` | `[0x108d14]` |
| `ldm` at 0x42990 | load | 4 | `abs 0x108d18` | `-` |
| `ldm` at 0x42990 | load | 4 | `abs 0x108d1c` | `-` |
| `stm` at 0x42994 | store | 4 | `sp -0x24` | `[0x108d18]` |
| `stm` at 0x42994 | store | 4 | `sp -0x20` | `[0x108d1c]` |
| `ldrb` at 0x42998 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x4299c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x429a0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x429ac | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x429b0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x429b8 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x429c0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x429d0 | load | 4 | `abs 0x42a20` | `-` |
| `str` at 0x429d4 | store | 4 | `sp -0x38` | `0x108cbc` |
| `ldr` at 0x429dc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x429e0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x429e8 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x429f0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x42a08 | load | 4 | `(unknown)` | `-` |
| `str` at 0x42a0c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x42a10 | load | 4 | `sp -0x38` | `-` |
| `ldmdb` at 0x42a14 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x42a14 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x42a14 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x429a8 | `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)` |
| `bhi` at 0x429c8 | `? > 0x5 (unsigned, 32-bit)` | `? <= 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x42a14 | `r0` | `?` |

## `portQuery`

- `portQuery`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x42a24 to 0x42dd4.
  Established. Two word stores write `ret(0x1b124)` at `[sp-28] +0x0` in separate arms (`str` at
  0x42a78, `str` at 0x42ab4). The initial call to `0x41e20` has entry r0, `sp-36`, entry r2, `sp-36`
  (`bl` at 0x42a50); its nonzero result takes the continuation and zero falls through (`bne` at
  0x42a64). Query-string interpretation and r0 at exit remain unknown (`ldmdb` at 0x42da0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x42a34 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x42a38 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x42a40 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x42a48 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x42a58 | store | 4 | `sp -0x30` | `ret(0x41e20)` |
| `ldr` at 0x42a5c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x42a68 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x42a78 | store | 4 | `[sp-28] +0x0` | `ret(0x1b124)` |
| `ldr` at 0x42a7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x42a80 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x42a84 | load | 4 | `abs 0x42da4` | `-` |
| `ldr` at 0x42a90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x42a94 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x42ab4 | store | 4 | `[sp-28] +0x0` | `ret(0x1b124)` |
| `ldr` at 0x42ab8 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x42abc | store | 4 | `sp -0x34` | `[[sp-28]]` |
| `ldr` at 0x42ac0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x42ac4 | load | 4 | `abs 0x42da8` | `-` |
| `ldr` at 0x42acc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x42adc | load | 4 | `sp -0x34` | `-` |
| `str` at 0x42ae4 | store | 4 | `sp -0x34` | `([sp-52] + ret(0x1b22c))` |
| `str` at 0x42aec | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x42af0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x42af4 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x42b04 | load | 1 | `sp -0x2c` | `-` |
| `ldr` at 0x42b0c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x42b1c | store | 4 | `sp -0x20` | `ret(0x4241c)` |
| `ldr` at 0x42b20 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x42b2c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x42b30 | load | 4 | `abs 0x42dac` | `-` |
| `ldr` at 0x42b3c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x42b50 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42b58 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x42b5c | load | 4 | `abs 0x42db0` | `-` |
| `ldr` at 0x42b60 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x42b6c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x42b78 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x42b8c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42b94 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x42b98 | load | 4 | `abs 0x42db4` | `-` |
| `ldr` at 0x42b9c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x42bac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x42bc0 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x42bc8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x42bd8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x42bec | load | 4 | `sp -0x24` | `-` |
| `str` at 0x42bf4 | store | 4 | `sp -0x38` | `?` |
| `ldrb` at 0x42bf8 | load | 1 | `sp -0x28` | `-` |
| `str` at 0x42bfc | store | 4 | `sp -0x3c` | `[sp-40]:1` |
| `ldrb` at 0x42c00 | load | 1 | `sp -0x28` | `-` |
| `ldr` at 0x42c14 | load | 4 | `abs 0x42db8` | `-` |
| `str` at 0x42c18 | store | 4 | `sp -0x40` | `0x108de4` |
| `ldr` at 0x42c20 | load | 4 | `abs 0x42dbc` | `-` |
| `str` at 0x42c24 | store | 4 | `sp -0x40` | `0x108de8` |
| `ldr` at 0x42c28 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x42c2c | store | 4 | `sp -0x6c` | `[sp-64]` |
| `ldr` at 0x42c30 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x42c34 | load | 4 | `abs 0x42dc0` | `-` |
| `ldr` at 0x42c38 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x42c3c | load | 4 | `[sp-56] +0x8` | `-` |
| `ldr` at 0x42c40 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x42c4c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x42c60 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x42c68 | store | 4 | `sp -0x44` | `?` |
| `ldrb` at 0x42c6c | load | 1 | `sp -0x28` | `-` |
| `str` at 0x42c70 | store | 4 | `sp -0x48` | `[sp-40]:1` |
| `ldrb` at 0x42c74 | load | 1 | `sp -0x28` | `-` |
| `ldr` at 0x42c88 | load | 4 | `abs 0x42dc4` | `-` |
| `str` at 0x42c8c | store | 4 | `sp -0x4c` | `0x108e1c` |
| `ldr` at 0x42c94 | load | 4 | `abs 0x42dc8` | `-` |
| `str` at 0x42c98 | store | 4 | `sp -0x4c` | `0x108e24` |
| `ldr` at 0x42c9c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x42ca0 | store | 4 | `sp -0x6c` | `[sp-76]` |
| `ldrb` at 0x42ca4 | load | 1 | `sp -0x28` | `-` |
| `ldr` at 0x42cb8 | load | 4 | `abs 0x42dc4` | `-` |
| `str` at 0x42cbc | store | 4 | `sp -0x50` | `0x108e1c` |
| `ldr` at 0x42cc4 | load | 4 | `abs 0x42dc8` | `-` |
| `str` at 0x42cc8 | store | 4 | `sp -0x50` | `0x108e24` |
| `ldr` at 0x42ccc | load | 4 | `sp -0x50` | `-` |
| `str` at 0x42cd0 | store | 4 | `sp -0x68` | `[sp-80]` |
| `ldrb` at 0x42cd4 | load | 1 | `sp -0x28` | `-` |
| `ldrb` at 0x42ce8 | load | 1 | `sp -0x28` | `-` |
| `ldr` at 0x42cfc | load | 4 | `abs 0x42dcc` | `-` |
| `str` at 0x42d00 | store | 4 | `sp -0x58` | `0x108e2c` |
| `ldr` at 0x42d08 | load | 4 | `abs 0x42dbc` | `-` |
| `str` at 0x42d0c | store | 4 | `sp -0x58` | `0x108de8` |
| `ldr` at 0x42d10 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x42d14 | store | 4 | `sp -0x54` | `[sp-88]` |
| `ldr` at 0x42d1c | load | 4 | `abs 0x42db8` | `-` |
| `str` at 0x42d20 | store | 4 | `sp -0x54` | `0x108de4` |
| `ldr` at 0x42d24 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x42d28 | store | 4 | `sp -0x64` | `?` |
| `ldrb` at 0x42d2c | load | 1 | `sp -0x28` | `-` |
| `ldr` at 0x42d40 | load | 4 | `abs 0x42db8` | `-` |
| `str` at 0x42d44 | store | 4 | `sp -0x5c` | `0x108de4` |
| `ldr` at 0x42d4c | load | 4 | `abs 0x42dbc` | `-` |
| `str` at 0x42d50 | store | 4 | `sp -0x5c` | `0x108de8` |
| `ldr` at 0x42d54 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x42d58 | store | 4 | `sp -0x60` | `[sp-92]` |
| `ldr` at 0x42d5c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x42d60 | load | 4 | `abs 0x42dd0` | `-` |
| `ldr` at 0x42d64 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x42d68 | load | 4 | `[sp-68] +0x8` | `-` |
| `ldr` at 0x42d6c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x42d74 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x42d84 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x42d8c | store | 4 | `sp -0x34` | `([sp-52] + ret(0x1b22c))` |
| `ldr` at 0x42d90 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x42d98 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldmdb` at 0x42da0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x42da0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x42da0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x42da0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x42a50 | `(unknown callee) 0x41e20` | `r0` | `sp-36` | `r2` | `sp-36` |
| `bl` at 0x42a70 | `(unknown callee) 0x1b124` | `0x50` | `?` | `?` | `ret(0x41e20)` |
| `bl` at 0x42a88 | `(unknown callee) 0x1b508` | `[[sp-28]]` | `0x108d20` | `?` | `[sp-28]` |
| `bl` at 0x42aac | `(unknown callee) 0x1b124` | `?` | `?` | `ret(0x41e20)` | `?` |
| `bl` at 0x42ac8 | `(unknown callee) 0x1b49c` | `[[sp-28]]` | `0x108d38` | `?` | `[[sp-28]]` |
| `bl` at 0x42ad0 | `(unknown callee) 0x1b22c` | `[sp-52]` | `?` | `?` | `?` |
| `bl` at 0x42b14 | `(unknown callee) 0x4241c` | `[sp-24]` | `[sp-44]:1` | `sp-40` | `[sp-44]:1` |
| `bl` at 0x42b64 | `(unknown callee) 0x1b49c` | `[sp-52]` | `0x108db0` | `?` | `?` |
| `bl` at 0x42b70 | `(unknown callee) 0x21674` | `ret(0x4241c)` | `?` | `ret(0x4241c)` | `0xfffffb4e` |
| `bl` at 0x42ba4 | `(unknown callee) 0x1b49c` | `[sp-52]` | `0x108dc4` | `?` | `ret(0x21674)` |
| `bl` at 0x42c44 | `(unknown callee) 0x1b49c` | `[sp-52]` | `0x108dec` | `[[sp-56]+8]` | `[sp-40]:1` |
| `bl` at 0x42d70 | `(unknown callee) 0x1b49c` | `[sp-52]` | `0x108e30` | `[[sp-68]+8]` | `[sp-40]:1` |
| `bl` at 0x42d78 | `(unknown callee) 0x1b22c` | `[sp-52]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x42a64 | `ret(0x41e20) != 0x0` | `ret(0x41e20) == 0x0` |
| `blt` at 0x42afc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x42b28 | `ret(0x4241c) == 0x0` | `ret(0x4241c) != 0x0` |
| `bne` at 0x42b38 | `ret(0x4241c) != 0xfffffb4e` | `ret(0x4241c) == 0xfffffb4e` |
| `beq` at 0x42bd4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x42c10 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x42c84 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x42cb4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x42ce4 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x42cf8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x42d3c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x42da0 | `r0` | `?` |

## `portInt`

- `portInt`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x42dd4 to 0x43254.
  Established. The opening call to `0x22a38` has `0x147fe4`, unknown, entry r2, entry r3 (`bl` at
  0x42e0c); its zero result takes the bypass and nonzero falls through (`beq` at 0x42e20). A later
  test takes its branch for `[sp-32] != 0x0` and falls through on zero (`bne` at 0x42e80).
  Interrupt-mask interpretation, channel ownership and the common r0 are unproved (`ldmdb` at
  0x4320c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x42de4 | store | 4 | `sp -0x18` | `r0` |
| `strb` at 0x42df0 | store | 1 | `sp -0x19` | `(r1 & 0xff)` |
| `strb` at 0x42df8 | store | 1 | `sp -0x1a` | `(r2 & 0xff)` |
| `strb` at 0x42e00 | store | 1 | `sp -0x1b` | `(r3 & 0xff)` |
| `ldr` at 0x42e04 | load | 4 | `abs 0x43210` | `-` |
| `str` at 0x42e14 | store | 4 | `sp -0x2c` | `ret(0x22a38)` |
| `ldr` at 0x42e18 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x42e24 | load | 4 | `abs 0x43214` | `-` |
| `str` at 0x42e28 | store | 4 | `sp -0x44` | `0x108e50` |
| `ldr` at 0x42e2c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x42e30 | store | 4 | `sp -0x40` | `ret(0x22a38)` |
| `ldr` at 0x42e38 | load | 4 | `abs 0x43218` | `-` |
| `ldr` at 0x42e3c | load | 4 | `abs 0x4321c` | `-` |
| `ldrb` at 0x42e48 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x42e4c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x42e5c | store | 4 | `sp -0x20` | `ret(0x43d48)` |
| `ldrb` at 0x42e60 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x42e64 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x42e74 | store | 4 | `sp -0x24` | `ret(0x43e34)` |
| `ldr` at 0x42e78 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x42e84 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42e90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x42e98 | load | 4 | `abs 0x43220` | `-` |
| `ldr` at 0x42e9c | load | 4 | `abs 0x43224` | `-` |
| `str` at 0x42ec8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x42ecc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x42ed4 | load | 4 | `abs 0x43220` | `-` |
| `ldr` at 0x42ed8 | load | 4 | `abs 0x43224` | `-` |
| `ldr` at 0x42ee4 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x42ef4 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x42ef8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x42f08 | load | 4 | `abs 0x43228` | `-` |
| `str` at 0x42f0c | store | 4 | `(unknown)` | `0x108e74` |
| `str` at 0x42f10 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `str` at 0x42f14 | store | 4 | `(unknown)` | `ret(0xd7558)` |
| `ldrb` at 0x42f18 | load | 1 | `sp -0x1a` | `-` |
| `str` at 0x42f1c | store | 4 | `(unknown)` | `[sp-26]:1` |
| `ldr` at 0x42f24 | load | 4 | `abs 0x43218` | `-` |
| `ldrb` at 0x42f34 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0x42f38 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0x42f3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x42f48 | load | 4 | `abs 0x4322c` | `-` |
| `ldr` at 0x42f4c | load | 4 | `abs 0x14850c` | `-` |
| `ldr` at 0x42f58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x42f60 | load | 4 | `abs 0x43230` | `-` |
| `ldr` at 0x42f64 | load | 4 | `abs 0x43234` | `-` |
| `str` at 0x42f90 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x42f94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x42f9c | load | 4 | `abs 0x43230` | `-` |
| `ldr` at 0x42fa0 | load | 4 | `abs 0x43234` | `-` |
| `ldr` at 0x42fac | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x42fbc | load | 4 | `abs 0x43238` | `-` |
| `str` at 0x42fc0 | store | 4 | `(unknown)` | `0x108ebc` |
| `str` at 0x42fc4 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldrb` at 0x42fc8 | load | 1 | `sp -0x19` | `-` |
| `str` at 0x42fcc | store | 4 | `(unknown)` | `[sp-25]:1` |
| `ldrb` at 0x42fd0 | load | 1 | `sp -0x1a` | `-` |
| `str` at 0x42fd4 | store | 4 | `(unknown)` | `[sp-26]:1` |
| `ldrb` at 0x42fd8 | load | 1 | `sp -0x1b` | `-` |
| `str` at 0x42fdc | store | 4 | `(unknown)` | `[sp-27]:1` |
| `ldr` at 0x42fe4 | load | 4 | `abs 0x43218` | `-` |
| `ldr` at 0x42fe8 | load | 4 | `abs 0x4323c` | `-` |
| `ldr` at 0x42ff4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x43000 | load | 1 | `sp -0x1a` | `-` |
| `ldrb` at 0x43008 | load | 1 | `sp -0x1b` | `-` |
| `str` at 0x43014 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x43018 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4301c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4302c | store | 4 | `sp -0x28` | `ret(0x43404)` |
| `ldr` at 0x43030 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4303c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x43044 | load | 4 | `abs 0x43220` | `-` |
| `ldr` at 0x43048 | load | 4 | `abs 0x43224` | `-` |
| `str` at 0x43074 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x43078 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x43080 | load | 4 | `abs 0x43220` | `-` |
| `ldr` at 0x43084 | load | 4 | `abs 0x43224` | `-` |
| `ldr` at 0x43090 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x430a0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x430ac | load | 4 | `abs 0x43240` | `-` |
| `str` at 0x430b0 | store | 4 | `(unknown)` | `0x108eec` |
| `ldr` at 0x430b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x430b8 | load | 4 | `[sp-32] +0x8` | `-` |
| `str` at 0x430bc | store | 4 | `(unknown)` | `[[sp-32]+8]` |
| `str` at 0x430c0 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `str` at 0x430c4 | store | 4 | `(unknown)` | `ret(0x219a8)` |
| `ldr` at 0x430cc | load | 4 | `abs 0x43218` | `-` |
| `ldr` at 0x430d0 | load | 4 | `abs 0x43244` | `-` |
| `ldr` at 0x430dc | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x430e8 | load | 1 | `sp -0x1a` | `-` |
| `ldrb` at 0x430f4 | load | 1 | `sp -0x1b` | `-` |
| `str` at 0x43104 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x43108 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4310c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x4311c | store | 4 | `sp -0x28` | `ret(0x43404)` |
| `ldr` at 0x43120 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4312c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x43134 | load | 4 | `abs 0x43220` | `-` |
| `ldr` at 0x43138 | load | 4 | `abs 0x43224` | `-` |
| `str` at 0x43164 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x43168 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x43170 | load | 4 | `abs 0x43220` | `-` |
| `ldr` at 0x43174 | load | 4 | `abs 0x43224` | `-` |
| `ldr` at 0x43180 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x43190 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4319c | load | 4 | `abs 0x43240` | `-` |
| `str` at 0x431a0 | store | 4 | `(unknown)` | `0x108eec` |
| `ldr` at 0x431a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x431a8 | load | 4 | `[sp-36] +0x8` | `-` |
| `str` at 0x431ac | store | 4 | `(unknown)` | `[[sp-36]+8]` |
| `str` at 0x431b0 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `str` at 0x431b4 | store | 4 | `(unknown)` | `ret(0x219a8)` |
| `ldr` at 0x431bc | load | 4 | `abs 0x43218` | `-` |
| `ldr` at 0x431c0 | load | 4 | `abs 0x43248` | `-` |
| `ldr` at 0x431cc | load | 4 | `abs 0x43210` | `-` |
| `str` at 0x431d8 | store | 4 | `sp -0x2c` | `ret(0x22dc0)` |
| `ldr` at 0x431dc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x431e8 | load | 4 | `abs 0x4324c` | `-` |
| `str` at 0x431ec | store | 4 | `(unknown)` | `0x108f08` |
| `ldr` at 0x431f0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x431f4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x431fc | load | 4 | `abs 0x43218` | `-` |
| `ldr` at 0x43200 | load | 4 | `abs 0x43250` | `-` |
| `ldmdb` at 0x4320c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x4320c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4320c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4320c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x42e0c | `(unknown callee) 0x22a38` | `0x147fe4` | `?` | `r2` | `r3` |
| `bl` at 0x42e44 | `(unknown callee) 0x21e28` | `0x9` | `0x108af4` | `0x2ef` | `0x0` |
| `bl` at 0x42e54 | `(unknown callee) 0x43d48` | `[sp-24]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0x42e6c | `(unknown callee) 0x43e34` | `[sp-24]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0x42ea0 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x108b40` | `0x108b48` |
| `bl` at 0x42eac | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x42edc | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x108b40` | `0x108b48` |
| `bl` at 0x42eec | `(unknown callee) 0x1b508` | `[sp-48]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x42f00 | `(unknown callee) 0xd7558` | `[sp-24]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0x42f30 | `(unknown callee) 0x21e28` | `0x1` | `0x108af4` | `0x2f8` | `0x0` |
| `bl` at 0x42f44 | `(unknown callee) 0xd6cf8` | `[sp-24]` | `[sp-25]:1` | `[sp-26]:1` | `[sp-25]:1` |
| `bl` at 0x42f68 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x108eb0` | `0x108eb8` |
| `bl` at 0x42f74 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x42fa4 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x108eb0` | `0x108eb8` |
| `bl` at 0x42fb4 | `(unknown callee) 0x1b508` | `[sp-48]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x42ff0 | `(unknown callee) 0x21e28` | `0x6` | `0x108af4` | `0x2ff` | `0x0` |
| `bl` at 0x43024 | `(unknown callee) 0x43404` | `[sp-24]` | `[sp-32]` | `?` | `?` |
| `bl` at 0x4304c | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x108b40` | `0x108b48` |
| `bl` at 0x43058 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x43088 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x108b40` | `0x108b48` |
| `bl` at 0x43098 | `(unknown callee) 0x1b508` | `[sp-48]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x430a4 | `(unknown callee) 0x219a8` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x430d8 | `(unknown callee) 0x21e28` | `0x3` | `0x108af4` | `0x307` | `0x0` |
| `bl` at 0x43114 | `(unknown callee) 0x43404` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x4313c | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x108b40` | `0x108b48` |
| `bl` at 0x43148 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x43178 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x108b40` | `0x108b48` |
| `bl` at 0x43188 | `(unknown callee) 0x1b508` | `[sp-48]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x43194 | `(unknown callee) 0x219a8` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x431c8 | `(unknown callee) 0x21e28` | `0x3` | `0x108af4` | `0x30d` | `0x0` |
| `bl` at 0x431d0 | `(unknown callee) 0x22dc0` | `0x147fe4` | `?` | `?` | `?` |
| `bl` at 0x43208 | `(unknown callee) 0x21e28` | `0x9` | `0x108af4` | `0x315` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x42e20 | `ret(0x22a38) == 0x0` | `ret(0x22a38) != 0x0` |
| `bne` at 0x42e80 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0x42e8c | `ret(0x43e34) != 0x0` | `ret(0x43e34) == 0x0` |
| `beq` at 0x42f54 | `[0x14850c] == 0x0` | `[0x14850c] != 0x0` |
| `beq` at 0x42ffc | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `beq` at 0x43038 | `ret(0x43404) == 0x0` | `ret(0x43404) != 0x0` |
| `beq` at 0x430e4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x43128 | `ret(0x43404) == 0x0` | `ret(0x43404) != 0x0` |
| `beq` at 0x431e4 | `ret(0x22dc0) == 0x0` | `ret(0x22dc0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4320c | `r0` | `?` |
