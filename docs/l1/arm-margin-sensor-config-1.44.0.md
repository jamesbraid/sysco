# ARM margin and sensor configuration in 1.44.0

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
Computed transfers in margin initialization, monitor reset and sensor configuration remain
unresolved. Each affected entry identifies its computed transfer. Those tables cover known edges only.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/margin-sensor-config-facts.txt) retain all tool sections.

## `powerMargin`

- `powerMargin`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3801c to 0x386b4.
  Established. The absolute word at `0x12d6f8` receives `0x5` on an included path (`str` at 0x381b8).
  A later assignment to the same address writes `[sp-28]` (`str` at 0x38650). The unnamed
  `0x386b4` call carries `[sp-68]`, `[sp-20]`, `[sp-40]`, `[sp-28]` (`bl` at 0x38298). Margin
  control policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3802c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x38030 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x38034 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x38038 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0x38040 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x38044 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x38058 | load | 4 | `abs 0x38670` | `-` |
| `ldr` at 0x3805c | load | 4 | `abs 0x13219c` | `-` |
| `str` at 0x38064 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0x3806c | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x38074 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x38078 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3807c | load | 4 | `[sp-60] +0x8` | `-` |
| `ldr` at 0x38080 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3808c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x38090 | load | 4 | `[sp-60] +0x4` | `-` |
| `ldr` at 0x38094 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3809c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x380a8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x380ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x380c0 | load | 4 | `[sp-60] +0x0` | `-` |
| `ldrb` at 0x380c8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x380d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x380e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x380e4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x380f4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x380fc | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x38100 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x38108 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x38110 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x38120 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0x38128 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x38150 | store | 4 | `sp -0x34` | `ret(0x1943c)` |
| `ldr` at 0x38154 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x38160 | load | 4 | `abs 0x38674` | `-` |
| `str` at 0x38164 | store | 4 | `sp -0x64` | `0xf595c` |
| `ldr` at 0x38168 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3816c | store | 4 | `sp -0x60` | `[sp-20]` |
| `ldr` at 0x38174 | load | 4 | `abs 0x38678` | `-` |
| `ldr` at 0x38178 | load | 4 | `abs 0x3867c` | `-` |
| `ldr` at 0x38184 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x381a0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x381b0 | load | 4 | `abs 0x38680` | `-` |
| `str` at 0x381b8 | store | 4 | `abs 0x12d6f8` | `0x5` |
| `str` at 0x381c8 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x381d0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x381d4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x381d8 | load | 4 | `[sp-60] +0x8` | `-` |
| `ldr` at 0x381dc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x381e8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x381ec | load | 4 | `[sp-60] +0x4` | `-` |
| `ldr` at 0x381f0 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x381f8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x38204 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x38208 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3821c | load | 4 | `[sp-60] +0x0` | `-` |
| `ldrb` at 0x38224 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x38230 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3823c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38240 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x38250 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3826c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x38274 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x38278 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3827c | store | 4 | `(unknown)` | `[sp-32]` |
| `str` at 0x38284 | store | 4 | `(unknown)` | `sp-64` |
| `ldr` at 0x38288 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x3828c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x38290 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x38294 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x382a0 | store | 4 | `sp -0x24` | `ret(0x386b4)` |
| `ldr` at 0x382a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x382b0 | load | 4 | `abs 0x38684` | `-` |
| `ldr` at 0x382b4 | load | 4 | `abs 0x132644` | `-` |
| `ldr` at 0x382c0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x382c4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x382d8 | load | 4 | `[sp-60] +0x0` | `-` |
| `ldr` at 0x382e0 | load | 4 | `abs 0x38688` | `-` |
| `ldr` at 0x382e4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x382ec | load | 4 | `sp -0x44` | `-` |
| `str` at 0x382f4 | store | 4 | `[sp-68] +0xc` | `0x0` |
| `ldr` at 0x382f8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x38304 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x38308 | store | 4 | `sp -0x38` | `[sp-36]` |
| `ldr` at 0x38310 | load | 4 | `abs 0x38684` | `-` |
| `ldr` at 0x38314 | load | 4 | `abs 0x132644` | `-` |
| `ldr` at 0x38320 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x38324 | load | 4 | `sp -0x44` | `-` |
| `ldrb` at 0x38328 | load | 1 | `[sp-68] +0x25` | `-` |
| `ldr` at 0x3832c | load | 4 | `sp -0x44` | `-` |
| `ldrb` at 0x38330 | load | 1 | `[sp-68] +0x24` | `-` |
| `ldr` at 0x38334 | load | 4 | `abs 0x3868c` | `-` |
| `ldr` at 0x38338 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x38340 | load | 4 | `sp -0x44` | `-` |
| `ldc` at 0x38344 | load | unknown | `[sp-68] +0x20` | `-` |
| `ldr` at 0x3834c | load | 4 | `sp -0x44` | `-` |
| `ldrb` at 0x38350 | load | 1 | `[sp-68] +0x29` | `-` |
| `ldr` at 0x38354 | load | 4 | `sp -0x44` | `-` |
| `ldrb` at 0x38358 | load | 1 | `[sp-68] +0x26` | `-` |
| `str` at 0x3835c | store | 4 | `(unknown)` | `[[sp-68]+38]:1` |
| `ldr` at 0x38360 | load | 4 | `sp -0x44` | `-` |
| `ldrb` at 0x38364 | load | 1 | `[sp-68] +0x28` | `-` |
| `str` at 0x38368 | store | 4 | `(unknown)` | `[[sp-68]+40]:1` |
| `ldr` at 0x3836c | load | 4 | `abs 0x38690` | `-` |
| `stc` at 0x38370 | store | unknown | `(unknown)` | `?` |
| `ldr` at 0x38380 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x38384 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x38388 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x3838c | load | 4 | `sp -0x44` | `-` |
| `ldrb` at 0x38390 | load | 1 | `[sp-68] +0x2c` | `-` |
| `str` at 0x38394 | store | 4 | `(unknown)` | `[[sp-68]+44]:1` |
| `ldr` at 0x38398 | load | 4 | `sp -0x44` | `-` |
| `ldrb` at 0x3839c | load | 1 | `[sp-68] +0x2d` | `-` |
| `str` at 0x383a0 | store | 4 | `(unknown)` | `[[sp-68]+45]:1` |
| `ldr` at 0x383a4 | load | 4 | `abs 0x38694` | `-` |
| `ldr` at 0x383a8 | load | 4 | `[sp-68] +0x18` | `-` |
| `ldr` at 0x383ac | load | 4 | `[sp-68] +0x14` | `-` |
| `ldr` at 0x383b0 | load | 4 | `[sp-68] +0x30` | `-` |
| `ldr` at 0x383b8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x383c4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x383cc | store | 4 | `[sp-68] +0xc` | `0x0` |
| `ldr` at 0x383d0 | load | 4 | `abs 0x38684` | `-` |
| `ldr` at 0x383d4 | load | 4 | `abs 0x132644` | `-` |
| `ldr` at 0x383e0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x383e4 | load | 4 | `abs 0x38698` | `-` |
| `ldr` at 0x383e8 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x383f0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x383f8 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x383fc | load | 4 | `sp -0x28` | `-` |
| `str` at 0x38404 | store | 4 | `sp -0x28` | `?` |
| `str` at 0x38410 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x38414 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x38424 | store | 4 | `sp -0x44` | `0x0` |
| `str` at 0x38434 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x38438 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3843c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x38448 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x38464 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x3846c | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x38470 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x38474 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x38488 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x38498 | store | 4 | `sp -0x24` | `ret(0x38ee4)` |
| `ldr` at 0x3849c | load | 4 | `abs 0x38684` | `-` |
| `ldr` at 0x384a0 | load | 4 | `abs 0x132644` | `-` |
| `ldr` at 0x384ac | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x384b0 | load | 4 | `abs 0x3869c` | `-` |
| `ldr` at 0x384b4 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x384bc | load | 4 | `sp -0x48` | `-` |
| `ldc` at 0x384c0 | load | unknown | `[sp-72] +0x1c` | `-` |
| `stc` at 0x384c8 | store | unknown | `sp -0x54` | `?` |
| `ldr` at 0x384cc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x384d0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x384d4 | load | 4 | `[sp-72] +0x18` | `-` |
| `ldr` at 0x384d8 | load | 4 | `[sp-72] +0x30` | `-` |
| `ldr` at 0x384e8 | load | 4 | `abs 0x386a0` | `-` |
| `str` at 0x384ec | store | 4 | `sp -0x58` | `0xf5a58` |
| `ldr` at 0x384f4 | load | 4 | `abs 0x386a4` | `-` |
| `str` at 0x384f8 | store | 4 | `sp -0x58` | `0xf5a60` |
| `ldr` at 0x384fc | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0x38500 | load | 1 | `[sp-72] +0x29` | `-` |
| `str` at 0x38504 | store | 4 | `(unknown)` | `[[sp-72]+41]:1` |
| `ldr` at 0x38508 | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0x3850c | load | 1 | `[sp-72] +0x26` | `-` |
| `str` at 0x38510 | store | 4 | `(unknown)` | `[[sp-72]+38]:1` |
| `ldr` at 0x38514 | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0x38518 | load | 1 | `[sp-72] +0x28` | `-` |
| `str` at 0x3851c | store | 4 | `(unknown)` | `[[sp-72]+40]:1` |
| `ldr` at 0x38520 | load | 4 | `abs 0x386a8` | `-` |
| `ldm` at 0x38528 | load | 4 | `sp -0x54` | `-` |
| `ldm` at 0x38528 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x3852c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x38534 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x38538 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3853c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x38540 | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0x38544 | load | 1 | `[sp-72] +0x2c` | `-` |
| `str` at 0x38548 | store | 4 | `(unknown)` | `[[sp-72]+44]:1` |
| `ldr` at 0x3854c | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0x38550 | load | 1 | `[sp-72] +0x2d` | `-` |
| `str` at 0x38554 | store | 4 | `(unknown)` | `[[sp-72]+45]:1` |
| `ldr` at 0x38558 | load | 4 | `abs 0x38694` | `-` |
| `ldr` at 0x3855c | load | 4 | `[sp-72] +0x18` | `-` |
| `ldr` at 0x38560 | load | 4 | `[sp-72] +0x14` | `-` |
| `ldr` at 0x38564 | load | 4 | `[sp-72] +0x30` | `-` |
| `ldr` at 0x3856c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38578 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x38584 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x38588 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x3858c | load | 4 | `sp -0x48` | `-` |
| `str` at 0x38594 | store | 4 | `[sp-72] +0xc` | `0x0` |
| `ldr` at 0x38598 | load | 4 | `abs 0x38684` | `-` |
| `ldr` at 0x3859c | load | 4 | `abs 0x132644` | `-` |
| `ldr` at 0x385a8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x385ac | load | 4 | `abs 0x386ac` | `-` |
| `ldr` at 0x385b0 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x385bc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x385c8 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x385d0 | store | 4 | `[sp-72] +0xc` | `0x0` |
| `ldr` at 0x385d4 | load | 4 | `abs 0x38684` | `-` |
| `ldr` at 0x385d8 | load | 4 | `abs 0x132644` | `-` |
| `ldr` at 0x385e4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x385e8 | load | 4 | `abs 0x386b0` | `-` |
| `ldr` at 0x385ec | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x385f8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x38600 | store | 4 | `sp -0x44` | `[sp-68]+1` |
| `ldr` at 0x38604 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3860c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x38614 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x38624 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3862c | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x38634 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3863c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x38648 | load | 4 | `abs 0x38680` | `-` |
| `ldr` at 0x3864c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x38650 | store | 4 | `abs 0x12d6f8` | `[sp-28]` |
| `ldr` at 0x3865c | load | 4 | `sp -0x38` | `-` |
| `str` at 0x38660 | store | 4 | `sp -0x4c` | `[sp-56]` |
| `ldr` at 0x38664 | load | 4 | `sp -0x4c` | `-` |
| `ldm` at 0x3866c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3866c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3866c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x38148 | `(unknown callee) 0x1943c` | `?` | `?` | `?` | `?` |
| `bl` at 0x38180 | `(unknown callee) 0x200a0` | `0x9` | `0xf565c` | `0x608` | `0x0` |
| `bl` at 0x381ac | `(unknown callee) 0x19748` | `?` | `0x0` | `?` | `?` |
| `bl` at 0x381c0 | `(unknown callee) 0x6a7b0` | `0x5` | `?` | `0x12d6f8` | `0x5` |
| `bl` at 0x38298 | `(unknown callee) 0x386b4` | `[sp-68]` | `[sp-20]` | `[sp-40]` | `[sp-28]` |
| `bl` at 0x382e8 | `(unknown callee) 0x221f0` | `0xf5980` | `?` | `?` | `?` |
| `bl` at 0x3833c | `(unknown callee) 0x221f0` | `0xf5994` | `[[sp-68]+16]` | `[[sp-68]+37]:1` | `[[sp-68]+36]:1` |
| `bl` at 0x3837c | `(unknown callee) 0x221f0` | `0xf59b8` | `?` | `?` | `[[sp-68]+41]:1` |
| `bl` at 0x383b4 | `(unknown callee) 0x221f0` | `0xf59f4` | `[[sp-68]+24]` | `[[sp-68]+20]` | `[[sp-68]+48]` |
| `bl` at 0x383ec | `(unknown callee) 0x221f0` | `0xf5a3c` | `[[sp-68]+16]` | `[sp-68]` | `[sp-68]` |
| `bl` at 0x3842c | `(unknown callee) 0x368c4` | `0x6e` | `?` | `?` | `0x0` |
| `bl` at 0x38490 | `(unknown callee) 0x38ee4` | `?` | `sp-64` | `?` | `sp-64` |
| `bl` at 0x384b8 | `(unknown callee) 0x221f0` | `0xf5a4c` | `[[sp-72]+16]` | `?` | `[sp-72]` |
| `bl` at 0x38530 | `(unknown callee) 0x221f0` | `0xf5a68` | `[sp-84]` | `[sp-80]` | `[sp-88]` |
| `bl` at 0x38568 | `(unknown callee) 0x221f0` | `0xf59f4` | `[[sp-72]+24]` | `[[sp-72]+20]` | `[[sp-72]+48]` |
| `bl` at 0x385b4 | `(unknown callee) 0x221f0` | `0xf5aa8` | `[[sp-72]+16]` | `[sp-72]` | `[sp-72]` |
| `bl` at 0x385f0 | `(unknown callee) 0x221f0` | `0xf5abc` | `[[sp-72]+16]` | `[sp-72]` | `[sp-72]` |
| `bl` at 0x38638 | `(unknown callee) 0x1970c` | `[sp-52]` | `?` | `?` | `?` |
| `bl` at 0x38658 | `(unknown callee) 0x6a7b0` | `[sp-28]` | `?` | `0x12d6f8` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x38088 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x380a4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x380d0 | `? == 0xff` | `? != 0xff` |
| `beq` at 0x380dc | `[sp-24] == 0x21` | `[sp-24] != 0x21` |
| `beq` at 0x380ec | `[sp-24] == [sp-40]` | `[sp-24] != [sp-40]` |
| `bne` at 0x38118 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x3815c | `ret(0x1943c) != 0x0` | `ret(0x1943c) == 0x0` |
| `ble` at 0x381e4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x38200 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x3822c | `? == 0xff` | `? != 0xff` |
| `beq` at 0x38238 | `[sp-24] == 0x21` | `[sp-24] != 0x21` |
| `beq` at 0x38248 | `[sp-24] == [sp-40]` | `[sp-24] != [sp-40]` |
| `beq` at 0x382ac | `ret(0x386b4) == 0x0` | `ret(0x386b4) != 0x0` |
| `beq` at 0x382bc | `[0x132644] == 0x0` | `[0x132644] != 0x0` |
| `bne` at 0x38300 | `[sp-56] != 0x0` | `[sp-56] == 0x0` |
| `beq` at 0x3831c | `[0x132644] == 0x0` | `[0x132644] != 0x0` |
| `beq` at 0x383c0 | `[sp-64] == 0x0` | `[sp-64] != 0x0` |
| `beq` at 0x383dc | `[0x132644] == 0x0` | `[0x132644] != 0x0` |
| `bgt` at 0x3841c | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bge` at 0x38444 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x3847c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x384a8 | `[0x132644] == 0x0` | `[0x132644] != 0x0` |
| `ble` at 0x384e4 | `? <= 0x0 (signed, 32-bit)` | `? > 0x0 (signed, 32-bit)` |
| `beq` at 0x38574 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x38580 | `[sp-56] != 0x0` | `[sp-56] == 0x0` |
| `beq` at 0x385a4 | `[0x132644] == 0x0` | `[0x132644] != 0x0` |
| `beq` at 0x385c4 | `[sp-64] == 0x0` | `[sp-64] != 0x0` |
| `beq` at 0x385e0 | `[0x132644] == 0x0` | `[0x132644] != 0x0` |
| `bne` at 0x3861c | `[sp-68] != 0x0` | `[sp-68] == 0x0` |
| `bne` at 0x38644 | `[sp-56] != 0x0` | `[sp-56] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3866c | `r0` | `?` |

