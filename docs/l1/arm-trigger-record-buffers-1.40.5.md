# ARM trigger records and buffers in 1.40.5

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
`UtilPortStr` has two unresolved computed transfers; its access, call and return lists
cover known edges only. Unknown data values and instruction effects remain explicit
in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/trigger-record-buffers-facts.txt) retain all tool sections.

## `TRIGTimeStamp`

- `TRIGTimeStamp`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbdf7c to 0xbebe0.
  Established. The unnamed `0xc0f2c` call at `0xbe01c` carries `0x7`, `0x22`, entry `r2`, entry `r3`
  (`bl` at 0xbe01c). The subsequent unnamed `0xbebe0` call carries `0x7`, `0x11`, `[sp-20]`,
  `[sp-24]` (`bl` at 0xbe050); its nonzero result takes `0xbeaec`, while zero falls through (`bne` at 0xbe05c). Trigger timestamp selection remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbdf8c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbdf90 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xbdf94 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xbdf98 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xbdf9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbdfa4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbdfac | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbdfc0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbdfc8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbdfd0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbdfe4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbdfec | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbdff4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe014 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe018 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe024 | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xbe028 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe02c | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbe030 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe034 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbe038 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbe03c | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbe048 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe04c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe060 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe068 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbe070 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe078 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbe080 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe09c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe0a0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe0ac | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xbe0b0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe0b4 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbe0b8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe0bc | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbe0c0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbe0c4 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbe0d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe0d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe0e8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe0f0 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbe0f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe100 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbe108 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe11c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe124 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbe12c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe140 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe148 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbe150 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe164 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe16c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbe174 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe194 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe198 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe1a4 | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xbe1a8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe1ac | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbe1b0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe1b4 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbe1b8 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbe1bc | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbe1c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe1cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe1e0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe1e8 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbe1ec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe1f0 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbe1f4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe1f8 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbe1fc | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbe200 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbe20c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe210 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe224 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe22c | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbe230 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe234 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbe238 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe23c | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbe240 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbe244 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbe250 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe254 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe268 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe270 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbe274 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe278 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbe27c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe280 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbe284 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbe288 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbe294 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe298 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe2ac | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe2b4 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbe2b8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe2bc | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbe2c0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe2c4 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbe2c8 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbe2cc | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbe2d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe2dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe2f0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe2f8 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbe2fc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe300 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbe304 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe308 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbe30c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbe310 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbe31c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe320 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe334 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe33c | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbe340 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe344 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbe348 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe34c | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbe350 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbe354 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbe360 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe364 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe378 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe380 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbe384 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe388 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbe38c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe390 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbe394 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbe398 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbe3a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe3a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe3bc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe3c4 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbe3cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe3d4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbe3dc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe3f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe3f8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbe400 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe414 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe41c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbe424 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe438 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe440 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbe448 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe468 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe46c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe478 | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xbe47c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe480 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbe484 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe488 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbe48c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbe490 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbe49c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe4a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe4b4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe4bc | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbe4c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe4cc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbe4d4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe4f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe4f4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe500 | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xbe508 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe510 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbe518 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe534 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe538 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe544 | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xbe54c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe554 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbe55c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe578 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe57c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe588 | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xbe590 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe598 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbe5a0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe5bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe5c0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe5cc | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xbe5d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe5dc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbe5e4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe600 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe604 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe610 | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xbe618 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe620 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbe628 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe644 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe648 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe654 | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xbe65c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe664 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbe66c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe688 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe68c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe698 | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xbe6a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe6a8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbe6b0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe6cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe6d0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe6dc | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xbe6e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe6ec | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbe6f4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe710 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe714 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe720 | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xbe728 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe730 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbe738 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe754 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe758 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe764 | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xbe768 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe76c | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbe770 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe774 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbe778 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbe77c | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbe788 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe78c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe7a0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe7a8 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbe7b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe7b8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbe7c0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe7dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe7e0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe7ec | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xbe7f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe7fc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbe804 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe820 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe824 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe830 | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xbe834 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe838 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbe83c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe840 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbe844 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbe848 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbe854 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe858 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe86c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe874 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbe878 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe87c | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbe880 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe884 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbe888 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbe88c | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbe898 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe89c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe8b0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe8b8 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbe8bc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe8c0 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbe8c4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe8c8 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbe8cc | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbe8d0 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbe8dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe8e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe8f4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe8fc | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbe904 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe90c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbe914 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe930 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe934 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe940 | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xbe944 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe948 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbe94c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe950 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbe954 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbe958 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbe964 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe968 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe97c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe984 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbe98c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe994 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbe99c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe9b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe9bc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe9c8 | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xbe9d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe9d8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbe9e0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe9fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbea00 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbea0c | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xbea10 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbea14 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbea18 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbea1c | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbea20 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbea24 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbea30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbea34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbea48 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbea50 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbea54 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbea58 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbea5c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbea60 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbea64 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbea68 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbea74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbea78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbea8c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbea94 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbea98 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbea9c | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbeaa0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbeaa4 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbeaa8 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbeaac | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbeab8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbeabc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbead0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbead8 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbeae0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbeae8 | store | 4 | `sp -0x28` | `r2+1` |
