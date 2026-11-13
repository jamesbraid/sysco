# ARM router registration and queue state in 1.40.5

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
The selected spans have no unresolved computed transfers. Numeric callee names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/router-registration-facts.txt) retain all tool sections.

## `EvtIRouter`

- `EvtIRouter`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x69714 to 0x69808.
  Established. A byte assignment at `r0 +0x16` writes `0x2` (`strb` at 0x697c0). The branch takes
  `[r0+22]:1 != 0x0` to `0x697fc` and falls through for equality (`bne` at 0x697a0). The unnamed
  `0x73278` call carries `[sp-20]`, `r1`, `0x2`, `[sp-28]` (`bl` at 0x697ec). Event payload and
  endpoint meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x69724 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x69728 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6972c | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x69730 | store | 4 | `sp -0x18` | `[r0+28]` |
| `ldr` at 0x69734 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x69740 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x69748 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x6974c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x69754 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x6975c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x69764 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x69768 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x6976c | store | 4 | `sp -0x1c` | `[sp-40]` |
| `ldr` at 0x69770 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x69780 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x69784 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x69790 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x69794 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x69798 | load | 1 | `r0 +0x16` | `-` |
| `ldr` at 0x697a4 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x697a8 | load | 2 | `r0 +0x22` | `-` |
| `ldr` at 0x697b8 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x697c0 | store | 1 | `r0 +0x16` | `0x2` |
| `ldr` at 0x697c4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x697c8 | store | 4 | `sp -0x2c` | `[sp-28]` |
| `ldr` at 0x697cc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x697d8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x697e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x697f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x697fc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x69804 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x69804 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x69804 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x697ec | `(unknown callee) 0x73278` | `[sp-20]` | `r1` | `0x2` | `[sp-28]` |
| `bl` at 0x697f8 | `(unknown callee) 0x86094` | `[sp-20]` | `r1` | `0x2` | `[sp-28]` |
| `bl` at 0x69800 | `(unknown callee) 0xb5fec` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x6973c | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `bne` at 0x697a0 | `[r0+22]:1 != 0x0` | `[r0+22]:1 == 0x0` |
| `beq` at 0x697b0 | `[r0+34]:2 == 0x0` | `[r0+34]:2 != 0x0` |
| `beq` at 0x697d4 | `[sp-28] == 0x4` | `[sp-28] != 0x4` |
| `beq` at 0x697e0 | `[sp-28] == 0xe` | `[sp-28] != 0xe` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x69804 | `r0` | `ret(0xb5fec)` |

## `RouteIRouter`

- `RouteIRouter`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb1a6c to 0xb1c34.
  Established. Included byte stores with unknown effective targets write `0x0` (`strb` at 0xb1ad4,
  `strb` at 0xb1af4). The unnamed `0xb785c` call has `0x2b28e0`, `sp-24`, `0x0`, `0x1` (`bl` at
  0xb1bd8); its nonzero result takes the branch to `0xb1bfc` (`bne` at 0xb1bec). A four-byte
  assignment at `ret(0xb785c) +0xc` retains `[[sp-20]+12]` (`str` at 0xb1c08). Routing fields and
  loop bounds remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb1a7c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb1a80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb1a84 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0xb1a88 | store | 4 | `sp -0x18` | `[r0+28]` |
