# ARM FRU event packing and reporting in 1.44.0

Read from `6.5.30/usr/cpu/firmware/sysco/l1.bin`, package `6.5.30:l1#4` (version 1.44.0),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53d154`.
Carrier SHA-256: `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
Package SHA-256: `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
Inner executable SHA-256: `8d492b1d00a2fa93aebb48890caf8cacc5a9747e79209e93b841378aabd75162`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1166540 bytes.
Segment SHA-256: `7f2caa3b9c656f2cc1caf5bdaf139038c25b4a58ac8ff8a26ea044818245a46d`. Analysed routine spans use the exact
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
Computed transfers leave unresolved edges in the display and port-string routines, identified below. Tables cover known control-flow edges only.
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/fru-event-packing-report-facts.txt) retain all tool sections.

## `FRUEventRank`

- `FRUEventRank`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb3f94 to 0xb427c.
  Established. Included halfword stores write `([sp-32] & 0xffff)` to `[sp-20] +0x1c` and an unknown value to `[sp-20] +0x1e` (`strh` at 0xb40c4, `strh` at 0xb41a0).
  A further halfword store has an unknown value and effective target (`strh` at 0xb4260).
  Event ranking interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb3fa4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb3fac | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xb3fb4 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xb3fbc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xb3fc0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb3fc4 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xb3fc8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb3fcc | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0xb3fd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3fdc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3ff8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb4014 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb4018 | load | 1 | `[sp-20] +0x26` | `-` |
| `ldr` at 0xb4024 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb4028 | load | 1 | `[sp-20] +0x26` | `-` |
| `ldr` at 0xb4034 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb403c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xb4040 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb4044 | load | 4 | `[sp-20] +0xa8` | `-` |
| `str` at 0xb4048 | store | 4 | `sp -0x14` | `[[sp-20]+168]` |
| `ldr` at 0xb4050 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb4054 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xb4058 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb4064 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb4080 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb409c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb40a0 | load | 1 | `[sp-20] +0x26` | `-` |
| `ldr` at 0xb40ac | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb40b0 | load | 1 | `[sp-20] +0x26` | `-` |
| `ldr` at 0xb40bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb40c0 | load | 4 | `sp -0x20` | `-` |
| `strh` at 0xb40c4 | store | 2 | `[sp-20] +0x1c` | `([sp-32] & 0xffff)` |
| `ldr` at 0xb40c8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb40d0 | store | 4 | `sp -0x20` | `[sp-32]-1` |
| `ldr` at 0xb40d4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb40d8 | store | 4 | `sp -0x1c` | `[sp-24]` |
| `str` at 0xb40e0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb40e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb40f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb410c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4128 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb412c | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0xb4138 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb413c | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0xb4148 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb414c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xb4150 | load | 2 | `[sp-28] +0x18` | `-` |
| `ldrh` at 0xb4154 | load | 2 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xb4160 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4164 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xb4168 | load | 2 | `[sp-28] +0x1a` | `-` |
| `ldrh` at 0xb416c | load | 2 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xb417c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb4184 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xb4188 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb418c | load | 4 | `[sp-28] +0xa8` | `-` |
| `str` at 0xb4190 | store | 4 | `sp -0x1c` | `[[sp-28]+168]` |
| `ldr` at 0xb4198 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb419c | load | 4 | `sp -0x24` | `-` |
| `strh` at 0xb41a0 | store | 2 | `[sp-20] +0x1e` | `?` |
| `ldr` at 0xb41a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb41a8 | load | 4 | `[sp-20] +0xa8` | `-` |
| `str` at 0xb41ac | store | 4 | `sp -0x14` | `[[sp-20]+168]` |
| `ldr` at 0xb41b4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb41b8 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xb41bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb41cc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xb41d0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb41d4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xb41d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb41e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb41e8 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xb41ec | load | 2 | `(unknown)` | `-` |
| `ldrh` at 0xb41f0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb41fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4200 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xb4204 | load | 2 | `(unknown)` | `-` |
| `ldrh` at 0xb4208 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb4218 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xb421c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb4228 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb422c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb4230 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb423c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb4240 | load | 2 | `(unknown)` | `-` |
| `str` at 0xb4244 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb4248 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb424c | load | 4 | `(unknown)` | `-` |
| `str` at 0xb4250 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xb4258 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb425c | load | 4 | `sp -0x28` | `-` |
| `strh` at 0xb4260 | store | 2 | `(unknown)` | `?` |
| `ldr` at 0xb4264 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb4268 | load | 4 | `[sp-20] +0xa8` | `-` |
| `str` at 0xb426c | store | 4 | `sp -0x14` | `[[sp-20]+168]` |
| `ldm` at 0xb4278 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb4278 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb4278 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb3fe8 | `(unknown callee) 0xb0840` | `?` | `0x3f` | `0x20` | `?` |
| `bl` at 0xb4004 | `(unknown callee) 0xb0840` | `[sp-20]` | `0x1f` | `0x0` | `ret(0xb0840)` |
| `bl` at 0xb4070 | `(unknown callee) 0xb0840` | `?` | `0x3f` | `0x20` | `?` |
| `bl` at 0xb408c | `(unknown callee) 0xb0840` | `[sp-20]` | `0x1f` | `0x0` | `ret(0xb0840)` |
| `bl` at 0xb40fc | `(unknown callee) 0xb0840` | `?` | `0x3f` | `0x20` | `?` |
| `bl` at 0xb4118 | `(unknown callee) 0xb0840` | `[sp-28]` | `0x1f` | `0x0` | `ret(0xb0840)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb3fd8 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb3ff4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb4010 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb4020 | `[[sp-20]+38]:1 == 0x4` | `[[sp-20]+38]:1 != 0x4` |
| `beq` at 0xb4030 | `[[sp-20]+38]:1 == 0x8` | `[[sp-20]+38]:1 != 0x8` |
| `beq` at 0xb4060 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb407c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb4098 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb40a8 | `[[sp-20]+38]:1 == 0x4` | `[[sp-20]+38]:1 != 0x4` |
| `beq` at 0xb40b8 | `[[sp-20]+38]:1 == 0x8` | `[[sp-20]+38]:1 != 0x8` |
| `beq` at 0xb40ec | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb4108 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb4124 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb4134 | `[[sp-28]+38]:1 == 0x4` | `[[sp-28]+38]:1 != 0x4` |
| `beq` at 0xb4144 | `[[sp-28]+38]:1 == 0x8` | `[[sp-28]+38]:1 != 0x8` |
| `beq` at 0xb415c | `[[sp-28]+24]:2 == [[sp-20]+24]:2` | `[[sp-28]+24]:2 != [[sp-20]+24]:2` |
| `beq` at 0xb4174 | `[[sp-28]+26]:2 == [[sp-20]+24]:2` | `[[sp-28]+26]:2 != [[sp-20]+24]:2` |
| `beq` at 0xb41c4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb41e0 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb41f8 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb4210 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb4224 | `Z == 1` | `not (Z == 1)` |
| `bls` at 0xb4238 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb4278 | `r0` | `?` |

## `FRUEventPrint`

- `FRUEventPrint`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb427c to 0xb4630.
  Established. A nonzero entry `r0` takes `0xb4368`, while zero falls through (`bne` at 0xb42b4).
  An indirect call with unknown target carries `[sp-28]`, `sp-164`, `ret(0x19538)`, `sp-164` (`mov` at 0xb42fc).
  The unnamed `0xb4630` call has four unknown candidates (`bl` at 0xb45cc).
  Event report callback arity remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb428c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb4290 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb4294 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb4298 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xb42a0 | store | 4 | `sp -0xa8` | `0x0` |
| `ldr` at 0xb42a4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb42a8 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0xb42ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb42b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb42cc | load | 4 | `abs 0xb45f8` | `-` |
| `ldr` at 0xb42ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb42f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4304 | load | 4 | `abs 0xb45fc` | `-` |
| `str` at 0xb4308 | store | 4 | `sp -0xb8` | `0x11e41c` |
| `ldr` at 0xb430c | load | 4 | `abs 0xb4600` | `-` |
| `str` at 0xb4310 | store | 4 | `sp -0xb4` | `0x11e424` |
| `ldr` at 0xb4314 | load | 4 | `abs 0xb4604` | `-` |
| `str` at 0xb4318 | store | 4 | `sp -0xb0` | `0x11e42c` |
| `ldr` at 0xb431c | load | 4 | `abs 0xb4608` | `-` |
| `str` at 0xb4320 | store | 4 | `sp -0xac` | `0x11e434` |
| `ldr` at 0xb4328 | load | 4 | `abs 0xb460c` | `-` |
| `ldr` at 0xb432c | load | 4 | `abs 0xb4610` | `-` |
| `ldr` at 0xb4330 | load | 4 | `abs 0xb4614` | `-` |
| `ldr` at 0xb4350 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb4354 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4368 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb4374 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb4388 | load | 4 | `abs 0xb4618` | `-` |
| `ldr` at 0xb43a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb43ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb43bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb43c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb43d4 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xb43d8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb43e4 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xb43e8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb43ec | load | 4 | `abs 0xb461c` | `-` |
| `ldr` at 0xb43fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb4404 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb440c | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xb4410 | load | 2 | `(unknown)` | `-` |
| `str` at 0xb4414 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0xb4418 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xb441c | load | 2 | `(unknown)` | `-` |
| `str` at 0xb4420 | store | 4 | `sp -0xb4` | `?` |
| `ldr` at 0xb4424 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xb4428 | load | 2 | `(unknown)` | `-` |
| `str` at 0xb442c | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0xb4430 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xb4434 | load | 2 | `(unknown)` | `-` |
| `str` at 0xb4438 | store | 4 | `sp -0xac` | `?` |
| `ldr` at 0xb4440 | load | 4 | `abs 0xb4620` | `-` |
| `ldr` at 0xb4458 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb4460 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb4468 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xb446c | load | 2 | `(unknown)` | `-` |
| `str` at 0xb4470 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0xb4474 | load | 4 | `abs 0xb4624` | `-` |
| `str` at 0xb4478 | store | 4 | `sp -0xb4` | `0x11e4d0` |
| `ldr` at 0xb447c | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xb4480 | load | 2 | `(unknown)` | `-` |
| `str` at 0xb4484 | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0xb4488 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xb448c | load | 2 | `(unknown)` | `-` |
| `str` at 0xb4490 | store | 4 | `sp -0xac` | `?` |
| `ldr` at 0xb4498 | load | 4 | `abs 0xb4628` | `-` |
| `ldr` at 0xb44c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb44c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb44d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb44e4 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xb44e8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb44ec | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xb44f0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb44f8 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0xb4504 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xb4508 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb450c | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xb4510 | load | 2 | `(unknown)` | `-` |
| `str` at 0xb4518 | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0xb451c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb4520 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb4524 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xb452c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb4538 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0xb454c | load | 4 | `abs 0xb462c` | `-` |
| `ldr` at 0xb456c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb4570 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4580 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0xb458c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb4590 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xb4594 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb45a0 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xb45a4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb45a8 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xb45ac | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb45b4 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0xb45c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb45c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb45c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb45d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb45d4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb45d8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xb45e0 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0xb45e8 | store | 4 | `sp -0xa8` | `?` |
| `ldm` at 0xb45f4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb45f4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb45f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb42d0 | `(unknown callee) 0x197a8` | `sp-164` | `0x11e3f0` | `r2` | `sp-164` |
| `bl` at 0xb42dc | `(unknown callee) 0x19538` | `sp-164` | `?` | `?` | `sp-164` |
| `mov` at 0xb42fc | `(unknown callee) (unknown target)` | `[sp-28]` | `sp-164` | `ret(0x19538)` | `sp-164` |
| `bl` at 0xb4334 | `(unknown callee) 0x197a8` | `sp-164` | `0x11e438` | `0x11e45c` | `0x11e460` |
| `bl` at 0xb4340 | `(unknown callee) 0x19538` | `sp-164` | `?` | `?` | `sp-164` |
| `mov` at 0xb4360 | `(unknown callee) (unknown target)` | `[sp-28]` | `sp-164` | `ret(0x19538)` | `sp-164` |
| `bl` at 0xb438c | `(unknown callee) 0x197a8` | `sp-164` | `0x11e468` | `r2` | `sp-164` |
| `bl` at 0xb4398 | `(unknown callee) 0x19538` | `sp-164` | `?` | `?` | `sp-164` |
| `mov` at 0xb43b8 | `(unknown callee) (unknown target)` | `[sp-28]` | `sp-164` | `ret(0x19538)` | `sp-164` |
| `bl` at 0xb444c | `(unknown callee) 0x197a8` | `sp-164` | `0x11e4ac` | `?` | `?` |
| `bl` at 0xb44a4 | `(unknown callee) 0x197a8` | `sp-164` | `0x11e4d8` | `?` | `?` |
| `bl` at 0xb44b0 | `(unknown callee) 0x19538` | `sp-164` | `?` | `?` | `sp-164` |
| `mov` at 0xb44d0 | `(unknown callee) (unknown target)` | `[sp-28]` | `sp-164` | `ret(0x19538)` | `sp-164` |
| `bl` at 0xb4550 | `(unknown callee) 0x197a8` | `sp-164` | `0x11e4fc` | `?` | `sp-164` |
| `bl` at 0xb455c | `(unknown callee) 0x19538` | `sp-164` | `?` | `?` | `sp-164` |
| `mov` at 0xb457c | `(unknown callee) (unknown target)` | `[sp-28]` | `sp-164` | `ret(0x19538)` | `sp-164` |
| `bl` at 0xb45cc | `(unknown callee) 0xb4630` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb42b4 | `r0 != 0x0` | `r0 == 0x0` |
| `beq` at 0xb42c0 | `r3 == 0x0` | `r3 != 0x0` |
| `bne` at 0xb4370 | `r0 != 0x2` | `r0 == 0x2` |
| `beq` at 0xb437c | `r3 == 0x0` | `r3 != 0x0` |
| `beq` at 0xb43c4 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xb43d0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xb43e0 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb43f4 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xb44e0 | `Z == 0` | `not (Z == 0)` |
| `bls` at 0xb4500 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xb4534 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xb4540 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xb4588 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb459c | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xb45bc | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb45f4 | `r0` | `?` |

## `FRUEventGuess`

- `FRUEventGuess`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb4630 to 0xb51b8.
  Established. A byte read addresses `[sp-28] +0xd` (`ldrb` at 0xb4660).
  A nonzero `[[sp-28]+13]:1` takes `0xb467c`, while zero falls through (`bne` at 0xb4668).
  An indirect call with unknown target carries `[sp-24]`, `sp-924`, `ret(0x19538)`, `sp-924` (`mov` at 0xb4e78).
  Event diagnosis string interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb4640 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb4644 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb4648 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xb4654 | load | 4 | `abs 0xb515c` | `-` |
