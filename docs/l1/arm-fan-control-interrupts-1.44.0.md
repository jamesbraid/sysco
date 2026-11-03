# ARM fan control and interrupt paths in 1.44.0

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
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/fan-control-interrupts-facts.txt) retain all tool sections.

## `fanReset`

- `fanReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3f364 to 0x3fa14.
  Established. Included absolute word reads use `0x13220c` and `0x132214` (`ldr` at 0x3f390, `ldr` at 0x3f3a8). The unnamed `0xca584` call carries `[sp-28]`, `[[sp-32]+3]:1`, `[[sp-32]+5]:1`,
  `[sp-37]:1` (`bl` at 0x3f4e4). The unnamed `0xcda48` call carries `[sp-28]`, unknown, unknown,
  `[sp-88]` (`bl` at 0x3f9c8). Fan reset effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3f374 | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0x3f378 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f38c | load | 4 | `abs 0x3f9ec` | `-` |
| `ldr` at 0x3f390 | load | 4 | `abs 0x13220c` | `-` |
| `str` at 0x3f398 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x3f39c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f3a4 | load | 4 | `abs 0x3f9f0` | `-` |
| `ldr` at 0x3f3a8 | load | 4 | `abs 0x132214` | `-` |
| `str` at 0x3f3b0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x3f3b8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3f3c8 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x3f3cc | load | 4 | `sp -0x38` | `-` |
| `ldrb` at 0x3f3d0 | load | 1 | `[sp-56] +0x4` | `-` |
| `ldr` at 0x3f3d4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3f3e0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3f3e4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3f3f8 | load | 4 | `[sp-56] +0x0` | `-` |
| `str` at 0x3f400 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x3f404 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3f408 | load | 1 | `(unknown)` | `-` |
| `str` at 0x3f40c | store | 4 | `sp -0x60` | `?` |
| `ldr` at 0x3f410 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x3f41c | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x3f428 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x3f438 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x3f448 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3f44c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3f45c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3f460 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3f464 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f4a0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3f4a4 | load | 1 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x3f4ac | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3f4b4 | load | 1 | `[sp-32] +0x5` | `-` |
| `strb` at 0x3f4bc | store | 1 | `sp -0x25` | `?` |
| `ldr` at 0x3f4c0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3f4c4 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x3f4c8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3f4cc | load | 1 | `[sp-32] +0x5` | `-` |
| `ldrb` at 0x3f4d0 | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0x3f4d4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3f4ec | store | 4 | `sp -0x2c` | `ret(0xca584)` |
| `ldr` at 0x3f4f0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3f4fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f504 | load | 4 | `abs 0x3f9f4` | `-` |
| `ldr` at 0x3f508 | load | 4 | `abs 0x3f9f8` | `-` |
| `str` at 0x3f544 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x3f548 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f550 | load | 4 | `abs 0x3f9f4` | `-` |
| `ldr` at 0x3f554 | load | 4 | `abs 0x3f9f8` | `-` |
| `ldr` at 0x3f560 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x3f570 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3f574 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x3f578 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f588 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3f594 | load | 4 | `abs 0x3f9fc` | `-` |
| `str` at 0x3f598 | store | 4 | `(unknown)` | `0xf6634` |
| `str` at 0x3f59c | store | 4 | `(unknown)` | `ret(0x19808)` |
| `str` at 0x3f5a0 | store | 4 | `(unknown)` | `ret(0xcb12c)` |
| `str` at 0x3f5a4 | store | 4 | `(unknown)` | `ret(0x1fc1c)` |
| `ldr` at 0x3f5ac | load | 4 | `abs 0x3fa00` | `-` |
| `ldr` at 0x3f5c0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3f5c4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3f5c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f5e4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3f5e8 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x3f5fc | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x3f604 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3f610 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3f614 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x3f628 | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x3f630 | load | 1 | `(unknown)` | `-` |
| `str` at 0x3f634 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x3f638 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3f63c | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x3f650 | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x3f658 | load | 1 | `(unknown)` | `-` |
| `str` at 0x3f65c | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x3f660 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3f668 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x3f670 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3f674 | load | 1 | `[sp-32] +0x4` | `-` |
| `str` at 0x3f67c | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x3f680 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f694 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3f698 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x3f69c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3f6a0 | load | 1 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x3f6a4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3f6a8 | load | 1 | `[sp-32] +0x7` | `-` |
| `ldr` at 0x3f6ac | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3f6b8 | store | 4 | `sp -0x2c` | `ret(0xc0b5c)` |
| `ldr` at 0x3f6bc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3f6c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f6d0 | load | 4 | `abs 0x3f9f4` | `-` |
| `ldr` at 0x3f6d4 | load | 4 | `abs 0x3f9f8` | `-` |
| `str` at 0x3f710 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x3f714 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f71c | load | 4 | `abs 0x3f9f4` | `-` |
| `ldr` at 0x3f720 | load | 4 | `abs 0x3f9f8` | `-` |
| `ldr` at 0x3f72c | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x3f73c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3f740 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x3f744 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f754 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3f760 | load | 4 | `abs 0x3fa04` | `-` |
| `str` at 0x3f764 | store | 4 | `(unknown)` | `0xf6668` |
| `str` at 0x3f768 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `str` at 0x3f76c | store | 4 | `(unknown)` | `ret(0xc1680)` |
| `str` at 0x3f770 | store | 4 | `(unknown)` | `ret(0x1fc1c)` |
| `ldr` at 0x3f778 | load | 4 | `abs 0x3fa00` | `-` |
| `ldr` at 0x3f78c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f794 | load | 4 | `abs 0x3fa08` | `-` |
| `ldr` at 0x3f798 | load | 4 | `abs 0x3fa0c` | `-` |
| `str` at 0x3f7d4 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x3f7d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f7e0 | load | 4 | `abs 0x3fa08` | `-` |
| `ldr` at 0x3f7e4 | load | 4 | `abs 0x3fa0c` | `-` |
| `ldr` at 0x3f7f0 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x3f800 | load | 4 | `abs 0x3fa10` | `-` |
| `str` at 0x3f804 | store | 4 | `(unknown)` | `0xf66a0` |
| `str` at 0x3f808 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0x3f80c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3f810 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x3f814 | store | 4 | `(unknown)` | `[[sp-32]]:1` |
| `ldr` at 0x3f81c | load | 4 | `abs 0x3fa00` | `-` |
| `ldr` at 0x3f82c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x3f834 | store | 4 | `sp -0x30` | `?` |
| `str` at 0x3f840 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x3f844 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x3f848 | load | 1 | `[sp-52] +0x4` | `-` |
| `ldr` at 0x3f84c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3f858 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3f85c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3f870 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldrb` at 0x3f878 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3f88c | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x3f894 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3f8a4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3f8a8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3f8bc | load | 4 | `[sp-52] +0x0` | `-` |
| `ldrb` at 0x3f8c4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3f8d8 | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x3f8e0 | load | 1 | `(unknown)` | `-` |
| `str` at 0x3f8e4 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x3f8e8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3f8ec | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3f900 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldrb` at 0x3f908 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3f91c | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x3f924 | load | 1 | `(unknown)` | `-` |
| `str` at 0x3f928 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x3f92c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3f930 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3f944 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldrb` at 0x3f94c | load | 1 | `(unknown)` | `-` |
| `str` at 0x3f950 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x3f954 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3f958 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3f96c | load | 4 | `[sp-52] +0x0` | `-` |
| `ldrb` at 0x3f974 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3f988 | load | 4 | `sp -0x4c` | `-` |
| `ldrh` at 0x3f990 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x3f994 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x3f9a4 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x3f9a8 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x3f9b0 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x3f9b8 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x3f9bc | store | 4 | `(unknown)` | `[sp-92]` |
| `ldr` at 0x3f9c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f9c4 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x3f9cc | load | 4 | `sp -0x30` | `-` |
| `str` at 0x3f9d4 | store | 4 | `sp -0x30` | `?` |
| `ldm` at 0x3f9e8 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x3f9e8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x3f9e8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3f9e8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3f9e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3f3c0 | `(unknown callee) 0xbfe74` | `r0` | `sp-76` | `?` | `sp-76` |
| `bl` at 0x3f46c | `(unknown callee) 0xca484` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x3f484 | `(unknown callee) 0x1d444` | `0x4` | `?` | `?` | `ret(0xca484)` |
| `bl` at 0x3f498 | `(unknown callee) 0x1d444` | `0x5` | `?` | `?` | `ret(0x1d444)` |
| `bl` at 0x3f4e4 | `(unknown callee) 0xca584` | `[sp-28]` | `[[sp-32]+3]:1` | `[[sp-32]+5]:1` | `[sp-37]:1` |
| `bl` at 0x3f50c | `(unknown callee) 0x48da8` | `[sp-28]` | `0x0` | `0xf6578` | `0xf6580` |
| `bl` at 0x3f518 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x3f558 | `(unknown callee) 0x48da8` | `[sp-28]` | `0x0` | `0xf6578` | `0xf6580` |
| `bl` at 0x3f568 | `(unknown callee) 0x19808` | `[sp-80]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x3f580 | `(unknown callee) 0xcb12c` | `[sp-28]` | `[[sp-32]+3]:1` | `?` | `[[sp-32]+3]:1` |
| `bl` at 0x3f58c | `(unknown callee) 0x1fc1c` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0x3f5b8 | `(unknown callee) 0x200a0` | `0x1` | `0xf6528` | `0xec` | `0x0` |
| `bl` at 0x3f5d0 | `(unknown callee) 0xbfb1c` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x3f690 | `(unknown callee) 0xcd98c` | `[sp-28]` | `?` | `?` | `[[sp-32]+4]:1` |
| `bl` at 0x3f6b0 | `(unknown callee) 0xc0b5c` | `[sp-28]` | `[[sp-32]+3]:1` | `[[sp-32]+4]:1` | `[[sp-32]+7]:1` |
| `bl` at 0x3f6d8 | `(unknown callee) 0x48da8` | `[sp-28]` | `0x0` | `0xf6578` | `0xf6580` |
| `bl` at 0x3f6e4 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x3f724 | `(unknown callee) 0x48da8` | `[sp-28]` | `0x0` | `0xf6578` | `0xf6580` |
| `bl` at 0x3f734 | `(unknown callee) 0x19808` | `[sp-80]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x3f74c | `(unknown callee) 0xc1680` | `[sp-28]` | `[[sp-32]+3]:1` | `?` | `[[sp-32]+3]:1` |
| `bl` at 0x3f758 | `(unknown callee) 0x1fc1c` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0x3f784 | `(unknown callee) 0x200a0` | `0x1` | `0xf6528` | `0x104` | `0x0` |
| `bl` at 0x3f79c | `(unknown callee) 0x48da8` | `[sp-28]` | `0x0` | `0xf6694` | `0xf669c` |
| `bl` at 0x3f7a8 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x3f7e8 | `(unknown callee) 0x48da8` | `[sp-28]` | `0x0` | `0xf6694` | `0xf669c` |
| `bl` at 0x3f7f8 | `(unknown callee) 0x19808` | `[sp-80]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x3f828 | `(unknown callee) 0x200a0` | `0x1` | `0xf6528` | `0x110` | `0x0` |
| `bl` at 0x3f9c8 | `(unknown callee) 0xcda48` | `[sp-28]` | `?` | `?` | `[sp-88]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x3f3dc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x3f418 | `Z == 1` | `not (Z == 1)` |
| `bgt` at 0x3f424 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x3f430 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x3f440 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x3f454 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x3f478 | `ret(0xca484) != 0x0` | `ret(0xca484) == 0x0` |
| `beq` at 0x3f490 | `ret(0x1d444) == 0x0` | `ret(0x1d444) != 0x0` |
| `beq` at 0x3f4f8 | `ret(0xca584) == 0x0` | `ret(0xca584) != 0x0` |
| `bne` at 0x3f5dc | `ret(0xbfb1c) != 0x0` | `ret(0xbfb1c) == 0x0` |
| `bne` at 0x3f60c | `? != 0x6` | `? == 0x6` |
| `beq` at 0x3f6c4 | `ret(0xc0b5c) == 0x0` | `ret(0xc0b5c) != 0x0` |
| `ble` at 0x3f854 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x3f89c | `? == 0x6` | `? != 0x6` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3f9e8 | `r0` | `0x0` |