## `power_margin_multi_init`

- `power_margin_multi_init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x386b4 to 0x38ee4.
  Established. The word through `[sp+4] +0x0` receives zero (`str` at 0x386dc). The word through
  `[sp-24] +0xc` receives `ret(0x3a044)` (`str` at 0x38784). An included byte assignment through
  `[sp-24] +0x26` writes `(ret(0x39f34) & 0xff)` (`strb` at 0x38b3c). Multi-device margin
  initialization remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x386c4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x386c8 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x386cc | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x386d0 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x386d4 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x386dc | store | 4 | `[sp+4] +0x0` | `0x0` |
| `ldr` at 0x386e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x386f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x386f4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x386f8 | store | 4 | `[sp-24] +0x0` | `[sp-28]` |
| `ldr` at 0x386fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38700 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x38704 | store | 4 | `[sp-24] +0x4` | `[sp-32]` |
| `ldr` at 0x38708 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3870c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x38710 | store | 4 | `[sp-24] +0x8` | `[sp-36]` |
| `ldr` at 0x38714 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38718 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x38720 | store | 4 | `[sp-24] +0x18` | `0x4` |
| `str` at 0x38728 | store | 4 | `[sp-24] +0x14` | `0x4` |
| `ldr` at 0x3872c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38730 | load | 4 | `abs 0x38ebc` | `-` |
| `str` at 0x38734 | store | 4 | `[sp-24] +0x1c` | `0x0` |
| `ldr` at 0x38738 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x38740 | store | 1 | `[sp-24] +0x25` | `0x0` |
| `ldr` at 0x38744 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x3874c | store | 1 | `[sp-24] +0x27` | `0x0` |
| `ldr` at 0x38750 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x38758 | store | 4 | `[sp-24] +0x30` | `0x1` |
| `ldr` at 0x3875c | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x38764 | store | 1 | `[sp-24] +0x2c` | `0x2` |
| `ldr` at 0x38768 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x38770 | store | 1 | `[sp-24] +0x2d` | `0x6` |
| `ldr` at 0x38774 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38778 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3877c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x38784 | store | 4 | `[sp-24] +0xc` | `ret(0x3a044)` |
| `ldr` at 0x38788 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x38794 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x387a0 | store | 4 | `sp -0x40` | `ret(0x3a0e4)` |
| `ldr` at 0x387a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x387b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x387b8 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x387bc | load | 4 | `[[sp-24]+12] +0x8` | `-` |
| `ldr` at 0x387c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x387cc | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x387d0 | load | 4 | `[[sp-24]+12] +0x8` | `-` |
| `ldr` at 0x387dc | load | 4 | `abs 0x38ec0` | `-` |
| `str` at 0x387e0 | store | 4 | `sp -0x40` | `0xfffff9b8` |
| `ldr` at 0x387e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x387ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x387f0 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x387f8 | store | 4 | `sp -0x34` | `ret(0x3a950)` |
| `ldr` at 0x387fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38800 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x38804 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x38810 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38814 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldrb` at 0x38818 | load | 1 | `[[sp-24]+12] +0x6` | `-` |
| `ldr` at 0x38824 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38828 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x3882c | load | 4 | `[[sp-24]+12] +0x8` | `-` |
| `ldrls` at 0x38838 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x38a68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x38a70 | load | 4 | `abs 0x38ec8` | `-` |
| `ldr` at 0x38a74 | load | 4 | `abs 0x38ecc` | `-` |
| `str` at 0x38ab0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x38ab4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x38abc | load | 4 | `abs 0x38ec8` | `-` |
| `ldr` at 0x38ac0 | load | 4 | `abs 0x38ecc` | `-` |
| `ldr` at 0x38acc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x38adc | load | 4 | `abs 0x38ed0` | `-` |
| `str` at 0x38ae0 | store | 4 | `(unknown)` | `0xf5940` |
| `str` at 0x38ae4 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0x38ae8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38aec | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x38af0 | load | 4 | `[[sp-24]+12] +0x8` | `-` |
| `str` at 0x38af4 | store | 4 | `(unknown)` | `[[[sp-24]+12]+8]` |
| `ldr` at 0x38afc | load | 4 | `abs 0x38ed4` | `-` |
| `ldr` at 0x38b00 | load | 4 | `abs 0x38ed8` | `-` |
| `ldr` at 0x38b10 | load | 4 | `abs 0x38edc` | `-` |
| `str` at 0x38b14 | store | 4 | `sp -0x40` | `0xfffff9be` |
| `ldr` at 0x38b1c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38b28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38b2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38b30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x38b34 | load | 4 | `[sp-24] +0xc` | `-` |
| `strb` at 0x38b3c | store | 1 | `[sp-24] +0x26` | `(ret(0x39f34) & 0xff)` |
| `ldr` at 0x38b40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38b44 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x38b48 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x38b54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38b58 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x38b5c | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x38b60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x38b64 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x38b70 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x38b78 | store | 4 | `[sp+4] +0x0` | `0x1` |
| `ldr` at 0x38b7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38b80 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x38b84 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x38b88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x38b8c | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x38b9c | store | 4 | `sp -0x40` | `ret(0x3a430)` |
| `ldr` at 0x38ba4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x38ba8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x38bb4 | store | 4 | `sp -0x2c` | `ret(0x3783c)` |
| `ldr` at 0x38bb8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x38bc4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x38bc8 | load | 4 | `abs 0x38ec0` | `-` |
| `ldr` at 0x38bd4 | load | 4 | `abs 0x38ee0` | `-` |
| `str` at 0x38bd8 | store | 4 | `sp -0x40` | `0xfffff9bd` |
| `ldr` at 0x38be0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38be4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x38be8 | load | 4 | `sp -0x20` | `-` |
| `stc` at 0x38bf0 | store | unknown | `[sp-24] +0x20` | `?` |
| `ldr` at 0x38bf4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x38bf8 | store | 4 | `sp -0x44` | `[sp-36]` |
| `ldr` at 0x38bfc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x38c08 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x38c14 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x38c24 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x38c34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38c38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38c3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x38c40 | load | 4 | `[sp-24] +0xc` | `-` |
| `strb` at 0x38c48 | store | 1 | `[sp-24] +0x26` | `(ret(0x39f08) & 0xff)` |
| `ldr` at 0x38c4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38c50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x38c54 | load | 4 | `sp -0x20` | `-` |
| `stc` at 0x38c5c | store | unknown | `[sp-24] +0x20` | `?` |
| `ldr` at 0x38c64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38c68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38c6c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x38c70 | load | 4 | `[sp-24] +0xc` | `-` |
| `strb` at 0x38c78 | store | 1 | `[sp-24] +0x26` | `(ret(0x39f34) & 0xff)` |
| `ldr` at 0x38c80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38c84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38c88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x38c8c | load | 4 | `[sp-24] +0xc` | `-` |
| `strb` at 0x38c94 | store | 1 | `[sp-24] +0x26` | `(ret(0x39f60) & 0xff)` |
| `ldr` at 0x38c98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38c9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x38ca0 | load | 4 | `sp -0x20` | `-` |
| `stc` at 0x38ca8 | store | unknown | `[sp-24] +0x20` | `?` |
| `ldr` at 0x38cb0 | load | 4 | `abs 0x38ec4` | `-` |
| `str` at 0x38cb4 | store | 4 | `sp -0x40` | `0xfffff9bc` |
| `ldr` at 0x38cbc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x38cc0 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x38ccc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38cd0 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x38cd4 | load | 4 | `[[sp-24]+12] +0x8` | `-` |
| `ldr` at 0x38ce0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38ce4 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x38ce8 | load | 4 | `[[sp-24]+12] +0x8` | `-` |
| `ldr` at 0x38cf8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x38cfc | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x38d00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38d04 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x38d1c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x38d20 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x38d24 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x38d28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38d2c | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x38d44 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x38d48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38d4c | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x38d58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38d5c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x38d60 | load | 1 | `[sp-24] +0x25` | `-` |
| `ldr` at 0x38d64 | load | 4 | `sp -0x38` | `-` |
| `strb` at 0x38d70 | store | 1 | `[sp-24] +0x29` | `?` |
| `ldr` at 0x38d74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38d78 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x38d7c | load | 1 | `[sp-24] +0x24` | `-` |
| `ldr` at 0x38d80 | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0x38d8c | store | 1 | `[sp-24] +0x28` | `?` |
| `ldr` at 0x38d94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38d98 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x38d9c | load | 1 | `[sp-24] +0x25` | `-` |
| `ldr` at 0x38da0 | load | 4 | `sp -0x38` | `-` |
| `strb` at 0x38dac | store | 1 | `[sp-24] +0x29` | `?` |
| `ldr` at 0x38db0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38db4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x38db8 | load | 1 | `[sp-24] +0x24` | `-` |
| `ldr` at 0x38dbc | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0x38dc8 | store | 1 | `[sp-24] +0x28` | `?` |
| `ldr` at 0x38dd0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x38dd4 | load | 1 | `[sp-24] +0x24` | `-` |
| `ldr` at 0x38dd8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x38ddc | load | 1 | `[sp-24] +0x25` | `-` |
| `str` at 0x38de4 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x38de8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38dec | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x38df0 | load | 1 | `[sp-24] +0x25` | `-` |
| `strb` at 0x38df4 | store | 1 | `[sp-24] +0x29` | `([[sp-24]+37]:1 & 0xff)` |
| `ldr` at 0x38df8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38dfc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x38e00 | load | 1 | `[sp-24] +0x24` | `-` |
| `strb` at 0x38e04 | store | 1 | `[sp-24] +0x28` | `([[sp-24]+36]:1 & 0xff)` |
| `ldr` at 0x38e08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38e0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38e10 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x38e14 | load | 1 | `[sp-24] +0x24` | `-` |
| `ldr` at 0x38e18 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x38e1c | load | 1 | `[sp-24] +0x25` | `-` |
| `ldrb` at 0x38e34 | load | 1 | `[sp-24] +0x25` | `-` |
| `strb` at 0x38e3c | store | 1 | `[sp-24] +0x26` | `?` |
| `ldr` at 0x38e40 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x38e48 | store | 1 | `[sp-24] +0x2d` | `0x2` |
| `ldr` at 0x38e4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38e50 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x38e54 | load | 1 | `[sp-24] +0x29` | `-` |
| `strb` at 0x38e58 | store | 1 | `[sp-24] +0x2b` | `([[sp-24]+41]:1 & 0xff)` |
| `ldr` at 0x38e5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38e60 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x38e64 | load | 1 | `[sp-24] +0x28` | `-` |
| `strb` at 0x38e68 | store | 1 | `[sp-24] +0x2a` | `([[sp-24]+40]:1 & 0xff)` |
| `ldr` at 0x38e6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38e70 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x38e74 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x38e78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x38e7c | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x38e8c | store | 4 | `sp -0x2c` | `ret(0x3a430)` |
| `ldr` at 0x38e90 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x38e9c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x38ea0 | store | 4 | `sp -0x40` | `ret(0x3a430)` |
| `str` at 0x38eac | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x38eb0 | load | 4 | `sp -0x40` | `-` |
| `ldm` at 0x38eb8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x38eb8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x38eb8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x38eb8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x386ec | `(unknown callee) 0x19748` | `[sp-24]` | `0x0` | `0x4` | `0x0` |
| `bl` at 0x38780 | `(unknown callee) 0x3a044` | `[sp-28]` | `[sp-32]` | `[sp-24]` | `0x6` |
| `bl` at 0x38798 | `(unknown callee) 0x3a0e4` | `[sp-28]` | `?` | `?` | `[[sp-24]+12]` |
| `bl` at 0x387f4 | `(unknown callee) 0x3a950` | `[sp-28]` | `[[sp-24]+12]` | `?` | `[sp-24]` |
| `bl` at 0x3880c | `(unknown callee) 0x3a98c` | `[sp-28]` | `[[sp-24]+12]` | `0x5` | `[sp-24]` |
| `bl` at 0x38a78 | `(unknown callee) 0x48da8` | `[sp-28]` | `0x0` | `0xf56b4` | `0xf56bc` |
| `bl` at 0x38a84 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x38ac4 | `(unknown callee) 0x48da8` | `[sp-28]` | `0x0` | `0xf56b4` | `0xf56bc` |
| `bl` at 0x38ad4 | `(unknown callee) 0x19808` | `[sp-56]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x38b08 | `(unknown callee) 0x200a0` | `0x9` | `0xf565c` | `0x6fd` | `0x0` |
| `bl` at 0x38b38 | `(unknown callee) 0x39f34` | `[sp-28]` | `[[sp-24]+12]` | `?` | `[sp-24]` |
| `bl` at 0x38b50 | `(unknown callee) 0x3a98c` | `[sp-28]` | `[[sp-24]+12]` | `0x0` | `[sp-24]` |
| `bl` at 0x38b6c | `(unknown callee) 0x3a914` | `[sp-28]` | `[[sp-24]+12]` | `[[sp-24]+38]:1` | `[[sp-24]+38]:1` |
| `bl` at 0x38b94 | `(unknown callee) 0x3a430` | `[sp-28]` | `[[sp-24]+12]` | `[[sp-24]+38]:1` | `[[sp-24]+38]:1` |
| `bl` at 0x38bac | `(unknown callee) 0x3783c` | `[sp-28]` | `[sp-32]` | `?` | `[sp-36]` |
| `bl` at 0x38bec | `(unknown callee) 0x39ddc` | `[sp-28]` | `[sp-32]` | `?` | `?` |
| `bl` at 0x38c44 | `(unknown callee) 0x39f08` | `[sp-28]` | `[[sp-24]+12]` | `?` | `[sp-24]` |
| `bl` at 0x38c58 | `(unknown callee) 0x39d14` | `[sp-28]` | `[sp-32]` | `?` | `?` |
| `bl` at 0x38c74 | `(unknown callee) 0x39f34` | `[sp-28]` | `[[sp-24]+12]` | `?` | `[sp-24]` |
| `bl` at 0x38c90 | `(unknown callee) 0x39f60` | `[sp-28]` | `[[sp-24]+12]` | `?` | `[sp-24]` |
| `bl` at 0x38ca4 | `(unknown callee) 0x39d78` | `[sp-28]` | `[sp-32]` | `?` | `?` |
| `bl` at 0x38e84 | `(unknown callee) 0x3a430` | `[sp-28]` | `[[sp-24]+12]` | `[[sp-24]+38]:1` | `[[sp-24]+38]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x38790 | `[[sp-24]+12] != 0x0` | `[[sp-24]+12] == 0x0` |
| `bne` at 0x387b0 | `[sp-36] != 0x6` | `[sp-36] == 0x6` |
| `beq` at 0x387c4 | `[[[sp-24]+12]+8] == 0x1` | `[[[sp-24]+12]+8] != 0x1` |
| `beq` at 0x387d8 | `[[[sp-24]+12]+8] == 0x2` | `[[[sp-24]+12]+8] != 0x2` |
| `beq` at 0x38820 | `[[[sp-24]+12]+6]:1 == 0xff` | `[[[sp-24]+12]+6]:1 != 0xff` |
| `ldrls` at 0x38838 | `[[[sp-24]+12]+8]-1 <= 0x4 (unsigned, 32-bit)` | `[[[sp-24]+12]+8]-1 > 0x4 (unsigned, 32-bit)` |
| `bne` at 0x38b24 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `beq` at 0x38bc0 | `ret(0x3783c) == 0x1` | `ret(0x3783c) != 0x1` |
| `beq` at 0x38bd0 | `ret(0x3783c) == 0xfffff9b8` | `ret(0x3783c) != 0xfffff9b8` |
| `beq` at 0x38c04 | `[sp-36] == 0x2` | `[sp-36] != 0x2` |
| `bgt` at 0x38c10 | `[sp-36] > 0x2 (signed, 32-bit)` | `[sp-36] <= 0x2 (signed, 32-bit)` |
| `beq` at 0x38c1c | `[sp-36] == 0x1` | `[sp-36] != 0x1` |
| `beq` at 0x38c2c | `[sp-36] == 0x3` | `[sp-36] != 0x3` |
| `beq` at 0x38cc8 | `[[sp-24]+38]:1 == 0x0` | `[[sp-24]+38]:1 != 0x0` |
| `beq` at 0x38cdc | `[[[sp-24]+12]+8] == 0x1` | `[[[sp-24]+12]+8] != 0x1` |
| `beq` at 0x38cf0 | `[[[sp-24]+12]+8] == 0x2` | `[[[sp-24]+12]+8] != 0x2` |
| `ble` at 0x38d54 | `[[sp-24]+48] <= 0x0 (signed, 32-bit)` | `[[sp-24]+48] > 0x0 (signed, 32-bit)` |
| `beq` at 0x38e98 | `ret(0x3a430) == 0x0` | `ret(0x3a430) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x38eb8 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x38838).

## `power_margin_multi`

- `power_margin_multi`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x38ee4 to 0x39830.
  Established. A zero entry `r0` takes `0x38f18`, while nonzero falls through (`beq` at 0x38f04). An
  included word assignment through `[sp-28] +0x0` writes zero (`str` at 0x38f88). The byte through
  `[sp-24] +0x2d` receives `0x2` (`strb` at 0x39778). The unnamed `0x3a430` call carries
  `[[sp-24]]`, `[[sp-24]+12]`, `[[sp-24]+38]:1`, `[[sp-24]+38]:1` (`bl` at 0x39798). Multi-device
  margin effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x38ef4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x38ef8 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x38efc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38f08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38f0c | load | 4 | `r0 +0xc` | `-` |
| `ldr` at 0x38f18 | load | 4 | `abs 0x397f0` | `-` |
| `str` at 0x38f1c | store | 4 | `sp -0x30` | `0xf5acc` |
| `ldr` at 0x38f24 | load | 4 | `abs 0x397f4` | `-` |
| `ldr` at 0x38f28 | load | 4 | `abs 0x397f8` | `-` |
| `ldr` at 0x38f34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38f38 | load | 4 | `(unknown)` | `-` |
| `str` at 0x38f3c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x38f40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38f44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38f48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38f50 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x38f54 | load | 4 | `(unknown)` | `-` |
| `str` at 0x38f64 | store | 4 | `sp -0x20` | `ret(0x398cc)` |
| `ldr` at 0x38f68 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x38f74 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x38f78 | store | 4 | `sp -0x28` | `ret(0x398cc)` |
| `ldr` at 0x38f80 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x38f88 | store | 4 | `[sp-28] +0x0` | `0x0` |
| `ldr` at 0x38f8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38f90 | load | 4 | `sp -0x18` | `-` |
| `ldc` at 0x38f94 | load | unknown | `[sp-24] +0x1c` | `-` |
| `ldc` at 0x38f98 | load | unknown | `[sp-24] +0x20` | `-` |
| `ldr` at 0x38fa8 | load | 4 | `abs 0x397fc` | `-` |
| `ldr` at 0x38fac | load | 4 | `abs 0x132644` | `-` |
| `ldr` at 0x38fb8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38fbc | load | 4 | `abs 0x39800` | `-` |
| `ldr` at 0x38fc0 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x38fc8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x38fd0 | store | 4 | `[sp-28] +0x0` | `0x1` |
| `ldr` at 0x38fd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x38fd8 | load | 4 | `sp -0x18` | `-` |
| `ldc` at 0x38fdc | load | unknown | `[sp-24] +0x1c` | `-` |
| `ldc` at 0x38fe0 | load | unknown | `[sp-24] +0x20` | `-` |
| `ldr` at 0x38ff0 | load | 4 | `sp -0x18` | `-` |
| `ldc` at 0x38ff4 | load | unknown | `[sp-24] +0x20` | `-` |
| `ldc` at 0x38ff8 | load | unknown | `sp -0x24` | `-` |
| `ldr` at 0x39008 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3900c | load | 4 | `[sp-24] +0x18` | `-` |
| `ldr` at 0x39020 | load | 4 | `abs 0x397fc` | `-` |
| `ldr` at 0x39024 | load | 4 | `abs 0x132644` | `-` |
| `ldr` at 0x39030 | load | 4 | `sp -0x18` | `-` |
| `ldc` at 0x39034 | load | unknown | `sp -0x24` | `-` |
| `ldr` at 0x3903c | load | 4 | `abs 0x39804` | `-` |
| `ldr` at 0x39040 | load | 4 | `[sp-24] +0x10` | `-` |
| `stc` at 0x39044 | store | unknown | `sp -0x38` | `?` |
| `ldr` at 0x39050 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39054 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39058 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3905c | load | 4 | `[sp-24] +0x18` | `-` |
| `ldrb` at 0x39060 | load | 1 | `[sp-24] +0x26` | `-` |
| `strb` at 0x39068 | store | 1 | `[sp-24] +0x26` | `?` |
| `ldr` at 0x3906c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x39074 | store | 4 | `[sp-28] +0x0` | `0x1` |
| `ldr` at 0x39078 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3907c | load | 4 | `sp -0x18` | `-` |
| `ldc` at 0x39080 | load | unknown | `[sp-24] +0x1c` | `-` |
| `ldc` at 0x39084 | load | unknown | `[sp-24] +0x20` | `-` |
| `ldr` at 0x39094 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39098 | load | 4 | `[sp-24] +0x18` | `-` |
| `ldr` at 0x390ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x390b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x390b4 | load | 4 | `[sp-24] +0x18` | `-` |
| `ldr` at 0x390b8 | load | 4 | `[sp-24] +0x14` | `-` |
| `ldr` at 0x390c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x390cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x390d0 | load | 4 | `[sp-24] +0x18` | `-` |
| `ldr` at 0x390d4 | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x390e8 | load | 4 | `abs 0x397fc` | `-` |
| `ldr` at 0x390ec | load | 4 | `abs 0x132644` | `-` |
| `ldr` at 0x390f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x390fc | load | 4 | `abs 0x39808` | `-` |
| `ldr` at 0x39100 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x39108 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x39110 | store | 4 | `[sp-28] +0x0` | `0x1` |
| `ldr` at 0x39114 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39118 | load | 4 | `sp -0x18` | `-` |
| `ldc` at 0x3911c | load | unknown | `[sp-24] +0x1c` | `-` |
| `ldc` at 0x39120 | load | unknown | `[sp-24] +0x20` | `-` |
| `ldr` at 0x39130 | load | 4 | `sp -0x18` | `-` |
| `ldc` at 0x39134 | load | unknown | `[sp-24] +0x20` | `-` |
| `ldc` at 0x39138 | load | unknown | `sp -0x24` | `-` |
| `ldr` at 0x39148 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3914c | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x39150 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39154 | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x3915c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x39160 | load | 1 | `[sp-24] +0x28` | `-` |
| `ldr` at 0x3916c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39170 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x39174 | load | 1 | `[sp-24] +0x28` | `-` |
| `ldrb` at 0x39178 | load | 1 | `[sp-24] +0x24` | `-` |
| `ldr` at 0x39184 | load | 4 | `abs 0x397fc` | `-` |
| `ldr` at 0x39188 | load | 4 | `abs 0x132644` | `-` |
| `ldr` at 0x39194 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39198 | load | 4 | `abs 0x3980c` | `-` |
| `ldr` at 0x3919c | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x391a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x391a8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x391ac | load | 1 | `[sp-24] +0x28` | `-` |
| `strb` at 0x391b0 | store | 1 | `[sp-24] +0x26` | `([[sp-24]+40]:1 & 0xff)` |
| `ldr` at 0x391b4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x391bc | store | 4 | `[sp-28] +0x0` | `0x1` |
| `ldr` at 0x391c4 | load | 4 | `sp -0x18` | `-` |
| `ldc` at 0x391c8 | load | unknown | `[sp-24] +0x20` | `-` |
| `ldc` at 0x391cc | load | unknown | `sp -0x24` | `-` |
| `ldr` at 0x391dc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x391e0 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x391e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x391e8 | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x391f0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x391f4 | load | 1 | `[sp-24] +0x29` | `-` |
| `ldr` at 0x39200 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39204 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x39208 | load | 1 | `[sp-24] +0x29` | `-` |
| `ldrb` at 0x3920c | load | 1 | `[sp-24] +0x25` | `-` |
| `ldr` at 0x39218 | load | 4 | `abs 0x397fc` | `-` |
| `ldr` at 0x3921c | load | 4 | `abs 0x132644` | `-` |
| `ldr` at 0x39228 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3922c | load | 4 | `abs 0x39810` | `-` |
| `ldr` at 0x39230 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x39238 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x39240 | store | 4 | `[sp-28] +0x0` | `0x1` |
| `ldr` at 0x39244 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x39248 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x39254 | load | 4 | `abs 0x397fc` | `-` |
| `ldr` at 0x39258 | load | 4 | `abs 0x132644` | `-` |
| `ldr` at 0x39264 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39268 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3926c | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x39270 | load | 4 | `abs 0x39814` | `-` |
| `ldr` at 0x39274 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x3927c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39280 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39284 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x39288 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x3928c | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x39290 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x392a0 | store | 4 | `sp -0x20` | `ret(0x3a430)` |
| `ldr` at 0x392a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x392b0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x392b8 | store | 4 | `[sp-28] +0x0` | `0x0` |
| `ldr` at 0x392bc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x392c0 | store | 4 | `sp -0x28` | `[sp-32]` |
| `ldr` at 0x392c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x392cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x392d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x392d4 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x392d8 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x392dc | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x392e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x392e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x392ec | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x392f0 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x392f4 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x392f8 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x39308 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x39310 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39314 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39318 | load | 4 | `[sp-24] +0x18` | `-` |
| `ldr` at 0x3931c | load | 4 | `[sp-24] +0x14` | `-` |
| `ldr` at 0x3932c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39330 | load | 4 | `[sp-24] +0x18` | `-` |
| `ldr` at 0x39344 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39348 | load | 4 | `[sp-24] +0x14` | `-` |
| `ldr` at 0x3935c | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x39364 | store | 1 | `[sp-24] +0x2c` | `0x1` |
| `ldr` at 0x39368 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3936c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39370 | load | 4 | `[sp-24] +0x18` | `-` |
| `str` at 0x39374 | store | 4 | `[sp-24] +0x14` | `[[sp-24]+24]` |
| `ldr` at 0x39378 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3937c | load | 4 | `sp -0x18` | `-` |
| `ldc` at 0x39380 | load | unknown | `[sp-24] +0x1c` | `-` |
| `ldc` at 0x39384 | load | unknown | `[sp-24] +0x20` | `-` |
| `ldr` at 0x39394 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39398 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3939c | load | 1 | `[sp-24] +0x26` | `-` |
| `ldrb` at 0x393a0 | load | 1 | `[sp-24] +0x24` | `-` |
| `ldr` at 0x393ac | load | 4 | `abs 0x39818` | `-` |
| `str` at 0x393b0 | store | 4 | `sp -0x28` | `0xfffff9ba` |
| `ldr` at 0x393b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x393bc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x393c0 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldrb` at 0x393c4 | load | 1 | `[sp-24] +0x28` | `-` |
| `ldr` at 0x393d0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x393d4 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x393d8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x393dc | load | 1 | `[sp-24] +0x28` | `-` |
| `ldr` at 0x393e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x393e4 | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x393f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x393fc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x39400 | load | 1 | `[sp-24] +0x28` | `-` |
| `ldrb` at 0x39404 | load | 1 | `[sp-24] +0x2a` | `-` |
| `ldr` at 0x39410 | load | 4 | `abs 0x397fc` | `-` |
| `ldr` at 0x39414 | load | 4 | `abs 0x132644` | `-` |
| `ldr` at 0x39420 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39424 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x39428 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x3942c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x39430 | load | 1 | `[sp-24] +0x28` | `-` |
| `ldr` at 0x39434 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39438 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x3943c | store | 4 | `(unknown)` | `[[sp-24]+48]` |
| `ldr` at 0x39440 | load | 4 | `abs 0x3981c` | `-` |
| `ldr` at 0x39444 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x39454 | load | 4 | `abs 0x39820` | `-` |
| `str` at 0x39458 | store | 4 | `sp -0x28` | `0xfffff9b7` |
| `ldr` at 0x39460 | load | 4 | `abs 0x397fc` | `-` |
| `ldr` at 0x39464 | load | 4 | `abs 0x132644` | `-` |
| `ldr` at 0x39470 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39474 | load | 4 | `abs 0x39824` | `-` |
| `ldr` at 0x39478 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x39480 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39484 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39488 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3948c | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x39490 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x39494 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x3949c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x394a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x394a4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x394a8 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x394ac | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x394b0 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x394bc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x394c4 | store | 4 | `[sp-28] +0x0` | `0x1` |
| `str` at 0x394cc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x394d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x394d8 | load | 4 | `[sp-24] +0x14` | `-` |
| `ldr` at 0x394ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x394f0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x394f4 | load | 1 | `[sp-24] +0x26` | `-` |
| `strb` at 0x394f8 | store | 1 | `[sp-24] +0x29` | `([[sp-24]+38]:1 & 0xff)` |
| `ldr` at 0x394fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39500 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x39504 | load | 1 | `[sp-24] +0x28` | `-` |
| `ldr` at 0x39508 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3950c | load | 1 | `[sp-24] +0x29` | `-` |
| `ldr` at 0x39524 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x39528 | load | 1 | `[sp-24] +0x2d` | `-` |
| `str` at 0x39544 | store | 4 | `[sp-24] +0x18` | `?` |
| `ldr` at 0x3954c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39550 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x39554 | load | 1 | `[sp-24] +0x2c` | `-` |
| `str` at 0x39558 | store | 4 | `[sp-24] +0x18` | `[[sp-24]+44]:1` |
| `ldr` at 0x39560 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39564 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x39568 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldrb` at 0x3956c | load | 1 | `[sp-24] +0x25` | `-` |
| `ldr` at 0x39578 | load | 4 | `abs 0x39818` | `-` |
| `str` at 0x3957c | store | 4 | `sp -0x28` | `0xfffff9ba` |
| `ldr` at 0x39584 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39588 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3958c | load | 1 | `[sp-24] +0x26` | `-` |
| `ldrb` at 0x39590 | load | 1 | `[sp-24] +0x29` | `-` |
| `ldr` at 0x3959c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x395a0 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x395a4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x395a8 | load | 1 | `[sp-24] +0x29` | `-` |
| `ldr` at 0x395ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x395b0 | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x395c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x395c8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x395cc | load | 1 | `[sp-24] +0x29` | `-` |
| `ldrb` at 0x395d0 | load | 1 | `[sp-24] +0x2b` | `-` |
| `ldr` at 0x395dc | load | 4 | `abs 0x397fc` | `-` |
| `ldr` at 0x395e0 | load | 4 | `abs 0x132644` | `-` |
| `ldr` at 0x395ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x395f0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x395f4 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x395f8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x395fc | load | 1 | `[sp-24] +0x29` | `-` |
| `ldr` at 0x39600 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39604 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x39608 | store | 4 | `(unknown)` | `[[sp-24]+48]` |
| `ldr` at 0x3960c | load | 4 | `abs 0x39828` | `-` |
| `ldr` at 0x39610 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x39620 | load | 4 | `abs 0x39820` | `-` |
| `str` at 0x39624 | store | 4 | `sp -0x28` | `0xfffff9b7` |
| `ldr` at 0x3962c | load | 4 | `abs 0x397fc` | `-` |
| `ldr` at 0x39630 | load | 4 | `abs 0x132644` | `-` |
| `ldr` at 0x3963c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39640 | load | 4 | `abs 0x39824` | `-` |
| `ldr` at 0x39644 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x3964c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39650 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39654 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39658 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x3965c | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x39660 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x39668 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3966c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39670 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x39674 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x39678 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x3967c | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x39688 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x39690 | store | 4 | `[sp-28] +0x0` | `0x1` |
| `str` at 0x39698 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x396a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x396a4 | load | 4 | `[sp-24] +0x14` | `-` |
| `ldr` at 0x396b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x396bc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x396c0 | load | 1 | `[sp-24] +0x26` | `-` |
| `strb` at 0x396c4 | store | 1 | `[sp-24] +0x28` | `([[sp-24]+38]:1 & 0xff)` |
| `ldr` at 0x396c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x396cc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x396d0 | load | 1 | `[sp-24] +0x28` | `-` |
| `ldr` at 0x396d4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x396d8 | load | 1 | `[sp-24] +0x29` | `-` |
| `str` at 0x396ec | store | 4 | `sp -0x2c` | `ret(0x1979c)` |
| `ldr` at 0x396f0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x396f4 | load | 1 | `[sp-24] +0x2d` | `-` |
| `ldr` at 0x396f8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x39710 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x39714 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3971c | store | 4 | `[sp-24] +0x18` | `?` |
| `ldr` at 0x39724 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39728 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3972c | load | 1 | `[sp-24] +0x2c` | `-` |
| `str` at 0x39734 | store | 4 | `[sp-24] +0x18` | `?` |
| `ldr` at 0x39738 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3973c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39740 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39744 | load | 4 | `[sp-24] +0x18` | `-` |
| `ldr` at 0x39748 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x39750 | store | 4 | `[sp-24] +0x18` | `?` |
| `ldr` at 0x39754 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39758 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3975c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39760 | load | 4 | `[sp-24] +0x18` | `-` |
| `ldrb` at 0x39764 | load | 1 | `[sp-24] +0x26` | `-` |
| `strb` at 0x3976c | store | 1 | `[sp-24] +0x26` | `(([[sp-24]+38]:1 + [[sp-24]+24]) & 0xff)` |
| `ldr` at 0x39770 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x39778 | store | 1 | `[sp-24] +0x2d` | `0x2` |
| `ldr` at 0x3977c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39780 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39784 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x39788 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x3978c | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x39790 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x397a0 | store | 4 | `sp -0x20` | `ret(0x3a430)` |
| `ldr` at 0x397a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x397b0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x397b4 | store | 4 | `sp -0x28` | `ret(0x3a430)` |
| `ldr` at 0x397bc | load | 4 | `abs 0x397fc` | `-` |
| `ldr` at 0x397c0 | load | 4 | `abs 0x132644` | `-` |
| `ldr` at 0x397cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x397d0 | load | 4 | `abs 0x3982c` | `-` |
| `ldr` at 0x397d4 | load | 4 | `[sp-24] +0x10` | `-` |
| `str` at 0x397e0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x397e4 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x397ec | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x397ec | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x397ec | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x397ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x38f30 | `(unknown callee) 0x200a0` | `0x9` | `0xf565c` | `0x763` | `0x0` |
| `bl` at 0x38f5c | `(unknown callee) 0x398cc` | `?` | `?` | `0x0` | `?` |
| `bl` at 0x38fc4 | `(unknown callee) 0x221f0` | `0xf5b08` | `[[sp-24]+16]` | `[sp-24]` | `[sp-24]` |
| `bl` at 0x39010 | `(unknown callee) 0x1979c` | `[[sp-24]+24]` | `?` | `[sp-24]` | `[sp-24]` |
| `bl` at 0x3904c | `(unknown callee) 0x221f0` | `0xf5b28` | `[[sp-24]+16]` | `?` | `?` |
| `bl` at 0x3909c | `(unknown callee) 0x1979c` | `[[sp-24]+24]` | `?` | `[sp-24]` | `[sp-24]` |
| `bl` at 0x39104 | `(unknown callee) 0x221f0` | `0xf5b60` | `[[sp-24]+16]` | `[sp-24]` | `[sp-24]` |
| `bl` at 0x391a0 | `(unknown callee) 0x221f0` | `0xf5b98` | `[[sp-24]+16]` | `[sp-24]` | `[sp-24]` |
| `bl` at 0x39234 | `(unknown callee) 0x221f0` | `0xf5bc4` | `[[sp-24]+16]` | `[sp-24]` | `[sp-24]` |
| `bl` at 0x39278 | `(unknown callee) 0x221f0` | `0xf5c00` | `[[sp-24]+16]` | `[[sp-24]+38]:1` | `[sp-24]` |
| `bl` at 0x39298 | `(unknown callee) 0x3a430` | `[[sp-24]]` | `[[sp-24]+12]` | `[[sp-24]+38]:1` | `[[sp-24]+38]:1` |
| `bl` at 0x392e0 | `(unknown callee) 0x3a98c` | `[[sp-24]]` | `[[sp-24]+12]` | `[[sp-24]+8]` | `[sp-24]` |
| `bl` at 0x39300 | `(unknown callee) 0x3a914` | `[[sp-24]]` | `[[sp-24]+12]` | `[[sp-24]+38]:1` | `[[sp-24]+38]:1` |
| `bl` at 0x39334 | `(unknown callee) 0x1979c` | `[[sp-24]+24]` | `[[sp-24]+24]` | `[sp-24]` | `[sp-24]` |
| `bl` at 0x3934c | `(unknown callee) 0x1979c` | `[[sp-24]+20]` | `?` | `?` | `[sp-24]` |
| `bl` at 0x39450 | `(unknown callee) 0x221f0` | `0xf5c20` | `[[sp-24]+16]` | `[[sp-24]+38]:1` | `[[sp-24]+40]:1` |
| `bl` at 0x3947c | `(unknown callee) 0x221f0` | `0xf5c50` | `[[sp-24]+16]` | `[sp-24]` | `[sp-24]` |
| `bl` at 0x39498 | `(unknown callee) 0x3a98c` | `[[sp-24]]` | `[[sp-24]+12]` | `[[sp-24]+8]` | `[sp-24]` |
| `bl` at 0x394b8 | `(unknown callee) 0x3a914` | `[[sp-24]]` | `[[sp-24]+12]` | `[[sp-24]+38]:1` | `[[sp-24]+38]:1` |
| `bl` at 0x394dc | `(unknown callee) 0x1979c` | `[[sp-24]+20]` | `[[sp-24]+38]:1` | `[[sp-24]+40]:1` | `[sp-24]` |
| `bl` at 0x39518 | `(unknown callee) 0x1979c` | `?` | `?` | `[[sp-24]+40]:1` | `?` |
| `bl` at 0x39534 | `(unknown callee) 0xeb174` | `ret(0x1979c)` | `[[sp-24]+45]:1` | `ret(0x1979c)` | `[[sp-24]+45]:1` |
| `bl` at 0x3961c | `(unknown callee) 0x221f0` | `0xf5c78` | `[[sp-24]+16]` | `[[sp-24]+38]:1` | `[[sp-24]+41]:1` |
| `bl` at 0x39648 | `(unknown callee) 0x221f0` | `0xf5c50` | `[[sp-24]+16]` | `[sp-24]` | `[sp-24]` |
| `bl` at 0x39664 | `(unknown callee) 0x3a98c` | `[[sp-24]]` | `[[sp-24]+12]` | `[[sp-24]+8]` | `[sp-24]` |
| `bl` at 0x39684 | `(unknown callee) 0x3a914` | `[[sp-24]]` | `[[sp-24]+12]` | `[[sp-24]+38]:1` | `[[sp-24]+38]:1` |
| `bl` at 0x396a8 | `(unknown callee) 0x1979c` | `[[sp-24]+20]` | `[[sp-24]+38]:1` | `[[sp-24]+41]:1` | `[sp-24]` |
| `bl` at 0x396e4 | `(unknown callee) 0x1979c` | `?` | `?` | `[[sp-24]+40]:1` | `?` |
| `bl` at 0x39700 | `(unknown callee) 0xeb174` | `ret(0x1979c)` | `[[sp-24]+45]:1` | `?` | `[[sp-24]+45]:1` |
| `bl` at 0x39798 | `(unknown callee) 0x3a430` | `[[sp-24]]` | `[[sp-24]+12]` | `[[sp-24]+38]:1` | `[[sp-24]+38]:1` |
| `bl` at 0x397d8 | `(unknown callee) 0x221f0` | `0xf5ca8` | `[[sp-24]+16]` | `?` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x38f04 | `r0 == 0x0` | `r0 != 0x0` |
| `bne` at 0x38f14 | `[r0+12] != 0x0` | `[r0+12] == 0x0` |
| `beq` at 0x38f70 | `ret(0x398cc) == 0x0` | `ret(0x398cc) != 0x0` |
| `beq` at 0x38fa0 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x38fb4 | `[0x132644] == 0x0` | `[0x132644] != 0x0` |
| `bmi` at 0x38fe8 | `N == 1` | `not (N == 1)` |
| `bls` at 0x39000 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x3901c | `ret(0x1979c) != 0x1` | `ret(0x1979c) == 0x1` |
| `beq` at 0x3902c | `[0x132644] == 0x0` | `[0x132644] != 0x0` |
| `bge` at 0x3908c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x390a8 | `ret(0x1979c) != 0x1` | `ret(0x1979c) == 0x1` |
| `bne` at 0x390c4 | `([[sp-24]+24] + [[sp-24]+20]) != 0x0` | `([[sp-24]+24] + [[sp-24]+20]) == 0x0` |
| `bgt` at 0x390e0 | `? > 0x0 (signed, 32-bit)` | `? <= 0x0 (signed, 32-bit)` |
| `beq` at 0x390f4 | `[0x132644] == 0x0` | `[0x132644] != 0x0` |
| `bmi` at 0x39128 | `N == 1` | `not (N == 1)` |
| `bgt` at 0x39140 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0x39168 | `([[sp-24]+38]:1 + [[sp-24]+48]) != [[sp-24]+40]:1` | `([[sp-24]+38]:1 + [[sp-24]+48]) == [[sp-24]+40]:1` |
| `beq` at 0x39180 | `[[sp-24]+40]:1 == [[sp-24]+36]:1` | `[[sp-24]+40]:1 != [[sp-24]+36]:1` |
| `beq` at 0x39190 | `[0x132644] == 0x0` | `[0x132644] != 0x0` |
| `bmi` at 0x391d4 | `N == 1` | `not (N == 1)` |
| `bne` at 0x391fc | `? != [[sp-24]+41]:1` | `? == [[sp-24]+41]:1` |
| `beq` at 0x39214 | `[[sp-24]+41]:1 == [[sp-24]+37]:1` | `[[sp-24]+41]:1 != [[sp-24]+37]:1` |
| `beq` at 0x39224 | `[0x132644] == 0x0` | `[0x132644] != 0x0` |
| `beq` at 0x39250 | `[[sp-28]] == 0x0` | `[[sp-28]] != 0x0` |
| `beq` at 0x39260 | `[0x132644] == 0x0` | `[0x132644] != 0x0` |
| `beq` at 0x392ac | `ret(0x3a430) == 0x0` | `ret(0x3a430) != 0x0` |
| `bge` at 0x39328 | `? >= 0x0 (signed, 32-bit)` | `? < 0x0 (signed, 32-bit)` |
| `bne` at 0x39340 | `ret(0x1979c) != 0x2` | `ret(0x1979c) == 0x2` |
| `bne` at 0x39358 | `ret(0x1979c) != 0x2` | `ret(0x1979c) == 0x2` |
| `bmi` at 0x3938c | `N == 1` | `not (N == 1)` |
| `bne` at 0x393a8 | `[[sp-24]+38]:1 != [[sp-24]+36]:1` | `[[sp-24]+38]:1 == [[sp-24]+36]:1` |
| `beq` at 0x393cc | `[[sp-24]+38]:1 == [[sp-24]+40]:1` | `[[sp-24]+38]:1 != [[sp-24]+40]:1` |
| `beq` at 0x393f0 | `[[sp-24]+38]:1 == ([[sp-24]+40]:1 + [[sp-24]+48])` | `[[sp-24]+38]:1 != ([[sp-24]+40]:1 + [[sp-24]+48])` |
| `bne` at 0x3940c | `[[sp-24]+40]:1 != [[sp-24]+42]:1` | `[[sp-24]+40]:1 == [[sp-24]+42]:1` |
| `beq` at 0x3941c | `[0x132644] == 0x0` | `[0x132644] != 0x0` |
| `beq` at 0x3946c | `[0x132644] == 0x0` | `[0x132644] != 0x0` |
| `ble` at 0x394e8 | `ret(0x1979c) <= 0x3 (signed, 32-bit)` | `ret(0x1979c) > 0x3 (signed, 32-bit)` |
| `bne` at 0x39574 | `[[sp-24]+38]:1 != [[sp-24]+37]:1` | `[[sp-24]+38]:1 == [[sp-24]+37]:1` |
| `beq` at 0x39598 | `[[sp-24]+38]:1 == [[sp-24]+41]:1` | `[[sp-24]+38]:1 != [[sp-24]+41]:1` |
| `beq` at 0x395bc | `[[sp-24]+38]:1 == ?` | `[[sp-24]+38]:1 != ?` |
| `bne` at 0x395d8 | `[[sp-24]+41]:1 != [[sp-24]+43]:1` | `[[sp-24]+41]:1 == [[sp-24]+43]:1` |
| `beq` at 0x395e8 | `[0x132644] == 0x0` | `[0x132644] != 0x0` |
| `beq` at 0x39638 | `[0x132644] == 0x0` | `[0x132644] != 0x0` |
| `ble` at 0x396b4 | `ret(0x1979c) <= 0x3 (signed, 32-bit)` | `ret(0x1979c) > 0x3 (signed, 32-bit)` |
| `beq` at 0x397ac | `ret(0x3a430) == 0x0` | `ret(0x3a430) != 0x0` |
| `beq` at 0x397c8 | `[0x132644] == 0x0` | `[0x132644] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x397ec | `r0` | `?` |

