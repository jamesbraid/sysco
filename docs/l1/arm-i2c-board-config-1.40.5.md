# ARM I2C board configuration callers in 1.40.5

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
These routines have no unresolved computed transfers in the fact sheets. Numeric callee effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/i2c-board-config-facts.txt) retain all tool sections.

## `Unknown_i2cConfig`

- `Unknown_i2cConfig`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24590 to 0x24618.
  Established. The first unnamed call targets `0xd1f18` with `0x0`, `0xff3f0`, `0x9`, `r3` (`bl` at
  0x245ac). A byte counter is assigned `0x0` at `sp -0x11` and later `([sp-17]:1+1 & 0xff)` (`strb`
  at 0x245bc, `strb` at 0x245e8). The loop branch has an unsigned-lower-or-same flags predicate with
  unknown operands (`bls` at 0x245c8). The exit carries `ret(0x202d0)` (`ldmdb` at 0x24604). I2C
  configuration effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x245a4 | load | 4 | `abs 0x24608` | `-` |
| `ldr` at 0x245b0 | load | 4 | `abs 0x2460c` | `-` |
| `strb` at 0x245bc | store | 1 | `sp -0x11` | `0x0` |
| `ldrb` at 0x245c0 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x245d0 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x245e0 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x245e8 | store | 1 | `sp -0x11` | `([sp-17]:1+1 & 0xff)` |
| `ldr` at 0x245f0 | load | 4 | `abs 0x24610` | `-` |
| `ldr` at 0x245fc | load | 4 | `abs 0x24614` | `-` |
| `ldmdb` at 0x24604 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x24604 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x24604 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x245ac | `(unknown callee) 0xd1f18` | `0x0` | `0xff3f0` | `0x9` | `r3` |
| `bl` at 0x245b4 | `(unknown callee) 0x202d0` | `0xff45c` | `?` | `?` | `?` |
| `bl` at 0x245dc | `(unknown callee) 0xd224c` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x245f4 | `(unknown callee) 0x202d0` | `0xff460` | `?` | `?` | `?` |
| `bl` at 0x245f8 | `(unknown callee) 0x41324` | `ret(0x202d0)` | `?` | `?` | `?` |
| `bl` at 0x24600 | `(unknown callee) 0x202d0` | `0xff464` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0x245c8 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x24604 | `r0` | `ret(0x202d0)` |

## `Vortix_i2cConfig`

