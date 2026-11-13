# ARM MMR, PIO and link status routines in 1.40.5

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
JChip_read_dmin_dmax, JChip_read_link_state and JChip_link_summary_f have unresolved computed transfers (`ldrls` at 0xf4114, `ldrls` at 0xf43b8, `ldrls` at 0xf4824). Their tables and returns cover known edges only. Indirect call targets and numeric callee names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/mmr-pio-link-facts.txt) retain all tool sections.

## `JChip_MMR_Read`

- `JChip_MMR_Read`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeae04 to 0xeaf08.
  Established. Included byte reads target `[sp-36] +0x19` and `[sp-36] +0x21` (`ldrb` at 0xeae78,
  `ldrsb` at 0xeae80). The unknown-target call has `[[sp-36]+25]:1`, unknown, `0x5`, `[sp-24]`
  (`mov` at 0xeaec4); its result equal to `0x1` takes the branch to `0xeaee0` (`beq` at 0xeaed0).
  MMR read addressing and callback effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeae14 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xeae18 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xeae1c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xeae20 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xeae24 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xeae30 | store | 4 | `sp -0x24` | `ret(0xebc18)` |
| `ldr` at 0xeae34 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeae40 | load | 4 | `abs 0xeaef0` | `-` |
| `str` at 0xeae44 | store | 4 | `sp -0x28` | `0xfffff7c6` |
| `ldr` at 0xeae4c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeae60 | load | 4 | `abs 0xeaef4` | `-` |
| `str` at 0xeae64 | store | 4 | `sp -0x28` | `0xfffff7c5` |
| `ldr` at 0xeae6c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeae70 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xeae74 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xeae78 | load | 1 | `[sp-36] +0x19` | `-` |
| `ldr` at 0xeae7c | load | 4 | `sp -0x24` | `-` |
| `ldrsb` at 0xeae80 | load | 1 | `[sp-36] +0x21` | `-` |
| `ldr` at 0xeae84 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xeae88 | store | 4 | `sp -0x3c` | `[sp-28]` |
| `ldr` at 0xeae8c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xeae90 | store | 4 | `sp -0x38` | `[sp-32]` |
| `ldr` at 0xeae94 | load | 4 | `abs 0xeaef8` | `-` |
| `str` at 0xeae98 | store | 4 | `sp -0x34` | `0x2ae008` |
| `ldr` at 0xeae9c | load | 4 | `abs 0xeaefc` | `-` |
| `str` at 0xeaea0 | store | 4 | `sp -0x30` | `0x2ae088` |
| `ldr` at 0xeaea4 | load | 4 | `abs 0xeaf00` | `-` |
| `str` at 0xeaea8 | store | 4 | `sp -0x2c` | `0x2b47f8` |
| `ldr` at 0xeaeac | load | 4 | `[[sp-36]] +0x14` | `-` |
| `ldr` at 0xeaebc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeaed4 | load | 4 | `abs 0xeaf04` | `-` |
| `str` at 0xeaed8 | store | 4 | `sp -0x28` | `0xfffff7c4` |
| `str` at 0xeaee4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xeaee8 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xeaeec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xeaeec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xeaeec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeae28 | `(unknown callee) 0xebc18` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xeae50 | `(unknown callee) 0xe9a1c` | `ret(0xebc18)` | `?` | `?` | `ret(0xebc18)` |
| `mov` at 0xeaec4 | `(unknown callee) (unknown target)` | `[[sp-36]+25]:1` | `?` | `0x5` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xeae3c | `ret(0xebc18) != 0x0` | `ret(0xebc18) == 0x0` |
| `beq` at 0xeae5c | `ret(0xe9a1c) == 0x0` | `ret(0xe9a1c) != 0x0` |
| `beq` at 0xeaed0 | `ret(unknown) == 0x1` | `ret(unknown) != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xeaeec | `r0` | `?` |

## `JChip_MMR_Write`

- `JChip_MMR_Write`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeaf08 to 0xeb014.
  Established. A four-byte read targets `[[sp-40]] +0x18` (`ldr` at 0xeafb4). The unknown-target
  call carries `[[sp-40]+25]:1`, unknown, `[0x143870]`, `[sp-28]` (`mov` at 0xeafcc); its result
  equal to `0x1` takes the branch to `0xeafe8` (`beq` at 0xeafd8). MMR write semantics and callback
  target remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeaf18 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xeaf1c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xeaf20 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xeaf24 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0xeaf28 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xeaf34 | store | 4 | `sp -0x28` | `ret(0xebc18)` |
| `ldr` at 0xeaf38 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeaf44 | load | 4 | `abs 0xeaff8` | `-` |
| `str` at 0xeaf48 | store | 4 | `sp -0x2c` | `0xfffff7c6` |
| `ldr` at 0xeaf50 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeaf64 | load | 4 | `abs 0xeaffc` | `-` |
| `str` at 0xeaf68 | store | 4 | `sp -0x2c` | `0xfffff7c5` |
| `ldr` at 0xeaf70 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeaf74 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0xeaf78 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xeaf7c | load | 1 | `[sp-40] +0x19` | `-` |
| `ldr` at 0xeaf80 | load | 4 | `sp -0x28` | `-` |
| `ldrsb` at 0xeaf84 | load | 1 | `[sp-40] +0x21` | `-` |
| `ldr` at 0xeaf88 | load | 4 | `abs 0xeb000` | `-` |
| `ldr` at 0xeaf8c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xeaf90 | store | 4 | `sp -0x40` | `[sp-32]` |
| `ldr` at 0xeaf94 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xeaf98 | store | 4 | `sp -0x3c` | `[sp-36]` |
| `ldr` at 0xeaf9c | load | 4 | `abs 0xeb004` | `-` |
| `str` at 0xeafa0 | store | 4 | `sp -0x38` | `0x2ae008` |
| `ldr` at 0xeafa4 | load | 4 | `abs 0xeb008` | `-` |
| `str` at 0xeafa8 | store | 4 | `sp -0x34` | `0x2ae088` |
| `ldr` at 0xeafac | load | 4 | `abs 0xeb00c` | `-` |
| `str` at 0xeafb0 | store | 4 | `sp -0x30` | `0x2b47f8` |
| `ldr` at 0xeafb4 | load | 4 | `[[sp-40]] +0x18` | `-` |
| `ldr` at 0xeafc0 | load | 4 | `abs 0x143870` | `-` |
| `ldr` at 0xeafc4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeafdc | load | 4 | `abs 0xeb010` | `-` |
| `str` at 0xeafe0 | store | 4 | `sp -0x2c` | `0xfffff7c4` |
| `str` at 0xeafec | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xeaff0 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xeaff4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xeaff4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xeaff4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xeaff4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeaf2c | `(unknown callee) 0xebc18` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xeaf54 | `(unknown callee) 0xe9a1c` | `ret(0xebc18)` | `?` | `?` | `ret(0xebc18)` |
| `mov` at 0xeafcc | `(unknown callee) (unknown target)` | `[[sp-40]+25]:1` | `?` | `[0x143870]` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xeaf40 | `ret(0xebc18) != 0x0` | `ret(0xebc18) == 0x0` |
| `beq` at 0xeaf60 | `ret(0xe9a1c) == 0x0` | `ret(0xe9a1c) != 0x0` |
| `beq` at 0xeafd8 | `ret(unknown) == 0x1` | `ret(unknown) != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xeaff4 | `r0` | `?` |

## `JChip_MMR_SC_f`

- `JChip_MMR_SC_f`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeb014 to 0xeb5ec.
  Established. Two four-byte reads target entry `r0 +0xc` (`ldr` at 0xeb034, `ldr` at 0xeb054). The
  unnamed `0xeaf08` call has `[sp-28]`, `([sp-72] + [sp-76])`, `[sp-64]`, `[sp-68]` (`bl` at
  0xeb3d0); its zero result takes the branch to `0xeb404` (`beq` at 0xeb3dc). A later unnamed
  `0xeae04` call carries `[sp-28]`, unknown, `sp-64`, `sp-68` (`bl` at 0xeb4e0). MMR command layout
  and hardware effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeb024 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xeb02c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb034 | load | 4 | `r0 +0xc` | `-` |
| `ldr` at 0xeb04c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb054 | load | 4 | `r0 +0xc` | `-` |
| `str` at 0xeb060 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xeb064 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb068 | load | 4 | `r0 +0x0` | `-` |
| `str` at 0xeb074 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xeb078 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xeb080 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xeb084 | load | 4 | `abs 0xeb5d4` | `-` |
| `ldr` at 0xeb088 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xeb098 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb0a0 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0xeb0b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb0b8 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0xeb0c4 | load | 4 | `abs 0xeb5d8` | `-` |
| `ldr` at 0xeb0d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb0dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeb0ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb0f4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xeb110 | store | 4 | `sp -0x1c` | `ret(0xebe18)` |
| `ldr` at 0xeb114 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xeb124 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xeb12c | load | 4 | `abs 0xeb5d4` | `-` |
| `ldr` at 0xeb130 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xeb13c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeb148 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xeb14c | store | 4 | `sp -0x70` | `[sp-32]` |
| `ldr` at 0xeb150 | load | 4 | `abs 0xeb5dc` | `-` |
| `ldr` at 0xeb158 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeb15c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeb164 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xeb174 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xeb178 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xeb180 | store | 4 | `sp -0x30` | `?` |
| `str` at 0xeb188 | store | 4 | `sp -0x34` | `0x2` |
| `ldr` at 0xeb18c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xeb1a0 | store | 4 | `sp -0x54` | `0x4` |
| `str` at 0xeb1ac | store | 4 | `sp -0x54` | `0x2` |
| `ldr` at 0xeb1b0 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xeb1b4 | store | 4 | `sp -0x38` | `[sp-84]` |
| `ldr` at 0xeb1b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeb1c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeb1cc | load | 4 | `sp -0x38` | `-` |
| `str` at 0xeb1d8 | store | 4 | `sp -0x58` | `ret(0xfb0a8)` |
| `ldr` at 0xeb1e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeb1e4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xeb1ec | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xeb1f0 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xeb1f4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xeb200 | store | 4 | `sp -0x58` | `ret(0xfb0a8)` |
| `ldr` at 0xeb204 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xeb208 | store | 4 | `sp -0x3c` | `ret(0xfb0a8)` |
| `str` at 0xeb214 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xeb218 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xeb22c | store | 4 | `sp -0x5c` | `0x2` |
| `str` at 0xeb238 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xeb23c | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xeb240 | store | 4 | `sp -0x30` | `[sp-92]` |
| `str` at 0xeb248 | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0xeb250 | store | 4 | `sp -0x38` | `0x2` |
| `ldr` at 0xeb254 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeb264 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeb268 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xeb274 | store | 4 | `sp -0x60` | `ret(0xfb0a8)` |
| `str` at 0xeb280 | store | 4 | `sp -0x60` | `0x1` |
| `ldr` at 0xeb284 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xeb288 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xeb28c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xeb29c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xeb2a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb2a4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xeb2b4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xeb2b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb2c0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xeb2d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb2dc | load | 4 | `(unknown)` | `-` |
| `str` at 0xeb2e0 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xeb2e4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeb2f4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xeb2fc | store | 4 | `sp -0x64` | `?` |
| `str` at 0xeb308 | store | 4 | `sp -0x64` | `0x0` |
| `ldr` at 0xeb30c | load | 4 | `sp -0x64` | `-` |
| `str` at 0xeb310 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xeb314 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xeb318 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb320 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xeb330 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb33c | load | 4 | `(unknown)` | `-` |
| `str` at 0xeb340 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xeb344 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xeb348 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb350 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xeb360 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb36c | load | 4 | `(unknown)` | `-` |
| `str` at 0xeb370 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xeb374 | load | 4 | `abs 0xeb5d4` | `-` |
| `ldr` at 0xeb378 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xeb384 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeb390 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xeb394 | store | 4 | `sp -0x70` | `[sp-64]` |
| `ldr` at 0xeb398 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xeb39c | store | 4 | `sp -0x6c` | `[sp-68]` |
| `ldr` at 0xeb3a0 | load | 4 | `abs 0xeb5e0` | `-` |
| `ldr` at 0xeb3a8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xeb3ac | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xeb3b4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xeb3b8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xeb3c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeb3c8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xeb3cc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xeb3e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb3e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb3e8 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xeb3f4 | store | 4 | `[sp-20] +0x0` | `?` |
| `str` at 0xeb3fc | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xeb404 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xeb40c | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0xeb418 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xeb41c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb420 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xeb430 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xeb434 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb43c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xeb44c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb458 | load | 4 | `(unknown)` | `-` |
| `str` at 0xeb45c | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xeb460 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeb470 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xeb478 | store | 4 | `sp -0x68` | `?` |
| `str` at 0xeb484 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0xeb488 | load | 4 | `sp -0x68` | `-` |
| `str` at 0xeb48c | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xeb490 | load | 4 | `abs 0xeb5d4` | `-` |
| `ldr` at 0xeb494 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xeb4a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeb4ac | load | 4 | `abs 0xeb5e4` | `-` |
| `ldr` at 0xeb4b4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xeb4b8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xeb4c0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xeb4c4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xeb4d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeb4f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb4f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb4f8 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xeb504 | store | 4 | `[sp-20] +0x0` | `?` |
| `str` at 0xeb50c | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xeb514 | load | 4 | `abs 0xeb5d4` | `-` |
| `ldr` at 0xeb518 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xeb524 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeb530 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xeb534 | store | 4 | `sp -0x70` | `[sp-64]` |
| `ldr` at 0xeb538 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xeb53c | store | 4 | `sp -0x6c` | `[sp-68]` |
| `ldr` at 0xeb540 | load | 4 | `abs 0xeb5e8` | `-` |
| `ldr` at 0xeb548 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xeb54c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xeb554 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xeb558 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb560 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xeb570 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb57c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xeb580 | store | 4 | `(unknown)` | `[sp-64]` |
| `ldr` at 0xeb584 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xeb588 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb590 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xeb5a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb5ac | load | 4 | `sp -0x44` | `-` |
| `str` at 0xeb5b0 | store | 4 | `(unknown)` | `[sp-68]` |
| `ldr` at 0xeb5b4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xeb5bc | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0xeb5c8 | store | 4 | `sp -0x50` | `0x1` |
| `ldr` at 0xeb5cc | load | 4 | `sp -0x50` | `-` |
| `ldmdb` at 0xeb5d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xeb5d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xeb5d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeb0cc | `(unknown callee) 0x23fbc` | `0x13704c` | `?` | `?` | `?` |
| `bl` at 0xeb108 | `(unknown callee) 0xebe18` | `?` | `?` | `0x4` | `?` |
| `bl` at 0xeb140 | `(unknown callee) 0xeaab4` | `[sp-28]` | `?` | `?` | `[0x2adfd8]` |
| `bl` at 0xeb160 | `(unknown callee) 0x23fbc` | `0x13706c` | `ret(0xeaab4)` | `[sp-28]` | `[sp-36]` |
| `bl` at 0xeb1d0 | `(unknown callee) 0xfb0a8` | `[sp-32]` | `[sp-84]` | `?` | `?` |
| `bl` at 0xeb1f8 | `(unknown callee) 0xfb0a8` | `[sp-88]` | `[sp-56]` | `0x2` | `?` |
| `bl` at 0xeb26c | `(unknown callee) 0xfb0a8` | `[sp-32]` | `0x2` | `?` | `?` |
| `bl` at 0xeb388 | `(unknown callee) 0xeaab4` | `[sp-28]` | `0x14` | `?` | `[0x2adfd8]` |
| `bl` at 0xeb3b0 | `(unknown callee) 0x23fbc` | `0x1370a4` | `ret(0xeaab4)` | `[sp-72]` | `[sp-76]` |
| `bl` at 0xeb3d0 | `(unknown callee) 0xeaf08` | `[sp-28]` | `([sp-72] + [sp-76])` | `[sp-64]` | `[sp-68]` |
| `bl` at 0xeb4a4 | `(unknown callee) 0xeaab4` | `[sp-28]` | `0x14` | `?` | `[0x2adfd8]` |
| `bl` at 0xeb4bc | `(unknown callee) 0x23fbc` | `0x1370dc` | `ret(0xeaab4)` | `[sp-76]` | `[sp-72]` |
| `bl` at 0xeb4e0 | `(unknown callee) 0xeae04` | `[sp-28]` | `?` | `sp-64` | `sp-68` |
| `bl` at 0xeb528 | `(unknown callee) 0xeaab4` | `[sp-28]` | `?` | `?` | `[0x2adfd8]` |
| `bl` at 0xeb550 | `(unknown callee) 0x23fbc` | `0x137108` | `ret(0xeaab4)` | `[sp-76]` | `[sp-72]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xeb090 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xeb11c | `Z == 0` | `not (Z == 0)` |
| `bls` at 0xeb138 | `[0x2adfd8] <= 0x1 (unsigned, 32-bit)` | `[0x2adfd8] > 0x1 (unsigned, 32-bit)` |
| `beq` at 0xeb16c | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `beq` at 0xeb198 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xeb1c4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xeb224 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xeb260 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xeb294 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `blt` at 0xeb2ac | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xeb2f0 | `? == 0x0` | `? != 0x0` |
| `bls` at 0xeb380 | `[0x2adfd8] <= 0x1 (unsigned, 32-bit)` | `[0x2adfd8] > 0x1 (unsigned, 32-bit)` |
| `beq` at 0xeb3dc | `ret(0xeaf08) == 0x0` | `ret(0xeaf08) != 0x0` |
| `blt` at 0xeb428 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xeb46c | `? == 0x0` | `? != 0x0` |
| `bls` at 0xeb49c | `[0x2adfd8] <= 0x1 (unsigned, 32-bit)` | `[0x2adfd8] > 0x1 (unsigned, 32-bit)` |
| `beq` at 0xeb4ec | `ret(0xeae04) == 0x0` | `ret(0xeae04) != 0x0` |
| `bls` at 0xeb520 | `[0x2adfd8] <= 0x1 (unsigned, 32-bit)` | `[0x2adfd8] > 0x1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xeb5d0 | `r0` | `?` |

