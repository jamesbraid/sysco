# ARM power-bay FRU and TIO link paths in 1.44.0

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
`SdrDecode` and `L1FRUSHUBDeadlock` each have one unresolved computed transfer.
Their tables cover known edges only. The other thirteen routines report none.
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/powerbay-fru-tio-link-facts.txt) retain all tool sections.

## `PBaySdrGetFru`

- `PBaySdrGetFru`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9f46c to 0x9fef8.
  Established. An included word assignment clears entry `r3 +0x0` (`str` at 0x9f494).
  The unnamed `0x1943c` call carries `0x400`, unknown, unknown, unknown (`bl` at 0x9f5e4).
  Nonzero `ret(0x9dc3c)` takes `0x9f6e8`, while zero falls through (`bne` at 0x9f6cc).
  FRU response assembly rules remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9f47c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9f480 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x9f484 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x9f488 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x9f48c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9f494 | store | 4 | `r3 +0x0` | `0x0` |
| `ldr` at 0x9f498 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9f4a4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9f4b8 | store | 4 | `sp -0x300` | `0x0` |
| `ldr` at 0x9f4c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9f4cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9f4d4 | load | 4 | `abs 0x9feb0` | `-` |
| `str` at 0x9f4dc | store | 4 | `sp -0x130` | `?` |
| `ldr` at 0x9f4e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9f4f0 | load | 4 | `abs 0x9feb4` | `-` |
| `str` at 0x9f4f4 | store | 4 | `sp -0x130` | `0x12d884` |
| `ldr` at 0x9f4fc | load | 4 | `abs 0x9feb0` | `-` |
| `str` at 0x9f500 | store | 4 | `sp -0x130` | `0x12d868` |
| `ldr` at 0x9f504 | load | 4 | `sp -0x130` | `-` |
| `ldrb` at 0x9f508 | load | 1 | `[sp-304] +0x0` | `-` |
| `str` at 0x9f518 | store | 4 | `sp -0x300` | `0x0` |
| `ldr` at 0x9f520 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9f52c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9f534 | load | 4 | `abs 0x9feb8` | `-` |
| `str` at 0x9f53c | store | 4 | `sp -0x2e4` | `?` |
| `ldr` at 0x9f540 | load | 4 | `sp -0x2e4` | `-` |
| `ldrb` at 0x9f544 | load | 1 | `[sp-740] +0x0` | `-` |
| `str` at 0x9f554 | store | 4 | `sp -0x300` | `0x0` |
| `ldr` at 0x9f560 | load | 4 | `sp -0x2e4` | `-` |
| `ldrb` at 0x9f564 | load | 1 | `[sp-740] +0x0` | `-` |
| `ldr` at 0x9f568 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f57c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9f58c | store | 4 | `sp -0x2cc` | `ret(0x9dc3c)` |
| `ldr` at 0x9f590 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x9f59c | load | 4 | `sp -0x140` | `-` |
| `str` at 0x9f5a4 | store | 4 | `sp -0x2e8` | `ret(0xa1808)` |
| `ldr` at 0x9f5ac | load | 4 | `sp -0x2e8` | `-` |
| `ldrb` at 0x9f5b0 | load | 1 | `[sp-744] +0x2` | `-` |
| `strb` at 0x9f5b8 | store | 1 | `(unknown)` | `([[sp-744]+2]:1 & 0xff)` |
| `ldrb` at 0x9f5c4 | load | 1 | `(unknown)` | `-` |
| `str` at 0x9f5d8 | store | 4 | `sp -0x300` | `0x0` |
| `str` at 0x9f5ec | store | 4 | `sp -0x28` | `ret(0x1943c)` |
| `ldr` at 0x9f5f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9f5f4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9f5f8 | store | 4 | `[sp-32] +0x0` | `ret(0x1943c)` |
| `ldr` at 0x9f5fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9f608 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9f60c | load | 4 | `abs 0x9febc` | `-` |
| `ldr` at 0x9f618 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9f620 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x9f628 | store | 1 | `[sp-40] +0x1` | `([sp-24]+48 & 0xff)` |
| `ldr` at 0x9f630 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9f63c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9f640 | load | 4 | `abs 0x9fec0` | `-` |
| `ldr` at 0x9f64c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9f650 | load | 4 | `abs 0x9fec4` | `-` |
| `ldr` at 0x9f658 | load | 4 | `abs 0x9fec8` | `-` |
| `strh` at 0x9f664 | store | 2 | `sp -0x2d2` | `0x0` |
| `ldr` at 0x9f668 | load | 4 | `abs 0x9fecc` | `-` |
| `strb` at 0x9f674 | store | 1 | `sp -0x2d3` | `0x8` |
| `ldr` at 0x9f67c | load | 4 | `abs 0x9fec8` | `-` |
| `ldrh` at 0x9f684 | load | 2 | `sp -0x2d2` | `-` |
| `ldr` at 0x9f688 | load | 4 | `abs 0x9fecc` | `-` |
| `ldrb` at 0x9f690 | load | 1 | `sp -0x2d3` | `-` |
| `str` at 0x9f694 | store | 4 | `sp -0x308` | `0x8` |
| `ldr` at 0x9f698 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f6a0 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0x9f6b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9f6c0 | store | 4 | `sp -0x2cc` | `ret(0x9dc3c)` |
| `ldr` at 0x9f6c4 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x9f6d0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9f6d4 | load | 4 | `abs 0x9fed0` | `-` |
| `str` at 0x9f6e0 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0x9f6e8 | load | 4 | `sp -0x140` | `-` |
| `str` at 0x9f6f0 | store | 4 | `sp -0x2ec` | `ret(0xa1808)` |
| `ldr` at 0x9f6f8 | load | 4 | `sp -0x2ec` | `-` |
| `ldr` at 0x9f710 | load | 4 | `abs 0x9fed4` | `-` |
| `ldrb` at 0x9f71c | load | 1 | `sp -0x2f0` | `-` |
| `ldr` at 0x9f728 | load | 4 | `abs 0x9fed4` | `-` |
| `ldrb` at 0x9f734 | load | 1 | `sp -0x2f0` | `-` |
| `ldr` at 0x9f740 | load | 4 | `abs 0x9fec8` | `-` |
| `ldr` at 0x9f744 | load | 4 | `abs 0x9fed4` | `-` |
| `ldrb` at 0x9f750 | load | 1 | `sp -0x2f0` | `-` |
| `strh` at 0x9f75c | store | 2 | `sp -0x2d2` | `?` |
| `ldr` at 0x9f760 | load | 4 | `abs 0x9fecc` | `-` |
| `strb` at 0x9f76c | store | 1 | `sp -0x2d3` | `0x3` |
| `ldr` at 0x9f774 | load | 4 | `abs 0x9fec8` | `-` |
| `ldrh` at 0x9f77c | load | 2 | `sp -0x2d2` | `-` |
| `ldr` at 0x9f780 | load | 4 | `abs 0x9fecc` | `-` |
| `ldrb` at 0x9f788 | load | 1 | `sp -0x2d3` | `-` |
| `str` at 0x9f78c | store | 4 | `sp -0x308` | `0x3` |
| `str` at 0x9f794 | store | 4 | `sp -0x304` | `sp-296` |
| `ldr` at 0x9f798 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f7a0 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0x9f7b0 | load | 4 | `abs 0x9fed8` | `-` |
| `strb` at 0x9f7b8 | store | 1 | `sp -0x2d4` | `(ret(0xa0bac) & 0xff)` |
| `ldr` at 0x9f7bc | load | 4 | `abs 0x9fed8` | `-` |
| `ldrb` at 0x9f7c4 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0x9f7d0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9f7d4 | load | 4 | `abs 0x9fed0` | `-` |
| `str` at 0x9f7e0 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0x9f7e8 | load | 4 | `abs 0x9fecc` | `-` |
| `ldr` at 0x9f7ec | load | 4 | `abs 0x9fedc` | `-` |
| `ldrb` at 0x9f7f4 | load | 1 | `sp -0x127` | `-` |
| `strb` at 0x9f800 | store | 1 | `sp -0x2d3` | `?` |
| `ldr` at 0x9f804 | load | 4 | `abs 0x9fecc` | `-` |
| `ldr` at 0x9f808 | load | 4 | `abs 0x9fecc` | `-` |
| `ldr` at 0x9f80c | load | 4 | `abs 0x9fed8` | `-` |
| `ldrb` at 0x9f814 | load | 1 | `sp -0x2d3` | `-` |
| `ldrb` at 0x9f81c | load | 1 | `sp -0x2d4` | `-` |
| `strb` at 0x9f828 | store | 1 | `sp -0x2d3` | `?` |
| `ldr` at 0x9f82c | load | 4 | `abs 0x9fec8` | `-` |
| `ldr` at 0x9f830 | load | 4 | `abs 0x9fec8` | `-` |
| `ldrh` at 0x9f838 | load | 2 | `sp -0x2d2` | `-` |
| `strh` at 0x9f844 | store | 2 | `sp -0x2d2` | `([sp-722]:2+3 & 0xffff)` |
| `ldr` at 0x9f84c | load | 4 | `abs 0x9fec8` | `-` |
| `ldrh` at 0x9f854 | load | 2 | `sp -0x2d2` | `-` |
| `ldr` at 0x9f858 | load | 4 | `abs 0x9fecc` | `-` |
| `ldrb` at 0x9f860 | load | 1 | `sp -0x2d3` | `-` |
| `str` at 0x9f864 | store | 4 | `sp -0x308` | `[sp-723]:1` |
| `str` at 0x9f86c | store | 4 | `sp -0x304` | `sp-296` |
| `ldr` at 0x9f870 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f878 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0x9f888 | load | 4 | `abs 0x9fed8` | `-` |
| `strb` at 0x9f890 | store | 1 | `sp -0x2d4` | `(ret(0xa0bac) & 0xff)` |
| `ldr` at 0x9f894 | load | 4 | `abs 0x9fed8` | `-` |
| `ldrb` at 0x9f89c | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0x9f8a8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9f8ac | load | 4 | `abs 0x9fed0` | `-` |
| `str` at 0x9f8b8 | store | 4 | `sp -0x300` | `?` |
| `str` at 0x9f8c4 | store | 4 | `sp -0x12c` | `0x0` |
| `str` at 0x9f8cc | store | 4 | `sp -0x2d0` | `0x0` |
| `ldr` at 0x9f8d0 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x9f8dc | load | 4 | `abs 0x9fee0` | `-` |
| `ldr` at 0x9f8e0 | load | 4 | `sp -0x12c` | `-` |
| `ldrb` at 0x9f8f0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x9f904 | load | 4 | `abs 0x9fee0` | `-` |
| `ldr` at 0x9f908 | load | 4 | `sp -0x12c` | `-` |
| `ldrb` at 0x9f918 | load | 1 | `(sp-16 + [sp-300]) -0x118` | `-` |
| `strb` at 0x9f924 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x9f928 | load | 4 | `sp -0x12c` | `-` |
| `str` at 0x9f930 | store | 4 | `sp -0x12c` | `[sp-300]+1` |
| `ldr` at 0x9f934 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9f944 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9f94c | store | 4 | `sp -0x24` | `(ret(0x19538) + [sp-40])` |
| `ldr` at 0x9f954 | load | 4 | `sp -0x12c` | `-` |
| `ldrb` at 0x9f964 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x9f968 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x9f980 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x9f984 | load | 4 | `sp -0x12c` | `-` |
| `str` at 0x9f98c | store | 4 | `sp -0x12c` | `?` |
| `ldrb` at 0x9f998 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x9f99c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9f9a4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x9f9a8 | load | 4 | `abs 0x9fee4` | `-` |
| `ldr` at 0x9f9ac | load | 4 | `sp -0x2d0` | `-` |
| `ldrb` at 0x9f9b8 | load | 1 | `[sp-720] +0x12d8b8` | `-` |
| `ldrb` at 0x9f9c0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x9f9cc | load | 4 | `abs 0x9fee4` | `-` |
| `ldr` at 0x9f9d0 | load | 4 | `sp -0x2d0` | `-` |
| `ldrb` at 0x9f9d8 | load | 1 | `[sp-720] +0x12d8b8` | `-` |
| `ldrb` at 0x9f9e4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x9f9ec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9f9fc | load | 4 | `abs 0x9fee4` | `-` |
| `ldr` at 0x9fa00 | load | 4 | `sp -0x2d0` | `-` |
| `ldrb` at 0x9fa08 | load | 1 | `[sp-720] +0x12d8b8` | `-` |
| `ldrb` at 0x9fa14 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x9fa1c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9fa24 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x9fa2c | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x9fa38 | store | 1 | `[sp-36] +0x0` | `0x20` |
| `str` at 0x9fa40 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x9fa44 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x9fa4c | store | 1 | `[sp-36] +0x1` | `0x0` |
| `ldr` at 0x9fa50 | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0x9fa58 | store | 4 | `sp -0x2d0` | `[sp-720]+1` |
| `ldr` at 0x9fa60 | load | 4 | `abs 0x9fed4` | `-` |
| `ldrb` at 0x9fa6c | load | 1 | `sp -0x2f1` | `-` |
| `ldr` at 0x9fa78 | load | 4 | `abs 0x9fed4` | `-` |
| `ldrb` at 0x9fa84 | load | 1 | `sp -0x2f1` | `-` |
| `ldr` at 0x9fa90 | load | 4 | `abs 0x9fec8` | `-` |
| `ldr` at 0x9fa94 | load | 4 | `abs 0x9fed4` | `-` |
| `ldrb` at 0x9faa0 | load | 1 | `sp -0x2f1` | `-` |
| `strh` at 0x9faac | store | 2 | `sp -0x2d2` | `?` |
| `ldr` at 0x9fab0 | load | 4 | `abs 0x9fecc` | `-` |
| `strb` at 0x9fabc | store | 1 | `sp -0x2d3` | `0x6` |
| `ldr` at 0x9fac4 | load | 4 | `abs 0x9fec8` | `-` |
| `ldrh` at 0x9facc | load | 2 | `sp -0x2d2` | `-` |
| `ldr` at 0x9fad0 | load | 4 | `abs 0x9fecc` | `-` |
| `ldrb` at 0x9fad8 | load | 1 | `sp -0x2d3` | `-` |
| `str` at 0x9fadc | store | 4 | `sp -0x308` | `0x6` |
| `str` at 0x9fae4 | store | 4 | `sp -0x304` | `sp-296` |
| `ldr` at 0x9fae8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9faf0 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0x9fb00 | load | 4 | `abs 0x9fed8` | `-` |
| `strb` at 0x9fb08 | store | 1 | `sp -0x2d4` | `(ret(0xa0bac) & 0xff)` |
| `ldr` at 0x9fb0c | load | 4 | `abs 0x9fed8` | `-` |
| `ldrb` at 0x9fb14 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0x9fb20 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9fb24 | load | 4 | `abs 0x9fed0` | `-` |
| `str` at 0x9fb30 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0x9fb38 | load | 4 | `abs 0x9fee8` | `-` |
| `ldrb` at 0x9fb40 | load | 1 | `sp -0x123` | `-` |
| `str` at 0x9fb44 | store | 4 | `sp -0x2dc` | `[sp-291]:1` |
| `ldr` at 0x9fb48 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x9fb50 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x9fb54 | load | 4 | `abs 0x9feec` | `-` |
| `ldrb` at 0x9fb5c | load | 1 | `sp -0x124` | `-` |
| `ldr` at 0x9fb60 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x9fb68 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x9fb6c | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x9fb74 | store | 4 | `sp -0x2dc` | `?` |
| `ldrb` at 0x9fb80 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x9fb84 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x9fb8c | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x9fb90 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x9fb9c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9fba8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9fbb4 | load | 4 | `abs 0x9fed4` | `-` |
| `ldrb` at 0x9fbc0 | load | 1 | `sp -0x2ef` | `-` |
| `ldr` at 0x9fbcc | load | 4 | `abs 0x9fed4` | `-` |
| `ldrb` at 0x9fbd8 | load | 1 | `sp -0x2ef` | `-` |
| `ldr` at 0x9fbe4 | load | 4 | `abs 0x9fec8` | `-` |
| `ldr` at 0x9fbe8 | load | 4 | `abs 0x9fed4` | `-` |
| `ldrb` at 0x9fbf4 | load | 1 | `sp -0x2ef` | `-` |
| `strh` at 0x9fc00 | store | 2 | `sp -0x2d2` | `?` |
| `str` at 0x9fc08 | store | 4 | `sp -0x2d0` | `0x0` |
| `ldr` at 0x9fc10 | load | 4 | `abs 0x9fec8` | `-` |
| `ldrh` at 0x9fc18 | load | 2 | `sp -0x2d2` | `-` |
| `str` at 0x9fc20 | store | 4 | `sp -0x308` | `0x8` |
| `str` at 0x9fc28 | store | 4 | `sp -0x304` | `sp-764` |
| `ldr` at 0x9fc2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9fc34 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0x9fc44 | load | 4 | `abs 0x9fed8` | `-` |
| `strb` at 0x9fc4c | store | 1 | `sp -0x2d4` | `(ret(0xa0bac) & 0xff)` |
| `ldr` at 0x9fc50 | load | 4 | `abs 0x9fed8` | `-` |
| `ldrb` at 0x9fc58 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0x9fc64 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9fc68 | load | 4 | `abs 0x9fed0` | `-` |
| `str` at 0x9fc74 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0x9fc9c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9fca0 | load | 4 | `abs 0x9fef0` | `-` |
| `str` at 0x9fcac | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0x9fcb4 | load | 4 | `abs 0x9fec8` | `-` |
| `ldr` at 0x9fcb8 | load | 4 | `abs 0x9fed8` | `-` |
| `ldrb` at 0x9fcc0 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0x9fcc8 | load | 4 | `abs 0x9fec8` | `-` |
| `ldrh` at 0x9fcd0 | load | 2 | `sp -0x2d2` | `-` |
| `strh` at 0x9fcdc | store | 2 | `sp -0x2d2` | `(([sp-724]:1 + [sp-722]:2) & 0xffff)` |
| `ldr` at 0x9fce4 | load | 4 | `abs 0x9fec8` | `-` |
| `ldrh` at 0x9fcec | load | 2 | `sp -0x2d2` | `-` |
| `ldr` at 0x9fcf0 | load | 4 | `abs 0x9fef4` | `-` |
| `ldrb` at 0x9fcfc | load | 1 | `sp -0x2fa` | `-` |
| `str` at 0x9fd00 | store | 4 | `sp -0x308` | `[sp-762]:1` |
| `str` at 0x9fd08 | store | 4 | `sp -0x304` | `sp-296` |
| `ldr` at 0x9fd0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9fd14 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0x9fd24 | load | 4 | `abs 0x9fed8` | `-` |
| `strb` at 0x9fd2c | store | 1 | `sp -0x2d4` | `(ret(0xa0bac) & 0xff)` |
| `ldr` at 0x9fd30 | load | 4 | `abs 0x9fef4` | `-` |
| `ldr` at 0x9fd3c | load | 4 | `abs 0x9fed8` | `-` |
| `ldrb` at 0x9fd40 | load | 1 | `sp -0x2fa` | `-` |
| `ldrb` at 0x9fd48 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0x9fd54 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9fd58 | load | 4 | `abs 0x9fed0` | `-` |
| `str` at 0x9fd64 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0x9fd6c | load | 4 | `abs 0x9fed8` | `-` |
| `ldrb` at 0x9fd74 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0x9fd7c | load | 4 | `abs 0x9fee0` | `-` |
| `ldr` at 0x9fd8c | load | 4 | `abs 0x9fef4` | `-` |
| `ldrb` at 0x9fd98 | load | 1 | `sp -0x2f9` | `-` |
| `strb` at 0x9fd9c | store | 1 | `(unknown)` | `([sp-761]:1 & 0xff)` |
| `ldr` at 0x9fda4 | load | 4 | `abs 0x9fed8` | `-` |
| `ldrb` at 0x9fdac | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0x9fdd0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9fdd4 | load | 4 | `abs 0x9fef0` | `-` |
| `str` at 0x9fde0 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0x9fde8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9fdf8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9fe00 | store | 4 | `sp -0x24` | `(ret(0x19538) + [sp-40])` |
| `ldr` at 0x9fe04 | load | 4 | `abs 0x9fef4` | `-` |
| `ldrb` at 0x9fe10 | load | 1 | `sp -0x2fc` | `-` |
| `ldr` at 0x9fe14 | load | 4 | `abs 0x9fef4` | `-` |
| `ldrb` at 0x9fe20 | load | 1 | `sp -0x2fa` | `-` |
| `ldr` at 0x9fe30 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9fe38 | load | 4 | `abs 0x9fec8` | `-` |
| `ldr` at 0x9fe3c | load | 4 | `abs 0x9fed8` | `-` |
| `ldrb` at 0x9fe44 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0x9fe4c | load | 4 | `abs 0x9fec8` | `-` |
| `ldrh` at 0x9fe54 | load | 2 | `sp -0x2d2` | `-` |
| `strh` at 0x9fe60 | store | 2 | `sp -0x2d2` | `(([sp-724]:1 + [sp-722]:2) & 0xffff)` |
| `ldr` at 0x9fe64 | load | 4 | `abs 0x9fef4` | `-` |
| `ldrsb` at 0x9fe70 | load | 1 | `sp -0x2fb` | `-` |
| `ldr` at 0x9fe80 | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0x9fe8c | store | 4 | `sp -0x2d0` | `[sp-720]+1` |
| `str` at 0x9fea0 | store | 4 | `sp -0x300` | `0x0` |
| `ldr` at 0x9fea4 | load | 4 | `sp -0x300` | `-` |
| `ldm` at 0x9feac | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9feac | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9feac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9f574 | `(unknown callee) 0xa0960` | `[sp-20]` | `sp-712` | `[[sp-740]]:1` | `[[sp-740]]:1` |
| `bl` at 0x9f584 | `(unknown callee) 0x9dc3c` | `[sp-20]` | `sp-712` | `?` | `sp-712` |
| `bl` at 0x9f5a0 | `(unknown callee) 0xa1808` | `[sp-320]` | `?` | `?` | `ret(0x9dc3c)` |
| `bl` at 0x9f5e4 | `(unknown callee) 0x1943c` | `0x400` | `?` | `?` | `?` |
| `bl` at 0x9f610 | `(unknown callee) 0x19808` | `[sp-40]` | `0x11303c` | `[sp-32]` | `[sp-24]` |
| `bl` at 0x9f644 | `(unknown callee) 0x19808` | `[sp-40]` | `0x113044` | `[sp-32]` | `[sp-24]` |
| `bl` at 0x9f654 | `(unknown callee) 0x19808` | `[sp-40]` | `0x11304c` | `[sp-32]` | `[sp-24]` |
| `bl` at 0x9f6a8 | `(unknown callee) 0xa0a88` | `[sp-20]` | `sp-712` | `[sp-304]` | `0x0` |
| `bl` at 0x9f6b8 | `(unknown callee) 0x9dc3c` | `[sp-20]` | `sp-712` | `?` | `sp-712` |
| `bl` at 0x9f6d8 | `(unknown callee) 0x195e0` | `[sp-40]` | `0x112ee0` | `?` | `ret(0x9dc3c)` |
| `bl` at 0x9f6ec | `(unknown callee) 0xa1808` | `[sp-320]` | `?` | `?` | `ret(0x9dc3c)` |
| `bl` at 0x9f70c | `(unknown callee) 0x19610` | `sp-756` | `ret(0xa1808)+1` | `0x8` | `ret(0xa1808)+1` |
| `bl` at 0x9f7a8 | `(unknown callee) 0xa0bac` | `[sp-20]` | `sp-712` | `[sp-304]` | `[sp-722]:2` |
| `bl` at 0x9f7d8 | `(unknown callee) 0x195e0` | `[sp-40]` | `0x112ee0` | `ret(0xa0bac)` | `(ret(0xa0bac) & 0xff)` |
| `bl` at 0x9f880 | `(unknown callee) 0xa0bac` | `[sp-20]` | `sp-712` | `[sp-304]` | `([sp-722]:2+3 & 0xffff)` |
| `bl` at 0x9f8b0 | `(unknown callee) 0x195e0` | `[sp-40]` | `0x112ee0` | `sp-16` | `(ret(0xa0bac) & 0xff)` |
| `bl` at 0x9f938 | `(unknown callee) 0x19538` | `[sp-40]` | `?` | `sp-16` | `[sp-300]+1` |
| `bl` at 0x9f974 | `(unknown callee) 0x19610` | `[sp-36]` | `(sp-296 + [sp-300])` | `?` | `?` |
| `bl` at 0x9f9f8 | `(unknown callee) 0x19748` | `[sp-36]` | `0x20` | `?` | `?` |
| `bl` at 0x9faf8 | `(unknown callee) 0xa0bac` | `[sp-20]` | `sp-712` | `[sp-304]` | `[sp-722]:2` |
| `bl` at 0x9fb28 | `(unknown callee) 0x195e0` | `[sp-40]` | `0x112ee0` | `ret(0xa0bac)` | `(ret(0xa0bac) & 0xff)` |
| `bl` at 0x9fb94 | `(unknown callee) 0xa10ec` | `[sp-732]` | `?` | `?` | `?` |
| `bl` at 0x9fba4 | `(unknown callee) 0x195e0` | `[sp-40]` | `ret(0xa10ec)` | `?` | `ret(0xa10ec)` |
| `bl` at 0x9fc3c | `(unknown callee) 0xa0bac` | `[sp-20]` | `sp-712` | `[sp-304]` | `[sp-722]:2` |
| `bl` at 0x9fc6c | `(unknown callee) 0x195e0` | `[sp-40]` | `0x112ee0` | `ret(0xa0bac)` | `(ret(0xa0bac) & 0xff)` |
| `bl` at 0x9fc88 | `(unknown callee) 0xa1148` | `sp-764` | `0x8` | `ret(0xa0bac)` | `sp-764` |
| `bl` at 0x9fca4 | `(unknown callee) 0x195e0` | `[sp-40]` | `0x113054` | `?` | `?` |
| `bl` at 0x9fd1c | `(unknown callee) 0xa0bac` | `[sp-20]` | `sp-712` | `[sp-304]` | `(([sp-724]:1 + [sp-722]:2) & 0xffff)` |
| `bl` at 0x9fd5c | `(unknown callee) 0x195e0` | `[sp-40]` | `0x112ee0` | `0xfffffd3c` | `(ret(0xa0bac) & 0xff)` |
| `bl` at 0x9fdbc | `(unknown callee) 0xa1148` | `sp-296` | `[sp-724]:1+1` | `sp-296` | `[sp-724]:1+1` |
| `bl` at 0x9fdd8 | `(unknown callee) 0x195e0` | `[sp-40]` | `0x113054` | `?` | `?` |
| `bl` at 0x9fdec | `(unknown callee) 0x19538` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x9fe34 | `(unknown callee) 0xa66e0` | `[sp-764]:1` | `[sp-762]:1` | `sp-296` | `(ret(0x19538) + [sp-40])` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x9f4a0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bgt` at 0x9f4ac | `[sp-24] > 0x6 (signed, 32-bit)` | `[sp-24] <= 0x6 (signed, 32-bit)` |
| `ble` at 0x9f4c8 | `[sp-24] <= 0x0 (signed, 32-bit)` | `[sp-24] > 0x0 (signed, 32-bit)` |
| `bge` at 0x9f4ec | `[sp-24] >= 0x0 (signed, 32-bit)` | `[sp-24] < 0x0 (signed, 32-bit)` |
| `bne` at 0x9f510 | `[[sp-304]]:1 != 0xff` | `[[sp-304]]:1 == 0xff` |
| `ble` at 0x9f528 | `[sp-24] <= 0x0 (signed, 32-bit)` | `[sp-24] > 0x0 (signed, 32-bit)` |
| `bne` at 0x9f54c | `[[sp-740]]:1 != 0xff` | `[[sp-740]]:1 == 0xff` |
| `beq` at 0x9f598 | `ret(0x9dc3c) == 0x0` | `ret(0x9dc3c) != 0x0` |
| `beq` at 0x9f5d0 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x9f604 | `[sp-24] <= 0x0 (signed, 32-bit)` | `[sp-24] > 0x0 (signed, 32-bit)` |
| `bge` at 0x9f638 | `[sp-24] >= 0x0 (signed, 32-bit)` | `[sp-24] < 0x0 (signed, 32-bit)` |
| `bne` at 0x9f6cc | `ret(0x9dc3c) != 0x0` | `ret(0x9dc3c) == 0x0` |
| `beq` at 0x9f724 | `[sp-752]:1 == 0x0` | `[sp-752]:1 != 0x0` |
| `beq` at 0x9f73c | `[sp-752]:1 == 0xff` | `[sp-752]:1 != 0xff` |
| `bne` at 0x9f7cc | `(ret(0xa0bac) & 0xff) != 0x0` | `(ret(0xa0bac) & 0xff) == 0x0` |
| `bne` at 0x9f8a4 | `(ret(0xa0bac) & 0xff) != 0x0` | `(ret(0xa0bac) & 0xff) == 0x0` |
| `bgt` at 0x9f8d8 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0x9f8f8 | `Z == 0` | `not (Z == 0)` |
| `bls` at 0x9f9c8 | `[[sp-720]+1235128]:1 <= ? (unsigned, 32-bit)` | `[[sp-720]+1235128]:1 > ? (unsigned, 32-bit)` |
| `beq` at 0x9fa74 | `[sp-753]:1 == 0x0` | `[sp-753]:1 != 0x0` |
| `beq` at 0x9fa8c | `[sp-753]:1 == 0xff` | `[sp-753]:1 != 0xff` |
| `bne` at 0x9fb1c | `(ret(0xa0bac) & 0xff) != 0x0` | `(ret(0xa0bac) & 0xff) == 0x0` |
| `beq` at 0x9fbb0 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `beq` at 0x9fbc8 | `[sp-751]:1 == 0x0` | `[sp-751]:1 != 0x0` |
| `beq` at 0x9fbe0 | `[sp-751]:1 == 0xff` | `[sp-751]:1 != 0xff` |
| `beq` at 0x9fc60 | `(ret(0xa0bac) & 0xff) == 0x8` | `(ret(0xa0bac) & 0xff) != 0x8` |
| `beq` at 0x9fc98 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9fd50 | `[sp-762]:1 == (ret(0xa0bac) & 0xff)` | `[sp-762]:1 != (ret(0xa0bac) & 0xff)` |
| `beq` at 0x9fdcc | `? == 0x0` | `? != 0x0` |
| `blt` at 0x9fe78 | `? < 0x0 (signed, 32-bit)` | `? >= 0x0 (signed, 32-bit)` |
| `bgt` at 0x9fe94 | `[sp-720] > 0x4 (signed, 32-bit)` | `[sp-720] <= 0x4 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9feac | `r0` | `?` |