## `fanOn`

- `fanOn`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3fa84 to 0x3fe90.
  Established. The unnamed `0x41224` call carries entry `r0`, `(r1 & 0xff)`, `sp-29`, `sp-30` (`bl` at 0x3fadc). A zero result takes `0x3fb00`, while nonzero falls through (`beq` at 0x3faf0). The
  unnamed `0xca538` call carries `[sp-20]`, `[ret(0x40e84)+1]:1`, `[ret(0x40e84)+2]:1`,
  `[ret(0x40e84)+2]:1` (`bl` at 0x3fbd8). Fan enable behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3fa94 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x3fa9c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x3faa0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3fab4 | load | 4 | `abs 0x3fe60` | `-` |
| `ldr` at 0x3fab8 | load | 4 | `abs 0x13220c` | `-` |
| `str` at 0x3fac0 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0x3fac4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3fad0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3fae4 | store | 4 | `sp -0x24` | `ret(0x41224)` |
| `ldr` at 0x3fae8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3faf4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3faf8 | store | 4 | `sp -0x34` | `ret(0x41224)` |
| `ldrb` at 0x3fb00 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x3fb04 | store | 1 | `sp -0x1f` | `([sp-29]:1 & 0xff)` |
| `ldrb` at 0x3fb08 | load | 1 | `sp -0x1f` | `-` |
| `ldrb` at 0x3fb0c | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x3fb18 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x3fb1c | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x3fb20 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldrb` at 0x3fb28 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x3fb38 | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x3fb3c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x3fb4c | load | 1 | `ret(0x40d44) +0x0` | `-` |
| `ldr` at 0x3fb50 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3fb60 | store | 4 | `sp -0x1c` | `ret(0x40e84)` |
| `ldr` at 0x3fb64 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fb68 | load | 1 | `ret(0x40e84) +0x0` | `-` |
| `str` at 0x3fb6c | store | 4 | `sp -0x38` | `[ret(0x40e84)]:1` |
| `ldr` at 0x3fb70 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3fb7c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3fb88 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3fb98 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3fba8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fbac | load | 1 | `ret(0x40e84) +0x1` | `-` |
| `ldr` at 0x3fbbc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fbc0 | load | 1 | `ret(0x40e84) +0x1` | `-` |
| `ldr` at 0x3fbc4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fbc8 | load | 1 | `ret(0x40e84) +0x2` | `-` |
| `ldr` at 0x3fbcc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3fbe0 | store | 4 | `sp -0x28` | `ret(0xca538)` |
| `ldr` at 0x3fbe4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3fbf0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3fbf8 | load | 4 | `abs 0x3fe64` | `-` |
| `ldr` at 0x3fbfc | load | 4 | `abs 0x3fe68` | `-` |
| `str` at 0x3fc38 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x3fc3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3fc44 | load | 4 | `abs 0x3fe64` | `-` |
| `ldr` at 0x3fc48 | load | 4 | `abs 0x3fe68` | `-` |
| `ldr` at 0x3fc54 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3fc64 | load | 4 | `abs 0x3fe6c` | `-` |
| `str` at 0x3fc68 | store | 4 | `(unknown)` | `0xf66c4` |
| `str` at 0x3fc6c | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0x3fc70 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fc74 | load | 1 | `[sp-28] +0x1` | `-` |
| `str` at 0x3fc78 | store | 4 | `(unknown)` | `[[sp-28]+1]:1` |
| `ldr` at 0x3fc80 | load | 4 | `abs 0x3fe70` | `-` |
| `ldr` at 0x3fc84 | load | 4 | `abs 0x3fe74` | `-` |
| `ldr` at 0x3fc90 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3fc9c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3fca0 | store | 4 | `sp -0x24` | `[sp-40]` |
| `ldr` at 0x3fca8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fcac | load | 1 | `ret(0x40e84) +0x3` | `-` |
| `ldr` at 0x3fcb0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fcb4 | load | 1 | `ret(0x40e84) +0x4` | `-` |
| `ldr` at 0x3fcb8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fcbc | load | 1 | `ret(0x40e84) +0x7` | `-` |
| `ldr` at 0x3fcc0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3fccc | store | 4 | `sp -0x28` | `ret(0xc0b5c)` |
| `ldr` at 0x3fcd0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3fcdc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3fce4 | load | 4 | `abs 0x3fe64` | `-` |
| `ldr` at 0x3fce8 | load | 4 | `abs 0x3fe68` | `-` |
| `str` at 0x3fd24 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x3fd28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3fd30 | load | 4 | `abs 0x3fe64` | `-` |
| `ldr` at 0x3fd34 | load | 4 | `abs 0x3fe68` | `-` |
| `ldr` at 0x3fd40 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3fd50 | load | 4 | `abs 0x3fe6c` | `-` |
| `str` at 0x3fd54 | store | 4 | `(unknown)` | `0xf66c4` |
| `str` at 0x3fd58 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0x3fd5c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fd60 | load | 1 | `[sp-28] +0x4` | `-` |
| `str` at 0x3fd64 | store | 4 | `(unknown)` | `[[sp-28]+4]:1` |
| `ldr` at 0x3fd6c | load | 4 | `abs 0x3fe70` | `-` |
| `ldr` at 0x3fd70 | load | 4 | `abs 0x3fe78` | `-` |
| `ldr` at 0x3fd7c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3fd88 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3fd8c | store | 4 | `sp -0x24` | `[sp-40]` |
| `ldr` at 0x3fd94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3fd9c | load | 4 | `abs 0x3fe7c` | `-` |
| `ldr` at 0x3fda0 | load | 4 | `abs 0x3fe80` | `-` |
| `str` at 0x3fddc | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x3fde0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3fde8 | load | 4 | `abs 0x3fe7c` | `-` |
| `ldr` at 0x3fdec | load | 4 | `abs 0x3fe80` | `-` |
| `ldr` at 0x3fdf8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3fe08 | load | 4 | `abs 0x3fe84` | `-` |
| `str` at 0x3fe0c | store | 4 | `(unknown)` | `0xf66e8` |
| `str` at 0x3fe10 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0x3fe14 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fe18 | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0x3fe1c | store | 4 | `(unknown)` | `[[sp-28]]:1` |
| `ldr` at 0x3fe24 | load | 4 | `abs 0x3fe70` | `-` |
| `ldr` at 0x3fe28 | load | 4 | `abs 0x3fe88` | `-` |
| `ldr` at 0x3fe34 | load | 4 | `abs 0x3fe8c` | `-` |
| `str` at 0x3fe38 | store | 4 | `sp -0x24` | `0xfffffa86` |
| `ldrb` at 0x3fe3c | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0x3fe44 | store | 1 | `sp -0x1f` | `?` |
| `ldr` at 0x3fe4c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3fe50 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x3fe54 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x3fe5c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3fe5c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3fe5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3fadc | `(unknown callee) 0x41224` | `r0` | `(r1 & 0xff)` | `sp-29` | `sp-30` |
| `bl` at 0x3fb44 | `(unknown callee) 0x40d44` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x3fb58 | `(unknown callee) 0x40e84` | `[sp-20]` | `[ret(0x40d44)]:1` | `?` | `[ret(0x40d44)]:1` |
| `bl` at 0x3fbd8 | `(unknown callee) 0xca538` | `[sp-20]` | `[ret(0x40e84)+1]:1` | `[ret(0x40e84)+2]:1` | `[ret(0x40e84)+2]:1` |
| `bl` at 0x3fc00 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0xf6578` | `0xf6580` |
| `bl` at 0x3fc0c | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x3fc4c | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0xf6578` | `0xf6580` |
| `bl` at 0x3fc5c | `(unknown callee) 0x19808` | `[sp-48]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x3fc8c | `(unknown callee) 0x200a0` | `0x1` | `0xf6528` | `0x16d` | `0x0` |
| `bl` at 0x3fcc4 | `(unknown callee) 0xc0b5c` | `[sp-20]` | `[ret(0x40e84)+3]:1` | `[ret(0x40e84)+4]:1` | `[ret(0x40e84)+7]:1` |
| `bl` at 0x3fcec | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0xf6578` | `0xf6580` |
| `bl` at 0x3fcf8 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x3fd38 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0xf6578` | `0xf6580` |
| `bl` at 0x3fd48 | `(unknown callee) 0x19808` | `[sp-48]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x3fd78 | `(unknown callee) 0x200a0` | `0x1` | `0xf6528` | `0x179` | `0x0` |
| `bl` at 0x3fda4 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0xf6694` | `0xf669c` |
| `bl` at 0x3fdb0 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x3fdf0 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0xf6694` | `0xf669c` |
| `bl` at 0x3fe00 | `(unknown callee) 0x19808` | `[sp-48]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x3fe30 | `(unknown callee) 0x200a0` | `0x1` | `0xf6528` | `0x186` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3faf0 | `ret(0x41224) == 0x0` | `ret(0x41224) != 0x0` |
| `bhi` at 0x3fb14 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0x3fb30 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x3fb78 | `[ret(0x40e84)]:1 == 0x2` | `[ret(0x40e84)]:1 != 0x2` |
| `bgt` at 0x3fb84 | `[ret(0x40e84)]:1 > 0x2 (signed, 32-bit)` | `[ret(0x40e84)]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0x3fb90 | `[ret(0x40e84)]:1 == 0x1` | `[ret(0x40e84)]:1 != 0x1` |
| `beq` at 0x3fba0 | `[ret(0x40e84)]:1 == 0x63` | `[ret(0x40e84)]:1 != 0x63` |
| `bne` at 0x3fbb4 | `[ret(0x40e84)+1]:1 != 0xff` | `[ret(0x40e84)+1]:1 == 0xff` |
| `beq` at 0x3fbec | `ret(0xca538) == 0x0` | `ret(0xca538) != 0x0` |
| `bne` at 0x3fc98 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `beq` at 0x3fcd8 | `ret(0xc0b5c) == 0x0` | `ret(0xc0b5c) != 0x0` |
| `bne` at 0x3fd84 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3fe5c | `r0` | `?` |

## `fanOff`

- `fanOff`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3fe90 to 0x40294.
  Established. The unnamed `0xca4e8` call carries `[sp-20]`, `[ret(0x40e84)+1]:1`,
  `[ret(0x40e84)+2]:1`, `[ret(0x40e84)+2]:1` (`bl` at 0x3ffe4). A zero result takes `0x4024c`, while
  nonzero falls through (`beq` at 0x3fff8). The unnamed `0xc0b5c` call carries `[sp-20]`,
  `[ret(0x40e84)+3]:1`, `[ret(0x40e84)+4]:1`, `0x0` (`bl` at 0x400d4). Fan disable effects remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3fea0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x3fea8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x3feac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3fec0 | load | 4 | `abs 0x40270` | `-` |
| `ldr` at 0x3fec4 | load | 4 | `abs 0x13220c` | `-` |
| `str` at 0x3fecc | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0x3fed0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3fedc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3fef0 | store | 4 | `sp -0x24` | `ret(0x41224)` |
| `ldr` at 0x3fef4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3ff00 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3ff04 | store | 4 | `sp -0x34` | `ret(0x41224)` |
| `ldrb` at 0x3ff0c | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x3ff10 | store | 1 | `sp -0x1f` | `([sp-29]:1 & 0xff)` |
| `ldrb` at 0x3ff14 | load | 1 | `sp -0x1f` | `-` |
| `ldrb` at 0x3ff18 | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x3ff24 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x3ff28 | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x3ff2c | load | 4 | `[sp-44] +0x8` | `-` |
| `ldrb` at 0x3ff34 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x3ff44 | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x3ff48 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x3ff58 | load | 1 | `ret(0x40d44) +0x0` | `-` |
| `ldr` at 0x3ff5c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3ff6c | store | 4 | `sp -0x1c` | `ret(0x40e84)` |
| `ldr` at 0x3ff70 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3ff74 | load | 1 | `ret(0x40e84) +0x0` | `-` |
| `str` at 0x3ff78 | store | 4 | `sp -0x38` | `[ret(0x40e84)]:1` |
| `ldr` at 0x3ff7c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3ff88 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3ff94 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3ffa4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3ffb4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3ffb8 | load | 1 | `ret(0x40e84) +0x1` | `-` |
| `ldr` at 0x3ffc8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3ffcc | load | 1 | `ret(0x40e84) +0x1` | `-` |
| `ldr` at 0x3ffd0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3ffd4 | load | 1 | `ret(0x40e84) +0x2` | `-` |
| `ldr` at 0x3ffd8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3ffec | store | 4 | `sp -0x28` | `ret(0xca4e8)` |
| `ldr` at 0x3fff0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3fffc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x40004 | load | 4 | `abs 0x40274` | `-` |
| `ldr` at 0x40008 | load | 4 | `abs 0x40278` | `-` |
| `str` at 0x40044 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x40048 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x40050 | load | 4 | `abs 0x40274` | `-` |
| `ldr` at 0x40054 | load | 4 | `abs 0x40278` | `-` |
| `ldr` at 0x40060 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x40070 | load | 4 | `abs 0x4027c` | `-` |
| `str` at 0x40074 | store | 4 | `(unknown)` | `0xf670c` |
| `str` at 0x40078 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0x4007c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x40080 | load | 1 | `[sp-28] +0x1` | `-` |
| `str` at 0x40084 | store | 4 | `(unknown)` | `[[sp-28]+1]:1` |
| `ldr` at 0x4008c | load | 4 | `abs 0x40280` | `-` |
| `ldr` at 0x40090 | load | 4 | `abs 0x40284` | `-` |
| `ldr` at 0x4009c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x400a8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x400ac | store | 4 | `sp -0x24` | `[sp-40]` |
| `ldr` at 0x400b4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x400b8 | load | 1 | `ret(0x40e84) +0x3` | `-` |
| `ldr` at 0x400bc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x400c0 | load | 1 | `ret(0x40e84) +0x4` | `-` |
| `ldr` at 0x400c4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x400dc | store | 4 | `sp -0x28` | `ret(0xc0b5c)` |
| `ldr` at 0x400e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x400ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x400f4 | load | 4 | `abs 0x40274` | `-` |
| `ldr` at 0x400f8 | load | 4 | `abs 0x40278` | `-` |
| `str` at 0x40134 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x40138 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x40140 | load | 4 | `abs 0x40274` | `-` |
| `ldr` at 0x40144 | load | 4 | `abs 0x40278` | `-` |
| `ldr` at 0x40150 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x40160 | load | 4 | `abs 0x4027c` | `-` |
| `str` at 0x40164 | store | 4 | `(unknown)` | `0xf670c` |
| `str` at 0x40168 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0x4016c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x40170 | load | 1 | `[sp-28] +0x3` | `-` |
| `str` at 0x40174 | store | 4 | `(unknown)` | `[[sp-28]+3]:1` |
| `ldr` at 0x4017c | load | 4 | `abs 0x40280` | `-` |
| `ldr` at 0x40180 | load | 4 | `abs 0x40288` | `-` |
| `ldr` at 0x4018c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x40198 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x4019c | store | 4 | `sp -0x24` | `[sp-40]` |
| `ldr` at 0x401a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x401ac | load | 4 | `abs 0x40274` | `-` |
| `ldr` at 0x401b0 | load | 4 | `abs 0x40278` | `-` |
| `str` at 0x401ec | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x401f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x401f8 | load | 4 | `abs 0x40274` | `-` |
| `ldr` at 0x401fc | load | 4 | `abs 0x40278` | `-` |
| `ldr` at 0x40208 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x40218 | load | 4 | `abs 0x4028c` | `-` |
| `str` at 0x4021c | store | 4 | `(unknown)` | `0xf6730` |
| `str` at 0x40220 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0x40224 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x40228 | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0x4022c | store | 4 | `(unknown)` | `[[sp-28]]:1` |
| `ldr` at 0x40234 | load | 4 | `abs 0x40280` | `-` |
| `ldr` at 0x40244 | load | 4 | `abs 0x40290` | `-` |
| `str` at 0x40248 | store | 4 | `sp -0x24` | `0xfffffa86` |
| `ldrb` at 0x4024c | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0x40254 | store | 1 | `sp -0x1f` | `?` |
| `ldr` at 0x4025c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x40260 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x40264 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x4026c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4026c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4026c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3fee8 | `(unknown callee) 0x41224` | `r0` | `(r1 & 0xff)` | `sp-29` | `sp-30` |
| `bl` at 0x3ff50 | `(unknown callee) 0x40d44` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x3ff64 | `(unknown callee) 0x40e84` | `[sp-20]` | `[ret(0x40d44)]:1` | `?` | `[ret(0x40d44)]:1` |
| `bl` at 0x3ffe4 | `(unknown callee) 0xca4e8` | `[sp-20]` | `[ret(0x40e84)+1]:1` | `[ret(0x40e84)+2]:1` | `[ret(0x40e84)+2]:1` |
| `bl` at 0x4000c | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0xf6578` | `0xf6580` |
| `bl` at 0x40018 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x40058 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0xf6578` | `0xf6580` |
| `bl` at 0x40068 | `(unknown callee) 0x19808` | `[sp-48]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x40098 | `(unknown callee) 0x200a0` | `0x1` | `0xf6528` | `0x1af` | `0x0` |
| `bl` at 0x400d4 | `(unknown callee) 0xc0b5c` | `[sp-20]` | `[ret(0x40e84)+3]:1` | `[ret(0x40e84)+4]:1` | `0x0` |
| `bl` at 0x400fc | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0xf6578` | `0xf6580` |
| `bl` at 0x40108 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x40148 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0xf6578` | `0xf6580` |
| `bl` at 0x40158 | `(unknown callee) 0x19808` | `[sp-48]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x40188 | `(unknown callee) 0x200a0` | `0x1` | `0xf6528` | `0x1bb` | `0x0` |
| `bl` at 0x401b4 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0xf6578` | `0xf6580` |
| `bl` at 0x401c0 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x40200 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0xf6578` | `0xf6580` |
| `bl` at 0x40210 | `(unknown callee) 0x19808` | `[sp-48]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x40240 | `(unknown callee) 0x200a0` | `0x1` | `0xf6528` | `0x1c8` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3fefc | `ret(0x41224) == 0x0` | `ret(0x41224) != 0x0` |
| `bhi` at 0x3ff20 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0x3ff3c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x3ff84 | `[ret(0x40e84)]:1 == 0x2` | `[ret(0x40e84)]:1 != 0x2` |
| `bgt` at 0x3ff90 | `[ret(0x40e84)]:1 > 0x2 (signed, 32-bit)` | `[ret(0x40e84)]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0x3ff9c | `[ret(0x40e84)]:1 == 0x1` | `[ret(0x40e84)]:1 != 0x1` |
| `beq` at 0x3ffac | `[ret(0x40e84)]:1 == 0x63` | `[ret(0x40e84)]:1 != 0x63` |
| `bne` at 0x3ffc0 | `[ret(0x40e84)+1]:1 != 0xff` | `[ret(0x40e84)+1]:1 == 0xff` |
| `beq` at 0x3fff8 | `ret(0xca4e8) == 0x0` | `ret(0xca4e8) != 0x0` |
| `bne` at 0x400a4 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `beq` at 0x400e8 | `ret(0xc0b5c) == 0x0` | `ret(0xc0b5c) != 0x0` |
| `bne` at 0x40194 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4026c | `r0` | `?` |

## `fanStatus`

- `fanStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x40294 to 0x404c4.
  Established. A nonzero `ret(0x37674)` takes `0x402f4`, while zero falls through (`bne` at 0x402e4). The unnamed `0x41224` call carries `[sp-20]`, `[sp-21]:1`, `sp-29`, `sp-30` (`bl` at 0x4030c). The unnamed `0xc0e08` call carries `[sp-20]`, `[ret(0x40e84)+3]:1`,
  `[ret(0x40e84)+4]:1`, `sp-37` (`bl` at 0x40454). Fan status classification remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x402a4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x402ac | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x402b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x402c4 | load | 4 | `abs 0x404c0` | `-` |