## `PIOWrite_Multi`

- `PIOWrite_Multi`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xedaec to 0xee254.
  Established. Included four-byte stack assignments retain `0xd0000000` at `sp -0x44` and
  `0x50000000` at `sp -0x48` (`str` at 0xedc54, `str` at 0xedc5c). The unnamed `0xebfe0` call has
  `[sp-76]`, `[sp-32]`, `0x42`, `sp-56` (`bl` at 0xedd30). A later unsigned branch takes `[0x2ae110] <= 0x2` to `0xee094` (`bls` at 0xee08c). Four-byte global assignments at `0x2ae110` retain `0x0`
  and `[0x2ae110]+1` (`str` at 0xee07c, `str` at 0xee0c8). Multi-target PIO transfer semantics
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xedafc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xedb00 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xedb04 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xedb08 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xedb0c | load | 4 | `abs 0xee228` | `-` |
| `ldr` at 0xedb10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xedb30 | load | 4 | `(unknown)` | `-` |
| `str` at 0xedb34 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xedb38 | load | 4 | `abs 0xee228` | `-` |
| `ldr` at 0xedb3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xedb5c | load | 4 | `(unknown)` | `-` |
| `str` at 0xedb60 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xedb64 | load | 4 | `abs 0xee228` | `-` |
| `ldr` at 0xedb68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xedb88 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xedb90 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xedb98 | load | 1 | `(unknown)` | `-` |
| `str` at 0xedb9c | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xedba0 | load | 4 | `abs 0xee228` | `-` |
| `ldr` at 0xedba4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xedbc4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xedbcc | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xedbd4 | load | 2 | `(unknown)` | `-` |
| `str` at 0xedbd8 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xedbdc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xedbf4 | load | 4 | `sp +0x18` | `-` |
| `str` at 0xedc00 | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0xedc0c | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0xedc10 | load | 4 | `abs 0xee228` | `-` |
| `ldr` at 0xedc14 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xedc34 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xedc3c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xedc44 | load | 1 | `(unknown)` | `-` |
| `str` at 0xedc54 | store | 4 | `sp -0x44` | `0xd0000000` |
| `str` at 0xedc5c | store | 4 | `sp -0x48` | `0x50000000` |
| `str` at 0xedc68 | store | 4 | `sp -0x44` | `0x8000000` |
| `str` at 0xedc70 | store | 4 | `sp -0x48` | `0xf0000000` |
| `ldr` at 0xedc74 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xedc78 | store | 4 | `sp -0x38` | `[sp-68]` |
| `ldr` at 0xedc80 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xedc84 | store | 4 | `sp -0x6c` | `[sp+16]` |
| `ldr` at 0xedc88 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xedc8c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xedc90 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xedc9c | load | 4 | `sp -0x50` | `-` |
| `str` at 0xedca0 | store | 4 | `sp -0x6c` | `[sp-80]` |
| `ldr` at 0xedca4 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xedca8 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xedcac | load | 4 | `sp +0x14` | `-` |
| `str` at 0xedcb0 | store | 4 | `sp -0x64` | `[sp+20]` |
| `ldr` at 0xedcb4 | load | 4 | `sp +0x18` | `-` |
| `str` at 0xedcb8 | store | 4 | `sp -0x60` | `[sp+24]` |
| `ldr` at 0xedcc0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xedce0 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xedce8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xedcec | store | 4 | `sp -0x34` | `[sp+12]` |
| `ldr` at 0xedcf0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xedcf4 | store | 4 | `sp -0x38` | `[sp+8]` |
| `str` at 0xedd14 | store | 4 | `sp -0x6c` | `sp-36` |
| `ldr` at 0xedd18 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xedd1c | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xedd20 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xedd24 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xedd34 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xedd38 | store | 4 | `sp -0x6c` | `[sp-36]` |
| `ldr` at 0xedd3c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xedd40 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xedd44 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xedd48 | store | 4 | `sp -0x64` | `[sp+20]` |
| `ldr` at 0xedd4c | load | 4 | `sp +0x18` | `-` |
| `str` at 0xedd50 | store | 4 | `sp -0x60` | `[sp+24]` |
| `str` at 0xedd78 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xedd80 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xedd84 | store | 4 | `sp -0x38` | `[sp-72]` |
| `ldr` at 0xedd8c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xedd90 | store | 4 | `sp -0x6c` | `[sp+16]` |
| `ldr` at 0xedd94 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xedd98 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xedd9c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xedda8 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xeddac | store | 4 | `sp -0x6c` | `[sp-80]` |
| `ldr` at 0xeddb0 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xeddb4 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xeddb8 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xeddbc | store | 4 | `sp -0x64` | `[sp+20]` |
| `ldr` at 0xeddc0 | load | 4 | `sp +0x18` | `-` |
| `str` at 0xeddc4 | store | 4 | `sp -0x60` | `[sp+24]` |
| `str` at 0xeddec | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xeddf4 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xede00 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xede04 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xede0c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xede10 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xede18 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xede1c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xede20 | load | 4 | `abs 0xee22c` | `-` |
| `ldr` at 0xede2c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xede34 | load | 4 | `abs 0xee22c` | `-` |
| `str` at 0xede40 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xede44 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xede48 | load | 4 | `abs 0xee230` | `-` |
| `ldr` at 0xede54 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xede5c | load | 4 | `abs 0xee230` | `-` |
| `str` at 0xede68 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xede6c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xede70 | load | 4 | `abs 0xee234` | `-` |
| `ldr` at 0xede7c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xede84 | load | 4 | `abs 0xee234` | `-` |
| `str` at 0xede90 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xede94 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xedea4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xedeb8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xedebc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xedecc | load | 4 | `sp -0x38` | `-` |
| `str` at 0xedee0 | store | 4 | `sp -0x38` | `?` |
| `str` at 0xedeec | store | 4 | `sp -0x6c` | `sp-36` |
| `ldr` at 0xedef0 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xedef4 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xedef8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xedefc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xedf0c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xedf10 | store | 4 | `sp -0x6c` | `[sp-36]` |
| `ldr` at 0xedf14 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xedf18 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xedf1c | load | 4 | `sp +0x14` | `-` |
| `str` at 0xedf20 | store | 4 | `sp -0x64` | `[sp+20]` |
| `ldr` at 0xedf24 | load | 4 | `sp +0x18` | `-` |
| `str` at 0xedf28 | store | 4 | `sp -0x60` | `[sp+24]` |
| `str` at 0xedf50 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xedf58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xedf68 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xedf6c | store | 4 | `sp -0x6c` | `[sp-36]` |
| `ldr` at 0xedf70 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xedf74 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xedf78 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xedf7c | store | 4 | `sp -0x64` | `[sp+20]` |
| `ldr` at 0xedf80 | load | 4 | `sp +0x18` | `-` |
| `str` at 0xedf84 | store | 4 | `sp -0x60` | `[sp+24]` |
| `str` at 0xedfac | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xedfb4 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xedfc4 | store | 4 | `sp -0x5c` | `0x1` |
| `ldr` at 0xedfcc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xedfd0 | store | 4 | `sp -0x38` | `[sp-68]` |
| `ldr` at 0xedfd8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xedfdc | store | 4 | `sp -0x6c` | `[sp+16]` |
| `ldr` at 0xedfe0 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xedfe4 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xedfe8 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xedff4 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xedff8 | store | 4 | `sp -0x6c` | `[sp-80]` |
| `ldr` at 0xedffc | load | 4 | `sp +0x10` | `-` |
| `str` at 0xee000 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xee004 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xee008 | store | 4 | `sp -0x64` | `[sp+20]` |
| `ldr` at 0xee00c | load | 4 | `sp +0x18` | `-` |
| `str` at 0xee010 | store | 4 | `sp -0x60` | `[sp+24]` |
| `str` at 0xee038 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xee040 | load | 4 | `abs 0xee238` | `-` |
| `ldr` at 0xee044 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xee050 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xee054 | store | 4 | `sp -0x6c` | `[sp+20]` |
| `ldr` at 0xee058 | load | 4 | `abs 0xee23c` | `-` |
| `ldr` at 0xee05c | load | 4 | `abs 0xee240` | `-` |
| `ldr` at 0xee060 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xee064 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xee070 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xee074 | load | 4 | `abs 0xee244` | `-` |
| `str` at 0xee07c | store | 4 | `abs 0x2ae110` | `0x0` |
| `ldr` at 0xee080 | load | 4 | `abs 0xee244` | `-` |
| `ldr` at 0xee084 | load | 4 | `abs 0x2ae110` | `-` |
| `ldr` at 0xee094 | load | 4 | `abs 0xee244` | `-` |
| `ldr` at 0xee098 | load | 4 | `abs 0x2ae110` | `-` |
| `str` at 0xee0b4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xee0b8 | load | 4 | `abs 0xee244` | `-` |
| `ldr` at 0xee0bc | load | 4 | `abs 0xee244` | `-` |
| `ldr` at 0xee0c0 | load | 4 | `abs 0x2ae110` | `-` |
| `str` at 0xee0c8 | store | 4 | `abs 0x2ae110` | `[0x2ae110]+1` |
| `str` at 0xee0d8 | store | 4 | `sp -0x6c` | `sp-36` |
| `ldr` at 0xee0dc | load | 4 | `sp +0x10` | `-` |
| `str` at 0xee0e0 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xee0e4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xee0e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xee0f4 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xee100 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xee10c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xee110 | store | 4 | `sp -0x6c` | `[sp-36]` |
| `ldr` at 0xee114 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xee118 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xee11c | load | 4 | `sp +0x14` | `-` |
| `str` at 0xee120 | store | 4 | `sp -0x64` | `[sp+20]` |
| `ldr` at 0xee124 | load | 4 | `sp +0x18` | `-` |
| `str` at 0xee128 | store | 4 | `sp -0x60` | `[sp+24]` |
| `str` at 0xee150 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xee158 | load | 4 | `abs 0xee238` | `-` |
| `ldr` at 0xee15c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xee168 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xee16c | store | 4 | `sp -0x6c` | `[sp+20]` |
| `ldr` at 0xee170 | load | 4 | `abs 0xee248` | `-` |
| `ldr` at 0xee174 | load | 4 | `abs 0xee240` | `-` |
| `ldr` at 0xee178 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xee17c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xee184 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xee190 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xee1a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xee1b0 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xee1c4 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xee1c8 | load | 4 | `abs 0xee24c` | `-` |
| `str` at 0xee1d4 | store | 4 | `sp -0x5c` | `0x0` |
| `str` at 0xee1e0 | store | 4 | `sp -0x40` | `0x1` |
| `ldr` at 0xee1e4 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xee1ec | store | 4 | `sp +0x0` | `[sp]-1` |
| `ldr` at 0xee1f4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xee200 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xee204 | load | 4 | `abs 0xee250` | `-` |
| `str` at 0xee210 | store | 4 | `sp -0x5c` | `0x0` |
| `str` at 0xee21c | store | 4 | `sp -0x5c` | `0x1` |
| `ldr` at 0xee220 | load | 4 | `sp -0x5c` | `-` |
| `ldmdb` at 0xee224 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xee224 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xee224 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xedbf8 | `(unknown callee) 0xdd33c` | `0x1` | `0x0` | `[sp+24]` | `?` |
| `bl` at 0xedc98 | `(unknown callee) 0xebd58` | `[sp-80]` | `[sp-88]` | `[sp-84]` | `sp-56` |
| `bl` at 0xedccc | `(unknown callee) 0xdd8e8` | `0x0` | `[sp-60]` | `0x1` | `0x0` |
| `bl` at 0xedd08 | `(unknown callee) 0xe2cf8` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0xedd30 | `(unknown callee) 0xebfe0` | `[sp-76]` | `[sp-32]` | `0x42` | `sp-56` |
| `bl` at 0xedd64 | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xedda4 | `(unknown callee) 0xebd58` | `[sp-80]` | `[sp-88]` | `[sp-84]` | `sp-56` |
| `bl` at 0xeddd8 | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xedf08 | `(unknown callee) 0xebfe0` | `[sp-76]` | `[sp-32]` | `0x1e` | `sp-56` |
| `bl` at 0xedf3c | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xedf98 | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xedff0 | `(unknown callee) 0xebd58` | `[sp-80]` | `[sp-88]` | `[sp-84]` | `sp-56` |
| `bl` at 0xee024 | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xee068 | `(unknown callee) 0xec0d0` | `0x137898` | `0x1378a8` | `[sp-80]` | `[sp+16]` |
| `bl` at 0xee0f0 | `(unknown callee) 0xebfe0` | `[sp-76]` | `[sp-32]` | `0x42` | `sp-56` |
| `bl` at 0xee13c | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xee180 | `(unknown callee) 0xec0d0` | `0x1378ac` | `0x1378a8` | `[sp-36]` | `[sp+16]` |
| `bl` at 0xee194 | `(unknown callee) 0xe2688` | `[sp-32]+64` | `[sp+20]` | `?` | `[sp-32]+64` |
| `bl` at 0xee1b4 | `(unknown callee) 0xe2688` | `[sp-32]+65` | `[sp+20]` | `?` | `[sp-32]+65` |
| `bl` at 0xee1cc | `(unknown callee) 0x1b598` | `[sp+24]` | `0x1378c4` | `?` | `ret(0xe2688)` |
| `bl` at 0xee208 | `(unknown callee) 0x1b598` | `[sp+24]` | `0x1378f8` | `?` | `[sp-64]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xedbe8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xedc4c | `? != 0x6` | `? == 0x6` |
| `bne` at 0xedcd8 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bne` at 0xedd70 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bne` at 0xedde4 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bne` at 0xedf48 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `beq` at 0xedf64 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xedfa4 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bne` at 0xedfbc | `[sp] != 0x0` | `[sp] == 0x0` |
| `bne` at 0xee030 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xee04c | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bls` at 0xee08c | `[0x2ae110] <= 0x2 (unsigned, 32-bit)` | `[0x2ae110] > 0x2 (unsigned, 32-bit)` |
| `ble` at 0xee0fc | `[sp] <= 0x0 (signed, 32-bit)` | `[sp] > 0x0 (signed, 32-bit)` |
| `bne` at 0xee108 | `[sp-64] != 0x0` | `[sp-64] == 0x0` |
| `bne` at 0xee148 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xee164 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xee1a0 | `ret(0xe2688) == 0x0` | `ret(0xe2688) != 0x0` |
| `beq` at 0xee1c0 | `ret(0xe2688) == 0x0` | `ret(0xe2688) != 0x0` |
| `bne` at 0xee1fc | `[sp-64] != 0x0` | `[sp-64] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xee224 | `r0` | `?` |

## `PIOWrite_PIC`

- `PIOWrite_PIC`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xef7c8 to 0xefee0.
  Established. Four-byte assignments at `sp -0x38` include `0x48000000`, `0x28000000` and
  `0x1000000` (`str` at 0xef8cc, `str` at 0xefa30, `str` at 0xefadc). The unnamed `0xebfe0` call
  carries `[sp-64]`, `[sp-24]`, `0x40`, `sp-56` (`bl` at 0xef9b0). A later unsigned branch takes
  `[0x2ae110] <= 0x1` to `0xefd1c` (`bls` at 0xefd14). The unnamed `0xe2688` call has `[sp-24]+57`,
  `[sp+12]`, unknown, `[sp-24]+57` (`bl` at 0xefe1c). PIC command encoding and hardware effects
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xef7d8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xef7dc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xef7e0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xef7e4 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xef7e8 | load | 4 | `abs 0xefeb0` | `-` |
| `ldr` at 0xef7ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef80c | load | 4 | `(unknown)` | `-` |
| `str` at 0xef810 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xef814 | load | 4 | `abs 0xefeb0` | `-` |
| `ldr` at 0xef818 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef838 | load | 4 | `(unknown)` | `-` |
| `str` at 0xef83c | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xef840 | load | 4 | `abs 0xefeb0` | `-` |
| `ldr` at 0xef844 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef864 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef86c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xef874 | load | 1 | `(unknown)` | `-` |
| `str` at 0xef878 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xef87c | load | 4 | `abs 0xefeb0` | `-` |
| `ldr` at 0xef880 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef8a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef8a8 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xef8b0 | load | 2 | `(unknown)` | `-` |
| `str` at 0xef8b4 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xef8c0 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xef8cc | store | 4 | `sp -0x38` | `0x48000000` |
| `ldr` at 0xef8d4 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef8d8 | store | 4 | `sp -0x60` | `[sp+8]` |
| `ldr` at 0xef8dc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xef8e0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xef8e4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xef8f0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xef8f4 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xef8f8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef8fc | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xef900 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef904 | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0xef908 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xef90c | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0xef934 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xef93c | load | 4 | `abs 0xefeb4` | `-` |
| `ldr` at 0xef940 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xef94c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef950 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xef954 | load | 4 | `abs 0xefeb8` | `-` |
| `ldr` at 0xef958 | load | 4 | `abs 0xefebc` | `-` |
| `ldr` at 0xef95c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xef960 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xef968 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xef96c | store | 4 | `sp -0x34` | `[sp+4]` |
| `ldr` at 0xef970 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xef974 | store | 4 | `sp -0x38` | `[sp]` |
| `str` at 0xef994 | store | 4 | `sp -0x60` | `sp-36` |
| `ldr` at 0xef998 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef99c | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xef9a0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xef9a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef9b4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xef9b8 | store | 4 | `sp -0x60` | `[sp-36]` |
| `ldr` at 0xef9bc | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef9c0 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xef9c4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef9c8 | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0xef9cc | load | 4 | `sp +0x10` | `-` |
| `str` at 0xef9d0 | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0xef9f8 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xefa00 | load | 4 | `abs 0xefeb4` | `-` |
| `ldr` at 0xefa04 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xefa10 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xefa14 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xefa18 | load | 4 | `abs 0xefec0` | `-` |
| `ldr` at 0xefa1c | load | 4 | `abs 0xefebc` | `-` |
| `ldr` at 0xefa20 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xefa24 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xefa30 | store | 4 | `sp -0x38` | `0x28000000` |
| `ldr` at 0xefa38 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xefa3c | store | 4 | `sp -0x60` | `[sp+8]` |
| `ldr` at 0xefa40 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xefa44 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xefa48 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xefa54 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xefa58 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xefa5c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xefa60 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xefa64 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xefa68 | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0xefa6c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xefa70 | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0xefa98 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xefaa0 | load | 4 | `abs 0xefeb4` | `-` |
| `ldr` at 0xefaa4 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xefab0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xefab4 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xefab8 | load | 4 | `abs 0xefec4` | `-` |
| `ldr` at 0xefabc | load | 4 | `abs 0xefebc` | `-` |
| `ldr` at 0xefac0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xefac4 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xefad0 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xefad4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xefadc | store | 4 | `sp -0x38` | `0x1000000` |
| `ldr` at 0xefae0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xefae4 | store | 4 | `sp -0x34` | `[sp-32]` |
| `str` at 0xefb04 | store | 4 | `sp -0x60` | `sp-36` |
| `ldr` at 0xefb08 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xefb0c | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xefb10 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xefb14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xefb24 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xefb28 | store | 4 | `sp -0x60` | `[sp-36]` |
| `ldr` at 0xefb2c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xefb30 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xefb34 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xefb38 | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0xefb3c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xefb40 | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0xefb68 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xefb70 | load | 4 | `abs 0xefeb4` | `-` |
| `ldr` at 0xefb74 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xefb80 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xefb84 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xefb88 | load | 4 | `abs 0xefec8` | `-` |
| `ldr` at 0xefb8c | load | 4 | `abs 0xefebc` | `-` |
| `ldr` at 0xefb90 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xefb94 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xefba0 | store | 4 | `sp -0x38` | `0x8000000` |
| `ldr` at 0xefba8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xefbac | store | 4 | `sp -0x60` | `[sp+8]` |
| `ldr` at 0xefbb0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xefbb4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xefbb8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xefbc4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xefbc8 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xefbcc | load | 4 | `sp +0x8` | `-` |
| `str` at 0xefbd0 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xefbd4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xefbd8 | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0xefbdc | load | 4 | `sp +0x10` | `-` |
| `str` at 0xefbe0 | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0xefc08 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xefc10 | load | 4 | `abs 0xefeb4` | `-` |
| `ldr` at 0xefc14 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xefc20 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xefc24 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xefc28 | load | 4 | `abs 0xefecc` | `-` |
| `ldr` at 0xefc2c | load | 4 | `abs 0xefebc` | `-` |
| `ldr` at 0xefc30 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xefc34 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xefc3c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xefc4c | store | 4 | `sp -0x50` | `0x1` |
| `str` at 0xefc58 | store | 4 | `sp -0x38` | `0x28000000` |
| `ldr` at 0xefc60 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xefc64 | store | 4 | `sp -0x60` | `[sp+8]` |
| `ldr` at 0xefc68 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xefc6c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xefc70 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xefc7c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xefc80 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xefc84 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xefc88 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xefc8c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xefc90 | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0xefc94 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xefc98 | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0xefcc0 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xefcc8 | load | 4 | `abs 0xefeb4` | `-` |
| `ldr` at 0xefccc | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xefcd8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xefcdc | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xefce0 | load | 4 | `abs 0xefec4` | `-` |
| `ldr` at 0xefce4 | load | 4 | `abs 0xefebc` | `-` |
| `ldr` at 0xefce8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xefcec | load | 4 | `sp +0x8` | `-` |
| `str` at 0xefcf8 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xefcfc | load | 4 | `abs 0xefed0` | `-` |
| `str` at 0xefd04 | store | 4 | `abs 0x2ae110` | `0x0` |
| `ldr` at 0xefd08 | load | 4 | `abs 0xefed0` | `-` |
| `ldr` at 0xefd0c | load | 4 | `abs 0x2ae110` | `-` |
| `ldr` at 0xefd1c | load | 4 | `abs 0xefed0` | `-` |
| `ldr` at 0xefd20 | load | 4 | `abs 0x2ae110` | `-` |
| `str` at 0xefd3c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xefd40 | load | 4 | `abs 0xefed0` | `-` |
| `ldr` at 0xefd44 | load | 4 | `abs 0xefed0` | `-` |
| `ldr` at 0xefd48 | load | 4 | `abs 0x2ae110` | `-` |
| `str` at 0xefd50 | store | 4 | `abs 0x2ae110` | `[0x2ae110]+1` |
| `str` at 0xefd60 | store | 4 | `sp -0x60` | `sp-36` |
| `ldr` at 0xefd64 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xefd68 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xefd6c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xefd70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xefd7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xefd88 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xefd94 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xefd98 | store | 4 | `sp -0x60` | `[sp-36]` |
| `ldr` at 0xefd9c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xefda0 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xefda4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xefda8 | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0xefdac | load | 4 | `sp +0x10` | `-` |
| `str` at 0xefdb0 | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0xefdd8 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xefde0 | load | 4 | `abs 0xefeb4` | `-` |
| `ldr` at 0xefde4 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xefdf0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xefdf4 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xefdf8 | load | 4 | `abs 0xefed4` | `-` |
| `ldr` at 0xefdfc | load | 4 | `abs 0xefebc` | `-` |
| `ldr` at 0xefe00 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xefe04 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xefe0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xefe18 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xefe2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xefe38 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xefe4c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xefe50 | load | 4 | `abs 0xefed8` | `-` |
| `str` at 0xefe5c | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0xefe68 | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0xefe6c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xefe74 | store | 4 | `sp -0x1c` | `[sp-28]-1` |
| `ldr` at 0xefe7c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xefe88 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xefe8c | load | 4 | `abs 0xefedc` | `-` |
| `str` at 0xefe98 | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0xefea4 | store | 4 | `sp -0x50` | `0x1` |
| `ldr` at 0xefea8 | load | 4 | `sp -0x50` | `-` |
| `ldmdb` at 0xefeac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xefeac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xefeac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xef8c4 | `(unknown callee) 0xdd33c` | `0x1` | `0x0` | `[sp+16]` | `?` |
| `bl` at 0xef8ec | `(unknown callee) 0xebd58` | `[sp-68]` | `[sp-76]` | `[sp-72]` | `sp-56` |
| `bl` at 0xef920 | `(unknown callee) 0xdd8e8` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0xef964 | `(unknown callee) 0xec0d0` | `0x1379e0` | `0x1378a8` | `[sp-68]` | `[sp+8]` |
| `bl` at 0xef988 | `(unknown callee) 0xe2cf8` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0xef9b0 | `(unknown callee) 0xebfe0` | `[sp-64]` | `[sp-24]` | `0x40` | `sp-56` |
| `bl` at 0xef9e4 | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xefa28 | `(unknown callee) 0xec0d0` | `0x1379f4` | `0x1378a8` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xefa50 | `(unknown callee) 0xebd58` | `[sp-68]` | `[sp-76]` | `[sp-72]` | `sp-56` |
| `bl` at 0xefa84 | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xefac8 | `(unknown callee) 0xec0d0` | `0x137a08` | `0x1378a8` | `[sp-68]` | `[sp+8]` |
| `bl` at 0xefaf8 | `(unknown callee) 0xe2cf8` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0xefb20 | `(unknown callee) 0xebfe0` | `[sp-64]` | `[sp-24]` | `0x40` | `sp-56` |
| `bl` at 0xefb54 | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xefb98 | `(unknown callee) 0xec0d0` | `0x137a1c` | `0x1378a8` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xefbc0 | `(unknown callee) 0xebd58` | `[sp-68]` | `[sp-76]` | `[sp-72]` | `sp-56` |
| `bl` at 0xefbf4 | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xefc38 | `(unknown callee) 0xec0d0` | `0x137a30` | `0x1378a8` | `[sp-68]` | `[sp+8]` |
| `bl` at 0xefc78 | `(unknown callee) 0xebd58` | `[sp-68]` | `[sp-76]` | `[sp-72]` | `sp-56` |
| `bl` at 0xefcac | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xefcf0 | `(unknown callee) 0xec0d0` | `0x137a08` | `0x1378a8` | `[sp-68]` | `[sp+8]` |
| `bl` at 0xefd78 | `(unknown callee) 0xebfe0` | `[sp-64]` | `[sp-24]` | `0x40` | `sp-56` |
| `bl` at 0xefdc4 | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xefe08 | `(unknown callee) 0xec0d0` | `0x137a44` | `0x1378a8` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xefe1c | `(unknown callee) 0xe2688` | `[sp-24]+57` | `[sp+12]` | `?` | `[sp-24]+57` |
| `bl` at 0xefe3c | `(unknown callee) 0xe2688` | `[sp-24]+58` | `[sp+12]` | `?` | `[sp-24]+58` |
| `bl` at 0xefe54 | `(unknown callee) 0x1b598` | `[sp+16]` | `0x137a60` | `?` | `ret(0xe2688)` |
| `bl` at 0xefe90 | `(unknown callee) 0x1b598` | `[sp+16]` | `0x137a98` | `?` | `[sp-60]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xef92c | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xef948 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xef9f0 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xefa0c | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xefa90 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xefaac | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xefb60 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xefb7c | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xefc00 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xefc1c | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xefc44 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xefcb8 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xefcd4 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bls` at 0xefd14 | `[0x2ae110] <= 0x1 (unsigned, 32-bit)` | `[0x2ae110] > 0x1 (unsigned, 32-bit)` |
| `ble` at 0xefd84 | `[sp-28] <= 0x0 (signed, 32-bit)` | `[sp-28] > 0x0 (signed, 32-bit)` |
| `bne` at 0xefd90 | `[sp-60] != 0x0` | `[sp-60] == 0x0` |
| `bne` at 0xefdd0 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xefdec | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xefe28 | `ret(0xe2688) == 0x0` | `ret(0xe2688) != 0x0` |
| `beq` at 0xefe48 | `ret(0xe2688) == 0x0` | `ret(0xe2688) != 0x0` |
| `bne` at 0xefe84 | `[sp-60] != 0x0` | `[sp-60] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xefeac | `r0` | `?` |