## `PBaySdrGetConnector`

- `PBaySdrGetConnector`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa0fbc to 0xa10ec.
  Established. The unnamed `0x9dc3c` call carries `[sp-20]`, `sp-428`, `0xfffffe6c`, `sp-428` (`bl` at 0xa1038);
  its zero result takes `0xa10cc`, while nonzero falls through (`beq` at 0xa104c).
  An included word assignment puts `0xa` at `sp -0x1a8` (`str` at 0xa0ff8).
  Connector response meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa0fcc | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xa0fdc | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xa0fe0 | load | 4 | `abs 0xa10e4` | `-` |
| `ldr` at 0xa0fe4 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xa0fe8 | load | 2 | `[sp-20] +0x6` | `-` |
| `strh` at 0xa0ff0 | store | 2 | `sp -0x1ac` | `([[sp-20]+6]:2 & 0xffff)` |
| `str` at 0xa0ff8 | store | 4 | `sp -0x1a8` | `0xa` |
| `str` at 0xa1000 | store | 4 | `sp -0x20` | `0x3` |
| `str` at 0xa1008 | store | 4 | `sp -0x1c` | `0x3` |
| `str` at 0xa1010 | store | 4 | `sp -0x114` | `0x1` |
| `ldr` at 0xa1014 | load | 4 | `abs 0xa10e8` | `-` |
| `strb` at 0xa1020 | store | 1 | `sp -0x1a4` | `0x0` |
| `str` at 0xa1028 | store | 4 | `sp -0x110` | `0xff` |
| `ldr` at 0xa1030 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa1040 | store | 4 | `sp -0x1b0` | `ret(0x9dc3c)` |
| `ldr` at 0xa1044 | load | 4 | `sp -0x1b0` | `-` |
| `ldr` at 0xa1050 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa1054 | store | 4 | `sp -0x1b4` | `[sp-36]` |
| `ldr` at 0xa1058 | load | 4 | `sp -0x1b4` | `-` |
| `ldrb` at 0xa105c | load | 1 | `[sp-36] +0x0` | `-` |
| `str` at 0xa1070 | store | 4 | `sp -0x1a8` | `0xb` |
| `str` at 0xa1078 | store | 4 | `sp -0x114` | `0x0` |
| `ldr` at 0xa1080 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa1090 | store | 4 | `sp -0x1b0` | `ret(0x9dc3c)` |
| `ldr` at 0xa1094 | load | 4 | `sp -0x1b0` | `-` |
| `ldr` at 0xa10a0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa10a4 | store | 4 | `sp -0x1b8` | `[sp-36]` |
| `ldr` at 0xa10a8 | load | 4 | `sp -0x1b8` | `-` |
| `ldrb` at 0xa10ac | load | 1 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xa10bc | load | 4 | `sp -0x1b8` | `-` |
| `ldrb` at 0xa10c0 | load | 1 | `[sp-440] +0x0` | `-` |
| `strb` at 0xa10c8 | store | 1 | `(unknown)` | `([[sp-440]]:1 & 0xff)` |
| `ldrb` at 0xa10d4 | load | 1 | `(unknown)` | `-` |
| `ldm` at 0xa10e0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa10e0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa10e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa1038 | `(unknown callee) 0x9dc3c` | `[sp-20]` | `sp-428` | `0xfffffe6c` | `sp-428` |
| `bl` at 0xa1088 | `(unknown callee) 0x9dc3c` | `[sp-20]` | `sp-428` | `?` | `sp-428` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa104c | `ret(0x9dc3c) == 0x0` | `ret(0x9dc3c) != 0x0` |
| `beq` at 0xa1068 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa109c | `ret(0x9dc3c) == 0x0` | `ret(0x9dc3c) != 0x0` |
| `beq` at 0xa10b4 | `[[sp-36]]:1 == 0x0` | `[[sp-36]]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa10e0 | `r0` | `?` |

## `PBaySdrGetNbrDps`

- `PBaySdrGetNbrDps`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa149c to 0xa15c8.
  Established. The unnamed `0x9dc3c` call carries `[sp-20]`, `sp-452`, unknown, `sp-452` (`bl` at 0xa1534);
  its nonzero result takes `0xa1558`, while zero falls through (`bne` at 0xa1548).
  An included byte assignment writes `0x1` to an unknown effective target (`strb` at 0xa1504).
  Supply count and array semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa14ac | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa14b0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa14b8 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xa14c0 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xa14c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa14d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa14d8 | load | 4 | `abs 0xa15c4` | `-` |
| `str` at 0xa14e0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xa14e4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xa14e8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa14f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa14f8 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xa1504 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0xa1510 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xa1514 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa1518 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa152c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa153c | store | 4 | `sp -0x1c8` | `ret(0x9dc3c)` |
| `ldr` at 0xa1540 | load | 4 | `sp -0x1c8` | `-` |
| `str` at 0xa1550 | store | 4 | `sp -0x1cc` | `0x0` |
| `ldr` at 0xa1558 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xa1560 | store | 4 | `sp -0x28` | `ret(0xa1808)` |
| `ldr` at 0xa1564 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xa1568 | load | 1 | `ret(0xa1808) +0x2` | `-` |
| `strb` at 0xa156c | store | 1 | `sp -0x29` | `([ret(0xa1808)+2]:1 & 0xff)` |
| `ldr` at 0xa1570 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa1574 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xa157c | load | 1 | `sp -0x29` | `-` |
| `strb` at 0xa1580 | store | 1 | `([sp-24] + [sp-32]) +0x0` | `(([ret(0xa1808)+2]:1 & 0xff) & 0xff)` |
| `ldrb` at 0xa1584 | load | 1 | `sp -0x29` | `-` |
| `ldr` at 0xa1594 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa159c | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xa15a0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa15a8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xa15b0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa15b4 | store | 4 | `sp -0x1cc` | `?` |
| `ldr` at 0xa15b8 | load | 4 | `sp -0x1cc` | `-` |
| `ldm` at 0xa15c0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa15c0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa15c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa1524 | `(unknown callee) 0xa0960` | `?` | `sp-452` | `?` | `?` |
| `bl` at 0xa1534 | `(unknown callee) 0x9dc3c` | `[sp-20]` | `sp-452` | `?` | `sp-452` |
| `bl` at 0xa155c | `(unknown callee) 0xa1808` | `[sp-60]` | `?` | `?` | `ret(0x9dc3c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0xa14cc | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0xa14f0 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xa1548 | `ret(0x9dc3c) != 0x0` | `ret(0x9dc3c) == 0x0` |
| `bne` at 0xa1590 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa15c0 | `r0` | `?` |

## `SdrDecode`

- `SdrDecode`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa03c4 to 0xa047c.
  Established. A byte load reads entry `r0 +0x3` (`ldrb` at 0xa03e4).
  A word assignment saves `[r0+3]:1` at `sp -0x1c` (`str` at 0xa03e8).
  The integer return candidate retains entry `r0` (`ldm` at 0xa0478).
  SDR dispatch targets remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa03d4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa03d8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa03dc | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xa03e0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa03e4 | load | 1 | `r0 +0x3` | `-` |
| `str` at 0xa03e8 | store | 4 | `sp -0x1c` | `[r0+3]:1` |
| `ldr` at 0xa03ec | load | 4 | `sp -0x1c` | `-` |
| `ldrls` at 0xa03f8 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xa0478 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa0478 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa0478 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xa03f8 | `[r0+3]:1-1 <= 0x11 (unsigned, 32-bit)` | `[r0+3]:1-1 > 0x11 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa0478 | `r0` | `r0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xa03f8).