| `ldr` at 0xbeaec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbeaf0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbeafc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbeb04 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbeb0c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xbeb10 | load | 4 | `sp -0x24` | `-` |
| `ldrsb` at 0xbeb14 | load | 1 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xbeb20 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbeb28 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbeb2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbeb34 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbeb3c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbeb50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbeb58 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbeb60 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbeb7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbeb80 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbeb8c | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xbeb90 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbeb94 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbeb98 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbeb9c | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbeba0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbeba4 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbebb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbebb4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbebc8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbebd0 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbebd4 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xbebdc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbebdc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbebdc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbe01c | `(unknown callee) 0xc0f2c` | `0x7` | `0x22` | `r2` | `r3` |
| `bl` at 0xbe050 | `(unknown callee) 0xbebe0` | `0x7` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbe0a4 | `(unknown callee) 0xc0f2c` | `0x6` | `0x22` | `r2` | `r3` |
| `bl` at 0xbe0d8 | `(unknown callee) 0xbebe0` | `0x6` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbe19c | `(unknown callee) 0xc0f2c` | `0x8` | `0x22` | `r2` | `r3` |
| `bl` at 0xbe1d0 | `(unknown callee) 0xbebe0` | `0x8` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbe214 | `(unknown callee) 0xbf7e4` | `0xa` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbe258 | `(unknown callee) 0xbf7e4` | `0xb` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbe29c | `(unknown callee) 0xbf7e4` | `0xc` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbe2e0 | `(unknown callee) 0xbf7e4` | `0xd` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbe324 | `(unknown callee) 0xbf7e4` | `0xe` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbe368 | `(unknown callee) 0xbf7e4` | `0xf` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbe3ac | `(unknown callee) 0xbf7e4` | `0x9` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbe470 | `(unknown callee) 0xc0f2c` | `0x5` | `0x22` | `r2` | `r3` |
| `bl` at 0xbe4a4 | `(unknown callee) 0xbebe0` | `0x5` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbe4f8 | `(unknown callee) 0xc0f2c` | `0x10` | `0x22` | `r2` | `r3` |
| `bl` at 0xbe53c | `(unknown callee) 0xc0f2c` | `0x14` | `0x22` | `r2` | `r3` |
| `bl` at 0xbe580 | `(unknown callee) 0xc0f2c` | `0x16` | `0x22` | `r2` | `r3` |
| `bl` at 0xbe5c4 | `(unknown callee) 0xc0f2c` | `0x18` | `0x22` | `r2` | `r3` |
| `bl` at 0xbe608 | `(unknown callee) 0xc0f2c` | `0x1a` | `0x22` | `r2` | `r3` |
| `bl` at 0xbe64c | `(unknown callee) 0xc0f2c` | `0x15` | `0x22` | `r2` | `r3` |
| `bl` at 0xbe690 | `(unknown callee) 0xc0f2c` | `0x17` | `0x22` | `r2` | `r3` |
| `bl` at 0xbe6d4 | `(unknown callee) 0xc0f2c` | `0x19` | `0x22` | `r2` | `r3` |
| `bl` at 0xbe718 | `(unknown callee) 0xc0f2c` | `0x1b` | `0x22` | `r2` | `r3` |
| `bl` at 0xbe75c | `(unknown callee) 0xc0f2c` | `0x7` | `0x23` | `r2` | `r3` |
| `bl` at 0xbe790 | `(unknown callee) 0xbebe0` | `0x7` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbe7e4 | `(unknown callee) 0xc0f2c` | `0x6` | `0x23` | `r2` | `r3` |
| `bl` at 0xbe828 | `(unknown callee) 0xc0f2c` | `0x8` | `0x23` | `r2` | `r3` |
| `bl` at 0xbe85c | `(unknown callee) 0xbebe0` | `0x8` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbe8a0 | `(unknown callee) 0xbf7e4` | `0xe` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbe8e4 | `(unknown callee) 0xbf7e4` | `0xf` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbe938 | `(unknown callee) 0xc0f2c` | `0x7` | `0x24` | `r2` | `r3` |
| `bl` at 0xbe96c | `(unknown callee) 0xbebe0` | `0x7` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbe9c0 | `(unknown callee) 0xc0f2c` | `0x6` | `0x24` | `r2` | `r3` |
| `bl` at 0xbea04 | `(unknown callee) 0xc0f2c` | `0x8` | `0x24` | `r2` | `r3` |
| `bl` at 0xbea38 | `(unknown callee) 0xbebe0` | `0x8` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbea7c | `(unknown callee) 0xbf7e4` | `0xe` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbeac0 | `(unknown callee) 0xbf7e4` | `0xf` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbeb84 | `(unknown callee) 0xc0f2c` | `0x4` | `0x22` | `[sp-28]` | `[sp-32]` |
| `bl` at 0xbebb8 | `(unknown callee) 0xbebe0` | `0x4` | `0x11` | `[sp-20]` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xbdfbc | `? == 0xc` | `? != 0xc` |
| `beq` at 0xbdfe0 | `? == 0xb` | `? != 0xb` |
| `beq` at 0xbe004 | `? == 0x9` | `? != 0x9` |
| `bne` at 0xbe05c | `ret(0xbebe0) != 0x0` | `ret(0xbebe0) == 0x0` |
| `bne` at 0xbe090 | `? != 0x7` | `? == 0x7` |
| `bne` at 0xbe0e4 | `ret(0xbebe0) != 0x0` | `ret(0xbebe0) == 0x0` |
| `beq` at 0xbe118 | `? == 0x17` | `? != 0x17` |
| `beq` at 0xbe13c | `? == 0x19` | `? != 0x19` |
| `beq` at 0xbe160 | `? == 0x1a` | `? != 0x1a` |
| `beq` at 0xbe184 | `? == 0x18` | `? != 0x18` |
| `bne` at 0xbe1dc | `ret(0xbebe0) != 0x0` | `ret(0xbebe0) == 0x0` |
| `bne` at 0xbe220 | `ret(0xbf7e4) != 0x0` | `ret(0xbf7e4) == 0x0` |
| `bne` at 0xbe264 | `ret(0xbf7e4) != 0x0` | `ret(0xbf7e4) == 0x0` |
| `bne` at 0xbe2a8 | `ret(0xbf7e4) != 0x0` | `ret(0xbf7e4) == 0x0` |
| `bne` at 0xbe2ec | `ret(0xbf7e4) != 0x0` | `ret(0xbf7e4) == 0x0` |
| `bne` at 0xbe330 | `ret(0xbf7e4) != 0x0` | `ret(0xbf7e4) == 0x0` |
| `bne` at 0xbe374 | `ret(0xbf7e4) != 0x0` | `ret(0xbf7e4) == 0x0` |
| `bne` at 0xbe3b8 | `ret(0xbf7e4) != 0x0` | `ret(0xbf7e4) == 0x0` |
| `beq` at 0xbe3ec | `? == 0x2` | `? != 0x2` |
| `beq` at 0xbe410 | `? == 0x3` | `? != 0x3` |
| `beq` at 0xbe434 | `? == 0x4` | `? != 0x4` |
| `beq` at 0xbe458 | `? == 0x5` | `? != 0x5` |
| `bne` at 0xbe4b0 | `ret(0xbebe0) != 0x0` | `ret(0xbebe0) == 0x0` |
| `bne` at 0xbe4e4 | `? != 0x1` | `? == 0x1` |
| `bne` at 0xbe528 | `? != 0xe` | `? == 0xe` |
| `bne` at 0xbe56c | `? != 0x10` | `? == 0x10` |
| `bne` at 0xbe5b0 | `? != 0x12` | `? == 0x12` |
| `bne` at 0xbe5f4 | `? != 0x14` | `? == 0x14` |
| `bne` at 0xbe638 | `? != 0xf` | `? == 0xf` |
| `bne` at 0xbe67c | `? != 0x11` | `? == 0x11` |
| `bne` at 0xbe6c0 | `? != 0x13` | `? == 0x13` |
| `bne` at 0xbe704 | `? != 0x15` | `? == 0x15` |
| `bne` at 0xbe748 | `? != 0xa` | `? == 0xa` |
| `bne` at 0xbe79c | `ret(0xbebe0) != 0x0` | `ret(0xbebe0) == 0x0` |
| `bne` at 0xbe7d0 | `? != 0x6` | `? == 0x6` |
| `bne` at 0xbe814 | `? != 0x16` | `? == 0x16` |
| `bne` at 0xbe868 | `ret(0xbebe0) != 0x0` | `ret(0xbebe0) == 0x0` |
| `bne` at 0xbe8ac | `ret(0xbf7e4) != 0x0` | `ret(0xbf7e4) == 0x0` |
| `bne` at 0xbe8f0 | `ret(0xbf7e4) != 0x0` | `ret(0xbf7e4) == 0x0` |
| `bne` at 0xbe924 | `? != 0xd` | `? == 0xd` |
| `bne` at 0xbe978 | `ret(0xbebe0) != 0x0` | `ret(0xbebe0) == 0x0` |
| `bne` at 0xbe9ac | `? != 0x8` | `? == 0x8` |
| `bne` at 0xbe9f0 | `? != 0x1b` | `? == 0x1b` |
| `bne` at 0xbea44 | `ret(0xbebe0) != 0x0` | `ret(0xbebe0) == 0x0` |
| `bne` at 0xbea88 | `ret(0xbf7e4) != 0x0` | `ret(0xbf7e4) == 0x0` |
| `bne` at 0xbeacc | `ret(0xbf7e4) != 0x0` | `ret(0xbf7e4) == 0x0` |
| `bhi` at 0xbeaf8 | `[sp-40] > [sp-28] (unsigned, 32-bit)` | `[sp-40] <= [sp-28] (unsigned, 32-bit)` |
| `blt` at 0xbeb1c | `? < 0x0 (signed, 32-bit)` | `? >= 0x0 (signed, 32-bit)` |
| `ble` at 0xbeb4c | `? <= 0x1b (signed, 32-bit)` | `? > 0x1b (signed, 32-bit)` |
| `bgt` at 0xbeb70 | `? > 0x24 (signed, 32-bit)` | `? <= 0x24 (signed, 32-bit)` |
| `bne` at 0xbebc4 | `ret(0xbebe0) != 0x0` | `ret(0xbebe0) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbebdc | `r0` | `[sp-40]` |

## `TRIGSetInFIRST`

- `TRIGSetInFIRST`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbebe0 to 0xbf7e4.
  Established. The word at `sp -0x28` receives `0x1` (`str` at 0xbec04). The unnamed `0xc0f2c` call
  at `0xbecb8` carries entry `r0`, `0x12`, `[sp]`, `[sp+4]` (`bl` at 0xbecb8); its unsigned result
  above `[sp]` takes `0xbed34`, while at most `[sp]` falls through (`bhi` at 0xbecd0). Trigger
  first-input matching remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbebf0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbebf4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xbebf8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xbebfc | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xbec04 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0xbec08 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbec10 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbec18 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbec34 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbec3c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbec50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbec5c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbec64 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbec6c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbec88 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xbec90 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbec9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbeca8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbecb0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbecb4 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbecc0 | store | 4 | `sp -0x24` | `ret(0xc0f2c)` |
| `ldr` at 0xbecc4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbecc8 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbecd4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbecdc | load | 4 | `sp +0x8` | `-` |
| `str` at 0xbece4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbece8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbecf0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbecf8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbed04 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbed0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbed14 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbed20 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbed30 | store | 4 | `sp -0x28` | `ret(0xbdc38)` |
| `ldr` at 0xbed34 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbed3c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbed44 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbed58 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbed60 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbed68 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbed84 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbed88 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbed94 | store | 4 | `sp -0x24` | `ret(0xc0f2c)` |
| `ldr` at 0xbed98 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbed9c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbeda8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbedb0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xbedb8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbedbc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbedc4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbedcc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbede0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbedf4 | store | 4 | `sp -0x28` | `ret(0xbdc38)` |
| `ldr` at 0xbedf8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbee00 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbee08 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbee24 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbee2c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbee48 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbee4c | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbee58 | store | 4 | `sp -0x24` | `ret(0xc0f2c)` |
| `ldr` at 0xbee5c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbee60 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbee6c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbee78 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbee80 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xbee88 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbee8c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbee94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbee9c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbeea8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbeeb0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbeeb8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbeec4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbeed4 | store | 4 | `sp -0x28` | `ret(0xbdc38)` |
| `ldr` at 0xbeedc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbeee4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbeeec | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbef00 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbef14 | store | 4 | `sp -0x28` | `ret(0xbdc38)` |
| `ldr` at 0xbef18 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbef20 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbef28 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbef44 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbef4c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbef68 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbef6c | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbef78 | store | 4 | `sp -0x24` | `ret(0xc0f2c)` |
| `ldr` at 0xbef7c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbef80 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbef8c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbef98 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbefa0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xbefa8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbefac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbefb4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbefbc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbefc8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbefd0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbefd8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbefe4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbeff4 | store | 4 | `sp -0x28` | `ret(0xbdc38)` |
| `ldr` at 0xbeffc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf004 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbf00c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbf020 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf034 | store | 4 | `sp -0x28` | `ret(0xbdc38)` |
| `ldr` at 0xbf038 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf040 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbf048 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf064 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbf06c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf088 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbf08c | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbf098 | store | 4 | `sp -0x24` | `ret(0xc0f2c)` |
| `ldr` at 0xbf09c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbf0a0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbf0ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbf0b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbf0c0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xbf0c8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbf0cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf0d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf0dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbf0e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf0f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf0f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbf104 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf114 | store | 4 | `sp -0x28` | `ret(0xbdc38)` |
| `ldr` at 0xbf11c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf124 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbf12c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbf140 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf154 | store | 4 | `sp -0x28` | `ret(0xbdc38)` |
| `ldr` at 0xbf158 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf160 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbf168 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf184 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbf18c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf1a8 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbf1ac | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbf1b8 | store | 4 | `sp -0x24` | `ret(0xc0f2c)` |
| `ldr` at 0xbf1bc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbf1c0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbf1cc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbf1d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbf1e0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xbf1e8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbf1ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf1f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf1fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbf208 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf210 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf218 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbf224 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf234 | store | 4 | `sp -0x28` | `ret(0xbdc38)` |
| `ldr` at 0xbf23c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf244 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbf24c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbf260 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf274 | store | 4 | `sp -0x28` | `ret(0xbdc38)` |
| `ldr` at 0xbf278 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf280 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbf288 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf2a4 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbf2ac | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf2c8 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbf2cc | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbf2d8 | store | 4 | `sp -0x24` | `ret(0xc0f2c)` |
| `ldr` at 0xbf2dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbf2e0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbf2ec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbf2f8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbf300 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xbf308 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbf30c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf314 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf31c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbf328 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf330 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf338 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbf344 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf354 | store | 4 | `sp -0x28` | `ret(0xbdc38)` |
| `ldr` at 0xbf35c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf364 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbf36c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbf380 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf394 | store | 4 | `sp -0x28` | `ret(0xbdc38)` |
| `ldr` at 0xbf398 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf3a0 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbf3a8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf3c4 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbf3cc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf3e8 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbf3ec | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbf3f8 | store | 4 | `sp -0x24` | `ret(0xc0f2c)` |
| `ldr` at 0xbf3fc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbf400 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbf40c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbf418 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbf420 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xbf428 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbf42c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf434 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf43c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbf448 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf450 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf458 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbf464 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf474 | store | 4 | `sp -0x28` | `ret(0xbdc38)` |
| `ldr` at 0xbf47c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf484 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbf48c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbf4a0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf4b4 | store | 4 | `sp -0x28` | `ret(0xbdc38)` |
| `ldr` at 0xbf4b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf4c0 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbf4c8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf4e4 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbf4ec | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf508 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbf50c | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbf518 | store | 4 | `sp -0x24` | `ret(0xc0f2c)` |
| `ldr` at 0xbf51c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbf520 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbf52c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbf538 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbf540 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xbf548 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbf54c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf554 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf55c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbf568 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf570 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf578 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbf584 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf594 | store | 4 | `sp -0x28` | `ret(0xbdc38)` |
| `ldr` at 0xbf59c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf5a4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbf5ac | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbf5c0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf5d4 | store | 4 | `sp -0x28` | `ret(0xbdc38)` |
| `ldr` at 0xbf5d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf5e0 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbf5e8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf604 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbf60c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf628 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbf62c | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbf638 | store | 4 | `sp -0x24` | `ret(0xc0f2c)` |
| `ldr` at 0xbf63c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbf640 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbf64c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbf658 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbf660 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xbf668 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbf66c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf674 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf67c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbf688 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf690 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf698 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbf6a4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf6b4 | store | 4 | `sp -0x28` | `ret(0xbdc38)` |
| `ldr` at 0xbf6bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf6c4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbf6cc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbf6e0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf6f4 | store | 4 | `sp -0x28` | `ret(0xbdc38)` |
| `ldr` at 0xbf6f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf700 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbf708 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf724 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbf72c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf748 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbf74c | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbf758 | store | 4 | `sp -0x24` | `ret(0xc0f2c)` |
| `ldr` at 0xbf75c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbf760 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbf76c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbf778 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbf780 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xbf788 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbf78c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf794 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf79c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbf7a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf7b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf7b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbf7c4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf7d4 | store | 4 | `sp -0x28` | `ret(0xbdc38)` |
| `ldr` at 0xbf7d8 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xbf7e0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbf7e0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbf7e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbecb8 | `(unknown callee) 0xc0f2c` | `r0` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xbed28 | `(unknown callee) 0xbdc38` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xbed8c | `(unknown callee) 0xc0f2c` | `0x4` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xbedec | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x0` | `0x0` | `?` |
| `bl` at 0xbee50 | `(unknown callee) 0xc0f2c` | `?` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xbeecc | `(unknown callee) 0xbdc38` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xbef0c | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x1` | `0x1` | `?` |
| `bl` at 0xbef70 | `(unknown callee) 0xc0f2c` | `?` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xbefec | `(unknown callee) 0xbdc38` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xbf02c | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x2` | `0x2` | `?` |
| `bl` at 0xbf090 | `(unknown callee) 0xc0f2c` | `?` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xbf10c | `(unknown callee) 0xbdc38` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xbf14c | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x3` | `0x3` | `?` |
| `bl` at 0xbf1b0 | `(unknown callee) 0xc0f2c` | `?` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xbf22c | `(unknown callee) 0xbdc38` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xbf26c | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x4` | `0x4` | `?` |
| `bl` at 0xbf2d0 | `(unknown callee) 0xc0f2c` | `?` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xbf34c | `(unknown callee) 0xbdc38` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xbf38c | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x5` | `0x5` | `?` |
| `bl` at 0xbf3f0 | `(unknown callee) 0xc0f2c` | `?` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xbf46c | `(unknown callee) 0xbdc38` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xbf4ac | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x6` | `0x6` | `?` |
| `bl` at 0xbf510 | `(unknown callee) 0xc0f2c` | `?` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xbf58c | `(unknown callee) 0xbdc38` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xbf5cc | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x7` | `0x7` | `?` |
| `bl` at 0xbf630 | `(unknown callee) 0xc0f2c` | `?` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xbf6ac | `(unknown callee) 0xbdc38` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xbf6ec | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x8` | `0x8` | `?` |
| `bl` at 0xbf750 | `(unknown callee) 0xc0f2c` | `?` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xbf7cc | `(unknown callee) 0xbdc38` | `[sp-44]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xbec58 | `? != r0` | `? == r0` |
| `bne` at 0xbeca4 | `? != r1` | `? == r1` |
| `bhi` at 0xbecd0 | `ret(0xc0f2c) > [sp] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp] (unsigned, 32-bit)` |
| `ble` at 0xbed54 | `? <= 0x1b (signed, 32-bit)` | `? > 0x1b (signed, 32-bit)` |
| `bgt` at 0xbed78 | `? > 0x24 (signed, 32-bit)` | `? <= 0x24 (signed, 32-bit)` |
| `bhi` at 0xbeda4 | `ret(0xc0f2c) > [sp] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp] (unsigned, 32-bit)` |
| `bne` at 0xbeddc | `? != 0x1d` | `? == 0x1d` |
| `bhi` at 0xbee68 | `ret(0xc0f2c) > [sp] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp] (unsigned, 32-bit)` |
| `bne` at 0xbee74 | `[sp-40] != 0x1` | `[sp-40] == 0x1` |
| `bne` at 0xbeefc | `? != 0x1e` | `? == 0x1e` |
| `bhi` at 0xbef88 | `ret(0xc0f2c) > [sp] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp] (unsigned, 32-bit)` |
| `bne` at 0xbef94 | `[sp-40] != 0x1` | `[sp-40] == 0x1` |
| `bne` at 0xbf01c | `? != 0x1f` | `? == 0x1f` |
| `bhi` at 0xbf0a8 | `ret(0xc0f2c) > [sp] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp] (unsigned, 32-bit)` |
| `bne` at 0xbf0b4 | `[sp-40] != 0x1` | `[sp-40] == 0x1` |
| `bne` at 0xbf13c | `? != 0x20` | `? == 0x20` |
| `bhi` at 0xbf1c8 | `ret(0xc0f2c) > [sp] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp] (unsigned, 32-bit)` |
| `bne` at 0xbf1d4 | `[sp-40] != 0x1` | `[sp-40] == 0x1` |
| `bne` at 0xbf25c | `? != 0x21` | `? == 0x21` |
| `bhi` at 0xbf2e8 | `ret(0xc0f2c) > [sp] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp] (unsigned, 32-bit)` |
| `bne` at 0xbf2f4 | `[sp-40] != 0x1` | `[sp-40] == 0x1` |
| `bne` at 0xbf37c | `? != 0x22` | `? == 0x22` |
| `bhi` at 0xbf408 | `ret(0xc0f2c) > [sp] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp] (unsigned, 32-bit)` |
| `bne` at 0xbf414 | `[sp-40] != 0x1` | `[sp-40] == 0x1` |
| `bne` at 0xbf49c | `? != 0x23` | `? == 0x23` |
| `bhi` at 0xbf528 | `ret(0xc0f2c) > [sp] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp] (unsigned, 32-bit)` |
| `bne` at 0xbf534 | `[sp-40] != 0x1` | `[sp-40] == 0x1` |
| `bne` at 0xbf5bc | `? != 0x24` | `? == 0x24` |
| `bhi` at 0xbf648 | `ret(0xc0f2c) > [sp] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp] (unsigned, 32-bit)` |
| `bne` at 0xbf654 | `[sp-40] != 0x1` | `[sp-40] == 0x1` |
| `bne` at 0xbf6dc | `? != 0x1c` | `? == 0x1c` |
| `bhi` at 0xbf768 | `ret(0xc0f2c) > [sp] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp] (unsigned, 32-bit)` |
| `bne` at 0xbf774 | `[sp-40] != 0x1` | `[sp-40] == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbf7e0 | `r0` | `?` |