## `PIOWrite_Special`

- `PIOWrite_Special`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xee254 to 0xee990.
  Established. Entry `r0 != 0x0` takes the branch to `0xee360` (`bne` at 0xee34c). Included
  four-byte stack assignments at `sp -0x40` retain `0xd0000000` and `0x8000000` on separate paths
  (`str` at 0xee3a4, `str` at 0xee3b8). The unnamed `0xebfe0` call has `[sp-72]`, `[sp-28]`, `0x42`,
  `sp-56` (`bl` at 0xee48c). A later signed branch takes `[sp-32] <= 0x0` to `0xee92c` (`ble` at
  0xee834). Special PIO operation meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xee264 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xee268 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xee26c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xee270 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xee274 | load | 4 | `abs 0xee960` | `-` |
| `ldr` at 0xee278 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xee298 | load | 4 | `(unknown)` | `-` |
| `str` at 0xee29c | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xee2a0 | load | 4 | `abs 0xee960` | `-` |
| `ldr` at 0xee2a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xee2c4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xee2c8 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xee2cc | load | 4 | `abs 0xee960` | `-` |
| `ldr` at 0xee2d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xee2f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xee2f8 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xee300 | load | 1 | `(unknown)` | `-` |
| `str` at 0xee304 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xee308 | load | 4 | `abs 0xee960` | `-` |
| `ldr` at 0xee30c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xee32c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xee334 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xee33c | load | 2 | `(unknown)` | `-` |
| `str` at 0xee340 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xee344 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xee358 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xee360 | load | 4 | `abs 0xee960` | `-` |
| `ldr` at 0xee364 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xee384 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xee38c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xee394 | load | 1 | `(unknown)` | `-` |
| `str` at 0xee3a4 | store | 4 | `sp -0x40` | `0xd0000000` |
| `str` at 0xee3ac | store | 4 | `sp -0x44` | `0x50000000` |
| `str` at 0xee3b8 | store | 4 | `sp -0x40` | `0x8000000` |
| `str` at 0xee3c0 | store | 4 | `sp -0x44` | `0xf0000000` |
| `ldr` at 0xee3c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xee3d0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xee3d4 | store | 4 | `sp -0x38` | `[sp-64]` |
| `ldr` at 0xee3dc | load | 4 | `sp +0xc` | `-` |
| `str` at 0xee3e0 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xee3e4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xee3e8 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xee3ec | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xee3f8 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xee3fc | store | 4 | `sp -0x68` | `[sp-76]` |
| `ldr` at 0xee400 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xee404 | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0xee408 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xee40c | store | 4 | `sp -0x60` | `[sp+16]` |
| `ldr` at 0xee410 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xee414 | store | 4 | `sp -0x5c` | `[sp+20]` |
| `str` at 0xee43c | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xee444 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xee448 | store | 4 | `sp -0x34` | `[sp+8]` |
| `ldr` at 0xee44c | load | 4 | `sp +0x4` | `-` |
| `str` at 0xee450 | store | 4 | `sp -0x38` | `[sp+4]` |
| `str` at 0xee470 | store | 4 | `sp -0x68` | `sp-36` |
| `ldr` at 0xee474 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xee478 | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0xee47c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xee480 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xee490 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xee494 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xee498 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xee49c | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0xee4a0 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xee4a4 | store | 4 | `sp -0x60` | `[sp+16]` |
| `ldr` at 0xee4a8 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xee4ac | store | 4 | `sp -0x5c` | `[sp+20]` |
| `str` at 0xee4d4 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xee4dc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xee4e0 | store | 4 | `sp -0x38` | `[sp-68]` |
| `ldr` at 0xee4e8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xee4ec | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xee4f0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xee4f4 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xee4f8 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xee504 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xee508 | store | 4 | `sp -0x68` | `[sp-76]` |
| `ldr` at 0xee50c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xee510 | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0xee514 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xee518 | store | 4 | `sp -0x60` | `[sp+16]` |
| `ldr` at 0xee51c | load | 4 | `sp +0x14` | `-` |
| `str` at 0xee520 | store | 4 | `sp -0x5c` | `[sp+20]` |
| `str` at 0xee548 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xee554 | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0xee55c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xee568 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xee56c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xee574 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xee578 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xee580 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xee584 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xee588 | load | 4 | `abs 0xee964` | `-` |
| `ldr` at 0xee594 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xee59c | load | 4 | `abs 0xee964` | `-` |
| `str` at 0xee5a8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xee5ac | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xee5b0 | load | 4 | `abs 0xee968` | `-` |
| `ldr` at 0xee5bc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xee5c4 | load | 4 | `abs 0xee968` | `-` |
| `str` at 0xee5d0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xee5d4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xee5d8 | load | 4 | `abs 0xee96c` | `-` |
| `ldr` at 0xee5e4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xee5ec | load | 4 | `abs 0xee96c` | `-` |
| `str` at 0xee5f8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xee5fc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xee60c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xee620 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xee624 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xee634 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xee648 | store | 4 | `sp -0x38` | `?` |
| `str` at 0xee654 | store | 4 | `sp -0x68` | `sp-36` |
| `ldr` at 0xee658 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xee65c | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0xee660 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xee664 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xee674 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xee678 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xee67c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xee680 | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0xee684 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xee688 | store | 4 | `sp -0x60` | `[sp+16]` |
| `ldr` at 0xee68c | load | 4 | `sp +0x14` | `-` |
| `str` at 0xee690 | store | 4 | `sp -0x5c` | `[sp+20]` |
| `str` at 0xee6b8 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xee6c0 | load | 4 | `abs 0xee970` | `-` |
| `ldr` at 0xee6c4 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xee6d0 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xee6d4 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xee6d8 | load | 4 | `abs 0xee974` | `-` |
| `ldr` at 0xee6dc | load | 4 | `abs 0xee978` | `-` |
| `ldr` at 0xee6e0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xee6e4 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xee6ec | load | 4 | `sp -0x20` | `-` |
| `str` at 0xee6fc | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0xee704 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xee708 | store | 4 | `sp -0x38` | `[sp-64]` |
| `ldr` at 0xee710 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xee714 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xee718 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xee71c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xee720 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xee72c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xee730 | store | 4 | `sp -0x68` | `[sp-76]` |
| `ldr` at 0xee734 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xee738 | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0xee73c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xee740 | store | 4 | `sp -0x60` | `[sp+16]` |
| `ldr` at 0xee744 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xee748 | store | 4 | `sp -0x5c` | `[sp+20]` |
| `str` at 0xee770 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xee778 | load | 4 | `abs 0xee970` | `-` |
| `ldr` at 0xee77c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xee788 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xee78c | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xee790 | load | 4 | `abs 0xee97c` | `-` |
| `ldr` at 0xee794 | load | 4 | `abs 0xee978` | `-` |
| `ldr` at 0xee798 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xee79c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xee7a8 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xee7ac | load | 4 | `abs 0xee980` | `-` |
| `str` at 0xee7b4 | store | 4 | `abs 0x2ae110` | `0x0` |
| `ldr` at 0xee7b8 | load | 4 | `abs 0xee980` | `-` |
| `ldr` at 0xee7bc | load | 4 | `abs 0x2ae110` | `-` |
| `ldr` at 0xee7cc | load | 4 | `abs 0xee980` | `-` |
| `ldr` at 0xee7d0 | load | 4 | `abs 0x2ae110` | `-` |
| `str` at 0xee7ec | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xee7f0 | load | 4 | `abs 0xee980` | `-` |
| `ldr` at 0xee7f4 | load | 4 | `abs 0xee980` | `-` |
| `ldr` at 0xee7f8 | load | 4 | `abs 0x2ae110` | `-` |
| `str` at 0xee800 | store | 4 | `abs 0x2ae110` | `[0x2ae110]+1` |
| `str` at 0xee810 | store | 4 | `sp -0x68` | `sp-36` |
| `ldr` at 0xee814 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xee818 | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0xee81c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xee820 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xee82c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xee838 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xee844 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xee848 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xee84c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xee850 | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0xee854 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xee858 | store | 4 | `sp -0x60` | `[sp+16]` |
| `ldr` at 0xee85c | load | 4 | `sp +0x14` | `-` |
| `str` at 0xee860 | store | 4 | `sp -0x5c` | `[sp+20]` |
| `str` at 0xee888 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xee890 | load | 4 | `abs 0xee970` | `-` |
| `ldr` at 0xee894 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xee8a0 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xee8a4 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xee8a8 | load | 4 | `abs 0xee984` | `-` |
| `ldr` at 0xee8ac | load | 4 | `abs 0xee978` | `-` |
| `ldr` at 0xee8b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xee8b4 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xee8bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xee8c8 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xee8dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xee8e8 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xee8fc | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xee900 | load | 4 | `abs 0xee988` | `-` |
| `str` at 0xee90c | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xee918 | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0xee91c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xee924 | store | 4 | `sp -0x20` | `[sp-32]-1` |
| `ldr` at 0xee92c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xee938 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xee93c | load | 4 | `abs 0xee98c` | `-` |
| `str` at 0xee948 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xee954 | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0xee958 | load | 4 | `sp -0x58` | `-` |
| `ldmdb` at 0xee95c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xee95c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xee95c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xee35c | `(unknown callee) 0xdd33c` | `0x1` | `0x0` | `[sp+20]` | `r0` |
| `bl` at 0xee3f4 | `(unknown callee) 0xebd58` | `[sp-76]` | `[sp-84]` | `[sp-80]` | `sp-56` |
| `bl` at 0xee428 | `(unknown callee) 0xdd8e8` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0xee464 | `(unknown callee) 0xe2cf8` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0xee48c | `(unknown callee) 0xebfe0` | `[sp-72]` | `[sp-28]` | `0x42` | `sp-56` |
| `bl` at 0xee4c0 | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xee500 | `(unknown callee) 0xebd58` | `[sp-76]` | `[sp-84]` | `[sp-80]` | `sp-56` |
| `bl` at 0xee534 | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xee670 | `(unknown callee) 0xebfe0` | `[sp-72]` | `?` | `0x1e` | `sp-56` |
| `bl` at 0xee6a4 | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xee6e8 | `(unknown callee) 0xec0d0` | `0x137930` | `0x1378a8` | `[sp-36]` | `[sp+12]` |
| `bl` at 0xee728 | `(unknown callee) 0xebd58` | `[sp-76]` | `[sp-84]` | `[sp-80]` | `sp-56` |
| `bl` at 0xee75c | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xee7a0 | `(unknown callee) 0xec0d0` | `0x137898` | `0x1378a8` | `[sp-76]` | `[sp+12]` |
| `bl` at 0xee828 | `(unknown callee) 0xebfe0` | `[sp-72]` | `[sp-28]` | `0x42` | `sp-56` |
| `bl` at 0xee874 | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xee8b8 | `(unknown callee) 0xec0d0` | `0x1378ac` | `0x1378a8` | `[sp-36]` | `[sp+12]` |
| `bl` at 0xee8cc | `(unknown callee) 0xe2688` | `[sp-28]+64` | `[sp+16]` | `?` | `[sp-28]+64` |
| `bl` at 0xee8ec | `(unknown callee) 0xe2688` | `[sp-28]+65` | `[sp+16]` | `?` | `[sp-28]+65` |
| `bl` at 0xee904 | `(unknown callee) 0x1b598` | `[sp+20]` | `0x1378c4` | `?` | `ret(0xe2688)` |
| `bl` at 0xee940 | `(unknown callee) 0x1b598` | `[sp+20]` | `0x1378f8` | `?` | `[sp-60]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xee34c | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xee39c | `? != 0x6` | `? == 0x6` |
| `bne` at 0xee3cc | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xee434 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bne` at 0xee4cc | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bne` at 0xee540 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bne` at 0xee6b0 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xee6cc | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xee6f4 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0xee768 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xee784 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bls` at 0xee7c4 | `[0x2ae110] <= 0x2 (unsigned, 32-bit)` | `[0x2ae110] > 0x2 (unsigned, 32-bit)` |
| `ble` at 0xee834 | `[sp-32] <= 0x0 (signed, 32-bit)` | `[sp-32] > 0x0 (signed, 32-bit)` |
| `bne` at 0xee840 | `[sp-60] != 0x0` | `[sp-60] == 0x0` |
| `bne` at 0xee880 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xee89c | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xee8d8 | `ret(0xe2688) == 0x0` | `ret(0xe2688) != 0x0` |
| `beq` at 0xee8f8 | `ret(0xe2688) == 0x0` | `ret(0xe2688) != 0x0` |
| `bne` at 0xee934 | `[sp-60] != 0x0` | `[sp-60] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xee95c | `r0` | `?` |