- `Vortix_i2cConfig`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x25988 to 0x25c98.
  Established. Included four-byte stack assignments retain `0x24a6c` at `sp -0x18` and later zero at
  `sp -0x18` and `sp -0x14` (`str` at 0x25a00, `str` at 0x25a74, `str` at 0x25a7c). Four zero-result
  branches take their edges past pairs of further calls (`beq` at 0x25b58, `beq` at 0x25b94,
  `beq` at 0x25bd0, `beq` at 0x25c0c, `bl` at 0x25b6c, `bl` at 0x25b80, `bl` at 0x25ba8,
  `bl` at 0x25bbc, `bl` at 0x25be4, `bl` at 0x25bf8, `bl` at 0x25c20, `bl` at 0x25c34). Each tests the result of the prior unnamed `0x5e754`
  call (`bl` at 0x25b4c, `bl` at 0x25b88, `bl` at 0x25bc4, `bl` at 0x25c00). The last unnamed `0xc8f8c` call
  carries `0xce6b0`, `0x5`, `0xf`, `0x12` (`bl` at 0x25c34). Device-registration and board meanings
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x25998 | load | 4 | `abs 0x25c3c` | `-` |
| `ldr` at 0x259a4 | load | 4 | `abs 0x25c40` | `-` |
| `ldr` at 0x259b4 | load | 4 | `abs 0x25c44` | `-` |
| `ldr` at 0x259c4 | load | 4 | `abs 0x25c48` | `-` |
| `ldr` at 0x259d4 | load | 4 | `abs 0x25c4c` | `-` |
| `ldr` at 0x259e4 | load | 4 | `abs 0x25c50` | `-` |
| `ldr` at 0x259fc | load | 4 | `abs 0x25c54` | `-` |
| `str` at 0x25a00 | store | 4 | `sp -0x18` | `0x24a6c` |
| `ldr` at 0x25a08 | load | 4 | `abs 0x25c58` | `-` |
| `ldr` at 0x25a10 | load | 4 | `abs 0x25c5c` | `-` |
| `ldr` at 0x25a1c | load | 4 | `abs 0x25c60` | `-` |
| `ldr` at 0x25a2c | load | 4 | `abs 0x25c64` | `-` |
| `ldr` at 0x25a3c | load | 4 | `abs 0x25c68` | `-` |
| `ldr` at 0x25a4c | load | 4 | `abs 0x25c6c` | `-` |
| `ldr` at 0x25a58 | load | 4 | `abs 0x25c70` | `-` |
| `ldr` at 0x25a64 | load | 4 | `abs 0x25c74` | `-` |
| `str` at 0x25a74 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x25a7c | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x25a80 | load | 4 | `abs 0x25c78` | `-` |
| `ldr` at 0x25a88 | load | 4 | `abs 0x25c7c` | `-` |
| `ldr` at 0x25a98 | load | 4 | `abs 0x25c80` | `-` |
| `ldr` at 0x25aa4 | load | 4 | `abs 0x25c84` | `-` |
| `ldr` at 0x25ac4 | load | 4 | `abs 0x25c88` | `-` |
| `ldr` at 0x25ad0 | load | 4 | `abs 0x25c8c` | `-` |
| `ldr` at 0x25ae4 | load | 4 | `abs 0x25c90` | `-` |
| `ldr` at 0x25af8 | load | 4 | `abs 0x25c94` | `-` |
| `ldr` at 0x25b0c | load | 4 | `abs 0x25c8c` | `-` |
| `ldr` at 0x25b20 | load | 4 | `abs 0x25c90` | `-` |
| `ldr` at 0x25b34 | load | 4 | `abs 0x25c94` | `-` |
| `ldr` at 0x25b5c | load | 4 | `abs 0x25c8c` | `-` |
| `ldr` at 0x25b70 | load | 4 | `abs 0x25c94` | `-` |
| `ldr` at 0x25b98 | load | 4 | `abs 0x25c8c` | `-` |
| `ldr` at 0x25bac | load | 4 | `abs 0x25c94` | `-` |
| `ldr` at 0x25bd4 | load | 4 | `abs 0x25c8c` | `-` |
| `ldr` at 0x25be8 | load | 4 | `abs 0x25c94` | `-` |
| `ldr` at 0x25c10 | load | 4 | `abs 0x25c8c` | `-` |
| `ldr` at 0x25c24 | load | 4 | `abs 0x25c94` | `-` |
| `ldmdb` at 0x25c38 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x25c38 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x25c38 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2599c | `(unknown callee) 0xc80a4` | `0x25774` | `r1` | `r2` | `r3` |
| `bl` at 0x259ac | `(unknown callee) 0xd4f24` | `0x0` | `0xff580` | `0x5` | `?` |
| `bl` at 0x259bc | `(unknown callee) 0xd8cd8` | `0x0` | `0xff5b8` | `0x6` | `?` |
| `bl` at 0x259cc | `(unknown callee) 0xd64f4` | `0x0` | `0xff8d0` | `0x36` | `?` |
| `bl` at 0x259dc | `(unknown callee) 0xd1f18` | `0x0` | `0xfff58` | `0x48` | `?` |
| `bl` at 0x259ec | `(unknown callee) 0xcc980` | `0x0` | `0x100318` | `0x13` | `?` |
| `bl` at 0x259f8 | `(unknown callee) 0xccbc4` | `0x0` | `0x40` | `?` | `?` |
| `bl` at 0x25a14 | `(unknown callee) 0x3c5cc` | `0x0` | `0x100568` | `0x29` | `0x24a3c` |
| `bl` at 0x25a24 | `(unknown callee) 0x5cb50` | `0x0` | `0x1009e4` | `0xe` | `?` |
| `bl` at 0x25a34 | `(unknown callee) 0x53488` | `0x0` | `0x100a9c` | `0x13` | `?` |
| `bl` at 0x25a44 | `(unknown callee) 0x458a4` | `0x0` | `0x100a54` | `0x1` | `?` |
| `bl` at 0x25a54 | `(unknown callee) 0x45618` | `0x0` | `0x100a78` | `0x3` | `?` |
| `bl` at 0x25a60 | `(unknown callee) 0xe9b2c` | `0x1430f8` | `0x6` | `?` | `?` |
| `bl` at 0x25a6c | `(unknown callee) 0x27898` | `0x100be4` | `0x2` | `?` | `?` |
| `bl` at 0x25a90 | `(unknown callee) 0x5f764` | `0x100b94` | `0x4` | `0x25750` | `0x0` |
| `bl` at 0x25aa0 | `(unknown callee) 0x4160c` | `0x0` | `0x100d14` | `0x1a` | `?` |
| `bl` at 0x25aac | `(unknown callee) 0x44e60` | `0x100b34` | `0x4` | `?` | `?` |
| `bl` at 0x25ab4 | `(unknown callee) 0x516d8` | `0x9` | `?` | `?` | `?` |
| `bl` at 0x25ab8 | `(unknown callee) 0x24874` | `ret(0x516d8)` | `?` | `?` | `?` |
| `bl` at 0x25abc | `(unknown callee) 0x41324` | `ret(0x24874)` | `?` | `?` | `?` |
| `bl` at 0x25ac0 | `(unknown callee) 0x24964` | `ret(0x41324)` | `?` | `?` | `?` |
| `bl` at 0x25acc | `(unknown callee) 0xc8df0` | `0x143128` | `0x6` | `?` | `?` |
| `bl` at 0x25ae0 | `(unknown callee) 0xc8f8c` | `0xd9e3c` | `0x0` | `0x0` | `0x0` |
| `bl` at 0x25af4 | `(unknown callee) 0xc8f8c` | `0xd7768` | `0x0` | `0x0` | `0x0` |
| `bl` at 0x25b08 | `(unknown callee) 0xc8f8c` | `0xce6b0` | `0x0` | `0x0` | `0x1` |
| `bl` at 0x25b1c | `(unknown callee) 0xc8f8c` | `0xd9e3c` | `0x1` | `0x1` | `0x1` |
| `bl` at 0x25b30 | `(unknown callee) 0xc8f8c` | `0xd7768` | `0x1` | `0x9` | `0x9` |
| `bl` at 0x25b44 | `(unknown callee) 0xc8f8c` | `0xce6b0` | `0x1` | `0x2` | `0x2` |
| `bl` at 0x25b4c | `(unknown callee) 0x5e754` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x25b6c | `(unknown callee) 0xc8f8c` | `0xd9e3c` | `0x2` | `0x2` | `0x2` |
| `bl` at 0x25b80 | `(unknown callee) 0xc8f8c` | `0xce6b0` | `0x2` | `0x3` | `0x6` |
| `bl` at 0x25b88 | `(unknown callee) 0x5e754` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x25ba8 | `(unknown callee) 0xc8f8c` | `0xd9e3c` | `0x3` | `0x3` | `0x3` |
| `bl` at 0x25bbc | `(unknown callee) 0xc8f8c` | `0xce6b0` | `0x3` | `0x7` | `0xa` |
| `bl` at 0x25bc4 | `(unknown callee) 0x5e754` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x25be4 | `(unknown callee) 0xc8f8c` | `0xd9e3c` | `0x4` | `0x4` | `0x4` |
| `bl` at 0x25bf8 | `(unknown callee) 0xc8f8c` | `0xce6b0` | `0x4` | `0xb` | `0xe` |
| `bl` at 0x25c00 | `(unknown callee) 0x5e754` | `0x3` | `?` | `?` | `?` |
| `bl` at 0x25c20 | `(unknown callee) 0xc8f8c` | `0xd9e3c` | `0x5` | `0x5` | `0x5` |
| `bl` at 0x25c34 | `(unknown callee) 0xc8f8c` | `0xce6b0` | `0x5` | `0xf` | `0x12` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x25b58 | `ret(0x5e754) == 0x0` | `ret(0x5e754) != 0x0` |
| `beq` at 0x25b94 | `ret(0x5e754) == 0x0` | `ret(0x5e754) != 0x0` |
| `beq` at 0x25bd0 | `ret(0x5e754) == 0x0` | `ret(0x5e754) != 0x0` |
| `beq` at 0x25c0c | `ret(0x5e754) == 0x0` | `ret(0x5e754) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x25c38 | `r0` | `?` |

## `Gemini_i2cConfig`

- `Gemini_i2cConfig`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x27438 to 0x27694.
  Established. A byte read targets absolute `0x148bd5` (`ldrb` at 0x275d0); its zero value takes the
  branch to `0x27604`, bypassing two unnamed `0xc8f8c` calls with candidates `0xd9e3c`, `0x1`,
  `0x1`, `0x1`, and `0xce6b0`, `0x1`, `0x3`, `0x6` (`beq` at 0x275d8, `bl` at 0x275ec, `bl` at
  0x27600). The final call to that target uses `0xce6b0`, `0x2`, `0x2`, `0x2` (`bl` at 0x27628). I2C
  topology effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x27448 | load | 4 | `abs 0x27630` | `-` |
| `ldr` at 0x27454 | load | 4 | `abs 0x27634` | `-` |
| `ldr` at 0x2746c | load | 4 | `abs 0x27638` | `-` |
| `ldr` at 0x2747c | load | 4 | `abs 0x2763c` | `-` |
| `ldr` at 0x2748c | load | 4 | `abs 0x27640` | `-` |
| `ldr` at 0x2749c | load | 4 | `abs 0x27644` | `-` |
| `ldr` at 0x274a8 | load | 4 | `abs 0x27648` | `-` |
| `str` at 0x274ac | store | 4 | `sp -0x18` | `0x26398` |
| `ldr` at 0x274b4 | load | 4 | `abs 0x2764c` | `-` |
| `ldr` at 0x274bc | load | 4 | `abs 0x27650` | `-` |
| `ldr` at 0x274c8 | load | 4 | `abs 0x27654` | `-` |
| `ldr` at 0x274d8 | load | 4 | `abs 0x27658` | `-` |
| `ldr` at 0x274e8 | load | 4 | `abs 0x2765c` | `-` |
| `ldr` at 0x274f8 | load | 4 | `abs 0x27660` | `-` |
| `ldr` at 0x27504 | load | 4 | `abs 0x27664` | `-` |
| `str` at 0x27514 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x2751c | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x27520 | load | 4 | `abs 0x27668` | `-` |
| `ldr` at 0x27528 | load | 4 | `abs 0x2766c` | `-` |
| `ldr` at 0x27538 | load | 4 | `abs 0x27670` | `-` |
| `ldr` at 0x27544 | load | 4 | `abs 0x27674` | `-` |
| `ldr` at 0x27564 | load | 4 | `abs 0x27678` | `-` |
| `ldr` at 0x27578 | load | 4 | `abs 0x2767c` | `-` |
| `ldr` at 0x27584 | load | 4 | `abs 0x27680` | `-` |
| `ldr` at 0x27590 | load | 4 | `abs 0x27684` | `-` |
| `ldr` at 0x275a4 | load | 4 | `abs 0x27688` | `-` |
| `ldr` at 0x275b8 | load | 4 | `abs 0x27684` | `-` |
| `ldr` at 0x275cc | load | 4 | `abs 0x2768c` | `-` |
| `ldrb` at 0x275d0 | load | 1 | `abs 0x148bd5` | `-` |
| `ldr` at 0x275dc | load | 4 | `abs 0x27690` | `-` |
| `ldr` at 0x275f0 | load | 4 | `abs 0x27688` | `-` |
| `ldr` at 0x27604 | load | 4 | `abs 0x27690` | `-` |
| `ldr` at 0x27618 | load | 4 | `abs 0x27688` | `-` |
| `ldmdb` at 0x2762c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2762c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2762c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2744c | `(unknown callee) 0xc80a4` | `0x2725c` | `r1` | `r2` | `r3` |
| `bl` at 0x2745c | `(unknown callee) 0xd4f24` | `0x0` | `0x101214` | `0x2` | `?` |
| `bl` at 0x27464 | `(unknown callee) 0xc9c0c` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x27474 | `(unknown callee) 0xd8cd8` | `0x0` | `0x10122c` | `0x2` | `?` |
| `bl` at 0x27484 | `(unknown callee) 0xd64f4` | `0x0` | `0x101340` | `0x12` | `?` |
| `bl` at 0x27494 | `(unknown callee) 0xd1f18` | `0x0` | `0x1014f8` | `0x15` | `?` |
| `bl` at 0x274a4 | `(unknown callee) 0xcc980` | `0x0` | `0x101630` | `0x7` | `?` |
| `bl` at 0x274c0 | `(unknown callee) 0x3c5cc` | `0x0` | `0x101748` | `0x14` | `0x26364` |
| `bl` at 0x274d0 | `(unknown callee) 0x53488` | `0x0` | `0x101a90` | `0x7` | `?` |
| `bl` at 0x274e0 | `(unknown callee) 0x5cb50` | `0x0` | `0x101978` | `0x5` | `?` |
| `bl` at 0x274f0 | `(unknown callee) 0x458a4` | `0x0` | `0x1019a0` | `0x4` | `?` |
| `bl` at 0x27500 | `(unknown callee) 0x45618` | `0x0` | `0x101a24` | `0x9` | `?` |
| `bl` at 0x2750c | `(unknown callee) 0xe9b2c` | `0x14319c` | `0x2` | `?` | `?` |
| `bl` at 0x27530 | `(unknown callee) 0x5f764` | `0x101c10` | `0x1` | `0x27008` | `0x0` |
| `bl` at 0x27540 | `(unknown callee) 0x4160c` | `0x0` | `0x101b74` | `0x6` | `?` |
| `bl` at 0x2754c | `(unknown callee) 0x44e60` | `0x101c04` | `0x1` | `?` | `?` |
| `bl` at 0x27554 | `(unknown callee) 0x516d8` | `0x11` | `?` | `?` | `?` |
| `bl` at 0x27558 | `(unknown callee) 0x25fac` | `ret(0x516d8)` | `?` | `?` | `?` |
| `bl` at 0x2755c | `(unknown callee) 0x41324` | `ret(0x25fac)` | `?` | `?` | `?` |
| `bl` at 0x27560 | `(unknown callee) 0x262b8` | `ret(0x41324)` | `?` | `?` | `?` |
| `bl` at 0x27570 | `(unknown callee) 0x53ab4` | `0x101af8` | `0x2` | `0x1` | `?` |
| `bl` at 0x27574 | `(unknown callee) 0x53d08` | `ret(0x53ab4)` | `?` | `?` | `?` |
| `bl` at 0x27580 | `(unknown callee) 0xdb6a0` | `0x101b2c` | `0x1` | `?` | `?` |
| `bl` at 0x2758c | `(unknown callee) 0xc8df0` | `0x1431ac` | `0x4` | `?` | `?` |
| `bl` at 0x275a0 | `(unknown callee) 0xc8f8c` | `0xd7768` | `0x0` | `0x11` | `0x11` |
| `bl` at 0x275b4 | `(unknown callee) 0xc8f8c` | `0xce6b0` | `0x0` | `0x0` | `0x1` |
| `bl` at 0x275c8 | `(unknown callee) 0xc8f8c` | `0xd7768` | `0x3` | `0x0` | `0x3` |
| `bl` at 0x275ec | `(unknown callee) 0xc8f8c` | `0xd9e3c` | `0x1` | `0x1` | `0x1` |
| `bl` at 0x27600 | `(unknown callee) 0xc8f8c` | `0xce6b0` | `0x1` | `0x3` | `0x6` |
| `bl` at 0x27614 | `(unknown callee) 0xc8f8c` | `0xd9e3c` | `0x2` | `0x0` | `0x0` |
| `bl` at 0x27628 | `(unknown callee) 0xc8f8c` | `0xce6b0` | `0x2` | `0x2` | `0x2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x275d8 | `[0x148bd5]:1 == 0x0` | `[0x148bd5]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2762c | `r0` | `ret(0xc8f8c)` |

## `OpusTio_i2cConfig`

- `OpusTio_i2cConfig`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2bc40 to 0x2bee8.
  Established. Zero `[0x148bee]:1` takes the branch to `0x2bdbc`, bypassing the unnamed `0x57d08`
  call (`beq` at 0x2bd78, `bl` at 0x2bdb8); its fall-through includes four-byte stack assignments
  `0x10383c`, `0x4`, `0x43520000` at `sp -0x1c`, `sp -0x18`, `sp -0x14` respectively (`str` at
  0x2bd94, `str` at 0x2bd9c, `str` at 0x2bda4). Later zero-byte branches test absolute `0x148bed`
  and `0x148bee` (`beq` at 0x2be10, `beq` at 0x2be48). Board and bus-registration effects remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x2bc50 | load | 4 | `abs 0x2be78` | `-` |
| `ldr` at 0x2bc5c | load | 4 | `abs 0x2be7c` | `-` |
| `ldr` at 0x2bc74 | load | 4 | `abs 0x2be80` | `-` |
| `ldr` at 0x2bc84 | load | 4 | `abs 0x2be84` | `-` |
| `ldr` at 0x2bc94 | load | 4 | `abs 0x2be88` | `-` |
| `ldr` at 0x2bca4 | load | 4 | `abs 0x2be8c` | `-` |
| `ldr` at 0x2bcb0 | load | 4 | `abs 0x2be90` | `-` |
| `str` at 0x2bcb4 | store | 4 | `sp -0x1c` | `0x2ab34` |
| `ldr` at 0x2bcbc | load | 4 | `abs 0x2be94` | `-` |
| `ldr` at 0x2bcc4 | load | 4 | `abs 0x2be98` | `-` |
| `ldr` at 0x2bcd0 | load | 4 | `abs 0x2be9c` | `-` |
| `ldr` at 0x2bce0 | load | 4 | `abs 0x2bea0` | `-` |
| `ldr` at 0x2bcf0 | load | 4 | `abs 0x2bea4` | `-` |
| `ldr` at 0x2bd00 | load | 4 | `abs 0x2bea8` | `-` |
| `ldr` at 0x2bd0c | load | 4 | `abs 0x2beac` | `-` |
| `str` at 0x2bd1c | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x2bd24 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x2bd28 | load | 4 | `abs 0x2beb0` | `-` |
| `ldr` at 0x2bd30 | load | 4 | `abs 0x2beb4` | `-` |
| `ldr` at 0x2bd40 | load | 4 | `abs 0x2beb8` | `-` |
| `ldr` at 0x2bd4c | load | 4 | `abs 0x2bebc` | `-` |
| `ldr` at 0x2bd6c | load | 4 | `abs 0x2bec0` | `-` |
| `ldrb` at 0x2bd70 | load | 1 | `abs 0x148bee` | `-` |
| `ldr` at 0x2bd7c | load | 4 | `abs 0x2bec4` | `-` |
| `ldr` at 0x2bd90 | load | 4 | `abs 0x2bec8` | `-` |
| `str` at 0x2bd94 | store | 4 | `sp -0x1c` | `0x10383c` |
| `str` at 0x2bd9c | store | 4 | `sp -0x18` | `0x4` |
| `ldr` at 0x2bda0 | load | 4 | `abs 0x2becc` | `-` |
| `str` at 0x2bda4 | store | 4 | `sp -0x14` | `0x43520000` |
| `ldr` at 0x2bdb0 | load | 4 | `abs 0x2bed0` | `-` |
| `ldr` at 0x2bdbc | load | 4 | `abs 0x2bed4` | `-` |
| `ldr` at 0x2bdc8 | load | 4 | `abs 0x2bed8` | `-` |
| `ldr` at 0x2bddc | load | 4 | `abs 0x2bedc` | `-` |
| `ldr` at 0x2bdf0 | load | 4 | `abs 0x2bed8` | `-` |
| `ldr` at 0x2be04 | load | 4 | `abs 0x2bee0` | `-` |
| `ldrb` at 0x2be08 | load | 1 | `abs 0x148bed` | `-` |
| `ldr` at 0x2be14 | load | 4 | `abs 0x2bee4` | `-` |
| `ldr` at 0x2be28 | load | 4 | `abs 0x2bedc` | `-` |
| `ldr` at 0x2be3c | load | 4 | `abs 0x2bec0` | `-` |
| `ldrb` at 0x2be40 | load | 1 | `abs 0x148bee` | `-` |
| `ldr` at 0x2be4c | load | 4 | `abs 0x2bee4` | `-` |
| `ldr` at 0x2be60 | load | 4 | `abs 0x2bedc` | `-` |
| `ldmdb` at 0x2be74 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2be74 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2be74 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2bc54 | `(unknown callee) 0xc80a4` | `0x2ba64` | `r1` | `r2` | `r3` |
| `bl` at 0x2bc64 | `(unknown callee) 0xd4f24` | `0x0` | `0x102f0c` | `0x2` | `?` |
| `bl` at 0x2bc6c | `(unknown callee) 0xc9c0c` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x2bc7c | `(unknown callee) 0xd8cd8` | `0x0` | `0x102f24` | `0x2` | `?` |
| `bl` at 0x2bc8c | `(unknown callee) 0xd64f4` | `0x0` | `0x103050` | `0x14` | `?` |
| `bl` at 0x2bc9c | `(unknown callee) 0xd1f18` | `0x0` | `0x103230` | `0x16` | `?` |
| `bl` at 0x2bcac | `(unknown callee) 0xcc980` | `0x0` | `0x103374` | `0x7` | `?` |
| `bl` at 0x2bcc8 | `(unknown callee) 0x3c5cc` | `0x0` | `0x103488` | `0x13` | `0x2ab00` |
| `bl` at 0x2bcd8 | `(unknown callee) 0x53488` | `0x0` | `0x1037b4` | `0x7` | `?` |
| `bl` at 0x2bce8 | `(unknown callee) 0x5cb50` | `0x0` | `0x10369c` | `0x5` | `?` |
| `bl` at 0x2bcf8 | `(unknown callee) 0x458a4` | `0x0` | `0x1036c4` | `0x4` | `?` |
| `bl` at 0x2bd08 | `(unknown callee) 0x45618` | `0x0` | `0x103748` | `0x9` | `?` |
| `bl` at 0x2bd14 | `(unknown callee) 0xe9b2c` | `0x143274` | `0x2` | `?` | `?` |
| `bl` at 0x2bd38 | `(unknown callee) 0x5f764` | `0x1038ec` | `0x1` | `0x2b810` | `0x0` |
| `bl` at 0x2bd48 | `(unknown callee) 0x4160c` | `0x0` | `0x103880` | `0x4` | `?` |
| `bl` at 0x2bd54 | `(unknown callee) 0x44e60` | `0x1038e0` | `0x1` | `?` | `?` |
| `bl` at 0x2bd5c | `(unknown callee) 0x516d8` | `0x13` | `?` | `?` | `?` |
| `bl` at 0x2bd60 | `(unknown callee) 0x2a728` | `ret(0x516d8)` | `?` | `?` | `?` |
| `bl` at 0x2bd64 | `(unknown callee) 0x41324` | `ret(0x2a728)` | `?` | `?` | `?` |
| `bl` at 0x2bd68 | `(unknown callee) 0x2aa50` | `ret(0x41324)` | `?` | `?` | `?` |
| `bl` at 0x2bd88 | `(unknown callee) 0x53ab4` | `0x103810` | `0x1` | `0x1` | `[0x148bee]:1` |
| `bl` at 0x2bd8c | `(unknown callee) 0x53d08` | `ret(0x53ab4)` | `?` | `?` | `?` |
| `bl` at 0x2bdb8 | `(unknown callee) 0x57d08` | `0x0` | `0x1` | `0x103824` | `0x2` |
| `bl` at 0x2bdc4 | `(unknown callee) 0xc8df0` | `0x143284` | `0x4` | `?` | `?` |
| `bl` at 0x2bdd8 | `(unknown callee) 0xc8f8c` | `0xd7768` | `0x0` | `0x13` | `0x13` |
| `bl` at 0x2bdec | `(unknown callee) 0xc8f8c` | `0xce6b0` | `0x0` | `0x0` | `0x1` |
| `bl` at 0x2be00 | `(unknown callee) 0xc8f8c` | `0xd7768` | `0x3` | `0x0` | `0x3` |
| `bl` at 0x2be24 | `(unknown callee) 0xc8f8c` | `0xd9e3c` | `0x1` | `0x1` | `0x1` |
| `bl` at 0x2be38 | `(unknown callee) 0xc8f8c` | `0xce6b0` | `0x1` | `0x3` | `0x6` |
| `bl` at 0x2be5c | `(unknown callee) 0xc8f8c` | `0xd9e3c` | `0x2` | `0x0` | `0x0` |
| `bl` at 0x2be70 | `(unknown callee) 0xc8f8c` | `0xce6b0` | `0x2` | `0x2` | `0x2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x2bd78 | `[0x148bee]:1 == 0x0` | `[0x148bee]:1 != 0x0` |
| `beq` at 0x2be10 | `[0x148bed]:1 == 0x0` | `[0x148bed]:1 != 0x0` |
| `beq` at 0x2be48 | `[0x148bee]:1 == 0x0` | `[0x148bee]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2be74 | `r0` | `ret(0xc8f8c)` |

## `Dorado_i2cConfig`

- `Dorado_i2cConfig`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2ecfc to 0x2ef80.
  Established. Before the unnamed `0x57d08` call, four-byte stack stores include `0x104f64`, `0x7`,
  `0x43520000` at `sp -0x1c`, `sp -0x18`, `sp -0x14` (`str` at 0x2ee50, `str` at 0x2ee58, `str` at
  0x2ee60). That call carries `0x0`, `0x1`, `0x104f34`, `0x4` (`bl` at 0x2ee74). Zero `[0x148c35]:1`
  takes the branch to `0x2eef8` (`beq` at 0x2eecc). Nonzero falls through (`beq` at 0x2eecc). Configuration-record and
  I2C-device meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x2ed0c | load | 4 | `abs 0x2ef10` | `-` |
| `ldr` at 0x2ed18 | load | 4 | `abs 0x2ef14` | `-` |
| `ldr` at 0x2ed30 | load | 4 | `abs 0x2ef18` | `-` |
| `ldr` at 0x2ed40 | load | 4 | `abs 0x2ef1c` | `-` |
| `ldr` at 0x2ed50 | load | 4 | `abs 0x2ef20` | `-` |
| `ldr` at 0x2ed60 | load | 4 | `abs 0x2ef24` | `-` |
| `ldr` at 0x2ed6c | load | 4 | `abs 0x2ef28` | `-` |
| `str` at 0x2ed70 | store | 4 | `sp -0x1c` | `0x2d540` |
| `ldr` at 0x2ed78 | load | 4 | `abs 0x2ef2c` | `-` |
| `ldr` at 0x2ed80 | load | 4 | `abs 0x2ef30` | `-` |
| `ldr` at 0x2ed8c | load | 4 | `abs 0x2ef34` | `-` |
| `ldr` at 0x2ed9c | load | 4 | `abs 0x2ef38` | `-` |
| `ldr` at 0x2edac | load | 4 | `abs 0x2ef3c` | `-` |
| `ldr` at 0x2edbc | load | 4 | `abs 0x2ef40` | `-` |
| `ldr` at 0x2edc8 | load | 4 | `abs 0x2ef44` | `-` |
| `ldr` at 0x2edd8 | load | 4 | `abs 0x2ef48` | `-` |
| `str` at 0x2ede8 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x2edf0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x2edf4 | load | 4 | `abs 0x2ef4c` | `-` |
| `ldr` at 0x2edfc | load | 4 | `abs 0x2ef50` | `-` |
| `ldr` at 0x2ee0c | load | 4 | `abs 0x2ef54` | `-` |
| `ldr` at 0x2ee2c | load | 4 | `abs 0x2ef58` | `-` |
| `ldr` at 0x2ee38 | load | 4 | `abs 0x2ef5c` | `-` |
| `ldr` at 0x2ee4c | load | 4 | `abs 0x2ef60` | `-` |
| `str` at 0x2ee50 | store | 4 | `sp -0x1c` | `0x104f64` |
| `str` at 0x2ee58 | store | 4 | `sp -0x18` | `0x7` |
| `ldr` at 0x2ee5c | load | 4 | `abs 0x2ef64` | `-` |
| `str` at 0x2ee60 | store | 4 | `sp -0x14` | `0x43520000` |
| `ldr` at 0x2ee6c | load | 4 | `abs 0x2ef68` | `-` |
| `ldr` at 0x2ee78 | load | 4 | `abs 0x2ef6c` | `-` |
| `ldr` at 0x2ee84 | load | 4 | `abs 0x2ef70` | `-` |
| `ldr` at 0x2ee98 | load | 4 | `abs 0x2ef74` | `-` |
| `ldr` at 0x2eeac | load | 4 | `abs 0x2ef78` | `-` |
| `ldr` at 0x2eec0 | load | 4 | `abs 0x2ef7c` | `-` |
| `ldrb` at 0x2eec4 | load | 1 | `abs 0x148c35` | `-` |
| `ldr` at 0x2eed0 | load | 4 | `abs 0x2ef70` | `-` |
| `ldr` at 0x2eee4 | load | 4 | `abs 0x2ef74` | `-` |
| `ldr` at 0x2eef8 | load | 4 | `abs 0x2ef78` | `-` |
| `ldmdb` at 0x2ef0c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2ef0c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2ef0c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2ed10 | `(unknown callee) 0xc80a4` | `0x2eb20` | `r1` | `r2` | `r3` |
| `bl` at 0x2ed20 | `(unknown callee) 0xd4f24` | `0x0` | `0x104594` | `0x2` | `?` |
| `bl` at 0x2ed28 | `(unknown callee) 0xc9c0c` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x2ed38 | `(unknown callee) 0xd8cd8` | `0x0` | `0x1045a4` | `0x2` | `?` |
| `bl` at 0x2ed48 | `(unknown callee) 0xd64f4` | `0x0` | `0x1046e4` | `0x16` | `?` |
| `bl` at 0x2ed58 | `(unknown callee) 0xd1f18` | `0x0` | `0x1048dc` | `0x14` | `?` |
| `bl` at 0x2ed68 | `(unknown callee) 0xcc980` | `0x0` | `0x104a08` | `0x7` | `?` |
| `bl` at 0x2ed84 | `(unknown callee) 0x3c5cc` | `0x0` | `0x104ae0` | `0xe` | `0x2d510` |
| `bl` at 0x2ed94 | `(unknown callee) 0x53488` | `0x0` | `0x104e9c` | `0x7` | `?` |
| `bl` at 0x2eda4 | `(unknown callee) 0x5cb50` | `0x0` | `0x104c68` | `0x4` | `?` |
| `bl` at 0x2edb4 | `(unknown callee) 0x458a4` | `0x0` | `0x104c88` | `0x5` | `?` |
| `bl` at 0x2edc4 | `(unknown callee) 0x45618` | `0x0` | `0x104d18` | `0xb` | `?` |
| `bl` at 0x2edd4 | `(unknown callee) 0x30a48` | `0x104dac` | `0x5` | `0x14` | `?` |
| `bl` at 0x2ede0 | `(unknown callee) 0xe9b2c` | `0x143304` | `0x2` | `?` | `?` |
| `bl` at 0x2ee04 | `(unknown callee) 0x5f764` | `0x10506c` | `0x1` | `0x2e8d4` | `0x0` |
| `bl` at 0x2ee14 | `(unknown callee) 0x4160c` | `0x0` | `0x104fd4` | `0x6` | `?` |
| `bl` at 0x2ee1c | `(unknown callee) 0x516d8` | `0x15` | `?` | `?` | `?` |
| `bl` at 0x2ee20 | `(unknown callee) 0x2d318` | `ret(0x516d8)` | `?` | `?` | `?` |
| `bl` at 0x2ee24 | `(unknown callee) 0x41324` | `ret(0x2d318)` | `?` | `?` | `?` |
| `bl` at 0x2ee28 | `(unknown callee) 0x2d464` | `ret(0x41324)` | `?` | `?` | `?` |
| `bl` at 0x2ee34 | `(unknown callee) 0x52efc` | `0x105080` | `0x3` | `?` | `?` |
| `bl` at 0x2ee44 | `(unknown callee) 0x53ab4` | `0x104f0c` | `0x2` | `0x1` | `?` |
| `bl` at 0x2ee48 | `(unknown callee) 0x53d08` | `ret(0x53ab4)` | `?` | `?` | `?` |
| `bl` at 0x2ee74 | `(unknown callee) 0x57d08` | `0x0` | `0x1` | `0x104f34` | `0x4` |
| `bl` at 0x2ee80 | `(unknown callee) 0xc8df0` | `0x143314` | `0x3` | `?` | `?` |
| `bl` at 0x2ee94 | `(unknown callee) 0xc8f8c` | `0xd9e3c` | `0x0` | `0x0` | `0x0` |
| `bl` at 0x2eea8 | `(unknown callee) 0xc8f8c` | `0xce6b0` | `0x0` | `0x0` | `0x2` |
| `bl` at 0x2eebc | `(unknown callee) 0xc8f8c` | `0xd7768` | `0x2` | `0x0` | `0x3` |
| `bl` at 0x2eee0 | `(unknown callee) 0xc8f8c` | `0xd9e3c` | `0x1` | `0x1` | `0x1` |
| `bl` at 0x2eef4 | `(unknown callee) 0xc8f8c` | `0xce6b0` | `0x1` | `0x3` | `0x6` |
| `bl` at 0x2ef08 | `(unknown callee) 0xc8f8c` | `0xd7768` | `0x0` | `0x15` | `0x15` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x2eecc | `[0x148c35]:1 == 0x0` | `[0x148c35]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2ef0c | `r0` | `ret(0xc8f8c)` |