## `TRIGSetInXNFIRST`

- `TRIGSetInXNFIRST`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbf7e4 to 0xbfd9c.
  Established. The word at `sp -0x28` receives `0x130820` (`str` at 0xbf808). The unnamed `0xc0f2c`
  call at `0xbf8c4` carries entry `r0`, `0x12`, `[sp]`, `[sp+4]` (`bl` at 0xbf8c4); its unsigned
  result above `[sp]` takes `0xbf940`, while at most `[sp]` falls through (`bhi` at 0xbf8dc).
  Trigger input selection meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbf7f4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbf7f8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xbf7fc | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xbf800 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xbf804 | load | 4 | `abs 0xbfd98` | `-` |
| `str` at 0xbf808 | store | 4 | `sp -0x28` | `0x130820` |
| `str` at 0xbf810 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xbf814 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf81c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbf824 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf840 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbf848 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf85c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbf868 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf870 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbf878 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf894 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xbf89c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbf8a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbf8b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbf8bc | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbf8c0 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbf8cc | store | 4 | `sp -0x2c` | `ret(0xc0f2c)` |
| `ldr` at 0xbf8d0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbf8d4 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbf8e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbf8e8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xbf8f0 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xbf8f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf8fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf904 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbf910 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf918 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf920 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbf92c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xbf93c | store | 4 | `sp -0x30` | `ret(0xbdc38)` |
| `ldr` at 0xbf940 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xbf950 | store | 4 | `sp -0x24` | `0x80` |
| `ldr` at 0xbf954 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf95c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbf964 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf980 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbf988 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf9a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf9ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf9b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbf9d0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbf9d8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbf9dc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xbf9e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf9e8 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbf9f0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbfa0c | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbfa14 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbfa30 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbfa38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbfa40 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbfa5c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbfa64 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbfa68 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xbfa6c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbfa74 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbfa7c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbfa98 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbfaa0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbfabc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbfac4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbfacc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbfae8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbfaf0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbfaf4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xbfaf8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbfb00 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbfb08 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbfb24 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbfb2c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbfb48 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbfb50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbfb58 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbfb74 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbfb7c | load | 4 | `(unknown)` | `-` |
| `str` at 0xbfb80 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xbfb84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbfb8c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbfb94 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbfbb0 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbfbb8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbfbd4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbfbdc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbfbe4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbfc00 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbfc08 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbfc0c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xbfc10 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbfc18 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbfc20 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbfc3c | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbfc44 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbfc60 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbfc68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbfc70 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbfc8c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbfc94 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbfc98 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xbfc9c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbfca4 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbfcac | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbfcc8 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbfcd0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbfcec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbfcf4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbfcfc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbfd18 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbfd20 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbfd24 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xbfd28 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbfd3c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbfd40 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbfd4c | store | 4 | `sp -0x2c` | `ret(0xc0f2c)` |
| `ldr` at 0xbfd50 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbfd54 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbfd60 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbfd68 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xbfd70 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xbfd74 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbfd78 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbfd7c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbfd88 | store | 4 | `sp -0x30` | `ret(0xbdc38)` |
| `ldr` at 0xbfd8c | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xbfd94 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbfd94 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbfd94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbf8c4 | `(unknown callee) 0xc0f2c` | `r0` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xbf934 | `(unknown callee) 0xbdc38` | `[sp-52]` | `?` | `?` | `?` |
| `bl` at 0xbfd44 | `(unknown callee) 0xc0f2c` | `0x8` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xbfd80 | `(unknown callee) 0xbdc38` | `[sp-52]` | `[sp-36]` | `[sp-36]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xbf864 | `? != r0` | `? == r0` |
| `bne` at 0xbf8b0 | `? != r1` | `? == r1` |
| `bhi` at 0xbf8dc | `ret(0xc0f2c) > [sp] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp] (unsigned, 32-bit)` |
| `beq` at 0xbf948 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xbf9a0 | `? != 0xa` | `? == 0xa` |
| `bne` at 0xbfa2c | `? != 0xb` | `? == 0xb` |
| `bne` at 0xbfab8 | `? != 0xc` | `? == 0xc` |
| `bne` at 0xbfb44 | `? != 0xd` | `? == 0xd` |
| `bne` at 0xbfbd0 | `? != 0xe` | `? == 0xe` |
| `bne` at 0xbfc5c | `? != 0xf` | `? == 0xf` |
| `bne` at 0xbfce8 | `? != 0x9` | `? == 0x9` |
| `bhi` at 0xbfd30 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bhi` at 0xbfd5c | `ret(0xc0f2c) > [sp] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp] (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbfd94 | `r0` | `?` |

## `TRIGReadDetails`

- `TRIGReadDetails`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbfd9c to 0xc0dfc.
  Established. The word at `sp -0x24` receives `0xffff` on an included path (`str` at 0xbfdd4). The
  unnamed `0xbdc38` call at `0xbff38` carries `[sp-44]`, `0x2a`, `0x20`, unknown (`bl` at 0xbff38).
  Another included assignment at `sp -0x24` writes `0x77` (`str` at 0xc04a8). Trigger detail field
  meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbfdac | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbfdb0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xbfdb4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xbfdb8 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xbfdc0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xbfdc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbfdd0 | load | 4 | `abs 0xc0c68` | `-` |
| `str` at 0xbfdd4 | store | 4 | `sp -0x24` | `0xffff` |
| `ldr` at 0xbfdd8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbfde8 | store | 4 | `sp -0x24` | `0x3` |
| `ldr` at 0xbfdec | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbfdfc | store | 4 | `sp -0x24` | `0x7` |
| `ldr` at 0xbfe00 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbfe10 | store | 4 | `sp -0x24` | `0x3` |
| `ldr` at 0xbfe14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbfe20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbfe2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbfe38 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbfe4c | store | 4 | `sp -0x24` | `0x7f` |
| `ldr` at 0xbfe50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbfe58 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xbfe60 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbfe74 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbfe84 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xbfe88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbfe90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbfe98 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbfe9c | load | 4 | `abs 0xc0c6c` | `-` |
| `ldr` at 0xbfeb0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbfeb8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbfec0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbfec4 | load | 4 | `abs 0xc0c70` | `-` |
| `str` at 0xbfedc | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xbfee0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbfef4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbfef8 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbff04 | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xbff08 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbff0c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbff18 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbff20 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbff28 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbff2c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbff40 | store | 4 | `sp -0x24` | `ret(0xbdc38)` |
| `ldr` at 0xbff50 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbff54 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbff60 | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xbff64 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbff68 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbff74 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbff7c | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbff84 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbff88 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbff9c | store | 4 | `sp -0x24` | `ret(0xbdc38)` |
| `ldr` at 0xbffa0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbffa8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xbffb0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbffc4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbffcc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xbffd4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbffec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0000 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0004 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xc0010 | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xc0014 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0018 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0024 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc002c | load | 4 | `sp +0x4` | `-` |
| `str` at 0xc0034 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc0038 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc0054 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0068 | store | 4 | `sp -0x24` | `ret(0xbdc38)` |
| `ldr` at 0xc006c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0074 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc007c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc0090 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc00a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc00a8 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xc00b4 | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xc00b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc00bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc00c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc00d0 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xc00d8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc00dc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc00f0 | store | 4 | `sp -0x24` | `ret(0xbdc38)` |
| `ldr` at 0xc00f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc00fc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc0104 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc0118 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc012c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0130 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xc013c | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xc0140 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0144 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0150 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0158 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xc0160 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc0164 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0178 | store | 4 | `sp -0x24` | `ret(0xbdc38)` |
| `ldr` at 0xc017c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0184 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc018c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc01a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc01b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc01b8 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xc01c4 | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xc01c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc01cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc01d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc01e0 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xc01e8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc01ec | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0200 | store | 4 | `sp -0x24` | `ret(0xbdc38)` |
| `ldr` at 0xc0204 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc020c | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xc0214 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc0234 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc023c | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xc0244 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc0260 | load | 4 | `sp +0x0` | `-` |
| `ldrh` at 0xc0268 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc0288 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0290 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xc0298 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc02b4 | load | 4 | `sp +0x0` | `-` |
| `ldrh` at 0xc02bc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc02dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc02e4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc02ec | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc0308 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc030c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xc0318 | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xc031c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0320 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc032c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0334 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xc033c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc0340 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc035c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0368 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc037c | store | 4 | `sp -0x24` | `ret(0xbdc38)` |
| `ldr` at 0xc0380 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc038c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc03a0 | store | 4 | `sp -0x24` | `ret(0xbdc38)` |
| `ldr` at 0xc03a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc03b0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc03c4 | store | 4 | `sp -0x24` | `ret(0xbdc38)` |
| `ldr` at 0xc03c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc03d4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc03f4 | store | 4 | `sp -0x24` | `ret(0xc0dfc)` |
| `ldr` at 0xc0404 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0408 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xc0414 | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xc0418 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc041c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0428 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0430 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xc0438 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc043c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc0458 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0464 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0478 | store | 4 | `sp -0x24` | `ret(0xbdc38)` |
| `ldr` at 0xc047c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc0498 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc04a8 | store | 4 | `sp -0x24` | `0x77` |
| `ldr` at 0xc04ac | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc04bc | store | 4 | `sp -0x24` | `0x77` |
| `ldr` at 0xc04c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc04d0 | store | 4 | `sp -0x24` | `0x77` |
| `ldr` at 0xc04d4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc04e4 | store | 4 | `sp -0x24` | `0x77` |
| `ldr` at 0xc04ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc04f8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc050c | store | 4 | `sp -0x24` | `ret(0xbdc38)` |
| `ldr` at 0xc0510 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc051c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0530 | store | 4 | `sp -0x24` | `ret(0xbdc38)` |
| `ldr` at 0xc0534 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0540 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0560 | store | 4 | `sp -0x24` | `ret(0xc0dfc)` |
| `ldr` at 0xc0568 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0570 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc0578 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc0594 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0598 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xc05a4 | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xc05a8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc05ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc05b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc05c0 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xc05c8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc05cc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc05e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc05f4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0608 | store | 4 | `sp -0x24` | `ret(0xbdc38)` |
| `ldr` at 0xc060c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0618 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc062c | store | 4 | `sp -0x24` | `ret(0xbdc38)` |
| `ldr` at 0xc0630 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc063c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0650 | store | 4 | `sp -0x24` | `ret(0xbdc38)` |
| `ldr` at 0xc0660 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0664 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xc0670 | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xc0674 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0678 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0684 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc068c | load | 4 | `sp +0x4` | `-` |
| `str` at 0xc0694 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc0698 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc06b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc06c0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc06d4 | store | 4 | `sp -0x24` | `ret(0xbdc38)` |
| `ldr` at 0xc06d8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc06f4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc0704 | store | 4 | `sp -0x24` | `0x77` |
| `ldr` at 0xc0708 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc0718 | store | 4 | `sp -0x24` | `0x77` |
| `ldr` at 0xc071c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc072c | store | 4 | `sp -0x24` | `0x77` |
| `ldr` at 0xc0730 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc0740 | store | 4 | `sp -0x24` | `0x77` |
| `ldr` at 0xc0748 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0754 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0768 | store | 4 | `sp -0x24` | `ret(0xbdc38)` |
| `ldr` at 0xc076c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0778 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc078c | store | 4 | `sp -0x24` | `ret(0xbdc38)` |
| `ldr` at 0xc0790 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc07a0 | store | 4 | `sp -0x24` | `0x78` |
| `ldr` at 0xc07a4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc07b4 | store | 4 | `sp -0x24` | `0x78` |
| `ldr` at 0xc07b8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc07c8 | store | 4 | `sp -0x24` | `0x78` |
| `ldr` at 0xc07cc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc07dc | store | 4 | `sp -0x24` | `0x78` |
| `ldr` at 0xc07e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc07ec | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xc07f4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc0810 | load | 4 | `sp +0x0` | `-` |
| `ldrh` at 0xc0818 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc0838 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0840 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc0848 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc0864 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0868 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xc0874 | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xc0878 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc087c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0888 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0890 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xc0898 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc089c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc08a8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc08c0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xc08c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc08d0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc08e4 | store | 4 | `sp -0x24` | `ret(0xbdc38)` |
| `ldr` at 0xc08e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc08f4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0908 | store | 4 | `sp -0x24` | `ret(0xbdc38)` |
| `ldr` at 0xc0910 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0918 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc0920 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc093c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0940 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xc094c | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xc0950 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0954 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0960 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0968 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xc0970 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc0974 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0980 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0998 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xc099c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc09a8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc09bc | store | 4 | `sp -0x24` | `ret(0xbdc38)` |
| `ldr` at 0xc09c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc09cc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc09e0 | store | 4 | `sp -0x24` | `ret(0xbdc38)` |
| `ldr` at 0xc09e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc09f0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc09f8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc0a14 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0a18 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xc0a24 | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xc0a28 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0a2c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0a38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0a44 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0a4c | load | 4 | `sp +0x4` | `-` |
| `str` at 0xc0a54 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc0a58 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0a6c | store | 4 | `sp -0x24` | `ret(0xbdc38)` |
| `ldr` at 0xc0a70 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc0a78 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xc0a84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0a88 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xc0a94 | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xc0a98 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0a9c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0aa8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0ab0 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xc0ab8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc0abc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc0ad0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc0ad8 | store | 4 | `sp -0x24` | `([sp-36] + ret(0xbdc38))` |
| `ldr` at 0xc0ae4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0ae8 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xc0af4 | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xc0af8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0afc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0b08 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0b10 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xc0b18 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc0b1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0b28 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0b3c | store | 4 | `sp -0x24` | `ret(0xbdc38)` |
| `ldr` at 0xc0b40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0b4c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0b60 | store | 4 | `sp -0x24` | `ret(0xbdc38)` |
| `ldr` at 0xc0b6c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0b70 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xc0b7c | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xc0b80 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0b84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0b90 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0b98 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xc0ba0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc0ba4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0bb0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0bd0 | store | 4 | `sp -0x24` | `ret(0xc0dfc)` |
| `ldr` at 0xc0bd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0be0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0c00 | store | 4 | `sp -0x24` | `ret(0xc0dfc)` |
| `ldr` at 0xc0c04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0c10 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0c30 | store | 4 | `sp -0x24` | `ret(0xc0dfc)` |
| `ldr` at 0xc0c34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0c40 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0c60 | store | 4 | `sp -0x24` | `ret(0xc0dfc)` |
| `ldr` at 0xc0c74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0c7c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc0c84 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc0ca0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0ca4 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xc0cb0 | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xc0cb4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0cb8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0cc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0cd0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0cd8 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xc0ce0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc0ce4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0cf8 | store | 4 | `sp -0x24` | `ret(0xbdc38)` |
| `ldr` at 0xc0cfc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc0d04 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xc0d10 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0d14 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xc0d20 | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xc0d24 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0d28 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0d34 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0d3c | load | 4 | `sp +0x4` | `-` |
| `str` at 0xc0d44 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc0d48 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc0d5c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc0d64 | store | 4 | `sp -0x24` | `([sp-36] + ret(0xbdc38))` |
| `ldr` at 0xc0d70 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0d74 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xc0d80 | store | 4 | `sp -0x28` | `ret(0xc0f2c)` |
| `ldr` at 0xc0d84 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0d88 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0d94 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0d9c | load | 4 | `sp +0x4` | `-` |
| `str` at 0xc0da4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc0da8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0db4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0dc8 | store | 4 | `sp -0x24` | `ret(0xbdc38)` |
| `ldr` at 0xc0dcc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0dd8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0dec | store | 4 | `sp -0x24` | `ret(0xbdc38)` |
| `ldr` at 0xc0df0 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xc0df8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc0df8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc0df8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbfea0 | `(unknown callee) 0x1b088` | `?` | `0x12da08` | `?` | `?` |
| `bl` at 0xbfec8 | `(unknown callee) 0x1b088` | `?` | `0x12da14` | `?` | `?` |
| `bl` at 0xbfefc | `(unknown callee) 0xc0f2c` | `0x7` | `0x15` | `[sp-32]` | `[sp]` |
| `bl` at 0xbff38 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x2a` | `0x20` | `?` |
| `bl` at 0xbff58 | `(unknown callee) 0xc0f2c` | `0x7` | `0x14` | `[sp-32]` | `[sp]` |
| `bl` at 0xbff94 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x30` | `0x26` | `?` |
| `bl` at 0xc0008 | `(unknown callee) 0xc0f2c` | `0x5` | `0x14` | `?` | `[sp]` |
| `bl` at 0xc0044 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x3f` | `0x3f` | `?` |
| `bl` at 0xc0060 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x25` | `0x18` | `ret(0xbdc38)` |
| `bl` at 0xc00ac | `(unknown callee) 0xc0f2c` | `0xa` | `0x15` | `?` | `[sp]` |
| `bl` at 0xc00e8 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x1a` | `0x10` | `?` |
| `bl` at 0xc0134 | `(unknown callee) 0xc0f2c` | `0xc` | `0x15` | `?` | `[sp]` |
| `bl` at 0xc0170 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x1a` | `0x10` | `?` |
| `bl` at 0xc01bc | `(unknown callee) 0xc0f2c` | `0x9` | `0x15` | `?` | `[sp]` |
| `bl` at 0xc01f8 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x1a` | `0x10` | `?` |
| `bl` at 0xc0310 | `(unknown callee) 0xc0f2c` | `0x7` | `0x1a` | `?` | `[sp]` |
| `bl` at 0xc034c | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x37` | `0x30` | `?` |
| `bl` at 0xc0374 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x2d` | `0x23` | `[sp-20]` |
| `bl` at 0xc0398 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x4` | `0x4` | `[sp-20]` |
| `bl` at 0xc03bc | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x20` | `0x1f` | `[sp-20]` |
| `bl` at 0xc03e0 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x37` | `0x30` | `[sp-20]` |
| `bl` at 0xc03ec | `(unknown callee) 0xc0dfc` | `ret(0xbdc38)` | `?` | `?` | `ret(0xbdc38)` |
| `bl` at 0xc040c | `(unknown callee) 0xc0f2c` | `0x7` | `0x25` | `[sp-32]` | `[sp]` |
| `bl` at 0xc0448 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x3f` | `0x38` | `?` |
| `bl` at 0xc0470 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x30` | `0x26` | `[sp-20]` |
| `bl` at 0xc0488 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x25` | `0x20` | `?` |
| `bl` at 0xc0504 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x7` | `0x7` | `[sp-20]` |
| `bl` at 0xc0528 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x23` | `0x22` | `[sp-20]` |
| `bl` at 0xc054c | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x3a` | `0x33` | `[sp-20]` |
| `bl` at 0xc0558 | `(unknown callee) 0xc0dfc` | `ret(0xbdc38)` | `?` | `?` | `ret(0xbdc38)` |
| `bl` at 0xc059c | `(unknown callee) 0xc0f2c` | `0x7` | `0x1e` | `?` | `[sp]` |
| `bl` at 0xc05d8 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x37` | `0x30` | `?` |
| `bl` at 0xc0600 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x2d` | `0x23` | `[sp-20]` |
| `bl` at 0xc0624 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x4` | `0x4` | `[sp-20]` |
| `bl` at 0xc0648 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x20` | `0x1f` | `[sp-20]` |
| `bl` at 0xc0668 | `(unknown callee) 0xc0f2c` | `0x7` | `0x25` | `[sp-32]` | `[sp]` |
| `bl` at 0xc06a4 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x3f` | `0x38` | `?` |
| `bl` at 0xc06cc | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x30` | `0x26` | `[sp-20]` |
| `bl` at 0xc06e4 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x25` | `0x20` | `?` |
| `bl` at 0xc0760 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x7` | `0x7` | `[sp-20]` |
| `bl` at 0xc0784 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x23` | `0x22` | `[sp-20]` |
| `bl` at 0xc086c | `(unknown callee) 0xc0f2c` | `0x6` | `0x1a` | `?` | `[sp]` |
| `bl` at 0xc08b4 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x7` | `0x7` | `[sp-20]` |
| `bl` at 0xc08dc | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x23` | `0x22` | `[sp-20]` |
| `bl` at 0xc0900 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0xb` | `0xb` | `[sp-20]` |
| `bl` at 0xc0944 | `(unknown callee) 0xc0f2c` | `0x6` | `0x1e` | `?` | `[sp]` |
| `bl` at 0xc098c | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x7` | `0x7` | `[sp-20]` |
| `bl` at 0xc09b4 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x23` | `0x22` | `[sp-20]` |
| `bl` at 0xc09d8 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0xb` | `0xb` | `[sp-20]` |
| `bl` at 0xc0a1c | `(unknown callee) 0xc0f2c` | `0x8` | `0x1d` | `?` | `[sp]` |
| `bl` at 0xc0a64 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x4` | `0x0` | `?` |
| `bl` at 0xc0a8c | `(unknown callee) 0xc0f2c` | `0x8` | `0x1c` | `[sp-32]` | `[sp]` |
| `bl` at 0xc0ac8 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x3f` | `0x39` | `?` |
| `bl` at 0xc0aec | `(unknown callee) 0xc0f2c` | `0x8` | `0x1c` | `[sp-32]` | `[sp]` |
| `bl` at 0xc0b34 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x1a` | `0x1a` | `[sp-20]` |
| `bl` at 0xc0b58 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x36` | `0x35` | `[sp-20]` |
| `bl` at 0xc0b74 | `(unknown callee) 0xc0f2c` | `0x8` | `0x1a` | `[sp-32]` | `[sp]` |
| `bl` at 0xc0bbc | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x7` | `0x0` | `[sp-20]` |
| `bl` at 0xc0bc8 | `(unknown callee) 0xc0dfc` | `ret(0xbdc38)` | `?` | `?` | `ret(0xbdc38)` |
| `bl` at 0xc0bec | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x17` | `0x10` | `[sp-20]` |
| `bl` at 0xc0bf8 | `(unknown callee) 0xc0dfc` | `ret(0xbdc38)` | `?` | `?` | `ret(0xbdc38)` |
| `bl` at 0xc0c1c | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x27` | `0x20` | `[sp-20]` |
| `bl` at 0xc0c28 | `(unknown callee) 0xc0dfc` | `ret(0xbdc38)` | `?` | `?` | `ret(0xbdc38)` |
| `bl` at 0xc0c4c | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x37` | `0x30` | `[sp-20]` |
| `bl` at 0xc0c58 | `(unknown callee) 0xc0dfc` | `ret(0xbdc38)` | `?` | `?` | `ret(0xbdc38)` |
| `bl` at 0xc0ca8 | `(unknown callee) 0xc0f2c` | `0x8` | `0x21` | `?` | `[sp]` |
| `bl` at 0xc0cf0 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x4` | `0x0` | `?` |
| `bl` at 0xc0d18 | `(unknown callee) 0xc0f2c` | `0x8` | `0x20` | `[sp-32]` | `[sp]` |
| `bl` at 0xc0d54 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x3f` | `0x39` | `?` |
| `bl` at 0xc0d78 | `(unknown callee) 0xc0f2c` | `0x8` | `0x20` | `[sp-32]` | `[sp]` |
| `bl` at 0xc0dc0 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x1a` | `0x1a` | `[sp-20]` |
| `bl` at 0xc0de4 | `(unknown callee) 0xbdc38` | `[sp-44]` | `0x36` | `0x35` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xbfdcc | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xbfde0 | `r0 != 0x1` | `r0 == 0x1` |
| `bne` at 0xbfdf4 | `r0 != 0x2` | `r0 == 0x2` |
| `bne` at 0xbfe08 | `r0 != 0x7` | `r0 == 0x7` |
| `beq` at 0xbfe1c | `r0 == 0x3` | `r0 != 0x3` |
| `beq` at 0xbfe28 | `r0 == 0x4` | `r0 != 0x4` |
| `beq` at 0xbfe34 | `r0 == 0x5` | `r0 != 0x5` |
| `beq` at 0xbfe40 | `r0 == 0x6` | `r0 != 0x6` |
| `bne` at 0xbfe70 | `? != 0xb` | `? == 0xb` |
| `bne` at 0xbfe7c | `r0 != 0x0` | `r0 == 0x0` |
| `beq` at 0xbfeac | `ret(0x1b088) == 0x0` | `ret(0x1b088) != 0x0` |
| `bne` at 0xbfed4 | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |
| `bne` at 0xbfee8 | `Z == 0` | `not (Z == 0)` |
| `bhi` at 0xbff14 | `ret(0xc0f2c) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp-32] (unsigned, 32-bit)` |
| `bhi` at 0xbff70 | `ret(0xc0f2c) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp-32] (unsigned, 32-bit)` |
| `beq` at 0xbffc0 | `? == 0x5` | `? != 0x5` |
| `beq` at 0xbffe4 | `? == 0x4` | `? != 0x4` |
| `bne` at 0xbfff4 | `Z == 0` | `not (Z == 0)` |
| `bhi` at 0xc0020 | `ret(0xc0f2c) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp-32] (unsigned, 32-bit)` |
| `beq` at 0xc0050 | `ret(0xbdc38) == 0x0` | `ret(0xbdc38) != 0x0` |
| `bne` at 0xc008c | `? != 0x19` | `? == 0x19` |
| `bne` at 0xc0098 | `Z == 0` | `not (Z == 0)` |
| `bhi` at 0xc00c4 | `ret(0xc0f2c) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xc0114 | `? != 0x1a` | `? == 0x1a` |
| `bne` at 0xc0120 | `Z == 0` | `not (Z == 0)` |
| `bhi` at 0xc014c | `ret(0xc0f2c) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xc019c | `? != 0x18` | `? == 0x18` |
| `bne` at 0xc01a8 | `Z == 0` | `not (Z == 0)` |
| `bhi` at 0xc01d4 | `ret(0xc0f2c) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp-32] (unsigned, 32-bit)` |
| `beq` at 0xc0230 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xc0280 | `? != 0x4` | `? == 0x4` |
| `bne` at 0xc02d4 | `? != 0x8` | `? == 0x8` |
| `bne` at 0xc02fc | `? != 0xa` | `? == 0xa` |
| `bhi` at 0xc0328 | `ret(0xc0f2c) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp-32] (unsigned, 32-bit)` |
| `beq` at 0xc0358 | `ret(0xbdc38) == 0xff` | `ret(0xbdc38) != 0xff` |
| `bne` at 0xc0364 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bne` at 0xc0388 | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `bne` at 0xc03ac | `[sp-20] != 0x2` | `[sp-20] == 0x2` |
| `bne` at 0xc03d0 | `[sp-20] != 0x3` | `[sp-20] == 0x3` |
| `bhi` at 0xc0424 | `ret(0xc0f2c) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xc0454 | `ret(0xbdc38) != 0xaa` | `ret(0xbdc38) == 0xaa` |
| `bne` at 0xc0460 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bne` at 0xc0494 | `ret(0xbdc38) != 0x8` | `ret(0xbdc38) == 0x8` |
| `bne` at 0xc04a0 | `[sp-20] != 0x3` | `[sp-20] == 0x3` |
| `bne` at 0xc04b4 | `[sp-20] != 0x4` | `[sp-20] == 0x4` |
| `bne` at 0xc04c8 | `[sp-20] != 0x5` | `[sp-20] == 0x5` |
| `bne` at 0xc04dc | `[sp-20] != 0x6` | `[sp-20] == 0x6` |
| `bne` at 0xc04f4 | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `bne` at 0xc0518 | `[sp-20] != 0x2` | `[sp-20] == 0x2` |
| `bne` at 0xc053c | `[sp-20] != 0x3` | `[sp-20] == 0x3` |
| `bne` at 0xc0588 | `? != 0xd` | `? == 0xd` |
| `bhi` at 0xc05b4 | `ret(0xc0f2c) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp-32] (unsigned, 32-bit)` |
| `beq` at 0xc05e4 | `ret(0xbdc38) == 0xff` | `ret(0xbdc38) != 0xff` |
| `bne` at 0xc05f0 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bne` at 0xc0614 | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `bne` at 0xc0638 | `[sp-20] != 0x2` | `[sp-20] == 0x2` |
| `bhi` at 0xc0680 | `ret(0xc0f2c) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xc06b0 | `ret(0xbdc38) != 0xaa` | `ret(0xbdc38) == 0xaa` |
| `bne` at 0xc06bc | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bne` at 0xc06f0 | `ret(0xbdc38) != 0x8` | `ret(0xbdc38) == 0x8` |
| `bne` at 0xc06fc | `[sp-20] != 0x3` | `[sp-20] == 0x3` |
| `bne` at 0xc0710 | `[sp-20] != 0x4` | `[sp-20] == 0x4` |
| `bne` at 0xc0724 | `[sp-20] != 0x5` | `[sp-20] == 0x5` |
| `bne` at 0xc0738 | `[sp-20] != 0x6` | `[sp-20] == 0x6` |
| `bne` at 0xc0750 | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `bne` at 0xc0774 | `[sp-20] != 0x2` | `[sp-20] == 0x2` |
| `bne` at 0xc0798 | `[sp-20] != 0x3` | `[sp-20] == 0x3` |
| `bne` at 0xc07ac | `[sp-20] != 0x4` | `[sp-20] == 0x4` |
| `bne` at 0xc07c0 | `[sp-20] != 0x5` | `[sp-20] == 0x5` |
| `bne` at 0xc07d4 | `[sp-20] != 0x6` | `[sp-20] == 0x6` |
| `bne` at 0xc0830 | `? != 0x1c` | `? == 0x1c` |
| `bne` at 0xc0858 | `? != 0x6` | `? == 0x6` |
| `bhi` at 0xc0884 | `ret(0xc0f2c) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xc08a4 | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `bne` at 0xc08cc | `[sp-20] != 0x2` | `[sp-20] == 0x2` |
| `bne` at 0xc08f0 | `[sp-20] != 0x7` | `[sp-20] == 0x7` |
| `bne` at 0xc0930 | `? != 0x8` | `? == 0x8` |
| `bhi` at 0xc095c | `ret(0xc0f2c) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xc097c | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `bne` at 0xc09a4 | `[sp-20] != 0x2` | `[sp-20] == 0x2` |
| `bne` at 0xc09c8 | `[sp-20] != 0x7` | `[sp-20] == 0x7` |
| `bne` at 0xc0a08 | `? != 0x16` | `? == 0x16` |
| `bhi` at 0xc0a34 | `ret(0xc0f2c) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xc0a40 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bhi` at 0xc0aa4 | `ret(0xc0f2c) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp-32] (unsigned, 32-bit)` |
| `bhi` at 0xc0b04 | `ret(0xc0f2c) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xc0b24 | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `bne` at 0xc0b48 | `[sp-20] != 0x2` | `[sp-20] == 0x2` |
| `bhi` at 0xc0b8c | `ret(0xc0f2c) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xc0bac | `[sp-20] != 0x3` | `[sp-20] == 0x3` |
| `bne` at 0xc0bdc | `[sp-20] != 0x4` | `[sp-20] == 0x4` |
| `bne` at 0xc0c0c | `[sp-20] != 0x5` | `[sp-20] == 0x5` |
| `bne` at 0xc0c3c | `[sp-20] != 0x6` | `[sp-20] == 0x6` |
| `bne` at 0xc0c94 | `? != 0x1b` | `? == 0x1b` |
| `bhi` at 0xc0cc0 | `ret(0xc0f2c) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xc0ccc | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bhi` at 0xc0d30 | `ret(0xc0f2c) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp-32] (unsigned, 32-bit)` |
| `bhi` at 0xc0d90 | `ret(0xc0f2c) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0f2c) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xc0db0 | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `bne` at 0xc0dd4 | `[sp-20] != 0x2` | `[sp-20] == 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc0df8 | `r0` | `?` |

## `BuffWriteInit`

- `BuffWriteInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc5cd4 to 0xc5d24.
  Established. The unnamed `0x1b1b4` call carries `0xc`, entry `r1`, `r2`, `r3` (`bl` at 0xc5ce8).
  The word through `ret(0x1b1b4) +0x0` receives `0x0` (`str` at 0xc5cfc). Included word assignments
  at `[sp-20] +0x4` and `[sp-20] +0x8` also write `0x0` (`str` at 0xc5d08, `str` at 0xc5d14). Buffer
  record semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc5cf0 | store | 4 | `sp -0x14` | `ret(0x1b1b4)` |