| `str` at 0xb1a90 | store | 4 | `sp -0x220` | `0x0` |
| `ldr` at 0xb1a94 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb1a98 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb1a9c | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0xb1aa8 | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0xb1ab8 | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0xb1abc | load | 4 | `abs 0xb1c24` | `-` |
| `strb` at 0xb1ad4 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xb1ad8 | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0xb1adc | load | 4 | `abs 0xb1c24` | `-` |
| `strb` at 0xb1af4 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xb1af8 | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0xb1afc | load | 4 | `abs 0xb1c28` | `-` |
| `str` at 0xb1b10 | store | 4 | `sp -0x224` | `?` |
| `ldr` at 0xb1b14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb1b18 | load | 4 | `sp -0x220` | `-` |
| `ldrb` at 0xb1b2c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb1b3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb1b40 | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0xb1b54 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb1b58 | store | 4 | `sp -0x228` | `?` |
| `ldr` at 0xb1b60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb1b64 | load | 4 | `sp -0x220` | `-` |
| `str` at 0xb1b6c | store | 4 | `sp -0x228` | `ret(0xb823c)` |
| `ldr` at 0xb1b70 | load | 4 | `sp -0x228` | `-` |
| `ldr` at 0xb1b74 | load | 4 | `sp -0x224` | `-` |
| `str` at 0xb1b78 | store | 4 | `[sp-548] +0x0` | `?` |
| `ldr` at 0xb1b7c | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0xb1b80 | load | 4 | `abs 0xb1c24` | `-` |
| `strh` at 0xb1b98 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0xb1b9c | load | 4 | `sp -0x220` | `-` |
| `str` at 0xb1ba4 | store | 4 | `sp -0x220` | `[sp-544]+1` |
| `str` at 0xb1bb4 | store | 4 | `sp -0x234` | `0x2` |
| `ldr` at 0xb1bb8 | load | 4 | `sp -0x220` | `-` |
| `str` at 0xb1bbc | store | 4 | `sp -0x230` | `?` |
| `str` at 0xb1bc4 | store | 4 | `sp -0x22c` | `sp-540` |
| `ldr` at 0xb1bc8 | load | 4 | `abs 0xb1c2c` | `-` |
| `str` at 0xb1be0 | store | 4 | `sp -0x1c` | `ret(0xb785c)` |
| `ldr` at 0xb1be4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1bf0 | load | 4 | `abs 0xb1c30` | `-` |
| `ldr` at 0xb1bfc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1c00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb1c04 | load | 4 | `[sp-20] +0xc` | `-` |
| `str` at 0xb1c08 | store | 4 | `ret(0xb785c) +0xc` | `[[sp-20]+12]` |
| `ldr` at 0xb1c0c | load | 4 | `abs 0xb1c2c` | `-` |
| `ldr` at 0xb1c10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1c18 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb1c20 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb1c20 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb1c20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb1b68 | `(unknown callee) 0xb823c` | `?` | `?` | `0x24` | `?` |
| `bl` at 0xb1bd8 | `(unknown callee) 0xb785c` | `0x2b28e0` | `sp-24` | `0x0` | `0x1` |
| `bl` at 0xb1bf4 | `(unknown callee) 0x202d0` | `0x1282a0` | `?` | `?` | `ret(0xb785c)` |
| `bl` at 0xb1c14 | `(unknown callee) 0xb7090` | `0x2b28e0` | `[sp-28]` | `[[sp-20]+12]` | `ret(0xb785c)` |
| `bl` at 0xb1c1c | `(unknown callee) 0xb5fec` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xb1aa4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `ble` at 0xb1ab0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xb1b38 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xb1bec | `ret(0xb785c) != 0x0` | `ret(0xb785c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb1c20 | `r0` | `ret(0xb5fec)` |

## `IRouterRegisterFunction`

- `IRouterRegisterFunction`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb4a70 to 0xb4b18.
  Established. The unnamed `0xb4494` call carries `r0`, `r1`, `0x1`, `0x0` (`bl` at 0xb4ac0). A
  later call to that target has unknown, unknown, `0x2`, `0x0` (`bl` at 0xb4b00). Registration
  effects and the reported unknown return remain Unknown (`ldmdb` at 0xb4b14).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb4a80 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb4a84 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb4a88 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb4a8c | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xb4a94 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb4a98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4aa8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb4aac | store | 4 | `sp -0x28` | `r3` |
| `ldr` at 0xb4ab0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb4ab4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb4ac8 | store | 4 | `sp -0x24` | `ret(0xb4494)` |
| `ldr` at 0xb4acc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4ad8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4ae8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb4aec | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb4af0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb4af4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb4b08 | store | 4 | `sp -0x24` | `ret(0xb4494)` |
| `ldr` at 0xb4b0c | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xb4b14 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb4b14 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb4b14 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb4ac0 | `(unknown callee) 0xb4494` | `r0` | `r1` | `0x1` | `0x0` |
| `bl` at 0xb4b00 | `(unknown callee) 0xb4494` | `?` | `?` | `0x2` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb4aa4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xb4ad4 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xb4ae4 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb4b14 | `r0` | `?` |

## `IRouterRegisterQueue`

- `IRouterRegisterQueue`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb4b18 to 0xb4bc0.
  Established. Two calls to unnamed `0xb4494` use `r0`, `r1`, `0x1`, `0x1`, then unknown, unknown,
  `0x2`, `0x1` respectively (`bl` at 0xb4b68, `bl` at 0xb4ba8). Their selector meanings remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb4b28 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb4b2c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb4b30 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb4b34 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xb4b3c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb4b40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4b50 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb4b54 | store | 4 | `sp -0x28` | `r3` |
| `ldr` at 0xb4b58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb4b5c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb4b70 | store | 4 | `sp -0x24` | `ret(0xb4494)` |
| `ldr` at 0xb4b74 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4b80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4b90 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb4b94 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb4b98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb4b9c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb4bb0 | store | 4 | `sp -0x24` | `ret(0xb4494)` |
| `ldr` at 0xb4bb4 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xb4bbc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb4bbc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb4bbc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb4b68 | `(unknown callee) 0xb4494` | `r0` | `r1` | `0x1` | `0x1` |
| `bl` at 0xb4ba8 | `(unknown callee) 0xb4494` | `?` | `?` | `0x2` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb4b4c | `? == 0x0` | `? != 0x0` |
| `bne` at 0xb4b7c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xb4b8c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb4bbc | `r0` | `?` |

## `IRouterEventRegisterFunction`

- `IRouterEventRegisterFunction`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb4efc to 0xb4f44.
  Established. A four-byte stack assignment at `sp -0x24` preserves entry `r3` (`str` at 0xb4f20).
  The unnamed `0xb4bc0` call carries `r0`, `r1`, `r2`, `0x0` (`bl` at 0xb4f34); its result is the
  reported return (`ldmdb` at 0xb4f40). Callback semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb4f0c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb4f10 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb4f14 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb4f18 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xb4f1c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb4f20 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0xb4f24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb4f28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb4f2c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xb4f40 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb4f40 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb4f40 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb4f34 | `(unknown callee) 0xb4bc0` | `r0` | `r1` | `r2` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb4f40 | `r0` | `ret(0xb4bc0)` |

## `IRouterEventRegisterQueue`

- `IRouterEventRegisterQueue`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb4f44 to 0xb4f8c.
  Established. Entry `r3` is retained by a four-byte stack assignment at `sp -0x24` (`str` at
  0xb4f68). The unnamed `0xb4bc0` call has `r0`, `r1`, `r2`, `0x1` (`bl` at 0xb4f7c). Queue
  registration behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb4f54 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb4f58 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb4f5c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb4f60 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xb4f64 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb4f68 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0xb4f6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb4f70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb4f74 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xb4f88 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb4f88 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb4f88 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb4f7c | `(unknown callee) 0xb4bc0` | `r0` | `r1` | `r2` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb4f88 | `r0` | `ret(0xb4bc0)` |

## `IRouterInit`

- `IRouterInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb5d64 to 0xb5e34.
  Established. Included four-byte assignments at absolute `0x14a85c` write `0x1` and `0x2` on
  separate routes (`str` at 0xb5d94, `str` at 0xb5da4). The deciding branch takes `[sp-21]:1 == 0x0`
  to `0xb5d9c` and falls through for inequality (`beq` at 0xb5d88). The unnamed `0xb5e34` call has
  `0x2b2a60`, `0x1`, `0x3`, unknown (`bl` at 0xb5de0). Router mode and address encoding remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb5d74 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb5d7c | store | 4 | `sp -0x18` | `0x1` |