## `OpusIP65_i2cConfig`

- `OpusIP65_i2cConfig`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x305bc to 0x30844.
  Established. The unnamed `0x64474` call carries `0x105b78`, `0x1`, `0x30198`, `0x0` (`bl` at
  0x306b4). A zero `[0x148c42]:1` later takes the branch to `0x3072c`, bypassing the unnamed
  `0x57d08` call (`beq` at 0x306e8, `bl` at 0x30728). That call has `0x0`, `0x1`, `0x105ab4`, `0x2`
  (`bl` at 0x30728). The last unnamed `0xc8f8c` call uses `0xce6b0`, `0x2`, `0x2`, `0x2` (`bl` at
  0x307d4). Board initialization meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x305cc | load | 4 | `abs 0x307dc` | `-` |
| `ldr` at 0x305d8 | load | 4 | `abs 0x307e0` | `-` |
| `ldr` at 0x305f0 | load | 4 | `abs 0x307e4` | `-` |
| `ldr` at 0x30600 | load | 4 | `abs 0x307e8` | `-` |
| `ldr` at 0x30610 | load | 4 | `abs 0x307ec` | `-` |
| `ldr` at 0x30620 | load | 4 | `abs 0x307f0` | `-` |
| `ldr` at 0x3062c | load | 4 | `abs 0x307f4` | `-` |
| `str` at 0x30630 | store | 4 | `sp -0x1c` | `0x2f584` |
| `ldr` at 0x30638 | load | 4 | `abs 0x307f8` | `-` |
| `ldr` at 0x30640 | load | 4 | `abs 0x307fc` | `-` |
| `ldr` at 0x3064c | load | 4 | `abs 0x30800` | `-` |
| `ldr` at 0x3065c | load | 4 | `abs 0x30804` | `-` |
| `ldr` at 0x3066c | load | 4 | `abs 0x30808` | `-` |
| `ldr` at 0x3067c | load | 4 | `abs 0x3080c` | `-` |
| `ldr` at 0x30688 | load | 4 | `abs 0x30810` | `-` |
| `str` at 0x30698 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x306a0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x306a4 | load | 4 | `abs 0x30814` | `-` |
| `ldr` at 0x306ac | load | 4 | `abs 0x30818` | `-` |
| `ldr` at 0x306bc | load | 4 | `abs 0x3081c` | `-` |
| `ldr` at 0x306dc | load | 4 | `abs 0x30820` | `-` |
| `ldrb` at 0x306e0 | load | 1 | `abs 0x148c42` | `-` |
| `ldr` at 0x306ec | load | 4 | `abs 0x30824` | `-` |
| `ldr` at 0x30700 | load | 4 | `abs 0x30828` | `-` |
| `str` at 0x30704 | store | 4 | `sp -0x1c` | `0x105acc` |
| `str` at 0x3070c | store | 4 | `sp -0x18` | `0x4` |
| `ldr` at 0x30710 | load | 4 | `abs 0x3082c` | `-` |
| `str` at 0x30714 | store | 4 | `sp -0x14` | `0x43520000` |
| `ldr` at 0x30720 | load | 4 | `abs 0x30830` | `-` |
| `ldr` at 0x3072c | load | 4 | `abs 0x30834` | `-` |
| `ldr` at 0x30740 | load | 4 | `abs 0x30838` | `-` |
| `ldr` at 0x30754 | load | 4 | `abs 0x30834` | `-` |
| `ldr` at 0x30768 | load | 4 | `abs 0x3083c` | `-` |
| `ldrb` at 0x3076c | load | 1 | `abs 0x148c41` | `-` |
| `ldr` at 0x30778 | load | 4 | `abs 0x30834` | `-` |
| `ldr` at 0x3078c | load | 4 | `abs 0x30838` | `-` |
| `ldr` at 0x307a0 | load | 4 | `abs 0x30820` | `-` |
| `ldrb` at 0x307a4 | load | 1 | `abs 0x148c42` | `-` |
| `ldr` at 0x307b0 | load | 4 | `abs 0x30840` | `-` |
| `ldr` at 0x307c4 | load | 4 | `abs 0x30838` | `-` |
| `ldmdb` at 0x307d8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x307d8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x307d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x305d0 | `(unknown callee) 0xc80a4` | `0x303e0` | `r1` | `r2` | `r3` |
| `bl` at 0x305e0 | `(unknown callee) 0xd4f24` | `0x0` | `0x1052ac` | `0x2` | `?` |
| `bl` at 0x305e8 | `(unknown callee) 0xc9c0c` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x305f8 | `(unknown callee) 0xd8cd8` | `0x0` | `0x1052c4` | `0x1` | `?` |
| `bl` at 0x30608 | `(unknown callee) 0xd64f4` | `0x0` | `0x105388` | `0xe` | `?` |
| `bl` at 0x30618 | `(unknown callee) 0xd1f18` | `0x0` | `0x1054ec` | `0x14` | `?` |
| `bl` at 0x30628 | `(unknown callee) 0xcc980` | `0x0` | `0x105618` | `0x6` | `?` |
| `bl` at 0x30644 | `(unknown callee) 0x3c5cc` | `0x0` | `0x10573c` | `0x14` | `0x2f550` |
| `bl` at 0x30654 | `(unknown callee) 0x53488` | `0x0` | `0x105a44` | `0x7` | `?` |
| `bl` at 0x30664 | `(unknown callee) 0x5cb50` | `0x0` | `0x10596c` | `0x3` | `?` |
| `bl` at 0x30674 | `(unknown callee) 0x458a4` | `0x0` | `0x105984` | `0x4` | `?` |
| `bl` at 0x30684 | `(unknown callee) 0x45618` | `0x0` | `0x1059e8` | `0x7` | `?` |
| `bl` at 0x30690 | `(unknown callee) 0xe9b2c` | `0x143378` | `0x2` | `?` | `?` |
| `bl` at 0x306b4 | `(unknown callee) 0x64474` | `0x105b78` | `0x1` | `0x30198` | `0x0` |
| `bl` at 0x306c4 | `(unknown callee) 0x4160c` | `0x0` | `0x105b10` | `0x4` | `?` |
| `bl` at 0x306cc | `(unknown callee) 0x516d8` | `0xd` | `?` | `?` | `?` |
| `bl` at 0x306d0 | `(unknown callee) 0x2f20c` | `ret(0x516d8)` | `?` | `?` | `?` |
| `bl` at 0x306d4 | `(unknown callee) 0x41324` | `ret(0x2f20c)` | `?` | `?` | `?` |
| `bl` at 0x306d8 | `(unknown callee) 0x2f4bc` | `ret(0x41324)` | `?` | `?` | `?` |
| `bl` at 0x306f8 | `(unknown callee) 0x53ab4` | `0x105aa0` | `0x1` | `0x1` | `[0x148c42]:1` |
| `bl` at 0x306fc | `(unknown callee) 0x53d08` | `ret(0x53ab4)` | `?` | `?` | `?` |
| `bl` at 0x30728 | `(unknown callee) 0x57d08` | `0x0` | `0x1` | `0x105ab4` | `0x2` |
| `bl` at 0x3073c | `(unknown callee) 0xc8f8c` | `0xd7768` | `0x0` | `0xd` | `0xd` |
| `bl` at 0x30750 | `(unknown callee) 0xc8f8c` | `0xce6b0` | `0x0` | `0x0` | `0x1` |
| `bl` at 0x30764 | `(unknown callee) 0xc8f8c` | `0xd7768` | `0x3` | `0x0` | `0x3` |
| `bl` at 0x30788 | `(unknown callee) 0xc8f8c` | `0xd7768` | `0x1` | `0xb` | `0xc` |
| `bl` at 0x3079c | `(unknown callee) 0xc8f8c` | `0xce6b0` | `0x1` | `0x3` | `0x5` |
| `bl` at 0x307c0 | `(unknown callee) 0xc8f8c` | `0xd9e3c` | `0x2` | `0x0` | `0x0` |
| `bl` at 0x307d4 | `(unknown callee) 0xc8f8c` | `0xce6b0` | `0x2` | `0x2` | `0x2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x306e8 | `[0x148c42]:1 == 0x0` | `[0x148c42]:1 != 0x0` |
| `beq` at 0x30774 | `[0x148c41]:1 == 0x0` | `[0x148c41]:1 != 0x0` |
| `beq` at 0x307ac | `[0x148c42]:1 == 0x0` | `[0x148c42]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x307d8 | `r0` | `ret(0xc8f8c)` |

