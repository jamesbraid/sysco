# ARM port state and interrupt paths in 1.40.5

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
These routines have no unresolved computed transfers. Numeric callee names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/port-state-facts.txt) retain all tool sections.

## `nl4r_init_ports`

- `nl4r_init_ports`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x28a1c to 0x28e24.
  Established. The unnamed `0xec86c` call carries `[sp-20]`, `0x5`, `0x0`, `sp-32` (`bl` at
  0x28d54). Included later unnamed `0xed170` calls use `[sp-20]`, `0x2`, `0x5`, `0x0` and `[sp-20]`,
  `0x2`, `0x9`, `0x0` (`bl` at 0x28d80, `bl` at 0x28dcc). An earlier byte read has effective target
  `([sp-40] + [sp-24]) +0x0` (`ldrb` at 0x28b78). Port selection and transfer encoding remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x28a2c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x28a30 | load | 4 | `abs 0x28e00` | `-` |
| `str` at 0x28a34 | store | 4 | `sp -0x28` | `0x101efc` |
| `ldr` at 0x28a38 | load | 4 | `abs 0x28e04` | `-` |
| `ldr` at 0x28a3c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x28a48 | load | 4 | `abs 0x28e08` | `-` |
| `ldr` at 0x28a4c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28a58 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x28a60 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x28a68 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x28a6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28a7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28a80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28a98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28aa0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x28aa8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x28ab0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x28ab4 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0x28ab8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x28abc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x28ac0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x28ac8 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x28ad0 | load | 4 | `abs 0x28e0c` | `-` |
| `ldr` at 0x28ad4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x28ae0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x28ae4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x28af8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x28afc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28b08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28b1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28b20 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x28b38 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x28b3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28b48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28b5c | load | 4 | `abs 0x28e04` | `-` |
| `ldr` at 0x28b60 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x28b6c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x28b70 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x28b78 | load | 1 | `([sp-40] + [sp-24]) +0x0` | `-` |
| `ldr` at 0x28b7c | load | 4 | `abs 0x28e10` | `-` |
| `ldr` at 0x28b84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x28b8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28b94 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28bac | store | 4 | `sp -0x24` | `ret(0xed170)` |
| `ldr` at 0x28bb0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x28bbc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x28bc0 | store | 4 | `sp -0x2c` | `ret(0xed170)` |
| `ldr` at 0x28bc8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28bd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28be8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x28bf8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28c00 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x28c08 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x28c0c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x28c14 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0x28c20 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x28c24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28c34 | load | 4 | `abs 0x28e04` | `-` |
| `ldr` at 0x28c38 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x28c44 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x28c48 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x28c50 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x28c54 | load | 4 | `abs 0x28e14` | `-` |
| `ldr` at 0x28c60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28c68 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28c80 | store | 4 | `sp -0x24` | `ret(0xed170)` |
| `ldr` at 0x28c84 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x28c90 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x28c94 | store | 4 | `sp -0x2c` | `ret(0xed170)` |
| `ldr` at 0x28c9c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x28ca4 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0x28cb0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x28cb4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28cc4 | load | 4 | `abs 0x28e04` | `-` |
| `ldr` at 0x28cc8 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x28cd4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x28cd8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x28ce0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x28ce4 | load | 4 | `abs 0x28e18` | `-` |
| `ldr` at 0x28cf0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28cf8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28d10 | store | 4 | `sp -0x24` | `ret(0xed170)` |
| `ldr` at 0x28d14 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x28d20 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x28d24 | store | 4 | `sp -0x2c` | `ret(0xed170)` |
| `ldr` at 0x28d2c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x28d34 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x28d3c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x28d40 | store | 4 | `sp -0x20` | `[sp-28]` |
| `ldr` at 0x28d48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28d58 | load | 4 | `abs 0x28e04` | `-` |
| `ldr` at 0x28d5c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x28d68 | load | 4 | `abs 0x28e1c` | `-` |
| `ldr` at 0x28d70 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28d88 | store | 4 | `sp -0x24` | `ret(0xed170)` |
| `ldr` at 0x28d8c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x28d98 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x28d9c | store | 4 | `sp -0x2c` | `ret(0xed170)` |
| `ldr` at 0x28da4 | load | 4 | `abs 0x28e04` | `-` |
| `ldr` at 0x28da8 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x28db4 | load | 4 | `abs 0x28e20` | `-` |
| `ldr` at 0x28dbc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28dd4 | store | 4 | `sp -0x24` | `ret(0xed170)` |
| `ldr` at 0x28dd8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x28de4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x28de8 | store | 4 | `sp -0x2c` | `ret(0xed170)` |
| `str` at 0x28df4 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x28df8 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x28dfc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x28dfc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x28dfc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x28a50 | `(unknown callee) 0x23fbc` | `0x101f08` | `r0` | `r2` | `[0x2adfd8]` |
| `bl` at 0x28a84 | `(unknown callee) 0x29510` | `?` | `?` | `?` | `?` |
| `bl` at 0x28ad8 | `(unknown callee) 0x2079c` | `0x101f2c` | `?` | `?` | `?` |
| `bl` at 0x28b18 | `(unknown callee) 0xec86c` | `[sp-20]` | `?` | `0x1` | `sp-32` |
| `bl` at 0x28b24 | `(unknown callee) 0x29510` | `[sp-20]` | `[sp-24]` | `?` | `?` |
| `bl` at 0x28b58 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+10` | `0x2` | `sp-32` |
| `bl` at 0x28b88 | `(unknown callee) 0x23fbc` | `0x101f38` | `[([sp-40] + [sp-24])]:1` | `[sp-32]` | `[([sp-40] + [sp-24])]:1` |
| `bl` at 0x28ba4 | `(unknown callee) 0xed170` | `[sp-20]` | `0x2` | `[sp-24]+10` | `0x0` |
| `bl` at 0x28be4 | `(unknown callee) 0xec33c` | `[sp-20]` | `[sp-24]+10` | `0x0` | `sp-32` |
| `bl` at 0x28c5c | `(unknown callee) 0x23fbc` | `0x101f64` | `?` | `[sp-40]` | `?` |
| `bl` at 0x28c78 | `(unknown callee) 0xed170` | `[sp-20]` | `0x2` | `?` | `0x0` |
| `bl` at 0x28cec | `(unknown callee) 0x23fbc` | `0x101f88` | `?` | `[sp-40]` | `?` |
| `bl` at 0x28d08 | `(unknown callee) 0xed170` | `[sp-20]` | `0x2` | `?` | `0x0` |
| `bl` at 0x28d54 | `(unknown callee) 0xec86c` | `[sp-20]` | `0x5` | `0x0` | `sp-32` |
| `bl` at 0x28d6c | `(unknown callee) 0x23fbc` | `0x101fac` | `?` | `?` | `[0x2adfd8]` |
| `bl` at 0x28d80 | `(unknown callee) 0xed170` | `[sp-20]` | `0x2` | `0x5` | `0x0` |
| `bl` at 0x28db8 | `(unknown callee) 0x23fbc` | `0x101fcc` | `?` | `?` | `[0x2adfd8]` |
| `bl` at 0x28dcc | `(unknown callee) 0xed170` | `[sp-20]` | `0x2` | `0x9` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x28a44 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `bls` at 0x28a74 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x28a90 | `ret(0x29510) == 0x0` | `ret(0x29510) != 0x0` |
| `bls` at 0x28aec | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bls` at 0x28b68 | `[0x2adfd8] <= 0x1 (unsigned, 32-bit)` | `[0x2adfd8] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x28bb8 | `ret(0xed170) >= 0x0 (signed, 32-bit)` | `ret(0xed170) < 0x0 (signed, 32-bit)` |
| `bne` at 0x28bf0 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bls` at 0x28c2c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bls` at 0x28c40 | `[0x2adfd8] <= 0x1 (unsigned, 32-bit)` | `[0x2adfd8] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x28c8c | `ret(0xed170) >= 0x0 (signed, 32-bit)` | `ret(0xed170) < 0x0 (signed, 32-bit)` |
| `bls` at 0x28cbc | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bls` at 0x28cd0 | `[0x2adfd8] <= 0x1 (unsigned, 32-bit)` | `[0x2adfd8] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x28d1c | `ret(0xed170) >= 0x0 (signed, 32-bit)` | `ret(0xed170) < 0x0 (signed, 32-bit)` |
| `bls` at 0x28d64 | `[0x2adfd8] <= 0x1 (unsigned, 32-bit)` | `[0x2adfd8] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x28d94 | `ret(0xed170) >= 0x0 (signed, 32-bit)` | `ret(0xed170) < 0x0 (signed, 32-bit)` |
| `bls` at 0x28db0 | `[0x2adfd8] <= 0x1 (unsigned, 32-bit)` | `[0x2adfd8] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x28de0 | `ret(0xed170) >= 0x0 (signed, 32-bit)` | `ret(0xed170) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x28dfc | `r0` | `?` |

## `portInit`

- `portInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4160c to 0x41958.
  Established. Included four-byte absolute stores write `0x1` at `0x148d14` and `ret(0x1b454)` at
  `0x148d10` (`str` at 0x41690, `str` at 0x416c4). Later byte stores with unknown effective targets
  retain `0x2`, `0x0` and `0x0` (`strb` at 0x418c8, `strb` at 0x418e8, `strb` at 0x41904). The
  unnamed `0x229b0` call has `0x148d18`, `0x109698`, `0x1`, `0x9` (`bl` at 0x41928). Port
  initialization fields and callback meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4161c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x41620 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x41624 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x41628 | load | 4 | `abs 0x41930` | `-` |