| `ldr` at 0xb465c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4660 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xb4674 | load | 4 | `abs 0xb5160` | `-` |
| `ldr` at 0xb467c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4680 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xb4698 | load | 4 | `abs 0xb515c` | `-` |
| `ldr` at 0xb46b4 | load | 4 | `abs 0xb5164` | `-` |
| `ldr` at 0xb46c4 | load | 4 | `abs 0xb5168` | `-` |
| `ldr` at 0xb46cc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb46d0 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xb46e8 | load | 4 | `abs 0xb515c` | `-` |
| `ldr` at 0xb4704 | load | 4 | `abs 0xb5164` | `-` |
| `ldr` at 0xb4714 | load | 4 | `abs 0xb516c` | `-` |
| `ldr` at 0xb471c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4720 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xb4738 | load | 4 | `abs 0xb515c` | `-` |
| `ldr` at 0xb4754 | load | 4 | `abs 0xb5164` | `-` |
| `ldr` at 0xb4764 | load | 4 | `abs 0xb5170` | `-` |
| `ldr` at 0xb476c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4770 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xb4788 | load | 4 | `abs 0xb515c` | `-` |
| `ldr` at 0xb47a4 | load | 4 | `abs 0xb5164` | `-` |
| `ldr` at 0xb47b4 | load | 4 | `abs 0xb5174` | `-` |
| `ldr` at 0xb47bc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb47c0 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xb47d8 | load | 4 | `abs 0xb515c` | `-` |
| `ldr` at 0xb47f4 | load | 4 | `abs 0xb5164` | `-` |
| `ldr` at 0xb4804 | load | 4 | `abs 0xb5178` | `-` |
| `ldr` at 0xb480c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4810 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xb4828 | load | 4 | `abs 0xb515c` | `-` |
| `ldr` at 0xb4844 | load | 4 | `abs 0xb5164` | `-` |
| `ldr` at 0xb4854 | load | 4 | `abs 0xb517c` | `-` |
| `ldr` at 0xb4864 | load | 4 | `abs 0xb515c` | `-` |
| `ldr` at 0xb486c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb4870 | load | 2 | `[sp-28] +0x1a` | `-` |
| `ldr` at 0xb4874 | load | 4 | `abs 0xb5180` | `-` |
| `ldr` at 0xb4880 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4884 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xb4894 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb4898 | load | 2 | `[sp-28] +0x1a` | `-` |
| `str` at 0xb48a4 | store | 4 | `sp -0x3a4` | `?` |
| `ldr` at 0xb48a8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb48ac | load | 1 | `[sp-28] +0x21` | `-` |
| `ldr` at 0xb48b8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb48bc | load | 1 | `[sp-28] +0x22` | `-` |
| `ldr` at 0xb48c8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb48cc | load | 1 | `[sp-28] +0x23` | `-` |
| `ldr` at 0xb48d8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb48dc | load | 1 | `[sp-28] +0x24` | `-` |
| `ldr` at 0xb48f0 | load | 4 | `abs 0xb5184` | `-` |
| `ldr` at 0xb4900 | load | 4 | `abs 0xb5188` | `-` |
| `ldr` at 0xb4904 | load | 4 | `sp -0x3a4` | `-` |
| `ldr` at 0xb4910 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4914 | load | 1 | `[sp-28] +0x21` | `-` |
| `ldr` at 0xb4920 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4924 | load | 1 | `[sp-28] +0x22` | `-` |
| `ldr` at 0xb4930 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4934 | load | 1 | `[sp-28] +0x23` | `-` |
| `ldr` at 0xb4940 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4944 | load | 1 | `[sp-28] +0x24` | `-` |
| `ldr` at 0xb4958 | load | 4 | `abs 0xb518c` | `-` |
| `ldr` at 0xb4968 | load | 4 | `abs 0xb5188` | `-` |
| `ldr` at 0xb496c | load | 4 | `sp -0x3a4` | `-` |
| `ldr` at 0xb4980 | load | 4 | `abs 0xb5188` | `-` |
| `ldr` at 0xb4984 | load | 4 | `sp -0x3a4` | `-` |
| `ldr` at 0xb498c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4990 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xb49a8 | load | 4 | `abs 0xb5190` | `-` |
| `ldr` at 0xb49b8 | load | 4 | `abs 0xb515c` | `-` |
| `ldr` at 0xb49c8 | load | 4 | `abs 0xb515c` | `-` |
| `ldr` at 0xb49d8 | load | 4 | `abs 0xb515c` | `-` |
| `ldr` at 0xb49e8 | load | 4 | `abs 0xb515c` | `-` |
| `ldr` at 0xb49f0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb49f4 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xb4a0c | load | 4 | `abs 0xb5194` | `-` |
| `ldr` at 0xb4a1c | load | 4 | `abs 0xb5194` | `-` |
| `ldr` at 0xb4a2c | load | 4 | `abs 0xb5194` | `-` |
| `ldr` at 0xb4a3c | load | 4 | `abs 0xb5194` | `-` |
| `str` at 0xb4a48 | store | 4 | `sp -0x3a8` | `0x8` |
| `ldr` at 0xb4a4c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4a50 | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0xb4a5c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4a60 | load | 1 | `[sp-28] +0x14` | `-` |
| `ldr` at 0xb4a6c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4a70 | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0xb4a78 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4a7c | load | 1 | `[sp-28] +0x14` | `-` |
| `str` at 0xb4a84 | store | 4 | `sp -0x3a8` | `?` |
| `str` at 0xb4a8c | store | 4 | `sp -0x3ac` | `0x3` |
| `str` at 0xb4a94 | store | 4 | `sp -0x3b0` | `0x3` |
| `str` at 0xb4a9c | store | 4 | `sp -0x3b4` | `0x3` |
| `str` at 0xb4aa4 | store | 4 | `sp -0x3b8` | `0x3` |
| `ldr` at 0xb4aa8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4aac | load | 1 | `[sp-28] +0x15` | `-` |
| `ldr` at 0xb4ab8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4abc | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0xb4ac8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4acc | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0xb4adc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4ae0 | load | 1 | `[sp-28] +0x15` | `-` |
| `str` at 0xb4ae4 | store | 4 | `sp -0x3ac` | `[[sp-28]+21]:1` |
| `ldr` at 0xb4ae8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4aec | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0xb4af8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4afc | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0xb4b0c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4b10 | load | 1 | `[sp-28] +0x21` | `-` |
| `ldr` at 0xb4b1c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4b20 | load | 1 | `[sp-28] +0x21` | `-` |
| `str` at 0xb4b30 | store | 4 | `sp -0x3ac` | `ret(0xb3a8c)` |
| `ldr` at 0xb4b34 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4b38 | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0xb4b44 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4b48 | load | 1 | `[sp-28] +0x22` | `-` |
| `ldr` at 0xb4b54 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4b58 | load | 1 | `[sp-28] +0x22` | `-` |
| `str` at 0xb4b68 | store | 4 | `sp -0x3b0` | `ret(0xb3a8c)` |
| `ldr` at 0xb4b6c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4b70 | load | 1 | `[sp-28] +0x23` | `-` |
| `ldr` at 0xb4b7c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4b80 | load | 1 | `[sp-28] +0x23` | `-` |
| `str` at 0xb4b90 | store | 4 | `sp -0x3b4` | `ret(0xb3a8c)` |
| `ldr` at 0xb4b94 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4b98 | load | 1 | `[sp-28] +0x24` | `-` |
| `ldr` at 0xb4ba4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4ba8 | load | 1 | `[sp-28] +0x24` | `-` |
| `str` at 0xb4bb8 | store | 4 | `sp -0x3b8` | `ret(0xb3a8c)` |
| `ldr` at 0xb4bbc | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xb4bc8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4bcc | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0xb4bd8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4bdc | load | 1 | `[sp-28] +0x21` | `-` |
| `ldr` at 0xb4be8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4bec | load | 1 | `[sp-28] +0x22` | `-` |
| `ldr` at 0xb4bf8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4bfc | load | 1 | `[sp-28] +0x23` | `-` |
| `ldr` at 0xb4c08 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4c0c | load | 1 | `[sp-28] +0x24` | `-` |
| `ldr` at 0xb4c1c | load | 4 | `abs 0xb5198` | `-` |
| `ldr` at 0xb4c20 | load | 4 | `sp -0x3ac` | `-` |
| `ldrb` at 0xb4c28 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb4c2c | load | 4 | `abs 0xb519c` | `-` |
| `ldr` at 0xb4c30 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4c34 | load | 1 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0xb4c38 | load | 1 | `[[sp-28]+20]:1 +0x114a78` | `-` |
| `str` at 0xb4c3c | store | 4 | `sp -0x3c0` | `[[[sp-28]+20]:1+1133176]:1` |
| `ldr` at 0xb4c44 | load | 4 | `abs 0xb51a0` | `-` |
| `ldr` at 0xb4c48 | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xb4c5c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4c60 | load | 1 | `[sp-28] +0xc` | `-` |
| `ldr` at 0xb4c64 | load | 4 | `abs 0xb5198` | `-` |
| `ldr` at 0xb4c68 | load | 4 | `sp -0x3ac` | `-` |
| `ldrb` at 0xb4c70 | load | 1 | `(unknown)` | `-` |
| `str` at 0xb4c74 | store | 4 | `sp -0x3c0` | `?` |
| `ldr` at 0xb4c78 | load | 4 | `abs 0xb519c` | `-` |
| `ldr` at 0xb4c7c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4c80 | load | 1 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0xb4c84 | load | 1 | `[[sp-28]+20]:1 +0x114a78` | `-` |
| `str` at 0xb4c88 | store | 4 | `sp -0x3bc` | `[[[sp-28]+20]:1+1133176]:1` |
| `ldr` at 0xb4c90 | load | 4 | `abs 0xb51a4` | `-` |
| `ldr` at 0xb4c94 | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xb4ca4 | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xb4cb0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4cb4 | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0xb4cc0 | load | 4 | `sp -0x3ac` | `-` |
| `ldr` at 0xb4cd0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4cd4 | load | 1 | `[sp-28] +0xc` | `-` |
| `ldr` at 0xb4cd8 | load | 4 | `abs 0xb5198` | `-` |
| `ldr` at 0xb4cdc | load | 4 | `sp -0x3ac` | `-` |
| `ldrb` at 0xb4ce4 | load | 1 | `(unknown)` | `-` |
| `str` at 0xb4ce8 | store | 4 | `sp -0x3c0` | `?` |
| `ldr` at 0xb4cec | load | 4 | `abs 0xb519c` | `-` |
| `ldr` at 0xb4cf0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4cf4 | load | 1 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0xb4cf8 | load | 1 | `[[sp-28]+20]:1 +0x114a78` | `-` |
| `str` at 0xb4cfc | store | 4 | `sp -0x3bc` | `[[[sp-28]+20]:1+1133176]:1` |
| `ldr` at 0xb4d04 | load | 4 | `abs 0xb51a4` | `-` |
| `ldr` at 0xb4d08 | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xb4d14 | load | 4 | `sp -0x3b0` | `-` |
| `ldr` at 0xb4d24 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4d28 | load | 1 | `[sp-28] +0xc` | `-` |
| `ldr` at 0xb4d2c | load | 4 | `abs 0xb5198` | `-` |
| `ldr` at 0xb4d30 | load | 4 | `sp -0x3b0` | `-` |
| `ldrb` at 0xb4d38 | load | 1 | `(unknown)` | `-` |
| `str` at 0xb4d3c | store | 4 | `sp -0x3c0` | `?` |
| `ldr` at 0xb4d40 | load | 4 | `abs 0xb519c` | `-` |
| `ldr` at 0xb4d44 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4d48 | load | 1 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0xb4d4c | load | 1 | `[[sp-28]+20]:1 +0x114a78` | `-` |
| `str` at 0xb4d50 | store | 4 | `sp -0x3bc` | `[[[sp-28]+20]:1+1133176]:1` |
| `ldr` at 0xb4d58 | load | 4 | `abs 0xb51a4` | `-` |
| `ldr` at 0xb4d5c | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xb4d68 | load | 4 | `sp -0x3b4` | `-` |
| `ldr` at 0xb4d78 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4d7c | load | 1 | `[sp-28] +0xc` | `-` |
| `ldr` at 0xb4d80 | load | 4 | `abs 0xb5198` | `-` |
| `ldr` at 0xb4d84 | load | 4 | `sp -0x3b4` | `-` |
| `ldrb` at 0xb4d8c | load | 1 | `(unknown)` | `-` |
| `str` at 0xb4d90 | store | 4 | `sp -0x3c0` | `?` |
| `ldr` at 0xb4d94 | load | 4 | `abs 0xb519c` | `-` |
| `ldr` at 0xb4d98 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4d9c | load | 1 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0xb4da0 | load | 1 | `[[sp-28]+20]:1 +0x114a78` | `-` |
| `str` at 0xb4da4 | store | 4 | `sp -0x3bc` | `[[[sp-28]+20]:1+1133176]:1` |
| `ldr` at 0xb4dac | load | 4 | `abs 0xb51a4` | `-` |
| `ldr` at 0xb4db0 | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xb4dbc | load | 4 | `sp -0x3b8` | `-` |
| `ldr` at 0xb4dcc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4dd0 | load | 1 | `[sp-28] +0xc` | `-` |
| `ldr` at 0xb4dd4 | load | 4 | `abs 0xb5198` | `-` |
| `ldr` at 0xb4dd8 | load | 4 | `sp -0x3b8` | `-` |
| `ldrb` at 0xb4de0 | load | 1 | `(unknown)` | `-` |
| `str` at 0xb4de4 | store | 4 | `sp -0x3c0` | `?` |
| `ldr` at 0xb4de8 | load | 4 | `abs 0xb519c` | `-` |
| `ldr` at 0xb4dec | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4df0 | load | 1 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0xb4df4 | load | 1 | `[[sp-28]+20]:1 +0x114a78` | `-` |
| `str` at 0xb4df8 | store | 4 | `sp -0x3bc` | `[[[sp-28]+20]:1+1133176]:1` |
| `ldr` at 0xb4e00 | load | 4 | `abs 0xb51a4` | `-` |
| `ldr` at 0xb4e04 | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xb4e10 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb4e14 | load | 2 | `[sp-28] +0x1c` | `-` |
| `ldr` at 0xb4e18 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb4e1c | load | 2 | `[sp-28] +0x1e` | `-` |
| `str` at 0xb4e24 | store | 4 | `sp -0x3a0` | `([[sp-28]+28]:2 + [[sp-28]+30]:2)` |
| `ldr` at 0xb4e2c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb4e30 | load | 2 | `[sp-28] +0x18` | `-` |
| `str` at 0xb4e38 | store | 4 | `sp -0x3c0` | `sp-284` |
| `ldr` at 0xb4e40 | load | 4 | `abs 0xb51a8` | `-` |
| `ldr` at 0xb4e44 | load | 4 | `sp -0x3a0` | `-` |
| `ldr` at 0xb4e68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb4e6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb4e80 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb4e84 | load | 2 | `[sp-28] +0x18` | `-` |
| `ldr` at 0xb4e88 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4e8c | load | 1 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xb4e90 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4e94 | load | 1 | `[sp-28] +0x9` | `-` |
| `str` at 0xb4e98 | store | 4 | `sp -0x3c0` | `[[sp-28]+9]:1` |
| `ldr` at 0xb4ea0 | load | 4 | `abs 0xb51ac` | `-` |
| `ldr` at 0xb4ec8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb4ecc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb4ee4 | load | 4 | `abs 0xb515c` | `-` |
| `ldr` at 0xb4f04 | load | 4 | `abs 0xb51b0` | `-` |
| `ldr` at 0xb4f24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb4f28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb4f40 | load | 4 | `abs 0xb515c` | `-` |
| `ldr` at 0xb4f5c | load | 4 | `abs 0xb5194` | `-` |
| `ldr` at 0xb4f7c | load | 4 | `abs 0xb51b0` | `-` |
| `ldr` at 0xb4f9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb4fa0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb4fb8 | load | 4 | `abs 0xb515c` | `-` |
| `ldr` at 0xb4fd4 | load | 4 | `abs 0xb5194` | `-` |
| `ldr` at 0xb4ff4 | load | 4 | `abs 0xb51b0` | `-` |
| `ldr` at 0xb5014 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5018 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb5030 | load | 4 | `abs 0xb515c` | `-` |
| `ldr` at 0xb504c | load | 4 | `abs 0xb5194` | `-` |
| `ldr` at 0xb506c | load | 4 | `abs 0xb51b0` | `-` |
| `ldr` at 0xb508c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5090 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb50a8 | load | 4 | `abs 0xb515c` | `-` |
| `ldr` at 0xb50c4 | load | 4 | `abs 0xb5194` | `-` |
| `ldr` at 0xb50e4 | load | 4 | `abs 0xb51b0` | `-` |
| `ldr` at 0xb5104 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5108 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb5120 | load | 4 | `abs 0xb51b4` | `-` |
| `ldr` at 0xb5140 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5144 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xb5158 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb5158 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb5158 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb4658 | `(unknown callee) 0x19808` | `sp-284` | `0x11e524` | `r2` | `sp-284` |
| `bl` at 0xb4678 | `(unknown callee) 0x195e0` | `sp-284` | `0x11e528` | `?` | `sp-284` |
| `bl` at 0xb469c | `(unknown callee) 0x19310` | `sp-284` | `0x11e524` | `?` | `sp-284` |
| `bl` at 0xb46b8 | `(unknown callee) 0x195e0` | `sp-284` | `0x11e530` | `?` | `sp-284` |
| `bl` at 0xb46c8 | `(unknown callee) 0x195e0` | `sp-284` | `0x11e534` | `?` | `sp-284` |
| `bl` at 0xb46ec | `(unknown callee) 0x19310` | `sp-284` | `0x11e524` | `?` | `sp-284` |
| `bl` at 0xb4708 | `(unknown callee) 0x195e0` | `sp-284` | `0x11e530` | `?` | `sp-284` |
| `bl` at 0xb4718 | `(unknown callee) 0x195e0` | `sp-284` | `0x11e548` | `?` | `sp-284` |
| `bl` at 0xb473c | `(unknown callee) 0x19310` | `sp-284` | `0x11e524` | `?` | `sp-284` |
| `bl` at 0xb4758 | `(unknown callee) 0x195e0` | `sp-284` | `0x11e530` | `?` | `sp-284` |
| `bl` at 0xb4768 | `(unknown callee) 0x195e0` | `sp-284` | `0x11e554` | `?` | `sp-284` |
| `bl` at 0xb478c | `(unknown callee) 0x19310` | `sp-284` | `0x11e524` | `?` | `sp-284` |
| `bl` at 0xb47a8 | `(unknown callee) 0x195e0` | `sp-284` | `0x11e530` | `?` | `sp-284` |
| `bl` at 0xb47b8 | `(unknown callee) 0x195e0` | `sp-284` | `0x11e568` | `?` | `sp-284` |
| `bl` at 0xb47dc | `(unknown callee) 0x19310` | `sp-284` | `0x11e524` | `?` | `sp-284` |
| `bl` at 0xb47f8 | `(unknown callee) 0x195e0` | `sp-284` | `0x11e530` | `?` | `sp-284` |
| `bl` at 0xb4808 | `(unknown callee) 0x195e0` | `sp-284` | `0x11e570` | `?` | `sp-284` |
| `bl` at 0xb482c | `(unknown callee) 0x19310` | `sp-284` | `0x11e524` | `?` | `sp-284` |
| `bl` at 0xb4848 | `(unknown callee) 0x195e0` | `sp-284` | `0x11e530` | `?` | `sp-284` |
| `bl` at 0xb4858 | `(unknown callee) 0x195e0` | `sp-284` | `0x11e594` | `?` | `sp-284` |
| `bl` at 0xb4868 | `(unknown callee) 0x19808` | `sp-156` | `0x11e524` | `?` | `sp-156` |
| `bl` at 0xb48f4 | `(unknown callee) 0x197a8` | `sp-284` | `0x11e5a0` | `[[sp-28]+26]:2` | `sp-284` |
| `bl` at 0xb4908 | `(unknown callee) 0x197a8` | `sp-156` | `0x11e5d0` | `[sp-932]` | `sp-156` |
| `bl` at 0xb495c | `(unknown callee) 0x197a8` | `sp-284` | `0x11e600` | `[[sp-28]+26]:2` | `sp-284` |
| `bl` at 0xb4970 | `(unknown callee) 0x197a8` | `sp-156` | `0x11e5d0` | `[sp-932]` | `sp-156` |
| `bl` at 0xb4988 | `(unknown callee) 0x197a8` | `sp-156` | `0x11e5d0` | `[sp-932]` | `sp-156` |
| `bl` at 0xb49ac | `(unknown callee) 0x197a8` | `sp-284` | `0x11e628` | `?` | `sp-284` |
| `bl` at 0xb49bc | `(unknown callee) 0x19808` | `sp-412` | `0x11e524` | `?` | `sp-412` |
| `bl` at 0xb49cc | `(unknown callee) 0x19808` | `sp-540` | `0x11e524` | `?` | `sp-540` |
| `bl` at 0xb49dc | `(unknown callee) 0x19808` | `sp-668` | `0x11e524` | `?` | `sp-668` |
| `bl` at 0xb49ec | `(unknown callee) 0x19808` | `sp-796` | `0x11e524` | `?` | `sp-796` |
| `bl` at 0xb4a10 | `(unknown callee) 0x195e0` | `sp-412` | `0x11e660` | `?` | `sp-412` |
| `bl` at 0xb4a20 | `(unknown callee) 0x195e0` | `sp-540` | `0x11e660` | `?` | `sp-540` |
| `bl` at 0xb4a30 | `(unknown callee) 0x195e0` | `sp-668` | `0x11e660` | `?` | `sp-668` |
| `bl` at 0xb4a40 | `(unknown callee) 0x195e0` | `sp-796` | `0x11e660` | `?` | `sp-796` |
| `bl` at 0xb4b28 | `(unknown callee) 0xb3a8c` | `[[sp-28]+33]:1` | `?` | `?` | `[[sp-28]+33]:1` |
| `bl` at 0xb4b60 | `(unknown callee) 0xb3a8c` | `[[sp-28]+34]:1` | `?` | `?` | `[[sp-28]+34]:1` |
| `bl` at 0xb4b88 | `(unknown callee) 0xb3a8c` | `[[sp-28]+35]:1` | `?` | `?` | `[[sp-28]+35]:1` |
| `bl` at 0xb4bb0 | `(unknown callee) 0xb3a8c` | `[[sp-28]+36]:1` | `?` | `?` | `[[sp-28]+36]:1` |
| `bl` at 0xb4c50 | `(unknown callee) 0x197a8` | `sp-412` | `0x11e664` | `?` | `?` |
| `bl` at 0xb4c9c | `(unknown callee) 0x197a8` | `sp-412` | `0x11e694` | `?` | `[[sp-28]+12]:1` |
| `bl` at 0xb4d10 | `(unknown callee) 0x197a8` | `sp-412` | `0x11e694` | `?` | `[[sp-28]+12]:1` |
| `bl` at 0xb4d64 | `(unknown callee) 0x197a8` | `sp-540` | `0x11e694` | `?` | `[[sp-28]+12]:1` |
| `bl` at 0xb4db8 | `(unknown callee) 0x197a8` | `sp-668` | `0x11e694` | `?` | `[[sp-28]+12]:1` |
| `bl` at 0xb4e0c | `(unknown callee) 0x197a8` | `sp-796` | `0x11e694` | `?` | `[[sp-28]+12]:1` |
| `bl` at 0xb4e4c | `(unknown callee) 0x197a8` | `sp-924` | `0x11e6c4` | `([[sp-28]+28]:2 + [[sp-28]+30]:2)` | `[[sp-28]+24]:2` |
| `bl` at 0xb4e58 | `(unknown callee) 0x19538` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xb4e78 | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x19538)` | `sp-924` |
| `bl` at 0xb4eac | `(unknown callee) 0x197a8` | `sp-924` | `0x11e6dc` | `[[sp-28]+24]:2` | `[[sp-28]+8]:1` |
| `bl` at 0xb4eb8 | `(unknown callee) 0x19538` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xb4ed8 | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x19538)` | `sp-924` |
| `bl` at 0xb4ee8 | `(unknown callee) 0x19310` | `sp-156` | `0x11e524` | `?` | `sp-156` |
| `bl` at 0xb4f08 | `(unknown callee) 0x197a8` | `sp-924` | `0x11e70c` | `sp-156` | `sp-924` |
| `bl` at 0xb4f14 | `(unknown callee) 0x19538` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xb4f34 | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x19538)` | `sp-924` |
| `bl` at 0xb4f44 | `(unknown callee) 0x19310` | `sp-412` | `0x11e524` | `?` | `sp-412` |
| `bl` at 0xb4f60 | `(unknown callee) 0x19310` | `sp-412` | `0x11e660` | `?` | `sp-412` |
| `bl` at 0xb4f80 | `(unknown callee) 0x197a8` | `sp-924` | `0x11e70c` | `sp-412` | `sp-924` |
| `bl` at 0xb4f8c | `(unknown callee) 0x19538` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xb4fac | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x19538)` | `sp-924` |
| `bl` at 0xb4fbc | `(unknown callee) 0x19310` | `sp-540` | `0x11e524` | `?` | `sp-540` |
| `bl` at 0xb4fd8 | `(unknown callee) 0x19310` | `sp-540` | `0x11e660` | `?` | `sp-540` |
| `bl` at 0xb4ff8 | `(unknown callee) 0x197a8` | `sp-924` | `0x11e70c` | `sp-540` | `sp-924` |
| `bl` at 0xb5004 | `(unknown callee) 0x19538` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xb5024 | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x19538)` | `sp-924` |
| `bl` at 0xb5034 | `(unknown callee) 0x19310` | `sp-668` | `0x11e524` | `?` | `sp-668` |
| `bl` at 0xb5050 | `(unknown callee) 0x19310` | `sp-668` | `0x11e660` | `?` | `sp-668` |
| `bl` at 0xb5070 | `(unknown callee) 0x197a8` | `sp-924` | `0x11e70c` | `sp-668` | `sp-924` |
| `bl` at 0xb507c | `(unknown callee) 0x19538` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xb509c | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x19538)` | `sp-924` |
| `bl` at 0xb50ac | `(unknown callee) 0x19310` | `sp-796` | `0x11e524` | `?` | `sp-796` |
| `bl` at 0xb50c8 | `(unknown callee) 0x19310` | `sp-796` | `0x11e660` | `?` | `sp-796` |
| `bl` at 0xb50e8 | `(unknown callee) 0x197a8` | `sp-924` | `0x11e70c` | `sp-796` | `sp-924` |
| `bl` at 0xb50f4 | `(unknown callee) 0x19538` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xb5114 | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x19538)` | `sp-924` |
| `bl` at 0xb5124 | `(unknown callee) 0x197a8` | `sp-924` | `0x11e714` | `?` | `sp-924` |
| `bl` at 0xb5130 | `(unknown callee) 0x19538` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xb5150 | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x19538)` | `sp-924` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb4668 | `[[sp-28]+13]:1 != 0x0` | `[[sp-28]+13]:1 == 0x0` |
| `beq` at 0xb468c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xb46a8 | `ret(0x19310) == 0x0` | `ret(0x19310) != 0x0` |
| `beq` at 0xb46dc | `? == 0x0` | `? != 0x0` |
| `beq` at 0xb46f8 | `ret(0x19310) == 0x0` | `ret(0x19310) != 0x0` |
| `beq` at 0xb472c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xb4748 | `ret(0x19310) == 0x0` | `ret(0x19310) != 0x0` |
| `beq` at 0xb477c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xb4798 | `ret(0x19310) == 0x0` | `ret(0x19310) != 0x0` |
| `beq` at 0xb47cc | `? == 0x0` | `? != 0x0` |
| `beq` at 0xb47e8 | `ret(0x19310) == 0x0` | `ret(0x19310) != 0x0` |
| `beq` at 0xb481c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xb4838 | `ret(0x19310) == 0x0` | `ret(0x19310) != 0x0` |
| `beq` at 0xb487c | `[[sp-28]+26]:2 == 0xffff` | `[[sp-28]+26]:2 != 0xffff` |
| `beq` at 0xb4890 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xb48b4 | `[[sp-28]+33]:1 != 0x77` | `[[sp-28]+33]:1 == 0x77` |
| `bne` at 0xb48c4 | `[[sp-28]+34]:1 != 0x77` | `[[sp-28]+34]:1 == 0x77` |
| `bne` at 0xb48d4 | `[[sp-28]+35]:1 != 0x77` | `[[sp-28]+35]:1 == 0x77` |
| `bne` at 0xb48e4 | `[[sp-28]+36]:1 != 0x77` | `[[sp-28]+36]:1 == 0x77` |
| `bne` at 0xb491c | `[[sp-28]+33]:1 != 0x78` | `[[sp-28]+33]:1 == 0x78` |
| `bne` at 0xb492c | `[[sp-28]+34]:1 != 0x78` | `[[sp-28]+34]:1 == 0x78` |
| `bne` at 0xb493c | `[[sp-28]+35]:1 != 0x78` | `[[sp-28]+35]:1 == 0x78` |
| `bne` at 0xb494c | `[[sp-28]+36]:1 != 0x78` | `[[sp-28]+36]:1 == 0x78` |
| `beq` at 0xb499c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xb4a00 | `? == 0x0` | `? != 0x0` |
| `bgt` at 0xb4a58 | `[[sp-28]+32]:1 > 0x6 (signed, 32-bit)` | `[[sp-28]+32]:1 <= 0x6 (signed, 32-bit)` |
| `bgt` at 0xb4a68 | `[[sp-28]+20]:1 > 0x2 (signed, 32-bit)` | `[[sp-28]+20]:1 <= 0x2 (signed, 32-bit)` |
| `bgt` at 0xb4ab4 | `[[sp-28]+21]:1 > 0x2 (signed, 32-bit)` | `[[sp-28]+21]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0xb4ac4 | `[[sp-28]+37]:1 == 0x6` | `[[sp-28]+37]:1 != 0x6` |
| `beq` at 0xb4ad4 | `[[sp-28]+37]:1 == 0x8` | `[[sp-28]+37]:1 != 0x8` |
| `beq` at 0xb4af4 | `[[sp-28]+37]:1 == 0xa` | `[[sp-28]+37]:1 != 0xa` |
| `beq` at 0xb4b04 | `[[sp-28]+37]:1 == 0x16` | `[[sp-28]+37]:1 != 0x16` |
| `bgt` at 0xb4b18 | `[[sp-28]+33]:1 > 0x7e (signed, 32-bit)` | `[[sp-28]+33]:1 <= 0x7e (signed, 32-bit)` |
| `bne` at 0xb4b40 | `[[sp-28]+37]:1 != 0x16` | `[[sp-28]+37]:1 == 0x16` |
| `bgt` at 0xb4b50 | `[[sp-28]+34]:1 > 0x7e (signed, 32-bit)` | `[[sp-28]+34]:1 <= 0x7e (signed, 32-bit)` |
| `bgt` at 0xb4b78 | `[[sp-28]+35]:1 > 0x7e (signed, 32-bit)` | `[[sp-28]+35]:1 <= 0x7e (signed, 32-bit)` |
| `bgt` at 0xb4ba0 | `[[sp-28]+36]:1 > 0x7e (signed, 32-bit)` | `[[sp-28]+36]:1 <= 0x7e (signed, 32-bit)` |
| `bhi` at 0xb4bc4 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0xb4bd4 | `[[sp-28]+37]:1 == 0x16` | `[[sp-28]+37]:1 != 0x16` |
| `bne` at 0xb4be4 | `[[sp-28]+33]:1 != 0x78` | `[[sp-28]+33]:1 == 0x78` |
| `bne` at 0xb4bf4 | `[[sp-28]+34]:1 != 0x78` | `[[sp-28]+34]:1 == 0x78` |
| `bne` at 0xb4c04 | `[[sp-28]+35]:1 != 0x78` | `[[sp-28]+35]:1 == 0x78` |
| `bne` at 0xb4c14 | `[[sp-28]+36]:1 != 0x78` | `[[sp-28]+36]:1 == 0x78` |
| `bhi` at 0xb4cac | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0xb4cbc | `[[sp-28]+37]:1 != 0x16` | `[[sp-28]+37]:1 == 0x16` |
| `bhi` at 0xb4cc8 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bhi` at 0xb4d1c | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bhi` at 0xb4d70 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bhi` at 0xb4dc4 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0xb4ef4 | `ret(0x19310) == 0x0` | `ret(0x19310) != 0x0` |
| `beq` at 0xb4f50 | `ret(0x19310) == 0x0` | `ret(0x19310) != 0x0` |
| `beq` at 0xb4f6c | `ret(0x19310) == 0x0` | `ret(0x19310) != 0x0` |
| `beq` at 0xb4fc8 | `ret(0x19310) == 0x0` | `ret(0x19310) != 0x0` |
| `beq` at 0xb4fe4 | `ret(0x19310) == 0x0` | `ret(0x19310) != 0x0` |
| `beq` at 0xb5040 | `ret(0x19310) == 0x0` | `ret(0x19310) != 0x0` |
| `beq` at 0xb505c | `ret(0x19310) == 0x0` | `ret(0x19310) != 0x0` |
| `beq` at 0xb50b8 | `ret(0x19310) == 0x0` | `ret(0x19310) != 0x0` |
| `beq` at 0xb50d4 | `ret(0x19310) == 0x0` | `ret(0x19310) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb5158 | `r0` | `ret(unknown)` |

## `FRUPackData`

- `FRUPackData`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb51ec to 0xb5f0c.
  Established. Included byte stores write `0x6` to `[[sp+28]] +0x0` and `0x8` to `[[sp+28]] +0x1` (`strb` at 0xb5534, `strb` at 0xb5550).
  An included unnamed `0x19610` call carries `[sp-40]+4`, `[sp+16]`, `[sp-52]`, `[sp-40]+4` (`bl` at 0xb5d78).
  Packed FRU data length validation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb51fc | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xb5200 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xb5204 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xb5208 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0xb520c | load | 4 | `sp +0x20` | `-` |
| `str` at 0xb5214 | store | 4 | `[sp+32] +0x0` | `0x1` |
| `ldr` at 0xb5218 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb521c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb5220 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xb5228 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+1` |
| `ldr` at 0xb522c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb5230 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb5234 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xb523c | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+1` |
| `ldr` at 0xb5240 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb5244 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb5248 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xb5250 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+1` |
| `ldr` at 0xb5254 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb5258 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb525c | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xb5264 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xb5268 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb526c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb5270 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xb5278 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xb527c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb5280 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb5284 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xb528c | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xb5290 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb5294 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb5298 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xb52a0 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xb52a4 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb52a8 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb52ac | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xb52b4 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xb52b8 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb52bc | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb52c0 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xb52c8 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xb52cc | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb52d0 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb52d4 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xb52dc | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `str` at 0xb52e4 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xb52e8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb52ec | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xb52f8 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb52fc | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb5300 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xb5308 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xb530c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb5310 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb5314 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xb531c | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xb5320 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb5324 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb5328 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xb5330 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xb5334 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb5338 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb533c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb5350 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xb5358 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb5360 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xb5368 | store | 4 | `[sp+32] +0x0` | `?` |
| `ldr` at 0xb536c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb5370 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb5374 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb5388 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xb5390 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb5398 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xb53a0 | store | 4 | `[sp+32] +0x0` | `?` |
| `ldr` at 0xb53a4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb53ac | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xb53b4 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb53b8 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb53bc | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xb53c4 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xb53c8 | load | 4 | `sp +0x20` | `-` |
| `str` at 0xb53cc | store | 4 | `sp -0x38` | `[sp+32]` |
| `ldr` at 0xb53d0 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb53d4 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xb53d8 | store | 4 | `sp -0x3c` | `[[sp+32]]` |
| `ldr` at 0xb53dc | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb53e8 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb53f4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb53fc | store | 4 | `sp -0x40` | `([sp-60] + ret(0x19538))` |
| `ldr` at 0xb5400 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xb5408 | store | 4 | `sp -0x40` | `([sp-60] + ret(0x19538))+1` |
| `ldr` at 0xb5410 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb5414 | store | 4 | `sp -0x40` | `[[sp+32]]` |
| `ldr` at 0xb5418 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb541c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xb5420 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xb5424 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb5428 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb542c | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xb5434 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xb5438 | load | 4 | `sp +0x20` | `-` |
| `str` at 0xb543c | store | 4 | `sp -0x44` | `[sp+32]` |
| `ldr` at 0xb5440 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb5444 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xb5448 | store | 4 | `sp -0x48` | `[[sp+32]]` |
| `ldr` at 0xb544c | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xb5458 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xb5464 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xb546c | store | 4 | `sp -0x4c` | `([sp-72] + ret(0x19538))` |
| `ldr` at 0xb5470 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xb5478 | store | 4 | `sp -0x4c` | `([sp-72] + ret(0x19538))+1` |
| `ldr` at 0xb5480 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xb5484 | store | 4 | `sp -0x4c` | `[[sp+32]]` |
| `ldr` at 0xb5488 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xb548c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xb5490 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xb5494 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb5498 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb549c | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xb54a4 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xb54a8 | load | 4 | `sp +0x20` | `-` |
| `str` at 0xb54ac | store | 4 | `sp -0x50` | `[sp+32]` |
| `ldr` at 0xb54b0 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb54b4 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xb54b8 | store | 4 | `sp -0x54` | `[[sp+32]]` |
| `ldr` at 0xb54bc | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xb54c8 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xb54d4 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xb54dc | store | 4 | `sp -0x58` | `([sp-84] + ret(0x19538))` |
| `ldr` at 0xb54e0 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xb54e8 | store | 4 | `sp -0x58` | `([sp-84] + ret(0x19538))+1` |
| `ldr` at 0xb54f0 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xb54f4 | store | 4 | `sp -0x58` | `[[sp+32]]` |
| `ldr` at 0xb54f8 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xb54fc | load | 4 | `sp -0x50` | `-` |
| `str` at 0xb5500 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xb5504 | load | 4 | `sp +0x1c` | `-` |
| `ldr` at 0xb5508 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb550c | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xb5518 | store | 4 | `[sp+28] +0x0` | `ret(0x1943c)` |
| `ldr` at 0xb551c | load | 4 | `[sp+28] +0x0` | `-` |
| `str` at 0xb5520 | store | 4 | `sp -0x28` | `[[sp+28]]` |
| `ldr` at 0xb5528 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0xb5534 | store | 1 | `[[sp+28]] +0x0` | `0x6` |
| `str` at 0xb553c | store | 4 | `sp -0x28` | `[[sp+28]]+1` |
| `ldr` at 0xb5544 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0xb5550 | store | 1 | `[[sp+28]] +0x1` | `0x8` |
| `str` at 0xb5558 | store | 4 | `sp -0x28` | `[[sp+28]]+2` |
| `ldr` at 0xb5560 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0xb556c | store | 1 | `[[sp+28]] +0x2` | `0x0` |
| `str` at 0xb5574 | store | 4 | `sp -0x28` | `[[sp+28]]+3` |
| `ldr` at 0xb557c | load | 4 | `sp -0x28` | `-` |
| `strb` at 0xb5588 | store | 1 | `[[sp+28]] +0x3` | `0x0` |
| `str` at 0xb5590 | store | 4 | `sp -0x28` | `[[sp+28]]+4` |
| `ldr` at 0xb5594 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5598 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xb55a0 | store | 1 | `[[sp+28]] +0x4` | `?` |
| `ldr` at 0xb55a8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb55b0 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xb55b8 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xb55c0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb55c8 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xb55d0 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xb55d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb55e0 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xb55e4 | store | 1 | `[sp-40] +0x3` | `([sp] & 0xff)` |
| `ldr` at 0xb55e8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb55f0 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xb55f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb55f8 | load | 4 | `sp +0x4` | `-` |
| `strb` at 0xb5600 | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xb5608 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5610 | load | 4 | `sp +0x4` | `-` |
| `strb` at 0xb5618 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xb5620 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5628 | load | 4 | `sp +0x4` | `-` |
| `strb` at 0xb5630 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xb5638 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5640 | load | 4 | `sp +0x4` | `-` |
| `strb` at 0xb5644 | store | 1 | `[sp-40] +0x3` | `([sp+4] & 0xff)` |
| `ldr` at 0xb5648 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb5650 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xb5654 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5658 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xb5660 | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xb5668 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5670 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xb5678 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xb5680 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5688 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xb5690 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xb5698 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb56a0 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xb56a4 | store | 1 | `[sp-40] +0x3` | `([sp-24] & 0xff)` |
| `ldr` at 0xb56a8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb56b0 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xb56b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb56b8 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xb56c0 | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xb56c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb56d0 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xb56d8 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xb56e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb56e8 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xb56f0 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xb56f8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5700 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xb5704 | store | 1 | `[sp-40] +0x3` | `([sp-28] & 0xff)` |
| `ldr` at 0xb5708 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb5710 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xb5714 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5718 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xb5720 | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xb5728 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5730 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xb5738 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xb5740 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5748 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xb5750 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xb5758 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5760 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xb5764 | store | 1 | `[sp-40] +0x3` | `([sp-32] & 0xff)` |
| `ldr` at 0xb5768 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb5770 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xb5774 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5778 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xb5780 | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xb5788 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5790 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xb5798 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xb57a0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb57a8 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xb57b0 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xb57b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb57c0 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xb57c4 | store | 1 | `[sp-40] +0x3` | `([sp-36] & 0xff)` |
| `ldr` at 0xb57c8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb57d0 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xb57d4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb57d8 | load | 4 | `sp +0x8` | `-` |
| `strb` at 0xb57e0 | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xb57e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb57f0 | load | 4 | `sp +0x8` | `-` |
| `strb` at 0xb57f8 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xb5800 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5808 | load | 4 | `sp +0x8` | `-` |
| `strb` at 0xb5810 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xb5818 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5820 | load | 4 | `sp +0x8` | `-` |
| `strb` at 0xb5824 | store | 1 | `[sp-40] +0x3` | `([sp+8] & 0xff)` |
| `ldr` at 0xb5828 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb5830 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `str` at 0xb5838 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xb583c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb5840 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xb584c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5850 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb5864 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xb586c | load | 4 | `(unknown)` | `-` |
| `strb` at 0xb5874 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xb587c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5884 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb5898 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xb58a0 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xb58a8 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xb58b0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb58b8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb58cc | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xb58d4 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xb58dc | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xb58e4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb58ec | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb5900 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xb5908 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xb590c | store | 1 | `[sp-40] +0x3` | `?` |
| `ldr` at 0xb5910 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb5918 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xb591c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5920 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb5934 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xb593c | load | 4 | `(unknown)` | `-` |
| `strb` at 0xb5944 | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xb594c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5954 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb5968 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xb5970 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xb5978 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xb5980 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5988 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb599c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xb59a4 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xb59ac | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xb59b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb59bc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb59d0 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xb59d8 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xb59dc | store | 1 | `[sp-40] +0x3` | `?` |
| `ldr` at 0xb59e0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb59e8 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xb59ec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb59f0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb5a04 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xb5a0c | load | 4 | `(unknown)` | `-` |
| `strb` at 0xb5a14 | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xb5a1c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5a24 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb5a38 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xb5a40 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xb5a48 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xb5a50 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5a58 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb5a6c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xb5a74 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xb5a7c | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xb5a84 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5a8c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb5aa0 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xb5aa8 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xb5aac | store | 1 | `[sp-40] +0x3` | `?` |
| `ldr` at 0xb5ab0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb5ab8 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `str` at 0xb5ac0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xb5ac4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb5ad8 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xb5ae0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb5ae4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb5af0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5af4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb5b08 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xb5b10 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb5b24 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb5b2c | load | 4 | `(unknown)` | `-` |
| `strb` at 0xb5b34 | store | 1 | `[sp-40] +0x0` | `?` |
| `ldr` at 0xb5b3c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5b44 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb5b58 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xb5b60 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb5b74 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb5b7c | load | 4 | `(unknown)` | `-` |
| `strb` at 0xb5b84 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xb5b8c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5b94 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb5ba8 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xb5bb0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb5bc4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb5bcc | load | 4 | `(unknown)` | `-` |
| `strb` at 0xb5bd4 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xb5bdc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5be4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb5bf8 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xb5c00 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb5c14 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb5c1c | load | 4 | `(unknown)` | `-` |
| `strb` at 0xb5c20 | store | 1 | `[sp-40] +0x3` | `?` |
| `ldr` at 0xb5c24 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb5c2c | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xb5c30 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb5c38 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xb5c40 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb5c54 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xb5c5c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb5c70 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xb5c78 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb5c80 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5c84 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb5c90 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb5ca4 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xb5cac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb5cb4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb5cbc | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb5cc0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb5cc8 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xb5cd0 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb5cdc | load | 4 | `sp +0x10` | `-` |
| `str` at 0xb5ce8 | store | 4 | `sp -0x5c` | `ret(0x19538)` |
| `ldr` at 0xb5cec | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xb5cf4 | store | 4 | `sp -0x5c` | `ret(0x19538)+1` |
| `str` at 0xb5d00 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xb5d04 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xb5d08 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xb5d0c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5d10 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xb5d18 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xb5d20 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5d28 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xb5d30 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xb5d38 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5d40 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xb5d48 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xb5d50 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5d58 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xb5d5c | store | 1 | `[sp-40] +0x3` | `([sp-52] & 0xff)` |
| `ldr` at 0xb5d60 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb5d68 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xb5d6c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5d70 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb5d74 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb5d7c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5d80 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xb5d88 | store | 4 | `sp -0x28` | `([sp-40] + [sp-52])` |
| `ldr` at 0xb5d8c | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xb5d98 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xb5da4 | store | 4 | `sp -0x60` | `ret(0x19538)` |
| `ldr` at 0xb5da8 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xb5db0 | store | 4 | `sp -0x60` | `ret(0x19538)+1` |
| `str` at 0xb5dbc | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xb5dc0 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xb5dc4 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xb5dc8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5dcc | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xb5dd4 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xb5ddc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5de4 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xb5dec | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xb5df4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5dfc | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xb5e04 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xb5e0c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5e14 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xb5e18 | store | 1 | `[sp-40] +0x3` | `([sp-52] & 0xff)` |
| `ldr` at 0xb5e1c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb5e24 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xb5e28 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5e2c | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xb5e30 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb5e38 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5e3c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xb5e44 | store | 4 | `sp -0x28` | `([sp-40] + [sp-52])` |
| `ldr` at 0xb5e48 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xb5e54 | load | 4 | `sp +0x18` | `-` |
| `str` at 0xb5e60 | store | 4 | `sp -0x64` | `ret(0x19538)` |
| `ldr` at 0xb5e64 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xb5e6c | store | 4 | `sp -0x64` | `ret(0x19538)+1` |
| `str` at 0xb5e78 | store | 4 | `sp -0x64` | `0x0` |
| `ldr` at 0xb5e7c | load | 4 | `sp -0x64` | `-` |
| `str` at 0xb5e80 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xb5e84 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5e88 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xb5e90 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xb5e98 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5ea0 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xb5ea8 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xb5eb0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5eb8 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xb5ec0 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xb5ec8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5ed0 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xb5ed4 | store | 1 | `[sp-40] +0x3` | `([sp-52] & 0xff)` |
| `ldr` at 0xb5ed8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb5ee0 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xb5ee4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5ee8 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xb5eec | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb5ef4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5ef8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xb5f00 | store | 4 | `sp -0x28` | `([sp-40] + [sp-52])` |
| `ldm` at 0xb5f08 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb5f08 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb5f08 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb5f08 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb53ec | `(unknown callee) 0x19538` | `[sp+16]` | `?` | `[sp+32]` | `[sp+16]` |
| `bl` at 0xb545c | `(unknown callee) 0x19538` | `[sp+20]` | `?` | `[sp+32]` | `[sp+20]` |
| `bl` at 0xb54cc | `(unknown callee) 0x19538` | `[sp+24]` | `?` | `[sp+32]` | `[sp+24]` |
| `bl` at 0xb5510 | `(unknown callee) 0x1943c` | `[[sp+32]]` | `?` | `?` | `[sp+32]` |
| `bl` at 0xb5c8c | `(unknown callee) 0x19610` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xb5ce0 | `(unknown callee) 0x19538` | `[sp+16]` | `?` | `?` | `[sp+16]` |
| `bl` at 0xb5d78 | `(unknown callee) 0x19610` | `[sp-40]+4` | `[sp+16]` | `[sp-52]` | `[sp-40]+4` |
| `bl` at 0xb5d9c | `(unknown callee) 0x19538` | `[sp+20]` | `?` | `[sp-40]` | `[sp+20]` |
| `bl` at 0xb5e34 | `(unknown callee) 0x19610` | `[sp-40]+4` | `[sp+20]` | `[sp-52]` | `[sp-40]+4` |
| `bl` at 0xb5e58 | `(unknown callee) 0x19538` | `[sp+24]` | `?` | `[sp-40]` | `[sp+24]` |
| `bl` at 0xb5ef0 | `(unknown callee) 0x19610` | `[sp-40]+4` | `[sp+24]` | `[sp-52]` | `[sp-40]+4` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xb52f4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xb53e4 | `[sp+16] == 0x0` | `[sp+16] != 0x0` |
| `beq` at 0xb5454 | `[sp+20] == 0x0` | `[sp+20] != 0x0` |
| `beq` at 0xb54c4 | `[sp+24] == 0x0` | `[sp+24] != 0x0` |
| `bge` at 0xb5848 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `ble` at 0xb5aec | `? <= [sp-44] (signed, 32-bit)` | `? > [sp-44] (signed, 32-bit)` |
| `beq` at 0xb5cd8 | `[sp+16] == 0x0` | `[sp+16] != 0x0` |
| `beq` at 0xb5d94 | `[sp+20] == 0x0` | `[sp+20] != 0x0` |
| `beq` at 0xb5e50 | `[sp+24] == 0x0` | `[sp+24] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb5f08 | `r0` | `ret(0x19610)` |

## `FRUUnpackData`

- `FRUUnpackData`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb63dc to 0xb6e88.
  Established. Equality `[[sp-28]] == 0x6` takes `0xb646c`, while inequality falls through (`beq` at 0xb6458).
  Included word stores write `0x5` and `0x6` to `[sp+12] +0x0` (`str` at 0xb6464, `str` at 0xb6790).
  FRU unpacking status codes remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb63ec | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xb63f0 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xb63f4 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xb63f8 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0xb63fc | load | 4 | `sp +0x10` | `-` |
| `str` at 0xb6404 | store | 4 | `[sp+16] +0x0` | `0x0` |
| `ldr` at 0xb6408 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xb6410 | store | 4 | `[sp+20] +0x0` | `0x0` |
| `ldr` at 0xb6414 | load | 4 | `sp +0x18` | `-` |
| `str` at 0xb641c | store | 4 | `[sp+24] +0x0` | `0x0` |
| `ldr` at 0xb6420 | load | 4 | `sp +0x1c` | `-` |
| `str` at 0xb6428 | store | 4 | `[sp+28] +0x0` | `0x0` |
| `ldr` at 0xb642c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb6434 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb643c | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0xb6440 | store | 4 | `[sp-28] +0x0` | `[[sp-24]]:1` |
| `str` at 0xb6448 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0xb644c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb6450 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xb645c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xb6464 | store | 4 | `[sp+12] +0x0` | `0x5` |
| `ldr` at 0xb6470 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6478 | load | 1 | `[sp-24] +0x1` | `-` |
| `str` at 0xb647c | store | 4 | `sp -0x30` | `[[sp-24]+1]:1` |
| `str` at 0xb6484 | store | 4 | `sp -0x18` | `[sp-24]+2` |
| `ldr` at 0xb648c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6494 | load | 1 | `[sp-24] +0x2` | `-` |
| `str` at 0xb6498 | store | 4 | `sp -0x34` | `[[sp-24]+2]:1` |
| `str` at 0xb64a0 | store | 4 | `sp -0x18` | `[sp-24]+3` |
| `ldr` at 0xb64a8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb64b0 | load | 1 | `[sp-24] +0x3` | `-` |
| `str` at 0xb64b4 | store | 4 | `sp -0x38` | `[[sp-24]+3]:1` |
| `str` at 0xb64bc | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xb64c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb64c4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb64c8 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xb64d4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb64dc | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xb64ec | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb64f4 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xb6504 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb650c | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xb6514 | store | 4 | `[sp-32] +0x0` | `?` |
| `ldr` at 0xb6518 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb6520 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xb6524 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb6528 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb652c | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xb6538 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6540 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xb6550 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6558 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xb6568 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6570 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xb6578 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0xb657c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb6584 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xb6588 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xb658c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6590 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xb659c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb65a4 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xb65b4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb65bc | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xb65cc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb65d4 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xb65dc | store | 4 | `[sp] +0x0` | `?` |
| `ldr` at 0xb65e0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb65e8 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xb65ec | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xb65f0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb65f4 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xb6600 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6608 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xb6618 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6620 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xb6630 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6638 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xb6640 | store | 4 | `[sp+4] +0x0` | `?` |
| `ldr` at 0xb6644 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb664c | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xb6650 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xb6654 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6658 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xb6664 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb666c | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xb667c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6684 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xb6694 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb669c | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xb66a4 | store | 4 | `[sp+8] +0x0` | `?` |
| `ldr` at 0xb66a8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb66b0 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xb66b4 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xb66b8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb66bc | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xb66c8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb66d0 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xb66e0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb66e8 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xb66f8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6700 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xb6708 | store | 4 | `[sp+12] +0x0` | `?` |
| `ldr` at 0xb670c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb6714 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xb6718 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb671c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6720 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xb672c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6734 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xb6744 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb674c | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xb675c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6764 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xb676c | store | 4 | `[sp+16] +0x0` | `?` |
| `ldr` at 0xb6770 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb6778 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xb677c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb6788 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xb6790 | store | 4 | `[sp+12] +0x0` | `0x6` |
| `ldr` at 0xb6798 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xb679c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb67a0 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xb67c0 | store | 4 | `[sp+20] +0x0` | `ret(0x1943c)` |
| `ldr` at 0xb67c4 | load | 4 | `[sp+20] +0x0` | `-` |
| `str` at 0xb67c8 | store | 4 | `sp -0x28` | `[[sp+20]]` |
| `ldr` at 0xb67cc | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xb67d0 | load | 4 | `[sp+20] +0x0` | `-` |
| `ldr` at 0xb67dc | load | 4 | `sp +0xc` | `-` |
| `str` at 0xb67e4 | store | 4 | `[sp+12] +0x0` | `0x8` |
| `str` at 0xb67f0 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xb67f4 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb67f8 | load | 4 | `[sp+16] +0x0` | `-` |
| `ldr` at 0xb67fc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xb6808 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xb681c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb6824 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6828 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xb6834 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb683c | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xb684c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6854 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xb6864 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb686c | load | 1 | `[sp-24] +0x3` | `-` |
| `str` at 0xb6874 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xb6878 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb6880 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xb6884 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xb6898 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb68a0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb68a4 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xb68b0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb68b8 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xb68c8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb68d0 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xb68e0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb68e8 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xb68f0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xb68f4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb68fc | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xb6900 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xb6914 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb691c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6920 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xb692c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6934 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xb6944 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb694c | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xb695c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6964 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xb696c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xb6970 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb6978 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xb697c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xb6990 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb6998 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xb69ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb69b4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb69c8 | store | 4 | `(unknown)` | `ret(0x1943c)` |
| `ldr` at 0xb69cc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xb69e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb69e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb69f4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xb69fc | store | 4 | `[sp+12] +0x0` | `0x8` |
| `ldr` at 0xb6a04 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xb6a18 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb6a20 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xb6a34 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb6a3c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb6a44 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb6a58 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xb6a5c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xb6a70 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb6a78 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb6a7c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb6a88 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6a8c | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xb6a98 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6aa0 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xb6ab0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6ab8 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xb6ac8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6ad0 | load | 1 | `[sp-24] +0x3` | `-` |
| `str` at 0xb6ad8 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xb6adc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb6ae4 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xb6ae8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xb6afc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb6b04 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb6b0c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb6b14 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xb6b18 | store | 4 | `(unknown)` | `[sp-68]` |
| `ldr` at 0xb6b1c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb6b24 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xb6b2c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xb6b40 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb6b48 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xb6b5c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb6b64 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb6b78 | store | 4 | `(unknown)` | `ret(0x1943c)` |
| `ldr` at 0xb6b7c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xb6b90 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb6b98 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb6ba4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xb6bac | store | 4 | `[sp+12] +0x0` | `0x8` |
| `ldr` at 0xb6bb4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xb6bc8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb6bd0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xb6be4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb6bec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb6bf4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb6c04 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xb6c18 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb6c20 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xb6c34 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb6c3c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb6c44 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb6c48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb6c54 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xb6c68 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb6c70 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb6c78 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb6c80 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xb6c84 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb6c8c | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0xb6c94 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6c98 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xb6ca4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6cac | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xb6cbc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6cc4 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xb6cd4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6cdc | load | 1 | `[sp-24] +0x3` | `-` |
| `str` at 0xb6ce4 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xb6ce8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb6cf0 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xb6cf4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb6d00 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xb6d04 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xb6d10 | store | 4 | `[sp+24] +0x0` | `ret(0x1943c)` |
| `ldr` at 0xb6d14 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xb6d18 | load | 4 | `[sp+24] +0x0` | `-` |
| `ldr` at 0xb6d1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb6d20 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb6d28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb6d2c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xb6d34 | store | 4 | `sp -0x18` | `([sp-24] + [sp-64])` |
| `ldr` at 0xb6d38 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6d3c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb6d48 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6d50 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb6d60 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6d68 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb6d78 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6d80 | load | 1 | `(unknown)` | `-` |
| `str` at 0xb6d88 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xb6d8c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb6d94 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xb6d98 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb6da4 | load | 4 | `sp +0x1c` | `-` |
| `ldr` at 0xb6da8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xb6db4 | store | 4 | `[sp+28] +0x0` | `ret(0x1943c)` |
| `ldr` at 0xb6db8 | load | 4 | `sp +0x1c` | `-` |
| `ldr` at 0xb6dbc | load | 4 | `[sp+28] +0x0` | `-` |
| `ldr` at 0xb6dc0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb6dc4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb6dcc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb6dd0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xb6dd8 | store | 4 | `sp -0x18` | `([sp-24] + [sp-64])` |
| `ldr` at 0xb6ddc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6de0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb6dec | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6df4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb6e04 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6e0c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb6e1c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6e24 | load | 1 | `(unknown)` | `-` |
| `str` at 0xb6e2c | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xb6e30 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb6e38 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xb6e3c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb6e48 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb6e4c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xb6e58 | store | 4 | `[sp+32] +0x0` | `ret(0x1943c)` |
| `ldr` at 0xb6e5c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb6e60 | load | 4 | `[sp+32] +0x0` | `-` |
| `ldr` at 0xb6e64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb6e68 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb6e70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb6e74 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xb6e7c | store | 4 | `sp -0x18` | `([sp-24] + [sp-64])` |
| `ldm` at 0xb6e84 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb6e84 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb6e84 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb6e84 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb67b8 | `(unknown callee) 0x1943c` | `?` | `?` | `[[sp+16]]` | `?` |
| `bl` at 0xb69c0 | `(unknown callee) 0x1943c` | `?` | `?` | `?` | `?` |
| `bl` at 0xb6a50 | `(unknown callee) 0x19748` | `?` | `0x0` | `?` | `?` |
| `bl` at 0xb6b70 | `(unknown callee) 0x1943c` | `?` | `?` | `?` | `?` |
| `bl` at 0xb6c00 | `(unknown callee) 0x19748` | `?` | `0x0` | `?` | `?` |
| `bl` at 0xb6c50 | `(unknown callee) 0x19610` | `?` | `[sp-24]` | `?` | `?` |
| `bl` at 0xb6d08 | `(unknown callee) 0x1943c` | `[sp-64]` | `?` | `0x3` | `[sp-64]` |
| `bl` at 0xb6d24 | `(unknown callee) 0x19610` | `[[sp+24]]` | `[sp-24]` | `[sp-64]` | `[sp+24]` |
| `bl` at 0xb6dac | `(unknown callee) 0x1943c` | `[sp-64]` | `?` | `0x3` | `[sp-64]` |
| `bl` at 0xb6dc8 | `(unknown callee) 0x19610` | `[[sp+28]]` | `[sp-24]` | `[sp-64]` | `[sp+28]` |
| `bl` at 0xb6e50 | `(unknown callee) 0x1943c` | `[sp-64]` | `?` | `0x3` | `[sp-64]` |
| `bl` at 0xb6e6c | `(unknown callee) 0x19610` | `[[sp+32]]` | `[sp-24]` | `[sp-64]` | `[sp+32]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb6458 | `[[sp-28]] == 0x6` | `[[sp-28]] != 0x6` |
| `beq` at 0xb6784 | `[sp-48] == 0x8` | `[sp-48] != 0x8` |
| `bne` at 0xb67d8 | `[[sp+20]] != 0x0` | `[[sp+20]] == 0x0` |
| `ble` at 0xb6804 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xb69f0 | `? != 0x0` | `? == 0x0` |
| `ble` at 0xb6a84 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xb6ba0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xb6cfc | `[sp-64] == 0x0` | `[sp-64] != 0x0` |
| `beq` at 0xb6da0 | `[sp-64] == 0x0` | `[sp-64] != 0x0` |
| `beq` at 0xb6e44 | `[sp-64] == 0x0` | `[sp-64] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb6e84 | `r0` | `?` |

## `FRUPackEvents`

- `FRUPackEvents`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb5f0c to 0xb63dc.
  Established. Included byte stores write `0x6` to `[[sp+12]] +0x0` and `([sp-52] & 0xff)` to `[[sp+12]] +0x1` (`strb` at 0xb6090, `strb` at 0xb60b0).
  The unnamed `0x19610` call carries two unknown candidates, `0xa8`, unknown (`bl` at 0xb63b4).
  Packed event-record format remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb5f1c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xb5f20 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xb5f24 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xb5f28 | store | 4 | `sp -0x24` | `r3` |
| `str` at 0xb5f30 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xb5f34 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xb5f38 | store | 4 | `sp -0x28` | `[sp+8]` |
| `ldr` at 0xb5f3c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5f48 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb5f50 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xb5f54 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb5f58 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb5f5c | store | 4 | `sp -0x28` | `?` |
| `str` at 0xb5f68 | store | 4 | `sp -0x34` | `0xa8` |
| `ldr` at 0xb5f6c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xb5f74 | store | 4 | `[sp+16] +0x0` | `0x1` |
| `ldr` at 0xb5f78 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb5f7c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb5f80 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xb5f88 | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+1` |
| `ldr` at 0xb5f8c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb5f90 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb5f94 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xb5f9c | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+1` |
| `ldr` at 0xb5fa0 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb5fa4 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb5fa8 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xb5fb0 | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+1` |
| `ldr` at 0xb5fb4 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb5fb8 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb5fbc | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xb5fc4 | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+4` |
| `ldr` at 0xb5fc8 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb5fcc | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb5fd0 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xb5fd8 | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+4` |
| `ldr` at 0xb5fdc | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb5fe0 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb5fe4 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xb5fec | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+4` |
| `ldr` at 0xb5ff0 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb5ff4 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb5ff8 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xb6000 | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+4` |
| `ldr` at 0xb6004 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb6008 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb600c | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xb6014 | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+4` |
| `ldr` at 0xb6018 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb601c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb6020 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xb6028 | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+4` |
| `ldr` at 0xb602c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb6030 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb6034 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xb603c | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+4` |
| `ldr` at 0xb6040 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb6044 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb6048 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb604c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb6054 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xb605c | store | 4 | `[sp+16] +0x0` | `?` |
| `ldr` at 0xb6060 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xb6064 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb6068 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xb6074 | store | 4 | `[sp+12] +0x0` | `ret(0x1943c)` |
| `ldr` at 0xb6078 | load | 4 | `[sp+12] +0x0` | `-` |
| `str` at 0xb607c | store | 4 | `sp -0x2c` | `[[sp+12]]` |
| `ldr` at 0xb6084 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0xb6090 | store | 1 | `[[sp+12]] +0x0` | `0x6` |
| `str` at 0xb6098 | store | 4 | `sp -0x2c` | `[[sp+12]]+1` |
| `ldr` at 0xb60a0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb60a4 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xb60b0 | store | 1 | `[[sp+12]] +0x1` | `([sp-52] & 0xff)` |
| `str` at 0xb60b8 | store | 4 | `sp -0x2c` | `[[sp+12]]+2` |
| `ldr` at 0xb60c0 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0xb60cc | store | 1 | `[[sp+12]] +0x2` | `0x0` |
| `str` at 0xb60d4 | store | 4 | `sp -0x2c` | `[[sp+12]]+3` |
| `ldr` at 0xb60dc | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0xb60e8 | store | 1 | `[[sp+12]] +0x3` | `0x0` |
| `str` at 0xb60f0 | store | 4 | `sp -0x2c` | `[[sp+12]]+4` |
| `ldr` at 0xb60f4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb60f8 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xb6100 | store | 1 | `[[sp+12]] +0x4` | `?` |
| `ldr` at 0xb6108 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb6110 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xb6118 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0xb6120 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb6128 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xb6130 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0xb6138 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb6140 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xb6144 | store | 1 | `[sp-44] +0x3` | `([sp] & 0xff)` |
| `ldr` at 0xb6148 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb6150 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0xb6154 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb6158 | load | 4 | `sp +0x4` | `-` |
| `strb` at 0xb6160 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0xb6168 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb6170 | load | 4 | `sp +0x4` | `-` |
| `strb` at 0xb6178 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0xb6180 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb6188 | load | 4 | `sp +0x4` | `-` |
| `strb` at 0xb6190 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0xb6198 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb61a0 | load | 4 | `sp +0x4` | `-` |
| `strb` at 0xb61a4 | store | 1 | `[sp-44] +0x3` | `([sp+4] & 0xff)` |
| `ldr` at 0xb61a8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb61b0 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0xb61b4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb61b8 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xb61c0 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0xb61c8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb61d0 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xb61d8 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0xb61e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb61e8 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xb61f0 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0xb61f8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb6200 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xb6204 | store | 1 | `[sp-44] +0x3` | `([sp-24] & 0xff)` |
| `ldr` at 0xb6208 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb6210 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0xb6214 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb6218 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xb6220 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0xb6228 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb6230 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xb6238 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0xb6240 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb6248 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xb6250 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0xb6258 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb6260 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xb6264 | store | 1 | `[sp-44] +0x3` | `([sp-28] & 0xff)` |
| `ldr` at 0xb6268 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb6270 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0xb6274 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb6278 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xb6280 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0xb6288 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb6290 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xb6298 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0xb62a0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb62a8 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xb62b0 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0xb62b8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb62c0 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xb62c4 | store | 1 | `[sp-44] +0x3` | `([sp-32] & 0xff)` |
| `ldr` at 0xb62c8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb62d0 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0xb62d4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb62d8 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xb62e0 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0xb62e8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb62f0 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xb62f8 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0xb6300 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb6308 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xb6310 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0xb6318 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb6320 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xb6324 | store | 1 | `[sp-44] +0x3` | `([sp-36] & 0xff)` |
| `ldr` at 0xb6328 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb6330 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0xb6334 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb6338 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0xb6340 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0xb6348 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb6350 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0xb6358 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0xb6360 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb6368 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0xb6370 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0xb6378 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb6380 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0xb6384 | store | 1 | `[sp-44] +0x3` | `([sp-48] & 0xff)` |
| `ldr` at 0xb6388 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb6390 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0xb6394 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xb6398 | store | 4 | `sp -0x28` | `[sp+8]` |
| `ldr` at 0xb639c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb63a8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb63ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb63b8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb63c0 | store | 4 | `sp -0x2c` | `[sp-44]+168` |
| `ldr` at 0xb63c4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb63c8 | load | 4 | `[sp-40] +0xa8` | `-` |
| `str` at 0xb63cc | store | 4 | `sp -0x28` | `[[sp-40]+168]` |
| `ldm` at 0xb63d8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb63d8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb63d8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb63d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb606c | `(unknown callee) 0x1943c` | `[[sp+16]]` | `[sp+16]` | `?` | `[sp+16]` |
| `bl` at 0xb63b4 | `(unknown callee) 0x19610` | `?` | `?` | `0xa8` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb5f44 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb63a4 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb63d8 | `r0` | `?` |

## `FRUUnpackEvents`

- `FRUUnpackEvents`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb6e88 to 0xb73f0.
  Established. Equality `[[sp-24]] == 0x6` takes `0xb6f00`, while inequality falls through (`beq` at 0xb6eec).
  Included word stores write `0x5` to `[sp+12] +0x0` and clear `[sp-36] +0xa8` (`str` at 0xb6ef8, `str` at 0xb7280).
  Event-record unpacking bounds remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb6e98 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb6e9c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb6ea0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb6ea4 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xb6ea8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xb6eb0 | store | 4 | `[sp+16] +0x0` | `0x0` |
| `ldr` at 0xb6eb4 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xb6ebc | store | 4 | `[sp+20] +0x0` | `0x0` |
| `ldr` at 0xb6ec0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb6ec8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb6ed0 | load | 1 | `[sp-20] +0x0` | `-` |
| `str` at 0xb6ed4 | store | 4 | `[sp-24] +0x0` | `[[sp-20]]:1` |
| `str` at 0xb6edc | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0xb6ee0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb6ee4 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xb6ef0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xb6ef8 | store | 4 | `[sp+12] +0x0` | `0x5` |
| `ldr` at 0xb6f04 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb6f0c | load | 1 | `[sp-20] +0x1` | `-` |
| `str` at 0xb6f10 | store | 4 | `sp -0x38` | `[[sp-20]+1]:1` |
| `str` at 0xb6f18 | store | 4 | `sp -0x14` | `[sp-20]+2` |
| `ldr` at 0xb6f20 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb6f28 | load | 1 | `[sp-20] +0x2` | `-` |
| `str` at 0xb6f2c | store | 4 | `sp -0x30` | `[[sp-20]+2]:1` |
| `str` at 0xb6f34 | store | 4 | `sp -0x14` | `[sp-20]+3` |
| `ldr` at 0xb6f3c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb6f44 | load | 1 | `[sp-20] +0x3` | `-` |
| `str` at 0xb6f48 | store | 4 | `sp -0x34` | `[[sp-20]+3]:1` |
| `str` at 0xb6f50 | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xb6f54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb6f58 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb6f5c | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xb6f68 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb6f70 | load | 1 | `[sp-20] +0x5` | `-` |
| `ldr` at 0xb6f80 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb6f88 | load | 1 | `[sp-20] +0x6` | `-` |
| `ldr` at 0xb6f98 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb6fa0 | load | 1 | `[sp-20] +0x7` | `-` |
| `str` at 0xb6fa8 | store | 4 | `[sp-28] +0x0` | `?` |
| `ldr` at 0xb6fac | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb6fb4 | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xb6fb8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb6fbc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb6fc0 | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xb6fcc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb6fd4 | load | 1 | `[sp-20] +0x5` | `-` |
| `ldr` at 0xb6fe4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb6fec | load | 1 | `[sp-20] +0x6` | `-` |
| `ldr` at 0xb6ffc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb7004 | load | 1 | `[sp-20] +0x7` | `-` |
| `str` at 0xb700c | store | 4 | `[sp-32] +0x0` | `?` |
| `ldr` at 0xb7010 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb7018 | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xb701c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xb7020 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb7024 | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xb7030 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb7038 | load | 1 | `[sp-20] +0x5` | `-` |
| `ldr` at 0xb7048 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb7050 | load | 1 | `[sp-20] +0x6` | `-` |
| `ldr` at 0xb7060 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb7068 | load | 1 | `[sp-20] +0x7` | `-` |
| `str` at 0xb7070 | store | 4 | `[sp] +0x0` | `?` |
| `ldr` at 0xb7074 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb707c | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xb7080 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xb7084 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb7088 | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xb7094 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb709c | load | 1 | `[sp-20] +0x5` | `-` |
| `ldr` at 0xb70ac | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb70b4 | load | 1 | `[sp-20] +0x6` | `-` |
| `ldr` at 0xb70c4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb70cc | load | 1 | `[sp-20] +0x7` | `-` |
| `str` at 0xb70d4 | store | 4 | `[sp+4] +0x0` | `?` |
| `ldr` at 0xb70d8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb70e0 | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xb70e4 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xb70e8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb70ec | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xb70f8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb7100 | load | 1 | `[sp-20] +0x5` | `-` |
| `ldr` at 0xb7110 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb7118 | load | 1 | `[sp-20] +0x6` | `-` |
| `ldr` at 0xb7128 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb7130 | load | 1 | `[sp-20] +0x7` | `-` |
| `str` at 0xb7138 | store | 4 | `[sp+8] +0x0` | `?` |
| `ldr` at 0xb713c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb7144 | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xb7148 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xb714c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb7150 | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xb715c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb7164 | load | 1 | `[sp-20] +0x5` | `-` |
| `ldr` at 0xb7174 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb717c | load | 1 | `[sp-20] +0x6` | `-` |
| `ldr` at 0xb718c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb7194 | load | 1 | `[sp-20] +0x7` | `-` |
| `str` at 0xb719c | store | 4 | `[sp+12] +0x0` | `?` |
| `ldr` at 0xb71a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb71a8 | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xb71ac | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb71b0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb71b4 | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xb71c0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb71c8 | load | 1 | `[sp-20] +0x5` | `-` |
| `ldr` at 0xb71d8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb71e0 | load | 1 | `[sp-20] +0x6` | `-` |
| `ldr` at 0xb71f0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb71f8 | load | 1 | `[sp-20] +0x7` | `-` |
| `str` at 0xb7200 | store | 4 | `[sp+16] +0x0` | `?` |
| `ldr` at 0xb7204 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb720c | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xb7210 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xb721c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xb7224 | store | 4 | `[sp+12] +0x0` | `0x7` |
| `str` at 0xb7230 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xb7234 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb7238 | load | 4 | `[sp+16] +0x0` | `-` |
| `ldr` at 0xb723c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb7254 | store | 4 | `sp -0x24` | `ret(0x1943c)` |
| `ldr` at 0xb7258 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb725c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7260 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xb7268 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb726c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xb7274 | store | 4 | `sp -0x14` | `([sp-20] + [sp-56])` |
| `ldr` at 0xb7278 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb7280 | store | 4 | `[sp-36] +0xa8` | `0x0` |
| `ldr` at 0xb7284 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb7288 | load | 4 | `[sp-36] +0x10` | `-` |
| `str` at 0xb728c | store | 4 | `sp -0x3c` | `[[sp-36]+16]` |
| `ldr` at 0xb7290 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb729c | store | 4 | `sp -0x3c` | `ret(0x19280)` |
| `ldr` at 0xb72a0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb72a4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb72a8 | store | 4 | `[sp-36] +0x10` | `ret(0x19280)` |
| `ldr` at 0xb72ac | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xb72b0 | load | 2 | `[sp-36] +0x18` | `-` |
| `str` at 0xb72b4 | store | 4 | `sp -0x3c` | `[[sp-36]+24]:2` |
| `ldr` at 0xb72b8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb72d8 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xb72dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb72e0 | load | 4 | `sp -0x3c` | `-` |
| `strh` at 0xb72e4 | store | 2 | `[sp-36] +0x18` | `([sp-60] & 0xffff)` |
| `ldr` at 0xb72e8 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xb72ec | load | 2 | `[sp-36] +0x1a` | `-` |
| `str` at 0xb72f0 | store | 4 | `sp -0x3c` | `[[sp-36]+26]:2` |
| `ldr` at 0xb72f4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb7314 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xb7318 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb731c | load | 4 | `sp -0x3c` | `-` |
| `strh` at 0xb7320 | store | 2 | `[sp-36] +0x1a` | `([sp-60] & 0xffff)` |
| `ldr` at 0xb7324 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xb7328 | load | 2 | `[sp-36] +0x1c` | `-` |
| `str` at 0xb732c | store | 4 | `sp -0x3c` | `[[sp-36]+28]:2` |
| `ldr` at 0xb7330 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb7350 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xb7354 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb7358 | load | 4 | `sp -0x3c` | `-` |
| `strh` at 0xb735c | store | 2 | `[sp-36] +0x1c` | `([sp-60] & 0xffff)` |
| `ldr` at 0xb7360 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xb7364 | load | 2 | `[sp-36] +0x1e` | `-` |
| `str` at 0xb7368 | store | 4 | `sp -0x3c` | `[[sp-36]+30]:2` |
| `ldr` at 0xb736c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb738c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xb7390 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb7394 | load | 4 | `sp -0x3c` | `-` |
| `strh` at 0xb7398 | store | 2 | `[sp-36] +0x1e` | `([sp-60] & 0xffff)` |
| `ldr` at 0xb739c | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xb73a0 | load | 4 | `[sp+20] +0x0` | `-` |
| `ldr` at 0xb73ac | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xb73b0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb73b4 | store | 4 | `[sp+20] +0x0` | `[sp-36]` |
| `ldr` at 0xb73b8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb73bc | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0xb73c4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb73c8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb73cc | store | 4 | `[sp-40] +0xa8` | `[sp-36]` |
| `ldr` at 0xb73d0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb73d4 | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0xb73d8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb73e0 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldm` at 0xb73ec | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb73ec | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb73ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb724c | `(unknown callee) 0x1943c` | `0xac` | `?` | `[[sp+16]]` | `?` |
| `bl` at 0xb7264 | `(unknown callee) 0x19610` | `ret(0x1943c)` | `[sp-20]` | `[sp-56]` | `ret(0x1943c)` |
| `bl` at 0xb7294 | `(unknown callee) 0x19280` | `[[sp-36]+16]` | `?` | `[sp-36]` | `[[sp-36]+16]` |
| `bl` at 0xb72c8 | `(unknown callee) 0x19514` | `?` | `?` | `[sp-36]` | `?` |
| `bl` at 0xb7304 | `(unknown callee) 0x19514` | `?` | `?` | `[sp-36]` | `?` |
| `bl` at 0xb7340 | `(unknown callee) 0x19514` | `?` | `?` | `[sp-36]` | `?` |
| `bl` at 0xb737c | `(unknown callee) 0x19514` | `?` | `?` | `[sp-36]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb6eec | `[[sp-24]] == 0x6` | `[[sp-24]] != 0x6` |
| `beq` at 0xb7218 | `[sp-56] == 0xa8` | `[sp-56] != 0xa8` |
| `ble` at 0xb7244 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xb73a8 | `[[sp+20]] != 0x0` | `[[sp+20]] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb73ec | `r0` | `?` |