## `i2cBus`

- `i2cBus`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc807c to 0xc80a4.
  Established. Two explicit stores save four-byte `r0` at `sp -0x14` and byte `(r1 & 0xff)` at
  `sp -0x15` (`str` at 0xc808c, `strb` at 0xc8094). The exit carries `(r1 & 0xff)` (`ldmdb` at 0xc80a0).
  No call or conditional branch occurs. Bus-selection effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc808c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xc8094 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xc8098 | load | 1 | `sp -0x15` | `-` |
| `ldmdb` at 0xc80a0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc80a0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc80a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc80a0 | `r0` | `(r1 & 0xff)` |

## `i2cGetSwitch`

- `i2cGetSwitch`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc830c to 0xc8348.
  Established. A four-byte absolute read at `0x2b2f20` supplies the base for a byte read at
  `[0x2b2f20] +0x0` (`ldr` at 0xc8328, `ldrb` at 0xc832c). A byte assignment at `r0 +0x0` has
  unknown value (`strb` at 0xc8334). The exit carries `0x0` (`ldmdb` at 0xc8340). Switch register
  and bit meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc831c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc8320 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc8324 | load | 4 | `abs 0xc8344` | `-` |
| `ldr` at 0xc8328 | load | 4 | `abs 0x2b2f20` | `-` |
| `ldrb` at 0xc832c | load | 1 | `[0x2b2f20] +0x0` | `-` |
| `strb` at 0xc8334 | store | 1 | `r0 +0x0` | `?` |
| `ldmdb` at 0xc8340 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc8340 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc8340 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc8340 | `r0` | `0x0` |