| `ldr` at 0x4162c | load | 4 | `abs 0x148d10` | `-` |
| `ldr` at 0x41638 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x41644 | load | 4 | `abs 0x41934` | `-` |
| `str` at 0x41648 | store | 4 | `sp -0x38` | `0x1095d0` |
| `ldr` at 0x4164c | load | 4 | `abs 0x41930` | `-` |
| `ldr` at 0x41650 | load | 4 | `abs 0x148d10` | `-` |
| `str` at 0x41654 | store | 4 | `sp -0x34` | `[0x148d10]` |
| `ldr` at 0x41658 | load | 4 | `abs 0x41938` | `-` |
| `ldr` at 0x4165c | load | 4 | `abs 0x148d14` | `-` |
| `str` at 0x41660 | store | 4 | `sp -0x30` | `[0x148d14]` |
| `ldr` at 0x41668 | load | 4 | `abs 0x4193c` | `-` |
| `ldr` at 0x41678 | load | 4 | `abs 0x41938` | `-` |
| `ldr` at 0x4167c | load | 4 | `abs 0x148d14` | `-` |
| `ldr` at 0x41688 | load | 4 | `abs 0x41938` | `-` |
| `str` at 0x41690 | store | 4 | `abs 0x148d14` | `0x1` |
| `ldr` at 0x41694 | load | 4 | `abs 0x41938` | `-` |
| `ldr` at 0x41698 | load | 4 | `abs 0x148d14` | `-` |
| `str` at 0x416ac | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x416b0 | load | 4 | `abs 0x41930` | `-` |
| `ldr` at 0x416b4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x416c4 | store | 4 | `abs 0x148d10` | `ret(0x1b454)` |
| `ldr` at 0x416c8 | load | 4 | `abs 0x148d10` | `-` |
| `ldr` at 0x416d4 | load | 4 | `abs 0x41940` | `-` |
| `str` at 0x416d8 | store | 4 | `sp -0x38` | `0x109614` |
| `ldr` at 0x416dc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x416e0 | store | 4 | `sp -0x34` | `[sp-44]` |
| `ldr` at 0x416e8 | load | 4 | `abs 0x4193c` | `-` |
| `ldr` at 0x416f8 | load | 4 | `abs 0x41930` | `-` |
| `ldr` at 0x416fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x41710 | load | 4 | `abs 0x148d10` | `-` |
| `str` at 0x41718 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x4171c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x41720 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x41724 | store | 4 | `[sp-40] +0x0` | `?` |
| `ldr` at 0x41728 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4172c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x41730 | store | 4 | `[sp-40] +0x8` | `[sp-32]` |
| `ldr` at 0x41734 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x41738 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4173c | load | 4 | `[sp-40] +0x8` | `-` |
| `str` at 0x4174c | store | 4 | `[sp-40] +0x4` | `ret(0x1b454)` |
| `ldr` at 0x41750 | load | 4 | `[sp-40] +0x4` | `-` |
| `ldr` at 0x4175c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x41764 | load | 4 | `abs 0x41944` | `-` |
| `ldr` at 0x41768 | load | 4 | `abs 0x41948` | `-` |
| `str` at 0x41794 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x41798 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x417a0 | load | 4 | `abs 0x41944` | `-` |
| `ldr` at 0x417a4 | load | 4 | `abs 0x41948` | `-` |
| `ldr` at 0x417b0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x417c0 | load | 4 | `abs 0x4194c` | `-` |
| `str` at 0x417c4 | store | 4 | `(unknown)` | `0x109658` |
| `str` at 0x417c8 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0x417cc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x417d0 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0x417d8 | load | 4 | `abs 0x4193c` | `-` |
| `strb` at 0x417ec | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0x417f0 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x417f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x417f8 | load | 4 | `[sp-40] +0x8` | `-` |
| `ldr` at 0x41808 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x4180c | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x41820 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldrb` at 0x41828 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x4182c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x41844 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x41848 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x4185c | load | 4 | `[sp-40] +0x0` | `-` |
| `ldrb` at 0x41864 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x41870 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x41874 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x41888 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldrb` at 0x41890 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x41894 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x418b0 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x418b4 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x418bc | load | 4 | `[sp-40] +0x4` | `-` |
| `strb` at 0x418c8 | store | 1 | `(unknown)` | `0x2` |
| `ldr` at 0x418d0 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x418d4 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x418dc | load | 4 | `[sp-40] +0x4` | `-` |
| `strb` at 0x418e8 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x418ec | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x418f0 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x418f8 | load | 4 | `[sp-40] +0x4` | `-` |
| `strb` at 0x41904 | store | 1 | `(unknown)` | `0x0` |
| `ldrb` at 0x41908 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x41910 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldr` at 0x41918 | load | 4 | `abs 0x41950` | `-` |
| `ldr` at 0x4191c | load | 4 | `abs 0x41954` | `-` |
| `ldmdb` at 0x4192c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x4192c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4192c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4192c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x41674 | `(unknown callee) 0x21eb8` | `0x9` | `0x109600` | `0x6c` | `0x0` |
| `bl` at 0x416bc | `(unknown callee) 0x1b454` | `[sp-44]` | `0x1` | `[0x148d14]` | `?` |
| `bl` at 0x416f4 | `(unknown callee) 0x21eb8` | `0x9` | `0x109600` | `0x78` | `0x0` |
| `bl` at 0x41744 | `(unknown callee) 0x1b454` | `[[sp-40]+8]` | `0x4` | `[sp-40]` | `[sp-40]` |
| `bl` at 0x4176c | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x10964c` | `0x109654` |
| `bl` at 0x41778 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x417a8 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x10964c` | `0x109654` |
| `bl` at 0x417b8 | `(unknown callee) 0x1b598` | `[sp-44]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x417e4 | `(unknown callee) 0x21eb8` | `0x9` | `0x109600` | `0x84` | `0x0` |
| `bl` at 0x41834 | `(unknown callee) 0xd72bc` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x4189c | `(unknown callee) 0xd72bc` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x41928 | `(unknown callee) 0x229b0` | `0x148d18` | `0x109698` | `0x1` | `0x9` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x41634 | `[0x148d10] == 0x0` | `[0x148d10] != 0x0` |
| `bne` at 0x41640 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0x41684 | `[0x148d14] != 0x0` | `[0x148d14] == 0x0` |
| `bne` at 0x416d0 | `[0x148d10] != 0x0` | `[0x148d10] == 0x0` |
| `bne` at 0x41758 | `[[sp-40]+4] != 0x0` | `[[sp-40]+4] == 0x0` |
| `blt` at 0x41800 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x41840 | `ret(0xd72bc) == 0x0` | `ret(0xd72bc) != 0x0` |
| `beq` at 0x4186c | `? == 0xff` | `? != 0xff` |
| `bne` at 0x418a8 | `ret(0xd72bc) != 0x0` | `ret(0xd72bc) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4192c | `r0` | `ret(0x229b0)` |

## `portEnable`

- `portEnable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x41958 to 0x41a8c.
  Established. An included byte store with unknown effective target writes `0x2` (`strb` at
  0x41a70). The prior branch takes `? == 0x0` to `0x41a74` (`beq` at 0x41a54). The unnamed
  `0x4f9a4` call carries `r0`, `0x0`, `0x10964c`, `0x109654` (`bl` at 0x419bc). Enable behavior and
  the flag operands remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x41968 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x41970 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x41974 | load | 4 | `abs 0x41a78` | `-` |