## `SdrSetChassisFru`

- `SdrSetChassisFru`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa11bc to 0xa1210.
  Established. Inequality `[0x12d868]:1 != 0xff` takes `0xa1208`, while equality falls through (`bne` at 0xa11e0).
  Included byte stores clear `0x12d869` and `0x12d86a` (`strb` at 0xa11f8, `strb` at 0xa1204).
  Another byte assignment places `((r0 & 0xff) & 0xff)` at `0x12d868` (`strb` at 0xa11ec).
  Chassis FRU identifier meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xa11d0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xa11d4 | load | 4 | `abs 0xa120c` | `-` |
| `ldrb` at 0xa11d8 | load | 1 | `abs 0x12d868` | `-` |
| `ldr` at 0xa11e4 | load | 4 | `abs 0xa120c` | `-` |
| `ldrb` at 0xa11e8 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0xa11ec | store | 1 | `abs 0x12d868` | `((r0 & 0xff) & 0xff)` |
| `ldr` at 0xa11f0 | load | 4 | `abs 0xa120c` | `-` |
| `strb` at 0xa11f8 | store | 1 | `abs 0x12d869` | `0x0` |
| `ldr` at 0xa11fc | load | 4 | `abs 0xa120c` | `-` |
| `strb` at 0xa1204 | store | 1 | `abs 0x12d86a` | `0x0` |
| `ldm` at 0xa1208 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xa1208 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa1208 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa1208 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa11e0 | `[0x12d868]:1 != 0xff` | `[0x12d868]:1 == 0xff` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa1208 | `r0` | `r0` |