## `i2cTest`

- `i2cTest`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb3588 to 0xb3834.
  Established. Initial unnamed calls include `0xd5440` with `0x0`, `sp-28`, `r2`, `sp-28`, and
  `0xd7210` with `0x0`, `sp-36`, unknown, `sp-36` (`bl` at 0xb35b0, `bl` at 0xb35c8). Later
  four-byte assignments at `sp -0x3c` retain sums involving `ret(0xb3834)`, `ret(0xb3968)`,
  `ret(0xb3aa0)` and `ret(0xce960)` (`str` at 0xb3684, `str` at 0xb36fc, `str` at 0xb3780, `str` at
  0xb37e8). Zero `[sp-60]` takes the branch to `0xb3814` (`beq` at 0xb3804). Test-result meanings
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb3598 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb35a0 | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0xb35b8 | store | 4 | `sp -0x18` | `ret(0xd5440)` |
| `str` at 0xb35d0 | store | 4 | `sp -0x20` | `ret(0xd7210)` |
| `str` at 0xb35e8 | store | 4 | `sp -0x28` | `ret(0xd273c)` |
| `str` at 0xb3600 | store | 4 | `sp -0x30` | `ret(0xcd15c)` |
| `str` at 0xb3608 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xb360c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xb3610 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb3624 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xb3628 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb362c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb363c | load | 1 | `sp -0x40` | `-` |
| `ldr` at 0xb3658 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb3660 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb3668 | load | 1 | `sp -0x40` | `-` |
| `ldr` at 0xb367c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb3684 | store | 4 | `sp -0x3c` | `([sp-60] + ret(0xb3834))` |
| `ldr` at 0xb3688 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xb3690 | store | 4 | `sp -0x40` | `[sp-64]+1` |
| `str` at 0xb369c | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xb36a0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb36a4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xb36b4 | load | 1 | `sp -0x40` | `-` |
| `ldr` at 0xb36d0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb36d8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xb36e0 | load | 1 | `sp -0x40` | `-` |
| `ldr` at 0xb36f4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb36fc | store | 4 | `sp -0x3c` | `([sp-60] + ret(0xb3968))` |
| `ldr` at 0xb3700 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xb3708 | store | 4 | `sp -0x40` | `[sp-64]+1` |
| `str` at 0xb3714 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xb3718 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb371c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xb372c | load | 1 | `sp -0x40` | `-` |
| `ldr` at 0xb3748 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb375c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xb3764 | load | 1 | `sp -0x40` | `-` |
| `ldr` at 0xb3778 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb3780 | store | 4 | `sp -0x3c` | `([sp-60] + ret(0xb3aa0))` |
| `ldr` at 0xb3784 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xb378c | store | 4 | `sp -0x40` | `[sp-64]+1` |
| `str` at 0xb3798 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xb379c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb37a0 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xb37b0 | load | 1 | `sp -0x40` | `-` |
| `ldrb` at 0xb37cc | load | 1 | `sp -0x40` | `-` |
| `ldr` at 0xb37e0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb37e8 | store | 4 | `sp -0x3c` | `([sp-60] + ret(0xce960))` |
| `ldr` at 0xb37ec | load | 4 | `sp -0x40` | `-` |
| `str` at 0xb37f4 | store | 4 | `sp -0x40` | `[sp-64]+1` |
| `ldr` at 0xb37fc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb380c | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xb3814 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xb381c | store | 4 | `sp -0x38` | `?` |
| `str` at 0xb3828 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xb382c | load | 4 | `sp -0x44` | `-` |
| `ldmdb` at 0xb3830 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb3830 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb3830 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb35b0 | `(unknown callee) 0xd5440` | `0x0` | `sp-28` | `r2` | `sp-28` |
| `bl` at 0xb35c8 | `(unknown callee) 0xd7210` | `0x0` | `sp-36` | `?` | `sp-36` |
| `bl` at 0xb35e0 | `(unknown callee) 0xd273c` | `0x0` | `sp-44` | `?` | `sp-44` |
| `bl` at 0xb35f8 | `(unknown callee) 0xcd15c` | `0x0` | `sp-52` | `?` | `sp-52` |
| `bl` at 0xb3648 | `(unknown callee) 0xd54ec` | `0x0` | `[sp-64]:1` | `?` | `[sp-64]:1` |
| `bl` at 0xb3674 | `(unknown callee) 0xb3834` | `?` | `[sp-64]:1` | `[sp-64]:1` | `?` |
| `bl` at 0xb36c0 | `(unknown callee) 0xd72bc` | `0x0` | `[sp-64]:1` | `?` | `[sp-64]:1` |
| `bl` at 0xb36ec | `(unknown callee) 0xb3968` | `?` | `[sp-64]:1` | `[sp-64]:1` | `?` |
| `bl` at 0xb3738 | `(unknown callee) 0xd27e4` | `0x0` | `[sp-64]:1` | `?` | `[sp-64]:1` |
| `bl` at 0xb3770 | `(unknown callee) 0xb3aa0` | `?` | `[sp-64]:1` | `[sp-64]:1` | `?` |
| `bl` at 0xb37bc | `(unknown callee) 0xcce28` | `0x0` | `[sp-64]:1` | `?` | `[sp-64]:1` |
| `bl` at 0xb37d8 | `(unknown callee) 0xce960` | `0x0` | `[sp-64]:1` | `?` | `[sp-64]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xb3618 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0xb3634 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xb3654 | `ret(0xd54ec) == 0x0` | `ret(0xd54ec) != 0x0` |
| `blt` at 0xb36ac | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xb36cc | `ret(0xd72bc) == 0x0` | `ret(0xd72bc) != 0x0` |
| `blt` at 0xb3724 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xb3744 | `ret(0xd27e4) == 0x0` | `ret(0xd27e4) != 0x0` |
| `blt` at 0xb37a8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xb37c8 | `ret(0xcce28) == 0x0` | `ret(0xcce28) != 0x0` |
| `beq` at 0xb3804 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb3830 | `r0` | `?` |

## `IP57_PowerUp`

- `IP57_PowerUp`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x60a6c to 0x60b74.
  Established. The unnamed `0x617b4` call has `(r0 & 0xff)`, `sp-24`, `sp-24`, `(r0 & 0xff)` (`bl`
  at 0x60a9c); its zero result takes the branch to `0x60ac0` (`beq` at 0x60ab0). A later unnamed
  `0xd76d4` call carries `0x0`, unknown, `0x80`, unknown (`bl` at 0x60b28); its zero-result branch
  reaches `0x60b54` (`beq` at 0x60b3c). Physical power-up effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x60a80 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x60a88 | store | 4 | `sp -0x1c` | `0x0` |
| `ldrb` at 0x60a8c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x60aa4 | store | 4 | `sp -0x20` | `ret(0x617b4)` |
| `ldr` at 0x60aa8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x60ab4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x60ab8 | store | 4 | `sp -0x28` | `ret(0x617b4)` |
| `strb` at 0x60ac4 | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0x60ac8 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0x60ad8 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x60ae4 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x60b04 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x60b08 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0x60b0c | load | 1 | `sp -0x21` | `-` |
| `str` at 0x60b30 | store | 4 | `sp -0x20` | `ret(0xd76d4)` |
| `ldr` at 0x60b34 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x60b40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x60b4c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x60b50 | store | 4 | `sp -0x1c` | `ret(0xd76d4)` |
| `ldrb` at 0x60b54 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x60b5c | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldr` at 0x60b64 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x60b68 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x60b6c | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x60b70 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x60b70 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x60b70 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x60a9c | `(unknown callee) 0x617b4` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x60af4 | `(unknown callee) 0x5e14c` | `?` | `?` | `?` | `?` |
| `bl` at 0x60b28 | `(unknown callee) 0xd76d4` | `0x0` | `?` | `0x80` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x60ab0 | `ret(0x617b4) == 0x0` | `ret(0x617b4) != 0x0` |
| `bls` at 0x60ad0 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x60b00 | `ret(0x5e14c) == 0x0` | `ret(0x5e14c) != 0x0` |
| `beq` at 0x60b3c | `ret(0xd76d4) == 0x0` | `ret(0xd76d4) != 0x0` |
| `bne` at 0x60b48 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x60b70 | `r0` | `?` |