| `ldr` at 0x41978 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4198c | load | 4 | `abs 0x148d10` | `-` |
| `str` at 0x41994 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x41998 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x4199c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x419a0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x419ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x419b4 | load | 4 | `abs 0x41a7c` | `-` |
| `ldr` at 0x419b8 | load | 4 | `abs 0x41a80` | `-` |
| `str` at 0x419e4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x419e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x419f0 | load | 4 | `abs 0x41a7c` | `-` |
| `ldr` at 0x419f4 | load | 4 | `abs 0x41a80` | `-` |
| `ldr` at 0x41a00 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x41a10 | load | 4 | `abs 0x41a84` | `-` |
| `str` at 0x41a14 | store | 4 | `(unknown)` | `0x1096a4` |
| `str` at 0x41a18 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldrb` at 0x41a1c | load | 1 | `sp -0x15` | `-` |
| `str` at 0x41a20 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x41a28 | load | 4 | `abs 0x41a88` | `-` |
| `ldr` at 0x41a38 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x41a3c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x41a44 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x41a4c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x41a58 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x41a5c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x41a64 | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0x41a70 | store | 1 | `(unknown)` | `0x2` |
| `ldmdb` at 0x41a74 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x41a74 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x41a74 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x419bc | `(unknown callee) 0x4f9a4` | `r0` | `0x0` | `0x10964c` | `0x109654` |
| `bl` at 0x419c8 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x419f8 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x10964c` | `0x109654` |
| `bl` at 0x41a08 | `(unknown callee) 0x1b598` | `[sp-32]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x41a34 | `(unknown callee) 0x21eb8` | `0x9` | `0x109600` | `0xa1` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x419a8 | `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` |
| `beq` at 0x41a54 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x41a74 | `r0` | `?` |

## `portDisable`

- `portDisable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x41a8c to 0x41bc0.
  Established. The byte assignment with unknown effective target writes `0x1` (`strb` at 0x41ba4).
  The unnamed `0x21eb8` call has `0x9`, `0x109600`, `0xad`, `0x0` (`bl` at 0x41b68). An earlier
  four-byte absolute read targets `0x148d10` (`ldr` at 0x41ac0). Port-disable semantics and pointee
  layout remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x41a9c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x41aa4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x41aa8 | load | 4 | `abs 0x41bac` | `-` |
| `ldr` at 0x41aac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41ac0 | load | 4 | `abs 0x148d10` | `-` |
| `str` at 0x41ac8 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x41acc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x41ad0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x41ad4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x41ae0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41ae8 | load | 4 | `abs 0x41bb0` | `-` |
| `ldr` at 0x41aec | load | 4 | `abs 0x41bb4` | `-` |
| `str` at 0x41b18 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x41b1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41b24 | load | 4 | `abs 0x41bb0` | `-` |
| `ldr` at 0x41b28 | load | 4 | `abs 0x41bb4` | `-` |
| `ldr` at 0x41b34 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x41b44 | load | 4 | `abs 0x41bb8` | `-` |
| `str` at 0x41b48 | store | 4 | `(unknown)` | `0x1096c8` |
| `str` at 0x41b4c | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldrb` at 0x41b50 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x41b54 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x41b5c | load | 4 | `abs 0x41bbc` | `-` |
| `ldr` at 0x41b6c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x41b70 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x41b78 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x41b80 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x41b8c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x41b90 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x41b98 | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0x41ba4 | store | 1 | `(unknown)` | `0x1` |
| `ldmdb` at 0x41ba8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x41ba8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x41ba8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x41af0 | `(unknown callee) 0x4f9a4` | `r0` | `0x0` | `0x10964c` | `0x109654` |
| `bl` at 0x41afc | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x41b2c | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x10964c` | `0x109654` |
| `bl` at 0x41b3c | `(unknown callee) 0x1b598` | `[sp-32]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x41b68 | `(unknown callee) 0x21eb8` | `0x9` | `0x109600` | `0xad` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x41adc | `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` |
| `beq` at 0x41b88 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x41ba8 | `r0` | `?` |

## `portIsEnabled`

- `portIsEnabled`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x41bc0 to 0x41ce0.
  Established. The branch takes signed `(r1 & 0xff) < [[sp-28]+8]` to `0x41ca0` (`blt` at 0x41c10);
  its fall-through includes the unnamed `0x4f9a4` call with `r0`, `0x0`, `0x10964c`, `0x109654`
  (`bl` at 0x41c24). The later byte read has unknown effective target (`ldrb` at 0x41cb4).
  Enabled-state interpretation and the merged result remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x41bd0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x41bd8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x41bdc | load | 4 | `abs 0x41ccc` | `-` |
| `ldr` at 0x41be0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41bf4 | load | 4 | `abs 0x148d10` | `-` |
| `str` at 0x41bfc | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x41c00 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x41c04 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x41c08 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x41c14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41c1c | load | 4 | `abs 0x41cd0` | `-` |
| `ldr` at 0x41c20 | load | 4 | `abs 0x41cd4` | `-` |
| `str` at 0x41c4c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x41c50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41c58 | load | 4 | `abs 0x41cd0` | `-` |
| `ldr` at 0x41c5c | load | 4 | `abs 0x41cd4` | `-` |
| `ldr` at 0x41c68 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x41c78 | load | 4 | `abs 0x41cd8` | `-` |
| `str` at 0x41c7c | store | 4 | `(unknown)` | `0x1096ec` |
| `str` at 0x41c80 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldrb` at 0x41c84 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x41c88 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x41c90 | load | 4 | `abs 0x41cdc` | `-` |
| `ldr` at 0x41ca0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x41ca4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x41cac | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x41cb4 | load | 1 | `(unknown)` | `-` |
| `ldmdb` at 0x41cc8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x41cc8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x41cc8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x41c24 | `(unknown callee) 0x4f9a4` | `r0` | `0x0` | `0x10964c` | `0x109654` |
| `bl` at 0x41c30 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x41c60 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x10964c` | `0x109654` |
| `bl` at 0x41c70 | `(unknown callee) 0x1b598` | `[sp-32]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x41c9c | `(unknown callee) 0x21eb8` | `0x9` | `0x109600` | `0xb9` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x41c10 | `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x41cc8 | `r0` | `?` |

## `portPresent`

- `portPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x41ce0 to 0x41df4.
  Established. A byte assignment at an unknown effective target writes `0x2` (`strb` at 0x41dd8).
  The unnamed `0x1b598` call carries `[sp-32]`, `ret(0x4f9a4)`, unknown, `ret(0x4f9a4)` (`bl` at
  0x41d90). A prior four-byte stack store at `sp -0x20` retains unknown (`str` at 0x41d6c).
  Presence-state meaning and pointer layout remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x41cf0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x41cf8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x41cfc | load | 4 | `abs 0x41de0` | `-` |