## `_FRUDump`

- `_FRUDump`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7504 to 0xb7a10.
  Established. A zero `[sp+12]` takes `0xb758c`, while nonzero falls through (`beq` at 0xb7584).
  Included unnamed `0xb73f0` calls carry `[sp-20]`, `[sp-24]`, `sp-124`, `sp-124` and `[sp-20]`, `[sp-24]`, `[sp+24]`, `[sp+24]` (`bl` at 0xb7550, `bl` at 0xb7978).
  FRU report output callback contract remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb7514 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb7518 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb751c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb7520 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xb7524 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xb7538 | load | 4 | `abs 0xb79ec` | `-` |
| `ldr` at 0xb7544 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7548 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb755c | load | 4 | `abs 0xb79f0` | `-` |
| `ldr` at 0xb7560 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb756c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7570 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb757c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xb75a4 | load | 4 | `abs 0xb79f4` | `-` |
| `ldr` at 0xb75b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb75b4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb75c4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb75c8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb75cc | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xb75d8 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb75e8 | store | 4 | `sp -0x9c` | `ret(0xb8050)` |
| `ldr` at 0xb75f0 | load | 4 | `abs 0xb79f8` | `-` |
| `ldr` at 0xb75f4 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xb75f8 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xb7604 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7608 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7614 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb7628 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xb7630 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb7634 | store | 4 | `sp -0x80` | `?` |
| `ldr` at 0xb7638 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xb7644 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xb7650 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xb7660 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xb7674 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb7688 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xb7694 | load | 4 | `abs 0xb79fc` | `-` |
| `ldr` at 0xb7698 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb76a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb76bc | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xb76c8 | load | 4 | `abs 0xb7a00` | `-` |
| `ldr` at 0xb76cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb76dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb76f0 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xb76fc | load | 4 | `abs 0xb7a04` | `-` |
| `ldr` at 0xb7700 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb770c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7710 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb7720 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xb7724 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb7738 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xb7740 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb7744 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7754 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb7768 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xb7770 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7778 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb7780 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb7794 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xb779c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb77a4 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xb77ac | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb77b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb77c4 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xb77cc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb77d4 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xb77dc | load | 1 | `(unknown)` | `-` |
| `str` at 0xb77e0 | store | 4 | `sp -0x9c` | `?` |
| `ldr` at 0xb77e4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb77f8 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xb7800 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7808 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xb7810 | load | 1 | `(unknown)` | `-` |
| `str` at 0xb7814 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0xb7818 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb782c | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xb7834 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb783c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xb7844 | load | 1 | `(unknown)` | `-` |
| `str` at 0xb7848 | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0xb784c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb7860 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xb7868 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7870 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xb7878 | load | 1 | `(unknown)` | `-` |
| `str` at 0xb787c | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xb7880 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb7894 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xb789c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb78a4 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xb78ac | load | 1 | `(unknown)` | `-` |
| `str` at 0xb78b0 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0xb78b4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb78c8 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xb78d0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb78d8 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xb78e0 | load | 1 | `(unknown)` | `-` |
| `str` at 0xb78e4 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xb78e8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb78fc | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xb7904 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb790c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xb7914 | load | 1 | `(unknown)` | `-` |
| `str` at 0xb7918 | store | 4 | `sp -0x84` | `?` |
| `ldr` at 0xb791c | load | 4 | `abs 0xb7a08` | `-` |
| `ldr` at 0xb7920 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb7930 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7934 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7940 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb7948 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xb7950 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb7958 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xb7960 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xb796c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7970 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7974 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xb797c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7980 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7984 | load | 4 | `abs 0xb7a0c` | `-` |
| `ldr` at 0xb798c | load | 4 | `sp +0x1c` | `-` |
| `ldr` at 0xb7998 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb799c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb79a0 | load | 4 | `sp +0x1c` | `-` |
| `ldr` at 0xb79a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb79ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb79b0 | load | 4 | `abs 0xb7a0c` | `-` |
| `ldr` at 0xb79b8 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb79c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb79c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb79cc | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xb79d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb79d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb79dc | load | 4 | `abs 0xb7a0c` | `-` |
| `ldm` at 0xb79e8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb79e8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb79e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7528 | `(unknown callee) 0xb7424` | `[sp+12]` | `r1` | `r2` | `r3` |
| `bl` at 0xb753c | `(unknown callee) 0x197a8` | `sp-124` | `0x11e7f4` | `ret(0xb7424)` | `sp-124` |
| `bl` at 0xb7550 | `(unknown callee) 0xb73f0` | `[sp-20]` | `[sp-24]` | `sp-124` | `sp-124` |
| `bl` at 0xb7564 | `(unknown callee) 0x197a8` | `sp-124` | `0x11e804` | `[sp-28]` | `sp-124` |
| `bl` at 0xb7578 | `(unknown callee) 0xb73f0` | `[sp-20]` | `[sp-24]` | `sp-124` | `sp-124` |
| `bl` at 0xb7594 | `(unknown callee) 0x19640` | `sp-32` | `?` | `?` | `sp-32` |
| `bl` at 0xb75a8 | `(unknown callee) 0x197a8` | `sp-124` | `0x11e818` | `ret(0x19640)` | `sp-124` |
| `bl` at 0xb75bc | `(unknown callee) 0xb73f0` | `[sp-20]` | `[sp-24]` | `sp-124` | `sp-124` |
| `bl` at 0xb75dc | `(unknown callee) 0xb8050` | `[sp]` | `?` | `?` | `[sp+16]` |
| `bl` at 0xb75fc | `(unknown callee) 0x197a8` | `sp-124` | `0x11e828` | `[sp+4]` | `[sp+8]` |
| `bl` at 0xb7610 | `(unknown callee) 0xb73f0` | `[sp-20]` | `[sp-24]` | `sp-124` | `sp-124` |
| `bl` at 0xb769c | `(unknown callee) 0x197a8` | `sp-124` | `0x11e838` | `?` | `?` |
| `bl` at 0xb76d0 | `(unknown callee) 0x197a8` | `sp-124` | `0x11e844` | `?` | `?` |
| `bl` at 0xb7704 | `(unknown callee) 0x197a8` | `sp-124` | `0x11e850` | `?` | `?` |
| `bl` at 0xb7718 | `(unknown callee) 0xb73f0` | `[sp-20]` | `[sp-24]` | `sp-124` | `sp-124` |
| `bl` at 0xb7928 | `(unknown callee) 0x197a8` | `sp-124` | `0x11e858` | `?` | `?` |
| `bl` at 0xb793c | `(unknown callee) 0xb73f0` | `[sp-20]` | `[sp-24]` | `sp-124` | `sp-124` |
| `bl` at 0xb7978 | `(unknown callee) 0xb73f0` | `[sp-20]` | `[sp-24]` | `[sp+24]` | `[sp+24]` |
| `bl` at 0xb7988 | `(unknown callee) 0xb73f0` | `[sp-20]` | `[sp-24]` | `0x11e714` | `?` |
| `bl` at 0xb79a4 | `(unknown callee) 0xb73f0` | `[sp-20]` | `[sp-24]` | `[sp+28]` | `[sp+28]` |
| `bl` at 0xb79b4 | `(unknown callee) 0xb73f0` | `[sp-20]` | `[sp-24]` | `0x11e714` | `?` |
| `bl` at 0xb79d0 | `(unknown callee) 0xb73f0` | `[sp-20]` | `[sp-24]` | `[sp+32]` | `[sp+32]` |
| `bl` at 0xb79e0 | `(unknown callee) 0xb73f0` | `[sp-20]` | `[sp-24]` | `0x11e714` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb7584 | `[sp+12] == 0x0` | `[sp+12] != 0x0` |
| `bge` at 0xb75d4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xb7640 | `[sp-128] == 0x2` | `[sp-128] != 0x2` |
| `bgt` at 0xb764c | `[sp-128] > 0x2 (signed, 32-bit)` | `[sp-128] <= 0x2 (signed, 32-bit)` |
| `beq` at 0xb7658 | `[sp-128] == 0x1` | `[sp-128] != 0x1` |
| `beq` at 0xb7668 | `[sp-128] == 0x3` | `[sp-128] != 0x3` |
| `ble` at 0xb774c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xb7968 | `[sp+24] == 0x0` | `[sp+24] != 0x0` |
| `beq` at 0xb7994 | `[sp+28] == 0x0` | `[sp+28] != 0x0` |
| `beq` at 0xb79c0 | `[sp+32] == 0x0` | `[sp+32] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb79e8 | `r0` | `ret(0xb73f0)` |

## `FRUDumpBuff`

- `FRUDumpBuff`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7c2c to 0xb7c7c.
  Established. The unnamed `0xb7a10` call carries `0xb8988`, `ret(0xb88c8)`, `[sp-20]`, `ret(0xb88c8)` (`bl` at 0xb7c58).
  A later unnamed `0xb8918` call carries `[sp-24]`, `0x0` and two unknown candidates (`bl` at 0xb7c64).
  Dump-buffer ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb7c3c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb7c48 | store | 4 | `sp -0x18` | `ret(0xb88c8)` |
| `ldr` at 0xb7c4c | load | 4 | `abs 0xb7c78` | `-` |
| `ldr` at 0xb7c50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7c54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7c5c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xb7c74 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb7c74 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb7c74 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7c40 | `(unknown callee) 0xb88c8` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xb7c58 | `(unknown callee) 0xb7a10` | `0xb8988` | `ret(0xb88c8)` | `[sp-20]` | `ret(0xb88c8)` |
| `bl` at 0xb7c64 | `(unknown callee) 0xb8918` | `[sp-24]` | `0x0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb7c74 | `r0` | `ret(0xb8918)` |