| `ldr` at 0xc5cf4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc5cfc | store | 4 | `ret(0x1b1b4) +0x0` | `0x0` |
| `ldr` at 0xc5d00 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc5d08 | store | 4 | `[sp-20] +0x4` | `0x0` |
| `ldr` at 0xc5d0c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc5d14 | store | 4 | `[sp-20] +0x8` | `0x0` |
| `ldr` at 0xc5d18 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc5d20 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc5d20 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc5d20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc5ce8 | `(unknown callee) 0x1b1b4` | `0xc` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc5d20 | `r0` | `[sp-20]` |

## `BuffWriteDone`

- `BuffWriteDone`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc5d24 to 0xc5d90.
  Established. Entry `r1 == 0x0` takes `0xc5d70`, while nonzero falls through (`beq` at 0xc5d4c).
  The unnamed `0x1b490` call at `0xc5d68` carries `[r0]`, `r1`, `r2`, `r0` (`bl` at 0xc5d68). The
  return reports `[sp-28]` (`ldmdb` at 0xc5d8c). Buffer ownership policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc5d34 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc5d38 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc5d40 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xc5d44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc5d50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5d54 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0xc5d60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5d64 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0xc5d70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5d74 | load | 4 | `r0 +0x0` | `-` |
| `str` at 0xc5d78 | store | 4 | `sp -0x1c` | `[r0]` |
| `ldr` at 0xc5d7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5d84 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xc5d8c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc5d8c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc5d8c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc5d68 | `(unknown callee) 0x1b490` | `[r0]` | `r1` | `r2` | `r0` |
| `bl` at 0xc5d80 | `(unknown callee) 0x1b490` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc5d4c | `r1 == 0x0` | `r1 != 0x0` |
| `beq` at 0xc5d5c | `[r0] == 0x0` | `[r0] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc5d8c | `r0` | `[sp-28]` |

## `BuffWrite`

- `BuffWrite`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc5d90 to 0xc5edc.
  Established. Signed `[sp-32] <= 0x3ff` takes `0xc5df0`, while larger values fall through (`ble` at 0xc5ddc). The word at `[sp-36] +0x8` receives `0x0` on an included path (`str` at 0xc5e38). The
  unnamed `0x1b598` call carries `([[sp-36]] + [[sp-36]+8])`, `[sp-28]`, `[sp-36]`, `([[sp-36]] + [[sp-36]+8])` (`bl` at 0xc5eb0). Buffer growth semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc5da0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xc5da4 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xc5da8 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0xc5dac | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc5db0 | store | 4 | `sp -0x24` | `r0` |
| `ldr` at 0xc5db4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc5dc0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc5dcc | store | 4 | `sp -0x20` | `ret(0x1b2bc)` |
| `ldr` at 0xc5dd0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc5dd4 | load | 4 | `abs 0xc5ed8` | `-` |
| `ldr` at 0xc5de0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc5de8 | store | 4 | `sp -0x2c` | `[sp-32]+200` |
| `str` at 0xc5df4 | store | 4 | `sp -0x2c` | `0x400` |
| `ldr` at 0xc5df8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc5dfc | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xc5e00 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc5e04 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xc5e10 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc5e14 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc5e18 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc5e1c | store | 4 | `[sp-36] +0x4` | `?` |
| `str` at 0xc5e2c | store | 4 | `[sp-36] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xc5e30 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc5e38 | store | 4 | `[sp-36] +0x8` | `0x0` |
| `ldr` at 0xc5e40 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc5e44 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0xc5e48 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc5e50 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc5e54 | load | 4 | `[sp-36] +0x4` | `-` |
| `ldr` at 0xc5e60 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc5e64 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc5e68 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc5e6c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc5e70 | load | 4 | `[sp-36] +0x4` | `-` |
| `ldr` at 0xc5e74 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc5e7c | store | 4 | `[sp-36] +0x4` | `?` |
| `ldr` at 0xc5e80 | load | 4 | `[sp-36] +0x0` | `-` |
| `str` at 0xc5e90 | store | 4 | `[sp-36] +0x0` | `ret(0x1b358)` |
| `ldr` at 0xc5e94 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc5e98 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc5e9c | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xc5ea0 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0xc5eac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5eb4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc5eb8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc5ebc | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0xc5ec0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc5ec8 | store | 4 | `[sp-36] +0x8` | `([[sp-36]+8] + [sp-32])` |
| `ldr` at 0xc5ecc | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xc5ed4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc5ed4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc5ed4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc5ed4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc5dc4 | `(unknown callee) 0x1b2bc` | `[sp-28]` | `r1` | `r2` | `r2` |
| `bl` at 0xc5e24 | `(unknown callee) 0x1b1b4` | `?` | `?` | `[sp-36]` | `?` |
| `bl` at 0xc5e88 | `(unknown callee) 0x1b358` | `[[sp-36]]` | `?` | `[[sp-36]+4]` | `?` |
| `bl` at 0xc5eb0 | `(unknown callee) 0x1b598` | `([[sp-36]] + [[sp-36]+8])` | `[sp-28]` | `[sp-36]` | `([[sp-36]] + [[sp-36]+8])` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc5dbc | `Z == 0` | `not (Z == 0)` |
| `ble` at 0xc5ddc | `[sp-32] <= 0x3ff (signed, 32-bit)` | `[sp-32] > 0x3ff (signed, 32-bit)` |
| `bne` at 0xc5e0c | `[[sp-36]] != 0x0` | `[[sp-36]] == 0x0` |
| `ble` at 0xc5e5c | `([[sp-36]+8] + [sp-32]) <= [[sp-36]+4] (signed, 32-bit)` | `([[sp-36]+8] + [sp-32]) > [[sp-36]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc5ed4 | `r0` | `[sp-32]` |

## `print`

- `print`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc25b4 to 0xc25e4.
  Established. The unnamed `0x1b058` call carries entry `r0`, `r1`, `r2`, `r3` (`bl` at 0xc25dc).
  The return reports that call result (`ldmdb` at 0xc25e0). Output semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc25c4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc25c8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc25cc | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xc25d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc25d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc25d8 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xc25e0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc25e0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc25e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc25dc | `(unknown callee) 0x1b058` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc25e0 | `r0` | `ret(0x1b058)` |

## `w`

- `w`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc47d4 to 0xc4804.
  Established. The indirect call at `0xc47fc` has an unknown target and entry candidates `r2`, `r1`,
  `r2`, `r0` (`mov` at 0xc47fc); its unknown-target result is returned (`ldmdb` at 0xc4800).
  Callback role remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc47e4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc47e8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc47ec | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xc47f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc47f4 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xc4800 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc4800 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc4800 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0xc47fc | `(unknown callee) (unknown target)` | `r2` | `r1` | `r2` | `r0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc4800 | `r0` | `ret(unknown)` |