| `ldr` at 0x402c8 | load | 4 | `abs 0x13220c` | `-` |
| `str` at 0x402d0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x402d4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x402ec | store | 4 | `sp -0x30` | `0x0` |
| `ldrb` at 0x402f4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x40300 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x40314 | store | 4 | `sp -0x24` | `ret(0x41224)` |
| `ldr` at 0x40318 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x40324 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x40328 | store | 4 | `sp -0x30` | `ret(0x41224)` |
| `ldrb` at 0x40330 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x40334 | store | 1 | `sp -0x1f` | `([sp-29]:1 & 0xff)` |
| `ldrb` at 0x40338 | load | 1 | `sp -0x1f` | `-` |
| `ldrb` at 0x4033c | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x40348 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x4034c | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x40350 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldrb` at 0x40358 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x40368 | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x4036c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x4037c | load | 1 | `ret(0x40d44) +0x0` | `-` |
| `ldr` at 0x40380 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x40390 | store | 4 | `sp -0x1c` | `ret(0x40e84)` |
| `ldr` at 0x40394 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x40398 | load | 1 | `ret(0x40e84) +0x0` | `-` |
| `str` at 0x4039c | store | 4 | `sp -0x34` | `[ret(0x40e84)]:1` |
| `ldr` at 0x403a0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x403ac | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x403b8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x403c8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x403d8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x403dc | load | 1 | `ret(0x40e84) +0x1` | `-` |
| `str` at 0x403ec | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x403f4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x403f8 | load | 1 | `ret(0x40e84) +0x1` | `-` |
| `ldr` at 0x403fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4040c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x40410 | load | 1 | `[sp-28] +0x2` | `-` |
| `str` at 0x40428 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x40430 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x40434 | load | 1 | `ret(0x40e84) +0x3` | `-` |
| `ldr` at 0x40438 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4043c | load | 1 | `ret(0x40e84) +0x4` | `-` |
| `ldr` at 0x40444 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x40458 | load | 1 | `sp -0x25` | `-` |
| `str` at 0x40468 | store | 4 | `sp -0x24` | `?` |
| `str` at 0x40474 | store | 4 | `sp -0x24` | `0x1` |
| `str` at 0x40480 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x40484 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x40490 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x40494 | store | 4 | `sp -0x30` | `[sp-36]` |
| `ldrb` at 0x4049c | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0x404a4 | store | 1 | `sp -0x1f` | `?` |
| `ldr` at 0x404ac | load | 4 | `sp -0x24` | `-` |
| `str` at 0x404b0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x404b4 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x404bc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x404bc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x404bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x402d8 | `(unknown callee) 0x37674` | `r0` | `r1` | `?` | `?` |
| `bl` at 0x4030c | `(unknown callee) 0x41224` | `[sp-20]` | `[sp-21]:1` | `sp-29` | `sp-30` |
| `bl` at 0x40374 | `(unknown callee) 0x40d44` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x40388 | `(unknown callee) 0x40e84` | `[sp-20]` | `[ret(0x40d44)]:1` | `?` | `[ret(0x40d44)]:1` |
| `bl` at 0x40404 | `(unknown callee) 0xca7a0` | `[sp-20]` | `[ret(0x40e84)+1]:1` | `?` | `[ret(0x40e84)+1]:1` |
| `bl` at 0x40454 | `(unknown callee) 0xc0e08` | `[sp-20]` | `[ret(0x40e84)+3]:1` | `[ret(0x40e84)+4]:1` | `sp-37` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x402e4 | `ret(0x37674) != 0x0` | `ret(0x37674) == 0x0` |
| `beq` at 0x40320 | `ret(0x41224) == 0x0` | `ret(0x41224) != 0x0` |
| `bhi` at 0x40344 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0x40360 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x403a8 | `[ret(0x40e84)]:1 == 0x2` | `[ret(0x40e84)]:1 != 0x2` |
| `bgt` at 0x403b4 | `[ret(0x40e84)]:1 > 0x2 (signed, 32-bit)` | `[ret(0x40e84)]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0x403c0 | `[ret(0x40e84)]:1 == 0x1` | `[ret(0x40e84)]:1 != 0x1` |
| `beq` at 0x403d0 | `[ret(0x40e84)]:1 == 0x63` | `[ret(0x40e84)]:1 != 0x63` |
| `bne` at 0x403e4 | `[ret(0x40e84)+1]:1 != 0xff` | `[ret(0x40e84)+1]:1 == 0xff` |
| `beq` at 0x4048c | `[sp-36] == 0x0` | `[sp-36] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x404bc | `r0` | `?` |

## `fanSetSpeedRaw`

- `fanSetSpeedRaw`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x404c4 to 0x40764.
  Established. The unnamed `0x1b5f0` call at `0x40640` carries `0x4`, `0x1`, unknown, unknown (`bl` at 0x40640). The unnamed `0xca584` call carries `[sp-20]`, `[[sp-28]+3]:1`, `[[sp-28]+5]:1`,
  `[sp-32]:1` (`bl` at 0x4068c). The unnamed `0xc0b5c` call carries `[sp-20]`, `[ret(0x40e84)+3]:1`,
  `[ret(0x40e84)+4]:1`, `[sp-22]:1` (`bl` at 0x406e0). Raw fan speed control effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x404d4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x404dc | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x404e4 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0x404e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x404fc | load | 4 | `abs 0x40758` | `-` |
| `ldr` at 0x40500 | load | 4 | `abs 0x13220c` | `-` |
| `str` at 0x40508 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0x4050c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x40518 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4052c | store | 4 | `sp -0x24` | `ret(0x41224)` |
| `ldr` at 0x40530 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4053c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x40540 | store | 4 | `sp -0x30` | `ret(0x41224)` |
| `ldrb` at 0x40548 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x4054c | store | 1 | `sp -0x1f` | `([sp-29]:1 & 0xff)` |
| `ldrb` at 0x40550 | load | 1 | `sp -0x1f` | `-` |
| `ldrb` at 0x40554 | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x40560 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x40564 | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x40568 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldrb` at 0x40570 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x4057c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x40580 | load | 1 | `[sp-44] +0x4` | `-` |
| `ldrb` at 0x40584 | load | 1 | `sp -0x1f` | `-` |
| `ldrb` at 0x40590 | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x40594 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x405a4 | load | 1 | `ret(0x40d44) +0x0` | `-` |
| `ldr` at 0x405a8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x405b8 | store | 4 | `sp -0x1c` | `ret(0x40e84)` |
| `ldr` at 0x405bc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x405c0 | load | 1 | `ret(0x40e84) +0x0` | `-` |
| `str` at 0x405c4 | store | 4 | `sp -0x34` | `[ret(0x40e84)]:1` |
| `ldr` at 0x405c8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x405d4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x405e0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x405f0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x40600 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x40604 | load | 1 | `ret(0x40e84) +0x3` | `-` |
| `ldr` at 0x40610 | load | 4 | `abs 0x4075c` | `-` |
| `str` at 0x40614 | store | 4 | `sp -0x30` | `0xfffffa87` |
| `ldrb` at 0x4061c | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x40624 | store | 1 | `sp -0x20` | `?` |
| `ldrb` at 0x40628 | load | 1 | `sp -0x20` | `-` |
| `ldrb` at 0x40644 | load | 1 | `sp -0x20` | `-` |
| `ldr` at 0x40648 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4064c | load | 1 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x40654 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4065c | load | 1 | `[sp-28] +0x5` | `-` |
| `strb` at 0x40664 | store | 1 | `sp -0x20` | `?` |
| `ldr` at 0x40668 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4066c | load | 1 | `[sp-28] +0x3` | `-` |
| `ldr` at 0x40670 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x40674 | load | 1 | `[sp-28] +0x5` | `-` |
| `ldrb` at 0x40678 | load | 1 | `sp -0x20` | `-` |
| `ldr` at 0x4067c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x40694 | store | 4 | `sp -0x28` | `ret(0xca584)` |
| `ldr` at 0x40698 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x406a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x406b0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x406b4 | store | 4 | `sp -0x24` | `ret(0xca584)` |
| `ldr` at 0x406bc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x406c0 | load | 1 | `ret(0x40e84) +0x3` | `-` |
| `ldr` at 0x406c4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x406c8 | load | 1 | `ret(0x40e84) +0x4` | `-` |
| `ldrb` at 0x406cc | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x406d0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x406e8 | store | 4 | `sp -0x28` | `ret(0xc0b5c)` |
| `ldr` at 0x406ec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x406f8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x40704 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x40708 | store | 4 | `sp -0x24` | `ret(0xc0b5c)` |
| `ldrb` at 0x4070c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x4072c | load | 4 | `abs 0x40760` | `-` |
| `str` at 0x40730 | store | 4 | `sp -0x24` | `0xfffffa86` |
| `ldrb` at 0x40734 | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0x4073c | store | 1 | `sp -0x1f` | `?` |
| `ldr` at 0x40744 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x40748 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x4074c | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x40754 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x40754 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x40754 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x40524 | `(unknown callee) 0x41224` | `r0` | `(r1 & 0xff)` | `sp-29` | `sp-30` |
| `bl` at 0x4059c | `(unknown callee) 0x40d44` | `[sp-20]` | `?` | `[[sp-44]+4]:1` | `?` |
| `bl` at 0x405b0 | `(unknown callee) 0x40e84` | `[sp-20]` | `[ret(0x40d44)]:1` | `?` | `[ret(0x40d44)]:1` |
| `bl` at 0x40634 | `(unknown callee) 0x1b5f0` | `0x5` | `[sp-32]:1` | `?` | `[sp-32]:1` |
| `bl` at 0x40640 | `(unknown callee) 0x1b5f0` | `0x4` | `0x1` | `?` | `?` |
| `bl` at 0x4068c | `(unknown callee) 0xca584` | `[sp-20]` | `[[sp-28]+3]:1` | `[[sp-28]+5]:1` | `[sp-32]:1` |
| `bl` at 0x406e0 | `(unknown callee) 0xc0b5c` | `[sp-20]` | `[ret(0x40e84)+3]:1` | `[ret(0x40e84)+4]:1` | `[sp-22]:1` |
| `bl` at 0x40718 | `(unknown callee) 0x1b5f0` | `0x5` | `[sp-22]:1` | `?` | `[sp-22]:1` |
| `bl` at 0x40724 | `(unknown callee) 0x1b5f0` | `0x4` | `0x1` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x40538 | `ret(0x41224) == 0x0` | `ret(0x41224) != 0x0` |
| `bhi` at 0x4055c | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0x40578 | `Z == 1` | `not (Z == 1)` |
| `bls` at 0x4058c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x405d0 | `[ret(0x40e84)]:1 == 0x2` | `[ret(0x40e84)]:1 != 0x2` |
| `bgt` at 0x405dc | `[ret(0x40e84)]:1 > 0x2 (signed, 32-bit)` | `[ret(0x40e84)]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0x405e8 | `[ret(0x40e84)]:1 == 0x1` | `[ret(0x40e84)]:1 != 0x1` |
| `beq` at 0x405f8 | `[ret(0x40e84)]:1 == 0x63` | `[ret(0x40e84)]:1 != 0x63` |
| `bne` at 0x4060c | `[ret(0x40e84)+3]:1 != 0xff` | `[ret(0x40e84)+3]:1 == 0xff` |
| `beq` at 0x406a0 | `ret(0xca584) == 0x0` | `ret(0xca584) != 0x0` |
| `bne` at 0x406ac | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `beq` at 0x406f4 | `ret(0xc0b5c) == 0x0` | `ret(0xc0b5c) != 0x0` |
| `bne` at 0x40700 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x40754 | `r0` | `?` |

## `fanSetSpeed`

- `fanSetSpeed`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x40764 to 0x4096c.
  Established. The unnamed `0x41224` call carries entry `r0`, `(r1 & 0xff)`, `sp-30`, `sp-31` (`bl` at 0x407c4). A zero result takes `0x407e8`, while nonzero falls through (`beq` at 0x407d8). The
  unnamed `0x404c4` call carries `[sp-20]`, `[sp-32]:1`, unknown, `[sp-32]:1` (`bl` at 0x40914). Fan
  speed selection policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x40774 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x4077c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x40784 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0x40788 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4079c | load | 4 | `abs 0x40964` | `-` |
| `ldr` at 0x407a0 | load | 4 | `abs 0x13220c` | `-` |
| `str` at 0x407a8 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0x407ac | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x407b8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x407cc | store | 4 | `sp -0x24` | `ret(0x41224)` |
| `ldr` at 0x407d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x407dc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x407e0 | store | 4 | `sp -0x30` | `ret(0x41224)` |
| `ldrb` at 0x407e8 | load | 1 | `sp -0x1e` | `-` |
| `strb` at 0x407ec | store | 1 | `sp -0x20` | `([sp-30]:1 & 0xff)` |
| `ldrb` at 0x407f0 | load | 1 | `sp -0x20` | `-` |
| `ldrb` at 0x407f4 | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x40800 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x40804 | load | 1 | `[sp-44] +0x4` | `-` |
| `ldrb` at 0x40808 | load | 1 | `sp -0x20` | `-` |
| `ldr` at 0x40814 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x40818 | load | 1 | `sp -0x20` | `-` |
| `ldr` at 0x4081c | load | 4 | `[sp-44] +0x8` | `-` |
| `ldrb` at 0x40824 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x40834 | load | 1 | `sp -0x20` | `-` |
| `ldr` at 0x40838 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x40848 | load | 1 | `ret(0x40d44) +0x0` | `-` |
| `ldr` at 0x4084c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4085c | store | 4 | `sp -0x1c` | `ret(0x40e84)` |
| `ldr` at 0x40860 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x40864 | load | 1 | `ret(0x40e84) +0x3` | `-` |
| `ldrb` at 0x40870 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x40880 | load | 4 | `abs 0x40968` | `-` |
| `str` at 0x40884 | store | 4 | `sp -0x30` | `0xfffffa87` |
| `ldrb` at 0x4088c | load | 1 | `sp -0x16` | `-` |
| `str` at 0x40890 | store | 4 | `sp -0x34` | `[sp-22]:1` |
| `ldr` at 0x40894 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x408a0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x408ac | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x408bc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x408cc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x408d0 | load | 1 | `ret(0x40e84) +0x7` | `-` |
| `strb` at 0x408d4 | store | 1 | `sp -0x1d` | `([ret(0x40e84)+7]:1 & 0xff)` |
| `ldr` at 0x408dc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x408e0 | load | 1 | `ret(0x40e84) +0x8` | `-` |
| `strb` at 0x408e4 | store | 1 | `sp -0x1d` | `([ret(0x40e84)+8]:1 & 0xff)` |
| `ldr` at 0x408ec | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x408f0 | load | 1 | `ret(0x40e84) +0x9` | `-` |
| `strb` at 0x408f4 | store | 1 | `sp -0x1d` | `([ret(0x40e84)+9]:1 & 0xff)` |
| `ldrb` at 0x408fc | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x40900 | store | 1 | `sp -0x1d` | `([sp-22]:1 & 0xff)` |
| `ldrb` at 0x40904 | load | 1 | `sp -0x20` | `-` |
| `ldrb` at 0x40908 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x4090c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4091c | store | 4 | `sp -0x28` | `ret(0x404c4)` |
| `ldr` at 0x40920 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4092c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x40938 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x4093c | store | 4 | `sp -0x24` | `ret(0x404c4)` |
| `ldrb` at 0x40940 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0x40948 | store | 1 | `sp -0x20` | `?` |
| `ldr` at 0x40950 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x40954 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x40958 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x40960 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x40960 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x40960 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x407c4 | `(unknown callee) 0x41224` | `r0` | `(r1 & 0xff)` | `sp-30` | `sp-31` |
| `bl` at 0x40840 | `(unknown callee) 0x40d44` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x40854 | `(unknown callee) 0x40e84` | `[sp-20]` | `[ret(0x40d44)]:1` | `?` | `[ret(0x40d44)]:1` |
| `bl` at 0x40914 | `(unknown callee) 0x404c4` | `[sp-20]` | `[sp-32]:1` | `?` | `[sp-32]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x407d8 | `ret(0x41224) == 0x0` | `ret(0x41224) != 0x0` |
| `bhi` at 0x407fc | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bls` at 0x40810 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x4082c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x4086c | `[ret(0x40e84)+3]:1 != 0xff` | `[ret(0x40e84)+3]:1 == 0xff` |
| `bne` at 0x40878 | `[sp-21]:1 != 0xff` | `[sp-21]:1 == 0xff` |
| `beq` at 0x4089c | `[sp-22]:1 == 0x1` | `[sp-22]:1 != 0x1` |
| `bgt` at 0x408a8 | `[sp-22]:1 > 0x1 (signed, 32-bit)` | `[sp-22]:1 <= 0x1 (signed, 32-bit)` |
| `beq` at 0x408b4 | `[sp-22]:1 == 0x0` | `[sp-22]:1 != 0x0` |
| `beq` at 0x408c4 | `[sp-22]:1 == 0x2` | `[sp-22]:1 != 0x2` |
| `beq` at 0x40928 | `ret(0x404c4) == 0x0` | `ret(0x404c4) != 0x0` |
| `bne` at 0x40934 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x40960 | `r0` | `?` |

## `fanGetSpeed`

- `fanGetSpeed`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4096c to 0x40afc.
  Established. A nonzero `ret(0x3f2f4)` takes `0x409cc`, while zero falls through (`bne` at 0x409bc). The unnamed `0xc0e08` call carries `[sp-20]`, `[ret(0x40e84)+3]:1`,
  `[ret(0x40e84)+4]:1`, `[sp-28]` (`bl` at 0x40ab0). An included byte assignment through `[sp-28] +0x0` writes zero (`strb` at 0x40ac8). Fan speed encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4097c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x40984 | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0x40988 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x4098c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x40990 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x409a0 | store | 4 | `sp -0x20` | `ret(0x40d44)` |
| `ldrb` at 0x409a4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x409a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x409c0 | load | 4 | `abs 0x40af0` | `-` |
| `str` at 0x409c4 | store | 4 | `sp -0x2c` | `0xfffffa85` |
| `ldr` at 0x409cc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x409d0 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x409d4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x409e4 | store | 4 | `sp -0x24` | `ret(0x40e84)` |
| `ldr` at 0x409e8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x409ec | load | 1 | `ret(0x40e84) +0x0` | `-` |
| `str` at 0x409f0 | store | 4 | `sp -0x30` | `[ret(0x40e84)]:1` |
| `ldr` at 0x409f4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x40a00 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x40a0c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x40a1c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x40a2c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x40a30 | load | 1 | `ret(0x40e84) +0x3` | `-` |
| `ldr` at 0x40a3c | load | 4 | `abs 0x40af4` | `-` |
| `str` at 0x40a40 | store | 4 | `sp -0x2c` | `0xfffffa87` |
| `ldr` at 0x40a48 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x40a4c | load | 1 | `ret(0x40e84) +0x3` | `-` |
| `ldr` at 0x40a50 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x40a60 | store | 1 | `sp -0x25` | `(ret(0xca7a0) & 0xff)` |
| `ldr` at 0x40a64 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x40a68 | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0x40a70 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x40a74 | load | 1 | `[sp-36] +0x5` | `-` |
| `ldr` at 0x40a7c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x40a80 | load | 1 | `[sp-36] +0x4` | `-` |
| `strb` at 0x40a88 | store | 1 | `[sp-28] +0x0` | `?` |
| `ldr` at 0x40a90 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x40a94 | load | 1 | `ret(0x40e84) +0x3` | `-` |
| `ldr` at 0x40a98 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x40a9c | load | 1 | `ret(0x40e84) +0x4` | `-` |
| `ldr` at 0x40aa0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x40aac | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x40ab8 | store | 4 | `sp -0x2c` | `ret(0xc0e08)` |
| `ldr` at 0x40ac0 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x40ac8 | store | 1 | `[sp-28] +0x0` | `0x0` |
| `ldr` at 0x40ad0 | load | 4 | `abs 0x40af8` | `-` |
| `str` at 0x40ad4 | store | 4 | `sp -0x2c` | `0xfffffa86` |
| `str` at 0x40ae0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x40ae4 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x40aec | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x40aec | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x40aec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x40998 | `(unknown callee) 0x40d44` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x409b0 | `(unknown callee) 0x3f2f4` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x409dc | `(unknown callee) 0x40e84` | `[sp-20]` | `[[sp-32]]:1` | `?` | `[[sp-32]]:1` |
| `bl` at 0x40a58 | `(unknown callee) 0xca7a0` | `[sp-20]` | `[ret(0x40e84)+3]:1` | `?` | `[ret(0x40e84)+3]:1` |
| `bl` at 0x40ab0 | `(unknown callee) 0xc0e08` | `[sp-20]` | `[ret(0x40e84)+3]:1` | `[ret(0x40e84)+4]:1` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x409bc | `ret(0x3f2f4) != 0x0` | `ret(0x3f2f4) == 0x0` |
| `beq` at 0x409fc | `[ret(0x40e84)]:1 == 0x2` | `[ret(0x40e84)]:1 != 0x2` |
| `bgt` at 0x40a08 | `[ret(0x40e84)]:1 > 0x2 (signed, 32-bit)` | `[ret(0x40e84)]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0x40a14 | `[ret(0x40e84)]:1 == 0x1` | `[ret(0x40e84)]:1 != 0x1` |
| `beq` at 0x40a24 | `[ret(0x40e84)]:1 == 0x63` | `[ret(0x40e84)]:1 != 0x63` |
| `bne` at 0x40a38 | `[ret(0x40e84)+3]:1 != 0xff` | `[ret(0x40e84)+3]:1 == 0xff` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x40aec | `r0` | `?` |