## `monitorInit`

- `monitorInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbf638 to 0xbf88c.
  Established. The absolute word at `0x297324` receives `0x1` (`str` at 0xbf6bc). The word at
  `0x297320` receives `ret(0x196d0)` (`str` at 0xbf6e4). The unnamed `0x196d0` call at `0xbf758`
  carries `[[sp-36]+8]`, `0x8`, `[sp-36]`, `[sp-36]` (`bl` at 0xbf758). Sensor table initialization
  meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbf648 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xbf64c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xbf650 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0xbf654 | load | 4 | `abs 0xbf860` | `-` |
| `ldr` at 0xbf658 | load | 4 | `abs 0x297320` | `-` |
| `ldr` at 0xbf664 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbf670 | load | 4 | `abs 0xbf864` | `-` |
| `str` at 0xbf674 | store | 4 | `sp -0x38` | `0x11fabc` |
| `ldr` at 0xbf678 | load | 4 | `abs 0xbf860` | `-` |
| `ldr` at 0xbf67c | load | 4 | `abs 0x297320` | `-` |
| `str` at 0xbf680 | store | 4 | `sp -0x34` | `[0x297320]` |
| `ldr` at 0xbf684 | load | 4 | `abs 0xbf868` | `-` |
| `ldr` at 0xbf688 | load | 4 | `abs 0x297324` | `-` |
| `str` at 0xbf68c | store | 4 | `sp -0x30` | `[0x297324]` |
| `ldr` at 0xbf694 | load | 4 | `abs 0xbf86c` | `-` |
| `ldr` at 0xbf698 | load | 4 | `abs 0xbf870` | `-` |
| `ldr` at 0xbf6a4 | load | 4 | `abs 0xbf868` | `-` |
| `ldr` at 0xbf6a8 | load | 4 | `abs 0x297324` | `-` |
| `ldr` at 0xbf6b4 | load | 4 | `abs 0xbf868` | `-` |
| `str` at 0xbf6bc | store | 4 | `abs 0x297324` | `0x1` |
| `ldr` at 0xbf6c0 | load | 4 | `abs 0xbf868` | `-` |
| `ldr` at 0xbf6c4 | load | 4 | `abs 0x297324` | `-` |
| `str` at 0xbf6cc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbf6d0 | load | 4 | `abs 0xbf860` | `-` |
| `ldr` at 0xbf6d4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf6e4 | store | 4 | `abs 0x297320` | `ret(0x196d0)` |
| `ldr` at 0xbf6e8 | load | 4 | `abs 0x297320` | `-` |
| `ldr` at 0xbf6f4 | load | 4 | `abs 0xbf874` | `-` |
| `str` at 0xbf6f8 | store | 4 | `sp -0x38` | `0x11fb08` |
| `ldr` at 0xbf6fc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf700 | store | 4 | `sp -0x34` | `[sp-44]` |
| `ldr` at 0xbf708 | load | 4 | `abs 0xbf86c` | `-` |
| `ldr` at 0xbf70c | load | 4 | `abs 0xbf878` | `-` |
| `ldr` at 0xbf718 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbf720 | load | 4 | `abs 0xbf860` | `-` |
| `ldr` at 0xbf724 | load | 4 | `abs 0x297320` | `-` |
| `str` at 0xbf72c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xbf730 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbf734 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbf738 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0xbf73c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbf740 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbf744 | store | 4 | `[sp-36] +0x8` | `[sp-32]` |
| `ldr` at 0xbf748 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbf74c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbf750 | load | 4 | `[sp-36] +0x8` | `-` |
| `str` at 0xbf760 | store | 4 | `[sp-36] +0x4` | `ret(0x196d0)` |
| `ldr` at 0xbf764 | load | 4 | `[sp-36] +0x4` | `-` |
| `ldr` at 0xbf770 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbf778 | load | 4 | `abs 0xbf87c` | `-` |
| `ldr` at 0xbf77c | load | 4 | `abs 0xbf880` | `-` |
| `str` at 0xbf7b8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbf7bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbf7c4 | load | 4 | `abs 0xbf87c` | `-` |
| `ldr` at 0xbf7c8 | load | 4 | `abs 0xbf880` | `-` |
| `ldr` at 0xbf7d4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbf7e4 | load | 4 | `abs 0xbf884` | `-` |
| `str` at 0xbf7e8 | store | 4 | `(unknown)` | `0x11fb50` |
| `str` at 0xbf7ec | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0xbf7f0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbf7f4 | load | 4 | `[sp-36] +0x8` | `-` |
| `str` at 0xbf7f8 | store | 4 | `(unknown)` | `[[sp-36]+8]` |
| `ldr` at 0xbf800 | load | 4 | `abs 0xbf86c` | `-` |
| `ldr` at 0xbf804 | load | 4 | `abs 0xbf888` | `-` |
| `str` at 0xbf814 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xbf818 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbf81c | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0xbf820 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbf82c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbf830 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbf838 | load | 4 | `[sp-36] +0x4` | `-` |
| `strb` at 0xbf844 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0xbf848 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xbf850 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldm` at 0xbf85c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xbf85c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbf85c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbf85c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbf6a0 | `(unknown callee) 0x200a0` | `0x9` | `0x11faec` | `0x10f` | `0x0` |
| `bl` at 0xbf6dc | `(unknown callee) 0x196d0` | `[sp-44]` | `0x1` | `0x297324` | `?` |
| `bl` at 0xbf714 | `(unknown callee) 0x200a0` | `0x9` | `0x11faec` | `0x11b` | `0x0` |
| `bl` at 0xbf758 | `(unknown callee) 0x196d0` | `[[sp-36]+8]` | `0x8` | `[sp-36]` | `[sp-36]` |
| `bl` at 0xbf780 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x11fb44` | `0x11fb4c` |
| `bl` at 0xbf78c | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xbf7cc | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x11fb44` | `0x11fb4c` |
| `bl` at 0xbf7dc | `(unknown callee) 0x19808` | `[sp-44]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xbf80c | `(unknown callee) 0x200a0` | `0x9` | `0x11faec` | `0x126` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xbf660 | `[0x297320] == 0x0` | `[0x297320] != 0x0` |
| `bne` at 0xbf66c | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xbf6b0 | `[0x297324] != 0x0` | `[0x297324] == 0x0` |
| `bne` at 0xbf6f0 | `[0x297320] != 0x0` | `[0x297320] == 0x0` |
| `bne` at 0xbf76c | `[[sp-36]+4] != 0x0` | `[[sp-36]+4] == 0x0` |
| `ble` at 0xbf828 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbf85c | `r0` | `?` |

## `monitorNotPresent`

- `monitorNotPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbf8d8 to 0xbf9f8.
  Established. The signed condition `(r1 & 0xff) >= [[sp-28]+8]` takes `0xbf940`, while smaller
  values fall through (`bge` at 0xbf91c). An included byte assignment at an unknown target writes
  zero (`strb` at 0xbf938). The absolute word at `0x297320` is read (`ldr` at 0xbf900). Monitor
  removal effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbf8e8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xbf8f0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xbf8f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbf8fc | load | 4 | `abs 0xbf9e4` | `-` |