## `print_data`

- `print_data`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbddc4 to 0xbdf7c.
  Established. The unnamed `0x1b4cc` call carries `sp-36`, `0x0`, `0x8`, `sp-36` (`bl` at 0xbddf0).
  The later unnamed `0x1b52c` call carries `0x14ab78`, `0x1331a0`, `[sp-36]:1`, `[sp-35]:1` (`bl` at 0xbdf64). The return reports `0x14ab78` (`ldmdb` at 0xbdf70). Printed data interpretation remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbddd4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbddd8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xbdddc | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xbddf4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbddf8 | store | 4 | `sp -0x28` | `[sp-24]` |
| `ldr` at 0xbddfc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbde00 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbde08 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbde0c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbde10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbde20 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbde34 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbde3c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbde40 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbde68 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xbde88 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xbde8c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbde98 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbded4 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xbdef8 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xbdf00 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xbdf04 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xbdf0c | store | 4 | `sp -0x28` | `[sp-40]-1` |
| `ldr` at 0xbdf10 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbdf18 | store | 4 | `sp -0x2c` | `[sp-44]-1` |
| `ldrb` at 0xbdf20 | load | 1 | `sp -0x24` | `-` |
| `ldrb` at 0xbdf24 | load | 1 | `sp -0x23` | `-` |
| `ldrb` at 0xbdf28 | load | 1 | `sp -0x22` | `-` |
| `str` at 0xbdf2c | store | 4 | `sp -0x48` | `[sp-34]:1` |
| `ldrb` at 0xbdf30 | load | 1 | `sp -0x21` | `-` |
| `str` at 0xbdf34 | store | 4 | `sp -0x44` | `[sp-33]:1` |
| `ldrb` at 0xbdf38 | load | 1 | `sp -0x20` | `-` |
| `str` at 0xbdf3c | store | 4 | `sp -0x40` | `[sp-32]:1` |
| `ldrb` at 0xbdf40 | load | 1 | `sp -0x1f` | `-` |
| `str` at 0xbdf44 | store | 4 | `sp -0x3c` | `[sp-31]:1` |
| `ldrb` at 0xbdf48 | load | 1 | `sp -0x1e` | `-` |
| `str` at 0xbdf4c | store | 4 | `sp -0x38` | `[sp-30]:1` |
| `ldrb` at 0xbdf50 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0xbdf54 | store | 4 | `sp -0x34` | `[sp-29]:1` |
| `ldr` at 0xbdf58 | load | 4 | `abs 0xbdf74` | `-` |
| `ldr` at 0xbdf5c | load | 4 | `abs 0xbdf78` | `-` |
| `ldr` at 0xbdf68 | load | 4 | `abs 0xbdf74` | `-` |
| `ldmdb` at 0xbdf70 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbdf70 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbdf70 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbddf0 | `(unknown callee) 0x1b4cc` | `sp-36` | `0x0` | `0x8` | `sp-36` |
| `bl` at 0xbdf64 | `(unknown callee) 0x1b52c` | `0x14ab78` | `0x1331a0` | `[sp-36]:1` | `[sp-35]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xbde18 | `[sp-40] >= [sp-28] (signed, 32-bit)` | `[sp-40] < [sp-28] (signed, 32-bit)` |
| `beq` at 0xbde94 | `[sp-48] == 0x0` | `[sp-48] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbdf70 | `r0` | `0x14ab78` |