## `PIOWrite_Common`

- `PIOWrite_Common`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xee990 to 0xef134.
  Established. Included four-byte stack assignments retain `0xd0000000` at `sp -0x40` and
  `0x50000000` at `sp -0x44` (`str` at 0xeead4, `str` at 0xeeadc). The unnamed `0xebfe0` call has
  `[sp-72]`, `[sp-24]`, `0x42`, `sp-56` (`bl` at 0xeebdc). A later signed branch takes `[sp-28] <= 0x0` to `0xef0c8` (`ble` at 0xeefd0). The unnamed `0xe2688` call carries `[sp-24]+64`, `[sp+12]`,
  unknown, `[sp-24]+64` (`bl` at 0xef068). Common PIO write encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xee9a0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xee9a4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xee9a8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xee9ac | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xee9b0 | load | 4 | `abs 0xef0fc` | `-` |
| `ldr` at 0xee9b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xee9d4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xee9d8 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xee9dc | load | 4 | `abs 0xef0fc` | `-` |
| `ldr` at 0xee9e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeea00 | load | 4 | `(unknown)` | `-` |
| `str` at 0xeea04 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xeea08 | load | 4 | `abs 0xef0fc` | `-` |
| `ldr` at 0xeea0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeea2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeea34 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xeea3c | load | 1 | `(unknown)` | `-` |
| `str` at 0xeea40 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xeea44 | load | 4 | `abs 0xef0fc` | `-` |
| `ldr` at 0xeea48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeea68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeea70 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xeea78 | load | 2 | `(unknown)` | `-` |
| `str` at 0xeea7c | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xeea88 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xeea90 | load | 4 | `abs 0xef0fc` | `-` |
| `ldr` at 0xeea94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeeab4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeeabc | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xeeac4 | load | 1 | `(unknown)` | `-` |
| `str` at 0xeead4 | store | 4 | `sp -0x40` | `0xd0000000` |
| `str` at 0xeeadc | store | 4 | `sp -0x44` | `0x50000000` |
| `str` at 0xeeae8 | store | 4 | `sp -0x40` | `0x8000000` |
| `str` at 0xeeaf0 | store | 4 | `sp -0x44` | `0xf0000000` |
| `ldr` at 0xeeaf4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xeeaf8 | store | 4 | `sp -0x38` | `[sp-64]` |
| `ldr` at 0xeeb00 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xeeb04 | store | 4 | `sp -0x68` | `[sp+8]` |
| `ldr` at 0xeeb08 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xeeb0c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xeeb10 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xeeb1c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xeeb20 | store | 4 | `sp -0x68` | `[sp-76]` |
| `ldr` at 0xeeb24 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xeeb28 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xeeb2c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xeeb30 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xeeb34 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xeeb38 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `str` at 0xeeb60 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xeeb68 | load | 4 | `abs 0xef100` | `-` |
| `ldr` at 0xeeb6c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xeeb78 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xeeb7c | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xeeb80 | load | 4 | `abs 0xef104` | `-` |
| `ldr` at 0xeeb84 | load | 4 | `abs 0xef108` | `-` |
| `ldr` at 0xeeb88 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xeeb8c | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xeeb94 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xeeb98 | store | 4 | `sp -0x34` | `[sp+4]` |
| `ldr` at 0xeeb9c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xeeba0 | store | 4 | `sp -0x38` | `[sp]` |
| `str` at 0xeebc0 | store | 4 | `sp -0x68` | `sp-36` |
| `ldr` at 0xeebc4 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xeebc8 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xeebcc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xeebd0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeebe0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xeebe4 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xeebe8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xeebec | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xeebf0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xeebf4 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xeebf8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xeebfc | store | 4 | `sp -0x5c` | `[sp+16]` |
| `str` at 0xeec24 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xeec2c | load | 4 | `abs 0xef100` | `-` |
| `ldr` at 0xeec30 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xeec3c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xeec40 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xeec44 | load | 4 | `abs 0xef10c` | `-` |
| `ldr` at 0xeec48 | load | 4 | `abs 0xef108` | `-` |
| `ldr` at 0xeec4c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeec50 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xeec58 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xeec5c | store | 4 | `sp -0x38` | `[sp-68]` |
| `ldr` at 0xeec64 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xeec68 | store | 4 | `sp -0x68` | `[sp+8]` |
| `ldr` at 0xeec6c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xeec70 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xeec74 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xeec80 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xeec84 | store | 4 | `sp -0x68` | `[sp-76]` |
| `ldr` at 0xeec88 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xeec8c | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xeec90 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xeec94 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xeec98 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xeec9c | store | 4 | `sp -0x5c` | `[sp+16]` |
| `str` at 0xeecc4 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xeeccc | load | 4 | `abs 0xef100` | `-` |
| `ldr` at 0xeecd0 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xeecdc | load | 4 | `sp +0xc` | `-` |
| `str` at 0xeece0 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xeece4 | load | 4 | `abs 0xef110` | `-` |
| `ldr` at 0xeece8 | load | 4 | `abs 0xef108` | `-` |
| `ldr` at 0xeecec | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xeecf0 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xeecf8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xeed04 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xeed08 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xeed10 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xeed14 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xeed1c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xeed20 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xeed24 | load | 4 | `abs 0xef114` | `-` |
| `ldr` at 0xeed30 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xeed38 | load | 4 | `abs 0xef114` | `-` |
| `str` at 0xeed44 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xeed48 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xeed4c | load | 4 | `abs 0xef118` | `-` |
| `ldr` at 0xeed58 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xeed60 | load | 4 | `abs 0xef118` | `-` |
| `str` at 0xeed6c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xeed70 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xeed74 | load | 4 | `abs 0xef11c` | `-` |
| `ldr` at 0xeed80 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xeed88 | load | 4 | `abs 0xef11c` | `-` |
| `str` at 0xeed94 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xeed98 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xeeda8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xeedbc | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xeedc0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xeedd0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xeede4 | store | 4 | `sp -0x38` | `?` |
| `str` at 0xeedf0 | store | 4 | `sp -0x68` | `sp-36` |
| `ldr` at 0xeedf4 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xeedf8 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xeedfc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xeee00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeee10 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xeee14 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xeee18 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xeee1c | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xeee20 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xeee24 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xeee28 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xeee2c | store | 4 | `sp -0x5c` | `[sp+16]` |
| `str` at 0xeee54 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xeee5c | load | 4 | `abs 0xef100` | `-` |
| `ldr` at 0xeee60 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xeee6c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xeee70 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xeee74 | load | 4 | `abs 0xef120` | `-` |
| `ldr` at 0xeee78 | load | 4 | `abs 0xef108` | `-` |
| `ldr` at 0xeee7c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeee80 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xeee88 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xeee98 | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0xeeea0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xeeea4 | store | 4 | `sp -0x38` | `[sp-64]` |
| `ldr` at 0xeeeac | load | 4 | `sp +0x8` | `-` |
| `str` at 0xeeeb0 | store | 4 | `sp -0x68` | `[sp+8]` |
| `ldr` at 0xeeeb4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xeeeb8 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xeeebc | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xeeec8 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xeeecc | store | 4 | `sp -0x68` | `[sp-76]` |
| `ldr` at 0xeeed0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xeeed4 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xeeed8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xeeedc | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xeeee0 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xeeee4 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `str` at 0xeef0c | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xeef14 | load | 4 | `abs 0xef100` | `-` |
| `ldr` at 0xeef18 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xeef24 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xeef28 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xeef2c | load | 4 | `abs 0xef104` | `-` |
| `ldr` at 0xeef30 | load | 4 | `abs 0xef108` | `-` |
| `ldr` at 0xeef34 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xeef38 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xeef44 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xeef48 | load | 4 | `abs 0xef124` | `-` |
| `str` at 0xeef50 | store | 4 | `abs 0x2ae110` | `0x0` |
| `ldr` at 0xeef54 | load | 4 | `abs 0xef124` | `-` |
| `ldr` at 0xeef58 | load | 4 | `abs 0x2ae110` | `-` |
| `ldr` at 0xeef68 | load | 4 | `abs 0xef124` | `-` |
| `ldr` at 0xeef6c | load | 4 | `abs 0x2ae110` | `-` |
| `str` at 0xeef88 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xeef8c | load | 4 | `abs 0xef124` | `-` |
| `ldr` at 0xeef90 | load | 4 | `abs 0xef124` | `-` |
| `ldr` at 0xeef94 | load | 4 | `abs 0x2ae110` | `-` |
| `str` at 0xeef9c | store | 4 | `abs 0x2ae110` | `[0x2ae110]+1` |
| `str` at 0xeefac | store | 4 | `sp -0x68` | `sp-36` |
| `ldr` at 0xeefb0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xeefb4 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xeefb8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xeefbc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeefc8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeefd4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xeefe0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xeefe4 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xeefe8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xeefec | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xeeff0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xeeff4 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xeeff8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xeeffc | store | 4 | `sp -0x5c` | `[sp+16]` |
| `str` at 0xef024 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xef02c | load | 4 | `abs 0xef100` | `-` |
| `ldr` at 0xef030 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xef03c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef040 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xef044 | load | 4 | `abs 0xef128` | `-` |
| `ldr` at 0xef048 | load | 4 | `abs 0xef108` | `-` |
| `ldr` at 0xef04c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xef050 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xef058 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef064 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xef078 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef084 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xef098 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xef09c | load | 4 | `abs 0xef12c` | `-` |
| `str` at 0xef0a8 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xef0b4 | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0xef0b8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xef0c0 | store | 4 | `sp -0x1c` | `[sp-28]-1` |
| `ldr` at 0xef0c8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xef0d4 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xef0d8 | load | 4 | `abs 0xef130` | `-` |
| `str` at 0xef0e4 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xef0f0 | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0xef0f4 | load | 4 | `sp -0x58` | `-` |
| `ldmdb` at 0xef0f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xef0f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xef0f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeea8c | `(unknown callee) 0xdd33c` | `0x1` | `0x0` | `[sp+16]` | `?` |
| `bl` at 0xeeb18 | `(unknown callee) 0xebd58` | `[sp-76]` | `[sp-84]` | `[sp-80]` | `sp-56` |
| `bl` at 0xeeb4c | `(unknown callee) 0xdd8e8` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0xeeb90 | `(unknown callee) 0xec0d0` | `0x137898` | `0x1378a8` | `[sp-76]` | `[sp+8]` |
| `bl` at 0xeebb4 | `(unknown callee) 0xe2cf8` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0xeebdc | `(unknown callee) 0xebfe0` | `[sp-72]` | `[sp-24]` | `0x42` | `sp-56` |
| `bl` at 0xeec10 | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xeec54 | `(unknown callee) 0xec0d0` | `0x137940` | `0x1378a8` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xeec7c | `(unknown callee) 0xebd58` | `[sp-76]` | `[sp-84]` | `[sp-80]` | `sp-56` |
| `bl` at 0xeecb0 | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xeecf4 | `(unknown callee) 0xec0d0` | `0x137950` | `0x1378a8` | `[sp-76]` | `[sp+8]` |
| `bl` at 0xeee0c | `(unknown callee) 0xebfe0` | `[sp-72]` | `[sp-24]` | `0x1e` | `sp-56` |
| `bl` at 0xeee40 | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xeee84 | `(unknown callee) 0xec0d0` | `0x137930` | `0x1378a8` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xeeec4 | `(unknown callee) 0xebd58` | `[sp-76]` | `[sp-84]` | `[sp-80]` | `sp-56` |
| `bl` at 0xeeef8 | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xeef3c | `(unknown callee) 0xec0d0` | `0x137898` | `0x1378a8` | `[sp-76]` | `[sp+8]` |
| `bl` at 0xeefc4 | `(unknown callee) 0xebfe0` | `[sp-72]` | `[sp-24]` | `0x42` | `sp-56` |
| `bl` at 0xef010 | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xef054 | `(unknown callee) 0xec0d0` | `0x1378ac` | `0x1378a8` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xef068 | `(unknown callee) 0xe2688` | `[sp-24]+64` | `[sp+12]` | `?` | `[sp-24]+64` |
| `bl` at 0xef088 | `(unknown callee) 0xe2688` | `[sp-24]+65` | `[sp+12]` | `?` | `[sp-24]+65` |
| `bl` at 0xef0a0 | `(unknown callee) 0x1b598` | `[sp+16]` | `0x1378c4` | `?` | `ret(0xe2688)` |
| `bl` at 0xef0dc | `(unknown callee) 0x1b598` | `[sp+16]` | `0x1378f8` | `?` | `[sp-60]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xeeacc | `? != 0x6` | `? == 0x6` |
| `bne` at 0xeeb58 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xeeb74 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xeec1c | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xeec38 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xeecbc | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xeecd8 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xeee4c | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xeee68 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xeee90 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xeef04 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xeef20 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bls` at 0xeef60 | `[0x2ae110] <= 0x2 (unsigned, 32-bit)` | `[0x2ae110] > 0x2 (unsigned, 32-bit)` |
| `ble` at 0xeefd0 | `[sp-28] <= 0x0 (signed, 32-bit)` | `[sp-28] > 0x0 (signed, 32-bit)` |
| `bne` at 0xeefdc | `[sp-60] != 0x0` | `[sp-60] == 0x0` |
| `bne` at 0xef01c | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xef038 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xef074 | `ret(0xe2688) == 0x0` | `ret(0xe2688) != 0x0` |
| `beq` at 0xef094 | `ret(0xe2688) == 0x0` | `ret(0xe2688) != 0x0` |
| `bne` at 0xef0d0 | `[sp-60] != 0x0` | `[sp-60] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xef0f8 | `r0` | `?` |

## `PIORead_Common`

- `PIORead_Common`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xef134 to 0xef7c8.
  Established. Two four-byte pointee stores clear `[sp+4] +0x0` and `[sp] +0x0` (`str` at 0xef2a4,
  `str` at 0xef2ac). The unnamed `0xebfe0` call has `[sp-88]`, `[sp-24]`, `0x1e`, `sp-56` (`bl` at
  0xef458). A later unsigned branch takes `[0x2ae110] <= 0x2` to `0xef5b4` (`bls` at 0xef5ac).
  Later included four-byte stores through those two pointees retain `[sp-68]` and `[sp-72]` (`str`
  at 0xef738, `str` at 0xef744). Common PIO read result meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xef144 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xef148 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xef14c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xef150 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xef154 | load | 4 | `abs 0xef794` | `-` |
| `ldr` at 0xef158 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef178 | load | 4 | `(unknown)` | `-` |
| `str` at 0xef17c | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xef180 | load | 4 | `abs 0xef794` | `-` |
| `ldr` at 0xef184 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef1a4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xef1a8 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0xef1ac | load | 4 | `abs 0xef794` | `-` |
| `ldr` at 0xef1b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef1d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef1d8 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xef1e0 | load | 1 | `(unknown)` | `-` |
| `str` at 0xef1e4 | store | 4 | `sp -0x60` | `?` |
| `ldr` at 0xef1e8 | load | 4 | `abs 0xef794` | `-` |
| `ldr` at 0xef1ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef20c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef214 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xef21c | load | 2 | `(unknown)` | `-` |
| `str` at 0xef220 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0xef22c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xef234 | load | 4 | `abs 0xef794` | `-` |
| `ldr` at 0xef238 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef258 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef260 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xef268 | load | 1 | `(unknown)` | `-` |
| `str` at 0xef278 | store | 4 | `sp -0x50` | `0xd0000000` |
| `str` at 0xef280 | store | 4 | `sp -0x54` | `0x50000000` |
| `str` at 0xef28c | store | 4 | `sp -0x50` | `0x8000000` |
| `str` at 0xef294 | store | 4 | `sp -0x54` | `0xf0000000` |
| `ldr` at 0xef298 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xef29c | load | 4 | `sp +0x4` | `-` |
| `str` at 0xef2a4 | store | 4 | `[sp+4] +0x0` | `0x0` |
| `str` at 0xef2ac | store | 4 | `[sp] +0x0` | `0x0` |
| `ldr` at 0xef2b0 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xef2b4 | store | 4 | `sp -0x38` | `[sp-84]` |
| `ldr` at 0xef2bc | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef2c0 | store | 4 | `sp -0x78` | `[sp+8]` |
| `ldr` at 0xef2c4 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xef2c8 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xef2cc | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xef2d8 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xef2dc | store | 4 | `sp -0x78` | `[sp-92]` |
| `ldr` at 0xef2e0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef2e4 | store | 4 | `sp -0x74` | `[sp+8]` |
| `ldr` at 0xef2e8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef2ec | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xef2f0 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xef2f4 | store | 4 | `sp -0x6c` | `[sp+16]` |
| `str` at 0xef31c | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0xef324 | load | 4 | `abs 0xef798` | `-` |
| `ldr` at 0xef328 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xef334 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef338 | store | 4 | `sp -0x78` | `[sp+12]` |
| `ldr` at 0xef33c | load | 4 | `abs 0xef79c` | `-` |
| `ldr` at 0xef340 | load | 4 | `abs 0xef7a0` | `-` |
| `ldr` at 0xef344 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xef348 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xef350 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xef35c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xef360 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xef368 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xef36c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xef370 | load | 4 | `abs 0xef7a4` | `-` |
| `ldr` at 0xef37c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xef384 | load | 4 | `abs 0xef7a4` | `-` |
| `str` at 0xef390 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xef394 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xef398 | load | 4 | `abs 0xef7a8` | `-` |
| `ldr` at 0xef3a4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xef3ac | load | 4 | `abs 0xef7a8` | `-` |
| `str` at 0xef3b8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xef3bc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xef3c0 | load | 4 | `abs 0xef7ac` | `-` |
| `ldr` at 0xef3cc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xef3d4 | load | 4 | `abs 0xef7ac` | `-` |
| `str` at 0xef3e0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xef3e4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xef3f4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xef408 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xef40c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xef41c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xef430 | store | 4 | `sp -0x38` | `?` |
| `str` at 0xef43c | store | 4 | `sp -0x78` | `sp-36` |
| `ldr` at 0xef440 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef444 | store | 4 | `sp -0x74` | `[sp+8]` |
| `ldr` at 0xef448 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xef44c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef45c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xef460 | store | 4 | `sp -0x78` | `[sp-36]` |
| `ldr` at 0xef464 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef468 | store | 4 | `sp -0x74` | `[sp+8]` |
| `ldr` at 0xef46c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef470 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xef474 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xef478 | store | 4 | `sp -0x6c` | `[sp+16]` |
| `str` at 0xef4a0 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0xef4a8 | load | 4 | `abs 0xef798` | `-` |
| `ldr` at 0xef4ac | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xef4b8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef4bc | store | 4 | `sp -0x78` | `[sp+12]` |
| `ldr` at 0xef4c0 | load | 4 | `abs 0xef7b0` | `-` |
| `ldr` at 0xef4c4 | load | 4 | `abs 0xef7a0` | `-` |
| `ldr` at 0xef4c8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xef4cc | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xef4d4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xef4e4 | store | 4 | `sp -0x68` | `0x1` |
| `ldr` at 0xef4ec | load | 4 | `sp -0x50` | `-` |
| `str` at 0xef4f0 | store | 4 | `sp -0x38` | `[sp-80]` |
| `ldr` at 0xef4f8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef4fc | store | 4 | `sp -0x78` | `[sp+8]` |
| `ldr` at 0xef500 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xef504 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xef508 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xef514 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xef518 | store | 4 | `sp -0x78` | `[sp-92]` |
| `ldr` at 0xef51c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef520 | store | 4 | `sp -0x74` | `[sp+8]` |
| `ldr` at 0xef524 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef528 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xef52c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xef530 | store | 4 | `sp -0x6c` | `[sp+16]` |
| `str` at 0xef558 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0xef560 | load | 4 | `abs 0xef798` | `-` |
| `ldr` at 0xef564 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xef570 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef574 | store | 4 | `sp -0x78` | `[sp+12]` |
| `ldr` at 0xef578 | load | 4 | `abs 0xef7b4` | `-` |
| `ldr` at 0xef57c | load | 4 | `abs 0xef7a0` | `-` |
| `ldr` at 0xef580 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xef584 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef590 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0xef594 | load | 4 | `abs 0xef7b8` | `-` |
| `str` at 0xef59c | store | 4 | `abs 0x2ae110` | `0x0` |
| `ldr` at 0xef5a0 | load | 4 | `abs 0xef7b8` | `-` |
| `ldr` at 0xef5a4 | load | 4 | `abs 0x2ae110` | `-` |
| `ldr` at 0xef5b4 | load | 4 | `abs 0xef7b8` | `-` |
| `ldr` at 0xef5b8 | load | 4 | `abs 0x2ae110` | `-` |
| `str` at 0xef5d4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xef5d8 | load | 4 | `abs 0xef7b8` | `-` |
| `ldr` at 0xef5dc | load | 4 | `abs 0xef7b8` | `-` |
| `ldr` at 0xef5e0 | load | 4 | `abs 0x2ae110` | `-` |
| `str` at 0xef5e8 | store | 4 | `abs 0x2ae110` | `[0x2ae110]+1` |
| `str` at 0xef5f8 | store | 4 | `sp -0x78` | `sp-36` |
| `ldr` at 0xef5fc | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef600 | store | 4 | `sp -0x74` | `[sp+8]` |
| `ldr` at 0xef604 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xef608 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef614 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xef620 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xef62c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xef630 | store | 4 | `sp -0x78` | `[sp-36]` |
| `ldr` at 0xef634 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xef638 | store | 4 | `sp -0x74` | `[sp+8]` |
| `ldr` at 0xef63c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef640 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xef644 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xef648 | store | 4 | `sp -0x6c` | `[sp+16]` |
| `str` at 0xef670 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0xef678 | load | 4 | `abs 0xef798` | `-` |
| `ldr` at 0xef67c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xef688 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xef68c | store | 4 | `sp -0x78` | `[sp+12]` |
| `ldr` at 0xef690 | load | 4 | `abs 0xef7bc` | `-` |
| `ldr` at 0xef694 | load | 4 | `abs 0xef7a0` | `-` |
| `ldr` at 0xef698 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xef69c | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xef6a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef6b0 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xef6c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef6d0 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xef6e4 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xef6e8 | load | 4 | `abs 0xef7c0` | `-` |
| `str` at 0xef6f4 | store | 4 | `sp -0x68` | `0x0` |
| `str` at 0xef704 | store | 4 | `sp -0x78` | `0x42` |
| `ldr` at 0xef708 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef70c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xef730 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xef734 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xef738 | store | 4 | `[sp+4] +0x0` | `[sp-68]` |
| `ldr` at 0xef73c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xef740 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xef744 | store | 4 | `[sp] +0x0` | `[sp-72]` |
| `str` at 0xef74c | store | 4 | `sp -0x4c` | `0x1` |
| `ldr` at 0xef750 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xef758 | store | 4 | `sp -0x1c` | `[sp-28]-1` |
| `ldr` at 0xef760 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xef76c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xef770 | load | 4 | `abs 0xef7c4` | `-` |
| `str` at 0xef77c | store | 4 | `sp -0x68` | `0x0` |
| `str` at 0xef788 | store | 4 | `sp -0x68` | `0x1` |
| `ldr` at 0xef78c | load | 4 | `sp -0x68` | `-` |
| `ldmdb` at 0xef790 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xef790 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xef790 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xef230 | `(unknown callee) 0xdd33c` | `0x1` | `0x0` | `[sp+16]` | `?` |
| `bl` at 0xef2d4 | `(unknown callee) 0xebd58` | `[sp-92]` | `[sp-100]` | `[sp-96]` | `sp-56` |
| `bl` at 0xef308 | `(unknown callee) 0xdd8e8` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0xef34c | `(unknown callee) 0xec0d0` | `0x137960` | `0x1378a8` | `[sp-92]` | `[sp+8]` |
| `bl` at 0xef458 | `(unknown callee) 0xebfe0` | `[sp-88]` | `[sp-24]` | `0x1e` | `sp-56` |
| `bl` at 0xef48c | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xef4d0 | `(unknown callee) 0xec0d0` | `0x137970` | `0x1378a8` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xef510 | `(unknown callee) 0xebd58` | `[sp-92]` | `[sp-100]` | `[sp-96]` | `sp-56` |
| `bl` at 0xef544 | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xef588 | `(unknown callee) 0xec0d0` | `0x137980` | `0x1378a8` | `[sp-92]` | `[sp+8]` |
| `bl` at 0xef610 | `(unknown callee) 0xebfe0` | `[sp-88]` | `[sp-24]` | `0x42` | `sp-56` |
| `bl` at 0xef65c | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xef6a0 | `(unknown callee) 0xec0d0` | `0x137990` | `0x1378a8` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xef6b4 | `(unknown callee) 0xe2688` | `[sp-24]+64` | `[sp+12]` | `?` | `[sp-24]+64` |
| `bl` at 0xef6d4 | `(unknown callee) 0xe2688` | `[sp-24]+65` | `[sp+12]` | `?` | `[sp-24]+65` |
| `bl` at 0xef6ec | `(unknown callee) 0x1b598` | `[sp+16]` | `0x1378c4` | `?` | `ret(0xe2688)` |
| `bl` at 0xef718 | `(unknown callee) 0xe2b10` | `[sp-24]` | `[sp+12]` | `0x0` | `sp-72` |
| `bl` at 0xef72c | `(unknown callee) 0xe2cf8` | `0x0` | `0x40` | `sp-72` | `sp-72` |
| `bl` at 0xef774 | `(unknown callee) 0x1b598` | `[sp+16]` | `0x1379a8` | `?` | `[sp-76]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xef270 | `? != 0x6` | `? == 0x6` |
| `bne` at 0xef314 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xef330 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xef498 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xef4b4 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xef4dc | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xef550 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xef56c | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bls` at 0xef5ac | `[0x2ae110] <= 0x2 (unsigned, 32-bit)` | `[0x2ae110] > 0x2 (unsigned, 32-bit)` |
| `ble` at 0xef61c | `[sp-28] <= 0x0 (signed, 32-bit)` | `[sp-28] > 0x0 (signed, 32-bit)` |
| `bne` at 0xef628 | `[sp-76] != 0x0` | `[sp-76] == 0x0` |
| `bne` at 0xef668 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xef684 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xef6c0 | `ret(0xe2688) == 0x0` | `ret(0xe2688) != 0x0` |
| `beq` at 0xef6e0 | `ret(0xe2688) == 0x0` | `ret(0xe2688) != 0x0` |
| `bne` at 0xef768 | `[sp-76] != 0x0` | `[sp-76] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xef790 | `r0` | `?` |