## `FRUDump`

- `FRUDump`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7a10 to 0xb7c2c.
  Established. The unnamed `0xb63dc` call carries entry `r2`, `sp-32`, `sp-36`, `sp-40` (`bl` at 0xb7aa8).
  A zero `[sp-64]` takes `0xb7bd4`, while nonzero falls through (`beq` at 0xb7b18).
  The unnamed `0xb7504` call carries `[sp-20]`, `[sp-24]`, `[sp-32]`, `[sp-36]` (`bl` at 0xb7b04).
  FRU dump record structure beyond these candidates remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb7a20 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb7a24 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb7a28 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb7a30 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xb7a38 | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0xb7a40 | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0xb7a54 | store | 4 | `sp -0xc4` | `sp-44` |
| `str` at 0xb7a5c | store | 4 | `sp -0xc0` | `sp-48` |
| `str` at 0xb7a64 | store | 4 | `sp -0xbc` | `sp-52` |
| `str` at 0xb7a6c | store | 4 | `sp -0xb8` | `sp-56` |
| `str` at 0xb7a74 | store | 4 | `sp -0xb4` | `sp-60` |
| `str` at 0xb7a7c | store | 4 | `sp -0xb0` | `sp-64` |
| `str` at 0xb7a84 | store | 4 | `sp -0xac` | `sp-72` |
| `str` at 0xb7a8c | store | 4 | `sp -0xa8` | `sp-76` |
| `str` at 0xb7a94 | store | 4 | `sp -0xa4` | `sp-80` |
| `ldr` at 0xb7a98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb7aac | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb7ab0 | store | 4 | `sp -0xc4` | `[sp-44]` |
| `ldr` at 0xb7ab4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb7ab8 | store | 4 | `sp -0xc0` | `[sp-48]` |
| `ldr` at 0xb7abc | load | 4 | `sp -0x34` | `-` |
| `str` at 0xb7ac0 | store | 4 | `sp -0xbc` | `[sp-52]` |
| `ldr` at 0xb7ac4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xb7ac8 | store | 4 | `sp -0xb8` | `[sp-56]` |
| `ldr` at 0xb7acc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb7ad0 | store | 4 | `sp -0xb4` | `[sp-60]` |
| `ldr` at 0xb7ad4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xb7ad8 | store | 4 | `sp -0xb0` | `[sp-64]` |
| `ldr` at 0xb7adc | load | 4 | `sp -0x48` | `-` |
| `str` at 0xb7ae0 | store | 4 | `sp -0xac` | `[sp-72]` |
| `ldr` at 0xb7ae4 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xb7ae8 | store | 4 | `sp -0xa8` | `[sp-76]` |
| `ldr` at 0xb7aec | load | 4 | `sp -0x50` | `-` |
| `str` at 0xb7af0 | store | 4 | `sp -0xa4` | `[sp-80]` |
| `ldr` at 0xb7af4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7af8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7afc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb7b00 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb7b0c | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xb7b10 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb7b1c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xb7b20 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xb7b2c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xb7b40 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb7b48 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb7b54 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xb7b68 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb7b70 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb7b78 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xb7b8c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb7b94 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb7ba0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xb7bb4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb7bbc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb7bc4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xb7bcc | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xb7bd4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb7be0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb7be8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xb7bf4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xb7bfc | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xb7c08 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xb7c10 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xb7c1c | load | 4 | `sp -0x50` | `-` |
| `ldm` at 0xb7c28 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb7c28 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb7c28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7aa8 | `(unknown callee) 0xb63dc` | `r2` | `sp-32` | `sp-36` | `sp-40` |
| `bl` at 0xb7b04 | `(unknown callee) 0xb7504` | `[sp-20]` | `[sp-24]` | `[sp-32]` | `[sp-36]` |
| `bl` at 0xb7b74 | `(unknown callee) 0x1970c` | `?` | `?` | `?` | `?` |
| `bl` at 0xb7bc0 | `(unknown callee) 0x1970c` | `?` | `?` | `?` | `?` |
| `bl` at 0xb7be4 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `[sp-64]` |
| `bl` at 0xb7bf8 | `(unknown callee) 0x1970c` | `[sp-72]` | `?` | `?` | `[sp-72]` |
| `bl` at 0xb7c0c | `(unknown callee) 0x1970c` | `[sp-76]` | `?` | `?` | `[sp-76]` |
| `bl` at 0xb7c20 | `(unknown callee) 0x1970c` | `[sp-80]` | `?` | `?` | `[sp-80]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb7b18 | `[sp-64] == 0x0` | `[sp-64] != 0x0` |
| `bge` at 0xb7b28 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xb7b50 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb7b9c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb7bdc | `[sp-64] == 0x0` | `[sp-64] != 0x0` |
| `beq` at 0xb7bf0 | `[sp-72] == 0x0` | `[sp-72] != 0x0` |
| `beq` at 0xb7c04 | `[sp-76] == 0x0` | `[sp-76] != 0x0` |
| `beq` at 0xb7c18 | `[sp-80] == 0x0` | `[sp-80] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb7c28 | `r0` | `?` |