## `get_data`

- `get_data`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbdc38 to 0xbddc4.
  Established. The unnamed `0x1b4cc` call carries `0x14ab70`, `0x0`, `0x8`, entry `r3` (`bl` at 0xbdc60). Included absolute byte reads use `0x14ab74`, `0x14ab75`, `0x14ab76` and `0x14ab77`
  (`ldrb` at 0xbdd7c, `ldrb` at 0xbdd88, `ldrb` at 0xbdd98, `ldrb` at 0xbdda8). The return reports
  `[sp-40]` (`ldmdb` at 0xbddbc). Extracted data meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbdc48 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbdc4c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xbdc50 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xbdc54 | load | 4 | `abs 0xbddc0` | `-` |
| `ldr` at 0xbdc64 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbdc68 | store | 4 | `sp -0x20` | `[sp-24]` |
| `ldr` at 0xbdc6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbdc70 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbdc78 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xbdc7c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbdc80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbdc90 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbdca4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbdcac | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbdcb0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbdcd8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbdcf8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xbdcfc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbdd08 | load | 4 | `abs 0xbddc0` | `-` |
| `ldr` at 0xbdd0c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbdd24 | load | 4 | `abs 0xbddc0` | `-` |
| `ldr` at 0xbdd2c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xbdd50 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xbdd58 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xbdd5c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbdd64 | store | 4 | `sp -0x20` | `[sp-32]-1` |
| `ldr` at 0xbdd68 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbdd70 | store | 4 | `sp -0x24` | `[sp-36]-1` |
| `ldr` at 0xbdd78 | load | 4 | `abs 0xbddc0` | `-` |
| `ldrb` at 0xbdd7c | load | 1 | `abs 0x14ab74` | `-` |
| `ldr` at 0xbdd84 | load | 4 | `abs 0xbddc0` | `-` |
| `ldrb` at 0xbdd88 | load | 1 | `abs 0x14ab75` | `-` |
| `ldr` at 0xbdd94 | load | 4 | `abs 0xbddc0` | `-` |
| `ldrb` at 0xbdd98 | load | 1 | `abs 0x14ab76` | `-` |
| `ldr` at 0xbdda4 | load | 4 | `abs 0xbddc0` | `-` |
| `ldrb` at 0xbdda8 | load | 1 | `abs 0x14ab77` | `-` |
| `str` at 0xbddb0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xbddb4 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xbddbc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbddbc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbddbc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbdc60 | `(unknown callee) 0x1b4cc` | `0x14ab70` | `0x0` | `0x8` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xbdc88 | `[sp-32] >= [sp-28] (signed, 32-bit)` | `[sp-32] < [sp-28] (signed, 32-bit)` |
| `beq` at 0xbdd04 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbddbc | `r0` | `[sp-40]` |