## `IP57_PowerDown`

- `IP57_PowerDown`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x60b74 to 0x60c50.
  Established. A byte counter is assigned zero at `sp -0x21` (`strb` at 0x60bcc). The loop includes
  a byte read at `[sp-24] +0x2` (`ldrb` at 0x60be4). The unnamed `0xd768c` call carries `0x0`,
  unknown, `0x80`, unknown (`bl` at 0x60c04). Zero `ret(0xd768c)` takes the branch to `0x60c30`
  (`beq` at 0x60c18). Nonzero falls through (`beq` at 0x60c18). Power-down signaling is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x60b88 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x60b90 | store | 4 | `sp -0x1c` | `0x0` |
| `ldrb` at 0x60b94 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x60bac | store | 4 | `sp -0x20` | `ret(0x617b4)` |
| `ldr` at 0x60bb0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x60bbc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x60bc0 | store | 4 | `sp -0x28` | `ret(0x617b4)` |
| `strb` at 0x60bcc | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0x60bd0 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x60be0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x60be4 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0x60be8 | load | 1 | `sp -0x21` | `-` |
| `str` at 0x60c0c | store | 4 | `sp -0x20` | `ret(0xd768c)` |
| `ldr` at 0x60c10 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x60c1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x60c28 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x60c2c | store | 4 | `sp -0x1c` | `ret(0xd768c)` |
| `ldrb` at 0x60c30 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x60c38 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldr` at 0x60c40 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x60c44 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x60c48 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x60c4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x60c4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x60c4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x60ba4 | `(unknown callee) 0x617b4` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x60c04 | `(unknown callee) 0xd768c` | `0x0` | `?` | `0x80` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x60bb8 | `ret(0x617b4) == 0x0` | `ret(0x617b4) != 0x0` |
| `bls` at 0x60bd8 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x60c18 | `ret(0xd768c) == 0x0` | `ret(0xd768c) != 0x0` |
| `bne` at 0x60c24 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x60c4c | `r0` | `?` |

## `IP65_GetCpuMask`

- `IP65_GetCpuMask`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x651d4 to 0x65204.
  Established. The data read targets byte `(r0 & 0xff) +0x14927c` (`ldrb` at 0x651f4). The return
  retains the exact expression `[(r0 & 0xff)+1348220]:1` (`ldmdb` at 0x651fc). There are no calls or
  conditional branches. CPU-mask encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x651e8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x651ec | load | 4 | `abs 0x65200` | `-` |
| `ldrb` at 0x651f0 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x651f4 | load | 1 | `(r0 & 0xff) +0x14927c` | `-` |
| `ldmdb` at 0x651fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x651fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x651fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x651fc | `r0` | `[(r0 & 0xff)+1348220]:1` |

## `OpusIP65_BrickOpsInit`

- `OpusIP65_BrickOpsInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3089c to 0x30a48.
  Established. Included assignments write four-byte `0x30844` at `r0 +0x0`, four-byte `0x305bc` at
  `[sp-20] +0x4`, and byte `0x52` at `[sp-20] +0xc` (`str` at 0x308c0, `str` at 0x308cc, `strb` at
  0x308d8). Later four-byte stores include `0x64d00`, `0x64db0`, `0x64d54` at `[sp-20] +0x90`,
  `+0x94`, `+0x98` (`str` at 0x30998, `str` at 0x309a4, `str` at 0x309b0). Those stored numbers do
  not establish calls. Record-field and callback meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x308ac | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x308b0 | load | 4 | `abs 0x30a0c` | `-` |