## `brickIsCompute`

- `brickIsCompute`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb87b8 to 0xb88c8.
  Established. Equality entry `r0 == 0x43` takes `0xb88b0`, while inequality falls through (`beq` at 0xb87dc).
  Included word stores write `0x0` and `0x1` to `sp -0x18` (`str` at 0xb87d0, `str` at 0xb88b4).
  Product classification associated with these values remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb87c8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb87d0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xb87d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb87e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb87ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb87f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8804 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8810 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb881c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8828 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8834 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8840 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb884c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8858 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8864 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8870 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb887c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8888 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8894 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb88a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb88b4 | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0xb88b8 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xb88c4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb88c4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb88c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb87dc | `r0 == 0x43` | `r0 != 0x43` |
| `beq` at 0xb87e8 | `r0 == 0x33` | `r0 != 0x33` |
| `beq` at 0xb87f4 | `r0 == 0x35` | `r0 != 0x35` |
| `beq` at 0xb8800 | `r0 == 0x34` | `r0 != 0x34` |
| `beq` at 0xb880c | `r0 == 0x41` | `r0 != 0x41` |
| `beq` at 0xb8818 | `r0 == 0x36` | `r0 != 0x36` |
| `beq` at 0xb8824 | `r0 == 0x39` | `r0 != 0x39` |
| `beq` at 0xb8830 | `r0 == 0x37` | `r0 != 0x37` |
| `beq` at 0xb883c | `r0 == 0x38` | `r0 != 0x38` |
| `beq` at 0xb8848 | `r0 == 0x30` | `r0 != 0x30` |
| `beq` at 0xb8854 | `r0 == 0x51` | `r0 != 0x51` |
| `beq` at 0xb8860 | `r0 == 0x40` | `r0 != 0x40` |
| `beq` at 0xb886c | `r0 == 0x4f` | `r0 != 0x4f` |
| `beq` at 0xb8878 | `r0 == 0x42` | `r0 != 0x42` |
| `beq` at 0xb8884 | `r0 == 0x46` | `r0 != 0x46` |
| `beq` at 0xb8890 | `r0 == 0x4c` | `r0 != 0x4c` |
| `beq` at 0xb889c | `r0 == 0x4d` | `r0 != 0x4d` |
| `beq` at 0xb88a8 | `r0 == 0x7c` | `r0 != 0x7c` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb88c4 | `r0` | `?` |