| `ldr` at 0xbf900 | load | 4 | `abs 0x297320` | `-` |
| `str` at 0xbf908 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xbf90c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xbf910 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf914 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xbf920 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbf924 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xbf92c | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0xbf938 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xbf940 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbf948 | load | 4 | `abs 0xbf9e8` | `-` |
| `ldr` at 0xbf94c | load | 4 | `abs 0xbf9ec` | `-` |
| `str` at 0xbf988 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xbf98c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbf994 | load | 4 | `abs 0xbf9e8` | `-` |
| `ldr` at 0xbf998 | load | 4 | `abs 0xbf9ec` | `-` |
| `ldr` at 0xbf9a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf9b4 | load | 4 | `abs 0xbf9f0` | `-` |
| `str` at 0xbf9b8 | store | 4 | `(unknown)` | `0x11fb88` |
| `str` at 0xbf9bc | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldrb` at 0xbf9c0 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xbf9c4 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0xbf9cc | load | 4 | `abs 0xbf9f4` | `-` |
| `ldm` at 0xbf9e0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbf9e0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbf9e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbf950 | `(unknown callee) 0x48da8` | `r0` | `0x0` | `0x11fb44` | `0x11fb4c` |
| `bl` at 0xbf95c | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xbf99c | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0x11fb44` | `0x11fb4c` |
| `bl` at 0xbf9ac | `(unknown callee) 0x19808` | `[sp-32]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xbf9d8 | `(unknown callee) 0x200a0` | `0x6` | `0x11faec` | `0x140` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xbf91c | `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbf9e0 | `r0` | `?` |