## `L1FRUSHUBInit`

- `L1FRUSHUBInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9c504 to 0x9c704.
  Established. Nonzero `[0x29a95c]` takes `0x9c538`, while zero falls through (`bne` at 0x9c530).
  Included word stores put `0x11736c` and `0x11a87c` at unknown effective targets (`str` at 0x9c5f0, `str` at 0x9c644).
  An absolute word assignment writes `[0x29a958]+1` to `0x29a958` (`str` at 0x9c6c8).
  SHUB capture record setup remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9c514 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x9c518 | load | 4 | `abs 0x9c6f0` | `-` |
| `ldr` at 0x9c51c | load | 4 | `abs 0x29a958` | `-` |
| `str` at 0x9c520 | store | 4 | `sp -0x24` | `[0x29a958]` |
| `ldr` at 0x9c524 | load | 4 | `abs 0x9c6f0` | `-` |
| `ldr` at 0x9c528 | load | 4 | `abs 0x29a95c` | `-` |
| `str` at 0x9c53c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x9c540 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9c544 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c550 | load | 4 | `abs 0x9c6f0` | `-` |
| `ldr` at 0x9c554 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9c568 | load | 4 | `abs 0x29a95c` | `-` |
| `str` at 0x9c574 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x9c578 | load | 4 | `abs 0x9c6f0` | `-` |
| `ldr` at 0x9c57c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9c590 | load | 4 | `abs 0x29a95c` | `-` |
| `ldr` at 0x9c598 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9c59c | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0x9c5a0 | load | 4 | `abs 0x9c6f0` | `-` |
| `ldr` at 0x9c5a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9c5b8 | load | 4 | `abs 0x29a95c` | `-` |
| `ldr` at 0x9c5c0 | load | 4 | `abs 0x9c6f4` | `-` |
| `ldr` at 0x9c5c4 | load | 4 | `abs 0x12d8cc` | `-` |
| `str` at 0x9c5c8 | store | 4 | `(unknown)` | `[0x12d8cc]` |
| `ldr` at 0x9c5cc | load | 4 | `abs 0x9c6f0` | `-` |
| `ldr` at 0x9c5d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9c5e4 | load | 4 | `abs 0x29a95c` | `-` |
| `ldr` at 0x9c5ec | load | 4 | `abs 0x9c6f8` | `-` |
| `str` at 0x9c5f0 | store | 4 | `(unknown)` | `0x11736c` |
| `ldr` at 0x9c5f4 | load | 4 | `abs 0x9c6f0` | `-` |
| `ldr` at 0x9c5f8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9c60c | load | 4 | `abs 0x29a95c` | `-` |
| `ldr` at 0x9c614 | load | 4 | `abs 0x9c6fc` | `-` |
| `ldr` at 0x9c618 | load | 4 | `abs 0x12d8d0` | `-` |
| `str` at 0x9c61c | store | 4 | `(unknown)` | `[0x12d8d0]` |
| `ldr` at 0x9c620 | load | 4 | `abs 0x9c6f0` | `-` |
| `ldr` at 0x9c624 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9c638 | load | 4 | `abs 0x29a95c` | `-` |
| `ldr` at 0x9c640 | load | 4 | `abs 0x9c700` | `-` |
| `str` at 0x9c644 | store | 4 | `(unknown)` | `0x11a87c` |
| `ldr` at 0x9c648 | load | 4 | `abs 0x9c6f4` | `-` |
| `ldr` at 0x9c64c | load | 4 | `abs 0x12d8cc` | `-` |
| `str` at 0x9c654 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x9c658 | load | 4 | `abs 0x9c6f0` | `-` |
| `ldr` at 0x9c65c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9c670 | load | 4 | `abs 0x29a95c` | `-` |
| `ldr` at 0x9c678 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x9c684 | store | 4 | `(unknown)` | `ret(0x1943c)` |
| `ldr` at 0x9c688 | load | 4 | `abs 0x9c6f0` | `-` |
| `ldr` at 0x9c68c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9c6a0 | load | 4 | `abs 0x29a95c` | `-` |
| `ldr` at 0x9c6a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9c6b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9c6b8 | load | 4 | `abs 0x9c6f0` | `-` |
| `ldr` at 0x9c6bc | load | 4 | `abs 0x9c6f0` | `-` |
| `ldr` at 0x9c6c0 | load | 4 | `abs 0x29a958` | `-` |
| `str` at 0x9c6c8 | store | 4 | `abs 0x29a958` | `[0x29a958]+1` |
| `ldr` at 0x9c6cc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9c6d4 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x9c6d8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9c6e0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldm` at 0x9c6ec | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x9c6ec | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9c6ec | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9c6ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9c67c | `(unknown callee) 0x1943c` | `[sp-28]` | `0x29a944` | `?` | `[0x29a95c]` |
| `bl` at 0x9c6b4 | `(unknown callee) 0x19748` | `?` | `0x0` | `[sp-28]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x9c530 | `[0x29a95c] != 0x0` | `[0x29a95c] == 0x0` |
| `bge` at 0x9c54c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9c6ec | `r0` | `?` |

## `L1FRUSHUBCapture`

- `L1FRUSHUBCapture`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9b600 to 0x9b8e0.
  Established. Included absolute word assignments write `0x1` and `0x4` to `0x29a94c` (`str` at 0x9b62c, `str` at 0x9b894).
  The unnamed `0xde16c` call carries `ret(0x4203c)`, `[sp-40]`, `sp-48`, `sp-52` (`bl` at 0x9b76c).
  Nonzero `[r0+24]` takes `0x9b634`, while zero falls through (`bne` at 0x9b620).
  SHUB capture result encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9b610 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x9b614 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9b618 | load | 4 | `r0 +0x18` | `-` |
| `ldr` at 0x9b624 | load | 4 | `abs 0x9b8d8` | `-` |
| `str` at 0x9b62c | store | 4 | `abs 0x29a94c` | `0x1` |
| `ldr` at 0x9b638 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9b63c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9b640 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0x9b648 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x9b65c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x9b660 | load | 4 | `abs 0x9b8d8` | `-` |
| `ldr` at 0x9b664 | load | 4 | `abs 0x29a94c` | `-` |
| `ldr` at 0x9b670 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9b674 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0x9b678 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9b688 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x9b68c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9b690 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9b6a4 | load | 4 | `[sp-20] +0xc` | `-` |
| `ldr` at 0x9b6ac | load | 4 | `(unknown)` | `-` |
| `str` at 0x9b6b0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x9b6b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9b6b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9b6cc | load | 4 | `[sp-20] +0xc` | `-` |
| `ldrb` at 0x9b6d4 | load | 1 | `(unknown)` | `-` |
| `str` at 0x9b6dc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x9b6e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9b6ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9b6f0 | load | 4 | `[sp-20] +0x4` | `-` |
| `str` at 0x9b700 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x9b704 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9b708 | store | 4 | `sp -0x40` | `[sp-40]` |
| `str` at 0x9b710 | store | 4 | `sp -0x3c` | `sp-48` |
| `str` at 0x9b718 | store | 4 | `sp -0x38` | `sp-52` |
| `str` at 0x9b740 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x9b748 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9b74c | load | 4 | `[sp-20] +0x4` | `-` |
| `ldr` at 0x9b764 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9b780 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x9b784 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9b790 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9b794 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9b79c | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x9b7a4 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0x9b7ac | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x9b7b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9b7b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9b7bc | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x9b7c4 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0x9b7cc | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x9b7d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9b7d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9b7dc | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x9b7e4 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0x9b7ec | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x9b7f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9b7f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9b7fc | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x9b804 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0x9b808 | store | 1 | `(unknown)` | `([sp-48] & 0xff)` |
| `ldr` at 0x9b80c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9b810 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9b818 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x9b820 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0x9b828 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x9b82c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9b830 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9b838 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x9b840 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0x9b848 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x9b84c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9b850 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9b858 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x9b860 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0x9b868 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x9b86c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9b870 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9b878 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x9b880 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0x9b884 | store | 1 | `(unknown)` | `([sp-52] & 0xff)` |
| `ldr` at 0x9b88c | load | 4 | `abs 0x9b8d8` | `-` |
| `str` at 0x9b894 | store | 4 | `abs 0x29a94c` | `0x4` |
| `ldr` at 0x9b898 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9b89c | store | 4 | `sp -0x44` | `[sp-44]` |
| `ldr` at 0x9b8a0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9b8a4 | store | 4 | `sp -0x40` | `[sp-36]` |
| `ldr` at 0x9b8a8 | load | 4 | `abs 0x9b8dc` | `-` |
| `ldr` at 0x9b8ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9b8b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9b8b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9b8bc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9b8c4 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldm` at 0x9b8d4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9b8d4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9b8d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9b634 | `(unknown callee) 0xd708c` | `r0` | `r1` | `r2` | `[r0+24]` |
| `bl` at 0x9b654 | `(unknown callee) 0x19748` | `[[sp-20]+24]` | `0x0` | `?` | `?` |
| `bl` at 0x9b6f4 | `(unknown callee) 0x4203c` | `[[sp-20]+4]` | `[sp-20]` | `?` | `[sp-20]` |
| `bl` at 0x9b72c | `(unknown callee) 0x5a344` | `ret(0x4203c)` | `?` | `?` | `?` |
| `bl` at 0x9b750 | `(unknown callee) 0x4203c` | `[[sp-20]+4]` | `[sp-20]` | `?` | `[sp-20]` |
| `bl` at 0x9b76c | `(unknown callee) 0xde16c` | `ret(0x4203c)` | `[sp-40]` | `sp-48` | `sp-52` |
| `bl` at 0x9b8b8 | `(unknown callee) 0x1e958` | `0x112524` | `[sp-20]` | `[sp-24]` | `[sp-40]` |
| `bl` at 0x9b8cc | `(unknown callee) 0xd70fc` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x9b620 | `[r0+24] != 0x0` | `[r0+24] == 0x0` |
| `bne` at 0x9b66c | `[0x29a94c] != 0x0` | `[0x29a94c] == 0x0` |
| `ble` at 0x9b680 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x9b6e8 | `[sp-44] != 0x1` | `[sp-44] == 0x1` |
| `beq` at 0x9b78c | `[sp-36] == 0x0` | `[sp-36] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9b8d4 | `r0` | `?` |

## `L1FRUSHUBDeadlock`

- `L1FRUSHUBDeadlock`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9bb90 to 0x9c4dc.
  Established. Nonzero `[0x133ad4]` takes `0x9bc34`, while zero falls through (`bne` at 0x9bbe0).
  Included absolute word stores put `0x1e` at `0x29a970` and `0x1` at `0x133ad4` (`str` at 0x9bbec, `str` at 0x9bc30).
  The unnamed `0xde16c` call carries `ret(0x4203c)`, unknown, `sp-116`, `sp-120` (`bl` at 0x9be20).
  SHUB deadlock decoding semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9bba0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x9bba4 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x9bba8 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x9bbb0 | store | 4 | `sp -0x48` | `0x1e` |
| `str` at 0x9bbb8 | store | 4 | `sp -0x4c` | `0x5` |
| `str` at 0x9bbc0 | store | 4 | `sp -0x64` | `0x0` |
| `str` at 0x9bbc8 | store | 4 | `sp -0x68` | `0x0` |
| `str` at 0x9bbd0 | store | 4 | `sp -0x6c` | `0x0` |
| `ldr` at 0x9bbd4 | load | 4 | `abs 0x9c4b0` | `-` |
| `ldr` at 0x9bbd8 | load | 4 | `abs 0x133ad4` | `-` |
| `ldr` at 0x9bbe4 | load | 4 | `abs 0x9c4b4` | `-` |
| `str` at 0x9bbec | store | 4 | `abs 0x29a970` | `0x1e` |
| `ldr` at 0x9bbf0 | load | 4 | `abs 0x9c4b8` | `-` |
| `ldr` at 0x9bbf4 | load | 4 | `abs 0x9c4b4` | `-` |
| `ldr` at 0x9bbf8 | load | 4 | `abs 0x29a970` | `-` |
| `str` at 0x9bc0c | store | 4 | `abs 0x29a974` | `?` |
| `ldr` at 0x9bc10 | load | 4 | `abs 0x9c4bc` | `-` |
| `ldr` at 0x9bc14 | load | 4 | `abs 0x9c4b8` | `-` |
| `ldr` at 0x9bc18 | load | 4 | `abs 0x29a974` | `-` |
| `str` at 0x9bc24 | store | 4 | `abs 0x133ad8` | `ret(0x1943c)` |
| `ldr` at 0x9bc28 | load | 4 | `abs 0x9c4b0` | `-` |
| `str` at 0x9bc30 | store | 4 | `abs 0x133ad4` | `0x1` |
| `ldr` at 0x9bc48 | load | 4 | `abs 0x9c4bc` | `-` |
| `ldr` at 0x9bc4c | load | 4 | `abs 0x9c4b8` | `-` |
| `ldr` at 0x9bc50 | load | 4 | `abs 0x133ad8` | `-` |
| `ldr` at 0x9bc58 | load | 4 | `abs 0x29a974` | `-` |
| `str` at 0x9bc64 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0x9bc6c | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0x9bc70 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x9bc74 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9bc80 | load | 4 | `abs 0x9c4c0` | `-` |
| `ldr` at 0x9bc84 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x9bc98 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x9bc9c | load | 4 | `(unknown)` | `-` |
| `str` at 0x9bca4 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x9bca8 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x9bcb0 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x9bcb8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9bcbc | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x9bcc8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9bccc | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x9bcec | store | 4 | `[sp-28] +0x0` | `ret(0x1943c)` |
| `ldr` at 0x9bcf0 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x9bcf4 | store | 4 | `sp -0x60` | `[[sp-28]]` |
| `ldr` at 0x9bcf8 | load | 4 | `sp -0x60` | `-` |
| `strb` at 0x9bd00 | store | 1 | `[[sp-28]] +0x0` | `0x0` |
| `ldr` at 0x9bd08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9bd0c | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x9bd18 | store | 4 | `sp -0x70` | `ret(0x19538)` |
| `ldr` at 0x9bd1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9bd20 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9bd24 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x9bd38 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x9bd40 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x9bd50 | store | 4 | `[sp-28] +0x0` | `ret(0x195d4)` |
| `ldr` at 0x9bd54 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x9bd58 | store | 4 | `sp -0x60` | `[[sp-28]]` |
| `ldr` at 0x9bd5c | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x9bd60 | load | 4 | `sp -0x70` | `-` |
| `str` at 0x9bd68 | store | 4 | `sp -0x60` | `([[sp-28]] + [sp-112])` |
| `str` at 0x9bd74 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x9bd7c | store | 4 | `sp -0x54` | `0x0` |
| `ldr` at 0x9bd80 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x9bd84 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x9bd90 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x9bda0 | store | 4 | `sp -0x70` | `0x0` |
| `ldr` at 0x9bda8 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x9bdb0 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x9bdc0 | store | 4 | `sp -0x70` | `0x2` |
| `str` at 0x9bdcc | store | 4 | `sp -0x70` | `0x1` |
| `str` at 0x9bdd4 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0x9bdd8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9bde4 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x9bde8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9bdf4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9be00 | load | 4 | `abs 0x9c4c0` | `-` |
| `ldr` at 0x9be04 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x9be14 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9be28 | store | 4 | `sp -0x24` | `ret(0xde16c)` |
| `ldr` at 0x9be2c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9be3c | load | 4 | `sp -0x74` | `-` |
| `strb` at 0x9be44 | store | 1 | `sp -0x80` | `?` |
| `ldr` at 0x9be48 | load | 4 | `sp -0x74` | `-` |
| `strb` at 0x9be50 | store | 1 | `sp -0x7f` | `?` |
| `ldr` at 0x9be54 | load | 4 | `sp -0x74` | `-` |
| `strb` at 0x9be5c | store | 1 | `sp -0x7e` | `?` |
| `ldr` at 0x9be60 | load | 4 | `sp -0x74` | `-` |
| `strb` at 0x9be64 | store | 1 | `sp -0x7d` | `([sp-116] & 0xff)` |
| `ldr` at 0x9be68 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x9be70 | store | 1 | `sp -0x7c` | `?` |
| `ldr` at 0x9be74 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x9be7c | store | 1 | `sp -0x7b` | `?` |
| `ldr` at 0x9be80 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x9be88 | store | 1 | `sp -0x7a` | `?` |
| `ldr` at 0x9be8c | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x9be90 | store | 1 | `sp -0x79` | `([sp-120] & 0xff)` |
| `ldr` at 0x9be94 | load | 4 | `abs 0x9c4c0` | `-` |
| `ldr` at 0x9be98 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x9beac | load | 4 | `(unknown)` | `-` |
| `ldrls` at 0x9beb4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9c104 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x9c10c | store | 4 | `sp -0x58` | `[sp-88]+1` |
| `ldr` at 0x9c114 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x9c11c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x9c138 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x9c140 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x9c14c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9c158 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x9c15c | load | 4 | `abs 0x9c4c4` | `-` |
| `ldr` at 0x9c160 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c168 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x9c178 | load | 4 | `sp -0x60` | `-` |
| `str` at 0x9c180 | store | 4 | `sp -0x60` | `([sp-96] + ret(0x19538))` |
| `str` at 0x9c188 | store | 4 | `sp -0xc4` | `0x0` |
| `str` at 0x9c194 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0x9c198 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x9c19c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9c1a8 | load | 4 | `abs 0x9c4c0` | `-` |
| `ldr` at 0x9c1ac | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x9c1c0 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x9c1c4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9c1cc | store | 4 | `sp -0x6c` | `?` |
| `str` at 0x9c1d4 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0x9c1d8 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x9c1e4 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x9c1f4 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x9c20c | load | 4 | `abs 0x9c4bc` | `-` |
| `ldr` at 0x9c210 | load | 4 | `abs 0x133ad8` | `-` |
| `ldr` at 0x9c218 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9c228 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x9c230 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0x9c234 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x9c244 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x9c25c | load | 4 | `abs 0x9c4bc` | `-` |
| `ldr` at 0x9c260 | load | 4 | `abs 0x133ad8` | `-` |
| `ldr` at 0x9c268 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9c278 | load | 4 | `sp -0x68` | `-` |
| `str` at 0x9c280 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0x9c284 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x9c28c | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x9c294 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x9c29c | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x9c2a4 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x9c2b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9c2bc | load | 4 | `sp -0x6c` | `-` |
| `str` at 0x9c2c0 | store | 4 | `sp -0xcc` | `[sp-108]` |
| `ldr` at 0x9c2c4 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x9c2c8 | load | 4 | `abs 0x9c4c8` | `-` |
| `ldr` at 0x9c2cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c2d0 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x9c2d8 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x9c2e8 | load | 4 | `sp -0x60` | `-` |
| `str` at 0x9c2f0 | store | 4 | `sp -0x60` | `([sp-96] + ret(0x19538))` |
| `ldr` at 0x9c2f8 | load | 4 | `sp -0x68` | `-` |
| `str` at 0x9c2fc | store | 4 | `sp -0xcc` | `[sp-104]` |
| `ldr` at 0x9c300 | load | 4 | `sp -0x6c` | `-` |
| `str` at 0x9c304 | store | 4 | `sp -0xc8` | `[sp-108]` |
| `ldr` at 0x9c308 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x9c30c | load | 4 | `abs 0x9c4cc` | `-` |
| `ldr` at 0x9c310 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c314 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x9c31c | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x9c32c | load | 4 | `sp -0x60` | `-` |
| `str` at 0x9c334 | store | 4 | `sp -0x60` | `([sp-96] + ret(0x19538))` |
| `str` at 0x9c33c | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0x9c340 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9c34c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x9c350 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x9c360 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0x9c364 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x9c370 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x9c380 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x9c398 | load | 4 | `abs 0x9c4bc` | `-` |
| `ldr` at 0x9c39c | load | 4 | `abs 0x133ad8` | `-` |
| `ldr` at 0x9c3a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9c3b8 | load | 4 | `abs 0x9c4c0` | `-` |
| `ldr` at 0x9c3bc | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x9c3d0 | load | 4 | `abs 0x9c4d0` | `-` |
| `ldr` at 0x9c3d4 | load | 4 | `abs 0x9c4c0` | `-` |
| `ldr` at 0x9c3d8 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x9c3ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9c3fc | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x9c404 | load | 4 | `abs 0x9c4d4` | `-` |
| `ldr` at 0x9c408 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9c40c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9c418 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x9c428 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x9c440 | load | 4 | `abs 0x9c4bc` | `-` |
| `ldr` at 0x9c444 | load | 4 | `abs 0x133ad8` | `-` |
| `ldr` at 0x9c44c | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x9c450 | load | 4 | `abs 0x9c4d8` | `-` |
| `ldr` at 0x9c458 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9c460 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x9c470 | load | 4 | `sp -0x60` | `-` |
| `str` at 0x9c478 | store | 4 | `sp -0x60` | `([sp-96] + ret(0x19538))` |
| `ldr` at 0x9c47c | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x9c484 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x9c48c | load | 4 | `sp -0x58` | `-` |
| `str` at 0x9c494 | store | 4 | `sp -0x58` | `?` |
| `str` at 0x9c4a0 | store | 4 | `sp -0xc4` | `0x0` |
| `ldr` at 0x9c4a4 | load | 4 | `sp -0xc4` | `-` |
| `ldm` at 0x9c4ac | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x9c4ac | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9c4ac | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9c4ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9bc1c | `(unknown callee) 0x1943c` | `[0x29a974]` | `0x29a974` | `[0x29a970]` | `0x29a974` |
| `bl` at 0x9bc44 | `(unknown callee) 0x19748` | `sp-68` | `0x0` | `0x20` | `sp-68` |
| `bl` at 0x9bc5c | `(unknown callee) 0x19748` | `[0x133ad8]` | `0x0` | `[0x29a974]` | `0x133ad8` |
| `bl` at 0x9bce4 | `(unknown callee) 0x1943c` | `?` | `?` | `?` | `?` |
| `bl` at 0x9bd10 | `(unknown callee) 0x19538` | `[[sp-28]]` | `?` | `?` | `[sp-28]` |
| `bl` at 0x9bd48 | `(unknown callee) 0x195d4` | `[[sp-28]]` | `?` | `?` | `?` |
| `bl` at 0x9bd6c | `(unknown callee) 0xd708c` | `?` | `?` | `[[sp-28]]` | `?` |
| `bl` at 0x9bdf8 | `(unknown callee) 0x4203c` | `[sp-24]` | `?` | `?` | `[sp-72]` |
| `bl` at 0x9be20 | `(unknown callee) 0xde16c` | `ret(0x4203c)` | `?` | `sp-116` | `sp-120` |
| `bl` at 0x9c128 | `(unknown callee) 0xd70fc` | `?` | `?` | `?` | `[sp-76]` |
| `bl` at 0x9c130 | `(unknown callee) 0x9c4dc` | `0x64` | `?` | `?` | `?` |
| `bl` at 0x9c134 | `(unknown callee) 0xd708c` | `ret(0x9c4dc)` | `?` | `?` | `?` |
| `bl` at 0x9c148 | `(unknown callee) 0xd70fc` | `?` | `?` | `?` | `[sp-76]` |
| `bl` at 0x9c164 | `(unknown callee) 0x197a8` | `[sp-96]` | `0x112ba4` | `[sp-24]` | `[sp-36]` |
| `bl` at 0x9c16c | `(unknown callee) 0x19538` | `[sp-96]` | `?` | `?` | `?` |
| `bl` at 0x9c2d4 | `(unknown callee) 0x197a8` | `[sp-96]` | `0x112bc8` | `[sp-24]` | `[sp-104]` |
| `bl` at 0x9c2dc | `(unknown callee) 0x19538` | `[sp-96]` | `?` | `?` | `?` |
| `bl` at 0x9c318 | `(unknown callee) 0x197a8` | `[sp-96]` | `0x112bf0` | `[sp-24]` | `[sp-100]` |
| `bl` at 0x9c320 | `(unknown callee) 0x19538` | `[sp-96]` | `?` | `?` | `?` |
| `bl` at 0x9c410 | `(unknown callee) 0x197a8` | `sp-192` | `0x112c1c` | `?` | `?` |
| `bl` at 0x9c45c | `(unknown callee) 0x197a8` | `[sp-96]` | `0x112c24` | `sp-192` | `?` |
| `bl` at 0x9c464 | `(unknown callee) 0x19538` | `[sp-96]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x9bbe0 | `[0x133ad4] != 0x0` | `[0x133ad4] == 0x0` |
| `bge` at 0x9bc7c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x9bcc4 | `[[sp-28]] != 0x0` | `[[sp-28]] == 0x0` |
| `bge` at 0x9bd8c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x9bd98 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x9bdb8 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x9bde0 | `Z == 0` | `not (Z == 0)` |
| `bge` at 0x9bdf0 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x9be34 | `ret(0xde16c) == 0x0` | `ret(0xde16c) != 0x0` |
| `ldrls` at 0x9beb4 | `? <= 0x8 (unsigned, 32-bit)` | `? > 0x8 (unsigned, 32-bit)` |
| `bge` at 0x9c124 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x9c154 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `bge` at 0x9c1a4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bgt` at 0x9c1e0 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0x9c2ac | `[sp-100] != 0x0` | `[sp-100] == 0x0` |
| `beq` at 0x9c2b8 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `bne` at 0x9c348 | `[sp-32] != 0x2` | `[sp-32] == 0x2` |
| `bge` at 0x9c358 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bgt` at 0x9c36c | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0x9c3ac | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9c4ac | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x9beb4).