| `str` at 0x308b4 | store | 4 | `sp -0x18` | `0x1433a8` |
| `ldr` at 0x308b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x308bc | load | 4 | `abs 0x30a10` | `-` |
| `str` at 0x308c0 | store | 4 | `r0 +0x0` | `0x30844` |
| `ldr` at 0x308c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x308c8 | load | 4 | `abs 0x30a14` | `-` |
| `str` at 0x308cc | store | 4 | `[sp-20] +0x4` | `0x305bc` |
| `ldr` at 0x308d0 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x308d8 | store | 1 | `[sp-20] +0xc` | `0x52` |
| `ldr` at 0x308dc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x308e4 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x308e8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x308f0 | store | 4 | `[sp-20] +0xa0` | `0xff` |
| `ldr` at 0x308f4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x308fc | store | 4 | `[sp-20] +0xa4` | `0xff` |
| `ldr` at 0x30900 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x30908 | store | 4 | `[sp-20] +0xa8` | `0xff` |
| `ldr` at 0x3090c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x30914 | store | 4 | `[sp-20] +0xac` | `0xff` |
| `ldr` at 0x30918 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3091c | load | 4 | `abs 0x30a18` | `-` |
| `str` at 0x30920 | store | 4 | `[sp-20] +0x14` | `0x105a9c` |
| `ldr` at 0x30924 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x30928 | load | 4 | `abs 0x30a1c` | `-` |
| `str` at 0x3092c | store | 4 | `[sp-20] +0xb0` | `0x10eee4` |
| `ldr` at 0x30930 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x30934 | load | 4 | `abs 0x30a20` | `-` |
| `str` at 0x30938 | store | 4 | `[sp-20] +0x7c` | `0x2f7c0` |
| `ldr` at 0x3093c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x30944 | store | 4 | `[sp-20] +0x68` | `0x1e` |
| `ldr` at 0x30948 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x30950 | store | 4 | `[sp-20] +0x6c` | `0x2d` |
| `ldr` at 0x30954 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3095c | store | 4 | `[sp-20] +0x70` | `0x32` |
| `ldr` at 0x30960 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x30968 | store | 4 | `[sp-20] +0x74` | `0x37` |
| `ldr` at 0x3096c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x30974 | store | 4 | `[sp-20] +0x78` | `0x17` |
| `ldr` at 0x30978 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x30980 | store | 4 | `[sp-20] +0x8c` | `0x1` |
| `ldr` at 0x30984 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3098c | store | 4 | `[sp-20] +0x88` | `0x1` |
| `ldr` at 0x30990 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x30994 | load | 4 | `abs 0x30a24` | `-` |
| `str` at 0x30998 | store | 4 | `[sp-20] +0x90` | `0x64d00` |
| `ldr` at 0x3099c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x309a0 | load | 4 | `abs 0x30a28` | `-` |
| `str` at 0x309a4 | store | 4 | `[sp-20] +0x94` | `0x64db0` |
| `ldr` at 0x309a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x309ac | load | 4 | `abs 0x30a2c` | `-` |
| `str` at 0x309b0 | store | 4 | `[sp-20] +0x98` | `0x64d54` |
| `ldr` at 0x309b4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x309bc | store | 4 | `[sp-20] +0xb4` | `0x1` |
| `ldr` at 0x309c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x309c4 | load | 4 | `abs 0x30a30` | `-` |
| `str` at 0x309c8 | store | 4 | `[sp-20] +0xc4` | `0x2f5ac` |
| `ldr` at 0x309cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x309d0 | load | 4 | `abs 0x30a34` | `-` |
| `str` at 0x309d4 | store | 4 | `[sp-20] +0xc8` | `0x2f668` |
| `ldr` at 0x309d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x309dc | load | 4 | `abs 0x30a38` | `-` |
| `str` at 0x309e0 | store | 4 | `[sp-20] +0xd0` | `0x2f720` |
| `ldr` at 0x309e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x309e8 | load | 4 | `abs 0x30a3c` | `-` |
| `str` at 0x309ec | store | 4 | `[sp-20] +0xd4` | `0x2f770` |
| `ldr` at 0x309f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x309f4 | load | 4 | `abs 0x30a40` | `-` |
| `str` at 0x309f8 | store | 4 | `[sp-20] +0xec` | `0x2fe04` |
| `ldr` at 0x309fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x30a00 | load | 4 | `abs 0x30a44` | `-` |
| `str` at 0x30a04 | store | 4 | `[sp-20] +0xf0` | `0x2fca4` |
| `ldmdb` at 0x30a08 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x30a08 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x30a08 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x30a08 | `r0` | `r0` |