| `ldr` at 0x41d00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41d14 | load | 4 | `abs 0x148d10` | `-` |
| `str` at 0x41d1c | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x41d20 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x41d24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x41d28 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x41d34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41d3c | load | 4 | `abs 0x41de4` | `-` |
| `ldr` at 0x41d40 | load | 4 | `abs 0x41de8` | `-` |
| `str` at 0x41d6c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x41d70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41d78 | load | 4 | `abs 0x41de4` | `-` |
| `ldr` at 0x41d7c | load | 4 | `abs 0x41de8` | `-` |
| `ldr` at 0x41d88 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x41d98 | load | 4 | `abs 0x41dec` | `-` |
| `str` at 0x41d9c | store | 4 | `(unknown)` | `0x109714` |
| `str` at 0x41da0 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldrb` at 0x41da4 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x41da8 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x41db0 | load | 4 | `abs 0x41df0` | `-` |
| `ldr` at 0x41dc0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x41dc4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x41dcc | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0x41dd8 | store | 1 | `(unknown)` | `0x2` |
| `ldmdb` at 0x41ddc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x41ddc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x41ddc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x41d44 | `(unknown callee) 0x4f9a4` | `r0` | `0x0` | `0x10964c` | `0x109654` |
| `bl` at 0x41d50 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x41d80 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x10964c` | `0x109654` |
| `bl` at 0x41d90 | `(unknown callee) 0x1b598` | `[sp-32]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x41dbc | `(unknown callee) 0x21eb8` | `0x9` | `0x109600` | `0xc8` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x41d30 | `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x41ddc | `r0` | `?` |

## `portNotPresent`

- `portNotPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x41df4 to 0x41f08.
  Established. The unnamed `0x21eb8` call uses `0x9`, `0x109600`, `0xd3`, `0x0` (`bl` at 0x41ed0).
  The later byte assignment has unknown effective target and writes `0x0` (`strb` at 0x41eec). An
  earlier four-byte read targets `[sp-28] +0x8` (`ldr` at 0x41e3c). Port absence and record meanings
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x41e04 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x41e0c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x41e10 | load | 4 | `abs 0x41ef4` | `-` |
| `ldr` at 0x41e14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41e28 | load | 4 | `abs 0x148d10` | `-` |
| `str` at 0x41e30 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x41e34 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x41e38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x41e3c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x41e48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41e50 | load | 4 | `abs 0x41ef8` | `-` |
| `ldr` at 0x41e54 | load | 4 | `abs 0x41efc` | `-` |
| `str` at 0x41e80 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x41e84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41e8c | load | 4 | `abs 0x41ef8` | `-` |
| `ldr` at 0x41e90 | load | 4 | `abs 0x41efc` | `-` |
| `ldr` at 0x41e9c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x41eac | load | 4 | `abs 0x41f00` | `-` |
| `str` at 0x41eb0 | store | 4 | `(unknown)` | `0x109738` |
| `str` at 0x41eb4 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldrb` at 0x41eb8 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x41ebc | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x41ec4 | load | 4 | `abs 0x41f04` | `-` |
| `ldr` at 0x41ed4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x41ed8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x41ee0 | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0x41eec | store | 1 | `(unknown)` | `0x0` |
| `ldmdb` at 0x41ef0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x41ef0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x41ef0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x41e58 | `(unknown callee) 0x4f9a4` | `r0` | `0x0` | `0x10964c` | `0x109654` |
| `bl` at 0x41e64 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x41e94 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x10964c` | `0x109654` |
| `bl` at 0x41ea4 | `(unknown callee) 0x1b598` | `[sp-32]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x41ed0 | `(unknown callee) 0x21eb8` | `0x9` | `0x109600` | `0xd3` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x41e44 | `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x41ef0 | `r0` | `?` |

## `portIsPresent`

- `portIsPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x41f08 to 0x42028.
  Established. An included byte read has unknown effective target (`ldrb` at 0x41ffc). The unnamed
  `0x1b598` call carries `[sp-32]`, `ret(0x4f9a4)`, unknown, `ret(0x4f9a4)` (`bl` at 0x41fb8). The
  return reports unknown (`ldmdb` at 0x42010). Presence testing and predicated result meanings
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x41f18 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x41f20 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x41f24 | load | 4 | `abs 0x42014` | `-` |
| `ldr` at 0x41f28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41f3c | load | 4 | `abs 0x148d10` | `-` |
| `str` at 0x41f44 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x41f48 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x41f4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x41f50 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x41f5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41f64 | load | 4 | `abs 0x42018` | `-` |
| `ldr` at 0x41f68 | load | 4 | `abs 0x4201c` | `-` |
| `str` at 0x41f94 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x41f98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41fa0 | load | 4 | `abs 0x42018` | `-` |
| `ldr` at 0x41fa4 | load | 4 | `abs 0x4201c` | `-` |
| `ldr` at 0x41fb0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x41fc0 | load | 4 | `abs 0x42020` | `-` |
| `str` at 0x41fc4 | store | 4 | `(unknown)` | `0x109760` |
| `str` at 0x41fc8 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldrb` at 0x41fcc | load | 1 | `sp -0x15` | `-` |
| `str` at 0x41fd0 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x41fd8 | load | 4 | `abs 0x42024` | `-` |
| `ldr` at 0x41fe8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x41fec | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x41ff4 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x41ffc | load | 1 | `(unknown)` | `-` |
| `ldmdb` at 0x42010 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x42010 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x42010 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x41f6c | `(unknown callee) 0x4f9a4` | `r0` | `0x0` | `0x10964c` | `0x109654` |
| `bl` at 0x41f78 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x41fa8 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x10964c` | `0x109654` |
| `bl` at 0x41fb8 | `(unknown callee) 0x1b598` | `[sp-32]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x41fe4 | `(unknown callee) 0x21eb8` | `0x9` | `0x109600` | `0xde` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x41f58 | `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x42010 | `r0` | `?` |

## `portGetCfg`

- `portGetCfg`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x42224 to 0x42284.
  Established. A four-byte assignment at `r1 +0x0` retains `[[sp-28]]` (`str` at 0x4226c). The later
  four-byte read at `[sp-28] +0x8` feeds the reported return `[[sp-28]+8]` (`ldr` at 0x42274,
  `ldmdb` at 0x4227c). Configuration record meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x42234 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x42238 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x4223c | load | 4 | `abs 0x42280` | `-` |