## `monitorPresent`

- `monitorPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbf9f8 to 0xbfb1c.
  Established. The signed condition `(r1 & 0xff) >= [[sp-28]+8]` takes `0xbfa60`, while smaller
  values fall through (`bge` at 0xbfa3c). A byte at an unknown target receives `0x1` on an included
  path (`strb` at 0xbfa58). The unnamed `0x48da8` call at `0xbfa70` carries entry `r0`, `0x0`,
  `0x11fb44`, `0x11fb4c` (`bl` at 0xbfa70). Monitor presence policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbfa08 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xbfa10 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xbfa14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbfa1c | load | 4 | `abs 0xbfb04` | `-` |
| `ldr` at 0xbfa20 | load | 4 | `abs 0x297320` | `-` |
| `str` at 0xbfa28 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xbfa2c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xbfa30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbfa34 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xbfa40 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbfa44 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xbfa4c | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0xbfa58 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0xbfa60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbfa68 | load | 4 | `abs 0xbfb08` | `-` |
| `ldr` at 0xbfa6c | load | 4 | `abs 0xbfb0c` | `-` |
| `str` at 0xbfaa8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xbfaac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbfab4 | load | 4 | `abs 0xbfb08` | `-` |
| `ldr` at 0xbfab8 | load | 4 | `abs 0xbfb0c` | `-` |
| `ldr` at 0xbfac4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbfad4 | load | 4 | `abs 0xbfb10` | `-` |
| `str` at 0xbfad8 | store | 4 | `(unknown)` | `0x11fbb0` |
| `str` at 0xbfadc | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldrb` at 0xbfae0 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xbfae4 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0xbfaec | load | 4 | `abs 0xbfb14` | `-` |
| `ldr` at 0xbfaf0 | load | 4 | `abs 0xbfb18` | `-` |
| `ldm` at 0xbfb00 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbfb00 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbfb00 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbfa70 | `(unknown callee) 0x48da8` | `r0` | `0x0` | `0x11fb44` | `0x11fb4c` |
| `bl` at 0xbfa7c | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xbfabc | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0x11fb44` | `0x11fb4c` |
| `bl` at 0xbfacc | `(unknown callee) 0x19808` | `[sp-32]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xbfaf8 | `(unknown callee) 0x200a0` | `0x6` | `0x11faec` | `0x14f` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xbfa3c | `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbfb00 | `r0` | `?` |

## `monitorReset`

- `monitorReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbfb64 to 0xbfe74.
  Established. The absolute word at `0x297320` is read (`ldr` at 0xbfb84). An included byte
  assignment through `[sp-36] +0x0` writes zero (`strb` at 0xbfc10). The unnamed `0xc1970` call
  carries `[sp-24]`, `[sp-37]:1`, unknown, `[sp-37]:1` (`bl` at 0xbfbd0). Monitor reset sequencing
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbfb74 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xbfb78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbfb80 | load | 4 | `abs 0xbfe54` | `-` |
| `ldr` at 0xbfb84 | load | 4 | `abs 0x297320` | `-` |
| `str` at 0xbfb8c | store | 4 | `sp -0x1c` | `?` |
| `strb` at 0xbfb94 | store | 1 | `sp -0x25` | `0x0` |
| `ldrb` at 0xbfb98 | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0xbfb9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbfba0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0xbfbac | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0xbfbb0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbfbc0 | store | 4 | `sp -0x20` | `ret(0xc183c)` |
| `ldrb` at 0xbfbc4 | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0xbfbc8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbfbd8 | store | 4 | `sp -0x24` | `ret(0xc1970)` |
| `ldr` at 0xbfbdc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xbfbe0 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xbfbec | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbfbf0 | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0xbfbf8 | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0xbfc04 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xbfc08 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xbfc10 | store | 1 | `[sp-36] +0x0` | `0x0` |
| `ldr` at 0xbfc14 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xbfc18 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbfc24 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xbfc28 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldrls` at 0xbfc34 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbfce0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbfce8 | load | 4 | `abs 0xbfe58` | `-` |
| `ldr` at 0xbfcec | load | 4 | `abs 0xbfe5c` | `-` |
| `str` at 0xbfd28 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xbfd2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbfd34 | load | 4 | `abs 0xbfe58` | `-` |
| `ldr` at 0xbfd38 | load | 4 | `abs 0xbfe5c` | `-` |
| `ldr` at 0xbfd44 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbfd54 | load | 4 | `abs 0xbfe60` | `-` |
| `str` at 0xbfd58 | store | 4 | `(unknown)` | `0x11fbd8` |
| `str` at 0xbfd5c | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0xbfd60 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xbfd64 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0xbfd68 | store | 4 | `(unknown)` | `[[sp-32]]:1` |
| `ldr` at 0xbfd70 | load | 4 | `abs 0xbfe64` | `-` |
| `ldr` at 0xbfd74 | load | 4 | `abs 0xbfe68` | `-` |
| `ldr` at 0xbfd80 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbfd8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbfd94 | load | 4 | `abs 0xbfe58` | `-` |
| `ldr` at 0xbfd98 | load | 4 | `abs 0xbfe5c` | `-` |
| `str` at 0xbfdd4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xbfdd8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbfde0 | load | 4 | `abs 0xbfe58` | `-` |
| `ldr` at 0xbfde4 | load | 4 | `abs 0xbfe5c` | `-` |
| `ldr` at 0xbfdf0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbfe00 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbfe0c | load | 4 | `abs 0xbfe6c` | `-` |
| `str` at 0xbfe10 | store | 4 | `(unknown)` | `0x11fbf0` |
| `str` at 0xbfe14 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0xbfe18 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbfe1c | load | 4 | `[sp-32] +0x8` | `-` |
| `str` at 0xbfe20 | store | 4 | `(unknown)` | `[[sp-32]+8]` |
| `str` at 0xbfe24 | store | 4 | `(unknown)` | `ret(0x1fc1c)` |
| `ldr` at 0xbfe2c | load | 4 | `abs 0xbfe64` | `-` |
| `ldr` at 0xbfe30 | load | 4 | `abs 0xbfe70` | `-` |
| `ldrb` at 0xbfe3c | load | 1 | `sp -0x25` | `-` |
| `strb` at 0xbfe44 | store | 1 | `sp -0x25` | `([sp-37]:1+1 & 0xff)` |
| `ldm` at 0xbfe50 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xbfe50 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbfe50 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbfe50 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbfbb8 | `(unknown callee) 0xc183c` | `?` | `?` | `[[sp-28]+8]` | `?` |
| `bl` at 0xbfbd0 | `(unknown callee) 0xc1970` | `[sp-24]` | `[sp-37]:1` | `?` | `[sp-37]:1` |
| `bl` at 0xbfcf0 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x11fb44` | `0x11fb4c` |
| `bl` at 0xbfcfc | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xbfd3c | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x11fb44` | `0x11fb4c` |
| `bl` at 0xbfd4c | `(unknown callee) 0x19808` | `[sp-48]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xbfd7c | `(unknown callee) 0x200a0` | `0x9` | `0x11faec` | `0x195` | `0x0` |
| `bl` at 0xbfd9c | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x11fb44` | `0x11fb4c` |
| `bl` at 0xbfda8 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xbfde8 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x11fb44` | `0x11fb4c` |
| `bl` at 0xbfdf8 | `(unknown callee) 0x19808` | `[sp-48]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xbfe04 | `(unknown callee) 0x1fc1c` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xbfe38 | `(unknown callee) 0x200a0` | `0x1` | `0x11faec` | `0x19b` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xbfba8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xbfbe8 | `[[sp-32]]:1 != 0x63` | `[[sp-32]]:1 == 0x63` |
| `beq` at 0xbfc20 | `Z == 1` | `not (Z == 1)` |
| `ldrls` at 0xbfc34 | `[[sp-32]]:1-1 <= 0x5 (unsigned, 32-bit)` | `[[sp-32]]:1-1 > 0x5 (unsigned, 32-bit)` |
| `beq` at 0xbfd88 | `[sp-44] == 0x0` | `[sp-44] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbfe50 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xbfc34).