## `L1CfgEventPowerState`

- `L1CfgEventPowerState`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x757f0 to 0x75848.
  Established. After an unnamed `0x75194` call, a four-byte read targets `ret(0x75194) +0x34` (`bl`
  at 0x75808, `ldr` at 0x7581c). Equality `[sp-20] == [ret(0x75194)+52]` takes the branch to
  `0x75844` (`beq` at 0x75824). The fall-through stores four-byte `[sp-20]` at `ret(0x75194) +0x34`
  and calls unnamed `0x75504` with `0x4`, `0x0`, `0x0`, `[sp-20]` (`str` at 0x75830, `bl` at
  0x75840). Power-state event semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x75800 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x75810 | store | 4 | `sp -0x18` | `ret(0x75194)` |
| `ldr` at 0x75814 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x75818 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7581c | load | 4 | `ret(0x75194) +0x34` | `-` |
| `ldr` at 0x75828 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7582c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x75830 | store | 4 | `ret(0x75194) +0x34` | `[sp-20]` |
| `ldmdb` at 0x75844 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x75844 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x75844 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x75808 | `(unknown callee) 0x75194` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x75840 | `(unknown callee) 0x75504` | `0x4` | `0x0` | `0x0` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x75824 | `[sp-20] == [ret(0x75194)+52]` | `[sp-20] != [ret(0x75194)+52]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x75844 | `r0` | `?` |

## `L1CfgEventNVRAM`

- `L1CfgEventNVRAM`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x75b54 to 0x75b80.
  Established. The unnamed `0x75504` call uses `0x66`, `r0`, `r1`, `r3` (`bl` at 0x75b78); its
  result reaches the exit (`ldmdb` at 0x75b7c). The explicit four-byte stack assignments retain
  entry `r0` at `sp -0x14` and `r1` at `sp -0x18` (`str` at 0x75b64, `str` at 0x75b68).
  Persistent-storage event effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x75b64 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x75b68 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x75b70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75b74 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x75b7c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x75b7c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x75b7c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x75b78 | `(unknown callee) 0x75504` | `0x66` | `r0` | `r1` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x75b7c | `r0` | `ret(0x75504)` |