| `ldrb` at 0xb5d80 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xb5d8c | load | 4 | `abs 0xb5e18` | `-` |
| `str` at 0xb5d94 | store | 4 | `abs 0x14a85c` | `0x1` |
| `ldr` at 0xb5d9c | load | 4 | `abs 0xb5e18` | `-` |
| `str` at 0xb5da4 | store | 4 | `abs 0x14a85c` | `0x2` |
| `str` at 0xb5dac | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xb5db0 | load | 4 | `abs 0xb5e1c` | `-` |
| `ldr` at 0xb5db4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb5dd0 | store | 4 | `abs 0x2b2a80` | `?` |
| `ldr` at 0xb5dd4 | load | 4 | `abs 0xb5e20` | `-` |
| `ldr` at 0xb5de4 | load | 4 | `abs 0xb5e24` | `-` |
| `ldr` at 0xb5de8 | load | 4 | `abs 0xb5e28` | `-` |
| `ldr` at 0xb5df8 | load | 4 | `abs 0xb5e2c` | `-` |
| `ldr` at 0xb5dfc | load | 4 | `abs 0xb5e30` | `-` |
| `ldr` at 0xb5e0c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xb5e14 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb5e14 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb5e14 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb5de0 | `(unknown callee) 0xb5e34` | `0x2b2a60` | `0x1` | `0x3` | `?` |
| `bl` at 0xb5df4 | `(unknown callee) 0x229b0` | `0x14a880` | `0x128cf0` | `0x1` | `0x9` |
| `bl` at 0xb5e08 | `(unknown callee) 0x229b0` | `0x14a8d4` | `0x128cfc` | `0x1` | `0x9` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb5d88 | `[sp-21]:1 == 0x0` | `[sp-21]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb5e14 | `r0` | `[sp-24]` |

## `IRouterInitID`

- `IRouterInitID`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb5e34 to 0xb5f1c.
  Established. The branch takes signed `[[sp-20]] >= 0x0` to `0xb5ec4` and falls through for a
  negative value (`bge` at 0xb5e94). A byte assignment at `[sp-20] +0x4` retains `([sp-28]:1 & 0xff)` (`strb` at 0xb5efc). A halfword assignment at `[sp-20] +0x6` writes `0x0` (`strh` at
  0xb5f08). Identifier layout remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb5e44 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb5e48 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb5e4c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xb5e50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5e54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5e58 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0xb5e64 | load | 4 | `abs 0xb5f18` | `-` |
| `ldr` at 0xb5e68 | load | 4 | `abs 0x2b2a80` | `-` |
| `str` at 0xb5e7c | store | 4 | `r0 +0x0` | `?` |
| `ldr` at 0xb5e80 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb5e84 | store | 4 | `sp -0x20` | `[sp-20]` |
| `ldr` at 0xb5e88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5e8c | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xb5e98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5e9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5ea0 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xb5ea8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb5eb8 | store | 4 | `[sp-20] +0x0` | `?` |
| `str` at 0xb5ebc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb5ec4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5ec8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5ecc | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xb5ed4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb5ee0 | store | 4 | `[sp-20] +0x0` | `?` |
| `str` at 0xb5ee4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb5ee8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb5eec | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb5ef0 | store | 4 | `[sp-32] +0x0` | `[sp-36]` |
| `ldr` at 0xb5ef4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb5ef8 | load | 1 | `sp -0x1c` | `-` |
| `strb` at 0xb5efc | store | 1 | `[sp-20] +0x4` | `([sp-28]:1 & 0xff)` |
| `ldr` at 0xb5f00 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xb5f08 | store | 2 | `[sp-20] +0x6` | `0x0` |
| `ldmdb` at 0xb5f14 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb5f14 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb5f14 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xb5e94 | `[[sp-20]] >= 0x0 (signed, 32-bit)` | `[[sp-20]] < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb5f14 | `r0` | `0x0` |