## `fanGetReading`

- `fanGetReading`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x40afc to 0x40cd8.
  Established. The unnamed `0xc0a10` call carries `[sp-20]`, `[[sp-40]+1]:1`, `[[sp-40]+2]:1`,
  `[sp-28]` (`bl` at 0x40bf0). The coprocessor store at `sp -0x40` has unknown width and value
  (`stc` at 0x40c40). Included word assignments through `[sp-52] +0x0` write unknown values (`str` at 0x40c60, `str` at 0x40c7c). Fan reading conversion remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x40b0c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x40b14 | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0x40b18 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x40b1c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x40b20 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x40b30 | store | 4 | `sp -0x28` | `ret(0x40d44)` |
| `ldrb` at 0x40b34 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x40b38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x40b50 | load | 4 | `abs 0x40cc4` | `-` |
| `str` at 0x40b54 | store | 4 | `sp -0x2c` | `0xfffffa85` |
| `ldr` at 0x40b5c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x40b60 | load | 1 | `[sp-40] +0x8` | `-` |
| `str` at 0x40b68 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x40b6c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x40b78 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x40b84 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x40b94 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x40ba4 | load | 4 | `abs 0x40cc8` | `-` |
| `str` at 0x40ba8 | store | 4 | `sp -0x24` | `0x3f000000` |
| `ldr` at 0x40bb0 | load | 4 | `abs 0x40ccc` | `-` |
| `str` at 0x40bb4 | store | 4 | `sp -0x24` | `0x3f800000` |
| `ldr` at 0x40bbc | load | 4 | `abs 0x40cd0` | `-` |
| `str` at 0x40bc0 | store | 4 | `sp -0x24` | `0x40000000` |
| `ldr` at 0x40bc8 | load | 4 | `abs 0x40cd4` | `-` |
| `str` at 0x40bcc | store | 4 | `sp -0x24` | `0x40800000` |
| `ldr` at 0x40bd0 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x40bd4 | load | 1 | `[sp-40] +0x1` | `-` |
| `ldr` at 0x40bd8 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x40bdc | load | 1 | `[sp-40] +0x2` | `-` |
| `ldr` at 0x40be0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x40bec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x40bf8 | store | 4 | `sp -0x20` | `ret(0xc0a10)` |
| `ldr` at 0x40bfc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x40c00 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0x40c04 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x40c08 | load | 4 | `[sp-28] +0x0` | `-` |
| `stc` at 0x40c10 | store | unknown | `sp -0x3c` | `?` |
| `ldr` at 0x40c14 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldc` at 0x40c20 | load | unknown | `abs 0x40cb8` | `-` |
| `ldc` at 0x40c24 | load | unknown | `sp -0x3c` | `-` |
| `stc` at 0x40c2c | store | unknown | `sp -0x3c` | `?` |
| `ldc` at 0x40c30 | load | unknown | `sp -0x3c` | `-` |
| `ldc` at 0x40c38 | load | unknown | `sp -0x24` | `-` |
| `stc` at 0x40c40 | store | unknown | `sp -0x40` | `?` |
| `ldc` at 0x40c44 | load | unknown | `abs 0x40cc0` | `-` |
| `ldc` at 0x40c48 | load | unknown | `sp -0x40` | `-` |
| `ldc` at 0x40c54 | load | unknown | `sp -0x40` | `-` |
| `ldr` at 0x40c5c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x40c60 | store | 4 | `[sp-52] +0x0` | `?` |
| `ldc` at 0x40c68 | load | unknown | `abs 0x40cc0` | `-` |
| `ldc` at 0x40c6c | load | unknown | `sp -0x40` | `-` |
| `ldr` at 0x40c78 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x40c7c | store | 4 | `[sp-52] +0x0` | `?` |
| `ldr` at 0x40c80 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x40c84 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x40c8c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x40c90 | store | 4 | `[sp-52] +0x0` | `?` |
| `ldr` at 0x40c94 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x40c98 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x40c9c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x40ca0 | store | 4 | `[sp-52] +0x0` | `[[sp-52]]` |
| `ldr` at 0x40ca4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x40ca8 | store | 4 | `sp -0x2c` | `[sp-32]` |
| `ldr` at 0x40cac | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x40cb4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x40cb4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x40cb4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x40b28 | `(unknown callee) 0x40d44` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x40b40 | `(unknown callee) 0x3f2f4` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x40bf0 | `(unknown callee) 0xc0a10` | `[sp-20]` | `[[sp-40]+1]:1` | `[[sp-40]+2]:1` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x40b4c | `ret(0x3f2f4) != 0x0` | `ret(0x3f2f4) == 0x0` |
| `beq` at 0x40b74 | `[sp-48] == 0x4` | `[sp-48] != 0x4` |
| `bgt` at 0x40b80 | `[sp-48] > 0x4 (signed, 32-bit)` | `[sp-48] <= 0x4 (signed, 32-bit)` |
| `beq` at 0x40b8c | `[sp-48] == 0x1` | `[sp-48] != 0x1` |
| `beq` at 0x40b9c | `[sp-48] == 0x8` | `[sp-48] != 0x8` |
| `bge` at 0x40c1c | `[[sp-28]] >= 0x0 (signed, 32-bit)` | `[[sp-28]] < 0x0 (signed, 32-bit)` |
| `bge` at 0x40c50 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x40cb4 | `r0` | `?` |

## `EnvFanInt`

- `EnvFanInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7294c to 0x72c2c.
  Established. The absolute word at `0x13321c` is read (`ldr` at 0x7297c). The byte through `[sp-40] +0x1` receives `([[sp-40]+1]:1+1 & 0xff)` (`strb` at 0x72b30). Included byte assignments through
  `[sp-40] +0x2` write `0x1` and `0x0` in separate paths (`strb` at 0x72bc8, `strb` at 0x72be8). Fan
  interrupt policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7295c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x72964 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x7296c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x72974 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x72978 | load | 4 | `abs 0x72c00` | `-` |