## `monitorSetVoltageLimits`

- `monitorSetVoltageLimits`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbff10 to 0xc01e8.
  Established. The byte at `sp -0x18` receives `([sp] & 0xff)` (`strb` at 0xbff48). The unnamed
  `0xc183c` call carries entry `r0`, `(r1 & 0xff)`, entry `r2`, `(r1 & 0xff)` (`bl` at 0xbff58). A
  word at `sp -0x2c` receives `0xfffff830` on an included path (`str` at 0xc0124). Voltage threshold
  programming remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbff20 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xbff28 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xbff30 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xbff38 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0xbff40 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `strb` at 0xbff48 | store | 1 | `sp -0x18` | `([sp] & 0xff)` |
| `ldrb` at 0xbff4c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xbff50 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbff60 | store | 4 | `sp -0x1c` | `ret(0xc183c)` |
| `ldrb` at 0xbff64 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xbff68 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbff78 | store | 4 | `sp -0x20` | `ret(0xc1970)` |
| `ldr` at 0xbff7c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xbff80 | load | 1 | `ret(0xc1970) +0x0` | `-` |
| `ldr` at 0xbff8c | load | 4 | `abs 0xc01e0` | `-` |
| `str` at 0xbff90 | store | 4 | `sp -0x2c` | `0xfffffc11` |
| `ldr` at 0xbff98 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbff9c | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0xbffa0 | store | 4 | `sp -0x30` | `[[sp-28]]:1` |
| `ldr` at 0xbffa4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbffb0 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xbffc0 | load | 1 | `sp -0x16` | `-` |
| `ldrls` at 0xbffc8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc0074 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0xc007c | load | 1 | `sp -0x16` | `-` |
| `ldrls` at 0xc0084 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc0118 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc0120 | load | 4 | `abs 0xc01e4` | `-` |
| `str` at 0xc0124 | store | 4 | `sp -0x2c` | `0xfffff830` |
| `ldr` at 0xc01d4 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0xc01dc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc01dc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc01dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbff58 | `(unknown callee) 0xc183c` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xbff70 | `(unknown callee) 0xc1970` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xbff88 | `[ret(0xc1970)]:1 != 0x0` | `[ret(0xc1970)]:1 == 0x0` |
| `beq` at 0xbffac | `[[sp-28]]:1 == 0x1` | `[[sp-28]]:1 != 0x1` |
| `beq` at 0xbffb8 | `[[sp-28]]:1 == 0x2` | `[[sp-28]]:1 != 0x2` |
| `ldrls` at 0xbffc8 | `[sp-22]:1 <= 0x9 (unsigned, 32-bit)` | `[sp-22]:1 > 0x9 (unsigned, 32-bit)` |
| `ldrls` at 0xc0084 | `[sp-22]:1 <= 0x8 (unsigned, 32-bit)` | `[sp-22]:1 > 0x8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc01dc | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xbffc8).

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xc0084).

## `monitorSetTempLimits`

- `monitorSetTempLimits`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc01e8 to 0xc04dc.
  Established. The byte at `sp -0x18` receives `([sp] & 0xff)` (`strb` at 0xc0220). The unnamed
  `0xc1970` call carries `[sp-20]`, `[sp-21]:1`, unknown, `[sp-21]:1` (`bl` at 0xc0248). The word at
  `sp -0x2c` receives `0xfffff830` on an included path (`str` at 0xc0418). Temperature threshold
  effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc01f8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc0200 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xc0208 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xc0210 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0xc0218 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `strb` at 0xc0220 | store | 1 | `sp -0x18` | `([sp] & 0xff)` |
| `ldrb` at 0xc0224 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc0228 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc0238 | store | 4 | `sp -0x1c` | `ret(0xc183c)` |
| `ldrb` at 0xc023c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc0240 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc0250 | store | 4 | `sp -0x20` | `ret(0xc1970)` |
| `ldr` at 0xc0254 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc0258 | load | 1 | `ret(0xc1970) +0x0` | `-` |
| `ldr` at 0xc0264 | load | 4 | `abs 0xc04d4` | `-` |
| `str` at 0xc0268 | store | 4 | `sp -0x2c` | `0xfffffc11` |
| `ldr` at 0xc0270 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc0274 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0xc0280 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc0414 | load | 4 | `abs 0xc04d8` | `-` |
| `str` at 0xc0418 | store | 4 | `sp -0x2c` | `0xfffff830` |
| `ldr` at 0xc04c8 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0xc04d0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc04d0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc04d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc0230 | `(unknown callee) 0xc183c` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xc0248 | `(unknown callee) 0xc1970` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc0260 | `[ret(0xc1970)]:1 != 0x0` | `[ret(0xc1970)]:1 == 0x0` |
| `ldrls` at 0xc0280 | `[[sp-28]]:1-1 <= 0x4 (unsigned, 32-bit)` | `[[sp-28]]:1-1 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc04d0 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xc0280).