## `IRouterSetAuthority`

- `IRouterSetAuthority`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb5f1c to 0xb5f4c.
  Established. A byte assignment at `r0 +0x4` writes `([sp-24]:1 & 0xff)` (`strb` at 0xb5f3c). The
  reported result is `0x0` (`ldmdb` at 0xb5f48). Authority encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb5f2c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb5f30 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xb5f34 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb5f38 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xb5f3c | store | 1 | `r0 +0x4` | `([sp-24]:1 & 0xff)` |
| `ldmdb` at 0xb5f48 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb5f48 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb5f48 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb5f48 | `r0` | `0x0` |

## `IRouterCreateQueue`

- `IRouterCreateQueue`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb5f4c to 0xb5f90.
  Established. The four-byte stack assignment at `sp -0x20` writes `0x1` (`str` at 0xb5f6c). The
  unnamed `0x23014` call carries `r0`, `r1`, `r2`, `0x7` (`bl` at 0xb5f80). Queue options and the
  callee effect remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb5f5c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb5f60 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb5f64 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb5f6c | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xb5f70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5f74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb5f78 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xb5f8c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb5f8c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb5f8c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb5f80 | `(unknown callee) 0x23014` | `r0` | `r1` | `r2` | `0x7` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb5f8c | `r0` | `ret(0x23014)` |

## `IRouterGetPacket`

- `IRouterGetPacket`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb5f90 to 0xb5fec.
  Established. The unnamed `0x233d0` call has `r1`, `r0`, `r2`, `r3` (`bl` at 0xb5fb8); its zero
  result takes the branch to `0xb5fdc`, while nonzero falls through (`beq` at 0xb5fcc). The reported
  return is `[sp-40]` (`ldmdb` at 0xb5fe8). Packet ownership and error semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb5fa0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb5fa4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb5fa8 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xb5fac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb5fb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5fb4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb5fc0 | store | 4 | `sp -0x24` | `ret(0x233d0)` |