## `PIORead_PIC`

- `PIORead_PIC`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xefee0 to 0xf0640.
  Established. Included four-byte stack assignments at `sp -0x38` retain `0x28000000`, `0x8000000`
  and `0x48000000` (`str` at 0xefffc, `str` at 0xf0160, `str` at 0xf0428). The unnamed `0xebfe0`
  call has `[sp-80]`, `[sp-24]`, `0x40`, `sp-56` (`bl` at 0xf00e0). A later signed branch takes
  `[sp-28] <= 0x0` to `0xf05dc` (`ble` at 0xf0344). Two four-byte pointee assignments retain
  `[sp-68]` through `[sp+4] +0x0` and `[sp-72]` through `[sp] +0x0` (`str` at 0xf05b4, `str` at
  0xf05c0). PIC read results and hardware effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xefef0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xefef4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xefef8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xefefc | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xeff00 | load | 4 | `abs 0xf0610` | `-` |
| `ldr` at 0xeff04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeff24 | load | 4 | `(unknown)` | `-` |
| `str` at 0xeff28 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xeff2c | load | 4 | `abs 0xf0610` | `-` |
| `ldr` at 0xeff30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeff50 | load | 4 | `(unknown)` | `-` |
| `str` at 0xeff54 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xeff58 | load | 4 | `abs 0xf0610` | `-` |
| `ldr` at 0xeff5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeff7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeff84 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xeff8c | load | 1 | `(unknown)` | `-` |
| `str` at 0xeff90 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xeff94 | load | 4 | `abs 0xf0610` | `-` |
| `ldr` at 0xeff98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeffb8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeffc0 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xeffc8 | load | 2 | `(unknown)` | `-` |
| `str` at 0xeffcc | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0xeffd8 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xeffe0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xeffe4 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xeffec | store | 4 | `[sp+4] +0x0` | `0x0` |
| `str` at 0xefff4 | store | 4 | `[sp] +0x0` | `0x0` |
| `str` at 0xefffc | store | 4 | `sp -0x38` | `0x28000000` |
| `ldr` at 0xf0004 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xf0008 | store | 4 | `sp -0x70` | `[sp+8]` |
| `ldr` at 0xf000c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xf0010 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xf0014 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xf0020 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xf0024 | store | 4 | `sp -0x70` | `[sp-84]` |
| `ldr` at 0xf0028 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xf002c | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xf0030 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xf0034 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xf0038 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xf003c | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0xf0064 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xf006c | load | 4 | `abs 0xf0614` | `-` |
| `ldr` at 0xf0070 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xf007c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xf0080 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xf0084 | load | 4 | `abs 0xf0618` | `-` |
| `ldr` at 0xf0088 | load | 4 | `abs 0xf061c` | `-` |
| `ldr` at 0xf008c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xf0090 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xf009c | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xf00a0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf00a4 | store | 4 | `sp -0x34` | `[sp-32]` |
| `str` at 0xf00c4 | store | 4 | `sp -0x70` | `sp-36` |
| `ldr` at 0xf00c8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xf00cc | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xf00d0 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xf00d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf00e4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf00e8 | store | 4 | `sp -0x70` | `[sp-36]` |
| `ldr` at 0xf00ec | load | 4 | `sp +0x8` | `-` |
| `str` at 0xf00f0 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xf00f4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xf00f8 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xf00fc | load | 4 | `sp +0x10` | `-` |
| `str` at 0xf0100 | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0xf0128 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xf0130 | load | 4 | `abs 0xf0614` | `-` |
| `ldr` at 0xf0134 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xf0140 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xf0144 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xf0148 | load | 4 | `abs 0xf0620` | `-` |
| `ldr` at 0xf014c | load | 4 | `abs 0xf061c` | `-` |
| `ldr` at 0xf0150 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf0154 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xf0160 | store | 4 | `sp -0x38` | `0x8000000` |
| `ldr` at 0xf0168 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xf016c | store | 4 | `sp -0x70` | `[sp+8]` |
| `ldr` at 0xf0170 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xf0174 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xf0178 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xf0184 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xf0188 | store | 4 | `sp -0x70` | `[sp-84]` |
| `ldr` at 0xf018c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xf0190 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xf0194 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xf0198 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xf019c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xf01a0 | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0xf01c8 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xf01d0 | load | 4 | `abs 0xf0614` | `-` |
| `ldr` at 0xf01d4 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xf01e0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xf01e4 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xf01e8 | load | 4 | `abs 0xf0624` | `-` |
| `ldr` at 0xf01ec | load | 4 | `abs 0xf061c` | `-` |
| `ldr` at 0xf01f0 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xf01f4 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xf01fc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf020c | store | 4 | `sp -0x60` | `0x1` |
| `str` at 0xf0218 | store | 4 | `sp -0x38` | `0x28000000` |
| `ldr` at 0xf0220 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xf0224 | store | 4 | `sp -0x70` | `[sp+8]` |
| `ldr` at 0xf0228 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xf022c | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xf0230 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xf023c | load | 4 | `sp -0x54` | `-` |
| `str` at 0xf0240 | store | 4 | `sp -0x70` | `[sp-84]` |
| `ldr` at 0xf0244 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xf0248 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xf024c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xf0250 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xf0254 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xf0258 | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0xf0280 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xf0288 | load | 4 | `abs 0xf0614` | `-` |
| `ldr` at 0xf028c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xf0298 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xf029c | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xf02a0 | load | 4 | `abs 0xf0618` | `-` |
| `ldr` at 0xf02a4 | load | 4 | `abs 0xf061c` | `-` |
| `ldr` at 0xf02a8 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xf02ac | load | 4 | `sp +0x8` | `-` |
| `str` at 0xf02b8 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0xf02bc | load | 4 | `abs 0xf0628` | `-` |
| `str` at 0xf02c4 | store | 4 | `abs 0x2ae110` | `0x0` |
| `ldr` at 0xf02c8 | load | 4 | `abs 0xf0628` | `-` |
| `ldr` at 0xf02cc | load | 4 | `abs 0x2ae110` | `-` |
| `ldr` at 0xf02dc | load | 4 | `abs 0xf0628` | `-` |
| `ldr` at 0xf02e0 | load | 4 | `abs 0x2ae110` | `-` |
| `str` at 0xf02fc | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xf0300 | load | 4 | `abs 0xf0628` | `-` |
| `ldr` at 0xf0304 | load | 4 | `abs 0xf0628` | `-` |
| `ldr` at 0xf0308 | load | 4 | `abs 0x2ae110` | `-` |
| `str` at 0xf0310 | store | 4 | `abs 0x2ae110` | `[0x2ae110]+1` |
| `str` at 0xf0320 | store | 4 | `sp -0x70` | `sp-36` |
| `ldr` at 0xf0324 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xf0328 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xf032c | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xf0330 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf033c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf0348 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xf0354 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf0358 | store | 4 | `sp -0x70` | `[sp-36]` |
| `ldr` at 0xf035c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xf0360 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xf0364 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xf0368 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xf036c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xf0370 | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0xf0398 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xf03a0 | load | 4 | `abs 0xf0614` | `-` |
| `ldr` at 0xf03a4 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xf03b0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xf03b4 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xf03b8 | load | 4 | `abs 0xf062c` | `-` |
| `ldr` at 0xf03bc | load | 4 | `abs 0xf061c` | `-` |
| `ldr` at 0xf03c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf03c4 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xf03cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf03d8 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xf03ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf03f8 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xf040c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xf0410 | load | 4 | `abs 0xf0630` | `-` |
| `str` at 0xf041c | store | 4 | `sp -0x60` | `0x0` |
| `str` at 0xf0428 | store | 4 | `sp -0x38` | `0x48000000` |
| `ldr` at 0xf0430 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xf0434 | store | 4 | `sp -0x70` | `[sp+8]` |
| `ldr` at 0xf0438 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xf043c | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xf0440 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xf044c | load | 4 | `sp -0x54` | `-` |
| `str` at 0xf0450 | store | 4 | `sp -0x70` | `[sp-84]` |
| `ldr` at 0xf0454 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xf0458 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xf045c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xf0460 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xf0464 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xf0468 | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0xf0490 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xf0498 | load | 4 | `abs 0xf0614` | `-` |
| `ldr` at 0xf049c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xf04a8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xf04ac | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xf04b0 | load | 4 | `abs 0xf0634` | `-` |
| `ldr` at 0xf04b4 | load | 4 | `abs 0xf061c` | `-` |
| `ldr` at 0xf04b8 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xf04bc | load | 4 | `sp +0x8` | `-` |
| `str` at 0xf04e0 | store | 4 | `sp -0x70` | `sp-36` |
| `ldr` at 0xf04e4 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xf04e8 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xf04ec | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xf04f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0500 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf0504 | store | 4 | `sp -0x70` | `[sp-36]` |
| `ldr` at 0xf0508 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xf050c | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xf0510 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xf0514 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xf0518 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xf051c | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0xf0544 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xf054c | load | 4 | `abs 0xf0614` | `-` |
| `ldr` at 0xf0550 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xf055c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xf0560 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xf0564 | load | 4 | `abs 0xf0638` | `-` |
| `ldr` at 0xf0568 | load | 4 | `abs 0xf061c` | `-` |
| `ldr` at 0xf056c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf0570 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xf0580 | store | 4 | `sp -0x70` | `0x40` |
| `ldr` at 0xf0584 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0588 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xf05ac | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xf05b0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xf05b4 | store | 4 | `[sp+4] +0x0` | `[sp-68]` |
| `ldr` at 0xf05b8 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xf05bc | load | 4 | `sp -0x48` | `-` |
| `str` at 0xf05c0 | store | 4 | `[sp] +0x0` | `[sp-72]` |
| `str` at 0xf05c8 | store | 4 | `sp -0x4c` | `0x1` |
| `ldr` at 0xf05cc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf05d4 | store | 4 | `sp -0x1c` | `[sp-28]-1` |
| `ldr` at 0xf05dc | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xf05e8 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xf05ec | load | 4 | `abs 0xf063c` | `-` |
| `str` at 0xf05f8 | store | 4 | `sp -0x60` | `0x0` |
| `str` at 0xf0604 | store | 4 | `sp -0x60` | `0x1` |
| `ldr` at 0xf0608 | load | 4 | `sp -0x60` | `-` |
| `ldmdb` at 0xf060c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf060c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf060c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeffdc | `(unknown callee) 0xdd33c` | `0x1` | `0x0` | `[sp+16]` | `?` |
| `bl` at 0xf001c | `(unknown callee) 0xebd58` | `[sp-84]` | `[sp-92]` | `[sp-88]` | `sp-56` |
| `bl` at 0xf0050 | `(unknown callee) 0xdd8e8` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0xf0094 | `(unknown callee) 0xec0d0` | `0x137ad4` | `0x1378a8` | `[sp-84]` | `[sp+8]` |
| `bl` at 0xf00b8 | `(unknown callee) 0xe2cf8` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0xf00e0 | `(unknown callee) 0xebfe0` | `[sp-80]` | `[sp-24]` | `0x40` | `sp-56` |
| `bl` at 0xf0114 | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf0158 | `(unknown callee) 0xec0d0` | `0x137ae8` | `0x1378a8` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xf0180 | `(unknown callee) 0xebd58` | `[sp-84]` | `[sp-92]` | `[sp-88]` | `sp-56` |
| `bl` at 0xf01b4 | `(unknown callee) 0xdd8e8` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0xf01f8 | `(unknown callee) 0xec0d0` | `0x137afc` | `0x1378a8` | `[sp-84]` | `[sp+8]` |
| `bl` at 0xf0238 | `(unknown callee) 0xebd58` | `[sp-84]` | `[sp-92]` | `[sp-88]` | `sp-56` |
| `bl` at 0xf026c | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf02b0 | `(unknown callee) 0xec0d0` | `0x137ad4` | `0x1378a8` | `[sp-84]` | `[sp+8]` |
| `bl` at 0xf0338 | `(unknown callee) 0xebfe0` | `[sp-80]` | `[sp-24]` | `0x40` | `sp-56` |
| `bl` at 0xf0384 | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf03c8 | `(unknown callee) 0xec0d0` | `0x137b10` | `0x1378a8` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xf03dc | `(unknown callee) 0xe2688` | `[sp-24]+57` | `[sp+12]` | `?` | `[sp-24]+57` |
| `bl` at 0xf03fc | `(unknown callee) 0xe2688` | `[sp-24]+58` | `[sp+12]` | `?` | `[sp-24]+58` |
| `bl` at 0xf0414 | `(unknown callee) 0x1b598` | `[sp+16]` | `0x137a60` | `?` | `ret(0xe2688)` |
| `bl` at 0xf0448 | `(unknown callee) 0xebd58` | `[sp-84]` | `[sp-92]` | `[sp-88]` | `sp-56` |
| `bl` at 0xf047c | `(unknown callee) 0xdd8e8` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0xf04c0 | `(unknown callee) 0xec0d0` | `0x137b2c` | `0x1378a8` | `[sp-84]` | `[sp+8]` |
| `bl` at 0xf04d4 | `(unknown callee) 0xe2cf8` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0xf04fc | `(unknown callee) 0xebfe0` | `[sp-80]` | `[sp-24]` | `0x40` | `sp-56` |
| `bl` at 0xf0530 | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf0574 | `(unknown callee) 0xec0d0` | `0x137b40` | `0x1378a8` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xf0594 | `(unknown callee) 0xe2b10` | `[sp-24]` | `[sp+12]` | `0x0` | `sp-72` |
| `bl` at 0xf05a8 | `(unknown callee) 0xe2cf8` | `0x0` | `0x40` | `sp-72` | `sp-72` |
| `bl` at 0xf05f0 | `(unknown callee) 0x1b598` | `[sp+16]` | `0x137b54` | `?` | `[sp-76]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf005c | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xf0078 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xf0120 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xf013c | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xf01c0 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xf01dc | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xf0204 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xf0278 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xf0294 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bls` at 0xf02d4 | `[0x2ae110] <= 0x1 (unsigned, 32-bit)` | `[0x2ae110] > 0x1 (unsigned, 32-bit)` |
| `ble` at 0xf0344 | `[sp-28] <= 0x0 (signed, 32-bit)` | `[sp-28] > 0x0 (signed, 32-bit)` |
| `bne` at 0xf0350 | `[sp-76] != 0x0` | `[sp-76] == 0x0` |
| `bne` at 0xf0390 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xf03ac | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xf03e8 | `ret(0xe2688) == 0x0` | `ret(0xe2688) != 0x0` |
| `beq` at 0xf0408 | `ret(0xe2688) == 0x0` | `ret(0xe2688) != 0x0` |
| `bne` at 0xf0488 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xf04a4 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xf053c | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xf0558 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xf05e4 | `[sp-76] != 0x0` | `[sp-76] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf060c | `r0` | `?` |

## `ioExpWrite`

- `ioExpWrite`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd73b0 to 0xd74b0.
  Established. The byte loaded through `ret(0xd8068) +0x0` decides the nonzero branch to `0xd742c`
  (`ldrb` at 0xd7414, `bne` at 0xd741c). The later unnamed `0xd8188` call has `[sp-20]`, `[sp-28]`,
  `[sp-32]`, `[sp-22]:1` (`bl` at 0xd748c). An included four-byte store at `sp -0x2c` retains
  `[sp-23]:1` (`str` at 0xd7478). Expander addressing and physical outputs remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd73c0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd73cc | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xd73d4 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0xd73dc | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldrb` at 0xd73e0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd73e4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd73f4 | store | 4 | `sp -0x1c` | `ret(0xd7f54)` |