## `UtilPortStr`

- `UtilPortStr`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6a6d8 to 0x6a918.
  Established. Entry `r0 == 0x40` takes `0x6a700`, while inequality falls through (`beq` at 0x6a6f8). Unsigned `r1-1 <= 0xb` takes an unresolved computed destination, while larger values
  fall through (`ldrls` at 0x6a70c). The known word assignment at `sp -0x1c` writes `0x10fb9c`
  (`str` at 0x6a7d8). Port label mappings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6a6e8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6a6ec | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x6a6f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6a700 | load | 4 | `sp -0x18` | `-` |
| `ldrls` at 0x6a70c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6a7d4 | load | 4 | `abs 0x6a8f4` | `-` |
| `str` at 0x6a7d8 | store | 4 | `sp -0x1c` | `0x10fb9c` |
| `ldr` at 0x6a7e0 | load | 4 | `sp -0x18` | `-` |
| `ldrls` at 0x6a7ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6a8b4 | load | 4 | `abs 0x6a8f4` | `-` |
| `str` at 0x6a8b8 | store | 4 | `sp -0x1c` | `0x10fb9c` |
| `ldr` at 0x6a8bc | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x6a8c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6a8c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6a8c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6a6f8 | `r0 == 0x40` | `r0 != 0x40` |
| `ldrls` at 0x6a70c | `r1-1 <= 0xb (unsigned, 32-bit)` | `r1-1 > 0xb (unsigned, 32-bit)` |
| `ldrls` at 0x6a7ec | `r1-1 <= 0xb (unsigned, 32-bit)` | `r1-1 > 0xb (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6a8c0 | `r0` | `0x10fb9c` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x6a70c).

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x6a7ec).

## `IRFmtClas`

- `IRFmtClas`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb85bc to 0xb8654.
  Established. Entry `r0 == 0x1` takes `0xb861c`, while inequality falls through (`beq` at 0xb85e0).
  The word at `sp -0x18` receives `0x128e6c` on that included path (`str` at 0xb8620). Another
  included assignment there writes `0x128e74` (`str` at 0xb8638). Classification labels remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb85cc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb85d0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb85d4 | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0xb85d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb85e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb85f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8600 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8610 | load | 4 | `abs 0xb8644` | `-` |
| `str` at 0xb8614 | store | 4 | `sp -0x18` | `0x128e68` |
| `ldr` at 0xb861c | load | 4 | `abs 0xb8648` | `-` |
| `str` at 0xb8620 | store | 4 | `sp -0x18` | `0x128e6c` |
| `ldr` at 0xb8628 | load | 4 | `abs 0xb864c` | `-` |
| `str` at 0xb862c | store | 4 | `sp -0x18` | `0x128e70` |
| `ldr` at 0xb8634 | load | 4 | `abs 0xb8650` | `-` |
| `str` at 0xb8638 | store | 4 | `sp -0x18` | `0x128e74` |
| `ldr` at 0xb863c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xb8640 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb8640 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb8640 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb85e0 | `r0 == 0x1` | `r0 != 0x1` |
| `bgt` at 0xb85ec | `r0 > 0x1 (signed, 32-bit)` | `r0 <= 0x1 (signed, 32-bit)` |
| `beq` at 0xb85f8 | `r0 == 0x0` | `r0 != 0x0` |
| `beq` at 0xb8608 | `r0 == 0x2` | `r0 != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb8640 | `r0` | `?` |