## `TIOInit`

- `TIOInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4b108 to 0x4b358.
  Established. The unnamed `0x4f81c` call carries `0x1`, `0x4ba54`, `0x4de94`, `0x4df78` (`bl` at 0x4b17c).
  An absolute word assignment stores `[sp-24]` at `0x132490` (`str` at 0x4b2b8).
  Byte stores assign `([sp-28] & 0xff)` to `0x299842` and `([sp-32] & 0xff)` to `0x299841` (`strb` at 0x4b2c4, `strb` at 0x4b2d0).
  TIO initialization record policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4b118 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x4b11c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x4b120 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x4b124 | load | 4 | `abs 0x4b324` | `-` |
| `ldr` at 0x4b128 | load | 4 | `abs 0x132490` | `-` |
| `ldr` at 0x4b134 | load | 4 | `abs 0x4b328` | `-` |
| `str` at 0x4b138 | store | 4 | `sp -0x38` | `0xf82ec` |
| `ldr` at 0x4b13c | load | 4 | `abs 0x4b32c` | `-` |
| `ldrb` at 0x4b140 | load | 1 | `abs 0x299842` | `-` |
| `str` at 0x4b144 | store | 4 | `sp -0x34` | `[0x299842]:1` |
| `ldr` at 0x4b14c | load | 4 | `abs 0x4b330` | `-` |
| `ldr` at 0x4b15c | load | 4 | `abs 0x4b334` | `-` |
| `str` at 0x4b160 | store | 4 | `sp -0x38` | `0x4be94` |
| `ldr` at 0x4b164 | load | 4 | `abs 0x4b338` | `-` |
| `str` at 0x4b168 | store | 4 | `sp -0x34` | `0x4c1d0` |
| `ldr` at 0x4b170 | load | 4 | `abs 0x4b33c` | `-` |
| `ldr` at 0x4b174 | load | 4 | `abs 0x4b340` | `-` |
| `ldr` at 0x4b178 | load | 4 | `abs 0x4b344` | `-` |
| `str` at 0x4b184 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x4b188 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4b18c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4b198 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4b1ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4b1b4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4b1c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4b1d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4b1d4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x4b1e0 | store | 4 | `sp -0x28` | `ret(0xdcfd0)` |
| `ldr` at 0x4b1e4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4b1f0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4b1fc | load | 4 | `abs 0x4b348` | `-` |
| `str` at 0x4b200 | store | 4 | `sp -0x38` | `0xf831c` |
| `ldr` at 0x4b204 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4b218 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4b220 | load | 4 | `(unknown)` | `-` |
| `str` at 0x4b224 | store | 4 | `sp -0x34` | `?` |
| `str` at 0x4b228 | store | 4 | `sp -0x30` | `ret(0x1f8e0)` |
| `ldr` at 0x4b230 | load | 4 | `abs 0x4b330` | `-` |
| `ldr` at 0x4b240 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x4b248 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x4b250 | load | 4 | `abs 0x4b34c` | `-` |
| `ldr` at 0x4b254 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4b268 | store | 4 | `abs 0x132498` | `ret(0x1943c)` |
| `ldr` at 0x4b26c | load | 4 | `abs 0x132498` | `-` |
| `ldr` at 0x4b278 | load | 4 | `abs 0x4b350` | `-` |
| `str` at 0x4b27c | store | 4 | `sp -0x38` | `0xf8338` |
| `ldr` at 0x4b284 | load | 4 | `abs 0x4b330` | `-` |
| `ldr` at 0x4b294 | load | 4 | `abs 0x4b34c` | `-` |
| `ldr` at 0x4b298 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4b2a0 | load | 4 | `abs 0x132498` | `-` |
| `ldr` at 0x4b2b0 | load | 4 | `abs 0x4b324` | `-` |
| `ldr` at 0x4b2b4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4b2b8 | store | 4 | `abs 0x132490` | `[sp-24]` |
| `ldr` at 0x4b2bc | load | 4 | `abs 0x4b32c` | `-` |
| `ldr` at 0x4b2c0 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x4b2c4 | store | 1 | `abs 0x299842` | `([sp-28] & 0xff)` |
| `ldr` at 0x4b2c8 | load | 4 | `abs 0x4b354` | `-` |
| `ldr` at 0x4b2cc | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x4b2d0 | store | 1 | `abs 0x299841` | `([sp-32] & 0xff)` |
| `str` at 0x4b2d8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x4b2dc | load | 4 | `abs 0x4b32c` | `-` |
| `ldrb` at 0x4b2e0 | load | 1 | `abs 0x299842` | `-` |
| `ldr` at 0x4b2e4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4b2f0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4b2f8 | load | 4 | `abs 0x4b34c` | `-` |
| `ldr` at 0x4b2fc | load | 4 | `abs 0x132498` | `-` |
| `strb` at 0x4b308 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x4b30c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x4b314 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldm` at 0x4b320 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x4b320 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4b320 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4b320 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4b158 | `(unknown callee) 0x200a0` | `0x9` | `0xf8308` | `0xd5` | `0x0` |
| `bl` at 0x4b17c | `(unknown callee) 0x4f81c` | `0x1` | `0x4ba54` | `0x4de94` | `0x4df78` |
| `bl` at 0x4b1d8 | `(unknown callee) 0xdcfd0` | `?` | `?` | `?` | `?` |
| `bl` at 0x4b1f4 | `(unknown callee) 0x1f8e0` | `ret(0xdcfd0)` | `?` | `?` | `ret(0xdcfd0)` |
| `bl` at 0x4b23c | `(unknown callee) 0x200a0` | `0x9` | `0xf8308` | `0xe2` | `0x0` |
| `bl` at 0x4b260 | `(unknown callee) 0x1943c` | `?` | `?` | `?` | `?` |
| `bl` at 0x4b290 | `(unknown callee) 0x200a0` | `0x9` | `0xf8308` | `0xea` | `0x0` |
| `bl` at 0x4b2ac | `(unknown callee) 0x19748` | `[0x132498]` | `0x0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4b130 | `[0x132490] == 0x0` | `[0x132490] != 0x0` |
| `bge` at 0x4b194 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x4b1ec | `ret(0xdcfd0) == 0x0` | `ret(0xdcfd0) != 0x0` |
| `bne` at 0x4b274 | `[0x132498] != 0x0` | `[0x132498] == 0x0` |
| `ble` at 0x4b2ec | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4b320 | `r0` | `ret(0x19748)` |

## `TIOSetLinkEnable`

- `TIOSetLinkEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4ba7c to 0x4bb04.
  Established. Nonzero `[0x132494]:1` takes `0x4bad4`, while zero falls through (`bne` at 0x4bac4).
  The unnamed `0x4bb04` call carries entry `r0`, entry `r1`, entry `r2`, `[0x132494]:1` (`bl` at 0x4bae0).
  An included word assignment saves `ret(0x4bb04)` at `sp -0x24` (`str` at 0x4baf0).
  Link enable policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4ba8c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4ba90 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x4ba94 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x4ba98 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4bab0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x4bab8 | load | 4 | `abs 0x4bb00` | `-` |