| `ldrb` at 0xd73f8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd73fc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd740c | store | 4 | `sp -0x20` | `ret(0xd8068)` |
| `ldr` at 0xd7410 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd7414 | load | 1 | `ret(0xd8068) +0x0` | `-` |
| `ldr` at 0xd7420 | load | 4 | `abs 0xd74ac` | `-` |
| `str` at 0xd7424 | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0xd742c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd7430 | load | 1 | `[sp-28] +0x1` | `-` |
| `ldr` at 0xd7434 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd7454 | store | 4 | `sp -0x24` | `ret(0xc8348)` |
| `ldr` at 0xd7458 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd7464 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd7468 | store | 4 | `sp -0x28` | `ret(0xc8348)` |
| `ldrb` at 0xd7470 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0xd7474 | load | 1 | `sp -0x17` | `-` |
| `str` at 0xd7478 | store | 4 | `sp -0x2c` | `[sp-23]:1` |
| `ldr` at 0xd747c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd7480 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd7484 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd7494 | store | 4 | `sp -0x24` | `ret(0xd8188)` |
| `ldr` at 0xd749c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd74a0 | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0xd74a4 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xd74a8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd74a8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd74a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd73ec | `(unknown callee) 0xd7f54` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xd7404 | `(unknown callee) 0xd8068` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0xd743c | `(unknown callee) 0xc807c` | `[sp-20]` | `[[sp-28]+1]:1` | `?` | `[[sp-28]+1]:1` |
| `bl` at 0xd744c | `(unknown callee) 0xc8348` | `?` | `?` | `?` | `?` |
| `bl` at 0xd748c | `(unknown callee) 0xd8188` | `[sp-20]` | `[sp-28]` | `[sp-32]` | `[sp-22]:1` |
| `bl` at 0xd7498 | `(unknown callee) 0xc8488` | `ret(0xd8188)` | `?` | `?` | `ret(0xd8188)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd741c | `[ret(0xd8068)]:1 != 0x0` | `[ret(0xd8068)]:1 == 0x0` |
| `beq` at 0xd7460 | `ret(0xc8348) == 0x0` | `ret(0xc8348) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd74a8 | `r0` | `?` |