## `UtilFmtBrick`

- `UtilFmtBrick`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5f954 to 0x5f9ec.
  Established. Included unnamed `0x19748` calls carry `[sp-20]`, `0x2d`, `0x3`, `[sp-24]` and `[sp-20]+4`, `0x2d`, `0x2`, `[sp-20]+4` (`bl` at 0x5f9b8, `bl` at 0x5f9dc).
  A nonzero `[sp-32]` takes `0x5f9e0`, while zero falls through (`bne` at 0x5f9c4).
  Brick label output length remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5f964 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x5f968 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x5f96c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x5f970 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x5f974 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5f984 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5f988 | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0x5f98c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5f990 | load | 4 | `abs 0x5f9e8` | `-` |
| `ldr` at 0x5f994 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5f9a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5f9ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5f9bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5f9c8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x5f9e4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x5f9e4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x5f9e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5f978 | `(unknown callee) 0xb8484` | `r2` | `r1` | `r2` | `r3` |
| `bl` at 0x5f99c | `(unknown callee) 0x197a8` | `[sp-20]` | `0xfbbc8` | `[sp-24]` | `?` |
| `bl` at 0x5f9b8 | `(unknown callee) 0x19748` | `[sp-20]` | `0x2d` | `0x3` | `[sp-24]` |
| `bl` at 0x5f9dc | `(unknown callee) 0x19748` | `[sp-20]+4` | `0x2d` | `0x2` | `[sp-20]+4` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5f9a8 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bne` at 0x5f9c4 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x5f9e4 | `r0` | `?` |

## `UtilDspBrick`

- `UtilDspBrick`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5f70c to 0x5f954.
  Established. The unnamed `0x5f954` call carries `sp-36`, `[ret(0x6a0f8)+8]`, `[ret(0x6a0f8)+16]`, `[ret(0x6a0f8)+12]` (`bl` at 0x5f74c).
  An included unnamed `0x22e24` call carries `0x31`, `sp-36`, `0x0`, `sp-36` on known edges (`bl` at 0x5f940).
  Brick display cases behind the computed transfer remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5f728 | store | 4 | `sp -0x28` | `ret(0x6a0f8)` |
| `ldr` at 0x5f730 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5f734 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5f738 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5f740 | load | 4 | `ret(0x6a0f8) +0x8` | `-` |
| `ldr` at 0x5f744 | load | 4 | `ret(0x6a0f8) +0x10` | `-` |
| `ldr` at 0x5f748 | load | 4 | `ret(0x6a0f8) +0xc` | `-` |
| `ldr` at 0x5f750 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5f754 | load | 4 | `[sp-40] +0x10` | `-` |
| `ldrls` at 0x5f760 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x5f948 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x5f948 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x5f948 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x5f948 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5f720 | `(unknown callee) 0x6a0f8` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x5f74c | `(unknown callee) 0x5f954` | `sp-36` | `[ret(0x6a0f8)+8]` | `[ret(0x6a0f8)+16]` | `[ret(0x6a0f8)+12]` |
| `bl` at 0x5f940 | `(unknown callee) 0x22e24` | `0x31` | `sp-36` | `0x0` | `sp-36` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x5f760 | `[[sp-40]+16]-35 <= 0x59 (unsigned, 32-bit)` | `[[sp-40]+16]-35 > 0x59 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x5f948 | `r0` | `ret(0x22e24)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x5f760).

## `UtilPortStr`

- `UtilPortStr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5fa6c to 0x5fcb0.
  Established. Equality entry `r0 == 0x40` takes `0x5fa94`, while inequality falls through (`beq` at 0x5fa8c).
  Included word stores write `0xfbc28` to `sp -0x1c` on known edges (`str` at 0x5fb6c, `str` at 0x5fc4c).
  Port string mappings behind the computed transfers remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5fa7c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x5fa80 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x5fa84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5fa94 | load | 4 | `sp -0x18` | `-` |