| `ldr` at 0x7297c | load | 4 | `abs 0x13321c` | `-` |
| `ldr` at 0x72980 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7298c | load | 4 | `abs 0x72c00` | `-` |
| `ldr` at 0x72990 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x729a4 | load | 4 | `abs 0x133228` | `-` |
| `str` at 0x729ac | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x729b0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x729b4 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x729b8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x729c4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x729c8 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x729cc | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x729dc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x729e4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x729ec | load | 4 | `abs 0x72c00` | `-` |
| `ldr` at 0x729f0 | load | 4 | `abs 0x13321c` | `-` |
| `ldr` at 0x729f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x72a00 | load | 4 | `abs 0x72c04` | `-` |
| `str` at 0x72a04 | store | 4 | `sp -0x3c` | `0xfd68c` |
| `ldrb` at 0x72a08 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x72a0c | store | 4 | `sp -0x38` | `(r1 & 0xff)` |
| `ldrb` at 0x72a10 | load | 1 | `sp -0x16` | `-` |
| `str` at 0x72a14 | store | 4 | `sp -0x34` | `(r2 & 0xff)` |
| `ldr` at 0x72a1c | load | 4 | `abs 0x72c08` | `-` |
| `ldr` at 0x72a20 | load | 4 | `abs 0x72c0c` | `-` |
| `str` at 0x72a30 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x72a38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x72a4c | load | 4 | `abs 0x72c10` | `-` |
| `str` at 0x72a54 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x72a58 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x72a5c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x72a68 | load | 4 | `abs 0x72c00` | `-` |
| `ldr` at 0x72a6c | load | 4 | `abs 0x13320c` | `-` |
| `ldr` at 0x72a78 | load | 4 | `abs 0x72c00` | `-` |
| `ldr` at 0x72a7c | load | 4 | `abs 0x133210` | `-` |
| `ldr` at 0x72a88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x72aa0 | load | 4 | `abs 0x72c14` | `-` |
| `str` at 0x72aa4 | store | 4 | `sp -0x3c` | `0xfd6c0` |
| `str` at 0x72aa8 | store | 4 | `sp -0x38` | `ret(0x40cd8)` |
| `ldrb` at 0x72aac | load | 1 | `sp -0x15` | `-` |
| `str` at 0x72ab0 | store | 4 | `sp -0x34` | `[sp-21]:1` |
| `ldrb` at 0x72ab4 | load | 1 | `sp -0x16` | `-` |
| `str` at 0x72ab8 | store | 4 | `sp -0x30` | `[sp-22]:1` |
| `ldr` at 0x72ac0 | load | 4 | `abs 0x72c08` | `-` |
| `ldr` at 0x72ac4 | load | 4 | `abs 0x72c18` | `-` |
| `str` at 0x72ad4 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0x72af4 | store | 4 | `sp -0x20` | `0x1e` |
| `str` at 0x72b14 | store | 4 | `sp -0x20` | `0x28` |
| `str` at 0x72b20 | store | 4 | `sp -0x20` | `0xf` |
| `ldr` at 0x72b24 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x72b28 | load | 1 | `[sp-40] +0x1` | `-` |
| `strb` at 0x72b30 | store | 1 | `[sp-40] +0x1` | `([[sp-40]+1]:1+1 & 0xff)` |
| `ldr` at 0x72b38 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x72b48 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0x72b50 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x72b54 | load | 1 | `[sp-36] +0x1` | `-` |
| `ldr` at 0x72b58 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x72b5c | load | 1 | `[sp-36] +0x2` | `-` |
| `ldr` at 0x72b70 | load | 4 | `abs 0x72c1c` | `-` |
| `ldr` at 0x72b74 | load | 4 | `abs 0x132f14` | `-` |
| `ldr` at 0x72b84 | load | 4 | `abs 0x72c20` | `-` |
| `ldr` at 0x72b88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x72b90 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x72b94 | load | 1 | `[sp-40] +0x2` | `-` |
| `ldr` at 0x72ba0 | load | 4 | `abs 0x72c1c` | `-` |
| `ldr` at 0x72ba4 | load | 4 | `abs 0x132f14` | `-` |
| `ldr` at 0x72bb4 | load | 4 | `abs 0x72c24` | `-` |
| `ldr` at 0x72bb8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x72bc0 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x72bc8 | store | 1 | `[sp-40] +0x2` | `0x1` |
| `ldr` at 0x72bcc | load | 4 | `abs 0x72c28` | `-` |
| `ldr` at 0x72be0 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x72be8 | store | 1 | `[sp-40] +0x2` | `0x0` |
| `str` at 0x72bf0 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0x72bf4 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x72bfc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x72bfc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x72bfc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x72a28 | `(unknown callee) 0x200a0` | `0x1` | `0xfd2d0` | `0x546` | `0x0` |
| `bl` at 0x72a98 | `(unknown callee) 0x40cd8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x72acc | `(unknown callee) 0x200a0` | `0x1` | `0xfd2d0` | `0x553` | `0x0` |
| `bl` at 0x72adc | `(unknown callee) 0x488cc` | `r0` | `?` | `?` | `?` |
| `bl` at 0x72afc | `(unknown callee) 0x488cc` | `ret(0x488cc)` | `?` | `?` | `?` |
| `bl` at 0x72b6c | `(unknown callee) 0xc12c0` | `0x0` | `[[sp-36]+1]:1` | `[[sp-36]+2]:1` | `[[sp-36]+2]:1` |
| `bl` at 0x72b8c | `(unknown callee) 0x221f0` | `0xfd6f0` | `[sp-28]` | `?` | `?` |
| `bl` at 0x72bbc | `(unknown callee) 0x221f0` | `0xfd704` | `[sp-28]` | `?` | `?` |
| `bl` at 0x72bd0 | `(unknown callee) 0x2105c` | `0x132fa4` | `?` | `[sp-40]` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0x72988 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x729c0 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x729d4 | `Z == 0` | `not (Z == 0)` |
| `bhi` at 0x729fc | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0x72a64 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x72a74 | `[0x13320c] != 0x0` | `[0x13320c] == 0x0` |
| `bne` at 0x72a84 | `[0x133210] != 0x0` | `[0x133210] == 0x0` |
| `bne` at 0x72aec | `? != 0x47` | `? == 0x47` |
| `bne` at 0x72b0c | `? != 0x4d` | `? == 0x4d` |
| `bge` at 0x72b40 | `? >= [sp-32] (signed, 32-bit)` | `? < [sp-32] (signed, 32-bit)` |
| `beq` at 0x72b80 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x72b9c | `[[sp-40]+2]:1 != 0x0` | `[[sp-40]+2]:1 == 0x0` |
| `beq` at 0x72bb0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x72bdc | `ret(0x2105c) == 0x0` | `ret(0x2105c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x72bfc | `r0` | `?` |

## `EnvPowerInt`

- `EnvPowerInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x726e8 to 0x7294c.
  Established. The absolute word at `0x133218` is read (`ldr` at 0x72718). The unnamed `0x39b0c`
  call at `0x72830` carries `0x0`, `ret(0x39f8c)`, unknown, `ret(0x39f8c)` (`bl` at 0x72830). An
  included byte assignment through `[sp-36] +0x1` writes zero (`strb` at 0x72908). Power interrupt
  handling remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x726f8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x72700 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x72708 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x72710 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x72714 | load | 4 | `abs 0x72920` | `-` |
| `ldr` at 0x72718 | load | 4 | `abs 0x133218` | `-` |
| `ldr` at 0x7271c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x72728 | load | 4 | `abs 0x72920` | `-` |
| `ldr` at 0x7272c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x72740 | load | 4 | `abs 0x133224` | `-` |
| `str` at 0x72748 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x7274c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x72750 | load | 1 | `[sp-32] +0x4` | `-` |
| `ldrb` at 0x72754 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x72760 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x72764 | load | 1 | `[sp-32] +0x5` | `-` |
| `ldrb` at 0x72768 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x72778 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x72780 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x72788 | load | 4 | `abs 0x72920` | `-` |
| `ldr` at 0x7278c | load | 4 | `abs 0x133218` | `-` |
| `ldr` at 0x72790 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7279c | load | 4 | `abs 0x72924` | `-` |
| `str` at 0x727a0 | store | 4 | `sp -0x38` | `0xfd5f4` |
| `ldrb` at 0x727a4 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x727a8 | store | 4 | `sp -0x34` | `?` |
| `ldrb` at 0x727ac | load | 1 | `sp -0x16` | `-` |
| `str` at 0x727b0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x727b8 | load | 4 | `abs 0x72928` | `-` |
| `ldr` at 0x727bc | load | 4 | `abs 0x7292c` | `-` |
| `str` at 0x727cc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x727d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x727dc | load | 4 | `abs 0x72930` | `-` |
| `str` at 0x727e4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x727e8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x727ec | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x727f8 | load | 4 | `abs 0x72920` | `-` |
| `ldr` at 0x727fc | load | 4 | `abs 0x13320c` | `-` |
| `ldr` at 0x72808 | load | 4 | `abs 0x72920` | `-` |
| `ldr` at 0x7280c | load | 4 | `abs 0x133210` | `-` |
| `ldr` at 0x7281c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x72838 | load | 4 | `abs 0x72934` | `-` |
| `str` at 0x7283c | store | 4 | `sp -0x38` | `0xfd62c` |
| `str` at 0x72840 | store | 4 | `sp -0x34` | `ret(0x39b0c)` |
| `ldrb` at 0x72844 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x72848 | store | 4 | `sp -0x30` | `[sp-21]:1` |
| `ldrb` at 0x7284c | load | 1 | `sp -0x16` | `-` |
| `str` at 0x72850 | store | 4 | `sp -0x2c` | `[sp-22]:1` |
| `ldr` at 0x72858 | load | 4 | `abs 0x72928` | `-` |
| `ldr` at 0x7285c | load | 4 | `abs 0x72938` | `-` |
| `str` at 0x7286c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x72874 | load | 4 | `abs 0x7293c` | `-` |
| `ldr` at 0x72878 | load | 4 | `abs 0x132f14` | `-` |
| `ldr` at 0x7288c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x728a8 | load | 4 | `abs 0x72940` | `-` |
| `ldr` at 0x728b4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x728b8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x728c4 | load | 4 | `abs 0x7293c` | `-` |
| `ldr` at 0x728c8 | load | 4 | `abs 0x132f14` | `-` |
| `ldr` at 0x728d8 | load | 4 | `abs 0x72944` | `-` |
| `ldr` at 0x728e0 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x728e8 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x728ec | load | 4 | `abs 0x72948` | `-` |
| `ldr` at 0x72900 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x72908 | store | 1 | `[sp-36] +0x1` | `0x0` |
| `str` at 0x72910 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x72914 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x7291c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7291c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7291c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x727c4 | `(unknown callee) 0x200a0` | `0x1` | `0xfd2d0` | `0x507` | `0x0` |
| `bl` at 0x72820 | `(unknown callee) 0x39f8c` | `0x0` | `[sp-32]` | `?` | `[0x133210]` |
| `bl` at 0x72830 | `(unknown callee) 0x39b0c` | `0x0` | `ret(0x39f8c)` | `?` | `ret(0x39f8c)` |
| `bl` at 0x72864 | `(unknown callee) 0x200a0` | `0x1` | `0xfd2d0` | `0x514` | `0x0` |
| `bl` at 0x72890 | `(unknown callee) 0x39f8c` | `0x0` | `[sp-32]` | `?` | `?` |
| `bl` at 0x728a0 | `(unknown callee) 0x39b0c` | `0x0` | `ret(0x39f8c)` | `?` | `ret(0x39f8c)` |
| `bl` at 0x728b0 | `(unknown callee) 0x221f0` | `0xfd660` | `ret(0x39b0c)` | `?` | `ret(0x39b0c)` |
| `bl` at 0x728dc | `(unknown callee) 0x221f0` | `0xfd674` | `?` | `?` | `?` |
| `bl` at 0x728f0 | `(unknown callee) 0x2105c` | `0x132f50` | `?` | `?` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0x72724 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x7275c | `[[sp-32]+4]:1 != [sp-21]:1` | `[[sp-32]+4]:1 == [sp-21]:1` |
| `bne` at 0x72770 | `[[sp-32]+5]:1 != [sp-22]:1` | `[[sp-32]+5]:1 == [sp-22]:1` |
| `bhi` at 0x72798 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0x727f4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x72804 | `[0x13320c] != 0x0` | `[0x13320c] == 0x0` |
| `bne` at 0x72814 | `[0x133210] != 0x0` | `[0x133210] == 0x0` |
| `beq` at 0x72884 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x728c0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x728d4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x728fc | `ret(0x2105c) == 0x0` | `ret(0x2105c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7291c | `r0` | `?` |

## `EnvAutoFanControl`

- `EnvAutoFanControl`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x77050 to 0x77070.
  Established. The absolute word at `0x132f3c` receives `0x1` (`str` at 0x77064). The return reports
  entry `r0` (`ldm` at 0x77068). Automatic fan control effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x7705c | load | 4 | `abs 0x7706c` | `-` |
| `str` at 0x77064 | store | 4 | `abs 0x132f3c` | `0x1` |
| `ldm` at 0x77068 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x77068 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x77068 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x77068 | `r0` | `r0` |

## `EnvIgnoreLowPower`

- `EnvIgnoreLowPower`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7701c to 0x77050.
  Established. The word at `sp -0x14` receives entry `r0` (`str` at 0x7702c). An absolute word
  assignment at `0x132f38` has an unknown value (`str` at 0x77044). The return reports entry `r0`
  (`ldm` at 0x77048). Low-power override meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7702c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x77030 | load | 4 | `abs 0x7704c` | `-` |
| `ldr` at 0x77034 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x77044 | store | 4 | `abs 0x132f38` | `?` |
| `ldm` at 0x77048 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x77048 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x77048 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x77048 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x77048 | `r0` | `r0` |

## `AutofanEnvInit`

- `AutofanEnvInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x32e54 to 0x32ec0.
  Established. The unnamed `0x77050` call carries entry `r0`, `r1`, `r2`, `r3` (`bl` at 0x32e6c).
  Included words at `sp -0x2c` and `sp -0x20` receive `0x800` and `0x9` (`str` at 0x32e7c, `str` at 0x32e94). The unnamed `0x2066c` call carries `0x132118`, `0xf4900`, `0x3226c`, `[sp-20]` (`bl` at 0x32ea8). Environment task setup policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x32e64 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x32e68 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x32e70 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x32e74 | store | 4 | `sp -0x30` | `[sp-24]` |
| `str` at 0x32e7c | store | 4 | `sp -0x2c` | `0x800` |
| `str` at 0x32e84 | store | 4 | `sp -0x28` | `0xa` |
| `str` at 0x32e8c | store | 4 | `sp -0x24` | `0xa` |
| `str` at 0x32e94 | store | 4 | `sp -0x20` | `0x9` |
| `ldr` at 0x32e98 | load | 4 | `abs 0x32eb4` | `-` |
| `ldr` at 0x32e9c | load | 4 | `abs 0x32eb8` | `-` |
| `ldr` at 0x32ea0 | load | 4 | `abs 0x32ebc` | `-` |
| `ldr` at 0x32ea4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x32eb0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x32eb0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x32eb0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x32e6c | `(unknown callee) 0x77050` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x32ea8 | `(unknown callee) 0x2066c` | `0x132118` | `0xf4900` | `0x3226c` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x32eb0 | `r0` | `ret(0x2066c)` |

## `vsc055_FanPWMInit`

- `vsc055_FanPWMInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcd98c to 0xcda48.
  Established. The byte at `sp -0x29` initially receives zero (`strb` at 0xcd9b8). The unnamed
  `0xca3d0` call carries entry `r0`, `sp-32`, entry `r2`, `sp-32` (`bl` at 0xcd9c8). The unnamed
  `0xbb498` call carries unknown, `[sp-22]:1`, unknown, `[sp-41]:1` (`bl` at 0xcda2c). PWM
  initialization effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcd99c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcd9a0 | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0xcd9a8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xcd9b0 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0xcd9b8 | store | 1 | `sp -0x29` | `0x0` |
| `ldr` at 0xcd9c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcd9d0 | store | 4 | `sp -0x24` | `ret(0xca3d0)` |
| `ldr` at 0xcd9d4 | load | 4 | `sp +0x0` | `-` |
| `ldrb` at 0xcd9dc | load | 1 | `sp -0x29` | `-` |
| `strb` at 0xcd9e4 | store | 1 | `sp -0x29` | `?` |
| `ldrb` at 0xcd9e8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcd9ec | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xcda00 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xcda04 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcda1c | load | 1 | `sp -0x29` | `-` |
| `str` at 0xcda34 | store | 4 | `sp -0x28` | `ret(0xbb498)` |
| `ldr` at 0xcda38 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xcda44 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcda44 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcda44 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcd9c8 | `(unknown callee) 0xca3d0` | `r0` | `sp-32` | `r2` | `sp-32` |
| `bl` at 0xcd9f4 | `(unknown callee) 0xbac78` | `[sp-20]` | `[sp-21]:1` | `[sp-41]:1` | `[sp-21]:1` |
| `bl` at 0xcda2c | `(unknown callee) 0xbb498` | `?` | `[sp-22]:1` | `?` | `[sp-41]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcda44 | `r0` | `ret(0xbb498)` |

## `vsc055_FanMonitorInit`

- `vsc055_FanMonitorInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcda48 to 0xcdb5c.
  Established. The byte at `sp -0x2a` initially receives zero (`strb` at 0xcda74). The unnamed
  `0xbb498` call at `0xcdad0` carries unknown, `[sp-22]:1`, `[sp-41]:1`, `0xff` (`bl` at 0xcdad0);
  its later call at `0xcdb40` carries unknown, `[sp-22]:1`, `[sp-41]:1`, `[sp-42]:1` (`bl` at 0xcdb40). Fan monitor configuration meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcda58 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcda5c | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0xcda64 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xcda6c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0xcda74 | store | 1 | `sp -0x2a` | `0x0` |
| `ldr` at 0xcda7c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcda8c | store | 4 | `sp -0x24` | `ret(0xca3d0)` |
| `ldr` at 0xcda90 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xcda9c | store | 1 | `sp -0x29` | `?` |
| `ldrb` at 0xcdaa0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcdaa4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xcdab8 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0xcdabc | load | 1 | `sp -0x29` | `-` |
| `str` at 0xcdad8 | store | 4 | `sp -0x28` | `ret(0xbb498)` |
| `ldrb` at 0xcdadc | load | 1 | `sp -0x2a` | `-` |
| `strb` at 0xcdae8 | store | 1 | `sp -0x2a` | `?` |
| `ldr` at 0xcdaec | load | 4 | `sp +0x0` | `-` |
| `ldrb` at 0xcdaf0 | load | 1 | `sp -0x2a` | `-` |
| `strb` at 0xcdaf8 | store | 1 | `sp -0x2a` | `?` |
| `ldr` at 0xcdafc | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xcdb08 | store | 1 | `sp -0x29` | `?` |
| `ldrb` at 0xcdb0c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcdb10 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xcdb24 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0xcdb28 | load | 1 | `sp -0x29` | `-` |
| `ldrb` at 0xcdb2c | load | 1 | `sp -0x2a` | `-` |
| `str` at 0xcdb48 | store | 4 | `sp -0x28` | `ret(0xbb498)` |
| `ldr` at 0xcdb4c | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xcdb58 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcdb58 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcdb58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcda84 | `(unknown callee) 0xca3d0` | `r0` | `sp-32` | `r2` | `sp-32` |
| `bl` at 0xcdaac | `(unknown callee) 0xbac78` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0xcdad0 | `(unknown callee) 0xbb498` | `?` | `[sp-22]:1` | `[sp-41]:1` | `0xff` |
| `bl` at 0xcdb18 | `(unknown callee) 0xbac78` | `[sp-20]` | `[sp-21]:1` | `[sp-42]:1` | `[sp-21]:1` |
| `bl` at 0xcdb40 | `(unknown callee) 0xbb498` | `?` | `[sp-22]:1` | `[sp-41]:1` | `[sp-42]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcdb58 | `r0` | `ret(0xbb498)` |