| `ldr` at 0xb5fc4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb5fd0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb5fd4 | store | 4 | `sp -0x28` | `ret(0x233d0)` |
| `str` at 0xb5fe0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xb5fe4 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xb5fe8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb5fe8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb5fe8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb5fb8 | `(unknown callee) 0x233d0` | `r1` | `r0` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb5fcc | `ret(0x233d0) == 0x0` | `ret(0x233d0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb5fe8 | `r0` | `[sp-40]` |

## `IRouterSetQueueSuspendTime`

- `IRouterSetQueueSuspendTime`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb63b8 to 0xb63f4.
  Established. Four-byte absolute assignments at `0x1436b8` and `0x1436bc` retain entry `r0` and
  `[sp-24]` respectively (`str` at 0xb63d8, `str` at 0xb63e4). Timing units and suspension behavior
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb63c8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb63cc | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xb63d0 | load | 4 | `abs 0xb63ec` | `-` |
| `ldr` at 0xb63d4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb63d8 | store | 4 | `abs 0x1436b8` | `r0` |
| `ldr` at 0xb63dc | load | 4 | `abs 0xb63f0` | `-` |
| `ldr` at 0xb63e0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb63e4 | store | 4 | `abs 0x1436bc` | `[sp-24]` |
| `ldmdb` at 0xb63e8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb63e8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb63e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb63e8 | `r0` | `r0` |

## `IRouterGetQueueSuspendTime`

- `IRouterGetQueueSuspendTime`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb63f4 to 0xb6438.
  Established. The four-byte assignment at `r0 +0x0` retains `[0x1436b8]` (`str` at 0xb6418).
  Another at `[sp-24] +0x0` retains `[0x1436bc]` (`str` at 0xb6428). The reported result preserves
  entry `r0` (`ldmdb` at 0xb642c). Output units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb6404 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb6408 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xb640c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb6410 | load | 4 | `abs 0xb6430` | `-` |
| `ldr` at 0xb6414 | load | 4 | `abs 0x1436b8` | `-` |
| `str` at 0xb6418 | store | 4 | `r0 +0x0` | `[0x1436b8]` |
| `ldr` at 0xb641c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb6420 | load | 4 | `abs 0xb6434` | `-` |
| `ldr` at 0xb6424 | load | 4 | `abs 0x1436bc` | `-` |
| `str` at 0xb6428 | store | 4 | `[sp-24] +0x0` | `[0x1436bc]` |
| `ldmdb` at 0xb642c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb642c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb642c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb642c | `r0` | `r0` |

## `B2BActive`

- `B2BActive`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7a658 to 0x7a700.
  Established. Included four-byte stack assignments at `sp -0x18` write `0x0` and `0x1` (`str` at
  0x7a670, `str` at 0x7a6ec). The intervening four-byte read has an unknown effective target (`ldr`
  at 0x7a68c). The first conditional branch takes an unknown value equal to zero to `0x7a6f0` and
  falls through for nonzero (`beq` at 0x7a694). Active-state meaning and the final result remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7a668 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x7a670 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x7a674 | load | 4 | `abs 0x7a6fc` | `-` |
| `ldr` at 0x7a678 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a68c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7a698 | load | 4 | `abs 0x7a6fc` | `-` |
| `ldr` at 0x7a69c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a6b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7a6b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7a6c0 | load | 4 | `abs 0x7a6fc` | `-` |
| `ldr` at 0x7a6c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a6d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7a6dc | load | 4 | `(unknown)` | `-` |
| `str` at 0x7a6ec | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0x7a6f0 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x7a6f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x7a6f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x7a6f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x7a694 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7a6bc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7a6e4 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x7a6f8 | `r0` | `?` |

## `B2BConfigured`

- `B2BConfigured`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7a700 to 0x7a748.
  Established. A four-byte read has an unknown effective target (`ldr` at 0x7a72c). The reported
  return is unknown (`ldmdb` at 0x7a740). Configuration representation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7a710 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x7a714 | load | 4 | `abs 0x7a744` | `-` |
| `ldr` at 0x7a718 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a72c | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x7a740 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x7a740 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x7a740 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x7a740 | `r0` | `?` |