| `ldrls` at 0x5faa0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5fb68 | load | 4 | `abs 0x5fc8c` | `-` |
| `str` at 0x5fb6c | store | 4 | `sp -0x1c` | `0xfbc28` |
| `ldr` at 0x5fb74 | load | 4 | `sp -0x18` | `-` |
| `ldrls` at 0x5fb80 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5fc48 | load | 4 | `abs 0x5fc8c` | `-` |
| `str` at 0x5fc4c | store | 4 | `sp -0x1c` | `0xfbc28` |
| `ldr` at 0x5fc50 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x5fc58 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x5fc58 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x5fc58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5fa8c | `r0 == 0x40` | `r0 != 0x40` |
| `ldrls` at 0x5faa0 | `r1-1 <= 0xb (unsigned, 32-bit)` | `r1-1 > 0xb (unsigned, 32-bit)` |
| `ldrls` at 0x5fb80 | `r1-1 <= 0xb (unsigned, 32-bit)` | `r1-1 > 0xb (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x5fc58 | `r0` | `0xfbc28` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x5faa0).

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x5fb80).

## `MInfoReset`

- `MInfoReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb9f94 to 0xba034.
  Established. Word stores clear `0x133f44` and `0x133f48` (`str` at 0xb9fc0, `str` at 0xb9fcc).
  The unnamed `0x19748` call at the tail carries `0x133f60`, `0x0`, `0x30000`, unknown (`bl` at 0xba00c).
  A nonzero `[0x133f5c]` takes `0xb9fb8`, while zero falls through (`bne` at 0xb9fb0).
  Information-reset buffer purposes remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xb9fa4 | load | 4 | `abs 0xba014` | `-` |