## `JChip_read_dmin_dmax`

- `JChip_read_dmin_dmax`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf40bc to 0xf4360.
  Established. The unnamed `0x23fbc` call has `0x13a288`, `r0`, `r1`, `r2` (`bl` at 0xf40fc). The
  computed transfer takes an unresolved edge for `? <= 0x5`, unsigned 32-bit, and falls through for
  `? > 0x5` (`ldrls` at 0xf4114). Known fall-through writes four-byte `0xfffff7c0` at `sp -0x34`
  (`str` at 0xf41d8). Selector-specific delay measurements and field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf40cc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf40d0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf40d4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xf40d8 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xf40dc | load | 4 | `abs 0xf4348` | `-` |
| `ldr` at 0xf40e0 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xf40ec | load | 4 | `abs 0xf434c` | `-` |
| `ldr` at 0xf40f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf40f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf40f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf4100 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0xf4114 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf41d4 | load | 4 | `abs 0xf4350` | `-` |
| `str` at 0xf41d8 | store | 4 | `sp -0x34` | `0xfffff7c0` |
| `ldr` at 0xf4340 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xf4344 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf4344 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf4344 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf40fc | `(unknown callee) 0x23fbc` | `0x13a288` | `r0` | `r1` | `r2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xf40e8 | `[0x2adfd8] <= 0x1 (unsigned, 32-bit)` | `[0x2adfd8] > 0x1 (unsigned, 32-bit)` |
| `ldrls` at 0xf4114 | `? <= 0x5 (unsigned, 32-bit)` | `? > 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf4344 | `r0` | `0xfffff7c0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xf4114).

## `JChip_read_link_state`

- `JChip_read_link_state`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf4360 to 0xf46fc.
  Established. The unsigned branch takes `[0x2adfd8] <= 0x1` to `0xf43a4` (`bls` at 0xf438c); its
  computed transfer takes an unresolved edge for `? <= 0x5`, unsigned 32-bit, and falls through for
  `? > 0x5` (`ldrls` at 0xf43b8). The known fall-through assigns four-byte `0xfffff7c0` at `sp -0x44` (`str` at 0xf44c0). Link state and selector-dependent operations remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf4370 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf4374 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf4378 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xf437c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xf4380 | load | 4 | `abs 0xf46e0` | `-` |
| `ldr` at 0xf4384 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xf4390 | load | 4 | `abs 0xf46e4` | `-` |
| `ldr` at 0xf4394 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf4398 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf439c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf43a4 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0xf43b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf44bc | load | 4 | `abs 0xf46e8` | `-` |
| `str` at 0xf44c0 | store | 4 | `sp -0x44` | `0xfffff7c0` |
| `ldr` at 0xf46d8 | load | 4 | `sp -0x44` | `-` |
| `ldmdb` at 0xf46dc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf46dc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf46dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf43a0 | `(unknown callee) 0x23fbc` | `0x13a288` | `r0` | `r1` | `r2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xf438c | `[0x2adfd8] <= 0x1 (unsigned, 32-bit)` | `[0x2adfd8] > 0x1 (unsigned, 32-bit)` |
| `ldrls` at 0xf43b8 | `? <= 0x5 (unsigned, 32-bit)` | `? > 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf46dc | `r0` | `0xfffff7c0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xf43b8).

## `JChip_link_summary_f`

- `JChip_link_summary_f`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf46fc to 0xf4bec.
  Established. The first explicit byte assignment clears `r3 +0x0` (`strb` at 0xf474c). The unnamed
  `0xf4360` call has `[sp-20]`, `[sp-24]`, `0x0`, unknown (`bl` at 0xf47cc); its nonnegative signed
  result takes the branch to `0xf47fc` (`bge` at 0xf47e0). A later computed transfer takes an
  unresolved edge for `? <= 0x5`, unsigned 32-bit, and falls through for `? > 0x5` (`ldrls` at
  0xf4824). Summary fields and selector-specific link meaning remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf470c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf4710 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf4714 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xf4718 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xf4720 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xf4728 | store | 4 | `sp -0x5c` | `0x0` |