| `ldrb` at 0x4babc | load | 1 | `abs 0x132494` | `-` |
| `str` at 0x4bacc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x4bad4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4bad8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4badc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4bae8 | store | 4 | `sp -0x20` | `ret(0x4bb04)` |
| `ldr` at 0x4baec | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4baf0 | store | 4 | `sp -0x24` | `ret(0x4bb04)` |
| `ldr` at 0x4baf4 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x4bafc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4bafc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4bafc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4bae0 | `(unknown callee) 0x4bb04` | `r0` | `r1` | `r2` | `[0x132494]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4baa8 | `? == 0x4` | `? != 0x4` |
| `bne` at 0x4bac4 | `[0x132494]:1 != 0x0` | `[0x132494]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4bafc | `r0` | `?` |

## `tio_set_link_enable`

- `tio_set_link_enable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4bb04 to 0x4bbf0.
  Established. An included word assignment stores `0x90060780` at `sp -0x20` (`str` at 0x4bba4).
  The unnamed `0xde274` call carries `[sp-20]`, `[sp-32]`, `0x0`, `0x1` (`bl` at 0x4bbbc).
  Zero `[0x132630]` takes `0x4bb88`, while nonzero falls through (`beq` at 0x4bb50).
  Link register request interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4bb14 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4bb18 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x4bb1c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x4bb20 | load | 4 | `abs 0x4bbd8` | `-` |
| `ldr` at 0x4bb24 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x4bb30 | load | 4 | `abs 0x4bbdc` | `-` |
| `ldr` at 0x4bb34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4bb38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4bb3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4bb44 | load | 4 | `abs 0x4bbe0` | `-` |
| `ldr` at 0x4bb48 | load | 4 | `abs 0x132630` | `-` |
| `ldr` at 0x4bb54 | load | 4 | `abs 0x4bbdc` | `-` |
| `str` at 0x4bb58 | store | 4 | `sp -0x34` | `0xf8398` |
| `ldr` at 0x4bb5c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4bb60 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x4bb64 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4bb68 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x4bb6c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4bb70 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x4bb78 | load | 4 | `abs 0x4bbe4` | `-` |
| `ldr` at 0x4bb7c | load | 4 | `abs 0x4bbe8` | `-` |
| `ldr` at 0x4bb88 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4bb98 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x4bba0 | load | 4 | `abs 0x4bbec` | `-` |
| `str` at 0x4bba4 | store | 4 | `sp -0x20` | `0x90060780` |
| `ldr` at 0x4bbac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4bbb0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4bbc8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x4bbcc | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x4bbd4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4bbd4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4bbd4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4bb40 | `(unknown callee) 0x221f0` | `0xf8398` | `r0` | `r1` | `r2` |
| `bl` at 0x4bb84 | `(unknown callee) 0x200a0` | `0x7` | `0xf8308` | `0x216` | `0x0` |
| `bl` at 0x4bba8 | `(unknown callee) 0xd708c` | `?` | `?` | `?` | `0x90060780` |
| `bl` at 0x4bbbc | `(unknown callee) 0xde274` | `[sp-20]` | `[sp-32]` | `0x0` | `0x1` |
| `bl` at 0x4bbc0 | `(unknown callee) 0xd70fc` | `ret(0xde274)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4bb2c | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `beq` at 0x4bb50 | `[0x132630] == 0x0` | `[0x132630] != 0x0` |
| `bne` at 0x4bb90 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4bbd4 | `r0` | `0x0` |