| `ldr` at 0x42240 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x42254 | load | 4 | `abs 0x148d10` | `-` |
| `str` at 0x4225c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x42260 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x42264 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x42268 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x4226c | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0x42270 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x42274 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldmdb` at 0x4227c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4227c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4227c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4227c | `r0` | `[[sp-28]+8]` |

## `portUpInternal`

- `portUpInternal`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x42284 to 0x4257c.
  Established. Included four-byte stack assignments at `sp -0x38` write `0xfffffb4e` and
  `0xfffffb4d` (`str` at 0x4234c, `str` at 0x42378). The unnamed `0x44048` call has `r0`,
  `(r1 & 0xff)`, unknown, `(r1 & 0xff)` (`bl` at 0x42304); its nonzero result takes the branch to
  `0x42328`
  (`bne` at 0x42310). Port-up sequencing and result codes remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x42294 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x4229c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x422a4 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x422ac | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x422b0 | load | 4 | `abs 0x42570` | `-` |
| `ldr` at 0x422b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x422c8 | load | 4 | `abs 0x148d10` | `-` |
| `str` at 0x422d0 | store | 4 | `sp -0x34` | `?` |
| `ldrb` at 0x422d4 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x422e4 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x422e8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x422ec | load | 4 | `[sp-52] +0x8` | `-` |
| `str` at 0x422f0 | store | 4 | `sp -0x30` | `[[sp-52]+8]` |
| `ldrb` at 0x422f8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x422fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x42314 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x42320 | store | 4 | `sp -0x38` | `ret(0x440dc)` |
| `ldr` at 0x42328 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x4232c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x42334 | load | 4 | `[sp-52] +0x4` | `-` |
| `ldrb` at 0x4233c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x42348 | load | 4 | `abs 0x42574` | `-` |
| `str` at 0x4234c | store | 4 | `sp -0x38` | `0xfffffb4e` |
| `ldr` at 0x42354 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x42358 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x42360 | load | 4 | `[sp-52] +0x4` | `-` |
| `ldrb` at 0x42368 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x42374 | load | 4 | `abs 0x42578` | `-` |
| `str` at 0x42378 | store | 4 | `sp -0x38` | `0xfffffb4d` |
| `ldrb` at 0x42380 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x42384 | store | 4 | `sp -0x2c` | `[sp-21]:1` |
| `ldrb` at 0x42388 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x42390 | store | 4 | `sp -0x30` | `[sp-21]:1+1` |
| `ldr` at 0x42394 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x42398 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x4239c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x423a0 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x423b0 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x423bc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x423c0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x423d4 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldrh` at 0x423dc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x423ec | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x423f0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x42404 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x4240c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x4241c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x42428 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x4242c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x42440 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldrh` at 0x42448 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x42458 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x4245c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x42470 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x42478 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x42488 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x4248c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x42494 | load | 4 | `[sp-52] +0x4` | `-` |
| `ldrb` at 0x4249c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x424a8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x424ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x424c0 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x424c8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x424d8 | store | 4 | `sp -0x24` | `ret(0x44324)` |
| `ldr` at 0x424dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x424e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x424f4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x424f8 | store | 4 | `sp -0x20` | `ret(0x44324)` |
| `ldr` at 0x424fc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x42500 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x42514 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x4251c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4252c | store | 4 | `sp -0x24` | `ret(0x43658)` |
| `ldr` at 0x42530 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4253c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x42548 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x4254c | store | 4 | `sp -0x20` | `ret(0x43658)` |
| `ldr` at 0x42550 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x42558 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x42560 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x42564 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x42568 | load | 4 | `sp -0x38` | `-` |
| `ldmdb` at 0x4256c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4256c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4256c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x42304 | `(unknown callee) 0x44048` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0x42318 | `(unknown callee) 0x440dc` | `[sp-20]` | `?` | `?` | `ret(0x44048)` |
| `bl` at 0x424d0 | `(unknown callee) 0x44324` | `?` | `?` | `?` | `?` |
| `bl` at 0x42524 | `(unknown callee) 0x43658` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x422dc | `(r1 & 0xff) != 0xff` | `(r1 & 0xff) == 0xff` |
| `bne` at 0x42310 | `ret(0x44048) != 0x0` | `ret(0x44048) == 0x0` |
| `bne` at 0x42344 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x42370 | `? > 0x1 (unsigned, 32-bit)` | `? <= 0x1 (unsigned, 32-bit)` |
| `blt` at 0x423a8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x423b8 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x423e8 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x42414 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x42424 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x42454 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x42480 | `Z == 0` | `not (Z == 0)` |
| `bls` at 0x424a4 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x424e4 | `ret(0x44324) == 0x0` | `ret(0x44324) != 0x0` |
| `bne` at 0x424f0 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x42538 | `ret(0x43658) == 0x0` | `ret(0x43658) != 0x0` |
| `bne` at 0x42544 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4256c | `r0` | `?` |

## `portDownInternal`

- `portDownInternal`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4257c to 0x42820.
  Established. The unnamed `0x44048` call carries `r0`, `(r1 & 0xff)`, unknown, `(r1 & 0xff)` (`bl`
  at 0x425fc); its zero-result fall-through includes the unnamed `0x440dc` call with `[sp-20]`,
  unknown, unknown, `ret(0x44048)` (`bne` at 0x42608, `bl` at 0x42610). A later four-byte stack
  assignment at `sp -0x20` retains `ret(0x445f0)` (`str` at 0x427f0). Port-down effects and result
  interpretation remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4258c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x42594 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x4259c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x425a4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x425a8 | load | 4 | `abs 0x42814` | `-` |
| `ldr` at 0x425ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x425c0 | load | 4 | `abs 0x148d10` | `-` |
| `str` at 0x425c8 | store | 4 | `sp -0x34` | `?` |
| `ldrb` at 0x425cc | load | 1 | `sp -0x15` | `-` |
| `str` at 0x425dc | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x425e0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x425e4 | load | 4 | `[sp-52] +0x8` | `-` |
| `str` at 0x425e8 | store | 4 | `sp -0x30` | `[[sp-52]+8]` |
| `ldrb` at 0x425f0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x425f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4260c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x42618 | store | 4 | `sp -0x38` | `ret(0x440dc)` |
| `ldr` at 0x42620 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x42624 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x4262c | load | 4 | `[sp-52] +0x4` | `-` |
| `ldrb` at 0x42634 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x42640 | load | 4 | `abs 0x42818` | `-` |
| `str` at 0x42644 | store | 4 | `sp -0x38` | `0xfffffb4e` |
| `ldr` at 0x4264c | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x42650 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x42658 | load | 4 | `[sp-52] +0x4` | `-` |
| `ldrb` at 0x42660 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x4266c | load | 4 | `abs 0x4281c` | `-` |
| `str` at 0x42670 | store | 4 | `sp -0x38` | `0xfffffb4d` |
| `ldrb` at 0x42678 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x4267c | store | 4 | `sp -0x2c` | `[sp-21]:1` |
| `ldrb` at 0x42680 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x42688 | store | 4 | `sp -0x30` | `[sp-21]:1+1` |
| `ldr` at 0x4268c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x42690 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x42694 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x42698 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x426a8 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x426b4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x426b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x426cc | load | 4 | `[sp-52] +0x0` | `-` |
| `ldrh` at 0x426d4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x426e4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x426e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x426fc | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x42704 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x42714 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x42720 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x42724 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x42738 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldrh` at 0x42740 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x42750 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x42754 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x42768 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x42770 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x42780 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x42784 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4278c | load | 4 | `[sp-52] +0x4` | `-` |
| `ldrb` at 0x42794 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x427a0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x427a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x427b8 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x427c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x427d0 | store | 4 | `sp -0x24` | `ret(0x445f0)` |
| `ldr` at 0x427d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x427e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x427ec | load | 4 | `sp -0x24` | `-` |
| `str` at 0x427f0 | store | 4 | `sp -0x20` | `ret(0x445f0)` |
| `ldr` at 0x427f4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x427fc | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x42804 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x42808 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x4280c | load | 4 | `sp -0x38` | `-` |
| `ldmdb` at 0x42810 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x42810 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x42810 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x425fc | `(unknown callee) 0x44048` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0x42610 | `(unknown callee) 0x440dc` | `[sp-20]` | `?` | `?` | `ret(0x44048)` |
| `bl` at 0x427c8 | `(unknown callee) 0x445f0` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x425d4 | `(r1 & 0xff) != 0xff` | `(r1 & 0xff) == 0xff` |
| `bne` at 0x42608 | `ret(0x44048) != 0x0` | `ret(0x44048) == 0x0` |
| `bne` at 0x4263c | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x42668 | `? > 0x1 (unsigned, 32-bit)` | `? <= 0x1 (unsigned, 32-bit)` |
| `blt` at 0x426a0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x426b0 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x426e0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x4270c | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x4271c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x4274c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x42778 | `Z == 0` | `not (Z == 0)` |
| `bls` at 0x4279c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x427dc | `ret(0x445f0) == 0x0` | `ret(0x445f0) != 0x0` |
| `bne` at 0x427e8 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x42810 | `r0` | `?` |

## `portStatus`

- `portStatus`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x42820 to 0x42cc8.
  Established. The unnamed `0xd7530` call carries `[sp-24]`, `[[sp-36]]:1`, `[sp-32]`, `[[sp-36]]:1`
  (`bl` at 0x428bc); its nonzero-result fall-through includes a four-byte stack assignment at
  `sp -0x38` retaining `ret(0xd7530)` (`beq` at 0x428d0, `str` at 0x428d8). Later four-byte assignments
  at `[sp-32] +0x0` retain `[sp-68]` and `[sp-72]` (`str` at 0x42a58, `str` at 0x42aac). Status
  fields and reply encoding remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x42830 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x42838 | store | 4 | `sp -0x20` | `r2` |