## `monitorSetTachLimit`

- `monitorSetTachLimit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc04dc to 0xc061c.
  Established. The byte through `ret(0xc1970) +0x0` is read (`ldrb` at 0xc0538). A nonzero value
  takes `0xc0550`, while zero falls through (`bne` at 0xc0540). The unnamed `0xc4b5c` call carries
  `[sp-20]`, `[sp-32]`, `[sp-22]:1`, `[sp-28]` (`bl` at 0xc05f0). Tachometer limit programming
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc04ec | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc04f0 | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0xc04f8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xc0500 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xc0504 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc0508 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc0518 | store | 4 | `sp -0x20` | `ret(0xc183c)` |
| `ldrb` at 0xc051c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc0520 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc0530 | store | 4 | `sp -0x24` | `ret(0xc1970)` |
| `ldr` at 0xc0534 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc0538 | load | 1 | `ret(0xc1970) +0x0` | `-` |
| `ldr` at 0xc0544 | load | 4 | `abs 0xc0614` | `-` |
| `str` at 0xc0548 | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0xc0550 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc0554 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0xc0558 | store | 4 | `sp -0x2c` | `[[sp-32]]:1` |
| `ldr` at 0xc055c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc0568 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc0574 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc0584 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xc0594 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xc0598 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc059c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc05a4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc05b0 | store | 4 | `sp -0x28` | `ret(0xc2888)` |
| `ldrb` at 0xc05b8 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xc05bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc05c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc05c8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc05d4 | store | 4 | `sp -0x28` | `ret(0xc3510)` |
| `ldrb` at 0xc05dc | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xc05e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc05e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc05ec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc05f8 | store | 4 | `sp -0x28` | `ret(0xc4b5c)` |
| `ldr` at 0xc0600 | load | 4 | `abs 0xc0618` | `-` |
| `str` at 0xc0604 | store | 4 | `sp -0x28` | `0xfffff830` |
| `ldr` at 0xc0608 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xc0610 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc0610 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc0610 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc0510 | `(unknown callee) 0xc183c` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xc0528 | `(unknown callee) 0xc1970` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0xc05a8 | `(unknown callee) 0xc2888` | `[sp-20]` | `[sp-32]` | `[sp-22]:1` | `[sp-28]` |
| `bl` at 0xc05cc | `(unknown callee) 0xc3510` | `[sp-20]` | `[sp-32]` | `[sp-22]:1` | `[sp-28]` |
| `bl` at 0xc05f0 | `(unknown callee) 0xc4b5c` | `[sp-20]` | `[sp-32]` | `[sp-22]:1` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc0540 | `[ret(0xc1970)]:1 != 0x0` | `[ret(0xc1970)]:1 == 0x0` |
| `beq` at 0xc0564 | `[[sp-32]]:1 == 0x2` | `[[sp-32]]:1 != 0x2` |
| `bgt` at 0xc0570 | `[[sp-32]]:1 > 0x2 (signed, 32-bit)` | `[[sp-32]]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0xc057c | `[[sp-32]]:1 == 0x1` | `[[sp-32]]:1 != 0x1` |
| `beq` at 0xc058c | `[[sp-32]]:1 == 0x6` | `[[sp-32]]:1 != 0x6` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc0610 | `r0` | `?` |

## `monitorReadVoltage`

- `monitorReadVoltage`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc061c to 0xc0808.
  Established. The halfword through entry `r3 +0x0` receives zero (`strh` at 0xc064c). A nonzero
  `[ret(0xc1970)]:1` takes `0xc069c`, while zero falls through (`bne` at 0xc068c). The unnamed
  `0xc313c` call carries `[sp-20]`, `[sp-32]`, `[sp-22]:1`, `[sp-28]` (`bl` at 0xc0764). Voltage
  read semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc062c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc0630 | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0xc0638 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xc0640 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0xc0644 | load | 4 | `sp -0x1c` | `-` |
| `strh` at 0xc064c | store | 2 | `r3 +0x0` | `0x0` |
| `ldrb` at 0xc0650 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc0654 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc0664 | store | 4 | `sp -0x20` | `ret(0xc183c)` |
| `ldrb` at 0xc0668 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc066c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc067c | store | 4 | `sp -0x24` | `ret(0xc1970)` |
| `ldr` at 0xc0680 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc0684 | load | 1 | `ret(0xc1970) +0x0` | `-` |
| `ldr` at 0xc0690 | load | 4 | `abs 0xc0800` | `-` |
| `str` at 0xc0694 | store | 4 | `sp -0x30` | `0xfffffc11` |
| `ldr` at 0xc069c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc06a0 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0xc06a4 | store | 4 | `sp -0x34` | `[[sp-32]]:1` |
| `ldr` at 0xc06a8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc06b4 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xc06c4 | load | 1 | `sp -0x16` | `-` |
| `ldrls` at 0xc06cc | load | 4 | `(unknown)` | `-` |
| `str` at 0xc0748 | store | 4 | `sp -0x30` | `?` |
| `ldrb` at 0xc0750 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xc0754 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0758 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0760 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc076c | store | 4 | `sp -0x30` | `ret(0xc313c)` |
| `ldr` at 0xc0774 | load | 4 | `abs 0xc0804` | `-` |
| `str` at 0xc0778 | store | 4 | `sp -0x30` | `0xfffff830` |
| `ldr` at 0xc07f4 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0xc07fc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc07fc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc07fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc065c | `(unknown callee) 0xc183c` | `[sp-20]` | `[sp-21]:1` | `r3` | `[sp-21]:1` |
| `bl` at 0xc0674 | `(unknown callee) 0xc1970` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0xc0764 | `(unknown callee) 0xc313c` | `[sp-20]` | `[sp-32]` | `[sp-22]:1` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc068c | `[ret(0xc1970)]:1 != 0x0` | `[ret(0xc1970)]:1 == 0x0` |
| `beq` at 0xc06b0 | `[[sp-32]]:1 == 0x1` | `[[sp-32]]:1 != 0x1` |
| `beq` at 0xc06bc | `[[sp-32]]:1 == 0x2` | `[[sp-32]]:1 != 0x2` |
| `ldrls` at 0xc06cc | `[sp-22]:1 <= 0x9 (unsigned, 32-bit)` | `[sp-22]:1 > 0x9 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc07fc | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xc06cc).

## `monitorReadTemp`

- `monitorReadTemp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc0808 to 0xc0a10.
  Established. The byte through entry `r3 +0x0` receives zero (`strb` at 0xc0838). The unnamed
  `0xc183c` call carries `[sp-20]`, `[sp-21]:1`, entry `r3`, `[sp-21]:1` (`bl` at 0xc0848). A word
  at `sp -0x2c` receives `0xfffff830` on an included path (`str` at 0xc09b4). Temperature read
  behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc0818 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc081c | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0xc0824 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xc082c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0xc0830 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xc0838 | store | 1 | `r3 +0x0` | `0x0` |
| `ldrb` at 0xc083c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc0840 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc0850 | store | 4 | `sp -0x20` | `ret(0xc183c)` |
| `ldrb` at 0xc0854 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc0858 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc0868 | store | 4 | `sp -0x24` | `ret(0xc1970)` |
| `ldr` at 0xc086c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc0870 | load | 1 | `ret(0xc1970) +0x0` | `-` |
| `ldr` at 0xc087c | load | 4 | `abs 0xc0a08` | `-` |
| `str` at 0xc0880 | store | 4 | `sp -0x2c` | `0xfffffc11` |
| `ldr` at 0xc0888 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc088c | load | 1 | `[sp-32] +0x0` | `-` |
| `ldrls` at 0xc0898 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc09b0 | load | 4 | `abs 0xc0a0c` | `-` |
| `str` at 0xc09b4 | store | 4 | `sp -0x2c` | `0xfffff830` |
| `ldr` at 0xc09fc | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0xc0a04 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc0a04 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc0a04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc0848 | `(unknown callee) 0xc183c` | `[sp-20]` | `[sp-21]:1` | `r3` | `[sp-21]:1` |
| `bl` at 0xc0860 | `(unknown callee) 0xc1970` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc0878 | `[ret(0xc1970)]:1 != 0x0` | `[ret(0xc1970)]:1 == 0x0` |
| `ldrls` at 0xc0898 | `[[sp-32]]:1-1 <= 0x4 (unsigned, 32-bit)` | `[[sp-32]]:1-1 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc0a04 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xc0898).

## `monitorReadOutput`

- `monitorReadOutput`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc0e08 to 0xc102c.
  Established. Included byte assignments at `sp -0x25` write `0x19`, `0x30`, `0x31` and `0x32` in
  separate paths (`strb` at 0xc0edc, `strb` at 0xc0f28, `strb` at 0xc0f34, `strb` at 0xc0f40). The
  unnamed `0xbb5c4` call at `0xc1008` carries unknown, `[[sp-32]+2]:1`, `[sp-37]:1`, `[sp-28]` (`bl` at 0xc1008).
  The byte through `[sp-28] +0x0` receives an unknown value on an included path (`strb` at 0xc0fb8).
  Output read format remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc0e18 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc0e1c | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0xc0e24 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xc0e2c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xc0e30 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc0e34 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc0e44 | store | 4 | `sp -0x20` | `ret(0xc183c)` |
| `ldrb` at 0xc0e48 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc0e4c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc0e5c | store | 4 | `sp -0x24` | `ret(0xc1970)` |
| `ldr` at 0xc0e60 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc0e64 | load | 1 | `ret(0xc1970) +0x0` | `-` |
| `ldr` at 0xc0e70 | load | 4 | `abs 0xc1020` | `-` |
| `str` at 0xc0e74 | store | 4 | `sp -0x30` | `0xfffffc11` |
| `ldr` at 0xc0e7c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc0e80 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0xc0e84 | store | 4 | `sp -0x38` | `[[sp-32]]:1` |
| `ldr` at 0xc0e88 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc0e94 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc0ea0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc0eb0 | load | 4 | `sp -0x38` | `-` |
| `ldrb` at 0xc0ec0 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xc0ecc | load | 4 | `abs 0xc1024` | `-` |
| `str` at 0xc0ed0 | store | 4 | `sp -0x30` | `0xfffff82e` |
| `strb` at 0xc0edc | store | 1 | `sp -0x25` | `0x19` |
| `ldrb` at 0xc0ee4 | load | 1 | `sp -0x16` | `-` |
| `str` at 0xc0ee8 | store | 4 | `sp -0x34` | `[sp-22]:1` |
| `ldr` at 0xc0eec | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc0ef8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc0f04 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc0f14 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xc0f28 | store | 1 | `sp -0x25` | `0x30` |
| `strb` at 0xc0f34 | store | 1 | `sp -0x25` | `0x31` |
| `strb` at 0xc0f40 | store | 1 | `sp -0x25` | `0x32` |
| `ldr` at 0xc0f48 | load | 4 | `abs 0xc1024` | `-` |
| `str` at 0xc0f4c | store | 4 | `sp -0x30` | `0xfffff82e` |
| `ldrb` at 0xc0f5c | load | 1 | `sp -0x16` | `-` |
| `strb` at 0xc0f64 | store | 1 | `sp -0x25` | `?` |
| `ldr` at 0xc0f68 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc0f6c | load | 1 | `[sp-32] +0x1` | `-` |
| `ldr` at 0xc0f70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0f84 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc0f88 | load | 1 | `[sp-32] +0x2` | `-` |
| `ldrb` at 0xc0f8c | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0xc0f98 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc0fa4 | store | 4 | `sp -0x2c` | `ret(0xbb5c4)` |
| `ldr` at 0xc0fa8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0fac | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc0fb0 | load | 1 | `[sp-28] +0x0` | `-` |
| `strb` at 0xc0fb8 | store | 1 | `[sp-28] +0x0` | `?` |
| `ldr` at 0xc0fbc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0fc0 | store | 4 | `sp -0x30` | `[sp-44]` |
| `ldr` at 0xc0fc8 | load | 4 | `abs 0xc1028` | `-` |
| `str` at 0xc0fcc | store | 4 | `sp -0x30` | `0xfffff830` |
| `ldr` at 0xc0fd4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc0fd8 | load | 1 | `[sp-32] +0x1` | `-` |
| `ldr` at 0xc0fdc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0ff0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc0ff4 | load | 1 | `[sp-32] +0x2` | `-` |
| `ldrb` at 0xc0ff8 | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0xc1004 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc1010 | store | 4 | `sp -0x30` | `ret(0xbb5c4)` |
| `ldr` at 0xc1014 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0xc101c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc101c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc101c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc0e3c | `(unknown callee) 0xc183c` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xc0e54 | `(unknown callee) 0xc1970` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0xc0f78 | `(unknown callee) 0xbac78` | `[sp-20]` | `[[sp-32]+1]:1` | `?` | `[[sp-32]+1]:1` |
| `bl` at 0xc0f9c | `(unknown callee) 0xbb5c4` | `?` | `[[sp-32]+2]:1` | `[sp-37]:1` | `[sp-28]` |
| `bl` at 0xc0fe4 | `(unknown callee) 0xbac78` | `[sp-20]` | `[[sp-32]+1]:1` | `?` | `[[sp-32]+1]:1` |
| `bl` at 0xc1008 | `(unknown callee) 0xbb5c4` | `?` | `[[sp-32]+2]:1` | `[sp-37]:1` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc0e6c | `[ret(0xc1970)]:1 != 0x0` | `[ret(0xc1970)]:1 == 0x0` |
| `beq` at 0xc0e90 | `[[sp-32]]:1 == 0x2` | `[[sp-32]]:1 != 0x2` |
| `bgt` at 0xc0e9c | `[[sp-32]]:1 > 0x2 (signed, 32-bit)` | `[[sp-32]]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0xc0ea8 | `[[sp-32]]:1 == 0x1` | `[[sp-32]]:1 != 0x1` |
| `beq` at 0xc0eb8 | `[[sp-32]]:1 == 0x6` | `[[sp-32]]:1 != 0x6` |
| `beq` at 0xc0ec8 | `[sp-22]:1 == 0x0` | `[sp-22]:1 != 0x0` |
| `beq` at 0xc0ef4 | `[sp-22]:1 == 0x1` | `[sp-22]:1 != 0x1` |
| `bgt` at 0xc0f00 | `[sp-22]:1 > 0x1 (signed, 32-bit)` | `[sp-22]:1 <= 0x1 (signed, 32-bit)` |
| `beq` at 0xc0f0c | `[sp-22]:1 == 0x0` | `[sp-22]:1 != 0x0` |
| `beq` at 0xc0f1c | `[sp-22]:1 == 0x2` | `[sp-22]:1 != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc101c | `r0` | `?` |

## `monitorWriteOutput`

- `monitorWriteOutput`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc0b5c to 0xc0e08.
  Established. The unnamed `0xbb498` call at `0xc0cfc` carries unknown, `[[sp-28]+2]:1`,
  `[sp-34]:1`, `0xe2` (`bl` at 0xc0cfc); its later call at `0xc0de4` carries unknown,
  `[[sp-28]+2]:1`, `[sp-33]:1`, `[sp-23]:1` (`bl` at 0xc0de4). The byte at `sp -0x17` receives an
  unknown value on an included path (`strb` at 0xc0d94). Output programming effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc0b6c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xc0b78 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xc0b80 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0xc0b88 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldrb` at 0xc0b8c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc0b90 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc0ba0 | store | 4 | `sp -0x1c` | `ret(0xc183c)` |