## `IRFmtError`

- `IRFmtError`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8c1c to 0xb8c74.
  Established. Entry `r0 == 0x0` takes `0xb8c5c`, while nonzero falls through (`beq` at 0xb8c38).
  The unnamed `0x21704` call carries unknown, entry `r1`, unknown, entry `r0` (`bl` at 0xb8c4c). The
  word at `sp -0x18` receives `0x1290cc` on an included path (`str` at 0xb8c60). Error text mapping
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb8c2c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb8c30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8c3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8c40 | load | 4 | `abs 0xb8c6c` | `-` |
| `str` at 0xb8c54 | store | 4 | `sp -0x18` | `ret(0x21704)` |
| `ldr` at 0xb8c5c | load | 4 | `abs 0xb8c70` | `-` |
| `str` at 0xb8c60 | store | 4 | `sp -0x18` | `0x1290cc` |
| `ldr` at 0xb8c64 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xb8c68 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb8c68 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb8c68 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8c4c | `(unknown callee) 0x21704` | `?` | `r1` | `?` | `r0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb8c38 | `r0 == 0x0` | `r0 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb8c68 | `r0` | `?` |

## `MemUsage`

- `MemUsage`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbae6c to 0xbae9c.
  Established. The unnamed `0x2420c` call carries `ret(0xbac2c)`, unknown, unknown, `ret(0xbac2c)`
  (`bl` at 0xbae84). The absolute word at `0x145ae4` is read (`ldr` at 0xbae8c). The unnamed
  `0x1b0ac` call carries `[0x145ae4]`, unknown, unknown, `0x145ae4` (`bl` at 0xbae90). Memory usage
  reporting details remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xbae88 | load | 4 | `abs 0xbae98` | `-` |
| `ldr` at 0xbae8c | load | 4 | `abs 0x145ae4` | `-` |
| `ldmdb` at 0xbae94 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbae94 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbae94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbae78 | `(unknown callee) 0xbac2c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xbae84 | `(unknown callee) 0x2420c` | `ret(0xbac2c)` | `?` | `?` | `ret(0xbac2c)` |
| `bl` at 0xbae90 | `(unknown callee) 0x1b0ac` | `[0x145ae4]` | `?` | `?` | `0x145ae4` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbae94 | `r0` | `ret(0x1b0ac)` |