| `strb` at 0x4283c | store | 1 | `sp -0x19` | `(r1 & 0xff)` |
| `strb` at 0x42844 | store | 1 | `sp -0x29` | `0x0` |
| `ldrb` at 0x42848 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x4284c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4285c | store | 4 | `sp -0x24` | `ret(0x44048)` |
| `ldr` at 0x42860 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4286c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x42878 | store | 4 | `sp -0x38` | `ret(0x440dc)` |
| `ldrb` at 0x42880 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x42884 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4289c | load | 4 | `abs 0x42cc0` | `-` |
| `str` at 0x428a0 | store | 4 | `sp -0x38` | `0xfffffb4e` |
| `ldr` at 0x428a8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x428ac | load | 1 | `[sp-36] +0x0` | `-` |
| `ldr` at 0x428b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x428b8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x428c4 | store | 4 | `sp -0x28` | `ret(0xd7530)` |
| `ldr` at 0x428c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x428d4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x428d8 | store | 4 | `sp -0x38` | `ret(0xd7530)` |
| `ldr` at 0x428e0 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x428e4 | load | 2 | `[sp-36] +0x4` | `-` |
| `ldr` at 0x428f4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x428f8 | store | 4 | `sp -0x3c` | `[sp-32]` |
| `ldr` at 0x428fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x42900 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x42904 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldrb` at 0x42908 | load | 1 | `[sp-36] +0x1` | `-` |
| `strb` at 0x42920 | store | 1 | `sp -0x3d` | `0x4` |
| `strb` at 0x4292c | store | 1 | `sp -0x3d` | `0x0` |
| `ldrb` at 0x42930 | load | 1 | `sp -0x3d` | `-` |
| `ldr` at 0x42934 | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0x42938 | store | 1 | `[sp-32] +0x0` | `([sp-61]:1 & 0xff)` |
| `ldr` at 0x42940 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x42944 | load | 1 | `[sp-36] +0x1` | `-` |
| `ldr` at 0x42950 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x42954 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x42958 | load | 1 | `[sp-32] +0x0` | `-` |
| `strb` at 0x42960 | store | 1 | `[sp-32] +0x0` | `?` |
| `ldr` at 0x42964 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x42968 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x4296c | load | 1 | `[sp-32] +0x0` | `-` |
| `strb` at 0x42974 | store | 1 | `[sp-32] +0x0` | `?` |
| `ldr` at 0x42978 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x4297c | load | 2 | `[sp-36] +0x4` | `-` |
| `ldr` at 0x4298c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x42990 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldrb` at 0x4299c | load | 1 | `sp -0x29` | `-` |
| `strb` at 0x429a4 | store | 1 | `sp -0x29` | `?` |
| `ldr` at 0x429a8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x429ac | load | 1 | `[sp-32] +0x0` | `-` |
| `ldrb` at 0x429b8 | load | 1 | `sp -0x29` | `-` |
| `strb` at 0x429c0 | store | 1 | `sp -0x29` | `?` |
| `ldr` at 0x429c4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x429c8 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldrb` at 0x429d4 | load | 1 | `sp -0x29` | `-` |
| `strb` at 0x429dc | store | 1 | `sp -0x29` | `?` |
| `ldr` at 0x429e0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x429e4 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldrb` at 0x429f0 | load | 1 | `sp -0x29` | `-` |
| `strb` at 0x429f8 | store | 1 | `sp -0x29` | `?` |
| `ldr` at 0x429fc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x42a00 | load | 1 | `sp -0x29` | `-` |
| `strb` at 0x42a04 | store | 1 | `[sp-32] +0x0` | `([sp-41]:1 & 0xff)` |
| `ldr` at 0x42a08 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x42a0c | load | 2 | `[sp-36] +0x4` | `-` |
| `ldr` at 0x42a1c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x42a20 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x42a24 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x42a34 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0x42a38 | store | 4 | `sp -0x44` | `[[sp-32]]` |
| `ldr` at 0x42a40 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x42a50 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x42a54 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x42a58 | store | 4 | `[sp-32] +0x0` | `[sp-68]` |
| `ldr` at 0x42a5c | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x42a60 | load | 2 | `[sp-36] +0x4` | `-` |
| `ldr` at 0x42a70 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x42a74 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x42a78 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x42a88 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0x42a8c | store | 4 | `sp -0x48` | `[[sp-32]]` |
| `ldr` at 0x42a94 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x42aa4 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x42aa8 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x42aac | store | 4 | `[sp-32] +0x0` | `[sp-72]` |
| `ldr` at 0x42ab0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x42ab4 | load | 1 | `[sp-36] +0x2` | `-` |
| `ldr` at 0x42abc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x42acc | store | 4 | `sp -0x28` | `ret(0xd7530)` |
| `ldr` at 0x42ad0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x42adc | load | 4 | `sp -0x28` | `-` |
| `str` at 0x42ae0 | store | 4 | `sp -0x38` | `ret(0xd7530)` |
| `ldr` at 0x42ae8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x42aec | load | 1 | `sp -0x29` | `-` |
| `ldrb` at 0x42af0 | load | 1 | `[sp-36] +0x3` | `-` |
| `strb` at 0x42af8 | store | 1 | `sp -0x29` | `?` |
| `ldr` at 0x42afc | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x42b00 | load | 2 | `[sp-36] +0x4` | `-` |
| `ldr` at 0x42b10 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x42b14 | load | 1 | `sp -0x29` | `-` |
| `ldr` at 0x42b24 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0x42b28 | store | 4 | `sp -0x4c` | `[[sp-32]]` |
| `ldr` at 0x42b30 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x42b40 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x42b44 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x42b48 | store | 4 | `[sp-32] +0x0` | `[sp-76]` |
| `ldr` at 0x42b50 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x42b54 | load | 1 | `sp -0x29` | `-` |
| `ldr` at 0x42b64 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0x42b68 | store | 4 | `sp -0x50` | `[[sp-32]]` |
| `ldr` at 0x42b70 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x42b80 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x42b84 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x42b88 | store | 4 | `[sp-32] +0x0` | `[sp-80]` |
| `ldr` at 0x42b8c | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x42b90 | load | 2 | `[sp-36] +0x4` | `-` |
| `ldr` at 0x42ba0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x42ba4 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x42bb8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x42bbc | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x42bd0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x42bd4 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x42bf4 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x42bf8 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x42c04 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x42c10 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x42c20 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x42c34 | store | 4 | `sp -0x34` | `0x2` |
| `str` at 0x42c40 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x42c48 | load | 4 | `abs 0x42cc4` | `-` |
| `ldr` at 0x42c58 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x42c64 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x42c80 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x42c80 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x42c98 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x42c9c | load | 1 | `[sp-32] +0x0` | `-` |
| `strb` at 0x42ca4 | store | 1 | `[sp-32] +0x0` | `?` |
| `str` at 0x42cac | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x42cb0 | load | 4 | `sp -0x38` | `-` |
| `ldmdb` at 0x42cb4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x42cb4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x42cb4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x42cb4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x42854 | `(unknown callee) 0x44048` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x42870 | `(unknown callee) 0x440dc` | `[sp-24]` | `?` | `?` | `ret(0x44048)` |
| `bl` at 0x4288c | `(unknown callee) 0x41f08` | `[sp-24]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0x428bc | `(unknown callee) 0xd7530` | `[sp-24]` | `[[sp-36]]:1` | `[sp-32]` | `[[sp-36]]:1` |
| `bl` at 0x42ac4 | `(unknown callee) 0xd7530` | `[sp-24]` | `[[sp-36]+2]:1` | `sp-41` | `[[sp-36]+2]:1` |
| `bl` at 0x42be8 | `(unknown callee) 0x4f4d0` | `[[sp-32]]` | `[sp-32]` | `?` | `?` |
| `bl` at 0x42c4c | `(unknown callee) 0x202d0` | `0x1097ac` | `?` | `?` | `[sp-84]` |
| `bl` at 0x42c60 | `(unknown callee) 0x400fc` | `0x0` | `?` | `0x0` | `sp-48` |
| `bl` at 0x42c68 | `(unknown callee) 0xfb644` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0x42c84 | `(unknown callee) 0xfbb5c` | `ret(0xfb644)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x42868 | `ret(0x44048) != 0x0` | `ret(0x44048) == 0x0` |
| `bne` at 0x42898 | `ret(0x41f08) != 0x0` | `ret(0x41f08) == 0x0` |
| `beq` at 0x428d0 | `ret(0xd7530) == 0x0` | `ret(0xd7530) != 0x0` |
| `beq` at 0x428f0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x42918 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x4294c | `[[sp-36]+1]:1 == 0x0` | `[[sp-36]+1]:1 != 0x0` |
| `beq` at 0x42988 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x42a18 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x42a6c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x42ad8 | `ret(0xd7530) == 0x0` | `ret(0xd7530) != 0x0` |
| `beq` at 0x42b0c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x42b9c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x42bb4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x42bcc | `? == 0x0` | `? != 0x0` |
| `bne` at 0x42be4 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x42c00 | `[sp-84] == 0x3d` | `[sp-84] != 0x3d` |
| `bgt` at 0x42c0c | `[sp-84] > 0x3d (signed, 32-bit)` | `[sp-84] <= 0x3d (signed, 32-bit)` |
| `beq` at 0x42c18 | `[sp-84] == 0x23` | `[sp-84] != 0x23` |
| `beq` at 0x42c28 | `[sp-84] == 0x4f` | `[sp-84] != 0x4f` |
| `blt` at 0x42c90 | `ret(0xfbb5c) < 0x0 (signed, 32-bit)` | `ret(0xfbb5c) >= 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x42cb4 | `r0` | `?` |

## `portName`

- `portName`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x42cc8 to 0x42d40.
  Established. The unnamed `0x44048` call has `r0`, `(r1 & 0xff)`, `r2`, `(r1 & 0xff)` (`bl` at
  0x42cf0); its zero-result branch takes the route to `0x42d28` (`beq` at 0x42d04). That route
  includes a four-byte stack assignment of `0x1097c8` at `sp -0x20` (`str` at 0x42d2c). Port naming
  and the fallback value meaning remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x42cd8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x42ce0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x42ce4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x42ce8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x42cf8 | store | 4 | `sp -0x1c` | `ret(0x44048)` |
| `ldr` at 0x42cfc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x42d08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x42d0c | load | 4 | `ret(0x44048) +0x8` | `-` |
| `ldr` at 0x42d18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x42d1c | load | 4 | `ret(0x44048) +0x8` | `-` |
| `str` at 0x42d20 | store | 4 | `sp -0x20` | `[ret(0x44048)+8]` |
| `ldr` at 0x42d28 | load | 4 | `abs 0x42d3c` | `-` |
| `str` at 0x42d2c | store | 4 | `sp -0x20` | `0x1097c8` |
| `ldr` at 0x42d30 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x42d38 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x42d38 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x42d38 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x42cf0 | `(unknown callee) 0x44048` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x42d04 | `ret(0x44048) == 0x0` | `ret(0x44048) != 0x0` |
| `beq` at 0x42d14 | `[ret(0x44048)+8] == 0x0` | `[ret(0x44048)+8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x42d38 | `r0` | `?` |

## `portState`

- `portState`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x42d40 to 0x42e28.
  Established. Included four-byte absolute reads target `0x109814`, `0x109818`, `0x10981c` and
  `0x109820` (`ldm` at 0x42d8c). The branch takes unsigned `? > 0x5` to `0x42dd4` (`bhi` at
  0x42dcc). That route includes a four-byte stack assignment at `sp -0x38` of `0x1097c8` (`str` at
  0x42dd8). State encoding and lookup table meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x42d50 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x42d58 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x42d5c | load | 4 | `abs 0x42e1c` | `-` |
| `ldr` at 0x42d60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x42d74 | load | 4 | `abs 0x148d10` | `-` |
| `str` at 0x42d7c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x42d80 | load | 4 | `abs 0x42e20` | `-` |
| `ldm` at 0x42d8c | load | 4 | `abs 0x109814` | `-` |
| `ldm` at 0x42d8c | load | 4 | `abs 0x109818` | `-` |
| `ldm` at 0x42d8c | load | 4 | `abs 0x10981c` | `-` |
| `ldm` at 0x42d8c | load | 4 | `abs 0x109820` | `-` |
| `stm` at 0x42d90 | store | 4 | `sp -0x34` | `[0x109814]` |
| `stm` at 0x42d90 | store | 4 | `sp -0x30` | `[0x109818]` |
| `stm` at 0x42d90 | store | 4 | `sp -0x2c` | `[0x10981c]` |
| `stm` at 0x42d90 | store | 4 | `sp -0x28` | `[0x109820]` |
| `ldm` at 0x42d94 | load | 4 | `abs 0x109824` | `-` |
| `ldm` at 0x42d94 | load | 4 | `abs 0x109828` | `-` |
| `stm` at 0x42d98 | store | 4 | `sp -0x24` | `[0x109824]` |
| `stm` at 0x42d98 | store | 4 | `sp -0x20` | `[0x109828]` |
| `ldrb` at 0x42d9c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x42da0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x42da4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x42db0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x42db4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x42dbc | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x42dc4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x42dd4 | load | 4 | `abs 0x42e24` | `-` |
| `str` at 0x42dd8 | store | 4 | `sp -0x38` | `0x1097c8` |
| `ldr` at 0x42de0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x42de4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x42dec | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x42df4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x42e0c | load | 4 | `(unknown)` | `-` |
| `str` at 0x42e10 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x42e14 | load | 4 | `sp -0x38` | `-` |
| `ldmdb` at 0x42e18 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x42e18 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x42e18 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x42dac | `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)` |
| `bhi` at 0x42dcc | `? > 0x5 (unsigned, 32-bit)` | `? <= 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x42e18 | `r0` | `?` |

## `portInt`

- `portInt`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x431d8 to 0x43658.
  Established. Included four-byte stores with unknown effective targets retain `0x1` (`str` at
  0x43418, `str` at 0x43508). The unnamed `0x43808` calls carry `[sp-24]`, `[sp-32]`, unknown,
  unknown and `[sp-24]`, unknown, unknown, unknown respectively (`bl` at 0x43428, `bl` at 0x43518).
  The later unnamed `0x22e50` call has `0x148d18` and three unknown candidates (`bl` at 0x435d4).
  Interrupt effects and mask meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x431e8 | store | 4 | `sp -0x18` | `r0` |
| `strb` at 0x431f4 | store | 1 | `sp -0x19` | `(r1 & 0xff)` |
| `strb` at 0x431fc | store | 1 | `sp -0x1a` | `(r2 & 0xff)` |
| `strb` at 0x43204 | store | 1 | `sp -0x1b` | `(r3 & 0xff)` |
| `ldr` at 0x43208 | load | 4 | `abs 0x43614` | `-` |
| `str` at 0x43218 | store | 4 | `sp -0x2c` | `ret(0x22ac8)` |
| `ldr` at 0x4321c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x43228 | load | 4 | `abs 0x43618` | `-` |
| `str` at 0x4322c | store | 4 | `sp -0x44` | `0x10995c` |
| `ldr` at 0x43230 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x43234 | store | 4 | `sp -0x40` | `ret(0x22ac8)` |
| `ldr` at 0x4323c | load | 4 | `abs 0x4361c` | `-` |
| `ldr` at 0x43240 | load | 4 | `abs 0x43620` | `-` |
| `ldrb` at 0x4324c | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x43250 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x43260 | store | 4 | `sp -0x20` | `ret(0x4414c)` |
| `ldrb` at 0x43264 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x43268 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x43278 | store | 4 | `sp -0x24` | `ret(0x44238)` |
| `ldr` at 0x4327c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x43288 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x43294 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4329c | load | 4 | `abs 0x43624` | `-` |
| `ldr` at 0x432a0 | load | 4 | `abs 0x43628` | `-` |
| `str` at 0x432cc | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x432d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x432d8 | load | 4 | `abs 0x43624` | `-` |
| `ldr` at 0x432dc | load | 4 | `abs 0x43628` | `-` |
| `ldr` at 0x432e8 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x432f8 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x432fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4330c | load | 4 | `abs 0x4362c` | `-` |
| `str` at 0x43310 | store | 4 | `(unknown)` | `0x109980` |
| `str` at 0x43314 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `str` at 0x43318 | store | 4 | `(unknown)` | `ret(0xd7eec)` |
| `ldrb` at 0x4331c | load | 1 | `sp -0x1a` | `-` |
| `str` at 0x43320 | store | 4 | `(unknown)` | `[sp-26]:1` |
| `ldr` at 0x43328 | load | 4 | `abs 0x4361c` | `-` |
| `ldrb` at 0x43338 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0x4333c | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0x43340 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4334c | load | 4 | `abs 0x43630` | `-` |
| `ldr` at 0x43350 | load | 4 | `abs 0x14928c` | `-` |
| `ldr` at 0x4335c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x43364 | load | 4 | `abs 0x43634` | `-` |
| `ldr` at 0x43368 | load | 4 | `abs 0x43638` | `-` |
| `str` at 0x43394 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x43398 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x433a0 | load | 4 | `abs 0x43634` | `-` |
| `ldr` at 0x433a4 | load | 4 | `abs 0x43638` | `-` |
| `ldr` at 0x433b0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x433c0 | load | 4 | `abs 0x4363c` | `-` |
| `str` at 0x433c4 | store | 4 | `(unknown)` | `0x1099c8` |
| `str` at 0x433c8 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldrb` at 0x433cc | load | 1 | `sp -0x19` | `-` |
| `str` at 0x433d0 | store | 4 | `(unknown)` | `[sp-25]:1` |
| `ldrb` at 0x433d4 | load | 1 | `sp -0x1a` | `-` |
| `str` at 0x433d8 | store | 4 | `(unknown)` | `[sp-26]:1` |
| `ldrb` at 0x433dc | load | 1 | `sp -0x1b` | `-` |
| `str` at 0x433e0 | store | 4 | `(unknown)` | `[sp-27]:1` |
| `ldr` at 0x433e8 | load | 4 | `abs 0x4361c` | `-` |
| `ldr` at 0x433ec | load | 4 | `abs 0x43640` | `-` |
| `ldr` at 0x433f8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x43404 | load | 1 | `sp -0x1a` | `-` |
| `ldrb` at 0x4340c | load | 1 | `sp -0x1b` | `-` |
| `str` at 0x43418 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x4341c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x43420 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x43430 | store | 4 | `sp -0x28` | `ret(0x43808)` |
| `ldr` at 0x43434 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x43440 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x43448 | load | 4 | `abs 0x43624` | `-` |
| `ldr` at 0x4344c | load | 4 | `abs 0x43628` | `-` |
| `str` at 0x43478 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x4347c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x43484 | load | 4 | `abs 0x43624` | `-` |
| `ldr` at 0x43488 | load | 4 | `abs 0x43628` | `-` |
| `ldr` at 0x43494 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x434a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x434b0 | load | 4 | `abs 0x43644` | `-` |
| `str` at 0x434b4 | store | 4 | `(unknown)` | `0x1099f8` |
| `ldr` at 0x434b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x434bc | load | 4 | `[sp-32] +0x8` | `-` |
| `str` at 0x434c0 | store | 4 | `(unknown)` | `[[sp-32]+8]` |
| `str` at 0x434c4 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `str` at 0x434c8 | store | 4 | `(unknown)` | `ret(0x21a38)` |
| `ldr` at 0x434d0 | load | 4 | `abs 0x4361c` | `-` |
| `ldr` at 0x434d4 | load | 4 | `abs 0x43648` | `-` |
| `ldr` at 0x434e0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x434ec | load | 1 | `sp -0x1a` | `-` |
| `ldrb` at 0x434f8 | load | 1 | `sp -0x1b` | `-` |
| `str` at 0x43508 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x4350c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x43510 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x43520 | store | 4 | `sp -0x28` | `ret(0x43808)` |
| `ldr` at 0x43524 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x43530 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x43538 | load | 4 | `abs 0x43624` | `-` |
| `ldr` at 0x4353c | load | 4 | `abs 0x43628` | `-` |
| `str` at 0x43568 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x4356c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x43574 | load | 4 | `abs 0x43624` | `-` |
| `ldr` at 0x43578 | load | 4 | `abs 0x43628` | `-` |
| `ldr` at 0x43584 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x43594 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x435a0 | load | 4 | `abs 0x43644` | `-` |
| `str` at 0x435a4 | store | 4 | `(unknown)` | `0x1099f8` |
| `ldr` at 0x435a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x435ac | load | 4 | `[sp-36] +0x8` | `-` |
| `str` at 0x435b0 | store | 4 | `(unknown)` | `[[sp-36]+8]` |
| `str` at 0x435b4 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `str` at 0x435b8 | store | 4 | `(unknown)` | `ret(0x21a38)` |
| `ldr` at 0x435c0 | load | 4 | `abs 0x4361c` | `-` |
| `ldr` at 0x435c4 | load | 4 | `abs 0x4364c` | `-` |
| `ldr` at 0x435d0 | load | 4 | `abs 0x43614` | `-` |
| `str` at 0x435dc | store | 4 | `sp -0x2c` | `ret(0x22e50)` |
| `ldr` at 0x435e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x435ec | load | 4 | `abs 0x43650` | `-` |
| `str` at 0x435f0 | store | 4 | `(unknown)` | `0x109a14` |
| `ldr` at 0x435f4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x435f8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x43600 | load | 4 | `abs 0x4361c` | `-` |
| `ldr` at 0x43604 | load | 4 | `abs 0x43654` | `-` |
| `ldmdb` at 0x43610 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x43610 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x43610 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x43610 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x43210 | `(unknown callee) 0x22ac8` | `0x148d18` | `?` | `r2` | `r3` |
| `bl` at 0x43248 | `(unknown callee) 0x21eb8` | `0x9` | `0x109600` | `0x2ef` | `0x0` |
| `bl` at 0x43258 | `(unknown callee) 0x4414c` | `[sp-24]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0x43270 | `(unknown callee) 0x44238` | `[sp-24]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0x432a4 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x10964c` | `0x109654` |
| `bl` at 0x432b0 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x432e0 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x10964c` | `0x109654` |
| `bl` at 0x432f0 | `(unknown callee) 0x1b598` | `[sp-48]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x43304 | `(unknown callee) 0xd7eec` | `[sp-24]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0x43334 | `(unknown callee) 0x21eb8` | `0x1` | `0x109600` | `0x2f8` | `0x0` |
| `bl` at 0x43348 | `(unknown callee) 0xd768c` | `[sp-24]` | `[sp-25]:1` | `[sp-26]:1` | `[sp-25]:1` |
| `bl` at 0x4336c | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x1099bc` | `0x1099c4` |
| `bl` at 0x43378 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x433a8 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x1099bc` | `0x1099c4` |
| `bl` at 0x433b8 | `(unknown callee) 0x1b598` | `[sp-48]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x433f4 | `(unknown callee) 0x21eb8` | `0x6` | `0x109600` | `0x2ff` | `0x0` |
| `bl` at 0x43428 | `(unknown callee) 0x43808` | `[sp-24]` | `[sp-32]` | `?` | `?` |
| `bl` at 0x43450 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x10964c` | `0x109654` |
| `bl` at 0x4345c | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x4348c | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x10964c` | `0x109654` |
| `bl` at 0x4349c | `(unknown callee) 0x1b598` | `[sp-48]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x434a8 | `(unknown callee) 0x21a38` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x434dc | `(unknown callee) 0x21eb8` | `0x3` | `0x109600` | `0x307` | `0x0` |
| `bl` at 0x43518 | `(unknown callee) 0x43808` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x43540 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x10964c` | `0x109654` |
| `bl` at 0x4354c | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x4357c | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x10964c` | `0x109654` |
| `bl` at 0x4358c | `(unknown callee) 0x1b598` | `[sp-48]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x43598 | `(unknown callee) 0x21a38` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x435cc | `(unknown callee) 0x21eb8` | `0x3` | `0x109600` | `0x30d` | `0x0` |
| `bl` at 0x435d4 | `(unknown callee) 0x22e50` | `0x148d18` | `?` | `?` | `?` |
| `bl` at 0x4360c | `(unknown callee) 0x21eb8` | `0x9` | `0x109600` | `0x315` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x43224 | `ret(0x22ac8) == 0x0` | `ret(0x22ac8) != 0x0` |
| `bne` at 0x43284 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0x43290 | `ret(0x44238) != 0x0` | `ret(0x44238) == 0x0` |
| `beq` at 0x43358 | `[0x14928c] == 0x0` | `[0x14928c] != 0x0` |
| `beq` at 0x43400 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `beq` at 0x4343c | `ret(0x43808) == 0x0` | `ret(0x43808) != 0x0` |
| `beq` at 0x434e8 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x4352c | `ret(0x43808) == 0x0` | `ret(0x43808) != 0x0` |
| `beq` at 0x435e8 | `ret(0x22e50) == 0x0` | `ret(0x22e50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x43610 | `r0` | `?` |