| `ldr` at 0xb9fa8 | load | 4 | `abs 0x133f5c` | `-` |
| `ldr` at 0xb9fb8 | load | 4 | `abs 0xba018` | `-` |
| `str` at 0xb9fc0 | store | 4 | `abs 0x133f44` | `0x0` |
| `ldr` at 0xb9fc4 | load | 4 | `abs 0xba01c` | `-` |
| `str` at 0xb9fcc | store | 4 | `abs 0x133f48` | `0x0` |
| `ldr` at 0xb9fd0 | load | 4 | `abs 0xba020` | `-` |
| `ldr` at 0xb9fd8 | load | 4 | `abs 0xba024` | `-` |
| `ldr` at 0xb9fe0 | load | 4 | `abs 0xba028` | `-` |
| `ldr` at 0xb9fe8 | load | 4 | `abs 0xba024` | `-` |
| `ldr` at 0xb9ff0 | load | 4 | `abs 0xba02c` | `-` |
| `ldr` at 0xb9ff8 | load | 4 | `abs 0xba024` | `-` |
| `ldr` at 0xba000 | load | 4 | `abs 0xba030` | `-` |
| `ldm` at 0xba010 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xba010 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xba010 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xba010 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb9fdc | `(unknown callee) 0x19748` | `0x163f60` | `0x0` | `0x66664` | `0x0` |
| `bl` at 0xb9fec | `(unknown callee) 0x19748` | `0x1ca5c4` | `0x0` | `0x66664` | `?` |
| `bl` at 0xb9ffc | `(unknown callee) 0x19748` | `0x230c28` | `0x0` | `0x66664` | `?` |
| `bl` at 0xba00c | `(unknown callee) 0x19748` | `0x133f60` | `0x0` | `0x30000` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb9fb0 | `[0x133f5c] != 0x0` | `[0x133f5c] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xba010 | `r0` | `?` |