| `str` at 0xf4730 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xf4734 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf4738 | store | 4 | `sp -0x64` | `r3` |
| `ldr` at 0xf473c | load | 4 | `abs 0xf4bbc` | `-` |
| `str` at 0xf4740 | store | 4 | `sp -0x68` | `0x13a30c` |
| `ldr` at 0xf4744 | load | 4 | `sp -0x64` | `-` |
| `strb` at 0xf474c | store | 1 | `r3 +0x0` | `0x0` |
| `ldr` at 0xf4750 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xf4760 | store | 4 | `sp -0x74` | `0x0` |
| `str` at 0xf4770 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xf4774 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xf4788 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xf4798 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xf47a4 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xf47ac | load | 4 | `sp -0x50` | `-` |
| `str` at 0xf47b8 | store | 4 | `sp -0x84` | `?` |
| `ldr` at 0xf47bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf47c0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf47d4 | store | 4 | `sp -0x4c` | `ret(0xf4360)` |
| `ldr` at 0xf47d8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xf47e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf47e8 | load | 4 | `abs 0xf4bc0` | `-` |
| `ldr` at 0xf47f0 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xf47f4 | store | 4 | `sp -0x74` | `[sp-76]` |
| `ldr` at 0xf47fc | load | 4 | `sp -0x50` | `-` |
| `str` at 0xf4804 | store | 4 | `sp -0x50` | `[sp-80]+1` |
| `ldr` at 0xf4810 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0xf4824 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf485c | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xf4860 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xf4870 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xf4888 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf488c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xf4898 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xf48b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf48b4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xf48c8 | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0xf48cc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xf48d0 | store | 4 | `sp -0x88` | `[sp-60]` |
| `ldr` at 0xf48d4 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xf48d8 | load | 4 | `abs 0xf4bc4` | `-` |
| `ldr` at 0xf48dc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xf48e0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xf48e8 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xf48f8 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xf4900 | store | 4 | `sp -0x64` | `([sp-100] + ret(0x1b2bc))` |
| `ldr` at 0xf4904 | load | 4 | `abs 0xf4bc8` | `-` |
| `str` at 0xf4908 | store | 4 | `sp -0x68` | `0x13a35c` |
| `ldr` at 0xf4910 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xf4918 | store | 4 | `sp -0x50` | `[sp-80]+1` |
| `ldr` at 0xf4920 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xf492c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf4938 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xf493c | load | 4 | `abs 0xf4bcc` | `-` |
| `ldr` at 0xf4944 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xf4954 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xf495c | store | 4 | `sp -0x64` | `([sp-100] + ret(0x1b2bc))` |
| `ldr` at 0xf4960 | load | 4 | `abs 0xf4bc8` | `-` |
| `str` at 0xf4964 | store | 4 | `sp -0x68` | `0x13a35c` |
| `str` at 0xf496c | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xf4970 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xf4980 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xf4998 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf499c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xf49ac | store | 4 | `sp -0x5c` | `0x1` |
| `ldr` at 0xf49b0 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xf49c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf49cc | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf49dc | store | 4 | `sp -0x60` | `0x1` |
| `ldr` at 0xf49e0 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xf49e8 | store | 4 | `sp -0x50` | `[sp-80]+1` |
| `ldr` at 0xf49f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf49fc | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xf4a08 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xf4a14 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xf4a24 | load | 4 | `abs 0xf4bbc` | `-` |
| `str` at 0xf4a28 | store | 4 | `sp -0x6c` | `0x13a30c` |
| `ldr` at 0xf4a2c | load | 4 | `abs 0xf4bbc` | `-` |
| `str` at 0xf4a30 | store | 4 | `sp -0x70` | `0x13a30c` |
| `ldr` at 0xf4a34 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xf4a40 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xf4a4c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xf4a58 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf4a64 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf4a70 | load | 4 | `abs 0xf4bd0` | `-` |
| `str` at 0xf4a74 | store | 4 | `sp -0x6c` | `0x13a370` |
| `ldr` at 0xf4a78 | load | 4 | `abs 0xf4bd4` | `-` |
| `str` at 0xf4a7c | store | 4 | `sp -0x70` | `0x13a380` |
| `ldr` at 0xf4a80 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xf4a8c | load | 4 | `abs 0xf4bd8` | `-` |
| `str` at 0xf4a90 | store | 4 | `sp -0x78` | `0x13a384` |
| `ldr` at 0xf4a98 | load | 4 | `abs 0xf4bdc` | `-` |
| `str` at 0xf4a9c | store | 4 | `sp -0x78` | `0x13a3a4` |
| `ldr` at 0xf4aa0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xf4aa4 | store | 4 | `sp -0x88` | `[sp-44]` |
| `ldr` at 0xf4aa8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf4aac | store | 4 | `sp -0x84` | `[sp-40]` |
| `ldr` at 0xf4ab0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf4ab4 | store | 4 | `sp -0x80` | `[sp-36]` |
| `ldr` at 0xf4ab8 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xf4abc | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xf4ac0 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xf4ac4 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0xf4acc | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xf4adc | load | 4 | `sp -0x64` | `-` |
| `str` at 0xf4ae4 | store | 4 | `sp -0x64` | `([sp-100] + ret(0x1b2bc))` |
| `ldr` at 0xf4ae8 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xf4af4 | load | 4 | `abs 0xf4be0` | `-` |
| `str` at 0xf4af8 | store | 4 | `sp -0x7c` | `0x13a3b8` |
| `ldr` at 0xf4b00 | load | 4 | `abs 0xf4be4` | `-` |
| `str` at 0xf4b04 | store | 4 | `sp -0x7c` | `0x13a3d4` |
| `ldr` at 0xf4b08 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xf4b0c | store | 4 | `sp -0x88` | `[sp-48]` |
| `ldr` at 0xf4b10 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xf4b14 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xf4b18 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf4b1c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf4b24 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xf4b34 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xf4b3c | store | 4 | `sp -0x64` | `([sp-100] + ret(0x1b2bc))` |
| `ldr` at 0xf4b40 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xf4b44 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0xf4b4c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xf4b5c | load | 4 | `sp -0x64` | `-` |
| `str` at 0xf4b64 | store | 4 | `sp -0x64` | `([sp-100] + ret(0x1b2bc))` |
| `ldr` at 0xf4b6c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf4b78 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf4b84 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xf4b88 | load | 4 | `abs 0xf4be8` | `-` |
| `ldr` at 0xf4b90 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xf4ba0 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xf4ba8 | store | 4 | `sp -0x64` | `([sp-100] + ret(0x1b2bc))` |
| `str` at 0xf4bb0 | store | 4 | `sp -0x74` | `0x0` |
| `ldr` at 0xf4bb4 | load | 4 | `sp -0x74` | `-` |
| `ldmdb` at 0xf4bb8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf4bb8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf4bb8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf4768 | `(unknown callee) 0xe3cf8` | `r0` | `r1` | `r3` | `[sp-20]` |
| `bl` at 0xf47cc | `(unknown callee) 0xf4360` | `[sp-20]` | `[sp-24]` | `0x0` | `?` |
| `bl` at 0xf47ec | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x13a310` | `?` | `ret(0xf4360)` |
| `bl` at 0xf480c | `(unknown callee) 0xe3d64` | `?` | `?` | `?` | `?` |
| `bl` at 0xf48e4 | `(unknown callee) 0x1b52c` | `[sp-100]` | `0x13a334` | `[sp-68]` | `[sp-64]` |
| `bl` at 0xf48ec | `(unknown callee) 0x1b2bc` | `[sp-100]` | `?` | `?` | `?` |
| `bl` at 0xf4940 | `(unknown callee) 0x1b52c` | `[sp-100]` | `0x13a360` | `?` | `[sp-28]` |
| `bl` at 0xf4948 | `(unknown callee) 0x1b2bc` | `[sp-100]` | `?` | `?` | `?` |
| `bl` at 0xf4ac8 | `(unknown callee) 0x1b52c` | `[sp-100]` | `[sp-120]` | `[sp-104]` | `?` |
| `bl` at 0xf4ad0 | `(unknown callee) 0x1b2bc` | `[sp-100]` | `?` | `?` | `?` |
| `bl` at 0xf4b20 | `(unknown callee) 0x1b52c` | `([sp-100] + ret(0x1b2bc))` | `[sp-124]` | `[sp-56]` | `[sp-52]` |
| `bl` at 0xf4b28 | `(unknown callee) 0x1b2bc` | `[sp-100]` | `?` | `?` | `?` |
| `bl` at 0xf4b48 | `(unknown callee) 0x1b364` | `([sp-100] + ret(0x1b2bc))` | `[sp-112]` | `ret(0x1b2bc)` | `([sp-100] + ret(0x1b2bc))` |
| `bl` at 0xf4b50 | `(unknown callee) 0x1b2bc` | `[sp-100]` | `?` | `?` | `?` |
| `bl` at 0xf4b8c | `(unknown callee) 0x1b52c` | `[sp-100]` | `0x13a3e0` | `?` | `[sp-56]` |
| `bl` at 0xf4b94 | `(unknown callee) 0x1b2bc` | `[sp-100]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf4758 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `ble` at 0xf477c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bge` at 0xf47e0 | `ret(0xf4360) >= 0x0 (signed, 32-bit)` | `ret(0xf4360) < 0x0 (signed, 32-bit)` |
| `ldrls` at 0xf4824 | `? <= 0x5 (unsigned, 32-bit)` | `? > 0x5 (unsigned, 32-bit)` |
| `ble` at 0xf4868 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xf4894 | `? != [sp-84]` | `? == [sp-84]` |
| `bne` at 0xf48bc | `? != [sp-68]` | `? == [sp-68]` |
| `bne` at 0xf4928 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xf4934 | `[sp-28] != 0x1` | `[sp-28] == 0x1` |
| `ble` at 0xf4978 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xf49a4 | `? == [sp-44]` | `? != [sp-44]` |
| `beq` at 0xf49d4 | `? == [sp-56]` | `? != [sp-56]` |
| `beq` at 0xf49f8 | `[sp-28] == 0x1` | `[sp-28] != 0x1` |
| `bne` at 0xf4a04 | `[sp-88] != 0x0` | `[sp-88] == 0x0` |
| `bne` at 0xf4a10 | `[sp-92] != 0x0` | `[sp-92] == 0x0` |
| `bne` at 0xf4a1c | `[sp-96] != 0x0` | `[sp-96] == 0x0` |
| `bne` at 0xf4a3c | `[sp-92] != 0x0` | `[sp-92] == 0x0` |
| `bne` at 0xf4a48 | `[sp-96] != 0x0` | `[sp-96] == 0x0` |
| `bne` at 0xf4a54 | `[sp-88] != 0x0` | `[sp-88] == 0x0` |
| `bne` at 0xf4a60 | `[sp-44] != 0x0` | `[sp-44] == 0x0` |
| `bne` at 0xf4a6c | `[sp-56] != 0x0` | `[sp-56] == 0x0` |
| `beq` at 0xf4a88 | `[sp-92] == 0x0` | `[sp-92] != 0x0` |
| `beq` at 0xf4af0 | `[sp-96] == 0x0` | `[sp-96] != 0x0` |
| `bne` at 0xf4b74 | `[sp-44] != 0x0` | `[sp-44] == 0x0` |
| `bne` at 0xf4b80 | `[sp-56] != 0x0` | `[sp-56] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf4bb8 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xf4824).

## `pwrStatusLEDInit`

- `pwrStatusLEDInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5f498 to 0x5f524.
  Established. The zero-global branch takes `[0x149194] == 0x0` to `0x5f4f4` (`beq` at 0x5f4bc). Two
  four-byte global assignments target `0x149194` with unknown value and `0x149198` with `[sp-24]`
  (`str` at 0x5f4fc, `str` at 0x5f508). The unnamed `0x5f524` call has unknown, unknown, `0x149198`,
  `[sp-24]` (`bl` at 0x5f50c). LED record meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5f4a8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x5f4ac | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x5f4b0 | load | 4 | `abs 0x5f514` | `-` |
| `ldr` at 0x5f4b4 | load | 4 | `abs 0x149194` | `-` |
| `ldr` at 0x5f4c0 | load | 4 | `abs 0x5f518` | `-` |
| `str` at 0x5f4c4 | store | 4 | `sp -0x24` | `0x10cf30` |
| `ldr` at 0x5f4c8 | load | 4 | `abs 0x5f514` | `-` |
| `ldr` at 0x5f4cc | load | 4 | `abs 0x149194` | `-` |
| `str` at 0x5f4d0 | store | 4 | `sp -0x20` | `[0x149194]` |
| `ldr` at 0x5f4d4 | load | 4 | `abs 0x5f51c` | `-` |
| `ldr` at 0x5f4d8 | load | 4 | `abs 0x149198` | `-` |
| `str` at 0x5f4dc | store | 4 | `sp -0x1c` | `[0x149198]` |
| `ldr` at 0x5f4e4 | load | 4 | `abs 0x5f520` | `-` |
| `ldr` at 0x5f4f4 | load | 4 | `abs 0x5f514` | `-` |
| `ldr` at 0x5f4f8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5f4fc | store | 4 | `abs 0x149194` | `?` |
| `ldr` at 0x5f500 | load | 4 | `abs 0x5f51c` | `-` |
| `ldr` at 0x5f504 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5f508 | store | 4 | `abs 0x149198` | `[sp-24]` |
| `ldmdb` at 0x5f510 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5f510 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5f510 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5f4f0 | `(unknown callee) 0x21eb8` | `0x9` | `0x10cf58` | `0x2c` | `0x0` |
| `bl` at 0x5f50c | `(unknown callee) 0x5f524` | `?` | `?` | `0x149198` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5f4bc | `[0x149194] == 0x0` | `[0x149194] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5f510 | `r0` | `ret(0x5f524)` |

## `pwrStatusLEDUpdate`

- `pwrStatusLEDUpdate`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5f524 to 0x5f764.
  Established. The unnamed `0x3e05c` call carries `0x0` followed by three unknown candidates (`bl`
  at 0x5f5b0). Separate equality branches take its result equal to `0x2` and `0xfffff9b9` to
  `0x5f5e4` (`beq` at 0x5f5c4, `beq` at 0x5f5d4). Included byte reads have unknown effective targets
  (`ldrb` at 0x5f578, `ldrb` at 0x5f594). Status encoding and LED output semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5f538 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x5f540 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x5f544 | load | 4 | `abs 0x5f758` | `-` |
| `ldr` at 0x5f548 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5f54c | load | 4 | `abs 0x149198` | `-` |
| `str` at 0x5f560 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x5f564 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5f56c | load | 4 | `abs 0x5f75c` | `-` |
| `ldr` at 0x5f570 | load | 4 | `abs 0x149194` | `-` |
| `ldrb` at 0x5f578 | load | 1 | `(unknown)` | `-` |
| `str` at 0x5f57c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x5f580 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5f588 | load | 4 | `abs 0x5f75c` | `-` |
| `ldr` at 0x5f58c | load | 4 | `abs 0x149194` | `-` |
| `ldrb` at 0x5f594 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5f598 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5f5ac | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5f5b8 | store | 4 | `sp -0x20` | `ret(0x3e05c)` |
| `ldr` at 0x5f5bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5f5c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5f5cc | load | 4 | `abs 0x5f760` | `-` |
| `ldr` at 0x5f5d8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5f5e0 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x5f5e4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5f5ec | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x5f5f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5f600 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5f608 | load | 4 | `abs 0x5f75c` | `-` |
| `ldr` at 0x5f60c | load | 4 | `abs 0x149194` | `-` |
| `ldrb` at 0x5f614 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5f620 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5f628 | load | 4 | `abs 0x5f75c` | `-` |
| `ldr` at 0x5f62c | load | 4 | `abs 0x149194` | `-` |
| `ldrb` at 0x5f634 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5f638 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5f640 | load | 4 | `abs 0x5f75c` | `-` |
| `ldr` at 0x5f644 | load | 4 | `abs 0x149194` | `-` |
| `ldrb` at 0x5f64c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5f660 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5f668 | load | 4 | `abs 0x5f75c` | `-` |
| `ldr` at 0x5f66c | load | 4 | `abs 0x149194` | `-` |
| `ldrb` at 0x5f674 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5f678 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5f680 | load | 4 | `abs 0x5f75c` | `-` |
| `ldr` at 0x5f684 | load | 4 | `abs 0x149194` | `-` |
| `ldrb` at 0x5f68c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5f69c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5f6a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5f6b0 | load | 4 | `abs 0x5f75c` | `-` |
| `ldr` at 0x5f6b4 | load | 4 | `abs 0x149194` | `-` |
| `ldrb` at 0x5f6bc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5f6c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5f6d0 | load | 4 | `abs 0x5f75c` | `-` |
| `ldr` at 0x5f6d4 | load | 4 | `abs 0x149194` | `-` |
| `ldrb` at 0x5f6dc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5f6e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5f6e8 | load | 4 | `abs 0x5f75c` | `-` |
| `ldr` at 0x5f6ec | load | 4 | `abs 0x149194` | `-` |
| `ldrb` at 0x5f6f4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5f708 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5f710 | load | 4 | `abs 0x5f75c` | `-` |
| `ldr` at 0x5f714 | load | 4 | `abs 0x149194` | `-` |
| `ldrb` at 0x5f71c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5f720 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5f728 | load | 4 | `abs 0x5f75c` | `-` |
| `ldr` at 0x5f72c | load | 4 | `abs 0x149194` | `-` |
| `ldrb` at 0x5f734 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5f744 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5f74c | store | 4 | `sp -0x14` | `?` |
| `ldmdb` at 0x5f754 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5f754 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5f754 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5f5b0 | `(unknown callee) 0x3e05c` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x5f658 | `(unknown callee) 0xd731c` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x5f698 | `(unknown callee) 0xd7368` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x5f700 | `(unknown callee) 0xd7368` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x5f740 | `(unknown callee) 0xd731c` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x5f554 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `ble` at 0x5f5a0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x5f5c4 | `ret(0x3e05c) == 0x2` | `ret(0x3e05c) != 0x2` |
| `beq` at 0x5f5d4 | `ret(0x3e05c) == 0xfffff9b9` | `ret(0x3e05c) != 0xfffff9b9` |
| `bne` at 0x5f5fc | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x5f61c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x5f6a4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x5f6c4 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5f754 | `r0` | `?` |