## `TIOTarget`

- `TIOTarget`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4b524 to 0x4b56c.
  Established. The byte at `sp -0x11` receives `(r0 & 0xff)` (`strb` at 0x4b538).
  A word load fetches absolute `0x132490` (`ldr` at 0x4b554).
  The included indexed word read has an unknown effective target (`ldr` at 0x4b55c).
  TIO target numbering remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4b538 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x4b53c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x4b550 | load | 4 | `abs 0x4b568` | `-` |
| `ldr` at 0x4b554 | load | 4 | `abs 0x132490` | `-` |
| `ldr` at 0x4b55c | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x4b564 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x4b564 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4b564 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4b564 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4b564 | `r0` | `?` |

## `TIONotPresent`

- `TIONotPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4be58 to 0x4be94.
  Established. An included byte store clears an unknown effective target (`strb` at 0x4be88).
  A word load fetches absolute `0x132498` (`ldr` at 0x4be7c).
  The integer return candidate retains entry `r0` (`ldm` at 0x4be8c).
  TIO absence record meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4be6c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x4be70 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x4be78 | load | 4 | `abs 0x4be90` | `-` |
| `ldr` at 0x4be7c | load | 4 | `abs 0x132498` | `-` |
| `strb` at 0x4be88 | store | 1 | `(unknown)` | `0x0` |
| `ldm` at 0x4be8c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x4be8c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4be8c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4be8c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4be8c | `r0` | `r0` |

## `TIOTest_SetSystemAceMode`

- `TIOTest_SetSystemAceMode`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2e858 to 0x2e924.
  Established. The unnamed `0xca584` calls carry `0x0`, `0xb`, `0x80`, `[sp-32]` and `0x0`, `0xc`, `0x38`, `[sp-36]` (`bl` at 0x2e8ac, `bl` at 0x2e8e8).
  Zero `ret(0x1d444)` takes `0x2e894`, while nonzero falls through (`beq` at 0x2e884).
  The integer return candidate is `[sp-28]` (`ldm` at 0x2e920).
  System ACE mode pin effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2e870 | store | 4 | `sp -0x14` | `ret(0x1d444)` |
| `str` at 0x2e878 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x2e87c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2e88c | store | 4 | `sp -0x20` | `0x80` |
| `str` at 0x2e898 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x2e8a8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x2e8b4 | store | 4 | `sp -0x1c` | `ret(0xca584)` |
| `ldr` at 0x2e8b8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2e8c8 | store | 4 | `sp -0x24` | `0x10` |
| `str` at 0x2e8d4 | store | 4 | `sp -0x24` | `0x28` |
| `ldr` at 0x2e8e4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x2e8f0 | store | 4 | `sp -0x18` | `ret(0xca584)` |
| `ldr` at 0x2e8f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2e900 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2e90c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2e910 | store | 4 | `sp -0x18` | `[sp-28]` |
| `ldr` at 0x2e914 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x2e920 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x2e920 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x2e920 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2e86c | `(unknown callee) 0x1d444` | `0x99` | `r1` | `r2` | `r3` |
| `bl` at 0x2e8ac | `(unknown callee) 0xca584` | `0x0` | `0xb` | `0x80` | `[sp-32]` |
| `bl` at 0x2e8e8 | `(unknown callee) 0xca584` | `0x0` | `0xc` | `0x38` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x2e884 | `ret(0x1d444) == 0x0` | `ret(0x1d444) != 0x0` |
| `beq` at 0x2e8c0 | `[sp-20] == 0x0` | `[sp-20] != 0x0` |
| `beq` at 0x2e8fc | `ret(0xca584) == 0x0` | `ret(0xca584) != 0x0` |
| `bne` at 0x2e908 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x2e920 | `r0` | `[sp-28]` |

## `TIO_Issue_CX_Probe`

- `TIO_Issue_CX_Probe`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4cce8 to 0x4d3f4.
  Established. The unnamed `0xde16c` call carries unknown, `0x90060e80`, `sp-44`, `sp-48` (`bl` at 0x4cdc0).
  Signed `ret(0xe0104) >= 0x0` takes `0x4cd60`, while negative results fall through (`bge` at 0x4cd50).
  The included unnamed `0xde274` call carries `[sp-20]`, `0xb0000108`, `0x0`, `0x0` (`bl` at 0x4cf04).
  CX probe register semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4ccf8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x4cd00 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `str` at 0x4cd08 | store | 4 | `sp -0x20` | `0x0` |
| `ldrb` at 0x4cd0c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x4cd18 | load | 4 | `abs 0x4d3c8` | `-` |
| `ldr` at 0x4cd1c | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x4cd28 | load | 4 | `abs 0x4d3cc` | `-` |
| `ldr` at 0x4cd30 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4cd44 | store | 4 | `sp -0x20` | `ret(0xe0104)` |
| `ldr` at 0x4cd48 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4cd54 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4cd58 | store | 4 | `sp -0x6c` | `ret(0xe0104)` |
| `ldr` at 0x4cd60 | load | 4 | `abs 0x4d3c8` | `-` |
| `ldr` at 0x4cd64 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x4cd70 | load | 4 | `abs 0x4d3d0` | `-` |
| `ldr` at 0x4cd78 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4cd8c | store | 4 | `sp -0x20` | `ret(0xe0104)` |
| `ldr` at 0x4cd90 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4cd9c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4cda0 | store | 4 | `sp -0x6c` | `ret(0xe0104)` |
| `ldr` at 0x4cdb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4cdb4 | load | 4 | `abs 0x4d3d4` | `-` |
| `str` at 0x4cdc8 | store | 4 | `sp -0x20` | `ret(0xde16c)` |
| `ldr` at 0x4cdcc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4cdd8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4cddc | store | 4 | `sp -0x6c` | `ret(0xde16c)` |
| `ldr` at 0x4cde4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x4cdf0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x4cdf4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x4ce00 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x4ce0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4ce10 | load | 4 | `abs 0x4d3d8` | `-` |
| `str` at 0x4ce24 | store | 4 | `sp -0x20` | `ret(0xde16c)` |
| `ldr` at 0x4ce28 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4ce34 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4ce38 | store | 4 | `sp -0x6c` | `ret(0xde16c)` |
| `ldr` at 0x4ce48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4ce4c | load | 4 | `abs 0x4d3dc` | `-` |
| `str` at 0x4ce60 | store | 4 | `sp -0x20` | `ret(0xde16c)` |
| `ldr` at 0x4ce64 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4ce70 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4ce74 | store | 4 | `sp -0x6c` | `ret(0xde16c)` |
| `ldr` at 0x4ce84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4ce88 | load | 4 | `abs 0x4d3e0` | `-` |
| `str` at 0x4ce9c | store | 4 | `sp -0x20` | `ret(0xde16c)` |
| `ldr` at 0x4cea0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4ceac | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4ceb0 | store | 4 | `sp -0x6c` | `ret(0xde16c)` |
| `ldr` at 0x4ceb8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x4cec0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4cec4 | load | 4 | `abs 0x4d3d8` | `-` |
| `ldr` at 0x4cecc | load | 4 | `sp -0x38` | `-` |
| `str` at 0x4ced8 | store | 4 | `sp -0x20` | `ret(0xde274)` |
| `ldr` at 0x4cedc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4cee8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4ceec | store | 4 | `sp -0x6c` | `ret(0xde274)` |
| `ldr` at 0x4cef4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4cef8 | load | 4 | `abs 0x4d3e0` | `-` |
| `str` at 0x4cf0c | store | 4 | `sp -0x20` | `ret(0xde274)` |
| `ldr` at 0x4cf10 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4cf1c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4cf20 | store | 4 | `sp -0x6c` | `ret(0xde274)` |
| `ldr` at 0x4cf28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4cf2c | load | 4 | `abs 0x4d3dc` | `-` |
| `str` at 0x4cf40 | store | 4 | `sp -0x20` | `ret(0xde274)` |
| `ldr` at 0x4cf44 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4cf50 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4cf54 | store | 4 | `sp -0x6c` | `ret(0xde274)` |
| `str` at 0x4cf60 | store | 4 | `sp -0x64` | `0x0` |
| `ldr` at 0x4cf64 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x4cf74 | store | 4 | `sp -0x7c` | `0x0` |
| `str` at 0x4cf7c | store | 4 | `sp -0x78` | `0x30000000` |
| `str` at 0x4cf84 | store | 4 | `sp -0x74` | `sp-44` |
| `str` at 0x4cf8c | store | 4 | `sp -0x70` | `sp-48` |
| `ldr` at 0x4cf90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4cf94 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4cf98 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x4cfa8 | store | 4 | `sp -0x1c` | `ret(0x5a344)` |
| `ldr` at 0x4cfac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4cfb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4cfbc | load | 4 | `abs 0x4d3d8` | `-` |
| `ldr` at 0x4cfc0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x4cfc4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x4cfd0 | store | 4 | `sp -0x20` | `ret(0xde274)` |
| `ldr` at 0x4cfd4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4cfe0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4cfe4 | store | 4 | `sp -0x6c` | `ret(0xde274)` |
| `ldr` at 0x4cfec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4cff0 | load | 4 | `abs 0x4d3dc` | `-` |
| `ldr` at 0x4cff4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x4cff8 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x4d004 | store | 4 | `sp -0x20` | `ret(0xde274)` |
| `ldr` at 0x4d008 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4d014 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4d018 | store | 4 | `sp -0x6c` | `ret(0xde274)` |
| `ldr` at 0x4d020 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4d024 | load | 4 | `abs 0x4d3e0` | `-` |
| `ldr` at 0x4d028 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x4d02c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x4d038 | store | 4 | `sp -0x20` | `ret(0xde274)` |
| `ldr` at 0x4d03c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4d048 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4d04c | store | 4 | `sp -0x6c` | `ret(0xde274)` |
| `str` at 0x4d058 | store | 4 | `sp -0x6c` | `0x0` |
| `ldr` at 0x4d060 | load | 4 | `abs 0x4d3c8` | `-` |
| `ldr` at 0x4d064 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x4d070 | load | 4 | `abs 0x4d3cc` | `-` |
| `ldr` at 0x4d078 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4d08c | store | 4 | `sp -0x20` | `ret(0xe0104)` |
| `ldr` at 0x4d090 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4d09c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4d0a0 | store | 4 | `sp -0x6c` | `ret(0xe0104)` |
| `ldr` at 0x4d0a8 | load | 4 | `abs 0x4d3c8` | `-` |
| `ldr` at 0x4d0ac | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x4d0b8 | load | 4 | `abs 0x4d3d0` | `-` |
| `ldr` at 0x4d0c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4d0d4 | store | 4 | `sp -0x20` | `ret(0xe0104)` |
| `ldr` at 0x4d0d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4d0e4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4d0e8 | store | 4 | `sp -0x6c` | `ret(0xe0104)` |
| `ldr` at 0x4d0f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4d0f4 | load | 4 | `abs 0x4d3e4` | `-` |
| `str` at 0x4d108 | store | 4 | `sp -0x20` | `ret(0xde274)` |
| `ldr` at 0x4d10c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4d118 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4d11c | store | 4 | `sp -0x6c` | `ret(0xde274)` |
| `ldr` at 0x4d124 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4d128 | load | 4 | `abs 0x4d3e4` | `-` |
| `str` at 0x4d13c | store | 4 | `sp -0x20` | `ret(0xde274)` |
| `ldr` at 0x4d140 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4d14c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4d150 | store | 4 | `sp -0x6c` | `ret(0xde274)` |
| `ldr` at 0x4d160 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4d164 | load | 4 | `abs 0x4d3e8` | `-` |
| `str` at 0x4d178 | store | 4 | `sp -0x20` | `ret(0xde16c)` |
| `ldr` at 0x4d17c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4d188 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4d18c | store | 4 | `sp -0x6c` | `ret(0xde16c)` |
| `str` at 0x4d198 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0x4d19c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x4d1ac | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x4d1b4 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x4d1c4 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x4d1dc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4d1f0 | store | 4 | `sp -0x20` | `ret(0xde16c)` |
| `ldr` at 0x4d1f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4d200 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4d204 | store | 4 | `sp -0x6c` | `ret(0xde16c)` |
| `ldr` at 0x4d20c | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x4d214 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x4d218 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x4d228 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4d230 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x4d234 | load | 4 | `sp -0x60` | `-` |
| `str` at 0x4d240 | store | 4 | `sp -0x20` | `ret(0xde274)` |
| `ldr` at 0x4d244 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4d250 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4d254 | store | 4 | `sp -0x6c` | `ret(0xde274)` |
| `ldr` at 0x4d25c | load | 4 | `sp -0x68` | `-` |
| `str` at 0x4d264 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0x4d26c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4d284 | store | 4 | `sp -0x20` | `ret(0xde274)` |
| `ldr` at 0x4d288 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4d294 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4d298 | store | 4 | `sp -0x6c` | `ret(0xde274)` |
| `ldr` at 0x4d2a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4d2a4 | load | 4 | `abs 0x4d3ec` | `-` |
| `str` at 0x4d2b8 | store | 4 | `sp -0x20` | `ret(0xde274)` |
| `ldr` at 0x4d2bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4d2c8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4d2cc | store | 4 | `sp -0x6c` | `ret(0xde274)` |
| `ldr` at 0x4d2d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4d2d8 | load | 4 | `abs 0x4d3f0` | `-` |
| `str` at 0x4d2ec | store | 4 | `sp -0x20` | `ret(0xde274)` |
| `ldr` at 0x4d2f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4d2fc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4d300 | store | 4 | `sp -0x6c` | `ret(0xde274)` |
| `ldr` at 0x4d308 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x4d310 | store | 4 | `sp -0x64` | `[sp-100]+1` |
| `ldr` at 0x4d318 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4d31c | load | 4 | `abs 0x4d3d8` | `-` |
| `ldr` at 0x4d320 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x4d324 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x4d330 | store | 4 | `sp -0x20` | `ret(0xde274)` |
| `ldr` at 0x4d334 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4d340 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4d344 | store | 4 | `sp -0x6c` | `ret(0xde274)` |
| `ldr` at 0x4d34c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4d350 | load | 4 | `abs 0x4d3dc` | `-` |
| `ldr` at 0x4d354 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x4d358 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x4d364 | store | 4 | `sp -0x20` | `ret(0xde274)` |
| `ldr` at 0x4d368 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4d374 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4d378 | store | 4 | `sp -0x6c` | `ret(0xde274)` |
| `ldr` at 0x4d380 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4d384 | load | 4 | `abs 0x4d3e0` | `-` |
| `ldr` at 0x4d388 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x4d38c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x4d398 | store | 4 | `sp -0x20` | `ret(0xde274)` |
| `ldr` at 0x4d39c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4d3a8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4d3ac | store | 4 | `sp -0x6c` | `ret(0xde274)` |
| `ldr` at 0x4d3b4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4d3b8 | store | 4 | `sp -0x6c` | `[sp-28]` |
| `ldr` at 0x4d3bc | load | 4 | `sp -0x6c` | `-` |
| `ldm` at 0x4d3c4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4d3c4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4d3c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4cd2c | `(unknown callee) 0x221f0` | `0xf8534` | `r1` | `r2` | `[0x29737c]` |
| `bl` at 0x4cd3c | `(unknown callee) 0xe0104` | `?` | `0x0` | `0x1` | `?` |
| `bl` at 0x4cd74 | `(unknown callee) 0x221f0` | `0xf8554` | `?` | `?` | `[0x29737c]` |
| `bl` at 0x4cd84 | `(unknown callee) 0xe0104` | `[sp-20]` | `0x0` | `0x2` | `?` |
| `bl` at 0x4cdc0 | `(unknown callee) 0xde16c` | `?` | `0x90060e80` | `sp-44` | `sp-48` |
| `bl` at 0x4ce1c | `(unknown callee) 0xde16c` | `[sp-20]` | `0xb0000028` | `sp-52` | `sp-56` |
| `bl` at 0x4ce58 | `(unknown callee) 0xde16c` | `[sp-20]` | `0xb0000100` | `sp-68` | `sp-72` |
| `bl` at 0x4ce94 | `(unknown callee) 0xde16c` | `[sp-20]` | `0xb0000108` | `sp-60` | `sp-64` |
| `bl` at 0x4ced0 | `(unknown callee) 0xde274` | `[sp-20]` | `0xb0000028` | `?` | `[sp-56]` |
| `bl` at 0x4cf04 | `(unknown callee) 0xde274` | `[sp-20]` | `0xb0000108` | `0x0` | `0x0` |
| `bl` at 0x4cf38 | `(unknown callee) 0xde274` | `[sp-20]` | `0xb0000100` | `0x0` | `0xc8` |
| `bl` at 0x4cfa0 | `(unknown callee) 0x5a344` | `[sp-20]` | `[sp-36]` | `[sp-40]` | `0xff` |
| `bl` at 0x4cfc8 | `(unknown callee) 0xde274` | `[sp-20]` | `0xb0000028` | `[sp-52]` | `[sp-56]` |
| `bl` at 0x4cffc | `(unknown callee) 0xde274` | `[sp-20]` | `0xb0000100` | `[sp-68]` | `[sp-72]` |
| `bl` at 0x4d030 | `(unknown callee) 0xde274` | `[sp-20]` | `0xb0000108` | `[sp-60]` | `[sp-64]` |
| `bl` at 0x4d074 | `(unknown callee) 0x221f0` | `0xf8534` | `?` | `?` | `[0x29737c]` |
| `bl` at 0x4d084 | `(unknown callee) 0xe0104` | `[sp-20]` | `0x0` | `0x1` | `?` |
| `bl` at 0x4d0bc | `(unknown callee) 0x221f0` | `0xf8554` | `?` | `?` | `[0x29737c]` |
| `bl` at 0x4d0cc | `(unknown callee) 0xe0104` | `[sp-20]` | `0x0` | `0x2` | `?` |
| `bl` at 0x4d100 | `(unknown callee) 0xde274` | `[sp-20]` | `0xb000b100` | `0x0` | `0x8` |
| `bl` at 0x4d134 | `(unknown callee) 0xde274` | `[sp-20]` | `0xb000b100` | `0x0` | `0x0` |
| `bl` at 0x4d170 | `(unknown callee) 0xde16c` | `[sp-20]` | `0xb0008118` | `sp-84` | `sp-88` |
| `bl` at 0x4d1e8 | `(unknown callee) 0xde16c` | `[sp-20]` | `?` | `sp-92` | `sp-96` |
| `bl` at 0x4d238 | `(unknown callee) 0xde274` | `[sp-20]` | `?` | `[sp-92]` | `[sp-96]` |
| `bl` at 0x4d27c | `(unknown callee) 0xde274` | `[sp-20]` | `0xb0000000` | `0x0` | `0x0` |
| `bl` at 0x4d2b0 | `(unknown callee) 0xde274` | `[sp-20]` | `0xb0008000` | `0x0` | `0x0` |
| `bl` at 0x4d2e4 | `(unknown callee) 0xde274` | `[sp-20]` | `0xb0008060` | `0x0` | `0x0` |
| `bl` at 0x4d328 | `(unknown callee) 0xde274` | `[sp-20]` | `0xb0000028` | `[sp-52]` | `[sp-56]` |
| `bl` at 0x4d35c | `(unknown callee) 0xde274` | `[sp-20]` | `0xb0000100` | `[sp-68]` | `[sp-72]` |
| `bl` at 0x4d390 | `(unknown callee) 0xde274` | `[sp-20]` | `0xb0000108` | `[sp-60]` | `[sp-64]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x4cd14 | `(r1 & 0xff) != 0x1` | `(r1 & 0xff) == 0x1` |
| `bls` at 0x4cd24 | `[0x29737c] <= 0x1 (unsigned, 32-bit)` | `[0x29737c] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x4cd50 | `ret(0xe0104) >= 0x0 (signed, 32-bit)` | `ret(0xe0104) < 0x0 (signed, 32-bit)` |
| `bls` at 0x4cd6c | `[0x29737c] <= 0x1 (unsigned, 32-bit)` | `[0x29737c] > 0x1 (unsigned, 32-bit)` |
| `beq` at 0x4cd98 | `ret(0xe0104) == 0x0` | `ret(0xe0104) != 0x0` |
| `beq` at 0x4cdd4 | `ret(0xde16c) == 0x0` | `ret(0xde16c) != 0x0` |
| `beq` at 0x4ce30 | `ret(0xde16c) == 0x0` | `ret(0xde16c) != 0x0` |
| `beq` at 0x4ce6c | `ret(0xde16c) == 0x0` | `ret(0xde16c) != 0x0` |
| `beq` at 0x4cea8 | `ret(0xde16c) == 0x0` | `ret(0xde16c) != 0x0` |
| `beq` at 0x4cee4 | `ret(0xde274) == 0x0` | `ret(0xde274) != 0x0` |
| `beq` at 0x4cf18 | `ret(0xde274) == 0x0` | `ret(0xde274) != 0x0` |
| `beq` at 0x4cf4c | `ret(0xde274) == 0x0` | `ret(0xde274) != 0x0` |
| `bhi` at 0x4cf6c | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0x4cfb4 | `ret(0x5a344) != 0x0` | `ret(0x5a344) == 0x0` |
| `beq` at 0x4cfdc | `ret(0xde274) == 0x0` | `ret(0xde274) != 0x0` |
| `beq` at 0x4d010 | `ret(0xde274) == 0x0` | `ret(0xde274) != 0x0` |
| `beq` at 0x4d044 | `ret(0xde274) == 0x0` | `ret(0xde274) != 0x0` |
| `bls` at 0x4d06c | `[0x29737c] <= 0x1 (unsigned, 32-bit)` | `[0x29737c] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x4d098 | `ret(0xe0104) >= 0x0 (signed, 32-bit)` | `ret(0xe0104) < 0x0 (signed, 32-bit)` |
| `bls` at 0x4d0b4 | `[0x29737c] <= 0x1 (unsigned, 32-bit)` | `[0x29737c] > 0x1 (unsigned, 32-bit)` |
| `beq` at 0x4d0e0 | `ret(0xe0104) == 0x0` | `ret(0xe0104) != 0x0` |
| `beq` at 0x4d114 | `ret(0xde274) == 0x0` | `ret(0xde274) != 0x0` |
| `beq` at 0x4d148 | `ret(0xde274) == 0x0` | `ret(0xde274) != 0x0` |
| `beq` at 0x4d184 | `ret(0xde16c) == 0x0` | `ret(0xde16c) != 0x0` |
| `bhi` at 0x4d1a4 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0x4d1c0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x4d1fc | `ret(0xde16c) == 0x0` | `ret(0xde16c) != 0x0` |
| `beq` at 0x4d24c | `ret(0xde274) == 0x0` | `ret(0xde274) != 0x0` |
| `beq` at 0x4d290 | `ret(0xde274) == 0x0` | `ret(0xde274) != 0x0` |
| `beq` at 0x4d2c4 | `ret(0xde274) == 0x0` | `ret(0xde274) != 0x0` |
| `beq` at 0x4d2f8 | `ret(0xde274) == 0x0` | `ret(0xde274) != 0x0` |
| `beq` at 0x4d33c | `ret(0xde274) == 0x0` | `ret(0xde274) != 0x0` |
| `beq` at 0x4d370 | `ret(0xde274) == 0x0` | `ret(0xde274) != 0x0` |
| `beq` at 0x4d3a4 | `ret(0xde274) == 0x0` | `ret(0xde274) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4d3c4 | `r0` | `[sp-108]` |