| `ldrb` at 0xc0ba4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc0ba8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc0bb8 | store | 4 | `sp -0x20` | `ret(0xc1970)` |
| `ldr` at 0xc0bbc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc0bc0 | load | 1 | `ret(0xc1970) +0x0` | `-` |
| `ldr` at 0xc0bcc | load | 4 | `abs 0xc0dfc` | `-` |
| `str` at 0xc0bd0 | store | 4 | `sp -0x2c` | `0xfffffc11` |
| `ldr` at 0xc0bd8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc0bdc | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0xc0be0 | store | 4 | `sp -0x34` | `[[sp-28]]:1` |
| `ldr` at 0xc0be4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc0bf0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc0bfc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc0c0c | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xc0c1c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xc0c28 | load | 4 | `abs 0xc0e00` | `-` |
| `str` at 0xc0c2c | store | 4 | `sp -0x2c` | `0xfffff82e` |
| `strb` at 0xc0c38 | store | 1 | `sp -0x21` | `0x19` |
| `ldrb` at 0xc0c40 | load | 1 | `sp -0x16` | `-` |
| `str` at 0xc0c44 | store | 4 | `sp -0x30` | `[sp-22]:1` |
| `ldr` at 0xc0c48 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc0c54 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc0c60 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc0c70 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0xc0c84 | store | 1 | `sp -0x21` | `0x30` |
| `strb` at 0xc0c8c | store | 1 | `sp -0x22` | `0x5c` |
| `strb` at 0xc0c98 | store | 1 | `sp -0x21` | `0x31` |
| `strb` at 0xc0ca0 | store | 1 | `sp -0x22` | `0x5d` |
| `strb` at 0xc0cac | store | 1 | `sp -0x21` | `0x32` |
| `strb` at 0xc0cb4 | store | 1 | `sp -0x22` | `0x5e` |
| `ldr` at 0xc0cbc | load | 4 | `abs 0xc0e00` | `-` |
| `str` at 0xc0cc0 | store | 4 | `sp -0x2c` | `0xfffff82e` |
| `ldr` at 0xc0cc8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc0ccc | load | 1 | `[sp-28] +0x1` | `-` |
| `ldr` at 0xc0cd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0ce4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc0ce8 | load | 1 | `[sp-28] +0x2` | `-` |
| `ldrb` at 0xc0cec | load | 1 | `sp -0x22` | `-` |
| `str` at 0xc0d04 | store | 4 | `sp -0x28` | `ret(0xbb498)` |
| `ldr` at 0xc0d08 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0d14 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc0d18 | store | 4 | `sp -0x2c` | `ret(0xbb498)` |
| `ldrb` at 0xc0d28 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0xc0d30 | store | 1 | `sp -0x21` | `?` |
| `ldr` at 0xc0d34 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc0d38 | load | 1 | `[sp-28] +0x1` | `-` |
| `ldr` at 0xc0d3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0d50 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc0d54 | load | 1 | `[sp-28] +0x2` | `-` |
| `ldrb` at 0xc0d58 | load | 1 | `sp -0x21` | `-` |
| `str` at 0xc0d78 | store | 4 | `sp -0x28` | `ret(0xbb5c4)` |
| `ldrb` at 0xc0d7c | load | 1 | `sp -0x23` | `-` |
| `strb` at 0xc0d84 | store | 1 | `sp -0x23` | `?` |
| `ldrb` at 0xc0d88 | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0xc0d8c | load | 1 | `sp -0x23` | `-` |
| `strb` at 0xc0d94 | store | 1 | `sp -0x17` | `?` |
| `ldr` at 0xc0d9c | load | 4 | `abs 0xc0e04` | `-` |
| `str` at 0xc0da0 | store | 4 | `sp -0x2c` | `0xfffff830` |
| `ldr` at 0xc0da8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc0dac | load | 1 | `[sp-28] +0x1` | `-` |
| `ldr` at 0xc0db0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0dc4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc0dc8 | load | 1 | `[sp-28] +0x2` | `-` |
| `ldrb` at 0xc0dcc | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0xc0dd0 | load | 1 | `sp -0x17` | `-` |
| `str` at 0xc0dec | store | 4 | `sp -0x2c` | `ret(0xbb498)` |
| `ldr` at 0xc0df0 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0xc0df8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc0df8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc0df8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc0b98 | `(unknown callee) 0xc183c` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xc0bb0 | `(unknown callee) 0xc1970` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0xc0cd8 | `(unknown callee) 0xbac78` | `[sp-20]` | `[[sp-28]+1]:1` | `?` | `[[sp-28]+1]:1` |
| `bl` at 0xc0cfc | `(unknown callee) 0xbb498` | `?` | `[[sp-28]+2]:1` | `[sp-34]:1` | `0xe2` |
| `bl` at 0xc0d44 | `(unknown callee) 0xbac78` | `[sp-20]` | `[[sp-28]+1]:1` | `?` | `[[sp-28]+1]:1` |
| `bl` at 0xc0d70 | `(unknown callee) 0xbb5c4` | `?` | `[[sp-28]+2]:1` | `[sp-33]:1` | `sp-35` |
| `bl` at 0xc0db8 | `(unknown callee) 0xbac78` | `[sp-20]` | `[[sp-28]+1]:1` | `?` | `[[sp-28]+1]:1` |
| `bl` at 0xc0de4 | `(unknown callee) 0xbb498` | `?` | `[[sp-28]+2]:1` | `[sp-33]:1` | `[sp-23]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc0bc8 | `[ret(0xc1970)]:1 != 0x0` | `[ret(0xc1970)]:1 == 0x0` |
| `beq` at 0xc0bec | `[[sp-28]]:1 == 0x2` | `[[sp-28]]:1 != 0x2` |
| `bgt` at 0xc0bf8 | `[[sp-28]]:1 > 0x2 (signed, 32-bit)` | `[[sp-28]]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0xc0c04 | `[[sp-28]]:1 == 0x1` | `[[sp-28]]:1 != 0x1` |
| `beq` at 0xc0c14 | `[[sp-28]]:1 == 0x6` | `[[sp-28]]:1 != 0x6` |
| `beq` at 0xc0c24 | `[sp-22]:1 == 0x0` | `[sp-22]:1 != 0x0` |
| `beq` at 0xc0c50 | `[sp-22]:1 == 0x1` | `[sp-22]:1 != 0x1` |
| `bgt` at 0xc0c5c | `[sp-22]:1 > 0x1 (signed, 32-bit)` | `[sp-22]:1 <= 0x1 (signed, 32-bit)` |
| `beq` at 0xc0c68 | `[sp-22]:1 == 0x0` | `[sp-22]:1 != 0x0` |
| `beq` at 0xc0c78 | `[sp-22]:1 == 0x2` | `[sp-22]:1 != 0x2` |
| `beq` at 0xc0d10 | `ret(0xbb498) == 0x0` | `ret(0xbb498) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc0df8 | `r0` | `?` |

## `monitorInt`

- `monitorInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc1428 to 0xc1680.
  Established. The unnamed `0xc183c` call has four unknown candidates (`bl` at 0xc1478). The word
  through `[sp-32] +0x4` receives `[[sp-32]+4]+1` on an included path (`str` at 0xc1648). The word
  at `sp -0x28` receives `([sp-40] + [sp-44])` (`str` at 0xc1634). Monitor interrupt accounting
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc1438 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xc1440 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xc1448 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0xc1450 | store | 4 | `sp -0x28` | `0x0` |
| `ldrb` at 0xc1454 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0xc1458 | store | 1 | `sp -0x21` | `((r1 & 0xff) & 0xff)` |
| `ldrb` at 0xc145c | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0xc1460 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0xc146c | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xc1470 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc1480 | store | 4 | `sp -0x1c` | `ret(0xc183c)` |
| `ldrb` at 0xc1484 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xc1488 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc1498 | store | 4 | `sp -0x20` | `ret(0xc1970)` |
| `ldr` at 0xc149c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc14a0 | load | 1 | `ret(0xc1970) +0x0` | `-` |
| `ldr` at 0xc14ac | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc14b0 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0xc14bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc157c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc1584 | load | 4 | `abs 0xc166c` | `-` |
| `ldr` at 0xc1588 | load | 4 | `abs 0xc1670` | `-` |
| `str` at 0xc15c4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xc15c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc15d0 | load | 4 | `abs 0xc166c` | `-` |
| `ldr` at 0xc15d4 | load | 4 | `abs 0xc1670` | `-` |
| `ldr` at 0xc15e0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc15f0 | load | 4 | `abs 0xc1674` | `-` |
| `str` at 0xc15f4 | store | 4 | `(unknown)` | `0x11fbd8` |
| `str` at 0xc15f8 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0xc15fc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1600 | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0xc1604 | store | 4 | `(unknown)` | `[[sp-28]]:1` |
| `ldr` at 0xc160c | load | 4 | `abs 0xc1678` | `-` |
| `ldr` at 0xc1610 | load | 4 | `abs 0xc167c` | `-` |
| `ldr` at 0xc161c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc1628 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc162c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc1634 | store | 4 | `sp -0x28` | `([sp-40] + [sp-44])` |
| `ldr` at 0xc1638 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc163c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc1640 | load | 4 | `[sp-32] +0x4` | `-` |
| `str` at 0xc1648 | store | 4 | `[sp-32] +0x4` | `[[sp-32]+4]+1` |
| `ldrb` at 0xc164c | load | 1 | `sp -0x21` | `-` |
| `strb` at 0xc1654 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldr` at 0xc165c | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xc1668 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc1668 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc1668 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc1478 | `(unknown callee) 0xc183c` | `?` | `?` | `?` | `?` |
| `bl` at 0xc1490 | `(unknown callee) 0xc1970` | `[sp-20]` | `[sp-33]:1` | `?` | `[sp-33]:1` |
| `bl` at 0xc158c | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0x11fb44` | `0x11fb4c` |
| `bl` at 0xc1598 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xc15d8 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0x11fb44` | `0x11fb4c` |
| `bl` at 0xc15e8 | `(unknown callee) 0x19808` | `[sp-48]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xc1618 | `(unknown callee) 0x200a0` | `0x9` | `0x11faec` | `0x531` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhi` at 0xc1468 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0xc14a8 | `[ret(0xc1970)]:1 == 0x0` | `[ret(0xc1970)]:1 != 0x0` |
| `ldrls` at 0xc14bc | `[[sp-28]]:1-1 <= 0x5 (unsigned, 32-bit)` | `[[sp-28]]:1-1 > 0x5 (unsigned, 32-bit)` |
| `beq` at 0xc1624 | `[sp-44] == 0x0` | `[sp-44] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc1668 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xc14bc).
